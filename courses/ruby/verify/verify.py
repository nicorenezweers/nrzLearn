#!/usr/bin/env python3
"""Runs the Ruby course code on the Compiler Explorer API and checks it.

Every program runs on two Ruby versions (the oldest and the newest supported
one). Both must give the same result, and that result must be the expected one.

- Every lesson code block: must run without error and without anything on
  stderr; stdout must equal `output` exactly (blocks without `output` must
  print nothing).
- Every exercise with an entry in checks.json (see README.md for the format).

Usage: python3 verify.py [course.json] [--only <unit prefix>] [--no-cache]
"""

import hashlib
import json
import os
import re
import sys
import time
import urllib.request

HERE = os.path.dirname(os.path.abspath(__file__))
COMPILERS = ["ruby334", "ruby405"]
URL = "https://godbolt.org/api/compiler/{}/compile"
CACHE_FILE = os.path.join(HERE, ".cache.json")

_cache = {}
_last_request = 0.0
stats = {"requests": 0, "cached": 0, "programs": 0}


def load_cache(use):
    global _cache
    if use and os.path.exists(CACHE_FILE):
        with open(CACHE_FILE) as f:
            _cache = json.load(f)


def save_cache():
    with open(CACHE_FILE, "w") as f:
        json.dump(_cache, f, indent=1, sort_keys=True)


def post(compiler, src):
    global _last_request
    wait = 1.0 - (time.time() - _last_request)
    if wait > 0:
        time.sleep(wait)
    body = {
        "source": src,
        "options": {"userArguments": "", "compilerOptions": {"executorRequest": True},
                    "filters": {"execute": True}, "tools": [], "libraries": []},
        "lang": "ruby",
        "allowStoreCodeDebug": True,
    }
    req = urllib.request.Request(
        URL.format(compiler),
        data=json.dumps(body).encode(),
        headers={"Content-Type": "application/json", "Accept": "application/json"},
    )
    for attempt in range(4):
        try:
            with urllib.request.urlopen(req, timeout=120) as r:
                _last_request = time.time()
                stats["requests"] += 1
                return json.load(r)
        except Exception as e:  # network hiccups: retry
            _last_request = time.time()
            if attempt == 3:
                raise
            print(f"    retry after error: {e}", file=sys.stderr)
            time.sleep(5 * (attempt + 1))


def text(items):
    return "\n".join(x["text"] for x in items or [])


def run_one(compiler, src):
    """Returns {"outcome", "stderr"}; outcome is stdout, `@syntax_error` or `@error:<Class>`."""
    key = hashlib.sha256((compiler + "\0" + src).encode()).hexdigest()
    if key in _cache:
        stats["cached"] += 1
        return _cache[key]
    r = post(compiler, src)
    out, err = text(r.get("stdout")), text(r.get("stderr"))
    if not r.get("didExecute"):
        build = text(r.get("buildResult", {}).get("stderr")) or err
        res = {"outcome": "@syntax_error", "stderr": build[:600]}
    elif r.get("code") == 0:
        res = {"outcome": out, "stderr": err[:600]}
    elif "syntax error" in err or "SyntaxError" in err or "Unmatched" in err or "--> " in err:
        res = {"outcome": "@syntax_error", "stderr": err[:600]}
    else:
        m = re.search(r"\(([A-Z]\w*(?:::\w+)*)\)\s*$", err.split("\n")[0])
        res = {"outcome": "@error:" + (m.group(1) if m else "?"), "stderr": err[:600]}
    _cache[key] = res
    save_cache()
    return res


def run(src):
    """Runs on every version. Returns (outcome, stderr, problem); problem is set when versions differ."""
    stats["programs"] += 1
    results = [run_one(c, src) for c in COMPILERS]
    outcomes = {r["outcome"] for r in results}
    problem = None
    if len(outcomes) > 1:
        problem = "versions differ: " + "; ".join(f"{c}={r['outcome']!r}" for c, r in zip(COMPILERS, results))
    return results[0]["outcome"], " | ".join(r["stderr"] for r in results if r["stderr"]), problem


def matches(got, expected):
    """`@error` matches any runtime error, `@error:NameError` only that class."""
    if expected.startswith("@"):
        return got == expected or (expected == "@error" and got.startswith("@error:"))
    return got == expected


def normalize(s):
    return re.sub(r"[\s_\"'`]", "", s).lower()


def strip_ticks(s):
    s = s.strip()
    return s[1:-1] if s.startswith("`") and s.endswith("`") and s.count("`") == 2 else s


class Checker:
    def __init__(self):
        self.failures = []
        self.passed = 0

    def ok(self, cond, what, detail=""):
        if cond:
            self.passed += 1
        else:
            self.failures.append(what)
            print(f"  FAIL {what}\n      {detail}")

    def expect(self, what, src, expected, clean=False):
        got, err, problem = run(src)
        self.ok(problem is None, f"{what}: same result on {', '.join(COMPILERS)}", problem)
        good = matches(got, expected) and (not clean or not err)
        self.ok(good, what, f"expected {expected!r}, got {got!r}\n      stderr {err[:300]!r}\n--- program ---\n{src}")
        return got

    def differ(self, what, src, expected):
        got, err, problem = run(src)
        self.ok(problem is None, f"{what}: same result on {', '.join(COMPILERS)}", problem)
        self.ok(not matches(got, expected), what, f"must NOT give {expected!r}, but did\n--- program ---\n{src}")
        return got


def check_exercise(c, ex, spec):
    eid, kind = ex["id"], ex["type"]
    for i, p in enumerate(spec.get("programs", [])):
        c.expect(f"{eid} program {i}", p["code"], p["expect"])

    if kind == "input":
        got, _, problem = run(ex["code"])
        c.ok(problem is None, f"{eid}: same result on all versions", problem)
        c.ok(any(normalize(a) == normalize(got) for a in ex["answers"]), f"{eid} input answer",
             f"stdout {got!r} not in answers {ex['answers']!r}")
        c.ok(ex["answers"][0] == got, f"{eid} first answer is the exact output", f"{got!r} vs {ex['answers'][0]!r}")
    elif kind == "choice" and "code" in ex:
        expected = spec.get("expect", strip_ticks(ex["options"][ex["answer"]]))
        c.expect(f"{eid} choice code", ex["code"], expected)
    elif kind == "true_false" and "code" in ex:
        c.expect(f"{eid} true_false code", ex["code"], spec["expect"])
    elif kind == "gap":
        prog = lambda opt: spec.get("template", "@@").replace("@@", ex["code"].replace("___", opt))
        c.expect(f"{eid} gap answer", prog(ex["options"][ex["answer"]]), spec["expect"], clean=True)
        for i, opt in enumerate(ex["options"]):
            if i != ex["answer"]:
                c.differ(f"{eid} gap wrong option {opt!r}", prog(opt), spec["expect"])
    elif kind == "order":
        src = spec.get("template", "@@").replace("@@", "\n".join(ex["lines"]))
        c.expect(f"{eid} order", src, spec["expect"], clean=True)
    elif kind == "blocks":
        line = " ".join(ex["tokens"])
        c.expect(f"{eid} blocks", spec.get("template", "@@").replace("@@", line), spec["expect"], clean=True)
    elif kind == "bug":
        tpl = spec.get("template", "@@")
        c.differ(f"{eid} bug version is wrong", tpl.replace("@@", "\n".join(ex["lines"])), spec["expect"])
        fixed = list(ex["lines"])
        fixed[ex["bug"]] = spec["fix"]
        c.expect(f"{eid} fixed version", tpl.replace("@@", "\n".join(fixed)), spec["expect"], clean=True)


def main():
    args = sys.argv[1:]
    only = None
    if "--only" in args:
        only = args[args.index("--only") + 1]
        del args[args.index("--only"):args.index("--only") + 2]
    use_cache = "--no-cache" not in args
    args = [a for a in args if a != "--no-cache"]
    course_path = args[0] if args else os.path.join(HERE, "..", "course.json")
    with open(course_path) as f:
        course = json.load(f)
    with open(os.path.join(HERE, "checks.json")) as f:
        checks = json.load(f)
    load_cache(use_cache)
    c = Checker()
    seen = set()
    for u in course["units"]:
        if only and not u["id"].startswith(only):
            continue
        print(f"== {u['id']}")
        for i, b in enumerate(u["lesson"]):
            if b["type"] == "code":
                c.expect(f"{u['id']}.lesson.{i}", b["code"], b.get("output", ""), clean=True)
        for ex in u["exercises"]:
            spec = checks.get(ex["id"])
            seen.add(ex["id"])
            if spec is None:
                c.ok(ex["type"] in ("pairs", "true_false", "choice") and "code" not in ex,
                     f"{ex['id']}: no entry in checks.json")
                continue
            check_exercise(c, ex, spec)
    if not only:
        for k in checks:
            c.ok(k in seen, f"checks.json: {k} has no exercise")
    print(f"\n{c.passed} checks passed, {len(c.failures)} failed; "
          f"{stats['programs']} programs ({stats['requests']} API requests, {stats['cached']} from cache)")
    sys.exit(1 if c.failures else 0)


if __name__ == "__main__":
    main()
