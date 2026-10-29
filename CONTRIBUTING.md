# Contributing

Contributions must follow the governance and verification contract in [`AGENTS.md`](AGENTS.md) and [`docs/PROTOCOL.md`](docs/PROTOCOL.md).

## 1. Epistemological Contract

- Represent every inter-module or inter-agent claim as a structured [`EpistemologicalCard`](crates/stochology-core/src/card.rs) (`cards/*.yaml`) that passes `EpistemologicalCard::audit()`.
- Keep zero external crate dependencies across the 5 workspace crates (`stochology-core`, `stochology-lenses`, `stochology-siga`, `stochology-governance`, `stochology-cli`).
- Run deterministic simulations with [`SplitMix64`](crates/stochology-core/src/rng.rs) and `DEFAULT_SEED = 2026`.

## 2. Verification

Before submitting a change, run the verification suite:

```bash
scripts/verify.sh
```

This script checks formatting (`cargo fmt --all --check`), lints (`cargo clippy --workspace --all-targets -- -D warnings`), tests (`cargo test --workspace`), all 7 laboratories (`labs`), the 8 positive epistemological gates (`gates`), and all 20 negative defect proofs (`gates -- --defect <...>`, the list in `scripts/verify.sh`).

## 3. Licensing

Documentation contributions are licensed under CC BY 4.0 ([`LICENSE-TEXT`](LICENSE-TEXT)) and code contributions under MIT ([`LICENSE`](LICENSE)). Datasets in [`data/`](data/) keep their original licenses, listed in [`data/SOURCES.md`](data/SOURCES.md).
