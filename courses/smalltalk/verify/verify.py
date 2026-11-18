#!/usr/bin/env python3
"""Run every Smalltalk snippet of the course with GNU Smalltalk (gst) and compare."""
import hashlib, itertools, json, os, re, subprocess, sys, tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
COURSE = os.path.join(HERE, "..", "course.json")
CHECKS = os.path.join(HERE, "checks.json")
CACHE = os.path.join(HERE, ".cache.json")
GST = os.environ.get("GST", "gst")

cache = json.load(open(CACHE)) if os.path.exists(CACHE) else {}
runs = 0
fails = []


def gst_version():
    return subprocess.run([GST, "--version"], capture_output=True, text=True).stdout.splitlines()[0]


ERR_RE = re.compile(r"^(Object: .* error: |Error class|.*doesNotUnderstand)", re.M)


def run(src):
    """Return (stdout without final newline, error flag)."""
    global runs
    key = hashlib.sha256((gst_version_str + "\0" + src).encode()).hexdigest()
    if key in cache:
        return cache[key]["out"], cache[key]["err"]
    with tempfile.TemporaryDirectory() as d:
        path = os.path.join(d, "snippet.st")
        with open(path, "w") as f:
            f.write(src + "\n")
        try:
            p = subprocess.run([GST, path], capture_output=True, text=True, timeout=20, cwd=d)
            out, stderr = p.stdout, p.stderr
        except subprocess.TimeoutExpired:
            out, stderr = "", "timeout"
    runs += 1
    stderr = "\n".join(l for l in stderr.splitlines() if "garbage collection" not in l)
    out = "\n".join(l for l in out.split("\n") if "garbage collection" not in l)
    err = bool(stderr.strip()) or bool(ERR_RE.search(out))
    if out.endswith("\n"):
        out = out[:-1]
    cache[key] = {"out": out, "err": err}
    return out, err


DECL_RE = re.compile(r"^\|[^|]*\|$")


def norm(s):
    return re.sub(r"[\s_'\"`]", "", s).lower()


def fill(tpl, code):
    return tpl.replace("{{CODE}}", code)


def fail(eid, msg):
    fails.append(f"{eid}: {msg}")


def expect(eid, src, out=None, err=False, what=""):
    got, e = run(src)
    if err:
        if not e:
            fail(eid, f"{what} should fail but printed {got!r}")
        return got
    if e:
        fail(eid, f"{what} failed:\n{got}")
    elif out is not None and got != out:
        fail(eid, f"{what} printed {got!r}, expected {out!r}")
    return got


def wrong(eid, src, out, what):
    got, e = run(src)
    if not e and got == out:
        fail(eid, f"{what} gives the right result too ({got!r})")


def strip_ticks(s):
    return s[1:-1] if s.startswith("`") and s.endswith("`") else s


def main(only):
    course = json.load(open(COURSE))
    checks = json.load(open(CHECKS))
    used = set()
    n_lesson = n_ex = 0
    if not only:
        used.add("_lesson")
        for x in checks.get("_lesson", {}).get("extra", []):
            expect("lesson claim", x["src"], x.get("out"), x.get("err", False), "extra program")
    for unit in course["units"]:
        if only and unit["id"] not in only:
            continue
        for i, b in enumerate(unit["lesson"]):
            if b["type"] != "code":
                continue
            n_lesson += 1
            expect(f"{unit['id']} lesson block {i}", b["code"], b.get("output", ""), what="lesson code")
        for ex in unit["exercises"]:
            eid, t = ex["id"], ex["type"]
            c = checks.get(eid, {})
            if c:
                used.add(eid)
            tpl = c.get("tpl", "{{CODE}}")
            checked = False
            for x in c.get("extra", []):
                expect(eid, x["src"], x.get("out"), x.get("err", False), "extra program")
                checked = True
            if t == "input":
                got = expect(eid, fill(tpl, c.get("code", ex["code"])), c.get("out", ex["answers"][0]), what="code")
                if norm(got) not in [norm(a) for a in ex["answers"]]:
                    fail(eid, f"output {got!r} not among answers")
                checked = True
            elif t in ("choice", "true_false") and (ex.get("code") or "code" in c):
                src = fill(tpl, c.get("code", ex.get("code")))
                if "err" in c:
                    expect(eid, src, err=True, what="code")
                elif t == "choice" and "out" not in c:
                    right = strip_ticks(ex["options"][ex["answer"]])
                    got = expect(eid, src, right, what="code")
                    for k, o in enumerate(ex["options"]):
                        if k != ex["answer"] and strip_ticks(o) == got:
                            fail(eid, f"wrong option {o} equals the output")
                else:
                    expect(eid, src, c["out"], what="code")
                checked = True
            elif t == "gap":
                for k, o in enumerate(ex["options"]):
                    src = fill(tpl, ex["code"].replace("___", o))
                    if k == ex["answer"]:
                        expect(eid, src, c["out"], what="right option")
                    else:
                        wrong(eid, src, c["out"], f"wrong option {o!r}")
                checked = True
            elif t == "bug":
                lines = list(ex["lines"])
                wrong(eid, fill(tpl, "\n".join(lines)), c["out"], "code with the bug")
                lines[ex["bug"]] = c["fix"]
                expect(eid, fill(tpl, "\n".join(lines)), c["out"], what="fixed code")
                checked = True
            elif t == "order":
                lines = ex["lines"]
                expect(eid, fill(tpl, "\n".join(lines)), c["out"], what="right order")
                n = len(lines)
                if n <= 5:
                    perms = itertools.permutations(range(n))
                else:
                    perms = []
                    for a, b in itertools.combinations(range(n), 2):
                        p = list(range(n)); p[a], p[b] = p[b], p[a]; perms.append(p)
                decl = [k for k, l in enumerate(lines) if DECL_RE.match(l)]
                for p in perms:
                    if list(p) == list(range(n)):
                        continue
                    if decl == [0] and p[0] != 0:
                        continue
                    wrong(eid, fill(tpl, "\n".join(lines[k] for k in p)), c["out"], f"order {list(p)}")
                checked = True
            elif t == "blocks":
                expect(eid, fill(tpl, " ".join(ex["tokens"])), c["out"], what="tokens")
                checked = True
            if checked:
                n_ex += 1
    for k in checks:
        if k not in used and not only:
            fail(k, "check without matching exercise")
    json.dump(cache, open(CACHE, "w"), indent=0)
    print(f"{gst_version_str}: {n_lesson} lesson blocks, {n_ex} exercises checked, {runs} new gst runs ({len(cache)} cached)")
    if fails:
        print(f"{len(fails)} FAILURES:")
        for f in fails:
            print(" -", f)
        sys.exit(1)
    print("ALL PASSED")


if __name__ == "__main__":
    gst_version_str = gst_version()
    main(set(sys.argv[1:]))
