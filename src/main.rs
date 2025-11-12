use clap::{Parser, Subcommand};
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use gitbook2text::{crawl_and_save, extract_gitbook_links, is_gitbook};
use gitbook2text::{download_page, markdown_to_text, save_markdown, save_text, txt_sanitize};
use std::collections::HashSet;
use std::fs;
use std::process;

#[derive(Parser)]
#[command(name = "gitbook2text")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Crawl {
        #[arg(value_name = "URL")]
        url: String,

        #[arg(short, long, default_value = "links.txt")]
        output: String,
    },

    Download {
        #[arg(short, long, default_value = "links.txt")]
        input: String,
    },

    All {
        #[arg(value_name = "URL")]
        url: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Some(Commands::Crawl { url, output }) => crawl_command(&url, &output).await,
        Some(Commands::Download { input }) => download_command(&input).await,
        Some(Commands::All { url }) => all_command(&url).await,
        None => download_command("links.txt").await,
    };

    if let Err(e) = result {
        eprintln!("❌ Error: {}", e);
        process::exit(1);
    }
}

async fn crawl_command(url: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🕷️ Crawl Mode");
    crawl_and_save(url, output).await?;
    Ok(())
}

async fn download_command(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("📥 Download Mode");

    let content = fs::read_to_string(input).map_err(|e| {
        format!(
            "Can't read file {} : {}. You can use 'gitbook2text crawl <URL>' to generate the file.",
            input, e
        )
    })?;

    let urls: HashSet<String> = content
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.to_string())
        .collect();

    if urls.is_empty() {
        return Err(format!("No URL found in {}", input).into());
    }

    download_pages(urls).await
}

async fn all_command(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Full Mode: Crawl + Download");

    println!("\n📍 Step 1: Crawling");
    println!("🔍 Vérification que {} est un GitBook...", url);

    if !is_gitbook(url).await? {
        return Err(format!("⚠️ Checking that {} is a GitBook...", url).into());
    }

    println!("✅ GitBook detected !");
    println!("🕷️ Extracting links...");

    let links = extract_gitbook_links(url).await?;

    println!("✅ {} page(s) found", links.len());

    println!("\n📍 Step 2: Downloading");
    download_pages(links.into_iter().collect()).await
}

async fn download_pages(mut urls: HashSet<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!("📥 Downloading {} page(s)...", urls.len());

    let new_urls_with_md = urls
        .drain()
        .map(|mut u| {
            if !u.ends_with(".md") {
                u.push_str(".md");
            }
            u
        })
        .collect();

    urls = new_urls_with_md;

    fs::create_dir_all("data/md")?;
    fs::create_dir_all("data/txt")?;

    let mut futures = FuturesUnordered::new();

    for url in urls {
        let url_clone = url.clone();
        futures.push(async move {
            let md_content = download_page(&url_clone).await?;
            save_markdown(&url_clone, &md_content).await?;

            let text_content = markdown_to_text(&md_content);
            let text_cleaned = txt_sanitize(&text_content);
            save_text(&url_clone, &text_cleaned).await?;

            Ok::<String, Box<dyn std::error::Error>>(url_clone)
        });
    }

    let mut success_count = 0;
    let mut error_count = 0;

    while let Some(result) = futures.next().await {
        match result {
            Ok(url) => {
                success_count += 1;
                println!("✅ Page saved: {}", url);
            }
            Err(e) => {
                error_count += 1;
                eprintln!("❌ Error: {:?}", e);
            }
        }
    }

    println!("\n📊 Summary:");
    println!("  ✅ Success: {}", success_count);
    println!("  ❌ Errors: {}", error_count);

    if error_count > 0 {
        println!("\n⚠️  {} page(s) could not be downloaded", error_count);
    }

    Ok(())
}
