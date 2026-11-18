use dioxus::prelude::*;

use super::VisualRahmen;
use crate::components::ui::{Button, ButtonSize, ButtonVariant};
use crate::i18n::use_sprache;

#[component]
pub fn VecDemo() -> Element {
    let l = use_sprache();
    let mut daten = use_signal(|| vec![10_i32, 20, 30]);
    let mut kap = use_signal(|| 4_usize);
    let mut naechster = use_signal(|| 40_i32);
    let mut log = use_signal(|| vec!["let mut v = vec![10, 20, 30];".to_string()]);
    let mut index = use_signal(|| 1_usize);
    let mut neu_alloziert = use_signal(|| false);

    let len = daten.read().len();
    let k = kap();
    let i = index();
    let get_text = daten.read().get(i).map_or_else(|| "None".to_string(), |w| format!("Some({w})"));

    rsx! {
        VisualRahmen { titel: tr!(l, "Vec<i32> – a growing array on the heap", "Vec<i32> – ein wachsendes Array auf dem Heap", "Vec<i32>: un array que crece en el montón"),
            div { class: "flex flex-wrap items-start gap-6",
                div { class: "rounded-lg border-2 bg-background p-3 font-mono text-xs",
                    div { class: "mb-1 text-[0.65rem] font-semibold uppercase tracking-wider text-muted-foreground", "Stack: v" }
                    div { "ptr → ●" }
                    div { "len = ", span { class: "font-bold text-rust", "{len}" } }
                    div { "cap = ", span { class: "font-bold text-foreground", "{k}" } }
                }
                div { class: "flex-1",
                    div { class: "mb-1 text-[0.65rem] font-semibold uppercase tracking-wider text-muted-foreground",
                        "Heap"
                        if neu_alloziert() {
                            span { class: "anim-pop ml-2 rounded bg-foreground/10 px-1.5 py-0.5 normal-case tracking-normal text-foreground", {tr!(l, "newly allocated & copied!", "neu reserviert & umkopiert!", "¡reservado de nuevo y copiado!")} }
                        }
                    }
                    div { class: "flex flex-wrap gap-1",
                        for slot in 0..k {
                            div { key: "{slot}-{k}", class: "flex flex-col items-center",
                                div {
                                    class: if slot < len {
                                        if slot == i { "anim-pop flex size-11 items-center justify-center rounded-md border-2 border-rust bg-rust font-mono text-sm font-bold text-background" } else { "anim-pop flex size-11 items-center justify-center rounded-md border-2 border-rust bg-rust-light font-mono text-sm font-semibold" }
                                    } else {
                                        "flex size-11 items-center justify-center rounded-md border-2 border-dashed text-muted-foreground/40"
                                    },
                                    if slot < len { "{daten.read()[slot]}" } else { "·" }
                                }
                                span { class: "font-mono text-[0.6rem] text-muted-foreground", "{slot}" }
                            }
                        }
                    }
                }
            }
            div { class: "mt-4 flex flex-wrap gap-2",
                Button {
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        let w = naechster();
                        let mut neu = false;
                        if daten.read().len() == kap() {
                            kap.set(kap() * 2);
                            neu = true;
                        }
                        daten.write().push(w);
                        neu_alloziert.set(neu);
                        naechster.set(w + 10);
                        log.write().push(format!("v.push({w});{}", if neu { tr!(l, "  // capacity doubled!", "  // Kapazität verdoppelt!", "  // ¡capacidad duplicada!") } else { "" }));
                    },
                    "v.push({naechster})"
                }
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        let r = daten.write().pop();
                        neu_alloziert.set(false);
                        log.write().push(format!("v.pop();  // → {}", r.map_or("None".into(), |x| format!("Some({x})"))));
                    },
                    "v.pop()"
                }
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        daten.write().clear();
                        neu_alloziert.set(false);
                        log.write().push(tr!(l, "v.clear();  // len = 0, cap stays!", "v.clear();  // len = 0, cap bleibt!", "v.clear();  // len = 0, ¡cap se queda!").into());
                    },
                    "v.clear()"
                }
            }
            div { class: "mt-4 flex flex-wrap items-center gap-3 rounded-lg bg-muted/70 p-3 text-sm",
                span { {tr!(l, "Index:", "Index:", "Índice:")} }
                input {
                    r#type: "range",
                    min: "0",
                    max: "9",
                    value: "{i}",
                    class: "accent-[var(--rust)]",
                    oninput: move |e| index.set(e.value().parse().unwrap_or(0)),
                }
                code { class: "font-mono", "v.get({i}) = " span { class: "font-bold text-rust", "{get_text}" } }
                code { class: "font-mono",
                    "v[{i}] = "
                    if i < len {
                        span { class: "font-bold text-rust", "{daten.read()[i]}" }
                    } else {
                        span { class: "font-bold text-destructive", "💥 panic: index out of bounds" }
                    }
                }
            }
            div { class: "mt-3 max-h-32 overflow-auto rounded-lg bg-term-bg px-3 py-2 font-mono text-xs text-term-fg",
                for (n, l) in log.read().iter().enumerate().rev() {
                    div { key: "{n}", class: if n + 1 == log.read().len() { "text-term-fg" } else { "text-term-fg/50" }, "{l}" }
                }
            }
        }
    }
}
