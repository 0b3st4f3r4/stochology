# `stochology-cli`

Executable CLI laboratories (`labs`) and epistemological gates (`gates`) for the **Stochology — CAPIBA Protocol** (*Calibrated, Anytime-valid, Performative-aware, Invariant, Bitemporal Auditing Protocol*) in pure Rust (`std` only).

## Binaries

- **`labs`**: Runs the 7 methodological and empirical laboratories (`--all`, `--layers`, `--uncertainty`, `--lenses`, `--invariants`, `--siga`, `--governance`, `--real-data`).
- **`gates`**: Enforces the 8 epistemological gates (`G1`–`G8`), portable external YAML card auditing (`--audit-card <path.yaml>`), multi-seed acceptance registry verification (`--print-acceptance-registry`), and all 27 negative defect proofs (`--defect <name>`, which must exit with code `1`).

## Usage

From the repository root (where the immutable real datasets `data/*.csv`, pre-registration manifest `data/SHA256SUMS`, and `cards/*.yaml` reside):

```bash
cargo run --release --bin labs -- --all
cargo run --release --bin gates
cargo run --release --bin gates -- --audit-card cards/card_01_atlantic_storms_evacuation.yaml
```

## License

MIT License — Copyright (c) 2026 Silvano Neto.
