//! Generators for worlds 7–9: ownership, borrowing, strings.

use super::*;

fn kompiliert(l: Sprache) -> &'static str {
    tr!(l, "Does this code compile?", "Kompiliert dieser Code?", "¿Compila este código?")
}

fn passt_zu(l: Sprache) -> &'static str {
    tr!(l, "Match them up.", "Ordne zu.", "Relaciona.")
}

// ─────────────────────────────── World 7: Ownership ───────────────────────────────

pub static W_OWNERSHIP: &[Gen] = &[
    move_wf, copy_auswahl, fehler_move, luecke_clone, drop_reihenfolge, ownership_paare, fn_move_wf, stack_heap,
    ownership_wf, heap_zaehlen,
];

/// (value expression, type name, is Copy, needs debug output)
const WERTE: &[(&str, &str, bool, bool)] = &[
    ("5", "i32", true, false),
    ("2.5", "f64", true, false),
    ("true", "bool", true, false),
    ("'R'", "char", true, false),
    ("String::from(\"hi\")", "String", false, false),
    ("vec![1, 2, 3]", "Vec<i32>", false, true),
];

fn move_wf(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let (wert, typ, copy, debug) = *r.wahl(WERTE);
    let fmt = if debug { "{a:?}" } else { "{a}" };
    wf(
        kompiliert(l),
        c(format!("let a = {wert};\nlet b = a;\nprintln!(\"{fmt}\");")),
        copy,
        if copy {
            tr!(l, format!("`{typ}` is `Copy`: `b` gets a copy, `a` stays valid."), format!("`{typ}` ist `Copy`: `b` bekommt eine Kopie, `a` bleibt gültig."), format!("`{typ}` es `Copy`: `b` recibe una copia y `a` sigue siendo válida."))
        } else {
            tr!(l, format!("`{typ}` is moved. Afterwards `a` is invalid – E0382."), format!("`{typ}` wird verschoben (move). Danach ist `a` ungültig – E0382."), format!("`{typ}` se mueve (move). Después `a` deja de ser válida: E0382."))
        },
    )
}

fn copy_auswahl(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let copy = ["`i32`", "`f64`", "`bool`", "`char`", "`u8`", "`(i32, i32)`"];
    let nicht = ["`String`", "`Vec<i32>`", "`Box<i32>`", "`HashMap<String, i32>`"];
    if r.bool() {
        let richtig = *r.wahl(&copy);
        auswahl(r, tr!(l, "Which type is `Copy`?", "Welcher Typ ist `Copy`?", "¿Qué tipo es `Copy`?"), None, richtig, vs(&nicht), tr!(l, "Simple fixed-size types on the stack are `Copy`.", "Einfache Typen mit fester Größe auf dem Stack sind `Copy`.", "Los tipos simples de tamaño fijo en la pila son `Copy`."))
    } else {
        let richtig = *r.wahl(&nicht);
        auswahl(r, tr!(l, "Which type is **not** `Copy`?", "Welcher Typ ist **nicht** `Copy`?", "¿Qué tipo **no** es `Copy`?"), None, richtig, vs(&copy), tr!(l, "Types that own data on the heap are not `Copy`.", "Typen, die Daten auf dem Heap besitzen, sind nicht `Copy`.", "Los tipos que poseen datos en el montón no son `Copy`."))
    }
}

fn fehler_move(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let (a, b) = *r.wahl(&[("s", "t"), ("name", "copy"), ("text", "other"), ("word", "w2")]);
    let n = *r.wahl(NAMEN);
    let mut zeilen = vec![format!("let {a} = String::from(\"{n}\");"), format!("let {b} = {a};")];
    let erst_b = r.bool();
    if erst_b {
        zeilen.push(format!("println!(\"{{{b}}}\");"));
        zeilen.push(format!("println!(\"{{{a}}}\");"));
    } else {
        zeilen.push(format!("println!(\"{{{a}}}\");"));
        zeilen.push(format!("println!(\"{{{b}}}\");"));
    }
    fehler(
        tr!(l, "In which line does the compiler report an error?", "In welcher Zeile meldet der Compiler einen Fehler?", "¿En qué línea da error el compilador?"),
        zeilen,
        if erst_b { 3 } else { 2 },
        tr!(l, format!("`{a}` was moved into `{b}` and can't be used any more."), format!("`{a}` wurde nach `{b}` verschoben und darf nicht mehr benutzt werden."), format!("`{a}` se movió a `{b}` y ya no se puede usar.")),
    )
}

fn luecke_clone(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    luecke(
        r,
        tr!(l, "Both variables should stay valid. What is missing?", "Beide Variablen sollen danach gültig sein. Was fehlt?", "Las dos variables deben seguir siendo válidas. ¿Qué falta?"),
        "let name = String::from(\"Ferris\");\nlet copy = name.___();\nprintln!(\"{name} {copy}\");",
        "clone",
        &["copy", "move", "borrow"],
        tr!(l, "`clone()` makes a deep copy including the heap data.", "`clone()` erzeugt eine tiefe Kopie mitsamt Heap-Daten.", "`clone()` hace una copia profunda, incluidos los datos del montón."),
    )
}

fn drop_reihenfolge(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let mut namen = vec!["a", "b", "c"];
    r.mischen(&mut namen);
    let kommentar = tr!(l, "cleanup happens here", "hier wird aufgeräumt", "aquí se libera todo");
    let code = format!("fn main() {{\n    let {} = String::from(\"1\");\n    let {} = String::from(\"2\");\n    let {} = String::from(\"3\");\n}} // {kommentar}", namen[0], namen[1], namen[2]);
    let richtig = format!("{}, {}, {}", namen[2], namen[1], namen[0]);
    let falsche = vec![format!("{}, {}, {}", namen[0], namen[1], namen[2]), format!("{}, {}, {}", namen[1], namen[0], namen[2]), format!("{}, {}, {}", namen[2], namen[0], namen[1])];
    auswahl(
        r,
        tr!(l, "In which order are the variables cleaned up (dropped) at the end?", "In welcher Reihenfolge werden die Variablen am Ende aufgeräumt (drop)?", "¿En qué orden se liberan (drop) las variables al final?"),
        c(code),
        richtig,
        falsche,
        tr!(l, "Variables are dropped in **reverse** order of creation – like a stack of plates.", "Variablen werden in **umgekehrter** Reihenfolge ihrer Erstellung aufgeräumt – wie ein Stapel Teller.", "Las variables se liberan en orden **inverso** a su creación, como una pila de platos."),
    )
}

fn ownership_paare(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    paare(
        r,
        tr!(l, "Match the terms.", "Ordne die Begriffe zu.", "Relaciona los términos."),
        &[
            ("Move", tr!(l, "ownership moves on", "Besitz wandert weiter", "la propiedad pasa a otro")),
            ("Clone", tr!(l, "deep copy", "tiefe Kopie", "copia profunda")),
            ("Copy", tr!(l, "cheap copy on the stack", "billige Kopie auf dem Stack", "copia barata en la pila")),
            ("Drop", tr!(l, "cleanup at scope end", "Aufräumen am Scope-Ende", "liberar al final del ámbito")),
            ("Owner", tr!(l, "the owner of a value", "der Besitzer eines Werts", "el dueño de un valor")),
            ("Heap", tr!(l, "memory for growing data", "Speicher für wachsende Daten", "memoria para datos que crecen")),
        ],
        4,
    )
}

fn fn_move_wf(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let basis = "fn take(s: String) {\n    println!(\"{s}\");\n}\n\nlet t = String::from(\"Hi\");\n";
    match r.idx(3) {
        0 => wf(kompiliert(l), c(format!("{basis}take(t);\nprintln!(\"{{t}}\");")), false, tr!(l, "`t` was passed to `take` (move) and is invalid afterwards.", "`t` wurde an `take` übergeben (move) und ist danach ungültig.", "`t` se pasó a `take` (move) y después ya no es válida.")),
        1 => wf(kompiliert(l), c(format!("{basis}take(t.clone());\nprintln!(\"{{t}}\");")), true, tr!(l, "`take` gets a copy, `t` stays valid.", "`take` bekommt eine Kopie, `t` bleibt gültig.", "`take` recibe una copia y `t` sigue siendo válida.")),
        _ => wf(kompiliert(l), c(format!("{basis}println!(\"{{t}}\");\ntake(t);")), true, tr!(l, "`t` is only moved after printing – that's allowed.", "`t` wird erst nach der Ausgabe verschoben – das ist erlaubt.", "`t` se mueve después de imprimir: eso está permitido.")),
    }
}

fn stack_heap(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    frage_pool(
        r,
        tr!(
            l,
            &[
                ("Where are the characters of a `String` stored?", "On the heap", &["On the stack", "In the compiler", "In Cargo.toml"], "A String can grow, so its data lives on the heap."),
                ("Where does an `i32` in a local variable live?", "On the stack", &["On the heap", "In a file", "In the garbage collector"], "Fixed-size values live directly on the stack."),
                ("What part of a `String` is on the stack?", "Pointer, length and capacity", &["All characters", "Nothing", "Only the length"], "The stack part is 24 bytes: pointer, length, capacity."),
                ("Who frees the heap memory of a `String`?", "Its owner, when it leaves the scope", &["A garbage collector", "You must call `free()`", "Nobody"], "Rust calls `drop` automatically when the owner goes away."),
            ],
            &[
                ("Wo liegen die Buchstaben eines `String`?", "Auf dem Heap", &["Auf dem Stack", "Im Compiler", "In der Cargo.toml"], "Ein String kann wachsen, deshalb liegen seine Daten auf dem Heap."),
                ("Wo liegt ein `i32` in einer lokalen Variable?", "Auf dem Stack", &["Auf dem Heap", "In einer Datei", "Im Garbage Collector"], "Werte fester Größe liegen direkt auf dem Stack."),
                ("Was liegt bei einem `String` auf dem Stack?", "Zeiger, Länge und Kapazität", &["Alle Buchstaben", "Nichts", "Nur die Länge"], "Der Stack-Teil ist 24 Bytes groß: Zeiger, Länge, Kapazität."),
                ("Wer gibt den Heap-Speicher eines `String` frei?", "Sein Besitzer, wenn er den Scope verlässt", &["Ein Garbage Collector", "Man muss `free()` aufrufen", "Niemand"], "Rust ruft automatisch `drop` auf, sobald der Besitzer verschwindet."),
            ],
            &[
                ("¿Dónde se guardan los caracteres de un `String`?", "En el montón (heap)", &["En la pila (stack)", "En el compilador", "En Cargo.toml"], "Un String puede crecer, por eso sus datos están en el montón."),
                ("¿Dónde vive un `i32` en una variable local?", "En la pila (stack)", &["En el montón (heap)", "En un archivo", "En el recolector de basura"], "Los valores de tamaño fijo viven directamente en la pila."),
                ("¿Qué parte de un `String` está en la pila?", "Puntero, longitud y capacidad", &["Todos los caracteres", "Nada", "Solo la longitud"], "La parte de la pila ocupa 24 bytes: puntero, longitud, capacidad."),
                ("¿Quién libera la memoria del montón de un `String`?", "Su dueño, cuando sale del ámbito", &["Un recolector de basura", "Hay que llamar a `free()`", "Nadie"], "Rust llama a `drop` automáticamente cuando el dueño desaparece."),
            ]
        ),
    )
}

fn ownership_wf(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    wf_pool(
        r,
        tr!(
            l,
            &[
                ("Every value has exactly one owner.", true, "That's the first ownership rule."),
                ("After `let b = a;` a `String` `a` is still valid.", false, "The String was moved."),
                ("`clone()` also copies the data on the heap.", true, "That's why it's more expensive than a move."),
                ("Rust only finds ownership errors at runtime.", false, "The compiler finds them while compiling."),
                ("`drop` is called automatically at the end of a scope.", true, "That's how memory is freed without a garbage collector."),
            ],
            &[
                ("Jeder Wert hat genau einen Besitzer.", true, "Das ist die erste Ownership-Regel."),
                ("Nach `let b = a;` ist ein `String` `a` weiterhin gültig.", false, "Der String wurde verschoben."),
                ("`clone()` kopiert auch die Daten auf dem Heap.", true, "Deshalb ist es teurer als ein Move."),
                ("Ownership-Fehler findet Rust erst beim Ausführen.", false, "Der Compiler findet sie schon beim Kompilieren."),
                ("Am Ende eines Scopes wird automatisch `drop` aufgerufen.", true, "So wird Speicher ohne Garbage Collector freigegeben."),
            ],
            &[
                ("Cada valor tiene exactamente un dueño.", true, "Es la primera regla de ownership."),
                ("Después de `let b = a;`, un `String` `a` sigue siendo válido.", false, "El String se movió."),
                ("`clone()` también copia los datos del montón.", true, "Por eso es más caro que un move."),
                ("Rust solo detecta errores de ownership al ejecutar.", false, "El compilador los detecta al compilar."),
                ("Al final de un ámbito se llama automáticamente a `drop`.", true, "Así se libera memoria sin recolector de basura."),
            ]
        ),
    )
}

fn heap_zaehlen(r: &mut Rng, s: u8, l: Sprache) -> Uebung {
    let mut code = "let a = String::from(\"x\");\n".to_string();
    let mut strings = 1;
    let mut namen = vec!["a".to_string()];
    let buchstaben = ["b", "c", "d", "e", "f"];
    for b in buchstaben.iter().take(2 + s as usize) {
        let quelle = namen.last().cloned().unwrap_or_default();
        if r.bool() {
            code.push_str(&format!("let {b} = {quelle}.clone();\n"));
            strings += 1;
        } else {
            code.push_str(&format!("let {b} = {quelle};\n"));
        }
        namen.push((*b).into());
    }
    eingabe(
        tr!(l, "How many strings are on the heap at the end?", "Wie viele Strings liegen am Ende auf dem Heap?", "¿Cuántos strings hay en el montón al final?"),
        c(code.trim_end()),
        strings.to_string(),
        tr!(l, "Only `String::from` and `clone()` create new heap data. A move just passes ownership on.", "Nur `String::from` und `clone()` legen neue Heap-Daten an. Ein Move gibt nur den Besitz weiter.", "Solo `String::from` y `clone()` crean datos nuevos en el montón. Un move solo pasa la propiedad."),
    )
}

// ─────────────────────────────── World 8: Borrowing ───────────────────────────────

pub static W_BORROWING: &[Gen] = &[
    borrow_kombi, deref_rechnen, fehler_borrow, luecke_ref, bausteine_ref, borrow_fragen, borrow_paare, deref_fn,
    borrowing_wf,
];

fn borrow_kombi(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let i = r.idx(5);
    let codes = [
        "let mut s = String::from(\"Hi\");\nlet a = &s;\nlet b = &s;\nprintln!(\"{a} {b}\");",
        "let mut s = String::from(\"Hi\");\nlet a = &s;\nlet b = &mut s;\nprintln!(\"{a}\");",
        "let mut s = String::from(\"Hi\");\nlet a = &mut s;\nlet b = &mut s;\na.push('!');",
        "let mut s = String::from(\"Hi\");\nlet a = &s;\nprintln!(\"{a}\");\nlet b = &mut s;\nb.push('!');",
        "let s = String::from(\"Hi\");\nlet b = &mut s;",
    ];
    let ok = [true, false, false, true, false];
    let erkl = [
        tr!(l, "Many shared references at the same time are allowed.", "Viele lesende Referenzen gleichzeitig sind erlaubt.", "Se permiten muchas referencias compartidas a la vez."),
        tr!(l, "`a` is still used after `&mut s` – reading and writing at the same time is forbidden (E0502).", "`a` wird nach `&mut s` noch benutzt – Lesen und Schreiben gleichzeitig ist verboten (E0502).", "`a` se usa después de `&mut s`: leer y escribir a la vez está prohibido (E0502)."),
        tr!(l, "Two `&mut` at the same time are forbidden (E0499).", "Zwei `&mut` gleichzeitig sind verboten (E0499).", "Dos `&mut` a la vez están prohibidos (E0499)."),
        tr!(l, "`a` is used for the last time before `&mut s` – its borrow is already over.", "`a` wird vor `&mut s` zum letzten Mal benutzt – seine Ausleihe ist dann schon vorbei.", "`a` se usa por última vez antes de `&mut s`: su préstamo ya terminó."),
        tr!(l, "For `&mut` the variable itself must be `mut` (E0596).", "Für `&mut` muss die Variable selbst `mut` sein (E0596).", "Para `&mut`, la propia variable debe ser `mut` (E0596)."),
    ];
    wf(kompiliert(l), c(codes[i]), ok[i], erkl[i])
}

fn deref_rechnen(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let (a, b, k) = (r.zahl(1, 10), r.zahl(1, 10), r.zahl(2, 4));
    eingabe(
        tr!(l, "What does the program print?", "Was gibt das Programm aus?", "¿Qué imprime el programa?"),
        c(format!("let mut x = {a};\n{{\n    let r = &mut x;\n    *r += {b};\n    *r *= {k};\n}}\nprintln!(\"{{x}}\");")),
        ((a + b) * k).to_string(),
        tr!(l, "`*r` changes the value `r` points to – that is `x` itself.", "Über `*r` wird der Wert verändert, auf den `r` zeigt – also `x` selbst.", "Con `*r` se cambia el valor al que apunta `r`, es decir, `x`."),
    )
}

fn fehler_borrow(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let mut zeilen = vs(&["let mut s = String::from(\"Hi\");", "let r1 = &s;", "let r2 = &mut s;", "println!(\"{r1}\");"]);
    let mut falsch = 2;
    if r.bool() {
        zeilen.insert(0, "let n = 5;".into());
        falsch += 1;
    }
    fehler(
        tr!(l, "Which line breaks the borrowing rules?", "Welche Zeile verletzt die Borrowing-Regeln?", "¿Qué línea rompe las reglas de préstamo?"),
        zeilen,
        falsch,
        tr!(l, "`r2` wants a mutable borrow while `r1` still reads later.", "`r2` will veränderbar ausleihen, während `r1` später noch liest.", "`r2` quiere un préstamo mutable mientras `r1` todavía lee después."),
    )
}

fn luecke_ref(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    if r.bool() {
        luecke(r, tr!(l, "The function should be allowed to change the string.", "Die Funktion soll den String verändern dürfen.", "La función debe poder cambiar el string."), "fn append(s: ___String) {\n    s.push('!');\n}", "&mut ", &["& ", "mut ", "* "], tr!(l, "To change it you need a mutable reference `&mut`.", "Zum Verändern braucht man eine veränderbare Referenz `&mut`.", "Para cambiarlo hace falta una referencia mutable `&mut`."))
    } else {
        luecke(r, tr!(l, "The function should only read the string.", "Die Funktion soll den String nur lesen.", "La función solo debe leer el string."), "fn length(s: ___String) -> usize {\n    s.len()\n}", "&", &["&mut ", "mut ", "*"], tr!(l, "For reading, a shared reference `&` is enough.", "Zum Lesen reicht eine geteilte Referenz `&`.", "Para leer basta con una referencia compartida `&`."))
    }
}

fn bausteine_ref(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let v = *r.wahl(&["text", "name", "list"]);
    bausteine(
        tr!(l, format!("Call `change` with a mutable reference to `{v}`."), format!("Rufe `change` mit einer veränderbaren Referenz auf `{v}` auf."), format!("Llama a `change` con una referencia mutable a `{v}`.")),
        vec!["change".into(), "(".into(), "&mut".into(), v.into(), ")".into(), ";".into()],
        &["&", "*", "mut"],
        tr!(l, "`&mut variable` creates a mutable reference.", "`&mut variable` erzeugt eine veränderbare Referenz.", "`&mut variable` crea una referencia mutable."),
    )
}

fn borrow_fragen(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    frage_pool(
        r,
        tr!(
            l,
            &[
                ("How many `&mut` references to the same value may exist at once?", "Exactly one", &["Any number", "Two", "None"], "Exactly one – and while it lives, no shared ones."),
                ("How many `&` references may exist at once?", "Any number", &["Exactly one", "At most two", "None"], "Reading doesn't disturb anyone."),
                ("What does `*` do in `*number += 1`?", "Accesses the value behind the reference", &["Multiplies", "Deletes the reference", "Copies the number"], "That's called dereferencing."),
                ("What does `fn f(x: &String)` mean?", "f borrows the string for reading", &["f owns the string", "f can change the string", "f gets a copy"], "`&` is a shared (read-only) borrow."),
            ],
            &[
                ("Wie viele `&mut`-Referenzen auf denselben Wert darf es gleichzeitig geben?", "Genau eine", &["Beliebig viele", "Zwei", "Keine"], "Genau eine – und solange sie lebt, keine lesenden."),
                ("Wie viele `&`-Referenzen darf es gleichzeitig geben?", "Beliebig viele", &["Genau eine", "Höchstens zwei", "Keine"], "Lesen stört niemanden."),
                ("Was macht `*` bei `*zahl += 1`?", "Greift auf den Wert hinter der Referenz zu", &["Multipliziert", "Löscht die Referenz", "Kopiert die Zahl"], "Das heißt Dereferenzieren."),
                ("Was bedeutet `fn f(x: &String)`?", "f leiht den String zum Lesen", &["f besitzt den String", "f kann den String ändern", "f bekommt eine Kopie"], "`&` ist eine lesende Ausleihe."),
            ],
            &[
                ("¿Cuántas referencias `&mut` al mismo valor pueden existir a la vez?", "Exactamente una", &["Las que quieras", "Dos", "Ninguna"], "Exactamente una y, mientras vive, ninguna compartida."),
                ("¿Cuántas referencias `&` pueden existir a la vez?", "Las que quieras", &["Exactamente una", "Como mucho dos", "Ninguna"], "Leer no molesta a nadie."),
                ("¿Qué hace `*` en `*numero += 1`?", "Accede al valor detrás de la referencia", &["Multiplica", "Borra la referencia", "Copia el número"], "Eso se llama desreferenciar."),
                ("¿Qué significa `fn f(x: &String)`?", "f toma prestado el string para leerlo", &["f es dueña del string", "f puede cambiar el string", "f recibe una copia"], "`&` es un préstamo de solo lectura."),
            ]
        ),
    )
}

fn borrow_paare(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    paare(
        r,
        passt_zu(l),
        &[
            ("&x", tr!(l, "borrow for reading", "lesend ausleihen", "prestar para leer")),
            ("&mut x", tr!(l, "borrow for changing", "veränderbar ausleihen", "prestar para cambiar")),
            ("*r", tr!(l, "dereference", "dereferenzieren", "desreferenciar")),
            ("Borrow checker", tr!(l, "checks borrows when compiling", "prüft Ausleihen beim Kompilieren", "comprueba préstamos al compilar")),
            ("E0502", tr!(l, "& and &mut at the same time", "& und &mut gleichzeitig", "& y &mut a la vez")),
        ],
        4,
    )
}

fn deref_fn(r: &mut Rng, s: u8, l: Sprache) -> Uebung {
    let mut x = r.zahl(1, 10);
    let start = x;
    let mut aufrufe = String::new();
    for _ in 0..(1 + s) {
        let n = r.zahl(1, 9);
        x += n;
        aufrufe.push_str(&format!("plus(&mut z, {n});\n"));
    }
    eingabe(
        tr!(l, "What is the value of `z` at the end?", "Welchen Wert hat `z` am Ende?", "¿Qué valor tiene `z` al final?"),
        c(format!("fn plus(z: &mut i32, n: i32) {{\n    *z += n;\n}}\n\nlet mut z = {start};\n{aufrufe}").trim_end()),
        x.to_string(),
        tr!(l, "Every call changes `z` through the reference.", "Jeder Aufruf verändert `z` über die Referenz.", "Cada llamada cambia `z` a través de la referencia."),
    )
}

fn borrowing_wf(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    wf_pool(
        r,
        tr!(
            l,
            &[
                ("A reference owns the value it points to.", false, "It only borrows it."),
                ("References in Rust are always valid.", true, "The borrow checker prevents dangling references."),
                ("For `&mut x`, `x` must be declared with `let mut`.", true, "Otherwise you get E0596."),
                ("Many readers and one writer at the same time are allowed.", false, "Either many readers or exactly one writer."),
                ("With `s.push('!')` Rust dereferences automatically.", true, "Method calls do it for you."),
            ],
            &[
                ("Eine Referenz besitzt den Wert, auf den sie zeigt.", false, "Sie leiht ihn nur aus."),
                ("Referenzen sind in Rust immer gültig.", true, "Der Borrow Checker verhindert hängende Referenzen."),
                ("Für `&mut x` muss `x` mit `let mut` angelegt sein.", true, "Sonst gibt es E0596."),
                ("Viele Leser und ein Schreiber gleichzeitig sind erlaubt.", false, "Entweder viele Leser oder genau ein Schreiber."),
                ("Bei `s.push('!')` dereferenziert Rust automatisch.", true, "Bei Methodenaufrufen passiert das von selbst."),
            ],
            &[
                ("Una referencia es dueña del valor al que apunta.", false, "Solo lo toma prestado."),
                ("En Rust, las referencias siempre son válidas.", true, "El borrow checker impide referencias colgantes."),
                ("Para `&mut x`, `x` debe declararse con `let mut`.", true, "Si no, aparece E0596."),
                ("Se permiten muchos lectores y un escritor a la vez.", false, "O muchos lectores o exactamente un escritor."),
                ("Con `s.push('!')`, Rust desreferencia automáticamente.", true, "Las llamadas a métodos lo hacen solas."),
            ]
        ),
    )
}

// ─────────────────────────────── World 9: Strings ───────────────────────────────

pub static W_STRINGS: &[Gen] = &[
    laenge_bytes, zeichen_zaehlen, slice, string_methoden, starts_with_wf, woerter_zaehlen, format_ergebnis, str_typ,
    string_paare, luecke_push, fehler_index,
];

fn woerter(l: Sprache) -> &'static [&'static str] {
    tr!(
        l,
        &["café", "naïve", "Ferris", "Rust", "jalapeño", "über", "piñata", "Zoë", "crème", "Hello", "résumé", "crab"],
        &["Grüße", "Käse", "Straße", "Ferris", "Rust", "Müll", "über", "Öl", "Bär", "Fuß", "Hallo", "Krabbe"],
        &["año", "niño", "canción", "Ferris", "Rust", "corazón", "España", "mañana", "jamón", "Hola", "acción", "cangrejo"]
    )
}

fn laenge_bytes(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let w = *r.wahl(woerter(l));
    let (z, b) = (w.chars().count(), w.len());
    eingabe(
        tr!(l, format!("What does `\"{w}\".len()` return?"), format!("Was liefert `\"{w}\".len()`?"), format!("¿Qué devuelve `\"{w}\".len()`?")),
        None,
        b.to_string(),
        tr!(
            l,
            format!("`len()` counts bytes: accented letters take 2 bytes. “{w}” has {z} characters but {b} bytes."),
            format!("`len()` zählt Bytes: Umlaute und ß brauchen 2 Bytes. „{w}“ hat {z} Zeichen, aber {b} Bytes."),
            format!("`len()` cuenta bytes: las letras con tilde o ñ ocupan 2 bytes. «{w}» tiene {z} caracteres, pero {b} bytes.")
        ),
    )
}

fn zeichen_zaehlen(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let w = format!("{} {}", r.wahl(woerter(l)), r.wahl(&["🦀", "!", "ß", "42"]));
    let b = w.len();
    eingabe(
        tr!(l, format!("What does `\"{w}\".chars().count()` return?"), format!("Was liefert `\"{w}\".chars().count()`?"), format!("¿Qué devuelve `\"{w}\".chars().count()`?")),
        None,
        w.chars().count().to_string(),
        tr!(l, format!("`chars()` counts characters (including spaces), not bytes. In bytes it would be {b}."), format!("`chars()` zählt Zeichen (inklusive Leerzeichen), nicht Bytes. In Bytes wären es {b}."), format!("`chars()` cuenta caracteres (incluidos espacios), no bytes. En bytes serían {b}.")),
    )
}

fn slice(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let w = *r.wahl(&["Ferris", "Crabby", "Rustacean", "Compiler", "Ownership", "Borrowing"]);
    let a = r.idx(3);
    let b = a + 2 + r.idx(w.len() - a - 2);
    eingabe(
        tr!(l, format!("What is `&s[{a}..{b}]`?"), format!("Was ist `&s[{a}..{b}]`?"), format!("¿Qué es `&s[{a}..{b}]`?")),
        c(format!("let s = String::from(\"{w}\");")),
        &w[a..b],
        tr!(l, format!("Bytes {a} up to but not including {b}."), format!("Bytes {a} bis ausschließlich {b}."), format!("Bytes de {a} hasta {b}, sin incluirlo.")),
    )
}

fn string_methoden(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let w = *r.wahl(&["rust", "crab", "hello", "banana"]);
    let frage = tr!(l, "What does the expression give?", "Was ergibt der Ausdruck?", "¿Qué da la expresión?");
    match r.idx(3) {
        0 => eingabe(frage, c(format!("\"{w}\".to_uppercase()")), w.to_uppercase(), tr!(l, "`to_uppercase` converts to capital letters.", "`to_uppercase` wandelt in Großbuchstaben um.", "`to_uppercase` convierte a mayúsculas.")),
        1 => eingabe(frage, c(format!("\"{w}\".replace(\"a\", \"o\")")), w.replace('a', "o"), tr!(l, "`replace` replaces every occurrence.", "`replace` ersetzt jedes Vorkommen.", "`replace` sustituye cada aparición.")),
        _ => {
            let n = r.zahl(2, 3) as usize;
            eingabe(frage, c(format!("\"{w}\".repeat({n})")), w.repeat(n), tr!(l, "`repeat(n)` puts the text n times in a row.", "`repeat(n)` hängt den Text n-mal hintereinander.", "`repeat(n)` repite el texto n veces seguidas."))
        }
    }
}

fn starts_with_wf(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let w = *r.wahl(&["Ferris", "Rustacean", "Compiler", "Crabby"]);
    let teil = *r.wahl(&["Fe", "Ru", "er", "ab", "Co", "ce", "Cr", "by"]);
    if r.bool() {
        let ok = w.starts_with(teil);
        wf(
            tr!(l, format!("`\"{w}\".starts_with(\"{teil}\")` returns `true`."), format!("`\"{w}\".starts_with(\"{teil}\")` ergibt `true`."), format!("`\"{w}\".starts_with(\"{teil}\")` devuelve `true`.")),
            None,
            ok,
            if ok { tr!(l, format!("“{w}” starts with “{teil}”."), format!("„{w}“ beginnt mit „{teil}“."), format!("«{w}» empieza por «{teil}».")) } else { tr!(l, format!("“{w}” doesn't start with “{teil}”."), format!("„{w}“ beginnt nicht mit „{teil}“."), format!("«{w}» no empieza por «{teil}».")) },
        )
    } else {
        let ok = w.contains(teil);
        wf(
            tr!(l, format!("`\"{w}\".contains(\"{teil}\")` returns `true`."), format!("`\"{w}\".contains(\"{teil}\")` ergibt `true`."), format!("`\"{w}\".contains(\"{teil}\")` devuelve `true`.")),
            None,
            ok,
            if ok { tr!(l, format!("“{teil}” occurs in “{w}”."), format!("„{teil}“ kommt in „{w}“ vor."), format!("«{teil}» aparece en «{w}».")) } else { tr!(l, format!("“{teil}” doesn't occur in “{w}”."), format!("„{teil}“ kommt in „{w}“ nicht vor."), format!("«{teil}» no aparece en «{w}».")) },
        )
    }
}

fn woerter_zaehlen(r: &mut Rng, s: u8, l: Sprache) -> Uebung {
    let pool: [&str; 8] = tr!(
        l,
        ["Rust", "is", "fast", "and", "safe", "Ferris", "likes", "code"],
        ["Rust", "ist", "schnell", "und", "sicher", "Ferris", "mag", "Code"],
        ["Rust", "es", "rápido", "y", "seguro", "Ferris", "ama", "código"]
    );
    let n = 3 + r.idx(2 + s as usize);
    let satz: Vec<&str> = (0..n).map(|_| *r.wahl(&pool)).collect();
    let text = satz.join(if s >= 3 { "   " } else { " " });
    eingabe(
        tr!(l, "What does the expression return?", "Was liefert der Ausdruck?", "¿Qué devuelve la expresión?"),
        c(format!("\"{text}\".split_whitespace().count()")),
        text.split_whitespace().count().to_string(),
        tr!(l, "`split_whitespace` splits at spaces – also at several in a row.", "`split_whitespace` trennt an Leerzeichen – auch an mehreren hintereinander.", "`split_whitespace` separa por espacios, también si hay varios seguidos."),
    )
}

fn format_ergebnis(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let n = *r.wahl(NAMEN);
    let a = r.zahl(1, 99);
    let frage = tr!(l, "What does `s` contain?", "Welchen Inhalt hat `s`?", "¿Qué contiene `s`?");
    match r.idx(3) {
        0 => eingabe(frage, c(format!("let s = format!(\"{{}}-{{}}\", \"{n}\", {a});")), format!("{n}-{a}"), tr!(l, "`format!` works like `println!` but returns a String.", "`format!` arbeitet wie `println!`, gibt aber einen String zurück.", "`format!` funciona como `println!`, pero devuelve un String.")),
        1 => eingabe(frage, c(format!("let x = {a};\nlet s = format!(\"[{{x:>4}}]\");")), format!("[{a:>4}]"), tr!(l, "`:>4` right-aligns to 4 characters.", "`:>4` richtet rechtsbündig auf 4 Zeichen aus.", "`:>4` alinea a la derecha en 4 caracteres.")),
        _ => {
            let f = a as f64 / 7.0;
            eingabe(frage, c(format!("let s = format!(\"{{:.2}}\", {a}.0 / 7.0);")), format!("{f:.2}"), tr!(l, "`:.2` rounds to two decimal places.", "`:.2` rundet auf zwei Nachkommastellen.", "`:.2` redondea a dos decimales."))
        }
    }
}

fn str_typ(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let (ausdruck, typ) = *r.wahl(&[("\"Hello\"", "&str"), ("String::from(\"Hello\")", "String"), ("&s[0..2]", "&str"), ("format!(\"{}\", 5)", "String"), ("\"Hi\".to_string()", "String")]);
    auswahl(
        r,
        tr!(l, format!("What type does `{ausdruck}` have?"), format!("Welchen Typ hat `{ausdruck}`?"), format!("¿Qué tipo tiene `{ausdruck}`?")),
        None,
        format!("`{typ}`"),
        vs(&["`&str`", "`String`", "`char`", "`&String`"]),
        tr!(l, "Literals and slices are `&str`; `String::from`, `format!` and `to_string` create an owned `String`.", "Literale und Slices sind `&str`; `String::from`, `format!` und `to_string` erzeugen einen eigenen `String`.", "Los literales y slices son `&str`; `String::from`, `format!` y `to_string` crean un `String` propio."),
    )
}

fn string_paare(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    paare(
        r,
        passt_zu(l),
        &[
            ("String", tr!(l, "owns text on the heap", "besitzt Text auf dem Heap", "posee texto en el montón")),
            ("&str", tr!(l, "borrowed piece of text", "geliehener Text-Ausschnitt", "trozo de texto prestado")),
            ("len()", tr!(l, "number of bytes", "Anzahl Bytes", "número de bytes")),
            ("chars()", tr!(l, "single characters", "einzelne Zeichen", "caracteres sueltos")),
            ("push_str", tr!(l, "append text", "Text anhängen", "añadir texto")),
            ("trim()", tr!(l, "remove spaces at the edges", "Leerzeichen am Rand entfernen", "quitar espacios en los bordes")),
        ],
        4,
    )
}

fn luecke_push(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let frage = tr!(l, "What goes in the gap?", "Was gehört in die Lücke?", "¿Qué va en el hueco?");
    let gruss = tr!(l, "Hello", "Hallo", "Hola");
    let welt = tr!(l, ", world", ", Welt", ", mundo");
    if r.bool() {
        luecke(r, frage, format!("let mut s = String::from(\"{gruss}\");\ns.___(\"{welt}\");"), "push_str", &["push", "add", "append_char"], tr!(l, "`push_str` appends text, `push` only a single character.", "`push_str` hängt einen Text an, `push` nur ein einzelnes Zeichen.", "`push_str` añade un texto; `push`, solo un carácter."))
    } else {
        luecke(r, frage, format!("let mut s = String::from(\"{gruss}\");\ns.___('!');"), "push", &["push_str", "add", "insert_str"], tr!(l, "For a single character (`char`) you use `push`.", "Für ein einzelnes Zeichen (`char`) nimmt man `push`.", "Para un solo carácter (`char`) se usa `push`."))
    }
}

fn fehler_index(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let n = *r.wahl(NAMEN);
    fehler(
        tr!(l, "Which line doesn't compile?", "Welche Zeile kompiliert nicht?", "¿Qué línea no compila?"),
        vec![format!("let s = String::from(\"{n}\");"), "let n = s.len();".into(), "let first = s[0];".into(), "println!(\"{n}\");".into()],
        2,
        tr!(l, "You can't index strings with `[0]`. Use `s.chars().next()`.", "Strings kann man nicht mit `[0]` indizieren. Nutze `s.chars().next()`.", "Los strings no se pueden indexar con `[0]`. Usa `s.chars().next()`."),
    )
}
