#!/usr/bin/env python3
"""Runs the Kotlin course code on the Compiler Explorer API and checks it.

- Every lesson code block: must compile and run; stdout must equal `output`
  exactly (blocks without `output` must print nothing).
- Every exercise with an entry in checks.json (see README.md for the format).

Usage: python3 verify.py [course.json] [--only <prefix>] [--no-cache]
"""

import hashlib
import json
import os
import re
import sys
import time
import urllib.request

HERE = os.path.dirname(os.path.abspath(__file__))
COMPILER = "kotlinc2421"
URL = f"https://godbolt.org/api/compiler/{COMPILER}/compile"
CACHE_FILE = os.path.join(HERE, ".cache.json")

# Compiler Explorer starts the JVM with -Xss136K. That stack is too small to
# load the Kotlin collection classes (StackOverflowError on `listOf`), and the
# sandbox cannot start new threads while the JIT threads run. So the runner
# disables the JIT (-Xint, set via JAVA_TOOL_OPTIONS) and calls the course's
# `main` (renamed) in a thread with a 16 MB stack. The course code itself is
# unchanged.
JVM_OPTIONS = "-Xint -XX:+ReduceSignalUsage -XX:-UsePerfData"
WRAPPER = """

fun main() {
    val t = Thread(null, { courseMain() }, "main", 16L * 1024 * 1024)
    t.start()
    t.join()
}
"""

COMPILE_ERROR = "@compile_error"
RUNTIME_ERROR = "@runtime_error"

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


def post(body):
    global _last_request
    wait = 1.0 - (time.time() - _last_request)
    if wait > 0:
        time.sleep(wait)
    req = urllib.request.Request(
        URL,
        data=json.dumps(body).encode(),
        headers={"Content-Type": "application/json", "Accept": "application/json"},
    )
    for attempt in range(4):
        try:
            with urllib.request.urlopen(req, timeout=180) as r:
                _last_request = time.time()
                stats["requests"] += 1
                return json.load(r)
        except Exception as e:  # network hiccups: retry
            _last_request = time.time()
            if attempt == 3:
                raise
            print(f"    retry after error: {e}", file=sys.stderr)
            time.sleep(5 * (attempt + 1))


def lines(items):
    return "\n".join(x["text"] for x in items or [])


def compile_only(src):
    r = post({
        "source": src,
        "options": {"userArguments": "", "compilerOptions": {}, "filters": {"execute": False},
                    "tools": [], "libraries": []},
        "lang": "kotlin",
    })
    errors = [x["text"] for x in r.get("stderr", []) if "error:" in x["text"]]
    return errors


def execute(src):
    if not re.search(r"\bfun main\(\)", src):
        raise ValueError("program has no `fun main()`:\n" + src)
    wrapped = re.sub(r"\bfun main\(\)", "fun courseMain()", src) + WRAPPER
    r = post({
        "source": wrapped,
        "options": {
            "userArguments": "",
            "executeParameters": {"args": [], "stdin": "", "runtimeTools": [
                {"name": "env", "options": [{"name": "JAVA_TOOL_OPTIONS", "value": JVM_OPTIONS}]}]},
            "compilerOptions": {"executorRequest": True},
            "filters": {"execute": True},
            "tools": [], "libraries": [],
        },
        "lang": "kotlin",
        "allowStoreCodeDebug": True,
    })
    stderr = "\n".join(x["text"] for x in r.get("stderr", []) if not x["text"].startswith("Picked up JAVA_TOOL_OPTIONS"))
    return r, lines(r.get("stdout")), stderr


def run(src):
    """Returns {"status": ok|compile_error|runtime_error, "stdout", "detail"}."""
    stats["programs"] += 1
    key = hashlib.sha256((COMPILER + JVM_OPTIONS + src).encode()).hexdigest()
    if key in _cache:
        stats["cached"] += 1
        return _cache[key]
    r, out, err = execute(src)
    if r.get("didExecute"):
        if "Exception in thread" in err or r.get("code") != 0:
            res = {"status": "runtime_error", "stdout": out, "detail": err[:400]}
        else:
            res = {"status": "ok", "stdout": out, "detail": err[:400]}
    else:
        errors = compile_only(src)
        if errors:
            res = {"status": "compile_error", "stdout": "", "detail": "\n".join(errors)[:600]}
        else:
            raise RuntimeError(f"API did not execute and no compile error was found: {err}")
    _cache[key] = res
    save_cache()
    return res


def outcome(res):
    if res["status"] == "compile_error":
        return COMPILE_ERROR
    if res["status"] == "runtime_error":
        return RUNTIME_ERROR
    return res["stdout"]


def as_program(code):
    if re.search(r"\bfun main\(\)", code):
        return code
    return "fun main() {\n" + "\n".join(("    " + l) if l else l for l in code.split("\n")) + "\n}"


def with_append(code, append):
    """Lines plus extra code; top-level lines stay top-level when the extra code has `main`."""
    if append and re.search(r"\bfun main\(\)", append):
        return code + append
    return as_program(code) + append


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
            self.failures.append(f"{what}\n      {detail}")
            print(f"  FAIL {what}\n      {detail}")

    def expect(self, what, src, expected):
        res = run(src)
        got = outcome(res)
        self.ok(got == expected, what,
                f"expected {expected!r}, got {got!r} {res['detail'][:300]!r}\n--- program ---\n{src}")
        return res

    def differ(self, what, src, expected):
        res = run(src)
        got = outcome(res)
        self.ok(got != expected, what, f"must NOT give {expected!r}, but did\n--- program ---\n{src}")
        return res


def check_exercise(c, ex, spec):
    eid, kind = ex["id"], ex["type"]
    for i, p in enumerate(spec.get("programs", [])):
        c.expect(f"{eid} program {i}", as_program(p["code"]), p["expect"])

    if kind == "input":
        res = run(as_program(ex["code"]))
        got = outcome(res)
        c.ok(any(normalize(a) == normalize(got) for a in ex["answers"]), f"{eid} input answer",
             f"stdout {got!r} not in answers {ex['answers']!r}")
        c.ok(ex["answers"][0] == got, f"{eid} first answer is the exact output", f"{got!r} vs {ex['answers'][0]!r}")
    elif kind == "choice" and "code" in ex:
        expected = spec.get("expect", strip_ticks(ex["options"][ex["answer"]]))
        c.expect(f"{eid} choice code", as_program(ex["code"]), expected)
    elif kind == "true_false" and "code" in ex:
        c.expect(f"{eid} true_false code", as_program(ex["code"]), spec["expect"])
    elif kind == "gap":
        prog = lambda opt: as_program(ex["code"].replace("___", opt))
        c.expect(f"{eid} gap answer", prog(ex["options"][ex["answer"]]), spec["expect"])
        for i, opt in enumerate(ex["options"]):
            if i != ex["answer"]:
                c.differ(f"{eid} gap wrong option {opt!r}", prog(opt), spec["expect"])
    elif kind == "order":
        src = with_append("\n".join(ex["lines"]), spec.get("append", ""))
        c.expect(f"{eid} order", src, spec["expect"])
    elif kind == "blocks":
        line = " ".join(ex["tokens"])
        c.expect(f"{eid} blocks", spec["template"].replace("@@", line), spec["expect"])
    elif kind == "bug":
        src = with_append("\n".join(ex["lines"]), spec.get("append", ""))
        c.differ(f"{eid} bug version is wrong", src, spec["expect"])
        fixed = list(ex["lines"])
        fixed[ex["bug"]] = spec["fix"]
        c.expect(f"{eid} fixed version", with_append("\n".join(fixed), spec.get("append", "")), spec["expect"])


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
                c.expect(f"{u['id']}.lesson.{i}", b["code"], b.get("output", ""))
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
