// A smoke test of the shipped module through a real wasm engine.
//
// The Rust tests exercise the port on the host; this script exercises the module the
// host builds, through the same `(i32, i32) -> i32` boundary an adapter will use. It
// is not part of `cargo test` because the workspace does not depend on a JavaScript
// runtime; run it by hand after the release build:
//
//   cargo build --release --target wasm32-unknown-unknown -p stoch-abi
//   node crates/stoch-abi/tests/wasm-smoke.mjs
//
// Every assertion is the document's: regime tags from `02` §2.4, the refusal codes
// from §2.3, and the digest from the fixture. Nothing is read out of a summary.

import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const module = join(here, "../../../target/wasm32-unknown-unknown/release/stoch_abi.wasm");
const bytes = readFileSync(module);
const { instance } = await WebAssembly.instantiate(bytes, {});
const e = instance.exports;
const view = new DataView(e.memory.buffer);

let failures = 0;
const check = (name, actual, expected) => {
  const ok = String(actual) === String(expected);
  if (!ok) failures += 1;
  console.log(`${ok ? "ok  " : "FAIL"} ${name}: ${actual}${ok ? "" : ` (expected ${expected})`}`);
};

// The arena: above nothing, below the data segment at 1048576.
const BASE = 200000;
const IN = BASE;
const OUT = BASE + 256;

const put = (offset, value) => view.setInt32(IN + offset, value, true);
const put64 = (offset, value) => view.setBigInt64(IN + offset, BigInt(value), true);

put64(0, 0); // instrument: sealed
put(0, 0);
put(4, 0); // no missing slot
put64(8, 200); // l
put64(16, 400); // u
put64(24, 120); // c
put64(32, 0); // b
put64(40, 1000); // r
put64(48, 100); // tts
put64(56, 100); // ttr_eff
put64(64, 1000); // n_eff
put64(72, 0); // defect
put(80, 1); // reversible
put(84, 0); // gov_corrupt
put(88, 0); // vital_floor
put(92, 0); // gov_floor
put(96, 1); // tape attested
put(100, 0); // site none
put64(104, 100_000); // accept_threshold
put64(112, 50_000); // band_width
for (let i = 0; i < 7; i += 1) put64(120 + i * 8, 900_000);
put64(176, 1000); // budget
put64(184, 10); // elapsed
put(192, 0); // proposition none
put(196, -1); // step none
put(200, 0); // lethal_omission
put(204, 0); // reopen_sealed

check("abi_version", e.abi_version(), 1n);
check("step buffered", e.step(IN, OUT), 0);
check("step payload_1", view.getInt32(OUT + 8, true), 0);
check("accept interior", e.accept(IN, OUT), 0);
check("accept site", view.getInt32(OUT + 4, true), 0);

// ESC-03 wire checks: binary lethal omission (cause 4), survival+gov joint (cause 3),
// survival pre-empts missing slot, and counter-inductive reopen (reason 5).
put64(24, 300); // c inside (200, 400)
put(80, 0); // irreversible
put(200, 1); // lethal_omission
check("step lethal omission tag", e.step(IN, OUT), 4);
check("step lethal omission cause", view.getInt32(OUT + 8, true), 4);
put(200, 0);
put(80, 1);
put64(24, 120);

put(0, 1); // instrument incomplete
put(4, 3); // missing_slot = 3
put64(48, 5); // tts = 5
put64(56, 10); // ttr_eff = 10
put(84, 1); // gov_corrupt = 1
check("step survival+gov pre-empts quarantine tag", e.step(IN, OUT), 4);
check("step survival+gov cause", view.getInt32(OUT + 8, true), 3);
put(0, 0);
put(4, 0);
put64(48, 100);
put64(56, 100);
put(84, 0);

put(204, 1); // reopen_sealed
check("accept counter-inductive reopen tag", e.accept(IN, OUT), 3);
check("accept counter-inductive reopen reason", view.getInt32(OUT + 24, true), 5);
put(204, 0);

// ESC-02 Chapter 36/41/44 boundary licences: L == C < U -> 0, U <= C -> 5.
put64(8, 200);
put64(16, 400);
put64(24, 200);
check("step L == C < U buffered", e.step(IN, OUT), 0);
put64(24, 400);
check("step L < C == U no action", e.step(IN, OUT), 5);
put64(16, 200);
put64(24, 200);
check("step L == U == C no action", e.step(IN, OUT), 5);
put64(8, 200);
put64(16, 400);
put64(24, 120);

// The refusals: -14 is the caller contract, -7 the unattested tape.
check("step misaligned out", e.step(IN, OUT + 1), -14);
check("step null", e.step(0, OUT), -14);
put(96, 0);
check("step unattested tape", e.step(IN, OUT), -7);
put(96, 1);

const HASH = BASE + 512;
check("vectors_hash", e.vectors_hash(HASH), 0);
const digest = Buffer.from(new Uint8Array(e.memory.buffer, HASH, 8)).toString("hex");
check("digest prefix", digest, "ec0ebb3b4f67ad5c");
check("vectors_hash null", e.vectors_hash(0), -14);

// A trajectory with one absorbed step and co-diagnostic theta_basin: the vital
// coordinate survives from the step before, the ruin step is named, governance binds,
// and theta_basin folds by infimum across steps.
const T = BASE + 1024;
const triples = [
  [500_000, 700_000, 600_000, 250_000, 0, 40],
  [400_000, -1, 300_000, 120_000, 1, 0],
];
triples.forEach(([epi, vital, gov, basin, absorbed, reserve], i) => {
  const at = T + i * 40;
  view.setBigInt64(at, BigInt(epi), true);
  view.setBigInt64(at + 8, BigInt(vital), true);
  view.setBigInt64(at + 16, BigInt(gov), true);
  view.setBigInt64(at + 24, BigInt(basin), true);
  view.setInt32(at + 32, absorbed, true);
  view.setInt32(at + 36, reserve, true);
});
const D = BASE + 2048;
check("diagnose", e.diagnose(T, 2, D), 0);
check("binding axis", view.getInt32(D, true), 3);
check("absorbed", view.getInt32(D + 4, true), 1);
check("vital fold", view.getBigInt64(D + 16, true), 700_000n);
check("ruin step", view.getBigInt64(D + 32, true), 1n);
check("theta_basin fold", view.getBigInt64(D + 48, true), 120_000n);
check("diagnose len<0", e.diagnose(T, -1, D), -14);
check("diagnose len>1024", e.diagnose(T, 1025, D), -14);

console.log(failures === 0 ? "smoke: all checks pass" : `smoke: ${failures} check(s) failed`);
process.exit(failures === 0 ? 0 : 1);
