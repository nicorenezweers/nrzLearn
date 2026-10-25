//! Ein kleiner, eigener Syntax-Highlighter für Rust-Code.
//! Er zerlegt Text in Tokens und ordnet jedem eine CSS-Klasse zu (siehe tailwind.css).

const KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern", "false",
    "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where", "while",
];

const PRIMITIVES: &[&str] = &[
    "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "f32", "f64",
    "bool", "char", "str",
];

pub type Token = (&'static str, String);

#[allow(clippy::too_many_lines)]
pub fn tokenize(src: &str) -> Vec<Token> {
    let chars: Vec<char> = src.chars().collect();
    let mut out: Vec<Token> = Vec::new();
    let mut i = 0;
    let push = |out: &mut Vec<Token>, class: &'static str, s: String| {
        if let Some(last) = out.last_mut()
            && last.0 == class
        {
            last.1.push_str(&s);
            return;
        }
        out.push((class, s));
    };

    while i < chars.len() {
        let c = chars[i];

        // Kommentare
        if c == '/' && chars.get(i + 1) == Some(&'/') {
            let start = i;
            while i < chars.len() && chars[i] != '\n' {
                i += 1;
            }
            push(&mut out, "tok-com", chars[start..i].iter().collect());
            continue;
        }

        // Strings
        if c == '"' {
            let start = i;
            i += 1;
            while i < chars.len() && chars[i] != '"' {
                if chars[i] == '\\' {
                    i += 1;
                }
                i += 1;
            }
            i = (i + 1).min(chars.len());
            push(&mut out, "tok-str", chars[start..i].iter().collect());
            continue;
        }

        // Char-Literal oder Lifetime
        if c == '\'' {
            if chars.get(i + 2) == Some(&'\'') || (chars.get(i + 1) == Some(&'\\') && chars.get(i + 3) == Some(&'\''))
            {
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

        // Zahlen
        if c.is_ascii_digit() {
            let start = i;
            while i < chars.len()
                && (chars[i].is_ascii_alphanumeric()
                    || chars[i] == '_'
                    || (chars[i] == '.' && chars.get(i + 1).is_some_and(char::is_ascii_digit)))
            {
                i += 1;
            }
            push(&mut out, "tok-num", chars[start..i].iter().collect());
            continue;
        }

        // Bezeichner
        if c.is_alphabetic() || c == '_' {
            let start = i;
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();
            let next = chars.get(i).copied();
            let class = if KEYWORDS.contains(&word.as_str()) {
                "tok-kw"
            } else if next == Some('!') {
                "tok-mac"
            } else if PRIMITIVES.contains(&word.as_str()) || word.starts_with(|ch: char| ch.is_uppercase()) {
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
