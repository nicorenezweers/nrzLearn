use dioxus::prelude::*;

use crate::i18n::use_sprache;
use crate::playground;
use crate::shell::{Aktion, Art, Shell};

/// Interactive simulated terminal.
/// `on_befehl` is called after every command (for terminal exercises),
/// `on_bearbeiten` when a file is opened with `code <file>`.
#[component]
pub fn TerminalView(
    shell: Signal<Shell>,
    #[props(optional)] on_befehl: Option<EventHandler<String>>,
    #[props(optional)] on_bearbeiten: Option<EventHandler<String>>,
    #[props(default = "h-80".to_string())] hoehe: String,
    /// Command set from outside that runs immediately.
    #[props(optional)] extern_befehl: Option<Signal<Option<String>>>,
) -> Element {
    let l = use_sprache();
    let mut eingabe = use_signal(String::new);
    let mut verlauf_pos = use_signal(|| None::<usize>);
    let mut beschaeftigt = use_signal(|| false);
    let id = use_hook(|| format!("term-{}", rand_id()));

    // Scroll to the end after every new line.
    let scroll_id = id.clone();
    use_effect(move || {
        let _ = shell.read().zeilen.len();
        document::eval(&format!(
            "requestAnimationFrame(()=>{{const e=document.getElementById('{scroll_id}');if(e)e.scrollTop=e.scrollHeight;}})"
        ));
    });

    let mut ausfuehren = move |befehl: String| {
        let aktion = shell.write().befehl(&befehl);
        if let Some(h) = &on_befehl {
            h.call(befehl.trim().to_string());
        }
        match aktion {
            Aktion::Keine => {}
            Aktion::Bearbeiten { pfad } => {
                if let Some(h) = &on_bearbeiten {
                    h.call(pfad);
                } else {
                    shell.write().info(tr!(l, "To edit files, open the terminal page (top menu).", "Zum Bearbeiten von Dateien öffne die Terminal-Seite (oben im Menü).", "Para editar archivos, abre la página del terminal (menú superior)."));
                }
            }
            Aktion::Kompilieren { code, name, pfad, modus } => {
                beschaeftigt.set(true);
                spawn(async move {
                    let tests = modus == crate::shell::Modus::Test;
                    let ergebnis = playground::ausfuehren(&code, tests).await;
                    shell.write().ergebnis(&name, &pfad, modus, ergebnis);
                    beschaeftigt.set(false);
                });
            }
        }
    };

    let mut absenden = move || {
        let befehl = eingabe();
        eingabe.set(String::new());
        verlauf_pos.set(None);
        ausfuehren(befehl);
    };

    use_effect(move || {
        if let Some(mut sig) = extern_befehl {
            let befehl = sig.read().clone();
            if let Some(b) = befehl {
                sig.set(None);
                ausfuehren(b);
            }
        }
    });

    let input_id = format!("{id}-in");
    let fokus_id = input_id.clone();

    rsx! {
        div {
            class: "overflow-hidden rounded-xl border border-black/20 bg-term-bg font-mono text-[0.8rem] leading-5 text-term-fg shadow-lg",
            onclick: move |_| {
                document::eval(&format!("document.getElementById('{fokus_id}')?.focus({{preventScroll:true}})"));
            },
            div { class: "flex items-center gap-1.5 border-b border-white/10 bg-white/5 px-3 py-2",
                span { class: "size-3 rounded-full bg-white/30" }
                span { class: "size-3 rounded-full bg-white/20" }
                span { class: "size-3 rounded-full bg-white/10" }
                span { class: "ml-2 text-xs text-term-fg/50", "Terminal – bash" }
            }
            div { id: "{id}", class: "{hoehe} overflow-y-auto px-4 py-3",
                for (i, z) in shell.read().zeilen.iter().enumerate() {
                    div {
                        key: "{i}",
                        class: match z.art {
                            Art::Prompt => "whitespace-pre-wrap break-all text-term-fg",
                            Art::Ausgabe => "whitespace-pre-wrap break-words text-term-fg/90",
                            Art::Fehler => "whitespace-pre-wrap break-words text-term-fg underline decoration-dotted underline-offset-4",
                            Art::Gedimmt => "whitespace-pre-wrap break-words text-term-fg/45",
                            Art::Erfolg => "whitespace-pre-wrap break-words text-term-fg font-semibold",
                            Art::Info => "whitespace-pre-wrap break-words text-term-fg/75 italic",
                        },
                        if z.art == Art::Prompt {
                            {prompt_zeile(&z.text)}
                        } else {
                            "{z.text}"
                        }
                    }
                }
                div { class: "flex items-center gap-2",
                    span { class: "shrink-0 text-term-fg font-semibold", "{shell.read().prompt()}" }
                    if beschaeftigt() {
                        span { class: "text-term-fg/50", {tr!(l, "running… ", "läuft… ", "ejecutando… ")} span { class: "anim-blink", "▋" } }
                    } else {
                        input {
                            id: "{input_id}",
                            class: "min-w-0 flex-1 bg-transparent text-term-fg caret-white outline-none",
                            spellcheck: "false",
                            autocomplete: "off",
                            autocapitalize: "off",
                            "aria-label": tr!(l, "Terminal input", "Terminal-Eingabe", "Entrada del terminal"),
                            value: "{eingabe}",
                            oninput: move |e| eingabe.set(e.value()),
                            onkeydown: move |e| {
                                match e.key() {
                                    Key::Enter => absenden(),
                                    Key::ArrowUp => {
                                        e.prevent_default();
                                        let v = shell.read().verlauf.clone();
                                        if v.is_empty() { return; }
                                        let pos = verlauf_pos().map_or(v.len() - 1, |p: usize| p.saturating_sub(1));
                                        verlauf_pos.set(Some(pos));
                                        eingabe.set(v[pos].clone());
                                    }
                                    Key::ArrowDown => {
                                        e.prevent_default();
                                        let v = shell.read().verlauf.clone();
                                        match verlauf_pos() {
                                            Some(p) if p + 1 < v.len() => {
                                                verlauf_pos.set(Some(p + 1));
                                                eingabe.set(v[p + 1].clone());
                                            }
                                            _ => {
                                                verlauf_pos.set(None);
                                                eingabe.set(String::new());
                                            }
                                        }
                                    }
                                    Key::Character(c) if c == "l" && e.modifiers().ctrl() => {
                                        e.prevent_default();
                                        shell.write().zeilen.clear();
                                    }
                                    _ => {}
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}

fn prompt_zeile(text: &str) -> Element {
    let (prompt, rest) = text.split_once("$ ").map_or((text, ""), |(p, r)| (p, r));
    rsx! {
        span { class: "text-term-fg font-semibold", "{prompt}$ " }
        span { "{rest}" }
    }
}

fn rand_id() -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    static ZAEHLER: AtomicU32 = AtomicU32::new(0);
    ZAEHLER.fetch_add(1, Ordering::Relaxed)
}
