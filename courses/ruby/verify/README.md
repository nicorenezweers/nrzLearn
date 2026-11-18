# Verifying the Ruby course

`verify.py` runs the course code on the [Compiler Explorer](https://godbolt.org)
API and compares the results. Only Python 3 is needed (the system Ruby on macOS
is too old).

```bash
python3 courses/ruby/verify/verify.py                 # everything
python3 courses/ruby/verify/verify.py --only loops    # one unit
python3 courses/ruby/verify/verify.py --no-cache      # ignore .cache.json
```

Every program runs on **two Ruby versions**, `ruby334` (Ruby 3.3.4) and
`ruby405` (Ruby 4.0.5). Both must give the same result. This catches output
that changed between versions (for example `p` on a hash, or error messages),
so the course avoids such output.

It waits 1 second between requests and caches the results in `.cache.json`
(git-ignored), so later runs only send changed programs.

## What is checked

- **Lesson code blocks:** every block runs without an error and without
  anything on stderr (no warnings), and stdout equals `output` exactly. A block
  without `output` must print nothing.
- **Exercises**, as described in `checks.json` (one entry per exercise id):
  - `input`: the code runs, and its output is one of `answers`. The first answer
    must be the exact output.
  - `choice` with code: the output equals the right option without backticks,
    or `expect`.
  - `true_false` with code: the output equals `expect`.
  - `gap`: the code with the right option prints `expect` (with a clean stderr).
    Every wrong option gives something else: an error or a different output.
  - `order`: the lines in the given order print `expect`.
  - `blocks`: the tokens joined with spaces print `expect`.
  - `bug`: the lines as given do **not** print `expect`. With line `bug`
    replaced by `fix`, they print `expect`.
  - `template` (for `gap`, `order`, `blocks`, `bug`): a program with `@@` as the
    place for the code, e.g. a class that the line uses.
  - `programs` (any type): extra programs with their expected result, used to
    check the facts behind questions without code (for example that the wrong
    options of a `choice` fail).

  `expect` can also be `@syntax_error`, `@error` (any exception) or
  `@error:<Class>`, e.g. `@error:NoMethodError`.

When you change an exercise, update its entry in `checks.json` too. Exercises without code
(`pairs`, some `choice` and `true_false`) need no entry; most still have
`programs` that check them. The `pairs` exercises are reviewed by reading.
