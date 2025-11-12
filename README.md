# gitbook2text

[![Crates.io](https://img.shields.io/crates/v/gitbook2text.svg)](https://crates.io/crates/gitbook2text)
[![Documentation](https://docs.rs/gitbook2text/badge.svg)](https://docs.rs/gitbook2text)
[![License](https://img.shields.io/crates/l/gitbook2text.svg)](https://github.com/Maki-Grz/gitbook2text#license)

A CLI tool and a Rust library for crawling GitBook sites, downloading their pages, and converting them to Markdown and plain text.

## ✨ What's New v0.3.0

- 🕷️ **Automatic Crawling**: Automatically discovers all pages of a GitBook
- ✅ **GitBook Verification**: Detects if a site is indeed a GitBook before crawling
- 🚀 **All-in-One Mode**: Crawl and download in a single command
- 📋 **Improved CLI Interface**: Clear subcommands with `clap`

## 🚀 Installation

### As a CLI Tool

```bash
cargo install gitbook2text
```

### As a Dependency

Add this to your `Cargo.toml`:

```toml
[dependencies]
gitbook2text = "0.3"
```

## 📖 Usage

### CLI

#### Full Mode (Recommended)

Crawls and downloads all pages in a single command:

```bash
gitbook2text all https://docs.example.com
```

#### Crawl Only Mode

Generates the `links.txt` file with all found links:

```bash
gitbook2text crawl https://docs.example.com

# With a custom output file
gitbook2text crawl https://docs.example.com -o my-links.txt
```

#### Download Only Mode

Downloads pages from an existing links file:

```bash
gitbook2text download

# With a custom file
gitbook2text download -i my-links.txt
```

#### Legacy Mode (Backward Compatible)

Without a subcommand, downloads from `links.txt`:

```bash
gitbook2text
```

### Structure of Generated Files

Files are saved in:

- `data/md/` - Original markdown files
- `data/txt/` - Cleaned text files

### Library

#### Crawling a GitBook

```rust
use gitbook2text::{is_gitbook, extract_gitbook_links, crawl_and_save};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
let url = "https://docs.example.com";

// Check if it's a GitBook
if is_gitbook(url).await? {
println!("It's a GitBook!");

// Extract all links
let links = extract_gitbook_links(url).await?;
println!("Found {} pages", links.len());

// Or directly save to a file
crawl_and_save(url, "links.txt").await?;
}

Ok(())
}
```

#### Download and Convert

```rust
use gitbook2text::{download_page, markdown_to_text, txt_sanitize};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
let url = "https://docs.example.com/page.md";

// Download the page
let content = download_page(url).await?;

// Convert to text
let text = markdown_to_text(&content);

// Clean the text
let cleaned = txt_sanitize(&text);

println!("{}", cleaned);
Ok(())
}
```

## 🔧 Features

- ✅ **Smart crawling**: Automatically discovers all pages of a documentation
- ✅ **GitBook verification**: Detects GitBook sites via their specific markers
- ✅ **Concurrent downloading**: Processes multiple pages simultaneously
- ✅ **Markdown to text conversion**: Clean content extraction
- ✅ **Advanced cleaning**: Removes special GitBook tags
- ✅ **Code block support**: Preserves titles and content
- ✅ **Normalization**: Uniform spaces and characters

## 🎯 Use cases

- 📚 Archive a complete documentation
- 🔍 Index content for a search engine
- 🤖 Prepare data for model training
- 📊 Analyze the structure of documentation
- 💾 Create documentation backups

## 📋 Practical Examples

### Archiving Complete Documentation

```bash
# All in one
gitbook2text all https://docs.mydomain.com

# Or step by step
gitbook2text crawl https://docs.mydomain.com
gitbook2text download
```

### Use with an automated workflow

```bash
#!/bin/bash
# backup-docs.sh

GITBOOK_URL="https://docs.example.com"
BACKUP_DIR="backups/$(date +%Y-%m-%d)"

mkdir -p "$BACKUP_DIR"
cd "$BACKUP_DIR"

gitbook2text all "$GITBOOK_URL"

echo "Backup completed in $BACKUP_DIR"
```

## 📚 API Documentation

For the full API documentation, visit [docs.rs/gitbook2text](https://docs.rs/gitbook2text).

## 🤝 Contribute

Contributions are welcome! Feel free to open an issue or a pull request.

## 📝 Changelog

See [CHANGELOG.md](CHANGELOG.md) for the version history.

## 📄 License

This project is dual-licensed under MIT or Apache-2.0, your choice.

- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

## 🔗 Links

- [Crates.io](https://crates.io/crates/gitbook2text)
- [Documentation](https://docs.rs/gitbook2text)
- [Repository](https://github.com/Maki-Grz/gitbook2text)
- [Issues](https://github.com/Maki-Grz/gitbook2text/issues)
