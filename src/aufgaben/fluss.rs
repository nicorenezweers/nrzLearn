//! Generators for worlds 5–6: decisions and loops.

use super::*;

// ─────────────────────────────── World 5: if ───────────────────────────────

pub static W_IF: &[Gen] = &[
    collatz, bool_ausdruck, if_kette, if_bool_fehler, luecke_vergleich, if_max_min, if_wf, operatoren_paare, schaltjahr,
    bausteine_if,
];

fn wert_von(l: Sprache, v: &str) -> String {
    tr!(l, format!("What is the value of `{v}`?"), format!("Welchen Wert hat `{v}`?"), format!("¿Qué valor tiene `{v}`?"))
}

fn collatz(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let a = r.zahl(3, 60);
    let b = if a % 2 == 0 { a / 2 } else { 3 * a + 1 };
    let gerade = if a % 2 == 0 { tr!(l, "even", "gerade", "par") } else { tr!(l, "odd", "ungerade", "impar") };
    eingabe(
        wert_von(l, "b"),
        c(format!("let a = {a};\nlet b = if a % 2 == 0 {{ a / 2 }} else {{ 3 * a + 1 }};")),
        b.to_string(),
        tr!(l, format!("{a} is {gerade} → {b}."), format!("{a} ist {gerade} → {b}."), format!("{a} es {gerade} → {b}.")),
    )
}

fn bool_ausdruck(r: &mut Rng, s: u8, l: Sprache) -> Uebung {
    let (a, b) = (r.zahl(1, 9), r.zahl(1, 9));
    let (x, k) = (r.zahl(10, 30), r.zahl(2, 5));
    let m = r.zahl(0, k - 1);
    let t1 = a > b;
    let t2 = x % k == m;
    let (ausdruck, wert) = match (s, r.idx(3)) {
        (1, _) => (format!("{a} > {b} && {x} % {k} == {m}"), t1 && t2),
        (_, 0) => (format!("!({a} > {b}) || {x} % {k} == {m}"), !t1 || t2),
        (_, 1) => (format!("({a} > {b}) != ({x} % {k} == {m})"), t1 != t2),
        _ => (format!("{a} > {b} || !({x} % {k} == {m}) && {a} == {b}"), t1 || (!t2 && a == b)),
    };
    let rest = x % k;
    auswahl(
        r,
        tr!(l, "What does this expression evaluate to?", "Was ergibt dieser Ausdruck?", "¿Qué da esta expresión?"),
        c(ausdruck),
        wert.to_string(),
        vec![(!wert).to_string()],
        tr!(l, format!("`&&` binds tighter than `||`. {x} % {k} = {rest}."), format!("`&&` bindet stärker als `||`. {x} % {k} = {rest}."), format!("`&&` tiene prioridad sobre `||`. {x} % {k} = {rest}.")),
    )
}

fn if_kette(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let t1 = r.zahl(5, 15);
    let t2 = t1 + r.zahl(5, 15);
    let v = r.zahl(t1 - 6, t2 + 6);
    let [kalt, mild, heiss] = tr!(l, ["cold", "pleasant", "hot"], ["kalt", "angenehm", "heiß"], ["frío", "agradable", "calor"]);
    let ergebnis = if v < t1 { kalt } else if v <= t2 { mild } else { heiss };
    auswahl(
        r,
        tr!(l, format!("What does the code print for `temp = {v}`?"), format!("Was gibt der Code für `temp = {v}` aus?"), format!("¿Qué imprime el código para `temp = {v}`?")),
        c(format!("if temp < {t1} {{\n    println!(\"{kalt}\");\n}} else if temp <= {t2} {{\n    println!(\"{mild}\");\n}} else {{\n    println!(\"{heiss}\");\n}}")),
        format!("`{ergebnis}`"),
        vec![format!("`{kalt}`"), format!("`{mild}`"), format!("`{heiss}`")],
        tr!(l, "The conditions are checked from top to bottom; the first matching branch wins.", "Die Bedingungen werden von oben nach unten geprüft, der erste passende Zweig gewinnt.", "Las condiciones se comprueban de arriba abajo; gana la primera rama que encaja."),
    )
}

fn if_bool_fehler(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let n = r.zahl(1, 9);
    let wahr = tr!(l, "true", "wahr", "verdadero");
    fehler(
        tr!(l, "Which line doesn't compile?", "Welche Zeile kompiliert nicht?", "¿Qué línea no compila?"),
        vec![format!("let n = {n};"), "if n {".into(), format!("    println!(\"{wahr}\");"), "}".into()],
        1,
        tr!(l, "The condition must be a `bool`. Correct would be e.g. `if n != 0`.", "Die Bedingung muss ein `bool` sein. Richtig wäre z.B. `if n != 0`.", "La condición debe ser un `bool`. Lo correcto sería, p. ej., `if n != 0`."),
    )
}

fn luecke_vergleich(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let paare: [(&str, &str); 6] = tr!(
        l,
        [("at least 18", ">="), ("greater than 18", ">"), ("exactly 18", "=="), ("at most 18", "<="), ("not 18", "!="), ("less than 18", "<")],
        [("mindestens 18", ">="), ("größer als 18", ">"), ("genau 18", "=="), ("höchstens 18", "<="), ("nicht 18", "!="), ("kleiner als 18", "<")],
        [("al menos 18", ">="), ("mayor que 18", ">"), ("exactamente 18", "=="), ("como mucho 18", "<="), ("distinta de 18", "!="), ("menor que 18", "<")]
    );
    let (text, op) = *r.wahl(&paare);
    let alle = [">=", ">", "==", "<=", "!=", "<"];
    let mut f: Vec<&str> = alle.iter().copied().filter(|o| *o != op).collect();
    r.mischen(&mut f);
    luecke(
        r,
        tr!(l, format!("The condition should check: `age` is **{text}**."), format!("Die Bedingung soll prüfen: `age` ist **{text}**."), format!("La condición debe comprobar: `age` es **{text}**.")),
        "if age ___ 18 {\n    println!(\"ok\");\n}",
        op,
        &f[..3],
        tr!(l, format!("“{text}” is written `{op}`."), format!("„{text}“ schreibt man `{op}`."), format!("«{text}» se escribe `{op}`.")),
    )
}

fn if_max_min(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let (a, b) = (r.zahl(-20, 40), r.zahl(-20, 40));
    if r.bool() {
        eingabe(wert_von(l, "m"), c(format!("let a = {a};\nlet b = {b};\nlet m = if a > b {{ a }} else {{ b }};")), a.max(b).to_string(), tr!(l, "`m` is the larger of the two numbers.", "`m` ist die größere der beiden Zahlen.", "`m` es el mayor de los dos números."))
    } else {
        eingabe(wert_von(l, "m"), c(format!("let a = {a};\nlet b = {b};\nlet m = if a < b {{ a }} else {{ b }};")), a.min(b).to_string(), tr!(l, "`m` is the smaller of the two numbers.", "`m` ist die kleinere der beiden Zahlen.", "`m` es el menor de los dos números."))
    }
}

fn if_wf(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    wf_pool(
        r,
        tr!(
            l,
            &[
                ("The condition of an `if` in Rust must be a `bool`.", true, "Numbers don't count as true or false automatically."),
                ("Both branches of an `if` inside a `let` must have the same type.", true, "Otherwise Rust doesn't know the variable's type."),
                ("Rust has an operator `condition ? a : b`.", false, "You write `if condition { a } else { b }` instead."),
                ("`&&` means “or”.", false, "`&&` is “and”, `||` is “or”."),
                ("`!=` means “not equal”.", true, "And `==` means “equal”."),
                ("With `else if`, every matching branch runs.", false, "Only the first matching branch runs."),
            ],
            &[
                ("Die Bedingung eines `if` muss in Rust ein `bool` sein.", true, "Zahlen gelten nicht automatisch als wahr oder falsch."),
                ("Beide Zweige eines `if` in einem `let` müssen denselben Typ haben.", true, "Sonst weiß Rust nicht, welchen Typ die Variable bekommt."),
                ("Rust hat einen Operator `bedingung ? a : b`.", false, "Stattdessen schreibt man `if bedingung { a } else { b }`."),
                ("`&&` bedeutet „oder“.", false, "`&&` ist „und“, `||` ist „oder“."),
                ("`!=` bedeutet „ungleich“.", true, "Und `==` bedeutet „gleich“."),
                ("Bei `else if` werden alle Zweige ausgeführt, die passen.", false, "Nur der erste passende Zweig läuft."),
            ],
            &[
                ("En Rust, la condición de un `if` debe ser un `bool`.", true, "Los números no cuentan como verdadero o falso automáticamente."),
                ("Las dos ramas de un `if` dentro de un `let` deben tener el mismo tipo.", true, "Si no, Rust no sabe qué tipo tiene la variable."),
                ("Rust tiene un operador `condicion ? a : b`.", false, "En su lugar se escribe `if condicion { a } else { b }`."),
                ("`&&` significa «o».", false, "`&&` es «y», `||` es «o»."),
                ("`!=` significa «distinto».", true, "Y `==` significa «igual»."),
                ("Con `else if` se ejecutan todas las ramas que encajan.", false, "Solo se ejecuta la primera rama que encaja."),
            ]
        ),
    )
}

fn operatoren_paare(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    paare(
        r,
        tr!(l, "Match the operators.", "Ordne die Operatoren zu.", "Relaciona los operadores."),
        &[
            ("==", tr!(l, "equal", "gleich", "igual")),
            ("!=", tr!(l, "not equal", "ungleich", "distinto")),
            ("&&", tr!(l, "and", "und", "y")),
            ("||", tr!(l, "or", "oder", "o")),
            ("!", tr!(l, "not", "nicht", "no")),
            (">=", tr!(l, "greater or equal", "größer oder gleich", "mayor o igual")),
            ("%", tr!(l, "remainder of division", "Rest der Division", "resto de la división")),
        ],
        4,
    )
}

fn schaltjahr(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let j = *r.wahl(&[1900, 2000, 2023, 2024, 2100, 2400, 1996, 2026, 2028, 1800]);
    let ist = (j % 4 == 0 && j % 100 != 0) || j % 400 == 0;
    auswahl(
        r,
        tr!(l, format!("Is **{j}** a leap year? What does the expression give?"), format!("Ist **{j}** ein Schaltjahr? Was ergibt der Ausdruck?"), format!("¿Es **{j}** un año bisiesto? ¿Qué da la expresión?")),
        c(format!("let y = {j};\n(y % 4 == 0 && y % 100 != 0) || y % 400 == 0")),
        ist.to_string(),
        vec![(!ist).to_string()],
        format!("{j} % 4 = {}, {j} % 100 = {}, {j} % 400 = {}.", j % 4, j % 100, j % 400),
    )
}

fn bausteine_if(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let n = r.zahl(2, 9);
    bausteine(
        tr!(l, format!("Build the line: if `x` is greater than {n} …"), format!("Baue die Zeile: Wenn `x` größer als {n} ist …"), format!("Construye la línea: si `x` es mayor que {n}…")),
        vec!["if".into(), "x".into(), ">".into(), n.to_string(), "{".into()],
        &["(", "then", ":", "=>"],
        tr!(l, "Rust doesn't need parentheses around an `if` condition.", "Bei `if` braucht Rust keine Klammern um die Bedingung.", "Rust no necesita paréntesis alrededor de la condición de un `if`."),
    )
}

// ─────────────────────────────── World 6: Loops ───────────────────────────────

pub static W_SCHLEIFEN: &[Gen] = &[
    range_anzahl, range_summe, while_schritte, loop_break, rev_erste, step_by, continue_summe, fizzbuzz, reihenfolge_loop,
    luecke_range, verschachtelt, schleifen_wf, bausteine_for,
];

fn am_ende(l: Sprache, v: &str) -> String {
    tr!(l, format!("What is the value of `{v}` at the end?"), format!("Welchen Wert hat `{v}` am Ende?"), format!("¿Qué valor tiene `{v}` al final?"))
}

fn range_anzahl(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let a = r.zahl(0, 10);
    let b = a + r.zahl(2, 12);
    let inkl = r.bool();
    let (bereich, n) = if inkl { (format!("{a}..={b}"), b - a + 1) } else { (format!("{a}..{b}"), b - a) };
    eingabe(
        tr!(l, "How many times does the loop run?", "Wie oft läuft die Schleife?", "¿Cuántas veces se ejecuta el bucle?"),
        c(format!("for i in {bereich} {{\n    println!(\"{{i}}\");\n}}")),
        n.to_string(),
        if inkl {
            tr!(l, format!("`..=` includes {b}: {b} − {a} + 1 = {n}."), format!("`..=` schließt {b} ein: {b} − {a} + 1 = {n}."), format!("`..=` incluye {b}: {b} − {a} + 1 = {n}."))
        } else {
            tr!(l, format!("`..` excludes {b}: {b} − {a} = {n}."), format!("`..` schließt {b} aus: {b} − {a} = {n}."), format!("`..` excluye {b}: {b} − {a} = {n}."))
        },
    )
}

fn range_summe(r: &mut Rng, s: u8, l: Sprache) -> Uebung {
    let (a, b) = match s {
        1 => (1, r.zahl(3, 10)),
        2 => {
            let a = r.zahl(2, 10);
            (a, a + r.zahl(3, 8))
        }
        _ => (1, *r.wahl(&[50, 100, 200, 1000])),
    };
    let summe: i64 = (a..=b).sum();
    let v = tr!(l, "total", "summe", "suma");
    eingabe(
        tr!(l, "What does the program print?", "Was gibt das Programm aus?", "¿Qué imprime el programa?"),
        c(format!("let mut {v} = 0;\nfor i in {a}..={b} {{\n    {v} += i;\n}}\nprintln!(\"{{{v}}}\");")),
        summe.to_string(),
        if a == 1 {
            tr!(l, format!("Gauss formula: n × (n + 1) / 2 = {b} × {} / 2 = {summe}.", b + 1), format!("Gauß-Formel: n × (n + 1) / 2 = {b} × {} / 2 = {summe}.", b + 1), format!("Fórmula de Gauss: n × (n + 1) / 2 = {b} × {} / 2 = {summe}.", b + 1))
        } else {
            tr!(l, format!("Count × (first + last) / 2 = {} × {} / 2 = {summe}.", b - a + 1, a + b), format!("Anzahl × (erste + letzte) / 2 = {} × {} / 2 = {summe}.", b - a + 1, a + b), format!("Cantidad × (primero + último) / 2 = {} × {} / 2 = {summe}.", b - a + 1, a + b))
        },
    )
}

fn while_schritte(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let k = r.zahl(2, 4);
    let limit = *r.wahl(&[50, 100, 200, 500, 1000]);
    let mut n = 1;
    let mut schritte = 0;
    let mut weg = vec!["1".to_string()];
    while n < limit {
        n *= k;
        schritte += 1;
        weg.push(n.to_string());
    }
    let v = tr!(l, "steps", "schritte", "pasos");
    eingabe(am_ende(l, v), c(format!("let mut n = 1;\nlet mut {v} = 0;\nwhile n < {limit} {{\n    n *= {k};\n    {v} += 1;\n}}")), schritte.to_string(), format!("n: {}", weg.join(" → ")))
}

fn loop_break(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let start = r.zahl(1, 7);
    let limit = r.zahl(30, 300);
    let mut x = start;
    let wert = loop {
        x *= 2;
        if x > limit {
            break x;
        }
    };
    let v = tr!(l, "result", "ergebnis", "resultado");
    eingabe(
        wert_von(l, v),
        c(format!("let mut x = {start};\nlet {v} = loop {{\n    x *= 2;\n    if x > {limit} {{\n        break x;\n    }}\n}};")),
        wert.to_string(),
        tr!(l, "`break x` ends the loop and returns `x` – the first value above the limit.", "`break x` beendet die Schleife und gibt `x` zurück – den ersten Wert über der Grenze.", "`break x` termina el bucle y devuelve `x`: el primer valor por encima del límite."),
    )
}

fn rev_erste(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let a = r.zahl(0, 5);
    let b = a + r.zahl(3, 9);
    if r.bool() {
        eingabe(
            tr!(l, "Which number is printed **first**?", "Welche Zahl wird **zuerst** ausgegeben?", "¿Qué número se imprime **primero**?"),
            c(format!("for i in ({a}..{b}).rev() {{\n    println!(\"{{i}}\");\n}}")),
            (b - 1).to_string(),
            tr!(l, format!("`{a}..{b}` ends at {}, `.rev()` starts there.", b - 1), format!("`{a}..{b}` endet bei {}, `.rev()` beginnt dort.", b - 1), format!("`{a}..{b}` termina en {}, y `.rev()` empieza ahí.", b - 1)),
        )
    } else {
        eingabe(
            tr!(l, "Which number is printed **last**?", "Welche Zahl wird **zuletzt** ausgegeben?", "¿Qué número se imprime **al final**?"),
            c(format!("for i in ({a}..={b}).rev() {{\n    println!(\"{{i}}\");\n}}")),
            a.to_string(),
            tr!(l, "Going backwards, the loop ends at the start of the range.", "Rückwärts endet die Schleife beim Anfang des Bereichs.", "Hacia atrás, el bucle termina en el inicio del rango."),
        )
    }
}

fn step_by(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let n = r.zahl(10, 30);
    let k = r.zahl(2, 5) as usize;
    let werte: Vec<i64> = (0..=n).step_by(k).collect();
    let letzte = *werte.last().unwrap_or(&0);
    let code = c(format!("for i in (0..={n}).step_by({k}) {{\n    println!(\"{{i}}\");\n}}"));
    if r.bool() {
        eingabe(tr!(l, "How many numbers are printed?", "Wie viele Zahlen werden ausgegeben?", "¿Cuántos números se imprimen?"), code, werte.len().to_string(), format!("0, {k}, {} … {letzte}.", 2 * k))
    } else {
        eingabe(
            tr!(l, "Which number is printed last?", "Welche Zahl wird zuletzt ausgegeben?", "¿Qué número se imprime el último?"),
            code,
            letzte.to_string(),
            tr!(l, format!("In steps of {k}, the largest number ≤ {n}."), format!("In {k}er-Schritten, die größte Zahl ≤ {n}."), format!("De {k} en {k}, el mayor número ≤ {n}.")),
        )
    }
}

fn continue_summe(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let n = r.zahl(6, 15);
    let k = r.zahl(2, 4);
    let summe: i64 = (1..=n).filter(|i| i % k != 0).sum();
    let v = tr!(l, "total", "summe", "suma");
    eingabe(
        am_ende(l, v),
        c(format!("let mut {v} = 0;\nfor i in 1..={n} {{\n    if i % {k} == 0 {{\n        continue;\n    }}\n    {v} += i;\n}}")),
        summe.to_string(),
        tr!(l, format!("`continue` skips every multiple of {k}."), format!("`continue` überspringt alle Vielfachen von {k}."), format!("`continue` se salta todos los múltiplos de {k}.")),
    )
}

fn fizzbuzz(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let i = r.zahl(1, 100);
    let erg = if i % 15 == 0 {
        "FizzBuzz".to_string()
    } else if i % 3 == 0 {
        "Fizz".to_string()
    } else if i % 5 == 0 {
        "Buzz".to_string()
    } else {
        i.to_string()
    };
    eingabe(
        tr!(l, format!("FizzBuzz: what is printed for `i = {i}`?"), format!("FizzBuzz: Was wird für `i = {i}` ausgegeben?"), format!("FizzBuzz: ¿qué se imprime para `i = {i}`?")),
        c("if i % 15 == 0 {\n    println!(\"FizzBuzz\");\n} else if i % 3 == 0 {\n    println!(\"Fizz\");\n} else if i % 5 == 0 {\n    println!(\"Buzz\");\n} else {\n    println!(\"{i}\");\n}"),
        erg,
        format!("{i} % 3 = {}, {i} % 5 = {}.", i % 3, i % 5),
    )
}

fn reihenfolge_loop(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let n = r.zahl(3, 9);
    let v = tr!(l, "total", "summe", "suma");
    reihenfolge(
        tr!(l, format!("Put the lines in order: print the sum of 1 to {n}."), format!("Bring die Zeilen in Reihenfolge: Summe von 1 bis {n} ausgeben."), format!("Ordena las líneas: imprimir la suma de 1 a {n}.")),
        vec![format!("let mut {v} = 0;"), format!("for i in 1..={n} {{"), format!("    {v} += i;"), "}".into(), format!("println!(\"{{{v}}}\");")],
        tr!(l, "Create the variable first, then the loop, print at the end.", "Erst die Variable anlegen, dann die Schleife, zum Schluss ausgeben.", "Primero la variable, luego el bucle y al final imprimir."),
    )
}

fn luecke_range(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let n = r.zahl(5, 20);
    luecke(
        r,
        tr!(l, format!("The loop should go from 1 to {n} **inclusive**."), format!("Die Schleife soll 1 bis **einschließlich** {n} durchlaufen."), format!("El bucle debe ir de 1 a {n} **incluido**.")),
        format!("for i in 1___{n} {{\n    println!(\"{{i}}\");\n}}"),
        "..=",
        &["..", "...", " to "],
        tr!(l, "`..=` includes the end, `..` doesn't.", "`..=` schließt das Ende ein, `..` nicht.", "`..=` incluye el final; `..` no."),
    )
}

fn verschachtelt(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let (a, b) = (r.zahl(2, 6), r.zahl(2, 6));
    eingabe(
        am_ende(l, "z"),
        c(format!("let mut z = 0;\nfor i in 0..{a} {{\n    for j in 0..{b} {{\n        z += 1;\n    }}\n}}")),
        (a * b).to_string(),
        tr!(l, format!("The inner loop runs {b} times per outer iteration: {a} × {b}."), format!("Die innere Schleife läuft {b}-mal pro äußerem Durchlauf: {a} × {b}."), format!("El bucle interior se ejecuta {b} veces por cada vuelta exterior: {a} × {b}.")),
    )
}

fn schleifen_wf(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    wf_pool(
        r,
        tr!(
            l,
            &[
                ("`0..5` contains the number 5.", false, "`..` excludes the end: 0, 1, 2, 3, 4."),
                ("`loop` runs forever until `break`.", true, "That's exactly what `loop` is for."),
                ("`continue` ends the whole loop.", false, "`continue` only jumps to the next iteration; `break` ends it."),
                ("A `loop` can return a value with `break value`.", true, "`let x = loop { break 5; };`"),
                ("`for i in (1..4).rev()` prints 3, 2, 1.", true, "Backwards, without the 4."),
            ],
            &[
                ("`0..5` enthält die Zahl 5.", false, "`..` schließt das Ende aus: 0, 1, 2, 3, 4."),
                ("`loop` läuft endlos, bis `break` kommt.", true, "Genau dafür ist `loop` da."),
                ("`continue` beendet die ganze Schleife.", false, "`continue` springt nur zum nächsten Durchlauf; `break` beendet."),
                ("Eine `loop`-Schleife kann mit `break wert` einen Wert zurückgeben.", true, "`let x = loop { break 5; };`"),
                ("`for i in (1..4).rev()` gibt 3, 2, 1 aus.", true, "Rückwärts, ohne die 4."),
            ],
            &[
                ("`0..5` contiene el número 5.", false, "`..` excluye el final: 0, 1, 2, 3, 4."),
                ("`loop` se repite sin fin hasta un `break`.", true, "Para eso existe `loop`."),
                ("`continue` termina todo el bucle.", false, "`continue` solo salta a la siguiente vuelta; `break` termina."),
                ("Un `loop` puede devolver un valor con `break valor`.", true, "`let x = loop { break 5; };`"),
                ("`for i in (1..4).rev()` imprime 3, 2, 1.", true, "Hacia atrás, sin el 4."),
            ]
        ),
    )
}

fn bausteine_for(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let n = r.zahl(3, 10);
    let falsch = format!("0..={n}");
    let m = n - 1;
    bausteine(
        tr!(l, format!("Build a loop where `i` goes from 0 to {m}."), format!("Baue eine Schleife, die `i` von 0 bis {m} durchläuft."), format!("Construye un bucle donde `i` vaya de 0 a {m}.")),
        vec!["for".into(), "i".into(), "in".into(), format!("0..{n}"), "{".into()],
        &["of", "while", ";", "=", falsch.as_str()],
        tr!(l, format!("`0..{n}` ends at {m}."), format!("`0..{n}` endet bei {m}."), format!("`0..{n}` termina en {m}.")),
    )
}
