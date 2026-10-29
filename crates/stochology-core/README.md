# `stochology-core`

Core primitives of the **Stochology — CAPIBA Protocol** (*Calibrated, Anytime-valid, Performative-aware, Invariant, Bitemporal Auditing Protocol*) in pure Rust (`std` only, zero external dependencies).

## Overview

`stochology-core` implements the foundational ontological layers, boundary invariants ($\Phi_1$–$\Phi_5$), and structured Epistemological Card contract:

- **Deterministic Simulability ($\Phi_1$, `rng`):** `SplitMix64` (`seed = 2026`), lock-free `.split(stream_id)` and `.split_fork_join_pair(branch_tag)` substream trees (`MonteCarloSplitTreeAudit`), and Box-Muller Gaussian sampling.
- **Ontological Layers & Forward Bridges (`layers`):**
  - *World Models ($L_1 \to L_2$):* `MengAudit` (`data_defect_correlation_ddc`, `data_defect_index_ddi`, `law_of_large_populations_z`) and non-parametric `ManskiBounds` with fractional Lee (2009) trimming bounds (`lee_trimming_bounds`, `lee_tipping_point_delta`).
  - *Decision Models ($L_2 \to L_3$):* `NonlinearSummary` ($f(\mathbb{E}[X])$, $\mathbb{E}[f(X)]$, exact Jensen's Gap $\Delta_J$, tail quantiles $q_{05}, q_{95}$, and exact empirical `CRPS`).
  - *World Decisions ($L_3 \to L_1$):* `PerformativeSystem` separating Performative Stability ($\theta_{\mathrm{PS}}$) from Stackelberg Performative Optimality ($\theta_{\mathrm{PO}}$ via `perfgd_trajectory`), RRM contraction auditing ($\frac{\varepsilon\beta}{\gamma} < 1$), `GoodhartTaxonomyAudit`, `DpiOperationalStage`, `RubinMultipleImputationAudit`, `EpistemicPkmProvenanceAudit`, and `ClaimProvenanceClass` (`OrderedLayerPair`).
- **Severe Uncertainty (`uncertainty`):** Stirling-Walker 2×2 uncertainty matrix, `KennedyOHaganDiscrepancy` anchored Gaussian Process, and `InfoGapOption` coupled with Sniedovich's global Maximin safeguard.
- **Boundary Invariants (`invariants`):** `ExceedanceDesignEffect` (Moulton-Campbell $\tilde{m} = \bar{m}(1 + \mathrm{CV}^2)$, `prospective_clustered_mde`, and threshold exceedance $n_{\mathrm{eff}}(p)$), `OperationalWeightingGap`, `ChouldechovaAudit` ($\mathrm{FPR}_A / \mathrm{FPR}_B$ and `minmax_penalty_disparity`), and `verify_resolution_phi5`.
- **Epistemological Card & Cryptographic Pre-Registration (`card`, `sha256`):** Structured YAML serialization/deserialization (`EpistemologicalCard::from_yaml` / `to_yaml`), `EpistemologicalCard::audit()`, and pure-Rust FIPS 180-4 SHA-256 (`sha256_hex`).

## License

MIT License — Copyright (c) 2026 Silvano Neto.
