#!/usr/bin/env python3
"""Runs the Elixir course code with a local `elixir` (1.17+) and checks it.

- every lesson code block: runs as a script; stdout == `output`, exit code 0, no warnings
- every exercise with an entry in checks.json (see README.md)
- formatting: every snippet is unchanged by Code.format_string!/1 (mix format)

Results are cached in .cache.json (key: sha256 of Elixir version + program).
Usage: python3 verify.py [unit-id ...]
"""
import hashlib, itertools, json, re, subprocess, sys, tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
COURSE = HERE.parent / "course.json"
CHECKS = HERE / "checks.json"
CACHE = HERE / ".cache.json"
TIMEOUT = 20
# limit the heap of every Erlang process (in words) so endless recursion stops early
ERL_FLAGS = "+hmax 20000000 +hmaxk true"

VERSION = subprocess.run(["elixir", "--version"], capture_output=True, text=True).stdout.strip().splitlines()[-1]
cache = json.loads(CACHE.read_text()) if CACHE.exists() else {}
runs = 0
tmpdir = Path(tempfile.mkdtemp(prefix="elixir-verify-"))


def run(src):
    """Returns (ok, stdout, stderr). ok: finished in time with exit code 0."""
    global runs
    key = hashlib.sha256((VERSION + "\0" + src).encode()).hexdigest()
    if key not in cache:
        f = tmpdir / "main.exs"
        f.write_text(src)
        try:
            p = subprocess.run(["elixir", "--erl", ERL_FLAGS, str(f)], capture_output=True, text=True,
                               timeout=TIMEOUT, cwd=tmpdir)
            res = {"ok": p.returncode == 0, "stdout": p.stdout.removesuffix("\n"), "stderr": p.stderr}
        except subprocess.TimeoutExpired:
            res = {"ok": False, "stdout": "", "stderr": "timeout"}
        cache[key] = res
        runs += 1
        if runs % 20 == 0:
            save()
    c = cache[key]
    return c["ok"], c["stdout"], c["stderr"]


def save():
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


errors, notes = [], []
fmt_items = []


def format_check():
    """Formats every snippet with Code.format_string!/1 (like `mix format`) and reports differences.
    `bug` and `order` lines cannot contain empty lines, so for them empty lines are ignored."""
    data = tmpdir / "fmt.json"
    data.write_text(json.dumps(fmt_items))
    script = tmpdir / "fmt.exs"
    script.write_text('''
strip = fn text -> text |> String.split("\\n") |> Enum.reject(&(&1 == "")) end

for [label, code, loose] <- JSON.decode!(File.read!(hd(System.argv()))) do
  try do
    formatted = IO.iodata_to_binary(Code.format_string!(code))
    same = if loose, do: strip.(formatted) == strip.(code), else: formatted == code
    if not same, do: IO.puts("DIFF #{label}:\\n#{formatted}")
  rescue
    e -> IO.puts("PARSE #{label}: #{Exception.message(e)}")
  end
end
''')
    p = subprocess.run(["elixir", str(script), str(data)], capture_output=True, text=True, timeout=120)
    if p.returncode != 0 or p.stdout.strip():
        errors.append("format differences:\n" + p.stdout + p.stderr)
    print(f"format: {len(fmt_items)} snippets checked")


def expect(label, src, chk, strict=False):
    ok, out, err = run(src)
    if chk.get("err"):
        if ok:
            errors.append(f"{label}: expected an error, got output {out!r}")
        return ok, out
    if not ok:
        errors.append(f"{label}: failed\n{err}\n--- program ---\n{src}")
    elif "out" in chk and out != chk["out"]:
        errors.append(f"{label}: output {out!r}, expected {chk['out']!r}")
    elif "warning" in err:
        (errors if strict else notes).append(f"{label}: warning\n{err}")
    return ok, out


def check_exercise(e, chk):
    eid, kind = e["id"], e["type"]
    tpl = chk.get("tpl", "{{CODE}}")

    def src_for(code):
        return fill(tpl, code)

    if kind in ("input", "choice", "true_false"):
        code = chk.get("code", e.get("code"))
        if code is not None:
            ok, out = expect(eid, src_for(code), chk, strict=True)
            if e.get("code"):
                fmt_items.append((eid, e["code"], False))
            if kind == "input" and ok and norm(out) not in {norm(a) for a in e["answers"]}:
                errors.append(f"{eid}: output {out!r} is not among the answers {e['answers']}")
    elif kind == "gap":
        for i, opt in enumerate(e["options"]):
            code = e["code"].replace("___", opt)
            if i == e["answer"]:
                expect(f"{eid} (right option)", src_for(code), chk, strict=True)
                fmt_items.append((eid, code, False))
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
        expect(f"{eid} (fixed)", src_for("\n".join(lines)), chk, strict=True)
        fmt_items.append((eid, "\n".join(lines), True))
    elif kind == "order":
        expect(eid, src_for("\n".join(e["lines"])), chk, strict=True)
        fmt_items.append((eid, "\n".join(e["lines"]), True))
        for perm in itertools.permutations(e["lines"]):
            if list(perm) == e["lines"]:
                continue
            ok, out, _ = run(src_for("\n".join(perm)))
            if ok and out == chk["out"]:
                errors.append(f"{eid}: another order also prints {out!r}: {perm}")
    elif kind == "blocks":
        toks = e["tokens"]
        expect(eid, src_for(" ".join(toks)), chk, strict=True)
        for d in e["distractors"]:
            for i in range(len(toks)):
                ok, out, _ = run(src_for(" ".join(toks[:i] + [d] + toks[i + 1:])))
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
            fmt_items.append((label, b["code"], False))
            if "output" in b:
                expect(label, b["code"] + "\n", {"out": b["output"]}, strict=True)
            else:
                ok, out, err = run(b["code"] + "\n")
                if not ok or out:
                    errors.append(f"{label}: has no output, but failed or printed {out!r}\n{err}")
            n_blocks += 1
        for e in u["exercises"]:
            chk = checks.get(e["id"])
            if chk is None:
                if e.get("code") or e["type"] in ("gap", "bug", "order", "blocks"):
                    errors.append(f"{e['id']}: no check")
                continue
            check_exercise(e, chk)
            n_ex += 1
    save()
    format_check()
    print(f"{VERSION}")
    print(f"lesson programs: {n_blocks}, exercises checked: {n_ex}, new runs: {runs}, cached runs: {len(cache)}")
    for n in notes:
        print("NOTE", n)
    for e in errors:
        print("ERROR", e)
    print("ALL PASSED" if not errors else f"{len(errors)} problem(s)")
    sys.exit(1 if errors else 0)


if __name__ == "__main__":
    main()
