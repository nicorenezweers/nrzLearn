//! Account shell: loads the instance status and the logged-in user and
//! decides whether to show the onboarding, the login or the app.

use dioxus::prelude::*;
use nrzlearn_shared::{Me, Status};

use crate::Route;
use crate::api;
use crate::i18n::{Sprache, use_sprache_signal};
use crate::pages::anmeldung::Anmeldung;
use crate::pages::onboarding::Onboarding;

#[derive(Clone, PartialEq)]
pub enum Konto {
    Laeuft,
    Fehler(String),
    Bereit { status: Status, me: Option<Me> },
}

impl Konto {
    pub fn me(&self) -> Option<&Me> {
        match self {
            Self::Bereit { me, .. } => me.as_ref(),
            _ => None,
        }
    }

    pub fn status(&self) -> Option<&Status> {
        match self {
            Self::Bereit { status, .. } => Some(status),
            _ => None,
        }
    }
}

pub fn use_konto() -> Signal<Konto> {
    use_context()
}

/// Loads status and user. A missing login is not an error.
pub async fn laden(mut konto: Signal<Konto>, mut sprache: Signal<Sprache>) {
    let status = match api::status().await {
        Ok(s) => s,
        Err(e) => {
            konto.set(Konto::Fehler(e));
            return;
        }
    };
    let me = api::me().await.ok();
    // Language: saved user choice, else this browser, else the instance default.
    let l = me
        .as_ref()
        .and_then(|m| Sprache::aus_code(&m.lang))
        .or_else(Sprache::aus_browser)
        .or_else(|| Sprache::aus_code(&status.default_lang))
        .unwrap_or_default();
    sprache.set(l);
    l.merken();
    konto.set(Konto::Bereit { status, me });
}

/// Called after a successful login, registration or onboarding.
pub fn angemeldet(mut konto: Signal<Konto>, mut sprache: Signal<Sprache>, me: Me) {
    if let Some(l) = Sprache::aus_code(&me.lang) {
        sprache.set(l);
        l.merken();
    }
    let status = konto.peek().status().cloned().map(|mut s| {
        s.setup_needed = false;
        s
    });
    if let Some(status) = status {
        konto.set(Konto::Bereit { status, me: Some(me) });
    }
}

pub fn abmelden(mut konto: Signal<Konto>) {
    spawn(async move {
        let _ = api::abmelden().await;
        let status = konto.peek().status().cloned();
        if let Some(status) = status {
            konto.set(Konto::Bereit { status, me: None });
        }
    });
}

#[component]
pub fn Huelle() -> Element {
    let konto = use_konto();
    let sprache = use_sprache_signal();
    let l = sprache();
    use_hook(move || {
        spawn(laden(konto, sprache));
    });

    match konto() {
        Konto::Laeuft => rsx! {
            div { class: "flex min-h-screen items-center justify-center",
                div { class: "anim-bounce flex size-14 items-center justify-center rounded-2xl bg-foreground text-2xl font-black text-background", "n" }
            }
        },
        Konto::Fehler(e) => rsx! {
            div { class: "flex min-h-screen flex-col items-center justify-center gap-4 p-6 text-center",
                div { class: "flex size-14 items-center justify-center rounded-2xl bg-foreground text-2xl font-black text-background", "n" }
                p { class: "text-lg font-semibold", {crate::i18n::fehlertext(l, &e)} }
                button {
                    class: "rounded-xl border-2 border-b-4 px-5 py-2 font-semibold",
                    onclick: move |_| {
                        let mut k = konto;
                        k.set(Konto::Laeuft);
                        spawn(laden(konto, sprache));
                    },
                    {tr!(l, "Try again", "Nochmal versuchen", "Reintentar")}
                }
            }
        },
        Konto::Bereit { status, me } => {
            if status.setup_needed {
                rsx! { Onboarding {} }
            } else if let Some(me) = me {
                rsx! {
                    // One entry list: a new user gets a fresh progress state.
                    for id in std::iter::once(me.id) {
                        AngemeldeteApp { key: "{id}", me: me.clone() }
                    }
                }
            } else {
                rsx! { Anmeldung {} }
            }
        }
    }
}

#[component]
fn AngemeldeteApp(me: Me) -> Element {
    crate::progress::provide_fortschritt(me.progress.as_deref());
    rsx! { Router::<Route> {} }
}
