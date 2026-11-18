#!/usr/bin/env python3
"""Checks the Assembly course: runs the code and compares the results.

Default backend: Compiler Explorer (NASM 3.01, assembled, linked with ld and run
on real x86-64 Linux). `--local` uses emu.py (nasm + Unicorn) instead.
The order-uniqueness check always runs locally (it needs many runs).
Results of Compiler Explorer are cached in .cache.json (key: sha256 of the program).

Usage: python3 verify.py [--local] [--no-perm] [--compiler=nasm21601] [unit-id ...]
See README.md for the format of checks.json.
"""
import hashlib, itertools, json, re, sys, time, urllib.request
from pathlib import Path

HERE = Path(__file__).resolve().parent
COURSE = HERE.parent / "course.json"
CHECKS = HERE / "checks.json"
CACHE = HERE / ".cache.json"
COMPILER = next((a.split("=", 1)[1] for a in sys.argv if a.startswith("--compiler=")), "nasm301")
API = f"https://godbolt.org/api/compiler/{COMPILER}/compile"

args = [a for a in sys.argv[1:] if not a.startswith("--")]
LOCAL = "--local" in sys.argv
PERM = "--no-perm" not in sys.argv
cache = json.loads(CACHE.read_text()) if CACHE.exists() else {}
runs = {"ce": 0, "local": 0, "cached": 0}
_emu = None


def emu():
    global _emu
    if _emu is None:
        sys.path.insert(0, str(HERE))
        import emu as e
        _emu = e
    return _emu


def run_ce(src):
    key = hashlib.sha256((src if COMPILER == "nasm301" else COMPILER + "\n" + src).encode()).hexdigest()
    if key in cache:
        runs["cached"] += 1
        return cache[key]
    body = {"source": src, "options": {"userArguments": "", "compilerOptions": {"executorRequest": True},
            "filters": {"execute": True}, "tools": [], "libraries": []}, "lang": "assembly", "allowStoreCodeDebug": True}
    for attempt in range(5):
        try:
            req = urllib.request.Request(API, data=json.dumps(body).encode(),
                                         headers={"Content-Type": "application/json", "Accept": "application/json"})
            r = json.load(urllib.request.urlopen(req, timeout=120))
            break
        except Exception as e:  # noqa: BLE001
            print("  retry:", e, file=sys.stderr)
            time.sleep(5 * (attempt + 1))
    else:
        raise SystemExit("Compiler Explorer not reachable")
    time.sleep(1)
    build = r.get("buildResult", {})
    asm_ok = build.get("code") == 0
    res = {"asm": asm_ok, "ran": bool(r.get("didExecute")), "code": r.get("code"),
           "stdout": "\n".join(x["text"] for x in r.get("stdout", [])),
           "diag": "\n".join(x["text"] for x in build.get("stderr", []) + r.get("stderr", []))}
    cache[key] = res
    tmp = CACHE.with_suffix(".tmp")
    tmp.write_text(json.dumps(cache, indent=0))
    tmp.replace(CACHE)
    runs["ce"] += 1
    return res


def run_local(src):
    runs["local"] += 1
    return emu().run(src)


def run(src):
    return run_local(src) if LOCAL else run_ce(src)


def ok(res):
    return res["asm"] and res["ran"] and res["code"] == 0


# --- program building --------------------------------------------------------

SHOW_LIB = """
section .bss
    __dump resq 8
    __buf resb 40
section .text
__show:
    xor r12, r12
__show_next:
    cmp r12, r13
    jge __show_end
    mov rax, [__dump+r12*8]
    lea rdi, [__buf+39]
    mov byte [rdi], 32
    mov rbx, r12
    inc rbx
    cmp rbx, r13
    jne __show_sep
    mov byte [rdi], 10
__show_sep:
    xor r8, r8
    cmp r15, 0
    je __show_conv
    test rax, rax
    jns __show_conv
    neg rax
    mov r8, 1
__show_conv:
    mov rcx, 10
    cmp r14, 0
    je __show_digit
    mov rcx, 16
__show_digit:
    xor rdx, rdx
    div rcx
    add dl, '0'
    cmp dl, '9'
    jbe __show_put
    add dl, 39
__show_put:
    dec rdi
    mov [rdi], dl
    test rax, rax
    jnz __show_digit
    cmp r14, 0
    je __show_minus
    dec rdi
    mov byte [rdi], 'x'
    dec rdi
    mov byte [rdi], '0'
__show_minus:
    cmp r8, 0
    je __show_write
    dec rdi
    mov byte [rdi], '-'
__show_write:
    lea rdx, [__buf+40]
    sub rdx, rdi
    mov rsi, rdi
    mov rax, 1
    mov rdi, 1
    syscall
    inc r12
    jmp __show_next
__show_end:
    ret
"""

DEFAULT_TPL = ("{{DATA}}\nsection .text\nglobal _start\n\n_start:\n{{CODE}}\n"
               "    mov rax, 60\n    mov rdi, 0\n    syscall\n{{FUNCS}}")


def show_code(chk):
    regs = chk["show"]
    lines = [f"    mov [__dump+{8 * i}], {r}" for i, r in enumerate(regs)]
    lines += [f"    mov r13, {len(regs)}", f"    mov r14, {1 if chk.get('hex') else 0}",
              f"    mov r15, {1 if chk.get('signed') else 0}", "    call __show"]
    return "\n".join(lines)


def build(code, chk, templates):
    """Turns exercise code into a complete program.

    Code with `global _start` is used as it is. Otherwise a part before
    `section .text` is data; the rest are instructions that go into the
    template (`tpl`: a name from `templates` or an inline template) at
    {{CODE}}, after `pre` and before `post`. {{DATA}} gets the data part plus
    `data` from the check, {{FUNCS}} gets `funcs`. With `show`, the listed
    registers are printed (decimal, `hex`, `signed`) at {{SHOW}} or after the code.
    """
    if "global _start" in code:
        return code + ("\n" + SHOW_LIB if chk.get("show") else "")
    data = ""
    if "section .text" in code:
        data, code = code.split("section .text", 1)
        code = code.strip("\n")
    data = (data.rstrip("\n") + "\n" + chk.get("data", "")).strip("\n")
    if chk.get("pre"):
        code = chk["pre"] + "\n" + code
    if chk.get("post"):
        code = code + "\n" + chk["post"]
    name = chk.get("tpl", "default")
    tpl = templates.get(name, DEFAULT_TPL if name == "default" else name)
    if chk.get("show"):
        if "{{SHOW}}" in tpl:
            tpl = tpl.replace("{{SHOW}}", show_code(chk))
        else:
            code = code + "\n" + show_code(chk)
    tpl = tpl.replace("{{SHOW}}", "")
    if "default rel" not in tpl:
        tpl = "default rel\n" + tpl
    src = tpl.replace("{{DATA}}", data).replace("{{FUNCS}}", chk.get("funcs", "")).replace("{{CODE}}", code)
    if chk.get("show"):
        src += "\n" + SHOW_LIB
    return src


# --- checking ----------------------------------------------------------------

def norm(s):
    return re.sub(r"[\s_'\"`]", "", s).lower()


errors = []
counts = {"lesson": 0, "programs": 0, "perm": 0}


def expect(eid, what, res, chk):
    """True if res matches the expectation in chk (out / exit / err)."""
    if chk.get("err"):
        good = not ok(res) and ("out" not in chk or res["stdout"] == chk["out"])
        if not good:
            errors.append(f"{eid} {what}: expected an error, got exit {res['code']} out {res['stdout']!r}")
        return good
    if not (res["asm"] and res["ran"]):
        errors.append(f"{eid} {what}: did not run: {res['diag'][:300]}")
        return False
    want_code = chk.get("exit", 0)
    good = res["code"] == want_code and ("out" not in chk or res["stdout"] == chk["out"])
    if not good:
        errors.append(f"{eid} {what}: got exit {res['code']} out {res['stdout']!r}, want exit {want_code} "
                      f"out {chk.get('out')!r} {res['diag'][:200]}")
    return good


def differs(eid, what, res, chk):
    """True if res does NOT match chk (a wrong option must fail or print something else)."""
    if chk.get("err"):
        return ok(res) or ("out" in chk and res["stdout"] != chk["out"])
    return not ok(res) or res["code"] != chk.get("exit", 0) or res["stdout"] != chk.get("out")


def cases(chk):
    """A check with `cases` runs the same code with several `pre` blocks."""
    if "cases" not in chk:
        return [chk]
    base = {k: v for k, v in chk.items() if k != "cases"}
    return [{**base, **c} for c in chk["cases"]]


def check_good(eid, what, code, chk, templates):
    for c in cases(chk):
        counts["programs"] += 1
        expect(eid, what, run(build(code, c, templates)), c)


def check_wrong(eid, what, code, chk, templates):
    """At least one case must show the difference."""
    for c in cases(chk):
        counts["programs"] += 1
        if differs(eid, what, run(build(code, c, templates)), c):
            return
    errors.append(f"{eid} {what}: wrong variant behaves like the right one")


def main():
    course = json.loads(COURSE.read_text())
    data = json.loads(CHECKS.read_text())
    templates = data.get("templates", {})
    checks = data["checks"]
    for u in course["units"]:
        if args and u["id"] not in args:
            continue
        for i, b in enumerate(u["lesson"]):
            if b["type"] != "code":
                continue
            counts["lesson"] += 1
            code = b["code"]
            want = b.get("output", "")
            if "output" in b and "global _start" not in code:
                errors.append(f"{u['id']}.lesson.{i}: block with output is not a complete program")
            if "global _start" not in code:
                continue
            res = run(code)
            if "warning" in res["diag"]:
                errors.append(f"{u['id']}.lesson.{i}: assembler warning: {res['diag'][:200]}")
            if not ok(res) or res["stdout"] != want:
                errors.append(f"{u['id']}.lesson.{i}: got exit {res['code']} out {res['stdout']!r}, want {want!r} {res['diag'][:300]}")
        for e in u["exercises"]:
            eid, t = e["id"], e["type"]
            chk = checks.get(eid)
            if chk is None:
                if t != "pairs":
                    print(f"  note: {eid} ({t}) has no check")
                continue
            if t in ("input", "choice", "true_false"):
                code = chk.get("code", e.get("code")) if chk.get("run", True) else None
                if code:
                    check_good(eid, "code", code, chk, templates)
                    if t == "input" and "out" in chk and norm(chk["out"]) not in [norm(a) for a in e["answers"]]:
                        errors.append(f"{eid}: output {chk['out']!r} is not among the answers")
                    if t == "choice" and chk.get("match_option"):
                        opts = [norm(o) for o in e["options"]]
                        if opts[e["answer"]] != norm(chk["out"]) or opts.count(norm(chk["out"])) != 1:
                            errors.append(f"{eid}: output {chk['out']!r} does not match exactly the right option")
                for j, opt in enumerate(chk.get("options", [])):
                    if opt is None:
                        continue
                    if j == e["answer"]:
                        check_good(eid, f"option {j}", opt, chk, templates)
                    else:
                        check_wrong(eid, f"option {j}", opt, chk, templates)
            elif t == "gap":
                for j, opt in enumerate(e["options"]):
                    code = e["code"].replace("___", opt)
                    if j == e["answer"]:
                        check_good(eid, f"option {opt!r}", code, chk, templates)
                    else:
                        check_wrong(eid, f"option {opt!r}", code, chk, templates)
            elif t == "bug":
                lines = list(e["lines"])
                check_wrong(eid, "as shown", "\n".join(lines), chk, templates)
                lines[e["bug"]] = chk["fix"]
                check_good(eid, "fixed", "\n".join(lines), chk, templates)
            elif t == "order":
                check_good(eid, "order", "\n".join(e["lines"]), chk, templates)
                if PERM and not chk.get("no_perm"):
                    check_perm(eid, e["lines"], chk, templates)
            elif t == "blocks":
                check_good(eid, "blocks", " ".join(e["tokens"]), chk, templates)
                for k, d in enumerate(chk.get("wrong", [])):
                    check_wrong(eid, f"wrong {k}", d, chk, templates)
            for k, x in enumerate(chk.get("extra", [])):
                xc = {**{kk: vv for kk, vv in chk.items() if kk not in ("extra", "cases", "out", "err", "exit")}, **x}
                check_good(eid, f"extra {k}", x["src"], xc, templates)
    print(f"lesson blocks: {counts['lesson']}, exercise programs: {counts['programs']}, "
          f"permutations: {counts['perm']}, runs: {runs}")
    if errors:
        print("\n".join("FAIL " + e for e in errors))
        sys.exit(1)
    print("all checks passed")


def check_perm(eid, lines, chk, templates):
    """No other order of the lines may behave like the right one (local emulator)."""
    perms = [p for p in itertools.permutations(range(len(lines))) if list(p) != list(range(len(lines)))]
    cs = cases(chk)
    srcs = [build("\n".join(lines[i] for i in p), c, templates) for p in perms for c in cs]
    results = emu().run_many(srcs)
    counts["perm"] += len(srcs)
    runs["local"] += len(srcs)
    for k, p in enumerate(perms):
        rs = results[k * len(cs):(k + 1) * len(cs)]
        if not any(differs(eid, "perm", r, c) for r, c in zip(rs, cs)):
            errors.append(f"{eid}: order is not unique, this order works too: {[lines[i] for i in p]}")
            return


if __name__ == "__main__":
    main()
