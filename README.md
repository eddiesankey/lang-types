[<img alt="crates.io" src="https://img.shields.io/crates/v/lang-types.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/lang-types)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-lang-types-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/lang-types)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/eddiesankey/lang-types/ci.yml?branch=main&style=for-the-badge" height="20">](https://github.com/eddiesankey/lang-types/actions?query=branch%3Amain)

# lang-types
This crate provides the Language enum for programming language identification and file extension mapping.

## Features
- File extension to language mapping
- Language name and alias lookup
- Optional serde support for serialization

## Usage

Add to your Cargo.toml:
```toml
[dependencies]
lang-types = "0.1"

# or

[dependencies]
lang-types = { version = "0.1", features = ["serde"] }
```

Basic usage

```rust
use lang_types::Language;

// Get a language from file extension
assert_eq!(Language::from_extension(".rs"), Some(Language::Rust));
assert_eq!(Language::from_extension("py"), Some(Language::Python));

// Get language from common name
assert_eq!(Language::from_name("python"), Some(Language::Python));
assert_eq!(Language::from_name("python3"), Some(Language::Python));

```

## Contributing new languages
To add support for a new programming language, submit a pull request modifying the languages.json file in the root directory.
Each language entry should follow this format:

```json
{
    "name": "Python",        // Display name of the language
    "enum_name": "Python",   // Optional: Name to use in the Rust enum (defaults to name)
    "extensions": [
        "py",
        "pyi",
        "pyc"
    ],
    "aliases": [            // Alternative names for the language
        "py",
        "python3"
    ]
}
```

Guidelines for new languages
- The language should be actively used (either historically significant or currently maintained)
- File extensions should be unique where possible
- Provide common aliases
- If the display name contains special characters (e.g., "C++"), provide a Rust-compatible enum_name (e.g., "Cpp")
