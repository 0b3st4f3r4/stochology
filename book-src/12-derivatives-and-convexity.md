# Derivatives and Convexity

<figure class="stoch-figure stoch-logo-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="660" height="132" viewBox="0 0 660 132" fill="none" role="img" aria-label="Stochology CH12 · CURVATURE — Derivatives, Subgradients, And Convexity">
  <defs>
    <linearGradient id="ch12-ring" x1="2" y1="78" x2="386" y2="78" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="38%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="68%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <radialGradient id="ch12-glow" cx="50%" cy="50%" r="50%">
      <stop offset="0%" stop-color="#34d399" stop-opacity="0.25"/>
      <stop offset="50%" stop-color="#10b981" stop-opacity="0.14"/>
      <stop offset="82%" stop-color="#fbbf24" stop-opacity="0.08"/>
      <stop offset="100%" stop-color="#090e1a" stop-opacity="0"/>
    </radialGradient>
    <linearGradient id="ch12-triad-stroke" x1="18" y1="24" x2="110" y2="104" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.85"/>
      <stop offset="50%" stop-color="var(--logo-gold, #f59e0b)" stop-opacity="0.90"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.85"/>
    </linearGradient>
    <linearGradient id="ch12-fill" x1="22" y1="30" x2="106" y2="98" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)" stop-opacity="0.26"/>
      <stop offset="48%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.28"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.26"/>
    </linearGradient>
    <linearGradient id="ch12-ink-curve" x1="20" y1="66" x2="108" y2="66" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-ink, #0f172a)"/>
      <stop offset="50%" stop-color="var(--logo-sub, #1d4ed8)"/>
      <stop offset="100%" stop-color="var(--logo-ink, #0f172a)"/>
    </linearGradient>
    <radialGradient id="ch12-pupil" cx="45%" cy="42%" r="58%">
      <stop offset="0%" stop-color="#020617"/>
      <stop offset="68%" stop-color="#0f172a"/>
      <stop offset="100%" stop-color="#451a03"/>
    </radialGradient>
    <linearGradient id="ch12-gold-mira" x1="54" y1="48" x2="84" y2="82" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#fef08a"/>
      <stop offset="45%" stop-color="#fbbf24"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <radialGradient id="ch12-gold-core" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#ffffff"/>
      <stop offset="35%" stop-color="#fde047"/>
      <stop offset="100%" stop-color="#d97706"/>
    </radialGradient>
    <radialGradient id="ch12-node-well" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="var(--logo-well-hi, var(--elevated-bg, #ffffff))"/>
      <stop offset="100%" stop-color="var(--logo-node-bg, #e2e8f0)"/>
    </radialGradient>
    <linearGradient id="ch12-ring-epi" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#7dd3fc"/>
      <stop offset="50%" stop-color="#2563eb"/>
      <stop offset="100%" stop-color="#1e3a8a"/>
    </linearGradient>
    <radialGradient id="ch12-core-epi" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#bae6fd"/>
      <stop offset="45%" stop-color="#3b82f6"/>
      <stop offset="100%" stop-color="#1d4ed8"/>
    </radialGradient>
    <linearGradient id="ch12-ring-vital" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#6ee7b7"/>
      <stop offset="50%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#065f46"/>
    </linearGradient>
    <radialGradient id="ch12-core-vital" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#a7f3d0"/>
      <stop offset="45%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#047857"/>
    </radialGradient>
    <linearGradient id="ch12-ring-gov" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#fda4af"/>
      <stop offset="50%" stop-color="#e11d48"/>
      <stop offset="100%" stop-color="#881337"/>
    </linearGradient>
    <radialGradient id="ch12-core-gov" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#fecdd3"/>
      <stop offset="45%" stop-color="#f43f5e"/>
      <stop offset="100%" stop-color="#be123c"/>
    </radialGradient>
  </defs>
  <g>
    <circle cx="64" cy="66" r="48" fill="url(#ch12-glow)"/>
    <circle cx="64" cy="66" r="42" stroke="var(--logo-ticks, #64748b)" stroke-width="1.2" stroke-dasharray="3 3" stroke-opacity="0.45"/>
    <path d="M 24 40 Q 64 112 104 40" fill="url(#ch12-fill)" stroke="url(#ch12-ink-curve)" stroke-width="2.8" stroke-linecap="round"/>
    <line x1="22" y1="88" x2="106" y2="64" stroke="var(--logo-green, #10b981)" stroke-width="2.4" stroke-linecap="round"/>
    <line x1="28" y1="46" x2="100" y2="46" stroke="var(--logo-crimson, #e11d48)" stroke-width="1.8" stroke-dasharray="3.5 2.5"/>
    <circle cx="28.00" cy="46.00" r="6.30" fill="url(#ch12-node-well)" stroke="url(#ch12-ring-epi)" stroke-width="2.33"/>
    <circle cx="28.00" cy="46.00" r="2.70" fill="url(#ch12-core-epi)"/>
    <circle cx="100.00" cy="46.00" r="6.30" fill="url(#ch12-node-well)" stroke="url(#ch12-ring-gov)" stroke-width="2.33"/>
    <circle cx="100.00" cy="46.00" r="2.70" fill="url(#ch12-core-gov)"/>
    <g>
      <circle cx="64.00" cy="76.00" r="6.81" fill="var(--logo-node-bg, #e2e8f0)"/>
      <circle cx="64.00" cy="76.00" r="5.89" fill="url(#ch12-pupil)" stroke="url(#ch12-gold-mira)" stroke-width="2.21"/>
      <line x1="55.17" y1="76.00" x2="60.14" y2="76.00" stroke="url(#ch12-gold-mira)" stroke-width="2.21" stroke-linecap="round"/>
      <line x1="67.86" y1="76.00" x2="72.83" y2="76.00" stroke="url(#ch12-gold-mira)" stroke-width="2.21" stroke-linecap="round"/>
      <line x1="64.00" y1="67.17" x2="64.00" y2="72.14" stroke="url(#ch12-gold-mira)" stroke-width="2.21" stroke-linecap="round"/>
      <line x1="64.00" y1="79.86" x2="64.00" y2="84.83" stroke="url(#ch12-gold-mira)" stroke-width="2.21" stroke-linecap="round"/>
      <circle cx="64.00" cy="76.00" r="2.02" fill="url(#ch12-gold-core)"/>
    </g>
  </g>
  <g transform="translate(148, 0)">
    <text x="2" y="27" fill="var(--logo-sub, #1d4ed8)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.2">PART II · THE BASES · MATHEMATICS</text>
    <text x="498" y="27" text-anchor="end" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.0">CH12 · CURVATURE</text>
    <text x="0" y="59" fill="var(--logo-ink, #0f172a)" font-family="system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif" font-size="18.5" font-weight="800" letter-spacing="0.95">DERIVATIVES, SUBGRADIENTS, AND CONVEXITY</text>
    <line x1="2" y1="78" x2="384" y2="78" stroke="url(#ch12-ring)" stroke-width="1.6" stroke-opacity="0.82"/>
    <circle cx="400" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="415" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.0"/>
    <circle cx="415" cy="78" r="1.6" fill="var(--logo-blue, #2563eb)"/>
    <circle cx="430" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="445" cy="78" r="6.2" stroke="var(--logo-gold, #f59e0b)" stroke-width="1.5" stroke-dasharray="2.2 1.6"/>
    <circle cx="445" cy="78" r="4.0" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-green, #10b981)" stroke-width="2.2"/>
    <circle cx="445" cy="78" r="1.8" fill="var(--logo-gold, #f59e0b)"/>
    <circle cx="460" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-green, #10b981)" stroke-width="2.0"/>
    <circle cx="460" cy="78" r="1.6" fill="var(--logo-green, #10b981)"/>
    <circle cx="475" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="490" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <text x="2" y="104" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.8" font-weight="600" letter-spacing="0.65">SECOND-ORDER FRAGILITY · SUPPORTING HYPERPLANES · JENSEN INEQUALITY</text>
  </g>
  </svg>
</figure>

> Fourth chapter of the mathematical block in Part II. The subject is rate of change, and the single
> geometric fact that decides whether planning with an average underfunds the damage.

## By the end of this chapter you can

1. Read a derivative as a rate and a second derivative as a curvature, and say which a decision
   needs.
2. Identify a function as convex or concave from its shape, and say what that implies for an average.
3. Compute the convexity gap for a simple case and say why it is never negative.

## 1. The Derivative as Local Sensitivity

A regional hospital network or a cloud-infrastructure team cuts reserve capacity because a linear
regression around calm mid-week traffic shows a mild marginal slope $f'(x)$ of a fraction of a second of
delay per additional request. That tangent slope measures how the system responds to an infinitesimal nudge
at the current operating point. It says nothing about the vertical wall that arrives when a winter epidemic
or a flash surge pushes load toward capacity. Taking a local first derivative for a global description of
risk is the mathematical root of unbuffered congestion collapse.

The derivative of a real function $f$ at point $x$ is defined as the limit of the difference quotient:

$$f'(x) = \lim\_{h \to 0} \frac{f(x + h) - f(x)}{h}.$$

Geometrically, $f'(x)$ is the slope of the tangent line. Operationally it measures local marginal
sensitivity: the rate of change of the output per unit change in input.

A derivative equal to zero ($f'(x^\ast) = 0$) marks a stationary point, which may be a local minimum, a
local maximum, or an inflection point. What $f'$ alone does not say is whether the stationary point is
stable, or how the function answers to finite, non-infinitesimal shocks. Local stability around an upper
ecological or physiological equilibrium $x\_{\mathrm{high}}^\ast$ needs a strictly negative restoring
derivative $f'(x\_{\mathrm{high}}^\ast) = -\lambda\_{\mathrm{eng}} < 0$, which sets the engineering
recovery time $\mathrm{TTR}\_{\mathrm{recovery}} = \kappa / \lambda\_{\mathrm{eng}} = -\kappa \Delta t / \ln \rho\_1$
(Holling 1973; Scheffer et al. 2009). As harvesting pressure approaches the fold bifurcation,
$f'(x\_{\mathrm{high}}^\ast) \to 0^-$, lag-1 autocorrelation $\rho\_1 \to 1$, and recovery time
diverges. Planning on marginal slopes $f'$ alone ignores non-linear tail vulnerability.

## 2. The Second Derivative and Curvature

A municipal trauma ward takes daily patient arrivals averaging $A$: on quiet Mondays arrivals fall below the
average by some amount, on stormy Tuesdays they rise above it by the same amount. The arithmetic average
across the two days is still $A$, and the two deviations do not cancel. The quiet day saves a few minutes of
paperwork; the surge day triggers ambulance diversions and corridor gridlock. Whenever equal-and-opposite
input shocks fail to cancel in the output, the response function carries non-zero curvature
($f''(x) \ne 0$).

Differentiating again yields the second derivative:

$$f''(x) = \lim\_{h \to 0} \frac{f'(x + h) - f'(x)}{h}.$$

The second derivative measures the rate of change of the slope, which is the **curvature** of the function:

- If $f''(x) > 0$ across an interval, the slope is strictly increasing, the graph bends upwards, and $f$ is **strictly convex**.
- If $f''(x) < 0$, the slope is strictly decreasing, the graph bends downwards, and $f$ is **strictly concave**.
- If $f''(x) = 0$, the function is locally affine (a straight line or inflection).

Curvature is the property that decides whether the expectation of a function equals the function of an
expectation. In physical, biological and organizational systems, damages are nearly always convex in stress:
an increase in load with capacity nearly full does not produce a proportional increase in delay. It produces
an explosive one.

## 3. Jensen's Inequality and the Taylor Expansion of the Jensen Gap

A function $\phi : \mathbb{R} \to \mathbb{R}$ is convex on an interval if for all $x, y$ in the interval and all
$\lambda \in [0, 1]$:

$$\phi\bigl(\lambda x + (1 - \lambda) y\bigr) \le \lambda \phi(x) + (1 - \lambda) \phi(y).$$

By Johan Jensen's inequality (1906) (Jensen 1906), for any integrable random variable $X$:

$$\mathbb{E}\bigl[\phi(X)\bigr] \ge \phi\bigl(\mathbb{E}[X]\bigr).$$

The difference is the **Jensen gap** (or convexity gap):

$$\Delta\_J(\phi, X) = \mathbb{E}\bigl[\phi(X)\bigr] - \phi\bigl(\mathbb{E}[X]\bigr) \ge 0.$$

To see the magnitude, expand $\phi(X)$ in a second-order Taylor series around the mean
$\mu = \mathbb{E}[X]$:

$$\phi(X) \approx \phi(\mu) + \phi'(\mu)(X - \mu) + \frac{1}{2} \phi''(\mu)(X - \mu)^2.$$

Take expectations on both sides:

$$\mathbb{E}\bigl[\phi(X)\bigr] \approx \phi(\mu) + \phi'(\mu)\mathbb{E}[X - \mu] + \frac{1}{2}\phi''(\mu)\mathbb{E}\bigl[(X - \mu)^2\bigr].$$

Because $\mathbb{E}[X - \mu] = 0$ and $\mathbb{E}[(X - \mu)^2] = \sigma^2 = \mathrm{Var}(X)$, the
second-order approximation takes its canonical form:

$$\Delta\_J(\phi, X) \approx \frac{1}{2} \phi''(\mu) \sigma^2.$$

The anatomy of the gap is in that identity: it is the product of physical or structural curvature
$\phi''(\mu)$ and environmental variance $\sigma^2$. The gap is zero if and only if either the function is
affine ($\phi'' \equiv 0$) or the world is deterministic ($\sigma^2 = 0$).

Across empirical loss distributions the three quantities $\phi(\mathbb{E}[X])$, $\mathbb{E}[\phi(X)]$ and
$\Delta\_J(\phi, X) = \mathbb{E}[\phi(X)] - \phi(\mathbb{E}[X])$ can be compared directly. Three canonical
shapes show how curvature governs the sign and the magnitude of the gap:

1. **Convex quadratic loss ($\phi(x) = x^2$, $\phi''(x) \equiv 2 > 0$):** $\mathbb{E}[X^2]$ exceeds
   $(\mathbb{E}[X])^2$ by the variance, so the Jensen gap equals $\mathrm{Var}(X)$ and is positive whenever
   dispersion is non-zero. Under a power-law stress function $\phi(x) = c x^{\beta}$ with $\beta > 1$ the gap
   is positive for the same reason, and larger the more of the loss distribution's mass sits in the tail.
2. **Concave utility ($\phi(x) = \sqrt{x}$ on $x > 0$, $\phi''(x) = -\frac{1}{4}x^{-3/2} < 0$):** concavity
   reverses the inequality, so $\mathbb{E}[\sqrt{X}] < \sqrt{\mathbb{E}[X]}$ and the gap is negative — a
   benefit valued at its mean is overstated.
3. **Affine tariff ($\phi(x) = a + bx$, $\phi''(x) \equiv 0$):** for a linear per-claim fee with the cost linear
   in the claim, $\mathbb{E}[\phi(X)] - \phi(\mathbb{E}[X]) = 0$ identically, whatever the heavy-tailed
   variance or excess kurtosis. Planning at the mean underfunds reserves if and only if $\phi'' > 0$.

## 4. The Explosive Curvature of Queues and Ruin

An emergency-room director or grid operator brags of raising average capacity utilization toward saturation
in the name of "eliminating idle slack", and is then bewildered when average waiting times jump far out of proportion to the
utilization gained. The queue did not break. It followed the rational curve $\rho / (1 - \rho)$ into its
high-curvature wall.

An institution that evaluates risk at the average, substituting $\phi(\mathbb{E}[X])$ for
$\mathbb{E}[\phi(X)]$, understates true expected harm by exactly $\Delta\_J$. In queueing and logistics,
John Kingman's heavy-traffic formula makes waiting time proportional to:

$$\phi(\rho) = \frac{\rho}{1 - \rho}, \qquad \rho \in [0, 1),$$

where $\rho$ is utilization. Computing the derivatives:

$$\phi'(\rho) = \frac{1}{(1 - \rho)^2}, \qquad \phi''(\rho) = \frac{2}{(1 - \rho)^3}.$$

As utilization approaches 100% ($\rho \to 1$), the curvature $\phi''(\rho)$ diverges cubically, as
$\mathcal{O}\bigl((1 - \rho)^{-3}\bigr)$. The divergence is steep enough that each tenfold cut in the idle share $1 - \rho$
multiplies the waiting factor by about ten and its curvature by about a thousand, since near saturation
$\phi \approx (1-\rho)^{-1}$ and $\phi'' = 2(1-\rho)^{-3}$. A tiny
fluctuation in arrival rate near saturation creates a monumental Jensen gap, and it wipes out unbuffered
operations.

The congestion multiplier separates two operating regimes, and it is one map on one interval: the passage
between them is a point of that map, not a second map. Inverting the display gives $\rho = \phi/(1+\phi)$,
so the image of $[0,1)$ is $[0,\infty)$, and every utilization has its multiplier and back. The multiplier equals
one at $\rho = 1/2$; there the heavy-traffic expected queue delay equals the dispersion-scaled service unit $\frac{c\_a^2 + c\_s^2}{2}\mkern2mu\tau\_s$ (one mean service time in the unit-dispersion exponential benchmark $M/M/1$ where $c\_a^2 = c\_s^2 = 1$), and there the buffered reading meets the stressed one. Below that point the utilization multiplier stays below one, so heavy-traffic delay is a fraction of $\frac{c\_a^2 + c\_s^2}{2}\mkern2mu\tau\_s$. Above it the multiplier exceeds one and rises without bound, so a buffered regime that held the multiplier below
one at every utilization short of saturation would be a different map wearing the same name.

The same point fixes how a perturbation in load is amplified. From $\phi'/\phi = 1/(\rho(1-\rho))$,
$$\frac{d\ln\phi}{d\ln\rho} = \frac{\rho\mkern3mu\phi'(\rho)}{\phi(\rho)} = \frac{1}{1-\rho} = 1 + \phi(\rho),$$
so a one per cent increase in utilization raises the waiting multiplier by $1+\phi$ per cent: under two per
cent below the crossing, two per cent at it, without bound above. The level crossing at $\phi = 1$ and the
elasticity crossing at $2$ are one point. That is what it means for the two regimes to be halves of a single
map.

The slack writes the same function another way. With $s = 1-\rho$ the map is $\phi = 1/s - 1$, one less
than the reciprocal of the slack and not the reciprocal itself; the difference is what leaves the factor at
zero at zero load. Halving the slack multiplies it by
$$\frac{\phi(s/2)}{\phi(s)} = \frac{2-s}{1-s},$$
which exceeds two at every positive slack and reaches two only at the wall. "Halve the slack and the wait
doubles" is the limit of the passage, not an identity across the interval.

The ruin chapter reads the same function from its other end. With net replenishment $c$, shock intensity
$\lambda$ and mean severity $\mu$, the load is the offered loss rate over the replenishment rate,
$\rho = \lambda\mu/c$, the same ratio of offered work to capacity that the queue writes as $\lambda/\mu\_s$.
The safety loading is
$$\theta\_L = \frac{c-\lambda\mu}{\lambda\mu} = \frac{1}{\rho} - 1 = \frac{1}{\phi(\rho)},$$
so the utilization multiplier and the reciprocal of the loading are one quantity. The wall at
$\rho \to 1$ is the point at which the loading vanishes, and the buffer that holds ruin below a declared
$\varepsilon$ grows in proportion to $\phi$ and not to some second measure of congestion
([Irreversible Time](47-irreversible-time-vital-margin.md) derives the coefficient). A theory that sized the
queue on one curve and the reserve on another would be reading two maps where the corpus has one. That
explosion is what drives an unbuffered optimizer into absorbing ruin: the operator sized capacity at the mean
of a convex function, and the convexity collected the difference.

## 5. What Curvature Decides

Three rules govern judgment under non-linear curvature. Planning at the mean guarantees underfunding by
$\Delta\_J \approx \frac{1}{2}\phi''(\mu)\sigma^2$. Concave benefits create false optimism, since
$\mathbb{E}[\phi(X)] \le \phi(\mathbb{E}[X])$ when $\phi'' < 0$. And linearity is a fragile fiction:
assuming $\phi'' = 0$ does not make the world linear, it blinds the modeller to the convexity tax.

### Curvature decides the sign

Curvature decides whether the average is optimistic or pessimistic. Dispersion decides by how much. The
split is why the sign of a Jensen gap can be read off $f''$ before any data is consulted, and why the
magnitude cannot.

### When the second order is not enough

The second-order approximation $\mathbb{E}[\varphi(X)] - \varphi(\mu) \approx \frac{1}{2}\varphi''(\mu)\sigma^2$ fails when higher-order moments dominate the quadratic term. For skewed distributions ($\gamma\_1 = \mathbb{E}[(X-\mu)^3]/\sigma^3 \ne 0$) with $\varphi''(\mu) \ne 0$, the cubic term $\frac{1}{6}\varphi'''(\mu)\gamma\_1\sigma^3$ overtakes the quadratic correction once
$$|\gamma\_1|\mkern2mu\sigma > \frac{3\mkern2mu|\varphi''(\mu)|}{|\varphi'''(\mu)|}$$
(while for symmetric heavy-tailed laws with $\gamma\_1 = 0$, the cubic term vanishes and fourth-order kurtosis $\frac{1}{24}|\varphi^{(4)}(\mu)|\mathbb{E}[(X-\mu)^4]$ sets the breakdown scale). Test across two dispersions, and if the gap does not scale linearly with $\sigma^2$, evaluate $\mathbb{E}[\varphi(X)]$ directly.

The risk literature separates the two shapes: a convex measure of loss is coherent and can be aggregated
(Artzner et al. 1999), and the queue curvature that makes waiting explode near saturation is the heavy-traffic
approximation (Kingman 1961).

## Bridge

[Distribution and Moment](31-distribution-and-moment.md) examines moments and tail distributions, while
[The Non-Identical, Scale Invariance, Convexity, and Heavy Tails](46-non-identical-asymmetry-heavy-tails.md)
computes empirical Jensen gaps across heavy-tailed fire and financial loss data. The vital safeguard carries
the second-order Jensen debiasing into the evaluation of Canguilhem–Holling attraction basins and reserve
depletion, so that sizing runs on the expected loss and not on the loss at the expected state.

[Integrals and Accumulation](13-integrals-and-accumulation.md) reverses the derivative: accumulation,
expectation as a Lebesgue integral, and coherent risk measures such as expected shortfall.
