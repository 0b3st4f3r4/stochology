# `stochology-governance`

Bitemporal production governance, cryptographic temporal vaults, conformal change-point detection, and label-shift adaptation for the **Stochology — CAPIBA Protocol** (*Calibrated, Anytime-valid, Performative-aware, Invariant, Bitemporal Auditing Protocol*) in pure Rust (`std` only).

## Overview

`stochology-governance` enforces production integrity and post-deployment drift adaptation:

- **Bitemporal Clock & Canary Auditing (`clock`):** Enforces $t_{\text{disp}} \ge t_{\text{ev}}$ (`BitemporalRecord`, `BitemporalClockAudit`, `query_with_bitemporal_clock`) to block look-ahead leakage, and verifies synthetic canary mutual information $I(\text{Canary}; Y) = 0$ (`canary_mutual_information_nats`).
- **Single-Opening Cryptographic Vault (`vault`):** `TemporalVault` (`VaultOpeningReport`, `VaultState`, `VaultError`) with SHA-256 pre-registration sealing and single-opening holdout discipline.
- **Conformal Drift, Circuit Breakers & Statutory Governance (`drift`):** `MurphyCorpCircuitBreaker` (`DriftRegime`), `AdaptiveConformalInference`, exponentially weighted conformal covariate-drift tracking (`CovariateDriftResult`, `fit_covariate_drift_extra_wcp`), `PredeterminedChangeControlPlan` (`PCCP`), `ProfitLossAttributionTest` (`PLAT`, `PlatTrafficLightZone`), `EvaluatedCompetenceCriterion`, `RegulatoryComplianceAudit`, and `CloudLocalLoraArchitectureAudit`.
- **Label-Shift Adaptation (`drift::label_shift`, re-exported at root):** `LabelShiftDiagnostic`, `estimate_label_shift_four_methods` (Classify & Count, Black-Box Shift Estimation `BBSE`, Regularized `RLLS`, and `MLLS` with Bias-Corrected Temperature Scaling `BCTS`), `adjust_posterior_for_label_shift`, and `fit_calibrated_logistic_1d`.

## License

MIT License — Copyright (c) 2026 Silvano Neto.
