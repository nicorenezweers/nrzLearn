//! Learning progress: completed levels with stars, XP, streak and daily
//! goal. Stored only in this browser (localStorage), never on a server.
//! It can be exported to a JSON file and imported again (profile page).

use std::collections::BTreeMap;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::welt;

pub const XP_PRO_LEVEL: u32 = 250;

/// localStorage key of the progress.
const SPEICHER: &str = "nrzlearn.progress";

fn storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok().flatten()
}

/// False when the browser refuses to store data (some private modes):
/// then the progress only lives until the tab is closed.
pub fn speicher_verfuegbar() -> bool {
    storage().is_some_and(|s| s.set_item("nrzlearn.test", "1").is_ok() && s.remove_item("nrzlearn.test").is_ok())
}

fn ja() -> bool {
    true
}
fn ziel_standard() -> u32 {
    60
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Fortschritt {
    /// Abgeschlossene Level → Sterne (1 bis 3).
    #[serde(default)]
    pub knoten: BTreeMap<String, u8>,
    #[serde(default)]
    pub xp: u32,
    #[serde(default)]
    pub serie: u32,
    #[serde(default)]
    pub letzter_tag: String,
    #[serde(default)]
    pub heute: String,
    #[serde(default)]
    pub heute_xp: u32,
    #[serde(default = "ziel_standard")]
    pub ziel: u32,
    #[serde(default = "ja")]
    pub ton: bool,
    #[serde(default)]
    pub alles_offen: bool,
    #[serde(default)]
    pub geloest: u32,
    #[serde(default)]
    pub perfekt: u32,
}

impl Default for Fortschritt {
    fn default() -> Self {
        Self {
            knoten: BTreeMap::new(),
            xp: 0,
            serie: 0,
            letzter_tag: String::new(),
            heute: String::new(),
            heute_xp: 0,
            ziel: ziel_standard(),
            ton: true,
            alles_offen: false,
            geloest: 0,
            perfekt: 0,
        }
    }
}

/// Was beim Abschließen eines Levels Besonderes passiert ist – für die Belohnungen.
#[derive(Clone, PartialEq, Default, Debug)]
pub struct Belohnung {
    pub xp: u32,
    pub level_neu: Option<u32>,
    pub serie_neu: Option<u32>,
    pub ziel_erreicht: bool,
    pub neue_karte: bool,
    pub sterne_vorher: u8,
}

fn datum(versatz_tage: f64) -> String {
    let d = js_sys::Date::new_0();
    d.set_time(js_sys::Date::now() - versatz_tage * 86_400_000.0);
    format!("{:04}-{:02}-{:02}", d.get_full_year(), d.get_month() + 1, d.get_date())
}

impl Fortschritt {
    fn laden(json: Option<&str>) -> Self {
        let mut f: Self = json.and_then(|j| serde_json::from_str(j).ok()).unwrap_or_default();
        f.tag_pruefen();
        f
    }

    fn aus_speicher() -> Self {
        Self::laden(storage().and_then(|s| s.get_item(SPEICHER).ok().flatten()).as_deref())
    }

    /// Writes the progress to this browser's local storage.
    fn speichern(&self) {
        if let (Some(s), Ok(json)) = (storage(), serde_json::to_string(self)) {
            let _ = s.set_item(SPEICHER, &json);
        }
    }

    /// Neuer Tag: Tages-XP zurücksetzen, Serie verfällt nach einem Tag Pause.
    fn tag_pruefen(&mut self) {
        let heute = datum(0.0);
        if self.heute != heute {
            self.heute = heute;
            self.heute_xp = 0;
        }
        if !self.letzter_tag.is_empty() && self.letzter_tag != datum(0.0) && self.letzter_tag != datum(1.0) {
            self.serie = 0;
        }
    }

    pub fn sterne(&self, id: &str) -> u8 {
        self.knoten.get(id).copied().unwrap_or(0)
    }

    pub fn fertig(&self, id: &str) -> bool {
        self.knoten.contains_key(id)
    }

    /// Ein Level ist offen, wenn das vorherige abgeschlossen ist.
    pub fn offen(&self, knoten: &[welt::Knoten], index: usize) -> bool {
        self.alles_offen || index == 0 || knoten.get(index - 1).is_some_and(|k| self.fertig(&k.id))
    }

    /// Index des nächsten nicht abgeschlossenen Levels.
    pub fn aktuell(&self, knoten: &[welt::Knoten]) -> usize {
        knoten.iter().position(|k| !self.fertig(&k.id)).unwrap_or(knoten.len().saturating_sub(1))
    }

    pub fn level(&self) -> u32 {
        self.xp / XP_PRO_LEVEL + 1
    }

    /// Levels finished in a course (by the prefix of their ids).
    pub fn fertig_im_kurs(&self, kurs: crate::kurs::Kurs) -> usize {
        if kurs.ist_rust() {
            self.knoten.keys().filter(|k| !k.contains(':')).count()
        } else {
            let prefix = format!("{}:", kurs.id());
            self.knoten.keys().filter(|k| k.starts_with(&prefix)).count()
        }
    }

    pub fn ist_heute_aktiv(&self) -> bool {
        self.letzter_tag == datum(0.0)
    }
}

/// Loads the progress from local storage and keeps it in sync when another
/// tab of nrzLearn saves (otherwise the last tab to save would win).
pub fn provide_fortschritt() {
    let mut sig = use_context_provider(|| Signal::new(Fortschritt::aus_speicher()));
    use_hook(move || {
        spawn(async move {
            let mut ereignisse = document::eval(&format!(
                "window.addEventListener('storage', function (e) {{ if (e.key === '{SPEICHER}' || e.key === null) dioxus.send(true); }});"
            ));
            while ereignisse.recv::<bool>().await.is_ok() {
                sig.set(Fortschritt::aus_speicher());
            }
        });
    });
}

pub fn use_fortschritt() -> Signal<Fortschritt> {
    use_context()
}

/// Art eines Levels aus seiner ID (`python:hello-lesson`, `w3-pruefung`, `m2` …).
fn level_art(id: &str) -> (&str, &'static str) {
    let (kurs, rest) = id.split_once(':').unwrap_or(("rust", id));
    let art = if rest.ends_with("-lesson") || rest.ends_with("-theorie") {
        "lesson"
    } else if rest.ends_with("-exam") || rest.ends_with("-pruefung") {
        "exam"
    } else if rest.starts_with("master-") || (kurs == "rust" && rest.len() > 1 && rest.starts_with('m') && rest[1..].chars().all(|c| c.is_ascii_digit())) {
        "master"
    } else if rest.ends_with("-labor") {
        "lab"
    } else {
        "practice"
    };
    (kurs, art)
}

/// Meldet ein abgeschlossenes Level anonym an den Server: nur Kurs,
/// Oberflächensprache und Art des Levels, ohne Kennung (siehe Datenschutz).
fn melden(id: &str, erstes_mal: bool, kursstart: bool) {
    let Some(w) = web_sys::window() else { return };
    let (kurs, art) = level_art(id);
    let lang = w
        .location()
        .pathname()
        .ok()
        .and_then(|p| p.split('/').nth(1).map(str::to_string))
        .filter(|c| nrzlearn_model::LANGUAGES.iter().any(|l| l.code == c))
        .unwrap_or_else(|| "en".to_string());
    let json = format!(r#"{{"c":"{kurs}","l":"{lang}","k":"{art}","f":{erstes_mal},"s":{kursstart}}}"#);
    let _ = w.navigator().send_beacon_with_opt_str("/api/stats", Some(&json));
}

/// Schließt ein Level ab und vergibt XP, Sterne und Tagesserie.
pub fn abschliessen(mut sig: Signal<Fortschritt>, id: &str, sterne: u8, xp: u32, geloest: u32, perfekt: bool) -> Belohnung {
    let mut f = sig.write();
    f.tag_pruefen();
    let level_vorher = f.level();
    let ziel_vorher = f.heute_xp >= f.ziel;
    let sterne_vorher = f.sterne(id);

    f.xp += xp;
    f.heute_xp += xp;
    f.geloest += geloest;
    if perfekt {
        f.perfekt += 1;
    }
    let neue_karte = id.ends_with("-theorie") && !f.fertig(id);
    let kursstart = match id.split_once(':') {
        Some((kurs, _)) => !f.knoten.keys().any(|k| k.split_once(':').is_some_and(|(x, _)| x == kurs)),
        None => !f.knoten.keys().any(|k| !k.contains(':')),
    };
    f.knoten.insert(id.to_string(), sterne.max(sterne_vorher));
    melden(id, sterne_vorher == 0, kursstart);

    let heute = datum(0.0);
    let mut serie_neu = None;
    if f.letzter_tag != heute {
        f.serie = if f.letzter_tag == datum(1.0) { f.serie + 1 } else { 1 };
        f.letzter_tag = heute;
        serie_neu = Some(f.serie);
    }
    let b = Belohnung {
        xp,
        level_neu: (f.level() > level_vorher).then(|| f.level()),
        serie_neu,
        ziel_erreicht: !ziel_vorher && f.heute_xp >= f.ziel,
        neue_karte,
        sterne_vorher,
    };
    f.speichern();
    b
}

pub fn aendern(mut sig: Signal<Fortschritt>, f: impl FnOnce(&mut Fortschritt)) {
    let mut w = sig.write();
    f(&mut w);
    w.speichern();
}

pub fn zuruecksetzen(mut sig: Signal<Fortschritt>) {
    let mut f = sig.write();
    *f = Fortschritt::default();
    f.tag_pruefen();
    f.speichern();
}

/// Replaces the whole progress, e.g. with an imported backup.
pub fn ersetzen(mut sig: Signal<Fortschritt>, neu: Fortschritt) {
    let mut f = sig.write();
    *f = neu;
    f.tag_pruefen();
    f.speichern();
}

// ───────────────────────────── Backup file ─────────────────────────────

/// Marker in exported files.
const APP: &str = "nrzLearn";

#[derive(Serialize, Deserialize)]
struct Sicherung {
    app: String,
    format: u32,
    exported: String,
    progress: Fortschritt,
}

/// The progress as a JSON backup file.
pub fn export_json(f: &Fortschritt) -> String {
    let s = Sicherung { app: APP.into(), format: 1, exported: datum(0.0), progress: f.clone() };
    serde_json::to_string_pretty(&s).unwrap_or_default()
}

/// File name for a backup made today.
pub fn export_dateiname() -> String {
    format!("nrzlearn-{}.json", datum(0.0))
}

/// Reads a backup file. Also accepts the bare progress object, as nrzLearn
/// 3.x stored it on its server. Unknown levels are dropped, stars clamped.
pub fn import_json(text: &str) -> Option<Fortschritt> {
    let wert: serde_json::Value = serde_json::from_str(text.trim_start_matches('\u{feff}')).ok()?;
    let inhalt = if wert.get("app").and_then(|a| a.as_str()) == Some(APP) { wert.get("progress")?.clone() } else { wert };
    let objekt = inhalt.as_object()?;
    if !objekt.contains_key("knoten") && !objekt.contains_key("xp") {
        return None;
    }
    let mut f: Fortschritt = serde_json::from_value(inhalt).ok()?;
    // Rust levels have plain ids, JSON courses `<course>:<level>`.
    f.knoten.retain(|id, _| match id.split_once(':') {
        Some((kurs, rest)) => !rest.is_empty() && kurs.parse::<crate::kurs::Kurs>().is_ok_and(|k| !k.ist_rust()),
        None => welt::finden(id).is_some(),
    });
    for sterne in f.knoten.values_mut() {
        *sterne = (*sterne).clamp(1, 3);
    }
    f.ziel = f.ziel.clamp(10, 1000);
    Some(f)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_kinds_from_ids() {
        assert_eq!(level_art("python:hello-lesson"), ("python", "lesson"));
        assert_eq!(level_art("machine-code:bits-p2"), ("machine-code", "practice"));
        assert_eq!(level_art("java:master-2"), ("java", "master"));
        assert_eq!(level_art("w3-pruefung"), ("rust", "exam"));
        assert_eq!(level_art("m4"), ("rust", "master"));
        assert_eq!(level_art("w5-labor"), ("rust", "lab"));
        assert_eq!(level_art("w0-u1"), ("rust", "practice"));
    }

    #[test]
    fn import_accepts_backups_and_old_progress() {
        let mut f = Fortschritt { xp: 120, serie: 3, ..Fortschritt::default() };
        f.knoten.insert("w0-theorie".into(), 3);
        let datei = serde_json::to_string(&Sicherung { app: APP.into(), format: 1, exported: "2026-11-18".into(), progress: f.clone() }).unwrap();
        let neu = import_json(&datei).expect("backup");
        assert_eq!(neu.xp, 120);
        assert_eq!(neu.sterne("w0-theorie"), 3);

        // nrzLearn 3.x kept the bare progress object on its server.
        let alt = r#"{"knoten":{"w0-theorie":2,"w0-u1":9,"gibt-es-nicht":1,"python:loops-p1":3,"klingon:a-p1":2},"xp":75,"serie":1,"ziel":5}"#;
        let neu = import_json(alt).expect("old progress");
        assert_eq!(neu.xp, 75);
        assert_eq!(neu.sterne("w0-u1"), 3, "stars are clamped");
        assert!(!neu.knoten.contains_key("gibt-es-nicht"), "unknown levels are dropped");
        assert_eq!(neu.sterne("python:loops-p1"), 3, "levels of JSON courses are kept");
        assert!(!neu.knoten.contains_key("klingon:a-p1"), "unknown courses are dropped");
        assert_eq!(neu.ziel, 10);
    }

    #[test]
    fn import_rejects_other_files() {
        assert!(import_json("").is_none());
        assert!(import_json("not json").is_none());
        assert!(import_json("[1, 2, 3]").is_none());
        assert!(import_json(r#"{"name": "something else"}"#).is_none());
        assert!(import_json(r#"{"app": "nrzLearn"}"#).is_none());
    }
}
