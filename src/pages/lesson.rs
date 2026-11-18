//! The theory of a world ("Learn" level): the Rust lessons with interactive
//! visuals, or the lesson blocks of a JSON course.

use dioxus::prelude::*;
use icons::{ArrowLeft, CircleCheck, Info, Languages, Lock, StickyNote, TriangleAlert};
use nrzlearn_model::Block as DatenBlock;

use crate::Route;
use crate::components::code::{CodeBlock, CodeRunner};
use crate::components::effekte::{Konfetti, Ton, ton};
use crate::components::rahmen::use_kurs;
use crate::components::spick::TaschenSpick;
use crate::components::terminal::TerminalView;
use crate::components::text::RichText;
use crate::components::ui::Separator;
use crate::content::{Block, lektionen};
use crate::i18n::{Sprache, use_sprache};
use crate::inhalt::{KursInhalt, use_inhalt};
use crate::kurs::Kurs;
use crate::progress::{abschliessen, use_fortschritt};
use crate::shell::Shell;
use crate::visuals::VisualView;

/// Note when the course texts are not in the interface language.
#[component]
pub fn InhaltSprachHinweis() -> Element {
    let l = use_sprache();
    let inhalt = use_inhalt();
    if inhalt.sprache == l {
        return rsx! {};
    }
    let text = if inhalt.kurs.ist_rust() { t!(l, CONTENT_LANG_RUST) } else { t!(l, CONTENT_LANG_FALLBACK) };
    rsx! {
        div { class: "mb-6 flex items-start gap-3 rounded-xl border-2 border-dashed p-3 text-sm text-muted-foreground",
            Languages { class: "mt-0.5 size-4 shrink-0" }
            span { "{text}" }
        }
    }
}

fn tipp(text: String) -> Element {
    rsx! {
        div { class: "flex gap-3 rounded-xl border border-info/30 bg-info-light/50 p-4 text-info-dark",
            Info { class: "mt-1 size-4 shrink-0" }
            RichText { text, class: "leading-7" }
        }
    }
}

fn achtung(text: String) -> Element {
    rsx! {
        div { class: "flex gap-3 rounded-xl border border-warning/40 bg-warning-light/60 p-4 text-warning-dark",
            TriangleAlert { class: "mt-1 size-4 shrink-0" }
            RichText { text, class: "leading-7" }
        }
    }
}

#[component]
pub fn LektionSeite(lang: Sprache, kurs: Kurs, id: usize) -> Element {
    let _ = (lang, kurs);
    let fortschritt = use_fortschritt();
    let l = use_sprache();
    let kurs = use_kurs();
    let inhalt = use_inhalt();
    let mut gefeiert = use_signal(|| false);

    // Scroll to the top when switching worlds.
    use_effect(use_reactive!(|id| {
        let _ = id;
        document::eval("window.scrollTo({top:0})");
    }));

    let Some(index) = id.checked_sub(1).filter(|i| *i < inhalt.anzahl_welten()) else {
        return rsx! {
            div { class: "mx-auto max-w-3xl px-4 py-16 text-center",
                h1 { class: "text-2xl font-bold", {t!(l, WORLD_MISSING)} }
                Link { to: Route::Home { lang: l, kurs }, class: "mt-4 inline-block underline", {t!(l, BACK_TO_PATH)} }
            }
        };
    };

    let f = fortschritt.read();
    let knoten_id = inhalt.theorie_id(index);
    let Some((knoten_index, knoten)) = inhalt.finden(&knoten_id).map(|(i, k)| (i, k.clone())) else {
        return rsx! {};
    };

    if !f.offen(inhalt.knoten(), knoten_index) {
        return rsx! {
            div { class: "mx-auto max-w-xl px-4 py-20 text-center",
                Lock { class: "mx-auto mb-4 size-10 text-muted-foreground" }
                h1 { class: "text-2xl font-bold", {tf!(l, LEVEL_LOCKED, n = knoten.nr)} }
                p { class: "mt-2 text-muted-foreground", {t!(l, LEVEL_LOCKED_TEXT)} }
                Link { to: Route::Home { lang: l, kurs }, class: "mt-6 inline-flex h-10 items-center gap-2 rounded-xl bg-foreground px-5 text-sm font-semibold text-background", ArrowLeft { class: "size-4 rtl:rotate-180" } {t!(l, TO_PATH)} }
            }
        };
    }

    let abgeschlossen = f.fertig(&knoten_id);
    let karte = inhalt.karten().into_iter().find(|k| k.welt == index);
    let ton_an = f.ton;
    let naechster = inhalt.knoten().get(knoten_index + 1).cloned();

    rsx! {
        div { class: "mx-auto flex max-w-6xl gap-8 px-4 py-8",
            // Seitenleiste
            aside { class: "sticky top-20 hidden h-[calc(100vh-6rem)] w-60 shrink-0 overflow-y-auto lg:block",
                div { class: "mb-2 text-xs font-semibold uppercase tracking-wider text-muted-foreground", {t!(l, THEORY_OF_WORLDS)} }
                nav { class: "space-y-0.5",
                    for w in 0..inhalt.anzahl_welten() {
                        if inhalt.finden(&inhalt.theorie_id(w)).is_some_and(|(ki, _)| f.offen(inhalt.knoten(), ki)) {
                            Link {
                                key: "{w}",
                                to: Route::LektionSeite { lang: l, kurs, id: w + 1 },
                                class: if w == index { "flex items-center gap-2 rounded-md bg-accent px-2 py-1.5 text-sm font-medium" } else { "flex items-center gap-2 rounded-md px-2 py-1.5 text-sm text-muted-foreground hover:bg-accent hover:text-foreground" },
                                span { class: "w-5 shrink-0 text-end font-mono text-xs", "{w + 1}" }
                                span { class: "truncate", "{inhalt.welt_titel(w)}" }
                                if inhalt.karte_offen(&f, w) { CircleCheck { class: "ms-auto size-3.5 shrink-0" } }
                            }
                        } else {
                            div { key: "{w}", class: "flex items-center gap-2 px-2 py-1.5 text-sm text-muted-foreground/50",
                                span { class: "w-5 shrink-0 text-end font-mono text-xs", "{w + 1}" }
                                span { class: "truncate", "{inhalt.welt_titel(w)}" }
                                Lock { class: "ms-auto size-3 shrink-0" }
                            }
                        }
                    }
                }
            }

            article { class: "min-w-0 flex-1",
                div { class: "mb-2 flex items-center gap-2 text-sm text-muted-foreground",
                    Link { to: Route::Home { lang: l, kurs }, class: "hover:text-foreground", {t!(l, NAV_PATH_TITLE)} }
                    span { "/" }
                    span { "{inhalt.welt_kapitel(index)}" }
                }
                div { class: "flex flex-wrap items-end justify-between gap-4",
                    div {
                        div { class: "font-mono text-sm text-muted-foreground", {tf!(l, WORLD_LEVEL_LEARN, world = id, n = knoten.nr)} }
                        h1 { class: "mt-1 text-3xl font-bold tracking-tight sm:text-4xl", "{inhalt.welt_titel(index)}" }
                        p { class: "mt-2 text-lg text-muted-foreground", "{inhalt.welt_kurz(index)}" }
                    }
                    if abgeschlossen {
                        span { class: "flex items-center gap-1.5 rounded-full border-2 px-3 py-1 text-sm font-semibold", CircleCheck { class: "size-4" } {t!(l, READ_DONE)} }
                    }
                }
                Separator { class: "my-6" }
                InhaltSprachHinweis {}

                if kurs.ist_rust() {
                    RustBloecke { index }
                } else {
                    DatenBloecke { inhalt: inhalt.clone(), index }
                }

                // Abschluss
                div { class: "mt-12 rounded-2xl border-2 p-6 text-center",
                    if gefeiert() { Konfetti { anzahl: 40 } }
                    if let Some(k) = karte {
                        div { class: "mx-auto mb-3 flex size-14 items-center justify-center rounded-2xl bg-foreground text-background", StickyNote { class: "size-7" } }
                        h3 { class: "text-xl font-extrabold",
                            if abgeschlossen { {tf!(l, CARD_COLLECTED, title = k.titel)} } else { {t!(l, READ_EVERYTHING)} }
                        }
                        p { class: "mx-auto mt-1 max-w-md text-sm text-muted-foreground",
                            if abgeschlossen { {t!(l, ON_TO_EXERCISES)} } else { {tf!(l, FINISH_THEORY, title = k.titel)} }
                        }
                    }
                    div { class: "mt-5 flex flex-col justify-center gap-3 sm:flex-row",
                        if !abgeschlossen {
                            button {
                                class: "h-12 rounded-2xl border-2 border-b-4 border-foreground bg-foreground px-8 font-bold uppercase tracking-wide text-background transition active:translate-y-0.5 active:border-b-2",
                                "data-primaer": "true",
                                onclick: {
                                    let knoten_id = knoten_id.clone();
                                    move |_| {
                                        abschliessen(fortschritt, &knoten_id, 3, 15, 0, true);
                                        ton(Ton::Geschafft, ton_an);
                                        gefeiert.set(true);
                                    }
                                },
                                {t!(l, COMPLETE_XP)}
                            }
                        } else if let Some(n) = naechster {
                            Link {
                                to: crate::pages::home::start_route(l, kurs, &n),
                                class: "flex h-12 items-center justify-center rounded-2xl border-2 border-b-4 border-foreground bg-foreground px-8 font-bold uppercase tracking-wide text-background",
                                {tf!(l, NEXT_LEVEL, n = n.nr)}
                            }
                        }
                        Link { to: Route::Home { lang: l, kurs }, class: "flex h-12 items-center justify-center rounded-2xl border-2 border-b-4 px-8 font-bold uppercase tracking-wide", {t!(l, TO_PATH)} }
                    }
                }
            }
        }
        TaschenSpick {}
    }
}

/// Lesson blocks of the Rust course.
#[component]
fn RustBloecke(index: usize) -> Element {
    let sp = use_sprache();
    let lektion = &lektionen(sp)[index];
    rsx! {
        div { class: "space-y-5",
            for (bi, block) in lektion.bloecke.iter().enumerate() {
                div { key: "{index}-{bi}",
                    match block {
                        Block::Ueberschrift(t) => rsx! { h2 { class: "pt-4 text-xl font-semibold tracking-tight", "{t}" } },
                        Block::Text(t) => rsx! { RichText { text: t.to_string() } },
                        Block::Code(c) => rsx! { CodeBlock { code: c.to_string() } },
                        Block::Ausfuehrbar(c) => rsx! { CodeRunner { code: c.to_string() } },
                        Block::Tipp(t) => tipp(t.to_string()),
                        Block::Achtung(t) => achtung(t.to_string()),
                        Block::Visual(_) => rsx! { VisualView { lektion: index, block: bi } },
                        Block::Terminal => rsx! { FreiesTerminal {} },
                    }
                }
            }
        }
    }
}

/// Lesson blocks of a JSON course.
#[component]
fn DatenBloecke(inhalt: KursInhalt, index: usize) -> Element {
    let Some(u) = inhalt.einheit(index) else { return rsx! {} };
    rsx! {
        div { class: "space-y-5",
            for (bi, block) in u.lesson.iter().enumerate() {
                div { key: "{index}-{bi}",
                    match block {
                        DatenBlock::Heading { text } => rsx! { h2 { class: "pt-4 text-xl font-semibold tracking-tight", "{text}" } },
                        DatenBlock::Text { md } => rsx! { RichText { text: md.clone() } },
                        DatenBlock::Code { code, output } => rsx! { CodeBlock { code: code.clone(), ausgabe: output.clone() } },
                        DatenBlock::Tip { md } => tipp(md.clone()),
                        DatenBlock::Warning { md } => achtung(md.clone()),
                    }
                }
            }
        }
    }
}

#[component]
fn FreiesTerminal() -> Element {
    let l = crate::i18n::use_sprache();
    let shell = use_signal(|| Shell::neu(l));
    rsx! { TerminalView { shell, hoehe: "h-72" } }
}
