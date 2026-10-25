//! Kleine Belohnungen: Töne (Web Audio) und Konfetti in Grautönen.

use dioxus::prelude::*;

use crate::rng::Rng;

#[derive(Clone, Copy)]
pub enum Ton {
    Richtig,
    Falsch,
    Serie,
    Geschafft,
}

/// Spielt einen kurzen, synthetischen Ton – ohne Audiodateien.
pub fn ton(art: Ton, an: bool) {
    if !an {
        return;
    }
    let (noten, welle) = match art {
        Ton::Richtig => ("[[880,0],[1318.5,0.09]]", "sine"),
        Ton::Falsch => ("[[196,0],[155,0.13]]", "triangle"),
        Ton::Serie => ("[[988,0],[1175,0.07],[1568,0.14],[1976,0.21]]", "sine"),
        Ton::Geschafft => ("[[523.3,0],[659.3,0.12],[784,0.24],[1046.5,0.36],[1318.5,0.5]]", "sine"),
    };
    let laut = 0.16;
    document::eval(&format!(
        "(function(){{const C=window.AudioContext||window.webkitAudioContext;if(!C)return;\
         window.__nrzAc=window.__nrzAc||new C();const ac=window.__nrzAc;if(ac.state==='suspended')ac.resume();\
         {noten}.forEach(function(n){{const o=ac.createOscillator(),g=ac.createGain(),t=ac.currentTime+n[1];\
         o.type='{welle}';o.frequency.value=n[0];g.gain.setValueAtTime(0.0001,t);\
         g.gain.exponentialRampToValueAtTime({laut},t+0.012);g.gain.exponentialRampToValueAtTime(0.0001,t+0.28);\
         o.connect(g);g.connect(ac.destination);o.start(t);o.stop(t+0.3);}});}})()"
    ));
}

/// Konfetti-Regen aus grauen Schnipseln.
#[component]
pub fn Konfetti(#[props(default = 70)] anzahl: usize) -> Element {
    let teile = use_hook(|| {
        let mut r = Rng::neu();
        (0..anzahl)
            .map(|_| {
                let grau = ["bg-foreground", "bg-muted-foreground", "bg-foreground/60", "bg-muted-foreground/50", "border-2 border-foreground"];
                let form = if r.bool() { "rounded-sm" } else { "rounded-full" };
                (
                    format!("{} {form}", r.wahl(&grau)),
                    format!(
                        "left:{}%;--dx:{}px;--rot:{}deg;--dur:{}s;--delay:{}s;width:{}px;height:{}px",
                        r.zahl(0, 100),
                        r.zahl(-160, 160),
                        r.zahl(-720, 720),
                        2.0 + r.zahl(0, 15) as f64 / 10.0,
                        r.zahl(0, 60) as f64 / 100.0,
                        r.zahl(6, 12),
                        r.zahl(8, 16)
                    ),
                )
            })
            .collect::<Vec<_>>()
    });
    rsx! {
        div { class: "pointer-events-none fixed inset-0 z-[60] overflow-hidden", "aria-hidden": "true",
            for (i, (class, style)) in teile.iter().enumerate() {
                div { key: "{i}", class: "confetti {class}", style: "{style}" }
            }
        }
    }
}
