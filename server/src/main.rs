//! nrzLearn server: delivers the web app, counts finished levels anonymously
//! and offers a small admin area on its own port.
//!
//! nrzLearn runs completely in the browser. There are no accounts and no
//! database; the learning progress stays in the browser's local storage.
//! The server hands out static files as cheaply as possible:
//!
//! - pre-compressed files (`.br`, `.gz`) are sent as they are,
//! - fingerprinted files under `/assets` are cached by the browser for a year,
//!   everything else is revalidated (`304 Not Modified`),
//! - directories get their `index.html` (the static landing and course pages),
//!   every other unknown path gets the app shell `app.html`, with the link
//!   preview of the course for routes like `/de/python/learn`.
//!
//! When the admin has added something to the pages (web analytics,
//! verification tags), HTML pages are changed once, compressed once and kept
//! in memory (see `html.rs`).
//!
//! Configuration (environment variables):
//! - `NRZLEARN_ADDR`           listen address, default `0.0.0.0:8080`
//! - `NRZLEARN_PUBLIC`         directory with the built web app, default `./public`
//! - `NRZLEARN_DATA`           folder for `settings.json` and `stats.json` (optional)
//! - `NRZLEARN_ADMIN_ADDR`     listen address of the admin area, default `0.0.0.0:8081`
//! - `NRZLEARN_ADMIN_PASSWORD` admin password; without it there is no admin area
//! - `NRZLEARN_ANALYTICS_SRC`, `NRZLEARN_ANALYTICS_DOMAIN` preset the analytics fields
//!
//! `nrzlearn-server healthcheck` checks a running server (exit code 0 = ok),
//! for container health checks in images without curl or wget.

mod admin;
mod html;
mod settings;
mod stats;

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};

use axum::Router;
use axum::body::{Body, Bytes};
use axum::extract::{DefaultBodyLimit, Request, State};
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::middleware::map_response;
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::{get, post};
use tower::ServiceExt;
use tower_http::services::ServeDir;

use settings::Settings;
use stats::{Event, Limiter, Stats};

const DEFAULT_ADDR: &str = "0.0.0.0:8080";
const DEFAULT_ADMIN_ADDR: &str = "0.0.0.0:8081";

fn env(key: &str, default: &str) -> String {
    std::env::var(key).ok().filter(|v| !v.trim().is_empty()).unwrap_or_else(|| default.to_string())
}

/// Shared state of both servers.
pub struct Zustand {
    pub public: PathBuf,
    /// Folder for settings and statistics (none: nothing is kept).
    pub daten: Option<PathBuf>,
    pub settings: RwLock<Settings>,
    pub stats: Mutex<Stats>,
    /// The statistics changed since they were last saved.
    pub geaendert: AtomicBool,
    pub limiter: Mutex<Limiter>,
    pub seiten: html::Seiten,
    pub admin_passwort: Option<String>,
    pub sitzungen: Mutex<HashMap<String, Instant>>,
}

impl Zustand {
    pub fn new(public: PathBuf, daten: Option<PathBuf>, admin_passwort: Option<String>) -> Self {
        Self {
            settings: RwLock::new(Settings::load(daten.as_deref())),
            stats: Mutex::new(Stats::load(daten.as_deref())),
            geaendert: AtomicBool::new(false),
            limiter: Mutex::new(Limiter::new(Duration::from_secs(600), 300, 20_000)),
            seiten: html::Seiten::new(public.clone()),
            public,
            daten,
            admin_passwort,
            sitzungen: Mutex::new(HashMap::new()),
        }
    }

    /// Saves the statistics if they changed.
    pub fn sichern(&self) {
        if let Some(dir) = &self.daten
            && self.geaendert.swap(false, Ordering::Relaxed) {
                let s = self.stats.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                if let Err(e) = s.save(dir) {
                    eprintln!("nrzLearn: cannot save statistics: {e}");
                    self.geaendert.store(true, Ordering::Relaxed);
                }
            }
    }
}

/// Fingerprinted assets never change under the same name.
async fn cache_forever(mut res: Response) -> Response {
    let ok = res.status().is_success() || res.status() == StatusCode::NOT_MODIFIED;
    let value = if ok { "public, max-age=31536000, immutable" } else { "no-store" };
    res.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static(value));
    res
}

/// Everything else (`index.html`, the icon, …) is checked again on each visit.
async fn revalidate(mut res: Response) -> Response {
    let h = res.headers_mut();
    if !h.contains_key(header::CACHE_CONTROL) {
        h.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-cache"));
    }
    h.insert(header::X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff"));
    res
}

async fn datei(public: &Path, req: Request) -> Response {
    match ServeDir::new(public).precompressed_br().precompressed_gzip().oneshot(req).await {
        Ok(res) => res.map(Body::new),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// Pages, files and the app shell.
async fn seite(State(z): State<Arc<Zustand>>, req: Request) -> Response {
    let pfad = req.uri().path().to_string();
    if pfad.contains("..") || pfad.contains('\\') {
        return StatusCode::NOT_FOUND.into_response();
    }
    let headers = req.headers().clone();
    let s = z.settings.read().unwrap_or_else(std::sync::PoisonError::into_inner).clone();
    let rel = pfad.trim_start_matches('/');
    if rel.is_empty() || rel.ends_with('/') {
        let index = format!("{rel}index.html");
        if z.public.join(&index).is_file() {
            if s.is_neutral() {
                return datei(&z.public, req).await;
            }
            if let Some(f) = z.seiten.seite(&index, &s) {
                return html::antwort(&f, &headers);
            }
        }
    } else if z.public.join(rel).is_file() {
        if rel.ends_with(".html") && !s.is_neutral()
            && let Some(f) = z.seiten.seite(rel, &s) {
                return html::antwort(&f, &headers);
            }
        return datei(&z.public, req).await;
    } else if z.public.join(rel).join("index.html").is_file() {
        return Redirect::permanent(&format!("{pfad}/")).into_response();
    }
    // A route of the app: `/de/python/learn` gets the link preview of `/de/python/`.
    let mut teile = rel.split('/');
    let kurs_seite = match (teile.next(), teile.next()) {
        (Some(l), Some(k)) if !l.is_empty() && !k.is_empty() => Some(format!("{l}/{k}/index.html")).filter(|p| z.public.join(p).is_file()),
        _ => None,
    };
    match z.seiten.app(kurs_seite.as_deref(), &s) {
        Some(f) => html::antwort(&f, &headers),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

/// A finished level from the app (anonymous; see `stats.rs`).
async fn melden(State(z): State<Arc<Zustand>>, headers: HeaderMap, body: Bytes) -> StatusCode {
    let Ok(e) = serde_json::from_slice::<Event>(&body) else { return StatusCode::BAD_REQUEST };
    if !e.is_valid() {
        return StatusCode::BAD_REQUEST;
    }
    let client = headers
        .get("x-forwarded-for")
        .or_else(|| headers.get("x-real-ip"))
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split(',').next())
        .unwrap_or("-")
        .trim()
        .to_string();
    if !z.limiter.lock().unwrap_or_else(std::sync::PoisonError::into_inner).allow(&client) {
        return StatusCode::TOO_MANY_REQUESTS;
    }
    z.stats.lock().unwrap_or_else(std::sync::PoisonError::into_inner).add(&e, &stats::today());
    z.geaendert.store(true, Ordering::Relaxed);
    StatusCode::NO_CONTENT
}

fn app(z: Arc<Zustand>) -> Router {
    let public = z.public.clone();
    // A missing asset is a 404 (not index.html), so a stale page fails loudly
    // instead of trying to run HTML as JavaScript.
    let assets = Router::new()
        .nest_service("/assets", ServeDir::new(public.join("assets")).precompressed_br().precompressed_gzip())
        .layer(map_response(cache_forever));
    // Course texts: a missing translation is a 404, so the app falls back to English.
    let content = ServeDir::new(public.join("content")).precompressed_br().precompressed_gzip();

    Router::new()
        .route("/healthz", get(|| async { "ok" }))
        .route("/api/stats", post(melden).layer(DefaultBodyLimit::max(512)))
        .merge(assets)
        .nest_service("/content", content)
        .fallback(seite)
        .with_state(z)
        .layer(map_response(revalidate))
}

/// Connects to the configured address and expects `200` from `/healthz`.
fn healthcheck(addr: &str) -> bool {
    let Ok(mut target) = addr.parse::<SocketAddr>() else {
        return false;
    };
    if target.ip().is_unspecified() {
        let ip = if target.is_ipv4() { IpAddr::V4(Ipv4Addr::LOCALHOST) } else { IpAddr::V6(Ipv6Addr::LOCALHOST) };
        target.set_ip(ip);
    }
    let Ok(mut stream) = TcpStream::connect_timeout(&target, Duration::from_secs(3)) else {
        return false;
    };
    let _ = stream.set_read_timeout(Some(Duration::from_secs(3)));
    let request = b"GET /healthz HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
    let mut status = [0u8; 12];
    stream.write_all(request).is_ok() && stream.read_exact(&mut status).is_ok() && status.ends_with(b" 200")
}

async fn shutdown() {
    let ctrl_c = async {
        let _ = tokio::signal::ctrl_c().await;
    };
    #[cfg(unix)]
    let term = async {
        if let Ok(mut s) = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            s.recv().await;
        }
    };
    #[cfg(not(unix))]
    let term = std::future::pending::<()>();
    tokio::select! {
        () = ctrl_c => {}
        () = term => {}
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let addr = env("NRZLEARN_ADDR", DEFAULT_ADDR);
    if std::env::args().nth(1).as_deref() == Some("healthcheck") {
        std::process::exit(if healthcheck(&addr) { 0 } else { 1 });
    }

    let public = PathBuf::from(env("NRZLEARN_PUBLIC", "public"));
    if !public.join("app.html").is_file() {
        eprintln!("nrzLearn: no app.html in {} (set NRZLEARN_PUBLIC to the built web app)", public.display());
        std::process::exit(1);
    }
    let daten = std::env::var("NRZLEARN_DATA").ok().filter(|v| !v.trim().is_empty()).map(PathBuf::from);
    if let Some(d) = &daten
        && let Err(e) = std::fs::create_dir_all(d) {
            eprintln!("nrzLearn: cannot create {}: {e}", d.display());
        }
    let passwort = std::env::var("NRZLEARN_ADMIN_PASSWORD").ok().filter(|v| !v.trim().is_empty());
    let z = Arc::new(Zustand::new(public.clone(), daten, passwort));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap_or_else(|e| {
        eprintln!("nrzLearn: cannot listen on {addr}: {e}");
        std::process::exit(1);
    });
    println!("nrzLearn on http://{addr} (app: {})", public.display());

    let (stop_tx, stop_rx) = tokio::sync::watch::channel(false);
    let warten = |mut rx: tokio::sync::watch::Receiver<bool>| async move {
        let _ = rx.wait_for(|v| *v).await;
    };

    if z.admin_passwort.is_some() {
        let admin_addr = env("NRZLEARN_ADMIN_ADDR", DEFAULT_ADMIN_ADDR);
        match tokio::net::TcpListener::bind(&admin_addr).await {
            Ok(l) => {
                println!("nrzLearn admin on http://{admin_addr}");
                let router = admin::router(Arc::clone(&z));
                let rx = stop_rx.clone();
                tokio::spawn(async move {
                    let _ = axum::serve(l, router).with_graceful_shutdown(warten(rx)).await;
                });
            }
            Err(e) => eprintln!("nrzLearn: admin area off, cannot listen on {admin_addr}: {e}"),
        }
    } else {
        println!("nrzLearn: admin area off (NRZLEARN_ADMIN_PASSWORD is not set)");
    }

    // Save the statistics every 30 seconds when they changed.
    let zs = Arc::clone(&z);
    tokio::spawn(async move {
        let mut t = tokio::time::interval(Duration::from_secs(30));
        loop {
            t.tick().await;
            zs.sichern();
        }
    });

    tokio::spawn(async move {
        shutdown().await;
        let _ = stop_tx.send(true);
    });
    axum::serve(listener, app(Arc::clone(&z))).with_graceful_shutdown(warten(stop_rx)).await.expect("server error");
    z.sichern();
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Request;

    /// A small fake build: landing and course page, app shell, one
    /// fingerprinted asset (plain and pre-compressed) and an icon.
    fn public_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("nrzlearn-test-{}-{name}", std::process::id()));
        std::fs::create_dir_all(dir.join("assets")).unwrap();
        std::fs::write(dir.join("index.html"), "<!doctype html><head><title>landing</title></head>").unwrap();
        std::fs::write(dir.join("app.html"), "<!doctype html><head><!--meta--><meta property=\"og:image\" content=\"/og.png\"><!--/meta--><title>app</title></head>").unwrap();
        std::fs::create_dir_all(dir.join("de/python")).unwrap();
        std::fs::write(dir.join("de/python/index.html"), "<!doctype html><head><!--meta--><title>Python lernen</title><meta property=\"og:image\" content=\"/og/python.png\"><!--/meta--></head>").unwrap();
        std::fs::create_dir_all(dir.join("de/privacy")).unwrap();
        std::fs::write(dir.join("de/privacy/index.html"), "<!doctype html><head></head><div data-analytics hidden>auf {host}</div>").unwrap();
        std::fs::create_dir_all(dir.join("content/python")).unwrap();
        std::fs::write(dir.join("content/python/en.json"), "{}").unwrap();
        std::fs::write(dir.join("assets/app-abc123.js"), "console.log(1)").unwrap();
        std::fs::write(dir.join("assets/app-abc123.js.gz"), "gzipped").unwrap();
        std::fs::write(dir.join("favicon.ico"), "icon").unwrap();
        dir
    }

    fn zustand(name: &str, passwort: Option<&str>) -> Arc<Zustand> {
        Arc::new(Zustand::new(public_dir(name), None, passwort.map(str::to_string)))
    }

    async fn get(router: &Router, path: &str, accept_gzip: bool) -> Response {
        let mut req = Request::builder().uri(path);
        if accept_gzip {
            req = req.header(header::ACCEPT_ENCODING, "gzip");
        }
        router.clone().oneshot(req.body(Body::empty()).unwrap()).await.unwrap()
    }

    async fn text(res: Response) -> String {
        String::from_utf8_lossy(&axum::body::to_bytes(res.into_body(), 100_000).await.unwrap()).to_string()
    }

    fn cache(res: &Response) -> &str {
        res.headers().get(header::CACHE_CONTROL).and_then(|v| v.to_str().ok()).unwrap_or("")
    }

    #[tokio::test]
    async fn serves_routes_assets_and_404s() {
        let router = app(zustand("routes", None));

        // Static pages and routes of the single-page app.
        for (path, titel) in [("/", "landing"), ("/de/python/", "Python lernen"), ("/de/python/learn", "Python lernen"), ("/world/3", "app")] {
            let res = get(&router, path, false).await;
            assert_eq!(res.status(), StatusCode::OK, "{path}");
            assert_eq!(cache(&res), "no-cache", "{path}");
            assert!(text(res).await.contains(titel), "{path}");
        }
        // App pages of a course carry the link preview of the course page.
        let app_seite = text(get(&router, "/de/python/play/python:hello-p1", false).await).await;
        assert!(app_seite.contains("og/python.png") && !app_seite.contains("content=\"/og.png\""));
        // Course texts: present → JSON, missing translation → 404 (not the app shell).
        assert_eq!(get(&router, "/content/python/en.json", false).await.status(), StatusCode::OK);
        let res = get(&router, "/content/python/xx.json", false).await;
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
        assert_eq!(cache(&res), "no-cache");

        // A directory without slash is redirected to the slash version.
        let res = get(&router, "/de/python", false).await;
        assert!(res.status().is_redirection());

        // Fingerprinted assets: cached for a year, pre-compressed when accepted.
        let res = get(&router, "/assets/app-abc123.js", true).await;
        assert_eq!(res.status(), StatusCode::OK);
        assert!(cache(&res).contains("immutable"));
        assert_eq!(res.headers().get(header::CONTENT_ENCODING).unwrap(), "gzip");

        // Missing assets are a 404 and never cached.
        let res = get(&router, "/assets/gone-999.wasm", false).await;
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
        assert_eq!(cache(&res), "no-store");

        // Other files are revalidated.
        let res = get(&router, "/favicon.ico", false).await;
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(cache(&res), "no-cache");
        assert_eq!(res.headers().get(header::X_CONTENT_TYPE_OPTIONS).unwrap(), "nosniff");

        assert_eq!(get(&router, "/healthz", false).await.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn adds_analytics_to_every_page() {
        let z = zustand("analytics", None);
        *z.settings.write().unwrap() = Settings { analytics_enabled: true, analytics_src: "https://a.example.org/js/script.js".into(), analytics_domain: "learn.example.org".into(), ..Settings::default() };
        let router = app(Arc::clone(&z));
        for path in ["/", "/de/python/", "/de/python/learn", "/de/privacy/"] {
            let res = get(&router, path, true).await;
            assert_eq!(res.headers().get(header::CONTENT_ENCODING).unwrap(), "gzip", "{path}");
            let etag = res.headers().get(header::ETAG).unwrap().clone();
            let plain = text(get(&router, path, false).await).await;
            assert!(plain.contains("data-domain=\"learn.example.org\""), "{path}");
            let again = router.clone().oneshot(Request::builder().uri(path).header(header::IF_NONE_MATCH, etag).body(Body::empty()).unwrap()).await.unwrap();
            assert_eq!(again.status(), StatusCode::NOT_MODIFIED, "{path}");
        }
        assert!(text(get(&router, "/de/privacy/", false).await).await.contains("<div data-analytics>auf a.example.org</div>"));
    }

    #[tokio::test]
    async fn counts_finished_levels() {
        let z = zustand("stats", None);
        let router = app(Arc::clone(&z));
        let post = |body: &'static str| Request::builder().method("POST").uri("/api/stats").header(header::CONTENT_TYPE, "text/plain").body(Body::from(body)).unwrap();
        let res = router.clone().oneshot(post(r#"{"c":"python","l":"de","k":"lesson","f":true,"s":true}"#)).await.unwrap();
        assert_eq!(res.status(), StatusCode::NO_CONTENT);
        for bad in [r#"{"c":"python","l":"xx","k":"lesson"}"#, r#"{"c":"nope","l":"de","k":"lesson"}"#, "not json"] {
            assert_eq!(router.clone().oneshot(post(bad)).await.unwrap().status(), StatusCode::BAD_REQUEST, "{bad}");
        }
        let s = z.stats.lock().unwrap();
        assert_eq!((s.total.levels, s.total.lessons, s.total.starts), (1, 1, 1));
        assert_eq!(s.courses["python"].starts, 1);
    }

    #[tokio::test]
    async fn admin_needs_the_password() {
        let z = zustand("admin", Some("geheim"));
        let router = admin::router(Arc::clone(&z));
        assert!(text(get(&router, "/", false).await).await.contains("type=\"password\""));
        let login = |pw: &str| Request::builder().method("POST").uri("/login").header(header::CONTENT_TYPE, "application/x-www-form-urlencoded").body(Body::from(format!("password={pw}"))).unwrap();
        let res = router.clone().oneshot(login("falsch")).await.unwrap();
        assert!(text(res).await.contains("stimmen nicht"));
        let res = router.clone().oneshot(login("geheim")).await.unwrap();
        let cookie = res.headers().get(header::SET_COOKIE).unwrap().to_str().unwrap().split(';').next().unwrap().to_string();
        let res = router.clone().oneshot(Request::builder().uri("/").header(header::COOKIE, cookie.clone()).body(Body::empty()).unwrap()).await.unwrap();
        assert!(text(res).await.contains("Beliebteste Programmiersprachen"));
        // Saving settings needs the session and valid values.
        let form = "analytics_enabled=1&analytics_src=https%3A%2F%2Fa.example.org%2Fjs%2Fscript.js&analytics_domain=learn.example.org&google=abc";
        let res = router.clone().oneshot(Request::builder().method("POST").uri("/settings").header(header::COOKIE, cookie).header(header::CONTENT_TYPE, "application/x-www-form-urlencoded").body(Body::from(form)).unwrap()).await.unwrap();
        assert!(res.status().is_redirection());
        let s = z.settings.read().unwrap();
        assert!(s.analytics_enabled && s.google == "abc");
    }

    #[test]
    fn healthcheck_fails_without_server() {
        assert!(!healthcheck("127.0.0.1:1"));
        assert!(!healthcheck("not an address"));
    }
}
