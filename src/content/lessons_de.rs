//! All lessons in German. Every lesson builds on the previous one.

use super::Status::{Freigegeben as F, Geliehen as G, Gueltig as OK, Neu as N, Verschoben as V};
use super::{Aufgabe, Block, Lektion, SpeicherSchritt, Visual, hb, sp, sv, ts};

pub static LEKTIONEN_DE: &[Lektion] = &[
    // ─────────────────────────────── Kapitel 1 ───────────────────────────────
    Lektion {
        titel: "Hallo, Rust!",
        kurz: "Dein erstes Programm, der Compiler und Cargo.",
        kapitel: "Erste Schritte",
        bloecke: &[
            Block::Text("Willkommen! **Rust** ist eine moderne Programmiersprache, die so schnell ist wie C oder C++, dich aber vor typischen Fehlern schützt: Abstürze durch ungültigen Speicher, vergessene Fehlerfälle oder gleichzeitige Zugriffe. Der Trick: Der **Compiler** prüft dein Programm sehr genau, bevor es überhaupt läuft."),
            Block::Text("Das Maskottchen der Rust-Community ist **Ferris**, eine kleine Krabbe 🦀. Rust-Programmierer nennen sich übrigens gerne „Rustaceans“."),
            Block::Ueberschrift("Dein erstes Programm"),
            Block::Text("Jedes Rust-Programm startet in der Funktion `main`. Drück auf **Ausführen** – der Code wird wirklich kompiliert und ausgeführt. Du kannst ihn auch ändern!"),
            Block::Ausfuehrbar(r#"fn main() {
    // Das ist ein Kommentar – der Compiler ignoriert ihn.
    println!("Hallo, Welt!");
    println!("Ich lerne Rust. 🦀");
}"#),
            Block::Text("Was passiert hier? `fn` leitet eine Funktion ein, `main` ist ihr Name, und die geschweiften Klammern `{ }` umschließen ihren Inhalt. `println!` gibt eine Zeile Text aus. Jede Anweisung endet mit einem Semikolon `;`."),
            Block::Tipp("`println!` ist keine normale Funktion, sondern ein **Makro** – das erkennst du am Ausrufezeichen `!`. Makros sind kleine Code-Generatoren. Für den Moment reicht: Mit `!` aufrufen."),
            Block::Ueberschrift("Vom Code zum Programm"),
            Block::Text("Rust ist eine **kompilierte** Sprache. Dein Text wird in Maschinencode übersetzt, den der Prozessor direkt versteht. Klick dich durch die Schritte:"),
            Block::Visual(Visual::Pipeline),
            Block::Ueberschrift("Cargo – dein Werkzeugkasten"),
            Block::Text("In der Praxis rufst du den Compiler `rustc` selten direkt auf. Stattdessen nutzt du **Cargo**: Es legt Projekte an, verwaltet Bibliotheken (sogenannte **Crates**) und baut dein Programm. Ein neues Projekt sieht so aus:"),
            Block::Code(r#"$ cargo new hallo
$ tree hallo
hallo
├── Cargo.toml      // Projekt-Infos & Abhängigkeiten
└── src
    └── main.rs     // dein Code"#),
            Block::Text("Mit `cargo run` wird das Projekt kompiliert und gestartet. Probier es im Terminal aus! Tippe nacheinander `cargo new hallo`, `cd hallo`, `cat src/main.rs` und `cargo run`."),
            Block::Terminal,
        ],
        aufgaben: &[
            Aufgabe::Quiz {
                id: "1.1",
                frage: "Welche Funktion wird beim Start eines Rust-Programms automatisch ausgeführt?",
                code: None,
                optionen: &["`start()`", "`main()`", "`run()`", "`init()`"],
                richtig: 1,
                erklaerung: "Genau! `fn main()` ist der Einstiegspunkt jedes ausführbaren Rust-Programms.",
            },
            Aufgabe::Quiz {
                id: "1.2",
                frage: "Woran erkennst du, dass `println!` ein Makro und keine Funktion ist?",
                code: None,
                optionen: &["Am Ausrufezeichen `!`", "An den runden Klammern", "Am Semikolon am Ende", "Das kann man nicht erkennen"],
                richtig: 0,
                erklaerung: "Makros werden in Rust immer mit `!` aufgerufen, z.B. `println!`, `vec!` oder `format!`.",
            },
            Aufgabe::Terminal {
                id: "1.3",
                aufgabe: "Lege mit Cargo ein Projekt namens `hallo` an, wechsle in den Ordner und führe das Programm aus.",
                ziele: &["cargo new hallo", "cd hallo", "cargo run"],
                hinweis: "Tippe die drei Befehle genau so ins Terminal und drücke jeweils Enter.",
            },
            Aufgabe::Code {
                id: "1.4",
                aufgabe: "Ändere das Programm so, dass es genau `Hallo, Rust!` ausgibt.",
                vorlage: r#"fn main() {
    // Schreib hier deinen Code:

}"#,
                erwartet: "Hallo, Rust!",
                hinweis: "Benutze `println!(\"...\");` – vergiss die Anführungszeichen und das Semikolon nicht.",
                loesung: r#"fn main() {
    println!("Hallo, Rust!");
}"#,
            },
            Aufgabe::Code {
                id: "1.5",
                aufgabe: "Gib die drei Wörter `Rust`, `ist` und `schnell` jeweils in einer eigenen Zeile aus.",
                vorlage: r#"fn main() {
    println!("Rust");
}"#,
                erwartet: "Rust\nist\nschnell",
                hinweis: "Jeder Aufruf von `println!` erzeugt eine neue Zeile. Du brauchst also drei Aufrufe.",
                loesung: r#"fn main() {
    println!("Rust");
    println!("ist");
    println!("schnell");
}"#,
            },
        ],
    },
    Lektion {
        titel: "Variablen & Veränderbarkeit",
        kurz: "let, mut, Shadowing und Konstanten.",
        kapitel: "Erste Schritte",
        bloecke: &[
            Block::Text("Mit `let` gibst du einem Wert einen Namen. In Rust sind Variablen **standardmäßig unveränderlich** (immutable). Das klingt zuerst seltsam, verhindert aber viele Fehler: Wenn sich ein Wert nicht ändern kann, musst du nicht überall aufpassen, wer ihn ändert."),
            Block::Ausfuehrbar(r#"fn main() {
    let x = 5;
    println!("x ist {x}");
    println!("x plus 1 ist {}", x + 1);
}"#),
            Block::Text("In `println!` setzt du Werte mit geschweiften Klammern ein: `{x}` setzt direkt die Variable ein, `{}` nimmt das nächste Argument nach dem Komma."),
            Block::Ueberschrift("Veränderbar mit mut"),
            Block::Text("Versuchst du, eine normale Variable zu ändern, meldet der Compiler einen Fehler:"),
            Block::Code(r#"let punkte = 10;
punkte = 20; // ❌ error[E0384]: cannot assign twice to immutable variable"#),
            Block::Text("Soll sich eine Variable ändern dürfen, markierst du sie mit `mut` (für „mutable“):"),
            Block::Ausfuehrbar(r#"fn main() {
    let mut punkte = 10;
    println!("Start: {punkte}");
    punkte = 20;
    punkte += 5; // Kurzform für: punkte = punkte + 5
    println!("Jetzt: {punkte}");
}"#),
            Block::Ueberschrift("Shadowing – Namen wiederverwenden"),
            Block::Text("Du kannst mit `let` einen neuen Wert unter demselben Namen anlegen. Die neue Variable **verdeckt** (shadowt) die alte. Das ist etwas anderes als `mut`: Es entsteht eine neue Variable, die sogar einen anderen Typ haben darf."),
            Block::Ausfuehrbar(r#"fn main() {
    let eingabe = "   42   ";
    let eingabe = eingabe.trim();    // &str ohne Leerzeichen
    let eingabe = eingabe.len();     // jetzt eine Zahl!
    println!("Länge: {eingabe}");
}"#),
            Block::Text("Variablen gelten nur innerhalb des Blocks `{ }`, in dem sie angelegt wurden – ihrem **Scope**. Klick dich durch das Beispiel:"),
            Block::Visual(Visual::Scope),
            Block::Ueberschrift("Konstanten"),
            Block::Text("Werte, die sich nie ändern und überall bekannt sein sollen, schreibst du als `const`. Konstanten werden GROSS geschrieben und brauchen immer eine Typangabe. Unterstriche in Zahlen dienen nur der Lesbarkeit."),
            Block::Code(r#"const MAX_PUNKTE: u32 = 100_000;
const PI_UNGEFAEHR: f64 = 3.14159;"#),
        ],
        aufgaben: &[
            Aufgabe::Quiz {
                id: "2.1",
                frage: "Was passiert bei diesem Code?",
                code: Some("let x = 5;\nx = 6;"),
                optionen: &["x ist danach 6", "Compiler-Fehler: x ist nicht veränderbar", "x ist danach 11", "Das Programm stürzt beim Ausführen ab"],
                richtig: 1,
                erklaerung: "Ohne `mut` ist `x` unveränderlich. Der Compiler meldet `E0384` – schon vor dem Ausführen.",
            },
            Aufgabe::Rechnung {
                id: "2.2",
                frage: "Welchen Wert hat `x` am Ende?",
                code: Some("let x = 2;\nlet x = x * 3;\nlet x = x + 1;"),
                antworten: &["7"],
                hinweis: "Rechne Zeile für Zeile. Jedes `let x` nimmt den vorherigen Wert von `x`.",
                erklaerung: "2 → 2 × 3 = 6 → 6 + 1 = 7. Jede Zeile erzeugt per Shadowing eine neue Variable `x`.",
            },
            Aufgabe::Rechnung {
                id: "2.3",
                frage: "Was gibt das Programm aus?",
                code: Some("let a = 10;\n{\n    let a = a * 2;\n    let b = a + 1;\n}\nprintln!(\"{a}\");"),
                antworten: &["10"],
                hinweis: "Das innere `a` existiert nur innerhalb der geschweiften Klammern.",
                erklaerung: "Das innere `a` (20) verdeckt das äußere nur im Block. Nach `}` ist wieder das äußere `a = 10` sichtbar.",
            },
            Aufgabe::Code {
                id: "2.4",
                aufgabe: "Das Programm kompiliert nicht. Repariere es, sodass `Zähler: 2` ausgegeben wird.",
                vorlage: r#"fn main() {
    let zaehler = 0;
    zaehler += 1;
    zaehler += 1;
    println!("Zähler: {zaehler}");
}"#,
                erwartet: "Zähler: 2",
                hinweis: "Die Variable muss veränderbar sein. Welches Schlüsselwort fehlt nach `let`?",
                loesung: r#"fn main() {
    let mut zaehler = 0;
    zaehler += 1;
    zaehler += 1;
    println!("Zähler: {zaehler}");
}"#,
            },
            Aufgabe::Code {
                id: "2.5",
                aufgabe: "Nutze **Shadowing**: Überschatte die Variable `wort` mit ihrer Länge (`wort.len()`), sodass `Länge: 6` ausgegeben wird. Ändere die `println!`-Zeile nicht.",
                vorlage: r#"fn main() {
    let wort = "Ferris";
    // Neue Zeile hier:

    println!("Länge: {wort}");
}"#,
                erwartet: "Länge: 6",
                hinweis: "Schreibe `let wort = wort.len();` vor die Ausgabe.",
                loesung: r#"fn main() {
    let wort = "Ferris";
    let wort = wort.len();
    println!("Länge: {wort}");
}"#,
            },
        ],
    },
    Lektion {
        titel: "Zahlen & Datentypen",
        kurz: "Ganzzahlen, Kommazahlen, Bits, Überlauf und mehr.",
        kapitel: "Erste Schritte",
        bloecke: &[
            Block::Text("Rust ist **statisch typisiert**: Jeder Wert hat einen festen Typ, den der Compiler kennt. Meistens errät er ihn selbst (Typinferenz), du kannst ihn aber auch angeben: `let x: i32 = 5;`"),
            Block::Ueberschrift("Ganzzahlen"),
            Block::Text("Ganzzahl-Typen beginnen mit `i` (mit Vorzeichen, also auch negativ) oder `u` (unsigned, nur ≥ 0). Die Zahl dahinter sagt, wie viele **Bits** der Typ belegt. Ein `u8` hat 8 Bits und kann daher 2⁸ = 256 verschiedene Werte speichern: 0 bis 255."),
            Block::Visual(Visual::Typgroessen),
            Block::Text("Spiel mit den Bits! Klicke auf einzelne Bits, wechsle den Typ und probiere, was bei `MAX + 1` passiert. Beachte: Bei `i8` zählt das oberste Bit **negativ** (−128). So funktioniert das **Zweierkomplement**."),
            Block::Visual(Visual::Bits),
            Block::Ueberschrift("Rechnen"),
            Block::Text("Die Grundrechenarten sind `+ - * /` und `%` (Rest einer Division). Achtung: Bei Ganzzahlen schneidet `/` die Nachkommastellen einfach ab! `7 / 2` ist `3`, nicht `3.5`. Für Kommazahlen gibt es `f32` und `f64` (Standard)."),
            Block::Ausfuehrbar(r#"fn main() {
    let a = 7;
    let b = 2;
    println!("{a} / {b} = {}", a / b);   // Ganzzahl-Division
    println!("{a} % {b} = {}", a % b);   // Rest
    println!("7.0 / 2.0 = {}", 7.0 / 2.0);

    // Typen mischen geht nicht direkt – mit `as` umwandeln:
    let schnitt = (a + b) as f64 / 2.0;
    println!("Schnitt: {schnitt}");
    println!("Gerundet auf 2 Stellen: {:.2}", 10.0 / 3.0);
}"#),
            Block::Achtung("**Überlauf:** Ein `u8` kann nicht größer als 255 werden. Im Debug-Modus bricht Rust mit einem **panic** ab, wenn du darüber hinaus rechnest. Willst du ein bestimmtes Verhalten, nutze `wrapping_add` (läuft rundherum), `checked_add` (gibt `None` zurück) oder `saturating_add` (bleibt beim Maximum)."),
            Block::Ausfuehrbar(r#"fn main() {
    let x: u8 = 250;
    println!("wrapping:   {}", x.wrapping_add(10));   // 260 - 256 = 4
    println!("checked:    {:?}", x.checked_add(10));  // None
    println!("saturating: {}", x.saturating_add(10)); // 255
    println!("u8 geht bis {}", u8::MAX);
    println!("i32 geht von {} bis {}", i32::MIN, i32::MAX);
}"#),
            Block::Ueberschrift("Weitere einfache Typen"),
            Block::Code(r#"let wahr: bool = true;               // true oder false
let buchstabe: char = 'ß';            // ein Unicode-Zeichen, einfache Anführungszeichen
let paar: (i32, f64) = (3, 2.5);      // Tupel: feste Anzahl, gemischte Typen
let erstes = paar.0;                  // Zugriff mit .0, .1, ...
let zahlen: [i32; 4] = [1, 2, 3, 4];  // Array: feste Länge, ein Typ
let zweite = zahlen[1];               // Index beginnt bei 0!
let binaer = 0b1010;                  // = 10
let hex = 0xFF;                       // = 255"#),
        ],
        aufgaben: &[
            Aufgabe::Rechnung {
                id: "3.1",
                frage: "Welchen Wert hat `x`?",
                code: Some("let x: i32 = 17 / 5;"),
                antworten: &["3"],
                hinweis: "Ganzzahl-Division schneidet den Rest ab. Wie oft passt 5 in 17?",
                erklaerung: "17 / 5 = 3 Rest 2. Bei Ganzzahlen bleibt nur die 3 übrig.",
            },
            Aufgabe::Rechnung {
                id: "3.2",
                frage: "Welchen Wert hat `rest`?",
                code: Some("let rest = 17 % 5;"),
                antworten: &["2"],
                hinweis: "`%` liefert den Rest der Division.",
                erklaerung: "17 = 3 × 5 + 2, also ist der Rest 2.",
            },
            Aufgabe::Rechnung {
                id: "3.3",
                frage: "Was ist der größte Wert, den ein `u16` speichern kann?",
                code: None,
                antworten: &["65535", "65 535", "65'535"],
                hinweis: "Ein `u16` hat 16 Bits. Der größte Wert ist 2¹⁶ − 1.",
                erklaerung: "2¹⁶ = 65 536 Werte, von 0 bis 65 535.",
            },
            Aufgabe::Rechnung {
                id: "3.4",
                frage: "Was gibt das Programm aus?",
                code: Some("let x: u8 = 250;\nprintln!(\"{}\", x.wrapping_add(10));"),
                antworten: &["4"],
                hinweis: "Bei `wrapping_add` fängt der Wert nach 255 wieder bei 0 an. 250 + 10 = 260 …",
                erklaerung: "260 passt nicht in einen u8. 260 − 256 = 4. Der Wert „wickelt“ sich einmal herum.",
            },
            Aufgabe::Rechnung {
                id: "3.5",
                frage: "Welchen Dezimalwert hat diese Binärzahl? (Tipp: Nutze die Bit-Grafik oben!)",
                code: Some("let b: u8 = 0b1010_0101;"),
                antworten: &["165"],
                hinweis: "Addiere die Stellenwerte aller 1-Bits: 128, 64, 32, 16, 8, 4, 2, 1.",
                erklaerung: "128 + 32 + 4 + 1 = 165.",
            },
            Aufgabe::Rechnung {
                id: "3.6",
                frage: "Was ist der kleinste Wert eines `i8`?",
                code: None,
                antworten: &["-128", "−128"],
                hinweis: "Bei `i8` zählt das oberste Bit −2⁷.",
                erklaerung: "Ein i8 reicht von −2⁷ = −128 bis 2⁷ − 1 = 127.",
            },
            Aufgabe::Quiz {
                id: "3.7",
                frage: "Was passiert bei diesem Code?",
                code: Some("let a: i32 = 5;\nlet b: f64 = 2.5;\nlet c = a + b;"),
                optionen: &["c ist 7.5", "c ist 7", "Compiler-Fehler: i32 und f64 kann man nicht addieren", "c ist \"52.5\""],
                richtig: 2,
                erklaerung: "Rust wandelt Typen nie heimlich um. Du musst explizit `a as f64 + b` schreiben.",
            },
            Aufgabe::Code {
                id: "3.8",
                aufgabe: "Berechne den Durchschnitt der fünf Noten als Kommazahl und gib ihn mit **einer** Nachkommastelle aus: `Durchschnitt: 6.6`",
                vorlage: r#"fn main() {
    let summe = 7 + 9 + 4 + 8 + 5;   // i32
    let anzahl = 5;
    // Wandle mit `as f64` um und gib mit {:.1} aus:

}"#,
                erwartet: "Durchschnitt: 6.6",
                hinweis: "`let schnitt = summe as f64 / anzahl as f64;` und dann `println!(\"Durchschnitt: {:.1}\", schnitt);`",
                loesung: r#"fn main() {
    let summe = 7 + 9 + 4 + 8 + 5;
    let anzahl = 5;
    let schnitt = summe as f64 / anzahl as f64;
    println!("Durchschnitt: {:.1}", schnitt);
}"#,
            },
        ],
    },
    Lektion {
        titel: "Funktionen",
        kurz: "Parameter, Rückgabewerte, Ausdrücke und Anweisungen.",
        kapitel: "Erste Schritte",
        bloecke: &[
            Block::Text("Funktionen fassen Code zusammen, damit du ihn wiederverwenden kannst. Sie werden mit `fn` definiert. **Parameter brauchen immer einen Typ**, und der Rückgabetyp steht hinter dem Pfeil `->`."),
            Block::Ausfuehrbar(r#"fn quadrat(x: i32) -> i32 {
    x * x   // kein Semikolon → das ist der Rückgabewert
}

fn begruessen(name: &str) {
    println!("Hallo, {name}!");
}

fn main() {
    begruessen("Ferris");
    let q = quadrat(7);
    println!("7² = {q}");
}"#),
            Block::Ueberschrift("Ausdrücke vs. Anweisungen"),
            Block::Text("Das ist ein wichtiges Rust-Konzept: Eine **Anweisung** (statement) tut etwas, liefert aber keinen Wert – sie endet mit `;`. Ein **Ausdruck** (expression) ergibt einen Wert. Die letzte Zeile einer Funktion ohne `;` ist ihr Rückgabewert. Sogar ein Block `{ }` ist ein Ausdruck:"),
            Block::Ausfuehrbar(r#"fn main() {
    let y = {
        let x = 3;
        x + 1       // ← Wert des Blocks
    };
    println!("y = {y}");
}"#),
            Block::Achtung("Ein Semikolon zu viel ist ein typischer Anfängerfehler: `fn f() -> i32 { 5; }` kompiliert nicht, weil `5;` eine Anweisung ist und die Funktion damit `()` (nichts) zurückgibt. Der Compiler meldet `E0308: mismatched types`."),
            Block::Text("Verfolge, wie Werte in Funktionen hinein- und wieder herauskommen:"),
            Block::Visual(Visual::Trace {
                code: r#"fn doppelt(x: i32) -> i32 {
    x * 2
}
fn main() {
    let a = 3;
    let b = doppelt(a);
    let c = doppelt(b) + 1;
    println!("{c}");
}"#,
                spalten: &["a", "b", "c", "x (in doppelt)"],
                schritte: &[
                    ts(5, &["3", "–", "–", "–"], "`a` bekommt den Wert 3."),
                    ts(2, &["3", "–", "–", "3"], "`doppelt(a)` wird aufgerufen: Der Parameter `x` erhält eine Kopie von `a`, also 3. Die Funktion rechnet 3 × 2."),
                    ts(6, &["3", "6", "–", "–"], "Der Rückgabewert 6 landet in `b`. `x` existiert nicht mehr."),
                    ts(2, &["3", "6", "–", "6"], "Zweiter Aufruf: diesmal ist `x` = 6, Ergebnis 12."),
                    ts(7, &["3", "6", "13", "–"], "12 + 1 = 13 wird in `c` gespeichert."),
                    ts(8, &["3", "6", "13", "–"], "Ausgabe: **13**"),
                ],
            }),
        ],
        aufgaben: &[
            Aufgabe::Quiz {
                id: "4.1",
                frage: "Warum kompiliert diese Funktion nicht?",
                code: Some("fn fuenf() -> i32 {\n    5;\n}"),
                optionen: &["Weil 5 kein i32 ist", "Weil `5;` eine Anweisung ist und nichts zurückgibt", "Weil die Funktion Parameter braucht", "Sie kompiliert ohne Probleme"],
                richtig: 1,
                erklaerung: "Das Semikolon macht aus dem Ausdruck `5` eine Anweisung. Ohne `;` wäre `5` der Rückgabewert.",
            },
            Aufgabe::Rechnung {
                id: "4.2",
                frage: "Welchen Wert liefert `f(f(3))`?",
                code: Some("fn f(x: i32) -> i32 {\n    x * 2 + 1\n}"),
                antworten: &["15"],
                hinweis: "Rechne von innen nach außen: zuerst `f(3)`, dann `f` vom Ergebnis.",
                erklaerung: "f(3) = 3 × 2 + 1 = 7, f(7) = 7 × 2 + 1 = 15.",
            },
            Aufgabe::Code {
                id: "4.3",
                aufgabe: "Vervollständige die Funktion `flaeche`, sodass sie Breite × Höhe zurückgibt. Erwartete Ausgabe: `Fläche: 24`",
                vorlage: r#"fn flaeche(breite: u32, hoehe: u32) -> u32 {
    // TODO: berechne die Fläche
    0
}

fn main() {
    println!("Fläche: {}", flaeche(4, 6));
}"#,
                erwartet: "Fläche: 24",
                hinweis: "Ersetze die `0` durch `breite * hoehe` – ohne Semikolon!",
                loesung: r#"fn flaeche(breite: u32, hoehe: u32) -> u32 {
    breite * hoehe
}

fn main() {
    println!("Fläche: {}", flaeche(4, 6));
}"#,
            },
            Aufgabe::Code {
                id: "4.4",
                aufgabe: "Schreibe die Funktion `zu_fahrenheit`. Formel: °F = °C × 9 / 5 + 32. Achte darauf, mit Kommazahlen zu rechnen (`9.0`, `5.0`, `32.0`).",
                vorlage: r#"fn zu_fahrenheit(celsius: f64) -> f64 {
    // TODO
    0.0
}

fn main() {
    for c in [0.0, 37.0, 100.0] {
        println!("{c}°C = {:.1}°F", zu_fahrenheit(c));
    }
}"#,
                erwartet: "0°C = 32.0°F\n37°C = 98.6°F\n100°C = 212.0°F",
                hinweis: "Die Funktion sollte `celsius * 9.0 / 5.0 + 32.0` zurückgeben.",
                loesung: r#"fn zu_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn main() {
    for c in [0.0, 37.0, 100.0] {
        println!("{c}°C = {:.1}°F", zu_fahrenheit(c));
    }
}"#,
            },
        ],
    },
    // ─────────────────────────────── Kapitel 2 ───────────────────────────────
    Lektion {
        titel: "Entscheidungen mit if",
        kurz: "Bedingungen, Vergleiche und if als Ausdruck.",
        kapitel: "Kontrollfluss",
        bloecke: &[
            Block::Text("Mit `if` entscheidet dein Programm, welcher Code ausgeführt wird. Die Bedingung muss in Rust **immer ein `bool`** sein – anders als in manchen Sprachen gilt eine Zahl nicht automatisch als wahr oder falsch."),
            Block::Ausfuehrbar(r#"fn main() {
    let temperatur = 23;

    if temperatur > 30 {
        println!("Heiß! 🥵");
    } else if temperatur >= 15 {
        println!("Angenehm. 😊");
    } else {
        println!("Kalt! 🥶");
    }
}"#),
            Block::Text("**Vergleiche:** `==` gleich, `!=` ungleich, `<`, `>`, `<=`, `>=`. **Logik:** `&&` (und), `||` (oder), `!` (nicht)."),
            Block::Ueberschrift("if ist ein Ausdruck"),
            Block::Text("Weil `if` einen Wert ergibt, kannst du es direkt in `let` verwenden. Beide Zweige müssen dann denselben Typ haben. Einen extra Operator `? :` wie in anderen Sprachen braucht Rust deshalb nicht."),
            Block::Ausfuehrbar(r#"fn main() {
    let zahl = 7;
    let art = if zahl % 2 == 0 { "gerade" } else { "ungerade" };
    println!("{zahl} ist {art}");

    let alter = 17;
    let darf_waehlen = alter >= 18 && alter < 150;
    println!("Darf wählen: {darf_waehlen}");
}"#),
        ],
        aufgaben: &[
            Aufgabe::Quiz {
                id: "5.1",
                frage: "Kompiliert dieser Code?",
                code: Some("let n = 3;\nif n {\n    println!(\"wahr\");\n}"),
                optionen: &["Ja, 3 gilt als wahr", "Nein, die Bedingung muss ein bool sein", "Ja, aber es wird nichts ausgegeben", "Nein, es fehlt ein else"],
                richtig: 1,
                erklaerung: "Rust erwartet ein `bool`. Du müsstest z.B. `if n != 0` schreiben.",
            },
            Aufgabe::Rechnung {
                id: "5.2",
                frage: "Welchen Wert hat `b`?",
                code: Some("let a = 7;\nlet b = if a % 2 == 0 { a / 2 } else { 3 * a + 1 };"),
                antworten: &["22"],
                hinweis: "Ist 7 gerade? Dann wird der else-Zweig ausgeführt.",
                erklaerung: "7 ist ungerade, also 3 × 7 + 1 = 22. (Das ist übrigens ein Schritt der berühmten Collatz-Folge!)",
            },
            Aufgabe::Rechnung {
                id: "5.3",
                frage: "Was ergibt dieser Ausdruck? (Antworte mit `true` oder `false`)",
                code: Some("(5 > 3) && !(2 == 2) || (10 % 3 == 1)"),
                antworten: &["true"],
                hinweis: "`&&` bindet stärker als `||`. Werte zuerst die einzelnen Klammern aus.",
                erklaerung: "(true && false) || true → false || true → **true**. 10 % 3 ist 1.",
            },
            Aufgabe::Code {
                id: "5.4",
                aufgabe: "Schreibe die Funktion `wetter`: unter 10 Grad → `kalt`, von 10 bis einschließlich 25 → `angenehm`, darüber → `heiß`.",
                vorlage: r#"fn wetter(grad: i32) -> &'static str {
    // TODO: if / else if / else
    "?"
}

fn main() {
    println!("5: {}", wetter(5));
    println!("18: {}", wetter(18));
    println!("25: {}", wetter(25));
    println!("30: {}", wetter(30));
}"#,
                erwartet: "5: kalt\n18: angenehm\n25: angenehm\n30: heiß",
                hinweis: "`if grad < 10 { \"kalt\" } else if grad <= 25 { \"angenehm\" } else { \"heiß\" }` – ohne Semikolon, dann ist es der Rückgabewert.",
                loesung: r#"fn wetter(grad: i32) -> &'static str {
    if grad < 10 {
        "kalt"
    } else if grad <= 25 {
        "angenehm"
    } else {
        "heiß"
    }
}

fn main() {
    println!("5: {}", wetter(5));
    println!("18: {}", wetter(18));
    println!("25: {}", wetter(25));
    println!("30: {}", wetter(30));
}"#,
            },
        ],
    },
    Lektion {
        titel: "Schleifen",
        kurz: "loop, while, for und Bereiche.",
        kapitel: "Kontrollfluss",
        bloecke: &[
            Block::Text("Rust hat drei Arten von Schleifen: `loop` (endlos, bis `break`), `while` (solange eine Bedingung gilt) und `for` (über eine Folge von Werten). Am häufigsten benutzt du `for`."),
            Block::Ausfuehrbar(r#"fn main() {
    // for über einen Bereich: 1, 2, 3, 4, 5
    for i in 1..=5 {
        print!("{i} ");
    }
    println!();

    // while: Countdown
    let mut n = 3;
    while n > 0 {
        println!("{n}…");
        n -= 1;
    }
    println!("Start! 🚀");

    // loop kann mit break einen Wert zurückgeben
    let mut x = 1;
    let ergebnis = loop {
        x *= 2;
        if x > 100 {
            break x;
        }
    };
    println!("Erste Zweierpotenz über 100: {ergebnis}");
}"#),
            Block::Tipp("`0..5` bedeutet 0, 1, 2, 3, 4 (die 5 ist **nicht** dabei). `0..=5` schließt die 5 mit ein. Mit `(0..5).rev()` läufst du rückwärts. `continue` springt zum nächsten Durchlauf, `break` beendet die Schleife."),
            Block::Ueberschrift("Eine Schleife Schritt für Schritt"),
            Block::Text("Drück auf **Abspielen** und beobachte, wie sich `i` und `summe` in jedem Durchlauf verändern:"),
            Block::Visual(Visual::Trace {
                code: r#"fn main() {
    let mut summe = 0;
    for i in 1..=4 {
        summe += i;
    }
    println!("Summe: {summe}");
}"#,
                spalten: &["i", "summe"],
                schritte: &[
                    ts(2, &["–", "0"], "`summe` startet bei 0. Sie muss `mut` sein, weil sie sich ändert."),
                    ts(3, &["1", "0"], "Erster Durchlauf: `i` ist 1."),
                    ts(4, &["1", "1"], "summe = 0 + 1 = **1**"),
                    ts(3, &["2", "1"], "Zweiter Durchlauf: `i` ist 2."),
                    ts(4, &["2", "3"], "summe = 1 + 2 = **3**"),
                    ts(3, &["3", "3"], "Dritter Durchlauf: `i` ist 3."),
                    ts(4, &["3", "6"], "summe = 3 + 3 = **6**"),
                    ts(3, &["4", "6"], "Vierter Durchlauf: `i` ist 4 (wegen `..=` ist die 4 dabei)."),
                    ts(4, &["4", "10"], "summe = 6 + 4 = **10**"),
                    ts(6, &["–", "10"], "Die Schleife ist fertig. Ausgabe: `Summe: 10`"),
                ],
            }),
        ],
        aufgaben: &[
            Aufgabe::Rechnung {
                id: "6.1",
                frage: "Wie oft wird der Schleifenkörper ausgeführt?",
                code: Some("for i in 3..=7 {\n    println!(\"{i}\");\n}"),
                antworten: &["5"],
                hinweis: "Zähle die Werte: 3, 4, … bis einschließlich 7.",
                erklaerung: "3, 4, 5, 6, 7 – also 5 Durchläufe. Allgemein: `a..=b` hat b − a + 1 Werte.",
            },
            Aufgabe::Rechnung {
                id: "6.2",
                frage: "Was gibt dieses Programm aus? Der junge Carl Friedrich Gauß soll das als Schüler in Sekunden gelöst haben.",
                code: Some("let mut summe = 0;\nfor i in 1..=100 {\n    summe += i;\n}\nprintln!(\"{summe}\");"),
                antworten: &["5050", "5 050", "5'050"],
                hinweis: "Bilde Paare: 1 + 100, 2 + 99, 3 + 98 … Jedes Paar ergibt 101. Wie viele Paare gibt es? Formel: n × (n + 1) / 2",
                erklaerung: "50 Paare × 101 = 5050. Oder mit der Formel: 100 × 101 / 2 = 5050.",
            },
            Aufgabe::Rechnung {
                id: "6.3",
                frage: "Welchen Wert hat `schritte` am Ende?",
                code: Some("let mut n = 1;\nlet mut schritte = 0;\nwhile n < 100 {\n    n *= 3;\n    schritte += 1;\n}"),
                antworten: &["5"],
                hinweis: "Schreib die Werte von `n` auf: 1, 3, 9, … Wann ist n nicht mehr kleiner als 100?",
                erklaerung: "n: 1 → 3 → 9 → 27 → 81 → 243. Nach 5 Schritten ist n = 243 ≥ 100.",
            },
            Aufgabe::Code {
                id: "6.4",
                aufgabe: "Der Klassiker **FizzBuzz**: Gib die Zahlen von 1 bis 15 aus. Bei Vielfachen von 3 gib `Fizz` aus, bei Vielfachen von 5 `Buzz`, bei Vielfachen von beiden `FizzBuzz`.",
                vorlage: r#"fn main() {
    for i in 1..=15 {
        // Tipp: i % 3 == 0 bedeutet "durch 3 teilbar"
        println!("{i}");
    }
}"#,
                erwartet: "1\n2\nFizz\n4\nBuzz\nFizz\n7\n8\nFizz\nBuzz\n11\nFizz\n13\n14\nFizzBuzz",
                hinweis: "Prüfe **zuerst** auf `i % 15 == 0` (FizzBuzz), dann auf 3, dann auf 5, sonst die Zahl.",
                loesung: r#"fn main() {
    for i in 1..=15 {
        if i % 15 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{i}");
        }
    }
}"#,
            },
            Aufgabe::Code {
                id: "6.5",
                aufgabe: "Gib das kleine Einmaleins der 7 aus – aber nur bis zum ersten Ergebnis, das größer als 40 ist (dieses mit eingeschlossen). Nutze `loop` und `break`.",
                vorlage: r#"fn main() {
    let mut i = 1;
    loop {
        let ergebnis = 7 * i;
        println!("7 x {i} = {ergebnis}");
        // TODO: Schleife beenden, wenn ergebnis > 40
        i += 1;
        if i > 20 { break; } // Notbremse
    }
}"#,
                erwartet: "7 x 1 = 7\n7 x 2 = 14\n7 x 3 = 21\n7 x 4 = 28\n7 x 5 = 35\n7 x 6 = 42",
                hinweis: "Füge nach der Ausgabe `if ergebnis > 40 { break; }` ein.",
                loesung: r#"fn main() {
    let mut i = 1;
    loop {
        let ergebnis = 7 * i;
        println!("7 x {i} = {ergebnis}");
        if ergebnis > 40 {
            break;
        }
        i += 1;
    }
}"#,
            },
        ],
    },
    // ─────────────────────────────── Kapitel 3 ───────────────────────────────
    Lektion {
        titel: "Ownership – Besitz",
        kurz: "Stack, Heap, Move und Clone – Rusts Superkraft.",
        kapitel: "Ownership",
        bloecke: &[
            Block::Text("Jetzt kommt das Konzept, das Rust einzigartig macht: **Ownership**. Damit verwaltet Rust Speicher automatisch und sicher – ganz ohne Garbage Collector, der im Hintergrund aufräumt."),
            Block::Ueberschrift("Stack und Heap"),
            Block::Text("Dein Programm hat zwei Speicherbereiche. Der **Stack** ist schnell und ordentlich wie ein Stapel Teller: Werte mit fester, bekannter Größe (`i32`, `bool`, …) kommen oben drauf und werden in umgekehrter Reihenfolge wieder entfernt. Der **Heap** ist ein großes Lager für Daten, deren Größe sich ändern kann – zum Beispiel ein `String`, der wachsen kann. Auf dem Stack liegt dann nur ein **Zeiger** auf die Daten."),
            Block::Ueberschrift("Die drei Regeln"),
            Block::Text("**1.** Jeder Wert hat genau einen Besitzer (Owner). **2.** Es gibt immer nur einen Besitzer gleichzeitig. **3.** Verlässt der Besitzer seinen Scope, wird der Wert aufgeräumt (drop)."),
            Block::Visual(Visual::Speicher {
                code: r#"fn main() {
    let a = 5;
    let b = a;
    let s1 = String::from("hallo");
    let s2 = s1;
    let s3 = s2.clone();
    println!("{a} {b} {s2} {s3}");
}"#,
                schritte: &[
                    SpeicherSchritt { zeile: 2, stack: &[sv("a", "5", N)], heap: &[], text: "`a` ist ein `i32` mit fester Größe (4 Bytes) – er liegt direkt auf dem Stack.", ausgabe: "" },
                    SpeicherSchritt { zeile: 3, stack: &[sv("a", "5", OK), sv("b", "5", N)], heap: &[], text: "Einfache Zahlen sind `Copy`: `b` bekommt eine echte Kopie. `a` bleibt gültig, weil das Kopieren billig ist.", ausgabe: "" },
                    SpeicherSchritt { zeile: 4, stack: &[sv("a", "5", OK), sv("b", "5", OK), sp("s1", "ptr, len 5, cap 5", 0, N)], heap: &[hb("h a l l o", N)], text: "Ein `String` speichert seine Buchstaben auf dem **Heap**. Auf dem Stack liegen nur Zeiger, Länge und Kapazität.", ausgabe: "" },
                    SpeicherSchritt { zeile: 5, stack: &[sv("a", "5", OK), sv("b", "5", OK), sp("s1", "ptr, len 5, cap 5", 0, V), sp("s2", "ptr, len 5, cap 5", 0, N)], heap: &[hb("h a l l o", OK)], text: "**Move!** Nur die Stack-Daten werden kopiert, und `s2` wird der neue Besitzer. `s1` ist ab jetzt **ungültig** – sonst würden am Ende zwei Variablen denselben Speicher freigeben (ein gefährlicher „double free“).", ausgabe: "" },
                    SpeicherSchritt { zeile: 6, stack: &[sv("a", "5", OK), sv("b", "5", OK), sp("s1", "ptr, len 5, cap 5", 0, V), sp("s2", "ptr, len 5, cap 5", 0, OK), sp("s3", "ptr, len 5, cap 5", 1, N)], heap: &[hb("h a l l o", OK), hb("h a l l o", N)], text: "`clone()` macht eine **tiefe Kopie**: Auch die Heap-Daten werden kopiert. `s3` hat seinen eigenen Speicher. Das kostet Zeit – deshalb muss man es ausdrücklich hinschreiben.", ausgabe: "" },
                    SpeicherSchritt { zeile: 7, stack: &[sv("a", "5", OK), sv("b", "5", OK), sp("s1", "ptr, len 5, cap 5", 0, V), sp("s2", "ptr, len 5, cap 5", 0, OK), sp("s3", "ptr, len 5, cap 5", 1, OK)], heap: &[hb("h a l l o", OK), hb("h a l l o", OK)], text: "Wir benutzen `a`, `b`, `s2` und `s3`. `s1` dürfen wir nicht mehr verwenden – der Compiler würde `E0382: borrow of moved value` melden.", ausgabe: "5 5 hallo hallo" },
                    SpeicherSchritt { zeile: 8, stack: &[sv("a", "5", F), sv("b", "5", F), sp("s1", "ptr, len 5, cap 5", 0, V), sp("s2", "ptr, len 5, cap 5", 0, F), sp("s3", "ptr, len 5, cap 5", 1, F)], heap: &[hb("h a l l o", F), hb("h a l l o", F)], text: "Ende des Scopes: Rust ruft automatisch `drop` auf. Jeder Heap-Speicher wird **genau einmal** von seinem Besitzer freigegeben. Kein Leck, kein doppeltes Freigeben.", ausgabe: "" },
                ],
            }),
            Block::Ueberschrift("Ownership und Funktionen"),
            Block::Text("Übergibst du einen `String` an eine Funktion, wird er dorthin **verschoben**. Danach kannst du ihn im Aufrufer nicht mehr benutzen – außer die Funktion gibt ihn zurück:"),
            Block::Ausfuehrbar(r#"fn nimm(s: String) -> String {
    println!("Ich habe: {s}");
    s // Besitz zurückgeben
}

fn main() {
    let text = String::from("Ferris");
    let text = nimm(text); // hin und zurück
    println!("Wieder da: {text}");
}"#),
            Block::Tipp("Ständig Werte hin- und zurückzugeben ist umständlich. In der nächsten Lektion lernst du **Referenzen** kennen: Damit kannst du einen Wert ausleihen, ohne ihn zu besitzen."),
        ],
        aufgaben: &[
            Aufgabe::Quiz {
                id: "7.1",
                frage: "Was gilt nach `let s2 = s1;`, wenn `s1` ein `String` ist?",
                code: None,
                optionen: &["s1 und s2 zeigen beide gültig auf denselben Text", "s1 ist ungültig, s2 ist der neue Besitzer", "Der Text wurde auf dem Heap kopiert", "s2 ist leer"],
                richtig: 1,
                erklaerung: "Das ist ein **Move**. Nur ein Besitzer gleichzeitig – `s1` darf nicht mehr verwendet werden.",
            },
            Aufgabe::Quiz {
                id: "7.2",
                frage: "Welcher dieser Typen ist `Copy` (wird beim Zuweisen kopiert statt verschoben)?",
                code: None,
                optionen: &["`String`", "`Vec<i32>`", "`i32`", "`Box<i32>`"],
                richtig: 2,
                erklaerung: "Einfache Typen mit fester Größe auf dem Stack (`i32`, `f64`, `bool`, `char`, …) sind `Copy`. Typen mit Heap-Daten nicht.",
            },
            Aufgabe::Quiz {
                id: "7.3",
                frage: "In welcher Zeile meldet der Compiler einen Fehler?",
                code: Some("let s = String::from(\"Ferris\");  // Zeile 1\nlet t = s;                       // Zeile 2\nprintln!(\"{t}\");                 // Zeile 3\nprintln!(\"{s}\");                 // Zeile 4"),
                optionen: &["Zeile 2", "Zeile 3", "Zeile 4", "Gar keiner"],
                richtig: 2,
                erklaerung: "In Zeile 2 wird `s` nach `t` verschoben. Zeile 4 versucht `s` zu benutzen → `E0382: borrow of moved value`.",
            },
            Aufgabe::Code {
                id: "7.4",
                aufgabe: "Das Programm kompiliert nicht, weil `name` verschoben wurde. Repariere es mit `.clone()`, sodass `Ferris und Ferris` ausgegeben wird.",
                vorlage: r#"fn main() {
    let name = String::from("Ferris");
    let kopie = name;
    println!("{name} und {kopie}");
}"#,
                erwartet: "Ferris und Ferris",
                hinweis: "Schreibe `let kopie = name.clone();` – dann bekommt `kopie` eine eigene Kopie.",
                loesung: r#"fn main() {
    let name = String::from("Ferris");
    let kopie = name.clone();
    println!("{name} und {kopie}");
}"#,
            },
            Aufgabe::Code {
                id: "7.5",
                aufgabe: "`begruessen` nimmt den String in Besitz, deshalb klappt der zweite Aufruf nicht. Repariere `main` (die Funktion darf bleiben), sodass zweimal `Hallo, Ada!` erscheint.",
                vorlage: r#"fn begruessen(name: String) {
    println!("Hallo, {name}!");
}

fn main() {
    let n = String::from("Ada");
    begruessen(n);
    begruessen(n);
}"#,
                erwartet: "Hallo, Ada!\nHallo, Ada!",
                hinweis: "Übergib beim ersten Aufruf eine Kopie: `begruessen(n.clone());`",
                loesung: r#"fn begruessen(name: String) {
    println!("Hallo, {name}!");
}

fn main() {
    let n = String::from("Ada");
    begruessen(n.clone());
    begruessen(n);
}"#,
            },
        ],
    },
    Lektion {
        titel: "Referenzen & Borrowing",
        kurz: "Werte ausleihen mit & und &mut.",
        kapitel: "Ownership",
        bloecke: &[
            Block::Text("Eine **Referenz** `&x` ist wie ein Zettel mit der Adresse eines Wertes. Du kannst den Wert benutzen, ohne ihn zu besitzen – du **leihst** ihn aus (Borrowing). Der Besitzer behält seinen Wert."),
            Block::Ausfuehrbar(r#"fn laenge(s: &String) -> usize {
    s.len() // nur lesen
}

fn anhaengen(s: &mut String) {
    s.push_str(", Welt"); // verändern
}

fn main() {
    let mut text = String::from("Hallo");
    let n = laenge(&text);       // ausleihen zum Lesen
    anhaengen(&mut text);        // ausleihen zum Ändern
    println!("{text} (vorher {n} Zeichen)");
}"#),
            Block::Ueberschrift("Die Borrowing-Regeln"),
            Block::Text("Zu jedem Zeitpunkt gilt **entweder**: beliebig viele lesende Referenzen `&T` **oder** genau eine veränderbare Referenz `&mut T`. Viele Leser oder ein Schreiber – nie beides. Außerdem müssen Referenzen immer gültig sein: Es kann keine Referenz auf bereits aufgeräumten Speicher geben."),
            Block::Visual(Visual::Speicher {
                code: r#"fn main() {
    let mut s = String::from("Hi");
    let r1 = &s;
    let r2 = &s;
    println!("{r1} {r2}");
    let r3 = &mut s;
    r3.push('!');
    println!("{s}");
}"#,
                schritte: &[
                    SpeicherSchritt { zeile: 2, stack: &[sp("s", "ptr, len 2, cap 2", 0, N)], heap: &[hb("H i", N)], text: "`s` besitzt einen String auf dem Heap.", ausgabe: "" },
                    SpeicherSchritt { zeile: 3, stack: &[sp("s", "ptr, len 2, cap 2", 0, OK), sv("r1", "&s  → zeigt auf s", G)], heap: &[hb("H i", OK)], text: "`r1` leiht `s` zum **Lesen** aus. `s` bleibt der Besitzer.", ausgabe: "" },
                    SpeicherSchritt { zeile: 4, stack: &[sp("s", "ptr, len 2, cap 2", 0, OK), sv("r1", "&s  → zeigt auf s", G), sv("r2", "&s  → zeigt auf s", G)], heap: &[hb("H i", OK)], text: "Mehrere lesende Referenzen gleichzeitig sind erlaubt – Lesen stört niemanden.", ausgabe: "" },
                    SpeicherSchritt { zeile: 5, stack: &[sp("s", "ptr, len 2, cap 2", 0, OK), sv("r1", "&s", F), sv("r2", "&s", F)], heap: &[hb("H i", OK)], text: "Nach ihrer letzten Benutzung endet die Ausleihe von `r1` und `r2` automatisch (der Compiler erkennt das).", ausgabe: "Hi Hi" },
                    SpeicherSchritt { zeile: 6, stack: &[sp("s", "ptr, len 2, cap 2", 0, OK), sv("r1", "&s", F), sv("r2", "&s", F), sv("r3", "&mut s  → zeigt auf s", N)], heap: &[hb("H i", OK)], text: "Jetzt gibt es keine Leser mehr, also darf `r3` **exklusiv** und veränderbar ausleihen.", ausgabe: "" },
                    SpeicherSchritt { zeile: 7, stack: &[sp("s", "ptr, len 3, cap 4", 0, OK), sv("r1", "&s", F), sv("r2", "&s", F), sv("r3", "&mut s  → zeigt auf s", G)], heap: &[hb("H i !", N)], text: "Über `r3` wird der String verändert. Weil kein anderer gleichzeitig liest, ist das sicher.", ausgabe: "" },
                    SpeicherSchritt { zeile: 8, stack: &[sp("s", "ptr, len 3, cap 4", 0, OK), sv("r1", "&s", F), sv("r2", "&s", F), sv("r3", "&mut s", F)], heap: &[hb("H i !", OK)], text: "Die Ausleihe ist vorbei – `s` kann wieder direkt benutzt werden und enthält die Änderung.", ausgabe: "Hi!" },
                ],
            }),
            Block::Text("Dieser Code verletzt die Regel und kompiliert deshalb nicht:"),
            Block::Code(r#"let mut s = String::from("Hi");
let leser = &s;
let schreiber = &mut s;   // ❌ E0502: cannot borrow `s` as mutable
println!("{leser}");      //    because it is also borrowed as immutable"#),
            Block::Tipp("Mit `*` greifst du auf den Wert hinter einer Referenz zu (Dereferenzieren): `*zahl += 1;` erhöht die Zahl, auf die `zahl: &mut i32` zeigt. Bei Methodenaufrufen wie `s.push('!')` macht Rust das automatisch."),
        ],
        aufgaben: &[
            Aufgabe::Quiz {
                id: "8.1",
                frage: "Wie viele veränderbare Referenzen (`&mut`) auf denselben Wert darf es gleichzeitig geben?",
                code: None,
                optionen: &["Beliebig viele", "Genau eine", "Zwei", "Keine – &mut gibt es nicht"],
                richtig: 1,
                erklaerung: "Genau eine – und solange sie existiert, auch keine lesenden Referenzen. So verhindert Rust „Data Races“.",
            },
            Aufgabe::Quiz {
                id: "8.2",
                frage: "Was bedeutet `fn f(x: &String)`?",
                code: None,
                optionen: &["f übernimmt den Besitz des Strings", "f leiht den String nur zum Lesen aus", "f kann den String verändern", "f bekommt eine Kopie"],
                richtig: 1,
                erklaerung: "`&String` ist eine lesende Referenz. Zum Verändern bräuchte man `&mut String`.",
            },
            Aufgabe::Code {
                id: "8.3",
                aufgabe: "Vervollständige `verdoppeln`, sodass die Zahl hinter der Referenz verdoppelt wird. Erwartete Ausgabe: `42`",
                vorlage: r#"fn verdoppeln(zahl: &mut i32) {
    // TODO: Benutze * zum Dereferenzieren
}

fn main() {
    let mut x = 21;
    verdoppeln(&mut x);
    println!("{x}");
}"#,
                erwartet: "42",
                hinweis: "`*zahl *= 2;` – der erste `*` dereferenziert, der zweite ist die Multiplikation.",
                loesung: r#"fn verdoppeln(zahl: &mut i32) {
    *zahl *= 2;
}

fn main() {
    let mut x = 21;
    verdoppeln(&mut x);
    println!("{x}");
}"#,
            },
            Aufgabe::Code {
                id: "8.4",
                aufgabe: "Ändere `ausrufen` so, dass sie den String per `&mut` ausleiht statt ihn zu besitzen. Dann soll `main` danach `Hallo!!!` ausgeben.",
                vorlage: r#"fn ausrufen(s: String) {
    s.push_str("!!!");
}

fn main() {
    let gruss = String::from("Hallo");
    ausrufen(gruss);
    println!("{gruss}");
}"#,
                erwartet: "Hallo!!!",
                hinweis: "Drei Stellen: Parameter `s: &mut String`, Variable `let mut gruss` und Aufruf `ausrufen(&mut gruss)`.",
                loesung: r#"fn ausrufen(s: &mut String) {
    s.push_str("!!!");
}

fn main() {
    let mut gruss = String::from("Hallo");
    ausrufen(&mut gruss);
    println!("{gruss}");
}"#,
            },
        ],
    },
    Lektion {
        titel: "Strings & Slices",
        kurz: "String vs. &str, UTF-8 und Teilstücke.",
        kapitel: "Ownership",
        bloecke: &[
            Block::Text("Rust hat zwei Haupt-Typen für Text: `String` besitzt seinen Text auf dem Heap und kann wachsen. `&str` (ein **String-Slice**) ist eine geliehene Ansicht auf Text – zum Beispiel auf ein Stück eines `String` oder auf einen festen Text im Programm wie `\"Hallo\"`."),
            Block::Ausfuehrbar(r#"fn main() {
    let fest: &str = "Hallo";                 // fest im Programm
    let mut eigen = String::from(fest);       // eigener, veränderbarer Text
    eigen.push_str(", Welt");
    eigen.push('!');

    let teil: &str = &eigen[0..5];            // Slice: Bytes 0 bis 4
    println!("{eigen} | Teil: {teil}");
    println!("Großbuchstaben: {}", eigen.to_uppercase());
    println!("Enthält 'Welt': {}", eigen.contains("Welt"));

    let satz = format!("{} hat {} Bytes", eigen, eigen.len());
    println!("{satz}");

    for wort in "Rust ist toll".split_whitespace() {
        println!("- {wort}");
    }
}"#),
            Block::Ueberschrift("UTF-8: Bytes sind nicht gleich Zeichen"),
            Block::Text("Rust-Strings sind immer gültiges **UTF-8**. Buchstaben wie `a` brauchen 1 Byte, Umlaute wie `ü` oder `ß` aber 2 Bytes und Emojis wie 🦀 sogar 4. `len()` zählt **Bytes**, nicht Zeichen! Für Zeichen nutzt du `chars()`."),
            Block::Ausfuehrbar(r#"fn main() {
    let wort = "Grüße";
    println!("Bytes:   {}", wort.len());
    println!("Zeichen: {}", wort.chars().count());
    for (i, c) in wort.char_indices() {
        println!("Byte {i}: '{c}' ({} Byte)", c.len_utf8());
    }
}"#),
            Block::Achtung("`s[0]` funktioniert bei Strings **nicht** – weil unklar wäre, ob du ein Byte oder ein Zeichen meinst. Und ein Slice wie `&\"Grüße\"[0..3]` stürzt ab, weil Byte 3 mitten im `ü` liegt. Nutze `chars()` oder Methoden wie `find`, `split` und `starts_with`."),
            Block::Tipp("Schreib Funktionen, die Text nur lesen, am besten mit `&str` als Parameter: `fn f(s: &str)`. Dann kannst du sowohl `&String` als auch `\"Text\"` übergeben."),
        ],
        aufgaben: &[
            Aufgabe::Rechnung {
                id: "9.1",
                frage: "Was liefert `\"Grüße\".len()`?",
                code: None,
                antworten: &["7"],
                hinweis: "`len()` zählt Bytes. G, r und e brauchen je 1 Byte – ü und ß je 2.",
                erklaerung: "1 + 1 + 2 + 2 + 1 = 7 Bytes, aber nur 5 Zeichen.",
            },
            Aufgabe::Rechnung {
                id: "9.2",
                frage: "Was liefert `\"Ferris 🦀\".chars().count()`?",
                code: None,
                antworten: &["8"],
                hinweis: "Zähle Zeichen, nicht Bytes: Buchstaben, Leerzeichen und das Emoji.",
                erklaerung: "F-e-r-r-i-s (6) + Leerzeichen (1) + 🦀 (1) = 8 Zeichen. In Bytes wären es 6 + 1 + 4 = 11.",
            },
            Aufgabe::Quiz {
                id: "9.3",
                frage: "Was ist `teil`?",
                code: Some("let s = String::from(\"Ferris\");\nlet teil = &s[1..4];"),
                optionen: &["`\"Fer\"`", "`\"err\"`", "`\"erri\"`", "`\"Ferr\"`"],
                richtig: 1,
                erklaerung: "Index 1 bis ausschließlich 4: Bytes 1, 2, 3 → `e`, `r`, `r`.",
            },
            Aufgabe::Code {
                id: "9.4",
                aufgabe: "Schreibe `erstes_wort`, das das erste Wort eines Textes zurückgibt. Erwartete Ausgabe: `Hallo` und `Rust`",
                vorlage: r#"fn erstes_wort(s: &str) -> &str {
    // TODO: Tipp: s.split_whitespace().next()
    s
}

fn main() {
    println!("{}", erstes_wort("Hallo schöne Welt"));
    println!("{}", erstes_wort("Rust"));
}"#,
                erwartet: "Hallo\nRust",
                hinweis: "`s.split_whitespace().next()` liefert ein `Option<&str>`. Mit `.unwrap_or(\"\")` bekommst du den `&str`.",
                loesung: r#"fn erstes_wort(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

fn main() {
    println!("{}", erstes_wort("Hallo schöne Welt"));
    println!("{}", erstes_wort("Rust"));
}"#,
            },
            Aufgabe::Code {
                id: "9.5",
                aufgabe: "Baue mit einer Schleife den Text `1-2-3-4-5` zusammen und gib ihn aus.",
                vorlage: r#"fn main() {
    let mut text = String::new();
    for i in 1..=5 {
        // TODO: Zahl anhängen, und einen Bindestrich – außer nach der letzten Zahl
    }
    println!("{text}");
}"#,
                erwartet: "1-2-3-4-5",
                hinweis: "`text.push_str(&i.to_string());` hängt die Zahl an. Mit `if i < 5 { text.push('-'); }` den Bindestrich.",
                loesung: r#"fn main() {
    let mut text = String::new();
    for i in 1..=5 {
        text.push_str(&i.to_string());
        if i < 5 {
            text.push('-');
        }
    }
    println!("{text}");
}"#,
            },
        ],
    },
    // ─────────────────────────────── Kapitel 4 ───────────────────────────────
    Lektion {
        titel: "Structs & Methoden",
        kurz: "Eigene Datentypen mit Feldern und Verhalten.",
        kapitel: "Eigene Typen",
        bloecke: &[
            Block::Text("Mit einem `struct` fasst du zusammengehörige Daten zu einem eigenen Typ zusammen. Jedes Feld hat einen Namen und einen Typ. In einem `impl`-Block definierst du **Methoden** – Funktionen, die zu deinem Typ gehören."),
            Block::Ausfuehrbar(r#"#[derive(Debug)] // erlaubt die Ausgabe mit {:?}
struct Rechteck {
    breite: u32,
    hoehe: u32,
}

impl Rechteck {
    // "Assoziierte Funktion" ohne self – wie ein Konstruktor
    fn neu(breite: u32, hoehe: u32) -> Self {
        Self { breite, hoehe }
    }

    // Methode: liest nur (&self)
    fn flaeche(&self) -> u32 {
        self.breite * self.hoehe
    }

    // Methode: verändert (&mut self)
    fn verdoppeln(&mut self) {
        self.breite *= 2;
        self.hoehe *= 2;
    }
}

fn main() {
    let mut r = Rechteck::neu(3, 4);
    println!("{r:?} hat Fläche {}", r.flaeche());
    r.verdoppeln();
    println!("Nach verdoppeln: {}×{} = {}", r.breite, r.hoehe, r.flaeche());
}"#),
            Block::Text("**Merke:** `&self` = Methode liest nur. `&mut self` = Methode verändert das Objekt. `self` = Methode verbraucht das Objekt (Ownership!). Ohne `self` ist es eine assoziierte Funktion, die du mit `Typ::name()` aufrufst."),
            Block::Tipp("Mit `#[derive(Debug)]` erzeugt Rust automatisch Code, um dein Struct auszugeben. `{:?}` zeigt es kompakt, `{:#?}` schön formatiert über mehrere Zeilen."),
        ],
        aufgaben: &[
            Aufgabe::Quiz {
                id: "10.1",
                frage: "Wie rufst du die assoziierte Funktion `neu` von `Rechteck` auf?",
                code: None,
                optionen: &["`Rechteck.neu(3, 4)`", "`Rechteck::neu(3, 4)`", "`neu Rechteck(3, 4)`", "`new Rechteck(3, 4)`"],
                richtig: 1,
                erklaerung: "Assoziierte Funktionen (ohne `self`) ruft man mit `::` am Typ auf. Methoden mit `.` an einer Instanz.",
            },
            Aufgabe::Quiz {
                id: "10.2",
                frage: "Eine Methode soll ein Feld des Structs ändern. Welchen ersten Parameter braucht sie?",
                code: None,
                optionen: &["`self`", "`&self`", "`&mut self`", "Gar keinen"],
                richtig: 2,
                erklaerung: "Zum Ändern braucht die Methode eine veränderbare Referenz auf das Objekt: `&mut self`.",
            },
            Aufgabe::Rechnung {
                id: "10.3",
                frage: "Welchen Wert hat `k.saldo` am Ende?",
                code: Some("struct Konto { saldo: i64 }\n\nimpl Konto {\n    fn einzahlen(&mut self, betrag: i64) { self.saldo += betrag; }\n    fn abheben(&mut self, betrag: i64) {\n        if betrag <= self.saldo { self.saldo -= betrag; }\n    }\n}\n\nlet mut k = Konto { saldo: 100 };\nk.einzahlen(50);\nk.abheben(200);\nk.abheben(30);\nk.einzahlen(5);"),
                antworten: &["125"],
                hinweis: "Achtung bei `abheben(200)`: Die Bedingung `betrag <= self.saldo` muss erfüllt sein.",
                erklaerung: "100 + 50 = 150. 200 abheben geht nicht (zu wenig Geld). 150 − 30 = 120, + 5 = 125.",
            },
            Aufgabe::Code {
                id: "10.4",
                aufgabe: "Implementiere die Methode `umfang` (2 × Breite + 2 × Höhe). Erwartete Ausgabe: `Fläche: 12, Umfang: 14`",
                vorlage: r#"struct Rechteck {
    breite: u32,
    hoehe: u32,
}

impl Rechteck {
    fn flaeche(&self) -> u32 {
        self.breite * self.hoehe
    }

    fn umfang(&self) -> u32 {
        // TODO
        0
    }
}

fn main() {
    let r = Rechteck { breite: 3, hoehe: 4 };
    println!("Fläche: {}, Umfang: {}", r.flaeche(), r.umfang());
}"#,
                erwartet: "Fläche: 12, Umfang: 14",
                hinweis: "Greife mit `self.breite` und `self.hoehe` auf die Felder zu.",
                loesung: r#"struct Rechteck {
    breite: u32,
    hoehe: u32,
}

impl Rechteck {
    fn flaeche(&self) -> u32 {
        self.breite * self.hoehe
    }

    fn umfang(&self) -> u32 {
        2 * self.breite + 2 * self.hoehe
    }
}

fn main() {
    let r = Rechteck { breite: 3, hoehe: 4 };
    println!("Fläche: {}, Umfang: {}", r.flaeche(), r.umfang());
}"#,
            },
            Aufgabe::Code {
                id: "10.5",
                aufgabe: "Ergänze die Methode `erhoehen`, die `wert` um 1 erhöht. Achte auf den richtigen `self`-Parameter. Erwartete Ausgabe: `Zähler: 3`",
                vorlage: r#"struct Zaehler {
    wert: u32,
}

impl Zaehler {
    // TODO: fn erhoehen(...)
}

fn main() {
    let mut z = Zaehler { wert: 0 };
    z.erhoehen();
    z.erhoehen();
    z.erhoehen();
    println!("Zähler: {}", z.wert);
}"#,
                erwartet: "Zähler: 3",
                hinweis: "`fn erhoehen(&mut self) { self.wert += 1; }`",
                loesung: r#"struct Zaehler {
    wert: u32,
}

impl Zaehler {
    fn erhoehen(&mut self) {
        self.wert += 1;
    }
}

fn main() {
    let mut z = Zaehler { wert: 0 };
    z.erhoehen();
    z.erhoehen();
    z.erhoehen();
    println!("Zähler: {}", z.wert);
}"#,
            },
        ],
    },
    Lektion {
        titel: "Enums & match",
        kurz: "Varianten, Pattern Matching und Option.",
        kapitel: "Eigene Typen",
        bloecke: &[
            Block::Text("Ein `enum` beschreibt einen Wert, der **genau eine** von mehreren Varianten ist. Das Besondere in Rust: Jede Variante kann eigene Daten tragen. Mit `match` prüfst du, welche Variante vorliegt – und der Compiler stellt sicher, dass du **keinen Fall vergisst**."),
            Block::Visual(Visual::Match),
            Block::Ueberschrift("Option – Rust hat kein null"),
            Block::Text("Viele Sprachen haben `null` für „kein Wert“ – eine riesige Fehlerquelle. Rust benutzt stattdessen das Enum `Option<T>`: Entweder `Some(wert)` oder `None`. Du musst den `None`-Fall behandeln, bevor du an den Wert kommst."),
            Block::Ausfuehrbar(r#"fn teilen(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    match teilen(10, 2) {
        Some(x) => println!("Ergebnis: {x}"),
        None => println!("Durch null teilen geht nicht!"),
    }

    // Kurzform, wenn dich nur ein Fall interessiert:
    if let Some(x) = teilen(9, 3) {
        println!("if let: {x}");
    }

    // Standardwert, falls None:
    let y = teilen(1, 0).unwrap_or(-1);
    println!("unwrap_or: {y}");
}"#),
            Block::Ueberschrift("Muster in match"),
            Block::Code(r#"match zahl {
    0 => println!("null"),
    1 | 2 => println!("eins oder zwei"),       // mehrere Werte
    3..=9 => println!("einstellig"),            // Bereich
    n if n < 0 => println!("negativ: {n}"),     // Guard mit if
    _ => println!("irgendwas anderes"),         // alles andere
}"#),
        ],
        aufgaben: &[
            Aufgabe::Quiz {
                id: "11.1",
                frage: "Wofür steht `Option<T>` in Rust?",
                code: None,
                optionen: &["Für eine Einstellung im Programm", "Für einen Wert, der vorhanden sein kann (`Some`) oder nicht (`None`)", "Für einen Fehler", "Für eine optionale Funktion"],
                richtig: 1,
                erklaerung: "Option ersetzt `null`. Der Compiler zwingt dich, den `None`-Fall zu behandeln.",
            },
            Aufgabe::Quiz {
                id: "11.2",
                frage: "Was passiert, wenn ein `match` nicht alle Varianten eines Enums abdeckt?",
                code: None,
                optionen: &["Der fehlende Fall wird übersprungen", "Das Programm stürzt ab, wenn der Fall eintritt", "Der Compiler meldet einen Fehler", "Es wird automatisch der erste Zweig genommen"],
                richtig: 2,
                erklaerung: "`match` muss vollständig (exhaustive) sein – sonst `E0004`. Mit `_ =>` fängst du alle restlichen Fälle ab.",
            },
            Aufgabe::Rechnung {
                id: "11.3",
                frage: "Welchen Wert hat `ergebnis`?",
                code: Some("let zahl = 7;\nlet ergebnis = match zahl {\n    1 => 10,\n    2 | 3 => 20,\n    4..=9 => 30,\n    _ => 0,\n};"),
                antworten: &["30"],
                hinweis: "Welches Muster passt als erstes auf 7?",
                erklaerung: "7 liegt im Bereich `4..=9`, also 30.",
            },
            Aufgabe::Code {
                id: "11.4",
                aufgabe: "Schreibe die Funktion `wert`, die für jede Münze den Wert in Rappen zurückgibt (5, 10, 20, 50). Erwartete Ausgabe: `Summe: 85 Rappen`",
                vorlage: r#"enum Muenze {
    Fuenf,
    Zehn,
    Zwanzig,
    Fuenfzig,
}

fn wert(m: &Muenze) -> u32 {
    // TODO: match m { ... }
    0
}

fn main() {
    let portemonnaie = [Muenze::Zwanzig, Muenze::Fuenfzig, Muenze::Zehn, Muenze::Fuenf];
    let mut summe = 0;
    for m in &portemonnaie {
        summe += wert(m);
    }
    println!("Summe: {summe} Rappen");
}"#,
                erwartet: "Summe: 85 Rappen",
                hinweis: "`match m { Muenze::Fuenf => 5, Muenze::Zehn => 10, ... }`",
                loesung: r#"enum Muenze {
    Fuenf,
    Zehn,
    Zwanzig,
    Fuenfzig,
}

fn wert(m: &Muenze) -> u32 {
    match m {
        Muenze::Fuenf => 5,
        Muenze::Zehn => 10,
        Muenze::Zwanzig => 20,
        Muenze::Fuenfzig => 50,
    }
}

fn main() {
    let portemonnaie = [Muenze::Zwanzig, Muenze::Fuenfzig, Muenze::Zehn, Muenze::Fuenf];
    let mut summe = 0;
    for m in &portemonnaie {
        summe += wert(m);
    }
    println!("Summe: {summe} Rappen");
}"#,
            },
            Aufgabe::Code {
                id: "11.5",
                aufgabe: "Schreibe `quadratwurzel`: Für negative Zahlen `None`, sonst `Some(x.sqrt())`. Behandle in `main` beide Fälle mit `match`.",
                vorlage: r#"fn quadratwurzel(x: f64) -> Option<f64> {
    // TODO
    None
}

fn main() {
    for x in [16.0, -4.0] {
        match quadratwurzel(x) {
            Some(w) => println!("√{x} = {w}"),
            None => println!("√{x} gibt es nicht"),
        }
    }
}"#,
                erwartet: "√16 = 4\n√-4 gibt es nicht",
                hinweis: "`if x < 0.0 { None } else { Some(x.sqrt()) }`",
                loesung: r#"fn quadratwurzel(x: f64) -> Option<f64> {
    if x < 0.0 {
        None
    } else {
        Some(x.sqrt())
    }
}

fn main() {
    for x in [16.0, -4.0] {
        match quadratwurzel(x) {
            Some(w) => println!("√{x} = {w}"),
            None => println!("√{x} gibt es nicht"),
        }
    }
}"#,
            },
        ],
    },
    Lektion {
        titel: "Vec & HashMap",
        kurz: "Listen und Wörterbücher, die wachsen können.",
        kapitel: "Eigene Typen",
        bloecke: &[
            Block::Text("Ein Array hat eine feste Länge. Brauchst du eine Liste, die wachsen und schrumpfen kann, nimmst du einen **`Vec<T>`** (Vektor). Seine Elemente liegen hintereinander auf dem Heap."),
            Block::Text("Probiere `push` und `pop` aus. Achte auf **len** (Anzahl der Elemente) und **cap** (reservierter Platz): Ist der Platz voll, reserviert Rust doppelt so viel neuen Speicher und kopiert alles um."),
            Block::Visual(Visual::Vec),
            Block::Ausfuehrbar(r#"fn main() {
    let mut zahlen = vec![3, 1, 4];
    zahlen.push(1);
    zahlen.push(5);
    println!("{zahlen:?}, Länge {}", zahlen.len());

    // Lesen: mit Index (kann abstürzen) oder get (gibt Option zurück)
    println!("Erstes: {}", zahlen[0]);
    println!("Zehntes: {:?}", zahlen.get(9));

    // Durchlaufen und verändern
    for z in &mut zahlen {
        *z *= 10;
    }
    let summe: i32 = zahlen.iter().sum();
    println!("{zahlen:?}, Summe {summe}");

    zahlen.sort();
    println!("Sortiert: {zahlen:?}");
}"#),
            Block::Ueberschrift("HashMap – Schlüssel und Werte"),
            Block::Text("Eine `HashMap<K, V>` ordnet **Schlüsseln** Werte zu, wie ein Wörterbuch. Besonders praktisch ist `entry(...).or_insert(...)`: „Hole den Eintrag, oder lege ihn mit einem Startwert an.“"),
            Block::Ausfuehrbar(r#"use std::collections::HashMap;

fn main() {
    let mut punkte: HashMap<&str, u32> = HashMap::new();
    punkte.insert("Ada", 10);
    punkte.insert("Linus", 7);

    if let Some(p) = punkte.get("Ada") {
        println!("Ada hat {p} Punkte");
    }

    // Wörter zählen
    let text = "der hund und der ball und der baum";
    let mut anzahl = HashMap::new();
    for wort in text.split_whitespace() {
        *anzahl.entry(wort).or_insert(0) += 1;
    }
    println!("'der' kommt {} mal vor", anzahl["der"]);
    println!("'und' kommt {} mal vor", anzahl["und"]);
}"#),
            Block::Tipp("Die Reihenfolge in einer `HashMap` ist zufällig. Brauchst du sortierte Schlüssel, nimm `BTreeMap` – die Benutzung ist fast identisch."),
        ],
        aufgaben: &[
            Aufgabe::Rechnung {
                id: "12.1",
                frage: "Welchen Wert hat `summe` am Ende?",
                code: Some("let mut v = vec![1, 2, 3];\nv.push(4);\nv.pop();\nv.push(10);\nlet summe: i32 = v.iter().sum();"),
                antworten: &["16"],
                hinweis: "`pop` entfernt das **letzte** Element. Schreib den Vektor nach jedem Schritt auf.",
                erklaerung: "[1,2,3] → [1,2,3,4] → [1,2,3] → [1,2,3,10]. Summe: 16.",
            },
            Aufgabe::Quiz {
                id: "12.2",
                frage: "`v` hat 3 Elemente. Was passiert bei `v[10]` und was bei `v.get(10)`?",
                code: None,
                optionen: &["Beide geben 0 zurück", "`v[10]` stürzt ab (panic), `v.get(10)` gibt `None`", "Beide stürzen ab", "`v[10]` gibt `None`, `v.get(10)` stürzt ab"],
                richtig: 1,
                erklaerung: "Indexzugriff mit `[]` prüft die Grenze und bricht ab. `get` gibt ein `Option` zurück, das du sicher behandeln kannst.",
            },
            Aufgabe::Rechnung {
                id: "12.3",
                frage: "Ein leerer `Vec<i32>` bekommt beim ersten `push` Platz für 4 Elemente und verdoppelt die Kapazität, wenn sie voll ist. Wie groß ist die Kapazität nach 9 `push`-Aufrufen?",
                code: None,
                antworten: &["16"],
                hinweis: "4 → 8 → … Beim 5. und beim 9. Element ist der Platz jeweils voll.",
                erklaerung: "Elemente 1–4: cap 4. Element 5: cap 8. Element 9: cap 16. (Probiere es oben in der Grafik aus!)",
            },
            Aufgabe::Code {
                id: "12.4",
                aufgabe: "Berechne mit einer `for`-Schleife Summe und Maximum der Zahlen. Erwartete Ausgabe: `Summe: 75, Maximum: 42`",
                vorlage: r#"fn main() {
    let zahlen = vec![3, 17, 8, 42, 5];
    let mut summe = 0;
    let mut maximum = 0;
    for z in &zahlen {
        // TODO
    }
    println!("Summe: {summe}, Maximum: {maximum}");
}"#,
                erwartet: "Summe: 75, Maximum: 42",
                hinweis: "`z` ist eine Referenz `&i32`. Benutze `*z`: `summe += *z;` und `if *z > maximum { maximum = *z; }`",
                loesung: r#"fn main() {
    let zahlen = vec![3, 17, 8, 42, 5];
    let mut summe = 0;
    let mut maximum = 0;
    for z in &zahlen {
        summe += *z;
        if *z > maximum {
            maximum = *z;
        }
    }
    println!("Summe: {summe}, Maximum: {maximum}");
}"#,
            },
            Aufgabe::Code {
                id: "12.5",
                aufgabe: "Zähle mit einer `HashMap`, wie oft jede Farbe vorkommt. Erwartete Ausgabe siehe unten.",
                vorlage: r#"use std::collections::HashMap;

fn main() {
    let text = "rot blau rot grün rot blau";
    let mut zaehler: HashMap<&str, u32> = HashMap::new();
    // TODO: für jedes Wort im Text den Zähler erhöhen

    for farbe in ["rot", "blau", "grün"] {
        println!("{farbe}: {}", zaehler.get(farbe).unwrap_or(&0));
    }
}"#,
                erwartet: "rot: 3\nblau: 2\ngrün: 1",
                hinweis: "`for wort in text.split_whitespace() { *zaehler.entry(wort).or_insert(0) += 1; }`",
                loesung: r#"use std::collections::HashMap;

fn main() {
    let text = "rot blau rot grün rot blau";
    let mut zaehler: HashMap<&str, u32> = HashMap::new();
    for wort in text.split_whitespace() {
        *zaehler.entry(wort).or_insert(0) += 1;
    }

    for farbe in ["rot", "blau", "grün"] {
        println!("{farbe}: {}", zaehler.get(farbe).unwrap_or(&0));
    }
}"#,
            },
        ],
    },
    // ─────────────────────────────── Kapitel 5 ───────────────────────────────
    Lektion {
        titel: "Fehlerbehandlung",
        kurz: "panic!, Result und der ?-Operator.",
        kapitel: "Fortgeschritten",
        bloecke: &[
            Block::Text("Rust unterscheidet zwei Arten von Fehlern: **Nicht behebbare** Fehler (ein Programmierfehler, z.B. Index außerhalb) führen zu `panic!` – das Programm bricht ab. **Behebbare** Fehler (z.B. eine falsche Benutzereingabe) werden mit dem Enum `Result<T, E>` zurückgegeben: `Ok(wert)` oder `Err(fehler)`."),
            Block::Visual(Visual::Parse),
            Block::Ausfuehrbar(r#"fn main() {
    let eingaben = ["42", "-7", "drei"];
    for e in eingaben {
        match e.parse::<i32>() {
            Ok(zahl) => println!("{e:>5} → Zahl {zahl}"),
            Err(fehler) => println!("{e:>5} → Fehler: {fehler}"),
        }
    }
}"#),
            Block::Ueberschrift("Der ?-Operator"),
            Block::Text("Oft willst du einen Fehler einfach an den Aufrufer weitergeben. Dafür gibt es `?`: Ist das Ergebnis `Ok`, packt `?` den Wert aus. Ist es `Err`, beendet `?` die Funktion sofort und gibt den Fehler zurück. Das geht nur in Funktionen, die selbst ein `Result` zurückgeben."),
            Block::Ausfuehrbar(r#"use std::num::ParseIntError;

fn doppelt(text: &str) -> Result<i32, ParseIntError> {
    let zahl: i32 = text.trim().parse()?; // bei Fehler: sofort zurück
    Ok(zahl * 2)
}

fn main() {
    println!("{:?}", doppelt("21"));
    println!("{:?}", doppelt("abc"));
}"#),
            Block::Achtung("`unwrap()` und `expect(\"…\")` holen den Wert aus einem `Option` oder `Result` – und lösen bei `None`/`Err` einen **panic** aus. Für schnelle Experimente okay, in echten Programmen behandelst du Fehler besser mit `match`, `if let`, `unwrap_or` oder `?`."),
        ],
        aufgaben: &[
            Aufgabe::Quiz {
                id: "13.1",
                frage: "Was macht der `?`-Operator bei einem `Err`-Wert?",
                code: None,
                optionen: &["Er ignoriert den Fehler", "Er lässt das Programm abstürzen", "Er gibt den Fehler sofort aus der aktuellen Funktion zurück", "Er wandelt den Fehler in 0 um"],
                richtig: 2,
                erklaerung: "`?` ist eine Kurzform für: bei `Ok(v)` weiter mit `v`, bei `Err(e)` sofort `return Err(e)`.",
            },
            Aufgabe::Quiz {
                id: "13.2",
                frage: "Was liefert `\"256\".parse::<u8>()`?",
                code: None,
                optionen: &["`Ok(256)`", "`Ok(0)`", "`Err(...)`, weil 256 nicht in ein u8 passt", "Einen panic"],
                richtig: 2,
                erklaerung: "Ein u8 geht nur bis 255. `parse` meldet das sauber als `Err` – ohne Absturz.",
            },
            Aufgabe::Rechnung {
                id: "13.3",
                frage: "Welchen Wert hat `n`?",
                code: Some("let a: i32 = \"42\".parse().unwrap_or(0);\nlet b: i32 = \"x\".parse().unwrap_or(8);\nlet n = a + b;"),
                antworten: &["50"],
                hinweis: "\"42\" lässt sich parsen, \"x\" nicht – dann greift der Standardwert.",
                erklaerung: "a = 42, b = 8 (weil \"x\" keine Zahl ist), also n = 50.",
            },
            Aufgabe::Code {
                id: "13.4",
                aufgabe: "Vervollständige `summe_text` mit dem `?`-Operator: Beide Texte parsen und die Summe zurückgeben.",
                vorlage: r#"use std::num::ParseIntError;

fn summe_text(a: &str, b: &str) -> Result<i32, ParseIntError> {
    // TODO: a und b mit .parse::<i32>()? umwandeln
    Ok(0)
}

fn main() {
    for (a, b) in [("10", "32"), ("10", "abc")] {
        match summe_text(a, b) {
            Ok(s) => println!("Summe: {s}"),
            Err(e) => println!("Fehler: {e}"),
        }
    }
}"#,
                erwartet: "Summe: 42\nFehler: invalid digit found in string",
                hinweis: "`let x = a.parse::<i32>()?;` und genauso für `b`. Dann `Ok(x + y)`.",
                loesung: r#"use std::num::ParseIntError;

fn summe_text(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x = a.parse::<i32>()?;
    let y = b.parse::<i32>()?;
    Ok(x + y)
}

fn main() {
    for (a, b) in [("10", "32"), ("10", "abc")] {
        match summe_text(a, b) {
            Ok(s) => println!("Summe: {s}"),
            Err(e) => println!("Fehler: {e}"),
        }
    }
}"#,
            },
        ],
    },
    Lektion {
        titel: "Traits & Generics",
        kurz: "Gemeinsames Verhalten und Code für viele Typen.",
        kapitel: "Fortgeschritten",
        bloecke: &[
            Block::Text("Ein **Trait** beschreibt, was ein Typ *kann* – ähnlich wie ein Interface in anderen Sprachen. Verschiedene Typen können denselben Trait auf ihre eigene Weise implementieren."),
            Block::Visual(Visual::Formen),
            Block::Ausfuehrbar(r#"trait Tier {
    fn name(&self) -> String;
    fn laut(&self) -> String;

    // Standard-Implementierung, die alle Tiere bekommen
    fn vorstellen(&self) -> String {
        format!("{} macht {}", self.name(), self.laut())
    }
}

struct Hund;
struct Katze { name: String }

impl Tier for Hund {
    fn name(&self) -> String { "Der Hund".to_string() }
    fn laut(&self) -> String { "Wuff!".to_string() }
}

impl Tier for Katze {
    fn name(&self) -> String { self.name.clone() }
    fn laut(&self) -> String { "Miau!".to_string() }
}

fn main() {
    let tiere: Vec<Box<dyn Tier>> = vec![
        Box::new(Hund),
        Box::new(Katze { name: "Minka".to_string() }),
    ];
    for t in &tiere {
        println!("{}", t.vorstellen());
    }
}"#),
            Block::Ueberschrift("Generics"),
            Block::Text("Mit **Generics** schreibst du Code, der für viele Typen funktioniert. `T` ist ein Platzhalter. Mit einer **Trait-Schranke** wie `T: PartialOrd` sagst du: „`T` darf jeder Typ sein, den man vergleichen kann.“ Rust erzeugt beim Kompilieren für jeden benutzten Typ eine eigene, schnelle Version."),
            Block::Ausfuehrbar(r#"fn groesser<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

fn main() {
    println!("{}", groesser(3, 7));
    println!("{}", groesser(2.5, 1.5));
    println!("{}", groesser("Apfel", "Birne"));
}"#),
            Block::Tipp("Viele Traits kannst du automatisch ableiten: `#[derive(Debug, Clone, PartialEq)]`. Wichtige Standard-Traits sind `Display` (für `{}`), `Debug` (für `{:?}`), `Clone`, `PartialEq` (für `==`) und `PartialOrd` (für `<`)."),
        ],
        aufgaben: &[
            Aufgabe::Quiz {
                id: "14.1",
                frage: "Was ist ein Trait?",
                code: None,
                optionen: &["Eine spezielle Art von Variable", "Eine Beschreibung von Verhalten (Methoden), das Typen implementieren können", "Ein Fehlertyp", "Ein anderer Name für struct"],
                richtig: 1,
                erklaerung: "Traits definieren gemeinsames Verhalten. Typen implementieren es mit `impl Trait for Typ`.",
            },
            Aufgabe::Rechnung {
                id: "14.2",
                frage: "Ein `Kreis { r: 2.0 }` berechnet `flaeche()` als π × r². Was kommt heraus? (Auf 2 Nachkommastellen gerundet – nutze die Grafik oben!)",
                code: None,
                antworten: &["12.57", "12,57"],
                hinweis: "π ≈ 3.14159. Rechne 3.14159 × 2 × 2.",
                erklaerung: "π × 4 ≈ 12.566… gerundet 12.57.",
            },
            Aufgabe::Code {
                id: "14.3",
                aufgabe: "Implementiere den Trait `Form` für `Quadrat`. Erwartete Ausgabe: `Fläche: 25`",
                vorlage: r#"trait Form {
    fn flaeche(&self) -> f64;
}

struct Quadrat {
    seite: f64,
}

// TODO: impl Form for Quadrat { ... }

fn main() {
    let q = Quadrat { seite: 5.0 };
    println!("Fläche: {}", q.flaeche());
}"#,
                erwartet: "Fläche: 25",
                hinweis: "`impl Form for Quadrat { fn flaeche(&self) -> f64 { self.seite * self.seite } }`",
                loesung: r#"trait Form {
    fn flaeche(&self) -> f64;
}

struct Quadrat {
    seite: f64,
}

impl Form for Quadrat {
    fn flaeche(&self) -> f64 {
        self.seite * self.seite
    }
}

fn main() {
    let q = Quadrat { seite: 5.0 };
    println!("Fläche: {}", q.flaeche());
}"#,
            },
            Aufgabe::Code {
                id: "14.4",
                aufgabe: "Schreibe die generische Funktion `groesstes`, die das größte Element einer Liste findet. Sie soll für ganze Zahlen und Kommazahlen funktionieren.",
                vorlage: r#"fn groesstes<T: PartialOrd + Copy>(liste: &[T]) -> T {
    let mut max = liste[0];
    // TODO: alle Elemente durchgehen und max aktualisieren
    max
}

fn main() {
    println!("Größte Zahl: {}", groesstes(&[3, 42, 7, 19]));
    println!("Größte Kommazahl: {}", groesstes(&[1.5, 0.2, 3.5, 2.0]));
}"#,
                erwartet: "Größte Zahl: 42\nGrößte Kommazahl: 3.5",
                hinweis: "`for &x in liste { if x > max { max = x; } }` – das `&x` packt die Referenz direkt aus.",
                loesung: r#"fn groesstes<T: PartialOrd + Copy>(liste: &[T]) -> T {
    let mut max = liste[0];
    for &x in liste {
        if x > max {
            max = x;
        }
    }
    max
}

fn main() {
    println!("Größte Zahl: {}", groesstes(&[3, 42, 7, 19]));
    println!("Größte Kommazahl: {}", groesstes(&[1.5, 0.2, 3.5, 2.0]));
}"#,
            },
        ],
    },
    Lektion {
        titel: "Closures & Iteratoren",
        kurz: "Kleine Funktionen und elegante Datenverarbeitung.",
        kapitel: "Fortgeschritten",
        bloecke: &[
            Block::Text("Eine **Closure** ist eine kleine, namenlose Funktion, die du in einer Variable speichern oder weitergeben kannst. Sie wird mit senkrechten Strichen geschrieben: `|x| x + 1`. Closures können Variablen aus ihrer Umgebung benutzen („einfangen“)."),
            Block::Ausfuehrbar(r#"fn main() {
    let plus_eins = |x: i32| x + 1;
    println!("{}", plus_eins(5));

    let faktor = 3;
    let mal_faktor = |x| x * faktor;   // fängt `faktor` ein
    println!("{}", mal_faktor(4));

    let mut zaehler = 0;
    let mut zaehlen = || zaehler += 1; // verändert die Umgebung
    zaehlen();
    zaehlen();
    println!("Gezählt: {zaehler}");
}"#),
            Block::Ueberschrift("Iteratoren"),
            Block::Text("Ein **Iterator** liefert Werte nacheinander. Mit Methoden wie `map` (umwandeln), `filter` (aussortieren) und `sum`, `count` oder `collect` (einsammeln) baust du Verarbeitungsketten. Bau dir selbst eine:"),
            Block::Visual(Visual::Iterator),
            Block::Ausfuehrbar(r#"fn main() {
    let namen = vec!["ada", "linus", "grace", "ferris"];

    let gross: Vec<String> = namen
        .iter()
        .filter(|n| n.len() > 3)
        .map(|n| n.to_uppercase())
        .collect();
    println!("{gross:?}");

    for (i, n) in namen.iter().enumerate() {
        println!("{}. {n}", i + 1);
    }

    let gesamt: usize = namen.iter().map(|n| n.len()).sum();
    println!("Buchstaben insgesamt: {gesamt}");
}"#),
            Block::Tipp("Iteratoren sind in Rust genauso schnell wie handgeschriebene Schleifen – der Compiler optimiert sie vollständig. Man spricht von **Zero-Cost Abstractions**."),
        ],
        aufgaben: &[
            Aufgabe::Rechnung {
                id: "15.1",
                frage: "Was ergibt dieser Ausdruck?",
                code: Some("(1..=5).map(|x| x * x).sum::<i32>()"),
                antworten: &["55"],
                hinweis: "Quadriere 1 bis 5 und addiere: 1 + 4 + 9 + …",
                erklaerung: "1 + 4 + 9 + 16 + 25 = 55.",
            },
            Aufgabe::Rechnung {
                id: "15.2",
                frage: "Was ergibt dieser Ausdruck?",
                code: Some("(1..=20).filter(|x| x % 3 == 0).count()"),
                antworten: &["6"],
                hinweis: "Wie viele Vielfache von 3 gibt es zwischen 1 und 20?",
                erklaerung: "3, 6, 9, 12, 15, 18 → 6 Stück. Oder: 20 / 3 = 6 (Ganzzahl-Division).",
            },
            Aufgabe::Rechnung {
                id: "15.3",
                frage: "Was gibt das Programm aus?",
                code: Some("let faktor = 3;\nlet mal = |x: i32| x * faktor;\nprintln!(\"{}\", mal(mal(2)));"),
                antworten: &["18"],
                hinweis: "Von innen nach außen: `mal(2)` zuerst.",
                erklaerung: "mal(2) = 6, mal(6) = 18.",
            },
            Aufgabe::Code {
                id: "15.4",
                aufgabe: "Berechne mit einer Iterator-Kette die Summe der **Quadrate aller geraden Zahlen** von 1 bis 10. Erwartete Ausgabe: `Ergebnis: 220`",
                vorlage: r#"fn main() {
    let ergebnis: i32 = (1..=10)
        // TODO: filter und map
        .sum();
    println!("Ergebnis: {ergebnis}");
}"#,
                erwartet: "Ergebnis: 220",
                hinweis: "`.filter(|x| x % 2 == 0).map(|x| x * x)` vor `.sum()`",
                loesung: r#"fn main() {
    let ergebnis: i32 = (1..=10)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .sum();
    println!("Ergebnis: {ergebnis}");
}"#,
            },
            Aufgabe::Code {
                id: "15.5",
                aufgabe: "Nimm alle Wörter mit **mehr als 3 Buchstaben**, wandle sie in Großbuchstaben um und verbinde sie mit `, `.",
                vorlage: r#"fn main() {
    let woerter = ["rust", "ist", "super", "schnell", "und", "sicher"];
    let ergebnis: Vec<String> = woerter
        .iter()
        // TODO
        .collect();
    println!("{}", ergebnis.join(", "));
}"#,
                erwartet: "RUST, SUPER, SCHNELL, SICHER",
                hinweis: "`.filter(|w| w.len() > 3).map(|w| w.to_uppercase())`",
                loesung: r#"fn main() {
    let woerter = ["rust", "ist", "super", "schnell", "und", "sicher"];
    let ergebnis: Vec<String> = woerter
        .iter()
        .filter(|w| w.len() > 3)
        .map(|w| w.to_uppercase())
        .collect();
    println!("{}", ergebnis.join(", "));
}"#,
            },
        ],
    },
    Lektion {
        titel: "Abschlussprojekt: Notenrechner",
        kurz: "Alles zusammen: Structs, Vec, Iteratoren, Option.",
        kapitel: "Fortgeschritten",
        bloecke: &[
            Block::Text("Zeit, alles zu verbinden! Du baust einen kleinen **Notenrechner** für eine Schulklasse (Schweizer Noten: 6 ist die beste, 4 genügend). Dabei benutzt du Structs, Methoden, Vektoren, Iteratoren und `Option`."),
            Block::Ausfuehrbar(r#"struct Schueler {
    name: String,
    noten: Vec<f64>,
}

impl Schueler {
    fn neu(name: &str, noten: &[f64]) -> Self {
        Self { name: name.to_string(), noten: noten.to_vec() }
    }

    fn schnitt(&self) -> Option<f64> {
        if self.noten.is_empty() {
            return None;
        }
        let summe: f64 = self.noten.iter().sum();
        Some(summe / self.noten.len() as f64)
    }
}

fn main() {
    let klasse = vec![
        Schueler::neu("Ada", &[5.5, 6.0, 5.0]),
        Schueler::neu("Linus", &[4.0, 4.5, 3.5]),
        Schueler::neu("Neu", &[]),
    ];

    for s in &klasse {
        match s.schnitt() {
            Some(n) => println!("{:<6} {:.2} {}", s.name, n, if n >= 4.0 { "✅" } else { "❌" }),
            None => println!("{:<6} noch keine Noten", s.name),
        }
    }
}"#),
            Block::Text("Leg das Projekt im Terminal an. Auf der **Terminal-Seite** (oben im Menü) kannst du die Datei sogar im Editor bearbeiten (`code src/main.rs`) und mit `cargo run` ausführen."),
            Block::Terminal,
            Block::Ueberschrift("Wie geht es weiter?"),
            Block::Text("Glückwunsch – du kennst jetzt die wichtigsten Grundlagen von Rust! Als Nächstes lohnen sich: **Lifetimes** (`'a`), **Module** und Sichtbarkeit (`mod`, `pub`), **Tests** mit `#[test]`, **Smart Pointer** (`Box`, `Rc`, `RefCell`) und **Nebenläufigkeit** mit Threads. Das offizielle Buch gibt es auch auf Deutsch: „Die Programmiersprache Rust“ (rust-lang-de.github.io/rustbook-de). Zum Üben sind die Rustlings-Aufgaben großartig."),
        ],
        aufgaben: &[
            Aufgabe::Terminal {
                id: "16.1",
                aufgabe: "Lege das Projekt `notenrechner` an, wechsle hinein, sieh dir die Dateien mit `tree` an und führe es aus.",
                ziele: &["cargo new notenrechner", "cd notenrechner", "tree", "cargo run"],
                hinweis: "Vier Befehle nacheinander: `cargo new notenrechner`, `cd notenrechner`, `tree`, `cargo run`.",
            },
            Aufgabe::Quiz {
                id: "16.2",
                frage: "Warum gibt `schnitt()` ein `Option<f64>` zurück statt einfach `f64`?",
                code: None,
                optionen: &["Weil f64 zu ungenau ist", "Weil es ohne Noten keinen Durchschnitt gibt (Division durch 0)", "Weil Option schneller ist", "Weil Methoden immer Option zurückgeben müssen"],
                richtig: 1,
                erklaerung: "Bei einer leeren Liste wäre es 0 / 0. Mit `None` sagt die Funktion ehrlich: Hier gibt es kein Ergebnis.",
            },
            Aufgabe::Rechnung {
                id: "16.3",
                frage: "Ada hat die Noten 5.5, 6.0 und 5.0. Wie lautet ihr Durchschnitt? (2 Nachkommastellen)",
                code: None,
                antworten: &["5.50", "5.5", "5,50", "5,5"],
                hinweis: "Summe durch Anzahl: (5.5 + 6.0 + 5.0) / 3",
                erklaerung: "16.5 / 3 = 5.50 ✅",
            },
            Aufgabe::Code {
                id: "16.4",
                aufgabe: "Erweitere den Notenrechner: Schreibe die Methode `beste_note`, die die höchste Note als `Option<f64>` zurückgibt. Gib dann für jeden Schüler die beste Note aus.",
                vorlage: r#"struct Schueler {
    name: String,
    noten: Vec<f64>,
}

impl Schueler {
    fn beste_note(&self) -> Option<f64> {
        // TODO: Tipp: Schleife mit einer Option-Variablen,
        // oder: self.noten.iter().copied().reduce(f64::max)
        None
    }
}

fn main() {
    let klasse = vec![
        Schueler { name: "Ada".to_string(), noten: vec![5.5, 6.0, 5.0] },
        Schueler { name: "Linus".to_string(), noten: vec![4.0, 4.5, 3.5] },
        Schueler { name: "Neu".to_string(), noten: vec![] },
    ];
    for s in &klasse {
        match s.beste_note() {
            Some(n) => println!("{}: beste Note {n}", s.name),
            None => println!("{}: keine Noten", s.name),
        }
    }
}"#,
                erwartet: "Ada: beste Note 6\nLinus: beste Note 4.5\nNeu: keine Noten",
                hinweis: "Einfachste Lösung: `self.noten.iter().copied().reduce(f64::max)`. `reduce` gibt bei leerer Liste `None` zurück.",
                loesung: r#"struct Schueler {
    name: String,
    noten: Vec<f64>,
}

impl Schueler {
    fn beste_note(&self) -> Option<f64> {
        let mut beste: Option<f64> = None;
        for &n in &self.noten {
            beste = match beste {
                Some(b) if b >= n => Some(b),
                _ => Some(n),
            };
        }
        beste
    }
}

fn main() {
    let klasse = vec![
        Schueler { name: "Ada".to_string(), noten: vec![5.5, 6.0, 5.0] },
        Schueler { name: "Linus".to_string(), noten: vec![4.0, 4.5, 3.5] },
        Schueler { name: "Neu".to_string(), noten: vec![] },
    ];
    for s in &klasse {
        match s.beste_note() {
            Some(n) => println!("{}: beste Note {n}", s.name),
            None => println!("{}: keine Noten", s.name),
        }
    }
}"#,
            },
        ],
    },
];
