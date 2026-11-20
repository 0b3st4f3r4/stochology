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

> Intuition-first account of Goodhart's Law and reactive feedback; formal apparatus belongs to [Institution, Incentive, Metric](40-institution-incentive-metric.md) and [Reactive Worlds: Causal Invariance, Performativity, and Goodhart's Law](48-reactive-worlds-performativity-goodhart.md).

## By the end of this chapter you can

1. Explain how a helpful indicator turns from an honest shortcut into a distorting target once high stakes attach to it.
2. Tell apart the four ways targets break (regressional luck, pushing past bounds, non-causal proxies, and active gaming) with everyday examples.
3. Explain why unmodeled feedback creates blind undersizing (Collapse Proposition 1) and cynical gaming (Collapse Proposition 3)—and how low-stakes diagnostic audits protect truth without stopping measurement.

## 1. A measure is a shortcut, and then it is the road

A measure is invented because the real thing in the **World** is too rich or too slow to watch directly. A school district cannot look inside ten thousand students' minds to see genuine reading comprehension $G$, so it runs a multiple-choice test $M$. A hospital cannot stand beside every operating table around the clock, so it counts post-surgery infection logs. A manufacturer cannot watch every weld on the assembly line, so it tracks warranty claims.

At that quiet diagnostic stage, the number in the **Archive** is an honest shortcut: errors in the score $M$ have nothing to do with bonuses or penalties. Then the **Institution** raises the stakes. An administrator ranks schools by test averages, newspapers print league tables, and principals win or lose tenure on the score. Inside the classroom, teachers set aside full novels and drill multiple-choice elimination tricks for six weeks. Test scores $M$ climb smoothly in the Archive while real reading comprehension $G$ stalls in the World.

The test paper did not change. What changed is that everyone now knows what is graded. Economist Charles Goodhart (1975) observed that any statistical pattern collapses once officials lean on it for control. Sociologist Donald Campbell (1979) warned that the heavier the stakes riding on a social indicator, the more it corrupts the very process it was meant to track. Anthropologist Marilyn Strathern (1997) distilled both laws into one sentence: **When a measure becomes a target, it ceases to be a good measure.**

From the instant rewards or punishments hang on the score, the behavior that produces the score and the real goal pull apart—while the Archive records only the targeted proxy $M$, hiding the widening gap $M - G$. Georg Lukács (1923) called that inversion *reification*: a human agreement about what to count turns into a rigid idol that stands over the classroom as if it were a law of nature, echoing Karl Marx's (1867) account of how social relations among producers disguise themselves as properties of things. Jean-Paul Sartre (1960) added why such scoreboards linger even when everyone sees the damage: past rules harden into institutional routines that trap the people feeding them. We see the same shift in everyday conversation ([the Prologue](00-preface.md)): raise surveillance over how every phrase lands, and candid speech shrinks into impression management.

## 2. Four ways the measure breaks

Once high stakes press on a single score, breakdown takes **four distinct shapes** (Manheim & Garrabrant 2018):

1. **Luck at the extremes (Regressional):** Every test mixes true skill $G$ with random day-of luck $\varepsilon$, giving $M = G + \varepsilon$. Picking only the highest scorers harvests lucky breaks along with skill—so next year's scores fall back toward earth even when nobody slacks off.
2. **Pushing past the safe boundary (Extremal):** A shortcut works only inside normal limits. Squeezing hospital beds toward $100\unicode{x25}$ occupancy wipes out physical slack—the **vital margin**—until the queue multiplier $\rho/(1-\rho)$ from [Chapter 03](03-the-biggest-number-in-the-room.md) blows up waiting times, or a policy wanders outside historical records where error explodes.
3. **Pulling a non-causal lever (Causal):** Big school libraries correlate with high reading scores, yet stacking books in an empty hallway without teaching reading leaves comprehension unchanged. Separating true causes from surface correlations (Peters et al. 2016) protects against mistaking correlation for control.
4. **Gaming the scoreboard (Adversarial):** High-stakes indicators distort and corrupt the processes they are meant to monitor (Campbell 1979)—from contractors coaching test-item answers and police downgrading complaints to contemporary clinics turning away high-risk surgical patients or schools reclassifying dropouts as "transfers."

Before tying stakes to any binary forecast score, check whether it separates real risk from noise using Allan Murphy's (1973) decomposition of the **Brier score** ($\mathrm{BS}$): **reliability** ($\mathrm{REL}$, calibration error), **resolution** ($\mathrm{RES}$, separation between dangerous and calm cases), and baseline **uncertainty** ($\mathrm{UNC}$, natural outcome unpredictability):

$$\mathrm{BS} = \mathrm{REL} - \mathrm{RES} + \mathrm{UNC}$$

Forecast error rises with miscalibration ($\mathrm{REL}$), drops with resolution ($\mathrm{RES}$), and rests on baseline unpredictability ($\mathrm{UNC}$). Predicting the overall average every day can yield $\mathrm{REL}$ near zero while losing all ability to tell dangerous days from safe ones ($\mathrm{RES}$ collapsing). Reproducible checks such as CORP-PAV avoid arbitrary bins and verify out-of-sample usefulness ([Chapter 48](48-reactive-worlds-performativity-goodhart.md); Dimitriadis, Gneiting, & Jordan 2021).

## 3. Performative prediction, Collapse 1, and Collapse 3

A weather map does not change tomorrow's clouds. A published risk score *does* change tomorrow's people.

Machine learning researchers call this **performative prediction** (Perdomo et al. 2020), and economic sociologists call it **Barnesian performativity** (MacKenzie 2006): publishing a rule or rating shifts the very population distribution it tries to predict. An agency that treats the World as passive and keeps recalibrating onto shifting sands either gets stuck on a distorted feedback loop or chases a moving target into instability—or triggers **counterperformativity**, where following the model undermines the model's own premise (MacKenzie 2006).

Watch how that blind spot feeds **Collapse Proposition 1 (Blind Undersizing)**. When a flood agency publishes low-risk ratings, builders pour concrete across the floodplain; sizing levees from old pre-map flood logs ignores how the rating itself reshaped settlement and future records (Goodhart 1975; Campbell 1979; formalized in [Chapter 48](48-reactive-worlds-performativity-goodhart.md)).

### From performative drift to Collapse Proposition 3

In [Who Is Missing From the Table](06-who-is-missing-from-the-table.md), the return arrow runs from **Institution** back into **World**, paired with accountability for anyone harmed downstream.

When steep bonuses reward a measurable proxy ($a\_1$) while unmeasured care or drainage maintenance ($a\_2$) gets zero points, workers shift effort out of care into the proxy (Holmström & Milgrom 1991; Holmström 2016), or invest in cosmetic gaming ($x\_{\mathrm{game}}$) over real quality ($x\_{\mathrm{real}}$; Kleinberg & Raghavan 2020). Worse, under **limited liability**, leaders pocket short-term gains while passing catastrophic tail damage onto public buffers and neighbors with no seat at the table (Innes 1990; Holmström 2016)—just as paving natural wetlands hits housing targets on paper while flooding downstream neighborhoods—and dismiss frontline warnings. That drives **Collapse Proposition 3 (Cynical Capture and Pillaging)**, stripping shared buffers and exhausting the finite **reception basin** of downstream communities ([the Prologue](00-preface.md)).

## 4. Why "just stop measuring" is not the answer

Critics of target-driven management hit real abuses. Steep bonuses on partial proxies drain unmeasured care, favoring flat pay or direct safeguards (Holmström & Milgrom 1991; Holmström 2016). Single-metric monocultures like Prussian *Normalbaum* forestry maximize timber counts right up to ecological collapse (Scott 1998), and checklist rituals turn into hollow theater (Power 1997). All of those justify **banning high-powered rewards tied to detachable scorecards**.

Throwing away open records altogether, however, destroys the **Archive** and leaves organizations trapped between unrecorded amnesia in the **World** and unaccountable personal whim in the **Institution**. Without honest diagnostics, power defaults to seniority and loud charisma, blind statistical breaks go unseen, and frontline whistleblowers lose corroboration. The answer is **keeping low-stakes diagnostic checks separate from high-powered bonuses** (Klein et al. 2000).

Three rules protect truth, physical slack, and accountability together:

1. **Decouple audits from bonuses:** Pair flat or low-powered incentives with independent diagnostic checks verified for Murphy resolution rather than mimicking averages. Because changing policy stakes alters behavior and breaks historical diagnostic relationships (Lucas 1976; Campbell 1979), keep diagnostic probes outside reward loops—just as low-stakes national NAEP exams exposed narrower gains behind rising state TAAS scores in Texas (Klein et al. 2000)—and pause recalibration whenever feedback begins running away.
2. **Protect frontline distress signals:** Keep an independent warning channel where frontline workers face zero status discounting—a nurse or plant technician heard on equal footing with a director—and tune false-alarm calibration ahead of shifts so routine noise never drowns urgent live distress signals or jams human bandwidth.
3. **Seal plans before results while keeping categories open:** Lock hypotheses, bounds, and reserves before outcomes arrive so goals cannot be moved in hindsight, while keeping framing open to independently attested forward revision without altering past logs whenever spillover harm appears.

### Where targeting stops breaking

Optimizing a metric avoids corruption only when the indicator sits inside a stable causal link within tested limits, public adaptation stays negligible, and unmeasured care is protected. Publishing bigger fines or fixed audit probabilities cannot stop Goodhart on its own: regressional, extremal, and causal breaks happen even without bad intent, legal gaming draws zero fine while limited liability caps penalties (Innes 1990; Holmström 2016; Kleinberg & Raghavan 2020), and announcing exact audit ceilings invites people to game the audit itself. Fair oversight needs open rules paired with sealed spot-checks unsealed only afterward, alongside low-powered pay.

## Bridge

[Institution, Incentive, Metric](40-institution-incentive-metric.md) and [Reactive Worlds: Causal Invariance, Performativity, and Goodhart's Law](48-reactive-worlds-performativity-goodhart.md) formalize limited liability, CORP-PAV calibration, causal invariance, and performative fixed points. [Three Things You Can't Trade](08-three-things-you-cant-trade.md) next gathers these three safeguards together and turns this chapter's warning back on the framework itself: its minimum stays strictly retrospective, never a live target.
