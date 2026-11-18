#!/usr/bin/env python3
"""Runs the Dart course code on the Compiler Explorer API (Dart 3.13.6) and checks it.

- every lesson code block: a complete program (void main); compiles; stdout == `output`
- every exercise with an entry in checks.json (see README.md)

Results are cached in .cache.json (key: sha256 of the program).
Usage: python3 verify.py [unit-id ...]
"""
import hashlib, json, re, sys, time, urllib.request
from pathlib import Path

HERE = Path(__file__).resolve().parent
COURSE = HERE.parent / "course.json"
CHECKS = HERE / "checks.json"
CACHE = HERE / ".cache.json"
COMPILER = "dart3136"
API = f"https://godbolt.org/api/compiler/{COMPILER}/compile"

cache = json.loads(CACHE.read_text()) if CACHE.exists() else {}
runs = 0


def run(src):
    """Returns (ok, stdout, diagnostics). ok: compiled and exit code 0."""
    global runs
    key = hashlib.sha256((COMPILER + src).encode()).hexdigest()
    if key not in cache:
        body = {"source": src, "options": {"userArguments": "", "compilerOptions": {"executorRequest": True},
                "filters": {"execute": True}, "tools": [], "libraries": []}, "lang": "dart", "allowStoreCodeDebug": True}
        for attempt in range(4):
            try:
                req = urllib.request.Request(API, data=json.dumps(body).encode(),
                                             headers={"Content-Type": "application/json", "Accept": "application/json"})
                r = json.load(urllib.request.urlopen(req, timeout=120))
                break
            except Exception as e:  # noqa: BLE001
                print("  retry:", e, file=sys.stderr)
                time.sleep(5 * (attempt + 1))
        else:
            raise SystemExit("API not reachable")
        time.sleep(1)
        build = r.get("buildResult", {})
        diag = "\n".join(x["text"] for x in build.get("stderr", []) + build.get("stdout", []) + r.get("stderr", []))
        cache[key] = {"ok": bool(r.get("didExecute")) and r.get("code") == 0,
                      "stdout": "\n".join(x["text"] for x in r.get("stdout", [])), "diag": diag}
        tmp = CACHE.with_suffix(".tmp")
        tmp.write_text(json.dumps(cache, indent=0))
        tmp.replace(CACHE)
        runs += 1
    c = cache[key]
    return c["ok"], c["stdout"], c["diag"]


def fill(tpl, code):
    """Puts code into the template at {{CODE}}, indented like the marker."""
    m = re.search(r"^([ ]*)\{\{CODE\}\}", tpl, re.M)
    ind = m.group(1)
    lines = code.split("\n")
    body = "\n".join([lines[0]] + [(ind + l) if l else l for l in lines[1:]])
    return tpl[:m.start()] + ind + body + tpl[m.end():]


def default_tpl(code):
    if re.search(r"^void main\(\)", code, re.M):
        return "{{CODE}}\n"
    return "void main() {\n  {{CODE}}\n}\n"


def norm(s):
    return re.sub(r"[\s_'\"`]", "", s).lower()


errors = []
notes = []


def expect(label, src, chk):
    ok, out, diag = run(src)
    if chk.get("err"):
        if ok:
            errors.append(f"{label}: expected a compile or run error, got output {out!r}")
        return ok, out
    if not ok:
        errors.append(f"{label}: failed\n{diag}\n--- program ---\n{src}")
    elif "out" in chk and out != chk["out"]:
        errors.append(f"{label}: output {out!r}, expected {chk['out']!r}")
    elif "\n".join(l for l in diag.split("\n") if not l.startswith("Generated:")).strip():
        notes.append(f"{label}: diagnostics:\n{diag}")
    return ok, out


def check_exercise(e, chk):
    eid, kind = e["id"], e["type"]
    tpl = chk.get("tpl")

    def src_for(code):
        return fill(tpl or default_tpl(code), code)

    if kind in ("input", "choice", "true_false"):
        code = chk.get("code", e.get("code"))
        if code is not None:
            ok, out = expect(eid, src_for(code), chk)
            if kind == "input" and ok and norm(out) not in {norm(a) for a in e["answers"]}:
                errors.append(f"{eid}: output {out!r} is not among the answers {e['answers']}")
    elif kind == "gap":
        for i, opt in enumerate(e["options"]):
            code = e["code"].replace("___", opt)
            if i == e["answer"]:
                expect(f"{eid} (right option)", src_for(code), chk)
            else:
                ok, out, _ = run(src_for(code))
                if ok and out == chk["out"]:
                    errors.append(f"{eid}: wrong option {opt!r} also prints {out!r}")
    elif kind == "bug":
        lines = list(e["lines"])
        ok, out, _ = run(src_for("\n".join(lines)))
        if ok and out == chk["out"]:
            errors.append(f"{eid}: buggy code already prints the expected output")
        lines[e["bug"]] = chk["fix"]
        expect(f"{eid} (fixed)", src_for("\n".join(lines)), chk)
    elif kind == "order":
        expect(eid, src_for("\n".join(e["lines"])), chk)
    elif kind == "blocks":
        expect(eid, src_for(" ".join(e["tokens"])), chk)
        if chk.get("distractors"):
            for d in e["distractors"]:
                for i in range(len(e["tokens"])):
                    alt = e["tokens"][:i] + [d] + e["tokens"][i + 1:]
                    ok, out, _ = run(src_for(" ".join(alt)))
                    if ok and out == chk["out"]:
                        errors.append(f"{eid}: distractor {d!r} at position {i} also works")
    for j, x in enumerate(chk.get("extra", [])):
        expect(f"{eid} (extra {j + 1})", x["src"], x)


def main():
    course = json.loads(COURSE.read_text())
    checks = json.loads(CHECKS.read_text())
    only = set(sys.argv[1:])
    n_blocks = n_ex = 0
    for u in course["units"]:
        if only and u["id"] not in only:
            continue
        for i, b in enumerate(u["lesson"]):
            if b["type"] != "code":
                continue
            label = f"{u['id']}.lesson.{i}"
            if not re.search(r"^void main\(\)", b["code"], re.M):
                errors.append(f"{label}: not a complete program")
                continue
            ok, out = expect(label, b["code"] + "\n", {"out": b["output"]} if "output" in b else {})
            if "output" not in b and out:
                errors.append(f"{label}: prints {out!r} but has no output")
            n_blocks += 1
        for e in u["exercises"]:
            chk = checks.get(e["id"])
            if chk is None:
                if e.get("code") or e["type"] in ("gap", "bug", "order", "blocks"):
                    errors.append(f"{e['id']}: no check")
                continue
            check_exercise(e, chk)
            n_ex += 1
    for n in notes:
        print("NOTE", n)
    print(f"lesson programs: {n_blocks}, exercises checked: {n_ex}, new API runs: {runs}, cached runs: {len(cache)}")
    for e in errors:
        print("ERROR", e)
    print("ALL PASSED" if not errors else f"{len(errors)} problem(s)")
    sys.exit(1 if errors else 0)


if __name__ == "__main__":
    main()
