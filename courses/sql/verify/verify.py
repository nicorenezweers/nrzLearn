#!/usr/bin/env python3
"""Checks the SQL course (../course.json) by running every query with sqlite3.

- Lesson: every code block runs without an error against a fresh sample
  database; blocks with `output` must print exactly that (`.mode table`).
- Exercises: input answers, choice/true_false facts, gap options, blocks,
  order lines (all permutations) and bug fixes, see README.md and checks.py.

Usage: python3 verify.py [path/to/course.json]
"""
import itertools
import json
import subprocess
import sys
from pathlib import Path

from checks import CHECKS

HERE = Path(__file__).resolve().parent
COURSE = Path(sys.argv[1]) if len(sys.argv) > 1 else HERE.parent / "course.json"

failures = []
stats = {"lesson": 0, "exercise_runs": 0, "auto": 0, "manual": []}


def fail(what, msg):
    failures.append(f"{what}: {msg}")


def run(setup, code, mode="list", headers=False):
    """Runs setup + code in a fresh in-memory database. Returns (ok, stdout, stderr)."""
    args = ["sqlite3", "-bail", "-batch"]
    if mode == "table":
        args += ["-cmd", ".mode table"]
    else:
        args += ["-cmd", ".mode list", "-cmd", ".headers " + ("on" if headers else "off")]
    p = subprocess.run(args + [":memory:"], input=setup + "\n" + code + "\n", capture_output=True, text=True)
    stats["exercise_runs"] += 1
    ok = p.returncode == 0 and not p.stderr.strip()
    return ok, p.stdout.rstrip("\n"), p.stderr.strip()


def norm(s):
    """Answer comparison of the app: ignores case, spaces, `_`, quotes and backticks."""
    return "".join(c for c in s.lower() if not c.isspace() and c not in "_'\"`")


def strip_ticks(s):
    return s.replace("`", "")


course = json.loads(COURSE.read_text())
units = course["units"]

setups = []
for u in units:
    for b in u["lesson"]:
        if b["type"] == "code" and "output" not in b and b["code"].startswith("CREATE TABLE") and u["id"] in ("select", "joins"):
            setups.append((u["id"], b["code"]))
setup_ids = [s[0] for s in setups]
if setup_ids != ["select", "joins"]:
    fail("setup", f"expected the books table in `select` and the authors table in `joins`, found {setup_ids}")


def setup_for(unit_index, before=None):
    """Sample database statements for a unit: books always, authors from `joins` on."""
    join_index = next(i for i, u in enumerate(units) if u["id"] == "joins")
    parts = []
    for uid, code in setups:
        if code == before:
            break
        if uid == "select" or unit_index >= join_index:
            parts.append(code)
    return "\n".join(parts)


# ───────────────────────────── lesson ─────────────────────────────
for ui, u in enumerate(units):
    for bi, b in enumerate(u["lesson"]):
        if b["type"] != "code":
            continue
        what = f"{u['id']}.lesson.{bi}"
        is_setup = any(b["code"] == c for _, c in setups)
        setup = setup_for(ui, before=b["code"] if is_setup else None)
        ok, out, err = run(setup, b["code"], mode="table")
        stats["lesson"] += 1
        if not ok:
            fail(what, f"error: {err}")
        elif out != b.get("output", ""):
            fail(what, f"output differs\n--- expected\n{b.get('output', '')}\n--- actual\n{out}")


# ───────────────────────────── exercises ─────────────────────────────
def apply_check(what, setup, ex, chk):
    """Runs one check dict. Returns the measured value (for input answers) or None."""
    sql = chk.get("sql", ex.get("code"))
    if sql is None:
        fail(what, "check has no sql and the exercise has no code")
        return None
    ok, out, err = run(setup, sql, mode=chk.get("mode", "list"), headers=chk.get("headers", False))
    if "error" in chk:
        if ok or chk["error"] not in err:
            fail(what, f"expected an error containing {chk['error']!r}, got ok={ok} out={out!r} err={err!r}")
        return err
    if not ok:
        fail(what, f"error: {err}\n{sql}")
        return None
    lines = out.split("\n") if out else []
    if "expect" in chk and out != chk["expect"]:
        fail(what, f"expected {chk['expect']!r}, got {out!r}")
    if "first" in chk and (not lines or lines[0] != chk["first"]):
        fail(what, f"expected first line {chk['first']!r}, got {lines[:1]!r}")
    if "contains_line" in chk and chk["contains_line"] not in lines:
        fail(what, f"expected a line {chk['contains_line']!r} in {lines!r}")
    if "rows" in chk and len(lines) != chk["rows"]:
        fail(what, f"expected {chk['rows']} rows, got {len(lines)}")
    measure = chk.get("measure", "output")
    if measure == "rows":
        return str(len(lines))
    if measure == "cols":
        return str(len(lines[0].split("|"))) if lines else "0"
    return out


for ui, u in enumerate(units):
    setup = setup_for(ui)
    for ex in u["exercises"]:
        eid, kind = ex["id"], ex["type"]
        checks = CHECKS.get(eid, [])
        if isinstance(checks, dict):
            checks = [checks]
        auto = False

        if kind == "input":
            if not checks:
                checks = [{}]
            values = [apply_check(eid, setup, ex, c) for c in checks]
            measured = [v for c, v in zip(checks, values) if v is not None and "error" not in c]
            if not measured or norm(measured[0]) != norm(ex["answers"][0]):
                fail(eid, f"answer {ex['answers'][0]!r} but the query gives {measured[:1]!r}")
            auto = True

        elif kind in ("choice", "true_false"):
            for c in checks:
                apply_check(eid, setup, ex, c)
                if "answer_contains" in c:
                    s = c["answer_contains"]
                    hits = [i for i, o in enumerate(ex["options"]) if s in o]
                    if hits != [ex["answer"]]:
                        fail(eid, f"{s!r} should be only in the right option, found in options {hits}")
            if checks:
                auto = True
            elif ex.get("code"):
                fail(eid, "has code but no check in checks.py")

        elif kind == "gap":
            outputs = []
            for i, opt in enumerate(ex["options"]):
                code = ex["code"].replace("___", opt)
                outputs.append(run(setup, code, mode="table"))
            ok, out, err = outputs[ex["answer"]]
            if not ok:
                fail(eid, f"right option gives an error: {err}")
            for i, (wok, wout, _) in enumerate(outputs):
                if i != ex["answer"] and wok and wout == out:
                    fail(eid, f"wrong option {ex['options'][i]!r} gives the same result as the right one")
            filled = dict(ex, code=ex["code"].replace("___", ex["options"][ex["answer"]]))
            for c in checks:
                apply_check(eid, setup, filled, c)
            auto = True

        elif kind == "blocks":
            code = " ".join(ex["tokens"])
            ok, out, err = run(setup, code)
            if not ok:
                fail(eid, f"joined tokens give an error: {err}\n{code}")
            for c in checks:
                apply_check(eid, setup, dict(ex, code=code), c)
            auto = True

        elif kind == "order":
            lines = ex["lines"]
            code = "\n".join(lines)
            ok, out, err = run(setup, code, mode="table")
            if not ok:
                fail(eid, f"lines in order give an error: {err}")
            for c in checks:
                apply_check(eid, setup, dict(ex, code=code), c)
            if len(lines) <= 7:
                for perm in itertools.permutations(lines):
                    if list(perm) == lines:
                        continue
                    pok, pout, _ = run(setup, "\n".join(perm), mode="table")
                    if pok and pout == out:
                        fail(eid, "another order gives the same result:\n" + "\n".join(perm))
                        break
            else:
                stats["manual"].append(f"{eid} (order not checked for ambiguity: more than 7 lines)")
            auto = True

        elif kind == "bug":
            fix = [c for c in checks if "fix" in c]
            if len(fix) != 1:
                fail(eid, "needs exactly one {'fix': ...} entry in checks.py")
            else:
                fixed = list(ex["lines"])
                fixed[ex["bug"]] = fix[0]["fix"]
                bok, bout, berr = run(setup, "\n".join(ex["lines"]), mode="table")
                fok, fout, ferr = run(setup, "\n".join(fixed), mode="table")
                if not fok:
                    fail(eid, f"fixed code gives an error: {ferr}")
                if bok and bout == fout:
                    fail(eid, "the code with the bug gives the same result as the fixed code")
                for c in checks:
                    if "fix" not in c:
                        apply_check(eid, setup, dict(ex, code="\n".join(fixed)), c)
                auto = True

        if auto:
            stats["auto"] += 1
        else:
            stats["manual"].append(f"{eid} ({kind})")

total = sum(len(u["exercises"]) for u in units)
print(f"lesson code blocks run: {stats['lesson']}")
print(f"exercises checked automatically: {stats['auto']} of {total}")
print(f"sqlite3 runs: {stats['exercise_runs']}")
print("checked by reading only: " + (", ".join(stats["manual"]) or "none"))
if failures:
    print(f"\n{len(failures)} FAILURE(S):")
    for f in failures:
        print(" -", f)
    sys.exit(1)
print("ALL PASSED")
