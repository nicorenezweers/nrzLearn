# Verifying the Scheme course

`verify.py` runs the course code locally with
[chibi-scheme](https://github.com/ashinn/chibi-scheme) (tested with 0.12), the
R7RS small reference implementation. Only Python 3 and `chibi-scheme` are needed
(`brew install chibi-scheme`, about 30 MB).

```bash
python3 courses/scheme/verify/verify.py              # all units
python3 courses/scheme/verify/verify.py lists        # only some units
```

The course code leaves out the `import` line (see the tip in unit `hello`).
Chibi runs files as strict R7RS programs, so the script puts this line in front
of every program:

```scheme
(import (scheme base) (scheme write) (scheme char) (scheme cxr) (scheme inexact))
```

This also makes sure the course uses only R7RS small procedures (for example
`exact->inexact`, `filter` or `iota` would fail as undefined).

What it checks:

- **Lesson code blocks**: each one is a complete program; it must run without
  error, and its stdout must equal `output` exactly.
- **Exercises** with an entry in `checks.json` (keyed by exercise id):
  - `input`, `choice`, `true_false`: the exercise `code` is run; stdout must equal
    `out` (for `input` it must also match one of the `answers`), or with
    `"err": true` it must fail.
  - `gap`: the right option must print `out`; every wrong option must fail or
    print something else.
  - `bug`: the code as shown must fail or print something else; with the bug line
    replaced by `fix` it must print `out`.
  - `order`: the lines in the given order must print `out`, and no other order
    (all permutations with balanced parentheses are run) may print `out`.
  - `blocks`: the tokens joined with spaces, put into `tpl` at `{{CODE}}`, must
    print `out`.
  - `extra`: additional programs (`src`) with `out` or `"err": true`, used for
    exercises whose options are code without a program.
  - `manual`: no program can decide the answer (whether a call is a tail call);
    the reason is printed and the exercise was checked by reading.
- Every exercise except `pairs` must have a check; review `pairs` by reading.
- No program may `define` a name that R7RS small already provides (such as
  `square` or `list`); chibi allows that, but a strict R7RS program may not.

Each program runs in a fresh `chibi-scheme` process with a time limit of 10
seconds (a timeout counts as a failure, so some order permutations take a while).
Nothing is cached and no files are written inside the repository.
