use dioxus::prelude::*;

use super::VisualRahmen;
use crate::components::code::Highlighted;
use crate::i18n::use_sprache;

#[component]
pub fn ParseDemo() -> Element {
    let l = use_sprache();
    let mut eingabe = use_signal(|| "42".to_string());
    let text = eingabe();
    let ergebnis = text.parse::<u8>();

    let (ok, darstellung, zweig) = match &ergebnis {
        Ok(n) => {
            let d = u16::from(*n) * 2;
            (true, format!("Ok({n})"), tr!(l, format!("Ok(number) => number is {n}, doubled: {d}"), format!("Ok(zahl) => zahl wird {n}, verdoppelt: {d}"), format!("Ok(numero) => numero es {n}, el doble: {d}")))
        }
        Err(e) => (false, "Err(ParseIntError)".to_string(), tr!(l, format!("Err(e) => error: \"{e}\""), format!("Err(e) => Fehler: \"{e}\""), format!("Err(e) => error: \"{e}\""))),
    };

    rsx! {
        VisualRahmen { titel: tr!(l, "Result<T, E> – success or error?", "Result<T, E> – Erfolg oder Fehler?", "Result<T, E>: ¿éxito o error?"),
            div { class: "flex flex-wrap items-center gap-2 text-sm",
                span { {tr!(l, "Input:", "Eingabe:", "Entrada:")} }
                input {
                    class: "h-9 w-40 rounded-md border bg-background px-3 font-mono outline-none focus-visible:ring-2 focus-visible:ring-rust/50",
                    value: "{text}",
                    oninput: move |e| eingabe.set(e.value()),
                }
                for beispiel in ["42", "255", "256", "-1", "abc", ""] {
                    button {
                        key: "{beispiel}",
                        class: "rounded border px-2 py-0.5 font-mono text-xs hover:bg-accent",
                        onclick: move |_| eingabe.set(beispiel.to_string()),
                        if beispiel.is_empty() { "\"\"" } else { "{beispiel}" }
                    }
                }
            }
            div { class: "mt-4 overflow-x-auto rounded-lg border bg-code-bg px-3 py-2 font-mono text-[0.8rem]",
                Highlighted { code: format!("let result: Result<u8, ParseIntError> = \"{text}\".parse::<u8>();") }
            }
            div { class: "mt-4 grid gap-3 sm:grid-cols-2",
                div { class: if ok { "rounded-lg border-2 border-success bg-success-light/60 p-3" } else { "rounded-lg border-2 border-dashed p-3 opacity-40" },
                    div { class: "font-mono text-sm font-bold text-success-dark", "Ok(T)" }
                    div { class: "text-xs text-muted-foreground", {tr!(l, "It worked – the value is inside.", "Es hat geklappt – drin steckt der Wert.", "Ha funcionado: dentro está el valor.")} }
                }
                div { class: if ok { "rounded-lg border-2 border-dashed p-3 opacity-40" } else { "rounded-lg border-2 border-destructive bg-destructive-light p-3" },
                    div { class: "font-mono text-sm font-bold text-destructive", "Err(E)" }
                    div { class: "text-xs text-muted-foreground", {tr!(l, "Something went wrong – the error is inside.", "Etwas ging schief – drin steckt der Fehler.", "Algo salió mal: dentro está el error.")} }
                }
            }
            div { class: "mt-3 rounded-lg bg-muted/70 p-3 font-mono text-sm",
                div { "result = ", span { class: if ok { "font-bold text-success-dark" } else { "font-bold text-destructive" }, "{darstellung}" } }
                div { class: "mt-1 text-xs text-muted-foreground", "match → {zweig}" }
            }
            p { class: "mt-3 text-xs text-muted-foreground",
                {tr!(l, "Try 256: a u8 only goes up to 255, so you get an error instead of a crash. Rust makes you handle both cases.", "Probiere 256: Ein u8 geht nur bis 255, also gibt es einen Fehler statt eines Absturzes. Rust zwingt dich, beide Fälle zu behandeln.", "Prueba 256: un u8 solo llega a 255, así que obtienes un error en vez de un fallo. Rust te obliga a tratar ambos casos.")}
            }
        }
    }
}
