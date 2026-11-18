# Verifying the D course

`verify.py` runs the course code online, because D is not installed locally.
Only Python 3 is needed.

- Default backend: Compiler Explorer (`godbolt.org`, compiler `dmd21120` = dmd 2.112.0).
- Second backend: Wandbox (`ldc-1.39.0`), when it is up: `--backend wandbox`.

```bash
python3 courses/d/verify/verify.py                      # all units
python3 courses/d/verify/verify.py loops functions      # only some units
python3 courses/d/verify/verify.py --backend wandbox    # same checks on Wandbox
python3 courses/d/verify/verify.py --run test.d         # run one file
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
    for facts stated in options, statements, or lesson tips and warnings (these
    are attached to the `pairs` exercises, which have no check of their own).

How exercise code becomes a program:

- Code without `void main` is a snippet: it is put into
  `import std.stdio; void main() { ... }`. A snippet may start with a local
  `import std.string;`, which D allows inside a function.
- Code with `void main` but no `import` line gets `import std.stdio;` in front.
- A check can give its own template `tpl` (`{{CODE}}` is the marker, indented like
  the marker).

A program counts as failed when it does not compile or link, or when it exits with
a code other than 0 (for example a range error or a null access).

Results are cached in `.cache.json` (not committed), keyed by backend and the
SHA-256 of the program, so reruns only send changed programs. Requests are sent
one at a time with a pause of one second.

The `pairs` exercises are reviewed by reading.
