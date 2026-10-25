use dioxus::prelude::*;
use icons::{Code, File, Folder, Play, Sparkles};

use crate::components::code::{CodeEditor, CodeRunner};
use crate::components::terminal::TerminalView;
use crate::components::ui::{Button, ButtonSize, ButtonVariant, Tabs, TabsContent, TabsList, TabsTrigger};
use crate::i18n::{Sprache, use_sprache};
use crate::shell::{HOME, Shell};

fn spielwiese(l: Sprache) -> &'static str {
    tr!(
        l,
        r#"// Your playground: write any Rust code here!
fn main() {
    let names = ["Ferris", "Ada", "Linus"];
    for (i, name) in names.iter().enumerate() {
        println!("{}. Hello, {name}!", i + 1);
    }

    let sum: i32 = (1..=100).sum();
    println!("1 + 2 + ... + 100 = {sum}");
}
"#,
        r#"// Deine Spielwiese: schreib hier beliebigen Rust-Code!
fn main() {
    let namen = ["Ferris", "Ada", "Linus"];
    for (i, name) in namen.iter().enumerate() {
        println!("{}. Hallo, {name}!", i + 1);
    }

    let summe: i32 = (1..=100).sum();
    println!("1 + 2 + ... + 100 = {summe}");
}
"#,
        r#"// Tu zona de pruebas: ¡escribe aquí el código Rust que quieras!
fn main() {
    let nombres = ["Ferris", "Ada", "Linus"];
    for (i, nombre) in nombres.iter().enumerate() {
        println!("{}. ¡Hola, {nombre}!", i + 1);
    }

    let suma: i32 = (1..=100).sum();
    println!("1 + 2 + ... + 100 = {suma}");
}
"#
    )
}

#[component]
pub fn TerminalSeite() -> Element {
    let l = use_sprache();
    rsx! {
        div { class: "mx-auto max-w-6xl px-4 py-8",
            h1 { class: "text-3xl font-bold tracking-tight", {tr!(l, "Terminal & playground", "Terminal & Spielwiese", "Terminal y zona de pruebas")} }
            p { class: "mt-2 text-muted-foreground",
                {tr!(l, "Practise freely here: create projects with cargo, edit files in the editor and run them – the code is really compiled.", "Hier kannst du frei üben: Lege Projekte mit cargo an, bearbeite Dateien im Editor und führe sie aus – der Code wird wirklich kompiliert.", "Practica libremente: crea proyectos con cargo, edita archivos en el editor y ejecútalos; el código se compila de verdad.")}
            }
            Tabs { default_value: "projekt", class: "mt-6",
                TabsList {
                    TabsTrigger { value: "projekt", class: "px-3", {tr!(l, "Project: terminal + editor", "Projekt: Terminal + Editor", "Proyecto: terminal + editor")} }
                    TabsTrigger { value: "spielwiese", class: "px-3", {tr!(l, "Quick playground", "Schnelle Spielwiese", "Zona de pruebas rápida")} }
                }
                TabsContent { value: "projekt", Projekt {} }
                TabsContent { value: "spielwiese",
                    p { class: "mb-3 text-sm text-muted-foreground", {tr!(l, "No project, no terminal: just write code and click “Run”.", "Ohne Projekt und Terminal: einfach Code schreiben und auf „Ausführen“ klicken.", "Sin proyecto ni terminal: escribe código y pulsa «Ejecutar».")} }
                    for sp in std::iter::once(l) {
                        CodeRunner { key: "{sp.code()}", code: spielwiese(sp) }
                    }
                }
            }
        }
    }
}

#[component]
fn Projekt() -> Element {
    let l = use_sprache();
    let mut shell = use_signal(|| Shell::neu(l));
    let mut datei = use_signal(|| None::<String>);
    let mut code = use_signal(String::new);
    let mut extern_befehl = use_signal(|| None::<String>);

    // Write editor changes back into the virtual file system.
    use_effect(move || {
        let inhalt = code();
        if let Some(p) = datei.peek().clone() {
            let mut s = shell.write();
            if s.dateien.get(&p) != Some(&inhalt) {
                s.dateien.insert(p, inhalt);
            }
        }
    });

    let mut oeffnen = move |pfad: String| {
        let inhalt = shell.read().dateien.get(&pfad).cloned().unwrap_or_default();
        datei.set(Some(pfad));
        code.set(inhalt);
    };

    let mut befehle = move |liste: &[&str]| {
        for b in liste {
            let _ = shell.write().befehl(b);
        }
    };

    let dateien: Vec<String> = shell
        .read()
        .dateien
        .iter()
        .filter(|(k, v)| k.starts_with(HOME) && !v.starts_with("#BINARY"))
        .map(|(k, _)| k.clone())
        .collect();
    let hat_projekt = shell.read().aktuelle_main().is_some();

    rsx! {
        div { class: "grid gap-4 lg:grid-cols-[14rem_1fr]",
            // Dateien
            div { class: "rounded-xl border bg-card p-3",
                div { class: "mb-2 flex items-center gap-1.5 text-xs font-semibold uppercase tracking-wider text-muted-foreground",
                    Folder { class: "size-3.5" }
                    "~ (home/ferris)"
                }
                div { class: "space-y-0.5",
                    for d in dateien {
                        button {
                            key: "{d}",
                            class: if datei().as_ref() == Some(&d) { "flex w-full items-center gap-1.5 rounded px-2 py-1 text-left font-mono text-xs bg-accent font-semibold" } else { "flex w-full items-center gap-1.5 rounded px-2 py-1 text-left font-mono text-xs text-muted-foreground hover:bg-accent hover:text-foreground" },
                            onclick: {
                                let d = d.clone();
                                move |_| oeffnen(d.clone())
                            },
                            File { class: "size-3 shrink-0" }
                            span { class: "truncate", {d.strip_prefix(HOME).unwrap_or(&d).trim_start_matches('/').to_string()} }
                        }
                    }
                }
                if !hat_projekt {
                    Button {
                        variant: ButtonVariant::Outline,
                        size: ButtonSize::Sm,
                        class: "mt-3 w-full",
                        onclick: move |_| {
                            let p = tr!(l, "hello", "hallo", "hola");
                            befehle(&[&format!("cargo new {p}"), &format!("cd {p}")]);
                            oeffnen(format!("{HOME}/{p}/src/main.rs"));
                        },
                        Sparkles {}
                        {tr!(l, "Sample project", "Beispielprojekt", "Proyecto de ejemplo")}
                    }
                }
            }
            div { class: "flex min-w-0 flex-col gap-4",
                div { class: "overflow-hidden rounded-xl border",
                    div { class: "flex items-center justify-between gap-2 border-b bg-muted/60 px-3 py-1.5",
                        span { class: "flex items-center gap-1.5 truncate font-mono text-xs text-muted-foreground",
                            Code { class: "size-3.5" }
                            {datei().map_or_else(|| tr!(l, "No file open", "Keine Datei geöffnet", "Ningún archivo abierto").to_string(), |d| d.replace(HOME, "~"))}
                        }
                        if datei().is_some_and(|d| d.ends_with(".rs")) {
                            Button {
                                size: ButtonSize::Sm,
                                class: "bg-rust text-background hover:bg-rust/90",
                                disabled: shell.read().zeilen.last().is_some_and(|z| z.text.ends_with("cargo run")),
                                onclick: move |_| {
                                    // Change into the project folder and run `cargo run` there.
                                    let Some(p) = datei() else { return };
                                    if let Some(w) = p.find("/src/").map(|i| p[..i].to_string()) {
                                        shell.write().cwd = w;
                                        extern_befehl.set(Some("cargo run".into()));
                                    } else {
                                        let name = p.rsplit('/').next().unwrap_or_default().to_string();
                                        shell.write().cwd = p[..p.len() - name.len() - 1].to_string();
                                        extern_befehl.set(Some(format!("rustc {name}")));
                                    }
                                },
                                Play {}
                                "cargo run"
                            }
                        }
                    }
                    if datei().is_some() {
                        CodeEditor { code }
                    } else {
                        div { class: "flex h-40 flex-col items-center justify-center gap-2 bg-code-bg p-6 text-center text-sm text-muted-foreground",
                            p { {tr!(l, "Create a project: ", "Lege ein Projekt an: ", "Crea un proyecto: ")} code { class: "rounded bg-muted px-1.5 font-mono", "cargo new hello" } }
                            p { {tr!(l, "Then open a file with ", "Dann öffne eine Datei mit ", "Luego abre un archivo con ")} code { class: "rounded bg-muted px-1.5 font-mono", "code hello/src/main.rs" } {tr!(l, " or click it on the left.", " oder klicke sie links an.", " o haz clic en él a la izquierda.")} }
                        }
                    }
                }
                TerminalView {
                    shell,
                    hoehe: "h-80",
                    on_bearbeiten: move |p: String| oeffnen(p),
                    extern_befehl,
                }
            }
        }
    }
}
