# Translating nrzLearn

nrzLearn is translated into 40 languages (list: `LANGUAGES` in
[`model/src/lib.rs`](../model/src/lib.rs)). English is the source. Two kinds of files:

| File | What | Keys |
|---|---|---|
| `i18n/<lang>.json` | Interface (`app.*`) and the static pages for search engines (`site.*`) | all keys of `i18n/en.json` |
| `courses/<course>/strings/<lang>.json` | Texts of one course (lessons, exercises, cheat sheet) | all keys of `courses/<course>/strings/en.json` |

Both are flat JSON objects `"key": "text"`. A translation has **exactly the same
keys** as the English file, in any order, every value translated. Never add,
remove or rename keys. Never edit `en.json`, `course.json` or any code.

```bash
export RUSTUP_TOOLCHAIN=stable              # if rustup has no default toolchain
cargo run -q -p nrzlearn-site -- check-lang fr    # checks i18n/fr.json and every courses/*/strings/fr.json
```

## Hard rules (the checker enforces them)

1. **Code spans stay identical.** Everything between backticks is code:
   `` `print(x)` ``, `` `for` ``, `` `"Hello"` ``, `` `3 + 4` ``. Copy every code span
   character for character, with its backticks. Do not translate, reformat or
   remove it, and do not add new ones. You may move it inside the sentence.
2. **Placeholders stay identical:** `{n}`, `{course}`, `{title}`, `{levels}` …
   Keep each one exactly once (as in English), you may move it.
3. **`**bold**`** markers: keep them around the translation of the bolded words.
4. **`|` separates alternatives** (`app.praise`, `app.consolation`): keep the
   separators; the number of alternatives may differ.
5. One line per text: no line breaks, no leading or trailing spaces, no HTML,
   no `<` outside code spans.

## Style

- Audience: beginners learning to program. Friendly, clear, short sentences.
- Address the learner the way modern learning apps in your language do (see the
  table). Be consistent across all files.
- Keep brand and product names: **nrzLearn**, the names of the programming
  languages (Rust, Python, Java, PHP, Haskell, COBOL, Solidity …), Rust
  Playground, Ferris, Cargo, XP. Exception: the course names *Machine code*
  (`app.course.machine-code`) and *Assembly* (`app.course.assembly`) are
  translated with the term people in your language search for (German
  *Maschinencode*, *Assembler*).
- Programming terms: use the term that is common in programming tutorials in
  your language. If developers in your language normally use the English word
  (often for "string", "array", "slice", "struct", "trait", "closure"), use it
  (adapted to grammar where that is normal). Translate the explanation around it.
- Use the same translation for a term everywhere (all courses and the interface).
  Core terms of the app: *world* (a unit of the learning path), *level*,
  *lesson*, *exercise*, *exam*, *master exam*, *cheat sheet*, *card*, *streak*,
  *daily goal*, *learning path*.
- Interface texts (`app.*`) are often buttons or labels: keep them about as short
  as the English text.
- Use your language's typography: quotation marks („…“, «…», «…», 「…」 …),
  number formats in prose, punctuation and spacing rules.
- `site.*` texts are for search engines and the landing page: natural,
  idiomatic phrases that people in your language would search for (for example
  "apprendre Python", "Python lernen"). Keep `site.meta_title` under about 65
  characters and `site.meta_description` under about 160 characters.
- Exercise prompts must stay unambiguous; explanations must stay correct. Do not
  change meaning, add information or drop details.
- Do not translate the German legal notice (it is not in these files).
- `site.impressum_note`: the imprint page shows the German legal notice and
  below it a short version **in English** in every language, so say "English"
  there, not your language.

| Language | Address | Notes |
|---|---|---|
| Deutsch | du | |
| Español | tú | |
| Français | tu | French typography (espace avant ? ! : ;) |
| Italiano | tu | |
| Português | você | Brazilian Portuguese, avoid strongly regional words |
| Nederlands | je/jij | |
| Polski, Čeština, Slovenčina, Slovenščina, Hrvatski, Български, Українська, Русский, Српски (Cyrillic), Македонски | 2nd person singular (ty / ти …) | |
| Magyar | te | |
| Română | tu | with ș/ț (comma below) |
| Ελληνικά | εσύ | |
| Dansk, Svenska, Norsk bokmål, Íslenska | du / þú | |
| Suomi, Eesti, Latviešu, Lietuvių | sinä / sina / tu / tu | |
| Gaeilge | tú | Caighdeán Oifigiúil |
| Malti | int | standard Maltese |
| Türkçe | sen | |
| Català | tu | |
| Shqip | ti | |
| 日本語 | です・ます | |
| 한국어 | 해요체 | |
| 简体中文 | 你 | Simplified Chinese |
| हिन्दी | आप | Devanagari |
| العربية | neutral where possible, else masculine singular | Modern Standard Arabic |
| Bahasa Indonesia | kamu | |
| Tiếng Việt | bạn | |

## Texts with placeholders: grammar for every value

The app inserts values into placeholders as they are, without plural rules, noun
cases or particles. A text must therefore be correct for **every value** it can
get. Check each key below with all listed values and choose a wording that fits
all of them: plural forms (Polish 4 *poziomy* but 5 *poziomów*; Romanian *de*
from 20: 34 *de* niveluri; Slovenian dual; Arabic plural for 3–10 but singular
from 11; Irish mutations after numbers; Baltic languages …), noun cases (*Naucz się Pythona*), particles and suffixes
that depend on the sound of the word before (Korean 을/를, Turkish *'u/'yu*,
Hungarian *a/az*, Finnish case endings …), gender and articles.

Where a form cannot fit all values, rephrase: put the noun in front as a label
("Levels: {n}", "Liczba poziomów: {n}"), use a construction that needs no
agreement ("Kurs {course}", "programming language {course}", "{course} 배우기"),
or a fixed form that is correct for all listed numbers. Keep it natural and
short; do not use mechanical forms like "level(s)" if a natural wording exists.

| Keys | Values |
|---|---|
| `site.course_h1`, `site.course_meta_title`, `site.course_meta_description`, `site.course_start` | `{course}`: Rust, Python, TypeScript, Go, JavaScript, C++, C#, Kotlin, SQL, Swift |
| `site.course_meta_description` | `{worlds}` 8 or 16, `{levels}` 34 or 100 |
| `site.course_stats` | `{worlds}` 8, `{levels}` 34, `{exercises}` 96 |
| `site.course_stats_rust` | `{worlds}` 16, `{levels}` 100 |
| `site.levels_count`, `app.end_of_map_text` | `{n}`/`{levels}` 34 or 100, `{worlds}` 8 or 16 |
| `app.cards_collected`, `app.cheatsheet_intro` | `{total}` 8 or 16, `{n}` 0 to `{total}` |
| `app.levels_done_of` | `{total}` 4 to 7, `{done}` 0 to `{total}` |
| `app.level.desc_practice_1` / `_2` / `_3` | `{n}` 7 or 8 / 8 or 9 / 10 |
| `app.level.desc_exam`, `app.level.desc_master` | `{n}` 10 / 12; `{from}`–`{to}`: 1–4, 5–8, 9–12, 13–16 |
| `app.n_in_a_row` | 3, 5, 10, 15, 20, 30 |
| `app.new_record` | 2 or 3 (stars) |
| `app.daily_goal_title` | `{done}` 0 or more, `{goal}` 30, 60 or 120 |
| `app.daily_streak_badge`, `app.wrong_tries` | 1 or more |
| `app.level_up` | 2 or more |
| `app.start.levels_done`, `app.replace_compare`, `app.xp_total`, `app.xp_to_level` | any number from 0 (`{xp}` in `app.xp_to_level` from 1) |
| `app.continue_level`, `app.finish_first`, `app.level_locked`, `app.level_n`, `app.level_n_title`, `app.next_level`, `app.unlock_world`, `app.world_level_learn`, `app.world_n_chapter`, `app.line_n`, `site.world_n` | numbers used as names (level 1–100, world 1–16, line 1–30): check particles and endings after them |
