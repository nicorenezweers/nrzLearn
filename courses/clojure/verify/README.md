# Verifying the Clojure course

`verify.py` runs the course code locally with [Babashka](https://babashka.org)
(`bb`, tested with v1.13.229), which runs Clojure 1.12 code. Only Python 3 and `bb`
are needed (`brew install borkdude/brew/babashka`).

```bash
python3 courses/clojure/verify/verify.py              # all units
python3 courses/clojure/verify/verify.py maps         # only some units
```

What it checks:

- **Lesson code blocks**: each one is a complete script; it must run without error,
  and its stdout must equal `output` exactly (blocks without `output` must print nothing).
- **Exercises** with an entry in `checks.json` (keyed by exercise id):
  - `input`, `choice`, `true_false`: the exercise `code` is run; stdout must equal
    `out` (for `input` it must also match one of the `answers`), or with
    `"err": true` it must fail.
  - `gap`: the right option must print `out`; every wrong option must fail or
    print something else.
  - `bug`: the code as shown must fail or print something else; with the bug line
    replaced by `fix` it must print `out`.
  - `order`: the lines in the given order must print `out`, and no other order
    (all permutations with balanced brackets are run) may print `out`.
  - `blocks`: the tokens joined with spaces, put into `tpl` at `{{CODE}}`, must
    print `out`.
  - `extra`: additional scripts (`src`) with `out` or `"err": true`, used for
    exercises whose options are code without a program (for example `hello-4`).
- Every exercise except `pairs` must have a check; review `pairs` by reading.

Each program runs in a fresh `bb` process with a time limit of 20 seconds (a
timeout counts as a failure). Nothing is cached and no files are written inside
the repository.
