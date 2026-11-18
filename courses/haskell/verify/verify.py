#!/usr/bin/env python3
"""Runs the Haskell course code and checks it (see README.md).

Backends: Wandbox (ghc-9.10.1, default) or a local `ghc` (--local).
Results are cached in .cache.json (key: backend + sha256 of the program).
Usage: python3 verify.py [--local] [unit-id ...]
"""
import hashlib, json, os, re, subprocess, sys, tempfile, time, urllib.request
from pathlib import Path

HERE = Path(__file__).resolve().parent
COURSE = HERE.parent / "course.json"
CHECKS = HERE / "checks.json"
CACHE = HERE / ".cache.json"
WANDBOX = "https://wandbox.org/api/compile.json"
WANDBOX_GHC = "ghc-9.10.1"
MAIN = re.compile(r"^main\s*(::|=)", re.M)

args = [a for a in sys.argv[1:] if not a.startswith("--")]
LOCAL = "--local" in sys.argv
BACKEND = "local:ghc-" + subprocess.run(["ghc", "--numeric-version"], capture_output=True, text=True).stdout.strip() \
    if LOCAL else "wandbox:" + WANDBOX_GHC
cache = json.loads(CACHE.read_text()) if CACHE.exists() else {}
runs = 0


def run_local(src):
    """Compiles with ghc (default warnings only) and runs the program. Returns (exit, stdout, diagnostics)."""
    with tempfile.TemporaryDirectory() as d:
        Path(d, "Main.hs").write_text(src)
        c = subprocess.run(["ghc", "-O0", "-v0", "-outputdir", d, "-o", os.path.join(d, "prog"), "Main.hs"],
                           cwd=d, capture_output=True, text=True, timeout=120)
        if c.returncode != 0:
            return c.returncode, "", c.stdout + c.stderr
        try:
            p = subprocess.run([os.path.join(d, "prog")], capture_output=True, text=True, timeout=10)
            return p.returncode, p.stdout, c.stdout + c.stderr + p.stderr
        except subprocess.TimeoutExpired:
            return -1, "", "timeout"


def run_wandbox(src):
    body = json.dumps({"compiler": WANDBOX_GHC, "code": src, "options": ""}).encode()
    for attempt in range(4):
        try:
            req = urllib.request.Request(WANDBOX, data=body, headers={"Content-Type": "application/json"})
            r = json.load(urllib.request.urlopen(req, timeout=60))
            time.sleep(1)
            return int(r.get("status", "1") or 1), r.get("program_output", ""), \
                r.get("program_error", "") + r.get("compiler_error", "") + r.get("signal", "")
        except Exception as e:  # noqa: BLE001
            print("  retry:", e, file=sys.stderr)
            time.sleep(5 * (attempt + 1))
    raise SystemExit("Wandbox not reachable (try --local)")


def run(src):
    """Returns (ok, stdout, diagnostics). ok: compiles without warnings, exit code 0, nothing on stderr."""
    global runs
    key = BACKEND + ":" + hashlib.sha256(src.encode()).hexdigest()
    if key not in cache:
        code, out, err = (run_local if LOCAL else run_wandbox)(src)
        cache[key] = {"ok": code == 0 and not err.strip(), "stdout": out, "diag": err}
        tmp = CACHE.with_suffix(".tmp")
        tmp.write_text(json.dumps(cache, indent=0))
        tmp.replace(CACHE)
        runs += 1
    c = cache[key]
    return c["ok"], c["stdout"], c["diag"]


def same(out, expected):
    """stdout equals the expected text, with or without one final newline."""
    return out == expected or out == expected + "\n"


def fill(tpl, code):
    """Puts code into the template at {{CODE}}, indented like the marker when it starts a line."""
    m = re.search(r"^([ ]*)\{\{CODE\}\}", tpl, re.M)
    if m is None:
        return tpl.replace("{{CODE}}", code)
    ind = m.group(1)
    lines = code.split("\n")
    body = "\n".join([lines[0]] + [(ind + l) if l else l for l in lines[1:]])
    return tpl[:m.start()] + ind + body + tpl[m.end():]


def program(code, tpl=None):
    """A complete program: with tpl filled in; code with `main` as is; other snippets as a do block."""
    if tpl:
        return fill(tpl, code) + "\n"
    if MAIN.search(code):
        return code + "\n"
    return fill("main :: IO ()\nmain = do\n    {{CODE}}", code) + "\n"


def norm(s):
    return re.sub(r"[\s_'\"`]", "", s).lower()


errors = []


def expect(label, src, chk):
    ok, out, diag = run(src)
    if chk.get("err"):
        if ok:
            errors.append(f"{label}: expected an error, got output {out!r}")
        elif "diag" in chk and chk["diag"] not in diag:
            errors.append(f"{label}: diagnostics do not contain {chk['diag']!r}:\n{diag}")
        return ok, out
    if "diag" in chk:
        if chk["diag"] not in diag:
            errors.append(f"{label}: diagnostics do not contain {chk['diag']!r}:\n{diag}")
    elif not ok:
        errors.append(f"{label}: failed\n{diag}\n--- program ---\n{src}")
    if "out" in chk and not same(out, chk["out"]):
        errors.append(f"{label}: output {out!r}, expected {chk['out']!r}")
    return ok, out


def check_exercise(e, chk):
    eid, kind = e["id"], e["type"]
    src_for = lambda code: program(code, chk.get("tpl"))
    if kind in ("input", "choice", "true_false"):
        code = chk.get("code", e.get("code"))
        if code is not None:
            ok, out = expect(eid, src_for(code), chk)
            if kind == "input" and ok and norm(out) not in {norm(a) for a in e["answers"]}:
                errors.append(f"{eid}: output {out!r} is not among the answers {e['answers']}")
    elif kind == "gap":
        for i, opt in enumerate(e["options"]):
            code = e["code"].replace("___", opt)
            if i == e["answer"]:
                expect(f"{eid} (right option)", src_for(code), chk)
            else:
                ok, out, _ = run(src_for(code))
                if ok and same(out, chk["out"]):
                    errors.append(f"{eid}: wrong option {opt!r} also prints {out!r}")
    elif kind == "bug":
        lines = list(e["lines"])
        ok, out, _ = run(src_for("\n".join(lines)))
        if ok and same(out, chk["out"]):
            errors.append(f"{eid}: buggy code already prints the expected output")
        lines[e["bug"]] = chk["fix"]
        expect(f"{eid} (fixed)", src_for("\n".join(lines)), chk)
    elif kind == "order":
        expect(eid, src_for("\n".join(e["lines"])), chk)
    elif kind == "blocks":
        expect(eid, src_for(" ".join(e["tokens"])), chk)
    for j, x in enumerate(chk.get("extra", [])):
        expect(f"{eid} (extra {j + 1})", program(x["src"]), x)


def main():
    course = json.loads(COURSE.read_text())
    checks = json.loads(CHECKS.read_text())
    only = set(args)
    n_blocks = n_ex = n_claims = 0
    for u in course["units"]:
        if only and u["id"] not in only:
            continue
        for i, b in enumerate(u["lesson"]):
            if b["type"] != "code":
                continue
            label = f"{u['id']}.lesson.{i}"
            if "output" in b and not MAIN.search(b["code"]):
                errors.append(f"{label}: not a complete program (no main)")
            expect(label, program(b["code"]), {"out": b.get("output", "")})
            n_blocks += 1
        for e in u["exercises"]:
            chk = checks.get(e["id"])
            if chk is None:
                if e.get("code") or e["type"] in ("gap", "bug", "order", "blocks"):
                    errors.append(f"{e['id']}: no check")
                continue
            check_exercise(e, chk)
            n_ex += 1
        for j, x in enumerate(checks.get("_claims", {}).get(u["id"], [])):
            expect(f"{u['id']} (lesson claim {j + 1})", program(x["src"]), x)
            n_claims += 1
    print(f"backend: {BACKEND}")
    print(f"lesson code blocks: {n_blocks}, exercises checked: {n_ex}, lesson claims: {n_claims}, "
          f"new runs: {runs}, cached results: {len(cache)}")
    for e in errors:
        print("ERROR", e)
    print("ALL PASSED" if not errors else f"{len(errors)} problem(s)")
    sys.exit(1 if errors else 0)


if __name__ == "__main__":
    main()
