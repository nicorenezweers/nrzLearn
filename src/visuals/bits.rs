use dioxus::prelude::*;

use super::VisualRahmen;
use crate::components::ui::{Button, ButtonSize, ButtonVariant};
use crate::i18n::use_sprache;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Typ {
    name: &'static str,
    bits: u32,
    signed: bool,
}

const TYPEN: &[Typ] = &[
    Typ { name: "u8", bits: 8, signed: false },
    Typ { name: "i8", bits: 8, signed: true },
    Typ { name: "u16", bits: 16, signed: false },
    Typ { name: "i16", bits: 16, signed: true },
];

impl Typ {
    const fn maske(self) -> u64 {
        (1_u64 << self.bits) - 1
    }
    fn wert(self, roh: u64) -> i64 {
        let roh = roh & self.maske();
        if self.signed && roh >> (self.bits - 1) & 1 == 1 {
            roh as i64 - (1_i64 << self.bits)
        } else {
            roh as i64
        }
    }
    fn min(self) -> i64 {
        if self.signed { -(1_i64 << (self.bits - 1)) } else { 0 }
    }
    fn max(self) -> i64 {
        if self.signed { (1_i64 << (self.bits - 1)) - 1 } else { (1_i64 << self.bits) - 1 }
    }
    fn gewicht(self, bit: u32) -> i64 {
        if self.signed && bit == self.bits - 1 { -(1_i64 << bit) } else { 1_i64 << bit }
    }
}

#[component]
pub fn Bits() -> Element {
    let l = use_sprache();
    let mut typ = use_signal(|| TYPEN[0]);
    let mut roh = use_signal(|| 42_u64);
    let mut meldung = use_signal(String::new);
    let t = typ();
    let r = roh() & t.maske();
    let wert = t.wert(r);

    let summanden: Vec<String> = (0..t.bits)
        .rev()
        .filter(|b| r >> b & 1 == 1)
        .map(|b| t.gewicht(b).to_string())
        .collect();
    let rechnung = if summanden.is_empty() { "0".to_string() } else { summanden.join(" + ").replace("+ -", "− ") };

    let mut setzen = move |neu: i128, op: &str| {
        let t = typ();
        let min = i128::from(t.min());
        let max = i128::from(t.max());
        if neu > max || neu < min {
            let n = t.name;
            meldung.set(tr!(
                l,
                format!("⚠️ Overflow at {op}! {neu} doesn't fit in {n} ({min} to {max}). In debug mode Rust would stop here (panic). With wrapping_* the value wraps around once."),
                format!("⚠️ Überlauf bei {op}! {neu} passt nicht in {n} ({min} bis {max}). Im Debug-Modus würde Rust hier abbrechen (panic). Mit wrapping_* läuft der Wert einmal „rund herum“."),
                format!("⚠️ ¡Desbordamiento en {op}! {neu} no cabe en {n} ({min} a {max}). En modo debug Rust se detendría aquí (panic). Con wrapping_* el valor da la vuelta.")
            ));
        } else {
            meldung.set(String::new());
        }
        roh.set((neu as u64) & t.maske());
    };

    rsx! {
        VisualRahmen { titel: tr!(l, "Bits & numbers – click the bits!", "Bits & Zahlen – klicke auf die Bits!", "Bits y números: ¡haz clic en los bits!"),
            div { class: "mb-4 flex flex-wrap items-center gap-2",
                span { class: "text-sm text-muted-foreground", {tr!(l, "Type:", "Typ:", "Tipo:")} }
                for tt in TYPEN.iter().copied() {
                    button {
                        key: "{tt.name}",
                        class: if tt == t {
                            "rounded-md border-2 border-rust bg-rust-light px-3 py-1 font-mono text-sm font-semibold"
                        } else {
                            "rounded-md border-2 px-3 py-1 font-mono text-sm hover:bg-accent"
                        },
                        onclick: move |_| {
                            let alt = typ().wert(roh());
                            typ.set(tt);
                            roh.set((alt.clamp(tt.min(), tt.max()) as u64) & tt.maske());
                            meldung.set(String::new());
                        },
                        "{tt.name}"
                    }
                }
            }
            div { class: "grid gap-1.5", style: "grid-template-columns: repeat(8, minmax(0, 1fr));",
                for b in (0..t.bits).rev() {
                    button {
                        key: "{t.name}-{b}",
                        class: "flex flex-col items-center gap-0.5",
                        onclick: move |_| {
                            roh.set(roh() ^ (1 << b));
                            meldung.set(String::new());
                        },
                        span {
                            class: if r >> b & 1 == 1 {
                                "flex h-10 w-full items-center justify-center rounded-md bg-rust font-mono text-lg font-bold text-background shadow transition"
                            } else {
                                "flex h-10 w-full items-center justify-center rounded-md border-2 border-dashed font-mono text-lg text-muted-foreground transition hover:border-rust/50"
                            },
                            if r >> b & 1 == 1 { "1" } else { "0" }
                        }
                        span {
                            class: if t.signed && b == t.bits - 1 { "font-mono text-[0.6rem] text-destructive" } else { "font-mono text-[0.6rem] text-muted-foreground" },
                            "{t.gewicht(b)}"
                        }
                    }
                }
            }
            div { class: "mt-4 grid gap-3 sm:grid-cols-3",
                div { class: "rounded-lg bg-muted/70 p-3",
                    div { class: "text-xs text-muted-foreground", {tr!(l, format!("Value as {}", t.name), format!("Wert als {}", t.name), format!("Valor como {}", t.name))} }
                    div { class: "font-mono text-2xl font-bold text-rust", "{wert}" }
                }
                div { class: "rounded-lg bg-muted/70 p-3 sm:col-span-2",
                    div { class: "text-xs text-muted-foreground", {tr!(l, "Calculation", "Rechnung", "Cálculo")} }
                    div { class: "break-all font-mono text-sm", "{rechnung} = {wert}" }
                    div { class: "mt-1 font-mono text-xs text-muted-foreground", {format!("{}: 0b{r:0>width$b} · Hex: 0x{r:0>hw$X}", tr!(l, "Binary", "Binär", "Binario"), width = t.bits as usize, hw = (t.bits / 4) as usize)} }
                }
            }
            div { class: "mt-3 text-sm text-muted-foreground",
                {tr!(l, format!("Range of {}: ", t.name), format!("Bereich von {}: ", t.name), format!("Rango de {}: ", t.name))}
                span { class: "font-mono text-foreground", "{t.min()}" }
                {tr!(l, " to ", " bis ", " a ")}
                span { class: "font-mono text-foreground", "{t.max()}" }
                if t.signed {
                    {tr!(l, format!("  (−2^{0} to 2^{0} − 1, the top bit counts negative)", t.bits - 1), format!("  (−2^{0} bis 2^{0} − 1, das oberste Bit zählt negativ)", t.bits - 1), format!("  (−2^{0} a 2^{0} − 1, el bit superior cuenta en negativo)", t.bits - 1))}
                } else {
                    {tr!(l, format!("  (0 to 2^{} − 1)", t.bits), format!("  (0 bis 2^{} − 1)", t.bits), format!("  (0 a 2^{} − 1)", t.bits))}
                }
            }
            div { class: "mt-3 flex flex-wrap gap-2",
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, onclick: move |_| setzen(i128::from(wert) + 1, "+1"), "+ 1" }
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, onclick: move |_| setzen(i128::from(wert) - 1, "−1"), "− 1" }
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, onclick: move |_| setzen(i128::from(wert) * 2, "×2"), "× 2  (<< 1)" }
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, onclick: move |_| { roh.set(!roh()); meldung.set(String::new()); }, {tr!(l, "! (invert)", "! (invertieren)", "! (invertir)")} }
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, onclick: move |_| setzen(i128::from(typ().max()), "MAX"), "MAX" }
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, onclick: move |_| setzen(i128::from(typ().min()), "MIN"), "MIN" }
                Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, onclick: move |_| setzen(0, "0"), "0" }
            }
            if !meldung().is_empty() {
                p { class: "anim-pop mt-3 rounded-lg border border-warning/40 bg-warning-light p-3 text-sm text-warning-dark", "{meldung}" }
            }
        }
    }
}
