# Verifying the JavaScript course

```bash
node courses/javascript/verify/verify.mjs
```

Needs only Node (tested with Node 26), no packages. Exit code 0 means every check passed.

What `verify.mjs` checks, reading `../course.json`:

- **Lesson code**: every `code` block is run with `node`. Its stdout must equal `output`
  exactly, or be empty when `output` is missing, and it must run without an error.
- **input**: the exercise code is run; the first answer must equal the output (compared like
  the app: case, spaces, `_`, quotes and backticks ignored) and the output must be one line.
- **choice**: if all options are code (or "An error"), the code is run and exactly the answer
  option must match the output. Other choices have a `pick` function in `checks.mjs` that runs
  code and returns the right index.
- **true_false**: `checks.mjs` has a `holds` function that runs code to decide the statement.
- **gap**: every option is put into the gap and run. Only the answer option may give the
  expected output (`out` in `checks.mjs`).
- **blocks** / **order**: the right order is run (with optional `before`/`after` code or several
  `cases`) and must give the expected output. Then every other order of the blocks or lines
  (up to 8 items) is run in a `vm` sandbox; none of them may give the expected output, so the
  right order is unambiguous.
- **bug**: the original lines must not give the expected output; with the bug line replaced by
  `fix` from `checks.mjs`, they must.
- **pairs**: a `check` function runs code that confirms the meanings.

It also lists every code line that contains `//`, so you can check that no natural-language
comment slipped in. When you change an exercise, update its entry in `checks.mjs`.
