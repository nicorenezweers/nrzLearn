#!/usr/bin/env python3
"""Runs the Java course code on the Compiler Explorer API and checks it.

Usage: python3 verify.py [course.json] [--only <unit prefix>] [--no-cache] [--compiler <id>]
See README.md for what is checked and the format of checks.json.
"""

import hashlib
import json
import os
import re
import sys
import time
import urllib.request

HERE = os.path.dirname(os.path.abspath(__file__))
COMPILER = "java2102"
CACHE_FILE = os.path.join(HERE, ".cache.json")

COMPILE_ERROR = "@compile_error"
RUNTIME_ERROR = "@runtime_error"
MAIN_LINE = "public static void main(String[] args)"

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


def post(src):
    global _last_request
    wait = 1.0 - (time.time() - _last_request)
    if wait > 0:
        time.sleep(wait)
    body = {
        "source": src,
        "options": {"userArguments": "", "compilerOptions": {"executorRequest": True},
                    "filters": {"execute": True}, "tools": [], "libraries": []},
        "lang": "java",
        "allowStoreCodeDebug": True,
    }
    req = urllib.request.Request(
        f"https://godbolt.org/api/compiler/{COMPILER}/compile",
        data=json.dumps(body).encode(),
        headers={"Content-Type": "application/json", "Accept": "application/json"},
    )
    for attempt in range(4):
        try:
            with urllib.request.urlopen(req, timeout=180) as r:
                _last_request = time.time()
                stats["requests"] += 1
                return json.load(r)
        except Exception as e:
            _last_request = time.time()
            if attempt == 3:
                raise
            print(f"    retry after error: {e}", file=sys.stderr)
            time.sleep(5 * (attempt + 1))


def text(items):
    return "\n".join(x["text"] for x in items or [])


def run(src, raw=False):
    """Returns {"status": ok|compile_error|runtime_error, "stdout", "detail"}.

    Compiler Explorer saves the source as example.java, so a `public class Main`
    does not compile there; unless `raw` is set it becomes `class Main`."""
    if not raw:
        src = re.sub(r"^public class Main\b", "class Main", src, flags=re.M)
    stats["programs"] += 1
    key = hashlib.sha256((COMPILER + "\0" + src).encode()).hexdigest()
    if key in _cache:
        stats["cached"] += 1
        return _cache[key]
    r = post(src)
    out, err = text(r.get("stdout")), text(r.get("stderr"))
    if r.get("didExecute"):
        status = "ok" if r.get("code") == 0 and "Exception in thread" not in err else "runtime_error"
        res = {"status": status, "stdout": out, "detail": err[:400]}
    else:
        build = r.get("buildResult", {})
        if build.get("code", 0) != 0:
            res = {"status": "compile_error", "stdout": "", "detail": text(build.get("stderr"))[:600]}
        else:
            raise RuntimeError(f"API did not execute and reported no compile error: {json.dumps(r)[:800]}")
    _cache[key] = res
    save_cache()
    return res


def outcome(res):
    return {"compile_error": COMPILE_ERROR, "runtime_error": RUNTIME_ERROR}.get(res["status"], res["stdout"])


def indent(code, n):
    return "\n".join((" " * n + l) if l else l for l in code.split("\n"))


def as_program(code, tpl=None):
    """Full programs stay as they are; snippets get the missing parts around them."""
    if tpl:
        m = re.search(r"^( *)\{\{CODE\}\}", tpl, flags=re.M)
        return tpl[:m.start()] + indent(code, len(m.group(1))) + tpl[m.end():]
    if re.search(r"\bclass Main\b", code):
        return code
    imports = "import java.util.*;\n\n" if re.search(r"\b(List|ArrayList|Arrays)\b", code) else ""
    if MAIN_LINE in code:
        return imports + "public class Main {\n" + indent(code, 4) + "\n}"
    return imports + "public class Main {\n    " + MAIN_LINE + " {\n" + indent(code, 8) + "\n    }\n}"


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

    def expect(self, what, src, expected, raw=False):
        res = run(src, raw)
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
    tpl = spec.get("tpl")
    prog = lambda code: as_program(code, tpl)
    for i, p in enumerate(spec.get("programs", [])):
        c.expect(f"{eid} program {i}", as_program(p["code"], p.get("tpl")), p["expect"], p.get("raw", False))

    if kind == "input":
        got = outcome(run(prog(ex["code"])))
        c.ok(any(normalize(a) == normalize(got) for a in ex["answers"]), f"{eid} input answer",
             f"stdout {got!r} not in answers {ex['answers']!r}")
        c.ok(ex["answers"][0] == got, f"{eid} first answer is the exact output", f"{got!r} vs {ex['answers'][0]!r}")
    elif kind == "choice" and "code" in ex:
        expected = spec.get("expect", strip_ticks(ex["options"][ex["answer"]]))
        c.expect(f"{eid} choice code", prog(ex["code"]), expected)
    elif kind == "true_false" and "code" in ex:
        c.expect(f"{eid} true_false code", prog(ex["code"]), spec["expect"])
    elif kind == "gap":
        fill = lambda opt: prog(ex["code"].replace("___", opt))
        c.expect(f"{eid} gap answer", fill(ex["options"][ex["answer"]]), spec["expect"])
        for i, opt in enumerate(ex["options"]):
            if i != ex["answer"]:
                c.differ(f"{eid} gap wrong option {opt!r}", fill(opt), spec["expect"])
    elif kind == "order":
        c.expect(f"{eid} order", prog("\n".join(ex["lines"])), spec["expect"])
    elif kind == "blocks":
        c.expect(f"{eid} blocks", prog(" ".join(ex["tokens"])), spec["expect"])
    elif kind == "bug":
        c.differ(f"{eid} bug version is wrong", prog("\n".join(ex["lines"])), spec["expect"])
        fixed = list(ex["lines"])
        fixed[ex["bug"]] = spec["fix"]
        c.expect(f"{eid} fixed version", prog("\n".join(fixed)), spec["expect"])


NEEDS_SPEC = {"gap", "order", "blocks", "bug"}


def main():
    global COMPILER
    args = sys.argv[1:]
    opts = {}
    for flag in ("--only", "--compiler"):
        if flag in args:
            i = args.index(flag)
            opts[flag] = args[i + 1]
            del args[i:i + 2]
    COMPILER = opts.get("--compiler", COMPILER)
    only = opts.get("--only")
    use_cache = "--no-cache" not in args
    args = [a for a in args if a != "--no-cache"]
    course_path = args[0] if args else os.path.join(HERE, "..", "course.json")
    with open(course_path) as f:
        course = json.load(f)
    with open(os.path.join(HERE, "checks.json")) as f:
        checks = json.load(f)
    load_cache(use_cache)
    print(f"compiler: {COMPILER}")
    c = Checker()
    seen = set()
    for u in course["units"]:
        if only and not u["id"].startswith(only):
            continue
        print(f"== {u['id']}")
        for i, b in enumerate(u["lesson"]):
            if b["type"] != "code":
                continue
            what = f"{u['id']}.lesson.{i}"
            c.ok("public class Main {" in b["code"] and MAIN_LINE in b["code"], f"{what} is a complete program")
            c.ok("//" not in b["code"] and "/*" not in b["code"], f"{what} has no comments")
            c.expect(what, b["code"], b.get("output", ""))
        for ex in u["exercises"]:
            seen.add(ex["id"])
            for field in ("code", "lines"):
                code = ex.get(field)
                code = "\n".join(code) if isinstance(code, list) else code
                if code:
                    c.ok("//" not in code and "/*" not in code, f"{ex['id']} has no comments")
            spec = checks.get(ex["id"])
            if spec is None:
                needs = ex["type"] in NEEDS_SPEC or (ex["type"] == "true_false" and "code" in ex)
                c.ok(not needs, f"{ex['id']}: no entry in checks.json")
                spec = {}
            check_exercise(c, ex, spec)
    if not only:
        for k in checks:
            c.ok(k in seen, f"checks.json: {k} has no exercise")
    print(f"\n{c.passed} checks passed, {len(c.failures)} failed; "
          f"{stats['programs']} programs ({stats['requests']} API requests, {stats['cached']} from cache)")
    sys.exit(1 if c.failures else 0)


if __name__ == "__main__":
    main()
