use scraper::{Html, Selector};
use std::collections::HashSet;
use url::Url;

/// Checks if a URL points to a GitBook site
///
/// # Arguments
///
/// * `url` - The URL to check
///
/// # Exemples
///
/// ```no_run
/// use gitbook2text::is_gitbook;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let is_gb = is_gitbook("https://docs.example.com").await?;
///     println!("Est un GitBook: {}", is_gb);
///     Ok(())
/// }
/// ```
pub async fn is_gitbook(url: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()?;

    let response = client.get(url).send().await?;
    let html = response.text().await?;

    let indicators = ["gitbook", "data-gitbook", "__GITBOOK__", "gitbook.com"];

    let html_lower = html.to_lowercase();
    Ok(indicators
        .iter()
        .any(|&indicator| html_lower.contains(indicator)))
}

/// Extracts all documentation links from a GitBook site
///
/// # Arguments
///
/// * `base_url` - The base URL of the GitBook
///
/// # Exemples
///
/// ```no_run
/// use gitbook2text::extract_gitbook_links;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let links = extract_gitbook_links("https://docs.example.com").await?;
///     for link in links {
///         println!("{}", link);
///     }
///     Ok(())
/// }
/// ```
pub async fn extract_gitbook_links(
    base_url: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()?;

    let base = Url::parse(base_url)?;
    let mut visited = HashSet::new();
    let mut to_visit = vec![base_url.to_string()];
    let mut all_links = HashSet::new();

    let link_selector = Selector::parse("a").unwrap();

    while let Some(current_url) = to_visit.pop() {
        if visited.contains(&current_url) {
            continue;
        }

        visited.insert(current_url.clone());

        println!("🔍 Exploration: {}", current_url);

        let response = match client.get(&current_url).send().await {
            Ok(r) => r,
            Err(e) => {
                eprintln!("⚠️ Error while retrieving {}: {}", current_url, e);
                continue;
            }
        };

        let html = match response.text().await {
            Ok(h) => h,
            Err(e) => {
                eprintln!("⚠️ Error while reading HTML: {}", e);
                continue;
            }
        };

        let document = Html::parse_document(&html);

        for element in document.select(&link_selector) {
            if let Some(href) = element.value().attr("href") {
                if let Ok(link_url) = base.join(href) {
                    let link_str = link_url.to_string();

                    if link_url.domain() == base.domain()
                        && !link_str.contains('#')
                        && !link_str.ends_with(".pdf")
                        && !link_str.ends_with(".zip")
                        && !link_str.ends_with(".jpg")
                        && !link_str.ends_with(".png")
                    {
                        let normalized = link_str.trim_end_matches('/').to_string();

                        all_links.insert(normalized.clone());

                        if !visited.contains(&normalized) && !to_visit.contains(&normalized) {
                            to_visit.push(normalized);
                        }
                    }
                }
            }
        }
    }

    let mut result: Vec<String> = all_links.into_iter().collect();
    result.sort();

    println!("✅ {} page(s) trouvée(s)", result.len());

    Ok(result)
}

/// Extracts links from a GitBook and saves them to a file
///
/// # Arguments
///
/// * `base_url` - The base URL of the GitBook
/// * `output_file` - The path to the output file (default: "links.txt")
///
/// # Exemples
///
/// ```no_run
/// use gitbook2text::crawl_and_save;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     crawl_and_save("https://docs.example.com", "links.txt").await?;
///     Ok(())
/// }
/// ```
pub async fn crawl_and_save(
    base_url: &str,
    output_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Checking that {} is a GitBook...", base_url);

    if !is_gitbook(base_url).await? {
        return Err(format!("⚠️ {} does not seem to be a GitBook site", base_url).into());
    }

    println!("✅ GitBook detected !");
    println!("🕷️ Starting crawling...");

    let links = extract_gitbook_links(base_url).await?;

    let content = links.join("\n");
    tokio::fs::write(output_file, content).await?;

    println!("💾 {} saved links in {}", links.len(), output_file);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_is_gitbook() {
        // This test requires an internet connection
        // You can skip it with #[ignore] if necessary
        let result = is_gitbook("https://docs.gitbook.com").await;
        assert!(result.is_ok());
    }
}
