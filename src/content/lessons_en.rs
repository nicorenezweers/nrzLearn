//! All lessons in English. Every lesson builds on the previous one.

use super::Status::{Freigegeben as F, Geliehen as G, Gueltig as OK, Neu as N, Verschoben as V};
use super::{Aufgabe, Block, Lektion, SpeicherSchritt, Visual, hb, sp, sv, ts};

pub static LEKTIONEN_EN: &[Lektion] = &[
    // ─────────────────────────────── Chapter 1 ───────────────────────────────
    Lektion {
        titel: "Hello, Rust!",
        kurz: "Your first program, the compiler and Cargo.",
        kapitel: "First steps",
        bloecke: &[
            Block::Text("Welcome! **Rust** is a modern programming language that is as fast as C or C++, but protects you from typical mistakes: crashes caused by invalid memory, forgotten error cases or simultaneous access. The trick: the **compiler** checks your program very carefully before it even runs."),
            Block::Text("The mascot of the Rust community is **Ferris**, a little crab 🦀. Rust programmers like to call themselves “Rustaceans”."),
            Block::Ueberschrift("Your first program"),
            Block::Text("Every Rust program starts in the function `main`. Press **Run** – the code is really compiled and executed. You can change it too!"),
            Block::Ausfuehrbar(r#"fn main() {
    // This is a comment – the compiler ignores it.
    println!("Hello, world!");
    println!("I'm learning Rust. 🦀");
}"#),
            Block::Text("What happens here? `fn` starts a function, `main` is its name, and the curly braces `{ }` enclose its body. `println!` prints a line of text. Every statement ends with a semicolon `;`."),
            Block::Tipp("`println!` isn't a normal function but a **macro** – you can tell by the exclamation mark `!`. Macros are small code generators. For now, it's enough to know: call them with `!`."),
            Block::Ueberschrift("From code to program"),
            Block::Text("Rust is a **compiled** language. Your text is translated into machine code that the processor understands directly. Click through the steps:"),
            Block::Visual(Visual::Pipeline),
            Block::Ueberschrift("Cargo – your toolbox"),
            Block::Text("In practice you rarely call the compiler `rustc` directly. Instead you use **Cargo**: it creates projects, manages libraries (called **crates**) and builds your program. A new project looks like this:"),
            Block::Code(r#"$ cargo new hello
$ tree hello
hello
├── Cargo.toml      // project info & dependencies
└── src
    └── main.rs     // your code"#),
            Block::Text("`cargo run` compiles and starts the project. Try it in the terminal! Type `cargo new hello`, `cd hello`, `cat src/main.rs` and `cargo run` one after another."),
            Block::Terminal,
        ],
        aufgaben: &[
            Aufgabe::Quiz {
                id: "1.1",
                frage: "Which function runs automatically when a Rust program starts?",
                code: None,
                optionen: &["`start()`", "`main()`", "`run()`", "`init()`"],
                richtig: 1,
                erklaerung: "Exactly! `fn main()` is the entry point of every executable Rust program.",
            },
            Aufgabe::Quiz {
                id: "1.2",
                frage: "How can you tell that `println!` is a macro and not a function?",
                code: None,
                optionen: &["By the exclamation mark `!`", "By the parentheses", "By the semicolon at the end", "You can't tell"],
                richtig: 0,
                erklaerung: "Macros in Rust are always called with `!`, e.g. `println!`, `vec!` or `format!`.",
            },
            Aufgabe::Terminal {
                id: "1.3",
                aufgabe: "Use Cargo to create a project called `hello`, change into its folder and run the program.",
                ziele: &["cargo new hello", "cd hello", "cargo run"],
                hinweis: "Type the three commands exactly like this and press Enter after each one.",
            },
            Aufgabe::Code {
                id: "1.4",
                aufgabe: "Change the program so that it prints exactly `Hello, Rust!`.",
                vorlage: r#"fn main() {
    // Write your code here:

}"#,
                erwartet: "Hello, Rust!",
                hinweis: "Use `println!(\"...\");` – don't forget the quotes and the semicolon.",
                loesung: r#"fn main() {
    println!("Hello, Rust!");
}"#,
            },
            Aufgabe::Code {
                id: "1.5",
                aufgabe: "Print the three words `Rust`, `is` and `fast`, each on its own line.",
                vorlage: r#"fn main() {
    println!("Rust");
}"#,
                erwartet: "Rust\nis\nfast",
                hinweis: "Every call to `println!` creates a new line. So you need three calls.",
                loesung: r#"fn main() {
    println!("Rust");
    println!("is");
    println!("fast");
}"#,
            },
        ],
    },
    Lektion {
        titel: "Variables & mutability",
        kurz: "let, mut, shadowing and constants.",
        kapitel: "First steps",
        bloecke: &[
            Block::Text("With `let` you give a value a name. In Rust, variables are **immutable by default**. That sounds odd at first, but it prevents many mistakes: if a value can't change, you don't have to watch out everywhere for who changes it."),
            Block::Ausfuehrbar(r#"fn main() {
    let x = 5;
    println!("x is {x}");
    println!("x plus 1 is {}", x + 1);
}"#),
            Block::Text("Inside `println!` you insert values with curly braces: `{x}` inserts the variable directly, `{}` takes the next argument after the comma."),
            Block::Ueberschrift("Mutable with mut"),
            Block::Text("If you try to change a normal variable, the compiler reports an error:"),
            Block::Code(r#"let points = 10;
points = 20; // ❌ error[E0384]: cannot assign twice to immutable variable"#),
            Block::Text("If a variable should be allowed to change, mark it with `mut` (for “mutable”):"),
            Block::Ausfuehrbar(r#"fn main() {
    let mut points = 10;
    println!("Start: {points}");
    points = 20;
    points += 5; // short for: points = points + 5
    println!("Now: {points}");
}"#),
            Block::Ueberschrift("Shadowing – reusing names"),
            Block::Text("You can use `let` to create a new value under the same name. The new variable **shadows** the old one. That's different from `mut`: a new variable is created, and it may even have a different type."),
            Block::Ausfuehrbar(r#"fn main() {
    let input = "   42   ";
    let input = input.trim();    // &str without spaces
    let input = input.len();     // now a number!
    println!("Length: {input}");
}"#),
            Block::Text("Variables only exist inside the block `{ }` they were created in – their **scope**. Click through the example:"),
            Block::Visual(Visual::Scope),
            Block::Ueberschrift("Constants"),
            Block::Text("Values that never change and should be known everywhere are written as `const`. Constants are written in CAPITALS and always need a type. Underscores in numbers are just for readability."),
            Block::Code(r#"const MAX_POINTS: u32 = 100_000;
const PI_ROUGHLY: f64 = 3.14159;"#),
        ],
        aufgaben: &[
            Aufgabe::Quiz {
                id: "2.1",
                frage: "What happens with this code?",
                code: Some("let x = 5;\nx = 6;"),
                optionen: &["x is 6 afterwards", "Compiler error: x is not mutable", "x is 11 afterwards", "The program crashes when it runs"],
                richtig: 1,
                erklaerung: "Without `mut`, `x` is immutable. The compiler reports `E0384` – before the program even runs.",
            },
            Aufgabe::Rechnung {
                id: "2.2",
                frage: "What is the value of `x` at the end?",
                code: Some("let x = 2;\nlet x = x * 3;\nlet x = x + 1;"),
                antworten: &["7"],
                hinweis: "Go line by line. Every `let x` takes the previous value of `x`.",
                erklaerung: "2 → 2 × 3 = 6 → 6 + 1 = 7. Every line creates a new variable `x` through shadowing.",
            },
            Aufgabe::Rechnung {
                id: "2.3",
                frage: "What does the program print?",
                code: Some("let a = 10;\n{\n    let a = a * 2;\n    let b = a + 1;\n}\nprintln!(\"{a}\");"),
                antworten: &["10"],
                hinweis: "The inner `a` only exists inside the curly braces.",
                erklaerung: "The inner `a` (20) only shadows the outer one inside the block. After `}` the outer `a = 10` is visible again.",
            },
            Aufgabe::Code {
                id: "2.4",
                aufgabe: "The program doesn't compile. Fix it so that it prints `Counter: 2`.",
                vorlage: r#"fn main() {
    let counter = 0;
    counter += 1;
    counter += 1;
    println!("Counter: {counter}");
}"#,
                erwartet: "Counter: 2",
                hinweis: "The variable must be mutable. Which keyword is missing after `let`?",
                loesung: r#"fn main() {
    let mut counter = 0;
    counter += 1;
    counter += 1;
    println!("Counter: {counter}");
}"#,
            },
            Aufgabe::Code {
                id: "2.5",
                aufgabe: "Use **shadowing**: shadow the variable `word` with its length (`word.len()`), so that `Length: 6` is printed. Don't change the `println!` line.",
                vorlage: r#"fn main() {
    let word = "Ferris";
    // New line here:

    println!("Length: {word}");
}"#,
                erwartet: "Length: 6",
                hinweis: "Write `let word = word.len();` before the output.",
                loesung: r#"fn main() {
    let word = "Ferris";
    let word = word.len();
    println!("Length: {word}");
}"#,
            },
        ],
    },
    Lektion {
        titel: "Numbers & data types",
        kurz: "Integers, decimals, bits, overflow and more.",
        kapitel: "First steps",
        bloecke: &[
            Block::Text("Rust is **statically typed**: every value has a fixed type that the compiler knows. Most of the time it guesses the type itself (type inference), but you can also write it: `let x: i32 = 5;`"),
            Block::Ueberschrift("Integers"),
            Block::Text("Integer types start with `i` (signed, so they can be negative) or `u` (unsigned, only ≥ 0). The number after it says how many **bits** the type takes. A `u8` has 8 bits and can therefore store 2⁸ = 256 different values: 0 to 255."),
            Block::Visual(Visual::Typgroessen),
            Block::Text("Play with the bits! Click individual bits, switch the type and try what happens with `MAX + 1`. Note: for `i8` the top bit counts **negative** (−128). That's how **two's complement** works."),
            Block::Visual(Visual::Bits),
            Block::Ueberschrift("Calculating"),
            Block::Text("The basic operators are `+ - * /` and `%` (remainder of a division). Careful: with integers, `/` simply cuts off the decimals! `7 / 2` is `3`, not `3.5`. For decimal numbers there are `f32` and `f64` (default)."),
            Block::Ausfuehrbar(r#"fn main() {
    let a = 7;
    let b = 2;
    println!("{a} / {b} = {}", a / b);   // integer division
    println!("{a} % {b} = {}", a % b);   // remainder
    println!("7.0 / 2.0 = {}", 7.0 / 2.0);

    // You can't mix types directly – convert with `as`:
    let average = (a + b) as f64 / 2.0;
    println!("Average: {average}");
    println!("Rounded to 2 decimals: {:.2}", 10.0 / 3.0);
}"#),
            Block::Achtung("**Overflow:** a `u8` can't get bigger than 255. In debug mode Rust stops with a **panic** if you calculate beyond that. If you want a specific behaviour, use `wrapping_add` (wraps around), `checked_add` (returns `None`) or `saturating_add` (stays at the maximum)."),
            Block::Ausfuehrbar(r#"fn main() {
    let x: u8 = 250;
    println!("wrapping:   {}", x.wrapping_add(10));   // 260 - 256 = 4
    println!("checked:    {:?}", x.checked_add(10));  // None
    println!("saturating: {}", x.saturating_add(10)); // 255
    println!("u8 goes up to {}", u8::MAX);
    println!("i32 goes from {} to {}", i32::MIN, i32::MAX);
}"#),
            Block::Ueberschrift("More simple types"),
            Block::Code(r#"let yes: bool = true;                 // true or false
let letter: char = 'ß';               // one Unicode character, single quotes
let pair: (i32, f64) = (3, 2.5);      // tuple: fixed size, mixed types
let first = pair.0;                   // access with .0, .1, ...
let numbers: [i32; 4] = [1, 2, 3, 4]; // array: fixed length, one type
let second = numbers[1];              // indices start at 0!
let binary = 0b1010;                  // = 10
let hex = 0xFF;                       // = 255"#),
        ],
        aufgaben: &[
            Aufgabe::Rechnung {
                id: "3.1",
                frage: "What is the value of `x`?",
                code: Some("let x: i32 = 17 / 5;"),
                antworten: &["3"],
                hinweis: "Integer division drops the remainder. How often does 5 fit into 17?",
                erklaerung: "17 / 5 = 3 remainder 2. With integers only the 3 is left.",
            },
            Aufgabe::Rechnung {
                id: "3.2",
                frage: "What is the value of `rest`?",
                code: Some("let rest = 17 % 5;"),
                antworten: &["2"],
                hinweis: "`%` gives the remainder of the division.",
                erklaerung: "17 = 3 × 5 + 2, so the remainder is 2.",
            },
            Aufgabe::Rechnung {
                id: "3.3",
                frage: "What is the largest value a `u16` can store?",
                code: None,
                antworten: &["65535", "65 535", "65,535"],
                hinweis: "A `u16` has 16 bits. The largest value is 2¹⁶ − 1.",
                erklaerung: "2¹⁶ = 65 536 values, from 0 to 65 535.",
            },
            Aufgabe::Rechnung {
                id: "3.4",
                frage: "What does the program print?",
                code: Some("let x: u8 = 250;\nprintln!(\"{}\", x.wrapping_add(10));"),
                antworten: &["4"],
                hinweis: "With `wrapping_add` the value starts again at 0 after 255. 250 + 10 = 260 …",
                erklaerung: "260 doesn't fit in a u8. 260 − 256 = 4. The value wraps around once.",
            },
            Aufgabe::Rechnung {
                id: "3.5",
                frage: "What is the decimal value of this binary number? (Tip: use the bit graphic above!)",
                code: Some("let b: u8 = 0b1010_0101;"),
                antworten: &["165"],
                hinweis: "Add the place values of all 1 bits: 128, 64, 32, 16, 8, 4, 2, 1.",
                erklaerung: "128 + 32 + 4 + 1 = 165.",
            },
            Aufgabe::Rechnung {
                id: "3.6",
                frage: "What is the smallest value of an `i8`?",
                code: None,
                antworten: &["-128", "−128"],
                hinweis: "For `i8` the top bit counts −2⁷.",
                erklaerung: "An i8 goes from −2⁷ = −128 to 2⁷ − 1 = 127.",
            },
            Aufgabe::Quiz {
                id: "3.7",
                frage: "What happens with this code?",
                code: Some("let a: i32 = 5;\nlet b: f64 = 2.5;\nlet c = a + b;"),
                optionen: &["c is 7.5", "c is 7", "Compiler error: you can't add i32 and f64", "c is \"52.5\""],
                richtig: 2,
                erklaerung: "Rust never converts types silently. You have to write `a as f64 + b` explicitly.",
            },
            Aufgabe::Code {
                id: "3.8",
                aufgabe: "Calculate the average of the five grades as a decimal number and print it with **one** decimal place: `Average: 6.6`",
                vorlage: r#"fn main() {
    let sum = 7 + 9 + 4 + 8 + 5;   // i32
    let count = 5;
    // Convert with `as f64` and print with {:.1}:

}"#,
                erwartet: "Average: 6.6",
                hinweis: "`let avg = sum as f64 / count as f64;` and then `println!(\"Average: {:.1}\", avg);`",
                loesung: r#"fn main() {
    let sum = 7 + 9 + 4 + 8 + 5;
    let count = 5;
    let avg = sum as f64 / count as f64;
    println!("Average: {:.1}", avg);
}"#,
            },
        ],
    },
    Lektion {
        titel: "Functions",
        kurz: "Parameters, return values, expressions and statements.",
        kapitel: "First steps",
        bloecke: &[
            Block::Text("Functions group code so you can reuse it. They're defined with `fn`. **Parameters always need a type**, and the return type comes after the arrow `->`."),
            Block::Ausfuehrbar(r#"fn square(x: i32) -> i32 {
    x * x   // no semicolon → this is the return value
}

fn greet(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    greet("Ferris");
    let q = square(7);
    println!("7² = {q}");
}"#),
            Block::Ueberschrift("Expressions vs. statements"),
            Block::Text("This is an important Rust concept: a **statement** does something but produces no value – it ends with `;`. An **expression** produces a value. The last line of a function without `;` is its return value. Even a block `{ }` is an expression:"),
            Block::Ausfuehrbar(r#"fn main() {
    let y = {
        let x = 3;
        x + 1       // ← value of the block
    };
    println!("y = {y}");
}"#),
            Block::Achtung("One semicolon too many is a typical beginner mistake: `fn f() -> i32 { 5; }` doesn't compile, because `5;` is a statement and the function then returns `()` (nothing). The compiler reports `E0308: mismatched types`."),
            Block::Text("Follow how values go into functions and come back out:"),
            Block::Visual(Visual::Trace {
                code: r#"fn double(x: i32) -> i32 {
    x * 2
}
fn main() {
    let a = 3;
    let b = double(a);
    let c = double(b) + 1;
    println!("{c}");
}"#,
                spalten: &["a", "b", "c", "x (in double)"],
                schritte: &[
                    ts(5, &["3", "–", "–", "–"], "`a` gets the value 3."),
                    ts(2, &["3", "–", "–", "3"], "`double(a)` is called: the parameter `x` gets a copy of `a`, so 3. The function computes 3 × 2."),
                    ts(6, &["3", "6", "–", "–"], "The return value 6 ends up in `b`. `x` doesn't exist any more."),
                    ts(2, &["3", "6", "–", "6"], "Second call: this time `x` = 6, result 12."),
                    ts(7, &["3", "6", "13", "–"], "12 + 1 = 13 is stored in `c`."),
                    ts(8, &["3", "6", "13", "–"], "Output: **13**"),
                ],
            }),
        ],
        aufgaben: &[
            Aufgabe::Quiz {
                id: "4.1",
                frage: "Why doesn't this function compile?",
                code: Some("fn five() -> i32 {\n    5;\n}"),
                optionen: &["Because 5 isn't an i32", "Because `5;` is a statement and returns nothing", "Because the function needs parameters", "It compiles without problems"],
                richtig: 1,
                erklaerung: "The semicolon turns the expression `5` into a statement. Without `;`, `5` would be the return value.",
            },
            Aufgabe::Rechnung {
                id: "4.2",
                frage: "What does `f(f(3))` return?",
                code: Some("fn f(x: i32) -> i32 {\n    x * 2 + 1\n}"),
                antworten: &["15"],
                hinweis: "Work from the inside out: first `f(3)`, then `f` of the result.",
                erklaerung: "f(3) = 3 × 2 + 1 = 7, f(7) = 7 × 2 + 1 = 15.",
            },
            Aufgabe::Code {
                id: "4.3",
                aufgabe: "Complete the function `area` so that it returns width × height. Expected output: `Area: 24`",
                vorlage: r#"fn area(width: u32, height: u32) -> u32 {
    // TODO: compute the area
    0
}

fn main() {
    println!("Area: {}", area(4, 6));
}"#,
                erwartet: "Area: 24",
                hinweis: "Replace the `0` with `width * height` – without a semicolon!",
                loesung: r#"fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn main() {
    println!("Area: {}", area(4, 6));
}"#,
            },
            Aufgabe::Code {
                id: "4.4",
                aufgabe: "Write the function `to_fahrenheit`. Formula: °F = °C × 9 / 5 + 32. Make sure to calculate with decimals (`9.0`, `5.0`, `32.0`).",
                vorlage: r#"fn to_fahrenheit(celsius: f64) -> f64 {
    // TODO
    0.0
}

fn main() {
    for c in [0.0, 37.0, 100.0] {
        println!("{c}°C = {:.1}°F", to_fahrenheit(c));
    }
}"#,
                erwartet: "0°C = 32.0°F\n37°C = 98.6°F\n100°C = 212.0°F",
                hinweis: "The function should return `celsius * 9.0 / 5.0 + 32.0`.",
                loesung: r#"fn to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn main() {
    for c in [0.0, 37.0, 100.0] {
        println!("{c}°C = {:.1}°F", to_fahrenheit(c));
    }
}"#,
            },
        ],
    },
    // ─────────────────────────────── Chapter 2 ───────────────────────────────
    Lektion {
        titel: "Decisions with if",
        kurz: "Conditions, comparisons and if as an expression.",
        kapitel: "Control flow",
        bloecke: &[
            Block::Text("With `if` your program decides which code runs. In Rust the condition must **always be a `bool`** – unlike in some languages, a number doesn't automatically count as true or false."),
            Block::Ausfuehrbar(r#"fn main() {
    let temperature = 23;

    if temperature > 30 {
        println!("Hot! 🥵");
    } else if temperature >= 15 {
        println!("Pleasant. 😊");
    } else {
        println!("Cold! 🥶");
    }
}"#),
            Block::Text("**Comparisons:** `==` equal, `!=` not equal, `<`, `>`, `<=`, `>=`. **Logic:** `&&` (and), `||` (or), `!` (not)."),
            Block::Ueberschrift("if is an expression"),
            Block::Text("Because `if` produces a value, you can use it directly in `let`. Both branches must then have the same type. That's why Rust doesn't need an extra `? :` operator like other languages."),
            Block::Ausfuehrbar(r#"fn main() {
    let number = 7;
    let kind = if number % 2 == 0 { "even" } else { "odd" };
    println!("{number} is {kind}");

    let age = 17;
    let may_vote = age >= 18 && age < 150;
    println!("May vote: {may_vote}");
}"#),
        ],
        aufgaben: &[
            Aufgabe::Quiz {
                id: "5.1",
                frage: "Does this code compile?",
                code: Some("let n = 3;\nif n {\n    println!(\"true\");\n}"),
                optionen: &["Yes, 3 counts as true", "No, the condition must be a bool", "Yes, but nothing is printed", "No, an else is missing"],
                richtig: 1,
                erklaerung: "Rust expects a `bool`. You'd have to write e.g. `if n != 0`.",
            },
            Aufgabe::Rechnung {
                id: "5.2",
                frage: "What is the value of `b`?",
                code: Some("let a = 7;\nlet b = if a % 2 == 0 { a / 2 } else { 3 * a + 1 };"),
                antworten: &["22"],
                hinweis: "Is 7 even? If not, the else branch runs.",
                erklaerung: "7 is odd, so 3 × 7 + 1 = 22. (By the way, that's one step of the famous Collatz sequence!)",
            },
            Aufgabe::Rechnung {
                id: "5.3",
                frage: "What does this expression evaluate to? (Answer `true` or `false`)",
                code: Some("(5 > 3) && !(2 == 2) || (10 % 3 == 1)"),
                antworten: &["true"],
                hinweis: "`&&` binds tighter than `||`. Evaluate the individual parentheses first.",
                erklaerung: "(true && false) || true → false || true → **true**. 10 % 3 is 1.",
            },
            Aufgabe::Code {
                id: "5.4",
                aufgabe: "Write the function `weather`: below 10 degrees → `cold`, from 10 up to and including 25 → `pleasant`, above → `hot`.",
                vorlage: r#"fn weather(degrees: i32) -> &'static str {
    // TODO: if / else if / else
    "?"
}

fn main() {
    println!("5: {}", weather(5));
    println!("18: {}", weather(18));
    println!("25: {}", weather(25));
    println!("30: {}", weather(30));
}"#,
                erwartet: "5: cold\n18: pleasant\n25: pleasant\n30: hot",
                hinweis: "`if degrees < 10 { \"cold\" } else if degrees <= 25 { \"pleasant\" } else { \"hot\" }` – without a semicolon, so it's the return value.",
                loesung: r#"fn weather(degrees: i32) -> &'static str {
    if degrees < 10 {
        "cold"
    } else if degrees <= 25 {
        "pleasant"
    } else {
        "hot"
    }
}

fn main() {
    println!("5: {}", weather(5));
    println!("18: {}", weather(18));
    println!("25: {}", weather(25));
    println!("30: {}", weather(30));
}"#,
            },
        ],
    },
    Lektion {
        titel: "Loops",
        kurz: "loop, while, for and ranges.",
        kapitel: "Control flow",
        bloecke: &[
            Block::Text("Rust has three kinds of loops: `loop` (forever, until `break`), `while` (as long as a condition holds) and `for` (over a sequence of values). You'll use `for` most often."),
            Block::Ausfuehrbar(r#"fn main() {
    // for over a range: 1, 2, 3, 4, 5
    for i in 1..=5 {
        print!("{i} ");
    }
    println!();

    // while: countdown
    let mut n = 3;
    while n > 0 {
        println!("{n}…");
        n -= 1;
    }
    println!("Liftoff! 🚀");

    // loop can return a value with break
    let mut x = 1;
    let result = loop {
        x *= 2;
        if x > 100 {
            break x;
        }
    };
    println!("First power of two above 100: {result}");
}"#),
            Block::Tipp("`0..5` means 0, 1, 2, 3, 4 (the 5 is **not** included). `0..=5` includes the 5. With `(0..5).rev()` you go backwards. `continue` jumps to the next iteration, `break` ends the loop."),
            Block::Ueberschrift("A loop step by step"),
            Block::Text("Press **Play** and watch how `i` and `sum` change in every iteration:"),
            Block::Visual(Visual::Trace {
                code: r#"fn main() {
    let mut sum = 0;
    for i in 1..=4 {
        sum += i;
    }
    println!("Sum: {sum}");
}"#,
                spalten: &["i", "sum"],
                schritte: &[
                    ts(2, &["–", "0"], "`sum` starts at 0. It must be `mut` because it changes."),
                    ts(3, &["1", "0"], "First iteration: `i` is 1."),
                    ts(4, &["1", "1"], "sum = 0 + 1 = **1**"),
                    ts(3, &["2", "1"], "Second iteration: `i` is 2."),
                    ts(4, &["2", "3"], "sum = 1 + 2 = **3**"),
                    ts(3, &["3", "3"], "Third iteration: `i` is 3."),
                    ts(4, &["3", "6"], "sum = 3 + 3 = **6**"),
                    ts(3, &["4", "6"], "Fourth iteration: `i` is 4 (because of `..=` the 4 is included)."),
                    ts(4, &["4", "10"], "sum = 6 + 4 = **10**"),
                    ts(6, &["–", "10"], "The loop is done. Output: `Sum: 10`"),
                ],
            }),
        ],
        aufgaben: &[
            Aufgabe::Rechnung {
                id: "6.1",
                frage: "How many times does the loop body run?",
                code: Some("for i in 3..=7 {\n    println!(\"{i}\");\n}"),
                antworten: &["5"],
                hinweis: "Count the values: 3, 4, … up to and including 7.",
                erklaerung: "3, 4, 5, 6, 7 – so 5 iterations. In general: `a..=b` has b − a + 1 values.",
            },
            Aufgabe::Rechnung {
                id: "6.2",
                frage: "What does this program print? Young Carl Friedrich Gauss supposedly solved this in seconds as a schoolboy.",
                code: Some("let mut sum = 0;\nfor i in 1..=100 {\n    sum += i;\n}\nprintln!(\"{sum}\");"),
                antworten: &["5050", "5 050", "5,050"],
                hinweis: "Make pairs: 1 + 100, 2 + 99, 3 + 98 … Every pair is 101. How many pairs are there? Formula: n × (n + 1) / 2",
                erklaerung: "50 pairs × 101 = 5050. Or with the formula: 100 × 101 / 2 = 5050.",
            },
            Aufgabe::Rechnung {
                id: "6.3",
                frage: "What is the value of `steps` at the end?",
                code: Some("let mut n = 1;\nlet mut steps = 0;\nwhile n < 100 {\n    n *= 3;\n    steps += 1;\n}"),
                antworten: &["5"],
                hinweis: "Write down the values of `n`: 1, 3, 9, … When is n no longer smaller than 100?",
                erklaerung: "n: 1 → 3 → 9 → 27 → 81 → 243. After 5 steps n = 243 ≥ 100.",
            },
            Aufgabe::Code {
                id: "6.4",
                aufgabe: "The classic **FizzBuzz**: print the numbers from 1 to 15. For multiples of 3 print `Fizz`, for multiples of 5 `Buzz`, for multiples of both `FizzBuzz`.",
                vorlage: r#"fn main() {
    for i in 1..=15 {
        // Tip: i % 3 == 0 means "divisible by 3"
        println!("{i}");
    }
}"#,
                erwartet: "1\n2\nFizz\n4\nBuzz\nFizz\n7\n8\nFizz\nBuzz\n11\nFizz\n13\n14\nFizzBuzz",
                hinweis: "Check **first** for `i % 15 == 0` (FizzBuzz), then for 3, then for 5, otherwise the number.",
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
                aufgabe: "Print the 7 times table – but only up to the first result greater than 40 (including that one). Use `loop` and `break`.",
                vorlage: r#"fn main() {
    let mut i = 1;
    loop {
        let result = 7 * i;
        println!("7 x {i} = {result}");
        // TODO: end the loop when result > 40
        i += 1;
        if i > 20 { break; } // emergency brake
    }
}"#,
                erwartet: "7 x 1 = 7\n7 x 2 = 14\n7 x 3 = 21\n7 x 4 = 28\n7 x 5 = 35\n7 x 6 = 42",
                hinweis: "After the output, add `if result > 40 { break; }`.",
                loesung: r#"fn main() {
    let mut i = 1;
    loop {
        let result = 7 * i;
        println!("7 x {i} = {result}");
        if result > 40 {
            break;
        }
        i += 1;
    }
}"#,
            },
        ],
    },
    // ─────────────────────────────── Chapter 3 ───────────────────────────────
    Lektion {
        titel: "Ownership",
        kurz: "Stack, heap, move and clone – Rust's superpower.",
        kapitel: "Ownership",
        bloecke: &[
            Block::Text("Now comes the concept that makes Rust unique: **ownership**. With it, Rust manages memory automatically and safely – without a garbage collector cleaning up in the background."),
            Block::Ueberschrift("Stack and heap"),
            Block::Text("Your program has two memory areas. The **stack** is fast and tidy like a stack of plates: values with a fixed, known size (`i32`, `bool`, …) go on top and are removed in reverse order. The **heap** is a big store for data whose size can change – for example a `String` that can grow. On the stack there's then only a **pointer** to the data."),
            Block::Ueberschrift("The three rules"),
            Block::Text("**1.** Every value has exactly one owner. **2.** There is only one owner at a time. **3.** When the owner leaves its scope, the value is cleaned up (drop)."),
            Block::Visual(Visual::Speicher {
                code: r#"fn main() {
    let a = 5;
    let b = a;
    let s1 = String::from("hello");
    let s2 = s1;
    let s3 = s2.clone();
    println!("{a} {b} {s2} {s3}");
}"#,
                schritte: &[
                    SpeicherSchritt { zeile: 2, stack: &[sv("a", "5", N)], heap: &[], text: "`a` is an `i32` with a fixed size (4 bytes) – it lives directly on the stack.", ausgabe: "" },
                    SpeicherSchritt { zeile: 3, stack: &[sv("a", "5", OK), sv("b", "5", N)], heap: &[], text: "Simple numbers are `Copy`: `b` gets a real copy. `a` stays valid because copying is cheap.", ausgabe: "" },
                    SpeicherSchritt { zeile: 4, stack: &[sv("a", "5", OK), sv("b", "5", OK), sp("s1", "ptr, len 5, cap 5", 0, N)], heap: &[hb("h e l l o", N)], text: "A `String` stores its letters on the **heap**. The stack only holds the pointer, length and capacity.", ausgabe: "" },
                    SpeicherSchritt { zeile: 5, stack: &[sv("a", "5", OK), sv("b", "5", OK), sp("s1", "ptr, len 5, cap 5", 0, V), sp("s2", "ptr, len 5, cap 5", 0, N)], heap: &[hb("h e l l o", OK)], text: "**Move!** Only the stack data is copied, and `s2` becomes the new owner. `s1` is **invalid** from now on – otherwise two variables would free the same memory at the end (a dangerous “double free”).", ausgabe: "" },
                    SpeicherSchritt { zeile: 6, stack: &[sv("a", "5", OK), sv("b", "5", OK), sp("s1", "ptr, len 5, cap 5", 0, V), sp("s2", "ptr, len 5, cap 5", 0, OK), sp("s3", "ptr, len 5, cap 5", 1, N)], heap: &[hb("h e l l o", OK), hb("h e l l o", N)], text: "`clone()` makes a **deep copy**: the heap data is copied too. `s3` has its own memory. That costs time – which is why you have to write it explicitly.", ausgabe: "" },
                    SpeicherSchritt { zeile: 7, stack: &[sv("a", "5", OK), sv("b", "5", OK), sp("s1", "ptr, len 5, cap 5", 0, V), sp("s2", "ptr, len 5, cap 5", 0, OK), sp("s3", "ptr, len 5, cap 5", 1, OK)], heap: &[hb("h e l l o", OK), hb("h e l l o", OK)], text: "We use `a`, `b`, `s2` and `s3`. We may not use `s1` any more – the compiler would report `E0382: borrow of moved value`.", ausgabe: "5 5 hello hello" },
                    SpeicherSchritt { zeile: 8, stack: &[sv("a", "5", F), sv("b", "5", F), sp("s1", "ptr, len 5, cap 5", 0, V), sp("s2", "ptr, len 5, cap 5", 0, F), sp("s3", "ptr, len 5, cap 5", 1, F)], heap: &[hb("h e l l o", F), hb("h e l l o", F)], text: "End of the scope: Rust calls `drop` automatically. Every piece of heap memory is freed **exactly once** by its owner. No leak, no double free.", ausgabe: "" },
                ],
            }),
            Block::Ueberschrift("Ownership and functions"),
            Block::Text("When you pass a `String` to a function, it is **moved** there. Afterwards you can't use it in the caller any more – unless the function gives it back:"),
            Block::Ausfuehrbar(r#"fn take(s: String) -> String {
    println!("I have: {s}");
    s // give ownership back
}

fn main() {
    let text = String::from("Ferris");
    let text = take(text); // there and back
    println!("Back again: {text}");
}"#),
            Block::Tipp("Passing values back and forth all the time is tedious. In the next lesson you'll get to know **references**: they let you borrow a value without owning it."),
        ],
        aufgaben: &[
            Aufgabe::Quiz {
                id: "7.1",
                frage: "What is true after `let s2 = s1;` if `s1` is a `String`?",
                code: None,
                optionen: &["s1 and s2 both validly point to the same text", "s1 is invalid, s2 is the new owner", "The text was copied on the heap", "s2 is empty"],
                richtig: 1,
                erklaerung: "That's a **move**. Only one owner at a time – `s1` may no longer be used.",
            },
            Aufgabe::Quiz {
                id: "7.2",
                frage: "Which of these types is `Copy` (copied instead of moved on assignment)?",
                code: None,
                optionen: &["`String`", "`Vec<i32>`", "`i32`", "`Box<i32>`"],
                richtig: 2,
                erklaerung: "Simple fixed-size types on the stack (`i32`, `f64`, `bool`, `char`, …) are `Copy`. Types with heap data aren't.",
            },
            Aufgabe::Quiz {
                id: "7.3",
                frage: "In which line does the compiler report an error?",
                code: Some("let s = String::from(\"Ferris\");  // line 1\nlet t = s;                       // line 2\nprintln!(\"{t}\");                 // line 3\nprintln!(\"{s}\");                 // line 4"),
                optionen: &["Line 2", "Line 3", "Line 4", "None"],
                richtig: 2,
                erklaerung: "In line 2, `s` is moved into `t`. Line 4 tries to use `s` → `E0382: borrow of moved value`.",
            },
            Aufgabe::Code {
                id: "7.4",
                aufgabe: "The program doesn't compile because `name` was moved. Fix it with `.clone()` so that `Ferris and Ferris` is printed.",
                vorlage: r#"fn main() {
    let name = String::from("Ferris");
    let copy = name;
    println!("{name} and {copy}");
}"#,
                erwartet: "Ferris and Ferris",
                hinweis: "Write `let copy = name.clone();` – then `copy` gets its own copy.",
                loesung: r#"fn main() {
    let name = String::from("Ferris");
    let copy = name.clone();
    println!("{name} and {copy}");
}"#,
            },
            Aufgabe::Code {
                id: "7.5",
                aufgabe: "`greet` takes ownership of the string, so the second call doesn't work. Fix `main` (the function stays as it is) so that `Hello, Ada!` appears twice.",
                vorlage: r#"fn greet(name: String) {
    println!("Hello, {name}!");
}

fn main() {
    let n = String::from("Ada");
    greet(n);
    greet(n);
}"#,
                erwartet: "Hello, Ada!\nHello, Ada!",
                hinweis: "Pass a copy in the first call: `greet(n.clone());`",
                loesung: r#"fn greet(name: String) {
    println!("Hello, {name}!");
}

fn main() {
    let n = String::from("Ada");
    greet(n.clone());
    greet(n);
}"#,
            },
        ],
    },
    Lektion {
        titel: "References & borrowing",
        kurz: "Borrowing values with & and &mut.",
        kapitel: "Ownership",
        bloecke: &[
            Block::Text("A **reference** `&x` is like a note with the address of a value. You can use the value without owning it – you **borrow** it. The owner keeps its value."),
            Block::Ausfuehrbar(r#"fn length(s: &String) -> usize {
    s.len() // only reading
}

fn append(s: &mut String) {
    s.push_str(", world"); // changing
}

fn main() {
    let mut text = String::from("Hello");
    let n = length(&text);       // borrow for reading
    append(&mut text);           // borrow for changing
    println!("{text} ({n} characters before)");
}"#),
            Block::Ueberschrift("The borrowing rules"),
            Block::Text("At any time **either**: any number of shared references `&T` **or** exactly one mutable reference `&mut T`. Many readers or one writer – never both. Also, references must always be valid: there can be no reference to memory that has already been cleaned up."),
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
                    SpeicherSchritt { zeile: 2, stack: &[sp("s", "ptr, len 2, cap 2", 0, N)], heap: &[hb("H i", N)], text: "`s` owns a String on the heap.", ausgabe: "" },
                    SpeicherSchritt { zeile: 3, stack: &[sp("s", "ptr, len 2, cap 2", 0, OK), sv("r1", "&s  → points to s", G)], heap: &[hb("H i", OK)], text: "`r1` borrows `s` for **reading**. `s` stays the owner.", ausgabe: "" },
                    SpeicherSchritt { zeile: 4, stack: &[sp("s", "ptr, len 2, cap 2", 0, OK), sv("r1", "&s  → points to s", G), sv("r2", "&s  → points to s", G)], heap: &[hb("H i", OK)], text: "Several shared references at the same time are allowed – reading doesn't disturb anyone.", ausgabe: "" },
                    SpeicherSchritt { zeile: 5, stack: &[sp("s", "ptr, len 2, cap 2", 0, OK), sv("r1", "&s", F), sv("r2", "&s", F)], heap: &[hb("H i", OK)], text: "After their last use, the borrows of `r1` and `r2` end automatically (the compiler notices that).", ausgabe: "Hi Hi" },
                    SpeicherSchritt { zeile: 6, stack: &[sp("s", "ptr, len 2, cap 2", 0, OK), sv("r1", "&s", F), sv("r2", "&s", F), sv("r3", "&mut s  → points to s", N)], heap: &[hb("H i", OK)], text: "Now there are no readers left, so `r3` may borrow **exclusively** and mutably.", ausgabe: "" },
                    SpeicherSchritt { zeile: 7, stack: &[sp("s", "ptr, len 3, cap 4", 0, OK), sv("r1", "&s", F), sv("r2", "&s", F), sv("r3", "&mut s  → points to s", G)], heap: &[hb("H i !", N)], text: "The String is changed through `r3`. Since nobody else is reading at the same time, that's safe.", ausgabe: "" },
                    SpeicherSchritt { zeile: 8, stack: &[sp("s", "ptr, len 3, cap 4", 0, OK), sv("r1", "&s", F), sv("r2", "&s", F), sv("r3", "&mut s", F)], heap: &[hb("H i !", OK)], text: "The borrow is over – `s` can be used directly again and contains the change.", ausgabe: "Hi!" },
                ],
            }),
            Block::Text("This code breaks the rule and therefore doesn't compile:"),
            Block::Code(r#"let mut s = String::from("Hi");
let reader = &s;
let writer = &mut s;      // ❌ E0502: cannot borrow `s` as mutable
println!("{reader}");     //    because it is also borrowed as immutable"#),
            Block::Tipp("With `*` you access the value behind a reference (dereferencing): `*number += 1;` increases the number that `number: &mut i32` points to. With method calls like `s.push('!')` Rust does this automatically."),
        ],
        aufgaben: &[
            Aufgabe::Quiz {
                id: "8.1",
                frage: "How many mutable references (`&mut`) to the same value may exist at the same time?",
                code: None,
                optionen: &["Any number", "Exactly one", "Two", "None – &mut doesn't exist"],
                richtig: 1,
                erklaerung: "Exactly one – and while it exists, no shared references either. That's how Rust prevents data races.",
            },
            Aufgabe::Quiz {
                id: "8.2",
                frage: "What does `fn f(x: &String)` mean?",
                code: None,
                optionen: &["f takes ownership of the string", "f only borrows the string for reading", "f can change the string", "f gets a copy"],
                richtig: 1,
                erklaerung: "`&String` is a shared (read-only) reference. To change it you'd need `&mut String`.",
            },
            Aufgabe::Code {
                id: "8.3",
                aufgabe: "Complete `double`, so that the number behind the reference is doubled. Expected output: `42`",
                vorlage: r#"fn double(number: &mut i32) {
    // TODO: use * to dereference
}

fn main() {
    let mut x = 21;
    double(&mut x);
    println!("{x}");
}"#,
                erwartet: "42",
                hinweis: "`*number *= 2;` – the first `*` dereferences, the second is the multiplication.",
                loesung: r#"fn double(number: &mut i32) {
    *number *= 2;
}

fn main() {
    let mut x = 21;
    double(&mut x);
    println!("{x}");
}"#,
            },
            Aufgabe::Code {
                id: "8.4",
                aufgabe: "Change `shout` so that it borrows the string with `&mut` instead of owning it. Then `main` should print `Hello!!!` afterwards.",
                vorlage: r#"fn shout(s: String) {
    s.push_str("!!!");
}

fn main() {
    let greeting = String::from("Hello");
    shout(greeting);
    println!("{greeting}");
}"#,
                erwartet: "Hello!!!",
                hinweis: "Three places: parameter `s: &mut String`, variable `let mut greeting` and call `shout(&mut greeting)`.",
                loesung: r#"fn shout(s: &mut String) {
    s.push_str("!!!");
}

fn main() {
    let mut greeting = String::from("Hello");
    shout(&mut greeting);
    println!("{greeting}");
}"#,
            },
        ],
    },
    Lektion {
        titel: "Strings & slices",
        kurz: "String vs. &str, UTF-8 and pieces of text.",
        kapitel: "Ownership",
        bloecke: &[
            Block::Text("Rust has two main types for text: `String` owns its text on the heap and can grow. `&str` (a **string slice**) is a borrowed view of text – for example of a piece of a `String` or of a fixed text in the program like `\"Hello\"`."),
            Block::Ausfuehrbar(r#"fn main() {
    let fixed: &str = "Hello";                // fixed in the program
    let mut own = String::from(fixed);        // own, mutable text
    own.push_str(", world");
    own.push('!');

    let part: &str = &own[0..5];              // slice: bytes 0 to 4
    println!("{own} | Part: {part}");
    println!("Uppercase: {}", own.to_uppercase());
    println!("Contains 'world': {}", own.contains("world"));

    let sentence = format!("{} has {} bytes", own, own.len());
    println!("{sentence}");

    for word in "Rust is great".split_whitespace() {
        println!("- {word}");
    }
}"#),
            Block::Ueberschrift("UTF-8: bytes aren't characters"),
            Block::Text("Rust strings are always valid **UTF-8**. Letters like `a` take 1 byte, accented letters like `é` or `ü` take 2 bytes and emojis like 🦀 even 4. `len()` counts **bytes**, not characters! For characters you use `chars()`."),
            Block::Ausfuehrbar(r#"fn main() {
    let word = "café";
    println!("Bytes:      {}", word.len());
    println!("Characters: {}", word.chars().count());
    for (i, c) in word.char_indices() {
        println!("Byte {i}: '{c}' ({} byte)", c.len_utf8());
    }
}"#),
            Block::Achtung("`s[0]` does **not** work on strings – because it would be unclear whether you mean a byte or a character. And a slice like `&\"café\"[0..4]` crashes, because byte 4 is in the middle of the `é`. Use `chars()` or methods like `find`, `split` and `starts_with`."),
            Block::Tipp("Write functions that only read text with `&str` as the parameter: `fn f(s: &str)`. Then you can pass both `&String` and `\"text\"`."),
        ],
        aufgaben: &[
            Aufgabe::Rechnung {
                id: "9.1",
                frage: "What does `\"naïve\".len()` return?",
                code: None,
                antworten: &["6"],
                hinweis: "`len()` counts bytes. n, a, v and e take 1 byte each – ï takes 2.",
                erklaerung: "1 + 1 + 2 + 1 + 1 = 6 bytes, but only 5 characters.",
            },
            Aufgabe::Rechnung {
                id: "9.2",
                frage: "What does `\"Ferris 🦀\".chars().count()` return?",
                code: None,
                antworten: &["8"],
                hinweis: "Count characters, not bytes: letters, the space and the emoji.",
                erklaerung: "F-e-r-r-i-s (6) + space (1) + 🦀 (1) = 8 characters. In bytes it would be 6 + 1 + 4 = 11.",
            },
            Aufgabe::Quiz {
                id: "9.3",
                frage: "What is `part`?",
                code: Some("let s = String::from(\"Ferris\");\nlet part = &s[1..4];"),
                optionen: &["`\"Fer\"`", "`\"err\"`", "`\"erri\"`", "`\"Ferr\"`"],
                richtig: 1,
                erklaerung: "Index 1 up to but not including 4: bytes 1, 2, 3 → `e`, `r`, `r`.",
            },
            Aufgabe::Code {
                id: "9.4",
                aufgabe: "Write `first_word`, which returns the first word of a text. Expected output: `Hello` and `Rust`",
                vorlage: r#"fn first_word(s: &str) -> &str {
    // TODO: tip: s.split_whitespace().next()
    s
}

fn main() {
    println!("{}", first_word("Hello beautiful world"));
    println!("{}", first_word("Rust"));
}"#,
                erwartet: "Hello\nRust",
                hinweis: "`s.split_whitespace().next()` gives an `Option<&str>`. With `.unwrap_or(\"\")` you get the `&str`.",
                loesung: r#"fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

fn main() {
    println!("{}", first_word("Hello beautiful world"));
    println!("{}", first_word("Rust"));
}"#,
            },
            Aufgabe::Code {
                id: "9.5",
                aufgabe: "Build the text `1-2-3-4-5` with a loop and print it.",
                vorlage: r#"fn main() {
    let mut text = String::new();
    for i in 1..=5 {
        // TODO: append the number, and a dash – except after the last number
    }
    println!("{text}");
}"#,
                erwartet: "1-2-3-4-5",
                hinweis: "`text.push_str(&i.to_string());` appends the number. With `if i < 5 { text.push('-'); }` the dash.",
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
    // ─────────────────────────────── Chapter 4 ───────────────────────────────
    Lektion {
        titel: "Structs & methods",
        kurz: "Your own data types with fields and behaviour.",
        kapitel: "Your own types",
        bloecke: &[
            Block::Text("With a `struct` you group related data into your own type. Every field has a name and a type. In an `impl` block you define **methods** – functions that belong to your type."),
            Block::Ausfuehrbar(r#"#[derive(Debug)] // allows printing with {:?}
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // "Associated function" without self – like a constructor
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    // Method: only reads (&self)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method: changes (&mut self)
    fn double(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }
}

fn main() {
    let mut r = Rectangle::new(3, 4);
    println!("{r:?} has area {}", r.area());
    r.double();
    println!("After double: {}×{} = {}", r.width, r.height, r.area());
}"#),
            Block::Text("**Remember:** `&self` = the method only reads. `&mut self` = the method changes the object. `self` = the method consumes the object (ownership!). Without `self` it's an associated function that you call with `Type::name()`."),
            Block::Tipp("With `#[derive(Debug)]` Rust automatically generates code to print your struct. `{:?}` shows it compactly, `{:#?}` nicely formatted over several lines."),
        ],
        aufgaben: &[
            Aufgabe::Quiz {
                id: "10.1",
                frage: "How do you call the associated function `new` of `Rectangle`?",
                code: None,
                optionen: &["`Rectangle.new(3, 4)`", "`Rectangle::new(3, 4)`", "`new Rectangle(3, 4)`", "`Rectangle->new(3, 4)`"],
                richtig: 1,
                erklaerung: "Associated functions (without `self`) are called on the type with `::`. Methods are called on an instance with `.`.",
            },
            Aufgabe::Quiz {
                id: "10.2",
                frage: "A method should change a field of the struct. Which first parameter does it need?",
                code: None,
                optionen: &["`self`", "`&self`", "`&mut self`", "None"],
                richtig: 2,
                erklaerung: "To change it, the method needs a mutable reference to the object: `&mut self`.",
            },
            Aufgabe::Rechnung {
                id: "10.3",
                frage: "What is `a.balance` at the end?",
                code: Some("struct Account { balance: i64 }\n\nimpl Account {\n    fn deposit(&mut self, amount: i64) { self.balance += amount; }\n    fn withdraw(&mut self, amount: i64) {\n        if amount <= self.balance { self.balance -= amount; }\n    }\n}\n\nlet mut a = Account { balance: 100 };\na.deposit(50);\na.withdraw(200);\na.withdraw(30);\na.deposit(5);"),
                antworten: &["125"],
                hinweis: "Careful with `withdraw(200)`: the condition `amount <= self.balance` must be met.",
                erklaerung: "100 + 50 = 150. Withdrawing 200 doesn't work (not enough money). 150 − 30 = 120, + 5 = 125.",
            },
            Aufgabe::Code {
                id: "10.4",
                aufgabe: "Implement the method `perimeter` (2 × width + 2 × height). Expected output: `Area: 12, perimeter: 14`",
                vorlage: r#"struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        // TODO
        0
    }
}

fn main() {
    let r = Rectangle { width: 3, height: 4 };
    println!("Area: {}, perimeter: {}", r.area(), r.perimeter());
}"#,
                erwartet: "Area: 12, perimeter: 14",
                hinweis: "Access the fields with `self.width` and `self.height`.",
                loesung: r#"struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * self.width + 2 * self.height
    }
}

fn main() {
    let r = Rectangle { width: 3, height: 4 };
    println!("Area: {}, perimeter: {}", r.area(), r.perimeter());
}"#,
            },
            Aufgabe::Code {
                id: "10.5",
                aufgabe: "Add the method `increment`, which increases `value` by 1. Mind the right `self` parameter. Expected output: `Counter: 3`",
                vorlage: r#"struct Counter {
    value: u32,
}

impl Counter {
    // TODO: fn increment(...)
}

fn main() {
    let mut c = Counter { value: 0 };
    c.increment();
    c.increment();
    c.increment();
    println!("Counter: {}", c.value);
}"#,
                erwartet: "Counter: 3",
                hinweis: "`fn increment(&mut self) { self.value += 1; }`",
                loesung: r#"struct Counter {
    value: u32,
}

impl Counter {
    fn increment(&mut self) {
        self.value += 1;
    }
}

fn main() {
    let mut c = Counter { value: 0 };
    c.increment();
    c.increment();
    c.increment();
    println!("Counter: {}", c.value);
}"#,
            },
        ],
    },
    Lektion {
        titel: "Enums & match",
        kurz: "Variants, pattern matching and Option.",
        kapitel: "Your own types",
        bloecke: &[
            Block::Text("An `enum` describes a value that is **exactly one** of several variants. The special thing in Rust: every variant can carry its own data. With `match` you check which variant you have – and the compiler makes sure you **don't forget a case**."),
            Block::Visual(Visual::Match),
            Block::Ueberschrift("Option – Rust has no null"),
            Block::Text("Many languages have `null` for “no value” – a huge source of bugs. Rust uses the enum `Option<T>` instead: either `Some(value)` or `None`. You have to handle the `None` case before you get to the value."),
            Block::Ausfuehrbar(r#"fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    match divide(10, 2) {
        Some(x) => println!("Result: {x}"),
        None => println!("You can't divide by zero!"),
    }

    // Short form if you only care about one case:
    if let Some(x) = divide(9, 3) {
        println!("if let: {x}");
    }

    // Default value if None:
    let y = divide(1, 0).unwrap_or(-1);
    println!("unwrap_or: {y}");
}"#),
            Block::Ueberschrift("Patterns in match"),
            Block::Code(r#"match number {
    0 => println!("zero"),
    1 | 2 => println!("one or two"),          // several values
    3..=9 => println!("single digit"),        // range
    n if n < 0 => println!("negative: {n}"),  // guard with if
    _ => println!("something else"),          // everything else
}"#),
        ],
        aufgaben: &[
            Aufgabe::Quiz {
                id: "11.1",
                frage: "What is `Option<T>` for in Rust?",
                code: None,
                optionen: &["For a setting in the program", "For a value that may be present (`Some`) or not (`None`)", "For an error", "For an optional function"],
                richtig: 1,
                erklaerung: "Option replaces `null`. The compiler forces you to handle the `None` case.",
            },
            Aufgabe::Quiz {
                id: "11.2",
                frage: "What happens if a `match` doesn't cover all variants of an enum?",
                code: None,
                optionen: &["The missing case is skipped", "The program crashes when the case occurs", "The compiler reports an error", "The first branch is taken automatically"],
                richtig: 2,
                erklaerung: "`match` must be exhaustive – otherwise `E0004`. With `_ =>` you catch all remaining cases.",
            },
            Aufgabe::Rechnung {
                id: "11.3",
                frage: "What is the value of `result`?",
                code: Some("let number = 7;\nlet result = match number {\n    1 => 10,\n    2 | 3 => 20,\n    4..=9 => 30,\n    _ => 0,\n};"),
                antworten: &["30"],
                hinweis: "Which pattern matches 7 first?",
                erklaerung: "7 lies in the range `4..=9`, so 30.",
            },
            Aufgabe::Code {
                id: "11.4",
                aufgabe: "Write the function `value`, which returns the value of each coin in cents (5, 10, 20, 50). Expected output: `Total: 85 cents`",
                vorlage: r#"enum Coin {
    Five,
    Ten,
    Twenty,
    Fifty,
}

fn value(c: &Coin) -> u32 {
    // TODO: match c { ... }
    0
}

fn main() {
    let purse = [Coin::Twenty, Coin::Fifty, Coin::Ten, Coin::Five];
    let mut total = 0;
    for c in &purse {
        total += value(c);
    }
    println!("Total: {total} cents");
}"#,
                erwartet: "Total: 85 cents",
                hinweis: "`match c { Coin::Five => 5, Coin::Ten => 10, ... }`",
                loesung: r#"enum Coin {
    Five,
    Ten,
    Twenty,
    Fifty,
}

fn value(c: &Coin) -> u32 {
    match c {
        Coin::Five => 5,
        Coin::Ten => 10,
        Coin::Twenty => 20,
        Coin::Fifty => 50,
    }
}

fn main() {
    let purse = [Coin::Twenty, Coin::Fifty, Coin::Ten, Coin::Five];
    let mut total = 0;
    for c in &purse {
        total += value(c);
    }
    println!("Total: {total} cents");
}"#,
            },
            Aufgabe::Code {
                id: "11.5",
                aufgabe: "Write `square_root`: for negative numbers `None`, otherwise `Some(x.sqrt())`. Handle both cases in `main` with `match`.",
                vorlage: r#"fn square_root(x: f64) -> Option<f64> {
    // TODO
    None
}

fn main() {
    for x in [16.0, -4.0] {
        match square_root(x) {
            Some(r) => println!("√{x} = {r}"),
            None => println!("√{x} doesn't exist"),
        }
    }
}"#,
                erwartet: "√16 = 4\n√-4 doesn't exist",
                hinweis: "`if x < 0.0 { None } else { Some(x.sqrt()) }`",
                loesung: r#"fn square_root(x: f64) -> Option<f64> {
    if x < 0.0 {
        None
    } else {
        Some(x.sqrt())
    }
}

fn main() {
    for x in [16.0, -4.0] {
        match square_root(x) {
            Some(r) => println!("√{x} = {r}"),
            None => println!("√{x} doesn't exist"),
        }
    }
}"#,
            },
        ],
    },
    Lektion {
        titel: "Vec & HashMap",
        kurz: "Lists and dictionaries that can grow.",
        kapitel: "Your own types",
        bloecke: &[
            Block::Text("An array has a fixed length. If you need a list that can grow and shrink, you use a **`Vec<T>`** (vector). Its elements are stored one after another on the heap."),
            Block::Text("Try `push` and `pop`. Watch **len** (number of elements) and **cap** (reserved space): when the space is full, Rust reserves twice as much new memory and copies everything over."),
            Block::Visual(Visual::Vec),
            Block::Ausfuehrbar(r#"fn main() {
    let mut numbers = vec![3, 1, 4];
    numbers.push(1);
    numbers.push(5);
    println!("{numbers:?}, length {}", numbers.len());

    // Reading: with an index (can crash) or get (returns an Option)
    println!("First: {}", numbers[0]);
    println!("Tenth: {:?}", numbers.get(9));

    // Iterate and change
    for n in &mut numbers {
        *n *= 10;
    }
    let sum: i32 = numbers.iter().sum();
    println!("{numbers:?}, sum {sum}");

    numbers.sort();
    println!("Sorted: {numbers:?}");
}"#),
            Block::Ueberschrift("HashMap – keys and values"),
            Block::Text("A `HashMap<K, V>` maps **keys** to values, like a dictionary. `entry(...).or_insert(...)` is especially handy: “get the entry, or create it with a starting value.”"),
            Block::Ausfuehrbar(r#"use std::collections::HashMap;

fn main() {
    let mut points: HashMap<&str, u32> = HashMap::new();
    points.insert("Ada", 10);
    points.insert("Linus", 7);

    if let Some(p) = points.get("Ada") {
        println!("Ada has {p} points");
    }

    // Counting words
    let text = "the dog and the ball and the tree";
    let mut count = HashMap::new();
    for word in text.split_whitespace() {
        *count.entry(word).or_insert(0) += 1;
    }
    println!("'the' appears {} times", count["the"]);
    println!("'and' appears {} times", count["and"]);
}"#),
            Block::Tipp("The order in a `HashMap` is random. If you need sorted keys, use `BTreeMap` – it works almost the same."),
        ],
        aufgaben: &[
            Aufgabe::Rechnung {
                id: "12.1",
                frage: "What is the value of `sum` at the end?",
                code: Some("let mut v = vec![1, 2, 3];\nv.push(4);\nv.pop();\nv.push(10);\nlet sum: i32 = v.iter().sum();"),
                antworten: &["16"],
                hinweis: "`pop` removes the **last** element. Write down the vector after every step.",
                erklaerung: "[1,2,3] → [1,2,3,4] → [1,2,3] → [1,2,3,10]. Sum: 16.",
            },
            Aufgabe::Quiz {
                id: "12.2",
                frage: "`v` has 3 elements. What happens with `v[10]` and with `v.get(10)`?",
                code: None,
                optionen: &["Both return 0", "`v[10]` crashes (panic), `v.get(10)` returns `None`", "Both crash", "`v[10]` returns `None`, `v.get(10)` crashes"],
                richtig: 1,
                erklaerung: "Indexing with `[]` checks the bounds and stops. `get` returns an `Option` you can handle safely.",
            },
            Aufgabe::Rechnung {
                id: "12.3",
                frage: "An empty `Vec<i32>` gets room for 4 elements on the first `push` and doubles its capacity whenever it's full. What's the capacity after 9 calls to `push`?",
                code: None,
                antworten: &["16"],
                hinweis: "4 → 8 → … The space is full at the 5th and at the 9th element.",
                erklaerung: "Elements 1–4: cap 4. Element 5: cap 8. Element 9: cap 16. (Try it in the graphic above!)",
            },
            Aufgabe::Code {
                id: "12.4",
                aufgabe: "Calculate the sum and maximum of the numbers with a `for` loop. Expected output: `Sum: 75, maximum: 42`",
                vorlage: r#"fn main() {
    let numbers = vec![3, 17, 8, 42, 5];
    let mut sum = 0;
    let mut maximum = 0;
    for n in &numbers {
        // TODO
    }
    println!("Sum: {sum}, maximum: {maximum}");
}"#,
                erwartet: "Sum: 75, maximum: 42",
                hinweis: "`n` is a reference `&i32`. Use `*n`: `sum += *n;` and `if *n > maximum { maximum = *n; }`",
                loesung: r#"fn main() {
    let numbers = vec![3, 17, 8, 42, 5];
    let mut sum = 0;
    let mut maximum = 0;
    for n in &numbers {
        sum += *n;
        if *n > maximum {
            maximum = *n;
        }
    }
    println!("Sum: {sum}, maximum: {maximum}");
}"#,
            },
            Aufgabe::Code {
                id: "12.5",
                aufgabe: "Use a `HashMap` to count how often each colour appears. Expected output: see below.",
                vorlage: r#"use std::collections::HashMap;

fn main() {
    let text = "red blue red green red blue";
    let mut counter: HashMap<&str, u32> = HashMap::new();
    // TODO: increase the counter for every word in the text

    for colour in ["red", "blue", "green"] {
        println!("{colour}: {}", counter.get(colour).unwrap_or(&0));
    }
}"#,
                erwartet: "red: 3\nblue: 2\ngreen: 1",
                hinweis: "`for word in text.split_whitespace() { *counter.entry(word).or_insert(0) += 1; }`",
                loesung: r#"use std::collections::HashMap;

fn main() {
    let text = "red blue red green red blue";
    let mut counter: HashMap<&str, u32> = HashMap::new();
    for word in text.split_whitespace() {
        *counter.entry(word).or_insert(0) += 1;
    }

    for colour in ["red", "blue", "green"] {
        println!("{colour}: {}", counter.get(colour).unwrap_or(&0));
    }
}"#,
            },
        ],
    },
    // ─────────────────────────────── Chapter 5 ───────────────────────────────
    Lektion {
        titel: "Error handling",
        kurz: "panic!, Result and the ? operator.",
        kapitel: "Advanced",
        bloecke: &[
            Block::Text("Rust distinguishes two kinds of errors: **unrecoverable** errors (a programming mistake, e.g. an index out of bounds) lead to `panic!` – the program stops. **Recoverable** errors (e.g. wrong user input) are returned with the enum `Result<T, E>`: `Ok(value)` or `Err(error)`."),
            Block::Visual(Visual::Parse),
            Block::Ausfuehrbar(r#"fn main() {
    let inputs = ["42", "-7", "three"];
    for e in inputs {
        match e.parse::<i32>() {
            Ok(number) => println!("{e:>5} → number {number}"),
            Err(error) => println!("{e:>5} → error: {error}"),
        }
    }
}"#),
            Block::Ueberschrift("The ? operator"),
            Block::Text("Often you just want to pass an error on to the caller. That's what `?` is for: if the result is `Ok`, `?` unwraps the value. If it's `Err`, `?` ends the function immediately and returns the error. This only works in functions that return a `Result` themselves."),
            Block::Ausfuehrbar(r#"use std::num::ParseIntError;

fn double(text: &str) -> Result<i32, ParseIntError> {
    let number: i32 = text.trim().parse()?; // on error: return immediately
    Ok(number * 2)
}

fn main() {
    println!("{:?}", double("21"));
    println!("{:?}", double("abc"));
}"#),
            Block::Achtung("`unwrap()` and `expect(\"…\")` get the value out of an `Option` or `Result` – and trigger a **panic** on `None`/`Err`. Fine for quick experiments, but in real programs you'd better handle errors with `match`, `if let`, `unwrap_or` or `?`."),
        ],
        aufgaben: &[
            Aufgabe::Quiz {
                id: "13.1",
                frage: "What does the `?` operator do with an `Err` value?",
                code: None,
                optionen: &["It ignores the error", "It crashes the program", "It returns the error from the current function immediately", "It turns the error into 0"],
                richtig: 2,
                erklaerung: "`?` is short for: with `Ok(v)` continue with `v`, with `Err(e)` immediately `return Err(e)`.",
            },
            Aufgabe::Quiz {
                id: "13.2",
                frage: "What does `\"256\".parse::<u8>()` return?",
                code: None,
                optionen: &["`Ok(256)`", "`Ok(0)`", "`Err(...)`, because 256 doesn't fit in a u8", "A panic"],
                richtig: 2,
                erklaerung: "A u8 only goes up to 255. `parse` reports that cleanly as an `Err` – no crash.",
            },
            Aufgabe::Rechnung {
                id: "13.3",
                frage: "What is the value of `n`?",
                code: Some("let a: i32 = \"42\".parse().unwrap_or(0);\nlet b: i32 = \"x\".parse().unwrap_or(8);\nlet n = a + b;"),
                antworten: &["50"],
                hinweis: "\"42\" can be parsed, \"x\" can't – then the default value applies.",
                erklaerung: "a = 42, b = 8 (because \"x\" isn't a number), so n = 50.",
            },
            Aufgabe::Code {
                id: "13.4",
                aufgabe: "Complete `sum_text` with the `?` operator: parse both texts and return the sum.",
                vorlage: r#"use std::num::ParseIntError;

fn sum_text(a: &str, b: &str) -> Result<i32, ParseIntError> {
    // TODO: convert a and b with .parse::<i32>()?
    Ok(0)
}

fn main() {
    for (a, b) in [("10", "32"), ("10", "abc")] {
        match sum_text(a, b) {
            Ok(s) => println!("Sum: {s}"),
            Err(e) => println!("Error: {e}"),
        }
    }
}"#,
                erwartet: "Sum: 42\nError: invalid digit found in string",
                hinweis: "`let x = a.parse::<i32>()?;` and the same for `b`. Then `Ok(x + y)`.",
                loesung: r#"use std::num::ParseIntError;

fn sum_text(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x = a.parse::<i32>()?;
    let y = b.parse::<i32>()?;
    Ok(x + y)
}

fn main() {
    for (a, b) in [("10", "32"), ("10", "abc")] {
        match sum_text(a, b) {
            Ok(s) => println!("Sum: {s}"),
            Err(e) => println!("Error: {e}"),
        }
    }
}"#,
            },
        ],
    },
    Lektion {
        titel: "Traits & generics",
        kurz: "Shared behaviour and code for many types.",
        kapitel: "Advanced",
        bloecke: &[
            Block::Text("A **trait** describes what a type *can do* – similar to an interface in other languages. Different types can implement the same trait in their own way."),
            Block::Visual(Visual::Formen),
            Block::Ausfuehrbar(r#"trait Animal {
    fn name(&self) -> String;
    fn sound(&self) -> String;

    // Default implementation that every animal gets
    fn introduce(&self) -> String {
        format!("{} says {}", self.name(), self.sound())
    }
}

struct Dog;
struct Cat { name: String }

impl Animal for Dog {
    fn name(&self) -> String { "The dog".to_string() }
    fn sound(&self) -> String { "Woof!".to_string() }
}

impl Animal for Cat {
    fn name(&self) -> String { self.name.clone() }
    fn sound(&self) -> String { "Meow!".to_string() }
}

fn main() {
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog),
        Box::new(Cat { name: "Minka".to_string() }),
    ];
    for a in &animals {
        println!("{}", a.introduce());
    }
}"#),
            Block::Ueberschrift("Generics"),
            Block::Text("With **generics** you write code that works for many types. `T` is a placeholder. With a **trait bound** like `T: PartialOrd` you say: “`T` can be any type that can be compared.” When compiling, Rust creates its own fast version for every type used."),
            Block::Ausfuehrbar(r#"fn larger<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

fn main() {
    println!("{}", larger(3, 7));
    println!("{}", larger(2.5, 1.5));
    println!("{}", larger("Apple", "Pear"));
}"#),
            Block::Tipp("Many traits can be derived automatically: `#[derive(Debug, Clone, PartialEq)]`. Important standard traits are `Display` (for `{}`), `Debug` (for `{:?}`), `Clone`, `PartialEq` (for `==`) and `PartialOrd` (for `<`)."),
        ],
        aufgaben: &[
            Aufgabe::Quiz {
                id: "14.1",
                frage: "What is a trait?",
                code: None,
                optionen: &["A special kind of variable", "A description of behaviour (methods) that types can implement", "An error type", "Another name for struct"],
                richtig: 1,
                erklaerung: "Traits define shared behaviour. Types implement it with `impl Trait for Type`.",
            },
            Aufgabe::Rechnung {
                id: "14.2",
                frage: "A `Circle { r: 2.0 }` computes `area()` as π × r². What's the result? (Rounded to 2 decimals – use the graphic above!)",
                code: None,
                antworten: &["12.57", "12,57"],
                hinweis: "π ≈ 3.14159. Calculate 3.14159 × 2 × 2.",
                erklaerung: "π × 4 ≈ 12.566… rounded 12.57.",
            },
            Aufgabe::Code {
                id: "14.3",
                aufgabe: "Implement the trait `Shape` for `Square`. Expected output: `Area: 25`",
                vorlage: r#"trait Shape {
    fn area(&self) -> f64;
}

struct Square {
    side: f64,
}

// TODO: impl Shape for Square { ... }

fn main() {
    let s = Square { side: 5.0 };
    println!("Area: {}", s.area());
}"#,
                erwartet: "Area: 25",
                hinweis: "`impl Shape for Square { fn area(&self) -> f64 { self.side * self.side } }`",
                loesung: r#"trait Shape {
    fn area(&self) -> f64;
}

struct Square {
    side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn main() {
    let s = Square { side: 5.0 };
    println!("Area: {}", s.area());
}"#,
            },
            Aufgabe::Code {
                id: "14.4",
                aufgabe: "Write the generic function `largest`, which finds the largest element of a list. It should work for integers and decimals.",
                vorlage: r#"fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];
    // TODO: go through all elements and update max
    max
}

fn main() {
    println!("Largest number: {}", largest(&[3, 42, 7, 19]));
    println!("Largest decimal: {}", largest(&[1.5, 0.2, 3.5, 2.0]));
}"#,
                erwartet: "Largest number: 42\nLargest decimal: 3.5",
                hinweis: "`for &x in list { if x > max { max = x; } }` – the `&x` unpacks the reference directly.",
                loesung: r#"fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];
    for &x in list {
        if x > max {
            max = x;
        }
    }
    max
}

fn main() {
    println!("Largest number: {}", largest(&[3, 42, 7, 19]));
    println!("Largest decimal: {}", largest(&[1.5, 0.2, 3.5, 2.0]));
}"#,
            },
        ],
    },
    Lektion {
        titel: "Closures & iterators",
        kurz: "Small functions and elegant data processing.",
        kapitel: "Advanced",
        bloecke: &[
            Block::Text("A **closure** is a small, nameless function that you can store in a variable or pass on. It's written with vertical bars: `|x| x + 1`. Closures can use (“capture”) variables from their environment."),
            Block::Ausfuehrbar(r#"fn main() {
    let plus_one = |x: i32| x + 1;
    println!("{}", plus_one(5));

    let factor = 3;
    let times_factor = |x| x * factor;   // captures `factor`
    println!("{}", times_factor(4));

    let mut counter = 0;
    let mut count = || counter += 1;     // changes the environment
    count();
    count();
    println!("Counted: {counter}");
}"#),
            Block::Ueberschrift("Iterators"),
            Block::Text("An **iterator** delivers values one after another. With methods like `map` (transform), `filter` (sort out) and `sum`, `count` or `collect` (gather) you build processing chains. Build one yourself:"),
            Block::Visual(Visual::Iterator),
            Block::Ausfuehrbar(r#"fn main() {
    let names = vec!["ada", "linus", "grace", "ferris"];

    let upper: Vec<String> = names
        .iter()
        .filter(|n| n.len() > 3)
        .map(|n| n.to_uppercase())
        .collect();
    println!("{upper:?}");

    for (i, n) in names.iter().enumerate() {
        println!("{}. {n}", i + 1);
    }

    let total: usize = names.iter().map(|n| n.len()).sum();
    println!("Letters in total: {total}");
}"#),
            Block::Tipp("Iterators in Rust are just as fast as hand-written loops – the compiler optimises them completely. They're called **zero-cost abstractions**."),
        ],
        aufgaben: &[
            Aufgabe::Rechnung {
                id: "15.1",
                frage: "What does this expression evaluate to?",
                code: Some("(1..=5).map(|x| x * x).sum::<i32>()"),
                antworten: &["55"],
                hinweis: "Square 1 to 5 and add them up: 1 + 4 + 9 + …",
                erklaerung: "1 + 4 + 9 + 16 + 25 = 55.",
            },
            Aufgabe::Rechnung {
                id: "15.2",
                frage: "What does this expression evaluate to?",
                code: Some("(1..=20).filter(|x| x % 3 == 0).count()"),
                antworten: &["6"],
                hinweis: "How many multiples of 3 are there between 1 and 20?",
                erklaerung: "3, 6, 9, 12, 15, 18 → 6 of them. Or: 20 / 3 = 6 (integer division).",
            },
            Aufgabe::Rechnung {
                id: "15.3",
                frage: "What does the program print?",
                code: Some("let factor = 3;\nlet times = |x: i32| x * factor;\nprintln!(\"{}\", times(times(2)));"),
                antworten: &["18"],
                hinweis: "From the inside out: `times(2)` first.",
                erklaerung: "times(2) = 6, times(6) = 18.",
            },
            Aufgabe::Code {
                id: "15.4",
                aufgabe: "Use an iterator chain to calculate the sum of the **squares of all even numbers** from 1 to 10. Expected output: `Result: 220`",
                vorlage: r#"fn main() {
    let result: i32 = (1..=10)
        // TODO: filter and map
        .sum();
    println!("Result: {result}");
}"#,
                erwartet: "Result: 220",
                hinweis: "`.filter(|x| x % 2 == 0).map(|x| x * x)` before `.sum()`",
                loesung: r#"fn main() {
    let result: i32 = (1..=10)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .sum();
    println!("Result: {result}");
}"#,
            },
            Aufgabe::Code {
                id: "15.5",
                aufgabe: "Take all words with **more than 3 letters**, convert them to uppercase and join them with `, `.",
                vorlage: r#"fn main() {
    let words = ["rust", "is", "super", "fast", "and", "safe"];
    let result: Vec<String> = words
        .iter()
        // TODO
        .collect();
    println!("{}", result.join(", "));
}"#,
                erwartet: "RUST, SUPER, FAST, SAFE",
                hinweis: "`.filter(|w| w.len() > 3).map(|w| w.to_uppercase())`",
                loesung: r#"fn main() {
    let words = ["rust", "is", "super", "fast", "and", "safe"];
    let result: Vec<String> = words
        .iter()
        .filter(|w| w.len() > 3)
        .map(|w| w.to_uppercase())
        .collect();
    println!("{}", result.join(", "));
}"#,
            },
        ],
    },
    Lektion {
        titel: "Final project: grade calculator",
        kurz: "Everything together: structs, Vec, iterators, Option.",
        kapitel: "Advanced",
        bloecke: &[
            Block::Text("Time to bring it all together! You'll build a small **grade calculator** for a school class (grades from 1 to 6, 6 is the best, 4 is a pass). Along the way you use structs, methods, vectors, iterators and `Option`."),
            Block::Ausfuehrbar(r#"struct Student {
    name: String,
    grades: Vec<f64>,
}

impl Student {
    fn new(name: &str, grades: &[f64]) -> Self {
        Self { name: name.to_string(), grades: grades.to_vec() }
    }

    fn average(&self) -> Option<f64> {
        if self.grades.is_empty() {
            return None;
        }
        let sum: f64 = self.grades.iter().sum();
        Some(sum / self.grades.len() as f64)
    }
}

fn main() {
    let class = vec![
        Student::new("Ada", &[5.5, 6.0, 5.0]),
        Student::new("Linus", &[4.0, 4.5, 3.5]),
        Student::new("New", &[]),
    ];

    for s in &class {
        match s.average() {
            Some(n) => println!("{:<6} {:.2} {}", s.name, n, if n >= 4.0 { "✅" } else { "❌" }),
            None => println!("{:<6} no grades yet", s.name),
        }
    }
}"#),
            Block::Text("Create the project in the terminal. On the **terminal page** (top menu) you can even edit the file in the editor (`code src/main.rs`) and run it with `cargo run`."),
            Block::Terminal,
            Block::Ueberschrift("What's next?"),
            Block::Text("Congratulations – you now know the most important basics of Rust! Worth learning next: **lifetimes** (`'a`), **modules** and visibility (`mod`, `pub`), **tests** with `#[test]`, **smart pointers** (`Box`, `Rc`, `RefCell`) and **concurrency** with threads. The official book “The Rust Programming Language” is free at doc.rust-lang.org/book. For practice, the Rustlings exercises are great."),
        ],
        aufgaben: &[
            Aufgabe::Terminal {
                id: "16.1",
                aufgabe: "Create the project `grades`, change into it, look at the files with `tree` and run it.",
                ziele: &["cargo new grades", "cd grades", "tree", "cargo run"],
                hinweis: "Four commands in a row: `cargo new grades`, `cd grades`, `tree`, `cargo run`.",
            },
            Aufgabe::Quiz {
                id: "16.2",
                frage: "Why does `average()` return an `Option<f64>` instead of just `f64`?",
                code: None,
                optionen: &["Because f64 is too imprecise", "Because there's no average without grades (division by 0)", "Because Option is faster", "Because methods must always return Option"],
                richtig: 1,
                erklaerung: "With an empty list it would be 0 / 0. With `None` the function honestly says: there's no result here.",
            },
            Aufgabe::Rechnung {
                id: "16.3",
                frage: "Ada has the grades 5.5, 6.0 and 5.0. What is her average? (2 decimals)",
                code: None,
                antworten: &["5.50", "5.5", "5,50", "5,5"],
                hinweis: "Sum divided by count: (5.5 + 6.0 + 5.0) / 3",
                erklaerung: "16.5 / 3 = 5.50 ✅",
            },
            Aufgabe::Code {
                id: "16.4",
                aufgabe: "Extend the grade calculator: write the method `best_grade`, which returns the highest grade as an `Option<f64>`. Then print the best grade of every student.",
                vorlage: r#"struct Student {
    name: String,
    grades: Vec<f64>,
}

impl Student {
    fn best_grade(&self) -> Option<f64> {
        // TODO: tip: a loop with an Option variable,
        // or: self.grades.iter().copied().reduce(f64::max)
        None
    }
}

fn main() {
    let class = vec![
        Student { name: "Ada".to_string(), grades: vec![5.5, 6.0, 5.0] },
        Student { name: "Linus".to_string(), grades: vec![4.0, 4.5, 3.5] },
        Student { name: "New".to_string(), grades: vec![] },
    ];
    for s in &class {
        match s.best_grade() {
            Some(n) => println!("{}: best grade {n}", s.name),
            None => println!("{}: no grades", s.name),
        }
    }
}"#,
                erwartet: "Ada: best grade 6\nLinus: best grade 4.5\nNew: no grades",
                hinweis: "Simplest solution: `self.grades.iter().copied().reduce(f64::max)`. `reduce` returns `None` for an empty list.",
                loesung: r#"struct Student {
    name: String,
    grades: Vec<f64>,
}

impl Student {
    fn best_grade(&self) -> Option<f64> {
        let mut best: Option<f64> = None;
        for &n in &self.grades {
            best = match best {
                Some(b) if b >= n => Some(b),
                _ => Some(n),
            };
        }
        best
    }
}

fn main() {
    let class = vec![
        Student { name: "Ada".to_string(), grades: vec![5.5, 6.0, 5.0] },
        Student { name: "Linus".to_string(), grades: vec![4.0, 4.5, 3.5] },
        Student { name: "New".to_string(), grades: vec![] },
    ];
    for s in &class {
        match s.best_grade() {
            Some(n) => println!("{}: best grade {n}", s.name),
            None => println!("{}: no grades", s.name),
        }
    }
}"#,
            },
        ],
    },
];
