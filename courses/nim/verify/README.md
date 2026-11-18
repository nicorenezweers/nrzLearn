# Verifying the Nim course

`verify.py` runs the course code online, because Nim is not installed locally.
Only Python 3 is needed.

- Default backend: the official Nim Playground (`play.nim-lang.org`, Nim 2.2.6).
  The Playground answers a few 2.2.6 programs (those using `contains` or `replace`
  from `std/strutils`) with HTTP 502 every time; after two 502 answers the program
  is sent again with Nim 2.2.4, and the script reports how many programs used it.
- Second backend: Wandbox (`nim-2.2.10`), when it is up: `--backend wandbox`.
  (Compiler Explorer only compiles Nim and cannot run it.)

```bash
python3 courses/nim/verify/verify.py                      # all units
python3 courses/nim/verify/verify.py loops procs          # only some units
python3 courses/nim/verify/verify.py --backend wandbox    # same checks on Wandbox
python3 courses/nim/verify/verify.py --run test.nim       # run one file
```

What it checks:

- **Lesson code blocks**: each one is a complete program; it must compile and run
  without an error, and its stdout must equal `output` exactly (blocks without
  `output` must print nothing).
- **Exercises** with an entry in `checks.json` (keyed by exercise id):
  - `input`, `choice`, `true_false`: the exercise `code` (or `code` from the check)
    is run; stdout must equal `out` (for `input` it must also match one of the
    `answers`), or with `"err": true` it must fail to compile or stop with an error.
  - `gap`: the right option must print `out`; every wrong option must fail or print
    something else.
  - `bug`: the code as shown must fail or print something else; with the line
    replaced by `fix` it must print `out`.
  - `order`: the lines in the given order must print `out`; each order in
    `alt_orders` (lists of line indexes) must fail or print something else.
  - `blocks`: the tokens joined with spaces must print `out`; with
    `"distractors": true`, putting any distractor in place of any token must fail
    or print something else.
  - `extra`: additional complete programs (`src`) with `out` or `"err": true`, used
    for exercises without code (facts stated in options or statements).

Exercise code is run as it is (Nim programs need no boilerplate), or inside `tpl`
from the check (`{{CODE}}` is the marker, indented like the marker).

The Playground returns stdout and stderr together; a program counts as failed when
the compile log has an `Error:` or the output has `Error: unhandled exception`.

Results are cached in `.cache.json` (not committed), keyed by backend and the
SHA-256 of the program, so reruns only send changed programs. Requests are sent
one at a time with a pause of one second.

The `pairs` exercises have no check; review them by reading.
