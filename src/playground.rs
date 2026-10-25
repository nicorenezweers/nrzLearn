//! Real execution of Rust code through the official Rust Playground
//! (https://play.rust-lang.org). The code is compiled and run there.

use serde::{Deserialize, Serialize};

use crate::i18n::Sprache;

const URL: &str = "https://play.rust-lang.org/execute";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Anfrage<'a> {
    channel: &'a str,
    mode: &'a str,
    edition: &'a str,
    crate_type: &'a str,
    tests: bool,
    code: &'a str,
    backtrace: bool,
}

#[derive(Deserialize, Clone, PartialEq, Debug, Default)]
pub struct Ergebnis {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

impl Ergebnis {
    /// Cargo's lines ("Compiling", "Finished", "Running") without error messages.
    pub fn cargo_zeilen(&self) -> Vec<String> {
        self.stderr
            .lines()
            .filter(|l| {
                let t = l.trim_start();
                t.starts_with("Compiling") || t.starts_with("Finished") || t.starts_with("Running")
            })
            .map(str::to_string)
            .collect()
    }

    /// Everything that isn't a normal Cargo status line (errors, warnings, panics).
    pub fn meldungen(&self) -> String {
        self.stderr
            .lines()
            .filter(|l| {
                let t = l.trim_start();
                !(t.starts_with("Compiling") || t.starts_with("Finished") || t.starts_with("Running"))
            })
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string()
    }
}

pub async fn ausfuehren(code: &str, tests: bool) -> Result<Ergebnis, String> {
    let anfrage = Anfrage {
        channel: "stable",
        mode: "debug",
        edition: "2024",
        crate_type: "bin",
        tests,
        code,
        backtrace: false,
    };
    let antwort = gloo_net::http::Request::post(URL)
        .json(&anfrage)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|_| "network".to_string())?;
    if !antwort.ok() {
        return Err(format!("status {}", antwort.status()));
    }
    antwort.json::<Ergebnis>().await.map_err(|_| "network".to_string())
}

/// Readable message for an error from `ausfuehren`.
pub fn fehlermeldung(l: Sprache, fehler: &str) -> String {
    if let Some(status) = fehler.strip_prefix("status ") {
        tr!(l, format!("The Rust Playground answered with status {status}."), format!("Der Rust Playground antwortet mit Status {status}."), format!("El Rust Playground respondió con el estado {status}."))
    } else {
        tr!(l, "No connection to the Rust Playground. Are you online?", "Keine Verbindung zum Rust Playground. Bist du online?", "No hay conexión con el Rust Playground. ¿Tienes internet?").to_string()
    }
}

/// Explains common compiler errors in the learner's language.
pub fn fehler_erklaerung(l: Sprache, stderr: &str) -> Option<&'static str> {
    const ERKLAERUNGEN: &[(&str, [&str; 3])] = &[
        ("E0384", ["You're trying to change an immutable variable. Write `let mut` instead of `let`.", "Du versuchst, eine unveränderliche Variable zu ändern. Schreibe `let mut` statt `let`.", "Intentas cambiar una variable inmutable. Escribe `let mut` en lugar de `let`."]),
        ("E0596", ["You want to change something that isn't marked `mut`. Check `let mut` or `&mut`.", "Du willst etwas verändern, das nicht als `mut` markiert ist. Prüfe `let mut` bzw. `&mut`.", "Quieres cambiar algo que no está marcado como `mut`. Revisa `let mut` o `&mut`."]),
        ("E0382", ["The value has already been moved. After `let b = a;` the value belongs to `b` – use `.clone()` or a reference `&a`.", "Der Wert wurde schon verschoben (move). Nach `let b = a;` gehört der Wert `b` – nutze `.clone()` oder eine Referenz `&a`.", "El valor ya se movió. Después de `let b = a;` el valor pertenece a `b`: usa `.clone()` o una referencia `&a`."]),
        ("E0499", ["There may only be one mutable reference (`&mut`) at a time.", "Es darf nur eine einzige veränderbare Referenz (`&mut`) gleichzeitig geben.", "Solo puede haber una referencia mutable (`&mut`) a la vez."]),
        ("E0502", ["You're mixing `&` and `&mut` at the same time. Either many readers OR one writer.", "Du mischst `&` und `&mut` gleichzeitig. Entweder viele Leser ODER genau ein Schreiber.", "Estás mezclando `&` y `&mut` a la vez. O muchos lectores O un solo escritor."]),
        ("E0505", ["You're moving a value while a reference to it still exists.", "Du verschiebst einen Wert, während noch eine Referenz darauf existiert.", "Estás moviendo un valor mientras todavía existe una referencia a él."]),
        ("E0308", ["The types don't match. Look at which type is expected and which one was found.", "Die Typen passen nicht zusammen. Schau dir an, welcher Typ erwartet wird (expected) und welcher gefunden wurde (found).", "Los tipos no coinciden. Mira qué tipo se espera (expected) y cuál se encontró (found)."]),
        ("E0425", ["The compiler doesn't know this name. A typo? Or is the variable outside its scope?", "Diesen Namen kennt der Compiler nicht. Tippfehler? Oder ist die Variable außerhalb ihres Gültigkeitsbereichs (Scope)?", "El compilador no conoce este nombre. ¿Una errata? ¿O la variable está fuera de su ámbito?"]),
        ("E0599", ["This method doesn't exist for this type. Check the name or whether a trait is missing.", "Diese Methode gibt es für diesen Typ nicht. Prüfe den Namen oder ob ein Trait fehlt.", "Este método no existe para este tipo. Revisa el nombre o si falta un trait."]),
        ("E0106", ["A lifetime annotation like `'a` is missing on a reference.", "Hier fehlt eine Lifetime-Angabe wie `'a` bei einer Referenz.", "Falta una anotación de lifetime como `'a` en una referencia."]),
        ("E0277", ["A trait isn't implemented for this type – e.g. you can't add `i32` and `f64` directly, or `{}` needs `Display`.", "Ein Trait ist für diesen Typ nicht implementiert – z.B. kann man `i32` und `f64` nicht direkt addieren, oder `{}` braucht `Display`.", "Un trait no está implementado para este tipo: p. ej. no se pueden sumar `i32` y `f64` directamente, o `{}` necesita `Display`."]),
        ("E0004", ["Your `match` doesn't cover every case. Add the missing variants or use `_ =>`.", "Dein `match` deckt nicht alle Fälle ab. Ergänze die fehlenden Varianten oder nutze `_ =>`.", "Tu `match` no cubre todos los casos. Añade las variantes que faltan o usa `_ =>`."]),
        ("E0061", ["The number of arguments doesn't match the function definition.", "Die Anzahl der Argumente stimmt nicht mit der Funktionsdefinition überein.", "El número de argumentos no coincide con la definición de la función."]),
        ("E0432", ["A `use` import couldn't be found.", "Ein `use`-Import konnte nicht gefunden werden.", "No se encontró una importación `use`."]),
        ("E0369", ["This operator (e.g. `+` or `==`) isn't defined for this type.", "Dieser Operator (z.B. `+` oder `==`) ist für diesen Typ nicht definiert.", "Este operador (p. ej. `+` o `==`) no está definido para este tipo."]),
        ("E0614", ["You're dereferencing (`*`) something that isn't a reference.", "Du dereferenzierst (`*`) etwas, das keine Referenz ist.", "Estás desreferenciando (`*`) algo que no es una referencia."]),
        ("E0381", ["The variable is used before it got a value.", "Die Variable wird benutzt, bevor sie einen Wert bekommen hat.", "La variable se usa antes de tener un valor."]),
        ("E0507", ["You're trying to move a value out of a reference. Use `.clone()` or work with the reference.", "Du versuchst, einen Wert aus einer Referenz herauszuverschieben. Nutze `.clone()` oder arbeite mit der Referenz.", "Intentas mover un valor fuera de una referencia. Usa `.clone()` o trabaja con la referencia."]),
        ("attempt to subtract with overflow", ["Overflow! The result doesn't fit the type (e.g. a `u8` can't become negative).", "Überlauf! Das Ergebnis passt nicht in den Typ (z.B. ein `u8` kann nicht negativ werden).", "¡Desbordamiento! El resultado no cabe en el tipo (p. ej. un `u8` no puede ser negativo)."]),
        ("attempt to add with overflow", ["Overflow! The result is bigger than the type's maximum.", "Überlauf! Das Ergebnis ist größer als der maximale Wert des Typs.", "¡Desbordamiento! El resultado es mayor que el máximo del tipo."]),
        ("attempt to multiply with overflow", ["Overflow! The product doesn't fit the type.", "Überlauf! Das Produkt passt nicht in den Typ.", "¡Desbordamiento! El producto no cabe en el tipo."]),
        ("index out of bounds", ["You're accessing an index that doesn't exist. Indices start at 0 and end at `len() - 1`.", "Du greifst auf einen Index zu, den es nicht gibt. Indizes beginnen bei 0 und enden bei `len() - 1`.", "Accedes a un índice que no existe. Los índices empiezan en 0 y terminan en `len() - 1`."]),
        ("called `Option::unwrap()` on a `None` value", ["`unwrap()` on `None` crashes the program. Use `match`, `if let` or `unwrap_or`.", "`unwrap()` auf `None` lässt das Programm abstürzen. Nutze `match`, `if let` oder `unwrap_or`.", "`unwrap()` sobre `None` hace fallar el programa. Usa `match`, `if let` o `unwrap_or`."]),
        ("called `Result::unwrap()` on an `Err` value", ["`unwrap()` on an `Err` crashes the program. Handle the error with `match` or `?`.", "`unwrap()` auf einem `Err` lässt das Programm abstürzen. Behandle den Fehler mit `match` oder `?`.", "`unwrap()` sobre un `Err` hace fallar el programa. Trata el error con `match` o `?`."]),
        ("expected `;`", ["A semicolon `;` is missing at the end of the statement.", "Hier fehlt ein Semikolon `;` am Ende der Anweisung.", "Falta un punto y coma `;` al final de la instrucción."]),
        ("unterminated double quote string", ["A string wasn't closed with `\"`.", "Ein String wurde nicht mit `\"` geschlossen.", "Un string no se cerró con `\"`."]),
    ];
    ERKLAERUNGEN.iter().find(|(muster, _)| stderr.contains(muster)).map(|(_, text)| text[l as usize])
}
