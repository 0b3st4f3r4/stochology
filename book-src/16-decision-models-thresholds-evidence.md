# 16. From Probability to Action

> **Three tempos.** **Detachment:** set aside the model as an end in itself. **Abstraction:** a probability is worth something only when it can change an action, and the signal must cross the narrow channel of human attention (Decision Models, $L\_2 \to L\_3$). **Return:** cost-loss thresholds, EVPI, decibans, alert fatigue, and anytime-valid evidence. **Negative proof:** none in this chapter; the tools here feed the proofs of later chapters.

## The Half-Million-Dollar Model That Could Never Change a Decision

A regional hospital system hires a data-science team for USD 500,000 to build a machine-learning model predicting whether patients admitted with severe fever have a bacterial infection.

Suppose treating a patient with broad-spectrum antibiotics costs $C = USD 100$, while failing to treat a true bacterial infection costs $L = USD 50\text{,}000$ in intensive-care complications (with treatment efficacy $\eta = 1.0$). By the Cost-Loss Rule of Chapter 9, a physician should administer antibiotics whenever the probability of bacterial infection exceeds:

$$p^{\ast} = \frac{C}{\eta L} = \frac{100}{50\text{,}000} = 0.002 \quad (0.2\%)$$

Assume (illustratively; these are not clinical estimates) that any patient arriving at the hospital with severe fever already has a baseline probability of bacterial infection of at least $15\%$. Even if the new USD 500,000 machine-learning test comes back **negative**, the patient's posterior probability drops only from $15\%$ to $3\%$, which remains **fifteen times higher than the $0.2\%$ threshold $p^{\ast}$.**

In other words, whether the expensive new model says "positive" or "negative," the doctor must administer antibiotics either way. **Under this worked example's assumed costs and probabilities, the Expected Value of Sample Information is zero, so the USD 500,000 project could have been stopped before a single line of code was written.**

The risk is that nobody checks whether a model can alter an action, whether the true cluster sample size $n\_{\mathrm{eff}}(p)$ is large enough, or whether the inference software can even recover known synthetic parameters until *after* months of computing have been spent.

To reverse this waste, `crates/stochology-siga` enforces two sequential operational instruments:
1. **The 4-Filter Eliminatory Funnel (`funnel/`):** A strict short-circuiting gate that kills economically, causally, or computationally invalid projects upfront.
2. **The 8-Step SIGA Operational Roadmap (`roadmap/`):** A disciplined execution cycle (**S**urvey, **I**nterrogate, **G**enerate, **A**ct) backed by anytime-valid *e-processes* and natural-frequency decision trees.

---

## 16.1 Filter 1 — Economic Viability and Question Resolution ($\Phi\_5$)

Filter 1 asks three elementary questions before touching data:
1. Is the critical cost-loss threshold non-degenerate: $p^{\ast} = \frac{C}{\eta L} \in (0, 1)$?
2. Does the **Expected Value of Perfect Information ($\mathrm{EVPI}$)** strictly exceed the total engineering, data-acquisition, and operational cost of the study ($\mathrm{EVPI} > C\_{\text{total}} \ge 0$)?
3. Does the temporal/spatial resolution of the dataset satisfy Invariant $\Phi\_5$ ($R\_{\text{data}} \ge R\_{\text{decision}}$)?

---

## 16.2 Anytime-Valid Sequential Evidence (*e-processes*) and Ville's Inequality

### Foundations: Why Classical $p$-Values Break Under Optional Stopping

In a classical fixed-horizon Neyman-Pearson test, you must fix the sample size $n$ in advance and look at the data **only once** at the end. Suppose a researcher or an automated monitoring dashboard computes a classical $p$-value every day as new data arrive and stops the moment $p\_t < 0.05$.

By the Law of the Iterated Logarithm, even under the null hypothesis $H\_0$ (where the true effect is zero), a standardized random walk crosses the fixed $1.96$ threshold infinitely often as $t \to \infty$. Peeking continuously at a classical $p$-value inflates the Type I false-positive rate all the way to **$100\%$**.

### How *e-values*, Test Martingales, and Ville's Inequality Solve Optional Stopping

Modern sequential statistics (**Glenn Shafer, Peter Grünwald, Aaditya Ramdas, and Vladimir Vovk**) replaces fragile $p$-values with **evidence values (*e-values*)** and **sequential *e-processes*** (`EvidenceProcess` in `crates/stochology-siga/src/roadmap/mod.rs`).

An **e-variable** is any non-negative random variable $L\_i \ge 0$ whose expected value under the null hypothesis $H\_0$ is at most $1$:

$$\mathbb{E}\_{H\_0}[L\_i \mid \mathcal{F}\_{i-1}] \le 1$$

For example, the likelihood ratio $L\_i = \frac{p\_1(Y\_i)}{p\_0(Y\_i)}$ between the alternative model $p\_1$ and the null model $p\_0$ automatically satisfies $\mathbb{E}\_{p\_0}\left[\frac{p\_1(Y\_i)}{p\_0(Y\_i)}\right] = \sum\_y p\_0(y)\frac{p\_1(y)}{p\_0(y)} = \sum\_y p\_1(y) = 1$.

Think of $L\_i$ as a fair betting multiplier against the null hypothesis: you start with USD 1.00 of initial capital ($E\_0 = 1$), and at each step $i = 1, \dots, t$ your capital multiplies by $L\_i$:

$$E\_t \triangleq \prod\_{i=1}^t L\_i$$

Because $E\_t$ is a non-negative test martingale (or supermartingale) under $H\_0$, **Jean Ville's (1939) Maximal Inequality** guarantees that the probability of your capital *ever* reaching $1/\alpha$ at *any* stopping time $t$ (regardless of how often you peek at the dashboard) is at most $\alpha$:

$$P\_{H\_0}\left(\sup\_{t \ge 1} E\_t \ge \frac{1}{\alpha}\right) \le \alpha$$

If an analyst tried $K \ge 1$ candidate specifications (Prohibition 4), dividing the cumulative evidence by $K$ preserves anytime validity across all $K$ attempts:

$$\frac{E\_t}{K} \ge \frac{1}{\alpha}$$

At $\alpha = 0.05$, reaching $E\_t / K \ge 20$ means the data have multiplied initial skeptical capital at least twenty-fold after paying the full multiplicity tax $K$.

### Non-Parametric GRAPA / Kelly Betting Martingales (`EvidenceProcess::from_grapa_betting`)

To construct an anytime-valid *e-process* from bounded zero-mean under-$H\_0$ payoff differences $X\_1, \dots, X\_t$ without assuming a parametric likelihood ratio $p\_1/p\_0$, we follow **Peter Grünwald, Rianne de Heide, and Wouter Koolen (2024, *Safe Testing*)** and **Ian Waudby-Smith and Aaditya Ramdas (2024)**, where `EvidenceProcess::from_grapa_betting` constructs a predictable **Growth-Rate Adaptive (`GRAPA` / Kelly) betting martingale**:

$$E\_t = \prod\_{i=1}^t \bigl(1 + \lambda\_i X\_i\bigr), \qquad \lambda\_i = \mathrm{clamp}\left(\frac{\hat{\mu}\_{i-1}}{\hat{\sigma}\_{i-1}^2 + \hat{\mu}\_{i-1}^2},\ -0.5,\ 0.5\right)$$

Because the predictable stake $\lambda\_i$ depends strictly on past observations $\mathcal{F}\_{i-1} = \sigma(X\_1, \dots, X\_{i-1})$, $\mathbb{E}\_{H\_0}[1 + \lambda\_i X\_i \mid \mathcal{F}\_{i-1}] = 1 + \lambda\_i \mathbb{E}\_{H\_0}[X\_i \mid \mathcal{F}\_{i-1}] = 1$, guaranteeing exact finite-sample Ville validity under arbitrary optional stopping while maximizing asymptotic log-capital growth under the alternative.

The guarantee requires bounded payoffs $|X\_i| \le 1$: with $|\lambda\_i| \le 0.5$ every factor satisfies $1 + \lambda\_i X\_i \ge 0.5 > 0$, so no floor is needed. An observation outside the support is a contract violation and `from_grapa_betting` returns `Err` (fail-closed): clamping $X\_i$ would bias the conditional expectation under asymmetric nulls, and silently neutralizing the bets would erase genuine evidence.

---

## 16.3 Cognitive Bottleneck ($10\text{ bit/s}$), Turing Decibans, and `AlertFatigueAudit` on Bridge $L\_2 \to L\_3$

In Step 8 of the SIGA Roadmap, alongside the integer Gigerenzer natural-frequency tree, the system verifies that the **rate and precision of operational alarms do not exceed the biological bandwidth of the human decision-maker in $L\_3$** (`AlertFatigueAudit` in `crates/stochology-siga/src/roadmap/mod.rs`).

1. **The $10\text{ bit/s}$ Conscious Bottleneck (Zheng & Meister, Caltech 2024/2025, `SensoryDpiOrganoidHybridAudit`):** Although the six million cones of the human retina capture **$1.6 \times 10^9\text{ bit/s}$** while the brain consumes only **$20\text{ W}$** of metabolic power, the biological sensory funnel compresses that stream by a factor of **$1.6 \times 10^8 : 1$** ($1.6 \times 10^9\text{ bit/s} \to 1.0 \times 10^7\text{ bit/s}$ at the retinal ganglion cells $\to 10\text{ bit/s}$) down to the invariant bottleneck of conscious human decision-making: **only $10\text{ bits per second}$** (`conscious_bottleneck_bps = 10.0`).
2. **Turing-Good Additive Deciban Weight of Evidence:** Given shift prevalence $\pi$ (prior odds $O\_{\text{prior}} = \frac{\pi}{1 - \pi}$), sensitivity $\mathrm{TPR}$, and false-positive rate $\mathrm{FPR}$, each independent diagnostic feature $k$ contributes an additive evidence weight in **decibans ($\mathrm{dB}$)**:
   $$W\_k = 10\log\_{10}\left(\frac{\mathrm{TPR}\_k}{\mathrm{FPR}\_k}\right), \qquad W\_{\text{total}} = \sum\_k W\_k, \qquad O\_{\text{post}} = O\_{\text{prior}} \times 10^{W\_{\text{total}} / 10}, \qquad \mathrm{PPV} = \frac{O\_{\text{post}}}{1 + O\_{\text{post}}}$$
3. **Why $88\%$ of the Epic Sepsis Model's Alerts Were False Alarms (`AlertFatigueAudit::canonical_epic_sepsis_vs_stochological_gate`):** In the external validation of **Wong et al. (*JAMA Internal Medicine* 2021, literature)** across $27\text{,}697$ patients ($38\text{,}455$ hospitalizations, alarming on $18\%$ of all admissions), the proprietary Epic Sepsis Model ($\mathrm{AUC} = 0.63, \mathrm{TPR} = 0.33, \mathrm{FPR} = 0.171$ at prevalence $\pi = 0.066$) achieved a Turing weight of evidence of only **$W = +2.86\text{ dB}$** and a Positive Predictive Value of **$\mathrm{PPV} = 0.120$** — meaning **$88\%$ of its alerts were false alarms ($7.33$ false alarms for every true case)**. This is one model in one study; it does not license a claim about clinical or operational AI alerts in general. Under an assumed 12-hour shift burst of $18\text{ alarms/min} \times 42\text{ bits/alarm}$ (illustrative, not reported by Wong et al.), it imposes **$12.6\text{ bit/s} > 10.0\text{ bit/s}$** of cognitive triage load (`approved_bridge_l2_l3 = false`). By contrast, a CORP-calibrated Stochological multi-signal triage gate at an **assumed design operating point** ($\mathrm{TPR} = 0.82, \mathrm{FPR} = 0.045 \implies \mathrm{LR}^+ = 18.22$, never measured on the Wong et al. cohort) would achieve **$W = +12.61\text{ dB} \ge 8.0\text{ dB}$**, raise Positive Predictive Value to **$\mathrm{PPV} = 0.563 \ge p^{\ast} = 0.200$** ($0.78$ false alarms per true case), and compress burst triage load ($4\text{ alarms/min} \times 48\text{ bits/alarm}$) to **$3.2\text{ bit/s} \le 10.0\text{ bit/s}$** (`approved_bridge_l2_l3 = true`). The contrast shows what the audit rejects and what it would accept; it is not evidence that any deployed triage reaches $\mathrm{PPV} = 0.563$.

---

## 16.4 Multi-Criteria Prioritization (`SaatyAhpRankReversalAudit`) and Correlated Multimodal Sensory Fusion (`MultimodalCorrelatedFusionAudit`)

- **Multi-Criteria Prioritization (`SaatyAhpRankReversalAudit` in `decision/` & `MultiCriteriaAhpRegulatoryPkmAudit` in `strategic/`):** Thomas Saaty's (1980) Analytic Hierarchy Process is exercised on an exactly consistent $3 \times 3$ reciprocal matrix ($w = (0.6, 0.3, 0.1)$, so $\lambda\_{\max} = 3$ and $\mathrm{CI} = 0$; the code computes only $\mathrm{CI}$, no random index or $\mathrm{CR}$), and Belton & Gear's (1983) example shows **rank reversal** under column normalization when an exact copy of alternative $B$ is added, while ideal-mode (fixed-anchor) scoring keeps the order. The multiplicative `ICE` (`Impact * Confidence * Ease`) product of means overstates the mean product by $-\mathrm{Cov}(I, E) = +10.0$ on the canonical negatively correlated vector (`ICE_bias=+10.0`, deterministic identity). The realized portfolio contrast (`AHP_val=18.26 vs 11.80`, simulated under a pre-registered seed) is **constructed**: the code assigns the rank-reversed choice to the naive arm by design (`strategic/`), so it illustrates the cost of rank reversal rather than measuring it. The separate cost-loss cascade of Axis 3 (local model, escalation of cases near $p^{\ast} = 0.20$ to a costlier verifier) yields an expected cost+loss per decision of $17.34$ vs. $24.87$ for an uncalibrated local model (`FinOps_cost=17.34 vs 24.87`, simulated under a pre-registered seed; a $30.3\%$ reduction derived from those two figures).
- **Correlated Multimodal Sensory Fusion (`MultimodalCorrelatedFusionAudit` in `crates/stochology-siga/src/roadmap/`):** When an autonomous system fuses $M = 5$ sensory channels (*text, audio, image, video, telemetry*) that share a latent cross-modal correlation $\rho \in [0, 1)$, naive independent summation of Turing deciban weights ($W\_{\text{naive}} = \sum\_{m=1}^M w\_m = +53.0\text{ dB}$ in the canonical benchmark) violates $\Phi\_2$ across modalities and allows a single corrupted $+35.0\text{ dB}$ sensor glitch to hijack the posterior; discounting by the multimodal design effect $\mathrm{DEFF}\_{\text{modal}} = 1 + (M - 1)\rho = 2.80$ and clipping individual channels at $\pm w\_{\max} = 6.0\text{ dB}$ restores bounded, calibrated evidence ($W\_{\text{shielded}} = +8.57\text{ dB}$).

---

## Interactive Laboratory (`/lab`)

Explore the sequential `aGRAPA` betting martingale $E\_t = \prod (1 + \lambda\_i X\_i)$, Ville's anytime-valid barrier $1/\alpha$, and the multiplicity penalty $E\_t / K$ executed during Step 7 of the SIGA Roadmap:

<div class="stoch-lab"
     data-lab="ville-savi-conformal"
     data-chapter="16-decision-models-thresholds-evidence.html"
     data-title="SIGA Step 7: Sequential e-Process &amp; Multiplicity Gate"
     data-subtitle="Test how optional stopping and K ≥ 1 attempted specifications interact with Ville's 1/α threshold">
</div>

---

## Computational Audit in Rust

Run Laboratory 5 (`crates/stochology-siga/src/roadmap/`) to inspect the Filter 3 SHELF conjugate Beta fit, the Step 6 `aGRAPA` betting martingale $E\_{35}$, and the Bridge $L\_2 \to L\_3$ `AlertFatigueAudit` (the per-card SIGA execution lines of Lab 5 are audited in [Chapter 26](26-funnel-siga.md)):

~~~bash
cargo run --release --bin labs -- --siga
~~~

~~~text
=== LAB 5: 4-FILTER ELIMINATORY FUNNEL AND 8-STEP SIGA ROADMAP ===
  [Filter 3 SHELF Beta Fit & Step 6 GRAPA] Beta(alpha=1.05, beta=15.03, CDF_RMSE=0.0111) | GRAPA E_35=992.26 (rejects_H0=true) | [Bridge L2->L3 AlertFatigueAudit (10 bit/s & Turing Decibans)] EpicSepsis(PPV=12.0%, FP/TP=7.3, approved=false) vs StochologicalTarget(assumed TPR/FPR, PPV=56.3%, W=+12.61 dB, load=3.2 bit/s <= 10, approved=true)
~~~

## Exercises

1. **(Calculation — Filter 1 Short-Circuiting)** A logistics company proposes spending $C\_{\text{total}} = USD 80\text{,}000$ to build a model predicting late port deliveries. Each late delivery causes loss $L = USD 10\text{,}000$, expedited air-freight protection costs $C = USD 2\text{,}000$ with efficacy $\eta = 1.0$, and across the year $100$ shipments are made with base rate $p\_0 = 0.05$ ($5$ late shipments per year). Even if the model were an infallible oracle, what is the maximum annual savings ($\mathrm{EVPI} \times 100$) it could generate? Will Filter 1 approve or kill the USD 80,000 project?
2. **(Calculation — Sequential *e-process* and Ville's Inequality)** Over $5$ sequential out-of-sample test weeks, a model achieves weekly likelihood ratios $L\_1 = 2.5, L\_2 = 1.8, L\_3 = 2.0, L\_4 = 0.8, L\_5 = 5.0$ against the null model, after $K = 2$ attempted specifications.
   - *(a)* Compute $E\_5 = \prod\_{i=1}^5 L\_i$ and the multiplicity-adjusted evidence $E\_5 / K$.
   - *(b)* By Ville's inequality, what is the upper bound on the probability that $E\_t / K$ could have reached this level under the null hypothesis?

<details>
<summary><strong>Analytical &amp; Computational Solutions (Click to Expand)</strong></summary>

1. **Filter 1 Short-Circuiting:**
   - Threshold: $p^{\ast} = \frac{2\text{,}000}{1.0 \times 10\text{,}000} = 0.20$. Since $p\_0 = 0.05 \le p^{\ast}$, default action without the model is `do_not_protect` (expected annual loss $= 5 \times 10\text{,}000 = USD 50\text{,}000$).
   - An infallible oracle protects on the $5$ late shipments ($5 \times 2\text{,}000 = USD 10\text{,}000$), saving at most $\mathrm{EVPI}\_{\text{annual}} = 50\text{,}000 - 10\text{,}000 = USD 40\text{,}000$.
   - Since $\mathrm{EVPI}\_{\text{annual}} = USD 40\text{,}000 \le C\_{\text{total}} = USD 80\text{,}000$, **Filter 1 immediately short-circuits and kills the project.**
2. **Sequential *e-process* Calculation:**
   - *(a)* $E\_5 = 2.5 \times 1.8 \times 2.0 \times 0.8 \times 5.0 = 36.0$. Multiplicity-adjusted evidence: $E\_5 / K = 36.0 / 2 = 18.0$.
   - *(b)* By Ville's inequality, $P\_{H\_0}(\sup\_t E\_t / K \ge 18.0) \le \frac{1}{18.0} \approx 0.0556$ ($5.56\%$).

</details>

## Summary

- **Filter 1 ($\mathrm{EVPI} > C\_{\text{total}}$ and $\Phi\_5$)** short-circuits economically non-viable or coarse-resolution projects before model fitting.
- **Anytime-Valid *e-processes* ($E\_t = \prod L\_i$ and `aGRAPA` betting martingales)** remain valid under arbitrary optional stopping via **Ville's Inequality** ($P\_{H\_0}(\sup\_t E\_t / K \ge 1/\alpha) \le \alpha$).
- **Cognitive Bottleneck (`AlertFatigueAudit`) and Correlated Multimodal Fusion (`MultimodalCorrelatedFusionAudit`)** couple Zheng & Meister's $10\text{ bit/s}$ conscious ceiling, Turing-Good deciban weights, Ideal-mode AHP prioritization (`SaatyAhpRankReversalAudit`), and cross-modal design-effect discounting ($\mathrm{DEFF}\_{\text{modal}} = 1 + (M - 1)\rho$) on Bridge $L\_2 \to L\_3$.
