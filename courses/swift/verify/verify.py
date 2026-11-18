#!/usr/bin/env python3
"""Runs every Swift snippet of ../course.json and checks outputs and exercise answers.

Usage: python3 courses/swift/verify/verify.py [--jobs N]
Exit code 0 means every check passed. See README.md.
"""
import hashlib
import itertools
import json
import re
import subprocess
import sys
import tempfile
import threading
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

HERE = Path(__file__).resolve().parent
COURSE = json.loads((HERE.parent / "course.json").read_text())
CHECKS = json.loads((HERE / "checks.json").read_text())
JOBS = int(sys.argv[sys.argv.index("--jobs") + 1]) if "--jobs" in sys.argv else 8

TMP = Path(tempfile.mkdtemp(prefix="swift-verify-"))
SLOTS = threading.Semaphore(JOBS)
CACHE = {}
CACHE_LOCK = threading.Lock()
RUNS = [0]


def run(src):
    """Runs Swift code with `swift file.swift`. Returns (ok, stdout without the final newline)."""
    key = hashlib.sha1(src.encode()).hexdigest()
    with CACHE_LOCK:
        if key in CACHE:
            return CACHE[key]
    path = TMP / f"s{key}.swift"
    path.write_text(src + "\n")
    with SLOTS:
        try:
            p = subprocess.run(["swift", str(path)], capture_output=True, text=True, timeout=60)
            res = (p.returncode == 0, p.stdout[:-1] if p.stdout.endswith("\n") else p.stdout)
        except subprocess.TimeoutExpired:
            res = (False, "<timeout>")
    with CACHE_LOCK:
        CACHE[key] = res
        RUNS[0] += 1
    return res


def norm(s):
    """Compares like the app: ignores case, spaces, `_`, quotes and backticks."""
    return re.sub(r"[\s_'\"`]", "", s).lower()


def strip_ticks(s):
    return s[1:-1] if len(s) > 1 and s.startswith("`") and s.endswith("`") and s.count("`") == 2 else None


def wrap(src, case):
    return "\n".join(x for x in (case.get("before", ""), src, case.get("after", "")) if x)


def cases_of(chk):
    if "cases" in chk:
        return chk["cases"]
    return [{"before": chk.get("before", ""), "after": chk.get("after", ""), "out": chk.get("out")}]


def gives(src, chk):
    """True when the code gives the expected output in every case."""
    for c in cases_of(chk):
        ok, out = run(wrap(src, c))
        if not ok or out != c["out"]:
            return False
    return True


def expectation(src, exp):
    ok, out = run(src)
    if exp == "!error":
        return not ok
    if exp == "!ok":
        return ok
    return ok and out == exp


def check_exercise(e):
    eid, kind, chk = e["id"], e["type"], CHECKS.get(e["id"], {})
    errs = []
    for code, exp in chk.get("probes", []):
        if not expectation(code, exp):
            errs.append(f"probe failed: {code!r} expected {exp!r}, got {run(code)}")

    if kind == "input":
        ok, out = run(wrap(e["code"], chk))
        if not ok:
            errs.append(f"code does not run: {out!r}")
        elif "\n" in out:
            errs.append(f"output has several lines: {out!r}")
        elif norm(out) != norm(e["answers"][0]):
            errs.append(f"output {out!r} != answer {e['answers'][0]!r}")

    elif kind == "choice":
        if "variants" in chk:
            hits = [i for i, v in enumerate(chk["variants"]) if gives(v, chk)]
            if hits != [e["answer"]]:
                errs.append(f"variants giving the output: {hits}, answer {e['answer']}")
        elif "map" in chk:
            ok, out = run(wrap(e["code"], chk))
            if chk["map"].get(out if ok else "!error") != e["answer"]:
                errs.append(f"output {out!r} maps to {chk['map'].get(out)}, answer {e['answer']}")
        elif e.get("code"):
            ok, out = run(wrap(e["code"], chk))
            hits = [i for i, o in enumerate(e["options"])
                    if (not ok and o == "An error") or (ok and strip_ticks(o) == out)]
            if hits != [e["answer"]]:
                errs.append(f"output {out!r} (ok={ok}) matches options {hits}, answer {e['answer']}")
        elif "probes" not in chk:
            errs.append("no check for this choice")

    elif kind == "true_false":
        if "probe" not in chk:
            errs.append("no probe")
        elif expectation(*chk["probe"]) != e["answer"]:
            errs.append(f"probe says {not e['answer']}, answer {e['answer']}: {run(chk['probe'][0])}")

    elif kind == "gap":
        hits = [i for i, o in enumerate(e["options"]) if gives(e["code"].replace("___", o), chk)]
        if hits != [e["answer"]]:
            errs.append(f"options giving the output: {hits}, answer {e['answer']}")

    elif kind == "blocks":
        toks = e["tokens"]
        if not gives(" ".join(toks), chk):
            errs.append(f"right order fails: {' '.join(toks)!r} -> {run(wrap(' '.join(toks), cases_of(chk)[0]))}")
        variants = set()
        for i, d in itertools.product(range(len(toks)), e["distractors"]):
            variants.add(" ".join(toks[:i] + [d] + toks[i + 1:]))
        variants |= {" ".join(p) for p in sample_orders(toks)}
        variants.discard(" ".join(toks))
        with ThreadPoolExecutor(16) as pool:
            good = [v for v, g in zip(variants, pool.map(lambda v: gives(v, chk), variants)) if g]
        if good:
            errs.append(f"other block lines also work: {good}")

    elif kind == "order":
        lines = e["lines"]
        if not gives("\n".join(lines), chk):
            errs.append(f"right order fails: {run(wrap(chr(10).join(lines), cases_of(chk)[0]))}")
        orders = {"\n".join(p) for p in sample_orders(lines)} - {"\n".join(lines)}
        with ThreadPoolExecutor(16) as pool:
            good = [o for o, g in zip(orders, pool.map(lambda o: gives(o, chk), orders)) if g]
        if good:
            errs.append(f"{len(good)} other orders also work, e.g. {good[0]!r}")

    elif kind == "bug":
        lines = e["lines"]
        if "fix" not in chk:
            errs.append("no fix")
        else:
            if gives("\n".join(lines), chk):
                errs.append("the original lines already give the expected output")
            fixed = lines[:e["bug"]] + [chk["fix"]] + lines[e["bug"] + 1:]
            if not gives("\n".join(fixed), chk):
                errs.append(f"the fixed lines fail: {run(wrap(chr(10).join(fixed), cases_of(chk)[0]))}")

    elif kind == "pairs":
        if "probes" not in chk:
            errs.append("no probes")
    return errs


def sample_orders(items):
    """All orders for up to 5 items, otherwise every order that moves one item or swaps two."""
    n = len(items)
    if n <= 5:
        return [list(p) for p in itertools.permutations(items)]
    out = []
    for i, j in itertools.permutations(range(n), 2):
        moved = items[:i] + items[i + 1:]
        moved.insert(j, items[i])
        out.append(moved)
        swapped = list(items)
        swapped[i], swapped[j] = swapped[j], swapped[i]
        out.append(swapped)
    return out


def main():
    jobs = []
    for u in COURSE["units"]:
        for i, b in enumerate(u["lesson"]):
            if b["type"] == "code":
                jobs.append((f"{u['id']}.lesson.{i}", "lesson", b))
        for e in u["exercises"]:
            jobs.append((e["id"], "exercise", e))

    def work(job):
        name, what, item = job
        if what == "lesson":
            ok, out = run(item["code"])
            exp = item.get("output", "")
            if not ok:
                return name, [f"does not run: {out!r}"]
            return name, ([] if out == exp else [f"output {out!r} != {exp!r}"])
        return name, check_exercise(item)

    with ThreadPoolExecutor(32) as pool:
        results = list(pool.map(work, jobs))

    failed = [(n, e) for n, e in results if e]
    for n, errs in failed:
        for x in errs:
            print(f"FAIL {n}: {x}")
    unused = set(CHECKS) - {e["id"] for u in COURSE["units"] for e in u["exercises"]}
    for x in sorted(unused):
        print(f"FAIL checks.json: unknown exercise {x}")

    slashes = []
    for u in COURSE["units"]:
        codes = [b["code"] for b in u["lesson"] if b["type"] == "code"]
        codes += [e.get("code", "") for e in u["exercises"]] + ["\n".join(e.get("lines", [])) for e in u["exercises"]]
        slashes += [line for c in codes for line in c.split("\n") if "//" in line]
    print("Lines with // (check that they hold no natural-language comment):")
    for line in slashes:
        print("   ", line)
    lessons = sum(1 for _, w, _ in jobs if w == "lesson")
    print(f"{lessons} lesson code blocks, {len(jobs) - lessons} exercises, {RUNS[0]} Swift runs, "
          f"{len(failed) + len(unused)} failures")
    sys.exit(1 if failed or unused else 0)


if __name__ == "__main__":
    main()
