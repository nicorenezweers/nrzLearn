#!/usr/bin/env python3
"""Runs the C# course code on the Compiler Explorer API (godbolt.org) and
checks lesson outputs and exercise answers. See README.md."""
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
URL = "https://godbolt.org/api/compiler/dotnet90csharpcoreclr/compile"

# What `dotnet new console` (.NET 8) gives you implicitly: global usings and nullable checks.
PREFIX = (
    "#nullable enable\n"
    "using System;\nusing System.Collections.Generic;\nusing System.IO;\nusing System.Linq;\n"
    "using System.Net.Http;\nusing System.Threading;\nusing System.Threading.Tasks;\n"
)

cache = json.load(open(CACHE)) if os.path.exists(CACHE) else {}
stats = {"runs": 0, "api": 0}


def run(src):
    """Compiles and runs `src`. Returns dict(status, out, warnings, detail)."""
    stats["runs"] += 1
    full = PREFIX + src
    key = hashlib.sha256(full.encode()).hexdigest()
    if key not in cache:
        body = {"source": full, "options": {"userArguments": "", "compilerOptions": {"executorRequest": True},
                "filters": {"execute": True}, "tools": [], "libraries": []}, "lang": "csharp", "allowStoreCodeDebug": True}
        for attempt in range(5):
            try:
                req = urllib.request.Request(URL, data=json.dumps(body).encode(),
                                             headers={"Content-Type": "application/json", "Accept": "application/json"})
                d = json.load(urllib.request.urlopen(req, timeout=120))
                break
            except Exception as e:  # noqa: BLE001
                print(f"  retry ({e})", file=sys.stderr)
                time.sleep(5 * (attempt + 1))
        else:
            raise SystemExit("API not reachable")
        stats["api"] += 1
        time.sleep(1)
        b = d.get("buildResult", {})
        build = [x["text"] for x in b.get("stdout", []) + b.get("stderr", [])]
        if b.get("code", 0) != 0 or not d.get("didExecute"):
            status = "compile_error"
        elif d.get("timedOut"):
            status = "timeout"
        elif d.get("code") != 0:
            status = "runtime_error"
        else:
            status = "ok"
        cache[key] = {
            "status": status,
            "out": "\n".join(x["text"] for x in d.get("stdout", [])),
            "warnings": [l for l in build if ": warning " in l],
            "detail": [l for l in build if ": error " in l][:3] + [x["text"] for x in d.get("stderr", [])][:2],
        }
        json.dump(cache, open(CACHE, "w"), indent=1)
    return cache[key]


def norm(s):
    """How the app compares input answers: ignore case, spaces, _, quotes and backticks."""
    return re.sub(r"[\s_'\"`‘’“”]", "", s).lower()


def strip_ticks(s):
    return s[1:-1] if s.startswith("`") and s.endswith("`") and s.count("`") == 2 else s


failures = []


def fail(where, msg, r=None):
    extra = f" | status={r['status']} out={r['out']!r} {r['detail']}" if r else ""
    failures.append(f"{where}: {msg}{extra}")


def expect_ok(where, src, out=None, warnings_ok=False):
    r = run(src)
    if r["status"] != "ok":
        fail(where, "does not run", r)
    elif out is not None and r["out"] != out:
        fail(where, f"output {r['out']!r} != expected {out!r}", r)
    elif r["warnings"] and not warnings_ok:
        fail(where, f"compiler warnings {r['warnings']}")
    return r


def expect_wrong(where, src, out):
    """`src` must fail to compile, crash, or print something other than `out`."""
    r = run(src)
    if r["status"] == "ok" and (out is None or r["out"] == out):
        fail(where, "a wrong variant gives the right result", r)
    return r


def main():
    course = json.load(open(COURSE))
    checks = json.load(open(CHECKS))
    lesson_blocks = 0
    for u in course["units"]:
        for i, b in enumerate(u["lesson"]):
            if b["type"] == "code":
                lesson_blocks += 1
                expect_ok(f"{u['id']}.lesson.{i}", b["code"], b.get("output", ""))
        for e in u["exercises"]:
            eid, t, c = e["id"], e["type"], checks.get(e["id"], {})
            if t == "input":
                r = expect_ok(eid, c.get("src", e.get("code", "")), c.get("out"))
                if r["status"] == "ok" and norm(r["out"]) not in {norm(a) for a in e["answers"]}:
                    fail(eid, f"output {r['out']!r} is not an accepted answer {e['answers']}")
            elif t in ("choice", "true_false", "pairs"):
                if "each" in c:
                    for k, opt in enumerate(e["options"]):
                        src = c["each"].replace("___", strip_ticks(opt))
                        if k == e["answer"]:
                            expect_ok(f"{eid} option {k}", src, c.get("out"), warnings_ok=True)
                        else:
                            expect_wrong(f"{eid} option {k}", src, c.get("out"))
                    continue
                src = c.get("src", e.get("code"))
                if src is None:
                    if t != "pairs":
                        print(f"  note: {eid} has no runnable check (conceptual)")
                    continue
                if c.get("error"):
                    r = run(src)
                    if r["status"] != "compile_error":
                        fail(eid, "expected a compile error", r)
                    continue
                r = expect_ok(eid, src, c.get("out"))
                if c.get("answer_out") and r["status"] == "ok":
                    opts = [strip_ticks(o) for o in e["options"]]
                    if r["out"] != opts[e["answer"]]:
                        fail(eid, f"output {r['out']!r} != answer option {opts[e['answer']]!r}")
                    if sum(o == r["out"] for o in opts) != 1:
                        fail(eid, "output matches more than one option")
            elif t == "gap":
                tpl = c.get("template", e["code"])
                for k, opt in enumerate(e["options"]):
                    src = tpl.replace("___", opt)
                    if k == e["answer"]:
                        expect_ok(f"{eid} option {k}", src, c["out"])
                    elif k not in c.get("skip", []):
                        expect_wrong(f"{eid} option {k}", src, c["out"])
            elif t == "bug":
                lines = e["lines"]
                tpl = c.get("template", "{x}")
                expect_wrong(f"{eid} (as given)", tpl.replace("{x}", "\n".join(lines)), c["out"])
                fixed = lines[:e["bug"]] + [c["fix"]] + lines[e["bug"] + 1:]
                expect_ok(f"{eid} (fixed)", tpl.replace("{x}", "\n".join(fixed)), c["out"])
            elif t == "order":
                lines = e["lines"]
                tpl = c.get("template", "{x}")
                expect_ok(eid, tpl.replace("{x}", "\n".join(lines)), c["out"])
                variants = []
                for j in range(len(lines) - 1):
                    p = list(range(len(lines)))
                    p[j], p[j + 1] = p[j + 1], p[j]
                    variants.append(p)
                tried = set()
                for p in variants:
                    if p == list(range(len(lines))) or tuple(p) in tried:
                        continue
                    tried.add(tuple(p))
                    src = tpl.replace("{x}", "\n".join(lines[j] for j in p))
                    r = run(src)
                    if r["status"] == "ok" and r["out"] == c["out"]:
                        fail(eid, f"order is ambiguous, this order works too: {p}")
            elif t == "blocks":
                line = " ".join(e["tokens"])
                expect_ok(eid, c["template"].replace("{x}", line), c["out"])
            else:
                fail(eid, f"unknown type {t}")
            if t in ("choice", "true_false") and e.get("code") and "src" in c and c["src"] != e["code"]:
                fail(eid, "check src differs from the exercise code")
    print(f"lesson code blocks: {lesson_blocks}, programs run: {stats['runs']} ({stats['api']} new API calls)")
    if failures:
        print(f"{len(failures)} FAILURE(S):")
        for f in failures:
            print("  " + f)
        sys.exit(1)
    print("ALL CHECKS PASSED")


if __name__ == "__main__":
    main()
