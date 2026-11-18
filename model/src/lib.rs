//! Data model of the nrzLearn courses that are stored as JSON
//! (`courses/<id>/course.json`), shared by the web app and the site tool.
//!
//! A course is written once in English (`course.json`). Translations only
//! contain the texts (`courses/<id>/strings/<lang>.json`, a flat map from key
//! to text), never code: [`strings`] extracts the translatable texts with
//! stable keys and [`apply`] puts a translation back into the course.
//! [`validate`] and [`check_translation`] enforce the rules from
//! `docs/COURSE_FORMAT.md`.

pub mod highlight;
pub mod icons;

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

/// A language of the user interface and the course texts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Language {
    /// Short code used in URLs and file names (`de`, `zh`, …).
    pub code: &'static str,
    /// Name in the language itself.
    pub name: &'static str,
    /// English name, for translators and tools.
    pub english: &'static str,
    /// BCP 47 tag for `hreflang` and `<html lang>`.
    pub hreflang: &'static str,
    pub rtl: bool,
    /// Open Graph locale.
    pub og_locale: &'static str,
}

/// All languages, English first. The order is the order of `Sprache` in the app.
pub const LANGUAGES: [Language; 40] = [
    Language { code: "en", name: "English", english: "English", hreflang: "en", rtl: false, og_locale: "en_US" },
    Language { code: "de", name: "Deutsch", english: "German", hreflang: "de", rtl: false, og_locale: "de_DE" },
    Language { code: "es", name: "Español", english: "Spanish", hreflang: "es", rtl: false, og_locale: "es_ES" },
    Language { code: "fr", name: "Français", english: "French", hreflang: "fr", rtl: false, og_locale: "fr_FR" },
    Language { code: "it", name: "Italiano", english: "Italian", hreflang: "it", rtl: false, og_locale: "it_IT" },
    Language { code: "pt", name: "Português", english: "Portuguese", hreflang: "pt", rtl: false, og_locale: "pt_BR" },
    Language { code: "nl", name: "Nederlands", english: "Dutch", hreflang: "nl", rtl: false, og_locale: "nl_NL" },
    Language { code: "pl", name: "Polski", english: "Polish", hreflang: "pl", rtl: false, og_locale: "pl_PL" },
    Language { code: "cs", name: "Čeština", english: "Czech", hreflang: "cs", rtl: false, og_locale: "cs_CZ" },
    Language { code: "sk", name: "Slovenčina", english: "Slovak", hreflang: "sk", rtl: false, og_locale: "sk_SK" },
    Language { code: "sl", name: "Slovenščina", english: "Slovenian", hreflang: "sl", rtl: false, og_locale: "sl_SI" },
    Language { code: "hr", name: "Hrvatski", english: "Croatian", hreflang: "hr", rtl: false, og_locale: "hr_HR" },
    Language { code: "hu", name: "Magyar", english: "Hungarian", hreflang: "hu", rtl: false, og_locale: "hu_HU" },
    Language { code: "ro", name: "Română", english: "Romanian", hreflang: "ro", rtl: false, og_locale: "ro_RO" },
    Language { code: "bg", name: "Български", english: "Bulgarian", hreflang: "bg", rtl: false, og_locale: "bg_BG" },
    Language { code: "el", name: "Ελληνικά", english: "Greek", hreflang: "el", rtl: false, og_locale: "el_GR" },
    Language { code: "da", name: "Dansk", english: "Danish", hreflang: "da", rtl: false, og_locale: "da_DK" },
    Language { code: "sv", name: "Svenska", english: "Swedish", hreflang: "sv", rtl: false, og_locale: "sv_SE" },
    Language { code: "fi", name: "Suomi", english: "Finnish", hreflang: "fi", rtl: false, og_locale: "fi_FI" },
    Language { code: "et", name: "Eesti", english: "Estonian", hreflang: "et", rtl: false, og_locale: "et_EE" },
    Language { code: "lv", name: "Latviešu", english: "Latvian", hreflang: "lv", rtl: false, og_locale: "lv_LV" },
    Language { code: "lt", name: "Lietuvių", english: "Lithuanian", hreflang: "lt", rtl: false, og_locale: "lt_LT" },
    Language { code: "ga", name: "Gaeilge", english: "Irish", hreflang: "ga", rtl: false, og_locale: "ga_IE" },
    Language { code: "mt", name: "Malti", english: "Maltese", hreflang: "mt", rtl: false, og_locale: "mt_MT" },
    Language { code: "nb", name: "Norsk bokmål", english: "Norwegian (Bokmål)", hreflang: "nb", rtl: false, og_locale: "nb_NO" },
    Language { code: "is", name: "Íslenska", english: "Icelandic", hreflang: "is", rtl: false, og_locale: "is_IS" },
    Language { code: "uk", name: "Українська", english: "Ukrainian", hreflang: "uk", rtl: false, og_locale: "uk_UA" },
    Language { code: "ru", name: "Русский", english: "Russian", hreflang: "ru", rtl: false, og_locale: "ru_RU" },
    Language { code: "tr", name: "Türkçe", english: "Turkish", hreflang: "tr", rtl: false, og_locale: "tr_TR" },
    Language { code: "ca", name: "Català", english: "Catalan", hreflang: "ca", rtl: false, og_locale: "ca_ES" },
    Language { code: "sr", name: "Српски", english: "Serbian (Cyrillic)", hreflang: "sr", rtl: false, og_locale: "sr_RS" },
    Language { code: "sq", name: "Shqip", english: "Albanian", hreflang: "sq", rtl: false, og_locale: "sq_AL" },
    Language { code: "mk", name: "Македонски", english: "Macedonian", hreflang: "mk", rtl: false, og_locale: "mk_MK" },
    Language { code: "ja", name: "日本語", english: "Japanese", hreflang: "ja", rtl: false, og_locale: "ja_JP" },
    Language { code: "ko", name: "한국어", english: "Korean", hreflang: "ko", rtl: false, og_locale: "ko_KR" },
    Language { code: "zh", name: "简体中文", english: "Chinese (Simplified)", hreflang: "zh-Hans", rtl: false, og_locale: "zh_CN" },
    Language { code: "hi", name: "हिन्दी", english: "Hindi", hreflang: "hi", rtl: false, og_locale: "hi_IN" },
    Language { code: "ar", name: "العربية", english: "Arabic", hreflang: "ar", rtl: true, og_locale: "ar_AR" },
    Language { code: "id", name: "Bahasa Indonesia", english: "Indonesian", hreflang: "id", rtl: false, og_locale: "id_ID" },
    Language { code: "vi", name: "Tiếng Việt", english: "Vietnamese", hreflang: "vi", rtl: false, og_locale: "vi_VN" },
];

pub fn language(code: &str) -> Option<&'static Language> {
    LANGUAGES.iter().find(|l| l.code == code)
}

/// A programming language course.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CourseInfo {
    /// ID in URLs and folder names.
    pub id: &'static str,
    pub name: &'static str,
    /// Two- or three-letter monogram (fallback when no logo can be shown).
    pub mono: &'static str,
    /// Accent colour of the tile and the colour of the logo on it.
    pub color: &'static str,
    pub on_color: &'static str,
    /// Group on the start page and in the course menu (see [`GROUPS`]).
    pub group: &'static str,
}

/// Groups of the course picker, in this order (texts: `app.group.<id>`).
pub const GROUPS: [&str; 8] = ["popular", "web", "systems", "functional", "lisp", "data", "classics", "lowlevel"];

const fn c(id: &'static str, name: &'static str, mono: &'static str, color: &'static str, on_color: &'static str, group: &'static str) -> CourseInfo {
    CourseInfo { id, name, mono, color, on_color, group }
}

/// All courses in the order of the course picker (grouped). Rust is built into
/// the app, the others are JSON courses in `courses/<id>/`.
pub const COURSES: [CourseInfo; 48] = [
    c("python", "Python", "Py", "#3776ab", "#ffffff", "popular"),
    c("javascript", "JavaScript", "JS", "#f7df1e", "#111111", "popular"),
    c("typescript", "TypeScript", "TS", "#3178c6", "#ffffff", "popular"),
    c("java", "Java", "Jv", "#e76f00", "#ffffff", "popular"),
    c("csharp", "C#", "C#", "#68217a", "#ffffff", "popular"),
    c("rust", "Rust", "Rs", "#ce422b", "#ffffff", "popular"),
    c("kotlin", "Kotlin", "Kt", "#7f52ff", "#ffffff", "web"),
    c("swift", "Swift", "Sw", "#f05138", "#ffffff", "web"),
    c("dart", "Dart", "Dt", "#0175c2", "#ffffff", "web"),
    c("php", "PHP", "PHP", "#777bb4", "#ffffff", "web"),
    c("ruby", "Ruby", "Rb", "#cc342d", "#ffffff", "web"),
    c("objective-c", "Objective-C", "OC", "#438eff", "#ffffff", "web"),
    c("crystal", "Crystal", "Cr", "#24292e", "#ffffff", "web"),
    c("solidity", "Solidity", "Sol", "#363636", "#ffffff", "web"),
    c("c", "C", "C", "#a8b9cc", "#111111", "systems"),
    c("cpp", "C++", "C++", "#00599c", "#ffffff", "systems"),
    c("go", "Go", "Go", "#00add8", "#ffffff", "systems"),
    c("zig", "Zig", "Zig", "#f7a41d", "#111111", "systems"),
    c("nim", "Nim", "Nim", "#ffe953", "#111111", "systems"),
    c("d", "D", "D", "#b03931", "#ffffff", "systems"),
    c("v", "V", "V", "#5d87bf", "#ffffff", "systems"),
    c("odin", "Odin", "Od", "#3882d2", "#ffffff", "systems"),
    c("haskell", "Haskell", "Hs", "#5d4f85", "#ffffff", "functional"),
    c("ocaml", "OCaml", "ML", "#ec6813", "#ffffff", "functional"),
    c("fsharp", "F#", "F#", "#378bba", "#ffffff", "functional"),
    c("scala", "Scala", "Sc", "#dc322f", "#ffffff", "functional"),
    c("elixir", "Elixir", "Ex", "#4b275f", "#ffffff", "functional"),
    c("erlang", "Erlang", "Erl", "#a90533", "#ffffff", "functional"),
    c("gleam", "Gleam", "Gl", "#ffaff3", "#111111", "functional"),
    c("clojure", "Clojure", "Clj", "#5881d8", "#ffffff", "lisp"),
    c("racket", "Racket", "Rkt", "#9f1d20", "#ffffff", "lisp"),
    c("scheme", "Scheme", "Scm", "#1e4aec", "#ffffff", "lisp"),
    c("common-lisp", "Common Lisp", "CL", "#3fb68b", "#ffffff", "lisp"),
    c("prolog", "Prolog", "Pl", "#74283c", "#ffffff", "lisp"),
    c("sql", "SQL", "SQL", "#0e7c86", "#ffffff", "data"),
    c("r", "R", "R", "#276dc3", "#ffffff", "data"),
    c("julia", "Julia", "Jl", "#9558b2", "#ffffff", "data"),
    c("bash", "Bash", "Sh", "#4eaa25", "#ffffff", "data"),
    c("lua", "Lua", "Lua", "#000080", "#ffffff", "data"),
    c("perl", "Perl", "Pl", "#0073a1", "#ffffff", "data"),
    c("fortran", "Fortran", "F90", "#734f96", "#ffffff", "classics"),
    c("cobol", "COBOL", "CBL", "#2b4c7e", "#ffffff", "classics"),
    c("pascal", "Pascal", "Pas", "#e3f171", "#111111", "classics"),
    c("delphi", "Delphi", "Dpr", "#e62431", "#ffffff", "classics"),
    c("ada", "Ada", "Ada", "#1a5fb4", "#ffffff", "classics"),
    c("smalltalk", "Smalltalk", "St", "#596706", "#ffffff", "classics"),
    c("assembly", "Assembly", "Asm", "#57534e", "#ffffff", "lowlevel"),
    c("machine-code", "Machine code", "01", "#1f2937", "#4ade80", "lowlevel"),
];

pub fn course_info(id: &str) -> Option<&'static CourseInfo> {
    COURSES.iter().find(|c| c.id == id)
}

pub const UNITS_MIN: usize = 6;
pub const UNITS_MAX: usize = 12;
pub const EXERCISES_MIN: usize = 10;
pub const EXERCISES_MAX: usize = 16;
/// Distinct exercise types a unit must use at least.
pub const TYPES_MIN: usize = 5;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Course {
    /// Course ID, equal to the folder name, e.g. `python`.
    pub course: String,
    /// Language of the texts, e.g. `en`.
    pub lang: String,
    pub units: Vec<Unit>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Unit {
    /// Stable ID, lowercase letters, digits and `-`, e.g. `variables`.
    pub id: String,
    pub title: String,
    /// Short group label shown above the title, e.g. "First steps".
    pub chapter: String,
    /// One sentence: what the unit is about.
    pub summary: String,
    pub lesson: Vec<Block>,
    /// Cheat sheet card: 2–5 short bullet points.
    pub card: Vec<String>,
    pub exercises: Vec<Exercise>,
}

/// A block of the theory lesson. Texts support `code` and **bold**.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Block {
    Heading {
        text: String,
    },
    Text {
        md: String,
    },
    /// Code with its exact output (stdout), verified by running it.
    Code {
        code: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        output: Option<String>,
    },
    Tip {
        md: String,
    },
    Warning {
        md: String,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Exercise {
    /// Pick the one right option (options are texts, code in backticks).
    Choice {
        id: String,
        prompt: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        code: Option<String>,
        options: Vec<String>,
        answer: usize,
        explain: String,
    },
    /// Type the answer, e.g. the output of the code. Answers are not translated.
    Input {
        id: String,
        prompt: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        code: Option<String>,
        answers: Vec<String>,
        explain: String,
    },
    TrueFalse {
        id: String,
        statement: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        code: Option<String>,
        answer: bool,
        explain: String,
    },
    /// Code with one gap `___`; the options are code (not translated).
    Gap {
        id: String,
        prompt: String,
        code: String,
        options: Vec<String>,
        answer: usize,
        explain: String,
    },
    /// Build one line of code from blocks (tokens in the right order) plus distractors.
    Blocks {
        id: String,
        prompt: String,
        tokens: Vec<String>,
        distractors: Vec<String>,
        explain: String,
    },
    /// Lines of code in the right order (they are shown shuffled).
    Order {
        id: String,
        prompt: String,
        lines: Vec<String>,
        explain: String,
    },
    /// Match code (left, not translated) to a description (right, translated).
    Pairs {
        id: String,
        prompt: String,
        pairs: Vec<[String; 2]>,
    },
    /// Tap the line with the bug (0-based index into `lines`).
    Bug {
        id: String,
        prompt: String,
        lines: Vec<String>,
        bug: usize,
        explain: String,
    },
}

impl Exercise {
    pub fn id(&self) -> &str {
        match self {
            Self::Choice { id, .. }
            | Self::Input { id, .. }
            | Self::TrueFalse { id, .. }
            | Self::Gap { id, .. }
            | Self::Blocks { id, .. }
            | Self::Order { id, .. }
            | Self::Pairs { id, .. }
            | Self::Bug { id, .. } => id,
        }
    }

    pub const fn kind(&self) -> &'static str {
        match self {
            Self::Choice { .. } => "choice",
            Self::Input { .. } => "input",
            Self::TrueFalse { .. } => "true_false",
            Self::Gap { .. } => "gap",
            Self::Blocks { .. } => "blocks",
            Self::Order { .. } => "order",
            Self::Pairs { .. } => "pairs",
            Self::Bug { .. } => "bug",
        }
    }
}

// ─────────────────────────────── Texts and translations ───────────────────────────────

/// Visits every translatable text of a course in a fixed order, with its key.
fn walk_texts(course: &mut Course, mut f: impl FnMut(String, &mut String)) {
    for u in &mut course.units {
        let uid = u.id.clone();
        f(format!("{uid}.title"), &mut u.title);
        f(format!("{uid}.chapter"), &mut u.chapter);
        f(format!("{uid}.summary"), &mut u.summary);
        for (i, b) in u.lesson.iter_mut().enumerate() {
            match b {
                Block::Heading { text } => f(format!("{uid}.lesson.{i}"), text),
                Block::Text { md } | Block::Tip { md } | Block::Warning { md } => f(format!("{uid}.lesson.{i}"), md),
                Block::Code { .. } => {}
            }
        }
        for (i, c) in u.card.iter_mut().enumerate() {
            f(format!("{uid}.card.{i}"), c);
        }
        for e in &mut u.exercises {
            let eid = e.id().to_string();
            match e {
                Exercise::Choice { prompt, options, explain, .. } => {
                    f(format!("{eid}.prompt"), prompt);
                    for (i, o) in options.iter_mut().enumerate() {
                        f(format!("{eid}.options.{i}"), o);
                    }
                    f(format!("{eid}.explain"), explain);
                }
                Exercise::TrueFalse { statement, explain, .. } => {
                    f(format!("{eid}.statement"), statement);
                    f(format!("{eid}.explain"), explain);
                }
                Exercise::Input { prompt, explain, .. }
                | Exercise::Gap { prompt, explain, .. }
                | Exercise::Blocks { prompt, explain, .. }
                | Exercise::Order { prompt, explain, .. }
                | Exercise::Bug { prompt, explain, .. } => {
                    f(format!("{eid}.prompt"), prompt);
                    f(format!("{eid}.explain"), explain);
                }
                Exercise::Pairs { prompt, pairs, .. } => {
                    f(format!("{eid}.prompt"), prompt);
                    for (i, p) in pairs.iter_mut().enumerate() {
                        f(format!("{eid}.pairs.{i}"), &mut p[1]);
                    }
                }
            }
        }
    }
}

/// All translatable texts of a course, in course order.
pub fn strings(course: &Course) -> Vec<(String, String)> {
    let mut c = course.clone();
    let mut out = Vec::new();
    walk_texts(&mut c, |k, v| out.push((k, v.clone())));
    out
}

/// The course with all texts replaced by a translation. Missing keys keep
/// the original text; use [`check_translation`] first.
pub fn apply(course: &Course, lang: &str, texts: &BTreeMap<String, String>) -> Course {
    let mut c = course.clone();
    c.lang = lang.to_string();
    walk_texts(&mut c, |k, v| {
        if let Some(t) = texts.get(&k) {
            v.clone_from(t);
        }
    });
    c
}

/// Typography the catalogs leave to the build. French puts a no-break space
/// before `?`, `!`, `;` and `:` and inside « »: a plain space there becomes a
/// narrow no-break space (U+202F) before `?`, `!` and `;` and a no-break space
/// (U+00A0) before `:` and inside guillemets, so that a line never starts with
/// the punctuation. Code spans (between backticks) stay as they are; other
/// languages are returned unchanged.
pub fn typography(lang: &str, text: &str) -> String {
    if lang != "fr" {
        return text.to_string();
    }
    let mut out = String::with_capacity(text.len() + 8);
    for (i, part) in text.split('`').enumerate() {
        if i > 0 {
            out.push('`');
        }
        if i % 2 == 1 {
            out.push_str(part);
            continue;
        }
        let chars: Vec<char> = part.chars().collect();
        for (j, &ch) in chars.iter().enumerate() {
            if ch == ' ' {
                match (j.checked_sub(1).map(|p| chars[p]), chars.get(j + 1)) {
                    (_, Some('?' | '!' | ';')) => {
                        out.push('\u{202F}');
                        continue;
                    }
                    (_, Some(':' | '»')) | (Some('«'), _) => {
                        out.push('\u{00A0}');
                        continue;
                    }
                    _ => {}
                }
            }
            out.push(ch);
        }
    }
    out
}

/// The `code` spans of a text (between backticks), sorted.
pub fn code_spans(text: &str) -> Vec<String> {
    let mut v: Vec<String> = text.split('`').skip(1).step_by(2).map(str::to_string).collect();
    v.sort();
    v
}

/// Problems of a translation against the English texts.
pub fn check_translation(english: &[(String, String)], texts: &BTreeMap<String, String>) -> Vec<String> {
    let mut errors = Vec::new();
    let keys: BTreeSet<&str> = english.iter().map(|(k, _)| k.as_str()).collect();
    for k in texts.keys() {
        if !keys.contains(k.as_str()) {
            errors.push(format!("{k}: unknown key"));
        }
    }
    for (k, en) in english {
        let Some(t) = texts.get(k) else {
            errors.push(format!("{k}: missing"));
            continue;
        };
        errors.extend(text_problems(k, t));
        if code_spans(t) != code_spans(en) {
            errors.push(format!("{k}: `code` spans differ from English {:?}", code_spans(en)));
        }
        if t.trim() == en.trim() && en.chars().filter(|c| c.is_alphabetic()).count() > 24 && code_spans(en).join("").len() * 2 < en.len() {
            errors.push(format!("{k}: looks untranslated"));
        }
    }
    errors
}

/// Formal problems of a single text: empty, unbalanced markup, line breaks.
pub fn text_problems(key: &str, t: &str) -> Vec<String> {
    let mut errors = Vec::new();
    if t.trim().is_empty() {
        errors.push(format!("{key}: empty"));
    }
    if t != t.trim() {
        errors.push(format!("{key}: leading or trailing whitespace"));
    }
    if !t.matches('`').count().is_multiple_of(2) {
        errors.push(format!("{key}: odd number of backticks"));
    }
    // `**` inside a code span is code (e.g. the power operator), not bold.
    let ausserhalb: String = t.split('`').step_by(2).collect();
    if !ausserhalb.matches("**").count().is_multiple_of(2) {
        errors.push(format!("{key}: odd number of ** markers"));
    }
    if t.contains('\n') {
        errors.push(format!("{key}: line break (one paragraph per text)"));
    }
    if t.contains('<') && t.split('`').step_by(2).any(|outside| outside.contains('<')) {
        errors.push(format!("{key}: `<` outside of a code span (no HTML)"));
    }
    // A word with Latin and Cyrillic or Greek letters is almost always a look-alike
    // typo (Latin `a` in a Cyrillic word); it breaks search and screen readers.
    let latin = |c: char| c.is_ascii_alphabetic();
    let other = |c: char| matches!(c, '\u{0370}'..='\u{03FF}' | '\u{0400}'..='\u{04FF}');
    if let Some(w) = ausserhalb.split(|c: char| !c.is_alphanumeric()).find(|w| w.chars().any(latin) && w.chars().any(other)) {
        errors.push(format!("{key}: word mixes Latin with Cyrillic or Greek letters: {w}"));
    }
    errors
}

// ─────────────────────────────── Validation ───────────────────────────────

fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id.split('-').all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()))
}

fn code_problems(what: &str, code: &str) -> Vec<String> {
    let mut errors = Vec::new();
    if code.trim().is_empty() {
        errors.push(format!("{what}: empty code"));
    }
    if code.contains('\t') {
        errors.push(format!("{what}: tab in code (use spaces)"));
    }
    if code.lines().any(|l| l != l.trim_end()) {
        errors.push(format!("{what}: trailing whitespace in code"));
    }
    if code.starts_with('\n') || code.ends_with('\n') {
        errors.push(format!("{what}: code starts or ends with an empty line"));
    }
    errors
}

fn unique(items: &[String]) -> bool {
    items.iter().collect::<BTreeSet<_>>().len() == items.len()
}

/// All rule violations of an (English) course. Empty means valid.
#[allow(clippy::too_many_lines)]
pub fn validate(c: &Course) -> Vec<String> {
    let mut errors = Vec::new();
    let mut err = |s: String| errors.push(s);

    if !valid_id(&c.course) {
        err(format!("course: invalid id {:?}", c.course));
    }
    if !(UNITS_MIN..=UNITS_MAX).contains(&c.units.len()) {
        err(format!("course: {} units, allowed {UNITS_MIN}–{UNITS_MAX}", c.units.len()));
    }
    let mut ids = BTreeSet::new();
    for u in &c.units {
        let uid = &u.id;
        if !valid_id(uid) {
            err(format!("unit {uid:?}: invalid id"));
        }
        if !ids.insert(uid.clone()) {
            err(format!("unit {uid}: duplicate id"));
        }
        if u.title.chars().count() > 40 {
            err(format!("{uid}.title: longer than 40 characters"));
        }
        if u.chapter.chars().count() > 24 {
            err(format!("{uid}.chapter: longer than 24 characters"));
        }
        if u.summary.chars().count() > 140 {
            err(format!("{uid}.summary: longer than 140 characters"));
        }
        let texts = u.lesson.iter().filter(|b| matches!(b, Block::Text { .. })).count();
        let codes = u.lesson.iter().filter(|b| matches!(b, Block::Code { .. })).count();
        if texts < 2 || codes < 2 {
            err(format!("{uid}.lesson: needs at least 2 text and 2 code blocks (has {texts} and {codes})"));
        }
        if u.lesson.len() > 16 {
            err(format!("{uid}.lesson: more than 16 blocks"));
        }
        for (i, b) in u.lesson.iter().enumerate() {
            if let Block::Code { code, output } = b {
                for e in code_problems(&format!("{uid}.lesson.{i}"), code) {
                    err(e);
                }
                if output.as_ref().is_some_and(|o| o.trim().is_empty()) {
                    err(format!("{uid}.lesson.{i}: empty output (leave it out instead)"));
                }
            }
        }
        if !(2..=5).contains(&u.card.len()) {
            err(format!("{uid}.card: needs 2–5 points"));
        }
        if !(EXERCISES_MIN..=EXERCISES_MAX).contains(&u.exercises.len()) {
            err(format!("{uid}: {} exercises, allowed {EXERCISES_MIN}–{EXERCISES_MAX}", u.exercises.len()));
        }
        let kinds: BTreeSet<&str> = u.exercises.iter().map(Exercise::kind).collect();
        if kinds.len() < TYPES_MIN {
            err(format!("{uid}: uses {} exercise types, needs at least {TYPES_MIN}", kinds.len()));
        }
        for e in &u.exercises {
            let eid = e.id();
            if !valid_id(eid) || !eid.starts_with(&format!("{uid}-")) {
                err(format!("exercise {eid:?}: id must be \"{uid}-<n>\""));
            }
            if !ids.insert(eid.to_string()) {
                err(format!("exercise {eid}: duplicate id"));
            }
            let code_of = |what: &str, code: &Option<String>| code.as_ref().map(|c| code_problems(&format!("{eid}.{what}"), c)).unwrap_or_default();
            match e {
                Exercise::Choice { options, answer, code, .. } => {
                    if !(3..=4).contains(&options.len()) || *answer >= options.len() || !unique(options) {
                        err(format!("{eid}: 3–4 different options and a valid answer index"));
                    }
                    for x in code_of("code", code) {
                        err(x);
                    }
                }
                Exercise::Input { answers, code, .. } => {
                    if answers.is_empty() || answers.iter().any(|a| a.trim().is_empty() || a.chars().count() > 60) {
                        err(format!("{eid}: 1+ answers, each 1–60 characters"));
                    }
                    for x in code_of("code", code) {
                        err(x);
                    }
                }
                Exercise::TrueFalse { code, .. } => {
                    for x in code_of("code", code) {
                        err(x);
                    }
                }
                Exercise::Gap { code, options, answer, .. } => {
                    if code.matches("___").count() != 1 {
                        err(format!("{eid}: code needs exactly one ___"));
                    }
                    if !(3..=4).contains(&options.len()) || *answer >= options.len() || !unique(options) {
                        err(format!("{eid}: 3–4 different options and a valid answer index"));
                    }
                    for x in code_problems(&format!("{eid}.code"), code) {
                        err(x);
                    }
                }
                Exercise::Blocks { tokens, distractors, .. } => {
                    if !(3..=10).contains(&tokens.len()) || !(1..=4).contains(&distractors.len()) {
                        err(format!("{eid}: 3–10 tokens and 1–4 distractors"));
                    }
                    if distractors.iter().any(|d| tokens.contains(d)) || !unique(distractors) {
                        err(format!("{eid}: distractors must differ from the tokens and each other"));
                    }
                    if tokens.iter().chain(distractors).any(|t| t.trim().is_empty() || t != t.trim()) {
                        err(format!("{eid}: empty token or token with outer spaces"));
                    }
                }
                Exercise::Order { lines, .. } => {
                    if !(3..=8).contains(&lines.len()) || !unique(lines) {
                        err(format!("{eid}: 3–8 different lines"));
                    }
                }
                Exercise::Pairs { pairs, .. } => {
                    let left: Vec<String> = pairs.iter().map(|p| p[0].clone()).collect();
                    let right: Vec<String> = pairs.iter().map(|p| p[1].clone()).collect();
                    if !(3..=5).contains(&pairs.len()) || !unique(&left) || !unique(&right) {
                        err(format!("{eid}: 3–5 pairs with different left and right sides"));
                    }
                }
                Exercise::Bug { lines, bug, .. } => {
                    if !(3..=12).contains(&lines.len()) || *bug >= lines.len() {
                        err(format!("{eid}: 3–12 lines and a valid bug index"));
                    }
                }
            }
            if let Exercise::Order { lines, .. } | Exercise::Bug { lines, .. } = e
                && lines.iter().any(|l| l.contains('\t') || l != l.trim_end() || l.trim().is_empty())
            {
                err(format!("{eid}: lines must not be empty, contain tabs or trailing spaces"));
            }
        }
    }
    for (k, v) in strings(c) {
        for e in text_problems(&k, &v) {
            err(e);
        }
    }
    errors
}

#[cfg(test)]
mod tests {
    use super::*;

    fn exercise(uid: &str, n: usize) -> Exercise {
        let id = format!("{uid}-{n}");
        match n % 8 {
            0 => Exercise::Choice { id, prompt: "Which prints?".into(), code: None, options: vec!["`print(1)`".into(), "`echo 1`".into(), "`say 1`".into()], answer: 0, explain: "Use `print`.".into() },
            1 => Exercise::Input { id, prompt: "Output?".into(), code: Some("print(2 + 3)".into()), answers: vec!["5".into()], explain: "2 + 3 is 5.".into() },
            2 => Exercise::TrueFalse { id, statement: "Python is fun.".into(), code: None, answer: true, explain: "Sure.".into() },
            3 => Exercise::Gap { id, prompt: "Fill the gap.".into(), code: "___(\"hi\")".into(), options: vec!["print".into(), "echo".into(), "say".into()], answer: 0, explain: "It is `print`.".into() },
            4 => Exercise::Blocks { id, prompt: "Build it.".into(), tokens: vec!["print".into(), "(".into(), ")".into()], distractors: vec!["echo".into()], explain: "Like this.".into() },
            5 => Exercise::Order { id, prompt: "Order.".into(), lines: vec!["a = 1".into(), "b = 2".into(), "print(a + b)".into()], explain: "Define first.".into() },
            6 => Exercise::Pairs { id, prompt: "Match.".into(), pairs: vec![["`int`".into(), "whole number".into()], ["`str`".into(), "text".into()], ["`bool`".into(), "yes or no".into()]] },
            _ => Exercise::Bug { id, prompt: "Bug?".into(), lines: vec!["a = 1".into(), "print(a".into(), "b = 2".into()], bug: 1, explain: "Missing `)`.".into() },
        }
    }

    #[test]
    fn mixed_scripts_are_flagged() {
        assert!(text_problems("k", "Операторот кревa број").iter().any(|e| e.contains("mixes")));
        assert!(text_problems("k", "Python-а и `print`").is_empty());
        assert!(text_problems("k", "Код `x = а`").is_empty());
    }

    #[test]
    fn every_course_has_a_logo_and_a_group() {
        let mut ids = std::collections::BTreeSet::new();
        for c in &COURSES {
            assert!(ids.insert(c.id), "duplicate course {}", c.id);
            assert!(valid_id(c.id), "{}", c.id);
            assert!(icons::icon(c.id).is_some(), "no logo for {}", c.id);
            assert!(GROUPS.contains(&c.group), "unknown group of {}", c.id);
        }
    }

    #[test]
    fn french_typography() {
        assert_eq!(typography("fr", "Quelle ligne ? Tape : « oui » ; bravo !"), "Quelle ligne\u{202F}? Tape\u{00A0}: «\u{00A0}oui\u{00A0}»\u{202F}; bravo\u{202F}!");
        assert_eq!(typography("fr", "Que fait `a ? b : c` ?"), "Que fait `a ? b : c`\u{202F}?");
        assert_eq!(typography("fr", "Affiche Score: 10"), "Affiche Score: 10");
        assert_eq!(typography("de", "Was ist das ?"), "Was ist das ?");
    }

    fn course() -> Course {
        let unit = |uid: String| Unit {
            title: "Hello".into(),
            chapter: "Start".into(),
            summary: "The first steps.".into(),
            lesson: vec![
                Block::Text { md: "Python prints with `print`.".into() },
                Block::Code { code: "print(\"Hi\")".into(), output: Some("Hi".into()) },
                Block::Text { md: "**Numbers** work too.".into() },
                Block::Code { code: "print(1 + 1)".into(), output: Some("2".into()) },
            ],
            card: vec!["`print(x)` prints".into(), "`#` starts a comment".into()],
            exercises: (0..10).map(|n| exercise(&uid, n)).collect(),
            id: uid,
        };
        Course { course: "python".into(), lang: "en".into(), units: (0..6).map(|i| unit(format!("u{i}"))).collect() }
    }

    #[test]
    fn valid_course_and_translation_round_trip() {
        let c = course();
        assert_eq!(validate(&c), Vec::<String>::new());
        let en = strings(&c);
        let mut de: BTreeMap<String, String> = en.iter().map(|(k, v)| (k.clone(), format!("{v} (de)"))).collect();
        assert_eq!(check_translation(&en, &de), Vec::<String>::new());
        let t = apply(&c, "de", &de);
        assert_eq!(t.lang, "de");
        assert_eq!(t.units[0].title, "Hello (de)");
        assert_eq!(validate(&t), Vec::<String>::new());

        // Changed code spans and missing keys are reported.
        de.insert("u0-0.explain".into(), "Nimm `echo`.".into());
        de.remove("u0.title");
        let e = check_translation(&en, &de);
        assert!(e.iter().any(|x| x.starts_with("u0-0.explain: `code` spans differ")), "{e:?}");
        assert!(e.iter().any(|x| x == "u0.title: missing"), "{e:?}");
    }

    #[test]
    fn rules_are_enforced() {
        let mut c = course();
        c.units[0].exercises[0] = Exercise::Choice { id: "u0-0".into(), prompt: "x".into(), code: None, options: vec!["a".into(), "a".into(), "b".into()], answer: 5, explain: "y".into() };
        c.units[1].lesson.push(Block::Code { code: "print(1)\t".into(), output: None });
        c.units[2].lesson.push(Block::Text { md: "Use `**` for powers and **bold** text.".into() });
        let e = validate(&c);
        assert!(e.iter().any(|x| x.starts_with("u0-0: 3–4 different options")), "{e:?}");
        assert!(e.iter().any(|x| x.contains("tab in code")), "{e:?}");
    }
}
