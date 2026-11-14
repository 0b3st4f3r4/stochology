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

> Second chapter of the mathematical block in Part II. The subject is the number line, and the reason
> a bound is a stronger object than an estimate.

## By the end of this chapter you can

1. Say why the real numbers need a completeness property that the rationals lack.
2. Compute a supremum and an infimum, and explain why neither need be a member of the set.
3. Read an interval $[L, U]$ as a guarantee rather than a summary, and say what changes when a
   quantity is replaced by it.

## 1. The Tower and Dedekind Completeness

An engineering safety solver searches for the exact stress level at which a containment vessel fractures,
stepping through rational test loads $q\_1, q\_2, \dots$. If the number system has microscopic pinholes at
irrational coordinates such as $\sqrt{2}$, a rising physical stress trajectory can cross from the safe
region ($q^2 < 2$) into catastrophic rupture ($q^2 > 2$) without ever passing a well-defined threshold
number in the system. Dedekind completeness is the structural guarantee that the real line has no such
pinholes: every bounded escalation of risk has a least upper bound where an institutional safety limit can
be anchored.

Number systems are built in layers. Each layer answers a deficiency of the one beneath it:

- The **naturals** $\mathbb{N} = \lbrace0, 1, 2, \dots\rbrace$ support counting and succession.
- The **integers** $\mathbb{Z}$ close the naturals under subtraction.
- The **rationals** $\mathbb{Q} = \lbrace{}p/q : p \in \mathbb{Z}, q \in \mathbb{N} \setminus \lbrace0\rbrace\rbrace$ close the
  integers under division by non-zero elements.

The rational field $\mathbb{Q}$ is dense: between any two distinct rationals $q\_1 < q\_2$ sits another
rational, $(q\_1 + q\_2)/2$. Density is not enough for analysis. The set
$S = \lbrace{}q \in \mathbb{Q} : q^2 < 2\rbrace$ is non-empty and bounded above (by $3/2$, for instance), and it has
no least upper bound inside $\mathbb{Q}$, because $\sqrt{2} \notin \mathbb{Q}$. Rational sequences that
ought to converge find no endpoint.

The **reals** $\mathbb{R}$ close the gap through the **least upper bound property** (Dedekind
completeness): every non-empty subset of $\mathbb{R}$ bounded above has a supremum in $\mathbb{R}$.
Completeness is constructed, not assumed. A **Dedekind cut** is a partition $(A, B)$ of $\mathbb{Q}$ with
$a < b$ for all $a \in A$ and $b \in B$, and $A$ without a greatest element; ordering the cuts by inclusion
makes $A^\ast = \bigcup\_i A\_i$ with $B^\ast = \mathbb{Q} \setminus A^\ast$ the least upper bound.
Limits, continuous extrema, Riemann and Lebesgue integrals, asymptotic bounds: all of them rest on
completeness, and without it limiting arguments collapse into unresolvable gaps.

## 2. Order, Intervals, and Bounds

A power-grid dispatcher works to an operating manual that requires transmission utilization $\rho$ to stay
strictly below saturation, inside the open interval $(0, 1)$. No normal operating hour records $\rho = 1.0$
before a cascading trip occurs, yet the number $1.0$, which lies outside the admissible operating set, is
the boundary that governs reserve sizing. Supremum and infimum formalize how a boundary outside an observed
set constrains every state inside it.

A total order on $\mathbb{R}$ lets us write bounded intervals for $a \le b$:

$$[a, b] = \lbrace{}x \in \mathbb{R} : a \le x \le b\rbrace, \qquad (a, b) = \lbrace{}x \in \mathbb{R} : a < x < b\rbrace.$$

An element $M$ is an **upper bound** of $S \subset \mathbb{R}$ if $x \le M$ for all $x \in S$. The **supremum**
$\sup S$ is the least upper bound: it satisfies $x \le \sup S$ for all $x \in S$, and for any $\epsilon > 0$,
there exists $s \in S$ such that $s > \sup S - \epsilon$. Symmetrically, the **infimum** $\inf S$ is the
greatest lower bound.

Neither the supremum nor the infimum need be an element of the set. For the open interval $(0, 1)$,
$\sup (0, 1) = 1 \notin (0, 1)$ and $\inf (0, 1) = 0 \notin (0, 1)$. When an institution states that a
failure probability is bounded above by $\epsilon$, it guarantees that no realized state exceeds
$\epsilon$, even as empirical conditions approach that ceiling arbitrarily closely. Across finite safeguards
$(S\_{\mathrm{epi}}, S\_{\mathrm{vital}}, S\_{\mathrm{gov}}) \in [0, 1]^3$, the dual
greatest-lower-bound operator
$M\_{\mathrm{Leontief}} = \min(S\_{\mathrm{epi}}, S\_{\mathrm{vital}}, S\_{\mathrm{gov}})$
(Leontief 1941) satisfies two boundary cases: $S\_1 = S\_2 = S\_3 = s \implies M\_{\mathrm{Leontief}} = M\_{\mathrm{arith}} = s$
(unanimity) and $\min\_i S\_i \to 0 \implies M\_{\mathrm{Leontief}} \to 0$ (non-compensatory collapse even
when the remaining two axes sit at near-unanimity).

## 3. Interval Arithmetic and Partial Identification

A labor ministry runs a job-training trial in which a substantial share of participants cannot be located
for the follow-up earnings survey. A conventional analyst drops the missing rows, averages the completers,
and presents that point to Parliament as the program's effect, assuming without saying so that unreachable
dropouts earned the same wage as respondents. Interval arithmetic refuses the assumption: it propagates the
full range of what the missing observations could have been.

Where a parameter cannot be known with point certainty, an interval carries the propagation guarantee.
Under interval arithmetic, for $X \in [L\_X, U\_X]$ and $Y \in [L\_Y, U\_Y]$:

$$[L\_X, U\_X] + [L\_Y, U\_Y] = [L\_X + L\_Y, U\_X + U\_Y],$$

$$[L\_X, U\_X] - [L\_Y, U\_Y] = [L\_X - U\_Y, U\_X - L\_Y].$$

Watch the subtraction. Uncertainty widens under difference; it does not cancel. A point estimate hides
that accumulation of ignorance, and interval arithmetic keeps it.

Charles Manski (1990) showed in causal inference and econometrics that missing counterfactual data yields
worst-case **partial identification bounds**, not point parameters. Observe an outcome
$Y \in [0, 1]$ only for treated units ($D=1$), with selection rate $p = \mathbb{P}(D=1)$. Without
uncheckable assumptions, the expected treated outcome $\mathbb{E}[Y(1)]$ is bounded by:

$$\mathbb{E}[Y(1)] \in \bigl[p \mathbb{E}[Y \mid D=1],\mkern5mu p \mathbb{E}[Y \mid D=1] + (1 - p)\bigr].$$

The width is $1 - p$, and no increase in sample size $n$ shrinks it to zero without structural
assumptions about the missing counterfactuals. Reporting the interval $[L, U]$ says what the evidence
actually constrains.

Two-arm program evaluation over an outcome bounded in $[y\_{\min}, y\_{\max}]$: let arm
$d \in \lbrace0, 1\rbrace$ have response rate $p\_d \in (0, 1]$ and respondent mean
$\bar{Y}\_d \in [y\_{\min}, y\_{\max}]$. By interval subtraction, the Average Treatment Effect
$\mathrm{ATE} = \mathbb{E}[Y(1)] - \mathbb{E}[Y(0)]$ lies in
$[L\_{\mathrm{ATE}}, U\_{\mathrm{ATE}}] = [L\_1 - U\_0,\mkern5mu U\_1 - L\_0]$, where
$L\_d = p\_d \bar{Y}\_d + (1 - p\_d)y\_{\min}$ and $U\_d = p\_d \bar{Y}\_d + (1 - p\_d)y\_{\max}$,
with exact identification width:

$$\Delta\_{\mathrm{id}} = U\_{\mathrm{ATE}} - L\_{\mathrm{ATE}} = \bigl((1 - p\_1) + (1 - p\_0)\bigr)(y\_{\max} - y\_{\min}) \ge 0.$$

The National Supported Work trial and its population comparison sample give two cases (LaLonde 1986; Dehejia & Wahba 1999):

1. **Zero attrition ($p\_1 = p\_0 = 1$):** $\Delta\_{\mathrm{id}} = 0$, and the identification interval collapses
   to the experimental point difference, while the naive observational difference against the comparison
   sample returns a large negative figure for the same programme.
2. **Bounded attrition ($p\_d < 1$) on a bounded support $[y\_{\min}, y\_{\max}]$:** the worst-case Manski
   bounds widen by $((1 - p\_1) + (1 - p\_0))(y\_{\max} - y\_{\min})$ against the zero-attrition case, and
   eliminating the attrition of one arm alone removes that arm's share of the widening, in proportion to
   $(1 - p\_d)(y\_{\max} - y\_{\min})$. David S. Lee's (2009) monotone selection assumption narrows the
   interval further, because monotone selection bounds the unobserved mean for the always-reporting
   subpopulation by trimming the upper and lower tails instead of inserting support extrema, and the
   gain depends on how much of the outcome support
   monotonicity excludes. To separate identification width $\Delta\_{\mathrm{id}}$ from finite-sample noise
   $\mathrm{SE}\_{\mathrm{ATE}}$, the critical value solves
   $\Phi(c\_\alpha + \Delta\_{\mathrm{id}}/\hat{\sigma}\_{\max}) - \Phi(-c\_\alpha) = 1 - \alpha$
   (Imbens & Manski 2004), so the confidence interval around the bounds is wider than the nominal one by the
   identification width itself.

## 4. Heavy Tails, Extremes, and the Tail Index

A property reinsurer prices catastrophe treaties from a decade of fire claims, assuming losses decay
exponentially like Gaussian measurement errors. Under an exponential tail a claim ten times the historical
mean is negligible. Under a power-law tail that single fire is a regular structural feature, and it wipes
out the treaty premium. The tail index $\alpha$ is the boundary parameter deciding whether sample variances
and standard errors exist at all.

Bounding a real-valued process gets acute when the distribution is heavy-tailed. A distribution is
**subexponential** when its tail decays more slowly than any exponential. Pareto-type power laws are the
standard heavy-tailed family:

$$\mathbb{P}(X > x) = L(x) x^{-\alpha}, \qquad \text{as } x \to \infty,$$

where $\alpha > 0$ is the Pareto tail index and $L(x)$ is a slowly varying function ($\lim\_{t\to\infty} L(tx)/L(t) = 1$).
Moment existence follows from $\alpha$:

- For $\alpha \le 1$, the theoretical mean $\mathbb{E}[X]$ is infinite.
- For $1 < \alpha \le 2$, the mean is finite but the population variance $\mathrm{Var}(X)$ is infinite,
  invalidating the classical Central Limit Theorem.
- For $\alpha > 2$, both mean and variance are finite, yet extreme outliers continue to dominate sample sums.

The Pickands–Balkema–de Haan theorem (1974/1975) shows that for a broad class of distributions the
conditional distribution of exceedances over a sufficiently high threshold $u$, $X - u \mid X > u$,
converges to the Generalized Pareto Distribution (GPD):

$$G\_\xi(y) = 1 - \left(1 + \frac{\xi y}{\sigma}\right)^{-1/\xi},$$

where $\xi = 1/\alpha$ is the shape parameter. For $\xi > 0$, the heavy-tail case, the tail has no upper
bound and sample maxima grow polynomially with sample size. The tail index is estimated from the top $k$
order statistics $X\_{(n)} \ge X\_{(n-1)} \ge \dots \ge X\_{(n-k+1)} > 0$, measured against the
threshold just below them, $X\_{(n-k)}$, through Bruce M. Hill's (1975) maximum-likelihood estimator
$\hat{\alpha}\_{k,n} = \bigl(\frac{1}{k}\sum\_{i=1}^k \ln(X\_{(n-i+1)} / X\_{(n-k)})\bigr)^{-1}$,
with asymptotic $95\unicode{x25}$ confidence interval $\hat{\alpha}\_{k,n}(1 \pm 1.96/\sqrt{k})$ (Hill 1975;
Embrechts, Klüppelberg & Mikosch 1997). The estimate matters only through its interval. That interval decides
a borderline case when it straddles the moment boundary: a confidence set lying strictly below $2$ certifies
that the population variance is infinite, one lying strictly above $2$ certifies that it is finite, and one
that contains $2$ certifies neither, so the reserve must be sized as though the variance did not exist. As
quoted the interval is centred on the estimator and does not discount the anchor's bias, so at small $k$ it
is displaced along with the point it brackets; §3.2 of [Distribution and Moment](31-distribution-and-moment.md)
derives the displacement.

## 5. Intervals as Honest Answers

A parliamentary appropriations committee reviews the job-training evidence. It accepts an identification
interval that straddles zero and rejects a consultant's smoothed point claim that sits inside it. That is
mathematical discipline, not analytical timidity. A bound $[L, U]$ in place of a point estimate shifts the
logic of institutional judgment:

1. **Bounds that say nothing**: if $[L, U]$ spans the entire feasible range, the data do not contain the
   answer. That is a rigorous empirical finding about the data, not a failure of analysis.
2. **A decision that survives the interval**: if an action $a^\ast$ remains optimal for every
   $\theta \in [L, U]$, the decision is justified across the bound without further measurement.
3. **Worst-case survival sizing**: size vital reserves against the upper bound $U$, so that the system does
   not cross an irreversible ruin barrier on heavy-tailed tail draws.

Knowing when an interval reverses a decision, and when it leaves the decision unchanged, matters as much
as the interval. A claim clears severe testing only when the lower confidence bound strictly exceeds the
minimum relevant effect threshold $\tau$ (Mayo 2018). If the interval straddles $\tau$, the bound blocks
the premature approval that the point estimate would have rubber-stamped. If the entire interval lies below
$\tau$, interval and point estimate agree on rejection, and spending resources to narrow $[L, U]$ further
buys nothing. Three cases exhaust the decision, interval above $\tau$, interval below $\tau$, interval
straddling $\tau$, and only the last one leaves work to do.

### What an interval buys

The observational comparison says the training programme destroyed earnings; the experimental estimate
says it raised them; both cannot describe the same effect. The negative figure lies outside every admissible
identification interval, while the experimental figure and the consultant's figure both lie inside one, so
reporting them is a choice of story and not arithmetic. Reporting the interval is the only answer whose
width can be audited, and the width is what a decision can be tested against. The bound families are not ad
hoc: worst-case bounds under attrition go back to the partial-identification programme (Manski 1990), the
monotone selection version is a later refinement (Lee 2009), and the confidence interval around the bounds
keeps sampling error from being read as identification (Imbens & Manski 2004).

### When the interval replaces the score

The thesis fails when the identification width exceeds the tolerance of the decision. With $U-L >
\Delta\tau$, publish the width, refuse the point decision, name the design change that would narrow it, and
suspend the decision until that change exists. Picking a number inside the interval is the one move the
width forbids.

## Bridge

[Distribution and Moment](31-distribution-and-moment.md) and [The Non-Identical, Scale Invariance, Convexity, and Heavy Tails](46-non-identical-asymmetry-heavy-tails.md)
operationalize these bounds on empirical data. The Pareto tail index $\hat{\alpha}$ is estimated alongside
its asymptotic $95\unicode{x25}$ confidence bounds, non-parametric Manski worst-case bounds are constructed over
unobserved outcomes, and a bound that comes back degenerate $[0, 0]$ or inverted $L > U$ is refused rather
than read as evidence, because an interval that cannot be read as a bound is not one.

[Sequences, Limits, and Continuity](11-sequences-limits-continuity.md) takes the next step, asking what it
means for a record to converge, and why continuity of a loss is not the same as robustness of a decision.
