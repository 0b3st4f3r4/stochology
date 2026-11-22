# stoch-crdt

Zero-float, dependency-free causal-conservative join-semilattices, revocable
rehabilitation, coordinator-free cross-witness reconciliation, and delay-tolerant
store-carry-forward custody queues (`#![no_std] + alloc`, `#![deny(unsafe_code)]`).

This crate sits in the Pure-TCB workspace (`C5`, `C5b`, `C6` gates) and carries zero
external dependencies. Like every executable artifact in this repository, it is
subordinate to the chapters in `book-src/`.

## Main Types

- `JoinSemilattice`, `SpentCapabilities` (`G-Set` preventing capability replay), and
  `LeontiefEpochState` (`RFDE-CRDT` epoch-dominant component-wise minimum).
- `ExilePhase`, `RehabPolicy`, and `RehabLattice` (clock-free monotonic rehabilitation
  with exponential pharmacovigilance backoff and zero probationary witness weight).
- `SignedWitness`, `EquivocationCertificate`, `WitnessOutcome`, `cross_witness_reconcile`,
  and `cross_witness_median` (self-report exclusion, probationary zero-weight filtering,
  and self-verifying equivocation certificates for partitioned conflicts).
- `DegreeLieProof` and `StoreCarryQueue` (Kingman backpressure shedding non-urgent
  telemetry while preserving headroom for urgent safety signals).

## Tests

```sh
TMPDIR="${TMPDIR:-$HOME/.cache/rs-tmp}" cargo test --offline -p stoch-crdt
```
