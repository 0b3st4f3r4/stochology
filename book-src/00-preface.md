# Preface — The Science and Ethics of Aim (*Stókhos*)

<p align="center">
  <img src="logo.svg" alt="Stochology Emblem" width="160" />
</p>

**Stochology** is the science and ethics of aim (*stókhos*, $\sigma\tau\acute{o}\chi o\varsigma$) for judgment and decision-making under uncertainty when action must precede the outcome.

In classical Greek thought, a sharp boundary separated the exact sciences of invariant measure (*metrētikē*) from the **stochastic arts** (*stokhastikaì tékhnai*)—medicine, navigation, agriculture, strategy, and statecraft—which operate in open, shifting, and irreversible worlds. In the *Philebus* (55e–56c), Plato contrasts the carpenter's rule with the navigator's conjecture (*stokhazesthai*); in the Hippocratic treatise *On Ancient Medicine* (§9), the physician has no external mechanical scale beyond the living body's response; in the *Nicomachean Ethics* (II.6, 1106b28; VI.5), Aristotle defines practical wisdom (*phrónēsis*) as the capacity to aim (*stokhastikos*) at the intermediate good amid particular contingencies; and in the Stoic doctrine of Antipater of Tarsus (Cicero, *De Finibus* III.22), the archer's ethical duty (**télos**, doing everything within one's power to aim well) is distinguished from the external result (**skopós**, hitting the target, which Remains exposed to crosswinds after the arrow leaves the bow).

Two millennia later, in Part IV of *Ars Conjectandi* (1713), Jakob Bernoulli coined *ars conjectandi sive stochastice*—the art of measuring the probabilities of things as accurately as possible so that in our judgments and actions we may always choose what is healthier, safer, or better advised. Yet over the twentieth century, the adjective *stochastic* was reduced to a technical synonym for a random process indexed by time, severed from the philosophical, institutional, and ethical problem of how human agents and institutions actually observe, conjecture, and intervene.

This book—and the zero-dependency Rust codebase that accompanies it—reclaims **Stochology** as an integrated scientific and ethical discipline.

![Three Historical Lineages of Stochology](figures/fig-00-three-traditions.svg)

---

## 1. Three Critical Lineages and the Nuclear Triad

Why do sophisticated statistical models, machine-learning pipelines, and bureaucratic metrics so often produce catastrophic failures when deployed in the real world? Across the history of science and social thought, three traditions diagnosed three distinct dimensions of this failure, yet rarely spoke to one another:

1. **The Epistemological and Statistical Lineage** (Cromwell, Peirce, Keynes, Neyman, Wald, Popper, Lakatos, Mayo, Manski, Meng, Rubin, Pearl) showed that data never speak for themselves: samples suffer from selection defects and cluster correlation, multiple hypotheses can fit the same finite archive, and point estimates routinely masquerade as certainties.
2. **The Physical, Biological, and Cybernetic Lineage** (Canguilhem, Wiener, Ashby, Shannon, Itô, Kelly, Kingman, Scarf, Georgescu-Roegen, Samuelson, Holling, Beer, Perdomo) showed that action takes place in irreversible entropic time: variance drains multiplicative growth, queues explode nonlinearly near full utilization, performative models alter the very world they predict, and living or institutional systems without physical slack collapse at absorbing ruin barriers.
3. **The Critical, Sociological, and Ethical Lineage** (Adorno, Horkheimer, Marcuse, Desrosières, Porter, Hacking, Harding, Williams, Nagel, Sosa, Fricker, Holmström, List, Pettit, Chouldechova) showed that archives are institutional artifacts: categories exclude the vulnerable, high-powered proxy targets corrupt unmeasured care, moral luck distorts retrospective blame, and frontline warnings are silenced by testimonial injustice.

Stochology unites these three lineages into a single **Nuclear Triad** of non-substitutable safeguards:

$$\mathcal{T}_{\mathrm{Stochology}} = \bigl(\text{Epistemic Margin},\; \text{Vital Margin},\; \text{Institutional Governance}\bigr)$$

1. **Epistemic Margin:** Never collapse underdetermination, selection bias, or cluster correlation into a dogmatic point estimate. Report partial-identification intervals $[L_t, U_t]$, effective sample size $n_{\mathrm{eff}}$, and Keynesian evidential weight $V$.
2. **Vital Margin:** Never confuse the evaluation at the mean $f(\mathbb{E}[X])$ with the expected evaluation $\mathbb{E}[f(X)]$, nor ensemble averages with single-trajectory survival over irreversible time. Maintain physical slack ($\rho < 1$, $\mathrm{TTS} \ge \mathrm{TTR}$) and shrink leverage against estimation error.
3. **Institutional Governance:** Never evaluate an agent by retrospective outcome luck (*skopós*) alone, nor govern a complex social system through high-powered single proxies or silenced frontline testimony. Separate ex-ante competence (*télos*) from ex-post fortune, protect the algedonic emergency channel ($S3^*$), and align liability with structural decision power.

---

## 2. The Three Collapse Propositions

The three axes of the Nuclear Triad are complements in the strict sense of Leontief:

$$W_{\mathrm{triad}} = S_{\mathrm{epi}} \cdot S_{\mathrm{vital}} \cdot S_{\mathrm{gov}}, \qquad M_{\mathrm{Leontief}} = \min\bigl(S_{\mathrm{epi}},\, S_{\mathrm{vital}},\, S_{\mathrm{gov}}\bigr)$$

Removing any single axis triggers a specific structural pathology (**Collapse Propositions 1–3**, implemented in [`CollapseProposition`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/synthesis.rs#L14-L28) and [`TriadicIntegrity`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/synthesis.rs#L34-L79)):

| Collapse Proposition | Missing Axis | Structural Pathology |
| :--- | :--- | :--- |
| **Proposition 1** | $\neg\text{Epistemic Margin}$ ($S_{\mathrm{epi}} \to 0$) | **Blind Undersizing:** Even well-intentioned institutions with physical buffers size them against a false point certainty, underestimating selection defects ($\rho_{R,Y} \ne 0$) and cluster echoes until a predictable surge overwhelms the buffer. |
| **Proposition 2** | $\neg\text{Vital Margin}$ ($S_{\mathrm{vital}} \to 0$) | **Impotent Lucidity Before Ruin:** The analyst calculates the exact width of uncertainty and sees the crisis coming, yet the system operates at $96\text{--}100\%$ utilization or excess leverage with $\mathrm{TTS} < \mathrm{TTR}$ and is absorbed at the ruin barrier before any steering can take effect. |
| **Proposition 3** | $\neg\text{Institutional Governance}$ ($S_{\mathrm{gov}} \to 0$) | **Cynical Capture and Pillaging:** Epistemic intervals are weaponized as an agnotological alibi for paralysis ("more studies are needed"), while common vital reserves are privatized under limited-liability moral hazard and frontline warnings are discounted. |

---

## 3. How to Read This Book: The Sequential Loop

Inspired by the pedagogy of Reinforcement Learning (Sutton & Barto), this book avoids dry encyclopedic catalogs. Instead, a single evolving sequential environment—**"The Community and the Reserve"** ([`CommunityReserveEnv`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/engine.rs#L309-L316))—accompanies you from **Chapter 1** to **Chapter 9**, always paired with **14 real-world empirical datasets** sealed in [`data/SHA256SUMS`](file:///home/silvano-neto/Documents/stochology/data/SHA256SUMS):

- **Chapter 1 ([`engine.rs`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/engine.rs))** introduces the sequential Agent–World–Institution loop and watches the standard [`NaiveOptimizer`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/engine.rs#L130) suffer catastrophic ruin at step $t = 3$.
- **Chapters 2 and 3 ([`inquiry.rs`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/inquiry.rs), [`identification.rs`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/identification.rs))** develop **Mediating Filter I** (from problem framing and Cromwell's antidogmatic openness to Manski/Lee/Peterson partial identification and betting martingales), tested on the LaLonde National Supported Work experiment (`nsw_experimental.csv`, `cps_observational.csv`).
- **Chapters 4 and 5 ([`asymmetry.rs`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/asymmetry.rs), [`archive.rs`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/archive.rs))** develop **Mediating Filter II** (from scale invariance, Jensen's convexity gap, and Hill's heavy-tail index to Meng's Big Data Paradox, Campbell–Moulton cluster design effects, Chao capture-recapture, and bitemporal vintages), tested on `danish_fire.csv`, `nyc_flights_2013.csv`, `nyc_flights_2023.csv`, `bank_calls.csv`, and `rtdsm_routput_vintages.csv`.
- **Chapters 6 and 7 ([`performativity.rs`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/performativity.rs), [`vital.rs`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/vital.rs))** develop **Mediating Filter III** (from isotonic CORP-PAV calibration, Invariant Causal Prediction, Perdomo performative prediction, and Goodhart's Law to Itô's variance drain, Baker–McHale epistemic Kelly shrinkage, Canguilhem–Holling resilience basins, Kingman heavy-traffic queues, and Wald–Sniedovich safeguards), tested on `atlantic_storms.csv`, `djia_1885_2023.csv`, and `vic_elec.csv`.
- **Chapters 8 and 9 ([`governance.rs`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/governance.rs), [`synthesis.rs`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/synthesis.rs))** complete the **Ethical & Institutional Mediation** and the **Five Methodological Families** (Sosa's virtue grades, Baron–Hershey bitemporal evaluation, List–Pettit discursive dilemma, Fricker's testimonial injustice on Beer's algedonic channel, Holmström–Innes contracts, Chouldechova's fairness trilemma on `toronto_arrests.csv`, qualitative Process Tracing, and the Three Internal Goodhart Brakes).

You can run every laboratory in this book with a single command:

```bash
cargo run --release --bin stochology
```

Or explore the sequential loop visually in your browser via the [Interactive Laboratory (`lab/index.html`)](lab/index.html).
