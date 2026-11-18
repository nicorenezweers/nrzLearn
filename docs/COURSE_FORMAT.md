# Course format

Every course except Rust (which has its own generators in `src/`) is a JSON
file: `courses/<id>/course.json`, written in **English**. Translations never
copy the course: they only translate its texts (`courses/<id>/strings/<lang>.json`,
see [TRANSLATING.md](TRANSLATING.md)). The data model lives in
[`model/src/lib.rs`](../model/src/lib.rs); `nrzlearn-site` checks the rules below.

```bash
export RUSTUP_TOOLCHAIN=stable              # if rustup has no default toolchain
cargo run -q -p nrzlearn-site -- check courses/python/course.json
cargo run -q -p nrzlearn-site -- strings courses/python/course.json   # writes courses/python/strings/en.json
```

## Structure

```json
{
  "course": "python",
  "lang": "en",
  "units": [
    {
      "id": "variables",
      "title": "Variables and types",
      "chapter": "Basics",
      "summary": "Store values in variables and get to know int, float, str and bool.",
      "lesson": [ { "type": "text", "md": "…" }, { "type": "code", "code": "…", "output": "…" } ],
      "card": [ "`x = 5` creates a variable", "`type(x)` shows the type" ],
      "exercises": [ { "type": "choice", "id": "variables-1", … } ]
    }
  ]
}
```

- `course`: the course ID, equal to the folder name and to an entry of `COURSES`
  in `model/src/lib.rs` (`python`, `java`, `cobol`, `machine-code` …).
- `lang`: always `"en"` in `course.json`.
- `units`: **8 units** (allowed 6–12), from the very first program to the
  fundamentals a beginner needs. Order = learning order; later units may use
  everything from earlier ones, never the other way round.

### Unit

| Field | Rules |
|---|---|
| `id` | Stable, lowercase `a-z`, `0-9`, `-` (e.g. `loops`). Never change it later: progress is stored under it. |
| `title` | ≤ 40 characters, e.g. "Loops". |
| `chapter` | ≤ 24 characters, a group label shown above the title, e.g. "Basics", "Control flow", "Data". |
| `summary` | One sentence, ≤ 140 characters. Also used on the SEO pages. |
| `lesson` | The theory: 6–14 blocks (≥ 2 `text`, ≥ 2 `code`). |
| `card` | Cheat sheet card: 2–5 short points, mostly code in backticks. |
| `exercises` | **12** exercises (allowed 10–16), at least **5 different types**. |

### Lesson blocks

```json
{ "type": "heading", "text": "Counting with range()" }
{ "type": "text", "md": "A `for` loop repeats code for every item. **Indentation** matters." }
{ "type": "code", "code": "for i in range(3):\n    print(i)", "output": "0\n1\n2" }
{ "type": "tip", "md": "…" }
{ "type": "warning", "md": "…" }
```

- Texts (`md`, `text`) are **one paragraph** each, no line breaks, no HTML. Inline
  markup: `` `code` `` and `**bold**`, nothing else. Every code element inside a
  sentence goes into backticks: names, keywords, operators, values, outputs.
- `code`: a complete, runnable program or snippet (see the language notes
  below), spaces not tabs, no trailing spaces, no empty first or last line.
  `\n` separates lines.
- `output`: the **exact** stdout of running the code (without the final
  newline). Leave `output` out only when the code prints nothing.
- **No natural-language comments in code.** Code is never translated, so a
  comment like `# add the numbers` would stay English in 40 languages. Explain in
  the texts instead. Allowed: comments that only contain code or results, e.g.
  `// 5`, `# -> [1, 2]`, and the bare syntax when a unit teaches comments.
- Names in code are English (`name`, `total`, `prices`), short and meaningful.
  String values in code may be English words (`"Hello"`, `"apple"`).

### Exercises

Every exercise has an `id` of the form `<unit id>-<n>` (`loops-1` … `loops-12`)
and, except `pairs`, an `explain`: one or two short sentences that say **why**
the answer is right. It is shown after every answer, right or wrong.

| `type` | Fields | Notes |
|---|---|---|
| `choice` | `prompt`, `code`?, `options` (3–4), `answer` (index), `explain` | Options are texts; code in options goes into backticks, e.g. `` "`len(x)`" ``. |
| `input` | `prompt`, `code`?, `answers` (1+), `explain` | Answers are **not translated**: outputs, numbers, code, values. Comparison ignores case, spaces, `_`, quotes and backticks. List every accepted spelling. |
| `true_false` | `statement`, `code`?, `answer` (bool), `explain` | A statement the learner judges. |
| `gap` | `prompt`, `code` with exactly one `___`, `options` (3–4, code, not translated), `answer`, `explain` | The right option completes the code. |
| `blocks` | `prompt`, `tokens` (3–10, the right order), `distractors` (1–4), `explain` | One line of code split into blocks; displayed shuffled, joined with spaces. No token with outer spaces; distractors must not equal a token. |
| `order` | `prompt`, `lines` (3–8, the right order, all different), `explain` | The lines are shown shuffled. Use leading spaces for indentation. The order must be unambiguous. |
| `pairs` | `prompt`, `pairs` (3–5 × `[code, meaning]`) | Left side is code (not translated, use backticks), right side a short translated meaning. All left sides different, all right sides different. |
| `bug` | `prompt`, `lines` (3–12), `bug` (0-based index), `explain` | Exactly **one** line is wrong; all others are correct. The explanation names the fix. |

Rules for good exercises:

- Every exercise is answerable from this unit and the units before it.
- Mix the types; per unit at least 3 exercises that read code (`input` with the
  output, `choice` with `code`, `bug`) and at least 2 that write code (`gap`,
  `blocks`, `order`).
- Wrong options are plausible mistakes, never jokes. Exactly one option is right.
- Prompts are short questions or instructions ("What does this print?",
  "Which line has the bug?", "Put the lines in order so the program prints 6.").
- Do not reveal the answer in the prompt or the code.
- `input` exercises need a single unambiguous answer, ideally the output of a
  short `code`.
- Check every answer by **running the code**. If an answer depends on the
  version of the language, choose another example.

## Style

- Audience: complete beginners who may have never programmed, and people who
  know another language. Friendly, short sentences, second person ("you"),
  present tense, no slang, no jokes that do not translate.
- One idea per paragraph; show code right after the idea it illustrates.
- Use modern, idiomatic code of the current stable version (see notes).
- Plain, international English. No idioms ("piece of cake"), no puns, no
  culture-specific references. Avoid the words "simply" and "just".

## Language notes

| Course | Version and how code is run | Notes |
|---|---|---|
| `python` | Python 3.12+ | f-strings, no `input()` (no stdin). |
| `typescript` | TypeScript 5, run with Deno or `node` (type stripping) | `console.log`, `let`/`const`, strict types. Outputs are what `console.log` prints in Node. |
| `go` | Go 1.22+, every program is `package main` with `func main()` | `fmt.Println`. Snippets in exercises may omit the boilerplate when obvious; lesson code blocks with `output` must be complete programs. |
| `javascript` | Modern JS (ES2023), run with `node` | `console.log` output as printed by Node (`[ 1, 2, 3 ]`, `{ a: 1 }`). |
| `cpp` | C++20, `#include <iostream>`, `std::cout`, `int main()` | Lesson code blocks with `output` are complete programs. Prefer `std::string`, `std::vector`, range-for. |
| `csharp` | C# 12 / .NET 8, top-level statements | `Console.WriteLine`, string interpolation. |
| `kotlin` | Kotlin 2, `fun main()` | `println`, `val`/`var`, string templates. |
| `sql` | SQLite 3 (standard SQL where possible) | Unit 1 introduces a small sample database (`CREATE TABLE` + `INSERT`) that the whole course uses. Outputs are tables as printed by `sqlite3` with `.mode table` (ASCII). |
| `swift` | Swift 5.9+/6, top-level code | `print`, `let`/`var`, string interpolation. |

## Checklist before committing

1. `cargo run -q -p nrzlearn-site -- check courses/<id>/course.json` prints `OK`.
2. Every code block with `output` was run and the output pasted exactly.
3. Every exercise answer was checked by running code where possible.
4. `cargo run -q -p nrzlearn-site -- strings courses/<id>/course.json` wrote
   `courses/<id>/strings/en.json`.
