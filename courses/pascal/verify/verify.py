#!/usr/bin/env python3
"""Compile and run every piece of Pascal in courses/pascal/course.json.

Usage: python3 courses/pascal/verify/verify.py [-v] [--local]

By default every program is compiled and run with Free Pascal 3.2.2 on
Compiler Explorer (godbolt.org, x86-64 Linux). Answers are cached in
verify/.cache/, so each program is sent only once (one request per second).
--local uses the `fpc` in PATH instead (set FPC to choose another one).

* Lesson code blocks: run, and stdout (without the final newline) compared
  with `output`. Blocks without `output` must compile and print nothing.
* Exercises: checked with the steps in checks.json (see README.md).

A "good" program must compile without any warning, exit with 0 and print the
expected text. A "bad" program (a wrong gap option, a wrong choice option, the
line with the bug) must fail: compile error, warning, runtime error, timeout
or a different output.
"""

import hashlib
import json
import os
import re
import subprocess
import sys
import tempfile
import time
import urllib.request
from pathlib import Path

HERE = Path(__file__).resolve().parent
COURSE = HERE.parent / "course.json"
CHECKS = HERE / "checks.json"
VERBOSE = "-v" in sys.argv
LOCAL = "--local" in sys.argv
GODBOLT = os.environ.get("GODBOLT_COMPILER", "fpc322")
FPC = os.environ.get("FPC", "fpc")
CACHE = HERE / ".cache" / (f"local.json" if LOCAL else f"godbolt-{GODBOLT}.json")
MODE = "{$mode objfpc}{$H+}"
HEADER = "program VerifySnippet;"


def norm(s):
    """The app's comparison: ignore case, spaces, `_`, quotes and backticks."""
    return re.sub(r"[\s_\"'`]", "", s).lower()


def build(src, decl=""):
    """Turns a snippet into a complete program.

    A snippet with a line that starts with `begin` or `program` is a program
    (the `program` header is optional in Pascal, Compiler Explorer needs it).
    Anything else is a list of statements and goes between begin and end.
    `decl` (var/type/... sections) goes before the main begin; a snippet
    with `decl` is always a list of statements.
    """
    full = not decl and re.search(r"^(program |begin\b)", src, re.M)
    if not full:
        body = "\n".join("  " + l if l else l for l in src.split("\n"))
        src = (decl + "\n" if decl else "") + "begin\n" + body + "\nend."
    if not re.search(r"^program ", src, re.M):
        src = HEADER + "\n" + src
    if "{$mode" not in src:
        src = MODE + "\n" + src
    return src + "\n"


def compile_local(prog):
    work = Path(tempfile.mkdtemp(prefix="pas-verify-"))
    (work / "prog.pas").write_text(prog)
    c = subprocess.run([FPC, "-vw", "prog.pas"], cwd=work, capture_output=True, text=True)
    build_log = c.stdout + c.stderr
    res = {"build_ok": c.returncode == 0, "build": build_log, "ran": False, "code": None, "stdout": ""}
    if c.returncode == 0:
        try:
            r = subprocess.run([str(work / "prog")], capture_output=True, text=True, timeout=5)
            res.update(ran=True, code=r.returncode, stdout=r.stdout)
        except subprocess.TimeoutExpired:
            res.update(ran=False, build=build_log + "\ntimeout")
    return res


def compile_godbolt(prog):
    body = {"source": prog, "lang": "pascal", "allowStoreCodeDebug": True,
            "options": {"userArguments": "", "compilerOptions": {"executorRequest": True},
                        "filters": {"execute": True}, "tools": [], "libraries": []}}
    req = urllib.request.Request(f"https://godbolt.org/api/compiler/{GODBOLT}/compile",
                                 data=json.dumps(body).encode(),
                                 headers={"Content-Type": "application/json", "Accept": "application/json"})
    for attempt in range(3):
        try:
            with urllib.request.urlopen(req, timeout=90) as r:
                d = json.load(r)
            break
        except Exception:
            if attempt == 2:
                raise
            time.sleep(5)
    b = d.get("buildResult", {})
    log = "\n".join(x["text"] for x in b.get("stdout", []) + b.get("stderr", []))
    time.sleep(1)
    return {"build_ok": b.get("code") == 0, "build": log, "ran": d.get("didExecute", False),
            "code": d.get("code"), "stdout": "".join(x["text"] + "\n" for x in d.get("stdout", []))}


CACHE_DATA = json.loads(CACHE.read_text()) if CACHE.exists() else {}


def run(prog):
    """Returns (ok, stdout, diagnostics). ok: compiled without warning, ran, exit code 0."""
    h = hashlib.sha1(prog.encode()).hexdigest()
    if h not in CACHE_DATA:
        CACHE_DATA[h] = compile_local(prog) if LOCAL else compile_godbolt(prog)
        CACHE.parent.mkdir(exist_ok=True)
        CACHE.write_text(json.dumps(CACHE_DATA, indent=1))
    c = CACHE_DATA[h]
    problems = [l for l in c["build"].split("\n") if re.search(r"\b(Warning|Error|Fatal):", l)]
    if not c["build_ok"] or problems:
        return False, None, "\n".join(problems) or "compile error"
    if not c["ran"]:
        return False, None, "did not run"
    out = c["stdout"][:-1] if c["stdout"].endswith("\n") else c["stdout"]
    if c["code"] != 0:
        return False, out, f"exit code {c['code']}"
    return True, out, ""


def unbacktick(s):
    return s.replace("`", "")


class Job:
    def __init__(self, where, prog, expect=None, accept=None, bad=False):
        self.where, self.prog, self.expect, self.accept, self.bad = where, prog, expect, accept, bad

    def __call__(self):
        ok, out, diag = run(self.prog)
        if self.bad and self.expect is None and self.accept is None:
            matches = ok
        elif self.accept is not None:
            matches = ok and norm(out) in {norm(a) for a in self.accept}
        else:
            matches = ok and out == self.expect
        if self.bad:
            errs = [l for l in (diag or "").split("\n") if "Error" in l or "Fatal" in l]
            self.reason = (errs or (diag or "").split("\n"))[0] if not ok else f"printed {out!r}"
            return (not matches), f"{self.where}: expected to fail, but compiled and printed {out!r}"
        if not ok:
            return False, f"{self.where}: {diag}\n--- program ---\n{self.prog}"
        if not matches:
            want = self.accept if self.accept is not None else self.expect
            return False, f"{self.where}: printed {out!r}, expected {want!r}"
        return True, ""


def subst(src, ex, option=None):
    t = ex["type"]
    code = ex.get("code", "")
    if t == "gap":
        code = code.replace("___", option if option is not None else ex["options"][ex["answer"]])
    answer = ""
    if t == "blocks":
        answer = " ".join(ex["tokens"])
    elif t == "order":
        answer = "\n".join(ex["lines"])
    if t == "gap":
        src = src.replace("@GAP@", option if option is not None else ex["options"][ex["answer"]])
    src = src.replace("@CODE@", code).replace("@ANSWER@", answer)
    if option is not None:
        src = src.replace("@OPTION@", unbacktick(option))
    return src


def exercise_jobs(ex, steps):
    eid, t = ex["id"], ex["type"]
    jobs, manual = [], []
    default_src = {"bug": "@FIXED@", "blocks": "@ANSWER@", "order": "@ANSWER@"}.get(t, "@CODE@")
    for i, st in enumerate(steps):
        where = f"{eid} step {i}"
        kind = st.get("kind", "run")
        if kind == "manual":
            manual.append(f"{eid}: {st['note']}")
            continue
        src = st.get("src", default_src)
        decl = st.get("decl", "")
        expect = st.get("expect")
        accept = None
        if kind == "run" and expect is None:
            if t == "input":
                accept = ex["answers"]
            elif t == "choice":
                accept = [unbacktick(ex["options"][ex["answer"]])]
            else:
                raise SystemExit(f"{where}: needs 'expect'")
        if kind == "fail":
            jobs.append(Job(where + " (must fail)", build(subst(src, ex), decl), expect, bad=True))
            continue
        if kind == "options":
            if expect is None:
                raise SystemExit(f"{where}: needs 'expect'")
            for j, opt in enumerate(ex["options"]):
                good = j == ex["answer"]
                jobs.append(Job(f"{where} option {j}{'' if good else ' (must fail)'}",
                                build(subst(src, ex, opt), decl), expect, bad=not good))
            continue
        if t == "bug":
            lines = list(ex["lines"])
            buggy = "\n".join(lines)
            lines[ex["bug"]] = st["fix"]
            fixed = "\n".join(lines)
            jobs.append(Job(where + " (fixed)", build(src.replace("@FIXED@", fixed), decl), expect))
            jobs.append(Job(where + " (bug line, must fail)", build(src.replace("@FIXED@", buggy), decl), expect, bad=True))
            continue
        if t == "gap":
            jobs.append(Job(where, build(subst(src, ex), decl), expect, accept))
            for j, opt in enumerate(ex["options"]):
                if j != ex["answer"]:
                    jobs.append(Job(f"{where} option {j} (must fail)", build(subst(src, ex, opt), decl), expect, accept, bad=True))
            continue
        jobs.append(Job(where, build(subst(src, ex), decl), expect, accept))
    return jobs, manual


def main():
    course = json.loads(COURSE.read_text())
    checks = json.loads(CHECKS.read_text())
    jobs, manual, errors = [], [], []
    seen = set()
    for u in course["units"]:
        for i, b in enumerate(u["lesson"]):
            if b["type"] == "code":
                if not b["code"].startswith("program ") or MODE not in b["code"]:
                    errors.append(f"{u['id']}.lesson.{i}: lesson code must be a complete program with {MODE}")
                jobs.append(Job(f"{u['id']}.lesson.{i}", build(b["code"]), b.get("output", "")))
        for ex in u["exercises"]:
            seen.add(ex["id"])
            if ex["type"] == "pairs":
                continue
            if ex["id"] not in checks:
                errors.append(f"{ex['id']}: no check in checks.json")
                continue
            j, m = exercise_jobs(ex, checks[ex["id"]])
            jobs += j
            manual += m
    errors += [f"checks.json: unknown exercise {k}" for k in checks if k not in seen]
    for job in jobs:
        ok, msg = job()
        if not ok:
            errors.append(msg)
        elif VERBOSE:
            print("ok  ", job.where, "->", job.reason if job.bad else "")
    good = sum(1 for j in jobs if not j.bad)
    where = f"local {FPC}" if LOCAL else f"Compiler Explorer ({GODBOLT})"
    print(f"{len(jobs)} programs compiled and run with {where} ({good} must pass, {len(jobs) - good} must fail)")
    if manual:
        print(f"{len(manual)} answers checked by reasoning (no runnable code):")
        for m in manual:
            print("  -", m)
    if errors:
        print(f"\n{len(errors)} FAILED:")
        for e in errors:
            print("\n" + e)
        sys.exit(1)
    print("ALL PASSED")


if __name__ == "__main__":
    main()
