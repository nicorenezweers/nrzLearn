//! The learning path: all worlds with their level dots, connected like a world map.

use dioxus::prelude::*;
use icons::{
    BookOpen, Check, Code, Crown, Dumbbell, Flame, Lock, RotateCcw, Shield, Star, StickyNote, Target, Trophy, Zap,
};

use crate::Route;
use crate::components::spick::TaschenSpick;
use crate::components::text::RichText;
use crate::components::ui::{Button, ButtonSize, ButtonVariant};
use crate::content::{ANZAHL_WELTEN, lektionen};
use crate::i18n::use_sprache;
use crate::progress::{XP_PRO_LEVEL, aendern, use_fortschritt, zuruecksetzen};
use crate::welt::{self, Art, Knoten};

/// Horizontal offsets of the dots – this makes the winding path.
const VERSATZ: &[f64] = &[0.0, -55.0, -95.0, -55.0, 0.0, 55.0, 95.0, 55.0];
const ZEILE: f64 = 104.0;
const BREITE: f64 = 320.0;

#[component]
pub fn Home() -> Element {
    let fortschritt = use_fortschritt();
    let l = use_sprache();
    let f = fortschritt.read();
    let aktuell = f.aktuell();
    let alle = welt::alle();
    let fertig = alle.iter().filter(|k| f.fertig(&k.id)).count();
    let level_xp = f.xp % XP_PRO_LEVEL;
    let ziel_prozent = (f64::from(f.heute_xp) / f64::from(f.ziel.max(1)) * 100.0).min(100.0);
    let mut bestaetigen = use_signal(|| false);
    let mut gewaehlt = use_signal(|| None::<usize>);

    // Scroll to the current level when opening.
    use_effect(move || {
        document::eval("setTimeout(()=>document.getElementById('aktuell')?.scrollIntoView({block:'center',behavior:'smooth'}),150)");
    });

    rsx! {
        div { class: "mx-auto grid max-w-6xl gap-8 px-4 py-8 lg:grid-cols-[1fr_20rem]",
            // Pfad
            div { class: "min-w-0", onclick: move |_| gewaehlt.set(None),
                for w in 0..ANZAHL_WELTEN {
                    WeltAbschnitt { key: "{w}", welt: w, aktuell, gewaehlt }
                }
                div { class: "mt-6 flex flex-col items-center gap-2 pb-10 text-center",
                    Crown { class: "size-10" }
                    p { class: "font-semibold", {tr!(l, "End of the world map", "Ende der Weltkarte", "Fin del mapa")} }
                    p { class: "text-sm text-muted-foreground", {tr!(l, format!("{} levels, {} exercise generators – every round is new. Collect all the stars!", alle.len(), crate::aufgaben::anzahl_generatoren()), format!("{} Level, {} Aufgaben-Generatoren – jede Runde ist neu. Sammle alle Sterne!", alle.len(), crate::aufgaben::anzahl_generatoren()), format!("{} niveles, {} generadores de ejercicios: cada ronda es nueva. ¡Consigue todas las estrellas!", alle.len(), crate::aufgaben::anzahl_generatoren()))} }
                }
            }

            // Seitenleiste
            aside { class: "order-first space-y-4 lg:order-none lg:sticky lg:top-20 lg:self-start",
                div { class: "rounded-2xl border-2 p-5",
                    div { class: "flex items-center gap-4",
                        div { class: "flex size-14 items-center justify-center rounded-2xl bg-foreground text-2xl font-extrabold text-background", "{f.level()}" }
                        div { class: "flex-1",
                            div { class: "text-sm text-muted-foreground", "Level" }
                            div { class: "font-bold", {tr!(l, format!("{} XP total", f.xp), format!("{} XP gesamt", f.xp), format!("{} XP en total", f.xp))} }
                            div { class: "mt-1.5 h-2 overflow-hidden rounded-full bg-muted",
                                div { class: "h-full rounded-full bg-foreground transition-all", style: "width: {f64::from(level_xp) / f64::from(XP_PRO_LEVEL) * 100.0}%" }
                            }
                            div { class: "mt-1 text-xs text-muted-foreground", {tr!(l, format!("{} XP to level {}", XP_PRO_LEVEL - level_xp, f.level() + 1), format!("{} XP bis Level {}", XP_PRO_LEVEL - level_xp, f.level() + 1), format!("{} XP para el nivel {}", XP_PRO_LEVEL - level_xp, f.level() + 1))} }
                        }
                    }
                }
                div { class: "grid grid-cols-2 gap-3",
                    div { class: "rounded-2xl border-2 p-4",
                        div { class: "flex items-center gap-1.5 text-xs font-semibold uppercase tracking-wide text-muted-foreground", Flame { class: "size-4" } {tr!(l, "Streak", "Serie", "Racha")} }
                        div { class: "mt-1 text-2xl font-extrabold", "{f.serie}" }
                        div { class: "text-xs text-muted-foreground", if f.ist_heute_aktiv() { {tr!(l, "done today ✓", "heute erledigt ✓", "hecho hoy ✓")} } else { {tr!(l, "still open today", "heute noch offen", "pendiente hoy")} } }
                    }
                    div { class: "rounded-2xl border-2 p-4",
                        div { class: "flex items-center gap-1.5 text-xs font-semibold uppercase tracking-wide text-muted-foreground", Target { class: "size-4" } {tr!(l, "Daily goal", "Tagesziel", "Objetivo diario")} }
                        div { class: "mt-1 text-2xl font-extrabold", "{f.heute_xp}/{f.ziel}" }
                        div { class: "mt-1 h-1.5 overflow-hidden rounded-full bg-muted",
                            div { class: "h-full rounded-full bg-foreground", style: "width: {ziel_prozent}%" }
                        }
                    }
                    div { class: "rounded-2xl border-2 p-4",
                        div { class: "flex items-center gap-1.5 text-xs font-semibold uppercase tracking-wide text-muted-foreground", Trophy { class: "size-4" } "Level" }
                        div { class: "mt-1 text-2xl font-extrabold", "{fertig}/{alle.len()}" }
                    }
                    div { class: "rounded-2xl border-2 p-4",
                        div { class: "flex items-center gap-1.5 text-xs font-semibold uppercase tracking-wide text-muted-foreground", StickyNote { class: "size-4" } {tr!(l, "Cards", "Karten", "Tarjetas")} }
                        div { class: "mt-1 text-2xl font-extrabold", "{f.karten()}/16" }
                    }
                }
                Link {
                    to: start_route(&alle[aktuell]),
                    class: "flex h-12 items-center justify-center gap-2 rounded-2xl border-2 border-b-4 border-foreground bg-foreground font-bold uppercase tracking-wide text-background transition active:translate-y-0.5 active:border-b-2",
                    if fertig == 0 { {tr!(l, "Let's go", "Los geht's", "Vamos")} } else { {tr!(l, format!("Continue: level {}", alle[aktuell].nr), format!("Weiter: Level {}", alle[aktuell].nr), format!("Seguir: nivel {}", alle[aktuell].nr))} }
                }
                div { class: "rounded-2xl border-2 border-dashed p-4 text-sm",
                    div { class: "mb-2 font-semibold", {tr!(l, "Settings", "Einstellungen", "Ajustes")} }
                    label { class: "flex cursor-pointer items-center gap-2 py-1",
                        input { r#type: "checkbox", class: "size-4 accent-[var(--foreground)]", checked: f.alles_offen, onchange: move |e| aendern(fortschritt, |f| f.alles_offen = e.checked()) }
                        {tr!(l, "Unlock all levels", "Alle Level freischalten", "Desbloquear todos los niveles")}
                    }
                    label { class: "flex cursor-pointer items-center gap-2 py-1",
                        input { r#type: "checkbox", class: "size-4 accent-[var(--foreground)]", checked: f.ton, onchange: move |e| aendern(fortschritt, |f| f.ton = e.checked()) }
                        {tr!(l, "Sounds for correct answers", "Töne bei richtigen Antworten", "Sonidos en respuestas correctas")}
                    }
                    div { class: "flex items-center gap-2 py-1",
                        span { {tr!(l, "Daily goal:", "Tagesziel:", "Objetivo diario:")} }
                        for z in [30_u32, 60, 120] {
                            button {
                                key: "{z}",
                                class: if f.ziel == z { "rounded-lg bg-foreground px-2 py-0.5 text-xs font-semibold text-background" } else { "rounded-lg border px-2 py-0.5 text-xs" },
                                onclick: move |_| aendern(fortschritt, |f| f.ziel = z),
                                "{z} XP"
                            }
                        }
                    }
                    div { class: "mt-2",
                        if bestaetigen() {
                            span { class: "mr-1 text-xs", {tr!(l, "Really delete everything?", "Wirklich alles löschen?", "¿Borrar todo de verdad?")} }
                            Button { variant: ButtonVariant::Destructive, size: ButtonSize::Sm, onclick: move |_| { zuruecksetzen(fortschritt); bestaetigen.set(false); }, {tr!(l, "Yes", "Ja", "Sí")} }
                            Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, onclick: move |_| bestaetigen.set(false), {tr!(l, "No", "Nein", "No")} }
                        } else {
                            Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, class: "px-0 text-muted-foreground", onclick: move |_| bestaetigen.set(true), RotateCcw {} {tr!(l, "Reset progress", "Fortschritt zurücksetzen", "Reiniciar progreso")} }
                        }
                    }
                }
            }
        }
        TaschenSpick {}
    }
}

pub fn start_route(k: &Knoten) -> Route {
    if k.art == Art::Theorie {
        Route::LektionSeite { id: k.welt + 1 }
    } else {
        Route::Spieler { id: k.id.clone() }
    }
}

#[component]
fn WeltAbschnitt(welt: usize, aktuell: usize, gewaehlt: Signal<Option<usize>>) -> Element {
    let fortschritt = use_fortschritt();
    let sp = use_sprache();
    let f = fortschritt.read();
    let l = &lektionen(sp)[welt];
    let knoten: Vec<(usize, &Knoten)> = welt::alle().iter().enumerate().filter(|(_, k)| k.welt == welt).collect();
    let fertig = knoten.iter().filter(|(_, k)| f.fertig(&k.id)).count();
    let sterne: u32 = knoten.iter().map(|(_, k)| u32::from(f.sterne(&k.id))).sum();
    let offen = knoten.first().is_some_and(|(i, _)| f.offen(*i));

    let punkte: Vec<(f64, f64)> = knoten.iter().enumerate().map(|(j, _)| (BREITE / 2.0 + VERSATZ[j % VERSATZ.len()], 48.0 + j as f64 * ZEILE)).collect();
    let hoehe = 48.0 + knoten.len() as f64 * ZEILE;
    let pfad: String = punkte.iter().enumerate().map(|(j, (x, y))| if j == 0 { format!("M{x},{y}") } else { format!(" L{x},{y}") }).collect();
    let fertig_bis = knoten.iter().take_while(|(_, k)| f.fertig(&k.id)).count();
    let pfad_fertig: String = punkte.iter().take(fertig_bis + 1).enumerate().map(|(j, (x, y))| if j == 0 { format!("M{x},{y}") } else { format!(" L{x},{y}") }).collect();

    rsx! {
        section { class: "mb-6",
            div {
                class: if offen { "sticky top-16 z-20 mb-6 flex items-center gap-4 rounded-2xl bg-foreground p-5 text-background shadow-lg" } else { "mb-6 flex items-center gap-4 rounded-2xl border-2 border-dashed p-5 text-muted-foreground" },
                div { class: "flex-1",
                    div { class: "text-xs font-bold uppercase tracking-widest opacity-70", {tr!(sp, format!("World {} · {}", welt + 1, l.kapitel), format!("Welt {} · {}", welt + 1, l.kapitel), format!("Mundo {} · {}", welt + 1, l.kapitel))} }
                    div { class: "text-xl font-extrabold", "{l.titel}" }
                    div { class: "text-sm opacity-80", "{l.kurz}" }
                }
                div { class: "text-right text-sm font-semibold",
                    div { class: "flex items-center justify-end gap-1", Star { class: "size-4 fill-current" } "{sterne}/{knoten.len() * 3}" }
                    div { class: "opacity-70", {tr!(sp, format!("{fertig}/{} levels", knoten.len()), format!("{fertig}/{} Level", knoten.len()), format!("{fertig}/{} niveles", knoten.len()))} }
                }
            }
            div { class: "relative mx-auto", style: "width: {BREITE}px; height: {hoehe}px",
                svg { class: "absolute inset-0", width: "{BREITE}", height: "{hoehe}", view_box: "0 0 {BREITE} {hoehe}", "aria-hidden": "true",
                    path { d: "{pfad}", fill: "none", stroke: "var(--border)", stroke_width: "10", stroke_linecap: "round", stroke_linejoin: "round", stroke_dasharray: "2 18" }
                    if fertig_bis > 0 {
                        path { d: "{pfad_fertig}", fill: "none", stroke: "var(--foreground)", stroke_width: "6", stroke_linecap: "round", stroke_linejoin: "round", stroke_dasharray: "2 14" }
                    }
                }
                for (j, (i, k)) in knoten.iter().enumerate() {
                    Punkt { key: "{k.id}", index: *i, x: punkte[j].0, y: punkte[j].1, ist_aktuell: *i == aktuell, gewaehlt }
                }
            }
        }
    }
}

#[component]
fn Punkt(index: usize, x: f64, y: f64, ist_aktuell: bool, gewaehlt: Signal<Option<usize>>) -> Element {
    let fortschritt = use_fortschritt();
    let l = use_sprache();
    let f = fortschritt.read();
    let k = &welt::alle()[index];
    let fertig = f.fertig(&k.id);
    let offen = f.offen(index);
    let sterne = f.sterne(&k.id);
    let gross = matches!(k.art, Art::Meister | Art::Pruefung);
    let groesse = if k.art == Art::Meister { 84.0 } else if gross { 76.0 } else { 68.0 };
    let ist_gewaehlt = gewaehlt() == Some(index);

    let kreis = if fertig {
        "border-foreground bg-foreground text-background shadow-[0_6px_0_0_var(--muted-foreground)]"
    } else if ist_aktuell && offen {
        "anim-ring border-foreground bg-foreground text-background shadow-[0_6px_0_0_var(--muted-foreground)]"
    } else if offen {
        "border-foreground bg-card text-foreground shadow-[0_6px_0_0_var(--border)]"
    } else {
        "border-border bg-muted text-muted-foreground shadow-[0_6px_0_0_var(--border)]"
    };

    let icon = match k.art {
        Art::Theorie => rsx! { BookOpen { class: "size-3.5" } },
        Art::Uebung(_) => rsx! { Dumbbell { class: "size-3.5" } },
        Art::Labor => rsx! { Code { class: "size-3.5" } },
        Art::Pruefung => rsx! { Shield { class: "size-3.5" } },
        Art::Meister => rsx! { Crown { class: "size-3.5" } },
    };

    rsx! {
        div {
            id: if ist_aktuell { "aktuell" } else { "" },
            class: if ist_gewaehlt { "absolute z-10 -translate-x-1/2 -translate-y-1/2" } else { "absolute -translate-x-1/2 -translate-y-1/2" },
            style: "left: {x}px; top: {y}px",
            if ist_aktuell && offen && !ist_gewaehlt {
                div { class: "anim-bounce absolute -top-11 left-1/2 -translate-x-1/2 whitespace-nowrap rounded-xl border-2 bg-card px-3 py-1 text-xs font-extrabold uppercase tracking-wide shadow",
                    if f.fertig(&k.id) { {tr!(l, "Repeat", "Wiederholen", "Repetir")} } else { {tr!(l, "Start", "Start", "Empezar")} }
                    div { class: "absolute -bottom-1.5 left-1/2 size-3 -translate-x-1/2 rotate-45 border-b-2 border-r-2 bg-card" }
                }
            }
            button {
                class: "relative flex flex-col items-center justify-center rounded-full border-4 font-extrabold transition hover:brightness-110 active:translate-y-1 active:shadow-none {kreis}",
                style: "width: {groesse}px; height: {groesse}px",
                title: "Level {k.nr}: {k.titel(l)}",
                onclick: move |e| {
                    e.stop_propagation();
                    gewaehlt.set(if gewaehlt() == Some(index) { None } else { Some(index) });
                },
                if !offen {
                    Lock { class: "size-6" }
                } else if k.art == Art::Meister {
                    Crown { class: "size-7" }
                } else {
                    span { class: if gross { "text-2xl leading-none" } else { "text-xl leading-none" }, "{k.nr}" }
                }
                span { class: "absolute -bottom-1 -right-1 flex size-6 items-center justify-center rounded-full border-2 border-background bg-foreground text-background",
                    if fertig { Check { class: "size-3.5" } } else { {icon} }
                }
            }
            if fertig {
                div { class: "mt-2 flex justify-center gap-0.5",
                    for s in 0..3_u8 {
                        Star { key: "{s}", class: if s < sterne { "size-3.5 fill-foreground text-foreground" } else { "size-3.5 text-muted-foreground/40" } }
                    }
                }
            }
            if ist_gewaehlt {
                div {
                    class: "anim-pop absolute left-1/2 top-full z-30 mt-4 w-72 -translate-x-1/2 rounded-2xl border-2 bg-card p-4 text-left shadow-2xl",
                    onclick: move |e| e.stop_propagation(),
                    div { class: "absolute -top-2 left-1/2 size-4 -translate-x-1/2 rotate-45 border-l-2 border-t-2 bg-card" }
                    div { class: "flex items-center gap-2 text-xs font-bold uppercase tracking-widest text-muted-foreground",
                        {tr!(l, format!("Level {}", k.nr), format!("Level {}", k.nr), format!("Nivel {}", k.nr))}
                        if k.ist_pruefung() {
                            span { class: "ml-auto flex items-center gap-1 rounded-full border px-2 py-0.5 normal-case tracking-normal",
                                if k.spickzettel_erlaubt() { StickyNote { class: "size-3" } {tr!(l, "cheat sheet allowed", "Spickzettel erlaubt", "chuleta permitida")} } else { Lock { class: "size-3" } {tr!(l, "no cheat sheet", "ohne Spickzettel", "sin chuleta")} }
                            }
                        }
                    }
                    div { class: "mt-1 text-lg font-extrabold", "{k.titel(l)}" }
                    RichText { text: k.beschreibung(l), class: "mt-1 text-sm leading-6 text-muted-foreground" }
                    if offen {
                        Link {
                            to: start_route(k),
                            class: "mt-3 flex h-11 items-center justify-center gap-2 rounded-xl border-2 border-b-4 border-foreground bg-foreground font-bold uppercase tracking-wide text-background transition active:translate-y-0.5 active:border-b-2",
                            if fertig { RotateCcw { class: "size-4" } {tr!(l, "Again", "Nochmal", "Otra vez")} } else if k.art == Art::Theorie { BookOpen { class: "size-4" } {tr!(l, "Read", "Lesen", "Leer")} } else { Zap { class: "size-4" } {tr!(l, "Start", "Starten", "Empezar")} }
                        }
                    } else {
                        div { class: "mt-3 flex h-11 items-center justify-center gap-2 rounded-xl border-2 border-dashed text-sm text-muted-foreground",
                            Lock { class: "size-4" }
                            {tr!(l, format!("Finish level {} first", k.nr - 1), format!("Schließe Level {} ab", k.nr - 1), format!("Termina el nivel {}", k.nr - 1))}
                        }
                    }
                }
            }
        }
    }
}
