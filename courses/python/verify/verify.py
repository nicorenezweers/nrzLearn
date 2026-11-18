#!/usr/bin/env python3
"""Checks every runnable part of courses/python/course.json.

Usage: python3 courses/python/verify/verify.py [--python python3.12] [-v]

- Lesson code blocks run in a fresh interpreter; stdout (minus the final
  newline) must equal `output` (or be empty when there is no `output`).
- Exercises are checked with the rules described in README.md, using the
  extra data in checks.json (expected outputs, bug fixes, proofs).
"""

import argparse
import contextlib
import io
import itertools
import json
import re
import subprocess
import sys
import tokenize
import warnings
from pathlib import Path

HERE = Path(__file__).resolve().parent
COURSE = HERE.parent / "course.json"
CHECKS = HERE / "checks.json"

PYTHON = "python3"
problems = []
counts = {"lesson": 0, "programs": 0, "permutations": 0}


def fail(where, msg):
    problems.append(f"{where}: {msg}")


def run(code, timeout=10):
    """Runs code in a fresh interpreter. Returns (stdout, ok)."""
    counts["programs"] += 1
    try:
        p = subprocess.run([PYTHON, "-I", "-c", code], capture_output=True, text=True, timeout=timeout)
    except subprocess.TimeoutExpired:
        return "", False
    out = p.stdout[:-1] if p.stdout.endswith("\n") else p.stdout
    return out, p.returncode == 0 and not p.stderr


class StepLimit(Exception):
    pass


def run_fast(code, limit=20000):
    """Runs code in-process with a step limit (for permutation checks). None on error."""
    try:
        with warnings.catch_warnings():
            warnings.simplefilter("ignore")
            compiled = compile(code, "<perm>", "exec")
    except SyntaxError:
        return None
    steps = 0

    def tracer(frame, event, arg):
        nonlocal steps
        steps += 1
        if steps > limit:
            raise StepLimit
        return tracer

    buf = io.StringIO()
    old = sys.gettrace()
    try:
        with contextlib.redirect_stdout(buf):
            sys.settrace(tracer)
            try:
                exec(compiled, {"__name__": "__main__"})
            finally:
                sys.settrace(old)
    except BaseException:
        return None
    out = buf.getvalue()
    return out[:-1] if out.endswith("\n") else out


def norm(s):
    """Same comparison as the app: ignores case, spaces, _, quotes and backticks."""
    return re.sub(r"[\s_'\"`]", "", s).lower()


def wrap(spec, body):
    parts = [spec.get("setup"), body, spec.get("after")]
    return "\n".join(p for p in parts if p)


def unbacktick(opt):
    m = re.fullmatch(r"`([^`]*)`", opt)
    return m.group(1) if m else None


def check_unique_order(where, items, joiner, spec, expect, max_items=8):
    """No other order of the items may give the expected output."""
    if len(items) > max_items:
        print(f"  note: {where}: {len(items)} items, permutation check skipped")
        return
    seen = set()
    right = tuple(items)
    for perm in itertools.permutations(items):
        if perm == right or perm in seen:
            continue
        seen.add(perm)
        counts["permutations"] += 1
        if run_fast(wrap(spec, joiner.join(perm))) == expect:
            fail(where, f"another order also prints the expected output: {list(perm)}")
            return


def comments(code):
    try:
        toks = list(tokenize.generate_tokens(io.StringIO(code).readline))
    except (tokenize.TokenError, SyntaxError, IndentationError):
        return []
    return [t.string for t in toks if t.type == tokenize.COMMENT]


def check_comments(where, code, unit_id):
    for c in comments(code):
        text = c.lstrip("#").strip()
        if unit_id == "hello":
            continue
        if not text.startswith("->"):
            fail(where, f"comment that is not a result: {c!r}")


PRELUDE = """import io as _io, contextlib as _cl
CODE = {code!r}
def output(code=CODE):
    b = _io.StringIO()
    with _cl.redirect_stdout(b):
        exec(code, {{"__name__": "__main__"}})
    v = b.getvalue()
    return v[:-1] if v.endswith("\\n") else v
"""


def proof(where, spec, required, expect=None, code=None):
    """Runs spec["proof"] (with output() and CODE available) and compares its stdout."""
    if "proof" not in spec:
        if required:
            fail(where, "needs a proof in checks.json")
        return False
    out, ok = run(PRELUDE.format(code=code) + spec["proof"])
    want = expect if expect is not None else spec.get("proof_out")
    if want is None:
        fail(where, "proof without proof_out")
    elif not ok or out != want:
        fail(where, f"proof printed {out!r} (ok={ok}), expected {want!r}")
    return True


def check_exercise(e, spec, unit_id):
    eid, kind = e["id"], e["type"]
    where = f"{eid} ({kind})"
    for key in ("code",):
        if key in e:
            check_comments(where, e[key], unit_id)
    for line in e.get("lines", []):
        check_comments(where, line, unit_id)

    if kind == "input":
        if "code" not in e:
            proof(where, spec, True, code=e.get("code"))
            return
        out, ok = run(e["code"])
        if not ok:
            fail(where, "code does not run cleanly")
        if "\n" in out:
            fail(where, f"output has several lines: {out!r}")
        if e["answers"][0] != out:
            fail(where, f"first answer {e['answers'][0]!r} is not the exact output {out!r}")
        for a in e["answers"]:
            if norm(a) != norm(out):
                fail(where, f"answer {a!r} does not match output {out!r}")
        proof(where, spec, False, code=e.get("code"))

    elif kind == "choice":
        plain = [unbacktick(o) for o in e["options"]]
        if "options_template" in spec:
            expect = spec.get("expect")
            for i, p in enumerate(plain):
                if p is None:
                    fail(where, f"option {i} is not code")
                    continue
                out, ok = run(spec["options_template"].replace("___", p))
                if i == e["answer"] and (not ok or out != expect):
                    fail(where, f"right option {p!r} printed {out!r} (ok={ok}), expected {expect!r}")
                if i != e["answer"] and ok and out == expect:
                    fail(where, f"wrong option {p!r} also prints {expect!r}")
            proof(where, spec, False, code=e.get("code"))
        elif "code" in e and all(p is not None for p in plain) and not spec.get("not_output"):
            out, ok = run(e["code"])
            matches = [i for i, p in enumerate(plain) if p == out]
            if matches != [e["answer"]]:
                fail(where, f"output {out!r} (ok={ok}) matches options {matches}, answer is {e['answer']}")
            proof(where, spec, False, code=e.get("code"))
        else:
            proof(where, spec, True, code=e.get("code"))

    elif kind == "true_false":
        proof(where, spec, True, expect=str(e["answer"]), code=e.get("code"))

    elif kind == "gap":
        expect = spec.get("expect")
        if expect is None:
            fail(where, "needs expect in checks.json")
            return
        for i, opt in enumerate(e["options"]):
            out, ok = run(wrap(spec, e["code"].replace("___", opt)))
            if i == e["answer"]:
                if not ok or out != expect:
                    fail(where, f"right option {opt!r} printed {out!r} (ok={ok}), expected {expect!r}")
            elif ok and out == expect:
                fail(where, f"wrong option {opt!r} also prints the expected output")
        proof(where, spec, False, code=e.get("code"))

    elif kind == "blocks":
        expect = spec.get("expect")
        if expect is None:
            fail(where, "needs expect in checks.json")
            return
        line = " ".join(e["tokens"])
        out, ok = run(wrap(spec, line))
        if not ok or out != expect:
            fail(where, f"{line!r} printed {out!r} (ok={ok}), expected {expect!r}")
        for d in e["distractors"]:
            for i in range(len(e["tokens"])):
                t = list(e["tokens"])
                t[i] = d
                if run_fast(wrap(spec, " ".join(t))) == expect:
                    fail(where, f"distractor {d!r} can replace token {e['tokens'][i]!r}")
        if not spec.get("skip_perm"):
            check_unique_order(where, e["tokens"], " ", spec, expect)
        proof(where, spec, False, code=e.get("code"))

    elif kind == "order":
        expect = spec.get("expect")
        if expect is None:
            fail(where, "needs expect in checks.json")
            return
        out, ok = run(wrap(spec, "\n".join(e["lines"])))
        if not ok or out != expect:
            fail(where, f"printed {out!r} (ok={ok}), expected {expect!r}")
        check_unique_order(where, e["lines"], "\n", spec, expect)
        proof(where, spec, False, code=e.get("code"))

    elif kind == "bug":
        expect, fix = spec.get("expect"), spec.get("fix")
        if expect is None or fix is None:
            fail(where, "needs expect and fix in checks.json")
            return
        lines = list(e["lines"])
        out, ok = run(wrap(spec, "\n".join(lines)))
        if ok and out == expect:
            fail(where, "the buggy program already prints the expected output")
        lines[e["bug"]] = fix
        out, ok = run(wrap(spec, "\n".join(lines)))
        if not ok or out != expect:
            fail(where, f"fixed program printed {out!r} (ok={ok}), expected {expect!r}")
        proof(where, spec, False, code=e.get("code"))

    elif kind == "pairs":
        proof(where, spec, True, code=e.get("code"))


def main():
    global PYTHON
    ap = argparse.ArgumentParser()
    ap.add_argument("--python", default="python3")
    args = ap.parse_args()
    PYTHON = args.python
    version = subprocess.run([PYTHON, "--version"], capture_output=True, text=True).stdout.strip()
    print(f"Checking with {version}")

    course = json.loads(COURSE.read_text())
    checks = json.loads(CHECKS.read_text())
    ids = set()
    n_ex = 0
    for unit in course["units"]:
        uid = unit["id"]
        for i, b in enumerate(unit["lesson"]):
            if b["type"] != "code":
                continue
            where = f"{uid}.lesson.{i}"
            counts["lesson"] += 1
            check_comments(where, b["code"], uid)
            out, ok = run(b["code"])
            if not ok:
                fail(where, "does not run cleanly")
            want = b.get("output", "")
            if out != want:
                fail(where, f"output differs\n--- expected ---\n{want}\n--- actual ---\n{out}")
        for e in unit["exercises"]:
            n_ex += 1
            ids.add(e["id"])
            check_exercise(e, checks.get(e["id"], {}), uid)
    for k in checks:
        if k not in ids:
            fail("checks.json", f"unknown exercise {k}")

    print(f"{counts['lesson']} lesson code blocks, {n_ex} exercises, "
          f"{counts['programs']} programs run, {counts['permutations']} permutations tried")
    if problems:
        print(f"\n{len(problems)} problem(s):")
        for p in problems:
            print(" -", p)
        sys.exit(1)
    print("All checks passed.")


if __name__ == "__main__":
    main()
