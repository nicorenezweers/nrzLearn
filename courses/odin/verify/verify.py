#!/usr/bin/env python3
"""Runs the code of the Odin course on Compiler Explorer and compares the results.

Usage:
    python3 courses/odin/verify/verify.py [unit ...]
    python3 courses/odin/verify/verify.py --run file.odin

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
COMPILER = "odin202611a"
HEAD = 'package main\n\nimport "core:fmt"\nimport "core:strings"\n'

cache = json.load(open(CACHE)) if os.path.exists(CACHE) else {}


def indent(code, pad):
    return "\n".join(pad + l if l else "" for l in code.split("\n"))


def program(code, tpl=None):
    """A complete program from a snippet."""
    if tpl:
        m = re.search(r"^([ \t]*)\{\{CODE\}\}", tpl, re.M)
        pad = m.group(1)
        return tpl.replace(pad + "{{CODE}}", indent(code, pad), 1) + "\n"
    if "package main" in code:
        return code + "\n"
    if "main :: proc" in code:
        return HEAD + "\n" + code + "\n"
    return HEAD + "\nmain :: proc() {\n" + indent(code, "    ") + "\n}\n"


def run(src):
    """(ok, output): ok is True when it compiled, ran and exited with 0."""
    key = COMPILER + ":" + hashlib.sha256(src.encode()).hexdigest()
    if key in cache:
        return tuple(cache[key])
    body = json.dumps({
        "source": src,
        "options": {"userArguments": "", "compilerOptions": {"executorRequest": True},
                    "filters": {"execute": True}, "tools": [], "libraries": []},
        "lang": "odin", "allowStoreCodeDebug": True,
    }).encode()
    req = urllib.request.Request(f"https://godbolt.org/api/compiler/{COMPILER}/compile", data=body,
                                 headers={"Content-Type": "application/json", "Accept": "application/json"})
    for attempt in range(4):
        time.sleep(1.1)
        try:
            r = json.load(urllib.request.urlopen(req, timeout=120))
            break
        except Exception as e:
            if attempt == 3:
                raise
            print(f"retry after {e}")
            time.sleep(5)
    out = "".join(l["text"] + "\n" for l in r.get("stdout", []))
    err = "".join(l["text"] + "\n" for l in r.get("stderr", []) + r.get("buildResult", {}).get("stderr", []))
    ok = bool(r.get("didExecute")) and r.get("code") == 0
    res = (ok, out if ok else out + err)
    cache[key] = list(res)
    json.dump(cache, open(CACHE, "w"))
    return res


def nl(s):
    """Expected stdout: the text plus the final newline, or nothing."""
    return s + "\n" if s else ""


def norm(s):
    return re.sub(r"[\s_'\"`]", "", s).lower()


def main():
    if sys.argv[1:2] == ["--run"]:
        ok, out = run(open(sys.argv[2]).read())
        print(f"ok={ok}\n{out}", end="")
        return
    args = sys.argv[1:]
    course = json.load(open(COURSE))
    checks = json.load(open(CHECKS))
    jobs = []  # (label, src, (mode, expected)); mode: out, fail, differ

    def want(label, src, out=None, err=False, differ=None):
        if err:
            jobs.append((label, src, ("fail", None)))
        elif differ is not None:
            jobs.append((label, src, ("differ", differ)))
        else:
            jobs.append((label, src, ("out", out)))

    unchecked = []
    problems = []
    for u in course["units"]:
        if args and u["id"] not in args:
            continue
        for i, b in enumerate(u["lesson"]):
            if b["type"] != "code":
                continue
            label = f"{u['id']}.lesson.{i}"
            if "output" in b and "package main" not in b["code"]:
                problems.append(f"FAIL {label}: block with output is not a complete program")
            want(label, b["code"] + "\n", b["output"] + "\n" if "output" in b else "")
        for e in u["exercises"]:
            eid, kind = e["id"], e["type"]
            c = checks.get(eid)
            if c is None:
                unchecked.append(f"{eid} ({kind})")
                continue
            tpl = c.get("tpl")
            out = nl(c["out"]) if "out" in c else None
            err = c.get("err", False)
            for j, x in enumerate(c.get("extra", [])):
                want(f"{eid}.extra.{j}", program(x["src"], x.get("tpl")),
                     nl(x["out"]) if "out" in x else None, x.get("err", False))
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
                fixed = list(lines)
                fixed[e["bug"]] = c["fix"]
                want(f"{eid}.fixed", program("\n".join(fixed), tpl), out)
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

    for p in problems:
        print(p)
    fails = len(problems)
    for label, src, (mode, exp) in jobs:
        ok, got = run(src)
        good = (mode == "out" and ok and got == exp) or (mode == "fail" and not ok) \
            or (mode == "differ" and (not ok or got != exp))
        if not good:
            fails += 1
            print(f"FAIL {label} ({mode})\n--- program\n{src}--- expected\n{exp!r}\n--- got (ok={ok})\n{got[:1500]}")
    print(f"Odin on Compiler Explorer ({COMPILER}): {len(jobs)} programs run")
    if unchecked:
        print("Not run (review by reading): " + ", ".join(unchecked))
    print("OK" if fails == 0 else f"{fails} problem(s)")
    sys.exit(1 if fails else 0)


if __name__ == "__main__":
    main()
