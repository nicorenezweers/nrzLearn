const run = (node, code) => {
  const r = node(code);
  return r.ok ? r.out : `ERROR ${r.err.split("\n").find((l) => /Error/.test(l)) ?? ""}`;
};
const outputs = (map) => ({ e, node }) => map[run(node, e.code)] ?? -1;
const onlyIndex = (flags) => (flags.filter(Boolean).length === 1 ? flags.indexOf(true) : -1);
const prints = (node, code, expected) => run(node, code) === expected;

export default {
  "hello-1": {
    pick: ({ e, node, strip }) => onlyIndex(e.options.map((o) => prints(node, strip(o), "Hello"))),
  },
  "hello-4": {
    holds: ({ e, node }) => {
      const out = run(node, e.code).split("\n");
      return out.indexOf("B") < out.indexOf("A");
    },
  },
  "hello-5": { out: "Hi" },
  "hello-6": { out: "Score: 10" },
  "hello-7": { out: "Ready\nSet\nGo!" },
  "hello-8": {
    check: ({ node }) => prints(node, 'console.log(typeof "7", typeof 7)', "string number")
      && prints(node, "console.log(7)", "7")
      && prints(node, "// 7", ""),
  },
  "hello-9": { fix: 'console.log("Middle");', out: "Start\nMiddle\nEnd" },
  "hello-12": { holds: ({ node }) => prints(node, 'Console.log("Hi");', "Hi") },

  "variables-2": {
    pick: ({ e, node, strip }) => onlyIndex(e.options.map((o) => {
      const kw = strip(o);
      return prints(node, `${kw} v = 1;\nconsole.log(v);`, "1") && !node(`${kw} v = 1;\nv = 2;`).ok;
    })),
  },
  "variables-4": { holds: ({ e, node }) => prints(node, e.code, "0") },
  "variables-5": { out: "boolean" },
  "variables-6": { fix: "console.log(total);", out: "12" },
  "variables-7": {
    check: ({ node }) => prints(node, 'console.log(typeof 42, typeof "42", typeof false, typeof undefined)', "number string boolean undefined")
      && prints(node, "let v;\nconsole.log(v, null);", "undefined null"),
  },
  "variables-9": { out: "12" },
  "variables-10": { after: "\nconsole.log(city);", out: "Rome" },
  "variables-12": { holds: ({ node }) => prints(node, "console.log(typeof null);", "null") },

  "operators-4": { holds: ({ e, node }) => prints(node, e.code, "true") },
  "operators-5": { out: "true" },
  "operators-6": { before: 'const name = "Ada";\n', after: "\nconsole.log(size);", out: "3" },
  "operators-7": { out: "Total: 12" },
  "operators-8": {
    check: ({ node }) => prints(node, 'let x = 1;\nx += 4;\nconsole.log(7 % 3, 2 ** 3, 2 === 2, 2 !== 2, x);', "1 8 true false 5"),
  },
  "operators-9": { fix: "const message = `Hello, ${name}!`;", out: "Hello, Sam!" },
  "operators-12": { holds: ({ e, node }) => prints(node, e.code, "c") },

  "conditions-2": { pick: outputs({ A: 0, B: 1, C: 2, "B\nC": 3 }) },
  "conditions-3": { holds: ({ node }) => prints(node, "console.log(true && false);", "true") },
  "conditions-4": { out: "Welcome" },
  "conditions-6": { fix: 'if (day === "Saturday" || day === "Sunday") {', out: "Weekend" },
  "conditions-7": {
    check: ({ node }) => prints(node, 'console.log(true && false, false || true, !true, 1 > 2 ? "a" : "b");', "false true false b"),
  },
  "conditions-8": {
    cases: [
      { before: "const points = 60;\n", out: "Win" },
      { before: "const points = 40;\n", out: "Lose" },
    ],
  },
  "conditions-9": {
    after: "\nconsole.log(answer);",
    cases: [
      { before: "const ok = true;\n", out: "yes" },
      { before: "const ok = 0;\n", out: "no" },
    ],
  },
  "conditions-10": {
    pick: ({ e, node, strip }) => onlyIndex(e.options.map((o) => prints(node, `console.log(Boolean(${strip(o)}));`, "true"))),
  },
  "conditions-12": { holds: ({ e, node }) => prints(node, e.code, "yes") },

  "loops-1": { expected: ({ e, node }) => String(run(node, e.code).split("\n").filter((l) => l === "Hi").length) },
  "loops-4": { holds: ({ e, node }) => prints(node, e.code, "0\n1\n3\n4") },
  "loops-5": { out: "5\n4\n3\n2\n1" },
  "loops-6": { fix: "  i++;", out: "1\n2\n3" },
  "loops-7": { out: "6" },
  "loops-8": {
    check: ({ node }) => prints(node, [
      "let seen = [];",
      "for (let i = 0; i < 5; i++) {",
      "  if (i === 1) {",
      "    continue;",
      "  }",
      "  if (i === 3) {",
      "    break;",
      "  }",
      "  seen.push(i);",
      "}",
      "let n = 0;",
      "n++;",
      "let w = 2;",
      "while (w) {",
      "  w--;",
      "}",
      'for (const c of "ab") {',
      "  seen.push(c);",
      "}",
      "console.log(seen.join(), n, w);",
    ].join("\n"), "0,2,a,b 1 0"),
  },
  "loops-9": { after: "\n  console.log(i);\n}", out: "0\n1\n2\n3\n4" },
  "loops-10": { pick: outputs({ "A\nB\nC": 0, ABC: 1, abc: 2 }) },
  "loops-12": { holds: ({ node }) => prints(node, "let runs = 0;\nwhile (false) {\n  runs++;\n}\nconsole.log(runs);", "0") },

  "functions-3": { pick: outputs({ "5\n5": 0, "5\nundefined": 1, undefined: 2, 5: 3 }) },
  "functions-4": { holds: ({ e, node }) => run(node, e.code + "\ntest();").includes("after") },
  "functions-5": { out: "8" },
  "functions-6": { after: "\nconsole.log(half(10));", out: "5" },
  "functions-7": { out: "Hi, Sam" },
  "functions-8": {
    check: ({ node }) => prints(node, [
      "function f(x = 1) {",
      "  return x * 2;",
      "}",
      "const g = (x) => x * 2;",
      "console.log(typeof f, f(), f(3), g(4));",
    ].join("\n"), "function 2 6 8"),
  },
  "functions-9": { fix: "  return width * height;", out: "20" },
  "functions-10": {
    pick: ({ e, node, strip }) => onlyIndex(e.options.map((o) => prints(node, `${e.code}\nconsole.log(${strip(o)});`, "Pen x1"))),
  },
  "functions-12": { holds: ({ node }) => prints(node, "const f = (x) => x + 1;\nconsole.log(f(2));", "3") },

  "arrays-4": { holds: ({ e, node }) => prints(node, e.code, "cup") },
  "arrays-5": { out: "[ 1, 4, 9 ]" },
  "arrays-6": { before: 'const words = ["sun", "moon", "star", "sky"];\n', after: "\nconsole.log(long);", out: "[ 'moon', 'star' ]" },
  "arrays-7": { out: "[ 'a', 'b', 'c' ]" },
  "arrays-8": {
    check: ({ node }) => prints(node, [
      'const a = ["x", "y"];',
      'a.push("z");',
      "const removed = a.pop();",
      'console.log(removed, a.length, a[0], a.includes("y"), a.includes("z"));',
    ].join("\n"), "z 2 x true false"),
  },
  "arrays-9": { fix: "const last = fruits[fruits.length - 1];", out: "plum" },
  "arrays-12": { holds: ({ node }) => !prints(node, "const a = [1];\na.push(2);\nconsole.log(a.length);", "2") },

  "objects-4": { holds: ({ e, node }) => prints(node, e.code, "red") },
  "objects-5": { out: "blue" },
  "objects-6": { before: 'const user = { name: "Ada", age: 36 };\n', after: "\nconsole.log(name, age);", out: "Ada 36" },
  "objects-7": { out: "Ada is 36" },
  "objects-8": {
    check: ({ node }) => prints(node, [
      "const obj = {",
      "  key: 1,",
      "  self() {",
      "    return this;",
      "  },",
      "};",
      "const { key } = obj;",
      'console.log(obj.key, obj["key"], key, JSON.stringify(obj), obj.self() === obj);',
    ].join("\n"), '1 1 1 {"key":1} true'),
  },
  "objects-9": { fix: "    return `Hi, I am ${this.name}`;", out: "Hi, I am Max" },
  "objects-12": {
    holds: ({ node }) => prints(node, "const o = {\n  check() {\n    return this === o;\n  },\n};\nconsole.log(o.check());", "true"),
  },
};
