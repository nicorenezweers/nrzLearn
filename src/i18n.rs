//! Languages of the app (40, see `nrzlearn_model::LANGUAGES`).
//!
//! - Interface texts come from the catalogs `i18n/<lang>.json` and are
//!   compiled in by `build.rs`: `t!(l, NAV_PATH)` reads the key
//!   `app.nav.path`, `tf!(l, LEVEL_N, n = 3)` fills `{n}`.
//! - The built-in Rust course is written in English, German and Spanish with
//!   `tr!(l, en, de, es)`; every other language gets the English text.
//! - The JSON courses are translated into every language (`courses/`).

use std::fmt::Display;
use std::str::FromStr;

use dioxus::prelude::*;
use nrzlearn_model::{LANGUAGES, Language};

include!(concat!(env!("OUT_DIR"), "/i18n.rs"));

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum Sprache {
    #[default]
    En,
    De,
    Es,
    Fr,
    It,
    Pt,
    Nl,
    Pl,
    Cs,
    Sk,
    Sl,
    Hr,
    Hu,
    Ro,
    Bg,
    El,
    Da,
    Sv,
    Fi,
    Et,
    Lv,
    Lt,
    Ga,
    Mt,
    Nb,
    Is,
    Uk,
    Ru,
    Tr,
    Ca,
    Sr,
    Sq,
    Mk,
    Ja,
    Ko,
    Zh,
    Hi,
    Ar,
    Id,
    Vi,
}

/// Picks the text of the Rust course: German, Spanish, otherwise English.
#[macro_export]
macro_rules! tr {
    ($l:expr, $en:expr, $de:expr, $es:expr $(,)?) => {
        match $l {
            $crate::i18n::Sprache::De => $de,
            $crate::i18n::Sprache::Es => $es,
            _ => $en,
        }
    };
}

/// An interface text from the catalog: `t!(l, NAV_PATH)` → key `app.nav.path`.
#[macro_export]
macro_rules! t {
    ($l:expr, $k:ident) => {
        $crate::i18n::text($l, $crate::i18n::k::$k)
    };
}

/// An interface text with placeholders: `tf!(l, LEVEL_N, n = 3)` fills `{n}`.
#[macro_export]
macro_rules! tf {
    ($l:expr, $k:ident, $($name:ident = $wert:expr),+ $(,)?) => {
        $crate::i18n::fuellen($crate::i18n::text($l, $crate::i18n::k::$k), &[$((stringify!($name), &$wert as &dyn ::std::fmt::Display)),+])
    };
}

const SPEICHER: &str = "nrzlearn.lang";

impl Sprache {
    pub const ALLE: [Self; 40] = [
        Self::En, Self::De, Self::Es, Self::Fr, Self::It, Self::Pt, Self::Nl, Self::Pl, Self::Cs, Self::Sk,
        Self::Sl, Self::Hr, Self::Hu, Self::Ro, Self::Bg, Self::El, Self::Da, Self::Sv, Self::Fi, Self::Et,
        Self::Lv, Self::Lt, Self::Ga, Self::Mt, Self::Nb, Self::Is, Self::Uk, Self::Ru, Self::Tr, Self::Ca,
        Self::Sr, Self::Sq, Self::Mk, Self::Ja, Self::Ko, Self::Zh, Self::Hi, Self::Ar, Self::Id, Self::Vi,
    ];

    /// Languages in which the Rust course is written.
    pub const RUST: [Self; 3] = [Self::En, Self::De, Self::Es];

    pub const fn info(self) -> &'static Language {
        &LANGUAGES[self as usize]
    }

    pub const fn code(self) -> &'static str {
        self.info().code
    }

    /// Name in the language itself.
    pub const fn name(self) -> &'static str {
        self.info().name
    }

    /// BCP 47 tag for `lang` attributes.
    pub const fn tag(self) -> &'static str {
        self.info().hreflang
    }

    pub const fn rtl(self) -> bool {
        self.info().rtl
    }

    /// The language the Rust course is shown in.
    pub const fn rust(self) -> Self {
        match self {
            Self::De => Self::De,
            Self::Es => Self::Es,
            _ => Self::En,
        }
    }

    /// Index 0–2 into the English/German/Spanish texts of the Rust course.
    pub const fn rust_index(self) -> usize {
        match self {
            Self::De => 1,
            Self::Es => 2,
            _ => 0,
        }
    }

    /// From a code like `de`, `de-CH`, `pt-BR`, `zh-Hans`, `no`.
    pub fn aus_code(code: &str) -> Option<Self> {
        let c = code.trim().to_ascii_lowercase().replace('_', "-");
        let basis = c.split('-').next().unwrap_or_default();
        let basis = match basis {
            "no" | "nn" => "nb",
            "bs" | "sh" => "hr",
            other => other,
        };
        Self::ALLE.into_iter().find(|s| s.code() == basis)
    }

    /// The language picked in this browser, if any.
    pub fn gespeichert() -> Option<Self> {
        let s = web_sys::window()?.local_storage().ok().flatten()?.get_item(SPEICHER).ok().flatten()?;
        Self::aus_code(&s)
    }

    /// Saved choice, otherwise the first supported browser language, otherwise English.
    pub fn aus_browser() -> Self {
        if let Some(s) = Self::gespeichert() {
            return s;
        }
        let Some(nav) = web_sys::window().map(|w| w.navigator()) else { return Self::En };
        nav.languages()
            .iter()
            .filter_map(|v| v.as_string())
            .chain(nav.language())
            .find_map(|l| Self::aus_code(&l))
            .unwrap_or_default()
    }

    /// Remembers the language in this browser and sets `lang`/`dir` of the page.
    pub fn merken(self) {
        if let Some(s) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
            let _ = s.set_item(SPEICHER, self.code());
        }
        document::eval(&format!(
            "document.documentElement.lang='{}';document.documentElement.dir='{}';",
            self.tag(),
            if self.rtl() { "rtl" } else { "ltr" }
        ));
    }
}

impl FromStr for Sprache {
    type Err = String;

    /// Only exact codes, for URLs (`/de/…`).
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::ALLE.into_iter().find(|l| l.code() == s).ok_or_else(|| format!("unknown language {s:?}"))
    }
}

impl Display for Sprache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.code())
    }
}

/// An interface text (see the `t!` macro).
pub fn text(l: Sprache, key: usize) -> &'static str {
    TEXTE[l as usize][key]
}

/// Replaces `{name}` placeholders.
pub fn fuellen(text: &str, werte: &[(&str, &dyn Display)]) -> String {
    let mut s = text.to_string();
    for (name, wert) in werte {
        s = s.replace(&format!("{{{name}}}"), &wert.to_string());
    }
    s
}

pub fn use_sprache() -> Sprache {
    use_context::<Signal<Sprache>>()()
}

pub fn use_sprache_signal() -> Signal<Sprache> {
    use_context::<Signal<Sprache>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn languages_match_the_model() {
        assert_eq!(Sprache::ALLE.len(), LANGUAGES.len());
        for (i, s) in Sprache::ALLE.iter().enumerate() {
            assert_eq!(*s as usize, i);
            assert_eq!(s.code().parse::<Sprache>(), Ok(*s));
        }
        assert_eq!(Sprache::aus_code("de-CH"), Some(Sprache::De));
        assert_eq!(Sprache::aus_code("zh-Hans-CN"), Some(Sprache::Zh));
        assert_eq!(Sprache::aus_code("no"), Some(Sprache::Nb));
        assert_eq!(Sprache::aus_code("xx"), None);
        assert_eq!(fuellen("{a} und {b}", &[("a", &1), ("b", &"zwei")]), "1 und zwei");
    }
}
