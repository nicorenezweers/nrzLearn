#!/usr/bin/env python3
"""Checks every code block and every exercise of courses/machine-code/course.json.

See README.md. Run with the virtual environment that has capstone and unicorn:
    courses/machine-code/verify/.venv/bin/python courses/machine-code/verify/verify.py
"""
import hashlib
import itertools
import json
import re
import shutil
import subprocess
import sys
import tempfile
import time
import traceback
import urllib.request
from pathlib import Path

import capstone
import unicorn
from unicorn import x86_const as X

HERE = Path(__file__).resolve().parent
COURSE = json.loads((HERE.parent / "course.json").read_text())
CACHE_FILE = HERE / ".cache.json"
CACHE = json.loads(CACHE_FILE.read_text()) if CACHE_FILE.exists() else {}
OFFLINE = "--offline" in sys.argv
STATS = {"python": 0, "nasm": 0, "roundtrip": 0, "emulated": 0, "native": 0, "display": 0}

EX = {e["id"]: e for u in COURSE["units"] for e in u["exercises"]}


class Fail(Exception):
    pass


def ok(cond, msg="check failed"):
    if not cond:
        raise Fail(msg)


# ------------------------------------------------------------------ Python

def py(code):
    """Runs Python code, returns stdout without the final newline, or None on error."""
    STATS["python"] += 1
    r = subprocess.run([sys.executable, "-I", "-c", code], capture_output=True, text=True, timeout=20)
    if r.returncode != 0:
        return None
    out = r.stdout
    return out[:-1] if out.endswith("\n") else out


# ------------------------------------------------------------------ NASM

def with_bits(src):
    return src if src.lstrip().startswith("bits ") else "bits 64\n" + src


def nasm_listing(src):
    """Assembles src with NASM. Returns (bytes, [(source line, bytes of that line)]) or None on error."""
    STATS["nasm"] += 1
    src = with_bits(src)
    with tempfile.TemporaryDirectory() as d:
        d = Path(d)
        (d / "a.asm").write_text(src + "\n")
        r = subprocess.run(["nasm", "-f", "bin", "-l", str(d / "a.lst"), "-o", str(d / "a.bin"), str(d / "a.asm")],
                           capture_output=True, text=True)
        if r.returncode != 0 or r.stderr.strip():
            return None
        data = (d / "a.bin").read_bytes()
        per_line = {}
        for line in (d / "a.lst").read_text().splitlines():
            m = re.match(r"^\s*(\d+) ([0-9A-F]{8}) ([0-9A-F]+)-?(\s|$)", line)
            if m:
                per_line.setdefault(int(m.group(1)), bytearray()).extend(bytes.fromhex(m.group(3)))
    lines = src.split("\n")
    return data, [(lines[i - 1], bytes(per_line.get(i, b""))) for i in range(1, len(lines) + 1)]


def nasm(src):
    r = nasm_listing(src)
    return None if r is None else r[0]


def hexb(text):
    return bytes.fromhex(text)


def fmt(data):
    return " ".join(f"{b:02X}" for b in data)


def asm_output(src):
    """The lesson format: the bytes of each source line that makes bytes, one line each."""
    r = nasm_listing(src)
    ok(r is not None, f"NASM failed:\n{src}")
    return "\n".join(fmt(b) for _, b in r[1] if b)


MD = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)


def disasm(data):
    return [(i.address, i.size, f"{i.mnemonic} {i.op_str}".strip()) for i in MD.disasm(data, 0)]


def roundtrip(data, allow_tail=False):
    """Capstone disassembles the bytes, NASM assembles that text again: it must give the same bytes."""
    lines, n = [], 0
    for addr, size, text in disasm(data):
        m = re.search(r"\[rip ([+-]) (0x[0-9a-f]+|\d+)\]", text)
        if m:
            disp = int(m.group(2), 0) * (1 if m.group(1) == "+" else -1)
            text = text[:m.start()] + f"[rel {addr + size + disp}]" + text[m.end():]
        if size == 2 and text.startswith("j"):
            text = text.replace(" ", " short ", 1)
        lines.append(text)
        n += size
    ok(allow_tail or n == len(data), f"capstone decoded only {n} of {len(data)} bytes: {fmt(data)}")
    again = nasm("\n".join(lines))
    ok(again == data[:n], f"round trip differs: {fmt(data[:n])} -> {lines} -> {fmt(again or b'')}")
    STATS["roundtrip"] += 1


# ------------------------------------------------------------------ Unicorn

CODE, SENTINEL, STACK = 0x1000, 0x80000, 0xF0000


class Run:
    pass


def emu(data, count=0, regs=None, limit=20000):
    """Runs x86-64 bytes at CODE with a fake return address on the stack.
    Linux system calls write (1) and exit (60) are emulated."""
    STATS["emulated"] += 1
    mu = unicorn.Uc(unicorn.UC_ARCH_X86, unicorn.UC_MODE_64)
    mu.mem_map(0, 0x100000)
    mu.mem_write(CODE, bytes(data))
    rsp = STACK - 0x100
    mu.mem_write(rsp, SENTINEL.to_bytes(8, "little"))
    mu.reg_write(X.UC_X86_REG_RSP, rsp)
    for name, value in (regs or {}).items():
        mu.reg_write(getattr(X, f"UC_X86_REG_{name.upper()}"), value)
    r = Run()
    r.out, r.exit, r.steps, r.returned, r.loop, r.fell_off, r.error = b"", None, 0, False, False, False, None

    def on_code(uc, address, size, _):
        if address == SENTINEL:
            r.returned = True
            uc.emu_stop()
            return
        if not CODE <= address < CODE + len(data):
            r.fell_off = True
            uc.emu_stop()
            return
        r.steps += 1
        if r.steps > limit:
            r.loop = True
            uc.emu_stop()

    def on_syscall(uc, _):
        nr = uc.reg_read(X.UC_X86_REG_RAX)
        if nr == 1:
            n = uc.reg_read(X.UC_X86_REG_RDX)
            if uc.reg_read(X.UC_X86_REG_RDI) == 1:
                r.out += bytes(uc.mem_read(uc.reg_read(X.UC_X86_REG_RSI), n))
            uc.reg_write(X.UC_X86_REG_RAX, n)
        elif nr == 60:
            r.exit = uc.reg_read(X.UC_X86_REG_RDI) & 0xFF
            uc.emu_stop()
        else:
            uc.reg_write(X.UC_X86_REG_RAX, -38 & (2 ** 64 - 1))

    mu.hook_add(unicorn.UC_HOOK_CODE, on_code)
    mu.hook_add(unicorn.UC_HOOK_INSN, on_syscall, None, 1, 0, X.UC_X86_INS_SYSCALL)
    try:
        mu.emu_start(CODE, 0xFFFFFFFF, count=count)
    except unicorn.UcError as e:
        r.error = str(e)
    for name in ("rax", "rcx", "rdx", "rbx", "rsi", "rdi", "rip"):
        setattr(r, name, mu.reg_read(getattr(X, f"UC_X86_REG_{name.upper()}")))
    r.eax = r.rax & 0xFFFFFFFF
    r.edi = r.rdi & 0xFFFFFFFF
    return r


def emu_asm(src, **kw):
    data = nasm(src)
    ok(data is not None, f"NASM failed:\n{src}")
    return emu(data, **kw)


# ------------------------------------------------------------------ real CPU (Compiler Explorer)

C_TEMPLATE = """#include <stdio.h>
#include <string.h>
#include <sys/mman.h>
#include <sys/wait.h>
#include <unistd.h>

static const unsigned char code[] = {%s};

int main(void) {
    unsigned char *p = mmap(0, 65536, PROT_READ | PROT_WRITE | PROT_EXEC, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    if (p == MAP_FAILED) {
        puts("mmap failed");
        return 99;
    }
    memcpy(p, code, sizeof code);
    int fds[2];
    pipe(fds);
    fflush(stdout);
    pid_t pid = fork();
    if (pid == 0) {
        dup2(fds[1], 1);
        close(fds[0]);
        int r = ((int (*)(void))p)();
        printf("[eax=%%d]", r);
        fflush(stdout);
        _exit(0);
    }
    close(fds[1]);
    unsigned char buf[4096];
    ssize_t total = 0, n;
    while ((n = read(fds[0], buf + total, sizeof buf - total)) > 0)
        total += n;
    int status;
    waitpid(pid, &status, 0);
    printf("out=");
    for (ssize_t i = 0; i < total; i++)
        printf("%%02x", buf[i]);
    printf("\\n");
    if (WIFEXITED(status))
        printf("exit=%%d\\n", WEXITSTATUS(status));
    else
        printf("signal=%%d\\n", WTERMSIG(status));
    return 0;
}
"""

WRAPPER = "push rbx\npush rbp\ncall code\npop rbp\npop rbx\nret\ncode:\n"


def native(data, as_function=False):
    """Runs the bytes on a real x86-64 Linux machine on godbolt.org (gcc 14.2), in a child process
    whose stdout goes into a pipe, so the exact bytes and the exit code are known.
    as_function: the bytes end with ret; they are called like a C function and EAX is printed.
    Returns (stdout, exit code). Results are cached in .cache.json."""
    if as_function:
        data = nasm(WRAPPER) + bytes(data)
    src = C_TEMPLATE % ", ".join(f"0x{b:02X}" for b in data)
    key = hashlib.sha256(src.encode()).hexdigest()
    if key not in CACHE:
        if OFFLINE:
            raise Fail("not cached and --offline given")
        body = {"source": src, "options": {"userArguments": "-O1", "compilerOptions": {"executorRequest": True},
                                           "filters": {"execute": True}, "tools": [], "libraries": []},
                "lang": "c", "allowStoreCodeDebug": True}
        req = urllib.request.Request("https://godbolt.org/api/compiler/cg142/compile", data=json.dumps(body).encode(),
                                     headers={"Content-Type": "application/json", "Accept": "application/json"})
        time.sleep(1.1)
        res = json.loads(urllib.request.urlopen(req, timeout=60).read())
        ok(res.get("didExecute") and res.get("code") == 0, f"godbolt run failed: {json.dumps(res)[:800]}")
        text = "".join(x["text"] + "\n" for x in res["stdout"])
        m = re.search(r"out=([0-9a-f]*)\nexit=(\d+)", text)
        ok(m, f"unexpected godbolt output: {text!r}")
        CACHE[key] = {"stdout": bytes.fromhex(m.group(1)).decode(), "code": int(m.group(2))}
        CACHE_FILE.write_text(json.dumps(CACHE, indent=1))
    STATS["native"] += 1
    return CACHE[key]["stdout"], CACHE[key]["code"]


def native_eax(src):
    data = nasm(src)
    if not data.endswith(b"\xc3"):
        data += b"\xc3"
    out, code = native(data, as_function=True)
    m = re.search(r"\[eax=(-?\d+)\]", out)
    ok(m and code == 0, f"native run failed: {out!r} {code}")
    return int(m.group(1))


# ------------------------------------------------------------------ exercise helpers

def norm(s):
    return re.sub(r"[\s_'\"`]", "", s).lower()


def strip(s):
    return s.strip("`")


def expect_answer(e, value):
    ok(norm(e["answers"][0]) == norm(str(value)), f"answer {e['answers'][0]!r}, computed {value!r}")


def right(e):
    return e["options"][e["answer"]]


def only_right(e, test):
    """test(option) is True for the right option and False for every wrong one."""
    for i, o in enumerate(e["options"]):
        res = bool(test(o))
        ok(res == (i == e["answer"]), f"option {o!r}: test gave {res}")


def gap_each(e, run, expected):
    """Every option is put into the gap; only the right one must produce `expected`."""
    only_right(e, lambda o: run(e["code"].replace("___", o)) == expected)


def py_input(e):
    expect_answer(e, py(e["code"]))


def asm_input(e):
    expect_answer(e, fmt(nasm(e["code"])))


def bug_py_comments(e):
    wrong = []
    for i, line in enumerate(e["lines"]):
        code, comment = line.split("  # ")
        if py(code) != comment:
            wrong.append(i)
    ok(wrong == [e["bug"]], f"lines with a wrong comment: {wrong}")


def bug_asm_comments(e, fix=None):
    lines = [l.split(";")[0].rstrip() for l in e["lines"]]
    r = nasm_listing("\n".join(lines))
    ok(r is not None, "NASM failed")
    wrong = []
    for i, line in enumerate(e["lines"]):
        if ";" in line and r[1][i + 1][1] != hexb(line.split(";")[1]):
            wrong.append(i)
    ok(wrong == [e["bug"]], f"lines with wrong bytes: {wrong}")


def order_unique(e, result, expected):
    """The given order gives `expected`, and no other order does."""
    ok(result(e["lines"]) == expected, "the given order does not give the expected result")
    hits = [p for p in itertools.permutations(e["lines"]) if result(list(p)) == expected]
    ok(len(hits) == 1, f"{len(hits)} orders give the expected result")


def asm_or_none(lines):
    data = nasm("\n".join(lines))
    return None if data is None else fmt(data)


def blocks_text(e):
    return " ".join(e["tokens"])


def pairs_map(e, fn):
    """fn(left without backticks, right) must be True for every pair, and False when the right sides are mixed up."""
    for left, rt in e["pairs"]:
        ok(fn(strip(left), rt), f"pair {left!r} / {rt!r} is wrong")
    for (l1, _), (_, r2) in itertools.permutations(e["pairs"], 2):
        ok(not fn(strip(l1), r2), f"pair {l1!r} also matches {r2!r}")


CHECKS = {}


def check(eid):
    def deco(f):
        CHECKS[eid] = f
        return f
    return deco


# ------------------------------------------------------------------ bits

check("bits-1")(py_input)
check("bits-2")(py_input)


@check("bits-3")
def _(e):
    only_right(e, lambda o: o == str(len(format(0xFF, "b"))))


@check("bits-4")
def _(e):
    out = py(e["code"])
    only_right(e, lambda o: strip(o) == out)


@check("bits-5")
def _(e):
    ok((len(set(format(n, "04b") for n in range(16))) == 16 == 2 ** 4) == e["answer"])


@check("bits-6")
def _(e):
    gap_each(e, py, "00001001")


@check("bits-7")
def _(e):
    ok(py(blocks_text(e)) == "256")


@check("bits-8")
def _(e):
    values = [int(l, 0) for l in e["lines"]]
    ok(values == sorted(set(values)))


@check("bits-9")
def _(e):
    words = {"two": 2, "four": 4, "seven": 7, "nine": 9}
    pairs_map(e, lambda l, r: int(l, 0) == words[r])


check("bits-10")(bug_py_comments)


@check("bits-11")
def _(e):
    expect_answer(e, len({format(n, "03b") for n in range(2 ** 3)}))


@check("bits-12")
def _(e):
    ok((py(e["code"]) == "0b1000") == e["answer"])


# ------------------------------------------------------------------ hex

check("hex-1")(py_input)
check("hex-2")(py_input)


@check("hex-3")
def _(e):
    only_right(e, lambda o: int(strip(o), 0) == 255)


@check("hex-4")
def _(e):
    out = py(e["code"])
    only_right(e, lambda o: strip(o) == out)


@check("hex-5")
def _(e):
    ok((all(len(format(b, "02X")) == 2 for b in range(256)) and len(format(256, "X")) > 2) == e["answer"])


@check("hex-6")
def _(e):
    gap_each(e, py, "195")


@check("hex-7")
def _(e):
    gap_each(e, py, "0A")


@check("hex-8")
def _(e):
    ok(py(blocks_text(e)) == "127")


@check("hex-9")
def _(e):
    values = [int(l, 0) for l in e["lines"]]
    ok(values == sorted(set(values)))


@check("hex-10")
def _(e):
    pairs_map(e, lambda l, r: r == f"binary `{int(l, 0):04b}`")


check("hex-11")(bug_py_comments)
check("hex-12")(py_input)

# ------------------------------------------------------------------ numbers

check("numbers-1")(py_input)


@check("numbers-2")
def _(e):
    expect_answer(e, fmt((0x0A0B0C0D).to_bytes(4, "little")))


@check("numbers-3")
def _(e):
    only_right(e, lambda o: int.from_bytes(bytes([int(strip(o), 0)]), "little", signed=True) == -1)


@check("numbers-4")
def _(e):
    out = py(e["code"])
    only_right(e, lambda o: strip(o) == out)


@check("numbers-5")
def _(e):
    ok((int.from_bytes(hexb("34 12"), "little") == 0x3412) == e["answer"])


@check("numbers-6")
def _(e):
    ok((-128 <= 200 <= 127) == e["answer"])
    ok(py("(200).to_bytes(1, 'little', signed=True)") is None)


@check("numbers-7")
def _(e):
    gap_each(e, py, "e8 03 00 00")


@check("numbers-8")
def _(e):
    gap_each(e, py, "-3")


@check("numbers-9")
def _(e):
    ok(py(blocks_text(e)) == "0xfa")


@check("numbers-10")
def _(e):
    ok(e["lines"] == fmt((0x11223344).to_bytes(4, "little")).split())


@check("numbers-11")
def _(e):
    values = {"plus one": 1, "the largest value, 127": 127, "the smallest value, -128": -128, "minus one": -1}
    pairs_map(e, lambda l, r: int.from_bytes(bytes([int(l, 0)]), "little", signed=True) == values[r])
    ok(max(values.values()) == 127 and min(values.values()) == -128)


check("numbers-12")(bug_py_comments)

# ------------------------------------------------------------------ instructions

check("instructions-1")(asm_input)


@check("instructions-2")
def _(e):
    ok(disasm(hexb("C3"))[0][2] == "ret")
    ok(nasm("ret") == hexb("C3") and nasm("nop") != hexb("C3") and nasm("int3") != hexb("C3"))
    ok(nasm("mov") is None)
    ok(strip(right(e)) == "ret")


@check("instructions-3")
def _(e):
    d = disasm(hexb(e["code"]))
    ok(sum(s for _, s, _ in d) == 4)
    expect_answer(e, len(d))


@check("instructions-4")
def _(e):
    r = emu(hexb("90"), count=1)
    ok(CODE == 0x1000)
    only_right(e, lambda o: int(strip(o), 0) == r.rip)


@check("instructions-5")
def _(e):
    ok((nasm("db 0xC3") == nasm("ret")) == e["answer"])


@check("instructions-6")
def _(e):
    r = emu(nasm("lea rax, [rel $]") + hexb("90"), count=1)
    ok(nasm("lea rax, [rel $]") == hexb("48 8D 05 F9 FF FF FF"))
    r2 = emu(hexb("48 8D 05 00 00 00 00"), count=1)
    ok(r.rax == CODE and r2.rax == CODE + 7 and e["answer"] is True)


@check("instructions-7")
def _(e):
    gap_each(e, nasm, hexb("90 90 C3"))
    ok(nasm("db 90") == hexb("5A"))


@check("instructions-8")
def _(e):
    ok(nasm(blocks_text(e)) == hexb("90 C3"))


@check("instructions-9")
def _(e):
    order_unique(e, asm_or_none, "90 CC C3 90 C3")


@check("instructions-10")
def _(e):
    names = {"nothing": "nop", "returns to the caller": "ret", "stops at a breakpoint": "int3"}
    pairs_map(e, lambda l, r: disasm(hexb(l))[0][2] == names[r])


check("instructions-11")(bug_asm_comments)


@check("instructions-12")
def _(e):
    r = emu(hexb(e["code"]))
    ok(r.returned and r.error is None)
    expect_answer(e, r.steps)


# ------------------------------------------------------------------ registers

check("registers-1")(asm_input)


@check("registers-2")
def _(e):
    expect_answer(e, nasm("mov esi, 1")[0] - 0xB8)


@check("registers-3")
def _(e):
    data = hexb(e["code"])
    ok(disasm(data)[0][2] == "mov edi, 0xa")
    only_right(e, lambda o: nasm(strip(o)) == data)


@check("registers-4")
def _(e):
    ok(right(e) == str(len(nasm("mov ebx, 1"))))
    only_right(e, lambda o: o == str(len(nasm("mov ebx, 1"))))


@check("registers-5")
def _(e):
    ok((nasm("mov ebx, 0")[0] - 0xB8 == 1) == e["answer"])
    ok(nasm("mov ecx, 0")[0] - 0xB8 == 1)


@check("registers-6")
def _(e):
    ok((fmt(nasm(e["code"])) == "B8 00 01 00 00") == e["answer"])


@check("registers-7")
def _(e):
    gap_each(e, hexb, nasm("mov ecx, 3"))


@check("registers-8")
def _(e):
    gap_each(e, nasm, hexb("B8 FF FF FF FF"))


@check("registers-9")
def _(e):
    ok(hexb(blocks_text(e)) == nasm("mov ebx, 0x0102"))


@check("registers-10")
def _(e):
    ok(hexb(" ".join(e["lines"])) == nasm("mov edx, 0x11223344"))


@check("registers-11")
def _(e):
    pairs_map(e, lambda l, r: nasm(f"mov {re.search(r'`(E..)`', r).group(1).lower()}, 0")[0] == int(l, 16))


check("registers-12")(bug_asm_comments)

# ------------------------------------------------------------------ arithmetic

check("arithmetic-1")(asm_input)


@check("arithmetic-2")
def _(e):
    r = emu_asm(e["code"])
    ok(r.fell_off and r.error is None)
    expect_answer(e, r.eax)
    ok(native_eax(e["code"]) == r.eax)


@check("arithmetic-3")
def _(e):
    data = hexb(e["code"])
    only_right(e, lambda o: nasm(strip(o)) == data)


@check("arithmetic-4")
def _(e):
    modrm = fmt(nasm("add edx, ecx")[1:])
    ok(modrm == f"{0b11_001_010:02X}")
    only_right(e, lambda o: strip(o) == modrm)
    ok(nasm("add ecx, edx")[1] == 0xD1)


@check("arithmetic-5")
def _(e):
    ok((nasm("inc eax") == nasm("add eax, 1")) == e["answer"])
    ok(nasm("inc eax") == hexb("FF C0") and nasm("add eax, 1") == hexb("83 C0 01"))


@check("arithmetic-6")
def _(e):
    ok((fmt(nasm(e["code"])) == "FF CB") == e["answer"])


@check("arithmetic-7")
def _(e):
    gap_each(e, hexb, nasm("add eax, 3"))
    ok(nasm("sub eax, 3") == hexb("83 E8 03"))


@check("arithmetic-8")
def _(e):
    gap_each(e, nasm, hexb("29 D9"))


@check("arithmetic-9")
def _(e):
    ok(nasm(blocks_text(e)) == hexb("29 D1"))


@check("arithmetic-10")
def _(e):
    def eax(lines):
        r = emu_asm("\n".join(lines), regs={"rax": 0x77})
        return r.eax
    order_unique(e, eax, 7)
    ok(native_eax("\n".join(e["lines"])) == 7)


@check("arithmetic-11")
def _(e):
    instr = {"adds `ECX` to `EAX`": "add eax, ecx", "subtracts `ECX` from `EAX`": "sub eax, ecx",
             "adds 1 to `EAX`": "inc eax", "subtracts 1 from `EAX`": "dec eax"}
    pairs_map(e, lambda l, r: nasm(instr[r]) == hexb(l))
    r = emu(hexb("01 C8 29 C8 FF C0 FF C8"), regs={"rax": 10, "rcx": 3}, count=1)
    ok(r.eax == 13)
    r = emu(hexb("29 C8"), regs={"rax": 10, "rcx": 3}, count=1)
    ok(r.eax == 7)


check("arithmetic-12")(bug_asm_comments)

# ------------------------------------------------------------------ jumps


@check("jumps-1")
def _(e):
    r = emu(hexb("90" * 0x10 + "EB 05" + "90" * 0x10), count=0x11)
    expect_answer(e, hex(r.rip - CODE))


check("jumps-2")(asm_input)


@check("jumps-3")
def _(e):
    data = nasm("times 16 nop\ntarget:\nnop\nnop\njmp target")
    ok(len(data) == 20)
    only_right(e, lambda o: hexb(strip(o)) == data[-1:])
    ok(f"{(16 - 20) & 0xFF:02X}" == strip(right(e)))


@check("jumps-4")
def _(e):
    data = hexb(e["code"])
    ok(disasm(data)[0][2] == "je 5")
    zf1 = emu(data, count=1, regs={"eflags": 0x42})
    zf0 = emu(data, count=1, regs={"eflags": 0x02})
    ok(zf1.rip == CODE + 5 and zf0.rip == CODE + 2)
    ok(right(e).startswith("It jumps 3 bytes forward if the zero flag is 1"))


@check("jumps-5")
def _(e):
    ok(nasm("jmp t\nnop\nt:") == hexb("EB 01 90"))
    ok(e["answer"] is False)


@check("jumps-6")
def _(e):
    r = emu_asm(e["code"])
    ok(r.returned)
    expect_answer(e, r.eax)
    ok(native_eax(e["code"]) == r.eax)


@check("jumps-7")
def _(e):
    def run(src):
        r = emu_asm(src)
        return r.eax if r.returned else None
    gap_each(e, run, 5)
    ok(native_eax(e["code"].replace("___", right(e))) == 5)


@check("jumps-8")
def _(e):
    only_right(e, lambda o: emu(hexb("EB " + o) + hexb("90" * 4), count=1).rip == CODE)
    ok(emu(hexb("EB 00 90"), count=1).rip == CODE + 2)


@check("jumps-9")
def _(e):
    ok(hexb(blocks_text(e)) == nasm("again:\ndec ecx\njnz again"))


@check("jumps-10")
def _(e):
    order_unique(e, asm_or_none, "B9 02 00 00 00 FF C9 75 FC C3")


@check("jumps-11")
def _(e):
    names = {"always jump": "jmp", "jump if zero": "je", "jump if not zero": "jne", "jump if less": "jl"}
    pairs_map(e, lambda l, r: disasm(hexb(l + " 00"))[0][2].split()[0] == names[r])
    ok(nasm("t:\njz t")[0] == 0x74 and nasm("t:\njnz t")[0] == 0x75 and nasm("t:\njl t")[0] == 0x7C
       and nasm("t:\njg t")[0] == 0x7F and nasm("t:\nje t")[0] == 0x74 and nasm("t:\njne t")[0] == 0x75)


check("jumps-12")(bug_asm_comments)

# ------------------------------------------------------------------ programs


def xxd(data):
    with tempfile.NamedTemporaryFile(suffix=".bin") as f:
        f.write(data)
        f.flush()
        return subprocess.run(["xxd", f.name], capture_output=True, text=True, check=True).stdout.rstrip("\n")


def ndisasm(data):
    with tempfile.NamedTemporaryFile(suffix=".bin") as f:
        f.write(data)
        f.flush()
        return subprocess.run(["ndisasm", "-b", "64", f.name], capture_output=True, text=True, check=True).stdout.rstrip("\n")


@check("programs-1")
def _(e):
    data = nasm("mov eax, 60\nmov edi, 42\nsyscall")
    ok(xxd(data) == e["code"], f"xxd gives {xxd(data)!r}")
    STATS["display"] += 1
    expect_answer(e, f"{data[5]:02X}")


@check("programs-2")
def _(e):
    data = hexb(e["code"])
    r = emu(data)
    expect_answer(e, r.exit)
    ok(native(data)[1] == r.exit)


@check("programs-3")
def _(e):
    data = nasm("mov eax, 60\nmov edi, 42\nsyscall")
    ok(native(data)[1] == 42)
    ok(right(e) == "60")


@check("programs-4")
def _(e):
    data = nasm(e["code"])
    r = emu(data)
    ok(r.exit == 0 and r.out == b"OK")
    ok(native(data) == ("OK", 0))
    only_right(e, lambda o: o == "`OK`")


@check("programs-5")
def _(e):
    r = emu_asm("mov edi, 5\nxor edi, edi", regs={"rdi": 12345})
    ok((r.edi == 0) == e["answer"])
    ok(len(nasm("xor edi, edi")) == 2 and len(nasm("mov edi, 0")) == 5)


@check("programs-6")
def _(e):
    data = hexb(e["code"])
    r = emu(data)
    ok((r.exit == 0) == e["answer"])
    ok(native(data)[1] == 0)


@check("programs-7")
def _(e):
    gap_each(e, lambda src: emu_asm(src).exit, 3)
    ok(native(nasm(e["code"].replace("___", right(e))))[1] == 3)


@check("programs-8")
def _(e):
    gap_each(e, lambda src: emu(hexb(src)).exit, 1)
    ok(native(hexb(e["code"].replace("___", right(e))))[1] == 1)


@check("programs-9")
def _(e):
    ok(hexb(blocks_text(e)) == nasm("mov eax, 60\nxor edi, edi\nsyscall"))


@check("programs-10")
def _(e):
    order_unique(e, asm_or_none, "31 FF B8 3C 00 00 00 FF C7 0F 05")


@check("programs-11")
def _(e):
    instr = {"system call": "syscall", "puts 60 into `EAX`": "mov eax, 60", "sets `EDI` to 0": "xor edi, edi",
             "returns from a function": "ret"}
    pairs_map(e, lambda l, r: nasm(instr[r]) == hexb(l))


@check("programs-12")
def _(e):
    lines = list(e["lines"])
    r = emu_asm("\n".join(lines))
    ok(r.out == b"Hi" and r.exit == 0)
    ok(native(nasm("\n".join(lines))) == ("Hi", 0))
    lines[e["bug"]] = "mov edx, 3"
    r = emu_asm("\n".join(lines))
    ok(r.out == b"Hi\n" and r.exit == 0)
    ok(native(nasm("\n".join(lines))) == ("Hi\n", 0))
    for i in range(len(lines)):
        if i != e["bug"]:
            ok(e["lines"][i] == lines[i])


# ------------------------------------------------------------------ claims made in lesson texts

def lesson_claims():
    ok(nasm("mov rax, 1") == hexb("B8 01 00 00 00"))
    ok(emu_asm("mov eax, 1", count=1, regs={"rax": 2 ** 64 - 1}).rax == 1)
    ok(nasm("add rax, rcx") == hexb("48 01 C8"))
    ok(nasm("add ecx, 127")[0] == 0x83 and nasm("add ecx, -128")[0] == 0x83 and nasm("add ecx, 128")[0] == 0x81)
    ok(nasm("add ecx, eax") == hexb("01 C1"))
    far = "times 200 nop\n"
    ok(nasm("jmp t\n" + far + "t:")[0] == 0xE9 and nasm("jz t\n" + far + "t:")[:2] == hexb("0F 84"))
    ok(nasm("jmp t\ntimes 127 nop\nt:")[0] == 0xEB and nasm("jmp t\ntimes 128 nop\nt:")[0] == 0xE9)
    ok(nasm("t:\ntimes 126 nop\njmp t")[-2] == 0xEB and nasm("t:\ntimes 127 nop\njmp t")[-5] == 0xE9)
    ok(nasm("syscall") == hexb("0F 05"))
    ok(len(nasm("mov eax, 60\nmov edi, 42\nsyscall")) == 12)
    exit_prog = nasm("mov eax, 60\nmov edi, 42\nsyscall")
    ok(emu(exit_prog).exit == 42 and native(exit_prog) == ("", 42))
    r = emu(nasm("mov ecx, 3\nagain:\ndec ecx\njnz again\nret"))
    ok(r.returned and r.rcx == 0 and r.steps == 1 + 3 * 2 + 1)
    for data in (nasm("cmp eax, ebx"),):
        a = emu(data, count=1, regs={"rax": 5, "rbx": 5})
        ok(a.eax == 5 and a.rbx == 5)
    ok(fmt(nasm("int3")) == "CC")


# ------------------------------------------------------------------ lesson blocks

HEX_OUTPUT = re.compile(r"^[0-9A-F]{2}( [0-9A-F]{2})*(\n[0-9A-F]{2}( [0-9A-F]{2})*)*$")


def check_lessons(errors):
    count = 0
    for u in COURSE["units"]:
        last_asm = None
        for i, b in enumerate(u["lesson"]):
            if b["type"] != "code":
                continue
            key = f"{u['id']}.lesson.{i}"
            code, out = b["code"], b.get("output")
            try:
                if code.startswith("bits 64"):
                    last_asm = code
                    data = nasm(code)
                    ok(data is not None, "NASM failed")
                    ok(out is not None, "assembly block without output")
                    if HEX_OUTPUT.match(out):
                        got = asm_output(code)
                        ok(got == out, f"NASM gives\n{got}")
                        roundtrip(data, allow_tail="db \"" in code)
                    else:
                        r = emu(data)
                        ok(r.exit is not None and r.error is None, "program did not exit")
                        got = r.out.decode()
                        got = got[:-1] if got.endswith("\n") else got
                        ok(got == out, f"emulated output {got!r}")
                        nout, ncode = native(data)
                        nout = nout[:-1] if nout.endswith("\n") else nout
                        ok(nout == out and ncode == 0, f"native output {nout!r}, exit code {ncode}")
                elif code.startswith("00000000: "):
                    ok(out is None and xxd(nasm(last_asm)) == code, f"xxd gives\n{xxd(nasm(last_asm))}")
                    STATS["display"] += 1
                elif code.startswith("00000000  "):
                    ok(out is None and ndisasm(nasm(last_asm)) == code, f"ndisasm gives\n{ndisasm(nasm(last_asm))}")
                    STATS["display"] += 1
                else:
                    got = py(code)
                    ok(got is not None, "Python failed")
                    ok(got == (out or ""), f"Python prints\n{got}")
                count += 1
            except Exception as ex:
                errors.append(f"{key}: {ex}")
    return count


def main():
    for tool in ("nasm", "ndisasm", "xxd"):
        if not shutil.which(tool):
            sys.exit(f"{tool} not found")
    errors = []
    n_blocks = check_lessons(errors)
    missing = [eid for eid in EX if eid not in CHECKS]
    for eid in missing:
        errors.append(f"{eid}: no check")
    for eid, f in CHECKS.items():
        if eid not in EX:
            errors.append(f"{eid}: check for unknown exercise")
            continue
        try:
            f(EX[eid])
        except Exception as ex:
            errors.append(f"{eid}: {type(ex).__name__}: {ex}")
            if not isinstance(ex, Fail):
                traceback.print_exc()
    try:
        lesson_claims()
    except Exception as ex:
        errors.append(f"lesson claims: {type(ex).__name__}: {ex}\n{traceback.format_exc()}")
    for e in errors:
        print("FAIL", e)
    print(f"lesson code blocks: {n_blocks}, exercises: {len(CHECKS)}/{len(EX)}, runs: {STATS}")
    print("nasm", subprocess.run(["nasm", "-v"], capture_output=True, text=True).stdout.strip(),
          "| capstone", capstone.__version__, "| unicorn", unicorn.__version__, "| python", sys.version.split()[0])
    print("ALL PASSED" if not errors else f"{len(errors)} FAILED")
    sys.exit(1 if errors else 0)


if __name__ == "__main__":
    main()
