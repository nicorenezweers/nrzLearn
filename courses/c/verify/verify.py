#!/usr/bin/env python3
"""Compile and run every piece of C in courses/c/course.json.

Usage: python3 courses/c/verify/verify.py [-v] [--remote]

--remote also runs every program that must pass on Compiler Explorer
(godbolt.org, x86-64 GCC on Linux, same flags). Answers are cached in
verify/.cache/ so that each program is sent only once.

* Lesson code blocks: compiled with cc -std=c17 -Wall -Wextra -pedantic,
  run, and stdout (without the final newline) compared with `output`. Blocks
  without `output` must compile and print nothing.
* Exercises: checked with the steps in checks.json (see README.md).

A "good" program must compile without any warning, exit with 0 and print the
expected text. A "bad" program (a wrong gap option, a wrong choice option, the
line with the bug) must fail: compile error, warning, crash, non-zero exit,
timeout or a different output.
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
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

HERE = Path(__file__).resolve().parent
COURSE = HERE.parent / "course.json"
CHECKS = HERE / "checks.json"
CC = os.environ.get("CC", "cc")
FLAGS = ["-std=c17", "-Wall", "-Wextra", "-pedantic"]
PRELUDE = "#include <stdio.h>\n#include <string.h>\n"
VERBOSE = "-v" in sys.argv
REMOTE = "--remote" in sys.argv
GODBOLT = os.environ.get("GODBOLT_COMPILER", "cg162")
CACHE = HERE / ".cache" / f"godbolt-{GODBOLT}.json"
WORK = Path(tempfile.mkdtemp(prefix="c-verify-"))


def norm(s):
    """The app's comparison: ignore case, spaces, `_`, quotes and backticks."""
    return re.sub(r"[\s_\"'`]", "", s).lower()


def build(src):
    """Wraps a snippet in main() unless it already is a complete program."""
    if "int main(" in src:
        return src if "#include" in src else PRELUDE + "\n" + src
    body = "\n".join("    " + l if l else l for l in src.split("\n"))
    return PRELUDE + "\nint main(void) {\n" + body + "\n    return 0;\n}\n"


def run(src):
    """Returns (ok, stdout, diagnostics). ok means: no warning, exit code 0."""
    prog = build(src)
    h = hashlib.sha1(prog.encode()).hexdigest()[:16]
    cpp, exe = WORK / f"{h}.c", WORK / h
    cpp.write_text(prog)
    c = subprocess.run([CC, *FLAGS, str(cpp), "-o", str(exe)], capture_output=True, text=True)
    if c.returncode != 0 or c.stderr.strip():
        return False, None, c.stderr.strip() or "compile error"
    try:
        r = subprocess.run([str(exe)], capture_output=True, text=True, timeout=5)
    except subprocess.TimeoutExpired:
        return False, None, "timeout"
    out = r.stdout[:-1] if r.stdout.endswith("\n") else r.stdout
    if r.returncode != 0:
        return False, out, f"exit code {r.returncode}"
    return True, out, ""


def run_remote(src, cache):
    """Like run(), but compiled and run by GCC on Compiler Explorer."""
    prog = build(src)
    h = hashlib.sha1(prog.encode()).hexdigest()
    if h not in cache:
        body = {"source": prog, "lang": "c", "allowStoreCodeDebug": True,
                "options": {"userArguments": " ".join(FLAGS + ["-fdiagnostics-color=never"]), "compilerOptions": {"executorRequest": True},
                            "filters": {"execute": True}, "tools": [], "libraries": []}}
        req = urllib.request.Request(f"https://godbolt.org/api/compiler/{GODBOLT}/compile",
                                     data=json.dumps(body).encode(),
                                     headers={"Content-Type": "application/json", "Accept": "application/json"})
        with urllib.request.urlopen(req, timeout=60) as r:
            d = json.load(r)
        b = d.get("buildResult", {})
        cache[h] = {"build": "\n".join(x["text"] for x in b.get("stderr", [])) if b else "",
                    "build_code": b.get("code", d.get("code")),
                    "ran": d.get("didExecute", False), "code": d.get("code"),
                    "stdout": "".join(x["text"] + "\n" for x in d.get("stdout", []))}
        CACHE.parent.mkdir(exist_ok=True)
        CACHE.write_text(json.dumps(cache, indent=1))
        time.sleep(1)
    c = cache[h]
    if c["build"].strip() or not c["ran"]:
        return False, None, c["build"] or "did not run"
    out = c["stdout"][:-1] if c["stdout"].endswith("\n") else c["stdout"]
    if c["code"] != 0:
        return False, out, f"exit code {c['code']}"
    return True, out, ""


def remote_check(jobs):
    cache = json.loads(CACHE.read_text()) if CACHE.exists() else {}
    errors, n = [], 0
    for job in jobs:
        if job.bad:
            continue
        n += 1
        ok, out, diag = run_remote(job.src, cache)
        if job.accept is not None:
            match = ok and norm(out) in {norm(a) for a in job.accept}
        else:
            match = ok and out == job.expect
        if not match:
            errors.append(f"{job.where} (GCC {GODBOLT}): {diag or repr(out)}")
        elif VERBOSE:
            print("ok  ", job.where, f"(GCC {GODBOLT})")
    print(f"{n} programs compiled and run with GCC ({GODBOLT}) on Compiler Explorer")
    return errors


def unbacktick(s):
    return s.replace("`", "")


class Job:
    def __init__(self, where, src, expect=None, accept=None, bad=False):
        self.where, self.src, self.expect, self.accept, self.bad = where, src, expect, accept, bad

    def __call__(self):
        ok, out, diag = run(self.src)
        if self.bad and self.expect is None and self.accept is None:
            matches = ok
        elif self.accept is not None:
            matches = ok and norm(out) in {norm(a) for a in self.accept}
        else:
            matches = ok and out == self.expect
        if self.bad:
            return (not matches), f"{self.where}: expected to fail, but compiled and printed {out!r}"
        if not ok:
            return False, f"{self.where}: {diag}\n--- program ---\n{build(self.src)}"
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
            jobs.append(Job(where + " (must fail)", subst(src, ex), expect, bad=True))
            continue
        if kind == "options":
            if expect is None:
                raise SystemExit(f"{where}: needs 'expect'")
            for j, opt in enumerate(ex["options"]):
                good = j == ex["answer"]
                jobs.append(Job(f"{where} option {j}{'' if good else ' (must fail)'}", subst(src, ex, opt), expect, bad=not good))
            continue
        if t == "bug":
            lines = list(ex["lines"])
            buggy = "\n".join(lines)
            lines[ex["bug"]] = st["fix"]
            fixed = "\n".join(lines)
            jobs.append(Job(where + " (fixed)", src.replace("@FIXED@", fixed), expect))
            jobs.append(Job(where + " (bug line, must fail)", src.replace("@FIXED@", buggy), expect, bad=True))
            continue
        if t == "gap":
            jobs.append(Job(where, subst(src, ex), expect, accept))
            for j, opt in enumerate(ex["options"]):
                if j != ex["answer"]:
                    jobs.append(Job(f"{where} option {j} (must fail)", subst(src, ex, opt), expect, accept, bad=True))
            continue
        jobs.append(Job(where, subst(src, ex), expect, accept))
    return jobs, manual


def main():
    course = json.loads(COURSE.read_text())
    checks = json.loads(CHECKS.read_text())
    jobs, manual, errors = [], [], []
    seen = set()
    for u in course["units"]:
        for i, b in enumerate(u["lesson"]):
            if b["type"] == "code":
                jobs.append(Job(f"{u['id']}.lesson.{i}", b["code"], b.get("output", "")))
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
    with ThreadPoolExecutor(max_workers=os.cpu_count() or 4) as pool:
        results = list(pool.map(lambda job: job(), jobs))
    for job, (ok, msg) in zip(jobs, results):
        if not ok:
            errors.append(msg)
        elif VERBOSE:
            print("ok  ", job.where)
    good = sum(1 for j in jobs if not j.bad)
    print(f"{len(jobs)} programs compiled and run ({good} must pass, {len(jobs) - good} must fail)")
    if REMOTE:
        errors += remote_check(jobs)
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
