#!/usr/bin/env python3
"""Runs the Clojure course code with Babashka (bb) and checks it.

- every lesson code block: runs without error; stdout == `output`
- every exercise with an entry in checks.json (see README.md)

Usage: python3 verify.py [unit-id ...]
"""
import itertools, json, re, subprocess, sys, tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
COURSE = HERE.parent / "course.json"
CHECKS = HERE / "checks.json"
TMP = Path(tempfile.mkdtemp(prefix="clj-verify-"))
runs = 0
errors = []


def run(src):
    """Returns (ok, stdout, stderr). ok: exit code 0 within the time limit."""
    global runs
    runs += 1
    f = TMP / "main.clj"
    f.write_text(src + "\n")
    try:
        p = subprocess.run(["bb", str(f)], capture_output=True, text=True, timeout=20)
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
    """Cheap filter for order permutations: brackets must nest (strings ignored)."""
    stack, pairs, in_str = [], {")": "(", "]": "[", "}": "{"}, False
    for ch in src:
        if ch == '"':
            in_str = not in_str
        elif in_str:
            continue
        elif ch in "([{":
            stack.append(ch)
        elif ch in pairs:
            if not stack or stack.pop() != pairs[ch]:
                return False
    return not stack


def check_exercise(e, chk):
    eid, kind, tpl = e["id"], e["type"], chk.get("tpl")
    if kind in ("input", "choice", "true_false"):
        code = chk.get("code", e.get("code"))
        if code is not None:
            ok, out = expect(eid, fill(tpl, code), chk)
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
    ver = subprocess.run(["bb", "--version"], capture_output=True, text=True).stdout.strip()
    print(f"{ver}: {lessons} lesson blocks, {checked} exercises checked, {runs} programs run")
    for err in errors:
        print("ERROR", err)
    print("OK" if not errors else f"{len(errors)} errors")
    sys.exit(1 if errors else 0)


if __name__ == "__main__":
    main()
