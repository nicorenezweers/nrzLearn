//! Builds the static part of nrzLearn into the bundle of the web app:
//!
//! - `/` (x-default) and `/<lang>/`: landing page with the course picker
//! - `/<lang>/<course>/`: course page (curriculum, sample code, call to action)
//! - `/<lang>/impressum/`, `/<lang>/privacy/`: legal notice and privacy
//! - `/content/<course>/<lang>.json`: the JSON courses with their translations
//! - `app.html` (the app shell, `noindex`), `sitemap.xml`, `robots.txt`,
//!   `manifest.webmanifest`, icons and the social image
//!
//! Every page has its own title, description, canonical URL, `hreflang`
//! alternates for all languages, Open Graph data and JSON-LD.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use nrzlearn_model::{Block, COURSES, Course, CourseInfo, LANGUAGES, Language, apply, check_translation, strings, typography};
use serde_json::{Value, json};

/// Size of the built-in Rust course (see `src/welt.rs`; a test there checks it).
pub const RUST_WORLDS: usize = 16;
pub const RUST_LEVELS: usize = 100;
/// Languages the Rust course is written in.
const RUST_LANGS: [&str; 3] = ["en", "de", "es"];
const EMAIL: &str = "dieter@zweers.ch";

pub struct Options {
    pub root: PathBuf,
    pub out: PathBuf,
    pub base: String,
}

/// UI and site texts per language, English as fallback.
struct Texte {
    by_lang: BTreeMap<String, BTreeMap<String, String>>,
}

impl Texte {
    fn load(root: &Path) -> Result<Self, String> {
        let mut by_lang = BTreeMap::new();
        for l in LANGUAGES {
            let p = root.join("i18n").join(format!("{}.json", l.code));
            if let Ok(text) = std::fs::read_to_string(&p) {
                let m: BTreeMap<String, String> = serde_json::from_str(&text).map_err(|e| format!("{}: {e}", p.display()))?;
                by_lang.insert(l.code.to_string(), m);
            }
        }
        if !by_lang.contains_key("en") {
            return Err("i18n/en.json missing".into());
        }
        Ok(Self { by_lang })
    }

    fn get(&self, lang: &str, key: &str) -> String {
        match self.by_lang.get(lang).and_then(|m| m.get(key)).filter(|t| !t.trim().is_empty()) {
            Some(t) => typography(lang, t),
            None => self.by_lang["en"].get(key).cloned().unwrap_or_else(|| panic!("missing text {key}")),
        }
    }

    /// Course name in a language (`app.course.<id>` if it exists, else the brand name).
    fn kursname(&self, lang: &str, info: &CourseInfo) -> String {
        let key = format!("app.course.{}", info.id);
        if self.by_lang["en"].contains_key(&key) { self.get(lang, &key) } else { info.name.to_string() }
    }

    fn fill(&self, lang: &str, key: &str, vars: &[(&str, String)]) -> String {
        let mut t = self.get(lang, key);
        for (k, v) in vars {
            t = t.replace(&format!("{{{k}}}"), v);
        }
        t
    }
}

/// A JSON course in every language that has a valid translation.
struct Kurs {
    info: &'static CourseInfo,
    english: Course,
    translated: BTreeMap<String, Course>,
}

impl Kurs {
    /// The course in `lang`, or English. Second value: the language it is in.
    fn in_lang<'a>(&'a self, lang: &'a str) -> (&'a Course, &'a str) {
        match self.translated.get(lang) {
            Some(c) => (c, lang),
            None => (&self.english, "en"),
        }
    }

    fn levels(&self) -> usize {
        let n = self.english.units.len();
        n * 4 + n / 4
    }

    fn exercises(&self) -> usize {
        self.english.units.iter().map(|u| u.exercises.len()).sum()
    }
}

fn load_courses(root: &Path, warnings: &mut Vec<String>) -> Result<Vec<Kurs>, String> {
    let mut out = Vec::new();
    for info in COURSES.iter().filter(|c| c.id != "rust") {
        let dir = root.join("courses").join(info.id);
        let file = dir.join("course.json");
        let Ok(text) = std::fs::read_to_string(&file) else {
            warnings.push(format!("course {} missing ({})", info.id, file.display()));
            continue;
        };
        let english: Course = serde_json::from_str(&text).map_err(|e| format!("{}: {e}", file.display()))?;
        let errors = nrzlearn_model::validate(&english);
        if !errors.is_empty() {
            return Err(format!("{}:\n{}", file.display(), errors.join("\n")));
        }
        let en_strings = strings(&english);
        let mut translated = BTreeMap::new();
        for l in LANGUAGES.iter().filter(|l| l.code != "en") {
            let p = dir.join("strings").join(format!("{}.json", l.code));
            let Ok(t) = std::fs::read_to_string(&p) else { continue };
            let map: BTreeMap<String, String> = serde_json::from_str(&t).map_err(|e| format!("{}: {e}", p.display()))?;
            let problems = check_translation(&en_strings, &map);
            if problems.is_empty() {
                let map: BTreeMap<String, String> = map.iter().map(|(k, v)| (k.clone(), typography(l.code, v))).collect();
                translated.insert(l.code.to_string(), apply(&english, l.code, &map));
            } else {
                warnings.push(format!("{}: {} problems, using English ({})", p.display(), problems.len(), problems[0]));
            }
        }
        out.push(Kurs { info, english, translated });
    }
    Ok(out)
}

// ─────────────────────────────── HTML helpers ───────────────────────────────

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// `code` and **bold** to HTML (text escaped).
fn md(s: &str) -> String {
    let mut out = String::new();
    for (i, part) in s.split('`').enumerate() {
        if i % 2 == 1 {
            let _ = write!(out, "<code class=\"rounded bg-muted px-1.5 py-0.5 font-mono text-[0.85em]\">{}</code>", esc(part));
        } else {
            for (j, p) in part.split("**").enumerate() {
                if j % 2 == 1 {
                    let _ = write!(out, "<strong class=\"font-semibold text-foreground\">{}</strong>", esc(p));
                } else {
                    out.push_str(&esc(p));
                }
            }
        }
    }
    out
}

fn code_html(src: &str, course: &str) -> String {
    nrzlearn_model::highlight::tokenize(src, course)
        .into_iter()
        .map(|(class, text)| if class.is_empty() { esc(&text) } else { format!("<span class=\"{class}\">{}</span>", esc(&text)) })
        .collect()
}

fn tile(c: &CourseInfo, size: &str) -> String {
    if let Some(logo) = nrzlearn_model::icons::icon(c.id) {
        return format!(
            "<span class=\"inline-flex shrink-0 select-none items-center justify-center rounded-xl {size}\" style=\"background:{};color:{}\" aria-hidden=\"true\"><svg class=\"size-[62%]\" viewBox=\"{}\" fill=\"currentColor\"><use href=\"/logos.svg?v={}#{}\"/></svg></span>",
            c.color,
            c.on_color,
            logo.view_box,
            logo_version(),
            c.id
        );
    }
    let small = if c.mono.len() > 2 { " tracking-tighter" } else { "" };
    format!(
        "<span class=\"inline-flex shrink-0 select-none items-center justify-center rounded-xl font-mono font-extrabold leading-none{small} {size}\" style=\"background:{};color:{}\" aria-hidden=\"true\">{}</span>",
        c.color,
        c.on_color,
        esc(c.mono)
    )
}

/// All course logos as one SVG sprite (`/logos.svg`). The pages refer to it with `<use>`
/// instead of repeating the logos in every page (keeps the pages and the package small).
fn logo_sprite() -> String {
    let mut s = String::from("<svg xmlns=\"http://www.w3.org/2000/svg\">");
    for c in COURSES.iter() {
        if let Some(i) = nrzlearn_model::icons::icon(c.id) {
            let _ = write!(s, "<symbol id=\"{}\" viewBox=\"{}\">{}</symbol>", c.id, i.view_box, i.body);
        }
    }
    s.push_str("</svg>");
    s
}

/// `?v=…` of the sprite (FNV-1a of its content), so browsers load a changed sprite at once.
fn logo_version() -> &'static str {
    static V: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    V.get_or_init(|| {
        let h = logo_sprite().bytes().fold(0xcbf2_9ce4_8422_2325_u64, |h, b| (h ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3));
        format!("{:08x}", h >> 32)
    })
}

const LOGO: &str = "<svg class=\"size-8\" viewBox=\"0 0 64 64\" aria-hidden=\"true\"><rect width=\"64\" height=\"64\" rx=\"15\" fill=\"var(--foreground)\"/><path d=\"M15 46 Q15 34 30 36 T47 21\" fill=\"none\" stroke=\"var(--background)\" stroke-width=\"5\" stroke-linecap=\"round\"/><circle cx=\"15\" cy=\"46\" r=\"5.5\" fill=\"var(--background)\"/><circle cx=\"30\" cy=\"36\" r=\"5.5\" fill=\"var(--background)\"/><circle cx=\"47\" cy=\"20.5\" r=\"9.5\" fill=\"var(--background)\"/><path d=\"M47 13.9 L49 18.5 L53.6 20.5 L49 22.5 L47 27.1 L45 22.5 L40.4 20.5 L45 18.5 Z\" fill=\"var(--foreground)\"/></svg>";

fn icon(paths: &str, class: &str) -> String {
    format!("<svg class=\"{class}\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\" aria-hidden=\"true\">{paths}</svg>")
}

const I_GLOBE: &str = "<circle cx=\"12\" cy=\"12\" r=\"10\"/><path d=\"M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20\"/><path d=\"M2 12h20\"/>";
const I_MOON: &str = "<path d=\"M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z\"/>";
const I_ARROW: &str = "<path d=\"M5 12h14\"/><path d=\"m12 5 7 7-7 7\"/>";
const I_BOOK: &str = "<path d=\"M12 7v14\"/><path d=\"M3 18a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h5a4 4 0 0 1 4 4 4 4 0 0 1 4-4h5a1 1 0 0 1 1 1v13a1 1 0 0 1-1 1h-6a3 3 0 0 0-3 3 3 3 0 0 0-3-3z\"/>";
const I_DUMBBELL: &str = "<path d=\"M14.4 14.4 9.6 9.6\"/><path d=\"M18.657 21.485a2 2 0 1 1-2.829-2.828l-1.767 1.768a2 2 0 1 1-2.829-2.829l6.364-6.364a2 2 0 1 1 2.829 2.829l-1.768 1.767a2 2 0 1 1 2.828 2.829z\"/><path d=\"m21.5 21.5-1.4-1.4\"/><path d=\"M3.9 3.9 2.5 2.5\"/><path d=\"M6.404 12.768a2 2 0 1 1-2.829-2.829l1.768-1.767a2 2 0 1 1-2.828-2.829l2.828-2.828a2 2 0 1 1 2.829 2.828l1.767-1.768a2 2 0 1 1 2.829 2.829z\"/>";
const I_LOCK_OPEN: &str = "<rect width=\"18\" height=\"11\" x=\"3\" y=\"11\" rx=\"2\" ry=\"2\"/><path d=\"M7 11V7a5 5 0 0 1 9.9-1\"/>";
const I_FLAME: &str = "<path d=\"M8.5 14.5A2.5 2.5 0 0 0 11 12c0-1.38-.5-2-1-3-1.072-2.143-.224-4.054 2-6 .5 2.5 2 4.9 4 6.5 2 1.6 3 3.5 3 5.5a7 7 0 1 1-14 0c0-1.153.433-2.294 1-3a2.5 2.5 0 0 0 2.5 2.5z\"/>";
const I_LANG: &str = "<path d=\"m5 8 6 6\"/><path d=\"m4 14 6-6 2-3\"/><path d=\"M2 5h12\"/><path d=\"M7 2h1\"/><path d=\"m22 22-5-10-5 10\"/><path d=\"M14 18h6\"/>";

/// What a page needs besides its body.
struct Seite<'a> {
    lang: &'a Language,
    /// Path of this page in its language, e.g. `/de/python/` (`/` for x-default).
    pfad: String,
    /// Path of the same page in another language.
    pfad_fuer: &'a dyn Fn(&str) -> String,
    title: String,
    description: String,
    body: String,
    jsonld: Vec<Value>,
    noindex: bool,
    /// Root page: send people to their language.
    weiterleiten: bool,
    /// Link preview image (path), e.g. `/og/python.png`.
    bild: String,
    /// Theme colour (browser bar, accent of link previews).
    farbe: &'static str,
}

struct Kontext<'a> {
    o: &'a Options,
    t: &'a Texte,
    css: String,
    /// Courses that have pages (all of `COURSES` once every course exists).
    kurse: Vec<&'static CourseInfo>,
}

impl Kontext<'_> {
    fn url(&self, pfad: &str) -> String {
        format!("{}{pfad}", self.o.base)
    }

    /// Link preview image of a course (`site/static/og/<id>.png`, else the general one).
    fn vorschau(&self, id: &str) -> String {
        if self.o.root.join("site/static/og").join(format!("{id}.png")).is_file() { format!("/og/{id}.png") } else { "/og.png".to_string() }
    }

    fn sprachmenue(&self, s: &Seite) -> String {
        let l = s.lang;
        let mut links = String::new();
        for x in &LANGUAGES {
            let aktiv = if x.code == l.code { " bg-accent font-semibold" } else { "" };
            let _ = write!(
                links,
                "<a href=\"{href}\" hreflang=\"{tag}\" lang=\"{tag}\" class=\"block rounded-lg px-3 py-1.5 text-sm hover:bg-accent{aktiv}\" onclick=\"try{{localStorage.setItem('nrzlearn.lang','{code}')}}catch(e){{}}\">{name}</a>",
                href = (s.pfad_fuer)(x.code),
                tag = x.hreflang,
                code = x.code,
                name = esc(x.name)
            );
        }
        format!(
            "<details class=\"relative\"><summary class=\"inline-flex h-9 cursor-pointer list-none items-center gap-1.5 rounded-lg px-2.5 text-sm font-semibold hover:bg-accent\" title=\"{title}\">{globe}<span class=\"uppercase\">{code}</span></summary><nav class=\"absolute end-0 top-11 z-50 max-h-[70vh] w-56 overflow-y-auto rounded-2xl border-2 bg-card p-1.5 shadow-xl\" aria-label=\"{title}\">{links}</nav></details>",
            title = esc(&self.t.get(l.code, "app.language")),
            globe = icon(I_GLOBE, "size-4"),
            code = l.code,
        )
    }

    fn kopf(&self, s: &Seite) -> String {
        let l = s.lang.code;
        format!(
            "<header class=\"sticky top-0 z-40 border-b bg-background/85 backdrop-blur\"><div class=\"mx-auto flex h-14 max-w-6xl items-center gap-2 px-4\"><a href=\"/{l}/\" class=\"flex items-center gap-2 text-lg font-extrabold tracking-tight\">{LOGO}<span>nrz<span class=\"font-light\">Learn</span></span></a><div class=\"ms-auto flex items-center gap-1\">{menue}<button type=\"button\" class=\"inline-flex size-9 items-center justify-center rounded-lg hover:bg-accent\" title=\"{theme}\" aria-label=\"{theme}\" onclick=\"nrzTheme()\">{moon}</button></div></div></header>",
            menue = self.sprachmenue(s),
            theme = esc(&self.t.get(l, "site.theme")),
            moon = icon(I_MOON, "size-4"),
        )
    }

    fn fuss(&self, s: &Seite) -> String {
        let l = s.lang.code;
        let t = self.t;
        let mut sprachen = String::new();
        for x in &LANGUAGES {
            let _ = write!(sprachen, "<a href=\"{}\" hreflang=\"{}\" lang=\"{}\" class=\"hover:text-foreground hover:underline\">{}</a>", (s.pfad_fuer)(x.code), x.hreflang, x.hreflang, esc(x.name));
        }
        let mut kurse = String::new();
        for c in &self.kurse {
            let _ = write!(kurse, "<a href=\"/{l}/{}/\" class=\"hover:text-foreground hover:underline\">{}</a>", c.id, esc(&t.fill(l, "site.course_h1", &[("course", t.kursname(l, c))])));
        }
        format!(
            "<footer class=\"mt-16 border-t\"><div class=\"mx-auto grid max-w-6xl gap-8 px-4 py-10 text-sm text-muted-foreground md:grid-cols-[1fr_2fr]\"><div><a href=\"/{l}/\" class=\"flex items-center gap-2 text-base font-extrabold tracking-tight text-foreground\">{LOGO}<span>nrz<span class=\"font-light\">Learn</span></span></a><p class=\"mt-3 max-w-xs leading-6\">{text}</p><p class=\"mt-4 flex flex-wrap gap-x-3 gap-y-1\"><a class=\"underline underline-offset-2 hover:text-foreground\" href=\"/{l}/impressum/\">{impressum}</a><a class=\"underline underline-offset-2 hover:text-foreground\" href=\"/{l}/privacy/\">{privacy}</a></p></div><div class=\"grid gap-8 sm:grid-cols-2\"><div><div class=\"mb-2 font-semibold text-foreground\">{courses}</div><div class=\"grid grid-cols-2 gap-x-4 gap-y-1 lg:grid-cols-3\">{kurse}</div></div><div><div class=\"mb-2 font-semibold text-foreground\">{languages}</div><div class=\"flex flex-wrap gap-x-3 gap-y-1\">{sprachen}</div></div></div></div></footer>",
            text = esc(&t.get(l, "site.footer_text")),
            impressum = esc(&t.get(l, "app.legal_notice")),
            privacy = esc(&t.get(l, "app.privacy")),
            courses = esc(&t.get(l, "app.courses")),
            languages = esc(&t.get(l, "site.languages")),
        )
    }

    fn seite(&self, s: &Seite) -> String {
        let l = s.lang;
        let canonical = self.url(&s.pfad);
        let mut head = String::new();
        let _ = write!(
            head,
            "<meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><!--meta--><title>{title}</title><meta name=\"description\" content=\"{desc}\"><link rel=\"canonical\" href=\"{canonical}\">",
            title = esc(&s.title),
            desc = esc(&s.description)
        );
        if s.noindex {
            head.push_str("<meta name=\"robots\" content=\"noindex, follow\">");
        }
        for x in &LANGUAGES {
            let _ = write!(head, "<link rel=\"alternate\" hreflang=\"{}\" href=\"{}\">", x.hreflang, self.url(&(s.pfad_fuer)(x.code)));
        }
        let _ = write!(head, "<link rel=\"alternate\" hreflang=\"x-default\" href=\"{}\">", self.url(&(s.pfad_fuer)("x-default")));
        let _ = write!(
            head,
            "<meta property=\"og:type\" content=\"website\"><meta property=\"og:site_name\" content=\"nrzLearn\"><meta property=\"og:title\" content=\"{title}\"><meta property=\"og:description\" content=\"{desc}\"><meta property=\"og:url\" content=\"{canonical}\"><meta property=\"og:image\" content=\"{img}\"><meta property=\"og:image:width\" content=\"1200\"><meta property=\"og:image:height\" content=\"630\"><meta property=\"og:image:alt\" content=\"{title}\"><meta property=\"og:locale\" content=\"{og}\"><meta name=\"twitter:card\" content=\"summary_large_image\"><meta name=\"twitter:title\" content=\"{title}\"><meta name=\"twitter:description\" content=\"{desc}\"><meta name=\"twitter:image\" content=\"{img}\"><meta name=\"theme-color\" content=\"{farbe}\"><!--/meta-->",
            title = esc(&s.title),
            desc = esc(&s.description),
            img = self.url(&s.bild),
            og = l.og_locale,
            farbe = s.farbe
        );
        let _ = write!(
            head,
            "<link rel=\"icon\" href=\"/favicon.svg\" type=\"image/svg+xml\"><link rel=\"apple-touch-icon\" href=\"/apple-touch-icon.png\"><link rel=\"manifest\" href=\"/manifest.webmanifest\"><link rel=\"stylesheet\" href=\"{}\">",
            self.css
        );
        // Theme before first paint; language redirect on the root page.
        let mut boot = String::from(
            "(function(){try{var d=localStorage.getItem('darkmode');if(d==='true'||(d===null&&matchMedia('(prefers-color-scheme: dark)').matches))document.documentElement.classList.add('dark');}catch(e){}})();function nrzTheme(){var h=document.documentElement;var on=!h.classList.contains('dark');h.classList.toggle('dark',on);try{localStorage.setItem('darkmode',on?'true':'false')}catch(e){}}",
        );
        if s.weiterleiten {
            let codes: Vec<String> = LANGUAGES.iter().map(|x| format!("'{}'", x.code)).collect();
            let _ = write!(
                boot,
                "(function(){{try{{var c=[{}];var l=localStorage.getItem('nrzlearn.lang');if(!l){{var n=navigator.languages||[navigator.language||''];for(var i=0;i<n.length;i++){{var b=(n[i]||'').toLowerCase().split('-')[0];if(b==='no'||b==='nn')b='nb';if(c.indexOf(b)>=0){{l=b;break;}}}}}}if(l&&l!=='en'&&c.indexOf(l)>=0)location.replace('/'+l+'/');}}catch(e){{}}}})();",
                codes.join(",")
            );
        }
        let _ = write!(head, "<script>{boot}</script>");
        for j in &s.jsonld {
            let _ = write!(head, "<script type=\"application/ld+json\">{}</script>", serde_json::to_string(j).unwrap_or_default().replace("</", "<\\/"));
        }
        // Progress from this browser: "continue" badges and buttons.
        let fortschritt = "<script>(function(){try{var p=(JSON.parse(localStorage.getItem('nrzlearn.progress')||'{}').knoten)||{};var c={};for(var k in p){var i=k.indexOf(':');var id=i<0?'rust':k.slice(0,i);c[id]=(c[id]||0)+1;}document.querySelectorAll('[data-kurs]').forEach(function(el){var n=c[el.getAttribute('data-kurs')];if(!n)return;el.querySelectorAll('[data-weiter]').forEach(function(b){b.hidden=false});var a=el.querySelector('[data-weiter-text]');if(a)a.textContent=a.getAttribute('data-weiter-text');});}catch(e){}})();</script>";
        format!(
            "<!doctype html>\n<html lang=\"{tag}\" dir=\"{dir}\"><head>{head}</head><body class=\"min-h-screen bg-background text-foreground antialiased\">{kopf}<main>{body}</main>{fuss}{fortschritt}</body></html>\n",
            tag = l.hreflang,
            dir = if l.rtl { "rtl" } else { "ltr" },
            kopf = self.kopf(s),
            body = s.body,
            fuss = self.fuss(s),
        )
    }
}

fn write(path: &Path, text: &str) -> Result<(), String> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    }
    std::fs::write(path, text).map_err(|e| format!("{}: {e}", path.display()))
}

fn heute() -> String {
    let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    // Civil date from days since 1970 (Howard Hinnant's algorithm).
    let z = (secs / 86_400) as i64 + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    format!("{y:04}-{m:02}-{d:02}")
}

// ─────────────────────────────── Pages ───────────────────────────────

struct KursDaten {
    info: &'static CourseInfo,
    welten: usize,
    levels: usize,
    aufgaben: Option<usize>,
}

fn kurs_daten(kurse: &[Kurs]) -> Vec<KursDaten> {
    COURSES
        .iter()
        .filter_map(|info| {
            if info.id == "rust" {
                return Some(KursDaten { info, welten: RUST_WORLDS, levels: RUST_LEVELS, aufgaben: None });
            }
            let k = kurse.iter().find(|k| k.info.id == info.id)?;
            Some(KursDaten { info, welten: k.english.units.len(), levels: k.levels(), aufgaben: Some(k.exercises()) })
        })
        .collect()
}

fn landing(cx: &Kontext, l: &Language, daten: &[KursDaten], root: bool) -> Result<(), String> {
    let t = cx.t;
    let c = l.code;
    let mut gruppen = String::new();
    for g in nrzlearn_model::GROUPS {
    let mut karten = String::new();
    for k in daten.iter().filter(|k| k.info.group == g) {
        let tagline = t.get(c, &format!("site.course.{}.tagline", k.info.id));
        let _ = write!(
            karten,
            "<a href=\"/{c}/{id}/\" data-kurs=\"{id}\" data-name=\"{suche}\" class=\"group flex flex-col gap-3 rounded-2xl border-2 border-b-4 bg-card p-5 transition hover:-translate-y-0.5 hover:border-foreground\"><div class=\"flex items-center gap-3\">{tile}<div class=\"min-w-0\"><h3 class=\"text-lg font-extrabold\">{name}</h3><p class=\"whitespace-nowrap text-xs text-muted-foreground\">{levels}</p></div></div><p class=\"text-sm leading-6 text-muted-foreground\">{tagline}</p><span class=\"mt-auto flex flex-wrap items-center justify-between gap-2\"><span class=\"inline-flex items-center gap-1 text-sm font-semibold\">{lernen}{pfeil}</span><span data-weiter hidden class=\"rounded-full bg-foreground px-2.5 py-1 text-xs font-bold text-background\">{weiter}</span></span></a>",
            id = k.info.id,
            tile = tile(k.info, "size-12 text-base"),
            name = esc(&t.kursname(c, k.info)),
            levels = esc(&t.fill(c, "site.levels_count", &[("n", k.levels.to_string())])),
            weiter = esc(&t.get(c, "site.continue")),
            tagline = esc(&tagline),
            lernen = esc(&t.fill(c, "site.course_h1", &[("course", t.kursname(c, k.info))])),
            pfeil = icon(I_ARROW, "size-4 transition group-hover:translate-x-0.5 rtl:rotate-180"),
            suche = esc(&format!("{} {} {}", t.kursname(c, k.info).to_lowercase(), k.info.name.to_lowercase(), k.info.id)),
        );
    }
    if !karten.is_empty() {
        let _ = write!(
            gruppen,
            "<div data-gruppe class=\"mt-8\"><h3 class=\"text-sm font-bold uppercase tracking-wider text-muted-foreground\">{}</h3><div class=\"mt-3 grid gap-4 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4\">{karten}</div></div>",
            esc(&t.get(c, &format!("app.group.{g}")))
        );
    }
    }
    let mut features = String::new();
    for (i, ic) in [(1, I_BOOK), (2, I_DUMBBELL), (3, I_LOCK_OPEN), (4, I_FLAME)] {
        let _ = write!(
            features,
            "<div class=\"rounded-2xl border-2 p-5\"><div class=\"flex size-10 items-center justify-center rounded-xl bg-foreground text-background\">{}</div><h3 class=\"mt-3 font-bold\">{}</h3><p class=\"mt-1 text-sm leading-6 text-muted-foreground\">{}</p></div>",
            icon(ic, "size-5"),
            esc(&t.get(c, &format!("site.feature{i}_title"))),
            esc(&t.get(c, &format!("site.feature{i}_text")))
        );
    }
    let mut faq = String::new();
    let mut faq_ld = Vec::new();
    for i in 1..=5 {
        let q = t.get(c, &format!("site.faq{i}_q"));
        let a = t.get(c, &format!("site.faq{i}_a"));
        let _ = write!(faq, "<details class=\"group rounded-2xl border-2 p-5\"><summary class=\"flex cursor-pointer list-none items-center justify-between gap-4 font-semibold\">{}<span class=\"text-xl leading-none transition group-open:rotate-45\">+</span></summary><p class=\"mt-3 leading-7 text-muted-foreground\">{}</p></details>", esc(&q), esc(&a));
        faq_ld.push(json!({"@type": "Question", "name": q, "acceptedAnswer": {"@type": "Answer", "text": a}}));
    }
    let body = format!(
        "<section class=\"mx-auto max-w-6xl px-4 pb-6 pt-14 text-center sm:pt-20\"><p class=\"text-xs font-bold uppercase tracking-widest text-muted-foreground\">{kicker}</p><h1 class=\"mx-auto mt-4 max-w-3xl text-balance text-4xl font-extrabold tracking-tight sm:text-6xl\">{h1}</h1><p class=\"mx-auto mt-5 max-w-2xl text-balance text-lg leading-8 text-muted-foreground\">{hero}</p><a href=\"#courses\" class=\"mt-8 inline-flex h-12 items-center gap-2 rounded-2xl border-2 border-b-4 border-foreground bg-foreground px-7 font-bold uppercase tracking-wide text-background transition active:translate-y-0.5 active:border-b-2\">{cta}{pfeil}</a></section><section id=\"courses\" class=\"mx-auto max-w-6xl scroll-mt-20 px-4 py-10\"><h2 class=\"text-2xl font-extrabold tracking-tight sm:text-3xl\">{courses_title}</h2><p class=\"mt-2 text-muted-foreground\">{courses_text}</p><input type=\"search\" class=\"mt-6 h-12 w-full max-w-md rounded-2xl border-2 bg-card px-4 outline-none transition focus:border-foreground\" placeholder=\"{suchen}\" aria-label=\"{suchen}\" oninput=\"nrzSuche(this.value)\"><p data-nichts hidden class=\"mt-6 text-muted-foreground\">–</p>{gruppen}<script>function nrzSuche(q){{q=q.trim().toLowerCase();var any=false;document.querySelectorAll('[data-gruppe]').forEach(function(g){{var n=0;g.querySelectorAll('[data-name]').forEach(function(a){{var ok=!q||a.getAttribute('data-name').indexOf(q)>=0;a.hidden=!ok;if(ok)n++;}});g.hidden=n===0;if(n)any=true;}});document.querySelector('[data-nichts]').hidden=any;}}</script></section><section class=\"mx-auto max-w-6xl px-4 py-10\"><h2 class=\"text-2xl font-extrabold tracking-tight sm:text-3xl\">{features_title}</h2><div class=\"mt-6 grid gap-4 sm:grid-cols-2 lg:grid-cols-4\">{features}</div></section><section class=\"mx-auto max-w-3xl px-4 py-10\"><h2 class=\"text-2xl font-extrabold tracking-tight sm:text-3xl\">{faq_title}</h2><div class=\"mt-6 space-y-3\">{faq}</div></section>",
        kicker = esc(&t.get(c, "site.hero_kicker")),
        h1 = esc(&t.get(c, "site.hero_title")),
        hero = esc(&t.get(c, "site.hero_text")),
        cta = esc(&t.get(c, "site.cta_choose")),
        pfeil = icon(I_ARROW, "size-4 rotate-90"),
        courses_title = esc(&t.get(c, "site.courses_title")),
        courses_text = esc(&t.get(c, "site.courses_text")),
        suchen = esc(&t.get(c, "site.search_placeholder")),
        features_title = esc(&t.get(c, "site.features_title")),
        faq_title = esc(&t.get(c, "site.faq_title")),
    );
    // The root page (x-default) also offers every language.
    let body = if root {
        let mut sprachen = String::new();
        for x in &LANGUAGES {
            let _ = write!(
                sprachen,
                "<a href=\"/{code}/\" hreflang=\"{tag}\" lang=\"{tag}\" class=\"rounded-xl border-2 px-3 py-2 text-sm font-semibold transition hover:border-foreground\" onclick=\"try{{localStorage.setItem('nrzlearn.lang','{code}')}}catch(e){{}}\">{name}</a>",
                code = x.code,
                tag = x.hreflang,
                name = esc(x.name)
            );
        }
        format!("{body}<section class=\"mx-auto max-w-6xl px-4 py-10\"><h2 class=\"text-2xl font-extrabold tracking-tight sm:text-3xl\">Choose your language</h2><p class=\"mt-2 text-muted-foreground\">nrzLearn is available in 40 languages.</p><div class=\"mt-6 flex flex-wrap gap-2\">{sprachen}</div></section>")
    } else {
        body
    };
    let pfad = if root { "/".to_string() } else { format!("/{c}/") };
    let liste: Vec<Value> = daten
        .iter()
        .enumerate()
        .map(|(i, k)| json!({"@type": "ListItem", "position": i + 1, "url": cx.url(&format!("/{c}/{}/", k.info.id)), "name": t.fill(c, "site.course_h1", &[("course", t.kursname(c, k.info))])}))
        .collect();
    let jsonld = vec![
        json!({"@context": "https://schema.org", "@type": "WebSite", "name": "nrzLearn", "url": cx.url(&pfad), "inLanguage": l.hreflang, "description": t.get(c, "site.meta_description")}),
        json!({"@context": "https://schema.org", "@type": "Organization", "name": "nrzLearn", "url": cx.url("/"), "logo": cx.url("/icon-512.png")}),
        json!({"@context": "https://schema.org", "@type": "ItemList", "itemListElement": liste}),
        json!({"@context": "https://schema.org", "@type": "FAQPage", "inLanguage": l.hreflang, "mainEntity": faq_ld}),
    ];
    let pfad_fuer = |code: &str| if code == "x-default" { "/".to_string() } else { format!("/{code}/") };
    let s = Seite {
        lang: l,
        pfad: pfad.clone(),
        pfad_fuer: &pfad_fuer,
        title: if root { "nrzLearn – learn 48 programming languages for free".to_string() } else { t.get(c, "site.meta_title") },
        description: t.get(c, "site.meta_description"),
        body,
        jsonld,
        noindex: false,
        weiterleiten: root,
        bild: "/og.png".to_string(),
        farbe: "#111111",
    };
    let datei = if root { cx.o.out.join("index.html") } else { cx.o.out.join(c).join("index.html") };
    write(&datei, &cx.seite(&s))
}

/// One world in the curriculum: title, chapter, summary.
struct Welt {
    titel: String,
    kapitel: String,
    kurz: String,
}

#[allow(clippy::too_many_lines)]
fn kursseite(cx: &Kontext, l: &Language, k: &KursDaten, kurse: &[Kurs], daten: &[KursDaten]) -> Result<(), String> {
    let t = cx.t;
    let c = l.code;
    let id = k.info.id;
    let name_s = t.kursname(c, k.info);
    let name = name_s.as_str();
    // Curriculum, sample code and the language of the course texts.
    let (welten, beispiel, inhalt_lang): (Vec<Welt>, (String, Option<String>), &str) = if id == "rust" {
        let welten = (0..RUST_WORLDS)
            .map(|w| Welt {
                titel: t.get(c, &format!("site.rust.w{w}.title")),
                kapitel: t.get(c, &format!("site.rust.w{w}.chapter")),
                kurz: t.get(c, &format!("site.rust.w{w}.summary")),
            })
            .collect();
        let inhalt = if RUST_LANGS.contains(&c) { c } else { "en" };
        (welten, ("fn main() {\n    let name = \"Ferris\";\n    println!(\"Hello, {name}!\");\n}".into(), Some("Hello, Ferris!".into())), inhalt)
    } else {
        let kurs = kurse.iter().find(|x| x.info.id == id).ok_or("course missing")?;
        let (course, inhalt) = kurs.in_lang(c);
        let welten = course.units.iter().map(|u| Welt { titel: u.title.clone(), kapitel: u.chapter.clone(), kurz: u.summary.clone() }).collect();
        let beispiel = course.units[0]
            .lesson
            .iter()
            .find_map(|b| match b {
                Block::Code { code, output: Some(o) } if code.lines().count() <= 12 => Some((code.clone(), Some(o.clone()))),
                _ => None,
            })
            .unwrap_or_default();
        (welten, beispiel, inhalt)
    };
    let vars = |_: &[(&'static str, String)]| -> Vec<(&'static str, String)> {
        let mut v = vec![("course", name.to_string()), ("worlds", k.welten.to_string()), ("levels", k.levels.to_string())];
        if let Some(a) = k.aufgaben {
            v.push(("exercises", a.to_string()));
        }
        v
    };
    let stats = if k.aufgaben.is_some() { t.fill(c, "site.course_stats", &vars(&[])) } else { t.fill(c, "site.course_stats_rust", &vars(&[])) };
    let h1 = t.fill(c, "site.course_h1", &vars(&[]));
    let start = t.fill(c, "site.course_start", &vars(&[]));
    let sprache_hinweis = if inhalt_lang == c {
        String::new()
    } else {
        format!("<p class=\"mt-4 inline-flex items-center gap-2 rounded-full border-2 border-dashed px-3 py-1 text-sm text-muted-foreground\">{}{}</p>", icon(I_LANG, "size-4"), esc(&t.get(c, "site.course_content_english")))
    };

    let mut lehrplan = String::new();
    for (i, w) in welten.iter().enumerate() {
        let _ = write!(
            lehrplan,
            "<li class=\"flex gap-4 rounded-2xl border-2 p-4\"><span class=\"flex size-10 shrink-0 items-center justify-center rounded-xl bg-foreground font-extrabold text-background\">{n}</span><div class=\"min-w-0\"><div class=\"text-xs font-bold uppercase tracking-widest text-muted-foreground\">{welt} · {kap}</div><h3 class=\"font-bold\">{titel}</h3><p class=\"text-sm leading-6 text-muted-foreground\">{kurz}</p></div></li>",
            n = i + 1,
            welt = esc(&t.fill(c, "site.world_n", &[("n", (i + 1).to_string())])),
            kap = esc(&w.kapitel),
            titel = esc(&w.titel),
            kurz = md(&w.kurz),
        );
    }
    let (code, ausgabe) = &beispiel;
    let ausgabe_html = ausgabe.as_ref().map_or_else(String::new, |o| {
        format!("<div class=\"border-t bg-term-bg px-4 py-3 font-mono text-[0.8rem] leading-6 text-term-fg\"><div class=\"mb-1 text-[0.65rem] font-semibold uppercase tracking-widest text-term-fg/50\">{}</div><pre class=\"overflow-x-auto whitespace-pre\">{}</pre></div>", esc(&t.get(c, "app.output")), esc(o))
    });
    let mut andere = String::new();
    for x in daten.iter().filter(|x| x.info.id != id) {
        let _ = write!(
            andere,
            "<a href=\"/{c}/{xid}/\" data-kurs=\"{xid}\" class=\"flex items-center gap-3 rounded-2xl border-2 p-3 transition hover:border-foreground\">{tile}<span class=\"font-semibold\">{xname}</span><span data-weiter hidden class=\"ms-auto size-2 rounded-full bg-foreground\"></span></a>",
            xid = x.info.id,
            tile = tile(x.info, "size-9 text-xs"),
            xname = esc(&t.kursname(c, x.info)),
        );
    }
    let struktur = if id == "rust" { t.get(c, "site.course_structure_text_rust") } else { t.get(c, "site.course_structure_text") };
    let body = format!(
        "<div class=\"mx-auto max-w-6xl px-4 pt-8\"><nav class=\"text-sm text-muted-foreground\" aria-label=\"Breadcrumb\"><a href=\"/{c}/\" class=\"hover:text-foreground\">{home}</a> <span aria-hidden=\"true\">›</span> <span>{name}</span></nav></div><section class=\"mx-auto grid max-w-6xl items-start gap-10 px-4 py-8 lg:grid-cols-[1.1fr_1fr]\" data-kurs=\"{id}\"><div>{tile}<h1 class=\"mt-5 text-balance text-4xl font-extrabold tracking-tight sm:text-5xl\">{h1}</h1><p class=\"mt-4 text-lg leading-8 text-muted-foreground\">{intro}</p><p class=\"mt-4 text-sm font-semibold\">{stats}</p>{sprache_hinweis}<div class=\"mt-7 flex flex-wrap items-center gap-3\"><a href=\"/{c}/{id}/learn\" rel=\"nofollow\" class=\"inline-flex h-12 items-center gap-2 rounded-2xl border-2 border-b-4 border-foreground bg-foreground px-7 font-bold uppercase tracking-wide text-background transition active:translate-y-0.5 active:border-b-2\"><span data-weiter-text=\"{weiter}\">{start}</span>{pfeil}</a><span class=\"text-sm text-muted-foreground\">{free}</span></div></div><div><div class=\"text-xs font-bold uppercase tracking-widest text-muted-foreground\">{sample}</div><div class=\"mt-2 overflow-hidden rounded-2xl border-2 bg-code-bg shadow-lg\" dir=\"ltr\"><pre class=\"overflow-x-auto p-5 font-mono text-[0.85rem] leading-6\"><code>{code}</code></pre>{ausgabe_html}</div></div></section><section class=\"mx-auto max-w-6xl px-4 py-8\"><h2 class=\"text-2xl font-extrabold tracking-tight sm:text-3xl\">{curriculum}</h2><ol class=\"mt-6 grid gap-3 md:grid-cols-2\">{lehrplan}</ol></section><section class=\"mx-auto max-w-6xl px-4 py-8\"><div class=\"rounded-2xl border-2 border-dashed p-6\"><h2 class=\"text-xl font-extrabold\">{struktur_titel}</h2><p class=\"mt-2 leading-7 text-muted-foreground\">{struktur}</p></div></section><section class=\"mx-auto max-w-6xl px-4 py-8\"><h2 class=\"text-xl font-extrabold\">{andere_titel}</h2><div class=\"mt-4 grid gap-3 sm:grid-cols-3 lg:grid-cols-5\">{andere}</div></section>",
        home = esc(&t.get(c, "site.home")),
        name = esc(name),
        tile = tile(k.info, "size-16 text-xl"),
        intro = esc(&t.get(c, &format!("site.course.{id}.intro"))),
        stats = esc(&stats),
        weiter = esc(&t.get(c, "site.continue")),
        start = esc(&start),
        pfeil = icon(I_ARROW, "size-4 rtl:rotate-180"),
        free = esc(&t.get(c, "site.course_free")),
        sample = esc(&t.get(c, "site.course_sample")),
        code = code_html(code, id),
        curriculum = esc(&t.get(c, "site.course_curriculum")),
        struktur_titel = esc(&t.get(c, "site.course_structure_title")),
        struktur = esc(&struktur),
        andere_titel = esc(&t.get(c, "site.other_courses")),
    );
    let titel = t.fill(c, "site.course_meta_title", &vars(&[]));
    let beschreibung = t.fill(c, "site.course_meta_description", &vars(&[]));
    let url = cx.url(&format!("/{c}/{id}/"));
    let stunden = (k.levels * 5).div_ceil(60);
    let inhalt_tag = LANGUAGES.iter().find(|x| x.code == inhalt_lang).map_or("en", |x| x.hreflang);
    let syllabus: Vec<Value> = welten.iter().map(|w| json!({"@type": "Syllabus", "name": w.titel, "description": w.kurz.replace('`', "").replace("**", "")})).collect();
    let jsonld = vec![
        json!({
            "@context": "https://schema.org",
            "@type": "Course",
            "name": h1,
            "description": t.get(c, &format!("site.course.{id}.intro")),
            "url": url,
            "image": cx.url("/og.png"),
            "inLanguage": inhalt_tag,
            "isAccessibleForFree": true,
            "educationalLevel": "Beginner",
            "teaches": name,
            "provider": {"@type": "Organization", "name": "nrzLearn", "sameAs": cx.url("/")},
            "offers": [{"@type": "Offer", "category": "Free", "price": 0, "priceCurrency": "EUR"}],
            "hasCourseInstance": [{"@type": "CourseInstance", "courseMode": "Online", "courseWorkload": format!("PT{stunden}H")}],
            "syllabusSections": syllabus,
        }),
        json!({
            "@context": "https://schema.org",
            "@type": "BreadcrumbList",
            "itemListElement": [
                {"@type": "ListItem", "position": 1, "name": "nrzLearn", "item": cx.url(&format!("/{c}/"))},
                {"@type": "ListItem", "position": 2, "name": name, "item": url},
            ],
        }),
    ];
    let pfad_fuer = move |code: &str| if code == "x-default" { format!("/en/{id}/") } else { format!("/{code}/{id}/") };
    let s = Seite {
        lang: l,
        pfad: format!("/{c}/{id}/"),
        pfad_fuer: &pfad_fuer,
        title: titel,
        description: beschreibung,
        body,
        jsonld,
        noindex: false,
        weiterleiten: false,
        bild: cx.vorschau(id),
        farbe: k.info.color,
    };
    write(&cx.o.out.join(c).join(id).join("index.html"), &cx.seite(&s))
}

fn abschnitt(titel: &str, inhalt: &str) -> String {
    format!("<section class=\"mt-6\"><h2 class=\"text-lg font-bold\">{}</h2><div class=\"mt-1 space-y-1 leading-7\">{inhalt}</div></section>", esc(titel))
}

fn impressum(cx: &Kontext, l: &Language) -> Result<(), String> {
    let t = cx.t;
    let c = l.code;
    let mail = format!("<a class=\"underline underline-offset-2\" href=\"mailto:{EMAIL}\">{EMAIL}</a>");
    let hinweis = if c == "de" { String::new() } else { format!("<p class=\"mt-4 rounded-xl border-2 border-dashed p-3 text-sm text-muted-foreground\">{}</p>", esc(&t.get(c, "site.impressum_note"))) };
    let deutsch = [
        abschnitt("Angaben gemäss § 5 DDG", "<p>Nico Rene Zweers<br>c/o Online-Impressum 5415<br>Europaring 90<br>53757 Sankt Augustin<br>Deutschland</p>"),
        abschnitt("Kontakt", &format!("<p>E-Mail: {mail}</p>")),
        abschnitt("Aufsichtsbehörde", "<p>Zuständige Aufsichtsbehörde: Landesanstalt für Medien Nordrhein-Westfalen | Sitz: Deutschland</p>"),
        abschnitt("Umsatzsteuer", "<p>Kein Umsatzsteuerausweis aufgrund Anwendung der Kleinunternehmerregelung gemäss § 19 UStG.</p>"),
        abschnitt("Verantwortlich für den Inhalt nach § 18 Abs. 2 MStV", "<p>Nico Rene Zweers, Anschrift wie oben.</p>"),
        abschnitt("Haftung für Inhalte", "<p>Die Inhalte dieser Website werden mit Sorgfalt erstellt. Für die Richtigkeit, Vollständigkeit und Aktualität der Inhalte wird jedoch keine Gewähr übernommen.</p>"),
        abschnitt("Haftung für Links", "<p>Diese Website enthält Links zu Websites Dritter, auf deren Inhalte kein Einfluss besteht. Für diese fremden Inhalte ist stets der jeweilige Anbieter oder Betreiber verantwortlich.</p>"),
        abschnitt("Urheberrecht", "<p>Die Texte, Bilder und Programme auf dieser Website unterliegen dem Urheberrecht, soweit nicht anders angegeben (zum Beispiel bei Open-Source-Lizenzen).</p>"),
    ]
    .concat();
    let zweers = if matches!(c, "en" | "de" | "fr" | "it" | "es" | "pt" | "nl" | "ja" | "zh") { c } else { "en" };
    let body = format!(
        "<div class=\"mx-auto max-w-3xl px-4 py-10\"><h1 class=\"text-3xl font-extrabold tracking-tight\">Impressum / Imprint</h1>{hinweis}<article lang=\"de\" dir=\"ltr\" class=\"mt-2\">{deutsch}</article><p lang=\"en\" dir=\"ltr\" class=\"mt-8 rounded-xl border-2 p-4 text-sm leading-6\"><span class=\"font-semibold\">Imprint (English): </span>Nico Rene Zweers, c/o Online-Impressum 5415, Europaring 90, 53757 Sankt Augustin, Germany · Email: {mail} · Supervisory authority: Landesanstalt für Medien Nordrhein-Westfalen · No VAT shown (small business, § 19 UStG).</p><p class=\"mt-6 flex flex-wrap gap-x-5 gap-y-2 text-sm\"><a class=\"font-semibold underline underline-offset-2\" href=\"/{c}/privacy/\">{privacy}</a><a class=\"text-muted-foreground underline underline-offset-2 hover:text-foreground\" href=\"https://zweers.ch/{zweers}/impressum\" rel=\"noopener\">{original}</a></p></div>",
        privacy = esc(&t.get(c, "app.privacy")),
        original = esc(&t.get(c, "site.impressum_original")),
    );
    let pfad_fuer = |code: &str| if code == "x-default" { "/en/impressum/".to_string() } else { format!("/{code}/impressum/") };
    let s = Seite {
        lang: l,
        pfad: format!("/{c}/impressum/"),
        pfad_fuer: &pfad_fuer,
        title: format!("{} · nrzLearn", t.get(c, "app.legal_notice")),
        description: format!("{} – nrzLearn", t.get(c, "app.legal_notice")),
        body,
        jsonld: Vec::new(),
        noindex: true,
        weiterleiten: false,
        bild: "/og.png".to_string(),
        farbe: "#111111",
    };
    write(&cx.o.out.join(c).join("impressum").join("index.html"), &cx.seite(&s))
}

fn datenschutz(cx: &Kontext, l: &Language) -> Result<(), String> {
    let t = cx.t;
    let c = l.code;
    let p = |k: &str| format!("<p>{}</p>", esc(&t.get(c, k)));
    let zweers = if matches!(c, "en" | "de" | "fr" | "it" | "es" | "pt" | "nl" | "ja" | "zh") { c } else { "en" };
    let body = format!(
        "<div class=\"mx-auto max-w-3xl px-4 py-10\"><h1 class=\"text-3xl font-extrabold tracking-tight\">{titel}</h1><p class=\"mt-4 leading-7\">{intro}</p>{a}{b}{stats}{d}<div data-analytics hidden>{analytics}</div>{e}<p class=\"mt-8 text-sm\"><a class=\"text-muted-foreground underline underline-offset-2 hover:text-foreground\" href=\"https://zweers.ch/{zweers}/datenschutz\" rel=\"noopener\">{voll}</a></p></div>",
        titel = esc(&t.get(c, "app.privacy")),
        intro = esc(&t.get(c, "site.privacy_intro")),
        a = abschnitt(&t.get(c, "site.privacy_local_title"), &p("site.privacy_local_text")),
        b = abschnitt(&t.get(c, "site.privacy_none_title"), &p("site.privacy_none_text")),
        d = abschnitt(&t.get(c, "site.privacy_playground_title"), &p("site.privacy_playground_text")),
        stats = abschnitt(&t.get(c, "site.privacy_stats_title"), &p("site.privacy_stats_text")),
        analytics = abschnitt(&t.get(c, "site.privacy_analytics_title"), &p("site.privacy_analytics_text")),
        e = abschnitt(
            &t.get(c, "site.privacy_contact_title"),
            &format!("<p>{} <a class=\"underline underline-offset-2\" href=\"/{c}/impressum/\">{}</a> · {}: <a class=\"underline underline-offset-2\" href=\"mailto:{EMAIL}\">{EMAIL}</a></p>", esc(&t.get(c, "site.privacy_contact_text")), esc(&t.get(c, "app.legal_notice")), esc(&t.get(c, "site.email")))
        ),
        voll = esc(&t.get(c, "site.privacy_full")),
    );
    let pfad_fuer = |code: &str| if code == "x-default" { "/en/privacy/".to_string() } else { format!("/{code}/privacy/") };
    let s = Seite {
        lang: l,
        pfad: format!("/{c}/privacy/"),
        pfad_fuer: &pfad_fuer,
        title: format!("{} · nrzLearn", t.get(c, "app.privacy")),
        description: t.get(c, "site.privacy_meta_description"),
        body,
        jsonld: Vec::new(),
        noindex: true,
        weiterleiten: false,
        bild: "/og.png".to_string(),
        farbe: "#111111",
    };
    write(&cx.o.out.join(c).join("privacy").join("index.html"), &cx.seite(&s))
}

/// The app shell: dx's index.html, renamed, with `noindex`.
fn app_shell(out: &Path, base: &str) -> Result<(), String> {
    let index = out.join("index.html");
    let app = out.join("app.html");
    let html = match std::fs::read_to_string(&index) {
        Ok(h) if h.contains("id=\"main\"") => h,
        _ => std::fs::read_to_string(&app).map_err(|_| format!("{}: no app shell (run dx bundle first)", index.display()))?,
    };
    let html = if html.contains("name=\"robots\"") {
        html
    } else {
        html.replacen("<head>", "<head>\n        <meta name=\"robots\" content=\"noindex\">", 1)
    };
    let html = html.replacen("<html>", "<html lang=\"en\">", 1);
    // The server replaces this block with the link preview of the course for
    // app pages like `/de/python/learn` (see server/src/html.rs).
    let html = if html.contains("<!--meta-->") {
        html
    } else {
        html.replacen("<head>", &format!("<head>\n        <!--meta--><meta property=\"og:site_name\" content=\"nrzLearn\"><meta property=\"og:image\" content=\"{base}/og.png\"><!--/meta-->"), 1)
    };
    write(&app, &html)
}

pub fn build(o: &Options) -> Result<Vec<String>, String> {
    let mut warnings = Vec::new();
    let t = Texte::load(&o.root)?;
    let kurse = load_courses(&o.root, &mut warnings)?;
    for l in LANGUAGES.iter().filter(|l| l.code != "en") {
        if !t.by_lang.contains_key(l.code) {
            warnings.push(format!("i18n/{}.json missing: English texts", l.code));
        }
    }

    app_shell(&o.out, &o.base)?;
    // The stylesheet built by dx (fingerprinted).
    let css = std::fs::read_dir(o.out.join("assets"))
        .map_err(|e| format!("{}/assets: {e}", o.out.display()))?
        .filter_map(Result::ok)
        .map(|e| e.file_name().to_string_lossy().to_string())
        .find(|n| n.starts_with("tailwind-") && n.ends_with(".css"))
        .map(|n| format!("/assets/{n}"))
        .ok_or("no assets/tailwind-*.css in the bundle")?;
    let vorhanden: Vec<&'static CourseInfo> = COURSES.iter().filter(|c| c.id == "rust" || kurse.iter().any(|k| k.info.id == c.id)).collect();
    let cx = Kontext { o, t: &t, css, kurse: vorhanden };

    // Course texts for the app.
    for k in &kurse {
        let dir = o.out.join("content").join(k.info.id);
        write(&dir.join("en.json"), &serde_json::to_string(&k.english).map_err(|e| e.to_string())?)?;
        for (lang, c) in &k.translated {
            write(&dir.join(format!("{lang}.json")), &serde_json::to_string(c).map_err(|e| e.to_string())?)?;
        }
    }

    let daten = kurs_daten(&kurse);
    landing(&cx, &LANGUAGES[0], &daten, true)?;
    let mut urls = vec!["/".to_string()];
    for l in &LANGUAGES {
        landing(&cx, l, &daten, false)?;
        urls.push(format!("/{}/", l.code));
        for k in &daten {
            kursseite(&cx, l, k, &kurse, &daten)?;
            urls.push(format!("/{}/{}/", l.code, k.info.id));
        }
        impressum(&cx, l)?;
        datenschutz(&cx, l)?;
    }

    let datum = heute();
    let mut sitemap = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");
    for u in &urls {
        let _ = writeln!(sitemap, "  <url><loc>{}</loc><lastmod>{datum}</lastmod></url>", cx.url(u));
    }
    sitemap.push_str("</urlset>\n");
    write(&o.out.join("sitemap.xml"), &sitemap)?;
    write(&o.out.join("robots.txt"), &format!("User-agent: *\nAllow: /\n\nSitemap: {}\n", cx.url("/sitemap.xml")))?;
    let manifest = json!({
        "name": "nrzLearn",
        "short_name": "nrzLearn",
        "description": t.get("en", "site.meta_description"),
        "start_url": "/",
        "display": "standalone",
        "background_color": "#ffffff",
        "theme_color": "#111111",
        "icons": [
            {"src": "/icon-192.png", "sizes": "192x192", "type": "image/png"},
            {"src": "/icon-512.png", "sizes": "512x512", "type": "image/png"},
            {"src": "/favicon.svg", "sizes": "any", "type": "image/svg+xml"}
        ]
    });
    write(&o.out.join("manifest.webmanifest"), &serde_json::to_string_pretty(&manifest).map_err(|e| e.to_string())?)?;
    write(&o.out.join("logos.svg"), &logo_sprite())?;
    copy_dir(&o.root.join("site").join("static"), &o.out)?;
    Ok(warnings)
}

/// Copies `site/static` with its subfolders (e.g. the link previews in `og/`) into the bundle.
fn copy_dir(from: &Path, to: &Path) -> Result<(), String> {
    std::fs::create_dir_all(to).map_err(|e| format!("{}: {e}", to.display()))?;
    for e in std::fs::read_dir(from).map_err(|e| format!("{}: {e}", from.display()))?.filter_map(Result::ok) {
        if e.file_name().to_string_lossy().starts_with('.') {
            continue;
        }
        let ziel = to.join(e.file_name());
        if e.path().is_dir() {
            copy_dir(&e.path(), &ziel)?;
        } else {
            std::fs::copy(e.path(), &ziel).map_err(|err| format!("{}: {err}", e.path().display()))?;
        }
    }
    Ok(())
}

/// Course texts for `dx serve` (development): `public/content/…`.
pub fn dev_content(root: &Path) -> Result<Vec<String>, String> {
    let mut warnings = Vec::new();
    let kurse = load_courses(root, &mut warnings)?;
    for k in &kurse {
        let dir = root.join("public").join("content").join(k.info.id);
        write(&dir.join("en.json"), &serde_json::to_string(&k.english).map_err(|e| e.to_string())?)?;
        for (lang, c) in &k.translated {
            write(&dir.join(format!("{lang}.json")), &serde_json::to_string(c).map_err(|e| e.to_string())?)?;
        }
    }
    Ok(warnings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markup_is_escaped() {
        assert_eq!(md("Use `<b>` and **bold** & more"), "Use <code class=\"rounded bg-muted px-1.5 py-0.5 font-mono text-[0.85em]\">&lt;b&gt;</code> and <strong class=\"font-semibold text-foreground\">bold</strong> &amp; more");
        assert_eq!(esc("\"<&>\""), "&quot;&lt;&amp;&gt;&quot;");
        assert_eq!(heute().len(), 10);
    }
}
