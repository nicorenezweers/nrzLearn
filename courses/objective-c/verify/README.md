# Verifying the Objective-C course

`verify.py` compiles and runs the course code with the local Apple clang
(`clang -fobjc-arc -Wall -framework Foundation`, macOS only). Only Python 3 and the
Xcode command line tools are needed.

```bash
python3 courses/objective-c/verify/verify.py              # all units
python3 courses/objective-c/verify/verify.py classes      # only some units
```

What it checks:

- **Lesson code blocks**: each one is a complete program; it must compile without
  warnings, and its stdout must equal `output` exactly.
- **Exercises** with an entry in `checks.json` (keyed by exercise id):
  - `input`, `choice`, `true_false`: the exercise `code` (or `code` from the check)
    is run; stdout must equal `out` (for `input` it must also match one of the
    `answers`), or with `"err": true` it must fail to compile or crash.
  - `gap`: the right option must print `out`; every wrong option must fail or print
    something else.
  - `bug`: the code as shown must fail or print something else; with the line
    replaced by `fix` it must print `out`.
  - `order`: the lines in the given order must print `out`.
  - `blocks`: the tokens joined with spaces must print `out`; with
    `"distractors": true`, every distractor in every position must not.
  - `extra`: additional complete programs (`src`) with `out` or `"err": true`, used
    for exercises without code (facts stated in options or statements).
- Right answers must compile without warnings unless the check has `"warn": true`.

Snippets are put into a program: as they are when they contain `int main(`;
with `tpl` (`{{CODE}}` marks the place); with `main` (the snippet is top-level
code such as a class, followed by `main` with that body); otherwise inside
`main`'s `@autoreleasepool`, with `before`/`after` lines around it and `pre`
(class definitions) before `main`. Programs run with a timeout of 5 seconds and
at most 100 kB of output.

Results are cached in `.cache.json` (not committed), keyed by the SHA-256 of the
clang version and the program.

The `pairs` exercises and the knowledge questions `hello-1` and `classes-1` have
no check; review them by reading.
