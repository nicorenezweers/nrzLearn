#!/usr/bin/env python3
"""Runs the Go course code on the Compiler Explorer API (Go 1.26) and checks it.

- every lesson code block (complete programs): compiles; stdout == `output`
- every exercise with an entry in checks.json (see README.md)
- gofmt: all code is formatted like gofmt (tabs shown as 4 spaces)

Results are cached in .cache.json (key: sha256 of the program).
Usage: python3 verify.py [unit-id ...]
"""
import hashlib, json, re, sys, time, urllib.request
from pathlib import Path

HERE = Path(__file__).resolve().parent
COURSE = HERE.parent / "course.json"
CHECKS = HERE / "checks.json"
CACHE = HERE / ".cache.json"
API = "https://godbolt.org/api/compiler/gl1260/compile"

cache = json.loads(CACHE.read_text()) if CACHE.exists() else {}
runs = 0


def run(src):
    """Returns (ok, stdout, diagnostics). ok: compiled and exit code 0."""
    global runs
    key = hashlib.sha256(src.encode()).hexdigest()
    if key not in cache:
        body = {"source": src, "options": {"userArguments": "", "compilerOptions": {"executorRequest": True},
                "filters": {"execute": True}, "tools": [], "libraries": []}, "lang": "go", "allowStoreCodeDebug": True}
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
        diag = "\n".join(x["text"] for x in build.get("stderr", []) + r.get("stderr", []))
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
    imps = ["fmt"] + [p for p in ("errors", "strconv") if p + "." in code]
    imp = 'import "fmt"' if len(imps) == 1 else "import (\n" + "".join(f'    "{i}"\n' for i in sorted(imps)) + ")"
    if re.search(r"^(func|type|var) ", code, re.M) and "func main()" in code:
        return f"package main\n\n{imp}\n\n{{{{CODE}}}}\n"
    return f"package main\n\n{imp}\n\nfunc main() {{\n    {{{{CODE}}}}\n}}\n"


def norm(s):
    return re.sub(r"[\s_'\"`]", "", s).lower()


errors = []
gofmt_items = []
no_blank_lines = set()


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
    tpl = chk.get("tpl")
    def src_for(code):
        return fill(tpl or default_tpl(code), code)
    if kind in ("input", "choice", "true_false"):
        code = chk.get("code", e.get("code"))
        if code is not None:
            ok, out = expect(eid, src_for(code), chk)
            if kind == "input" and ok and norm(out) not in {norm(a) for a in e["answers"]}:
                errors.append(f"{eid}: output {out!r} is not among the answers {e['answers']}")
            if e.get("code") and not chk.get("err"):
                gofmt_items.append((eid, e["code"]))
    elif kind == "gap":
        for i, opt in enumerate(e["options"]):
            code = e["code"].replace("___", opt)
            if i == e["answer"]:
                expect(f"{eid} (right option)", src_for(code), chk)
                gofmt_items.append((eid, code))
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
        gofmt_items.append((eid, "\n".join(lines)))
        no_blank_lines.add(eid)
    elif kind == "order":
        code = "\n".join(e["lines"])
        expect(eid, src_for(code), chk)
        gofmt_items.append((eid, code))
        no_blank_lines.add(eid)
    elif kind == "blocks":
        code = " ".join(e["tokens"])
        expect(eid, src_for(code), chk)
        for d in e["distractors"]:
            for i in range(len(e["tokens"])):
                alt = e["tokens"][:i] + [d] + e["tokens"][i + 1:]
                ok, out, _ = run(src_for(" ".join(alt))) if chk.get("distractors") else (False, "", "")
                if ok and out == chk["out"]:
                    errors.append(f"{eid}: distractor {d!r} at position {i} also works")
    for j, x in enumerate(chk.get("extra", [])):
        expect(f"{eid} (extra {j + 1})", x["src"], x)


def gofmt_check():
    """One program that formats every snippet with go/format and reports differences.
    `bug` and `order` lines cannot contain empty lines, so for them empty lines are ignored."""
    lit = lambda s: json.dumps(s, ensure_ascii=False)
    entries = ""
    for label, c in gofmt_items:
        tabbed = "\n".join(re.sub(r"^((?:    )+)", lambda m: "\t" * (len(m.group(1)) // 4), l) for l in c.split("\n"))
        loose = "y" if label in no_blank_lines else ""
        entries += f"    {{{lit(label)}, {lit(tabbed)}, {lit(loose)}}},\n"
    src = """package main

import (
    "fmt"
    "go/format"
    "strings"
)

var items = [][3]string{
""" + entries + """}

func norm(s string, loose bool) string {
    s = strings.TrimSuffix(s, "\\n")
    if !loose {
        return s
    }
    var keep []string
    for _, l := range strings.Split(s, "\\n") {
        if l != "" {
            keep = append(keep, l)
        }
    }
    return strings.Join(keep, "\\n")
}

func main() {
    for _, it := range items {
        out, err := format.Source([]byte(it[1]))
        loose := it[2] != ""
        if err != nil {
            fmt.Printf("PARSE %s: %v\\n", it[0], err)
        } else if norm(string(out), loose) != norm(it[1], loose) {
            fmt.Printf("DIFF %s:\\n%s\\n", it[0], out)
        }
    }
    fmt.Println("checked", len(items))
}
"""
    ok, out, diag = run(src)
    if not ok:
        errors.append("gofmt program failed:\n" + diag)
        return
    for line in out.split("\n"):
        if not line.startswith("checked"):
            print("gofmt:", line)
    if any(l.startswith(("DIFF", "PARSE")) for l in out.split("\n")):
        errors.append("gofmt differences (see above)")
    print("gofmt:", out.split("\n")[-1], "snippets")


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
            gofmt_items.append((label, b["code"]))
            if not b["code"].startswith("package main"):
                errors.append(f"{label}: not a complete program")
                continue
            expect(label, b["code"] + "\n", {"out": b["output"]} if "output" in b else {})
            if "output" not in b:
                ok, out, _ = run(b["code"] + "\n")
                if out:
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
    gofmt_check()
    print(f"lesson programs: {n_blocks}, exercises checked: {n_ex}, new API runs: {runs}, cached runs: {len(cache)}")
    for e in errors:
        print("ERROR", e)
    print("ALL PASSED" if not errors else f"{len(errors)} problem(s)")
    sys.exit(1 if errors else 0)


if __name__ == "__main__":
    main()
