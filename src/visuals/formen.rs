use dioxus::prelude::*;

use super::VisualRahmen;
use crate::components::code::CodeBlock;
use crate::i18n::use_sprache;

const CODE: [&str; 3] = [
    "struct Circle { r: f64 }\n\nimpl Shape for Circle {\n    fn area(&self) -> f64 {\n        std::f64::consts::PI * self.r * self.r\n    }\n    fn perimeter(&self) -> f64 {\n        2.0 * std::f64::consts::PI * self.r\n    }\n}",
    "struct Rectangle { a: f64, b: f64 }\n\nimpl Shape for Rectangle {\n    fn area(&self) -> f64 {\n        self.a * self.b\n    }\n    fn perimeter(&self) -> f64 {\n        2.0 * (self.a + self.b)\n    }\n}",
    "struct Square { a: f64 }\n\nimpl Shape for Square {\n    fn area(&self) -> f64 {\n        self.a * self.a\n    }\n    fn perimeter(&self) -> f64 {\n        4.0 * self.a\n    }\n}",
];

#[component]
pub fn Formen() -> Element {
    let l = use_sprache();
    let mut form = use_signal(|| 0_usize);
    let mut a = use_signal(|| 4.0_f64);
    let mut b = use_signal(|| 3.0_f64);
    let f = form();
    let (av, bv) = (a(), b());

    let (name, flaeche, umfang, formel) = match f {
        0 => ("Circle", std::f64::consts::PI * av * av, 2.0 * std::f64::consts::PI * av, format!("π × r² = 3.14159 × {av} × {av}")),
        1 => ("Rectangle", av * bv, 2.0 * (av + bv), format!("a × b = {av} × {bv}")),
        _ => ("Square", av * av, 4.0 * av, format!("a × a = {av} × {av}")),
    };

    // SVG scale
    let skala = 18.0;
    let svg = match f {
        0 => rsx! { circle { cx: "110", cy: "100", r: "{av * skala}", class: "fill-rust/25 stroke-rust", stroke_width: "3" } },
        1 => rsx! { rect { x: "{110.0 - av * skala / 2.0}", y: "{100.0 - bv * skala / 2.0}", width: "{av * skala}", height: "{bv * skala}", class: "fill-rust/25 stroke-rust", stroke_width: "3" } },
        _ => rsx! { rect { x: "{110.0 - av * skala / 2.0}", y: "{100.0 - av * skala / 2.0}", width: "{av * skala}", height: "{av * skala}", class: "fill-rust/25 stroke-rust", stroke_width: "3" } },
    };
    let knoepfe = tr!(l, ["⚪ Circle", "▭ Rectangle", "◻ Square"], ["⚪ Kreis", "▭ Rechteck", "◻ Quadrat"], ["⚪ Círculo", "▭ Rectángulo", "◻ Cuadrado"]);

    rsx! {
        VisualRahmen { titel: tr!(l, "One trait, many types: trait Shape", "Ein Trait, viele Typen: trait Shape", "Un trait, muchos tipos: trait Shape"),
            div { class: "mb-4 flex flex-wrap gap-2",
                for (i, n) in knoepfe.iter().enumerate() {
                    button {
                        key: "{i}",
                        class: if f == i { "rounded-md border-2 border-rust bg-rust-light px-3 py-1.5 text-sm font-medium" } else { "rounded-md border-2 px-3 py-1.5 text-sm hover:bg-accent" },
                        onclick: move |_| form.set(i),
                        "{n}"
                    }
                }
            }
            div { class: "grid gap-4 lg:grid-cols-2",
                div {
                    svg { view_box: "0 0 220 200", class: "h-48 w-full rounded-lg bg-muted/50", {svg} }
                    div { class: "mt-3 space-y-2 text-sm",
                        label { class: "flex items-center gap-2",
                            span { class: "w-24", if f == 0 { {tr!(l, "Radius r", "Radius r", "Radio r")} } else { {tr!(l, "Side a", "Seite a", "Lado a")} } }
                            input { r#type: "range", min: "1", max: "5", step: "0.5", value: "{av}", class: "flex-1 accent-[var(--rust)]", oninput: move |e| a.set(e.value().parse().unwrap_or(1.0)) }
                            span { class: "w-10 font-mono", "{av}" }
                        }
                        if f == 1 {
                            label { class: "flex items-center gap-2",
                                span { class: "w-24", {tr!(l, "Side b", "Seite b", "Lado b")} }
                                input { r#type: "range", min: "1", max: "5", step: "0.5", value: "{bv}", class: "flex-1 accent-[var(--rust)]", oninput: move |e| b.set(e.value().parse().unwrap_or(1.0)) }
                                span { class: "w-10 font-mono", "{bv}" }
                            }
                        }
                    }
                    div { class: "mt-3 rounded-lg bg-muted/70 p-3 font-mono text-sm",
                        div { "{name}.area() = {formel} = ", span { class: "font-bold text-rust", "{flaeche:.2}" } }
                        div { "{name}.perimeter() = ", span { class: "font-bold text-rust", "{umfang:.2}" } }
                    }
                }
                CodeBlock { code: CODE[f].to_string() }
            }
        }
    }
}
