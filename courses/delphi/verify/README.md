# Verifying the Delphi course

```bash
python3 courses/delphi/verify/verify.py        # add -v to list every check
```

Delphi itself is not available on Linux/macOS build machines, so every program
is compiled and run with **Free Pascal 3.2.2 in Delphi mode** on Compiler
Explorer (godbolt.org, compiler id `fpc322`, override with `GODBOLT_COMPILER`).
Options: `-Mdelphi -Cr -Co -Sa` (Delphi mode, range, overflow and assertion
checks). Answers are cached in `.cache/` (not committed), so each program is
sent only once, at most one request per second. Only Python 3 is needed.

The course avoids features that behave differently in real Delphi: no inline
variables, no FPC-only syntax, no `case` on strings, no bare `Writeln` of a
`Double` (the scientific format differs), no locale-dependent float formatting
(`FloatToStr`, `%f`), and no reliance on uninitialised memory.

## What is checked

- **Lesson code blocks** with `output`: must be complete programs; compiled,
  run, stdout (without the final newline) compared character by character.
- **Exercises**: the steps in `checks.json` (key = exercise id). `pairs` have no
  check. A snippet that does not start with `program` gets the header
  `program Check; {$APPTYPE CONSOLE}`; if it does not end with `end.` it is
  also wrapped in `begin ... end.` (with an optional `decl` before `begin`).

A "good" program must compile without warnings or errors, exit with 0 and print
the expected text. A "bad" program must fail: compile error, warning, runtime
error, non-zero exit or a different output.

| Exercise | What the default step does |
|---|---|
| `input` | runs `code`; output must match one of `answers` (case, spaces, `_`, quotes, backticks ignored, like the app) |
| `choice` | runs `code`; output must equal the right option (without backticks) or `expect` |
| `true_false` | runs `src`, must print `expect` |
| `gap` | right option must print `expect` in every step; **every wrong option must fail at least one step** |
| `blocks` | the tokens joined with spaces, inserted into `src`, must print `expect` |
| `order` | the lines in the given order must print `expect`; **every swap of two neighbouring lines must fail** (unless `"swaps": false`) |
| `bug` | the original lines must fail; with the bug line replaced by `fix` (or removed with `"delete": true`) they must print `expect` |

Step fields: `src` (program with the placeholders `@CODE@`, `@ANSWER@`,
`@OPTION@`, `@LINES@`), `decl`, `expect`, `fix`, `delete`, `swaps` and `kind`:

- `run` (default): see the table.
- `options`: every option of a `choice` is inserted at `@OPTION@`; the right one
  must print `expect`, all others must fail.
- `fail`: the program must fail (statements that are false because the code
  does not compile).
- `manual`: facts that cannot be shown by running code; `note` gives the reason.
  They are listed at the end of the output.
