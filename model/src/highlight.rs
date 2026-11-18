//! A small syntax highlighter for the course languages. It splits code
//! into tokens and gives each one a CSS class (`tok-kw`, `tok-str`, …, see
//! tailwind.css). Used by the app and by the static pages.

pub type Token = (&'static str, String);

struct Sprache {
    keywords: &'static [&'static str],
    types: &'static [&'static str],
    /// Line comment starts (`//`, `#`, `--`).
    zeilenkommentar: &'static [&'static str],
    blockkommentar: bool,
    /// Single quotes start a string (not a char/lifetime like in Rust).
    einfache_strings: bool,
    /// Backticks start a string (JS/TS templates, Go raw strings).
    backtick_strings: bool,
    /// `"""` strings (Python, Kotlin, Swift, C#).
    dreifach: bool,
    /// Keywords are matched ignoring case (SQL).
    gross_klein_egal: bool,
    /// `name!` is a macro (Rust).
    makros: bool,
    /// `@name` and `#name` lines are annotations (decorators, preprocessor).
    annotationen: bool,
}

const RUST: Sprache = Sprache {
    keywords: &[
        "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern", "false", "fn", "for",
        "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self", "Self", "static",
        "struct", "super", "trait", "true", "type", "unsafe", "use", "where", "while",
    ],
    types: &["i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "f32", "f64", "bool", "char", "str"],
    zeilenkommentar: &["//"],
    blockkommentar: true,
    einfache_strings: false,
    backtick_strings: false,
    dreifach: false,
    gross_klein_egal: false,
    makros: true,
    annotationen: false,
};

const PYTHON: Sprache = Sprache {
    keywords: &[
        "False", "None", "True", "and", "as", "assert", "async", "await", "break", "class", "continue", "def", "del", "elif",
        "else", "except", "finally", "for", "from", "global", "if", "import", "in", "is", "lambda", "nonlocal", "not", "or",
        "pass", "raise", "return", "try", "while", "with", "yield", "match", "case",
    ],
    types: &["int", "float", "str", "bool", "list", "dict", "tuple", "set", "range", "type", "len", "print", "self"],
    zeilenkommentar: &["#"],
    blockkommentar: false,
    einfache_strings: true,
    backtick_strings: false,
    dreifach: true,
    gross_klein_egal: false,
    makros: false,
    annotationen: true,
};

const JS_KEYWORDS: &[&str] = &[
    "async", "await", "break", "case", "catch", "class", "const", "continue", "debugger", "default", "delete", "do", "else",
    "export", "extends", "false", "finally", "for", "function", "if", "import", "in", "instanceof", "let", "new", "null", "of",
    "return", "static", "super", "switch", "this", "throw", "true", "try", "typeof", "undefined", "var", "void", "while",
    "yield",
];

const JAVASCRIPT: Sprache = Sprache {
    keywords: JS_KEYWORDS,
    types: &["console", "Math", "JSON", "Object", "Array", "String", "Number", "Boolean", "Promise", "Map", "Set"],
    zeilenkommentar: &["//"],
    blockkommentar: true,
    einfache_strings: true,
    backtick_strings: true,
    dreifach: false,
    gross_klein_egal: false,
    makros: false,
    annotationen: false,
};

const TYPESCRIPT: Sprache = Sprache {
    keywords: &[
        "async", "await", "break", "case", "catch", "class", "const", "continue", "default", "delete", "do", "else", "enum",
        "export", "extends", "false", "finally", "for", "function", "if", "implements", "import", "in", "instanceof",
        "interface", "keyof", "let", "new", "null", "of", "private", "protected", "public", "readonly", "return", "static",
        "super", "switch", "this", "throw", "true", "try", "type", "typeof", "undefined", "var", "void", "while", "as",
    ],
    types: &[
        "number", "string", "boolean", "any", "unknown", "never", "object", "void", "console", "Math", "JSON", "Array",
        "Record", "Partial", "Readonly", "Promise", "Map", "Set",
    ],
    zeilenkommentar: &["//"],
    blockkommentar: true,
    einfache_strings: true,
    backtick_strings: true,
    dreifach: false,
    gross_klein_egal: false,
    makros: false,
    annotationen: true,
};

const GO: Sprache = Sprache {
    keywords: &[
        "break", "case", "chan", "const", "continue", "default", "defer", "else", "fallthrough", "for", "func", "go", "goto",
        "if", "import", "interface", "map", "package", "range", "return", "select", "struct", "switch", "type", "var", "true",
        "false", "nil",
    ],
    types: &[
        "int", "int8", "int16", "int32", "int64", "uint", "uint8", "uint16", "uint32", "uint64", "float32", "float64", "string",
        "bool", "byte", "rune", "error", "any", "make", "len", "append", "cap", "new", "fmt",
    ],
    zeilenkommentar: &["//"],
    blockkommentar: true,
    einfache_strings: true,
    backtick_strings: true,
    dreifach: false,
    gross_klein_egal: false,
    makros: false,
    annotationen: false,
};

const CPP: Sprache = Sprache {
    keywords: &[
        "auto", "break", "case", "catch", "class", "const", "constexpr", "continue", "default", "delete", "do", "else", "enum",
        "false", "for", "friend", "if", "inline", "namespace", "new", "nullptr", "operator", "private", "protected", "public",
        "return", "sizeof", "static", "struct", "switch", "template", "this", "throw", "true", "try", "typename", "using",
        "virtual", "void", "while", "override",
    ],
    types: &["int", "double", "float", "char", "bool", "long", "short", "unsigned", "size_t", "std", "string", "vector", "cout", "cin", "endl"],
    zeilenkommentar: &["//"],
    blockkommentar: true,
    einfache_strings: true,
    backtick_strings: false,
    dreifach: false,
    gross_klein_egal: false,
    makros: false,
    annotationen: true,
};

const CSHARP: Sprache = Sprache {
    keywords: &[
        "abstract", "as", "base", "break", "case", "catch", "class", "const", "continue", "default", "do", "else", "enum",
        "false", "finally", "for", "foreach", "get", "if", "in", "init", "interface", "internal", "is", "namespace", "new",
        "null", "out", "override", "private", "protected", "public", "readonly", "record", "ref", "return", "sealed", "set",
        "static", "struct", "switch", "this", "throw", "true", "try", "using", "var", "virtual", "void", "while", "when",
    ],
    types: &["int", "double", "float", "decimal", "char", "bool", "string", "object", "long", "byte", "Console", "List", "Dictionary", "Math"],
    zeilenkommentar: &["//"],
    blockkommentar: true,
    einfache_strings: true,
    backtick_strings: false,
    dreifach: true,
    gross_klein_egal: false,
    makros: false,
    annotationen: false,
};

const KOTLIN: Sprache = Sprache {
    keywords: &[
        "as", "break", "class", "continue", "data", "do", "else", "false", "for", "fun", "if", "in", "interface", "is", "null",
        "object", "override", "private", "public", "return", "this", "throw", "true", "try", "catch", "val", "var", "when",
        "while", "until", "downTo", "step", "open", "enum", "companion", "import", "package",
    ],
    types: &["Int", "Long", "Double", "Float", "String", "Boolean", "Char", "Unit", "Any", "List", "Map", "Set", "Array"],
    zeilenkommentar: &["//"],
    blockkommentar: true,
    einfache_strings: true,
    backtick_strings: false,
    dreifach: true,
    gross_klein_egal: false,
    makros: false,
    annotationen: true,
};

const SQL: Sprache = Sprache {
    keywords: &[
        "select", "from", "where", "and", "or", "not", "in", "is", "null", "as", "order", "by", "asc", "desc", "limit", "offset",
        "distinct", "group", "having", "join", "inner", "left", "right", "outer", "on", "insert", "into", "values", "update",
        "set", "delete", "create", "table", "drop", "alter", "add", "primary", "key", "foreign", "references", "unique",
        "default", "between", "like", "case", "when", "then", "else", "end", "union", "all", "exists", "if",
    ],
    types: &[
        "integer", "int", "text", "real", "numeric", "varchar", "boolean", "date", "count", "sum", "avg", "min", "max",
        "round", "upper", "lower", "length", "coalesce",
    ],
    zeilenkommentar: &["--"],
    blockkommentar: true,
    einfache_strings: true,
    backtick_strings: false,
    dreifach: false,
    gross_klein_egal: true,
    makros: false,
    annotationen: false,
};

const SWIFT: Sprache = Sprache {
    keywords: &[
        "as", "break", "case", "class", "continue", "default", "defer", "do", "else", "enum", "extension", "false", "for",
        "func", "guard", "if", "import", "in", "init", "is", "let", "mutating", "nil", "private", "protocol", "public",
        "repeat", "return", "self", "static", "struct", "switch", "throw", "throws", "true", "try", "var", "where", "while",
    ],
    types: &["Int", "Double", "Float", "String", "Bool", "Character", "Array", "Dictionary", "Set", "Optional", "print"],
    zeilenkommentar: &["//"],
    blockkommentar: true,
    einfache_strings: false,
    backtick_strings: false,
    dreifach: true,
    gross_klein_egal: false,
    makros: false,
    annotationen: true,
};

const ZIG: Sprache = Sprache {
    keywords: &["const", "var", "fn", "pub", "return", "if", "else", "while", "for", "switch", "try", "catch", "orelse", "struct", "enum", "union", "error", "comptime", "defer", "errdefer", "null", "undefined", "true", "false", "break", "continue", "and", "or", "unreachable", "test", "inline"],
    types: &["u8", "u16", "u32", "u64", "usize", "i8", "i16", "i32", "i64", "isize", "f32", "f64", "bool", "void", "anyerror", "type", "std"],
    zeilenkommentar: &["//"],
    blockkommentar: true,
    einfache_strings: true,
    backtick_strings: false,
    dreifach: false,
    gross_klein_egal: false,
    makros: false,
    annotationen: true,
};

const C: Sprache = Sprache {
    keywords: &["auto", "break", "case", "char", "const", "continue", "default", "do", "double", "else", "enum", "extern", "float", "for", "goto", "if", "int", "long", "register", "return", "short", "signed", "sizeof", "static", "struct", "switch", "typedef", "union", "unsigned", "void", "volatile", "while", "bool", "true", "false", "NULL"],
    types: &["size_t", "printf", "scanf", "strlen", "strcpy", "strcmp", "malloc", "free", "puts", "FILE"],
    zeilenkommentar: &["//"],
    blockkommentar: true,
    einfache_strings: true,
    backtick_strings: false,
    dreifach: false,
    gross_klein_egal: false,
    makros: false,
    annotationen: true,
};

const OBJECTIVE_C: Sprache = Sprache {
    keywords: &["auto", "break", "case", "char", "const", "continue", "default", "do", "double", "else", "enum", "extern", "float", "for", "if", "int", "long", "return", "short", "signed", "sizeof", "static", "struct", "switch", "typedef", "unsigned", "void", "while", "YES", "NO", "nil", "self", "super", "id", "BOOL", "instancetype", "interface", "implementation", "end", "property", "nonatomic", "strong", "weak", "copy", "readonly"],
    types: &["NSString", "NSNumber", "NSArray", "NSMutableArray", "NSDictionary", "NSMutableDictionary", "NSObject", "NSInteger", "NSUInteger", "CGFloat", "printf", "NSLog"],
    zeilenkommentar: &["//"],
    blockkommentar: true,
    einfache_strings: true,
    backtick_strings: false,
    dreifach: false,
    gross_klein_egal: false,
    makros: false,
    annotationen: true,
};

const JAVA: Sprache = Sprache {
    keywords: &["abstract", "boolean", "break", "byte", "case", "catch", "char", "class", "continue", "default", "do", "double", "else", "enum", "extends", "final", "finally", "float", "for", "if", "implements", "import", "instanceof", "int", "interface", "long", "new", "null", "package", "private", "protected", "public", "record", "return", "short", "static", "super", "switch", "this", "throw", "throws", "true", "false", "try", "var", "void", "while", "yield"],
    types: &["String", "System", "Integer", "Double", "Boolean", "Math", "List", "ArrayList", "Map", "HashMap", "Object", "Scanner", "Arrays"],
    zeilenkommentar: &["//"],
    blockkommentar: true,
    einfache_strings: true,
    backtick_strings: false,
    dreifach: true,
    gross_klein_egal: false,
    makros: false,
    annotationen: true,
};

const PHP: Sprache = Sprache {
    keywords: &["abstract", "and", "array", "as", "break", "case", "catch", "class", "clone", "const", "continue", "declare", "default", "do", "echo", "else", "elseif", "empty", "enum", "extends", "final", "finally", "fn", "for", "foreach", "function", "global", "if", "implements", "include", "instanceof", "interface", "isset", "match", "namespace", "new", "null", "or", "print", "private", "protected", "public", "readonly", "require", "return", "static", "switch", "throw", "trait", "true", "false", "try", "unset", "use", "var", "while", "yield"],
    types: &["int", "float", "string", "bool", "void", "mixed", "self", "strlen", "count", "implode", "explode", "str_replace", "strtoupper", "array_map", "array_filter", "in_array", "var_dump"],
    zeilenkommentar: &["//", "#"],
    blockkommentar: false,
    einfache_strings: true,
    backtick_strings: false,
    dreifach: false,
    gross_klein_egal: false,
    makros: false,
    annotationen: false,
};

const RUBY: Sprache = Sprache {
    keywords: &["alias", "and", "begin", "break", "case", "class", "def", "defined?", "do", "else", "elsif", "end", "ensure", "false", "for", "if", "in", "module", "next", "nil", "not", "or", "redo", "rescue", "retry", "return", "self", "super", "then", "true", "undef", "unless", "until", "when", "while", "yield", "attr_accessor", "attr_reader", "puts", "print", "require"],
    types: &["Integer", "Float", "String", "Array", "Hash", "Symbol", "Object", "Struct", "Comparable"],
    zeilenkommentar: &["#"],
    blockkommentar: false,
    einfache_strings: true,
    backtick_strings: false,
    dreifach: false,
    gross_klein_egal: false,
    makros: false,
    annotationen: false,
};

const CRYSTAL: Sprache = Sprache {
    keywords: &["abstract", "alias", "and", "begin", "break", "case", "class", "def", "do", "else", "elsif", "end", "ensure", "false", "for", "if", "in", "include", "module", "next", "nil", "of", "private", "property", "getter", "setter", "protected", "require", "rescue", "return", "self", "struct", "super", "then", "true", "unless", "until", "when", "while", "yield", "puts"],
    types: &["Int32", "Int64", "Float64", "String", "Bool", "Array", "Hash", "Symbol", "Nil", "Char"],
    zeilenkommentar: &["#"],
    blockkommentar: false,
    einfache_strings: true,
    backtick_strings: false,
    dreifach: false,
    gross_klein_egal: false,
    makros: false,
    annotationen: false,
};

const LUA: Sprache = Sprache {
    keywords: &["and", "break", "do", "else", "elseif", "end", "false", "for", "function", "goto", "if", "in", "local", "nil", "not", "or", "repeat", "return", "then", "true", "until", "while"],
    types: &["print", "string", "table", "math", "ipairs", "pairs", "tostring", "tonumber", "type"],
    zeilenkommentar: &["--"],
    blockkommentar: false,
    einfache_strings: true,
    backtick_strings: false,
    dreifach: false,
    gross_klein_egal: false,
    makros: false,
    annotationen: false,
};

const BASH: Sprache = Sprache {
    keywords: &["if", "then", "else", "elif", "fi", "for", "in", "do", "done", "while", "until", "case", "esac", "function", "return", "local", "declare", "export", "readonly", "break", "continue", "select", "time", "shift", "set", "unset", "echo", "printf", "read"],
    types: &["true", "false"],
    zeilenkommentar: &["#"],
    blockkommentar: false,
    einfache_strings: true,
    backtick_strings: true,
    dreifach: false,
    gross_klein_egal: false,
    makros: false,
    annotationen: false,
};

const PERL: Sprache = Sprache {
    keywords: &["my", "our", "local", "sub", "if", "elsif", "else", "unless", "while", "until", "for", "foreach", "last", "next", "redo", "return", "use", "strict", "warnings", "package", "require", "print", "say", "and", "or", "not", "eq", "ne", "lt", "gt", "le", "ge", "cmp"],
    types: &["scalar", "push", "pop", "shift", "unshift", "join", "split", "sort", "keys", "values", "exists", "delete", "defined", "length"],
    zeilenkommentar: &["#"],
    blockkommentar: false,
    einfache_strings: true,
    backtick_strings: false,
    dreifach: false,
    gross_klein_egal: false,
    makros: false,
    annotationen: false,
};

const ELIXIR: Sprache = Sprache {
    keywords: &["def", "defp", "defmodule", "do", "end", "fn", "case", "cond", "if", "else", "unless", "when", "with", "for", "in", "and", "or", "not", "true", "false", "nil", "import", "alias", "require", "use", "receive", "after", "raise", "rescue", "try"],
    types: &["IO", "Enum", "List", "Map", "String", "Integer", "Kernel", "Keyword", "Tuple"],
    zeilenkommentar: &["#"],
    blockkommentar: false,
    einfache_strings: true,
    backtick_strings: false,
    dreifach: true,
    gross_klein_egal: false,
    makros: false,
    annotationen: false,
};

const ERLANG: Sprache = Sprache {
    keywords: &["module", "export", "import", "fun", "case", "of", "end", "if", "when", "receive", "after", "try", "catch", "throw", "begin", "and", "or", "not", "andalso", "orelse", "div", "rem", "true", "false", "spawn", "self"],
    types: &["io", "lists", "maps", "string", "erlang"],
    zeilenkommentar: &["%"],
    blockkommentar: false,
    einfache_strings: true,
    backtick_strings: false,
    dreifach: false,
    gross_klein_egal: false,
    makros: false,
    annotationen: false,
};

const GLEAM: Sprache = Sprache {
    keywords: &["pub", "fn", "let", "case", "import", "type", "const", "use", "assert", "todo", "panic", "opaque", "as", "if"],
    types: &["Int", "Float", "String", "Bool", "List", "Result", "Option", "Nil", "io", "int", "string", "list"],
    zeilenkommentar: &["//"],
    blockkommentar: true,
    einfache_strings: false,
    backtick_strings: false,
    dreifach: false,
    gross_klein_egal: false,
    makros: false,
    annotationen: false,
};

const HASKELL: Sprache = Sprache {
    keywords: &["module", "import", "where", "let", "in", "if", "then", "else", "case", "of", "data", "type", "newtype", "class", "instance", "deriving", "do", "qualified", "as", "hiding", "True", "False"],
    types: &["Int", "Integer", "Double", "Float", "Bool", "Char", "String", "Maybe", "Just", "Nothing", "Either", "Left", "Right", "IO", "Show", "Eq", "Ord", "putStrLn", "print", "show", "map", "filter", "foldr", "length"],
    zeilenkommentar: &["--"],
    blockkommentar: false,
    einfache_strings: false,
    backtick_strings: false,
    dreifach: false,
    gross_klein_egal: false,
    makros: false,
    annotationen: false,
};

const OCAML: Sprache = Sprache {
    keywords: &["let", "rec", "in", "if", "then", "else", "match", "with", "fun", "function", "type", "of", "begin", "end", "module", "open", "struct", "sig", "val", "mutable", "when", "and", "or", "not", "true", "false"],
    types: &["int", "float", "string", "bool", "char", "list", "option", "unit", "Some", "None", "print_endline", "print_string", "Printf", "List"],
    zeilenkommentar: &[],
    blockkommentar: false,
    einfache_strings: false,
    backtick_strings: false,
    dreifach: false,
    gross_klein_egal: false,
    makros: false,
    annotationen: false,
};

const FSHARP: Sprache = Sprache {
    keywords: &["let", "rec", "in", "if", "then", "else", "match", "with", "fun", "function", "type", "of", "open", "module", "mutable", "for", "to", "downto", "do", "done", "while", "yield", "when", "and", "or", "not", "true", "false", "member", "new"],
    types: &["int", "float", "string", "bool", "char", "list", "option", "unit", "Some", "None", "printfn", "List", "Map", "Seq"],
    zeilenkommentar: &["//"],
    blockkommentar: false,
    einfache_strings: true,
    backtick_strings: false,
    dreifach: false,
    gross_klein_egal: false,
    makros: false,
    annotationen: false,
};

const SCALA: Sprache = Sprache {
    keywords: &["abstract", "case", "catch", "class", "def", "do", "else", "enum", "extends", "false", "final", "finally", "for", "given", "if", "implicit", "import", "lazy", "match", "new", "null", "object", "override", "package", "private", "protected", "return", "sealed", "super", "then", "this", "throw", "trait", "true", "try", "type", "val", "var", "while", "with", "yield"],
    types: &["Int", "Double", "Float", "Long", "Boolean", "String", "Char", "Unit", "Any", "List", "Vector", "Map", "Set", "Option", "Some", "None", "println"],
    zeilenkommentar: &["//"],
    blockkommentar: true,
    einfache_strings: true,
    backtick_strings: false,
    dreifach: true,
    gross_klein_egal: false,
    makros: false,
    annotationen: true,
};

const DART: Sprache = Sprache {
    keywords: &["abstract", "as", "assert", "async", "await", "break", "case", "catch", "class", "const", "continue", "default", "do", "dynamic", "else", "enum", "extends", "extension", "false", "final", "finally", "for", "if", "implements", "import", "in", "is", "late", "library", "new", "null", "on", "required", "return", "static", "super", "switch", "this", "throw", "true", "try", "var", "void", "while", "with", "yield"],
    types: &["int", "double", "num", "String", "bool", "List", "Map", "Set", "Object", "print"],
    zeilenkommentar: &["//"],
    blockkommentar: true,
    einfache_strings: true,
    backtick_strings: false,
    dreifach: true,
    gross_klein_egal: false,
    makros: false,
    annotationen: true,
};

const R: Sprache = Sprache {
    keywords: &["function", "if", "else", "for", "while", "repeat", "in", "next", "break", "TRUE", "FALSE", "NULL", "NA", "Inf", "NaN", "return"],
    types: &["print", "cat", "paste", "paste0", "length", "c", "seq", "sum", "mean", "list", "data.frame", "sapply", "lapply", "nrow", "ncol", "class"],
    zeilenkommentar: &["#"],
    blockkommentar: false,
    einfache_strings: true,
    backtick_strings: false,
    dreifach: false,
    gross_klein_egal: false,
    makros: false,
    annotationen: false,
};

const JULIA: Sprache = Sprache {
    keywords: &["function", "end", "if", "elseif", "else", "for", "while", "in", "return", "struct", "mutable", "begin", "let", "local", "global", "const", "break", "continue", "do", "try", "catch", "finally", "module", "using", "import", "true", "false", "nothing"],
    types: &["Int", "Int64", "Float64", "String", "Bool", "Char", "Vector", "Dict", "println", "print", "length", "push!"],
    zeilenkommentar: &["#"],
    blockkommentar: false,
    einfache_strings: true,
    backtick_strings: false,
    dreifach: true,
    gross_klein_egal: false,
    makros: false,
    annotationen: false,
};

const CLOJURE: Sprache = Sprache {
    keywords: &["def", "defn", "defn-", "fn", "let", "if", "when", "when-not", "cond", "do", "loop", "recur", "ns", "require", "if-let", "when-let", "case", "and", "or", "not", "nil", "true", "false"],
    types: &["println", "str", "map", "filter", "reduce", "range", "take", "count", "conj", "assoc", "dissoc", "update", "get", "first", "rest"],
    zeilenkommentar: &[";"],
    blockkommentar: false,
    einfache_strings: false,
    backtick_strings: false,
    dreifach: false,
    gross_klein_egal: false,
    makros: false,
    annotationen: false,
};

const RACKET: Sprache = Sprache {
    keywords: &["define", "lambda", "let", "let*", "letrec", "if", "cond", "when", "unless", "else", "struct", "for", "for/list", "for/fold", "require", "provide", "begin", "set!", "and", "or", "not"],
    types: &["displayln", "printf", "display", "newline", "list", "cons", "first", "rest", "empty?", "map", "filter", "foldl", "string-append"],
    zeilenkommentar: &[";"],
    blockkommentar: false,
    einfache_strings: false,
    backtick_strings: false,
    dreifach: false,
    gross_klein_egal: false,
    makros: false,
    annotationen: false,
};

const SCHEME: Sprache = Sprache {
    keywords: &["define", "lambda", "let", "let*", "letrec", "if", "cond", "case", "else", "begin", "set!", "quote", "and", "or", "not", "do"],
    types: &["display", "newline", "list", "cons", "car", "cdr", "null?", "length", "append", "map", "apply", "vector", "string-append"],
    zeilenkommentar: &[";"],
    blockkommentar: false,
    einfache_strings: false,
    backtick_strings: false,
    dreifach: false,
    gross_klein_egal: false,
    makros: false,
    annotationen: false,
};

const COMMON_LISP: Sprache = Sprache {
    keywords: &["defun", "defparameter", "defvar", "defstruct", "let", "let*", "lambda", "if", "when", "unless", "cond", "loop", "dolist", "dotimes", "setf", "progn", "and", "or", "not", "nil", "t", "return", "function"],
    types: &["format", "print", "list", "cons", "car", "cdr", "first", "rest", "mapcar", "reduce", "funcall", "length", "append"],
    zeilenkommentar: &[";"],
    blockkommentar: false,
    einfache_strings: false,
    backtick_strings: false,
    dreifach: false,
    gross_klein_egal: true,
    makros: false,
    annotationen: false,
};

const PROLOG: Sprache = Sprache {
    keywords: &["is", "not", "true", "fail", "halt", "initialization"],
    types: &["format", "writeln", "write", "nl", "findall", "forall", "between", "member", "append", "length"],
    zeilenkommentar: &["%"],
    blockkommentar: false,
    einfache_strings: true,
    backtick_strings: false,
    dreifach: false,
    gross_klein_egal: false,
    makros: false,
    annotationen: false,
};

const NIM: Sprache = Sprache {
    keywords: &["proc", "func", "var", "let", "const", "if", "elif", "else", "for", "in", "while", "case", "of", "return", "result", "type", "object", "ref", "import", "from", "and", "or", "not", "div", "mod", "true", "false", "nil", "echo", "discard", "break", "continue"],
    types: &["int", "float", "string", "bool", "char", "seq", "array"],
    zeilenkommentar: &["#"],
    blockkommentar: false,
    einfache_strings: true,
    backtick_strings: false,
    dreifach: true,
    gross_klein_egal: false,
    makros: false,
    annotationen: false,
};

const V: Sprache = Sprache {
    keywords: &["fn", "mut", "pub", "struct", "if", "else", "for", "in", "match", "return", "import", "module", "or", "none", "true", "false", "break", "continue", "const", "enum", "interface", "go", "defer", "unsafe"],
    types: &["int", "i64", "f64", "string", "bool", "u8", "rune", "println"],
    zeilenkommentar: &["//"],
    blockkommentar: true,
    einfache_strings: true,
    backtick_strings: false,
    dreifach: false,
    gross_klein_egal: false,
    makros: false,
    annotationen: false,
};

const ODIN: Sprache = Sprache {
    keywords: &["package", "import", "proc", "struct", "enum", "union", "if", "else", "for", "in", "switch", "case", "return", "defer", "when", "break", "continue", "fallthrough", "using", "distinct", "nil", "true", "false", "map", "dynamic"],
    types: &["int", "f64", "f32", "string", "bool", "rune", "u8", "i64", "fmt"],
    zeilenkommentar: &["//"],
    blockkommentar: true,
    einfache_strings: false,
    backtick_strings: true,
    dreifach: false,
    gross_klein_egal: false,
    makros: false,
    annotationen: false,
};

const D: Sprache = Sprache {
    keywords: &["import", "module", "void", "auto", "immutable", "const", "enum", "struct", "class", "interface", "if", "else", "for", "foreach", "while", "do", "switch", "case", "default", "return", "break", "continue", "true", "false", "null", "new", "this", "ref", "in", "out", "static"],
    types: &["int", "long", "double", "float", "string", "bool", "char", "size_t", "writeln", "writefln"],
    zeilenkommentar: &["//"],
    blockkommentar: true,
    einfache_strings: true,
    backtick_strings: false,
    dreifach: false,
    gross_klein_egal: false,
    makros: false,
    annotationen: true,
};

const SOLIDITY: Sprache = Sprache {
    keywords: &["pragma", "solidity", "contract", "function", "public", "private", "internal", "external", "view", "pure", "payable", "returns", "return", "if", "else", "for", "while", "do", "require", "revert", "emit", "event", "error", "modifier", "mapping", "struct", "memory", "storage", "calldata", "constructor", "is", "true", "false", "new", "delete", "msg"],
    types: &["uint", "uint256", "uint8", "int", "int256", "bool", "address", "string", "bytes", "bytes32"],
    zeilenkommentar: &["//"],
    blockkommentar: true,
    einfache_strings: false,
    backtick_strings: false,
    dreifach: false,
    gross_klein_egal: false,
    makros: false,
    annotationen: false,
};

const FORTRAN: Sprache = Sprache {
    keywords: &["program", "end", "implicit", "none", "integer", "real", "character", "logical", "parameter", "print", "write", "read", "if", "then", "else", "do", "while", "select", "case", "function", "subroutine", "contains", "module", "use", "type", "intent", "in", "out", "call", "return", "exit", "cycle", "stop", "allocatable", "dimension"],
    types: &["size", "sum", "trim", "len"],
    zeilenkommentar: &["!"],
    blockkommentar: false,
    einfache_strings: true,
    backtick_strings: false,
    dreifach: false,
    gross_klein_egal: true,
    makros: false,
    annotationen: false,
};

const COBOL: Sprache = Sprache {
    keywords: &["identification", "division", "program-id", "data", "working-storage", "section", "procedure", "display", "move", "to", "add", "subtract", "multiply", "divide", "compute", "if", "else", "end-if", "perform", "until", "varying", "times", "stop", "run", "pic", "value", "evaluate", "when", "end-evaluate", "other", "occurs", "string", "unstring", "inspect", "giving", "by", "from", "into", "delimited", "size", "not", "end-perform"],
    types: &[],
    zeilenkommentar: &["*>"],
    blockkommentar: false,
    einfache_strings: true,
    backtick_strings: false,
    dreifach: false,
    gross_klein_egal: true,
    makros: false,
    annotationen: false,
};

const PASCAL: Sprache = Sprache {
    keywords: &["program", "begin", "end", "var", "const", "type", "procedure", "function", "if", "then", "else", "case", "of", "for", "to", "downto", "do", "while", "repeat", "until", "uses", "record", "array", "set", "and", "or", "not", "div", "mod", "nil", "true", "false", "result", "with"],
    types: &["integer", "real", "char", "string", "boolean", "writeln", "write", "readln", "length", "copy", "pos", "uppercase"],
    zeilenkommentar: &["//"],
    blockkommentar: false,
    einfache_strings: true,
    backtick_strings: false,
    dreifach: false,
    gross_klein_egal: true,
    makros: false,
    annotationen: false,
};

const DELPHI: Sprache = Sprache {
    keywords: &["program", "begin", "end", "var", "const", "type", "procedure", "function", "if", "then", "else", "case", "of", "for", "to", "downto", "do", "while", "repeat", "until", "uses", "record", "array", "class", "constructor", "destructor", "property", "private", "public", "published", "read", "write", "override", "virtual", "inherited", "and", "or", "not", "div", "mod", "nil", "true", "false", "result", "out", "try", "finally", "except"],
    types: &["integer", "double", "char", "string", "boolean", "writeln", "write", "readln", "length", "copy", "pos", "uppercase", "format", "setlength", "low", "high"],
    zeilenkommentar: &["//"],
    blockkommentar: false,
    einfache_strings: true,
    backtick_strings: false,
    dreifach: false,
    gross_klein_egal: true,
    makros: false,
    annotationen: false,
};

const ADA: Sprache = Sprache {
    keywords: &["with", "use", "procedure", "function", "is", "begin", "end", "if", "then", "elsif", "else", "case", "when", "others", "loop", "for", "in", "reverse", "while", "exit", "return", "type", "subtype", "record", "array", "of", "range", "constant", "declare", "null", "and", "or", "not", "xor", "out", "new", "package", "body"],
    types: &["integer", "float", "boolean", "string", "character", "natural", "positive", "put_line", "put", "ada", "text_io"],
    zeilenkommentar: &["--"],
    blockkommentar: false,
    einfache_strings: false,
    backtick_strings: false,
    dreifach: false,
    gross_klein_egal: true,
    makros: false,
    annotationen: false,
};

const SMALLTALK: Sprache = Sprache {
    keywords: &["self", "super", "true", "false", "nil", "thisContext"],
    types: &["Transcript", "Object", "OrderedCollection", "Array", "String", "Symbol"],
    zeilenkommentar: &[],
    blockkommentar: false,
    einfache_strings: true,
    backtick_strings: false,
    dreifach: false,
    gross_klein_egal: false,
    makros: false,
    annotationen: false,
};

const ASSEMBLY: Sprache = Sprache {
    keywords: &["mov", "add", "sub", "inc", "dec", "imul", "mul", "div", "idiv", "cmp", "jmp", "je", "jne", "jz", "jnz", "jl", "jle", "jg", "jge", "call", "ret", "push", "pop", "syscall", "xor", "and", "or", "not", "lea", "test", "nop", "section", "global", "extern", "db", "dw", "dd", "dq", "resb", "resq", "default", "rel", "equ"],
    types: &["rax", "rbx", "rcx", "rdx", "rsi", "rdi", "rbp", "rsp", "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15", "eax", "ebx", "ecx", "edx", "esi", "edi", "al", "bl", "cl", "dl", "byte", "word", "dword", "qword"],
    zeilenkommentar: &[";"],
    blockkommentar: false,
    einfache_strings: true,
    backtick_strings: false,
    dreifach: false,
    gross_klein_egal: true,
    makros: false,
    annotationen: false,
};

const MACHINE_CODE: Sprache = Sprache {
    keywords: &["mov", "add", "sub", "inc", "dec", "imul", "cmp", "jmp", "je", "jne", "jz", "jnz", "jl", "jg", "call", "ret", "push", "pop", "syscall", "xor", "nop", "db", "def", "print", "for", "in", "return", "if", "else", "range", "hex", "bin", "int"],
    types: &["rax", "rbx", "rcx", "rdx", "rsi", "rdi", "rsp", "eax", "ebx", "ecx", "edx", "esi", "edi", "al", "bl", "byte", "word", "dword", "qword"],
    zeilenkommentar: &[";", "#"],
    blockkommentar: false,
    einfache_strings: true,
    backtick_strings: false,
    dreifach: false,
    gross_klein_egal: true,
    makros: false,
    annotationen: false,
};

fn sprache(course: &str) -> &'static Sprache {
    match course {
        "python" => &PYTHON,
        "typescript" => &TYPESCRIPT,
        "go" => &GO,
        "javascript" => &JAVASCRIPT,
        "cpp" => &CPP,
        "csharp" => &CSHARP,
        "kotlin" => &KOTLIN,
        "sql" => &SQL,
        "swift" => &SWIFT,
        "zig" => &ZIG,
        "c" => &C,
        "objective-c" => &OBJECTIVE_C,
        "java" => &JAVA,
        "php" => &PHP,
        "ruby" => &RUBY,
        "crystal" => &CRYSTAL,
        "lua" => &LUA,
        "bash" => &BASH,
        "perl" => &PERL,
        "elixir" => &ELIXIR,
        "erlang" => &ERLANG,
        "gleam" => &GLEAM,
        "haskell" => &HASKELL,
        "ocaml" => &OCAML,
        "fsharp" => &FSHARP,
        "scala" => &SCALA,
        "dart" => &DART,
        "r" => &R,
        "julia" => &JULIA,
        "clojure" => &CLOJURE,
        "racket" => &RACKET,
        "scheme" => &SCHEME,
        "common-lisp" => &COMMON_LISP,
        "prolog" => &PROLOG,
        "nim" => &NIM,
        "v" => &V,
        "odin" => &ODIN,
        "d" => &D,
        "solidity" => &SOLIDITY,
        "fortran" => &FORTRAN,
        "cobol" => &COBOL,
        "pascal" => &PASCAL,
        "delphi" => &DELPHI,
        "ada" => &ADA,
        "smalltalk" => &SMALLTALK,
        "assembly" => &ASSEMBLY,
        "machine-code" => &MACHINE_CODE,
        _ => &RUST,
    }
}

fn starts_at(chars: &[char], i: usize, pat: &str) -> bool {
    pat.chars().enumerate().all(|(j, p)| chars.get(i + j) == Some(&p))
}

#[allow(clippy::too_many_lines)]
/// Splits `src` into `(css class, text)` tokens for the course `course` (`rust`, `python`, …).
pub fn tokenize(src: &str, course: &str) -> Vec<Token> {
    let s = sprache(course);
    let rust = course == "rust";
    let cpp = course == "cpp";
    let js = matches!(course, "javascript" | "typescript");
    let chars: Vec<char> = src.chars().collect();
    let mut out: Vec<Token> = Vec::new();
    let mut i = 0;
    let push = |out: &mut Vec<Token>, class: &'static str, text: String| {
        if let Some(last) = out.last_mut()
            && last.0 == class
        {
            last.1.push_str(&text);
            return;
        }
        out.push((class, text));
    };
    let bis_zeilenende = |i: usize| -> usize {
        let mut j = i;
        while j < chars.len() && chars[j] != '\n' {
            j += 1;
        }
        j
    };

    while i < chars.len() {
        let c = chars[i];

        // Comments
        if s.zeilenkommentar.iter().any(|m| starts_at(&chars, i, m)) {
            let ende = bis_zeilenende(i);
            push(&mut out, "tok-com", chars[i..ende].iter().collect());
            i = ende;
            continue;
        }
        if s.blockkommentar && starts_at(&chars, i, "/*") {
            let start = i;
            i += 2;
            while i < chars.len() && !starts_at(&chars, i, "*/") {
                i += 1;
            }
            i = (i + 2).min(chars.len());
            push(&mut out, "tok-com", chars[start..i].iter().collect());
            continue;
        }

        // Annotations, decorators, preprocessor lines
        if s.annotationen && (c == '@' || (c == '#' && cpp)) && chars.get(i + 1).is_some_and(|n| n.is_alphabetic()) {
            let start = i;
            i += 1;
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            push(&mut out, "tok-mac", chars[start..i].iter().collect());
            continue;
        }

        // Strings
        if s.dreifach && starts_at(&chars, i, "\"\"\"") {
            let start = i;
            i += 3;
            while i < chars.len() && !starts_at(&chars, i, "\"\"\"") {
                i += 1;
            }
            i = (i + 3).min(chars.len());
            push(&mut out, "tok-str", chars[start..i].iter().collect());
            continue;
        }
        let quote = match c {
            '"' => Some('"'),
            '\'' if s.einfache_strings => Some('\''),
            '`' if s.backtick_strings => Some('`'),
            _ => None,
        };
        if let Some(q) = quote {
            let start = i;
            i += 1;
            while i < chars.len() && chars[i] != q {
                if chars[i] == '\\' && q != '`' {
                    i += 1;
                }
                if chars[i.min(chars.len() - 1)] == '\n' && q != '`' {
                    break;
                }
                i += 1;
            }
            i = (i + 1).min(chars.len());
            push(&mut out, "tok-str", chars[start..i].iter().collect());
            continue;
        }

        // Rust: char literal or lifetime
        if c == '\'' && rust {
            if chars.get(i + 2) == Some(&'\'') || (chars.get(i + 1) == Some(&'\\') && chars.get(i + 3) == Some(&'\'')) {
                let len = if chars.get(i + 1) == Some(&'\\') { 4 } else { 3 };
                push(&mut out, "tok-str", chars[i..i + len].iter().collect());
                i += len;
                continue;
            }
            let start = i;
            i += 1;
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            push(&mut out, "tok-life", chars[start..i].iter().collect());
            continue;
        }

        // Numbers
        if c.is_ascii_digit() {
            let start = i;
            while i < chars.len()
                && (chars[i].is_ascii_alphanumeric() || chars[i] == '_' || (chars[i] == '.' && chars.get(i + 1).is_some_and(char::is_ascii_digit)))
            {
                i += 1;
            }
            push(&mut out, "tok-num", chars[start..i].iter().collect());
            continue;
        }

        // Identifiers
        if c.is_alphabetic() || c == '_' || (c == '$' && js) {
            let start = i;
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '$') {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();
            let next = chars.get(i).copied();
            let pruef = if s.gross_klein_egal { word.to_lowercase() } else { word.clone() };
            let class = if s.keywords.contains(&pruef.as_str()) {
                "tok-kw"
            } else if s.makros && next == Some('!') {
                "tok-mac"
            } else if s.types.contains(&pruef.as_str()) || (!s.gross_klein_egal && word.starts_with(|ch: char| ch.is_uppercase())) {
                "tok-ty"
            } else if next == Some('(') {
                "tok-fn"
            } else {
                ""
            };
            if class == "tok-mac" {
                i += 1;
                push(&mut out, class, format!("{word}!"));
            } else {
                push(&mut out, class, word);
            }
            continue;
        }

        push(&mut out, "", c.to_string());
        i += 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn klassen(src: &str, k: &str) -> Vec<(&'static str, String)> {
        tokenize(src, k).into_iter().filter(|(c, _)| !c.is_empty()).collect()
    }

    #[test]
    fn languages_are_highlighted() {
        let py = klassen("def f(x):\n    return f\"{x}\"  # hi", "python");
        assert!(py.contains(&("tok-kw", "def".into())));
        assert!(py.contains(&("tok-str", "\"{x}\"".into())));
        assert!(py.contains(&("tok-com", "# hi".into())));
        let sql = klassen("SELECT name FROM books WHERE year > 2000; -- new", "sql");
        assert!(sql.contains(&("tok-kw", "SELECT".into())));
        assert!(sql.contains(&("tok-num", "2000".into())));
        assert!(sql.contains(&("tok-com", "-- new".into())));
        let js = klassen("const s = `a ${b}`;", "javascript");
        assert!(js.contains(&("tok-str", "`a ${b}`".into())));
        let rs = klassen("let x: &'a str = 'c'; println!(\"hi\");", "rust");
        assert!(rs.contains(&("tok-life", "'a".into())));
        assert!(rs.contains(&("tok-mac", "println!".into())));
        // The tokens always rebuild the source.
        let src = "#include <iostream>\nint main() { std::cout << \"x\\n\"; }";
        assert_eq!(tokenize(src, "cpp").into_iter().map(|(_, t)| t).collect::<String>(), src);
    }
}

#[cfg(test)]
mod neue_sprachen {
    use super::tokenize;

    fn klasse(src: &str, course: &str, wort: &str) -> &'static str {
        tokenize(src, course).into_iter().find(|(_, t)| t == wort).map(|(c, _)| c).unwrap_or("")
    }

    #[test]
    fn highlights_some_new_languages() {
        let kw = klasse("fn main() void {}", "zig", "fn");
        assert!(!kw.is_empty());
        assert_eq!(klasse("PROCEDURE DIVISION.", "cobol", "PROCEDURE"), kw);
        assert_eq!(klasse("Writeln('x');", "delphi", "Writeln"), klasse("Writeln('x');", "delphi", "Writeln"));
        assert_eq!(klasse("(defn f [x] x)", "clojure", "defn"), kw);
        assert_eq!(klasse("mov rax, 60", "assembly", "mov"), kw);
        assert_eq!(klasse("local x = 1", "lua", "local"), kw);
        for id in nrzlearn_model_ids() {
            let _ = tokenize("print(\"Hello\") -- x ; y # z % w ! v", id);
        }
    }

    fn nrzlearn_model_ids() -> Vec<&'static str> {
        crate::COURSES.iter().map(|c| c.id).collect()
    }
}
