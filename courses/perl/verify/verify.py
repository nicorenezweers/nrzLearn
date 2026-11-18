#!/usr/bin/env python3
"""Runs the Perl course code and checks it (see README.md).

Every program runs with the local `perl` and online with Perl 5.44.0
(Compiler Explorer by default, Wandbox with --wandbox; --local skips the online run).
Online results are cached in .cache.json (key: sha256 of service + program).
Usage: python3 verify.py [--local | --wandbox] [unit-id ...]
"""
import hashlib, itertools, json, re, subprocess, sys, tempfile, time, urllib.request
from pathlib import Path

HERE = Path(__file__).resolve().parent
COURSE = HERE.parent / "course.json"
CHECKS = HERE / "checks.json"
CACHE = HERE / ".cache.json"
WANDBOX = "https://wandbox.org/api/compile.json"
WANDBOX_PERL = "perl-5.44.0"
GODBOLT = "https://godbolt.org/api/compiler/perl5440/compile"
HEAD = "use strict;\nuse warnings;\nuse feature 'say';\n"

args = [a for a in sys.argv[1:] if not a.startswith("--")]
LOCAL_ONLY = "--local" in sys.argv
cache = json.loads(CACHE.read_text()) if CACHE.exists() else {}
stats = {"local": 0, "online": 0, "programs": set()}
errors = []


def run_local(src):
    with tempfile.NamedTemporaryFile("w", suffix=".pl", delete=False) as f:
        f.write(src)
    try:
        p = subprocess.run(["perl", f.name], capture_output=True, text=True, timeout=10, stdin=subprocess.DEVNULL)
        res = {"code": p.returncode, "stdout": p.stdout, "stderr": p.stderr}
    except subprocess.TimeoutExpired:
        res = {"code": -1, "stdout": "", "stderr": "timeout"}
    Path(f.name).unlink()
    stats["local"] += 1
    return res


def post(url, body):
    req = urllib.request.Request(url, data=json.dumps(body).encode(), headers={
        "Content-Type": "application/json", "Accept": "application/json", "User-Agent": "course-verify/1.0"})
    return json.load(urllib.request.urlopen(req, timeout=60))


def call_wandbox(src):
    r = post(WANDBOX, {"compiler": WANDBOX_PERL, "code": src})
    return {"code": int(r.get("status") or 1), "stdout": r.get("program_output", ""),
            "stderr": r.get("program_error", "") + r.get("compiler_error", "")}


def call_godbolt(src):
    r = post(GODBOLT, {"source": src, "options": {"userArguments": "", "compilerOptions": {"executorRequest": True},
                       "filters": {"execute": True}, "tools": [], "libraries": []}, "lang": "perl",
                       "allowStoreCodeDebug": True})
    if not r.get("didExecute"):
        return {"code": -1, "stdout": "", "stderr": json.dumps(r)[:500]}
    lines = [x["text"] for x in r.get("stdout", [])]
    return {"code": r.get("code"), "stdout": "\n".join(lines) + ("\n" if lines else ""),
            "stderr": "\n".join(x["text"] for x in r.get("stderr", []))}


def run_online(src):
    name = ONLINE[0]
    key = hashlib.sha256((name + "\0" + src).encode()).hexdigest()
    if key not in cache:
        for attempt in range(4):
            try:
                res = ONLINE[1](src)
                break
            except Exception as e:  # noqa: BLE001
                print(f"  {name} retry:", e, file=sys.stderr)
                time.sleep(5 * (attempt + 1))
        else:
            raise SystemExit(f"{name} not reachable (use --local or the other service)")
        time.sleep(1)
        cache[key] = res
        tmp = CACHE.with_suffix(".tmp")
        tmp.write_text(json.dumps(cache, indent=0))
        tmp.replace(CACHE)
        stats["online"] += 1
    return cache[key]


def runs(src):
    stats["programs"].add(src)
    out = [("perl 5.34 local", run_local(src))]
    if not LOCAL_ONLY:
        out.append((ONLINE[0], run_online(src)))
    return out


def matches(res, chk):
    """True when a result fulfils the check (out, warn, err)."""
    if chk.get("err"):
        return res["code"] != 0
    if res["code"] != 0:
        return False
    if bool(res["stderr"].strip()) != bool(chk.get("warn")):
        return False
    want = chk["out"] + "\n" if chk.get("out") else ""
    return res["stdout"] == want


def expect(label, src, chk):
    for name, res in runs(src):
        if not matches(res, chk):
            errors.append(f"{label} [{name}]: exit {res['code']}, stdout {res['stdout']!r}, stderr {res['stderr'].strip()!r}, "
                          f"expected {chk}\n--- program ---\n{src}")


def expect_wrong(label, src, chk):
    """A wrong variant must not give the expected result (it fails, warns or prints something else)."""
    for name, res in runs(src):
        if matches(res, chk) and not res["stderr"].strip():
            errors.append(f"{label} [{name}]: wrong variant also prints {res['stdout']!r}\n--- program ---\n{src}")


def fill(tpl, code):
    m = re.search(r"^([ ]*)\{\{CODE\}\}", tpl, re.M)
    ind = m.group(1)
    lines = code.split("\n")
    body = "\n".join([lines[0]] + [(ind + l) if l else l for l in lines[1:]])
    return tpl[:m.start()] + ind + body + tpl[m.end():]


def program(code, chk):
    if chk.get("tpl"):
        return fill(chk["tpl"], code)
    if code.startswith("use strict;"):
        return code + "\n"
    return HEAD + code + "\n"


def norm(s):
    return re.sub(r"[\s_'\"`]", "", s).lower()


def check_order_unique(e, chk):
    """Every other order of the lines must fail or print something else (local perl only).
    Orders that differ only in the indentation of equal lines (`}` and `    }`) count as the same order."""
    lines = e["lines"]
    if len(lines) > 7:
        return
    same = [l.strip() for l in lines]
    for perm in itertools.permutations(range(len(lines))):
        if [lines[i].strip() for i in perm] == same:
            continue
        res = run_local(program("\n".join(lines[i] for i in perm), chk))
        if matches(res, chk) and not res["stderr"].strip():
            errors.append(f"{e['id']}: another order also works: {[lines[i] for i in perm]}")


def check_blocks_distractors(e, chk):
    """Each distractor in place of each token must fail or print something else (local perl only).
    The final `;` is skipped: without it, the line would continue into the template code."""
    toks = e["tokens"]
    for d in e["distractors"]:
        for i in range(len(toks)):
            if toks[i] == ";":
                continue
            alt = toks[:i] + [d] + toks[i + 1:]
            res = run_local(program(" ".join(alt), chk))
            if matches(res, chk) and not res["stderr"].strip():
                errors.append(f"{e['id']}: distractor {d!r} at position {i} also works")


def check_exercise(e, chk):
    eid, kind = e["id"], e["type"]
    if kind in ("input", "choice", "true_false") and e.get("code"):
        code = chk.get("code", e["code"])
        expect(eid, program(code, chk), chk)
        if kind == "input" and norm(chk.get("out", "")) not in {norm(a) for a in e["answers"]}:
            errors.append(f"{eid}: expected output {chk.get('out')!r} is not among the answers {e['answers']}")
    elif kind == "gap":
        for i, opt in enumerate(e["options"]):
            src = program(e["code"].replace("___", opt), chk)
            if i == e["answer"]:
                expect(f"{eid} (right option)", src, chk)
            else:
                expect_wrong(f"{eid} (option {opt!r})", src, chk)
    elif kind == "bug":
        lines = list(e["lines"])
        expect_wrong(f"{eid} (as shown)", program("\n".join(lines), chk), chk)
        lines[e["bug"]] = chk["fix"]
        expect(f"{eid} (fixed)", program("\n".join(lines), chk), chk)
    elif kind == "order":
        expect(eid, program("\n".join(e["lines"]), chk), chk)
        check_order_unique(e, chk)
    elif kind == "blocks":
        expect(eid, program(" ".join(e["tokens"]), chk), chk)
        if chk.get("distractors"):
            check_blocks_distractors(e, chk)
    for j, x in enumerate(chk.get("extra", [])):
        expect(f"{eid} (extra {j + 1})", x["src"], x)


ONLINE = (f"wandbox {WANDBOX_PERL}", call_wandbox) if "--wandbox" in sys.argv else ("godbolt perl 5.44.0", call_godbolt)


def main():
    course = json.loads(COURSE.read_text())
    checks = json.loads(CHECKS.read_text())
    n_blocks = n_ex = 0
    for u in course["units"]:
        if args and u["id"] not in args:
            continue
        for i, b in enumerate(u["lesson"]):
            if b["type"] != "code":
                continue
            label = f"{u['id']}.lesson.{i}"
            if not b["code"].startswith("use strict;\nuse warnings;\n"):
                errors.append(f"{label}: does not start with use strict; use warnings;")
            expect(label, b["code"] + "\n", {"out": b.get("output", "")})
            n_blocks += 1
        for e in u["exercises"]:
            chk = checks.get(e["id"])
            if chk is None:
                if e.get("code") or e["type"] in ("gap", "bug", "order", "blocks"):
                    errors.append(f"{e['id']}: no check")
                continue
            check_exercise(e, chk)
            n_ex += 1
    print(f"lesson programs: {n_blocks}, exercises checked: {n_ex}, distinct programs: {len(stats['programs'])}, "
          f"local runs: {stats['local']}, new {ONLINE[0]} runs: {stats['online']}")
    for e in errors:
        print("ERROR", e)
    print("ALL PASSED" if not errors else f"{len(errors)} problem(s)")
    sys.exit(1 if errors else 0)


if __name__ == "__main__":
    main()
