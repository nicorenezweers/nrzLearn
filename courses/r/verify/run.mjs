// Runs every R snippet of courses/r/course.json with webR and checks outputs and answers.
// Usage: node courses/r/verify/run.mjs   (set WEBR_MODULE to the path of webr's dist/webr.mjs if not installed here)
import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const course = JSON.parse(readFileSync(join(here, '..', 'course.json'), 'utf8'));
const checks = JSON.parse(readFileSync(join(here, 'checks.json'), 'utf8'));
const { WebR } = await import(process.env.WEBR_MODULE || 'webr');

const webR = new WebR();
await webR.init();
const shelter = await new webR.Shelter();
let runs = 0, failures = 0;
const cache = new Map();

async function run(code) {
  if (cache.has(code)) return cache.get(code);
  runs++;
  await webR.evalRVoid('rm(list = ls(all.names = TRUE))');
  let res;
  try {
    const r = await shelter.captureR(code, { withAutoprint: true, captureStreams: true, captureConditions: true });
    const out = r.output.filter(o => o.type === 'stdout').map(o => o.data).join('\n').replace(/\n+$/, '');
    res = { out, err: null };
  } catch (e) {
    res = { out: null, err: e.message.split('\n')[0] };
  }
  await shelter.purge();
  cache.set(code, res);
  return res;
}

function fail(where, msg) { failures++; console.log(`FAIL ${where}: ${msg}`); }
const ok = (r, expect) => !r.err && r.out === expect;
const show = r => r.err ? `ERROR ${r.err}` : JSON.stringify(r.out);
const strip = s => s.replace(/^`|`$/g, '');
const norm = s => s.toLowerCase().replace(/[\s_"'`]/g, '');

function* permutations(a) {
  if (a.length <= 1) { yield a; return; }
  for (let i = 0; i < a.length; i++)
    for (const p of permutations([...a.slice(0, i), ...a.slice(i + 1)])) yield [a[i], ...p];
}

for (const u of course.units) {
  for (const [i, b] of u.lesson.entries()) {
    if (b.type !== 'code') continue;
    const r = await run(b.code);
    const expect = b.output ?? '';
    if (!ok(r, expect)) fail(`${u.id}.lesson.${i}`, `expected ${JSON.stringify(expect)}, got ${show(r)}`);
  }
  for (const e of u.exercises) {
    const c = checks[e.id] || {};
    const where = e.id;
    let checked = false;
    for (const x of c.runs || []) {
      checked = true;
      const r = await run(x.code);
      if (x.error ? !r.err : !ok(r, x.expect)) fail(where, `run ${JSON.stringify(x.code)}: expected ${x.error ? 'an error' : JSON.stringify(x.expect)}, got ${show(r)}`);
    }
    if (e.type === 'input') {
      checked = true;
      const r = await run(e.code);
      if (!ok(r, e.answers[0])) fail(where, `answer ${JSON.stringify(e.answers[0])}, got ${show(r)}`);
      for (const a of e.answers.slice(1))
        if (r.out !== null && norm(a) !== norm(r.out) && norm(a) !== norm(r.out.replace(/^\[1\]/, '')))
          fail(where, `alternative answer ${JSON.stringify(a)} does not match ${show(r)}`);
    }
    if (e.type === 'choice' && c.auto) {
      checked = true;
      const r = await run(e.code);
      e.options.forEach((o, k) => {
        const same = !r.err && strip(o) === r.out;
        if ((k === e.answer) !== same) fail(where, `option ${k} ${o} vs output ${show(r)}`);
      });
    }
    if (e.type === 'gap') {
      checked = true;
      for (const [k, o] of e.options.entries()) {
        const r = await run(e.code.replace('___', o));
        const good = ok(r, c.expect);
        if ((k === e.answer) !== good) fail(where, `option ${k} ${JSON.stringify(o)}: got ${show(r)}, expected ${k === e.answer ? '' : 'NOT '}${JSON.stringify(c.expect)}`);
      }
    }
    if (e.type === 'blocks') {
      checked = true;
      const code = [c.prelude, e.tokens.join(' '), c.suffix].filter(Boolean).join('\n');
      const r = await run(code);
      if (!ok(r, c.expect)) fail(where, `${JSON.stringify(code)} gave ${show(r)}`);
    }
    if (e.type === 'order') {
      checked = true;
      const r = await run(e.lines.join('\n'));
      if (!ok(r, c.expect)) fail(where, `right order gave ${show(r)}`);
      if (e.lines.length <= 7) {
        let n = 0;
        for (const p of permutations(e.lines)) if (ok(await run(p.join('\n')), c.expect)) n++;
        if (n !== 1) fail(where, `${n} orders print the expected output`);
      } else console.log(`note ${where}: too many lines for the permutation check`);
    }
    if (e.type === 'bug' && c.fixed !== undefined) {
      checked = true;
      if (!c.skip_original) {
        const r0 = await run(e.lines.join('\n'));
        if (ok(r0, c.expect)) fail(where, `buggy program already prints the expected output`);
      }
      const lines = [...e.lines]; lines[e.bug] = c.fixed;
      const r = await run(lines.join('\n'));
      if (!ok(r, c.expect)) fail(where, `fixed program gave ${show(r)}`);
    }
    if (!checked) console.log(`note ${where}: ${e.type}, checked by hand`);
  }
}
const version = await webR.evalRString('R.version.string');
console.log(`${version} (webR); ${runs} snippets run, ${failures} failures`);
webR.close();
process.exit(failures ? 1 : 0);
