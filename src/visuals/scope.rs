use dioxus::prelude::*;

use super::VisualRahmen;
use crate::components::code::Highlighted;
use crate::i18n::use_sprache;

const CODE: &[&str] = &[
    "fn main() {",
    "    let a = 1;",
    "    {",
    "        let b = 2;",
    "        let a = a + 10;   // Shadowing",
    "        println!(\"{a} {b}\");",
    "    }",
    "    println!(\"{a}\");",
    "}",
];

/// Which variables are visible in which line?
const SICHTBAR: &[&[(&str, &str)]] = &[
    &[],
    &[("a", "1")],
    &[("a", "1")],
    &[("a", "1"), ("b", "2")],
    &[("a", "11 *"), ("b", "2")],
    &[("a", "11"), ("b", "2")],
    &[("a", "1")],
    &[("a", "1")],
    &[],
];

const AUSGABE: &[&str] = &["", "", "", "", "", "11 2", "", "1", ""];

#[component]
pub fn ScopeDemo() -> Element {
    let l = use_sprache();
    let mut zeile = use_signal(|| 4_usize);
    let z = zeile();

    rsx! {
        VisualRahmen { titel: tr!(l, "Scopes – click a line", "Gültigkeitsbereiche (Scopes) – klicke auf eine Zeile", "Ámbitos (scopes): haz clic en una línea"),
            div { class: "grid gap-4 lg:grid-cols-2",
                div { class: "overflow-hidden rounded-lg border bg-code-bg py-2 font-mono text-[0.8rem] leading-6",
                    for (i, c) in CODE.iter().enumerate() {
                        button {
                            key: "{i}",
                            class: if i == z { "flex w-full border-l-4 border-rust bg-rust/15 pr-3 text-left" } else if (2..=6).contains(&i) { "flex w-full border-l-4 border-foreground/30 bg-foreground/5 pr-3 text-left hover:bg-accent" } else { "flex w-full border-l-4 border-transparent pr-3 text-left hover:bg-accent" },
                            onclick: move |_| zeile.set(i),
                            span { class: "w-8 shrink-0 select-none pr-2 text-right text-muted-foreground/50", "{i + 1}" }
                            span { class: "whitespace-pre", Highlighted { code: c.to_string() } }
                        }
                    }
                }
                div {
                    div { class: "mb-2 text-xs font-semibold uppercase tracking-wider text-muted-foreground", {tr!(l, format!("Visible in line {}", z + 1), format!("Sichtbar in Zeile {}", z + 1), format!("Visible en la línea {}", z + 1))} }
                    div { class: "flex min-h-24 flex-col gap-2",
                        if SICHTBAR[z].is_empty() {
                            p { class: "text-sm text-muted-foreground", {tr!(l, "No variables visible.", "Keine Variablen sichtbar.", "No hay variables visibles.")} }
                        }
                        for (name, wert) in SICHTBAR[z].iter() {
                            div { key: "{z}-{name}", class: "anim-pop flex items-center gap-3 rounded-md border-2 bg-background px-3 py-2",
                                span { class: "font-mono font-semibold", "{name}" }
                                span { class: "font-mono text-sm text-muted-foreground", "= {wert}" }
                            }
                        }
                    }
                    if !AUSGABE[z].is_empty() {
                        div { class: "mt-3 rounded-lg bg-term-bg px-3 py-2 font-mono text-xs text-term-fg",
                            span { class: "text-term-fg font-semibold", {tr!(l, "Output: ", "Ausgabe: ", "Salida: ")} }
                            "{AUSGABE[z]}"
                        }
                    }
                    p { class: "mt-3 text-xs leading-5 text-muted-foreground",
                        {tr!(l, "The marked block { } is its own scope. Variables created inside disappear at the closing brace. The inner a shadows the outer one only inside the block (* = shadows a = 1).", "Der markierte Block { } ist ein eigener Scope. Variablen, die darin entstehen, verschwinden an der schließenden Klammer. Das innere a verdeckt (Shadowing) das äußere nur innerhalb des Blocks (* = verdeckt a = 1).", "El bloque marcado { } es un ámbito propio. Las variables creadas dentro desaparecen en la llave de cierre. La a interior oculta a la exterior solo dentro del bloque (* = oculta a = 1).")}
                    }
                }
            }
        }
    }
}
