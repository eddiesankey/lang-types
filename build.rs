use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

const LIB_DOC: &str = r#"//! A crate for working with programming language types and file extensions.
//!
//! Provides functionality to identify programming languages from file extensions
//! and names, as well as getting information about supported languages.
"#;

const LANG_DOC: &str = r#"/// Represents a programming language.
///
/// Each variant corresponds to a specific programming language with associated
/// file extensions and common names/aliases.
"#;

#[derive(Deserialize, Serialize)]
struct Language {
    name: String,
    enum_name: Option<String>,
    extensions: Vec<String>,
    aliases: Vec<String>,
}

fn generate_code(languages: &[Language]) -> String {
    let mut code = String::new();

    code.push_str(LIB_DOC);
    code.push_str("\n");

    // imports
    code.push_str("use std::collections::HashMap;\n");
    code.push_str("use std::sync::LazyLock;\n\n");
    code.push_str("use std::str::FromStr;\n");
    code.push_str("use std::fmt;\n\n");

    code.push_str(LANG_DOC);
    code.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n");
    code.push_str(
        "#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]\n",
    );
    code.push_str("pub enum Language {\n");
    for lang in languages {
        let enum_name = lang.enum_name.clone().unwrap_or(lang.name.clone());
        code.push_str(&format!("    {},\n", enum_name));
    }
    code.push_str("}\n\n");

    // generate extension map
    code.push_str(
        "static EXTENSION_MAP: LazyLock<HashMap<&'static str, Language>> = LazyLock::new(|| {\n",
    );
    code.push_str("    let mut m = HashMap::new();\n");
    for lang in languages {
        for ext in &lang.extensions {
            let ext = ext.trim_start_matches('.');
            code.push_str(&format!(
                "    m.insert(\"{}\", Language::{});\n",
                ext,
                lang.enum_name.clone().unwrap_or(lang.name.clone())
            ));
        }
    }
    code.push_str("    m\n");
    code.push_str("});\n\n");

    // generate name map
    code.push_str(
        "static NAME_MAP: LazyLock<HashMap<&'static str, Language>> = LazyLock::new(|| {\n",
    );
    code.push_str("    let mut m = HashMap::new();\n");
    for lang in languages {
        let variant = lang.enum_name.clone().unwrap_or(lang.name.clone());
        // Add canonical name
        code.push_str(&format!(
            "    m.insert(\"{}\", Language::{});\n",
            lang.name.to_lowercase(),
            variant
        ));
        // Add aliases
        for alias in &lang.aliases {
            code.push_str(&format!(
                "    m.insert(\"{}\", Language::{});\n",
                alias.to_lowercase(),
                variant
            ));
        }
    }
    code.push_str("    m\n");
    code.push_str("});\n\n");

    // generate implementations
    code.push_str("impl Language {\n");

    // from_extension
    code.push_str("    pub fn from_extension(ext: &str) -> Option<Language> {\n");
    code.push_str("        let ext = ext.strip_prefix('.').unwrap_or(ext);\n");
    code.push_str("        EXTENSION_MAP.get(ext).copied()\n");
    code.push_str("    }\n\n");

    // from_name
    code.push_str("    pub fn from_name(name: &str) -> Option<Language> {\n");
    code.push_str("        NAME_MAP.get(name.to_lowercase().as_str()).copied()\n");
    code.push_str("    }\n\n");

    // name method
    code.push_str("    pub fn name(&self) -> &'static str {\n");
    code.push_str("        match self {\n");
    for lang in languages {
        let variant = lang.enum_name.clone().unwrap_or(lang.name.clone());
        code.push_str(&format!(
            "            Language::{} => \"{}\",\n",
            variant, lang.name
        ));
    }
    code.push_str("        }\n");
    code.push_str("    }\n");

    // extensions method
    code.push_str("    pub fn extensions(&self) -> Vec<&'static str> {\n");
    code.push_str("        EXTENSION_MAP.iter()\n");
    code.push_str(
        "            .filter_map(|(ext, lang)| if lang == self { Some(*ext) } else { None })\n",
    );
    code.push_str("            .collect()\n");
    code.push_str("    }\n");

    // all method
    code.push_str("    pub fn all() -> &'static [Language] {\n");
    code.push_str("        static ALL: LazyLock<Vec<Language>> = LazyLock::new(|| vec![\n");
    for lang in languages {
        let variant = lang.enum_name.clone().unwrap_or(lang.name.clone());
        code.push_str(&format!("            Language::{},\n", variant));
    }
    code.push_str("        ]);\n");
    code.push_str("        &ALL\n");
    code.push_str("    }\n\n");

    code.push_str("}\n");

    // add trait impls

    // Display impl
    code.push_str("impl fmt::Display for Language {\n");
    code.push_str("    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {\n");
    code.push_str("        write!(f, \"{}\", self.name())\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");

    // FromStr impl
    code.push_str("impl FromStr for Language {\n");
    code.push_str("    type Err = String;\n\n");
    code.push_str("    fn from_str(s: &str) -> Result<Self, Self::Err> {\n");
    code.push_str(
        "        Self::from_name(s).ok_or_else(|| format!(\"Unknown language: {}\", s))\n",
    );
    code.push_str("    }\n");
    code.push_str("}\n");

    code
}

fn main() {
    let json_str = fs::read_to_string("languages.json").expect("Failed to read languages.json");

    let languages: Vec<Language> =
        serde_json::from_str(&json_str).expect("Failed to parse languages.json");

    let generated_code = generate_code(&languages);

    let dest_path = Path::new("src").join("languages.rs");
    fs::write(&dest_path, generated_code).unwrap();

    println!("cargo:rerun-if-changed=languages.json");
}
