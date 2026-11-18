#!/usr/bin/env python3
"""Run every lesson code block and every checkable exercise of the Prolog course with swipl."""
import hashlib, itertools, json, os, re, subprocess, sys, tempfile
from concurrent.futures import ThreadPoolExecutor

HERE = os.path.dirname(os.path.abspath(__file__))
COURSE = os.path.join(HERE, "..", "course.json")
CHECKS = os.path.join(HERE, "checks.json")
CACHE = os.path.join(HERE, ".cache.json")
SWIPL = os.environ.get("SWIPL", "swipl")
TIMEOUT = 5
DIRECTIVE = ":- initialization(main, main).\n\n"

VERSION = subprocess.run([SWIPL, "--version"], capture_output=True, text=True).stdout.strip()
cache = json.load(open(CACHE)) if os.path.exists(CACHE) else {}


def run(prog, halt=False):
    key = hashlib.sha256((VERSION + str(halt) + prog).encode()).hexdigest()
    if key in cache:
        return cache[key]
    with tempfile.TemporaryDirectory() as tmp:
        path = os.path.join(tmp, "t.pl")
        with open(path, "w") as f:
            f.write(prog + "\n")
        cmd = [SWIPL] + (["-g", "halt"] if halt else []) + [path]
        try:
            p = subprocess.run(cmd, stdin=subprocess.DEVNULL, capture_output=True, text=True, timeout=TIMEOUT, cwd=tmp)
            res = {"out": p.stdout, "err": p.stderr, "rc": p.returncode, "hang": False}
        except subprocess.TimeoutExpired as e:
            res = {"out": "", "err": "", "rc": -1, "hang": True}
    res["err"] = res["err"].replace(tmp, "TMP")
    cache[key] = res
    return res


def strip1(s):
    return s[:-1] if s.endswith("\n") else s


def ok(res, out):
    return not res["hang"] and res["rc"] == 0 and strip1(res["out"]) == out


def failed(res):
    return res["hang"] or res["rc"] != 0 or "error" in res["err"].lower()


def norm(s):
    return re.sub(r"[\s_'\"`]", "", s).lower()


def build(code, chk):
    prog = chk["tpl"].replace("{{CODE}}", code) if "tpl" in chk else code
    if not chk.get("raw") and "initialization(" not in prog:
        prog = DIRECTIVE + prog
    return prog


problems = []
count = {"lesson": 0, "exercise": 0, "runs": 0}


def bad(where, msg, res=None):
    detail = ""
    if res is not None:
        detail = f"\n    stdout: {res['out']!r}\n    stderr: {res['err'][:400]!r}\n    rc={res['rc']} hang={res['hang']}"
    problems.append(f"{where}: {msg}{detail}")


def lint(where, code):
    for line in code.split("\n"):
        if "\t" in line or line != line.rstrip():
            bad(where, "tab or trailing space in code")
        if "%" in line:
            bad(where, "comment in code")


def check_extra(eid, extras):
    for i, x in enumerate(extras):
        res = run(build(x["src"], {}))
        w = f"{eid} extra {i}"
        if "out" in x and not ok(res, x["out"]):
            bad(w, f"expected {x['out']!r}", res)
        if "not" in x and ok(res, x["not"]):
            bad(w, f"must not print {x['not']!r}", res)
        if x.get("err") and not failed(res):
            bad(w, "expected an error", res)
        if x.get("hang") and not res["hang"]:
            bad(w, "expected no termination", res)
        if "stderr" in x and x["stderr"] not in res["err"]:
            bad(w, f"expected {x['stderr']!r} in stderr", res)


def check_exercise(unit, e, chk):
    eid, kind = e["id"], e["type"]
    if "code" in e:
        lint(eid, e["code"])
    for k in ("lines", "tokens"):
        for line in e.get(k, []):
            if "%" in line:
                bad(eid, "comment in code")
    if chk is None:
        return
    count["exercise"] += 1
    if "extra" in chk:
        check_extra(eid, chk["extra"])
    out = chk.get("out")
    if kind in ("input", "choice", "true_false") and "code" in e and ("out" in chk or "hang" in chk or "err" in chk):
        res = run(build(e["code"], chk))
        if out is not None and not ok(res, out):
            bad(eid, f"expected {out!r}", res)
        if chk.get("hang") and not res["hang"]:
            bad(eid, "expected no termination", res)
        if chk.get("err") and not failed(res):
            bad(eid, "expected an error", res)
        if kind == "input" and out is not None and norm(out) not in [norm(a) for a in e["answers"]]:
            bad(eid, f"output {out!r} is not among the answers")
        if kind == "choice" and chk.get("opt"):
            opts = [o.strip("`") for o in e["options"]]
            if opts[e["answer"]] != out:
                bad(eid, f"right option differs from output {out!r}")
            if opts.count(out) != 1:
                bad(eid, "output matches more than one option")
    elif kind == "gap":
        for i, opt in enumerate(e["options"]):
            res = run(build(e["code"].replace("___", opt), chk))
            if i == e["answer"] and not ok(res, out):
                bad(eid, f"right option {opt!r} does not print {out!r}", res)
            if i != e["answer"] and ok(res, out):
                bad(eid, f"wrong option {opt!r} also prints {out!r}")
    elif kind == "bug":
        lines = e["lines"]
        res = run(build("\n".join(lines), chk))
        if ok(res, out):
            bad(eid, "the code with the bug already prints the expected output")
        fixed = list(lines)
        fixed[e["bug"]] = chk["fix"]
        res = run(build("\n".join(fixed), chk))
        if not ok(res, out):
            bad(eid, f"the fixed code does not print {out!r}", res)
    elif kind == "order":
        lines = e["lines"]
        res = run(build("\n".join(lines), chk))
        if not ok(res, out):
            bad(eid, f"the right order does not print {out!r}", res)
        perms = [p for p in itertools.permutations(lines) if list(p) != lines]
        progs = [build("\n".join(p), chk) for p in perms]
        with ThreadPoolExecutor(8) as pool:
            results = list(pool.map(run, progs))
        count["runs"] += len(progs)
        for p, r in zip(perms, results):
            if ok(r, out):
                bad(eid, "another order also works:\n      " + "\n      ".join(p))
                break
    elif kind == "blocks":
        res = run(build(" ".join(e["tokens"]), chk))
        if not ok(res, out):
            bad(eid, f"the tokens do not print {out!r}", res)


def main():
    course = json.load(open(COURSE))
    checks = json.load(open(CHECKS))
    only = set(sys.argv[1:])
    ids = set()
    unchecked = []
    for unit in course["units"]:
        if only and unit["id"] not in only:
            continue
        for i, b in enumerate(unit["lesson"]):
            if b["type"] != "code":
                continue
            where = f"{unit['id']}.lesson.{i}"
            lint(where, b["code"])
            count["lesson"] += 1
            if "output" in b:
                res = run(b["code"])
                if not ok(res, b["output"]):
                    bad(where, "output differs", res)
            else:
                res = run(b["code"], halt=True)
                if res["out"] or res["rc"] != 0:
                    bad(where, "block without output prints something", res)
            if res["err"]:
                bad(where, "warnings or errors on stderr", res)
        for e in unit["exercises"]:
            ids.add(e["id"])
            chk = checks.get(e["id"])
            if chk is None and e["type"] != "pairs":
                unchecked.append(e["id"])
            check_exercise(unit, e, chk)
    for k in checks:
        if k not in ids and not only:
            bad(k, "check without exercise")
    json.dump(cache, open(CACHE, "w"))
    print(VERSION)
    print(f"lesson blocks: {count['lesson']}, exercises checked: {count['exercise']}, extra order permutations: {count['runs']}")
    if unchecked:
        print("not checked by running (review by reading):", ", ".join(unchecked))
    if problems:
        print(f"\n{len(problems)} problem(s):")
        for p in problems:
            print(" -", p)
        sys.exit(1)
    print("all checks passed")


main()
