# Host Workspace (`crates/host-workspace`)

Excluded from the root Pure-TCB workspace (`Cargo.toml`), this workspace holds host-side
tooling, durable custody, governance cryptography, property-based falsification, and
Wasmtime runtime/AOT verification.

## Members

- `stoch-custody` — `redb` (2PC + `Durability::Immediate`) bitemporal two-tape custody
  ledger with per-read `BLAKE3` integrity verification, monotonic anti-replay tables
  (`SPENT_UCANS`, `DELEGATION_SEQS`), `R0` snapshots, and escrowed `REOPEN_LOG`.
- `stoch-host` — ephemeral 3-round DKG + 2-round FROST (`frost-ed25519`), `HyParView`
  two-phase membership rewiring, peaceful lineage branching, two-level OCap firewall,
  `HsmBackend` / `StrictHsmSigner` (`verify_strict` mandatory; simulated C-ABI FFI mock
  gated behind `--features simulated-hsm-ffi`), `SolarWatchdogEngine`, lock-free
  OpenMetrics, `<= 10 bits/s` `Level1Cue`, constitutional `EmergencyLease`, and local
  autonomous `Regime III-B` survival halt evaluation.
- `stoch-aot` — offline deterministic Wasmtime `.cwasm` precompiler (`stoch_aot_repro`)
  and independent Wasmtime runtime parity tests against `stoch_abi.wasm`.
- `stoch-proptest` — randomized property-based falsification suite covering `stoch-core`,
  `stoch-abi`, `stoch-math`, `stoch-crdt`, `stoch-custody`, and `stoch-host`.

## Verification

```sh
cd crates/host-workspace
TMPDIR="${TMPDIR:-$HOME/.cache/rs-tmp}" cargo test --offline --locked
TMPDIR="${TMPDIR:-$HOME/.cache/rs-tmp}" cargo test --offline --locked -p stoch-host --features simulated-hsm-ffi
```
