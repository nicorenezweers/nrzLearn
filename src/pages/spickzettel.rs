use dioxus::prelude::*;
use icons::{Lock, StickyNote};

use crate::components::spick::KarteInhalt;
use crate::i18n::{Sprache, use_sprache};
use crate::inhalt::use_inhalt;
use crate::kurs::Kurs;
use crate::progress::use_fortschritt;

#[component]
pub fn Spickzettel(lang: Sprache, kurs: Kurs) -> Element {
    let _ = (lang, kurs);
    let fortschritt = use_fortschritt();
    let l = use_sprache();
    let inhalt = use_inhalt();
    let f = fortschritt.read();
    let karten = inhalt.karten();
    let offen = karten.iter().filter(|k| inhalt.karte_offen(&f, k.welt)).count();
    rsx! {
        div { class: "mx-auto max-w-6xl px-4 py-8",
            div { class: "flex items-center gap-3",
                div { class: "flex size-12 items-center justify-center rounded-2xl bg-foreground text-background", StickyNote { class: "size-6" } }
                div {
                    h1 { class: "text-3xl font-extrabold tracking-tight", {format!("{} · {}", t!(l, NAV_CHEATSHEET), inhalt.kurs.titel(l))} }
                    p { class: "text-muted-foreground", {tf!(l, CHEATSHEET_INTRO, n = offen, total = karten.len())} }
                }
            }
            p { class: "mt-4 rounded-xl border-2 border-dashed p-3 text-sm text-muted-foreground", {t!(l, CHEATSHEET_RULES)} }
            div { class: "mt-6 columns-1 gap-4 md:columns-2 xl:columns-3",
                for k in karten.into_iter() {
                    div { key: "{k.welt}", class: "mb-4 break-inside-avoid",
                        div { class: "mb-2 flex items-center gap-2 text-sm font-semibold",
                            span { class: "flex size-6 items-center justify-center rounded-md bg-foreground text-xs text-background", "{k.welt + 1}" }
                            "{k.titel}"
                        }
                        if inhalt.karte_offen(&f, k.welt) {
                            KarteInhalt { karte: k.clone() }
                        } else {
                            div { class: "relative overflow-hidden rounded-xl border-2 border-dashed",
                                div { class: "pointer-events-none select-none p-4 opacity-30 blur-[3px]", "aria-hidden": "true", KarteInhalt { karte: k.clone() } }
                                div { class: "absolute inset-0 flex flex-col items-center justify-center gap-1 p-2 text-center text-sm font-semibold",
                                    Lock { class: "size-5" }
                                    {tf!(l, UNLOCK_WORLD, n = k.welt + 1)}
                                    span { class: "text-xs font-normal text-muted-foreground", {tf!(l, UNLOCK_WORLD_HINT, title = inhalt.welt_titel(k.welt))} }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
