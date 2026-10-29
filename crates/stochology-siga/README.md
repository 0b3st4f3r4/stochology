# `stochology-siga`

The 4-Filter Eliminatory Funnel and 8-Step **SIGA** Operational Roadmap (*Survey, Interrogate, Generate, Act*) of the **Stochology — CAPIBA Protocol** (*Calibrated, Anytime-valid, Performative-aware, Invariant, Bitemporal Auditing Protocol*) in pure Rust (`std` only).

## Overview

`stochology-siga` implements the operational workflow and anytime-valid sequential auditing primitives:

- **4-Filter Eliminatory Funnel (`funnel`):**
  1. *Filter 1 (Decision, EVPI & Resolution $\Phi_5$):* Non-degenerate cost-loss threshold $p^* = \frac{C}{\eta L} \in (0, 1)$, positive net $\mathrm{EVPI} > C_{\text{total}}$, and operational resolution parity (`verify_resolution_phi5`).
  2. *Filter 2 (Independent Unit $\Phi_2$ & Causal Markov Blanket):* Moulton-Campbell cluster-adjusted effective tail sample size $n_{\mathrm{eff}}(p)$ and causal Markov blanket without Berkson collider bias (`SimpsonCausalParadoxAudit`, `LongContextDpiDilutionAudit`).
  3. *Filter 3 (Generative Physics & Prior Predictive Check):* Simulation-Based Calibration rank uniformity (`sbc_rank_chi2`) and `SHELF` quartile prior elicitation (`fit_shelf_beta_prior`), plus prospective `PPI` gold-label sizing (`prospective_ppi_gold_sample_size`, `ppi_effective_sample_size_multiplier`).
  4. *Filter 4 (Cryptographic Pre-Registration $\Phi_3$ & Cross-Authorship $\Phi_4$):* Valid SHA-256 pre-registration hash, independent role separation (`null_model` / `decision_rule` $\neq$ `adversarial_reviewer`), and automatic numeric exit clause.
- **8-Step SIGA Operational Roadmap (`roadmap`):**
  - End-to-end roadmap execution (`run_full_siga_roadmap`, `run_full_siga_roadmap_empirical`, `SigaExecutionReport`) and Beta-Binomial SBC rank simulation (`simulate_sbc_beta_binomial_ranks`).
  - Anytime-valid non-parametric betting martingales (`EvidenceProcess`: `evaluate`, `from_grapa_betting`, `gro_betting`), `SaviConfidenceSequence`, and `VarEsEBacktest` (`GREE` e-backtesting for VaR and Expected Shortfall).
  - Gigerenzer natural-frequency decision memoranda (`NaturalFrequencyMemorandum`), Bridge $L_2 \to L_3$ clinical/cognitive triage auditing (`AlertFatigueAudit`), correlated multimodal sensor fusion (`MultimodalCorrelatedFusionAudit`), and prospective Pre-Mortem counterfactual root-cause analysis (`PreMortemCounterfactualRootCauseAudit`).

## License

MIT License — Copyright (c) 2026 Silvano Neto.
