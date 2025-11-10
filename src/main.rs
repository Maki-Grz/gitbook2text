use futures::StreamExt;
use futures::stream::FuturesUnordered;
use gitbook2text::{download_page, markdown_to_text, save_markdown, save_text, txt_sanitize};
use std::collections::HashSet;
use std::fs;
use std::process;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Erreur: {}", e);
        process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string("links.txt").map_err(|e| {
        format!(
            "Impossible de lire links.txt: {}. Assurez-vous que le fichier existe.",
            e
        )
    })?;

    let mut urls: HashSet<String> = content
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.to_string())
        .collect();

    if urls.is_empty() {
        return Err("Aucune URL trouvée dans links.txt".into());
    }

    println!("📥 Téléchargement de {} page(s)...", urls.len());

    let new_urls_with_md = urls
        .drain()
        .map(|mut u| {
            u.push_str(".md");
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
                println!("✅ Page sauvegardée: {}", url);
            }
            Err(e) => {
                error_count += 1;
                eprintln!("❌ Erreur: {:?}", e);
            }
        }
    }

    println!("\n📊 Résumé:");
    println!("  ✅ Succès: {}", success_count);
    println!("  ❌ Erreurs: {}", error_count);

    if error_count > 0 {
        return Err(format!("{} page(s) n'ont pas pu être téléchargées", error_count).into());
    }

    Ok(())
}
