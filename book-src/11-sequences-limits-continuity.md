# Sequences, Limits, and Continuity

<figure class="stoch-figure stoch-logo-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="660" height="132" viewBox="0 0 660 132" fill="none" role="img" aria-label="Stochology CH11 · CONTINUITY — Sequences, Limits, And Stability">
  <defs>
    <linearGradient id="ch11-ring" x1="2" y1="78" x2="386" y2="78" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="38%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="68%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <radialGradient id="ch11-glow" cx="50%" cy="50%" r="50%">
      <stop offset="0%" stop-color="#34d399" stop-opacity="0.25"/>
      <stop offset="50%" stop-color="#10b981" stop-opacity="0.14"/>
      <stop offset="82%" stop-color="#fbbf24" stop-opacity="0.08"/>
      <stop offset="100%" stop-color="#090e1a" stop-opacity="0"/>
    </radialGradient>
    <linearGradient id="ch11-triad-stroke" x1="18" y1="24" x2="110" y2="104" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.85"/>
      <stop offset="50%" stop-color="var(--logo-gold, #f59e0b)" stop-opacity="0.90"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.85"/>
    </linearGradient>
    <linearGradient id="ch11-fill" x1="22" y1="30" x2="106" y2="98" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)" stop-opacity="0.26"/>
      <stop offset="48%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.28"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.26"/>
    </linearGradient>
    <linearGradient id="ch11-ink-curve" x1="20" y1="66" x2="108" y2="66" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-ink, #0f172a)"/>
      <stop offset="50%" stop-color="var(--logo-sub, #1d4ed8)"/>
      <stop offset="100%" stop-color="var(--logo-ink, #0f172a)"/>
    </linearGradient>
    <radialGradient id="ch11-pupil" cx="45%" cy="42%" r="58%">
      <stop offset="0%" stop-color="#020617"/>
      <stop offset="68%" stop-color="#0f172a"/>
      <stop offset="100%" stop-color="#451a03"/>
    </radialGradient>
    <linearGradient id="ch11-gold-mira" x1="54" y1="48" x2="84" y2="82" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#fef08a"/>
      <stop offset="45%" stop-color="#fbbf24"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <radialGradient id="ch11-gold-core" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#ffffff"/>
      <stop offset="35%" stop-color="#fde047"/>
      <stop offset="100%" stop-color="#d97706"/>
    </radialGradient>
    <radialGradient id="ch11-node-well" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="var(--logo-well-hi, var(--elevated-bg, #ffffff))"/>
      <stop offset="100%" stop-color="var(--logo-node-bg, #e2e8f0)"/>
    </radialGradient>
    <linearGradient id="ch11-ring-epi" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#7dd3fc"/>
      <stop offset="50%" stop-color="#2563eb"/>
      <stop offset="100%" stop-color="#1e3a8a"/>
    </linearGradient>
    <radialGradient id="ch11-core-epi" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#bae6fd"/>
      <stop offset="45%" stop-color="#3b82f6"/>
      <stop offset="100%" stop-color="#1d4ed8"/>
    </radialGradient>
    <linearGradient id="ch11-ring-vital" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#6ee7b7"/>
      <stop offset="50%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#065f46"/>
    </linearGradient>
    <radialGradient id="ch11-core-vital" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#a7f3d0"/>
      <stop offset="45%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#047857"/>
    </radialGradient>
    <linearGradient id="ch11-ring-gov" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#fda4af"/>
      <stop offset="50%" stop-color="#e11d48"/>
      <stop offset="100%" stop-color="#881337"/>
    </linearGradient>
    <radialGradient id="ch11-core-gov" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#fecdd3"/>
      <stop offset="45%" stop-color="#f43f5e"/>
      <stop offset="100%" stop-color="#be123c"/>
    </radialGradient>
  </defs>
  <g>
    <circle cx="64" cy="66" r="48" fill="url(#ch11-glow)"/>
    <path d="M 20 34 L 106 56" stroke="var(--logo-blue, #2563eb)" stroke-width="1.6" stroke-dasharray="3 2.5"/>
    <path d="M 20 98 L 106 76" stroke="var(--logo-blue, #2563eb)" stroke-width="1.6" stroke-dasharray="3 2.5"/>
    <path d="M 22 38 C 34 94, 46 42, 58 82 C 70 52, 82 74, 98 66" stroke="url(#ch11-ink-curve)" stroke-width="2.6" stroke-linecap="round"/>
    <circle cx="28.00" cy="66.00" r="6.05" fill="url(#ch11-node-well)" stroke="url(#ch11-ring-epi)" stroke-width="2.23"/>
    <circle cx="28.00" cy="66.00" r="2.59" fill="url(#ch11-core-epi)"/>
    <circle cx="52.00" cy="62.00" r="6.05" fill="url(#ch11-node-well)" stroke="url(#ch11-ring-vital)" stroke-width="2.23"/>
    <circle cx="52.00" cy="62.00" r="2.59" fill="url(#ch11-core-vital)"/>
    <g>
      <circle cx="98.00" cy="66.00" r="6.81" fill="var(--logo-node-bg, #e2e8f0)"/>
      <circle cx="98.00" cy="66.00" r="5.89" fill="url(#ch11-pupil)" stroke="url(#ch11-gold-mira)" stroke-width="2.21"/>
      <line x1="89.17" y1="66.00" x2="94.14" y2="66.00" stroke="url(#ch11-gold-mira)" stroke-width="2.21" stroke-linecap="round"/>
      <line x1="101.86" y1="66.00" x2="106.83" y2="66.00" stroke="url(#ch11-gold-mira)" stroke-width="2.21" stroke-linecap="round"/>
      <line x1="98.00" y1="57.17" x2="98.00" y2="62.14" stroke="url(#ch11-gold-mira)" stroke-width="2.21" stroke-linecap="round"/>
      <line x1="98.00" y1="69.86" x2="98.00" y2="74.83" stroke="url(#ch11-gold-mira)" stroke-width="2.21" stroke-linecap="round"/>
      <circle cx="98.00" cy="66.00" r="2.02" fill="url(#ch11-gold-core)"/>
    </g>
  </g>
  <g transform="translate(148, 0)">
    <text x="2" y="27" fill="var(--logo-sub, #1d4ed8)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.2">PART II · THE BASES · MATHEMATICS</text>
    <text x="498" y="27" text-anchor="end" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.0">CH11 · CONTINUITY</text>
    <text x="0" y="59" fill="var(--logo-ink, #0f172a)" font-family="system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif" font-size="20.2" font-weight="800" letter-spacing="1.15">SEQUENCES, LIMITS, AND STABILITY</text>
    <line x1="2" y1="78" x2="384" y2="78" stroke="url(#ch11-ring)" stroke-width="1.6" stroke-opacity="0.82"/>
    <circle cx="400" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.0"/>
    <circle cx="400" cy="78" r="1.6" fill="var(--logo-blue, #2563eb)"/>
    <circle cx="415" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="430" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="445" cy="78" r="6.2" stroke="var(--logo-gold, #f59e0b)" stroke-width="1.5" stroke-dasharray="2.2 1.6"/>
    <circle cx="445" cy="78" r="4.0" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-green, #10b981)" stroke-width="2.2"/>
    <circle cx="445" cy="78" r="1.8" fill="var(--logo-gold, #f59e0b)"/>
    <circle cx="460" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-green, #10b981)" stroke-width="2.0"/>
    <circle cx="460" cy="78" r="1.6" fill="var(--logo-green, #10b981)"/>
    <circle cx="475" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="490" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <text x="2" y="104" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.8" font-weight="600" letter-spacing="0.65">CAUCHY CONVERGENCE · COMPACTNESS · DISCONTINUOUS FOLD BIFURCATIONS</text>
  </g>
  </svg>
</figure>

> Third chapter of the mathematical block in Part II. The subject is what it means for something to
> approach something else, and where that fails.

## By the end of this chapter you can

1. Say what it means for a sequence to converge, without a circular appeal to "getting closer".
2. Distinguish a limit from a value, and explain why a function can be continuous everywhere except
   at the one point a decision depends on.
3. Say what continuity is not, using a case where small causes have large effects.

## 1. Sequences and Cauchy Convergence

A water authority watches a rolling average of reservoir inflows settle inside a narrow band over thirty
consecutive months and concludes that the series has converged to a permanent equilibrium. The multi-year
drought that shifts the baseline downward arrives the month after the conclusion. Recent terms close to one
another say that recent months were quiet. They do not prove an infinite limit exists.

A sequence is an ordered mapping from the natural numbers into a metric space, written $(a\_n)\_{n=1}^\infty$.
It **converges** to a limit $L \in \mathbb{R}$ if for every adversarial tolerance $\epsilon > 0$, there exists
an index $N(\epsilon) \in \mathbb{N}$ such that for all $n \ge N(\epsilon)$:

$$|a\_n - L| < \epsilon.$$

The order of the quantifiers is the whole definition. An adversary picks the error threshold
$\epsilon$ first; the threshold index $N(\epsilon)$ is produced afterwards, and it may depend on the
tolerance it was handed.

A sequence is a **Cauchy sequence** when its terms become arbitrarily close to one another: for every
$\epsilon > 0$ there exists $N$ such that $|a\_n - a\_m| < \epsilon$ for all $n, m \ge N$. In
$\mathbb{R}$, Cauchy completeness ensures that every Cauchy sequence converges to a limit in
$\mathbb{R}$.

Asymptotic convergence ($n \to \infty$) guarantees nothing about finite prefixes. A sequence may converge
at rate $\mathcal{O}(1/\ln n)$, which demands astronomical samples for modest precision. A running
second-moment accumulator over a stationary sequence with finite variance ($\alpha > 2$) is consistent, and
its standard error still shrinks only at $\mathcal{O}(n^{-1/2})$. With infinite variance
($\alpha \le 2$), the running variance is not Cauchy at all: it jumps upward with every new tail record.
Under regime shifts, an apparently settled sequence diverges the moment the conditions change.

## 2. Limits, Continuity, and Uniform Stability

An emergency department sees triage delay rise roughly linearly with bed occupancy across the middle of
the admissible range. Administrators rely on that smooth local behaviour and schedule the ward close to
saturation, expecting another small nudge in occupancy to add a few minutes and not a multi-hour congestion
blow-up. Pointwise continuity promises that an infinitesimal perturbation at a fixed interior point has a
small effect. It promises nothing about the same small shock near a capacity boundary.

For a real-valued function $f : D \to \mathbb{R}$, the statement $\lim\_{x \to x\_0} f(x) = L$ asserts that
$f(x)$ can be forced arbitrarily close to $L$ by choosing $x$ sufficiently close to $x\_0$. The function is
**continuous at $x\_0$** if $x\_0 \in D$ and $\lim\_{x \to x\_0} f(x) = f(x\_0)$.

Continuity formalizes one intuition, that *small perturbations produce small responses*, and point
continuity does not give **uniform continuity**: on an open domain $f(x) = 1/x$ is continuous at every
$x > 0$, yet $\sup |f'(x)| = \infty$ as $x \to 0$. Institutional decision systems need the uniform kind,
because an admissible rule must not amplify tiny observation errors into large operational disruptions.

Discontinuities are where judgment is forced. There are **jump discontinuities** from threshold switches,
such as activating an emergency buffer when risk clears $p^\ast$; **asymptotic blow-ups** from divergence at
a finite boundary, such as Kingman waiting times as $\rho \to 1$; and **regime oscillations**, when limits
fail to exist under undamped delayed feedback.

## 3. Continuity Is Not Robustness

A function can be infinitely differentiable everywhere and still unstable. Take $f(x) = x^k$ for large
$k$: a fractional input error $\delta x / x$ induces an output relative error

$$\frac{\delta f}{f} \approx k \frac{\delta x}{x}.$$

Smoothness and stability part company again in the rational amplification factor
$f(x) = \frac{1}{1 - x}$ on the open half-interval $x \in [0, 1)$, with derivative
$f'(x) = \frac{1}{(1 - x)^2} > 0$, which is Kingman's congestion multiplier for queue waiting times
(Kingman 1961). The function $f$ is $C^\infty$-smooth at every point in $[0, 1)$, and its two boundary limits
behave oppositely. As $x \to 0^+$, $f(x) \to 1$ and $f'(x) \to 1$, so local sensitivity is the unit and
the arrangement is uniformly stable. As $x \to 1^-$, $f(x) \to +\infty$ and $f'(x) \to +\infty$, so an
arbitrarily small perturbation near $x = 1$ produces an arbitrarily large jump. For non-linear models, local
continuity gives no guarantee at all against catastrophic sensitivity. Deterministic chaos is the same
lesson in a dynamical system: the equations of the Lorenz attractor are continuous and smooth at every point
in phase space, and nearby trajectories diverge exponentially at rate $e^{\lambda t}$, with $\lambda > 0$
the maximal Lyapunov exponent (Lorenz 1963). Determinism does not imply predictability. Continuity does not
imply robustness.

## 4. Non-Ergodicity and the Itô Variance Drain

A pension fund's board sets the annual payout rate equal to the arithmetic average daily return of the
equity index, confident that a century of data guarantees convergence. Within a decade of drawdowns the fund
is insolvent, with the index's arithmetic average matching the projection all along. The board had confused
the average across parallel counterfactual worlds with the compounded growth rate along the single timeline
the fund had to survive.

The largest failure of convergence is the gap between **ensemble averages** and **time averages**. A system
is ergodic when the time average along a single trajectory equals the ensemble average:

$$\lim\_{T \to \infty} \frac{1}{T} \int\_0^T X\_t\mkern3mudt = \mathbb{E}[X].$$

Ergodicity fails systematically in multiplicative and dynamic processes. Take a wealth or resource
trajectory modelled by Geometric Brownian Motion:

$$dX\_t = \mu X\_t\mkern3mudt + \sigma X\_t\mkern3mudW\_t, \qquad X\_0 > 0.$$

Itô's lemma gives the solution for an individual trajectory:

$$X\_t = X\_0 \exp\left( \left(\mu - \frac{\sigma^2}{2}\right) t + \sigma W\_t \right).$$

Average over all possible universes and the **ensemble average** is:

$$\mathbb{E}[X\_t] = X\_0 e^{\mu t}.$$

For $\mu > 0$ the ensemble average grows exponentially without bound. The law of the iterated logarithm for
Brownian motion says almost every single trajectory does something else, at the time-average growth rate

$$g = \lim\_{t \to \infty} \frac{1}{t} \ln X\_t = \mu - \frac{\sigma^2}{2} \quad \text{almost surely}.$$

The negative term $-\frac{1}{2}\sigma^2$ is the **Itô–Kelly variance drain** (derived via Itô's 1951
stochastic differential lemma and applied to continuous-time compounding by Kelly 1956, Robert C. Merton
1969, and Peters 2019 (Itô 1951; Kelly 1956; Merton 1969; Peters 2019)). Set $\mu > 0$ with
$\mu < \frac{1}{2}\sigma^2$ and the ensemble average explodes to $+\infty$, while with probability 1 every
individual institution following the process goes bankrupt as $t \to \infty$. An infinitesimal fraction of
astronomically lucky trajectories drives the ensemble average. Typical systems suffer irreversible
collapse.

In canonical form $\mu\_{\mathrm{time}} = g = \mu - \frac{1}{2}\sigma^2$: the ensemble drift
$\mu \in \mathbb{R}$ ($\text{day}^{-1}$, unrestricted sign) and the daily return variance
$\sigma^2 \ge 0$ ($\text{day}^{-2}$) map onto the experienced trajectory growth rate
$\mu\_{\mathrm{time}} \in (-\infty, \mu]$ ($\text{day}^{-1}$). Three boundary regimes govern the
trajectory:

1. **Negligible volatility ($\sigma^2 / 2 \ll \mu$, $\sigma \to 0^+$):** $\mu\_{\mathrm{time}} \to \mu$; the
   variance drain vanishes and the ensemble drift approximates single-path compounding well.
2. **Zero-growth horizon ($\sigma = \sqrt{2\mu}$):** $\mu\_{\mathrm{time}} = 0$; whatever the drift, a volatility
   exists that is large enough to annihilate net compounding growth.
3. **Absorbing drain ($\sigma > \sqrt{2\mu}$ with $\mu > 0$):** $\mu\_{\mathrm{time}} < 0$ while
   $\mathbb{E}[X\_t] \to +\infty$. The drain enters as $\sigma^2 / 2$, so doubling the daily volatility
   quadruples the drained term and cuts time-average growth by a factor no additional history can repair.
   Baker and McHale's (2013) parameter-uncertainty penalty shrinks the plug-in Kelly leverage fraction
   $f\_{\mathrm{plugin}} = \mu / \sigma^2$ toward the epistemically defensible
   $f\_{\mathrm{epi}} < f\_{\mathrm{plugin}}$, by a shrinkage factor $k\_{\mathrm{BM}} < 1$ that grows with
   the estimation error of $\mu$ (Baker & McHale 2013).

## 5. Consequences for Sequential Decision-Making

Institutions live along a single trajectory and cannot average across counterfactual worlds, so risk
evaluated with ensemble expectations assumes that bankruptcy in one run is compensated by extreme wealth in
another. Under non-ergodic multiplicative dynamics, volatility is a direct drag on survival. A decision rule
that ignores the Itô variance drain takes a path to certain ruin for a profitable investment: the ensemble
expectation is positive while almost every single trajectory is absorbed at the ruin barrier.

Conversely, an institution that genuinely pools and continuously rebalances $N$ mutually independent
trajectories of equal drift $\mu$ and variance drain $\sigma^2 / 2$ sees the pooled variance scale as
$\sigma^2 / N$, and the experienced growth rate rises to
$\mu\_{\mathrm{pool}}(N) = \mu - \frac{\sigma^2}{2N}$. The drain shrinks by the factor $N$: at $N = 10$
independent units it is a tenth of the unpooled drain, at $N = 100$ a hundredth. Sizing an authentically
diversified $N$-unit pool on the unpooled $N = 1$ time-average drift understates capacity systematically,
just as sizing a single unpooled reserve on the ensemble drift overstates accumulation by the factor
$1/(1 - \sigma^2/(2\mu))$.

### The drift nobody experiences

The Dow grows at one rate averaged across parallel worlds and at a smaller rate if you live in one of
them. The gap is the variance drain. A fixed share of the expected growth is never experienced by any
investor, and a reserve sized on the ensemble drift overestimates its own accumulation by that share. No
amount of additional history closes the gap, because the gap is not sampling error. It is the difference
between averaging over worlds and living in one.

The arithmetic is a single term. The time-average drift is $\mu - \sigma^2/2$, so the experienced drift is
the ensemble drift less half the variance. Set $\sigma = \sqrt{2\mu}$ and the time-average drift is zero;
above that the typical trajectory loses while the ensemble gains.

### When the ensemble average is the right one

The thesis fails where the institution can genuinely diversify: $N$ independent units with continuous
rebalancing do experience an average, and the relevant quantity becomes the ensemble drift. Replicate the
units, then, instead of lengthening the history. A hundred independent copies buy the ensemble; a hundred
more years of one copy buy a better estimate of a number no one receives.

The term that produces the drain is not a modelling choice. It is what the stochastic calculus contributes
to any continuous-time path (Itô 1951), and the rule that turns it into a sizing decision is Kelly's (Kelly
1956). Both predate any particular price series, so the overestimate is a property of the arithmetic, not a
finding about the Dow.

## Exercises

1. Derive the ratio of time-average drift to ensemble drift, $1 - \sigma^2/(2\mu)$, and the share of expected growth that no single trajectory ever experiences.
2. Given a yearly drift $\mu$ and volatility $\sigma$, write the time-average drift and say when it turns negative (answer: $\mu - \sigma^2/2$; negative once $\sigma^2 > 2\mu$).
3. Find the volatility that annihilates the time drift at a given $\mu$ and say what follows (answer: $\sigma = \sqrt{2\mu}$; above it the typical path loses).
4. State when the ensemble average is the correct quantity and how to reach it (answer: many independent units with rebalancing; replicate the unit instead of extending the series).

## Bridge

[Distribution and Moment](31-distribution-and-moment.md) examines the mechanics of sample convergence, while
[Time and Dependence](38-time-and-dependence.md) analyzes how variance drains multiplicative trajectories
over time. The time-average growth rate $g = \mu - \frac{1}{2}\sigma^2$ is compared against ensemble
returns, which diagnoses when an institution is being misled by ensemble optimism toward absorbing ruin
barriers.

[Derivatives and Convexity](12-derivatives-and-convexity.md) differentiates the limit into local sensitivity
and curvature, the two pieces that make the Jensen gap between an average and a risk visible.
