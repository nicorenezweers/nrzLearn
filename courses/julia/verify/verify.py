#!/usr/bin/env python3
"""Runs the Julia course code on the Compiler Explorer API and checks it.

Every program runs on two Julia versions (1.10 and 1.12); both must agree.
See README.md for what is checked. Usage: python3 verify.py [unit-id ...]
"""
import base64, hashlib, json, re, sys, time, urllib.request
from pathlib import Path

HERE = Path(__file__).resolve().parent
COURSE = HERE.parent / "course.json"
CHECKS = HERE / "checks.json"
CACHE = HERE / ".cache.json"
HARNESS = (HERE / "harness.jl").read_text()
API = "https://godbolt.org/api/compiler/{}/compile"
COMPILERS = {"julia_1_10_0": 10, "julia_1_12_5": 5}  # compiler id -> snippets per request

cache = json.loads(CACHE.read_text()) if CACHE.exists() else {}
missing = {}  # key -> (compiler, src)
requests = 0


def key(comp, src):
    return comp + ":" + hashlib.sha256(src.encode()).hexdigest()


def save_cache():
    tmp = CACHE.with_suffix(".tmp")
    tmp.write_text(json.dumps(cache, indent=0))
    tmp.replace(CACHE)


def post(comp, program):
    global requests
    body = {"source": program, "options": {"userArguments": "", "compilerOptions": {"executorRequest": True},
            "filters": {"execute": True}, "tools": [], "libraries": []}, "lang": "julia", "allowStoreCodeDebug": True}
    for attempt in range(4):
        try:
            req = urllib.request.Request(API.format(comp), data=json.dumps(body).encode(),
                                         headers={"Content-Type": "application/json", "Accept": "application/json"})
            r = json.load(urllib.request.urlopen(req, timeout=180))
            break
        except Exception as e:  # noqa: BLE001
            print("  retry:", e, file=sys.stderr)
            time.sleep(5 * (attempt + 1))
    else:
        raise SystemExit("API not reachable")
    requests += 1
    time.sleep(1)
    return r


def b64(s):
    return base64.b64encode(s.encode()).decode()


def run_batch(comp, srcs):
    """Runs several snippets in one Julia process, each in a fresh module (see harness.jl)."""
    program = HARNESS.replace("SNIPS_HERE", ",\n".join(f'"{b64(s)}"' for s in srcs))
    r = post(comp, program)
    got = {}
    for line in r.get("stdout", []):
        parts = line["text"].split(" ")
        if parts[0] == "@@" and len(parts) == 5:
            got[int(parts[1]) - 1] = {"ok": parts[2] == "true", "out": base64.b64decode(parts[3]).decode(),
                                      "err": base64.b64decode(parts[4]).decode()}
    return got


def run_single(comp, src):
    """Runs one snippet as the main script (needed when output shows type names, e.g. structs)."""
    program = ('using Base64\ntry\n    include_string(Main, String(base64decode("' + b64(src) + '")), "main.jl")\n'
               'catch e\n    showerror(stderr, e)\n    exit(1)\nend\n')
    r = post(comp, program)
    out = "\n".join(x["text"] for x in r.get("stdout", []))
    err = "\n".join(x["text"] for x in r.get("stderr", []))
    return {"ok": r.get("code") == 0 and bool(r.get("didExecute")), "out": out + "\n" if out else "", "err": err}


def single_mode(src):
    return "struct " in src


def execute_missing():
    for comp, size in COMPILERS.items():
        todo = [src for (c, src) in missing.values() if c == comp]
        singles = [s for s in todo if single_mode(s)]
        batch = [s for s in todo if not single_mode(s)]
        for s in singles:
            cache[key(comp, s)] = run_single(comp, s)
            save_cache()
        while batch:
            part, batch = batch[:size], batch[size:]
            got = run_batch(comp, part)
            for i, s in enumerate(part):
                if i in got:
                    cache[key(comp, s)] = got[i]
            done = len(got)
            if done < len(part):
                if done == 0:
                    cache[key(comp, part[0])] = {"ok": False, "out": "", "err": "killed (time limit)"}
                    done = 1
                batch = part[done:] + batch
            save_cache()
            print(f"  {comp}: {len(batch)} left", file=sys.stderr)
    missing.clear()


def run(src):
    """Returns (ok, stdout without the final newline, diagnostics) or None while results are missing."""
    res = []
    for comp in COMPILERS:
        k = key(comp, src)
        if k not in cache:
            missing[k] = (comp, src)
        else:
            res.append(cache[k])
    if len(res) < len(COMPILERS):
        return None
    a, b = res
    norm_out = lambda c: c["out"][:-1] if c["out"].endswith("\n") else c["out"]
    if a["ok"] != b["ok"] or norm_out(a) != norm_out(b):
        version_diff.append((src, a, b))
    ok = a["ok"] and not a["err"].strip()
    return ok, norm_out(a), a["err"]


def norm(s):
    return re.sub(r"[\s_'\"`]", "", s).lower()


errors, version_diff = [], []
n_programs = 0


def expect(label, src, chk):
    r = run(src)
    if r is None:
        return None
    ok, out, diag = r
    if chk.get("err"):
        if ok:
            errors.append(f"{label}: expected an error, got output {out!r}")
    elif not ok:
        errors.append(f"{label}: failed\n{diag}\n--- program ---\n{src}")
    elif "out" in chk and out != chk["out"]:
        errors.append(f"{label}: output {out!r}, expected {chk['out']!r}")
    return r


def must_differ(label, src, out):
    r = run(src)
    if r is not None and r[0] and r[1] == out:
        errors.append(f"{label}: also prints {out!r}")


def fill(tpl, code):
    return tpl.replace("{{CODE}}", code)


def check_exercise(e, chk):
    eid, kind = e["id"], e["type"]
    src_for = lambda code: fill(chk.get("tpl", "{{CODE}}"), code)
    if kind in ("input", "choice", "true_false"):
        code = chk.get("code", e.get("code"))
        if code is not None:
            r = expect(eid, src_for(code), chk)
            if kind == "input" and r and r[0] and norm(r[1]) not in {norm(a) for a in e["answers"]}:
                errors.append(f"{eid}: output {r[1]!r} is not among the answers {e['answers']}")
    elif kind == "gap":
        for i, opt in enumerate(e["options"]):
            code = src_for(e["code"].replace("___", opt))
            if i == e["answer"]:
                expect(f"{eid} (right option)", code, chk)
            else:
                must_differ(f"{eid}: wrong option {opt!r}", code, chk["out"])
    elif kind == "bug":
        lines = list(e["lines"])
        must_differ(f"{eid}: the code with the bug", src_for("\n".join(lines)), chk["out"])
        lines[e["bug"]] = chk["fix"]
        expect(f"{eid} (fixed)", src_for("\n".join(lines)), chk)
    elif kind == "order":
        lines = e["lines"]
        expect(eid, src_for("\n".join(lines)), chk)
        for i in range(len(lines) - 1):
            alt = lines[:i] + [lines[i + 1], lines[i]] + lines[i + 2:]
            must_differ(f"{eid}: lines {i} and {i + 1} swapped", src_for("\n".join(alt)), chk["out"])
    elif kind == "blocks":
        expect(eid, src_for(" ".join(e["tokens"])), chk)
    for j, x in enumerate(chk.get("extra", [])):
        expect(f"{eid} (extra {j + 1})", x["src"], x)


def static_checks(course):
    """Comments in code must contain only code (reviewed by hand); print them."""
    for u in course["units"]:
        codes = [(f"{u['id']}.lesson.{i}", b["code"]) for i, b in enumerate(u["lesson"]) if b["type"] == "code"]
        for e in u["exercises"]:
            for k in ("code",):
                if e.get(k):
                    codes.append((e["id"], e[k]))
            if e["type"] in ("bug", "order"):
                codes.append((e["id"], "\n".join(e["lines"])))
        for label, c in codes:
            for line in c.split("\n"):
                if re.search(r'#(?![^"]*"[^"]*$)', line) and not re.search(r'"[^"]*#[^"]*"', line):
                    print(f"comment in {label}: {line}")


def all_checks(course, checks, only):
    global n_programs
    n_programs = 0
    for j, x in enumerate(checks.get("_lesson", {}).get("extra", [])):
        expect(f"lesson claim {j + 1}", x["src"], x)
    for u in course["units"]:
        if only and u["id"] not in only:
            continue
        for i, b in enumerate(u["lesson"]):
            if b["type"] == "code":
                expect(f"{u['id']}.lesson.{i}", b["code"], {"out": b.get("output", "")})
                n_programs += 1
        for e in u["exercises"]:
            chk = checks.get(e["id"])
            if chk is None:
                if e.get("code") or e["type"] in ("gap", "bug", "order", "blocks"):
                    errors.append(f"{e['id']}: no check")
                continue
            check_exercise(e, chk)


def main():
    course = json.loads(COURSE.read_text())
    checks = json.loads(CHECKS.read_text())
    only = set(sys.argv[1:])
    all_checks(course, checks, only)
    if missing:
        print(f"running {len(missing)} new program runs ...", file=sys.stderr)
        execute_missing()
        errors.clear()
        version_diff.clear()
        all_checks(course, checks, only)
    static_checks(course)
    n_ex = sum(1 for u in course["units"] if not only or u["id"] in only for e in u["exercises"] if e["id"] in checks)
    print(f"lesson programs: {n_programs}, exercises checked: {n_ex}, API requests: {requests}, cached runs: {len(cache)}")
    for src, a, b in version_diff:
        errors.append(f"Julia 1.10 and 1.12 differ:\n{src}\n1.10: {a}\n1.12: {b}")
    for e in errors:
        print("ERROR", e)
    print("ALL PASSED" if not errors else f"{len(errors)} problem(s)")
    sys.exit(1 if errors else 0)


if __name__ == "__main__":
    main()
