use dioxus::prelude::*;
use icons::{Lock, StickyNote};

use crate::components::code::CodeBlock;
use crate::content::lektionen;
use crate::content::spick::KARTEN;
use crate::i18n::use_sprache;
use crate::progress::use_fortschritt;

#[component]
pub fn Spickzettel() -> Element {
    let fortschritt = use_fortschritt();
    let l = use_sprache();
    let f = fortschritt.read();
    rsx! {
        div { class: "mx-auto max-w-6xl px-4 py-8",
            div { class: "flex items-center gap-3",
                div { class: "flex size-12 items-center justify-center rounded-2xl bg-foreground text-background", StickyNote { class: "size-6" } }
                div {
                    h1 { class: "text-3xl font-extrabold tracking-tight", {tr!(l, "Cheat sheet", "Spickzettel", "Chuleta")} }
                    p { class: "text-muted-foreground", {tr!(l, format!("{} of {} cards collected. Every world unlocks its card through its “Learn” level.", f.karten(), KARTEN.len()), format!("{} von {} Karten gesammelt. Jede Welt schaltet ihre Karte über das „Lernen“-Level frei.", f.karten(), KARTEN.len()), format!("{} de {} tarjetas conseguidas. Cada mundo desbloquea su tarjeta con su nivel «Aprender».", f.karten(), KARTEN.len()))} }
                }
            }
            p { class: "mt-4 rounded-xl border-2 border-dashed p-3 text-sm text-muted-foreground",
                {tr!(l, "You can take the cheat sheet everywhere – bottom right or at the top of every lesson. It's allowed in world exams, but not in master exams.", "Den Spickzettel kannst du überall mitnehmen – unten rechts oder oben in jeder Lektion. In Welt-Prüfungen ist er erlaubt, in Meisterprüfungen nicht.", "Puedes llevar la chuleta a todas partes: abajo a la derecha o arriba en cada lección. Está permitida en los exámenes de mundo, pero no en los exámenes maestros.")}
            }
            div { class: "mt-6 columns-1 gap-4 md:columns-2 xl:columns-3",
                for k in KARTEN.iter() {
                    div { key: "{k.welt}", class: "mb-4 break-inside-avoid",
                        div { class: "mb-2 flex items-center gap-2 text-sm font-semibold",
                            span { class: "flex size-6 items-center justify-center rounded-md bg-foreground text-xs text-background", "{k.welt + 1}" }
                            "{k.titel(l)}"
                        }
                        if f.karte_offen(k.welt) {
                            CodeBlock { code: k.code(l).to_string() }
                        } else {
                            div { class: "relative overflow-hidden rounded-xl border-2 border-dashed",
                                pre { class: "select-none p-4 font-mono text-[0.8rem] leading-6 opacity-30 blur-[3px]", "{k.code(l)}" }
                                div { class: "absolute inset-0 flex flex-col items-center justify-center gap-1 text-sm font-semibold",
                                    Lock { class: "size-5" }
                                    {tr!(l, format!("Unlock world {}", k.welt + 1), format!("Schalte Welt {} frei", k.welt + 1), format!("Desbloquea el mundo {}", k.welt + 1))}
                                    span { class: "text-xs font-normal text-muted-foreground", {tr!(l, format!("“{}” → Learn", lektionen(l)[k.welt].titel), format!("„{}“ → Lernen", lektionen(l)[k.welt].titel), format!("«{}» → Aprender", lektionen(l)[k.welt].titel))} }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
