//! Views for all exercise types. Every view writes the learner's choice into
//! the `antwort` signal; checking happens in the lesson player.

use dioxus::prelude::*;
use icons::{Check, Eye, Lightbulb, Play, RotateCcw};

use crate::components::code::{Ausgabe, CodeBlock, CodeEditor, Highlighted, Lauf};
use crate::components::terminal::TerminalView;
use crate::components::text::RichText;
use crate::components::ui::{Button, ButtonSize, ButtonVariant};
use crate::i18n::use_sprache;
use crate::playground;
use crate::rng::Rng;
use crate::shell::Shell;
use crate::uebung::{Antwort, Uebung};

const CHIP: &str = "rounded-xl border-2 border-b-4 bg-card px-3 py-2 font-mono text-sm transition active:translate-y-0.5 active:border-b-2 hover:bg-accent disabled:opacity-40";

#[component]
fn Frage(text: String) -> Element {
    rsx! { RichText { text, class: "text-lg font-semibold leading-8 sm:text-xl" } }
}

/// Picks the matching view for the exercise.
#[component]
pub fn UebungView(
    uebung: Uebung,
    antwort: Signal<Antwort>,
    gemischt: Vec<String>,
    gesperrt: bool,
    on_enter: EventHandler<()>,
    hilfe: Signal<bool>,
) -> Element {
    let l = use_sprache();
    match uebung {
        Uebung::Auswahl { frage, code, optionen, .. } => rsx! {
            Frage { text: frage }
            if let Some(c) = code { CodeBlock { code: c } }
            Optionen { optionen, antwort, gesperrt }
        },
        Uebung::WahrFalsch { frage, code, .. } => rsx! {
            div { class: "text-xs font-semibold uppercase tracking-widest text-muted-foreground", {tr!(l, "Is this right?", "Stimmt das?", "¿Es correcto?")} }
            Frage { text: frage }
            if let Some(c) = code { CodeBlock { code: c } }
            Optionen { optionen: vec![tr!(l, "True", "Wahr", "Verdadero").to_string(), tr!(l, "False", "Falsch", "Falso").to_string()], antwort, gesperrt, gross: true }
        },
        Uebung::Eingabe { frage, code, .. } => rsx! {
            Frage { text: frage }
            if let Some(c) = code { CodeBlock { code: c } }
            EingabeFeld { antwort, gesperrt, on_enter }
        },
        Uebung::Luecke { frage, code, optionen, .. } => rsx! {
            Frage { text: frage }
            LueckeView { code, optionen, antwort, gesperrt }
        },
        Uebung::Bausteine { frage, .. } => rsx! {
            Frage { text: frage }
            BausteinView { teile: gemischt, antwort, gesperrt, zeilen: false }
        },
        Uebung::Reihenfolge { frage, .. } => rsx! {
            Frage { text: frage }
            BausteinView { teile: gemischt, antwort, gesperrt, zeilen: true }
        },
        Uebung::Paare { frage, paare } => rsx! {
            Frage { text: frage }
            PaareView { paare, antwort }
        },
        Uebung::FehlerZeile { frage, zeilen, .. } => rsx! {
            Frage { text: frage }
            FehlerView { zeilen, antwort, gesperrt }
        },
        Uebung::Code { aufgabe, vorlage, erwartet, hinweis, loesung } => rsx! {
            Frage { text: aufgabe }
            CodeView { vorlage, erwartet, hinweis, loesung, antwort, hilfe }
        },
        Uebung::Terminal { aufgabe, ziele, hinweis } => rsx! {
            Frage { text: aufgabe }
            TerminalUebung { ziele, hinweis, antwort }
        },
    }
}

#[component]
fn Optionen(optionen: Vec<String>, antwort: Signal<Antwort>, gesperrt: bool, #[props(default)] gross: bool) -> Element {
    let raster = if gross || optionen.len() <= 2 { "grid grid-cols-2 gap-3" } else { "grid gap-3 sm:grid-cols-2" };
    rsx! {
        div { class: "{raster}",
            for (i, o) in optionen.into_iter().enumerate() {
                button {
                    key: "{i}",
                    disabled: gesperrt,
                    "data-taste": "{i + 1}",
                    class: if antwort() == Antwort::Index(i) {
                        "flex items-center gap-3 rounded-2xl border-2 border-b-4 border-foreground bg-foreground/5 px-4 py-3.5 text-left font-medium transition"
                    } else {
                        "flex items-center gap-3 rounded-2xl border-2 border-b-4 bg-card px-4 py-3.5 text-left transition hover:bg-accent active:translate-y-0.5 active:border-b-2 disabled:hover:bg-card"
                    },
                    onclick: move |_| antwort.set(Antwort::Index(i)),
                    span {
                        class: if antwort() == Antwort::Index(i) { "flex size-7 shrink-0 items-center justify-center rounded-lg bg-foreground text-xs font-bold text-background" } else { "flex size-7 shrink-0 items-center justify-center rounded-lg border-2 text-xs font-bold text-muted-foreground" },
                        "{i + 1}"
                    }
                    span { class: if gross { "w-full text-center text-lg font-semibold" } else { "" },
                        RichText { text: o, class: "leading-6" }
                    }
                }
            }
        }
    }
}

#[component]
fn EingabeFeld(antwort: Signal<Antwort>, gesperrt: bool, on_enter: EventHandler<()>) -> Element {
    let l = use_sprache();
    let wert = match antwort() {
        Antwort::Text(t) => t,
        _ => String::new(),
    };
    // Set the focus even if the previous button keeps it while re-rendering.
    use_hook(|| {
        document::eval("setTimeout(function(){var e=document.querySelector('[data-autofokus]');if(e)e.focus();},80)");
    });
    rsx! {
        input {
            "data-autofokus": "true",
            class: "h-14 w-full rounded-2xl border-2 bg-card px-4 font-mono text-lg outline-none transition focus:border-foreground disabled:opacity-70",
            placeholder: tr!(l, "Your answer …", "Deine Antwort …", "Tu respuesta…"),
            autofocus: true,
            autocomplete: "off",
            autocapitalize: "off",
            spellcheck: "false",
            disabled: gesperrt,
            value: "{wert}",
            onmounted: move |e| async move {
                let _ = e.set_focus(true).await;
            },
            oninput: move |e| antwort.set(Antwort::Text(e.value())),
            onkeydown: move |e| {
                if e.key() == Key::Enter {
                    e.stop_propagation();
                    on_enter.call(());
                }
            },
        }
    }
}

#[component]
fn LueckeView(code: String, optionen: Vec<String>, antwort: Signal<Antwort>, gesperrt: bool) -> Element {
    let (vor, nach) = code.split_once("___").map_or((code.clone(), String::new()), |(a, b)| (a.to_string(), b.to_string()));
    let gewaehlt = match antwort() {
        Antwort::Index(i) => optionen.get(i).cloned(),
        _ => None,
    };
    rsx! {
        div { class: "overflow-x-auto rounded-xl border bg-code-bg p-4 font-mono text-[0.85rem] leading-7",
            pre { class: "inline whitespace-pre",
                Highlighted { code: vor }
                if let Some(g) = gewaehlt {
                    span { class: "anim-pop mx-0.5 rounded-md border-2 border-foreground bg-foreground px-1.5 py-0.5 font-bold text-background", "{g.trim()}" }
                } else {
                    span { class: "mx-0.5 inline-block w-16 rounded-md border-2 border-dashed border-foreground/50 align-middle", "\u{00a0}" }
                }
                Highlighted { code: nach }
            }
        }
        div { class: "flex flex-wrap justify-center gap-2",
            for (i, o) in optionen.into_iter().enumerate() {
                button {
                    key: "{i}",
                    disabled: gesperrt,
                    "data-taste": "{i + 1}",
                    class: if antwort() == Antwort::Index(i) { "{CHIP} border-foreground bg-foreground text-background hover:bg-foreground" } else { "{CHIP}" },
                    onclick: move |_| antwort.set(Antwort::Index(i)),
                    "{o.trim()}"
                }
            }
        }
    }
}

#[component]
fn BausteinView(teile: Vec<String>, antwort: Signal<Antwort>, gesperrt: bool, zeilen: bool) -> Element {
    let l = use_sprache();
    let gewaehlt: Vec<usize> = match antwort() {
        Antwort::Liste(l) => l,
        _ => Vec::new(),
    };
    let ziel_class = if zeilen {
        "flex min-h-32 flex-col gap-1.5 rounded-2xl border-2 border-dashed p-3"
    } else {
        "flex min-h-16 flex-wrap items-center gap-2 rounded-2xl border-2 border-dashed p-3"
    };
    let bank_class = if zeilen { "flex flex-col gap-1.5" } else { "flex flex-wrap justify-center gap-2" };
    let chip = if zeilen { "w-full whitespace-pre text-left" } else { "" };
    // Fixed height for lines so the choices below don't jump while tapping.
    let ziel_style = if zeilen { format!("min-height: {}px", teile.len() * 46 + 28) } else { String::new() };
    rsx! {
        div { class: "{ziel_class}", style: "{ziel_style}",
            if gewaehlt.is_empty() {
                span { class: "m-auto text-sm text-muted-foreground",
                    if zeilen { {tr!(l, "Tap the lines below in the right order.", "Tippe die Zeilen unten in der richtigen Reihenfolge an.", "Toca las líneas de abajo en el orden correcto.")} } else { {tr!(l, "Tap the blocks below.", "Tippe die Bausteine unten an.", "Toca los bloques de abajo.")} }
                }
            }
            for (pos, &i) in gewaehlt.iter().enumerate() {
                button {
                    key: "g{pos}-{i}",
                    disabled: gesperrt,
                    class: "anim-pop {CHIP} {chip} border-foreground",
                    onclick: move |_| {
                        let mut l = match antwort() { Antwort::Liste(l) => l, _ => Vec::new() };
                        l.remove(pos);
                        antwort.set(if l.is_empty() { Antwort::Keine } else { Antwort::Liste(l) });
                    },
                    Highlighted { code: teile[i].clone() }
                }
            }
        }
        div { class: "{bank_class}",
            for (i, t) in teile.iter().enumerate() {
                if gewaehlt.contains(&i) {
                    span { key: "b{i}", class: "{CHIP} {chip} invisible", "{t}" }
                } else {
                    button {
                        key: "b{i}",
                        disabled: gesperrt,
                        class: "{CHIP} {chip}",
                        onclick: move |_| {
                            let mut l = match antwort() { Antwort::Liste(l) => l, _ => Vec::new() };
                            l.push(i);
                            antwort.set(Antwort::Liste(l));
                        },
                        Highlighted { code: t.clone() }
                    }
                }
            }
        }
    }
}

#[component]
fn PaareView(paare: Vec<(String, String)>, antwort: Signal<Antwort>) -> Element {
    let l = use_sprache();
    let n = paare.len();
    let ordnung = use_hook(|| {
        let mut r = Rng::neu();
        let mut l: Vec<usize> = (0..n).collect();
        let mut re: Vec<usize> = (0..n).collect();
        r.mischen(&mut l);
        r.mischen(&mut re);
        (l, re)
    });
    let mut links = use_signal(|| None::<usize>);
    let mut fertig = use_signal(Vec::<usize>::new);
    let mut fehler = use_signal(|| None::<usize>);
    let mut fehlversuche = use_signal(|| 0_u32);

    let mut waehle_rechts = move |j: usize| {
        let Some(i) = links() else { return };
        if i == j {
            fertig.write().push(i);
            links.set(None);
            if fertig.read().len() == n {
                // Only error-free matching counts as correct.
                antwort.set(Antwort::Fertig(fehlversuche() == 0));
            }
        } else {
            fehler.set(Some(j));
            fehlversuche += 1;
            links.set(None);
        }
    };

    let knopf = |aktiv: bool, erledigt: bool, falsch: bool| {
        if erledigt {
            "rounded-xl border-2 border-dashed px-3 py-3 text-sm text-muted-foreground line-through opacity-50"
        } else if falsch {
            "anim-shake rounded-xl border-2 border-b-4 border-foreground/40 bg-card px-3 py-3 text-sm"
        } else if aktiv {
            "rounded-xl border-2 border-b-4 border-foreground bg-foreground px-3 py-3 text-sm font-semibold text-background"
        } else {
            "rounded-xl border-2 border-b-4 bg-card px-3 py-3 text-sm transition hover:bg-accent active:translate-y-0.5"
        }
    };

    rsx! {
        div { class: "grid grid-cols-2 gap-3",
            div { class: "flex flex-col gap-2",
                for &i in ordnung.0.iter() {
                    button {
                        key: "l{i}",
                        disabled: fertig().contains(&i),
                        class: knopf(links() == Some(i), fertig().contains(&i), false),
                        onclick: move |_| {
                            links.set(Some(i));
                            fehler.set(None);
                        },
                        span { class: "font-mono", "{paare[i].0}" }
                    }
                }
            }
            div { class: "flex flex-col gap-2",
                for &j in ordnung.1.iter() {
                    button {
                        key: "r{j}",
                        disabled: fertig().contains(&j),
                        class: knopf(false, fertig().contains(&j), fehler() == Some(j)),
                        onclick: move |_| waehle_rechts(j),
                        "{paare[j].1}"
                    }
                }
            }
        }
        p { class: "text-center text-xs text-muted-foreground",
            if links().is_some() { {tr!(l, "Now tap the matching meaning on the right.", "Jetzt die passende Bedeutung rechts antippen.", "Ahora toca el significado correcto a la derecha.")} } else { {tr!(l, "Tap a term on the left.", "Tippe links einen Begriff an.", "Toca un término a la izquierda.")} }
            if fehlversuche() > 0 { {tr!(l, format!(" · {} wrong tr(ies)", fehlversuche()), format!(" · {} Fehlversuch(e)", fehlversuche()), format!(" · {} fallo(s)", fehlversuche()))} }
        }
    }
}

#[component]
fn FehlerView(zeilen: Vec<String>, antwort: Signal<Antwort>, gesperrt: bool) -> Element {
    rsx! {
        div { class: "overflow-hidden rounded-xl border bg-code-bg py-2 font-mono text-[0.85rem] leading-7",
            for (i, z) in zeilen.into_iter().enumerate() {
                button {
                    key: "{i}",
                    disabled: gesperrt,
                    "data-taste": "{i + 1}",
                    class: if antwort() == Antwort::Index(i) { "flex w-full border-l-4 border-foreground bg-foreground/15 pr-3 text-left" } else { "flex w-full border-l-4 border-transparent pr-3 text-left transition hover:bg-accent" },
                    onclick: move |_| antwort.set(Antwort::Index(i)),
                    span { class: "w-9 shrink-0 select-none pr-2 text-right text-muted-foreground/60", "{i + 1}" }
                    span { class: "whitespace-pre", Highlighted { code: z } }
                }
            }
        }
    }
}

fn ausgabe_passt(ist: &str, soll: &str) -> bool {
    let norm = |s: &str| s.trim().lines().map(str::trim_end).collect::<Vec<_>>().join("\n");
    norm(ist) == norm(soll)
}

#[component]
fn CodeView(vorlage: String, erwartet: String, hinweis: String, loesung: String, antwort: Signal<Antwort>, hilfe: Signal<bool>) -> Element {
    let l = use_sprache();
    let start = vorlage.trim_end().to_string() + "\n";
    let mut code = use_signal(|| start.clone());
    let mut lauf = use_signal(|| Lauf::Leer);
    let mut versuche = use_signal(|| 0_u32);
    let mut hinweis_offen = use_signal(|| false);
    let mut loesung_offen = use_signal(|| false);
    let geloest = antwort() == Antwort::Fertig(true);
    let erwartet_vergleich = erwartet.clone();
    let loesung_code = loesung.clone();

    let pruefen = move |_| {
        lauf.set(Lauf::Laeuft);
        let soll = erwartet_vergleich.clone();
        spawn(async move {
            match playground::ausfuehren(&code(), false).await {
                Ok(e) => {
                    if e.success && ausgabe_passt(&e.stdout, &soll) {
                        antwort.set(Antwort::Fertig(true));
                    } else {
                        versuche += 1;
                    }
                    lauf.set(Lauf::Fertig(e));
                }
                Err(e) => lauf.set(Lauf::Fehler(e)),
            }
        });
    };

    rsx! {
        div { class: "rounded-xl bg-muted px-3 py-2 text-xs",
            span { class: "font-semibold text-muted-foreground", {tr!(l, "Expected output", "Erwartete Ausgabe", "Salida esperada")} }
            pre { class: "mt-1 font-mono text-[0.8rem]", "{erwartet}" }
        }
        div { class: "overflow-hidden rounded-xl border",
            div { class: "flex items-center justify-between border-b bg-muted/60 px-3 py-1.5",
                span { class: "font-mono text-xs text-muted-foreground", "src/main.rs" }
                div { class: "flex gap-1",
                    Button {
                        variant: ButtonVariant::Ghost,
                        size: ButtonSize::Sm,
                        title: tr!(l, "Reset", "Zurücksetzen", "Restablecer"),
                        onclick: move |_| { code.set(start.clone()); lauf.set(Lauf::Leer); },
                        RotateCcw {}
                    }
                    Button {
                        size: ButtonSize::Sm,
                        disabled: lauf() == Lauf::Laeuft || geloest,
                        onclick: pruefen,
                        if geloest { Check {} {tr!(l, "Correct!", "Richtig!", "¡Correcto!")} } else { Play {} {tr!(l, "Run & check", "Ausführen & prüfen", "Ejecutar y comprobar")} }
                    }
                }
            }
            CodeEditor { code, klein: true }
            Ausgabe { lauf: lauf() }
        }
        if versuche() > 0 && !geloest {
            p { class: "anim-pop text-sm font-medium", {tr!(l, "The output doesn't match yet. Compare carefully – spaces count too.", "Die Ausgabe stimmt noch nicht. Vergleiche genau – auch Leerzeichen zählen.", "La salida todavía no coincide. Compara con cuidado: los espacios también cuentan.")} }
        }
        div { class: "flex flex-wrap gap-2",
            if hinweis_offen() {
                div { class: "anim-pop flex w-full gap-2 rounded-xl border bg-muted/50 p-3 text-sm",
                    Lightbulb { class: "mt-0.5 size-4 shrink-0" }
                    RichText { text: hinweis.clone(), class: "leading-6" }
                }
            } else {
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, onclick: move |_| hinweis_offen.set(true), Lightbulb {} {tr!(l, "Hint", "Hinweis", "Pista")} }
            }
            if versuche() >= 1 && !geloest {
                if loesung_offen() {
                    div { class: "anim-pop w-full space-y-2",
                        CodeBlock { code: loesung.clone(), titel: tr!(l, "Sample solution", "Musterlösung", "Solución de ejemplo") }
                        Button {
                            size: ButtonSize::Sm,
                            onclick: move |_| {
                                code.set(loesung_code.trim_end().to_string() + "\n");
                                hilfe.set(true);
                                lauf.set(Lauf::Leer);
                            },
                            {tr!(l, "Use solution", "Lösung übernehmen", "Usar la solución")}
                        }
                        p { class: "text-xs text-muted-foreground", {tr!(l, "Take over the solution, read it line by line and run it – then the exercise counts as solved (with half XP).", "Übernimm die Lösung, lies sie Zeile für Zeile und führe sie aus – dann zählt die Aufgabe als gelöst (mit halben XP).", "Usa la solución, léela línea a línea y ejecútala: así el ejercicio cuenta como resuelto (con la mitad de XP).")} }
                    }
                } else {
                    Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, onclick: move |_| loesung_offen.set(true), Eye {} {tr!(l, "Show solution", "Lösung ansehen", "Ver solución")} }
                }
            }
        }
    }
}

fn befehl_passt(befehl: &str, ziel: &str) -> bool {
    let norm = |s: &str| s.split_whitespace().collect::<Vec<_>>().join(" ");
    norm(befehl) == norm(ziel)
}

#[component]
fn TerminalUebung(ziele: Vec<String>, hinweis: String, antwort: Signal<Antwort>) -> Element {
    let l = crate::i18n::use_sprache();
    let shell = use_signal(|| Shell::neu(l));
    let mut erreicht = use_signal(|| 0_usize);
    let anzahl = ziele.len();
    let ziele_check = ziele.clone();
    rsx! {
        ol { class: "flex flex-wrap gap-2 text-sm",
            for (i, z) in ziele.iter().enumerate() {
                li {
                    key: "{i}",
                    class: if i < erreicht() { "flex items-center gap-1.5 rounded-full bg-foreground px-3 py-1 font-mono text-xs text-background" } else { "flex items-center gap-1.5 rounded-full border-2 px-3 py-1 font-mono text-xs" },
                    if i < erreicht() { Check { class: "size-3" } } else { span { class: "text-muted-foreground", "{i + 1}." } }
                    "{z}"
                }
            }
        }
        TerminalView {
            shell,
            hoehe: "h-56",
            on_befehl: move |b: String| {
                let n = erreicht();
                if n < anzahl && befehl_passt(&b, &ziele_check[n]) {
                    erreicht.set(n + 1);
                    if n + 1 == anzahl {
                        antwort.set(Antwort::Fertig(true));
                    }
                }
            },
        }
        p { class: "text-xs text-muted-foreground", "{hinweis}" }
    }
}
