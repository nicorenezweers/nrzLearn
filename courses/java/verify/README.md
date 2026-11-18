# Verifying the Java course

`verify.py` runs the course code on the Compiler Explorer API (`java2102`, JDK 21.0.2),
because Java is not installed locally. Only Python 3 is needed.

```bash
python3 courses/java/verify/verify.py                      # all units
python3 courses/java/verify/verify.py --only loops         # units whose id starts with "loops"
python3 courses/java/verify/verify.py --compiler java2502  # cross-check with another JDK (here 25.0.2)
python3 courses/java/verify/verify.py --no-cache           # ignore the cache
```

Compiler Explorer saves the source as `example.java`, so a `public class Main` does not
compile there. The script turns `public class Main` into `class Main` before sending a
program (except for checks marked `"raw": true`); nothing else in the code changes.

What it checks:

- **Lesson code blocks**: each is a complete program (`public class Main` with
  `public static void main(String[] args)`), has no comments, compiles, and its stdout
  equals `output` exactly (blocks without `output` must print nothing).
- **Exercises**, with the options from `checks.json` (keyed by exercise id):
  - `input`: the exercise `code` is run; stdout must match one of the `answers`, and the
    first answer must be the exact output.
  - `choice` with `code`: stdout must equal the right option (without backticks) or `expect`.
  - `true_false` with `code`: stdout must equal `expect`.
  - `gap`: the right option must print `expect`; every wrong option must fail or print
    something else.
  - `bug`: the code as shown must fail or print something else than `expect`; with the
    bug line replaced by `fix` it must print `expect`.
  - `order`: the lines in the given order must print `expect`.
  - `blocks`: the tokens joined with spaces must print `expect`.
  - `programs`: extra programs (`code`, `expect`) for facts stated in options or
    statements, e.g. that a wrong option does not compile.
- `expect` can be an output or `@compile_error` / `@runtime_error`.

Snippets get the missing parts around them: `tpl` from the check (with `{{CODE}}` as
the marker, indented like the marker), otherwise code that contains `class Main` stays as
it is, code with a `main` method is put into `public class Main { }`, and other code is
put into `main`. Snippets that use `List`, `ArrayList` or `Arrays` get `import java.util.*;`.

Results are cached in `.cache.json` (not committed), keyed by compiler and program.
Requests are sent one at a time with a pause of one second.

The `pairs` exercises and the knowledge question `classes-3` have no program;
review them by reading.
