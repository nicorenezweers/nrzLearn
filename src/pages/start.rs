//! Entry points of the app. In production `/` and `/<lang>/` are static
//! landing pages (built by `nrzlearn-site`); these routes only show up
//! during development or for unknown links.

use dioxus::prelude::*;

use crate::Route;
use crate::components::kurswahl::KursKachel;
use crate::components::logo::Logo;
use crate::i18n::{Sprache, use_sprache_signal};
use crate::kurs::Kurs;
use crate::progress::use_fortschritt;

#[component]
pub fn Start() -> Element {
    let l = use_sprache_signal()();
    rsx! { StartSeite { lang: l } }
}

#[component]
pub fn StartSprache(lang: Sprache) -> Element {
    let mut sprache = use_sprache_signal();
    use_effect(use_reactive!(|lang| {
        lang.merken();
        if *sprache.peek() != lang {
            sprache.set(lang);
        }
    }));
    rsx! { StartSeite { lang } }
}

/// A simple course picker (the real one is the static landing page).
#[component]
fn StartSeite(lang: Sprache) -> Element {
    let l = lang;
    let fortschritt = use_fortschritt();
    let f = fortschritt.read();
    rsx! {
        document::Title { "nrzLearn" }
        div { class: "mx-auto max-w-5xl px-4 py-12",
            div { class: "flex items-center gap-3",
                Logo { class: "size-12" }
                span { class: "text-3xl font-extrabold tracking-tight", "nrz" span { class: "font-light", "Learn" } }
            }
            h1 { class: "mt-8 text-4xl font-extrabold tracking-tight", {t!(l, START_TITLE)} }
            p { class: "mt-2 text-lg text-muted-foreground", {t!(l, START_SUBTITLE)} }
            for g in nrzlearn_model::GROUPS {
                h2 { key: "{g}", class: "mt-10 text-sm font-bold uppercase tracking-wider text-muted-foreground", {crate::kurs::gruppe(l, g)} }
                div { class: "mt-3 grid gap-4 sm:grid-cols-2 lg:grid-cols-3",
                    for k in Kurs::ALLE.into_iter().filter(|k| k.info().group == g) {
                        Link {
                            key: "{k.id()}",
                            to: Route::Home { lang: l, kurs: k },
                            class: "flex items-center gap-4 rounded-2xl border-2 border-b-4 p-4 transition hover:border-foreground active:translate-y-0.5 active:border-b-2",
                            KursKachel { kurs: k, class: "size-12" }
                            div { class: "min-w-0",
                                div { class: "text-lg font-bold", "{k.titel(l)}" }
                                div { class: "text-sm text-muted-foreground",
                                    {tf!(l, START_LEVELS_DONE, n = f.fertig_im_kurs(k))}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// `/<lang>/<course>` without a page: open the learning path.
#[component]
pub fn KursStart(lang: Sprache, kurs: Kurs) -> Element {
    let nav = navigator();
    use_effect(move || {
        nav.replace(Route::Home { lang, kurs });
    });
    rsx! {}
}

/// Old links of nrzLearn 3/4 and unknown paths.
#[component]
pub fn Unbekannt(rest: Vec<String>) -> Element {
    let nav = navigator();
    let l = Sprache::aus_browser();
    let kurs = Kurs::zuletzt().unwrap_or_default();
    let teile: Vec<&str> = rest.iter().map(String::as_str).filter(|s| !s.is_empty()).collect();
    let ziel: Result<Route, String> = match teile.as_slice() {
        ["world", n] => n.parse().map(|id| Route::LektionSeite { lang: l, kurs: Kurs::RUST, id }).map_err(|_| String::new()),
        ["learn", id] => Ok(Route::Spieler { lang: l, kurs: Kurs::RUST, id: (*id).to_string() }),
        ["profile"] => Ok(Route::Profil { lang: l, kurs }),
        ["terminal"] => Ok(Route::TerminalSeite { lang: l, kurs: Kurs::RUST }),
        ["cheatsheet"] => Ok(Route::Spickzettel { lang: l, kurs: Kurs::RUST }),
        ["impressum"] | ["privacy"] => Err(format!("/{l}/{}/", teile[0])),
        _ => Err(String::new()),
    };
    use_effect(move || match &ziel {
        Ok(r) => {
            nav.replace(r.clone());
        }
        Err(pfad) if !pfad.is_empty() => crate::seite_laden(pfad),
        Err(_) => {
            nav.replace(Route::StartSprache { lang: l });
        }
    });
    rsx! {}
}
