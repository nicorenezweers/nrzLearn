//! Profile: statistics, progress in every course and per world, and the
//! backup of the progress, which only exists in this browser.

use dioxus::prelude::*;
use icons::{Check, Download, Flame, HardDrive, RotateCcw, Star, Target, TriangleAlert, Upload, UserRound, Zap};

use crate::Route;
use crate::components::kurswahl::KursKachel;
use crate::components::ui::{Button, ButtonSize, ButtonVariant};
use crate::i18n::{Sprache, use_sprache};
use crate::inhalt::use_inhalt;
use crate::kurs::Kurs;
use crate::progress::{
    Fortschritt, ersetzen, export_dateiname, export_json, import_json, speicher_verfuegbar, use_fortschritt, zuruecksetzen,
};

const KNOPF: &str = "inline-flex h-10 cursor-pointer items-center justify-center gap-2 rounded-xl border-2 border-b-4 px-4 text-sm font-bold transition hover:bg-accent active:translate-y-0.5 active:border-b-2";
const KNOPF_VOLL: &str = "inline-flex h-10 cursor-pointer items-center justify-center gap-2 rounded-xl border-2 border-b-4 border-foreground bg-foreground px-4 text-sm font-bold text-background transition active:translate-y-0.5 active:border-b-2";

#[component]
pub fn Profil(lang: Sprache, kurs: Kurs) -> Element {
    let _ = (lang, kurs);
    let l = use_sprache();
    let fortschritt = use_fortschritt();
    let f = fortschritt.read();

    rsx! {
        div { class: "mx-auto max-w-4xl space-y-8 px-4 py-8",
            div { class: "flex items-center gap-3",
                div { class: "flex size-12 items-center justify-center rounded-2xl bg-foreground text-background", UserRound { class: "size-6" } }
                div {
                    h1 { class: "text-3xl font-extrabold tracking-tight", {t!(l, PROFILE_TITLE)} }
                    p { class: "text-muted-foreground", {t!(l, PROFILE_SUBTITLE)} }
                }
            }

            if !speicher_verfuegbar() {
                div { class: "anim-shake flex gap-3 rounded-2xl border-2 border-foreground p-4",
                    TriangleAlert { class: "size-5 shrink-0" }
                    p { class: "text-sm font-semibold", {t!(l, STORAGE_BLOCKED)} }
                }
            }

            div { class: "grid grid-cols-2 gap-3 sm:grid-cols-5",
                Kachel { icon: rsx! { Zap { class: "size-4" } }, titel: t!(l, LEVEL_LABEL), wert: "{f.level()}", unten: tf!(l, XP_TOTAL, xp = f.xp) }
                Kachel { icon: rsx! { Flame { class: "size-4" } }, titel: t!(l, STREAK_SHORT), wert: "{f.serie}", unten: t!(l, DAYS_IN_A_ROW) }
                Kachel { icon: rsx! { Check { class: "size-4" } }, titel: t!(l, EXERCISES), wert: "{f.geloest}", unten: t!(l, SOLVED) }
                Kachel { icon: rsx! { Star { class: "size-4 fill-foreground" } }, titel: t!(l, PERFECT_TITLE), wert: "{f.perfekt}", unten: t!(l, ROUNDS_WITHOUT_MISTAKE) }
                Kachel { icon: rsx! { Target { class: "size-4" } }, titel: t!(l, TODAY), wert: "{f.heute_xp}/{f.ziel}", unten: t!(l, XP_OF_GOAL) }
            }

            section { class: "rounded-2xl border-2",
                h2 { class: "border-b-2 p-5 text-lg font-bold", {t!(l, COURSES)} }
                div { class: "grid gap-px bg-border sm:grid-cols-2",
                    for k in Kurs::ALLE {
                        Link {
                            key: "{k.id()}",
                            to: Route::Home { lang: l, kurs: k },
                            class: "flex items-center gap-3 bg-background px-5 py-3 transition hover:bg-accent",
                            KursKachel { kurs: k, class: "size-9 text-xs" }
                            span { class: "flex-1 font-semibold", "{k.titel(l)}" }
                            span { class: "text-sm text-muted-foreground", {tf!(l, START_LEVELS_DONE, n = f.fertig_im_kurs(k))} }
                        }
                    }
                }
            }

            Welten {}
            Sicherung {}
        }
    }
}

#[component]
fn Kachel(icon: Element, #[props(into)] titel: String, #[props(into)] wert: String, #[props(into)] unten: String) -> Element {
    rsx! {
        div { class: "rounded-2xl border-2 p-4",
            div { class: "flex items-center gap-1.5 text-xs font-semibold uppercase tracking-wide text-muted-foreground", {icon} "{titel}" }
            div { class: "mt-1 text-2xl font-extrabold", "{wert}" }
            div { class: "text-xs text-muted-foreground", "{unten}" }
        }
    }
}

/// Progress per world of the current course.
#[component]
fn Welten() -> Element {
    let l = use_sprache();
    let inhalt = use_inhalt();
    let fortschritt = use_fortschritt();
    let f = fortschritt.read();
    rsx! {
        section { class: "rounded-2xl border-2",
            h2 { class: "flex items-center gap-2 border-b-2 p-5 text-lg font-bold",
                KursKachel { kurs: inhalt.kurs, class: "size-7 rounded-lg text-[0.65rem]" }
                {format!("{} · {}", inhalt.kurs.titel(l), t!(l, WORLDS))}
            }
            div { class: "divide-y",
                for w in 0..inhalt.anzahl_welten() {
                    {
                        let knoten: Vec<_> = inhalt.knoten().iter().filter(|k| k.welt == w).collect();
                        let fertig = knoten.iter().filter(|k| f.fertig(&k.id)).count();
                        let sterne: u32 = knoten.iter().map(|k| u32::from(f.sterne(&k.id))).sum();
                        let prozent = fertig as f64 / knoten.len().max(1) as f64 * 100.0;
                        rsx! {
                            div { key: "{w}", class: "flex items-center gap-4 px-5 py-3",
                                div { class: if fertig == knoten.len() { "flex size-9 shrink-0 items-center justify-center rounded-xl bg-foreground font-bold text-background" } else { "flex size-9 shrink-0 items-center justify-center rounded-xl border-2 font-bold" }, "{w + 1}" }
                                div { class: "min-w-0 flex-1",
                                    div { class: "truncate font-semibold", "{inhalt.welt_titel(w)}" }
                                    div { class: "mt-1.5 h-1.5 overflow-hidden rounded-full bg-muted",
                                        div { class: "h-full rounded-full bg-foreground transition-all", style: "width: {prozent}%" }
                                    }
                                }
                                div { class: "shrink-0 text-end text-sm",
                                    div { class: "font-mono font-bold", "{fertig}/{knoten.len()}" }
                                    div { class: "flex items-center justify-end gap-0.5 text-xs text-muted-foreground", Star { class: "size-3" } "{sterne}/{knoten.len() * 3}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Lets the browser download a text file.
fn herunterladen(name: &str, inhalt: &str) {
    let js = format!(
        "(function () {{ var b = new Blob([{inhalt}], {{ type: 'application/json' }}); var a = document.createElement('a'); \
         a.href = URL.createObjectURL(b); a.download = {name}; document.body.appendChild(a); a.click(); \
         setTimeout(function () {{ URL.revokeObjectURL(a.href); a.remove(); }}, 1000); }})();",
        inhalt = serde_json::to_string(inhalt).unwrap_or_default(),
        name = serde_json::to_string(name).unwrap_or_default(),
    );
    document::eval(&js);
}

#[derive(Clone, PartialEq)]
enum Meldung {
    Exportiert,
    Importiert,
    Geloescht,
    KeineSicherung,
}

#[component]
fn Sicherung() -> Element {
    let l = use_sprache();
    let fortschritt = use_fortschritt();
    let mut kandidat = use_signal(|| None::<Fortschritt>);
    let mut meldung = use_signal(|| None::<Meldung>);
    let mut loeschen = use_signal(|| false);
    let f = fortschritt.read();

    let datei_gewaehlt = move |e: FormEvent| {
        let dateien = e.files();
        spawn(async move {
            // Allow picking the same file again later.
            document::eval("var i = document.getElementById('nrzlearn-import'); if (i) i.value = '';");
            let Some(datei) = dateien.first() else { return };
            let gelesen = if datei.size() <= 2_000_000 { datei.read_string().await.ok() } else { None };
            match gelesen.as_deref().and_then(import_json) {
                Some(neu) => {
                    meldung.set(None);
                    kandidat.set(Some(neu));
                }
                None => {
                    kandidat.set(None);
                    meldung.set(Some(Meldung::KeineSicherung));
                }
            }
        });
    };

    rsx! {
        section { class: "rounded-2xl border-2 p-6",
            div { class: "flex items-center gap-2",
                HardDrive { class: "size-5" }
                h2 { class: "text-lg font-bold", {t!(l, BACKUP)} }
            }
            p { class: "mt-2 text-sm text-muted-foreground", {t!(l, BACKUP_TEXT)} }
            div { class: "mt-4 flex flex-wrap items-center gap-3",
                button {
                    class: KNOPF_VOLL,
                    onclick: move |_| {
                        herunterladen(&export_dateiname(), &export_json(&fortschritt.peek()));
                        meldung.set(Some(Meldung::Exportiert));
                    },
                    Download { class: "size-4" }
                    {t!(l, EXPORT)}
                }
                label { r#for: "nrzlearn-import", class: KNOPF,
                    Upload { class: "size-4" }
                    {t!(l, IMPORT)}
                }
                input { id: "nrzlearn-import", r#type: "file", accept: ".json,application/json", class: "sr-only", onchange: datei_gewaehlt }
            }

            if let Some(neu) = kandidat() {
                div { class: "anim-pop mt-4 rounded-xl border-2 border-foreground p-4",
                    p { class: "font-semibold", {t!(l, REPLACE_QUESTION)} }
                    p { class: "mt-1 text-sm text-muted-foreground",
                        {tf!(l, REPLACE_COMPARE, xp = f.xp, levels = f.knoten.len(), backup_xp = neu.xp, backup_levels = neu.knoten.len())}
                    }
                    div { class: "mt-3 flex gap-2",
                        button {
                            class: KNOPF_VOLL,
                            onclick: move |_| {
                                if let Some(neu) = kandidat.take() {
                                    ersetzen(fortschritt, neu);
                                    meldung.set(Some(Meldung::Importiert));
                                }
                            },
                            {t!(l, REPLACE)}
                        }
                        button { class: KNOPF, onclick: move |_| kandidat.set(None), {t!(l, CANCEL)} }
                    }
                }
            }

            match meldung() {
                Some(m) => rsx! {
                    p { class: if m == Meldung::KeineSicherung { "anim-shake mt-4 text-sm font-semibold" } else { "anim-pop mt-4 flex items-center gap-1.5 text-sm font-semibold" },
                        if m != Meldung::KeineSicherung { Check { class: "size-4" } }
                        {match m {
                            Meldung::Exportiert => t!(l, EXPORTED),
                            Meldung::Importiert => t!(l, IMPORTED),
                            Meldung::Geloescht => t!(l, DELETED),
                            Meldung::KeineSicherung => t!(l, NOT_A_BACKUP),
                        }}
                    }
                },
                None => rsx! {},
            }

            div { class: "mt-6 border-t pt-4",
                if loeschen() {
                    span { class: "me-2 text-sm font-semibold", {t!(l, DELETE_CONFIRM)} }
                    Button {
                        variant: ButtonVariant::Destructive,
                        size: ButtonSize::Sm,
                        onclick: move |_| {
                            zuruecksetzen(fortschritt);
                            loeschen.set(false);
                            kandidat.set(None);
                            meldung.set(Some(Meldung::Geloescht));
                        },
                        {t!(l, DELETE)}
                    }
                    Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, onclick: move |_| loeschen.set(false), {t!(l, CANCEL)} }
                } else {
                    Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, class: "px-0 text-muted-foreground", onclick: move |_| loeschen.set(true),
                        RotateCcw {}
                        {t!(l, DELETE_PROGRESS)}
                    }
                }
            }
        }
    }
}
