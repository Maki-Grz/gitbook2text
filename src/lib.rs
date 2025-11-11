//! # gitbook2text
//!
//! Une bibliothèque et outil CLI pour télécharger des pages GitBook et les convertir
//! en markdown et texte brut.
//!
//! ## Exemples
//!
//! ### Crawling d'un GitBook
//!
//! ```no_run
//! use gitbook2text::{is_gitbook, extract_gitbook_links};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let url = "https://docs.example.com";
//!
//!     if is_gitbook(url).await? {
//!         let links = extract_gitbook_links(url).await?;
//!         println!("Trouvé {} pages", links.len());
//!     }
//!     Ok(())
//! }
//! ```
//!
//! ### Téléchargement et conversion
//!
//! ```no_run
//! use gitbook2text::{download_page, markdown_to_text, txt_sanitize};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let url = "https://example.com/page.md";
//!     let content = download_page(url).await?;
//!     let text = markdown_to_text(&content);
//!     let cleaned = txt_sanitize(&text);
//!     println!("{}", cleaned);
//!     Ok(())
//! }
//! ```

mod crawler;
mod utils;

pub use utils::{
    download_page, markdown_to_text, save_markdown, save_text, txt_sanitize, url_to_filename,
};

pub use crawler::{crawl_and_save, extract_gitbook_links, is_gitbook};

#[derive(Debug)]
pub enum GitBookError {
    NetworkError(reqwest::Error),
    IoError(std::io::Error),
    InvalidUrl(String),
    NotAGitBook(String),
}

impl std::fmt::Display for GitBookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GitBookError::NetworkError(e) => write!(f, "Erreur réseau: {}", e),
            GitBookError::IoError(e) => write!(f, "Erreur I/O: {}", e),
            GitBookError::InvalidUrl(url) => write!(f, "URL invalide: {}", url),
            GitBookError::NotAGitBook(url) => write!(f, "{} n'est pas un GitBook", url),
        }
    }
}

impl std::error::Error for GitBookError {}

impl From<reqwest::Error> for GitBookError {
    fn from(err: reqwest::Error) -> Self {
        GitBookError::NetworkError(err)
    }
}

impl From<std::io::Error> for GitBookError {
    fn from(err: std::io::Error) -> Self {
        GitBookError::IoError(err)
    }
}
