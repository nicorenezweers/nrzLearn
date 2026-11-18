#!/usr/bin/env python3
"""Runs the code of the Ada course on Compiler Explorer and compares the results.

Usage:
    python3 courses/ada/verify/verify.py [unit ...]

See README.md for what is checked.
"""
import hashlib
import itertools
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
COMPILER = os.environ.get("ADA_COMPILER", "gnat152")
FLAGS = "-gnat2012"
WITH = "with Ada.Text_IO; use Ada.Text_IO;\n\n"
NOISE = re.compile(r"file name does not match unit name")

cache = json.load(open(CACHE)) if os.path.exists(CACHE) else {}


def program(code, tpl=None):
    """A complete program from a snippet.

    - with a template: the snippet replaces {{CODE}};
    - "with ..." at the start: already complete;
    - "procedure Main" at the start: the context clause is added;
    - a line "begin": declarations and statements, wrapped in procedure Main;
    - otherwise: statements only.
    """
    if tpl:
        code = tpl.replace("{{CODE}}", code)
    if code.startswith("with "):
        return code + "\n"
    if code.startswith("procedure Main"):
        return WITH + code + "\n"
    if re.search(r"^begin$", code, re.M):
        return WITH + "procedure Main is\n" + code + "\nend Main;\n"
    return WITH + "procedure Main is\nbegin\n" + code + "\nend Main;\n"


def run(src):
    """(status, stdout, messages): status is ok, compile or runtime."""
    key = f"{COMPILER}:{FLAGS}:" + hashlib.sha256(src.encode()).hexdigest()
    if key in cache:
        return tuple(cache[key])
    body = json.dumps({
        "source": src,
        "options": {"userArguments": FLAGS, "compilerOptions": {"executorRequest": True},
                    "filters": {"execute": True}, "tools": [], "libraries": []},
        "lang": "ada", "allowStoreCodeDebug": True,
    }).encode()
    req = urllib.request.Request(f"https://godbolt.org/api/compiler/{COMPILER}/compile", data=body,
                                 headers={"Content-Type": "application/json", "Accept": "application/json"})
    for attempt in range(4):
        time.sleep(1.1)
        try:
            r = json.load(urllib.request.urlopen(req, timeout=120))
            break
        except Exception as e:  # noqa: BLE001
            if attempt == 3:
                raise
            print(f"  retry ({e})", file=sys.stderr)
            time.sleep(5)
    build = r.get("buildResult", {})
    msgs = [l["text"] for l in build.get("stderr", []) + build.get("stdout", []) if not NOISE.search(l["text"])]
    stdout = "\n".join(l["text"] for l in r.get("stdout", []))
    stderr = [l["text"] for l in r.get("stderr", [])]
    if not r.get("didExecute"):
        status = "compile"
    elif r.get("code") != 0:
        status = "runtime"
        msgs += stderr
    else:
        status = "ok"
        msgs += stderr
    res = (status, stdout, msgs)
    cache[key] = list(res)
    with open(CACHE, "w") as f:
        json.dump(cache, f)
    return res


def norm(s):
    return re.sub(r"[\s_'\"`]", "", s).lower()


def main():
    only = sys.argv[1:]
    course = json.load(open(COURSE))
    checks = json.load(open(CHECKS))
    fails, runs, unchecked, warnings = [], 0, [], []

    def want(label, src, out=None, err=None, differ=None):
        nonlocal runs
        runs += 1
        status, stdout, msgs = run(src)
        if err:
            ok = status != "ok" and (err is True or status == err)
            why = f"expected a {err} error, got {status}"
        elif differ is not None:
            ok = status != "ok" or stdout != differ
            why = f"expected an error or another output than {differ!r}"
        else:
            ok = status == "ok" and stdout == out
            why = f"expected {out!r}, got {status} {stdout!r}"
        if ok:
            print(f"ok   {label}")
        else:
            fails.append(label)
            print(f"FAIL {label}: {why}")
            print("     " + src.replace("\n", "\n     "))
            for m in msgs[:8]:
                print("     | " + m)
        return status, msgs

    for u in course["units"]:
        if only and u["id"] not in only:
            continue
        for i, b in enumerate(u["lesson"]):
            if b["type"] != "code":
                continue
            label = f"{u['id']}.lesson.{i}"
            if not b["code"].startswith("with Ada.Text_IO; use Ada.Text_IO;"):
                fails.append(label)
                print(f"FAIL {label}: lesson code must be a complete program")
            status, msgs = want(label, b["code"] + "\n", b.get("output", ""))
            warnings += [f"{label}: {m}" for m in msgs if "warning" in m]
        for e in u["exercises"]:
            eid, kind = e["id"], e["type"]
            c = checks.get(eid)
            if c is None:
                unchecked.append(f"{eid} ({kind})")
                continue
            tpl = c.get("tpl")
            out = c.get("out")
            err = c.get("err")
            for j, x in enumerate(c.get("extra", [])):
                want(f"{eid}.extra.{j}", program(x["code"], x.get("tpl")), x.get("out"), x.get("err"))
            if kind == "input" and not any(norm(a) == norm(out) for a in e["answers"]):
                fails.append(eid)
                print(f"FAIL {eid}: out {out!r} is not among the answers {e['answers']}")
            if kind in ("input", "choice", "true_false"):
                want(eid, program(c.get("code") or e["code"], tpl), out, err)
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
                for a, b in itertools.combinations(range(len(lines)), 2):
                    sw = list(lines)
                    sw[a], sw[b] = sw[b], sw[a]
                    want(f"{eid}.swap.{a}.{b}", program("\n".join(sw), tpl), differ=out)
            elif kind == "blocks":
                want(f"{eid}.tokens", program(" ".join(e["tokens"]), tpl), out)
            else:
                unchecked.append(f"{eid} ({kind})")

    print()
    for w in warnings:
        print("warn " + w)
    print(f"{runs} programs run on Compiler Explorer ({COMPILER}, {FLAGS})")
    print("not checked by running code: " + ", ".join(unchecked))
    if fails:
        print(f"{len(fails)} FAILED: " + ", ".join(fails))
        sys.exit(1)
    print("all passing")


if __name__ == "__main__":
    main()
