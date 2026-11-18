#!/usr/bin/env python3
"""Runs a NASM x86-64 Linux program in a small Unicorn-based emulator.

The program is assembled with `nasm -f bin` at a fixed address and started at
`_start`. Only the system calls write (1) and exit (60) are implemented.
Used for fast local runs and the order-uniqueness check; the authoritative
runs happen on real Linux via Compiler Explorer (see verify.py).
"""
import os, subprocess, tempfile

from unicorn import Uc, UcError, UC_ARCH_X86, UC_MODE_64, UC_HOOK_INSN
from unicorn.x86_const import UC_X86_INS_SYSCALL, UC_X86_REG_RAX, UC_X86_REG_RDI, UC_X86_REG_RSI, \
    UC_X86_REG_RDX, UC_X86_REG_RSP, UC_X86_REG_RCX, UC_X86_REG_R11, UC_X86_REG_RIP

BASE = 0x400000
STACK_TOP = 0x7ff000000
HEADER = "bits 64\norg 0x400000\nsection .text\njmp _start\n"


def assemble_many(srcs):
    """Assembles several programs with one shell process (fork is slow here)."""
    with tempfile.TemporaryDirectory() as d:
        for i, src in enumerate(srcs):
            with open(os.path.join(d, f"{i}.asm"), "w") as f:
                f.write(HEADER + src + "\n")
        script = 'for i in $(seq 0 %d); do nasm -f bin -o "$i.bin" "$i.asm" 2> "$i.err"; done; true' % (len(srcs) - 1)
        subprocess.run(["sh", "-c", script], cwd=d, check=True)
        out = []
        for i in range(len(srcs)):
            b = os.path.join(d, f"{i}.bin")
            err = open(os.path.join(d, f"{i}.err")).read()
            ok = os.path.exists(b) and "error" not in err
            out.append((open(b, "rb").read() if ok else None, err))
        return out


def run(src, max_steps=200_000):
    """Returns dict(asm, ran, code, stdout, diag) like verify.run()."""
    return run_many([src], max_steps)[0]


def run_many(srcs, max_steps=200_000):
    return [execute(img, err, max_steps) for img, err in assemble_many(srcs)]


def execute(image, err, max_steps):
    if image is None:
        return {"asm": False, "ran": False, "code": None, "stdout": "", "diag": err}
    mu = Uc(UC_ARCH_X86, UC_MODE_64)
    mu.mem_map(BASE, 8 * 1024 * 1024)
    mu.mem_write(BASE, image)
    mu.mem_map(STACK_TOP - 1024 * 1024, 1024 * 1024)
    rsp = STACK_TOP - 0x1000
    mu.mem_write(rsp, (1).to_bytes(8, "little") + bytes(24))
    mu.reg_write(UC_X86_REG_RSP, rsp)
    out = bytearray()
    state = {"code": None}

    def on_syscall(uc, _):
        nr = uc.reg_read(UC_X86_REG_RAX)
        a1, a2, a3 = (uc.reg_read(r) for r in (UC_X86_REG_RDI, UC_X86_REG_RSI, UC_X86_REG_RDX))
        uc.reg_write(UC_X86_REG_RCX, uc.reg_read(UC_X86_REG_RIP) + 2)
        uc.reg_write(UC_X86_REG_R11, 0x246)
        if nr == 1:
            if a1 in (1, 2):
                data = bytes(uc.mem_read(a2, a3))
                if a1 == 1:
                    out.extend(data)
                uc.reg_write(UC_X86_REG_RAX, a3)
            else:
                uc.reg_write(UC_X86_REG_RAX, (-9) & 0xFFFFFFFFFFFFFFFF)
        elif nr == 60 or nr == 231:
            state["code"] = a1 & 0xFF
            uc.emu_stop()
        else:
            uc.reg_write(UC_X86_REG_RAX, (-38) & 0xFFFFFFFFFFFFFFFF)

    mu.hook_add(UC_HOOK_INSN, on_syscall, None, 1, 0, UC_X86_INS_SYSCALL)
    diag = ""
    try:
        mu.emu_start(BASE, BASE + len(image) + 0x100000, count=max_steps)
    except UcError as e:
        diag = str(e)
    code = state["code"]
    if code is None:
        diag = diag or "no exit"
        code = -1
    text = out.decode("utf-8", "replace")
    if text.endswith("\n"):
        text = text[:-1]
    return {"asm": True, "ran": True, "code": code, "stdout": text, "diag": diag}


if __name__ == "__main__":
    import sys
    print(run(open(sys.argv[1]).read()))
