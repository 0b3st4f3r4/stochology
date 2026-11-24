# Derivatives and Convexity

<figure class="stoch-figure stoch-logo-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="800" height="132" viewBox="0 0 800 132" fill="none" role="img" aria-label="Stochology CH12 · CURVATURE — Derivatives, Subgradients, And Convexity">
  <defs>
    <!-- Canonical Ring Gradient -->
    <linearGradient id="ch12-ring" x1="2" y1="78" x2="386" y2="78" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="38%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="68%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <!-- 1. Atmospheric Cosmic Field -->
    <radialGradient id="ch12-glow" cx="52%" cy="52%" r="50%">
      <stop offset="0%" stop-color="#fbbf24" stop-opacity="0.24"/>
      <stop offset="48%" stop-color="#38bdf8" stop-opacity="0.14"/>
      <stop offset="80%" stop-color="#e11d48" stop-opacity="0.08"/>
      <stop offset="100%" stop-color="#090e1a" stop-opacity="0"/>
    </radialGradient>
    <!-- 3D Spherical Node Gradients -->
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
    <!-- Stókhos Reticle Gradients -->
    <radialGradient id="ch12-pupil" cx="45%" cy="42%" r="58%">
      <stop offset="0%" stop-color="#020617"/>
      <stop offset="68%" stop-color="#0f172a"/>
      <stop offset="100%" stop-color="#451a03"/>
    </radialGradient>
    <linearGradient id="ch12-gold-mira" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#fef08a"/>
      <stop offset="45%" stop-color="#fbbf24"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <radialGradient id="ch12-gold-core" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#ffffff"/>
      <stop offset="35%" stop-color="#fde047"/>
      <stop offset="100%" stop-color="#d97706"/>
    </radialGradient>
    <!-- Ink curve gradient -->
    <linearGradient id="ch12-ink-curve" x1="0%" y1="0%" x2="100%" y2="0%">
      <stop offset="0%" stop-color="var(--logo-ink, #0f172a)"/>
      <stop offset="50%" stop-color="var(--logo-sub, #1d4ed8)"/>
      <stop offset="100%" stop-color="var(--logo-ink, #0f172a)"/>
    </linearGradient>
    <linearGradient id="ch12-basin-fill" x1="22" y1="88" x2="106" y2="36" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#10b981" stop-opacity="0.28"/>
      <stop offset="50%" stop-color="#2563eb" stop-opacity="0.30"/>
      <stop offset="100%" stop-color="#e11d48" stop-opacity="0.20"/>
    </linearGradient>
    <linearGradient id="ch12-tangent-grad" x1="24" y1="96" x2="98" y2="48" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#fef08a"/>
      <stop offset="50%" stop-color="#f59e0b"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <linearGradient id="ch12-normal-grad" x1="58" y1="74" x2="44" y2="50" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#fda4af"/>
      <stop offset="60%" stop-color="#e11d48"/>
      <stop offset="100%" stop-color="#881337"/>
    </linearGradient>
  </defs>
  <g>
    <circle cx="64" cy="66" r="48" fill="url(#ch12-glow)"/>
    <path d="M 22 88 Q 64 102 106 36 L 106 96 L 22 96 Z" fill="url(#ch12-basin-fill)"/>
    <path d="M 22 88 Q 64 102 106 36" stroke="url(#ch12-ink-curve)" stroke-width="3.4" stroke-linecap="round"/>
    <line x1="24" y1="96" x2="98" y2="48" stroke="url(#ch12-tangent-grad)" stroke-width="2.8" stroke-dasharray="6 3"/>
    <line x1="58" y1="74" x2="45.01" y2="51.73" stroke="url(#ch12-normal-grad)" stroke-width="3.2" stroke-linecap="round"/>
    <path d="M 50.38 53.95 L 44 50 L 44.30 57.49" stroke="url(#ch12-normal-grad)" stroke-width="2.6" stroke-linecap="round" stroke-linejoin="round"/>
    <circle cx="28" cy="90" r="8.5" fill="url(#ch12-node-well)" stroke="url(#ch12-ring-vital)" stroke-width="3.0"/>
    <circle cx="28" cy="90" r="3.8" fill="url(#ch12-core-vital)"/>
    <circle cx="96" cy="50" r="8.5" fill="url(#ch12-node-well)" stroke="url(#ch12-ring-epi)" stroke-width="3.0"/>
    <circle cx="96" cy="50" r="3.8" fill="url(#ch12-core-epi)"/>
    <g>
      <circle cx="58" cy="74" r="7.4" fill="var(--logo-node-bg, #e2e8f0)"/>
      <circle cx="58" cy="74" r="6.4" fill="url(#ch12-pupil)" stroke="url(#ch12-gold-mira)" stroke-width="2.4"/>
      <line x1="48.40" y1="74.00" x2="53.80" y2="74.00" stroke="url(#ch12-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="62.20" y1="74.00" x2="67.60" y2="74.00" stroke="url(#ch12-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="58.00" y1="64.40" x2="58.00" y2="69.80" stroke="url(#ch12-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="58.00" y1="78.20" x2="58.00" y2="83.60" stroke="url(#ch12-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <circle cx="58" cy="74" r="2.2" fill="url(#ch12-gold-core)"/>
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

> Rate of change, second-order curvature, and the Jensen geometry separating average risk from mean exposure.

## By the end of this chapter you can

1. Read a derivative as local sensitivity and a second derivative as curvature, and say which a decision
   needs.
2. Identify convexity or concavity from chords, supporting lines, or second derivatives, and say what that implies for an average.
3. Compute the convexity gap for a simple case and say why it is never negative for any convex function.

## 1. The Derivative as Local Sensitivity

A hospital network trims reserve beds because a linear fit around calm mid-week traffic shows a gentle slope: each extra patient adds only a fraction of a second of wait time. That tangent slope describes tiny nudges near today's operating point. It misses the vertical wall waiting when a winter flu wave pushes load toward full capacity. Mistaking a local first derivative for a global risk profile invites unbuffered collapse.

Start with slope intuition on a curve you can check by hand: $f(x) = x^2$. How steep is $f$ right at $x = 2$? Compute the average slope between $x = 2$ and a point $h$ units away, $x = 2 + h$:

$$\frac{f(2 + h) - f(2)}{h} = \frac{(2 + h)^2 - 4}{h} = \frac{4h + h^2}{h} = 4 + h.$$

Try shrinking the step $h$: at $h = 1$ the chord slope is $5$; at $h = 0.1$ it is $4.1$; as $h$ vanishes toward $0$, the chord settles onto the exact tangent slope $4$.

In general, for any real function $f$ at point $x$ and step $h \ne 0$, the **derivative** $f'(x)$ is the limit of that difference quotient as $h$ tends to $0$:

$$f'(x) = \lim\_{h \to 0} \frac{f(x + h) - f(x)}{h}.$$

Read that formula aloud: divide the output change $f(x + h) - f(x)$ by the tiny input nudge $h$, and watch what ratio remains as the nudge shrinks to zero. Geometrically, $f'(x)$ is the slope of the tangent line; operationally, it measures local marginal sensitivity.

A zero slope ($f'(x^\ast) = 0$) marks a flat point: either a valley, a peak, or a momentary pause. When $f$ is smooth and bowl-shaped (convex), $f'(x^\ast) = 0$ locates a global minimum, yet it still says nothing about stability against shocks. In a dynamical system $\dot{x} = F(x)$, at an equilibrium $F(x^\ast) = 0$, a negative slope $F'(x^\ast) < 0$ guarantees only local linear asymptotic stability inside an arbitrarily small neighborhood ([Statistical Physics and Time](23-statistical-physics-and-time.md)); it cannot detect narrow dynamical attraction-basin width or sudden jumps across a cliff ([Chapter 47](47-irreversible-time-vital-margin.md)).

## 2. The Second Derivative and Curvature

A trauma ward averages $A$ arrivals per day: quiet Mondays fall below $A$ by the same count that stormy Tuesdays rise above it. The two-day average stays exactly $A$, yet the swings never cancel out. Quiet Mondays save a few minutes of paperwork while stormy Tuesdays force ambulance diversions and corridor crowding. Whenever equal-and-opposite swings fail to cancel, the response bends: it has non-zero curvature.

To measure how fast the slope $f'(x)$ itself is changing at point $x$ over step $h$, differentiate a second time to get the **second derivative** $f''(x)$:

$$f''(x) = \lim\_{h \to 0} \frac{f'(x + h) - f'(x)}{h}.$$

Read that aloud: the second derivative is the rate of change of the slope. For our parabola $f(x) = x^2$, the slope is $f'(x) = 2x$, which increases by $2$ every time $x$ moves right by $1$, so $f''(x) = 2 > 0$.

Across an open interval $I$ where $f \in C^2(I)$ is twice continuously differentiable, the sign of $f''$ classifies the bend:

- **$f''(x) \ge 0$ everywhere on $I$:** the curve bends upward like a bowl (**convex**); strict inequality $f'' > 0$ is sufficient, though not necessary (witness $f(x) = x^4$ at $0$), for strict convexity.
- **$f''(x) \le 0$ everywhere on $I$:** the curve arches downward like a dome (**concave**); strict inequality $f'' < 0$ ensures strict concavity.
- **$f''(x) = 0$ identically across $I$:** $f$ is a flat straight line (affine). Beware single points: $f(x) = x^3$ has $f''(0) = 0$ at an inflection where curvature flips sign, whereas $f(x) = x^4$ has $f''(0) = 12(0)^2 = 0$ yet still holds a strict bowl minimum at $0$.

Important operational rules bend sharply at corners without possessing second derivatives everywhere. Piecewise-linear shortfalls $\phi(x) = (x - c)\_+ = \max(0, x - c)$ ([Chapter 46](46-non-identical-asymmetry-heavy-tails.md)) are flat ($\phi'' = 0$) away from threshold $c$ yet convex overall. Conversely, pointwise minima such as the best loss $\min\_a \mathcal{L}(a, s)$ at state $s$ in decision analysis ([Chapter 36](36-decision-loss-threshold.md)), and Bayes risk $\min\_a \mathbb{E}\_\pi[\mathcal{L}(a, S)]$ in belief weights $\pi$, or the weakest-link minimum $\psi(\mathbf{x}) = \min\_j x\_j$ across non-substitutable safeguards ([Chapter 41](41-the-triad-and-the-minimum.md)) are concave as functions of the loss vector, belief weights, or safeguard vector without being smooth. Understanding convexity therefore rests directly on chords and supporting straight lines.

## 3. Jensen's Inequality and the Taylor Expansion of the Jensen Gap

Check what curvature does to an average using simple arithmetic on $\phi(x) = x^2$. Suppose a random daily load $X$ equals $1$ half the time and $3$ half the time. Its average load is $\mathbb{E}[X] = (1 + 3)/2 = 2$, and the loss evaluated at that average is $\phi(\mathbb{E}[X]) = 2^2 = 4$. Now average the *actual* losses across both days: $\mathbb{E}[\phi(X)] = (1^2 + 3^2)/2 = (1 + 9)/2 = 5$.

The average loss ($5$) exceeds the loss at the average ($4$) by a gap of $\Delta\_J = 5 - 4 = 1$. Geometrically, the straight chord connecting $(1, 1)$ and $(3, 9)$ sits at height $5$ above $x = 2$, strictly above the bowl curve $\phi(2) = 4$. That vertical separation is why planning for the average systematically underfunds fluctuating risks.

Formally, a function $\phi : I \to \mathbb{R}$ on an interval $I \subseteq \mathbb{R}$ is **convex** if, for every pair of points $x, y \in I$ and every weight $w \in [0, 1]$, the chord sits on or above the curve:

$$\phi\bigl(w x + (1 - w) y\bigr) \le w \phi(x) + (1 - w) \phi(y).$$

Read that aloud: evaluating $\phi$ at a weighted blend $wx + (1-w)y$ never exceeds the same weighted blend of $\phi(x)$ and $\phi(y)$. At any interior point $x\_0 \in \mathrm{int}(I)$ of a convex function, the left and right slopes $\phi'\_-(x\_0) \le \phi'\_+(x\_0)$ are finite. Any slope $g$ between them (written $g \in \partial\phi(x\_0) = [\phi'\_-(x\_0), \phi'\_+(x\_0)]$, the **subdifferential**) gives a **subgradient** supporting line touching the curve at $x\_0$ and staying on or below it everywhere across $I$:

$$\phi(x) \ge \phi(x\_0) + g\mkern3mu (x - x\_0) \qquad \forall x \in I.$$

Where $\phi$ is smooth, $g$ is simply the derivative $\phi'(x\_0)$. At a sharp kink like $x\_0 = c$ on $(x - c)\_+$, $g$ can be any slope in $[0, 1]$. For concave functions such as $\min\_j x\_j$, the supporting line sits *above* the curve ($\le$).

Now let $\phi : I \to \mathbb{R}$ be convex and let $X$ be an integrable random variable ($\mathbb{E}[|X|] < \infty$) taking values in $I$ with mean $\mu = \mathbb{E}[X]$. If $X = \mu$ almost surely, equality is immediate. Otherwise $\mu$ lies strictly inside $I$, so picking a supporting slope $g \in \partial\phi(\mu)$ yields $\phi(X) \ge \phi(\mu) + g\mkern3mu (X - \mu)$ almost surely. Because $\mathbb{E}[|X|] < \infty$, the negative part of $\phi(X)$ stays integrable and $\mathbb{E}[\phi(X)] \in (-\infty, +\infty]$ is well-defined. Taking expectations kills the linear term $g\mkern3mu \mathbb{E}[X - \mu] = 0$, proving Johan Jensen's inequality (Jensen 1906), where $\Delta\_J(\phi, X)$ denotes the **Jensen gap**:

$$\mathbb{E}\bigl[\phi(X)\bigr] \ge \phi\bigl(\mathbb{E}[X]\bigr), \qquad \Delta\_J(\phi, X) := \mathbb{E}\bigl[\phi(X)\bigr] - \phi\bigl(\mathbb{E}[X]\bigr) \ge 0.$$

Read that aloud: for any convex cost $\phi$, expected cost $\mathbb{E}[\phi(X)]$ is never smaller than the cost at the expected input $\phi(\mathbb{E}[X])$, making the Jensen gap $\Delta\_J$ non-negative. Notice that our supporting-line proof never used a second derivative, so it holds just as cleanly at sharp kinks like $(x - c)\_+$. Extending the same supporting-hyperplane argument to the concave multivariate minimum $\min\_m x\_m$ (where $\min\_m \hat{\theta}\_m \le \hat{\theta}\_k$ for every integrable coordinate $k$) proves downward finite-sample bias in plug-in estimates: $\mathbb{E}[\min\_m \hat{\theta}\_m] \le \min\_m \mathbb{E}[\hat{\theta}\_m]$ ([Chapter 33](33-measuring-and-erring.md); [Appendix A](appendix-a-mathematics.md)). Keep finiteness in mind: if $\mathbb{E}[\phi(X)] = +\infty$ (say $\phi(x) = x^2$ under a Pareto tail $1 < \alpha \le 2$), $X$ has a finite mean yet $\Delta\_J = +\infty$.

When $\phi$ has two smooth derivatives near $\mu$ and $X$ has finite variance $\sigma^2 = \mathrm{Var}(X) < \infty$, expanding $\phi(X) \approx \phi(\mu) + \phi'(\mu)(X - \mu) + \frac{1}{2}\phi''(\mu)(X - \mu)^2$ and taking expectations ($\mathbb{E}[X - \mu] = 0$, $\mathbb{E}[(X - \mu)^2] = \sigma^2$) estimates the gap:

$$\Delta\_J(\phi, X) \approx \frac{1}{2} \phi''(\mu) \sigma^2.$$

Test that formula on our opening toy example $\phi(x) = x^2$ with $X \in \lbrace 1, 3 \rbrace$ equiprobably: $\mu = 2$, $\sigma^2 = ((1-2)^2 + (3-2)^2)/2 = 1$, and $\phi''(2) = 2$, giving $\frac{1}{2}(2)(1) = 1$, matching $\Delta\_J = 5 - 4 = 1$ exactly.

More broadly, for convex $\phi : I \to \mathbb{R}$ and $X \in I$ with both $\mathbb{E}[|X|]$ and $\mathbb{E}[|\phi(X)|]$ finite, $\Delta\_J(\phi, X) = 0$ holds **if and only if** either $X = \mu$ almost surely (covering boundary atoms such as $x = 0$ on $-\sqrt{x}$ where $\partial\phi(0) = \emptyset$), or $X$ is non-degenerate (forcing $\mu \in \mathrm{int}(I)$) and $\phi(X) = \phi(\mu) + g\mkern3mu (X - \mu)$ holds almost surely under $X$ for some supporting slope $g \in \partial\phi(\mu)$; equivalently, $\phi$ is affine across $\mathrm{int}(\mathrm{conv}(\mathrm{supp}(X)))$ and assigns zero $X$-probability to any upward jump at a boundary endpoint (which reduces to $\phi$ being affine across all of $\mathrm{conv}(\mathrm{supp}(X))$ whenever $\phi$ is continuous, and forces $X = \mu$ a.s. when $\phi$ is strictly convex). Watch for edge traps in the quadratic formula: $\frac{1}{2}\phi''(\mu)\sigma^2$ equals zero at $\mu = 0$ for $\phi(x) = x^4$ even though $\Delta\_J > 0$, and it stays finite for $\phi(x) = x^3$ under Pareto $\alpha = 2.5$ even though $\Delta\_J = +\infty$.

Four canonical shapes summarize how curvature and support control $\Delta\_J$:

1. **Convex quadratic and power-law losses:** For $\phi(x) = x^2$ ($\phi'' \equiv 2$), $\Delta\_J = \mathrm{Var}(X) > 0$ whenever $0 < \sigma^2 < \infty$. For $\phi(x) = c x^\beta$ ($c > 0$, $x \ge 0$, $\beta > 1$) and non-degenerate $X \ge 0$ under a power-law tail $\mathbb{P}(X > x) = L(x)x^{-\alpha}$ with finite mean $\mathbb{E}[X] < \infty$ ($\alpha > 1$, or $\alpha = 1$ with $\int^\infty x^{-1} L(x)\,dx < \infty$), Jensen yields $\Delta\_J > 0$: finite if $\mathbb{E}[X^\beta] < \infty$ ($\alpha > \beta$, or $\alpha = \beta$ with $\int^\infty x^{-1} L(x)\,dx < \infty$), and $+\infty$ for $1 \le \alpha < \beta$ (or at $\alpha = \beta$ when $\int^\infty x^{-1} L(x)\,dx = \infty$). When $0 < \alpha < 1$ (or $\alpha = 1$ with $\int^\infty x^{-1} L(x)\,dx = \infty$), both $\mathbb{E}[X]$ and $\mathbb{E}[X^\beta]$ hit $+\infty$, leaving $\Delta\_J = \infty - \infty$ undefined.
2. **Piecewise-linear shortfall ($\phi(x) = (x - c)\_+$, $\partial\phi(c) = [0, 1]$):** Here $\phi'' = 0$ off the kink $c$, yet $\Delta\_J > 0$ whenever $X$ places probability on both $(-\infty, c)$ and $(c, \infty)$; conversely, $\Delta\_J = 0$ if $X$ stays entirely on one branch even with $\sigma^2 > 0$.
3. **Concave utility ($\phi(x) = \sqrt{x}$ on $x \ge 0$, $\phi'' = -\frac{1}{4}x^{-3/2} < 0$ for $x > 0$):** Reversing the inequality yields $\mathbb{E}[\sqrt{X}] < \sqrt{\mathbb{E}[X]}$ ($\Delta\_J < 0$) for non-degenerate integrable $X \ge 0$.
4. **Affine tariff ($\phi(x) = a + bx$, $\phi'' \equiv 0$):** Provided $\mathbb{E}[|X|] < \infty$ ($\alpha > 1$, or $\alpha = 1$ with $\int^\infty x^{-1} L(x)\,dx < \infty$; for $\alpha < 1$, or $\alpha = 1$ with $\int^\infty x^{-1} L(x)\,dx = \infty$, $\mathbb{E}[X]$ diverges or is undefined), $\Delta\_J = 0$. For any convex loss $\phi : I \to \mathbb{R}$ with $X \in I$ and $\mathbb{E}[|X|] < \infty$, planning at the mean strictly underfunds expected harm ($\Delta\_J > 0$, finite or $+\infty$) **if and only if** $X$ is non-degenerate and $\phi(X)$ fails to match any supporting affine line $\phi(\mu) + g\mkern3mu (X - \mu)$ ($g \in \partial\phi(\mu)$) almost surely under $X$; equivalently, $\phi$ is non-affine on $\mathrm{int}(\mathrm{conv}(\mathrm{supp}(X)))$ or places positive $X$-probability on an upward jump at an endpoint of $\mathrm{conv}(\mathrm{supp}(X))$. That condition covers both smooth bowls ($\phi'' > 0$) and threshold kinks like $(x - c)\_+$.

## 4. The Explosive Curvature of Queues and Ruin

An emergency-room director pushes average bed utilization toward $100\%$ to eliminate "idle slack" and is shocked when waiting times explode. Evaluating convex congestion at average load understates expected delay by $\Delta\_J$. For stationary $GI/G/1$ queues with independent, identically distributed renewal arrivals and service times having finite variances ($\sigma\_a^2, \sigma\_s^2 < \infty$) and non-zero jitter ($\sigma\_a^2 + \sigma\_s^2 > 0$, since a deterministic $D/D/1$ queue with $\sigma\_a^2 = \sigma\_s^2 = 0$ has zero wait for $\rho < 1$; broken whenever infinite-variance heavy-tailed service with $\alpha < 2$, or $\alpha = 2$ and $\int^\infty x^{-1}L(x)\,dx = \infty$, sends $\sigma\_s^2 = \infty$), John Kingman's (1961, 1962) heavy-traffic bound scales waiting time through the strictly convex hyperbola $\phi(\rho)$ on utilization load $\rho \in [0, 1)$ ([Chapter 38](38-time-and-dependence.md)):

$$\phi(\rho) = \frac{\rho}{1 - \rho}, \qquad \rho \in [0, 1), \qquad \phi'(\rho) = \frac{1}{(1 - \rho)^2}, \qquad \phi''(\rho) = \frac{2}{(1 - \rho)^3}.$$

As the load sweep in [Chapter 03](03-the-biggest-number-in-the-room.md) illustrated from half capacity toward saturation, shrinking slack $s := 1 - \rho \in (0, 1)$ tenfold toward $\rho \to 1^-$ multiplies delay $\phi$ roughly tenfold while exploding curvature $\phi''$ by a factor of $1{,}000$, and for $\rho \in (0, 1)$, load elasticity $d\ln\phi / d\ln\rho = (1 - \rho)^{-1} = 1 + \phi(\rho)$ crosses $2$ right at $\rho = 1/2$. How this queue curvature interacts with arrival–service variability, deliberation time, ruin bounds, and soft-budget spillover onto external reception perimeters, outside communities and habitats (Kornai 1986), is developed in [Chapter 26](26-regulation-and-homeostasis.md), [Chapter 38](38-time-and-dependence.md), and [Chapter 47](47-irreversible-time-vital-margin.md).

## 5. What Curvature Decides

Three clear rules govern judgment whenever outcomes bend with scale. First, for **globally convex** costs on $I$ ($\Delta\_J \ge 0$, smooth or kinked), planning on the mean understates expected damage whenever $X$ is non-degenerate and $\phi$ fails to match a supporting affine line almost surely under $X$. Second, for **globally concave** benefits ($\Delta\_J \le 0$), planning on the mean overstates expected return, reversing cleanly for concave losses or convex gains. Third, assuming straight-line scaling ($\phi'' \equiv 0$) merely conceals this convexity tax. Remember that the sign of $\Delta\_J$ requires *global* convexity or concavity across $I$, never just $\phi''(\mu) > 0$ at one interior point: an $S$-shaped function can bend upward at $\mu$ while still yielding $\Delta\_J < 0$.

### When the second order is not enough

The quadratic estimate $\frac{1}{2}\phi''(\mu)\sigma^2$ is exact for quadratic $\phi$ ($\phi''' \equiv 0$) whenever $\sigma^2 < \infty$, even if $\mathbb{E}[|X|^3] = \infty$ ($2 < \alpha \le 3$), yet breaks across sharp kinks, outside Taylor convergence, or when higher moments dominate. Suppose $\sigma > 0$ and $\mathbb{E}[|X|^3] < \infty$ ([Chapter 31](31-distribution-and-moment.md)), so skewness $\gamma\_1 = \mathbb{E}[(X-\mu)^3]/\sigma^3$ exists; whenever both $\phi''(\mu) \ne 0$ and $\phi'''(\mu) \ne 0$, the cubic term $\frac{1}{6}\phi'''(\mu)\gamma\_1\sigma^3$ overtakes the quadratic term once:

$$|\gamma\_1|\mkern2mu\sigma > \frac{3\mkern2mu|\phi''(\mu)|}{|\phi'''(\mu)|}.$$

Read that threshold aloud: the quadratic estimate breaks whenever skewness times spread exceeds three times the ratio of second derivative to third derivative (and when $\gamma\_1 = 0$ with $\mathbb{E}[X^4] < \infty$, the fourth-order term $\frac{1}{24}|\phi^{(4)}(\mu)|\mathbb{E}[(X-\mu)^4]$ takes over). When variance diverges ($\sigma^2 = \infty$, i.e., $\alpha < 2$, or $\alpha = 2$ with $\int^\infty x^{-1}L(x)\,dx = \infty$), quadratic approximations collapse entirely: compare dispersions or compute $\mathbb{E}[\phi(X)]$ directly if finite, and recognize when power-law harm ($\beta > \alpha$, or $\beta = \alpha$ with $\int^\infty x^{-1}L(x)\,dx = \infty$) leaves no finite expectation at all ([Chapter 31](31-distribution-and-moment.md); [Chapter 46](46-non-identical-asymmetry-heavy-tails.md)).

Risk theory and living regulation share this exact curvature logic. *Coherent* risk measures (Artzner et al. 1999) combine subadditivity with positive homogeneity, rewarding diversification unlike Value-at-Risk; the same paper lists convexity of the acceptance set (Axiom 2.3) separately from the positively homogeneous cone (Axiom 2.4, called *"a less natural requirement"*) and handles illiquidity by folding liquidation effects into future net worth (Remark 1). Likewise, Georges Canguilhem (1943/1966) argued that living health amidst environmental inconstancy (*infidélité du milieu*) demands generous physiological slack rather than a fragile setpoint, the intuition behind the vital margin and concave weakest-link minimum of [Chapter 41](41-the-triad-and-the-minimum.md).

## Bridge

[Distribution and Moment](31-distribution-and-moment.md), [Time and Dependence](38-time-and-dependence.md), and [Chapter 47](47-irreversible-time-vital-margin.md) carry Jensen's gap $\Delta\_J$ and queue curvature $\phi(\rho)$ into heavy-tail regimes and survival clocks alongside [The Triad and the Minimum](41-the-triad-and-the-minimum.md). [Integrals and Accumulation](13-integrals-and-accumulation.md) next reverses the derivative into accumulated stock drawdowns, weighted expectations, and coherent shortfall over extreme tails.
