# Chapter 46 — The Non-Identical, Scale Invariance, Convexity, and Heavy Tails

> Laboratory 5 of the `stochology` CLI: `--chapter 5`.

Even if an archive is audited by **Canonical Operator I** (Chapters 43–45), how does a physical or social phenomenon act upon a living institution when we move from epistemic estimation to physical capacity sizing?

**Mediating Filter II · Axis 2 ($S_{\mathrm{vital}}$: Vital Margin)** governs the passage across **Chapters 46 and 47**, unified in [`crates/stochology/src/vital.rs`](crates/stochology/src/vital.rs) around **Canonical Operator II** ([`VitalMarginState`](crates/stochology/src/vital.rs#L361-L450)):

$$\Delta_J(\phi, X) = \mathbb{E}\bigl[\phi(X)\bigr] - \phi\bigl(\mathbb{E}[X]\bigr) \approx \frac{1}{2}\phi''\bigl(\mathbb{E}[X]\bigr)\,\mathrm{Var}(X) + \mathcal{R}_{\mathrm{tail}}(\hat{\alpha})$$

Its first constitutive tension (**Tension 3**) confronts **formal equivalence and the tyranny of the lone mean** against **the non-identical, measurement scale invariance, convex harm, and heavy-tailed extremes** (Stevens, 1946; Jensen, 1906; Adorno, 1966; Georgescu-Roegen, 1971; Hill, 1975; Desrosières, 1993; Porter, 1995; Hacking, 2002; Gneiting & Raftery, 2007).

---

## 5.1 The Non-Identical and the Dialectical Penumbra

In *Negative Dialectics* (1966), Theodor W. Adorno observed that conceptual and statistical classification operates by forcing heterogeneous particulars under an identity sign ("the non-identical," *das Nichtidentische*). A few years later, Nicholas Georgescu-Roegen (*The Entropy Law and the Economic Process*, 1971) distinguished **arithmomorphic concepts**—discrete numbers separated by an empty void—from **dialectical concepts** (justice, clinical severity, ecological distress, delay disruption), which are surrounded by a continuous **dialectical penumbra** $[c_L, c_U)$ where opposites overlap.

When a bureaucracy replaces a continuous phenomenon $X$ with a rigid binary indicator $\mathbb{I}\lbrace X \ge c\rbrace$ (for instance, defining a flight as "on time" at $14.9$ minutes and "delayed" at $15.0$ minutes, or a household as "eligible" vs. "ineligible" at an arbitrary poverty line), how much of the real population sits inside the contested boundary zone $[c_L, c_U)$?

In [`VitalMarginState::from_heavy_tails_and_asymmetry`](crates/stochology/src/vital.rs#L535-L671), `penumbra_mass` computes $\mathbb{P}(c_L \le X < c_U)$ across the penumbra. Whenever `penumbra_mass` is non-negligible, reporting a single binary classification rate hides the boundary sensitivity of the category itself.

---

## 5.2 Stevens's Scale Hierarchy and the Forbidden Ordinal Mean

In his landmark 1946 paper *On the Theory of Scales of Measurement*, S. S. Stevens proved that a statistical summary is meaningful only if its conclusions are invariant under the admissible transformations $\phi$ of its measurement scale:

| Scale Type | Admissible Transformation $\phi(x)$ | Invariant Statistics | Illegitimate Operations |
| :--- | :--- | :--- | :--- |
| **Nominal** | Any bijection | Mode, frequency, entropy | Mean, median, order |
| **Ordinal** | Any strictly increasing $\phi$ ($x < y \implies \phi(x) < \phi(y)$) | Median, quantiles, Mann–Whitney $U = \mathbb{P}(A > B) + \frac{1}{2}\mathbb{P}(A = B)$ | Arithmetic mean $\bar{x}$, standard deviation |
| **Interval** | Positive affine $\phi(x) = a x + b$ ($a > 0$) | Mean, variance, Pearson correlation | Coefficient of variation $\sigma / \mu$, ratios |
| **Ratio** | Positive similarity $\phi(x) = a x$ ($a > 0$) | All of the above + geometric mean, ratios, Kelly growth | — |

Institutions routinely collect **ordinal** scores—Likert satisfaction rubrics $1..5$, clinical pain scales, ESG grades, or LLM judge rubrics—and average them as if the gap between $1$ and $2$ were identical to the gap between $4$ and $5$. In [`mann_whitney_dominance`](crates/stochology/src/vital.rs#L146-L161) and [`VitalMarginState::from_heavy_tails_and_asymmetry`](crates/stochology/src/vital.rs#L535-L671), two facts are verified simultaneously:
1. A strictly increasing convex transformation $\phi(x) = x^2$ can **invert the arithmetic mean ranking** ($\bar{A} > \bar{B}$ while $\overline{\phi(A)} < \overline{\phi(B)}$, `mean_ranking_reversed = true`);
2. The ordinal Mann–Whitney dominance probability $U(A, B) = \mathbb{P}(A > B) + \frac{1}{2}\mathbb{P}(A = B)$ is **100% invariant** (`ordinal_invariance_error = 0.0`).

---

## 5.3 Jensen's Convexity Gap and the Continuous Ranked Probability Score (CRPS)

Even on a genuine ratio scale (megawatts of electricity demand, minutes of delay, millions of kroner in fire damage), reporting only the sample mean $\mathbb{E}[X]$ commits **the flaw of averages** whenever harm $\phi(x)$ is convex ($\phi''(x) > 0$). By Johan Jensen's (1906) inequality:
$$\Delta_J = \mathbb{E}[\phi(X)] - \phi(\mathbb{E}[X]) \ge 0$$
For quadratic loss $\phi(x) = x^2$, the gap is exactly the variance: $\Delta_J = \mathrm{Var}(X)$. A second-order Taylor expansion around the mean $\mu = \mathbb{E}[X]$ reveals the general dependence on curvature:
$$\Delta_J \approx \frac{1}{2}\phi''(\mu)\,\sigma^2 + \frac{1}{6}\phi'''(\mu)\,\mathbb{E}[(X - \mu)^3] + \dots$$
Whenever the underlying distribution is right-skewed or heavy-tailed, higher-order moments dominate, widening $\Delta_J$ far beyond the Gaussian approximation.

In [`VitalMarginState`](crates/stochology/src/vital.rs#L361-L450), every nonlinear evaluation reports the full distribution summary:
- the sample mean $\mathbb{E}[X]$, the median $q_{50}$, and the tail quantiles $(q_{05}, q_{95})$;
- the evaluation at the mean $\phi(\mathbb{E}[X])$, the expected evaluation $\mathbb{E}[\phi(X)]$, and the **Jensen gap** $\Delta_J$ introduced in [Chapter 31](31-distribution-and-moment.md) §5;
- Tilmann Gneiting & Adrian Raftery's (2007) strictly proper **Continuous Ranked Probability Score** ([`crps_empirical`](crates/stochology/src/vital.rs#L165-L186)) of the predictive distribution $F$ against realized outcome $y$.

### 5.3.1 CRPS Integral Representation vs. MSE Failure

Under heavy-tailed losses, classical Mean Squared Error ($\mathrm{MSE} = (y - \hat{y})^2$) is pathological: squaring an extreme loss produces massive variance in the evaluation metric itself. Gneiting & Raftery's CRPS solves this by integrating the squared difference between the cumulative distribution function $F(t)$ and the empirical Heaviside step $\mathbf{1}\{y \le t\}$ across all thresholds $t \in \mathbb{R}$:
$$\mathrm{CRPS}(F, y) = \int_{-\infty}^\infty \bigl(F(t) - \mathbf{1}\{y \le t\}\bigr)^2 dt = \mathbb{E}_F|X - y| - \frac{1}{2}\mathbb{E}_F|X - X'|$$
where $X, X' \stackrel{\text{i.i.d.}}{\sim} F$. CRPS possesses three critical virtues:
1. **Strict Propriety:** The expected CRPS is minimized if and only if the forecaster reports the true data-generating distribution $F$;
2. **Linear Tail Sensitivity:** Unlike MSE, CRPS grows linearly with extreme errors in the tails ($|X - y|$), remaining finite and numerically stable even when second moments diverge;
3. **Distribution-Wide Evaluation:** It assesses the entire distribution—center, dispersion, and extreme quantiles—in the natural units of the outcome.

---

## 5.4 Bruce M. Hill's (1975) Tail Index, Four Moment Regimes, and the Breakdown of the CLT

Why can $\Delta_J$ grow by orders of magnitude as sample size increases? Because many physical, financial, and operational losses follow **Pareto power-law tails**:
$$\mathbb{P}(X > x) \sim L(x)\,x^{-\alpha} \quad \text{as } x \to \infty$$
where $L(x)$ is a slowly varying function ($\lim_{t\to\infty} L(t x)/L(t) = 1$) and $\alpha > 0$ is the **tail index**.

Using the top $k + 1$ order statistics $X_{(n)} \ge X_{(n-1)} \ge \dots \ge X_{(n-k)} > 0$, Bruce M. Hill's (1975) conditional maximum-likelihood tail estimator in [`VitalMarginState::hill_alpha_on_sorted_pos`](crates/stochology/src/vital.rs#L531-L544) and [`VitalMarginState::from_heavy_tails_and_asymmetry`](crates/stochology/src/vital.rs#L535-L671) is:
$$\hat{\alpha}_{k,n} = \left(\frac{1}{k}\sum_{i=1}^k \ln\frac{X_{(n - i + 1)}}{X_{(n - k)}}\right)^{-1}$$

### 5.4.1 The Four Power-Law Moment Regimes

The tail exponent $\alpha$ dictates the convergence of the tail integral $\mathbb{E}[X^p] = \int_0^\infty p t^{p-1}\mathbb{P}(X > t) dt$, partitioning all phenomena into **four fundamental regimes**:

1. **$\alpha \le 1$ — Infinite Mean and Variance (`InfiniteMeanAndVariance`):**
   Even the population mean does not exist ($\mathbb{E}[X] = \infty$). The sample average $\bar{X}_n$ does not stabilize around any constant; as $n \to \infty$, $\bar{X}_n$ continues to grow without bound, dominated by rare colossal records.
2. **$1 < \alpha \le 2$ — Finite Mean, Infinite Variance (`FiniteMeanInfiniteVariance`):**
   The population mean exists ($\mathbb{E}[X] < \infty$), but the population variance is infinite ($\mathrm{Var}(X) = \infty$). As established in [Chapter 13](13-integrals-and-accumulation.md) §2, the integral for the first moment converges, but the second moment integral diverges. Danish fire insurance claims ($\hat{\alpha} = 1.39$, `[Measured]`) sit squarely in this regime.
3. **$2 < \alpha \le 4$ — Finite Variance, Infinite Kurtosis:**
   The mean and variance are finite, but the fourth moment diverges ($\mathbb{E}[X^4] = \infty$). Sample estimates of variance fluctuate erratically across samples, and higher-order risk measures (such as tail risk measures) suffer massive pre-asymptotic instability.
4. **$\alpha > 4$ — Gaussian Domain (Finite Kurtosis):**
   Higher moments exist, sample variances converge stably, and classical Gaussian asymptotics provide reliable approximations.

### 5.4.2 Failure and Extreme Pre-Asymptotic Slowdown of the Central Limit Theorem

When $\alpha < 2$, the Lindeberg–Lévy Central Limit Theorem fails completely: normalized sums $(S_n - n\mu) / n^{1/\alpha}$ do not converge to a normal distribution, but to an **$\alpha$-stable Lévy distribution** with heavy tails and infinite variance (Gnedenko & Kolmogorov, 1954).

Even more dangerous is the pre-asymptotic behavior for $1 < \alpha \le 2$: while the Strong Law of Large Numbers guarantees $\bar{X}_n \xrightarrow{\mathrm{a.s.}} \mu$, the rate of convergence is governed by the stable scaling factor:
$$|\bar{X}_n - \mu| = \mathcal{O}_p\bigl(n^{1/\alpha - 1}\bigr)$$
For a Gaussian distribution, errors shrink as $n^{-0.50}$. But for Danish fire losses with $\hat{\alpha} = 1.39$:
$$n^{1/1.39 - 1} = n^{-0.28}$$
To reduce estimation uncertainty by a factor of $10$, a Gaussian process requires $100$ times more data ($10^{1/0.50}$), whereas a process with $\alpha = 1.39$ requires **$3,710$ times more data** ($10^{1/0.28}$)! Operating on the naive sample mean creates a dangerous illusion of certainty while leaving the institution completely exposed to unmodeled tail surprises.

When extreme tail observations are clustered ($\mathrm{DEFF}_{\mathrm{tail}} > 1$), the effective number of tail order statistics shrinks to $k_{\mathrm{eff}} = k / \mathrm{DEFF}_{\mathrm{tail}}$ and $\hat{\alpha}_k \sim \mathrm{Inv\text{-}Gamma}(k_{\mathrm{eff}}, k_{\mathrm{eff}}\alpha_0)$. Because the Holling basin rupture probability $\hat{P}_{\mathrm{rup}}(\hat{\alpha}_k) = \exp(-\hat{\alpha}_k \ln(D_t / x_m))$ is strictly convex in $\hat{\alpha}_k$, [`HollingTailRuptureAudit`](crates/stochology/src/vital.rs) computes both the second-order Jensen-debiased estimate and the exact $90\%$ confidence interval $[q_{05}, q_{95}]$ across `vital.rs` (`tail_effective_k`, `holling_rupture_prob_plugin`, `holling_rupture_prob_debiased`, `holling_rupture_ci90`).

---

## 5.5 Laboratory 5: Real Data on Danish Fire Losses and Out-of-Time / Out-of-Distribution Flight Delays

Running laboratory 5 (`cargo run --release --bin stochology -- --chapter 5`) evaluates [`vital.rs`](crates/stochology/src/vital.rs) on six real-world datasets (`data/danish_fire.csv`, $n = 2{,}167$, `data/nyc_flights_2013.csv`, `data/nyc_flights_2023.csv`, `data/nyc_flights_2013_h1.csv`, `data/nyc_flights_2023_h1.csv`, and `data/houston_flights_2011.csv`, `[Measured]`):

```text
=== CHAPTER 5: FILTER II (S_VITAL) — ASYMMETRY, SCALE INVARIANCE & HEAVY TAILS ===
  Danish Fire Losses [Measured] (data/danish_fire.csv, n = 2167):
    Hill alpha_hat = 1.39 (k=150, 95% CI=[1.17, 1.61], k-stability [50..250] = [1.36, 1.87], regime = FiniteMeanInfiniteVariance, median = 1.78, q95 = 9.97)
    Jensen Gap Delta_J = E[phi(X)] - phi(E[X]) = 1.015 (E[phi(X)] = 1.613 vs phi(E[X]) = 0.598)
    Empirical CRPS = 1.458 | Ordinal Invariance Error = 0.0000 (Mean Ranking Reversed = true) | Penumbra Mass = 0.396
    Hill-Holling DEFF_tail Coupling [Simulated] (DEFF=10.1, k_eff=11.9): P_true=0.0166, E[P_plugin]=0.0219 (Jensen +32.3%), P_debiased=0.0119, CI90=[0.00083, 0.06704] (80.3x)
  NYC Flights Conditional Delay>0 Tail Index [Measured, k=n+/20 on non-NA rows]: 2013 alpha_hat = 2.89 (95% CI=[2.78, 3.00], k=2616, n=138375, n+=52330, total_rows=141193, skipped_NA=2818, penumbra_[10,30]=12.7%) vs 2023 alpha_hat = 2.27 (95% CI=[2.18, 2.36], k=2430, n=173566, n+=48602, total_rows=176739, skipped_NA=3173, penumbra_[10,30]=8.6%) | OOT H1 (2013=3.13 [k=4033, n=188971, n+=80674, total=195583, NA=6612], 2023=2.39 [k=4648, n=249252, n+=92972, total=258613, NA=9361]) & OOD Houston 2011=2.41 (k=5346, n=223874, n+=106920, total=224591, NA=717)
```

- **Danish Fire Insurance Losses (`danish_fire.csv`, $n = 2{,}167$, `[Measured]` in millions of DKK):** The median claim is $1.78\text{M}$, the mean is $3.39\text{M}$, the 95th percentile reaches $9.97\text{M}$, and the out-of-sample split-half CRPS is $1.458$ (`[Measured]`). Under convex reinsurance stress $\phi(x) = 0.08\,x^{1.65}$ (`[Assumed]`), the evaluation at the mean is $\phi(\mathbb{E}[X]) = 0.598$, whereas the true expected stress is $\mathbb{E}[\phi(X)] = 1.613$—a **Jensen gap $\Delta_J = +1.015$** ($+170\%$ of $\phi(\mathbb{E}[X])$, `[Measured]`). Hill's estimator at $k = 150$ gives $\hat{\alpha} = 1.39 \in (1, 2]$ ($95\%$ CI $= [1.17, 1.61]$, `FiniteMeanInfiniteVariance`), and [`hill_stability_across_k`](crates/stochology/src/vital.rs) verifies across $k \in \{50, 100, 150, 200, 250\}$ that the entire stability envelope $[1.36, 1.87]$ remains strictly inside $(1, 2]$ (`[Measured]`), showing why Gaussian variance formulas fail on catastrophic insurance losses.
- **Cross-Axis Hill–Holling Coupling (`HollingTailRuptureAudit`, `[Simulated]`):** Under cluster design effect $\mathrm{DEFF}_{\mathrm{tail}} = 10.10$ ($k_{\mathrm{eff}} = 11.9$), plug-in evaluation inflates the expected rupture probability by **$+32.3\%$** ($0.0219$ vs. $P_{\mathrm{true}} = 0.0166$) with an **$80.3\times$** $90\%$ confidence interval dispersion ($[0.00083, 0.06704]$), corrected by the second-order Jensen-debiased estimator (`P_debiased = 0.0119`).
- **NYC & Houston Conditional Positive-Delay ($\text{Delay} > 0$) Tails Across Full Untruncated CSVs (`nyc_flights_2013.csv`, `nyc_flights_2023.csv`, `nyc_flights_2013_h1.csv`, `nyc_flights_2023_h1.csv`, & `houston_flights_2011.csv`, `[Measured]`):** Evaluating the full untruncated flight files with proportional top-5% positive tail thresholds $k = \lfloor 0.05\,n_+ \rfloor$ and explicit `NA`/cancellation accounting (`count_flight_csv_rows_and_missing`) shows a statistically significant decadal tail thickening with non-overlapping 95% confidence intervals from 2013 ($\hat{\alpha} = 2.89, 95\%\text{ CI} = [2.78, 3.00], k = 2{,}616, n = 138{,}375, n_+ = 52{,}330, \text{total\_rows} = 141{,}193, \text{skipped\_NA} = 2{,}818$, `[Measured]`) to 2023 ($\hat{\alpha} = 2.27, 95\%\text{ CI} = [2.18, 2.36], k = 2{,}430, n = 173{,}566, n_+ = 48{,}602, \text{total\_rows} = 176{,}739, \text{skipped\_NA} = 3{,}173$, `[Measured]`), replicated out-of-time in H1 ($\hat{\alpha}_{2013\text{ H1}} = 3.13, k = 4{,}033, n = 188{,}971, n_+ = 80{,}674, \text{total} = 195{,}583, \text{NA} = 6{,}612$ vs. $\hat{\alpha}_{2023\text{ H1}} = 2.39, k = 4{,}648, n = 249{,}252, n_+ = 92{,}972, \text{total} = 258{,}613, \text{NA} = 9{,}361$, `[Measured]`) and out-of-distribution in Houston 2011 ($\hat{\alpha} = 2.41, k = 5{,}346, n = 223{,}874, n_+ = 106{,}920, \text{total} = 224{,}591, \text{NA} = 717$, `[Measured]`).

## 5.6 The same battery on a national air-transport archive

The five sealed flight tables in `data/*.csv` are North American: three New York departure windows and one Houston
window. The Brazilian mesh is distributed as a separate open archive — the National Civil Aviation Agency's
statistical transport series, published as CSV and naturally combined with a live tracking feed `[Literature]` —
and the reason for naming it explicitly is methodological rather than patriotic: it is a second institutional
producer, with its own reporting thresholds, its own airport taxonomy and its own missingness pattern, so it
exercises the archive-to-population inference of [Chapter 34](34-sample-population-archive.md) on a different
σ-algebra of questions.

The battery of §5.3–§5.4 transfers without modification, and the order in which it should be run is the point:

1. **Declare the archive boundary before computing anything.** Which carrier-months are present, which scheduled
   services are absent, which airports appear only after a certain year. The deltas that matter are computed inside
   the declared window; a tail index estimated across a boundary shift estimates the boundary.
2. **Estimate the tail on the declared support.** Hill's estimator $\hat{lpha}_{k,n}$ is a function of the number
   of upper-order statistics retained, so the plateau — not the single value at a chosen $k$ — is the object to
   report. Compare it with the New York and Houston plateaus: a *different* $\hat{lpha}$ is informative, and a
   suspiciously identical one usually means the same cancellation policy is being measured.
3. **Measure concentration as a topology, not a mean.** Hub structure is a statement about the share of movements
   carried by a few nodes, and the right summary is a concentration curve plus the betweenness ordering of
   [Chapter 53](53-layer-macro-coupled-network-contagion-order.md), not an average delay.
4. **Test the cascade separately from the level.** A delay that propagates is a queue phenomenon with the Kingman
   multiplier of [Chapter 47](47-irreversible-time-vital-margin.md); a delay that does not propagate is a local
   shock. The two have different tail behaviour and the same mean.

The same discipline governs the two graphical environments in which this material is usually taught. A no-code
statistics front end (Orange) and a spreadsheet-adjacent statistical package (JASP) both make it easy to run a
test whose assumptions are invisible: a $t$-test on a heavy-tailed delay distribution, a correlation on a
non-randomly-missing pair, a $p$-value reported without the number of comparisons behind it. The audit questions
are the ones this treatise has derived, and they belong on a written checklist rather than in an analyst's memory:
which population was sampled, what was dropped by the join, how many models were tried before the reported one,
and whether the reported interval is a confidence interval or a credible interval. A tool that will not answer
those four questions about its own output has automated the production of Type III error.

The national archive is **not** sealed in `data/SHA256SUMS` in this edition, and the reason is a provenance
commitment rather than an oversight: a dataset enters the sealed manifest only with a pinned digest, a documented
retrieval path and a stated schema contract, because a table whose upstream can change silently is not evidence.
Until that is in place, every quantitative claim in this section remains `[Literature]`, and the measured tails of
this laboratory remain the four North American windows.

---

## Exercises

1. **Jensen Correction in `StochologicalAgent`:** Inspect [`StochologicalAgent::act`](crates/stochology/src/engine.rs#L291-L368) in `engine.rs`. How does `jensen_correction = (vital.expected_eval.sqrt() - vital.sample_mean).max(0.0)` protect shift capacity when cluster demand has high right-skewness?
2. **Hill Threshold Sensitivity:** In `run_chapter_5`, evaluate `VitalMarginState::from_heavy_tails_and_asymmetry(&danish, k, ...)` for $k \in \lbrace 50, 100, 150, 200\rbrace$. Why does every choice in this range confirm $1 < \hat{\alpha} < 2$?

