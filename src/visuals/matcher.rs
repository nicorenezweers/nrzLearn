use dioxus::prelude::*;

use super::VisualRahmen;
use crate::components::code::Highlighted;
use crate::i18n::{Sprache, use_sprache};

/// Texts of the example per language: (sunglasses, storm, umbrella, slush, snow).
fn texte(l: Sprache) -> (&'static str, &'static str, &'static str, &'static str, &'static str) {
    tr!(
        l,
        ("Sunglasses! 😎", "Storm! ⛈️", "Take an umbrella", "Just slush.", "cm of snow ⛄"),
        ("Sonnenbrille! 😎", "Unwetter! ⛈️", "Schirm mit", "Nur Matsch.", "cm Schnee ⛄"),
        ("¡Gafas de sol! 😎", "¡Tormenta! ⛈️", "Lleva paraguas", "Solo aguanieve.", "cm de nieve ⛄")
    )
}

fn code(l: Sprache) -> Vec<String> {
    let (sonne, sturm, schirm, matsch, schnee) = texte(l);
    let mm = tr!(l, "// millimetres", "// Millimeter", "// milímetros");
    vec![
        "enum Weather {".into(),
        "    Sun,".into(),
        format!("    Rain(u32),           {mm}"),
        "    Snow { cm: u32 },".into(),
        "}".into(),
        String::new(),
        "let text = match weather {".into(),
        format!("    Weather::Sun => \"{sonne}\".to_string(),"),
        format!("    Weather::Rain(mm) if mm > 20 => \"{sturm}\".to_string(),"),
        format!("    Weather::Rain(mm) => format!(\"{schirm}, {{mm}} mm 🌧️\"),"),
        format!("    Weather::Snow {{ cm: 0 }} => \"{matsch}\".to_string(),"),
        format!("    Weather::Snow {{ cm }} => format!(\"{{cm}} {schnee}\"),"),
        "};".into(),
    ]
}

#[component]
pub fn MatchDemo() -> Element {
    let l = use_sprache();
    let mut variante = use_signal(|| 1_u8);
    let mut wert = use_signal(|| 12_u32);
    let v = variante();
    let w = wert();
    let (sonne, sturm, schirm, matsch, schnee) = texte(l);

    let (zeile, ausgabe, wert_text) = match v {
        0 => (8, sonne.to_string(), "Weather::Sun".to_string()),
        1 if w > 20 => (9, sturm.to_string(), format!("Weather::Rain({w})")),
        1 => (10, format!("{schirm}, {w} mm 🌧️"), format!("Weather::Rain({w})")),
        _ if w == 0 => (11, matsch.to_string(), "Weather::Snow { cm: 0 }".to_string()),
        _ => (12, format!("{w} {schnee}"), format!("Weather::Snow {{ cm: {w} }}")),
    };
    let knoepfe = tr!(l, ["☀️ Sun", "🌧️ Rain", "❄️ Snow"], ["☀️ Sonne", "🌧️ Regen", "❄️ Schnee"], ["☀️ Sol", "🌧️ Lluvia", "❄️ Nieve"]);

    rsx! {
        VisualRahmen { titel: tr!(l, "match – which branch fits?", "match – welcher Zweig passt?", "match: ¿qué rama encaja?"),
            div { class: "mb-4 flex flex-wrap items-center gap-2",
                for (i, name) in knoepfe.iter().enumerate() {
                    button {
                        key: "{i}",
                        class: if v as usize == i { "rounded-md border-2 border-rust bg-rust-light px-3 py-1.5 text-sm font-medium" } else { "rounded-md border-2 px-3 py-1.5 text-sm hover:bg-accent" },
                        onclick: move |_| variante.set(i as u8),
                        span { class: "emoji", "{name}" }
                    }
                }
                if v != 0 {
                    div { class: "flex items-center gap-2 text-sm",
                        input {
                            r#type: "range",
                            min: "0",
                            max: "40",
                            value: "{w}",
                            class: "accent-[var(--rust)]",
                            oninput: move |e| wert.set(e.value().parse().unwrap_or(0)),
                        }
                        span { class: "w-14 font-mono", if v == 1 { "{w} mm" } else { "{w} cm" } }
                    }
                }
            }
            div { class: "mb-3 font-mono text-sm",
                span { class: "text-muted-foreground", "let weather = " }
                span { class: "font-semibold text-rust", "{wert_text}" }
                ";"
            }
            div { class: "overflow-x-auto rounded-lg border bg-code-bg py-2 font-mono text-[0.8rem] leading-6",
                for (i, z) in code(l).into_iter().enumerate() {
                    div {
                        key: "{i}",
                        class: if i + 1 == zeile { "border-l-4 border-success bg-success/15 pr-3 pl-3" } else if (8..=12).contains(&(i + 1)) && i + 1 < zeile { "border-l-4 border-transparent pr-3 pl-3 opacity-40" } else { "border-l-4 border-transparent pr-3 pl-3" },
                        span { class: "whitespace-pre", Highlighted { code: z } }
                        if i + 1 == zeile { span { class: "ml-2 text-xs font-sans font-semibold text-success", {tr!(l, "← match!", "← passt!", "← ¡encaja!")} } }
                    }
                }
            }
            p { class: "mt-3 text-sm text-muted-foreground",
                {tr!(l, "Rust checks the branches from top to bottom. The first matching one wins. ", "Rust prüft die Zweige von oben nach unten. Der erste passende gewinnt. ", "Rust comprueba las ramas de arriba abajo. Gana la primera que encaja. ")}
                {tr!(l, "Result: ", "Ergebnis: ", "Resultado: ")}
                span { class: "emoji font-mono font-semibold text-foreground", "\"{ausgabe}\"" }
            }
        }
    }
}
