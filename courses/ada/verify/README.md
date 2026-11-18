# Verifying the Ada course

`verify.py` runs the course code on [Compiler Explorer](https://godbolt.org)
(x86-64 GNAT 15.2, compiler id `gnat152`, flag `-gnat2012`) and compares the
results. Only Python 3 and network access are needed; nothing is installed.

```bash
python3 courses/ada/verify/verify.py                 # all units
python3 courses/ada/verify/verify.py loops records   # only some units
ADA_COMPILER=gnat162 python3 courses/ada/verify/verify.py   # another GNAT
```

Results are cached in `.cache.json` (ignored by git), keyed by compiler, flags
and source, so a second run only sends changed programs. Requests are sent one
by one with a pause of about one second.

A program has one of three results: `ok` (compiled and exited with 0),
`compile` (did not compile) or `runtime` (stopped with an error, for example
`Constraint_Error`). Only stdout is compared.

What it checks:

- **Lesson code blocks**: each one must be a complete program starting with
  `with Ada.Text_IO; use Ada.Text_IO;`; its stdout must equal `output` exactly.
  Compiler warnings are listed at the end for review.
- **Exercises** with an entry in `checks.json` (keyed by exercise id). Snippets
  become programs like this: a `tpl` replaces `{{CODE}}`; code starting with
  `procedure Main` gets the `with`/`use` line; code with a `begin` line is
  wrapped in `procedure Main is ... end Main;`; other code is wrapped as
  statements.
  - `input`, `choice`, `true_false`: the exercise `code` (or `code` from the
    check) must print `out` (for `input`, `out` must also match one of the
    `answers` the way the app compares them), or fail with `err`
    (`compile` or `runtime`).
  - `gap`: the right option must print `out`; every wrong option must fail or
    print something else.
  - `bug`: the lines as shown must fail or print something else; with the bug
    line replaced by `fix` they must print `out`.
  - `order`: the lines in the given order must print `out`; every swap of any
    two lines must fail or print something else.
  - `blocks`: the tokens joined with spaces must print `out`.
  - `extra`: more programs with their expected `out` or `err`.

`pairs` and the `choice` exercises without code are knowledge questions and
are checked by reading; the script lists them as "not checked by running code".
