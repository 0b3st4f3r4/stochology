# When the Measure Becomes the Goal

<figure class="stoch-figure stoch-logo-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="800" height="132" viewBox="0 0 800 132" fill="none" role="img" aria-label="Stochology CH07 · GOODHART — When Metrics Become Targets">
  <defs>
    <!-- Divider Progression: Vital Emerald -> Epistemic Sapphire -> Stókhos Gold -> Institutional Crimson -->
    <linearGradient id="ch07-ring" x1="2" y1="78" x2="386" y2="78" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="38%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="68%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <!-- Multi-spectral Atmospheric Glow -->
    <radialGradient id="ch07-glow" cx="52%" cy="52%" r="50%">
      <stop offset="0%" stop-color="#f43f5e" stop-opacity="0.24"/>
      <stop offset="48%" stop-color="#fbbf24" stop-opacity="0.14"/>
      <stop offset="80%" stop-color="#38bdf8" stop-opacity="0.08"/>
      <stop offset="100%" stop-color="#090e1a" stop-opacity="0"/>
    </radialGradient>
    <!-- Pre-Intervention Coupled History Conduit (Metric is informative proxy) -->
    <linearGradient id="ch07-braid-pre" x1="22" y1="92" x2="66" y2="56" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="50%" stop-color="#3b82f6"/>
      <stop offset="100%" stop-color="var(--logo-gold, #f59e0b)"/>
    </linearGradient>
    <!-- Post-Intervention Proxy Gaming Surge Conduit (Skyrocketing target) -->
    <linearGradient id="ch07-proxy-surge" x1="66" y1="56" x2="106" y2="24" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#fbbf24"/>
      <stop offset="45%" stop-color="#f43f5e"/>
      <stop offset="100%" stop-color="#be123c"/>
    </linearGradient>
    <!-- Post-Intervention Vital System Collapse Drain Conduit (Depleted reality) -->
    <linearGradient id="ch07-vital-drop" x1="66" y1="56" x2="106" y2="96" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="55%" stop-color="#047857"/>
      <stop offset="100%" stop-color="#022c22"/>
    </linearGradient>
    <!-- Intervention Policy Barrier Line -->
    <linearGradient id="ch07-barrier-grad" x1="66" y1="22" x2="66" y2="110" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#fef08a"/>
      <stop offset="50%" stop-color="#f59e0b"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <!-- 3D Spherical Node Gradients -->
    <radialGradient id="ch07-node-well" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="var(--logo-well-hi, var(--elevated-bg, #ffffff))"/>
      <stop offset="100%" stop-color="var(--logo-node-bg, #e2e8f0)"/>
    </radialGradient>
    <linearGradient id="ch07-ring-epi" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#7dd3fc"/>
      <stop offset="50%" stop-color="#2563eb"/>
      <stop offset="100%" stop-color="#1e3a8a"/>
    </linearGradient>
    <radialGradient id="ch07-core-epi" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#bae6fd"/>
      <stop offset="45%" stop-color="#3b82f6"/>
      <stop offset="100%" stop-color="#1d4ed8"/>
    </radialGradient>
    <linearGradient id="ch07-ring-gov" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#fda4af"/>
      <stop offset="50%" stop-color="#e11d48"/>
      <stop offset="100%" stop-color="#881337"/>
    </linearGradient>
    <radialGradient id="ch07-core-gov" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#fecdd3"/>
      <stop offset="45%" stop-color="#f43f5e"/>
      <stop offset="100%" stop-color="#be123c"/>
    </radialGradient>
    <linearGradient id="ch07-ring-vital" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#6ee7b7"/>
      <stop offset="50%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#065f46"/>
    </linearGradient>
    <radialGradient id="ch07-core-vital" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#a7f3d0"/>
      <stop offset="45%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#047857"/>
    </radialGradient>
    <!-- Golden Stókhos Reticle Gradients -->
    <radialGradient id="ch07-pupil" cx="45%" cy="42%" r="58%">
      <stop offset="0%" stop-color="#020617"/>
      <stop offset="68%" stop-color="#0f172a"/>
      <stop offset="100%" stop-color="#451a03"/>
    </radialGradient>
    <linearGradient id="ch07-gold-mira" x1="54" y1="48" x2="84" y2="82" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#fef08a"/>
      <stop offset="45%" stop-color="#fbbf24"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <radialGradient id="ch07-gold-core" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#ffffff"/>
      <stop offset="35%" stop-color="#fde047"/>
      <stop offset="100%" stop-color="#d97706"/>
    </radialGradient>
  </defs>
  <g>
    <!-- Atmospheric Glow & Reference Coordinate Horizon -->
    <circle cx="64" cy="66" r="48" fill="url(#ch07-glow)"/>
    <circle cx="64" cy="66" r="42" stroke="var(--logo-ticks, #64748b)" stroke-width="1.2" stroke-dasharray="3 3" stroke-opacity="0.45"/>
    <!-- High-Stakes Target Intervention Barrier (Goodhart / Campbell Threshold) -->
    <line x1="66" y1="22" x2="66" y2="110" stroke="url(#ch07-barrier-grad)" stroke-width="2.4" stroke-dasharray="3.5 2.5" stroke-linecap="round"/>
    <!-- Pre-Intervention Coupled History (Metric is a reliable proxy) -->
    <path d="M 22 92 C 38 84, 52 70, 66 56" stroke="url(#ch07-braid-pre)" stroke-width="3.4" stroke-linecap="round"/>
    <path d="M 22 95 C 38 87, 52 73, 66 59" stroke="var(--logo-blue, #2563eb)" stroke-width="1.6" stroke-dasharray="2 2" stroke-opacity="0.65"/>
    <!-- Post-Intervention Divergence / Goodhart Bifurcation -->
    <!-- Proxy Metric Gaming Surge (Skyrocketing target under optimization) -->
    <path d="M 66 56 C 78 46, 92 34, 106 24" stroke="url(#ch07-proxy-surge)" stroke-width="3.4" stroke-dasharray="4 2.5" stroke-linecap="round"/>
    <!-- Vital System Drain (Plunging underlying reality) -->
    <path d="M 66 56 C 78 64, 90 84, 106 96" stroke="url(#ch07-vital-drop)" stroke-width="3.6" stroke-linecap="round"/>
    <!-- 3D Spherical Nodes -->
    <!-- Initial Base Epistemic Anchor at (22, 92) -->
    <circle cx="22" cy="92" r="8.0" fill="url(#ch07-node-well)" stroke="url(#ch07-ring-epi)" stroke-width="3.0"/>
    <circle cx="22" cy="92" r="3.6" fill="url(#ch07-core-epi)"/>
    <!-- Proxy Gaming Node (Goodhart Surge) at (106, 24) -->
    <circle cx="106" cy="24" r="8.5" fill="url(#ch07-node-well)" stroke="url(#ch07-ring-gov)" stroke-width="3.2"/>
    <circle cx="106" cy="24" r="3.8" fill="url(#ch07-core-gov)"/>
    <!-- Vital Reserve Collapse Floor Node at (106, 96) -->
    <circle cx="106" cy="96" r="8.5" fill="url(#ch07-node-well)" stroke="url(#ch07-ring-vital)" stroke-width="3.2"/>
    <circle cx="106" cy="96" r="3.8" fill="url(#ch07-core-vital)"/>
    <!-- Bifurcation Apex Stókhos Reticle at (66, 56) -->
    <g id="ch07-reticle">
      <circle cx="66" cy="56" r="7.4" fill="var(--logo-node-bg, #e2e8f0)"/>
      <circle cx="66" cy="56" r="6.4" fill="url(#ch07-pupil)" stroke="url(#ch07-gold-mira)" stroke-width="2.4"/>
      <line x1="56.4" y1="56" x2="61.8" y2="56" stroke="url(#ch07-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="70.2" y1="56" x2="75.6" y2="56" stroke="url(#ch07-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="66" y1="46.4" x2="66" y2="51.8" stroke="url(#ch07-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="66" y1="60.2" x2="66" y2="65.6" stroke="url(#ch07-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <circle cx="66" cy="56" r="2.2" fill="url(#ch07-gold-core)"/>
    </g>
  </g>
  <g transform="translate(148, 0)">
    <text x="2" y="27" fill="var(--logo-sub, #1d4ed8)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.2">PART I · THINKING UNDER UNCERTAINTY</text>
    <text x="498" y="27" text-anchor="end" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.0">CH07 · GOODHART</text>
    <text x="0" y="59" fill="var(--logo-ink, #0f172a)" font-family="system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif" font-size="22.2" font-weight="800" letter-spacing="1.35">WHEN METRICS BECOME TARGETS</text>
    <line x1="2" y1="78" x2="384" y2="78" stroke="url(#ch07-ring)" stroke-width="1.6" stroke-opacity="0.82"/>
    <circle cx="400" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="415" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="430" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.0"/>
    <circle cx="430" cy="78" r="1.6" fill="var(--logo-blue, #2563eb)"/>
    <circle cx="445" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="460" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="475" cy="78" r="6.2" stroke="var(--logo-gold, #f59e0b)" stroke-width="1.5" stroke-dasharray="2.2 1.6"/>
    <circle cx="475" cy="78" r="4.0" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-crimson, #e11d48)" stroke-width="2.2"/>
    <circle cx="475" cy="78" r="1.8" fill="var(--logo-gold, #f59e0b)"/>
    <circle cx="490" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-crimson, #e11d48)" stroke-width="2.0"/>
    <circle cx="490" cy="78" r="1.6" fill="var(--logo-crimson, #e11d48)"/>
    <text x="2" y="104" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.8" font-weight="600" letter-spacing="0.65">CAMPBELL AND GOODHART · PROXY GAMING · PERFORMATIVE FEEDBACK</text>
  </g>
  </svg>
</figure>

> School-level chapter of Part I. The formal version is
> [Institution, Incentive, Metric](40-institution-incentive-metric.md) and
> [Reactive Worlds: Causal Invariance, Performativity, and Goodhart's Law](48-reactive-worlds-performativity-goodhart.md).

## By the end of this chapter you can

1. Explain how a metric moves from an informative proxy to a corrupting target under high-stakes optimization.
2. Define and illustrate the four Goodhart variants (regressional, extremal, causal, adversarial) with operational examples.
3. Explain why metric gaming leads to Collapse Proposition 1 (blind undersizing), and formulate dual-signal defenses that keep institutional feedback intact.

## 1. A measure is a shortcut, and then it is the road

A measure is invented because the real thing in the **World** is too rich or too slow to observe directly. A school district cannot look inside ten thousand students' heads to see genuine reading comprehension $G$, so it administers a standardized multiple-choice test $M$. A hospital cannot continuously watch every surgical team's sterility discipline, so it tracks post-operative infection logs. A manufacturer cannot directly observe long-term mechanical craftsmanship on the assembly line, so it counts customer warranty claims.

At that diagnostic stage the measure in the **Archive** is an honest shortcut: errors in $M$ are roughly independent of institutional rewards. Then the **Institution** changes the rules. An administrator compares two schools by their test scores, a ministry publishes the ranking in the newspaper, an executive ties bonuses and tenure to the metric. Inside the classroom, teachers stop assigning full novels and spend six weeks drilling multiple-choice elimination templates. The test score $M$ climbs steadily in the Archive while actual literacy $G$ in the World stalls.

The measurement tool has not changed. What changed is that everybody now knows what is being counted. In 1975 the economist Charles Goodhart formulated the monetary law that *any observed statistical regularity will tend to collapse once pressure is placed upon it for control purposes* (Goodhart 1975), which the anthropologist Marilyn Strathern (1997) distilled into the canonical institutional maxim: **When a measure becomes a target, it ceases to be a good measure**. From the moment rewards or penalties attach to the number, the behavior that produces the number and the underlying reality you intended to serve begin to tear apart. At decision time the missing information is the widening gap $G - M$: the Archive records only the targeted proxy $M$, so the separation stays invisible inside the reported statistics. Lukács named this inversion reification — a relation between persons comes to confront them as a thing with laws of its own, so that what was produced by an agreement about what to count stands over the classroom as an external fact the classroom must satisfy (Lukács 1923). He read the figure out of Marx's account of the commodity, where a social relation between producers appears as a property of things (Marx 1867). The score is that thing, and the disappearance of the agreement from the surface of the score is the whole of its authority. The substitution happens between two people before it happens inside an institution: raise the surveillance of how the other receives your words, and the tolerance margin of the conversation narrows in proportion, because the other is now managing an appearance rather than answering. Sartre named the inertia that follows, the practico-inert: action sedimented into matter that then constrains the action which produced it (Sartre 1960). And keep the distinction, because a score is not only a thing confronting its makers. It is a thing they must keep feeding, and the cost of stopping is why criticism of the score so rarely ends it.

## 2. Four ways the measure breaks

Before an institution attaches incentives to a score, the ex-ante gate of Canonical Operator III audits whether the score tracks reality, using Allan Murphy's (1973) exact decomposition of the quadratic Brier score:

$$\mathrm{BS} = \mathrm{REL} - \mathrm{RES} + \mathrm{UNC}$$

Each term isolates a distinct layer of the mediation circuit:

- **Reliability ($\mathrm{REL} \ge 0$)** measures calibration error between the Archive's stated probabilities and realized empirical frequencies (zero for a perfectly calibrated forecast).
- **Resolution ($\mathrm{RES} \ge 0$)** measures how sharply the forecast separates high-risk from low-risk regimes (entering with a negative sign because stronger discrimination reduces total quadratic error).
- **Uncertainty ($\mathrm{UNC} = \bar{y}(1 - \bar{y})$)** is the irreducible baseline variance of the **World** itself.

The decomposition makes a score auditable even when it is bad, because the three terms can move in opposite directions. A model can improve its calibration while losing the resolution that made it useful, and the total score can then stay flat or improve while the institution gets worse at the only thing it wanted. Recalibration procedures, the pool-adjacent-violators family in particular, are the standard remedy for the first term, and the out-of-sample optimism of a recalibration is bounded by cross-validation rather than asserted.

Apply high-stakes optimization pressure and the breakdown of a target falls into **four distinct Goodhart variants**, as decision theorists and AI safety researchers classify them (Manheim & Garrabrant 2018):

1. **The Regressional Variant (luck at the extremes):** In any noisy measurement $M = G + \eta$, the top of the ranking reflects a combination of competence $G$ and positive noise $\eta$. An institution that rewards only the top scores selects extreme outliers. Next year, as noise reverts toward zero, performance appears to "collapse," though nothing about the underlying skill changed.
2. **The Extremal Variant (pushing out of bounds):** Historical relationships between a proxy and reality hold only within a normal operating range. Push a proxy past the region where the relation was estimated and the system lands out-of-distribution, in regimes where the correlation breaks. A hospital ordered to run its beds at the last per cent of occupancy replaces a finite queueing multiplier with one that diverges as occupancy approaches one, $\rho/(1-\rho)$, and eliminates the **Vital Margin** ($S\_{\mathrm{vital}}$) needed to absorb epidemic surges.
3. **The Causal Variant (breaking the non-causal link):** A metric often correlates with quality without causing it. Schools with extensive libraries frequently produce high-scoring students; buying thousands of books and stacking them in classrooms without changing pedagogy will not raise scores, because the books were a symptom of an engaged community, not the mechanical cause of reading ability. Invariant causal prediction (Peters et al. 2016) separates the two: the predictors whose relation to the outcome survives every regime shift are the ones inside the mechanism, and the ones whose coefficient drifts are the proxies that will break.
4. **The Adversarial Variant (strategic gaming):** Agents actively alter their appearance to fool the metric: teaching exclusively to the format of the exam, refusing to admit high-risk patients who might die during surgery, reclassifying dropouts as "transferred." Where the metric is a surrogate fitted to the institution's own historical support, an unconstrained policy can wander outside that support into a region where the surrogate still returns a number and the world returns ruin. The gap between the dreamed return and the realized one measures how far outside the support the policy was allowed to go.

## 3. Performative prediction and Collapse Proposition 1

Modern machine learning and economic sociology formalize the phenomenon as **performative prediction** (Perdomo et al. 2020) and **Barnesian performativity** (MacKenzie 2006). Deploying a decision parameter $\theta$ shifts the underlying population distribution $D(\theta)$ from baseline mean $\mu\_0$ to $\mu(\theta) = \mu\_0 + \varepsilon\theta$, where $\varepsilon$ is the performative feedback sensitivity.

The institution then faces the expected quadratic performative risk:

$$\mathrm{PR}(\theta) = \frac{1}{2}\Bigl[\bigl(\mu\_0 - (1 - \varepsilon)\theta\bigr)^2 + \sigma^2\Bigr] + \frac{\lambda}{2}\theta^2$$

where $(\mu\_0 - (1 - \varepsilon)\theta)^2$ is the squared prediction mismatch after the World reacts to $\theta$, $\sigma^2$ is irreducible outcome variance, and $\frac{\lambda}{2}\theta^2$ is the cost of institutional control effort.

Two quantities are computable from that expression, and the distance between them is the whole content of performativity:

- **Performative stability ($\theta\_{\mathrm{PS}}$):** An institution that treats the World as a passive camera and repeatedly recalibrates $\theta$ without accounting for its own feedback $\varepsilon$ converges to the fixed point $\theta\_{\mathrm{PS}} = \mu\_0 / (1 + \lambda - \varepsilon)$, where the realized mismatch is exactly the response the institution refuses to model.
- **Performative optimality ($\theta\_{\mathrm{PO}}$):** A Stackelberg planner that internalizes the feedback derivative $\partial\mu / \partial\theta = \varepsilon$ before choosing $\theta$ selects $\theta\_{\mathrm{PO}} = (1 - \varepsilon)\mu\_0 / ((1 - \varepsilon)^2 + \lambda)$, the smallest risk attainable inside the family.

Ignore performative feedback and the institution overshoots the optimal target and inflates its own risk. The overshoot is a function of $\varepsilon$: it vanishes as feedback vanishes and grows without bound as feedback approaches the point where the fixed point ceases to exist. Two limits and a counterexample mark the boundaries:

- **A passive camera ($\varepsilon \to 0$ or $\varepsilon = \lambda$).** When the World does not react to the deployed metric ($\varepsilon = 0$), stability and optimality coincide at $\theta\_{\mathrm{PS}} = \theta\_{\mathrm{PO}} = \mu\_0 / (1 + \lambda)$ with zero performative gap. When feedback exactly balances effort regularization ($\varepsilon = \lambda$), the fixed point sits at $\theta\_{\mathrm{PS}} = \mu\_0$ and the institution's own correction cancels its own distortion.
- **A counterperformative runaway ($\varepsilon \to 1 + \lambda$).** As feedback approaches $1 + \lambda$ the fixed point diverges and the stable risk exceeds twice the optimal risk — the regime in which the institution's attempt to correct itself moves the target further away each round. This feedback triggers **Collapse Proposition 1 (Blind Undersizing)** directly: an emergency agency that sizes its vital flood reserves against historical averages while developers pave over natural drainage basins to hit housing targets sizes its buffer against an epistemic mirage.
- **A direct causal target (counterexample).** Optimizing a metric does not corrupt the goal when the indicator is a physical link inside the causal mechanism, within the operating range where the relation was established. Targeting *door-to-needle thrombolysis time* in acute ischemic stroke care directly reduces brain tissue loss, because the clock interval is the treatment pathway itself rather than a detachable proxy.

## 4. Why "just stop measuring" is not the answer

Recognize Goodhart's Law and the temptation is to conclude that all quantitative measurement corrupts, and that institutions should rely on informal human judgment alone.

That response is more dangerous than the corruption. Without open metrics, institutions regress to unexamined impressions, seniority bias, and the dominance of whoever speaks with the greatest social authority. The quiet, effective worker disappears; the charismatic self-promoter thrives.

The proper institutional defense across the **Nuclear Triad** ($S\_{\mathrm{epi}}, S\_{\mathrm{vital}}, S\_{\mathrm{gov}}$) consists of three architectural rules:

1. **Never optimize a single proxy:** Maintain dual, competing signals that create friction against gaming, and check that the pair has discriminative resolution — a sharp indicator distinguishes regimes, a lazy one merely restates the base rate, and the difference is visible in Murphy's resolution term before any incentive is attached.
2. **Protect the algedonic warning channel:** Maintain an independent, un-gameable reporting bypass where frontline operators can flag systemic degradation without testimonial discount, so that the warning arrives with its credibility intact.
3. **Seal decision premises before outcomes:** Record what the measure meant at the moment of decision, in a tamper-evident record that can be opened and checked afterwards, rather than retroactively adjusting the target after the consequences arrive.

### The point the measure converges to

A score is auditable even when it is bad. Before asking whether a measure improved, ask which of
Murphy's three terms moved (Murphy 1973), because a score can rise while the model gets better at the
part the institution does not care about. Once the institution pays for the score, the parameter stops
being a photograph of the world and becomes the fixed point of a loop that moves the target past the
optimum, and the difference is paid in risk rather than in score: the four variants of section 2 name
the mechanisms by which the gap is exploited.

### When gaming stops paying

The thesis stops holding when the expected cost of gaming exceeds the gain in the indicator. Write $p$ for the probability of an audit, $F$ for the fine and $\Delta M$ for the gain in the measured quantity: the conclusion survives while $p\mkern3muF \le \Delta M$ across the range the institution controls, and it fails once the inequality reverses. Publish the audit probability, the fine and the ceiling of the indicator together, a cost that is not published is not a cost the optimiser sees. The pattern was named half a century ago, when Goodhart observed that any statistical regularity
collapses once pressure is placed on it for control purposes (Goodhart 1975); the decomposition above
adds the price tag, not the observation.

## Bridge

[Institution, Incentive, Metric](40-institution-incentive-metric.md) traces how bureaucratic contracts
generate these failure modes, while [Reactive Worlds](48-reactive-worlds-performativity-goodhart.md)
formulates performative prediction and causal invariance testing. The four variants of section 2 are
the diagnostic vocabulary that chapter makes operational. The interpersonal analogue — impression
management as the corruption of dialogue — is worked out in
[Chapter 27](27-what-judging-is.md), where the same
substitution of a monitored appearance for an honest aim appears at the scale of two people rather
than two thousand. [Three Things You Can't Trade](08-three-things-you-cant-trade.md)
synthesizes these defenses into the complete Nuclear Triad.
