# Consumer A — Node over the TypeScript adapter (a test double)

**These two consumers are test doubles built to rehearse the ABI; the trigger of
PLAN §4.2 remains unmet.**

The rule, in the language the team uses: these two consumers are test doubles created
to rehearse the ABI; the trigger of PLAN §4.2 remains unmet. Neither is a product, a
host application, or evidence that a real consumer exists.

```
node consumers/node/run.mjs [--vectors PATH]
```

It reads `crates/stoch-core/vectors.kv`, writes every record into the caller's arena through
`adapters/ts/adapter.ts`, calls `step`, `accept`, `diagnose` and `seal`, prints one
`id regime=N` / `id verdict=N` line per row, and compares a canonical line with the
row's declaration. A divergence prints `MISMATCH id` with both lines and exits 1.

Declared limits of this consumer:

- **L1 — the boundary does not carry the reason.** `VerdictOut` has no room for an
  `UnparsableReason`, so an `Unparsable` row is compared on its tag alone and the
  reason is printed as `-`. The Rust harness compares the reason because it sees the
  `Verdict` value; the wire cannot.
- **L2 / L3 — two rows cannot be written on this wire.** `a10` declares an axis with
  count 0 and a present reading, `a11` an axis with count 1 and an absent reading. The
  ABI derives each axis's count from the seven readings, so neither declaration is
  expressible; both are `SKIP`ped, and the set of skipped ids is declared (`a10`,
  `a11`) rather than counted, so a new inexpressible row cannot hide in a matching
  total.

- **The arena is the top of memory, not a measured floor.** The record is placed at
  `memoryBytes - (464 + 320)`, re-derived on every call, so a grown memory moves it. The
  module's statics live inside the image it was instantiated with and growth only appends
  pages, so a base at or above that image cannot overlap the store — which is also why
  `align8(__data_end)` is the wrong place to start: the module folds into a
  `[StepReading; 1024]` buffer that begins exactly there, and the ABI now refuses a caller
  pointer that overlaps it with `-14`. Nothing above the store is in use because the
  module has no allocator and no imports, and the top needs no measurement of the layout.

The consumer also checks that the digest the module carries is the digest of the
fixture on disk before it compares a single row: a fixture edited without rebuilding
the module is a divergence, not a silently different test — and that path exits `1`,
which was verified by running it against a corrupted copy of the fixture (see the
transcript in the task report: `fixture digest diverges: module efe2bad3…, file
5f6dbfdb…`, exit `1`).
