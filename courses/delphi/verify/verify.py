#!/usr/bin/env python3
"""Checks the Delphi course with Free Pascal in Delphi mode on Compiler Explorer.

Usage: python3 courses/delphi/verify/verify.py [-v]
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
CACHE = os.path.join(HERE, ".cache")
COMPILER = os.environ.get("GODBOLT_COMPILER", "fpc322")
ARGS = "-Mdelphi -Cr -Co -Sa"
HEADER = "program Check;\n\n{$APPTYPE CONSOLE}\n\n"
VERBOSE = "-v" in sys.argv
_last = [0.0]


def compile_run(src):
    """Returns (ok, stdout, info). ok: compiled without warnings, ran, exit code 0."""
    key = hashlib.sha256(f"{COMPILER}\0{ARGS}\0{src}".encode()).hexdigest()
    path = os.path.join(CACHE, key + ".json")
    if os.path.exists(path):
        with open(path) as f:
            d = json.load(f)
    else:
        body = {"source": src, "lang": "pascal", "allowStoreCodeDebug": True,
                "options": {"userArguments": ARGS, "compilerOptions": {"executorRequest": True},
                            "filters": {"execute": True}, "tools": [], "libraries": []}}
        for attempt in range(5):
            wait = 1.0 - (time.time() - _last[0])
            if wait > 0:
                time.sleep(wait)
            _last[0] = time.time()
            try:
                req = urllib.request.Request(
                    f"https://godbolt.org/api/compiler/{COMPILER}/compile", data=json.dumps(body).encode(),
                    headers={"Content-Type": "application/json", "Accept": "application/json"})
                d = json.load(urllib.request.urlopen(req, timeout=60))
                break
            except Exception as e:  # network hiccup: retry
                if attempt == 4:
                    raise
                time.sleep(3 + attempt * 3)
        os.makedirs(CACHE, exist_ok=True)
        with open(path, "w") as f:
            json.dump(d, f)
    build = [x["text"] for x in d.get("buildResult", {}).get("stdout", [])]
    build += [x["text"] for x in d.get("buildResult", {}).get("stderr", [])]
    problems = [l for l in build if re.search(r"\b(Warning|Error|Fatal):", l)]
    out = "\n".join(x["text"] for x in d.get("stdout", []))
    if not d.get("didExecute"):
        return False, out, "compile: " + " | ".join(problems[:3])
    if problems:
        return False, out, "warning: " + " | ".join(problems[:3])
    if d.get("code") != 0:
        err = " | ".join(x["text"] for x in d.get("stderr", []))[:200]
        return False, out, f"exit {d.get('code')} {err}"
    return True, out, ""


def wrap(code, decl=""):
    code = code.strip("\n")
    if code.startswith("program "):
        return code
    if code.rstrip().endswith("end."):
        return HEADER + code
    body = "\n".join("  " + l if l else l for l in code.split("\n"))
    return HEADER + (decl + "\n" if decl else "") + "begin\n" + body + "\nend."


def norm(s):
    return re.sub(r"[\s_'\"`]", "", s).lower()


def strip_ticks(s):
    return s[1:-1] if s.startswith("`") and s.endswith("`") else s


passed, failed, manual = 0, 0, []
fails = []


def ok(label):
    global passed
    passed += 1
    if VERBOSE:
        print("  ok  ", label)


def bad(label, msg, src=None):
    global failed
    failed += 1
    fails.append(label)
    print("  FAIL", label, "-", msg)
    if src and VERBOSE:
        print("       " + src.replace("\n", "\n       "))


def good(label, src, expect):
    r, out, info = compile_run(src)
    if not r:
        return bad(label, info, src)
    if out != expect:
        return bad(label, f"output {out!r}, expected {expect!r}", src)
    ok(label)


def fails_or_differs(src, expect):
    r, out, _ = compile_run(src)
    return (not r) or (expect is not None and out != expect)


def fill(template, **kw):
    s = template
    for k, v in kw.items():
        s = s.replace(f"@{k}@", v)
    return s


def build(ex, step, **kw):
    if "src" in step:
        return wrap(fill(step["src"], **kw), step.get("decl", ""))
    return None


def check_exercise(ex, steps):
    eid, t = ex["id"], ex["type"]
    for n, st in enumerate(steps):
        label = f"{eid}#{n}"
        kind = st.get("kind", "run")
        code = ex.get("code", "")
        if kind == "manual":
            manual.append(f"{eid}: {st['note']}")
            continue
        if kind == "fail":
            src = wrap(fill(st["src"], CODE=code), st.get("decl", ""))
            if fails_or_differs(src, None):
                ok(label + " fails as expected")
            else:
                bad(label, "program should fail but passed", src)
            continue
        if kind == "options":
            for i, opt in enumerate(ex["options"]):
                src = wrap(fill(st["src"], OPTION=strip_ticks(opt), CODE=code), st.get("decl", ""))
                if i == ex["answer"]:
                    good(f"{label} right option", src, st["expect"])
                elif fails_or_differs(src, st["expect"]):
                    ok(f"{label} wrong option {i} fails")
                else:
                    bad(label, f"wrong option {opt} passes", src)
            continue
        if t == "input":
            src = build(ex, st, CODE=code) or wrap(code, st.get("decl", ""))
            r, out, info = compile_run(src)
            if not r:
                bad(label, info, src)
            elif norm(out) not in [norm(a) for a in ex["answers"]]:
                bad(label, f"output {out!r} not in answers {ex['answers']}", src)
            else:
                ok(label)
        elif t == "choice":
            src = build(ex, st, CODE=code) or wrap(code, st.get("decl", ""))
            good(label, src, st.get("expect", strip_ticks(ex["options"][ex["answer"]])))
        elif t == "true_false":
            good(label, build(ex, st, CODE=code) or wrap(code, st.get("decl", "")), st["expect"])
        elif t == "gap":
            pass  # handled below (all steps together)
        elif t == "blocks":
            ans = " ".join(ex["tokens"])
            good(label, build(ex, st, ANSWER=ans) or wrap(ans, st.get("decl", "")), st["expect"])
        elif t == "order":
            ans = "\n".join(ex["lines"])
            mk = lambda a: build(ex, st, ANSWER=a) or wrap(a, st.get("decl", ""))
            good(label, mk(ans), st["expect"])
            if st.get("swaps", True):
                for i in range(len(ex["lines"]) - 1):
                    ls = list(ex["lines"])
                    ls[i], ls[i + 1] = ls[i + 1], ls[i]
                    if fails_or_differs(mk("\n".join(ls)), st["expect"]):
                        ok(f"{label} swap {i}/{i + 1} fails")
                    else:
                        bad(label, f"swapping lines {i} and {i + 1} also works")
        elif t == "bug":
            mk = lambda ls: build(ex, st, LINES="\n".join(ls)) or wrap("\n".join(ls), st.get("decl", ""))
            if fails_or_differs(mk(ex["lines"]), st["expect"]):
                ok(f"{label} original fails")
            else:
                bad(label, "original lines pass", mk(ex["lines"]))
            ls = list(ex["lines"])
            if st.get("delete"):
                del ls[ex["bug"]]
            else:
                ls[ex["bug"]] = st["fix"]
            good(f"{label} fixed", mk(ls), st["expect"])
    if t == "gap":
        run_steps = [s for s in steps if s.get("kind", "run") == "run"]
        for i, opt in enumerate(ex["options"]):
            filled = ex["code"].replace("___", opt)
            results = []
            for n, st in enumerate(run_steps):
                src = build(ex, st, CODE=filled) or wrap(filled, st.get("decl", ""))
                if i == ex["answer"]:
                    good(f"{eid}#{n} right option", src, st["expect"])
                else:
                    results.append(fails_or_differs(src, st["expect"]))
            if i != ex["answer"]:
                if any(results):
                    ok(f"{eid} wrong option {opt!r} fails")
                else:
                    bad(eid, f"wrong option {opt!r} passes every step")


def main():
    with open(COURSE) as f:
        course = json.load(f)
    with open(CHECKS) as f:
        checks = json.load(f)
    blocks = 0
    for u in course["units"]:
        print(u["id"])
        for i, b in enumerate(u["lesson"]):
            if b["type"] == "code" and "output" in b:
                blocks += 1
                if not b["code"].startswith("program "):
                    bad(f"{u['id']}.lesson.{i}", "lesson code with output must be a complete program")
                    continue
                good(f"{u['id']}.lesson.{i}", b["code"], b["output"])
        for ex in u["exercises"]:
            if ex["type"] == "pairs":
                continue
            if ex["id"] not in checks:
                bad(ex["id"], "no check in checks.json")
                continue
            check_exercise(ex, checks[ex["id"]])
    print(f"\n{blocks} lesson programs, {passed} checks passed, {failed} failed, {len(manual)} manual")
    for m in manual:
        print("  manual:", m)
    if fails:
        print("failed:", ", ".join(fails))
    sys.exit(1 if failed else 0)


if __name__ == "__main__":
    main()
