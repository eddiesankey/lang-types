use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    Abap,
    AgsScript,
    Ampl,
    Antlr,
    Apl,
    Asp,
    Ats,
    ActionScript,
    Ada,
    Agda,
    Alloy,
    Apex,
    AppleScript,
    Arc,
    Arduino,
    AspectJ,
    Assembly,
    Augeas,
    AutoHotkey,
    AutoIt,
    Awk,
    Batchfile,
    Befunge,
    Bison,
    BitBake,
    BlitzBasic,
    BlitzMax,
    Bluespec,
    Boo,
    Brainfuck,
    Brightscript,
    Bro,
    C,
    CSharp,
    Cpp,
    C2hsHaskell,
    Clips,
    CMake,
    Cobol,
    CapnProto,
    CartoCss,
    Ceylon,
    Chapel,
    Charity,
    Chuck,
    Cirru,
    Clarion,
    Clean,
    Click,
    Clojure,
    CoffeeScript,
    ColdFusion,
    ColdFusionCfc,
    CommonLisp,
    ComponentPascal,
    Cool,
    Coq,
    Crystal,
    Cucumber,
    Cuda,
    Cycript,
    Cython,
    D,
    DigitalCommandLanguage,
    Dm,
    DTrace,
    Dart,
    Dogescript,
    Dylan,
    E,
    Ecl,
    Eclipse,
    Eiffel,
    Elixir,
    Elm,
    EmacsLisp,
    EmberScript,
    Erlang,
    FSharp,
    Flux,
    Fortran,
    Factor,
    Fancy,
    Fantom,
    Filterscript,
    Forth,
    FreeMarker,
    Frege,
    Gams,
    Gap,
    Gas,
    GdScript,
    Glsl,
    GameMakerLanguage,
    Genshi,
    GentooEbuild,
    GentooEclass,
    Glyph,
    Gnuplot,
    Go,
    Golo,
    Gosu,
    Grace,
    GrammaticalFramework,
    Groovy,
    GroovyServerPages,
    Hcl,
    Hlsl,
    Hack,
    Harbour,
    Haskell,
    Haxe,
    Hy,
    HyPhy,
    Idl,
    IgorPro,
    Idris,
    Inform7,
    InnoSetup,
    Io,
    Ioke,
    Isabelle,
    J,
    JFlex,
    Jsoniq,
    Jsx,
    Jasmin,
    Java,
    JavaServerPages,
    JavaScript,
    Julia,
    Krl,
    KiCad,
    Kotlin,
    Lfe,
    Llvm,
    Lolcode,
    Lsl,
    LabView,
    Lasso,
    Lean,
    Lex,
    LilyPond,
    Limbo,
    LiterateAgda,
    LiterateCoffeeScript,
    LiterateHaskell,
    LiveScript,
    Logos,
    Logtalk,
    LookMl,
    LoomScript,
    Lua,
    M,
    M4,
    M4Sugar,
    MaxScript,
    Muf,
    Makefile,
    Mako,
    Mathematica,
    Matlab,
    Max,
    Mercury,
    Metal,
    MiniD,
    Mirah,
    Modelica,
    Modula2,
    ModuleManagementSystem,
    Monkey,
    Moocode,
    MoonScript,
    Myghty,
    Ncl,
    Nsis,
    Nemerle,
    NetLinx,
    NetLinxErb,
    NetLogo,
    NewLisp,
    Nimrod,
    Nit,
    Nix,
    Nu,
    NumPy,
    OCaml,
    ObjectiveC,
    ObjectiveCpp,
    ObjectiveJ,
    Omgrofl,
    Opa,
    Opal,
    OpenCl,
    OpenEdgeAbl,
    OpenScad,
    Ox,
    Oxygene,
    Oz,
    Pawn,
    Php,
    Plsql,
    PlPgSql,
    PovRaySdl,
    Pan,
    Papyrus,
    Parrot,
    ParrotAssembly,
    ParrotInternalRepresentation,
    Pascal,
    Perl,
    Perl6,
    PicoLisp,
    PigLatin,
    Pike,
    PogoScript,
    Pony,
    PowerShell,
    Processing,
    Prolog,
    PropellerSpin,
    Puppet,
    PureData,
    PureBasic,
    PureScript,
    Python,
    Qml,
    QMake,
    R,
    RealBasic,
    Racket,
    RagelInRubyHost,
    Rebol,
    Red,
    Redcode,
    RenPy,
    RenderScript,
    RobotFramework,
    Rouge,
    Ruby,
    Rust,
    Sas,
    Smt,
    Sqf,
    Sqlpl,
    Sage,
    SaltStack,
    Scala,
    Scheme,
    Scilab,
    Self_,
    Shell,
    ShellSession,
    Shen,
    Slash,
    Smali,
    Smalltalk,
    Smarty,
    SourcePawn,
    Squirrel,
    Stan,
    StandardMl,
    Stata,
    SuperCollider,
    Swift,
    SystemVerilog,
    Txl,
    Tcl,
    Tcsh,
    Terra,
    Thrift,
    Turing,
    TypeScript,
    UnifiedParallelC,
    Uno,
    UnrealScript,
    UrWeb,
    Vcl,
    Vhdl,
    Vala,
    Verilog,
    VimL,
    VisualBasic,
    Volt,
    WebIdl,
    X10,
    Xc,
    XPages,
    XProc,
    XQuery,
    Xs,
    Xslt,
    Xojo,
    Xtend,
    Yacc,
    Zephir,
    Zig,
    Zimpl,
    EC,
    Fish,
    Mupad,
    NesC,
    Ooc,
    Wisp,
    XBase,
}

static EXTENSION_MAP: LazyLock<HashMap<&'static str, Language>> = LazyLock::new(|| {
    let mut m = HashMap::new();

    m.insert("abap", Language::Abap);
    m.insert("asc", Language::AgsScript);
    m.insert("ash", Language::AgsScript);
    m.insert("ampl", Language::Ampl);
    m.insert("mod", Language::Ampl);
    m.insert("g4", Language::Antlr);
    m.insert("apl", Language::Apl);
    m.insert("dyalog", Language::Apl);
    m.insert("asp", Language::Asp);
    m.insert("asax", Language::Asp);
    m.insert("ascx", Language::Asp);
    m.insert("ashx", Language::Asp);
    m.insert("asmx", Language::Asp);
    m.insert("aspx", Language::Asp);
    m.insert("axd", Language::Asp);
    m.insert("dats", Language::Ats);
    m.insert("hats", Language::Ats);
    m.insert("sats", Language::Ats);
    m.insert("as", Language::ActionScript);
    m.insert("adb", Language::Ada);
    m.insert("ada", Language::Ada);
    m.insert("ads", Language::Ada);
    m.insert("agda", Language::Agda);
    m.insert("als", Language::Alloy);
    m.insert("cls", Language::Apex);
    m.insert("applescript", Language::AppleScript);
    m.insert("scpt", Language::AppleScript);
    m.insert("arc", Language::Arc);
    m.insert("ino", Language::Arduino);
    m.insert("aj", Language::AspectJ);
    m.insert("asm", Language::Assembly);
    m.insert("a51", Language::Assembly);
    m.insert("inc", Language::Assembly);
    m.insert("nasm", Language::Assembly);
    m.insert("aug", Language::Augeas);
    m.insert("ahk", Language::AutoHotkey);
    m.insert("ahkl", Language::AutoHotkey);
    m.insert("au3", Language::AutoIt);
    m.insert("awk", Language::Awk);
    m.insert("auk", Language::Awk);
    m.insert("gawk", Language::Awk);
    m.insert("mawk", Language::Awk);
    m.insert("nawk", Language::Awk);
    m.insert("bat", Language::Batchfile);
    m.insert("cmd", Language::Batchfile);
    m.insert("befunge", Language::Befunge);
    m.insert("bison", Language::Bison);
    m.insert("bb", Language::BitBake);
    m.insert("bb", Language::BlitzBasic);
    m.insert("decls", Language::BlitzBasic);
    m.insert("bmx", Language::BlitzMax);
    m.insert("bsv", Language::Bluespec);
    m.insert("boo", Language::Boo);
    m.insert("b", Language::Brainfuck);
    m.insert("bf", Language::Brainfuck);
    m.insert("brs", Language::Brightscript);
    m.insert("bro", Language::Bro);
    m.insert("c", Language::C);
    m.insert("cats", Language::C);
    m.insert("h", Language::C);
    m.insert("idc", Language::C);
    m.insert("w", Language::C);
    m.insert("cs", Language::CSharp);
    m.insert("cake", Language::CSharp);
    m.insert("cshtml", Language::CSharp);
    m.insert("csx", Language::CSharp);
    m.insert("cpp", Language::Cpp);
    m.insert("c++", Language::Cpp);
    m.insert("cc", Language::Cpp);
    m.insert("cp", Language::Cpp);
    m.insert("cxx", Language::Cpp);
    m.insert("h++", Language::Cpp);
    m.insert("hh", Language::Cpp);
    m.insert("hpp", Language::Cpp);
    m.insert("hxx", Language::Cpp);
    m.insert("inc", Language::Cpp);
    m.insert("inl", Language::Cpp);
    m.insert("ipp", Language::Cpp);
    m.insert("tcc", Language::Cpp);
    m.insert("tpp", Language::Cpp);
    m.insert("chs", Language::C2hsHaskell);
    m.insert("clp", Language::Clips);
    m.insert("cmake", Language::CMake);
    m.insert("cmake.in", Language::CMake);
    m.insert("cob", Language::Cobol);
    m.insert("cbl", Language::Cobol);
    m.insert("ccp", Language::Cobol);
    m.insert("cobol", Language::Cobol);
    m.insert("cpy", Language::Cobol);
    m.insert("capnp", Language::CapnProto);
    m.insert("mss", Language::CartoCss);
    m.insert("ceylon", Language::Ceylon);
    m.insert("chpl", Language::Chapel);
    m.insert("ch", Language::Charity);
    m.insert("ck", Language::Chuck);
    m.insert("cirru", Language::Cirru);
    m.insert("clw", Language::Clarion);
    m.insert("icl", Language::Clean);
    m.insert("dcl", Language::Clean);
    m.insert("click", Language::Click);
    m.insert("clj", Language::Clojure);
    m.insert("boot", Language::Clojure);
    m.insert("cl2", Language::Clojure);
    m.insert("cljc", Language::Clojure);
    m.insert("cljs", Language::Clojure);
    m.insert("cljs.hl", Language::Clojure);
    m.insert("cljscm", Language::Clojure);
    m.insert("cljx", Language::Clojure);
    m.insert("hic", Language::Clojure);
    m.insert("coffee", Language::CoffeeScript);
    m.insert("_coffee", Language::CoffeeScript);
    m.insert("cake", Language::CoffeeScript);
    m.insert("cjsx", Language::CoffeeScript);
    m.insert("cson", Language::CoffeeScript);
    m.insert("iced", Language::CoffeeScript);
    m.insert("cfm", Language::ColdFusion);
    m.insert("cfml", Language::ColdFusion);
    m.insert("cfc", Language::ColdFusionCfc);
    m.insert("lisp", Language::CommonLisp);
    m.insert("asd", Language::CommonLisp);
    m.insert("cl", Language::CommonLisp);
    m.insert("l", Language::CommonLisp);
    m.insert("lsp", Language::CommonLisp);
    m.insert("ny", Language::CommonLisp);
    m.insert("podsl", Language::CommonLisp);
    m.insert("sexp", Language::CommonLisp);
    m.insert("cp", Language::ComponentPascal);
    m.insert("cps", Language::ComponentPascal);
    m.insert("cl", Language::Cool);
    m.insert("coq", Language::Coq);
    m.insert("v", Language::Coq);
    m.insert("cr", Language::Crystal);
    m.insert("feature", Language::Cucumber);
    m.insert("cu", Language::Cuda);
    m.insert("cuh", Language::Cuda);
    m.insert("cy", Language::Cycript);
    m.insert("pyx", Language::Cython);
    m.insert("pxd", Language::Cython);
    m.insert("pxi", Language::Cython);
    m.insert("d", Language::D);
    m.insert("di", Language::D);
    m.insert("com", Language::DigitalCommandLanguage);
    m.insert("dm", Language::Dm);
    m.insert("d", Language::DTrace);
    m.insert("dart", Language::Dart);
    m.insert("djs", Language::Dogescript);
    m.insert("dylan", Language::Dylan);
    m.insert("dyl", Language::Dylan);
    m.insert("intr", Language::Dylan);
    m.insert("lid", Language::Dylan);
    m.insert("E", Language::E);
    m.insert("ecl", Language::Ecl);
    m.insert("eclxml", Language::Ecl);
    m.insert("ecl", Language::Eclipse);
    m.insert("e", Language::Eiffel);
    m.insert("ex", Language::Elixir);
    m.insert("exs", Language::Elixir);
    m.insert("elm", Language::Elm);
    m.insert("el", Language::EmacsLisp);
    m.insert("emacs", Language::EmacsLisp);
    m.insert("emacs.desktop", Language::EmacsLisp);
    m.insert("em", Language::EmberScript);
    m.insert("emberscript", Language::EmberScript);
    m.insert("erl", Language::Erlang);
    m.insert("es", Language::Erlang);
    m.insert("escript", Language::Erlang);
    m.insert("hrl", Language::Erlang);
    m.insert("xrl", Language::Erlang);
    m.insert("yrl", Language::Erlang);
    m.insert("fs", Language::FSharp);
    m.insert("fsi", Language::FSharp);
    m.insert("fsx", Language::FSharp);
    m.insert("fx", Language::Flux);
    m.insert("flux", Language::Flux);
    m.insert("f90", Language::Fortran);
    m.insert("f", Language::Fortran);
    m.insert("f03", Language::Fortran);
    m.insert("f08", Language::Fortran);
    m.insert("f77", Language::Fortran);
    m.insert("f95", Language::Fortran);
    m.insert("for", Language::Fortran);
    m.insert("fpp", Language::Fortran);
    m.insert("factor", Language::Factor);
    m.insert("fy", Language::Fancy);
    m.insert("fancypack", Language::Fancy);
    m.insert("fan", Language::Fantom);
    m.insert("fs", Language::Filterscript);
    m.insert("fth", Language::Forth);
    m.insert("4th", Language::Forth);
    m.insert("f", Language::Forth);
    m.insert("for", Language::Forth);
    m.insert("forth", Language::Forth);
    m.insert("fr", Language::Forth);
    m.insert("frt", Language::Forth);
    m.insert("fs", Language::Forth);
    m.insert("ftl", Language::FreeMarker);
    m.insert("fr", Language::Frege);
    m.insert("gms", Language::Gams);
    m.insert("g", Language::Gap);
    m.insert("gap", Language::Gap);
    m.insert("gd", Language::Gap);
    m.insert("gi", Language::Gap);
    m.insert("tst", Language::Gap);
    m.insert("s", Language::Gas);
    m.insert("ms", Language::Gas);
    m.insert("gd", Language::GdScript);
    m.insert("glsl", Language::Glsl);
    m.insert("fp", Language::Glsl);
    m.insert("frag", Language::Glsl);
    m.insert("frg", Language::Glsl);
    m.insert("fs", Language::Glsl);
    m.insert("fsh", Language::Glsl);
    m.insert("fshader", Language::Glsl);
    m.insert("geo", Language::Glsl);
    m.insert("geom", Language::Glsl);
    m.insert("glslv", Language::Glsl);
    m.insert("gshader", Language::Glsl);
    m.insert("shader", Language::Glsl);
    m.insert("vert", Language::Glsl);
    m.insert("vrx", Language::Glsl);
    m.insert("vsh", Language::Glsl);
    m.insert("vshader", Language::Glsl);
    m.insert("gml", Language::GameMakerLanguage);
    m.insert("kid", Language::Genshi);
    m.insert("ebuild", Language::GentooEbuild);
    m.insert("eclass", Language::GentooEclass);
    m.insert("glf", Language::Glyph);
    m.insert("gp", Language::Gnuplot);
    m.insert("gnu", Language::Gnuplot);
    m.insert("gnuplot", Language::Gnuplot);
    m.insert("plot", Language::Gnuplot);
    m.insert("plt", Language::Gnuplot);
    m.insert("go", Language::Go);
    m.insert("golo", Language::Golo);
    m.insert("gs", Language::Gosu);
    m.insert("gst", Language::Gosu);
    m.insert("gsx", Language::Gosu);
    m.insert("vark", Language::Gosu);
    m.insert("grace", Language::Grace);
    m.insert("gf", Language::GrammaticalFramework);
    m.insert("groovy", Language::Groovy);
    m.insert("grt", Language::Groovy);
    m.insert("gtpl", Language::Groovy);
    m.insert("gvy", Language::Groovy);
    m.insert("gsp", Language::GroovyServerPages);
    m.insert("hcl", Language::Hcl);
    m.insert("tf", Language::Hcl);
    m.insert("hlsl", Language::Hlsl);
    m.insert("fx", Language::Hlsl);
    m.insert("fxh", Language::Hlsl);
    m.insert("hlsli", Language::Hlsl);
    m.insert("hh", Language::Hack);
    m.insert("php", Language::Hack);
    m.insert("hb", Language::Harbour);
    m.insert("hs", Language::Haskell);
    m.insert("hsc", Language::Haskell);
    m.insert("hx", Language::Haxe);
    m.insert("hxsl", Language::Haxe);
    m.insert("hy", Language::Hy);
    m.insert("bf", Language::HyPhy);
    m.insert("pro", Language::Idl);
    m.insert("dlm", Language::Idl);
    m.insert("ipf", Language::IgorPro);
    m.insert("idr", Language::Idris);
    m.insert("lidr", Language::Idris);
    m.insert("ni", Language::Inform7);
    m.insert("i7x", Language::Inform7);
    m.insert("iss", Language::InnoSetup);
    m.insert("io", Language::Io);
    m.insert("ik", Language::Ioke);
    m.insert("thy", Language::Isabelle);
    m.insert("ijs", Language::J);
    m.insert("flex", Language::JFlex);
    m.insert("jflex", Language::JFlex);
    m.insert("jq", Language::Jsoniq);
    m.insert("jsx", Language::Jsx);
    m.insert("j", Language::Jasmin);
    m.insert("java", Language::Java);
    m.insert("jsp", Language::JavaServerPages);
    m.insert("js", Language::JavaScript);
    m.insert("_js", Language::JavaScript);
    m.insert("bones", Language::JavaScript);
    m.insert("es", Language::JavaScript);
    m.insert("es6", Language::JavaScript);
    m.insert("frag", Language::JavaScript);
    m.insert("gs", Language::JavaScript);
    m.insert("jake", Language::JavaScript);
    m.insert("jsb", Language::JavaScript);
    m.insert("jscad", Language::JavaScript);
    m.insert("jsfl", Language::JavaScript);
    m.insert("jsm", Language::JavaScript);
    m.insert("jss", Language::JavaScript);
    m.insert("njs", Language::JavaScript);
    m.insert("pac", Language::JavaScript);
    m.insert("sjs", Language::JavaScript);
    m.insert("ssjs", Language::JavaScript);
    m.insert("sublime-build", Language::JavaScript);
    m.insert("sublime-commands", Language::JavaScript);
    m.insert("sublime-completions", Language::JavaScript);
    m.insert("sublime-keymap", Language::JavaScript);
    m.insert("sublime-macro", Language::JavaScript);
    m.insert("sublime-menu", Language::JavaScript);
    m.insert("sublime-mousemap", Language::JavaScript);
    m.insert("sublime-project", Language::JavaScript);
    m.insert("sublime-settings", Language::JavaScript);
    m.insert("sublime-theme", Language::JavaScript);
    m.insert("sublime-workspace", Language::JavaScript);
    m.insert("sublime_metrics", Language::JavaScript);
    m.insert("sublime_session", Language::JavaScript);
    m.insert("xsjs", Language::JavaScript);
    m.insert("xsjslib", Language::JavaScript);
    m.insert("jl", Language::Julia);
    m.insert("krl", Language::Krl);
    m.insert("sch", Language::KiCad);
    m.insert("brd", Language::KiCad);
    m.insert("kicad_pcb", Language::KiCad);
    m.insert("kt", Language::Kotlin);
    m.insert("ktm", Language::Kotlin);
    m.insert("kts", Language::Kotlin);
    m.insert("lfe", Language::Lfe);
    m.insert("ll", Language::Llvm);
    m.insert("lol", Language::Lolcode);
    m.insert("lsl", Language::Lsl);
    m.insert("lslp", Language::Lsl);
    m.insert("lvproj", Language::LabView);
    m.insert("lasso", Language::Lasso);
    m.insert("las", Language::Lasso);
    m.insert("lasso8", Language::Lasso);
    m.insert("lasso9", Language::Lasso);
    m.insert("ldml", Language::Lasso);
    m.insert("lean", Language::Lean);
    m.insert("hlean", Language::Lean);
    m.insert("l", Language::Lex);
    m.insert("lex", Language::Lex);
    m.insert("ly", Language::LilyPond);
    m.insert("ily", Language::LilyPond);
    m.insert("b", Language::Limbo);
    m.insert("m", Language::Limbo);
    m.insert("lagda", Language::LiterateAgda);
    m.insert("litcoffee", Language::LiterateCoffeeScript);
    m.insert("lhs", Language::LiterateHaskell);
    m.insert("ls", Language::LiveScript);
    m.insert("_ls", Language::LiveScript);
    m.insert("xm", Language::Logos);
    m.insert("x", Language::Logos);
    m.insert("xi", Language::Logos);
    m.insert("lgt", Language::Logtalk);
    m.insert("logtalk", Language::Logtalk);
    m.insert("lookml", Language::LookMl);
    m.insert("ls", Language::LoomScript);
    m.insert("lua", Language::Lua);
    m.insert("fcgi", Language::Lua);
    m.insert("nse", Language::Lua);
    m.insert("pd_lua", Language::Lua);
    m.insert("rbxs", Language::Lua);
    m.insert("wlua", Language::Lua);
    m.insert("mumps", Language::M);
    m.insert("m", Language::M);
    m.insert("m4", Language::M4);
    m.insert("m4", Language::M4Sugar);
    m.insert("ms", Language::MaxScript);
    m.insert("mcr", Language::MaxScript);
    m.insert("muf", Language::Muf);
    m.insert("m", Language::Muf);
    m.insert("mak", Language::Makefile);
    m.insert("d", Language::Makefile);
    m.insert("mk", Language::Makefile);
    m.insert("mkfile", Language::Makefile);
    m.insert("mako", Language::Mako);
    m.insert("mao", Language::Mako);
    m.insert("mathematica", Language::Mathematica);
    m.insert("cdf", Language::Mathematica);
    m.insert("m", Language::Mathematica);
    m.insert("ma", Language::Mathematica);
    m.insert("mt", Language::Mathematica);
    m.insert("nb", Language::Mathematica);
    m.insert("nbp", Language::Mathematica);
    m.insert("wl", Language::Mathematica);
    m.insert("wlt", Language::Mathematica);
    m.insert("matlab", Language::Matlab);
    m.insert("m", Language::Matlab);
    m.insert("maxpat", Language::Max);
    m.insert("maxhelp", Language::Max);
    m.insert("maxproj", Language::Max);
    m.insert("mxt", Language::Max);
    m.insert("pat", Language::Max);
    m.insert("m", Language::Mercury);
    m.insert("moo", Language::Mercury);
    m.insert("metal", Language::Metal);
    m.insert("minid", Language::MiniD);
    m.insert("druby", Language::Mirah);
    m.insert("duby", Language::Mirah);
    m.insert("mir", Language::Mirah);
    m.insert("mirah", Language::Mirah);
    m.insert("mo", Language::Modelica);
    m.insert("mod", Language::Modula2);
    m.insert("mms", Language::ModuleManagementSystem);
    m.insert("mmk", Language::ModuleManagementSystem);
    m.insert("monkey", Language::Monkey);
    m.insert("moo", Language::Moocode);
    m.insert("moon", Language::MoonScript);
    m.insert("myt", Language::Myghty);
    m.insert("ncl", Language::Ncl);
    m.insert("nsi", Language::Nsis);
    m.insert("nsh", Language::Nsis);
    m.insert("n", Language::Nemerle);
    m.insert("axs", Language::NetLinx);
    m.insert("axi", Language::NetLinx);
    m.insert("axs.erb", Language::NetLinxErb);
    m.insert("axi.erb", Language::NetLinxErb);
    m.insert("nlogo", Language::NetLogo);
    m.insert("nl", Language::NewLisp);
    m.insert("lisp", Language::NewLisp);
    m.insert("lsp", Language::NewLisp);
    m.insert("nim", Language::Nimrod);
    m.insert("nimrod", Language::Nimrod);
    m.insert("nit", Language::Nit);
    m.insert("nix", Language::Nix);
    m.insert("nu", Language::Nu);
    m.insert("numpy", Language::NumPy);
    m.insert("numpyw", Language::NumPy);
    m.insert("numsc", Language::NumPy);
    m.insert("ml", Language::OCaml);
    m.insert("eliom", Language::OCaml);
    m.insert("eliomi", Language::OCaml);
    m.insert("ml4", Language::OCaml);
    m.insert("mli", Language::OCaml);
    m.insert("mll", Language::OCaml);
    m.insert("mly", Language::OCaml);
    m.insert("m", Language::ObjectiveC);
    m.insert("h", Language::ObjectiveC);
    m.insert("mm", Language::ObjectiveCpp);
    m.insert("j", Language::ObjectiveJ);
    m.insert("sj", Language::ObjectiveJ);
    m.insert("omgrofl", Language::Omgrofl);
    m.insert("opa", Language::Opa);
    m.insert("opal", Language::Opal);
    m.insert("cl", Language::OpenCl);
    m.insert("opencl", Language::OpenCl);
    m.insert("p", Language::OpenEdgeAbl);
    m.insert("cls", Language::OpenEdgeAbl);
    m.insert("scad", Language::OpenScad);
    m.insert("ox", Language::Ox);
    m.insert("oxh", Language::Ox);
    m.insert("oxo", Language::Ox);
    m.insert("oxygene", Language::Oxygene);
    m.insert("oz", Language::Oz);
    m.insert("pwn", Language::Pawn);
    m.insert("inc", Language::Pawn);
    m.insert("php", Language::Php);
    m.insert("aw", Language::Php);
    m.insert("ctp", Language::Php);
    m.insert("fcgi", Language::Php);
    m.insert("inc", Language::Php);
    m.insert("php3", Language::Php);
    m.insert("php4", Language::Php);
    m.insert("php5", Language::Php);
    m.insert("phps", Language::Php);
    m.insert("phpt", Language::Php);
    m.insert("pls", Language::Plsql);
    m.insert("pck", Language::Plsql);
    m.insert("pkb", Language::Plsql);
    m.insert("pks", Language::Plsql);
    m.insert("plb", Language::Plsql);
    m.insert("plsql", Language::Plsql);
    m.insert("sql", Language::Plsql);
    m.insert("sql", Language::PlPgSql);
    m.insert("pov", Language::PovRaySdl);
    m.insert("inc", Language::PovRaySdl);
    m.insert("pan", Language::Pan);
    m.insert("psc", Language::Papyrus);
    m.insert("parrot", Language::Parrot);
    m.insert("pasm", Language::ParrotAssembly);
    m.insert("pir", Language::ParrotInternalRepresentation);
    m.insert("pas", Language::Pascal);
    m.insert("dfm", Language::Pascal);
    m.insert("dpr", Language::Pascal);
    m.insert("inc", Language::Pascal);
    m.insert("lpr", Language::Pascal);
    m.insert("pp", Language::Pascal);
    m.insert("pl", Language::Perl);
    m.insert("al", Language::Perl);
    m.insert("cgi", Language::Perl);
    m.insert("fcgi", Language::Perl);
    m.insert("perl", Language::Perl);
    m.insert("ph", Language::Perl);
    m.insert("plx", Language::Perl);
    m.insert("pm", Language::Perl);
    m.insert("pod", Language::Perl);
    m.insert("psgi", Language::Perl);
    m.insert("t", Language::Perl);
    m.insert("6pl", Language::Perl6);
    m.insert("6pm", Language::Perl6);
    m.insert("nqp", Language::Perl6);
    m.insert("p6", Language::Perl6);
    m.insert("p6l", Language::Perl6);
    m.insert("p6m", Language::Perl6);
    m.insert("pl", Language::Perl6);
    m.insert("pl6", Language::Perl6);
    m.insert("pm", Language::Perl6);
    m.insert("pm6", Language::Perl6);
    m.insert("t", Language::Perl6);
    m.insert("l", Language::PicoLisp);
    m.insert("pig", Language::PigLatin);
    m.insert("pike", Language::Pike);
    m.insert("pmod", Language::Pike);
    m.insert("pogo", Language::PogoScript);
    m.insert("pony", Language::Pony);
    m.insert("ps1", Language::PowerShell);
    m.insert("psd1", Language::PowerShell);
    m.insert("psm1", Language::PowerShell);
    m.insert("pde", Language::Processing);
    m.insert("pl", Language::Prolog);
    m.insert("pro", Language::Prolog);
    m.insert("prolog", Language::Prolog);
    m.insert("yap", Language::Prolog);
    m.insert("spin", Language::PropellerSpin);
    m.insert("pp", Language::Puppet);
    m.insert("pd", Language::PureData);
    m.insert("pb", Language::PureBasic);
    m.insert("pbi", Language::PureBasic);
    m.insert("purs", Language::PureScript);
    m.insert("py", Language::Python);
    m.insert("bzl", Language::Python);
    m.insert("cgi", Language::Python);
    m.insert("fcgi", Language::Python);
    m.insert("gyp", Language::Python);
    m.insert("lmi", Language::Python);
    m.insert("pyde", Language::Python);
    m.insert("pyp", Language::Python);
    m.insert("pyt", Language::Python);
    m.insert("pyw", Language::Python);
    m.insert("rpy", Language::Python);
    m.insert("tac", Language::Python);
    m.insert("wsgi", Language::Python);
    m.insert("xpy", Language::Python);
    m.insert("qml", Language::Qml);
    m.insert("qbs", Language::Qml);
    m.insert("pro", Language::QMake);
    m.insert("pri", Language::QMake);
    m.insert("r", Language::R);
    m.insert("rd", Language::R);
    m.insert("rsx", Language::R);
    m.insert("rbbas", Language::RealBasic);
    m.insert("rbfrm", Language::RealBasic);
    m.insert("rbmnu", Language::RealBasic);
    m.insert("rbres", Language::RealBasic);
    m.insert("rbtbar", Language::RealBasic);
    m.insert("rbuistate", Language::RealBasic);
    m.insert("rkt", Language::Racket);
    m.insert("rktd", Language::Racket);
    m.insert("rktl", Language::Racket);
    m.insert("scrbl", Language::Racket);
    m.insert("rl", Language::RagelInRubyHost);
    m.insert("reb", Language::Rebol);
    m.insert("r", Language::Rebol);
    m.insert("r2", Language::Rebol);
    m.insert("r3", Language::Rebol);
    m.insert("rebol", Language::Rebol);
    m.insert("red", Language::Red);
    m.insert("reds", Language::Red);
    m.insert("cw", Language::Redcode);
    m.insert("rpy", Language::RenPy);
    m.insert("rs", Language::RenderScript);
    m.insert("rsh", Language::RenderScript);
    m.insert("robot", Language::RobotFramework);
    m.insert("rg", Language::Rouge);
    m.insert("rb", Language::Ruby);
    m.insert("builder", Language::Ruby);
    m.insert("fcgi", Language::Ruby);
    m.insert("gemspec", Language::Ruby);
    m.insert("god", Language::Ruby);
    m.insert("irbrc", Language::Ruby);
    m.insert("jbuilder", Language::Ruby);
    m.insert("mspec", Language::Ruby);
    m.insert("pluginspec", Language::Ruby);
    m.insert("podspec", Language::Ruby);
    m.insert("rabl", Language::Ruby);
    m.insert("rake", Language::Ruby);
    m.insert("rbuild", Language::Ruby);
    m.insert("rbw", Language::Ruby);
    m.insert("rbx", Language::Ruby);
    m.insert("ru", Language::Ruby);
    m.insert("ruby", Language::Ruby);
    m.insert("thor", Language::Ruby);
    m.insert("watchr", Language::Ruby);
    m.insert("rs", Language::Rust);
    m.insert("rs.in", Language::Rust);
    m.insert("sas", Language::Sas);
    m.insert("smt2", Language::Smt);
    m.insert("smt", Language::Smt);
    m.insert("sqf", Language::Sqf);
    m.insert("hqf", Language::Sqf);
    m.insert("sql", Language::Sqlpl);
    m.insert("db2", Language::Sqlpl);
    m.insert("sage", Language::Sage);
    m.insert("sagews", Language::Sage);
    m.insert("sls", Language::SaltStack);
    m.insert("scala", Language::Scala);
    m.insert("sbt", Language::Scala);
    m.insert("sc", Language::Scala);
    m.insert("scm", Language::Scheme);
    m.insert("sld", Language::Scheme);
    m.insert("sls", Language::Scheme);
    m.insert("sps", Language::Scheme);
    m.insert("ss", Language::Scheme);
    m.insert("sci", Language::Scilab);
    m.insert("sce", Language::Scilab);
    m.insert("tst", Language::Scilab);
    m.insert("self", Language::Self_);
    m.insert("sh", Language::Shell);
    m.insert("bash", Language::Shell);
    m.insert("bats", Language::Shell);
    m.insert("cgi", Language::Shell);
    m.insert("command", Language::Shell);
    m.insert("fcgi", Language::Shell);
    m.insert("ksh", Language::Shell);
    m.insert("sh.in", Language::Shell);
    m.insert("tmux", Language::Shell);
    m.insert("tool", Language::Shell);
    m.insert("zsh", Language::Shell);
    m.insert("sh-session", Language::ShellSession);
    m.insert("shen", Language::Shen);
    m.insert("sl", Language::Slash);
    m.insert("smali", Language::Smali);
    m.insert("st", Language::Smalltalk);
    m.insert("cs", Language::Smalltalk);
    m.insert("tpl", Language::Smarty);
    m.insert("sp", Language::SourcePawn);
    m.insert("inc", Language::SourcePawn);
    m.insert("sma", Language::SourcePawn);
    m.insert("nut", Language::Squirrel);
    m.insert("stan", Language::Stan);
    m.insert("ML", Language::StandardMl);
    m.insert("fun", Language::StandardMl);
    m.insert("sig", Language::StandardMl);
    m.insert("sml", Language::StandardMl);
    m.insert("do", Language::Stata);
    m.insert("ado", Language::Stata);
    m.insert("doh", Language::Stata);
    m.insert("ihlp", Language::Stata);
    m.insert("mata", Language::Stata);
    m.insert("matah", Language::Stata);
    m.insert("sthlp", Language::Stata);
    m.insert("sc", Language::SuperCollider);
    m.insert("scd", Language::SuperCollider);
    m.insert("swift", Language::Swift);
    m.insert("sv", Language::SystemVerilog);
    m.insert("svh", Language::SystemVerilog);
    m.insert("vh", Language::SystemVerilog);
    m.insert("txl", Language::Txl);
    m.insert("tcl", Language::Tcl);
    m.insert("adp", Language::Tcl);
    m.insert("tm", Language::Tcl);
    m.insert("tcsh", Language::Tcsh);
    m.insert("csh", Language::Tcsh);
    m.insert("t", Language::Terra);
    m.insert("thrift", Language::Thrift);
    m.insert("t", Language::Turing);
    m.insert("tu", Language::Turing);
    m.insert("ts", Language::TypeScript);
    m.insert("tsx", Language::TypeScript);
    m.insert("upc", Language::UnifiedParallelC);
    m.insert("uno", Language::Uno);
    m.insert("uc", Language::UnrealScript);
    m.insert("ur", Language::UrWeb);
    m.insert("urs", Language::UrWeb);
    m.insert("vcl", Language::Vcl);
    m.insert("vhdl", Language::Vhdl);
    m.insert("vhd", Language::Vhdl);
    m.insert("vhf", Language::Vhdl);
    m.insert("vhi", Language::Vhdl);
    m.insert("vho", Language::Vhdl);
    m.insert("vhs", Language::Vhdl);
    m.insert("vht", Language::Vhdl);
    m.insert("vhw", Language::Vhdl);
    m.insert("vala", Language::Vala);
    m.insert("vapi", Language::Vala);
    m.insert("v", Language::Verilog);
    m.insert("veo", Language::Verilog);
    m.insert("vim", Language::VimL);
    m.insert("vb", Language::VisualBasic);
    m.insert("bas", Language::VisualBasic);
    m.insert("cls", Language::VisualBasic);
    m.insert("frm", Language::VisualBasic);
    m.insert("frx", Language::VisualBasic);
    m.insert("vba", Language::VisualBasic);
    m.insert("vbhtml", Language::VisualBasic);
    m.insert("vbs", Language::VisualBasic);
    m.insert("volt", Language::Volt);
    m.insert("webidl", Language::WebIdl);
    m.insert("x10", Language::X10);
    m.insert("xc", Language::Xc);
    m.insert("xsp-config", Language::XPages);
    m.insert("xsp.metadata", Language::XPages);
    m.insert("xpl", Language::XProc);
    m.insert("xproc", Language::XProc);
    m.insert("xquery", Language::XQuery);
    m.insert("xq", Language::XQuery);
    m.insert("xql", Language::XQuery);
    m.insert("xqm", Language::XQuery);
    m.insert("xqy", Language::XQuery);
    m.insert("xs", Language::Xs);
    m.insert("xslt", Language::Xslt);
    m.insert("xsl", Language::Xslt);
    m.insert("xojo_code", Language::Xojo);
    m.insert("xojo_menu", Language::Xojo);
    m.insert("xojo_report", Language::Xojo);
    m.insert("xojo_script", Language::Xojo);
    m.insert("xojo_toolbar", Language::Xojo);
    m.insert("xojo_window", Language::Xojo);
    m.insert("xtend", Language::Xtend);
    m.insert("y", Language::Yacc);
    m.insert("yacc", Language::Yacc);
    m.insert("yy", Language::Yacc);
    m.insert("zep", Language::Zephir);
    m.insert("zig", Language::Zig);
    m.insert("zimpl", Language::Zimpl);
    m.insert("zmpl", Language::Zimpl);
    m.insert("zpl", Language::Zimpl);
    m.insert("ec", Language::C);
    m.insert("eh", Language::C);
    m.insert("fish", Language::Fish);
    m.insert("mu", Language::Mupad);
    m.insert("nc", Language::NesC);
    m.insert("ooc", Language::Ooc);
    m.insert("wisp", Language::Wisp);
    m.insert("prg", Language::XBase);
    m.insert("ch", Language::XBase);
    m.insert("prw", Language::XBase);

    m
});

impl Language {
    pub fn from_extension(ext: &str) -> Option<Language> {
        let ext = ext.strip_prefix('.').unwrap_or(ext);
        EXTENSION_MAP.get(ext).copied()
    }
}
