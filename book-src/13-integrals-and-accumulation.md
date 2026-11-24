# Integrals and Accumulation

<figure class="stoch-figure stoch-logo-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="800" height="132" viewBox="0 0 800 132" fill="none" role="img" aria-label="Stochology CH13 · MEASURE — Integrals, Stocks, And Accumulation">
  <defs>
    <!-- Canonical Ring Gradient -->
    <linearGradient id="ch13-ring" x1="2" y1="78" x2="386" y2="78" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="38%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="68%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <!-- 1. Atmospheric Cosmic Field -->
    <radialGradient id="ch13-glow" cx="52%" cy="52%" r="50%">
      <stop offset="0%" stop-color="#fbbf24" stop-opacity="0.24"/>
      <stop offset="48%" stop-color="#38bdf8" stop-opacity="0.14"/>
      <stop offset="80%" stop-color="#e11d48" stop-opacity="0.08"/>
      <stop offset="100%" stop-color="#090e1a" stop-opacity="0"/>
    </radialGradient>
    <!-- 3D Spherical Node Gradients -->
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
    <!-- Stókhos Reticle Gradients -->
    <radialGradient id="ch13-pupil" cx="45%" cy="42%" r="58%">
      <stop offset="0%" stop-color="#020617"/>
      <stop offset="68%" stop-color="#0f172a"/>
      <stop offset="100%" stop-color="#451a03"/>
    </radialGradient>
    <linearGradient id="ch13-gold-mira" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#fef08a"/>
      <stop offset="45%" stop-color="#fbbf24"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <radialGradient id="ch13-gold-core" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#ffffff"/>
      <stop offset="35%" stop-color="#fde047"/>
      <stop offset="100%" stop-color="#d97706"/>
    </radialGradient>
    <!-- Ink curve gradient -->
    <linearGradient id="ch13-ink-curve" x1="0%" y1="0%" x2="100%" y2="0%">
      <stop offset="0%" stop-color="var(--logo-ink, #0f172a)"/>
      <stop offset="50%" stop-color="var(--logo-sub, #1d4ed8)"/>
      <stop offset="100%" stop-color="var(--logo-ink, #0f172a)"/>
    </linearGradient>
    <linearGradient id="ch13-stock-column" x1="0" y1="94" x2="0" y2="48" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#065f46"/>
      <stop offset="50%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#34d399"/>
    </linearGradient>
    <linearGradient id="ch13-rate-curve" x1="22" y1="84" x2="106" y2="38" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#1e3a8a"/>
      <stop offset="50%" stop-color="#2563eb"/>
      <stop offset="100%" stop-color="#38bdf8"/>
    </linearGradient>
  </defs>
  <g>
    <circle cx="64" cy="66" r="48" fill="url(#ch13-glow)"/>
    <line x1="20" y1="94" x2="108" y2="94" stroke="var(--logo-ticks, #64748b)" stroke-width="2.0" stroke-linecap="round"/>
    <rect x="28" y="70" width="12" height="24" rx="2" fill="url(#ch13-stock-column)" fill-opacity="0.65" stroke="var(--logo-green, #10b981)" stroke-width="1.6"/>
    <rect x="42" y="48" width="12" height="46" rx="2" fill="url(#ch13-stock-column)" fill-opacity="0.75" stroke="var(--logo-green, #10b981)" stroke-width="1.6"/>
    <rect x="56" y="52" width="12" height="42" rx="2" fill="url(#ch13-stock-column)" fill-opacity="0.80" stroke="var(--logo-green, #10b981)" stroke-width="1.6"/>
    <rect x="70" y="72" width="12" height="22" rx="2" fill="url(#ch13-stock-column)" fill-opacity="0.70" stroke="var(--logo-green, #10b981)" stroke-width="1.6"/>
    <rect x="84" y="60" width="12" height="34" rx="2" fill="url(#ch13-stock-column)" fill-opacity="0.75" stroke="var(--logo-green, #10b981)" stroke-width="1.6"/>
    <path d="M 22 84 C 40 42, 58 42, 74 68 C 86 84, 94 62, 106 38" stroke="url(#ch13-rate-curve)" stroke-width="3.4" stroke-linecap="round"/>
    <circle cx="24" cy="94" r="8.5" fill="url(#ch13-node-well)" stroke="url(#ch13-ring-vital)" stroke-width="3.0"/>
    <circle cx="24" cy="94" r="3.8" fill="url(#ch13-core-vital)"/>
    <g>
      <circle cx="65" cy="36" r="7.4" fill="var(--logo-node-bg, #e2e8f0)"/>
      <circle cx="65" cy="36" r="6.4" fill="url(#ch13-pupil)" stroke="url(#ch13-gold-mira)" stroke-width="2.4"/>
      <line x1="55.40" y1="36.00" x2="60.80" y2="36.00" stroke="url(#ch13-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="69.20" y1="36.00" x2="74.60" y2="36.00" stroke="url(#ch13-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="65.00" y1="26.40" x2="65.00" y2="31.80" stroke="url(#ch13-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="65.00" y1="40.20" x2="65.00" y2="45.60" stroke="url(#ch13-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <circle cx="65" cy="36" r="2.2" fill="url(#ch13-gold-core)"/>
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

> Accumulation turns flows into totals: why an expectation, and a stock drawdown, is an integral rather
> than a discrete sum.

## By the end of this chapter you can

1. Read a definite integral as stock-flow accumulation and Lebesgue expectation as level-weighted integration on $\mathbb{E}[|X|] < \infty$, computing means via the tail-integral identity.
2. State when Dominated Convergence, $\mathrm{CRPS}$, and coherent $\mathrm{CVaR}\_p$ hold or break across tail regimes ($\alpha \le 1$ vs. $1 < \alpha \le 2$).
3. Derive Manski bounds by partitioning $\Omega$, and say why bounded support and assignment govern width.

## 1. Accumulation and Integration

A reservoir operator in a drought sees only an instantaneous outflow meter. Whether the town runs dry before rains return depends on total gallons drained over time, the accumulated stock change that turns a flow rate into an exhausted tank.

Start by accumulating rectangular slices. Suppose water drains at $2$ units per hour for the first $3$ hours and $4$ units per hour for the next $2$ hours. Total outflow adds the slice areas: $2(3) + 4(2) = 14$ units. For a changing continuous rate $f(t)$ on $[a, b]$, slice the interval into $n$ narrow strips of width $\Delta t\_i$ with sample heights $f(t\_i^\ast)$. Add the strip areas and let the widest slice $\max\_i \Delta t\_i$ shrink toward $0$ to define the **definite integral** $I$:

$$I = \int\_a^b f(t)\mkern3mudt = \lim\_{\max\_i \Delta t\_i \to 0} \sum\_{i=1}^n f(t\_i^\ast) \Delta t\_i.$$

Read that equation aloud: total accumulated change $I$ is the limit of adding strip height times strip width as slices grow arbitrarily thin. This reverses the derivative from [Chapter 12](12-derivatives-and-convexity.md). Whereas a derivative measures the *instantaneous rate* at which a curve climbs or drains, an integral adds those rates back together to recover the *total net change*.

Two basic rules govern all integrals of integrable functions $f, g$ on $[a, c]$ ($a \le b \le c$):

- **Linearity:** $\int\_a^b (c\_1 f(t) + c\_2 g(t))\mkern3mudt = c\_1 \int\_a^b f(t)\mkern3mudt + c\_2 \int\_a^b g(t)\mkern3mudt$ for constants $c\_1, c\_2 \in \mathbb{R}$.
- **Interval additivity:** $\int\_a^c f(t)\mkern3mudt = \int\_a^b f(t)\mkern3mudt + \int\_b^c f(t)\mkern3mudt$.

Now connect accumulation to physical survival. Hold an initial reserve stock $R\_0 \in (0, \infty)$ against a locally integrable drawdown rate $D \in L^1\_{\mathrm{loc}}([0, \infty))$ with $D(s) \ge 0$ (leaving sudden jumps to [Chapter 11](11-sequences-limits-continuity.md) and [Chapter 46](46-non-identical-asymmetry-heavy-tails.md)). After elapsed time $t \ge 0$, remaining stock is $R\_t = R\_0 - \int\_0^t D(s)\mkern3muds$ (so $-R'\_t = D(t)$ wherever smooth). Avoiding depletion before an exogenous recovery duration $T\_{\mathrm{rec}} > 0$ requires $\int\_0^{T\_{\mathrm{rec}}} D(s)\mkern3muds \le R\_0$: survival time must meet or exceed recovery time (Simchi-Levi et al. 2014). That scalar stock-flow check is necessary yet never sufficient by itself. Soft budgets (Kornai 1986) or costs pushed onto outside communities can disguise depletion, and hoarding hits opportunity-cost ceilings (Scarf 1960; Samuelson 1971). Queue delays and shock dynamics belong to [Chapter 26](26-regulation-and-homeostasis.md), [Chapter 38](38-time-and-dependence.md), and [Chapter 47](47-irreversible-time-vital-margin.md).

## 2. Expectation as a Lebesgue Integral

To see how **expectation** works as weighted accumulation, start with a three-outcome loss $X$: it costs $0$ with probability $0.8$, $10$ with probability $0.1$, and $50$ with probability $0.1$. Weight each outcome by its probability and add them up: $0(0.8) + 10(0.1) + 50(0.1) = 6$. Notice what you just did: you grouped worlds by *outcome level* ($0$, $10$, or $50$) rather than ordering them along a clock.

That vertical grouping is Henri Lebesgue's insight. Riemann integration slices the horizontal time axis of $f(t)$ step by step. By contrast, Lebesgue integration slices the vertical outcome range of a random variable $X : \Omega \to \mathbb{R}$ by level. It groups all world states $\omega \in \Omega$ with $X(\omega) \approx x$, weights each slice by its probability $\mathbb{P}$, and adds across levels.

Work on a probability triple $(\Omega, \mathcal{F}, \mathbb{P})$ satisfying non-negativity ($\mathbb{P}(A) \ge 0$), unit total mass ($\mathbb{P}(\Omega) = 1$), and countable additivity on disjoint events (Kolmogorov 1933; [Chapter 15](15-probability-as-a-measure.md)). Split $X$ into its positive part $X^+ = \max(X, 0)$ and negative part $X^- = \max(-X, 0)$. Whenever at least one of $\mathbb{E}[X^+]$ or $\mathbb{E}[X^-]$ is finite, expectation $\mathbb{E}[X]$ is defined as their difference, and is finite **if and only if** $|X|$ has a finite integral ($\mathbb{E}[|X|] = \int\_\Omega |X|\mkern3mud\mathbb{P} < \infty$):

$$\mathbb{E}[X] = \int\_\Omega X(\omega)\mkern3mud\mathbb{P}(\omega) = \mathbb{E}[X^+] - \mathbb{E}[X^-].$$

Read that aloud: expected value equals the probability-weighted integral of positive outcomes minus that of negative outcomes. Discrete tables and continuous curves obey the exact same definition. When $X$ carries a continuous density $p(x) \ge 0$ ($\int\_{-\infty}^\infty p(x)\mkern3mudx = 1$) with finite absolute integral $\int\_{-\infty}^\infty |x|\mkern2mup(x)\mkern3mudx < \infty$, the formula specializes to $\mathbb{E}[X] = \int\_{-\infty}^\infty x\mkern2mup(x)\mkern3mudx$. Merely having area $1$ is not enough: a Cauchy density integrates to $1$ yet sends both $\mathbb{E}[X^+]$ and $\mathbb{E}[X^-]$ to $+\infty$, leaving $\mathbb{E}[X]$ undefined ($\infty - \infty$).

For any non-negative loss $X \ge 0$, Tonelli's theorem reveals a clean geometric shortcut, the **tail-integral identity** (finite or $+\infty$):

$$\mathbb{E}[X] = \int\_0^\infty \mathbb{P}(X > t)\mkern3mudt.$$

Read that aloud: the expected value of a non-negative variable equals the total area underneath its survival curve $S(t) = \mathbb{P}(X > t)$. Verify it by hand on our three-outcome toy example ($0$ at $0.8$, $10$ at $0.1$, $50$ at $0.1$). On $t \in [0, 10)$, $X > t$ holds for $10$ and $50$, giving $S(t) = 0.2$ across width $10$ (area $2$). On $t \in [10, 50)$, $X > t$ holds only for $50$, giving $S(t) = 0.1$ across width $40$ (area $4$). Their sum is $2 + 4 = 6$, matching our weighted average.

The proof writes $X(\omega) = \int\_0^\infty \mathbf{1}\lbrace X(\omega) > t \rbrace\mkern3mudt$ and swaps integration order via Tonelli–Fubini: $\mathbb{E}[X] = \int\_\Omega \bigl(\int\_0^\infty \mathbf{1}\lbrace X(\omega) > t \rbrace\mkern3mudt\bigr)\mkern3mud\mathbb{P}(\omega) = \int\_0^\infty \mathbb{P}(X > t)\mkern3mudt$. Exponential and Gaussian curves decay quickly and keep that tail area finite. By contrast, a Pareto survival curve $S(t) = (t/x\_m)^{-\alpha}$ on $t \ge x\_m > 0$ ($\alpha > 0$) gives $\mathbb{E}[X] = x\_m + \int\_{x\_m}^\infty (t/x\_m)^{-\alpha}\mkern3mudt = \frac{\alpha}{\alpha - 1}x\_m < \infty$ only for $\alpha > 1$, diverging to $+\infty$ as $\alpha \to 1^+$.

That same threshold integral scores forecast distributions $F$ against realized values $y \in \mathbb{R}$ using the **Continuous Ranked Probability Score** ($\mathrm{CRPS}$; Hersbach 2000; Gneiting & Raftery 2007; [Chapter 46](46-non-identical-asymmetry-heavy-tails.md)). Writing $X, X' \stackrel{\mathrm{i.i.d.}}{\sim} F$ for independent draws from forecast $F$, $\mathcal{P}$ for all Borel distributions, and $\mathcal{P}\_1$ for finite-mean laws ($\mathbb{E}\_F|X| < \infty$):

$$\mathrm{CRPS}(F, y) = \int\_{-\infty}^\infty \bigl(F(t) - \mathbf{1}\lbrace y \le t \rbrace\bigr)^2\mkern3mudt \stackrel{\mathcal{P}\_1}{=} \mathbb{E}\_F|X - y| - \frac{1}{2}\mathbb{E}\_F|X - X'|.$$

Read the left integral aloud: at every threshold $t$, square the gap between the forecast cumulative curve $F(t)$ and the step jump $\mathbf{1}\lbrace y \le t \rbrace$ marking where $y$ actually landed, then integrate across all thresholds $t$. Three regularity regimes govern propriety (Gneiting & Raftery 2007, §4.2): on all Borel distributions $\mathcal{P}$, unshifted CRPS is **proper** only in the extended-real expectation sense $\mathbb{E}\_G[\mathrm{CRPS}(F, Y)] \in [0, +\infty]$ (via Tonelli, tying vacuously at $+\infty \le +\infty$ whenever $\mathbb{E}\_G|Y| = \infty$); pointwise finiteness of $\mathrm{CRPS}(F, y)$ and finite pairwise score differences require the finite-$L^2$-tail-integral subclass $\mathcal{P}\_{1/2}$ satisfying $\int\_{-\infty}^0 F(t)^2\,dt + \int\_0^\infty (1 - F(t))^2\,dt < \infty$ (Pareto $\alpha > 1/2$, whereas for $\alpha \le 1/2$, $\mathrm{CRPS}(G, y) \equiv +\infty$ for all $y$ and differences against $F = \delta\_0$ drop to $-\infty$); and **strict propriety** with finite expected scores and the right-hand absolute-difference kernel holds on the finite-first-moment subclass $\mathcal{P}\_1$ ($\alpha > 1$ for pure Pareto, while $\alpha \in (1/2, 1]$ sends $\mathbb{E}[\mathrm{CRPS}] = +\infty$ and collapses the kernel form to $\infty - \infty$). A Dirac point prediction $F = \delta\_{\hat{y}}$ simplifies CRPS to $|\hat{y} - y|$ (Mean Absolute Error, evaluated in $\mathcal{O}(1)$ time), whereas an $m$-member empirical ensemble $F\_m = \frac{1}{m}\sum\_{i=1}^m \delta\_{x\_i}$ evaluates in $\mathcal{O}(m \log m)$ time ($\mathcal{O}(m)$ once sorted).

## 3. Limit-Integral Exchange and Fat-Tailed Failures

When can you pass a limit inside an integral so $\lim\_{n\to\infty} \int f\_n = \int \lim\_{n\to\infty} f\_n$? Lebesgue's **Dominated Convergence Theorem** supplies a crisp rule: if measurable functions $f\_n$ converge pointwise to $f$ almost everywhere and one fixed integrable ceiling $g$ traps them all ($|f\_n(x)| \le g(x)$ a.e. with $\int |g| < \infty$), you may safely swap limit and integral.

Keep two distinct failure modes separate. For independent and identically distributed (`i.i.d.`) draws with a finite mean ($\mathbb{E}[|X\_1|] < \infty$, covering pure Pareto $\alpha > 1$ even when $1 < \alpha \le 2$), Kolmogorov's Strong Law ensures sample averages $\bar{X}\_n = \frac{1}{n}\sum\_{i=1}^n X\_i$ converge to $\mathbb{E}[X\_1]$ almost surely and in $L^1$, keeping $\lbrace \bar{X}\_n \rbrace\_{n \ge 1}$ uniformly integrable. Under $\alpha \le 1$ (pure Pareto), integrability fails at the start: non-negative $X\_i \ge 0$ send $\bar{X}\_n \xrightarrow{\mathrm{a.s.}} +\infty = \mathbb{E}[X\_1]$, while symmetric Cauchy draws ($\alpha = 1$) never converge at all. Limit–integral breakdown ($\lim\_{n\to\infty} \mathbb{E}[f\_n] \ne \mathbb{E}[\lim\_{n\to\infty} f\_n]$) happens instead when a sequence $f\_n \to f$ a.e. escapes every integrable ceiling $g$.

Test that escape with concentrating spikes $f\_n(x) = n\mkern3mu\mathbf{1}\_{(0, 1/n)}(x)$ on $(0, 1)$. For $n = 2$, this is a rectangle of width $1/2$ and height $2$, with area $1$; for $n = 10$, width $1/10$ and height $10$, still with area $1$. Yet fix any point $x \in (0, 1)$: once $n \ge 1/x$, $f\_n(x) = 0$, so the pointwise limit is $0$ everywhere, giving $\int\_0^1 \lim\_{n\to\infty} f\_n(x)\mkern3mudx = 0 \ne 1 = \lim\_{n\to\infty} \int\_0^1 f\_n(x)\mkern3mudx$. Dominated convergence fails because the envelope $\sup\_{n \ge 1} f\_n(x) = \lceil 1/x \rceil - 1 \ge 1/x - 1$ has integral $\sum\_{k=1}^\infty k\bigl(\frac{1}{k} - \frac{1}{k+1}\bigr) = \sum\_{k=1}^\infty \frac{1}{k+1} = \infty$, leaving no integrable dominant $g$.

Under pure Pareto decay $S(x) = L(x)x^{-\alpha}$ ($\alpha > 0$, slowly varying factor $L(x) \equiv c > 0$ distinct from §4's loss $L$; Embrechts, Klüppelberg & Mikosch 1997), moments switch sharply at integer thresholds. (Decaying $L(x)$ such as $(\ln x)^{-2}$ can restore finiteness right on those boundaries, [Chapter 31](31-distribution-and-moment.md).) For pure Pareto, $\alpha \le 1$ diverges $\mathbb{E}[X]$; $1 < \alpha \le 2$ keeps $\mathbb{E}[X] < \infty$ while $\mathbb{E}[X^2] = \int\_0^\infty 2t\mkern3mu\mathbb{P}(X > t)\mkern3mudt = \infty$; and both moments exist for $\alpha > 2$. A finite-sample Hill interval at cut-off $k$ summarizes evidence inside that window rather than proving asymptotic moments ([Chapter 31](31-distribution-and-moment.md); [Chapter 46](46-non-identical-asymmetry-heavy-tails.md)). Intervals sitting cleanly below or above a boundary inform classification, while straddling intervals keep judgment open.

## 4. Coherent Risk Measures: Expected Shortfall vs. Value-at-Risk

A reinsurance committee pricing a disaster layer cannot stop at a high quantile. Imagine $10$ equally likely scenarios where $9$ lose $0$ and the worst $1$ loses $100$. Ask for the $90\%$ threshold (**Value-at-Risk**, $\mathrm{VaR}\_{0.9}$), and the answer is $0$, drawing a curtain right across the catastrophe. Ask instead for the average loss across the worst $10\%$ (**Expected Shortfall**, $\mathrm{CVaR}\_{0.9}$), and you see $100$. A balance sheet cannot pay a curtain; it must pay the accumulated severity past the threshold.

Working in **loss convention** where $X$ measures positive harm (Artzner et al. 1999 state axioms for net worth with reversed signs), Philippe Artzner et al. (1999) proposed four axioms for **coherent risk measures**: monotonicity, subadditivity, positive homogeneity, and translation invariance. Subadditivity ($\rho(X + Y) \le \rho(X) + \rho(Y)$, writing $\rho$ for the risk functional) guarantees diversification never raises measured risk. At quantile level $p \in (0, 1)$, Value-at-Risk $\mathrm{VaR}\_p(X) = \inf\lbrace x \in \mathbb{R} : \mathbb{P}(X \le x) \ge p \rbrace$ violates subadditivity: merging disjoint fat-tailed positions can jump $\mathrm{VaR}\_p$ past the sum of standalone thresholds.

For any loss with finite positive expectation ($\mathbb{E}[X^+] < \infty$, covering $\alpha > 1$ for pure Pareto), Carlo Acerbi and Dirk Tasche (2002) proved across continuous and discrete laws that **Expected Shortfall** ($\mathrm{CVaR}\_p$) satisfies all coherence axioms. Their proof fixes the subadditivity failure of naive tail averages at discrete jumps (Artzner et al. 1999) by integrating all upper-tail quantiles from $p$ to $1$:

$$\mathrm{CVaR}\_p(X) = \frac{1}{1 - p} \int\_p^1 \mathrm{VaR}\_u(X)\mkern3mudu = \frac{1}{1 - p}\Bigl(\mathbb{E}\bigl[X\mkern2mu\mathbf{1}\lbrace X > \mathrm{VaR}\_p(X) \rbrace\bigr] + \mathrm{VaR}\_p(X)\bigl(1 - p - \mathbb{P}(X > \mathrm{VaR}\_p(X))\bigr)\Bigr).$$

Read that aloud: $\mathrm{CVaR}\_p(X)$ averages the quantile curve $\mathrm{VaR}\_u(X)$ across the worst $1 - p$ tail share, splitting any boundary point atom cleanly so tail mass totals $1 - p$ (reducing simply to $\mathbb{E}[X \mid X \ge \mathrm{VaR}\_p(X)]$ when $\mathbb{P}(X = \mathrm{VaR}\_p(X)) = 0$). Three cases at illustrative level $p = 0.95$ clarify the contrast:

- **Comonotonic limit ($Y = X$):** Lockstep risks double linearly under both: $\mathrm{VaR}\_p(2X) = 2\mkern3mu\mathrm{VaR}\_p(X)$ and $\mathrm{CVaR}\_p(2X) = 2\mkern3mu\mathrm{CVaR}\_p(X)$.
- **Disjoint tail concentration (Acerbi & Tasche 2002):** For $p \in [1/2, 1)$, let two positions $A$ and $B$ each lose $L > 0$ with probability $1 - p$ on mutually exclusive events and $0$ otherwise. Because $\mathbb{P}(A \le 0) = p$, standalone $\mathrm{VaR}\_p(A) = \mathrm{VaR}\_p(B) = 0$, whereas $\mathrm{VaR}\_u(A) = L$ for $u > p$ yields $\mathrm{CVaR}\_p(A) = \mathrm{CVaR}\_p(B) = L$ (naive $\mathbb{E}[A \mid A \ge 0] = (1 - p)L$ breaks at the atom $0$). Combined, $A + B$ loses $L$ with probability $2(1 - p) > 1 - p$, making $\mathrm{VaR}\_p(A + B) = L > 0 + 0$ (non-subadditive) while $\mathrm{CVaR}\_p(A + B) = L \le L + L$ remains coherent.
- **Strictly bounded deposit contracts:** Suppose an illustrative clearinghouse contract caps total liability at deposit $q\_{0.95}$. It returns that deposit when $X \le q\_{0.95}$ and forfeits it on breach with zero residual liability spilled onto survivors or external reception perimeters, namely outside bystanders and habitats ([Chapter 46](46-non-identical-asymmetry-heavy-tails.md); [Chapter 50](50-methodological-families-reflexivity.md) Brake 2). Only when physical and legal obligation truly stops at $q\_{0.95}$ does posting $\mathrm{CVaR}\_{0.95}$ lock up extra capital without buying trigger protection.

### Two numbers for the same tail

Ask a file for its worst few per cent and it gives both threshold $\mathrm{VaR}\_p(X)$ and shortfall $\mathrm{CVaR}\_p(X)$. Whenever outcomes can exceed the threshold ($\mathbb{P}(X > \mathrm{VaR}\_p(X)) > 0$), $\mathrm{CVaR}\_p$ exceeds $\mathrm{VaR}\_p$. A threshold marks where trouble starts, whereas the integral tells you what the institution actually owes. Cap logged claims at a ceiling $K$ and reported shortfall drops while unrecorded excess $(X - K)\_+$ remains exposed ([Chapter 46](46-non-identical-asymmetry-heavy-tails.md)). A reporting cap is therefore a choice to look away, not a measurement. Furthermore, single-portfolio coherence cannot stop system-wide contagion when shared exposures or crisis correlations couple portfolios into heavy-tailed spillovers (Embrechts, Klüppelberg & Mikosch 1997).

## 5. Partial Identification and Worst-Case Bounds

Selective records leave the integral domain incomplete. Test the logic on concrete numbers before writing symbols. Suppose outcomes $Y$ must lie between $0$ and $10$ ($y\_{\min} = 0$, $y\_{\max} = 10$). We observe $80\%$ of cases ($p\_{\mathrm{obs}} = 0.8$) with observed mean $4$, while $20\%$ ($1 - p\_{\mathrm{obs}} = 0.2$) are missing. If every missing case was $0$, the true mean is $0.8(4) + 0.2(0) = 3.2$. If every missing case was $10$, the true mean is $0.8(4) + 0.2(10) = 5.2$. Therefore $\mathbb{E}[Y]$ must sit inside $[3.2, 5.2]$, an interval of width $0.2(10 - 0) = 2.0$.

Charles Manski's partial identification framework (Manski 1990, 2003, 2026) generalizes that arithmetic via Lebesgue integration. When outcome $Y$ lies on **bounded support** $[y\_{\min}, y\_{\max}]$ ($-\infty < y\_{\min} \le y\_{\max} < \infty$) and is observed only when recording indicator $R \in \lbrace 0, 1 \rbrace$ (distinct from §1's reserve stock $R\_t$) equals $R = 1$ with probability $p\_{\mathrm{obs}} := \mathbb{P}(R = 1) \in (0, 1)$, splitting $\Omega = \lbrace R = 1 \rbrace \cup \lbrace R = 0 \rbrace$ partitions the integral into observed and unobserved parts:

$$\mathbb{E}[Y] = \int\_{\lbrace R=1\rbrace} Y\mkern3mud\mathbb{P} + \int\_{\lbrace R=0\rbrace} Y\mkern3mud\mathbb{P} = p\_{\mathrm{obs}}\mkern3mu\mathbb{E}[Y \mid R = 1] + (1 - p\_{\mathrm{obs}})\mkern3mu\mathbb{E}[Y \mid R = 0].$$

Because $\mathbb{E}[Y \mid R = 0]$ must lie inside $[y\_{\min}, y\_{\max}]$, substituting the two extremes brackets the unconditional mean:

$$\mathbb{E}[Y] \in \bigl[p\_{\mathrm{obs}}\mkern2mu\mathbb{E}[Y \mid R=1] + (1 - p\_{\mathrm{obs}})y\_{\min},\mkern5mu p\_{\mathrm{obs}}\mkern2mu\mathbb{E}[Y \mid R=1] + (1 - p\_{\mathrm{obs}})y\_{\max}\bigr].$$

Read that aloud: the interval anchors on the observed mean scaled by $p\_{\mathrm{obs}}$, then widens outward by the missing share $(1 - p\_{\mathrm{obs}})$ times the full support range, fixing width $\Delta\_{\mathrm{id}} = (1 - p\_{\mathrm{obs}})(y\_{\max} - y\_{\min})$ (which collapses to $0$ for non-degenerate support $y\_{\min} < y\_{\max}$ only when $p\_{\mathrm{obs}} = 1$). Finite bounds $y\_{\min}, y\_{\max}$ are indispensable: under unbounded heavy tails ($y\_{\max} = +\infty$), any missing share $1 - p\_{\mathrm{obs}} > 0$ sends the upper bound to $+\infty$ ([Chapter 44](44-underdetermination-partial-identification.md); [Chapter 46](46-non-identical-asymmetry-heavy-tails.md)). In two-arm causal comparisons ($\mathrm{ATE} = \mathbb{E}[Y(1)] - \mathbb{E}[Y(0)]$), treatment assignment bias and follow-up attrition combine additively inside the difference interval of [Numbers, Order, and Bounds](10-numbers-order-bounds.md) §3 (Manski 1990, 2026; [Chapter 44](44-underdetermination-partial-identification.md)).

### When the tail is not integrable

This shortfall machinery breaks down at $\alpha \le 1$. Without a finite mean, both $\mathrm{CVaR}\_p$ and strict $\mathcal{P}\_1$ propriety of $\mathrm{CRPS}$ fail. Across $1 < \alpha \le 2$, population $\mathrm{CVaR}\_p$ exists yet tail variance diverges, blocking standard $\sqrt{n}$ confidence bands ([Chapter 46](46-non-identical-asymmetry-heavy-tails.md)), and standalone $\mathrm{CVaR}\_p$ is not elicitable on its own by any strictly consistent point scoring function (Gneiting 2011), leaving operational workflows to inspect it beside quantiles or full-distribution scores. Likewise, unbounded heavy tails ($y\_{\max} = +\infty$) with $1 - p\_{\mathrm{obs}} > 0$ send Manski upper bounds to $+\infty$. Bounded support keeps distribution-free bounds informative, whereas unbounded tails must pair conditional Hill diagnostics ([Chapter 31](31-distribution-and-moment.md); Embrechts, Klüppelberg & Mikosch 1997) with full-distribution scoring rules before Part IV sets operational limits.

## Bridge

[Chapter 12](12-derivatives-and-convexity.md) pairs curvature with this chapter's accumulation integral, which [Chapter 26](26-regulation-and-homeostasis.md), [Chapter 38](38-time-and-dependence.md), and [Chapter 47](47-irreversible-time-vital-margin.md) extend to queuing, jump shocks, and recovery clocks across the three-plane circuit ([Chapter 29](29-aim-and-the-three-planes.md)). [Chapter 31](31-distribution-and-moment.md) and [Chapter 46](46-non-identical-asymmetry-heavy-tails.md) develop moment boundaries, $\mathrm{CRPS}$, and coherent $\mathrm{CVaR}\_p$ alongside external reception-perimeter limits, while [Chapter 35](35-inference-and-evidence.md) and [Chapter 44](44-underdetermination-partial-identification.md) formalize two-arm Manski bounds.

[Linear Algebra](14-linear-algebra.md) next turns from scalar integrals to vectors, matrices, rank, and spectral stability (starting from two-dimensional arrows before building projections and high-dimensional geometry) before [Chapter 15](15-probability-as-a-measure.md) closes Part II's mathematical bases on $(\Omega, \mathcal{F}, \mathbb{P})$.
