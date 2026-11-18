#!/usr/bin/env python3
"""Runs the D course code on an online compiler (D 2.x) and checks it.

- every lesson code block (complete programs): compiles and runs; stdout == `output`
- every exercise with an entry in checks.json (see README.md)

Backends: Compiler Explorer (default, dmd 2.112.0) or Wandbox (ldc/dmd, when it is up).
Results are cached in .cache.json (key: backend + sha256 of the program).
Usage: python3 verify.py [--backend godbolt|wandbox] [unit-id ...]
       python3 verify.py [--backend ...] --run file.d   (runs one file, prints the result)
"""
import hashlib, json, re, sys, time, urllib.request
from pathlib import Path

HERE = Path(__file__).resolve().parent
COURSE = HERE.parent / "course.json"
CHECKS = HERE / "checks.json"
CACHE = HERE / ".cache.json"
GODBOLT = "https://godbolt.org/api/compiler/{}/compile"
GODBOLT_COMPILER = "dmd21120"  # dmd 2.112.0
WANDBOX = "https://wandbox.org/api/compile.json"
WANDBOX_COMPILER = "ldc-1.39.0"

cache = json.loads(CACHE.read_text()) if CACHE.exists() else {}
runs = 0
backend = "godbolt"
DEFAULT_TPL = "import std.stdio;\n\nvoid main()\n{\n    {{CODE}}\n}\n"


def post(url, body):
    req = urllib.request.Request(url, data=json.dumps(body).encode(),
                                 headers={"Content-Type": "application/json", "Accept": "application/json",
                                          "User-Agent": "curl/8.7.1"})
    return json.load(urllib.request.urlopen(req, timeout=120))


def lines(xs):
    return "\n".join(x.get("text", "") for x in xs or [])


def run_godbolt(src):
    r = post(GODBOLT.format(GODBOLT_COMPILER),
             {"source": src, "lang": "d", "allowStoreCodeDebug": True,
              "options": {"userArguments": "", "compilerOptions": {"executorRequest": True},
                          "filters": {"execute": True}, "tools": [], "libraries": []}})
    if not r.get("didExecute"):
        return {"ok": False, "stdout": "", "diag": lines(r.get("buildResult", {}).get("stderr"))[-2000:]}
    return {"ok": r.get("code") == 0, "stdout": lines(r.get("stdout")), "diag": lines(r.get("stderr"))[-2000:]}


def run_wandbox(src):
    r = post(WANDBOX, {"compiler": WANDBOX_COMPILER, "code": src})
    out = r.get("program_output", "")
    return {"ok": str(r.get("status")) == "0", "stdout": out[:-1] if out.endswith("\n") else out,
            "diag": (r.get("compiler_error", "") + r.get("program_error", ""))[-2000:]}


def run(src):
    """Returns (ok, stdout, diagnostics). ok: compiled and ran without an error."""
    global runs
    key = backend + ":" + hashlib.sha256(src.encode()).hexdigest()
    if key not in cache:
        for attempt in range(8):
            try:
                res = run_godbolt(src) if backend == "godbolt" else run_wandbox(src)
                break
            except Exception as e:  # noqa: BLE001
                print("  retry:", e, file=sys.stderr)
                time.sleep(min(60, 10 * (attempt + 1)))
        else:
            raise SystemExit("API not reachable for this program:\n" + src)
        time.sleep(1)
        cache[key] = res
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


def norm(s):
    return re.sub(r"[\s_'\"`]", "", s).lower()


errors = []


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
    return ok, out


def check_exercise(e, chk):
    eid, kind = e["id"], e["type"]

    def src_for(code):
        if "void main" in code:
            dflt = "{{CODE}}\n" if re.search(r"^import ", code, re.M) else "import std.stdio;\n\n{{CODE}}\n"
        else:
            dflt = DEFAULT_TPL
        tpl = chk.get("tpl", dflt)
        return fill(tpl, code)

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
        for j, alt in enumerate(chk.get("alt_orders", [])):
            ok, out, _ = run(src_for("\n".join(e["lines"][k] for k in alt)))
            if ok and out == chk["out"]:
                errors.append(f"{eid}: other order {alt} also prints {out!r}")
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
    global backend
    args = sys.argv[1:]
    if args[:1] == ["--backend"]:
        backend, args = args[1], args[2:]
    if args[:1] == ["--run"]:
        ok, out, diag = run(Path(args[1]).read_text())
        print("OK" if ok else "FAILED", "\n" + out + ("" if ok else "\n" + diag))
        return
    course = json.loads(COURSE.read_text())
    checks = json.loads(CHECKS.read_text())
    only = set(args)
    n_blocks = n_ex = 0
    for u in course["units"]:
        if only and u["id"] not in only:
            continue
        for i, b in enumerate(u["lesson"]):
            if b["type"] != "code":
                continue
            label = f"{u['id']}.lesson.{i}"
            ok, out = expect(label, b["code"] + "\n", {"out": b["output"]} if "output" in b else {})
            if "output" not in b and ok and out:
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
    print(f"backend: {backend}, lesson programs: {n_blocks}, exercises checked: {n_ex}, "
          f"new runs: {runs}, cached results: {len(cache)}")
    for e in errors:
        print("ERROR", e)
    print("ALL PASSED" if not errors else f"{len(errors)} problem(s)")
    sys.exit(1 if errors else 0)


if __name__ == "__main__":
    main()
