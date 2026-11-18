# Verifying the F# course

`verify.py` runs the course code on the Compiler Explorer API
(`dotnet80fsharpcoreclr`, .NET 8.0 CoreCLR with F# 8), because .NET is not
installed locally. Only Python 3 is needed.

```bash
python3 courses/fsharp/verify/verify.py              # all units
python3 courses/fsharp/verify/verify.py lists        # only some units
```

What it checks:

- **Lesson code blocks**: each one is a complete program (top-level F# code); it
  must compile without warnings (`warning FS…`), and its stdout must equal
  `output` exactly (blocks without `output` must print nothing).
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
- **Style**: no tabs, no trailing spaces and no comments (`//`, `(*`) in any code.

When exercise code is only a part (`blocks`), `tpl` in the check holds the
program, with `{{CODE}}` as the marker (indented like the marker). Programs that
should work must also compile without warnings. An "error" result only counts
when the compiler reports `error FS…` or the program crashes; any other server
answer stops the script.

Results are cached in `.cache.json` (not committed), keyed by the SHA-256 of the
program, so reruns only send changed programs. Requests are sent one at a time
with a pause of one second.

Not checked by the script: the `pairs` exercises (review by reading) and the
uniqueness of `order` exercises (each was reasoned through by hand: the lines
depend on each other, or every other order prints something else).
