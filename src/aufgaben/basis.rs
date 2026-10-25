//! Generators for worlds 1–4: Hello Rust, variables, numbers, functions.

use super::*;

// ─────────────────────────────── World 1: Hello, Rust ───────────────────────────────

pub static W_HALLO: &[Gen] = &[
    println_ausgabe, makro, bausteine_println, fehler_syntax, cargo_paare, cargo_auswahl, projekt_reihenfolge,
    terminal_projekt, hallo_wf, ausgabe_zeilen,
];

fn println_ausgabe(r: &mut Rng, s: u8, l: Sprache) -> Uebung {
    let name = *r.wahl(NAMEN);
    let frage = tr!(l, "What does this line print?", "Was gibt diese Zeile aus?", "¿Qué imprime esta línea?");
    match s {
        1 => {
            let gruss = tr!(l, "Hello", "Hallo", "Hola");
            eingabe(
                frage,
                c(format!("println!(\"{gruss}, {{}}!\", \"{name}\");")),
                format!("{gruss}, {name}!"),
                tr!(l, "`{}` is replaced by the first argument after the comma.", "`{}` wird durch das erste Argument nach dem Komma ersetzt.", "`{}` se sustituye por el primer argumento después de la coma."),
            )
        }
        2 => {
            let (a, b) = (r.zahl(2, 20), r.zahl(2, 20));
            eingabe(
                frage,
                c(format!("println!(\"{{}} + {{}} = {{}}\", {a}, {b}, {a} + {b});")),
                format!("{a} + {b} = {}", a + b),
                tr!(l, "The placeholders are filled in order – the third one with the result of the calculation.", "Die Platzhalter werden der Reihe nach gefüllt – der dritte mit dem Ergebnis der Rechnung.", "Los marcadores se rellenan en orden; el tercero con el resultado del cálculo."),
            )
        }
        _ => {
            let w = *r.wahl(&["Rust", "Ferris", "Code"]);
            eingabe(
                tr!(l, "What does this line print? (positional placeholders!)", "Was gibt diese Zeile aus? (Positions-Platzhalter!)", "¿Qué imprime esta línea? (¡marcadores por posición!)"),
                c(format!("println!(\"{{0}} {{1}} {{0}}\", \"{name}\", \"{w}\");")),
                format!("{name} {w} {name}"),
                tr!(l, "`{0}` is always the first argument, `{1}` the second – no matter how often they appear.", "`{0}` ist immer das erste Argument, `{1}` das zweite – egal wie oft sie vorkommen.", "`{0}` es siempre el primer argumento y `{1}` el segundo, aparezcan las veces que aparezcan."),
            )
        }
    }
}

fn makro(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let richtig = *r.wahl(&["`println!`", "`vec!`", "`format!`", "`print!`", "`panic!`", "`assert!`"]);
    auswahl(
        r,
        tr!(l, "Which of these is a **macro**?", "Welches davon ist ein **Makro**?", "¿Cuál de estos es una **macro**?"),
        None,
        richtig,
        vs(&["`main()`", "`len()`", "`String::new()`", "`push()`", "`to_string()`", "`clone()`"]),
        tr!(l, "You recognise macros by the exclamation mark `!`.", "Makros erkennst du am Ausrufezeichen `!`.", "Las macros se reconocen por el signo de exclamación `!`."),
    )
}

fn bausteine_println(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let name = *r.wahl(NAMEN);
    let gruss = tr!(l, "Hello", "Hallo", "Hola");
    bausteine(
        tr!(l, format!("Build the line that prints `{gruss}, {name}!`."), format!("Baue die Zeile, die `{gruss}, {name}!` ausgibt."), format!("Construye la línea que imprime `{gruss}, {name}!`.")),
        vec!["println!".into(), "(".into(), format!("\"{gruss}, {name}!\""), ")".into(), ";".into()],
        &["print", "echo", "{", "fn"],
        tr!(l, "Macro name with `!`, then the text in quotes inside parentheses, a semicolon at the end.", "Makro-Name mit `!`, dann der Text in Anführungszeichen in Klammern, am Ende ein Semikolon.", "Nombre de la macro con `!`, luego el texto entre comillas dentro de paréntesis y un punto y coma al final."),
    )
}

fn fehler_syntax(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let woerter = tr!(l, ["One", "Two", "Three"], ["Eins", "Zwei", "Drei"], ["Uno", "Dos", "Tres"]);
    let kaputt = r.idx(3);
    let art = r.idx(3);
    let mut zeilen = vec!["fn main() {".to_string()];
    for (i, w) in woerter.iter().enumerate() {
        zeilen.push(if i == kaputt {
            match art {
                0 => format!("    println!(\"{w}\")"),
                1 => format!("    println!('{w}');"),
                _ => format!("    println(\"{w}\");"),
            }
        } else {
            format!("    println!(\"{w}\");")
        });
    }
    zeilen.push("}".into());
    let erkl = match art {
        0 => tr!(l, "The semicolon `;` at the end is missing.", "Hier fehlt das Semikolon `;` am Ende.", "Falta el punto y coma `;` al final."),
        1 => tr!(l, "Text goes in double quotes `\"…\"`. Single quotes are only for single characters (`char`).", "Text steht in doppelten Anführungszeichen `\"…\"`. Einfache sind nur für einzelne Zeichen (`char`).", "El texto va entre comillas dobles `\"…\"`. Las simples son solo para caracteres sueltos (`char`)."),
        _ => tr!(l, "`println` is missing its `!` – it's a macro.", "Bei `println` fehlt das `!` – es ist ein Makro.", "A `println` le falta el `!`: es una macro."),
    };
    fehler(tr!(l, "Which line has the bug? Tap it.", "In welcher Zeile ist der Fehler? Tippe sie an.", "¿Qué línea tiene el error? Tócala."), zeilen, kaputt + 1, erkl)
}

fn cargo_tabelle(l: Sprache) -> Vec<(&'static str, &'static str)> {
    vec![
        ("cargo new", tr!(l, "create a new project", "neues Projekt anlegen", "crear un proyecto nuevo")),
        ("cargo run", tr!(l, "compile and run", "kompilieren und starten", "compilar y ejecutar")),
        ("cargo build", tr!(l, "compile only", "nur kompilieren", "solo compilar")),
        ("cargo check", tr!(l, "quickly check for errors", "schnell auf Fehler prüfen", "comprobar errores rápido")),
        ("cargo test", tr!(l, "run tests", "Tests ausführen", "ejecutar tests")),
        ("cargo add", tr!(l, "add a dependency", "Abhängigkeit hinzufügen", "añadir una dependencia")),
        ("cargo fmt", tr!(l, "format code", "Code formatieren", "formatear el código")),
        ("cargo clippy", tr!(l, "improvement tips", "Verbesserungstipps", "consejos de mejora")),
    ]
}

fn cargo_paare(r: &mut Rng, s: u8, l: Sprache) -> Uebung {
    paare(r, tr!(l, "Match the Cargo commands to their meaning.", "Ordne die Cargo-Befehle ihrer Bedeutung zu.", "Relaciona los comandos de Cargo con su significado."), &cargo_tabelle(l), 3 + s as usize)
}

fn cargo_auswahl(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let tabelle = cargo_tabelle(l);
    let (cmd, was) = *r.wahl(&tabelle);
    let falsche: Vec<String> = tabelle.iter().filter(|(c, _)| *c != cmd).map(|(c, _)| format!("`{c}`")).collect();
    auswahl(
        r,
        tr!(l, format!("Which command lets you **{was}**?"), format!("Mit welchem Befehl kannst du: **{was}**?"), format!("¿Con qué comando puedes **{was}**?")),
        None,
        format!("`{cmd}`"),
        falsche,
        format!("`{cmd}` – {was}."),
    )
}

fn projekt_reihenfolge(r: &mut Rng, s: u8, l: Sprache) -> Uebung {
    let p = *r.wahl(projekte(l));
    let mut z = vec![format!("cargo new {p}"), format!("cd {p}")];
    if s >= 2 {
        z.push("cat src/main.rs".into());
    }
    z.push("cargo run".into());
    reihenfolge(
        tr!(l, format!("Put the commands in order to create and run the project `{p}`."), format!("Bring die Befehle in die richtige Reihenfolge, um das Projekt `{p}` anzulegen und zu starten."), format!("Ordena los comandos para crear y ejecutar el proyecto `{p}`.")),
        z,
        tr!(l, "Create first, then change into the folder – that's where Cargo finds Cargo.toml.", "Erst anlegen, dann in den Ordner wechseln – dort findet Cargo die Cargo.toml.", "Primero crear y luego entrar en la carpeta: ahí Cargo encuentra el Cargo.toml."),
    )
}

fn terminal_projekt(r: &mut Rng, s: u8, l: Sprache) -> Uebung {
    let p = *r.wahl(projekte(l));
    let mut ziele = vec![format!("cargo new {p}"), format!("cd {p}")];
    if s >= 2 {
        ziele.push("tree".into());
    }
    if s >= 3 {
        ziele.push("cargo build".into());
    }
    ziele.push("cargo run".into());
    Uebung::Terminal {
        aufgabe: tr!(l, format!("Create the project `{p}` and run the commands in order."), format!("Lege das Projekt `{p}` an und führe die Befehle der Reihe nach aus."), format!("Crea el proyecto `{p}` y ejecuta los comandos en orden.")),
        ziele,
        hinweis: tr!(l, "Type each command exactly like this and press Enter.", "Tippe jeden Befehl genau so ein und drücke Enter.", "Escribe cada comando tal cual y pulsa Enter.").into(),
    }
}

fn hallo_wf(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    wf_pool(
        r,
        tr!(
            l,
            &[
                ("Comments in Rust start with `//`.", true, "The compiler ignores everything after `//` up to the end of the line."),
                ("Every statement in Rust ends with a comma.", false, "Statements end with a semicolon `;`."),
                ("Rust source files end in `.rs`.", true, "For example `main.rs`."),
                ("`cargo run` compiles and starts the program.", true, "Exactly – in one step."),
                ("Rust needs a garbage collector.", false, "Rust frees memory through ownership – without a garbage collector."),
                ("The Rust compiler is called `rustc`.", true, "Cargo calls `rustc` for you."),
                ("`Cargo.toml` lists a project's dependencies.", true, "Under `[dependencies]`."),
                ("You recognise macros by the question mark `?`.", false, "You recognise macros by the exclamation mark `!`."),
                ("Rust programs need a virtual machine to run.", false, "Rust compiles straight to machine code."),
            ],
            &[
                ("Kommentare beginnen in Rust mit `//`.", true, "Alles nach `//` bis zum Zeilenende ignoriert der Compiler."),
                ("Jede Anweisung endet in Rust mit einem Komma.", false, "Anweisungen enden mit einem Semikolon `;`."),
                ("Rust-Quelldateien enden auf `.rs`.", true, "Zum Beispiel `main.rs`."),
                ("`cargo run` kompiliert und startet das Programm.", true, "Genau – in einem Schritt."),
                ("Rust braucht einen Garbage Collector.", false, "Rust räumt Speicher über Ownership auf – ganz ohne Garbage Collector."),
                ("Der Rust-Compiler heißt `rustc`.", true, "Cargo ruft `rustc` für dich auf."),
                ("In der `Cargo.toml` stehen die Abhängigkeiten eines Projekts.", true, "Unter `[dependencies]`."),
                ("Makros erkennt man am Fragezeichen `?`.", false, "Makros erkennt man am Ausrufezeichen `!`."),
                ("Rust-Programme brauchen zum Laufen eine virtuelle Maschine.", false, "Rust kompiliert direkt zu Maschinencode."),
            ],
            &[
                ("En Rust, los comentarios empiezan con `//`.", true, "El compilador ignora todo lo que va después de `//` hasta el final de la línea."),
                ("En Rust, cada instrucción termina con una coma.", false, "Las instrucciones terminan con punto y coma `;`."),
                ("Los archivos de código Rust terminan en `.rs`.", true, "Por ejemplo `main.rs`."),
                ("`cargo run` compila y ejecuta el programa.", true, "Exacto, en un solo paso."),
                ("Rust necesita un recolector de basura.", false, "Rust libera la memoria mediante ownership, sin recolector de basura."),
                ("El compilador de Rust se llama `rustc`.", true, "Cargo llama a `rustc` por ti."),
                ("En `Cargo.toml` están las dependencias de un proyecto.", true, "En `[dependencies]`."),
                ("Las macros se reconocen por el signo de interrogación `?`.", false, "Las macros se reconocen por el signo de exclamación `!`."),
                ("Los programas en Rust necesitan una máquina virtual para ejecutarse.", false, "Rust compila directamente a código máquina."),
            ]
        ),
    )
}

fn ausgabe_zeilen(r: &mut Rng, s: u8, l: Sprache) -> Uebung {
    let n = 3 + s as usize;
    let mut code = String::new();
    let mut ausgabe = String::new();
    for i in 0..n {
        let buchstabe = (b'A' + i as u8) as char;
        if r.bool() {
            code.push_str(&format!("println!(\"{buchstabe}\");\n"));
            ausgabe.push(buchstabe);
            ausgabe.push('\n');
        } else {
            code.push_str(&format!("print!(\"{buchstabe}\");\n"));
            ausgabe.push(buchstabe);
        }
    }
    let zeilen = ausgabe.lines().count();
    let gezeigt = ausgabe.trim_end().replace('\n', " / ");
    eingabe(
        tr!(l, "How many lines appear in the output?", "Wie viele Zeilen erscheinen in der Ausgabe?", "¿Cuántas líneas aparecen en la salida?"),
        c(code.trim_end()),
        zeilen.to_string(),
        tr!(
            l,
            format!("`print!` doesn't add a line break, `println!` does. The output is: {gezeigt}"),
            format!("`print!` macht keinen Zeilenumbruch, `println!` schon. Die Ausgabe ist: {gezeigt}"),
            format!("`print!` no añade salto de línea; `println!` sí. La salida es: {gezeigt}")
        ),
    )
}

// ─────────────────────────────── World 2: Variables ───────────────────────────────

pub static W_VARIABLEN: &[Gen] = &[
    shadowing, zusammengesetzt, mut_kompiliert, scope_wert, fehler_mut, luecke_mut, bausteine_let, konstante,
    variablen_wf, variablen_paare,
];

fn rechne(x: i64, op: char, y: i64) -> i64 {
    match op {
        '+' => x + y,
        '-' => x - y,
        _ => x * y,
    }
}

fn welchen_wert(l: Sprache, v: &str) -> String {
    tr!(l, format!("What is the value of `{v}` at the end?"), format!("Welchen Wert hat `{v}` am Ende?"), format!("¿Qué valor tiene `{v}` al final?"))
}

fn shadowing(r: &mut Rng, s: u8, l: Sprache) -> Uebung {
    let v = *r.wahl(vars(l));
    let a = r.zahl(1, 9);
    let mut x = a;
    let mut code = format!("let {v} = {a};\n");
    for _ in 0..(1 + s) {
        let op = *r.wahl(&['+', '-', '*']);
        let b = r.zahl(2, 6);
        x = rechne(x, op, b);
        code.push_str(&format!("let {v} = {v} {op} {b};\n"));
    }
    eingabe(
        welchen_wert(l, v),
        c(code.trim_end()),
        x.to_string(),
        tr!(l, "Every `let` creates a new variable (shadowing) from the previous value.", "Jedes `let` erzeugt eine neue Variable (Shadowing) aus dem vorherigen Wert.", "Cada `let` crea una variable nueva (shadowing) a partir del valor anterior."),
    )
}

fn zusammengesetzt(r: &mut Rng, s: u8, l: Sprache) -> Uebung {
    let v = *r.wahl(vars(l));
    let a = r.zahl(1, 10);
    let mut x = a;
    let mut code = format!("let mut {v} = {a};\n");
    for _ in 0..(2 + s) {
        let op = *r.wahl(&['+', '-', '*']);
        let b = r.zahl(2, 5);
        x = rechne(x, op, b);
        code.push_str(&format!("{v} {op}= {b};\n"));
    }
    eingabe(
        welchen_wert(l, v),
        c(code.trim_end()),
        x.to_string(),
        tr!(l, "`x += 3` is short for `x = x + 3` – the same goes for `-=` and `*=`.", "`x += 3` ist die Kurzform für `x = x + 3` – genauso bei `-=` und `*=`.", "`x += 3` es la forma corta de `x = x + 3`; igual con `-=` y `*=`."),
    )
}

fn kompiliert(l: Sprache) -> &'static str {
    tr!(l, "Does this code compile?", "Kompiliert dieser Code?", "¿Compila este código?")
}

fn mut_kompiliert(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let v = *r.wahl(vars(l));
    let (a, b) = (r.zahl(1, 9), r.zahl(10, 20));
    match r.idx(3) {
        0 => wf(
            kompiliert(l),
            c(format!("let {v} = {a};\n{v} = {b};\nprintln!(\"{{{v}}}\");")),
            false,
            tr!(l, format!("Without `mut`, `{v}` can't be reassigned (E0384)."), format!("Ohne `mut` darf `{v}` nicht neu zugewiesen werden (E0384)."), format!("Sin `mut`, `{v}` no se puede reasignar (E0384).")),
        ),
        1 => wf(kompiliert(l), c(format!("let mut {v} = {a};\n{v} = {b};\nprintln!(\"{{{v}}}\");")), true, tr!(l, "With `mut` the reassignment is allowed.", "Mit `mut` ist die Neuzuweisung erlaubt.", "Con `mut` la reasignación está permitida.")),
        _ => wf(
            kompiliert(l),
            c(format!("let {v} = {a};\nlet {v} = {b};\nprintln!(\"{{{v}}}\");")),
            true,
            tr!(l, "That's shadowing: the second `let` creates a new variable – no `mut` needed.", "Das ist Shadowing: Das zweite `let` legt eine neue Variable an – dafür braucht es kein `mut`.", "Eso es shadowing: el segundo `let` crea una variable nueva, sin necesidad de `mut`."),
        ),
    }
}

fn scope_wert(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let a = r.zahl(2, 9);
    let k = r.zahl(2, 5);
    let (z1, z2) = tr!(l, ("line 1", "line 2"), ("Zeile 1", "Zeile 2"), ("línea 1", "línea 2"));
    let code = format!("let a = {a};\n{{\n    let a = a * {k};\n    println!(\"{{a}}\"); // {z1}\n}}\nprintln!(\"{{a}}\");     // {z2}");
    if r.bool() {
        eingabe(
            tr!(l, "What does **line 2** print?", "Was gibt **Zeile 2** aus?", "¿Qué imprime la **línea 2**?"),
            c(code),
            a.to_string(),
            tr!(l, "The inner `a` only exists inside the block. Afterwards the outer `a` is visible again.", "Das innere `a` existiert nur im Block. Danach ist wieder das äußere `a` sichtbar.", "La `a` interior solo existe dentro del bloque. Después vuelve a verse la `a` exterior."),
        )
    } else {
        eingabe(
            tr!(l, "What does **line 1** print?", "Was gibt **Zeile 1** aus?", "¿Qué imprime la **línea 1**?"),
            c(code),
            (a * k).to_string(),
            tr!(l, "Inside the block the new `a` shadows the outer one.", "Im Block verdeckt das neue `a` das äußere.", "Dentro del bloque, la nueva `a` oculta a la exterior."),
        )
    }
}

fn fehler_mut(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let v = *r.wahl(vars(l));
    let faktor = tr!(l, "factor", "faktor", "factor");
    let mut zeilen = vec![format!("let {v} = 5;"), format!("let {faktor} = 2;"), format!("{v} = {v} * {faktor};"), format!("println!(\"{{{v}}}\");")];
    let mut falsch = 2;
    if r.bool() {
        zeilen.insert(0, "let name = \"Ferris\";".into());
        falsch += 1;
    }
    fehler(
        tr!(l, "Which line makes the compiler complain?", "Welche Zeile lässt den Compiler meckern?", "¿Qué línea hace protestar al compilador?"),
        zeilen,
        falsch,
        tr!(l, format!("`{v}` isn't declared with `mut` and can't be changed."), format!("`{v}` ist nicht mit `mut` deklariert und darf nicht verändert werden."), format!("`{v}` no está declarada con `mut` y no se puede cambiar.")),
    )
}

fn luecke_mut(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let v = *r.wahl(vars(l));
    luecke(
        r,
        tr!(l, "Fill the gap so the code compiles.", "Fülle die Lücke, damit der Code kompiliert.", "Rellena el hueco para que el código compile."),
        format!("let ___ {v} = 0;\n{v} += 10;"),
        "mut",
        &["const", "var", "static"],
        tr!(l, "Mutable variables need `mut`.", "Veränderbare Variablen brauchen `mut`.", "Las variables mutables necesitan `mut`."),
    )
}

fn bausteine_let(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let v = *r.wahl(vars(l));
    let w = r.zahl(1, 99);
    bausteine(
        tr!(l, format!("Create a **mutable** variable `{v}` with the value {w}."), format!("Lege eine **veränderbare** Variable `{v}` mit dem Wert {w} an."), format!("Crea una variable **mutable** `{v}` con el valor {w}.")),
        vec!["let".into(), "mut".into(), v.into(), "=".into(), w.to_string(), ";".into()],
        &["var", "const", ":", "=="],
        "`let mut name = …;`",
    )
}

fn konstante(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    auswahl(
        r,
        tr!(l, "Which constant is written correctly?", "Welche Konstante ist korrekt geschrieben?", "¿Qué constante está bien escrita?"),
        None,
        "`const MAX: u32 = 100;`",
        vs(&["`const max = 100;`", "`const MAX = 100;`", "`let const MAX: u32 = 100;`", "`constant MAX: u32 = 100;`"]),
        tr!(l, "Constants: keyword `const`, name in CAPITALS and always with a type.", "Konstanten: Schlüsselwort `const`, Name in GROSSBUCHSTABEN und immer mit Typ.", "Constantes: palabra clave `const`, nombre en MAYÚSCULAS y siempre con tipo."),
    )
}

fn variablen_wf(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    wf_pool(
        r,
        tr!(
            l,
            &[
                ("Variables in Rust are immutable by default.", true, "Only `mut` makes them mutable."),
                ("With shadowing a variable can get a different type.", true, "Because it's a brand-new variable."),
                ("`mut` lets you change the type of a variable.", false, "`mut` only allows new values of the same type."),
                ("Constants always need a type annotation.", true, "For example `const MAX: u32 = 10;`."),
                ("A variable lives on after the closing `}` of its block.", false, "It disappears at the end of the block (scope)."),
                ("`1_000_000` is a valid number in Rust.", true, "Underscores are just for readability."),
            ],
            &[
                ("Variablen sind in Rust standardmäßig unveränderlich.", true, "Erst `mut` macht sie veränderbar."),
                ("Mit Shadowing kann eine Variable einen anderen Typ bekommen.", true, "Weil es eine ganz neue Variable ist."),
                ("Mit `mut` darf man den Typ einer Variable ändern.", false, "`mut` erlaubt nur neue Werte desselben Typs."),
                ("Konstanten brauchen immer eine Typangabe.", true, "Zum Beispiel `const MAX: u32 = 10;`."),
                ("Eine Variable lebt auch nach der schließenden `}` ihres Blocks weiter.", false, "Am Ende des Blocks (Scope) verschwindet sie."),
                ("`1_000_000` ist eine gültige Zahl in Rust.", true, "Unterstriche dienen nur der Lesbarkeit."),
            ],
            &[
                ("En Rust, las variables son inmutables por defecto.", true, "Solo `mut` las hace mutables."),
                ("Con shadowing una variable puede tener otro tipo.", true, "Porque es una variable completamente nueva."),
                ("`mut` permite cambiar el tipo de una variable.", false, "`mut` solo permite valores nuevos del mismo tipo."),
                ("Las constantes siempre necesitan un tipo.", true, "Por ejemplo `const MAX: u32 = 10;`."),
                ("Una variable sigue viva después de la `}` de su bloque.", false, "Desaparece al final del bloque (scope)."),
                ("`1_000_000` es un número válido en Rust.", true, "Los guiones bajos solo ayudan a leer."),
            ]
        ),
    )
}

fn variablen_paare(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    paare(
        r,
        tr!(l, "Match them up.", "Ordne zu.", "Relaciona."),
        &[
            ("let", tr!(l, "immutable variable", "unveränderliche Variable", "variable inmutable")),
            ("let mut", tr!(l, "mutable variable", "veränderbare Variable", "variable mutable")),
            ("const", tr!(l, "constant with a type", "Konstante mit Typ", "constante con tipo")),
            ("Shadowing", tr!(l, "new variable, same name", "neue Variable, gleicher Name", "variable nueva, mismo nombre")),
            ("Scope", tr!(l, "visibility area { }", "Gültigkeitsbereich { }", "ámbito de validez { }")),
        ],
        4,
    )
}

// ─────────────────────────────── World 3: Numbers & types ───────────────────────────────

pub static W_ZAHLEN: &[Gen] = &[
    division, modulo, typ_grenze, wrapping, checked, saturating, bin_dez, dez_bin, hex, cast, typ_fuer_wert,
    int_oder_float, mischen_wf, tupel_array, shift, typen_paare, bytes,
];

fn division(r: &mut Rng, s: u8, l: Sprache) -> Uebung {
    let (lo, hi) = nach_stufe(s, (7, 30), (20, 99), (100, 999));
    let a = r.zahl(lo, hi);
    let b = r.zahl(2, 9);
    let (q, m) = (a / b, a % b);
    eingabe(
        tr!(l, "What is the value of `x`?", "Welchen Wert hat `x`?", "¿Qué valor tiene `x`?"),
        c(format!("let x: i32 = {a} / {b};")),
        q.to_string(),
        tr!(l, format!("Integer division cuts off: {a} / {b} = {q} remainder {m}."), format!("Ganzzahl-Division schneidet ab: {a} / {b} = {q} Rest {m}."), format!("La división entera trunca: {a} / {b} = {q}, resto {m}.")),
    )
}

fn modulo(r: &mut Rng, s: u8, l: Sprache) -> Uebung {
    let (lo, hi) = nach_stufe(s, (7, 30), (20, 99), (100, 999));
    let a = r.zahl(lo, hi);
    let b = r.zahl(2, 9);
    let v = tr!(l, "rest", "rest", "resto");
    eingabe(tr!(l, format!("What is the value of `{v}`?"), format!("Welchen Wert hat `{v}`?"), format!("¿Qué valor tiene `{v}`?")), c(format!("let {v} = {a} % {b};")), (a % b).to_string(), format!("{a} = {} × {b} + {}", a / b, a % b))
}

fn typ_grenze(r: &mut Rng, s: u8, l: Sprache) -> Uebung {
    let typen: &[&str] = match s {
        1 => &["u8", "i8"],
        2 => &["u8", "i8", "u16", "i16"],
        _ => &["u16", "i16", "u32", "i32"],
    };
    let t = *r.wahl(typen);
    let max = r.bool() || t.starts_with('u');
    let wert: i64 = match (t, max) {
        ("u8", true) => u8::MAX.into(),
        ("i8", true) => i8::MAX.into(),
        ("i8", false) => i8::MIN.into(),
        ("u16", true) => u16::MAX.into(),
        ("i16", true) => i16::MAX.into(),
        ("i16", false) => i16::MIN.into(),
        ("u32", true) => u32::MAX.into(),
        ("i32", true) => i32::MAX.into(),
        _ => i32::MIN.into(),
    };
    let was = if max { "MAX" } else { "MIN" };
    let bits = &t[1..];
    eingabe(
        tr!(l, format!("What is `{t}::{was}`?"), format!("Was ist `{t}::{was}`?"), format!("¿Cuánto es `{t}::{was}`?")),
        None,
        wert.to_string(),
        if t.starts_with('u') {
            tr!(l, format!("A {t} has {bits} bits: 0 to 2^{bits} − 1."), format!("Ein {t} hat {bits} Bits: 0 bis 2^{bits} − 1."), format!("Un {t} tiene {bits} bits: de 0 a 2^{bits} − 1."))
        } else {
            tr!(l, format!("An {t} ranges from −2^({bits}−1) to 2^({bits}−1) − 1."), format!("Ein {t} reicht von −2^({bits}−1) bis 2^({bits}−1) − 1."), format!("Un {t} va de −2^({bits}−1) a 2^({bits}−1) − 1."))
        },
    )
}

fn programm_ausgabe(l: Sprache) -> &'static str {
    tr!(l, "What does the program print?", "Was gibt das Programm aus?", "¿Qué imprime el programa?")
}

fn wrapping(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let x = r.zahl(200, 255) as u8;
    let y = r.zahl(10, 120) as u8;
    if r.bool() {
        let summe = u16::from(x) + u16::from(y);
        eingabe(
            programm_ausgabe(l),
            c(format!("let x: u8 = {x};\nprintln!(\"{{}}\", x.wrapping_add({y}));")),
            x.wrapping_add(y).to_string(),
            tr!(l, format!("{x} + {y} = {summe}. That doesn't fit in a u8, so − 256."), format!("{x} + {y} = {summe}. Das passt nicht in u8, also − 256."), format!("{x} + {y} = {summe}. No cabe en un u8, así que − 256.")),
        )
    } else {
        let a = r.zahl(0, 40) as u8;
        let b = r.zahl(41, 120) as u8;
        eingabe(
            programm_ausgabe(l),
            c(format!("let x: u8 = {a};\nprintln!(\"{{}}\", x.wrapping_sub({b}));")),
            a.wrapping_sub(b).to_string(),
            tr!(l, format!("{a} − {b} would be negative. A u8 wraps around: 256 + ({a} − {b})."), format!("{a} − {b} wäre negativ. Bei u8 läuft es um: 256 + ({a} − {b})."), format!("{a} − {b} sería negativo. Un u8 da la vuelta: 256 + ({a} − {b}).")),
        )
    }
}

fn welcher_ausdruck(l: Sprache) -> &'static str {
    tr!(l, "What does the expression evaluate to?", "Was ergibt der Ausdruck?", "¿Qué da la expresión?")
}

fn checked(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let x = r.zahl(150, 250) as u8;
    let y = r.zahl(1, 100) as u8;
    let richtig = x.checked_add(y).map_or_else(|| "`None`".to_string(), |z| format!("`Some({z})`"));
    let summe = u16::from(x) + u16::from(y);
    auswahl(
        r,
        welcher_ausdruck(l),
        c(format!("let x: u8 = {x};\nx.checked_add({y})")),
        richtig,
        vec!["`None`".into(), format!("`Some({summe})`"), format!("`Some({})`", x.wrapping_add(y)), "`Some(255)`".into()],
        tr!(l, "`checked_add` returns `None` if the result doesn't fit the type – otherwise `Some(result)`.", "`checked_add` gibt `None` zurück, wenn das Ergebnis nicht in den Typ passt – sonst `Some(ergebnis)`.", "`checked_add` devuelve `None` si el resultado no cabe en el tipo; si no, `Some(resultado)`."),
    )
}

fn saturating(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let x = r.zahl(100, 250) as u8;
    let y = r.zahl(10, 200) as u8;
    if r.bool() {
        eingabe(welcher_ausdruck(l), c(format!("{x}u8.saturating_add({y})")), x.saturating_add(y).to_string(), tr!(l, "`saturating_add` stops at the maximum (255) instead of overflowing.", "`saturating_add` bleibt beim Maximum (255) stehen, statt überzulaufen.", "`saturating_add` se queda en el máximo (255) en lugar de desbordarse."))
    } else {
        let a = r.zahl(0, 100) as u8;
        eingabe(welcher_ausdruck(l), c(format!("{a}u8.saturating_sub({y})")), a.saturating_sub(y).to_string(), tr!(l, "`saturating_sub` stops at 0 instead of going negative.", "`saturating_sub` bleibt bei 0 stehen, statt negativ zu werden.", "`saturating_sub` se queda en 0 en lugar de volverse negativo."))
    }
}

fn bin_dez(r: &mut Rng, s: u8, l: Sprache) -> Uebung {
    let (lo, hi) = nach_stufe(s, (1, 15), (16, 127), (128, 255));
    let x = r.zahl(lo, hi) as u8;
    let bits = if s == 1 { format!("0b{x:04b}") } else { format!("0b{:04b}_{:04b}", x >> 4, x & 0xF) };
    let summe: Vec<String> = (0..8).rev().filter(|b| x >> b & 1 == 1).map(|b| (1 << b).to_string()).collect();
    eingabe(
        tr!(l, "What is the decimal value of this binary number?", "Welchen Dezimalwert hat diese Binärzahl?", "¿Qué valor decimal tiene este número binario?"),
        c(format!("let b: u8 = {bits};")),
        x.to_string(),
        format!("{} = {x}", summe.join(" + ")),
    )
}

fn dez_bin(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let x = r.zahl(5, 250) as u8;
    let falsche: Vec<String> = [x ^ 1, x ^ 4, x ^ 16, x ^ 64, x.rotate_left(1)].iter().map(|v| format!("`0b{v:08b}`")).collect();
    auswahl(
        r,
        tr!(l, format!("Which binary number is **{x}**?"), format!("Welche Binärzahl ist **{x}**?"), format!("¿Qué número binario es **{x}**?")),
        None,
        format!("`0b{x:08b}`"),
        falsche,
        tr!(l, "Place values from the left: 128, 64, 32, 16, 8, 4, 2, 1.", "Stellenwerte von links: 128, 64, 32, 16, 8, 4, 2, 1.", "Valores posicionales desde la izquierda: 128, 64, 32, 16, 8, 4, 2, 1."),
    )
}

fn hex(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let x = r.zahl(16, 255);
    eingabe(
        tr!(l, "What is the decimal value of this hexadecimal number?", "Welchen Dezimalwert hat die Hexadezimalzahl?", "¿Qué valor decimal tiene este número hexadecimal?"),
        c(format!("let h = 0x{x:X};")),
        x.to_string(),
        format!("0x{x:X} = {} × 16 + {} = {x}", x / 16, x % 16),
    )
}

fn cast(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let frage = tr!(l, "What does the conversion give?", "Welchen Wert ergibt die Umwandlung?", "¿Qué valor da la conversión?");
    match r.idx(3) {
        0 => {
            let v = r.zahl(256, 700) as i32;
            let (rest, ab) = (v as u8, 256 * (v / 256));
            eingabe(frage, c(format!("{v}_i32 as u8")), rest.to_string(), tr!(l, format!("`as u8` keeps only the lowest 8 bits: {v} − {ab} = {rest}."), format!("`as u8` behält nur die unteren 8 Bits: {v} − {ab} = {rest}."), format!("`as u8` conserva solo los 8 bits inferiores: {v} − {ab} = {rest}.")))
        }
        1 => {
            let k = r.zahl(1, 50) as i32;
            let w = (-k) as u8;
            eingabe(frage, c(format!("-{k}_i32 as u8")), w.to_string(), tr!(l, format!("Negative numbers are reinterpreted: 256 − {k} = {w}."), format!("Negative Zahlen werden umgedeutet: 256 − {k} = {w}."), format!("Los números negativos se reinterpretan: 256 − {k} = {w}.")))
        }
        _ => {
            let f = r.zahl(10, 99) as f64 + r.zahl(1, 99) as f64 / 100.0;
            eingabe(frage, c(format!("{f}_f64 as i32")), (f as i32).to_string(), tr!(l, "`as i32` cuts off the decimals (no rounding!).", "`as i32` schneidet die Nachkommastellen ab (kein Runden!).", "`as i32` corta los decimales (¡no redondea!)."))
        }
    }
}

fn typ_fuer_wert(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let (wert, typ) = *r.wahl(&[
        (200_i64, "u8"),
        (255, "u8"),
        (256, "u16"),
        (40_000, "u16"),
        (70_000, "u32"),
        (3_000_000_000, "u32"),
        (5_000_000_000, "u64"),
        (-5, "i8"),
        (-200, "i16"),
        (-40_000, "i32"),
    ]);
    auswahl(
        r,
        tr!(l, format!("What is the **smallest** integer type that `{wert}` fits into?"), format!("Welches ist der **kleinste** Ganzzahl-Typ, in den `{wert}` passt?"), format!("¿Cuál es el tipo entero **más pequeño** en el que cabe `{wert}`?")),
        None,
        format!("`{typ}`"),
        vs(&["`u8`", "`i8`", "`u16`", "`i16`", "`u32`", "`i32`", "`u64`"]),
        tr!(
            l,
            "u8: up to 255, u16: up to 65 535, u32: up to ~4.3 billion. Negative numbers need an i type (i8: from −128, i16: from −32 768).",
            "u8: bis 255, u16: bis 65 535, u32: bis ~4,3 Mrd. Negative Zahlen brauchen einen i-Typ (i8: ab −128, i16: ab −32 768).",
            "u8: hasta 255, u16: hasta 65 535, u32: hasta ~4300 millones. Los negativos necesitan un tipo i (i8: desde −128, i16: desde −32 768)."
        ),
    )
}

fn int_oder_float(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let a = r.zahl(5, 30) * 2 + 1;
    let b = 2;
    let fehler = tr!(l, "Compiler error", "Compiler-Fehler", "Error de compilación");
    if r.bool() {
        auswahl(
            r,
            tr!(l, format!("What does `{a} / {b}` give?"), format!("Was ergibt `{a} / {b}`?"), format!("¿Qué da `{a} / {b}`?")),
            None,
            format!("`{}`", a / b),
            vec![format!("`{}.5`", a / b), format!("`{}`", a / b + 1), fehler.into()],
            tr!(l, "Two integers → integer division, the remainder is dropped.", "Zwei Ganzzahlen → Ganzzahl-Division, der Rest fällt weg.", "Dos enteros → división entera, el resto se pierde."),
        )
    } else {
        auswahl(
            r,
            tr!(l, format!("What does `{a}.0 / {b}.0` give?"), format!("Was ergibt `{a}.0 / {b}.0`?"), format!("¿Qué da `{a}.0 / {b}.0`?")),
            None,
            format!("`{}.5`", a / b),
            vec![format!("`{}`", a / b), format!("`{}.0`", a / b + 1), fehler.into()],
            tr!(l, "With floating-point numbers (`f64`) the fractional part is kept.", "Mit Kommazahlen (`f64`) bleibt der Nachkommateil erhalten.", "Con números decimales (`f64`) se conserva la parte decimal."),
        )
    }
}

fn mischen_wf(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let a = r.zahl(1, 9);
    if r.bool() {
        wf(kompiliert(l), c(format!("let a: i32 = {a};\nlet b: f64 = 2.5;\nlet c = a + b;")), false, tr!(l, "You can't add i32 and f64 directly. Rust never converts silently.", "i32 und f64 kann man nicht direkt addieren. Rust wandelt nie heimlich um.", "No se pueden sumar i32 y f64 directamente. Rust nunca convierte en silencio."))
    } else {
        wf(kompiliert(l), c(format!("let a: i32 = {a};\nlet b: f64 = 2.5;\nlet c = a as f64 + b;")), true, tr!(l, "With `as f64`, `a` is converted explicitly – that's allowed.", "Mit `as f64` wird `a` ausdrücklich umgewandelt – das ist erlaubt.", "Con `as f64`, `a` se convierte de forma explícita: eso está permitido."))
    }
}

fn tupel_array(r: &mut Rng, s: u8, l: Sprache) -> Uebung {
    let v: Vec<i64> = (0..4).map(|_| r.zahl(1, 50)).collect();
    if r.bool() {
        let i = r.idx(3);
        eingabe(
            tr!(l, format!("What is the value of `t.{i}`?"), format!("Welchen Wert hat `t.{i}`?"), format!("¿Qué valor tiene `t.{i}`?")),
            c(format!("let t = ({}, {}, {});", v[0], v[1], v[2])),
            v[i].to_string(),
            tr!(l, "Tuple fields count from 0: `.0`, `.1`, `.2`.", "Tupel-Felder zählen ab 0: `.0`, `.1`, `.2`.", "Los campos de una tupla se cuentan desde 0: `.0`, `.1`, `.2`."),
        )
    } else {
        let i = r.idx(4);
        let extra = if s >= 2 { " + arr[0]" } else { "" };
        let ergebnis = v[i] + if s >= 2 { v[0] } else { 0 };
        eingabe(
            tr!(l, format!("What does `arr[{i}]{extra}` give?"), format!("Was ergibt `arr[{i}]{extra}`?"), format!("¿Qué da `arr[{i}]{extra}`?")),
            c(format!("let arr = [{}, {}, {}, {}];", v[0], v[1], v[2], v[3])),
            ergebnis.to_string(),
            tr!(l, "Array indices start at 0.", "Array-Indizes beginnen bei 0.", "Los índices de un array empiezan en 0."),
        )
    }
}

fn shift(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let x = r.zahl(1, 20) as u32;
    let n = r.zahl(1, 4) as u32;
    let p = 1 << n;
    if r.bool() {
        eingabe(welcher_ausdruck(l), c(format!("{x}_u32 << {n}")), (x << n).to_string(), tr!(l, format!("Shifting left by {n} bits = times 2^{n} = times {p}."), format!("Linksschieben um {n} Bits = mal 2^{n} = mal {p}."), format!("Desplazar {n} bits a la izquierda = por 2^{n} = por {p}.")))
    } else {
        let y = x * 16 + r.zahl(0, 15) as u32;
        eingabe(welcher_ausdruck(l), c(format!("{y}_u32 >> {n}")), (y >> n).to_string(), tr!(l, format!("Shifting right by {n} bits = divided by {p} (rounded down)."), format!("Rechtsschieben um {n} Bits = geteilt durch {p} (abgerundet)."), format!("Desplazar {n} bits a la derecha = dividir entre {p} (redondeando hacia abajo).")))
    }
}

fn typen_paare(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    paare(
        r,
        tr!(l, "Which type fits?", "Welcher Typ passt?", "¿Qué tipo encaja?"),
        &[
            ("u8", tr!(l, "0 to 255", "0 bis 255", "de 0 a 255")),
            ("i8", tr!(l, "−128 to 127", "−128 bis 127", "de −128 a 127")),
            ("f64", tr!(l, "decimal number (default)", "Kommazahl (Standard)", "número decimal (por defecto)")),
            ("bool", tr!(l, "true or false", "true oder false", "true o false")),
            ("char", tr!(l, "one Unicode character", "ein Unicode-Zeichen", "un carácter Unicode")),
            ("usize", tr!(l, "index or length", "Index oder Länge", "índice o longitud")),
            ("i32", tr!(l, "default integer", "Standard-Ganzzahl", "entero por defecto")),
        ],
        4,
    )
}

fn bytes(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let (t, b) = *r.wahl(&[("u8", 1), ("i16", 2), ("char", 4), ("i32", 4), ("f32", 4), ("u64", 8), ("f64", 8), ("i128", 16), ("bool", 1)]);
    let bit = b * 8;
    eingabe(
        tr!(l, format!("How many **bytes** does a `{t}` take?"), format!("Wie viele **Bytes** belegt ein `{t}`?"), format!("¿Cuántos **bytes** ocupa un `{t}`?")),
        None,
        b.to_string(),
        tr!(l, format!("`{t}` takes {b} byte(s) = {bit} bits."), format!("`{t}` belegt {b} Byte(s) = {bit} Bit."), format!("`{t}` ocupa {b} byte(s) = {bit} bits.")),
    )
}

// ─────────────────────────────── World 4: Functions ───────────────────────────────

pub static W_FUNKTIONEN: &[Gen] = &[
    fn_verschachtelt, fn_zwei_parameter, luecke_pfeil, semikolon_wf, block_ausdruck, bausteine_fn, fehler_parameter,
    reihenfolge_main, funktionen_wf, rueckgabe,
];

fn fn_verschachtelt(r: &mut Rng, s: u8, l: Sprache) -> Uebung {
    let a = r.zahl(2, 4);
    let b = r.zahl(0, 5);
    let x = r.zahl(1, 5);
    let f = |v: i64| v * a + b;
    let (ausdruck, wert) = if s == 1 { (format!("f({x})"), f(x)) } else { (format!("f(f({x}))"), f(f(x))) };
    eingabe(
        tr!(l, format!("What does `{ausdruck}` return?"), format!("Welchen Wert liefert `{ausdruck}`?"), format!("¿Qué devuelve `{ausdruck}`?")),
        c(format!("fn f(x: i32) -> i32 {{\n    x * {a} + {b}\n}}")),
        wert.to_string(),
        tr!(l, "Work from the inside out: first the inner `f`, then the outer one.", "Von innen nach außen rechnen: erst das innere `f`, dann das äußere.", "Calcula de dentro hacia fuera: primero la `f` interior y luego la exterior."),
    )
}

fn fn_zwei_parameter(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let (x, y) = (r.zahl(2, 9), r.zahl(2, 9));
    let (koerper, wert) = match r.idx(3) {
        0 => ("a * b - a", x * y - x),
        1 => ("(a + b) * 2", (x + y) * 2),
        _ => ("a * a + b", x * x + y),
    };
    eingabe(
        tr!(l, format!("What does `g({x}, {y})` return?"), format!("Was liefert `g({x}, {y})`?"), format!("¿Qué devuelve `g({x}, {y})`?")),
        c(format!("fn g(a: i32, b: i32) -> i32 {{\n    {koerper}\n}}")),
        wert.to_string(),
        tr!(l, format!("Plug in a = {x}, b = {y}."), format!("a = {x}, b = {y} einsetzen."), format!("Sustituye a = {x}, b = {y}.")),
    )
}

fn luecke_pfeil(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    luecke(
        r,
        tr!(l, "What goes in the gap?", "Was gehört in die Lücke?", "¿Qué va en el hueco?"),
        "fn square(x: i32) ___ i32 {\n    x * x\n}",
        "->",
        &[":", "=>", "="],
        tr!(l, "The return type comes after the arrow `->`.", "Der Rückgabetyp steht nach dem Pfeil `->`.", "El tipo de retorno va después de la flecha `->`."),
    )
}

fn semikolon_wf(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let n = r.zahl(1, 99);
    let frage = tr!(l, "Does this function compile?", "Kompiliert diese Funktion?", "¿Compila esta función?");
    match r.idx(3) {
        0 => wf(frage, c(format!("fn number() -> i32 {{\n    {n};\n}}")), false, tr!(l, "With `;` it's a statement – the function then returns `()` instead of `i32`.", "Mit `;` ist es eine Anweisung – die Funktion gibt dann `()` statt `i32` zurück.", "Con `;` es una instrucción: la función devuelve `()` en lugar de `i32`.")),
        1 => wf(frage, c(format!("fn number() -> i32 {{\n    {n}\n}}")), true, tr!(l, "Without a semicolon, the last line is the return value.", "Ohne Semikolon ist die letzte Zeile der Rückgabewert.", "Sin punto y coma, la última línea es el valor de retorno.")),
        _ => wf(frage, c(format!("fn number() -> i32 {{\n    return {n};\n}}")), true, tr!(l, "With `return`, a semicolon after it is fine.", "Mit `return` darf ein Semikolon dahinter stehen.", "Con `return` puede ir un punto y coma detrás.")),
    }
}

fn block_ausdruck(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let (a, b, k) = (r.zahl(1, 9), r.zahl(2, 5), r.zahl(1, 9));
    eingabe(
        tr!(l, "What is the value of `y`?", "Welchen Wert hat `y`?", "¿Qué valor tiene `y`?"),
        c(format!("let y = {{\n    let x = {a};\n    x * {b} + {k}\n}};")),
        (a * b + k).to_string(),
        tr!(l, "A block is an expression: its value is the last line without a semicolon.", "Ein Block ist ein Ausdruck: Sein Wert ist die letzte Zeile ohne Semikolon.", "Un bloque es una expresión: su valor es la última línea sin punto y coma."),
    )
}

fn bausteine_fn(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let name = *r.wahl(&["double", "square", "halve", "plus_one"]);
    bausteine(
        tr!(l, format!("Build the function header: `{name}` takes an `i32` and returns an `i32`."), format!("Baue den Funktionskopf: `{name}` nimmt ein `i32` und gibt ein `i32` zurück."), format!("Construye la cabecera: `{name}` recibe un `i32` y devuelve un `i32`.")),
        vec!["fn".into(), name.into(), "(x: i32)".into(), "->".into(), "i32".into(), "{".into()],
        &["=>", "func", "return", ":"],
        tr!(l, "`fn name(parameter: Type) -> ReturnType {`", "`fn name(parameter: Typ) -> Rückgabetyp {`", "`fn nombre(parametro: Tipo) -> TipoRetorno {`"),
    )
}

fn fehler_parameter(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let frage = tr!(l, "Where is the bug?", "Wo ist der Fehler?", "¿Dónde está el error?");
    if r.bool() {
        fehler(frage, vs(&["fn sum(a, b: i32) -> i32 {", "    a + b", "}"]), 0, tr!(l, "Every parameter needs a type: `a: i32, b: i32`.", "Jeder Parameter braucht einen Typ: `a: i32, b: i32`.", "Cada parámetro necesita un tipo: `a: i32, b: i32`."))
    } else {
        fehler(
            frage,
            vs(&["fn sum(a: i32, b: i32) -> i32 {", "    a + b", "}", "fn main() {", "    let s = sum(1);", "}"]),
            4,
            tr!(l, "`sum` needs two arguments, only one is passed here (E0061).", "`sum` braucht zwei Argumente, hier wird nur eines übergeben (E0061).", "`sum` necesita dos argumentos y aquí solo se pasa uno (E0061)."),
        )
    }
}

fn reihenfolge_main(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let a = r.zahl(2, 9);
    reihenfolge(
        tr!(l, "Put the lines in an order that compiles.", "Bring die Zeilen in eine Reihenfolge, die kompiliert.", "Ordena las líneas para que compilen."),
        vec!["fn main() {".into(), format!("    let a = {a};"), "    let b = double(a);".into(), "    println!(\"{b}\");".into(), "}".into()],
        tr!(l, "Variables must exist before you use them.", "Variablen müssen angelegt sein, bevor man sie benutzt.", "Las variables deben existir antes de usarlas."),
    )
}

fn funktionen_wf(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    wf_pool(
        r,
        tr!(
            l,
            &[
                ("Parameters in Rust always need a type.", true, "Rust doesn't guess parameter types."),
                ("The last line without `;` is the return value.", true, "It's an expression."),
                ("Functions must appear before `main` in the code.", false, "The order doesn't matter."),
                ("Function names are written in snake_case.", true, "For example `compute_sum`."),
                ("`return` is forbidden in Rust.", false, "It's allowed, but mostly used for early returns."),
                ("A function without `->` returns `()`.", true, "`()` is called unit – “nothing”."),
            ],
            &[
                ("Parameter brauchen in Rust immer einen Typ.", true, "Rust errät Parametertypen nicht."),
                ("Die letzte Zeile ohne `;` ist der Rückgabewert.", true, "Sie ist ein Ausdruck."),
                ("Funktionen müssen im Code vor `main` stehen.", false, "Die Reihenfolge ist egal."),
                ("Funktionsnamen schreibt man in snake_case.", true, "Zum Beispiel `berechne_summe`."),
                ("`return` ist in Rust verboten.", false, "Es ist erlaubt, wird aber meist nur für frühe Rücksprünge benutzt."),
                ("Eine Funktion ohne `->` gibt `()` zurück.", true, "`()` heißt Unit – also „nichts“."),
            ],
            &[
                ("En Rust, los parámetros siempre necesitan un tipo.", true, "Rust no adivina los tipos de los parámetros."),
                ("La última línea sin `;` es el valor de retorno.", true, "Es una expresión."),
                ("Las funciones deben ir antes de `main` en el código.", false, "El orden da igual."),
                ("Los nombres de funciones se escriben en snake_case.", true, "Por ejemplo `calcular_suma`."),
                ("`return` está prohibido en Rust.", false, "Está permitido, pero se usa sobre todo para salir antes."),
                ("Una función sin `->` devuelve `()`.", true, "`()` se llama unit: «nada»."),
            ]
        ),
    )
}

fn rueckgabe(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    auswahl(
        r,
        tr!(l, "What return type does this function have?", "Welchen Rückgabetyp hat diese Funktion?", "¿Qué tipo de retorno tiene esta función?"),
        c("fn greet() {\n    println!(\"Hi!\");\n}"),
        tr!(l, "`()` – nothing", "`()` – nichts", "`()` – nada"),
        vs(&["`String`", "`&str`", "`i32`", "`bool`"]),
        tr!(l, "Without `->` the return type is `()`, the empty type.", "Ohne `->` ist der Rückgabetyp `()`, der leere Typ.", "Sin `->`, el tipo de retorno es `()`, el tipo vacío."),
    )
}
