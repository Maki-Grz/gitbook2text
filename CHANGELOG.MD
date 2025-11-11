# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2025-11-11

### Added

- **Crawling automatique** : Nouvelle fonctionnalité pour découvrir automatiquement toutes les pages d'un GitBook
  - `is_gitbook()` : Vérifie si une URL pointe vers un site GitBook
  - `extract_gitbook_links()` : Extrait tous les liens de documentation
  - `crawl_and_save()` : Crawl et sauvegarde les liens dans un fichier
- **Interface CLI améliorée** avec `clap` :
  - `gitbook2text all <URL>` : Mode complet (crawl + téléchargement)
  - `gitbook2text crawl <URL>` : Mode crawl uniquement
  - `gitbook2text download` : Mode téléchargement uniquement
  - Support de l'ancien mode sans sous-commande pour la rétro-compatibilité
- Nouvelles dépendances :
  - `scraper` pour le parsing HTML
  - `url` pour la manipulation d'URLs
  - `clap` pour l'interface CLI

### Changed

- Le fichier `links.txt` est maintenant optionnel et peut être généré automatiquement
- Amélioration de la structure du projet avec un module `crawler.rs` dédié
- Messages de progression plus détaillés pendant le crawling
- Meilleure gestion des erreurs avec des messages plus explicites

### Fixed

- Gestion des URLs avec ou sans trailing slash
- Filtrage des liens vers des fichiers non-documentation (PDF, images, etc.)
- Évite les doublons lors du crawling
- Meilleure détection des domaines GitBook

## [0.2.2] - 2025-11-10

### Added

- Initial public release
- CLI tool for downloading GitBook pages
- Library functions for markdown conversion
- Text sanitization utilities

### Changed

- Improved error handling
- Added comprehensive documentation

### Fixed

- URL filename conversion edge cases
