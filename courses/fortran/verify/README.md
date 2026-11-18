# Verifying the Fortran course

`verify.py` runs the course code with gfortran on Compiler Explorer (godbolt.org),
default compiler `gfortran152` (x86-64 gfortran 15.2), flags `-std=f2018 -fcheck=all`.
Only Python 3 and network access are needed; nothing is installed.

```bash
python3 courses/fortran/verify/verify.py                                   # all units
python3 courses/fortran/verify/verify.py loops arrays                      # only some units
python3 courses/fortran/verify/verify.py --lessons --compiler=gfortran132  # lesson blocks on another gfortran
```

The script waits about 1 second between requests. Results are cached in
`.cache.json` (not committed), keyed by compiler, flags and the SHA-256 of the program.
A program "fails" when it does not compile or exits with a non-zero code
(for example a runtime error from `-fcheck=all`). Only stdout is compared.

What it checks:

- **Lesson code blocks**: each one is a complete program; its stdout must equal
  `output` exactly.
- **Exercises** with an entry in `checks.json` (keyed by exercise id):
  - `input`, `choice`, `true_false`: the exercise `code` is run; the output must
    equal `out` (for `input` it must also match one of the `answers`), or with
    `"err": true` the program must fail.
  - `gap`: the right option must print `out`; every wrong option must fail or
    print something else.
  - `bug`: the lines as shown must fail or print something else; with the bug line
    replaced by `fix` (or the whole program `fixed`) they must print `out`.
  - `order`: the lines in the given order must print `out`; every swap of two
    neighbouring lines must fail or print something else.
  - `blocks`: the tokens joined with spaces must print `out`.
  - `extra`: additional complete programs (`src`) with `out` or `"err": true`, used
    for facts stated in knowledge questions (each option or the statement).

Snippets are put into a template: `tpl` from the check (`{{CODE}}` is the marker,
indented like the marker), or by default the code as it is (when it contains a
`program` line) or the code inside `program main` / `implicit none` / `end program main`.

The `pairs` exercises have no check; review them by reading.
