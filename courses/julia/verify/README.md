# Verifying the Julia course

`verify.py` runs the course code on the Compiler Explorer API, because Julia is not
installed locally. Every program runs on **Julia 1.10.0** and **Julia 1.12.5**
(`julia_1_10_0`, `julia_1_12_5`); both versions must give the same result. Only
Python 3 is needed.

```bash
python3 courses/julia/verify/verify.py              # all units
python3 courses/julia/verify/verify.py loops        # only some units
```

What it checks:

- **Lesson code blocks**: each one runs as a complete script; its stdout must equal
  `output` exactly (blocks without `output` must print nothing).
- **Exercises** with an entry in `checks.json` (keyed by exercise id):
  - `input`, `choice`, `true_false`: the exercise `code` (or `code` from the check)
    is run; stdout must equal `out` (for `input` it must also match one of the
    `answers`), or with `"err": true` it must stop with an error.
  - `gap`: the right option must print `out`; every wrong option must fail or print
    something else.
  - `bug`: the code as shown must fail or print something else; with the line
    replaced by `fix` it must print `out`.
  - `order`: the lines in the given order must print `out`; swapping any two
    neighbouring lines must fail or print something else.
  - `blocks`: the tokens joined with spaces must print `out`.
  - `extra`: additional programs (`src`) with `out` or `"err": true`, used for
    claims in lesson texts (key `_lesson`) and for
    exercises without code (facts stated in options or statements).
- A run only counts as successful when it exits normally **and** writes nothing to
  stderr, so warnings (for example the soft-scope warning for loops at the top
  level) count as failures.
- Code lines with `#` are printed so comments can be reviewed by hand.

`tpl` in a check wraps the exercise code (`{{CODE}}` is replaced by it).

How programs run: Julia needs several seconds to start on Compiler Explorer, so
`harness.jl` runs up to 10 snippets in one request. Each snippet is evaluated with
`include_string` in a fresh module (the same top-level scope rules as a script
file); its stdout is captured, and errors and log messages are reported. Snippets
that define a `struct` run alone with `include_string(Main, ...)`, because type
names print differently inside other modules.

Results are cached in `.cache.json` (not committed), keyed by compiler and the
SHA-256 of the snippet. Requests are sent one at a time with a pause of one second.

The `pairs` exercises have no check; review them by reading.
