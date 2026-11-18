# Verifying the Haskell course

`verify.py` compiles and runs the course code with GHC and checks it. Only Python 3
is needed, plus either internet access (Wandbox) or a local `ghc`.

```bash
python3 courses/haskell/verify/verify.py              # Wandbox, ghc-9.10.1
python3 courses/haskell/verify/verify.py --local      # local `ghc` (e.g. `brew install ghc`)
python3 courses/haskell/verify/verify.py --local lists recursion   # only some units
```

A program counts as successful only if it compiles **without any warning** (GHC's
default warnings, no `-Wall`), exits with code 0 and writes nothing to stderr.
Locally, it is compiled with `ghc -O0 -v0`; on Wandbox, the default `-Wall`
switch is turned off. Output comparisons accept the expected text with or without
one final newline.

What it checks:

- **Lesson code blocks**: each one with an `output` is a complete program (it
  defines `main`), compiles cleanly, and its stdout equals `output` exactly.
- **Exercises** with an entry in `checks.json` (keyed by exercise id):
  - `input`, `choice`, `true_false`: the exercise `code` (or `code` from the check)
    is run; stdout must equal `out` (for `input` it must also match one of the
    `answers`), or with `"err": true` it must fail.
  - `gap`: the right option must print `out`; every wrong option must fail or print
    something else.
  - `bug`: the code as shown must fail or print something else; with the bug line
    replaced by `fix` it must print `out`.
  - `order`: the lines in the given order must print `out`.
  - `blocks`: the tokens joined with spaces must print `out`.
  - `extra`: additional programs (`src`) for facts stated in options or statements.
- **Lesson claims** (`_claims`, per unit): statements in lesson texts, tips and
  warnings, e.g. that `putStrLn "Age: " ++ show 36` is a type error, that `/` does
  not work with an `Int`, or that a missing pattern stops the program with
  `Non-exhaustive patterns`. `diag` must appear in the diagnostics.

Code that defines `main` is run as it is. Other snippets become the body of
`main = do`; `tpl` (with the marker `{{CODE}}`, indented like the marker when it
starts a line) wraps code that needs surrounding lines, e.g. a `main` for a
function definition.

Results are cached in `.cache.json` (not committed), keyed by backend and the
SHA-256 of the program. Wandbox requests are sent one at a time with a pause of
one second.

Not checked by code (review by reading): the `pairs` exercises, the knowledge
question `hello-1`, and the order exercises' uniqueness (they avoid top-level type
signatures, which could stand anywhere in a file).

Last full run: GHC 9.14.1 (Homebrew, `--local`), because Wandbox returned
internal errors at the time: 39 lesson blocks, 89 exercises,
9 lesson claims, 194 programs, all passing.
