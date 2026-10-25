//! The pocket cheat sheet: collected cards, always at hand – except in
//! exams where it is not allowed.

use dioxus::prelude::*;
use icons::{Lock, StickyNote, X};

use crate::components::code::CodeBlock;
use crate::content::spick::KARTEN;
use crate::i18n::use_sprache;
use crate::progress::use_fortschritt;

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
    let mut offen = use_signal(|| false);
    let anzahl = fortschritt.read().karten();
    let mut filter = use_signal(|| None::<usize>);

    let knopf = if kompakt {
        "relative flex size-10 items-center justify-center rounded-xl border-2 border-b-4 bg-card transition hover:bg-accent active:translate-y-0.5 disabled:cursor-not-allowed disabled:opacity-40"
    } else {
        "fixed bottom-5 right-5 z-40 flex items-center gap-2 rounded-2xl border-2 border-b-4 border-foreground bg-foreground px-4 py-3 text-sm font-semibold text-background shadow-xl transition hover:-translate-y-0.5 active:translate-y-0.5"
    };

    rsx! {
        button {
            class: "{knopf}",
            disabled: verboten,
            title: if verboten { tr!(l, "No cheat sheet allowed in this exam", "In dieser Prüfung ist kein Spickzettel erlaubt", "En este examen no se permite la chuleta") } else { tr!(l, "Open cheat sheet", "Spickzettel öffnen", "Abrir chuleta") },
            onclick: move |_| offen.set(true),
            if verboten {
                Lock { class: "size-4" }
            } else {
                StickyNote { class: "size-4" }
            }
            if !kompakt {
                {tr!(l, "Cheat sheet", "Spickzettel", "Chuleta")}
                span { class: "rounded-full bg-background/20 px-1.5 text-xs", "{anzahl}" }
            }
        }
        if offen() && !verboten {
            div { class: "fixed inset-0 z-50 flex justify-end bg-black/40 backdrop-blur-[2px]", onclick: move |_| offen.set(false),
                aside {
                    class: "anim-slide-up flex h-full w-full max-w-md flex-col border-l bg-background shadow-2xl sm:animate-none",
                    onclick: move |e| e.stop_propagation(),
                    div { class: "flex items-center gap-3 border-b px-5 py-4",
                        StickyNote { class: "size-5" }
                        div { class: "flex-1",
                            div { class: "font-bold", {tr!(l, "Your cheat sheet", "Dein Spickzettel", "Tu chuleta")} }
                            div { class: "text-xs text-muted-foreground", {tr!(l, format!("{anzahl} of {} cards collected", KARTEN.len()), format!("{anzahl} von {} Karten gesammelt", KARTEN.len()), format!("{anzahl} de {} tarjetas conseguidas", KARTEN.len()))} }
                        }
                        button { class: "rounded-lg p-2 hover:bg-accent", onclick: move |_| offen.set(false), X { class: "size-5" } }
                    }
                    if anzahl == 0 {
                        div { class: "flex flex-1 flex-col items-center justify-center gap-2 p-8 text-center text-sm text-muted-foreground",
                            StickyNote { class: "size-10 opacity-40" }
                            p { {tr!(l, "Still empty. Finish the first “Learn” level of a world and its card lands here.", "Noch leer. Schließe das erste „Lernen“-Level einer Welt ab, dann landet ihre Karte hier.", "Todavía vacía. Termina el nivel «Aprender» de un mundo y su tarjeta aparecerá aquí.")} }
                        }
                    } else {
                        div { class: "flex flex-wrap gap-1.5 border-b px-5 py-3",
                            button {
                                class: if filter().is_none() { "rounded-full bg-foreground px-2.5 py-1 text-xs text-background" } else { "rounded-full border px-2.5 py-1 text-xs" },
                                onclick: move |_| filter.set(None),
                                {tr!(l, "All", "Alle", "Todas")}
                            }
                            for k in KARTEN.iter().filter(|k| fortschritt.read().karte_offen(k.welt)) {
                                button {
                                    key: "f{k.welt}",
                                    class: if filter() == Some(k.welt) { "rounded-full bg-foreground px-2.5 py-1 text-xs text-background" } else { "rounded-full border px-2.5 py-1 text-xs" },
                                    onclick: move |_| filter.set(Some(k.welt)),
                                    "{k.welt + 1}"
                                }
                            }
                        }
                        div { class: "flex-1 space-y-4 overflow-y-auto px-5 py-4",
                            for k in KARTEN.iter().filter(|k| fortschritt.read().karte_offen(k.welt) && filter().is_none_or(|f| f == k.welt)) {
                                div { key: "{k.welt}",
                                    div { class: "mb-1.5 flex items-center gap-2 text-sm font-semibold",
                                        span { class: "flex size-5 items-center justify-center rounded bg-foreground text-[0.65rem] text-background", "{k.welt + 1}" }
                                        "{k.titel(l)}"
                                    }
                                    CodeBlock { code: k.code(l).to_string() }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
