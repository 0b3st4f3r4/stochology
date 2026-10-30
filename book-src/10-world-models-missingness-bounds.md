# 10. How the World Reaches the Model

> **Three tempos.** **Detachment:** set aside the database as a faithful portrait of the world. **Abstraction:** what was not observed did not vanish at random; silence has structure, and the model must declare the interval of its ignorance (World Models, $L\_1 \to L\_2$). **Return:** Meng's identity, Rubin's taxonomy, Manski bounds, and Lee trimming bounds. **Negative proof:** `gates -- --defect lee_point_estimate` (exit `1`).

## The Invisible Filter Between Territory and Spreadsheet

Suppose, as a thought experiment, that a city health department wants to estimate the average income of workers affected by an industrial downturn, and an analyst reaches for a large observational sample. The measured stand-in used in this chapter is the Current Population Survey comparison group from Robert LaLonde's (1986) labor-market study ($n = 15\text{,}992$ workers, `cps_observational.csv`): its standard error $\sigma / \sqrt{n}$ is so small that the confidence interval looks razor-sharp. Yet when that survey sample is pooled with the randomized National Supported Work controls (`nsw_experimental.csv`) as the target population, the recorded mean of 1978 earnings is off by $+164.65$ dollars (measured, Lab 1 `selection_err`), and the filter shrinks the sample's effective size to $n\_{\mathrm{eff}} = 2\text{,}842.70$. What went wrong? Before a single line of regression or machine learning is run in Layer $L\_2$, **Bridge $L\_1 \to L\_2$** acts as an observation filter: who gets recorded ($R\_i = 1$) and who goes missing ($R\_i = 0$) is rarely independent of the outcome $Y\_i$ itself.

## 10.1 Bridge $L\_1 \to L\_2$: Rubin's Taxonomy, Meng's Identity, and Manski's Bounds

### Foundations: What Happens When Data Are Missing or Filtered?

Suppose a target population consists of $N$ units (for instance, all $N$ workers in a labor market or all $N$ patients visiting an emergency department), each carrying an outcome of interest $Y\_i$. We want to know the true population average:

$$\bar{Y}\_N = \frac{1}{N}\sum\_{i=1}^N Y\_i$$

However, we only observe a subset of $n < N$ units. To make the filter visible in mathematics, we define a binary **recording indicator** $R\_i$ for every unit $i \in \lbrace 1, \dots, N \rbrace$:

$$R\_i = \begin{cases} 1, & \text{if unit } i \text{ is recorded in our dataset,} \\\\ 0, & \text{if unit } i \text{ is missing or excluded.} \end{cases}$$

The total sample size is $n = \sum\_{i=1}^N R\_i$, the sampling fraction is $f = \frac{n}{N}$, and the sample mean we actually compute on our computer is:

$$\bar{Y}\_n = \frac{\sum\_{i=1}^N R\_i Y\_i}{\sum\_{i=1}^N R\_i}$$

Donald Rubin (1976) classified the physical mechanism governing $R\_i$ into three regimes:
1. **`MCAR` (Missing Completely at Random):** Whether a record is observed ($R\_i$) is statistically independent of both the observed covariates $X\_i$ and the unseen outcome $Y\_i$ ($R \perp (X, Y)$). *Example:* A laboratory technician accidentally drops one out of twenty blood vials on the floor. Under `MCAR`, the sample mean $\bar{Y}\_n$ is an unbiased estimate of $\bar{Y}\_N$.
2. **`MAR` (Missing at Random):** Conditional on the covariates $X\_i$ that we *did* record, missingness no longer depends on the unseen outcome $Y\_i$ ($R \perp Y \mid X$). *Example:* Younger respondents are less likely to answer a telephone survey, but *within* each age bracket, respondents and non-respondents have the same income distribution. Under `MAR`, we can recover $\bar{Y}\_N$ by reweighting across the observed strata $X$.
3. **`MNAR` (Missing Not at Random):** Even after conditioning on all observed variables $X\_i$, the probability of being recorded still depends directly on the unobserved outcome $Y\_i$ itself ($R \not\perp Y \mid X$). *Example:* People with severely depressed earnings self-select into a government training program, or dissatisfied customers leave a platform without filling out the exit survey.

### Xiao-Li Meng's Identity (2018) and the "Big Data Paradox"

A widespread myth in data science holds that *"if the dataset is massive enough ($n$ in the hundreds of thousands or millions), selection bias no longer matters."* In 2018, Harvard statistician Xiao-Li Meng proved an exact algebraic identity that refutes this assumption.

Because $\bar{Y}\_n = \frac{\frac{1}{N}\sum\_{i=1}^N R\_i Y\_i}{\frac{1}{N}\sum\_{i=1}^N R\_i} = \frac{\mathbb{E}\_N[R Y]}{\mathbb{E}\_N[R]}$ and $\mathrm{Cov}\_N(R, Y) = \mathbb{E}\_N[RY] - \mathbb{E}\_N[R]\mathbb{E}\_N[Y]$, a three-line algebraic rearrangement yields **Meng's Identity**:

$$\bar{Y}\_n - \bar{Y}\_N = \underbrace{\rho\_{R,Y}}\_{\text{Data Quality}} \times \underbrace{\sqrt{\frac{N - n}{n}}}\_{\text{Data Quantity}} \times \underbrace{\sigma\_Y}\_{\text{Problem Difficulty}}$$

Three factors multiply on the right-hand side:
1. **$\rho\_{R,Y} = \mathrm{Corr}\_N(R, Y)$ (Data Quality):** The population correlation between *being recorded* ($R\_i$) and *the outcome itself* ($Y\_i$). Under true simple random sampling (`MCAR`), $\mathbb{E}[\rho\_{R,Y}^2] = \frac{1}{N - 1}$, which shrinks rapidly with population size. Under self-selection or unadjusted observational filtering (`MNAR`), $\rho\_{R,Y}$ stays stuck at a non-zero constant (say, $+0.05$ or $+0.13$).
2. **$\sqrt{\frac{N - n}{n}} = \sqrt{\frac{1 - f}{f}}$ (Data Quantity):** A factor determined by the sampling fraction $f = n/N$.
3. **$\sigma\_Y$ (Problem Difficulty):** The true population standard deviation of the outcome $Y$.

This identity shows why sample size alone cannot rescue an unrepresentative dataset. If a sample of size $n$ has selection correlation $\rho\_{R,Y}$, its mean squared error equals that of a genuinely random sample of **effective sample size**:

$$n\_{\mathrm{eff}} \approx \frac{n}{1 + n \left(\frac{N - n}{N}\right) \rho\_{R,Y}^2 \cdot (\text{design scaling})}$$

In our empirical benchmark on Robert LaLonde's (1986) labor-market data (`nsw_experimental.csv`, the randomized National Supported Work experiment, versus `cps_observational.csv`, the observational Current Population Survey comparison group of $n = 15\text{,}992$ workers), the selection correlation is $\rho\_{R,Y} = +0.1334$. That correlation of $0.1334$ shrinks a nominal sample of $n = 15\text{,}992$ workers down to an effective sample size of **$n\_{\mathrm{eff}} = 2\text{,}842.70$**. Over $82\%$ of the apparent sample size was an illusion created by the filter on Bridge $L\_1 \to L\_2$.

### Charles Manski's Non-Parametric Worst-Case Bounds

When missingness or selection is `MNAR` and we do not know $\rho\_{R,Y}$ exactly, what can an honest scientist say without inventing untestable parametric assumptions?

Econometrician **Charles Manski** (1989, 1990) showed that whenever the outcome $Y$ is bounded by known physical or institutional limits $[Y\_{\min}, Y\_{\max}]$, the true population expectation $\mathbb{E}[Y]$ is strictly trapped inside a **non-parametric identification interval**:

$$\mathbb{E}[Y] = P(R = 1) \mathbb{E}[Y \mid R = 1] + P(R = 0) \mathbb{E}[Y \mid R = 0]$$

Since we observe the response rate $p\_{\text{obs}} = P(R = 1)$ and the observed mean $\bar{Y}\_{\text{obs}} = \mathbb{E}[Y \mid R = 1]$, and since the unobserved mean $\mathbb{E}[Y \mid R = 0]$ cannot be smaller than $Y\_{\min}$ or larger than $Y\_{\max}$, the truth must lie inside **Manski's Bounds**:

$$\bigl[  p\_{\text{obs}} \bar{Y}\_{\text{obs}} + (1 - p\_{\text{obs}}) Y\_{\min} ,\quad p\_{\text{obs}} \bar{Y}\_{\text{obs}} + (1 - p\_{\text{obs}}) Y\_{\max}  \bigr]$$

No matter how adversarial the missingness mechanism is, the population mean cannot escape this interval (`crates/stochology-core/src/layers/`, `ManskiBounds`). If a decision remains optimal at both endpoints of Manski's interval, the decision is immune to missing-data bias; if the optimal action flips between the lower and upper bound, the analyst must declare that the conclusion depends on an untestable assumption about the unseen records.

### Stratified `MAR` Bounds, David Lee's (2009) Sharp Trimming Bounds, and Rubin's Multiple Imputation (`RubinMultipleImputationAudit`)

Can we tighten Manski's worst-case interval when additional structure is observed, and when does parametric Multiple Imputation fail? The protocol provides two non-parametric refinements in `ManskiBounds` alongside an audit of Donald Rubin's (1987) Multiple Imputation (`RubinMultipleImputationAudit` in `crates/stochology-core/src/layers/`):

1. **Covariate-Stratified Bounds (`ManskiBounds::compute_stratified`):** When the population is partitioned into $S$ observed covariate strata with population weights $w\_s \ge 0$ ($\sum\_{s=1}^S w\_s = 1$), stratum response rates $p\_{\text{obs}, s}$, and stratum-specific physical bounds $[Y\_{\min, s}, Y\_{\max, s}]$, aggregating the stratum bounds yields:
   $$\mathbb{E}[Y] \in \left[\sum\_{s=1}^S w\_s\bigl(p\_{\text{obs}, s}\bar{Y}\_{\text{obs}, s} + (1 - p\_{\text{obs}, s})Y\_{\min, s}\bigr),\quad \sum\_{s=1}^S w\_s\bigl(p\_{\text{obs}, s}\bar{Y}\_{\text{obs}, s} + (1 - p\_{\text{obs}, s})Y\_{\max, s}\bigr)\right]$$
   which is guaranteed to be at least as tight as the unstratified Manski interval whenever $Y\_{\min, s} \ge Y\_{\min}$ and $Y\_{\max, s} \le Y\_{\max}$.
2. **David Lee's (2009) Sharp Trimming Bounds for Differential Attrition (`ManskiBounds::lee_trimming_bounds`):** In randomized experiments with attrition (where treatment affects who remains observed, $p\_1 = P(R(1)=1) > p\_0 = P(R(0)=1)$) under **monotonic selection** ($R\_i(1) \ge R\_i(0)$), an excess fraction $q = \frac{p\_1 - p\_0}{p\_1}$ of the observed treated group consists of units induced to remain observed by treatment. Without bounded support assumptions $[Y\_{\min}, Y\_{\max}]$, David Lee (2009) proved that the sharp non-parametric bounds for the average treatment effect among **always-responders** ($R(1)=1, R(0)=1$) are obtained by trimming the top $q$ or bottom $q$ quantiles of the observed treated distribution:
   $$\Delta\_{\mathrm{Lee}} \in \Bigl[  \mathbb{E}\bigl[Y\_1 \mid R\_1 = 1,\ Y\_1 \le Q\_{1-q}(Y\_1)\bigr] - \mathbb{E}[Y\_0 \mid R\_0 = 1],\quad \mathbb{E}\bigl[Y\_1 \mid R\_1 = 1,\ Y\_1 \ge Q\_q(Y\_1)\bigr] - \mathbb{E}[Y\_0 \mid R\_0 = 1]  \Bigr]$$
   Alongside Lee's bounds, `empirical_wasserstein_1` computes the exact 1D Kantorovich-Wasserstein $W\_1(P, Q) = \int\_0^1 |F\_P^{-1}(u) - F\_Q^{-1}(u)|\,du$ distance between the empirical outcome distributions.
3. **Rubin's (1987) Multiple Imputation Pooling vs. `MNAR` Selection (`RubinMultipleImputationAudit`):** Given $m \ge 2$ completed datasets with within-imputation point estimates $\hat{Q}\_k$ and variances $U\_k$, Rubin's pooling rules combine the mean estimate $\bar{Q}\_m = \frac{1}{m}\sum\_{k=1}^m \hat{Q}\_k$, within-imputation variance $\bar{U}\_m = \frac{1}{m}\sum\_{k=1}^m U\_k$, and between-imputation variance $B\_m = \frac{1}{m-1}\sum\_{k=1}^m (\hat{Q}\_k - \bar{Q}\_m)^2$ into the total variance identity:
   $$T\_m = \bar{U}\_m + \left(1 + \frac{1}{m}\right)B\_m, \qquad r\_m = \frac{(1 + m^{-1})B\_m}{\bar{U}\_m}, \qquad \gamma\_m = \frac{r\_m + 2/(\nu\_m + 3)}{r\_m + 1}$$
   In `ShapleyCausalRubinMiDoeAudit` (Axis 2 of the strategic synthesis in `crates/stochology-lenses/src/strategic/`, filed under World Models by the book's authorial mapping; simulated, `seed = 2026`, $n = 300$ units with about $40\%$ `MAR` missingness and $M = 5$ stochastic-regression imputations), Rubin's variance decomposition identity closes to machine precision (residual checked below $10^{-14}$), and the completed datasets retain $0.93$ of the true outcome variance (`Rubin_MI_var_ret=0.93`) versus $0.62$ for single mean imputation (the naive arm printed by `--defect strategic_accommodation`). The `MNAR` check is narrower than its name: the code truncates the simulated sample to $Y > 5.5$ and asks whether a complete-case interval $\bar{Y} \pm 1.96\,\mathrm{SE}$ on the truncated sample covers the true mean. It covers it on $0.0\%$ of the 200 confirmation seeds, so the claim sealed as `strategic.a2.parametric_mar_mi_covers_true_mean_under_mnar` is recorded as refuted in `discarded_claims` (the sealed name says "MAR MI", but no imputation is run in this check). Manski's non-parametric interval on the same truncated sample, with an assumed support $[-2, 14]$, covers the true mean; that coverage is folded into the axis identity check, which must hold on every seed.

On Bridge $L\_1 \to L\_2$, Invariant $\Phi\_3$ (`EpistemologicalCard::audit()` in `crates/stochology-core/src/card/`, whose causal-ladder and cryptographic pre-registration requirements are completed in [Chapter 17](17-model-decisions-card-preregistration.md)) enforces as its first requirement:

1. An explicit Rubin missingness classification (`MCAR`, `MAR`, or `MNAR`) paired with Meng's Identity and Manski bounds;

---

## Interactive Laboratory (`/lab`)

Use the interactive dual-bridge simulator below to test how even a tiny selection correlation $|\rho\_{R,Y}|$ collapses Meng's effective sample size $n\_{\mathrm{eff}}$ on Bridge $L\_1 \to L\_2$ (together with the performative feedback loop on Bridge $L\_3 \to L\_1$ examined in [Chapter 19](19-world-decisions-performativity.md)):

<div class="stoch-lab" data-lab="meng-performativity" data-chapter="10-world-models-missingness-bounds.html" data-title="Chapter 10 Lab: Meng Big-Data Paradox (L₁→L₂) &amp; Performative RRM vs. PerfGD (L₃→L₁)" data-cli="cargo run --release --bin labs -- --layers"></div>

---

## Computational Audit in Rust

In `crates/stochology-core/src/layers/`, the three bridges are implemented in `MengAudit`, `ManskiBounds`, `NonlinearSummary`, and `PerformativeSystem`. Run Laboratory 1 from your terminal:

~~~bash
cargo run --release --bin labs -- --layers
~~~

~~~text
=== LAB 1: THREE ONTOLOGICAL LAYERS AND BRIDGE MECHANICS (L1 -> L2 -> L3 -> L1) ===
  [Bridge L1->L2] Meng (CPS n=15992 vs NSW n=260): rho_R_Y=+0.1334, selection_err=+164.65, n_eff_Meng=2842.70 | Manski MNAR=[5463.2, 13835.3] | LeeBounds(q=0.031)=[872.5, 1995.8], W1=1722.7
~~~

Examine the Bridge $L\_1 \to L\_2$ output line above (the companion lines for Bridge $L\_2 \to L\_3$ and Bridge $L\_3 \to L\_1$ are audited in [Chapter 15](15-decision-models-curvature-scoring.md) and [Chapter 19](19-world-decisions-performativity.md)):
1. **On Bridge $L\_1 \to L\_2$**, comparing the observational CPS sample ($n = 15\text{,}992$) against the randomized NSW benchmark reveals $\rho\_{R,Y} = +0.1334$, collapsing the effective sample size to $n\_{\mathrm{eff}} = 2\text{,}842.70$, while the Manski `MNAR` bounds trap the treated arm's mean earnings within $[5\text{,}463.2,\ 13\text{,}835.3]$ and David Lee's (2009) trimming bounds under $q = 3.1\%$ differential attrition bound the always-responder treatment effect within $[872.5,\ 1\text{,}995.8]$ (`W1 = 1722.7`). The earnings are measured, but the missingness behind both intervals is assumed: Lab 1 adds 30 hypothetical unobserved treated units with an assumed support $[0, 60\text{,}000]$ for Manski, and 15 treated plus 30 control hypothetical dropouts for Lee.

### Negative Proof: A Point Estimate Under `MNAR` (`--defect lee_point_estimate`)

The negative proof `lee_point_estimate` shows what the $\Phi\_3$ card audit (Gate `G3`) does with an analyst who reports a single number where only an interval is identified. It takes the canonical card `card_04` and overwrites its `missingness_mechanism` with the hand-written text `"MNAR naive point estimate +1340.80 only"`. It then treats a positive 1978 earnings record in `nsw_experimental.csv` as "observed"; the treated arm has the higher share of positive earners ($p\_1 > p\_0$), so the excess fraction to trim from the treated arm is $q = \frac{p\_1 - p\_0}{p\_1} = 0.1462$ (measured). `ManskiBounds::lee_trimming_bounds` trims that fraction from the top and from the bottom of the sorted treated earnings (fractional trimming, $K = (1-q)m$), and `ManskiBounds::lee_tipping_point_delta` computes $\delta\_1^* = -\frac{\bar{Y}\_1^{\mathrm{obs}} - \bar{Y}\_0^{\mathrm{obs}}}{1 - p\_1}$, the shift in the unobserved treated outcomes that would bring the effect to zero. The proof blocks (exit code `1`) only when three things hold together: the Lee interval contains zero, $\delta\_1^*$ is negative, and `EpistemologicalCard::audit()` rejects the card, because an `MNAR` declaration containing the word "only" fails the `PHI3_MISSINGNESS_MECHANISM` check:

~~~bash
cargo run --release --bin gates -- --defect lee_point_estimate
~~~

~~~text
NEGATIVE PROOF [lee_point_estimate] BLOCKED SUCCESSFULLY: naive positive point estimate spans zero under fractional Lee bounds [-1142.29, 2621.20] (q=0.1462, tipping delta*=-5512.35)
~~~

Read the line as a measured result on `nsw_experimental.csv` under the stated definition of response. The interval $[-1\text{,}142.29,\ 2\text{,}621.20]$ dollars contains zero, so the sign of the effect among always-employed workers is not identified, and a reduction of about $5\text{,}512$ dollars in the unseen treated earnings would erase the observed difference. A card that reports only the positive point estimate claims more than the data identify, and the gate refuses it.

## Exercises

1. **(Calculation — Meng's Identity)** Suppose a national health registry covers $n = 2\text{,}000\text{,}000$ self-enrolled citizens out of a population of $N = 10\text{,}000\text{,}000$, with outcome standard deviation $\sigma\_Y = 50$ and a tiny selection correlation $\rho\_{R,Y} = +0.04$. Using Meng's Identity, calculate the exact selection bias $\bar{Y}\_n - \bar{Y}\_N$. Why did a sample of two million people fail to make this error vanish?

<details>
<summary><strong>Analytical &amp; Computational Solutions (Click to Expand)</strong></summary>

1. **Meng's Identity Calculation:**
   $$\bar{Y}\_n - \bar{Y}\_N = \rho\_{R,Y} \sqrt{\frac{N - n}{n}} \sigma\_Y = 0.04 \times \sqrt{\frac{8\text{,}000\text{,}000}{2\text{,}000\text{,}000}} \times 50 = 0.04 \times 2 \times 50 = +4.00$$
   Because $\sqrt{(N-n)/n} = \sqrt{(1-f)/f}$ depends only on the sampling fraction $f = n/N = 0.20$ and $\rho\_{R,Y} = +0.04$ does not decay as $1/\sqrt{N}$ under self-selection, the bias $+4.00$ remains even with $n = 2\text{,}000\text{,}000$.

</details>

## Summary

- **Bridge $L\_1 \to L\_2$** is governed by Rubin's missingness taxonomy (`MCAR`, `MAR`, `MNAR`), **Meng's Identity** ($\bar{Y}\_n - \bar{Y}\_N = \rho\_{R,Y}\sqrt{\frac{N-n}{n}}\sigma\_Y$), and **Manski's and Lee's non-parametric bounds**.
- **$\Phi\_3$ (Severe Testing & Missingness)** enforces explicit missingness typing, Meng's `ddc`/`DDI` audit, and non-parametric bounds on Bridge $L\_1 \to L\_2$, paired with Pearl's Causal Ladder rung and SHA-256 pre-registration ([Chapter 17](17-model-decisions-card-preregistration.md)).
