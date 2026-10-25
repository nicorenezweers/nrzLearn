//! Generators for worlds 10–12: structs, enums & match, Vec & HashMap.

use super::*;

fn passt_zu(l: Sprache) -> &'static str {
    tr!(l, "Match them up.", "Ordne zu.", "Relaciona.")
}

fn was_luecke(l: Sprache) -> &'static str {
    tr!(l, "What goes in the gap?", "Was gehört in die Lücke?", "¿Qué va en el hueco?")
}

// ─────────────────────────────── World 10: Structs ───────────────────────────────

pub static W_STRUCTS: &[Gen] = &[
    self_art, konto, rechteck, luecke_aufruf, bausteine_struct, reihenfolge_impl, fehler_self, struct_paare, feld_wert,
    struct_wf,
];

fn self_art(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let keiner = tr!(l, "none at all", "gar keinen", "ninguno");
    let pool: [(&str, &str); 6] = tr!(
        l,
        [
            ("only reads the fields and returns the area", "`&self`"),
            ("increases the counter by 1", "`&mut self`"),
            ("consumes the object and returns a String", "`self`"),
            ("creates a new object (like a constructor)", "none at all"),
            ("changes the name", "`&mut self`"),
            ("checks whether the account is empty", "`&self`"),
        ],
        [
            ("liest nur die Felder und gibt die Fläche zurück", "`&self`"),
            ("erhöht den Zähler um 1", "`&mut self`"),
            ("verbraucht das Objekt und gibt einen String zurück", "`self`"),
            ("erzeugt ein neues Objekt (wie ein Konstruktor)", "gar keinen"),
            ("ändert den Namen", "`&mut self`"),
            ("prüft, ob das Konto leer ist", "`&self`"),
        ],
        [
            ("solo lee los campos y devuelve el área", "`&self`"),
            ("aumenta el contador en 1", "`&mut self`"),
            ("consume el objeto y devuelve un String", "`self`"),
            ("crea un objeto nuevo (como un constructor)", "ninguno"),
            ("cambia el nombre", "`&mut self`"),
            ("comprueba si la cuenta está vacía", "`&self`"),
        ]
    );
    let (was, richtig) = *r.wahl(&pool);
    auswahl(
        r,
        tr!(l, format!("A method **{was}**. What first parameter does it need?"), format!("Eine Methode **{was}**. Welchen ersten Parameter braucht sie?"), format!("Un método **{was}**. ¿Qué primer parámetro necesita?")),
        None,
        richtig,
        vec!["`&self`".into(), "`&mut self`".into(), "`self`".into(), keiner.into()],
        tr!(l, "`&self` reads, `&mut self` changes, `self` consumes – without `self` it's an associated function like `new()`.", "`&self` liest, `&mut self` ändert, `self` verbraucht – ohne `self` ist es eine assoziierte Funktion wie `new()`.", "`&self` lee, `&mut self` cambia, `self` consume; sin `self` es una función asociada como `new()`."),
    )
}

fn konto(r: &mut Rng, s: u8, l: Sprache) -> Uebung {
    let mut saldo = r.zahl(5, 20) * 10;
    let start = saldo;
    let mut code = "struct Account { balance: i64 }\n\nimpl Account {\n    fn deposit(&mut self, b: i64) { self.balance += b; }\n    fn withdraw(&mut self, b: i64) {\n        if b <= self.balance { self.balance -= b; }\n    }\n}\n\n".to_string();
    code.push_str(&format!("let mut a = Account {{ balance: {start} }};\n"));
    for _ in 0..(2 + s) {
        let b = r.zahl(1, 25) * 10;
        if r.bool() {
            saldo += b;
            code.push_str(&format!("a.deposit({b});\n"));
        } else {
            if b <= saldo {
                saldo -= b;
            }
            code.push_str(&format!("a.withdraw({b});\n"));
        }
    }
    eingabe(
        tr!(l, "What is `a.balance` at the end?", "Welchen Wert hat `a.balance` am Ende?", "¿Qué valor tiene `a.balance` al final?"),
        c(code.trim_end()),
        saldo.to_string(),
        tr!(l, "Careful: `withdraw` does nothing if there isn't enough money.", "Achtung: `withdraw` tut nichts, wenn nicht genug Geld da ist.", "Ojo: `withdraw` no hace nada si no hay dinero suficiente."),
    )
}

fn rechteck(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let (b, h) = (r.zahl(2, 12), r.zahl(2, 12));
    let code = format!("impl Rectangle {{\n    fn area(&self) -> u32 {{ self.w * self.h }}\n    fn perimeter(&self) -> u32 {{ 2 * (self.w + self.h) }}\n}}\n\nlet r = Rectangle {{ w: {b}, h: {h} }};");
    if r.bool() {
        eingabe(tr!(l, "What does `r.area()` return?", "Was liefert `r.area()`?", "¿Qué devuelve `r.area()`?"), c(code), (b * h).to_string(), format!("{b} × {h}"))
    } else {
        eingabe(tr!(l, "What does `r.perimeter()` return?", "Was liefert `r.perimeter()`?", "¿Qué devuelve `r.perimeter()`?"), c(code), (2 * (b + h)).to_string(), format!("2 × ({b} + {h})"))
    }
}

fn luecke_aufruf(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    if r.bool() {
        luecke(r, tr!(l, "Call the associated function `new`:", "Die assoziierte Funktion `new` aufrufen:", "Llama a la función asociada `new`:"), "let r = Rectangle___new(3, 4);", "::", &[".", "->", ":"], tr!(l, "Associated functions are called on the type with `::`.", "Assoziierte Funktionen ruft man am Typ mit `::` auf.", "Las funciones asociadas se llaman sobre el tipo con `::`."))
    } else {
        luecke(r, tr!(l, "Call the method `area`:", "Die Methode `area` aufrufen:", "Llama al método `area`:"), "let r = Rectangle::new(3, 4);\nlet a = r___area();", ".", &["::", "->", ":"], tr!(l, "Methods are called on an instance with `.`.", "Methoden ruft man an einer Instanz mit `.` auf.", "Los métodos se llaman sobre una instancia con `.`."))
    }
}

fn bausteine_struct(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let (name, f1, f2) = *r.wahl(&[("Point", "x: i32,", "y: i32"), ("Person", "name: String,", "age: u8"), ("Car", "brand: String,", "hp: u32")]);
    bausteine(
        tr!(l, format!("Define the struct `{name}`."), format!("Definiere das Struct `{name}`."), format!("Define el struct `{name}`.")),
        vec!["struct".into(), name.into(), "{".into(), f1.into(), f2.into(), "}".into()],
        &["class", "=", ";", "fn"],
        tr!(l, "`struct Name { field: Type, … }`", "`struct Name { feld: Typ, … }`", "`struct Nombre { campo: Tipo, … }`"),
    )
}

fn reihenfolge_impl(_: &mut Rng, _: u8, l: Sprache) -> Uebung {
    reihenfolge(
        tr!(l, "Put the lines in the right order.", "Bring die Zeilen in die richtige Reihenfolge.", "Ordena las líneas."),
        vs(&["impl Counter {", "    fn increment(&mut self) {", "        self.value += 1;", "    }", "}"]),
        tr!(l, "First the `impl` block, inside it the method, inside that the body.", "Erst der `impl`-Block, darin die Methode, darin der Körper.", "Primero el bloque `impl`, dentro el método y dentro el cuerpo."),
    )
}

fn fehler_self(_: &mut Rng, _: u8, l: Sprache) -> Uebung {
    fehler(
        tr!(l, "Which line doesn't compile?", "Welche Zeile kompiliert nicht?", "¿Qué línea no compila?"),
        vs(&["impl Counter {", "    fn increment(&self) {", "        self.value += 1;", "    }", "}"]),
        2,
        tr!(l, "With `&self` the method can't change anything. Correct would be `&mut self`.", "Mit `&self` darf die Methode nichts ändern. Richtig wäre `&mut self`.", "Con `&self` el método no puede cambiar nada. Lo correcto sería `&mut self`."),
    )
}

fn struct_paare(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    paare(
        r,
        passt_zu(l),
        &[
            ("struct", tr!(l, "own data type with fields", "eigener Datentyp mit Feldern", "tipo propio con campos")),
            ("impl", tr!(l, "block for methods", "Block für Methoden", "bloque para métodos")),
            ("&self", tr!(l, "method reads", "Methode liest", "el método lee")),
            ("&mut self", tr!(l, "method changes", "Methode ändert", "el método cambia")),
            ("Self", tr!(l, "the own type", "der eigene Typ", "el propio tipo")),
            ("#[derive(Debug)]", tr!(l, "printing with {:?}", "Ausgabe mit {:?}", "imprimir con {:?}")),
        ],
        4,
    )
}

fn feld_wert(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let (a, b, k) = (r.zahl(1, 9), r.zahl(1, 9), r.zahl(1, 9));
    let x = a + k;
    eingabe(
        tr!(l, "What is `p.y` at the end?", "Welchen Wert hat `p.y` am Ende?", "¿Qué valor tiene `p.y` al final?"),
        c(format!("let mut p = Point {{ x: {a}, y: {b} }};\np.x += {k};\np.y = p.x * 2;")),
        (x * 2).to_string(),
        tr!(l, format!("p.x becomes {x}, so p.y = {x} × 2."), format!("p.x wird {x}, also p.y = {x} × 2."), format!("p.x pasa a {x}, así que p.y = {x} × 2.")),
    )
}

fn struct_wf(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    wf_pool(
        r,
        tr!(
            l,
            &[
                ("Methods are defined in an `impl` block.", true, "`impl Type { … }`"),
                ("To change a field, the variable must be `mut`.", true, "`let mut p = Point { … };`"),
                ("`Self` is another name for the own type.", true, "Handy in `fn new() -> Self`."),
                ("Structs in Rust can't have methods.", false, "That's what `impl` is for."),
                ("`Point { x, y }` works if there are variables `x` and `y`.", true, "The field init shorthand."),
            ],
            &[
                ("Methoden werden in einem `impl`-Block definiert.", true, "`impl Typ { … }`"),
                ("Um ein Feld zu ändern, muss die Variable `mut` sein.", true, "`let mut p = Point { … };`"),
                ("`Self` ist ein anderer Name für den eigenen Typ.", true, "Praktisch in `fn new() -> Self`."),
                ("Structs können in Rust keine Methoden haben.", false, "Dafür gibt es `impl`."),
                ("`Point { x, y }` funktioniert, wenn es Variablen `x` und `y` gibt.", true, "Die Kurzschreibweise (field init shorthand)."),
            ],
            &[
                ("Los métodos se definen en un bloque `impl`.", true, "`impl Tipo { … }`"),
                ("Para cambiar un campo, la variable debe ser `mut`.", true, "`let mut p = Point { … };`"),
                ("`Self` es otro nombre para el propio tipo.", true, "Útil en `fn new() -> Self`."),
                ("En Rust, los structs no pueden tener métodos.", false, "Para eso está `impl`."),
                ("`Point { x, y }` funciona si existen variables `x` e `y`.", true, "La forma abreviada de inicializar campos."),
            ]
        ),
    )
}

// ─────────────────────────────── World 11: Enums & match ───────────────────────────────

pub static W_ENUMS: &[Gen] = &[
    match_zahl, unwrap_or, teilen, muenzen, match_vollstaendig, if_let, luecke_some, enum_paare, wetter, bausteine_arm,
];

fn welcher_ausdruck(l: Sprache) -> &'static str {
    tr!(l, "What does the expression give?", "Welchen Wert ergibt der Ausdruck?", "¿Qué valor da la expresión?")
}

fn match_zahl(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let (a, b, c_, d) = (r.zahl(1, 5) * 10, r.zahl(6, 9) * 10, r.zahl(10, 20) * 10, 0);
    let n = r.zahl(0, 14);
    let wert = match n {
        1 => a,
        2 | 3 => b,
        4..=9 => c_,
        _ => d,
    };
    eingabe(
        tr!(l, format!("What is `e` for `n = {n}`?"), format!("Welchen Wert hat `e` für `n = {n}`?"), format!("¿Qué valor tiene `e` para `n = {n}`?")),
        c(format!("let e = match n {{\n    1 => {a},\n    2 | 3 => {b},\n    4..=9 => {c_},\n    _ => {d},\n}};")),
        wert.to_string(),
        tr!(l, "The first matching pattern wins. `_` catches everything else.", "Das erste passende Muster gewinnt. `_` fängt alles andere.", "Gana el primer patrón que encaja. `_` atrapa todo lo demás."),
    )
}

fn unwrap_or(r: &mut Rng, s: u8, l: Sprache) -> Uebung {
    let x = r.zahl(1, 20);
    let d = r.zahl(0, 9);
    let some = r.bool();
    let o = if some { format!("Some({x})") } else { "None".into() };
    if s >= 2 {
        let k = r.zahl(2, 5);
        let w = if some { x * k } else { d };
        eingabe(welcher_ausdruck(l), c(format!("let o: Option<i32> = {o};\no.map(|v| v * {k}).unwrap_or({d})")), w.to_string(), tr!(l, "`map` only acts on `Some`, `unwrap_or` gives the default for `None`.", "`map` wirkt nur auf `Some`, `unwrap_or` liefert bei `None` den Standardwert.", "`map` solo actúa sobre `Some`; `unwrap_or` da el valor por defecto con `None`."))
    } else {
        eingabe(welcher_ausdruck(l), c(format!("let o: Option<i32> = {o};\no.unwrap_or({d})")), if some { x } else { d }.to_string(), tr!(l, "`unwrap_or` returns the content of `Some` or else the default.", "`unwrap_or` gibt den Inhalt von `Some` oder sonst den Standardwert.", "`unwrap_or` devuelve el contenido de `Some` o, si no, el valor por defecto."))
    }
}

fn teilen(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let a = r.zahl(10, 60);
    let b = if r.idx(3) == 0 { 0 } else { r.zahl(2, 9) };
    let richtig = if b == 0 { "`None`".to_string() } else { format!("`Some({})`", a / b) };
    auswahl(
        r,
        tr!(l, format!("What does `divide({a}, {b})` return?"), format!("Was liefert `divide({a}, {b})`?"), format!("¿Qué devuelve `divide({a}, {b})`?")),
        c("fn divide(a: i32, b: i32) -> Option<i32> {\n    if b == 0 { None } else { Some(a / b) }\n}"),
        richtig,
        vec!["`None`".into(), format!("`Some({})`", if b == 0 { a } else { a / b + 1 }), "`Some(0)`".into(), tr!(l, "Crash (panic)", "Absturz (panic)", "Fallo (panic)").into()],
        tr!(l, "When dividing by 0 the function safely returns `None`.", "Bei Division durch 0 gibt die Funktion sicher `None` zurück.", "Al dividir entre 0, la función devuelve `None` con seguridad."),
    )
}

fn muenzen(r: &mut Rng, s: u8, l: Sprache) -> Uebung {
    let arten = [("Five", 5), ("Ten", 10), ("Twenty", 20), ("Fifty", 50)];
    let n = 3 + s as usize;
    let mut summe = 0;
    let mut liste = Vec::new();
    for _ in 0..n {
        let (name, w) = *r.wahl(&arten);
        summe += w;
        liste.push(format!("Coin::{name}"));
    }
    eingabe(
        tr!(l, "How many cents is the total?", "Wie viele Cent ergibt die Summe?", "¿Cuántos céntimos suma en total?"),
        c(format!("fn value(c: &Coin) -> u32 {{\n    match c {{\n        Coin::Five => 5,\n        Coin::Ten => 10,\n        Coin::Twenty => 20,\n        Coin::Fifty => 50,\n    }}\n}}\n\nlet purse = [{}];", liste.join(", "))),
        summe.to_string(),
        tr!(l, "Add up the values of the coins.", "Werte der Münzen zusammenzählen.", "Suma los valores de las monedas."),
    )
}

fn match_vollstaendig(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let frage = tr!(l, "Does this code compile?", "Kompiliert dieser Code?", "¿Compila este código?");
    if r.bool() {
        wf(frage, c("enum Light { Red, Yellow, Green }\n\nlet text = match light {\n    Light::Red => \"Stop\",\n    Light::Green => \"Go\",\n};"), false, tr!(l, "`Light::Yellow` is missing – `match` must cover every case (E0004).", "`Light::Yellow` fehlt – `match` muss alle Fälle abdecken (E0004).", "Falta `Light::Yellow`: `match` debe cubrir todos los casos (E0004)."))
    } else {
        wf(frage, c("enum Light { Red, Yellow, Green }\n\nlet text = match light {\n    Light::Red => \"Stop\",\n    _ => \"Go on\",\n};"), true, tr!(l, "`_` catches all remaining variants.", "`_` fängt alle übrigen Varianten ab.", "`_` atrapa todas las variantes restantes."))
    }
}

fn if_let(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let x = r.zahl(1, 9);
    let some = r.bool();
    let o = if some { format!("Some({x})") } else { "None".into() };
    let (wert, nichts) = tr!(l, ("Value", "Nothing"), ("Wert", "Nichts"), ("Valor", "Nada"));
    let richtig = if some { format!("`{wert}: {x}`") } else { format!("`{nichts}`") };
    auswahl(
        r,
        tr!(l, "What is printed?", "Was wird ausgegeben?", "¿Qué se imprime?"),
        c(format!("let o: Option<i32> = {o};\nif let Some(v) = o {{\n    println!(\"{wert}: {{v}}\");\n}} else {{\n    println!(\"{nichts}\");\n}}")),
        richtig,
        vec![format!("`{wert}: {x}`"), format!("`{nichts}`"), format!("`{wert}: None`"), tr!(l, "Compiler error", "Compiler-Fehler", "Error de compilación").into()],
        tr!(l, "`if let` checks a single pattern.", "`if let` prüft nur ein einziges Muster.", "`if let` comprueba un único patrón."),
    )
}

fn luecke_some(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    luecke(r, was_luecke(l), "let n = match option {\n    ___(x) => x,\n    None => 0,\n};", "Some", &["Ok", "Option", "Value"], tr!(l, "The two variants of `Option` are `Some(value)` and `None`.", "Die beiden Varianten von `Option` heißen `Some(wert)` und `None`.", "Las dos variantes de `Option` son `Some(valor)` y `None`."))
}

fn enum_paare(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    paare(
        r,
        passt_zu(l),
        &[
            ("Option<T>", tr!(l, "a value or nothing", "Wert oder nichts", "un valor o nada")),
            ("Some(x)", tr!(l, "there is a value", "es gibt einen Wert", "hay un valor")),
            ("None", tr!(l, "no value", "kein Wert", "sin valor")),
            ("match", tr!(l, "check patterns", "Muster prüfen", "comprobar patrones")),
            ("_", tr!(l, "everything else", "alles andere", "todo lo demás")),
            ("if let", tr!(l, "check a single pattern", "nur ein Muster prüfen", "comprobar un solo patrón")),
        ],
        4,
    )
}

fn wetter(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let v = r.idx(3);
    let w = r.zahl(0, 40);
    let [sonne, sturm, matsch] = tr!(l, ["Sunglasses", "Storm", "Slush"], ["Sonnenbrille", "Unwetter", "Matsch"], ["Gafas de sol", "Tormenta", "Aguanieve"]);
    let schirm = tr!(l, "Umbrella", "Schirm", "Paraguas");
    let schnee = tr!(l, "cm of snow", "cm Schnee", "cm de nieve");
    let (wert, ausgabe) = match v {
        0 => ("Weather::Sun".to_string(), sonne.to_string()),
        1 if w > 20 => (format!("Weather::Rain({w})"), sturm.to_string()),
        1 => (format!("Weather::Rain({w})"), format!("{schirm}, {w} mm")),
        _ if w == 0 => ("Weather::Snow { cm: 0 }".to_string(), matsch.to_string()),
        _ => (format!("Weather::Snow {{ cm: {w} }}"), format!("{w} {schnee}")),
    };
    let falsche = vec![format!("`{sonne}`"), format!("`{sturm}`"), format!("`{schirm}, {w} mm`"), format!("`{matsch}`"), format!("`{w} {schnee}`")];
    auswahl(
        r,
        tr!(l, format!("What is `text` for `weather = {wert}`?"), format!("Was ergibt `text` für `weather = {wert}`?"), format!("¿Qué vale `text` para `weather = {wert}`?")),
        c(format!("let text = match weather {{\n    Weather::Sun => \"{sonne}\".to_string(),\n    Weather::Rain(mm) if mm > 20 => \"{sturm}\".to_string(),\n    Weather::Rain(mm) => format!(\"{schirm}, {{mm}} mm\"),\n    Weather::Snow {{ cm: 0 }} => \"{matsch}\".to_string(),\n    Weather::Snow {{ cm }} => format!(\"{{cm}} {schnee}\"),\n}};")),
        format!("`{ausgabe}`"),
        falsche,
        tr!(l, "Top to bottom: the first branch whose pattern (and `if` condition) matches wins.", "Von oben nach unten: Der erste Zweig, dessen Muster (und `if`-Bedingung) passt, gewinnt.", "De arriba abajo: gana la primera rama cuyo patrón (y condición `if`) encaja."),
    )
}

fn bausteine_arm(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let k = r.zahl(2, 9);
    bausteine(
        tr!(l, format!("Build the match arm: for `Some(x)` the result should be `x * {k}`."), format!("Baue den match-Zweig: Bei `Some(x)` soll `x * {k}` herauskommen."), format!("Construye la rama del match: para `Some(x)` el resultado debe ser `x * {k}`.")),
        vec!["Some(x)".into(), "=>".into(), format!("x * {k}"), ",".into()],
        &["->", "Ok(x)", ":", "None"],
        tr!(l, "`Pattern => expression,`", "`Muster => Ausdruck,`", "`Patrón => expresión,`"),
    )
}

// ─────────────────────────────── World 12: Vec & HashMap ───────────────────────────────

pub static W_COLLECTIONS: &[Gen] = &[
    vec_ops, kapazitaet, vec_index, hashmap_zaehlen, sortieren, contains_wf, luecke_vec, collection_paare, vec_summe,
    hashmap_ueberschreiben,
];

fn liste(v: &[i64]) -> String {
    v.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ")
}

fn vec_ops(r: &mut Rng, s: u8, l: Sprache) -> Uebung {
    let mut v: Vec<i64> = (0..3).map(|_| r.zahl(1, 9)).collect();
    let mut code = format!("let mut v = vec![{}, {}, {}];\n", v[0], v[1], v[2]);
    for _ in 0..(2 + s) {
        if r.idx(3) == 0 && !v.is_empty() {
            v.pop();
            code.push_str("v.pop();\n");
        } else {
            let x = r.zahl(1, 20);
            v.push(x);
            code.push_str(&format!("v.push({x});\n"));
        }
    }
    let ende = tr!(l, format!("At the end: {v:?}"), format!("Am Ende: {v:?}"), format!("Al final: {v:?}"));
    match r.idx(3) {
        0 => eingabe(tr!(l, "What is `v.len()` at the end?", "Was ist `v.len()` am Ende?", "¿Cuánto vale `v.len()` al final?"), c(code.trim_end()), v.len().to_string(), ende),
        1 => eingabe(tr!(l, "What is the sum of all elements at the end?", "Was ist die Summe aller Elemente am Ende?", "¿Cuál es la suma de todos los elementos al final?"), c(code.trim_end()), v.iter().sum::<i64>().to_string(), ende),
        _ => eingabe(tr!(l, "Which element is last at the end?", "Welches Element steht am Ende ganz hinten?", "¿Qué elemento queda el último al final?"), c(code.trim_end()), v.last().copied().unwrap_or(0).to_string(), tr!(l, format!("`pop` removes the last element. {ende}"), format!("`pop` entfernt das letzte Element. {ende}"), format!("`pop` quita el último elemento. {ende}"))),
    }
}

fn kapazitaet(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let n = r.zahl(1, 40) as usize;
    let mut cap = 0;
    while cap < n {
        cap = if cap == 0 { 4 } else { cap * 2 };
    }
    eingabe(
        tr!(l, format!("An empty `Vec<i32>` gets {n} × `push`. How big is its capacity afterwards?"), format!("Ein leerer `Vec<i32>` bekommt {n}-mal `push`. Wie groß ist danach die Kapazität?"), format!("Un `Vec<i32>` vacío recibe {n} × `push`. ¿Qué capacidad tiene después?")),
        None,
        cap.to_string(),
        tr!(l, "First `push`: room for 4. When full, the capacity doubles: 4 → 8 → 16 → 32 → 64.", "Erster `push`: Platz für 4. Ist er voll, verdoppelt sich die Kapazität: 4 → 8 → 16 → 32 → 64.", "Primer `push`: espacio para 4. Al llenarse, la capacidad se duplica: 4 → 8 → 16 → 32 → 64."),
    )
}

fn vec_index(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let v: Vec<i64> = (0..r.zahl(3, 5)).map(|_| r.zahl(1, 50)).collect();
    let i = r.idx(v.len() + 3);
    let crash = tr!(l, "Crash (panic)", "Absturz (panic)", "Fallo (panic)");
    if r.bool() {
        let richtig = v.get(i).map_or_else(|| "`None`".to_string(), |x| format!("`Some({x})`"));
        auswahl(
            r,
            tr!(l, format!("What does `v.get({i})` return?"), format!("Was liefert `v.get({i})`?"), format!("¿Qué devuelve `v.get({i})`?")),
            c(format!("let v = vec![{}];", liste(&v))),
            richtig,
            vec!["`None`".into(), format!("`Some({})`", v[i % v.len()]), crash.into(), format!("`Some({})`", v[0] + 1)],
            tr!(l, "`get` returns `Some(element)` or `None` for an invalid index.", "`get` gibt `Some(element)` oder bei ungültigem Index `None` zurück.", "`get` devuelve `Some(elemento)` o `None` si el índice no es válido."),
        )
    } else {
        let richtig = v.get(i).map_or_else(|| crash.to_string(), |x| format!("`{x}`"));
        auswahl(
            r,
            tr!(l, format!("What does `v[{i}]` return?"), format!("Was liefert `v[{i}]`?"), format!("¿Qué devuelve `v[{i}]`?")),
            c(format!("let v = vec![{}];", liste(&v))),
            richtig,
            vec![crash.into(), "`None`".into(), format!("`{}`", v[i % v.len()]), format!("`{}`", v[v.len() - 1])],
            tr!(l, "`v[i]` crashes with an invalid index. Indices go from 0 to len − 1.", "`v[i]` stürzt bei einem ungültigen Index ab. Indizes gehen von 0 bis len − 1.", "`v[i]` falla con un índice no válido. Los índices van de 0 a len − 1."),
        )
    }
}

fn hashmap_zaehlen(r: &mut Rng, s: u8, l: Sprache) -> Uebung {
    let pool: [&str; 3] = tr!(l, ["red", "blue", "green"], ["rot", "blau", "grün"], ["rojo", "azul", "verde"]);
    let n = 5 + s as usize * 2;
    let woerter: Vec<&str> = (0..n).map(|_| *r.wahl(&pool)).collect();
    let ziel = *r.wahl(&pool);
    let anzahl = woerter.iter().filter(|w| **w == ziel).count();
    eingabe(
        tr!(l, format!("What does `counter.get(\"{ziel}\")` contain? (Just the number, 0 if missing)"), format!("Was gibt `counter.get(\"{ziel}\")` zurück? (Nur die Zahl, 0 falls nicht vorhanden)"), format!("¿Qué contiene `counter.get(\"{ziel}\")`? (Solo el número, 0 si no está)")),
        c(format!("let text = \"{}\";\nlet mut counter = HashMap::new();\nfor w in text.split_whitespace() {{\n    *counter.entry(w).or_insert(0) += 1;\n}}", woerter.join(" "))),
        anzahl.to_string(),
        tr!(l, format!("“{ziel}” appears {anzahl} times."), format!("„{ziel}“ kommt {anzahl}-mal vor."), format!("«{ziel}» aparece {anzahl} veces.")),
    )
}

fn sortieren(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let mut v: Vec<i64> = (0..5).map(|_| r.zahl(1, 60)).collect();
    let code = format!("let mut v = vec![{}];\nv.sort();", liste(&v));
    v.sort_unstable();
    let i = r.idx(5);
    eingabe(
        tr!(l, format!("Which value is at `v[{i}]` afterwards?"), format!("Welcher Wert steht danach an `v[{i}]`?"), format!("¿Qué valor hay después en `v[{i}]`?")),
        c(code),
        v[i].to_string(),
        tr!(l, format!("Sorted: {v:?}"), format!("Sortiert: {v:?}"), format!("Ordenado: {v:?}")),
    )
}

fn contains_wf(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let v: Vec<i64> = (0..4).map(|_| r.zahl(1, 20)).collect();
    let x = r.zahl(1, 20);
    let ok = v.contains(&x);
    wf(
        tr!(l, format!("`v.contains(&{x})` returns `true`."), format!("`v.contains(&{x})` ergibt `true`."), format!("`v.contains(&{x})` devuelve `true`.")),
        c(format!("let v = vec![{}];", liste(&v))),
        ok,
        if ok { tr!(l, format!("{x} is in the vector."), format!("{x} ist im Vektor."), format!("{x} está en el vector.")) } else { tr!(l, format!("{x} is not in the vector."), format!("{x} ist nicht im Vektor."), format!("{x} no está en el vector.")) },
    )
}

fn luecke_vec(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    luecke(r, was_luecke(l), "let mut v = ___![1, 2, 3];\nv.push(4);", "vec", &["Vec", "array", "list"], tr!(l, "The macro `vec!` creates a vector with initial values.", "Das Makro `vec!` erzeugt einen Vektor mit Startwerten.", "La macro `vec!` crea un vector con valores iniciales."))
}

fn collection_paare(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    paare(
        r,
        tr!(l, "Match the methods.", "Ordne die Methoden zu.", "Relaciona los métodos."),
        &[
            ("push", tr!(l, "append at the end", "hinten anhängen", "añadir al final")),
            ("pop", tr!(l, "remove the last one", "letztes entfernen", "quitar el último")),
            ("len", tr!(l, "number of elements", "Anzahl der Elemente", "número de elementos")),
            ("get", tr!(l, "safe access with Option", "sicherer Zugriff mit Option", "acceso seguro con Option")),
            ("insert", tr!(l, "store key and value", "Schlüssel und Wert speichern", "guardar clave y valor")),
            ("entry", tr!(l, "get or create an entry", "Eintrag holen oder anlegen", "obtener o crear una entrada")),
        ],
        4,
    )
}

fn vec_summe(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let v: Vec<i64> = (0..r.zahl(3, 6)).map(|_| r.zahl(1, 30)).collect();
    let k = r.zahl(2, 3);
    eingabe(
        tr!(l, "What is the value of `total`?", "Welchen Wert hat `total`?", "¿Qué valor tiene `total`?"),
        c(format!("let mut v = vec![{}];\nfor x in &mut v {{\n    *x *= {k};\n}}\nlet total: i32 = v.iter().sum();", liste(&v))),
        (v.iter().sum::<i64>() * k).to_string(),
        tr!(l, format!("Every element is multiplied by {k}, then everything is added up."), format!("Jedes Element wird mit {k} multipliziert, dann wird addiert."), format!("Cada elemento se multiplica por {k} y luego se suma todo.")),
    )
}

fn hashmap_ueberschreiben(r: &mut Rng, _: u8, l: Sprache) -> Uebung {
    let n = *r.wahl(NAMEN);
    let (a, b) = (r.zahl(1, 50), r.zahl(51, 99));
    eingabe(
        tr!(l, format!("What does `points[\"{n}\"]` return?"), format!("Was liefert `points[\"{n}\"]`?"), format!("¿Qué devuelve `points[\"{n}\"]`?")),
        c(format!("let mut points = HashMap::new();\npoints.insert(\"{n}\", {a});\npoints.insert(\"{n}\", {b});")),
        b.to_string(),
        tr!(l, "`insert` with an existing key overwrites the old value.", "`insert` mit einem vorhandenen Schlüssel überschreibt den alten Wert.", "`insert` con una clave existente sobrescribe el valor anterior."),
    )
}
