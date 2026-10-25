//! Exercise generators: every function creates a new exercise with random
//! values on each call, in the requested language. The correct answer is
//! computed with real Rust (e.g. `wrapping_add`, `parse`, `len`), so it is
//! guaranteed to be right.

mod basis;
mod besitz;
mod fluss;
mod profi;
mod typen;

use crate::content::{ANZAHL_WELTEN, Aufgabe, lektionen};
pub use crate::i18n::Sprache;
use crate::rng::Rng;
use crate::uebung::Uebung;

/// A generator gets the randomness, the difficulty (1 to 3) and the language.
pub type Gen = fn(&mut Rng, u8, Sprache) -> Uebung;

/// All generators of a world (index as in `lektionen`).
pub fn generatoren(welt: usize) -> &'static [Gen] {
    match welt {
        0 => basis::W_HALLO,
        1 => basis::W_VARIABLEN,
        2 => basis::W_ZAHLEN,
        3 => basis::W_FUNKTIONEN,
        4 => fluss::W_IF,
        5 => fluss::W_SCHLEIFEN,
        6 => besitz::W_OWNERSHIP,
        7 => besitz::W_BORROWING,
        8 => besitz::W_STRINGS,
        9 => typen::W_STRUCTS,
        10 => typen::W_ENUMS,
        11 => typen::W_COLLECTIONS,
        12 => profi::W_FEHLER,
        13 => profi::W_TRAITS,
        14 => profi::W_CLOSURES,
        _ => &[],
    }
}

/// Number of different generators.
pub fn anzahl_generatoren() -> usize {
    (0..ANZAHL_WELTEN).map(|w| generatoren(w).len()).sum()
}

/// A random exercise from a world. The final project mixes all worlds.
pub fn zufaellig(r: &mut Rng, welt: usize, stufe: u8, l: Sprache) -> Uebung {
    let mut w = welt;
    while generatoren(w).is_empty() {
        w = r.idx(15);
    }
    let g = r.wahl(generatoren(w));
    g(r, stufe, l)
}

/// Fixed tasks of a lesson as exercises.
pub fn feste(welt: usize, l: Sprache) -> Vec<Uebung> {
    lektionen(l)[welt].aufgaben.iter().map(fest).collect()
}

/// Shuffles the options of a multiple-choice exercise.
pub fn optionen_mischen(u: Uebung, r: &mut Rng) -> Uebung {
    match u {
        Uebung::Auswahl { frage, code, optionen, richtig, erklaerung } => {
            let loesung = optionen[richtig].clone();
            let mut o = optionen;
            r.mischen(&mut o);
            let richtig = o.iter().position(|x| *x == loesung).unwrap_or(0);
            Uebung::Auswahl { frage, code, optionen: o, richtig, erklaerung }
        }
        andere => andere,
    }
}

pub fn fest(a: &Aufgabe) -> Uebung {
    match a {
        Aufgabe::Quiz { frage, code, optionen, richtig, erklaerung, .. } => Uebung::Auswahl {
            frage: (*frage).into(),
            code: code.map(Into::into),
            optionen: optionen.iter().map(|o| (*o).into()).collect(),
            richtig: *richtig,
            erklaerung: (*erklaerung).into(),
        },
        Aufgabe::Rechnung { frage, code, antworten, hinweis: _, erklaerung, .. } => Uebung::Eingabe {
            frage: (*frage).into(),
            code: code.map(Into::into),
            antworten: antworten.iter().map(|o| (*o).into()).collect(),
            erklaerung: (*erklaerung).into(),
        },
        Aufgabe::Code { aufgabe, vorlage, erwartet, hinweis, loesung, .. } => Uebung::Code {
            aufgabe: (*aufgabe).into(),
            vorlage: (*vorlage).into(),
            erwartet: (*erwartet).into(),
            hinweis: (*hinweis).into(),
            loesung: (*loesung).into(),
        },
        Aufgabe::Terminal { aufgabe, ziele, hinweis, .. } => Uebung::Terminal {
            aufgabe: (*aufgabe).into(),
            ziele: ziele.iter().map(|z| (*z).into()).collect(),
            hinweis: (*hinweis).into(),
        },
    }
}

// ───────────────────────────── Helpers ─────────────────────────────

pub const NAMEN: &[&str] = &["Ferris", "Ada", "Linus", "Grace", "Alan", "Mia", "Noah", "Lena", "Sara", "Leo"];

pub fn projekte(l: Sprache) -> &'static [&'static str] {
    tr!(
        l,
        &["hello", "calculator", "game", "notes", "weather", "quiz", "counter", "crab"],
        &["hallo", "rechner", "spiel", "notizen", "wetter", "quiz", "zaehler", "krabbe"],
        &["hola", "calculadora", "juego", "notas", "clima", "quiz", "contador", "cangrejo"]
    )
}

pub fn vars(l: Sprache) -> &'static [&'static str] {
    tr!(
        l,
        &["x", "number", "points", "value", "n", "total", "age", "level"],
        &["x", "zahl", "punkte", "wert", "n", "summe", "alter", "level"],
        &["x", "numero", "puntos", "valor", "n", "suma", "edad", "nivel"]
    )
}

pub fn vs(a: &[&str]) -> Vec<String> {
    a.iter().map(|s| (*s).to_string()).collect()
}

pub fn c(s: impl Into<String>) -> Option<String> {
    Some(s.into())
}

/// Multiple choice with shuffled options. Duplicates and wrong answers equal to
/// the solution are removed; at most three wrong answers remain.
pub fn auswahl(
    r: &mut Rng,
    frage: impl Into<String>,
    code: Option<String>,
    richtig: impl Into<String>,
    falsche: Vec<String>,
    erklaerung: impl Into<String>,
) -> Uebung {
    let richtig = richtig.into();
    let mut f: Vec<String> = Vec::new();
    for x in falsche {
        if x != richtig && !f.contains(&x) {
            f.push(x);
        }
    }
    r.mischen(&mut f);
    f.truncate(3);
    let mut optionen = f;
    optionen.push(richtig.clone());
    r.mischen(&mut optionen);
    let idx = optionen.iter().position(|o| *o == richtig).unwrap_or(0);
    Uebung::Auswahl { frage: frage.into(), code, optionen, richtig: idx, erklaerung: erklaerung.into() }
}

pub fn eingabe(frage: impl Into<String>, code: Option<String>, antwort: impl Into<String>, erklaerung: impl Into<String>) -> Uebung {
    Uebung::Eingabe { frage: frage.into(), code, antworten: vec![antwort.into()], erklaerung: erklaerung.into() }
}

pub fn wf(frage: impl Into<String>, code: Option<String>, wahr: bool, erklaerung: impl Into<String>) -> Uebung {
    Uebung::WahrFalsch { frage: frage.into(), code, wahr, erklaerung: erklaerung.into() }
}

pub fn luecke(
    r: &mut Rng,
    frage: impl Into<String>,
    code: impl Into<String>,
    richtig: &str,
    falsche: &[&str],
    erklaerung: impl Into<String>,
) -> Uebung {
    let mut optionen = vs(falsche);
    optionen.truncate(3);
    optionen.push(richtig.into());
    r.mischen(&mut optionen);
    let idx = optionen.iter().position(|o| o == richtig).unwrap_or(0);
    Uebung::Luecke { frage: frage.into(), code: code.into(), optionen, richtig: idx, erklaerung: erklaerung.into() }
}

pub fn bausteine(frage: impl Into<String>, teile: Vec<String>, extra: &[&str], erklaerung: impl Into<String>) -> Uebung {
    Uebung::Bausteine { frage: frage.into(), teile, extra: vs(extra), erklaerung: erklaerung.into() }
}

pub fn reihenfolge(frage: impl Into<String>, zeilen: Vec<String>, erklaerung: impl Into<String>) -> Uebung {
    Uebung::Reihenfolge { frage: frage.into(), zeilen, erklaerung: erklaerung.into() }
}

pub fn fehler(frage: impl Into<String>, zeilen: Vec<String>, falsch: usize, erklaerung: impl Into<String>) -> Uebung {
    Uebung::FehlerZeile { frage: frage.into(), zeilen, falsch, erklaerung: erklaerung.into() }
}

/// Picks `n` random pairs from the pool.
pub fn paare(r: &mut Rng, frage: impl Into<String>, pool: &[(&str, &str)], n: usize) -> Uebung {
    let mut v: Vec<(String, String)> = pool.iter().map(|(a, b)| ((*a).into(), (*b).into())).collect();
    r.mischen(&mut v);
    v.truncate(n);
    Uebung::Paare { frage: frage.into(), paare: v }
}

/// True/false statement from a pool: (statement, true?, explanation).
pub fn wf_pool(r: &mut Rng, pool: &[(&str, bool, &str)]) -> Uebung {
    let (a, w, e) = *r.wahl(pool);
    wf(a, None, w, e)
}

/// Multiple-choice question from a pool: (question, correct, wrong, explanation).
pub type FragePool = &'static [(&'static str, &'static str, &'static [&'static str], &'static str)];

pub fn frage_pool(r: &mut Rng, pool: FragePool) -> Uebung {
    let (f, richtig, falsche, e) = *r.wahl(pool);
    auswahl(r, f, None, richtig, vs(falsche), e)
}

/// Number range per difficulty.
pub fn nach_stufe(s: u8, leicht: (i64, i64), mittel: (i64, i64), schwer: (i64, i64)) -> (i64, i64) {
    match s {
        1 => leicht,
        2 => mittel,
        _ => schwer,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::uebung::Antwort;

    /// Every generator must produce consistent exercises in every language.
    #[test]
    fn alle_generatoren_sind_konsistent() {
        let mut r = Rng::mit_seed(42);
        for l in Sprache::ALLE {
        for welt in 0..ANZAHL_WELTEN {
            for (gi, g) in generatoren(welt).iter().enumerate() {
                for stufe in 1..=3 {
                    for _ in 0..40 {
                        let u = g(&mut r, stufe, l);
                        let ort = format!("{l:?} world {welt}, generator {gi}, level {stufe}: {u:?}");
                        match &u {
                            Uebung::Auswahl { optionen, richtig, .. } | Uebung::Luecke { optionen, richtig, .. } => {
                                assert!(*richtig < optionen.len(), "{ort}");
                                assert!(optionen.len() >= 2, "{ort}");
                                let mut o = optionen.clone();
                                o.sort();
                                o.dedup();
                                assert_eq!(o.len(), optionen.len(), "doppelte Optionen: {ort}");
                            }
                            Uebung::Eingabe { antworten, .. } => assert!(!antworten[0].is_empty(), "{ort}"),
                            Uebung::FehlerZeile { zeilen, falsch, .. } => assert!(*falsch < zeilen.len(), "{ort}"),
                            Uebung::Paare { paare, .. } => {
                                assert!(paare.len() >= 3, "{ort}");
                                let mut l: Vec<_> = paare.iter().map(|p| &p.0).collect();
                                l.dedup();
                                assert_eq!(l.len(), paare.len(), "{ort}");
                            }
                            Uebung::Bausteine { teile, .. } | Uebung::Reihenfolge { zeilen: teile, .. } => {
                                assert!(teile.len() >= 3, "{ort}");
                                let gemischt = u.gemischt(&mut r);
                                let mut liste = Vec::new();
                                let mut benutzt = vec![false; gemischt.len()];
                                for t in teile {
                                    let i = (0..gemischt.len()).find(|&i| !benutzt[i] && gemischt[i] == *t).expect(&ort);
                                    benutzt[i] = true;
                                    liste.push(i);
                                }
                                assert!(u.pruefen(&Antwort::Liste(liste), &gemischt), "{ort}");
                            }
                            Uebung::Terminal { ziele, .. } => assert!(!ziele.is_empty(), "{ort}"),
                            Uebung::WahrFalsch { .. } | Uebung::Code { .. } => {}
                        }
                    }
                }
            }
        }
        }
    }

    /// The same seed must create the same exercise structure in all languages
    /// (same answer index, same number of options) – translations only change text.
    #[test]
    fn sprachen_haben_gleiche_struktur() {
        for welt in 0..ANZAHL_WELTEN {
            for (gi, g) in generatoren(welt).iter().enumerate() {
                for seed in 1..30_u64 {
                    let u: Vec<Uebung> = Sprache::ALLE.iter().map(|&l| g(&mut Rng::mit_seed(seed), 2, l)).collect();
                    let form = |u: &Uebung| match u {
                        Uebung::Auswahl { optionen, richtig, .. } | Uebung::Luecke { optionen, richtig, .. } => (optionen.len(), *richtig),
                        Uebung::Eingabe { antworten, .. } => (antworten.len(), 0),
                        Uebung::WahrFalsch { wahr, .. } => (2, usize::from(*wahr)),
                        Uebung::Bausteine { teile, .. } | Uebung::Reihenfolge { zeilen: teile, .. } => (teile.len(), 0),
                        Uebung::Paare { paare, .. } => (paare.len(), 0),
                        Uebung::FehlerZeile { zeilen, falsch, .. } => (zeilen.len(), *falsch),
                        Uebung::Terminal { ziele, .. } => (ziele.len(), 0),
                        Uebung::Code { .. } => (0, 0),
                    };
                    assert_eq!(form(&u[0]), form(&u[1]), "world {welt} generator {gi} seed {seed}: {:?} vs {:?}", u[0], u[1]);
                    assert_eq!(form(&u[0]), form(&u[2]), "world {welt} generator {gi} seed {seed}: {:?} vs {:?}", u[0], u[2]);
                }
            }
        }
    }

    /// Prints one example per generator (cargo test -- --nocapture beispiele).
    #[test]
    fn beispiele() {
        for l in Sprache::ALLE {
            let mut r = Rng::mit_seed(7);
            for welt in 0..ANZAHL_WELTEN {
                for (gi, g) in generatoren(welt).iter().enumerate() {
                    let u = g(&mut r, 2, l);
                    println!("{l:?} W{welt} G{gi}: {u:?}\n   → {}\n", u.loesung_text(l));
                }
            }
        }
    }
}
