//! All lessons in Spanish. Every lesson builds on the previous one.

use super::Status::{Freigegeben as F, Geliehen as G, Gueltig as OK, Neu as N, Verschoben as V};
use super::{Aufgabe, Block, Lektion, SpeicherSchritt, Visual, hb, sp, sv, ts};

pub static LEKTIONEN_ES: &[Lektion] = &[
    // ─────────────────────────────── Capítulo 1 ───────────────────────────────
    Lektion {
        titel: "¡Hola, Rust!",
        kurz: "Tu primer programa, el compilador y Cargo.",
        kapitel: "Primeros pasos",
        bloecke: &[
            Block::Text("¡Bienvenido! **Rust** es un lenguaje de programación moderno, tan rápido como C o C++, pero que te protege de errores típicos: fallos por memoria inválida, casos de error olvidados o accesos simultáneos. El truco: el **compilador** revisa tu programa con mucho cuidado antes de que llegue a ejecutarse."),
            Block::Text("La mascota de la comunidad de Rust es **Ferris**, un pequeño cangrejo 🦀. A los programadores de Rust les gusta llamarse «rustáceos»."),
            Block::Ueberschrift("Tu primer programa"),
            Block::Text("Todo programa en Rust empieza en la función `main`. Pulsa **Ejecutar**: el código se compila y se ejecuta de verdad. ¡También puedes cambiarlo!"),
            Block::Ausfuehrbar(r#"fn main() {
    // Esto es un comentario: el compilador lo ignora.
    println!("¡Hola, mundo!");
    println!("Estoy aprendiendo Rust. 🦀");
}"#),
            Block::Text("¿Qué ocurre aquí? `fn` inicia una función, `main` es su nombre y las llaves `{ }` encierran su contenido. `println!` imprime una línea de texto. Cada instrucción termina con un punto y coma `;`."),
            Block::Tipp("`println!` no es una función normal, sino una **macro**: se reconoce por el signo de exclamación `!`. Las macros son pequeños generadores de código. Por ahora basta con saber que se llaman con `!`."),
            Block::Ueberschrift("Del código al programa"),
            Block::Text("Rust es un lenguaje **compilado**. Tu texto se traduce a código máquina que el procesador entiende directamente. Recorre los pasos:"),
            Block::Visual(Visual::Pipeline),
            Block::Ueberschrift("Cargo: tu caja de herramientas"),
            Block::Text("En la práctica casi nunca llamas al compilador `rustc` directamente. En su lugar usas **Cargo**: crea proyectos, gestiona bibliotecas (llamadas **crates**) y compila tu programa. Un proyecto nuevo tiene este aspecto:"),
            Block::Code(r#"$ cargo new hola
$ tree hola
hola
├── Cargo.toml      // información del proyecto y dependencias
└── src
    └── main.rs     // tu código"#),
            Block::Text("Con `cargo run` el proyecto se compila y se ejecuta. ¡Pruébalo en el terminal! Escribe uno tras otro `cargo new hola`, `cd hola`, `cat src/main.rs` y `cargo run`."),
            Block::Terminal,
        ],
        aufgaben: &[
            Aufgabe::Quiz {
                id: "1.1",
                frage: "¿Qué función se ejecuta automáticamente al arrancar un programa en Rust?",
                code: None,
                optionen: &["`start()`", "`main()`", "`run()`", "`init()`"],
                richtig: 1,
                erklaerung: "¡Exacto! `fn main()` es el punto de entrada de todo programa ejecutable en Rust.",
            },
            Aufgabe::Quiz {
                id: "1.2",
                frage: "¿Cómo sabes que `println!` es una macro y no una función?",
                code: None,
                optionen: &["Por el signo de exclamación `!`", "Por los paréntesis", "Por el punto y coma final", "No se puede saber"],
                richtig: 0,
                erklaerung: "En Rust, las macros siempre se llaman con `!`, p. ej. `println!`, `vec!` o `format!`.",
            },
            Aufgabe::Terminal {
                id: "1.3",
                aufgabe: "Crea con Cargo un proyecto llamado `hola`, entra en su carpeta y ejecuta el programa.",
                ziele: &["cargo new hola", "cd hola", "cargo run"],
                hinweis: "Escribe los tres comandos tal cual y pulsa Enter después de cada uno.",
            },
            Aufgabe::Code {
                id: "1.4",
                aufgabe: "Cambia el programa para que imprima exactamente `¡Hola, Rust!`.",
                vorlage: r#"fn main() {
    // Escribe aquí tu código:

}"#,
                erwartet: "¡Hola, Rust!",
                hinweis: "Usa `println!(\"...\");`: no olvides las comillas ni el punto y coma.",
                loesung: r#"fn main() {
    println!("¡Hola, Rust!");
}"#,
            },
            Aufgabe::Code {
                id: "1.5",
                aufgabe: "Imprime las tres palabras `Rust`, `es` y `rápido`, cada una en su propia línea.",
                vorlage: r#"fn main() {
    println!("Rust");
}"#,
                erwartet: "Rust\nes\nrápido",
                hinweis: "Cada llamada a `println!` crea una línea nueva. Necesitas tres llamadas.",
                loesung: r#"fn main() {
    println!("Rust");
    println!("es");
    println!("rápido");
}"#,
            },
        ],
    },
    Lektion {
        titel: "Variables y mutabilidad",
        kurz: "let, mut, shadowing y constantes.",
        kapitel: "Primeros pasos",
        bloecke: &[
            Block::Text("Con `let` le das un nombre a un valor. En Rust, las variables son **inmutables por defecto**. Al principio suena raro, pero evita muchos errores: si un valor no puede cambiar, no tienes que vigilar quién lo cambia."),
            Block::Ausfuehrbar(r#"fn main() {
    let x = 5;
    println!("x vale {x}");
    println!("x más 1 vale {}", x + 1);
}"#),
            Block::Text("Dentro de `println!` insertas valores con llaves: `{x}` inserta la variable directamente y `{}` toma el siguiente argumento después de la coma."),
            Block::Ueberschrift("Mutable con mut"),
            Block::Text("Si intentas cambiar una variable normal, el compilador da un error:"),
            Block::Code(r#"let puntos = 10;
puntos = 20; // ❌ error[E0384]: cannot assign twice to immutable variable"#),
            Block::Text("Si una variable debe poder cambiar, márcala con `mut` (de «mutable»):"),
            Block::Ausfuehrbar(r#"fn main() {
    let mut puntos = 10;
    println!("Inicio: {puntos}");
    puntos = 20;
    puntos += 5; // forma corta de: puntos = puntos + 5
    println!("Ahora: {puntos}");
}"#),
            Block::Ueberschrift("Shadowing: reutilizar nombres"),
            Block::Text("Con `let` puedes crear un valor nuevo con el mismo nombre. La nueva variable **oculta** (shadowing) a la anterior. No es lo mismo que `mut`: se crea una variable nueva, que incluso puede tener otro tipo."),
            Block::Ausfuehrbar(r#"fn main() {
    let entrada = "   42   ";
    let entrada = entrada.trim();    // &str sin espacios
    let entrada = entrada.len();     // ¡ahora es un número!
    println!("Longitud: {entrada}");
}"#),
            Block::Text("Las variables solo existen dentro del bloque `{ }` en el que se crearon: su **ámbito** (scope). Recorre el ejemplo:"),
            Block::Visual(Visual::Scope),
            Block::Ueberschrift("Constantes"),
            Block::Text("Los valores que nunca cambian y deben conocerse en todas partes se escriben como `const`. Las constantes se escriben en MAYÚSCULAS y siempre necesitan un tipo. Los guiones bajos en los números solo ayudan a leerlos."),
            Block::Code(r#"const MAX_PUNTOS: u32 = 100_000;
const PI_APROX: f64 = 3.14159;"#),
        ],
        aufgaben: &[
            Aufgabe::Quiz {
                id: "2.1",
                frage: "¿Qué ocurre con este código?",
                code: Some("let x = 5;\nx = 6;"),
                optionen: &["x vale 6 después", "Error de compilación: x no es mutable", "x vale 11 después", "El programa falla al ejecutarse"],
                richtig: 1,
                erklaerung: "Sin `mut`, `x` es inmutable. El compilador da el error `E0384`, antes incluso de ejecutar.",
            },
            Aufgabe::Rechnung {
                id: "2.2",
                frage: "¿Qué valor tiene `x` al final?",
                code: Some("let x = 2;\nlet x = x * 3;\nlet x = x + 1;"),
                antworten: &["7"],
                hinweis: "Ve línea a línea. Cada `let x` toma el valor anterior de `x`.",
                erklaerung: "2 → 2 × 3 = 6 → 6 + 1 = 7. Cada línea crea una nueva variable `x` mediante shadowing.",
            },
            Aufgabe::Rechnung {
                id: "2.3",
                frage: "¿Qué imprime el programa?",
                code: Some("let a = 10;\n{\n    let a = a * 2;\n    let b = a + 1;\n}\nprintln!(\"{a}\");"),
                antworten: &["10"],
                hinweis: "La `a` interior solo existe dentro de las llaves.",
                erklaerung: "La `a` interior (20) solo oculta a la exterior dentro del bloque. Después de `}` vuelve a verse la `a = 10` exterior.",
            },
            Aufgabe::Code {
                id: "2.4",
                aufgabe: "El programa no compila. Arréglalo para que imprima `Contador: 2`.",
                vorlage: r#"fn main() {
    let contador = 0;
    contador += 1;
    contador += 1;
    println!("Contador: {contador}");
}"#,
                erwartet: "Contador: 2",
                hinweis: "La variable tiene que ser mutable. ¿Qué palabra clave falta después de `let`?",
                loesung: r#"fn main() {
    let mut contador = 0;
    contador += 1;
    contador += 1;
    println!("Contador: {contador}");
}"#,
            },
            Aufgabe::Code {
                id: "2.5",
                aufgabe: "Usa **shadowing**: oculta la variable `palabra` con su longitud (`palabra.len()`) para que se imprima `Longitud: 6`. No cambies la línea de `println!`.",
                vorlage: r#"fn main() {
    let palabra = "Ferris";
    // Nueva línea aquí:

    println!("Longitud: {palabra}");
}"#,
                erwartet: "Longitud: 6",
                hinweis: "Escribe `let palabra = palabra.len();` antes de imprimir.",
                loesung: r#"fn main() {
    let palabra = "Ferris";
    let palabra = palabra.len();
    println!("Longitud: {palabra}");
}"#,
            },
        ],
    },
    Lektion {
        titel: "Números y tipos de datos",
        kurz: "Enteros, decimales, bits, desbordamiento y más.",
        kapitel: "Primeros pasos",
        bloecke: &[
            Block::Text("Rust tiene **tipado estático**: cada valor tiene un tipo fijo que el compilador conoce. Casi siempre lo deduce solo (inferencia de tipos), pero también puedes indicarlo: `let x: i32 = 5;`"),
            Block::Ueberschrift("Enteros"),
            Block::Text("Los tipos enteros empiezan por `i` (con signo, también negativos) o `u` (sin signo, solo ≥ 0). El número de detrás indica cuántos **bits** ocupa el tipo. Un `u8` tiene 8 bits y por eso puede guardar 2⁸ = 256 valores distintos: de 0 a 255."),
            Block::Visual(Visual::Typgroessen),
            Block::Text("¡Juega con los bits! Haz clic en bits sueltos, cambia el tipo y prueba qué pasa con `MAX + 1`. Fíjate: en `i8` el bit superior cuenta en **negativo** (−128). Así funciona el **complemento a dos**."),
            Block::Visual(Visual::Bits),
            Block::Ueberschrift("Calcular"),
            Block::Text("Las operaciones básicas son `+ - * /` y `%` (resto de una división). ¡Cuidado! Con enteros, `/` corta los decimales sin más: `7 / 2` es `3`, no `3.5`. Para números decimales existen `f32` y `f64` (por defecto)."),
            Block::Ausfuehrbar(r#"fn main() {
    let a = 7;
    let b = 2;
    println!("{a} / {b} = {}", a / b);   // división entera
    println!("{a} % {b} = {}", a % b);   // resto
    println!("7.0 / 2.0 = {}", 7.0 / 2.0);

    // No se pueden mezclar tipos directamente: convierte con `as`:
    let media = (a + b) as f64 / 2.0;
    println!("Media: {media}");
    println!("Redondeado a 2 decimales: {:.2}", 10.0 / 3.0);
}"#),
            Block::Achtung("**Desbordamiento:** un `u8` no puede pasar de 255. En modo debug, Rust se detiene con un **panic** si calculas por encima. Si quieres un comportamiento concreto, usa `wrapping_add` (da la vuelta), `checked_add` (devuelve `None`) o `saturating_add` (se queda en el máximo)."),
            Block::Ausfuehrbar(r#"fn main() {
    let x: u8 = 250;
    println!("wrapping:   {}", x.wrapping_add(10));   // 260 - 256 = 4
    println!("checked:    {:?}", x.checked_add(10));  // None
    println!("saturating: {}", x.saturating_add(10)); // 255
    println!("u8 llega hasta {}", u8::MAX);
    println!("i32 va de {} a {}", i32::MIN, i32::MAX);
}"#),
            Block::Ueberschrift("Más tipos sencillos"),
            Block::Code(r#"let verdad: bool = true;              // true o false
let letra: char = 'ñ';                // un carácter Unicode, comillas simples
let par: (i32, f64) = (3, 2.5);       // tupla: tamaño fijo, tipos mezclados
let primero = par.0;                  // acceso con .0, .1, ...
let numeros: [i32; 4] = [1, 2, 3, 4]; // array: longitud fija, un solo tipo
let segundo = numeros[1];             // ¡los índices empiezan en 0!
let binario = 0b1010;                 // = 10
let hex = 0xFF;                       // = 255"#),
        ],
        aufgaben: &[
            Aufgabe::Rechnung {
                id: "3.1",
                frage: "¿Qué valor tiene `x`?",
                code: Some("let x: i32 = 17 / 5;"),
                antworten: &["3"],
                hinweis: "La división entera descarta el resto. ¿Cuántas veces cabe 5 en 17?",
                erklaerung: "17 / 5 = 3, resto 2. Con enteros solo queda el 3.",
            },
            Aufgabe::Rechnung {
                id: "3.2",
                frage: "¿Qué valor tiene `resto`?",
                code: Some("let resto = 17 % 5;"),
                antworten: &["2"],
                hinweis: "`%` da el resto de la división.",
                erklaerung: "17 = 3 × 5 + 2, así que el resto es 2.",
            },
            Aufgabe::Rechnung {
                id: "3.3",
                frage: "¿Cuál es el valor más grande que puede guardar un `u16`?",
                code: None,
                antworten: &["65535", "65 535", "65.535"],
                hinweis: "Un `u16` tiene 16 bits. El valor máximo es 2¹⁶ − 1.",
                erklaerung: "2¹⁶ = 65 536 valores, de 0 a 65 535.",
            },
            Aufgabe::Rechnung {
                id: "3.4",
                frage: "¿Qué imprime el programa?",
                code: Some("let x: u8 = 250;\nprintln!(\"{}\", x.wrapping_add(10));"),
                antworten: &["4"],
                hinweis: "Con `wrapping_add` el valor vuelve a empezar en 0 después de 255. 250 + 10 = 260…",
                erklaerung: "260 no cabe en un u8. 260 − 256 = 4. El valor da la vuelta una vez.",
            },
            Aufgabe::Rechnung {
                id: "3.5",
                frage: "¿Qué valor decimal tiene este número binario? (Consejo: ¡usa el gráfico de bits de arriba!)",
                code: Some("let b: u8 = 0b1010_0101;"),
                antworten: &["165"],
                hinweis: "Suma los valores posicionales de todos los bits a 1: 128, 64, 32, 16, 8, 4, 2, 1.",
                erklaerung: "128 + 32 + 4 + 1 = 165.",
            },
            Aufgabe::Rechnung {
                id: "3.6",
                frage: "¿Cuál es el valor más pequeño de un `i8`?",
                code: None,
                antworten: &["-128", "−128"],
                hinweis: "En `i8` el bit superior cuenta −2⁷.",
                erklaerung: "Un i8 va de −2⁷ = −128 a 2⁷ − 1 = 127.",
            },
            Aufgabe::Quiz {
                id: "3.7",
                frage: "¿Qué ocurre con este código?",
                code: Some("let a: i32 = 5;\nlet b: f64 = 2.5;\nlet c = a + b;"),
                optionen: &["c vale 7.5", "c vale 7", "Error de compilación: no se pueden sumar i32 y f64", "c vale \"52.5\""],
                richtig: 2,
                erklaerung: "Rust nunca convierte tipos en silencio. Tienes que escribir explícitamente `a as f64 + b`.",
            },
            Aufgabe::Code {
                id: "3.8",
                aufgabe: "Calcula la media de las cinco notas como número decimal e imprímela con **un** decimal: `Media: 6.6`",
                vorlage: r#"fn main() {
    let suma = 7 + 9 + 4 + 8 + 5;   // i32
    let cantidad = 5;
    // Convierte con `as f64` e imprime con {:.1}:

}"#,
                erwartet: "Media: 6.6",
                hinweis: "`let media = suma as f64 / cantidad as f64;` y luego `println!(\"Media: {:.1}\", media);`",
                loesung: r#"fn main() {
    let suma = 7 + 9 + 4 + 8 + 5;
    let cantidad = 5;
    let media = suma as f64 / cantidad as f64;
    println!("Media: {:.1}", media);
}"#,
            },
        ],
    },
    Lektion {
        titel: "Funciones",
        kurz: "Parámetros, valores de retorno, expresiones e instrucciones.",
        kapitel: "Primeros pasos",
        bloecke: &[
            Block::Text("Las funciones agrupan código para poder reutilizarlo. Se definen con `fn`. **Los parámetros siempre necesitan un tipo** y el tipo de retorno va después de la flecha `->`."),
            Block::Ausfuehrbar(r#"fn cuadrado(x: i32) -> i32 {
    x * x   // sin punto y coma → este es el valor de retorno
}

fn saludar(nombre: &str) {
    println!("¡Hola, {nombre}!");
}

fn main() {
    saludar("Ferris");
    let q = cuadrado(7);
    println!("7² = {q}");
}"#),
            Block::Ueberschrift("Expresiones e instrucciones"),
            Block::Text("Es un concepto importante en Rust: una **instrucción** (statement) hace algo, pero no produce valor; termina con `;`. Una **expresión** produce un valor. La última línea de una función sin `;` es su valor de retorno. Incluso un bloque `{ }` es una expresión:"),
            Block::Ausfuehrbar(r#"fn main() {
    let y = {
        let x = 3;
        x + 1       // ← valor del bloque
    };
    println!("y = {y}");
}"#),
            Block::Achtung("Un punto y coma de más es un error típico de principiante: `fn f() -> i32 { 5; }` no compila, porque `5;` es una instrucción y la función devuelve entonces `()` (nada). El compilador muestra `E0308: mismatched types`."),
            Block::Text("Sigue cómo entran los valores en las funciones y vuelven a salir:"),
            Block::Visual(Visual::Trace {
                code: r#"fn doble(x: i32) -> i32 {
    x * 2
}
fn main() {
    let a = 3;
    let b = doble(a);
    let c = doble(b) + 1;
    println!("{c}");
}"#,
                spalten: &["a", "b", "c", "x (en doble)"],
                schritte: &[
                    ts(5, &["3", "–", "–", "–"], "`a` recibe el valor 3."),
                    ts(2, &["3", "–", "–", "3"], "Se llama a `doble(a)`: el parámetro `x` recibe una copia de `a`, es decir, 3. La función calcula 3 × 2."),
                    ts(6, &["3", "6", "–", "–"], "El valor de retorno 6 acaba en `b`. `x` ya no existe."),
                    ts(2, &["3", "6", "–", "6"], "Segunda llamada: esta vez `x` = 6, resultado 12."),
                    ts(7, &["3", "6", "13", "–"], "12 + 1 = 13 se guarda en `c`."),
                    ts(8, &["3", "6", "13", "–"], "Salida: **13**"),
                ],
            }),
        ],
        aufgaben: &[
            Aufgabe::Quiz {
                id: "4.1",
                frage: "¿Por qué no compila esta función?",
                code: Some("fn cinco() -> i32 {\n    5;\n}"),
                optionen: &["Porque 5 no es un i32", "Porque `5;` es una instrucción y no devuelve nada", "Porque la función necesita parámetros", "Compila sin problemas"],
                richtig: 1,
                erklaerung: "El punto y coma convierte la expresión `5` en una instrucción. Sin `;`, `5` sería el valor de retorno.",
            },
            Aufgabe::Rechnung {
                id: "4.2",
                frage: "¿Qué devuelve `f(f(3))`?",
                code: Some("fn f(x: i32) -> i32 {\n    x * 2 + 1\n}"),
                antworten: &["15"],
                hinweis: "Calcula de dentro hacia fuera: primero `f(3)` y luego `f` del resultado.",
                erklaerung: "f(3) = 3 × 2 + 1 = 7, f(7) = 7 × 2 + 1 = 15.",
            },
            Aufgabe::Code {
                id: "4.3",
                aufgabe: "Completa la función `area` para que devuelva ancho × alto. Salida esperada: `Área: 24`",
                vorlage: r#"fn area(ancho: u32, alto: u32) -> u32 {
    // TODO: calcula el área
    0
}

fn main() {
    println!("Área: {}", area(4, 6));
}"#,
                erwartet: "Área: 24",
                hinweis: "Sustituye el `0` por `ancho * alto`, ¡sin punto y coma!",
                loesung: r#"fn area(ancho: u32, alto: u32) -> u32 {
    ancho * alto
}

fn main() {
    println!("Área: {}", area(4, 6));
}"#,
            },
            Aufgabe::Code {
                id: "4.4",
                aufgabe: "Escribe la función `a_fahrenheit`. Fórmula: °F = °C × 9 / 5 + 32. Asegúrate de calcular con decimales (`9.0`, `5.0`, `32.0`).",
                vorlage: r#"fn a_fahrenheit(celsius: f64) -> f64 {
    // TODO
    0.0
}

fn main() {
    for c in [0.0, 37.0, 100.0] {
        println!("{c}°C = {:.1}°F", a_fahrenheit(c));
    }
}"#,
                erwartet: "0°C = 32.0°F\n37°C = 98.6°F\n100°C = 212.0°F",
                hinweis: "La función debería devolver `celsius * 9.0 / 5.0 + 32.0`.",
                loesung: r#"fn a_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn main() {
    for c in [0.0, 37.0, 100.0] {
        println!("{c}°C = {:.1}°F", a_fahrenheit(c));
    }
}"#,
            },
        ],
    },
    // ─────────────────────────────── Capítulo 2 ───────────────────────────────
    Lektion {
        titel: "Decisiones con if",
        kurz: "Condiciones, comparaciones e if como expresión.",
        kapitel: "Control de flujo",
        bloecke: &[
            Block::Text("Con `if` tu programa decide qué código se ejecuta. En Rust la condición debe ser **siempre un `bool`**: a diferencia de otros lenguajes, un número no cuenta automáticamente como verdadero o falso."),
            Block::Ausfuehrbar(r#"fn main() {
    let temperatura = 23;

    if temperatura > 30 {
        println!("¡Calor! 🥵");
    } else if temperatura >= 15 {
        println!("Agradable. 😊");
    } else {
        println!("¡Frío! 🥶");
    }
}"#),
            Block::Text("**Comparaciones:** `==` igual, `!=` distinto, `<`, `>`, `<=`, `>=`. **Lógica:** `&&` (y), `||` (o), `!` (no)."),
            Block::Ueberschrift("if es una expresión"),
            Block::Text("Como `if` produce un valor, puedes usarlo directamente en un `let`. Las dos ramas deben tener entonces el mismo tipo. Por eso Rust no necesita un operador `? :` como otros lenguajes."),
            Block::Ausfuehrbar(r#"fn main() {
    let numero = 7;
    let tipo = if numero % 2 == 0 { "par" } else { "impar" };
    println!("{numero} es {tipo}");

    let edad = 17;
    let puede_votar = edad >= 18 && edad < 150;
    println!("Puede votar: {puede_votar}");
}"#),
        ],
        aufgaben: &[
            Aufgabe::Quiz {
                id: "5.1",
                frage: "¿Compila este código?",
                code: Some("let n = 3;\nif n {\n    println!(\"verdadero\");\n}"),
                optionen: &["Sí, 3 cuenta como verdadero", "No, la condición debe ser un bool", "Sí, pero no se imprime nada", "No, falta un else"],
                richtig: 1,
                erklaerung: "Rust espera un `bool`. Tendrías que escribir, por ejemplo, `if n != 0`.",
            },
            Aufgabe::Rechnung {
                id: "5.2",
                frage: "¿Qué valor tiene `b`?",
                code: Some("let a = 7;\nlet b = if a % 2 == 0 { a / 2 } else { 3 * a + 1 };"),
                antworten: &["22"],
                hinweis: "¿Es 7 par? Si no, se ejecuta la rama else.",
                erklaerung: "7 es impar, así que 3 × 7 + 1 = 22. (Por cierto, ¡es un paso de la famosa sucesión de Collatz!)",
            },
            Aufgabe::Rechnung {
                id: "5.3",
                frage: "¿Qué da esta expresión? (Responde `true` o `false`)",
                code: Some("(5 > 3) && !(2 == 2) || (10 % 3 == 1)"),
                antworten: &["true"],
                hinweis: "`&&` tiene prioridad sobre `||`. Evalúa primero cada paréntesis.",
                erklaerung: "(true && false) || true → false || true → **true**. 10 % 3 es 1.",
            },
            Aufgabe::Code {
                id: "5.4",
                aufgabe: "Escribe la función `clima`: por debajo de 10 grados → `frío`, de 10 a 25 incluido → `agradable`, por encima → `calor`.",
                vorlage: r#"fn clima(grados: i32) -> &'static str {
    // TODO: if / else if / else
    "?"
}

fn main() {
    println!("5: {}", clima(5));
    println!("18: {}", clima(18));
    println!("25: {}", clima(25));
    println!("30: {}", clima(30));
}"#,
                erwartet: "5: frío\n18: agradable\n25: agradable\n30: calor",
                hinweis: "`if grados < 10 { \"frío\" } else if grados <= 25 { \"agradable\" } else { \"calor\" }`, sin punto y coma, así es el valor de retorno.",
                loesung: r#"fn clima(grados: i32) -> &'static str {
    if grados < 10 {
        "frío"
    } else if grados <= 25 {
        "agradable"
    } else {
        "calor"
    }
}

fn main() {
    println!("5: {}", clima(5));
    println!("18: {}", clima(18));
    println!("25: {}", clima(25));
    println!("30: {}", clima(30));
}"#,
            },
        ],
    },
    Lektion {
        titel: "Bucles",
        kurz: "loop, while, for y rangos.",
        kapitel: "Control de flujo",
        bloecke: &[
            Block::Text("Rust tiene tres tipos de bucle: `loop` (sin fin, hasta un `break`), `while` (mientras se cumpla una condición) y `for` (sobre una secuencia de valores). El que más usarás es `for`."),
            Block::Ausfuehrbar(r#"fn main() {
    // for sobre un rango: 1, 2, 3, 4, 5
    for i in 1..=5 {
        print!("{i} ");
    }
    println!();

    // while: cuenta atrás
    let mut n = 3;
    while n > 0 {
        println!("{n}…");
        n -= 1;
    }
    println!("¡Despegue! 🚀");

    // loop puede devolver un valor con break
    let mut x = 1;
    let resultado = loop {
        x *= 2;
        if x > 100 {
            break x;
        }
    };
    println!("Primera potencia de dos mayor que 100: {resultado}");
}"#),
            Block::Tipp("`0..5` significa 0, 1, 2, 3, 4 (el 5 **no** está incluido). `0..=5` sí incluye el 5. Con `(0..5).rev()` vas hacia atrás. `continue` salta a la siguiente vuelta y `break` termina el bucle."),
            Block::Ueberschrift("Un bucle paso a paso"),
            Block::Text("Pulsa **Reproducir** y observa cómo cambian `i` y `suma` en cada vuelta:"),
            Block::Visual(Visual::Trace {
                code: r#"fn main() {
    let mut suma = 0;
    for i in 1..=4 {
        suma += i;
    }
    println!("Suma: {suma}");
}"#,
                spalten: &["i", "suma"],
                schritte: &[
                    ts(2, &["–", "0"], "`suma` empieza en 0. Tiene que ser `mut` porque cambia."),
                    ts(3, &["1", "0"], "Primera vuelta: `i` vale 1."),
                    ts(4, &["1", "1"], "suma = 0 + 1 = **1**"),
                    ts(3, &["2", "1"], "Segunda vuelta: `i` vale 2."),
                    ts(4, &["2", "3"], "suma = 1 + 2 = **3**"),
                    ts(3, &["3", "3"], "Tercera vuelta: `i` vale 3."),
                    ts(4, &["3", "6"], "suma = 3 + 3 = **6**"),
                    ts(3, &["4", "6"], "Cuarta vuelta: `i` vale 4 (por `..=` el 4 está incluido)."),
                    ts(4, &["4", "10"], "suma = 6 + 4 = **10**"),
                    ts(6, &["–", "10"], "El bucle ha terminado. Salida: `Suma: 10`"),
                ],
            }),
        ],
        aufgaben: &[
            Aufgabe::Rechnung {
                id: "6.1",
                frage: "¿Cuántas veces se ejecuta el cuerpo del bucle?",
                code: Some("for i in 3..=7 {\n    println!(\"{i}\");\n}"),
                antworten: &["5"],
                hinweis: "Cuenta los valores: 3, 4, … hasta 7 incluido.",
                erklaerung: "3, 4, 5, 6, 7: 5 vueltas. En general, `a..=b` tiene b − a + 1 valores.",
            },
            Aufgabe::Rechnung {
                id: "6.2",
                frage: "¿Qué imprime este programa? Se dice que el joven Carl Friedrich Gauss lo resolvió en segundos siendo un escolar.",
                code: Some("let mut suma = 0;\nfor i in 1..=100 {\n    suma += i;\n}\nprintln!(\"{suma}\");"),
                antworten: &["5050", "5 050", "5.050"],
                hinweis: "Forma parejas: 1 + 100, 2 + 99, 3 + 98… Cada pareja suma 101. ¿Cuántas parejas hay? Fórmula: n × (n + 1) / 2",
                erklaerung: "50 parejas × 101 = 5050. O con la fórmula: 100 × 101 / 2 = 5050.",
            },
            Aufgabe::Rechnung {
                id: "6.3",
                frage: "¿Qué valor tiene `pasos` al final?",
                code: Some("let mut n = 1;\nlet mut pasos = 0;\nwhile n < 100 {\n    n *= 3;\n    pasos += 1;\n}"),
                antworten: &["5"],
                hinweis: "Apunta los valores de `n`: 1, 3, 9, … ¿Cuándo deja n de ser menor que 100?",
                erklaerung: "n: 1 → 3 → 9 → 27 → 81 → 243. Después de 5 pasos n = 243 ≥ 100.",
            },
            Aufgabe::Code {
                id: "6.4",
                aufgabe: "El clásico **FizzBuzz**: imprime los números del 1 al 15. Para múltiplos de 3 imprime `Fizz`, para múltiplos de 5 `Buzz` y para múltiplos de ambos `FizzBuzz`.",
                vorlage: r#"fn main() {
    for i in 1..=15 {
        // Consejo: i % 3 == 0 significa "divisible por 3"
        println!("{i}");
    }
}"#,
                erwartet: "1\n2\nFizz\n4\nBuzz\nFizz\n7\n8\nFizz\nBuzz\n11\nFizz\n13\n14\nFizzBuzz",
                hinweis: "Comprueba **primero** `i % 15 == 0` (FizzBuzz), luego 3, luego 5 y si no, el número.",
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
                aufgabe: "Imprime la tabla del 7, pero solo hasta el primer resultado mayor que 40 (incluido). Usa `loop` y `break`.",
                vorlage: r#"fn main() {
    let mut i = 1;
    loop {
        let resultado = 7 * i;
        println!("7 x {i} = {resultado}");
        // TODO: termina el bucle cuando resultado > 40
        i += 1;
        if i > 20 { break; } // freno de emergencia
    }
}"#,
                erwartet: "7 x 1 = 7\n7 x 2 = 14\n7 x 3 = 21\n7 x 4 = 28\n7 x 5 = 35\n7 x 6 = 42",
                hinweis: "Después de imprimir, añade `if resultado > 40 { break; }`.",
                loesung: r#"fn main() {
    let mut i = 1;
    loop {
        let resultado = 7 * i;
        println!("7 x {i} = {resultado}");
        if resultado > 40 {
            break;
        }
        i += 1;
    }
}"#,
            },
        ],
    },
    // ─────────────────────────────── Capítulo 3 ───────────────────────────────
    Lektion {
        titel: "Ownership (propiedad)",
        kurz: "Pila, montón, move y clone: el superpoder de Rust.",
        kapitel: "Ownership",
        bloecke: &[
            Block::Text("Llega el concepto que hace único a Rust: **ownership** (propiedad). Con él, Rust gestiona la memoria de forma automática y segura, sin un recolector de basura limpiando en segundo plano."),
            Block::Ueberschrift("Pila y montón"),
            Block::Text("Tu programa tiene dos zonas de memoria. La **pila** (stack) es rápida y ordenada como una pila de platos: los valores de tamaño fijo y conocido (`i32`, `bool`, …) se ponen encima y se quitan en orden inverso. El **montón** (heap) es un gran almacén para datos cuyo tamaño puede cambiar, por ejemplo un `String` que puede crecer. En la pila queda solo un **puntero** a los datos."),
            Block::Ueberschrift("Las tres reglas"),
            Block::Text("**1.** Cada valor tiene exactamente un dueño (owner). **2.** Solo hay un dueño a la vez. **3.** Cuando el dueño sale de su ámbito, el valor se libera (drop)."),
            Block::Visual(Visual::Speicher {
                code: r#"fn main() {
    let a = 5;
    let b = a;
    let s1 = String::from("hola");
    let s2 = s1;
    let s3 = s2.clone();
    println!("{a} {b} {s2} {s3}");
}"#,
                schritte: &[
                    SpeicherSchritt { zeile: 2, stack: &[sv("a", "5", N)], heap: &[], text: "`a` es un `i32` de tamaño fijo (4 bytes): vive directamente en la pila.", ausgabe: "" },
                    SpeicherSchritt { zeile: 3, stack: &[sv("a", "5", OK), sv("b", "5", N)], heap: &[], text: "Los números simples son `Copy`: `b` recibe una copia real. `a` sigue siendo válida porque copiar es barato.", ausgabe: "" },
                    SpeicherSchritt { zeile: 4, stack: &[sv("a", "5", OK), sv("b", "5", OK), sp("s1", "ptr, len 4, cap 4", 0, N)], heap: &[hb("h o l a", N)], text: "Un `String` guarda sus letras en el **montón**. En la pila solo están el puntero, la longitud y la capacidad.", ausgabe: "" },
                    SpeicherSchritt { zeile: 5, stack: &[sv("a", "5", OK), sv("b", "5", OK), sp("s1", "ptr, len 4, cap 4", 0, V), sp("s2", "ptr, len 4, cap 4", 0, N)], heap: &[hb("h o l a", OK)], text: "**¡Move!** Solo se copian los datos de la pila y `s2` pasa a ser el nuevo dueño. `s1` queda **inválida** a partir de ahora; si no, al final dos variables liberarían la misma memoria (un peligroso «double free»).", ausgabe: "" },
                    SpeicherSchritt { zeile: 6, stack: &[sv("a", "5", OK), sv("b", "5", OK), sp("s1", "ptr, len 4, cap 4", 0, V), sp("s2", "ptr, len 4, cap 4", 0, OK), sp("s3", "ptr, len 4, cap 4", 1, N)], heap: &[hb("h o l a", OK), hb("h o l a", N)], text: "`clone()` hace una **copia profunda**: también se copian los datos del montón. `s3` tiene su propia memoria. Eso cuesta tiempo, por eso hay que escribirlo explícitamente.", ausgabe: "" },
                    SpeicherSchritt { zeile: 7, stack: &[sv("a", "5", OK), sv("b", "5", OK), sp("s1", "ptr, len 4, cap 4", 0, V), sp("s2", "ptr, len 4, cap 4", 0, OK), sp("s3", "ptr, len 4, cap 4", 1, OK)], heap: &[hb("h o l a", OK), hb("h o l a", OK)], text: "Usamos `a`, `b`, `s2` y `s3`. `s1` ya no se puede usar: el compilador daría `E0382: borrow of moved value`.", ausgabe: "5 5 hola hola" },
                    SpeicherSchritt { zeile: 8, stack: &[sv("a", "5", F), sv("b", "5", F), sp("s1", "ptr, len 4, cap 4", 0, V), sp("s2", "ptr, len 4, cap 4", 0, F), sp("s3", "ptr, len 4, cap 4", 1, F)], heap: &[hb("h o l a", F), hb("h o l a", F)], text: "Fin del ámbito: Rust llama automáticamente a `drop`. Cada trozo de memoria del montón lo libera **una sola vez** su dueño. Sin fugas ni doble liberación.", ausgabe: "" },
                ],
            }),
            Block::Ueberschrift("Ownership y funciones"),
            Block::Text("Si pasas un `String` a una función, se **mueve** allí. Después ya no puedes usarlo en quien llamó, salvo que la función lo devuelva:"),
            Block::Ausfuehrbar(r#"fn tomar(s: String) -> String {
    println!("Tengo: {s}");
    s // devolver la propiedad
}

fn main() {
    let texto = String::from("Ferris");
    let texto = tomar(texto); // ida y vuelta
    println!("De vuelta: {texto}");
}"#),
            Block::Tipp("Pasar valores de un lado a otro todo el rato es pesado. En la próxima lección conocerás las **referencias**: con ellas puedes tomar prestado un valor sin ser su dueño."),
        ],
        aufgaben: &[
            Aufgabe::Quiz {
                id: "7.1",
                frage: "¿Qué ocurre después de `let s2 = s1;` si `s1` es un `String`?",
                code: None,
                optionen: &["s1 y s2 apuntan, ambos válidos, al mismo texto", "s1 queda inválida y s2 es el nuevo dueño", "El texto se copió en el montón", "s2 está vacía"],
                richtig: 1,
                erklaerung: "Es un **move**. Solo un dueño a la vez: `s1` ya no se puede usar.",
            },
            Aufgabe::Quiz {
                id: "7.2",
                frage: "¿Cuál de estos tipos es `Copy` (se copia en vez de moverse al asignarlo)?",
                code: None,
                optionen: &["`String`", "`Vec<i32>`", "`i32`", "`Box<i32>`"],
                richtig: 2,
                erklaerung: "Los tipos simples de tamaño fijo en la pila (`i32`, `f64`, `bool`, `char`, …) son `Copy`. Los que tienen datos en el montón, no.",
            },
            Aufgabe::Quiz {
                id: "7.3",
                frage: "¿En qué línea da error el compilador?",
                code: Some("let s = String::from(\"Ferris\");  // línea 1\nlet t = s;                       // línea 2\nprintln!(\"{t}\");                 // línea 3\nprintln!(\"{s}\");                 // línea 4"),
                optionen: &["Línea 2", "Línea 3", "Línea 4", "En ninguna"],
                richtig: 2,
                erklaerung: "En la línea 2, `s` se mueve a `t`. La línea 4 intenta usar `s` → `E0382: borrow of moved value`.",
            },
            Aufgabe::Code {
                id: "7.4",
                aufgabe: "El programa no compila porque `nombre` se movió. Arréglalo con `.clone()` para que se imprima `Ferris y Ferris`.",
                vorlage: r#"fn main() {
    let nombre = String::from("Ferris");
    let copia = nombre;
    println!("{nombre} y {copia}");
}"#,
                erwartet: "Ferris y Ferris",
                hinweis: "Escribe `let copia = nombre.clone();`: así `copia` tiene su propia copia.",
                loesung: r#"fn main() {
    let nombre = String::from("Ferris");
    let copia = nombre.clone();
    println!("{nombre} y {copia}");
}"#,
            },
            Aufgabe::Code {
                id: "7.5",
                aufgabe: "`saludar` se queda con la propiedad del string, por eso la segunda llamada no funciona. Arregla `main` (la función se queda igual) para que aparezca dos veces `¡Hola, Ada!`.",
                vorlage: r#"fn saludar(nombre: String) {
    println!("¡Hola, {nombre}!");
}

fn main() {
    let n = String::from("Ada");
    saludar(n);
    saludar(n);
}"#,
                erwartet: "¡Hola, Ada!\n¡Hola, Ada!",
                hinweis: "En la primera llamada pasa una copia: `saludar(n.clone());`",
                loesung: r#"fn saludar(nombre: String) {
    println!("¡Hola, {nombre}!");
}

fn main() {
    let n = String::from("Ada");
    saludar(n.clone());
    saludar(n);
}"#,
            },
        ],
    },
    Lektion {
        titel: "Referencias y préstamos",
        kurz: "Tomar valores prestados con & y &mut.",
        kapitel: "Ownership",
        bloecke: &[
            Block::Text("Una **referencia** `&x` es como una nota con la dirección de un valor. Puedes usar el valor sin ser su dueño: lo **tomas prestado** (borrowing). El dueño conserva su valor."),
            Block::Ausfuehrbar(r#"fn longitud(s: &String) -> usize {
    s.len() // solo leer
}

fn anadir(s: &mut String) {
    s.push_str(", mundo"); // cambiar
}

fn main() {
    let mut texto = String::from("Hola");
    let n = longitud(&texto);     // prestar para leer
    anadir(&mut texto);           // prestar para cambiar
    println!("{texto} (antes {n} caracteres)");
}"#),
            Block::Ueberschrift("Las reglas del préstamo"),
            Block::Text("En todo momento se cumple **o bien**: cualquier número de referencias compartidas `&T`, **o bien** exactamente una referencia mutable `&mut T`. Muchos lectores o un escritor, nunca ambos. Además, las referencias siempre deben ser válidas: no puede haber una referencia a memoria ya liberada."),
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
                    SpeicherSchritt { zeile: 2, stack: &[sp("s", "ptr, len 2, cap 2", 0, N)], heap: &[hb("H i", N)], text: "`s` es dueña de un String en el montón.", ausgabe: "" },
                    SpeicherSchritt { zeile: 3, stack: &[sp("s", "ptr, len 2, cap 2", 0, OK), sv("r1", "&s  → apunta a s", G)], heap: &[hb("H i", OK)], text: "`r1` toma prestado `s` para **leer**. `s` sigue siendo la dueña.", ausgabe: "" },
                    SpeicherSchritt { zeile: 4, stack: &[sp("s", "ptr, len 2, cap 2", 0, OK), sv("r1", "&s  → apunta a s", G), sv("r2", "&s  → apunta a s", G)], heap: &[hb("H i", OK)], text: "Se permiten varias referencias compartidas a la vez: leer no molesta a nadie.", ausgabe: "" },
                    SpeicherSchritt { zeile: 5, stack: &[sp("s", "ptr, len 2, cap 2", 0, OK), sv("r1", "&s", F), sv("r2", "&s", F)], heap: &[hb("H i", OK)], text: "Tras su último uso, los préstamos de `r1` y `r2` terminan automáticamente (el compilador lo detecta).", ausgabe: "Hi Hi" },
                    SpeicherSchritt { zeile: 6, stack: &[sp("s", "ptr, len 2, cap 2", 0, OK), sv("r1", "&s", F), sv("r2", "&s", F), sv("r3", "&mut s  → apunta a s", N)], heap: &[hb("H i", OK)], text: "Ya no quedan lectores, así que `r3` puede tomar prestado de forma **exclusiva** y mutable.", ausgabe: "" },
                    SpeicherSchritt { zeile: 7, stack: &[sp("s", "ptr, len 3, cap 4", 0, OK), sv("r1", "&s", F), sv("r2", "&s", F), sv("r3", "&mut s  → apunta a s", G)], heap: &[hb("H i !", N)], text: "El String se cambia a través de `r3`. Como nadie más lee a la vez, es seguro.", ausgabe: "" },
                    SpeicherSchritt { zeile: 8, stack: &[sp("s", "ptr, len 3, cap 4", 0, OK), sv("r1", "&s", F), sv("r2", "&s", F), sv("r3", "&mut s", F)], heap: &[hb("H i !", OK)], text: "El préstamo ha terminado: `s` se puede usar directamente otra vez y contiene el cambio.", ausgabe: "Hi!" },
                ],
            }),
            Block::Text("Este código rompe la regla y por eso no compila:"),
            Block::Code(r#"let mut s = String::from("Hi");
let lector = &s;
let escritor = &mut s;    // ❌ E0502: cannot borrow `s` as mutable
println!("{lector}");     //    because it is also borrowed as immutable"#),
            Block::Tipp("Con `*` accedes al valor que hay detrás de una referencia (desreferenciar): `*numero += 1;` aumenta el número al que apunta `numero: &mut i32`. En llamadas a métodos como `s.push('!')`, Rust lo hace automáticamente."),
        ],
        aufgaben: &[
            Aufgabe::Quiz {
                id: "8.1",
                frage: "¿Cuántas referencias mutables (`&mut`) al mismo valor pueden existir a la vez?",
                code: None,
                optionen: &["Las que quieras", "Exactamente una", "Dos", "Ninguna: &mut no existe"],
                richtig: 1,
                erklaerung: "Exactamente una y, mientras existe, ninguna compartida. Así evita Rust las carreras de datos.",
            },
            Aufgabe::Quiz {
                id: "8.2",
                frage: "¿Qué significa `fn f(x: &String)`?",
                code: None,
                optionen: &["f se queda con la propiedad del string", "f solo toma prestado el string para leerlo", "f puede cambiar el string", "f recibe una copia"],
                richtig: 1,
                erklaerung: "`&String` es una referencia de solo lectura. Para cambiarlo haría falta `&mut String`.",
            },
            Aufgabe::Code {
                id: "8.3",
                aufgabe: "Completa `duplicar` para que se duplique el número que hay detrás de la referencia. Salida esperada: `42`",
                vorlage: r#"fn duplicar(numero: &mut i32) {
    // TODO: usa * para desreferenciar
}

fn main() {
    let mut x = 21;
    duplicar(&mut x);
    println!("{x}");
}"#,
                erwartet: "42",
                hinweis: "`*numero *= 2;`: el primer `*` desreferencia y el segundo es la multiplicación.",
                loesung: r#"fn duplicar(numero: &mut i32) {
    *numero *= 2;
}

fn main() {
    let mut x = 21;
    duplicar(&mut x);
    println!("{x}");
}"#,
            },
            Aufgabe::Code {
                id: "8.4",
                aufgabe: "Cambia `gritar` para que tome prestado el string con `&mut` en lugar de quedarse con él. Así `main` debe imprimir después `Hola!!!`.",
                vorlage: r#"fn gritar(s: String) {
    s.push_str("!!!");
}

fn main() {
    let saludo = String::from("Hola");
    gritar(saludo);
    println!("{saludo}");
}"#,
                erwartet: "Hola!!!",
                hinweis: "Tres sitios: el parámetro `s: &mut String`, la variable `let mut saludo` y la llamada `gritar(&mut saludo)`.",
                loesung: r#"fn gritar(s: &mut String) {
    s.push_str("!!!");
}

fn main() {
    let mut saludo = String::from("Hola");
    gritar(&mut saludo);
    println!("{saludo}");
}"#,
            },
        ],
    },
    Lektion {
        titel: "Strings y slices",
        kurz: "String frente a &str, UTF-8 y trozos de texto.",
        kapitel: "Ownership",
        bloecke: &[
            Block::Text("Rust tiene dos tipos principales para texto: `String` posee su texto en el montón y puede crecer. `&str` (un **string slice**) es una vista prestada de un texto: por ejemplo, de un trozo de un `String` o de un texto fijo del programa como `\"Hola\"`."),
            Block::Ausfuehrbar(r#"fn main() {
    let fijo: &str = "Hola";                  // fijo en el programa
    let mut propio = String::from(fijo);      // texto propio y mutable
    propio.push_str(", mundo");
    propio.push('!');

    let trozo: &str = &propio[0..4];          // slice: bytes 0 a 3
    println!("{propio} | Trozo: {trozo}");
    println!("Mayúsculas: {}", propio.to_uppercase());
    println!("Contiene 'mundo': {}", propio.contains("mundo"));

    let frase = format!("{} tiene {} bytes", propio, propio.len());
    println!("{frase}");

    for palabra in "Rust es genial".split_whitespace() {
        println!("- {palabra}");
    }
}"#),
            Block::Ueberschrift("UTF-8: bytes no son caracteres"),
            Block::Text("Los strings de Rust siempre son **UTF-8** válido. Letras como `a` ocupan 1 byte, letras con tilde o la `ñ` ocupan 2 bytes y los emojis como 🦀 incluso 4. `len()` cuenta **bytes**, ¡no caracteres! Para caracteres se usa `chars()`."),
            Block::Ausfuehrbar(r#"fn main() {
    let palabra = "niño";
    println!("Bytes:      {}", palabra.len());
    println!("Caracteres: {}", palabra.chars().count());
    for (i, c) in palabra.char_indices() {
        println!("Byte {i}: '{c}' ({} byte)", c.len_utf8());
    }
}"#),
            Block::Achtung("`s[0]` **no** funciona con strings, porque no estaría claro si te refieres a un byte o a un carácter. Y un slice como `&\"niño\"[0..3]` falla, porque el byte 3 está en medio de la `ñ`. Usa `chars()` o métodos como `find`, `split` y `starts_with`."),
            Block::Tipp("Escribe las funciones que solo leen texto con `&str` como parámetro: `fn f(s: &str)`. Así puedes pasarles tanto `&String` como `\"texto\"`."),
        ],
        aufgaben: &[
            Aufgabe::Rechnung {
                id: "9.1",
                frage: "¿Qué devuelve `\"canción\".len()`?",
                code: None,
                antworten: &["8"],
                hinweis: "`len()` cuenta bytes. c, a, n, c, i y n ocupan 1 byte cada una; la ó ocupa 2.",
                erklaerung: "6 × 1 + 2 = 8 bytes, pero solo 7 caracteres.",
            },
            Aufgabe::Rechnung {
                id: "9.2",
                frage: "¿Qué devuelve `\"Ferris 🦀\".chars().count()`?",
                code: None,
                antworten: &["8"],
                hinweis: "Cuenta caracteres, no bytes: letras, el espacio y el emoji.",
                erklaerung: "F-e-r-r-i-s (6) + espacio (1) + 🦀 (1) = 8 caracteres. En bytes serían 6 + 1 + 4 = 11.",
            },
            Aufgabe::Quiz {
                id: "9.3",
                frage: "¿Qué es `trozo`?",
                code: Some("let s = String::from(\"Ferris\");\nlet trozo = &s[1..4];"),
                optionen: &["`\"Fer\"`", "`\"err\"`", "`\"erri\"`", "`\"Ferr\"`"],
                richtig: 1,
                erklaerung: "Del índice 1 hasta el 4 sin incluirlo: bytes 1, 2, 3 → `e`, `r`, `r`.",
            },
            Aufgabe::Code {
                id: "9.4",
                aufgabe: "Escribe `primera_palabra`, que devuelve la primera palabra de un texto. Salida esperada: `Hola` y `Rust`",
                vorlage: r#"fn primera_palabra(s: &str) -> &str {
    // TODO: consejo: s.split_whitespace().next()
    s
}

fn main() {
    println!("{}", primera_palabra("Hola mundo bonito"));
    println!("{}", primera_palabra("Rust"));
}"#,
                erwartet: "Hola\nRust",
                hinweis: "`s.split_whitespace().next()` da un `Option<&str>`. Con `.unwrap_or(\"\")` obtienes el `&str`.",
                loesung: r#"fn primera_palabra(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

fn main() {
    println!("{}", primera_palabra("Hola mundo bonito"));
    println!("{}", primera_palabra("Rust"));
}"#,
            },
            Aufgabe::Code {
                id: "9.5",
                aufgabe: "Construye con un bucle el texto `1-2-3-4-5` e imprímelo.",
                vorlage: r#"fn main() {
    let mut texto = String::new();
    for i in 1..=5 {
        // TODO: añade el número y un guion, salvo después del último número
    }
    println!("{texto}");
}"#,
                erwartet: "1-2-3-4-5",
                hinweis: "`texto.push_str(&i.to_string());` añade el número. Con `if i < 5 { texto.push('-'); }`, el guion.",
                loesung: r#"fn main() {
    let mut texto = String::new();
    for i in 1..=5 {
        texto.push_str(&i.to_string());
        if i < 5 {
            texto.push('-');
        }
    }
    println!("{texto}");
}"#,
            },
        ],
    },
    // ─────────────────────────────── Capítulo 4 ───────────────────────────────
    Lektion {
        titel: "Structs y métodos",
        kurz: "Tipos de datos propios con campos y comportamiento.",
        kapitel: "Tipos propios",
        bloecke: &[
            Block::Text("Con un `struct` agrupas datos relacionados en un tipo propio. Cada campo tiene un nombre y un tipo. En un bloque `impl` defines **métodos**: funciones que pertenecen a tu tipo."),
            Block::Ausfuehrbar(r#"#[derive(Debug)] // permite imprimir con {:?}
struct Rectangulo {
    ancho: u32,
    alto: u32,
}

impl Rectangulo {
    // "Función asociada" sin self: como un constructor
    fn nuevo(ancho: u32, alto: u32) -> Self {
        Self { ancho, alto }
    }

    // Método: solo lee (&self)
    fn area(&self) -> u32 {
        self.ancho * self.alto
    }

    // Método: cambia (&mut self)
    fn duplicar(&mut self) {
        self.ancho *= 2;
        self.alto *= 2;
    }
}

fn main() {
    let mut r = Rectangulo::nuevo(3, 4);
    println!("{r:?} tiene área {}", r.area());
    r.duplicar();
    println!("Tras duplicar: {}×{} = {}", r.ancho, r.alto, r.area());
}"#),
            Block::Text("**Recuerda:** `&self` = el método solo lee. `&mut self` = el método cambia el objeto. `self` = el método consume el objeto (¡ownership!). Sin `self` es una función asociada que se llama con `Tipo::nombre()`."),
            Block::Tipp("Con `#[derive(Debug)]` Rust genera automáticamente el código para imprimir tu struct. `{:?}` lo muestra compacto y `{:#?}` bien formateado en varias líneas."),
        ],
        aufgaben: &[
            Aufgabe::Quiz {
                id: "10.1",
                frage: "¿Cómo llamas a la función asociada `nuevo` de `Rectangulo`?",
                code: None,
                optionen: &["`Rectangulo.nuevo(3, 4)`", "`Rectangulo::nuevo(3, 4)`", "`nuevo Rectangulo(3, 4)`", "`Rectangulo->nuevo(3, 4)`"],
                richtig: 1,
                erklaerung: "Las funciones asociadas (sin `self`) se llaman sobre el tipo con `::`. Los métodos, sobre una instancia con `.`.",
            },
            Aufgabe::Quiz {
                id: "10.2",
                frage: "Un método debe cambiar un campo del struct. ¿Qué primer parámetro necesita?",
                code: None,
                optionen: &["`self`", "`&self`", "`&mut self`", "Ninguno"],
                richtig: 2,
                erklaerung: "Para cambiarlo, el método necesita una referencia mutable al objeto: `&mut self`.",
            },
            Aufgabe::Rechnung {
                id: "10.3",
                frage: "¿Qué valor tiene `c.saldo` al final?",
                code: Some("struct Cuenta { saldo: i64 }\n\nimpl Cuenta {\n    fn ingresar(&mut self, importe: i64) { self.saldo += importe; }\n    fn retirar(&mut self, importe: i64) {\n        if importe <= self.saldo { self.saldo -= importe; }\n    }\n}\n\nlet mut c = Cuenta { saldo: 100 };\nc.ingresar(50);\nc.retirar(200);\nc.retirar(30);\nc.ingresar(5);"),
                antworten: &["125"],
                hinweis: "Ojo con `retirar(200)`: debe cumplirse la condición `importe <= self.saldo`.",
                erklaerung: "100 + 50 = 150. Retirar 200 no es posible (no hay dinero suficiente). 150 − 30 = 120, + 5 = 125.",
            },
            Aufgabe::Code {
                id: "10.4",
                aufgabe: "Implementa el método `perimetro` (2 × ancho + 2 × alto). Salida esperada: `Área: 12, perímetro: 14`",
                vorlage: r#"struct Rectangulo {
    ancho: u32,
    alto: u32,
}

impl Rectangulo {
    fn area(&self) -> u32 {
        self.ancho * self.alto
    }

    fn perimetro(&self) -> u32 {
        // TODO
        0
    }
}

fn main() {
    let r = Rectangulo { ancho: 3, alto: 4 };
    println!("Área: {}, perímetro: {}", r.area(), r.perimetro());
}"#,
                erwartet: "Área: 12, perímetro: 14",
                hinweis: "Accede a los campos con `self.ancho` y `self.alto`.",
                loesung: r#"struct Rectangulo {
    ancho: u32,
    alto: u32,
}

impl Rectangulo {
    fn area(&self) -> u32 {
        self.ancho * self.alto
    }

    fn perimetro(&self) -> u32 {
        2 * self.ancho + 2 * self.alto
    }
}

fn main() {
    let r = Rectangulo { ancho: 3, alto: 4 };
    println!("Área: {}, perímetro: {}", r.area(), r.perimetro());
}"#,
            },
            Aufgabe::Code {
                id: "10.5",
                aufgabe: "Añade el método `incrementar`, que aumenta `valor` en 1. Fíjate en el parámetro `self` correcto. Salida esperada: `Contador: 3`",
                vorlage: r#"struct Contador {
    valor: u32,
}

impl Contador {
    // TODO: fn incrementar(...)
}

fn main() {
    let mut c = Contador { valor: 0 };
    c.incrementar();
    c.incrementar();
    c.incrementar();
    println!("Contador: {}", c.valor);
}"#,
                erwartet: "Contador: 3",
                hinweis: "`fn incrementar(&mut self) { self.valor += 1; }`",
                loesung: r#"struct Contador {
    valor: u32,
}

impl Contador {
    fn incrementar(&mut self) {
        self.valor += 1;
    }
}

fn main() {
    let mut c = Contador { valor: 0 };
    c.incrementar();
    c.incrementar();
    c.incrementar();
    println!("Contador: {}", c.valor);
}"#,
            },
        ],
    },
    Lektion {
        titel: "Enums y match",
        kurz: "Variantes, pattern matching y Option.",
        kapitel: "Tipos propios",
        bloecke: &[
            Block::Text("Un `enum` describe un valor que es **exactamente una** de varias variantes. Lo especial en Rust: cada variante puede llevar sus propios datos. Con `match` compruebas qué variante tienes, y el compilador se asegura de que **no olvides ningún caso**."),
            Block::Visual(Visual::Match),
            Block::Ueberschrift("Option: Rust no tiene null"),
            Block::Text("Muchos lenguajes tienen `null` para «sin valor», una enorme fuente de errores. Rust usa en su lugar el enum `Option<T>`: o `Some(valor)` o `None`. Tienes que tratar el caso `None` antes de llegar al valor."),
            Block::Ausfuehrbar(r#"fn dividir(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    match dividir(10, 2) {
        Some(x) => println!("Resultado: {x}"),
        None => println!("¡No se puede dividir entre cero!"),
    }

    // Forma corta si solo te interesa un caso:
    if let Some(x) = dividir(9, 3) {
        println!("if let: {x}");
    }

    // Valor por defecto si es None:
    let y = dividir(1, 0).unwrap_or(-1);
    println!("unwrap_or: {y}");
}"#),
            Block::Ueberschrift("Patrones en match"),
            Block::Code(r#"match numero {
    0 => println!("cero"),
    1 | 2 => println!("uno o dos"),             // varios valores
    3..=9 => println!("una cifra"),             // rango
    n if n < 0 => println!("negativo: {n}"),    // guarda con if
    _ => println!("otra cosa"),                 // todo lo demás
}"#),
        ],
        aufgaben: &[
            Aufgabe::Quiz {
                id: "11.1",
                frage: "¿Para qué sirve `Option<T>` en Rust?",
                code: None,
                optionen: &["Para un ajuste del programa", "Para un valor que puede estar (`Some`) o no (`None`)", "Para un error", "Para una función opcional"],
                richtig: 1,
                erklaerung: "Option sustituye a `null`. El compilador te obliga a tratar el caso `None`.",
            },
            Aufgabe::Quiz {
                id: "11.2",
                frage: "¿Qué ocurre si un `match` no cubre todas las variantes de un enum?",
                code: None,
                optionen: &["Se salta el caso que falta", "El programa falla cuando ocurre ese caso", "El compilador da un error", "Se toma automáticamente la primera rama"],
                richtig: 2,
                erklaerung: "`match` debe ser exhaustivo; si no, `E0004`. Con `_ =>` atrapas todos los casos restantes.",
            },
            Aufgabe::Rechnung {
                id: "11.3",
                frage: "¿Qué valor tiene `resultado`?",
                code: Some("let numero = 7;\nlet resultado = match numero {\n    1 => 10,\n    2 | 3 => 20,\n    4..=9 => 30,\n    _ => 0,\n};"),
                antworten: &["30"],
                hinweis: "¿Qué patrón encaja primero con 7?",
                erklaerung: "7 está en el rango `4..=9`, así que 30.",
            },
            Aufgabe::Code {
                id: "11.4",
                aufgabe: "Escribe la función `valor`, que devuelve el valor de cada moneda en céntimos (5, 10, 20, 50). Salida esperada: `Total: 85 céntimos`",
                vorlage: r#"enum Moneda {
    Cinco,
    Diez,
    Veinte,
    Cincuenta,
}

fn valor(m: &Moneda) -> u32 {
    // TODO: match m { ... }
    0
}

fn main() {
    let monedero = [Moneda::Veinte, Moneda::Cincuenta, Moneda::Diez, Moneda::Cinco];
    let mut total = 0;
    for m in &monedero {
        total += valor(m);
    }
    println!("Total: {total} céntimos");
}"#,
                erwartet: "Total: 85 céntimos",
                hinweis: "`match m { Moneda::Cinco => 5, Moneda::Diez => 10, ... }`",
                loesung: r#"enum Moneda {
    Cinco,
    Diez,
    Veinte,
    Cincuenta,
}

fn valor(m: &Moneda) -> u32 {
    match m {
        Moneda::Cinco => 5,
        Moneda::Diez => 10,
        Moneda::Veinte => 20,
        Moneda::Cincuenta => 50,
    }
}

fn main() {
    let monedero = [Moneda::Veinte, Moneda::Cincuenta, Moneda::Diez, Moneda::Cinco];
    let mut total = 0;
    for m in &monedero {
        total += valor(m);
    }
    println!("Total: {total} céntimos");
}"#,
            },
            Aufgabe::Code {
                id: "11.5",
                aufgabe: "Escribe `raiz_cuadrada`: para números negativos `None` y si no, `Some(x.sqrt())`. Trata los dos casos en `main` con `match`.",
                vorlage: r#"fn raiz_cuadrada(x: f64) -> Option<f64> {
    // TODO
    None
}

fn main() {
    for x in [16.0, -4.0] {
        match raiz_cuadrada(x) {
            Some(r) => println!("√{x} = {r}"),
            None => println!("√{x} no existe"),
        }
    }
}"#,
                erwartet: "√16 = 4\n√-4 no existe",
                hinweis: "`if x < 0.0 { None } else { Some(x.sqrt()) }`",
                loesung: r#"fn raiz_cuadrada(x: f64) -> Option<f64> {
    if x < 0.0 {
        None
    } else {
        Some(x.sqrt())
    }
}

fn main() {
    for x in [16.0, -4.0] {
        match raiz_cuadrada(x) {
            Some(r) => println!("√{x} = {r}"),
            None => println!("√{x} no existe"),
        }
    }
}"#,
            },
        ],
    },
    Lektion {
        titel: "Vec y HashMap",
        kurz: "Listas y diccionarios que pueden crecer.",
        kapitel: "Tipos propios",
        bloecke: &[
            Block::Text("Un array tiene una longitud fija. Si necesitas una lista que pueda crecer y encogerse, usas un **`Vec<T>`** (vector). Sus elementos están uno detrás de otro en el montón."),
            Block::Text("Prueba `push` y `pop`. Fíjate en **len** (número de elementos) y **cap** (espacio reservado): cuando el espacio se llena, Rust reserva el doble de memoria nueva y copia todo."),
            Block::Visual(Visual::Vec),
            Block::Ausfuehrbar(r#"fn main() {
    let mut numeros = vec![3, 1, 4];
    numeros.push(1);
    numeros.push(5);
    println!("{numeros:?}, longitud {}", numeros.len());

    // Leer: con índice (puede fallar) o con get (devuelve un Option)
    println!("Primero: {}", numeros[0]);
    println!("Décimo: {:?}", numeros.get(9));

    // Recorrer y cambiar
    for n in &mut numeros {
        *n *= 10;
    }
    let suma: i32 = numeros.iter().sum();
    println!("{numeros:?}, suma {suma}");

    numeros.sort();
    println!("Ordenado: {numeros:?}");
}"#),
            Block::Ueberschrift("HashMap: claves y valores"),
            Block::Text("Un `HashMap<K, V>` asocia **claves** con valores, como un diccionario. Muy práctico es `entry(...).or_insert(...)`: «obtén la entrada o créala con un valor inicial»."),
            Block::Ausfuehrbar(r#"use std::collections::HashMap;

fn main() {
    let mut puntos: HashMap<&str, u32> = HashMap::new();
    puntos.insert("Ada", 10);
    puntos.insert("Linus", 7);

    if let Some(p) = puntos.get("Ada") {
        println!("Ada tiene {p} puntos");
    }

    // Contar palabras
    let texto = "el perro y el balón y el árbol";
    let mut cuenta = HashMap::new();
    for palabra in texto.split_whitespace() {
        *cuenta.entry(palabra).or_insert(0) += 1;
    }
    println!("'el' aparece {} veces", cuenta["el"]);
    println!("'y' aparece {} veces", cuenta["y"]);
}"#),
            Block::Tipp("El orden de un `HashMap` es aleatorio. Si necesitas las claves ordenadas, usa `BTreeMap`: se usa casi igual."),
        ],
        aufgaben: &[
            Aufgabe::Rechnung {
                id: "12.1",
                frage: "¿Qué valor tiene `suma` al final?",
                code: Some("let mut v = vec![1, 2, 3];\nv.push(4);\nv.pop();\nv.push(10);\nlet suma: i32 = v.iter().sum();"),
                antworten: &["16"],
                hinweis: "`pop` quita el **último** elemento. Apunta el vector después de cada paso.",
                erklaerung: "[1,2,3] → [1,2,3,4] → [1,2,3] → [1,2,3,10]. Suma: 16.",
            },
            Aufgabe::Quiz {
                id: "12.2",
                frage: "`v` tiene 3 elementos. ¿Qué ocurre con `v[10]` y con `v.get(10)`?",
                code: None,
                optionen: &["Los dos devuelven 0", "`v[10]` falla (panic), `v.get(10)` devuelve `None`", "Los dos fallan", "`v[10]` devuelve `None`, `v.get(10)` falla"],
                richtig: 1,
                erklaerung: "Indexar con `[]` comprueba los límites y se detiene. `get` devuelve un `Option` que puedes tratar con seguridad.",
            },
            Aufgabe::Rechnung {
                id: "12.3",
                frage: "Un `Vec<i32>` vacío recibe espacio para 4 elementos en el primer `push` y duplica su capacidad cada vez que se llena. ¿Qué capacidad tiene después de 9 llamadas a `push`?",
                code: None,
                antworten: &["16"],
                hinweis: "4 → 8 → … El espacio se llena en el 5.º y en el 9.º elemento.",
                erklaerung: "Elementos 1–4: cap 4. Elemento 5: cap 8. Elemento 9: cap 16. (¡Pruébalo en el gráfico de arriba!)",
            },
            Aufgabe::Code {
                id: "12.4",
                aufgabe: "Calcula con un bucle `for` la suma y el máximo de los números. Salida esperada: `Suma: 75, máximo: 42`",
                vorlage: r#"fn main() {
    let numeros = vec![3, 17, 8, 42, 5];
    let mut suma = 0;
    let mut maximo = 0;
    for n in &numeros {
        // TODO
    }
    println!("Suma: {suma}, máximo: {maximo}");
}"#,
                erwartet: "Suma: 75, máximo: 42",
                hinweis: "`n` es una referencia `&i32`. Usa `*n`: `suma += *n;` y `if *n > maximo { maximo = *n; }`",
                loesung: r#"fn main() {
    let numeros = vec![3, 17, 8, 42, 5];
    let mut suma = 0;
    let mut maximo = 0;
    for n in &numeros {
        suma += *n;
        if *n > maximo {
            maximo = *n;
        }
    }
    println!("Suma: {suma}, máximo: {maximo}");
}"#,
            },
            Aufgabe::Code {
                id: "12.5",
                aufgabe: "Cuenta con un `HashMap` cuántas veces aparece cada color. Salida esperada: ver abajo.",
                vorlage: r#"use std::collections::HashMap;

fn main() {
    let texto = "rojo azul rojo verde rojo azul";
    let mut contador: HashMap<&str, u32> = HashMap::new();
    // TODO: aumenta el contador de cada palabra del texto

    for color in ["rojo", "azul", "verde"] {
        println!("{color}: {}", contador.get(color).unwrap_or(&0));
    }
}"#,
                erwartet: "rojo: 3\nazul: 2\nverde: 1",
                hinweis: "`for palabra in texto.split_whitespace() { *contador.entry(palabra).or_insert(0) += 1; }`",
                loesung: r#"use std::collections::HashMap;

fn main() {
    let texto = "rojo azul rojo verde rojo azul";
    let mut contador: HashMap<&str, u32> = HashMap::new();
    for palabra in texto.split_whitespace() {
        *contador.entry(palabra).or_insert(0) += 1;
    }

    for color in ["rojo", "azul", "verde"] {
        println!("{color}: {}", contador.get(color).unwrap_or(&0));
    }
}"#,
            },
        ],
    },
    // ─────────────────────────────── Capítulo 5 ───────────────────────────────
    Lektion {
        titel: "Manejo de errores",
        kurz: "panic!, Result y el operador ?.",
        kapitel: "Avanzado",
        bloecke: &[
            Block::Text("Rust distingue dos tipos de errores: los **irrecuperables** (un fallo de programación, p. ej. un índice fuera de rango) llevan a `panic!` y el programa se detiene. Los **recuperables** (p. ej. una entrada incorrecta del usuario) se devuelven con el enum `Result<T, E>`: `Ok(valor)` o `Err(error)`."),
            Block::Visual(Visual::Parse),
            Block::Ausfuehrbar(r#"fn main() {
    let entradas = ["42", "-7", "tres"];
    for e in entradas {
        match e.parse::<i32>() {
            Ok(numero) => println!("{e:>5} → número {numero}"),
            Err(error) => println!("{e:>5} → error: {error}"),
        }
    }
}"#),
            Block::Ueberschrift("El operador ?"),
            Block::Text("A menudo solo quieres pasar un error a quien llamó. Para eso existe `?`: si el resultado es `Ok`, `?` saca el valor. Si es `Err`, `?` termina la función al instante y devuelve el error. Solo funciona en funciones que devuelven ellas mismas un `Result`."),
            Block::Ausfuehrbar(r#"use std::num::ParseIntError;

fn doble(texto: &str) -> Result<i32, ParseIntError> {
    let numero: i32 = texto.trim().parse()?; // si hay error: vuelve ya
    Ok(numero * 2)
}

fn main() {
    println!("{:?}", doble("21"));
    println!("{:?}", doble("abc"));
}"#),
            Block::Achtung("`unwrap()` y `expect(\"…\")` sacan el valor de un `Option` o `Result` y provocan un **panic** con `None`/`Err`. Vale para experimentos rápidos, pero en programas reales es mejor tratar los errores con `match`, `if let`, `unwrap_or` o `?`."),
        ],
        aufgaben: &[
            Aufgabe::Quiz {
                id: "13.1",
                frage: "¿Qué hace el operador `?` con un valor `Err`?",
                code: None,
                optionen: &["Ignora el error", "Hace fallar el programa", "Devuelve el error de la función actual al instante", "Convierte el error en 0"],
                richtig: 2,
                erklaerung: "`?` es la forma corta de: con `Ok(v)` sigue con `v`; con `Err(e)`, `return Err(e)` inmediatamente.",
            },
            Aufgabe::Quiz {
                id: "13.2",
                frage: "¿Qué devuelve `\"256\".parse::<u8>()`?",
                code: None,
                optionen: &["`Ok(256)`", "`Ok(0)`", "`Err(...)`, porque 256 no cabe en un u8", "Un panic"],
                richtig: 2,
                erklaerung: "Un u8 solo llega a 255. `parse` lo indica limpiamente con un `Err`, sin fallar.",
            },
            Aufgabe::Rechnung {
                id: "13.3",
                frage: "¿Qué valor tiene `n`?",
                code: Some("let a: i32 = \"42\".parse().unwrap_or(0);\nlet b: i32 = \"x\".parse().unwrap_or(8);\nlet n = a + b;"),
                antworten: &["50"],
                hinweis: "\"42\" se puede convertir, \"x\" no: entonces se usa el valor por defecto.",
                erklaerung: "a = 42, b = 8 (porque \"x\" no es un número), así que n = 50.",
            },
            Aufgabe::Code {
                id: "13.4",
                aufgabe: "Completa `suma_texto` con el operador `?`: convierte los dos textos y devuelve la suma.",
                vorlage: r#"use std::num::ParseIntError;

fn suma_texto(a: &str, b: &str) -> Result<i32, ParseIntError> {
    // TODO: convierte a y b con .parse::<i32>()?
    Ok(0)
}

fn main() {
    for (a, b) in [("10", "32"), ("10", "abc")] {
        match suma_texto(a, b) {
            Ok(s) => println!("Suma: {s}"),
            Err(e) => println!("Error: {e}"),
        }
    }
}"#,
                erwartet: "Suma: 42\nError: invalid digit found in string",
                hinweis: "`let x = a.parse::<i32>()?;` y lo mismo para `b`. Después `Ok(x + y)`.",
                loesung: r#"use std::num::ParseIntError;

fn suma_texto(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x = a.parse::<i32>()?;
    let y = b.parse::<i32>()?;
    Ok(x + y)
}

fn main() {
    for (a, b) in [("10", "32"), ("10", "abc")] {
        match suma_texto(a, b) {
            Ok(s) => println!("Suma: {s}"),
            Err(e) => println!("Error: {e}"),
        }
    }
}"#,
            },
        ],
    },
    Lektion {
        titel: "Traits y genéricos",
        kurz: "Comportamiento común y código para muchos tipos.",
        kapitel: "Avanzado",
        bloecke: &[
            Block::Text("Un **trait** describe lo que un tipo *sabe hacer*, parecido a una interfaz en otros lenguajes. Distintos tipos pueden implementar el mismo trait a su manera."),
            Block::Visual(Visual::Formen),
            Block::Ausfuehrbar(r#"trait Animal {
    fn nombre(&self) -> String;
    fn sonido(&self) -> String;

    // Implementación por defecto que reciben todos los animales
    fn presentarse(&self) -> String {
        format!("{} dice {}", self.nombre(), self.sonido())
    }
}

struct Perro;
struct Gato { nombre: String }

impl Animal for Perro {
    fn nombre(&self) -> String { "El perro".to_string() }
    fn sonido(&self) -> String { "¡Guau!".to_string() }
}

impl Animal for Gato {
    fn nombre(&self) -> String { self.nombre.clone() }
    fn sonido(&self) -> String { "¡Miau!".to_string() }
}

fn main() {
    let animales: Vec<Box<dyn Animal>> = vec![
        Box::new(Perro),
        Box::new(Gato { nombre: "Minka".to_string() }),
    ];
    for a in &animales {
        println!("{}", a.presentarse());
    }
}"#),
            Block::Ueberschrift("Genéricos"),
            Block::Text("Con los **genéricos** escribes código que funciona para muchos tipos. `T` es un marcador. Con una **restricción de trait** como `T: PartialOrd` dices: «`T` puede ser cualquier tipo que se pueda comparar». Al compilar, Rust crea una versión propia y rápida para cada tipo usado."),
            Block::Ausfuehrbar(r#"fn mayor<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

fn main() {
    println!("{}", mayor(3, 7));
    println!("{}", mayor(2.5, 1.5));
    println!("{}", mayor("Manzana", "Pera"));
}"#),
            Block::Tipp("Muchos traits se pueden derivar automáticamente: `#[derive(Debug, Clone, PartialEq)]`. Traits estándar importantes son `Display` (para `{}`), `Debug` (para `{:?}`), `Clone`, `PartialEq` (para `==`) y `PartialOrd` (para `<`)."),
        ],
        aufgaben: &[
            Aufgabe::Quiz {
                id: "14.1",
                frage: "¿Qué es un trait?",
                code: None,
                optionen: &["Un tipo especial de variable", "Una descripción de comportamiento (métodos) que los tipos pueden implementar", "Un tipo de error", "Otro nombre para struct"],
                richtig: 1,
                erklaerung: "Los traits definen comportamiento común. Los tipos lo implementan con `impl Trait for Tipo`.",
            },
            Aufgabe::Rechnung {
                id: "14.2",
                frage: "Un `Circle { r: 2.0 }` calcula `area()` como π × r². ¿Qué resultado da? (Redondeado a 2 decimales: ¡usa el gráfico de arriba!)",
                code: None,
                antworten: &["12.57", "12,57"],
                hinweis: "π ≈ 3.14159. Calcula 3.14159 × 2 × 2.",
                erklaerung: "π × 4 ≈ 12.566…, redondeado 12.57.",
            },
            Aufgabe::Code {
                id: "14.3",
                aufgabe: "Implementa el trait `Forma` para `Cuadrado`. Salida esperada: `Área: 25`",
                vorlage: r#"trait Forma {
    fn area(&self) -> f64;
}

struct Cuadrado {
    lado: f64,
}

// TODO: impl Forma for Cuadrado { ... }

fn main() {
    let c = Cuadrado { lado: 5.0 };
    println!("Área: {}", c.area());
}"#,
                erwartet: "Área: 25",
                hinweis: "`impl Forma for Cuadrado { fn area(&self) -> f64 { self.lado * self.lado } }`",
                loesung: r#"trait Forma {
    fn area(&self) -> f64;
}

struct Cuadrado {
    lado: f64,
}

impl Forma for Cuadrado {
    fn area(&self) -> f64 {
        self.lado * self.lado
    }
}

fn main() {
    let c = Cuadrado { lado: 5.0 };
    println!("Área: {}", c.area());
}"#,
            },
            Aufgabe::Code {
                id: "14.4",
                aufgabe: "Escribe la función genérica `el_mayor`, que encuentra el elemento más grande de una lista. Debe funcionar con enteros y con decimales.",
                vorlage: r#"fn el_mayor<T: PartialOrd + Copy>(lista: &[T]) -> T {
    let mut max = lista[0];
    // TODO: recorre todos los elementos y actualiza max
    max
}

fn main() {
    println!("Número mayor: {}", el_mayor(&[3, 42, 7, 19]));
    println!("Decimal mayor: {}", el_mayor(&[1.5, 0.2, 3.5, 2.0]));
}"#,
                erwartet: "Número mayor: 42\nDecimal mayor: 3.5",
                hinweis: "`for &x in lista { if x > max { max = x; } }`: el `&x` desempaqueta la referencia directamente.",
                loesung: r#"fn el_mayor<T: PartialOrd + Copy>(lista: &[T]) -> T {
    let mut max = lista[0];
    for &x in lista {
        if x > max {
            max = x;
        }
    }
    max
}

fn main() {
    println!("Número mayor: {}", el_mayor(&[3, 42, 7, 19]));
    println!("Decimal mayor: {}", el_mayor(&[1.5, 0.2, 3.5, 2.0]));
}"#,
            },
        ],
    },
    Lektion {
        titel: "Closures e iteradores",
        kurz: "Funciones pequeñas y procesamiento de datos elegante.",
        kapitel: "Avanzado",
        bloecke: &[
            Block::Text("Una **closure** es una función pequeña y sin nombre que puedes guardar en una variable o pasar a otra parte. Se escribe con barras verticales: `|x| x + 1`. Las closures pueden usar («capturar») variables de su entorno."),
            Block::Ausfuehrbar(r#"fn main() {
    let mas_uno = |x: i32| x + 1;
    println!("{}", mas_uno(5));

    let factor = 3;
    let por_factor = |x| x * factor;   // captura `factor`
    println!("{}", por_factor(4));

    let mut contador = 0;
    let mut contar = || contador += 1; // cambia el entorno
    contar();
    contar();
    println!("Contado: {contador}");
}"#),
            Block::Ueberschrift("Iteradores"),
            Block::Text("Un **iterador** entrega valores uno tras otro. Con métodos como `map` (transformar), `filter` (seleccionar) y `sum`, `count` o `collect` (reunir) construyes cadenas de procesamiento. Construye una tú mismo:"),
            Block::Visual(Visual::Iterator),
            Block::Ausfuehrbar(r#"fn main() {
    let nombres = vec!["ada", "linus", "grace", "ferris"];

    let mayusculas: Vec<String> = nombres
        .iter()
        .filter(|n| n.len() > 3)
        .map(|n| n.to_uppercase())
        .collect();
    println!("{mayusculas:?}");

    for (i, n) in nombres.iter().enumerate() {
        println!("{}. {n}", i + 1);
    }

    let total: usize = nombres.iter().map(|n| n.len()).sum();
    println!("Letras en total: {total}");
}"#),
            Block::Tipp("En Rust, los iteradores son igual de rápidos que los bucles escritos a mano: el compilador los optimiza por completo. Se llaman **abstracciones de coste cero**."),
        ],
        aufgaben: &[
            Aufgabe::Rechnung {
                id: "15.1",
                frage: "¿Qué da esta expresión?",
                code: Some("(1..=5).map(|x| x * x).sum::<i32>()"),
                antworten: &["55"],
                hinweis: "Eleva al cuadrado del 1 al 5 y suma: 1 + 4 + 9 + …",
                erklaerung: "1 + 4 + 9 + 16 + 25 = 55.",
            },
            Aufgabe::Rechnung {
                id: "15.2",
                frage: "¿Qué da esta expresión?",
                code: Some("(1..=20).filter(|x| x % 3 == 0).count()"),
                antworten: &["6"],
                hinweis: "¿Cuántos múltiplos de 3 hay entre 1 y 20?",
                erklaerung: "3, 6, 9, 12, 15, 18 → 6. O bien: 20 / 3 = 6 (división entera).",
            },
            Aufgabe::Rechnung {
                id: "15.3",
                frage: "¿Qué imprime el programa?",
                code: Some("let factor = 3;\nlet por = |x: i32| x * factor;\nprintln!(\"{}\", por(por(2)));"),
                antworten: &["18"],
                hinweis: "De dentro hacia fuera: primero `por(2)`.",
                erklaerung: "por(2) = 6, por(6) = 18.",
            },
            Aufgabe::Code {
                id: "15.4",
                aufgabe: "Calcula con una cadena de iteradores la suma de los **cuadrados de todos los números pares** del 1 al 10. Salida esperada: `Resultado: 220`",
                vorlage: r#"fn main() {
    let resultado: i32 = (1..=10)
        // TODO: filter y map
        .sum();
    println!("Resultado: {resultado}");
}"#,
                erwartet: "Resultado: 220",
                hinweis: "`.filter(|x| x % 2 == 0).map(|x| x * x)` antes de `.sum()`",
                loesung: r#"fn main() {
    let resultado: i32 = (1..=10)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .sum();
    println!("Resultado: {resultado}");
}"#,
            },
            Aufgabe::Code {
                id: "15.5",
                aufgabe: "Toma todas las palabras con **más de 3 letras**, conviértelas a mayúsculas y únelas con `, `.",
                vorlage: r#"fn main() {
    let palabras = ["rust", "es", "super", "veloz", "y", "seguro"];
    let resultado: Vec<String> = palabras
        .iter()
        // TODO
        .collect();
    println!("{}", resultado.join(", "));
}"#,
                erwartet: "RUST, SUPER, VELOZ, SEGURO",
                hinweis: "`.filter(|w| w.len() > 3).map(|w| w.to_uppercase())`",
                loesung: r#"fn main() {
    let palabras = ["rust", "es", "super", "veloz", "y", "seguro"];
    let resultado: Vec<String> = palabras
        .iter()
        .filter(|w| w.len() > 3)
        .map(|w| w.to_uppercase())
        .collect();
    println!("{}", resultado.join(", "));
}"#,
            },
        ],
    },
    Lektion {
        titel: "Proyecto final: calculadora de notas",
        kurz: "Todo junto: structs, Vec, iteradores, Option.",
        kapitel: "Avanzado",
        bloecke: &[
            Block::Text("¡Hora de juntarlo todo! Vas a construir una pequeña **calculadora de notas** para una clase (notas del 1 al 6: 6 es la mejor y 4 es aprobado). Por el camino usarás structs, métodos, vectores, iteradores y `Option`."),
            Block::Ausfuehrbar(r#"struct Alumno {
    nombre: String,
    notas: Vec<f64>,
}

impl Alumno {
    fn nuevo(nombre: &str, notas: &[f64]) -> Self {
        Self { nombre: nombre.to_string(), notas: notas.to_vec() }
    }

    fn media(&self) -> Option<f64> {
        if self.notas.is_empty() {
            return None;
        }
        let suma: f64 = self.notas.iter().sum();
        Some(suma / self.notas.len() as f64)
    }
}

fn main() {
    let clase = vec![
        Alumno::nuevo("Ada", &[5.5, 6.0, 5.0]),
        Alumno::nuevo("Linus", &[4.0, 4.5, 3.5]),
        Alumno::nuevo("Nuevo", &[]),
    ];

    for a in &clase {
        match a.media() {
            Some(n) => println!("{:<6} {:.2} {}", a.nombre, n, if n >= 4.0 { "✅" } else { "❌" }),
            None => println!("{:<6} todavía sin notas", a.nombre),
        }
    }
}"#),
            Block::Text("Crea el proyecto en el terminal. En la **página del terminal** (menú superior) incluso puedes editar el archivo en el editor (`code src/main.rs`) y ejecutarlo con `cargo run`."),
            Block::Terminal,
            Block::Ueberschrift("¿Y ahora qué?"),
            Block::Text("¡Enhorabuena! Ya conoces lo más importante de Rust. Merece la pena seguir con: **lifetimes** (`'a`), **módulos** y visibilidad (`mod`, `pub`), **tests** con `#[test]`, **smart pointers** (`Box`, `Rc`, `RefCell`) y **concurrencia** con hilos. El libro oficial «The Rust Programming Language» es gratuito en doc.rust-lang.org/book y existe una traducción comunitaria al español. Para practicar, los ejercicios de Rustlings son geniales."),
        ],
        aufgaben: &[
            Aufgabe::Terminal {
                id: "16.1",
                aufgabe: "Crea el proyecto `notas`, entra en él, mira los archivos con `tree` y ejecútalo.",
                ziele: &["cargo new notas", "cd notas", "tree", "cargo run"],
                hinweis: "Cuatro comandos seguidos: `cargo new notas`, `cd notas`, `tree`, `cargo run`.",
            },
            Aufgabe::Quiz {
                id: "16.2",
                frage: "¿Por qué `media()` devuelve un `Option<f64>` en lugar de un simple `f64`?",
                code: None,
                optionen: &["Porque f64 es poco preciso", "Porque sin notas no hay media (división entre 0)", "Porque Option es más rápido", "Porque los métodos siempre deben devolver Option"],
                richtig: 1,
                erklaerung: "Con una lista vacía sería 0 / 0. Con `None` la función dice con sinceridad: aquí no hay resultado.",
            },
            Aufgabe::Rechnung {
                id: "16.3",
                frage: "Ada tiene las notas 5.5, 6.0 y 5.0. ¿Cuál es su media? (2 decimales)",
                code: None,
                antworten: &["5.50", "5.5", "5,50", "5,5"],
                hinweis: "Suma entre cantidad: (5.5 + 6.0 + 5.0) / 3",
                erklaerung: "16.5 / 3 = 5.50 ✅",
            },
            Aufgabe::Code {
                id: "16.4",
                aufgabe: "Amplía la calculadora: escribe el método `mejor_nota`, que devuelve la nota más alta como `Option<f64>`. Después imprime la mejor nota de cada alumno.",
                vorlage: r#"struct Alumno {
    nombre: String,
    notas: Vec<f64>,
}

impl Alumno {
    fn mejor_nota(&self) -> Option<f64> {
        // TODO: consejo: un bucle con una variable Option,
        // o bien: self.notas.iter().copied().reduce(f64::max)
        None
    }
}

fn main() {
    let clase = vec![
        Alumno { nombre: "Ada".to_string(), notas: vec![5.5, 6.0, 5.0] },
        Alumno { nombre: "Linus".to_string(), notas: vec![4.0, 4.5, 3.5] },
        Alumno { nombre: "Nuevo".to_string(), notas: vec![] },
    ];
    for a in &clase {
        match a.mejor_nota() {
            Some(n) => println!("{}: mejor nota {n}", a.nombre),
            None => println!("{}: sin notas", a.nombre),
        }
    }
}"#,
                erwartet: "Ada: mejor nota 6\nLinus: mejor nota 4.5\nNuevo: sin notas",
                hinweis: "La solución más sencilla: `self.notas.iter().copied().reduce(f64::max)`. `reduce` devuelve `None` si la lista está vacía.",
                loesung: r#"struct Alumno {
    nombre: String,
    notas: Vec<f64>,
}

impl Alumno {
    fn mejor_nota(&self) -> Option<f64> {
        let mut mejor: Option<f64> = None;
        for &n in &self.notas {
            mejor = match mejor {
                Some(b) if b >= n => Some(b),
                _ => Some(n),
            };
        }
        mejor
    }
}

fn main() {
    let clase = vec![
        Alumno { nombre: "Ada".to_string(), notas: vec![5.5, 6.0, 5.0] },
        Alumno { nombre: "Linus".to_string(), notas: vec![4.0, 4.5, 3.5] },
        Alumno { nombre: "Nuevo".to_string(), notas: vec![] },
    ];
    for a in &clase {
        match a.mejor_nota() {
            Some(n) => println!("{}: mejor nota {n}", a.nombre),
            None => println!("{}: sin notas", a.nombre),
        }
    }
}"#,
            },
        ],
    },
];
