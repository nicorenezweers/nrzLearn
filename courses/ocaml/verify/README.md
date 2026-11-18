# Verifying the OCaml course

`verify.py` runs the course code on the Compiler Explorer API (`ocaml5502`,
x86-64 ocamlopt 5.5.2), because OCaml is not installed locally. Only Python 3 is
needed.

```bash
python3 courses/ocaml/verify/verify.py              # all units
python3 courses/ocaml/verify/verify.py lists        # only some units
```

What it checks:

- **Lesson code blocks**: each one is a complete program (`let () = ...`); it must
  compile without warnings, and its stdout must equal `output` exactly (blocks
  without `output` must print nothing).
- **Exercises** with an entry in `checks.json` (keyed by exercise id):
  - `input`, `choice`, `true_false`: the exercise `code` is run; stdout must equal
    `out` (for `input` it must also match one of the `answers`), or with
    `"err": true` it must fail to compile or crash.
  - `gap`: the right option must print `out`; every wrong option must fail or print
    something else.
  - `bug`: the code as shown must fail or print something else; with the bug line
    replaced by `fix` it must print `out`.
  - `order`: the lines in the given order must print `out`.
  - `blocks`: the tokens joined with spaces must print `out`; with
    `"distractors": true`, no distractor may replace a token and still print `out`.
  - `extra`: additional complete programs (`src`) with `out` or `"err": true`, used
    for exercises without code (facts stated in options or statements). For
    `choice` exercises, there is usually one program per option.
- **Style**: no tabs, no trailing spaces and no comments in any code.

Exercise code is a complete program. When it is only a part (`blocks`), `tpl`
in the check holds the program, with `{{CODE}}` as the marker (indented like the
marker). Programs that should work must also compile without warnings.

Results are cached in `.cache.json` (not committed), keyed by the SHA-256 of the
program, so reruns only send changed programs. Requests are sent one at a time
with a pause of one second.

The `pairs` exercises have no check; review them by reading.
