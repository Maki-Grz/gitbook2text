//! # gitbook2text
//!
//! Une bibliothèque et outil CLI pour télécharger des pages GitBook et les convertir
//! en markdown et texte brut.
//!
//! ## Exemples
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

mod utils;

pub use utils::{
    download_page, markdown_to_text, save_markdown, save_text, txt_sanitize, url_to_filename,
};

#[derive(Debug)]
pub enum GitBookError {
    NetworkError(reqwest::Error),
    IoError(std::io::Error),
    InvalidUrl(String),
}

impl std::fmt::Display for GitBookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GitBookError::NetworkError(e) => write!(f, "Erreur réseau: {}", e),
            GitBookError::IoError(e) => write!(f, "Erreur I/O: {}", e),
            GitBookError::InvalidUrl(url) => write!(f, "URL invalide: {}", url),
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
