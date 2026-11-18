# Verifying the Scala course

Scala is not installed locally. `verify.py` runs the code on the
[Compiler Explorer](https://godbolt.org) API (`scalac371`, Scala 3.7.1 on
JDK 21) and compares the results.

```bash
python3 courses/scala/verify/verify.py                 # everything
python3 courses/scala/verify/verify.py --only loops    # one unit
python3 courses/scala/verify/verify.py --no-cache      # ignore .cache.json
```

It waits 1 second between requests and caches the results in `.cache.json`
(git-ignored), so later runs only send changed programs.

## What is checked

- **Lesson code blocks:** every block compiles without warnings and runs, and
  stdout equals `output` exactly. A block without `output` must print nothing.
- **Exercises**, as described in `checks.json` (one entry per exercise id).
  Exercise code (and the right answer of gaps, blocks, orders and fixed bugs)
  must also compile without warnings.
  - `input`: the code runs, and its output is one of `answers`. The first answer
    must be the exact output.
  - `choice` with code: the output equals the right option without backticks,
    or `expect`, and no wrong option equals the output.
  - `true_false` with code: the output equals `expect`.
  - `gap`: the code with the right option prints `expect`. Every wrong option
    gives something else: a compile error, a runtime error or a different output.
  - `order`: the lines in the given order (plus `prepend`/`append`) print `expect`.
  - `blocks`: the tokens joined with spaces, put into `template` at `@@`, print `expect`.
  - `bug`: the lines as given do **not** print `expect`. With line `bug`
    replaced by `fix`, they print `expect`.
  - `programs` (any type): extra programs with their expected result, e.g. to
    show that the wrong options of a `choice` do not compile.

  `expect` can also be `@compile_error` or `@runtime_error`. Code without
  `@main` is wrapped into `@main def run(): Unit =`. Exercises without code
  (`pairs`, some `choice` and `true_false`) need no entry; most of them still
  have `programs` that check the facts behind them.

## Why the runner adds a launcher

Compiler Explorer starts the program with `java -Xss136K <MainClass>` and only
the program's own classes on the class path, so the Scala library is missing at
run time (`NoClassDefFoundError`). The runner therefore removes `@main` from
`@main def run(): Unit =` (so `run` becomes a top-level function in
`example$package`) and appends a launcher object that uses only Java classes:
it loads the program with a class loader that also sees Compiler Explorer's
`scala.jar`, and calls `run` in a thread with a 16 MB stack. It sets
`JAVA_TOOL_OPTIONS=-Xint …` so the sandbox has room for that thread. The course
code itself is not changed; the launcher is appended after it, so compile error
line numbers stay the same.
