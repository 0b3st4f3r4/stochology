# 20. The Regime of the Decision

> **Three tempos.** **Detachment:** set aside individuals and observations as isolated atoms. **Abstraction:** the world returns clustering, resolution limits, and fairness trade-offs to the decision (Decision Worlds, $L\_1 \to L\_3$, reading frame). **Return:** Moulton-Campbell $n\_{\mathrm{eff}}(p)$, $\Phi\_5$ resolution parity, and Chouldechova's impossibility. **Negative proof:** `gates -- --defect moulton`, `gates -- --defect phi5`, `gates -- --defect chouldechova`; Full track adds `gates -- --defect subgroup_miscalibration` (exit `1`).

## Five Hundred Fish in Two Tanks, and One Hundred Coin Tossers

In 1984, ecologist **Stuart Hurlbert** published a famous review of biological experiments that shocked the scientific community. Suppose a researcher wants to test whether a new water additive makes trout grow faster. She places $250$ trout into Tank A (with the additive) and $250$ trout into Tank B (without the additive). Six months later, she weighs all $500$ fish, runs a two-sample $t$-test with sample size $n = 500$ ($498$ degrees of freedom), and publishes a tiny $p$-value ($p = 0.0001$).

If Tank A happened to sit two meters closer to the window, had a slightly warmer heater, or held a single dominant fish that stressed the others, **all $250$ fish in Tank A shared that exact same tank environment.** Measuring $250$ fish inside one tank does not give you $250$ independent experiments on the additive; it gives you a very precise measurement of **one tank**. The true independent unit of the experiment is the **tank ($n\_{\text{true}} = 2$)**, not the fish ($n\_{\text{nominal}} = 500$). Hurlbert named this error **pseudoreplication**.

The same illusion happens every day in data science:
- In a hospital dataset of $50\text{,}000$ hourly vital-sign readings taken from $400$ patients, the hourly readings of the same patient are not independent; the patient (or the hospital ward) is the cluster.
- In `bank_calls.csv`, there are $n = 27\text{,}716$ five-minute arrival windows recorded across $164$ working days. If a promotional campaign or a Monday morning surge hits the bank, all $169$ five-minute windows of that day surge together. Treating $n = 27\text{,}716$ as the independent sample size understates the tail variance by **$28.26$-fold**.

Now consider a second paradox: **the Hot Hand and the Forgotten Null Model ($\Phi\_1, \Phi\_3$)**.
Ask $100$ people in an auditorium to toss a fair coin $20$ times each and write down their longest streak of consecutive Heads. Even though no one has "coin-tossing talent," in $100\%$ of simulated auditoriums someone will toss at least $6$ Heads in a row, and the longest streak in the room will average **$9.5$ consecutive Heads**. If a journalist interviews only the winner after the fact, tomorrow's headline announces a miracle. Even more subtly, when Joshua Miller and Adam Sanjurjo (2018) wrote down the exact finite-sample generative null model for basketball shooting streaks, they discovered that in $3$ tosses of a fair coin, the average proportion of Heads *immediately following a Head* across sequences is not $\frac{1}{2} = 0.50$, but **$\frac{5}{12} \approx 0.4167$**. For thirty years, psychologists had compared basketball players against $0.50$ instead of simulating the actual null model on a computer.

To prevent these errors from ever entering production, Stochology enforces **Five Boundary Invariants ($\Phi\_1$ to $\Phi\_5$)** as a strict boolean product $\Phi = \prod\_{j=1}^5 \Phi\_j$: if even one invariant fails, the gate drops to $0$ and blocks the claim.

<figure class="stoch-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="1140" height="230" viewBox="0 0 1140 230" fill="none" role="img" aria-labelledby="title-fig-07-invariants">
  <title id="title-fig-07-invariants">The Five Boundary Invariants (Phi_1 to Phi_5) as a Multiplicative Gate Product</title>
  <defs>
  <style>text { font-family: "Inter", system-ui, -apple-system, "Segoe UI", Roboto, sans-serif; } .mono { font-family: "JetBrains Mono", ui-monospace, "Cascadia Code", monospace; }</style>
  </defs>
  <rect x="1" y="1" width="1138" height="228" rx="14" fill="var(--canvas)" stroke="var(--border)" stroke-width="1.5"/>
  <rect x="16" y="24" width="206" height="182" rx="11" fill="var(--elevated)" stroke="var(--sky)" stroke-width="2"/>
  <text x="119" y="54" text-anchor="middle" fill="var(--sky)" class="mono" font-size="12" font-weight="800">INVARIANT Φ₁</text>
  <text x="119" y="82" text-anchor="middle" fill="var(--ink)" font-size="14.5" font-weight="800">Simulability</text>
  <text x="119" y="114" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11">Pre-Registered Seed</text>
  <text x="119" y="138" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11">SplitMix64 RNG</text>
  <text x="119" y="162" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11">Generative Null H₀</text>
  <rect x="240" y="24" width="206" height="182" rx="11" fill="var(--elevated)" stroke="var(--violet)" stroke-width="2"/>
  <text x="343" y="54" text-anchor="middle" fill="var(--violet)" class="mono" font-size="12" font-weight="800">INVARIANT Φ₂</text>
  <text x="343" y="82" text-anchor="middle" fill="var(--ink)" font-size="14.5" font-weight="800">Independent Unit</text>
  <text x="343" y="114" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11">m̃ = m̄(1 + CV²)</text>
  <text x="343" y="138" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11">Exceedance ρ_I(p)</text>
  <text x="343" y="162" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11">n_eff(p) = n / DEFF</text>
  <rect x="464" y="24" width="212" height="182" rx="11" fill="var(--elevated)" stroke="var(--emerald)" stroke-width="2"/>
  <text x="570" y="54" text-anchor="middle" fill="var(--emerald)" class="mono" font-size="12" font-weight="800">INVARIANT Φ₃</text>
  <text x="570" y="82" text-anchor="middle" fill="var(--ink)" font-size="14.5" font-weight="800">Severe Test &amp; Seal</text>
  <text x="570" y="114" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11">Rubin · Meng · Manski</text>
  <text x="570" y="138" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11">Pearl Rung 1 / 2 / 3</text>
  <text x="570" y="162" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11">SHA-256 Pre-Reg Seal</text>
  <rect x="694" y="24" width="206" height="182" rx="11" fill="var(--elevated)" stroke="var(--amber)" stroke-width="2"/>
  <text x="797" y="54" text-anchor="middle" fill="var(--amber)" class="mono" font-size="12" font-weight="800">INVARIANT Φ₄</text>
  <text x="797" y="82" text-anchor="middle" fill="var(--ink)" font-size="14.5" font-weight="800">Fairness &amp; Exit</text>
  <text x="797" y="114" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11">p* = C / (η L)</text>
  <text x="797" y="138" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11">Chouldechova + KMR</text>
  <text x="797" y="162" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11">Numeric Exit Clause</text>
  <rect x="918" y="24" width="206" height="182" rx="11" fill="var(--elevated)" stroke="var(--rose)" stroke-width="2"/>
  <text x="1021" y="54" text-anchor="middle" fill="var(--rose)" class="mono" font-size="12" font-weight="800">INVARIANT Φ₅</text>
  <text x="1021" y="82" text-anchor="middle" fill="var(--ink)" font-size="14.5" font-weight="800">Resolution Match</text>
  <text x="1021" y="114" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11">R_data ≥ R_decision</text>
  <text x="1021" y="138" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11">No R₀ annual/monthly</text>
  <text x="1021" y="162" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11">for shift/daily acts</text>
  </svg>
  <figcaption><strong>Figure 20.1 — The Five Boundary Invariants (Φ = Φ₁ · Φ₂ · Φ₃ · Φ₄ · Φ₅)</strong></figcaption>
</figure>

---

## Overview of the Five Boundary Invariants

| Invariant | Name | Formal Requirement | Rust Implementation |
| --- | --- | --- | --- |
| **$\Phi\_1$** | **Simulability** | Pre-registered seed (`ProtocolProfile::seed`, `SplitMix64`) and an executable generative null model before any parameter fitting. | `stochology_core::rng::Rng` & `card/` |
| **$\Phi\_2$** | **Independent Unit & Exceedance Design Effect** | Margin computed over the true independent unit; Moulton-Campbell unequal-cluster correction $\tilde{m} = \bar{m}(1 + \mathrm{CV}^2(m\_j))$, threshold exceedance ICC $\rho\_I(p)$, and $n\_{\mathrm{eff}}(p) = \frac{n}{1 + (\tilde{m}-1)\rho\_I(p)}$. | `stochology_core::invariants::{ExceedanceDesignEffect, OperationalWeightingGap}` |
| **$\Phi\_3$** | **Severe Test, Missingness & Pre-Registration** | Explicit Rubin missingness (`MCAR`, `MAR`, `MNAR`), Meng's Identity ($\rho\_{R,Y}$), Manski bounds, 64-hex SHA-256 pre-registration seal, and Pearl's Ladder rung. | `stochology_core::{layers, sha256, card}` |
| **$\Phi\_4$** | **Scope, Fairness & Automatic Exit** | Economic threshold $p^{\ast} = \frac{C}{\eta L}$, Chouldechova's algebraic fairness audit ($\mathrm{FPR}\_A / \mathrm{FPR}\_B$), declared scope, and machine-actionable numeric exit clause. | `stochology_core::invariants::ChouldechovaAudit` & `card/` |
| **$\Phi\_5$** | **Question Resolution Alignment** | Ban on answering a fine-grained operational decision (5-min shift, 6-hr synoptic track) with coarse annual or monthly aggregates ($R\_{\text{data}} \ge R\_{\text{decision}}$). | `stochology_core::invariants::verify_resolution_phi5` |

---

## 20.1 Invariant $\Phi\_2$ — The Independent Unit, Moulton-Campbell Correction, and Tail Exceedance

### Step-by-Step Derivation: From Intraclass Correlation to Tail Exceedance $n\_{\mathrm{eff}}(p)$

Suppose our dataset contains $n = \sum\_{j=1}^C m\_j$ elementary observations grouped into $C$ clusters (for instance, $C = 164$ days in `bank_calls.csv`, each containing $m\_j$ five-minute intervals, or $C$ hospitals each containing $m\_j$ patients).

Inside cluster $j$, any two observations share a common cluster shock, inducing an **intraclass correlation coefficient (ICC)** $\rho \in [0, 1]$.
1. **Equal Cluster Sizes (Leslie Kish's Classic Formula, 1965):**
   If every cluster has the exact same size $\bar{m} = n / C$, the variance of the sample mean is inflated relative to simple random sampling by the **Design Effect ($\mathrm{DEFF}$)**:
   $$\mathrm{DEFF}\_{\text{Kish}} = 1 + (\bar{m} - 1)\rho$$

2. **Unequal Cluster Sizes (Moulton 1986 & Campbell et al. 2001):**
   In the real world, clusters are rarely equal in size: one hospital treats $400$ patients while a rural clinic treats $15$; one turbulent storm system has $80$ track points while a short storm has $12$. When cluster sizes $m\_j$ vary with coefficient of variation $\mathrm{CV}(m\_j) = \frac{\mathrm{sd}(m\_j)}{\bar{m}}$, the larger clusters dominate the sum of squared errors. **Brent Moulton and Mike Campbell** proved that the average cluster size $\bar{m}$ in the design effect must be replaced by the **effective cluster size $\tilde{m}$**:

   $$\tilde{m} = \frac{\sum\_{j=1}^C m\_j^2}{\sum\_{j=1}^C m\_j} = \bar{m}\Bigl(1 + \mathrm{CV}^2(m\_j)\Bigr) \ge \bar{m}$$

   When cluster sizes are skewed, say $\bar{m} = 59.59$ and $\mathrm{CV}(m\_j) \approx 1.0$, then $\tilde{m} = 119.05$, **doubling the cluster multiplier before we even multiply by $\rho$.**

3. **Threshold Exceedance ICC ($\rho\_I(p)$) in the Tails:**
   Standard textbooks compute $\rho$ on the *mean* of $Y$. Yet operational decisions (staffing for peak call surges, evacuating for category-2 hurricane winds, holding flights during severe delays) depend on **tail quantiles $q\_p$** (such as $p = 0.90$ or $p = 0.95$).
   Even when everyday fluctuations have weak correlation across a day, **extreme tail events cluster fiercely inside surge days.** Therefore, `ExceedanceDesignEffect` (`crates/stochology-core/src/invariants.rs`) defines the binary tail-exceedance indicator $I\_{ij}(p) = \mathbb{I}\lbrace Y\_{ij} > q\_p \rbrace$ (which has variance $p(1-p)$) and computes the **ANOVA Threshold Exceedance ICC ($\rho\_I(p)$)** across the $C$ clusters:

   $$\mathrm{DEFF}\_{\text{exceedance}}(p) = 1 + (\tilde{m} - 1) \rho\_I(p), \qquad n\_{\mathrm{eff}}(p) = \frac{n}{1 + (\tilde{m} - 1) \rho\_I(p)}$$

   Let us check the numbers on `bank_calls.csv` ($n = 27\text{,}716$ five-minute windows across $C = 164$ days, $\tilde{m} = 169.00$):
   - At the $90\text{th}$ percentile ($p = 0.90$), the tail-exceedance ICC is **$\rho\_I(0.90) = 0.1623$**.
   - The resulting design effect is **$\mathrm{DEFF}(0.90) = 1 + (169 - 1) \times 0.1623 = 28.26$**.
   - Consequently, the true effective sample size for tail staffing decisions is not $27\text{,}716$, but **$n\_{\mathrm{eff}}(0.90) = \frac{27\text{,}716}{28.26} = 980.7$**. Any confidence interval computed on $n = 27\text{,}716$ is more than five times too narrow (`--defect moulton`).
   - Furthermore, `ExceedanceDesignEffect::profile_across_quantiles` profiles $n\_{\mathrm{eff}}(p)$ across multiple quantiles simultaneously ($p \in \lbrace 0.50, 0.90, 0.99 \rbrace$), revealing on `bank_calls.csv` how effective sample size collapses from the median ($n\_{\mathrm{eff}}(0.50) = 9\text{,}239$) into the surge tail ($n\_{\mathrm{eff}}(0.90) = 981$, $n\_{\mathrm{eff}}(0.99) = 1\text{,}308$).

### The Operational Weighting Gap (The Inspection Paradox)

Invariant $\Phi\_2$ also audits a cousin of cluster correlation (`OperationalWeightingGap`): **who are you averaging over: the calendar day or the arriving customer?**
Recall the hypothetical municipal health clinic from Chapter 1 (invented figures): on calm days, $5$ patients/hour arrive and wait $3.6\text{ min}$; on surge days, $11$ patients/hour arrive and wait $55\text{ min}$.
- Averaged **by day**, calm and surge days weigh $50\%$ each.
- Averaged **by patient**, $\frac{11}{5 + 11} = 68.75\%$ of all human beings arrive on the surge day.
This is the **Inspection Paradox** (length-biased sampling): sampling by arriving user always weights high-volume clusters by $m\_j / \bar{m}$, opening a positive gap whenever cluster outcome $Y\_j$ correlates with cluster volume $m\_j$.

### Filter 2 — Cluster Independence ($\Phi\_2$) and Causal Hygiene

Filter 2 audits Bridge $L\_1 \to L\_2$ and Lens 4:
1. Using the Moulton-Campbell unequal-cluster formula $\tilde{m} = \bar{m}(1 + \mathrm{CV}^2)$ and the tail-exceedance intraclass correlation $\rho\_I(p)$, is the effective sample size large enough to support inference ($n\_{\mathrm{eff}}(p) = \frac{n}{1 + (\tilde{m}-1)\rho\_I(p)} \ge n\_{\min}$)? This blocks the ***Homo clausus* pseudoreplication fallacy** (`ExceedanceDesignEffect::benchmark_pseudoreplication_500_to_86` in `crates/stochology-core/src/invariants.rs`, Chapter 22), where $n = 500$ grouped observations with $\tilde{m} = 25.0$ and tail $\rho\_I(0.90) = 0.2005$ suffer a design effect of **$\mathrm{DEFF} = 5.81\times$**, collapsing the true independent sample size to only **$n\_{\mathrm{eff}}(0.90) = 86.0$ degrees of freedom**.
2. Has the analyst mapped the **Markov Blanket** of the target variable (its direct causal parents, direct causal children, and co-parents), and verified that **zero conditioning occurs on a Berkson collider** (`conditioned_on_collider == false`)?

---

## 20.2 Invariant $\Phi\_4$ — Scope, Algorithmic Fairness (Chouldechova's Trilemma), and the Exit Clause

### Step-by-Step Derivation of Alexandra Chouldechova's (2017) Impossibility Identity

Whenever a model triggers consequential actions across demographic, geographic, or institutional groups $g \in \lbrace A, B \rbrace$ (such as pretrial detention, loan approval, medical triage, or storm evacuation), committees naturally ask that the algorithm satisfy three fairness properties simultaneously:
1. **Predictive Parity (Equal $\mathrm{PPV}$):** When the model raises an alarm ($S = 1$), the probability that the person truly has the outcome ($Y = 1$) is the same in both groups: $\mathrm{PPV}\_A = \mathrm{PPV}\_B$, where $\mathrm{PPV}\_g = P(Y = 1 \mid S = 1, G = g)$.
2. **Equal False Negative Rate (Equal $\mathrm{FNR}$):** True cases have the same probability of being missed in both groups: $\mathrm{FNR}\_A = \mathrm{FNR}\_B$, where $\mathrm{FNR}\_g = P(S = 0 \mid Y = 1, G = g)$.
3. **Equal False Positive Rate (Equal $\mathrm{FPR}$):** Innocent or event-free individuals have the same probability of suffering a false alarm in both groups: $\mathrm{FPR}\_A = \mathrm{FPR}\_B$, where $\mathrm{FPR}\_g = P(S = 1 \mid Y = 0, G = g)$.

In 2017, statistician **Alexandra Chouldechova** (and independently Jon Kleinberg, Sendhil Mullainathan, and Manish Raghavan) proved that **no classifier can satisfy all three conditions simultaneously whenever the two groups have different base rates ($p\_A \neq p\_B$), except in degenerate cases.**

Let us derive **Chouldechova's Identity** in three lines directly from Bayes' rule. For any group $g$ with base rate $p\_g = P(Y = 1 \mid G = g) \in (0, 1)$, write Bayes' rule for $\mathrm{PPV}\_g$:

$$\mathrm{PPV}\_g = \frac{p\_g (1 - \mathrm{FNR}\_g)}{p\_g (1 - \mathrm{FNR}\_g) + (1 - p\_g) \mathrm{FPR}\_g}$$

Multiply both sides by the denominator and solve for $\mathrm{FPR}\_g$:

$$\mathrm{FPR}\_g = \frac{p\_g}{1 - p\_g} \frac{1 - \mathrm{PPV}\_g}{\mathrm{PPV}\_g} (1 - \mathrm{FNR}\_g)$$

In that algebraic equation (`ChouldechovaAudit` in `crates/stochology-core/src/invariants.rs`):
- Suppose you force Predictive Parity ($\mathrm{PPV}\_A = \mathrm{PPV}\_B$) and Equal Opportunity ($\mathrm{FNR}\_A = \mathrm{FNR}\_B$).
- Then the second and third factors on the right-hand side are identical for Group A and Group B.
- Dividing $\mathrm{FPR}\_A$ by $\mathrm{FPR}\_B$ immediately yields:

$$\frac{\mathrm{FPR}\_A}{\mathrm{FPR}\_B} = \frac{p\_A / (1 - p\_A)}{p\_B / (1 - p\_B)}$$

The ratio of false positive rates is locked algebraically to the **ratio of prior base-rate odds**.
In `toronto_arrests.csv` ($n = 5\text{,}226$ police arrest records), two demographic strata exhibit empirical base rates $p\_A = 0.3926$ and $p\_B = 0.1119$. If a classifier equalizes $\mathrm{PPV} = 0.65$ and $\mathrm{FNR} = 0.25$ across both groups, Chouldechova's identity forces $\mathrm{FPR}\_A = 0.2610$ versus $\mathrm{FPR}\_B = 0.0509$, so innocent members of Group A suffer false alarms at **$5.13\times$ the rate of Group B**.

Complementing Chouldechova's binary threshold identity, **Jon Kleinberg, Sendhil Mullainathan, and Manish Raghavan (2017)** proved a **continuous risk-score impossibility theorem** (`ChouldechovaAudit::kmr_continuous_impossibility_gap`): for any perfectly calibrated continuous risk score $S \in (0, 1)$ ($\mathbb{E}[Y \mid S = s, G = g] = s$), the Law of Iterated Expectations locks the mean score of group $g$ to $\mathbb{E}[S \mid G = g] = p\_g$, and if a non-perfect score additionally equalizes average score among negative cases ($\mathbb{E}[S \mid Y = 0, G = g]$) and positive cases ($\mathbb{E}[S \mid Y = 1, G = g]$), the Bernoulli variance ceiling $p\_g(1 - p\_g)$ forces an irreducible variance gap (`KMR_var_gap = 0.0960` on `toronto_arrests.csv`) whenever $p\_A \neq p\_B$ and $p\_A + p\_B \neq 1$.

Because no algorithm can repeal Bayes' theorem, Invariant $\Phi\_4$ requires every Epistemological Card to:
1. Audit Chouldechova's identity explicitly and declare **`who_pays_for_error`** (naming the human or institutional bearer of False Positives and False Negatives)—grounding algorithmic deployment in **T. M. Scanlon's (1998) contractualism**, **Sven Ove Hansson's (2013) ethics of risk**, and **Ernest Sosa's (2007, 2015) *fully apt agency*** (Tension 7, [Chapter 23](23-stochary-conjectural-cybernetics.md)), which forbid using mathematical formalism as a technocratic alibi to impose uncompensated false-positive or false-negative burdens on vulnerable populations;
2. Declare an explicit **`scope_statement`** (where the model is forbidden from being applied);
3. Declare a machine-actionable numeric **`exit_clause`** (for instance: *"Immediately suspend automated shift scheduling if $\mathrm{REL}\_{\mathrm{CORP}} > 0.020$ or $\mathrm{RES}\_{\mathrm{CORP}} < 0.025$ over a 10-business-day rolling window"*), operationalizing Sosa's meta-competence of knowing when uncertainty or subgroup disparity requires lowering the bow.

---

### 20.2.1 Calibration Inside Every Subgroup (`MulticalibrationAudit`)

Chouldechova's trilemma is about error rates. A forecast can also be calibrated on average and miscalibrated inside every group that matters, because the groups' errors cancel. Multicalibration (Hébert-Johnson, Kim, Reingold and Rothblum 2018, *ICML*, informative) asks that $|\mathbb{E}[Y - p \mid S, v]| \le \alpha$ for every declared subgroup $S$ and forecast bin $v$. The audit bounds each cell's gap with Hoeffding's inequality at a Bonferroni level and returns one of three verdicts: `Violated` (some gap is above $\alpha$ with confidence), `Certified` (every gap is below $\alpha$ with confidence), or `Inconclusive`. "No violation found" is never read as "fair".

**Measured** on `toronto_arrests.csv`: a release predictor that never sees `colour`, fitted on odd rows and audited on even rows, over colour × sex × employment × citizenship:

~~~text
  [G4] A: multicalibration on toronto_arrests.csv: 59 cells, 0 violations, verdict Inconclusive (largest confidence radius 0.360, tolerance 0.05)
~~~

Cells hold 30 to 523 arrests. The smallest has a confidence radius of $0.36$, and certifying a gap below $0.05$ at this Bonferroni level would need about 1,550 units per cell, so the audit cannot resolve gaps of the tolerance's size, and it says so. On a **simulated** predictor (under a pre-registered seed) whose subgroup errors cancel exactly, the global CORP check sees nothing and the subgroup audit blocks:

~~~text
NEGATIVE PROOF [subgroup_miscalibration] BLOCKED SUCCESSFULLY: global REL_CORP=0.0009 hides 10 multicalibration violations (simulated, seed 2026)
~~~

## 20.3 Invariant $\Phi\_5$ — Question Resolution Alignment

Finally, **Invariant $\Phi\_5$ (`verify_resolution_phi5`)** guards the temporal and spatial scale of the data against the scale of the action.

Suppose a hospital director needs to staff nurses for **5-minute arrival surges** on Tuesday mornings, or an emergency manager needs to issue **6-hour hurricane evacuation advisories**, and an analyst hands them a regression fitted on **annual or monthly averages** ($R\_0$). Monthly aggregation averages out the intra-day spikes that overload the queue.

In `crates/stochology-core/src/invariants.rs`, every resolution level is mapped to an ordered scale:
$$\text{Sub-Hourly / 5-Min } (4) > \text{Hourly / 6-Hr Synoptic / Shift } (3) > \text{Daily } (2) > \text{Weekly / Monthly / Annual } (1)$$

Invariant $\Phi\_5$ enforces:

$$R\_{\text{data}} \ge R\_{\text{decision}}$$

Any attempt to answer a shift, hourly, or daily operational question with coarse monthly or annual aggregates is immediately blocked by Gate `G5` (`--defect phi5`).

<figure class="stoch-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="1120" height="300" viewBox="0 0 1120 300" fill="none" role="img" aria-labelledby="title-fig-16-2">
    <title id="title-fig-16-2">Moulton-Campbell Tail Sample-Size Collapse and Chouldechova Base-Rate Fairness Frontier</title>
    <defs>
      <style>text { font-family: "Inter", system-ui, -apple-system, "Segoe UI", Roboto, sans-serif; } .mono { font-family: "JetBrains Mono", ui-monospace, "Cascadia Code", monospace; }</style>
    </defs>
    <rect x="1" y="1" width="1118" height="298" rx="14" fill="var(--canvas)" stroke="var(--border)" stroke-width="1.5"/>
    <!-- Left panel: Moulton-Campbell n_eff(p) profile on bank_calls.csv -->
    <rect x="20" y="20" width="530" height="260" rx="12" fill="var(--elevated)" stroke="var(--sky)" stroke-width="2"/>
    <text x="40" y="48" fill="var(--sky)" class="mono" font-size="11.5" font-weight="800">Φ₂ · MOULTON-CAMPBELL TAIL COLLAPSE (bank_calls.csv)</text>
    <line x1="65" y1="230" x2="515" y2="230" stroke="var(--border)" stroke-width="1.5"/>
    <line x1="65" y1="70" x2="65" y2="230" stroke="var(--border)" stroke-width="1.5"/>
    <!-- Bars for Nominal n=27716, q50 n_eff=9239, q99 n_eff=1308, q90 n_eff=981 -->
    <rect x="95" y="78" width="72" height="152" rx="5" fill="var(--muted)" fill-opacity="0.35" stroke="var(--muted)" stroke-width="1.6"/>
    <text x="131" y="70" text-anchor="middle" fill="var(--ink)" class="mono" font-size="11" font-weight="700">n = 27,716</text>
    <text x="131" y="248" text-anchor="middle" fill="var(--muted)" class="mono" font-size="10.5">Nominal IID</text>
    <rect x="205" y="176" width="72" height="54" rx="5" fill="var(--sky)" fill-opacity="0.30" stroke="var(--sky)" stroke-width="1.8"/>
    <text x="241" y="168" text-anchor="middle" fill="var(--sky)" class="mono" font-size="11" font-weight="700">9,239</text>
    <text x="241" y="248" text-anchor="middle" fill="var(--muted)" class="mono" font-size="10.5">n_eff(q₅₀)</text>
    <rect x="315" y="220" width="72" height="10" rx="3" fill="var(--rose)" fill-opacity="0.45" stroke="var(--rose)" stroke-width="2"/>
    <text x="351" y="211" text-anchor="middle" fill="var(--rose)" class="mono" font-size="11" font-weight="800">980.7</text>
    <text x="351" y="248" text-anchor="middle" fill="var(--rose)" class="mono" font-size="10.5" font-weight="700">n_eff(q₉₀)</text>
    <rect x="425" y="218" width="72" height="12" rx="3" fill="var(--amber)" fill-opacity="0.40" stroke="var(--amber)" stroke-width="1.8"/>
    <text x="461" y="209" text-anchor="middle" fill="var(--amber)" class="mono" font-size="11" font-weight="700">1,308</text>
    <text x="461" y="248" text-anchor="middle" fill="var(--muted)" class="mono" font-size="10.5">n_eff(q₉₉)</text>
    <text x="285" y="270" text-anchor="middle" fill="var(--muted)" class="mono" font-size="10.5">DEFF(0.90) = 28.26× variance inflation across 164 business days</text>
    <!-- Right panel: Chouldechova & KMR Fairness Frontier on toronto_arrests.csv -->
    <rect x="570" y="20" width="530" height="260" rx="12" fill="var(--elevated)" stroke="var(--violet)" stroke-width="2"/>
    <text x="590" y="48" fill="var(--violet)" class="mono" font-size="11.5" font-weight="800">Φ₄ · CHOULDECHOVA &amp; KMR FRONTIER (toronto_arrests.csv)</text>
    <line x1="620" y1="230" x2="1070" y2="230" stroke="var(--border)" stroke-width="1.5"/>
    <line x1="620" y1="70" x2="620" y2="230" stroke="var(--border)" stroke-width="1.5"/>
    <!-- Hyperbolic curve of FPR_A as a function of PPV under fixed p_A=0.3926, p_B=0.1119 -->
    <path d="M 640 84 Q 760 155 1050 218" stroke="var(--rose)" stroke-width="2.6" fill="none"/>
    <path d="M 640 192 Q 760 212 1050 227" stroke="var(--emerald)" stroke-width="2.4" fill="none"/>
    <circle cx="810" cy="148" r="5.5" fill="var(--rose)"/>
    <circle cx="810" cy="214" r="5.5" fill="var(--emerald)"/>
    <line x1="810" y1="148" x2="810" y2="214" stroke="var(--amber)" stroke-width="2" stroke-dasharray="4 3"/>
    <text x="822" y="146" fill="var(--rose)" class="mono" font-size="11" font-weight="700">Group A: FPR_A = 0.2610 (p_A = 0.3926)</text>
    <text x="822" y="212" fill="var(--emerald)" class="mono" font-size="11" font-weight="700">Group B: FPR_B = 0.0509 (p_B = 0.1119)</text>
    <text x="822" y="182" fill="var(--amber)" class="mono" font-size="11" font-weight="800">Locked Ratio = 5.13× | KMR ΔVar = 0.0960</text>
    <text x="845" y="268" text-anchor="middle" fill="var(--muted)" class="mono" font-size="10.5">Equalizing PPV = 0.65 &amp; FNR = 0.25 locks FPR_A / FPR_B to base-rate odds</text>
  </svg>
  <figcaption><strong>Figure 20.2 — Quantitative Audit of Invariants Φ₂ (Moulton-Campbell Tail Design Effect) and Φ₄ (Chouldechova &amp; KMR Impossibility Frontier)</strong></figcaption>
</figure>

---

## 20.4 Knightian Uncertainty: Info-Gap Decision Theory and Sniedovich's Safeguard

### Foundations: Decision Criteria When Probabilities Cannot Be Trusted

What should an agent ($L\_3$) do when operating in the right-hand column of the Stirling Matrix, where probability distributions over future states of the world $u \in \mathcal{U}$ do not exist or cannot be trusted?

Classical decision theory offers four rules, illustrated by **David van Dantzig (1956)** after the 1953 North Sea flood prompted the Netherlands to redesign the height of its coastal dikes:

| Classical Criterion | Mathematical Rule (Reward $R(q, u)$) | Epistemological Posture |
| --- | --- | --- |
| **1. Abraham Wald (Maximin)** | $\max\_q \min\_{u \in \mathcal{U}} R(q, u)$ | **Maximum Pessimism:** Judge each decision $q$ by its absolute worst possible outcome across the entire universe of states $\mathcal{U}$. |
| **2. Leonard Savage (Minimax Regret)** | $\min\_q \max\_{u \in \mathcal{U}} \bigl[\max\_{q'} R(q', u) - R(q, u)\bigr]$ | **Regret Avoidance:** Minimize the worst possible hindsight difference between what you did and what you *wish* you had done had you known $u$. |
| **3. Pierre-Simon Laplace (Insufficient Reason)** | $\max\_q \frac{1}{|\mathcal{U}|}\sum\_{u \in \mathcal{U}} R(q, u)$ | **Uniform Ignorance:** Treat all listed states as equally probable. |
| **4. Leonid Hurwicz ($\lambda$-Optimism)** | $\max\_q \bigl[\lambda \max\_u R(q, u) + (1-\lambda)\min\_u R(q, u)\bigr]$ | **Declared Compromise:** Weight the best and worst cases by a declared optimism index $\lambda \in [0, 1]$. |

### Yakov Ben-Haim's Info-Gap Decision Theory

In engineering design and operational resilience, **Yakov Ben-Haim** proposed **Info-Gap Decision Theory** as a way to navigate severe uncertainty when you have a nominal (best-guess) model $\tilde{u}$ that you know may be wrong, and a critical survival requirement $r\_c$ (the minimum reward $R(q, u) \ge r\_c$ the system needs to avoid disaster).

Instead of fixing a single uncertainty set, Info-Gap surrounds the nominal model $\tilde{u}$ with a nested family of uncertainty balls $\mathcal{U}(\alpha, \tilde{u})$ indexed by an **uncertainty horizon $\alpha \ge 0$**:

$$\mathcal{U}(\alpha, \tilde{u}) = \bigl\lbrace   u : \lVert u - \tilde{u} \rVert \le \alpha  \bigr \rbrace$$

When $\alpha = 0$, the ball contains only the nominal guess $\tilde{u}$. As $\alpha$ grows, the ball expands to allow larger and larger departures from $\tilde{u}$. Info-Gap then defines two horizons for each candidate decision $q$:

1. **The Robustness Horizon ($\hat{\alpha}(q, r\_c)$, Immunity to Failure):**
   $$\hat{\alpha}(q, r\_c) = \max\left\lbrace  \alpha \ge 0 : \min\_{u \in \mathcal{U}(\alpha, \tilde{u})} R(q, u) \ge r\_c  \right\rbrace$$
   In plain words: *How wrong can our nominal model $\tilde{u}$ be (how large can $\alpha$ get) before decision $q$ risks violating the survival threshold $r\_c$?* A larger $\hat{\alpha}$ means more robustness.

2. **The Opportuneness Horizon ($\hat{\beta}(q, r\_w)$, Propensity for Windfall):**
   $$\hat{\beta}(q, r\_w) = \min\left\lbrace  \alpha \ge 0 : \max\_{u \in \mathcal{U}(\alpha, \tilde{u})} R(q, u) \ge r\_w  \right\rbrace$$
   In plain words: *What is the smallest departure $\alpha$ from the nominal model needed to make an ambitious windfall reward $r\_w > r\_c$ possible?* A smaller $\hat{\beta}$ means windfall requires less luck.

A central theorem of Info-Gap is the **Zero-ing Property**: if you demand a critical reward $r\_c$ equal to what the nominal model predicts ($r\_c = R(q, \tilde{u})$), your robustness horizon is identically zero ($\hat{\alpha}(q, R(q, \tilde{u})) = 0$). **Optimizing for the nominal model buys zero robustness against model error.** To gain positive robustness ($\hat{\alpha} > 0$), you must step back from nominal optimality and *satisfice*.

### Moshe Sniedovich's Critique ("No-Man's Land") and the Global Wald Safeguard

In a series of mathematical papers (2007–2012), operations researcher **Moshe Sniedovich** identified a structural flaw in unguarded Info-Gap analysis. Because $\hat{\alpha}(q, r\_c)$ expands outward from a single local point $\tilde{u}$ and stops looking the instant a curve crosses $r\_c$, **Info-Gap is blind to what happens at $\alpha > \hat{\alpha}(q, r\_c)$**, a region Sniedovich called **"No-Man's Land."**

Raw Info-Gap can easily recommend a fragile option:
- Suppose **Option A (Fragile Turkey)** performs well near the nominal guess $\tilde{u}$ and stays above $r\_c = 100$ up to $\hat{\alpha}\_A = 1.60$, but at $\alpha = 2.0$ its performance plunges off a cliff to $-450$ (total systemic ruin).
- Suppose **Option B (Robust Engineering)** has a slightly smaller local crossing or a modest trade-off near $\tilde{u}$, or stays above $r\_c$ up to $\hat{\alpha}\_B = 3.50$, and even under the worst conceivable state in the entire physical universe $\mathcal{U}\_{\text{global}}$, its reward never drops below $+45.0$.

If $\tilde{u}$ is severely misspecified (the definition of Knightian uncertainty), reality may easily lie outside $\mathcal{U}(\hat{\alpha}, \tilde{u})$. An algorithm that checks only local robustness around a wrong guess $\tilde{u}$ is performing a **local maximin** around a fiction while ignoring global catastrophe.

To prevent this failure, `InfoGapOption` (`crates/stochology-core/src/uncertainty.rs`) enforces **Sniedovich's Global Maximin Safeguard**:

<figure class="stoch-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="1100" height="400" viewBox="0 0 1100 400" fill="none" role="img" aria-labelledby="title-fig-02-infogap">
  <title id="title-fig-02-infogap">Info-Gap Robustness Horizon versus Sniedovich's No-Man's Land and the Global Wald Maximin Safeguard</title>
  <defs>
  <marker id="axis-arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
  <path d="M 0 1 L 9 5 L 0 9 z" fill="var(--muted)"/>
  </marker>
  <style>text { font-family: "Inter", system-ui, -apple-system, "Segoe UI", Roboto, sans-serif; } .mono { font-family: "JetBrains Mono", ui-monospace, "Cascadia Code", monospace; }</style>
  </defs>
  <rect x="1" y="1" width="1098" height="398" rx="14" fill="var(--canvas)" stroke="var(--border)" stroke-width="1.5"/>
  <rect x="410" y="28" width="650" height="300" rx="10" fill="var(--rose-bg)" opacity="0.58"/>
  <text x="735" y="56" text-anchor="middle" fill="var(--rose)" class="mono" font-size="12.5" font-weight="800" letter-spacing="0.04em">SNIEDOVICH'S "NO-MAN'S LAND" (α &gt; α̂_A = 1.60)</text>
  <text x="735" y="78" text-anchor="middle" fill="var(--muted)" font-size="11.5">Uninspected by local Info-Gap; policed by Global Wald Maximin Floor</text>
  <line x1="110" y1="336" x2="110" y2="20" stroke="var(--muted)" stroke-width="2" marker-end="url(#axis-arrow)"/>
  <line x1="110" y1="236" x2="1068" y2="236" stroke="var(--muted)" stroke-width="2" marker-end="url(#axis-arrow)"/>
  <text x="98" y="34" text-anchor="end" fill="var(--ink)" font-size="12.5" font-weight="700">Reward R(q, u)</text>
  <text x="735" y="378" text-anchor="middle" fill="var(--ink)" font-size="12.5" font-weight="700">Uncertainty Horizon α →</text>
  <line x1="110" y1="156" x2="1060" y2="156" stroke="var(--sky)" stroke-width="1.8" stroke-dasharray="6 4"/>
  <text x="98" y="160" text-anchor="end" fill="var(--sky)" class="mono" font-size="11.5" font-weight="700">r_c = 100</text>
  <line x1="110" y1="196" x2="1060" y2="196" stroke="var(--emerald)" stroke-width="1.8" stroke-dasharray="3 3"/>
  <text x="98" y="200" text-anchor="end" fill="var(--emerald)" class="mono" font-size="11.5" font-weight="700">Wald +45</text>
  <text x="98" y="240" text-anchor="end" fill="var(--muted)" class="mono" font-size="11.5" font-weight="600">0</text>
  <text x="98" y="320" text-anchor="end" fill="var(--rose)" class="mono" font-size="11.5" font-weight="700">-450</text>
  <line x1="410" y1="156" x2="410" y2="336" stroke="var(--rose)" stroke-width="1.5" stroke-dasharray="4 4"/>
  <text x="410" y="356" text-anchor="middle" fill="var(--rose)" class="mono" font-size="11.5" font-weight="700">α̂_A = 1.60</text>
  <line x1="745" y1="156" x2="745" y2="236" stroke="var(--emerald)" stroke-width="1.5" stroke-dasharray="4 4"/>
  <text x="745" y="258" text-anchor="middle" fill="var(--emerald)" class="mono" font-size="11.5" font-weight="700">α̂_B = 3.50</text>
  <path d="M 110 100 C 310 110, 550 136, 745 156 C 845 167, 945 182, 1040 188" fill="none" stroke="var(--emerald)" stroke-width="3.5" stroke-linecap="round"/>
  <circle cx="745" cy="156" r="5.5" fill="var(--surface)" stroke="var(--emerald)" stroke-width="3"/>
  <text x="735" y="122" text-anchor="middle" fill="var(--emerald)" font-size="12.5" font-weight="800">Option B — Robust Satisficing (Global Min ≥ +45.0 · APPROVED)</text>
  <path d="M 110 56 C 235 70, 338 112, 410 156 C 465 194, 520 308, 600 316 L 1030 316" fill="none" stroke="var(--rose)" stroke-width="3.2" stroke-dasharray="8 5" stroke-linecap="round"/>
  <circle cx="410" cy="156" r="5.5" fill="var(--surface)" stroke="var(--rose)" stroke-width="3"/>
  <text x="128" y="50" fill="var(--rose)" font-size="12.5" font-weight="800">Option A — Fragile Nominal Turkey</text>
  <text x="745" y="298" text-anchor="middle" fill="var(--rose)" font-size="12.5" font-weight="800">Catastrophic Collapse to -450 (BLOCKED by Gate G6!)</text>
</svg>
  <figcaption><strong>Figure 20.3 — Info-Gap Robustness Curve vs. Sniedovich's No-Man's Land and the Global Wald Safeguard</strong></figcaption>
</figure>

Every decision under Knightian uncertainty must pass **three simultaneous checks**:
1. **Zero-ing Audit:** $\hat{\alpha}(q, R(q, \tilde{u})) = 0$ is verified algebraically;
2. **Positive Local Robustness:** $\hat{\alpha}(q, r\_c) > 0$ at the declared critical threshold $r\_c$;
3. **Global Wald Maximin Safeguard:** Across the entire physically admissible state space $\mathcal{U}\_{\text{global}}$ (including all states outside $\mathcal{U}(\hat{\alpha}, \tilde{u})$), the global worst-case payoff must stay above the catastrophic ruin floor:
   $$\min\_{u \in \mathcal{U}\_{\text{global}}} R(q, u) \ge r\_{\text{Wald, min}}$$

Any option that violates the global Wald floor in No-Man's Land is automatically rejected (`Gate G6` and `--defect sniedovich`).

---

## Interactive Laboratory (`/lab`)

Experiment interactively with the Moulton-Campbell unequal-cluster design effect $\mathrm{DEFF}(p) = 1 + (\tilde{m}-1)\rho\_I(p)$ and Chouldechova's algebraic fairness identity:

<div class="stoch-lab"
     data-lab="invariants-moulton-chouldechova"
     data-chapter="20-decision-worlds-regime.html"
     data-title="Invariants Φ₂ &amp; Φ₄: Moulton-Campbell n_eff(p) &amp; Chouldechova Trilemma"
     data-subtitle="Vary cluster CV, tail ICC ρ_I(p), and group base rates (p_A, p_B) to inspect DEFF(p) and FPR_A / FPR_B">
</div>

Adjust the critical survival requirement $r\_c$, the tail drop in Sniedovich's "No-Man's Land," and the Kennedy-O'Hagan GP boundary anchor weight in the interactive laboratory below:

<div class="stoch-lab" data-lab="koh-infogap-wald" data-chapter="20-decision-worlds-regime.html" data-title="Chapter 20 Lab: Kennedy-O'Hagan GP Discrepancy &amp; Info-Gap + Sniedovich Wald Floor" data-cli="cargo run --release --bin labs -- --uncertainty"></div>

---

## Computational Audit in Rust

Run Laboratory 4 from your terminal to see $\Phi\_1$–$\Phi\_5$, Moulton-Campbell tail-exceedance design effects, and Chouldechova's fairness audit on real data:

~~~bash
cargo run --release --bin labs -- --invariants
~~~

~~~text
=== LAB 4: INVARIANTS PHI1-PHI5, MOULTON-CAMPBELL, EXCEEDANCE, AND CHOULDECHOVA ===
  [Phi1 SplitMix64::split(stream_id)] streams=16, max_|rho|=0.0874, Hamming_avalanche=32.3 bits (verified=true) | [Benchmarks Phi2] n=25028 -> m_bar=59.59, m_tilde=119.05, rho_I(0.90)=0.155, n_eff=1297.0 (naive Kish=2482.6) | n=500 -> n_eff=86.0 | var_amp=69.0x
  [Real Data: bank_calls.csv] n=27716, clusters=164, m_bar=169.00, m_tilde=169.00, rho_I(0.90)=0.1623, DEFF=28.26, n_eff(0.90)=980.7 (profile q50/q90/q99 n_eff=[9239, 981, 1308])
  [Phi4: Chouldechova on toronto_arrests.csv] p_A=0.3926 vs p_B=0.1119 -> equalizing PPV=0.65 & FNR=0.25 forces FPR_A=0.2610 vs FPR_B=0.0509 (ratio=5.13x, KMR_var_gap=0.0960)
~~~

~~~bash
cargo run --release --bin labs -- --uncertainty
~~~

Next, inspect the Info-Gap + Sniedovich output line of Laboratory 2 (whose companion Kennedy-O'Hagan GP line was audited in [Chapter 11](11-world-models-discrepancy-authentic-form.md)):

~~~text
=== LAB 2: KENNEDY-O'HAGAN GP DISCREPANCY AND INFO-GAP + SNIEDOVICH ===
  [Info-Gap + Sniedovich] fragile_nominal_turkey: alpha_hat=1.60, zero_ing=0.0, Wald=-450.0, approved=false | robust_satisficing_wald: alpha_hat=3.50, beta_hat=2.13, Wald=45.0, approved=true
~~~

2. **In `[Info-Gap + Sniedovich]`**, the `fragile_nominal_turkey` option boasts a positive local robustness horizon $\hat{\alpha} = 1.60$, yet collapses to $\text{Wald} = -450.0$ in No-Man's Land and is blocked (`approved = false`). The `robust_satisficing_wald` option achieves $\hat{\alpha} = 3.50$, opportuneness $\hat{\beta} = 2.13$, and a positive global Wald floor of $+45.0$, earning approval (`approved = true`).

Three negative defect proofs test these invariants directly:
- `cargo run --release --bin gates -- --defect phi5` injects a monthly aggregate dataset against a daily/shift decision and verifies that Gate `G5` halts with exit code `1`.
- `cargo run --release --bin gates -- --defect moulton` injects a pseudoreplicated card that ignores cluster inflation and verifies that Gate `G2` halts with exit code `1`.
- `cargo run --release --bin gates -- --defect chouldechova` injects a card claiming simultaneous equality of $\mathrm{FPR}, \mathrm{PPV}, \mathrm{FNR}$ under unequal base rates and verifies that Gate `G4` halts with exit code `1`.

## Exercises

1. **(Calculation — Moulton-Campbell $\Phi\_2$)** A retail chain collects $n = 10\text{,}000$ customer satisfaction scores across $C = 50$ stores. The mean cluster size is $\bar{m} = 200$, but because flagship stores are much larger than rural stores, the coefficient of variation of store sizes is $\mathrm{CV}(m\_j) = 1.0$.
   - *(a)* Compute the Moulton-Campbell effective cluster size $\tilde{m} = \bar{m}(1 + \mathrm{CV}^2)$.
   - *(b)* If the threshold exceedance ICC at $p = 0.90$ is $\rho\_I(0.90) = 0.10$, compute the Kish design effect (using $\bar{m}$), the Moulton-Campbell design effect (using $\tilde{m}$), and the true effective sample size $n\_{\mathrm{eff}}(0.90)$.
2. **(Calculation — Chouldechova's Trilemma $\Phi\_4$)** In a medical triage algorithm, Group A has disease prevalence $p\_A = 0.25$ and Group B has prevalence $p\_B = 0.05$. If the triage score is calibrated so that $\mathrm{PPV}\_A = \mathrm{PPV}\_B = 0.50$ and $\mathrm{FNR}\_A = \mathrm{FNR}\_B = 0.20$, compute $\mathrm{FPR}\_A$, $\mathrm{FPR}\_B$, and the ratio $\mathrm{FPR}\_A / \mathrm{FPR}\_B$.
3. **(Concept — Question Resolution $\Phi\_5$)** Why can an electric utility whose annual generation exceeds annual demand by $+25\%$ still suffer rolling blackouts on a hot August afternoon? Which invariant blocks using annual totals to plan peaking turbine capacity?
4. **(Laboratory — Sniedovich Negative Proof)** Run `cargo run --release --bin gates -- --defect sniedovich` and verify that deploying an option that violates the global Wald floor in No-Man's Land is blocked with exit code `1`.

<details>
<summary><strong>Analytical &amp; Computational Solutions (Click to Expand)</strong></summary>

1. **Moulton-Campbell Calculation ($\Phi\_2$):**
   - *(a)* $\tilde{m} = \bar{m}(1 + \mathrm{CV}^2) = 200 \times (1 + 1.0^2) = 400$.
   - *(b)* Naive Kish design effect: $\mathrm{DEFF}\_{\text{Kish}} = 1 + (200 - 1)\times 0.10 = 20.9$.
     Moulton-Campbell design effect: $\mathrm{DEFF}\_{\text{MC}} = 1 + (400 - 1)\times 0.10 = 40.9$.
     True effective sample size: $n\_{\mathrm{eff}}(0.90) = \frac{10\text{,}000}{40.9} \approx 244.5$ (more than $40\times$ smaller than $n = 10\text{,}000$).
2. **Chouldechova's Trilemma Calculation ($\Phi\_4$):**
   - $\mathrm{FPR}\_A = \frac{0.25}{0.75} \times \frac{1 - 0.50}{0.50} \times (1 - 0.20) = \frac{1}{3} \times 1.0 \times 0.80 \approx 0.2667$ ($26.67\%$).
   - $\mathrm{FPR}\_B = \frac{0.05}{0.95} \times \frac{1 - 0.50}{0.50} \times (1 - 0.20) = \frac{1}{19} \times 1.0 \times 0.80 \approx 0.0421$ ($4.21\%$).
   - Ratio: $\frac{\mathrm{FPR}\_A}{\mathrm{FPR}\_B} = \frac{1/3}{1/19} = \frac{19}{3} \approx 6.33\times$.
3. **Question Resolution ($\Phi\_5$):**
   Electricity cannot be trivially stored across seasons at grid scale; supply must match demand at **sub-hourly / 5-minute resolution** (`vic_elec.csv`). Annual totals ($R\_0$) smooth out peak air-conditioning surges. **Invariant $\Phi\_5$ (`verify_resolution_phi5`)** blocks answering a sub-hourly operational dispatch decision with annual aggregates.
4. **Sniedovich Negative Proof:** Running `cargo run --release --bin gates -- --defect sniedovich` prints `NEGATIVE PROOF [sniedovich] BLOCKED SUCCESSFULLY: collapse in No-Man's Land (Wald=-500)` and exits with code `1`.

</details>

## Summary

- **$\Phi\_2$ (Independent Unit & Prospective Clustered DoE):** Discounts nominal sample size $n$ for unequal cluster sizes ($\tilde{m} = \bar{m}(1 + \mathrm{CV}^2)$) and tail clustering ($\rho\_I(p)$), yielding $n\_{\mathrm{eff}}(p) = \frac{n}{1 + (\tilde{m}-1)\rho\_I(p)}$, and sizes prospective cluster-randomized experiments via `prospective_clustered_mde` ($\mathrm{MDE}\_{\text{clustered}} = \mathrm{MDE}\_{\text{iid}}\sqrt{\mathrm{DEFF}}$, raising simulated power from $0.17$ under naive i.i.d. sizing to $0.90$ under DEFF-aware sizing in Axis 2 of the strategic synthesis (`ShapleyCausalRubinMiDoeAudit`, `Clustered_Power=0.90` vs. naive `0.17`, simulated under a pre-registered seed), and inflating the required MDE by $5.32\times$ across the $164$ days of measured `bank_calls.csv`).
- **$\Phi\_4$ (Scope, Fairness & Exit):** Audits Chouldechova's algebraic identity $\mathrm{FPR}\_g = \frac{p\_g}{1-p\_g}\frac{1-\mathrm{PPV}\_g}{\mathrm{PPV}\_g}(1-\mathrm{FNR}\_g)$, names who pays for false positives and false negatives, bounds scope, and sets a numeric exit clause.
- **$\Phi\_5$ (Resolution Alignment):** Blocks coarse temporal or spatial aggregates from answering fine-grained operational decisions ($R\_{\text{data}} \ge R\_{\text{decision}}$).
- **Info-Gap Decision Theory coupled with Sniedovich's Global Maximin Safeguard** measures local robustness ($\hat{\alpha}$) and opportuneness ($\hat{\beta}$) while guaranteeing a global Wald worst-case floor across No-Man's Land.
