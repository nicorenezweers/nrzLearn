# Verifying the TypeScript course

```bash
node courses/typescript/verify/verify.mjs
```

Needs `node` (26+, runs `.ts` by stripping types) and `deno` (2.x, used only for
`deno check`). The script reads `../course.json` and `checks.json`, writes every
snippet to a temporary directory, type-checks all of them in one `deno check`
call and runs each one with `node`. It exits with 1 if anything fails.

What is checked:

- **Lesson code**: type-checks cleanly (or has exactly the error lines listed in
  `checks.json` → `lessons`), and prints exactly `output`; blocks without
  `output` must print nothing.
- **input**: the code type-checks and its output matches one of the answers
  (ignoring case, spaces, `_`, quotes and backticks, like the app).
- **choice / true_false**: `snippets` in `checks.json` (code, expected type
  error lines, expected output; `@CODE` stands for the exercise code), or
  `answerOutput`: the code prints the right option.
- **gap**: the right option type-checks and prints `output`; every wrong option
  fails the type check or prints something else.
- **bug**: the code has type errors only on line `bug + 1`; with the line
  replaced by `fix` (at `fixAt`, if given) it type-checks and prints `output`.
- **order**: the lines in the given order print `output`; moving any single line
  to any other position breaks the type check, throws or changes the output.
- **blocks**: the tokens joined with spaces, inserted into `wrap` at `$`,
  type-check and print `output`.
- Code containing `//` or `/*` is reported as a warning (no comments in code).

`checks.json` is keyed by lesson block (`<unit>.lesson.<index>`) and exercise id.
Keep it in sync when you change an exercise.
