# Verifying the Perl course

`verify.py` runs the course code with the local `perl` (5.34.1 was used) and online
with Perl 5.44.0. Only Python 3 and `perl` are needed.

```bash
python3 courses/perl/verify/verify.py              # local perl + Compiler Explorer (perl5440)
python3 courses/perl/verify/verify.py --wandbox    # local perl + Wandbox (perl-5.44.0)
python3 courses/perl/verify/verify.py --local      # local perl only
python3 courses/perl/verify/verify.py hashes subs  # only some units
```

A program passes when it exits with code 0, prints nothing on stderr (no warnings) and
its stdout equals the expected output plus a final newline. With `"warn": true` a
warning is expected; with `"err": true` the program must fail (compile error or `die`).
Compiler Explorer returns stdout as lines, so the online run cannot see a missing final
newline; the local run checks it exactly.

What it checks:

- **Lesson code blocks**: each starts with `use strict; use warnings;`, and its stdout
  must equal `output` (blocks without `output` must print nothing).
- **Exercises** with an entry in `checks.json` (keyed by exercise id):
  - `input`, `choice`, `true_false` with `code`: the code is run and must give `out`
    (for `input`, `out` must also match one of the `answers`) or fail with `"err": true`.
  - `gap`: the right option must give `out`; every wrong option must fail, warn or
    print something else.
  - `bug`: the code as shown must fail, warn or print something else; with the bug line
    replaced by `fix` it must give `out`.
  - `order`: the given order must give `out`, and every other order (up to 7 lines,
    local perl only) must fail or print something else. Orders that differ only in the
    indentation of equal lines (`}` and `    }`) count as the same.
  - `blocks`: the tokens joined with spaces must give `out`; with `"distractors": true`
    each distractor in place of each token (except the final `;`) must not work.
  - `extra`: additional complete programs (`src`) for facts stated in options or
    statements, with `out`, `warn` or `err`.

Snippets are put into a template: `tpl` from the check (`{{CODE}}` marks the place),
or by default `use strict; use warnings; use feature 'say';` on three lines, followed by
the code. Code that already starts with `use strict;` runs as it is.

Online results are cached in `.cache.json` (not committed), keyed by the SHA-256 of
service and program, so reruns only send changed programs. Requests are sent one at a
time with a pause of one second.

The `pairs` exercises and the knowledge question `hello-1` have no check; review them
by reading.
