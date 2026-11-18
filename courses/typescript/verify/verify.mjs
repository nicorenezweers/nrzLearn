import { readFileSync, writeFileSync, mkdtempSync, rmSync } from "node:fs";
import { execFile } from "node:child_process";
import { tmpdir } from "node:os";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const course = JSON.parse(readFileSync(join(here, "..", "course.json"), "utf8"));
const checks = JSON.parse(readFileSync(join(here, "checks.json"), "utf8"));

const jobs = [];
const failures = [];
const warnings = [];
const job = (label, code, assert) => jobs.push({ label, code, assert, file: `j${String(jobs.length).padStart(4, "0")}.ts` });
const fail = (label, msg) => failures.push(`${label}: ${msg}`);
const norm = (s) => s.toLowerCase().replace(/[\s_'"`]/g, "");
const same = (a, b) => JSON.stringify([...a].sort()) === JSON.stringify([...b].sort());
const lines = (r) => [...new Set(r.errors)];

function expectSnippet(label, s, r) {
  const want = s.errors ?? [];
  if (!same(lines(r), want)) fail(label, `type errors on lines ${JSON.stringify(lines(r))}, expected ${JSON.stringify(want)}`);
  if (s.output !== undefined && r.stdout !== s.output) fail(label, `output ${JSON.stringify(r.stdout)}, expected ${JSON.stringify(s.output)}`);
}

for (const u of course.units) {
  u.lesson.forEach((b, i) => {
    if (b.type !== "code") return;
    const key = `${u.id}.lesson.${i}`;
    const meta = checks.lessons[key] ?? {};
    job(key, b.code, (r) => {
      expectSnippet(key, { errors: meta.errors ?? [] }, r);
      if (b.output !== undefined) {
        if (r.stdout !== b.output) fail(key, `output ${JSON.stringify(r.stdout)}, expected ${JSON.stringify(b.output)}`);
        if (r.code !== 0) fail(key, `exit code ${r.code}: ${r.stderr.split("\n")[0]}`);
      } else if (r.stdout !== "") fail(key, `prints ${JSON.stringify(r.stdout)} but has no output`);
    });
  });

  for (const e of u.exercises) {
    const id = e.id;
    const m = checks.exercises[id] ?? {};
    const snippets = (m.snippets ?? []).map((s) => ({ ...s, code: s.code.replace("@CODE", e.code ?? "") }));
    snippets.forEach((s, k) => job(`${id} snippet ${k}`, s.code, (r) => expectSnippet(`${id} snippet ${k}`, s, r)));

    switch (e.type) {
      case "input":
        if (!e.code) { if (!snippets.length) warnings.push(`${id}: input without code and without snippets`); break; }
        job(id, e.code, (r) => {
          expectSnippet(id, { errors: m.errors ?? [] }, r);
          if (!e.answers.some((a) => norm(a) === norm(r.stdout))) fail(id, `output ${JSON.stringify(r.stdout)} matches no answer ${JSON.stringify(e.answers)}`);
          if (r.stdout !== e.answers[0]) warnings.push(`${id}: first answer ${JSON.stringify(e.answers[0])} differs in spelling from output ${JSON.stringify(r.stdout)}`);
        });
        break;
      case "choice":
        if (m.answerOutput) {
          job(id, e.code, (r) => expectSnippet(id, { errors: [], output: e.options[e.answer].replace(/`/g, "") }, r));
        } else if (!snippets.length) warnings.push(`${id}: choice without check`);
        break;
      case "true_false":
        if (!snippets.length) warnings.push(`${id}: true_false without check`);
        break;
      case "gap":
        e.options.forEach((o, k) => {
          const label = `${id} option ${k} (${o})`;
          job(label, e.code.replace("___", o), (r) => {
            const ok = r.errors.length === 0 && r.code === 0 && (m.output === undefined || r.stdout === m.output);
            if (k === e.answer && !ok) fail(label, `right option fails: errors ${JSON.stringify(lines(r))}, output ${JSON.stringify(r.stdout)}`);
            if (k !== e.answer && ok) fail(label, "wrong option type-checks and gives the expected output");
            if (k !== e.answer && r.errors.length === 0 && m.output === undefined) warnings.push(`${label}: wrong option type-checks; add an expected output`);
          });
        });
        break;
      case "bug": {
        const suffix = m.suffix ? "\n" + m.suffix : "";
        job(`${id} buggy`, e.lines.join("\n") + suffix, (r) => {
          if (!r.errors.length) fail(`${id} buggy`, "no type error");
          else if (!same(lines(r), [e.bug + 1])) fail(`${id} buggy`, `type errors on lines ${JSON.stringify(lines(r))}, expected only ${e.bug + 1}`);
        });
        if (m.fix === undefined) { warnings.push(`${id}: bug without fix`); break; }
        const fixed = [...e.lines];
        fixed[m.fixAt ?? e.bug] = m.fix;
        job(`${id} fixed`, fixed.join("\n") + suffix, (r) => {
          expectSnippet(`${id} fixed`, { errors: [], output: m.output }, r);
          if (r.code !== 0) fail(`${id} fixed`, `exit code ${r.code}`);
        });
        break;
      }
      case "order": {
        job(id, e.lines.join("\n"), (r) => expectSnippet(id, { errors: [], output: m.output }, r));
        const seen = new Set([e.lines.join("\n")]);
        for (let i = 0; i < e.lines.length; i++) {
          for (let j = 0; j < e.lines.length; j++) {
            const v = [...e.lines];
            const [x] = v.splice(i, 1);
            v.splice(j, 0, x);
            const c = v.join("\n");
            if (seen.has(c)) continue;
            seen.add(c);
            job(`${id} move ${i}->${j}`, c, (r) => {
              if (r.errors.length === 0 && r.code === 0 && r.stdout === m.output) fail(`${id} move ${i}->${j}`, "another order also works");
            });
          }
        }
        break;
      }
      case "blocks":
        job(id, m.wrap.replace("$", e.tokens.join(" ")), (r) => expectSnippet(id, { errors: [], output: m.output }, r));
        break;
    }

    for (const c of [e.code, ...(e.lines ?? [])].filter(Boolean)) {
      if (/\/\/|\/\*/.test(c)) warnings.push(`${id}: code contains a comment`);
    }
  }
  for (const b of u.lesson) if (b.type === "code" && /\/\/|\/\*/.test(b.code)) warnings.push(`${u.id}: lesson code contains a comment`);
}

const dir = mkdtempSync(join(tmpdir(), "ts-course-"));
for (const j of jobs) writeFileSync(join(dir, j.file), j.code + "\n");

const run = (cmd, args) => new Promise((res) => {
  execFile(cmd, args, { cwd: dir, env: { ...process.env, NO_COLOR: "1" }, maxBuffer: 64 << 20 }, (err, stdout, stderr) => {
    res({ code: err ? (typeof err.code === "number" ? err.code : 1) : 0, stdout, stderr });
  });
});

const errors = new Map(jobs.map((j) => [j.file, []]));
let pending = jobs.map((j) => j.file);
while (pending.length) {
  const r = await run("deno", ["check", "--quiet", "--no-lock", "--no-config", ...pending]);
  const parse = !/^TS\d+ \[ERROR\]/m.test(r.stderr) && r.stderr.match(/^error: [\s\S]*?(j\d{4}\.ts):(\d+):\d+/m);
  if (parse) {
    errors.get(parse[1]).push(Number(parse[2]));
    pending = pending.filter((f) => f !== parse[1]);
    continue;
  }
  for (const chunk of r.stderr.split(/^(?=TS\d+ \[ERROR\])/m)) {
    const at = chunk.match(/^TS\d+ \[ERROR\][\s\S]*?at file:\/\/\S*?(j\d{4}\.ts):(\d+):\d+/);
    if (at) errors.get(at[1]).push(Number(at[2]));
  }
  if (r.code !== 0 && ![...errors.values()].some((e) => e.length)) {
    console.error(r.stderr);
    process.exit(2);
  }
  break;
}

let next = 0;
const results = new Map();
await Promise.all(Array.from({ length: 8 }, async () => {
  while (next < jobs.length) {
    const j = jobs[next++];
    const r = await run("node", [j.file]);
    results.set(j.file, { ...r, stdout: r.stdout.replace(/\n$/, ""), errors: errors.get(j.file) });
  }
}));
for (const j of jobs) j.assert(results.get(j.file));
rmSync(dir, { recursive: true, force: true });

const lessonRuns = jobs.filter((j) => j.label.includes(".lesson.")).length;
for (const w of warnings) console.log(`warning: ${w}`);
for (const f of failures) console.log(`FAIL ${f}`);
console.log(`${jobs.length} snippets type-checked and run (${lessonRuns} lesson blocks), ${failures.length} failures, ${warnings.length} warnings`);
process.exit(failures.length ? 1 : 0);
