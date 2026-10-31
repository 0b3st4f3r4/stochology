# AGENTS.md — Governance, Engineering, and Pedagogical Contract (Stochology)

This file is the binding contract for any agent or contributor working in this repository. **Stochology** is the **science and ethics of aim** (*stókhos*, from Plato's *Philebus* 55e–56c, Hippocratic *sēmeîa*, *kairós*, and *mētis*, and Antipater of Tarsus's archer to Bernoulli's *Ars Conjectandi*) for judgment and decision-making under uncertainty when action must precede the outcome.

Its foundational treatise is [`notes/The Ethics of Aim.md`](notes/The%20Ethics%20of%20Aim.md), and its implementation and textbook are organized 1-to-1 around the **Sequential Agent–World–Institution Loop** (`CommunityReserveEnv`) in **pure Rust (`std` only, zero external dependencies)**.

---

## I. The Nuclear Triad and the Three Collapse Propositions

Stochology unites three historical lineages (Epistemological/Statistical, Physical/Cybernetic, and Critical/Ethical) into a **Nuclear Triad** of three non-substitutable safeguards:

$$\mathcal{T}_{\mathrm{Stochology}} = \bigl(\text{Epistemic Margin},\; \text{Vital Margin},\; \text{Institutional Governance}\bigr), \qquad M_{\mathrm{Leontief}} = \min\bigl(S_{\mathrm{epi}},\, S_{\mathrm{vital}},\, S_{\mathrm{gov}}\bigr)$$

Removing any single axis triggers a specific structural collapse (`CollapseProposition` and `TriadicIntegrity` in `crates/stochology/src/synthesis.rs`, demonstrated on `CommunityReserveEnv` in `crates/stochology/src/engine.rs`):

1. **Collapse Proposition 1 ($\neg\text{Epistemic Margin} \implies \text{Blind Undersizing}$):** Even an institution that maintains physical reserves sizes them against a false point certainty when it ignores non-random missingness ($\rho_{R,Y} \ne 0$), cluster design effects ($\mathrm{DEFF}$), and partial-identification bounds (`StochologicalAgent::without_epistemic_margin()`).
2. **Collapse Proposition 2 ($\neg\text{Vital Margin} \implies \text{Impotent Lucidity Before Ruin}$):** Knowing the exact width of uncertainty while operating at $96\text{--}100\%$ utilization or excess leverage with $\mathrm{TTS} < \mathrm{TTR}$ leads to immediate absorption at the ruin barrier (`StochologicalAgent::epistemic_only()`).
3. **Collapse Proposition 3 ($\neg\text{Institutional Governance} \implies \text{Cynical Capture and Pillaging}$):** Epistemic doubt is weaponized as an agnotological alibi while common vital reserves are pillaged under limited-liability moral hazard, high-powered proxy gaming, and testimonial discounting of frontline warnings (`StochologicalAgent::without_governance()`).

---

## II. 1-to-1 Architecture: Treatise $\leftrightarrow$ Code (`crates/stochology`) $\leftrightarrow$ Book (`book-src/`)

The workspace consists of exactly **two pure-Rust crates**:
- `crates/stochology`: the core mathematical, physical, and institutional library (`std` only).
- `crates/stochology-cli`: the single binary `stochology` that verifies `data/SHA256SUMS` and runs the 9 progressive chapter laboratories over `CommunityReserveEnv` and the 14 real-world CSV datasets in `data/*.csv`.

| Chapter | Textbook (`book-src/`) | Treatise (`notes/The Ethics of Aim.md`) | Module (`crates/stochology/src/`) | Empirical Datasets (`data/*.csv`) |
| :---: | :--- | :--- | :--- | :--- |
| **00** | `00-preface.md` | §I–§II (Genealogy, Nuclear Triad, 3 Collapse Propositions) | `lib.rs` | — |
| **01** | `01-sequential-loop-naive-agent.md` | §II.2 (Sequential Mediation Loop) | `rng.rs`, `sha256.rs`, `engine.rs` (`CommunityReserveEnv`, `NaiveOptimizer`, `StochologicalAgent`) | — |
| **02** | `02-inquiry-openness-multiverse.md` | §III.1 (Filter I · **Tension 1**: Framing, Cromwell, Agnotology vs. Severity, Type III, *e-BH*, Multiverse) | `inquiry.rs` | — |
| **03** | `03-underdetermination-partial-identification.md` | §III.2 (Filter I · **Tension 2**: *e-processes*, Lakatos, Manski, Lee, Peterson, Mayo–Manski) | `identification.rs` | `nsw_experimental.csv`, `cps_observational.csv` |
| **04** | `04-non-identical-asymmetry-heavy-tails.md` | §IV.1 (Filter II · **Tension 3**: Dialectical Penumbra, Ordinal Invariance, Jensen $\Delta_J$, CRPS, Hill $\hat{\alpha}$) | `asymmetry.rs` | `danish_fire.csv`, `nyc_flights_2013.csv`, `nyc_flights_2023.csv` |
| **05** | `05-world-to-archive-big-data-paradox.md` | §IV.2 (Filter II · **Tension 4**: Keynes $V$, Meng $\rho_{R,Y}$, Campbell–Moulton $\mathrm{DEFF}$, Kish, Chao $\hat{n}_{00}$, Bitemporal Vintages) | `archive.rs` | `cps_observational.csv`, `bank_calls.csv`, `rtdsm_routput_vintages.csv` |
| **06** | `06-reactive-worlds-performativity-goodhart.md` | §V.1 (Filter III · **Tension 5**: CORP-PAV, ICP, Kennedy–O'Hagan GP, Tian–Pearl, Perdomo/MacKenzie, Goodhart) | `performativity.rs` | `atlantic_storms.csv` |
| **07** | `07-irreversible-time-vital-margin.md` | §V.2 (Filter III · **Tension 6**: Itô Drain, Baker–McHale Kelly, Canguilhem–Holling, Kingman, $\mathrm{TTS}\ge\mathrm{TTR}$, Wald, Sniedovich) | `vital.rs` | `djia_1885_2023.csv`, `vic_elec.csv` |
| **08** | `08-moral-luck-apt-agency-institutions.md` | §VI (Ethical Mediation · **Tension 7**: Sosa/Baron–Hershey, List–Pettit, Fricker/Beer Algedonic, Holmström–Innes, Chouldechova, Zheng–Meister, `TemporalVault`) | `governance.rs` | `toronto_arrests.csv` |
| **09** | `09-methodological-families-reflexivity.md` | §VII–§VIII (5 Methodological Families, Process Tracing, 3 Reflexive Goodhart Brakes, 4 Refutability Conditions) | `synthesis.rs` | All 14 sealed datasets |

---

## III. Claim Provenance Discipline

Every quantitative claim in `book-src/`, `README.md`, or derived text must carry its honest provenance:
1. **Measured:** Computed by `stochology-cli` on a sealed dataset in `data/*.csv` (name the dataset, e.g., `danish_fire.csv`).
2. **Simulated:** Output of a deterministic `SplitMix64` simulation under a pre-registered seed (e.g., `CommunityReserveEnv` under `DEFAULT_SEED = 2026`).
3. **Assumed / Analytical:** A hand-set parameter or closed-form benchmark input (e.g., $C = 100, L_{\mathrm{inst}} = 200$ in `CostLossFraming`).
4. **Literature (Informative):** An external empirical finding cited from the literature (e.g., Wong et al. 2021 on the Epic Sepsis Model's $88\%$ false discovery rate, or Zheng & Meister 2025 on the $10\text{ bits/s}$ human conscious throughput limit).

---

## IV. Verification & Execution Policy

- **Zero External Dependencies:** Both `crates/stochology` and `crates/stochology-cli` use pure Rust (`std` only) and deterministic `SplitMix64` streams.
- **Verification Pipeline (`scripts/verify.sh`):**
  1. `cargo fmt --all -- --check`
  2. `cargo clippy --all-targets -- -D warnings`
  3. `cargo test`
  4. `cargo run --release --bin stochology`
  5. `sha256sum -c --quiet data/SHA256SUMS`
- **Human Review Protection:** Modifying `data/SHA256SUMS` or `notes/The Ethics of Aim.md` requires explicit human confirmation.
