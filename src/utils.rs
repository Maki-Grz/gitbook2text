use pulldown_cmark::{Event, Parser};
use regex::Regex;
use tokio::fs;

/// Télécharge le contenu d'une page depuis une URL
///
/// # Arguments
///
/// * `url` - L'URL de la page à télécharger
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
/// # Erreurs
///
/// Retourne une erreur si la requête HTTP échoue ou si la réponse ne peut pas être lue
pub async fn download_page(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?;
    let text = resp.text().await?;
    Ok(text)
}

/// Sauvegarde le contenu markdown dans un fichier
///
/// Le fichier sera créé dans le répertoire `data/md/` avec un nom basé sur l'URL
///
/// # Arguments
///
/// * `url` - L'URL source (utilisée pour générer le nom de fichier)
/// * `content` - Le contenu markdown à sauvegarder
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
/// # Erreurs
///
/// Retourne une erreur si l'écriture du fichier échoue
pub async fn save_markdown(url: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let filename = url_to_filename(url) + ".md";
    fs::write(format!("data/md/{}", filename), content).await?;
    Ok(())
}

/// Convertit une URL en nom de fichier sûr
///
/// Remplace les caractères `/` et `:` par des underscores
///
/// # Arguments
///
/// * `url` - L'URL à convertir
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

/// Convertit le markdown en texte brut
///
/// Extrait le texte des événements markdown en ignorant la mise en forme
///
/// # Arguments
///
/// * `md` - Le contenu markdown à convertir
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

/// Nettoie et sanitise le texte en retirant les balises GitBook spéciales
///
/// Retire les balises `{% code %}`, `{% endcode %}` et autres balises spéciales GitBook,
/// normalise les espaces et retire les tirets et guillemets.
///
/// # Arguments
///
/// * `txt` - Le texte à nettoyer
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

/// Sauvegarde le contenu texte dans un fichier
///
/// Le fichier sera créé dans le répertoire `data/txt/` avec un nom basé sur l'URL
///
/// # Arguments
///
/// * `url` - L'URL source (utilisée pour générer le nom de fichier)
/// * `content` - Le contenu texte à sauvegarder
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
/// # Erreurs
///
/// Retourne une erreur si l'écriture du fichier échoue
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
