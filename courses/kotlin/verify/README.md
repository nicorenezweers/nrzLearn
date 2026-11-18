# Verifying the Kotlin course

Kotlin is not installed locally. `verify.py` runs the code on the
[Compiler Explorer](https://godbolt.org) API (`kotlinc2421`, Kotlin 2.4) and
compares the results.

```bash
python3 courses/kotlin/verify/verify.py                 # everything
python3 courses/kotlin/verify/verify.py --only loops    # one unit
python3 courses/kotlin/verify/verify.py --no-cache      # ignore .cache.json
```

It waits 1 second between requests and caches the results in `.cache.json`
(git-ignored), so later runs only send changed programs.

## What is checked

- **Lesson code blocks:** every block compiles and runs, and stdout equals
  `output` exactly. A block without `output` must print nothing.
- **Exercises**, as described in `checks.json` (one entry per exercise id):
  - `input`: the code runs, and its output is one of `answers`. The first answer
    must be the exact output.
  - `choice` with code: the output equals the right option without backticks,
    or `expect`.
  - `true_false` with code: the output equals `expect`.
  - `gap`: the code with the right option prints `expect`. Every wrong option
    gives something else: a compile error, a runtime error or a different output.
  - `order`: the lines in the given order (plus `append`) print `expect`.
  - `blocks`: the tokens joined with spaces, put into `template` at `@@`, print `expect`.
  - `bug`: the lines as given do **not** print `expect`. With line `bug`
    replaced by `fix`, they print `expect`.
  - `programs` (any type): extra programs with their expected result, e.g. to
    show that the wrong options of a `choice` do not compile.

  `expect` can also be `@compile_error` or `@runtime_error`. Code without
  `fun main()` is wrapped into one. Exercises without code (`pairs`, some
  `choice` and `true_false`) need no entry. Most of them still have
  `programs` that check the facts behind them.

## Why the runner wraps `main`

Compiler Explorer starts the JVM with `-Xss136K`. With that stack, the first
use of a Kotlin collection (`listOf`, `mapOf`, …) fails with a
`StackOverflowError`. The runner therefore renames the course's `fun main()`
to `courseMain()` and adds a `main` that calls it in a thread with a 16 MB
stack. It also sets `JAVA_TOOL_OPTIONS=-Xint …` so the sandbox has room for
that thread. The course code itself is not changed. In execute mode, compile
errors only return `Assertion failed`, so the runner then compiles the
original program without running it and reads the `error:` lines.
