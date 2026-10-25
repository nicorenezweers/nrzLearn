//! Cheat sheet cards. Every world unlocks its card through its "Learn" level.
//! `titel` and `code` hold English, German and Spanish.

use crate::i18n::Sprache;

pub struct Karte {
    pub welt: usize,
    titel: [&'static str; 3],
    code: [&'static str; 3],
}

impl Karte {
    pub const fn titel(&self, l: Sprache) -> &'static str {
        self.titel[l as usize]
    }
    pub const fn code(&self, l: Sprache) -> &'static str {
        self.code[l as usize]
    }
}

pub static KARTEN: &[Karte] = &[
    Karte {
        welt: 0,
        titel: ["Hello & Cargo", "Hallo & Cargo", "Hola y Cargo"],
        code: [
            "fn main() {\n    println!(\"Hello!\");  // macro with !\n}\n\ncargo new project   # create\ncargo run           # build + run\ncargo build / check # build / check\ncargo test / add    # tests / crate",
            "fn main() {\n    println!(\"Hallo!\");  // Makro mit !\n}\n\ncargo new projekt   # anlegen\ncargo run           # bauen + starten\ncargo build / check # bauen / prüfen\ncargo test / add    # Tests / Crate",
            "fn main() {\n    println!(\"¡Hola!\");  // macro con !\n}\n\ncargo new proyecto  # crear\ncargo run           # compilar + ejecutar\ncargo build / check # compilar / comprobar\ncargo test / add    # tests / crate",
        ],
    },
    Karte {
        welt: 1,
        titel: ["Variables", "Variablen", "Variables"],
        code: [
            "let x = 5;          // immutable\nlet mut y = 10;     // mutable\ny += 1;\nlet x = x * 2;      // shadowing (new variable)\nconst MAX: u32 = 100;\n{ let z = 1; }      // z only lives in the block",
            "let x = 5;          // unveränderlich\nlet mut y = 10;     // veränderlich\ny += 1;\nlet x = x * 2;      // Shadowing (neue Variable)\nconst MAX: u32 = 100;\n{ let z = 1; }      // z gilt nur im Block",
            "let x = 5;          // inmutable\nlet mut y = 10;     // mutable\ny += 1;\nlet x = x * 2;      // shadowing (variable nueva)\nconst MAX: u32 = 100;\n{ let z = 1; }      // z solo vive en el bloque",
        ],
    },
    Karte {
        welt: 2,
        titel: ["Numbers & types", "Zahlen & Typen", "Números y tipos"],
        code: [
            "u8: 0..=255      i8: -128..=127\nu16: 0..=65535   i32: default\nf64: decimal     bool, char ('ß')\n7 / 2 == 3       7 % 2 == 1\n7.0 / 2.0 == 3.5\nx as f64         // convert\n250u8.wrapping_add(10)   == 4\n250u8.checked_add(10)    == None\n250u8.saturating_add(10) == 255\n0b1010 == 10     0xFF == 255",
            "u8: 0..=255      i8: -128..=127\nu16: 0..=65535   i32: Standard\nf64: Kommazahl   bool, char ('ß')\n7 / 2 == 3       7 % 2 == 1\n7.0 / 2.0 == 3.5\nx as f64         // umwandeln\n250u8.wrapping_add(10)   == 4\n250u8.checked_add(10)    == None\n250u8.saturating_add(10) == 255\n0b1010 == 10     0xFF == 255",
            "u8: 0..=255      i8: -128..=127\nu16: 0..=65535   i32: por defecto\nf64: decimal     bool, char ('ñ')\n7 / 2 == 3       7 % 2 == 1\n7.0 / 2.0 == 3.5\nx as f64         // convertir\n250u8.wrapping_add(10)   == 4\n250u8.checked_add(10)    == None\n250u8.saturating_add(10) == 255\n0b1010 == 10     0xFF == 255",
        ],
    },
    Karte {
        welt: 3,
        titel: ["Functions", "Funktionen", "Funciones"],
        code: [
            "fn add(a: i32, b: i32) -> i32 {\n    a + b      // no ; → return value\n}\n\nlet y = { let x = 3; x + 1 };  // block = expression\nfn nothing() {}  // returns ()",
            "fn addiere(a: i32, b: i32) -> i32 {\n    a + b      // kein ; → Rückgabewert\n}\n\nlet y = { let x = 3; x + 1 };  // Block = Ausdruck\nfn nichts() {}  // gibt () zurück",
            "fn sumar(a: i32, b: i32) -> i32 {\n    a + b      // sin ; → valor de retorno\n}\n\nlet y = { let x = 3; x + 1 };  // bloque = expresión\nfn nada() {}  // devuelve ()",
        ],
    },
    Karte {
        welt: 4,
        titel: ["if & logic", "if & Logik", "if y lógica"],
        code: [
            "if x > 5 { … } else if x == 0 { … } else { … }\nlet m = if a > b { a } else { b };\n\n==  !=  <  >  <=  >=\n&& (and)  || (or)  ! (not)\n&& binds tighter than ||",
            "if x > 5 { … } else if x == 0 { … } else { … }\nlet m = if a > b { a } else { b };\n\n==  !=  <  >  <=  >=\n&& (und)  || (oder)  ! (nicht)\n&& bindet stärker als ||",
            "if x > 5 { … } else if x == 0 { … } else { … }\nlet m = if a > b { a } else { b };\n\n==  !=  <  >  <=  >=\n&& (y)  || (o)  ! (no)\n&& tiene prioridad sobre ||",
        ],
    },
    Karte {
        welt: 5,
        titel: ["Loops", "Schleifen", "Bucles"],
        code: [
            "for i in 0..5 { }     // 0,1,2,3,4\nfor i in 1..=5 { }    // 1..5 inclusive\nfor i in (0..5).rev() { }\nfor i in (0..10).step_by(2) { }\nwhile n > 0 { n -= 1; }\nlet r = loop { break 42; };\ncontinue;  // next iteration\n1+2+…+n = n*(n+1)/2",
            "for i in 0..5 { }     // 0,1,2,3,4\nfor i in 1..=5 { }    // 1..5 inklusive\nfor i in (0..5).rev() { }\nfor i in (0..10).step_by(2) { }\nwhile n > 0 { n -= 1; }\nlet r = loop { break 42; };\ncontinue;  // nächster Durchlauf\n1+2+…+n = n*(n+1)/2",
            "for i in 0..5 { }     // 0,1,2,3,4\nfor i in 1..=5 { }    // 1..5 incluido\nfor i in (0..5).rev() { }\nfor i in (0..10).step_by(2) { }\nwhile n > 0 { n -= 1; }\nlet r = loop { break 42; };\ncontinue;  // siguiente vuelta\n1+2+…+n = n*(n+1)/2",
        ],
    },
    Karte {
        welt: 6,
        titel: ["Ownership", "Ownership", "Ownership"],
        code: [
            "let s1 = String::from(\"hi\");\nlet s2 = s1;          // move: s1 invalid\nlet s3 = s2.clone();  // deep copy\nlet a = 5; let b = a; // Copy: both valid\n\nCopy: i32, f64, bool, char, (i32, i32)\nnot Copy: String, Vec, Box\nDrop: reverse order",
            "let s1 = String::from(\"hi\");\nlet s2 = s1;          // move: s1 ungültig\nlet s3 = s2.clone();  // tiefe Kopie\nlet a = 5; let b = a; // Copy: beide gültig\n\nCopy: i32, f64, bool, char, (i32, i32)\nkein Copy: String, Vec, Box\nDrop: umgekehrte Reihenfolge",
            "let s1 = String::from(\"hi\");\nlet s2 = s1;          // move: s1 inválida\nlet s3 = s2.clone();  // copia profunda\nlet a = 5; let b = a; // Copy: ambas válidas\n\nCopy: i32, f64, bool, char, (i32, i32)\nsin Copy: String, Vec, Box\nDrop: orden inverso",
        ],
    },
    Karte {
        welt: 7,
        titel: ["Borrowing", "Borrowing", "Préstamos"],
        code: [
            "fn length(s: &String) -> usize { s.len() }\nfn append(s: &mut String) { s.push('!'); }\n\nlength(&text);\nappend(&mut text);\n*number += 1;   // dereference\n\nMany &  OR  exactly one &mut",
            "fn laenge(s: &String) -> usize { s.len() }\nfn anhaengen(s: &mut String) { s.push('!'); }\n\nlaenge(&text);\nanhaengen(&mut text);\n*zahl += 1;   // dereferenzieren\n\nViele &  ODER  genau ein &mut",
            "fn longitud(s: &String) -> usize { s.len() }\nfn anadir(s: &mut String) { s.push('!'); }\n\nlongitud(&texto);\nanadir(&mut texto);\n*numero += 1;   // desreferenciar\n\nMuchas &  O  exactamente una &mut",
        ],
    },
    Karte {
        welt: 8,
        titel: ["Strings", "Strings", "Strings"],
        code: [
            "let s: &str = \"Text\";        // slice\nlet mut t = String::from(\"Hello\");\nt.push_str(\", world\"); t.push('!');\n&t[0..5]                      // \"Hello\"\nt.len()            // bytes! (ü = 2)\nt.chars().count()  // characters\nformat!(\"{t} {n}\")\ns.split_whitespace()",
            "let s: &str = \"Text\";        // Slice\nlet mut t = String::from(\"Hallo\");\nt.push_str(\", Welt\"); t.push('!');\n&t[0..5]                      // \"Hallo\"\nt.len()            // Bytes! (ü = 2)\nt.chars().count()  // Zeichen\nformat!(\"{t} {n}\")\ns.split_whitespace()",
            "let s: &str = \"Texto\";       // slice\nlet mut t = String::from(\"Hola\");\nt.push_str(\", mundo\"); t.push('!');\n&t[0..4]                      // \"Hola\"\nt.len()            // ¡bytes! (ñ = 2)\nt.chars().count()  // caracteres\nformat!(\"{t} {n}\")\ns.split_whitespace()",
        ],
    },
    Karte {
        welt: 9,
        titel: ["Structs", "Structs", "Structs"],
        code: [
            "struct Point { x: f64, y: f64 }\n\nimpl Point {\n    fn new(x: f64, y: f64) -> Self { Self { x, y } }\n    fn length(&self) -> f64 { … }   // reads\n    fn shift(&mut self) { … }       // changes\n}\nPoint::new(1.0, 2.0)   p.length()",
            "struct Punkt { x: f64, y: f64 }\n\nimpl Punkt {\n    fn neu(x: f64, y: f64) -> Self { Self { x, y } }\n    fn betrag(&self) -> f64 { … }   // liest\n    fn schieben(&mut self) { … }    // ändert\n}\nPunkt::neu(1.0, 2.0)   p.betrag()",
            "struct Punto { x: f64, y: f64 }\n\nimpl Punto {\n    fn nuevo(x: f64, y: f64) -> Self { Self { x, y } }\n    fn longitud(&self) -> f64 { … }  // lee\n    fn mover(&mut self) { … }        // cambia\n}\nPunto::nuevo(1.0, 2.0)   p.longitud()",
        ],
    },
    Karte {
        welt: 10,
        titel: ["Enums & match", "Enums & match", "Enums y match"],
        code: [
            "enum Shape { Circle(f64), Square(f64) }\n\nmatch number {\n    0 => …,\n    1 | 2 => …,          // or\n    3..=9 => …,          // range\n    n if n < 0 => …,     // guard\n    _ => …,              // rest\n}\nif let Some(x) = opt { … }\nopt.unwrap_or(0)",
            "enum Form { Kreis(f64), Quadrat(f64) }\n\nmatch zahl {\n    0 => …,\n    1 | 2 => …,          // oder\n    3..=9 => …,          // Bereich\n    n if n < 0 => …,     // Guard\n    _ => …,              // Rest\n}\nif let Some(x) = opt { … }\nopt.unwrap_or(0)",
            "enum Forma { Circulo(f64), Cuadrado(f64) }\n\nmatch numero {\n    0 => …,\n    1 | 2 => …,          // o\n    3..=9 => …,          // rango\n    n if n < 0 => …,     // guarda\n    _ => …,              // resto\n}\nif let Some(x) = opt { … }\nopt.unwrap_or(0)",
        ],
    },
    Karte {
        welt: 11,
        titel: ["Vec & HashMap", "Vec & HashMap", "Vec y HashMap"],
        code: [
            "let mut v = vec![1, 2, 3];\nv.push(4); v.pop(); v.len();\nv[0]        // panics if invalid\nv.get(10)   // None\nv.sort();  v.contains(&3)\nCapacity: 4 → 8 → 16 → 32\n\nlet mut m = HashMap::new();\nm.insert(\"a\", 1);\n*m.entry(\"a\").or_insert(0) += 1;",
            "let mut v = vec![1, 2, 3];\nv.push(4); v.pop(); v.len();\nv[0]        // panic, wenn ungültig\nv.get(10)   // None\nv.sort();  v.contains(&3)\nKapazität: 4 → 8 → 16 → 32\n\nlet mut m = HashMap::new();\nm.insert(\"a\", 1);\n*m.entry(\"a\").or_insert(0) += 1;",
            "let mut v = vec![1, 2, 3];\nv.push(4); v.pop(); v.len();\nv[0]        // panic si no es válido\nv.get(10)   // None\nv.sort();  v.contains(&3)\nCapacidad: 4 → 8 → 16 → 32\n\nlet mut m = HashMap::new();\nm.insert(\"a\", 1);\n*m.entry(\"a\").or_insert(0) += 1;",
        ],
    },
    Karte {
        welt: 12,
        titel: ["Option & Result", "Option & Result", "Option y Result"],
        code: [
            "Option<T>: Some(x) | None\nResult<T, E>: Ok(x) | Err(e)\n\n\"42\".parse::<i32>()   // Ok(42)\n\"x\".parse::<i32>()    // Err(…)\nlet n = s.parse()?;    // Err → return early\n.unwrap()     // panics on Err/None\n.unwrap_or(0) // default value",
            "Option<T>: Some(x) | None\nResult<T, E>: Ok(x) | Err(e)\n\n\"42\".parse::<i32>()   // Ok(42)\n\"x\".parse::<i32>()    // Err(…)\nlet n = s.parse()?;    // Err → sofort zurück\n.unwrap()     // panic bei Err/None\n.unwrap_or(0) // Standardwert",
            "Option<T>: Some(x) | None\nResult<T, E>: Ok(x) | Err(e)\n\n\"42\".parse::<i32>()   // Ok(42)\n\"x\".parse::<i32>()    // Err(…)\nlet n = s.parse()?;    // Err → devuelve ya\n.unwrap()     // panic con Err/None\n.unwrap_or(0) // valor por defecto",
        ],
    },
    Karte {
        welt: 13,
        titel: ["Traits & generics", "Traits & Generics", "Traits y genéricos"],
        code: [
            "trait Shape {\n    fn area(&self) -> f64;\n    fn info(&self) -> String { … } // default\n}\nimpl Shape for Circle { … }\n\nfn larger<T: PartialOrd>(a: T, b: T) -> T\n#[derive(Debug, Clone, PartialEq)]\nCircle: π·r²   Rectangle: a·b",
            "trait Form {\n    fn flaeche(&self) -> f64;\n    fn info(&self) -> String { … } // Standard\n}\nimpl Form for Kreis { … }\n\nfn groesser<T: PartialOrd>(a: T, b: T) -> T\n#[derive(Debug, Clone, PartialEq)]\nKreis: π·r²   Rechteck: a·b",
            "trait Forma {\n    fn area(&self) -> f64;\n    fn info(&self) -> String { … } // por defecto\n}\nimpl Forma for Circulo { … }\n\nfn mayor<T: PartialOrd>(a: T, b: T) -> T\n#[derive(Debug, Clone, PartialEq)]\nCírculo: π·r²   Rectángulo: a·b",
        ],
    },
    Karte {
        welt: 14,
        titel: ["Closures & iterators", "Closures & Iteratoren", "Closures e iteradores"],
        code: [
            "let double = |x: i32| x * 2;\n\n(1..=5).map(|x| x * x).sum::<i32>()   // 55\n.filter(|x| x % 2 == 0)\n.count()  .collect::<Vec<_>>()\n.enumerate()  .rev()  .any(…)  .all(…)\n(1..=n).product()  .fold(0, |a, x| a + x)",
            "let doppelt = |x: i32| x * 2;\n\n(1..=5).map(|x| x * x).sum::<i32>()   // 55\n.filter(|x| x % 2 == 0)\n.count()  .collect::<Vec<_>>()\n.enumerate()  .rev()  .any(…)  .all(…)\n(1..=n).product()  .fold(0, |a, x| a + x)",
            "let doble = |x: i32| x * 2;\n\n(1..=5).map(|x| x * x).sum::<i32>()   // 55\n.filter(|x| x % 2 == 0)\n.count()  .collect::<Vec<_>>()\n.enumerate()  .rev()  .any(…)  .all(…)\n(1..=n).product()  .fold(0, |a, x| a + x)",
        ],
    },
    Karte {
        welt: 15,
        titel: ["Formatting output", "Ausgabe formatieren", "Formatear la salida"],
        code: [
            "println!(\"{}\", x);      // Display\nprintln!(\"{x}\");        // inline\nprintln!(\"{:?}\", v);    // Debug\nprintln!(\"{:.2}\", 3.14159);  // 3.14\nprintln!(\"{:>5}|\", 42); // right-aligned\nprintln!(\"{0} {1} {0}\", a, b);\neprintln!(\"Error!\");    // stderr",
            "println!(\"{}\", x);      // Display\nprintln!(\"{x}\");        // direkt\nprintln!(\"{:?}\", v);    // Debug\nprintln!(\"{:.2}\", 3.14159);  // 3.14\nprintln!(\"{:>5}|\", 42); // rechtsbündig\nprintln!(\"{0} {1} {0}\", a, b);\neprintln!(\"Fehler!\");   // stderr",
            "println!(\"{}\", x);      // Display\nprintln!(\"{x}\");        // directo\nprintln!(\"{:?}\", v);    // Debug\nprintln!(\"{:.2}\", 3.14159);  // 3.14\nprintln!(\"{:>5}|\", 42); // alineado a la derecha\nprintln!(\"{0} {1} {0}\", a, b);\neprintln!(\"¡Error!\");   // stderr",
        ],
    },
];
