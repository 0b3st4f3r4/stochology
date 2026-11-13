// One test per divergence point of the archived design document (`02-arquitetura.md` §8.2).
//
// Each test is written so that it FAILS when its point is violated: the assertion is
// the property, not the name of the property. Run with `node --test`.

import { test } from "node:test";
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFile } from "node:fs/promises";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

import {
  ABI_VERSION,
  AbiError,
  MAX_STEPS,
  actionSet,
  arenaBase,
  arenaBytes,
  assertBase,
  decodeRegime,
  open,
  readDataEnd,
  toSafeNumber,
  type StateRecord,
} from "./adapter.ts";

const here = dirname(fileURLToPath(import.meta.url));
const root = join(here, "..", "..");
const wasmPath = join(root, "target/wasm32-unknown-unknown/release/stoch_abi.wasm");
const fixturePath = join(root, "crates/stoch-core/vectors.kv");
const bytes = new Uint8Array(await readFile(wasmPath));

const BASE_STATE: StateRecord = {
  instrument: 0,
  missingSlot: 0,
  l: 0n,
  u: 1400n,
  c: 400n,
  b: 0n,
  r: 0n,
  tts: 10n,
  ttrEff: 10n,
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
  guard: [500_000n, 500_000n, 500_000n, 500_000n, 500_000n, 500_000n, 500_000n],
  budget: 1000n,
  elapsed: 10n,
  proposition: 0,
  stepIndex: -1,
};

const state = (patch: Partial<StateRecord> = {}): StateRecord => ({ ...BASE_STATE, ...patch });

const core = open(bytes);

test("D1 — i64 crosses as bigint, and the one declared conversion refuses 2**53", () => {
  assert.equal(typeof core.version, "bigint");
  assert.equal(core.version, ABI_VERSION);
  assert.equal(toSafeNumber(9007199254740991n), 9007199254740991);
  assert.throws(
    () => toSafeNumber(9007199254740992n),
    (error: unknown) => error instanceof AbiError && error.abiCode === "E_BIGINT_PRECISION",
  );
  // A number is refused at the write door, not silently truncated.
  assert.throws(
    () => core.step(state({ l: 0 as unknown as bigint })),
    (error: unknown) => error instanceof AbiError && error.abiCode === "E_BIGINT_REQUIRED",
  );
  // A fraction of 999999 millionths is a bigint end to end.
  const regime = core.step(state({ l: 0n, u: 1_000_000_000n, c: 1n }));
  assert.equal(regime.tag, 1);
  assert.equal(regime.fraction, 999_999n);
});

test("D2 — the fraction comes from the module, truncating: 714285", () => {
  const regime = core.step(state({ l: 0n, u: 1400n, c: 400n }));
  assert.equal(regime.tag, 1);
  assert.equal(regime.fraction, 714_285n);
  assert.equal(Number(regime.fraction), 714_285); // the division is not re-done in JS
});

test("D3 — the record is little-endian; a big-endian read would answer differently", () => {
  const regime = core.step(state({ l: 1n, u: 3n, c: 2n }));
  assert.equal(regime.tag, 1);
  assert.equal(regime.fraction, 500_000n);
  // The same eight bytes, read the other way, are 2**56 — and would give -10.
  const view = new DataView(new ArrayBuffer(8));
  view.setBigInt64(0, 1n, true);
  assert.equal(view.getBigInt64(0, false), 72057594037927936n);
  // A big-endian reader would hand the core 2**56 as `l` and be refused for scale.
  assert.throws(
    () => core.step(state({ l: 72057594037927936n, u: 216172782113783808n, c: 2n })),
    (error: unknown) =>
      error instanceof AbiError && error.abiCode === "E_CORE" && error.coreCode === -5,
  );
});

test("D4 — -1 is Missing on the wire, and it is not zero", () => {
  const missing = [(-1n), (-1n), (-1n), (-1n), (-1n), (-1n), (-1n)];
  const verdict = core.accept(state({ guard: missing }));
  assert.equal(verdict.tag, 3, "no readings is unparsable, not interior");
  const one = core.accept(state({ guard: [500_000n, -1n, -1n, -1n, -1n, -1n, -1n] }));
  assert.equal(one.tag, 3, "two empty axes are still unparsable");
  const diagnosis = core.diagnose([
    { sEpi: 500_000n, sVital: -1n, sGov: 600_000n, absorbed: 0, reserve: 0 },
  ]);
  assert.equal(diagnosis.sVital, -1n, "an absent reading stays -1 on the way out");
  assert.notEqual(diagnosis.sVital, 0n);
});

test("D5 — a tag outside 0..=5 is refused, never coerced", () => {
  assert.throws(
    () => decodeRegime(6, 0n, 0),
    (error: unknown) => error instanceof AbiError && error.abiCode === "E_UNKNOWN_TAG",
  );
  assert.throws(() => decodeRegime(-1, 0n, 0), /E_UNKNOWN_TAG/);
  assert.throws(() => decodeRegime(5.5, 0n, 0), /E_UNKNOWN_TAG/);
});

test("D6 — after memory grows the view is rebuilt and the next call still works", () => {
  const before = core.memoryBytes();
  const pages = core.reserve(1024);
  assert.ok(core.memoryBytes() > before, "the arena for 1024 steps needs more memory");
  assert.equal(pages, core.memoryBytes() / 65536);
  const regime = core.step(state({ l: 0n, u: 1400n, c: 400n }));
  assert.equal(regime.fraction, 714_285n, "a detached buffer would have thrown here");
});

test("D7 — the arena sits at the top of memory, above the module's image", () => {
  const bytes1 = arenaBytes(1);
  assert.equal(core.base % 8, 0, "the base is 8-aligned");
  assert.ok(
    core.base + bytes1 <= core.memoryBytes(),
    "the whole arena fits inside the memory",
  );
  assert.ok(
    core.base >= core.memoryBytes() - bytes1 - 7,
    "the arena is the top region, not a floor-adjacent one",
  );
  // The ABI's static fold buffer is MAX_STEPS readings of 64 bytes starting at
  // align8(__data_end); the arena must begin above it, and that is arithmetic,
  // not a measured address the adapter depends on.
  const storeEnd = readDataEnd(bytes) + MAX_STEPS * 64;
  assert.ok(core.base >= storeEnd, `base ${core.base} is above the store end ${storeEnd}`);
  assert.equal(arenaBase(core.memoryBytes(), 1) <= core.base + 7, true);
  assert.throws(
    () => assertBase(storeEnd, storeEnd + 8),
    (error: unknown) => error instanceof AbiError && error.abiCode === "E_ARENA_BASE",
  );
  assert.throws(() => assertBase(16, 1024), /below the floor/);
  assert.throws(() => arenaBytes(MAX_STEPS + 1), /outside 0\.\.=/);
});

test("D8 — a negative return throws with the core's code; it is never a tag", () => {
  assert.throws(
    () => core.step(state({ l: 5n, u: 5n, c: 5n })),
    (error: unknown) =>
      error instanceof AbiError && error.abiCode === "E_CORE" && error.coreCode === -10,
  );
  assert.throws(
    () =>
      core.step(state({ l: 0n, u: 1_000_000_000n, c: 999_999_999n })),
    (error: unknown) =>
      error instanceof AbiError && error.abiCode === "E_CORE" && error.coreCode === -9,
  );
});

test("D9 — Q1 preempts Q2: an incomplete instrument quarantines before the latency halts", () => {
  const regime = core.step(state({ instrument: 1, missingSlot: 3, tts: 5n, ttrEff: 10n }));
  assert.equal(regime.tag, 3, "Quarantine, not AlgedonicHalt");
  assert.equal(regime.missing, 3);
  const halt = core.step(state({ instrument: 0, tts: 5n, ttrEff: 10n }));
  assert.equal(halt.tag, 4);
  assert.equal(halt.cause, 0);
});

test("D10 — the band is half-open: threshold+width is interior, one below is boundary", () => {
  const atUpper = core.accept(
    state({ guard: [150_000n, 150_000n, 150_000n, 150_000n, 150_000n, 150_000n, 150_000n], site: 1 }),
  );
  assert.equal(atUpper.tag, 0, "fold == threshold + width is not inside the band");
  const below = core.accept(
    state({ guard: [149_999n, 149_999n, 149_999n, 149_999n, 149_999n, 149_999n, 149_999n], site: 1 }),
  );
  assert.equal(below.tag, 1);
  assert.equal(below.margin, 149_999n);
  assert.equal(below.site, 1);
});

test("D11 — the module declares no import, and instantiates against {}", () => {
  const module = new WebAssembly.Module(bytes);
  assert.deepEqual(WebAssembly.Module.imports(module), []);
  const names = core.exportNames();
  for (const name of ["memory", "abi_version", "vectors_hash", "step", "accept", "diagnose"]) {
    assert.ok(names.includes(name), `missing export ${name}`);
  }
  assert.equal(names.length, 6);
});

test("D12 — the digest the module carries is the digest of the fixture on disk", async () => {
  const fixture = await readFile(fixturePath);
  const expected = createHash("sha256").update(fixture).digest("hex");
  assert.equal(core.digestHex(), expected);
  assert.equal(core.digest().length, 32);
});

test("D13 — ActionSet is derived on this side and only tag 5 is None", () => {
  const table = [0, 1, 2, 3, 4, 5].map((tag) => actionSet(tag));
  assert.deepEqual(table, [0, 0, 1, 1, 1, 2]);
  assert.throws(
    () => actionSet(6),
    (error: unknown) => error instanceof AbiError && error.abiCode === "E_UNKNOWN_TAG",
  );
});

test("D14 — an unknown payload is refused, never defaulted to the first case", () => {
  assert.throws(
    () => decodeRegime(4, 0n, 3),
    (error: unknown) => error instanceof AbiError && error.abiCode === "E_UNKNOWN_PAYLOAD",
  );
  assert.throws(() => decodeRegime(2, 0n, 2), /E_UNKNOWN_PAYLOAD/);
  assert.throws(() => decodeRegime(3, 0n, 0), /E_UNKNOWN_PAYLOAD/);
});

test("the handshake refuses another ABI version by name", () => {
  assert.throws(
    () => open(bytes, 2n),
    (error: unknown) =>
      error instanceof AbiError &&
      error.abiCode === "E_ABI_VERSION" &&
      /ABI 1n.*ABI 2n|1.*2/.test((error as Error).message),
  );
});
