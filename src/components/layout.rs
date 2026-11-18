use dioxus::prelude::*;
use icons::{Flame, Map, StickyNote, Terminal, UserRound, Volume2, VolumeX, Zap};

use crate::Route;
use crate::components::kurswahl::KursMenue;
use crate::components::logo::Logo;
use crate::components::rahmen::{KursRahmen, use_kurs};
use crate::components::sprachwahl::Sprachwahl;
use crate::components::ui::ThemeToggle;
use crate::i18n::use_sprache;
use crate::inhalt::use_inhalt;
use crate::progress::{aendern, use_fortschritt};

#[component]
pub fn Layout() -> Element {
    let route: Route = use_route();
    let (lang, kurs) = route.lang_kurs().unwrap_or_default();
    rsx! {
        KursRahmen { lang, kurs,
            div { class: "flex min-h-screen flex-col",
                Kopf {}
                main { class: "flex-1", Outlet::<Route> {} }
                Fusszeile {}
            }
        }
    }
}

#[component]
fn Kopf() -> Element {
    let fortschritt = use_fortschritt();
    let l = use_sprache();
    let kurs = use_kurs();
    let inhalt = use_inhalt();
    let route: Route = use_route();
    let f = fortschritt.read();
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
        document::Title { "{kurs.titel(l)} · nrzLearn" }
        header { class: "sticky top-0 z-40 border-b bg-background/85 backdrop-blur",
            div { class: "mx-auto flex h-14 max-w-6xl items-center gap-1 px-3 sm:px-4",
                a { href: "/{l}/", class: "me-1 flex items-center gap-2 text-lg font-extrabold tracking-tight", title: t!(l, ALL_COURSES),
                    Logo { class: "size-8" }
                    span { class: "hidden xl:inline", "nrz" span { class: "font-light", "Learn" } }
                }
                KursMenue { kurs }
                nav { class: "ms-1 flex items-center gap-0.5",
                    Link { to: Route::Home { lang: l, kurs }, class: nav(matches!(route, Route::Home { .. } | Route::LektionSeite { .. })), title: t!(l, NAV_PATH_TITLE), Map { class: "size-4" } span { class: "hidden md:inline", {t!(l, NAV_PATH)} } }
                    Link { to: Route::Profil { lang: l, kurs }, class: nav(matches!(route, Route::Profil { .. })), title: t!(l, NAV_PROFILE_TITLE), UserRound { class: "size-4" } span { class: "hidden lg:inline", {t!(l, NAV_PROFILE)} } }
                    if inhalt.kurs.ist_rust() {
                        Link { to: Route::TerminalSeite { lang: l, kurs }, class: nav(matches!(route, Route::TerminalSeite { .. })), title: "Terminal", Terminal { class: "size-4" } span { class: "hidden xl:inline", "Terminal" } }
                    }
                    Link { to: Route::Spickzettel { lang: l, kurs }, class: nav(matches!(route, Route::Spickzettel { .. })), title: t!(l, NAV_CHEATSHEET), StickyNote { class: "size-4" } span { class: "hidden xl:inline", {t!(l, NAV_CHEATSHEET)} } }
                }
                div { class: "ms-auto flex items-center gap-0.5 sm:gap-1",
                    div {
                        class: if f.ist_heute_aktiv() { "flex items-center gap-1 rounded-xl px-1.5 py-1 text-sm font-bold" } else { "flex items-center gap-1 rounded-xl px-1.5 py-1 text-sm font-bold text-muted-foreground" },
                        title: t!(l, STREAK),
                        Flame { key: "{f.ist_heute_aktiv()}", class: if f.ist_heute_aktiv() { "size-4 fill-foreground" } else { "size-4" } }
                        "{f.serie}"
                    }
                    div { class: "hidden items-center gap-1.5 rounded-xl px-1.5 py-1 text-sm font-bold sm:flex", title: tf!(l, DAILY_GOAL_TITLE, done = f.heute_xp, goal = f.ziel),
                        svg { class: "-rotate-90", width: "22", height: "22", view_box: "0 0 22 22",
                            circle { cx: "11", cy: "11", r: "9", fill: "none", stroke: "var(--muted)", stroke_width: "3" }
                            circle { cx: "11", cy: "11", r: "9", fill: "none", stroke: "var(--foreground)", stroke_width: "3", stroke_linecap: "round", stroke_dasharray: "56.5", stroke_dashoffset: "{ring}" }
                        }
                    }
                    div { class: "hidden items-center gap-1 whitespace-nowrap rounded-xl px-1.5 py-1 text-sm font-bold sm:flex", title: "XP · Level {f.level()}",
                        Zap { class: "size-4 fill-foreground" }
                        "{f.xp}"
                    }
                    Sprachwahl {}
                    button {
                        class: "hidden size-8 items-center justify-center rounded-md transition-colors hover:bg-accent sm:inline-flex",
                        title: if f.ton { t!(l, SOUND_OFF) } else { t!(l, SOUND_ON) },
                        onclick: move |_| aendern(fortschritt, |f| f.ton = !f.ton),
                        if f.ton { Volume2 { class: "size-4" } } else { VolumeX { class: "size-4" } }
                    }
                    ThemeToggle {}
                }
            }
        }
    }
}

/// Footer with the legal links.
#[component]
pub fn Fusszeile() -> Element {
    let l = use_sprache();
    rsx! {
        footer { class: "border-t px-4 py-6 text-center text-xs text-muted-foreground",
            p {
                "nrzLearn · Rust · Dioxus · "
                a { class: "underline underline-offset-2 hover:text-foreground", href: "https://github.com/rust-ui/ui", target: "_blank", rel: "noopener", "rust-ui" }
                " · "
                {t!(l, FOOTER_LOCAL)}
            }
            p { class: "mt-2 flex flex-wrap items-center justify-center gap-x-3 gap-y-1",
                a { class: "underline underline-offset-2 hover:text-foreground", href: "/{l}/", {t!(l, ALL_COURSES)} }
                span { "·" }
                a { class: "underline underline-offset-2 hover:text-foreground", href: "/{l}/impressum/", {t!(l, LEGAL_NOTICE)} }
                span { "·" }
                a { class: "underline underline-offset-2 hover:text-foreground", href: "/{l}/privacy/", {t!(l, PRIVACY)} }
            }
        }
    }
}
