//! The lesson player: one exercise after another, with feedback, combo
//! streaks, lives in exams and a reward at the end.

use dioxus::prelude::*;
use icons::{
    ArrowRight, BookOpen, Check, Clock, Crown, Flame, Gauge, Heart, HeartCrack, Lock, RotateCcw, Sparkles, StickyNote,
    Target, Trophy, X, Zap,
};

use crate::Route;
use crate::components::effekte::{Konfetti, Ton, ton};
use crate::components::spick::TaschenSpick;
use crate::components::text::RichText;
use crate::components::uebungen::UebungView;
use crate::content::lektionen;
use crate::i18n::{Sprache, use_sprache};
use crate::progress::{Belohnung, abschliessen, use_fortschritt};
use crate::rng::Rng;
use crate::uebung::{Antwort, Uebung};
use crate::welt::{self, Art};

fn lob(l: Sprache) -> &'static [&'static str] {
    tr!(
        l,
        &["Great!", "Exactly!", "Nice!", "Correct!", "You're on fire!", "Excellent!", "Like a true Rustacean!", "The compiler would be proud!", "Awesome!", "Compiled without errors!", "Spot on!"],
        &["Stark!", "Genau!", "Sauber!", "Richtig!", "Läuft bei dir!", "Ausgezeichnet!", "Wie ein echter Rustacean!", "Der Compiler wäre stolz!", "Super!", "Fehlerfrei kompiliert!", "Volltreffer!"],
        &["¡Genial!", "¡Exacto!", "¡Muy bien!", "¡Correcto!", "¡Estás en racha!", "¡Excelente!", "¡Como un auténtico rustáceo!", "¡El compilador estaría orgulloso!", "¡Increíble!", "¡Compilado sin errores!", "¡En el blanco!"]
    )
}

fn trost(l: Sprache) -> &'static [&'static str] {
    tr!(l, &["Not quite.", "Close.", "Almost!", "That wasn't it."], &["Nicht ganz.", "Knapp daneben.", "Fast!", "Das war's leider nicht."], &["Casi.", "Por poco.", "¡Casi!", "No era eso."])
}

#[derive(Clone, PartialEq)]
enum Phase {
    Antworten,
    Feedback { richtig: bool, text: &'static str },
    Fertig(Ergebnis),
    Verloren,
}

#[derive(Clone, PartialEq)]
struct Ergebnis {
    belohnung: Belohnung,
    sterne: u8,
    genauigkeit: u32,
    sekunden: u32,
    max_combo: u32,
    perfekt: bool,
}

#[component]
pub fn Spieler(id: String) -> Element {
    let fortschritt = use_fortschritt();
    let l = use_sprache();
    let mut runde = use_signal(|| 0_u32);
    let Some((index, k)) = welt::finden(&id) else {
        return rsx! {
            div { class: "flex min-h-screen flex-col items-center justify-center gap-3",
                p { {tr!(l, "This level doesn't exist.", "Dieses Level gibt es nicht.", "Este nivel no existe.")} }
                Link { to: Route::Home {}, class: "underline", {tr!(l, "To the learning path", "Zum Lernpfad", "Al camino de aprendizaje")} }
            }
        };
    };
    if k.art == Art::Theorie {
        return rsx! { crate::pages::lesson::LektionSeite { id: k.welt + 1 } };
    }
    if !fortschritt.read().offen(index) {
        return rsx! {
            div { class: "flex min-h-screen flex-col items-center justify-center gap-4 p-6 text-center",
                Lock { class: "size-12 text-muted-foreground" }
                h1 { class: "text-2xl font-bold", {tr!(l, format!("Level {} is still locked", k.nr), format!("Level {} ist noch gesperrt", k.nr), format!("El nivel {} sigue bloqueado", k.nr))} }
                p { class: "text-muted-foreground", {tr!(l, format!("Finish level {} first.", k.nr - 1), format!("Schließe zuerst Level {} ab.", k.nr - 1), format!("Termina primero el nivel {}.", k.nr - 1))} }
                Link { to: Route::Home {}, class: "rounded-xl border-2 border-b-4 px-5 py-2.5 font-semibold", {tr!(l, "To the learning path", "Zum Lernpfad", "Al camino de aprendizaje")} }
            }
        };
    }
    // One-entry list so that "Again" really mounts a new round.
    let schluessel = format!("{id}-{}", runde());
    rsx! {
        for k in std::iter::once(schluessel) {
            Sitzung { key: "{k}", index, on_neu: move |_| runde += 1 }
        }
    }
}

#[component]
fn Sitzung(index: usize, on_neu: EventHandler<()>) -> Element {
    let k = &welt::alle()[index];
    let fortschritt = use_fortschritt();
    let ton_an = fortschritt.read().ton;
    let navigator = use_navigator();
    let l = use_sprache();
    // The round keeps the language it started with.
    let sl = use_hook(|| l);

    let mut rng = use_signal(Rng::neu);
    let mut liste: Signal<Vec<(Uebung, bool)>> = use_signal(|| welt::sitzung(k, &mut rng.write(), sl).into_iter().map(|u| (u, false)).collect());
    let n = use_hook(|| liste.peek().len());
    let mut pos = use_signal(|| 0_usize);
    let mut antwort = use_signal(Antwort::default);
    let mut gemischt = use_signal(|| liste.peek().first().map(|(u, _)| u.gemischt(&mut rng.write())).unwrap_or_default());
    let mut phase = use_signal(|| Phase::Antworten);
    let mut richtig = use_signal(|| 0_usize);
    let mut beantwortet = use_signal(|| 0_usize);
    let mut fehler = use_signal(|| 0_u32);
    let mut leben = use_signal(|| k.leben());
    let mut combo = use_signal(|| 0_u32);
    let mut max_combo = use_signal(|| 0_u32);
    let mut xp = use_signal(|| 0_u32);
    let mut hilfe = use_signal(|| false);
    let mut toast = use_signal(|| None::<u32>);
    let mut xp_anzeige = use_signal(|| (0_u32, 0_u32));
    let start = use_hook(js_sys::Date::now);

    let aktuelle = liste.read().get(pos()).cloned();
    let pruefung = k.ist_pruefung();

    let mut pruefen = move || {
        if phase() != Phase::Antworten {
            return;
        }
        let Some((u, wiederholt)) = liste.read().get(pos()).cloned() else { return };
        if !u.bereit(&antwort()) {
            return;
        }
        let ok = u.pruefen(&antwort(), &gemischt());
        beantwortet += 1;
        let text = if ok { *rng.write().wahl(lob(sl)) } else { *rng.write().wahl(trost(sl)) };
        if ok {
            richtig += 1;
            combo += 1;
            if combo() > max_combo() {
                max_combo.set(combo());
            }
            let mut gewinn = u.xp();
            if wiederholt || hilfe() {
                gewinn /= 2;
            }
            xp += gewinn;
            let (zaehler, _) = xp_anzeige();
            xp_anzeige.set((zaehler + 1, gewinn));
            if [3, 5, 10, 15, 20, 30].contains(&combo()) {
                toast.set(Some(combo()));
                ton(Ton::Serie, ton_an);
                spawn(async move {
                    gloo_timers::future::TimeoutFuture::new(1800).await;
                    toast.set(None);
                });
            } else {
                ton(Ton::Richtig, ton_an);
            }
        } else {
            fehler += 1;
            combo.set(0);
            ton(Ton::Falsch, ton_an);
            if let Some(l) = leben() {
                leben.set(Some(l.saturating_sub(1)));
            } else {
                // Mistakes come back at the end.
                liste.write().push((u, true));
            }
        }
        phase.set(Phase::Feedback { richtig: ok, text });
    };

    let mut weiter = move || {
        if !matches!(phase(), Phase::Feedback { .. }) {
            return;
        }
        if leben() == Some(0) {
            phase.set(Phase::Verloren);
            return;
        }
        let naechste = pos() + 1;
        if naechste >= liste.read().len() {
            let k = &welt::alle()[index];
            let versuche = beantwortet().max(1) as f64;
            let genau = if k.ist_pruefung() { richtig() as f64 / n.max(1) as f64 } else { n as f64 / versuche };
            let sterne = if genau >= 0.95 { 3 } else if genau >= 0.75 { 2 } else { 1 };
            let perfekt = fehler() == 0;
            let bonus = 10 + if perfekt { 10 } else { 0 } + if k.ist_pruefung() { 20 } else { 0 };
            let belohnung = abschliessen(fortschritt, &k.id, sterne, xp() + bonus, richtig() as u32, perfekt);
            ton(Ton::Geschafft, ton_an);
            phase.set(Phase::Fertig(Ergebnis {
                belohnung,
                sterne,
                genauigkeit: (genau * 100.0).round() as u32,
                sekunden: ((js_sys::Date::now() - start) / 1000.0) as u32,
                max_combo: max_combo(),
                perfekt,
            }));
            return;
        }
        pos.set(naechste);
        antwort.set(Antwort::Keine);
        hilfe.set(false);
        let neu = liste.read()[naechste].0.gemischt(&mut rng.write());
        gemischt.set(neu);
        phase.set(Phase::Antworten);
    };

    match phase() {
        Phase::Fertig(e) => return rsx! { Abschluss { index, ergebnis: e, on_neu } },
        Phase::Verloren => return rsx! { Verloren { on_neu } },
        _ => {}
    }

    let Some((uebung, wiederholt)) = aktuelle else {
        return rsx! {};
    };
    let fortschritt_wert = if pruefung { beantwortet() } else { richtig() };
    let prozent = (fortschritt_wert as f64 / n.max(1) as f64 * 100.0).min(100.0);
    let bereit = uebung.bereit(&antwort());
    let feedback = phase();

    rsx! {
        div {
            class: "flex min-h-screen flex-col bg-background",

            // Kopfzeile
            header { class: "sticky top-0 z-30 bg-background/90 backdrop-blur",
                div { class: "mx-auto flex max-w-3xl items-center gap-3 px-4 py-4",
                    button {
                        class: "rounded-lg p-1.5 text-muted-foreground transition hover:bg-accent hover:text-foreground",
                        title: tr!(l, "Leave lesson", "Lektion verlassen", "Salir de la lección"),
                        onclick: move |_| { navigator.push(Route::Home {}); },
                        X { class: "size-6" }
                    }
                    div { class: "relative h-4 flex-1 overflow-hidden rounded-full bg-muted",
                        div {
                            class: "h-full rounded-full bg-foreground transition-all duration-500 ease-out",
                            style: "width: {prozent}%",
                            div { class: "mx-2 mt-1 h-1 rounded-full bg-background/30" }
                        }
                    }
                    if combo() >= 2 {
                        div { class: "anim-pop flex items-center gap-1 font-bold", title: tr!(l, "Correct in a row", "Richtig in Folge", "Aciertos seguidos"),
                            Flame { class: "size-5 fill-foreground" }
                            "{combo}"
                        }
                    }
                    if let Some(lv) = leben() {
                        div { class: "flex items-center gap-0.5", title: tr!(l, "Lives", "Leben", "Vidas"),
                            for i in 0..3_u8 {
                                Heart { key: "{i}-{lv}", class: if i < lv { "size-5 fill-foreground text-foreground" } else { "size-5 text-muted-foreground/40" } }
                            }
                        }
                    }
                    TaschenSpick { verboten: !k.spickzettel_erlaubt(), kompakt: true }
                }
                if xp_anzeige().0 > 0 {
                    div { key: "{xp_anzeige().0}", class: "anim-float-up pointer-events-none absolute right-24 top-10 text-sm font-bold", "+{xp_anzeige().1} XP" }
                }
            }

            if let Some(c) = toast() {
                div { class: "pointer-events-none fixed inset-x-0 bottom-32 z-40 flex justify-center",
                    div { class: "anim-pop flex items-center gap-2 rounded-2xl bg-foreground px-5 py-3 text-lg font-bold text-background shadow-2xl",
                        Flame { class: "size-6 fill-background" }
                        {tr!(l, format!("{c} correct in a row!"), format!("{c} richtig in Folge!"), format!("¡{c} aciertos seguidos!"))}
                    }
                }
            }

            // Übung
            main { class: "mx-auto w-full max-w-2xl flex-1 px-4 pb-48 pt-2",
                div { class: "mb-4 flex items-center gap-2 text-xs font-semibold uppercase tracking-widest text-muted-foreground",
                    Zap { class: "size-3.5" }
                    "{uebung.art(l)}"
                    if wiederholt {
                        span { class: "rounded-full border px-2 py-0.5 normal-case tracking-normal", {tr!(l, "Review", "Wiederholung", "Repaso")} }
                    }
                    if pruefung {
                        span { class: "ml-auto rounded-full border px-2 py-0.5 normal-case tracking-normal",
                            if k.spickzettel_erlaubt() { {tr!(l, "Exam · cheat sheet allowed", "Prüfung · Spickzettel erlaubt", "Examen · chuleta permitida")} } else { {tr!(l, "Master exam · no cheat sheet", "Meisterprüfung · ohne Spickzettel", "Examen maestro · sin chuleta")} }
                        }
                    }
                }
                // One-entry list: the key makes every exercise mount fresh
                // (even when two of the same kind follow each other).
                for p in std::iter::once(pos()) {
                    div { key: "{p}", class: "anim-pop space-y-5",
                        UebungView {
                            uebung: uebung.clone(),
                            antwort,
                            gemischt: gemischt(),
                            gesperrt: feedback != Phase::Antworten,
                            on_enter: move |_| { if phase() == Phase::Antworten { pruefen() } else { weiter() } },
                            hilfe,
                        }
                    }
                }
            }

            // Fußleiste mit Prüfen / Feedback
            footer { class: "fixed inset-x-0 bottom-0 z-30",
                match feedback {
                    Phase::Feedback { richtig: true, text } => rsx! {
                        div { class: "anim-slide-up border-t-4 border-foreground bg-foreground text-background",
                            div { class: "mx-auto flex max-w-2xl flex-col gap-4 px-4 py-5 sm:flex-row sm:items-center",
                                div { class: "flex flex-1 gap-3",
                                    div { class: "anim-star flex size-12 shrink-0 items-center justify-center rounded-full bg-background text-foreground", Check { class: "size-7" } }
                                    div {
                                        div { class: "text-xl font-bold", "{text}" }
                                        if !uebung.erklaerung().is_empty() {
                                            RichText { text: uebung.erklaerung().to_string(), class: "text-sm leading-6 opacity-80" }
                                        }
                                    }
                                }
                                button {
                                    "data-primaer": "true",
                                    class: "h-12 rounded-2xl border-2 border-b-4 border-background bg-background px-8 font-bold uppercase tracking-wide text-foreground transition active:translate-y-0.5 active:border-b-2",
                                    onclick: move |_| weiter(),
                                    {tr!(l, "Continue", "Weiter", "Continuar")}
                                }
                            }
                        }
                    },
                    Phase::Feedback { richtig: false, text } => rsx! {
                        div { class: "anim-slide-up border-t-4 border-foreground bg-muted",
                            div { class: "mx-auto flex max-w-2xl flex-col gap-4 px-4 py-5 sm:flex-row sm:items-center",
                                div { class: "flex min-w-0 flex-1 gap-3",
                                    div { class: "anim-shake flex size-12 shrink-0 items-center justify-center rounded-full border-2 border-foreground", X { class: "size-7" } }
                                    div { class: "min-w-0",
                                        div { class: "text-xl font-bold", "{text}" }
                                        if !uebung.loesung_text(l).is_empty() {
                                            div { class: "mt-1 text-sm font-semibold", {tr!(l, "Correct answer:", "Richtige Lösung:", "Respuesta correcta:")} }
                                            if uebung.loesung_text(l).contains('\n') {
                                                pre { class: "mt-0.5 overflow-x-auto whitespace-pre-wrap font-mono text-sm", "{uebung.loesung_text(l)}" }
                                            } else {
                                                RichText { text: uebung.loesung_text(l), class: "mt-0.5 font-mono text-sm" }
                                            }
                                        }
                                        if !uebung.erklaerung().is_empty() {
                                            RichText { text: uebung.erklaerung().to_string(), class: "mt-1 text-sm leading-6 text-muted-foreground" }
                                        }
                                        if matches!(uebung, Uebung::Paare { .. }) {
                                            p { class: "mt-1 text-sm text-muted-foreground", {tr!(l, "Pairs only count if you match them without a wrong try.", "Paare zählen nur, wenn du sie ohne Fehlversuch zuordnest.", "Las parejas solo cuentan si las relacionas sin fallar.")} }
                                        }
                                        if leben().is_none() {
                                            p { class: "mt-1 text-xs text-muted-foreground", {tr!(l, "Don't worry – this exercise comes back at the end.", "Keine Sorge – diese Aufgabe kommt am Ende nochmal.", "Tranquilo: este ejercicio vuelve al final.")} }
                                        }
                                    }
                                }
                                button {
                                    "data-primaer": "true",
                                    class: "h-12 shrink-0 rounded-2xl border-2 border-b-4 border-foreground bg-foreground px-8 font-bold uppercase tracking-wide text-background transition active:translate-y-0.5 active:border-b-2",
                                    onclick: move |_| weiter(),
                                    {tr!(l, "Continue", "Weiter", "Continuar")}
                                }
                            }
                        }
                    },
                    _ => rsx! {
                        div { class: "border-t-2 bg-background",
                            div { class: "mx-auto flex max-w-2xl items-center justify-between gap-4 px-4 py-5",
                                p { class: "hidden text-xs text-muted-foreground sm:block",
                                    if matches!(uebung, Uebung::Code { .. } | Uebung::Terminal { .. } | Uebung::Paare { .. }) { {tr!(l, "Solve the exercise above, then continue.", "Löse die Aufgabe oben, dann geht's weiter.", "Resuelve el ejercicio de arriba y continúa.")} } else { {tr!(l, "Tip: Enter checks, numbers pick answers.", "Tipp: Enter prüft, Zahlen wählen Antworten.", "Consejo: Enter comprueba y los números eligen respuestas.")} }
                                }
                                button {
                                    class: if bereit { "ml-auto h-12 rounded-2xl border-2 border-b-4 border-foreground bg-foreground px-10 font-bold uppercase tracking-wide text-background transition hover:opacity-90 active:translate-y-0.5 active:border-b-2" } else { "ml-auto h-12 cursor-not-allowed rounded-2xl border-2 border-b-4 bg-muted px-10 font-bold uppercase tracking-wide text-muted-foreground" },
                                    disabled: !bereit,
                                    "data-primaer": "true",
                                    onclick: move |_| pruefen(),
                                    {tr!(l, "Check", "Prüfen", "Comprobar")}
                                }
                            }
                        }
                    },
                }
            }
        }
    }
}

#[component]
fn Abschluss(index: usize, ergebnis: Ergebnis, on_neu: EventHandler<()>) -> Element {
    let k = &welt::alle()[index];
    let navigator = use_navigator();
    let l = use_sprache();
    let b = &ergebnis.belohnung;
    let titel = if ergebnis.perfekt {
        tr!(l, "Perfect!", "Perfekt!", "¡Perfecto!")
    } else if k.art == Art::Meister {
        tr!(l, "Master exam passed!", "Meisterprüfung bestanden!", "¡Examen maestro aprobado!")
    } else if k.art == Art::Pruefung {
        tr!(l, "Exam passed!", "Prüfung bestanden!", "¡Examen aprobado!")
    } else {
        tr!(l, "Level complete!", "Level geschafft!", "¡Nivel superado!")
    };
    let min = ergebnis.sekunden / 60;
    let sek = ergebnis.sekunden % 60;

    rsx! {
        Konfetti {}
        div { class: "flex min-h-screen flex-col items-center justify-center px-4 py-10",
            div { class: "w-full max-w-md text-center",
                div { class: "anim-star mx-auto mb-4 flex size-24 items-center justify-center rounded-full bg-foreground text-background shadow-2xl",
                    if k.art == Art::Meister { Crown { class: "size-12" } } else { Trophy { class: "size-12" } }
                }
                h1 { class: "text-3xl font-extrabold tracking-tight sm:text-4xl", "{titel}" }
                p { class: "mt-1 text-muted-foreground", "Level {k.nr} · {lektionen(l)[k.welt].titel}" }
                div { class: "my-6 flex justify-center gap-3",
                    for i in 0..3_u8 {
                        div {
                            key: "{i}",
                            class: "anim-star",
                            style: "animation-delay: {0.3 + f64::from(i) * 0.25}s",
                            icons::Star { class: if i < ergebnis.sterne { "size-12 fill-foreground text-foreground" } else { "size-12 text-muted-foreground/30" } }
                        }
                    }
                }
                div { class: "grid grid-cols-2 gap-3 sm:grid-cols-4",
                    Kachel { titel: "XP", wert: format!("+{}", b.xp), Zap { class: "size-4" } }
                    Kachel { titel: tr!(l, "Accuracy", "Genauigkeit", "Precisión"), wert: format!("{}%", ergebnis.genauigkeit), Target { class: "size-4" } }
                    Kachel { titel: tr!(l, "Time", "Zeit", "Tiempo"), wert: format!("{min}:{sek:02}"), Clock { class: "size-4" } }
                    Kachel { titel: tr!(l, "Streak", "Serie", "Racha"), wert: ergebnis.max_combo.to_string(), Flame { class: "size-4" } }
                }
                div { class: "mt-5 space-y-2",
                    if let Some(s) = b.serie_neu {
                        Badge { Flame { class: "size-5 fill-foreground" } {tr!(l, format!("Daily streak: {s} day(s) in a row!"), format!("Tagesserie: {s} Tag(e) in Folge!"), format!("Racha diaria: ¡{s} día(s) seguidos!"))} }
                    }
                    if b.ziel_erreicht {
                        Badge { Target { class: "size-5" } {tr!(l, "Daily goal reached!", "Tagesziel erreicht!", "¡Objetivo diario cumplido!")} }
                    }
                    if let Some(lv) = b.level_neu {
                        Badge { Gauge { class: "size-5" } {tr!(l, format!("Level up: you're now level {lv}!"), format!("Level-Aufstieg: Du bist jetzt Level {lv}!"), format!("¡Subes de nivel: ahora eres nivel {lv}!"))} }
                    }
                    if b.neue_karte {
                        Badge { StickyNote { class: "size-5" } {tr!(l, "New cheat sheet card collected!", "Neue Spickzettel-Karte gesammelt!", "¡Nueva tarjeta de chuleta conseguida!")} }
                    }
                    if ergebnis.sterne > b.sterne_vorher && b.sterne_vorher > 0 {
                        Badge { Sparkles { class: "size-5" } {tr!(l, format!("New record: {} stars!", ergebnis.sterne), format!("Neuer Rekord: {} Sterne!", ergebnis.sterne), format!("¡Nuevo récord: {} estrellas!", ergebnis.sterne))} }
                    }
                }
                div { class: "mt-8 flex flex-col gap-3 sm:flex-row-reverse",
                    button {
                        class: "h-12 flex-1 rounded-2xl border-2 border-b-4 border-foreground bg-foreground font-bold uppercase tracking-wide text-background transition active:translate-y-0.5 active:border-b-2",
                        "data-primaer": "true",
                        onclick: move |_| { navigator.push(Route::Home {}); },
                        {tr!(l, "Continue", "Weiter", "Continuar")}
                        ArrowRight { class: "ml-2 inline size-4" }
                    }
                    button {
                        class: "h-12 flex-1 rounded-2xl border-2 border-b-4 font-bold uppercase tracking-wide transition hover:bg-accent active:translate-y-0.5 active:border-b-2",
                        onclick: move |_| on_neu.call(()),
                        RotateCcw { class: "mr-2 inline size-4" }
                        {tr!(l, "Again", "Nochmal", "Otra vez")}
                    }
                }
            }
        }
    }
}

#[component]
fn Kachel(titel: &'static str, wert: String, children: Element) -> Element {
    rsx! {
        div { class: "anim-pop rounded-2xl border-2 p-3",
            div { class: "flex items-center justify-center gap-1 whitespace-nowrap text-[0.65rem] font-semibold uppercase text-muted-foreground", {children} "{titel}" }
            div { class: "mt-1 text-2xl font-extrabold", "{wert}" }
        }
    }
}

#[component]
fn Badge(children: Element) -> Element {
    rsx! {
        div { class: "anim-pop flex items-center justify-center gap-2 rounded-2xl bg-muted px-4 py-3 font-semibold", {children} }
    }
}

#[component]
fn Verloren(on_neu: EventHandler<()>) -> Element {
    let l = use_sprache();
    rsx! {
        div { class: "flex min-h-screen flex-col items-center justify-center gap-4 px-4 text-center",
            div { class: "anim-shake flex size-24 items-center justify-center rounded-full border-4 border-foreground", HeartCrack { class: "size-12" } }
            h1 { class: "text-3xl font-extrabold", {tr!(l, "Out of lives", "Keine Leben mehr", "Sin vidas")} }
            p { class: "max-w-sm text-muted-foreground", {tr!(l, "Exams are no walk in the park. Review the theory or try again right away – the exercises are generated anew.", "Prüfungen sind kein Spaziergang. Schau dir die Theorie nochmal an oder versuch es direkt nochmal – die Aufgaben werden neu erzeugt.", "Los exámenes no son un paseo. Repasa la teoría o vuelve a intentarlo: los ejercicios se generan de nuevo.")} }
            div { class: "mt-4 flex flex-col gap-3 sm:flex-row",
                button {
                    class: "h-12 rounded-2xl border-2 border-b-4 border-foreground bg-foreground px-8 font-bold uppercase text-background",
                    onclick: move |_| on_neu.call(()),
                    RotateCcw { class: "mr-2 inline size-4" }
                    {tr!(l, "Try again", "Nochmal versuchen", "Reintentar")}
                }
                Link { to: Route::Home {}, class: "flex h-12 items-center rounded-2xl border-2 border-b-4 px-8 font-bold uppercase",
                    BookOpen { class: "mr-2 size-4" }
                    {tr!(l, "To the learning path", "Zum Lernpfad", "Al camino de aprendizaje")}
                }
            }
        }
    }
}
