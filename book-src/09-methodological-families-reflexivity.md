# Chapter 9 — Five Methodological Families, Process Tracing, and Reflexive Self-Application

Across Chapters 1 through 8, we constructed the complete [`StochologicalAgent`](../crates/stochology/src/engine.rs#L162-L173) by uniting the **Epistemic Margin**, the **Vital Margin**, and **Institutional Governance**.

In this final chapter (implemented in [`crates/stochology/src/synthesis.rs`](../crates/stochology/src/synthesis.rs)), we answer two culminating questions of the disciplinary treatise ([`notes/The Ethics of Aim.md`](../notes/The%20Ethics%20of%20Aim.md) §VI–§VIII):
1. **Methodological Generality:** How does Stochology govern not only quantitative simulation and econometrics, but also **qualitative, historical, archival, legal, and clinical inquiry** (*Process Tracing*)?
2. **Reflexive Self-Application:** What prevents Stochology itself from degenerating under Goodhart's Law when its three safeguards are turned into bureaucratic metrics, and under what exact conditions would Stochology's own foundational claims be **refuted**?

---

## 9.1 The Five Methodological Families of Stochology

Stochology is not restricted to a single statistical school; its Nuclear Triad and seven constitutive tensions organize **Five Methodological Families**:

| Family | Domain of Inquiry | Primary Failure Mode (`NaiveOptimizer`) | Stochological Safeguards (`crates/stochology`) |
| :---: | :--- | :--- | :--- |
| **1** | **Observational, Administrative & Survey Inquiry** | Confounding, selection paradox ($\rho\_{R,Y} \ne 0$), cluster echoes, uncounted strata | `MengBigDataDefect`, `ClusterDesignEffect`, `ManskiBounds`, `KishWeightAudit`, `CaptureRecaptureAudit` |
| **2** | **Experimental, Clinical & Sequential Trials** | Optional stopping ($p$-hacking), differential attrition, dependent censoring, agnotological low power | `SeverePowerAnalysis`, `EvidenceProcess` (*e-processes*), `LeeAttritionBounds`, `PetersonSurvivalBounds` |
| **3** | **Predictive, Algorithmic & Performative Modeling** | Miscalibration, regime drift, Goodhart proxy gaming, Chouldechova disparity | `CorpPavDecomposition`, `evaluate_invariant_predictors`, `PerformativeSystem`, `GoodhartAudit`, `ChouldechovaTrilemma` |
| **4** | **Physical, Ecological & Operational Systems** | Flaw of averages ($\Delta\_J$), power-law tails ($\alpha < 2$), Kingman queue collapse, Itô variance drain | `NonlinearSummary`, `HillTailEstimate`, `VitalSlackBuffer` ($\mathrm{TTS} \ge \mathrm{TTR}$), `EpistemicKellyAllocation`, `CanguilhemHollingBasin` |
| **5** | **Qualitative, Historical, Legal & Clinical Process Tracing** | Confirmation bias, ignoring alternative hypotheses ($\neg H$), confusing narrative coherence with evidential weight $V$ | `DiagnosticClue`, `ProcessTracingTestKind`, `ProcessTracingInference` |

---

## 9.2 Family 5 in Detail: Galenic *Sēmeîa*, Process Tracing, and Keynes's Weight $V$

In historical archival work, investigative journalism, legal fact-finding, and bedside clinical diagnosis, the investigator does not estimate a regression slope over $N = 50{,}000$ rows; the investigator traces a causal mechanism in a **single case** by evaluating qualitative clues (*sēmeîa* in Galen and Hippocrates; *Process Tracing* in Stephen Van Evera, 1997, and Andrew Bennett & Jeffrey Checkel, 2015).

In [`DiagnosticClue`](../crates/stochology/src/synthesis.rs#L98-L134) and [`ProcessTracingTestKind`](../crates/stochology/src/synthesis.rs#L84-L93), every qualitative clue $E$ is evaluated by two conditional probabilities—its **sensitivity** $\mathbb{P}(E \mid H)$ under the focal hypothesis $H$ and its **false-positive rate** $\mathbb{P}(E \mid \neg H)$ under the rival explanation $\neg H$—which classify the clue into Van Evera's four canonical tests:

1. **`StrawInTheWind`** ($\mathbb{P}(E \mid H) < 0.85$ and $\mathbb{P}(E \mid \neg H) > 0.10$): neither necessary nor sufficient; nudges the odds modestly.
2. **`HoopTest`** ($\mathbb{P}(E \mid H) \ge 0.85$ and $\mathbb{P}(E \mid \neg H) > 0.10$): a **necessary condition** ($\mathrm{LR}^- = \frac{1 - \mathbb{P}(E \mid H)}{1 - \mathbb{P}(E \mid \neg H)} \ll 1$). Passing a Hoop Test does not prove $H$, but **failing a Hoop Test refutes $H$**!
3. **`SmokingGun`** ($\mathbb{P}(E \mid H) < 0.85$ and $\mathbb{P}(E \mid \neg H) \le 0.10$): a **sufficient / pathognomonic sign** ($\mathrm{LR}^+ = \frac{\mathbb{P}(E \mid H)}{\mathbb{P}(E \mid \neg H)} \gg 1$). Observing a Smoking Gun strongly confirms $H$, though its absence does not refute $H$.
4. **`DoublyDecisive`** ($\mathbb{P}(E \mid H) \ge 0.85$ and $\mathbb{P}(E \mid \neg H) \le 0.10$): both necessary and sufficient.

In [`ProcessTracingInference::evaluate`](../crates/stochology/src/synthesis.rs#L156-L196), qualitative inference is valid only if:
- probabilities respect Cromwell's interior bounds;
- each clue updates both the Bayesian posterior odds (via Turing–Good decibans $W = 10\log\_{10}\mathrm{LR}$) **and** Keynes's evidential weight $V\_{\mathrm{post}} = \frac{K\_0 + \sum \Delta K}{K\_0 + \sum \Delta K + I\_0}$;
- **Popper–Mayo Refutability (`has_refutable_hoop_condition = true`):** the inquiry includes at least one `HoopTest` or `DoublyDecisive` clue whose absence could have falsified the hypothesis.

---

## 9.3 Reflexive Self-Application: The Three Internal Goodhart Brakes

What happens if an institution turns Stochology's own three axes into isolated bureaucratic targets? Left unchecked, each axis would suffer its own Goodhart perversion:
- **Perversion of the Epistemic Margin (*Skeptical Inflation*):** An analyst could achieve "100% epistemic coverage" lazily by reporting the trivial interval $[0, 1]$ or predicting the constant base rate $\bar{y}$ everywhere.
  - **Internal Brake 1 (CORP Resolution $\mathrm{RES}\_{\mathrm{CORP}} > 0$):** In [`ReflexiveBrakeAudit`](../crates/stochology/src/synthesis.rs#L203-L271), any lazy constant forecast has $\mathrm{RES}\_{\mathrm{CORP}} = 0.0$ and is rejected; epistemic intervals must earn discriminative resolution.
- **Perversion of the Vital Margin (*Regressive Bureaucratic Hoarding*):** An administrator could hoard an excessive reserve $R\_{\mathrm{hoard}} \gg R^\*$ while starving urgent present needs of the community.
  - **Internal Brake 2 (Scarf–Samuelson Marginal Opportunity Cost):** Net social welfare $W(R) = \kappa \ln(1 + R) - c\_{\mathrm{urgent}} R$ balances concave precautionary benefit against the marginal cost $c\_{\mathrm{urgent}}$ of unmet present need (`optimal_buffer_social_welfare > regressive_hoarding_welfare`).
- **Perversion of Institutional Governance (*Procedural Paralysis / Audit Theater*):** A committee could demand a $12$-shift bureaucratic review ($\tau\_{\mathrm{bureaucratic}} > \mathrm{TTS}$) while a physical crisis has a Time-to-Survive of $\mathrm{TTS} = 4$ shifts.
  - **Internal Brake 3 (Kairós Emergency Fast-Path $\tau\_{\mathrm{fast}} \le \mathrm{TTS}$):** Whenever $\mathrm{TTS} < \tau\_{\mathrm{bureaucratic}}$, the Wald–Sniedovich global Maximin fast-path pre-empts procedural delay (`emergency_fast_path_time <= emergency_tts`), deferring Stage-1 telic audit until the physical survival barrier is secured.

---

## 9.4 Empirical Falsifiers, Constitutive Normative Commitments, and Meta-Topological Openness

Because Stochology demands severe refutability, it separates contingent **empirical falsifiers** of its structural hypotheses from its **constitutive methodological-normative commitments** ([`notes/The Ethics of Aim.md`](../notes/The%20Ethics%20of%20Aim.md) §7.3):

1. **Empirical Falsifier 1 — Substitutability of the Triad in Open Dynamic Regimes:** Demonstrating across $N \ge 200$ out-of-sample trajectories ($T \ge 24$) with heavy-tailed shocks ($\alpha < 2$), non-random missingness ($\rho\_{R,Y} \ne 0$), and irreversible ruin barriers that an agent with one zeroed safeguard ($M\_{\mathrm{Leontief}} = 0$) achieves equal or lower ruin frequency ($\Delta p\_{\mathrm{ruin}} \le 0.05$) and convex social harm ($\le 1.10\times$) than an agent maintaining $M\_{\mathrm{Leontief}} > 0$ at severity $\mathrm{SEV} \ge 0.95$—tested in `CommunityReserveEnv` via **Collapse Propositions 1, 2, and 3**.
2. **Empirical Falsifier 2 — Sufficiency of Point-Certainty Sizing Under Heavy Traffic and Heavy Tails:** Demonstrating in empirical operational queues ($\rho \ge 0.85$, e.g., `vic_elec.csv`, `bank_calls.csv`) or power-law loss regimes ($\hat{\alpha} < 2$, e.g., `danish_fire.csv`) across $K \ge 50$ rolling out-of-sample windows that sizing capacity for the plug-in point evaluation $f(\hat{\mu}\_n)$ without Jensen convexity correction $\Delta\_J$ or Kingman slack achieves equal survival and cumulative deficit within $5\%$ of stochological buffering at severity $\mathrm{SEV} \ge 0.95$.
3. **Empirical Falsifier 3 — Benign Convergence of Unaudited Administrative Big Data:** Demonstrating across $M \ge 20$ paired observational-experimental benchmarks with non-zero selection correlation ($|\rho\_{R,Y}| \ge 0.01$) or cluster correlation ($\rho\_I \ge 0.02$, e.g., `cps_observational.csv` vs. `nsw_experimental.csv`) that increasing raw sample size $n \to N$ without Meng/Moulton/Manski adjustments yields nominal $95\%$ interval coverage of at least $90\%$ across at least $80\%$ of benchmarks.
4. **Empirical Falsifier 4 — Topological Redundancy or Incompleteness of the Seven Constitutive Tensions:** Demonstrating across a multi-domain corpus of $J \ge 50$ post-mortem institutional and algorithmic failures either (a) **redundancy** (that a single tension's diagnostic score is predicted with $R^2 \ge 0.85$ by the remaining six, collapsing the 7-tension architecture into $\le 6$), or (b) **incompleteness** (that $\ge 15\%$ of audited failures under uncertainty cannot be diagnosed by any combination of Tensions 1–7, forcing the addition of an eighth constitutive tension).
5. **Constitutive Methodological-Normative Commitment (Bitemporal Accountability via `TemporalVault`):** Separating ex-ante competence (*télos*, Stage 1) from ex-post outcome (*skopós*, Stage 2) via [`TemporalVault`](../crates/stochology/src/governance.rs#L376-L426) is an explicit normative and institutional design commitment grounded in Sosa (2007, 2015) and Baron & Hershey (1988), audited empirically by whether it reduces hindsight outcome bias and defensive risk-shifting relative to purely retrospective evaluation.
6. **Meta-Topological Openness (Boundary of Failure):** In genuinely novel ontological shocks where the state space $\mathcal{Y}$ itself ruptures beyond pre-registered bounds $[y\_{\min}, y\_{\max}]$, formal partial-identification bounds lose coverage; survival depends on the physical Vital Margin ($\mathrm{TTS} \ge \mathrm{TTR}$) and the unthrottled frontline algedonic channel ($S3^\*$) while the epistemic frame is reconstructed.

---

## 9.5 Chapter 9 Laboratory: Process Tracing, Reflexive Brakes, and the Three Collapse Propositions

Running Chapter 9 (`cargo run --release --bin stochology -- --chapter 9`) executes the complete synthesis and verifies all three Collapse Propositions on `CommunityReserveEnv`:

```text
=== Chapter 9: Methodological Families, Process Tracing & Reflexive Brakes ===
  Process Tracing (Family 5): prior=0.25 -> post=0.925 (+15.7 dB), Keynes_V: 0.20 -> 0.56, hoop_refutable=true
  Three Reflexive Goodhart Brakes: sharp_RES=0.250 vs lazy_RES=0.000 | optimal_welfare=2.55 > hoarding=-11.97 | fast_path=1.5 <= TTS=4.0
  Three Collapse Propositions Verified: P1=Proposition1BlindUndersizing, P2=Proposition2ImpotentLucidity, P3=Proposition3CynicalCapture -> Full=IntactTriad
```

Look at the final line of the laboratory (`Simulated`, `seed = 2026`, $T = 24$):
- **`StochologicalAgent::without_epistemic_margin()`** ($\neg\text{Epistemic Margin}$): diagnoses **`Proposition1BlindUndersizing`**;
- **`StochologicalAgent::epistemic_only()`** ($\neg\text{Vital Margin}$): diagnoses **`Proposition2ImpotentLucidity`**;
- **`StochologicalAgent::without_governance()`** ($\neg\text{Institutional Governance}$): diagnoses **`Proposition3CynicalCapture`**;
- **`StochologicalAgent::full()`** (all three safeguards active): achieves **`IntactTriad`**.

---

## Exercises

1. **Failing a Hoop Test:** In `run_chapter_9`, change `clues[0].clue_observed` (the Hoop Test with $\mathbb{P}(E \mid H) = 0.95, \mathbb{P}(E \mid \neg H) = 0.45$) from `true` to `false`. How many negative decibans does the absent Hoop clue contribute, and what happens to `pt.posterior_probability`?
2. **Interactive Exploration:** Open the [Interactive Laboratory (`lab/index.html`)](lab/index.html) and toggle each of the three axes of the Nuclear Triad on and off in real time to observe the reserve trajectory $R\_t$, utilization $\rho\_t$, and convex social harm $H\_t$ across all 24 shifts.

