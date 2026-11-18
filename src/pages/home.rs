//! The learning path: all worlds of a course with their level dots,
//! connected like a world map.

use dioxus::prelude::*;
use icons::{
    BookOpen, Check, Code, Crown, Dumbbell, Flame, HardDrive, Lock, RotateCcw, Shield, Star, StickyNote, Target, Trophy, Zap,
};

use crate::Route;
use crate::components::rahmen::use_kurs;
use crate::components::spick::TaschenSpick;
use crate::components::text::RichText;
use crate::components::ui::{Button, ButtonSize, ButtonVariant};
use crate::i18n::{Sprache, use_sprache};
use crate::inhalt::use_inhalt;
use crate::kurs::Kurs;
use crate::progress::{XP_PRO_LEVEL, aendern, use_fortschritt, zuruecksetzen};
use crate::welt::{Art, Knoten};

/// Horizontal offsets of the dots – this makes the winding path.
const VERSATZ: &[f64] = &[0.0, -55.0, -95.0, -55.0, 0.0, 55.0, 95.0, 55.0];
const ZEILE: f64 = 104.0;
const BREITE: f64 = 320.0;

/// Where a level starts: theory page or the player.
pub fn start_route(l: Sprache, kurs: Kurs, k: &Knoten) -> Route {
    if k.art == Art::Theorie {
        Route::LektionSeite { lang: l, kurs, id: k.welt + 1 }
    } else {
        Route::Spieler { lang: l, kurs, id: k.id.clone() }
    }
}

#[component]
pub fn Home(lang: Sprache, kurs: Kurs) -> Element {
    let _ = (lang, kurs);
    let fortschritt = use_fortschritt();
    let l = use_sprache();
    let kurs = use_kurs();
    let inhalt = use_inhalt();
    let f = fortschritt.read();
    let alle = inhalt.knoten();
    let aktuell = f.aktuell(alle);
    let fertig = inhalt.fertig(&f);
    let karten = (0..inhalt.anzahl_welten()).filter(|w| inhalt.karte_offen(&f, *w)).count();
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
                for w in 0..inhalt.anzahl_welten() {
                    WeltAbschnitt { key: "{w}", welt: w, aktuell, gewaehlt }
                }
                div { class: "mt-6 flex flex-col items-center gap-2 pb-10 text-center",
                    Crown { class: "size-10" }
                    p { class: "font-semibold", {t!(l, END_OF_MAP)} }
                    p { class: "text-sm text-muted-foreground", {tf!(l, END_OF_MAP_TEXT, levels = alle.len(), worlds = inhalt.anzahl_welten())} }
                }
            }

            // Seitenleiste
            aside { class: "order-first space-y-4 lg:order-none lg:sticky lg:top-20 lg:self-start",
                div { class: "rounded-2xl border-2 p-5",
                    div { class: "flex items-center gap-4",
                        div { class: "flex size-14 items-center justify-center rounded-2xl bg-foreground text-2xl font-extrabold text-background", "{f.level()}" }
                        div { class: "flex-1",
                            div { class: "text-sm text-muted-foreground", {t!(l, LEVEL_LABEL)} }
                            div { class: "font-bold", {tf!(l, XP_TOTAL, xp = f.xp)} }
                            div { class: "mt-1.5 h-2 overflow-hidden rounded-full bg-muted",
                                div { class: "h-full rounded-full bg-foreground transition-all", style: "width: {f64::from(level_xp) / f64::from(XP_PRO_LEVEL) * 100.0}%" }
                            }
                            div { class: "mt-1 text-xs text-muted-foreground", {tf!(l, XP_TO_LEVEL, xp = XP_PRO_LEVEL - level_xp, level = f.level() + 1)} }
                        }
                    }
                }
                div { class: "grid grid-cols-2 gap-3",
                    div { class: "rounded-2xl border-2 p-4",
                        div { class: "flex items-center gap-1.5 text-xs font-semibold uppercase tracking-wide text-muted-foreground", Flame { class: "size-4" } {t!(l, STREAK_SHORT)} }
                        div { class: "mt-1 text-2xl font-extrabold", "{f.serie}" }
                        div { class: "text-xs text-muted-foreground", if f.ist_heute_aktiv() { {t!(l, DONE_TODAY)} } else { {t!(l, OPEN_TODAY)} } }
                    }
                    div { class: "rounded-2xl border-2 p-4",
                        div { class: "flex items-center gap-1.5 text-xs font-semibold uppercase tracking-wide text-muted-foreground", Target { class: "size-4" } {t!(l, DAILY_GOAL)} }
                        div { class: "mt-1 text-2xl font-extrabold", "{f.heute_xp}/{f.ziel}" }
                        div { class: "mt-1 h-1.5 overflow-hidden rounded-full bg-muted",
                            div { class: "h-full rounded-full bg-foreground", style: "width: {ziel_prozent}%" }
                        }
                    }
                    div { class: "rounded-2xl border-2 p-4",
                        div { class: "flex items-center gap-1.5 text-xs font-semibold uppercase tracking-wide text-muted-foreground", Trophy { class: "size-4" } {t!(l, LEVELS)} }
                        div { class: "mt-1 text-2xl font-extrabold", "{fertig}/{alle.len()}" }
                    }
                    div { class: "rounded-2xl border-2 p-4",
                        div { class: "flex items-center gap-1.5 text-xs font-semibold uppercase tracking-wide text-muted-foreground", StickyNote { class: "size-4" } {t!(l, CARDS)} }
                        div { class: "mt-1 text-2xl font-extrabold", "{karten}/{inhalt.anzahl_welten()}" }
                    }
                }
                if let Some(k) = alle.get(aktuell) {
                    Link {
                        to: start_route(l, kurs, k),
                        class: "flex h-12 items-center justify-center gap-2 rounded-2xl border-2 border-b-4 border-foreground bg-foreground font-bold uppercase tracking-wide text-background transition active:translate-y-0.5 active:border-b-2",
                        if fertig == 0 { {t!(l, LETS_GO)} } else { {tf!(l, CONTINUE_LEVEL, n = k.nr)} }
                    }
                }
                div { class: "rounded-2xl border-2 border-dashed p-4 text-sm",
                    div { class: "mb-2 font-semibold", {t!(l, SETTINGS)} }
                    label { class: "flex cursor-pointer items-center gap-2 py-1",
                        input { r#type: "checkbox", class: "size-4 accent-[var(--foreground)]", checked: f.alles_offen, onchange: move |e| aendern(fortschritt, |f| f.alles_offen = e.checked()) }
                        {t!(l, UNLOCK_ALL)}
                    }
                    label { class: "flex cursor-pointer items-center gap-2 py-1",
                        input { r#type: "checkbox", class: "size-4 accent-[var(--foreground)]", checked: f.ton, onchange: move |e| aendern(fortschritt, |f| f.ton = e.checked()) }
                        {t!(l, SOUNDS_CORRECT)}
                    }
                    div { class: "flex flex-wrap items-center gap-2 py-1",
                        span { {t!(l, DAILY_GOAL_LABEL)} }
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
                            span { class: "me-1 text-xs", {t!(l, RESET_CONFIRM)} }
                            Button { variant: ButtonVariant::Destructive, size: ButtonSize::Sm, onclick: move |_| { zuruecksetzen(fortschritt); bestaetigen.set(false); }, {t!(l, YES)} }
                            Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, onclick: move |_| bestaetigen.set(false), {t!(l, NO)} }
                        } else {
                            Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, class: "px-0 text-muted-foreground", onclick: move |_| bestaetigen.set(true), RotateCcw {} {t!(l, RESET_PROGRESS)} }
                        }
                    }
                    Link { to: Route::Profil { lang: l, kurs }, class: "mt-1 flex items-center gap-1.5 text-xs text-muted-foreground underline-offset-2 hover:text-foreground hover:underline",
                        HardDrive { class: "size-3.5 shrink-0" }
                        {t!(l, SAVED_HINT)}
                    }
                }
            }
        }
        TaschenSpick {}
    }
}

#[component]
fn WeltAbschnitt(welt: usize, aktuell: usize, gewaehlt: Signal<Option<usize>>) -> Element {
    let fortschritt = use_fortschritt();
    let l = use_sprache();
    let inhalt = use_inhalt();
    let f = fortschritt.read();
    let knoten: Vec<(usize, &Knoten)> = inhalt.knoten().iter().enumerate().filter(|(_, k)| k.welt == welt).collect();
    let fertig = knoten.iter().filter(|(_, k)| f.fertig(&k.id)).count();
    let sterne: u32 = knoten.iter().map(|(_, k)| u32::from(f.sterne(&k.id))).sum();
    let offen = knoten.first().is_some_and(|(i, _)| f.offen(inhalt.knoten(), *i));

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
                    div { class: "text-xs font-bold uppercase tracking-widest opacity-70", {tf!(l, WORLD_N_CHAPTER, n = welt + 1, chapter = inhalt.welt_kapitel(welt))} }
                    div { class: "text-xl font-extrabold", "{inhalt.welt_titel(welt)}" }
                    div { class: "text-sm opacity-80", "{inhalt.welt_kurz(welt)}" }
                }
                div { class: "text-end text-sm font-semibold",
                    div { class: "flex items-center justify-end gap-1", Star { class: "size-4 fill-current" } "{sterne}/{knoten.len() * 3}" }
                    div { class: "opacity-70", {tf!(l, LEVELS_DONE_OF, done = fertig, total = knoten.len())} }
                }
            }
            // The map keeps its shape in right-to-left languages too.
            div { class: "relative mx-auto", dir: "ltr", style: "width: {BREITE}px; height: {hoehe}px",
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
    let kurs = use_kurs();
    let inhalt = use_inhalt();
    let f = fortschritt.read();
    let k = &inhalt.knoten()[index];
    let fertig = f.fertig(&k.id);
    let offen = f.offen(inhalt.knoten(), index);
    let sterne = f.sterne(&k.id);
    let gross = matches!(k.art, Art::Meister | Art::Pruefung);
    let groesse = if k.art == Art::Meister { 84.0 } else if gross { 76.0 } else { 68.0 };
    let ist_gewaehlt = gewaehlt() == Some(index);
    let welt_titel = inhalt.welt_titel(k.welt);

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
                    if fertig { {t!(l, REPEAT)} } else { {t!(l, START)} }
                    div { class: "absolute -bottom-1.5 left-1/2 size-3 -translate-x-1/2 rotate-45 border-b-2 border-r-2 bg-card" }
                }
            }
            button {
                class: "relative flex flex-col items-center justify-center rounded-full border-4 font-extrabold transition hover:brightness-110 active:translate-y-1 active:shadow-none {kreis}",
                style: "width: {groesse}px; height: {groesse}px",
                title: tf!(l, LEVEL_N_TITLE, n = k.nr, title = k.titel(l)),
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
                    class: "anim-pop absolute left-1/2 top-full z-30 mt-4 w-72 -translate-x-1/2 rounded-2xl border-2 bg-card p-4 text-start shadow-2xl",
                    dir: if l.rtl() { "rtl" } else { "ltr" },
                    onclick: move |e| e.stop_propagation(),
                    div { class: "absolute -top-2 left-1/2 size-4 -translate-x-1/2 rotate-45 border-l-2 border-t-2 bg-card" }
                    div { class: "flex items-center gap-2 text-xs font-bold uppercase tracking-widest text-muted-foreground",
                        {tf!(l, LEVEL_N, n = k.nr)}
                        if k.ist_pruefung() {
                            span { class: "ms-auto flex items-center gap-1 rounded-full border px-2 py-0.5 normal-case tracking-normal",
                                if k.spickzettel_erlaubt() { StickyNote { class: "size-3" } {t!(l, CHEATSHEET_ALLOWED)} } else { Lock { class: "size-3" } {t!(l, NO_CHEATSHEET)} }
                            }
                        }
                    }
                    div { class: "mt-1 text-lg font-extrabold", "{k.titel(l)}" }
                    RichText { text: k.beschreibung(l, &welt_titel), class: "mt-1 text-sm leading-6 text-muted-foreground" }
                    if offen {
                        Link {
                            to: start_route(l, kurs, k),
                            class: "mt-3 flex h-11 items-center justify-center gap-2 rounded-xl border-2 border-b-4 border-foreground bg-foreground font-bold uppercase tracking-wide text-background transition active:translate-y-0.5 active:border-b-2",
                            if fertig { RotateCcw { class: "size-4" } {t!(l, AGAIN)} } else if k.art == Art::Theorie { BookOpen { class: "size-4" } {t!(l, READ)} } else { Zap { class: "size-4" } {t!(l, START_LEVEL)} }
                        }
                    } else {
                        div { class: "mt-3 flex h-11 items-center justify-center gap-2 rounded-xl border-2 border-dashed text-sm text-muted-foreground",
                            Lock { class: "size-4" }
                            {tf!(l, FINISH_FIRST, n = k.nr - 1)}
                        }
                    }
                }
            }
        }
    }
}
