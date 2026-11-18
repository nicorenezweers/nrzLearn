//! Admin area on its own port: web analytics (tracking script), SEO entries
//! (verification codes, extra `<head>` code) and the anonymous learning
//! statistics. Login with the password from `NRZLEARN_ADMIN_PASSWORD`.

use std::collections::HashMap;
use std::fmt::Write as _;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};

use axum::Form;
use axum::Router;
use axum::extract::{Query, State};
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::routing::{get, post};

use crate::Zustand;
use crate::settings::{Settings, VERIFICATIONS};
use crate::stats::{Counter, Stats};

const COOKIE: &str = "nrzlearn_admin";
const SITZUNG: Duration = Duration::from_secs(12 * 3600);

const CSS: &str = "*{box-sizing:border-box}body{margin:0;font:15px/1.5 system-ui,-apple-system,Segoe UI,Roboto,sans-serif;background:#f6f6f6;color:#111}a{color:inherit}main{max-width:1100px;margin:0 auto;padding:24px 16px 64px}header{display:flex;align-items:center;gap:12px;margin-bottom:24px}header h1{font-size:20px;margin:0}header form{margin-left:auto}h2{font-size:17px;margin:0 0 12px}section{background:#fff;border:2px solid #e5e5e5;border-radius:16px;padding:20px;margin-top:16px}.kpis{display:grid;grid-template-columns:repeat(auto-fit,minmax(180px,1fr));gap:12px}.kpi{background:#fff;border:2px solid #e5e5e5;border-radius:16px;padding:16px}.kpi b{display:block;font-size:28px;line-height:1.2}.kpi span{color:#666;font-size:13px}table{width:100%;border-collapse:collapse}td,th{padding:6px 8px;text-align:left;border-bottom:1px solid #eee;font-size:14px}th{color:#666;font-weight:600}td.n{text-align:right;font-variant-numeric:tabular-nums}.bar{height:8px;border-radius:4px;background:#111;min-width:2px}.tile{display:inline-flex;width:26px;height:26px;border-radius:7px;align-items:center;justify-content:center;vertical-align:middle;margin-right:8px}.tile svg{width:62%;height:62%}.days{display:flex;align-items:flex-end;gap:3px;height:120px;border-bottom:1px solid #ddd}.days div{flex:1;background:#111;border-radius:3px 3px 0 0;min-height:2px}.days div.leer{background:#e5e5e5}.achse{display:flex;justify-content:space-between;color:#666;font-size:12px;margin-top:4px}label{display:block;font-weight:600;margin:12px 0 4px}input[type=text],input[type=password],textarea{width:100%;padding:9px 11px;border:2px solid #ddd;border-radius:10px;font:inherit}textarea{min-height:110px;font-family:ui-monospace,Menlo,monospace;font-size:13px}input:focus,textarea:focus{outline:none;border-color:#111}.check{display:flex;gap:8px;align-items:center;font-weight:600;margin-top:8px}button{font:inherit;font-weight:700;border:2px solid #111;background:#111;color:#fff;border-radius:12px;padding:9px 16px;cursor:pointer;margin-top:14px}button.leise{background:#fff;color:#111;margin:0}button.rot{background:#b42318;border-color:#b42318}code,pre{font-family:ui-monospace,Menlo,monospace;font-size:13px}pre{background:#111;color:#eee;padding:12px;border-radius:10px;overflow:auto;white-space:pre-wrap}.hint{color:#666;font-size:13px;margin:4px 0 0}.ok{background:#e7f6ec;border-color:#a6dfb6}.err{background:#fdecea;border-color:#f5b5ae}.grid2{display:grid;grid-template-columns:repeat(auto-fit,minmax(320px,1fr));gap:16px}.login{max-width:380px;margin:12vh auto}";

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

fn seite(titel: &str, body: &str) -> Html<String> {
    Html(format!(
        "<!doctype html><html lang=\"de\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><meta name=\"robots\" content=\"noindex\"><title>{} · nrzLearn Admin</title><style>{CSS}</style></head><body><main>{body}</main></body></html>",
        esc(titel)
    ))
}

fn zahl(n: u64) -> String {
    let s = n.to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            out.push('\u{202f}');
        }
        out.push(c);
    }
    out
}

/// Random session token (32 bytes from the OS, hex).
fn token() -> String {
    let mut buf = [0u8; 32];
    let ok = std::fs::File::open("/dev/urandom").and_then(|mut f| std::io::Read::read_exact(&mut f, &mut buf)).is_ok();
    if !ok {
        let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0);
        buf[..16].copy_from_slice(&t.to_le_bytes());
    }
    buf.iter().map(|b| format!("{b:02x}")).collect()
}

fn gleich(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && a.iter().zip(b).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

fn angemeldet(z: &Zustand, headers: &HeaderMap) -> bool {
    let Some(cookie) = headers.get(header::COOKIE).and_then(|v| v.to_str().ok()) else { return false };
    let Some(t) = cookie.split(';').filter_map(|p| p.trim().strip_prefix(&format!("{COOKIE}="))).next() else { return false };
    let mut s = z.sitzungen.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    s.retain(|_, ab| ab.elapsed() < SITZUNG);
    s.contains_key(t)
}

pub fn router(z: Arc<Zustand>) -> Router {
    Router::new()
        .route("/", get(start))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/settings", post(speichern))
        .route("/stats/reset", post(zuruecksetzen))
        .route("/stats.json", get(export))
        .with_state(z)
}

fn login_seite(fehler: bool) -> Response {
    let hinweis = if fehler { "<p class=\"hint\" style=\"color:#b42318\">Benutzer oder Passwort stimmen nicht.</p>" } else { "" };
    seite(
        "Anmelden",
        &format!("<div class=\"login\"><section><h2>nrzLearn · Admin</h2><form method=\"post\" action=\"/login\"><label for=\"user\">Benutzer</label><input id=\"user\" type=\"text\" name=\"user\" value=\"admin\" autocomplete=\"username\"><label for=\"pw\">Passwort</label><input id=\"pw\" type=\"password\" name=\"password\" autofocus autocomplete=\"current-password\">{hinweis}<button>Anmelden</button></form><p class=\"hint\">Benutzer: admin. Auf umbrelOS steht das Passwort in den Details der App (Standard-Zugangsdaten).</p></section></div>"),
    )
    .into_response()
}

async fn login(State(z): State<Arc<Zustand>>, Form(f): Form<HashMap<String, String>>) -> Response {
    let Some(pw) = z.admin_passwort.as_deref() else { return StatusCode::NOT_FOUND.into_response() };
    let eingabe = f.get("password").map(String::as_str).unwrap_or("");
    let user = f.get("user").map(|u| u.trim()).unwrap_or("admin");
    if user != "admin" || !gleich(eingabe.as_bytes(), pw.as_bytes()) {
        tokio::time::sleep(Duration::from_millis(800)).await;
        return login_seite(true);
    }
    let t = token();
    z.sitzungen.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(t.clone(), Instant::now());
    let mut res = Redirect::to("/").into_response();
    let cookie = format!("{COOKIE}={t}; HttpOnly; SameSite=Strict; Path=/; Max-Age={}", SITZUNG.as_secs());
    if let Ok(v) = HeaderValue::from_str(&cookie) {
        res.headers_mut().insert(header::SET_COOKIE, v);
    }
    res
}

async fn logout(State(z): State<Arc<Zustand>>, headers: HeaderMap) -> Response {
    if let Some(cookie) = headers.get(header::COOKIE).and_then(|v| v.to_str().ok())
        && let Some(t) = cookie.split(';').filter_map(|p| p.trim().strip_prefix(&format!("{COOKIE}="))).next() {
            z.sitzungen.lock().unwrap_or_else(std::sync::PoisonError::into_inner).remove(t);
        }
    let mut res = Redirect::to("/").into_response();
    res.headers_mut().insert(header::SET_COOKIE, HeaderValue::from_static("nrzlearn_admin=; HttpOnly; SameSite=Strict; Path=/; Max-Age=0"));
    res
}

fn kachel(id: &str) -> String {
    let Some(c) = nrzlearn_model::course_info(id) else { return String::new() };
    match nrzlearn_model::icons::icon(id) {
        Some(i) => format!("<span class=\"tile\" style=\"background:{};color:{}\"><svg viewBox=\"{}\" fill=\"currentColor\">{}</svg></span>", c.color, c.on_color, i.view_box, i.body),
        None => String::new(),
    }
}

fn tabelle(zeilen: &[(String, String, Counter)], erste: &str) -> String {
    if zeilen.is_empty() {
        return "<p class=\"hint\">Noch keine Daten. Sobald jemand ein Level abschließt, erscheint es hier.</p>".to_string();
    }
    let max = zeilen.iter().map(|z| z.2.levels).max().unwrap_or(1).max(1);
    let mut t = format!("<table><tr><th>{erste}</th><th style=\"text-align:right\">Gestartet</th><th style=\"text-align:right\">Lektionen</th><th style=\"text-align:right\">Level</th><th style=\"width:30%\"></th></tr>");
    for (vorne, name, c) in zeilen {
        let breite = c.levels * 100 / max;
        let _ = write!(
            t,
            "<tr><td>{vorne}{}</td><td class=\"n\">{}</td><td class=\"n\">{}</td><td class=\"n\">{}</td><td><div class=\"bar\" style=\"width:{breite}%\"></div></td></tr>",
            esc(name),
            zahl(c.starts),
            zahl(c.lessons),
            zahl(c.levels)
        );
    }
    t.push_str("</table>");
    t
}

/// SEO overview from the built pages: URLs in the sitemap, base URL.
fn seo_info(z: &Zustand) -> (usize, String) {
    let sitemap = std::fs::read_to_string(z.public.join("sitemap.xml")).unwrap_or_default();
    let n = sitemap.matches("<loc>").count();
    let basis = sitemap.split("<loc>").nth(1).and_then(|r| r.split("</loc>").next()).map(|u| u.trim_end_matches('/').to_string()).unwrap_or_default();
    (n, basis)
}

async fn start(State(z): State<Arc<Zustand>>, headers: HeaderMap, Query(q): Query<HashMap<String, String>>) -> Response {
    if z.admin_passwort.is_none() {
        return StatusCode::NOT_FOUND.into_response();
    }
    if !angemeldet(&z, &headers) {
        return login_seite(false);
    }
    let s = z.settings.read().unwrap_or_else(std::sync::PoisonError::into_inner).clone();
    let (meldung, fehler) = match q.get("m").map(String::as_str) {
        Some("saved") => ("<section class=\"ok\">Gespeichert. Alle Seiten enthalten die neuen Einträge sofort.</section>".to_string(), String::new()),
        Some("reset") => ("<section class=\"ok\">Die Statistik wurde zurückgesetzt.</section>".to_string(), String::new()),
        _ => (String::new(), String::new()),
    };
    let body = dashboard(&z, &s, &meldung, &fehler);
    seite("Übersicht", &body).into_response()
}

fn dashboard(z: &Zustand, s: &Settings, meldung: &str, fehler: &str) -> String {
    let stats = z.stats.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let mut kurse: Vec<(String, String, Counter)> = stats
        .courses
        .iter()
        .filter_map(|(id, c)| nrzlearn_model::course_info(id).map(|i| (kachel(id), i.name.to_string(), *c)))
        .collect();
    kurse.sort_by_key(|k| std::cmp::Reverse((k.2.starts, k.2.levels)));
    let mut sprachen: Vec<(String, String, Counter)> = stats
        .langs
        .iter()
        .filter_map(|(code, c)| nrzlearn_model::LANGUAGES.iter().find(|l| l.code == code).map(|l| (String::new(), format!("{} ({})", l.name, l.code), *c)))
        .collect();
    sprachen.sort_by_key(|k| std::cmp::Reverse((k.2.levels, k.2.starts)));

    // The last 30 days.
    let mut tage = String::new();
    let heute = crate::stats::today_number();
    let letzte: Vec<(String, Counter)> = (0..30).rev().map(|i| crate::stats::day(heute - i)).map(|t| (t.clone(), stats.days.get(&t).copied().unwrap_or_default())).collect();
    let max = letzte.iter().map(|(_, c)| c.levels).max().unwrap_or(0).max(1);
    for (tag, c) in &letzte {
        let h = c.levels * 100 / max;
        let leer = if c.levels == 0 { " class=\"leer\"" } else { "" };
        let _ = write!(tage, "<div{leer} title=\"{tag}: {} Level, {} gestartet\" style=\"height:{h}%\"></div>", c.levels, c.starts);
    }
    let tage = if stats.days.is_empty() {
        "<p class=\"hint\">Noch keine Daten.</p>".to_string()
    } else {
        format!("<div class=\"days\">{tage}</div><div class=\"achse\"><span>{}</span><span>{}</span></div><p class=\"hint\">Abgeschlossene Level pro Tag (UTC).</p>", letzte[0].0, letzte[29].0)
    };

    let t = &stats.total;
    let seit = if stats.since.is_empty() { "–".to_string() } else { stats.since.clone() };
    let kpis = format!(
        "<div class=\"kpis\"><div class=\"kpi\"><b>{}</b><span>Abgeschlossene Lektionen</span></div><div class=\"kpi\"><b>{}</b><span>Abgeschlossene Level (mit Wiederholungen)</span></div><div class=\"kpi\"><b>{}</b><span>Gestartete Kurse</span></div><div class=\"kpi\"><b>{}</b><span>Bestandene Prüfungen</span></div><div class=\"kpi\"><b>{seit}</b><span>Gezählt seit</span></div></div>",
        zahl(t.lessons),
        zahl(t.levels),
        zahl(t.starts),
        zahl(t.exams)
    );

    let tag_vorschau = s.analytics_tag().unwrap_or_else(|| "(ausgeschaltet)".to_string());
    let mut verifikation = String::new();
    for (key, name, meta) in VERIFICATIONS {
        let _ = write!(
            verifikation,
            "<label for=\"{key}\">{name}</label><input id=\"{key}\" type=\"text\" name=\"{key}\" value=\"{}\" placeholder=\"Nur der Code aus content=&quot;…&quot;\" autocomplete=\"off\"><p class=\"hint\">Wird zu <code>&lt;meta name=\"{meta}\" content=\"…\"&gt;</code></p>",
            esc(s.get(key))
        );
    }
    let (urls, basis) = seo_info(z);
    let links = if basis.is_empty() {
        String::new()
    } else {
        format!(
            "<p>Öffentliche Seite: <a href=\"{b}/\" target=\"_blank\" rel=\"noopener\">{b}</a> · <a href=\"{b}/sitemap.xml\" target=\"_blank\" rel=\"noopener\">sitemap.xml</a> · <a href=\"{b}/robots.txt\" target=\"_blank\" rel=\"noopener\">robots.txt</a></p>",
            b = esc(&basis)
        )
    };
    let n_kurse = nrzlearn_model::COURSES.len();
    let n_sprachen = nrzlearn_model::LANGUAGES.len();
    let speicher = if z.daten.is_some() { "" } else { "<section class=\"err\">Kein Datenordner (NRZLEARN_DATA): Einstellungen und Statistik gehen beim Neustart verloren.</section>" };
    format!(
        "<header><h1>nrzLearn · Admin</h1><form method=\"post\" action=\"/logout\"><button class=\"leise\">Abmelden</button></form></header>{speicher}{meldung}{fehler}{kpis}\
<section><h2>Beliebteste Programmiersprachen</h2>{kurse}</section>\
<div class=\"grid2\"><section><h2>Oberflächensprachen</h2>{sprachen}</section><section><h2>Letzte 30 Tage</h2>{tage}<p class=\"hint\">Die Zählung ist anonym: Der Browser meldet nur Kurs, Oberflächensprache und Art des Levels – ohne Kennung, Cookie oder gespeicherte IP-Adresse.</p><p><a href=\"/stats.json\">Statistik als JSON herunterladen</a></p></section></div>\
<form method=\"post\" action=\"/settings\"><div class=\"grid2\"><section><h2>Tracking (Webanalyse)</h2><p class=\"hint\">Für Plausible-kompatible Webanalyse wie nrzAnalytics oder Plausible. Das Skript kommt auf alle Seiten, und die Datenschutzseite nennt den Analyse-Server automatisch.</p><label class=\"check\"><input type=\"checkbox\" name=\"analytics_enabled\" value=\"1\"{an}> Webanalyse einschalten</label><label for=\"src\">Skript-URL (Tracking-Link)</label><input id=\"src\" type=\"text\" name=\"analytics_src\" value=\"{src}\" placeholder=\"https://analytics.example.org/js/script.js\" autocomplete=\"off\"><label for=\"domain\">Domain (data-domain)</label><input id=\"domain\" type=\"text\" name=\"analytics_domain\" value=\"{domain}\" placeholder=\"learn.example.org\" autocomplete=\"off\"><label>So sieht das Skript aus</label><pre>{tag}</pre></section>\
<section><h2>SEO-Einträge</h2><p>{urls} Seiten in der Sitemap · {n_kurse} Kurse · {n_sprachen} Sprachen. Jede Seite hat hreflang, eine kanonische URL, strukturierte Daten und eine eigene Link-Vorschau.</p>{links}<p class=\"hint\">Bestätige die Website bei den Suchmaschinen (<a href=\"https://search.google.com/search-console\" target=\"_blank\" rel=\"noopener\">Google Search Console</a>, <a href=\"https://www.bing.com/webmasters\" target=\"_blank\" rel=\"noopener\">Bing Webmaster Tools</a>) per Meta-Tag und reiche dort die Sitemap ein.</p>{verifikation}<label for=\"extra\">Weiterer Code im &lt;head&gt; jeder Seite</label><textarea id=\"extra\" name=\"head_extra\" placeholder=\"&lt;meta name=&quot;…&quot; content=&quot;…&quot;&gt;\">{extra}</textarea><p class=\"hint\">Für eigene Skripte bist du selbst für die Datenschutzerklärung verantwortlich.</p></section></div><button>Speichern</button></form>\
<section><h2>Statistik zurücksetzen</h2><form method=\"post\" action=\"/stats/reset\" onsubmit=\"return confirm('Statistik wirklich löschen?')\"><label class=\"check\"><input type=\"checkbox\" name=\"sicher\" value=\"1\" required> Ja, alle Zählungen löschen</label><button class=\"rot\">Zurücksetzen</button></form></section>",
        kurse = tabelle(&kurse, "Programmiersprache"),
        sprachen = tabelle(&sprachen, "Sprache"),
        an = if s.analytics_enabled { " checked" } else { "" },
        src = esc(&s.analytics_src),
        domain = esc(&s.analytics_domain),
        tag = esc(&tag_vorschau),
        extra = esc(&s.head_extra),
    )
}

async fn speichern(State(z): State<Arc<Zustand>>, headers: HeaderMap, Form(f): Form<HashMap<String, String>>) -> Response {
    if !angemeldet(&z, &headers) {
        return Redirect::to("/").into_response();
    }
    let feld = |k: &str| f.get(k).map(|v| v.trim().to_string()).unwrap_or_default();
    let mut neu = Settings { analytics_enabled: f.get("analytics_enabled").is_some_and(|v| !v.trim().is_empty()), analytics_src: feld("analytics_src"), analytics_domain: feld("analytics_domain"), head_extra: f.get("head_extra").cloned().unwrap_or_default(), ..Settings::default() };
    for (key, _, _) in VERIFICATIONS {
        neu.set(key, feld(key));
    }
    let probleme = neu.problems();
    if !probleme.is_empty() {
        let liste: String = probleme.iter().map(|p| format!("<li>{}</li>", esc(p))).collect();
        let fehler = format!("<section class=\"err\"><b>Nicht gespeichert:</b><ul>{liste}</ul></section>");
        return seite("Übersicht", &dashboard(&z, &neu, "", &fehler)).into_response();
    }
    if let Some(dir) = &z.daten
        && let Err(e) = neu.save(dir) {
            let fehler = format!("<section class=\"err\">Speichern fehlgeschlagen: {}</section>", esc(&e.to_string()));
            return seite("Übersicht", &dashboard(&z, &neu, "", &fehler)).into_response();
        }
    *z.settings.write().unwrap_or_else(std::sync::PoisonError::into_inner) = neu;
    z.seiten.leeren();
    Redirect::to("/?m=saved").into_response()
}

async fn zuruecksetzen(State(z): State<Arc<Zustand>>, headers: HeaderMap, Form(f): Form<HashMap<String, String>>) -> Response {
    if !angemeldet(&z, &headers) || !f.contains_key("sicher") {
        return Redirect::to("/").into_response();
    }
    *z.stats.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Stats::default();
    z.geaendert.store(true, Ordering::Relaxed);
    Redirect::to("/?m=reset").into_response()
}

async fn export(State(z): State<Arc<Zustand>>, headers: HeaderMap) -> Response {
    if !angemeldet(&z, &headers) {
        return Redirect::to("/").into_response();
    }
    let json = serde_json::to_string_pretty(&*z.stats.lock().unwrap_or_else(std::sync::PoisonError::into_inner)).unwrap_or_default();
    ([(header::CONTENT_TYPE, "application/json"), (header::CONTENT_DISPOSITION, "attachment; filename=\"nrzlearn-stats.json\"")], json).into_response()
}
