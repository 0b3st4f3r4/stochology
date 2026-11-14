# Inference and Evidence

<figure class="stoch-figure stoch-logo-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="800" height="132" viewBox="0 0 800 132" fill="none" role="img" aria-label="Stochology CH35 · MARTINGALE — Inference, Severity, And E-Processes">
  <defs>
    <linearGradient id="ch35-ring" x1="2" y1="78" x2="386" y2="78" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="38%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="68%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <radialGradient id="ch35-glow" cx="50%" cy="50%" r="50%">
      <stop offset="0%" stop-color="#38bdf8" stop-opacity="0.24"/>
      <stop offset="48%" stop-color="#2563eb" stop-opacity="0.14"/>
      <stop offset="80%" stop-color="#e11d48" stop-opacity="0.08"/>
      <stop offset="100%" stop-color="#090e1a" stop-opacity="0"/>
    </radialGradient>
    <linearGradient id="ch35-triad-stroke" x1="18" y1="24" x2="110" y2="104" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.85"/>
      <stop offset="50%" stop-color="var(--logo-gold, #f59e0b)" stop-opacity="0.90"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.85"/>
    </linearGradient>
    <linearGradient id="ch35-fill" x1="22" y1="30" x2="106" y2="98" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)" stop-opacity="0.26"/>
      <stop offset="48%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.28"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.26"/>
    </linearGradient>
    <linearGradient id="ch35-ink-curve" x1="20" y1="66" x2="108" y2="66" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-ink, #0f172a)"/>
      <stop offset="50%" stop-color="var(--logo-sub, #1d4ed8)"/>
      <stop offset="100%" stop-color="var(--logo-ink, #0f172a)"/>
    </linearGradient>
    <radialGradient id="ch35-pupil" cx="45%" cy="42%" r="58%">
      <stop offset="0%" stop-color="#020617"/>
      <stop offset="68%" stop-color="#0f172a"/>
      <stop offset="100%" stop-color="#451a03"/>
    </radialGradient>
    <linearGradient id="ch35-gold-mira" x1="54" y1="48" x2="84" y2="82" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#fef08a"/>
      <stop offset="45%" stop-color="#fbbf24"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <radialGradient id="ch35-gold-core" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#ffffff"/>
      <stop offset="35%" stop-color="#fde047"/>
      <stop offset="100%" stop-color="#d97706"/>
    </radialGradient>
    <radialGradient id="ch35-node-well" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="var(--logo-well-hi, var(--elevated-bg, #ffffff))"/>
      <stop offset="100%" stop-color="var(--logo-node-bg, #e2e8f0)"/>
    </radialGradient>
    <linearGradient id="ch35-ring-epi" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#7dd3fc"/>
      <stop offset="50%" stop-color="#2563eb"/>
      <stop offset="100%" stop-color="#1e3a8a"/>
    </linearGradient>
    <radialGradient id="ch35-core-epi" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#bae6fd"/>
      <stop offset="45%" stop-color="#3b82f6"/>
      <stop offset="100%" stop-color="#1d4ed8"/>
    </radialGradient>
    <linearGradient id="ch35-ring-vital" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#6ee7b7"/>
      <stop offset="50%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#065f46"/>
    </linearGradient>
    <radialGradient id="ch35-core-vital" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#a7f3d0"/>
      <stop offset="45%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#047857"/>
    </radialGradient>
    <linearGradient id="ch35-ring-gov" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#fda4af"/>
      <stop offset="50%" stop-color="#e11d48"/>
      <stop offset="100%" stop-color="#881337"/>
    </linearGradient>
    <radialGradient id="ch35-core-gov" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#fecdd3"/>
      <stop offset="45%" stop-color="#f43f5e"/>
      <stop offset="100%" stop-color="#be123c"/>
    </radialGradient>
  </defs>
  <g>
    <circle cx="64" cy="66" r="48" fill="url(#ch35-glow)"/>
    <line x1="20" y1="30" x2="108" y2="30" stroke="var(--logo-crimson, #e11d48)" stroke-width="2.4" stroke-dasharray="4 2.5" stroke-linecap="round"/>
    <path d="M 22 94 L 38 82 L 50 88 L 66 64 L 78 70 L 94 30 L 106 24" stroke="var(--logo-blue, #2563eb)" stroke-width="3.0" stroke-linecap="round" stroke-linejoin="round"/>
    <circle cx="22" cy="94" r="6.6" fill="url(#ch35-node-well)" stroke="url(#ch35-ring-epi)" stroke-width="2.4"/>
    <circle cx="22" cy="94" r="2.8" fill="url(#ch35-core-epi)"/>
    <circle cx="66" cy="64" r="6.6" fill="url(#ch35-node-well)" stroke="url(#ch35-ring-vital)" stroke-width="2.4"/>
    <circle cx="66" cy="64" r="2.8" fill="url(#ch35-core-vital)"/>
    <g>
      <circle cx="94" cy="30" r="7.0" fill="var(--logo-node-bg, #e2e8f0)"/>
      <circle cx="94" cy="30" r="6.0" fill="url(#ch35-pupil)" stroke="url(#ch35-gold-mira)" stroke-width="2.2"/>
      <line x1="85.0" y1="30" x2="90.0" y2="30" stroke="url(#ch35-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
      <line x1="98.0" y1="30" x2="103.0" y2="30" stroke="url(#ch35-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
      <line x1="94" y1="21.0" x2="94" y2="26.0" stroke="url(#ch35-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
      <line x1="94" y1="34.0" x2="94" y2="39.0" stroke="url(#ch35-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
      <circle cx="94" cy="30" r="2.0" fill="url(#ch35-gold-core)"/>
    </g>
  </g>
  <g transform="translate(148, 0)">
    <text x="2" y="27" fill="var(--logo-sub, #1d4ed8)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.2">PART III · FOUNDATIONS OF JUDGMENT</text>
    <text x="498" y="27" text-anchor="end" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.0">CH35 · MARTINGALE</text>
    <text x="0" y="59" fill="var(--logo-ink, #0f172a)" font-family="system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif" font-size="18.5" font-weight="800" letter-spacing="0.95">INFERENCE, SEVERITY, AND E-PROCESSES</text>
    <line x1="2" y1="78" x2="384" y2="78" stroke="url(#ch35-ring)" stroke-width="1.6" stroke-opacity="0.82"/>
    <circle cx="400" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.0"/>
    <circle cx="400" cy="78" r="1.6" fill="var(--logo-blue, #2563eb)"/>
    <circle cx="415" cy="78" r="6.2" stroke="var(--logo-gold, #f59e0b)" stroke-width="1.5" stroke-dasharray="2.2 1.6"/>
    <circle cx="415" cy="78" r="4.0" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.2"/>
    <circle cx="415" cy="78" r="1.8" fill="var(--logo-gold, #f59e0b)"/>
    <circle cx="430" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.0"/>
    <circle cx="430" cy="78" r="1.6" fill="var(--logo-blue, #2563eb)"/>
    <circle cx="445" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="460" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="475" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="490" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <text x="2" y="104" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.8" font-weight="600" letter-spacing="0.65">VILLE INEQUALITY · ANYTIME-VALID OPTIONAL STOPPING · SEVERE CORROBORATION</text>
  </g>
  </svg>
</figure>

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

An estimator is a rule that maps a sample to a numerical summary. Its properties belong to the procedure, across hypothetical repeated runs; they do not belong to any single historical realization.

- **Bias** ($\mathbb{E}[\hat{\theta}] - \theta^\ast$) is the systematic offset between the expected output of the rule
  and the true parameter.
- **Variance** ($\mathbb{E}[(\hat{\theta} - \mathbb{E}[\hat{\theta}])^2]$) is the sampling instability of the rule
  across parallel draws.
- **Admissibility** dictates that no competing rule achieves uniformly lower expected loss across all states of nature.

A point estimate quoted without its sampling distribution is half a result. The estimand itself is not always a single point either. When data are subject to non-random selection, non-response, or unmeasured confounding,
the estimand licensed by empirical reality is an **identification region**, a mathematical interval. Treating a set
as a failure of method is a category error: where the evidence identifies only a set, the set is the honest scientific
conclusion.

## 2. Confidence intervals and interval width as finding

A classical $95\unicode{x25}$ confidence interval is the set of parameter values that would not have been rejected by a
significance test at level $\alpha = 0.05$. Three misreadings account for most uses of the interval.

1. *The interval is not a Bayesian probability statement about the parameter.* The parameter $\theta^\ast$ is a
   fixed constant, not a random variable; it is either inside the interval or outside it, and the $95\unicode{x25}$ belongs
   to the apparatus across repeated applications.
2. *The width of the interval is the primary operational finding.* A wide interval does not represent
   "noisy precision"; it shows that the available evidence cannot differentiate a massive benefit from a
   catastrophic loss. Dropping the interval and publishing only the point estimate is the single most
   destructive distortion in applied science.
3. *The interval is strictly conditional upon the assumed likelihood model.* If the data-generating mechanism
   violates the model's structural assumptions (heavy tails, unmodeled clustering), the nominal coverage
   guarantee is void.

## 3. P-values, statistical power, and Mayo's severity principle

A p-value answers a narrow counterfactual question: *supposing that the null hypothesis were true, how probable
would it be to observe a test statistic at least as extreme as the one calculated from the data?* A p-value is
not the probability that the null hypothesis is true, nor does its complement measure the probability of the
alternative.

What an empirical test communicates depends fundamentally on whether the procedure was capable of detecting a flaw.
A non-significant test result produced by a study with low statistical power provides zero evidence that the effect
is absent; it merely confirms that the sample was too small to see it. **Power is the capacity to have failed.** A test that cannot fail is an instrument of institutional self-deception.

To evaluate evidence after the data are observed, Deborah Mayo formulated the **Severity Principle**. For a
substantive claim $H$ asserting that an effect exceeds a critical threshold ($\theta > \theta\_{\mathrm{crit}}$),
severity asks: *how probable was it that this experimental design would have produced a result less favorable
to $H$ than the observed data, if $H$ were false?*

- High severity means the claim survived a grueling probe designed to expose its specific vulnerabilities.
- Low severity means the test was so lax that almost any state of affairs would have passed it, rendering the
  pass uninformative.

Severity evaluates power at the specific boundary that matters to decision-makers, providing an objective error-statistical
foundation for belief.

## 4. Underdetermination and Manski partial-identification bounds

The deepest divide in inference separates a parameter that is hard to estimate from a parameter the data cannot identify at any sample size. Sampling variance belongs to the first. The second survives an infinite sample.

Consider a binary treatment $D \in \lbrace0, 1\rbrace$ with potential outcomes $Y(1)$ and $Y(0)$ bounded in $[0, 1]$.
Charles Manski (1989, 2003) proved that under arbitrary non-random selection into treatment, the Average Treatment
Effect $\mathrm{ATE} = \mathbb{E}[Y(1) - Y(0)]$ cannot be identified as a point. Because $Y(0)$ is unobserved for
treated units and $Y(1)$ is unobserved for untreated units, the **Manski worst-case bounds** are:

$$\mathrm{ATE} \in \Bigl[\mathbb{E}[Y \mid D=1]P(D=1) - P(D=1) - \mathbb{E}[Y \mid D=0]P(D=0), \mkern5mu \mathbb{E}[Y \mid D=1]P(D=1) + P(D=0) - \mathbb{E}[Y \mid D=0]P(D=0)\Bigr].$$

The width of this non-parametric identification interval is $1.0$ whatever the sample. An infinite sample size drives sampling error to zero and leaves the identification width where it was. Narrowing the interval further requires
substantive structural assumptions (e.g., monotone treatment response or instrumental variables), each of which must
be audited.

When treatment evaluation suffers from attrition (loss to follow-up), David Lee (2009) established sharp **attrition
bounds** under the assumption that treatment assignment affects attrition monotonically: the excess retention in
one arm is trimmed at the upper and lower quantiles of the outcome distribution, bounding the treatment effect
without parametric imputation.

When constructing confidence intervals for a partially identified parameter $\theta\_0 \in [\theta\_L, \theta\_U]$,
standard two-sided intervals are overly conservative. Guido Imbens and Charles Manski (2004) derived the modified
critical value $c\_\alpha$ that satisfies:

$$\Phi\left(c\_\alpha + \frac{\sqrt{n}(\hat{\theta}\_U - \hat{\theta}\_L)}{\hat{\sigma}}\right) - \Phi(-c\_\alpha) = 1 - \alpha.$$

As the identification interval width expands relative to sampling error ($\sqrt{n}(\Delta) \to \infty$), the critical
value $c\_\alpha$ smoothly transitions from the two-sided value $z\_{1-\alpha/2}$ (for point identification, $1.96$
at $95\unicode{x25}$) down to the one-sided critical value $z\_{1-\alpha}$ ($1.645$ at $95\unicode{x25}$), preserving exact coverage
probabilities for the true parameter $\theta\_0$.

Under competing risks where cause of failure is censored by an alternative hazard, Arthur V. Peterson (1976)
proved that the joint survival $S\_T(t) = \mathbb{P}(\min(T\_1, T\_2) > t)$ satisfies sharp Fréchet bounds:

$$\max\bigl(0,\mkern3mu S\_1(t) + S\_2(t) - 1\bigr) \le S\_T(t) \le \min\bigl(S\_1(t),\mkern3mu S\_2(t)\bigr).$$

## 5. Epigraphy and legal standards of proof

Classical epigraphers formalized partial identification on stone two millennia ago. Under the **Leiden Convention**
(van Groningen 1932), when a carved inscription is fractured, the physical dimensions of the break dictate the minimum
and maximum number of missing characters, printed strictly within brackets `[...]`. Fabricating a single point
restoration without brackets fabricates false certainty; printing the bracketed bounds, or setting the *crux
desperationis* ($\dagger\dots\dagger$) when no hypothesis passes severe scrutiny, is partial identification
preserved in marble.

In legal epistemology, L. Jonathan Cohen's **Gatecrasher Paradox** (Cohen 1977) shows why
bare statistical probability cannot sustain an actionable verdict. Suppose $499$ attendees paid for
admission and $501$ climbed the fence at a $1{,}000\text{-seat}$ rodeo. For any
randomly selected spectator, the probability that they gatecrashed is $0.501 > 0.50$, satisfying the
literal numerical threshold of "preponderance of the evidence". Yet no court can convict an
individual based solely on this aggregate ratio without individualized forensic testimony. The
Keynesian evidential weight $V(x, q)$ for that individual is zero:
statistical correlation without causal coverage cannot justify the coercive imposition of liability.

### Design error is bigger than precision

The observational comparison is not merely wider than the experimental one; it is wrong. A design failure that leaves a confounder unmeasured does not buy precision with an assumption, it moves the estimand, and precision bought at that price is width purchased around the wrong quantity. Precision narrows an interval around the right estimand; a design error moves the estimand, and no amount of narrowing recovers it.

The two bound families differ by an amount that is easy to state: worst-case attrition bounds are wider than the bounds a monotone selection assumption licenses, and the assumption buys exactly that difference in width. That gain is the entire return on a strong premise, and it is paid for by every decision the premise misstates.

The third quantity is the one that usually goes unreported: a test can be at its full declared power and still inform nothing, because power is about detecting an effect and not about whether the effect matters. What is missing in that case is the cost of the third kind of error, the answer that is correct and irrelevant.

### Deciding from an interval

The limit of identification is the right object when the estimand is a decision with asymmetric cost. The procedure is to convert the interval into a decision through the declared threshold, decide on the worst case, and state which side of the interval is worst. The rule costs something and the cost should be named: deciding on the worst case gives up the precision the interval advertised, in exchange for a decision that does not depend on a point nobody can defend.

## 6. The width that does not shrink, the confidence that does

### Two limits solved by hand

The Manski interval has a width that can be written down and then checked against arithmetic. Write $p = P(D=1)$, $a = \mathbb{E}[Y \mid D=1]$ and $b = \mathbb{E}[Y \mid D=0]$. The lower end is
$a p - p - b(1-p)$ and the upper end is
$a p + (1-p) - b(1-p)$, so the region is $[ap - p - b(1-p),\ ap + (1-p) - b(1-p)]$ and its width is
the difference of the two ends, $(1-p) + p = 1$. The identity is exact and independent of every number that
entered it: the two ends differ by $(1 - p) + p$ where $p = P(D=1)$, so the width is one for every archive, every
sample size and every outcome scale normalised to $[0, 1]$. What the formula buys is not a narrower answer but an
honest one, and the honest one is exactly one unit wide.

The second limit belongs to the coverage correction. The Imbens–Manski critical value solves
$\Phi(c\_\alpha + r) - \Phi(-c\_\alpha) = 1 - \alpha$ with $r = \sqrt{n}\mkern3mu\Delta / \hat\sigma$ the identification
width measured in standard errors. As $r \to 0$ the equation collapses to the two-sided form and
$c\_\alpha \to z\_{1-\alpha/2}$; as $r \to \infty$ the first term saturates at one and $c\_\alpha \to z\_{1-\alpha}$.
The transition is fast rather than gradual: the normal CDF on the left saturates across a range of $r$ of order a
couple of standard errors, so an identification width that spans only a few standard errors already buys most of the
way to the one-sided value, and the whole return on the correction is available only to an analyst who has already
admitted that the estimand is a set.

### The case where the severe recommendation was the wrong one

The chapter's advice is a maxim of severity: do not assert a causal claim that the evidence could not have refuted, and where the evidence yields a region, publish the region. Both halves were followed, correctly, in the dispute over smoking and lung cancer, and the recommendation they generated was wrong. Richard Doll and Austin Bradford Hill's 1950 case-control series reported an association between heavy smoking and lung carcinoma that no measured confounder in their tables could carry (Doll & Hill 1950); the error-statistical reading available at the time was that no randomized experiment could ever be run, that the region therefore remained open, and that causal language should be withheld — the discipline of §3 applied to a region, recommending that we wait.

The arithmetic that decided the matter was an interval argument of a different kind. Take the published ratio as
$\mathrm{RR}$; the sensitivity bound — the $E$-value of confounding, a quantity of confounder strength, and not the
$e$-value of a sequential test, which is a nonnegative random variable whose expectation under the null is bounded by
one — is the confounder strength needed to explain the association away,
$\mathrm{RR} + \sqrt{\mathrm{RR}\mkern3mu(\mathrm{RR} - 1)}$ in risk-ratio units, on both the exposure side and the outcome side
(VanderWeele & Ding 2017). For the magnitude the 1950 series reported, that bound demands a confounder associated with
both smoking and lung cancer at a strength no measured
covariate in the mid-century epidemiologic record approached, so the region that remained open was not a region the world could inhabit, and the 1964 report of
the Advisory Committee to the Surgeon General drew the causal conclusion and the policy consequence from it
(Advisory Committee to the Surgeon General 1964). The severity principle did not fail because it was
sloppy; it failed because it priced refutation and did not price the confounder that refutation would require.

### When the region is honest and the decision is empty

The chapter's thesis is that where the evidence identifies a set, the set is the conclusion. The condition that
voids it is the case in which the set brackets the decision threshold: when the region contains every action the institution can take, the report that is maximally honest about the parameter is
maximally empty about the decision, and an institution that publishes only the region has handed the judgement to
whoever reads the number next. The procedure is to publish the region together with the map from region to
admissible actions: state which actions survive every point of the region and which survive none, and decide on
the regret of the action rather than on the parameter. The regime is implementable: the Imbens–Manski critical value supplies the corrected coverage,
Manski's bounds carry the region into the margin,
the severity audit tests whether the boundary that matters to
the decision was probed at all, and Peterson's competing-risks bounds are
the smallest working example of the shape: they return a set rather than a number, because independence would have
to be assumed before a number existed. When the region decides nothing, the honest sentence is not *the effect lies in
this interval* but *every action survives this interval, and we are choosing on grounds the interval cannot see*.

## Bridge

Severe testing, partial-identification bounds, and confidence adjustments are formalised in
[Chapter 44](44-underdetermination-partial-identification.md) through Manski's ATE bounds, which evaluate the
identification region alongside
their adjusted interval; David Lee's sharp trimming bounds for
attrition; the Imbens–Manski
critical value $c\_\alpha$ for parameter coverage; and the Mayo severity audit against the critical policy threshold.
Evidential weight under partial coverage is governed by Keynes's weight. Evidence becomes an act only through a loss and a threshold, which is [Chapter 36](36-decision-loss-threshold.md), *Decision, Loss, Threshold*.
