//! Learning progress: completed levels with stars, XP, streak and daily
//! goal. Stored on the server per user (see `api::fortschritt_speichern`).

use std::collections::BTreeMap;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::welt;

pub const XP_PRO_LEVEL: u32 = nrzlearn_shared::XP_PER_LEVEL;

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

    /// Sends the progress to the server in the background.
    fn speichern(&self) {
        if let Ok(json) = serde_json::to_string(self) {
            dioxus::core::spawn_forever(async move {
                let _ = crate::api::fortschritt_speichern(json).await;
            });
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
    pub fn offen(&self, index: usize) -> bool {
        self.alles_offen || index == 0 || welt::alle().get(index - 1).is_some_and(|k| self.fertig(&k.id))
    }

    /// Index des nächsten nicht abgeschlossenen Levels.
    pub fn aktuell(&self) -> usize {
        welt::alle().iter().position(|k| !self.fertig(&k.id)).unwrap_or(welt::alle().len() - 1)
    }

    pub fn level(&self) -> u32 {
        self.xp / XP_PRO_LEVEL + 1
    }

    pub fn karte_offen(&self, welt: usize) -> bool {
        self.fertig(&format!("w{welt}-theorie"))
    }

    pub fn karten(&self) -> usize {
        crate::content::spick::KARTEN.iter().filter(|k| self.karte_offen(k.welt)).count()
    }

    pub fn ist_heute_aktiv(&self) -> bool {
        self.letzter_tag == datum(0.0)
    }
}

pub fn provide_fortschritt(json: Option<&str>) {
    let json = json.map(str::to_string);
    use_context_provider(move || Signal::new(Fortschritt::laden(json.as_deref())));
}

pub fn use_fortschritt() -> Signal<Fortschritt> {
    use_context()
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
    f.knoten.insert(id.to_string(), sterne.max(sterne_vorher));

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
