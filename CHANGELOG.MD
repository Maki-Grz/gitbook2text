# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.1] - 2025-11-12

### Changed

- Translated entire codebase and documentation to English
  - All source code comments now in English
  - All documentation strings now in English
  - CLI messages and error outputs now in English
  - README.md fully translated to English
  - CHANGELOG.md fully translated to English

### Fixed

- Fixed duplicate version declaration in Cargo.toml (removed duplicate "0.3.0" line)

## [0.3.0] - 2025-11-11

### Added

- **Automatic crawling**: New feature to automatically discover all pages of a GitBook
  - `is_gitbook()`: Checks if a URL points to a GitBook site
  - `extract_gitbook_links()`: Extracts all documentation links
  - `crawl_and_save()`: Crawls and saves links to a file
- **Improved CLI interface** with `clap`:
  - `gitbook2text all <URL>`: Full mode (crawl + download)
  - `gitbook2text crawl <URL>`: Crawl only mode
  - `gitbook2text download`: Download only mode
  - Support for legacy mode without subcommand for backward compatibility
- New dependencies:
  - `scraper` for HTML parsing
  - `url` for URL manipulation
  - `clap` for CLI interface

### Changed

- The `links.txt` file is now optional and can be generated automatically
- Improved project structure with a dedicated `crawler.rs` module
- More detailed progress messages during crawling
- Better error handling with more explicit messages

### Fixed

- Handling of URLs with or without trailing slash
- Filtering of links to non-documentation files (PDF, images, etc.)
- Avoids duplicates during crawling
- Better detection of GitBook domains

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
