# Distribution and Moment

> Foundation chapter of Part III. Its formal content is worked out in
> [Chapter 46](46-non-identical-asymmetry-heavy-tails.md).

## By the end of this chapter you can

1. Describe a quantity by its distribution rather than by its average, and say what the average
   discards.
2. Read expectation, variance and covariance as summaries with different failure modes.
3. Recognise a heavy tail from data, and say why it breaks the arithmetic you were taught first.

## 1. From a number to a distribution

A random variable is a formal rule mapping outcomes of a sample space to numerical values; its probability
distribution is the exhaustive mathematical description of that rule. The word *exhaustive* is load-bearing.
Everything that the variable can physically do — every extreme excursion, every asymmetry, every tail decay rate,
and every non-linear coupling with other variables — is encoded within the distribution and in nothing less.

Replacing an entire probability distribution with a single scalar average is the most pervasive and destructive
simplification in applied science and policy. As demonstrated in [The Average Is Not the Story](02-the-average-is-not-the-story.md),
a distribution with an arithmetic mean of eleven and a median of one will utterly annihilate an organization whose
reserves were sized on the mean when the single catastrophic draw occurs. What this chapter formalizes is the
mathematical machinery of moments, their structural failure modes, and the non-linear gaps that appear when
convex loss functions confront physical dispersion.

## 2. Expectation, variance, covariance, and their failure modes

The mathematical expectation $\mathbb{E}[X]$ is a probability-weighted integral over all *counterfactual possibilities*,
not a descriptive summary of past observations. This distinction is critical: an expectation can sit at a value that
has never occurred in the historical record and may be physically impossible (e.g., $2.4$ children per household).
Furthermore, when the underlying data-generating process undergoes a structural shift or exhibits heavy tails, sample
averages do not converge to the expectation at the rates intuition assumes.

The variance $\mathrm{Var}(X) = \mathbb{E}[(X - \mathbb{E}[X])^2]$ measures dispersion around the mean, but it
inherits and squares every sensitivity of the expectation. By computing deviations to the second power, variance
heavily penalizes extreme outliers. In heavy-tailed distributions characteristic of operational risk, disaster
losses, and network delays, the theoretical variance can be literally infinite while every individual observation in
the database is finite. Treating sample standard deviations as meaningful measures of risk in such regimes is a
dangerous mathematical fallacy.

The covariance $\mathrm{Cov}(X, Y) = \mathbb{E}[(X - \mathbb{E}[X])(Y - \mathbb{E}[Y])]$ quantifies linear co-movement,
answering a far narrower question than decision-makers assume. Two variables can exhibit massive covariance because
$X$ causes $Y$, because $Y$ causes $X$, because an unmeasured common shock drives both, or because the archive's
selection filter admitted only units possessing high values of both. A covariance matrix carries zero causal authority;
the transition from *observed co-movement* to *interventional response* requires structural causal identification
([Chapter 39](39-cause-intervention-counterfactual.md)).

## 3. Heavy tails, the Hill estimator, and Pickands-Balkema-de Haan

Distributions are classified fundamentally by how their upper tails decay. Under a power-law tail
$\mathbb{P}(X > x) \sim L(x) x^{-\alpha}$, the magnitude of the tail index $\alpha > 0$ governs which moments
exist:
- **$\alpha > 2$**: The variance and mean are both finite. The classical Central Limit Theorem holds, and sample
  averages converge at standard $O(1/\sqrt{n})$ rates.
- **$1 < \alpha \le 2$**: The mean is finite, but the theoretical variance is infinite. The sample average converges
  extremely slowly, while sample variances wander erratically and grow without bound as more data is collected.
- **$\alpha \le 1$**: Even the theoretical mean is infinite. The arithmetic average of a sample does not settle to
  a constant; it drifts upward indefinitely as larger historical records encounter larger catastrophic draws.

Under the Pickands-Balkema-de Haan theorem of extreme value theory, the distribution of excesses over a sufficiently
high threshold $u$ converges asymptotically to the Generalized Pareto Distribution (GPD):

$$G(y; \xi, \beta) = 1 - \left(1 + \frac{\xi y}{\beta}\right)^{-1/\xi} \qquad \text{\texttt{[Assumed]}},$$

where the shape parameter $\xi = 1/\alpha$ determines tail heaviness ($\xi > 0$ indicating heavy Fréchet-type tails).
To estimate $\alpha$ directly from sample order statistics $X_{(1)} \le X_{(2)} \le \dots \le X_{(n)}$, the
classical **Hill estimator** evaluates the $k$ largest observations:

$$\hat{\alpha}_k = \frac{k}{\sum_{i=1}^k \ln\left(X_{(n-i+1)} / X_{(n-k)}\right)} \qquad \text{\texttt{[Assumed]}}.$$

In real-world data, the estimated $\hat{\alpha}_k$ varies dramatically with the choice of cut-off $k$. A stable
"Hill plot" plateau is the only valid empirical evidence for a stable tail index; reporting a single tail estimate
without an interval of stability across varying $k$ is deceptive.

## 4. Proper scoring rules and CRPS decomposition

Because point forecasts discard distribution information, evaluating probabilistic predictions requires **proper
scoring rules** that reward both calibration (honesty) and sharpness (resolution). Tilmann Gneiting and Adrian Raftery
formalized the **Continuous Ranked Probability Score (CRPS)** for a predictive cumulative distribution $F$ and an
observed scalar outcome $y$:

$$\mathrm{CRPS}(F, y) = \int_{-\infty}^{\infty} \bigl(F(x) - \mathbf{1}_{\{x \ge y\}}\bigr)^2\, dx \qquad \text{\texttt{[Assumed]}}.$$

CRPS measures the integrated squared distance between the predicted probability distribution and the empirical
Heaviside step function of the realization. It generalizes mean absolute error to probabilistic forecasts while
retaining the same units as the underlying variable. Crucially, CRPS admits a clean mathematical decomposition:

$$\mathrm{CRPS} = \text{Reliability (Miscalibration)} - \text{Resolution} + \text{Uncertainty} \qquad \text{\texttt{[Assumed]}}.$$

An honest forecasting institution strives to minimize CRPS by improving resolution while holding reliability near zero,
rather than gaming simple point accuracy metrics.

## 5. Convexity, Jensen's inequality, and the Jensen gap

When a decision system converts a physical quantity $X$ into financial cost, casualty loss, or waiting delay, the
transformation function $\phi(x)$ is rarely linear. In almost all real-world processes, damage accelerates: a
doubling of flood height produces a quadrupling of structural damage; a ten percent surge in arrival rate near
capacity produces a hundred percent increase in queue length.

A function $\phi$ is strictly convex if any secant line between two points lies strictly above the curve.
**Jensen's inequality** states that for any convex loss function $\phi$ and any non-degenerate random variable $X$:

$$\phi\bigl(\mathbb{E}[X]\bigr) \le \mathbb{E}\bigl[\phi(X)\bigr] \qquad \text{\texttt{[Assumed]}}.$$

The difference between the expected loss and the loss evaluated at the expected input is the **Jensen gap**:

$$\Delta_J = \mathbb{E}\bigl[\phi(X)\bigr] - \phi\bigl(\mathbb{E}[X]\bigr) \ge 0 \qquad \text{\texttt{[Assumed]}}.$$

The operational consequence is severe: an institution that sizes its vital reserves, staffing, or financial liquidity
against the *average* load $\mathbb{E}[X]$ will systematically underfund the actual expected damage $\mathbb{E}[\phi(X)]$.
The Jensen gap $\Delta_J$ is not an optional buffer added for conservatism; it is the exact mathematical deficit
incurred by mistaking a point expectation for a distributed physical reality.

## 6. Underdetermination and competing risks bounds

When the historical archive is incomplete or subject to censoring, even the probability distribution $F$ cannot be
uniquely identified. Under dependent competing risks (e.g., patient death from disease vs death from competing
co-morbidities), Arthur V. Peterson (1976) proved that without unverifiable independence assumptions, the true
joint survival function $S_T(t) = \mathbb{P}(\min(T_1, T_2) > t)$ is bounded by sharp Fréchet bounds:

$$\max\bigl(0,\, S_1(t) + S_2(t) - 1\bigr) \le S_T(t) \le \min\bigl(S_1(t),\, S_2(t)\bigr) \qquad \text{\texttt{[Assumed]}},$$

where $S_1(t)$ and $S_2(t)$ are the observed marginal survival rates. A naive calculation that multiplies
$S_1(t) S_2(t)$ asserts dogmatic independence where the empirical archive supports only the bracketed interval.
When underdetermination is structural, the set is the only honest distribution.

## Bridge

Tail estimation, extreme value properties, and the convexity gap are formalised in
[Chapter 46](46-non-identical-asymmetry-heavy-tails.md) as
`vital.rs::from_heavy_tails_and_asymmetry`, with the Hill tail index and its 95% confidence interval computed by
`vital.rs::hill_alpha_with_ci95`. The scoring rule that evaluates full predictive distributions against realized
outcomes without collapsing to point summaries is `vital.rs::crps_empirical`.
