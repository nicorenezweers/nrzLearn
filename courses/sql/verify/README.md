# Verifying the SQL course

`verify.py` runs every query of `../course.json` with the `sqlite3` command-line
tool (SQLite 3.54) and fails on the first difference. It needs only Python 3 and
`sqlite3`.

```bash
python3 courses/sql/verify/verify.py
```

## What it checks

Every query runs against a fresh in-memory copy of the sample database:

- the `books` table (the `CREATE TABLE` + `INSERT` block in the unit `select`),
- plus the `authors` table from the unit `joins` on (its setup block).

**Lesson:** every code block must run without an error. Blocks with `output`
must print exactly that output in `.mode table`; blocks without `output` must
print nothing.

**Exercises** (in `.mode list`, without headers, unless a check says otherwise):

| Type | Check |
|---|---|
| `input` | The query result (or its number of rows or columns) equals the first answer, compared like the app does (case, spaces, `_`, quotes and backticks ignored). |
| `choice`, `true_false` | The facts behind the answer, defined in `checks.py` (expected outputs, expected errors, text that only the right option contains). |
| `gap` | The right option runs; every wrong option gives an error or a different result. |
| `blocks` | The tokens joined with spaces run (plus the expected result from `checks.py`). |
| `order` | The lines in order run, and no other order of the lines gives the same result. |
| `bug` | The fixed line from `checks.py` makes the code run, and the code with the bug gives an error or a different result. |

`pairs` exercises and a few conceptual choice/true-false questions have nothing
to run; the script lists them as "checked by reading only".

## When you change the course

Run the script, then update `checks.py` if you changed an exercise. Never copy
an output into `course.json` without running it.
