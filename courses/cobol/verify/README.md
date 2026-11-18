# Verifying the COBOL course

`verify.py` compiles and runs the course code with GnuCOBOL (`cobc -x`, written
for GnuCOBOL 3.2). Only Python 3 and `cobc` are needed (`brew install gnucobol`).

```bash
python3 courses/cobol/verify/verify.py                # all units
python3 courses/cobol/verify/verify.py loops tables   # only some units
python3 courses/cobol/verify/verify.py --godbolt      # also run the lesson blocks on Compiler Explorer (GnuCOBOL 3.2)
```

The script compares stdout. A program "fails" when it does not compile, exits
with an error or runs longer than 10 seconds (an endless loop). Results are
cached in `.cache.json` (not committed).

Snippets without `PROGRAM-ID` are wrapped into a complete free-format program:
lines that start with a level number (`01`, `05`, `88` ...) go into
`WORKING-STORAGE`, all other lines into the `PROCEDURE DIVISION`, followed by
`STOP RUN.` (unless the snippet already has `STOP RUN` or has paragraphs).

What it checks:

- **Lesson code blocks**: each block with an output is a complete program that
  starts with `>>SOURCE FORMAT FREE`; its stdout must equal `output` exactly,
  it must compile without warnings, and the output has no trailing spaces.
- **Exercises** with an entry in `checks.json` (keyed by exercise id; `tpl`
  wraps the exercise code at `{{CODE}}`, `extra` lists more programs to run):
  - `input`, `choice`, `true_false`: the exercise `code` is run and must print
    `out` (for `input` it must also match one of the `answers`, for `choice` it
    must equal exactly the right option unless `no_option` is set).
  - `gap`: the right option must print `out`; every wrong option must fail,
    print something else or compile with a warning.
  - `bug`: the lines as shown must fail, print something else or warn; with the
    bug line replaced by `fix` they must print `out`.
  - `order`: the lines in the given order must print `out`; every other order
    (all permutations up to 5 lines, otherwise every swap of two neighbouring
    lines) must fail, print something else or warn.
  - `blocks`: the tokens joined with spaces must print `out`.
- Exercises without an entry (`pairs` and pure knowledge questions) are listed
  at the end for review by reading.
