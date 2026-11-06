# Integrals and Accumulation

> Fifth chapter of the mathematical block in Part II. The subject is accumulation, and the reason an
> expectation is an integral rather than a sum.

## By the end of this chapter you can

1. Read a definite integral as an accumulated quantity rather than as an area formula.
2. Explain why an expectation is a weighted integral, and what the weights have to satisfy.
3. Use the tail-integral identity to compute an average from the probability of exceeding each level.

## 1. Accumulation and Integration

While the derivative measures local rates of change, the integral measures total accumulated substance.
If $f(t)$ represents an instantaneous rate — gallons per minute, financial claims per day, or metabolic
toxin load per hour — the definite integral accumulates that rate across an interval $[a, b]$:

$$I = \int_a^b f(t)\,dt = \lim_{\Delta t_i \to 0} \sum_{i=1}^n f(t_i^*) \Delta t_i.$$

Integration satisfies two fundamental structural properties:

- **Linearity**: $\int_a^b \bigl(\alpha f(t) + \beta g(t)\bigr)\,dt = \alpha \int_a^b f(t)\,dt + \beta \int_a^b g(t)\,dt$.
- **Interval additivity**: $\int_a^c f(t)\,dt = \int_a^b f(t)\,dt + \int_b^c f(t)\,dt$.

In vital reserve dynamics, interval additivity is what governs survival. If an institution holds a vital
reserve stock $R_0$ subject to stochastic drawdown rate $D(t)$, the time to exhaustion $\mathrm{TTS}$ (time-to-survival)
is the solution to an integral equation:

$$R_0 - \int_0^{\mathrm{TTS}} D(t)\,dt = 0.$$

Survival is an accumulated integral process, not an instantaneous snapshot.

## 2. Expectation as a Lebesgue Integral

In rigorous probability, expectation is formulated as a Lebesgue integral of a measurable random variable
$X : \Omega \to \mathbb{R}$ against the probability measure $\mathbb{P}$:

$$\mathbb{E}[X] = \int_\Omega X(\omega)\,d\mathbb{P}(\omega).$$

When $X$ admits a continuous probability density function $p(x)$, this specializes to $\mathbb{E}[X] = \int_{-\infty}^\infty x p(x)\,dx$.
The weighting measure must satisfy the Kolmogorov axioms: non-negativity ($p(x) \ge 0$) and total unit mass
($\int p(x)\,dx = 1$). Any weighting scheme that violates unit mass is not a probability expectation, and
any weighting scheme that permits negative weights violates coherence.

For non-negative random variables ($X \ge 0$), Fubini's theorem yields the fundamental **tail-integral identity**:

$$\mathbb{E}[X] = \int_0^\infty \mathbb{P}(X > t)\,dt.$$

This identity provides a powerful geometric insight: the expected value is the integral of the survival function
$S(t) = \mathbb{P}(X > t)$. If a distribution is thin-tailed (exponential or Gaussian), $S(t)$ drops exponentially
to zero, making the tail's contribution to the integral negligible. But if $S(t) \sim t^{-\alpha}$ with $\alpha \le 1$,
the area under $S(t)$ diverges to infinity: $\mathbb{E}[X] = \infty$. The expectation is driven entirely by the
unbounded upper tail.

## 3. Limit-Integral Exchange and Fat-Tailed Failures

In analysis, exchanging the order of a limit and an integral, $\lim_{n\to\infty} \int f_n = \int \lim_{n\to\infty} f_n$,
is governed by Henri Lebesgue's **Dominated Convergence Theorem**. The exchange is valid only if there exists
an integrable dominating function $g$ such that $|f_n(x)| \le g(x)$ for all $n$.

In systems subject to heavy tails, dominated convergence fails. When probability mass escapes toward infinity
(as in distributions with tail index $\alpha \le 1$), sample running averages $\bar{X}_n = \frac{1}{n}\sum_{i=1}^n X_i$
do not converge to any finite population constant. The limit of the sample averages is not the average of the
underlying process. Relying on sample sums to estimate tail risk without verifying uniform integrability is a
fatal epistemic blunder.

## 4. Coherent Risk Measures: Expected Shortfall vs. Value-at-Risk

How should an institution integrate tail risk over thresholds? Philippe Artzner et al. (1999) formalized the
axioms of **coherent risk measures** (monotonicity, subadditivity, positive homogeneity, translation invariance).
Subadditivity, $\rho(X + Y) \le \rho(X) + \rho(Y)$, requires that diversifying across two exposures cannot increase
total risk.

The conventional industry metric, Value-at-Risk at level $\alpha \in (0, 1)$ ($\mathrm{VaR}_\alpha(X) = \inf\{x : \mathbb{P}(X \le x) \ge \alpha\}$),
evaluates only a single quantile threshold. $\mathrm{VaR}$ is not subadditive: combining two independent fat-tailed
risks can produce a portfolio with a higher $\mathrm{VaR}$ than the sum of the individual $\mathrm{VaR}$ values, penalizing
diversification.

In contrast, **Expected Shortfall** (Conditional Value-at-Risk, $\mathrm{CVaR}_\alpha$) integrates all losses
exceeding $\mathrm{VaR}_\alpha$:

$$\mathrm{CVaR}_\alpha(X) = \frac{1}{1 - \alpha} \int_\alpha^1 \mathrm{VaR}_u(X)\,du = \mathbb{E}\bigl[X \mid X \ge \mathrm{VaR}_\alpha(X)\bigr].$$

Because Expected Shortfall is defined as an integral over the entire upper tail, it satisfies subadditivity,
rewards diversification, and penalizes heavy-tailed extreme losses that standard quantiles ignore.

## 5. Partial Identification and Worst-Case Bounds

When data are missing or selective, the integration domain is incomplete. Under Charles Manski's partial identification
framework, when an outcome $Y \in [y_{\min}, y_{\max}]$ is unobserved for a fraction $1 - p$ of the population,
the true expectation is bounded by integrating the unobserved mass at the extrema:

$$\mathbb{E}[Y] \in \bigl[p \mathbb{E}[Y \mid R=1] + (1 - p)y_{\min},\; p \mathbb{E}[Y \mid R=1] + (1 - p)y_{\max}\bigr].$$

Integrating over worst-case bounds guarantees that the institution's reserve sizing remains robust to arbitrary,
adversarial selection in the missing data.

## Bridge

[Distribution and Moment](31-distribution-and-moment.md) and [The Non-Identical, Scale Invariance, Convexity, and Heavy Tails](46-non-identical-asymmetry-heavy-tails.md)
deploy these integral identities to evaluate scoring rules across continuous thresholds. In the Stochology library,
`epistemic.rs::from_ate_bounds` integrates worst-case bounds over missing counterfactuals, while `vital.rs::crps_empirical`
computes the Continuous Ranked Probability Score by integrating squared threshold forecast errors across the full
support of the empirical distribution.

[Linear Algebra](14-linear-algebra.md) carries the same accumulation into finite dimensions —
operators, rank, projections, and the effective dimension of an archive with more columns than
evidence.
