# Verifying the Erlang course

`verify.py` runs the course code with a local `escript` (Erlang/OTP 26 or newer;
checked with OTP 29). Compiler Explorer cannot execute Erlang, so the code runs
locally. Only Python 3 and Erlang are needed.

```bash
python3 courses/erlang/verify/verify.py               # all units
python3 courses/erlang/verify/verify.py maps lists    # only some units
python3 courses/erlang/verify/verify.py --wandbox     # also run lesson blocks on Wandbox (Erlang 26.2)
```

What it checks:

- **Lesson code blocks**: each one is a complete escript; it must run without
  compiler warnings, and its stdout must equal `output` exactly (blocks without
  `output` must print nothing).
- **Exercises** with an entry in `checks.json` (keyed by exercise id):
  - `input`, `choice`, `true_false`: the exercise `code` (or `code` from the check)
    is run; stdout must equal `out` (for `input` it must also match one of the
    `answers`; for a "What does this print?" `choice` the right option must equal
    the output), or with `"err": true` it must fail to compile or crash.
  - `gap`: the right option must print `out`; every wrong option must fail or print
    something else.
  - `bug`: the code as shown must fail, hang (time limit) or print something else;
    with the bug line replaced by `fix` it must print `out`.
  - `order`: the lines in the given order must print `out`, and (up to 6 lines)
    **no other permutation** may print `out`.
  - `blocks`: the tokens joined with spaces must print `out`.
  - `extra`: additional programs (`src`) with `out` or `"err": true`, used for
    exercises without runnable code (facts stated in options or statements).
- Code lint: no tabs, no trailing spaces, no natural-language comments.

Snippets become complete escripts like this: a `tpl` from the check (with
`{{CODE}}` as the marker) is filled first; code that contains `main(` gets the
header (`#!/usr/bin/env escript`, `-module(t).`, `-export([main/1]).`); with
`call` the snippet's functions are kept and `main` prints `call` with `~p`;
otherwise the snippet becomes the body of `main(_)`.

`escript` prints compiler warnings (for example unused variables in snippets) to
stdout; the script removes them from the output and only reports them for lesson
blocks.

Results are cached in `.cache.json` (not committed), keyed by the SHA-256 of the
OTP version and the program.

Not checked by running: the `pairs` exercises and `recursion-4` (whether a
function is tail recursive); review them by reading.
