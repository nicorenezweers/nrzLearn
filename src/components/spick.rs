//! The pocket cheat sheet: collected cards, always at hand – except in
//! exams where it is not allowed.

use dioxus::prelude::*;
use icons::{Lock, StickyNote, X};

use crate::components::code::CodeBlock;
use crate::components::text::RichText;
use crate::i18n::use_sprache;
use crate::inhalt::{Karte, use_inhalt};
use crate::progress::use_fortschritt;

/// The content of a card: Rust cards are code, JSON courses have points.
#[component]
pub fn KarteInhalt(karte: Karte) -> Element {
    if let Some(code) = karte.code {
        return rsx! { CodeBlock { code } };
    }
    rsx! {
        ul { class: "space-y-1.5 rounded-xl border bg-code-bg p-4",
            for (i, p) in karte.punkte.iter().enumerate() {
                li { key: "{i}", class: "flex gap-2 text-sm",
                    span { class: "mt-2 size-1.5 shrink-0 rounded-full bg-foreground" }
                    RichText { text: p.clone(), class: "leading-6" }
                }
            }
        }
    }
}

#[component]
pub fn TaschenSpick(
    /// Not allowed in this exam?
    #[props(default)]
    verboten: bool,
    /// Floating button (pages) or compact (lesson player).
    #[props(default)]
    kompakt: bool,
) -> Element {
    let fortschritt = use_fortschritt();
    let l = use_sprache();
    let inhalt = use_inhalt();
    let mut offen = use_signal(|| false);
    let mut filter = use_signal(|| None::<usize>);
    let f = fortschritt.read();
    let karten: Vec<Karte> = inhalt.karten().into_iter().filter(|k| inhalt.karte_offen(&f, k.welt)).collect();
    let gesamt = inhalt.anzahl_welten();
    let anzahl = karten.len();

    let knopf = if kompakt {
        "relative flex size-10 items-center justify-center rounded-xl border-2 border-b-4 bg-card transition hover:bg-accent active:translate-y-0.5 disabled:cursor-not-allowed disabled:opacity-40"
    } else {
        "fixed bottom-5 end-5 z-40 flex items-center gap-2 rounded-2xl border-2 border-b-4 border-foreground bg-foreground px-4 py-3 text-sm font-semibold text-background shadow-xl transition hover:-translate-y-0.5 active:translate-y-0.5"
    };

    rsx! {
        button {
            class: "{knopf}",
            disabled: verboten,
            title: if verboten { t!(l, CHEATSHEET_FORBIDDEN) } else { t!(l, CHEATSHEET_OPEN) },
            onclick: move |_| offen.set(true),
            if verboten {
                Lock { class: "size-4" }
            } else {
                StickyNote { class: "size-4" }
            }
            if !kompakt {
                {t!(l, NAV_CHEATSHEET)}
                span { class: "rounded-full bg-background/20 px-1.5 text-xs", "{anzahl}" }
            }
        }
        if offen() && !verboten {
            div { class: "fixed inset-0 z-50 flex justify-end bg-black/40 backdrop-blur-[2px]", onclick: move |_| offen.set(false),
                aside {
                    class: "anim-slide-up flex h-full w-full max-w-md flex-col border-s bg-background shadow-2xl sm:animate-none",
                    onclick: move |e| e.stop_propagation(),
                    div { class: "flex items-center gap-3 border-b px-5 py-4",
                        StickyNote { class: "size-5" }
                        div { class: "flex-1",
                            div { class: "font-bold", {t!(l, YOUR_CHEATSHEET)} }
                            div { class: "text-xs text-muted-foreground", {tf!(l, CARDS_COLLECTED, n = anzahl, total = gesamt)} }
                        }
                        button { class: "rounded-lg p-2 hover:bg-accent", onclick: move |_| offen.set(false), X { class: "size-5" } }
                    }
                    if anzahl == 0 {
                        div { class: "flex flex-1 flex-col items-center justify-center gap-2 p-8 text-center text-sm text-muted-foreground",
                            StickyNote { class: "size-10 opacity-40" }
                            p { {t!(l, CHEATSHEET_EMPTY)} }
                        }
                    } else {
                        div { class: "flex flex-wrap gap-1.5 border-b px-5 py-3",
                            button {
                                class: if filter().is_none() { "rounded-full bg-foreground px-2.5 py-1 text-xs text-background" } else { "rounded-full border px-2.5 py-1 text-xs" },
                                onclick: move |_| filter.set(None),
                                {t!(l, ALL)}
                            }
                            for k in karten.iter() {
                                button {
                                    key: "f{k.welt}",
                                    class: if filter() == Some(k.welt) { "rounded-full bg-foreground px-2.5 py-1 text-xs text-background" } else { "rounded-full border px-2.5 py-1 text-xs" },
                                    onclick: {
                                        let w = k.welt;
                                        move |_| filter.set(Some(w))
                                    },
                                    "{k.welt + 1}"
                                }
                            }
                        }
                        div { class: "flex-1 space-y-4 overflow-y-auto px-5 py-4",
                            for k in karten.iter().filter(|k| filter().is_none_or(|f| f == k.welt)) {
                                div { key: "{k.welt}",
                                    div { class: "mb-1.5 flex items-center gap-2 text-sm font-semibold",
                                        span { class: "flex size-5 items-center justify-center rounded bg-foreground text-[0.65rem] text-background", "{k.welt + 1}" }
                                        "{k.titel}"
                                    }
                                    KarteInhalt { karte: k.clone() }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
