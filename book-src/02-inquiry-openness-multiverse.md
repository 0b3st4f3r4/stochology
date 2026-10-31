# Chapter 2 — Inquiry, Framing, and Antidogmatic Openness

The first breakdown of the `NaiveOptimizer` occurs before a single parameter is estimated: it treats the analyst's framing as neutral, permits dogmatic extreme probabilities ($p = 0$ or $p = 1$), mistakes an underpowered study for evidence of safety, and hides the analytical choices ("garden of forking paths") that preceded the reported specification.

In the architecture of Stochology, **Mediating Filter I** governs the transition from practical doubt to structured conjecture. Its first constitutive tension (**Tension 1**, implemented in [`crates/stochology/src/inquiry.rs`](../crates/stochology/src/inquiry.rs)) confronts **situated cognitive interest and framing power** against **antidogmatic openness and full attempt memory**.

---

## 2.1 Cromwell's Rule and Peirce's First Rule of Reason

In 1650, Oliver Cromwell pleaded with the General Assembly of the Church of Scotland: *"I beseech you, in the bowels of Christ, think it possible you may be mistaken."* In Bayesian probability, Dennis Lindley (1985) formalized this plea as **Cromwell's Rule**, while Charles Sanders Peirce (1899) formulated its philosophical counterpart as the **First Rule of Reason**: *Do not block the way of inquiry.*

If a prior or forecast assigns $p = 0$ or $p = 1$ to an empirically possible event, Bayes's theorem yields:

$$\mathbb{P}(H \mid E) = \frac{\mathbb{P}(E \mid H)\\,\mathbb{P}(H)}{\mathbb{P}(E \mid H)\\,\mathbb{P}(H) + \mathbb{P}(E \mid \neg H)\\,\bigl(1 - \mathbb{P}(H)\bigr)} \in \\{0, 1\\}$$

No finite likelihood ratio—no matter how overwhelming the field evidence—can ever move a dogmatic $0$ or $1$. In [`inquiry.rs`](../crates/stochology/src/inquiry.rs), [`verify_cromwell_probability`](../crates/stochology/src/inquiry.rs#L17-L32) and [`cromwell_clamp`](../crates/stochology/src/inquiry.rs#L35-L38) enforce an explicit interior floor $p \in [\varepsilon\_{\mathrm{floor}}, 1 - \varepsilon\_{\mathrm{floor}}]$:

```rust
pub fn cromwell_clamp(p: f64, floor: f64) -> f64 {
    let eps = floor.clamp(1e-12, 0.49);
    p.clamp(eps, 1.0 - eps)
}
```

---

## 2.2 Strong Objectivity, Inductive Risk, and Cost–Loss Framing

Following Richard Rudner (1953), Heather Douglas (2000), and Sandra Harding (1991), setting an action or acceptance threshold under uncertainty is never value-free: it weighs the cost of protective action $C > 0$ against the mitigated loss $\eta L > 0$ of an unbuffered catastrophe. Under the economic cost–loss rule ([`CostLossFraming`](../crates/stochology/src/inquiry.rs#L42-L97)):

$$p^\* = \frac{C}{\eta L} \in (0, 1), \qquad \mathrm{EVPI}(\pi\_0) = \min\bigl(\pi\_0 \eta L,\\, C\bigr) - \pi\_0 C$$

Why do institutions and affected communities so often disagree even when they agree on the probability $\pi\_0$ of a hazard? Because the **institutional budget** measures $L\_{\mathrm{inst}}$ as bureaucratic liability ($L\_{\mathrm{inst}} = 200$), yielding a high action threshold $p^\*\_{\mathrm{inst}} = \frac{100}{200} = 0.50$, whereas the **vulnerable community** bears the irreversible human and livelihood loss ($L\_{\mathrm{affected}} = 2000$), yielding $p^\*\_{\mathrm{affected}} = \frac{100}{2000} = 0.05$.

Harding's *strong objectivity* requires computing [`CostLossFraming`](../crates/stochology/src/inquiry.rs#L42-L97) from both standpoints and reporting the **framing gap** $|p^\*\_{\mathrm{inst}} - p^\*\_{\mathrm{affected}}| = 0.45$, rather than disguising institutional cost-saving as scientific conservatism.

---

## 2.3 Agnotology vs. Severe Power and Kimball's Type III Error

Robert Proctor (2008) coined **agnotology** for the strategic production of ignorance. One of its most common statistical instruments is the **deliberately underpowered study**: running a small sample $n$ with low statistical power $1 - \beta(\delta\_{\min})$ against a substantively dangerous harm $\delta\_{\min} > 0$, and then proclaiming "no statistically significant effect ($p > \alpha$)" as proof that the system is safe.

In [`SeverePowerAnalysis`](../crates/stochology/src/inquiry.rs#L101-L128), for a minimum substantively relevant harm $\delta\_{\min} > 0$, standard deviation $\sigma > 0$, sample size $n$, and one-sided level $\alpha$, the statistical power to detect $\delta\_{\min}$ is:

$$\mathrm{Power}(\delta\_{\min}) = 1 - \Phi\\!\left(z\_{1-\alpha} - \frac{\delta\_{\min}\sqrt{n}}{\sigma}\right)$$

By Deborah Mayo's (2018) **Severity Principle**, a non-rejection of the null hypothesis warrants the claim "true harm is less than $\delta\_{\min}$" only if the test had high power ($\ge 0.80$) to detect $\delta\_{\min}$ had it been present.

At the same time, A. W. Kimball (1957) and Ian Mitroff & Francisco Sagasti (1973) identified **Type III Error** ([`TypeThreeFramingAudit`](../crates/stochology/src/inquiry.rs#L172-L202)): solving the wrong problem precisely—achieving a positive narrow proxy gain $+G\_{\mathrm{narrow}}$ while inflicting a larger unmeasured systemic damage $-D\_{\mathrm{unmeasured}} < -G\_{\mathrm{narrow}}$.

---

## 2.4 Multiplicity ($e\text{-BH}$) and the Specification Curve (Multiverse)

When an analyst explores $K \ge 1$ plausible data-cleaning rules, covariate sets, or functional forms (Gelman & Loken's *garden of forking paths*), reporting only the single specification that crossed a significance threshold is a classic cherry-picking distortion. Stochology enforces two complementary defenses in [`inquiry.rs`](../crates/stochology/src/inquiry.rs):

1. **The $e\text{-BH}$ Procedure ([`e_bh_discoveries`](../crates/stochology/src/inquiry.rs#L207-L234), Wang & Ramdas, 2022):** Given $K$ e-values $E\_{(1)} \ge E\_{(2)} \ge \dots \ge E\_{(K)}$ (where $\mathbb{E}\_{H\_0}[E\_k] \le 1$), the $e\text{-BH}$ rule controls the False Discovery Rate at level $\alpha$ under **arbitrary dependence** across the $K$ specifications by finding the largest $k^\*$ such that:

   $$\frac{k^\* \\, E\_{(k^\*)}}{K} \ge \frac{1}{\alpha}$$

2. **Simonsohn, Simmons & Nelson's (2020) Specification Curve / Steegen et al.'s (2016) Multiverse Analysis ([`SpecificationCurve`](../crates/stochology/src/inquiry.rs#L238-L290)):** Instead of hiding the $K - 1$ alternative models, `SpecificationCurve::evaluate` records the full vector of $K$ effect estimates and e-values, reporting the median effect, the min–max envelope across all $K$ specifications, the **consistency ratio** (fraction of specifications sharing the dominant sign and exceeding $1/\alpha$), and the average e-value $\bar{E} = \frac{1}{K}\sum\_{k=1}^K E\_k$ (which remains a valid e-value under arbitrary dependence).

---

## 2.5 Chapter 2 Laboratory

Running Chapter 2 (`cargo run --release --bin stochology -- --chapter 2`, Simulated/Analytical) executes all four instruments of Filter I · Tension 1:

```text
=== Chapter 2: Inquiry, Framing & Antidogmatic Openness (Filter I · Tension 1) ===
  Cromwell floor=0.0001 | Framing p*_inst=0.50 vs p*_affected=0.05 (gap=0.45, EVPI_affected=80.0)
  Agnotology vs Severe Power (delta=0.30): n=18 power=0.283 vs n=140 power=0.956 | Type III net=-16.0
  Multiverse (K=6): robust consistency=100% (avg_E=35.0) vs cherry-picked consistency=16.7% (e-BH discoveries=0)
```

- **Framing Standpoint Gap (Assumed/Analytical):** At prior hazard $\pi\_0 = 0.20$ and protective action cost $C = 100$, the institutional threshold $p^\*\_{\mathrm{inst}} = 0.50$ prescribes **inaction** ($\pi\_0 < 0.50$), whereas the affected community's threshold $p^\*\_{\mathrm{affected}} = 0.05$ prescribes **immediate protection** ($\pi\_0 > 0.05$, with $\mathrm{EVPI}\_{\mathrm{affected}} = 80.0$).
- **Agnotology vs. Severity (Analytical):** For minimum relevant harm $\delta\_{\min} = 0.30\sigma$, a token study of $n = 18$ has only $28.3\\%$ power (`power = 0.283 < 0.80`), exposing any claim of "no effect" as agnotological; increasing sample size to $n = 140$ achieves $95.6\\%$ power (`power = 0.956`).
- **Multiverse & $e\text{-BH}$ (Analytical):** Across $K = 6$ specifications, a genuine structural finding achieves $100\\%$ sign-and-evidence consistency (`avg_E = 35.0`), whereas a single cherry-picked specification ($E\_1 = 21.5$, while $E\_2, \dots, E\_6 < 1$) has only $16.7\\%$ consistency and yields **$0$ discoveries** under $e\text{-BH}$ at $\alpha = 0.05$ because $\frac{1 \times 21.5}{6} = 3.58 < \frac{1}{0.05} = 20$.

---

## Exercises

1. **Why $e\text{-BH}$ Needs No Independence Assumption:** Verify from Markov's inequality that if $E\_1, \dots, E\_K$ are non-negative random variables with $\mathbb{E}\_{H\_0}[E\_k] \le 1$, their arithmetic mean $\bar{E} = \frac{1}{K}\sum\_{k=1}^K E\_k$ also satisfies $\mathbb{E}\_{H\_0}[\bar{E}] \le 1$ regardless of correlation among the $K$ specifications.
2. **Standpoint Elicitation:** In [`CostLossFraming::evaluate`](../crates/stochology/src/inquiry.rs#L65-L89), compute the minimum efficacy $\eta \in (0, 1]$ required for the institutional threshold $p^\*\_{\mathrm{inst}}$ to agree with the community's action decision at $\pi\_0 = 0.20$.

