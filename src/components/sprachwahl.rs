use dioxus::prelude::*;
use icons::{Check, Globe};

use crate::Route;
use crate::i18n::{Sprache, use_sprache};

/// Language menu: opens the same page in another language. The choice is
/// remembered in this browser only.
#[component]
pub fn Sprachwahl() -> Element {
    let l = use_sprache();
    let route: Route = use_route();
    let mut offen = use_signal(|| false);

    rsx! {
        div { class: "relative",
            button {
                class: "inline-flex h-8 items-center gap-1 rounded-md px-2 text-sm font-semibold transition-colors hover:bg-accent",
                title: t!(l, LANGUAGE),
                "aria-haspopup": "true",
                "aria-expanded": "{offen()}",
                onclick: move |_| offen.set(!offen()),
                Globe { class: "size-4" }
                span { class: "uppercase", "{l.code()}" }
            }
            if offen() {
                div { class: "fixed inset-0 z-40", onclick: move |_| offen.set(false) }
                div { class: "anim-pop absolute end-0 top-10 z-50 max-h-[70vh] w-56 overflow-y-auto rounded-2xl border-2 bg-card p-1.5 shadow-xl",
                    for sp in Sprache::ALLE {
                        Link {
                            key: "{sp.code()}",
                            to: route.mit_sprache(sp),
                            class: if sp == l { "flex w-full items-center justify-between rounded-lg bg-accent px-3 py-1.5 text-sm" } else { "flex w-full items-center justify-between rounded-lg px-3 py-1.5 text-sm hover:bg-accent" },
                            lang: sp.tag(),
                            onclick: move |_| {
                                sp.merken();
                                offen.set(false);
                            },
                            "{sp.name()}"
                            if sp == l { Check { class: "size-4" } }
                        }
                    }
                }
            }
        }
    }
}
