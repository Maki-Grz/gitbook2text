# gitbook2text

[![Crates.io](https://img.shields.io/crates/v/gitbook2text.svg)](https://crates.io/crates/gitbook2text)
[![Documentation](https://docs.rs/gitbook2text/badge.svg)](https://docs.rs/gitbook2text)
[![License](https://img.shields.io/crates/l/gitbook2text.svg)](https://github.com/Maki-Grz/gitbook2text#license)

Un outil CLI et une bibliothèque Rust pour crawler des sites GitBook, télécharger leurs pages et les convertir en markdown et texte brut.

## ✨ Nouveautés v0.3.0

- 🕷️ **Crawling automatique** : Découvre automatiquement toutes les pages d'un GitBook
- ✅ **Vérification GitBook** : Détecte si un site est bien un GitBook avant de crawler
- 🚀 **Mode tout-en-un** : Crawl et télécharge en une seule commande
- 📋 **Interface CLI améliorée** : Sous-commandes claires avec `clap`

## 🚀 Installation

### En tant qu'outil CLI

```bash
cargo install gitbook2text
```

### En tant que dépendance

Ajoutez ceci à votre `Cargo.toml`:

```toml
[dependencies]
gitbook2text = "0.3"
```

## 📖 Usage

### CLI

#### Mode Complet (Recommandé)

Crawl et télécharge toutes les pages en une seule commande :

```bash
gitbook2text all https://docs.example.com
```

#### Mode Crawl uniquement

Génère le fichier `links.txt` avec tous les liens trouvés :

```bash
gitbook2text crawl https://docs.example.com

# Avec un fichier de sortie personnalisé
gitbook2text crawl https://docs.example.com -o my-links.txt
```

#### Mode Téléchargement uniquement

Télécharge les pages depuis un fichier de liens existant :

```bash
gitbook2text download

# Avec un fichier personnalisé
gitbook2text download -i my-links.txt
```

#### Mode Legacy (rétro-compatible)

Sans sous-commande, télécharge depuis `links.txt` :

```bash
gitbook2text
```

### Structure des fichiers générés

Les fichiers sont sauvegardés dans :

- `data/md/` - Fichiers markdown originaux
- `data/txt/` - Fichiers texte nettoyés

### Bibliothèque

#### Crawler un GitBook

```rust
use gitbook2text::{is_gitbook, extract_gitbook_links, crawl_and_save};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://docs.example.com";

    // Vérifier si c'est un GitBook
    if is_gitbook(url).await? {
        println!("C'est un GitBook !");

        // Extraire tous les liens
        let links = extract_gitbook_links(url).await?;
        println!("Trouvé {} pages", links.len());

        // Ou sauvegarder directement dans un fichier
        crawl_and_save(url, "links.txt").await?;
    }

    Ok(())
}
```

#### Télécharger et convertir

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

- ✅ **Crawling intelligent** : Découvre automatiquement toutes les pages d'une documentation
- ✅ **Vérification GitBook** : Détecte les sites GitBook via leurs marqueurs spécifiques
- ✅ **Téléchargement concurrent** : Traite plusieurs pages simultanément
- ✅ **Conversion markdown vers texte** : Extraction propre du contenu
- ✅ **Nettoyage avancé** : Retire les balises GitBook spéciales
- ✅ **Support des blocs de code** : Préserve les titres et le contenu
- ✅ **Normalisation** : Espaces et caractères uniformisés

## 🎯 Cas d'usage

- 📚 Archiver une documentation complète
- 🔍 Indexer du contenu pour un moteur de recherche
- 🤖 Préparer des données pour l'entraînement de modèles
- 📊 Analyser la structure d'une documentation
- 💾 Créer des backups de documentations

## 📋 Exemples pratiques

### Archiver une documentation complète

```bash
# Tout en un
gitbook2text all https://docs.mydomain.com

# Ou étape par étape
gitbook2text crawl https://docs.mydomain.com
gitbook2text download
```

### Utiliser avec un workflow automatisé

```bash
#!/bin/bash
# backup-docs.sh

GITBOOK_URL="https://docs.example.com"
BACKUP_DIR="backups/$(date +%Y-%m-%d)"

mkdir -p "$BACKUP_DIR"
cd "$BACKUP_DIR"

gitbook2text all "$GITBOOK_URL"

echo "Backup terminé dans $BACKUP_DIR"
```

## 📚 API Documentation

Pour la documentation complète de l'API, visitez [docs.rs/gitbook2text](https://docs.rs/gitbook2text).

## 🤝 Contribuer

Les contributions sont les bienvenues ! N'hésitez pas à ouvrir une issue ou une pull request.

## 📝 Changelog

Voir [CHANGELOG.md](CHANGELOG.md) pour l'historique des versions.

## 📄 License

Ce projet est sous double licence MIT ou Apache-2.0, à votre choix.

- MIT License ([LICENSE-MIT](LICENSE-MIT) ou http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) ou http://www.apache.org/licenses/LICENSE-2.0)

## 🔗 Liens

- [Crates.io](https://crates.io/crates/gitbook2text)
- [Documentation](https://docs.rs/gitbook2text)
- [Repository](https://github.com/Maki-Grz/gitbook2text)
- [Issues](https://github.com/Maki-Grz/gitbook2text/issues)
