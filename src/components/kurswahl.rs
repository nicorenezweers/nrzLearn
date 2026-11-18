use dioxus::prelude::*;
use icons::{Check, ChevronDown, LayoutGrid};

use crate::Route;
use crate::i18n::use_sprache;
use crate::kurs::Kurs;
use crate::progress::use_fortschritt;

/// The coloured tile of a course with the logo of the language.
#[component]
pub fn KursKachel(kurs: Kurs, #[props(into, default)] class: String) -> Element {
    let info = kurs.info();
    match nrzlearn_model::icons::icon(info.id) {
        Some(logo) => rsx! {
            span { class: "inline-flex shrink-0 select-none items-center justify-center rounded-xl {class}", style: kurs.kachel_stil(), "aria-hidden": "true",
                svg { class: "size-[62%]", view_box: logo.view_box, fill: "currentColor", dangerous_inner_html: logo.body }
            }
        },
        None => {
            let klein = if info.mono.len() > 2 { "tracking-tighter" } else { "" };
            rsx! {
                span { class: "inline-flex shrink-0 select-none items-center justify-center rounded-xl font-mono font-extrabold leading-none {klein} {class}", style: kurs.kachel_stil(), "aria-hidden": "true", "{info.mono}" }
            }
        }
    }
}

/// Course switcher in the header.
#[component]
pub fn KursMenue(kurs: Kurs) -> Element {
    let l = use_sprache();
    let fortschritt = use_fortschritt();
    let mut offen = use_signal(|| false);
    rsx! {
        div { class: "relative",
            button {
                class: "flex h-9 items-center gap-2 rounded-xl px-1.5 transition hover:bg-accent sm:pe-2",
                title: t!(l, COURSE_MENU),
                "aria-haspopup": "true",
                "aria-expanded": "{offen()}",
                onclick: move |_| offen.set(!offen()),
                KursKachel { kurs, class: "size-7 rounded-lg text-[0.7rem]" }
                span { class: "hidden text-sm font-bold sm:inline", "{kurs.titel(l)}" }
                ChevronDown { class: "hidden size-3.5 text-muted-foreground sm:block" }
            }
            if offen() {
                div { class: "fixed inset-0 z-40", onclick: move |_| offen.set(false) }
                div { class: "anim-pop absolute start-0 top-11 z-50 max-h-[75vh] w-[min(36rem,calc(100vw-2rem))] overflow-y-auto rounded-2xl border-2 bg-card p-1.5 shadow-xl",
                    for g in nrzlearn_model::GROUPS {
                        div { key: "{g}",
                            div { class: "px-2.5 pb-1 pt-2 text-xs font-bold uppercase tracking-wider text-muted-foreground", {crate::kurs::gruppe(l, g)} }
                            div { class: "grid gap-0.5 sm:grid-cols-2",
                                for k in Kurs::ALLE.into_iter().filter(|k| k.info().group == g) {
                                    Link {
                                        key: "{k.id()}",
                                        to: Route::Home { lang: l, kurs: k },
                                        class: if k == kurs { "flex items-center gap-3 rounded-xl bg-accent px-2.5 py-1.5" } else { "flex items-center gap-3 rounded-xl px-2.5 py-1.5 hover:bg-accent" },
                                        onclick: move |_| offen.set(false),
                                        KursKachel { kurs: k, class: "size-8 rounded-lg" }
                                        span { class: "flex-1 font-semibold", "{k.titel(l)}" }
                                        {
                                            let n = fortschritt.read().fertig_im_kurs(k);
                                            rsx! {
                                                if k == kurs { Check { class: "size-4" } } else if n > 0 { span { class: "text-xs text-muted-foreground", "{n}" } }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    a { class: "mt-1 flex items-center gap-2 rounded-xl border-t px-2.5 py-2 text-sm text-muted-foreground hover:bg-accent hover:text-foreground", href: "/{l}/",
                        LayoutGrid { class: "size-4" }
                        {t!(l, ALL_COURSES)}
                    }
                }
            }
        }
    }
}
