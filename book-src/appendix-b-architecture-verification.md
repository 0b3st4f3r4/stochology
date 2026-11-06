# Appendix B — Architecture, Crates, and Verification

Stochology is implemented in **pure Rust (`std` only, zero external Cargo crates; floating-point elementary functions `exp`, `ln`, and `sqrt` delegate to the toolchain's IEEE 754 `libm` within 1–2 ULP as documented in [`kernel.rs`](crates/stochology/src/kernel.rs) and [`rust-toolchain.toml`](rust-toolchain.toml))** across two workspace crates: [`crates/stochology`](crates/stochology/src/lib.rs) (the mathematical and institutional library) and [`crates/stochology-cli`](crates/stochology-cli/src/lib.rs) (the dataset loader, the SHA-256 manifest verifier, and the nine-laboratory runner for Chapters 42–50).

![Pure-Rust Two-Crate Workspace](figures/fig-12-workspace-crates.svg)

---

## B.1 1-to-1 Mapping Between the Nine Framework Chapters (`42`–`50`), Contiguous Nuclear Triad Axes, and Rust Modules

| Chapter | Nuclear Triad Axis & Mediating Filter | Library Module (`crates/stochology/src/`) | Primary Exported Canonical Types & Functions |
| :---: | :--- | :--- | :--- |
| **00** | Genealogy, Nuclear Triad, 3 Collapse Propositions, 3 Canonical Operators | [`lib.rs`](crates/stochology/src/lib.rs) | Crate-level overview and canonical re-exports |
| **42** | Sequential Agent–World–Institution Loop | [`kernel.rs`](crates/stochology/src/kernel.rs), [`engine.rs`](crates/stochology/src/engine.rs) | `SplitMix64`, `sha256_hex`, `imbens_manski_critical_value`, `CommunityReserveEnv`, `AgentPolicy`, `NaiveOptimizer`, `CompensatoryBufferAgent`, `StochologicalAgent`, `MultiSeedSummary` |
| **43** | **Filter I · Axis 1 ($S_{\mathrm{epi}}$):** Inquiry, Framing & Antidogmatic Openness | [`epistemic.rs`](crates/stochology/src/epistemic.rs) | `EpistemicMarginState::from_inquiry_framing`, `CognitiveFramingRegime`, `pauker_kassirer_threshold`, `cromwell_clamp`, `e_bh_discoveries` |
| **44** | **Filter I · Axis 1 ($S_{\mathrm{epi}}$):** Underdetermination & Partial Identification | [`epistemic.rs`](crates/stochology/src/epistemic.rs) | `EpistemicMarginState::from_ate_bounds`, `from_peterson_competing_risks_bounds`, `from_censoring_bounds` (two-arm Manski & Lee ATE bounds, Peterson Fréchet & censoring bounds, Ville $e$-processes, Lakatos shift) |
| **45** | **Filter I · Axis 1 ($S_{\mathrm{epi}}$):** World to Archive & Big Data Paradox | [`epistemic.rs`](crates/stochology/src/epistemic.rs) | `EpistemicMarginState::from_archive_clusters`, `from_meng_big_data` (Keynes $V$, Meng $\rho_{R,Y}$, Campbell–Moulton $\mathrm{DEFF}$, Kish $n_{\mathrm{eff}}$, Chao $\hat{n}_{00}$), `BitemporalRecord` |
| **46** | **Filter II · Axis 2 ($S_{\mathrm{vital}}$):** Asymmetry, Scale Invariance & Heavy Tails | [`vital.rs`](crates/stochology/src/vital.rs) | `VitalMarginState::from_heavy_tails_and_asymmetry`, `TailMomentRegime`, `HollingTailRuptureAudit`, `hill_stability_across_k` (dialectical penumbra, ordinal invariance, Jensen $\Delta_J$, CRPS, Hill $\hat{\alpha}$, $\mathrm{DEFF}_{\mathrm{tail}}$ Holling rupture coupling) |
| **47** | **Filter II · Axis 2 ($S_{\mathrm{vital}}$):** Irreversible Time, Non-Ergodicity & Vital Margin | [`vital.rs`](crates/stochology/src/vital.rs) | `VitalMarginState::from_queue_basin_and_survival`, `from_queue_basin_and_derived_ttr`, `from_non_ergodic_returns`, `VitalMarginState::from_regime_aware_kelly`, `KellySizingRegime`, `WaldSurvivorshipAudit`, `SniedovichInfoGapAudit` (Itô drain, Baker–McHale Kelly vs. WDRO-CVaR Kelly, Kingman $G/G/1$ wait, $\mathrm{TTS}_{\mathrm{survival}} \ge \mathrm{TTR}_{\mathrm{recovery}}$, Canguilhem–Holling, Wald, Sniedovich) |
| **48** | **Filter III · Axis 3 ($S_{\mathrm{gov}}$):** Reactive Worlds, Performativity & Goodhart | [`governance.rs`](crates/stochology/src/governance.rs) | `GovernanceState::from_ex_ante_gate`, `GovernanceState::evaluate_corp_pav_optimism_cv`, `CausalEnvironment`, `IcpIdentificationStatus`, `MacKenzieRegime`, `GoodhartVariant` (CORP-PAV, split-half CV optimism debiasing, subset-intersection ICP, Kennedy–O'Hagan GP, Tian–Pearl PN/PS/PNS, Perdomo) |
| **49** | **Filter III · Axis 3 ($S_{\mathrm{gov}}$):** Moral Luck, Apt Agency & Institutional Design | [`governance.rs`](crates/stochology/src/governance.rs) | `GovernanceState::from_real_time_and_ex_post_gates`, `GovernanceState::evaluate_list_pettit_votes`, `TelicGrade` (`SosaJudgmentGrade`), `AlgedonicChannel`, `BatesonDoubleBindAudit`, `CognitiveAlertFilter`, `ContractAndLiabilityAudit`, `TemporalVault`, `AuditLog`, `AuditLogEntry`, `KairosSynchronousPanel`, `ZhengMeisterQueueAudit` (List–Pettit, Fricker, Holmström–Innes, Bateson, Simon–Girard, Chouldechova, Zheng–Meister two-tier cognitive queue, Gigerenzer) |
| **50** | **Triadic Synthesis ($M_{\mathrm{Leontief}}$):** 5 Families, Process Tracing & Reflexivity | [`synthesis.rs`](crates/stochology/src/synthesis.rs) | `CollapseProposition`, `TriadicIntegrity`, `StructuralCollapseProofAudit`, `DiagnosticClue`, `ProcessTracingTestKind`, `ProcessTracingInference`, `ReflexiveBrakeAudit`, `LeontiefVsMultiplicativeAudit`, `PreregisteredFalsifier` |

---

## B.2 The Grammar of Aim — Syntax Layer and Acceptance Predicate

The treatise calls Stochology a *transversal grammar* (§7.1). That phrase is formalized in its technical sense — alphabet, productions, decision procedure — by [`notes/The Grammar of Aim.md`](../notes/The%20Grammar%20of%20Aim.md) and executed by [`grammar.rs`](crates/stochology/src/grammar.rs), the only module that is not chapter-bound: it sits over Chapters 01–09 at once.

The module owns **syntax and acceptance, never dynamics**. Its lexicon is the one the crate already uses: three planes, the three axes of the Nuclear Triad, and the **seven guard sites**, one per constitutive tension, partitioned $3/2/2$ over $(S_{\mathrm{epi}}, S_{\mathrm{vital}}, S_{\mathrm{gov}})$. A step of the loop is a **shot**: a control tape (*télos*, `AgentDecision`) paired with a contingency tape (*skópos*, `StepOutcome`). A guarded trajectory is a `Sentence`, sealed by a `TemporalVault` over a dependency-free canonical payload, so a derivation cannot be revised after its verdict and still presented as the same derivation.

`judge` returns exactly four verdicts, in a fixed order of evaluation: `Interior`, `Boundary` (inside the near-critical band above $\tau$), `Collapse { proposition, at, step_t }`, and `Unparsable`. The third reuses `CollapseProposition`, so the grammar inherits the eight-cell cube instead of inventing a parallel taxonomy; the fourth is a mandatory **quarantine**, because a grammar whose only failure mode is collapse would diagnose every phenomenon it cannot parse as a catastrophe — the precise dogmatic error the treatise spends three sections diagnosing. A non-finite margin is quarantined rather than compared, and an unguarded transit is not a weak derivation but a non-derivation.

The same productions are read in the opposite direction by two generative regimes: `enumerate` covers the finite guard grid exhaustively (`[Assumed]` for the finite grid enumeration), and `shortest_violation` returns the shortest single-site counterexample against the declared thresholds (`[Simulated]` when quoted on a concrete grid). Coverage interrogates the language; the adversarial regime interrogates the thresholds themselves, before any field study begins.

**Non-commutation property (`[Assumed]`).** The acceptance predicate reads the control tape and the guard margins, and never the contingency tape. Hence no admissible verdict is a function of *skopós*: an accurate hit cannot buy `Interior` from margins that do not support it. Antipater's separation of *télos* and *skópós* is thereby a theorem about the predicate rather than a remark about archers.

**Parity.** [`tests/grammar_spec.rs`](crates/stochology/tests/grammar_spec.rs) parses the note, resolves its anchor block against the sources, and requires agreement with `TrajectoryReport::triadic_integrity.diagnose` on all eight factorial cells. The note and the module cannot drift apart, and the grammar cannot quietly compute a margin of its own.

## B.3 Running the Verification Pipeline (`scripts/verify.sh`)

The workspace verification script [`scripts/verify.sh`](scripts/verify.sh) executes the complete quality and empirical verification pipeline in six numbered stages (plus the audit-report sub-stage `6b`):

1. **`cargo fmt --all -- --check`:** Verifies standard Rust formatting across both crates.
2. **`cargo clippy --all-targets -- -D warnings`:** Enforces zero Clippy warnings across the workspace and every test target.
3. **`cargo test`:** Runs all unit and integration tests across `crates/stochology` and `crates/stochology-cli` (FIPS 180-4 SHA-256 vectors, PAV isotonic regression, Cholesky SPD solver, `CommunityReserveEnv` $2\times 2\times 2$ factorial, `book_anchors`, `grammar_spec`, `multilayer_spec`).
4. **`cargo llvm-cov --workspace --summary-only --fail-under-lines 90 --fail-under-regions 90 --fail-under-functions 90`:** Gates aggregate coverage on three metrics, not lines alone.
5. **`cargo run --release --bin stochology`:** Cryptographically verifies all 14 real-world CSV datasets (`[Measured]`) against [`data/SHA256SUMS`](data/SHA256SUMS) and executes the nine CLI laboratories (Chapters 42–50) (`[Simulated]`, `[Assumed]`, `[Literature]`):
```text
[SEAL VERIFIED] All 14 empirical datasets in data/SHA256SUMS match their SHA-256 digests and 14 upstream schema contracts.
=== CHAPTER 1: THE SEQUENTIAL LOOP & THE NAIVE AGENT ===
  Environment [Simulated]: CommunityReserveEnv (seed = 2026, horizon = 24 shifts, R_0 = 45.0)
  NaiveOptimizer          -> coverage =  16.7%, mean_harm = 125.83, terminal_reserve =   0.00, ruin_step = Some(2)
  Prop 1 Degrad (No E-Mar)-> coverage =  25.0%, mean_harm =   4.49, terminal_reserve =  27.21, ruin_step = None
  Collapse 2 (No V-Margin)-> coverage =  95.8%, mean_harm =  77.19, terminal_reserve =   0.00, ruin_step = Some(10)
  Collapse 3 (No Gov.)    -> coverage =  95.8%, mean_harm =  68.01, terminal_reserve =   0.00, ruin_step = Some(12)
  StochologicalAgent(Full)-> coverage =  95.8% (shock_cov =  75.0%), mean_harm =   1.59, terminal_reserve =  58.17, ruin_step = None
=== CHAPTER 2: FILTER I (S_EPI) — INQUIRY, OPENNESS & MULTIVERSE ===
  Cost-Loss Thresholds [Assumed]: institutional p* = 0.588 (EVPI=12.60) vs affected-citizen p* = 0.147 (EVPI=82.00), gap = 0.441
  Mayo Severe Power [Assumed]: 1.000 | Type III Error: true (global_net=-65.0) | Simon Framing: StrategicPremiseRigging vs CommunicativeInfluence (PRI = 0.864 vs 0.000, threshold = 0.20)
  Multiverse & e-BH [Assumed]: consistency = 83.3%, interval = [1.38, 1.61], e-BH discoveries = 5
=== CHAPTER 3: FILTER I (S_EPI) — UNDERDETERMINATION & PARTIAL IDENTIFICATION ===
  LaLonde Benchmark [Measured] (data/nsw_experimental.csv vs data/cps_observational.csv):
    Experimental RCT ATE [Measured, zero survey attrition] = $1794 (n_t=185, n_c=260) vs Naive CPS Diff = $-8498 (n_cps=15992)
    Lee (2009) Positive-Wage Selection Bounds [Measured, re78>0: 140/185 vs 168/260]: [$-1142, $2621] (width=$3763)
    Manski Worst-Case Two-Arm ATE Bounds [Assumed: 15/200 & 30/290 attrition]: [$-487, $3439] (width=$3926, Imbens-Manski 95% CI=[$-1639, $4568])
    Lee Monotone Attrition Two-Arm ATE Bounds [Assumed: 15/200 & 30/290 attrition]: [$872, $1996] (width=$1123)
    Censoring worst-case survival bounds [Assumed]: [0.750, 0.860] (KM naive=0.843)
    Peterson (1976) competing-risk Frechet bounds [Assumed]: [0.750, 0.860] (indep=0.765, width=0.110)
    Ville e-Process [Assumed]: terminal E_T = 2164.62 (33.4 decibans), anytime CI = [-0.03, 2.75]
=== CHAPTER 4: FILTER I (S_EPI) — FROM WORLD TO ARCHIVE & THE BIG DATA PARADOX ===
  Canonical Operator I (Harmonic n_eff & Keynes Weight V = n_eff / (n_eff + n_0)):
  1. Meng Big Data Paradox [Assumed] (N = 10M, n = 2.3M, rho_RY = -0.05): n_eff = 119.5, meng_error = -0.0457, V = 0.046
  2. Demographic Cluster DEFF on real re78 [Measured] (data/cps_observational.csv, [Assumed n_0=25.565]): DEFF = 10.37, n_eff = 1542.0, V = 0.984
  3. Temporal Cohort DEFF [Measured] (data/bank_calls.csv, [Assumed n_0=100.0]): DEFF = 8.19, n_eff = 3384.5, V = 0.971
  4. Kish Weighting Floor [Assumed]: DEFF_Kish = 2.78, n_eff_floor = 360.0 | Chao Unseen Dark Archive [Assumed]: n00_hat = 751.6, N_Chao_LB = 2237.1, U_Chao_95 = 2719.9
  5. Bitemporal Vintages [Measured] (data/rtdsm_routput_vintages.csv, Q=241 evaluated quarters across 244 vintages): mean_rev = 1.47 pp, max_rev = 8.02 pp, sign_flips = 6.6% (causal_view_len=1)
=== CHAPTER 5: FILTER II (S_VITAL) — ASYMMETRY, SCALE INVARIANCE & HEAVY TAILS ===
  Danish Fire Losses [Measured] (data/danish_fire.csv, n = 2167):
    Hill alpha_hat = 1.39 (k=150, 95% CI=[1.17, 1.61], k-stability [50..250] = [1.36, 1.87], regime = FiniteMeanInfiniteVariance, median = 1.78, q95 = 9.97)
    Jensen Gap Delta_J = E[phi(X)] - phi(E[X]) = 1.015 (E[phi(X)] = 1.613 vs phi(E[X]) = 0.598)
    Empirical CRPS = 1.458 | Ordinal Invariance Error = 0.0000 (Mean Ranking Reversed = true) | Penumbra Mass = 0.396
    Hill-Holling DEFF_tail Coupling [Simulated] (DEFF=10.1, k_eff=11.9): P_true=0.0166, E[P_plugin]=0.0219 (Jensen +32.3%), P_debiased=0.0119, CI90=[0.00083, 0.06704] (80.3x)
  NYC Flights Conditional Delay>0 Tail Index [Measured, k=n+/20 on non-NA rows]: 2013 alpha_hat = 2.89 (95% CI=[2.78, 3.00], k=2616, n=138375, n+=52330, total_rows=141193, skipped_NA=2818, penumbra_[10,30]=12.7%) vs 2023 alpha_hat = 2.27 (95% CI=[2.18, 2.36], k=2430, n=173566, n+=48602, total_rows=176739, skipped_NA=3173, penumbra_[10,30]=8.6%) | OOT H1 (2013=3.13 [k=4033, n=188971, n+=80674, total=195583, NA=6612], 2023=2.39 [k=4648, n=249252, n+=92972, total=258613, NA=9361]) & OOD Houston 2011=2.41 (k=5346, n=223874, n+=106920, total=224591, NA=717)
=== CHAPTER 6: FILTER II (S_VITAL) — IRREVERSIBLE TIME, NON-ERGODICITY & VITAL MARGIN ===
  Canonical Operator II (Convexity, Time-Average Growth & Survival Inequality TTS_survival >= TTR_recovery):
  1. DJIA Non-Ergodic Trajectory [Measured] (data/djia_1885_2023.csv, T = 37930 trading days, emp_alpha_loss = 2.83):
     Arithmetic Ensemble Drift = +0.000243/day vs Time-Average Growth = +0.000186/day (Itô drain = 23.6%)
     Plug-in Kelly = 2.12 vs Baker-McHale Epistemic Kelly = 1.99 (Shrinkage = 0.9416)
     Heavy-Tail Kelly Regime Selector [Assumed] (alpha=1.65 <= 2.2): regime = TailRobustWdroCvarKelly, naive_quadratic_f = 1.0000 vs WDRO_CVaR95_f = 0.0314
  2. Victoria Electricity Grid [Measured] (data/vic_elec.csv, n = 52608 half-hours, q50=4635 MW, q95=6106 MW, q99=6981 MW, max=9345 MW, ramp_p99=897 MW/h, scale=1.18/1.05 [Assumed]):
     Stressed [Measured quantiles + Assumed ODE h=1.80,K=10] (rho=96.9%): Kingman Wait = 31.2x (G/G/1 E[W_q]=16.17h, var_factor=0.52, rho1_worn=0.732), Basin D_t/D_0 = 0.46, TTS_survival = 2.6h < TTR_recovery = 8.0h (survives=false)
     Buffered [Measured quantiles + Assumed ODE h=1.80,K=10] (rho=47.2%): Kingman Wait = 0.9x (G/G/1 E[W_q]=0.46h, var_factor=0.52, rho1_worn=0.593), Basin D_t/D_0 = 0.92, TTS_survival = 16.5h >= TTR_recovery = 4.8h (survives=true)
  3. Survivorship & Robustness: Wald Latent 2x2 Solver [Assumed] = true (reconstructed M_A=18, M_B=210 -> Zone B 84.0% > Zone A 7.2%) | Empirical NSW Survivor Bias [Measured] (re78>0 only): full=$1794 vs survivor=$1341 (bias=$-453) | Sniedovich Mirage [Assumed] = true
=== CHAPTER 7: FILTER III (S_GOV) — REACTIVE WORLDS, PERFORMATIVITY & GOODHART ===
  Canonical Operator III — Gate 1 (Ex-Ante Calibration, Causal Invariance & Performative Gaming Guard):
  1. CORP-PAV +6h Lead-Time Storm Forecast Brier Decomposition [Measured] (data/atlantic_storms.csv, n_transitions = 19025 from 20778 total rows, 1753 track-terminal rows [1752 unpaired + 1 tail]):
     Total Brier = 0.0731 = REL (0.0028) - RES (0.1171) + UNC (0.1873) [closure_err = 4.62e-15]
     Finite-Sample CORP-PAV & ICP Guard [Measured]: b_PAV_CV = 0.0002, REL_debiased = 0.0026, ICP(n_eff/env=184.5) = IdentifiedInvariantSet (benchmark n_eff/env=30 -> EpistemicUnderpoweredSet [Assumed])
  2. Multi-Regime Invariant Causal Prediction (ICP) [Measured]: physical slope range = 0.009 < spurious proxy range = 0.076 (accepted_subsets = 2, S_hat_mask = 0b01)
  3. Kennedy-O'Hagan GP Discrepancy [Measured tercile anchors]: in_sample (mean=-0.0283, std=0.044) vs out_of_sample (mean=-0.0000, std=0.500) | Tian-Pearl [Assumed] PN=[0.75, 1.00], PS=[0.50, 0.75], PNS=[0.50, 0.70]
  4. Perdomo Performative Stability vs Optimality [Assumed]: theta_PS = 1.333 (risk=0.263) vs theta_PO = 1.095 (risk=0.249, MacKenzie = BarnesianSelfFulfilling)
  5. Goodhart Metric Collapse [Assumed]: collapsed = true, variant = Adversarial
=== CHAPTER 8: FILTER III (S_GOV) — MORAL LUCK, APT AGENCY & INSTITUTIONAL DESIGN ===
  Canonical Operator III — Gate 2 (Real-Time S3* Algedonic Veto) & Gate 3 (Ex-Post Responsibility):
  1. Sosa / Baron-Hershey Telic Evaluation [Assumed]: grade = AdroitExAnteUnluckyOutcome, eliminated_outcome_bias = +0.45, vault_confirmatory = true (tamper_rejected=true, replay_rejected=true)
  2. List-Pettit & Beer S3* Algedonic Channel [Assumed]: premise_majority = true, conjunctive = false, discursive_dilemma = true
     Undiscounted TTR_recovery = 5 <= TTS_survival = 8 vs Discounted TTR_recovery = 12 > TTS_survival = 8 (fatal_delay=true)
  3. Holmstrom-Innes Liability & Simon-Girard Scapegoating [Assumed]: structural_share = 83.3%, scapegoat_transfer = 78.7% (blame_inversion_ratio = 13.6x), Bateson DB_t = 0.51 (brake_on = 0.00)
  4. Chouldechova Fairness Trilemma [Measured] (data/toronto_arrests.csv, n_all = 5226, n_matched = 891, n_poles = 3577): full marginal prev_Black = 0.259, prev_White = 0.142, odds_FPR_ratio = 2.11x | matched (Unemp+Citizen) prev_Black = 0.354, prev_White = 0.263, odds_FPR_ratio = 1.54x | intersectional poles prev_A = 0.393 (FPR_A=0.278), prev_B = 0.112 (FPR_B=0.054), FPR_A/FPR_B = 5.13x
     Empirical Custody-Risk Classifier [Assumed rule checks>=3 on Measured data/toronto_arrests.csv]: PPV_B=0.365/PPV_W=0.267, FNR_B=0.324/FNR_W=0.438, FPR_Black=0.409 vs FPR_White=0.256 (empirical FPR_ratio=1.60x, Chouldechova identity closure_err=1.11e-16)
  5. Zheng-Meister 10 bits/s & Gigerenzer Natural Frequencies [Assumed]: noisy FDR = 93.9% -> filtered PPV = 52.7% (16 true alerts vs 15 false alarms / 1,000 shifts)
     Two-Tier Cognitive Queue (TTS=45s): Kairos(30b,6s) rho=0.12, TTR=18.5s (ok=true) | Card(120b,18s) rho=0.48, TTR=40.8s | Full(240b,36s) rho=0.96, TTR=759.0s (ok=false)
=== CHAPTER 9: METHODOLOGICAL FAMILIES, PROCESS TRACING & REFLEXIVITY ===
  2x2x2 Factorial Ablation of the Nuclear Triad [Simulated] (M_Leontief = min(S_epi, S_vital, S_gov)):
    [E=1 V=1 G=1] StochologicalAgent(FullTriad:111)            -> M_Leontief=0.814 (M_arith=0.895), collapse=IntactTriad                 , cov= 95.8% (shock= 75.0%), harm=  1.59 (MC100=  1.55±0.01 [  1.44,   1.69], ruin=  0.0%), R_24= 58.17
    [E=0 V=1 G=1] DefectiveAgent(NoEpistemicMargin->Prop1:011) -> M_Leontief=0.235 (M_arith=0.624), collapse=Proposition1BlindUndersizing, cov= 25.0% (shock=  0.0%), harm=  4.49 (MC100=  4.91±0.12 [  4.38,   5.18], ruin=  4.0%), R_24= 27.21
    [E=1 V=0 G=1] DefectiveAgent(NoVitalMargin->Prop2:101)     -> M_Leontief=0.021 (M_arith=0.503), collapse=Proposition2ImpotentLucidity, cov= 95.8% (shock= 75.0%), harm= 77.19 (MC100= 80.99±0.27 [ 77.12,  82.77], ruin=100.0%), R_24=  0.00
    [E=1 V=1 G=0] DefectiveAgent(NoGovernance->Prop3:110)      -> M_Leontief=0.101 (M_arith=0.612), collapse=Proposition3CynicalCapture  , cov= 95.8% (shock= 75.0%), harm= 68.01 (MC100= 67.89±0.01 [ 67.69,  68.10], ruin=100.0%), R_24=  0.00
    [E=1 V=0 G=0] PartialAgent(EpistemicOnly:100)              -> M_Leontief=0.010 (M_arith=0.295), collapse=CombinedPropositions2And3   , cov= 95.8% (shock= 75.0%), harm=114.79 (MC100=114.71±0.02 [114.42, 114.98], ruin=100.0%), R_24=  0.00
    [E=0 V=1 G=0] PartialAgent(VitalOnly:010)                  -> M_Leontief=0.094 (M_arith=0.392), collapse=CombinedPropositions1And3   , cov= 20.8% (shock=  0.0%), harm= 80.50 (MC100= 85.45±0.42 [ 80.08,  91.81], ruin=100.0%), R_24=  0.00
    [E=0 V=0 G=1] PartialAgent(GovernanceOnly:001)             -> M_Leontief=0.011 (M_arith=0.288), collapse=CombinedPropositions1And2   , cov= 16.7% (shock=  0.0%), harm=109.58 (MC100=110.81±0.24 [109.50, 115.11], ruin=100.0%), R_24=  0.00
    [E=0 V=0 G=0] DefectiveAgent(AllAxesOff:000)               -> M_Leontief=0.008 (M_arith=0.085), collapse=TotalTriadicCollapse        , cov= 16.7% (shock=  0.0%), harm=125.83 (MC100=124.38±0.24 [120.32, 126.04], ruin=100.0%), R_24=  0.00
  Fixed-Budget Mechanism Isolation Factorial (2^3 x 3 = 24 cells, B_fixed in [8.0, 14.0, 20.0], f=0.65) [Simulated]: delta_harm_Epi=+5.57, delta_harm_Vital=+5.81, delta_harm_Gov=+91.28 (FullTriad mean_harm=19.86)
  Non-Substitutability & Negative Controls [Simulated]: CompensatoryGiantBuffer(NoEpi+2xBuffer) cov=25.0%, harm=3.89 (MC100=4.06±0.02), R_24=39.88, diag=Proposition1BlindUndersizing | BenignSmallWorld(NaiveOptimizer) R_24=71.40, ruin=false | 3^5=243 Threshold Invariance=100.0% | 3^5=243 Local Structural DGP Invariance=100.0% | Wide-Band Stress Grid: dom=66.7%, intact=88.9%
  Closed-Form Prop 1 Coverage Decay Ladder [Assumed]: engine(n=46,rho=0.62)=35.9% | Meng ladder(rho=-0.05): n=46->90.5%, n=500->46.6%, n=5k->0.0%, n=50k->0.00%, n=2.3M->0.0000% (benchmark_met=true, manski_cov=100.0%, reserve_at_TTR=-78.8)
  Process Tracing (Bennett-Checkel) [Assumed]: prior = 0.25 -> posterior = 0.9988 (decibans = +34.1, V: 0.20 -> 0.64)
  Four Reflexive Brakes [Measured forecasts + Assumed welfare/TTR]: RES_sharp=0.117 > RES_lazy=0.000, welfare_opt=2.55 > hoard=-11.97, fast_path (TTR_recovery=1.5 <= TTS_survival=4.0), scalar_target_absent=true (all_active = true)
  Nested Leontief vs Multiplicative Damping [Assumed] (tau=0.50, 3^5=243 grid): V_min=0.1250 vs V_prod=0.0333 (-73.35% vol) | False Collapses: PureLeontief=0.00% (0/243) vs HybridGov=74.07% (180/243) vs Prod5=99.59% (242/243; vol-matched tau_prod=0.2951: 192/243; power-p05: 0/243) | Single-Axis Ruin False-Safe: Arith=100.0% (36/36) vs Leontief=0.0% (0/36)
  Four Pre-Registered Falsifiers [Assumed design + Simulated paired MC100 evaluation] (Mayo SEV >= 0.95): F1(n_eff=24, d_crit=0.0661, d_obs=0.321, SEV=0.995, refuted=false) | F2(n_eff=19, d_crit=0.6698, d_obs=2.514, SEV=0.982, refuted=false) | F3(n_eff=20, d_crit=0.0897, d_obs=0.385, SEV=0.972, refuted=false) | F4(n_eff=17, d_crit=0.1104, d_obs=0.077, SEV=0.996, refuted=true)
[STOCHOLOGY VERIFIED] All 9 progressive chapter laboratories and 3 Collapse Propositions passed.
```

6. **Bidirectional provenance, structural, and parity checks (stage `6`):** Re-derives `sha256sum -c --quiet data/SHA256SUMS`; requires every line the CLI emits to appear in [`README.md`](README.md) and in this appendix, and, in the reverse direction, requires every line inside a `=== CHAPTER ` block here, in the README and in the nine framework chapters to be a line the CLI actually emits; refuses a non-canonical provenance tag, a framework chapter carrying fewer than ten canonical tags, a phantom `` `Type::member` ``, a stale `#L` declaration window, an absolute local path or URI in the prose, a KaTeX hygiene violation, and an SVG figure with hardcoded hex colours outside `<style>` or without a reference from `book-src/*.md`; re-checks `book-src/lab/index.html` against `--export-lab-json` (trajectory table, the 35 multilayer nodes, the three frozen inequalities, and the eight-distinct-values requirement per axis) and the frozen cross-domain audit report against `--audit-report` (stage `6b`); and requires every dataset named in `data/SHA256SUMS` to be loaded in `crates/stochology-cli/src/loaders.rs` `[Measured]`.

> **Regime Interpretation of `F1`–`F4` (`[Assumed design + Simulated paired MC100 evaluation]`):** In the final line of Chapter 50, `F1` ($d_{\mathrm{obs}} = 0.321 > d_{\mathrm{crit}} = 0.0661, \mathrm{SEV} = 0.995$), `F2` ($d_{\mathrm{obs}} = 2.514 > d_{\mathrm{crit}} = 0.6698, \mathrm{SEV} = 0.982$), and `F3` ($d_{\mathrm{obs}} = 0.385 > d_{\mathrm{crit}} = 0.0897, \mathrm{SEV} = 0.972$) corroborate the necessity of each safeguard (`refuted=false`) without saturating at $\mathrm{SEV} = 1.000$. By contrast, `F4` ($d_{\mathrm{obs}} = 0.077 \le d_{\mathrm{crit}} = 0.1104, \mathrm{SEV} = 0.996, \text{refuted} = \text{true}$) genuinely refutes unconditional non-compensability across a mostly mild archival-defect cohort where `CompensatoryBufferAgent`'s doubled physical reserve buffer partially absorbs epistemic undersizing—showing that strict non-substitutability holds when an absorbing survival barrier or severe defect is active, whereas partial compensation is possible under moderate shocks with doubled physical slack.

---

## B.4 The Multilayer Graph — 35 Orders, 7 Strata, 5 Exposure Regimes

[`multilayer.rs`](crates/stochology/src/multilayer.rs) carries Part V. It defines `KnowledgeStratum` (the seven strata `S1`–`S7`, each anchored on one guard site by `KnowledgeStratum::canonical_guard_site` and its inverse `KnowledgeStratum::from_canonical_guard_site`), `KnowledgeOrder` (the 35 orders `L01`–`L35`, each exposing `KnowledgeOrder::guard_sites`, `KnowledgeOrder::primary_guard_site`, `KnowledgeOrder::stratum`, `KnowledgeOrder::parent_layers`, `KnowledgeOrder::exposure_regime`, `KnowledgeOrder::chapter_number` and `KnowledgeOrder::chapter_file`), `TriadicExposureRegime` (the five axis-occupancy classes $12 + 9 + 9 + 4 + 1 = 35$), `MultilayerGraph1` / `Graph1Node` (the knowledge DAG and its ancestor cones), `MultilayerGraph2` / `Graph2Node` (the guard-triple projection) and `MultilayerHomomorphism` / `LayerTriadReading` (the 35-layer transfer map).

The graph is frozen against drift by [`tests/multilayer_spec.rs`](crates/stochology/tests/multilayer_spec.rs), which parses [The Multilayer Graph of Knowledge and Aim](notes/The%20Multilayer%20Graph%20of%20Knowledge%20and%20Aim.md), resolves its `multilayer-anchors` block against the sources, and verifies the $\binom{7}{3} = 35$ bijection, the seven-to-seven stratum ↔ guard-site bijection, the $7 \times 5 = 35$ primary-site balance, and the 1-to-1 alignment with Part V chapters `51`–`85` (`[Assumed]` for the combinatorial counts, which are exact).

### Table B.2 — Complete 35-Layer Multilayer Knowledge Graph Mapping (Part V, Chapters 51–85)

| Code | Ch | Stratum | Knowledge Order (`KnowledgeOrder`) | Primary Site | Unique Guard Triple (`guard_sites`) | Signature $(E,V,G)$ | Exposure Regime (`TriadicExposureRegime`) |
| :---: | :---: | :---: | :--- | :---: | :---: | :---: | :--- |
| `L01` | 51 | `S1` | `KnowledgeOrder::L01MicroKinetic` | `T5` | `{T1, T5, T7}` | `(1, 1, 1)` | `TransTriadicKeystone` |
| `L02` | 52 | `S1` | `KnowledgeOrder::L02MesoBufferedQueue` | `T7` | `{T4, T6, T7}` | `(0, 1, 2)` | `EpistemicBlindSpot` |
| `L03` | 53 | `S1` | `KnowledgeOrder::L03MacroCoupledTopology` | `T6` | `{T4, T5, T6}` | `(0, 2, 1)` | `EpistemicBlindSpot` |
| `L04` | 54 | `S1` | `KnowledgeOrder::L04GeophysicalChaosDataAssimilation` | `T3` | `{T3, T4, T5}` | `(1, 2, 0)` | `GovernanceBlindSpot` |
| `L05` | 55 | `S1` | `KnowledgeOrder::L05ContinuumFractureStructural` | `T4` | `{T2, T4, T7}` | `(1, 1, 1)` | `TransTriadicKeystone` |
| `L06` | 56 | `S2` | `KnowledgeOrder::L06FormalComputableLinearType` | `T1` | `{T1, T4, T5}` | `(1, 2, 0)` | `GovernanceBlindSpot` |
| `L07` | 57 | `S2` | `KnowledgeOrder::L07ProbabilityCredenceMeasurement` | `T4` | `{T1, T2, T4}` | `(2, 1, 0)` | `GovernanceBlindSpot` |
| `L08` | 58 | `S2` | `KnowledgeOrder::L08MetrologicalInfoGeometric` | `T3` | `{T3, T4, T6}` | `(1, 1, 1)` | `TransTriadicKeystone` |
| `L09` | 59 | `S2` | `KnowledgeOrder::L09ExperimentalSamplingCausal` | `T2` | `{T1, T2, T3}` | `(3, 0, 0)` | `PureEpistemicConcentration` |
| `L10` | 60 | `S2` | `KnowledgeOrder::L10CryptographicZeroKnowledgeAdversarial` | `T7` | `{T2, T6, T7}` | `(1, 0, 2)` | `VitalBlindSpot` |
| `L11` | 61 | `S3` | `KnowledgeOrder::L11BiochemicalKineticMolecular` | `T2` | `{T2, T5, T6}` | `(1, 1, 1)` | `TransTriadicKeystone` |
| `L12` | 62 | `S3` | `KnowledgeOrder::L12MorphogeneticDevelopmental` | `T5` | `{T1, T5, T6}` | `(1, 1, 1)` | `TransTriadicKeystone` |
| `L13` | 63 | `S3` | `KnowledgeOrder::L13ImmunologicalSelfGovernance` | `T7` | `{T2, T5, T7}` | `(1, 1, 1)` | `TransTriadicKeystone` |
| `L14` | 64 | `S3` | `KnowledgeOrder::L14TaxonomySystematicsOntology` | `T4` | `{T1, T3, T4}` | `(2, 1, 0)` | `GovernanceBlindSpot` |
| `L15` | 65 | `S3` | `KnowledgeOrder::L15StratigraphyPaleobiologyDeepTime` | `T1` | `{T1, T3, T5}` | `(2, 1, 0)` | `GovernanceBlindSpot` |
| `L16` | 66 | `S4` | `KnowledgeOrder::L16PharmacologicalClinicalDiagnostic` | `T4` | `{T2, T4, T5}` | `(1, 2, 0)` | `GovernanceBlindSpot` |
| `L17` | 67 | `S4` | `KnowledgeOrder::L17EpidemiologyOutbreakSurveillance` | `T3` | `{T2, T3, T5}` | `(2, 1, 0)` | `GovernanceBlindSpot` |
| `L18` | 68 | `S4` | `KnowledgeOrder::L18PharmacovigilanceRegulatoryToxicology` | `T2` | `{T2, T3, T7}` | `(2, 0, 1)` | `VitalBlindSpot` |
| `L19` | 69 | `S4` | `KnowledgeOrder::L19MetascienceScientometricsReplication` | `T6` | `{T2, T3, T6}` | `(2, 0, 1)` | `VitalBlindSpot` |
| `L20` | 70 | `S4` | `KnowledgeOrder::L20AcousticAestheticTragicPedagogical` | `T1` | `{T1, T4, T6}` | `(1, 1, 1)` | `TransTriadicKeystone` |
| `L21` | 71 | `S5` | `KnowledgeOrder::L21LinguisticEvidentialPragmatic` | `T7` | `{T3, T6, T7}` | `(1, 0, 2)` | `VitalBlindSpot` |
| `L22` | 72 | `S5` | `KnowledgeOrder::L22PhilologicalEpigraphicTextual` | `T3` | `{T2, T3, T4}` | `(2, 1, 0)` | `GovernanceBlindSpot` |
| `L23` | 73 | `S5` | `KnowledgeOrder::L23TaphonomicHistoriographical` | `T5` | `{T3, T5, T7}` | `(1, 1, 1)` | `TransTriadicKeystone` |
| `L24` | 74 | `S5` | `KnowledgeOrder::L24CriticalArchivalTransitionalJustice` | `T1` | `{T1, T3, T7}` | `(2, 0, 1)` | `VitalBlindSpot` |
| `L25` | 75 | `S5` | `KnowledgeOrder::L25MediaEcologyInformationRetrieval` | `T6` | `{T1, T3, T6}` | `(2, 0, 1)` | `VitalBlindSpot` |
| `L26` | 76 | `S6` | `KnowledgeOrder::L26SequentialDecisionBanditRealOptions` | `T5` | `{T1, T2, T5}` | `(2, 1, 0)` | `GovernanceBlindSpot` |
| `L27` | 77 | `S6` | `KnowledgeOrder::L27GameTheoryStrategicAdversarial` | `T2` | `{T1, T2, T6}` | `(2, 0, 1)` | `VitalBlindSpot` |
| `L28` | 78 | `S6` | `KnowledgeOrder::L28MacroFinanceEconometricsActuarial` | `T6` | `{T2, T4, T6}` | `(1, 1, 1)` | `TransTriadicKeystone` |
| `L29` | 79 | `S6` | `KnowledgeOrder::L29OfficialStatisticsForensicAccounting` | `T3` | `{T3, T4, T7}` | `(1, 1, 1)` | `TransTriadicKeystone` |
| `L30` | 80 | `S6` | `KnowledgeOrder::L30CorporateGovernancePrincipalAgent` | `T1` | `{T1, T6, T7}` | `(1, 0, 2)` | `VitalBlindSpot` |
| `L31` | 81 | `S7` | `KnowledgeOrder::L31CriminalProofForensicEpistemology` | `T2` | `{T1, T2, T7}` | `(2, 0, 1)` | `VitalBlindSpot` |
| `L32` | 82 | `S7` | `KnowledgeOrder::L32EvidentiaryTortConstitutional` | `T7` | `{T1, T4, T7}` | `(1, 1, 1)` | `TransTriadicKeystone` |
| `L33` | 83 | `S7` | `KnowledgeOrder::L33DistributedConsensusSre` | `T5` | `{T5, T6, T7}` | `(0, 1, 2)` | `EpistemicBlindSpot` |
| `L34` | 84 | `S7` | `KnowledgeOrder::L34HighHazardInfrastructureSafety` | `T4` | `{T4, T5, T7}` | `(0, 2, 1)` | `EpistemicBlindSpot` |
| `L35` | 85 | `S7` | `KnowledgeOrder::L35PolycentricCommonsBiospheric` | `T6` | `{T3, T5, T6}` | `(1, 1, 1)` | `TransTriadicKeystone` |

## B.5 The Grammar of Aim — Export Summary

[`grammar.rs`](crates/stochology/src/grammar.rs) exports `Plane`, `GuardSite`, `CircuitTransit`, `TriadAxis`, `Sentence`, `Shot`, `ReadReading`, `GuardReading`, `GrammarVerdict`, `UnparsableReason`, `GrammarBudget`, `AgentAction`, `AgentActionKind`, `EbnfGrammar`, `EbnfRule`, `Coverage` and the entry points `judge`, `enumerate` and `shortest_violation`, together with the guard-site accessors `GuardSite::plane`, `GuardSite::transit`, `GuardSite::axis` and `GuardSite::code`. Its acceptance semantics are frozen by [`tests/grammar_spec.rs`](crates/stochology/tests/grammar_spec.rs) against the note and against `TrajectoryReport::triadic_integrity.diagnose` across all eight factorial cells `[Assumed]`.

## B.6 Cross-Domain Audit Report

The four audits that exercise the Stochological operators beyond the seeded datasets are emitted, frozen, by `cargo run --release --bin stochology -- --audit-report` `[Simulated, seed = 2026]`. `scripts/verify.sh` requires the block below and the command output to agree line by line, in both directions.

```text
=== CROSS-DOMAIN AUDIT REPORT ===
DREAM  slope = 0.99187 | dream_return = 18.9307 | realized_return = 4.2663 | ruin_step = Some(6) | extrapolation_depth = 2.8695 | out_of_support = 0.5833 | discrepancy = 0.8828 | goodhart = Adversarial
TAU    [0] tau = 0.0500 | realized_return = 4.2663 | ruin_step = Some(6) | discrepancy = 0.8754
TAU    [1] tau = 0.2000 | realized_return = 4.8369 | ruin_step = Some(7) | discrepancy = 0.6821
TAU    [2] tau = 0.8000 | realized_return = 5.4298 | ruin_step = Some(8) | discrepancy = 0.0894
TAU    [3] tau = 3.2000 | realized_return = 7.8848 | ruin_step = None | discrepancy = 0.0195
TAU    [4] tau = 12.8000 | realized_return = 7.8848 | ruin_step = None | discrepancy = 0.0195
CAR    mu_deploy = 0.95 | mu_sim = 1.20 | mu_identified = 0.95 | target = 15.00 | gain = 1.40
CAR    ModelBasedMpc | completed = true | lap_time = Some("13.3566") | violations = 0 | max_utilization = 0.9657 | grip_loss_step = None | mean_speed = 15.5277
CAR    InSimulationTuned | completed = false | lap_time = None | violations = 2 | max_utilization = 1.2071 | grip_loss_step = Some(87) | mean_speed = 14.5038
CAR    DemonstrationFitted | completed = false | lap_time = None | violations = 2 | max_utilization = 1.1220 | grip_loss_step = Some(87) | mean_speed = 15.2474
NET    nodes = 160 | blocks = 4 | edges = 2116 | mean_degree = 13.2250 | density = 0.0832 | diameter = 3 | clustering = 0.3822
NET    Q_planted = 0.6037 | Q_label_propagation = 0.6037 | communities = 4 | pagerank_prestige = true
NET    eigenvector_max = 0.1268 | hits_hub_max = 0.1349 | hits_authority_max = 0.1607 | betweenness_max = 0.0189 | bridge_betweenness = 0.0069 | interior_betweenness = 0.0009
DRIFT  change_point = 10 | detection = Some(11) | false_alarm = None | tts = 26 | ttr = Some(23) | reserve_with_fallback = 7.5000 | reserve_without_fallback = 0.0000 | survived_without = false | survived_with = true
DRIFT  slow_retrain ttr = Some(35) | survived_with_slow_retrain = false
CELL   R0 | cells = 201 | steps = 200 | fixed_point = Some(2) | period = Some(1) | block_entropy = 0.0000 | block_entropy_max = 0.3608 | live_fraction = 0.0000 | regime = Settling
CELL   R90 | cells = 201 | steps = 200 | fixed_point = None | period = None | block_entropy = 1.8575 | block_entropy_max = 4.5363 | live_fraction = 0.0398 | regime = Structured
CELL   R30 | cells = 201 | steps = 200 | fixed_point = None | period = None | block_entropy = 7.0463 | block_entropy_max = 7.1870 | live_fraction = 0.5224 | regime = Dispersed
CELL   R110 | cells = 201 | steps = 200 | fixed_point = None | period = None | block_entropy = 5.9839 | block_entropy_max = 6.1609 | live_fraction = 0.6020 | regime = Dispersed
```

`Dream` is the latent world-model exploitation audit (`governance.rs::LatentModelExploitationAudit`), configured by `LatentWorldModelConfig::canonical` and classified with `governance.rs::GoodhartVariant`: a policy optimised against a linear surrogate fitted inside the anchor support leaves it by `extrapolation_depth` radii, reaches a structural discrepancy `discrepancy` at the dreamed terminal state, and destroys the world at `ruin_step`, while its dreamed return stays several times its realised return; the `TAU` sweep is the Boltzmann temperature `[Simulated]`. `Car` is the friction-envelope benchmark (`engine.rs::FrictionEnvelopeControlAudit`), whose three arms are the variants of `engine.rs::ControlRegime`: the model-based controller completes the lap inside the ellipse, while the in-simulation-tuned and the demonstration-fitted controllers both lose grip on the deployed surface `[Simulated]`. `Cell` is the elementary-automaton battery (`kernel.rs::CellularAutomatonAudit`), one line per member of `kernel.rs::ElementaryRule`, with the observed `kernel.rs::AutomatonRegime` beside the observables: rule 0 settles at step 2, rule 90 stays structured with block entropy 1.86 bits, rules 30 and 110 are both dispersed (7.05 and 5.98 bits), which is the point — the audit separates three regimes and states that it cannot separate universality from entropy `[Simulated]`. `Net` is the topology battery (`multilayer.rs::TopologyCentralityAudit`), drawn from `multilayer.rs::SocialNetworkSpec`: modularity of the planted and of the label-propagation partition, communities recovered, PageRank recovery of the planted prestige order, eigenvector centrality, HITS and Brandes betweenness `[Simulated]`. `Drift` is the model-operations audit (`vital.rs::ModelDriftFallbackAudit`), configured by `vital.rs::DriftMonitorConfig`: a CUSUM monitor detects the shift one step after it happens, the institution without a fallback exhausts its reserve at step 26 and never recovers, the institution with a fallback completes recovery at step 23 and keeps 7.50 of its 8.00 reserve, and an institution whose retraining takes twice as long recovers too late `[Simulated]`.

## B.7 Cross-Domain Audit Coverage Map

Every audit domain that the treatise absorbs beyond the nine seeded laboratories is listed below with the chapters that carry it, the code anchors that make it verifiable, and the status of the claim. `audit_spec` in [`tests/audit_spec.rs`](crates/stochology/tests/audit_spec.rs) parses this table: a listed chapter must exist as a file, a listed anchor must resolve to a real symbol, and no row may be dropped. `medido` = computed by the CLI on a sealed dataset in `data/*.csv`; `simulado` = deterministic simulation under a pre-registered seed; `prosa` = argument carried by the text with `[Literature]` provenance only.

| Id | Audit domain | Chapters | Code anchors | Status |
| :--- | :--- | :--- | :--- | :--- |
| `AC-01` | Agent bridges and tool contracts: multi-agent firms, OAuth2 bridges, read/write skill contracts | 56, 83 | `grammar.rs::AgentActionKind`, `governance.rs::TemporalVault` | simulado |
| `AC-02` | Judgment at scale: technology as an evolving system, search-based play, fluent wrongness, cognitive bias, metacognitive protocol | 28, 49 | `governance.rs::CognitiveAlertFilter` | prosa |
| `AC-03` | The Perceptron saga (1956–1988): Dartmouth, ARPA, ALPAC, AGC, Minsky–Papert parity and connectivity theorems, Dreyfus, Mansfield, backpropagation, Hopfield, Society of Mind | 14, 28, 53 | `multilayer.rs::TopologyCentralityAudit` | prosa |
| `AC-04` | World models: perception–dynamics–controller, dream exploitation and temperature, latent planning, joint-embedding prediction, diffusion world models | 48, 51 | `governance.rs::LatentModelExploitationAudit` | simulado |
| `AC-05` | Digital twins: a vehicle driven outside its design envelope, reconfigurable simulators, the Earth-system replica and its admission testbeds, connectome emulation | 51, 54, 56, 84 | `vital.rs::VitalMarginState` | prosa |
| `AC-06` | Cellular automata and their four regimes, neural cellular automata, implicit representations, complex-network metrics | 53, 58, 62, 67 | `kernel.rs::CellularAutomatonAudit`, `multilayer.rs::TopologyCentralityAudit` | simulado |
| `AC-07` | Artificial intelligence as a historical object: the 1950 imitation game, the incomputable AIXI ceiling, and capability taxonomies under physical, cognitive and institutional limits | 28, 56 | `grammar.rs::judge` | prosa |
| `AC-08` | Compensatory executive scoring (weighted sums, ratio scores, impact matrices) against the non-compensatory Leontief operator | 41, 50, 76 | `synthesis.rs::LeontiefVsMultiplicativeAudit` | simulado |
| `AC-09` | No-code statistical tooling and personal knowledge management: audit discipline over graphical environments and private archives | 46, 75 | `epistemic.rs::EpistemicMarginState` | prosa |
| `AC-10` | Probability, information theory, data quality, missingness and leakage | 30, 43, 44, 45 | `epistemic.rs::EpistemicMarginState` | medido |
| `AC-11` | Tabular ensembles, kernels, dimensionality reduction, hyper-parameter search, and the causal limit of attribution | 37, 39 | `governance.rs::GovernanceState` | prosa |
| `AC-12` | Attention as a learned kernel, scaling laws as fitted extrapolations and their finite input supply, low-rank adapters, retrieval-augmented generation, test-time compute and evals | 14, 58, 75, 83 | `vital.rs::VitalMarginState` | prosa |
| `AC-13` | Model operations: continuous training, drift, quantisation, model cards, prompt and chain-of-thought artefacts, trigger/action automation, statutory fallback and cost governance | 47, 82, 83 | `vital.rs::ModelDriftFallbackAudit` | simulado |
| `AC-14` | Autonomous race car: model-based, in-simulation-tuned and demonstration-fitted control on the friction envelope | 51 | `engine.rs::FrictionEnvelopeControlAudit` | simulado |
| `AC-15` | A national air-transport archive as a second institutional producer of the same tail statistics | 46 | `vital.rs::VitalMarginState` | medido |
| `AC-16` | Human and social behaviour in complex networks: modularity, betweenness, eigenvector centrality, HITS | 53, 75 | `multilayer.rs::TopologyCentralityAudit` | simulado |

## B.8 Complete Export Inventory of the Two Crates

The nine laboratory chapters quote their canonical entry points; this section is the completeness reference for the rest of the surface. It lists every public type, public function, and public constant the two crates export, module by module, so that a reader can locate any canonical operator member without a grep. The library crate root re-exports all eight of its modules with `pub use`, so each item in B.8.1–B.8.9 is reachable as `stochology::Name`; the CLI crate root re-exports its three modules in the same way, and every remaining item stays reachable through its module path. Two modules are inventoried in full here — the syntax layer `grammar.rs` (B.8.4) and the multilayer graph `multilayer.rs` (B.8.7) — because B.5 and B.4 quote only their canonical judgement, enumeration and stratum accessors. The inventory is descriptive `[Assumed]`: it asserts export status, never a numeric claim, and it leaves every measurement to the sealed datasets of B.8.15.

**Counting rule (the contract the drift guard pins).** A row is one declared public item of one module, counted in five categories: a `pub struct`, `pub enum` or `pub trait`; a `pub fn`, including a `pub const fn` and a trait method implemented inside an `impl … for …` block; a `pub const NAME:` item; a `pub static NAME:` item; and a `pub type` alias. Four kinds of name are not rows. A `pub use` re-export and a `pub mod` declaration add no declaration of their own, enum variants belong to their `enum` row, and everything after the first `#[cfg(test)]` in a module is test scaffolding rather than exported surface. A name declared more than once inside one module — the same method implemented on several types — counts once, so `evaluate`, `new` or `canonical_score` appears in a module's function list exactly once even where several implementations declare it. Under that rule the fourteen module sections account for **380** public items: **377** declarations across the first four categories plus the **3** type aliases recorded under B.8.3, B.8.4 and B.8.5. Each `### B.8.N` heading reports that module's count with its aliases included, so a reader who wants the declaration-only figure subtracts those three.

### B.8.1 `engine.rs` — 31 public items

Types: `AgentDecision`, `AgentPolicy`, `ArchiveObservation`, `CommunityReserveEnv`, `CompensatoryBufferAgent`, `ControlLapResult`, `ControlRegime`, `FixedBudgetFactorialAudit`, `FrictionEnvelopeControlAudit`, `MultiSeedSummary`, `NaiveOptimizer`, `StepOutcome`, `StochologicalAgent`, `TrajectoryReport`

Functions: `epistemic_only`, `evaluate`, `evaluate_mechanism_vs_budget_factorial`, `evaluate_multi_seed`, `export_lab_json`, `full`, `new`, `run_benign_small_world_control`, `run_trajectory`, `run_trajectory_with_full_params`, `run_trajectory_with_thresholds`, `verify_structural_perturbation_grid`, `verify_threshold_sensitivity`, `verify_wide_band_structural_sensitivity`, `with_axes`, `without_epistemic_margin`, `without_governance`

### B.8.2 `epistemic.rs` — 30 public items

Types: `BitemporalRecord`, `CognitiveFramingRegime`, `EpistemicMarginState`

Functions: `anytime_valid_radius`, `bitemporal_view_at`, `canonical_score`, `e_bh_discoveries`, `for_shift_observation`, `framing_gap`, `from_archive_clusters`, `from_ate_bounds`, `from_capture_recapture`, `from_censoring_bounds`, `from_e_process`, `from_inquiry_framing`, `from_kish_strata`, `from_lee_attrition_bounds`, `from_meng_big_data`, `from_multiverse`, `from_peterson_competing_risks_bounds`, `from_single_arm_bounds`, `from_vintages`, `imbens_manski_critical_value`, `is_causally_valid`, `keynes_weight_from_n_eff`, `lakatos_shift_ratio`, `passes_severe_boundary`, `pauker_kassirer_threshold`, `unified_effective_n`

Constants: `CANONICAL_KEYNES_N0`

### B.8.3 `governance.rs` — 57 public items

Types: `AlgedonicChannel`, `AuditLog`, `AuditLogEntry`, `BatesonDoubleBindAudit`, `CausalEnvironment`, `CognitiveAlertFilter`, `ContractAndLiabilityAudit`, `CrpsDecomposition`, `EmpiricalChouldechovaAudit`, `GoodhartVariant`, `GovernanceState`, `IcpIdentificationStatus`, `KairosSynchronousPanel`, `LatentModelExploitationAudit`, `LatentWorldModelConfig`, `MacKenzieRegime`, `TelicGrade`, `TemporalVault`, `ZhengMeisterQueueAudit`

Functions: `append_with_key`, `canonical`, `canonical_score`, `classify`, `classify_icp_from_intersection`, `classify_icp_power`, `corp_pav_crps_decomposition`, `evaluate`, `evaluate_chouldechova_classifier_fpr`, `evaluate_corp_pav`, `evaluate_corp_pav_optimism_cv`, `evaluate_corp_pav_regularized`, `evaluate_icp_subset_intersection`, `evaluate_list_pettit_votes`, `evaluate_performative_system`, `evaluate_shift_governance`, `evaluate_three_canonical_regimes`, `evaluate_tian_pearl_bounds`, `from_all_three_gates`, `from_deficit_series`, `from_ex_ante_gate`, `from_real_time_and_ex_post_gates`, `icp_f_critical_value`, `new`, `seal`, `seal_at_shift_with_key`, `seal_with_key`, `try_evaluate_corp_pav`, `try_evaluate_icp_subset_intersection`, `unseal`, `unseal_with_key`, `verify_chain`, `verify_two_shift_lifecycle`

Constants: `DEFAULT_VAULT_HMAC_KEY`, `HUMAN_BITS_PER_SEC`, `MAX_KAIROS_BITS`, `SWEEP_LEN`

Aliases: `SosaJudgmentGrade` (governance.rs:53) — an alias of `TelicGrade`, the moral-luck name under which the third gate's telic grade is exported.

### B.8.4 `grammar.rs` — 49 public items

Types: `AgentAction`, `AgentActionKind`, `CircuitTransit`, `Coverage`, `EbnfGrammar`, `EbnfRule`, `GrammarBudget`, `GrammarVerdict`, `GuardReading`, `GuardSite`, `Plane`, `ReadReading`, `Sentence`, `Shot`, `TriadAxis`, `UnparsableReason`

Functions: `assess_report`, `attested`, `axis`, `canonical`, `code`, `coverage`, `depth`, `enumerate`, `enumerate_with_coverage`, `index`, `is_bijection_over_circuit_transits`, `judge`, `lex_tokens`, `new`, `parse_ebnf_grammar`, `parse_sentence`, `payload`, `payload_of`, `payload_of_full`, `plane`, `shortest_violation`, `shortest_violation_with_coverage`, `to_ebnf_surface`, `transit`, `verify_seal`, `with_agent_audit`

Constants: `ALL`, `BOUNDARY_BAND`, `CANONICAL_CIRCUIT`, `HARM_MAX_ASSUMED`, `MAX_DEPTH`, `MAX_ENUMERATION`

Aliases: `ResourceId` (grammar.rs:326) — a type alias for String, the identifier a parsed action carries as its referent.

### B.8.5 `kernel.rs` — 57 public items

Types: `AutomatonRegime`, `CellularAutomatonAudit`, `ElementaryRule`, `OnlineMoments`, `SplitMix64`

Functions: `bernoulli`, `beta`, `code`, `cromwell_clamp`, `excess_kurtosis`, `exponential`, `from_slice`, `gamma`, `hmac_sha256_hex`, `inverse_standard_normal_cdf`, `is_valid_sha256_hex`, `kolmogorov_smirnov_distance`, `log_standard_gamma`, `mean`, `median`, `median_absolute_deviation`, `new`, `next_f64`, `next_u64`, `normal`, `obeys_cromwell_rule`, `ols_ridge_fit`, `pareto`, `parse_quarter`, `pav_isotonic_non_decreasing`, `poisson`, `push`, `quantile`, `quantile_sorted`, `run`, `sample_std`, `sample_variance`, `sha256_bytes`, `sha256_hex`, `shuffle`, `skewness`, `solve_cholesky_spd`, `spearman_rank_correlation`, `split`, `split_fork_join_pair`, `standard_normal`, `standard_normal_cdf`, `student_t`, `table`, `uniform`, `with_default_seed`

Constants: `ALL`, `CELLS`, `DEFAULT_SEED`, `MAX_PERIOD`, `STEPS`

Aliases: `Rng` (kernel.rs:26) — an alias of `SplitMix64`, the deterministic stream type behind every seeded simulation.

### B.8.6 `lib.rs` — the crate root as a re-export hub

Public modules: `engine`, `epistemic`, `governance`, `grammar`, `kernel`, `multilayer`, `synthesis`, `vital`, each re-exported wholesale by `pub use`, so the eight module inventories above are the crate's entire public surface.

### B.8.7 `multilayer.rs` — 44 public items

Types: `Graph1Node`, `Graph2Node`, `KnowledgeOrder`, `KnowledgeStratum`, `LayerTriadReading`, `MultilayerGraph1`, `MultilayerGraph2`, `MultilayerHomomorphism`, `SocialNetworkSpec`, `TopologyCentralityAudit`, `TriadicExposureRegime`

Functions: `ancestors_of`, `canonical`, `canonical_guard_site`, `canonical_isomorphism`, `chapter_file`, `chapter_number`, `code`, `covers_all_seven_guard_sites`, `edge_count`, `evaluate`, `every_ancestor_cone_spans_all_three_axes`, `every_stratum_covers_all_seven_sites`, `expected_count`, `exposure_regime`, `from_canonical_guard_site`, `guard_sites`, `has_distinct_primary_guard_sites`, `index`, `is_balanced_five_orders_per_primary_site`, `is_bijection_over_35_guard_combinations`, `is_strict_dag`, `layer_index`, `local_integrity`, `orders`, `parent_layers`, `primary_guard_site`, `primary_triad_axis`, `stratum`, `title`, `triadic_occupancy`, `uncoupled_collapse`, `uncoupled_vulnerability`

Constants: `ALL`

### B.8.8 `synthesis.rs` — 27 public items

Types: `CollapseProposition`, `DiagnosticClue`, `FalsifierVerdict`, `LeontiefVsMultiplicativeAudit`, `NotTestable`, `PreregisteredFalsifier`, `ProcessTracingInference`, `ProcessTracingTestKind`, `ReflexiveBrakeAudit`, `RefutabilityPreconditions`, `StructuralCollapseProofAudit`, `TriadicIntegrity`

Functions: `active_likelihood_ratio`, `all_benchmark_conditions_met`, `all_brakes_active`, `all_three_propositions_proven`, `canonical_four`, `ces_aggregate`, `classify`, `design`, `evaluate`, `evaluate_analytical`, `evaluate_observations`, `evaluate_quantile_calibrated`, `evaluate_with_declaration`, `from_canonical_states`, `is_satisfied_for`

### B.8.9 `vital.rs` — 32 public items

Types: `DriftMonitorConfig`, `HollingTailRuptureAudit`, `KellySizingRegime`, `ModelDriftFallbackAudit`, `SniedovichInfoGapAudit`, `TailMomentRegime`, `VitalMarginState`, `WaldSurvivorshipAudit`

Functions: `canonical`, `canonical_score`, `crps_empirical`, `derive_ttr_from_rho1`, `empirical_cvar95_and_wdro_scale_sq`, `evaluate`, `evaluate_empirical_truncation_bias`, `for_shift_sizing`, `from_heavy_tails_and_asymmetry`, `from_kelly_parameters`, `from_non_ergodic_returns`, `from_queue_basin_and_derived_ttr`, `from_queue_basin_and_survival`, `from_queue_basin_and_survival_with_variability`, `from_regime_aware_kelly`, `hill_alpha_on_sorted_pos`, `hill_alpha_with_ci95`, `hill_stability_across_k`, `mann_whitney_dominance`, `sniedovich_2012_benchmark`, `solve_canguilhem_holling_basin`, `solve_latent_two_cohort`, `survives_disruption`, `wald_1943_bomber_benchmark`

### B.8.10 `chapters.rs` — 14 public items (CLI crate)

Functions: `audit_report`, `export_lab_trajectories_json`, `run_all_or_chapter`, `run_all_or_chapter_with_brake`, `run_chapter_1`, `run_chapter_2`, `run_chapter_3`, `run_chapter_4`, `run_chapter_5`, `run_chapter_6`, `run_chapter_7`, `run_chapter_8`, `run_chapter_8_with_brake`, `run_chapter_9`

### B.8.11 `csv.rs` — 13 public items (CLI crate)

Types: `CsvRow`, `CsvTable`

Functions: `f64_at`, `from_string`, `header`, `is_empty`, `len`, `open`, `parse`, `rows`, `str_at`, `u32_at`, `validate_schema`

### B.8.12 `lib.rs` — the CLI crate root as a re-export hub

Public modules: `chapters`, `csv`, `loaders`; the root re-exports the fourteen chapter entry points, the two CSV types, and the loader and manifest-verifier functions listed in B.8.10, B.8.11 and B.8.13. The `try_load_toronto_arrests_*` variants stay reachable through `loaders::` rather than the root, because they expose a caller-supplied table path that the CLI never uses.

### B.8.13 `loaders.rs` — 26 public items (CLI crate)

Functions: `count_atlantic_storms_transitions`, `count_flight_csv_rows_and_missing`, `load_atlantic_storms_environments`, `load_atlantic_storms_forecasts`, `load_bank_call_clusters`, `load_cps_cohort_clusters`, `load_danish_fire`, `load_djia_returns`, `load_lalonde_nsw_cps`, `load_nyc_flight_delays`, `load_oot_ood_flight_delays`, `load_rtdsm_routput_vintages`, `load_toronto_arrests_full_summary`, `load_toronto_arrests_matched_and_classifier`, `load_toronto_arrests_prevalences`, `load_vic_elec_demand`, `resolve_workspace_root`, `try_load_toronto_arrests_full_summary`, `try_load_toronto_arrests_full_summary_from_table`, `try_load_toronto_arrests_matched_and_classifier`, `try_load_toronto_arrests_matched_and_classifier_from_table`, `verify_external_csv_bytes_and_schema`, `verify_external_upstream_schema_contract`, `verify_live_upstream_url_if_enabled`, `verify_sha256sums_manifest`

Constants: `UPSTREAM_DATASET_CONTRACTS`

### B.8.14 `main.rs` — the single binary

The `stochology` binary exposes no library surface of its own: it parses `--chapter N`, `--export-lab-json` and `--audit-report`, then dispatches to the entry points of B.8.10.

### B.8.15 Datasets — the fourteen sealed archives

Every `[Measured]` claim in this book is computed from exactly these fourteen files, and no other CSV is read: `atlantic_storms.csv`, `bank_calls.csv`, `cps_observational.csv`, `danish_fire.csv`, `djia_1885_2023.csv`, `houston_flights_2011.csv`, `nsw_experimental.csv`, `nyc_flights_2013.csv`, `nyc_flights_2013_h1.csv`, `nyc_flights_2023.csv`, `nyc_flights_2023_h1.csv`, `rtdsm_routput_vintages.csv`, `toronto_arrests.csv` and `vic_elec.csv`. Provenance, domain semantics, row counts, and the laboratory that consumes each archive are given in [Appendix C, Table C.1](appendix-c-sources.md), and `scripts/verify.sh` re-derives the SHA-256 digest of all fourteen against [`data/SHA256SUMS`](data/SHA256SUMS) on every run `[Measured]`.
