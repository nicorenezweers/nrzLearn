#!/usr/bin/env python3
"""Runs the code of the V course and compares the results.

Usage:
    python3 courses/v/verify/verify.py [--nofmt] [unit ...]

See README.md for what is checked.
"""
import concurrent.futures as cf
import hashlib
import itertools
import json
import os
import re
import subprocess
import sys
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
COURSE = os.path.join(HERE, "..", "course.json")
CHECKS = os.path.join(HERE, "checks.json")
CACHE = os.path.join(HERE, ".cache.json")

cache = json.load(open(CACHE)) if os.path.exists(CACHE) else {}
v_version = subprocess.run(["v", "version"], capture_output=True, text=True).stdout.strip()


def indent(code, pad):
    return "\n".join(pad + l if l else "" for l in code.split("\n"))


def program(code, tpl=None):
    """A complete program from a snippet."""
    if tpl:
        m = re.search(r"^([ \t]*)\{\{CODE\}\}", tpl, re.M)
        pad = m.group(1)
        return tpl.replace(pad + "{{CODE}}", indent(code, pad), 1) + "\n"
    if re.search(r"^fn main\(\)", code, re.M):
        return code + "\n"
    return "fn main() {\n" + indent(code, "    ") + "\n}\n"


def key(kind, src):
    return kind + ":" + v_version + ":" + hashlib.sha256(src.encode()).hexdigest()


def run(src):
    """(ok, clean, stdout): ok = compiled and exited with 0, clean = no compiler messages."""
    k = key("run", src)
    if k in cache:
        return tuple(cache[k])
    with tempfile.TemporaryDirectory() as d:
        path = os.path.join(d, "main.v")
        with open(path, "w") as f:
            f.write(src)
        try:
            p = subprocess.run(["v", "run", path], capture_output=True, text=True, timeout=20, cwd=d)
            res = (p.returncode == 0, p.stderr.strip() == "", p.stdout if p.returncode == 0 else p.stderr + p.stdout)
        except subprocess.TimeoutExpired:
            res = (False, False, "timeout")
    cache[k] = list(res)
    return res


def fmt_ok(src):
    """True when `v fmt` (tabs shown as 4 spaces) changes nothing."""
    k = key("fmt", src)
    if k not in cache:
        with tempfile.TemporaryDirectory() as d:
            path = os.path.join(d, "main.v")
            with open(path, "w") as f:
                f.write(src)
            p = subprocess.run(["v", "fmt", path], capture_output=True, text=True, timeout=60)
            cache[k] = p.returncode == 0 and p.stdout.replace("\t", "    ") == src
    return cache[k]


def norm(s):
    return re.sub(r"[\s_'\"`]", "", s).lower()


class Checker:
    def __init__(self, nofmt):
        self.nofmt = nofmt
        self.errors = []
        self.runs = 0

    def fail(self, where, msg):
        self.errors.append(f"{where}: {msg}")

    def good(self, where, src, out, fmt=True):
        """src must compile without messages, run and print exactly out."""
        self.runs += 1
        ok, clean, got = run(src)
        if not ok:
            self.fail(where, "does not run:\n" + got + "\n--- program ---\n" + src)
        elif not clean:
            self.fail(where, "compiler messages (warnings/notices):\n--- program ---\n" + src)
        elif got.rstrip("\n") != out:
            self.fail(where, f"output {got.rstrip(chr(10))!r} != expected {out!r}")
        if fmt and not self.nofmt and not fmt_ok(src):
            self.fail(where, "not formatted like `v fmt`:\n" + src)

    def bad(self, where, src, out):
        """src must fail, warn or print something other than out."""
        self.runs += 1
        ok, clean, got = run(src)
        if ok and clean and got.rstrip("\n") == out:
            self.fail(where, "a wrong variant prints the expected output too:\n" + src)

    def err(self, where, src):
        self.runs += 1
        ok, _, _ = run(src)
        if ok:
            self.fail(where, "should not compile/run but does:\n" + src)

    def lesson(self, uid, i, block):
        if block["type"] == "code":
            self.good(f"{uid}.lesson.{i}", program(block["code"]), block.get("output", ""))

    def exercise(self, ex, chk):
        eid, t, tpl = ex["id"], ex["type"], chk.get("tpl")
        fmt = not chk.get("nofmt")
        for j, x in enumerate(chk.get("extra", [])):
            if x.get("err"):
                self.err(f"{eid}.extra.{j}", program(x["src"], x.get("tpl")))
            else:
                self.good(f"{eid}.extra.{j}", program(x["src"], x.get("tpl")), x["out"], False)
        if "out" not in chk:
            return
        out = chk["out"]
        if t in ("input", "choice", "true_false"):
            code = chk.get("code", ex.get("code"))
            if chk.get("err"):
                self.err(eid, program(code, tpl))
            else:
                self.good(eid, program(code, tpl), out, fmt)
            if t == "input" and norm(out) not in [norm(a) for a in ex["answers"]]:
                self.fail(eid, f"output {out!r} is not among the answers {ex['answers']}")
        elif t == "gap":
            for j, o in enumerate(ex["options"]):
                src = program(ex["code"].replace("___", o), tpl)
                (self.good(eid, src, out, fmt) if j == ex["answer"] else self.bad(f"{eid} option {j}", src, out))
        elif t == "bug":
            lines = ex["lines"]
            self.bad(f"{eid} as shown", program("\n".join(lines), tpl), out)
            fixed = lines[:ex["bug"]] + [chk["fix"]] + lines[ex["bug"] + 1:]
            self.good(f"{eid} fixed", program("\n".join(fixed), tpl), out, fmt)
        elif t == "order":
            lines = ex["lines"]
            self.good(eid, program("\n".join(lines), tpl), out, fmt)
            n = len(lines)
            if n <= 5:
                perms = [p for p in itertools.permutations(range(n)) if list(p) != list(range(n))]
            else:
                perms = []
                for a, b in itertools.combinations(range(n), 2):
                    p = list(range(n))
                    p[a], p[b] = p[b], p[a]
                    perms.append(p)
            jobs = [program("\n".join(lines[k] for k in p), tpl) for p in perms]
            with cf.ThreadPoolExecutor(8) as pool:
                list(pool.map(run, jobs))
            for src in jobs:
                self.bad(f"{eid} other order", src, out)
        elif t == "blocks":
            self.good(eid, program(" ".join(ex["tokens"]), tpl), out, False)
        else:
            self.fail(eid, f"no check for type {t}")


def main():
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    nofmt = "--nofmt" in sys.argv
    course = json.load(open(COURSE))
    checks = json.load(open(CHECKS))
    c = Checker(nofmt)
    unchecked = []
    facts = checks.pop("_facts", [])
    try:
        for j, x in enumerate(facts):
            where = f"_facts.{j} ({x['about']})"
            if x.get("warn"):
                c.runs += 1
                ok, clean, _ = run(program(x["src"]))
                if not ok or clean:
                    c.fail(where, "should compile with a warning:\n" + x["src"])
            elif x.get("err"):
                c.err(where, program(x["src"]))
            else:
                c.good(where, program(x["src"]), x["out"], False)
        for u in course["units"]:
            if args and u["id"] not in args:
                continue
            for i, b in enumerate(u["lesson"]):
                c.lesson(u["id"], i, b)
            for ex in u["exercises"]:
                if ex["id"] in checks:
                    c.exercise(ex, checks[ex["id"]])
                else:
                    unchecked.append(ex["id"])
            print(f"{u['id']}: done", file=sys.stderr)
    finally:
        json.dump(cache, open(CACHE, "w"))
    known = {e["id"] for u in course["units"] for e in u["exercises"]}
    for k in checks:
        if k not in known:
            c.fail(k, "check for an unknown exercise")
    print(f"{v_version}: {c.runs} programs checked")
    print("without check (review by reading): " + ", ".join(unchecked))
    for e in c.errors:
        print("FAIL " + e + "\n")
    print("ALL PASS" if not c.errors else f"{len(c.errors)} problems")
    sys.exit(1 if c.errors else 0)


if __name__ == "__main__":
    main()
