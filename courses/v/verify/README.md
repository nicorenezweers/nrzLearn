# Verifying the V course

`verify.py` runs the course code with the local V compiler (`v run`, written for
V 0.5.2). Only Python 3 and `v` are needed (for example `brew install vlang`;
Compiler Explorer has V only without execution, Wandbox has no V).

```bash
python3 courses/v/verify/verify.py                 # all units
python3 courses/v/verify/verify.py loops structs   # only some units
python3 courses/v/verify/verify.py --nofmt         # skip the `v fmt` check
```

A program "works" when it compiles **without any compiler message** (no warning,
no notice) and exits with 0. A program "fails" when it does not compile, panics
(for example `index out of range`), or runs longer than 20 seconds.

What it checks:

- **Lesson code blocks**: each one is a complete program; its stdout must equal
  `output` exactly.
- **Exercises** with an entry in `checks.json` (keyed by exercise id):
  - `input`, `choice`, `true_false`: the exercise `code` (or `code` from the check)
    is run; the output must equal `out` (for `input` it must also match one of the
    `answers`), or with `"err": true` the program must fail.
  - `gap`: the right option must print `out`; every wrong option must fail, warn
    or print something else.
  - `bug`: the lines as shown must fail or print something else; with the bug line
    replaced by `fix` they must print `out`.
  - `order`: the lines in the given order must print `out`; every other order must
    fail or print something else (all permutations up to 5 lines, every swap of
    two lines for longer exercises).
  - `blocks`: the tokens joined with spaces must print `out`.
  - `extra`: additional complete programs (`src`) with `out` or `"err": true`, used
    for facts stated in options or statements.
- **`_facts`** in `checks.json`: claims in the lesson texts that have no output
  (compile errors, warnings, panics), each with `out`, `"err": true` or
  `"warn": true` (compiles, but with a warning).
- **v fmt**: every lesson block and every correct exercise program must already be
  formatted, with tabs shown as 4 spaces (`"nofmt": true` skips this, for example
  for bug exercises whose lines have no empty line between functions).

Snippets without `fn main()` are put inside `fn main() { … }`, or into `tpl` from
the check (`{{CODE}}` is the marker, indented like the marker).

Results are cached in `.cache.json` (not committed), keyed by V version and the
SHA-256 of the program.

The `pairs` exercises and the knowledge question `hello-1` have no check; review
them by reading.
