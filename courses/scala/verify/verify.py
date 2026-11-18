#!/usr/bin/env python3
"""Runs the Scala course code on the Compiler Explorer API and checks it.

- Every lesson code block: must compile without warnings and run; stdout must
  equal `output` exactly (blocks without `output` must print nothing).
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
COMPILER = "scalac371"
SCALA_JAR = "/opt/compiler-explorer/scala3-3.7.1/lib/scala.jar"
URL = f"https://godbolt.org/api/compiler/{COMPILER}/compile"
CACHE_FILE = os.path.join(HERE, ".cache.json")
MAIN = "@main def run(): Unit ="

# Compiler Explorer runs `java -Xss136K <MainClass>` with only the program's own
# classes on the class path, so the Scala library is missing at run time. The
# runner therefore removes `@main` (the course's `run` becomes a plain
# top-level function in `example$package`) and adds a launcher that uses only
# Java classes: it loads the program with a class loader that also sees the
# Scala library, and calls `run` in a thread with a 16 MB stack. The JIT is
# switched off (JAVA_TOOL_OPTIONS) so that the sandbox has room for the thread.
JVM_OPTIONS = "-Xint -XX:+ReduceSignalUsage -XX:-UsePerfData"
LAUNCHER = f"""

object CeLauncher:
  def main(args: Array[String]): Unit =
    val urls = Array(new java.io.File("{SCALA_JAR}").toURI.toURL, new java.io.File("/app/").toURI.toURL)
    val loader = new java.net.URLClassLoader(urls, ClassLoader.getPlatformClassLoader)
    val method = loader.loadClass("example$package").getMethod("run")
    val t = new Thread(null, () => method.invoke(null), "main", 16L * 1024 * 1024)
    t.start()
    t.join()
"""

COMPILE_ERROR = "@compile_error"
RUNTIME_ERROR = "@runtime_error"
ANSI = re.compile(r"\x1b\[[0-9;]*m")

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


def text(items):
    return "\n".join(ANSI.sub("", x["text"]) for x in items or [])


def execute(src):
    if src.count(MAIN) > 1:
        raise ValueError(f"program has more than one `{MAIN}`:\n" + src)
    # Without the exact `@main` line (wrong options such as `@main fun run()`),
    # the program is sent as it is; it then fails to compile or to start.
    wrapped = src.replace(MAIN, "def run(): Unit =") + LAUNCHER
    return post({
        "source": wrapped,
        "options": {
            "userArguments": "",
            "executeParameters": {"args": [], "stdin": "", "runtimeTools": [
                {"name": "env", "options": [{"name": "JAVA_TOOL_OPTIONS", "value": JVM_OPTIONS}]}]},
            "compilerOptions": {"executorRequest": True},
            "filters": {"execute": True},
            "tools": [], "libraries": [],
        },
        "lang": "scala",
        "allowStoreCodeDebug": True,
    })


def run(src):
    """Returns {"status": ok|compile_error|runtime_error, "stdout", "warnings", "detail"}."""
    stats["programs"] += 1
    key = hashlib.sha256((COMPILER + JVM_OPTIONS + LAUNCHER + src).encode()).hexdigest()
    if key in _cache:
        stats["cached"] += 1
        return _cache[key]
    r = execute(src)
    build = text(r.get("buildResult", {}).get("stderr"))
    warnings = [l for l in build.split("\n") if "Warning" in l]
    if r.get("didExecute"):
        out = text(r.get("stdout"))
        err = "\n".join(l for l in text(r.get("stderr")).split("\n") if not l.startswith("Picked up JAVA_TOOL_OPTIONS"))
        failed = "Exception in thread" in err or r.get("code") != 0 or r.get("timedOut")
        res = {"status": "runtime_error" if failed else "ok", "stdout": out,
               "warnings": warnings, "detail": err[:400]}
    else:
        err = text(r.get("stderr")) or build
        if "Error" in err or "error" in err:
            res = {"status": "compile_error", "stdout": "", "warnings": warnings, "detail": err[:600]}
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
    if "@main" in code:
        return code
    return MAIN + "\n" + "\n".join(("  " + l) if l else l for l in code.split("\n"))


def assemble(code, spec):
    """Lines of an order/bug exercise plus `prepend`/`append` from checks.json."""
    append = spec.get("append", "")
    if append and "@main" in append:
        return spec.get("prepend", "") + code + append
    return spec.get("prepend", "") + as_program(code) + append


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

    def expect(self, what, src, expected, strict=True):
        """strict: course code must also compile without warnings."""
        res = run(src)
        got = outcome(res)
        self.ok(got == expected, what,
                f"expected {expected!r}, got {got!r} {res['detail'][:300]!r}\n--- program ---\n{src}")
        if strict and got == expected and expected != COMPILE_ERROR:
            self.ok(not res["warnings"], f"{what}: no compiler warnings", "\n".join(res["warnings"]))
        return res

    def differ(self, what, src, expected):
        res = run(src)
        got = outcome(res)
        self.ok(got != expected, what, f"must NOT give {expected!r}, but did\n--- program ---\n{src}")
        return res


def check_exercise(c, ex, spec):
    eid, kind = ex["id"], ex["type"]
    for i, p in enumerate(spec.get("programs", [])):
        c.expect(f"{eid} program {i}", as_program(p["code"]), p["expect"], strict=False)

    if kind == "input":
        res = run(as_program(ex["code"]))
        got = outcome(res)
        c.ok(any(normalize(a) == normalize(got) for a in ex["answers"]), f"{eid} input answer",
             f"stdout {got!r} not in answers {ex['answers']!r}")
        c.ok(ex["answers"][0] == got, f"{eid} first answer is the exact output", f"{got!r} vs {ex['answers'][0]!r}")
        c.ok(not res["warnings"], f"{eid}: no compiler warnings", "\n".join(res["warnings"]))
    elif kind == "choice" and "code" in ex:
        expected = spec.get("expect", strip_ticks(ex["options"][ex["answer"]]))
        c.expect(f"{eid} choice code", as_program(ex["code"]), expected)
        for i, opt in enumerate(ex["options"]):
            if i != ex["answer"]:
                c.ok(strip_ticks(opt) != expected, f"{eid} wrong option {opt!r} differs from the output")
    elif kind == "true_false" and "code" in ex:
        c.expect(f"{eid} true_false code", as_program(ex["code"]), spec["expect"])
    elif kind == "gap":
        prog = lambda opt: as_program(ex["code"].replace("___", opt))
        c.expect(f"{eid} gap answer", prog(ex["options"][ex["answer"]]), spec["expect"])
        for i, opt in enumerate(ex["options"]):
            if i != ex["answer"]:
                c.differ(f"{eid} gap wrong option {opt!r}", prog(opt), spec["expect"])
    elif kind == "order":
        c.expect(f"{eid} order", assemble("\n".join(ex["lines"]), spec), spec["expect"])
    elif kind == "blocks":
        line = " ".join(ex["tokens"])
        c.expect(f"{eid} blocks", spec["template"].replace("@@", line), spec["expect"])
    elif kind == "bug":
        c.differ(f"{eid} bug version is wrong", assemble("\n".join(ex["lines"]), spec), spec["expect"])
        fixed = list(ex["lines"])
        fixed[ex["bug"]] = spec["fix"]
        c.expect(f"{eid} fixed version", assemble("\n".join(fixed), spec), spec["expect"])


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
        print(f"== {u['id']}", flush=True)
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
