use dioxus::prelude::*;
use icons::ArrowRight;

use super::VisualRahmen;
use crate::components::code::Highlighted;
use crate::i18n::{Sprache, use_sprache};

const FILTER_EN: &[(&str, &str)] = &[("–", ""), ("even", ".filter(|x| x % 2 == 0)"), ("odd", ".filter(|x| x % 2 == 1)"), ("> 5", ".filter(|&x| x > 5)")];
const FILTER_DE: &[(&str, &str)] = &[("–", ""), ("gerade", ".filter(|x| x % 2 == 0)"), ("ungerade", ".filter(|x| x % 2 == 1)"), ("> 5", ".filter(|&x| x > 5)")];
const FILTER_ES: &[(&str, &str)] = &[("–", ""), ("pares", ".filter(|x| x % 2 == 0)"), ("impares", ".filter(|x| x % 2 == 1)"), ("> 5", ".filter(|&x| x > 5)")];

fn filter_optionen(l: Sprache) -> &'static [(&'static str, &'static str)] {
    tr!(l, FILTER_EN, FILTER_DE, FILTER_ES)
}
const MAP: &[(&str, &str)] = &[("–", ""), ("x * x", ".map(|x| x * x)"), ("x * 10", ".map(|x| x * 10)"), ("x + 1", ".map(|x| x + 1)")];
const ENDE: &[(&str, &str)] = &[("collect", ".collect::<Vec<_>>()"), ("sum", ".sum::<i32>()"), ("count", ".count()"), ("max", ".max()")];

fn filtern(f: usize, x: i32) -> bool {
    match f {
        1 => x % 2 == 0,
        2 => x % 2 == 1,
        3 => x > 5,
        _ => true,
    }
}

fn abbilden(m: usize, x: i32) -> i32 {
    match m {
        1 => x * x,
        2 => x * 10,
        3 => x + 1,
        _ => x,
    }
}

#[component]
fn Auswahl(titel: &'static str, optionen: &'static [(&'static str, &'static str)], wahl: Signal<usize>) -> Element {
    rsx! {
        div {
            div { class: "mb-1 text-xs font-semibold text-muted-foreground", "{titel}" }
            div { class: "flex flex-wrap gap-1",
                for (i, (name, _)) in optionen.iter().enumerate() {
                    button {
                        key: "{i}",
                        class: if wahl() == i { "rounded-md border-2 border-rust bg-rust-light px-2.5 py-1 font-mono text-xs font-semibold" } else { "rounded-md border-2 px-2.5 py-1 font-mono text-xs hover:bg-accent" },
                        onclick: move |_| wahl.set(i),
                        "{name}"
                    }
                }
            }
        }
    }
}

#[component]
fn Chips(titel: String, werte: Vec<i32>, alle: Option<Vec<i32>>) -> Element {
    rsx! {
        div { class: "rounded-lg bg-muted/60 p-2",
            div { class: "mb-1 font-mono text-[0.7rem] text-muted-foreground", "{titel}" }
            div { class: "flex flex-wrap gap-1",
                if let Some(alle) = alle {
                    for (i, x) in alle.iter().enumerate() {
                        span {
                            key: "{i}-{x}",
                            class: if werte.contains(x) { "anim-pop rounded bg-rust px-2 py-0.5 font-mono text-xs font-semibold text-background" } else { "rounded border border-dashed px-2 py-0.5 font-mono text-xs text-muted-foreground line-through opacity-50" },
                            "{x}"
                        }
                    }
                } else {
                    for (i, x) in werte.iter().enumerate() {
                        span { key: "{i}-{x}", class: "anim-pop rounded bg-rust px-2 py-0.5 font-mono text-xs font-semibold text-background", "{x}" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn IterDemo() -> Element {
    let l = use_sprache();
    let filter_opt = filter_optionen(l);
    let filter = use_signal(|| 1_usize);
    let map = use_signal(|| 1_usize);
    let ende = use_signal(|| 1_usize);

    let quelle: Vec<i32> = (1..=10).collect();
    let gefiltert: Vec<i32> = quelle.iter().copied().filter(|&x| filtern(filter(), x)).collect();
    let abgebildet: Vec<i32> = gefiltert.iter().map(|&x| abbilden(map(), x)).collect();
    let ergebnis = match ende() {
        0 => format!("{abgebildet:?}"),
        1 => abgebildet.iter().sum::<i32>().to_string(),
        2 => abgebildet.len().to_string(),
        _ => format!("{:?}", abgebildet.iter().max()),
    };
    let code = format!("let result = (1..=10){}{}{};", filter_opt[filter()].1, MAP[map()].1, ENDE[ende()].1);

    rsx! {
        VisualRahmen { titel: tr!(l, "Build an iterator chain", "Iterator-Kette zusammenbauen", "Construye una cadena de iteradores"),
            div { class: "grid gap-3 sm:grid-cols-3",
                Auswahl { titel: "1. filter", optionen: filter_opt, wahl: filter }
                Auswahl { titel: "2. map", optionen: MAP, wahl: map }
                Auswahl { titel: tr!(l, "3. finish", "3. Abschluss", "3. final"), optionen: ENDE, wahl: ende }
            }
            div { class: "mt-4 overflow-x-auto rounded-lg border bg-code-bg px-3 py-2 font-mono text-[0.8rem]",
                Highlighted { code }
            }
            div { class: "mt-4 flex flex-col gap-2 lg:flex-row lg:items-stretch",
                div { class: "flex-1", Chips { titel: "(1..=10)", werte: quelle.clone(), alle: None } }
                ArrowRight { class: "size-4 self-center text-muted-foreground max-lg:rotate-90" }
                div { class: "flex-1", Chips { titel: "filter", werte: gefiltert.clone(), alle: Some(quelle.clone()) } }
                ArrowRight { class: "size-4 self-center text-muted-foreground max-lg:rotate-90" }
                div { class: "flex-1", Chips { titel: "map", werte: abgebildet.clone(), alle: None } }
                ArrowRight { class: "size-4 self-center text-muted-foreground max-lg:rotate-90" }
                div { class: "flex flex-col justify-center rounded-lg border-2 border-rust bg-rust-light p-2 text-center lg:w-36",
                    div { class: "font-mono text-[0.7rem] text-muted-foreground", "{ENDE[ende()].0}" }
                    div { class: "break-all font-mono text-sm font-bold", "{ergebnis}" }
                }
            }
            p { class: "mt-3 text-xs text-muted-foreground",
                {tr!(l, "Iterators are lazy: filter and map only do something once collect, sum, count or similar pull the values at the end.", "Iteratoren sind „faul“ (lazy): filter und map tun erst etwas, wenn am Ende collect, sum, count o.ä. die Werte abholt.", "Los iteradores son perezosos: filter y map solo actúan cuando al final collect, sum, count o similares recogen los valores.")}
            }
        }
    }
}
