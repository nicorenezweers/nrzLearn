use dioxus::prelude::*;
use icons::{ChevronLeft, ChevronRight, Play};

use super::VisualRahmen;
use crate::components::code::Highlighted;
use crate::components::ui::{Button, ButtonSize, ButtonVariant};
use crate::content::{Block, Visual, lektionen};
use crate::i18n::use_sprache;

/// Follows variable values line by line (e.g. in loops).
#[component]
pub fn Trace(lektion: usize, block: usize) -> Element {
    let l = use_sprache();
    let Block::Visual(Visual::Trace { code, spalten, schritte }) = &lektionen(l)[lektion].bloecke[block] else {
        return rsx! {};
    };
    let mut pos = use_signal(|| 0_usize);
    let mut auto = use_signal(|| false);
    let p = pos().min(schritte.len() - 1);
    let s = &schritte[p];
    let n = schritte.len();

    // Automatic playback
    use_future(move || async move {
        loop {
            gloo_timers::future::TimeoutFuture::new(700).await;
            if auto() {
                if pos() + 1 < n {
                    pos += 1;
                } else {
                    auto.set(false);
                }
            }
        }
    });

    rsx! {
        VisualRahmen { titel: tr!(l, "Follow the program flow", "Programm-Ablauf verfolgen", "Sigue el flujo del programa"),
            div { class: "grid gap-4 lg:grid-cols-2",
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
                div { class: "overflow-auto rounded-lg border",
                    table { class: "w-full text-sm",
                        thead { class: "bg-muted/70",
                            tr {
                                th { class: "px-3 py-1.5 text-left text-xs font-medium text-muted-foreground", "#" }
                                for sp in spalten.iter() {
                                    th { key: "{sp}", class: "px-3 py-1.5 text-left font-mono text-xs font-semibold", "{sp}" }
                                }
                            }
                        }
                        tbody {
                            for (i, schritt) in schritte.iter().enumerate().take(p + 1) {
                                tr {
                                    key: "{i}",
                                    class: if i == p { "anim-pop bg-rust/10" } else { "border-t" },
                                    td { class: "px-3 py-1 font-mono text-xs text-muted-foreground", "{i + 1}" }
                                    for (j, w) in schritt.werte.iter().enumerate() {
                                        td { key: "{j}", class: "px-3 py-1 font-mono", "{w}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div { class: "mt-4 min-h-12 rounded-lg bg-muted/70 p-3 text-sm leading-6",
                crate::components::text::RichText { text: s.text.to_string(), class: "inline" }
            }
            div { class: "mt-3 flex items-center justify-between gap-2",
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, disabled: p == 0, onclick: move |_| { auto.set(false); pos.set(p.saturating_sub(1)); }, ChevronLeft {} {tr!(l, "Back", "Zurück", "Atrás")} }
                Button {
                    variant: ButtonVariant::Secondary,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        if pos() + 1 >= n { pos.set(0); }
                        auto.set(!auto());
                    },
                    Play {}
                    if auto() { {tr!(l, "Pause", "Pause", "Pausa")} } else { {tr!(l, "Play", "Abspielen", "Reproducir")} }
                }
                Button { size: ButtonSize::Sm, disabled: p + 1 >= n, onclick: move |_| { auto.set(false); pos.set(p + 1); }, {tr!(l, "Next", "Weiter", "Siguiente")} ChevronRight {} }
            }
        }
    }
}
