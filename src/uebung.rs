//! Runtime model of a single exercise. Exercises are created again and again
//! by the generators (see `aufgaben/`) or taken from the fixed lesson tasks.

use crate::i18n::Sprache;
use crate::rng::Rng;

#[derive(Clone, PartialEq, Debug)]
pub enum Uebung {
    /// Eine richtige Antwort aus mehreren auswählen.
    Auswahl { frage: String, code: Option<String>, optionen: Vec<String>, richtig: usize, erklaerung: String },
    /// Freie Eingabe (Zahl, Wort, Ausgabe …).
    Eingabe { frage: String, code: Option<String>, antworten: Vec<String>, erklaerung: String },
    /// Stimmt die Aussage?
    WahrFalsch { frage: String, code: Option<String>, wahr: bool, erklaerung: String },
    /// Code mit einer Lücke `___`, die passende Option füllt sie.
    Luecke { frage: String, code: String, optionen: Vec<String>, richtig: usize, erklaerung: String },
    /// Eine Codezeile aus Bausteinen zusammensetzen (mit Ablenkern).
    Bausteine { frage: String, teile: Vec<String>, extra: Vec<String>, erklaerung: String },
    /// Zeilen in die richtige Reihenfolge bringen.
    Reihenfolge { frage: String, zeilen: Vec<String>, erklaerung: String },
    /// Begriffe einander zuordnen.
    Paare { frage: String, paare: Vec<(String, String)> },
    /// Die fehlerhafte Zeile antippen.
    FehlerZeile { frage: String, zeilen: Vec<String>, falsch: usize, erklaerung: String },
    /// Echten Code schreiben, der kompiliert und die erwartete Ausgabe liefert.
    Code { aufgabe: String, vorlage: String, erwartet: String, hinweis: String, loesung: String },
    /// Befehle im simulierten Terminal ausführen.
    Terminal { aufgabe: String, ziele: Vec<String>, hinweis: String },
}

impl Uebung {
    pub const fn art(&self, l: Sprache) -> &'static str {
        match self {
            Self::Auswahl { .. } => tr!(l, "Multiple choice", "Auswahl", "Opción múltiple"),
            Self::Eingabe { .. } => tr!(l, "Calculate & type", "Rechnen & Tippen", "Calcula y escribe"),
            Self::WahrFalsch { .. } => tr!(l, "True or false?", "Wahr oder falsch?", "¿Verdadero o falso?"),
            Self::Luecke { .. } => tr!(l, "Fill the gap", "Lückentext", "Rellena el hueco"),
            Self::Bausteine { .. } => tr!(l, "Build the code", "Code bauen", "Construye el código"),
            Self::Reihenfolge { .. } => tr!(l, "Put in order", "Reihenfolge", "Ordena"),
            Self::Paare { .. } => tr!(l, "Match pairs", "Paare finden", "Empareja"),
            Self::FehlerZeile { .. } => tr!(l, "Find the bug", "Fehler finden", "Encuentra el error"),
            Self::Code { .. } => tr!(l, "Write code", "Code schreiben", "Escribe código"),
            Self::Terminal { .. } => tr!(l, "Terminal", "Terminal", "Terminal"),
        }
    }

    pub const fn xp(&self) -> u32 {
        match self {
            Self::Code { .. } => 20,
            Self::Terminal { .. } | Self::Bausteine { .. } | Self::Reihenfolge { .. } => 12,
            _ => 10,
        }
    }

    /// The correct answer as text – for the feedback after a mistake.
    pub fn loesung_text(&self, l: Sprache) -> String {
        match self {
            Self::Auswahl { optionen, richtig, .. } | Self::Luecke { optionen, richtig, .. } => {
                optionen[*richtig].clone()
            }
            Self::Eingabe { antworten, .. } => antworten[0].clone(),
            Self::WahrFalsch { wahr, .. } => if *wahr { tr!(l, "True", "Wahr", "Verdadero") } else { tr!(l, "False", "Falsch", "Falso") }.into(),
            Self::Bausteine { teile, .. } => teile.join(" "),
            Self::Reihenfolge { zeilen, .. } => zeilen.join("\n"),
            Self::FehlerZeile { zeilen, falsch, .. } => format!("{} {}: {}", tr!(l, "Line", "Zeile", "Línea"), falsch + 1, zeilen[*falsch].trim()),
            Self::Paare { .. } | Self::Code { .. } | Self::Terminal { .. } => String::new(),
        }
    }

    pub fn erklaerung(&self) -> &str {
        match self {
            Self::Auswahl { erklaerung, .. }
            | Self::Eingabe { erklaerung, .. }
            | Self::WahrFalsch { erklaerung, .. }
            | Self::Luecke { erklaerung, .. }
            | Self::Bausteine { erklaerung, .. }
            | Self::Reihenfolge { erklaerung, .. }
            | Self::FehlerZeile { erklaerung, .. } => erklaerung,
            Self::Paare { .. } | Self::Code { .. } | Self::Terminal { .. } => "",
        }
    }
}

/// What the learner has selected or typed.
#[derive(Clone, PartialEq, Debug, Default)]
pub enum Antwort {
    #[default]
    Keine,
    Index(usize),
    Text(String),
    Liste(Vec<usize>),
    /// Self-checking exercises (pairs, code, terminal) report their result directly.
    Fertig(bool),
}

pub fn normalisieren(s: &str) -> String {
    s.trim()
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '_' && *c != '\'' && *c != '`')
        .collect::<String>()
        .replace(',', ".")
        .replace('−', "-")
        .trim_end_matches('.')
        .to_string()
}

impl Uebung {
    pub fn bereit(&self, a: &Antwort) -> bool {
        match (self, a) {
            (_, Antwort::Keine) => false,
            (Self::Eingabe { .. }, Antwort::Text(t)) => !t.trim().is_empty(),
            (Self::Bausteine { teile, .. }, Antwort::Liste(l)) => l.len() == teile.len(),
            (Self::Reihenfolge { zeilen, .. }, Antwort::Liste(l)) => l.len() == zeilen.len(),
            (_, Antwort::Liste(l)) => !l.is_empty(),
            _ => true,
        }
    }

    /// `bausteine` are the shuffled blocks or lines as displayed.
    pub fn pruefen(&self, a: &Antwort, bausteine: &[String]) -> bool {
        match (self, a) {
            (_, Antwort::Fertig(ok)) => *ok,
            (Self::Auswahl { richtig, .. } | Self::Luecke { richtig, .. }, Antwort::Index(i)) => i == richtig,
            (Self::FehlerZeile { falsch, .. }, Antwort::Index(i)) => i == falsch,
            (Self::WahrFalsch { wahr, .. }, Antwort::Index(i)) => (*i == 0) == *wahr,
            (Self::Eingabe { antworten, .. }, Antwort::Text(t)) => {
                antworten.iter().any(|x| normalisieren(x) == normalisieren(t))
            }
            (Self::Bausteine { teile, .. }, Antwort::Liste(l)) => {
                let gebaut: Vec<&str> = l.iter().map(|&i| bausteine[i].as_str()).collect();
                gebaut == teile.iter().map(String::as_str).collect::<Vec<_>>()
            }
            (Self::Reihenfolge { zeilen, .. }, Antwort::Liste(l)) => {
                let gebaut: Vec<&str> = l.iter().map(|&i| bausteine[i].as_str()).collect();
                gebaut == zeilen.iter().map(String::as_str).collect::<Vec<_>>()
            }
            _ => false,
        }
    }

    /// Shuffled blocks or lines for display.
    pub fn gemischt(&self, rng: &mut Rng) -> Vec<String> {
        match self {
            Self::Bausteine { teile, extra, .. } => {
                let mut v: Vec<String> = teile.iter().chain(extra.iter()).cloned().collect();
                rng.mischen(&mut v);
                v
            }
            Self::Reihenfolge { zeilen, .. } => {
                let mut v = zeilen.clone();
                // Never show them already in the right order.
                for _ in 0..10 {
                    rng.mischen(&mut v);
                    if v != *zeilen {
                        break;
                    }
                }
                v
            }
            _ => Vec::new(),
        }
    }
}
