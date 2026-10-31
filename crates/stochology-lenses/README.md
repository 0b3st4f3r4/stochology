# `stochology-lenses`

The Five Transversal Lenses, Almost-Perfect Machine, Multi-Seed Acceptance Registry, and Domain Syntheses of **Stochology** in pure Rust (`std` only).

## Overview

`stochology-lenses` builds on `stochology-core` to provide:

1. **Information Lens (`information`):** 3-step Markov chain Data Processing Inequality (`DpiMarkovChain`), strong contraction hierarchy (`ContractionHierarchy`: $\eta_{\chi^2}(W) = \rho_m^2 \le \eta_{\mathrm{KL}}(W) \le \eta_{\mathrm{TV}}(W)$), `kl_divergence`, `chi_square_divergence`, `total_variation_distance`, `dobrushin_tv_contraction`, and `fano_lower_bound_binary`.
2. **Time Lens (`time`):** Itô variance drain ($g_{\text{time}} = \mu - \frac{1}{2}\sigma^2$), asymmetric Kelly fraction ($f^* = \frac{p}{a} - \frac{1-p}{b}$), absorbing ruin barriers, Langevin/MCMC mixing time, and `RobustKalmanControlAudit`.
3. **Scale & Tail Lens (`tail`):** Hill tail-index estimator ($\hat{\alpha}_{k,n}$), DEdH moment tail index (`dedh_moment_tail_index`), and moment-existence regime classification ($\alpha \le 1$, $1 < \alpha \le 2$, $\alpha > 2$).
4. **Structure & Invariance Lens (`invariance`):** Invariant Causal Prediction (`run_icp`, `IcpResult` with `env_ols_invariance_penalties`), `chow_invariance_f_stat`, nonlinear Hermite-basis ICP/NILE (`run_nonlinear_icp_nile`), `ShapleyCausalAudit`, and critical hidden confounding threshold $\Lambda^*$ (`hidden_confounding_threshold_lambda_star`, `cinelli_hazlett_robustness_value`).
5. **Decision & Calibration Lens (`decision`):** Cost-loss boundary $p^* = \frac{C}{\eta L}$ (`CostLossRule`), `EVPI`, pinball loss (`quantile_score`), isotonic PAV `CorpPavDecomposition` (Brier and Logarithmic scores via `compute_logarithmic`), `MurphyDiagram`, `CorpRocMurphyTriptych`, `ExactCoinCalibrationBenchmark`, `SaatyAhpRankReversalAudit`, and `LlmJudgePpiRectifier`.
6. **Almost-Perfect Machine & Multi-Seed Acceptance (`machine`, `acceptance`):** `AlmostPerfectMachine`, `AuthenticModelForm`, `IdentityClosureAudit`, and the 20-development / 200-confirmation seed acceptance engine (`evaluate_all_syntheses`, `canonical_registry_lines`, `verify_registry_content`).
7. **Domain & Empirical Syntheses:** `relational`, `analog`, `critical`, `stocks`, `triad`, `strategic`, and `society` (Stock-Flow Consistent `SfcFlowMatrix`, `CoupledPublicPrivateSocietySynthesis`, and `EmpiricalCoupledSocietyAudit`).

## License

MIT License — Copyright (c) 2026 Silvano Neto.
