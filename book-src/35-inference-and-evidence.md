# Inference and Evidence

> Foundation chapter of Part III. It presupposes
> [Chapter 11](11-sequences-limits-continuity.md) and
> [Chapter 15](15-probability-as-a-measure.md). Its formal content is worked out in
> [Chapter 44](44-underdetermination-partial-identification.md).

## By the end of this chapter you can

1. Distinguish an estimate, its interval, and the claim the interval supports.
2. Say what a p-value is not, and what must be added before a test means anything.
3. Separate the question *is the effect nonzero* from *is the effect large enough to matter*, and
   say which one a decision requires.

## 1. Estimators and their procedural properties

An estimator is a mathematical rule that maps a sample of empirical data to a numerical summary. Its properties
belong strictly to the procedure across hypothetical repeated runs, not to any single historical realization:
- **Bias** ($\mathbb{E}[\hat{\theta}] - \theta^*$) is the systematic offset between the expected output of the rule
  and the true parameter.
- **Variance** ($\mathbb{E}[(\hat{\theta} - \mathbb{E}[\hat{\theta}])^2]$) is the sampling instability of the rule
  across parallel draws.
- **Admissibility** dictates that no competing rule achieves uniformly lower expected loss across all states of nature.

A point estimate quoted without its sampling distribution is half a result. More fundamentally, the estimand itself
is not always a single point. When data are subject to non-random selection, non-response, or unmeasured confounding,
the estimand licensed by empirical reality is an **identification region** — a mathematical interval. Treating a set
as a failure of method is a category error: where the evidence identifies only a set, the set is the honest scientific
conclusion.

## 2. Confidence intervals and interval width as finding

A classical $95\%$ confidence interval is the set of parameter values that would not have been rejected by a
significance test at level $\alpha = 0.05$. Read correctly, this definition dissolves three pervasive confusions:
1. *The interval is not a Bayesian probability statement about the parameter.* In frequentist foundations, the
   parameter $\theta^*$ is a fixed constant, not a random variable; it is either inside the interval or outside it.
   The $95\%$ belongs exclusively to the performance of the apparatus across repeated applications.
2. *The width of the interval is the primary operational finding.* A wide confidence interval does not represent
   "noisy precision"; it demonstrates that the available evidence cannot differentiate between a massive benefit
   and a catastrophic loss. Dropping the interval and publishing only the point estimate is the single most
   destructive distortion in applied science.
3. *The interval is strictly conditional upon the assumed likelihood model.* If the data-generating mechanism
   violates the model's structural assumptions (e.g., heavy tails or unmodeled clustering), the nominal coverage
   guarantee is completely void.

## 3. P-values, statistical power, and Mayo's severity principle

A p-value answers a narrow counterfactual question: *supposing that the null hypothesis were true, how probable
would it be to observe a test statistic at least as extreme as the one calculated from the data?* A p-value is
not the probability that the null hypothesis is true, nor does its complement measure the probability of the
alternative.

What an empirical test communicates depends fundamentally on whether the procedure was capable of detecting a flaw.
A non-significant test result produced by a study with low statistical power provides zero evidence that the effect
is absent; it merely confirms that the sample was too small to see it. **Power is the capacity to have failed**,
and a test that cannot fail is an instrument of institutional self-deception.

To evaluate evidence after the data are observed, Deborah Mayo formulated the **Severity Principle**. For a
substantive claim $H$ asserting that an effect exceeds a critical threshold ($\theta > \theta_{\mathrm{crit}}$),
severity asks: *how probable was it that this experimental design would have produced a result less favorable
to $H$ than the observed data, if $H$ were false?*
- High severity means the claim survived a grueling probe designed to expose its specific vulnerabilities.
- Low severity means the test was so lax that almost any state of affairs would have passed it, rendering the
  pass uninformative.

Severity evaluates power at the specific boundary that matters to decision-makers, providing an objective error-statistical
foundation for belief.

## 4. Underdetermination and Manski partial-identification bounds

The deepest epistemic divide in inference separates a parameter that is difficult to estimate due to sampling
variance from a parameter that the data **cannot uniquely identify** even with an infinite sample size.

Consider a binary treatment $D \in \{0, 1\}$ with potential outcomes $Y(1)$ and $Y(0)$ bounded in $[0, 1]$.
Charles Manski (1989, 2003) proved that under arbitrary non-random selection into treatment, the Average Treatment
Effect $\mathrm{ATE} = \mathbb{E}[Y(1) - Y(0)]$ cannot be identified as a point. Because $Y(0)$ is unobserved for
treated units and $Y(1)$ is unobserved for untreated units, the **Manski worst-case bounds** are:

$$\mathrm{ATE} \in \Bigl[\mathbb{E}[Y \mid D=1]P(D=1) - P(D=1) - \mathbb{E}[Y \mid D=0]P(D=0), \; \mathbb{E}[Y \mid D=1]P(D=1) + P(D=0) - \mathbb{E}[Y \mid D=0]P(D=0)\Bigr] \qquad \text{\texttt{[Assumed]}}.$$

The width of this non-parametric identification interval is always exactly $1.0$ `[Assumed]`. An infinite sample size
narrows sampling error to zero, but leaves the identification width unchanged. Narrowing the interval further requires
substantive structural assumptions (e.g., monotone treatment response or instrumental variables), each of which must
be audited.

When treatment evaluation suffers from attrition (loss to follow-up), David Lee (2009) established sharp **attrition
bounds** under the assumption that treatment assignment affects attrition monotonically: the excess retention in
one arm is trimmed at the upper and lower quantiles of the outcome distribution, bounding the treatment effect
without parametric imputation.

When constructing confidence intervals for a partially identified parameter $\theta_0 \in [\theta_L, \theta_U]$,
standard two-sided intervals are overly conservative. Guido Imbens and Charles Manski (2004) derived the modified
critical value $c_\alpha$ that satisfies:

$$\Phi\left(c_\alpha + \frac{\sqrt{n}(\hat{\theta}_U - \hat{\theta}_L)}{\hat{\sigma}}\right) - \Phi(-c_\alpha) = 1 - \alpha \qquad \text{\texttt{[Assumed]}}.$$

As the identification interval width expands relative to sampling error ($\sqrt{n}(\Delta) \to \infty$), the critical
value $c_\alpha$ smoothly transitions from the two-sided value $z_{1-\alpha/2}$ (for point identification, $1.96$
at $95\%$) down to the one-sided critical value $z_{1-\alpha}$ ($1.645$ at $95\%$), preserving exact coverage
probabilities for the true parameter $\theta_0$.

Under competing risks where cause of failure is censored by an alternative hazard, Arthur V. Peterson (1976)
proved that the joint survival $S_T(t) = \mathbb{P}(\min(T_1, T_2) > t)$ satisfies sharp Fréchet bounds:

$$\max\bigl(0,\, S_1(t) + S_2(t) - 1\bigr) \le S_T(t) \le \min\bigl(S_1(t),\, S_2(t)\bigr) \qquad \text{\texttt{[Assumed]}}.$$

## 5. Epigraphy and legal standards of proof

Classical epigraphers formalized partial identification on stone two millennia ago. Under the **Leiden Convention**
`[Literature]`, when a carved inscription is fractured, the physical dimensions of the break dictate the minimum
and maximum number of missing characters, printed strictly within brackets `[...]`. Fabricating a single point
restoration without brackets fabricates false certainty; printing the bracketed bounds — or setting the *crux
desperationis* ($\dagger\dots\dagger$) when no hypothesis passes severe scrutiny — is partial identification
preserved in marble.

In legal epistemology, L. Jonathan Cohen's **Gatecrasher Paradox** `[Literature]` demonstrates why
bare statistical probability cannot sustain an actionable verdict. Suppose $499$ attendees paid for
admission and $501$ climbed the fence at a $1{,}000\text{-seat}$ rodeo `[Assumed]`. For any
randomly selected spectator, the probability that they gatecrashed is $0.501 > 0.50$, satisfying the
literal numerical threshold of "preponderance of the evidence". Yet no court can convict an
individual based solely on this aggregate ratio without individualized forensic testimony. The
Keynesian evidential weight $V(x, q)$ (
`epistemic.rs::EpistemicMarginState::keynes_weight_from_n_eff`) for that individual is zero:
statistical correlation without causal coverage cannot justify the coercive imposition of liability.

## Bridge

Severe testing, partial-identification bounds, and confidence adjustments are formalised in
[Chapter 44](44-underdetermination-partial-identification.md) as
`epistemic.rs::EpistemicMarginState::from_ate_bounds`, which evaluates Manski ATE bounds alongside
their adjusted interval; `epistemic.rs::EpistemicMarginState::from_lee_attrition_bounds` evaluates
David Lee's sharp trimming bounds;
`epistemic.rs::EpistemicMarginState::imbens_manski_critical_value` computes the parameter coverage
critical value $c_\alpha$; and
`epistemic.rs::EpistemicMarginState::passes_severe_boundary` issues the Mayo severity audit against the critical policy threshold.
Evidential weight under partial coverage is governed by `epistemic.rs::EpistemicMarginState::keynes_weight_from_n_eff`.
