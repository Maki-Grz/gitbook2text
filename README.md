# gitbook2text

[![Crates.io](https://img.shields.io/crates/v/gitbook2text.svg)](https://crates.io/crates/gitbook2text)
[![Documentation](https://docs.rs/gitbook2text/badge.svg)](https://docs.rs/gitbook2text)
[![License](https://img.shields.io/crates/l/gitbook2text.svg)](https://github.com/Maki-Grz/gitbook2text#license)

Un outil CLI et une bibliothèque Rust pour télécharger des pages GitBook et les convertir en markdown et texte brut.

## 🚀 Installation

### En tant qu'outil CLI

```bash
cargo install gitbook2text
```

### En tant que dépendance

Ajoutez ceci à votre `Cargo.toml`:

```toml
[dependencies]
gitbook2text = "0.2"
```

## 📖 Usage

### CLI

1. Créez un fichier `links.txt` contenant les URLs des pages GitBook (une par ligne):

```text
https://docs.example.com/introduction
https://docs.example.com/getting-started
```

2. Exécutez la commande:

```bash
gitbook2text
```

Les fichiers seront sauvegardés dans:
- `data/md/` - Fichiers markdown originaux
- `data/txt/` - Fichiers texte nettoyés

### Bibliothèque

```rust
use gitbook2text::{download_page, markdown_to_text, txt_sanitize};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://docs.example.com/page.md";
    
    // Télécharger la page
    let content = download_page(url).await?;
    
    // Convertir en texte
    let text = markdown_to_text(&content);
    
    // Nettoyer le texte
    let cleaned = txt_sanitize(&text);
    
    println!("{}", cleaned);
    Ok(())
}
```

## 🔧 Fonctionnalités

- ✅ Téléchargement concurrent de multiples pages
- ✅ Conversion markdown vers texte brut
- ✅ Nettoyage des balises GitBook spéciales
- ✅ Support des blocs de code avec titres
- ✅ Normalisation des espaces et caractères

## 📝 API Documentation

Pour la documentation complète de l'API, visitez [docs.rs/gitbook2text](https://docs.rs/gitbook2text).

## 🤝 Contribuer

Les contributions sont les bienvenues! N'hésitez pas à ouvrir une issue ou une pull request.

## 📄 License

Ce projet est sous double licence MIT ou Apache-2.0, à votre choix.

- MIT License ([LICENSE-MIT](LICENSE-MIT) ou http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) ou http://www.apache.org/licenses/LICENSE-2.0)

## 🔗 Liens

- [Crates.io](https://crates.io/crates/gitbook2text)
- [Documentation](https://docs.rs/gitbook2text)
- [Repository](https://github.com/Maki-Grz/gitbook2text)
- [Issues](https://github.com/Maki-Grz/gitbook2text/issues)