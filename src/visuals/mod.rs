//! Interactive visualisations for the lessons.

mod bits;
mod formen;
mod iterator;
mod matcher;
mod memory;
mod parse;
mod pipeline;
mod scope;
mod sizes;
mod trace;
mod vec;

use dioxus::prelude::*;

use crate::content::{Block, Visual, lektionen};
use crate::i18n::use_sprache;

/// Frame for all visualisations.
#[component]
pub fn VisualRahmen(titel: String, children: Element) -> Element {
    let l = use_sprache();
    rsx! {
        div { class: "overflow-hidden rounded-xl border bg-card shadow-sm",
            div { class: "flex items-center gap-2 border-b bg-gradient-to-r from-rust/10 to-transparent px-4 py-2",
                span { class: "text-xs font-semibold uppercase tracking-wider text-rust", {tr!(l, "Interactive", "Interaktiv", "Interactivo")} }
                span { class: "text-sm font-medium", "{titel}" }
            }
            div { class: "p-4 sm:p-5", {children} }
        }
    }
}

#[component]
pub fn VisualView(lektion: usize, block: usize) -> Element {
    let l = use_sprache();
    let Block::Visual(v) = &lektionen(l)[lektion].bloecke[block] else {
        return rsx! {};
    };
    match v {
        Visual::Pipeline => rsx! { pipeline::Pipeline {} },
        Visual::Bits => rsx! { bits::Bits {} },
        Visual::Typgroessen => rsx! { sizes::Typgroessen {} },
        Visual::Speicher { .. } => rsx! { memory::Speicher { lektion, block } },
        Visual::Trace { .. } => rsx! { trace::Trace { lektion, block } },
        Visual::Vec => rsx! { vec::VecDemo {} },
        Visual::Match => rsx! { matcher::MatchDemo {} },
        Visual::Iterator => rsx! { iterator::IterDemo {} },
        Visual::Parse => rsx! { parse::ParseDemo {} },
        Visual::Formen => rsx! { formen::Formen {} },
        Visual::Scope => rsx! { scope::ScopeDemo {} },
    }
}
