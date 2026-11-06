# Chapter 43 — Inquiry, Framing, and Antidogmatic Openness

> Laboratory 2 of the `stochology` CLI: `--chapter 2`.

The first breakdown of the `NaiveOptimizer` occurs before a single parameter is estimated: it treats the analyst's framing as neutral, permits dogmatic extreme probabilities ($p = 0$ or $p = 1$), mistakes an underpowered study for evidence of safety, and hides the analytical choices ("garden of forking paths") that preceded the reported specification.

In the architecture of Stochology, **Mediating Filter I · Axis 1 ($S_{\mathrm{epi}}$: Epistemic Margin)** governs the transition from the open world to the institutional archive across **Chapters 43, 44, and 45**, unified in [`crates/stochology/src/epistemic.rs`](crates/stochology/src/epistemic.rs) around **Canonical Operator I** ([`EpistemicMarginState`](crates/stochology/src/epistemic.rs#L145-L198)). Its first constitutive tension (**Tension 1**) confronts **situated cognitive interest and framing power** against **antidogmatic openness and full attempt memory**.

![The 2x2 Stirling Diagnostic Matrix of Uncertainty](figures/fig-02-stirling-matrix.svg)

---

## 2.1 Cromwell's Rule and Peirce's First Rule of Reason

In 1650, Oliver Cromwell pleaded with the General Assembly of the Church of Scotland: *"I beseech you, in the bowels of Christ, think it possible you may be mistaken."* In Bayesian probability, Dennis Lindley (1985) formalized this plea as **Cromwell's Rule**, while Charles Sanders Peirce (1899) formulated its philosophical counterpart as the **First Rule of Reason**: *Do not block the way of inquiry.*

If a prior or forecast assigns $p = 0$ or $p = 1$ to an empirically possible event, Bayes's theorem yields:

$$\mathbb{P}(H \mid E) = \frac{\mathbb{P}(E \mid H)\,\mathbb{P}(H)}{\mathbb{P}(E \mid H)\,\mathbb{P}(H) + \mathbb{P}(E \mid \neg H)\,\bigl(1 - \mathbb{P}(H)\bigr)} \in \lbrace 0, 1\rbrace$$

No finite likelihood ratio—no matter how overwhelming the field evidence—can ever move a dogmatic $0$ or $1$. In [`kernel.rs`](crates/stochology/src/kernel.rs), [`obeys_cromwell_rule`](crates/stochology/src/kernel.rs#L590-L596) and [`cromwell_clamp`](crates/stochology/src/kernel.rs#L580-L587) enforce an explicit interior floor $p \in [\varepsilon_{\mathrm{floor}}, 1 - \varepsilon_{\mathrm{floor}}]$:

```rust
pub fn cromwell_clamp(p: f64, epsilon_floor: f64) -> f64 {
    let eps = epsilon_floor.clamp(1e-12, 0.49);
    p.clamp(eps, 1.0 - eps)
}
```

---

## 2.2 Strong Objectivity, Inductive Risk, and Cost–Loss Framing

Following Richard Rudner (1953), Heather Douglas (2000), and Sandra Harding (1991), setting an action or acceptance threshold under uncertainty is never value-free: it weighs the cost of protective action $C > 0$ against the mitigated loss $\eta L > 0$ of an unbuffered catastrophe. Consider the canonical $2\times 2$ decision payoff matrix under state of nature $\theta \in \{\text{Hazard } H, \text{Safe } \neg H\}$:

| Action | Hazard Present ($H$, prob $\pi_0$) | Safe State ($\neg H$, prob $1 - \pi_0$) | Expected Cost / Loss |
| :--- | :--- | :--- | :--- |
| **Protective Action ($A$)** | $C + (1 - \eta)L$ | $C$ | $C + \pi_0(1 - \eta)L$ |
| **Inaction / Wait ($W$)** | $L$ | $0$ | $\pi_0 L$ |

Equating the expected loss of action and inaction yields the indifference threshold $p^*$:
$$C + \pi_0(1 - \eta)L = \pi_0 L \implies C = \pi_0 \eta L \implies p^* = \frac{C}{\eta L} \in (0, 1)$$
Action is rational whenever prior hazard belief satisfies $\pi_0 > p^*$. Under the economic cost–loss rule ([`EpistemicMarginState::from_inquiry_framing`](crates/stochology/src/epistemic.rs#L247-L320)):
$$p^* = \frac{C}{\eta L} \in (0, 1), \qquad \mathrm{EVPI}(\pi_0) = \min\bigl(C,\, \pi_0 \eta L\bigr) - \pi_0 \min\bigl(C,\, \eta L\bigr) \ge 0$$

Why do institutions and affected communities so often disagree even when they agree on the probability $\pi_0 = 0.18$ of a hazard? Because the **institutional budget** measures $L_{\mathrm{inst}}$ strictly as bureaucratic liability ($L_{\mathrm{inst}} = 200$, $C = 100$, $\eta = 0.85$, `[Assumed]`), yielding a high action threshold:
$$p^*_{\mathrm{inst}} = \frac{100}{0.85 \times 200} = \frac{100}{170} \approx 0.588$$
whereas the **vulnerable community** bears the irreversible human and livelihood loss ($L_{\mathrm{affected}} = 800$, $C = 100$, $\eta = 0.85$, `[Assumed]`), yielding:
$$p^*_{\mathrm{affected}} = \frac{100}{0.85 \times 800} = \frac{100}{680} \approx 0.147$$
Because $\pi_0 = 0.18$ lies strictly between these two thresholds ($0.147 < \pi_0 = 0.18 < 0.588$, `[Assumed]`), the institution rationalizes waiting as "scientific prudence," while the community correctly views inaction as reckless abandonment.

Harding's *strong objectivity* requires computing [`EpistemicMarginState::from_inquiry_framing`](crates/stochology/src/epistemic.rs#L247-L320) from both standpoints and reporting the **framing gap** ([`EpistemicMarginState::framing_gap`](crates/stochology/src/epistemic.rs#L372-L374), $|p^*_{\mathrm{inst}} - p^*_{\mathrm{affected}}| = 0.441$, `[Assumed]`), rather than disguising institutional cost-saving as scientific conservatism.

---

## 2.3 Agnotology vs. Severe Power and Kimball's Type III Error

Robert Proctor (2008) coined **agnotology** for the strategic production of ignorance. One of its most common statistical instruments is the **deliberately underpowered study**: running a small sample $n$ with low statistical power $1 - \beta(\delta_{\min})$ against a substantively dangerous harm $\delta_{\min} > 0$, and then proclaiming "no statistically significant effect ($p > \alpha$)" as proof that the system is safe.

In [`EpistemicMarginState::from_inquiry_framing`](crates/stochology/src/epistemic.rs#L247-L320), for a minimum substantively relevant harm $\delta_{\min} > 0$, standard deviation $\sigma > 0$, effective sample size $n_{\mathrm{eff}}$, and one-sided level $\alpha$, the statistical power to detect $\delta_{\min}$ is:
$$\mathrm{Power}(\delta_{\min}) = 1 - \Phi\!\left(z_{1-\alpha} - \frac{\delta_{\min}\sqrt{n_{\mathrm{eff}}}}{\sigma}\right)$$

By Deborah Mayo's (2018) **Severity Principle**, a hypothesis or claim $H$ passes a severe test with data $\mathbf{x}_0$ only if:
1. $\mathbf{x}_0$ accords with $H$; and
2. with very high probability, the test procedure would have produced a less favorable result if $H$ were false.

Formally, for a test statistic $T(\mathbf{X})$ where larger values indicate deviation from the null towards alternative $H_1$, the severity with which data $\mathbf{x}_0$ warrants non-harm $H_0: \delta \le \delta_{\min}$ against $H_1: \delta > \delta_{\min}$ is defined as:
$$\mathrm{SEV}(T(\mathbf{x}_0) \le c) = \mathbb{P}_{H_1}\bigl(T(\mathbf{X}) > T(\mathbf{x}_0)\bigr)$$
A non-rejection of the null hypothesis warrants the claim "true harm is less than $\delta_{\min}$" only if $\mathrm{SEV} \ge 0.80$. When power is low, $\mathbb{P}_{H_1}(T(\mathbf{X}) > c) = \mathrm{Power} \approx 1 - \beta$ is low ($\mathrm{SEV} \ll 0.80$), rendering the claim unseverely tested and epistemically baseless.

At the same time, A. W. Kimball (1957) and Ian Mitroff & Francisco Sagasti (1973) identified **Type III Error** (`is_type_three_error` in [`EpistemicMarginState`](crates/stochology/src/epistemic.rs#L145-L198)): solving the wrong problem precisely—achieving a positive narrow proxy gain $+G_{\mathrm{narrow}}$ while inflicting a larger unmeasured systemic damage $-D_{\mathrm{unmeasured}} < -G_{\mathrm{narrow}}$.

Joining Type III framing analysis with **Herbert Simon's (1947/1997 [1979])** theory of bounded rationality and **Jürgen Habermas's (1984 [1981/2012])** theory of communicative action ([`CognitiveFramingRegime`](crates/stochology/src/epistemic.rs#L22-L32)) exposes the organizational root of framing distortion: because human behavioral and sensory-motor information throughput is bounded by $\approx 10\text{ bits/s}$ (`[Literature]`, Zheng & Meister, 2025), creating a structural compression bottleneck at the organizational interface, every manager acts inevitably on the **decision premises** of subordinates (*"every manager is a cognitive influencer by obligation, but becomes a cognitive manipulator by choice"*).

To audit this institutional manipulation, Stochology formalizes the **Simon Premise Rigging Index** ($\mathrm{PRI}$). Let $\mathcal{P} = \{p_1, \dots, p_m\}$ be the discrete set of admissible decision premises disclosed to frontline decision-makers with probability/weight distribution $\boldsymbol{\omega} = (\omega_1, \dots, \omega_m)$. The Shannon entropy of disclosed premises against maximum entropy $H_{\max} = \log_2 m$ defines the constraint metric:
$$\mathrm{PRI} = 1 - \frac{H(\mathcal{P})}{H_{\max}} = 1 - \frac{-\sum_{i=1}^m \omega_i \log_2 \omega_i}{\log_2 m} \in [0, 1]$$
**Legitimate cognitive influence** (`CommunicativeInfluence`) shares partial-identification bounds $[L_t, U_t]$ and collateral spillover costs openly ($\mathrm{PRI} \le 0.20$), preserving operational variety. Conversely, **cognitive manipulation** (`StrategicPremiseRigging`) conceals epistemic width or spillover harm ($\mathrm{PRI} > 0.20$, reaching $\mathrm{PRI} = 0.864$ in our laboratory audit, `[Assumed]`) to trap frontline operators inside a **decision labyrinth pre-rigged at the top**, violating **Immanuel Kant's (1996 [1785/2007])** principle against instrumentalizing another agent's rational judgment.

---

## 2.4 Multiplicity ($e\text{-BH}$) and the Specification Curve (Multiverse)

When an analyst explores $K \ge 1$ plausible data-cleaning rules, covariate sets, or functional forms (Gelman & Loken's *garden of forking paths*), reporting only the single specification that crossed a significance threshold is a classic cherry-picking distortion. Stochology enforces two complementary defenses in [`epistemic.rs`](crates/stochology/src/epistemic.rs):

1. **The $e\text{-BH}$ Procedure ([`e_bh_discoveries`](crates/stochology/src/epistemic.rs#L83-L103), Wang & Ramdas, 2022):** Given $K$ e-values $E_{(1)} \ge E_{(2)} \ge \dots \ge E_{(K)}$ (where $\mathbb{E}_{H_0}[E_k] \le 1$), the $e\text{-BH}$ rule controls the False Discovery Rate at level $\alpha$ under **arbitrary dependence** across the $K$ specifications by finding the largest $k^*$ such that:

   $$\frac{k^* \, E_{(k^*)}}{K} \ge \frac{1}{\alpha}$$

2. **Simonsohn, Simmons & Nelson's (2020) Specification Curve / Steegen et al.'s (2016) Multiverse Analysis ([`EpistemicMarginState::from_multiverse`](crates/stochology/src/epistemic.rs#L382-L421)):** Instead of hiding the $K - 1$ alternative models, `EpistemicMarginState::from_multiverse` records the full vector of $K$ effect estimates and e-values, reporting the min–max envelope $[L_t, U_t]$ across all $K$ specifications, the **consistency ratio** (`multiverse_consistency`), and the average e-value $\bar{E} = \frac{1}{K}\sum_{k=1}^K E_k$.

---

## 2.5 Laboratory 2

Running laboratory 2 (`cargo run --release --bin stochology -- --chapter 2`, Simulated/Analytical) executes all four instruments of Filter I · Tension 1 via [`EpistemicMarginState`](crates/stochology/src/epistemic.rs#L145-L198):

```text
=== CHAPTER 2: FILTER I (S_EPI) — INQUIRY, OPENNESS & MULTIVERSE ===
  Cost-Loss Thresholds [Assumed]: institutional p* = 0.588 (EVPI=12.60) vs affected-citizen p* = 0.147 (EVPI=82.00), gap = 0.441
  Mayo Severe Power [Assumed]: 1.000 | Type III Error: true (global_net=-65.0) | Simon Framing: StrategicPremiseRigging vs CommunicativeInfluence (PRI = 0.864 vs 0.000, threshold = 0.20)
  Multiverse & e-BH [Assumed]: consistency = 83.3%, interval = [1.38, 1.61], e-BH discoveries = 5
```

- **Framing Standpoint Gap (`[Assumed]`):** At prior hazard $\pi_0 = 0.18$, efficacy $\eta = 0.85$, and protective action cost $C = 100$, the institutional threshold $p^*_{\mathrm{inst}} = 0.588$ (`[Assumed]`) prescribes **inaction** ($\pi_0 < 0.588$), whereas the affected community's threshold $p^*_{\mathrm{affected}} = 0.147$ (`[Assumed]`) prescribes **immediate protection** ($\pi_0 > 0.147$, with $\mathrm{EVPI}_{\mathrm{affected}} = 82.00$).
- **Type III Error & Simon Premise Audit (`[Assumed]`):** A narrow focal gain ($+45.0$) accompanied by unmeasured spillover harm ($-110.0$) produces negative global utility (`global_net = -65.0`, `is_type_three_error = true`, `[Assumed]`), while concealing epistemic width classifies the executive dashboard as `StrategicPremiseRigging`.
- **Multiverse & $e\text{-BH}$ (`[Assumed]`):** Across $K = 6$ specifications, `EpistemicMarginState::from_multiverse` yields $83.3\%$ consistency across `[1.38, 1.61]` and $5$ discoveries under $e\text{-BH}$ at $\alpha = 0.05$ (`[Assumed]`).

---

## Exercises

1. **Why $e\text{-BH}$ Needs No Independence Assumption:** Verify from Markov's inequality that if $E_1, \dots, E_K$ are non-negative random variables with $\mathbb{E}_{H_0}[E_k] \le 1$, their arithmetic mean $\bar{E} = \frac{1}{K}\sum_{k=1}^K E_k$ also satisfies $\mathbb{E}_{H_0}[\bar{E}] \le 1$ regardless of correlation among the $K$ specifications.
2. **Standpoint Elicitation:** In [`EpistemicMarginState::from_inquiry_framing`](crates/stochology/src/epistemic.rs#L247-L320), compute the minimum efficacy $\eta \in (0, 1]$ required for the institutional threshold $p^*_{\mathrm{inst}}$ to agree with the community's action decision at $\pi_0 = 0.18$.

