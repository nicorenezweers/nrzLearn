# Verifying the Python course

```bash
python3 courses/python/verify/verify.py                      # default python3
python3 courses/python/verify/verify.py --python python3.12  # oldest supported version
```

`verify.py` reads `../course.json` and `checks.json` and prints `All checks passed.`
or a list of problems. It runs every piece of code in a fresh interpreter
(`python -I -c`).

What it checks:

- **Lesson code blocks**: the code runs without errors, and its stdout (without the
  final newline) equals `output`. Blocks without `output` must print nothing.
- **Comments**: code outside the `hello` unit may only contain result comments (`# -> 5`).
- **input**: the code runs, prints one line, the first answer is exactly that
  line, and every answer matches it with the app's comparison (ignores case,
  spaces, `_`, quotes and backticks).
- **choice**: if all options are code spans and the exercise has `code`, the output
  must match the right option and no other one. With `options_template` in
  `checks.json`, each option is put into the template (`___`) and run: only the
  right option may print `expect`. Otherwise a `proof` is needed.
- **true_false**: `proof` must print `True` or `False`, equal to the answer.
- **gap**: each option is put into `___` (plus `setup`/`after`); the right one must
  print `expect`, every wrong one must fail or print something else.
- **blocks**: the tokens joined with spaces must print `expect`; no distractor may
  replace a token and still print `expect`; no other order of the tokens may
  print `expect` (up to 8 tokens).
- **order**: the lines must print `expect`, and no other order may (checked for all
  permutations in-process with a step limit).
- **bug**: the program as given must not print `expect`; with line `bug` replaced
  by `fix` it must.
- **pairs**: `proof` must print `proof_out`.

`checks.json` holds this extra data per exercise id: `expect`, `setup`, `after`,
`fix`, `options_template`, `not_output`, `proof`, `proof_out`. Inside a `proof`,
`CODE` is the exercise code and `output(code=CODE)` returns what the code prints.

When you change an exercise in `course.json`, update its entry in `checks.json`
and run the script again.
