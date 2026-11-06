# Chapter 50 — Five Methodological Families, Process Tracing, and Reflexive Self-Application

> Laboratory 9 of the `stochology` CLI: `--chapter 9`.

Across Chapters 42 through 49, we constructed the complete [`StochologicalAgent`](crates/stochology/src/engine.rs) by uniting the Three Canonical Operators of the Nuclear Triad:
1. **Canonical Operator I — Epistemic Margin ($S_{\mathrm{epi}}$, Chapters 43, 44, 45):** [`EpistemicMarginState`](crates/stochology/src/epistemic.rs) in [`crates/stochology/src/epistemic.rs`](crates/stochology/src/epistemic.rs);
2. **Canonical Operator II — Vital Margin ($S_{\mathrm{vital}}$, Chapters 46, 47):** [`VitalMarginState`](crates/stochology/src/vital.rs) in [`crates/stochology/src/vital.rs`](crates/stochology/src/vital.rs);
3. **Canonical Operator III — Institutional Governance ($S_{\mathrm{gov}}$, Chapters 48, 49):** [`GovernanceState`](crates/stochology/src/governance.rs) in [`crates/stochology/src/governance.rs`](crates/stochology/src/governance.rs).

In this final framework chapter (implemented in [`crates/stochology/src/synthesis.rs`](crates/stochology/src/synthesis.rs)), we resolve the two overarching questions of the disciplinary treatise ([`notes/The Ethics of Aim.md`](../notes/The%20Ethics%20of%20Aim.md) §VI–§VIII):
1. **Methodological Generality:** How does Stochology govern not only quantitative simulation and econometrics, but also **qualitative, historical, archival, legal, and clinical inquiry** (*Process Tracing*)?
2. **Reflexive Self-Application:** What prevents Stochology itself from degenerating under Goodhart's Law when its three safeguards are turned into bureaucratic metrics, and under what exact conditions would Stochology's own foundational claims be **refuted**?

---

## 9.1 The Five Methodological Families of Stochology

Stochology does not replace individual disciplines; it acts as an integrative syntax layer governing the full mediation circuit across world, archive, and institution.

```
       World (Phenomenon)
          /        \
         /          \  [Filter I: S_epi]
   (Action)        Archive (Records, Data)
       |               |
 [Filter II: S_vital]  |
       |          Model / Inference
       \          /    [Filter III: S_gov]
        \        /
       Institution (Decision, Court, Clinic)
```

The Five Methodological Families map 1-to-1 to distinct segments of this mediation loop:

| Family | Domain of Inquiry | Primary Failure Mode (`NaiveOptimizer`) | Canonical Stochological Safeguards (`crates/stochology`) |
| :---: | :--- | :--- | :--- |
| **1** | **Observational, Administrative & Survey Inquiry** | Confounding, selection paradox ($\rho_{R,Y} \ne 0$), cluster echoes, uncounted strata | `EpistemicMarginState::from_archive_clusters` (Meng $\rho_{R,Y}$, Campbell–Moulton $\mathrm{DEFF}$, Kish $n_{\mathrm{eff}}$, Chao $\hat{n}_{00}$), `EpistemicMarginState::from_ate_bounds` (two-arm Manski & Lee bounds) |
| **2** | **Experimental, Clinical & Sequential Trials** | Optional stopping ($p$-hacking), differential attrition, dependent censoring, agnotological low power | `EpistemicMarginState::from_inquiry_framing` (Mayo–Popper severity, $e$-BH), `EpistemicMarginState::from_ate_bounds` (Ville $e$-processes, Lee attrition, Peterson survival) |
| **3** | **Predictive, Algorithmic & Performative Modeling** | Miscalibration, regime drift, Goodhart proxy gaming, Chouldechova disparity | `GovernanceState::from_ex_ante_gate` (CORP-PAV, ICP, Perdomo/MacKenzie, Goodhart), `GovernanceState::from_real_time_and_ex_post_gates` (Chouldechova trilemma) |
| **4** | **Physical, Ecological & Operational Systems** | Flaw of averages ($\Delta_J$), power-law tails ($\alpha < 2$), Kingman queue collapse, Itô variance drain | `VitalMarginState::from_heavy_tails_and_asymmetry` (Jensen $\Delta_J$, CRPS, Hill $\hat{\alpha}$), `VitalMarginState::from_queue_basin_and_survival` ($\mathrm{TTS}_{\mathrm{survival}} \ge \mathrm{TTR}_{\mathrm{recovery}}$, Baker–McHale Kelly, Canguilhem–Holling) |
| **5** | **Qualitative, Historical, Legal & Clinical Process Tracing** | Confirmation bias, ignoring alternative hypotheses ($\neg H$), confusing narrative coherence with evidential weight $V$ | `DiagnosticClue`, `ProcessTracingTestKind`, `ProcessTracingInference` |

---

## 9.2 Family 5 in Detail: Galenic *Sēmeîa*, Process Tracing, and Keynes's Evidential Weight $V$

In historical archival work, investigative journalism, legal fact-finding, and bedside clinical diagnosis, the investigator does not estimate a regression slope over $N = 50{,}000$ rows; the investigator traces a causal mechanism in a **single unfolding case** by evaluating qualitative clues (*sēmeîa* in Galen and Hippocrates; *Process Tracing* in Stephen Van Evera, 1997, and Andrew Bennett & Jeffrey Checkel, 2015, `[Literature]`).

### 9.2.1 Epistemic Foundations: The Clinical and Archival Clue

In [`DiagnosticClue`](crates/stochology/src/synthesis.rs#L222-L231), every qualitative clue $E$ is evaluated by two conditional probabilities:
1. **Sensitivity under Focal Hypothesis $H$:** $p_H = \mathbb{P}(E \mid H) \in (0, 1)$
2. **False-Positive Rate under Rival Explanation $\neg H$:** $p_{\neg H} = \mathbb{P}(E \mid \neg H) \in (0, 1)$

### 9.2.2 Bennett & Checkel Diagnostic Test Taxonomy

In [`ProcessTracingTestKind`](crates/stochology/src/synthesis.rs#L208-L218), the two-dimensional space $(p_H, p_{\neg H})$ partitions into Van Evera's four canonical tests (`[Literature]`):
- **`StrawInTheWind`** ($p_H < 0.85$ and $p_{\neg H} > 0.10$): Modest likelihood ratio; neither necessary nor sufficient; merely nudges the odds.
- **`HoopTest`** ($p_H \ge 0.85$ and $p_{\neg H} > 0.10$): **High necessity.** If $H$ is true, the clue *must* be present.
  - Likelihood ratio if observed: $\mathrm{LR}^+ = \frac{p_H}{p_{\neg H}} > 1$ (modest confirmation).
  - Likelihood ratio if **absent**: $\mathrm{LR}^- = \frac{1 - p_H}{1 - p_{\neg H}} \ll 1$ (**severe elimination**).
  - *Epistemic principle:* Passing a Hoop test does not prove $H$, but **failing a Hoop test refutes $H$**!
- **`SmokingGun`** ($p_H < 0.85$ and $p_{\neg H} \le 0.10$): **High sufficiency / Pathognomonic sign.**
  - Likelihood ratio if observed: $\mathrm{LR}^+ = \frac{p_H}{p_{\neg H}} \gg 1$ (**massive confirmation**).
  - Likelihood ratio if absent: $\mathrm{LR}^- = \frac{1 - p_H}{1 - p_{\neg H}} \approx 1$ (does not refute $H$).
- **`DoublyDecisive`** ($p_H \ge 0.85$ and $p_{\neg H} \le 0.10$): **Both necessary and sufficient.**
  - If observed: proves $H$ ($\mathrm{LR}^+ \gg 1$).
  - If absent: refutes $H$ ($\mathrm{LR}^- \ll 1$).

### 9.2.3 Turing–Good Deciban Calculus

Following Alan Turing and I. J. Good (1950, 1979) at Bletchley Park, evidence is accumulated on a logarithmic scale in **decibans** ($\mathrm{dB}$, `[Literature]`):
$$W(E; H : \neg H) = 10 \log_{10} \frac{\mathbb{P}(E \mid H)}{\mathbb{P}(E \mid \neg H)} = 10 \log_{10} \mathrm{LR}$$
Under conditional independence of clues:
$$W_{\mathrm{total}} = \sum_{j=1}^m W(E_j; H : \neg H) = 10 \log_{10} \prod_{j=1}^m \mathrm{LR}_j$$
Bayes' Rule in odds form:
$$\mathrm{Odds}_{\mathrm{post}} = \mathrm{Odds}_0 \times 10^{W_{\mathrm{total}} / 10}, \qquad \mathbb{P}(H \mid E_{1..m}) = \frac{\mathrm{Odds}_{\mathrm{post}}}{1 + \mathrm{Odds}_{\mathrm{post}}}$$

### 9.2.4 John Maynard Keynes's Evidential Weight $V$

In *A Treatise on Probability* (1921), John Maynard Keynes distinguished probability balance $P$ from **evidential weight** $V$ (the absolute volume and depth of the evidentiary base, `[Literature]`):
$$V = \frac{K}{K + I}$$
where $K$ is relevant knowledge acquired and $I$ is recognized remaining ignorance. As new clues are audited with evidentiary depth $\Delta K_j > 0$:
$$V_{\mathrm{post}} = \frac{K_0 + \sum_{j=1}^m \Delta K_j}{K_0 + \sum_{j=1}^m \Delta K_j + I_0}$$
A hypothesis can have high posterior probability $P = 0.95$ on flimsy weight $V = 0.05$ (a dangerous guess), or high probability $P = 0.95$ on mature weight $V = 0.85$ (a grounded finding).

### 9.2.5 Concrete Step-by-Step Worked Arithmetic Example

Let prior probability $p_0 = 0.25$, initial knowledge $K_0 = 10.0$, initial ignorance $I_0 = 40.0$ (`[Assumed]`).
Initial odds:
$$\mathrm{Odds}_0 = \frac{0.25}{1 - 0.25} = \frac{0.25}{0.75} = \frac{1}{3} \approx 0.33333, \qquad V_0 = \frac{10.0}{10.0 + 40.0} = 0.2000$$

Now evaluate three diagnostic clues from `run_chapter_9`:
1. **Clue 1 (`DoublyDecisive`):**
   $p_H = 0.96 \ge 0.85$ (necessary), $p_{\neg H} = 0.02 \le 0.10$ (sufficient). Observed: `true`.
   $\mathrm{LR}_1 = \frac{0.96}{0.02} = 48.0 \implies W_1 = 10 \log_{10}(48.0) = +16.812\text{ dB}, \; \Delta K_1 = 25.0$.
2. **Clue 2 (`SmokingGun`):**
   $p_H = 0.75 < 0.85$ (not necessary), $p_{\neg H} = 0.04 \le 0.10$ (sufficient). Observed: `true`.
   $\mathrm{LR}_2 = \frac{0.75}{0.04} = 18.75 \implies W_2 = 10 \log_{10}(18.75) = +12.730\text{ dB}, \; \Delta K_2 = 20.0$.
3. **Clue 3 (`HoopTest`):**
   $p_H = 0.99 \ge 0.85$ (necessary), $p_{\neg H} = 0.35 > 0.10$ (not sufficient). Observed: `true`.
   $\mathrm{LR}_3 = \frac{0.99}{0.35} = \frac{99}{35} \approx 2.82857 \implies W_3 = 10 \log_{10}(2.82857) = +4.516\text{ dB}, \; \Delta K_3 = 15.0$.

Total Bayesian Likelihood Ratio and Decibans:
$$\mathrm{LR}_{\mathrm{total}} = 48.0 \times 18.75 \times \frac{99}{35} = 900.0 \times 2.8285714 = 2545.7143$$
$$W_{\mathrm{total}} = 10 \log_{10}(2545.7143) = +34.058 \approx +34.1\text{ dB}$$
Updated Posterior Odds and Probability:
$$\mathrm{Odds}_{\mathrm{post}} = \frac{1}{3} \times 2545.7143 = 848.5714 \implies p_{\mathrm{post}} = \frac{848.5714}{1 + 848.5714} = 0.998823 \approx 0.9988$$
Updated Keynes Evidential Weight:
$$K_{\mathrm{post}} = 10.0 + 25.0 + 20.0 + 15.0 = 70.0 \implies V_{\mathrm{post}} = \frac{70.0}{70.0 + 40.0} = \frac{70.0}{110.0} = 0.63636 \approx 0.64$$
This matches the CLI output verbatim: `Process Tracing (Bennett-Checkel) [Assumed]: prior = 0.25 -> posterior = 0.9988 (decibans = +34.1, V: 0.20 -> 0.64)`.

### 9.2.6 Refutability and the Asymmetry of the Hoop Test

In [`ProcessTracingInference::evaluate`](crates/stochology/src/synthesis.rs#L263-L311), qualitative inference is valid only if:
- probabilities respect Cromwell's interior bounds;
- each clue updates both Bayesian posterior odds and Keynes's evidential weight;
- **Popper–Mayo Refutability (`has_refutable_hoop_condition = true`):** the inquiry includes at least one `HoopTest` or `DoublyDecisive` clue.

Suppose Clue 3 was absent (`clue_observed = false`). Its active likelihood ratio becomes:
$$\mathrm{LR}_3^- = \frac{1 - 0.99}{1 - 0.35} = \frac{0.01}{0.65} = \frac{1}{65} \approx 0.01538 \implies W_3^- = 10 \log_{10}(0.01538) = -18.13\text{ dB}$$
Total likelihood ratio collapses to $\frac{900}{65} \approx 13.85$ ($W = +11.4\text{ dB}$). And crucially, `ProcessTracingInference` sets `hoop_refuted = true` and `has_refutable_hoop_condition = false`, mathematically rejecting the hypothesis despite positive remaining clues!

---

## 9.3 Reflexive Self-Application: The Four Internal Goodhart Brakes

When an institution turns any evaluative metric into an administrative target, it ceases to be a good metric (Goodhart, 1975; Strathern, 1997; Espeland & Stevens, 1998, `[Literature]`). Stochology prevents its own three safeguards from degenerating through four internal mathematical and institutional brakes:

### 9.3.1 Brake 1 (Epistemic Margin): Skeptical Inflation vs. CORP Resolution

- *Perversion:* Analysts could achieve $100\%$ coverage by inflating interval bounds to the trivial domain $[y_{\min}, y_{\max}]$, or predicting the constant sample mean everywhere.
- *Mathematical Brake:* Under the Murphy (1973) and Dimitriadis, Gneiting, and Jordan (2021) CORP decomposition:
  $$\mathrm{Brier}(p, y) = \bar{y}(1 - \bar{y}) + \mathrm{REL}_{\mathrm{CORP}} - \mathrm{RES}_{\mathrm{CORP}}$$
  A constant base-rate forecast achieves $\mathrm{REL} = 0$, but has **zero resolution** ($\mathrm{RES}_{\mathrm{CORP}} = 0.000$).
  [`ReflexiveBrakeAudit`](crates/stochology/src/synthesis.rs#L335-L355) requires $\mathrm{RES}_{\mathrm{sharp}} > \mathrm{RES}_{\mathrm{lazy}} + 0.01$.
  On `data/atlantic_storms.csv`, the sharp forecast yields $\mathrm{RES}_{\mathrm{sharp}} = 0.117 > \mathrm{RES}_{\mathrm{lazy}} = 0.000$, defeating lazy skeptical inflation (`[Measured]`).

### 9.3.2 Brake 2 (Vital Margin): Regressive Hoarding vs. Scarf–Samuelson Welfare

- *Perversion:* Administrators could hoard excessive reserves ($R \to \infty$) under the pretext of risk aversion, starving urgent present human needs.
- *Mathematical Brake:* Net social welfare balances concave precautionary benefit against linear marginal opportunity cost:
  $$W(R) = \kappa \ln(1 + R) - c_{\mathrm{urgent}} R$$
  Differentiating with respect to $R$:
  $$\frac{dW}{dR} = \frac{\kappa}{1 + R} - c_{\mathrm{urgent}} = 0 \implies R^* = \max\left(0,\, \frac{\kappa}{c_{\mathrm{urgent}}} - 1\right)$$
  For $\kappa = 4.0$ and $c_{\mathrm{urgent}} = 1.0$:
  $$R^* = \frac{4.0}{1.0} - 1 = 3.0 \implies W(R^*) = 4.0 \ln(4) - 3.0 \approx 4.0 \times 1.38629 - 3.0 = +2.55 \quad \text{\texttt{[Assumed]}}$$
  Under regressive hoarding ($R_{\mathrm{hoard}} = 25.0$):
  $$W(R_{\mathrm{hoard}}) = 4.0 \ln(26.0) - 25.0 \approx 4.0 \times 3.25810 - 25.0 = 13.032 - 25.0 = -11.97 \quad \text{\texttt{[Assumed]}}$$
  Because $W(R^*) = +2.55 > W(R_{\mathrm{hoard}}) = -11.97$, hoarding is penalized as value-destructive.

### 9.3.3 Brake 3 (Institutional Governance): Procedural Theater vs. Kairós Emergency Fast-Path

- *Perversion:* Compliance committees demand exhaustive reviews ($\tau_{\mathrm{bureaucratic}} = 12.0$ shifts) during an acute physical emergency where time-to-survival is only $\mathrm{TTS}_{\mathrm{survival}} = 4.0$ shifts.
- *Mathematical Brake:* Whenever $\mathrm{TTS}_{\mathrm{survival}} < \tau_{\mathrm{bureaucratic}}$, procedural delay guarantees catastrophic ruin. Sosa's (2015) meta-aptness activates the pre-committed emergency fast-path:
  $$\tau_{\mathrm{fast}} \le \mathrm{TTS}_{\mathrm{survival}} < \tau_{\mathrm{bureaucratic}}$$
  In Laboratory 9: $\tau_{\mathrm{fast}} = 1.5 \le \mathrm{TTS}_{\mathrm{survival}} = 4.0 < 12.0$ (`[Assumed]`). The fast-path releases vital buffers immediately, records the wide partial-identification interval $[L_t, U_t]$ into [`TemporalVault`](crates/stochology/src/governance.rs#L472-L556), and defers collegiate forensic review until physical survival is secured.

### 9.3.4 Brake 4 (Composite Metric): Anti-Checklist Rule & Homothetic Degree-1 Nested Leontief Aggregation

- *Perversion:* The composite index $M_{\mathrm{Leontief}} = \min(S_{\mathrm{epi}}, S_{\mathrm{vital}}, S_{\mathrm{gov}})$ is turned into an executive bonus metric or certification badge, inviting box-ticking, cosmetic denominator inflation, and gaming (Espeland & Stevens, 1998; Hacking, 1995, `[Literature]`).
- *Institutional & Mathematical Brake:*
  1. *Constitutional Rule:* $M_{\mathrm{Leontief}}$ is legally prohibited from serving as a scalar reward target (`leontief_scalar_target_absent = true`, `[Assumed]`). It operates strictly as a diagnostic circuit-breaker.
  2. *Degree-1 Homotheticity:* Degree-1 nested Leontief aggregation $\min_i g_i$ avoids multiplicative volume collapse (proven below).

---

## 9.4 Leontief Non-Substitutable Aggregation vs. Multiplicative and Arithmetic Collapse

### 9.4.1 Proof of 3D Lebesgue Feasible Volume Contraction

Consider evaluating three safeguards $(e, v, g) \in [0, 1]^3$ against safety threshold $\tau \in (0, 1)$.

**1. Pure Nested Leontief ($M_{\mathrm{Leontief}} = \min(e, v, g) \ge \tau$):**
The feasible region is the cube $[\tau, 1]^3$. Its Lebesgue volume is:
$$V_{\min}(\tau) = \int_\tau^1 de \int_\tau^1 dv \int_\tau^1 dg = (1 - \tau)^3$$
For $\tau = 0.50$:
$$V_{\min}(0.50) = (1 - 0.50)^3 = 0.50^3 = 0.1250 \quad \text{\texttt{[Assumed]}}$$

**2. Continuous Multiplicative Product ($e \cdot v \cdot g \ge \tau$):**
The feasible region is $\Omega_{\mathrm{prod}} = \{(e, v, g) \in [0, 1]^3 : e \ge \tau, \; v \ge \tau / e, \; g \ge \tau / (e v)\}$.
We integrate directly:
$$V_{\mathrm{prod}}(\tau) = \int_\tau^1 de \int_{\tau/e}^1 dv \int_{\tau/(ev)}^1 dg = \int_\tau^1 de \int_{\tau/e}^1 \left(1 - \frac{\tau}{ev}\right) dv$$
Inner integral with respect to $v$:
$$\int_{\tau/e}^1 \left(1 - \frac{\tau}{ev}\right) dv = \left[ v - \frac{\tau}{e}\ln v \right]_{\tau/e}^1 = 1 - \left(\frac{\tau}{e} - \frac{\tau}{e}\ln\frac{\tau}{e}\right) = 1 - \frac{\tau}{e} + \frac{\tau}{e}(\ln\tau - \ln e)$$
Outer integral with respect to $e$:
$$V_{\mathrm{prod}}(\tau) = \int_\tau^1 \left(1 - \frac{\tau}{e} + \frac{\tau\ln\tau}{e} - \frac{\tau\ln e}{e}\right) de$$
Computing each of the four terms from $\tau$ to $1$:
1. $\int_\tau^1 1\,de = 1 - \tau$
2. $-\int_\tau^1 \frac{\tau}{e}de = -\tau [\ln 1 - \ln\tau] = \tau\ln\tau$
3. $\int_\tau^1 \frac{\tau\ln\tau}{e}de = \tau\ln\tau [-\ln\tau] = -\tau(\ln\tau)^2$
4. $-\int_\tau^1 \frac{\tau\ln e}{e}de = -\tau \left[ \frac{(\ln e)^2}{2} \right]_\tau^1 = \frac{\tau}{2}(\ln\tau)^2$

Summing the terms:
$$V_{\mathrm{prod}}(\tau) = 1 - \tau + \tau\ln\tau - \frac{\tau}{2}(\ln\tau)^2$$

Evaluating at $\tau = 0.50$:
$$\ln(0.50) \approx -0.693147, \qquad (\ln 0.50)^2 \approx 0.480453$$
$$V_{\mathrm{prod}}(0.50) = 0.500000 - 0.346574 - 0.120113 = 0.033313 \approx 0.0333 \quad \text{\texttt{[Assumed]}}$$

**Volume Contraction Ratio:**
$$\frac{V_{\mathrm{prod}}(0.50)}{V_{\min}(0.50)} = \frac{0.033313}{0.125000} \approx 0.2665 \implies \text{Volume Loss} = 1 - 0.2665 = -73.35\% \quad \text{\texttt{[Assumed]}}$$

### 9.4.2 False Collapses on Healthy Grids vs. False-Safe Failures on Ruin Boundaries

In [`LeontiefVsMultiplicativeAudit`](crates/stochology/src/synthesis.rs#L455-L493):
1. **Healthy Factorial Grid $\{0.60, 0.75, 0.90\}^5$ ($243$ points):**
   - Every coordinate is strictly above $\tau = 0.50$.
   - **Pure Leontief ($\min_i \theta_i \ge 0.50$):** $0 / 243$ false collapses ($0.00\%$, `[Assumed]`).
   - **Hybrid Governance ($\min(\theta_1, \theta_2, \theta_3\theta_4\theta_5) \ge 0.50$):** $180 / 243$ false collapses (**$74.07\%$**, `[Assumed]`)!
   - **5-Factor Product ($\prod_{i=1}^5 \theta_i \ge 0.50$):** $242 / 243$ false collapses (**$99.59\%$**, `[Assumed]`)! Only $(0.90)^5 = 0.59049$ survives; all other healthy states are condemned by uncalibrated multiplicative damping!
2. **Single-Axis Ruin Boundary Grid ($36$ points):**
   - One axis is in acute collapse ($r \in \{0.00, 0.15, 0.30\} < \tau = 0.50$) while the other two are healthy ($s_1, s_2 \in \{0.80, 0.90\}$).
   - **Compensatory Arithmetic Mean ($M_{\mathrm{arith}} = \frac{e + v + g}{3} \ge 0.50$):**
     Even at total zero $r = 0.00$, $M_{\mathrm{arith}} = \frac{0.00 + 0.80 + 0.80}{3} = \frac{1.60}{3} = 0.533 \ge 0.50$.
     Arithmetic mean issues a **false-safe pass in $36 / 36$ cases ($100.0\%$, `[Assumed]`)**!
   - **Pure Leontief ($M_{\mathrm{Leontief}} = \min(e, v, g) \ge 0.50$):**
     $0 / 36$ false-safes ($0.0\%$, `[Assumed]`). Leontief immediately halts execution on every single catastrophic failure.

---

## 9.5 Severe Testing and the Four Pre-Registered Falsifiers

### 9.5.1 Deborah Mayo's Severity Principle and Burden-of-Proof Inversion

A common methodological flaw is setting the null hypothesis as "no difference" ($H_0: \Delta = 0$) and claiming a theory is "corroborated" merely because a noisy test failed to reject $H_0$. Stochology inverts the burden of proof (Mayo, 2018, `[Literature]`):
$$H_0^{\mathrm{fals}}: \Delta_{\mathrm{harm}} \ge \delta_{\min} \quad \text{vs.} \quad H_1^{\mathrm{fals}}: \Delta_{\mathrm{harm}} < \delta_{\min}$$
To refute a Stochological safeguard, the challenger must actively demonstrate with high post-data severity that omitting the safeguard incurs less than the substantive harm threshold $\delta_{\min}$.

### 9.5.2 Pre-Registered Design Formula

Given significance $\alpha = 0.05$ ($z_\alpha = 1.6449$) and power $1 - \beta = 0.90$ ($z_{1-\beta} = 1.2816$):
$$n_{\mathrm{eff}}^{\mathrm{req}} = \left\lceil \left( \frac{(|z_\alpha| + z_{1-\beta})\sigma_d}{\delta_{\min}} \right)^2 \right\rceil$$
$$\mathrm{SE} = \frac{\sigma_d}{\sqrt{n_{\mathrm{eff}}^{\mathrm{req}}}}, \qquad d_{\mathrm{crit}} = \delta_{\min} - |z_\alpha|\mathrm{SE}, \qquad d_{\mathrm{corrob}} = \delta_{\min} + |z_\alpha|\mathrm{SE}$$
Post-data severity for refutation when $\bar{d} \le d_{\mathrm{crit}}$:
$$\mathrm{SEV}_{\mathrm{fals}} = \Phi\left(\frac{\delta_{\min} - \bar{d}}{\mathrm{SE}_{\mathrm{obs}}}\right) \ge 0.95$$

### 9.5.3 The Four Canonical Falsifiers

In [`PreregisteredFalsifier`](crates/stochology/src/synthesis.rs#L714-L734):
1. **$F_1$ ($S_{\mathrm{epi}}$, Observational Transport):** $\delta_{\min} = 0.15, \sigma_d = 0.25 \implies n_{\mathrm{eff}}^{\mathrm{req}} = 24, d_{\mathrm{crit}} = 0.0661$ (`[Assumed]`).
2. **$F_2$ ($S_{\mathrm{vital}}$, Heavy Tails & Queues):** $\delta_{\min} = 1.50, \sigma_d = 2.20 \implies n_{\mathrm{eff}}^{\mathrm{req}} = 19, d_{\mathrm{crit}} = 0.6698$ (`[Assumed]`).
3. **$F_3$ ($S_{\mathrm{gov}}$, Multitask Proxy Gaming):** $\delta_{\min} = 0.20, \sigma_d = 0.30 \implies n_{\mathrm{eff}}^{\mathrm{req}} = 20, d_{\mathrm{crit}} = 0.0897$ (`[Assumed]`).
4. **$F_4$ ($M_{\mathrm{Leontief}}$, Non-Compensability):** $\delta_{\min} = 0.25, \sigma_d = 0.35 \implies n_{\mathrm{eff}}^{\mathrm{req}} = 17, d_{\mathrm{crit}} = 0.1104$ (`[Assumed]`).

### 9.5.4 Empirical Results in Laboratory 9

- `F1` ($d_{\mathrm{obs}} = 0.321 > d_{\mathrm{crit}} = 0.0661, \mathrm{SEV} = 0.995$): Corroborates $S_{\mathrm{epi}}$ (`[Simulated]`).
- `F2` ($d_{\mathrm{obs}} = 2.514 > d_{\mathrm{crit}} = 0.6698, \mathrm{SEV} = 0.982$): Corroborates $S_{\mathrm{vital}}$ (`[Simulated]`).
- `F3` ($d_{\mathrm{obs}} = 0.385 > d_{\mathrm{crit}} = 0.0897, \mathrm{SEV} = 0.972$): Corroborates $S_{\mathrm{gov}}$ (`[Simulated]`).
- `F4` ($d_{\mathrm{obs}} = 0.077 \le d_{\mathrm{crit}} = 0.1104, \mathrm{SEV} = 0.996, \text{refuted} = \text{true}$): **Genuinely refuted!** (`[Simulated]`)
  *Theoretical significance:* `CompensatoryBufferAgent` (doubling physical reserve buffer to $50\%$) partially absorbs mild demand shocks when no absorbing survival barrier is active. This confirms that non-substitutability is a **regime-conditional boundary law**: strict when absorbing ruin barriers and severe non-ergodic shocks are present, but partially compensable under moderate shocks with doubled physical slack.

---

## 9.6 Laboratory 9: Synthesis, Process Tracing, and the Factorial Ablation

Running laboratory 9 (`cargo run --release --bin stochology -- --chapter 9`) executes the complete synthesis and evaluates and stress-tests all three Collapse Propositions ([`CollapseProposition`](crates/stochology/src/synthesis.rs#L18-L49) & [`TriadicIntegrity`](crates/stochology/src/synthesis.rs#L54-L74)), the $2\times 2\times 2$ factorial design, the `CompensatoryBufferAgent` non-compensability test, the $243$-point threshold perturbation grid (`LeontiefVsMultiplicativeAudit`), the four pre-registered falsifiers (`PreregisteredFalsifier`), and the $100$-seed Monte Carlo evaluation (`seeds = 2000..2099`) on `CommunityReserveEnv` (`[Simulated]`):

```text
=== CHAPTER 9: METHODOLOGICAL FAMILIES, PROCESS TRACING & REFLEXIVITY ===
  2x2x2 Factorial Ablation of the Nuclear Triad [Simulated] (M_Leontief = min(S_epi, S_vital, S_gov)):
    [E=1 V=1 G=1] StochologicalAgent(FullTriad:111)            -> M_Leontief=0.814 (M_arith=0.895), collapse=IntactTriad                 , cov= 95.8% (shock= 75.0%), harm=  1.59 (MC100=  1.55±0.01 [  1.44,   1.69], ruin=  0.0%), R_24= 58.17
    [E=0 V=1 G=1] DefectiveAgent(NoEpistemicMargin->Prop1:011) -> M_Leontief=0.235 (M_arith=0.624), collapse=Proposition1BlindUndersizing, cov= 25.0% (shock=  0.0%), harm=  4.49 (MC100=  4.91±0.12 [  4.38,   5.18], ruin=  4.0%), R_24= 27.21
    [E=1 V=0 G=1] DefectiveAgent(NoVitalMargin->Prop2:101)     -> M_Leontief=0.021 (M_arith=0.503), collapse=Proposition2ImpotentLucidity, cov= 95.8% (shock= 75.0%), harm= 77.19 (MC100= 80.99±0.27 [ 77.12,  82.77], ruin=100.0%), R_24=  0.00
    [E=1 V=1 G=0] DefectiveAgent(NoGovernance->Prop3:110)      -> M_Leontief=0.101 (M_arith=0.612), collapse=Proposition3CynicalCapture  , cov= 95.8% (shock= 75.0%), harm= 68.01 (MC100= 67.89±0.01 [ 67.69,  68.10], ruin=100.0%), R_24=  0.00
    [E=1 V=0 G=0] PartialAgent(EpistemicOnly:100)              -> M_Leontief=0.010 (M_arith=0.295), collapse=CombinedPropositions2And3   , cov= 95.8% (shock= 75.0%), harm=114.79 (MC100=114.71±0.02 [114.42, 114.98], ruin=100.0%), R_24=  0.00
    [E=0 V=1 G=0] PartialAgent(VitalOnly:010)                  -> M_Leontief=0.094 (M_arith=0.392), collapse=CombinedPropositions1And3   , cov= 20.8% (shock=  0.0%), harm= 80.50 (MC100= 85.45±0.42 [ 80.08,  91.81], ruin=100.0%), R_24=  0.00
    [E=0 V=0 G=1] PartialAgent(GovernanceOnly:001)             -> M_Leontief=0.011 (M_arith=0.288), collapse=CombinedPropositions1And2   , cov= 16.7% (shock=  0.0%), harm=109.58 (MC100=110.81±0.24 [109.50, 115.11], ruin=100.0%), R_24=  0.00
    [E=0 V=0 G=0] DefectiveAgent(AllAxesOff:000)               -> M_Leontief=0.008 (M_arith=0.085), collapse=TotalTriadicCollapse        , cov= 16.7% (shock=  0.0%), harm=125.83 (MC100=124.38±0.24 [120.32, 126.04], ruin=100.0%), R_24=  0.00
  Fixed-Budget Mechanism Isolation Factorial (2^3 x 3 = 24 cells, B_fixed in [8.0, 14.0, 20.0], f=0.65) [Simulated]: delta_harm_Epi=+5.57, delta_harm_Vital=+5.81, delta_harm_Gov=+91.28 (FullTriad mean_harm=19.86)
  Non-Substitutability & Negative Controls [Simulated]: CompensatoryGiantBuffer(NoEpi+2xBuffer) cov=25.0%, harm=3.89 (MC100=4.06±0.02), R_24=39.88, diag=Proposition1BlindUndersizing | BenignSmallWorld(NaiveOptimizer) R_24=71.40, ruin=false | 3^5=243 Threshold Invariance=100.0% | 3^5=243 Local Structural DGP Invariance=100.0% | Wide-Band Stress Grid: dom=66.7%, intact=88.9%
  Closed-Form Prop 1 Coverage Decay Ladder [Assumed]: engine(n=46,rho=0.62)=35.9% | Meng ladder(rho=-0.05): n=46->90.5%, n=500->46.6%, n=5k->0.0%, n=50k->0.00%, n=2.3M->0.0000% (benchmark_met=true, manski_cov=100.0%, reserve_at_TTR=-78.8)
  Process Tracing (Bennett-Checkel) [Assumed]: prior = 0.25 -> posterior = 0.9988 (decibans = +34.1, V: 0.20 -> 0.64)
  Four Reflexive Brakes [Measured forecasts + Assumed welfare/TTR]: RES_sharp=0.117 > RES_lazy=0.000, welfare_opt=2.55 > hoard=-11.97, fast_path (TTR_recovery=1.5 <= TTS_survival=4.0), scalar_target_absent=true (all_active = true)
  Nested Leontief vs Multiplicative Damping [Assumed] (tau=0.50, 3^5=243 grid): V_min=0.1250 vs V_prod=0.0333 (-73.35% vol) | False Collapses: PureLeontief=0.00% (0/243) vs HybridGov=74.07% (180/243) vs Prod5=99.59% (242/243; vol-matched tau_prod=0.2951: 192/243; power-p05: 0/243) | Single-Axis Ruin False-Safe: Arith=100.0% (36/36) vs Leontief=0.0% (0/36)
  Four Pre-Registered Falsifiers [Assumed design + Simulated paired MC100 evaluation] (Mayo SEV >= 0.95): F1(n_eff=24, d_crit=0.0661, d_obs=0.321, SEV=0.995, refuted=false) | F2(n_eff=19, d_crit=0.6698, d_obs=2.514, SEV=0.982, refuted=false) | F3(n_eff=20, d_crit=0.0897, d_obs=0.385, SEV=0.972, refuted=false) | F4(n_eff=17, d_crit=0.1104, d_obs=0.077, SEV=0.996, refuted=true)
[STOCHOLOGY VERIFIED] All 9 progressive chapter laboratories and 3 Collapse Propositions passed.
```

### 9.6.1 In-Depth Analysis of the Empirical Results

- **`StochologicalAgent::without_epistemic_margin()` (`011`)** ($\neg\text{Epistemic Margin}$): diagnoses **`Proposition1BlindUndersizing`** ($25.0\%$ demand coverage under the naive i.i.d. 95% CI, $0.0\%$ shock coverage, $M_{\mathrm{Leontief}} = 0.235$, nearly triple convex social harm $H = 4.49$ / `MC100 = 4.91±0.12 [4.38, 5.18]`, $4.0\%$ absorbing ruin across $100$ seeds, and **net reserve erosion** $R_{24} = 27.21 < R_0 = 45.0$, `[Simulated]`).
- **Fixed-Budget Mechanism Isolation Factorial ([`FixedBudgetFactorialAudit`](crates/stochology/src/engine.rs#L156-L167), `[Simulated]`):** Holding reserve buffer $B_{\mathrm{fixed}} \in \lbrace 8.0, 14.0, 20.0\rbrace$, base capacity, and growth leverage $f = 0.65$ identical across all $2^3 = 8$ mechanism toggles ($24$ cells) confirms that every axis contributes strictly positive harm reduction independent of resource endowment ($\Delta H_{\mathrm{epi}} = +5.57, \Delta H_{\mathrm{vital}} = +5.81, \Delta H_{\mathrm{gov}} = +91.28$, `[Simulated]`).
- **Closed-Form Proposition 1 Coverage Decay Ladder ([`StructuralCollapseProofAudit`](crates/stochology/src/synthesis.rs#L859-L876), `[Assumed]`):** Unites the finite-sample simulator regime (`n = 46, f = 46/60, |rho_RY| = 0.62`, theoretical i.i.d. coverage $= 35.9\%$ vs. empirical $25.0\%$ with cluster shocks) and the 5-step Meng Big Data Paradox ladder across $n \in \lbrace 46, 500, 5{,}000, 50{,}000, 2{,}300{,}000\rbrace$ at $\rho_{R,Y} = -0.05$, evaluating analytical coverage decay across sample sizes ($90.5\% \to 46.6\% \to 0.0\% \to 0.0000\%$, with Bhatia–Davis support coverage `manski_cov = 100.0%` and signed reserve balance at recovery `reserve_at_TTR = -78.8`, `[Assumed]`).
- **`CompensatoryBufferAgent`** ($\neg\text{Epistemic Margin}$ with doubled $50\%$ static reserve buffer): still suffers $25.0\%$ demand coverage and $2.45\times$ convex social harm (`harm = 3.89`, `MC100 = 4.06±0.02 > 1.55 Full`, and net reserve erosion `R_24 = 39.88 < 45.0`, `[Simulated]`), demonstrating that physical buffer stock cannot compensate for epistemic blindness.
- **`StochologicalAgent::epistemic_only()` (`101`)** ($\neg\text{Vital Margin}$): diagnoses **`Proposition2ImpotentLucidity`** ($100\%$ absorbing ruin across $100$ seeds, `harm = 77.19`, `MC100 = 80.99±0.27 [77.12, 82.77]`, with arithmetic mean $M_{\mathrm{arith}} = 0.503$ giving a false-safe signal against $M_{\mathrm{Leontief}} = 0.021$, `[Simulated]`).
- **`StochologicalAgent::without_governance()` (`110`)** ($\neg\text{Institutional Governance}$): diagnoses **`Proposition3CynicalCapture`** ($100\%$ absorbing ruin across $100$ seeds, `harm = 68.01`, `MC100 = 67.89±0.01 [67.69, 68.10]`, `[Simulated]`).
- **$3^5 = 243$ Threshold, Local DGP, and Wide-Band Stress Grids (`verify_threshold_sensitivity`, `verify_structural_perturbation_grid`, & `verify_wide_band_structural_sensitivity`, `[Simulated]`):** Perturbing the 5 diagnostic thresholds $(\tau, \rho_{\mathrm{crit}}, \rho_{1,\mathrm{crit}}, f_{\mathrm{ceil}}, B_{\min})$ and local DGP parameters yields **$100.0\%$ ($243/243$) invariance**, while the wide-band stress grid (`severe_record_prob` $\in \lbrace 0.20, 0.40, 0.65\rbrace$, `pareto_alpha` $\in \lbrace 1.15, 1.45, 2.05\rbrace$, `overload_coeff` $\in \lbrace 0.8, 1.8, 3.2\rbrace$) maps the exact breakdown boundary of net reserve growth and survival ($66.7\%$ full reserve-growth dominance and $88.9\%$ intact survival, with absorbing ruin under simultaneous $\alpha = 1.15$ and $3.2\times$ overload cost, `[Simulated]`).
- **`StochologicalAgent::full()` (`111`)** (all three safeguards active): achieves **`IntactTriad`** with $0.0\%$ ruin across all $100$ seeds (`harm = 1.59`, `MC100 = 1.55±0.01 [1.44, 1.69]`, `M_Leontief = 0.814`, `R_24 = 58.17`, `[Simulated]`).
- **Four Pre-Registered Falsifiers (`F1`–`F4`, `[Assumed design + Simulated paired MC100 evaluation]`):** Evaluated on two-stratum (benign vs. stressed) multi-regime cohorts at $\mathrm{DEFF} = 1.5$, **`F1`** ($d_{\mathrm{obs}} = 0.321 > d_{\mathrm{crit}} = 0.0661, \mathrm{SEV} = 0.995, \text{refuted} = \text{false}$), **`F2`** ($d_{\mathrm{obs}} = 2.514 > d_{\mathrm{crit}} = 0.6698, \mathrm{SEV} = 0.982, \text{refuted} = \text{false}$), and **`F3`** ($d_{\mathrm{obs}} = 0.385 > d_{\mathrm{crit}} = 0.0897, \mathrm{SEV} = 0.972, \text{refuted} = \text{false}$) corroborate the necessity of $S_{\mathrm{epi}}, S_{\mathrm{vital}}, S_{\mathrm{gov}}$ on the same order of magnitude as $d_{\mathrm{crit}}$ without saturating at $\mathrm{SEV} = 1.000$. Conversely, **`F4`** ($d_{\mathrm{obs}} = 0.077 \le d_{\mathrm{crit}} = 0.1104, \mathrm{SEV} = 0.996, \text{refuted} = \text{true}$) **refutes unconditional non-compensability** in a mostly mild archival-defect regime where doubled physical reserves (`CompensatoryBufferAgent`) partially absorb epistemic undersizing—confirming that non-substitutability is a regime-conditional boundary law (strict when an absorbing ruin barrier or severe defect is active; partially compensable under moderate shocks with doubled physical slack).

---

## Exercises

1. **Failing a Hoop Test and Deciban Dynamics in Process Tracing:**
   In [`ProcessTracingInference::evaluate`](crates/stochology/src/synthesis.rs#L263-L311), suppose a historical investigator is evaluating hypothesis $H$ against rival $\neg H$ with prior $p_0 = 0.25$, $K_0 = 10.0$, and $I_0 = 40.0$. The investigator uncovers Clue 1 (a Smoking Gun with $p_H = 0.75, p_{\neg H} = 0.04, \Delta K = 20.0$, observed `true`). However, Clue 2 is a crucial Hoop Test ($p_H = 0.98, p_{\neg H} = 0.40, \Delta K = 15.0$) that searches for an official ministerial memo authorizing the intervention, but the archives reveal the memo was **never written** (`clue_observed = false`).
   - Calculate the active likelihood ratio $\mathrm{LR}_2^-$ and its deciban contribution $W_2^-$.
   - Compute the cumulative decibans $W_{\mathrm{total}}$ and posterior probability $p_{\mathrm{post}}$.
   - Explain why `ProcessTracingInference` flags `hoop_refuted = true` even if posterior probability remains above $0.50$, and relate this to Karl Popper's asymmetry between falsification and confirmation.
   - *Guidance:* $\mathrm{LR}_2^- = \frac{1 - 0.98}{1 - 0.40} = \frac{0.02}{0.60} = \frac{1}{30} \approx 0.03333$. $W_2^- = 10 \log_{10}(1/30) = -14.77\text{ dB}$. For Clue 1, $\mathrm{LR}_1^+ = 0.75 / 0.04 = 18.75$ ($+12.73\text{ dB}$). Total likelihood ratio $\mathrm{LR}_{\mathrm{total}} = 18.75 \times (1/30) = 0.625$ ($W_{\mathrm{total}} = -2.04\text{ dB}$). Posterior odds $O_{\mathrm{post}} = (1/3) \times 0.625 \approx 0.2083$, yielding $p_{\mathrm{post}} \approx 17.2\%$. Even with positive initial signs, failing a necessary condition refutes $H$.

2. **Analytical Lebesgue Volume Contraction Proof:**
   Consider an institution choosing between pure Leontief aggregation $M_{\mathrm{Leontief}} = \min(S_1, S_2, S_3) \ge \tau$ and multiplicative product aggregation $M_{\mathrm{prod}} = S_1 S_2 S_3 \ge \tau$ over safety scores $(S_1, S_2, S_3) \in [0, 1]^3$ at threshold $\tau = 0.60$.
   - Calculate the analytical feasible volume $V_{\min}(0.60)$.
   - Using the closed-form formula $V_{\mathrm{prod}}(\tau) = 1 - \tau + \tau \ln\tau - \frac{\tau}{2}(\ln\tau)^2$, compute $V_{\mathrm{prod}}(0.60)$.
   - Determine the percentage of viable operating space destroyed by multiplicative damping.
   - Explain why multiplicative gate products cause organizational burnout when applied across five independent operational divisions.
   - *Guidance:* $V_{\min}(0.60) = (1 - 0.60)^3 = 0.40^3 = 0.0640$. With $\ln(0.60) \approx -0.5108$, $V_{\mathrm{prod}}(0.60) \approx 0.4000 - 0.3065 - 0.0783 = 0.0152$. Contraction loss is $1 - 0.0152/0.0640 = 76.22\%$. Under 5 divisions, each at $0.85$, the product is $0.85^5 = 0.4437 < 0.60$, falsely diagnosing failure.

3. **Pre-Registered Falsifier Power and Severity Calculations:**
   Design a pre-registered empirical falsifier $F_{\mathrm{test}}$ to evaluate whether an institutional governance safeguard ($S_{\mathrm{gov}}$) prevents multitask proxy gaming. The substantive harm reduction cutoff is pre-registered at $\delta_{\min} = 0.20$, and paired differences have standard deviation $\sigma_d = 0.30$.
   - At significance level $\alpha = 0.05$ ($z_\alpha = 1.6449$) and statistical power $1 - \beta = 0.90$ ($z_{1-\beta} = 1.2816$), calculate the required effective sample size $n_{\mathrm{eff}}^{\mathrm{req}}$ and critical falsification threshold $d_{\mathrm{crit}}$.
   - If the observed sample of organizations has cluster design effect $\mathrm{DEFF} = 2.0$, how many raw paired organizations $n_{\mathrm{raw}}$ must be audited?
   - Suppose the trial audits $n_{\mathrm{eff}} = 20$ units and observes a mean harm reduction of $\bar{d} = 0.050$. Evaluate Deborah Mayo's post-data severity $\mathrm{SEV}(\Delta_{\mathrm{harm}} < \delta_{\min}; \bar{d})$. Is Stochology's safeguard refuted or corroborated?
   - *Guidance:* Ratio is $\frac{(1.6449 + 1.2816) \times 0.30}{0.20} = 4.38975 \implies n_{\mathrm{eff}}^{\mathrm{req}} = \lceil 4.38975^2 \rceil = 20$. $\mathrm{SE} = 0.30 / \sqrt{20} \approx 0.0671$, $d_{\mathrm{crit}} = 0.20 - 1.6449 \times 0.0671 = 0.0897$. Raw sample $n_{\mathrm{raw}} = 20 \times 2.0 = 40$. For $\bar{d} = 0.050$, $Z = (0.20 - 0.050) / 0.0671 \approx 2.236 \implies \mathrm{SEV} \approx 0.9873 \ge 0.95$. Verdict: `Refuted`.

4. **Scarf–Samuelson Welfare Optimization vs. Bureaucratic Hoarding:**
   An emergency food bank maintains an inventory buffer $R \ge 0$ (in thousands of rations). Precautionary security is modeled by logarithmic benefit $B(R) = \kappa \ln(1 + R)$ with $\kappa = 6.0$, while the marginal social cost of withholding rations from starving families today is $c_{\mathrm{urgent}} = 1.5$ per thousand rations.
   - Solve for the socially optimal precautionary buffer $R^*$.
   - Compute net social welfare $W(R^*) = B(R^*) - c_{\mathrm{urgent}} R^*$.
   - Suppose a risk-averse director demands a buffer of $R_{\mathrm{hoard}} = 30.0$. Compute $W(R_{\mathrm{hoard}})$.
   - How does Reflexive Brake 2 utilize this calculation to prevent bureaucratic hoarding?
   - *Guidance:* $\frac{dW}{dR} = \frac{6.0}{1 + R} - 1.5 = 0 \implies R^* = 3.0$ ($3{,}000$ rations). $W(R^*) = 6.0 \ln(4) - 4.5 \approx +3.818$. For hoarding, $W(30.0) = 6.0 \ln(31.0) - 45.0 \approx -24.396$. Brake 2 penalizes hoarding because $W(R^*) > W(R_{\mathrm{hoard}})$.
