# Verifying the Solidity course

`verify.mjs` compiles every program with solc (npm package `solc`, written for
0.8.38) and runs it in a local EVM (`@ethereumjs/evm` 10). ABI encoding and
decoding uses `ethers` 6. Keep these packages outside the repository and point
`SOL_DEPS` at the folder:

```bash
mkdir -p /tmp/soldeps && (cd /tmp/soldeps && npm init -y && npm install solc ethers @ethereumjs/evm @ethereumjs/util)
SOL_DEPS=/tmp/soldeps node courses/solidity/verify/verify.mjs              # all units
SOL_DEPS=/tmp/soldeps node courses/solidity/verify/verify.mjs loops tokens # only some units
```

## Output convention

A program is deployed fresh from `you`, with the constructor arguments in
`deploy`. Then the calls in `calls` run in order (`bob: f(1)` is a call from
Bob). `you` = `0x5B38Da6a701c568545dCfcB03FcB875f56beddC4`, `bob` =
`0xAb8483F64d9C6d1EcF9b849Ae677dD3315835cb2` (the first two Remix accounts).
Each call adds lines to the output:

- every emitted event: `event Name(arg, arg)`;
- a reverted call: `revert: <message>`, `revert: ErrorName(args)` or
  `revert: Panic(0x11)`;
- otherwise the return values, separated by `, ` (none: no line). Text has no
  quotes, arrays are `[1, 2]`.

## What is checked

- **Lesson code blocks** with an `output`: the calls come from
  `checks.json` (`lesson:<unit>:<n>` = the n-th block with output). The output
  must match exactly, and the compiler must report no warning other than the
  missing SPDX line.
- **Exercises** with an entry in `checks.json` (keyed by id), same rules as the
  other courses:
  - `input`, `choice`, `true_false`: the code (or `src`) must give `out`, or with
    `"err": true` must not compile. For `input`, one answer must match `out`
    (its last line with `"last": true`).
  - `gap`: the right option gives `out`; every wrong option fails to compile or
    gives a different output.
  - `bug`: the lines as shown fail or differ; with the bug line replaced by
    `fix` they give `out`.
  - `order`: the lines give `out`; every swap of two neighbouring lines fails or
    differs (`swapOk` lists swaps of lines that only differ in indentation).
  - `blocks`: the tokens joined with spaces give `out`.
  - `extra`: more programs with `out` or `"err": true` for facts in options and
    statements.

Snippets without `contract` are wrapped in `contract Test { ... }` with the
pragma; `tpl` (with `{{CODE}}`) overrides this. Compile results are cached in
`.cache.json` (not committed).

Not checked by code: the `pairs` exercises and the knowledge questions
`hello-1`, `hello-12` and `events-4`.
