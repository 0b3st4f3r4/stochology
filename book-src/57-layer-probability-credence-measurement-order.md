# Chapter 57 — Layer 07: Probability Foundations, Imprecise Credences & Measurement Axiomatics Order

> Layer 07 (`multilayer.rs::KnowledgeOrder`, `KnowledgeOrder::L07ProbabilityCredenceMeasurement`) of Part V · Stratum II (`multilayer.rs::KnowledgeStratum`, `KnowledgeStratum::S2FormalMetrological`) · Unique Guard Combination `{T1Framing, T2Severity, T4Formalization}` · Signature `(2, 1, 0)` (`multilayer.rs::TriadicExposureRegime`, `TriadicExposureRegime::GovernanceBlindSpot`). It builds on [Layer 01](51-layer-micro-kinetic-sensorimotor-order.md), [Layer 06](56-layer-formal-computable-type-order.md) in $\mathcal{G}_1$ and maps into $\mathcal{G}_2$ via [`notes/The Multilayer Graph of Knowledge and Aim.md`](../notes/The%20Multilayer%20Graph%20of%20Knowledge%20and%20Aim.md).

## By the end of this chapter you can
1. Locate **Probability Foundations, Imprecise Credences & Representational Measurement (`L07`)** at the unique 3-combination `{T1Framing, T2Severity, T4Formalization}` in Stratum II (`multilayer.rs::KnowledgeStratum`, `KnowledgeStratum::S2FormalMetrological`).
2. Prove the mathematical duality between **Walley's Lower/Upper Previsions** $[\underline{P}(A), \overline{P}(A)]$ (`T1Framing`, `T2Severity`), **Manski Partial-Identification Bounds** (`epistemic.rs::EpistemicMarginState::from_ate_bounds`), and **Thermodynamic Landauer–Jarzynski Information Limits** $\langle e^{-\beta W} \rangle = e^{-\beta \Delta F}$ (`T4Formalization`).
3. Formalize the **Stevens–Suppes Scale-Type Hierarchy** (nominal, ordinal, interval, ratio) and prove why a `(2, 1, 0)` probability foundation without institutional governance (`k_gov = 0`) triggers **Collapse Proposition 3 (`synthesis.rs::CollapseProposition`, `CollapseProposition::Proposition3CynicalCapture`)** when interval ambiguity is weaponized as an agnotological alibi.

## 1. The problem: The Myth of the Sharp Prior and Kolmogorovian Closure

Classical statistical decision theory rests on Andrei Kolmogorov's 1933 axiomatization: a sample space $\Omega$, a $\sigma$-algebra $\mathcal{F}$ of pre-
enumerated events, and a single countably additive probability measure $P: \mathcal{F} \to [0, 1]$. In physical laboratory settings with controlled repetition,
this framework provides formidable analytical clarity. But in natural, clinical, and institutional worlds where the hypothesis space is open (`T1Framing`), the
assumption that an agent possesses a unique, infinitely sharp prior across all possible futures is an epistemic fiction.

As John Maynard Keynes argued in *A Treatise on Probability* (1921), probability reflects not merely a numerical ratio, but carries an independent dimension:
the **weight of argument** $V \in [0, 1]$, measuring how much relevant evidence has been accumulated
(`epistemic.rs::EpistemicMarginState::from_archive_clusters`). A coin tossed twice and showing one head yields an empirical frequency of $1/2$ with negligible
weight; a coin tossed ten thousand times yields $1/2$ with massive evidentiary weight. Conflating these two epistemic states under a single point mass
$P(H) = 0.5$ commits a Type III category error (`epistemic.rs::EpistemicMarginState::from_inquiry_framing`).

In the lineage of Bernard Koopman (1940), I. J. Good (1950), Cédric Smith (1961), and Peter Walley (1991), rational credence under incomplete information is
represented by a closed convex **credal set** $\mathcal{M}$ of probability measures, bounded by lower and upper previsions.

## 2. Imprecise Credal Sets, Choquet Capacities, and Manski Bounds (`T1`, `T2`)

For any event $A \in \mathcal{F}$, the lower prevision $\underline{P}(A)$ and upper prevision $\overline{P}(A)$ are defined as the infimum and supremum over the
credal set $\mathcal{M}$:

$$\underline{P}(A) = \inf_{P \in \mathcal{M}} P(A), \qquad \overline{P}(A) = \sup_{P \in \mathcal{M}} P(A).$$

These non-additive monotone measures satisfy conjugacy $\overline{P}(A) = 1 - \underline{P}(A^c)$ and super-additivity for disjoint events:

$$\underline{P}(A \cup B) \ge \underline{P}(A) + \underline{P}(B), \qquad \overline{P}(A \cup B) \le \overline{P}(A) + \overline{P}(B).$$

The interval width $\Delta_{\mathrm{cred}}(A) = \overline{P}(A) - \underline{P}(A) \ge 0$ quantifies Knightian ambiguity. When $\Delta_{\mathrm{cred}}(A) = 0$,
the agent operates in the Kolmogorovian point regime. When $\Delta_{\mathrm{cred}}(A) > 0$, no single Dutch-book argument can compel the agent to accept bets
outside the interval $[\underline{P}(A), \overline{P}(A)]$.

This formulation is dual to Charles Manski's partial-identification bounds in causal inference (`epistemic.rs::EpistemicMarginState::from_ate_bounds`). When
observational data suffers non-random selection or unobserved counterfactuals, point identification requires dogmatic, non-refutable exclusion restrictions. By
refusing these assumptions, Manski bounds bracket the Average Treatment Effect within $[\Delta_{\mathrm{lower}}, \Delta_{\mathrm{upper}}]$, providing severe
empirical tests (`T2Severity`) that do not claim unwarranted certainty.

## 3. Thermodynamic Information Erasure and Measurement Scales (`T4`)

Information acquisition and state formalization are physical processes bounded by thermodynamics. Rolf Landauer (1961) proved that erasing one bit of
informational entropy in any computational memory dissipates a minimum thermodynamic heat into the surrounding thermal reservoir:

$$W_{\mathrm{diss}} \ge k_B T \ln 2,$$

where $k_B$ is the Boltzmann constant and $T$ is temperature. In non-equilibrium statistical mechanics, Christopher Jarzynski's 1997 equality connects the
distribution of non-equilibrium work $W$ performed along arbitrary state transitions to the equilibrium Helmholtz free energy difference $\Delta F$:

$$\langle e^{-\beta W} \rangle = e^{-\beta \Delta F}, \qquad \beta = \frac{1}{k_B T}.$$

By Jensen's inequality, $\langle W \rangle \ge \Delta F$, proving that formalizing knowledge through physical measurement inevitably dissipates energy and
encounters finite-rate thermodynamic friction (`T4Formalization`).

Simultaneously, Representational Measurement Theory (Stevens 1946; Krantz, Luce, Suppes & Tversky 1971) dictates that assigning numbers to physical phenomena is
valid only relative to an empirical relational structure $(\mathcal{A}, \succsim)$ and its admissible transformation group $\Phi$:
- **Ordinal scale:** Invariant only under strictly increasing monotonic transformations $\phi(x) = f(x)$. Arithmetic means $\bar{x}$ are non-invariant; averaging Likert scores or ordinal hazard ranks produces phantom quantities. Only medians and stochastic dominance are mathematically sound (`vital.rs::VitalMarginState`).
- **Interval & Ratio scales:** Invariant under affine transformations $\phi(x) = ax + b$ ($a > 0$) or similarity transformations $\phi(x) = ax$. Only on ratio scales do convex Jensen gaps $\Delta_J = \mathbb{E}[L(X)] - L(\mathbb{E}[X])$ and heavy-tailed Hill moments represent real physical risks.

## 4. Why `(2, 1, 0)` Requires Institutional Governance Coupling

Layer 07 has the occupancy signature `(2, 1, 0)` (`multilayer.rs::TriadicExposureRegime`, `TriadicExposureRegime::GovernanceBlindSpot`). It formalizes epistemic
framing (`T1Framing`), empirical severity (`T2Severity`), and axiomatic scale structures (`T4Formalization`), but possesses zero governance sites (`k_gov = 0`).

An institution armed with imprecise probabilities and measurement theory without institutional governance (`T6Performative`, `T7Liability`) collapses into
**Proposition 3 (`synthesis.rs::CollapseProposition`, `CollapseProposition::Proposition3CynicalCapture`)**:
- Corporate or state actors weaponize wide credal intervals $[\underline{P}, \overline{P}]$ as an **agnotological alibi**: claiming that because statistical intervals include zero effect, regulatory action on endocrine disruptors, carbon emissions, or algorithmic bias must be deferred indefinitely.
- Technocrats exploit ordinal-scale non-invariance to re-scale consumer risk metrics, manufacturing an artificial appearance of safety while vital reserves are eroded.

Foundational probability and measurement require coupling to immutable pre-registered analysis vaults (`governance.rs::TemporalVault`) and statutory liability
rules (`L31`, `L32`) to prevent rigorous uncertainty from becoming an instrument of regulatory paralysis.

## Reproduce

```bash
cargo run --release --bin stochology -- --chapter 2
cargo run --release --bin stochology -- --chapter 3
```

## Bridge

Layer 07 (`multilayer.rs::KnowledgeOrder`, `KnowledgeOrder::L07ProbabilityCredenceMeasurement`, `{T1Framing, T2Severity, T4Formalization}`) propagates its local
and ancestor-cone margins through `multilayer.rs::MultilayerHomomorphism::evaluate` into [Chapter 58 — Layer 08: Metrological & Information-Geometric
Order](58-layer-metrological-information-geometric-order.md) (`KnowledgeOrder::L08MetrologicalInfoGeometric`, Stratum II, `{T3Archive, T4Formalization,
T6Performative}`).
