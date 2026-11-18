# Verifying the Go course

`verify.py` runs the course code on the Compiler Explorer API (`gl1260`, Go 1.26),
because Go is not installed locally. Only Python 3 is needed.

```bash
python3 courses/go/verify/verify.py              # all units
python3 courses/go/verify/verify.py slices       # only some units
```

What it checks:

- **Lesson code blocks**: each one is a complete program; it must compile, and its
  stdout must equal `output` exactly (blocks without `output` must print nothing).
- **Exercises** with an entry in `checks.json` (keyed by exercise id):
  - `input`, `choice`, `true_false`: the exercise `code` (or `code` from the check)
    is run; stdout must equal `out` (for `input` it must also match one of the
    `answers`), or with `"err": true` it must fail to compile or crash.
  - `gap`: the right option must print `out`; every wrong option must fail or print
    something else.
  - `bug`: the code as shown must fail or print something else; with the line
    replaced by `fix` it must print `out`.
  - `order`: the lines in the given order must print `out`.
  - `blocks`: the tokens joined with spaces must print `out`.
  - `extra`: additional complete programs (`src`) with `out` or `"err": true`, used
    for exercises without code (facts stated in options or statements).
- **gofmt**: one Go program formats every snippet with `go/format` (after turning
  4-space indentation into tabs) and reports any difference.

Snippets are put into a template: `tpl` from the check (with `{{CODE}}` as the
marker, indented like the marker), or by default `package main` + imports
(`fmt`, plus `errors`/`strconv` when used) + either the code as top-level
declarations (when it contains `func main()`) or the code inside `func main()`.

Results are cached in `.cache.json` (not committed), keyed by the SHA-256 of the
program, so reruns only send changed programs. Requests are sent one at a time
with a pause of one second.

The `pairs` exercises and the knowledge question `hello-1` have no check; review
them by reading.
