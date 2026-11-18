# Verifying the Machine code course

`verify.py` checks every lesson code block and every exercise of `../course.json`.

## Setup

```bash
brew install nasm                       # nasm + ndisasm (3.02 was used)
python3.12 -m venv courses/machine-code/verify/.venv
courses/machine-code/verify/.venv/bin/pip install capstone unicorn
courses/machine-code/verify/.venv/bin/python courses/machine-code/verify/verify.py
```

`xxd` must be installed too (it comes with macOS and most Linux systems).
`--offline` uses only cached real-CPU results (see below) and fails if one is missing.

## What it checks

- **Lesson code blocks**, by kind:
  - Python (`print(...)`): run with the venv Python; stdout must equal `output`.
  - NASM (first line `bits 64`) with a hex `output`: assembled with `nasm -f bin`;
    the bytes of each source line (from the NASM listing) must equal the output
    lines. Capstone then disassembles the bytes and NASM assembles that text again
    (round trip), which must give the same bytes.
  - NASM with a text `output` (the "Hi" program): run in Unicorn (with the Linux
    system calls `write` and `exit` emulated) and on a real x86-64 Linux machine;
    what it prints must equal `output`.
  - The `xxd` hex dump and the `ndisasm -b 64` listing (no `output`) are produced
    again from the program before them and must be equal.
- **Exercises**: every exercise has its own function in `CHECKS` (keyed by id).
  Inputs are compared with the first accepted answer (normalized like the app
  does). `choice` and `gap` check that the right option gives the expected result
  and every wrong option does not. `bug` checks that exactly the given line is
  wrong (comments with bytes or results are compared with NASM/Python). `order`
  checks that the given order gives the result and that no other order does.
  `pairs` and knowledge questions check the stated facts with NASM, capstone or
  Unicorn. Exercise code without `bits 64` gets it added.
- **Lesson claims** (`lesson_claims`): facts from the texts, e.g. `mov rax, 1`
  gives `B8 01 00 00 00`, the short/near jump limits, `48` for 64-bit `add`.

## Real CPU

Programs whose result is a run (exit code, printed text, value of `EAX`) are also
run natively on Compiler Explorer (`cg142`, x86-64 gcc 14.2): a C program copies
the bytes into executable memory and runs them in a child process; the exact
stdout bytes and the exit code are reported back. Results are cached in
`.cache.json` (not committed), keyed by the SHA-256 of the C program. Requests
are sent one at a time with a pause of about one second.
