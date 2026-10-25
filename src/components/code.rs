use dioxus::prelude::*;
use icons::{Check, Copy, Play, RotateCcw};

use crate::components::ui::{Button, ButtonSize, ButtonVariant};
use crate::highlight::tokenize;
use crate::i18n::use_sprache;
use crate::playground::{self, Ergebnis};

/// Highlighted code as a sequence of <span>s.
#[component]
pub fn Highlighted(code: String) -> Element {
    rsx! {
        for (i, (class, text)) in tokenize(&code).into_iter().enumerate() {
            span { key: "{i}", class: "{class}", "{text}" }
        }
    }
}

/// Static code block (read-only).
#[component]
pub fn CodeBlock(code: String, #[props(into, optional)] titel: Option<String>) -> Element {
    let l = use_sprache();
    let mut kopiert = use_signal(|| false);
    let code_kopie = code.clone();
    rsx! {
        div { class: "group relative overflow-hidden rounded-xl border bg-code-bg",
            if let Some(t) = titel {
                div { class: "border-b px-4 py-1.5 font-mono text-xs text-muted-foreground", "{t}" }
            }
            button {
                class: "absolute right-2 top-2 rounded-md border bg-background/80 p-1.5 text-muted-foreground opacity-0 transition group-hover:opacity-100 hover:text-foreground",
                title: tr!(l, "Copy", "Kopieren", "Copiar"),
                onclick: move |_| {
                    let js = format!("navigator.clipboard.writeText({})", serde_json::to_string(&code_kopie).unwrap_or_default());
                    document::eval(&js);
                    kopiert.set(true);
                },
                if kopiert() { Check { class: "size-3.5" } } else { Copy { class: "size-3.5" } }
            }
            pre { class: "overflow-x-auto p-4 font-mono text-[0.85rem] leading-6",
                code { Highlighted { code: code.trim_end().to_string() } }
            }
        }
    }
}

/// Editor: transparent textarea on top of highlighted code.
#[component]
pub fn CodeEditor(code: Signal<String>, #[props(default = false)] klein: bool) -> Element {
    let zeilen = code.read().lines().count().max(1) + usize::from(code.read().ends_with('\n'));
    let nummern: String = (1..=zeilen).map(|n| format!("{n}\n")).collect();
    let max_h = if klein { "max-h-72" } else { "max-h-[32rem]" };
    rsx! {
        div { class: "flex overflow-auto bg-code-bg {max_h}",
            pre {
                class: "code-layer select-none border-r !pr-3 text-right text-muted-foreground/50",
                style: "grid-area:auto",
                "aria-hidden": "true",
                "{nummern}"
            }
            div { class: "grid min-w-0 flex-1",
                pre { class: "code-layer", "aria-hidden": "true",
                    Highlighted { code: code() }
                    // Placeholder so an empty last line gets a height
                    " "
                }
                textarea {
                    class: "code-layer w-full",
                    spellcheck: "false",
                    autocomplete: "off",
                    autocapitalize: "off",
                    rows: "{zeilen}",
                    wrap: "off",
                    value: "{code}",
                    oninput: move |e| code.set(e.value()),
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Lauf {
    Leer,
    Laeuft,
    Fertig(Ergebnis),
    Fehler(String),
}

/// Shows the result of a program run in terminal style.
#[component]
pub fn Ausgabe(lauf: Lauf) -> Element {
    let l = use_sprache();
    match lauf {
        Lauf::Leer => rsx! {},
        Lauf::Laeuft => rsx! {
            div { class: "bg-term-bg px-4 py-3 font-mono text-sm text-term-fg/70",
                span { class: "text-success", "   Compiling " }
                "playground v0.1.0 "
                span { class: "anim-blink", "▋" }
            }
        },
        Lauf::Fehler(msg) => rsx! {
            div { class: "bg-term-bg px-4 py-3 font-mono text-sm text-term-fg", {playground::fehlermeldung(l, &msg)} }
        },
        Lauf::Fertig(e) => {
            let meldungen = e.meldungen();
            let kompiliert = e.cargo_zeilen().iter().any(|z| z.contains("Finished"));
            let tipp = if e.success { None } else { playground::fehler_erklaerung(l, &e.stderr) };
            rsx! {
                div { class: "max-h-80 overflow-auto bg-term-bg px-4 py-3 font-mono text-[0.8rem] leading-5 text-term-fg",
                    for (i, z) in e.cargo_zeilen().into_iter().enumerate() {
                        div { key: "c{i}", class: "whitespace-pre text-success/80", "{z}" }
                    }
                    if !e.stdout.is_empty() {
                        div { class: "mt-1 whitespace-pre-wrap text-term-fg", "{e.stdout}" }
                    } else if kompiliert && e.success {
                        div { class: "mt-1 italic text-term-fg/50", {tr!(l, "(no output)", "(keine Ausgabe)", "(sin salida)")} }
                    }
                    if !meldungen.is_empty() {
                        div {
                            class: if e.success { "mt-2 whitespace-pre-wrap text-warning" } else { "mt-2 whitespace-pre-wrap text-term-fg underline decoration-dotted underline-offset-4" },
                            "{meldungen}"
                        }
                    }
                }
                if let Some(t) = tipp {
                    div { class: "flex gap-2 border-t border-warning/30 bg-warning-light px-4 py-2.5 text-sm text-warning-dark",
                        span { "💡" }
                        crate::components::text::RichText { text: t.to_string(), class: "leading-6" }
                    }
                }
            }
        }
    }
}

/// Edit code and run it for real (through the Rust Playground).
#[component]
pub fn CodeRunner(code: String) -> Element {
    let l = use_sprache();
    let start = code.trim_end().to_string() + "\n";
    let mut text = use_signal(|| start.clone());
    let mut lauf = use_signal(|| Lauf::Leer);

    let ausfuehren = move |_| {
        lauf.set(Lauf::Laeuft);
        spawn(async move {
            let code = text();
            lauf.set(match playground::ausfuehren(&code, false).await {
                Ok(e) => Lauf::Fertig(e),
                Err(e) => Lauf::Fehler(e),
            });
        });
    };

    rsx! {
        div { class: "overflow-hidden rounded-xl border",
            div { class: "flex items-center justify-between border-b bg-muted/60 px-3 py-1.5",
                span { class: "font-mono text-xs text-muted-foreground", {tr!(l, "main.rs · editable", "main.rs · editierbar", "main.rs · editable")} }
                div { class: "flex gap-1",
                    Button {
                        variant: ButtonVariant::Ghost,
                        size: ButtonSize::Sm,
                        title: tr!(l, "Reset", "Zurücksetzen", "Restablecer"),
                        onclick: move |_| {
                            text.set(start.clone());
                            lauf.set(Lauf::Leer);
                        },
                        RotateCcw {}
                    }
                    Button {
                        size: ButtonSize::Sm,
                        class: "bg-rust text-background hover:bg-rust/90",
                        disabled: lauf() == Lauf::Laeuft,
                        onclick: ausfuehren,
                        Play {}
                        {tr!(l, "Run", "Ausführen", "Ejecutar")}
                    }
                }
            }
            CodeEditor { code: text, klein: true }
            Ausgabe { lauf: lauf() }
        }
    }
}
