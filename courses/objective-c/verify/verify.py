#!/usr/bin/env python3
"""Compiles and runs the Objective-C course code with the local Apple clang and checks it.

- every lesson code block (complete programs): compiles without warnings; stdout == `output`
- every exercise with an entry in checks.json (see README.md)

Results are cached in .cache.json (key: sha256 of compiler version + program).
Usage: python3 verify.py [unit-id ...]
"""
import hashlib, json, os, re, subprocess, sys, tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
COURSE = HERE.parent / "course.json"
CHECKS = HERE / "checks.json"
CACHE = HERE / ".cache.json"
CLANG = ["clang", "-fobjc-arc", "-Wall", "-framework", "Foundation"]
TIMEOUT = 5
FOUND = "#import <Foundation/Foundation.h>"

VERSION = subprocess.run(["clang", "--version"], capture_output=True, text=True).stdout.split("\n")[0]
cache = json.loads(CACHE.read_text()) if CACHE.exists() else {}
runs = 0
tmpdir = Path(tempfile.mkdtemp(prefix="objc-verify-"))


def run(src):
    """Returns dict: ok (compiled, exit code 0, no timeout), stdout, diag, warn (compiler warnings)."""
    global runs
    key = hashlib.sha256((VERSION + "\n" + src).encode()).hexdigest()
    if key not in cache:
        m, exe = tmpdir / "main.m", tmpdir / "main"
        m.write_text(src)
        if exe.exists():
            exe.unlink()
        c = subprocess.run(CLANG + [str(m), "-o", str(exe)], capture_output=True, text=True)
        res = {"ok": False, "stdout": "", "diag": c.stderr, "warn": "warning:" in c.stderr}
        if c.returncode == 0:
            try:
                p = subprocess.run(["/bin/bash", "-c", 'set -o pipefail; "$0" | head -c 100000', str(exe)],
                                   capture_output=True, text=True, errors="replace", timeout=TIMEOUT)
                res["ok"] = p.returncode == 0
                res["stdout"] = p.stdout
                res["diag"] += p.stderr[-2000:]
            except subprocess.TimeoutExpired:
                res["diag"] += "timeout"
        if res["stdout"].endswith("\n"):
            res["stdout"] = res["stdout"][:-1]
        cache[key] = res
        tmp = CACHE.with_suffix(".tmp")
        tmp.write_text(json.dumps(cache, indent=0))
        tmp.replace(CACHE)
        runs += 1
    return cache[key]


def indent(code, n):
    return "\n".join((" " * n + l) if l else "" for l in code.split("\n"))


def program(body, pre=""):
    head = FOUND + "\n\n" + (pre + "\n\n" if pre else "")
    return head + "int main(void) {\n    @autoreleasepool {\n" + indent(body, 8) + "\n    }\n    return 0;\n}\n"


def fill(tpl, code):
    """Puts code into the template at {{CODE}}, indented like the marker."""
    m = re.search(r"^([ ]*)\{\{CODE\}\}", tpl, re.M)
    ind = m.group(1)
    lines = code.split("\n")
    body = "\n".join([lines[0]] + [(ind + l) if l else l for l in lines[1:]])
    return tpl[:m.start()] + ind + body + tpl[m.end():]


def build(code, chk):
    if "tpl" in chk:
        return fill(chk["tpl"], code)
    if "int main(" in code:
        return code + "\n"
    if "main" in chk:
        return program(chk["main"], code)
    body = "\n".join(x for x in (chk.get("before"), code, chk.get("after")) if x)
    return program(body, chk.get("pre", ""))


def norm(s):
    return re.sub(r"[\s_'\"`]", "", s).lower()


errors = []


def expect(label, src, chk):
    r = run(src)
    if chk.get("err"):
        if r["ok"]:
            errors.append(f"{label}: expected a compile or run error, got output {r['stdout']!r}")
        return r
    if not r["ok"]:
        errors.append(f"{label}: failed\n{r['diag']}\n--- program ---\n{src}")
    elif "out" in chk and r["stdout"] != chk["out"]:
        errors.append(f"{label}: output {r['stdout']!r}, expected {chk['out']!r}")
    elif r["warn"] and not chk.get("warn"):
        errors.append(f"{label}: compiler warning\n{r['diag']}")
    return r


def wrong(label, src, chk):
    r = run(src)
    if r["ok"] and r["stdout"] == chk["out"]:
        errors.append(f"{label}: also prints {r['stdout']!r}\n--- program ---\n{src}")


def check_exercise(e, chk):
    eid, kind = e["id"], e["type"]
    if kind in ("input", "choice", "true_false"):
        code = chk.get("code", e.get("code"))
        if code is not None:
            r = expect(eid, build(code, chk), chk)
            if kind == "input" and r["ok"] and norm(r["stdout"]) not in {norm(a) for a in e["answers"]}:
                errors.append(f"{eid}: output {r['stdout']!r} is not among the answers {e['answers']}")
    elif kind == "gap":
        for i, opt in enumerate(e["options"]):
            code = e["code"].replace("___", opt)
            if i == e["answer"]:
                expect(f"{eid} (right option)", build(code, chk), chk)
            else:
                wrong(f"{eid}: wrong option {opt!r}", build(code, chk), chk)
    elif kind == "bug":
        lines = list(e["lines"])
        wrong(f"{eid}: buggy code", build("\n".join(lines), chk), chk)
        lines[e["bug"]] = chk["fix"]
        expect(f"{eid} (fixed)", build("\n".join(lines), chk), chk)
    elif kind == "order":
        expect(eid, build("\n".join(e["lines"]), chk), chk)
    elif kind == "blocks":
        expect(eid, build(" ".join(e["tokens"]), chk), chk)
        if chk.get("distractors"):
            for d in e["distractors"]:
                for i in range(len(e["tokens"])):
                    alt = e["tokens"][:i] + [d] + e["tokens"][i + 1:]
                    wrong(f"{eid}: distractor {d!r} at position {i}", build(" ".join(alt), chk), chk)
    for j, x in enumerate(chk.get("extra", [])):
        expect(f"{eid} (extra {j + 1})", x["src"] if x["src"].endswith("\n") else x["src"] + "\n", x)


def main():
    course = json.loads(COURSE.read_text())
    checks = json.loads(CHECKS.read_text())
    only = set(sys.argv[1:])
    n_blocks = n_ex = 0
    for u in course["units"]:
        if only and u["id"] not in only:
            continue
        for i, b in enumerate(u["lesson"]):
            if b["type"] != "code":
                continue
            label = f"{u['id']}.lesson.{i}"
            if not b["code"].startswith(FOUND) or "int main(" not in b["code"]:
                errors.append(f"{label}: not a complete program")
                continue
            expect(label, b["code"] + "\n", {"out": b.get("output", "")})
            n_blocks += 1
        for e in u["exercises"]:
            chk = checks.get(e["id"])
            if chk is None:
                if e.get("code") or e["type"] in ("gap", "bug", "order", "blocks"):
                    errors.append(f"{e['id']}: no check")
                continue
            check_exercise(e, chk)
            n_ex += 1
    print(VERSION)
    print(f"lesson programs: {n_blocks}, exercises checked: {n_ex}, new compiles: {runs}, cached programs: {len(cache)}")
    for e in errors:
        print("ERROR", e)
    print("ALL PASSED" if not errors else f"{len(errors)} problem(s)")
    sys.exit(1 if errors else 0)


if __name__ == "__main__":
    main()
