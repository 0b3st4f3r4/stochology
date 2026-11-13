# Integrals and Accumulation

<figure class="stoch-figure stoch-logo-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="660" height="132" viewBox="0 0 660 132" fill="none" role="img" aria-label="Stochology CH13 · MEASURE — Integrals, Stocks, And Accumulation">
  <defs>
    <linearGradient id="ch13-ring" x1="2" y1="78" x2="386" y2="78" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="38%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="68%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <radialGradient id="ch13-glow" cx="50%" cy="50%" r="50%">
      <stop offset="0%" stop-color="#34d399" stop-opacity="0.25"/>
      <stop offset="50%" stop-color="#10b981" stop-opacity="0.14"/>
      <stop offset="82%" stop-color="#fbbf24" stop-opacity="0.08"/>
      <stop offset="100%" stop-color="#090e1a" stop-opacity="0"/>
    </radialGradient>
    <linearGradient id="ch13-triad-stroke" x1="18" y1="24" x2="110" y2="104" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.85"/>
      <stop offset="50%" stop-color="var(--logo-gold, #f59e0b)" stop-opacity="0.90"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.85"/>
    </linearGradient>
    <linearGradient id="ch13-fill" x1="22" y1="30" x2="106" y2="98" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)" stop-opacity="0.26"/>
      <stop offset="48%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.28"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.26"/>
    </linearGradient>
    <linearGradient id="ch13-ink-curve" x1="20" y1="66" x2="108" y2="66" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-ink, #0f172a)"/>
      <stop offset="50%" stop-color="var(--logo-sub, #1d4ed8)"/>
      <stop offset="100%" stop-color="var(--logo-ink, #0f172a)"/>
    </linearGradient>
    <radialGradient id="ch13-pupil" cx="45%" cy="42%" r="58%">
      <stop offset="0%" stop-color="#020617"/>
      <stop offset="68%" stop-color="#0f172a"/>
      <stop offset="100%" stop-color="#451a03"/>
    </radialGradient>
    <linearGradient id="ch13-gold-mira" x1="54" y1="48" x2="84" y2="82" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#fef08a"/>
      <stop offset="45%" stop-color="#fbbf24"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <radialGradient id="ch13-gold-core" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#ffffff"/>
      <stop offset="35%" stop-color="#fde047"/>
      <stop offset="100%" stop-color="#d97706"/>
    </radialGradient>
    <radialGradient id="ch13-node-well" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="var(--logo-well-hi, var(--elevated-bg, #ffffff))"/>
      <stop offset="100%" stop-color="var(--logo-node-bg, #e2e8f0)"/>
    </radialGradient>
    <linearGradient id="ch13-ring-epi" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#7dd3fc"/>
      <stop offset="50%" stop-color="#2563eb"/>
      <stop offset="100%" stop-color="#1e3a8a"/>
    </linearGradient>
    <radialGradient id="ch13-core-epi" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#bae6fd"/>
      <stop offset="45%" stop-color="#3b82f6"/>
      <stop offset="100%" stop-color="#1d4ed8"/>
    </radialGradient>
    <linearGradient id="ch13-ring-vital" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#6ee7b7"/>
      <stop offset="50%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#065f46"/>
    </linearGradient>
    <radialGradient id="ch13-core-vital" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#a7f3d0"/>
      <stop offset="45%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#047857"/>
    </radialGradient>
    <linearGradient id="ch13-ring-gov" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#fda4af"/>
      <stop offset="50%" stop-color="#e11d48"/>
      <stop offset="100%" stop-color="#881337"/>
    </linearGradient>
    <radialGradient id="ch13-core-gov" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#fecdd3"/>
      <stop offset="45%" stop-color="#f43f5e"/>
      <stop offset="100%" stop-color="#be123c"/>
    </radialGradient>
  </defs>
  <g>
    <circle cx="64" cy="66" r="48" fill="url(#ch13-glow)"/>
    <rect x="26" y="68" width="14" height="28" fill="var(--logo-blue, #2563eb)" fill-opacity="0.28" stroke="var(--logo-blue, #2563eb)" stroke-width="1.6"/>
    <rect x="42" y="48" width="14" height="48" fill="var(--logo-green, #10b981)" fill-opacity="0.28" stroke="var(--logo-green, #10b981)" stroke-width="1.6"/>
    <rect x="58" y="36" width="14" height="60" fill="var(--logo-gold, #f59e0b)" fill-opacity="0.28" stroke="var(--logo-gold, #f59e0b)" stroke-width="1.6"/>
    <rect x="74" y="54" width="14" height="42" fill="var(--logo-crimson, #e11d48)" fill-opacity="0.28" stroke="var(--logo-crimson, #e11d48)" stroke-width="1.6"/>
    <path d="M 22 86 C 36 52, 54 28, 66 34 C 78 40, 92 70, 106 88" stroke="url(#ch13-ink-curve)" stroke-width="2.8" stroke-linecap="round"/>
    <g>
      <circle cx="65.00" cy="36.00" r="6.66" fill="var(--logo-node-bg, #e2e8f0)"/>
      <circle cx="65.00" cy="36.00" r="5.76" fill="url(#ch13-pupil)" stroke="url(#ch13-gold-mira)" stroke-width="2.16"/>
      <line x1="56.36" y1="36.00" x2="61.22" y2="36.00" stroke="url(#ch13-gold-mira)" stroke-width="2.16" stroke-linecap="round"/>
      <line x1="68.78" y1="36.00" x2="73.64" y2="36.00" stroke="url(#ch13-gold-mira)" stroke-width="2.16" stroke-linecap="round"/>
      <line x1="65.00" y1="27.36" x2="65.00" y2="32.22" stroke="url(#ch13-gold-mira)" stroke-width="2.16" stroke-linecap="round"/>
      <line x1="65.00" y1="39.78" x2="65.00" y2="44.64" stroke="url(#ch13-gold-mira)" stroke-width="2.16" stroke-linecap="round"/>
      <circle cx="65.00" cy="36.00" r="1.98" fill="url(#ch13-gold-core)"/>
    </g>
  </g>
  <g transform="translate(148, 0)">
    <text x="2" y="27" fill="var(--logo-sub, #1d4ed8)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.2">PART II · THE BASES · MATHEMATICS</text>
    <text x="498" y="27" text-anchor="end" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.0">CH13 · MEASURE</text>
    <text x="0" y="59" fill="var(--logo-ink, #0f172a)" font-family="system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif" font-size="18.5" font-weight="800" letter-spacing="0.95">INTEGRALS, STOCKS, AND ACCUMULATION</text>
    <line x1="2" y1="78" x2="384" y2="78" stroke="url(#ch13-ring)" stroke-width="1.6" stroke-opacity="0.82"/>
    <circle cx="400" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="415" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="430" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.0"/>
    <circle cx="430" cy="78" r="1.6" fill="var(--logo-blue, #2563eb)"/>
    <circle cx="445" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-green, #10b981)" stroke-width="2.0"/>
    <circle cx="445" cy="78" r="1.6" fill="var(--logo-green, #10b981)"/>
    <circle cx="460" cy="78" r="6.2" stroke="var(--logo-gold, #f59e0b)" stroke-width="1.5" stroke-dasharray="2.2 1.6"/>
    <circle cx="460" cy="78" r="4.0" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-green, #10b981)" stroke-width="2.2"/>
    <circle cx="460" cy="78" r="1.8" fill="var(--logo-gold, #f59e0b)"/>
    <circle cx="475" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="490" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <text x="2" y="104" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.8" font-weight="600" letter-spacing="0.65">LEBESGUE EXPECTATION · CUMULATIVE LOSS TAILS · IRREVERSIBLE DEPLETION</text>
  </g>
  </svg>
</figure>

> Fifth chapter of the mathematical block in Part II. The subject is accumulation, and the reason an
> expectation is an integral rather than a sum.

## By the end of this chapter you can

1. Read a definite integral as an accumulated quantity, not as an area formula.
2. Say why an expectation is a weighted integral, and what the weights have to satisfy.
3. Use the tail-integral identity to compute an average from the probability of exceeding each level.

## 1. Accumulation and Integration

A reservoir operator watches a crisis dashboard through a prolonged drought. The gauge reports the
instantaneous net outflow rate. That number cannot say whether the town runs out of water before relief
arrives. What empties the tank is the running sum of every gallon drawn across every minute of the emergency.
Accumulation is the bridge between an instantaneous flow rate and a stock that can be exhausted.

The derivative measures local rates of change. The integral measures total accumulated substance. Let $f(t)$
be an instantaneous rate: gallons per minute, financial claims per day, metabolic toxin load per hour. The
definite integral accumulates that rate across an interval $[a, b]$:

$$I = \int\_a^b f(t)\mkern3mudt = \lim\_{\Delta t\_i \to 0} \sum\_{i=1}^n f(t\_i^\ast) \Delta t\_i.$$

Two structural properties carry everything that follows: **linearity**, where $\int\_a^b \bigl(\alpha f(t) + \beta g(t)\bigr)\mkern3mudt = \alpha \int\_a^b f(t)\mkern3mudt + \beta \int\_a^b g(t)\mkern3mudt$, and **interval additivity**, where $\int\_a^c f(t)\mkern3mudt = \int\_a^b f(t)\mkern3mudt + \int\_b^c f(t)\mkern3mudt$.

In vital reserve dynamics, interval additivity governs survival. Hold a vital reserve stock $R\_0$ against a
stochastic drawdown rate $D(t)$. Time to exhaustion, the time-to-survival $\mathrm{TTS}$ is then the
solution of an integral equation:

$$R\_0 - \int\_0^{\mathrm{TTS}} D(t)\mkern3mudt = 0.$$

Survival is an accumulated process, not an instantaneous snapshot.

To make the equation operational, type each term. The initial buffer is a positive stock $R\_0 \in (0, \infty)$.
The net crisis drawdown is a non-negative trajectory $D: [0, \infty) \to [0, \infty)$ in stock units per unit
time. The autonomous horizon is the first-passage functional $\mathrm{TTS}\_{\mathrm{survival}} = \inf\bigl\lbrace t \ge 0: \int\_0^t D(s)\mkern3muds \ge R\_0\bigr\rbrace \in [0, \infty]$.
The institution escapes absorption at the ruin barrier ($R\_t \le 0$) exactly when that horizon clears the
operational recovery lead time, $\mathrm{TTS}\_{\mathrm{survival}} \ge \mathrm{TTR}\_{\mathrm{recovery}}$ (Simchi-Levi et al. 2014).
Two boundary cases show interval additivity deciding the buffer:

- **Constant crisis drain ($D(t) \equiv d\_{\mathrm{crisis}} > 0$):** Linearity pulls the constant outside the integral, $\int\_0^t d\_{\mathrm{crisis}}\mkern3muds = d\_{\mathrm{crisis}} t$, and the equation collapses to the ratio $\mathrm{TTS}\_{\mathrm{survival}} = R\_0 / d\_{\mathrm{crisis}}$. Stock over flow, and that ratio is what decides a stressed system against a buffered one: the operator whose reserve covers less than its recovery lead time collapses before recovery, and the operator whose reserve covers more clears the same disruption with room to spare.
- **Piecewise pulsed shock ($D(t) = d\_1$ on $[0, t\_1]$ and $D(t) = d\_2$ on $(t\_1, t]$):** Interval additivity splits the depletion path at $t\_1$, giving $\int\_0^t D(s)\mkern3muds = d\_1 t\_1 + d\_2(t - t\_1)$ and remaining survival time $\mathrm{TTS}\_{\mathrm{survival}} = t\_1 + (R\_0 - d\_1 t\_1)/d\_2$, provided the first pulse does not exhaust the stock outright ($d\_1 t\_1 < R\_0$).

## 2. Expectation as a Lebesgue Integral

A catastrophe actuary averaging annual losses can slice the sum two ways. Riemann slices the domain
vertically, marching day by day along the horizontal axis. Lebesgue slices the codomain horizontally: group the
world states $\omega \in \Omega$ that produce roughly the same loss $X(\omega) \approx x$, multiply that level by
$\mathbb{P}(\{\omega : X(\omega) \in dx\})$, accumulate across levels. The shift from chronological
summation to measure-weighted accumulation by level is why expectation is a Lebesgue integral.

In rigorous probability, expectation is the Lebesgue integral of a measurable random variable
$X : \Omega \to \mathbb{R}$ against the probability measure $\mathbb{P}$:

$$\mathbb{E}[X] = \int\_\Omega X(\omega)\mkern3mud\mathbb{P}(\omega).$$

When $X$ admits a continuous probability density function $p(x)$, this specializes to $\mathbb{E}[X] = \int\_{-\infty}^\infty x p(x)\mkern3mudx$.
The weighting measure has to satisfy the Kolmogorov axioms: non-negativity ($p(x) \ge 0$) and total unit mass
($\int p(x)\mkern3mudx = 1$). A weighting that violates unit mass is not a probability expectation. One that
permits negative weights is not coherent.

For non-negative random variables ($X \ge 0$), Fubini's theorem gives the **tail-integral identity**:

$$\mathbb{E}[X] = \int\_0^\infty \mathbb{P}(X > t)\mkern3mudt.$$

The identity is geometric: the expected value is the integral of the survival function $S(t) = \mathbb{P}(X > t)$.
Thin tails, exponential and Gaussian, drop $S(t)$ exponentially to zero, and the tail's contribution to the
integral is negligible. Set $S(t) \sim t^{-\alpha}$ with $\alpha \le 1$ and the area under $S(t)$ diverges:
$\mathbb{E}[X] = \infty$. The expectation is then driven entirely by the unbounded upper tail.

The derivation shows why horizontal slicing is natural. For any realization $X(\omega) \ge 0$, write the number
itself as an integral of a step indicator, $X(\omega) = \int\_0^\infty \mathbf{1}\{X(\omega) > t\}\mkern3mudt$, and
apply Tonelli–Fubini to swap the expectation and the threshold integral,
$\mathbb{E}[X] = \int\_\Omega \bigl(\int\_0^\infty \mathbf{1}\{X(\omega) > t\}\mkern3mudt\bigr)\mkern3mud\mathbb{P}(\omega) = \int\_0^\infty \mathbb{P}(X > t)\mkern3mudt$.
For a Pareto survival tail $S(t) = (t/x\_m)^{-\alpha}$ on $t \ge x\_m > 0$ the integral evaluates to
$\mathbb{E}[X] = x\_m + \int\_{x\_m}^\infty (t/x\_m)^{-\alpha}\mkern3mudt = \frac{\alpha}{\alpha - 1}x\_m < \infty$ when $\alpha > 1$,
and diverges to $+\infty$ as $\alpha \to 1^+$.

The same threshold-accumulation geometry governs forecast verification. Instead of scoring a predictive
distribution $F$ by the distance of its mean from a realized outcome $y \in \mathbb{R}$, the Continuous Ranked
Probability Score integrates the squared Brier discrepancy across every threshold $t \in \mathbb{R}$ (Hersbach 2000; Gneiting & Raftery 2007),
the scoring rule displayed in
[The Non-Identical, Scale Invariance, Convexity, and Heavy Tails](46-non-identical-asymmetry-heavy-tails.md).
In the degenerate limit where $F = \delta\_{\hat{y}}$ is a point forecast the spread term vanishes and the score
reduces to the Mean Absolute Error. Against a sorted sample of size $m$ the full integral costs
$\mathcal{O}(m \log m)$ operations, so the score is computable for any archive large enough to matter. On the
Copenhagen Reinsurance fire-loss data (McNeil 1997) it can be taken out of sample by splitting the claims in
half and scoring each held-out claim against the empirical distribution of the rest. That reports how well the
tail is represented, not how well its mean is estimated.

## 3. Limit-Integral Exchange and Fat-Tailed Failures

Exchanging a limit and an integral, $\lim\_{n\to\infty} \int f\_n = \int \lim\_{n\to\infty} f\_n$, is governed by
Henri Lebesgue's **Dominated Convergence Theorem**. The exchange is valid when an integrable dominating function
$g$ exists with $|f\_n(x)| \le g(x)$ for all $n$.

Heavy tails break it. Let probability mass escape toward infinity in a distribution with tail index
$\alpha \le 1$, and the sample running averages $\bar{X}\_n = \frac{1}{n}\sum\_{i=1}^n X\_i$ converge to nothing
finite. The limit of the sample averages is not the average of the underlying process. Relying on sample sums to
estimate tail risk, without verifying uniform integrability, is a fatal epistemic blunder.

The failure shows up in miniature. Take the escaping-mass spikes $f\_n(x) = n\mkern3mu\mathbf{1}\_{(0, 1/n)}(x)$ on
the unit interval $(0, 1)$. For every fixed $x \in (0, 1)$, once $n > 1/x$ we have $f\_n(x) = 0$, so the pointwise
limit vanishes ($\lim\_{n\to\infty} f\_n(x) = 0$) and the integral of the limit is $\int\_0^1 0\mkern3mudx = 0$.
For every finite $n$, though, the accumulated mass is $\int\_0^1 f\_n(x)\mkern3mudx = n \cdot (1/n) = 1$, so
$\lim\_{n\to\infty} \int\_0^1 f\_n(x)\mkern3mudx = 1 \ne 0$. Dominated convergence fails because the pointwise
supremum $\sup\_{n \ge 1} f\_n(x) = 1/x$ has a diverging integral $\int\_0^1 x^{-1}\mkern3mudx = \infty$.

Heavy-tailed loss distributions with Pareto survival decay $S(x) \propto x^{-\alpha}$ (Embrechts et al. 1997)
put the moment boundary at a point. For $\alpha \le 1$, $\int\_1^\infty x^{-\alpha}\mkern3mudx = \infty$ and not
even the population mean exists. For $1 < \alpha \le 2$ the first moment $\mathbb{E}[X]$ converges while the
second diverges, $\mathbb{E}[X^2] = \int\_0^\infty 2t\mkern3mu\mathbb{P}(X > t)\mkern3mudt = \infty$. Both exist
only for $\alpha > 2$. An estimated index is therefore an interval. Entirely below the boundary, it certifies the
non-existence of the moment; entirely above, existence; straddling, it certifies nothing until more of the tail
has been observed.

## 4. Coherent Risk Measures: Expected Shortfall vs. Value-at-Risk

A reinsurance underwriting committee prices an excess-of-loss layer for the worst few per cent of fire-claim
years. Report only the high quantile threshold and the committee has drawn a curtain at the doorway of the worst
outcomes: a moderate loss and a catastrophic loss cast the same shadow on a quantile. A balance sheet cannot pay
a curtain. It must pay the accumulated severity of every claim that crosses the threshold.

How does an institution integrate tail risk over thresholds? Philippe Artzner et al. (1999) set out the axioms
of **coherent risk measures**: monotonicity, subadditivity, positive homogeneity, translation invariance.
Subadditivity, $\rho(X + Y) \le \rho(X) + \rho(Y)$, says that diversifying across two exposures cannot raise
total risk.

The industry metric is Value-at-Risk at level $\alpha \in (0, 1)$, $\mathrm{VaR}\_\alpha(X) = \inf\{x : \mathbb{P}(X \le x) \ge \alpha\}$,
and it reads a single quantile. $\mathrm{VaR}$ is not subadditive. Combine two independent fat-tailed risks and
the portfolio can carry a higher $\mathrm{VaR}$ than the sum of the individual $\mathrm{VaR}$ values, so
diversification is penalized.

**Expected Shortfall** (Conditional Value-at-Risk, $\mathrm{CVaR}\_\alpha$) integrates every loss above
$\mathrm{VaR}\_\alpha$:

$$\mathrm{CVaR}\_\alpha(X) = \frac{1}{1 - \alpha} \int\_\alpha^1 \mathrm{VaR}\_u(X)\mkern3mudu = \frac{1}{1 - \alpha}\Bigl(\mathbb{E}\bigl[X\mkern2mu\mathbf{1}\{X > \mathrm{VaR}\_\alpha(X)\}\bigr] + \mathrm{VaR}\_\alpha(X)\bigl(1 - \alpha - \mathbb{P}(X > \mathrm{VaR}\_\alpha(X))\bigr)\Bigr),$$

which simplifies to $\mathbb{E}[X \mid X \ge \mathrm{VaR}\_\alpha(X)]$ whenever the loss distribution has no point mass at $\mathrm{VaR}\_\alpha(X) = \inf\{x : F\_X(x) \ge \alpha\}$. Being an integral over the whole upper tail, Expected Shortfall is coherent and subadditive across both continuous and discrete distributions. It rewards diversification and penalizes heavy-tailed extreme losses that standard quantiles ignore.

For a continuous loss law, the empirical threshold $q\_{0.95}$ is paired with the tail average over realizations above $q\_{0.95}$ (splitting any discrete boundary atom so total tail weight equals $1 - \alpha$). Two analytical boundary cases and one operational counterexample fix the relation between the quantile and the tail integral:

- **Comonotonic limit ($Y = X$):** Two exposures moving in lockstep make both quantiles and tail integrals additive: $\mathrm{VaR}\_\alpha(2X) = 2\mkern3mu\mathrm{VaR}\_\alpha(X)$ and $\mathrm{CVaR}\_\alpha(2X) = 2\mkern3mu\mathrm{CVaR}\_\alpha(X)$.
- **Disjoint tail concentration (subadditivity failure of $\mathrm{VaR}$):** For $\alpha \in [1/2, 1)$, two credit positions $A$ and $B$ each lose $L > 0$ with probability $1 - \alpha$ on mutually exclusive events and $0$ otherwise. Because $\mathbb{P}(A \le 0) = \alpha$, each standalone position has $\mathrm{VaR}\_\alpha(A) = \mathrm{VaR}\_\alpha(B) = 0$ while $\mathrm{VaR}\_u(A) = L$ for all $u \in (\alpha, 1]$, giving $\mathrm{CVaR}\_\alpha(A) = \mathrm{CVaR}\_\alpha(B) = L$ (whereas naive $\mathbb{E}[A \mid A \ge 0] = (1 - \alpha)L$ fails because of the atom at $0$). The combined portfolio $A + B$ loses $L$ with probability $2(1 - \alpha) > 1 - \alpha$, so $\mathrm{VaR}\_\alpha(A + B) = L > 0 + 0$ penalizes diversification, whereas $\mathrm{CVaR}\_\alpha(A + B) = L \le L + L$ obeys subadditivity.
- **When the quantile is the right instrument:** Take a clearinghouse contract that settles strictly by *threshold trigger*, returning a fixed collateral deposit whenever $X \le q\_{0.95}$ and seizing that deposit on breach regardless of excess magnitude. Posting the expected shortfall instead of the value-at-risk immobilizes a large multiple of the capital and buys not one unit of additional trigger protection.

## 5. Partial Identification and Worst-Case Bounds

Missing or selective data leave the integration domain incomplete. Under Charles Manski's partial identification
framework, when an outcome $Y \in [y\_{\min}, y\_{\max}]$ is unobserved for a fraction $1 - p$ of the population,
integrating the unobserved mass at the extrema bounds the true expectation:

$$\mathbb{E}[Y] \in \bigl[p \mathbb{E}[Y \mid R=1] + (1 - p)y\_{\min},\mkern5mu p \mathbb{E}[Y \mid R=1] + (1 - p)y\_{\max}\bigr].$$

Worst-case bounds give reserve sizing that withstands arbitrary, adversarial selection in the missing data.

Additivity of the Lebesgue integral over the partition $\Omega = \{R = 1\} \cup \{R = 0\}$ gives the interval
directly (Manski 1990; Manski 2003):

$$\mathbb{E}[Y] = \int\_{\{R=1\}} Y\mkern3mud\mathbb{P} + \int\_{\{R=0\}} Y\mkern3mud\mathbb{P} = p\mkern3mu\mathbb{E}[Y \mid R = 1] + (1 - p)\mkern3mu\mathbb{E}[Y \mid R = 0].$$

Because $y\_{\min} \le Y(\omega) \le y\_{\max}$ pointwise on $\{R = 0\}$, integrating the constant bounds over a set
of measure $1 - p$ traps the unobserved integral inside $[(1 - p)y\_{\min},\mkern3mu (1 - p)y\_{\max}]$. The
identification width $\Delta\_{\mathrm{id}} = (1 - p)(y\_{\max} - y\_{\min})$ is irreducible: first for a single arm,
then, by interval subtraction, for two-arm causal comparisons. In the National Supported Work trial (LaLonde 1986),
complete follow-up leaves $1 - p = 0$ and shrinks $\Delta\_{\mathrm{id}}$ to $0$ around the experimental point
estimate. Under bounded attrition on the observed support, integrating the missing mass at the support extrema
widens the two-arm Manski interval by $\bigl((1 - p\_1) + (1 - p\_0)\bigr)(y\_{\max} - y\_{\min})$.

### Two numbers for the same tail

Ask the file for the worst few per cent and it answers with two quantities: the threshold, and the average loss
above it. The second exceeds the first whenever the tail has any spread at all. A margin posted at the threshold
is enough to be breached and insufficient to be paid. The integral is what the balance sheet owes, and it is the
only one of the two that is subadditive. Where the tail index exceeds one the mean exists, so the shortfall is
finite and estimable, and the reporting problem becomes the width of the estimate and not its existence. A
ceiling moves the answer by a substantial fraction: truncate every claim at a high percentile and the shortfall
falls toward that ceiling. A cap, then, is a decision about what the institution refuses to see, not a
measurement.

### When the tail is not integrable

This thesis fails one regime earlier, where the index does not exceed one. The mean does not exist, so no
integral converges and a point estimate of the shortfall has nothing to converge to. Report bounds. Reduce
exposure. The tail index is estimated with an interval precisely so that this case is recognised (Artzner et al. 1999).

## Exercises

1. Derive the expected shortfall from the value-at-risk by integrating over levels above it, and say why their
   ratio exceeds one whenever the tail has spread (answer: the shortfall integrates the excess above the
   quantile, so the ratio measures how much of the tail the quantile refuses to see).
2. Compute the ratio of the tighter expected shortfall to the looser one and say what it measures (answer: the
   speed at which the tail grows as the level tightens).
3. Construct a portfolio for which the quantile is the right measure and say why (answer: a margin requirement
   set per threshold, where cost is independent of magnitude).
4. Say how to report risk when the index does not exceed one, and why the point estimate misleads (answer:
   bounds plus an exposure reduction; the mean does not exist, so a point has no meaning).

## Bridge

[Distribution and Moment](31-distribution-and-moment.md) and [The Non-Identical, Scale Invariance, Convexity, and Heavy Tails](46-non-identical-asymmetry-heavy-tails.md)
deploy these integral identities to score rules across continuous thresholds. The same identities integrate
worst-case bounds over missing counterfactuals, and compute the Continuous Ranked Probability Score by
integrating squared threshold forecast errors across the full support of the empirical distribution.

[Linear Algebra](14-linear-algebra.md) carries the same accumulation into finite dimensions, operators, rank,
projections, and the effective dimension of an archive with more columns than evidence.
