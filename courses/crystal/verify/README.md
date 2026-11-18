# Verifying the Crystal course

`verify.py` runs the course code on the Compiler Explorer API (`crystal1221`,
Crystal 1.22.1), because Crystal is not installed locally. Only Python 3 is needed.

```bash
python3 courses/crystal/verify/verify.py              # all units
python3 courses/crystal/verify/verify.py loops        # only some units
```

What it checks:

- **Lesson code blocks**: each one is a complete program; it must compile, and its
  stdout must equal `output` exactly (blocks without `output` must print nothing).
- **Exercises** with an entry in `checks.json` (keyed by exercise id):
  - `input`, `choice`, `true_false`: the exercise `code` (or `code` from the check)
    is run; stdout must equal `out` (for `input` it must also match one of the
    `answers`; for `choice` the right option in backticks must equal the output and
    no wrong option may equal it; with `"lines": n` the output must have n lines and
    the right option must be n), or with `"err": true` it must fail to compile or crash.
  - `gap`: the right option must print `out`; every wrong option must fail or print
    something else.
  - `bug`: the code as shown must fail or print something else; with the bug line
    replaced by `fix` it must print `out`.
  - `order`: the lines in the given order must print `out`.
  - `blocks`: the tokens joined with spaces must print `out`; with
    `"distractors": true`, every distractor put in place of every token must fail or
    print something else.
  - `extra`: additional complete programs (`src`) with `out` or `"err": true`, used
    for exercises without code (facts stated in options or statements).

Snippets are put into `tpl` from the check (`{{CODE}}` is the marker, indented like
the marker; it may appear more than once), or run as they are. Crystal runs
top-level code, so every snippet is a complete program.

Results are cached in `.cache.json` (not committed), keyed by the SHA-256 of the
compiler id and the program, so reruns only send changed programs. Requests are
sent one at a time with a pause of one second.

Not checked by running: the `pairs` exercises, and the uniqueness of the `order`
exercises (review them by reading; the prompts of `methods-8` and `classes-9` say
that the definition comes first, because Crystal also accepts top-level methods and
classes defined after their use).
