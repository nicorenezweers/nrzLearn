use dioxus::prelude::*;
use icons::{ArrowLeft, CircleCheck, Info, Lock, StickyNote, TriangleAlert};

use crate::Route;
use crate::components::code::{CodeBlock, CodeRunner};
use crate::components::effekte::{Konfetti, Ton, ton};
use crate::components::spick::TaschenSpick;
use crate::components::terminal::TerminalView;
use crate::components::text::RichText;
use crate::components::ui::Separator;
use crate::content::{ANZAHL_WELTEN, Block, lektionen};
use crate::i18n::use_sprache;
use crate::content::spick::KARTEN;
use crate::progress::{abschliessen, use_fortschritt};
use crate::shell::Shell;
use crate::visuals::VisualView;
use crate::welt;

#[component]
pub fn LektionSeite(id: usize) -> Element {
    let fortschritt = use_fortschritt();
    let sp = use_sprache();
    let Some(index) = id.checked_sub(1).filter(|i| *i < ANZAHL_WELTEN) else {
        return rsx! {
            div { class: "mx-auto max-w-3xl px-4 py-16 text-center",
                h1 { class: "text-2xl font-bold", {tr!(sp, "This world doesn't exist.", "Diese Welt gibt es nicht.", "Este mundo no existe.")} }
                Link { to: Route::Home {}, class: "mt-4 inline-block underline", {tr!(sp, "Back to the learning path", "Zurück zum Lernpfad", "Volver al camino de aprendizaje")} }
            }
        };
    };

    // Scroll to the top when switching worlds.
    use_effect(use_reactive!(|id| {
        let _ = id;
        document::eval("window.scrollTo({top:0})");
    }));

    let f = fortschritt.read();
    let l = &lektionen(sp)[index];
    let knoten_id = format!("w{index}-theorie");
    let (knoten_index, knoten) = welt::finden(&knoten_id).expect("theory level exists");
    let mut gefeiert = use_signal(|| false);

    if !f.offen(knoten_index) {
        return rsx! {
            div { class: "mx-auto max-w-xl px-4 py-20 text-center",
                Lock { class: "mx-auto mb-4 size-10 text-muted-foreground" }
                h1 { class: "text-2xl font-bold", {tr!(sp, format!("Level {} is still locked", knoten.nr), format!("Level {} ist noch gesperrt", knoten.nr), format!("El nivel {} sigue bloqueado", knoten.nr))} }
                p { class: "mt-2 text-muted-foreground", {tr!(sp, "Finish the previous level on the learning path first – or unlock all levels there.", "Schließe zuerst das vorherige Level auf dem Lernpfad ab – oder schalte dort alle Level frei.", "Termina primero el nivel anterior del camino, o desbloquea allí todos los niveles.")} }
                Link { to: Route::Home {}, class: "mt-6 inline-flex h-10 items-center gap-2 rounded-xl bg-foreground px-5 text-sm font-semibold text-background", ArrowLeft { class: "size-4" } {tr!(sp, "To the learning path", "Zum Lernpfad", "Al camino de aprendizaje")} }
            }
        };
    }

    let abgeschlossen = f.fertig(&knoten_id);
    let karte = KARTEN.iter().find(|k| k.welt == index);
    let ton_an = f.ton;

    rsx! {
        div { class: "mx-auto flex max-w-6xl gap-8 px-4 py-8",
            // Seitenleiste
            aside { class: "sticky top-20 hidden h-[calc(100vh-6rem)] w-60 shrink-0 overflow-y-auto lg:block",
                div { class: "mb-2 text-xs font-semibold uppercase tracking-wider text-muted-foreground", {tr!(sp, "Theory of the worlds", "Theorie der Welten", "Teoría de los mundos")} }
                nav { class: "space-y-0.5",
                    for (i, lek) in lektionen(sp).iter().enumerate() {
                        if welt::finden(&format!("w{i}-theorie")).is_some_and(|(ki, _)| f.offen(ki)) {
                            Link {
                                key: "{i}",
                                to: Route::LektionSeite { id: i + 1 },
                                class: if i == index { "flex items-center gap-2 rounded-md bg-accent px-2 py-1.5 text-sm font-medium" } else { "flex items-center gap-2 rounded-md px-2 py-1.5 text-sm text-muted-foreground hover:bg-accent hover:text-foreground" },
                                span { class: "w-5 shrink-0 text-right font-mono text-xs", "{i + 1}" }
                                span { class: "truncate", "{lek.titel}" }
                                if f.karte_offen(i) { CircleCheck { class: "ml-auto size-3.5 shrink-0" } }
                            }
                        } else {
                            div { key: "{i}", class: "flex items-center gap-2 px-2 py-1.5 text-sm text-muted-foreground/50",
                                span { class: "w-5 shrink-0 text-right font-mono text-xs", "{i + 1}" }
                                span { class: "truncate", "{lek.titel}" }
                                Lock { class: "ml-auto size-3 shrink-0" }
                            }
                        }
                    }
                }
            }

            article { class: "min-w-0 flex-1",
                div { class: "mb-2 flex items-center gap-2 text-sm text-muted-foreground",
                    Link { to: Route::Home {}, class: "hover:text-foreground", {tr!(sp, "Learning path", "Lernpfad", "Camino")} }
                    span { "/" }
                    span { "{l.kapitel}" }
                }
                div { class: "flex flex-wrap items-end justify-between gap-4",
                    div {
                        div { class: "font-mono text-sm text-muted-foreground", {tr!(sp, format!("World {id} · Level {} · Learn", knoten.nr), format!("Welt {id} · Level {} · Lernen", knoten.nr), format!("Mundo {id} · Nivel {} · Aprender", knoten.nr))} }
                        h1 { class: "mt-1 text-3xl font-bold tracking-tight sm:text-4xl", "{l.titel}" }
                        p { class: "mt-2 text-lg text-muted-foreground", "{l.kurz}" }
                    }
                    if abgeschlossen {
                        span { class: "flex items-center gap-1.5 rounded-full border-2 px-3 py-1 text-sm font-semibold", CircleCheck { class: "size-4" } {tr!(sp, "Read", "Gelesen", "Leído")} }
                    }
                }
                Separator { class: "my-6" }

                div { class: "space-y-5",
                    for (bi, block) in l.bloecke.iter().enumerate() {
                        div { key: "{id}-{bi}",
                            match block {
                                Block::Ueberschrift(t) => rsx! { h2 { class: "pt-4 text-xl font-semibold tracking-tight", "{t}" } },
                                Block::Text(t) => rsx! { RichText { text: t.to_string() } },
                                Block::Code(c) => rsx! { CodeBlock { code: c.to_string() } },
                                Block::Ausfuehrbar(c) => rsx! { CodeRunner { code: c.to_string() } },
                                Block::Tipp(t) => rsx! {
                                    div { class: "flex gap-3 rounded-xl border border-info/30 bg-info-light/50 p-4 text-info-dark",
                                        Info { class: "mt-1 size-4 shrink-0" }
                                        RichText { text: t.to_string(), class: "leading-7" }
                                    }
                                },
                                Block::Achtung(t) => rsx! {
                                    div { class: "flex gap-3 rounded-xl border border-warning/40 bg-warning-light/60 p-4 text-warning-dark",
                                        TriangleAlert { class: "mt-1 size-4 shrink-0" }
                                        RichText { text: t.to_string(), class: "leading-7" }
                                    }
                                },
                                Block::Visual(_) => rsx! { VisualView { lektion: index, block: bi } },
                                Block::Terminal => rsx! { FreiesTerminal {} },
                            }
                        }
                    }
                }

                // Abschluss
                div { class: "mt-12 rounded-2xl border-2 p-6 text-center",
                    if gefeiert() { Konfetti { anzahl: 40 } }
                    if let Some(k) = karte {
                        div { class: "mx-auto mb-3 flex size-14 items-center justify-center rounded-2xl bg-foreground text-background", StickyNote { class: "size-7" } }
                        h3 { class: "text-xl font-extrabold",
                            if abgeschlossen { {tr!(sp, format!("Card “{}” is in your cheat sheet", k.titel(sp)), format!("Karte „{}“ ist in deinem Spickzettel", k.titel(sp)), format!("La tarjeta «{}» está en tu chuleta", k.titel(sp)))} } else { {tr!(sp, "Read everything?", "Alles gelesen?", "¿Lo has leído todo?")} }
                        }
                        p { class: "mx-auto mt-1 max-w-md text-sm text-muted-foreground",
                            if abgeschlossen { {tr!(sp, "On to the exercises – they're generated anew every time.", "Weiter geht's mit den Übungen – die Aufgaben werden jedes Mal neu erzeugt.", "Ahora a los ejercicios: se generan de nuevo cada vez.")} } else { {tr!(sp, format!("Finish the theory and collect the cheat sheet card “{}”. Then the exercises are waiting for you.", k.titel(sp)), format!("Schließ die Theorie ab und sammle die Spickzettel-Karte „{}“. Danach warten die Übungen auf dich.", k.titel(sp)), format!("Termina la teoría y consigue la tarjeta «{}». Después te esperan los ejercicios.", k.titel(sp)))} }
                        }
                    }
                    div { class: "mt-5 flex flex-col justify-center gap-3 sm:flex-row",
                        if !abgeschlossen {
                            button {
                                class: "h-12 rounded-2xl border-2 border-b-4 border-foreground bg-foreground px-8 font-bold uppercase tracking-wide text-background transition active:translate-y-0.5 active:border-b-2",
                                onclick: {
                                    let knoten_id = knoten_id.clone();
                                    move |_| {
                                        abschliessen(fortschritt, &knoten_id, 3, 15, 0, true);
                                        ton(Ton::Geschafft, ton_an);
                                        gefeiert.set(true);
                                    }
                                },
                                {tr!(sp, "Complete · +15 XP", "Abschließen · +15 XP", "Completar · +15 XP")}
                            }
                        } else if let Some(naechster) = welt::alle().get(knoten_index + 1) {
                            Link {
                                to: crate::pages::home::start_route(naechster),
                                class: "flex h-12 items-center justify-center rounded-2xl border-2 border-b-4 border-foreground bg-foreground px-8 font-bold uppercase tracking-wide text-background",
                                {tr!(sp, format!("Next: level {}", naechster.nr), format!("Weiter: Level {}", naechster.nr), format!("Siguiente: nivel {}", naechster.nr))}
                            }
                        }
                        Link { to: Route::Home {}, class: "flex h-12 items-center justify-center rounded-2xl border-2 border-b-4 px-8 font-bold uppercase tracking-wide", {tr!(sp, "To the learning path", "Zum Lernpfad", "Al camino de aprendizaje")} }
                    }
                }
            }
        }
        TaschenSpick {}
    }
}

#[component]
fn FreiesTerminal() -> Element {
    let l = crate::i18n::use_sprache();
    let shell = use_signal(|| Shell::neu(l));
    rsx! { TerminalView { shell, hoehe: "h-72" } }
}
