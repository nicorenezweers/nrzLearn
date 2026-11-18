use dioxus::prelude::*;

use super::VisualRahmen;
use crate::i18n::{Sprache, use_sprache};

fn typen(l: Sprache) -> [(&'static str, u32, &'static str); 12] {
    [
        ("bool", 1, tr!(l, "true or false", "true oder false", "true o false")),
        ("u8 / i8", 1, tr!(l, "0..=255 or -128..=127", "0..=255 bzw. -128..=127", "0..=255 o -128..=127")),
        ("u16 / i16", 2, tr!(l, "up to about 65 thousand", "bis ca. 65 Tausend", "hasta unos 65 mil")),
        ("char", 4, tr!(l, "one Unicode character, e.g. 'ß' or '🦀'", "ein Unicode-Zeichen, z.B. 'ß' oder '🦀'", "un carácter Unicode, p. ej. 'ñ' o '🦀'")),
        ("u32 / i32", 4, tr!(l, "default for whole numbers (i32)", "Standard für ganze Zahlen (i32)", "por defecto para enteros (i32)")),
        ("f32", 4, tr!(l, "decimal number, ~7 digits precise", "Kommazahl, ~7 Stellen genau", "número decimal, ~7 cifras de precisión")),
        ("u64 / i64", 8, tr!(l, "very large whole numbers", "sehr große ganze Zahlen", "enteros muy grandes")),
        ("usize", 8, tr!(l, "sizes & indices (on 64-bit systems)", "Größen & Indizes (auf 64-Bit-Systemen)", "tamaños e índices (en sistemas de 64 bits)")),
        ("f64", 8, tr!(l, "default for decimals, ~15 digits", "Standard für Kommazahlen, ~15 Stellen", "por defecto para decimales, ~15 cifras")),
        ("&str", 16, tr!(l, "pointer + length (fat pointer)", "Zeiger + Länge (Fat Pointer)", "puntero + longitud (fat pointer)")),
        ("u128 / i128", 16, tr!(l, "huge numbers", "riesige Zahlen", "números enormes")),
        ("String", 24, tr!(l, "pointer + length + capacity (data on the heap)", "Zeiger + Länge + Kapazität (Daten auf dem Heap)", "puntero + longitud + capacidad (datos en el montón)")),
    ]
}

#[component]
pub fn Typgroessen() -> Element {
    let l = use_sprache();
    let typen = typen(l);
    let mut aktiv = use_signal(|| None::<usize>);
    rsx! {
        VisualRahmen { titel: tr!(l, "How much memory does a type need? (bytes on the stack)", "Wie viel Speicher braucht ein Typ? (Bytes auf dem Stack)", "¿Cuánta memoria necesita un tipo? (bytes en la pila)"),
            div { class: "space-y-1.5",
                for (i, (name, bytes, _)) in typen.iter().enumerate() {
                    div {
                        key: "{i}",
                        class: "flex cursor-default items-center gap-3",
                        onmouseenter: move |_| aktiv.set(Some(i)),
                        onmouseleave: move |_| aktiv.set(None),
                        span { class: "w-24 shrink-0 text-right font-mono text-xs sm:text-sm", "{name}" }
                        div { class: "flex flex-1 gap-0.5",
                            for b in 0..*bytes {
                                span {
                                    key: "{b}",
                                    class: if aktiv() == Some(i) { "h-5 flex-1 max-w-6 rounded-sm bg-rust" } else { "h-5 flex-1 max-w-6 rounded-sm bg-rust/50" },
                                }
                            }
                        }
                        span { class: "w-16 shrink-0 font-mono text-xs text-muted-foreground",
                            "{bytes} B · {bytes * 8} bit"
                        }
                    }
                }
            }
            p { class: "mt-4 min-h-6 text-sm text-muted-foreground",
                if let Some(i) = aktiv() {
                    span { class: "font-mono text-foreground", "{typen[i].0}" }
                    ": {typen[i].2}"
                } else {
                    {tr!(l, "Hover over a type. 1 byte = 8 bits.", "Fahre mit der Maus über einen Typ. 1 Byte = 8 Bit.", "Pasa el ratón por encima de un tipo. 1 byte = 8 bits.")}
                }
            }
        }
    }
}
