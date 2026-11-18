# Verifying the Odin course

`verify.py` runs the course code on Compiler Explorer (godbolt.org, compiler
`odin202611a` = Odin `dev-2026-11`). Only Python 3 and network access are needed;
nothing is installed locally.

```bash
python3 courses/odin/verify/verify.py                 # all units
python3 courses/odin/verify/verify.py loops maps      # only some units
python3 courses/odin/verify/verify.py --run file.odin # run one program
```

Requests are sent one at a time with a pause of about one second. Results are
cached in `.cache.json` (not committed), keyed by compiler and the SHA-256 of the
program, so a second run is fast. A program "fails" when it does not compile or
exits with a code other than 0. Only stdout is compared.

What it checks:

- **Lesson code blocks**: each block with an `output` must be a complete program
  (`package main`); its stdout must equal `output` exactly.
- **Exercises** with an entry in `checks.json` (keyed by exercise id):
  - `input`, `choice`, `true_false`: the exercise `code` (or `code` from the check)
    is run; the output must equal `out` (for `input` it must also match one of the
    `answers`), or with `"err": true` the program must fail.
  - `gap`: the right option must print `out`; every wrong option must fail or print
    something else.
  - `bug`: the lines as shown must fail or print something else; with the bug line
    replaced by `fix` they must print `out`.
  - `order`: the lines in the given order must print `out`; every swap of two
    neighbouring lines must fail or print something else.
  - `blocks`: the tokens joined with spaces must print `out`.
  - `extra`: additional programs (`src`, optionally with their own `tpl`) with
    `out` or `"err": true`. They check facts stated in options, statements and
    lesson texts (for example that `'Hi'` is an error, or that a missing enum case
    in a `switch` is an error).

Snippets are put into a template: `tpl` from the check (`{{CODE}}` is the marker,
indented like the marker), or by default `package main` with `import "core:fmt"`
and `import "core:strings"` plus either the code as it is (when it contains
`main :: proc`) or the code inside `main :: proc() { }`. Unused imports are allowed
in Odin, so the extra import does not change any result.

Not run (review by reading): the eight `pairs` exercises and `maps-4` (the order
of a map loop is not fixed, which cannot be shown by one run).
