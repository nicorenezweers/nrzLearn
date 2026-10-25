//! Instance administration: settings and users.

use dioxus::prelude::*;
use icons::{Check, KeyRound, Shield, ShieldOff, Trash2, Users};
use nrzlearn_shared::{AdminUser, InstanceSettings, PASSWORD_MIN};

use crate::api;
use crate::i18n::{Sprache, fehlertext, use_sprache};
use crate::konto::{Konto, use_konto};

const FELD: &str = "h-11 w-full rounded-xl border-2 bg-card px-3 outline-none transition focus:border-foreground";
const KNOPF: &str = "inline-flex h-10 items-center justify-center gap-2 rounded-xl border-2 border-b-4 border-foreground bg-foreground px-4 text-sm font-bold text-background transition active:translate-y-0.5 active:border-b-2 disabled:opacity-40";

#[component]
pub fn Admin() -> Element {
    let l = use_sprache();
    let konto = use_konto();
    if !konto.read().me().is_some_and(|m| m.is_admin) {
        return rsx! { p { class: "py-20 text-center", {fehlertext(l, nrzlearn_shared::code::FORBIDDEN)} } };
    }
    rsx! {
        div { class: "mx-auto max-w-4xl space-y-8 px-4 py-8",
            div { class: "flex items-center gap-3",
                div { class: "flex size-12 items-center justify-center rounded-2xl bg-foreground text-background", Shield { class: "size-6" } }
                h1 { class: "text-3xl font-extrabold tracking-tight", {tr!(l, "Administration", "Verwaltung", "Administración")} }
            }
            Einstellungen {}
            Benutzerliste {}
        }
    }
}

#[component]
fn Einstellungen() -> Element {
    let l = use_sprache();
    let mut konto = use_konto();
    let mut form = use_signal(|| None::<InstanceSettings>);
    let mut meldung = use_signal(|| None::<Result<(), String>>);
    use_hook(move || {
        spawn(async move {
            if let Ok(s) = api::admin_einstellungen().await {
                form.set(Some(s));
            }
        });
    });
    let Some(f) = form() else {
        return rsx! { div { class: "rounded-2xl border-2 p-6 text-muted-foreground", "…" } };
    };

    let speichern = move |_| {
        let Some(s) = form() else { return };
        spawn(async move {
            match api::admin_einstellungen_speichern(&s).await {
                Ok(neu) => {
                    // Update the cached status so headers and login show the new name.
                    if let Konto::Bereit { status, me } = konto() {
                        let mut status = status;
                        status.instance_name = neu.instance_name.clone();
                        status.registration_open = neu.registration_open;
                        status.default_lang = neu.default_lang.clone();
                        konto.set(Konto::Bereit { status, me });
                    }
                    form.set(Some(neu));
                    meldung.set(Some(Ok(())));
                }
                Err(e) => meldung.set(Some(Err(e))),
            }
        });
    };

    rsx! {
        section { class: "rounded-2xl border-2 p-6",
            h2 { class: "text-lg font-bold", {tr!(l, "Instance", "Instanz", "Instancia")} }
            div { class: "mt-4 grid gap-4 sm:grid-cols-2",
                div {
                    label { class: "text-sm font-semibold", {tr!(l, "Name", "Name", "Nombre")} }
                    input {
                        class: "{FELD} mt-1",
                        value: "{f.instance_name}",
                        maxlength: "60",
                        oninput: move |e| form.with_mut(|f| if let Some(f) = f { f.instance_name = e.value() }),
                    }
                }
                div {
                    label { class: "text-sm font-semibold", {tr!(l, "Default language", "Standardsprache", "Idioma predeterminado")} }
                    select {
                        class: "{FELD} mt-1",
                        value: "{f.default_lang}",
                        onchange: move |e| form.with_mut(|f| if let Some(f) = f { f.default_lang = e.value() }),
                        for sp in Sprache::ALLE {
                            option { key: "{sp.code()}", value: "{sp.code()}", selected: f.default_lang == sp.code(), "{sp.name()}" }
                        }
                    }
                }
            }
            label { class: "mt-4 flex cursor-pointer items-center gap-3",
                input {
                    r#type: "checkbox",
                    class: "size-5 accent-[var(--foreground)]",
                    checked: f.registration_open,
                    onchange: move |e| form.with_mut(|f| if let Some(f) = f { f.registration_open = e.checked() }),
                }
                span {
                    span { class: "font-semibold", {tr!(l, "Open registration", "Offene Registrierung", "Registro abierto")} }
                    span { class: "block text-sm text-muted-foreground", {tr!(l, "New people can create an account themselves.", "Neue Leute können selbst ein Konto anlegen.", "Las personas nuevas pueden crear una cuenta.")} }
                }
            }
            div { class: "mt-5 flex items-center gap-3",
                button { class: "{KNOPF}", disabled: f.instance_name.trim().is_empty(), onclick: speichern, {tr!(l, "Save", "Speichern", "Guardar")} }
                match meldung() {
                    Some(Ok(())) => rsx! { span { class: "anim-pop flex items-center gap-1 text-sm font-semibold", Check { class: "size-4" } {tr!(l, "Saved", "Gespeichert", "Guardado")} } },
                    Some(Err(e)) => rsx! { span { class: "text-sm", {fehlertext(l, &e)} } },
                    None => rsx! {},
                }
            }
        }
    }
}

#[component]
fn Benutzerliste() -> Element {
    let l = use_sprache();
    let konto = use_konto();
    let ich = konto.read().me().map_or(0, |m| m.id);
    let mut liste = use_resource(|| async { api::admin_benutzer().await });
    let mut fehler = use_signal(|| None::<String>);

    rsx! {
        section { class: "rounded-2xl border-2",
            div { class: "flex items-center gap-2 border-b-2 p-5",
                Users { class: "size-5" }
                h2 { class: "text-lg font-bold", {tr!(l, "Users", "Benutzer", "Usuarios")} }
            }
            if let Some(e) = fehler() {
                p { class: "anim-shake mx-5 mt-4 rounded-xl border-2 border-foreground p-3 text-sm font-semibold", {fehlertext(l, &e)} }
            }
            match &*liste.read() {
                None => rsx! { p { class: "p-5 text-muted-foreground", "…" } },
                Some(Err(e)) => rsx! { p { class: "p-5", {fehlertext(l, e)} } },
                Some(Ok(benutzer)) => rsx! {
                    div { class: "divide-y",
                        for u in benutzer.iter() {
                            BenutzerZeile {
                                key: "{u.id}",
                                u: u.clone(),
                                ich: u.id == ich,
                                on_fertig: move |r: Result<(), String>| {
                                    match r {
                                        Ok(()) => { fehler.set(None); liste.restart(); }
                                        Err(e) => fehler.set(Some(e)),
                                    }
                                },
                            }
                        }
                    }
                },
            }
        }
    }
}

#[component]
fn BenutzerZeile(u: AdminUser, ich: bool, on_fertig: EventHandler<Result<(), String>>) -> Element {
    let l = use_sprache();
    let mut modus = use_signal(|| 0_u8); // 0 = normal, 1 = Passwort, 2 = Löschen bestätigen
    let mut neues = use_signal(String::new);
    let datum = js_sys::Date::new(&js_sys::wasm_bindgen::JsValue::from_f64(u.created_at as f64 * 1000.0));
    let datum = format!("{:04}-{:02}-{:02}", datum.get_full_year(), datum.get_month() + 1, datum.get_date());
    let id = u.id;
    let ist_admin = u.is_admin;

    rsx! {
        div { class: "flex flex-wrap items-center gap-3 px-5 py-3",
            div { class: "flex size-9 items-center justify-center rounded-full border-2 font-bold",
                {u.username.chars().next().map(|c| c.to_uppercase().to_string()).unwrap_or_default()}
            }
            div { class: "min-w-0 flex-1",
                div { class: "flex items-center gap-2 font-semibold",
                    span { class: "truncate", "{u.username}" }
                    if u.is_admin { span { class: "rounded-full bg-foreground px-2 py-0.5 text-[0.65rem] font-bold text-background", "Admin" } }
                    if ich { span { class: "text-xs text-muted-foreground", {tr!(l, "(you)", "(du)", "(tú)")} } }
                }
                div { class: "text-xs text-muted-foreground",
                    {tr!(l, format!("{} XP · {} levels · since {datum}", u.xp, u.levels_done), format!("{} XP · {} Level · seit {datum}", u.xp, u.levels_done), format!("{} XP · {} niveles · desde {datum}", u.xp, u.levels_done))}
                }
            }
            match modus() {
                1 => rsx! {
                    input { class: "{FELD} w-44", r#type: "password", placeholder: tr!(l, "New password", "Neues Passwort", "Nueva contraseña"), value: "{neues}", oninput: move |e| neues.set(e.value()) }
                    button {
                        class: "{KNOPF}",
                        disabled: neues().chars().count() < PASSWORD_MIN,
                        onclick: move |_| {
                            let pw = neues();
                            spawn(async move {
                                let r = api::admin_passwort(id, pw).await;
                                if r.is_ok() { modus.set(0); neues.set(String::new()); }
                                on_fertig.call(r);
                            });
                        },
                        {tr!(l, "Set", "Setzen", "Guardar")}
                    }
                    button { class: "text-sm underline", onclick: move |_| modus.set(0), {tr!(l, "Cancel", "Abbrechen", "Cancelar")} }
                },
                2 => rsx! {
                    span { class: "text-sm font-semibold", {tr!(l, "Delete with all progress?", "Mit allem Fortschritt löschen?", "¿Eliminar con todo el progreso?")} }
                    button {
                        class: "{KNOPF}",
                        onclick: move |_| {
                            spawn(async move {
                                let r = api::admin_loeschen(id).await;
                                on_fertig.call(r);
                            });
                        },
                        {tr!(l, "Delete", "Löschen", "Eliminar")}
                    }
                    button { class: "text-sm underline", onclick: move |_| modus.set(0), {tr!(l, "Cancel", "Abbrechen", "Cancelar")} }
                },
                _ => rsx! {
                    div { class: "flex gap-1",
                        button {
                            class: "rounded-lg p-2 hover:bg-accent disabled:opacity-30",
                            disabled: ich,
                            title: if ist_admin { tr!(l, "Remove admin rights", "Admin-Rechte entziehen", "Quitar permisos de administrador") } else { tr!(l, "Make admin", "Zum Admin machen", "Hacer administrador") },
                            onclick: move |_| {
                                spawn(async move {
                                    let r = api::admin_rechte(id, !ist_admin).await;
                                    on_fertig.call(r);
                                });
                            },
                            if ist_admin { ShieldOff { class: "size-4" } } else { Shield { class: "size-4" } }
                        }
                        button { class: "rounded-lg p-2 hover:bg-accent", title: tr!(l, "Reset password", "Passwort zurücksetzen", "Restablecer contraseña"), onclick: move |_| modus.set(1), KeyRound { class: "size-4" } }
                        button { class: "rounded-lg p-2 hover:bg-accent disabled:opacity-30", disabled: ich, title: tr!(l, "Delete user", "Benutzer löschen", "Eliminar usuario"), onclick: move |_| modus.set(2), Trash2 { class: "size-4" } }
                    }
                },
            }
        }
    }
}
