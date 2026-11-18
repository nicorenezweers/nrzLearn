#!/usr/bin/env python3
"""Runs the Scheme course code with chibi-scheme (R7RS small) and checks it.

- every lesson code block: runs without error; stdout == `output`
- every exercise with an entry in checks.json (see README.md)
- no program defines a name that R7RS small already provides

Usage: python3 verify.py [unit-id ...]
"""
import itertools, json, re, subprocess, sys, tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
COURSE = HERE.parent / "course.json"
CHECKS = HERE / "checks.json"
TMP = Path(tempfile.mkdtemp(prefix="scm-verify-"))
PRELUDE = "(import (scheme base) (scheme write) (scheme char) (scheme cxr) (scheme inexact))\n"
runs = 0
errors = []
defined = set()


def run(src):
    """Returns (ok, stdout, stderr). ok: exit code 0 within the time limit."""
    global runs
    runs += 1
    defined.update(re.findall(r"^\s*\(define \(?([^\s()]+)", src, re.M))
    f = TMP / "main.scm"
    f.write_text(PRELUDE + src + "\n")
    try:
        p = subprocess.run(["chibi-scheme", str(f)], capture_output=True, text=True, timeout=10)
    except subprocess.TimeoutExpired:
        return False, "", "timeout"
    return p.returncode == 0, p.stdout.removesuffix("\n"), p.stderr


def fill(tpl, code):
    return tpl.replace("{{CODE}}", code) if tpl else code


def norm(s):
    return re.sub(r"[\s_'\"`]", "", s).lower()


def expect(label, src, chk):
    ok, out, err = run(src)
    if chk.get("err"):
        if ok:
            errors.append(f"{label}: expected an error, got output {out!r}")
        return ok, out
    if not ok:
        errors.append(f"{label}: failed\n{err[:600]}\n--- program ---\n{src}")
    elif "out" in chk and out != chk["out"]:
        errors.append(f"{label}: output {out!r}, expected {chk['out']!r}")
    return ok, out


def balanced(src):
    """Cheap filter for order permutations: parentheses must nest (strings ignored)."""
    depth, in_str = 0, False
    for ch in src:
        if ch == '"':
            in_str = not in_str
        elif in_str:
            continue
        elif ch == "(":
            depth += 1
        elif ch == ")":
            depth -= 1
            if depth < 0:
                return False
    return depth == 0


def check_exercise(e, chk):
    eid, kind, tpl = e["id"], e["type"], chk.get("tpl")
    if kind in ("input", "choice", "true_false"):
        if e.get("code") is not None and "manual" not in chk:
            ok, out = expect(eid, fill(tpl, e["code"]), chk)
            if kind == "input" and ok and norm(out) not in {norm(a) for a in e["answers"]}:
                errors.append(f"{eid}: output {out!r} is not among the answers {e['answers']}")
    elif kind == "gap":
        for i, opt in enumerate(e["options"]):
            src = fill(tpl, e["code"].replace("___", opt))
            if i == e["answer"]:
                expect(f"{eid} (right option)", src, chk)
            else:
                ok, out, _ = run(src)
                if ok and out == chk["out"]:
                    errors.append(f"{eid}: wrong option {opt!r} also prints {out!r}")
    elif kind == "bug":
        lines = list(e["lines"])
        ok, out, _ = run(fill(tpl, "\n".join(lines)))
        if ok and out == chk["out"]:
            errors.append(f"{eid}: buggy code already prints the expected output")
        lines[e["bug"]] = chk["fix"]
        expect(f"{eid} (fixed)", fill(tpl, "\n".join(lines)), chk)
    elif kind == "order":
        lines = e["lines"]
        expect(eid, fill(tpl, "\n".join(lines)), chk)
        tried = 0
        for perm in itertools.permutations(range(len(lines))):
            if list(perm) == list(range(len(lines))):
                continue
            src = fill(tpl, "\n".join(lines[i] for i in perm))
            if not balanced(src):
                continue
            tried += 1
            ok, out, _ = run(src)
            if ok and out == chk["out"]:
                errors.append(f"{eid}: another order also prints {out!r}: {perm}")
        print(f"  {eid}: {tried} other balanced orders tried")
    elif kind == "blocks":
        expect(eid, fill(tpl, " ".join(e["tokens"])), chk)
    for j, x in enumerate(chk.get("extra", [])):
        expect(f"{eid} (extra {j + 1})", x["src"], x)
    if "manual" in chk:
        print(f"  {eid}: checked by reading ({chk['manual']})")


def main():
    course = json.loads(COURSE.read_text())
    checks = json.loads(CHECKS.read_text())
    only = set(sys.argv[1:])
    lessons = checked = 0
    for u in course["units"]:
        if only and u["id"] not in only:
            continue
        print("unit", u["id"])
        for i, b in enumerate(u["lesson"]):
            if b["type"] == "code":
                lessons += 1
                expect(f"{u['id']} lesson block {i}", b["code"], {"out": b.get("output", "")})
        for e in u["exercises"]:
            if e["id"] in checks:
                checked += 1
                check_exercise(e, checks[e["id"]])
            elif e["type"] != "pairs":
                errors.append(f"{e['id']}: no check")
    for name in sorted(defined):
        f = TMP / "name.scm"
        f.write_text(PRELUDE + name + "\n")
        p = subprocess.run(["chibi-scheme", str(f)], capture_output=True, text=True, timeout=10)
        if p.returncode == 0:
            errors.append(f"`{name}` is defined in the course but is already a standard name")
    ver = subprocess.run(["chibi-scheme", "-V"], capture_output=True, text=True).stdout.strip()
    print(f"{ver}: {lessons} lesson blocks, {checked} exercises checked, {runs} programs run, {len(defined)} defined names checked")
    for err in errors:
        print("ERROR", err)
    print("OK" if not errors else f"{len(errors)} errors")
    sys.exit(1 if errors else 0)


if __name__ == "__main__":
    main()
