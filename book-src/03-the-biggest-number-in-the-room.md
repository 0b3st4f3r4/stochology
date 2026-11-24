# The Biggest Number in the Room

<figure class="stoch-figure stoch-logo-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="800" height="132" viewBox="0 0 800 132" fill="none" role="img" aria-label="Stochology CH03 · HEAVY TAILS — Light Tails And Pareto Shocks">
  <defs>
    <!-- Divider Progression: Vital Emerald -> Epistemic Sapphire -> Stókhos Gold -> Institutional Crimson -->
    <linearGradient id="ch03-ring" x1="2" y1="78" x2="386" y2="78" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="38%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="68%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <!-- Multi-spectral Atmospheric Glow -->
    <radialGradient id="ch03-glow" cx="52%" cy="52%" r="50%">
      <stop offset="0%" stop-color="#34d399" stop-opacity="0.25"/>
      <stop offset="48%" stop-color="#fbbf24" stop-opacity="0.16"/>
      <stop offset="80%" stop-color="#e11d48" stop-opacity="0.10"/>
      <stop offset="100%" stop-color="#090e1a" stop-opacity="0"/>
    </radialGradient>
    <!-- Pareto Power-Law Heavy Tail Fire Density -->
    <linearGradient id="ch03-tail-fire" x1="22" y1="92" x2="106" y2="48" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#059669" stop-opacity="0.46"/>
      <stop offset="38%" stop-color="#2563eb" stop-opacity="0.48"/>
      <stop offset="68%" stop-color="#f59e0b" stop-opacity="0.44"/>
      <stop offset="100%" stop-color="#e11d48" stop-opacity="0.52"/>
    </linearGradient>
    <!-- Pareto Tail Ink Stroke -->
    <linearGradient id="ch03-ink-curve" x1="22" y1="70" x2="106" y2="70" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-ink, #0f172a)"/>
      <stop offset="50%" stop-color="var(--logo-sub, #1d4ed8)"/>
      <stop offset="100%" stop-color="var(--logo-ink, #0f172a)"/>
    </linearGradient>
    <!-- Light-Tailed Gaussian Curve Stroke -->
    <linearGradient id="ch03-gauss-stroke" x1="22" y1="68" x2="74" y2="68" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#64748b"/>
      <stop offset="50%" stop-color="#94a3b8"/>
      <stop offset="100%" stop-color="#64748b"/>
    </linearGradient>
    <!-- 3D Spherical Node Gradients -->
    <radialGradient id="ch03-node-well" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="var(--logo-well-hi, var(--elevated-bg, #ffffff))"/>
      <stop offset="100%" stop-color="var(--logo-node-bg, #e2e8f0)"/>
    </radialGradient>
    <linearGradient id="ch03-ring-epi" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#7dd3fc"/>
      <stop offset="50%" stop-color="#2563eb"/>
      <stop offset="100%" stop-color="#1e3a8a"/>
    </linearGradient>
    <radialGradient id="ch03-core-epi" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#bae6fd"/>
      <stop offset="45%" stop-color="#3b82f6"/>
      <stop offset="100%" stop-color="#1d4ed8"/>
    </radialGradient>
    <!-- Golden Stókhos Reticle Gradients -->
    <radialGradient id="ch03-pupil" cx="45%" cy="42%" r="58%">
      <stop offset="0%" stop-color="#020617"/>
      <stop offset="68%" stop-color="#0f172a"/>
      <stop offset="100%" stop-color="#451a03"/>
    </radialGradient>
    <linearGradient id="ch03-gold-mira" x1="54" y1="48" x2="84" y2="82" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#fef08a"/>
      <stop offset="45%" stop-color="#fbbf24"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <radialGradient id="ch03-gold-core" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#ffffff"/>
      <stop offset="35%" stop-color="#fde047"/>
      <stop offset="100%" stop-color="#d97706"/>
    </radialGradient>
  </defs>
  <g>
    <!-- Atmospheric Glow & Reference Coordinate Horizon -->
    <circle cx="64" cy="66" r="48" fill="url(#ch03-glow)"/>
    <circle cx="64" cy="66" r="42" stroke="var(--logo-ticks, #64748b)" stroke-width="1.2" stroke-dasharray="3 3" stroke-opacity="0.45"/>
    <!-- Volumetric Pareto Heavy Tail Density Area -->
    <path d="M 22 92 C 30 92, 42 78, 52 70 C 66 60, 84 48, 106 48 L 106 94 L 22 94 Z" fill="url(#ch03-tail-fire)"/>
    <line x1="20" y1="94" x2="108" y2="94" stroke="var(--logo-ticks, #64748b)" stroke-width="1.4"/>
    <!-- Pareto Power Law Tail Contour -->
    <path d="M 22 92 C 30 92, 42 78, 52 70 C 66 60, 84 48, 106 48" stroke="url(#ch03-ink-curve)" stroke-width="3.2" stroke-linecap="round"/>
    <!-- Light-Tailed Gaussian Bell Curve -->
    <path d="M 22 92 C 34 92, 40 44, 48 44 C 56 44, 62 92, 74 92" stroke="url(#ch03-gauss-stroke)" stroke-width="2.2" stroke-dasharray="3.5 2.5" stroke-linecap="round"/>
    <!-- Single-Observation Catastrophe / Ruin Warning Threshold -->
    <line x1="96" y1="94" x2="96" y2="34" stroke="var(--logo-crimson, #e11d48)" stroke-width="2.2" stroke-dasharray="2.5 2" stroke-linecap="round"/>
    <!-- Light-Tailed Gaussian Expected Mean Node at (48, 44) -->
    <circle cx="48" cy="44" r="8.0" fill="url(#ch03-node-well)" stroke="url(#ch03-ring-epi)" stroke-width="3.0"/>
    <circle cx="48" cy="44" r="3.6" fill="url(#ch03-core-epi)"/>
    <!-- Heavy-Tail Ruin Horizon Stókhos Reticle at (96, 48) -->
    <g id="ch03-reticle">
      <circle cx="96" cy="48" r="7.4" fill="var(--logo-node-bg, #e2e8f0)"/>
      <circle cx="96" cy="48" r="6.4" fill="url(#ch03-pupil)" stroke="url(#ch03-gold-mira)" stroke-width="2.4"/>
      <line x1="86.4" y1="48" x2="91.8" y2="48" stroke="url(#ch03-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="100.2" y1="48" x2="105.6" y2="48" stroke="url(#ch03-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="96" y1="38.4" x2="96" y2="43.8" stroke="url(#ch03-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="96" y1="52.2" x2="96" y2="57.6" stroke="url(#ch03-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <circle cx="96" cy="48" r="2.2" fill="url(#ch03-gold-core)"/>
    </g>
  </g>
  <g transform="translate(148, 0)">
    <text x="2" y="27" fill="var(--logo-sub, #1d4ed8)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.2">PART I · THINKING UNDER UNCERTAINTY</text>
    <text x="498" y="27" text-anchor="end" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.0">CH03 · HEAVY TAILS</text>
    <text x="0" y="59" fill="var(--logo-ink, #0f172a)" font-family="system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif" font-size="22.2" font-weight="800" letter-spacing="1.35">LIGHT TAILS AND PARETO SHOCKS</text>
    <line x1="2" y1="78" x2="384" y2="78" stroke="url(#ch03-ring)" stroke-width="1.6" stroke-opacity="0.82"/>
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
    <text x="2" y="104" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.8" font-weight="600" letter-spacing="0.65">SCALE INVARIANCE · PARETO TAIL INDEX · SINGLE-OBSERVATION DOMINANCE</text>
  </g>
  </svg>
</figure>

> Intuition-first guide to heavy tails and survival slack; full proofs belong to [Distribution and Moment](31-distribution-and-moment.md), [Time and Dependence](38-time-and-dependence.md), and [Irreversible Time, Non-Ergodicity, and the Vital Margin](47-irreversible-time-vital-margin.md).

## By the end of this chapter you can

1. Say why a handful of extreme events carries the overwhelming share of the damage in a heavy-tailed system.
2. Say why "it never happened before" is weak evidence, and how much the length of the record limits what can be inferred about tail risk.
3. Explain why a physical reserve must keep the system alive longer than recovery takes, rather than matching an average day.

## 1. Where the total comes from

A river town's levee board meets every autumn to set the flood-maintenance budget. Most years the river
rises half a meter and nothing happens. Some years it rises two meters and floods the park. Once in a
generation a stationary storm stalls upstream and the river rises seven meters. Add up the damage across
a hundred years in the **World** and almost all of it comes from the one or two years that were not
ordinary.

That is the difference between a **thin tail** and a **heavy tail**. Adult human heights have a thin tail: almost everyone clusters around the average, nobody is ten times taller than average, and adding the tallest person in the city barely budges the mean. Industrial fires, floods, epidemics, financial crashes, and power outages have a **heavy tail**: the far edge is stretched out so wide that a tiny handful of extreme shocks delivers most of the century's damage. Notice immediately why the biggest number in your logbook is *never* the worst possible event: it is only the largest surprise your short window happened to catch so far.

Alexander McNeil's study of $2{,}156$ Danish fire-insurance losses exceeding one million Danish kroner from 1980 to 1990 (1985 values, covering buildings, contents, and lost profits) shows this directly (McNeil 1997). Statisticians measure tail heaviness with a **tail index** $\alpha$ (or, for power-law heavy tails, its reciprocal $\xi = 1/\alpha > 0$, estimated directly as a generalized Pareto shape parameter $\hat{\xi}$), where smaller $\alpha$ means wilder extremes. Above high thresholds in McNeil's fire claims, average excess keeps climbing and generalized Pareto shape estimates $\hat{\xi}$ sit between $0.50$ and $0.72$—with a full-series fit of $0.60$—placing the implied tail-index point estimate $\hat{\alpha} = 1/\hat{\xi}$ at or below $2$ (McNeil 1997). Even more telling is sensitivity to one event: omitting the single largest claim of $263$ million kroner drops the shape estimate at the $10$-million-kroner threshold from $0.50$ to $0.39$ (McNeil 1997).

Bruce Hill (1975) built a classic way to estimate that tail index $\hat{\alpha}\_k$ from the spacing among the top $k$ biggest extremes (formalized in [Distribution and Moment](31-distribution-and-moment.md)). The plain intuition is simple. When $\alpha > 2$, population average damage and variance are finite (though across $2 < \alpha \le 4$ the fourth moment still diverges, so sample spread stabilizes cleanly only once $\alpha > 4$). When $1 < \alpha \le 2$, the long-run average stays finite while spread runs wild, so sample averages lurch every time a giant shock lands. And when $\alpha \le 1$, even the population average is infinite, so the running average drifts higher the longer you watch. Because single-cutoff estimates wiggle with $k$, honest analysts plot the tail index across several high thresholds with its uncertainty band instead of trusting one number.

For the **Archive** the lesson is sharp: when one rare shock decides the total, yesterday's average tells you mostly whether a big shock happened to fall inside the short window you watched.

## 2. "It never happened before"

Every town that has ever flooded had, at some point, never flooded before. True, and it tells you
almost nothing.

What "never happened in my lifetime" really says is *nothing that large happened in the window I was
watching.* That is a fact about the length of the **Archive** window, not about the physical limits
of the **World**. Suppose a rare hazard has an average return period of $T > 1$ years—a "100-year flood" has $T = 100$ and chance $1/100$ of striking in any independent year—and you watch for $n \ge 1$ years. To see zero occurrences across all $n$ years, each year independently has to come up calm with chance $1 - 1/T$:

$$\mathbb{P}(\text{0 events in } n\text{ years}) = \left(1 - \frac{1}{T}\right)^n$$

Read that formula aloud: the probability of zero events in $n$ years equals the single-year calm chance $(1 - 1/T)$ multiplied by itself $n$ times. Plug in numbers by hand for a $100$-year flood ($T = 100$):

- **A short window ($n = 20$ years):** $(0.99)^{20} \approx 0.82$. There is an $82\unicode{x25}$ chance of watching for two decades and seeing nothing at all—silence is normal, not proof of safety.
- **A full return period ($n = 100$ years):** Even when your archive spans the full $T = 100$ years ($n = T$), $(0.99)^{100} \approx 0.366$ (and $(1 - 1/T)^T$ exceeds $1/3$ for every $T \ge 6$). A full century still carries more than a one-in-three chance of missing a $100$-year flood completely.
- **Waiting for confidence:** To be $95\unicode{x25}$ sure of witnessing a $100$-year shock at least once (keeping the chance of total silence below failure threshold $\delta = 0.05$), you have to wait roughly three times the return period—around $n \approx T\ln(1/\delta) \approx 300$ years. And if storms cluster in bad decades, independent opportunities shrink further, making quiet stretches even easier to misread.

A second trap sits inside the **Institution**. Records are kept by organizations that pay for disasters, and when a shock wipes out the record-keeper, the failed cohort vanishes from the archive—the survivorship filter examined in [Who Is Missing From the Table](06-who-is-missing-from-the-table.md). So the honest statement is never *this cannot happen*; it is *this has not happened in the period we watched, which is short*.

## 3. What a buffer is for: The survival inequality

A town that cannot stop a flood keeps something in reserve: a higher levee, a spare warehouse of medicine, a backup generator—just as Andean communities linked state *qollqa* storehouses across altitudes to carry multi-year surpluses through prolonged droughts (Murra 1975). Georges Canguilhem (1943/1966) defined vital health not as clinging to a rigid average, but as keeping an adaptive **Vital Margin** able to absorb shocks, while C. S. Holling (1973) separated quick recovery near equilibrium from the wider safety basin before an irreversible cliff. At the same time, banning every ordinary risk to chase absolute worst-case survival would paralyze daily life (Harsanyi 1975).

So how big must a physical reserve be? Think in plain time before introducing shorthand. Two clocks govern survival during a breakdown:

1. **Survival time** asks: once normal supply stops, how many hours or days can the system keep running on what it already holds?
2. **Recovery time** asks: how many hours or days will pass before the alarm is noticed, leaders approve action, and fresh supplies or repairs actually arrive?

Put concrete numbers on a hospital's backup generator. Suppose the reserve fuel tank holds $B = 72$ hours of diesel at the crisis burn rate $\dot{D}\_{\mathrm{crisis}} = 1$ tank-hour per hour, giving $72$ hours of **Time-to-Survival** ($\mathrm{TTS}\_{\mathrm{survival}}$). Meanwhile, detecting a grid cut takes $4$ hours, approving emergency refilling takes $8$ hours, and dispatching a fuel tanker across flooded roads takes $36$ hours—so **Time-to-Recovery** ($\mathrm{TTR}\_{\mathrm{recovery}}$) sums to $4 + 8 + 36 = 48$ hours. The tank outlasts the delay with $24$ hours to spare. Now suppose deliberation stretches to $16$ hours and road dispatch to $60$ hours, pushing recovery to $4 + 16 + 60 = 80$ hours: survival time ($72$ hours) falls below recovery time ($80$ hours), and the lights go out $8$ hours before the tanker reaches the gate.

In general notation, for an unencumbered reserve buffer $B > 0$ and crisis burn rate $\dot{D}\_{\mathrm{crisis}} > 0$:

$$\mathrm{TTS}\_{\mathrm{survival}} = \frac{B}{\dot{D}\_{\mathrm{crisis}}}$$

Read that fraction aloud: **Time-to-Survival equals your usable buffer stock divided by how fast the crisis drains it.** And for recovery latency:

$$\mathrm{TTR}\_{\mathrm{recovery}} = \text{Detection Delay} + \text{Deliberation Delay} + \text{Logistical Lead Time}$$

Read that sum aloud: **Time-to-Recovery adds the time to spot the trouble, the time to decide, and the time for physical help to arrive.** Improvising statistical debates or waiting for committee sign-offs mid-crisis pushes deliberation delay past survival ([Decision, Loss, Threshold](36-decision-loss-threshold.md)), while forced emergency sell-offs lengthen recovery even further ([Chapter 47](47-irreversible-time-vital-margin.md)).

Physical survival (Simchi-Levi et al. 2014) therefore requires the **survival inequality** of [Chapter 47](47-irreversible-time-vital-margin.md):

$$\mathrm{TTS}\_{\mathrm{survival}} \ge \mathrm{TTR}\_{\mathrm{recovery}}$$

Read that inequality aloud: **survival time must meet or exceed recovery time.** Strict safety slack ($\mathrm{TTS}\_{\mathrm{survival}} > \mathrm{TTR}\_{\mathrm{recovery}}$) leaves breathing room; equality sits on the razor's edge; falling below ($\mathrm{TTS}\_{\mathrm{survival}} < \mathrm{TTR}\_{\mathrm{recovery}}$) hits ruin before help arrives. When a system sits near the boundary of its stability basin (Holling 1973), heavy-tailed shocks ($\alpha < 2$) deliver abrupt jumps that cross that cliff without gradual warning ([Chapter 47](47-irreversible-time-vital-margin.md)). Still, clearing the test once grants no permanent immunity: a common-mode storm can spoil part of the reserve stock $B$ right when demand spikes, and a second failure can double recovery time.

Separating the crisis window from calm stretches also clears up how long-run buffers behave. When supply halts during a crisis (arrival rate $\lambda > 0$ of shocks averaging size $\mu > 0$ gives average burn rate $\dot{D}\_{\mathrm{crisis}} = \lambda \mu$), the expected (fluid) survival time is $B/\dot{D}\_{\mathrm{crisis}}$—though random bursts of shocks can still exhaust $B$ sooner. Over long peaceful stretches where steady replenishment $c$ exceeds average drain ($c > \lambda \mu > 0$), light-tailed independent shocks allow safety buffers to cut ruin odds exponentially fast (Lundberg 1903; Cramér 1930). Under power-law heavy-tailed shocks with a finite mean ($1 < \alpha < \infty$), by contrast, single extreme hits ruin even large buffers, so ruin odds shrink far more slowly—only algebraically in buffer size $B$ (Embrechts, Klüppelberg & Mikosch 1997; [Chapter 47](47-irreversible-time-vital-margin.md))—and for $\alpha \le 1$ the expected shock size $\mu$ itself becomes infinite, so no finite replenishment rate $c$ can keep up.

## 4. Why this is uncomfortable

Accept the tail and you accept that the most important number in the room is the one you have seen
least often, and that a plan can look sleek for ninety-nine years and fail in the hundredth—the people
who ran the good years praised for "lean efficiency" while quietly stripping the reserves the hundredth required.

An organization that calculates exact uncertainty intervals yet strips away its physical slack illustrates **Collapse Proposition 2** (*Impotent Lucidity Before Ruin*) in plain daylight: it sees the storm coming, publishes the math, and collapses anyway while a well-buffered neighbor survives. The tail decides when a shock is irreversible; sizing a buffer turns probability into institutional design.

### The queue factor, written out

Why does running at "nearly $100\unicode{x25}$ efficiency" break recovery clocks so abruptly?

Picture a single highway tollbooth—or an emergency triage desk—that can clear $10$ arrivals per hour. When $5$ cars arrive per hour (half full), any random cluster of two or three cars clears within minutes. When $9.5$ cars arrive per hour ($95\unicode{x25}$ full), almost zero idle minutes sit between arrivals: one slow case stalls the line, new cars pile up behind it, and wait times explode.

Put that intuition into simple arithmetic. Write $\rho \in [0, 1)$ for utilization—the fraction of capacity already busy, strictly below $1$ so queues do not explode—and $s = 1 - \rho \in (0, 1]$ for the remaining queue slack (distinct from [Chapter 02](02-the-average-is-not-the-story.md)'s sample standard deviation $s$):

- At $50\unicode{x25}$ busy ($\rho = 0.50$, slack $s = 0.50$), the congestion multiplier is $0.50 / 0.50 = 1$.
- At $90\unicode{x25}$ busy ($\rho = 0.90$, slack $s = 0.10$), the multiplier jumps to $0.90 / 0.10 = 9$—nine times worse.
- At $99\unicode{x25}$ busy ($\rho = 0.99$, slack $s = 0.01$), it explodes to $0.99 / 0.01 = 99$.

John Kingman (1961, 1962) proved that for steady renewal queues with finite arrival and service variance ($\alpha > 2$) and positive variability in arrivals or service times ($c\_a^2 + c\_s^2 > 0$, unlike a strictly clockwork schedule where nothing varies), average waiting time scales directly with that **heavy-traffic congestion multiplier**:

$$\frac{\rho}{1 - \rho} = \frac{1}{s} - 1$$

Read that equation aloud: **utilization divided by remaining slack equals one over slack minus one.** As slack $s$ shrinks toward zero (utilization $\rho$ approaches $1$), waiting times shoot toward infinity, stretching $\mathrm{TTR}\_{\mathrm{recovery}}$ right when you need help fastest. Keeping utilization low cannot replace reserve endurance ($\mathrm{TTS}\_{\mathrm{survival}} \ge \mathrm{TTR}\_{\mathrm{recovery}}$). When service times carry infinite variance ($1 < \alpha \le 2$), expected waiting delays diverge at *any* positive utilization $\rho \in (0, 1)$ (and for $\alpha \le 1$, no steady utilization $\rho < 1$ exists at all), while correlated (non-renewal) arrivals fall outside Kingman's renewal bound and inflate waits without by themselves making average waiting time infinite at $\rho < 1$. Full queueing bounds belong to [Time and Dependence](38-time-and-dependence.md) and [Chapter 47](47-irreversible-time-vital-margin.md).

### When the buffer is the wrong answer

Keeping a big reserve is not always right ([Time and Dependence](38-time-and-dependence.md); [Chapter 47](47-irreversible-time-vital-margin.md)). Holding idle stocks loses its point when three conditions hold together: (i) the shock is genuinely reversible without hitting an unrecoverable cliff; (ii) damage grows in a straight line without convex spikes or queue pileups; and (iii) no capped ledger dumps unpaid damage onto uncounted neighbors ([Who Is Missing From the Table](06-who-is-missing-from-the-table.md)) or an assumed public bailout. When all three hold and quick repair costs less than warehousing reserves, repair rather than hoarding—remembering that rigid over-hoarding against mild hazards carries real opportunity cost (Harsanyi 1975). Otherwise, buffer the irreversible tail-coupled risk, and keep quick-repair plans only for the small, self-contained reversible slice.

## Bridge

[Distribution and Moment](31-distribution-and-moment.md), [Time and Dependence](38-time-and-dependence.md), and [Irreversible Time, Non-Ergodicity, and the Vital Margin](47-irreversible-time-vital-margin.md) formalize heavy-tail estimators, queueing delays, and coupled ruin bounds. [It Worked, So I Was Right](04-it-worked-so-i-was-right.md) turns next to the error of judging choices by short-term outcomes.
