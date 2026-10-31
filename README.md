<p align="center">
  <img src="book-src/logo.svg" alt="Stochology Emblem" />
</p>

# Stochology — The Science and Ethics of Aim (*Stókhos*) under Uncertainty

**Stochology** is the science and ethics of aim (*stókhos*, from Plato's *Philebus* 55e–56c, Hippocratic *sēmeîa*, *kairós*, and *mētis*, and Antipater of Tarsus's archer to Jakob Bernoulli's *Ars Conjectandi*) for judgment and decision-making under uncertainty when action must precede the outcome.

This repository implements the complete disciplinary program of [`notes/The Ethics of Aim.md`](notes/The%20Ethics%20of%20Aim.md) in **pure Rust (`std` only, zero external dependencies)**, organized 1-to-1 with its 10-chapter textbook (`book-src/`) and interactive browser simulator (`book-src/lab/index.html`) around the **Sequential Agent–World–Institution Loop** (`CommunityReserveEnv`).

---

## 1. The Nuclear Triad and the Three Collapse Propositions

Stochology unites three historical lineages—**Epistemological/Statistical**, **Physical/Cybernetic**, and **Critical/Ethical**—into a single **Nuclear Triad** of non-substitutable safeguards:

$$\mathcal{T}_{\mathrm{Stochology}} = \bigl(\text{Epistemic Margin},\; \text{Vital Margin},\; \text{Institutional Governance}\bigr), \qquad M_{\mathrm{Leontief}} = \min\bigl(S_{\mathrm{epi}},\, S_{\mathrm{vital}},\, S_{\mathrm{gov}}\bigr)$$

| Collapse Proposition | Missing Axis | Structural Pathology |
| :--- | :--- | :--- |
| **Proposition 1** | $\neg\text{Epistemic Margin}$ | **Blind Undersizing:** Reserves are sized on a false point certainty that ignores non-random missingness ($\rho_{R,Y} \ne 0$) and cluster echoes ($\mathrm{DEFF}$). |
| **Proposition 2** | $\neg\text{Vital Margin}$ | **Impotent Lucidity Before Ruin:** The analyst knows the exact width of uncertainty, yet the system operates at $96\text{--}100\%$ utilization or excess leverage with $\mathrm{TTS} < \mathrm{TTR}$ and is absorbed at the ruin barrier. |
| **Proposition 3** | $\neg\text{Institutional Governance}$ | **Cynical Capture and Pillaging:** Epistemic doubt is weaponized as an alibi for paralysis while common vital reserves are pillaged under limited-liability moral hazard, proxy gaming, and testimonial discounting of frontline warnings. |

---

## 2. 1-to-1 Architecture: Treatise $\leftrightarrow$ Rust Crates $\leftrightarrow$ Textbook

The workspace contains two pure-Rust crates:
- [`crates/stochology`](crates/stochology): core mathematical, physical, and institutional library (`std` only).
- [`crates/stochology-cli`](crates/stochology-cli): CLI runner (`stochology`) verifying [`data/SHA256SUMS`](data/SHA256SUMS) and executing the 9 progressive chapter laboratories.

| Ch. | Textbook (`book-src/`) | Treatise (`notes/The Ethics of Aim.md`) | Module (`crates/stochology/src/`) | Empirical Datasets (`data/*.csv`) |
| :-: | :--- | :--- | :--- | :--- |
| **00** | [`00-preface.md`](book-src/00-preface.md) | §I–§II (Genealogy, Nuclear Triad, 3 Collapse Propositions) | [`lib.rs`](crates/stochology/src/lib.rs) | — |
| **01** | [`01-sequential-loop-naive-agent.md`](book-src/01-sequential-loop-naive-agent.md) | §II.2 (Sequential Mediation Loop) | [`rng.rs`](crates/stochology/src/rng.rs), [`sha256.rs`](crates/stochology/src/sha256.rs), [`engine.rs`](crates/stochology/src/engine.rs) | — |
| **02** | [`02-inquiry-openness-multiverse.md`](book-src/02-inquiry-openness-multiverse.md) | §III.1 (Filter I · **Tension 1**: Framing, Cromwell, Agnotology vs. Severity, Type III, *e-BH*, Multiverse) | [`inquiry.rs`](crates/stochology/src/inquiry.rs) | — |
| **03** | [`03-underdetermination-partial-identification.md`](book-src/03-underdetermination-partial-identification.md) | §III.2 (Filter I · **Tension 2**: *e-processes*, Lakatos, Manski, Lee, Peterson, Mayo–Manski) | [`identification.rs`](crates/stochology/src/identification.rs) | `nsw_experimental.csv`, `cps_observational.csv` |
| **04** | [`04-non-identical-asymmetry-heavy-tails.md`](book-src/04-non-identical-asymmetry-heavy-tails.md) | §IV.1 (Filter II · **Tension 3**: Dialectical Penumbra, Ordinal Invariance, Jensen $\Delta_J$, CRPS, Hill $\hat{\alpha}$) | [`asymmetry.rs`](crates/stochology/src/asymmetry.rs) | `danish_fire.csv`, `nyc_flights_2013.csv`, `nyc_flights_2023.csv` |
| **05** | [`05-world-to-archive-big-data-paradox.md`](book-src/05-world-to-archive-big-data-paradox.md) | §IV.2 (Filter II · **Tension 4**: Keynes $V$, Meng $\rho_{R,Y}$, Campbell–Moulton $\mathrm{DEFF}$, Kish, Chao $\hat{n}_{00}$, Bitemporal Vintages) | [`archive.rs`](crates/stochology/src/archive.rs) | `cps_observational.csv`, `bank_calls.csv`, `rtdsm_routput_vintages.csv` |
| **06** | [`06-reactive-worlds-performativity-goodhart.md`](book-src/06-reactive-worlds-performativity-goodhart.md) | §V.1 (Filter III · **Tension 5**: CORP-PAV, ICP, Kennedy–O'Hagan GP, Tian–Pearl, Perdomo/MacKenzie, Goodhart) | [`performativity.rs`](crates/stochology/src/performativity.rs) | `atlantic_storms.csv` |
| **07** | [`07-irreversible-time-vital-margin.md`](book-src/07-irreversible-time-vital-margin.md) | §V.2 (Filter III · **Tension 6**: Itô Drain, Baker–McHale Kelly, Canguilhem–Holling, Kingman, $\mathrm{TTS}\ge\mathrm{TTR}$, Wald, Sniedovich) | [`vital.rs`](crates/stochology/src/vital.rs) | `djia_1885_2023.csv`, `vic_elec.csv` |
| **08** | [`08-moral-luck-apt-agency-institutions.md`](book-src/08-moral-luck-apt-agency-institutions.md) | §VI (Ethical Mediation · **Tension 7**: Sosa/Baron–Hershey, List–Pettit, Fricker/Beer Algedonic, Holmström–Innes, Chouldechova, Zheng–Meister, `TemporalVault`) | [`governance.rs`](crates/stochology/src/governance.rs) | `toronto_arrests.csv` |
| **09** | [`09-methodological-families-reflexivity.md`](book-src/09-methodological-families-reflexivity.md) | §VII–§VIII (5 Methodological Families, Process Tracing, 3 Reflexive Goodhart Brakes, 4 Refutability Conditions) | [`synthesis.rs`](crates/stochology/src/synthesis.rs) | All 14 sealed datasets |

---

## 3. Quick Start & Verification

Run the complete test suite and all 9 chapter laboratories:

```bash
./scripts/verify.sh
```

Or run the CLI directly:

```bash
cargo run --release --bin stochology
cargo run --release --bin stochology -- --chapter 1
```

Sample output (`cargo run --release --bin stochology`):

```text
[OK] Cryptographic manifest data/SHA256SUMS verified (14 sealed datasets).
=== Chapter 1: The Sequential Loop (CommunityReserveEnv, seed=2026, T=24) ===
  NaivePointOptimizer      -> coverage=0.0%, mean_harm=105.80, terminal_reserve=0.00, ruin_step=Some(3)
  StochologicalAgent(Full) -> coverage=100.0%, mean_harm=2.23, terminal_reserve=66.50, ruin_step=None, triad=IntactTriad
=== Chapter 2: Inquiry, Framing & Antidogmatic Openness (Filter I · Tension 1) ===
  Cromwell floor=0.0001 | Framing p*_inst=0.50 vs p*_affected=0.05 (gap=0.45, EVPI_affected=80.0)
  Agnotology vs Severe Power (delta=0.30): n=18 power=0.283 vs n=140 power=0.956 | Type III net=-16.0
  Multiverse (K=6): robust consistency=100% (avg_E=35.0) vs cherry-picked consistency=16.7% (e-BH discoveries=0)
=== Chapter 3: Underdetermination & Partial Identification (Filter I · Tension 2) ===
  LaLonde (nsw_experimental.csv vs cps_observational.csv): experimental_ATE=$1794 vs naive_CPS_diff=$-8498
  Manski bounds=[$5339, $10907] (width=$5568) | Lee attrition bounds=[$321, $2235]
  Peterson survival bounds=[0.55, 0.80] vs naive=0.73 | e-process E_t=8.53 (Lakatos prog=8.53 vs deg=0.66)
=== Chapter 4: Asymmetry, Scale Invariance & Heavy Tails (Filter II · Tension 3) ===
  Danish Fire (danish_fire.csv, n=2167): mean=3.39, median=1.78, q95=9.97, Jensen_gap=72.34, Hill_alpha=1.446 (FiniteMeanInfiniteVariance)
  NYC Flights (nyc_flights_2013/2023.csv): delay Jensen_gap=2536.0 min^2, penumbra_mass(10..20m)=8.2%, ordinal_err=0.0e0
=== Chapter 5: From World to Archive & Big Data Paradox (Filter II · Tension 4) ===
  Meng Big Data (n=2.3M / N=10M, rho=-0.05): n_eff=119.5 | CPS clusters (n=15992): DEFF=13.00, n_eff=1230.2, Keynes_V=0.330
  Bank Calls (bank_calls.csv): DEFF=22.48, n_eff=1233.0 | Kish floor n_eff=12.3 | Chao uncounted n00=450
  Real-Time Vintages (rtdsm_routput_vintages.csv, Q=241): mean_revision=1.47 pp, max_revision=8.02 pp, sign_flips=6.6%
=== Chapter 6: Reactive Worlds, Performativity & Goodhart (Filter III · Tension 5) ===
  Atlantic Storms CORP-PAV (atlantic_storms.csv, n=20778): BS=0.2691 = REL(0.2114) - RES(0.1310) + UNC(0.1887), closure_err=3.7e-15
  Storms ICP across 3 seasons: physical_pressure_slope_range=0.009 vs proxy_latitude_slope_range=0.076
  Perdomo/MacKenzie (BarnesianSelfFulfilling): theta_PS=1.333 vs theta_PO=1.095 (risk_PO=0.249 < risk_PS=0.263) | KOH std_in=0.050 -> std_out=0.500
  Tian-Pearl PN=[0.75, 1.00], PNS=[0.50, 0.70] | Goodhart collapsed=true
=== Chapter 7: Irreversible Time, Non-Ergodicity & Vital Margin (Filter III · Tension 6) ===
  DJIA (djia_1885_2023.csv, n=37930): ensemble_mu=0.000243, time_growth_g=0.000186, Kelly_f*=2.12
  Victoria Grid (vic_elec.csv, n=52608): mean=4665 MW, q95=6106 MW | Kingman wait: rho=0.78 -> 3.55x vs rho=0.97 -> 32.33x
  Canguilhem-Holling basin: fresh=6.40 -> worn(McEwen)=2.96 | Scheffer CSD rho1: 0.578 -> 0.791
  Wald 1943 lethality: wing=7.2% vs engine=84.0% | Sniedovich No-Man's-Land mirage=true
=== Chapter 8: Moral Luck, Apt Agency & Institutional Design (Tension 7) ===
  Toronto Arrests Chouldechova (toronto_arrests.csv): p_A=0.393, p_B=0.112 -> FPR_A/FPR_B disparity=5.13x
  List-Pettit Discursive Dilemma: premise_maj=true vs constitutional_veto=false | Fricker algedonic delay: 5 -> 11 shifts (TTS=8)
  Sosa/Baron-Hershey grade=AdroitExAnteUnluckyOutcome (bias_removed=0.45) | Holmstrom structural_liability=83.3% vs frontline=6.2%
  Zheng-Meister & Gigerenzer Alerts (N=1000): noisy FDR=93.9% vs filtered PPV=52.7% (16 true alerts, 15 false alarms)
=== Chapter 9: Methodological Families, Process Tracing & Reflexive Brakes ===
  Process Tracing (Family 5): prior=0.25 -> post=0.925 (+15.7 dB), Keynes_V: 0.20 -> 0.56, hoop_refutable=true
  Three Reflexive Goodhart Brakes: sharp_RES=0.250 vs lazy_RES=0.000 | optimal_welfare=2.55 > hoarding=-11.97 | fast_path=1.5 <= TTS=4.0
  Three Collapse Propositions Verified: P1=Proposition1BlindUndersizing, P2=Proposition2ImpotentLucidity, P3=Proposition3CynicalCapture -> Full=IntactTriad
=== ALL 9 STOCHOLOGICAL CHAPTER LABORATORIES & INVARIANTS VERIFIED (EXIT CODE 0) ===
```
