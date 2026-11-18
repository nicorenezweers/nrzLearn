# Verifying the C# course

.NET is not needed. `verify.py` sends every program to the
[Compiler Explorer](https://godbolt.org) API (`dotnet90csharpcoreclr`), runs
it there and compares the output.

```bash
python3 courses/csharp/verify/verify.py
```

It prints `ALL CHECKS PASSED` or lists every failure. Results are cached in
`.cache.json` (not committed), keyed by the program text, so a second run only
sends changed programs. It waits one second between requests.

## What it does

Each program gets this prefix, the same as a `dotnet new console` project
(.NET 8 implicit usings, nullable warnings on):
`#nullable enable` + `using System; System.Collections.Generic; System.IO;
System.Linq; System.Net.Http; System.Threading; System.Threading.Tasks;`.

- **Lesson code blocks**: they must compile without warnings, run, and print
  exactly `output` (or nothing if there is no `output`).
- **Exercises**: `checks.json` describes how each exercise is checked, keyed
  by exercise ID:
  - `input`: the code runs, and its output must be one of the `answers`
    (compared the same way as the app: case, spaces, `_`, quotes and backticks are ignored).
  - `choice` / `true_false`: `src` (default: the exercise code) runs and must print
    `out`. `answer_out` means the output must equal exactly the right option
    and none of the others. `error` means the code must not compile. `each` is a
    template with `___`: the right option must run and print `out`, every
    wrong option must fail or print something else.
  - `gap`: every option is put into the gap. The right one must print `out`,
    each wrong one must fail to compile, crash, or print something else.
  - `bug`: the lines as given must fail or print something other than `out`,
    and with the bug line replaced by `fix` they must print `out`.
  - `order`: the lines in the given order (inside `template`, if any) must
    print `out`. Every order with two neighbouring lines swapped must
    *not* print `out` (a check that the order is unambiguous).
  - `blocks`: the tokens joined with spaces, put into `template`, must print `out`.

The C# version on Compiler Explorer is newer (C# 13 / .NET 9) than the course
target (C# 12 / .NET 8), so the course code only uses C# 12 features.
