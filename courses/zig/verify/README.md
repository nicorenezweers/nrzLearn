# Verifying the Zig course

`verify.py` runs the course code with the local Zig compiler (`zig run`, written
for Zig 0.17.1). Only Python 3 and `zig` are needed.

```bash
python3 courses/zig/verify/verify.py                # all units
python3 courses/zig/verify/verify.py loops errors   # only some units
python3 courses/zig/verify/verify.py --godbolt      # also run the lesson blocks on Compiler Explorer (zig 0.17.0)
```

`std.debug.print` writes to stderr, so the script compares stderr; stdout must be
empty. A program "fails" when it does not compile or exits with an error (for
example a panic such as `integer overflow`).

What it checks:

- **Lesson code blocks**: each one is a complete program; its stderr must equal
  `output` exactly.
- **Exercises** with an entry in `checks.json` (keyed by exercise id):
  - `input`, `choice`, `true_false`: the exercise `code` (or `code` from the check)
    is run; the output must equal `out` (for `input` it must also match one of the
    `answers`), or with `"err": true` the program must fail.
  - `gap`: the right option must print `out`; every wrong option must fail or print
    something else.
  - `bug`: the lines as shown must fail or print something else; with the bug line
    replaced by `fix` they must print `out`.
  - `order`: the lines in the given order must print `out`; every swap of two
    neighbouring lines must fail or print something else.
  - `blocks`: the tokens joined with spaces must print `out`.
  - `extra`: additional complete programs (`src`) with `out` or `"err": true`, used
    for facts stated in options or statements (each option of a knowledge question).
- **zig fmt**: every lesson block and every correct exercise program must already
  be formatted (`zig fmt --stdin` changes nothing).

Snippets are put into a template: `tpl` from the check (`{{CODE}}` is the marker,
indented like the marker), or by default `const std = @import("std");` plus either
the code as it is (when it contains `pub fn main`) or the code inside
`pub fn main() !void`.

Results are cached in `.cache.json` (not committed), keyed by compiler and the
SHA-256 of the program.

The `pairs` exercises and the knowledge statement `hello-4` (`zig run` compiles and
runs) have no check; review them by reading.
