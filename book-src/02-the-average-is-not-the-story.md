# The Average Is Not the Story

<figure class="stoch-figure stoch-logo-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="800" height="132" viewBox="0 0 800 132" fill="none" role="img" aria-label="Stochology CH02 · CONVEXITY — Averages And Jensen's Gap">
  <defs>
    <!-- Divider Progression: Vital Emerald -> Epistemic Sapphire -> Stókhos Gold -> Institutional Crimson -->
    <linearGradient id="ch02-ring" x1="2" y1="78" x2="386" y2="78" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="38%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="68%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <!-- Multi-spectral Atmospheric Glow -->
    <radialGradient id="ch02-glow" cx="52%" cy="52%" r="50%">
      <stop offset="0%" stop-color="#34d399" stop-opacity="0.25"/>
      <stop offset="48%" stop-color="#2563eb" stop-opacity="0.14"/>
      <stop offset="80%" stop-color="#fbbf24" stop-opacity="0.08"/>
      <stop offset="100%" stop-color="#090e1a" stop-opacity="0"/>
    </radialGradient>
    <!-- Convex Basin Volumetric Shading -->
    <linearGradient id="ch02-convex-fill" x1="24" y1="34" x2="104" y2="96" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#10b981" stop-opacity="0.28"/>
      <stop offset="50%" stop-color="#2563eb" stop-opacity="0.32"/>
      <stop offset="100%" stop-color="#e11d48" stop-opacity="0.24"/>
    </linearGradient>
    <!-- Convex Payoff Curve Ink Stroke -->
    <linearGradient id="ch02-ink-curve" x1="24" y1="66" x2="104" y2="66" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-ink, #0f172a)"/>
      <stop offset="50%" stop-color="var(--logo-sub, #1d4ed8)"/>
      <stop offset="100%" stop-color="var(--logo-ink, #0f172a)"/>
    </linearGradient>
    <!-- Secant Chord Gradient: Sapphire-to-Crimson -->
    <linearGradient id="ch02-secant-grad" x1="28" y1="84" x2="100" y2="40" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="52%" stop-color="#a855f7"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <!-- Jensen Gap Caliper Ruler: Golden Luminous -->
    <linearGradient id="ch02-gap-ruler" x1="64" y1="62" x2="64" y2="85" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#fde047"/>
      <stop offset="50%" stop-color="#f59e0b"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <!-- 3D Spherical Node Gradients -->
    <radialGradient id="ch02-node-well" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="var(--logo-well-hi, var(--elevated-bg, #ffffff))"/>
      <stop offset="100%" stop-color="var(--logo-node-bg, #e2e8f0)"/>
    </radialGradient>
    <linearGradient id="ch02-ring-vital" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#6ee7b7"/>
      <stop offset="50%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#065f46"/>
    </linearGradient>
    <radialGradient id="ch02-core-vital" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#a7f3d0"/>
      <stop offset="45%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#047857"/>
    </radialGradient>
    <linearGradient id="ch02-ring-gov" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#fda4af"/>
      <stop offset="50%" stop-color="#e11d48"/>
      <stop offset="100%" stop-color="#881337"/>
    </linearGradient>
    <radialGradient id="ch02-core-gov" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#fecdd3"/>
      <stop offset="45%" stop-color="#f43f5e"/>
      <stop offset="100%" stop-color="#be123c"/>
    </radialGradient>
    <!-- Golden Stókhos Reticle Gradients -->
    <radialGradient id="ch02-pupil" cx="45%" cy="42%" r="58%">
      <stop offset="0%" stop-color="#020617"/>
      <stop offset="68%" stop-color="#0f172a"/>
      <stop offset="100%" stop-color="#451a03"/>
    </radialGradient>
    <linearGradient id="ch02-gold-mira" x1="54" y1="48" x2="84" y2="82" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#fef08a"/>
      <stop offset="45%" stop-color="#fbbf24"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <radialGradient id="ch02-gold-core" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#ffffff"/>
      <stop offset="35%" stop-color="#fde047"/>
      <stop offset="100%" stop-color="#d97706"/>
    </radialGradient>
  </defs>
  <g>
    <!-- Atmospheric Glow & Reference Coordinate Horizon -->
    <circle cx="64" cy="66" r="48" fill="url(#ch02-glow)"/>
    <circle cx="64" cy="66" r="42" stroke="var(--logo-ticks, #64748b)" stroke-width="1.2" stroke-dasharray="3 3" stroke-opacity="0.45"/>
    <!-- Volumetric Convex Basin Shading & Floor Axis -->
    <path d="M 24 88 Q 64 104 104 34 L 104 96 L 24 96 Z" fill="url(#ch02-convex-fill)"/>
    <line x1="20" y1="96" x2="108" y2="96" stroke="var(--logo-ticks, #64748b)" stroke-width="1.4" stroke-dasharray="3 3" stroke-opacity="0.5"/>
    <!-- Strictly Convex Payoff Curve f(x) -->
    <path d="M 24 88 Q 64 104 104 34" stroke="url(#ch02-ink-curve)" stroke-width="3.4" stroke-linecap="round"/>
    <!-- Secant Chord Connecting Points A and B -->
    <line x1="28" y1="84" x2="100" y2="40" stroke="url(#ch02-secant-grad)" stroke-width="2.6" stroke-dasharray="4 3" stroke-linecap="round"/>
    <!-- Jensen's Gap Caliper: E[f(x)] >= f(E[x]) -->
    <line x1="64" y1="62" x2="64" y2="85" stroke="url(#ch02-gap-ruler)" stroke-width="2.8" stroke-linecap="round"/>
    <line x1="59" y1="62" x2="69" y2="62" stroke="url(#ch02-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
    <line x1="59" y1="85" x2="69" y2="85" stroke="url(#ch02-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
    <!-- 3D Spherical Nodes at Secant Anchors -->
    <!-- Point A (Vital Margin Reserve) at (28, 84) -->
    <circle cx="28" cy="84" r="8.5" fill="url(#ch02-node-well)" stroke="url(#ch02-ring-vital)" stroke-width="3.2"/>
    <circle cx="28" cy="84" r="3.8" fill="url(#ch02-core-vital)"/>
    <!-- Point B (Governance Threshold) at (100, 40) -->
    <circle cx="100" cy="40" r="8.5" fill="url(#ch02-node-well)" stroke="url(#ch02-ring-gov)" stroke-width="3.2"/>
    <circle cx="100" cy="40" r="3.8" fill="url(#ch02-core-gov)"/>
    <!-- Expectation Point Stókhos Reticle at Secant Center (64, 62) -->
    <g id="ch02-reticle">
      <circle cx="64" cy="62" r="7.4" fill="var(--logo-node-bg, #e2e8f0)"/>
      <circle cx="64" cy="62" r="6.4" fill="url(#ch02-pupil)" stroke="url(#ch02-gold-mira)" stroke-width="2.4"/>
      <line x1="54.4" y1="62" x2="59.8" y2="62" stroke="url(#ch02-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="68.2" y1="62" x2="73.6" y2="62" stroke="url(#ch02-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="64" y1="52.4" x2="64" y2="57.8" stroke="url(#ch02-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="64" y1="66.2" x2="64" y2="71.6" stroke="url(#ch02-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <circle cx="64" cy="62" r="2.2" fill="url(#ch02-gold-core)"/>
    </g>
  </g>
  <g transform="translate(148, 0)">
    <text x="2" y="27" fill="var(--logo-sub, #1d4ed8)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.2">PART I · THINKING UNDER UNCERTAINTY</text>
    <text x="498" y="27" text-anchor="end" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.0">CH02 · CONVEXITY</text>
    <text x="0" y="59" fill="var(--logo-ink, #0f172a)" font-family="system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif" font-size="24.8" font-weight="800" letter-spacing="1.65">AVERAGES AND JENSEN'S GAP</text>
    <line x1="2" y1="78" x2="384" y2="78" stroke="url(#ch02-ring)" stroke-width="1.6" stroke-opacity="0.82"/>
    <circle cx="400" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="415" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="430" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.0"/>
    <circle cx="430" cy="78" r="1.6" fill="var(--logo-blue, #2563eb)"/>
    <circle cx="445" cy="78" r="6.2" stroke="var(--logo-gold, #f59e0b)" stroke-width="1.5" stroke-dasharray="2.2 1.6"/>
    <circle cx="445" cy="78" r="4.0" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-green, #10b981)" stroke-width="2.2"/>
    <circle cx="445" cy="78" r="1.8" fill="var(--logo-gold, #f59e0b)"/>
    <circle cx="460" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-green, #10b981)" stroke-width="2.0"/>
    <circle cx="460" cy="78" r="1.6" fill="var(--logo-green, #10b981)"/>
    <circle cx="475" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="490" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <text x="2" y="104" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.8" font-weight="600" letter-spacing="0.65">NONLINEAR PAYOFFS · BIMODAL BASINS · CONVEXITY DIVERGENCE</text>
  </g>
  </svg>
</figure>

> Intuition-first analysis of dispersion and convexity; formal proofs sit in [Distribution and Moment](31-distribution-and-moment.md) and [Sample, Population, Archive](34-sample-population-archive.md).

## By the end of this chapter you can

1. Compute an average and identify the spread, extreme shocks, and selective omission it systematically leaves out.
2. Give two sets of numbers with the same average and completely different behaviour under upward-curving penalties.
3. Explain why planning with an average can be right about the number and wrong about the plan, and why huge selective datasets cannot cure bias.

## 1. One number for many cases

A reinsurance manager prices industrial fire cover for the coming year. She looks back at historical
fire claims, computes the arithmetic average severity per claim, sets the per-claim reserve buffer
around that average, and reports a balanced ledger. Months later an industrial complex burns for a sum
the buffer cannot cover. The reserve is gone in an afternoon.

Ordinary arithmetic shows why. A football team scores $2$, $0$, $3$, $1$, and $4$ goals in five matches; add them up ($10$) and divide by $5$, and the average is $2$. As a summary that is honest. As a forecast it says nothing, because two teams can both average $2$ and be nothing alike. One scores $2$ every week. The other scores $0$, $0$, $0$, $0$, and $10$.

Now line each team's scores up from smallest to largest and point at the middle one. That middle value is the **median**. For the steady team ($2, 2, 2, 2, 2$), both the average and the median are $2$. For the feast-or-famine team ($0, 0, 0, 0, 10$), the average is $2$ while the median is $0$. If that single $10$-goal explosion turns into $100$, the average jumps to $20$ while the middle score stays $0$.

Let us write that simple contrast down in symbols. For $n \ge 1$ observations $X\_1, \dots, X\_n$, the arithmetic **mean** (average) $\bar{X}$ adds them up and divides by the count $n$:

$$\bar{X} = \frac{1}{n}\sum\_{i=1}^n X\_i$$

Read that aloud: sum all $n$ observations indexed by $i$ from $1$ to $n$, and divide by $n$. Sort the same numbers from smallest $X\_{(1)}$ to largest $X\_{(n)}$, and the **median** $M$ sits at the middle position (or halfway between the two middle values when $n$ is even). Raise any observation above the middle, and $\bar{X}$ climbs immediately while $M$ stays put. The mean tells you total divided by count, not a typical day. In a skewed ledger, **the median captures a typical single event while hiding the rare blowups that drive total damage, whereas the mean scales the total while hiding spread and volatility.**

## 2. A hundred numbers and the convexity trap

Here is an example small enough to check by hand. Ninety-nine measurements equal $1$, one equals
$1{,}001$, and the average is $11$:

$$\bar{X} = \frac{99 \times 1 + 1 \times 1001}{100} = \frac{1100}{100} = 11$$

Read that arithmetic aloud: $99$ ones plus one $1{,}001$ makes $1{,}100$ in total, which divided by $100$ items gives an average of $11$. Almost every value is $1$ and one is a thousand times bigger. Plan on $11$ and you are wrong ninety-nine times out of a hundred, and catastrophically wrong once. Since $n = 100$ is even, the median $M$ averages the fiftieth and fifty-first sorted entries—both equal to $1$, so $M = 1$.

Now notice why averages betray plans even faster when real damage bends upward. Imagine a power grid or hospital ward: running slightly below capacity causes minor delay, whereas running well above capacity triggers blackouts or jammed hallways. When each extra step of overload hurts *more* than the previous step—so the penalty curve bends upward like a bowl—we call the penalty **convex**.

Check a two-day toy example with quadratic harm $\phi(x) = x^2$. Suppose over two steady days your overload is $2$ and $2$: the average overload is $2$, the harm each day is $2^2 = 4$, and the average harm is $4$. Now suppose over two bumpy days your overload is $0$ and $4$: the average overload is *still* $(0 + 4)/2 = 2$, yet the harms are $0^2 = 0$ and $4^2 = 16$, making the average harm $(0 + 16)/2 = 8$—twice as high. Harm evaluated at the average ($2^2 = 4$) understates the true average harm ($8$). Evaluating [Chapter 01](01-deciding-without-knowing.md)'s loss $L$ at an average severity repeats the exact same trap whenever real harm curves upward.

Johan Jensen proved this general rule in 1906 (Jensen 1906). Write $X$ for a random outcome with finite mean ($\mathbb{E}[|X|] < \infty$), $\mathbb{E}$ for expectation, and $\phi$ for any strictly upward-curving (strictly convex) finite-valued harm function across outcomes that actually vary:

$$\mathbb{E}[\phi(X)] > \phi(\mathbb{E}[X])$$

Read that inequality aloud: **the average of a curved-upward harm is strictly greater than the harm of the average.** Whenever expected harm is also finite ($\mathbb{E}[|\phi(X)|] < \infty$), plugging an average forecast into a curved cost curve leaves your budget short by the finite **convexity gap** $\Delta\_J = \mathbb{E}[\phi(X)] - \phi(\mathbb{E}[X])$.

On our ninety-nine ones and single $1{,}001$ under quadratic harm $\phi(x) = x^2$, plugging in the average gives $\phi(\bar{X}) = 11^2 = 121$, while averaging the actual squared harms gives $\mathbb{E}[X^2] = \frac{99 \times 1^2 + 1 \times 1001^2}{100} = 10{,}021$. The hidden budget hole is $\Delta\_J = 10{,}021 - 121 = 9{,}900$—over eighty times the naive estimate.

Two simple boundaries govern that gap:

1. **Flat costs or zero wobble ($\Delta\_J = 0$):** If costs climb in a straight line (doubling overload simply doubles cost) or every day looks identical, there is no convexity gap.
2. **Exploding extreme shocks:** When huge outliers dominate so severely that average curved damage grows without bound as you collect more records, sample averages jump unpredictably from one shock to the next. Instead of trusting a sample average, analysts inspect how fast extreme values thin out across high cutoffs (Hill 1975; taken up in [Chapter 03](03-the-biggest-number-in-the-room.md) and [Chapter 31](31-distribution-and-moment.md), with formal boundaries in [Chapter 10](10-numbers-order-bounds.md)).

## 3. Spread and shape

Spread is the second thing an average erases, and lopsided concentration is the third.

Two classes average $70$ on the same test. One class scores $68, 69, 70, 71, 72$; the other scores $40, 55, 70, 85, 100$. In the first, everyone is in roughly the same place; in the second, half the room is lost and half is bored. To measure how wide values scatter around their mean $\bar{X}$, take each value's gap from the mean ($X\_i - \bar{X}$), square it so positives and negatives do not cancel out, average those squared gaps over $n - 1$ (for a sample of $n \ge 2$ rows), and take the square root to return to the original units. That gives the **sample standard deviation** $s$:

$$s = \sqrt{\frac{1}{n - 1}\sum\_{i=1}^n (X\_i - \bar{X})^2}$$

Read that aloud: square each distance from the mean, average over $n - 1$, and take the square root. Check the two classes by hand: for $68, 69, 70, 71, 72$, the gaps are $-2, -1, 0, +1, +2$, their squares add to $4 + 1 + 0 + 1 + 4 = 10$, and $s = \sqrt{10/4} = \sqrt{2.5} \approx 1.58$. For $40, 55, 70, 85, 100$, the gaps are $-30, -15, 0, +15, +30$, their squares add to $900 + 225 + 0 + 225 + 900 = 2{,}250$, and $s = \sqrt{2250/4} = \sqrt{562.5} \approx 23.72$—fifteen times wider on the exact same average.

Yet spread alone still treats upward and downward surprises symmetrically. When non-negative losses ($X\_i \ge 0$, summing to a positive total) lean heavily to one side, we also ask how much of the entire damage comes from just the worst $k$ cases out of $n$ (for $1 \le k \le n$). Using the sorted values $X\_{(1)}, \dots, X\_{(n)}$, the **top-$k$ concentration share** $C\_k$ is:

$$C\_k = \frac{\sum\_{i=n-k+1}^{n} X\_{(i)}}{\sum\_{i=1}^{n} X\_i} \qquad \left(X\_i \ge 0,\mkern5mu \sum\_{i=1}^{n} X\_i > 0,\mkern5mu 1 \le k \le n\right)$$

Read that fraction aloud: add up the $k$ biggest values in the sorted list, and divide by the sum of all $n$ values. Two quick checks bracket it. If everyone is identical, the top $k$ rows hold exactly $k/n$ of the total—the top $1\unicode{x25}$ holds $1\unicode{x25}$. In our ninety-nine ones plus one $1{,}001$, the single largest row ($k = 1$, just $1\unicode{x25}$ of rows) holds $C\_1 = 1001 / 1100 = 91.0\unicode{x25}$ of the entire sum. Real failures live in spread and concentration, not in the average.

## 4. Why plans break: Big data and the illusion of certainty

Planning around an average bets that the unusual day will not arrive while the plan runs. A hospital staffed for average arrivals jams during a flu surge; a bridge built for average wind snaps in a gale—reflecting Georges Canguilhem's (1943/1966) observation that living health depends on flexible margins rather than clinging to a static norm.

Piling up huge amounts of data does not fix a blind average either. If an app survey misses older residents or a program follow-up tracks only graduates who stayed in touch ([Who Is Missing From the Table](06-who-is-missing-from-the-table.md)), Xiao-Li Meng (2018) proved with the **Big Data Paradox** that adding a million more rows through the same skewed intake door shrinks error bars around the wrong center instead of curing bias.

When stakes are high, analysts therefore look beyond the mean to upper percentiles and to the average loss inside the worst slice of outcomes—a coherent tail measure analyzed by Carlo Acerbi and Dirk Tasche (Acerbi & Tasche 2002)—while keeping simple structural floors in reserve, because simple rules often beat fragile models fitted to short or shifting records (Neth & Gigerenzer 2015; Haldane & Madouros 2012).

### What the average hides, and what bounds it

An archive often exposes its own mean: when the mean sits far above the median, skewness and extreme cases are already pulling the ledger apart.

Nor does a contractual ceiling or payout cap make the average safe. Suppose an insurer caps payouts at some limit $K$ (meaning no claim on its books ever exceeds $K$). That cap truncates the *payer's bill*, never the real physical fire or flood damage. Curved harm still hits below $K$, capped claims still pull the mean above the median, and every dollar of damage above $K$ spills silently onto uninsured households and neighbors absent from the insurer's ledger ([Who Is Missing From the Table](06-who-is-missing-from-the-table.md); [Chapter 31](31-distribution-and-moment.md)). Extreme-value specialists therefore inspect how tail estimates behave across multiple high cutoffs—together with their uncertainty bands—rather than trusting a single average or a single threshold (McNeil 1997; Embrechts, Klüppelberg & Mikosch 1997).

### When the average is the right instrument

Planning by the average works safely only when four conditions hold together:

- Harms scale in a straight line rather than curving upward, so there is no convexity gap.
- Losses are genuinely bounded without a cap dumping overflow onto outsiders.
- Many small, independent events pool smoothly without one catastrophic shock wiping out the buffer.
- The collection door is unbiased and watched long enough to catch rare extremes ([Chapter 03](03-the-biggest-number-in-the-room.md)).

When short or unstable histories break estimation, simple structural safety floors govern instead; otherwise, inspect percentiles, worst-slice tail averages, and stability checks across high cutoffs.

## Bridge

[Distribution and Moment](31-distribution-and-moment.md) formalizes moment regimes, the convexity gap $\Delta\_J$, tail-index intervals, and capped liabilities, while [Integrals and Accumulation](13-integrals-and-accumulation.md) and [Sample, Population, Archive](34-sample-population-archive.md) supply coherent shortfall and effective sample audits. [The Biggest Number in the Room](03-the-biggest-number-in-the-room.md) turns next to the extreme shocks that decide physical survival.
