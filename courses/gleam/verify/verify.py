#!/usr/bin/env python3
"""Runs the Gleam course code with a local `gleam` (1.x) and checks it.

- every lesson code block: compiled and run on the JavaScript AND the Erlang target;
  stdout == `output` on both, exit code 0, no compiler warnings
- every exercise with an entry in checks.json (see README.md)
- formatting: every snippet (inside its template) is unchanged by `gleam format --stdin`

Results are cached in .cache.json (key: sha256 of Gleam version + target + program).
Usage: python3 verify.py [unit-id ...]
"""
import concurrent.futures, hashlib, itertools, json, os, re, shutil, subprocess, sys, tempfile, threading
from pathlib import Path

HERE = Path(__file__).resolve().parent
COURSE = HERE.parent / "course.json"
CHECKS = HERE / "checks.json"
CACHE = HERE / ".cache.json"
TIMEOUT = 30
WORKERS = 6
TARGETS = ("javascript", "erlang")

VERSION = subprocess.run(["gleam", "--version"], capture_output=True, text=True).stdout.strip()
cache = json.loads(CACHE.read_text()) if CACHE.exists() else {}
lock = threading.Lock()
pending = {}
collecting = True
tmpdir = Path(tempfile.mkdtemp(prefix="gleam-verify-"))


def key_of(src, target):
    return hashlib.sha256((VERSION + "\0" + target + "\0" + src).encode()).hexdigest()


def run(src, target="javascript"):
    """Returns (ok, stdout, stderr). ok: compiled and finished in time with exit code 0.
    In the first pass programs are only collected; they run in parallel afterwards."""
    k = key_of(src, target)
    if k in cache:
        c = cache[k]
        return c["ok"], c["stdout"], c["stderr"]
    if collecting:
        pending[k] = (src, target)
        return False, None, ""
    raise RuntimeError("program was not collected")


def setup_projects():
    base = tmpdir / "base"
    subprocess.run(["gleam", "new", "--skip-git", "--skip-github", "--name", "proj", str(base)],
                   capture_output=True, check=True)
    (base / "test").exists() and shutil.rmtree(base / "test")
    for t in TARGETS:
        subprocess.run(["gleam", "build", "--target", t], cwd=base, capture_output=True, check=True)
    dirs = []
    for i in range(WORKERS):
        d = tmpdir / f"w{i}"
        shutil.copytree(base, d, symlinks=True)
        dirs.append(d)
    return dirs


def execute(dirs):
    free = list(dirs)
    cond = threading.Condition()

    def job(item):
        k, (src, target) = item
        with cond:
            while not free:
                cond.wait()
            d = free.pop()
        try:
            (d / "src" / "proj.gleam").write_text(src)
            try:
                p = subprocess.run(["gleam", "run", "--target", target, "--no-print-progress"], cwd=d,
                                   capture_output=True, text=True, timeout=TIMEOUT)
                res = {"ok": p.returncode == 0, "stdout": p.stdout.removesuffix("\n"), "stderr": p.stderr}
            except subprocess.TimeoutExpired:
                res = {"ok": False, "stdout": "", "stderr": "timeout"}
        finally:
            with cond:
                free.append(d)
                cond.notify()
        with lock:
            cache[k] = res

    items = list(pending.items())
    with concurrent.futures.ThreadPoolExecutor(WORKERS) as ex:
        for i, _ in enumerate(ex.map(job, items), 1):
            if i % 50 == 0:
                print(f"  {i}/{len(items)} programs run", flush=True)
                save()
    save()
    return len(items)


def save():
    with lock:
        tmp = CACHE.with_suffix(".tmp")
        tmp.write_text(json.dumps(cache, indent=0))
        tmp.replace(CACHE)


def fill(tpl, code):
    """Puts code into the template at {{CODE}}, indented like the marker."""
    pos = tpl.index("{{CODE}}")
    line_start = tpl.rfind("\n", 0, pos) + 1
    before = tpl[line_start:pos]
    ind = before if before.strip() == "" else ""
    lines = code.split("\n")
    body = "\n".join([lines[0]] + [(ind + l) if l else l for l in lines[1:]])
    return tpl[:pos] + body + tpl[pos + len("{{CODE}}"):] + ("" if tpl.endswith("\n") else "\n")


def norm(s):
    return re.sub(r"[\s_'\"`]", "", s).lower()


STRING = re.compile(r'"(?:\\.|[^"\\])*"')


def plausible(lines):
    """Cheap filter for `order` permutations: brackets never close below zero and every
    line is indented by 2 spaces per open bracket (the `gleam format` layout). Orders that
    fail this cannot be formatted code, so they are not run."""
    depth = 0
    for line in lines:
        s = STRING.sub('""', line).split("//")[0]
        ind = len(line) - len(line.lstrip(" "))
        stripped = s.strip()
        want = depth - (1 if stripped[:1] in ")]}" else 0)
        if want < 0 or ind != 2 * want:
            return False
        depth += sum(s.count(c) for c in "([{") - sum(s.count(c) for c in ")]}")
        if depth < 0:
            return False
    return depth == 0


errors, notes = [], []
fmt_items = []


def format_check():
    """Formats every snippet with `gleam format --stdin`. `bug` and `order` lines cannot
    contain empty lines, so for them empty lines are ignored."""
    def strip(t):
        return [l for l in t.split("\n") if l != ""]

    for label, code, loose in fmt_items:
        p = subprocess.run(["gleam", "format", "--stdin"], input=code + "\n", capture_output=True, text=True)
        if p.returncode != 0:
            errors.append(f"format {label}: does not parse\n{p.stderr}")
            continue
        out, code = p.stdout.rstrip("\n"), code.rstrip("\n")
        same = strip(out) == strip(code) if loose else out == code
        if not same:
            errors.append(f"format {label}: differs, gleam format gives:\n{out}")
    print(f"format: {len(fmt_items)} snippets checked")


def expect(label, src, chk, strict=False, both=True):
    """Runs src (on both targets when `both`) and compares with chk."""
    first = None
    for target in (TARGETS if both else TARGETS[:1]):
        ok, out, err = run(src, target)
        if collecting:
            continue
        lab = f"{label} [{target}]"
        if chk.get("err"):
            if ok:
                errors.append(f"{lab}: expected an error, got output {out!r}")
        elif not ok:
            errors.append(f"{lab}: failed\n{err}\n--- program ---\n{src}")
        elif "out" in chk and out != chk["out"]:
            errors.append(f"{lab}: output {out!r}, expected {chk['out']!r}")
        elif "warning" in err:
            (errors if strict else notes).append(f"{lab}: warning\n{err}")
        if first is None:
            first = (ok, out)
    return first or (False, None)


def check_exercise(e, chk):
    eid, kind = e["id"], e["type"]
    tpl = chk.get("tpl", "{{CODE}}")

    def src_for(code):
        return fill(tpl, code)

    def other(code):
        ok, out, _ = run(src_for(code))
        return ok and out == chk["out"]

    if kind in ("input", "choice", "true_false"):
        code = chk.get("code", e.get("code"))
        if code is not None:
            ok, out = expect(eid, src_for(code), chk, strict=True)
            if e.get("code") and not collecting:
                fmt_items.append((eid, src_for(e["code"]), False))
            if kind == "input" and ok and not collecting and norm(out) not in {norm(a) for a in e["answers"]}:
                errors.append(f"{eid}: output {out!r} is not among the answers {e['answers']}")
    elif kind == "gap":
        for i, opt in enumerate(e["options"]):
            code = e["code"].replace("___", opt)
            if i == e["answer"]:
                expect(f"{eid} (right option)", src_for(code), chk, strict=True)
                if not collecting:
                    fmt_items.append((eid, src_for(code), False))
            elif other(code) and not collecting:
                errors.append(f"{eid}: wrong option {opt!r} also prints {chk['out']!r}")
    elif kind == "bug":
        lines = list(e["lines"])
        if other("\n".join(lines)) and not collecting:
            errors.append(f"{eid}: buggy code already prints the expected output")
        lines[e["bug"]] = chk["fix"]
        expect(f"{eid} (fixed)", src_for("\n".join(lines)), chk, strict=True)
        if not collecting:
            fmt_items.append((eid, src_for("\n".join(lines)), True))
    elif kind == "order":
        expect(eid, src_for("\n".join(e["lines"])), chk, strict=True)
        if not collecting:
            fmt_items.append((eid, src_for("\n".join(e["lines"])), True))
        for perm in itertools.permutations(e["lines"]):
            if list(perm) == e["lines"] or not plausible(perm):
                continue
            if other("\n".join(perm)) and not collecting:
                errors.append(f"{eid}: another order also prints {chk['out']!r}: {perm}")
    elif kind == "blocks":
        toks = e["tokens"]
        expect(eid, src_for(" ".join(toks)), chk, strict=True)
        for d in e["distractors"]:
            for i in range(len(toks)):
                if other(" ".join(toks[:i] + [d] + toks[i + 1:])) and not collecting:
                    errors.append(f"{eid}: distractor {d!r} at position {i} also works")
    for j, x in enumerate(chk.get("extra", [])):
        expect(f"{eid} (extra {j + 1})", x["src"], x)


def check_all(course, checks, only):
    n_blocks = n_ex = 0
    for u in course["units"]:
        if only and u["id"] not in only:
            continue
        for i, b in enumerate(u["lesson"]):
            if b["type"] != "code":
                continue
            label = f"{u['id']}.lesson.{i}"
            if not collecting:
                fmt_items.append((label, b["code"], False))
            expect(label, b["code"] + "\n", {"out": b.get("output", "")}, strict=True)
            n_blocks += 1
        for e in u["exercises"]:
            chk = checks.get(e["id"])
            if chk is None:
                if e.get("code") or e["type"] in ("gap", "bug", "order", "blocks"):
                    if not collecting:
                        errors.append(f"{e['id']}: no check")
                continue
            check_exercise(e, chk)
            n_ex += 1
    return n_blocks, n_ex


def main():
    global collecting
    course = json.loads(COURSE.read_text())
    checks = json.loads(CHECKS.read_text())
    only = set(sys.argv[1:])
    check_all(course, checks, only)
    new = 0
    if pending:
        print(f"running {len(pending)} new programs ...", flush=True)
        new = execute(setup_projects())
    collecting = False
    n_blocks, n_ex = check_all(course, checks, only)
    format_check()
    shutil.rmtree(tmpdir, ignore_errors=True)
    print(VERSION)
    print(f"lesson programs: {n_blocks} (x{len(TARGETS)} targets), exercises checked: {n_ex}, "
          f"new runs: {new}, cached runs: {len(cache)}")
    for n in notes:
        print("NOTE", n)
    for e in errors:
        print("ERROR", e)
    print("ALL PASSED" if not errors else f"{len(errors)} problem(s)")
    sys.exit(1 if errors else 0)


if __name__ == "__main__":
    main()
