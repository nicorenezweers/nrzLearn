#!/usr/bin/env python3
"""Runs the code of the Fortran course on Compiler Explorer (gfortran) and compares the results.

Usage:
    python3 courses/fortran/verify/verify.py [--compiler=gfortran152] [--lessons] [unit ...]

See README.md for what is checked.
"""
import hashlib
import json
import os
import re
import sys
import time
import urllib.request

HERE = os.path.dirname(os.path.abspath(__file__))
COURSE = os.path.join(HERE, "..", "course.json")
CHECKS = os.path.join(HERE, "checks.json")
CACHE = os.path.join(HERE, ".cache.json")
FLAGS = "-std=f2018 -fcheck=all"

cache = json.load(open(CACHE)) if os.path.exists(CACHE) else {}


def indent(code, pad):
    return "\n".join(pad + l if l else "" for l in code.split("\n"))


def program(code, tpl=None):
    """A complete program from a snippet."""
    if tpl:
        m = re.search(r"^([ \t]*)\{\{CODE\}\}", tpl, re.M)
        pad = m.group(1)
        return tpl.replace(pad + "{{CODE}}", indent(code, pad), 1) + "\n"
    if re.search(r"^\s*program\s", code, re.M | re.I):
        return code + "\n"
    return "program main\n  implicit none\n" + indent(code, "  ") + "\nend program main\n"


def run(src, compiler):
    """(ok, stdout): ok is True when it compiled, ran and exited with 0."""
    key = f"{compiler}:{FLAGS}:" + hashlib.sha256(src.encode()).hexdigest()
    if key in cache:
        return tuple(cache[key])
    body = json.dumps({
        "source": src,
        "options": {"userArguments": FLAGS, "compilerOptions": {"executorRequest": True},
                    "filters": {"execute": True}, "tools": [], "libraries": []},
        "lang": "fortran", "allowStoreCodeDebug": True,
    }).encode()
    req = urllib.request.Request(f"https://godbolt.org/api/compiler/{compiler}/compile", data=body,
                                 headers={"Content-Type": "application/json", "Accept": "application/json"})
    for attempt in range(3):
        time.sleep(1.1)
        try:
            r = json.load(urllib.request.urlopen(req, timeout=120))
            break
        except Exception as e:  # network hiccup: retry
            if attempt == 2:
                raise
            print(f"retry after {e}")
            time.sleep(5)
    out = "".join(l["text"] + "\n" for l in r.get("stdout", []))
    res = (bool(r.get("didExecute")) and r.get("code") == 0, out)
    cache[key] = list(res)
    return res


def norm(s):
    return re.sub(r"[\s_'\"`]", "", s).lower()


def lines_out(s):
    return s + "\n" if s else ""


def main():
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    compiler = next((a.split("=", 1)[1] for a in sys.argv if a.startswith("--compiler=")), "gfortran152")
    lessons_only = "--lessons" in sys.argv
    course = json.load(open(COURSE))
    checks = json.load(open(CHECKS))
    jobs = []  # (label, src, mode, expected) ; mode: out | fail | differ
    problems = []

    def want(label, src, out=None, err=False, differ=None):
        if err:
            jobs.append((label, src, "fail", None))
        elif differ is not None:
            jobs.append((label, src, "differ", differ))
        else:
            jobs.append((label, src, "out", out))

    unchecked = []
    for u in course["units"]:
        if args and u["id"] not in args:
            continue
        for i, b in enumerate(u["lesson"]):
            if b["type"] == "code":
                want(f"{u['id']}.lesson.{i}", b["code"] + "\n", lines_out(b.get("output", "")))
        if lessons_only:
            continue
        for e in u["exercises"]:
            eid, kind = e["id"], e["type"]
            c = checks.get(eid)
            if c is None:
                unchecked.append(f"{eid} ({kind})")
                continue
            tpl = c.get("tpl")
            out = lines_out(c["out"]) if "out" in c else None
            err = c.get("err", False)
            for j, x in enumerate(c.get("extra", [])):
                want(f"{eid}.extra.{j}", x["src"] + "\n", lines_out(x["out"]) if "out" in x else None, x.get("err", False))
            if kind == "input" and out is not None and not any(norm(a) == norm(c["out"]) for a in e["answers"]):
                problems.append(f"FAIL {eid}: out {c['out']!r} is not among the answers {e['answers']}")
            if kind in ("input", "choice", "true_false") and (out is not None or err):
                want(eid, program(c.get("code") or e.get("code"), tpl), out, err)
            elif kind == "gap":
                for k, opt in enumerate(e["options"]):
                    src = program(e["code"].replace("___", opt), tpl)
                    if k == e["answer"]:
                        want(f"{eid}.option.{k}", src, out)
                    else:
                        want(f"{eid}.option.{k}", src, differ=out)
            elif kind == "bug":
                lines = e["lines"]
                want(f"{eid}.as-shown", program("\n".join(lines), tpl), differ=out)
                if "fixed" in c:
                    fixed = c["fixed"]
                else:
                    fl = list(lines)
                    fl[e["bug"]] = c["fix"]
                    fixed = "\n".join(fl)
                want(f"{eid}.fixed", program(fixed, tpl), out)
            elif kind == "order":
                lines = e["lines"]
                want(f"{eid}.order", program("\n".join(lines), tpl), out)
                for k in range(len(lines) - 1):
                    sw = list(lines)
                    sw[k], sw[k + 1] = sw[k + 1], sw[k]
                    want(f"{eid}.swap{k}", program("\n".join(sw), tpl), differ=out)
            elif kind == "blocks":
                want(f"{eid}.blocks", program(" ".join(e["tokens"]), tpl), out)
            elif not c.get("extra"):
                problems.append(f"FAIL {eid}: check has nothing to run")

    fails = len(problems)
    for p in problems:
        print(p)
    for n, (label, src, mode, exp) in enumerate(jobs, 1):
        ok, got = run(src, compiler)
        if n % 25 == 0:
            json.dump(cache, open(CACHE, "w"))
        good = (mode == "out" and ok and got == exp) or (mode == "fail" and not ok) \
            or (mode == "differ" and (not ok or got != exp))
        if not good:
            fails += 1
            print(f"FAIL {label} ({mode})\n--- program\n{src}--- expected\n{exp!r}\n--- got (ok={ok})\n{got[:1500]!r}")
    json.dump(cache, open(CACHE, "w"))
    print(f"{compiler} ({FLAGS}) on Compiler Explorer: {len(jobs)} programs run")
    if unchecked:
        print("Not run (review by reading): " + ", ".join(unchecked))
    print("OK" if fails == 0 else f"{fails} problem(s)")
    sys.exit(1 if fails else 0)


if __name__ == "__main__":
    main()
