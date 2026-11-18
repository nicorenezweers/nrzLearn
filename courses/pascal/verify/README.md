# Verifying the Pascal course

```bash
python3 courses/pascal/verify/verify.py        # Free Pascal 3.2.2 on Compiler Explorer; add -v to list every program
python3 courses/pascal/verify/verify.py --local  # use the fpc in PATH instead (set FPC to choose another one)
```

By default every program is compiled and run with **Free Pascal 3.2.2**
(x86-64 Linux) on Compiler Explorer (godbolt.org, compiler id in
`GODBOLT_COMPILER`, default `fpc322`). Answers are cached in `.cache/` (not
committed), so each program is sent only once, one request per second. A
compiler **warning counts as a failure**; notes and hints are allowed.

## What is checked

- **Lesson code blocks**: must be complete programs with `{$mode objfpc}{$H+}`;
  compiled, run, stdout (without the final newline) compared with `output`
  character by character.
- **Exercises**: the steps in `checks.json` (key = exercise id). `pairs` have no
  check.

Snippets are completed like this: `{$mode objfpc}{$H+}` is added when missing,
and `program VerifySnippet;` when there is no `program` line (Compiler Explorer
needs it, Free Pascal itself does not). A snippet without a line that starts
with `begin` (or any snippet with a `decl` field) is a list of statements and
is wrapped in `begin ... end.`, with `decl` (var/type sections) in front.

A "good" program must compile without warnings, exit with 0 and print the
expected text. A "bad" program must fail: compile error, warning, runtime
error, timeout or a different output. With `-v`, the reason why each bad
program fails is printed, so you can check that it fails for the intended reason.

| Exercise | What the default step does |
|---|---|
| `input` | runs `code`; output must match one of `answers` (case, spaces, `_`, quotes, backticks ignored, like the app) |
| `choice` | runs `code`; output must equal the right option (without backticks) |
| `gap` | right option must print `expect`; **every wrong option must fail** |
| `blocks` | the tokens joined with spaces, inserted into `src`, must print `expect` |
| `order` | the lines in the given order, inserted into `src`, must print `expect` |
| `bug` | the line replaced by `fix` must print `expect`; the original lines must fail |

Step fields: `src` (program with placeholders `@CODE@`, `@ANSWER@`, `@OPTION@`,
`@GAP@` = the gap option; for bugs `@FIXED@`), `decl`, `expect`, `fix`, and `kind`:

- `run` (default): see table.
- `options`: every option of a `choice` is inserted at `@OPTION@`; the right one
  must print `expect`, all others must fail.
- `fail`: the program must fail (statements that are false because the code
  does not compile).
- `manual`: no runnable code; `note` says why the answer is right.
