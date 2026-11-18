# Verifying the Lua course

`verify.py` runs the course code with a local Lua 5.4 (`brew install lua@5.4`;
default path `/opt/homebrew/opt/lua@5.4/bin/lua`, or `lua5.4` on the `PATH`).
Only Python 3 is needed besides Lua.

```bash
python3 courses/lua/verify/verify.py                      # all units, local Lua 5.4
python3 courses/lua/verify/verify.py loops tables         # only some units
python3 courses/lua/verify/verify.py --lua /opt/homebrew/bin/lua   # another Lua (e.g. 5.5)
python3 courses/lua/verify/verify.py --online             # also compare with Compiler Explorer (Lua 5.4.7)
```

What it checks:

- **Lesson code blocks**: each one is a complete script; it must run without error,
  and its stdout must equal `output` exactly (blocks without `output` must print
  nothing). Comments in code must only contain results (`-- 2`).
- **Exercises** with an entry in `checks.json` (keyed by exercise id). `pre` and
  `post` are code put before and after the exercise code.
  - `input`, `choice`, `true_false`: the exercise `code` is run; stdout must equal
    `out`. For `input`, `out` must match one of the `answers` (same normalization as
    the app); for `choice`, the right option (when it is code in backticks) must equal
    `out` and no wrong option may; for `true_false`, `claim` is what the statement
    says, and the answer must be `claim == out`.
  - `gap`: the right option must print `out`; every wrong option must fail or print
    something else.
  - `bug`: the lines as shown must fail (or loop forever, 2 s timeout) or print
    something else; with the bug line replaced by `fix` they must print `out`.
  - `order`: the lines in the given order must print `out`, and they must be indented
    4 spaces per block. Every other order that is also indented consistently is run
    too; none of them may print `out` (the order must be unambiguous).
  - `blocks`: the tokens joined with spaces must print `out`.
  - `extra`: additional scripts (`src`) with `out` or `"err": true`, for exercises
    without code (facts stated in a statement).
- **`_facts`** in `checks.json`: scripts that back up claims made only in the lesson
  texts (for example `#"é"` is `2`, `#t` is `0` for a table with only keys).

With `--online`, every script that is expected to succeed is also sent to Compiler
Explorer (`lua547`, one request per second, cached in `.cache.json`, not committed),
and the result must match the local run.

Not checked by code: the `pairs` exercises, `hello-1` (the command `lua hello.lua`)
and `objects-3` (the order of `pairs` is not defined); review them by reading.
