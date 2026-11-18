# Verifying the Common Lisp course

`verify.py` runs the course code with SBCL and checks it. Only Python 3 is needed,
plus a local `sbcl` (e.g. `brew install sbcl`) or internet access (Wandbox).

```bash
python3 courses/common-lisp/verify/verify.py                 # local `sbcl --script`
python3 courses/common-lisp/verify/verify.py --wandbox       # Wandbox, sbcl-2.4.9
python3 courses/common-lisp/verify/verify.py lists mapping   # only some units
```

A program counts as successful only if `sbcl --script` exits with code 0 and
writes nothing to stderr (so compiler warnings, such as an undefined variable,
count as failures). Output comparisons accept the expected text with or without
one final newline (`"exact": true` turns this off, used for `print`).

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
    other permutation whose parentheses balance is run and must fail or print
    something else, so the order is unique.
  - `blocks`: the tokens joined with spaces must print `out`.
  - `extra`: additional programs (`src`) for facts stated in options or statements
    (e.g. that `(2 + 3)` is an error or that `0` counts as true).
- **Lesson claims** (`_claims`, per unit): statements in texts, tips and warnings,
  e.g. that `(= "a" "a")` is an error, that `*Price*` and `*price*` are the same
  variable, or that a `let` variable does not exist outside the `let`.

`tpl` (with the marker `{{CODE}}`) wraps code that needs surrounding lines.

Results are cached in `.cache.json` (not committed), keyed by backend and the
SHA-256 of the program.

Not checked by code (review by reading): the `pairs` exercises.

Last full run: SBCL 2.6.10 (Homebrew, local), because Wandbox returned internal
errors at the time: 38 lesson blocks, 88 exercises, 14 lesson claims,
348 programs, all passing.
