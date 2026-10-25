//! Client for the nrzLearn server API (`/api/...`). Errors are API error
//! codes (see `nrzlearn_shared::code`) or `"network"`.

use gloo_net::http::{Request, RequestBuilder, Response};
use nrzlearn_shared::{
    AdminUser, ApiError, Credentials, InstanceSettings, Leaderboard, Me, NewPassword, SaveProgress, SetAdmin, SetLanguage,
    SetupRequest, Status,
};
use serde::Serialize;
use serde::de::DeserializeOwned;

pub type Ergebnis<T> = Result<T, String>;

async fn antwort(res: Result<Response, gloo_net::Error>) -> Ergebnis<Response> {
    let res = res.map_err(|_| "network".to_string())?;
    if res.ok() {
        Ok(res)
    } else {
        Err(res.json::<ApiError>().await.map_or_else(|_| format!("http_{}", res.status()), |e| e.error))
    }
}

async fn json<T: DeserializeOwned>(req: RequestBuilder) -> Ergebnis<T> {
    antwort(req.send().await).await?.json::<T>().await.map_err(|_| "network".into())
}

async fn senden<B: Serialize, T: DeserializeOwned>(req: RequestBuilder, body: &B) -> Ergebnis<T> {
    let req = req.json(body).map_err(|_| "network".to_string())?;
    antwort(req.send().await).await?.json::<T>().await.map_err(|_| "network".into())
}

async fn leer<B: Serialize>(req: RequestBuilder, body: &B) -> Ergebnis<()> {
    let req = req.json(body).map_err(|_| "network".to_string())?;
    antwort(req.send().await).await.map(|_| ())
}

pub async fn status() -> Ergebnis<Status> {
    json(Request::get("/api/status")).await
}

pub async fn me() -> Ergebnis<Me> {
    json(Request::get("/api/me")).await
}

pub async fn setup(req: &SetupRequest) -> Ergebnis<Me> {
    senden(Request::post("/api/setup"), req).await
}

pub async fn registrieren(c: &Credentials) -> Ergebnis<Me> {
    senden(Request::post("/api/register"), c).await
}

pub async fn anmelden(c: &Credentials) -> Ergebnis<Me> {
    senden(Request::post("/api/login"), c).await
}

pub async fn abmelden() -> Ergebnis<()> {
    antwort(Request::post("/api/logout").send().await).await.map(|_| ())
}

pub async fn fortschritt_speichern(progress: String) -> Ergebnis<()> {
    leer(Request::put("/api/progress"), &SaveProgress { progress }).await
}

pub async fn sprache_speichern(lang: &str) -> Ergebnis<()> {
    leer(Request::put("/api/me/lang"), &SetLanguage { lang: lang.into() }).await
}

pub async fn rangliste() -> Ergebnis<Leaderboard> {
    json(Request::get("/api/leaderboard")).await
}

pub async fn admin_benutzer() -> Ergebnis<Vec<AdminUser>> {
    json(Request::get("/api/admin/users")).await
}

pub async fn admin_einstellungen() -> Ergebnis<InstanceSettings> {
    json(Request::get("/api/admin/settings")).await
}

pub async fn admin_einstellungen_speichern(s: &InstanceSettings) -> Ergebnis<InstanceSettings> {
    senden(Request::put("/api/admin/settings"), s).await
}

pub async fn admin_rechte(id: i64, is_admin: bool) -> Ergebnis<()> {
    leer(Request::post(&format!("/api/admin/users/{id}/admin")), &SetAdmin { is_admin }).await
}

pub async fn admin_passwort(id: i64, password: String) -> Ergebnis<()> {
    leer(Request::post(&format!("/api/admin/users/{id}/password")), &NewPassword { password }).await
}

pub async fn admin_loeschen(id: i64) -> Ergebnis<()> {
    antwort(Request::delete(&format!("/api/admin/users/{id}")).send().await).await.map(|_| ())
}
