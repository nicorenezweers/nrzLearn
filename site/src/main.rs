//! nrzLearn site tool.
//!
//! ```text
//! nrzlearn-site check <course.json>                 validate an English course
//! nrzlearn-site strings <course.json>               write strings/en.json next to it
//! nrzlearn-site check-strings <course.json> <xx.json>   validate a translation
//! nrzlearn-site check-all                           validate every course, translation and UI catalog
//! nrzlearn-site check-lang <code>                   validate everything translated into one language
//! nrzlearn-site build <bundle dir> [--base-url URL]  static pages, course texts, sitemap … into the bundle
//! nrzlearn-site dev-content                         course texts for `dx serve` into public/content
//! ```

mod build;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use nrzlearn_model::{Course, Exercise, LANGUAGES, check_translation, code_spans, strings, validate};

fn read_course(path: &Path) -> Result<Course, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))
}

fn read_map(path: &Path) -> Result<BTreeMap<String, String>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))
}

/// Writes key → text pairs as a JSON object, keeping the given order.
fn write_ordered(path: &Path, pairs: &[(String, String)]) -> Result<(), String> {
    let mut out = String::from("{\n");
    for (i, (k, v)) in pairs.iter().enumerate() {
        let sep = if i + 1 < pairs.len() { "," } else { "" };
        out.push_str(&format!(
            "  {}: {}{sep}\n",
            serde_json::to_string(k).map_err(|e| e.to_string())?,
            serde_json::to_string(v).map_err(|e| e.to_string())?
        ));
    }
    out.push_str("}\n");
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    }
    std::fs::write(path, out).map_err(|e| format!("{}: {e}", path.display()))
}

fn words(s: &str) -> usize {
    s.split_whitespace().count()
}

fn check(path: &Path) -> Result<(), String> {
    let c = read_course(path)?;
    let folder = path.parent().and_then(Path::file_name).and_then(|n| n.to_str()).unwrap_or_default();
    let mut errors = validate(&c);
    if c.lang != "en" {
        errors.push(format!("course: lang must be \"en\" in course.json, not {:?}", c.lang));
    }
    if !folder.is_empty() && folder != c.course {
        errors.push(format!("course: id {:?} must equal the folder name {folder:?}", c.course));
    }
    let mut kinds: BTreeMap<&str, usize> = BTreeMap::new();
    for u in &c.units {
        for e in &u.exercises {
            *kinds.entry(e.kind()).or_default() += 1;
        }
    }
    let texts = strings(&c);
    let total: usize = texts.iter().map(|(_, v)| words(v)).sum();
    let exercises: usize = c.units.iter().map(|u| u.exercises.len()).sum();
    println!("{}: {} units, {exercises} exercises, {} texts, {total} words", c.course, c.units.len(), texts.len());
    println!("  exercise types: {kinds:?}");
    for u in &c.units {
        let outputs = u.lesson.iter().filter(|b| matches!(b, nrzlearn_model::Block::Code { output: Some(_), .. })).count();
        let inputs = u.exercises.iter().filter(|e| matches!(e, Exercise::Input { .. })).count();
        println!("  {:<16} {:<40} {} blocks ({outputs} with output), {} exercises ({inputs} input)", u.id, u.title, u.lesson.len(), u.exercises.len());
    }
    if errors.is_empty() {
        println!("OK");
        Ok(())
    } else {
        Err(errors.join("\n"))
    }
}

fn strings_cmd(path: &Path) -> Result<(), String> {
    let c = read_course(path)?;
    let errors = validate(&c);
    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }
    let out = path.with_file_name("strings").join("en.json");
    write_ordered(&out, &strings(&c))?;
    println!("wrote {}", out.display());
    Ok(())
}

fn check_strings(course: &Path, file: &Path) -> Result<(), String> {
    let c = read_course(course)?;
    let texts = read_map(file)?;
    let errors = check_translation(&strings(&c), &texts);
    if errors.is_empty() {
        println!("{}: OK ({} texts)", file.display(), texts.len());
        Ok(())
    } else {
        Err(errors.into_iter().map(|e| format!("{}: {e}", file.display())).collect::<Vec<_>>().join("\n"))
    }
}

/// Every course, every translation of it, every UI catalog.
/// `allow_missing`: interface texts that are not translated yet are only warnings
/// (the build falls back to English), for releases while translations are running.
fn check_all(root: &Path, allow_missing: bool) -> Result<(), String> {
    let mut errors = Vec::new();
    let courses = root.join("courses");
    let mut dirs: Vec<PathBuf> = std::fs::read_dir(&courses)
        .map_err(|e| format!("{}: {e}", courses.display()))?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.join("course.json").is_file())
        .collect();
    dirs.sort();
    for dir in dirs {
        let file = dir.join("course.json");
        match check(&file) {
            Ok(()) => {}
            Err(e) => errors.push(e),
        }
        let Ok(c) = read_course(&file) else { continue };
        let en = strings(&c);
        if let Ok(rd) = std::fs::read_dir(dir.join("strings")) {
            let mut files: Vec<PathBuf> = rd.filter_map(Result::ok).map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "json")).collect();
            files.sort();
            for f in files {
                if f.file_stem().is_some_and(|s| s == "en") {
                    // Must be up to date with course.json.
                    match read_map(&f) {
                        Ok(m) if m == en.iter().cloned().collect::<BTreeMap<_, _>>() => {}
                        Ok(_) => errors.push(format!("{}: outdated, run `nrzlearn-site strings {}`", f.display(), file.display())),
                        Err(e) => errors.push(e),
                    }
                    continue;
                }
                match read_map(&f) {
                    Ok(m) => errors.extend(check_translation(&en, &m).into_iter().map(|e| format!("{}: {e}", f.display()))),
                    Err(e) => errors.push(e),
                }
            }
        }
    }
    let mut fehlend = 0;
    for e in check_i18n(root) {
        if allow_missing && e.ends_with(": missing") {
            fehlend += 1;
        } else {
            errors.push(e);
        }
    }
    if fehlend > 0 {
        eprintln!("warning: {fehlend} interface texts are not translated yet (English is used)");
    }
    if errors.is_empty() {
        println!("all OK");
        Ok(())
    } else {
        Err(errors.join("\n"))
    }
}

/// The UI catalog and every course translation of one language.
fn check_lang(root: &Path, code: &str) -> Result<(), String> {
    if !LANGUAGES.iter().any(|l| l.code == code) || code == "en" {
        return Err(format!("unknown or source language {code:?}"));
    }
    let mut errors: Vec<String> = check_i18n(root).into_iter().filter(|e| e.contains(&format!("i18n/{code}.json"))).collect();
    let mut fertig = 0;
    let courses = root.join("courses");
    let mut dirs: Vec<PathBuf> = std::fs::read_dir(&courses).map_err(|e| e.to_string())?.filter_map(Result::ok).map(|e| e.path()).filter(|p| p.join("course.json").is_file()).collect();
    dirs.sort();
    for dir in dirs {
        let file = dir.join("strings").join(format!("{code}.json"));
        let c = read_course(&dir.join("course.json"))?;
        match read_map(&file) {
            Ok(m) => {
                let e = check_translation(&strings(&c), &m);
                if e.is_empty() {
                    fertig += 1;
                }
                errors.extend(e.into_iter().map(|x| format!("{}: {x}", file.display())));
            }
            Err(_) => errors.push(format!("{}: missing", file.display())),
        }
    }
    if errors.is_empty() {
        println!("{code}: OK (UI catalog and {fertig} courses)");
        Ok(())
    } else {
        Err(errors.join("\n"))
    }
}

/// `{name}` placeholders of a text, sorted.
fn platzhalter(s: &str) -> Vec<String> {
    let mut v: Vec<String> = s.split('{').skip(1).filter_map(|r| r.split_once('}').map(|(n, _)| n.to_string())).filter(|n| !n.is_empty() && n.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')).collect();
    v.sort();
    v
}

/// The UI catalogs: same keys as English, same placeholders and code.
fn check_i18n(root: &Path) -> Vec<String> {
    let mut errors = Vec::new();
    let dir = root.join("i18n");
    let en = match read_map(&dir.join("en.json")) {
        Ok(m) => m,
        Err(e) => return vec![e],
    };
    for l in LANGUAGES.iter().filter(|l| l.code != "en") {
        let file = dir.join(format!("{}.json", l.code));
        let Ok(m) = read_map(&file) else {
            errors.push(format!("{}: missing", file.display()));
            continue;
        };
        for k in m.keys().filter(|k| !en.contains_key(*k)) {
            errors.push(format!("{}: {k}: unknown key", file.display()));
        }
        for (k, v_en) in &en {
            let Some(v) = m.get(k) else {
                errors.push(format!("{}: {k}: missing", file.display()));
                continue;
            };
            if v.trim().is_empty() && !v_en.trim().is_empty() {
                errors.push(format!("{}: {k}: empty", file.display()));
            }
            if platzhalter(v) != platzhalter(v_en) {
                errors.push(format!("{}: {k}: placeholders {:?} instead of {:?}", file.display(), platzhalter(v), platzhalter(v_en)));
            }
            if code_spans(v) != code_spans(v_en) {
                errors.push(format!("{}: {k}: `code` differs from English", file.display()));
            }
            if v_en.contains('|') && !v.contains('|') {
                errors.push(format!("{}: {k}: alternatives must stay separated by |", file.display()));
            }
        }
    }
    errors
}

/// The repository the tool works on: the current directory or one above it
/// with `courses/` and `i18n/` (so a prebuilt binary works in any checkout).
fn repo_root() -> PathBuf {
    let cwd = std::env::current_dir().unwrap_or_default();
    cwd.ancestors()
        .find(|d| d.join("courses").is_dir() && d.join("i18n").is_dir())
        .map_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(".."), Path::to_path_buf)
}

fn usage() -> ExitCode {
    eprintln!("usage: nrzlearn-site check <course.json> | strings <course.json> | check-strings <course.json> <xx.json> | check-all [--allow-missing] | check-lang <code> | build <bundle dir> [--base-url URL] | dev-content | courses-json");
    ExitCode::FAILURE
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let root = repo_root();
    let result = match args.iter().map(String::as_str).collect::<Vec<_>>().as_slice() {
        ["check", course] => check(Path::new(course)),
        ["strings", course] => strings_cmd(Path::new(course)),
        ["check-strings", course, file] => check_strings(Path::new(course), Path::new(file)),
        ["check-all"] => check_all(&root, false),
        ["check-all", "--allow-missing"] => check_all(&root, true),
        ["check-lang", code] => check_lang(&root, code),
        ["build", out, rest @ ..] => {
            let base = match rest {
                ["--base-url", url] => url.trim_end_matches('/').to_string(),
                [] => std::env::var("NRZLEARN_BASE_URL").unwrap_or_else(|_| "https://learn.zweers.ch".into()).trim_end_matches('/').to_string(),
                _ => return usage(),
            };
            build::build(&build::Options { root: root.clone(), out: PathBuf::from(out), base }).map(|warnings| {
                for w in warnings {
                    eprintln!("warning: {w}");
                }
                println!("built static pages into {out}");
            })
        }
        ["courses-json"] => {
            // Course list with colours and logos, e.g. for scripts/og-images.mjs.
            let list: Vec<serde_json::Value> = nrzlearn_model::COURSES
                .iter()
                .map(|c| {
                    let icon = nrzlearn_model::icons::icon(c.id);
                    serde_json::json!({"id": c.id, "name": c.name, "color": c.color, "on_color": c.on_color, "group": c.group,
                        "view_box": icon.as_ref().map(|i| i.view_box), "body": icon.as_ref().map(|i| i.body)})
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&list).unwrap_or_default());
            Ok(())
        }
        ["dev-content"] => build::dev_content(&root).map(|warnings| {
            for w in warnings {
                eprintln!("warning: {w}");
            }
            println!("wrote public/content");
        }),
        _ => return usage(),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}
