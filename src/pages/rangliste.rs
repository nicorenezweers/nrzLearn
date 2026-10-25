//! Leaderboard of the instance.

use dioxus::prelude::*;
use icons::{Crown, Flame, Medal, RotateCcw, Trophy};
use nrzlearn_shared::{LeaderEntry, Leaderboard};

use crate::api;
use crate::i18n::{fehlertext, use_sprache};
use crate::konto::use_konto;

#[component]
pub fn Rangliste() -> Element {
    let l = use_sprache();
    let konto = use_konto();
    let instanz = konto.read().status().map(|s| s.instance_name.clone()).unwrap_or_default();
    let mut daten = use_resource(|| async { api::rangliste().await });

    rsx! {
        div { class: "mx-auto max-w-3xl px-4 py-8",
            div { class: "flex items-center gap-3",
                div { class: "flex size-12 items-center justify-center rounded-2xl bg-foreground text-background", Trophy { class: "size-6" } }
                div { class: "flex-1",
                    h1 { class: "text-3xl font-extrabold tracking-tight", {tr!(l, "Leaderboard", "Rangliste", "Clasificación")} }
                    p { class: "text-muted-foreground", "{instanz}" }
                }
                button { class: "rounded-xl border-2 border-b-4 p-2.5 hover:bg-accent", title: tr!(l, "Refresh", "Aktualisieren", "Actualizar"), onclick: move |_| daten.restart(), RotateCcw { class: "size-4" } }
            }
            match &*daten.read() {
                None => rsx! { div { class: "mt-10 text-center text-muted-foreground", "…" } },
                Some(Err(e)) => rsx! { p { class: "mt-10 text-center", {fehlertext(l, e)} } },
                Some(Ok(b)) => rsx! { Tabelle { brett: b.clone() } },
            }
        }
    }
}

#[component]
fn Tabelle(brett: Leaderboard) -> Element {
    let l = use_sprache();
    let podest: Vec<LeaderEntry> = brett.entries.iter().take(3).cloned().collect();
    // Podium order: 2nd, 1st, 3rd
    let reihenfolge = [1_usize, 0, 2];

    rsx! {
        if brett.entries.len() >= 2 {
            div { class: "mt-8 grid grid-cols-3 items-end gap-3",
                for i in reihenfolge {
                    if let Some(e) = podest.get(i) {
                        div { key: "p{i}", class: "flex flex-col items-center text-center",
                            div { class: if i == 0 { "anim-star mb-2 flex size-16 items-center justify-center rounded-full bg-foreground text-2xl font-extrabold text-background" } else { "mb-2 flex size-12 items-center justify-center rounded-full border-4 border-foreground text-lg font-extrabold" },
                                {e.username.chars().next().map(|c| c.to_uppercase().to_string()).unwrap_or_default()}
                            }
                            div { class: "max-w-full truncate font-bold", "{e.username}" }
                            div { class: "text-sm text-muted-foreground", "{e.xp} XP" }
                            div {
                                class: match i { 0 => "mt-2 flex h-28 w-full items-start justify-center rounded-t-2xl bg-foreground pt-3 text-background", 1 => "mt-2 flex h-20 w-full items-start justify-center rounded-t-2xl bg-muted-foreground pt-3 text-background", _ => "mt-2 flex h-14 w-full items-start justify-center rounded-t-2xl border-2 border-b-0 pt-2" },
                                if i == 0 { Crown { class: "size-6" } } else { Medal { class: "size-5" } }
                            }
                        }
                    }
                }
            }
        }
        div { class: "mt-6 overflow-hidden rounded-2xl border-2",
            div { class: "grid grid-cols-[3rem_1fr_4rem_5rem_4rem] items-center gap-2 border-b-2 bg-muted px-4 py-2 text-xs font-bold uppercase tracking-wide text-muted-foreground",
                span { "#" }
                span { {tr!(l, "Name", "Name", "Nombre")} }
                span { class: "text-right", {tr!(l, "Levels", "Level", "Niveles")} }
                span { class: "text-right", "XP" }
                span { class: "text-right", Flame { class: "ml-auto size-4" } }
            }
            for e in brett.entries.iter() {
                Zeile { key: "{e.rank}", e: e.clone() }
            }
        }
        if let Some(me) = brett.me.clone().filter(|m| !brett.entries.iter().any(|e| e.is_me && e.rank == m.rank)) {
            div { class: "mt-3 overflow-hidden rounded-2xl border-2 border-foreground", Zeile { e: me } }
        }
        p { class: "mt-4 text-center text-sm text-muted-foreground",
            {tr!(l, format!("{} learners on this instance", brett.total_users), format!("{} Lernende auf dieser Instanz", brett.total_users), format!("{} estudiantes en esta instancia", brett.total_users))}
        }
    }
}

#[component]
fn Zeile(e: LeaderEntry) -> Element {
    let l = use_sprache();
    rsx! {
        div { class: if e.is_me { "grid grid-cols-[3rem_1fr_4rem_5rem_4rem] items-center gap-2 bg-foreground/10 px-4 py-3 font-semibold" } else { "grid grid-cols-[3rem_1fr_4rem_5rem_4rem] items-center gap-2 border-t px-4 py-3" },
            span { class: "font-mono font-bold", "{e.rank}" }
            span { class: "flex min-w-0 items-center gap-2",
                span { class: "truncate", "{e.username}" }
                if e.is_me { span { class: "shrink-0 rounded-full bg-foreground px-2 py-0.5 text-[0.65rem] font-bold text-background", {tr!(l, "You", "Du", "Tú")} } }
                span { class: "shrink-0 rounded-md border px-1.5 text-[0.65rem] text-muted-foreground", "Lv {e.level}" }
            }
            span { class: "text-right font-mono", "{e.levels_done}" }
            span { class: "text-right font-mono font-bold", "{e.xp}" }
            span { class: "text-right font-mono", "{e.streak}" }
        }
    }
}
