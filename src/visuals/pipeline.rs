use dioxus::prelude::*;
use icons::{ChevronRight, RotateCcw};

use super::VisualRahmen;
use crate::components::ui::{Button, ButtonSize, ButtonVariant};
use crate::i18n::{Sprache, use_sprache};

fn schritte(l: Sprache) -> [(&'static str, &'static str, &'static str); 5] {
    tr!(
        l,
        [
            ("📝", "main.rs", "You write Rust source code in a text file ending in .rs."),
            ("📦", "cargo build", "Cargo – Rust's project tool – reads Cargo.toml and calls the compiler rustc."),
            ("🔍", "Check", "The compiler checks types, ownership and borrowing. If it finds a mistake, it stops and explains the problem – before the program even runs!"),
            ("⚙️", "Machine code", "If everything is correct, rustc (with the help of LLVM) translates your code into machine code for your processor."),
            ("🚀", "./hello", "The result is a standalone, fast program file. It needs no runtime and runs directly."),
        ],
        [
            ("📝", "main.rs", "Du schreibst Rust-Quellcode in eine Textdatei mit der Endung .rs."),
            ("📦", "cargo build", "Cargo – Rusts Werkzeug für Projekte – liest Cargo.toml und ruft den Compiler rustc auf."),
            ("🔍", "Prüfen", "Der Compiler prüft Typen, Ownership und Borrowing. Findet er einen Fehler, bricht er ab und erklärt dir das Problem – bevor das Programm überhaupt läuft!"),
            ("⚙️", "Maschinencode", "Ist alles korrekt, übersetzt rustc (mit Hilfe von LLVM) deinen Code in Maschinencode für deinen Prozessor."),
            ("🚀", "./hallo", "Es entsteht eine eigenständige, schnelle Programmdatei. Sie braucht keine Laufzeitumgebung und läuft direkt."),
        ],
        [
            ("📝", "main.rs", "Escribes código fuente Rust en un archivo de texto que termina en .rs."),
            ("📦", "cargo build", "Cargo, la herramienta de proyectos de Rust, lee Cargo.toml y llama al compilador rustc."),
            ("🔍", "Comprobar", "El compilador comprueba tipos, ownership y préstamos. Si encuentra un error, se detiene y te explica el problema, ¡antes de que el programa llegue a ejecutarse!"),
            ("⚙️", "Código máquina", "Si todo está bien, rustc (con ayuda de LLVM) traduce tu código a código máquina para tu procesador."),
            ("🚀", "./hola", "Se obtiene un programa independiente y rápido. No necesita ningún entorno de ejecución y funciona directamente."),
        ]
    )
}

#[component]
pub fn Pipeline() -> Element {
    let l = use_sprache();
    let schritte = schritte(l);
    let mut schritt = use_signal(|| 0_usize);
    let s = schritt();

    rsx! {
        VisualRahmen { titel: tr!(l, "From source code to program", "Vom Quellcode zum Programm", "Del código fuente al programa"),
            div { class: "flex flex-wrap items-center justify-center gap-1 sm:gap-2",
                for (i, (icon, name, _)) in schritte.iter().enumerate() {
                    div { key: "{i}", class: "flex items-center gap-1 sm:gap-2",
                        button {
                            class: if i == s {
                                "anim-pop flex w-20 flex-col items-center gap-1 rounded-xl border-2 border-rust bg-rust-light p-2 shadow-md sm:w-24"
                            } else if i < s {
                                "flex w-20 flex-col items-center gap-1 rounded-xl border-2 border-success/50 bg-success-light/40 p-2 sm:w-24"
                            } else {
                                "flex w-20 flex-col items-center gap-1 rounded-xl border-2 border-dashed p-2 opacity-50 sm:w-24"
                            },
                            onclick: move |_| schritt.set(i),
                            span { class: "emoji text-2xl", "{icon}" }
                            span { class: "font-mono text-[0.65rem] sm:text-xs", "{name}" }
                        }
                        if i + 1 < schritte.len() {
                            ChevronRight { class: if i < s { "size-4 text-success" } else { "size-4 text-muted-foreground/40" } }
                        }
                    }
                }
            }
            p { class: "mt-4 min-h-12 text-center text-sm leading-6", "{schritte[s].2}" }
            div { class: "mt-3 flex justify-center gap-2",
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    onclick: move |_| schritt.set(0),
                    RotateCcw {}
                    {tr!(l, "From the start", "Von vorn", "Desde el principio")}
                }
                Button {
                    size: ButtonSize::Sm,
                    disabled: s + 1 >= 5,
                    onclick: move |_| schritt.set((s + 1).min(4)),
                    {tr!(l, "Next step", "Nächster Schritt", "Siguiente paso")}
                    ChevronRight {}
                }
            }
        }
    }
}
