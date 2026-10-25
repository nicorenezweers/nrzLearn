//! Data model for lessons, content blocks and tasks. Each language has its
//! own complete set of lessons with the same structure.

mod lessons_de;
mod lessons_en;
mod lessons_es;
pub mod spick;

use crate::i18n::Sprache;

pub const ANZAHL_WELTEN: usize = 16;

// Short constructors used by the lesson files.
pub(super) const fn sv(name: &'static str, wert: &'static str, status: Status) -> StackVar {
    StackVar { name, wert, heap: None, status }
}
pub(super) const fn sp(name: &'static str, wert: &'static str, heap: usize, status: Status) -> StackVar {
    StackVar { name, wert, heap: Some(heap), status }
}
pub(super) const fn hb(inhalt: &'static str, status: Status) -> HeapBlock {
    HeapBlock { inhalt, status }
}
pub(super) const fn ts(zeile: usize, werte: &'static [&'static str], text: &'static str) -> TraceSchritt {
    TraceSchritt { zeile, werte, text }
}


pub fn lektionen(l: Sprache) -> &'static [Lektion] {
    match l {
        Sprache::En => lessons_en::LEKTIONEN_EN,
        Sprache::De => lessons_de::LEKTIONEN_DE,
        Sprache::Es => lessons_es::LEKTIONEN_ES,
    }
}

pub struct Lektion {
    pub titel: &'static str,
    pub kurz: &'static str,
    pub kapitel: &'static str,
    pub bloecke: &'static [Block],
    pub aufgaben: &'static [Aufgabe],
}

pub enum Block {
    Ueberschrift(&'static str),
    /// Fließtext. Unterstützt `code` und **fett**.
    Text(&'static str),
    /// Statischer, eingefärbter Code.
    Code(&'static str),
    /// Editierbarer Code, der echt ausgeführt werden kann.
    Ausfuehrbar(&'static str),
    Tipp(&'static str),
    Achtung(&'static str),
    Visual(Visual),
    /// Ein simuliertes Terminal zum freien Ausprobieren.
    Terminal,
}

pub enum Visual {
    Pipeline,
    Bits,
    Typgroessen,
    Speicher { code: &'static str, schritte: &'static [SpeicherSchritt] },
    Trace { code: &'static str, spalten: &'static [&'static str], schritte: &'static [TraceSchritt] },
    Vec,
    Match,
    Iterator,
    Parse,
    Formen,
    Scope,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Gueltig,
    Neu,
    Verschoben,
    Geliehen,
    Freigegeben,
}

pub struct StackVar {
    pub name: &'static str,
    pub wert: &'static str,
    /// Index des Heap-Blocks, auf den die Variable zeigt.
    pub heap: Option<usize>,
    pub status: Status,
}

pub struct HeapBlock {
    pub inhalt: &'static str,
    pub status: Status,
}

pub struct SpeicherSchritt {
    /// 1-basierte Codezeile, die gerade ausgeführt wurde.
    pub zeile: usize,
    pub stack: &'static [StackVar],
    pub heap: &'static [HeapBlock],
    pub text: &'static str,
    pub ausgabe: &'static str,
}

pub struct TraceSchritt {
    pub zeile: usize,
    pub werte: &'static [&'static str],
    pub text: &'static str,
}

/// Feste Aufgaben aus den Lektionen. Die `id`s bleiben zur Orientierung im Inhalt.
#[allow(dead_code)]
pub enum Aufgabe {
    Quiz {
        id: &'static str,
        frage: &'static str,
        code: Option<&'static str>,
        optionen: &'static [&'static str],
        richtig: usize,
        erklaerung: &'static str,
    },
    Rechnung {
        id: &'static str,
        frage: &'static str,
        code: Option<&'static str>,
        antworten: &'static [&'static str],
        hinweis: &'static str,
        erklaerung: &'static str,
    },
    Code {
        id: &'static str,
        aufgabe: &'static str,
        vorlage: &'static str,
        erwartet: &'static str,
        hinweis: &'static str,
        loesung: &'static str,
    },
    Terminal {
        id: &'static str,
        aufgabe: &'static str,
        ziele: &'static [&'static str],
        hinweis: &'static str,
    },
}
