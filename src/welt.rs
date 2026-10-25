//! The learning path: every lesson is a world with several levels (nodes).
//! Levels unlock one after another – 1 → 2 → 3 …

use std::sync::OnceLock;

use crate::aufgaben;
use crate::content::{ANZAHL_WELTEN, Aufgabe, lektionen};
use crate::i18n::Sprache;
use crate::rng::Rng;
use crate::uebung::Uebung;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Art {
    /// Theorie lesen, Grafiken ausprobieren.
    Theorie,
    /// Generierte Übungen, Schwierigkeit 1 bis 3.
    Uebung(u8),
    /// Echten Code schreiben und Terminal-Aufgaben.
    Labor,
    /// Abschlussprüfung einer Welt (mit Spickzettel, 3 Leben).
    Pruefung,
    /// Meisterprüfung über vier Welten (ohne Spickzettel, 3 Leben).
    Meister,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Knoten {
    pub id: String,
    /// Fortlaufende Level-Nummer ab 1.
    pub nr: usize,
    pub welt: usize,
    pub art: Art,
}

impl Knoten {
    pub const fn titel(&self, l: Sprache) -> &'static str {
        match self.art {
            Art::Theorie => tr!(l, "Learn", "Lernen", "Aprender"),
            Art::Uebung(3) => tr!(l, "Training", "Training", "Entrenamiento"),
            Art::Uebung(_) => tr!(l, "Practice", "Übung", "Práctica"),
            Art::Labor => tr!(l, "Code lab", "Code-Labor", "Laboratorio de código"),
            Art::Pruefung => tr!(l, "World exam", "Welt-Prüfung", "Examen del mundo"),
            Art::Meister => tr!(l, "Master exam", "Meisterprüfung", "Examen maestro"),
        }
    }

    pub fn beschreibung(&self, l: Sprache) -> String {
        let welt = lektionen(l)[self.welt].titel;
        match self.art {
            Art::Theorie => tr!(
                l,
                format!("Theory, examples and interactive graphics on “{welt}”. Unlocks a cheat sheet card."),
                format!("Theorie, Beispiele und interaktive Grafiken zu „{welt}“. Schaltet eine Spickzettel-Karte frei."),
                format!("Teoría, ejemplos y gráficos interactivos sobre «{welt}». Desbloquea una tarjeta de chuleta.")
            ),
            Art::Uebung(1) => tr!(l, "8 new exercises – to warm up.", "8 neue Aufgaben – zum Warmwerden.", "8 ejercicios nuevos para calentar.").into(),
            Art::Uebung(2) => tr!(l, "9 exercises, a bit trickier, with review.", "9 Aufgaben, etwas kniffliger, mit Wiederholung.", "9 ejercicios, algo más difíciles, con repaso.").into(),
            Art::Uebung(_) => tr!(l, "10 hard exercises with bigger numbers.", "10 schwere Aufgaben mit größeren Zahlen.", "10 ejercicios difíciles con números más grandes.").into(),
            Art::Labor => tr!(l, "Write real code and practise in the terminal.", "Echten Code schreiben und im Terminal üben.", "Escribe código real y practica en el terminal.").into(),
            Art::Pruefung => tr!(l, "10 exercises, 3 lives. Cheat sheet allowed.", "10 Aufgaben, 3 Leben. Spickzettel erlaubt.", "10 ejercicios, 3 vidas. Chuleta permitida.").into(),
            Art::Meister => {
                let (von, bis) = (self.welt - 2, self.welt + 1);
                tr!(
                    l,
                    format!("12 exercises from worlds {von} to {bis}, 3 lives. **No cheat sheet!**"),
                    format!("12 Aufgaben aus den Welten {von} bis {bis}, 3 Leben. **Kein Spickzettel!**"),
                    format!("12 ejercicios de los mundos {von} a {bis}, 3 vidas. **¡Sin chuleta!**")
                )
            }
        }
    }

    pub const fn spickzettel_erlaubt(&self) -> bool {
        !matches!(self.art, Art::Meister)
    }

    pub const fn leben(&self) -> Option<u8> {
        match self.art {
            Art::Pruefung | Art::Meister => Some(3),
            _ => None,
        }
    }

    pub const fn ist_pruefung(&self) -> bool {
        matches!(self.art, Art::Pruefung | Art::Meister)
    }

    /// Welten, aus denen eine Meisterprüfung Aufgaben zieht.
    fn welten(&self) -> std::ops::RangeInclusive<usize> {
        match self.art {
            Art::Meister => self.welt.saturating_sub(3)..=self.welt,
            _ => self.welt..=self.welt,
        }
    }
}

fn hat_labor(welt: usize) -> bool {
    lektionen(Sprache::En)[welt].aufgaben.iter().any(|a| matches!(a, Aufgabe::Code { .. } | Aufgabe::Terminal { .. }))
}

pub fn alle() -> &'static [Knoten] {
    static KNOTEN: OnceLock<Vec<Knoten>> = OnceLock::new();
    KNOTEN.get_or_init(|| {
        let mut v: Vec<Knoten> = Vec::new();
        let neu = |welt: usize, id: String, art: Art, v: &mut Vec<Knoten>| {
            let nr = v.len() + 1;
            v.push(Knoten { id, nr, welt, art });
        };
        for w in 0..ANZAHL_WELTEN {
            neu(w, format!("w{w}-theorie"), Art::Theorie, &mut v);
            neu(w, format!("w{w}-u1"), Art::Uebung(1), &mut v);
            neu(w, format!("w{w}-u2"), Art::Uebung(2), &mut v);
            neu(w, format!("w{w}-u3"), Art::Uebung(3), &mut v);
            if hat_labor(w) {
                neu(w, format!("w{w}-labor"), Art::Labor, &mut v);
            }
            neu(w, format!("w{w}-pruefung"), Art::Pruefung, &mut v);
            if (w + 1) % 4 == 0 {
                neu(w, format!("m{}", (w + 1) / 4), Art::Meister, &mut v);
            }
        }
        v
    })
}

pub fn finden(id: &str) -> Option<(usize, &'static Knoten)> {
    alle().iter().enumerate().find(|(_, k)| k.id == id)
}

/// Creates a new random round for a level.
pub fn sitzung(k: &Knoten, r: &mut Rng, l: Sprache) -> Vec<Uebung> {
    let mut v: Vec<Uebung> = Vec::new();
    match k.art {
        Art::Theorie => {}
        Art::Labor => {
            v = lektionen(l)[k.welt]
                .aufgaben
                .iter()
                .filter(|a| matches!(a, Aufgabe::Code { .. } | Aufgabe::Terminal { .. }))
                .map(aufgaben::fest)
                .collect();
        }
        Art::Uebung(s) => {
            let anzahl = 7 + s as usize;
            // Feste Aufgaben (ohne Code) aus der Lektion einstreuen.
            let mut feste: Vec<Uebung> = aufgaben::feste(k.welt, l)
                .into_iter()
                .filter(|u| !matches!(u, Uebung::Code { .. } | Uebung::Terminal { .. }))
                .map(|u| aufgaben::optionen_mischen(u, r))
                .collect();
            r.mischen(&mut feste);
            v.extend(feste.into_iter().take(match s {
                1 => 2,
                2 => 1,
                _ => 0,
            }));
            // Wiederholung aus einer früheren Welt.
            if s >= 2 && k.welt > 0 {
                let alt = r.idx(k.welt);
                v.push(aufgaben::zufaellig(r, alt, s - 1, l));
            }
            ohne_wiederholung(r, &mut v, k.welt..=k.welt, s, anzahl, l);
            r.mischen(&mut v);
        }
        Art::Pruefung => {
            ohne_wiederholung(r, &mut v, k.welten(), 3, 10, l);
            r.mischen(&mut v);
        }
        Art::Meister => {
            ohne_wiederholung(r, &mut v, k.welten(), 2, 6, l);
            ohne_wiederholung(r, &mut v, k.welten(), 3, 12, l);
            r.mischen(&mut v);
        }
    }
    v
}

/// Füllt bis `anzahl` mit Übungen auf. Jeder Generator kommt erst wieder dran,
/// wenn alle anderen einmal dran waren – so bleibt es abwechslungsreich.
fn ohne_wiederholung(r: &mut Rng, v: &mut Vec<Uebung>, welten: std::ops::RangeInclusive<usize>, stufe: u8, anzahl: usize, l: Sprache) {
    let mut gens: Vec<aufgaben::Gen> = welten.flat_map(|w| aufgaben::generatoren(w).iter().copied()).collect();
    if gens.is_empty() {
        // Abschlussprojekt: aus allen Welten.
        gens = (0..15).flat_map(|w| aufgaben::generatoren(w).iter().copied()).collect();
    }
    let mut stapel: Vec<aufgaben::Gen> = Vec::new();
    while v.len() < anzahl {
        if stapel.is_empty() {
            stapel = gens.clone();
            r.mischen(&mut stapel);
        }
        if let Some(g) = stapel.pop() {
            // Code-Terminal-Aufgaben in Prüfungen weglassen (zu langsam für 3 Leben).
            let u = g(r, stufe, l);
            if stufe == 3 && matches!(u, Uebung::Terminal { .. }) && v.len() + 1 < anzahl {
                continue;
            }
            v.push(u);
        }
    }
}
