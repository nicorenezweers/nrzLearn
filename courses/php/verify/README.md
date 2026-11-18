# Verifying the PHP course

`verify.py` runs the course code with PHP and checks it. Only Python 3 is needed,
plus either internet access (Wandbox) or a local `php` binary.

```bash
python3 courses/php/verify/verify.py              # Wandbox, php-8.3.12
python3 courses/php/verify/verify.py --local      # local `php` (e.g. `brew install php`)
python3 courses/php/verify/verify.py --local loops functions   # only some units
```

A program counts as successful only if it exits with code 0 **and** prints no PHP
diagnostic (`Warning:`, `Notice:`, `Deprecated:`, `Fatal error:`, `Parse error:`).
Locally, PHP runs with `-n -d display_errors=stderr -d error_reporting=-1`.
Output comparisons accept the expected text with or without one final newline.

What it checks:

- **Lesson code blocks**: each one starts with `<?php`, runs without diagnostics,
  and its stdout equals `output` exactly.
- **Exercises** with an entry in `checks.json` (keyed by exercise id):
  - `input`, `choice`, `true_false`: the exercise `code` (or `code` from the check)
    is run; stdout must equal `out` (for `input` it must also match one of the
    `answers`), or with `"err": true` it must fail.
  - `gap`: the right option must print `out`; every wrong option must fail or print
    something else.
  - `bug`: the code as shown must fail or print something else; with the bug line
    replaced by `fix` it must print `out`.
  - `order`: the lines in the given order must print `out`.
  - `blocks`: the tokens joined with spaces must print `out`.
  - `extra`: additional programs (`src`) for facts stated in options or statements.
- **Lesson claims** (`_claims`, per unit): statements in lesson texts, tips and
  warnings, e.g. that a missing `;` is a parse error, that `strlen("é")` is `2`, or
  that reading a private property fails. `diag` must appear in the diagnostics.

Snippets without `<?php` get `<?php` prepended; `tpl` (with the marker `{{CODE}}`,
indented like the marker) wraps code that needs surrounding lines.

Results are cached in `.cache.json` (not committed), keyed by backend and the
SHA-256 of the program. Wandbox requests are sent one at a time with a pause of
one second.

Not checked by code (review by reading): the `pairs` exercises and the knowledge
questions `hello-1`, `functions-4` and `classes-1`.

Last full run: PHP 8.5.12 (Homebrew, `--local`), because Wandbox returned
internal errors at the time: 33 lesson blocks, 85 exercises, 16 lesson claims,
197 programs, all passing.
