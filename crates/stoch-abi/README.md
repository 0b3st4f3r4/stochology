# `stoch-abi` — the linear port of the core

This crate is the binary boundary described in the archived design document
(`02-arquitetura.md` §2 and §8.1), built on top of the already delivered `stoch-core`. It is not a second
implementation: the six regimes, the four verdicts and the diagnostic fold are the core's, and the crate
does only three things — it reads the `#[repr(C)]` records, rebuilds the core's sealed
objects from them, and writes the output records.

**Phase 4a, not activation.** The plan (archived outside the repository) required real
consumers to activate the phase; the two consumers of this crate are acknowledged
doubles. The artefact is ready for when they exist. `02-arquitetura.md` and
`03-cadeia-de-suprimentos.md` are internal project documents and are not part of this
repository; the references stand as a record of the design, and nothing here needs them
to be opened.

## Closed surface

Five functions and one memory, nothing else (`02` §2.1):

| Export | Signature | Input | Output |
| :--- | :--- | :--- | :--- |
| `step` | `(i32, i32) -> i32` | `StateRec*` (256 B) | `RegimeOut*` (64 B), returns the regime tag |
| `accept` | `(i32, i32) -> i32` | `StateRec*` | `VerdictOut*` (64 B), returns the verdict tag |
| `diagnose` | `(i32, i32, i32) -> i32` | `StepTriple*`, `len` | `DiagnosisOut*` (48 B) |
| `vectors_hash` | `(i32) -> i32` | `u8[32]*` | writes the fixture digest, returns `0` |
| `abi_version` | `() -> i64` | — | `1` |

The caller's arena step is **320 bytes** (`ARENA_STEP_BYTES`): 256 for `StateRec`
plus 64 for `RegimeOut`, with no overlap — `size_of::<StateRec>() +
size_of::<RegimeOut>() == 320` is a compile-time assertion, not a comment.

Where that arena may sit is decided by the module, not by the caller: any range that
touches the module's static store — the 65 536-byte buffer `diagnose` uses to borrow
the trajectory — is refused with `-14` before a single read. `store_span()` and
`store_overlaps()` expose the same computation the gate performs, so that an adapter can
check its placement instead of assuming it.

## Layouts

Every field, size and offset is a `const` assertion against `02` §2.2 and §8.1
(`core::mem::offset_of!` at the top of `src/lib.rs`). If one line of the document changes,
the crate stops compiling instead of starting to lie.

| Record | Size | Alignment | Fields |
| :--- | ---: | ---: | :--- |
| `StateRec` | 256 | 8 | `instrument` 0, `missing_slot` 4, `l` 8, `u` 16, `c` 24, `b` 32, `r` 40, `tts` 48, `ttr_eff` 56, `n_eff` 64, `defect` 72, `reversible` 80, `gov_corrupt` 84, `vital_floor` 88, `gov_floor` 92, `tape` 96, `site` 100, `accept_threshold` 104, `band_width` 112, `guard[7]` 120, `budget` 176, `elapsed` 184, `proposition` 192, `step_index` 196, `reserved1[56]` 200 |
| `RegimeOut` | 64 | 8 | `payload_0` 0 (fraction), `payload_1` 8 (floor / slot / cause), `payload_2` 12, `reserved[48]` 16 |
| `VerdictOut` | 64 | 8 | `proposition` 0, `site` 4, `margin` 8, `at_step` 16, `reserved[40]` 24 |
| `StepTriple` | 32 | 8 | `s_epi` 0, `s_vital` 8, `s_gov` 16, `absorbed` 24, `reserve` 28 |
| `DiagnosisOut` | 48 | 8 | `binding_axis` 0, `absorbed` 4, `s_epi` 8, `s_vital` 16, `s_gov` 24, `ruin_step` 32, `terminal_reserve` 40 |

## Return codes

The error channel is the return `i32` (`02` §2.3). `0 ..= 5` is a regime, `0 ..= 3` is a
verdict or a binding axis, and the negatives are refusals:

* `-1 ..= -12` — the core's codes, verbatim from `Error::code()`;
* `-14` — `E_CALLER_CONTRACT`: a null pointer, a misaligned one, or one outside the
  module's memory; a negative `len` or one above `MAX_STEPS`; a tag or flag outside the
  document's closed set; an absorbed step that carries a vital reading; a reading in the
  slot the record itself declares missing;
* `-5` — `E_SCALE_OVERFLOW` — also used when a guard reading falls outside
  `-1 ∪ 0 ..= SCALE`, because a margin outside the scale is not a large margin.

**No export panics, and none returns success with partial state.** Every pointer, flag
and padding field is validated before the core is called; every output record is
assembled in full before a single write; on any negative return the output region is left
intact.

## The three gates the port adds

1. **Field gate.** A tag, a flag or a slot outside its closed set is refused with `-14`
   instead of being coerced. In particular `guard[i]` must be `-1` (absent) or a margin
   in `0 ..= SCALE`.
2. **Range gate.** A pointer whose range is not entirely inside the module's memory is
   refused before it is read — in wasm32 memory is one instruction away, and a truncated
   `len` must not carry the caller outside its own arena.
3. **Store gate.** A range that intersects the module's own static store — the
   65 536-byte buffer `diagnose` lends to the trajectory — is refused with `-14`, for
   reading and for writing, at all four entry points that receive pointers. Without this
   gate, an arena placed at the floor the document published *passed*: aligned, inside
   memory, and `step` wrote the caller's result over the module's readings, or read the
   module's readings as the caller's records. The failure mode was silent, and it is the
   reason the gate exists instead of a warning. The boundary comes from `store_span()` —
   `addr_of!` over the store — not from a measured number: the caller's arena **may not**
   overlap the store, and the module refuses instead of trusting a floor.

### The axes are derived from the seven sites

`accept` receives the state, not three margins per axis: `02` §2.2 has no axis column.
An axis reading is the weakest of its sites together with the count of those that exist
— which is the definition of the fold (`01` §3.3), not a new rule. An absent site (`-1`)
does not count. A record that declares the instrument incomplete **and** gives a reading
for the missing slot is refused with `-14`: it would be declaring a measurement it also
declares it did not make.

## The separation in `diagnose`

The consumer receives nothing that can come back as an argument to `step`: `DiagnosisOut`
has no `StateRec` field, no regime, no action set, and no conversion exists in either
direction. The separation is of the type, not of the convention.

The sealed-reading gate that `Trajectory::seal` does not perform (it is a `const fn` over
a borrow, and a borrow cannot be refused) **exists at the port**: each of the three
readings of each triple is checked against `-1 ∪ 0 ..= SCALE` before the fold runs. An
absorbed step must carry `s_vital = -1`, because after absorption the vital coordinate is
a barrier and not a number — a value there would be a measurement the barrier cannot
have, and the core would simply ignore it.

The fold needs the readings as a single slice and this crate has no allocator: it uses a
static buffer of `MAX_STEPS = 1024` steps. A larger length is refused with `-14`, never
truncated — a truncated fold is another fold and would name the wrong step.

## Recorded divergences (the document is not adjusted)

| # | Divergence | Reason |
| :--- | :--- | :--- |
| D1 | `crate-type = ["cdylib", "rlib"]`, not only `cdylib` | `cargo test -p stoch-abi` links the crate into test binaries on the host; a `cdylib`-only lib is not linkable as a Rust dependency. The shipped artefact is still the `cdylib`'s `.wasm`. |
| D2 | `no_std` only on the shipped target (`cfg(all(not(test), target_arch = "wasm32"))`) | The host build exists only for the tests; it links `std` and needs no `#[panic_handler]`. On wasm32 the handler comes from `stoch-core`. |
| D3 | `vectors_hash` writes the 32 bytes at the pointer and returns `0` | §2.1 fixes `(i32) -> i32`: 64 bits do not fit in an `i32`. The first 8 bytes the adapter reads are the handshake token; truncating the digest at the boundary would lose the rest for no reason. |
| D4 | The record has no contingency field | §2.2 does not define one and `accept` never reads it (it is what the non-commutation test substitutes). The port uses `ContingencyTape::new(0)`. |
| D5 | `MAX_STEPS = 1024` steps per call | Without an allocator the slice must be static and therefore bounded. A declared refusal, not truncation. |
| D6 | `__data_end` and `__stack_pointer` are **not** exported in this build; and the caller's arena **may not** overlap the module's static store | Exporting them requires linker flags (`-C link-arg=--export=...`) that would have to be workspace globals; the accepted build is the plain command. Consequence for §8.1: the adapter cannot compute `align8(__data_end)` from the exported surface — and, if it did, it would land on the module's memory. The defence is not a floor: **every pointer received is refused with `-14` if the range `[ptr, ptr + len)` intersects the module's static store**, and the boundary is derived in code from `core::ptr::addr_of!` over that store (`store_span`), never from a measured literal. The number below is a **measurement of this build** and serves for diagnosis, not as placement advice. |
| D7 | The `vectors.kv` fixture carries three margins per axis; the record carries seven readings per site | They are two forms of the same object: the `key=value` file is the harness's form, the record is the boundary's form. `accept` derives the axes from the seven sites (above), and the differential test compares the port with the core over the same state. |

## Module measurement (new measurement)

Measured in this build, not inherited from §0: the release `stoch_abi.wasm` is
**8 124 bytes**, with an initial memory of **18 pages** (1 179 648 bytes), **10 sections**
(Type, Function, Memory, Global, Export, Code, Data, and three `custom`: `name`,
`producers`, `target_features`), **6 exports** (`memory` and the five functions),
**0 import sections**, **0 `memory.grow`**, **0 tables**, **0 start functions**. The
single data segment is 60 bytes at `1048576` and holds the digest
`efe2bad329e5727899e4c89b7989d4c9e92cc01c961ce896f71e7f1aed6bd3d0` of the current
fixture; `__stack_pointer` initialises at `1048576`.

The data segment ends at `1048636` (`1048576 + 60`), and the linker placed the static
store (`TRAJECTORY`, the 1 024 × 64 bytes of `.bss`) at the first address that respects
the array's 8-byte alignment: **start `1048640`**, **end `1114176`** (`1048640 + 65 536`),
against a `memory.buffer.byteLength` of **1 179 648** (18 pages) — inside memory, and that
is the reserve declared in this build. The three values were measured in this module, not
inherited: a probe build outside the tree, with two temporary exports returning
`addr_of!(TRAJECTORY.0)` and `addr_of!(TRAJECTORY.0) + 65 536`, read through
`WebAssembly.instantiate` in Node, plus `memory.buffer.byteLength`. The probe is not in
the tree — the shipped `-14` derives the address of the array itself, which is why it
cannot go stale.

The start is **not** `align8(__data_end)` from an earlier build: `__data_end` is the end
of `.rodata` in this build (`1048636`), and what falls on the first multiple of 8 from it
is `.bss`, not data. The number that holds is the address the linker gave the array —
which is why the module derives it in code, with `core::ptr::addr_of!`, and refuses with
`-14` (`E_CALLER_CONTRACT`) any range that overlaps it. The value here is a diagnostic
measurement, not a placement instruction: whoever places the arena must derive it from
the top of memory, and receives a refusal — not corruption — if they get it wrong.

**Why the number moved from 7 943 to 8 124.** The delta is **+181 bytes, all in the Code
section** (6 959 → 7 140); Type, Function, Memory, Global, Export, Data and the three
`custom` sections are exactly the same size as before. It is the overlap gate, inlined at
the four entry points that receive pointers (`step`, `accept`, `diagnose`,
`vectors_hash`): derivation of the store's address, range comparison, and the
zero-length case. There is no new function in the Function section (there are still 14:
three gatekeepers, the five exports and six core functions), no new export, and the data
segment did not change — the store remains in zero-initialised memory.

**Why the number moved from 7 925 to 7 943.** The previous measurement was of the
single-file crate. After the split (C6: `#![deny(unsafe_code)]` in the crate,
`#![allow(unsafe_code)]` in a single module, `src/abi.rs`), the three internal gatekeepers
are now called `stoch_abi::abi::*` instead of `stoch_abi::*`, and it is only `name` — a
`custom` section, not executable — that carries module names. The 18 bytes of difference
are all there: `name` is 521 bytes (0x209), and without the three `custom` sections the
module of that time was **7 189 bytes**, the part that depends on no name. The code, the
export set, the return codes and the layout did not change (the 14 functions of the Code
section are still the same — three gatekeepers, the five exports and six core functions —
and the module test compares the export set by equality, not by containment). There was
no code growth.

With the overlap gate, the same build without the three `custom` sections measures
**7 370 bytes** (`RUSTFLAGS='-C strip=symbols'`), that is, the gate's 181 bytes added to
7 189: the gate is code, not name.

The figure `2 748` of §0 belongs to another reduction (194 lines) and was **not** touched;
this is the size of the artefact §2 describes, now that it exists.

## What remains to be done

1. **Sealed-reading gate.** Present at the port for `diagnose` and for the margins; what
   remains is not verifiable by bytes: the port can refuse an out-of-scale reading, it
   cannot know whether the reading was *taken*. The provenance of the seven readings (the
   sealed instrument upstream) is the caller's obligation, and the adapter's README must
   say so — here it stands declared.
2. **Module size.** Measured above; the acceptable limit for the adapter is not yet fixed
   by anyone, and fixing it is the author's decision, not this crate's.
3. **Thompson/Wheeler limit.** The supply chain is not closed by this phase: the module is
   compiled by a single `rustc`, and residue A4 (`03:107–148`) remains open — Thompson's
   (1984) attack survives a complete reading of the source, and Wheeler's criterion
   (2005; 2009) requires *Diverse Double-Compiling* with independent bootstrap chains
   (`mrustc`/GNU Guix), which Phase 4 does not run. This crate can be audited byte by
   byte; it cannot be *proved* free of the compiler that produced it.

## How to verify

```sh
export TMPDIR="${TMPDIR:-$HOME/.cache/rs-tmp}"   # only when /tmp is a small tmpfs
cargo test --offline -p stoch-abi
cargo build --release --target wasm32-unknown-unknown -p stoch-abi
cargo test --offline --workspace
```

`tests/port.rs` exercises the port on the host (happy vector per regime, refusal of
`accept` with a core code, absorbed trajectory, partial state refused without a write,
digest against `sha256sum` of the file on disk, and sensitivity to a single flipped byte).
`tests/module.rs` reads the binary: it walks the section table and the export section and
compares the set of names — it does not look for a name inside a report.

Optional, with a real wasm engine: `node crates/stoch-abi/tests/wasm-smoke.mjs`
instantiates the published `.wasm` and calls the five exports through the `i32` boundary
(arena at `200000`, below the data segment), including the `-14` refusal cases and the
digest `efe2bad329e57278`. It is not part of `cargo test` because the workspace does not
depend on a JavaScript runtime.
