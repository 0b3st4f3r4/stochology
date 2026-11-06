# Chapter 59 — Layer 09: Experimental Design, Complex Survey Sampling & Causal Methodology Order

> Layer 09 (`multilayer.rs::KnowledgeOrder`, `KnowledgeOrder::L09ExperimentalSamplingCausal`) of Part V · Stratum II (`multilayer.rs::KnowledgeStratum`, `KnowledgeStratum::S2FormalMetrological`) · Unique Guard Combination `{T1Framing, T2Severity, T3Archive}` · Signature `(3, 0, 0)` (`multilayer.rs::TriadicExposureRegime`, `TriadicExposureRegime::PureEpistemicConcentration`). It builds on [Layer 07](57-layer-probability-credence-measurement-order.md), [Layer 08](58-layer-metrological-information-geometric-order.md) in $\mathcal{G}_1$ and maps into $\mathcal{G}_2$ via [`notes/The Multilayer Graph of Knowledge and Aim.md`](../notes/The%20Multilayer%20Graph%20of%20Knowledge%20and%20Aim.md).

## By the end of this chapter you can
1. Identify **Experimental Design, Complex Survey Sampling & Causal Methodology (`L09`)** as the unique `(3, 0, 0)` combination `{T1Framing, T2Severity, T3Archive}` across the 35 orders of knowledge in $\mathcal{G}_1 \leftrightarrow \mathcal{G}_2$.
2. Unify **Neyman–Rubin Potential Outcomes & Fisherian Randomization** (`T2Severity`), **Kish Weighting & Moulton Intra-Cluster Correlation** (`T3Archive`, `epistemic.rs::EpistemicMarginState::from_archive_clusters`), and **Simonsohn Specification Curve Multiverse Analysis** (`T1Framing`, `epistemic.rs::EpistemicMarginState::from_inquiry_framing`).
3. Prove by the **Disciplinary Incompleteness Theorem** why `L09` is the empirical realization of the unbuffered agent (`engine.rs::StochologicalAgent::epistemic_only`), and why executing pure methodology without Vital (`T4`, `T5`) and Governance (`T6`, `T7`) coupling triggers **Combined Propositions 2 & 3 (`synthesis.rs::CollapseProposition`, `CollapseProposition::CombinedPropositions2And3`)** with 100% ruin probability.

## 1. The problem: The Fundamental Problem of Causal Inference

Causal inference addresses a missing-data challenge formalized by Jerzy Neyman (1923), Ronald Fisher (1935), and Donald Rubin (1974).
For any unit $i$ exposed to binary intervention $D_i \in \{0, 1\}$, there exist two potential outcomes: $Y_i(1)$ under treatment and
$Y_i(0)$ under control. The individual causal effect $\tau_i = Y_i(1) - Y_i(0)$ is strictly unobservable because the realized outcome
is:

$$Y_i = D_i Y_i(1) + (1 - D_i) Y_i(0).$$

Physical randomization solves this by guaranteeing orthogonality between treatment assignment and potential outcomes:
$D_i \perp (Y_i(1), Y_i(0))$. Under randomization, the difference in observed sample means unbiasedly identifies the Average Treatment
Effect:

$$\mathbb{E}[Y \mid D = 1] - \mathbb{E}[Y \mid D = 0] = \mathbb{E}[Y(1) - Y(0)] = \mathrm{ATE}.$$

In non-experimental archives, this identity fails. Confounding variables, unobserved selection on gains, and voluntary participation
distort naive comparisons. Establishing severe causal claims requires coordinating all three epistemic gates: framing the specification
space (`T1Framing`), subjecting hypotheses to severe identification bounds (`T2Severity`), and correcting archival survey clustering
(`T3Archive`).

## 2. The Pure Epistemic Triad: Multiverse (`T1`), ATE Bounds (`T2`), and Clusters (`T3`)

A complete empirical methodology clears three gates in sequence:
1. **`T1Framing` (Multiverse Specification Curve Analysis):** In complex observational datasets, researchers possess dozens
   of defensible choices regarding covariates, functional forms, and outlier exclusions. Cherry-picking a single specification that
crosses the nominal $p < 0.05$ threshold constitutes researcher $p$-hacking. Uri Simonsohn, Joseph Simmons, and Leif Nelson's
multiverse analysis executes all $K$ defensible specifications simultaneously
(`epistemic.rs::EpistemicMarginState::from_inquiry_framing`). The multiverse robustness ratio:

$$R_{\mathrm{multi}} = \frac{1}{K}\sum_{k=1}^K \mathbb{I}\{\text{specification } k \text{ confirms hypothesis}\}$$

screens out fragile knife-edge findings and exposes Type III framing errors.
2. **`T2Severity` (Experimental Benchmarks vs. Observational Collapse):** On Robert LaLonde's National Supported Work (NSW)
   demonstration benchmark, randomized assignment (`data/nsw_experimental.csv`, $n = 445$ [Measured]) yields an empirical experimental ATE of
`+$1,794.34`. When the randomized control arm is replaced by the non-random Current Population Survey archive (`data/cps_observational.csv`, $n = 15{,}992$ [Measured]), naive regression collapses to `-$8,497.52`. Reconciling this gap requires non-parametric Manski worst-case bounds and Lee (2009) monotone selection trimming bounds (`epistemic.rs::EpistemicMarginState::from_ate_bounds`), testing treatment efficacy without fragile parametric crutches.
3. **`T3Archive` (Survey Weights and Moulton Cluster Design Effects):** Observational surveys rarely follow independent and
   identically distributed sampling. In stratified sampling with sampling weights $w_i$, Leslie Kish showed that variance inflates
by the weight dispersion:

$$n_{\mathrm{Kish}} = \frac{\bigl(\sum_{i=1}^n w_i\bigr)^2}{\sum_{i=1}^n w_i^2}.$$

When units are clustered into $G$ groups of average size $m$ with intra-cluster correlation $\rho_{\mathrm{cl}}$, Brent Moulton
demonstrated that nominal standard errors are understated by the design effect:

$$\mathrm{DEFF}_{\mathrm{Moulton}} = 1 + (m - 1)\rho_{\mathrm{cl}} \implies n_{\mathrm{eff}} = \frac{n_{\mathrm{Kish}}}{\mathrm{DEFF}_{\mathrm{Moulton}}}$$

(`epistemic.rs::EpistemicMarginState::from_archive_clusters`). Treating clustered observations as independent overstates statistical
precision by up to an order of magnitude.

## 3. The `(3, 0, 0)` Singularity and the Epistemic-Only Ruin Regime

Among all $\binom{7}{3} = 35$ three-site combinations, Layer 09 is the unique triple whose sites all belong to Axis 1
($S_{\mathrm{epi}}$):

$$\mathrm{guards}(L_{09}) = \{T_1,\, T_2,\, T_3\}, \qquad \mathrm{occupancy}(L_{09}) = (3,\, 0,\, 0)$$

(`multilayer.rs::TriadicExposureRegime`, `TriadicExposureRegime::PureEpistemicConcentration`). It represents the pure methodologist: an
agent who calculates exact Manski bounds, computes Kish cluster design effects, and maps complete multiverse curves, yet operates with
zero vital buffer reserves ($S_{\mathrm{vital}} = 0$) and zero institutional veto authority ($S_{\mathrm{gov}} = 0$).

In the sequential mediation environment `CommunityReserveEnv` (`crates/stochology/src/engine.rs`), this corresponds exactly to
`engine.rs::StochologicalAgent::epistemic_only`. When evaluated across 24 sequential periods under heavy-tailed environmental shocks,
this agent experiences 100% absorption at the ruin boundary:

$$\mathrm{Ruin}(L_{09}^{\mathrm{uncoupled}}) = 1.000$$

under **Combined Propositions 2 and 3 (`synthesis.rs::CollapseProposition`, `CollapseProposition::CombinedPropositions2And3`)**
[Simulated]. Knowing the exact mathematical bounds of uncertainty while operating with zero physical margin and zero governance leads
to ruin. Methodology achieves survival only when coupled through the multilayer DAG to the ancestor layers that supply the axes the `(3, 0, 0)` triple
cannot carry: physical buffers (`L04`, which supplies both `T4Formalization` and `T5Temporal`, and `L08`, which supplies `T4Formalization`) and liability
(`L01`, `T7Liability`). `L30` and `L31` sit downstream of this layer and therefore cannot export margin back to it.

## Reproduce

```bash
cargo run --release --bin stochology -- --chapter 2
cargo run --release --bin stochology -- --chapter 3
cargo run --release --bin stochology -- --chapter 4
```

## Bridge

Layer 09 (`multilayer.rs::KnowledgeOrder`, `KnowledgeOrder::L09ExperimentalSamplingCausal`, `{T1Framing, T2Severity, T3Archive}`)
propagates its local and ancestor-cone margins through `multilayer.rs::MultilayerHomomorphism::evaluate` into [Chapter 60 — Layer 10:
Cryptographic Protocols, Zero-Knowledge Proofs & Adversarial Verification Order](60-layer-cryptographic-zero-knowledge-adversarial-order.md)
(`KnowledgeOrder::L10CryptographicZeroKnowledgeAdversarial`, Stratum II, `{T2Severity, T6Performative, T7Liability}`).
