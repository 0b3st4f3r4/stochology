# Justification and Induction

<figure class="stoch-figure stoch-logo-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="800" height="132" viewBox="0 0 800 132" fill="none" role="img" aria-label="Stochology CH17 · SEVERITY — Induction, Corroboration, And Testing">
  <defs>
    <!-- Canonical Ring Gradient -->
    <linearGradient id="ch17-ring" x1="2" y1="78" x2="386" y2="78" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="38%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="68%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <!-- 1. Atmospheric Cosmic Field -->
    <radialGradient id="ch17-glow" cx="52%" cy="52%" r="50%">
      <stop offset="0%" stop-color="#fbbf24" stop-opacity="0.24"/>
      <stop offset="48%" stop-color="#38bdf8" stop-opacity="0.14"/>
      <stop offset="80%" stop-color="#e11d48" stop-opacity="0.08"/>
      <stop offset="100%" stop-color="#090e1a" stop-opacity="0"/>
    </radialGradient>
    <!-- 3D Spherical Node Gradients -->
    <radialGradient id="ch17-node-well" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="var(--logo-well-hi, var(--elevated-bg, #ffffff))"/>
      <stop offset="100%" stop-color="var(--logo-node-bg, #e2e8f0)"/>
    </radialGradient>
    <linearGradient id="ch17-ring-epi" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#7dd3fc"/>
      <stop offset="50%" stop-color="#2563eb"/>
      <stop offset="100%" stop-color="#1e3a8a"/>
    </linearGradient>
    <radialGradient id="ch17-core-epi" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#bae6fd"/>
      <stop offset="45%" stop-color="#3b82f6"/>
      <stop offset="100%" stop-color="#1d4ed8"/>
    </radialGradient>
    <linearGradient id="ch17-ring-vital" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#6ee7b7"/>
      <stop offset="50%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#065f46"/>
    </linearGradient>
    <radialGradient id="ch17-core-vital" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#a7f3d0"/>
      <stop offset="45%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#047857"/>
    </radialGradient>
    <linearGradient id="ch17-ring-gov" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#fda4af"/>
      <stop offset="50%" stop-color="#e11d48"/>
      <stop offset="100%" stop-color="#881337"/>
    </linearGradient>
    <radialGradient id="ch17-core-gov" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#fecdd3"/>
      <stop offset="45%" stop-color="#f43f5e"/>
      <stop offset="100%" stop-color="#be123c"/>
    </radialGradient>
    <!-- Stókhos Reticle Gradients -->
    <radialGradient id="ch17-pupil" cx="45%" cy="42%" r="58%">
      <stop offset="0%" stop-color="#020617"/>
      <stop offset="68%" stop-color="#0f172a"/>
      <stop offset="100%" stop-color="#451a03"/>
    </radialGradient>
    <linearGradient id="ch17-gold-mira" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#fef08a"/>
      <stop offset="45%" stop-color="#fbbf24"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <radialGradient id="ch17-gold-core" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#ffffff"/>
      <stop offset="35%" stop-color="#fde047"/>
      <stop offset="100%" stop-color="#d97706"/>
    </radialGradient>
    <!-- Ink curve gradient -->
    <linearGradient id="ch17-ink-curve" x1="0%" y1="0%" x2="100%" y2="0%">
      <stop offset="0%" stop-color="var(--logo-ink, #0f172a)"/>
      <stop offset="50%" stop-color="var(--logo-sub, #1d4ed8)"/>
      <stop offset="100%" stop-color="var(--logo-ink, #0f172a)"/>
    </linearGradient>
    <linearGradient id="ch17-prior-path" x1="22" y1="84" x2="64" y2="56" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#1e3a8a"/>
      <stop offset="60%" stop-color="#2563eb"/>
      <stop offset="100%" stop-color="#38bdf8"/>
    </linearGradient>
    <linearGradient id="ch17-test-barrier" x1="64" y1="22" x2="64" y2="110" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#fde047"/>
      <stop offset="50%" stop-color="#f59e0b"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <linearGradient id="ch17-corrob-branch" x1="64" y1="56" x2="104" y2="28" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#10b981"/>
      <stop offset="50%" stop-color="#34d399"/>
      <stop offset="100%" stop-color="#a7f3d0"/>
    </linearGradient>
    <linearGradient id="ch17-refute-branch" x1="64" y1="56" x2="104" y2="94" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#e11d48"/>
      <stop offset="100%" stop-color="#881337" stop-opacity="0.4"/>
    </linearGradient>
  </defs>
  <g>
    <circle cx="64" cy="66" r="48" fill="url(#ch17-glow)"/>
    <circle cx="64" cy="66" r="42" stroke="var(--logo-ticks, #64748b)" stroke-width="1.2" stroke-dasharray="3 3" stroke-opacity="0.45"/>
    <path d="M 22 84 C 36 68, 48 58, 64 56" stroke="url(#ch17-prior-path)" stroke-width="3.4" stroke-linecap="round"/>
    <line x1="64" y1="22" x2="64" y2="110" stroke="url(#ch17-test-barrier)" stroke-width="3.0" stroke-dasharray="4 2.5"/>
    <path d="M 64 56 C 74 54, 85.72 46.28, 91.38 40.62" stroke="url(#ch17-corrob-branch)" stroke-width="3.2" stroke-linecap="round"/>
    <path d="M 90.60 46.38 L 92.79 39.21 L 85.62 41.40" stroke="url(#ch17-corrob-branch)" stroke-width="2.6" stroke-linecap="round" stroke-linejoin="round"/>
    <path d="M 64 56 C 78 62, 92 80, 104 94" stroke="url(#ch17-refute-branch)" stroke-width="2.6" stroke-dasharray="3 3" stroke-linecap="round"/>
    <circle cx="26" cy="80" r="8.5" fill="url(#ch17-node-well)" stroke="url(#ch17-ring-epi)" stroke-width="3.0"/>
    <circle cx="26" cy="80" r="3.8" fill="url(#ch17-core-epi)"/>
    <circle cx="100" cy="32" r="8.5" fill="url(#ch17-node-well)" stroke="url(#ch17-ring-vital)" stroke-width="3.0"/>
    <circle cx="100" cy="32" r="3.8" fill="url(#ch17-core-vital)"/>
    <g>
      <circle cx="64" cy="56" r="7.4" fill="var(--logo-node-bg, #e2e8f0)"/>
      <circle cx="64" cy="56" r="6.4" fill="url(#ch17-pupil)" stroke="url(#ch17-gold-mira)" stroke-width="2.4"/>
      <line x1="54.40" y1="56.00" x2="59.80" y2="56.00" stroke="url(#ch17-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="68.20" y1="56.00" x2="73.60" y2="56.00" stroke="url(#ch17-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="64.00" y1="46.40" x2="64.00" y2="51.80" stroke="url(#ch17-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="64.00" y1="60.20" x2="64.00" y2="65.60" stroke="url(#ch17-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <circle cx="64" cy="56" r="2.2" fill="url(#ch17-gold-core)"/>
    </g>
  </g>
  <g transform="translate(148, 0)">
    <text x="2" y="27" fill="var(--logo-sub, #1d4ed8)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.2">PART II · THE BASES · PHILOSOPHY</text>
    <text x="498" y="27" text-anchor="end" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.0">CH17 · SEVERITY</text>
    <text x="0" y="59" fill="var(--logo-ink, #0f172a)" font-family="system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif" font-size="18.5" font-weight="800" letter-spacing="0.95">INDUCTION, CORROBORATION, AND TESTING</text>
    <line x1="2" y1="78" x2="384" y2="78" stroke="url(#ch17-ring)" stroke-width="1.6" stroke-opacity="0.82"/>
    <circle cx="400" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.0"/>
    <circle cx="400" cy="78" r="1.6" fill="var(--logo-blue, #2563eb)"/>
    <circle cx="415" cy="78" r="6.2" stroke="var(--logo-gold, #f59e0b)" stroke-width="1.5" stroke-dasharray="2.2 1.6"/>
    <circle cx="415" cy="78" r="4.0" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.2"/>
    <circle cx="415" cy="78" r="1.8" fill="var(--logo-gold, #f59e0b)"/>
    <circle cx="430" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.0"/>
    <circle cx="430" cy="78" r="1.6" fill="var(--logo-blue, #2563eb)"/>
    <circle cx="445" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="460" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="475" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="490" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <text x="2" y="104" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.8" font-weight="600" letter-spacing="0.65">HUME'S HORIZON · POPPERIAN REFUTABILITY · MAYO SEVERE TESTING CRITERION</text>
  </g>
</svg>
</figure>

> On what basis does the past constrain the future, and why does evidence still have weight even when
> no formula settles induction?

## By the end of this chapter you can

1. Distinguish deduction, induction, and abduction using everyday examples, and state Hume's problem of induction alongside Goodman's *grue* riddle.
2. Separate truth transfer inside a closed finite batch ($f = 1$) from extrapolating to future cases, and explain why inquiry stays open through Peirce's inferential cycle.
3. Explain what a probability model adds and cannot add to induction, contrast likelihood support with Keynesian evidential weight $V$, and separate testable doubt from agnotology.

## 1. Deduction and the Limits of Formal Truth Transfer

People reach conclusions from clues in three fundamentally different ways every morning:

- **Deduction**: Every express bus on Route 4 stops at Central Clinic; this is a Route 4 express bus; therefore it stops at Central Clinic. If the premises are true, the conclusion *must* be true.
- **Induction**: The last thirty mornings on Route 4 were delayed ten minutes by roadwork; therefore tomorrow morning's bus will be delayed ten minutes too. The pattern extends from observed cases to an unseen future case.
- **Abduction**: You step outside and notice the hospital driveway is soaked even though the roof is dry; perhaps an underground water pipe burst nearby. You conjecture an explanatory cause that would make the surprising clue make sense.

Only the first step guarantees truth automatically—and that strength also sets its limit. Consider a software verification team that mathematically proves a reserve-allocation routine satisfies every algebraic invariant and never divides by zero. Two weeks after launch, the reserve still hits zero. The theorem never failed; the winter demand premise fed into the routine was simply half of real demand. Deduction transfers truth reliably from premise to conclusion, yet cannot certify whether an empirical premise fits the world.

Formally, a deductive entailment ($\Gamma \vdash \varphi$) guarantees that whenever every premise in $\Gamma$ holds, conclusion $\varphi$ cannot fail. Logic and arithmetic extract consequences already implicit in premises rather than testing uncertain guesses about reality (a distinction formalized in [Chapter 27](27-what-judging-is.md)). Classical deduction also binds the Principle of Explosion ($\lbrace A, \neg A \rbrace \vdash B$), so non-explosive paraconsistent calculi (Jaśkowski 1969 [1948]; da Costa 1974) isolate conflicting archive premises without trivializing entailment, while still leaving empirical extrapolation non-deductive ([Layer 06](56-layer-formal-computable-type-order.md); [The Triad and the Minimum](41-the-triad-and-the-minimum.md) §3). In a **closed, completely counted batch** of size $N > 0$ with zero recording error ($f = n/N = 1$), summarizing every entry collapses into finite arithmetic. Extending claims *beyond* that closed set—to tomorrow's bus, future lending cohorts, or shifting seasons—always crosses a non-deductive bridge.

## 2. Hume's Problem, Goodman's Riddle, and Probability's Scope

Across five years of low interest rates, a credit model trained on rapid short-term borrowing predicted safe repayment while cheap refinancing held. On policy cutoff date $T$, the central bank raises rates sharply, and the exact same feature begins predicting default. Up to $T$, the rules "rapid borrowing predicts safety" and "rapid borrowing predicts safety before $T$ and default after $T$" matched every single historical ledger row identically.

David Hume (1739/1748) named the nerve of this puzzle the **problem of induction**. Moving from observed cases ($A$ has always been followed by $B$) to unobserved cases relies on a **Uniformity Principle**: the assumption that future or unseen instances resemble observed ones. Hume showed this principle cannot be proved by pure logic—denying that the sun will rise tomorrow involves no self-contradiction. Nor can it be proved by citing past inductive success without circularity (*petitio principii*), because appealing to past success assumes the future resembles the past.

Hume was part of a wider global conversation. Centuries earlier in classical India, Cārvāka skeptics denied that finite observation can ever prove universal concomitance (*vyāpti*), prompting sustained Nyāya and Buddhist epistemologies of causal invariance (Matilal 1986). Alongside those debates, Jaina logicians developed *syādvāda* and *anekāntavāda* to refute one-sided absolutism (*ekānta*) and qualify assertions relative to explicit standpoints (Matilal 1981), prefiguring modern non-explosive logical pluralism (da Costa 1980). Six centuries before Hume, in eleventh-century Baghdad, al-Ghazālī's *Tahāfut al-Falāsifa* argued that repeated observation reveals constant conjunction rather than necessary causal power (Marmura 2000). Still, abandoning evidence is impossible when action cannot wait. William Kingdon Clifford urged that it is *"wrong always, everywhere, and for anyone, to believe anything upon insufficient evidence"* (Clifford 1879 [1877]). Under urgent, irreversible decisions (*kairós*), however, suspending judgment does not pause physical danger, so inquiry must measure the inductive gap instead of ignoring it.

Nelson Goodman (1955) pushed the puzzle deeper with the **new riddle of induction**. Imagine a contrived predicate $\mathrm{Grue}(x)$ ("$x$ is grue") tied to a future cutoff date $T$, paired with its symmetric twin $\mathrm{Bleen}(x)$ ("$x$ is bleen"). An emerald counts as grue if it is examined before $T$ and is green, *or* if it is not examined before $T$ and is blue (and bleen swaps the colors across $T$). Writing $\mathrm{Green}(x)$, $\mathrm{Blue}(x)$, and $\mathrm{ExaminedBefore}(x, T)$ for whether stone $x$ is green, blue, or inspected before $T$, we define:

$$\mathrm{Grue}(x) \iff \bigl(\mathrm{ExaminedBefore}(x, T) \land \mathrm{Green}(x)\bigr) \lor \bigl(\neg\mathrm{ExaminedBefore}(x, T) \land \mathrm{Blue}(x)\bigr), \qquad \mathrm{Bleen}(x) \iff \bigl(\mathrm{ExaminedBefore}(x, T) \land \mathrm{Blue}(x)\bigr) \lor \bigl(\neg\mathrm{ExaminedBefore}(x, T) \land \mathrm{Green}(x)\bigr).$$

Read aloud, an object is grue if examined before $T$ and green, or unexamined before $T$ and blue (and symmetrically for bleen). Every emerald examined up to today ($t < T$) is both green and grue, so positive instances confirm "all emeralds are green" and "all emeralds are grue" equally, even though their predictions split after $T$. Goodman (1955) proved syntax alone cannot pick *green* over *grue*, since *green* and *blue* can be defined back out of *grue* and *bleen*. Separating stable kinds from cooked-up rules requires **entrenchment**—and better yet, causal invariance ([Chapter 18](18-philosophy-of-science.md))—since institutional scorecards inherit fragile habits until regimes cross $T$.

Placing induction inside a **probability model** ([Probability as a Measure](15-probability-as-a-measure.md)) does not dissolve Hume or Goodman either. Given a fixed family of mutually exclusive candidate hypotheses $\mathcal{H}\_t$ ($|\mathcal{H}\_t| \ge 2$) over an observable event algebra $\mathcal{F}\_t$ and non-extreme priors $\mathbb{P}(H) \in (0, 1)$, Bayesian conditioning keeps bets coherent, while likelihood ratios measure relative evidence. Yet as Bruno de Finetti (1937) proved via exchangeability (*événements équivalents*), probabilistic updating presupposes an *a priori* judgment about which features count as symmetric. Probability cannot prove Hume's Uniformity Principle, pick natural categories over Goodman's *grue*, invent hypotheses outside $\mathcal{H}\_t$, or repair selective omissions in the archive ([Chapter 16](16-what-knowledge-is.md)).

## 3. Peirce's Inferential Triad and Population Boundaries

Because induction has no logical guarantee, scientific inquiry treats knowledge as an open, self-correcting loop rather than a one-shot proof. Across three riverfront clinics, an epidemiology team notices that every respiratory patient works near a cold-storage depot and conjectures an aerosolized coolant leak ($A$). The team predicts testable consequences—downwind secondary cases within forty-eight hours, none upwind—and places samplers in both corridors so nature can prove the guess wrong.

Charles Sanders Peirce built inquiry around this three-stage cycle (Peirce 1903/1934):

1. **Abduction** (*Hypothesis formation*): From a surprising observation $C$, propose an explanatory guess $A$ (*if $A$ were true, $C$ would follow as a matter of course; hence, there is reason to suspect that $A$ is true*)—the only step that introduces new ideas.
2. **Deduction** (*Consequence tracing*): Work out necessary, observable predictions implied by $A$.
3. **Induction** (*Empirical testing*): Check how closely those predictions match fresh observations, opening the door to severe testing ([Chapter 18](18-philosophy-of-science.md)).

Without abduction, induction searches blindly; without deduction, guesses stay vague; without induction, deductions remain untested stories.

Good inference respects a sharp **closed-population boundary**. Inspecting every voucher in a closed ledger of size $N > 0$ ($f = n/N = 1$) eliminates selection error—as seen directly in the unfactored identity $\bar{Y}\_n - \bar{Y}\_N = \mathrm{Cov}(R, Y)/f = 0/1 = 0$ when $R\_i \equiv 1$ (since $f = 1$ also kills Xiao-Li Meng's (2018) factor $\sqrt{(1 - f)/f}$ while leaving $\sigma\_R = 0$ and $\rho\_{R,Y}$ undefined as $0/0$ in the factored form)—matching sample mean $\bar{Y}\_n$ to batch mean $\bar{Y}\_N$ whenever measurement holds. Extrapolating *beyond* that batch reintroduces uncertainty. As [Chapter 06](06-who-is-missing-from-the-table.md) warned, clustering shrinks effective sample size far below raw count $n$ ([Chapter 45](45-world-to-archive-big-data-paradox.md)), unrandomized pooled averages flip causal findings ([Numbers, Order, and Bounds](10-numbers-order-bounds.md); [Chapter 44](44-underdetermination-partial-identification.md)), and larger biased samples never fix selective omission.

## 4. Agnotology, Premise Rigging, and Severity

In public controversies, fallibilism can be weaponized. Robert Proctor named **agnotology**—the deliberate production of ignorance—in documenting manufactured industrial doubt (Proctor & Schiebinger 2008). As David Michaels (2008) documents—echoed in conflict-of-interest research (Schäffer et al. 2023)—a tobacco executive wrote, *"Doubt is our product, since it is the best means of competing with the body of fact that exists in the minds of the general public"*. Vested actors demand impossible certainty, fund noisy underpowered studies, and shift irreversible harm onto external reception perimeters.

Three guards keep open inquiry from turning into paralyzing doubt:

1. **Skeptical paralysis and weak tests**: Across $|\mathcal{H}\_t| \ge 2$ mutually exclusive candidate hypotheses, Cromwell's rule keeps prior weights $\pi\_0 \in [\varepsilon\_{\mathrm{floor}}, 1 - (|\mathcal{H}\_t| - 1)\varepsilon\_{\mathrm{floor}}] \subset (0, 1)$ (with positive floor $\varepsilon\_{\mathrm{floor}} > 0$, as in [Chapter 15](15-probability-as-a-measure.md)) revisable within $\mathcal{H}\_t$, while Peirce's First Rule of Reason (*do not block the way of inquiry*, Peirce 1899/1931) keeps $\mathcal{H}\_t$ open to new abductions. [Chapter 01](01-deciding-without-knowing.md)'s cost–loss rule authorizes protection once calibrated hazard probability $p$ clears $\min(1, C/(\eta L))$ (matching $\pi\_0$ only pre-data). Deborah Mayo's (2018) severity blocks reading low-power non-rejection ($p > \alpha$) as safety ([Chapter 18](18-philosophy-of-science.md)), while resolution sharper than base-rate guessing defeats empty hedging ([Chapter 50](50-methodological-families-reflexivity.md)). Teams pre-register hypotheses against affected stakes with a public exclusion log to block bad-faith noise ([Chapter 43](43-inquiry-openness-multiverse.md)).
2. **Premise rigging and hidden spillover**: Narrowing questions pre-data or concealing harm pushed onto external reception perimeters creates **premise rigging** ([Chapter 32](32-the-question-before-the-data.md)), countered by world–archive cross-checks and protected civic inspection ([Legitimacy, Accountability, and Institutions](20-legitimacy-and-institutions.md)).
3. **Post-hoc patching**: Bolting *ad hoc* auxiliary excuses onto a refuted model without novel corroborated predictions—or silently contracting the observable event algebra ($\mathcal{F}\_{t+1} \subsetneq \mathcal{F}\_t$) to drop adverse misses—marks Imre Lakatos's (1970) **degenerating problemshift** ([Philosophy of Science](18-philosophy-of-science.md)), whereas attested expansion ($\mathcal{F}\_t \subsetneq \mathcal{F}\_{t+1}^\ast$, writing $\mathcal{F}\_{t+1}^\ast$ for [Chapter 15](15-probability-as-a-measure.md)'s expanded archive $\mathcal{F}'$, alongside a re-anchored hypothesis family $\mathcal{H}\_{t+1}$; Feyerabend 1970) replaces broken premises below.

## 5. Consequences for Epistemic Margin Sizing

When a committee stalls action demanding "more proof," careful judgment pairs probability with **epistemic weight** $V \in [0, 1)$—following John Maynard Keynes (1921), who separated what evidence on balance favors from how much relevant evidence backs it. Weight $V$ differs from deciban likelihood ratios ([Probability as a Measure](15-probability-as-a-measure.md)): strong ratios inside a hand-picked frame cannot manufacture weight, and multiplying records when groups are missing ([Chapter 06](06-who-is-missing-from-the-table.md)) deepens false precision without curing selective omission ([Measuring and Erring](33-measuring-and-erring.md)).

Weight $V$ and sample scale (formalized as $n\_0$ in [Chapter 33](33-measuring-and-erring.md) §4) follow three rules across Parts III and IV:

1. Reporting uncertainty bounds from missing data and underdetermination alongside point estimates;
2. Indexing question framing and external spillover to affected stakes rather than internal budgets alone;
3. Separating reversible low-stakes pauses from irreversible danger—avoiding agnotological delay, paralyzing maximin conservatism (Harsanyi 1975), unpriced countervailing risks (Sunstein 2003; Hahn & Sunstein 2005), or hoarding past opportunity-cost limits ([Numbers, Order, and Bounds](10-numbers-order-bounds.md); [The Triad and the Minimum](41-the-triad-and-the-minimum.md)).

### When the generator changes

Accumulated weight decays under **non-stationarity** when the world shifts ([What Knowledge Is](16-what-knowledge-is.md)). Rather than quietly resetting history, Parts III and IV formalize **attested reopening** ([Aim and the Three Planes](29-aim-and-the-three-planes.md); [Chapter 43](43-inquiry-openness-multiverse.md)): broken models pause, earlier records stay chained in a tamper-evident append-only log, categories re-anchor ($\mathcal{F}\_t \subsetneq \mathcal{F}\_{t+1}^\ast$), and weight rebuilds post-break. Distinct from regime shifts is **multiplicative non-ergodicity**, where ensemble averages diverge from one history under fixed rules ([Chapter 23](23-statistical-physics-and-time.md); [Chapter 47](47-irreversible-time-vital-margin.md)).

## Bridge

Across Part III, [Chapter 27](27-what-judging-is.md) grounds conjectural *stókhos*, [Chapter 29](29-aim-and-the-three-planes.md) maps how world, archive, and institution guard framing and evidence, and [Chapter 41](41-the-triad-and-the-minimum.md) shows why higher-order epistemic regress terminates on the agent's own commitment. The formal tools for framing, measurement scale, partial identification, and sample weight are built in [Chapter 32](32-the-question-before-the-data.md), [Measuring and Erring](33-measuring-and-erring.md), [Chapter 43](43-inquiry-openness-multiverse.md), [Chapter 44](44-underdetermination-partial-identification.md), [Chapter 45](45-world-to-archive-big-data-paradox.md), and [Chapter 50](50-methodological-families-reflexivity.md).

[Philosophy of Science](18-philosophy-of-science.md) next moves from solitary inference to public scientific testing: theory-laden observation, Popperian falsification, paradigm shifts and Duhem–Quine holism, Lakatosian programmes, Mayo's severe testing, and testimonial justice.
