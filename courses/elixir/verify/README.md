# Verifying the Elixir course

`verify.py` runs the course code with a local `elixir` (checked with Elixir 1.20.5 on
Erlang/OTP 29; the course targets 1.17+). The online compilers were not usable
(Compiler Explorer has no Elixir executor, Wandbox was down), so Elixir was installed
with `brew install elixir`. Only Python 3 and `elixir` are needed.

```bash
python3 courses/elixir/verify/verify.py              # all units
python3 courses/elixir/verify/verify.py enum         # only some units
```

What it checks:

- **Lesson code blocks**: each one runs as a script; exit code 0, no warnings, and
  stdout must equal `output` exactly (blocks without `output` must print nothing).
- **Exercises** with an entry in `checks.json` (keyed by exercise id):
  - `input`, `choice`, `true_false`: the exercise `code` (or `code` from the check) is
    run; stdout must equal `out` (for `input` it must also match one of the `answers`),
    or with `"err": true` it must fail.
  - `gap`: the right option must print `out`; every wrong option must fail or print
    something else.
  - `bug`: the code as shown must fail or print something else; with the bug line
    replaced by `fix` it must print `out`.
  - `order`: the lines in the given order must print `out`, and **no other order** of
    the lines may print `out` (all permutations are run).
  - `blocks`: the tokens joined with spaces must print `out`; no distractor may replace
    any token and still print `out`.
  - `extra`: additional programs (`src`) with `out` or `"err": true`, used for facts in
    options or statements without code.
- **Formatting**: every snippet must be unchanged by `Code.format_string!/1` (the
  `mix format` style); for `bug` and `order` empty lines are ignored.

Snippets are put into `tpl` from the check (`{{CODE}}` is the marker); without `tpl`
the code runs as it is. Every program runs with a heap limit per Erlang process and a
20 s timeout, so wrong orders that recurse forever stop quickly.

Results are cached in `.cache.json` (not committed), keyed by the SHA-256 of the
Elixir version and the program.

The `pairs` exercises and the knowledge questions `hello-1` and `recursion-4` have no
check; review them by reading.
