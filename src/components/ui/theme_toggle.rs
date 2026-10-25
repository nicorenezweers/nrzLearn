use dioxus::document::eval;
use dioxus::prelude::*;
use icons::{Moon, Sun};

#[component]
pub fn ThemeToggle() -> Element {
    // Synchron beim Start lesen (das Original las asynchron, wodurch der zweite
    // Effect den gespeicherten Wert vorher mit `false` überschrieb).
    let mut dark = use_signal(|| {
        let window = web_sys::window();
        let gespeichert = window
            .as_ref()
            .and_then(|w| w.local_storage().ok().flatten())
            .and_then(|s| s.get_item("darkmode").ok().flatten());
        match gespeichert.as_deref() {
            Some(v) => v == "true",
            None => window
                .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok().flatten())
                .is_some_and(|m| m.matches()),
        }
    });

    // Whenever dark changes: toggle .dark on <html> and persist to localStorage.
    use_effect(move || {
        let is_dark = dark();
        eval(&format!(
            "document.documentElement.classList.toggle('dark', {is_dark}); \
             localStorage.setItem('darkmode', '{is_dark}');"
        ));
    });

    let toggle = move |_| {
        dark.set(!dark());
    };

    rsx! {
        button {
            r#type: "button",
            "aria-label": "Toggle theme",
            class: "inline-flex items-center justify-center size-8 rounded-md hover:bg-accent transition-colors cursor-pointer",
            onclick: toggle,
            if dark() {
                Sun { class: "size-4" }
            } else {
                Moon { class: "size-4" }
            }
        }
    }
}
