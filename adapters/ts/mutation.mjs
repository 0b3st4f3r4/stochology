// The mutation study behind the 14 divergence points.
//
// `02-arquitetura.md` §8.2 does not ask for a test *named* after each point; it asks for
// a test that FAILS when the point is violated. This script is the proof: for every row
// it applies one targeted mutation to a copy of the adapter, runs that row's test
// against the mutated copy, and requires the run to fail — and runs the same test
// against the unmutated adapter first, requiring it to pass, so a mistyped pattern
// cannot masquerade as a caught violation.
//
// Run: node adapters/ts/mutation.mjs
//
// The copies are written beside the originals (same directory depth, so the relative
// paths to the module and the fixture still resolve) and removed on exit.

import { readFile, writeFile, rm } from "node:fs/promises";
import { spawnSync } from "node:child_process";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const adapter = join(here, "adapter.ts");
const tests = join(here, "adapter.test.ts");
const mutantAdapter = join(here, "adapter.mutant.ts");
const mutantTests = join(here, "adapter.mutant.test.ts");

/** point, the test pattern that must catch it, and the one-line violation to inject. */
const MUTATIONS = [
  {
    point: "D1",
    pattern: "D1 ",
    why: "the safe-integer limit moves past 2^53, so a lost bit is accepted",
    from: "const limit = 9007199254740991n;",
    to: "const limit = 9007199254740994n;",
  },
  {
    point: "D2",
    pattern: "D2 ",
    why: "the probe fraction is adjusted on this side instead of crossing as the module decided it",
    from: "      fraction: payload0,",
    to: "      fraction: payload0 + 1n,",
  },
  {
    point: "D3",
    pattern: "D3 ",
    why: "the low word is written big-endian",
    from: 'view.setBigInt64(offset + 8, i64(state.l, "l"), true);',
    to: 'view.setBigInt64(offset + 8, i64(state.l, "l"), false);',
  },
  {
    point: "D4",
    pattern: "D4 ",
    why: "the Missing sentinel is coerced to a present zero",
    from: "        sVital: view.getBigInt64(out + 16, true),",
    to: "        sVital: view.getBigInt64(out + 16, true) < 0n ? 0n : view.getBigInt64(out + 16, true),",
  },
  {
    point: "D5",
    pattern: "D5 ",
    why: "the tag range widens to 0..=6, so an unknown tag decodes as a regime",
    from: `  if (!Number.isInteger(tag) || tag < 0 || tag > 5) {
    throw new AbiError("E_UNKNOWN_TAG", \`regime tag \${tag} is outside 0..=5\`);
  }
  const name = REGIME_NAMES[tag];`,
    to: `  if (!Number.isInteger(tag) || tag < 0 || tag > 6) {
    throw new AbiError("E_UNKNOWN_TAG", \`regime tag \${tag} is outside 0..=5\`);
  }
  const name = REGIME_NAMES[tag];`,
  },
  {
    point: "D6",
    pattern: "D6 ",
    why: "the view is not rebuilt after a grow, so the second call reads a detached buffer",
    from: "    if (live.memory.buffer !== view.buffer) {",
    to: "    if (false) {",
  },
  {
    point: "D7",
    pattern: "D7 ",
    why: "a base below the floor is accepted, so the arena can land inside the module's store",
    from: "  if (base < floor) {",
    to: "  if (false) {",
  },
  {
    point: "D8",
    pattern: "D8 ",
    why: "a negative return is not treated as a refusal, so a code reads as a tag",
    from: '      if (tag < 0) core(tag, "step");',
    to: '      if (tag < -100) core(tag, "step");',
  },
  {
    point: "D9",
    pattern: "D9 ",
    why: "the instrument field is dropped, so a quarantined row reaches the latency question",
    from: "    view.setInt32(offset + 0, state.instrument, true);",
    to: "    view.setInt32(offset + 0, 0, true);",
  },
  {
    point: "D10",
    pattern: "D10 ",
    why: "the acceptance threshold moves by one, so the band edge lands on the wrong side",
    from: 'view.setBigInt64(offset + 104, i64(state.acceptThreshold, "accept_threshold"), true);',
    to: 'view.setBigInt64(offset + 104, i64(state.acceptThreshold + 1n, "accept_threshold"), true);',
  },
  {
    point: "D11",
    pattern: "D11 ",
    why: "the adapter demands an export the module does not declare",
    from: '  "diagnose",\n] as const;',
    to: '  "diagnose",\n  "diagnose_v2",\n] as const;',
  },
  {
    point: "D12",
    pattern: "D12 ",
    why: "the digest is read one byte off, so it no longer matches the fixture",
    from: "      const status = live.vectors_hash(at);",
    to: "      const status = live.vectors_hash(at + 1);",
  },
  {
    point: "D13",
    pattern: "D13 ",
    why: "NoAction is reported as an inward action, so the derived table is wrong",
    from: "  if (tag === 5) return 2; // ActionSet::None",
    to: "  if (tag === 5) return 1; // ActionSet::None",
  },
  {
    point: "D14",
    pattern: "D14 ",
    why: "an unknown constraint payload is defaulted instead of refused",
    from: "    if (payload1 < 0 || payload1 > 1) {",
    to: "    if (false) {",
  },
];

function run(pattern, file) {
  const result = spawnSync(
    process.execPath,
    ["--test", "--test-name-pattern", pattern, file],
    { encoding: "utf8" },
  );
  const output = `${result.stdout ?? ""}${result.stderr ?? ""}`;
  const pass = Number(output.match(/pass (\d+)/)?.[1] ?? "-1");
  const fail = Number(output.match(/fail (\d+)/)?.[1] ?? "-1");
  return { status: result.status, pass, fail, output };
}

const source = await readFile(adapter, "utf8");
const testSource = await readFile(tests, "utf8");
const rows = [];
let clean = true;

try {
  for (const mutation of MUTATIONS) {
    const hits = source.split(mutation.from).length - 1;
    if (hits !== 1) {
      rows.push({ ...mutation, verdict: `the mutation point occurs ${hits} times` });
      clean = false;
      continue;
    }
    await writeFile(mutantAdapter, source.replace(mutation.from, mutation.to));
    await writeFile(
      mutantTests,
      testSource.replace('from "./adapter.ts"', 'from "./adapter.mutant.ts"'),
    );
    const control = run(mutation.pattern, tests);
    const mutated = run(mutation.pattern, mutantTests);
    const caught = control.status === 0 && control.fail === 0 && control.pass >= 1 && mutated.status !== 0;
    if (!caught) clean = false;
    rows.push({
      ...mutation,
      verdict: caught
        ? `caught (control ${control.pass} pass, mutant ${mutated.fail} fail)`
        : `NOT CAUGHT (control ${control.status}/${control.pass} pass/${control.fail} fail, mutant ${mutated.status}/${mutated.fail} fail)`,
    });
  }
} finally {
  await rm(mutantAdapter, { force: true });
  await rm(mutantTests, { force: true });
}

for (const row of rows) {
  console.log(`${row.point}  ${row.verdict}`);
  console.log(`     violation: ${row.why}`);
}
console.log(
  `\n${rows.filter((row) => row.verdict.startsWith("caught")).length} of ${MUTATIONS.length} points are caught by their own test`,
);
process.exit(clean ? 0 : 1);
