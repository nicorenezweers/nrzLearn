#!/usr/bin/env node
import { spawnSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import util from "node:util";
import vm from "node:vm";
import checks from "./checks.mjs";

const here = dirname(fileURLToPath(import.meta.url));
const course = JSON.parse(readFileSync(join(here, "..", "course.json"), "utf8"));
const work = mkdtempSync(join(tmpdir(), "nrzlearn-js-"));

let runs = 0;
let file = 0;

function node(code) {
  runs++;
  const path = join(work, `snippet${file++}.js`);
  writeFileSync(path, code + "\n");
  const r = spawnSync(process.execPath, [path], { cwd: work, encoding: "utf8", timeout: 3000, maxBuffer: 1 << 20 });
  const out = (r.stdout ?? "").replace(/\n$/, "");
  return { ok: r.status === 0 && !r.error, out, err: r.error ? String(r.error) : r.stderr };
}

function sandbox(code) {
  let script;
  try {
    script = new vm.Script(code);
  } catch {
    return { ok: false, out: "" };
  }
  const lines = [];
  const log = (...args) => {
    if (lines.length > 1000) throw new Error("too much output");
    lines.push(util.format(...args));
  };
  try {
    script.runInContext(vm.createContext({ console: { log } }), { timeout: 200 });
    return { ok: true, out: lines.join("\n") };
  } catch {
    return { ok: false, out: lines.join("\n") };
  }
}

function* permutations(items) {
  if (items.length <= 1) {
    yield items;
    return;
  }
  for (let i = 0; i < items.length; i++) {
    const rest = [...items.slice(0, i), ...items.slice(i + 1)];
    for (const p of permutations(rest)) yield [items[i], ...p];
  }
}

const strip = (s) => s.replace(/^`|`$/g, "");
const isCode = (s) => /^`[^`]+`$/.test(s);
const norm = (s) => s.toLowerCase().replace(/[\s_'"`]/g, "");
const onlyIndex = (flags) => (flags.filter(Boolean).length === 1 ? flags.indexOf(true) : -1);

const failures = [];
let passed = 0;
function expect(what, ok, detail = "") {
  if (ok) passed++;
  else failures.push(`${what}${detail ? `: ${detail}` : ""}`);
}

function works(code, cases, runner) {
  return cases.every((c) => {
    const r = runner(c.before + code + c.after);
    return r.ok && r.out === c.out;
  });
}

function uniqueByOutput(what, items, join, cases, rightKey) {
  const seen = new Set();
  let others = 0;
  for (const p of permutations(items)) {
    const key = join(p);
    if (key === rightKey || seen.has(key)) continue;
    seen.add(key);
    if (works(key, cases, sandbox)) {
      others++;
      console.log(`  ${what} also works:\n${key.replace(/^/gm, "    ")}`);
    }
  }
  expect(`${what}: no other order gives the expected output (${seen.size} orders tried)`, others === 0, `${others} other orders work`);
}

const commentLines = [];
function scanComments(where, code) {
  for (const line of code.split("\n")) if (line.includes("//")) commentLines.push(`${where}: ${line.trim()}`);
}

for (const unit of course.units) {
  unit.lesson.forEach((b, i) => {
    if (b.type !== "code") return;
    const where = `${unit.id}.lesson.${i}`;
    scanComments(where, b.code);
    const r = node(b.code);
    expect(`${where} runs without error`, r.ok, r.err);
    expect(`${where} output`, r.out === (b.output ?? ""), `\n--- expected\n${b.output}\n--- got\n${r.out}`);
  });

  for (const e of unit.exercises) {
    const spec = checks[e.id] ?? {};
    const ctx = { e, node, sandbox, strip };
    const before = spec.before ?? "";
    const after = spec.after ?? "";
    if (e.code) scanComments(e.id, e.code);
    for (const l of e.lines ?? []) scanComments(e.id, l);

    switch (e.type) {
      case "input": {
        const expected = spec.expected ? spec.expected(ctx) : (() => {
          const r = node(e.code);
          expect(`${e.id} code runs`, r.ok, r.err);
          return r.out;
        })();
        expect(`${e.id} first answer equals the output`, norm(e.answers[0]) === norm(expected), `answer ${JSON.stringify(e.answers[0])}, output ${JSON.stringify(expected)}`);
        expect(`${e.id} output fits in one line`, !expected.includes("\n"));
        break;
      }
      case "choice": {
        let pick;
        if (spec.pick) {
          pick = spec.pick(ctx);
        } else if (e.code && e.options.every((o) => isCode(o) || o === "An error")) {
          const r = node(e.code);
          pick = onlyIndex(e.options.map((o) => (o === "An error" ? !r.ok : r.ok && strip(o) === r.out)));
        } else {
          failures.push(`${e.id}: choice needs a check in checks.mjs`);
          break;
        }
        expect(`${e.id} answer`, pick === e.answer, `checked ${pick}, course says ${e.answer}`);
        break;
      }
      case "true_false": {
        if (!spec.holds) {
          failures.push(`${e.id}: true_false needs a check in checks.mjs`);
          break;
        }
        const holds = spec.holds(ctx);
        expect(`${e.id} answer`, holds === e.answer, `checked ${holds}, course says ${e.answer}`);
        break;
      }
      case "gap": {
        e.options.forEach((o, i) => {
          const r = node(before + e.code.replace("___", o) + after);
          const right = r.ok && r.out === spec.out;
          expect(`${e.id} option ${i} (${o}) ${i === e.answer ? "gives" : "does not give"} the expected output`, right === (i === e.answer), JSON.stringify(r.out));
        });
        break;
      }
      case "blocks":
      case "order": {
        const cases = (spec.cases ?? [{}]).map((c) => ({ before: c.before ?? before, after: c.after ?? after, out: c.out ?? spec.out }));
        const items = e.type === "blocks" ? e.tokens : e.lines;
        const join = (p) => p.join(e.type === "blocks" ? " " : "\n");
        const code = join(items);
        expect(`${e.id} right order gives the expected output`, works(code, cases, node), code);
        if (items.length <= 8) uniqueByOutput(e.id, items, join, cases, code);
        break;
      }
      case "bug": {
        const broken = node(before + e.lines.join("\n") + after);
        expect(`${e.id} original code does not give the expected output`, !(broken.ok && broken.out === spec.out));
        const fixed = e.lines.map((l, i) => (i === e.bug ? spec.fix : l));
        expect(`${e.id} fix changes the bug line`, spec.fix !== undefined && spec.fix !== e.lines[e.bug]);
        const r = node(before + fixed.join("\n") + after);
        expect(`${e.id} fixed code gives the expected output`, r.ok && r.out === spec.out, `${JSON.stringify(r.out)} ${r.err}`);
        break;
      }
      case "pairs": {
        if (!spec.check) {
          failures.push(`${e.id}: pairs needs a check in checks.mjs`);
          break;
        }
        expect(`${e.id} pairs`, spec.check(ctx) === true);
        break;
      }
      default:
        failures.push(`${e.id}: unknown type ${e.type}`);
    }
  }
}

rmSync(work, { recursive: true, force: true });

if (commentLines.length) {
  console.log("Lines with // (must only contain code or results):");
  for (const l of commentLines) console.log(`  ${l}`);
}
console.log(`${passed} checks passed, ${failures.length} failed, ${runs} node runs`);
for (const f of failures) console.log(`FAIL ${f}`);
process.exit(failures.length ? 1 : 0);
