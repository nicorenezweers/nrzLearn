#!/usr/bin/env python3
"""Runs the code of the COBOL course and compares the results.

Usage:
    python3 courses/cobol/verify/verify.py [--godbolt] [unit ...]

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
import time
import urllib.request

HERE = os.path.dirname(os.path.abspath(__file__))
COURSE = os.path.join(HERE, "..", "course.json")
CHECKS = os.path.join(HERE, "checks.json")
CACHE = os.path.join(HERE, ".cache.json")
GODBOLT = "gnucobol32"
HEADER = ">>SOURCE FORMAT FREE\nIDENTIFICATION DIVISION.\nPROGRAM-ID. DEMO.\n"
PARAGRAPH = re.compile(r"^[A-Z0-9-]+\.$")
DATA_LINE = re.compile(r"^\s*(0[1-9]|[1-4][0-9]|66|77|88)\s")

cache = json.load(open(CACHE)) if os.path.exists(CACHE) else {}
cobc_version = subprocess.run(["cobc", "--version"], capture_output=True, text=True).stdout.split("\n")[0]


def indent(code, pad):
    return "\n".join(pad + l if l else "" for l in code.split("\n"))


def program(code, tpl=None):
    """A complete program from a snippet.

    A snippet without PROGRAM-ID is split: lines that start with a level number
    (01, 05, 77, 88 ...) go into WORKING-STORAGE, all other lines into the
    PROCEDURE DIVISION, followed by STOP RUN (unless the snippet has one or
    has paragraphs).
    """
    if tpl:
        m = re.search(r"^([ \t]*)\{\{CODE\}\}", tpl, re.M)
        pad = m.group(1)
        return program(tpl.replace(pad + "{{CODE}}", indent(code, pad), 1))
    if "PROGRAM-ID" in code:
        return code + "\n"
    lines = code.split("\n")
    data = [l for l in lines if DATA_LINE.match(l)]
    proc = [l for l in lines if not DATA_LINE.match(l)]
    src = HEADER
    if data:
        src += "DATA DIVISION.\nWORKING-STORAGE SECTION.\n" + "\n".join(data) + "\n"
    src += "PROCEDURE DIVISION.\n" + indent("\n".join(proc), "    ") + "\n"
    if "STOP RUN" not in code and not any(PARAGRAPH.match(l) for l in proc):
        src += "    STOP RUN.\n"
    return src


def run_local(src):
    """(ok, output, warnings): ok is True when it compiled and exited with 0."""
    key = "local:" + cobc_version + ":" + hashlib.sha256(src.encode()).hexdigest()
    if key in cache:
        return tuple(cache[key])
    with tempfile.TemporaryDirectory() as d:
        path = os.path.join(d, "main.cob")
        with open(path, "w") as f:
            f.write(src)
        c = subprocess.run(["cobc", "-x", "-o", "main", "main.cob"], capture_output=True, text=True, cwd=d)
        warn = "\n".join(l for l in c.stderr.split("\n") if "warning" in l)
        if c.returncode != 0:
            res = (False, c.stderr, warn)
        else:
            try:
                p = subprocess.run(["./main"], capture_output=True, text=True, timeout=10, cwd=d)
                res = (p.returncode == 0, p.stdout + p.stderr, warn)
            except subprocess.TimeoutExpired:
                res = (False, "timeout", warn)
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
        "lang": "cobol", "allowStoreCodeDebug": True,
    }).encode()
    req = urllib.request.Request(f"https://godbolt.org/api/compiler/{GODBOLT}/compile", data=body,
                                 headers={"Content-Type": "application/json", "Accept": "application/json"})
    time.sleep(1.1)
    r = json.load(urllib.request.urlopen(req, timeout=120))
    out = "".join(l["text"] + "\n" for l in r.get("stdout", []))
    res = (bool(r.get("didExecute")) and r.get("code") == 0, out)
    cache[key] = list(res)
    return res


def norm(s):
    return re.sub(r"[\s_'\"`]", "", s).lower()


def main():
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    use_godbolt = "--godbolt" in sys.argv
    course = json.load(open(COURSE))
    checks = json.load(open(CHECKS))
    jobs = []  # (label, src, (mode, expected)); mode: out | fail | differ
    fails = 0

    def problem(msg):
        nonlocal fails
        fails += 1
        print("FAIL " + msg)

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
            if "output" in b:
                if not b["code"].startswith(">>SOURCE FORMAT FREE\n") or "PROGRAM-ID" not in b["code"]:
                    problem(f"{label}: a lesson block with output must be a complete free-format program")
                if any(l != l.rstrip() for l in b["output"].split("\n")):
                    problem(f"{label}: output has trailing spaces")
                want(label, program(b["code"]), b["output"] + "\n")
            else:
                want(label, program(b["code"]), "")
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
                src = program(x["src"], x.get("tpl", tpl))
                if "differ" in x:
                    want(f"{eid}.extra.{j}", src, differ=x["differ"] + "\n")
                else:
                    want(f"{eid}.extra.{j}", src, x["out"] + "\n" if "out" in x else None, x.get("err", False))
            if kind == "input" and out is not None and not any(norm(a) == norm(c["out"]) for a in e["answers"]):
                problem(f"{eid}: out {c['out']!r} is not among the answers {e['answers']}")
            if kind == "choice" and out is not None and not c.get("code") and not c.get("no_option"):
                opts = [o.strip("`") for o in e["options"]]
                if opts[e["answer"]] != c["out"] or opts.count(c["out"]) != 1:
                    problem(f"{eid}: out {c['out']!r} must equal exactly the right option")
            if kind in ("input", "choice", "true_false") and (out is not None or err):
                code = c.get("code") or e.get("code")
                want(eid, program(code, tpl), out, err)
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
                if len(lines) <= 5:
                    perms = [p for p in itertools.permutations(lines) if list(p) != lines]
                else:
                    perms = []
                    for k in range(len(lines) - 1):
                        sw = list(lines)
                        sw[k], sw[k + 1] = sw[k + 1], sw[k]
                        perms.append(sw)
                for k, p in enumerate(perms):
                    want(f"{eid}.perm{k}", program("\n".join(p), tpl), differ=out)
            elif kind == "blocks":
                want(f"{eid}.blocks", program(" ".join(e["tokens"]), tpl), out)
            elif not c.get("extra"):
                problem(f"{eid}: check has nothing to run")

    with cf.ThreadPoolExecutor(max_workers=6) as pool:
        results = list(pool.map(lambda j: run_local(j[1]), jobs))
    for (label, src, (mode, exp)), (ok, got, warn) in zip(jobs, results):
        if ok and got and not got.endswith("\n"):
            got += "\n"
        good = (mode == "out" and ok and got == exp and not warn) or (mode == "fail" and not ok) \
            or (mode == "differ" and (not ok or got != exp or warn))
        if not good:
            problem(f"{label} ({mode})\n--- program\n{src}--- expected\n{exp!r}\n--- got (ok={ok})\n{got[:1500]}\n{warn}")
    json.dump(cache, open(CACHE, "w"))
    gb = 0
    if use_godbolt:
        for label, src, (mode, exp) in jobs:
            if ".lesson." not in label:
                continue
            ok, got = run_godbolt(src)
            gb += 1
            if not (ok and got == exp):
                problem(f"GODBOLT {label}\n--- expected\n{exp!r}\n--- got (ok={ok})\n{got[:1500]}")
            json.dump(cache, open(CACHE, "w"))
    print(f"{cobc_version}: {len(jobs)} programs run"
          + (f", {gb} lesson blocks on Compiler Explorer ({GODBOLT})" if use_godbolt else ""))
    if unchecked:
        print("Not run (review by reading): " + ", ".join(unchecked))
    print("OK" if fails == 0 else f"{fails} problem(s)")
    sys.exit(1 if fails else 0)


if __name__ == "__main__":
    main()
