//! Generates the UI text tables from `i18n/<lang>.json` (keys starting with
//! `app.`). Missing translations fall back to English; `nrzlearn-site
//! check-all` reports them. Usage in code: `t!(l, NAV_PATH)` for the key
//! `app.nav.path`.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;

fn read(code: &str) -> BTreeMap<String, String> {
    let path = Path::new("i18n").join(format!("{code}.json"));
    match std::fs::read_to_string(&path) {
        Ok(text) => serde_json::from_str(&text).unwrap_or_else(|e| panic!("{}: {e}", path.display())),
        Err(_) => BTreeMap::new(),
    }
}

fn const_name(key: &str) -> String {
    key.trim_start_matches("app.").replace(['.', '-'], "_").to_uppercase()
}

fn main() {
    println!("cargo::rerun-if-changed=i18n");
    let en = read("en");
    let keys: Vec<&String> = en.keys().filter(|k| k.starts_with("app.")).collect();
    let mut out = String::new();
    writeln!(out, "/// Number of UI texts.\npub const ANZAHL_TEXTE: usize = {};", keys.len()).unwrap();
    writeln!(out, "/// Indices of the UI texts, one constant per key in `i18n/en.json`.\n#[allow(dead_code)]\npub mod k {{").unwrap();
    for (i, k) in keys.iter().enumerate() {
        writeln!(out, "    pub const {}: usize = {i};", const_name(k)).unwrap();
    }
    writeln!(out, "}}").unwrap();
    writeln!(out, "static TEXTE: [[&str; ANZAHL_TEXTE]; {}] = [", nrzlearn_model::LANGUAGES.len()).unwrap();
    for lang in nrzlearn_model::LANGUAGES {
        let m = if lang.code == "en" { en.clone() } else { read(lang.code) };
        writeln!(out, "    [").unwrap();
        for k in &keys {
            let text = match m.get(*k).filter(|t| !t.trim().is_empty()) {
                Some(t) => nrzlearn_model::typography(lang.code, t),
                None => en[*k].clone(),
            };
            writeln!(out, "        {text:?},").unwrap();
        }
        writeln!(out, "    ],").unwrap();
    }
    writeln!(out, "];").unwrap();
    let dest = Path::new(&std::env::var("OUT_DIR").unwrap()).join("i18n.rs");
    std::fs::write(dest, out).unwrap();
}
