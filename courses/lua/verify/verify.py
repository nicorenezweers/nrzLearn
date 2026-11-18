#!/usr/bin/env python3
"""Runs the Lua course code and checks it (see README.md).

Usage: python3 verify.py [--lua PATH] [--online] [unit-id ...]
"""
import argparse, hashlib, itertools, json, re, shutil, subprocess, sys, tempfile, time, urllib.request
from pathlib import Path

HERE = Path(__file__).resolve().parent
COURSE = json.loads((HERE.parent / "course.json").read_text())
CHECKS = json.loads((HERE / "checks.json").read_text())
CACHE = HERE / ".cache.json"

ap = argparse.ArgumentParser()
ap.add_argument("--lua", default=shutil.which("lua5.4") or "/opt/homebrew/opt/lua@5.4/bin/lua")
ap.add_argument("--online", action="store_true", help="also run every checked program on Compiler Explorer (Lua 5.4.7)")
ap.add_argument("units", nargs="*")
args = ap.parse_args()

TMP = Path(tempfile.mkdtemp())
runs = 0


def run(src, timeout=2):
    """Returns (ok, stdout). ok: exit code 0 within the timeout."""
    global runs
    runs += 1
    f = TMP / "main.lua"
    f.write_text(src + "\n")
    try:
        p = subprocess.run([args.lua, str(f)], capture_output=True, text=True, timeout=timeout)
    except subprocess.TimeoutExpired:
        return False, "<timeout>"
    out = p.stdout[:-1] if p.stdout.endswith("\n") else p.stdout
    return p.returncode == 0, out


cache = json.loads(CACHE.read_text()) if CACHE.exists() else {}
online_programs = []


def online(src):
    """Runs src on Compiler Explorer (Lua 5.4.7); cached by SHA-256."""
    key = hashlib.sha256(src.encode()).hexdigest()
    if key not in cache:
        body = json.dumps({"source": src + "\n", "options": {"userArguments": "", "compilerOptions": {"executorRequest": True},
                           "filters": {"execute": True}, "tools": [], "libraries": []},
                           "lang": "lua", "allowStoreCodeDebug": True}).encode()
        for attempt in range(4):
            try:
                req = urllib.request.Request("https://godbolt.org/api/compiler/lua547/compile", data=body,
                                             headers={"Content-Type": "application/json", "Accept": "application/json"})
                r = json.load(urllib.request.urlopen(req, timeout=60))
                break
            except Exception as e:  # noqa: BLE001
                print("  retry:", e, file=sys.stderr)
                time.sleep(5 * (attempt + 1))
        else:
            raise SystemExit("Compiler Explorer not reachable")
        time.sleep(1)
        cache[key] = {"ok": bool(r.get("didExecute")) and r.get("code") == 0,
                      "stdout": "\n".join(x["text"] for x in r.get("stdout", []))}
        CACHE.write_text(json.dumps(cache, indent=0))
    c = cache[key]
    return c["ok"], c["stdout"]


def norm(s):
    return re.sub(r"[\s_'\"`]", "", s).lower()


def strip_ticks(s):
    m = re.fullmatch(r"`([^`]*)`", s)
    return m.group(1) if m else None


errors = []
checked = 0


def expect(what, src, chk):
    """Runs src; must print chk['out'] (or fail when chk['err'])."""
    ok, out = run(src)
    if args.online:
        online_programs.append((what, src, ok, out))
    if chk.get("err"):
        if ok:
            errors.append(f"{what}: expected an error, got {out!r}")
        return out
    if not ok:
        errors.append(f"{what}: failed: {out!r}\n{src}")
    elif out != chk["out"]:
        errors.append(f"{what}: printed {out!r}, expected {chk['out']!r}")
    return out


def wrap(chk, code):
    parts = [chk["pre"]] if "pre" in chk else []
    parts.append(code)
    if "post" in chk:
        parts.append(chk["post"])
    return "\n".join(parts)


def well_indented(lines):
    """True when every line is indented 4 spaces per open block (what a learner sees)."""
    depth = 0
    for line in lines:
        code = line.strip()
        if re.match(r"(end|else|elseif|until)\b", code):
            depth -= 1
        if depth < 0 or len(line) - len(line.lstrip(" ")) != 4 * depth:
            return False
        if re.search(r"\b(then|do)$", code) or re.match(r"(else|repeat)\b", code) or re.search(r"\bfunction\b[^)]*\)$", code):
            depth += 1
    return depth == 0


def differs(what, src, out):
    ok, got = run(src)
    if ok and got == out:
        errors.append(f"{what}: should fail or print something else, but printed {got!r}")


if not args.units:
    for k, x in enumerate(CHECKS.get("_facts", [])):
        checked += 1
        expect(f"fact {k} ({x['what']})", x["src"], x)

for unit in COURSE["units"]:
    if args.units and unit["id"] not in args.units:
        continue
    for i, b in enumerate(unit["lesson"]):
        if b["type"] != "code":
            continue
        checked += 1
        what = f"{unit['id']}.lesson.{i}"
        out = expect(what, b["code"], {"out": b.get("output", "")})
        for line in b["code"].split("\n"):
            if "--" in line and not re.search(r"-- [\w.\"' ]+$", line):
                errors.append(f"{what}: check the comment in {line!r}")
    for ex in unit["exercises"]:
        eid, t = ex["id"], ex["type"]
        chk = CHECKS.get(eid)
        if t == "pairs":
            continue
        if chk is None:
            print(f"  no check: {eid} ({t}), review by reading")
            continue
        checked += 1
        for k, x in enumerate(chk.get("extra", [])):
            expect(f"{eid}.extra.{k}", x["src"], x)
        if "out" not in chk and not chk.get("err"):
            continue
        if t == "input":
            expect(eid, wrap(chk, chk.get("code", ex.get("code", ""))), chk)
            if not any(norm(a) == norm(chk["out"]) for a in ex["answers"]):
                errors.append(f"{eid}: output {chk['out']!r} is not among the answers {ex['answers']}")
        elif t == "choice":
            expect(eid, wrap(chk, chk.get("code", ex.get("code", ""))), chk)
            shown = [strip_ticks(o) for o in ex["options"]]
            right = shown[ex["answer"]]
            if chk.get("err"):
                if right is not None:
                    errors.append(f"{eid}: the right option should describe the error")
            elif right is not None and right != chk["out"] and "How many" not in ex["prompt"]:
                errors.append(f"{eid}: right option {right!r} != output {chk['out']!r}")
            for k, o in enumerate(shown):
                if k != ex["answer"] and o is not None and o == chk.get("out"):
                    errors.append(f"{eid}: wrong option {k} equals the output")
        elif t == "true_false":
            out = expect(eid, wrap(chk, chk.get("code", ex.get("code", ""))), chk)
            if (chk["claim"] == chk["out"]) != ex["answer"]:
                errors.append(f"{eid}: answer {ex['answer']} does not fit claim {chk['claim']!r} / output {chk['out']!r}")
        elif t == "gap":
            for k, o in enumerate(ex["options"]):
                src = wrap(chk, ex["code"].replace("___", o))
                if k == ex["answer"]:
                    expect(f"{eid} (right option)", src, chk)
                else:
                    differs(f"{eid} (option {o!r})", src, chk["out"])
        elif t == "bug":
            lines = list(ex["lines"])
            differs(f"{eid} (as shown)", wrap(chk, "\n".join(lines)), chk["out"])
            lines[ex["bug"]] = chk["fix"]
            expect(f"{eid} (fixed)", wrap(chk, "\n".join(lines)), chk)
            for k in range(len(ex["lines"])):
                if k != ex["bug"] and ex["lines"][k] in (chk["fix"],):
                    errors.append(f"{eid}: fix equals line {k}")
        elif t == "order":
            expect(eid, wrap(chk, "\n".join(ex["lines"])), chk)
            if not well_indented(ex["lines"]):
                errors.append(f"{eid}: the right order is not indented consistently")
            same = []
            for perm in itertools.permutations(range(len(ex["lines"]))):
                if list(perm) == list(range(len(ex["lines"]))) or not well_indented([ex["lines"][p] for p in perm]):
                    continue
                ok, out = run(wrap(chk, "\n".join(ex["lines"][p] for p in perm)), timeout=0.5)
                if ok and out == chk["out"]:
                    same.append(perm)
            if same:
                errors.append(f"{eid}: ambiguous, other orders also print the output: {same[:3]}")
        elif t == "blocks":
            expect(eid, wrap(chk, " ".join(ex["tokens"])), chk)

if args.online:
    for what, src, ok, out in online_programs:
        wok, wout = online(src)
        if (wok, wout) != (ok, out) and not (not ok and not wok):
            errors.append(f"{what}: Compiler Explorer Lua 5.4.7 differs: {wout!r} vs local {out!r}")

ver = subprocess.run([args.lua, "-v"], capture_output=True, text=True).stdout.strip()
print(f"{ver}: {checked} items checked, {runs} local runs" + (f", {len(online_programs)} Compiler Explorer programs" if args.online else ""))
for e in errors:
    print("ERROR", e)
print("OK" if not errors else f"{len(errors)} problems")
sys.exit(1 if errors else 0)
