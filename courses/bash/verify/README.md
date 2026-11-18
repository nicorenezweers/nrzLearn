# Verifying the Bash course

`verify.py` runs the course code with a current Bash and checks it. Only Python 3
and Homebrew are needed:

```bash
brew install bash coreutils grep                         # Bash 5, GNU tools for the second run
python3 courses/bash/verify/verify.py                    # all units
python3 courses/bash/verify/verify.py loops arrays       # only some units
python3 courses/bash/verify/verify.py --wandbox          # also run on Linux (Wandbox, Bash 5.2)
```

Every program runs as `bash script.sh` in a new, empty temporary folder, on each backend:

- **mac**: Homebrew Bash 5 with the macOS (BSD) tools (`sort`, `grep`, `wc`, …).
- **gnu**: the same Bash, with GNU coreutils and GNU grep first in `PATH`, as on Linux.
- **wandbox** (with `--wandbox`): Bash 5.2 on Linux via the Wandbox API. Results are
  cached in `.cache.json` (not committed), keyed by the SHA-256 of the program;
  requests are sent one at a time with a pause of one second.

A run passes when the exit status is `0` and nothing is printed to stderr. All
backends must give the same, expected stdout (one final newline is ignored).

What it checks:

- **Lesson code blocks**: each one is a complete script; its stdout must equal
  `output` exactly (blocks without `output` must print nothing).
- **Exercises** with an entry in `checks.json` (keyed by exercise id):
  - `input`, `choice`, `true_false`: the exercise `code` (or `code` from the check) must
    print `out`; for `input`, the output must also match one of the `answers`.
    `"err": true` means the code must fail; `"stderr": true` allows error messages.
  - `gap`: the right option must print `out`; every wrong option must fail or print
    something else.
  - `bug`: the code as shown must fail or print something else; with the bug line
    replaced by `fix`, it must print `out`.
  - `order`: the lines in the given order must print `out`. For up to 6 lines, every
    other order is also run, and none may print `out` (the order must be unambiguous).
  - `blocks`: the tokens joined with spaces must print `out`; with `"distractors": true`,
    no distractor may replace a token and still print `out`.
  - `tpl`: a template with `{{CODE}}`, for code that needs a variable or arguments first.
  - `extra`: additional scripts (`src`) with `out` or `"err": true`, for exercises
    without code (facts stated in options or statements).

The `pairs` exercises and the knowledge question `hello-1` have no check; review them
by reading.
