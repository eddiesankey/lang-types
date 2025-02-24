//! A crate for working with programming language types and file extensions.
//!
//! Provides functionality to identify programming languages from file extensions
//! and names, as well as getting information about supported languages.

use std::collections::HashMap;
use std::sync::LazyLock;

use std::fmt;
use std::str::FromStr;

/// Represents a programming language.
///
/// Each variant corresponds to a specific programming language with associated
/// file extensions and common names/aliases.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Language {
    C,
    CSharp,
    Cpp,
    Clojure,
    Dart,
    Elixir,
    Erlang,
    Go,
    Html,
    Haskell,
    Java,
    JavaScript,
    Julia,
    Kotlin,
    Lua,
    OCaml,
    Php,
    Perl,
    Python,
    R,
    Ruby,
    Rust,
    Scala,
    Shell,
    Swift,
    TypeScript,
    Zig,
}

static EXTENSION_MAP: LazyLock<HashMap<&'static str, Language>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("c", Language::C);
    m.insert("h", Language::C);
    m.insert("cs", Language::CSharp);
    m.insert("csx", Language::CSharp);
    m.insert("cpp", Language::Cpp);
    m.insert("cc", Language::Cpp);
    m.insert("cxx", Language::Cpp);
    m.insert("hpp", Language::Cpp);
    m.insert("hh", Language::Cpp);
    m.insert("h++", Language::Cpp);
    m.insert("hxx", Language::Cpp);
    m.insert("clj", Language::Clojure);
    m.insert("cljs", Language::Clojure);
    m.insert("cljc", Language::Clojure);
    m.insert("edn", Language::Clojure);
    m.insert("dart", Language::Dart);
    m.insert("ex", Language::Elixir);
    m.insert("exs", Language::Elixir);
    m.insert("erl", Language::Erlang);
    m.insert("hrl", Language::Erlang);
    m.insert("go", Language::Go);
    m.insert("html", Language::Html);
    m.insert("htm", Language::Html);
    m.insert("xhtml", Language::Html);
    m.insert("hs", Language::Haskell);
    m.insert("lhs", Language::Haskell);
    m.insert("hsc", Language::Haskell);
    m.insert("java", Language::Java);
    m.insert("class", Language::Java);
    m.insert("jar", Language::Java);
    m.insert("js", Language::JavaScript);
    m.insert("mjs", Language::JavaScript);
    m.insert("cjs", Language::JavaScript);
    m.insert("jsx", Language::JavaScript);
    m.insert("jl", Language::Julia);
    m.insert("kt", Language::Kotlin);
    m.insert("kts", Language::Kotlin);
    m.insert("lua", Language::Lua);
    m.insert("luau", Language::Lua);
    m.insert("ml", Language::OCaml);
    m.insert("mli", Language::OCaml);
    m.insert("mll", Language::OCaml);
    m.insert("mly", Language::OCaml);
    m.insert("php", Language::Php);
    m.insert("php3", Language::Php);
    m.insert("php4", Language::Php);
    m.insert("php5", Language::Php);
    m.insert("phtml", Language::Php);
    m.insert("pl", Language::Perl);
    m.insert("pm", Language::Perl);
    m.insert("t", Language::Perl);
    m.insert("pod", Language::Perl);
    m.insert("py", Language::Python);
    m.insert("pyi", Language::Python);
    m.insert("pyc", Language::Python);
    m.insert("pyw", Language::Python);
    m.insert("pyx", Language::Python);
    m.insert("r", Language::R);
    m.insert("R", Language::R);
    m.insert("Rmd", Language::R);
    m.insert("rb", Language::Ruby);
    m.insert("rake", Language::Ruby);
    m.insert("gemspec", Language::Ruby);
    m.insert("rs", Language::Rust);
    m.insert("scala", Language::Scala);
    m.insert("sc", Language::Scala);
    m.insert("sh", Language::Shell);
    m.insert("bash", Language::Shell);
    m.insert("swift", Language::Swift);
    m.insert("ts", Language::TypeScript);
    m.insert("tsx", Language::TypeScript);
    m.insert("zig", Language::Zig);
    m
});

static NAME_MAP: LazyLock<HashMap<&'static str, Language>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("c", Language::C);
    m.insert("c#", Language::CSharp);
    m.insert("csharp", Language::CSharp);
    m.insert("cs", Language::CSharp);
    m.insert("c++", Language::Cpp);
    m.insert("cpp", Language::Cpp);
    m.insert("cxx", Language::Cpp);
    m.insert("clojure", Language::Clojure);
    m.insert("clj", Language::Clojure);
    m.insert("dart", Language::Dart);
    m.insert("elixir", Language::Elixir);
    m.insert("ex", Language::Elixir);
    m.insert("erlang", Language::Erlang);
    m.insert("erl", Language::Erlang);
    m.insert("go", Language::Go);
    m.insert("golang", Language::Go);
    m.insert("html", Language::Html);
    m.insert("haskell", Language::Haskell);
    m.insert("hs", Language::Haskell);
    m.insert("java", Language::Java);
    m.insert("javascript", Language::JavaScript);
    m.insert("js", Language::JavaScript);
    m.insert("node", Language::JavaScript);
    m.insert("nodejs", Language::JavaScript);
    m.insert("julia", Language::Julia);
    m.insert("julia-lang", Language::Julia);
    m.insert("kotlin", Language::Kotlin);
    m.insert("lua", Language::Lua);
    m.insert("lua-lang", Language::Lua);
    m.insert("ocaml", Language::OCaml);
    m.insert("ml", Language::OCaml);
    m.insert("php", Language::Php);
    m.insert("perl", Language::Perl);
    m.insert("pl", Language::Perl);
    m.insert("perl5", Language::Perl);
    m.insert("perl6", Language::Perl);
    m.insert("python", Language::Python);
    m.insert("py", Language::Python);
    m.insert("python3", Language::Python);
    m.insert("r", Language::R);
    m.insert("ruby", Language::Ruby);
    m.insert("rust", Language::Rust);
    m.insert("scala", Language::Scala);
    m.insert("shell", Language::Shell);
    m.insert("bash", Language::Shell);
    m.insert("sh", Language::Shell);
    m.insert("swift", Language::Swift);
    m.insert("typescript", Language::TypeScript);
    m.insert("ts", Language::TypeScript);
    m.insert("zig", Language::Zig);
    m.insert("zig-lang", Language::Zig);
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
            Language::C => "C",
            Language::CSharp => "C#",
            Language::Cpp => "C++",
            Language::Clojure => "Clojure",
            Language::Dart => "Dart",
            Language::Elixir => "Elixir",
            Language::Erlang => "Erlang",
            Language::Go => "Go",
            Language::Html => "HTML",
            Language::Haskell => "Haskell",
            Language::Java => "Java",
            Language::JavaScript => "JavaScript",
            Language::Julia => "Julia",
            Language::Kotlin => "Kotlin",
            Language::Lua => "Lua",
            Language::OCaml => "OCaml",
            Language::Php => "PHP",
            Language::Perl => "Perl",
            Language::Python => "Python",
            Language::R => "R",
            Language::Ruby => "Ruby",
            Language::Rust => "Rust",
            Language::Scala => "Scala",
            Language::Shell => "Shell",
            Language::Swift => "Swift",
            Language::TypeScript => "TypeScript",
            Language::Zig => "Zig",
        }
    }
    pub fn extensions(&self) -> Vec<&'static str> {
        EXTENSION_MAP
            .iter()
            .filter_map(|(ext, lang)| if lang == self { Some(*ext) } else { None })
            .collect()
    }
    pub fn all() -> &'static [Language] {
        static ALL: LazyLock<Vec<Language>> = LazyLock::new(|| {
            vec![
                Language::C,
                Language::CSharp,
                Language::Cpp,
                Language::Clojure,
                Language::Dart,
                Language::Elixir,
                Language::Erlang,
                Language::Go,
                Language::Html,
                Language::Haskell,
                Language::Java,
                Language::JavaScript,
                Language::Julia,
                Language::Kotlin,
                Language::Lua,
                Language::OCaml,
                Language::Php,
                Language::Perl,
                Language::Python,
                Language::R,
                Language::Ruby,
                Language::Rust,
                Language::Scala,
                Language::Shell,
                Language::Swift,
                Language::TypeScript,
                Language::Zig,
            ]
        });
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
