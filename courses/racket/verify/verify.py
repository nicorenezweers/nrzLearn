#!/usr/bin/env python3
"""Runs the Racket course code with a local `racket` and checks it (see README.md).

Programs without a `#lang` line get `#lang racket` in front.
Results are cached in .cache.json (key: racket version + sha256 of the program).
Usage: python3 verify.py [unit-id ...]
"""
import hashlib, itertools, json, subprocess, sys, tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
COURSE = HERE.parent / "course.json"
CHECKS = HERE / "checks.json"
CACHE = HERE / ".cache.json"
MAX_PERMUTATION_RUNS = 400

args = [a for a in sys.argv[1:] if not a.startswith("--")]
BACKEND = "local:" + subprocess.run(["racket", "--version"], capture_output=True, text=True).stdout.strip()
cache = json.loads(CACHE.read_text()) if CACHE.exists() else {}
runs = 0
fails = []
counts = {"lesson": 0, "exercises": 0, "claims": 0}


def with_lang(src):
    return src if any(l.startswith("#lang") for l in src.splitlines()) else "#lang racket\n" + src


def run_local(src):
    with tempfile.TemporaryDirectory() as d:
        f = Path(d, "main.rkt")
        f.write_text(src + "\n")
        try:
            p = subprocess.run(["racket", str(f)], capture_output=True, text=True, timeout=20, cwd=d)
            return p.returncode, p.stdout, p.stderr
        except subprocess.TimeoutExpired:
            return -1, "", "timeout"


def run(src):
    """Returns (ok, stdout, diagnostics). ok: exit code 0 and nothing on stderr."""
    global runs
    src = with_lang(src)
    key = BACKEND + ":" + hashlib.sha256(src.encode()).hexdigest()
    if key not in cache:
        code, out, err = run_local(src)
        cache[key] = {"ok": code == 0 and not err.strip(), "stdout": out, "diag": err[:2000]}
        tmp = CACHE.with_suffix(".tmp")
        tmp.write_text(json.dumps(cache, indent=0))
        tmp.replace(CACHE)
        runs += 1
    c = cache[key]
    return c["ok"], c["stdout"], c["diag"]


def same(out, expected, exact=False):
    return out == expected if exact else out in (expected, expected + "\n")


def fill(tpl, code):
    return tpl.replace("{{CODE}}", code) if tpl else code


def fail(where, msg, src=None, out=None, diag=None):
    lines = [f"FAIL {where}: {msg}"]
    if src is not None:
        lines.append("  --- program ---\n  " + src.replace("\n", "\n  "))
    if out is not None:
        lines.append(f"  --- stdout --- {out!r}")
    if diag:
        lines.append("  --- diagnostics ---\n  " + diag.strip()[:600].replace("\n", "\n  "))
    fails.append("\n".join(lines))


def expect(where, src, chk):
    """chk: {"out": text} (clean run, exact stdout) or {"err": true} (must fail)."""
    ok, out, diag = run(src)
    if chk.get("err"):
        if ok:
            fail(where, "expected an error, but it ran cleanly", src, out)
        elif chk.get("diag") and chk["diag"] not in diag:
            fail(where, f"diagnostics lack {chk['diag']!r}", src, out, diag)
        return
    if not ok:
        fail(where, "did not run cleanly", src, out, diag)
    elif not same(out, chk["out"], chk.get("exact")):
        fail(where, f"expected {chk['out']!r}", src, out, diag)


def differs(where, src, expected):
    """The program must fail or print something other than expected."""
    ok, out, _ = run(src)
    if ok and same(out, expected):
        fail(where, "a wrong variant prints the expected output", src, out)


def normalize(s):
    return "".join(ch for ch in s.lower() if ch not in " \t\n_\"'`")


def balanced_prefixes(lines):
    """Paren depth never negative and ends at 0 (ignores parens in strings)."""
    depth = 0
    for line in lines:
        in_str = False
        for ch in line:
            if ch == '"':
                in_str = not in_str
            elif not in_str and ch in "([":
                depth += 1
            elif not in_str and ch in ")]":
                depth -= 1
                if depth < 0:
                    return False
    return depth == 0


def check_order_unique(where, lines, tpl, expected):
    tried = 0
    for perm in itertools.permutations(range(len(lines))):
        if list(perm) == list(range(len(lines))):
            continue
        cand = [lines[i] for i in perm]
        if not balanced_prefixes(cand):
            continue
        tried += 1
        if tried > MAX_PERMUTATION_RUNS:
            fail(where, "too many balanced permutations to check uniqueness")
            return
        differs(where + " (other order)", fill(tpl, "\n".join(cand)), expected)


def check_exercise(e, chk):
    eid, t = e["id"], e["type"]
    tpl = chk.get("tpl")
    for i, x in enumerate(chk.get("extra", [])):
        expect(f"{eid}.extra[{i}]", x["src"], x)
    if t in ("input", "choice", "true_false"):
        if "out" in chk or "err" in chk or t == "input":
            want = dict(chk)
            if t == "input":
                want.setdefault("out", e["answers"][0])
                if normalize(want["out"]) != normalize(e["answers"][0]):
                    fail(eid, "first answer does not match the expected output")
            src = chk.get("src") or fill(tpl, e["code"])
            expect(eid, src, want)
            if t == "input":
                _, out, _ = run(src)
                if not any(normalize(out) == normalize(a) for a in e["answers"]):
                    fail(eid, f"output {out!r} matches none of the answers {e['answers']}")
    elif t == "gap":
        for i, opt in enumerate(e["options"]):
            src = fill(tpl, e["code"].replace("___", opt))
            if i == e["answer"]:
                expect(eid, src, chk)
            else:
                differs(f"{eid} (option {opt!r})", src, chk["out"])
    elif t == "bug":
        lines = list(e["lines"])
        differs(f"{eid} (as shown)", fill(tpl, "\n".join(lines)), chk["out"])
        lines[e["bug"]] = chk["fix"]
        expect(eid + " (fixed)", fill(tpl, "\n".join(lines)), chk)
    elif t == "order":
        expect(eid, fill(tpl, "\n".join(e["lines"])), chk)
        check_order_unique(eid, e["lines"], tpl, chk["out"])
    elif t == "blocks":
        expect(eid, fill(tpl, " ".join(e["tokens"])), chk)
    counts["exercises"] += 1


def main():
    course = json.loads(COURSE.read_text())
    checks = json.loads(CHECKS.read_text())
    unchecked = []
    for u in course["units"]:
        if args and u["id"] not in args:
            continue
        for i, b in enumerate(u["lesson"]):
            if b["type"] != "code":
                continue
            where = f"{u['id']}.lesson.{i}"
            expect(where, b["code"], {"out": b.get("output", ""), "exact": "output" not in b})
            counts["lesson"] += 1
        for i, c in enumerate(checks.get("_claims", {}).get(u["id"], [])):
            expect(f"{u['id']}.claim[{i}]", c["src"], c)
            counts["claims"] += 1
        for e in u["exercises"]:
            if e["id"] in checks:
                check_exercise(e, checks[e["id"]])
            else:
                unchecked.append(e["id"])
    print(f"backend {BACKEND}: {counts['lesson']} lesson blocks, {counts['exercises']} exercises, "
          f"{counts['claims']} claims, {runs} new runs, {len(cache)} cached programs")
    if unchecked:
        print("not checked by code:", ", ".join(unchecked))
    for f in fails:
        print(f)
    print("ALL PASSED" if not fails else f"{len(fails)} FAILURES")
    sys.exit(1 if fails else 0)


main()
