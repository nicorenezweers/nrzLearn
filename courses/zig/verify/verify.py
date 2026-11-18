#!/usr/bin/env python3
"""Runs the code of the Zig course and compares the results.

Usage:
    python3 courses/zig/verify/verify.py [--godbolt] [unit ...]

See README.md for what is checked.
"""
import concurrent.futures as cf
import hashlib
import json
import os
import re
import subprocess
import sys
import tempfile
import time
import urllib.request

HERE = os.path.dirname(os.path.abspath(__file__))
COURSE = os.path.join(HERE, "..", "course.json")
CHECKS = os.path.join(HERE, "checks.json")
CACHE = os.path.join(HERE, ".cache.json")
GODBOLT = "z0170"
STD = 'const std = @import("std");'

cache = json.load(open(CACHE)) if os.path.exists(CACHE) else {}
zig_version = subprocess.run(["zig", "version"], capture_output=True, text=True).stdout.strip()


def indent(code, pad):
    return "\n".join(pad + l if l else "" for l in code.split("\n"))


def program(code, tpl=None):
    """A complete program from a snippet."""
    if tpl:
        m = re.search(r"^([ \t]*)\{\{CODE\}\}", tpl, re.M)
        pad = m.group(1)
        return tpl.replace(pad + "{{CODE}}", indent(code, pad), 1) + "\n"
    if "pub fn main" in code:
        return (code if STD in code else STD + "\n\n" + code) + "\n"
    return STD + "\n\npub fn main() !void {\n" + indent(code, "    ") + "\n}\n"


def run_local(src):
    """(ok, output): ok is True when it compiled and exited with 0."""
    key = "local:" + zig_version + ":" + hashlib.sha256(src.encode()).hexdigest()
    if key in cache:
        return tuple(cache[key])
    with tempfile.TemporaryDirectory() as d:
        path = os.path.join(d, "main.zig")
        with open(path, "w") as f:
            f.write(src)
        try:
            p = subprocess.run(["zig", "run", path], capture_output=True, text=True, timeout=120, cwd=d)
            res = (p.returncode == 0 and p.stdout == "", p.stderr + p.stdout)
        except subprocess.TimeoutExpired:
            res = (False, "timeout")
    cache[key] = list(res)
    return res


def run_godbolt(src):
    key = "godbolt:" + GODBOLT + ":" + hashlib.sha256(src.encode()).hexdigest()
    if key in cache:
        return tuple(cache[key])
    body = json.dumps({
        "source": src,
        "options": {"userArguments": "", "compilerOptions": {"executorRequest": True},
                    "filters": {"execute": True}, "tools": [], "libraries": []},
        "lang": "zig", "allowStoreCodeDebug": True,
    }).encode()
    req = urllib.request.Request(f"https://godbolt.org/api/compiler/{GODBOLT}/compile", data=body,
                                 headers={"Content-Type": "application/json", "Accept": "application/json"})
    time.sleep(1.1)
    r = json.load(urllib.request.urlopen(req, timeout=120))
    out = "".join(l["text"] + "\n" for l in r.get("stderr", []) + r.get("stdout", []))
    res = (bool(r.get("didExecute")) and r.get("code") == 0, out)
    cache[key] = list(res)
    return res


def fmt_ok(src):
    p = subprocess.run(["zig", "fmt", "--stdin"], input=src, capture_output=True, text=True)
    return p.returncode == 0 and p.stdout == src


def norm(s):
    return re.sub(r"[\s_'\"`]", "", s).lower()


def main():
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    use_godbolt = "--godbolt" in sys.argv
    course = json.load(open(COURSE))
    checks = json.load(open(CHECKS))
    jobs = []  # (label, src, expected) ; expected: str output, None = must fail, "!" + out = must not print out
    fmt_jobs = []  # (label, src)

    def want(label, src, out=None, err=False, differ=None):
        if err:
            jobs.append((label, src, ("fail", None)))
        elif differ is not None:
            jobs.append((label, src, ("differ", differ)))
        else:
            jobs.append((label, src, ("out", out)))

    unchecked = []
    for u in course["units"]:
        if args and u["id"] not in args:
            continue
        for i, b in enumerate(u["lesson"]):
            if b["type"] != "code":
                continue
            label = f"{u['id']}.lesson.{i}"
            want(label, b["code"] + "\n", b.get("output", "") + "\n" if "output" in b else "")
            fmt_jobs.append((label, b["code"] + "\n"))
        for e in u["exercises"]:
            eid, kind = e["id"], e["type"]
            c = checks.get(eid)
            if c is None:
                unchecked.append(f"{eid} ({kind})")
                continue
            tpl = c.get("tpl")
            out = c["out"] + "\n" if "out" in c else None
            err = c.get("err", False)
            for j, x in enumerate(c.get("extra", [])):
                want(f"{eid}.extra.{j}", x["src"] + "\n", x["out"] + "\n" if "out" in x else None, x.get("err", False))
            if kind == "input" and out is not None and not any(norm(a) == norm(c["out"]) for a in e["answers"]):
                print(f"FAIL {eid}: out {c['out']!r} is not among the answers {e['answers']}")
            if kind in ("input", "choice", "true_false") and (out is not None or err):
                code = c.get("code") or e.get("code")
                src = program(code, tpl)
                want(eid, src, out, err)
                if e.get("code") and not err and not c.get("code"):
                    fmt_jobs.append((eid, src))
            elif kind == "gap":
                for k, opt in enumerate(e["options"]):
                    src = program(e["code"].replace("___", opt), tpl)
                    if k == e["answer"]:
                        want(f"{eid}.option.{k}", src, out)
                        fmt_jobs.append((eid, src))
                    else:
                        want(f"{eid}.option.{k}", src, differ=out)
            elif kind == "bug":
                lines = e["lines"]
                want(f"{eid}.as-shown", program("\n".join(lines), tpl), differ=out)
                fixed = list(lines)
                fixed[e["bug"]] = c["fix"]
                src = program("\n".join(fixed), tpl)
                want(f"{eid}.fixed", src, out)
                fmt_jobs.append((eid, src))
            elif kind == "order":
                lines = e["lines"]
                src = program("\n".join(lines), tpl)
                want(f"{eid}.order", src, out)
                fmt_jobs.append((eid, src))
                for k in range(len(lines) - 1):
                    sw = list(lines)
                    sw[k], sw[k + 1] = sw[k + 1], sw[k]
                    want(f"{eid}.swap{k}", program("\n".join(sw), tpl), differ=out)
            elif kind == "blocks":
                want(f"{eid}.blocks", program(" ".join(e["tokens"]), tpl), out)
            elif not c.get("extra"):
                print(f"FAIL {eid}: check has nothing to run")

    fails = 0
    with cf.ThreadPoolExecutor(max_workers=6) as pool:
        results = list(pool.map(lambda j: run_local(j[1]), jobs))
    for (label, src, (mode, exp)), (ok, got) in zip(jobs, results):
        good = (mode == "out" and ok and got == exp) or (mode == "fail" and not ok) \
            or (mode == "differ" and (not ok or got != exp))
        if not good:
            fails += 1
            print(f"FAIL {label} ({mode})\n--- program\n{src}--- expected\n{exp!r}\n--- got (ok={ok})\n{got[:1500]}")
    for label, src in fmt_jobs:
        if not fmt_ok(src):
            fails += 1
            p = subprocess.run(["zig", "fmt", "--stdin"], input=src, capture_output=True, text=True)
            print(f"FMT {label}\n--- program\n{src}--- zig fmt\n{p.stdout or p.stderr}")
    gb = 0
    if use_godbolt:
        for label, src, (mode, exp) in jobs:
            if ".lesson." not in label:
                continue
            ok, got = run_godbolt(src)
            gb += 1
            if not (ok and got == exp):
                fails += 1
                print(f"GODBOLT FAIL {label}\n--- expected\n{exp!r}\n--- got (ok={ok})\n{got[:1500]}")
            json.dump(cache, open(CACHE, "w"))
    json.dump(cache, open(CACHE, "w"))
    print(f"zig {zig_version}: {len(jobs)} programs run, {len(fmt_jobs)} formatted"
          + (f", {gb} lesson blocks on Compiler Explorer ({GODBOLT})" if use_godbolt else ""))
    if unchecked:
        print("Not run (review by reading): " + ", ".join(unchecked))
    print("OK" if fails == 0 else f"{fails} problem(s)")
    sys.exit(1 if fails else 0)


if __name__ == "__main__":
    main()
