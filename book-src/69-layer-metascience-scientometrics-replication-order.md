# Chapter 69 — Layer 19: Metascience, Scientometrics, Publication Bias & Replication Order

> Layer 19 (`multilayer.rs::KnowledgeOrder`, `KnowledgeOrder::L19MetascienceScientometricsReplication`) of Part V · Stratum IV (`multilayer.rs::KnowledgeStratum`, `KnowledgeStratum::S4ClinicalFormative`) · Unique Guard Combination `{T2Severity, T3Archive, T6Performative}` · Signature `(2, 0, 1)` (`multilayer.rs::TriadicExposureRegime`, `TriadicExposureRegime::VitalBlindSpot`). It builds on [Layer 08](58-layer-metrological-information-geometric-order.md), [Layer 09](59-layer-experimental-sampling-causal-order.md), [Layer 18](68-layer-pharmacovigilance-regulatory-toxicology-order.md) in $\mathcal{G}_1$ and maps into $\mathcal{G}_2$ via [`notes/The Multilayer Graph of Knowledge and Aim.md`](../notes/The%20Multilayer%20Graph%20of%20Knowledge%20and%20Aim.md).

## By the end of this chapter you can
1. Locate **Metascience, Scientometrics, Publication Bias & Replication (`L19`)** at the unique 3-combination `{T2Severity, T3Archive, T6Performative}` in Stratum IV (`multilayer.rs::KnowledgeStratum`, `KnowledgeStratum::S4ClinicalFormative`).
2. Formalize **John Ioannidis's Positive Predictive Value (PPV) Theorem** and **Anytime *e-BH* False Discovery Control** (`T2Severity`, `epistemic.rs::EpistemicMarginState::from_inquiry_framing`, `epistemic.rs::EpistemicMarginState::from_ate_bounds`), **Robert Rosenthal's File-Drawer Effect & Egger Regression** (`T3Archive`, `epistemic.rs::EpistemicMarginState::from_meng_big_data`), and **Smaldino & McElreath's *Natural Selection of Bad Science*** (`T6Performative`, `governance.rs::GovernanceState::from_ex_ante_gate`, `synthesis.rs::ReflexiveBrakeAudit`).
3. Prove by the **Disciplinary Incompleteness Theorem** why a `(2, 0, 1)` metascientific audit lacking $S_{\mathrm{vital}}$ (`T4`, `T5`) triggers **Collapse Proposition 2 (`synthesis.rs::CollapseProposition`, `CollapseProposition::Proposition2ImpotentLucidity`)** when replication initiatives require multi-year horizons ($\mathrm{TTR}_{\mathrm{replication}} \sim 5\text{--}10\text{ years}$) while unverified clinical interventions or algorithmic policies are deployed in real time ($\mathrm{TTS}_{\mathrm{survival}} \ll \mathrm{TTR}$).

## 1. The problem: The Epistemic Reflexivity of the Scientific Enterprise

Modern empirical science investigates the world through rigorous observation, mathematical formalization, and statistical
hypothesis testing. Metascience turns these exact tools reflexively upon the scientific enterprise itself. Tracing from Robert K.
Merton's 1942 institutional sociology of science (the CUDOS norms: communalism, universalism, disinterestedness, organized
skepticism) and Derek de Solla Price's scientometrics (1963) to contemporary reproducibility research, metascience investigates
how scientific institutions produce, filter, and reward empirical findings.

The impetus for this reflexivity emerged from the replication crisis. In 2015, the Open Science Collaboration attempted to
replicate 100 landmark experimental psychology findings published in top-tier journals; only 36% produced statistically
significant replications, with mean effect sizes shrinking by half. Similar crises unfolded in pre-clinical oncology (Errington
et al. 2021) and experimental economics.

These failures are not isolated instances of scientific fraud. They represent structural pathologies of an institutional reward
system that commodifies positive findings ($p < 0.05$) while ignoring statistical severity and archival completeness.

## 2. The Mathematical Anatomy of False Findings: PPV, File Drawers, and Metric Selection (`T2`, `T3`, `T6`)

Metascientific degradation operates across three coupled sites:
1. **`T2Severity` (John Ioannidis's Positive Predictive Value Theorem):** In his 2005 landmark paper, John Ioannidis
   proved why most published research findings are false. Let $R = P(H_1)/P(H_0)$ be the pre-study odds that a tested
scientific relationship is true, $1 - \beta$ be statistical power, $\alpha$ be the nominal false-positive rate (typically
$0.05$), and $u \in [0, 1]$ represent researcher degrees of freedom ($p$-hacking, flexible stopping rules, and selective
subgroup reporting). The probability that a statistically significant finding reflects a true effect (the Positive Predictive
Value) is:

$$\mathrm{PPV} = \frac{(1 - \beta)R + u\beta R}{(1 - \beta)R + u\beta R + \alpha + u(1 - \alpha)}.$$

In exploratory scientific fields testing novel hypotheses with low pre-study odds ($R \le 0.10$) and typical statistical power
($1 - \beta \approx 0.50$), even modest researcher bias ($u = 0.20$) forces $\mathrm{PPV} < 0.30$. Under multiple testing across
laboratories, classical Benjamini–Hochberg procedures fail due to complex cross-study dependencies; the *e-BH* procedure
(`epistemic.rs::EpistemicMarginState::from_inquiry_framing`) over *e-values*
(`epistemic.rs::EpistemicMarginState::from_ate_bounds`) guarantees exact False Discovery Rate control under arbitrary dependence.
2. **`T3Archive` (Robert Rosenthal's File-Drawer Filter and Egger Regression):** Scientific journals exhibit severe
   publication bias: studies with $p < 0.05$ are published eagerly, while null results are relegated to file drawers (Rosenthal
1979). In funnel plots of effect size versus precision ($1/\mathrm{SE}$), this filter creates profound asymmetry, formally
tested via Egger regression. The resulting positive data-defect correlation $\rho_{R,Y} > 0$
(`epistemic.rs::EpistemicMarginState::from_meng_big_data`) inflates published effect sizes ($M$-error) and reverses effect
signs ($S$-error).
3. **`T6Performative` (Smaldino & McElreath's Natural Selection of Bad Science):** When universities and funding
   agencies evaluate scientists using quantitative bibliometric proxies ($h$-index, citation impact, publication counts),
laboratories that perform underpowered studies with low severity (`epistemic.rs::EpistemicMarginState::from_inquiry_framing`)
publish more papers per dollar. Paul Smaldino and Richard McElreath (2016) demonstrated that such practices replicate across
academic generations, driving an evolutionary race to the bottom where bad methodology out-competes rigorous science
(`governance.rs::GovernanceState::from_ex_ante_gate`, `synthesis.rs::ReflexiveBrakeAudit`).

## 3. Why `(2, 0, 1)` Suffers Vital Blindness and the Autopsy Dilemma

Layer 19 possesses the triadic occupancy signature `(2, 0, 1)` (`multilayer.rs::TriadicExposureRegime`,
`TriadicExposureRegime::VitalBlindSpot`). It formalizes rigorous replication testing (`T2Severity`), archival publication bias
estimation (`T3Archive`), and institutional metric gaming (`T6Performative`), but contains zero vital margin sites (`k_vital =
0`).

A metascientific research apparatus that operates without physical buffer reserves and survival time horizons suffers **Collapse
Proposition 2 (`synthesis.rs::CollapseProposition`, `CollapseProposition::Proposition2ImpotentLucidity`)**:
- A meta-analysis or replication team correctly identifies that a hyped clinical treatment, educational software tool,
   or algorithmic predictive policing system is a false-positive artifact ($u > 0, \mathrm{PPV} < 0.50$).
- However, funding, organizing, and executing a multi-site prospective replication consortium requires extensive
   bureaucratic deliberation:

$$\mathrm{TTR}_{\mathrm{replication}} \sim 5\text{--}10\text{ years.}$$

- Meanwhile, commercial tech companies and healthcare providers deploy the flawed tool immediately at population scale
   ($\mathrm{TTS}_{\mathrm{survival}} \ll \mathrm{TTR}_{\mathrm{replication}}$).
- By the time the definitive negative replication paper appears, thousands of patients have suffered medical harm or
   citizens have experienced unjust algorithmic incarceration. Epistemic clarity arrives only as an autopsy.

Metascience achieves preventative power only when coupled to preregistered registered reports (`governance.rs::TemporalVault`),
physical stop-work buffers drawn from its vital-typed ancestors (`L08`, `L16`), and institutional liability rules (`L18`, `T7Liability`).

## Reproduce

```bash
cargo run --release --bin stochology -- --chapter 2
cargo run --release --bin stochology -- --chapter 8
```

## Bridge

Layer 19 (`multilayer.rs::KnowledgeOrder`, `KnowledgeOrder::L19MetascienceScientometricsReplication`, `{T2Severity, T3Archive,
T6Performative}`) propagates its local and ancestor-cone margins through `multilayer.rs::MultilayerHomomorphism::evaluate` into
[Chapter 70 — Layer 20: Acoustic, Aesthetic, Tragic & Pedagogical Formative Order](70-layer-acoustic-aesthetic-tragic-pedagogical-order.md) (`KnowledgeOrder::L20AcousticAestheticTragicPedagogical`, Stratum IV, `{T1Framing, T4Formalization, T6Performative}`).
