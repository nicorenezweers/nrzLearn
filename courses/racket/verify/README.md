# Verifying the Racket course

`verify.py` runs the course code with a local `racket` and checks it. Only Python 3
and Racket are needed (`brew install minimal-racket`, about 100 MB; Wandbox and
Compiler Explorer have no Racket).

```bash
python3 courses/racket/verify/verify.py                 # all units
python3 courses/racket/verify/verify.py lists higher    # only some units
```

Every program is written to `main.rkt` and run with `racket main.rkt`. Programs
without a `#lang` line (the exercise snippets) get `#lang racket` in front. A
program counts as successful only if it exits with code 0 and writes nothing to
stderr. Output comparisons accept the expected text with or without one final
newline (`"exact": true` turns this off).

What it checks:

- **Lesson code blocks**: each one runs cleanly and its stdout equals `output`.
- **Exercises** with an entry in `checks.json` (keyed by exercise id):
  - `input`, `choice`, `true_false`: the exercise `code` (or `src`) is run; stdout
    must equal `out` (for `input`: by default the first answer, and the output must
    match one of the `answers` under the app's comparison rules), or with
    `"err": true` it must fail.
  - `gap`: the right option must print `out`; every wrong option must fail or
    print something else.
  - `bug`: the code as shown must fail or print something else; with the bug line
    replaced by `fix` it must print `out`.
  - `order`: the lines in the given order must print `out`. In addition, every
    other permutation whose parentheses and brackets balance is run and must fail
    or print something else, so the order is unique.
  - `blocks`: the tokens joined with spaces must print `out`.
  - `extra`: additional programs (`src`) for facts stated in options or statements
    (e.g. that `(3 * 4)` is an error, or what each option of a choice prints).
- **Lesson claims** (`_claims`, per unit): statements in texts, tips and warnings,
  e.g. that `if` without an else part is an error, that `(if "" ...)` takes the
  then part, or that an opaque struct prints as `#<point>`.

`tpl` (with the marker `{{CODE}}`) wraps code that needs surrounding lines.

Results are cached in `.cache.json` (not committed), keyed by the Racket version
and the SHA-256 of the program.

Not checked by code (review by reading): the `pairs` exercises and `hello-2`
(which line comes first in a program).

Last full run: Racket v9.3 [cs] (Homebrew `minimal-racket`, local): 41 lesson
blocks, 87 exercises, 18 lesson claims, 380 programs, all passing. The course
avoids features that differ between Racket 8.x and 9.x.
