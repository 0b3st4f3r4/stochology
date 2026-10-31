# `stochology-cli`

Command-line runner for **Stochology**, executing the 9 progressive chapter laboratories over `CommunityReserveEnv` and the sealed real-world datasets in `data/*.csv`.

## Usage

- Run all 9 progressive chapter laboratories and verify `data/SHA256SUMS`:
  ```bash
  cargo run --release --bin stochology
  ```
- Run a single chapter laboratory (`1` to `9`):
  ```bash
  cargo run --release --bin stochology -- --chapter 1
  ```
