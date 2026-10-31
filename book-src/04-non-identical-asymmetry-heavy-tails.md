# Chapter 4 — The Non-Identical, Scale Invariance, Convexity, and Heavy Tails

Even if an archive had zero missing records, how does a physical or social phenomenon enter a table of numbers in the first place?

**Mediating Filter II** governs the passage from the qualitative, heterogeneous world to the discrete archive. Its first constitutive tension (**Tension 3**, implemented in [`crates/stochology/src/asymmetry.rs`](../crates/stochology/src/asymmetry.rs)) confronts **formal equivalence and the tyranny of the lone mean** against **the non-identical, measurement scale invariance, convex harm, and heavy-tailed extremes** (Stevens, 1946; Jensen, 1906; Adorno, 1966; Georgescu-Roegen, 1971; Hill, 1975; Desrosières, 1993; Porter, 1995; Hacking, 2002; Gneiting & Raftery, 2007).

---

## 4.1 The Non-Identical and the Dialectical Penumbra

In *Negative Dialectics* (1966), Theodor W. Adorno observed that conceptual and statistical classification operates by forcing heterogeneous particulars under an identity sign ("the non-identical," *das Nichtidentische*). A few years later, Nicholas Georgescu-Roegen (*The Entropy Law and the Economic Process*, 1971) distinguished **arithmomorphic concepts**—discrete numbers separated by an empty void—from **dialectical concepts** (justice, clinical severity, ecological distress, delay disruption), which are surrounded by a continuous **dialectical penumbra** $[a, b]$ where opposites overlap.

When a bureaucracy replaces a continuous phenomenon $X$ with a rigid binary indicator $\mathbb{I}\\{X \ge c\\}$ (for instance, defining a flight as "on time" at $14.9$ minutes and "delayed" at $15.0$ minutes, or a household as "eligible" vs. "ineligible" at an arbitrary poverty line), how much of the real population sits inside the contested boundary zone $[a, b]$?

In [`DialecticalPenumbra`](../crates/stochology/src/asymmetry.rs#L17-L77), `DialecticalPenumbra::evaluate(xs, lower_a, upper_b)` computes both the **penumbra mass** $\mathbb{P}(a \le X \le b)$ and the **cutoff instability** $\bigl|\mathbb{P}(X \ge a) - \mathbb{P}(X \ge b)\bigr|$ across the penumbra. Whenever the penumbra mass is non-negligible, reporting a single binary classification rate hides the boundary sensitivity of the category itself.

---

## 4.2 Stevens's Scale Hierarchy and the Forbidden Ordinal Mean

In his landmark 1946 paper *On the Theory of Scales of Measurement*, S. S. Stevens proved that a statistical summary is meaningful only if its conclusions are invariant under the admissible transformations $\phi$ of its measurement scale:

| Scale Type | Admissible Transformation $\phi(x)$ | Invariant Statistics | Illegitimate Operations |
| :--- | :--- | :--- | :--- |
| **Nominal** | Any bijection | Mode, frequency, entropy | Mean, median, order |
| **Ordinal** | Any strictly increasing $\phi$ ($x < y \implies \phi(x) < \phi(y)$) | Median, quantiles, Mann–Whitney $U = \mathbb{P}(A > B) + \frac{1}{2}\mathbb{P}(A = B)$ | Arithmetic mean $\bar{x}$, standard deviation |
| **Interval** | Positive affine $\phi(x) = a x + b$ ($a > 0$) | Mean, variance, Pearson correlation | Coefficient of variation $\sigma / \mu$, ratios |
| **Ratio** | Positive similarity $\phi(x) = a x$ ($a > 0$) | All of the above + geometric mean, ratios, Kelly growth | — |

Institutions routinely collect **ordinal** scores—Likert satisfaction rubrics $1..5$, clinical pain scales, ESG grades, or LLM judge rubrics—and average them as if the gap between $1$ and $2$ were identical to the gap between $4$ and $5$. In [`OrdinalScaleAudit`](../crates/stochology/src/asymmetry.rs#L82-L162), `OrdinalScaleAudit::evaluate(group_a, group_b, monotone_phi)` proves two facts simultaneously:
1. A strictly increasing convex transformation $\phi(x) = x^2$ can **invert the arithmetic mean ranking** ($\bar{A} > \bar{B}$ while $\overline{\phi(A)} < \overline{\phi(B)}$, `mean_ranking_reversed = true`);
2. The ordinal Mann–Whitney dominance probability $U(A, B) = \mathbb{P}(A > B) + \frac{1}{2}\mathbb{P}(A = B)$ is **100% invariant** (`ordinal_invariance_error = 0.0`).

---

## 4.3 Jensen's Convexity Gap and the Continuous Ranked Probability Score (CRPS)

Even on a genuine ratio scale (megawatts of electricity demand, minutes of delay, millions of kroner in fire damage), reporting only the sample mean $\mathbb{E}[X]$ commits **the flaw of averages** whenever harm $f(x)$ is convex ($f''(x) > 0$). By Johan Jensen's (1906) inequality:

$$\Delta\_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X]) \ge 0$$

For a quadratic loss $f(x) = x^2$, $\Delta\_J = \mathrm{Var}(X)$. In [`NonlinearSummary`](../crates/stochology/src/asymmetry.rs#L166-L243), every nonlinear evaluation reports the full distribution summary:
- the sample mean $\mathbb{E}[X]$, the median $q\_{50}$, and the tail quantiles $(q\_{05}, q\_{95})$;
- the evaluation at the mean $f(\mathbb{E}[X])$, the expected evaluation $\mathbb{E}[f(X)]$, and **Jensen's Gap** $\Delta\_J$;
- Gneiting & Raftery's (2007) strictly proper **Continuous Ranked Probability Score** ($\mathrm{CRPS}$) of the empirical predictive distribution $F$:

$$\mathrm{CRPS}(F, y) = \mathbb{E}\_F|X - y| - \frac{1}{2}\mathbb{E}\_F|X - X'|$$

---

## 4.4 Bruce M. Hill's (1975) Tail Index and Moment Existence Regimes

Why can $\Delta\_J$ grow by orders of magnitude as sample size increases? Because many physical, financial, and operational losses follow **Pareto power-law tails** $\mathbb{P}(X > x) \sim (x / x\_{\min})^{-\alpha}$. Using the top $k + 1$ order statistics $X\_{(n)} \ge X\_{(n-1)} \ge \dots \ge X\_{(n-k)} > 0$, Bruce M. Hill's (1975) maximum-likelihood tail estimator ([`HillTailEstimate`](../crates/stochology/src/asymmetry.rs#L261-L321)) is:

$$\hat{\alpha}\_{k,n} = \left(\frac{1}{k}\sum\_{i=1}^k \ln\frac{X\_{(n - i + 1)}}{X\_{(n - k)}}\right)^{-1}$$

`HillTailEstimate::evaluate` classifies the distribution into three moment regimes ([`TailMomentRegime`](../crates/stochology/src/asymmetry.rs#L247-L256)):
- $\hat{\alpha} \le 1$ (`InfiniteMeanAndVariance`): even the population mean is infinite;
- $1 < \hat{\alpha} \le 2$ (`FiniteMeanInfiniteVariance`): the population mean exists, but the population variance $\mathrm{Var}(X) = \infty$, so Gaussian $\mu \pm 2\sigma$ intervals are mathematically meaningless;
- $\hat{\alpha} > 2$ (`FiniteVariance`): both mean and variance are finite.

---

## 4.5 Chapter 4 Laboratory: Real Data on Danish Fire Losses and NYC Flight Delays

Running Chapter 4 (`cargo run --release --bin stochology -- --chapter 4`) evaluates [`asymmetry.rs`](../crates/stochology/src/asymmetry.rs) on two real-world datasets (`data/danish_fire.csv`, $n = 2{,}167$, and `data/nyc_flights_2013.csv` / `data/nyc_flights_2023.csv`, Measured):

```text
=== Chapter 4: Asymmetry, Scale Invariance & Heavy Tails (Filter II · Tension 3) ===
  Danish Fire (danish_fire.csv, n=2167): mean=3.39, median=1.78, q95=9.97, Jensen_gap=72.34, Hill_alpha=1.446 (FiniteMeanInfiniteVariance)
  NYC Flights (nyc_flights_2013/2023.csv): delay Jensen_gap=2536.0 min^2, penumbra_mass(10..20m)=8.2%, ordinal_err=0.0e0
```

- **Danish Fire Insurance Losses (`danish_fire.csv`, $n = 2{,}167$, Measured in millions of DKK):** The median claim is $1.78\text{M}$, the mean is $3.39\text{M}$, and the 95th percentile reaches $9.97\text{M}$. Under quadratic reinsurance stress $f(x) = x^2$, the evaluation at the mean is $f(\mathbb{E}[X]) = 3.39^2 = 11.49$, whereas the true expected stress is $\mathbb{E}[X^2] = 83.83$—a **Jensen's Gap of $\Delta\_J = +72.34\text{M}^2$** (over **$630\\%$** of $f(\mathbb{E}[X])$!). Hill's estimator at $k = 120$ gives $\hat{\alpha} = 1.446 \in (1, 2]$ (`FiniteMeanInfiniteVariance`), proving why Gaussian variance formulas fail on catastrophic insurance losses.
- **NYC Flight Delays (`nyc_flights_2013.csv` & `nyc_flights_2023.csv`, Measured):** Quadratic passenger connection harm exhibits a Jensen's Gap of $\Delta\_J = 2{,}536.0\text{ min}^2$, while $8.2\\%$ of flights sit inside the $10\text{--}20\text{ min}$ dialectical penumbra around the FAA's $15$-minute cutoff.

---

## Exercises

1. **Jensen Correction in `StochologicalAgent`:** Inspect [`StochologicalAgent::act`](../crates/stochology/src/engine.rs#L274-L283) in `engine.rs`. How does `jensen_correction = (nonlin.expected_eval.sqrt() - nonlin.sample_mean).max(0.0)` protect shift capacity when cluster demand has high right-skewness?
2. **Hill Threshold Sensitivity:** In `run_chapter_4`, evaluate `HillTailEstimate::evaluate(&fire_losses, k)` for $k \in \\{50, 100, 120, 200\\}$. Why does every choice in this range confirm $1 < \hat{\alpha} < 2$?

