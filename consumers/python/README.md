# Consumer B — Python over Wasmtime (a test double)

**These two consumers are test doubles built to rehearse the ABI; the trigger of
PLAN §4.2 remains unmet.**

It shares no code with consumer A: it loads the same `.wasm` directly through
Wasmtime, rebuilds the byte layouts from the chapter's table, and checks the boundary
invariants on its own.

```
TMPDIR=$HOME/.cache/rs-tmp PYTHONPATH=$HOME/pylibs python3 consumers/python/run.py
```

Exit code is `0` only when every invariant passes; any failure prints `FAIL <name>`
and exits `1` (the first version of this script returned `0` while printing failures,
which is itself a defect of the consumer, not of the port).

## What it checks, with no adapter in the path

- the module declares **zero imports** and exports exactly `memory`, `abi_version`,
  `step`, `accept`, `diagnose`, `vectors_hash` (D11);
- `abi_version()` is `1`, the version the chapter declares;
- the record sizes: `StateRec` 256, `RegimeOut` 64, `VerdictOut` 64, `StepTriple` 32,
  `DiagnosisOut` 48, one arena step 320 = 256 + 64;
- the store's start is `align8(__data_end)`, recomputed here by parsing section 11 of the
  binary (including the `i32.const … end` offset expression);
- `vectors_hash` returns the digest of `crates/stoch-core/vectors.kv` (D12), and the
  digest is not disturbed by a `step` or a `diagnose`;
- return codes are codes, not tags: a null or misaligned pointer is `-14`, `L == C` is
  `-10`, a fraction that truncates to zero is `-9`, a margin off the scale is `-5`, a
  flag outside `0|1` is `-14`, an unprovided extent is `-6` (D8);
- a sealed instrument whose seven readings are not all margins is refused with `-5`
  (the core's seventh check), while an incomplete instrument quarantines with tag `3`;
- one step answers tag `1` with `fraction 500000` for `l=200, u=400, c=300`, and one
  accept answers tag `0` for an interior transcript (D2);
- the fold over three triples: `s_epi` and `s_gov` are minima over every step,
  `s_vital` skips the absorbed step, the ruin is the third step, the terminal reserve
  is Missing, and the binding axis is `Governance` — after absorption the vital
  coordinate is a barrier and leaves the read set. A two-step live fold binds `Vital`.
- the fold gates: an absorbed step carrying a vital reading, a length above
  `MAX_STEPS`, and a null output pointer are all `-14`.

## Decisions taken from evidence, invariant by invariant

The first run of this consumer reported eight failures. Each was decided by reading
the port and re-running, never by loosening the check:

| Invariant | What the evidence said | Decision |
| :--- | :--- | :--- |
| `L == C` refused with `-10` | The record carried `guard = -1` on a **sealed** instrument; the core's seventh check refuses that with `-5` before the step runs. | Expectation error of the consumer: the record must carry seven margins. Fixed the record; the check is kept. |
| A fraction that truncates to zero is `-9` | Same cause — the `-5` of the seventh check fired first. | Same fix. |
| `s02` answers regime `1` | `step` returns the **tag** (`0..=5`) on success, not `0`; `0` is `BufferedAction`. | Expectation error: compare with the tag. |
| `s02` carries fraction `500000` | `payload_0` is an `i64` at offset `0`; the consumer read an `i32` at offset `8`, i.e. the high half of the same number. | Expectation error: read the width the table declares. |
| `a01` answers verdict `0` | The record had `c = 0`, so stage 1 answered `Unparsable` (tag `3`) before any fold. | Expectation error: an accept needs a positive capacity; `c = 120, r = 1000` as the port's own host test uses. |
| The arena base does not overlap the embedded digest | The digest was hashed into the same address the record was then written to — a consumer bug. | Fixed by a scratch address; see the L4 entry below for the store overlap that motivated moving the arena to the top. |
| Binding axis is the vital one | With an absorbed step the read set is `{Epistemic, Governance}` — the core drops the vital coordinate after the barrier. | Expectation error: the absorbed fold binds `Governance` (`3`); a live fold binds `Vital` (`2`). Both are now tested. |
| Terminal reserve is Missing | With the arena on the declared base, the fold read back `0` instead of `-1`; with the arena at the top of memory it reads `-1`. | The old base was inside the module's store; the arena moved to the top, and the ABI now refuses a record there with `-14`. |

## Limits open against the port (not repaired here)

- **L1.** `VerdictOut` carries no `UnparsableReason`, so an `Unparsable` verdict is
  compared on its tag alone.
- **L2.** An axis reading with count `0` and a present margin cannot be written on the
  wire: the ABI derives each axis's count and weakest value from the same seven
  readings.
- **L3.** An axis reading with count `1` and an absent margin cannot be written, for
  the same reason. Rows `a10` and `a11` of the fixture are therefore inexpressible
  and consumer A declares them by id.
- **L4 — retired as a limit, kept as a placement rule.** `align8(__data_end)` is where
  the module's static fold buffer begins (`[StepReading; 1024]`, 64 bytes per step), so
  it is the wrong address for a caller's arena: records and fold would share bytes. The
  ABI now refuses any caller pointer that overlaps the store with `-14`, and this
  consumer (like the adapter) places its arena at the **top of memory**,
  `(memoryBytes - (320 + 3 * 4096)) & ~7`, with 4096 bytes between regions. The
  constant that used to account for the fold (`STATIC_FOLD_BYTES`) is gone: with the
  arena at the top it accounted for a region the arena no longer approaches, and keeping
  it would have been a number carried out of habit. Why the top is right: the module has
  no allocator and no imports, so nothing above its statics is in use, and growth only
  appends pages — so the top is above the store by construction and needs no measurement
  of the layout. Checks that remain: the store begins at `align8(__data_end)` and is
  `8`-aligned; a record written there is refused with `-14`; the arena lies above the
  store start; the whole layout fits inside the instantiated memory.

## Operational note: `pip` and the `/tmp` tmpfs

On this host `TMPDIR` is the `/tmp` tmpfs, which runs near full (80% at the time of
writing). `pip install wasmtime` then fails at unpack time with:

```
OSError: [Errno 122] Disk quota exceeded
```

It installs with `TMPDIR` on the home filesystem, or with `--target` a directory under
`$HOME` and that directory on `PYTHONPATH`. That is a packaging problem of the host,
not an adapter failure and not a port failure; nothing in `adapters/ts` is involved.
