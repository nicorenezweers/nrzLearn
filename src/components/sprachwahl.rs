use dioxus::prelude::*;
use icons::{Check, Globe};

use crate::api;
use crate::i18n::{Sprache, use_sprache_signal};
use crate::konto::use_konto;

/// Language menu. Saves the choice in the browser and, when logged in, on the server.
#[component]
pub fn Sprachwahl() -> Element {
    let mut sprache = use_sprache_signal();
    let konto = use_konto();
    let mut offen = use_signal(|| false);
    let l = sprache();

    rsx! {
        div { class: "relative",
            button {
                class: "inline-flex h-8 items-center gap-1 rounded-md px-2 text-sm font-semibold transition-colors hover:bg-accent",
                title: "Language",
                onclick: move |_| offen.set(!offen()),
                Globe { class: "size-4" }
                span { class: "uppercase", "{l.code()}" }
            }
            if offen() {
                div { class: "fixed inset-0 z-40", onclick: move |_| offen.set(false) }
                div { class: "anim-pop absolute right-0 top-10 z-50 w-40 overflow-hidden rounded-xl border-2 bg-card shadow-xl",
                    for sp in Sprache::ALLE {
                        button {
                            key: "{sp.code()}",
                            class: "flex w-full items-center justify-between px-3 py-2 text-left text-sm hover:bg-accent",
                            onclick: move |_| {
                                sprache.set(sp);
                                sp.merken();
                                offen.set(false);
                                if konto.peek().me().is_some() {
                                    spawn(async move {
                                        let _ = api::sprache_speichern(sp.code()).await;
                                    });
                                }
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
