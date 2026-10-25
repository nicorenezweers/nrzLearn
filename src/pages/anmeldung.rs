//! Login and registration.

use dioxus::prelude::*;
use icons::{Flame, Map, Trophy, Zap};
use nrzlearn_shared::{Credentials, PASSWORD_MIN, valid_username};

use crate::api;
use crate::components::sprachwahl::Sprachwahl;
use crate::components::ui::ThemeToggle;
use crate::i18n::{fehlertext, use_sprache_signal};
use crate::konto::{angemeldet, use_konto};

const FELD: &str = "h-12 w-full rounded-xl border-2 bg-card px-4 outline-none transition focus:border-foreground";

#[component]
pub fn Anmeldung() -> Element {
    let konto = use_konto();
    let sprache = use_sprache_signal();
    let l = sprache();
    let status = konto.read().status().cloned().unwrap_or_default();
    let mut registrieren = use_signal(|| false);
    let mut name = use_signal(String::new);
    let mut passwort = use_signal(String::new);
    let mut fehler = use_signal(|| None::<String>);
    let mut sendet = use_signal(|| false);

    let reg = registrieren() && status.registration_open;
    let bereit = if reg { valid_username(name().trim()) && passwort().chars().count() >= PASSWORD_MIN } else { !name().trim().is_empty() && !passwort().is_empty() };

    let mut absenden = move || {
        if sendet() {
            return;
        }
        sendet.set(true);
        fehler.set(None);
        let c = Credentials { username: name().trim().to_string(), password: passwort(), lang: Some(sprache().code().into()) };
        spawn(async move {
            let r = if reg { api::registrieren(&c).await } else { api::anmelden(&c).await };
            match r {
                Ok(me) => angemeldet(konto, sprache, me),
                Err(e) => fehler.set(Some(e)),
            }
            sendet.set(false);
        });
    };

    rsx! {
        div { class: "flex min-h-screen flex-col",
            header { class: "flex items-center justify-end gap-1 p-4",
                Sprachwahl {}
                ThemeToggle {}
            }
            div { class: "flex flex-1 items-center justify-center px-4 pb-16",
                div { class: "grid w-full max-w-4xl items-center gap-10 md:grid-cols-2",
                    div { class: "hidden md:block",
                        div { class: "flex size-14 items-center justify-center rounded-2xl bg-foreground text-3xl font-black text-background", "n" }
                        h1 { class: "mt-6 text-4xl font-extrabold tracking-tight", "{status.instance_name}" }
                        p { class: "mt-3 text-lg text-muted-foreground",
                            {tr!(l, "Learn Rust step by step – in short levels, with real code.", "Lerne Rust Schritt für Schritt – in kurzen Levels, mit echtem Code.", "Aprende Rust paso a paso: niveles cortos y código real.")}
                        }
                        ul { class: "mt-8 space-y-3",
                            li { class: "flex items-center gap-3", span { class: "flex size-9 items-center justify-center rounded-xl border-2", Map { class: "size-4" } } {tr!(l, "100 levels across 16 worlds", "100 Level in 16 Welten", "100 niveles en 16 mundos")} }
                            li { class: "flex items-center gap-3", span { class: "flex size-9 items-center justify-center rounded-xl border-2", Zap { class: "size-4" } } {tr!(l, "New exercises every round", "Jede Runde neue Aufgaben", "Ejercicios nuevos en cada ronda")} }
                            li { class: "flex items-center gap-3", span { class: "flex size-9 items-center justify-center rounded-xl border-2", Flame { class: "size-4" } } {tr!(l, "Streaks, stars and XP", "Serien, Sterne und XP", "Rachas, estrellas y XP")} }
                            li { class: "flex items-center gap-3", span { class: "flex size-9 items-center justify-center rounded-xl border-2", Trophy { class: "size-4" } } {tr!(l, "A leaderboard for this instance", "Eine Rangliste für diese Instanz", "Una clasificación para esta instancia")} }
                        }
                    }
                    div { class: "rounded-3xl border-2 p-6 sm:p-8",
                        div { class: "mb-6 md:hidden",
                            div { class: "flex size-12 items-center justify-center rounded-2xl bg-foreground text-2xl font-black text-background", "n" }
                            div { class: "mt-3 text-2xl font-extrabold", "{status.instance_name}" }
                        }
                        if status.registration_open {
                            div { class: "mb-6 grid grid-cols-2 rounded-xl bg-muted p-1",
                                button {
                                    class: if !reg { "rounded-lg bg-background py-2 text-sm font-bold shadow-sm" } else { "rounded-lg py-2 text-sm text-muted-foreground" },
                                    onclick: move |_| { registrieren.set(false); fehler.set(None); },
                                    {tr!(l, "Log in", "Anmelden", "Iniciar sesión")}
                                }
                                button {
                                    class: if reg { "rounded-lg bg-background py-2 text-sm font-bold shadow-sm" } else { "rounded-lg py-2 text-sm text-muted-foreground" },
                                    onclick: move |_| { registrieren.set(true); fehler.set(None); },
                                    {tr!(l, "Sign up", "Registrieren", "Registrarse")}
                                }
                            }
                        } else {
                            h2 { class: "mb-6 text-xl font-extrabold", {tr!(l, "Log in", "Anmelden", "Iniciar sesión")} }
                        }
                        form {
                            onsubmit: move |e| {
                                e.prevent_default();
                                if bereit { absenden(); }
                            },
                            label { class: "block text-sm font-semibold", {tr!(l, "Username", "Benutzername", "Nombre de usuario")} }
                            input { class: "{FELD} mt-2", name: "username", value: "{name}", autocomplete: "username", autocapitalize: "off", spellcheck: "false", oninput: move |e| name.set(e.value()) }
                            if reg && !name().is_empty() && !valid_username(name().trim()) {
                                p { class: "mt-1 text-xs", {fehlertext(l, nrzlearn_shared::code::INVALID_USERNAME)} }
                            }
                            label { class: "mt-4 block text-sm font-semibold", {tr!(l, "Password", "Passwort", "Contraseña")} }
                            input {
                                class: "{FELD} mt-2",
                                r#type: "password",
                                name: "password",
                                value: "{passwort}",
                                autocomplete: if reg { "new-password" } else { "current-password" },
                                oninput: move |e| passwort.set(e.value()),
                            }
                            if reg {
                                p { class: "mt-1 text-xs text-muted-foreground", {tr!(l, "At least 8 characters.", "Mindestens 8 Zeichen.", "Al menos 8 caracteres.")} }
                            }
                            if let Some(e) = fehler() {
                                p { class: "anim-shake mt-4 rounded-xl border-2 border-foreground p-3 text-sm font-semibold", {fehlertext(l, &e)} }
                            }
                            button {
                                r#type: "submit",
                                class: "mt-6 flex h-12 w-full items-center justify-center rounded-2xl border-2 border-b-4 border-foreground bg-foreground font-bold uppercase tracking-wide text-background transition active:translate-y-0.5 active:border-b-2 disabled:opacity-40",
                                disabled: !bereit || sendet(),
                                if reg { {tr!(l, "Create account", "Konto erstellen", "Crear cuenta")} } else { {tr!(l, "Log in", "Anmelden", "Iniciar sesión")} }
                            }
                        }
                        if !status.registration_open {
                            p { class: "mt-4 text-center text-xs text-muted-foreground",
                                {tr!(l, "Registration is closed. Ask the admin of this instance for an account.", "Die Registrierung ist geschlossen. Frag den Admin dieser Instanz nach einem Konto.", "El registro está cerrado. Pide una cuenta al administrador de esta instancia.")}
                            }
                        }
                    }
                }
            }
        }
    }
}
