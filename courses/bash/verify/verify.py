#!/usr/bin/env python3
"""Runs the Bash course code and checks it (see README.md).

Backends:
  mac      Homebrew Bash 5 with the macOS (BSD) tools
  gnu      Homebrew Bash 5 with GNU coreutils and GNU grep first in PATH
  wandbox  Bash 5.2 on Linux via the Wandbox API (only with --wandbox; cached)

Usage: python3 verify.py [--wandbox] [unit-id ...]
"""
import hashlib, itertools, json, os, re, shutil, subprocess, sys, tempfile, time, urllib.request
from pathlib import Path

HERE = Path(__file__).resolve().parent
COURSE = HERE.parent / "course.json"
CHECKS = HERE / "checks.json"
CACHE = HERE / ".cache.json"
WANDBOX = "https://wandbox.org/api/compile.json"

args = [a for a in sys.argv[1:] if not a.startswith("--")]
use_wandbox = "--wandbox" in sys.argv

brew = subprocess.run(["brew", "--prefix"], capture_output=True, text=True).stdout.strip() or "/opt/homebrew"
BASH = f"{brew}/bin/bash"
BASE_PATH = f"{brew}/bin:/usr/bin:/bin:/usr/sbin:/sbin"
GNU_PATH = f"{brew}/opt/coreutils/libexec/gnubin:{brew}/opt/grep/libexec/gnubin:" + BASE_PATH
for p in (BASH, f"{brew}/opt/coreutils/libexec/gnubin/sort", f"{brew}/opt/grep/libexec/gnubin/grep"):
    if not os.path.exists(p):
        raise SystemExit(f"missing {p}: brew install bash coreutils grep")

BACKENDS = ["mac", "gnu"] + (["wandbox"] if use_wandbox else [])
cache = json.loads(CACHE.read_text()) if CACHE.exists() else {}
stats = {"programs": 0, "runs": 0, "wandbox": 0}


def run_local(src, path):
    with tempfile.TemporaryDirectory() as d:
        Path(d, "script.sh").write_text(src + "\n")
        env = {"PATH": path, "HOME": d, "LANG": "en_US.UTF-8", "LC_ALL": "en_US.UTF-8"}
        try:
            r = subprocess.run([BASH, "script.sh"], cwd=d, env=env, capture_output=True, text=True, timeout=10)
            return r.returncode, r.stdout, r.stderr
        except subprocess.TimeoutExpired:
            return -1, "", "timeout"


def run_wandbox(src):
    key = "wandbox:" + hashlib.sha256(src.encode()).hexdigest()
    if key not in cache:
        body = json.dumps({"compiler": "bash", "code": src + "\n"}).encode()
        for attempt in range(4):
            try:
                req = urllib.request.Request(WANDBOX, data=body, headers={"Content-Type": "application/json", "User-Agent": "curl/8"})
                r = json.load(urllib.request.urlopen(req, timeout=60))
                break
            except Exception as e:  # noqa: BLE001
                print("  wandbox retry:", e, file=sys.stderr)
                time.sleep(5 * (attempt + 1))
        else:
            raise SystemExit("Wandbox not reachable (run without --wandbox)")
        time.sleep(1)
        cache[key] = [int(r.get("status", "1")), r.get("program_output", ""), r.get("program_error", "")]
        tmp = CACHE.with_suffix(".tmp")
        tmp.write_text(json.dumps(cache, indent=0))
        tmp.replace(CACHE)
        stats["wandbox"] += 1
    return tuple(cache[key])


def run1(src, backend):
    stats["runs"] += 1
    if backend == "mac":
        return run_local(src, BASE_PATH)
    if backend == "gnu":
        return run_local(src, GNU_PATH)
    return run_wandbox(src)


def strip1(s):
    return s[:-1] if s.endswith("\n") else s


def run(src, backends=None):
    """{backend: (ok, stdout, stderr)}; ok = exit status 0 and no stderr."""
    stats["programs"] += 1
    res = {}
    for b in backends or BACKENDS:
        code, out, err = run1(src, b)
        res[b] = (code == 0 and err == "", strip1(out), err)
    return res


def norm(s):
    return re.sub(r"[\s_'\"`]", "", s).lower()


errors = []


def expect(label, src, out=None, err=False, stderr_ok=False):
    """Every backend must give the same, expected result."""
    res = run(src)
    for b, (ok, o, e) in res.items():
        if err:
            if ok:
                errors.append(f"{label} [{b}]: expected an error, got {o!r}")
            continue
        if not ok and not stderr_ok:
            errors.append(f"{label} [{b}]: failed: {e.strip()}\n--- script ---\n{src}")
        elif out is not None and o != out:
            errors.append(f"{label} [{b}]: output {o!r}, expected {out!r}")
    return res


def same_as(res, out):
    """True when some backend runs the code without error and prints `out`."""
    return any(ok and o == out for ok, o, _ in res.values())


def fill(tpl, code):
    return tpl.replace("{{CODE}}", code) if tpl else code


def check_exercise(e, chk):
    eid, kind = e["id"], e["type"]
    src = lambda code: fill(chk.get("tpl"), code)
    out = chk.get("out")
    if kind in ("input", "choice", "true_false"):
        code = chk.get("code", e.get("code"))
        if code is not None:
            res = expect(eid, src(code), out, chk.get("err", False), chk.get("stderr", False))
            if kind == "input":
                for b, (ok, o, _) in res.items():
                    if norm(o) not in {norm(a) for a in e["answers"]}:
                        errors.append(f"{eid} [{b}]: output {o!r} is not among the answers {e['answers']}")
    elif kind == "gap":
        for i, opt in enumerate(e["options"]):
            code = e["code"].replace("___", opt)
            if i == e["answer"]:
                expect(f"{eid} (right option)", src(code), out)
            elif same_as(run(src(code)), out):
                errors.append(f"{eid}: wrong option {opt!r} also prints {out!r}")
    elif kind == "bug":
        lines = list(e["lines"])
        if same_as(run(src("\n".join(lines))), out):
            errors.append(f"{eid}: the buggy code already prints {out!r}")
        lines[e["bug"]] = chk["fix"]
        expect(f"{eid} (fixed)", src("\n".join(lines)), out)
    elif kind == "order":
        lines = e["lines"]
        expect(eid, src("\n".join(lines)), out)
        if len(lines) <= 6:
            for perm in itertools.permutations(lines):
                if list(perm) != lines and same_as(run(src("\n".join(perm)), ["mac"]), out):
                    errors.append(f"{eid}: another order also prints {out!r}: {perm}")
                    break
    elif kind == "blocks":
        toks = e["tokens"]
        expect(eid, src(" ".join(toks)), out)
        if chk.get("distractors"):
            for d in e["distractors"]:
                for i in range(len(toks)):
                    alt = toks[:i] + [d] + toks[i + 1:]
                    if same_as(run(src(" ".join(alt)), ["mac"]), out):
                        errors.append(f"{eid}: distractor {d!r} at position {i} also works")
    for j, x in enumerate(chk.get("extra", [])):
        expect(f"{eid} (extra {j + 1})", x["src"], x.get("out"), x.get("err", False), x.get("stderr", False))


course = json.loads(COURSE.read_text())
checks = json.loads(CHECKS.read_text())
unchecked, lessons = [], 0
for u in course["units"]:
    if args and u["id"] not in args:
        continue
    for i, b in enumerate(u["lesson"]):
        if b["type"] == "code":
            lessons += 1
            expect(f"{u['id']}.lesson.{i}", b["code"], b.get("output", ""))
    for e in u["exercises"]:
        if e["id"] in checks:
            check_exercise(e, checks[e["id"]])
        else:
            unchecked.append(e["id"])

print(f"backends: {', '.join(BACKENDS)} ({subprocess.run([BASH, '--version'], capture_output=True, text=True).stdout.splitlines()[0]})")
print(f"lesson blocks: {lessons}, exercises checked: {sum(1 for u in course['units'] for e in u['exercises'] if e['id'] in checks and (not args or u['id'] in args))}")
print(f"programs: {stats['programs']}, runs: {stats['runs']}, new Wandbox requests: {stats['wandbox']}")
print("not checked (review by reading):", ", ".join(unchecked) or "-")
if errors:
    print(f"\n{len(errors)} problem(s):")
    for x in errors:
        print("-", x)
    sys.exit(1)
print("all checks passed")
