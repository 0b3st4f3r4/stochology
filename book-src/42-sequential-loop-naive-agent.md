# The Sequential Loop and the Naive Optimizer

<figure class="stoch-figure stoch-logo-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="800" height="132" viewBox="0 0 800 132" fill="none" role="img" aria-label="Stochology CH42 — The Sequential Loop and the Naive Optimizer">
  <defs>
    <linearGradient id="ch42-ring" x1="2" y1="78" x2="386" y2="78" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="38%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="68%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <radialGradient id="ch42-glow" cx="50%" cy="50%" r="50%">
      <stop offset="0%" stop-color="#fbbf24" stop-opacity="0.24"/>
      <stop offset="48%" stop-color="#38bdf8" stop-opacity="0.14"/>
      <stop offset="80%" stop-color="#e11d48" stop-opacity="0.08"/>
      <stop offset="100%" stop-color="#090e1a" stop-opacity="0"/>
    </radialGradient>
    <linearGradient id="ch42-triad-stroke" x1="18" y1="24" x2="110" y2="104" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.85"/>
      <stop offset="50%" stop-color="var(--logo-gold, #f59e0b)" stop-opacity="0.90"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.85"/>
    </linearGradient>
    <linearGradient id="ch42-fill" x1="22" y1="30" x2="106" y2="98" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)" stop-opacity="0.26"/>
      <stop offset="48%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.28"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.26"/>
    </linearGradient>
    <linearGradient id="ch42-ink-curve" x1="20" y1="66" x2="108" y2="66" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-ink, #0f172a)"/>
      <stop offset="50%" stop-color="var(--logo-sub, #1d4ed8)"/>
      <stop offset="100%" stop-color="var(--logo-ink, #0f172a)"/>
    </linearGradient>
    <radialGradient id="ch42-pupil" cx="45%" cy="42%" r="58%">
      <stop offset="0%" stop-color="#020617"/>
      <stop offset="68%" stop-color="#0f172a"/>
      <stop offset="100%" stop-color="#451a03"/>
    </radialGradient>
    <linearGradient id="ch42-gold-mira" x1="54" y1="48" x2="84" y2="82" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#fef08a"/>
      <stop offset="45%" stop-color="#fbbf24"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <radialGradient id="ch42-gold-core" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#ffffff"/>
      <stop offset="35%" stop-color="#fde047"/>
      <stop offset="100%" stop-color="#d97706"/>
    </radialGradient>
    <radialGradient id="ch42-node-well" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="var(--logo-well-hi, var(--elevated-bg, #ffffff))"/>
      <stop offset="100%" stop-color="var(--logo-node-bg, #e2e8f0)"/>
    </radialGradient>
    <linearGradient id="ch42-ring-epi" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#7dd3fc"/>
      <stop offset="50%" stop-color="#2563eb"/>
      <stop offset="100%" stop-color="#1e3a8a"/>
    </linearGradient>
    <radialGradient id="ch42-core-epi" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#bae6fd"/>
      <stop offset="45%" stop-color="#3b82f6"/>
      <stop offset="100%" stop-color="#1d4ed8"/>
    </radialGradient>
    <linearGradient id="ch42-ring-vital" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#6ee7b7"/>
      <stop offset="50%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#065f46"/>
    </linearGradient>
    <radialGradient id="ch42-core-vital" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#a7f3d0"/>
      <stop offset="45%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#047857"/>
    </radialGradient>
    <linearGradient id="ch42-ring-gov" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#fda4af"/>
      <stop offset="50%" stop-color="#e11d48"/>
      <stop offset="100%" stop-color="#881337"/>
    </linearGradient>
    <radialGradient id="ch42-core-gov" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#fecdd3"/>
      <stop offset="45%" stop-color="#f43f5e"/>
      <stop offset="100%" stop-color="#be123c"/>
    </radialGradient>
  </defs>
  <g>
    <circle cx="64" cy="66" r="48" fill="url(#ch42-glow)"/>
    <line x1="20" y1="98" x2="108" y2="98" stroke="var(--logo-crimson, #e11d48)" stroke-width="2.6" stroke-linecap="round"/>
    <path d="M 22 68 L 36 98" stroke="var(--logo-crimson, #e11d48)" stroke-width="2.6" stroke-linecap="round"/>
    <path d="M 22 68 C 40 64, 54 76, 68 98" stroke="var(--logo-crimson, #e11d48)" stroke-width="2.2" stroke-dasharray="3.5 2.5" stroke-linecap="round"/>
    <path d="M 22 68 C 46 72, 74 78, 104 82" stroke="var(--logo-blue, #2563eb)" stroke-width="2.4" stroke-dasharray="4 2.5" stroke-linecap="round"/>
    <path d="M 22 68 C 46 58, 74 44, 104 34" stroke="var(--logo-green, #10b981)" stroke-width="3.2" stroke-linecap="round"/>
    <circle cx="22" cy="68" r="6.6" fill="url(#ch42-node-well)" stroke="url(#ch42-ring-epi)" stroke-width="2.4"/>
    <circle cx="22" cy="68" r="2.8" fill="url(#ch42-core-epi)"/>
    <circle cx="104" cy="34" r="6.6" fill="url(#ch42-node-well)" stroke="url(#ch42-ring-vital)" stroke-width="2.4"/>
    <circle cx="104" cy="34" r="2.8" fill="url(#ch42-core-vital)"/>
    <g>
      <circle cx="64" cy="66" r="7.0" fill="var(--logo-node-bg, #e2e8f0)"/>
      <circle cx="64" cy="66" r="6.0" fill="url(#ch42-pupil)" stroke="url(#ch42-gold-mira)" stroke-width="2.2"/>
      <line x1="55.0" y1="66" x2="60.0" y2="66" stroke="url(#ch42-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
      <line x1="68.0" y1="66" x2="73.0" y2="66" stroke="url(#ch42-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
      <line x1="64" y1="57.0" x2="64" y2="62.0" stroke="url(#ch42-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
      <line x1="64" y1="70.0" x2="64" y2="75.0" stroke="url(#ch42-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
      <circle cx="64" cy="66" r="2.0" fill="url(#ch42-gold-core)"/>
    </g>
  </g>
  <g transform="translate(148, 0)">
    <text x="2" y="27" fill="var(--logo-sub, #1d4ed8)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.2">PART IV · THE FRAMEWORK</text>
    <text x="498" y="27" text-anchor="end" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.0">CH42</text>
    <text x="0" y="59" fill="var(--logo-ink, #0f172a)" font-family="system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif" font-size="17.2" font-weight="800" letter-spacing="0.7">THE SEQUENTIAL LOOP AND THE NAIVE OPTIMIZER</text>
    <line x1="2" y1="78" x2="384" y2="78" stroke="url(#ch42-ring)" stroke-width="1.6" stroke-opacity="0.82"/>
    <circle cx="400" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.0"/>
    <circle cx="400" cy="78" r="1.6" fill="var(--logo-blue, #2563eb)"/>
    <circle cx="415" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="430" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="445" cy="78" r="6.2" stroke="var(--logo-gold, #f59e0b)" stroke-width="1.5" stroke-dasharray="2.2 1.6"/>
    <circle cx="445" cy="78" r="4.0" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-green, #10b981)" stroke-width="2.2"/>
    <circle cx="445" cy="78" r="1.8" fill="var(--logo-gold, #f59e0b)"/>
    <circle cx="460" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="475" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="490" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-crimson, #e11d48)" stroke-width="2.0"/>
    <circle cx="490" cy="78" r="1.6" fill="var(--logo-crimson, #e11d48)"/>
    <text x="2" y="104" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.2" font-weight="600" letter-spacing="0.55">THE COMMUNITY AND THE RESERVE · NAIVE VS. TRIADIC AGENT · ABSORBING RUIN BARRIER</text>
  </g>
  </svg>
</figure>

## By the end of this chapter you can

1. Name the four structural realities that separate this loop from a textbook Markov decision process, and say which part of the mediation circuit each one breaks.
2. Decide from the terminal reserve and the ruin step alone whether a run survived the horizon or terminated in absorbing ruin.
3. Say why the naive point optimizer loses all three margins at once, and which axis its failure enters through first.

In Part III, we built the three-plane mediation circuit and the Nuclear Triad from static principles. Now, in Part IV, we run that framework step by step on an unfolding trajectory.

In textbook Reinforcement Learning (Sutton & Barto 2018), an agent acts across steps $t = 0, 1, \dots, T-1$. It observes a state, picks an action, and collects scalar rewards across episodes that reset cleanly whenever they end.

Move from a board game to a hospital emergency network, water reserve, credit guarantee fund, or evacuation system, and that clean loop breaks. **Four structural realities** disrupt the three-plane mediation circuit of [Chapter 29, *Aim and the Three Planes*](29-aim-and-the-three-planes.md):

1. **The Archive is Mutilated ($P\_1 \to P\_2$, *Observe* $T\_3$):** The decider never sees true latent demand directly. It sees only a clustered archive $O\_t$ that systematically misrecords vulnerable households ($\rho\_{R,Y} \ne 0$, meaning selection correlation links whether a unit is recorded to how severe its need is—under-recording vulnerable households whenever $\rho\_{R,Y} < 0$).
2. **Harm is Convex and Tails are Heavy ($P\_2 \to P\_3$, *Formalize* $T\_4$, and $P\_1 \to P\_3$, *Temporal* $T\_5$):** Even with finite-variance arrivals and service (Pareto tail exponent $\alpha > 2$), running near capacity drives queue delays upward. As expected utilization $\rho\_t = \mathbb{E}[S\_t]/C\_t \in [0, 1)$ approaches $1^-$, Kingman's heavy-traffic law scales waiting time like $\frac{\rho\_t}{1 - \rho\_t}$ (Kingman 1961, 1962). Under power-law surges ($\alpha \le 2$), arrival variance explodes beyond stationary queue bounds. Realized overload ($S\_t > C\_t$) then turns unmet demand into steep convex harm ([Chapter 46, *The Non-Identical, Scale Invariance, Convexity, and Heavy Tails*](46-non-identical-asymmetry-heavy-tails.md); [Chapter 47, *Irreversible Time, Non-Ergodicity, and the Vital Margin*](47-irreversible-time-vital-margin.md)).
3. **The World Reacts Performatively ($P\_3 \to P\_1$, *Perform* $T\_6$):** High-powered proxy targets ($\beta\_{\mathrm{proxy}} \to 1$, meaning intense administrative pressure to hit a measured score) induce gaming. Workers and managers hit the published metric by cannibalizing unmeasured care and future capacity (Holmström & Milgrom 1991; Holmström 2016).
4. **Time is Irreversible and Ruin is Absorbing (*Temporal* $T\_5$ and integral *Liability* $T\_7$):** The community relies on a finite vital reserve $R\_t \ge 0$. Suppose excess leverage or unbuffered overload drains that reserve to $R\_{t+1} \le 0$ before recovery finishes. Recovery requires detection, deliberation—slowed by status discounting of frontline warnings and limited human attention ([Chapter 49](49-moral-luck-apt-agency-institutions.md))—and physical actuation. Whenever time-to-survival falls short of time-to-recovery ($\mathrm{TTS}\_{\mathrm{survival}} < \mathrm{TTR}\_{\mathrm{recovery}}$), the system hits an absorbing ruin barrier.

![The Sequential Agent–World–Institution Loop](figures/fig-01-sequential-loop.svg)

Across this loop, per-step actions $A\_t$ come strictly from the canonical operators of the three filters under ex-ante constitutional rules ([Chapter 41, *The Triad and the Minimum*](41-the-triad-and-the-minimum.md)). What the diagram marks as *ex post*—the weakest-axis score $M\_{\mathrm{Leontief}}$, the four reflexive Goodhart brakes ([Chapter 50, *Five Methodological Families, Process Tracing, and Reflexive Self-Application*](50-methodological-families-reflexivity.md)), the co-diagnostic pair $\mathcal{D}\_{\mathrm{diag}} = \bigl(M\_{\mathrm{Leontief}}, \Theta\_{\mathrm{basin}}\bigr)$ (where $\Theta\_{\mathrm{basin}}$ scores harm spilled across outside reception perimeters), and $(S\_{\mathrm{epi}}, S\_{\mathrm{vital}}, S\_{\mathrm{gov}})$—folds strictly retrospectively to report what failed, never to steer live control.

---

## 1.1 The Evolving Environment: "The Community and the Reserve"

To test every safeguard on one concrete stage, Part IV tracks a single institutional ledger across $T \ge 1$ discrete shifts before scaling to multi-appropriator commons. A community of $N \ge 1$ household units spans both well-served and high-vulnerability clusters. Nothing in the framework hinges on exact cluster counts—only on systematic inequality across strata.

- **Latent Community Demand ($S\_t$):** At shift $t$, each unit $i \in \lbrace 1, \dots, N \rbrace$ carries true need $Y\_{t,i} \ge 0$. The true population mean is $\bar{Y}\_t = \frac{1}{N}\sum\_{i=1}^N Y\_{t,i}$, and aggregate community demand is $S\_t = \sum\_{i=1}^N Y\_{t,i}$. Vulnerable clusters carry higher baseline need and heavy-tailed Pareto surge exposure ($\alpha \le 2$).
- **Selective Archive Recording ($O\_t$):** Write $\mathbf{I}\_t^{\mathrm{rec}} \in \lbrace 0, 1 \rbrace^N$ for whether each household is recorded (kept distinct from physical reserve $R\_t \ge 0$). Low capture in vulnerable neighborhoods creates negative selection correlation ($\rho\_{R,Y} < 0$, where $R \in \lbrace 0, 1\rbrace$ denotes Meng's (2018) recording indicator). That negative correlation pulls the raw observed mean $\bar{O}\_t$ below true mean $\bar{Y}\_t$ (whereas $\rho\_{R,Y} > 0$ reverses the displacement upward), while clustering inside neighborhoods inflates sampling variance.
- **Operational Action ($A\_t$) vs. Constitutional Rules:** On each shift, the operating policy chooses an epistemic interval $[L\_t, U\_t]$, a variance-effective sample size $n\_{\mathrm{eff}}^{\mathrm{var}}$ (raw sample size deflated for clustering and weights), and a selection sensitivity bound $\bar{\rho} \ge |\rho\_{R,Y}|$. That sensitivity bound fixes the bias-adjusted evidential count $n\_{\mathrm{eff}}(\bar{\rho})$. The operator also selects physical capacity $C\_t$, vital buffer $B\_t$, and leverage fraction $f\_t$.
- **External Constitutional Safeguards:** By contrast, proxy pressure $\beta\_{\mathrm{proxy}}$, status discounting of warnings ($\delta\_{\mathrm{status}}$), and direct sporadic operational safety audits (Beer's System $3^\ast$, written $S3^\ast$ and kept distinct from Triad scores $S\_a$ and demand $S\_t$) sit at the constitutional level above turn-by-turn discretion ([Chapter 40, *Institution, Incentive, Metric*](40-institution-incentive-metric.md); [Chapter 41, *The Triad and the Minimum*](41-the-triad-and-the-minimum.md); [Chapter 49, *Moral Luck, Apt Agency, and Institutional Design*](49-moral-luck-apt-agency-institutions.md)). Self-imposed safety rules fail precisely when cutting corners looks profitable.

### 1.1.1 State-Space Dynamics of the Triadic Mediation Loop

In plain English, each shift moves through four coupled updates linking World ($P\_1$), Archive ($P\_2$), and Institution ($P\_3$). Need shifts, records filter what is seen, buffers absorb shortfalls, and human caregivers accumulate fatigue:

1. **Latent World Transition:** $\mathbf{Y}\_{t+1} \sim \mathcal{T}(\mathbf{Y}\_t, A\_t, \boldsymbol{\omega}\_t)$. Exogenous cluster shocks $\boldsymbol{\omega}\_t \sim \mathcal{D}\_{\mathrm{shock}}$ combine with performative erosion caused by prior policies and proxy incentives.
2. **Selective Archive Filtering:** $O\_t \sim \mathcal{M}(\mathbf{Y}\_t, \mathbf{I}\_t^{\mathrm{rec}})$. Whenever selection correlation $\rho\_{R,Y} < 0$, the observed mean $\bar{O}\_t$ remains systematically displaced downward from $\bar{Y}\_t$ (whereas positive $\rho\_{R,Y} > 0$ displaces $\bar{O}\_t$ upward above $\bar{Y}\_t$).
3. **Vital Reserve Evolution:** $R\_{t+1} = R\_t + \Delta R\_t(A\_t, S\_t)$. Net drift $\Delta R\_t$ balances replenishment and fixed baseline maintenance against emergency spot-procurement costs during overload ($S\_t > C\_t$) and queue congestion penalties.
4. **Operator and Caregiver Allostatic Wear Accumulation:** Starting from $W\_{\mathrm{allostatic}, 0} = 0$, shift workload $\Delta W\_{\mathrm{load}, t} \ge 0$ and recovery rest $\Delta W\_{\mathrm{rest}, t} \ge 0$ update caregiver strain via
   $$W\_{\mathrm{allostatic}, t+1} = \max\bigl(0,\mkern3mu W\_{\mathrm{allostatic}, t} + \Delta W\_{\mathrm{load}, t} - \Delta W\_{\mathrm{rest}, t}\bigr).$$
   In plain words, chronic strain is tracked in raw caregiver fatigue units prior to $[0, 1]$ triad normalization—separate from physical reserve stock $R\_t$ and channel throughput. That accumulated wear shrinks remaining caregiver endurance $S\_{\mathrm{vital}, t}^{\mathrm{care}} = \max(0,\mkern3mu R\_0^{\mathrm{care}} - W\_{\mathrm{allostatic}, t})$ from baseline endurance ceiling $R\_0^{\mathrm{care}} > 0$ (distinct from physical reserve $R\_t$), even when financial ledgers still look healthy. Under unbroken shifts with zero recovery ($\Delta W\_{\mathrm{rest}, t} = 0$, writing $\psi\_t := \Delta W\_{\mathrm{load}, t}$), wear reduces for $t \ge 1$ to the cumulative sum $W\_{\mathrm{allostatic}, t} = \sum\_{s=0}^{t-1} \psi\_s$ ([Chapter 99](99-unsealing-the-vault-after-impact.md)), while rest unwinding follows [Chapter 47](47-irreversible-time-vital-margin.md).

### 1.1.2 Symmetric Convex Loss vs. Unilateral Infinite Absorbing Ruin

Why do textbook decision rules break on physical reserves? Standard reinforcement learning and econometrics typically judge error using symmetric or unconstrained convex penalties. Writing $\mathcal{L}\_{\mathrm{sym}}$ for quadratic loss and $\mathcal{L}\_{\mathrm{lin}}$ for absolute error between capacity $C\_t$ and demand $S\_t$, the textbook benchmark computes:
$$\mathcal{L}\_{\mathrm{sym}}(C\_t, S\_t) = \frac{1}{2}(C\_t - S\_t)^2 \quad \text{or} \quad \mathcal{L}\_{\mathrm{lin}}(C\_t, S\_t) = |C\_t - S\_t|$$

In plain words, both formulas treat being under-prepared ($C\_t < S\_t$) and holding spare stock ($C\_t > S\_t$) as ordinary finite costs. Across resettable game episodes, no absorbing wall stops tomorrow's surpluses from paying off today's shortfalls on average.

Physical reserves obey an asymmetric boundary: crossing zero destroys the system permanently. Above zero ($R\_{t+1} > 0$), unmet demand incurs a finite convex, non-decreasing shortfall penalty $\phi\bigl((S\_t - C\_t)\_+\bigr)$. At or below the **absorbing ruin half-line** $R\_{t+1} \le 0$, vital loss $\mathcal{L}\_{\mathrm{vital}}$ jumps across a unilateral infinite barrier:
$$\mathcal{L}\_{\mathrm{vital}}(R\_{t+1}, C\_t, S\_t) = \begin{cases} \phi\bigl((S\_t - C\_t)\_+\bigr) & \text{if } R\_{t+1} > 0 \\\\ +\infty & \text{if } R\_{t+1} \le 0 \end{cases}$$

In plain words, hitting or crossing zero is not a steep fine—it is game over. Bankruptcy, stockout, or clinical shutdown forces future capacity to zero ($C\_\tau = 0$) for every later shift $\tau > t$. No stream of expected future surpluses can resurrect a collapsed reserve. Sizing lean capacity to an underestimated average therefore guarantees that an unbuffered tail shock drives $R\_{t+1} \le 0$.

---

## 1.2 Why the Point Optimizer Collapses

Consider the benchmark every practitioner reaches for first: a **naive point optimizer**. Its arithmetic is flawless, yet its assumptions fail at every link across the three-plane circuit:

- **Epistemic blindness:** It treats the archive as independent and missing completely at random, ignoring non-random recording ($\rho\_{R,Y} \ne 0$; Meng 2018) and cluster design effects (Kish 1965; Moulton 1986).
- **Vital over-commitment:** It provisions lean capacity around that distorted mean and bets an unshrunk plug-in Kelly fraction unsupported by finite samples (Kelly 1956; Baker & McHale 2013).
- **Governance capture:** It imposes high proxy pressure ($\beta\_{\mathrm{proxy}} \to 1$) that cannibalizes unmeasured care (Holmström & Milgrom 1991; Holmström 2016) and discounts frontline algedonic alarms ($\delta\_{\mathrm{status}} > 0$) up the hierarchy (Beer 1972).

It breaks all three axes of the Nuclear Triad simultaneously. What makes that bundle lethal is the absorbing ruin barrier of §1.1.2: a policy that misreads true demand *and* holds no physical slack gets no second shift in which to learn. [Chapter 43](43-inquiry-openness-multiverse.md) through [Chapter 50](50-methodological-families-reflexivity.md) construct the three safeguards in sequence because the naive rule fails across all three—and its collapse surfaces first through the **Epistemic Margin** ($S\_{\mathrm{epi}}$).

---

## 1.3 The Naive Rule Against the Triadic Agent

To see the difference in practice, trace both policies across the exact same sequence of demand shocks. The naive point rule and the intact triadic agent diverge in three stages: first in interval coverage, next in realized harm, and finally in whether the run avoids absorbing ruin.

### Where the naive interval fails

A naive confidence interval centered on the raw mean $\bar{O}\_t$ misses true demand for two compounding reasons. First, negative selection correlation ($\rho\_{R,Y} < 0$) shifts the center downward (whereas positive $\rho\_{R,Y} > 0$ displaces it upward). Second, clustering inflates variance by a design effect $\mathrm{DEFF} > 1$, so naive standard errors look far narrower than they are. Adding more rows through the same biased filter only tightens a misleading interval around the wrong number. Once a surge pushes demand past lean capacity ($S\_t > C\_t$) or spikes queue delays, emergency spot costs drain $R\_t$ across zero and lock $C\_\tau = 0$ for the rest of the run.

### How the intact triadic agent `(1, 1, 1)` answers

The intact triadic agent guards all three axes at every step:

- **Epistemic Margin ($S\_{\mathrm{epi}}$):** It adopts Manski's (1989, 2003, 2026) partial-identification bounds $[L\_t, U\_t]$ instead of a fabricated point guess. It replaces impossible point corrections for selection correlation $\rho\_{R,Y}$ with an explicit sensitivity bound $|\rho\_{R,Y}| \le \bar{\rho}$ ([Chapter 45, *From World to Archive: Evidential Weight, Big Data Paradox, and the Uncounted*](45-world-to-archive-big-data-paradox.md)). It also scales sampling variance by $\mathrm{DEFF} \ge 1$—yielding variance-effective sample size $n\_{\mathrm{eff}}^{\mathrm{var}} = n/\mathrm{DEFF}$, distinct from the bias-adjusted Keynesian count $n\_{\mathrm{eff}}(\bar{\rho})$.
- **Vital Margin ($S\_{\mathrm{vital}}$):** It prices convex shortfall, keeps routine utilization below Kingman's hyperbolic wall, and holds buffers for heavy-tailed surges ($\alpha \le 2$). It shrinks Kelly leverage using $n\_{\mathrm{eff}}^{\mathrm{var}}$ and guards the Canguilhem–Holling resilience basin (Canguilhem 1943/1966; Holling 1973) rather than optimizing around a fragile mean.
- **Institutional Governance ($S\_{\mathrm{gov}}$):** It enforces constitutional bounds above turn-level discretion. It keeps proxy incentives low-powered (Holmström 2016) and preserves undiscounted frontline warning channels ($\delta\_{\mathrm{status}} = 0$ on Beer's System $3^\ast$ sporadic audit $S3^\ast$, distinct from Triad axis scores $S\_a$).

Under positive net replenishment and zero marginal idle-capacity charges, `(1, 1, 1)` covers true demand, cuts mean harm, and keeps reserves positive across buffered shocks. Still, at any finite reserve level, unbounded tail shocks retain non-zero ruin probability ([Chapter 46](46-non-identical-asymmetry-heavy-tails.md); [Chapter 47](47-irreversible-time-vital-margin.md)).

### Syntax checks and interpersonal scale

Syntactically ([Appendix B](appendix-b-architecture-verification.md) §B.1), any shift whose interval is missing, non-finite, or inverted ($L\_t > U\_t$) triggers `Unparsable` quarantine instead of silent averaging. A finite point interval ($L\_t = U\_t$) remains syntactically valid ([Chapter 41, *The Triad and the Minimum*](41-the-triad-and-the-minimum.md)) and passes into epistemic audit in [Chapter 43](43-inquiry-openness-multiverse.md) and [Chapter 44](44-underdetermination-partial-identification.md). At the two-person scale ([Chapter 29, *Aim and the Three Planes*](29-aim-and-the-three-planes.md)), policing how a listener receives candid speech commits the same failure: it shuts epistemic openness ($S\_{\mathrm{epi}}$), exhausts vital tolerance ($S\_{\mathrm{vital}}$), and overflows the listener's reception perimeter ($\Theta\_{\mathrm{basin}}$).

### Coverage does not order harm

Covering true demand inside an interval ($S\_t \in [L\_t, U\_t]$) is not the same as safely serving shifts ($s \in (0, 1]$), and neither metric ranks policies by total harm. Serving most shifts under captured incentives scales up harm while reporting apparent success. Internal ledger survival is settled jointly by the terminal reserve and the ruin step, while true community survival requires keeping external reception perimeters ($\Theta\_{\mathrm{basin}}$) intact.

In standard Reinforcement Learning (Sutton & Barto 2018), only discount factor $\gamma = 0$ is strictly myopic; at $\gamma \in (0, 1)$ (or $\gamma = 1$ in finite-horizon episodes), the agent maximizes cumulative return. Yet the naive optimizer fails even as $\gamma \to 1$, because risk-neutral point optimization over a distorted archive ignores absorbing ruin ($R\_{t+1} \le 0$), proxy gaming, and externalized perimeter damage.

To see why raw served shares mislead, compare two policies over horizon $T > 0$ with continuous served-demand shares $s\_A, s\_B \in (0, 1]$ (or discrete shift shares $s \ge 1/T$ in the large-$T$ limit). Let policy $A$ serve share $s\_A \in (0, 1]$ with positive harm $H\_A > 0$, and policy $B$ serve share $s\_B \in (0, 1]$ with positive harm $H\_B > 0$. Harm per served shift is $H/(sT)$. Their harm-per-shift ratio $(H\_A/H\_B)(s\_B/s\_A)$ grows without bound as $s\_A \to 0^+$ at fixed positive harms—an explosion that neither coverage nor served-share counts reveal.

### When the naive rule is the correct rule

Full precautionary buffers are mandatory when ruin is reachable or harm crosses community boundaries. By contrast, suppose actions are strictly reversible and the absorbing barrier cannot be reached under a **hard budget constraint**—meaning no implicit bailout masks deficits (`soft budget constraint`, Kornai 1986). Suppose also that third-party reception perimeters ($\Theta\_{\mathrm{basin}}$) face auditable non-export ([Chapter 29, *Aim and the Three Planes*](29-aim-and-the-three-planes.md); [Chapter 49, *Moral Luck, Apt Agency, and Institutional Design*](49-moral-luck-apt-agency-institutions.md)).

Under those narrow conditions, holding idle buffer $B\_t$ is pure waste and may be dropped, while $S\_{\mathrm{epi}}$ and $S\_{\mathrm{gov}}$ stay active to verify that absorption and spillover stay excluded—running [Chapter 41, *The Triad and the Minimum*](41-the-triad-and-the-minimum.md) in reverse rather than retreating to naive point optimization.

### Two families, not an ordering

Policies tested on one environment do not form a smooth ladder—they split into two incommensurable families. Intact triadic policies `(1, 1, 1)` within survival headroom finish the horizon with positive reserves and an empty ruin step. By contrast, ablations dropping the epistemic `(0, 1, 1)`, vital `(1, 0, 1)`, or governance `(1, 1, 0)` axis hit absorbing ruin at a finite step ([Chapter 41, *The Triad and the Minimum*](41-the-triad-and-the-minimum.md); [Chapter 50, *Five Methodological Families, Process Tracing, and Reflexive Self-Application*](50-methodological-families-reflexivity.md)). Averaging harm across surviving and ruined trajectories yields a meaningless number.

That behavioral split mirrors three institutional payoff regimes governed by whether shared rules bind above competing actors (Ostrom 2010; [Chapter 40, *Institution, Incentive, Metric*](40-institution-incentive-metric.md)):

- **`(0, 0, 0)` (Naive optimizer):** myopic extraction over shared reserves.
- **`(1, 1, 0)` (Sovereign capture):** limited-liability opportunism that weaponizes uncertainty to privatize gains while holding data and slack.
- **`(1, 1, 1)` (Polycentric commons):** self-governance that sustains shared buffers within biophysical basin limits (Ostrom 2010; Holling 1973), audited ex post through $\mathcal{D}\_{\mathrm{diag}} = \bigl(M\_{\mathrm{Leontief}}, \Theta\_{\mathrm{basin}}\bigr)$.

### What a trajectory shows that a summary cannot

Two observables decide internal survival across every path: the terminal reserve and the ruin step (an empty ruin step confirms horizon survival; a populated one marks termination regardless of nominal leftover balance, while true survival also keeps $\Theta\_{\mathrm{basin}}$ unbreached). Because uncalibrated summaries hide that split, this chapter reasons over trajectory shapes rather than printed run scores.

---

## 1.4 The barrier at both ends, and the buffer that is the drain

### Two limits of the same balance equation

Both queues and reserves obey balance equations whose danger sits near the boundary.

First, look at the **queue term**. Under finite-variance arrivals and service ($\alpha > 2$, with squared variation coefficients $c\_a^2, c\_s^2 < \infty$ and $c\_a^2 + c\_s^2 > 0$, as in the introductory list) and subcritical utilization $\rho = \mathbb{E}[S\_t]/C\_t \in [0, 1)$ (distinct from selection correlation $\rho\_{R,Y}$), mean delay scales with Kingman's multiplier $W(\rho) = \rho/(1-\rho)$ (distinct from allostatic wear $W\_{\mathrm{allostatic}, t}$). Compare buffered slack $s\_{\mathrm{buf}} = 1 - \rho\_{\mathrm{buf}}$ against lean slack $s\_{\mathrm{lean}} = 1 - \rho\_{\mathrm{lean}}$ ($0 < s\_{\mathrm{lean}} \ll s\_{\mathrm{buf}} < 1$). With $\rho < 1/2$, $W(\rho) < 1$ stays flat; toward saturation, $W(\rho\_{\mathrm{lean}})/W(\rho\_{\mathrm{buf}}) = (\rho\_{\mathrm{lean}}/\rho\_{\mathrm{buf}})(s\_{\mathrm{buf}}/s\_{\mathrm{lean}}) \to \infty$ as $s\_{\mathrm{lean}} \to 0^+$, since $\lim\_{\rho \to 1^-} W(\rho) = \infty$ with slope $W'(\rho) = (1-\rho)^{-2}$ while $W(\rho) \approx \rho$ near zero.

Second, look at the **reserve term**. Starting from endowment $R\_0 > 0$, a constant net drain $d > 0$ per shift hits zero at the first step $t$ where $R\_0 - t\mkern3mud \le 0$. That gives discrete ruin time $\tau\_{\mathrm{ruin}} = \lceil R\_0/d \rceil$. In plain words, survival time scales linearly with initial reserve $R\_0$ and inversely with drain $d$: doubling the drain halves continuous survival time $R\_0/d$, and shrinks discrete $\lceil R\_0/d \rceil$ until capacity locks at $C\_\tau = 0$.

### The regime where the buffer is the drain

Buffers are not always free. On perishable supplies or leased standby capacity—think of refrigerated medicines or contracted hospital beds—holding spare stock imposes a per-shift carrying charge that can itself exhaust the reserve across $T \ge 1$ shifts.

Consider a simple concrete setup from endowment $R\_0 > 0$. Suppose demand sits at calm baseline $\bar{S}$ for $T - 1$ shifts and hits one surge $\bar{S} + \Delta$ ($\Delta > 0$). Unmet demand costs $e > 0$ per unit (the local unit shortfall cost, distinct from Euler's constant and the sequential $e$-values in $e\text{-BH}$ below), while holding idle capacity costs a carrying charge $h > 0$ per unit per shift. Compare two rules:

- **Lean rule ($C = \bar{S}$):** It pays zero carrying cost on calm shifts and loses $e\Delta$ on the surge, surviving if and only if $R\_0 > e\Delta$.
- **Precautionary rule ($C = \bar{S} + B$, with buffer $B > 0$):** On calm shifts it pays carrying cost $hB$ every shift. Without any surge at all, carrying costs alone drain the reserve to zero at $\tau\_{\mathrm{ruin}} = \lceil R\_0/(hB) \rceil$.

Across all $T$ shifts with surge $\Delta$, the net comparison depends on whether the buffer is smaller or larger than the surge:

- When $0 < B \le \Delta$ and carrying charges apply only to idle buffer ($B$ on $T - 1$ calm shifts and $0$ on the surge), total drain is $(T - 1)hB + e(\Delta - B)$. That total exceeds the lean rule's $e\Delta$ once $(T - 1)h > e$ (or $Th > e$ if $B$ is charged across all $T$ shifts).
- When $B \ge \Delta$ (so even the surge leaves $B - \Delta$ idle), total drain $T h B - h\Delta$ crosses $e\Delta$ at $T h B = (e + h)\Delta$.

In plain words, a buffer whose carrying charges outrun the surge loss it prevents becomes a drain itself. While §1.1.1 includes only fixed baseline maintenance and omits marginal idle charges $h(C\_t - S\_t)\_+$, perishable or leased systems trigger the regressive-hoarding ceiling of Reflexive Brake 2 ([Chapter 50, *Five Methodological Families, Process Tracing, and Reflexive Self-Application*](50-methodological-families-reflexivity.md)).

### When the horizon outlives the decider

The absorbing-barrier warning fails when decision horizons end before physical ruin arrives. If a manager's tenure $T\_{\mathrm{episode}}$ is far shorter than physical recovery time ($0 < T\_{\mathrm{episode}} \ll \mathrm{TTR}\_{\mathrm{recovery}}$, as in slow aquifer depletion), terminal reserve depletion becomes a cost handed to successors.

The institutional fix sets the evaluation horizon to $T = \max\bigl(T\_{\mathrm{episode}},\mkern3mu \lceil\mathrm{TTR}\_{\mathrm{recovery}}\rceil\bigr)$ and binds choices to pre-outcome commitments that outlive tenure ([Chapter 49, *Moral Luck, Apt Agency, and Institutional Design*](49-moral-luck-apt-agency-institutions.md)). Because cryptographic seals only certify chronology from write-time onward without blocking pre-seal omissions ([Chapter 49](49-moral-luck-apt-agency-institutions.md)), cross-tenure survival also demands constitutional floors above the unit (Ostrom 2010), clawback escrow, and affected-party standing over $\Theta\_{\mathrm{basin}}$. Without enforceable cross-tenure commitment, an unconstrained short-horizon optimum yields only a **lower bound on true institutional cost** and an **upper bound on sustainable net payoff**.

## Bridge

This chapter hands forward a running loop and a failure mode. The loop links a world that emits heavy-tailed demand and suffers irreversible ruin, an archive that selects and clusters records, and an institution acting under proxy incentives and bounded attention. The naive optimizer breaks that loop from the start by treating the raw mean $\bar{O}\_t$ as an exact point measurement inside an unexamined frame.

[Inquiry, Framing, and Antidogmatic Openness](43-inquiry-openness-multiverse.md) opens that frame before any bound or correction is estimated. It enforces Cromwell's non-zero probability floor, publishes standpoint gaps in cost–loss thresholds, separates severe testing from underpowered safety claims and Type III errors, audits premise rigging, and replaces cherry-picked models with $e\text{-BH}$ multiverse curves. Once [Chapter 43, *Inquiry, Framing, and Antidogmatic Openness*](43-inquiry-openness-multiverse.md) holds inquiry open, partial-identification bounds in [Chapter 44](44-underdetermination-partial-identification.md) and world-to-archive selection weighting in [Chapter 45](45-world-to-archive-big-data-paradox.md) complete Mediating Filter I across the loop launched here.
