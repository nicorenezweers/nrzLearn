//! Course content for the pages, the same for every course: the built-in
//! Rust course or a JSON course loaded from `/content/<course>/<lang>.json`
//! (built by `nrzlearn-site build` from `courses/`).
//!
//! JSON courses get these levels per unit: Learn, Practice 1, Practice 2,
//! World exam – and after every fourth unit a master exam over those units.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use dioxus::prelude::*;
use nrzlearn_model::{Course, Exercise, Unit};

use crate::content::lektionen;
use crate::i18n::Sprache;
use crate::kurs::Kurs;
use crate::progress::Fortschritt;
use crate::rng::Rng;
use crate::uebung::Uebung;
use crate::welt::{self, Art, Knoten};

/// Exercises per round in JSON courses.
const RUNDE_1: usize = 7;
const RUNDE_2: usize = 8;
const PRUEFUNG: usize = 10;
const MEISTER: usize = 12;

#[derive(Clone)]
enum Quelle {
    Rust,
    Daten(Rc<Course>),
}

/// A cheat sheet card of a world.
#[derive(Clone, PartialEq)]
pub struct Karte {
    pub welt: usize,
    pub titel: String,
    /// Rust cards: a block of code with comments.
    pub code: Option<String>,
    /// JSON courses: short points with `code`.
    pub punkte: Vec<String>,
}

#[derive(Clone)]
pub struct KursInhalt {
    pub kurs: Kurs,
    /// Language of the course texts (English when there is no translation).
    pub sprache: Sprache,
    quelle: Quelle,
    knoten: Rc<Vec<Knoten>>,
}

impl PartialEq for KursInhalt {
    fn eq(&self, other: &Self) -> bool {
        self.kurs == other.kurs && self.sprache == other.sprache && Rc::ptr_eq(&self.knoten, &other.knoten)
    }
}

thread_local! {
    static RUST_KNOTEN: Rc<Vec<Knoten>> = Rc::new(welt::alle().to_vec());
    static CACHE: RefCell<HashMap<(Kurs, Sprache), KursInhalt>> = RefCell::new(HashMap::new());
}

fn level_id(kurs: Kurs, rest: &str) -> String {
    format!("{}:{rest}", kurs.id())
}

/// The levels of a JSON course.
fn knoten_bauen(kurs: Kurs, c: &Course) -> Vec<Knoten> {
    let mut v: Vec<Knoten> = Vec::new();
    let mut neu = |welt: usize, id: String, art: Art, anzahl: usize| {
        let nr = v.len() + 1;
        v.push(Knoten { id, nr, welt, art, anzahl });
    };
    for (w, u) in c.units.iter().enumerate() {
        let pool = u.exercises.len();
        neu(w, level_id(kurs, &format!("{}-lesson", u.id)), Art::Theorie, 0);
        neu(w, level_id(kurs, &format!("{}-p1", u.id)), Art::Uebung(1), RUNDE_1.min(pool));
        neu(w, level_id(kurs, &format!("{}-p2", u.id)), Art::Uebung(2), RUNDE_2.min(pool));
        neu(w, level_id(kurs, &format!("{}-exam", u.id)), Art::Pruefung, PRUEFUNG.min(pool));
        if (w + 1) % 4 == 0 {
            neu(w, level_id(kurs, &format!("master-{}", (w + 1) / 4)), Art::Meister, MEISTER);
        }
    }
    v
}

/// Turns an exercise of a JSON course into the runtime model; options are shuffled.
pub fn uebung(e: &Exercise, r: &mut Rng) -> Uebung {
    let mischen = |optionen: &[String], richtig: usize, r: &mut Rng| -> (Vec<String>, usize) {
        let mut idx: Vec<usize> = (0..optionen.len()).collect();
        r.mischen(&mut idx);
        let neu = idx.iter().map(|&i| optionen[i].clone()).collect();
        let pos = idx.iter().position(|&i| i == richtig).unwrap_or(0);
        (neu, pos)
    };
    match e {
        Exercise::Choice { prompt, code, options, answer, explain, .. } => {
            let (optionen, richtig) = mischen(options, *answer, r);
            Uebung::Auswahl { frage: prompt.clone(), code: code.clone(), optionen, richtig, erklaerung: explain.clone() }
        }
        Exercise::Input { prompt, code, answers, explain, .. } => {
            Uebung::Eingabe { frage: prompt.clone(), code: code.clone(), antworten: answers.clone(), erklaerung: explain.clone() }
        }
        Exercise::TrueFalse { statement, code, answer, explain, .. } => {
            Uebung::WahrFalsch { frage: statement.clone(), code: code.clone(), wahr: *answer, erklaerung: explain.clone() }
        }
        Exercise::Gap { prompt, code, options, answer, explain, .. } => {
            let (optionen, richtig) = mischen(options, *answer, r);
            Uebung::Luecke { frage: prompt.clone(), code: code.clone(), optionen, richtig, erklaerung: explain.clone() }
        }
        Exercise::Blocks { prompt, tokens, distractors, explain, .. } => {
            Uebung::Bausteine { frage: prompt.clone(), teile: tokens.clone(), extra: distractors.clone(), erklaerung: explain.clone() }
        }
        Exercise::Order { prompt, lines, explain, .. } => {
            Uebung::Reihenfolge { frage: prompt.clone(), zeilen: lines.clone(), erklaerung: explain.clone() }
        }
        Exercise::Pairs { prompt, pairs, .. } => {
            Uebung::Paare { frage: prompt.clone(), paare: pairs.iter().map(|[a, b]| (a.clone(), b.clone())).collect() }
        }
        Exercise::Bug { prompt, lines, bug, explain, .. } => {
            Uebung::FehlerZeile { frage: prompt.clone(), zeilen: lines.clone(), falsch: *bug, erklaerung: explain.clone() }
        }
    }
}

/// `n` random exercises from the units `welten`, without repeating one.
fn ziehen<'a>(c: &'a Course, welten: std::ops::RangeInclusive<usize>, n: usize, r: &mut Rng) -> Vec<&'a Exercise> {
    let mut pool: Vec<&Exercise> = welten.filter_map(|w| c.units.get(w)).flat_map(|u| u.exercises.iter()).collect();
    r.mischen(&mut pool);
    pool.truncate(n);
    pool
}

impl KursInhalt {
    pub fn rust(l: Sprache) -> Self {
        Self { kurs: Kurs::RUST, sprache: l.rust(), quelle: Quelle::Rust, knoten: RUST_KNOTEN.with(Rc::clone) }
    }

    pub fn daten(kurs: Kurs, sprache: Sprache, c: Course) -> Self {
        let knoten = Rc::new(knoten_bauen(kurs, &c));
        Self { kurs, sprache, quelle: Quelle::Daten(Rc::new(c)), knoten }
    }

    pub fn knoten(&self) -> &[Knoten] {
        &self.knoten
    }

    pub fn finden(&self, id: &str) -> Option<(usize, &Knoten)> {
        self.knoten.iter().enumerate().find(|(_, k)| k.id == id)
    }

    pub fn anzahl_welten(&self) -> usize {
        match &self.quelle {
            Quelle::Rust => crate::content::ANZAHL_WELTEN,
            Quelle::Daten(c) => c.units.len(),
        }
    }

    /// The unit of a JSON course (`None` for Rust).
    pub fn einheit(&self, welt: usize) -> Option<&Unit> {
        match &self.quelle {
            Quelle::Rust => None,
            Quelle::Daten(c) => c.units.get(welt),
        }
    }

    pub fn welt_titel(&self, welt: usize) -> String {
        match &self.quelle {
            Quelle::Rust => lektionen(self.sprache).get(welt).map(|l| l.titel.to_string()).unwrap_or_default(),
            Quelle::Daten(c) => c.units.get(welt).map(|u| u.title.clone()).unwrap_or_default(),
        }
    }

    pub fn welt_kapitel(&self, welt: usize) -> String {
        match &self.quelle {
            Quelle::Rust => lektionen(self.sprache).get(welt).map(|l| l.kapitel.to_string()).unwrap_or_default(),
            Quelle::Daten(c) => c.units.get(welt).map(|u| u.chapter.clone()).unwrap_or_default(),
        }
    }

    pub fn welt_kurz(&self, welt: usize) -> String {
        match &self.quelle {
            Quelle::Rust => lektionen(self.sprache).get(welt).map(|l| l.kurz.to_string()).unwrap_or_default(),
            Quelle::Daten(c) => c.units.get(welt).map(|u| u.summary.clone()).unwrap_or_default(),
        }
    }

    /// ID of the theory level of a world (it unlocks the cheat sheet card).
    pub fn theorie_id(&self, welt: usize) -> String {
        match &self.quelle {
            Quelle::Rust => format!("w{welt}-theorie"),
            Quelle::Daten(c) => c.units.get(welt).map(|u| level_id(self.kurs, &format!("{}-lesson", u.id))).unwrap_or_default(),
        }
    }

    pub fn karte_offen(&self, f: &Fortschritt, welt: usize) -> bool {
        f.fertig(&self.theorie_id(welt))
    }

    /// All cheat sheet cards of the course.
    pub fn karten(&self) -> Vec<Karte> {
        match &self.quelle {
            Quelle::Rust => crate::content::spick::KARTEN
                .iter()
                .map(|k| Karte { welt: k.welt, titel: k.titel(self.sprache).to_string(), code: Some(k.code(self.sprache).to_string()), punkte: Vec::new() })
                .collect(),
            Quelle::Daten(c) => c
                .units
                .iter()
                .enumerate()
                .map(|(w, u)| Karte { welt: w, titel: u.title.clone(), code: None, punkte: u.card.clone() })
                .collect(),
        }
    }

    /// Levels completed in this course.
    pub fn fertig(&self, f: &Fortschritt) -> usize {
        self.knoten.iter().filter(|k| f.fertig(&k.id)).count()
    }

    /// A new random round for a level.
    pub fn sitzung(&self, k: &Knoten, r: &mut Rng, l: Sprache) -> Vec<Uebung> {
        match &self.quelle {
            Quelle::Rust => welt::sitzung(k, r, l),
            Quelle::Daten(c) => {
                let gezogen: Vec<&Exercise> = match k.art {
                    Art::Theorie | Art::Labor => Vec::new(),
                    Art::Uebung(1) => ziehen(c, k.welt..=k.welt, k.anzahl, r),
                    Art::Uebung(_) => {
                        // Mostly this unit, plus up to two from earlier units as review.
                        let wiederholung = if k.welt > 0 { 2 } else { 0 };
                        let mut v = ziehen(c, k.welt..=k.welt, k.anzahl - wiederholung, r);
                        if k.welt > 0 {
                            v.extend(ziehen(c, 0..=k.welt - 1, wiederholung, r));
                        }
                        v
                    }
                    Art::Pruefung => ziehen(c, k.welt..=k.welt, k.anzahl, r),
                    Art::Meister => ziehen(c, k.welten(), k.anzahl, r),
                };
                let mut v: Vec<Uebung> = gezogen.into_iter().map(|e| uebung(e, r)).collect();
                r.mischen(&mut v);
                v
            }
        }
    }
}

/// Loads the content of a course in a language (JSON courses: from the server,
/// English if the translation is missing). Results are cached.
pub async fn laden(kurs: Kurs, l: Sprache) -> Result<KursInhalt, String> {
    if kurs.ist_rust() {
        return Ok(KursInhalt::rust(l));
    }
    if let Some(i) = CACHE.with(|c| c.borrow().get(&(kurs, l)).cloned()) {
        return Ok(i);
    }
    let holen = |sprache: Sprache| async move {
        let url = format!("/content/{}/{}.json", kurs.id(), sprache.code());
        let res = gloo_net::http::Request::get(&url).send().await.map_err(|_| "network".to_string())?;
        if !res.ok() {
            return Err(format!("http_{}", res.status()));
        }
        res.json::<Course>().await.map_err(|e| e.to_string())
    };
    let (sprache, course) = match holen(l).await {
        Ok(c) => (l, c),
        // No translation (404) or a broken file: show the English course.
        Err(_) if l != Sprache::En => (Sprache::En, holen(Sprache::En).await?),
        Err(e) => return Err(e),
    };
    let inhalt = KursInhalt::daten(kurs, sprache, course);
    CACHE.with(|c| c.borrow_mut().insert((kurs, l), inhalt.clone()));
    Ok(inhalt)
}

/// The loaded content of the current course (provided by the course frame).
pub fn use_inhalt() -> KursInhalt {
    use_context::<KursInhalt>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use nrzlearn_model::{Block, Unit};

    fn kurs(units: usize) -> Course {
        let unit = |i: usize| Unit {
            id: format!("u{i}"),
            title: format!("Unit {i}"),
            chapter: "Basics".into(),
            summary: "About it.".into(),
            lesson: vec![Block::Text { md: "Hi".into() }],
            card: vec!["`x`".into()],
            exercises: (0..12)
                .map(|n| Exercise::Choice { id: format!("u{i}-{n}"), prompt: format!("Q{n}"), code: None, options: vec!["a".into(), "b".into(), "c".into()], answer: 1, explain: "b".into() })
                .collect(),
        };
        Course { course: "python".into(), lang: "en".into(), units: (0..units).map(unit).collect() }
    }

    #[test]
    fn rust_course_size_matches_the_site() {
        // site/src/build.rs: RUST_WORLDS and RUST_LEVELS
        assert_eq!(crate::content::ANZAHL_WELTEN, 16);
        assert_eq!(welt::alle().len(), 100);
    }

    #[test]
    fn levels_and_rounds_of_json_courses() {
        let i = KursInhalt::daten(Kurs::PYTHON, Sprache::De, kurs(8));
        // 8 units × 4 levels + 2 master exams
        assert_eq!(i.knoten().len(), 34);
        assert_eq!(i.knoten()[0].id, "python:u0-lesson");
        assert_eq!(i.knoten()[16].id, "python:master-1");
        assert_eq!(i.knoten()[16].welten(), 0..=3);
        let mut r = Rng::mit_seed(7);
        for k in i.knoten() {
            let runde = i.sitzung(k, &mut r, Sprache::De);
            assert_eq!(runde.len(), k.anzahl, "{}", k.id);
            for u in runde {
                if let Uebung::Auswahl { optionen, richtig, .. } = u {
                    assert_eq!(optionen[richtig], "b", "the shuffled answer still points to the right option");
                }
            }
        }
    }
}
