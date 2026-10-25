//! nrzLearn – learn Rust step by step, in English, German and Spanish.
//! Built with Dioxus and components from rust-ui (https://github.com/rust-ui/ui).

use dioxus::prelude::*;

#[macro_use]
mod i18n;
mod api;
mod components;
mod content;
mod aufgaben;
mod highlight;
mod konto;
mod pages;
mod playground;
mod progress;
mod rng;
mod shell;
mod uebung;
mod visuals;
mod welt;

use components::layout::Layout;
use pages::home::Home;
use pages::admin::Admin;
use pages::lesson::LektionSeite;
use pages::rangliste::Rangliste;
use pages::spieler::Spieler;
use pages::spickzettel::Spickzettel;
use pages::terminal::TerminalSeite;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

/// Small helper script: applies dark mode before the first render, Tab in the
/// code editor inserts four spaces, keyboard shortcuts in the lesson player.
const BOOT_JS: &str = r#"
(function () {
  try {
    var d = localStorage.getItem('darkmode');
    if (d === 'true' || (d === null && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
      document.documentElement.classList.add('dark');
    }
  } catch (e) {}
  if (window.__rustLernenBoot) return;
  window.__rustLernenBoot = true;
  document.addEventListener('keydown', function (e) {
    var t = e.target;
    if (e.key === 'Tab' && !e.shiftKey && t && t.classList && t.classList.contains('code-layer')) {
      e.preventDefault();
      t.setRangeText('    ', t.selectionStart, t.selectionEnd, 'end');
      t.dispatchEvent(new Event('input', { bubbles: true }));
      return;
    }
    // Lesson player: Enter presses the main button, 1-9 picks answers.
    var tag = (t && t.tagName) || '';
    if (tag === 'TEXTAREA' || tag === 'INPUT') return;
    if (e.key === 'Enter') {
      var b = document.querySelector('[data-primaer]:not([disabled])');
      if (b) { e.preventDefault(); b.click(); }
    } else if (/^[1-9]$/.test(e.key)) {
      var o = document.querySelector('[data-taste="' + e.key + '"]:not([disabled])');
      if (o) { e.preventDefault(); o.click(); }
    }
  });
})();
"#;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
        #[route("/")]
        Home {},
        #[route("/world/:id")]
        LektionSeite { id: usize },
        #[route("/terminal")]
        TerminalSeite {},
        #[route("/cheatsheet")]
        Spickzettel {},
        #[route("/leaderboard")]
        Rangliste {},
        #[route("/admin")]
        Admin {},
    #[end_layout]
    #[route("/learn/:id")]
    Spieler { id: String },
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(i18n::Sprache::aus_browser().unwrap_or_default()));
    use_context_provider(|| Signal::new(konto::Konto::Laeuft));
    let k = konto::use_konto();
    let name = k.read().status().map_or_else(|| "nrzLearn".to_string(), |s| s.instance_name.clone());

    rsx! {
        document::Title { "{name}" }
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }
        document::Stylesheet { href: TAILWIND_CSS }
        document::Script { {BOOT_JS} }
        konto::Huelle {}
    }
}
