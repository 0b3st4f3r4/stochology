// Consumer A: the normative fixture over the wasm boundary, walked step by step.
//
// It is a DOUBLÊ — a test double, not a product. It exercises the ABI the way a host
// would: it decodes `crates/stoch-core/vectors.kv`, writes each record into the caller's
// arena, calls the five exports and compares the answer with the row's declaration
// using the same canonical spelling the Rust harness uses. A divergence is an exit
// code, not a warning.
//
// Run: node consumers/node/run.mjs [--vectors PATH]

import { readFileSync, existsSync, mkdirSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { homedir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import { ABI_VERSION, AbiError, open, toSafeNumber } from "../../adapters/ts/adapter.ts";

const here = dirname(fileURLToPath(import.meta.url));
const root = join(here, "..", "..");
const wasmPath = join(root, "target/wasm32-unknown-unknown/release/stoch_abi.wasm");
const defaultVectors = join(root, "crates/stoch-core/vectors.kv");
// Some hosts mount /tmp as a small tmpfs and the linker dies mid-link there.
// Use the caller's TMPDIR when it has one, and keep the fallback inside $HOME.
const tmpFallback = join(homedir(), ".cache", "rs-tmp");

const args = process.argv.slice(2);
const vectorsIndex = args.indexOf("--vectors");
const vectorsPath = vectorsIndex >= 0 ? resolve(args[vectorsIndex + 1]) : defaultVectors;

// ---- the module ----------------------------------------------------------

function loadWasm() {
  if (!existsSync(wasmPath)) {
    if (!process.env.TMPDIR) mkdirSync(tmpFallback, { recursive: true });
    const built = spawnSync(
      "cargo",
      ["build", "--offline", "--release", "-p", "stoch-abi", "--target", "wasm32-unknown-unknown"],
      {
        cwd: root,
        stdio: "inherit",
        env: { ...process.env, TMPDIR: process.env.TMPDIR ?? tmpFallback },
      },
    );
    if (built.status !== 0 || !existsSync(wasmPath)) {
      console.error(`cannot build ${wasmPath}; run the same command by hand`);
      process.exit(2);
    }
  }
  return new Uint8Array(readFileSync(wasmPath));
}

// ---- the fixture ---------------------------------------------------------

const COLUMNS = 33;
const C = {
  id: 0, fn: 1, tape: 2, instrument: 3, missingSlot: 4, tts: 5, ttr: 6, l: 7, u: 8,
  c: 9, r: 10, reversible: 11, govCorrupt: 12, vitalFloor: 13, govFloor: 14,
  epi: 15, vital: 16, gov: 17, site: 18, proposition: 19, stepIndex: 20,
  mEpi: 21, mVital: 22, mGov: 23, absorbed: 24,
  expectVariant: 25, expectCause: 26, expectConstraint: 27, expectFraction: 28,
  expectVerdict: 29, expectBinding: 30, expectReason: 31, expectError: 32,
};
const FN = { step: 0, accept: 1, diagnose: 2, seal: 3 };

// The record keys, in the order the fixture writes them. Only the names matter
// here: the reader looks a column up by key, never by position.
const KEYS = [
  "id", "fn", "tape", "instrument", "missing_slot", "tts", "ttr", "l", "u", "c", "r",
  "reversible", "gov_corrupt", "vital_floor", "gov_floor", "epi", "vital", "gov",
  "site", "proposition", "step_index", "m_epi", "m_vital", "m_gov", "absorbed",
  "expect_variant", "expect_cause", "expect_constraint", "expect_fraction",
  "expect_verdict", "expect_binding_axis", "expect_reason", "expect_error",
];
const COLUMN_OF = new Map(KEYS.map((key, column) => [key, column]));

/// One record per line, `key=value` pairs separated by a single space. A key the
/// record does not name is absent, which is `-1` for the reader below.
function parse(text) {
  const rows = [];
  for (const [index, raw] of text.split("\n").entries()) {
    const line = raw.trim();
    if (line.length === 0 || line.startsWith("#")) continue;
    const values = new Array(COLUMNS).fill(-1);
    let id = "";
    for (const field of line.split(" ")) {
      const equals = field.indexOf("=");
      if (equals < 0) throw new Error(`line ${index + 1}: ${field} is not key=value`);
      const key = field.slice(0, equals);
      const spelling = field.slice(equals + 1);
      const column = COLUMN_OF.get(key);
      if (column === undefined) throw new Error(`line ${index + 1}: unknown key ${key}`);
      if (column === 0) {
        id = spelling;
        continue;
      }
      const value = Number(spelling);
      if (!Number.isInteger(value)) {
        throw new Error(`line ${index + 1}: key ${key} is not an integer: ${spelling}`);
      }
      values[column] = value;
    }
    if (id === "") throw new Error(`line ${index + 1}: record without an id`);
    rows.push({ id, values });
  }
  return rows;
}

const at = (row, column) => row.values[column];
const flag = (row, column) => (at(row, column) === 1 ? 1 : 0);
const score = (value) => (value < 0 ? -1n : BigInt(value));
const dash = (value) => (value < 0 ? "-" : String(value));

/** The fixture's own declaration, spelled as the Rust harness spells it. */
function expectedLine(row) {
  const fn = at(row, C.fn);
  const error = at(row, C.expectError);
  if (error !== 0) {
    const name = Object.keys(FN).find((key) => FN[key] === fn) ?? `fn=${fn}`;
    return `${name} err=${error}`;
  }
  if (fn === FN.step) {
    const variant = at(row, C.expectVariant);
    return `step regime=${variant} cause=${dash(at(row, C.expectCause))} constraint=${dash(
      at(row, C.expectConstraint),
    )} fraction=${dash(at(row, C.expectFraction))} missing=${
      variant === 3 ? at(row, C.missingSlot) : "-"
    }`;
  }
  if (fn === FN.accept) {
    const verdict = at(row, C.expectVerdict);
    if (verdict === 0) return "accept verdict=0";
    if (verdict === 1) {
      const site = at(row, C.site);
      const margin =
        site <= 3 ? at(row, C.mEpi) : site <= 5 ? at(row, C.mVital) : at(row, C.mGov);
      return `accept verdict=1 site=${site} margin=${margin}`;
    }
    if (verdict === 2) {
      return `accept verdict=2 proposition=${at(row, C.proposition)} site=${at(
        row,
        C.site,
      )} step=${at(row, C.stepIndex)}`;
    }
    return `accept verdict=${verdict} reason=${at(row, C.expectReason)}`;
  }
  if (fn === FN.diagnose) {
    const absorbed = at(row, C.absorbed) === 1;
    const binding = at(row, C.expectBinding);
    return `diagnose binding=${binding <= 0 ? "-" : binding} absorbed=${absorbed ? 1 : 0} ruin=${
      absorbed ? "0" : "-"
    } reserve=${Math.max(at(row, C.r), 0)} epi=${dash(at(row, C.epi))} vital=${
      absorbed ? "-" : dash(at(row, C.vital))
    } gov=${dash(at(row, C.gov))}`;
  }
  if (fn === FN.seal) return "seal sealed";
  return `fn=${fn}`;
}

// ---- the row on the wire -------------------------------------------------

const BASE = {
  instrument: 0,
  missingSlot: 0,
  l: 0n,
  u: 0n,
  c: 0n,
  b: 0n,
  r: 0n,
  tts: 0n,
  ttrEff: 0n,
  nEff: 1000n,
  defect: 0n,
  reversible: 1,
  govCorrupt: 0,
  vitalFloor: 0,
  govFloor: 0,
  tape: 1,
  site: 0,
  acceptThreshold: 100_000n,
  bandWidth: 50_000n,
  guard: [-1n, -1n, -1n, -1n, -1n, -1n, -1n],
  budget: 1000n,
  elapsed: 10n,
  proposition: 0,
  stepIndex: -1,
};

function stateOf(row) {
  const fn = at(row, C.fn);
  const instrument = at(row, C.instrument) === 1 ? 1 : 0;
  const sealedAll = (value) => Array.from({ length: 7 }, () => value);
  // A step row carries one margin, read on all seven sites; a seal row declares the
  // flat 500_000 the core's seal vectors use; an accept row derives the axis
  // readings from the counts and the per-axis margins it declares.
  let guard = sealedAll(BigInt(Math.max(at(row, C.mEpi), 0)));
  if (fn === FN.seal) guard = sealedAll(500_000n);
  if (fn === FN.accept) {
    guard = [-1n, -1n, -1n, -1n, -1n, -1n, -1n];
    const axis = (count, margin, first, last) => {
      if (count >= 1) guard[first] = score(margin);
      for (let k = first + 1; k <= last; k += 1) guard[k] = -1n;
    };
    axis(at(row, C.epi), at(row, C.mEpi), 0, 2);
    axis(at(row, C.vital), at(row, C.mVital), 3, 4);
    axis(at(row, C.gov), at(row, C.mGov), 5, 6);
  }
  return {
    ...BASE,
    instrument,
    missingSlot: instrument === 1 ? at(row, C.missingSlot) : 0,
    l: BigInt(at(row, C.l)),
    u: BigInt(at(row, C.u)),
    c: BigInt(at(row, C.c)),
    r: fn === FN.accept || fn === FN.step ? BigInt(Math.max(at(row, C.r), 0)) : BigInt(at(row, C.r)),
    tts: BigInt(at(row, C.tts)),
    ttrEff: BigInt(at(row, C.ttr)),
    reversible: flag(row, C.reversible),
    govCorrupt: flag(row, C.govCorrupt),
    vitalFloor: flag(row, C.vitalFloor),
    govFloor: flag(row, C.govFloor),
    tape: at(row, C.tape) === 0 ? 0 : 1,
    site: Math.max(at(row, C.site), 0),
    proposition: Math.max(at(row, C.proposition), 0),
    stepIndex: at(row, C.stepIndex),
    guard,
  };
}

/** Whether this row's declaration can be written on the wire at all. */
function inexpressible(row) {
  if (at(row, C.fn) !== FN.accept) return null;
  const axes = [
    ["epistemic", C.epi, C.mEpi],
    ["vital", C.vital, C.mVital],
    ["governance", C.gov, C.mGov],
  ];
  for (const [name, countColumn, marginColumn] of axes) {
    const count = at(row, countColumn);
    const margin = at(row, marginColumn);
    if (count > 1) return `${name} count ${count} > 1 is not derivable from seven readings`;
    if (count === 1 && margin < 0) {
      return `${name} count 1 with an absent reading: the wire derives the count from the readings`;
    }
    if (count === 0 && margin >= 0) {
      return `${name} count 0 with a present reading: the wire derives the count from the readings`;
    }
  }
  return null;
}

function observedLine(core, row) {
  const fn = at(row, C.fn);
  try {
    if (fn === FN.step || fn === FN.seal) {
      const regime = core.step(stateOf(row));
      if (fn === FN.seal) return "seal sealed";
      return `step regime=${regime.tag} cause=${dash(regime.cause ?? -1)} constraint=${dash(
        regime.constraint ?? -1,
      )} fraction=${dash(regime.fraction === null ? -1 : toSafeNumber(regime.fraction))} missing=${
        regime.missing === null ? "-" : regime.missing
      }`;
    }
    if (fn === FN.accept) {
      const verdict = core.accept(stateOf(row));
      if (verdict.tag === 0) return "accept verdict=0";
      if (verdict.tag === 1) {
        return `accept verdict=1 site=${verdict.site} margin=${toSafeNumber(verdict.margin)}`;
      }
      if (verdict.tag === 2) {
        return `accept verdict=2 proposition=${verdict.proposition} site=${verdict.site} step=${toSafeNumber(
          verdict.atStep,
        )}`;
      }
      // The wire carries no UnparsableReason (README limit L1): the tag is all the
      // boundary says, so the harness compares the tag and prints the reason as `-`.
      return `accept verdict=${verdict.tag} reason=-`;
    }
    if (fn === FN.diagnose) {
      const absorbed = at(row, C.absorbed) === 1;
      const view = core.diagnose([
        {
          sEpi: score(at(row, C.epi)),
          sVital: score(at(row, C.vital)),
          sGov: score(at(row, C.gov)),
          absorbed: absorbed ? 1 : 0,
          reserve: Math.max(at(row, C.r), 0),
        },
      ]);
      const value = (big) => (big < 0n ? "-" : big.toString());
      return `diagnose binding=${view.bindingAxis === 0 ? "-" : view.bindingAxis} absorbed=${
        view.absorbed
      } ruin=${view.ruinStep < 0n ? "-" : view.ruinStep.toString()} reserve=${value(
        view.terminalReserve,
      )} epi=${value(view.sEpi)} vital=${absorbed ? "-" : value(view.sVital)} gov=${value(
        view.sGov,
      )}`;
    }
    return `fn=${fn}`;
  } catch (error) {
    const name =
      fn === FN.seal ? "seal" : fn === FN.accept ? "accept" : fn === FN.diagnose ? "diagnose" : "step";
    if (error instanceof AbiError && error.abiCode === "E_CORE") return `${name} err=${error.coreCode}`;
    throw error;
  }
}

// ---- the run -------------------------------------------------------------

const bytes = loadWasm();
const core = open(bytes, ABI_VERSION);
const digest = createHash("sha256").update(readFileSync(vectorsPath)).digest("hex");
if (core.digestHex() !== digest) {
  console.error(`fixture digest diverges: module ${core.digestHex()}, file ${digest}`);
  process.exit(1);
}

const rows = parse(readFileSync(vectorsPath, "utf8"));
const skipped = [];
const diverged = [];
let compared = 0;

console.log(`— consumer A: ${rows.length} rows over the wire, digest ${digest.slice(0, 12)}… —`);
for (const row of rows) {
  const reason = inexpressible(row);
  if (reason !== null) {
    skipped.push({ id: row.id, reason });
    console.log(`${row.id} SKIP ${reason}`);
    continue;
  }
  const expected = expectedLine(row);
  const actual = observedLine(core, row);
  compared += 1;
  const fn = at(row, C.fn);
  const head = actual.includes(" err=")
    ? `err=${actual.match(/err=(-?\d+)/)?.[1] ?? "?"}`
    : fn === FN.step
      ? `regime=${actual.match(/regime=(\d+)/)?.[1] ?? "?"}`
      : fn === FN.accept
        ? `verdict=${actual.match(/verdict=(\d+)/)?.[1] ?? "?"}`
        : fn === FN.diagnose
          ? "fold"
          : "seal";
  console.log(`${row.id} ${head}`);
  // A `reason` is not on the wire, so an Unparsable row is compared on its tag.
  const same =
    expected === actual ||
    (expected.startsWith("accept verdict=3 reason=") && actual === "accept verdict=3 reason=-");
  if (!same) {
    diverged.push({ id: row.id, expected, actual });
    console.error(`MISMATCH ${row.id}\n  declared: ${expected}\n  returned: ${actual}`);
  }
}

// Two rows cannot be written on this wire at all, and the set is declared rather than
// counted so that a new inexpressible row cannot hide inside a matching total.
const DECLARED_SKIPS = new Set(["a10", "a11"]); // README limits L2, L3
console.log(
  `\n${compared} rows compared, ${skipped.length} declared inexpressible, ${diverged.length} diverging`,
);
const skippedIds = new Set(skipped.map((entry) => entry.id));
const lost = [...DECLARED_SKIPS].filter((id) => !skippedIds.has(id));
const grown = [...skippedIds].filter((id) => !DECLARED_SKIPS.has(id));
if (lost.length !== 0 || grown.length !== 0) {
  console.error(
    `the set of inexpressible rows changed: now writable [${lost.join(", ")}], newly inexpressible [${grown.join(", ")}]`,
  );
  process.exit(1);
}
if (diverged.length !== 0) process.exit(1);
console.log("consumer A agrees with the normative vectors");
