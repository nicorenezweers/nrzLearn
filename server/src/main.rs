//! nrzLearn server: serves the web app and a small JSON API for accounts,
//! progress, the instance leaderboard and administration.
//!
//! Configuration (environment variables):
//! - `NRZLEARN_ADDR`   listen address, default `0.0.0.0:8080`
//! - `NRZLEARN_DATA`   data directory for the SQLite database, default `./data`
//! - `NRZLEARN_PUBLIC` directory with the built web app, default `./public`
//! - `NRZLEARN_SECURE_COOKIES` set to `true` when served over HTTPS

mod db;

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use nrzlearn_shared::{
    AdminUser, ApiError, Credentials, InstanceSettings, LANGUAGES, LeaderEntry, Leaderboard, Me, NewPassword, PASSWORD_MIN,
    SaveProgress, SetAdmin, SetLanguage, SetupRequest, Status, code, level_for, valid_username,
};
use rand::RngCore;
use sha2::{Digest, Sha256};
use tower_http::compression::CompressionLayer;
use tower_http::services::{ServeDir, ServeFile};

use db::Db;

const COOKIE: &str = "nrzlearn_session";
const SESSION_DAYS: i64 = 30;
const LEADERBOARD_SIZE: usize = 50;

struct AppState {
    db: Db,
    secure_cookies: bool,
}

type Shared = Arc<AppState>;

// ─────────────────────────────── Errors ───────────────────────────────

struct Fail(StatusCode, &'static str);

impl IntoResponse for Fail {
    fn into_response(self) -> Response {
        (self.0, Json(ApiError { error: self.1.into() })).into_response()
    }
}

type ApiResult<T> = Result<T, Fail>;

fn internal<E>(_: E) -> Fail {
    Fail(StatusCode::INTERNAL_SERVER_ERROR, code::INVALID_INPUT)
}

// ─────────────────────────────── Helpers ───────────────────────────────

fn hash_password(password: &str) -> ApiResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default().hash_password(password.as_bytes(), &salt).map(|h| h.to_string()).map_err(internal)
}

fn verify_password(password: &str, hash: &str) -> bool {
    PasswordHash::new(hash).is_ok_and(|h| Argon2::default().verify_password(password.as_bytes(), &h).is_ok())
}

fn token_hash(token: &str) -> String {
    hex::encode(Sha256::digest(token.as_bytes()))
}

fn clean_lang(lang: &str) -> String {
    if LANGUAGES.contains(&lang) { lang.to_string() } else { "en".to_string() }
}

fn check_password(password: &str) -> ApiResult<()> {
    if password.chars().count() < PASSWORD_MIN || password.len() > 256 {
        return Err(Fail(StatusCode::BAD_REQUEST, code::WEAK_PASSWORD));
    }
    Ok(())
}

fn cookie_token(headers: &HeaderMap) -> Option<String> {
    headers
        .get_all(header::COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .flat_map(|v| v.split(';'))
        .find_map(|c| c.trim().strip_prefix(&format!("{COOKIE}=")).map(str::to_string))
}

fn session_cookie(state: &AppState, token: &str, max_age: i64) -> HeaderValue {
    let secure = if state.secure_cookies { "; Secure" } else { "" };
    HeaderValue::from_str(&format!("{COOKIE}={token}; Path=/; HttpOnly; SameSite=Lax; Max-Age={max_age}{secure}"))
        .unwrap_or_else(|_| HeaderValue::from_static(""))
}

/// Creates a session and returns the `Set-Cookie` header for it.
fn login_response(state: &AppState, user_id: i64) -> ApiResult<Response> {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    let token = hex::encode(bytes);
    let max_age = SESSION_DAYS * 86_400;
    state.db.create_session(&token_hash(&token), user_id, db::now() + max_age).map_err(internal)?;
    let me = me_for(state, user_id)?;
    let mut res = Json(me).into_response();
    res.headers_mut().insert(header::SET_COOKIE, session_cookie(state, &token, max_age));
    Ok(res)
}

fn current_user(state: &AppState, headers: &HeaderMap) -> ApiResult<db::User> {
    cookie_token(headers)
        .and_then(|t| state.db.session_user(&token_hash(&t)))
        .and_then(|id| state.db.user_by_id(id))
        .ok_or(Fail(StatusCode::UNAUTHORIZED, code::UNAUTHORIZED))
}

fn current_admin(state: &AppState, headers: &HeaderMap) -> ApiResult<db::User> {
    let user = current_user(state, headers)?;
    if user.is_admin { Ok(user) } else { Err(Fail(StatusCode::FORBIDDEN, code::FORBIDDEN)) }
}

fn me_for(state: &AppState, user_id: i64) -> ApiResult<Me> {
    let u = state.db.user_by_id(user_id).ok_or(Fail(StatusCode::UNAUTHORIZED, code::UNAUTHORIZED))?;
    Ok(Me { id: u.id, username: u.username, is_admin: u.is_admin, lang: u.lang, progress: state.db.progress(user_id) })
}

fn settings(state: &AppState) -> InstanceSettings {
    InstanceSettings {
        instance_name: state.db.setting("instance_name").unwrap_or_else(|| "nrzLearn".into()),
        registration_open: state.db.setting("registration_open").is_none_or(|v| v == "true"),
        default_lang: state.db.setting("default_lang").unwrap_or_else(|| "en".into()),
    }
}

/// Slows down failed logins a little to make guessing expensive.
async fn penalty() {
    tokio::time::sleep(Duration::from_millis(400)).await;
}

// ─────────────────────────────── Handlers ───────────────────────────────

async fn status(State(s): State<Shared>) -> Json<Status> {
    let set = settings(&s);
    Json(Status {
        setup_needed: s.db.admin_count() == 0,
        instance_name: set.instance_name,
        registration_open: set.registration_open,
        default_lang: set.default_lang,
    })
}

async fn setup(State(s): State<Shared>, Json(req): Json<SetupRequest>) -> ApiResult<Response> {
    if s.db.admin_count() > 0 {
        return Err(Fail(StatusCode::CONFLICT, code::SETUP_DONE));
    }
    let name = req.username.trim();
    if !valid_username(name) {
        return Err(Fail(StatusCode::BAD_REQUEST, code::INVALID_USERNAME));
    }
    check_password(&req.password)?;
    let instance = req.instance_name.trim();
    let instance = if instance.is_empty() { "nrzLearn" } else { instance };
    let lang = clean_lang(&req.default_lang);
    s.db.set_setting("instance_name", &instance.chars().take(60).collect::<String>()).map_err(internal)?;
    s.db.set_setting("registration_open", if req.registration_open { "true" } else { "false" }).map_err(internal)?;
    s.db.set_setting("default_lang", &lang).map_err(internal)?;
    let hash = hash_password(&req.password)?;
    let id = s.db.create_user(name, &hash, true, &lang).ok_or(Fail(StatusCode::CONFLICT, code::USERNAME_TAKEN))?;
    login_response(&s, id)
}

async fn register(State(s): State<Shared>, Json(req): Json<Credentials>) -> ApiResult<Response> {
    if s.db.admin_count() == 0 || !settings(&s).registration_open {
        return Err(Fail(StatusCode::FORBIDDEN, code::REGISTRATION_CLOSED));
    }
    let name = req.username.trim();
    if !valid_username(name) {
        return Err(Fail(StatusCode::BAD_REQUEST, code::INVALID_USERNAME));
    }
    check_password(&req.password)?;
    let lang = clean_lang(req.lang.as_deref().unwrap_or(&settings(&s).default_lang));
    let hash = hash_password(&req.password)?;
    let id = s.db.create_user(name, &hash, false, &lang).ok_or(Fail(StatusCode::CONFLICT, code::USERNAME_TAKEN))?;
    login_response(&s, id)
}

async fn login(State(s): State<Shared>, Json(req): Json<Credentials>) -> ApiResult<Response> {
    let user = s.db.user_by_name(req.username.trim());
    match user {
        Some(u) if verify_password(&req.password, &u.password_hash) => login_response(&s, u.id),
        _ => {
            penalty().await;
            Err(Fail(StatusCode::UNAUTHORIZED, code::INVALID_CREDENTIALS))
        }
    }
}

async fn logout(State(s): State<Shared>, headers: HeaderMap) -> Response {
    if let Some(t) = cookie_token(&headers) {
        let _ = s.db.delete_session(&token_hash(&t));
    }
    let mut res = StatusCode::NO_CONTENT.into_response();
    res.headers_mut().insert(header::SET_COOKIE, session_cookie(&s, "", 0));
    res
}

async fn me(State(s): State<Shared>, headers: HeaderMap) -> ApiResult<Json<Me>> {
    let u = current_user(&s, &headers)?;
    me_for(&s, u.id).map(Json)
}

async fn set_language(State(s): State<Shared>, headers: HeaderMap, Json(req): Json<SetLanguage>) -> ApiResult<StatusCode> {
    let u = current_user(&s, &headers)?;
    s.db.set_lang(u.id, &clean_lang(&req.lang)).map_err(internal)?;
    Ok(StatusCode::NO_CONTENT)
}

async fn save_progress(State(s): State<Shared>, headers: HeaderMap, Json(req): Json<SaveProgress>) -> ApiResult<StatusCode> {
    let u = current_user(&s, &headers)?;
    if req.progress.len() > 512 * 1024 {
        return Err(Fail(StatusCode::PAYLOAD_TOO_LARGE, code::INVALID_INPUT));
    }
    let v: serde_json::Value = serde_json::from_str(&req.progress).map_err(|_| Fail(StatusCode::BAD_REQUEST, code::INVALID_INPUT))?;
    let num = |k: &str| v.get(k).and_then(serde_json::Value::as_u64).unwrap_or(0).min(u64::from(u32::MAX)) as u32;
    let levels = v.get("knoten").and_then(serde_json::Value::as_object).map_or(0, |o| o.len() as u32);
    s.db.save_progress(u.id, &req.progress, num("xp"), num("serie"), levels).map_err(internal)?;
    Ok(StatusCode::NO_CONTENT)
}

async fn leaderboard(State(s): State<Shared>, headers: HeaderMap) -> ApiResult<Json<Leaderboard>> {
    let me = current_user(&s, &headers)?;
    let ranking = s.db.ranking();
    let entry = |i: usize, r: &db::Stats| LeaderEntry {
        rank: i as u32 + 1,
        username: r.username.clone(),
        xp: r.xp,
        level: level_for(r.xp),
        streak: r.streak,
        levels_done: r.levels_done,
        is_me: r.user_id == me.id,
    };
    Ok(Json(Leaderboard {
        entries: ranking.iter().take(LEADERBOARD_SIZE).enumerate().map(|(i, r)| entry(i, r)).collect(),
        me: ranking.iter().enumerate().find(|(_, r)| r.user_id == me.id).map(|(i, r)| entry(i, r)),
        total_users: ranking.len() as u32,
    }))
}

async fn admin_users(State(s): State<Shared>, headers: HeaderMap) -> ApiResult<Json<Vec<AdminUser>>> {
    current_admin(&s, &headers)?;
    Ok(Json(s.db.admin_users()))
}

async fn admin_get_settings(State(s): State<Shared>, headers: HeaderMap) -> ApiResult<Json<InstanceSettings>> {
    current_admin(&s, &headers)?;
    Ok(Json(settings(&s)))
}

async fn admin_put_settings(State(s): State<Shared>, headers: HeaderMap, Json(req): Json<InstanceSettings>) -> ApiResult<Json<InstanceSettings>> {
    current_admin(&s, &headers)?;
    let name = req.instance_name.trim();
    if name.is_empty() {
        return Err(Fail(StatusCode::BAD_REQUEST, code::INVALID_INPUT));
    }
    s.db.set_setting("instance_name", &name.chars().take(60).collect::<String>()).map_err(internal)?;
    s.db.set_setting("registration_open", if req.registration_open { "true" } else { "false" }).map_err(internal)?;
    s.db.set_setting("default_lang", &clean_lang(&req.default_lang)).map_err(internal)?;
    Ok(Json(settings(&s)))
}

async fn admin_set_admin(State(s): State<Shared>, headers: HeaderMap, Path(id): Path<i64>, Json(req): Json<SetAdmin>) -> ApiResult<StatusCode> {
    let admin = current_admin(&s, &headers)?;
    let target = s.db.user_by_id(id).ok_or(Fail(StatusCode::NOT_FOUND, code::NOT_FOUND))?;
    if !req.is_admin && target.is_admin && (s.db.admin_count() <= 1 || target.id == admin.id) {
        return Err(Fail(StatusCode::CONFLICT, code::LAST_ADMIN));
    }
    s.db.set_admin(id, req.is_admin).map_err(internal)?;
    Ok(StatusCode::NO_CONTENT)
}

async fn admin_set_password(State(s): State<Shared>, headers: HeaderMap, Path(id): Path<i64>, Json(req): Json<NewPassword>) -> ApiResult<StatusCode> {
    current_admin(&s, &headers)?;
    check_password(&req.password)?;
    let hash = hash_password(&req.password)?;
    if s.db.set_password(id, &hash).map_err(internal)? == 0 {
        return Err(Fail(StatusCode::NOT_FOUND, code::NOT_FOUND));
    }
    Ok(StatusCode::NO_CONTENT)
}

async fn admin_delete_user(State(s): State<Shared>, headers: HeaderMap, Path(id): Path<i64>) -> ApiResult<StatusCode> {
    let admin = current_admin(&s, &headers)?;
    if id == admin.id {
        return Err(Fail(StatusCode::CONFLICT, code::LAST_ADMIN));
    }
    if s.db.delete_user(id).map_err(internal)? == 0 {
        return Err(Fail(StatusCode::NOT_FOUND, code::NOT_FOUND));
    }
    Ok(StatusCode::NO_CONTENT)
}

async fn api_not_found() -> Fail {
    Fail(StatusCode::NOT_FOUND, code::NOT_FOUND)
}

// ─────────────────────────────── Main ───────────────────────────────

/// Stops on Ctrl+C and on SIGTERM (as sent by `docker stop`).
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

#[tokio::main]
async fn main() {
    let addr = std::env::var("NRZLEARN_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".into());
    let data = PathBuf::from(std::env::var("NRZLEARN_DATA").unwrap_or_else(|_| "data".into()));
    let public = PathBuf::from(std::env::var("NRZLEARN_PUBLIC").unwrap_or_else(|_| "public".into()));
    let secure_cookies = std::env::var("NRZLEARN_SECURE_COOKIES").is_ok_and(|v| v == "true" || v == "1");

    std::fs::create_dir_all(&data).expect("cannot create data directory");
    let db = Db::open(&data.join("nrzlearn.db")).expect("cannot open database");
    let state: Shared = Arc::new(AppState { db, secure_cookies });

    let api = Router::new()
        .route("/status", get(status))
        .route("/setup", post(setup))
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/me", get(me))
        .route("/me/lang", put(set_language))
        .route("/progress", put(save_progress))
        .route("/leaderboard", get(leaderboard))
        .route("/admin/users", get(admin_users))
        .route("/admin/settings", get(admin_get_settings).put(admin_put_settings))
        .route("/admin/users/{id}/admin", post(admin_set_admin))
        .route("/admin/users/{id}/password", post(admin_set_password))
        .route("/admin/users/{id}", delete(admin_delete_user))
        .fallback(api_not_found)
        .with_state(state);

    let spa = ServeDir::new(&public).fallback(ServeFile::new(public.join("index.html")));
    let app = Router::new().nest("/api", api).fallback_service(spa).layer(CompressionLayer::new());

    let listener = tokio::net::TcpListener::bind(&addr).await.expect("cannot bind address");
    println!("nrzLearn listening on http://{addr} (data: {}, app: {})", data.display(), public.display());
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown())
        .await
        .expect("server error");
}
