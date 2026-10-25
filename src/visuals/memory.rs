use dioxus::prelude::*;
use icons::{ChevronLeft, ChevronRight};

use super::VisualRahmen;
use crate::components::code::Highlighted;
use crate::components::ui::{Button, ButtonSize, ButtonVariant};
use crate::content::{Block, Status, Visual, lektionen};
use crate::i18n::use_sprache;

const FARBEN: &[&str] = &["bg-foreground", "border-2 border-foreground bg-transparent", "bg-muted-foreground", "border-2 border-dashed border-foreground"];

/// Step-by-step view of stack and heap.
#[component]
pub fn Speicher(lektion: usize, block: usize) -> Element {
    let l = use_sprache();
    let Block::Visual(Visual::Speicher { code, schritte }) = &lektionen(l)[lektion].bloecke[block] else {
        return rsx! {};
    };
    let mut pos = use_signal(|| 0_usize);
    let p = pos().min(schritte.len() - 1);
    let s = &schritte[p];

    rsx! {
        VisualRahmen { titel: tr!(l, "Stack & heap – step by step", "Stack & Heap – Schritt für Schritt", "Pila y montón, paso a paso"),
            div { class: "grid gap-4 lg:grid-cols-2",
                // Code mit markierter Zeile
                div { class: "overflow-hidden rounded-lg border bg-code-bg py-2 font-mono text-[0.8rem] leading-6",
                    for (i, zeile) in code.lines().enumerate() {
                        div {
                            key: "{i}",
                            class: if i + 1 == s.zeile { "flex border-l-4 border-rust bg-rust/15 pr-3" } else { "flex border-l-4 border-transparent pr-3" },
                            span { class: "w-8 shrink-0 select-none pr-2 text-right text-muted-foreground/50", "{i + 1}" }
                            span { class: "whitespace-pre", Highlighted { code: zeile.to_string() } }
                        }
                    }
                }
                div { class: "grid grid-cols-2 gap-3",
                    div {
                        div { class: "mb-1.5 text-xs font-semibold uppercase tracking-wider text-muted-foreground", "Stack" }
                        div { class: "flex min-h-32 flex-col-reverse justify-end gap-1.5 rounded-lg border-2 border-dashed p-2",
                            for (i, v) in s.stack.iter().enumerate() {
                                div {
                                    key: "{p}-{i}",
                                    class: match v.status {
                                        Status::Neu => "anim-pop rounded-md border-2 border-rust bg-rust-light p-2",
                                        Status::Verschoben | Status::Freigegeben => "rounded-md border-2 border-dashed p-2 opacity-45",
                                        Status::Geliehen => "rounded-md border-2 border-foreground/60 bg-foreground/10 p-2",
                                        Status::Gueltig => "rounded-md border-2 bg-background p-2",
                                    },
                                    div { class: "flex items-center justify-between gap-1",
                                        span { class: "font-mono text-sm font-semibold", "{v.name}" }
                                        if let Some(h) = v.heap {
                                            span { class: "size-2.5 shrink-0 rounded-full {FARBEN[h % FARBEN.len()]}" }
                                        }
                                    }
                                    div {
                                        class: if matches!(v.status, Status::Verschoben) { "font-mono text-xs line-through" } else { "font-mono text-xs text-muted-foreground" },
                                        "{v.wert}"
                                    }
                                    if v.status == Status::Verschoben {
                                        div { class: "text-[0.65rem] font-medium text-destructive", {tr!(l, "invalid (moved)", "ungültig (verschoben)", "inválida (movida)")} }
                                    }
                                    if v.status == Status::Freigegeben {
                                        div { class: "text-[0.65rem] font-medium text-muted-foreground", {tr!(l, "cleaned up (drop)", "aufgeräumt (drop)", "liberada (drop)")} }
                                    }
                                }
                            }
                        }
                    }
                    div {
                        div { class: "mb-1.5 text-xs font-semibold uppercase tracking-wider text-muted-foreground", "Heap" }
                        div { class: "flex min-h-32 flex-col gap-1.5 rounded-lg border-2 border-dashed p-2",
                            for (i, h) in s.heap.iter().enumerate() {
                                div {
                                    key: "{p}-h{i}",
                                    class: if h.status == Status::Freigegeben { "rounded-md border-2 border-dashed p-2 opacity-40" } else if h.status == Status::Neu { "anim-pop rounded-md border-2 border-rust bg-rust-light p-2" } else { "rounded-md border-2 bg-background p-2" },
                                    div { class: "flex items-center gap-1.5",
                                        span { class: "size-2.5 shrink-0 rounded-full {FARBEN[i % FARBEN.len()]}" }
                                        span { class: "break-all font-mono text-xs", "{h.inhalt}" }
                                    }
                                    if h.status == Status::Freigegeben {
                                        div { class: "text-[0.65rem] text-muted-foreground", {tr!(l, "freed", "freigegeben", "liberado")} }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div { class: "mt-4 rounded-lg bg-muted/70 p-3 text-sm leading-6",
                span { class: "mr-2 font-mono text-xs text-muted-foreground", {tr!(l, format!("Step {}/{}", p + 1, schritte.len()), format!("Schritt {}/{}", p + 1, schritte.len()), format!("Paso {}/{}", p + 1, schritte.len()))} }
                crate::components::text::RichText { text: s.text.to_string(), class: "inline" }
            }
            if !s.ausgabe.is_empty() {
                div { class: "mt-2 rounded-lg bg-term-bg px-3 py-2 font-mono text-xs text-term-fg",
                    span { class: "text-term-fg font-semibold", {tr!(l, "Output: ", "Ausgabe: ", "Salida: ")} }
                    "{s.ausgabe}"
                }
            }
            div { class: "mt-3 flex items-center justify-between",
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, disabled: p == 0, onclick: move |_| pos.set(p.saturating_sub(1)), ChevronLeft {} {tr!(l, "Back", "Zurück", "Atrás")} }
                div { class: "flex gap-1",
                    for i in 0..schritte.len() {
                        button {
                            key: "{i}",
                            class: if i == p { "h-2 w-5 rounded-full bg-rust" } else { "size-2 rounded-full bg-muted-foreground/30" },
                            onclick: move |_| pos.set(i),
                        }
                    }
                }
                Button { size: ButtonSize::Sm, disabled: p + 1 >= schritte.len(), onclick: move |_| pos.set(p + 1), {tr!(l, "Next", "Weiter", "Siguiente")} ChevronRight {} }
            }
        }
    }
}
