//! Generators for worlds 13–15: error handling, traits, closures & iterators.

use super::*;

fn passt_zu(l: Sprache) -> &'static str {
    tr!(l, "Match them up.", "Ordne zu.", "Relaciona.")
}

fn was_luecke(l: Sprache) -> &'static str {
    tr!(l, "What goes in the gap?", "Was gehört in die Lücke?", "¿Qué va en el hueco?")
}

fn welcher_ausdruck(l: Sprache) -> &'static str {
    tr!(l, "What does the expression give?", "Was ergibt der Ausdruck?", "¿Qué da la expresión?")
}

// ─────────────────────────────── World 13: Error handling ───────────────────────────────

pub static W_FEHLER: &[Gen] = &[parse, unwrap_or_parse, fragezeichen, result_match, fehler_paare, fehler_wf, luecke_ok, fehler_main];

const EINGABEN: &[&str] = &["42", "255", "256", "-1", " 7", "3.5", "abc", "0", "+5", "1000", "12"];

fn parse(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let e = *r.wahl(EINGABEN);
    let (typ, ergebnis) = if r.bool() {
        ("u8", e.parse::<u8>().map(i64::from).map_err(|err| err.to_string()))
    } else {
        ("i32", e.parse::<i32>().map(i64::from).map_err(|err| err.to_string()))
    };
    let richtig = ergebnis.as_ref().map_or_else(|_| "`Err(…)`".to_string(), |v| format!("`Ok({v})`"));
    let erkl = match &ergebnis {
        Ok(v) => tr!(l, format!("“{e}” can be read as {typ}: {v}."), format!("„{e}“ lässt sich als {typ} lesen: {v}."), format!("«{e}» se puede leer como {typ}: {v}.")),
        Err(msg) => tr!(
            l,
            format!("Error: {msg}. (Spaces, commas, letters or values that are too big fail.)"),
            format!("Fehler: {msg}. (Leerzeichen, Kommas, Buchstaben oder zu große Werte schlagen fehl.)"),
            format!("Error: {msg}. (Los espacios, comas, letras o valores demasiado grandes fallan.)")
        ),
    };
    let zahl = e.trim().trim_start_matches('+').split('.').next().filter(|z| z.parse::<i64>().is_ok()).unwrap_or("255");
    auswahl(
        r,
        tr!(l, format!("What does `\"{e}\".parse::<{typ}>()` return?"), format!("Was liefert `\"{e}\".parse::<{typ}>()`?"), format!("¿Qué devuelve `\"{e}\".parse::<{typ}>()`?")),
        None,
        richtig,
        vec!["`Err(…)`".into(), "`Ok(0)`".into(), format!("`Ok({zahl})`"), tr!(l, "Crash (panic)", "Absturz (panic)", "Fallo (panic)").into()],
        erkl,
    )
}

fn unwrap_or_parse(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let a = *r.wahl(EINGABEN);
    let b = *r.wahl(EINGABEN);
    let (da, db) = (r.zahl(0, 9), r.zahl(0, 9));
    let wa = a.parse::<i32>().unwrap_or(da as i32);
    let wb = b.parse::<i32>().unwrap_or(db as i32);
    eingabe(
        tr!(l, "What is the value of `n`?", "Welchen Wert hat `n`?", "¿Qué valor tiene `n`?"),
        c(format!("let a: i32 = \"{a}\".parse().unwrap_or({da});\nlet b: i32 = \"{b}\".parse().unwrap_or({db});\nlet n = a + b;")),
        (wa + wb).to_string(),
        format!("a = {wa}, b = {wb}."),
    )
}

fn fragezeichen(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    frage_pool(
        r,
        tr!(
            l,
            &[
                ("What does `?` do with an `Err`?", "Returns the error from the function immediately", &["Ignores the error", "Crashes the program", "Turns the error into 0"], "`?` = with `Ok(v)` continue with `v`, with `Err(e)` immediately `return Err(e)`."),
                ("What does `?` do with an `Ok(5)`?", "Unwraps the 5 and carries on", &["Ends the function", "Returns `Ok(5)`", "Triggers a panic"], "On success `?` simply gives you the content."),
                ("Where may you use `?`?", "In functions that return `Result` (or `Option`) themselves", &["Everywhere", "Only in `main`", "Only in loops"], "The error has to be returned somewhere."),
                ("What happens with `unwrap()` on an `Err`?", "The program crashes (panic)", &["You get 0", "The error is ignored", "`None` is returned"], "That's why `unwrap` should be used carefully in real programs."),
            ],
            &[
                ("Was macht `?` bei einem `Err`?", "Gibt den Fehler sofort aus der Funktion zurück", &["Ignoriert den Fehler", "Lässt das Programm abstürzen", "Wandelt den Fehler in 0 um"], "`?` = bei `Ok(v)` weiter mit `v`, bei `Err(e)` sofort `return Err(e)`."),
                ("Was macht `?` bei einem `Ok(5)`?", "Packt die 5 aus und rechnet weiter", &["Beendet die Funktion", "Gibt `Ok(5)` zurück", "Löst einen panic aus"], "Bei Erfolg liefert `?` einfach den Inhalt."),
                ("Wo darf man `?` benutzen?", "In Funktionen, die selbst `Result` (oder `Option`) zurückgeben", &["Überall", "Nur in `main`", "Nur in Schleifen"], "Der Fehler muss ja irgendwohin zurückgegeben werden."),
                ("Was passiert bei `unwrap()` auf einem `Err`?", "Das Programm stürzt ab (panic)", &["Es kommt 0 heraus", "Der Fehler wird ignoriert", "Es wird `None` zurückgegeben"], "Deshalb ist `unwrap` in echten Programmen mit Vorsicht zu genießen."),
            ],
            &[
                ("¿Qué hace `?` con un `Err`?", "Devuelve el error de la función al instante", &["Ignora el error", "Hace fallar el programa", "Convierte el error en 0"], "`?` = con `Ok(v)` sigue con `v`; con `Err(e)`, `return Err(e)` inmediatamente."),
                ("¿Qué hace `?` con un `Ok(5)`?", "Saca el 5 y continúa", &["Termina la función", "Devuelve `Ok(5)`", "Provoca un panic"], "Si todo va bien, `?` te da el contenido."),
                ("¿Dónde se puede usar `?`?", "En funciones que devuelven `Result` (u `Option`)", &["En cualquier sitio", "Solo en `main`", "Solo en bucles"], "El error tiene que devolverse a algún sitio."),
                ("¿Qué pasa con `unwrap()` sobre un `Err`?", "El programa falla (panic)", &["Sale 0", "Se ignora el error", "Se devuelve `None`"], "Por eso `unwrap` debe usarse con cuidado en programas reales."),
            ]
        ),
    )
}

fn result_match(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let e = *r.wahl(&["21", "8", "x", "4.5", "100", "-3"]);
    let k = r.zahl(2, 4);
    let fehler = tr!(l, "Error", "Fehler", "Error");
    let erg = e.parse::<i32>().map_or_else(|_| fehler.to_string(), |v| (v * k as i32).to_string());
    eingabe(
        tr!(l, format!("What is printed? (a number or `{fehler}`)"), format!("Was wird ausgegeben? (Zahl oder `{fehler}`)"), format!("¿Qué se imprime? (un número o `{fehler}`)")),
        c(format!("match \"{e}\".parse::<i32>() {{\n    Ok(v) => println!(\"{{}}\", v * {k}),\n    Err(_) => println!(\"{fehler}\"),\n}}")),
        erg,
        tr!(l, "`parse` returns `Ok(number)` or `Err(…)` – `match` handles both cases.", "`parse` liefert `Ok(zahl)` oder `Err(…)` – `match` behandelt beide Fälle.", "`parse` devuelve `Ok(numero)` o `Err(…)`; `match` trata los dos casos."),
    )
}

fn fehler_paare(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    paare(
        r,
        passt_zu(l),
        &[
            ("unwrap()", tr!(l, "get the value or crash", "Wert holen oder abstürzen", "obtener el valor o fallar")),
            ("expect(\"…\")", tr!(l, "like unwrap, with own message", "wie unwrap, mit eigener Meldung", "como unwrap, con mensaje propio")),
            ("?", tr!(l, "pass the error on", "Fehler weitergeben", "propagar el error")),
            ("unwrap_or(x)", tr!(l, "default value on error", "Standardwert bei Fehler", "valor por defecto si hay error")),
            ("panic!", tr!(l, "stop the program right away", "Programm sofort abbrechen", "detener el programa al instante")),
            ("Result<T, E>", tr!(l, "Ok(T) or Err(E)", "Ok(T) oder Err(E)", "Ok(T) o Err(E)")),
        ],
        4,
    )
}

fn fehler_wf(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    wf_pool(
        r,
        tr!(
            l,
            &[
                ("`Result` has the variants `Ok` and `Err`.", true, "`Ok(value)` or `Err(error)`."),
                ("`\"256\".parse::<u8>()` crashes.", false, "It cleanly returns an `Err` – no crash."),
                ("`?` can be used in any function.", false, "Only in functions that return `Result` or `Option`."),
                ("`panic!` is meant for recoverable errors.", false, "For recoverable errors you use `Result`."),
                ("`unwrap_or(0)` never crashes.", true, "On an error you get the default value."),
            ],
            &[
                ("`Result` hat die Varianten `Ok` und `Err`.", true, "`Ok(wert)` oder `Err(fehler)`."),
                ("`\"256\".parse::<u8>()` stürzt ab.", false, "Es liefert sauber ein `Err` – ohne Absturz."),
                ("`?` darf in jeder Funktion benutzt werden.", false, "Nur in Funktionen, die `Result` oder `Option` zurückgeben."),
                ("`panic!` ist für behebbare Fehler gedacht.", false, "Für behebbare Fehler nimmt man `Result`."),
                ("`unwrap_or(0)` stürzt nie ab.", true, "Bei einem Fehler kommt der Standardwert heraus."),
            ],
            &[
                ("`Result` tiene las variantes `Ok` y `Err`.", true, "`Ok(valor)` o `Err(error)`."),
                ("`\"256\".parse::<u8>()` hace fallar el programa.", false, "Devuelve limpiamente un `Err`, sin fallar."),
                ("`?` se puede usar en cualquier función.", false, "Solo en funciones que devuelven `Result` u `Option`."),
                ("`panic!` sirve para errores recuperables.", false, "Para errores recuperables se usa `Result`."),
                ("`unwrap_or(0)` nunca falla.", true, "Si hay un error, sale el valor por defecto."),
            ]
        ),
    )
}

fn luecke_ok(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    luecke(
        r,
        was_luecke(l),
        "fn double(s: &str) -> Result<i32, ParseIntError> {\n    let n: i32 = s.parse()?;\n    ___(n * 2)\n}",
        "Ok",
        &["Some", "Err", "return"],
        tr!(l, "Success is wrapped in `Ok(…)`.", "Erfolg wird in `Ok(…)` verpackt.", "El éxito se envuelve en `Ok(…)`."),
    )
}

fn fehler_main(_: &mut Rng, _: u8, l: Sprache) -> Uebung {
    fehler(
        tr!(l, "Which line doesn't compile?", "Welche Zeile kompiliert nicht?", "¿Qué línea no compila?"),
        vs(&["fn main() {", "    let n: i32 = \"42\".parse()?;", "    println!(\"{n}\");", "}"]),
        1,
        tr!(
            l,
            "Here `main` returns nothing, so `?` isn't allowed. With `fn main() -> Result<(), ParseIntError>` it would work.",
            "`main` gibt hier nichts zurück, deshalb ist `?` nicht erlaubt. Mit `fn main() -> Result<(), ParseIntError>` ginge es.",
            "Aquí `main` no devuelve nada, así que `?` no está permitido. Con `fn main() -> Result<(), ParseIntError>` funcionaría."
        ),
    )
}

// ─────────────────────────────── World 14: Traits & generics ───────────────────────────────

pub static W_TRAITS: &[Gen] = &[formen, trait_paare, groesser, luecke_for, derive, traits_wf, standard_methode, bausteine_impl, luecke_schranke];

fn formen(r: &mut Rng, s: u8, l: Sprache) -> Uebung {
    match r.idx(3) {
        0 => {
            let rad = r.zahl(1, 6) as f64 + if s >= 2 { 0.5 } else { 0.0 };
            let f = std::f64::consts::PI * rad * rad;
            Uebung::Eingabe {
                frage: tr!(
                    l,
                    format!("A `Circle {{ r: {rad:?} }}` computes `area()` as π × r². What's the result? (2 decimals)"),
                    format!("Ein `Circle {{ r: {rad:?} }}` berechnet `area()` als π × r². Was kommt heraus? (2 Nachkommastellen)"),
                    format!("Un `Circle {{ r: {rad:?} }}` calcula `area()` como π × r². ¿Qué resultado da? (2 decimales)")
                ),
                code: None,
                antworten: vec![format!("{f:.2}")],
                erklaerung: format!("3.14159 × {rad} × {rad} ≈ {f:.2}"),
            }
        }
        1 => {
            let (a, b) = (r.zahl(2, 12), r.zahl(2, 12));
            eingabe(
                tr!(l, format!("A `Rectangle {{ a: {a}.0, b: {b}.0 }}` – what does `perimeter()` return?"), format!("Ein `Rectangle {{ a: {a}.0, b: {b}.0 }}` – was liefert `perimeter()`?"), format!("Un `Rectangle {{ a: {a}.0, b: {b}.0 }}`: ¿qué devuelve `perimeter()`?")),
                c("fn perimeter(&self) -> f64 {\n    2.0 * (self.a + self.b)\n}"),
                (2 * (a + b)).to_string(),
                format!("2 × ({a} + {b})"),
            )
        }
        _ => {
            let a = r.zahl(2, 15);
            eingabe(
                tr!(l, format!("A `Square {{ side: {a}.0 }}` – what does `area()` return?"), format!("Ein `Square {{ side: {a}.0 }}` – was liefert `area()`?"), format!("Un `Square {{ side: {a}.0 }}`: ¿qué devuelve `area()`?")),
                c("fn area(&self) -> f64 {\n    self.side * self.side\n}"),
                (a * a).to_string(),
                format!("{a} × {a}"),
            )
        }
    }
}

fn trait_paare(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    paare(
        r,
        tr!(l, "Which trait for what?", "Welcher Trait wofür?", "¿Qué trait para qué?"),
        &[
            ("Display", tr!(l, "printing with {}", "Ausgabe mit {}", "imprimir con {}")),
            ("Debug", tr!(l, "printing with {:?}", "Ausgabe mit {:?}", "imprimir con {:?}")),
            ("Clone", ".clone()"),
            ("PartialEq", tr!(l, "comparing with ==", "Vergleich mit ==", "comparar con ==")),
            ("PartialOrd", tr!(l, "comparing with <", "Vergleich mit <", "comparar con <")),
            ("Default", tr!(l, "default value", "Standardwert", "valor por defecto")),
        ],
        4,
    )
}

fn groesser(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let code = "fn larger<T: PartialOrd>(a: T, b: T) -> T {\n    if a > b { a } else { b }\n}";
    if r.bool() {
        let (a, b) = (r.zahl(-50, 50), r.zahl(-50, 50));
        eingabe(
            tr!(l, format!("What does `larger({a}, {b})` return?"), format!("Was liefert `larger({a}, {b})`?"), format!("¿Qué devuelve `larger({a}, {b})`?")),
            c(code),
            a.max(b).to_string(),
            tr!(l, "The larger number.", "Die größere Zahl.", "El número mayor."),
        )
    } else {
        let mut w = vec!["Apple", "Pear", "Lemon", "Banana", "apple", "Kiwi"];
        r.mischen(&mut w);
        let (a, b) = (w[0], w[1]);
        let erg = if a > b { a } else { b };
        auswahl(
            r,
            tr!(l, format!("What does `larger(\"{a}\", \"{b}\")` return?"), format!("Was liefert `larger(\"{a}\", \"{b}\")`?"), format!("¿Qué devuelve `larger(\"{a}\", \"{b}\")`?")),
            c(code),
            format!("`\"{erg}\"`"),
            vec![format!("`\"{a}\"`"), format!("`\"{b}\"`")],
            tr!(l, "Texts are compared character by character. Lowercase letters come after all capitals.", "Texte werden Zeichen für Zeichen verglichen. Kleinbuchstaben kommen nach allen Großbuchstaben.", "Los textos se comparan carácter a carácter. Las minúsculas van después de todas las mayúsculas."),
        )
    }
}

fn luecke_for(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    luecke(r, was_luecke(l), "impl Shape ___ Square {\n    fn area(&self) -> f64 {\n        self.side * self.side\n    }\n}", "for", &["in", "with", "to"], "`impl Trait for Type`")
}

fn derive(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let (was, richtig) = *r.wahl(&[("`println!(\"{:?}\", p)`", "Debug"), ("`p1 == p2`", "PartialEq"), ("`let q = p.clone();`", "Clone"), ("`p1 < p2`", "PartialOrd")]);
    auswahl(
        r,
        tr!(l, format!("Which trait must you derive so that {was} works?"), format!("Welchen Trait musst du ableiten, damit {was} funktioniert?"), format!("¿Qué trait debes derivar para que funcione {was}?")),
        c("#[derive(___)]\nstruct Point { x: i32, y: i32 }"),
        format!("`{richtig}`"),
        vs(&["`Debug`", "`PartialEq`", "`Clone`", "`PartialOrd`", "`Display`"]),
        tr!(l, "With `#[derive(...)]` Rust writes the implementation for you.", "Mit `#[derive(...)]` schreibt Rust die Implementierung für dich.", "Con `#[derive(...)]` Rust escribe la implementación por ti."),
    )
}

fn traits_wf(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    wf_pool(
        r,
        tr!(
            l,
            &[
                ("A trait describes shared behaviour.", true, "Similar to an interface."),
                ("A trait can contain default implementations.", true, "Types can keep or override them."),
                ("Generics make Rust programs slower.", false, "Rust generates its own fast code for every type."),
                ("`T: PartialOrd` means: T must be comparable.", true, "That's a trait bound."),
                ("`Display` can be derived with `#[derive]`.", false, "You implement `Display` yourself; `Debug` can be derived."),
            ],
            &[
                ("Ein Trait beschreibt gemeinsames Verhalten.", true, "Ähnlich wie ein Interface."),
                ("Ein Trait kann Standard-Implementierungen enthalten.", true, "Typen können sie übernehmen oder überschreiben."),
                ("Generics machen Rust-Programme langsamer.", false, "Rust erzeugt für jeden Typ eigenen, schnellen Code."),
                ("`T: PartialOrd` heißt: T muss vergleichbar sein.", true, "Das ist eine Trait-Schranke."),
                ("`Display` kann man mit `#[derive]` ableiten.", false, "`Display` musst du selbst implementieren, `Debug` lässt sich ableiten."),
            ],
            &[
                ("Un trait describe un comportamiento común.", true, "Parecido a una interfaz."),
                ("Un trait puede contener implementaciones por defecto.", true, "Los tipos pueden usarlas o sobrescribirlas."),
                ("Los genéricos hacen más lentos los programas en Rust.", false, "Rust genera código propio y rápido para cada tipo."),
                ("`T: PartialOrd` significa: T debe poder compararse.", true, "Es una restricción de trait."),
                ("`Display` se puede derivar con `#[derive]`.", false, "`Display` lo implementas tú; `Debug` sí se puede derivar."),
            ]
        ),
    )
}

fn standard_methode(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let ueberschreibt = r.bool();
    let (ich, miau) = tr!(l, ("I am", "Meow!"), ("Ich bin", "Miau!"), ("Soy", "¡Miau!"));
    let code = format!(
        "trait Animal {{\n    fn name(&self) -> String;\n    fn intro(&self) -> String {{\n        format!(\"{ich} {{}}\", self.name())\n    }}\n}}\n\nstruct Cat;\nimpl Animal for Cat {{\n    fn name(&self) -> String {{ \"Minka\".into() }}{}\n}}",
        if ueberschreibt { format!("\n    fn intro(&self) -> String {{ \"{miau}\".into() }}") } else { String::new() }
    );
    let richtig = if ueberschreibt { format!("`{miau}`") } else { format!("`{ich} Minka`") };
    auswahl(
        r,
        tr!(l, "What does `Cat.intro()` return?", "Was liefert `Cat.intro()`?", "¿Qué devuelve `Cat.intro()`?"),
        c(code),
        richtig,
        vec![format!("`{miau}`"), format!("`{ich} Minka`"), format!("`{ich} Cat`"), tr!(l, "Compiler error", "Compiler-Fehler", "Error de compilación").into()],
        tr!(l, "An own implementation overrides the trait's default method.", "Eine eigene Implementierung überschreibt die Standard-Methode des Traits.", "Una implementación propia sobrescribe el método por defecto del trait."),
    )
}

fn bausteine_impl(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let (t, ty) = *r.wahl(&[("Animal", "Dog"), ("Shape", "Circle"), ("Display", "Point")]);
    bausteine(
        tr!(l, format!("Implement the trait `{t}` for `{ty}`."), format!("Implementiere den Trait `{t}` für `{ty}`."), format!("Implementa el trait `{t}` para `{ty}`.")),
        vec!["impl".into(), t.into(), "for".into(), ty.into(), "{".into()],
        &["in", "trait", ":", "struct"],
        "`impl Trait for Type {`",
    )
}

fn luecke_schranke(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    luecke(
        r,
        tr!(l, "Which trait bound does the function need?", "Welche Trait-Schranke braucht die Funktion?", "¿Qué restricción de trait necesita la función?"),
        "fn largest<T: ___ + Copy>(list: &[T]) -> T {\n    let mut max = list[0];\n    for &x in list {\n        if x > max { max = x; }\n    }\n    max\n}",
        "PartialOrd",
        &["Display", "Clone", "Debug"],
        tr!(l, "For `>` the type `T` must be comparable: `PartialOrd`.", "Für `>` muss `T` vergleichbar sein: `PartialOrd`.", "Para `>`, `T` debe poder compararse: `PartialOrd`."),
    )
}

// ─────────────────────────────── World 15: Closures & iterators ───────────────────────────────

pub static W_CLOSURES: &[Gen] = &[map_sum, filter_count, capture, kette, collect_vec, iter_paare, luecke_filter, iter_wf, enumerate, any_all, fold];

fn map_sum(r: &mut Rng, s: u8, l: Sprache) -> Uebung {
    let n = r.zahl(3, 4 + 2 * i64::from(s));
    let k = r.zahl(2, 5);
    if r.bool() {
        eingabe(welcher_ausdruck(l), c(format!("(1..={n}).map(|x| x * {k}).sum::<i32>()")), ((1..=n).map(|x| x * k).sum::<i64>()).to_string(), format!("{k} × (1 + … + {n})"))
    } else {
        eingabe(welcher_ausdruck(l), c(format!("(1..={n}).map(|x| x * x).sum::<i32>()")), ((1..=n).map(|x| x * x).sum::<i64>()).to_string(), tr!(l, "Sum of squares: 1 + 4 + 9 + …", "Summe der Quadrate: 1 + 4 + 9 + …", "Suma de cuadrados: 1 + 4 + 9 + …"))
    }
}

fn filter_count(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let n = r.zahl(10, 50);
    let k = r.zahl(2, 7);
    let q = n / k;
    eingabe(
        welcher_ausdruck(l),
        c(format!("(1..={n}).filter(|x| x % {k} == 0).count()")),
        ((1..=n).filter(|x| x % k == 0).count()).to_string(),
        tr!(l, format!("Multiples of {k} up to {n}: {n} / {k} = {q} (rounded down)."), format!("Vielfache von {k} bis {n}: {n} / {k} = {q} (abgerundet)."), format!("Múltiplos de {k} hasta {n}: {n} / {k} = {q} (redondeando hacia abajo).")),
    )
}

fn capture(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let f = r.zahl(2, 5);
    let x = r.zahl(1, 6);
    eingabe(
        tr!(l, "What is printed?", "Was wird ausgegeben?", "¿Qué se imprime?"),
        c(format!("let factor = {f};\nlet times = |x: i32| x * factor;\nprintln!(\"{{}}\", times(times({x})));")),
        (x * f * f).to_string(),
        tr!(l, "The closure captures `factor` from its environment. Work from the inside out.", "Die Closure fängt `factor` aus der Umgebung ein. Von innen nach außen rechnen.", "La closure captura `factor` de su entorno. Calcula de dentro hacia fuera."),
    )
}

fn kette(r: &mut Rng, s: u8, l: Sprache) -> Uebung {
    let n = r.zahl(5, 6 + 2 * i64::from(s));
    let gerade = r.bool();
    let k = r.zahl(2, 4);
    let quadrat = r.bool();
    let werte: Vec<i64> = (1..=n).filter(|x| (x % 2 == 0) == gerade).map(|x| if quadrat { x * x } else { x * k }).collect();
    let map = if quadrat { "|x| x * x".to_string() } else { format!("|x| x * {k}") };
    eingabe(
        tr!(l, "What does the chain give?", "Was ergibt die Kette?", "¿Qué da la cadena?"),
        c(format!("(1..={n})\n    .filter(|x| x % 2 == {})\n    .map({map})\n    .sum::<i32>()", if gerade { 0 } else { 1 })),
        werte.iter().sum::<i64>().to_string(),
        tr!(l, format!("After filter and map: {werte:?}"), format!("Nach filter und map: {werte:?}"), format!("Después de filter y map: {werte:?}")),
    )
}

fn collect_vec(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let n = r.zahl(5, 12);
    let k = r.zahl(2, 4);
    let v: Vec<i64> = (1..=n).filter(|x| x % k == 0).collect();
    eingabe(
        tr!(l, "What does `v` contain? (e.g. `[1, 2, 3]`)", "Welchen Inhalt hat `v`? (z.B. `[1, 2, 3]`)", "¿Qué contiene `v`? (p. ej. `[1, 2, 3]`)"),
        c(format!("let v: Vec<i32> = (1..={n})\n    .filter(|x| x % {k} == 0)\n    .collect();")),
        format!("{v:?}"),
        tr!(l, "`collect` gathers the remaining values into a Vec.", "`collect` sammelt die übrigen Werte in einen Vec.", "`collect` reúne los valores restantes en un Vec."),
    )
}

fn iter_paare(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    paare(
        r,
        tr!(l, "Match the iterator methods.", "Ordne die Iterator-Methoden zu.", "Relaciona los métodos de iterador."),
        &[
            ("map", tr!(l, "transform every value", "jeden Wert umwandeln", "transformar cada valor")),
            ("filter", tr!(l, "keep some values", "Werte aussortieren", "quedarse con algunos valores")),
            ("sum", tr!(l, "add everything up", "alles addieren", "sumarlo todo")),
            ("collect", tr!(l, "put into a collection", "in eine Sammlung packen", "meter en una colección")),
            ("enumerate", tr!(l, "add the index", "Index dazu nehmen", "añadir el índice")),
            ("count", tr!(l, "count the values", "Anzahl zählen", "contar los valores")),
            ("rev", tr!(l, "backwards", "rückwärts", "al revés")),
        ],
        4,
    )
}

fn luecke_filter(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    if r.bool() {
        luecke(r, tr!(l, "Keep only even numbers:", "Nur gerade Zahlen behalten:", "Quédate solo con los pares:"), "let e: Vec<i32> = v.iter()\n    .___(|x| *x % 2 == 0)\n    .copied()\n    .collect();", "filter", &["map", "sum", "find_all"], tr!(l, "`filter` keeps only the values for which the closure returns `true`.", "`filter` behält nur die Werte, für die die Closure `true` liefert.", "`filter` conserva solo los valores para los que la closure devuelve `true`."))
    } else {
        luecke(r, tr!(l, "Double every number:", "Jede Zahl verdoppeln:", "Duplica cada número:"), "let d: Vec<i32> = v.iter()\n    .___(|x| x * 2)\n    .collect();", "map", &["filter", "each", "double"], tr!(l, "`map` transforms every value.", "`map` wandelt jeden Wert um.", "`map` transforma cada valor."))
    }
}

fn iter_wf(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    wf_pool(
        r,
        tr!(
            l,
            &[
                ("Iterators are lazy: `map` alone does nothing yet.", true, "Only `sum`, `collect`, `count` etc. pull the values."),
                ("Closures can use variables from their environment.", true, "That's called capturing."),
                ("Iterators are slower than normal loops.", false, "They're just as fast – a zero-cost abstraction."),
                ("`|x| x + 1` is a closure.", true, "Parameters between vertical bars."),
                ("`filter` changes the values.", false, "`filter` only sorts out, `map` changes."),
            ],
            &[
                ("Iteratoren sind lazy: `map` allein tut noch nichts.", true, "Erst `sum`, `collect`, `count` usw. holen die Werte ab."),
                ("Closures können Variablen aus ihrer Umgebung benutzen.", true, "Das nennt man „einfangen“ (capture)."),
                ("Iteratoren sind langsamer als normale Schleifen.", false, "Sie sind genauso schnell – Zero-Cost Abstraction."),
                ("`|x| x + 1` ist eine Closure.", true, "Parameter zwischen senkrechten Strichen."),
                ("`filter` verändert die Werte.", false, "`filter` sortiert nur aus, `map` verändert."),
            ],
            &[
                ("Los iteradores son perezosos: `map` sola no hace nada todavía.", true, "Solo `sum`, `collect`, `count`, etc. recogen los valores."),
                ("Las closures pueden usar variables de su entorno.", true, "Eso se llama capturar."),
                ("Los iteradores son más lentos que los bucles normales.", false, "Son igual de rápidos: una abstracción de coste cero."),
                ("`|x| x + 1` es una closure.", true, "Los parámetros van entre barras verticales."),
                ("`filter` cambia los valores.", false, "`filter` solo selecciona; `map` cambia."),
            ]
        ),
    )
}

fn enumerate(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let pool = ["ada", "linus", "grace", "ferris", "alan", "mia"];
    let n = r.zahl(3, 6) as usize;
    let w: Vec<&str> = pool.iter().take(n).copied().collect();
    let ziel = r.idx(n);
    let name = w[ziel];
    eingabe(
        tr!(l, format!("Which index `i` is printed for “{name}”?"), format!("Welcher Index `i` wird bei „{name}“ ausgegeben?"), format!("¿Qué índice `i` se imprime para «{name}»?")),
        c(format!("let names = [{}];\nfor (i, n) in names.iter().enumerate() {{\n    println!(\"{{i}}: {{n}}\");\n}}", w.iter().map(|x| format!("\"{x}\"")).collect::<Vec<_>>().join(", "))),
        ziel.to_string(),
        tr!(l, "`enumerate` counts from 0.", "`enumerate` zählt ab 0.", "`enumerate` cuenta desde 0."),
    )
}

fn any_all(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let v: Vec<i64> = (0..5).map(|_| r.zahl(1, 30)).collect();
    let k = r.zahl(5, 25);
    let liste = v.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ");
    if r.bool() {
        let erg = v.iter().any(|&x| x > k);
        auswahl(r, welcher_ausdruck(l), c(format!("let v = vec![{liste}];\nv.iter().any(|&x| x > {k})")), erg.to_string(), vec![(!erg).to_string()], tr!(l, format!("`any` is true if **at least one** value is greater than {k}."), format!("`any` ist true, wenn **mindestens ein** Wert größer als {k} ist."), format!("`any` es true si **al menos un** valor es mayor que {k}.")))
    } else {
        let erg = v.iter().all(|&x| x > k);
        auswahl(r, welcher_ausdruck(l), c(format!("let v = vec![{liste}];\nv.iter().all(|&x| x > {k})")), erg.to_string(), vec![(!erg).to_string()], tr!(l, format!("`all` is only true if **every** value is greater than {k}."), format!("`all` ist nur true, wenn **jeder** Wert größer als {k} ist."), format!("`all` solo es true si **todos** los valores son mayores que {k}.")))
    }
}

fn fold(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    if r.bool() {
        let n = r.zahl(3, 6);
        eingabe(welcher_ausdruck(l), c(format!("(1..={n}).product::<i32>()")), ((1..=n).product::<i64>()).to_string(), tr!(l, format!("1 × 2 × … × {n} (factorial)"), format!("1 × 2 × … × {n} (Fakultät)"), format!("1 × 2 × … × {n} (factorial)")))
    } else {
        let n = r.zahl(3, 8);
        let start = r.zahl(0, 10);
        eingabe(welcher_ausdruck(l), c(format!("(1..={n}).fold({start}, |acc, x| acc + x)")), ((1..=n).fold(start, |acc, x| acc + x)).to_string(), tr!(l, format!("`fold` starts at {start} and adds every value."), format!("`fold` startet bei {start} und addiert jeden Wert dazu."), format!("`fold` empieza en {start} y suma cada valor.")))
    }
}
