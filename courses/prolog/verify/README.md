# Verifying the Prolog course

`verify.py` runs the course code with a local `swipl` (SWI-Prolog 9 or newer;
checked with SWI-Prolog 10.0.2). Wandbox and Compiler Explorer cannot run
Prolog, so the code runs locally (`brew install swi-prolog`). Only Python 3 and
SWI-Prolog are needed. Set `SWIPL` to use another binary.

```bash
python3 courses/prolog/verify/verify.py               # all units
python3 courses/prolog/verify/verify.py lists rules   # only some units
```

What it checks:

- **Lesson code blocks**: each block with `output` is a complete program
  (`:- initialization(main, main).`); stdout must equal `output` exactly and
  stderr must be empty (no warnings). Blocks without `output` are loaded with
  `swipl -g halt` and must print nothing.
- **Exercises** with an entry in `checks.json` (keyed by exercise id):
  - `input`, `choice`, `true_false`: the exercise `code` is run; stdout must equal
    `out` (for `input` it must also match one of the `answers`; with `"opt": true`
    the right option must equal the output and no other option may), or with
    `"err": true` it must fail, or with `"hang": true` it must not terminate
    within 5 seconds.
  - `gap`: the right option must print `out`; every wrong option must not.
  - `bug`: the code as shown must not print `out`; with the bug line replaced by
    `fix` it must print `out`.
  - `order`: the lines in the given order must print `out`, and **no other
    permutation** may print `out`.
  - `blocks`: the tokens joined with spaces must print `out`.
  - `extra`: additional programs (`src`) with `out`, `not` (must not print
    this), `err`, `hang` or `stderr` (text that must appear on stderr), used for
    facts stated in options or statements.
- Code lint: no tabs, no trailing spaces, no `%` comments.

Programs are built like this: a `tpl` from the check (with `{{CODE}}` as the
marker, used to add facts or a `main`) is filled first; then the directive
`:- initialization(main, main).` is added at the top when the program has none
(not with `"raw": true`). Every run uses stdin from `/dev/null`.

Results are cached in `.cache.json` (not committed), keyed by the SHA-256 of the
SWI-Prolog version and the program.

Not checked by running: the `pairs` exercises, `rules-1` (how to read `:-`) and
`recursion-2` (which clause is the base case); review them by reading.
