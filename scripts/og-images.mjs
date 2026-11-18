#!/usr/bin/env node
// Renders the link preview images (Open Graph, 1200×630) into site/static:
//   og.png           general image (all course logos)
//   og/<course>.png  one image per course (logo, colour, "Hello, World!")
// Run after adding a course:
//   npm install --no-save playwright-core   (or set PLAYWRIGHT to its path)
//   CHROME=/path/to/chrome node scripts/og-images.mjs
import { execFileSync } from "node:child_process";
import { mkdirSync } from "node:fs";
import { createRequire } from "node:module";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { existsSync } from "node:fs";
import os from "node:os";

const require = createRequire(import.meta.url);
const { chromium } = require(process.env.PLAYWRIGHT || "playwright-core");
const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const cargo = process.env.CARGO || [path.join(os.homedir(), ".cargo/bin/cargo"), "/opt/homebrew/bin/cargo"].find(existsSync) || "cargo";
const courses = JSON.parse(execFileSync(cargo, ["run", "-q", "--release", "-p", "nrzlearn-site", "--", "courses-json"], { cwd: root, encoding: "utf8" }));

const HELLO = {
  python: 'print("Hello, World!")', javascript: 'console.log("Hello, World!");', typescript: 'console.log("Hello, World!");',
  java: 'System.out.println("Hello, World!");', csharp: 'Console.WriteLine("Hello, World!");', rust: 'println!("Hello, World!");',
  kotlin: 'println("Hello, World!")', swift: 'print("Hello, World!")', dart: "print('Hello, World!');", php: 'echo "Hello, World!";',
  ruby: 'puts "Hello, World!"', "objective-c": 'NSLog(@"Hello, World!");', crystal: 'puts "Hello, World!"',
  solidity: 'string public greeting = "Hello, World!";', c: 'printf("Hello, World!\\n");', cpp: 'std::cout << "Hello, World!\\n";',
  go: 'fmt.Println("Hello, World!")', zig: 'std.debug.print("Hello, World!\\n", .{});', nim: 'echo "Hello, World!"',
  d: 'writeln("Hello, World!");', v: "println('Hello, World!')", odin: 'fmt.println("Hello, World!")',
  haskell: 'main = putStrLn "Hello, World!"', ocaml: 'print_endline "Hello, World!"', fsharp: 'printfn "Hello, World!"',
  scala: 'println("Hello, World!")', elixir: 'IO.puts "Hello, World!"', erlang: 'io:format("Hello, World!~n").',
  gleam: 'io.println("Hello, World!")', clojure: '(println "Hello, World!")', racket: '(displayln "Hello, World!")',
  scheme: '(display "Hello, World!")', "common-lisp": '(format t "Hello, World!~%")', prolog: ":- writeln('Hello, World!').",
  sql: "SELECT 'Hello, World!';", r: 'print("Hello, World!")', julia: 'println("Hello, World!")', bash: 'echo "Hello, World!"',
  lua: 'print("Hello, World!")', perl: 'print "Hello, World!\\n";', fortran: 'print *, "Hello, World!"',
  cobol: 'DISPLAY "Hello, World!".', pascal: "writeln('Hello, World!');", delphi: "Writeln('Hello, World!');",
  ada: 'Put_Line ("Hello, World!");', smalltalk: "Transcript showCr: 'Hello, World!'.", assembly: "mov rax, 1",
  "machine-code": "B8 3C 00 00 00 0F 05",
};
const esc = s => s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
const LOGO = '<svg width="64" height="64" viewBox="0 0 64 64"><rect width="64" height="64" rx="15" fill="#fff"/><path d="M15 46 Q15 34 30 36 T47 21" fill="none" stroke="#0b0b0c" stroke-width="5" stroke-linecap="round"/><circle cx="15" cy="46" r="5.5" fill="#0b0b0c"/><circle cx="30" cy="36" r="5.5" fill="#0b0b0c"/><circle cx="47" cy="20.5" r="9.5" fill="#0b0b0c"/><path d="M47 13.9 L49 18.5 L53.6 20.5 L49 22.5 L47 27.1 L45 22.5 L40.4 20.5 L45 18.5 Z" fill="#fff"/></svg>';
const tile = (c, size, radius) => `<div style="width:${size}px;height:${size}px;border-radius:${radius}px;background:${c.color};color:${c.on_color};display:flex;align-items:center;justify-content:center;flex:none"><svg viewBox="${c.view_box}" width="${Math.round(size * 0.62)}" height="${Math.round(size * 0.62)}" fill="currentColor">${c.body}</svg></div>`;
const page = (inner, glow) => `<!doctype html><html><head><meta charset="utf-8"><style>*{box-sizing:border-box}body{margin:0;width:1200px;height:630px;overflow:hidden;font-family:-apple-system,"SF Pro Display","Helvetica Neue",Arial,sans-serif;background:#0b0b0c;color:#fff}.wrap{position:relative;width:100%;height:100%;padding:64px 72px;display:flex;flex-direction:column;justify-content:space-between;background:radial-gradient(900px 520px at 100% 0%,${glow}33,transparent 70%)}.brand{display:flex;align-items:center;gap:18px;font-size:42px;letter-spacing:-.02em}.brand b{font-weight:800}.brand span{font-weight:300}</style></head><body><div class="wrap">${inner}</div></body></html>`;
const brand = `<div class="brand">${LOGO}<div><b>nrz</b><span>Learn</span></div></div>`;

mkdirSync(path.join(root, "site/static/og"), { recursive: true });
const browser = await chromium.launch({ executablePath: process.env.CHROME });
const p = await browser.newPage({ viewport: { width: 1200, height: 630 } });
for (const c of courses) {
  if (!c.body) continue;
  const code = HELLO[c.id] || "";
  const inner = `<div style="display:flex;align-items:center;gap:60px;margin-top:20px">${tile(c, 264, 60)}<div style="min-width:0"><div style="font-size:${c.name.length > 10 ? 92 : 112}px;font-weight:800;letter-spacing:-.03em;line-height:1">${esc(c.name)}</div>${code ? `<div style="margin-top:34px;display:inline-block;font:500 ${code.length > 34 ? 22 : code.length > 26 ? 25 : 30}px/1.35 ui-monospace,'SF Mono',Menlo,monospace;background:#18181b;border:2px solid #2a2a30;border-radius:18px;padding:18px 24px;color:#e4e4e7;white-space:nowrap">${esc(code)}</div>` : ""}</div></div><div style="display:flex;align-items:center;justify-content:space-between">${brand}<div style="width:120px;height:12px;border-radius:6px;background:${c.color}"></div></div>`;
  await p.setContent(page(inner, c.color));
  await p.screenshot({ path: path.join(root, `site/static/og/${c.id}.png`) });
}
const grid = courses.filter(c => c.body).map(c => tile(c, 62, 15)).join("");
const inner = `<div style="display:flex;gap:56px;align-items:center;height:100%"><div style="flex:none;width:420px">${brand}<div style="margin-top:36px;font-size:120px;font-weight:800;line-height:1;letter-spacing:-.04em">48</div><div style="margin-top:14px;font:500 28px/1.4 ui-monospace,'SF Mono',Menlo,monospace;color:#a1a1aa">print("Hello, World!")</div></div><div style="display:grid;grid-template-columns:repeat(8,62px);gap:12px">${grid}</div></div>`;
await p.setContent(page(inner, "#ffffff"));
await p.screenshot({ path: path.join(root, "site/static/og.png") });
await browser.close();
console.log(`${courses.length} course images + og.png`);
