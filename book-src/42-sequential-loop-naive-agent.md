# The Sequential Loop and the Naive Optimizer

<figure class="stoch-figure stoch-logo-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="660" height="132" viewBox="0 0 660 132" fill="none" role="img" aria-label="Stochology CH42 — The Sequential Loop And Naive Agent">
  <defs>
    <linearGradient id="ch42-ring" x1="2" y1="78" x2="386" y2="78" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="38%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="68%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <radialGradient id="ch42-glow" cx="50%" cy="50%" r="50%">
      <stop offset="0%" stop-color="#fbbf24" stop-opacity="0.24"/>
      <stop offset="50%" stop-color="#38bdf8" stop-opacity="0.14"/>
      <stop offset="82%" stop-color="#e11d48" stop-opacity="0.08"/>
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
    <line x1="20" y1="98" x2="108" y2="98" stroke="var(--logo-crimson, #e11d48)" stroke-width="2.4"/>
    <path d="M 22 68 L 36 98" stroke="var(--logo-crimson, #e11d48)" stroke-width="2.4" stroke-linecap="round"/>
    <path d="M 22 68 C 40 64, 54 76, 68 98" stroke="var(--logo-crimson, #e11d48)" stroke-width="2.2" stroke-dasharray="3.5 2.5" stroke-linecap="round"/>
    <path d="M 22 68 C 46 72, 74 78, 104 82" stroke="var(--logo-blue, #2563eb)" stroke-width="2.4" stroke-dasharray="4 2.5" stroke-linecap="round"/>
    <path d="M 22 68 C 46 58, 74 44, 104 34" stroke="var(--logo-green, #10b981)" stroke-width="3.2" stroke-linecap="round"/>
    <circle cx="22.00" cy="68.00" r="6.55" fill="url(#ch42-node-well)" stroke="url(#ch42-ring-epi)" stroke-width="2.42"/>
    <circle cx="22.00" cy="68.00" r="2.81" fill="url(#ch42-core-epi)"/>
    <circle cx="68.00" cy="98.00" r="6.22" fill="url(#ch42-node-well)" stroke="url(#ch42-ring-gov)" stroke-width="2.29"/>
    <circle cx="68.00" cy="98.00" r="2.66" fill="url(#ch42-core-gov)"/>
    <g>
      <circle cx="104.00" cy="34.00" r="6.81" fill="var(--logo-node-bg, #e2e8f0)"/>
      <circle cx="104.00" cy="34.00" r="5.89" fill="url(#ch42-pupil)" stroke="url(#ch42-gold-mira)" stroke-width="2.21"/>
      <line x1="95.17" y1="34.00" x2="100.14" y2="34.00" stroke="url(#ch42-gold-mira)" stroke-width="2.21" stroke-linecap="round"/>
      <line x1="107.86" y1="34.00" x2="112.83" y2="34.00" stroke="url(#ch42-gold-mira)" stroke-width="2.21" stroke-linecap="round"/>
      <line x1="104.00" y1="25.17" x2="104.00" y2="30.14" stroke="url(#ch42-gold-mira)" stroke-width="2.21" stroke-linecap="round"/>
      <line x1="104.00" y1="37.86" x2="104.00" y2="42.83" stroke="url(#ch42-gold-mira)" stroke-width="2.21" stroke-linecap="round"/>
      <circle cx="104.00" cy="34.00" r="2.02" fill="url(#ch42-gold-core)"/>
    </g>
  </g>
  <g transform="translate(148, 0)">
    <text x="2" y="27" fill="var(--logo-sub, #1d4ed8)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.2">PART IV · THE FRAMEWORK</text>
    <text x="498" y="27" text-anchor="end" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.0">CH42</text>
    <text x="0" y="59" fill="var(--logo-ink, #0f172a)" font-family="system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif" font-size="18.5" font-weight="800" letter-spacing="0.95">THE SEQUENTIAL LOOP AND NAIVE AGENT</text>
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
    <text x="2" y="104" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.8" font-weight="600" letter-spacing="0.65">COMMUNITY RESERVE ENVIRONMENT · TRIADIC ABLATIONS · ABSORBING RUIN BARRIER</text>
  </g>
  </svg>
</figure>

## By the end of this chapter you can

1. Name the four structural realities that separate this loop from a textbook Markov decision process, and say which part of the mediation circuit each one breaks.
2. Decide from the terminal reserve and the ruin step alone whether a run ended.
3. Say why the naive point optimiser loses all three margins at once, and which axis its failure enters through first.

In Reinforcement Learning (Sutton & Barto), an agent interacts with an environment across discrete time steps $t = 0, 1, 2, \dots, T-1$. At each step, the agent receives a state or observation, chooses an action, and observes a scalar reward and the next state.

That abstraction holds on a board game. Move it to a hospital emergency network, a municipal water reserve, a public credit guarantee fund or a civil-defense evacuation system, and **four structural realities** intervene between the world and the decision-maker:

1. **The Archive is Mutilated:** The decision-maker never sees the latent world state $S\_t$ directly; the decision-maker sees an institutional archive $O\_t$ that arrives in correlated clusters and systematically under-records the most vulnerable strata ($\rho\_{R,Y} \ne 0$).
2. **Harm is Convex and Tails are Heavy:** Demand surges follow power-law tails ($\alpha < 2$), and when service utilization $\rho\_t = S\_t / C\_t$ approaches $100\%$, queue waiting times and unmet human needs explode nonlinearly according to Kingman's heavy-traffic law $\frac{\rho\_t}{1 - \rho\_t}$.
3. **The World Reacts Performatively:** High-powered bureaucratic targets on proxy metrics ($\beta\_{\mathrm{proxy}} \to 1$) induce strategic gaming and erode the underlying capacity of the institution over subsequent steps.
4. **Time is Irreversible and Ruin is Absorbing:** The community depends on a finite vital reserve $R\_t \ge 0$. If over-leverage or unbuffered overload drains $R\_{t+1} \le 0$ before the institution can recover ($\mathrm{TTS}\_{\mathrm{survival}} < \mathrm{TTR}\_{\mathrm{recovery}}$), the system hits an absorbing ruin barrier.

![The Sequential Agent–World–Institution Loop](figures/fig-01-sequential-loop.svg)

---

## 1.1 The Evolving Environment: "The Community and the Reserve"

The loop this chapter builds has three stations and a fixed number of shifts. At each shift the community presents a latent need $S\_t$; the archive carries a partial recording $O\_t$ of it; the institution provisions capacity, holds a reserve buffer, chooses growth leverage, sets the pressure on a proxy target, and decides whether to hear its frontline. The stylized environment is deliberately small enough to reason about: a handful of neighbourhood clusters, a subset of them well served and the rest high-vulnerability, with the vulnerable subset carrying both a higher baseline need and the exposure to the far tail of the shock distribution. Nothing in the argument depends on the particular counts; what matters is that the heterogeneity between clusters is systematic rather than random.

- **Latent Community Demand ($S\_t$):** At each shift, the community consists of a small number of neighbourhood clusters of households. Some clusters are well-served neighbourhoods; others are high-vulnerability neighbourhoods experiencing higher baseline need and stronger exposure to Pareto tail surges ($\alpha < 2$) on crisis shifts.
- **Selective Archive Recording ($O\_t$):** While well-served neighbourhoods have a high recording propensity in the administrative database, vulnerable neighbourhoods are substantially non-recorded, so the exclusion is missing-not-at-random. Consequently, the raw archive mean $\bar{O}\_t$ systematically underestimates true community need $S\_t$, while intra-cluster shocks inflate the variance of $\bar{O}\_t$.
- **Agent Action:** Given an archive observation, any policy chooses
  - an epistemic demand interval $[L\_t, U\_t]$ and a recognized effective sample size $n\_{\mathrm{eff}}$;
  - provisioned shift capacity $C\_t$, vital reserve buffer $B\_t$, and growth leverage $f\_t$;
  - proxy incentive pressure $\beta\_{\mathrm{proxy}}$, status-based testimonial discount $\delta\_{\mathrm{status}}$, and whether the constitutional emergency brake ($S3^\ast$) is active.

### 1.1.1 State-Space Dynamics of the Triadic Mediation Loop

Formally, the interaction unfolds as a non-Markovian dynamical system across World ($P\_1$), Archive ($P\_2$), and Institution ($P\_3$):

1. **Latent World Transition:** The true community state evolves as:
   $$S\_{t+1} \sim \mathcal{T}(S\_t, A\_t, \boldsymbol{\omega}\_t)$$
   where $\boldsymbol{\omega}\_t \sim \mathcal{D}\_{\mathrm{shock}}$ is an exogenous vector of cluster-level baseline vulnerabilities and heavy-tailed Pareto shocks, modulated by performative erosion induced by prior policy choices $A\_t$.
2. **Selective Archive Filtering:** The administrative observation is generated by a censoring and clustering filter:
   $$O\_t \sim \mathcal{M}(S\_t, \mathbf{R}\_t)$$
   where $\mathbf{R}\_t \in \{0, 1\}^N$ is the recording indicator vector. Because vulnerable strata have a lower recording propensity than well-served strata, $\rho\_{R,Y} \ne 0$, and the raw sample mean $\bar{O}\_t$ is systematically displaced downward from the true latent mean $\bar{S}\_t$.
3. **Vital Reserve Evolution:** The community's physical buffer evolves by the balance equation:
   $$R\_{t+1} = R\_t + \Delta R\_t(A\_t, S\_t)$$
   where net drift $\Delta R\_t$ accounts for shift replenishment, baseline holding costs, spot-procurement expediting costs when demand exceeds provisioned capacity ($S\_t > C\_t$), and queue congestion penalties.

### 1.1.2 Symmetric Convex Loss vs. Dirac-Infinite Absorbing Ruin

Standard reinforcement learning and econometric loss functions typically assume symmetric or unconstrained convex penalties:
$$\mathcal{L}\_{\mathrm{sym}}(C\_t, S\_t) = \frac{1}{2}(C\_t - S\_t)^2 \quad \text{or} \quad \mathcal{L}\_{\mathrm{lin}}(C\_t, S\_t) = |C\_t - S\_t|$$
Under such formulations, under-provisioning ($C\_t < S\_t$) is treated as commensurable with over-provisioning ($C\_t > S\_t$): a deficit of one unit today can supposedly be offset by a surplus of one unit tomorrow.

In a physical reserve the symmetry fails at a point. The vital reserve $R\_t$ faces an **absorbing ruin boundary** at zero:
$$\mathcal{L}\_{\mathrm{vital}}(R\_{t+1}) = \begin{cases} \phi(S\_t - C\_t) & \text{if } R\_{t+1} > 0 \\ +\infty & \text{if } R\_{t+1} \le 0 \end{cases}$$
When $R\_{t+1} \le 0$, the institution enters bankruptcy, stockout, or clinical shutdown: effective capacity drops to zero ($C\_\tau = 0$) for all remaining time steps $\tau > t$. Once absorbed, no future positive expectation can resurrect the institution. Sizing capacity for lean utilization of an underestimated mean guarantees that an unbuffered tail shock drives $R\_{t+1} \le 0$, converting a finite temporary deficit into an absorbing institutional fatality.

---

## 1.2 Why the Point Optimiser Collapses

Consider the benchmark every practitioner reaches for first: a policy that reads the archive, takes its raw mean, treats the records as independent and identically distributed, sizes capacity for lean utilization of that mean, applies an unshrunk plug-in growth fraction, presses hard on the proxy target, and discounts what the frontline reports. Call it the naive point optimiser. Its arithmetic is impeccable and its premises are false one by one.

The naive rule treats the raw archive sample as i.i.d. and missing completely at random, sizes capacity for lean utilization of that downward-biased mean, applies unshrunk plug-in Kelly leverage, imposes high proxy pressure ($\beta\_{\mathrm{proxy}} \to 1$) and discounts frontline algedonic warnings by status discount $\delta\_{\mathrm{status}} > 0$. It violates all three axes of the Nuclear Triad at once. Each violation is a separate literature: Meng (2018) for the displacement of the archive mean under non-random recording, Kish (1965) and Moulton (1986) for the variance the cluster design hides, Kelly (1956) and Baker and McHale (2013) for the growth fraction that a finite estimation sample cannot support, Holmström and Milgrom (1991) for the care a high-powered proxy cannibalizes, and Beer (1972) for the alarm channel a hierarchy can mute.

What makes the combination fatal rather than expensive is the absorbing barrier of §1.1.2. A policy can be wrong about the mean and lose money; a policy that is wrong about the mean *and* holds no buffer does not get a second shift to be right in. The three margins are introduced one at a time in the chapters that follow because the naive rule fails them jointly, and the order of introduction is the order in which its failure becomes visible.

---

## 1.3 The Naive Rule Against the Triadic Agent

The comparison the chapter is built around is not between two numbers but between two policies on one trajectory. On the same environment, under the same exogenous shock stream, the naive rule and the triadic agent diverge first in coverage, then in harm, and finally in whether the run has a terminal reserve at all.

The naive rule's interval covers the true demand on a minority of shifts, because the selection correlation of §1.1 displaces its centre downward while the cluster correlation of §1.1.1 understates its width; the two defects point the same way and neither is repaired by more records. Where true demand then exceeds its lean capacity, Kingman congestion and spot procurement draw on the reserve, the reserve crosses zero, and absorbing ruin sets operational capacity to zero for the rest of the horizon. The harm that follows is the integral of what nobody served, and it is not a bad week at the end of a good quarter: it is the remainder of the horizon.

The triadic agent answers with three safeguards rather than one. The **Epistemic Margin** takes Manski's worst-case bounds, corrects the archive centre for the selection correlation, and contracts the sampling variance by the design effect, so the interval it reports is the interval the design supports. The **Vital Margin** prices convexity, holds utilisation below the point at which Kingman's factor turns vertical, shrinks the growth fraction by the epistemic Kelly correction with the effective sample size, and protects the Canguilhem–Holling basin rather than the equilibrium point. **Institutional Governance** keeps the proxy pressure low and transmits frontline warning undiscounted. The result is a policy that covers the demand it was built to cover, reduces mean harm, and ends the horizon holding a reserve it did not begin with.

Chapters 43 through 50 open each safeguard and build it step by step. At the trajectory level the loop also carries the grammar of the refusal: a shift whose declared demand interval is degenerate or inverted is not silently averaged into the margin, it is quarantined, because an unparsable interval is a statement about the institution rather than about the world. At the interpersonal two-agent scale the same structure runs over a dialogue trajectory, where the epistemic margin becomes the reception margin and the tolerance margin: the speaker who polices the listener's reception has committed, in miniature, the error the institution commits at scale.

---

### Coverage does not order harm

Coverage does not order harm. A policy that covers a small fraction of the shifts can harm less than one that covers almost all of them, because serving often while captured is worse than serving rarely while blind: the first keeps delivering a captured service and books it as success, the second at least knows it is not serving. The terminal reserve is the only observable that knows whether the community survived. The failure of the naive rule is the textbook case of myopic optimisation (Sutton & Barto 2018).

The comparison is worth doing once by hand, and in symbols rather than in the units of any one run. Let policy $A$ serve a share $s\_A$ of a horizon of $T$ shifts and policy $B$ a share $s\_B$, with realised harms $H\_A$ and $H\_B$; the cost per served shift is $H/(sT)$, and the ratio of the two is $(H\_A/H\_B)(s\_B/s\_A)$, which grows without bound as the smaller share falls at fixed harm. No coverage statistic reports that ratio, because coverage counts the shifts a system meant to serve and not the ones it managed to keep.

### When the naive rule is the correct rule

The comparison is a lesson under absorbing ruin and a short horizon. With absorption impossible, the terminal result is the only criterion and the retained margin is pure cost; the procedure is then to test the barrier first, and to drop the margin where the barrier cannot be reached. That is not a concession to the optimiser, it is the same test the chapter's three propositions impose, applied in the other direction.

### Two families, not an ordering

A set of policies compared on one environment is worth reading as a single object rather than as a ranking. Some of them end the horizon holding a reserve; the others end at a step number, because the reserve crossed zero and the run stopped being a run. The comparison that matters is not between the reserves but between the two families: the policies that keep the vital margin end with a reserve, the policies that drop it end at a step, and the two are not commensurable on a scale of mean harm. An average over both families puts a run that ended early next to one that never ended, and reports a number that no decision can use.

Beyond the physical distinction between survival and absorption, the comparison contrasts three political philosophies of collective resources: the naive optimiser models myopic extractive predation that treats common buffers as expendable slack; the policy that keeps the epistemic margin and drops governance models sovereign capture, where epistemic doubt is weaponized to privatize vital reserves under limited liability; and the full triad embodies immanent precautionary self-governance of the commons, sustaining shared reserves through horizontal accountability and biophysical basin ceilings.

### What a trajectory shows that a summary cannot

A trajectory is not a summary. The two observables that decide whether a run ended are not summaries at all: the terminal reserve and the ruin step. A run whose ruin step is empty survived the horizon; a run with a ruin step stopped there, and the reserve left at that point is not a reserve the community can draw on.

That is also why the argument in this chapter is stated over shapes and not over printed values. A number copied from one run into prose is a claim that cannot be checked without the run, while a proposition about which family survives the barrier can be checked against any run that respects the loop: the family that keeps the vital margin survives it, and the family that drops it does not. The shape is what a reader can hold a second run against.

## 1.4 The barrier at both ends, and the buffer that is the drain

### Two limits of the same balance equation

The queue term first. The factor the chapter's convexity argument rests on is $W(\rho) = \rho/(1-\rho)$, and the two values that matter are the slack the full agent is allowed and the lean utilisation the optimiser chooses. The comparison is not between two numbers but between two regimes of one function: at half the servers busy the factor is $1$, at nine tenths it is $9$, at ninety-five hundredths it is $19$, so the lean rule pays a waiting factor that has no bound on the same demand. Symbolically $\lim\_{\rho \to 1} W = \infty$ while $W(\rho) \approx \rho$ for small $\rho$, so the factor is flat where institutions feel comfortable and vertical where they are.

The reserve term second. With the reserve opened at $R\_0$, a constant per-shift drain $d$ absorbs at the first $t$ with $R\_0 - t\mkern3mud \le 0$, so $\tau\_{\mathrm{ruin}} = \lceil R\_0/d \rceil$: doubling the drain halves the survival time, and the identity is linear in the drain and linear in the endowment. After absorption the effective capacity is $0$ for every remaining shift, so the same identity fixes the size of the loss: the shifts that follow absorption are the remainder of the trajectory and not a bad week. The ruin step and the terminal reserve are the two observables that decide whether a run ended.

### The regime where the buffer is the drain

The form this chapter builds (convex harm, a finite reserve, an absorbing barrier at zero) applies to a perishable stock exactly as it applies to a credit reserve, and in that regime the recommendation it issues inverts. Let demand sit at a level $\bar{S}$ with one surge of $\bar{S} + \Delta$, let unmet demand cost $e$ per unit and let idle provisioned capacity carry a charge $h$ per unit per shift paid out of the reserve. The lean rule provisions $C = \bar{S}$: the single surge costs $e\Delta$, the reserve falls by that amount, and the horizon closes alive if the endowment covers it. The precautionary rule provisions $C = \bar{S} + B$: idle capacity is $B$ units on every shift, the carrying charge is $hB$, and the reserve absorbs at the first shift with $R\_0 - hB \le 0$. Symbolically $\tau\_{\mathrm{ruin}} = \lceil R\_0/(hB)\rceil$ for carrying charge $h$ and idle buffer $B$, so a buffer held against a surge that may never arrive is a drain proportional to its own size, and the two rules cross where $hB$ approaches the surge's unmet demand. The loop built in this chapter has no such term: it charges overload and congestion against the reserve and charges nothing for idle provisioned capacity, which is exactly why the buffer is cheap there. A reader who carries this loop into cold storage, blood products or leased capacity has carried it into the regime above, and the advice to keep slack reverses without a single equation changing.

### When the horizon outlives the decider

The thesis under test is that the absorbing barrier is the binding criterion and that the three margins are the answer to it. The condition that invalidates the thesis is a mismatch of horizons: the loop presumes that the agent whose reserve is being drawn lives at least as long as the ruin accrues. Where the accounting period is shorter than the recovery latency (a four-year mandate against a forty-year aquifer) the terminal reserve is not an outcome for the decider but an externality handed to a successor, and the instruction to preserve it has no addressee. The procedure when the condition fails is not to argue about the optimiser. Set the loop's horizon to $T = \max(T\_{\mathrm{episode}},\mkern3mu \mathrm{TTR}\_{\mathrm{recovery}})$, using the recovery latency the vital margin already computes, and bind the decision with a pre-outcome commitment that outlives the tenure: an append-only sealed record of the ex-ante state, which Chapter 49 formalises. Where no such commitment can be sealed, the episode-level optimum is quotable only as a lower bound on the institutional one.

## Exercises

1. **Lean Utilization Trap:** Take a policy that provisions capacity at the raw archive mean divided by a target utilisation and keeps the epistemic margin switched off. Why does it still underestimate crisis surges when the vulnerable clusters are substantially under-recorded? Answer in terms of Meng's selection correlation and Kingman's factor, and say which of the two the utilisation target moves.
2. **Independent Exogenous Streams:** Why does drawing the environment's shocks from a substream dedicated to the time step guarantee that changing an agent's internal random draws never shifts the exogenous shocks? Give the argument in terms of non-interference between streams, and say what breaks if the environment and the agent share one source.
3. **Vulnerability Shift and Time-to-Ruin Sensitivity:** Raise the vulnerable cluster's baseline need, or thicken the Pareto tail by lowering $\alpha$ toward $1$. How does $\tau\_{\mathrm{ruin}}$ for the naive rule move toward $t = 1$, and why does raising the initial reserve $R\_0$ provide only linear delay against power-law tail shocks?

## Bridge

This chapter hands forward a loop and a defect. The loop is the one every later chapter runs: an
environment that emits a signal, an archive that stores it, an institution that acts on the archive and
a world that absorbs the consequence. The defect is that the naive optimiser treats the signal as
a sample from a known frame, so its reserve is sized against a variance it believes it has measured.

[Inquiry, Openness and the Multiverse](43-inquiry-openness-multiverse.md) opens that frame rather than
widening the sample inside it: Cromwell's floor, severity as the criterion a test must satisfy, agnotology
as the production of the missingness, and the multiverse as the set of analyses the archive permits. What
travels forward is the epistemic input of the loop, the first of the three margins, still a scalar and
not yet a set. The debt this chapter leaves is the width: a loop that reports a coverage figure without
saying which frame produced it has reported nothing that survives a change of frame, and [Chapter 43](43-inquiry-openness-multiverse.md) is where
that debt is paid or declared.
