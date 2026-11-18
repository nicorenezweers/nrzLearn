# Verifying the R course

`run.mjs` runs every R snippet of `../course.json` with [webR](https://docs.r-wasm.org/webr/) (R 4.x compiled to WebAssembly) in Node:

- every lesson code block (output compared exactly, blocks without `output` must print nothing),
- every `input` answer (first answer exact, the others must match after the app's normalisation),
- `choice`, `true_false`, `gap`, `blocks`, `order` and `bug` exercises using the expectations in `checks.json`:
  - `gap`: every option is put into the code; only the right one may print `expect`,
  - `order`: the right order must print `expect`, and (up to 7 lines) no other permutation may,
  - `bug`: the program with the `fixed` line must print `expect`, the original must not,
  - `runs`: extra programs with their expected output or an expected error.

`pairs` exercises are checked by hand.

```bash
cd /tmp/webr-cache && npm install webr      # once, outside the repository
WEBR_MODULE=/tmp/webr-cache/node_modules/webr/dist/webr.mjs node courses/r/verify/run.mjs
```

Each snippet runs in a clean global environment with autoprint, like `Rscript`.
