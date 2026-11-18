# Verifying the Assembly course

`verify.py` assembles, links and runs the course code on real x86-64 Linux through
the Compiler Explorer API (`nasm301`, NASM 3.01 + `ld`). `emu.py` is a small local
runner (local `nasm -f bin` + the Unicorn emulator, only the system calls write and
exit); it is used for the order-uniqueness check and for fast runs while editing.

```bash
python3 courses/assembly/verify/verify.py --no-perm                     # Compiler Explorer, NASM 3.01
python3 courses/assembly/verify/verify.py --no-perm --compiler=nasm21601 # NASM 2.16.01
python3 -m venv /tmp/asm-venv && /tmp/asm-venv/bin/pip install unicorn  # once, plus `brew install nasm`
/tmp/asm-venv/bin/python courses/assembly/verify/verify.py --local       # all checks locally, with permutations
/tmp/asm-venv/bin/python courses/assembly/verify/verify.py stack         # only some units
```

What it checks:

- **Lesson code blocks**: complete programs; they must assemble without warnings,
  exit with code 0 and print exactly `output` (or nothing when there is no `output`).
- **Exercises** with an entry in `checks.json` (`checks`, keyed by exercise id):
  - `input`, `choice`, `true_false`: the exercise `code` (or `code` from the check;
    `"run": false` skips it) must print `out` and exit with `exit` (default 0); with
    `"err": true` it must fail to assemble or crash (and print `out` if given). For
    `input`, `out` must match one of the `answers`; with `"match_option": true` it must
    match exactly the right `choice` option. `options` (list of code, one per option):
    the right one must behave as expected, every other one must not.
  - `gap`: the right option must behave as expected; every wrong option must fail or
    print something else (in at least one of the `cases`).
  - `bug`: the lines as shown must behave differently; with the bug line replaced by
    `fix` they must behave as expected.
  - `order`: the lines in the given order must behave as expected, and (locally) **no
    other permutation** of the lines may do so.
  - `blocks`: the tokens joined with spaces must behave as expected; the variants in
    `wrong` must not.
  - `extra`: further programs (`src`) with their own `out` / `exit` / `err`, used to
    check facts stated in options, statements and lesson texts.

How a snippet becomes a program: code with `global _start` runs as it is. Otherwise
the part before `section .text` is data, and the instructions go into the template
`tpl` (default: `default rel`, the data, `_start:` + code + exit). `pre` and `post`
are added before and after the code, `data` and `funcs` go to `{{DATA}}` and
`{{FUNCS}}`. `show: ["rax", ...]` prints these registers after the code (or at
`{{SHOW}}`) in decimal (`hex`, `signed` for other formats). `cases` runs the same
check with several `pre`/`out` pairs.

Results from Compiler Explorer are cached in `.cache.json` (not committed), keyed by
the SHA-256 of the program; requests are sent one at a time with a pause of one
second. `pairs` exercises and a few knowledge questions (`hello-1`, `hello-3`,
`functions-1`, `functions-2`, `functions-4`) have no check; review them by reading.
