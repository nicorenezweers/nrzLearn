use dioxus::prelude::*;

/// Rendert Text mit einfacher Auszeichnung: `code` und **fett**.
#[component]
pub fn RichText(text: String, #[props(into, optional)] class: Option<String>) -> Element {
    let teile = zerlegen(&text);
    rsx! {
        p { class: class.unwrap_or_else(|| "leading-7 text-foreground/90".into()),
            for (i, (art, inhalt)) in teile.into_iter().enumerate() {
                match art {
                    Teil::Code => rsx! {
                        code { key: "{i}", class: "rounded bg-muted px-1.5 py-0.5 font-mono text-[0.85em] text-rust", "{inhalt}" }
                    },
                    Teil::Fett => rsx! {
                        strong { key: "{i}", class: "font-semibold text-foreground", "{inhalt}" }
                    },
                    Teil::Normal => rsx! {
                        span { key: "{i}", "{inhalt}" }
                    },
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Teil {
    Normal,
    Code,
    Fett,
}

fn zerlegen(text: &str) -> Vec<(Teil, String)> {
    let mut out = Vec::new();
    let mut rest = text;
    while !rest.is_empty() {
        let code = rest.find('`');
        let fett = rest.find("**");
        match (code, fett) {
            (Some(c), f) if f.is_none_or(|f| c < f) => {
                if let Some(ende) = rest[c + 1..].find('`') {
                    out.push((Teil::Normal, rest[..c].to_string()));
                    out.push((Teil::Code, rest[c + 1..c + 1 + ende].to_string()));
                    rest = &rest[c + 2 + ende..];
                } else {
                    out.push((Teil::Normal, rest.to_string()));
                    break;
                }
            }
            (_, Some(f)) => {
                if let Some(ende) = rest[f + 2..].find("**") {
                    out.push((Teil::Normal, rest[..f].to_string()));
                    out.push((Teil::Fett, rest[f + 2..f + 2 + ende].to_string()));
                    rest = &rest[f + 4 + ende..];
                } else {
                    out.push((Teil::Normal, rest.to_string()));
                    break;
                }
            }
            _ => {
                out.push((Teil::Normal, rest.to_string()));
                break;
            }
        }
    }
    out.retain(|(_, s)| !s.is_empty());
    out
}
