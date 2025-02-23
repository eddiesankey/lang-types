//! A crate for working with programming language types and file extensions.
//!
//! Provides functionality to identify programming languages from file extensions
//! and names, as well as getting information about supported languages.

use std::collections::HashMap;
use std::sync::LazyLock;

use std::str::FromStr;
use std::fmt;

/// Represents a programming language.
///
/// Each variant corresponds to a specific programming language with associated
/// file extensions and common names/aliases.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Language {
    Python,
    JavaScript,
    TypeScript,
    Java,
    Cpp,
    CSharp,
    Rust,
    Go,
    Ruby,
    Php,
    Swift,
    Kotlin,
    C,
    R,
    Scala,
    Dart,
    Html,
    Shell,
}

static EXTENSION_MAP: LazyLock<HashMap<&'static str, Language>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("py", Language::Python);
    m.insert("pyi", Language::Python);
    m.insert("pyc", Language::Python);
    m.insert("pyw", Language::Python);
    m.insert("pyx", Language::Python);
    m.insert("js", Language::JavaScript);
    m.insert("mjs", Language::JavaScript);
    m.insert("cjs", Language::JavaScript);
    m.insert("jsx", Language::JavaScript);
    m.insert("ts", Language::TypeScript);
    m.insert("tsx", Language::TypeScript);
    m.insert("java", Language::Java);
    m.insert("class", Language::Java);
    m.insert("jar", Language::Java);
    m.insert("cpp", Language::Cpp);
    m.insert("cc", Language::Cpp);
    m.insert("cxx", Language::Cpp);
    m.insert("hpp", Language::Cpp);
    m.insert("hh", Language::Cpp);
    m.insert("h++", Language::Cpp);
    m.insert("hxx", Language::Cpp);
    m.insert("cs", Language::CSharp);
    m.insert("csx", Language::CSharp);
    m.insert("rs", Language::Rust);
    m.insert("go", Language::Go);
    m.insert("rb", Language::Ruby);
    m.insert("rake", Language::Ruby);
    m.insert("gemspec", Language::Ruby);
    m.insert("php", Language::Php);
    m.insert("php3", Language::Php);
    m.insert("php4", Language::Php);
    m.insert("php5", Language::Php);
    m.insert("phtml", Language::Php);
    m.insert("swift", Language::Swift);
    m.insert("kt", Language::Kotlin);
    m.insert("kts", Language::Kotlin);
    m.insert("c", Language::C);
    m.insert("h", Language::C);
    m.insert("r", Language::R);
    m.insert("R", Language::R);
    m.insert("Rmd", Language::R);
    m.insert("scala", Language::Scala);
    m.insert("sc", Language::Scala);
    m.insert("dart", Language::Dart);
    m.insert("html", Language::Html);
    m.insert("htm", Language::Html);
    m.insert("xhtml", Language::Html);
    m.insert("sh", Language::Shell);
    m.insert("bash", Language::Shell);
    m
});

static NAME_MAP: LazyLock<HashMap<&'static str, Language>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("python", Language::Python);
    m.insert("py", Language::Python);
    m.insert("python3", Language::Python);
    m.insert("javascript", Language::JavaScript);
    m.insert("js", Language::JavaScript);
    m.insert("node", Language::JavaScript);
    m.insert("nodejs", Language::JavaScript);
    m.insert("typescript", Language::TypeScript);
    m.insert("ts", Language::TypeScript);
    m.insert("java", Language::Java);
    m.insert("c++", Language::Cpp);
    m.insert("cpp", Language::Cpp);
    m.insert("cxx", Language::Cpp);
    m.insert("c#", Language::CSharp);
    m.insert("csharp", Language::CSharp);
    m.insert("cs", Language::CSharp);
    m.insert("rust", Language::Rust);
    m.insert("go", Language::Go);
    m.insert("golang", Language::Go);
    m.insert("ruby", Language::Ruby);
    m.insert("php", Language::Php);
    m.insert("swift", Language::Swift);
    m.insert("kotlin", Language::Kotlin);
    m.insert("c", Language::C);
    m.insert("r", Language::R);
    m.insert("scala", Language::Scala);
    m.insert("dart", Language::Dart);
    m.insert("html", Language::Html);
    m.insert("shell", Language::Shell);
    m.insert("bash", Language::Shell);
    m.insert("sh", Language::Shell);
    m
});

impl Language {
    pub fn from_extension(ext: &str) -> Option<Language> {
        let ext = ext.strip_prefix('.').unwrap_or(ext);
        EXTENSION_MAP.get(ext).copied()
    }

    pub fn from_name(name: &str) -> Option<Language> {
        NAME_MAP.get(name.to_lowercase().as_str()).copied()
    }

    pub fn name(&self) -> &'static str {
        match self {
            Language::Python => "Python",
            Language::JavaScript => "JavaScript",
            Language::TypeScript => "TypeScript",
            Language::Java => "Java",
            Language::Cpp => "C++",
            Language::CSharp => "C#",
            Language::Rust => "Rust",
            Language::Go => "Go",
            Language::Ruby => "Ruby",
            Language::Php => "PHP",
            Language::Swift => "Swift",
            Language::Kotlin => "Kotlin",
            Language::C => "C",
            Language::R => "R",
            Language::Scala => "Scala",
            Language::Dart => "Dart",
            Language::Html => "HTML",
            Language::Shell => "Shell",
        }
    }
    pub fn extensions(&self) -> Vec<&'static str> {
        EXTENSION_MAP.iter()
            .filter_map(|(ext, lang)| if lang == self { Some(*ext) } else { None })
            .collect()
    }
    pub fn all() -> &'static [Language] {
        static ALL: LazyLock<Vec<Language>> = LazyLock::new(|| vec![
            Language::Python,
            Language::JavaScript,
            Language::TypeScript,
            Language::Java,
            Language::Cpp,
            Language::CSharp,
            Language::Rust,
            Language::Go,
            Language::Ruby,
            Language::Php,
            Language::Swift,
            Language::Kotlin,
            Language::C,
            Language::R,
            Language::Scala,
            Language::Dart,
            Language::Html,
            Language::Shell,
        ]);
        &ALL
    }

}
impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_name(s).ok_or_else(|| format!("Unknown language: {}", s))
    }
}
