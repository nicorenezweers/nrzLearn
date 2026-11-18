# Verifying the Gleam course

`verify.py` compiles and runs the course code with a local `gleam` (checked with
Gleam 1.19.2, `gleam_stdlib` 1.x, Node.js for the JavaScript target and Erlang/OTP for the
Erlang target). Install with `brew install gleam` (it pulls in Erlang and rebar3); the
first run downloads `gleam_stdlib` from Hex. Only Python 3 and `gleam` are needed.

```bash
python3 courses/gleam/verify/verify.py              # all units
python3 courses/gleam/verify/verify.py lists        # only some units
```

Every program is written to `src/proj.gleam` of a small scratch project in a temporary
folder and run with `gleam run --target <target> --no-print-progress`. Several project
copies run in parallel.

What it checks:

- **Lesson code blocks**: each one runs on **both targets** (JavaScript and Erlang);
  exit code 0, no compiler warnings, and stdout must equal `output` exactly.
- **Exercises** with an entry in `checks.json` (keyed by exercise id):
  - `input`, `choice`, `true_false`: the exercise `code` (or `code` from the check) runs
    on both targets; stdout must equal `out` (for `input` it must also match one of the
    `answers`), or with `"err": true` it must fail.
  - `gap`: the right option must print `out` (both targets); every wrong option must
    fail or print something else.
  - `bug`: the code as shown must fail or print something else; with the bug line
    replaced by `fix` it must print `out` (both targets, no warnings).
  - `order`: the lines in the given order must print `out`, and no other order may
    print `out`. Orders that cannot be formatted Gleam (brackets closing below zero or
    indentation that does not match the nesting) are skipped; all others are run.
  - `blocks`: the tokens joined with spaces must print `out`; no distractor may replace
    any token and still print `out`.
  - `extra`: additional programs (`src`) with `out` or `"err": true`, used for facts in
    options or statements without code.
- **Formatting**: every lesson block and every exercise snippet (inside its template)
  must be unchanged by `gleam format --stdin`; for `bug` and `order` empty lines are
  ignored.

Snippets are put into `tpl` from the check (`{{CODE}}` is the marker, continuation
lines get the marker's indentation); without `tpl` the code runs as it is. Wrong
variants run only on the JavaScript target, with a 30 s timeout.

Results are cached in `.cache.json` (not committed), keyed by the SHA-256 of the Gleam
version, the target and the program.

The `pairs` exercises and the knowledge question `hello-1` have no check; review them
by reading.
