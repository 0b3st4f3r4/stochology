# Contributing

Contributions must follow the governance and verification contract in [`AGENTS.md`](AGENTS.md) and [`notes/The Ethics of Aim.md`](notes/The%20Ethics%20of%20Aim.md).

## 1. Epistemological & Engineering Contract

- Connect every mathematical, physical, or institutional primitive to the **Sequential Agent–World–Institution Loop** ([`CommunityReserveEnv`](crates/stochology/src/engine.rs)) and the sealed empirical datasets in [`data/`](data/).
- Keep zero external crate dependencies (`std` only) across both workspace crates ([`crates/stochology`](crates/stochology) and [`crates/stochology-cli`](crates/stochology-cli)).
- Run deterministic simulations with [`SplitMix64`](crates/stochology/src/rng.rs) and `DEFAULT_SEED = 2026`.

## 2. Verification

Before submitting a change, run the verification suite:

```bash
./scripts/verify.sh
```

This script checks formatting (`cargo fmt --all -- --check`), lints (`cargo clippy --all-targets -- -D warnings`), unit and integration tests (`cargo test`), test coverage (`>= 90%`), all 9 progressive chapter laboratories (`cargo run --release --bin stochology`), and cryptographic dataset integrity ([`data/SHA256SUMS`](data/SHA256SUMS)).

## 3. Licensing

Documentation contributions are licensed under CC BY 4.0 ([`LICENSE-TEXT`](LICENSE-TEXT)) and code contributions under MIT ([`LICENSE`](LICENSE)). Datasets in [`data/`](data/) keep their original licenses, listed in [`data/SOURCES.md`](data/SOURCES.md).
