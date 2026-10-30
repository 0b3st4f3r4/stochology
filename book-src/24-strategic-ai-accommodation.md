# 24. Strategic AI Six-Pair Methodological Accommodation

> **Three tempos.** **Detachment:** set aside the toolbox of current AI practice (feature attribution, single imputation, LLM-as-a-judge, uniform quantization, consistency-ratio rankings, Gaussian Kalman filters) as if each tool were neutral. **Abstraction:** each shortcut fails at a specific crossing between World, Model, and Decision; read together, the six repairs cover all six ordered pairs. **Return:** `do`-Shapley anchored on invariant causal prediction, Rubin's pooling rule, Prediction-Powered Inference, outlier-protected quantization, Laplacian and contagion identities, Belton-Gear rank reversal, and Huber innovation-weighted Kalman filtering. **Negative proof:** `gates -- --defect strategic_accommodation` (exit `1`).

This chapter gives the strategic synthesis the chapter it lacked in earlier editions. It is a Tier 3 synthesis gated by `G8`. The axis-to-pair labels used below come from the code's doc comments and are an **authorial** reading frame (`AGENTS.md` §II.C rule 7), not additional bridges. Unless a dataset is named, every number is **simulated** (canonical shielded run at `seed = 2026`). The specification is mirrored in [`docs/syntheses/strategic.md`](../docs/syntheses/strategic.md).

## Six Shortcuts and Their Repairs

As a worked example, consider a team shipping an AI-assisted decision service. It explains predictions with observational feature attributions, fills missing fields with column means, grades its outputs with a language-model judge, compresses the model to 4 bits by clipping, ranks vendors with a pairwise-comparison matrix, and tracks a noisy sensor with a Gaussian filter. Each step is common practice; each one, under the conditions simulated below, gives a wrong answer that looks right. The chapter pairs every shortcut with a repair and with a naive arm that the negative proof forces to fail.

The synthesis formalizes six axes (`StrategicAiAccommodationSynthesis` and the 6-dataset empirical coupler `EmpiricalStrategicAccommodationAudit`) under the same three-way verification contract: `shielded = false` executes the naive baseline, `all_identities_closed` closes on every seed in both modes, and literature constants are informative only. `G8` gates 15 named claims (11 seed-dependent, 4 `.det.`), each passing on $\ge 95\%$ of the 20 development seeds `2026..=2045` and of the 200 confirmation seeds `9_000_000..=9_000_199` (lowest one-sided 95% Clopper-Pearson bound on a confirmation pass rate: $0.977$); two further claims are discarded and the empirical coupler records one discarded finding (all three are listed below and none is approved). Unless a dataset is named, every number below is **simulated** (canonical shielded run at `seed = 2026`, printed by `labs --lenses`). The code's doc comments assign each axis to one of the Six Directed Pairs (Chapter 1, `book-src/01-layers-six-pairs.md`); these axis-to-pair labels are an **authorial** reading frame (`AGENTS.md` §II.C rule 7), not additional bridges. Enforced in Gate `G8` and `--defect strategic_accommodation`:

## 24.1 Axis 1 — Order, Connectivity, and Correlated Ensembles

**Axis 1 (`MinskyPapertGeometrySocietyOfMindAudit`; authorial label $L\_2 \to L\_1$).** Identities: Walsh parity orthogonality on $\{-1, +1\}^4$ (maximum lower-order correlation $0.0$), `XOR` order separation (best order-1 affine threshold on the searched grid $3/4$ vs. $4/4$ with the order-2 product feature $x\_0 x\_1$), and the correlated-ensemble variance identity $\sigma^2\bigl(\rho + \frac{1-\rho}{M}\bigr)$. Claims: a global reachability (graph search) operator classifies connected vs. disconnected ring patterns with accuracy $1.00$ vs. $0.51$ (chance level) for a pooled local-mass receptor; $M = 5$ weakly correlated voters ($\rho = 0.05$) reach Brier $0.110$ vs. $0.268$ for strongly correlated voters ($\rho = 0.85$) that also carry an **assumed**, hand-set framing bias of $0.28$ toward $0.5$, so the gap is not attributable to correlation alone.

## 24.2 Axis 2 — Causal Attribution, Imputation, and Trial Design

**Axis 2 (`ShapleyCausalRubinMiDoeAudit`; authorial label $L\_1 \to L\_2$).** Identities: exact 3-feature Shapley Efficiency, Null-Player, and Symmetry plus `PDP` cancellation (`ShapleyCausalAudit`), Rubin's $T\_M = \bar{U}\_M + (1 + M^{-1})B\_M$ (`RubinMultipleImputationAudit`), Manski bounds covering the full-sample mean under truncation $Y > 5.5$, and the GP ratio below exceeding $5$. Claims: intervening on the top feature chosen by ICP-anchored `do`-Shapley gives counterfactual MSE $0.000$ vs. $0.14$ for Rung-1 observational Shapley, which selects the downstream symptom $X\_2$; $M = 5$ stochastic-regression imputations under `MAR` retain $0.93$ of the true variance vs. $0.62$ for single mean imputation; trial sizing from `prospective_clustered_mde` (ICC $0.25$, $m = 25$, $\mathrm{DEFF} = 7$) reaches power $0.90$ vs. $0.17$ for a hard-coded naive allocation of 4 clusters. The deterministic claim named `gp_expected_improvement_ratio` is computed as a ratio of anchored `KennedyOHaganDiscrepancy` posterior standard deviations at an unexplored vs. an anchored input ($50.2$); no Bayesian-optimization loop or grid search is run. **Discarded claim** `strategic.a2.parametric_mar_mi_covers_true_mean_under_mnar` (passes on $0.0\%$ of confirmation seeds): despite the sealed name, the code runs no Multiple Imputation for this check; it tests whether a complete-case $\pm 1.96\,\mathrm{SE}$ interval on the truncated sample $Y > 5.5$ covers the full-sample mean.

## 24.3 Axis 3 — Retrieval, Judges, and Quantization

**Axis 3 (`GenerativeRagLlmJudgeCompressionAudit`; authorial label $L\_2 \to L\_3$).** Identities: Data Processing Inequality on a 4-node retrieval chain Corpus $\to$ Chunk $\to$ Embedding $\to$ Rerank (`DpiMarkovChain` over binary symmetric channels) and the Rogan-Gladen population inversion inside `LlmJudgePpiRectifier`. Claims: end-to-end $I(W; Z) = 0.382$ nats under the low-noise chunking chain (deterministic); a cost-loss cascade ($p^\ast = 0.20$) that escalates boundary cases to a frontier arm modelled as returning the true probability costs $17.34$ vs. $24.87$ per decision for a local model with a hand-set under-prediction ($0.35\,p$); outlier-protected `INT4` (values above the clip kept at full precision) retains $1.00$ of Pareto ($\alpha = 1.65$) $\mathrm{ES}\_{0.99}$ vs. $0.17$ for uniform clipping at $4.0$. **Discarded claim** `strategic.a3.llm_judge_deployed_abs_error_below_0.06` (Prediction-Powered Inference with $N = 800$, $n\_{\text{gold}} = 160$): passes on $88.0\%$ of confirmation seeds (the code notes a PPI standard error $\approx 0.035$ at this $n\_{\text{gold}}$), so it is refuted, not approved.

## 24.4 Axis 4 — Networks, Hubs, and Contagion

**Axis 4 (`ComplexNetworkHubContagionAudit`; authorial label $L\_3 \to L\_1$).** Identities on a fixed 12-node hub-and-spoke graph: $L\mathbf{1} = \mathbf{0}$, handshaking $\sum\_i k\_i = 24$, Fiedler $\lambda\_2 = 0.362 > 0$, and $\langle k^2 \rangle / \langle k \rangle > \langle k \rangle$. Claims: under simulated delay shocks at hub 0, a schedule with **assumed** decoupling slack (26–42 min) keeps $0.85$ of airports on time vs. $0.00$ with 5–10 min slack; Centola threshold-2 complex contagion on a 20-node ring lattice reaches adoption $1.00$ from adjacent seeds vs. $0.10$ from distant seeds (deterministic).

## 24.5 Axis 5 — Multi-Criteria Ranking, Regulation, and Provenance

**Axis 5 (`MultiCriteriaAhpRegulatoryPkmAudit`; authorial labels $L\_1 \to L\_3$ & $L\_3 \to L\_2$).** Identities (`SaatyAhpRankReversalAudit`): an exactly consistent $3 \times 3$ reciprocal matrix has $\lambda\_{\max} = 3$ and $\mathrm{CI} = 0$ (no random index or Consistency Ratio is computed); Belton-Gear rank reversal under column-normalized scoring when an exact copy of $B$ is added, with Ideal-mode scores unchanged; and the `ICE` product-of-means bias $= -\mathrm{Cov}(I, E) = +10.0$. Claims: the portfolio contrast $18.26$ vs. $11.80$ is **constructed** (the naive arm is hard-coded to realize the rank-reversed choice), so it illustrates the cost of a reversal rather than detecting one. The deterministic regulatory/provenance claim combines an operational proxy checklist mapped to EU AI Act Arts. 9/10/14/15 and LGPD Art. 20, evaluated on **assumed** inputs ($3.2$ bit/s oversight load, $12.61$ decibans, $\mathrm{REL}\_{\mathrm{CORP}} = 0.004$; naive arm $12.6$, $2.86$, $0.085$), with `EpistemicPkmProvenanceAudit` over hand-set `ClaimProvenanceClass` transitions (the naive arm launders an `UnanchoredLlmSynthesis` node into `Measured`). It uses no dataset and is not a statutory compliance certificate. The same checklist exists as `RegulatoryComplianceAudit` (`stochology_governance::drift`); `gates -- --audit-card` evaluates it with hard-coded Art. 9/10 inputs ($\hat{\alpha} = 1.25$, `true`, `true`) and the deciban input floored at $10$, so its `approved=true` line is a proxy, not a legal determination.

## 24.6 Axis 6 — Robust Filtering and Closed-Loop Control

**Axis 6 (`RobustKalmanClosedLoopControlAudit`; authorial label $L\_2 \leftrightarrow L\_3$).** Identities (`RobustKalmanControlAudit`): two-sensor precision fusion ($\sigma\_{\text{fused}}^2 = 0.2304$) and the Joseph-form covariance update equal to the information-form posterior. Claims: a Huber innovation-weighted scalar Kalman filter (Joseph update) tracks a state observed with simulated heavy-tailed glitches on every tenth step at RMSE $0.189$ vs. $1.47$ for the Gaussian $L\_2$ filter; a fixed safety-buffer clearance ($0.85$, labelled Tube-MPC) survives $1.00$ of simulated disturbances vs. $0.59$ for the certainty-equivalent clearance $0.05$.

## 24.7 Real-Data Coupler (`EmpiricalStrategicAccommodationAudit`)

**Real-data coupler (`EmpiricalStrategicAccommodationAudit`, single run, gated in `G8`).** **Measured**: `bank_calls.csv` clustered MDE inflation $\sqrt{\mathrm{DEFF}(0.90)} = 5.32\times$; `danish_fire.csv` $\mathrm{ES}\_{0.99}$ retention $1.00$ for outlier-protected quantization vs. $0.17$ for clipping at $q\_{0.95}$; `nsw_experimental.csv` / `cps_observational.csv` Meng $n\_{\mathrm{eff}}$ below $40\%$ of the CPS sample and Manski width above $1{,}000$ (thresholds checked; values not printed); `nyc_flights_2023_h1.csv` CORP identity closure on hand-set logistic probabilities. `atlantic_storms.csv`: ICP accepts only the pressure deficit $X\_0$, and the printed `do`-Shapley share $1.00$ then follows from hard-coded linear coefficients, so it is not an estimated attribution. `vic_elec.csv`: the first 400 measured half-hourly demand slots with **simulated** $+4500$ MW glitches every 20 slots give Huber-Kalman RMSE $138.9$ vs. $619.3$ MW for the $L\_2$ filter (measured series + simulated glitches). **Discarded finding**: "uniform 4-bit clipping at $q\_{0.95}$ preserves `danish_fire.csv` $\mathrm{ES}\_{0.99}$ within $10\%$" is refuted (retention $0.17$, i.e. $\approx 83\%$ erased; the finding text in the code says "> 65%").

## Computational Audit in Rust

`StrategicAiAccommodationSynthesis::evaluate(2026)` and the coupler `EmpiricalStrategicAccommodationAudit` run in `labs --all` and in Gate `G8`:

~~~bash
cargo run --release --bin labs -- --all
cargo run --release --bin gates
~~~

~~~text
  [Strategic AI Six-Pair Accommodation Synthesis] Axis1_Minsky(Walsh_corr=0.0e0, Conn_acc=1.00 vs 0.51, SoM_Brier=0.110 vs 0.268) | Axis2_CausalXAI_MI_DoE(SHAP_do_MSE=0.000 vs 0.14, Rubin_MI_var_ret=0.93, Clustered_Power=0.90, GP_EI_ratio=50.2) | Axis3_GenAI_RAG_Judge(RAG_MI=0.382 nats, PPI_err=0.011 vs bias=0.188, FinOps_cost=17.34 vs 24.87, INT4_ES99_ret=1.00 vs 0.17) | Axis4_Networks(Fiedler_l2=0.362, Hub_survival=0.85 vs 0.00, Centola_adopt=1.00 vs 0.10) | Axis5_MCDM_AIAct_PKM(AHP_val=18.26 vs 11.80, ICE_bias=+10.0, AIAct_LGPD=true, PKM_audit=true) | Axis6_Kalman_MPC(Huber_RMSE=0.189 vs L2=1.47, Tube_MPC_surv=1.00 vs CE=0.59) | Emp6CSVs(Storms_SHAP_p0=1.00, Bank_MDE_ratio=5.32x, Fire_ES99_prot=1.00 vs clip=0.17, Vic_Huber_RMSE=138.9 vs L2=619.3) | identities=true | discarded=["parametric Multiple Imputation assuming MAR covers the true population mean under MNAR truncation Y > 5.5", "PPI LLM-as-a-Judge absolute error < 0.06 at n_gold = 160 (passed 19/20 dev seeds but 0.880 on 200 confirmation seeds due to SE ~ 0.035)", "danish_fire.csv: uniform 4-bit activation clipping at q95 preserves tail Expected Shortfall ES_0.99 within 10% (refuted: clipping erases > 65% of ES_0.99)"] | verified=true
~~~

## Negative Proof

With `shielded = false`, every axis runs its naive baseline while the identities still close:

~~~bash
cargo run --release --bin gates -- --defect strategic_accommodation
~~~

~~~text
NEGATIVE PROOF [strategic_accommodation] BLOCKED SUCCESSFULLY on all 20 development seeds through seed-dependent claims (identities closed on every seed): at seed 2026 diameter-limited connectivity acc=0.51, homogeneous voter Brier=0.268, Rung-1 symptom SHAP do-MSE=0.14, single-mean imputation var retention=0.62, naive i.i.d. clustered power=0.17, uncalibrated LLM-Judge err=0.199, clipped INT4 ES99 retention=0.17, unbuffered hub survival=0.00, rank-reversed AHP portfolio val=11.80, L2 Kalman glitch RMSE=1.47, certainty-equivalent MPC survival=0.59
~~~

## Provenance Ledger

| Number | Class | Source |
| --- | --- | --- |
| Connectivity accuracy $1.00$ vs. $0.51$; voter Brier $0.110$ vs. $0.268$ | Simulated (the correlated arm carries an assumed framing bias of $0.28$) | `seed = 2026` |
| `do`-Shapley MSE $0.000$ vs. $0.14$; imputation variance retention $0.93$ vs. $0.62$; power $0.90$ vs. $0.17$ | Simulated | `seed = 2026` |
| RAG $I(W;Z) = 0.382$ nats; cascade cost $17.34$ vs. $24.87$; `INT4` $\mathrm{ES}\_{0.99}$ retention $1.00$ vs. $0.17$ | Simulated | `seed = 2026` |
| Hub survival $0.85$ vs. $0.00$ | Simulated (decoupling slack is assumed) | `seed = 2026` |
| AHP portfolio $18.26$ vs. $11.80$ | Constructed (the naive arm is hard-coded) | `MultiCriteriaAhpRegulatoryPkmAudit` |
| Oversight load $3.2$ bit/s, $12.61$ decibans, $\mathrm{REL}\_{\mathrm{CORP}} = 0.004$ | Assumed | regulatory proxy checklist inputs |
| Huber-Kalman RMSE $0.189$ vs. $1.47$; Tube-MPC survival $1.00$ vs. $0.59$ | Simulated | `seed = 2026` |
| MDE inflation $5.32\times$ | Measured | `bank_calls.csv` |
| $\mathrm{ES}\_{0.99}$ retention $1.00$ vs. $0.17$ | Measured | `danish_fire.csv` |
| Huber-Kalman RMSE $138.9$ vs. $619.3$ MW | Measured series with simulated glitches | `vic_elec.csv` |
| `do`-Shapley share $1.00$ | Follows from hard-coded coefficients once ICP accepts $X\_0$ | `atlantic_storms.csv` |
| Discarded: MAR interval coverage ($0.0\%$), PPI judge error ($88.0\%$), uniform `INT4` clipping on `danish_fire.csv` | Simulated / measured, **discarded** | `gates` |

## Exercises

1. **(Concept: Observational vs. Interventional Attribution)** In Axis 2, the observational Shapley ranking selects the downstream symptom $X\_2$ as the most important feature. Explain why intervening on a symptom cannot change the outcome, and why ICP anchoring fixes the ranking.
2. **(Calculation: Correlated Ensembles)** Using the identity $\sigma^2\bigl(\rho + \frac{1-\rho}{M}\bigr)$, compute the variance of the average of $M = 5$ voters with unit variance at $\rho = 0.05$ and at $\rho = 0.85$. What is the floor as $M \to \infty$ in each case?
3. **(Calculation: Rubin's Rule)** With $M = 5$ imputations, a mean within-imputation variance $\bar{U}\_M = 0.04$, and a between-imputation variance $B\_M = 0.01$, compute the total variance $T\_M$. What would single mean imputation report instead, and why is that too small?
4. **(Concept: Reading a Discarded Claim)** The claim `strategic.a3.llm_judge_deployed_abs_error_below_0.06` passed on $19/20$ development seeds and on $88.0\%$ of confirmation seeds. State in one sentence how a derived text must quote it.

<details>
<summary><strong>Analytical &amp; Computational Solutions (Click to Expand)</strong></summary>

1. **Attribution:** Observational Shapley credits whatever co-varies with the outcome, including descendants of the cause. Setting $\mathrm{do}(X\_2)$ cuts $X\_2$ from its parents but leaves the outcome's mechanism untouched, so the counterfactual error stays at $0.14$. ICP keeps only features whose conditional relation to the outcome is invariant across environments; a symptom is not, so the anchored ranking selects the cause (MSE $0.000$, simulated).
2. **Ensembles:** At $\rho = 0.05$: $0.05 + 0.95/5 = 0.24$. At $\rho = 0.85$: $0.85 + 0.15/5 = 0.88$. As $M \to \infty$ the variance tends to $\rho$: $0.05$ and $0.85$. Adding correlated voters cannot remove the shared error.
3. **Rubin:** $T\_M = \bar{U}\_M + (1 + M^{-1}) B\_M = 0.04 + 1.2 \times 0.01 = 0.052$. Single mean imputation treats the filled values as observed, reporting only within-sample variance and shrinking the spread of the imputed column, so its intervals are too narrow.
4. **Quoting:** It must be quoted as a discarded (refuted) claim with its confirmation pass rate, for example: "PPI judge error below $0.06$ is discarded: it passed on $88.0\%$ of 200 confirmation seeds, below the $95\%$ threshold (simulated)."

</details>

## Summary

- **Axis 1:** global operators separate what local receptors cannot (accuracy $1.00$ vs. $0.51$, simulated); correlated voters keep a variance floor of $\rho$.
- **Axis 2:** ICP-anchored `do`-Shapley, Rubin pooling, and clustered trial sizing repair observational attribution, single imputation, and naive allocation. The MAR-coverage claim is **discarded**.
- **Axis 3:** the DPI bounds what a retrieval chain can carry; PPI rectifies a biased judge, though the error-below-$0.06$ claim is **discarded**; outlier-protected `INT4` keeps the tail that clipping erases.
- **Axis 4:** Laplacian identities hold on the hub graph; assumed slack keeps $0.85$ of airports on time; complex contagion needs adjacent seeds.
- **Axis 5:** consistent AHP still reverses ranks under normalization; the regulatory checklist is a proxy on assumed inputs, not a compliance certificate.
- **Axis 6:** Huber innovation weighting tracks through heavy-tailed glitches where the Gaussian filter fails; on `vic_elec.csv` the series is measured and the glitches are simulated.
