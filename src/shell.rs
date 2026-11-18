//! A simulated shell with a virtual file system, `cargo` and `rustc`.
//! Real compilation happens through the Rust Playground (see `playground.rs`).

use std::collections::{BTreeMap, BTreeSet};

use crate::i18n::Sprache;

pub const HOME: &str = "/home/ferris";

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Art {
    Prompt,
    Ausgabe,
    Fehler,
    Gedimmt,
    Erfolg,
    Info,
}

#[derive(Clone, PartialEq)]
pub struct Zeile {
    pub art: Art,
    pub text: String,
}

/// What the shell still has to do (asynchronously) after a command.
#[derive(Clone, PartialEq)]
pub enum Aktion {
    Keine,
    Kompilieren { code: String, name: String, pfad: String, modus: Modus },
    Bearbeiten { pfad: String },
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Modus {
    Run,
    Build,
    Test,
    /// `rustc file.rs` – creates a "binary" in the virtual file system.
    Rustc,
    /// `./program` – runs a file built with rustc before.
    Binary,
}

#[derive(Clone, PartialEq)]
pub struct Shell {
    pub cwd: String,
    pub dateien: BTreeMap<String, String>,
    pub ordner: BTreeSet<String>,
    pub zeilen: Vec<Zeile>,
    pub verlauf: Vec<String>,
    pub sprache: Sprache,
}

const HALLO_WELT: &str = "fn main() {\n    println!(\"Hello, world!\");\n}\n";

impl Shell {
    pub fn neu(l: Sprache) -> Self {
        let mut s = Self {
            cwd: HOME.into(),
            dateien: BTreeMap::new(),
            ordner: BTreeSet::from(["/".into(), "/home".into(), HOME.into()]),
            zeilen: Vec::new(),
            verlauf: Vec::new(),
            sprache: l,
        };
        s.dateien.insert(
            format!("{HOME}/{}", tr!(l, "notes.txt", "notizen.txt", "notas.txt")),
            tr!(
                l,
                "Welcome to the Rust terminal!\nType `help` to see all commands.\n",
                "Willkommen im Rust-Terminal!\nTippe `hilfe`, um alle Befehle zu sehen.\n",
                "¡Bienvenido al terminal de Rust!\nEscribe `ayuda` para ver todos los comandos.\n"
            )
            .into(),
        );
        s.info(tr!(
            l,
            "Welcome! This is a simulated terminal. Type `help` for a list of commands.",
            "Willkommen! Das ist ein simuliertes Terminal. Tippe `hilfe` für eine Liste der Befehle.",
            "¡Bienvenido! Este es un terminal simulado. Escribe `ayuda` para ver la lista de comandos."
        ));
        s
    }

    pub fn prompt(&self) -> String {
        let pfad = self.cwd.strip_prefix(HOME).map_or_else(|| self.cwd.clone(), |rest| format!("~{rest}"));
        format!("ferris@rust:{pfad}$")
    }

    fn zeile(&mut self, art: Art, text: impl Into<String>) {
        self.zeilen.push(Zeile { art, text: text.into() });
    }
    pub fn ausgabe(&mut self, t: impl Into<String>) {
        self.zeile(Art::Ausgabe, t);
    }
    pub fn fehler(&mut self, t: impl Into<String>) {
        self.zeile(Art::Fehler, t);
    }
    pub fn gedimmt(&mut self, t: impl Into<String>) {
        self.zeile(Art::Gedimmt, t);
    }
    pub fn erfolg(&mut self, t: impl Into<String>) {
        self.zeile(Art::Erfolg, t);
    }
    pub fn info(&mut self, t: impl Into<String>) {
        self.zeile(Art::Info, t);
    }

    pub fn pfad(&self, eingabe: &str) -> String {
        let basis = if eingabe.starts_with('/') {
            String::new()
        } else if let Some(rest) = eingabe.strip_prefix('~') {
            return self.normalisieren(&format!("{HOME}{rest}"));
        } else {
            self.cwd.clone()
        };
        self.normalisieren(&format!("{basis}/{eingabe}"))
    }

    #[allow(clippy::unused_self)]
    fn normalisieren(&self, pfad: &str) -> String {
        let mut teile: Vec<&str> = Vec::new();
        for t in pfad.split('/') {
            match t {
                "" | "." => {}
                ".." => {
                    teile.pop();
                }
                x => teile.push(x),
            }
        }
        format!("/{}", teile.join("/"))
    }

    fn eltern(pfad: &str) -> String {
        match pfad.rfind('/') {
            Some(0) | None => "/".into(),
            Some(i) => pfad[..i].into(),
        }
    }

    fn name(pfad: &str) -> &str {
        pfad.rsplit('/').next().unwrap_or(pfad)
    }

    fn kinder(&self, ordner: &str) -> Vec<(String, bool)> {
        let praefix = if ordner == "/" { "/".to_string() } else { format!("{ordner}/") };
        let mut v: Vec<(String, bool)> = self
            .ordner
            .iter()
            .filter(|o| o.starts_with(&praefix) && o.len() > praefix.len() && !o[praefix.len()..].contains('/'))
            .map(|o| (o[praefix.len()..].to_string(), true))
            .collect();
        v.extend(
            self.dateien
                .keys()
                .filter(|d| d.starts_with(&praefix) && !d[praefix.len()..].contains('/'))
                .map(|d| (d[praefix.len()..].to_string(), false)),
        );
        v.sort();
        v
    }

    /// Searches upwards from the current folder for a Cargo.toml.
    fn projekt_wurzel(&self) -> Option<String> {
        let mut p = self.cwd.clone();
        loop {
            if self.dateien.contains_key(&format!("{p}/Cargo.toml")) {
                return Some(p);
            }
            if p == "/" {
                return None;
            }
            p = Self::eltern(&p);
        }
    }

    fn projekt_name(&self, wurzel: &str) -> String {
        self.dateien
            .get(&format!("{wurzel}/Cargo.toml"))
            .and_then(|t| t.lines().find_map(|l| l.strip_prefix("name = ")))
            .map_or_else(|| Self::name(wurzel).to_string(), |n| n.trim_matches('"').to_string())
    }

    /// Path of main.rs of the current (or last created) project.
    pub fn aktuelle_main(&self) -> Option<String> {
        if let Some(w) = self.projekt_wurzel() {
            return Some(format!("{w}/src/main.rs"));
        }
        self.dateien.keys().find(|k| k.ends_with("/src/main.rs")).cloned()
    }

    #[allow(clippy::too_many_lines)]
    pub fn befehl(&mut self, eingabe: &str) -> Aktion {
        let l = self.sprache;
        let eingabe = eingabe.trim();
        let prompt = self.prompt();
        self.zeile(Art::Prompt, format!("{prompt} {eingabe}"));
        if eingabe.is_empty() {
            return Aktion::Keine;
        }
        self.verlauf.push(eingabe.to_string());

        let teile: Vec<&str> = eingabe.split_whitespace().collect();
        let args = &teile[1..];

        match teile[0] {
            "hilfe" | "help" | "ayuda" => self.hilfe(),
            "clear" | "cls" => self.zeilen.clear(),
            "pwd" => {
                let cwd = self.cwd.clone();
                self.ausgabe(cwd);
            }
            "whoami" => self.ausgabe("ferris"),
            "echo" => self.ausgabe(args.join(" ").trim_matches('"').to_string()),
            "ls" | "dir" => {
                let ziel = args.iter().find(|a| !a.starts_with('-')).map_or_else(|| self.cwd.clone(), |a| self.pfad(a));
                if !self.ordner.contains(&ziel) {
                    let a = args.join(" ");
                    self.fehler(tr!(l, format!("ls: '{a}': no such file or directory"), format!("ls: '{a}': Datei oder Verzeichnis nicht gefunden"), format!("ls: '{a}': no existe el archivo o directorio")));
                } else {
                    let liste: Vec<String> = self
                        .kinder(&ziel)
                        .into_iter()
                        .map(|(n, ist_ordner)| if ist_ordner { format!("{n}/") } else { n })
                        .collect();
                    if !liste.is_empty() {
                        self.ausgabe(liste.join("  "));
                    }
                }
            }
            "cd" => {
                let ziel = args.first().map_or_else(|| HOME.to_string(), |a| self.pfad(a));
                if self.ordner.contains(&ziel) {
                    self.cwd = ziel;
                } else {
                    let a = args.join(" ");
                    self.fehler(tr!(l, format!("cd: {a}: no such directory"), format!("cd: {a}: Verzeichnis nicht gefunden"), format!("cd: {a}: no existe el directorio")));
                }
            }
            "mkdir" => match args.iter().find(|a| !a.starts_with('-')) {
                None => self.fehler(tr!(l, "mkdir: missing folder name", "mkdir: Ordnername fehlt", "mkdir: falta el nombre de la carpeta")),
                Some(a) => {
                    let p = self.pfad(a);
                    let mut x = p.clone();
                    while x != "/" {
                        self.ordner.insert(x.clone());
                        x = Self::eltern(&x);
                    }
                }
            },
            "touch" => match args.first() {
                None => self.fehler(tr!(l, "touch: missing file name", "touch: Dateiname fehlt", "touch: falta el nombre del archivo")),
                Some(a) => {
                    let p = self.pfad(a);
                    if self.ordner.contains(&Self::eltern(&p)) {
                        self.dateien.entry(p).or_default();
                    } else {
                        self.fehler(tr!(l, format!("touch: '{a}': folder doesn't exist"), format!("touch: '{a}': Ordner existiert nicht"), format!("touch: '{a}': la carpeta no existe")));
                    }
                }
            },
            "rm" => match args.iter().find(|a| !a.starts_with('-')) {
                None => self.fehler(tr!(l, "rm: missing file", "rm: Datei fehlt", "rm: falta el archivo")),
                Some(a) => {
                    let p = self.pfad(a);
                    if self.dateien.remove(&p).is_none() {
                        if self.ordner.contains(&p) && args.contains(&"-r") {
                            let pre = format!("{p}/");
                            self.ordner.retain(|o| o != &p && !o.starts_with(&pre));
                            self.dateien.retain(|d, _| !d.starts_with(&pre));
                        } else {
                            self.fehler(tr!(l, format!("rm: cannot remove '{a}' (folder? use rm -r)"), format!("rm: '{a}' kann nicht entfernt werden (Ordner? nutze rm -r)"), format!("rm: no se puede borrar '{a}' (¿carpeta? usa rm -r)")));
                        }
                    }
                }
            },
            "cat" => match args.first() {
                None => self.fehler(tr!(l, "cat: missing file", "cat: Datei fehlt", "cat: falta el archivo")),
                Some(a) => {
                    let p = self.pfad(a);
                    if let Some(inhalt) = self.dateien.get(&p).cloned() {
                        for l in inhalt.trim_end().lines() {
                            self.ausgabe(l.to_string());
                        }
                    } else {
                        self.fehler(tr!(l, format!("cat: {a}: file not found"), format!("cat: {a}: Datei nicht gefunden"), format!("cat: {a}: archivo no encontrado")));
                    }
                }
            },
            "tree" => {
                let start = args.first().map_or_else(|| self.cwd.clone(), |a| self.pfad(a));
                self.ausgabe(".".to_string());
                self.baum(&start, "");
            }
            "code" | "nano" | "vim" | "edit" => match args.first() {
                None => {
                    let b = teile[0];
                    self.fehler(tr!(l, format!("{b}: missing file, e.g. `{b} src/main.rs`"), format!("{b}: Datei fehlt, z.B. `{b} src/main.rs`"), format!("{b}: falta el archivo, p. ej. `{b} src/main.rs`")));
                }
                Some(a) => {
                    let p = self.pfad(a);
                    if self.ordner.contains(&Self::eltern(&p)) {
                        self.dateien.entry(p.clone()).or_default();
                        return Aktion::Bearbeiten { pfad: p };
                    }
                    let b = teile[0];
                    self.fehler(tr!(l, format!("{b}: folder for '{a}' doesn't exist"), format!("{b}: Ordner für '{a}' existiert nicht"), format!("{b}: la carpeta de '{a}' no existe")));
                }
            },
            "rustc" => return self.rustc(args),
            "rustup" => match args.first().copied() {
                Some("--version" | "-V") => {
                    self.ausgabe(tr!(l, "rustup 1.28.2 (simulated)", "rustup 1.28.2 (simuliert)", "rustup 1.28.2 (simulado)"));
                    self.gedimmt("info: This is the version for the rustup toolchain manager, not the rustc compiler.");
                }
                Some("update") => {
                    self.gedimmt("info: syncing channel updates for 'stable-aarch64-apple-darwin'");
                    self.erfolg("  stable-aarch64-apple-darwin unchanged - rustc 1.90.0");
                }
                Some("doc") => self.info(tr!(l, "Opens the official documentation (offline) in the browser – here: https://doc.rust-lang.org", "Öffnet die offizielle Dokumentation (offline) im Browser – hier: https://doc.rust-lang.org", "Abre la documentación oficial (sin conexión) en el navegador; aquí: https://doc.rust-lang.org")),
                _ => self.ausgabe(tr!(l, "Usage: rustup [--version | update | doc]", "Benutzung: rustup [--version | update | doc]", "Uso: rustup [--version | update | doc]")),
            },
            "cargo" => return self.cargo(args),
            "date" => self.ausgabe(tr!(l, "Today is a good day to learn Rust.", "Heute ist ein guter Tag, um Rust zu lernen.", "Hoy es un buen día para aprender Rust.")),
            "exit" => self.info(tr!(l, "There's no escape here – only more Rust. 🦀", "Hier gibt es kein Entkommen – nur mehr Rust. 🦀", "Aquí no hay escapatoria: solo más Rust. 🦀")),
            befehl if befehl.starts_with("./") => {
                let p = self.pfad(befehl);
                match self.dateien.get(&p) {
                    Some(inhalt) if inhalt.starts_with("#BINARY\n") => {
                        return Aktion::Kompilieren {
                            code: inhalt["#BINARY\n".len()..].to_string(),
                            name: Self::name(&p).into(),
                            pfad: p,
                            modus: Modus::Binary,
                        };
                    }
                    Some(_) => self.fehler(tr!(l, format!("{befehl}: not an executable"), format!("{befehl}: Keine ausführbare Datei"), format!("{befehl}: no es un ejecutable"))),
                    None => self.fehler(tr!(l, format!("{befehl}: file not found"), format!("{befehl}: Datei nicht gefunden"), format!("{befehl}: archivo no encontrado"))),
                }
            }
            other => {
                self.fehler(tr!(l, format!("{other}: command not found"), format!("{other}: Befehl nicht gefunden"), format!("{other}: comando no encontrado")));
                self.gedimmt(tr!(l, "Type `help` for a list of all commands.", "Tippe `hilfe` für eine Liste aller Befehle.", "Escribe `ayuda` para ver todos los comandos."));
            }
        }
        Aktion::Keine
    }

    fn baum(&mut self, ordner: &str, einrueckung: &str) {
        let kinder = self.kinder(ordner);
        let n = kinder.len();
        for (i, (name, ist_ordner)) in kinder.into_iter().enumerate() {
            let letzter = i + 1 == n;
            let zweig = if letzter { "└── " } else { "├── " };
            self.ausgabe(format!("{einrueckung}{zweig}{name}"));
            if ist_ordner {
                let neu = format!("{einrueckung}{}", if letzter { "    " } else { "│   " });
                let pfad = if ordner == "/" { format!("/{name}") } else { format!("{ordner}/{name}") };
                self.baum(&pfad, &neu);
            }
        }
    }

    fn hilfe(&mut self) {
        let l = self.sprache;
        self.info(tr!(l, "Available commands:", "Verfügbare Befehle:", "Comandos disponibles:"));
        for (b, t) in [
            ("cargo new <name>", tr!(l, "create a new Rust project", "neues Rust-Projekt anlegen", "crear un proyecto Rust nuevo")),
            ("cargo run", tr!(l, "compile and run the project (for real!)", "Projekt kompilieren und ausführen (echt!)", "compilar y ejecutar el proyecto (¡de verdad!)")),
            ("cargo build / check", tr!(l, "compile or check only", "nur kompilieren bzw. prüfen", "solo compilar o comprobar")),
            ("cargo test", tr!(l, "run tests", "Tests ausführen", "ejecutar tests")),
            ("cargo add <crate>", tr!(l, "add a dependency", "Abhängigkeit hinzufügen", "añadir una dependencia")),
            ("rustc <file.rs>", tr!(l, "compile a file directly", "eine Datei direkt kompilieren", "compilar un archivo directamente")),
            ("./<program>", tr!(l, "start a compiled program", "kompiliertes Programm starten", "ejecutar un programa compilado")),
            ("code <file>", tr!(l, "open a file in the editor (terminal page)", "Datei im Editor öffnen (Terminal-Seite)", "abrir un archivo en el editor (página del terminal)")),
            ("ls, cd, pwd, tree", tr!(l, "move around the file system", "im Dateisystem bewegen", "moverse por el sistema de archivos")),
            ("cat, touch, mkdir, rm", tr!(l, "view and create files", "Dateien ansehen und anlegen", "ver y crear archivos")),
            ("rustc --version", tr!(l, "show the version", "Version anzeigen", "mostrar la versión")),
            ("clear", tr!(l, "clear the screen", "Bildschirm leeren", "limpiar la pantalla")),
        ] {
            self.ausgabe(format!("  {b:<24}{t}"));
        }
        self.gedimmt(tr!(l, "Tip: use ↑ and ↓ to go through earlier commands.", "Tipp: Mit ↑ und ↓ blätterst du durch frühere Befehle.", "Consejo: con ↑ y ↓ recorres los comandos anteriores."));
    }

    fn rustc(&mut self, args: &[&str]) -> Aktion {
        let l = self.sprache;
        match args.first().copied() {
            None => self.fehler("error: no input filename given"),
            Some("--version" | "-V") => self.ausgabe(tr!(l, "rustc 1.90.0 (simulated, compiling uses Rust stable)", "rustc 1.90.0 (simuliert, kompiliert wird mit Rust stable)", "rustc 1.90.0 (simulado; se compila con Rust stable)")),
            Some(datei) => {
                let p = self.pfad(datei);
                match self.dateien.get(&p).cloned() {
                    None => self.fehler(format!("error: couldn't read `{datei}`: No such file or directory (os error 2)")),
                    Some(code) => {
                        let name = Self::name(&p).trim_end_matches(".rs").to_string();
                        return Aktion::Kompilieren { code, name, pfad: p, modus: Modus::Rustc };
                    }
                }
            }
        }
        Aktion::Keine
    }

    fn cargo(&mut self, args: &[&str]) -> Aktion {
        let l = self.sprache;
        match args.first().copied() {
            None | Some("--help" | "help") => {
                self.ausgabe("Rust's package manager");
                self.ausgabe(tr!(l, "Commands: new, init, run, build, check, test, add, fmt, clippy, --version", "Befehle: new, init, run, build, check, test, add, fmt, clippy, --version", "Comandos: new, init, run, build, check, test, add, fmt, clippy, --version"));
            }
            Some("--version" | "-V") => self.ausgabe(tr!(l, "cargo 1.90.0 (simulated)", "cargo 1.90.0 (simuliert)", "cargo 1.90.0 (simulado)")),
            Some("new") => match args.get(1) {
                None => self.fehler("error: the following required arguments were not provided: <PATH>"),
                Some(name) => {
                    let wurzel = self.pfad(name);
                    if self.ordner.contains(&wurzel) {
                        self.fehler(format!("error: destination `{wurzel}` already exists"));
                    } else {
                        self.projekt_anlegen(&wurzel);
                        self.erfolg(format!("    Creating binary (application) `{name}` package"));
                        self.gedimmt("note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html");
                    }
                }
            },
            Some("init") => {
                let wurzel = self.cwd.clone();
                if self.dateien.contains_key(&format!("{wurzel}/Cargo.toml")) {
                    self.fehler("error: `cargo init` cannot be run on existing Cargo packages");
                } else {
                    self.projekt_anlegen(&wurzel);
                    self.erfolg("    Creating binary (application) package");
                }
            }
            Some(sub @ ("run" | "build" | "check" | "test" | "r" | "b")) => {
                let Some(wurzel) = self.projekt_wurzel() else {
                    self.fehler(format!(
                        "error: could not find `Cargo.toml` in `{}` or any parent directory",
                        self.cwd
                    ));
                    self.gedimmt(tr!(l, "Tip: change into a project with `cd <project>` or create one with `cargo new`.", "Tipp: Wechsle mit `cd <projekt>` in ein Projekt oder lege eines mit `cargo new` an.", "Consejo: entra en un proyecto con `cd <proyecto>` o crea uno con `cargo new`."));
                    return Aktion::Keine;
                };
                let Some(code) = self.dateien.get(&format!("{wurzel}/src/main.rs")).cloned() else {
                    self.fehler("error: no targets specified in the manifest (src/main.rs missing)");
                    return Aktion::Keine;
                };
                let modus = match sub {
                    "run" | "r" => Modus::Run,
                    "test" => Modus::Test,
                    _ => Modus::Build,
                };
                let name = self.projekt_name(&wurzel);
                return Aktion::Kompilieren { code, name, pfad: wurzel, modus };
            }
            Some("add") => match (self.projekt_wurzel(), args.get(1)) {
                (None, _) => self.fehler("error: could not find `Cargo.toml`"),
                (_, None) => self.fehler(tr!(l, "error: no crate name given, e.g. `cargo add rand`", "error: kein Crate-Name angegeben, z.B. `cargo add rand`", "error: falta el nombre del crate, p. ej. `cargo add rand`")),
                (Some(w), Some(krate)) => {
                    let pfad = format!("{w}/Cargo.toml");
                    let toml = self.dateien.get(&pfad).cloned().unwrap_or_default();
                    self.dateien.insert(pfad, format!("{}{krate} = \"1.0\"\n", toml));
                    self.gedimmt("    Updating crates.io index");
                    self.erfolg(format!("      Adding {krate} v1.0 to dependencies"));
                    self.info(tr!(l, "Note: only the most popular crates are available in the playground (e.g. rand, serde, regex).", "Hinweis: Im Playground sind nur die beliebtesten Crates verfügbar (z.B. rand, serde, regex).", "Nota: en el playground solo están los crates más populares (p. ej. rand, serde, regex)."));
                }
            },
            Some("fmt") => self.erfolg(tr!(l, "Code formatted. ✨ (rustfmt keeps the style consistent)", "Code formatiert. ✨ (rustfmt sorgt für einheitlichen Stil)", "Código formateado. ✨ (rustfmt mantiene un estilo uniforme)")),
            Some("clippy") => {
                self.gedimmt("    Checking project v0.1.0");
                self.erfolg(tr!(l, "    Finished – Clippy has no suggestions. 📎", "    Finished – Clippy hat keine Verbesserungsvorschläge. 📎", "    Finished – Clippy no tiene sugerencias. 📎"));
            }
            Some(x) => self.fehler(format!("error: no such command: `{x}`")),
        }
        Aktion::Keine
    }

    fn projekt_anlegen(&mut self, wurzel: &str) {
        let name = Self::name(wurzel).to_string();
        let mut x = wurzel.to_string();
        while x != "/" {
            self.ordner.insert(x.clone());
            x = Self::eltern(&x);
        }
        self.ordner.insert(format!("{wurzel}/src"));
        self.dateien.insert(
            format!("{wurzel}/Cargo.toml"),
            format!("[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\n"),
        );
        self.dateien.insert(format!("{wurzel}/src/main.rs"), HALLO_WELT.into());
        self.dateien.insert(format!("{wurzel}/.gitignore"), "/target\n".into());
    }

    /// Shows the result of a compilation in the terminal.
    pub fn ergebnis(&mut self, name: &str, pfad: &str, modus: Modus, ergebnis: Result<crate::playground::Ergebnis, String>) {
        let l = self.sprache;
        let e = match ergebnis {
            Ok(e) => e,
            Err(msg) => {
                self.fehler(msg);
                return;
            }
        };
        let anzeige = pfad.strip_prefix(HOME).map_or_else(|| pfad.to_string(), |r| format!("~{r}"));
        if matches!(modus, Modus::Run | Modus::Build | Modus::Test) {
            self.erfolg(format!("   Compiling {name} v0.1.0 ({anzeige})"));
        }
        let meldungen = e.meldungen().replace("/playground", &anzeige).replace("playground", name);
        let kompiliert = e.cargo_zeilen().iter().any(|z| z.contains("Finished"));
        if !kompiliert {
            for l in meldungen.lines() {
                self.fehler(l.to_string());
            }
            if let Some(tipp) = crate::playground::fehler_erklaerung(l, &e.stderr) {
                self.info(format!("💡 {tipp}"));
            }
            return;
        }
        match modus {
            Modus::Build => {
                if !meldungen.is_empty() {
                    for l in meldungen.lines() {
                        self.gedimmt(l.to_string());
                    }
                }
                self.erfolg("    Finished `dev` profile [unoptimized + debuginfo] target(s)".to_string());
            }
            Modus::Rustc => {
                let bin = format!("{}/{name}", Self::eltern(pfad));
                let code = self.dateien.get(pfad).cloned().unwrap_or_default();
                self.dateien.insert(bin, format!("#BINARY\n{code}"));
                self.gedimmt(tr!(l, format!("(Compiled. Start the program with ./{name})"), format!("(Kompiliert. Starte das Programm mit ./{name})"), format!("(Compilado. Ejecuta el programa con ./{name})")));
            }
            Modus::Run | Modus::Test | Modus::Binary => {
                if modus != Modus::Binary {
                    self.erfolg("    Finished `dev` profile [unoptimized + debuginfo] target(s)".to_string());
                    if modus == Modus::Run {
                        self.erfolg(format!("     Running `target/debug/{name}`"));
                    }
                }
                for l in e.stdout.lines() {
                    self.ausgabe(l.to_string());
                }
                if !e.success {
                    for l in meldungen.lines() {
                        self.fehler(l.to_string());
                    }
                    if let Some(tipp) = crate::playground::fehler_erklaerung(l, &e.stderr) {
                        self.info(format!("💡 {tipp}"));
                    }
                }
            }
        }
    }
}
