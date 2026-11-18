//! Frame of every course page: takes language and course from the URL,
//! remembers them, loads the course content and provides it to the page.

use dioxus::prelude::*;

use crate::components::logo::Logo;
use crate::i18n::Sprache;
use crate::inhalt::{self, KursInhalt};
use crate::kurs::Kurs;

#[component]
pub fn KursRahmen(lang: Sprache, kurs: Kurs, children: Element) -> Element {
    // A new frame for every language and course: all pages below read the right
    // ones. Keys only take effect in lists, hence the one-entry list.
    rsx! {
        for schluessel in std::iter::once(format!("{lang}-{kurs}")) {
            Rahmen { key: "{schluessel}", lang, kurs, {children.clone()} }
        }
    }
}

#[component]
fn Rahmen(lang: Sprache, kurs: Kurs, children: Element) -> Element {
    use_context_provider(|| Signal::new(lang));
    use_context_provider(|| kurs);
    use_hook(move || {
        lang.merken();
        kurs.merken();
    });
    let mut daten = use_resource(move || async move { inhalt::laden(kurs, lang).await });
    let l = lang;

    match &*daten.read() {
        None => rsx! {
            div { class: "flex min-h-[60vh] items-center justify-center",
                Logo { class: "anim-bounce size-14" }
                span { class: "sr-only", {t!(l, LOADING)} }
            }
        },
        Some(Err(_)) => rsx! {
            div { class: "flex min-h-[60vh] flex-col items-center justify-center gap-4 p-6 text-center",
                Logo { class: "size-14" }
                p { class: "text-lg font-semibold", {t!(l, LOAD_ERROR)} }
                button { class: "rounded-xl border-2 border-b-4 px-5 py-2 font-semibold", onclick: move |_| daten.restart(), {t!(l, TRY_AGAIN)} }
            }
        },
        Some(Ok(i)) => rsx! {
            InhaltGeben { inhalt: i.clone(), {children} }
        },
    }
}

#[component]
fn InhaltGeben(inhalt: KursInhalt, children: Element) -> Element {
    use_context_provider(|| inhalt.clone());
    rsx! { {children} }
}

/// The course of the current page.
pub fn use_kurs() -> Kurs {
    use_context::<Kurs>()
}
