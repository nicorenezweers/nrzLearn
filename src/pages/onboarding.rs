//! First-run onboarding for the instance admin.

use dioxus::prelude::*;
use icons::{ArrowLeft, ArrowRight, Check, Shield, Trophy, Users};
use nrzlearn_shared::{PASSWORD_MIN, SetupRequest, valid_username};

use crate::api;
use crate::components::effekte::Konfetti;
use crate::i18n::{Sprache, fehlertext, use_sprache_signal};
use crate::konto::{angemeldet, use_konto};

const FELD: &str = "h-12 w-full rounded-xl border-2 bg-card px-4 outline-none transition focus:border-foreground";
const KNOPF: &str = "flex h-12 items-center justify-center gap-2 rounded-2xl border-2 border-b-4 border-foreground bg-foreground px-6 font-bold uppercase tracking-wide text-background transition active:translate-y-0.5 active:border-b-2 disabled:opacity-40";
const KNOPF2: &str = "flex h-12 items-center justify-center gap-2 rounded-2xl border-2 border-b-4 px-6 font-bold uppercase tracking-wide transition hover:bg-accent active:translate-y-0.5 active:border-b-2";

#[component]
pub fn Onboarding() -> Element {
    let konto = use_konto();
    let mut sprache = use_sprache_signal();
    let l = sprache();
    let mut schritt = use_signal(|| 0_usize);
    let mut instanz = use_signal(|| "nrzLearn".to_string());
    let mut offen = use_signal(|| true);
    let mut name = use_signal(String::new);
    let mut passwort = use_signal(String::new);
    let mut passwort2 = use_signal(String::new);
    let mut fehler = use_signal(|| None::<String>);
    let mut sendet = use_signal(|| false);
    let mut fertig = use_signal(|| None::<nrzlearn_shared::Me>);

    let name_ok = valid_username(name().trim());
    let pw_ok = passwort().chars().count() >= PASSWORD_MIN;
    let gleich = passwort() == passwort2();

    let absenden = move |_| {
        sendet.set(true);
        fehler.set(None);
        let req = SetupRequest {
            instance_name: instanz(),
            username: name().trim().to_string(),
            password: passwort(),
            registration_open: offen(),
            default_lang: sprache().code().into(),
        };
        spawn(async move {
            match api::setup(&req).await {
                Ok(me) => fertig.set(Some(me)),
                Err(e) => fehler.set(Some(e)),
            }
            sendet.set(false);
        });
    };

    let s = schritt();
    let titel = [
        tr!(l, "Welcome", "Willkommen", "Bienvenida"),
        tr!(l, "Your instance", "Deine Instanz", "Tu instancia"),
        tr!(l, "Admin account", "Admin-Konto", "Cuenta de administrador"),
    ];

    if let Some(me) = fertig() {
        return rsx! {
            Konfetti {}
            div { class: "flex min-h-screen items-center justify-center px-4",
                div { class: "w-full max-w-md text-center",
                    div { class: "anim-star mx-auto mb-5 flex size-20 items-center justify-center rounded-full bg-foreground text-background", Check { class: "size-10" } }
                    h1 { class: "text-3xl font-extrabold", {tr!(l, "All set!", "Alles bereit!", "¡Todo listo!")} }
                    p { class: "mt-2 text-muted-foreground",
                        {tr!(
                            l,
                            format!("{} is ready. You're logged in as admin “{}”.", instanz(), me.username),
                            format!("{} ist bereit. Du bist als Admin „{}“ angemeldet.", instanz(), me.username),
                            format!("{} está lista. Has iniciado sesión como administrador «{}».", instanz(), me.username)
                        )}
                    }
                    ul { class: "mx-auto mt-6 max-w-sm space-y-2 text-left text-sm",
                        li { class: "flex gap-2", Users { class: "size-4 shrink-0" }
                            {if offen() { tr!(l, "Others can sign up themselves.", "Andere können sich selbst registrieren.", "Otras personas pueden registrarse.") } else { tr!(l, "Registration is closed – you can open it in the admin area.", "Die Registrierung ist geschlossen – du kannst sie im Admin-Bereich öffnen.", "El registro está cerrado; puedes abrirlo en el área de administración.") }}
                        }
                        li { class: "flex gap-2", Trophy { class: "size-4 shrink-0" } {tr!(l, "Everyone on this instance shares one leaderboard.", "Alle auf dieser Instanz teilen eine Rangliste.", "Todos en esta instancia comparten una clasificación.")} }
                        li { class: "flex gap-2", Shield { class: "size-4 shrink-0" } {tr!(l, "Manage users and settings under “Admin”.", "Benutzer und Einstellungen verwaltest du unter „Admin“.", "Gestiona usuarios y ajustes en «Admin».")} }
                    }
                    button { class: "{KNOPF} mx-auto mt-8 w-full", onclick: move |_| angemeldet(konto, sprache, me.clone()),
                        {tr!(l, "Start learning", "Loslernen", "Empezar a aprender")}
                        ArrowRight { class: "size-4" }
                    }
                }
            }
        };
    }

    rsx! {
        div { class: "flex min-h-screen items-center justify-center px-4 py-10",
            div { class: "w-full max-w-md",
                div { class: "mb-8 flex items-center justify-center gap-2",
                    for i in 0..3_usize {
                        div { key: "{i}", class: if i <= s { "h-2 w-10 rounded-full bg-foreground transition-all" } else { "h-2 w-10 rounded-full bg-muted transition-all" } }
                    }
                }
                div { class: "rounded-3xl border-2 p-6 sm:p-8",
                    div { class: "text-xs font-bold uppercase tracking-widest text-muted-foreground",
                        {tr!(l, format!("Setup · step {} of 3", s + 1), format!("Einrichtung · Schritt {} von 3", s + 1), format!("Configuración · paso {} de 3", s + 1))}
                    }
                    h1 { class: "mt-1 text-2xl font-extrabold", "{titel[s]}" }

                    match s {
                        0 => rsx! {
                            div { class: "mx-auto my-6 flex size-16 items-center justify-center rounded-2xl bg-foreground text-3xl font-black text-background", "n" }
                            p { class: "text-muted-foreground",
                                {tr!(
                                    l,
                                    "nrzLearn teaches Rust step by step – with a world map, generated exercises and a leaderboard. Let's set up your instance. You become its admin.",
                                    "nrzLearn bringt Rust Schritt für Schritt bei – mit Weltkarte, generierten Aufgaben und Rangliste. Richten wir deine Instanz ein. Du wirst ihr Admin.",
                                    "nrzLearn enseña Rust paso a paso, con un mapa de mundos, ejercicios generados y una clasificación. Vamos a configurar tu instancia. Tú serás su administrador."
                                )}
                            }
                            div { class: "mt-6 text-sm font-semibold", {tr!(l, "Default language", "Standardsprache", "Idioma predeterminado")} }
                            div { class: "mt-2 grid grid-cols-3 gap-2",
                                for sp in Sprache::ALLE {
                                    button {
                                        key: "{sp.code()}",
                                        class: if sp == l { "rounded-xl border-2 border-b-4 border-foreground bg-foreground px-3 py-2.5 font-semibold text-background" } else { "rounded-xl border-2 border-b-4 px-3 py-2.5 font-semibold hover:bg-accent" },
                                        onclick: move |_| { sprache.set(sp); sp.merken(); },
                                        "{sp.name()}"
                                    }
                                }
                            }
                            button { class: "{KNOPF} mt-8 w-full", onclick: move |_| schritt.set(1),
                                {tr!(l, "Let's go", "Los geht's", "Vamos")}
                                ArrowRight { class: "size-4" }
                            }
                        },
                        1 => rsx! {
                            label { class: "mt-6 block text-sm font-semibold", {tr!(l, "Name of the instance", "Name der Instanz", "Nombre de la instancia")} }
                            input { class: "{FELD} mt-2", value: "{instanz}", maxlength: "60", oninput: move |e| instanz.set(e.value()) }
                            p { class: "mt-1 text-xs text-muted-foreground", {tr!(l, "Shown on the login page, e.g. “Rust Club Zurich”.", "Steht auf der Anmeldeseite, z.B. „Rust-Club Zürich“.", "Aparece en la página de inicio de sesión, p. ej. «Club Rust Madrid».")} }
                            div { class: "mt-6 text-sm font-semibold", {tr!(l, "Who may join?", "Wer darf mitmachen?", "¿Quién puede unirse?")} }
                            div { class: "mt-2 grid gap-2",
                                button {
                                    class: if offen() { "rounded-xl border-2 border-foreground bg-foreground/5 p-3 text-left" } else { "rounded-xl border-2 p-3 text-left hover:bg-accent" },
                                    onclick: move |_| offen.set(true),
                                    div { class: "font-semibold", {tr!(l, "Open registration", "Offene Registrierung", "Registro abierto")} }
                                    div { class: "text-sm text-muted-foreground", {tr!(l, "Anyone who can reach the instance can create an account.", "Jeder, der die Instanz erreicht, kann ein Konto anlegen.", "Cualquiera que acceda a la instancia puede crear una cuenta.")} }
                                }
                                button {
                                    class: if !offen() { "rounded-xl border-2 border-foreground bg-foreground/5 p-3 text-left" } else { "rounded-xl border-2 p-3 text-left hover:bg-accent" },
                                    onclick: move |_| offen.set(false),
                                    div { class: "font-semibold", {tr!(l, "Closed", "Geschlossen", "Cerrado")} }
                                    div { class: "text-sm text-muted-foreground", {tr!(l, "Only you. You can open registration later.", "Nur du. Die Registrierung kannst du später öffnen.", "Solo tú. Puedes abrir el registro más tarde.")} }
                                }
                            }
                            div { class: "mt-8 flex gap-3",
                                button { class: "{KNOPF2}", onclick: move |_| schritt.set(0), ArrowLeft { class: "size-4" } }
                                button { class: "{KNOPF} flex-1", disabled: instanz().trim().is_empty(), onclick: move |_| schritt.set(2),
                                    {tr!(l, "Next", "Weiter", "Siguiente")}
                                    ArrowRight { class: "size-4" }
                                }
                            }
                        },
                        _ => rsx! {
                            label { class: "mt-6 block text-sm font-semibold", {tr!(l, "Username", "Benutzername", "Nombre de usuario")} }
                            input { class: "{FELD} mt-2", value: "{name}", autocomplete: "username", autocapitalize: "off", oninput: move |e| name.set(e.value()) }
                            if !name().is_empty() && !name_ok {
                                p { class: "mt-1 text-xs", {fehlertext(l, nrzlearn_shared::code::INVALID_USERNAME)} }
                            }
                            label { class: "mt-4 block text-sm font-semibold", {tr!(l, "Password", "Passwort", "Contraseña")} }
                            input { class: "{FELD} mt-2", r#type: "password", value: "{passwort}", autocomplete: "new-password", oninput: move |e| passwort.set(e.value()) }
                            p { class: "mt-1 text-xs text-muted-foreground", {tr!(l, "At least 8 characters.", "Mindestens 8 Zeichen.", "Al menos 8 caracteres.")} }
                            label { class: "mt-4 block text-sm font-semibold", {tr!(l, "Repeat password", "Passwort wiederholen", "Repite la contraseña")} }
                            input { class: "{FELD} mt-2", r#type: "password", value: "{passwort2}", autocomplete: "new-password", oninput: move |e| passwort2.set(e.value()) }
                            if !passwort2().is_empty() && !gleich {
                                p { class: "mt-1 text-xs", {tr!(l, "The passwords don't match.", "Die Passwörter stimmen nicht überein.", "Las contraseñas no coinciden.")} }
                            }
                            if let Some(e) = fehler() {
                                p { class: "anim-shake mt-4 rounded-xl border-2 border-foreground p-3 text-sm font-semibold", {fehlertext(l, &e)} }
                            }
                            div { class: "mt-8 flex gap-3",
                                button { class: "{KNOPF2}", onclick: move |_| schritt.set(1), ArrowLeft { class: "size-4" } }
                                button { class: "{KNOPF} flex-1", disabled: !(name_ok && pw_ok && gleich) || sendet(), onclick: absenden,
                                    {tr!(l, "Create instance", "Instanz erstellen", "Crear instancia")}
                                }
                            }
                        },
                    }
                }
            }
        }
    }
}
