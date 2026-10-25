use dioxus::prelude::*;
use icons::{Flame, LogOut, Map, Shield, StickyNote, Terminal, Trophy, UserRound, Volume2, VolumeX, Zap};

use crate::Route;
use crate::components::sprachwahl::Sprachwahl;
use crate::components::ui::ThemeToggle;
use crate::i18n::use_sprache;
use crate::konto::{abmelden, use_konto};
use crate::progress::{aendern, use_fortschritt};

#[component]
pub fn Layout() -> Element {
    let fortschritt = use_fortschritt();
    let konto = use_konto();
    let l = use_sprache();
    let route: Route = use_route();
    let f = fortschritt.read();
    let me = konto.read().me().cloned();
    let instanz = konto.read().status().map(|s| s.instance_name.clone()).unwrap_or_default();
    let mut menue = use_signal(|| false);
    let ziel_prozent = (f64::from(f.heute_xp) / f64::from(f.ziel.max(1))).min(1.0);
    // Daily goal ring: circumference 2·π·9 ≈ 56.5
    let ring = 56.5 * (1.0 - ziel_prozent);

    let nav = |aktiv: bool| {
        if aktiv {
            "flex items-center gap-1.5 rounded-xl bg-foreground px-3 py-1.5 text-sm font-semibold text-background"
        } else {
            "flex items-center gap-1.5 rounded-xl px-3 py-1.5 text-sm text-muted-foreground transition hover:bg-accent hover:text-foreground"
        }
    };

    rsx! {
        div { class: "flex min-h-screen flex-col",
            header { class: "sticky top-0 z-40 border-b bg-background/85 backdrop-blur",
                div { class: "mx-auto flex h-14 max-w-6xl items-center gap-1 px-4",
                    Link { to: Route::Home {}, class: "mr-2 flex items-center gap-2 text-lg font-extrabold tracking-tight",
                        span { class: "flex size-8 items-center justify-center rounded-lg bg-foreground text-sm font-black text-background", "n" }
                        span { class: "hidden lg:inline", "nrz" span { class: "font-light", "Learn" } }
                    }
                    nav { class: "flex items-center gap-0.5",
                        Link { to: Route::Home {}, class: nav(matches!(route, Route::Home {} | Route::LektionSeite { .. })), title: tr!(l, "Learning path", "Lernpfad", "Camino"), Map { class: "size-4" } span { class: "hidden md:inline", {tr!(l, "Path", "Lernpfad", "Camino")} } }
                        Link { to: Route::Rangliste {}, class: nav(matches!(route, Route::Rangliste {})), title: tr!(l, "Leaderboard", "Rangliste", "Clasificación"), Trophy { class: "size-4" } span { class: "hidden md:inline", {tr!(l, "Leaderboard", "Rangliste", "Clasificación")} } }
                        Link { to: Route::TerminalSeite {}, class: nav(matches!(route, Route::TerminalSeite {})), title: "Terminal", Terminal { class: "size-4" } span { class: "hidden xl:inline", "Terminal" } }
                        Link { to: Route::Spickzettel {}, class: nav(matches!(route, Route::Spickzettel {})), title: tr!(l, "Cheat sheet", "Spickzettel", "Chuleta"), StickyNote { class: "size-4" } span { class: "hidden xl:inline", {tr!(l, "Cheat sheet", "Spickzettel", "Chuleta")} } }
                        if me.as_ref().is_some_and(|m| m.is_admin) {
                            Link { to: Route::Admin {}, class: nav(matches!(route, Route::Admin {})), title: "Admin", Shield { class: "size-4" } span { class: "hidden xl:inline", "Admin" } }
                        }
                    }
                    div { class: "ml-auto flex items-center gap-0.5 sm:gap-1",
                        div {
                            class: if f.ist_heute_aktiv() { "flex items-center gap-1 rounded-xl px-1.5 py-1 text-sm font-bold" } else { "flex items-center gap-1 rounded-xl px-1.5 py-1 text-sm font-bold text-muted-foreground" },
                            title: tr!(l, "Daily streak", "Tagesserie", "Racha diaria"),
                            Flame { key: "{f.ist_heute_aktiv()}", class: if f.ist_heute_aktiv() { "size-4 fill-foreground" } else { "size-4" } }
                            "{f.serie}"
                        }
                        div { class: "hidden items-center gap-1.5 rounded-xl px-1.5 py-1 text-sm font-bold sm:flex", title: tr!(l, format!("Daily goal: {} of {} XP", f.heute_xp, f.ziel), format!("Tagesziel: {} von {} XP", f.heute_xp, f.ziel), format!("Objetivo diario: {} de {} XP", f.heute_xp, f.ziel)),
                            svg { class: "-rotate-90", width: "22", height: "22", view_box: "0 0 22 22",
                                circle { cx: "11", cy: "11", r: "9", fill: "none", stroke: "var(--muted)", stroke_width: "3" }
                                circle { cx: "11", cy: "11", r: "9", fill: "none", stroke: "var(--foreground)", stroke_width: "3", stroke_linecap: "round", stroke_dasharray: "56.5", stroke_dashoffset: "{ring}" }
                            }
                        }
                        div { class: "flex items-center gap-1 whitespace-nowrap rounded-xl px-1.5 py-1 text-sm font-bold", title: "XP · Level {f.level()}",
                            Zap { class: "size-4 fill-foreground" }
                            "{f.xp}"
                        }
                        Sprachwahl {}
                        button {
                            class: "hidden size-8 items-center justify-center rounded-md transition-colors hover:bg-accent sm:inline-flex",
                            title: if f.ton { tr!(l, "Sounds off", "Töne aus", "Silenciar") } else { tr!(l, "Sounds on", "Töne an", "Activar sonidos") },
                            onclick: move |_| aendern(fortschritt, |f| f.ton = !f.ton),
                            if f.ton { Volume2 { class: "size-4" } } else { VolumeX { class: "size-4" } }
                        }
                        ThemeToggle {}
                        if let Some(m) = me.clone() {
                            div { class: "relative",
                                button {
                                    class: "flex size-8 items-center justify-center rounded-full border-2 text-sm font-bold transition hover:bg-accent",
                                    title: "{m.username}",
                                    onclick: move |_| menue.set(!menue()),
                                    {m.username.chars().next().map(|c| c.to_uppercase().to_string()).unwrap_or_default()}
                                }
                                if menue() {
                                    div { class: "fixed inset-0 z-40", onclick: move |_| menue.set(false) }
                                    div { class: "anim-pop absolute right-0 top-10 z-50 w-56 overflow-hidden rounded-xl border-2 bg-card shadow-xl",
                                        div { class: "flex items-center gap-2 border-b px-3 py-2.5",
                                            UserRound { class: "size-4" }
                                            div { class: "min-w-0",
                                                div { class: "truncate font-semibold", "{m.username}" }
                                                div { class: "truncate text-xs text-muted-foreground", "{instanz}" }
                                            }
                                        }
                                        button {
                                            class: "flex w-full items-center gap-2 px-3 py-2.5 text-left text-sm hover:bg-accent",
                                            onclick: move |_| {
                                                menue.set(false);
                                                abmelden(konto);
                                            },
                                            LogOut { class: "size-4" }
                                            {tr!(l, "Log out", "Abmelden", "Cerrar sesión")}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            main { class: "flex-1", Outlet::<Route> {} }
            footer { class: "border-t py-6 text-center text-xs text-muted-foreground",
                "{instanz} · nrzLearn · "
                {tr!(l, "built with Rust, Dioxus and ", "gebaut mit Rust, Dioxus und ", "hecho con Rust, Dioxus y ")}
                a { class: "underline underline-offset-2 hover:text-foreground", href: "https://github.com/rust-ui/ui", target: "_blank", "rust-ui" }
                {tr!(l, " · Code runs on the official Rust Playground.", " · Code läuft über den offiziellen Rust Playground.", " · El código se ejecuta en el Rust Playground oficial.")}
            }
        }
    }
}
