#!/usr/bin/env python3
"""Runs the Erlang course code with a local `escript` and checks it.

- every lesson code block (complete programs): runs; stdout == `output`
- every exercise with an entry in checks.json (see README.md)
- order exercises: no other order of the lines gives the same output

Results are cached in .cache.json (key: sha256 of OTP version + program).
Usage: python3 verify.py [unit-id ...]
       python3 verify.py --wandbox    (also run all lesson blocks on Wandbox, Erlang 26)
"""
import hashlib, itertools, json, re, subprocess, sys, tempfile, time, urllib.request
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

HERE = Path(__file__).resolve().parent
COURSE = HERE.parent / "course.json"
CHECKS = HERE / "checks.json"
CACHE = HERE / ".cache.json"
HEAD = "#!/usr/bin/env escript\n-module(t).\n-export([main/1]).\n\n"
WHAT = "What does this print?"

OTP = subprocess.run(["erl", "-noshell", "-eval",
                      'io:format("~s ~s", [erlang:system_info(otp_release), erlang:system_info(version)]), halt().'],
                     capture_output=True, text=True).stdout.strip()
warnings = {}
cache = json.loads(CACHE.read_text()) if CACHE.exists() else {}
runs = 0


def run(src, timeout=10):
    """Returns (ok, stdout, stderr). ok: exit code 0 within the time limit."""
    global runs
    key = hashlib.sha256((OTP + "\0" + src).encode()).hexdigest()
    if key not in cache:
        with tempfile.TemporaryDirectory() as d:
            f = Path(d) / "t.erl"
            f.write_text(src + "\n")
            try:
                p = subprocess.run(["escript", str(f)], capture_output=True, text=True, timeout=timeout, cwd=d)
                res = {"ok": p.returncode == 0, "stdout": p.stdout, "diag": p.stderr[-2000:]}
            except subprocess.TimeoutExpired:
                res = {"ok": False, "stdout": "", "diag": f"timeout after {timeout}s"}
        res["stdout"], res["warn"] = split_warnings(res["stdout"], str(f))
        out = res["stdout"]
        res["stdout"] = out[:-1] if out.endswith("\n") else out
        cache[key] = res
        runs += 1
    c = cache[key]
    if c["warn"]:
        warnings[src] = c["warn"]
    return c["ok"], c["stdout"], c["diag"]


def split_warnings(out, path):
    """escript prints compiler warnings to stdout; returns (stdout without them, warnings)."""
    warn, keep, lines, i = [], [], out.split("\n"), 0
    while i < len(lines):
        if lines[i].startswith(path + ":") and ": Warning:" in lines[i]:
            warn.append(lines[i][len(path) + 1:])
            i += 1
            while i < len(lines) and lines[i].startswith("%"):
                i += 1
            if i < len(lines) and lines[i] == "":
                i += 1
        else:
            keep.append(lines[i])
            i += 1
    return "\n".join(keep), warn


def save_cache():
    tmp = CACHE.with_suffix(".tmp")
    tmp.write_text(json.dumps(cache, indent=0))
    tmp.replace(CACHE)


def fill(tpl, code):
    """Puts code into the template at {{CODE}}; extra lines get the marker's indentation."""
    m = re.search(r"^([ ]*)\{\{CODE\}\}", tpl, re.M)
    if not m:
        return tpl.replace("{{CODE}}", code)
    ind = m.group(1)
    lines = code.split("\n")
    body = "\n".join([lines[0]] + [ind + l if l else l for l in lines[1:]])
    return tpl.replace("{{CODE}}", body)


def complete(code, chk=None):
    """Turns a snippet into a complete escript."""
    chk = chk or {}
    if "tpl" in chk:
        code = fill(chk["tpl"], code)
    if code.startswith("#!"):
        return code
    if "-module(" in code:
        return "#!/usr/bin/env escript\n" + code
    if re.search(r"^main\(", code, re.M):
        return HEAD + code
    if "call" in chk:
        return HEAD + 'main(_) ->\n    io:format("~p~n", [' + chk["call"] + "]).\n\n" + code
    lines = code.split("\n")
    return HEAD + "main(_) ->\n" + "\n".join("    " + l if l else l for l in lines) + "."


def norm(s):
    return re.sub(r"[\s_\"'`]", "", s).lower()


errors = []
checked = 0


def fail(where, msg):
    errors.append(f"{where}: {msg}")
    print(f"  FAIL {where}: {msg}")


def expect(where, src, chk, timeout=10):
    """Runs src; checks against chk['out'] or chk['err']. Returns stdout."""
    global checked
    checked += 1
    ok, out, diag = run(src, timeout)
    if chk.get("err"):
        if ok:
            fail(where, f"expected an error, but it ran and printed {out!r}")
    elif not ok:
        fail(where, f"failed:\n{diag}\n--- program ---\n{src}")
    elif out != chk["out"]:
        fail(where, f"printed {out!r}, expected {chk['out']!r}")
    return out


def must_differ(where, src, good, timeout=10):
    global checked
    checked += 1
    ok, out, _ = run(src, timeout)
    if ok and out == good:
        fail(where, f"wrong variant also prints {good!r}:\n{src}")


def lint(where, code):
    for i, l in enumerate(code.split("\n")):
        if "\t" in l or l != l.rstrip():
            fail(where, f"line {i + 1}: tab or trailing space")
        if re.search(r"%\s*[A-Za-z]{3,}", l) and "~" not in l:
            fail(where, f"line {i + 1}: looks like a natural-language comment")


def check_unit(u, checks):
    uid = u["id"]
    print(f"[{uid}]")
    for i, b in enumerate(u["lesson"]):
        if b["type"] != "code":
            continue
        where = f"{uid}.lesson.{i}"
        lint(where, b["code"])
        if not b["code"].startswith("#!/usr/bin/env escript\n"):
            fail(where, "lesson code must be a complete escript")
        expect(where, b["code"], {"out": b.get("output", "")})
        if b["code"] in warnings:
            fail(where, f"compiler warnings: {warnings[b['code']]}")
    for e in u["exercises"]:
        eid, t = e["id"], e["type"]
        for k in ("code",):
            if k in e:
                lint(eid, e[k])
        chk = checks.get(eid)
        if chk is None:
            print(f"  (no check: {eid}, {t})")
            continue
        for j, x in enumerate(chk.get("extra", [])):
            expect(f"{eid}.extra.{j}", complete(x["src"], x), x)
        if "out" not in chk and not chk.get("err"):
            continue
        if t in ("input", "choice", "true_false"):
            code = chk.get("code", e.get("code"))
            out = expect(eid, complete(code, chk), chk)
            if t == "input" and not chk.get("err") and norm(out) not in [norm(a) for a in e["answers"]]:
                fail(eid, f"output {out!r} is not among the answers {e['answers']}")
            if t == "choice" and e.get("prompt") == WHAT and not chk.get("err"):
                opt = e["options"][e["answer"]]
                if opt.startswith("`") and opt.endswith("`") and opt[1:-1] != out:
                    fail(eid, f"right option {opt} differs from output {out!r}")
        elif t == "gap":
            for j, opt in enumerate(e["options"]):
                src = complete(e["code"].replace("___", opt), chk)
                if j == e["answer"]:
                    expect(eid, src, chk)
                else:
                    must_differ(f"{eid} option {opt!r}", src, chk["out"])
        elif t == "bug":
            lines = e["lines"]
            must_differ(f"{eid} as shown", complete("\n".join(lines), chk), chk["out"], chk.get("timeout", 5))
            fixed = lines[:e["bug"]] + [chk["fix"]] + lines[e["bug"] + 1:]
            expect(f"{eid} fixed", complete("\n".join(fixed), chk), chk)
        elif t == "order":
            lines = e["lines"]
            expect(eid, complete("\n".join(lines), chk), chk)
            if len(lines) <= 6:
                perms = [p for p in itertools.permutations(lines) if list(p) != lines]
                srcs = [complete("\n".join(p), chk) for p in perms]
                with ThreadPoolExecutor(8) as ex:
                    res = list(ex.map(lambda s: run(s, 4), srcs))
                for p, (ok, out, _) in zip(perms, res):
                    if ok and out == chk["out"]:
                        fail(eid, f"another order also works: {list(p)}")
                global checked
                checked += len(perms)
        elif t == "blocks":
            expect(eid, complete(" ".join(e["tokens"]), chk), chk)
        save_cache()


def wandbox(course):
    """Runs every lesson block on Wandbox (Erlang 26) and compares the output."""
    for u in course["units"]:
        for i, b in enumerate(u["lesson"]):
            if b["type"] != "code":
                continue
            body = json.dumps({"compiler": "erlang-26.2.5.3", "code": b["code"] + "\n"}).encode()
            req = urllib.request.Request("https://wandbox.org/api/compile.json", data=body,
                                         headers={"Content-Type": "application/json", "User-Agent": "course-verify"})
            try:
                r = json.load(urllib.request.urlopen(req, timeout=60))
            except Exception as ex:  # noqa: BLE001
                raise SystemExit(f"Wandbox not reachable: {ex}")
            out = r.get("program_output", "").removesuffix("\n")
            mark = "ok" if out == b.get("output", "") else "FAIL"
            print(f"  wandbox {mark} {u['id']}.lesson.{i}")
            if mark == "FAIL":
                errors.append(f"wandbox {u['id']}.lesson.{i}: {r}")
            time.sleep(1)


def main():
    course = json.loads(COURSE.read_text())
    checks = json.loads(CHECKS.read_text())
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    print(f"Erlang/OTP {OTP}")
    for u in course["units"]:
        if not args or u["id"] in args:
            check_unit(u, checks)
    if "--wandbox" in sys.argv:
        wandbox(course)
    save_cache()
    print(f"\n{checked} checks, {runs} new runs, {len(errors)} errors")
    sys.exit(1 if errors else 0)


if __name__ == "__main__":
    main()
