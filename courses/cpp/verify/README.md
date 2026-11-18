# Verifying the C++ course

```bash
python3 courses/cpp/verify/verify.py      # add -v to list every program
```

Needs `clang++` (or set `CXX`). Every program is compiled with
`-std=c++20 -Wall -Wextra -pedantic`; **any warning counts as a failure**.

## What is checked

- **Lesson code blocks**: compiled, run, stdout (without the final newline)
  compared with `output` character by character.
- **Exercises**: the steps in `checks.json` (key = exercise id). `pairs` have no
  check. A snippet without `int main(` is wrapped in `main()` with the includes
  `<iostream>`, `<memory>`, `<string>` and `<vector>`.

A "good" program must compile without warnings, exit with 0 and print the
expected text. A "bad" program must fail: compile error, warning, crash,
non-zero exit, timeout or a different output.

| Exercise | What the default step does |
|---|---|
| `input` | runs `code`; output must match one of `answers` (case, spaces, `_`, quotes, backticks ignored, like the app) |
| `choice` | runs `code`; output must equal the right option (without backticks) |
| `gap` | right option must print `expect`; **every wrong option must fail** |
| `blocks` | the tokens joined with spaces, inserted into `src`, must print `expect` |
| `order` | the lines in the given order, inserted into `src`, must print `expect` |
| `bug` | the line replaced by `fix` must print `expect`; the original lines must fail |

Step fields: `src` (program with placeholders `@CODE@`, `@ANSWER@`, `@OPTION@`;
for bugs `@FIXED@`), `expect`, `fix`, and `kind`:

- `run` (default): see table.
- `options`: every option of a `choice` is inserted at `@OPTION@`; the right one
  must print `expect`, all others must fail.
- `fail`: the program must fail (used for statements that are false because the
  code does not compile).
- `manual`: no runnable code (pure definitions); `note` says why the answer is
  right. These are listed in the output.

## Checked by reasoning

- `order`: the verifier runs the right order only. That no other order gives
  the same result was checked by hand (data dependencies, the prompt fixes the
  structure).
- `blocks`: no distractor can stand in for a token and still give valid code.
- `hello-4`, `classes-5`: definitions (`manual` steps).
