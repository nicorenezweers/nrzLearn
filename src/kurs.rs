//! The programming languages you can learn. Rust is built into the app (with
//! generated exercises), all others are JSON courses (see `inhalt.rs`).

use std::fmt::Display;
use std::str::FromStr;

use nrzlearn_model::{COURSES, CourseInfo};

use crate::i18n::Sprache;

/// A course: an index into [`COURSES`].
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub struct Kurs(u8);

const SPEICHER: &str = "nrzlearn.course";
const ANZAHL: usize = COURSES.len();

const fn gleich(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

impl Default for Kurs {
    fn default() -> Self {
        Self::RUST
    }
}

impl Kurs {
    pub const RUST: Self = Self::finde("rust");
    #[cfg(test)]
    pub const PYTHON: Self = Self::finde("python");

    /// All courses in the order of the course picker.
    pub const ALLE: [Self; ANZAHL] = {
        let mut a = [Self(0); ANZAHL];
        let mut i = 0;
        while i < ANZAHL {
            a[i] = Self(i as u8);
            i += 1;
        }
        a
    };

    const fn finde(id: &str) -> Self {
        let mut i = 0;
        while i < ANZAHL {
            if gleich(COURSES[i].id, id) {
                return Self(i as u8);
            }
            i += 1;
        }
        panic!("unknown course id")
    }

    pub const fn info(self) -> &'static CourseInfo {
        &COURSES[self.0 as usize]
    }

    pub const fn id(self) -> &'static str {
        self.info().id
    }

    pub const fn name(self) -> &'static str {
        self.info().name
    }

    /// Name of the course in the interface language (most names are brands and
    /// stay the same; "Machine code" and "Assembly" are translated).
    pub fn titel(self, l: Sprache) -> &'static str {
        match self.id() {
            "assembly" => t!(l, COURSE_ASSEMBLY),
            "machine-code" => t!(l, COURSE_MACHINE_CODE),
            _ => self.name(),
        }
    }

    pub const fn ist_rust(self) -> bool {
        self.0 == Self::RUST.0
    }

    /// Inline style of the course tile (accent colour).
    pub fn kachel_stil(self) -> String {
        format!("background:{};color:{}", self.info().color, self.info().on_color)
    }

    /// The course used last in this browser.
    pub fn zuletzt() -> Option<Self> {
        let s = web_sys::window()?.local_storage().ok().flatten()?.get_item(SPEICHER).ok().flatten()?;
        s.parse().ok()
    }

    pub fn merken(self) {
        if let Some(s) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
            let _ = s.set_item(SPEICHER, self.id());
        }
    }
}

/// Name of a course group (`nrzlearn_model::GROUPS`).
pub fn gruppe(l: Sprache, g: &str) -> &'static str {
    match g {
        "popular" => t!(l, GROUP_POPULAR),
        "web" => t!(l, GROUP_WEB),
        "systems" => t!(l, GROUP_SYSTEMS),
        "functional" => t!(l, GROUP_FUNCTIONAL),
        "lisp" => t!(l, GROUP_LISP),
        "data" => t!(l, GROUP_DATA),
        "classics" => t!(l, GROUP_CLASSICS),
        _ => t!(l, GROUP_LOWLEVEL),
    }
}

impl FromStr for Kurs {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::ALLE.into_iter().find(|k| k.id() == s).ok_or_else(|| format!("unknown course {s:?}"))
    }
}

impl Display for Kurs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.id())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn courses_match_the_model() {
        assert_eq!(Kurs::ALLE.len(), COURSES.len());
        for (i, k) in Kurs::ALLE.iter().enumerate() {
            assert_eq!(k.info().id, COURSES[i].id);
            assert_eq!(k.id().parse::<Kurs>(), Ok(*k));
        }
        assert!(Kurs::RUST.ist_rust());
        assert_eq!(Kurs::PYTHON.id(), "python");
    }
}
