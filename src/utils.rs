use pulldown_cmark::{Event, Parser};
use regex::Regex;
use tokio::fs;

/// Download the content of a page from a URL
///
/// # Arguments
///
/// * `url` - The URL of the page to download
///
/// # Exemples
///
/// ```no_run
/// use gitbook2text::download_page;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let content = download_page("https://example.com/page.md").await?;
///     println!("Contenu téléchargé: {} octets", content.len());
///     Ok(())
/// }
/// ```
///
/// # Errors
///
/// Returns an error if the HTTP request fails or if the response cannot be read
pub async fn download_page(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?;
    let text = resp.text().await?;
    Ok(text)
}

/// Save the markdown content to a file
///
/// The file will be created in the `data/md/` directory with a name based on the URL
///
/// # Arguments
///
/// * `url` - The source URL (used to generate the file name)
/// * `content` - The markdown content to save
///
/// # Exemples
///
/// ```no_run
/// use gitbook2text::save_markdown;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     save_markdown("https://example.com/page", "# Titre\nContenu").await?;
///     Ok(())
/// }
/// ```
///
/// # Errors
///
/// Returns an error if the file write fails
pub async fn save_markdown(url: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let filename = url_to_filename(url) + ".md";
    fs::write(format!("data/md/{}", filename), content).await?;
    Ok(())
}

/// Converts a URL into a safe filename
///
/// Replaces the characters `/` and `:` with underscores
///
/// # Arguments
///
/// * `url` - The URL to convert
///
/// # Exemples
///
/// ```
/// use gitbook2text::url_to_filename;
///
/// let filename = url_to_filename("https://example.com/path/to/page");
/// assert_eq!(filename, "https___example.com_path_to_page");
/// ```
pub fn url_to_filename(url: &str) -> String {
    url.replace(['/', ':'], "_")
}

/// Converts markdown to plain text
///
/// Extracts text from markdown events, ignoring formatting
///
/// # Arguments
///
/// * `md` - The markdown content to convert
///
/// # Exemples
///
/// ```
/// use gitbook2text::markdown_to_text;
///
/// let markdown = "# Titre\n\nParagraphe avec **gras**";
/// let text = markdown_to_text(markdown);
/// assert!(text.contains("Titre"));
/// assert!(text.contains("gras"));
/// ```
pub fn markdown_to_text(md: &str) -> String {
    let parser = Parser::new(md);
    let mut text = String::new();

    for event in parser {
        match event {
            Event::Text(t) => text.push_str(&t),
            Event::Code(t) => text.push_str(&t),
            Event::SoftBreak | Event::HardBreak => text.push('\n'),
            _ => {}
        }
    }

    text
}

/// Cleans and sanitizes the text by removing special GitBook tags
///
/// Removes `{% code %}`, `{% endcode %}`, and other special GitBook tags,
/// normalizes spaces, and removes dashes and quotation marks.
///
/// # Arguments
///
/// * `txt` - The text to clean
///
/// # Exemples
///
/// ```
/// use gitbook2text::txt_sanitize;
///
/// let dirty = r#"{% code title="example.rs" %}fn main() {}{% endcode %}"#;
/// let clean = txt_sanitize(dirty);
/// assert!(clean.contains("example.rs"));
/// assert!(clean.contains("fn main()"));
/// ```
pub fn txt_sanitize(txt: &str) -> String {
    let mut result = String::from(txt);

    let re_code =
        Regex::new(r#"\{%\s*code[^}]*title\s*=\s*"([^"]+)"[^}]*%}(.*?)\{%\s*endcode\s*%\}"#)
            .unwrap();
    result = re_code
        .replace_all(&result, |caps: &regex::Captures| {
            format!("{} {}", &caps[1], &caps[2])
        })
        .to_string();

    let re_code_no_title = Regex::new(r#"\{%\s*code[^}]*%}(.*?)\{%\s*endcode\s*%\}"#).unwrap();
    result = re_code_no_title
        .replace_all(&result, |caps: &regex::Captures| caps[1].to_string())
        .to_string();

    let re_title = Regex::new(r#"\{%\s*[^}]*title\s*=\s*"([^"]+)"[^}]*%\}"#).unwrap();
    result = re_title.replace_all(&result, "$1").to_string();

    let re_generic = Regex::new(r#"\{%\s*[^}]*%\}"#).unwrap();
    result = re_generic.replace_all(&result, "").to_string();

    let re_dash = Regex::new(r#"["|-]"#).unwrap();
    result = re_dash.replace_all(&result, "").to_string();
    let re_space = Regex::new(r"\s+").unwrap();
    result = re_space.replace_all(&result, " ").to_string();

    result.trim().to_string()
}

/// Saves the text content to a file
///
/// The file will be created in the `data/txt/` directory with a name based on the URL
///
/// # Arguments
///
/// * `url` - The source URL (used to generate the file name)
/// * `content` - The text content to save
///
/// # Exemples
///
/// ```no_run
/// use gitbook2text::save_text;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     save_text("https://example.com/page", "Contenu texte").await?;
///     Ok(())
/// }
/// ```
///
/// # Errors
///
/// Returns an error if writing the file fails
pub async fn save_text(url: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let filename = url_to_filename(url) + ".txt";
    fs::write(format!("data/txt/{}", filename), content).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_to_filename() {
        let url = "https://example.com/path/to/page";
        let filename = url_to_filename(url);
        assert_eq!(filename, "https___example.com_path_to_page");
    }

    #[test]
    fn test_markdown_to_text() {
        let md = "# Title\n\nSome **bold** text";
        let text = markdown_to_text(md);
        assert!(text.contains("Title"));
        assert!(text.contains("bold"));
    }

    #[test]
    fn test_txt_sanitize() {
        let input = r#"{% code title="test.rs" %}fn main(){}{% endcode %}"#;
        let output = txt_sanitize(input);
        assert!(output.contains("test.rs"));
        assert!(output.contains("fn main(){}"));
    }
}
