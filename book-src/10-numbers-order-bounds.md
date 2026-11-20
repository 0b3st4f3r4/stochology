# Numbers, Order, and Bounds

<figure class="stoch-figure stoch-logo-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="800" height="132" viewBox="0 0 800 132" fill="none" role="img" aria-label="Stochology CH10 · BOUNDS — Order, Supremum, And Infimum">
  <defs>
    <!-- Canonical Ring Gradient -->
    <linearGradient id="ch10-ring" x1="2" y1="78" x2="386" y2="78" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="38%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="68%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <!-- 1. Atmospheric Cosmic Field -->
    <radialGradient id="ch10-glow" cx="52%" cy="52%" r="50%">
      <stop offset="0%" stop-color="#fbbf24" stop-opacity="0.24"/>
      <stop offset="48%" stop-color="#38bdf8" stop-opacity="0.14"/>
      <stop offset="80%" stop-color="#e11d48" stop-opacity="0.08"/>
      <stop offset="100%" stop-color="#090e1a" stop-opacity="0"/>
    </radialGradient>
    <!-- 3D Spherical Node Gradients -->
    <radialGradient id="ch10-node-well" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="var(--logo-well-hi, var(--elevated-bg, #ffffff))"/>
      <stop offset="100%" stop-color="var(--logo-node-bg, #e2e8f0)"/>
    </radialGradient>
    <linearGradient id="ch10-ring-epi" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#7dd3fc"/>
      <stop offset="50%" stop-color="#2563eb"/>
      <stop offset="100%" stop-color="#1e3a8a"/>
    </linearGradient>
    <radialGradient id="ch10-core-epi" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#bae6fd"/>
      <stop offset="45%" stop-color="#3b82f6"/>
      <stop offset="100%" stop-color="#1d4ed8"/>
    </radialGradient>
    <linearGradient id="ch10-ring-vital" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#6ee7b7"/>
      <stop offset="50%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#065f46"/>
    </linearGradient>
    <radialGradient id="ch10-core-vital" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#a7f3d0"/>
      <stop offset="45%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#047857"/>
    </radialGradient>
    <linearGradient id="ch10-ring-gov" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#fda4af"/>
      <stop offset="50%" stop-color="#e11d48"/>
      <stop offset="100%" stop-color="#881337"/>
    </linearGradient>
    <radialGradient id="ch10-core-gov" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#fecdd3"/>
      <stop offset="45%" stop-color="#f43f5e"/>
      <stop offset="100%" stop-color="#be123c"/>
    </radialGradient>
    <!-- Stókhos Reticle Gradients -->
    <radialGradient id="ch10-pupil" cx="45%" cy="42%" r="58%">
      <stop offset="0%" stop-color="#020617"/>
      <stop offset="68%" stop-color="#0f172a"/>
      <stop offset="100%" stop-color="#451a03"/>
    </radialGradient>
    <linearGradient id="ch10-gold-mira" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#fef08a"/>
      <stop offset="45%" stop-color="#fbbf24"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <radialGradient id="ch10-gold-core" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#ffffff"/>
      <stop offset="35%" stop-color="#fde047"/>
      <stop offset="100%" stop-color="#d97706"/>
    </radialGradient>
    <!-- Ink curve gradient -->
    <linearGradient id="ch10-ink-curve" x1="0%" y1="0%" x2="100%" y2="0%">
      <stop offset="0%" stop-color="var(--logo-ink, #0f172a)"/>
      <stop offset="50%" stop-color="var(--logo-sub, #1d4ed8)"/>
      <stop offset="100%" stop-color="var(--logo-ink, #0f172a)"/>
    </linearGradient>
    <linearGradient id="ch10-lattice-fill" x1="64" y1="26" x2="64" y2="106" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#e11d48" stop-opacity="0.25"/>
      <stop offset="50%" stop-color="#10b981" stop-opacity="0.28"/>
      <stop offset="100%" stop-color="#2563eb" stop-opacity="0.25"/>
    </linearGradient>
    <linearGradient id="ch10-bound-sup" x1="24" y1="26" x2="104" y2="26" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#fda4af"/>
      <stop offset="50%" stop-color="#e11d48"/>
      <stop offset="100%" stop-color="#881337"/>
    </linearGradient>
    <linearGradient id="ch10-bound-inf" x1="24" y1="106" x2="104" y2="106" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#7dd3fc"/>
      <stop offset="50%" stop-color="#2563eb"/>
      <stop offset="100%" stop-color="#1e3a8a"/>
    </linearGradient>
    <linearGradient id="ch10-hasse-edge" x1="64" y1="106" x2="64" y2="26" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#38bdf8"/>
      <stop offset="50%" stop-color="#fbbf24"/>
      <stop offset="100%" stop-color="#f43f5e"/>
    </linearGradient>
  </defs>
  <g>
    <circle cx="64" cy="66" r="48" fill="url(#ch10-glow)"/>
    <line x1="24" y1="26" x2="104" y2="26" stroke="url(#ch10-bound-sup)" stroke-width="3.2" stroke-linecap="round"/>
    <line x1="24" y1="21" x2="24" y2="31" stroke="url(#ch10-bound-sup)" stroke-width="2.4" stroke-linecap="round"/>
    <line x1="104" y1="21" x2="104" y2="31" stroke="url(#ch10-bound-sup)" stroke-width="2.4" stroke-linecap="round"/>
    <line x1="24" y1="106" x2="104" y2="106" stroke="url(#ch10-bound-inf)" stroke-width="3.2" stroke-linecap="round"/>
    <line x1="24" y1="101" x2="24" y2="111" stroke="url(#ch10-bound-inf)" stroke-width="2.4" stroke-linecap="round"/>
    <line x1="104" y1="101" x2="104" y2="111" stroke="url(#ch10-bound-inf)" stroke-width="2.4" stroke-linecap="round"/>
    <polygon points="64,26 98,66 64,106 30,66" fill="url(#ch10-lattice-fill)" stroke="url(#ch10-ink-curve)" stroke-width="2.8" stroke-linejoin="round"/>
    <line x1="64" y1="106" x2="64" y2="26" stroke="url(#ch10-hasse-edge)" stroke-width="1.8" stroke-dasharray="3 3"/>
    <circle cx="30" cy="66" r="8.5" fill="url(#ch10-node-well)" stroke="url(#ch10-ring-vital)" stroke-width="3.0"/>
    <circle cx="30" cy="66" r="3.8" fill="url(#ch10-core-vital)"/>
    <circle cx="98" cy="66" r="8.5" fill="url(#ch10-node-well)" stroke="url(#ch10-ring-gov)" stroke-width="3.0"/>
    <circle cx="98" cy="66" r="3.8" fill="url(#ch10-core-gov)"/>
    <circle cx="64" cy="106" r="8.5" fill="url(#ch10-node-well)" stroke="url(#ch10-ring-epi)" stroke-width="3.0"/>
    <circle cx="64" cy="106" r="3.8" fill="url(#ch10-core-epi)"/>
    <g>
      <circle cx="64" cy="26" r="7.4" fill="var(--logo-node-bg, #e2e8f0)"/>
      <circle cx="64" cy="26" r="6.4" fill="url(#ch10-pupil)" stroke="url(#ch10-gold-mira)" stroke-width="2.4"/>
      <line x1="54.40" y1="26.00" x2="59.80" y2="26.00" stroke="url(#ch10-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="68.20" y1="26.00" x2="73.60" y2="26.00" stroke="url(#ch10-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="64.00" y1="16.40" x2="64.00" y2="21.80" stroke="url(#ch10-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="64.00" y1="30.20" x2="64.00" y2="35.60" stroke="url(#ch10-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <circle cx="64" cy="26" r="2.2" fill="url(#ch10-gold-core)"/>
    </g>
  </g>
  <g transform="translate(148, 0)">
    <text x="2" y="27" fill="var(--logo-sub, #1d4ed8)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.2">PART II · THE BASES · MATHEMATICS</text>
    <text x="498" y="27" text-anchor="end" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.0">CH10 · BOUNDS</text>
    <text x="0" y="59" fill="var(--logo-ink, #0f172a)" font-family="system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif" font-size="22.2" font-weight="800" letter-spacing="1.35">ORDER, SUPREMUM, AND INFIMUM</text>
    <line x1="2" y1="78" x2="384" y2="78" stroke="url(#ch10-ring)" stroke-width="1.6" stroke-opacity="0.82"/>
    <circle cx="400" cy="78" r="6.2" stroke="var(--logo-gold, #f59e0b)" stroke-width="1.5" stroke-dasharray="2.2 1.6"/>
    <circle cx="400" cy="78" r="4.0" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.2"/>
    <circle cx="400" cy="78" r="1.8" fill="var(--logo-gold, #f59e0b)"/>
    <circle cx="415" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.0"/>
    <circle cx="415" cy="78" r="1.6" fill="var(--logo-blue, #2563eb)"/>
    <circle cx="430" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="445" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-green, #10b981)" stroke-width="2.0"/>
    <circle cx="445" cy="78" r="1.6" fill="var(--logo-green, #10b981)"/>
    <circle cx="460" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="475" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="490" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <text x="2" y="104" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.8" font-weight="600" letter-spacing="0.65">PARTIAL ORDERS · COMPLETENESS AXIOM · WORST-CASE INTERVAL ENVELOPES</text>
  </g>
</svg>
</figure>

> Completeness, finite-precision interval enclosures, and why a bound outlives a point estimate.

## By the end of this chapter you can

1. Say why the real numbers need a completeness property that the rationals lack.
2. Compute a supremum and an infimum, and explain why neither need be a member of the set.
3. Read an interval $[L, U]$ as a guarantee rather than a summary, and say what changes when a
   quantity is replaced by it.

## 1. The Tower and Dedekind Completeness

An engineering safety program steps through rational test loads $q\_1, q\_2, \dots$ to find the exact stress where a pressure vessel fractures. Suppose the vessel actually ruptures at $\sqrt{2}$ units of load. If the number system lacks $\sqrt{2}$, rising stress can jump from safe ($q^2 < 2$) straight into failure ($q^2 > 2$) without ever touching a boundary value inside the system. **Completeness** fills every such pinhole on the number line so every bounded risk threshold sits at a genuine number.

Number systems grow step by step to mend what earlier layers could not express:

- The **natural numbers** $\mathbb{N} = \lbrace 0, 1, 2, \dots \rbrace$ count whole items along the positive side of the number line.
- The **integers** $\mathbb{Z} = \lbrace \dots, -2, -1, 0, 1, 2, \dots \rbrace$ extend the line leftward so subtraction never gets stuck below zero.
- The **rational numbers** $\mathbb{Q} = \lbrace p/q : p \in \mathbb{Z}, q \in \mathbb{N} \setminus \lbrace 0 \rbrace \rbrace$ place fractions between integers so division by non-zero counts always produces a point.

That climb was already planetary before modern European axiomatization. In seventh-century India, Brahmagupta's *Brāhmasphuṭasiddhānta* codified explicit arithmetic rules for zero and negative numbers read as fortunes and debts (Plofker 2009). Two centuries later, al-Khwārizmī's treatise on *al-jabr wa-l-muqābala* established systematic algebraic balancing (Rashed 2009).

Even though the rationals $\mathbb{Q}$ are dense—their midpoint $(q\_1 + q\_2)/2$ sits strictly between any two fractions $q\_1 < q\_2$—the rational line still contains pinholes. Check this by hand near $\sqrt{2}$: $1.4^2 = 1.96 < 2$, while $1.5^2 = 2.25 > 2$. Refining further gives $1.41^2 = 1.9881 < 2 < 1.42^2 = 2.0164$, yet no integer fraction $p/q$ ever satisfies $(p/q)^2 = 2$. The set $S = \lbrace q \in \mathbb{Q} : q^2 < 2 \rbrace$ is non-empty and bounded above (for instance by $3/2$), yet it has no tightest upper ceiling inside $\mathbb{Q}$.

The **real numbers** $\mathbb{R}$ close every pinhole through the **least upper bound property** (Dedekind completeness): every non-empty subset of $\mathbb{R}$ that is bounded above possesses a tightest ceiling in $\mathbb{R}$. Formally, a **Dedekind cut** splits $\mathbb{Q}$ into two non-empty halves $(A, B)$ where every $a \in A$ lies left of every $b \in B$ ($a < b$) and $A$ has no largest element. Ordering cuts by subset inclusion makes $A^\ast = \bigcup\_i A\_i$ and $B^\ast = \mathbb{Q} \setminus A^\ast$ the tightest upper ceiling of any bounded family of cuts. Limits, maxima, integrals, and asymptotic bounds all stand on this gapless real line.

## 2. Order, Intervals, and Bounds

A power-grid manual instructs operators to keep line utilization $\rho$ strictly below saturation inside $(0, 1)$. No normal hour ever logs $\rho = 1.0$ before a cascading blackout. Still, that excluded ceiling governs every reserve calculation. **Supremum** and **infimum** explain how a boundary outside a set constrains every point inside it.

The total order on the real line $\mathbb{R}$ defines **intervals** for any endpoints $a \le b$. A **closed interval** includes its endpoints, whereas an **open interval** leaves them out:

$$[a, b] = \lbrace x \in \mathbb{R} : a \le x \le b \rbrace, \qquad (a, b) = \lbrace x \in \mathbb{R} : a < x < b \rbrace.$$

For instance, $[2, 5]$ contains both $2$ and $5$, whereas $(2, 5)$ contains $2.001$ and $4.999$ while excluding $2$ and $5$ themselves. Half-open intervals such as $[a, b)$ include $a$ and exclude $b$.

Now contrast four boundary concepts that beginners often confuse: **maximum ($\max$)**, **minimum ($\min$)**, **supremum ($\sup$)**, and **infimum ($\inf$)**. Let $S \subset \mathbb{R}$ be a non-empty set of real numbers:

- A number $M \in \mathbb{R}$ is an **upper bound** of $S$ if $x \le M$ holds for every $x \in S$.
- When $S$ has an upper bound, its **supremum** ($\sup S$, or least upper bound) is the smallest upper bound: $x \le \sup S$ for all $x \in S$, and every tiny step $\epsilon > 0$ below it admits some $s \in S$ with $s > \sup S - \epsilon$.
- Symmetrically, its **infimum** ($\inf S$, or greatest lower bound) is the largest number sitting on or below every element of $S$.
- A **maximum** ($\max S$) exists only when $\sup S$ actually belongs to $S$; a **minimum** ($\min S$) exists only when $\inf S$ belongs to $S$.

Check two small examples side by side. On the finite set $S = \lbrace 1, 3, 5 \rbrace$, the bounds sit inside the set, so $\max S = \sup S = 5$ and $\min S = \inf S = 1$. On the open interval $(0, 1)$, however, neither a maximum nor a minimum exists: pick any $x \in (0, 1)$ and the midpoint $(x + 1)/2$ is still in $(0, 1)$ and strictly larger. Yet $1$ is the tightest ceiling ($\sup(0, 1) = 1 \notin (0, 1)$) and $0$ is the tightest floor ($\inf(0, 1) = 0 \notin (0, 1)$).

Inequalities let bounds travel safely through arithmetic. Adding any constant $c$ preserves order ($a \le b \implies a + c \le b + c$), and multiplying by a positive factor $k > 0$ keeps direction ($ka \le kb$). Multiplying by a negative number flips the direction ($-a \ge -b$), and bounding two variables below $M\_1$ and $M\_2$ bounds their sum by $M\_1 + M\_2$. Comparing non-substitutable safeguards along the ordered scale $[0, 1]$ belongs to [The Triad and the Minimum](41-the-triad-and-the-minimum.md).

## 3. Interval Arithmetic and Partial Identification

A labor ministry runs a job-training program in which many participants drop out before follow-up. Deleting missing rows and reporting only the respondents' average implicitly assumes dropouts fared just as well as people who answered. **Interval arithmetic** refuses to guess; it propagates the full range consistent with what missing cases could have been.

To see how intervals combine, start with concrete bounds $X \in [2, 5]$ and $Y \in [1, 3]$. The smallest possible sum $X + Y$ pairs the two lows ($2 + 1 = 3$), and the largest pairs the two highs ($5 + 3 = 8$), giving $[3, 8]$. Now subtract $Y$ from $X$: the smallest possible difference $X - Y$ pairs the lowest $X$ with the *highest* $Y$ ($2 - 3 = -1$), while the largest difference pairs the highest $X$ with the *lowest* $Y$ ($5 - 1 = 4$), giving $[-1, 4]$. Notice the widths: $X$ had width $3$ and $Y$ had width $2$, and $X - Y$ widened to $4 - (-1) = 5$.

In general, for any unknown quantities $X$ bounded in $[L\_X, U\_X]$ and $Y$ bounded in $[L\_Y, U\_Y]$, addition and subtraction obey:

$$[L\_X, U\_X] + [L\_Y, U\_Y] = [L\_X + L\_Y,\mkern4mu U\_X + U\_Y],$$

$$[L\_X, U\_X] - [L\_Y, U\_Y] = [L\_X - U\_Y,\mkern4mu U\_X - L\_Y].$$

Read both equations aloud. Adding two intervals adds their lower bounds and adds their upper bounds. Subtracting two intervals subtracts the second interval's upper bound from your lower bound, and subtracts the second interval's lower bound from your upper bound—so uncertainty widens under subtraction instead of canceling out.

Digital computers give a second, hardware-level reason to keep intervals explicit. Unlike the gapless real line $\mathbb{R}$, digital memory stores numbers on a finite grid. Standard binary IEEE-754 formats (`FP64`, `FP32`, `FP16`) and machine-learning formats (`BF16`, `FP8` `E5M2`/`E4M3`) pack each number into $1$ sign bit, $w \ge 1$ biased exponent bits, and $m \ge 1$ explicit mantissa bits. The exponent bias is $\mathrm{bias} = 2^{w-1} - 1$, giving minimum normal exponent $e\_{\min} = 1 - \mathrm{bias}$ and smallest positive normal number $x\_{\min,\mathrm{norm}} = 2^{e\_{\min}}$. Together, the implicit leading bit and stored mantissa give effective precision $p = 1 + m$, with grid spacing $\mathrm{ulp}(z) = 2^{e-(p-1)}$ across $2^e \le |z| < 2^{e+1}$.

| Format | $1+w+m$ | $p$ | $x\_{\min,\mathrm{norm}}$ | $\mathbf{u}=2^{-p}$ |
| :--- | :---: | :---: | :---: | :---: |
| `FP64` | $1+11+52$ | $53$ | $2^{-1022}$ | $2^{-53}$ |
| `FP32` | $1+8+23$ | $24$ | $2^{-126}$ | $2^{-24}$ |
| `BF16` | $1+8+7$ | $8$ | $2^{-126}$ | $2^{-8}$ |
| `FP16` | $1+5+10$ | $11$ | $2^{-14}$ | $2^{-11}$ |
| `FP8` (`E5M2`) | $1+5+2$ | $3$ | $2^{-14}$ | $2^{-3}$ |
| `FP8` (`E4M3`) | $1+4+3$ | $4$ | $2^{-6}$ | $2^{-4}$ |

To read the table, look at **machine epsilon** $\varepsilon\_{\mathrm{mach}} := \mathrm{ulp}(1) = 2^{-(p-1)}$ and **unit roundoff** $\mathbf{u} := 2^{-p}$ (Higham 2002; cf. Goldberg 1991, who calls $2^{-p}$ machine epsilon). On `BF16`, $p = 8$, so $\varepsilon\_{\mathrm{mach}} = 2^{-7} = 1/128$: numbers spaced closer than $1/128$ around $1$ collide on the same grid point, and adding $1 + 0.001$ drops the $0.001$ completely. For nonzero results inside the normalized exponent range (absent overflow or underflow to subnormals), every arithmetic operation $z = a \circ b$ ($\circ \in \lbrace +, -, \times, / \rbrace$, $b \ne 0$) suffers relative rounding error $\mathrm{fl}(a \circ b) = z(1 + \delta)$, where $|\delta| \le \mathbf{u}$ under round-to-nearest-even and $|\delta| < 2\mathbf{u} = \varepsilon\_{\mathrm{mach}}$ under directed rounding toward $\pm\infty$.

That finite grid creates two traps. First, rounding breaks associativity: $(a \oplus b) \oplus c \ne a \oplus (b \oplus c)$ for $a \oplus b := \mathrm{fl}(a + b)$, causing parallel matrix sums ([Chapter 14](14-linear-algebra.md) §3) to drift across hardware schedules. Second, subtracting two nearly equal numbers $a \approx b$ wipes out shared leading digits and leaves prior rounding noise dominant—Goldberg's (1991) **catastrophic cancellation**.

Separating stored mantissa bits $m$ from precision $p = 1 + m$ also reveals range limits. While `BF16` keeps `FP32`'s $8$-bit exponent ($e\_{\min} = -126$), `FP16` and `FP8` shrink $w$ to $5$ or $4$, triggering early overflow and underflow. Near zero, gradual underflow fills the gap below $x\_{\min,\mathrm{norm}}$ with subnormals spaced by $x\_{\min,\mathrm{sub}} = 2^{e\_{\min}-(p-1)}$, keeping $\mathrm{fl}(x - y) = 0 \iff x = y$ (Goldberg 1991). Hardware modes called **flush-to-zero (`FTZ`)** and **denormals-are-zero (`DAZ`)** disable that safeguard by clamping $0 < |z| < x\_{\min,\mathrm{norm}}$ to $0$: for $x = x\_{\min,\mathrm{norm}}(1 + 2^{-(p-1)}) > y = x\_{\min,\mathrm{norm}}$, the positive difference $x - y = x\_{\min,\mathrm{sub}}$ snaps falsely to $\mathrm{fl}(x - y) = 0$. Reliable machine intervals therefore require IEEE-754 **outward-directed rounding** (lower bounds toward $-\infty$, upper bounds toward $+\infty$) with gradual underflow active. Without gradual underflow, any underflowing positive upper bound must widen outward to $+x\_{\min,\mathrm{norm}}$ (and any negative lower bound to $-x\_{\min,\mathrm{norm}}$) so true states never slip outside the enclosure.

Charles Manski (1990) applied this exact interval subtraction to missing counterfactuals. Subtracting a control outcome in $[L\_0, U\_0]$ from a treated outcome in $[L\_1, U\_1]$ yields the **partial identification interval** $[L\_1 - U\_0,\mkern5mu U\_1 - L\_0]$, never a point estimate. Collecting larger samples $n$ cannot shrink missing-outcome width without extra assumptions. On the National Supported Work benchmark (LaLonde 1986; Dehejia & Wahba 1999; quantitative arms audited in [Chapter 59](59-layer-experimental-sampling-causal-order.md)), observational comparison groups leave counterfactuals ranging widely, explaining why naive point-imputed estimates drift far from randomized controls. Full derivations of Manski's two-arm bounds belong to [Chapter 13](13-integrals-and-accumulation.md) §5, [Inference and Evidence](35-inference-and-evidence.md), and [Underdetermination, Partial Identification, and Severe Testing](44-underdetermination-partial-identification.md).

## 4. Heavy Tails, Extremes, and Order Collapse

A property reinsurer prices catastrophe insurance assuming extreme storms decay rapidly like a bell curve, making one giant storm negligible. Under a **heavy-tailed** power law, however, a single extreme year can dwarf all earlier years combined and prevent averages from settling.

Formally, a heavy power-law tail satisfies $\mathbb{P}(X > x) = L(x)x^{-\alpha}$ for tail index $\alpha > 0$ and slowly varying function $L(x)$ (Embrechts, Klüppelberg & Mikosch 1997). That probability decays far slower than any exponential curve $e^{-cx}$. Strictly inside $\alpha < 2$ (or for pure Pareto tails $L(x) \equiv c > 0$ at $\alpha \le 2$), variance diverges to infinity; strictly inside $\alpha < 1$ (or pure Pareto $\alpha \le 1$), even the expected value diverges, so running averages jump wildly with every new record. Right at an integer threshold $\alpha = k \in \lbrace 1, 2 \rbrace$, the $k$-th moment stays finite only when $\int^\infty x^{-1} L(x)\,dx < \infty$ (such as $L(x) = (\ln x)^{-2}$), diverging otherwise.

Studying cotton price records, Benoît B. Mandelbrot (1963) showed that real market returns follow infinite-variance stable laws. He warned analysts never to discard extreme jumps as "outliers," because removing the largest moves deletes the core physics of the series. Formalizing moment regimes, Generalized Pareto exceedances, and Hill tail estimators belongs to [Distribution and Moment](31-distribution-and-moment.md), [Time and Dependence](38-time-and-dependence.md), and [The Non-Identical, Scale Invariance, Convexity, and Heavy Tails](46-non-identical-asymmetry-heavy-tails.md), while [Sequences, Limits, and Continuity](11-sequences-limits-continuity.md) tracks which sample averages still converge.

## 5. Intervals as Honest Answers

A parliamentary committee reviewing the job-training evidence accepts an identification interval straddling zero and rejects a smoothed point number inside it. Replacing a point estimate with $[L, U]$ relative to a decision threshold $\tau$ changes what an institution is allowed to claim:

1. **Vacuous bounds:** if $[L, U]$ covers every feasible value, the archive lacks identifying evidence—an honest finding about the data rather than a failure of math.
2. **Dominance across the interval:** if every value $\theta \in [L, U]$ clears the same side of a pre-set threshold $\tau$ (meaning $L > \tau$ or $U \le \tau$), the decision holds without needing to narrow $[L, U]$ any further.
3. **Bounded worst-case sizing:** when harm has a finite ceiling $U < \infty$, sizing survival reserves against $U$ protects against ruin without turning $U$ into a rigid per-step maximin rule. Such rigidity invites the decision paralysis criticized by Harsanyi (1975) and cannot justify wasteful hoarding past opportunity-cost limits (Scarf 1960; Samuelson 1971; [Chapter 50](50-methodological-families-reflexivity.md) §9.3.2). Under unbounded tails ($U = +\infty$), reserve sizing turns to the tail-index diagnostics of [Chapter 31](31-distribution-and-moment.md).

A claim survives severe testing only when its entire lower bound clears threshold $\tau$ under verified background assumptions (Mayo 2018). When $L \le \tau < U$ **straddles** $\tau$, the interval blocks rubber-stamping a point figure. Yet in irreversible time, freezing action until certainty arrives turns unresolved width into manufactured delay (Proctor & Schiebinger 2008). How institutions weigh reversible probes against protective escalation using tolerance width $\Delta\_{\mathrm{tol}} > 0$ belongs to [The Triad and the Minimum](41-the-triad-and-the-minimum.md) and [Underdetermination, Partial Identification, and Severe Testing](44-underdetermination-partial-identification.md).

### What an interval buys

On the National Supported Work benchmark, publishing Manski's (1990) partial bounds instead of picking one point figure keeps missing-data width and causal assumptions visible side by side ([Chapter 35](35-inference-and-evidence.md); [Chapter 44](44-underdetermination-partial-identification.md)).

### When the interval replaces the score

Static $[L, U]$ intervals fail in four distinct regimes. First, on **unbounded support** ($U = +\infty$ or $L = -\infty$), worst-case bounds stretch to infinity and carry no information. Second, under **capped or truncated logs** ($X \wedge K$), an interval narrower than tolerance $\Delta\_{\mathrm{tol}}$ hides excess damage $(X - K)\_+$ spilled onto external reception perimeters—outside communities and habitats ([Chapter 31](31-distribution-and-moment.md) §7)—or hidden inside non-randomly missing subgroups.

Third, **manufactured doubt** can deliberately stretch $[L, U]$ past $\Delta\_{\mathrm{tol}}$ to stall protective action (Proctor & Schiebinger 2008). Fourth, **regime shifts** may require institutions to revise thresholds openly instead of obeying outdated bounds ([Chapter 02](02-the-average-is-not-the-story.md); [Chapter 29](29-aim-and-the-three-planes.md)). When $L \le \tau < U$ and width $U - L > \Delta\_{\mathrm{tol}}$, the interval records that unresolved gap faithfully instead of faking a point estimate inside it.

## Bridge

[Distribution and Moment](31-distribution-and-moment.md), [Inference and Evidence](35-inference-and-evidence.md), [Chapter 44](44-underdetermination-partial-identification.md), and [Chapter 46](46-non-identical-asymmetry-heavy-tails.md) build heavy-tail diagnostics and partial-identification bounds directly onto these order foundations, while [Chapter 56](56-layer-formal-computable-type-order.md) tracks non-associative floating-point accumulation across `FP64`, `FP32`, and `BF16` and enforces computable outward interval enclosures. [Sequences, Limits, and Continuity](11-sequences-limits-continuity.md) next follows ordered values across time, asking when sequences settle and why single timelines diverge from ensemble averages.
