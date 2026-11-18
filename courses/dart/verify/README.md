# Verifying the Dart course

`verify.py` runs the course code on the Compiler Explorer API (`dart3136`, Dart 3.13.6),
because Dart is not installed locally. Only Python 3 is needed.

```bash
python3 courses/dart/verify/verify.py                # all units
python3 courses/dart/verify/verify.py collections    # only some units
```

What it checks:

- **Lesson code blocks**: each one is a complete program with `void main()`; it must
  compile, and its stdout must equal `output` exactly (blocks without `output` must
  print nothing).
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
    `"distractors": true`, putting a distractor in place of any token must not
    print `out`.
  - `extra`: additional complete programs (`src`) with `out` or `"err": true`, used
    for exercises without code (facts stated in options or statements).

Snippets are put into a template: `tpl` from the check (with `{{CODE}}` as the
marker, indented like the marker), or by default the code as it is (when it
contains `void main()`) or wrapped in `void main() { ... }`.

Results are cached in `.cache.json` (not committed), keyed by the SHA-256 of the
compiler id and the program, so reruns only send changed programs. Requests are sent
one at a time with a pause of one second.

The `pairs` exercises have no check; review them by reading.
