# Verifying the Smalltalk course

`verify.py` runs the course code with GNU Smalltalk (`gst`, tested with 3.2.5 from
Homebrew: `brew install gnu-smalltalk`). Only Python 3 and `gst` are needed; set
`GST=/path/to/gst` to use another binary.

```bash
python3 courses/smalltalk/verify/verify.py              # all units
python3 courses/smalltalk/verify/verify.py loops        # only some units
```

A program counts as failing when gst writes to stderr (parse errors) or prints an
error report (`Object: ... error: ...`) on stdout; gst itself always exits with 0.

What it checks:

- **Lesson code blocks**: each one runs as a complete `.st` script; stdout must equal
  `output` exactly (blocks without `output` must print nothing).
- **`_lesson`** in `checks.json`: small programs that confirm facts stated in lesson
  texts (for example that `[:x | x * 2] value` is an error).
- **Exercises** (keyed by exercise id in `checks.json`; `tpl` wraps the code, with
  `{{CODE}}` as the marker):
  - `input`: the code must print `answers[0]` (or `out`), and the output must match an
    answer the way the app compares answers.
  - `choice` with code: the output must equal the right option (without backticks),
    or `out`; no wrong option may equal the output.
  - `true_false` with code: the output must equal `out`.
  - `gap`: the right option must print `out`; every wrong option must fail or print
    something else.
  - `bug`: the code as shown must fail or print something else; with the line replaced
    by `fix` it must print `out`.
  - `order`: the lines in order must print `out`. Every other order (all permutations
    up to 5 lines, all swaps of two lines above that) must fail or print something
    else. gst accepts `| x |` declarations anywhere at the top level of a script, but
    Smalltalk syntax puts them first (the lesson says so), so orders that move a
    leading declaration line are skipped.
  - `blocks`: the tokens joined with spaces must print `out`.
  - `extra`: additional programs (`src`) with `out` or `"err": true`, for exercises
    without code (facts stated in options or statements).

The `pairs` exercises have no check; review them by reading.

Results are cached in `.cache.json` (not committed), keyed by the SHA-256 of the gst
version and the program.

Note: the Homebrew build of gst 3.2.5 on arm64 prints wrong results for large
integers (for example `2 raisedTo: 100`) and crashes when printing some floats
(`0.1 + 0.2`). The course avoids both.
