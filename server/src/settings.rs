//! Settings of the admin area, stored as `settings.json` in the data folder.

use std::path::Path;

use serde::{Deserialize, Serialize};

/// Everything the admin can change. Empty strings mean "not set".
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
#[serde(default)]
pub struct Settings {
    /// Plausible-compatible web analytics (Plausible, nrzAnalytics …).
    pub analytics_enabled: bool,
    /// Script URL, e.g. `https://analytics.example.org/js/script.js`.
    pub analytics_src: String,
    /// `data-domain` of the script, e.g. `learn.example.org`.
    pub analytics_domain: String,
    /// Site verification codes of the search engines.
    pub google: String,
    pub bing: String,
    pub yandex: String,
    pub naver: String,
    pub baidu: String,
    /// More HTML for the `<head>` of every page (the admin is responsible for it).
    pub head_extra: String,
}

/// Search engines with their verification meta tag.
pub const VERIFICATIONS: [(&str, &str, &str); 5] = [
    ("google", "Google Search Console", "google-site-verification"),
    ("bing", "Bing Webmaster Tools", "msvalidate.01"),
    ("yandex", "Yandex Webmaster", "yandex-verification"),
    ("naver", "Naver Search Advisor", "naver-site-verification"),
    ("baidu", "Baidu Webmaster", "baidu-site-verification"),
];

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

impl Settings {
    /// Defaults for a new installation: the analytics fields can be preset with
    /// `NRZLEARN_ANALYTICS_SRC` and `NRZLEARN_ANALYTICS_DOMAIN` (still switched off).
    pub fn defaults() -> Self {
        let var = |k: &str| std::env::var(k).ok().map(|v| v.trim().to_string()).unwrap_or_default();
        Self { analytics_src: var("NRZLEARN_ANALYTICS_SRC"), analytics_domain: var("NRZLEARN_ANALYTICS_DOMAIN"), ..Self::default() }
    }

    pub fn load(dir: Option<&Path>) -> Self {
        dir.and_then(|d| std::fs::read_to_string(d.join("settings.json")).ok())
            .and_then(|t| serde_json::from_str(&t).ok())
            .unwrap_or_else(Self::defaults)
    }

    pub fn save(&self, dir: &Path) -> std::io::Result<()> {
        let tmp = dir.join("settings.json.tmp");
        std::fs::write(&tmp, serde_json::to_string_pretty(self).unwrap_or_default())?;
        std::fs::rename(tmp, dir.join("settings.json"))
    }

    pub fn get(&self, key: &str) -> &str {
        match key {
            "google" => &self.google,
            "bing" => &self.bing,
            "yandex" => &self.yandex,
            "naver" => &self.naver,
            "baidu" => &self.baidu,
            _ => "",
        }
    }

    pub fn set(&mut self, key: &str, value: String) {
        match key {
            "google" => self.google = value,
            "bing" => self.bing = value,
            "yandex" => self.yandex = value,
            "naver" => self.naver = value,
            "baidu" => self.baidu = value,
            _ => {}
        }
    }

    /// Problems with the values (empty = valid).
    pub fn problems(&self) -> Vec<String> {
        let mut p = Vec::new();
        let src = self.analytics_src.trim();
        if !src.is_empty() && (!(src.starts_with("https://") || src.starts_with("http://")) || src.contains(['"', '<', '>', ' ', '\'']))
        {
            p.push("Die Skript-URL muss mit https:// beginnen und darf keine Leerzeichen oder Anführungszeichen enthalten.".into());
        }
        let domain = self.analytics_domain.trim();
        if !domain.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | ',' | ':')) {
            p.push("Die Domain darf nur Buchstaben, Ziffern, Punkte, Bindestriche und Kommas enthalten.".into());
        }
        if self.analytics_enabled && (src.is_empty() || domain.is_empty()) {
            p.push("Zum Einschalten braucht die Webanalyse Skript-URL und Domain.".into());
        }
        for (key, name, _) in VERIFICATIONS {
            if !self.get(key).chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_')) {
                p.push(format!("{name}: Der Code darf nur Buchstaben, Ziffern, - und _ enthalten (nur den Code, nicht das ganze Meta-Tag)."));
            }
        }
        if self.head_extra.len() > 20_000 {
            p.push("Der zusätzliche Code im <head> ist zu lang (höchstens 20 000 Zeichen).".into());
        }
        p
    }

    /// The tag of the web analytics, if it is switched on.
    pub fn analytics_tag(&self) -> Option<String> {
        (self.analytics_enabled && !self.analytics_src.trim().is_empty() && !self.analytics_domain.trim().is_empty())
            .then(|| format!("<script defer data-domain=\"{}\" src=\"{}\"></script>", esc(self.analytics_domain.trim()), esc(self.analytics_src.trim())))
    }

    /// Host of the analytics script (for the privacy page).
    pub fn analytics_host(&self) -> Option<String> {
        self.analytics_tag()?;
        let rest = self.analytics_src.trim().split("://").nth(1)?;
        Some(rest.split('/').next()?.to_string()).filter(|h| !h.is_empty())
    }

    /// Everything that goes into the `<head>` of every page.
    pub fn head_html(&self) -> String {
        let mut h = String::new();
        for (key, _, meta) in VERIFICATIONS {
            let v = self.get(key).trim();
            if !v.is_empty() {
                h.push_str(&format!("<meta name=\"{meta}\" content=\"{}\">", esc(v)));
            }
        }
        if let Some(tag) = self.analytics_tag() {
            h.push_str(&tag);
        }
        h.push_str(self.head_extra.trim());
        h
    }

    /// Nothing to change in the pages.
    pub fn is_neutral(&self) -> bool {
        self.head_html().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_the_head_and_checks_values() {
        let mut s = Settings { analytics_src: "https://analytics.example.org/js/script.js".into(), analytics_domain: "learn.example.org".into(), ..Settings::default() };
        assert!(s.is_neutral());
        assert_eq!(s.analytics_host(), None);
        s.analytics_enabled = true;
        s.google = "abc_DEF-123".into();
        assert!(s.problems().is_empty());
        let head = s.head_html();
        assert!(head.contains("<meta name=\"google-site-verification\" content=\"abc_DEF-123\">"));
        assert!(head.contains("<script defer data-domain=\"learn.example.org\" src=\"https://analytics.example.org/js/script.js\"></script>"));
        assert_eq!(s.analytics_host().as_deref(), Some("analytics.example.org"));
        s.bing = "<meta name=x>".into();
        s.analytics_src = "javascript:alert(1)".into();
        assert_eq!(s.problems().len(), 2);
    }
}
