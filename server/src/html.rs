//! HTML pages with the admin's additions (web analytics, verification tags,
//! the privacy section about analytics) and the link preview of the course for
//! app pages like `/de/python/learn`. Every page is changed and compressed only
//! once and then kept in memory until the settings change.

use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use axum::body::Body;
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::response::Response;

use crate::settings::Settings;

const START: &str = "<!--meta-->";
const ENDE: &str = "<!--/meta-->";

/// A finished page: plain, gzip and brotli.
pub struct Fertig {
    plain: Vec<u8>,
    gzip: Vec<u8>,
    br: Vec<u8>,
    etag: String,
}

pub struct Seiten {
    public: PathBuf,
    cache: Mutex<HashMap<String, Arc<Fertig>>>,
}

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// The link preview block of a static page (between `<!--meta-->` and `<!--/meta-->`).
fn block(html: &str) -> Option<&str> {
    let a = html.find(START)? + START.len();
    let b = a + html[a..].find(ENDE)?;
    Some(&html[a..b])
}

/// Puts `neu` into the link preview block of the app shell.
fn ersetze_block(html: &str, neu: &str) -> String {
    match (html.find(START), html.find(ENDE)) {
        (Some(a), Some(b)) if a < b => format!("{}{START}{neu}{}", &html[..a], &html[b..]),
        _ => html.replacen("<head>", &format!("<head>{START}{neu}{ENDE}"), 1),
    }
}

/// The admin's additions.
pub fn anpassen(html: String, s: &Settings) -> String {
    let mut html = html;
    let head = s.head_html();
    if !head.is_empty() {
        html = html.replacen("</head>", &format!("{head}</head>"), 1);
    }
    if let Some(host) = s.analytics_host() {
        html = html.replacen("<div data-analytics hidden>", "<div data-analytics>", 1).replace("{host}", &esc(&host));
    }
    html
}

fn fnv(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in bytes {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    h
}

fn komprimieren(plain: Vec<u8>) -> Fertig {
    let mut gz = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::best());
    let _ = gz.write_all(&plain);
    let gzip = gz.finish().unwrap_or_default();
    let mut br = Vec::new();
    {
        let mut w = brotli::CompressorWriter::new(&mut br, 4096, 9, 22);
        let _ = w.write_all(&plain);
    }
    let etag = format!("\"h{:016x}\"", fnv(&plain));
    Fertig { plain, gzip, br, etag }
}

impl Seiten {
    pub fn new(public: PathBuf) -> Self {
        Self { public, cache: Mutex::new(HashMap::new()) }
    }

    pub fn leeren(&self) {
        if let Ok(mut c) = self.cache.lock() {
            c.clear();
        }
    }

    fn holen(&self, key: &str, machen: impl FnOnce() -> Option<String>) -> Option<Arc<Fertig>> {
        if let Some(f) = self.cache.lock().ok()?.get(key) {
            return Some(Arc::clone(f));
        }
        let fertig = Arc::new(komprimieren(machen()?.into_bytes()));
        let mut c = self.cache.lock().ok()?;
        if c.len() > 4000 {
            c.clear();
        }
        c.insert(key.to_string(), Arc::clone(&fertig));
        Some(fertig)
    }

    /// A static page (`de/python/index.html`) with the admin's additions.
    pub fn seite(&self, datei: &str, s: &Settings) -> Option<Arc<Fertig>> {
        self.holen(datei, || Some(anpassen(std::fs::read_to_string(self.public.join(datei)).ok()?, s)))
    }

    /// The app shell; for routes of a course (`kurs_seite` = `de/python/index.html`)
    /// with the link preview of that course page.
    pub fn app(&self, kurs_seite: Option<&str>, s: &Settings) -> Option<Arc<Fertig>> {
        let key = format!("app:{}", kurs_seite.unwrap_or(""));
        self.holen(&key, || {
            let mut html = std::fs::read_to_string(self.public.join("app.html")).ok()?;
            if let Some(seite) = kurs_seite.and_then(|k| std::fs::read_to_string(self.public.join(k)).ok())
                && let Some(meta) = block(&seite) {
                    html = ersetze_block(&html, meta);
                }
            Some(anpassen(html, s))
        })
    }
}

/// Sends a finished page: `304` for a matching ETag, else the best encoding.
pub fn antwort(f: &Fertig, req: &HeaderMap) -> Response {
    let etag = HeaderValue::from_str(&f.etag).unwrap_or_else(|_| HeaderValue::from_static("\"x\""));
    if req.get(header::IF_NONE_MATCH).and_then(|v| v.to_str().ok()).is_some_and(|v| v.split(',').any(|t| t.trim() == f.etag)) {
        return Response::builder().status(StatusCode::NOT_MODIFIED).header(header::ETAG, etag).body(Body::empty()).unwrap_or_default();
    }
    let accept = req.get(header::ACCEPT_ENCODING).and_then(|v| v.to_str().ok()).unwrap_or("");
    let (body, encoding) = if accept.contains("br") {
        (f.br.clone(), Some("br"))
    } else if accept.contains("gzip") {
        (f.gzip.clone(), Some("gzip"))
    } else {
        (f.plain.clone(), None)
    };
    let mut res = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
        .header(header::VARY, "accept-encoding")
        .header(header::ETAG, etag);
    if let Some(e) = encoding {
        res = res.header(header::CONTENT_ENCODING, e);
    }
    res.body(Body::from(body)).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replaces_the_link_preview_and_adds_the_head() {
        let seite = "<head><!--meta--><title>Python lernen</title><meta property=\"og:image\" content=\"/og/python.png\"><!--/meta--></head>";
        let app = "<head><!--meta--><meta property=\"og:image\" content=\"/og.png\"><!--/meta--><title>nrzLearn</title></head>";
        let neu = ersetze_block(app, block(seite).unwrap());
        assert!(neu.contains("<title>Python lernen</title>") && neu.contains("og/python.png") && !neu.contains("content=\"/og.png\""));
        let s = Settings { analytics_enabled: true, analytics_src: "https://a.example.org/js/script.js".into(), analytics_domain: "learn.example.org".into(), ..Settings::default() };
        let html = anpassen("<head></head><div data-analytics hidden><p>on {host}</p></div>".into(), &s);
        assert!(html.contains("data-domain=\"learn.example.org\" src=\"https://a.example.org/js/script.js\"></script></head>"));
        assert!(html.contains("<div data-analytics><p>on a.example.org</p></div>"));
        assert_eq!(anpassen("<head></head>".into(), &Settings::default()), "<head></head>");
    }
}
