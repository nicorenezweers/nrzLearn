# Verifying the Swift course

```bash
python3 courses/swift/verify/verify.py        # optional: --jobs 8
```

Needs only Python 3 and Swift (tested with Swift 6.4), no packages. Every snippet is run with
`swift file.swift`, using only the standard library. Exit code 0 means every check passed
(about 1100 Swift runs, roughly 2–3 minutes on 8 cores).

`verify.py` reads `../course.json` and `checks.json` (the expected results per exercise id):

- **Lesson code**: every `code` block must run without an error, and its stdout must equal
  `output` exactly (or be empty when `output` is missing).
- **input**: the code (with optional `before`/`after`) must print one line that equals the first
  answer, compared like the app (case, spaces, `_`, quotes and backticks ignored).
- **choice**: with `variants`, every option is run as code and only the answer may give `out`.
  With `map`, the code's output is mapped to an option index. Otherwise, if the exercise has code,
  its output must match exactly the answer option (`An error` matches a failing program).
  Choices about concepts have `probes` instead.
- **true_false**: `probe` is `[code, expectation]`; whether the expectation holds must equal the
  answer. An expectation is an exact output, `!error` (does not compile or crashes) or `!ok`.
- **gap**: every option is put into the gap; only the answer may give `out` (in every `cases` entry).
- **blocks**: the tokens joined with spaces must give `out`. Every line where one token is
  replaced by a distractor, and every other order of the tokens (all orders up to 5 tokens,
  otherwise every move or swap of one token), must not.
- **order**: the same for lines: the right order gives `out`, no other order does.
- **bug**: the original lines must not give `out`; with the bug line replaced by `fix`, they must.
- **pairs**: `probes` (code with the expected output) confirm the meanings.
- `probes` in any entry must all hold.

It also lists every code line that contains `//`, so you can check that no natural-language
comment slipped in. When you change an exercise, update its entry in `checks.json`.

Extra check used once: all lesson code blocks were also compiled with
`swiftc -swift-version 6` and gave the same output.
