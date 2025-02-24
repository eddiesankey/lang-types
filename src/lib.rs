//! The enum [`Language`] with variants corresponding to a specific programming language with associated
//! file extensions and common names/aliases.
//!
//! [`Language`]: enum.Language.html
//!
//! **Crate features:**
//!
//! * `"serde"`
//!   Disabled by default. Enable to `#[derive(Serialize, Deserialize)]` for `Language`
//!

#![doc(html_root_url = "https://docs.rs/lang-types/1/")]

mod languages;
pub use languages::Language;
