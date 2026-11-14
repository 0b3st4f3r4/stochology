# The Average Is Not the Story

<figure class="stoch-figure stoch-logo-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="660" height="132" viewBox="0 0 660 132" fill="none" role="img" aria-label="Stochology CH02 · CONVEXITY — Averages And Jensen'S Gap">
  <defs>
    <linearGradient id="ch02-ring" x1="2" y1="78" x2="386" y2="78" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="38%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="68%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <radialGradient id="ch02-glow" cx="50%" cy="50%" r="50%">
      <stop offset="0%" stop-color="#34d399" stop-opacity="0.25"/>
      <stop offset="50%" stop-color="#10b981" stop-opacity="0.14"/>
      <stop offset="82%" stop-color="#fbbf24" stop-opacity="0.08"/>
      <stop offset="100%" stop-color="#090e1a" stop-opacity="0"/>
    </radialGradient>
    <linearGradient id="ch02-triad-stroke" x1="18" y1="24" x2="110" y2="104" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.85"/>
      <stop offset="50%" stop-color="var(--logo-gold, #f59e0b)" stop-opacity="0.90"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.85"/>
    </linearGradient>
    <linearGradient id="ch02-fill" x1="22" y1="30" x2="106" y2="98" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)" stop-opacity="0.26"/>
      <stop offset="48%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.28"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.26"/>
    </linearGradient>
    <linearGradient id="ch02-ink-curve" x1="20" y1="66" x2="108" y2="66" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-ink, #0f172a)"/>
      <stop offset="50%" stop-color="var(--logo-sub, #1d4ed8)"/>
      <stop offset="100%" stop-color="var(--logo-ink, #0f172a)"/>
    </linearGradient>
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
    <radialGradient id="ch02-node-well" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="var(--logo-well-hi, var(--elevated-bg, #ffffff))"/>
      <stop offset="100%" stop-color="var(--logo-node-bg, #e2e8f0)"/>
    </radialGradient>
    <linearGradient id="ch02-ring-epi" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#7dd3fc"/>
      <stop offset="50%" stop-color="#2563eb"/>
      <stop offset="100%" stop-color="#1e3a8a"/>
    </linearGradient>
    <radialGradient id="ch02-core-epi" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#bae6fd"/>
      <stop offset="45%" stop-color="#3b82f6"/>
      <stop offset="100%" stop-color="#1d4ed8"/>
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
  </defs>
  <g>
    <circle cx="64" cy="66" r="48" fill="url(#ch02-glow)"/>
    <circle cx="64" cy="66" r="42" stroke="var(--logo-ticks, #64748b)" stroke-width="1.2" stroke-dasharray="3 3" stroke-opacity="0.45"/>
    <path d="M 24 88 Q 64 102 104 34" stroke="url(#ch02-ink-curve)" stroke-width="3.0" stroke-linecap="round"/>
    <line x1="28" y1="84" x2="100" y2="40" stroke="var(--logo-blue, #2563eb)" stroke-width="2.2" stroke-dasharray="4 3" stroke-linecap="round"/>
    <line x1="64" y1="62" x2="64" y2="85" stroke="url(#ch02-gold-mira)" stroke-width="2.8" stroke-linecap="round"/>
    <circle cx="28.00" cy="84.00" r="6.55" fill="url(#ch02-node-well)" stroke="url(#ch02-ring-vital)" stroke-width="2.42"/>
    <circle cx="28.00" cy="84.00" r="2.81" fill="url(#ch02-core-vital)"/>
    <circle cx="100.00" cy="40.00" r="6.55" fill="url(#ch02-node-well)" stroke="url(#ch02-ring-gov)" stroke-width="2.42"/>
    <circle cx="100.00" cy="40.00" r="2.81" fill="url(#ch02-core-gov)"/>
    <g>
      <circle cx="64.00" cy="62.00" r="6.81" fill="var(--logo-node-bg, #e2e8f0)"/>
      <circle cx="64.00" cy="62.00" r="5.89" fill="url(#ch02-pupil)" stroke="url(#ch02-gold-mira)" stroke-width="2.21"/>
      <line x1="55.17" y1="62.00" x2="60.14" y2="62.00" stroke="url(#ch02-gold-mira)" stroke-width="2.21" stroke-linecap="round"/>
      <line x1="67.86" y1="62.00" x2="72.83" y2="62.00" stroke="url(#ch02-gold-mira)" stroke-width="2.21" stroke-linecap="round"/>
      <line x1="64.00" y1="53.17" x2="64.00" y2="58.14" stroke="url(#ch02-gold-mira)" stroke-width="2.21" stroke-linecap="round"/>
      <line x1="64.00" y1="65.86" x2="64.00" y2="70.83" stroke="url(#ch02-gold-mira)" stroke-width="2.21" stroke-linecap="round"/>
      <circle cx="64.00" cy="62.00" r="2.02" fill="url(#ch02-gold-core)"/>
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

> School-level chapter of Part I. It needs only division. The formal version is
> [Distribution and Moment](31-distribution-and-moment.md), and the tail argument is
> [The Biggest Number in the Room](03-the-biggest-number-in-the-room.md).

## By the end of this chapter you can

1. Compute an average and identify the dispersion, tail risk, and selection bias it systematically leaves out.
2. Give two sets of numbers with the same average and completely different behaviour under nonlinear and convex penalties.
3. Explain why planning with an average can be right about the number and wrong about the plan, showing how sample size inflates false certainty when the collection door is biased.

## 1. One number for many cases

A reinsurance manager prices industrial fire cover for the coming year. She looks back at the historical
fire claims, computes the arithmetic average severity per claim, sets the per-claim reserve buffer
around that average, and reports a balanced ledger. Months later an industrial complex burns for a sum
the buffer cannot begin to cover. The reserve is gone in an afternoon.

Ordinary arithmetic shows why. A football team scores 2, 0, 3, 1, and 4 goals in five matches; the
average is 2. As a summary that is honest. As a forecast it says nothing, because two teams can both
average 2 and be nothing alike. One scores 2 every week. The other scores 0, 0, 0, 0, and 10.

Nothing about the second team is unusual, and the average cannot show it. Averaging is an operation of
erasure. In $\bar{X} = \frac{1}{n}\sum\_{i=1}^n X\_i$, every observation enters with equal weight $1/n$
and linearly in its size. The order-statistic median $M = X\_{((n+1)/2)}$ depends only on which value
sits in the middle rank. So the mean is sensitive to every observation and the median to none of the
extremes: raise the largest observation by any amount and the mean rises with it, while the median does
not move unless the increase carries that observation across the middle rank. What the mean reports is
the ratio of the total to the count, not the size of a typical case. In a skewed portfolio, that is the
number nobody needs. **The mean is the statistic that the extreme event moves, and the median is the statistic
that the extreme event leaves untouched.**

## 2. A hundred numbers and the convexity trap

Here is an example small enough to check by hand. Ninety-nine measurements equal 1, one equals
1 001, and the average is 11:

$$\bar{X} = \frac{99 \times 1 + 1 \times 1001}{100} = \frac{1100}{100} = 11$$

Almost every value is 1 and one is a thousand times bigger. Plan with the average and you expect
something near 11 every time, are wrong ninety-nine times out of a hundred, and catastrophically wrong
once. The median is 1: the value with half the observations above it and half below.

Consequences are not usually linear. In real systems the harm of a shortage or delay is not
proportional: running 10% below capacity is a minor inconvenience, running 50% above it triggers
cascading grid collapse or an emergency room that cannot move. Johan Jensen proved in 1906 (Jensen
1906) that whenever a penalty function $\phi(X)$ is **convex** (curving upward like a bowl), the
expected loss is strictly greater than the loss evaluated at the average:

$$\mathbb{E}[\phi(X)] > \phi(\mathbb{E}[X])$$

So **Jensen's inequality** holds, and planning for the average guarantees systematic
under-provisioning. The difference $\Delta\_J = \mathbb{E}[\phi(X)] - \phi(\mathbb{E}[X])$
is the **convexity gap**: the unbudgeted ruin that falls on whoever planned on the mean.

Measure $\Delta\_J$ on the hundred numbers under a quadratic stress penalty $\phi(x) = x^2$. At the average
the penalty is $\phi(\bar{X}) = 11^2 = 121$. Averaging the actual squared penalties gives
$\mathbb{E}[X^2] = \frac{99 \times 1^2 + 1 \times 1001^2}{100} = 10{,}021$. The unbudgeted gap is
$\Delta\_J = 10{,}021 - 121 = 9{,}900$, more than eighty times the naive estimate. It comes from
neither the data alone nor the curvature alone but from their product: a portfolio with the same mean
and no extreme value has a gap near zero, and a linear penalty has none at all. Two limits govern
$\Delta\_J$:

1. **No spread, or no curvature ($\mathrm{Var}(X) \to 0^+$ or $p \to 1^+ \implies \Delta\_J \to 0$).** If every observation equals the mean ($X\_i \equiv \bar{X}$) or the cost function is strictly linear ($\phi(x) = a x + b$, zero curvature), then $\mathbb{E}[\phi(X)] = \phi(\mathbb{E}[X])$ and $\Delta\_J = 0$.
2. **A tail heavier than the penalty ($\alpha \le p \implies \mathbb{E}[X^p] \to \infty$).** When the Pareto tail index $\hat{\alpha}$ (Hill 1975) lies below the convexity exponent $p$ of the stress function (and strictly below $2$, so population variance is infinite), the population expectation $\mathbb{E}[X^p]$ diverges as sample size grows. No fixed markup on $\phi(\bar{X})$ can then bound the tail stress. Estimating the index is the only honest description of the portfolio, and reporting the mean of a divergent moment is not a conservative choice but a category error.

## 3. Spread and shape

Spread is the second measure, and shape the third: whether the spread is symmetric or pulled into a
lopsided tail.

Two classes average 70 on the same test. One has marks 68, 69, 70, 71, 72 (sample standard deviation
$s = 1.58$); the other has 40, 55, 70, 85,
100 ($s = 23.72$). No teacher would call those two classes the same. In the first everybody learned
roughly the same thing. In the second half the class is lost and half is bored.

With an asymmetric distribution, symmetric spread is still not enough. Measure how much of the total
sum the top $k$ order statistics carry, $X\_{(n-k+1)}, \dots, X\_{(n)}$,

$$S\_k = \frac{\sum\_{i=n-k+1}^{n} X\_{(i)}}{\sum\_{i=1}^{n} X\_i}$$

Two exact boundary cases bracket it. In an egalitarian sample ($X\_1 = \dots = X\_n > 0$), $S\_k = k / n$, so the top $1\%$ of observations carries $1\%$ of the total. In the single-spike sample of ninety-nine ones and a single $1{,}001$, the largest observation ($k = 1$, or $1\%$ of the rows) carries $S\_1 = 1001 / 1100 = 91.0\%$ of the entire sum, and $S\_1 \to 1$ as $X\_{(n)} \to \infty$. A fire portfolio's top percentile of claims can carry a share of the total the mean gives no hint of, and its largest claim can sit many multiples of the median away. Spread and tail concentration are not nuisances to be averaged away. They are where the failures happen.

## 4. Why plans break: Big data and the illusion of certainty

Planning with an average bets that the unusual case will not arrive while the plan runs. Usually a fine
bet. Occasionally fatal. A hospital averaging ten arrivals per hour is fine at eleven and in trouble at
thirty; a bus sized to the average leaves children behind on the trip. The question was never the
average, but how bad the worst hours get and how often. An agent that sizes its community reserve on
the unbuffered archive average, without an epistemic margin, covers demand on ordinary shifts and
abandons the shifts the buffer was bought for. The average it priced is the one the archive happens to
show, not the one the shock will deliver. The reserve draws down shift after shift until the corner it
was meant to cover arrives.

The modern temptation is to think that collecting massive amounts of data solves this. It does not. Let
an organisation collect millions of rows through a selective door — voluntary smartphone responses that
under-represent the elderly, post-program surveys that average only over employed "survivors" (Wald
1943) — and the sample average still does not converge to the truth. Xiao-Li Meng's **Big Data
Paradox** (Meng 2018) gives the reason: whenever the correlation between whether an event is recorded
and its outcome is non-zero ($\rho\_{R,Y} \ne 0$) at sampling fraction $f = n/N$, the unified effective
sample size obeys the harmonic precision law displayed in
[Who Is Missing From the Table](06-who-is-missing-from-the-table.md). Millions of draws through a door
correlated even weakly with the outcome can carry the information of a few hundred random draws.
Keynesian evidential weight collapses with them.

So the naive interval shrinks around a wrong number. The **Institution** grows certain of an
**Archive** illusion while its **Vital Margin** ($S\_{\mathrm{vital}}$) and **Epistemic Margin**
($S\_{\mathrm{epi}}$) collapse together. An honest analysis therefore never stops at a point average.
It reports the quantiles, the conditional tail expectation above them, the convex penalty gap
$\Delta\_J$, and the effective sample size $n\_{\mathrm{eff}}$ of the archive that produced it.

### What the average hides, and what bounds it

The file that gives the average also refutes it as a description. In a heavy-tailed portfolio the mean
sits a multiple of the median away, the largest one per cent carries a large share of the total paid,
and the largest ten per cent carries most of it. The gap between mean and median is not noise. It is
the shape of the support showing through.

The counter-example is the bounded archive. Where every claim is capped by the structure of the
exposure (a call volume its trunk capacity bounds, a loss its contract limits), mean and median sit
close together and the largest observation is a small multiple of the mean. Pricing at the average then
errs by a factor the support already bounds. Where nothing caps the next claim, a reserve priced at the
average of the file is not conservative. It is a bet that the largest claim in the file will not recur.
The actuarial treatment of heavy-tailed claim sizes was consolidated in the
1990s (McNeil 1997; Embrechts, Klüppelberg & Mikosch 1997), and the rule it leaves is the one used
here: report the index, not the moment.

### When the average is the right instrument

The thesis stops holding when the decision is evaluated over many independent, small, bounded
realisations. Then the aggregate loss concentrates and the mean decides the plan. So test the file
before choosing the statistic. If mean and median nearly coincide and the maximum is a small multiple
of the mean, the average is admissible. If the mean sits well above the median and the largest
observation is many multiples of it, price the plan on a quantile and on the conditional loss above it,
and report the tail index next to both.

## Bridge

[Distribution and Moment](31-distribution-and-moment.md) makes this precise: the distribution replaces
the average, and the convexity gap $\Delta\_J = \mathbb{E}[\phi(X)] - \phi(\mathbb{E}[X])$ explains why
a loss evaluated at the mean understates the true expected loss. That chapter also carries the tail
index and the conditional loss above a quantile. [Measuring and Erring](33-measuring-and-erring.md)
turns the effective sample size and the Keynesian weight into an audit every summary statistic has to
pass, and the tolerance margin of Canguilhem's physiology, the latitude an organism keeps around its
own norms, is the biological form of the same warning against static averages.
[The Biggest Number in the Room](03-the-biggest-number-in-the-room.md) turns to the cases where one
extreme value decides everything.
