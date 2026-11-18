import { createRequire } from 'node:module';
import { readFileSync, writeFileSync, existsSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const deps = process.env.SOL_DEPS;
if (!deps) {
  console.error('Set SOL_DEPS to a folder with node_modules/{solc,ethers,@ethereumjs/evm,@ethereumjs/util}');
  process.exit(2);
}
const require = createRequire(join(resolve(deps), 'package.json'));
const solc = require('solc');
const { createEVM } = require('@ethereumjs/evm');
const { Address, hexToBytes, bytesToHex } = require('@ethereumjs/util');
const { Interface } = require('ethers');

const course = JSON.parse(readFileSync(join(here, '..', 'course.json'), 'utf8'));
const checks = JSON.parse(readFileSync(join(here, 'checks.json'), 'utf8'));
const only = process.argv.slice(2).filter((a) => !a.startsWith('--'));

const ACCOUNTS = {
  you: '0x5B38Da6a701c568545dCfcB03FcB875f56beddC4',
  bob: '0xAb8483F64d9C6d1EcF9b849Ae677dD3315835cb2',
};
const PRAGMA = 'pragma solidity ^0.8.20;';
const CALL_GAS = 3_000_000n;

const cachePath = join(here, '.cache.json');
const cache = existsSync(cachePath) ? JSON.parse(readFileSync(cachePath, 'utf8')) : {};
const sha = (s) => createHash('sha256').update(s).digest('hex');

function compile(src) {
  const key = solc.version() + ':' + sha(src);
  if (cache[key]) return cache[key];
  const input = {
    language: 'Solidity',
    sources: { 'Main.sol': { content: src } },
    settings: { evmVersion: 'cancun', outputSelection: { '*': { '*': ['abi', 'evm.bytecode.object'] } } },
  };
  const out = JSON.parse(solc.compile(JSON.stringify(input)));
  const errs = (out.errors || []).filter((e) => e.severity === 'error').map((e) => e.formattedMessage.trim());
  const warns = (out.errors || [])
    .filter((e) => e.severity !== 'error' && !/SPDX license identifier/.test(e.message))
    .map((e) => e.formattedMessage.trim());
  const res = { errs, warns, contracts: {} };
  if (!errs.length) {
    for (const [name, c] of Object.entries(out.contracts['Main.sol'])) {
      res.contracts[name] = { abi: c.abi, bin: c.evm.bytecode.object };
    }
  }
  cache[key] = res;
  return res;
}

function splitTop(s) {
  const parts = [];
  let depth = 0, cur = '', q = false;
  for (const ch of s) {
    if (q) { cur += ch; if (ch === '"') q = false; continue; }
    if (ch === '"') { q = true; cur += ch; continue; }
    if (ch === '[' || ch === '(') depth++;
    if (ch === ']' || ch === ')') depth--;
    if (ch === ',' && depth === 0) { parts.push(cur.trim()); cur = ''; continue; }
    cur += ch;
  }
  if (cur.trim() !== '') parts.push(cur.trim());
  return parts;
}

function parseArg(a) {
  if (a.startsWith('"')) return JSON.parse(a);
  if (a === 'true') return true;
  if (a === 'false') return false;
  if (a in ACCOUNTS) return ACCOUNTS[a];
  if (a.startsWith('[')) return splitTop(a.slice(1, -1)).map(parseArg);
  if (/^0x[0-9a-fA-F]{40}$/.test(a)) return a;
  if (/^-?\d+$/.test(a)) return BigInt(a);
  throw new Error('cannot parse argument ' + a);
}

function parseCall(s) {
  const m = s.match(/^(?:(\w+):\s*)?(\w+)\((.*)\)$/);
  if (!m) throw new Error('bad call ' + s);
  return { from: m[1] || 'you', fn: m[2], args: splitTop(m[3]).map(parseArg) };
}

function fmt(v, t) {
  if (t.baseType === 'array') return '[' + [...v].map((x) => fmt(x, t.arrayChildren)).join(', ') + ']';
  if (t.baseType === 'tuple') return '(' + [...v].map((x, i) => fmt(x, t.components[i])).join(', ') + ')';
  if (typeof v === 'bigint') return v.toString();
  return String(v);
}
const fmtList = (vals, params) => params.map((p, i) => fmt(vals[i], p)).join(', ');

function fmtRevert(iface, ret, exc) {
  const hex = bytesToHex(ret);
  if (hex === '0x') return exc.error === 'revert' ? 'revert' : 'revert: ' + exc.error;
  if (hex.startsWith('0x4e487b71')) {
    const code = BigInt('0x' + hex.slice(10));
    return 'revert: Panic(0x' + code.toString(16).padStart(2, '0') + ')';
  }
  const e = iface.parseError(hex);
  if (!e) return 'revert: ' + hex;
  if (e.name === 'Error') return 'revert: ' + e.args[0];
  return 'revert: ' + e.name + '(' + fmtList(e.args, e.fragment.inputs) + ')';
}

const addr = (n) => new Address(hexToBytes(ACCOUNTS[n] || n));

async function run(src, spec = {}) {
  const c = compile(src);
  if (c.errs.length) return { err: true, msg: c.errs[0], warns: c.warns };
  const names = [...src.matchAll(/^\s*contract\s+(\w+)/gm)].map((m) => m[1]);
  const name = spec.contract || names[names.length - 1];
  const k = c.contracts[name];
  if (!k) return { err: true, msg: 'no contract ' + name };
  const iface = new Interface(k.abi);
  const evm = await createEVM();
  const deployArgs = (spec.deploy || []).map((a) => parseArg(String(a)));
  const data = '0x' + k.bin + (deployArgs.length ? iface.encodeDeploy(deployArgs).slice(2) : '');
  const d = await evm.runCall({ caller: addr('you'), data: hexToBytes(data), gasLimit: 30_000_000n });
  if (d.execResult.exceptionError) return { err: false, out: 'deploy failed: ' + fmtRevert(iface, d.execResult.returnValue, d.execResult.exceptionError), warns: c.warns };
  const to = d.createdAddress;
  const lines = [];
  for (const cs of spec.calls || []) {
    const call = parseCall(cs);
    const f = iface.getFunction(call.fn);
    if (!f) return { err: false, out: 'no function ' + call.fn, warns: c.warns };
    let calldata;
    try { calldata = iface.encodeFunctionData(f, call.args); } catch (e) { return { err: false, out: 'bad args for ' + cs + ': ' + e.shortMessage, warns: c.warns }; }
    const r = await evm.runCall({ caller: addr(call.from), to, data: hexToBytes(calldata), gasLimit: CALL_GAS });
    const ex = r.execResult;
    if (ex.exceptionError) { lines.push(fmtRevert(iface, ex.returnValue, ex.exceptionError)); continue; }
    for (const l of ex.logs || []) {
      const p = iface.parseLog({ topics: l[1].map(bytesToHex), data: bytesToHex(l[2]) });
      lines.push('event ' + p.name + '(' + fmtList(p.args, p.fragment.inputs) + ')');
    }
    if (f.outputs.length) lines.push(fmtList(iface.decodeFunctionResult(f, bytesToHex(ex.returnValue)), f.outputs));
  }
  return { err: false, out: lines.join('\n'), warns: c.warns };
}

function indent(code, ind) {
  return code.split('\n').map((l) => (l ? ind + l : l)).join('\n');
}
function wrap(code, tpl) {
  if (tpl) {
    return tpl.replace(/^([ \t]*)\{\{CODE\}\}/m, (_, ind) => indent(code, ind));
  }
  if (/^\s*contract\s/m.test(code)) return code.includes('pragma') ? code : PRAGMA + '\n\n' + code;
  return PRAGMA + '\n\ncontract Test {\n' + indent(code, '    ') + '\n}';
}

const norm = (s) => s.toLowerCase().replace(/[\s_"'`]/g, '');
let fails = 0, passes = 0;
const fail = (id, msg) => { fails++; console.log('FAIL', id, msg); };
const pass = () => { passes++; };

async function expectOut(id, src, spec, out, strict = true) {
  const r = await run(src, spec);
  if (r.err) return fail(id, 'compile error: ' + r.msg + '\n' + src);
  if (strict && r.warns.length) return fail(id, 'warning: ' + r.warns[0]);
  if (r.out !== out) return fail(id, `output\n--- expected\n${out}\n--- got\n${r.out}\n--- src\n${src}`);
  pass();
}
async function expectErr(id, src, spec) {
  const r = await run(src, spec);
  if (!r.err) return fail(id, 'expected compile error, got output:\n' + r.out + '\n' + src);
  pass();
}
async function expectDiff(id, src, spec, out) {
  const r = await run(src, spec);
  if (!r.err && r.out === out) return fail(id, 'expected compile error or different output:\n' + src);
  pass();
}

const STYLE = (id, code) => {
  if (/\t/.test(code)) fail(id, 'tab in code');
  if (/ +$/m.test(code)) fail(id, 'trailing space in code');
};

for (const unit of course.units) {
  if (only.length && !only.includes(unit.id)) continue;
  let k = 0;
  for (const b of unit.lesson) {
    if (b.type !== 'code') continue;
    STYLE(unit.id + ' lesson', b.code);
    if (b.output === undefined) {
      const r = await run(wrap(b.code), {});
      if (r.err) fail(`${unit.id} lesson block (no output)`, r.msg);
      else pass();
      continue;
    }
    k++;
    const id = `lesson:${unit.id}:${k}`;
    const spec = checks[id];
    if (!spec) { fail(id, 'no calls in checks.json'); continue; }
    await expectOut(id, b.code, spec, b.output);
  }
  for (const ex of unit.exercises) {
    const ck = checks[ex.id];
    if (ex.code) STYLE(ex.id, ex.code);
    if (!ck) { if (ex.type !== 'pairs') console.log('SKIP', ex.id, '(no check, review by reading)'); continue; }
    const id = ex.id;
    if (ck.out !== undefined || ck.err) {
      const shown = ck.last && ck.out !== undefined ? ck.out.split('\n').pop() : ck.out;
      if (ex.type === 'input' && ck.out !== undefined && !ex.answers.some((a) => norm(a) === norm(shown))) {
        fail(id, `no answer matches output ${JSON.stringify(shown)}`);
      }
      if (ex.type === 'gap') {
        for (let i = 0; i < ex.options.length; i++) {
          const src = wrap(ex.code.replace('___', ex.options[i]), ck.tpl);
          if (i === ex.answer) await expectOut(`${id} [right ${ex.options[i]}]`, src, ck, ck.out);
          else await expectDiff(`${id} [wrong ${ex.options[i]}]`, src, ck, ck.out);
        }
      } else if (ex.type === 'bug') {
        const lines = [...ex.lines];
        await expectDiff(`${id} [as shown]`, wrap(lines.join('\n'), ck.tpl), ck, ck.out);
        lines[ex.bug] = ck.fix;
        await expectOut(`${id} [fixed]`, wrap(lines.join('\n'), ck.tpl), ck, ck.out);
      } else if (ex.type === 'order') {
        const src = wrap(ex.lines.join('\n'), ck.tpl);
        await expectOut(`${id} [order]`, src, ck, ck.out);
        for (let i = 0; i + 1 < ex.lines.length; i++) {
          if ((ck.swapOk || []).includes(i)) continue;
          const l = [...ex.lines];
          [l[i], l[i + 1]] = [l[i + 1], l[i]];
          await expectDiff(`${id} [swap ${i}/${i + 1}]`, wrap(l.join('\n'), ck.tpl), ck, ck.out);
        }
      } else if (ex.type === 'blocks') {
        const src = wrap(ex.tokens.join(' '), ck.tpl);
        await expectOut(`${id} [blocks]`, src, ck, ck.out);
      } else {
        const code = ck.src || ex.code;
        const src = wrap(code, ck.tpl);
        if (ck.err) await expectErr(id, src, ck);
        else await expectOut(id, src, ck, ck.out);
      }
    }
    for (const [i, e] of (ck.extra || []).entries()) {
      const src = wrap(e.src, e.tpl);
      if (e.err) await expectErr(`${id} extra ${i}`, src, e);
      else await expectOut(`${id} extra ${i}`, src, e, e.out, false);
    }
  }
}

writeFileSync(cachePath, JSON.stringify(cache));
console.log(`solc ${solc.version()}: ${passes} checks passed, ${fails} failed`);
process.exit(fails ? 1 : 0);
