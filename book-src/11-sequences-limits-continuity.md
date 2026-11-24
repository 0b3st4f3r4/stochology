# Sequences, Limits, and Continuity

<figure class="stoch-figure stoch-logo-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="800" height="132" viewBox="0 0 800 132" fill="none" role="img" aria-label="Stochology CH11 · CONTINUITY — Sequences, Limits, And Stability">
  <defs>
    <!-- Canonical Ring Gradient -->
    <linearGradient id="ch11-ring" x1="2" y1="78" x2="386" y2="78" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="38%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="68%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <!-- 1. Atmospheric Cosmic Field -->
    <radialGradient id="ch11-glow" cx="52%" cy="52%" r="50%">
      <stop offset="0%" stop-color="#fbbf24" stop-opacity="0.24"/>
      <stop offset="48%" stop-color="#38bdf8" stop-opacity="0.14"/>
      <stop offset="80%" stop-color="#e11d48" stop-opacity="0.08"/>
      <stop offset="100%" stop-color="#090e1a" stop-opacity="0"/>
    </radialGradient>
    <!-- 3D Spherical Node Gradients -->
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
    <!-- Stókhos Reticle Gradients -->
    <radialGradient id="ch11-pupil" cx="45%" cy="42%" r="58%">
      <stop offset="0%" stop-color="#020617"/>
      <stop offset="68%" stop-color="#0f172a"/>
      <stop offset="100%" stop-color="#451a03"/>
    </radialGradient>
    <linearGradient id="ch11-gold-mira" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#fef08a"/>
      <stop offset="45%" stop-color="#fbbf24"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <radialGradient id="ch11-gold-core" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#ffffff"/>
      <stop offset="35%" stop-color="#fde047"/>
      <stop offset="100%" stop-color="#d97706"/>
    </radialGradient>
    <!-- Ink curve gradient -->
    <linearGradient id="ch11-ink-curve" x1="0%" y1="0%" x2="100%" y2="0%">
      <stop offset="0%" stop-color="var(--logo-ink, #0f172a)"/>
      <stop offset="50%" stop-color="var(--logo-sub, #1d4ed8)"/>
      <stop offset="100%" stop-color="var(--logo-ink, #0f172a)"/>
    </linearGradient>
    <linearGradient id="ch11-corridor-fill" x1="20" y1="66" x2="106" y2="66" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#2563eb" stop-opacity="0.24"/>
      <stop offset="60%" stop-color="#38bdf8" stop-opacity="0.16"/>
      <stop offset="100%" stop-color="#fbbf24" stop-opacity="0.12"/>
    </linearGradient>
    <linearGradient id="ch11-corridor-edge" x1="20" y1="66" x2="106" y2="66" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#38bdf8"/>
      <stop offset="100%" stop-color="#fbbf24"/>
    </linearGradient>
    <linearGradient id="ch11-wave-stroke" x1="20" y1="66" x2="106" y2="66" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#2563eb"/>
      <stop offset="35%" stop-color="#38bdf8"/>
      <stop offset="70%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#fbbf24"/>
    </linearGradient>
  </defs>
  <g>
    <circle cx="64" cy="66" r="48" fill="url(#ch11-glow)"/>
    <polygon points="20,34 106,56 106,76 20,98" fill="url(#ch11-corridor-fill)"/>
    <line x1="20" y1="34" x2="106" y2="56" stroke="url(#ch11-corridor-edge)" stroke-width="2.0" stroke-dasharray="4 2.5"/>
    <line x1="20" y1="98" x2="106" y2="76" stroke="url(#ch11-corridor-edge)" stroke-width="2.0" stroke-dasharray="4 2.5"/>
    <line x1="18" y1="66" x2="110" y2="66" stroke="var(--logo-ticks, #64748b)" stroke-width="1.4" stroke-dasharray="2 3"/>
    <path d="M 20 66 Q 34 22 48 66 Q 62 98 76 66 Q 88 50 98 66 L 106 66" stroke="url(#ch11-wave-stroke)" stroke-width="3.4" stroke-linecap="round"/>
    <circle cx="48" cy="66" r="8.5" fill="url(#ch11-node-well)" stroke="url(#ch11-ring-vital)" stroke-width="3.0"/>
    <circle cx="48" cy="66" r="3.8" fill="url(#ch11-core-vital)"/>
    <circle cx="76" cy="66" r="8.5" fill="url(#ch11-node-well)" stroke="url(#ch11-ring-gov)" stroke-width="3.0"/>
    <circle cx="76" cy="66" r="3.8" fill="url(#ch11-core-gov)"/>
    <g>
      <circle cx="98" cy="66" r="7.4" fill="var(--logo-node-bg, #e2e8f0)"/>
      <circle cx="98" cy="66" r="6.4" fill="url(#ch11-pupil)" stroke="url(#ch11-gold-mira)" stroke-width="2.4"/>
      <line x1="88.40" y1="66.00" x2="93.80" y2="66.00" stroke="url(#ch11-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="102.20" y1="66.00" x2="107.60" y2="66.00" stroke="url(#ch11-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="98.00" y1="56.40" x2="98.00" y2="61.80" stroke="url(#ch11-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="98.00" y1="70.20" x2="98.00" y2="75.60" stroke="url(#ch11-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <circle cx="98" cy="66" r="2.2" fill="url(#ch11-gold-core)"/>
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

> Convergence, uniform stability, and non-ergodic path limits across time.

## By the end of this chapter you can

1. Say what it means for a sequence to converge, without a circular appeal to "getting closer".
2. Distinguish a limit from a value, and explain why a function can be continuous everywhere except
   at the one point a decision depends on.
3. Say what continuity is not, using a case where small causes have large effects.

## 1. Sequences and Cauchy Convergence

A water authority watches a rolling average of reservoir inflows hover inside a tight band for thirty months and declares that inflows have stabilized permanently. The multi-year drought that drops the baseline arrives the next month. Watching recent points stay close together proves only that recent months were calm, not that an infinite sequence has settled.

A **real sequence** $(a\_n)\_{n=1}^\infty$ lists one real number $a\_n \in \mathbb{R}$ for each step $n \in \mathbb{Z}\_{\ge 1}$ (i.e., $n \in \mathbb{N} \setminus \lbrace 0 \rbrace$, keeping $\mathbb{N} = \lbrace 0, 1, 2, \dots \rbrace$ from [Chapter 10](10-numbers-order-bounds.md); or elements in a metric space $(X, d)$ where distance $|a\_n - L|$ becomes $d(a\_n, L)$). Before writing the formal rule, compare three concrete sequences by hand:

- **Settling down:** $a\_n = 1/n$ produces $1,\; 1/2,\; 1/3,\; 1/4, \dots$, squeezing steadily closer to $0$ and staying there.
- **Persistent bouncing:** $b\_n = (-1)^n$ produces $-1,\; 1,\; -1,\; 1, \dots$, remaining bounded between $-1$ and $1$ without ever settling on one target.
- **Deceptive slow drift:** $c\_n = \sum\_{k=1}^n 1/k$ starts as $1,\; 1.5,\; 1.833\dots,\; 2.083\dots$. Adjacent steps shrink toward zero ($c\_{n+1} - c\_n = 1/(n+1)$), yet adding enough terms pushes $c\_n$ past every ceiling toward $+\infty$.

That third example explains why formal analysis never defines convergence as "neighboring terms getting close." Instead, sequence $(a\_n)$ **converges** to limit $L \in \mathbb{R}$ when every tolerance $\epsilon > 0$ admits a cutoff index $N(\epsilon) \in \mathbb{Z}\_{\ge 1}$ such that all $n \ge N(\epsilon)$ stay inside tolerance:

$$|a\_n - L| < \epsilon.$$

Read that condition as an adversarial challenge. A skeptic chooses the error budget $\epsilon$ first, and you must answer with a starting step $N(\epsilon)$ beyond which every single term $a\_n$ stays within $\epsilon$ of $L$. For $a\_n = 1/n$ and $\epsilon = 0.01$, choosing $N = 101$ works because $1/n < 0.01$ for every $n \ge 101$.

A **Cauchy sequence** demands something stronger than shrinking adjacent steps: for every $\epsilon > 0$, there exists $N$ such that *all* pairs $n, m \ge N$ satisfy $|a\_n - a\_m| < \epsilon$, not just neighbors $n$ and $n + 1$. On the gapless real line $\mathbb{R}$ ([Chapter 10](10-numbers-order-bounds.md)), Cauchy completeness guarantees that every Cauchy sequence converges to a real limit $L \in \mathbb{R}$. Well before modern European analysis, Mādhava of Sangamagrāma and the Kerala school built converging series expansions for trigonometric functions and $\pi/4$, pairing partial sums with remainder bounds to sharpen finite approximations (Plofker 2009).

Even when an infinite limit exists ($n \to \infty$), finite samples give no speed guarantee. A rate of $\mathcal{O}(1/\ln n)$ demands astronomical sample sizes for modest accuracy. Heavy power-law tails $\mathbb{P}(|X| > x) = L(x)x^{-\alpha}$ ($\alpha > 0$, $L$ slowly varying; [Chapter 10](10-numbers-order-bounds.md)) sharpen those finite-sample warnings across three tail regimes:

- **Finite variance ($\alpha > 2$, or $\alpha = 2$ with $\int^\infty x^{-1} L(x)\,dx < \infty$, such as $L(x) = (\ln x)^{-2}$):** under short-range dependence, the sample mean settles at standard error $\mathcal{O}(n^{-1/2})$ ([Distribution and Moment](31-distribution-and-moment.md)).
- **Finite variance yet infinite fourth moment ($2 < \alpha < 4$, or $\alpha = 2$ with $\int^\infty x^{-1} L(x)\,dx < \infty$, or $\alpha = 4$ with $\int^\infty x^{-1} L(x)\,dx = \infty$, such as constant $L$ or $L(x) = (\ln x)^{-1}$):** under ergodicity or short-range dependence, the running second-moment average $\hat{m}\_{2,n} = \frac{1}{n}\sum\_{i=1}^n X\_i^2$ still converges to $\mathbb{E}[X^2]$, yet $\mathrm{Var}(X^2) = \infty$ prevents $\mathcal{O}(n^{-1/2})$ standard errors. Achieving $\mathcal{O}(n^{-1/2})$ precision for $\hat{m}\_{2,n}$ requires finite fourth moment ($\alpha > 4$, or $\alpha = 4$ with $\int^\infty x^{-1} L(x)\,dx < \infty$) and bounded long-run variance—in particular, $\mathbb{E}[X^4] < \infty$ together with absolutely summable autocovariances of $X\_t^2$ is sufficient.
- **Infinite variance ($\alpha < 2$, or $\alpha = 2$ with $\int^\infty x^{-1} L(x)\,dx = \infty$, even when $L(x) = (\ln x)^{-1}$ decays to zero):** running variance fails the Cauchy test and lurches upward whenever a fresh record arrives. Under regime shifts or long-memory persistence, calm prefixes give false comfort right before conditions break.

## 2. Limits, Continuity, and Uniform Stability

An emergency department tracks triage waiting time against bed occupancy across the middle of a normal week and observes a gentle straight-line slope. Managers expect another tiny rise in occupancy to add only a few minutes, forgetting that a smooth curve can shoot upward near a ceiling. Pointwise continuity promises small output changes for tiny nudges at a calm interior point; it promises nothing near a capacity cliff.

To build the intuition for a **function limit**, check a function with a single missing pinhole: $f(x) = (x^2 - 1)/(x - 1)$. At every $x \ne 1$, factoring gives $f(x) = x + 1$, so near $x = 1$ the values $f(0.99) = 1.99$ and $f(1.01) = 2.01$ crowd around $2$. Even if $f(1)$ is undefined or set to $99$, the limit as $x$ approaches $1$ is still $2$.

Formally, for a function $f : D \to \mathbb{R}$ and a limit point $x\_0$ of domain $D$, we write $\lim\_{x \to x\_0} f(x) = L$ when every tolerance $\epsilon > 0$ admits a neighborhood radius $\delta > 0$ such that $0 < |x - x\_0| < \delta$ ($x \in D$) forces $|f(x) - L| < \epsilon$. Notice the condition $0 < |x - x\_0|$ skips $x\_0$ itself. A function is **continuous at $x\_0$** when $x\_0 \in D$ and its value actually meets its limit: $\lim\_{x \to x\_0} f(x) = f(x\_0)$ (at any isolated point $x\_0 \in D$ with no other points of $D$ inside some $\delta > 0$, continuity holds automatically via $|x - x\_0| < \delta \implies |f(x) - f(x\_0)| < \epsilon$).

Continuity splits into two strengths. **Pointwise continuity** allows the safe step size $\delta(x, \epsilon)$ to shrink depending on where $x$ stands. **Uniform continuity** requires one global step size $\delta(\epsilon) > 0$ that works across the entire domain $D$: any $x, y \in D$ with $|x - y| < \delta(\epsilon)$ must keep $|f(x) - f(y)| < \epsilon$.

Test that contrast on two elementary functions. On $(0, \infty)$, $f(x) = 1/x$ is continuous at every point, yet fails uniform continuity near $0$: squeeze $x$ and $y$ inside $(0, \delta)$ and $|1/x - 1/y|$ still blows up arbitrarily large. By contrast, $f(x) = \sqrt{x}$ on $(0, 1]$ is uniformly continuous even though its slope $f'(x)$ diverges as $x \to 0^+$. Rules governing human institutions need uniform stability so tiny measurement errors cannot trigger wild operational swings.

Three kinds of breakdown turn small shifts into sudden danger. **Jump discontinuities** flip behavior abruptly at a threshold, such as triggering an emergency reserve once risk crosses probability $p^\ast$. **Asymptotic blow-ups** send delays toward infinity near a hard ceiling, such as queue waits as utilization $\rho$ approaches $1^-$. **Regime oscillations** prevent any limit from settling when delayed feedback overshoots its target.

## 3. Continuity Is Not Robustness

Before leaving stability, look at how repeated steps settle or explode around a **fixed point** $x^\ast$ where a rule maps a state back onto itself ($g(x^\ast) = x^\ast$). Test two simple updates starting from $x\_0 = 0.01$:

- Under $g(x) = 0.5 x$, the fixed point $x^\ast = 0$ is **stable**: steps halve ($0.01,\; 0.005,\; 0.0025 \dots$) and pull perturbations back to rest.
- Under $g(x) = 2 x$, the fixed point $x^\ast = 0$ is **unstable**: steps double ($0.01,\; 0.02,\; 0.04 \dots$), turning any tiny error into exponential drift.

Even an infinitely differentiable function can amplify small errors violently. Take the power function $f(x) = x^k$ ($x \ne 0$) for a large exponent $k$. For a small relative input error $|\delta x / x| \ll 1/k$, the fractional output error $\delta f / f$ scales directly with $k$:

$$\frac{\delta f}{f} \approx k \frac{\delta x}{x}.$$

Read that equation aloud: a tiny $1\%$ error in measuring $x$ becomes roughly a $k\%$ error in $x^k$. A similar instability hits the smooth rational pole $f(x) = x/(1 - x)$ on $[0, 1)$, which drives the queue congestion curve of [Chapter 12](12-derivatives-and-convexity.md) and [Chapter 38](38-time-and-dependence.md). Although $f$ is smooth everywhere on $[0, 1)$, it explodes as $x \to 1^-$, magnifying minor load fluctuations near saturation into huge delays.

Deterministic chaos teaches the same warning. Edward Lorenz's (1963) atmospheric equations are smooth everywhere, yet two nearby starting points separate locally at exponential rate $e^{\lambda t}$ inside the linearized regime when the maximal Lyapunov exponent $\lambda > 0$ is positive. Determinism never guarantees predictability, and continuity never guarantees robustness.

## 4. Non-Ergodicity and the Itô Variance Drain

A pension fund board sets its annual payout equal to an index's average yearly return, reassured by decades of positive returns. Within ten rocky years the fund goes broke, even while the index's average return still looks healthy. The board mistook the average across many parallel universes for the compounded trajectory along the single timeline the fund had to survive.

Check that trap with a coin-flip game you can verify in two lines of arithmetic. Start with $\$1$ (unit wealth $1$). On each round, a fair coin either multiplies your wealth by $1.5$ ($+50\%$) or by $0.6$ ($-40\%$).

- **Ensemble average (across parallel worlds):** half of the worlds reach $1.5$ and half reach $0.6$, giving an average single-round multiplier of $(1.5 + 0.6)/2 = 1.05$, a $+5\%$ gain per round.
- **Single-path time average (along one timeline):** after one win and one loss in either order, your wealth is $1.5 \times 0.6 = 0.90$, a $-10\%$ cumulative drop (geometric factor $\sqrt{0.90} \approx 0.949$ per round).

As the rounds repeat, almost every individual player drifts toward zero even while the cross-world average climbs exponentially. That divergence is the core failure of **ergodicity**. A jointly measurable stationary process $(X\_t)\_{t \ge 0}$ with finite expectation ($\mathbb{E}[|X\_0|] < \infty$) satisfies **pointwise (Birkhoff) ergodicity** when one timeline's long-run time average equals the cross-world expectation almost surely (whereas $L^2$ mean-square ergodicity additionally requires finite variance $\mathrm{Var}(X\_0) < \infty$):

$$\lim\_{T \to \infty} \frac{1}{T} \int\_0^T X\_t\mkern3mudt = \mathbb{E}[X\_0] \quad \text{almost surely}.$$

Here $X\_t$ is the state at time $t$, $T$ is the averaging horizon, and $\mathbb{E}[X\_0]$ is the stationary expectation across all worlds.

Multiplicative growth breaks ergodicity systematically because wealth itself becomes non-stationary. Model continuous compounding via Geometric Brownian Motion with drift rate $\mu \in \mathbb{R}$, positive volatility $\sigma > 0$, standard Brownian motion $W\_t$, and positive starting stake $X\_0 > 0$ (deferring sudden jump shocks to §5):

$$dX\_t = \mu X\_t\mkern3mudt + \sigma X\_t\mkern3mudW\_t, \qquad X\_0 > 0.$$

Read that aloud: each tiny instant $dt$ nudges wealth $X\_t$ proportionally by steady drift $\mu\,dt$ plus random noise $\sigma\,dW\_t$. Applying Itô's lemma (Itô 1951) gives the exact position along a single trajectory at time $t$:

$$X\_t = X\_0 \exp\left( \left(\mu - \frac{\sigma^2}{2}\right) t + \sigma W\_t \right).$$

Average $X\_t$ across all possible worlds and the random noise cancels in expectation, leaving the **ensemble average**:

$$\mathbb{E}[X\_t] = X\_0 e^{\mu t}.$$

Because $X\_t$ is **non-stationary** (its ensemble expectation $\mathbb{E}[X\_t] = X\_0 e^{\mu t}$ grows or shrinks with $t$), $\frac{1}{T}\int\_0^T X\_t\,dt$ never converges to $\mathbb{E}[X\_t]$—instead, whenever $0 < \mu < \frac{1}{2}\sigma^2$, individual paths collapse to $X\_t \to 0$ (and $\frac{1}{T}\int\_0^T X\_t\,dt \to 0$) almost surely even while $\mathbb{E}[X\_t] \to +\infty$. Its **log-return increments** $d\ln X\_t$, by contrast, are stationary: since the strong law for Brownian increments guarantees $W\_t / t \to 0$ almost surely as $t \to \infty$, almost every individual path compounds at the lower log-growth rate $g$ (distinct from §3's fixed-point map $g$) rather than the ensemble-mean growth rate $\mu$:

$$g = \lim\_{t \to \infty} \frac{1}{t} \ln\\!\left(\frac{X\_t}{X\_0}\right) = \mu - \frac{\sigma^2}{2} \quad \text{almost surely}.$$

Read that formula aloud: your lived growth rate $g$ equals the ensemble drift $\mu$ **minus half the variance rate $\frac{1}{2}\sigma^2$**. That penalty $-\frac{1}{2}\sigma^2$ is the **Itô–Kelly variance drain**. Kiyosi Itô (1951) derived it in continuous time, John Kelly (1956) and Robert Merton (1969) built log-optimal betting and portfolio rules around it, and Ole Peters (2019) placed it at the center of non-ergodic critique.

Notice the trap when $0 < \mu < \frac{1}{2}\sigma^2$: the ensemble expectation $\mathbb{E}[X\_t]$ rockets toward $+\infty$ while nearly every single trajectory decays toward zero. Rare jackpot paths lift the average across worlds while typical solo paths wipe out. Still, diagnosing variance drain does not make log-growth a universal rule of choice: finite horizons, risk aversion, and correlated returns can pull rational decisions away from asymptotic Kelly betting (Samuelson 1971; Ford & Kay 2023, 2024), explored in [Time and Dependence](38-time-and-dependence.md).

In the path growth formula $\mu\_{\mathrm{time}} = g = \mu - \frac{1}{2}\sigma^2$, drift $\mu$ ($\text{day}^{-1}$) and variance rate $\sigma^2 \ge 0$ ($\text{day}^{-1}$, with $[\sigma] = \text{day}^{-1/2}$) partition $\mu > 0$ into three regimes (when $\mu \le 0 < \sigma$, $\mu\_{\mathrm{time}} < 0$ always):

1. **Negligible volatility ($\sigma^2/2 \ll \mu$, $\sigma \to 0^+$):** $\mu\_{\mathrm{time}}$ approaches $\mu$, so ensemble drift safely tracks path growth.
2. **Zero-growth boundary ($\sigma = \sqrt{2\mu}$, $\mu > 0$):** volatility completely eats drift ($\mu\_{\mathrm{time}} = 0$), leaving $\ln(X\_t/X\_0) = \sigma W\_t$ an un-drifted random walk that still hits any lower threshold $b \in (0, X\_0)$ almost surely.
3. **Decay drain ($\sigma > \sqrt{2\mu}$, $\mu > 0$):** $\mu\_{\mathrm{time}} < 0$ while $\mathbb{E}[X\_t] \to +\infty$; doubling volatility $\sigma$ quadruples the additive drain $\frac{1}{2}\sigma^2$. When $\mu$ itself must be estimated from small or clustered data, finite-sample Kelly shrinkage ($k$, distinct from §1's index $k$ and §3's exponent $k$; Baker & McHale 2013) scales bets back inside [Time and Dependence](38-time-and-dependence.md) and [Chapter 47](47-irreversible-time-vital-margin.md).

## 5. Consequences for Sequential Decision-Making

Institutions live along one timeline and cannot average losses across parallel worlds. Whenever $0 < \mu < \frac{1}{2}\sigma^2$, ignoring variance drain drives unpooled trajectories toward ruin while ensemble projections promise abundance. Continuously rebalanced equal-weighted pooling across $N$ independent units shrinks the drain only to $\sigma^2/(2N)$ (whereas static buy-and-hold equal-weighted pooling $\frac{1}{N}\sum\_{i=1}^N X\_t^{(i)}$ retains asymptotic log-growth rate $\mu - \frac{1}{2}\sigma^2$), leaving every finite pool exposed.

### When the ensemble average fails to return

Pooling fails whenever continuous rebalancing or independence breaks down. Heavy-tailed Paretian jumps ($\alpha < 2$) generate càdlàg Lévy trajectories—continuous at almost every fixed instant $t$, with countably many jumps—that break continuous Brownian diffusion and leap across stop-loss orders (Mandelbrot 1963), crisis correlations erase diversification, and operational delays stall rebalancing. Finite-horizon choice under non-linear utility, leverage, and spillover onto external reception perimeters—outside communities and habitats—belongs to [Time and Dependence](38-time-and-dependence.md) and [Chapter 47](47-irreversible-time-vital-margin.md) (Samuelson 1971; Ford & Kay 2023, 2024; Itô 1951; Kelly 1956; Merton 1969).

## Bridge

[Distribution and Moment](31-distribution-and-moment.md), [Time and Dependence](38-time-and-dependence.md), and [Chapter 47](47-irreversible-time-vital-margin.md) carry these convergence limits and the path–ensemble gap $g = \mu - \frac{1}{2}\sigma^2$ into tail diagnostics and vital survival clocks. [Derivatives and Convexity](12-derivatives-and-convexity.md) next turns local limits into slopes and curvature, exposing why curving-upward costs systematically punish planning on averages.
