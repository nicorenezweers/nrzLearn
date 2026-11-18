//! nrzLearn – learn programming step by step: Rust, Python, TypeScript, Go,
//! JavaScript, C++, C#, Kotlin, SQL and Swift, in 40 languages.
//! Built with Dioxus and components from rust-ui (https://github.com/rust-ui/ui).
//!
//! Everything runs in the browser: no accounts, no server-side data. The
//! progress lives in localStorage. The static landing pages for search
//! engines are built by `nrzlearn-site` (see `site/`); this app runs under
//! `/<lang>/<course>/…`.

use dioxus::prelude::*;

#[macro_use]
mod i18n;
mod aufgaben;
mod components;
mod content;
mod highlight;
mod inhalt;
mod kurs;
mod pages;
mod playground;
mod progress;
mod rng;
mod shell;
mod uebung;
mod visuals;
mod welt;

use components::layout::Layout;
use i18n::Sprache;
use kurs::Kurs;
use pages::home::Home;
use pages::lesson::LektionSeite;
use pages::profil::Profil;
use pages::spieler::Spieler;
use pages::spickzettel::Spickzettel;
use pages::start::{KursStart, Start, StartSprache, Unbekannt};
use pages::terminal::TerminalSeite;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const FAVICON: Asset = asset!("/assets/logo.svg");

/// Small helper script: applies dark mode and the text direction before the
/// first render, Tab in the code editor inserts four spaces, keyboard
/// shortcuts in the lesson player.
const BOOT_JS: &str = r#"
(function () {
  try {
    var seg = location.pathname.split('/')[1] || '';
    if (/^[a-z]{2}$/.test(seg)) {
      document.documentElement.lang = seg === 'zh' ? 'zh-Hans' : seg;
      document.documentElement.dir = seg === 'ar' ? 'rtl' : 'ltr';
    }
    var d = localStorage.getItem('darkmode');
    if (d === 'true' || (d === null && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
      document.documentElement.classList.add('dark');
    }
  } catch (e) {}
  if (window.__nrzLearnBoot) return;
  window.__nrzLearnBoot = true;
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

pub use routen::Route;

/// The routes live in their own module: the catch-all route makes the code
/// derived by `Routable` end early, which would warn about unreachable code.
#[allow(unreachable_code)]
mod routen {
    use super::*;

    #[derive(Debug, Clone, Routable, PartialEq)]
    #[rustfmt::skip]
    pub enum Route {
        // Only reached without the static landing pages (development).
        #[route("/")]
        Start {},
        #[route("/:lang")]
        StartSprache { lang: Sprache },
        #[route("/:lang/:kurs")]
        KursStart { lang: Sprache, kurs: Kurs },
        #[layout(Layout)]
            #[route("/:lang/:kurs/learn")]
            Home { lang: Sprache, kurs: Kurs },
            #[route("/:lang/:kurs/world/:id")]
            LektionSeite { lang: Sprache, kurs: Kurs, id: usize },
            #[route("/:lang/:kurs/terminal")]
            TerminalSeite { lang: Sprache, kurs: Kurs },
            #[route("/:lang/:kurs/cheatsheet")]
            Spickzettel { lang: Sprache, kurs: Kurs },
            #[route("/:lang/:kurs/profile")]
            Profil { lang: Sprache, kurs: Kurs },
        #[end_layout]
        #[route("/:lang/:kurs/play/:id")]
        Spieler { lang: Sprache, kurs: Kurs, id: String },
        // Old links (/world/3, /learn/…, /profile, /leaderboard …).
        #[route("/:..rest")]
        Unbekannt { rest: Vec<String> },
    }
}

impl Route {
    /// Language and course of an app page.
    pub const fn lang_kurs(&self) -> Option<(Sprache, Kurs)> {
        match self {
            Self::Home { lang, kurs }
            | Self::LektionSeite { lang, kurs, .. }
            | Self::TerminalSeite { lang, kurs }
            | Self::Spickzettel { lang, kurs }
            | Self::Profil { lang, kurs }
            | Self::Spieler { lang, kurs, .. }
            | Self::KursStart { lang, kurs } => Some((*lang, *kurs)),
            _ => None,
        }
    }

    /// The same page in another language.
    pub fn mit_sprache(&self, l: Sprache) -> Self {
        let mut r = self.clone();
        match &mut r {
            Self::Home { lang, .. }
            | Self::LektionSeite { lang, .. }
            | Self::TerminalSeite { lang, .. }
            | Self::Spickzettel { lang, .. }
            | Self::Profil { lang, .. }
            | Self::Spieler { lang, .. }
            | Self::KursStart { lang, .. }
            | Self::StartSprache { lang } => *lang = l,
            Self::Start {} | Self::Unbekannt { .. } => {}
        }
        r
    }
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(Sprache::aus_browser()));
    progress::provide_fortschritt();

    rsx! {
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }
        document::Meta { name: "robots", content: "noindex" }
        document::Link { rel: "icon", r#type: "image/svg+xml", href: FAVICON }
        document::Stylesheet { href: TAILWIND_CSS }
        document::Script { {BOOT_JS} }
        Router::<Route> {}
    }
}

/// Leaves the app for a static page (landing page, legal notice …).
pub fn seite_laden(pfad: &str) {
    if let Some(w) = web_sys::window() {
        let _ = w.location().assign(pfad);
    }
}
