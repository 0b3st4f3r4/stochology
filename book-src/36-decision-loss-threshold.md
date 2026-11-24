# Decision, Loss, Threshold

<figure class="stoch-figure stoch-logo-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="800" height="132" viewBox="0 0 800 132" fill="none" role="img" aria-label="Stochology CH36 · THRESHOLDS — Decision, Asymmetric Loss, And Regret">
  <defs>
    <linearGradient id="ch36-ring" x1="2" y1="78" x2="386" y2="78" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="38%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="68%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <radialGradient id="ch36-glow" cx="50%" cy="50%" r="50%">
      <stop offset="0%" stop-color="#38bdf8" stop-opacity="0.22"/>
      <stop offset="48%" stop-color="#fb7185" stop-opacity="0.14"/>
      <stop offset="80%" stop-color="#e11d48" stop-opacity="0.08"/>
      <stop offset="100%" stop-color="#090e1a" stop-opacity="0"/>
    </radialGradient>
    <linearGradient id="ch36-triad-stroke" x1="18" y1="24" x2="110" y2="104" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.85"/>
      <stop offset="50%" stop-color="var(--logo-gold, #f59e0b)" stop-opacity="0.90"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.85"/>
    </linearGradient>
    <linearGradient id="ch36-fill" x1="22" y1="30" x2="106" y2="98" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)" stop-opacity="0.26"/>
      <stop offset="48%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.28"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.26"/>
    </linearGradient>
    <linearGradient id="ch36-ink-curve" x1="20" y1="66" x2="108" y2="66" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-ink, #0f172a)"/>
      <stop offset="50%" stop-color="var(--logo-sub, #1d4ed8)"/>
      <stop offset="100%" stop-color="var(--logo-ink, #0f172a)"/>
    </linearGradient>
    <radialGradient id="ch36-pupil" cx="45%" cy="42%" r="58%">
      <stop offset="0%" stop-color="#020617"/>
      <stop offset="68%" stop-color="#0f172a"/>
      <stop offset="100%" stop-color="#451a03"/>
    </radialGradient>
    <linearGradient id="ch36-gold-mira" x1="54" y1="48" x2="84" y2="82" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#fef08a"/>
      <stop offset="45%" stop-color="#fbbf24"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <radialGradient id="ch36-gold-core" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#ffffff"/>
      <stop offset="35%" stop-color="#fde047"/>
      <stop offset="100%" stop-color="#d97706"/>
    </radialGradient>
    <radialGradient id="ch36-node-well" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="var(--logo-well-hi, var(--elevated-bg, #ffffff))"/>
      <stop offset="100%" stop-color="var(--logo-node-bg, #e2e8f0)"/>
    </radialGradient>
    <linearGradient id="ch36-ring-epi" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#7dd3fc"/>
      <stop offset="50%" stop-color="#2563eb"/>
      <stop offset="100%" stop-color="#1e3a8a"/>
    </linearGradient>
    <radialGradient id="ch36-core-epi" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#bae6fd"/>
      <stop offset="45%" stop-color="#3b82f6"/>
      <stop offset="100%" stop-color="#1d4ed8"/>
    </radialGradient>
    <linearGradient id="ch36-ring-vital" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#6ee7b7"/>
      <stop offset="50%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#065f46"/>
    </linearGradient>
    <radialGradient id="ch36-core-vital" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#a7f3d0"/>
      <stop offset="45%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#047857"/>
    </radialGradient>
    <linearGradient id="ch36-ring-gov" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#fda4af"/>
      <stop offset="50%" stop-color="#e11d48"/>
      <stop offset="100%" stop-color="#881337"/>
    </linearGradient>
    <radialGradient id="ch36-core-gov" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#fecdd3"/>
      <stop offset="45%" stop-color="#f43f5e"/>
      <stop offset="100%" stop-color="#be123c"/>
    </radialGradient>
  </defs>
  <g>
    <circle cx="64" cy="66" r="48" fill="url(#ch36-glow)"/>
    <circle cx="64" cy="66" r="42" stroke="var(--logo-ticks, #64748b)" stroke-width="1.2" stroke-dasharray="3 3" stroke-opacity="0.45"/>
    <line x1="24" y1="94" x2="104" y2="38" stroke="var(--logo-crimson, #e11d48)" stroke-width="2.8" stroke-linecap="round"/>
    <line x1="24" y1="46" x2="104" y2="86" stroke="var(--logo-blue, #2563eb)" stroke-width="2.8" stroke-linecap="round"/>
    <line x1="64" y1="28" x2="64" y2="104" stroke="url(#ch36-gold-mira)" stroke-width="1.8" stroke-dasharray="3 2.5" stroke-linecap="round"/>
    <line x1="24" y1="66" x2="104" y2="66" stroke="url(#ch36-gold-mira)" stroke-width="1.8" stroke-dasharray="3 2.5" stroke-linecap="round"/>
    <circle cx="24" cy="46" r="6.6" fill="url(#ch36-node-well)" stroke="url(#ch36-ring-epi)" stroke-width="2.4"/>
    <circle cx="24" cy="46" r="2.8" fill="url(#ch36-core-epi)"/>
    <circle cx="104" cy="38" r="6.6" fill="url(#ch36-node-well)" stroke="url(#ch36-ring-gov)" stroke-width="2.4"/>
    <circle cx="104" cy="38" r="2.8" fill="url(#ch36-core-gov)"/>
    <g>
      <circle cx="64" cy="66" r="7.0" fill="var(--logo-node-bg, #e2e8f0)"/>
      <circle cx="64" cy="66" r="6.0" fill="url(#ch36-pupil)" stroke="url(#ch36-gold-mira)" stroke-width="2.2"/>
      <line x1="55.0" y1="66" x2="60.0" y2="66" stroke="url(#ch36-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
      <line x1="68.0" y1="66" x2="73.0" y2="66" stroke="url(#ch36-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
      <line x1="64" y1="57.0" x2="64" y2="62.0" stroke="url(#ch36-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
      <line x1="64" y1="70.0" x2="64" y2="75.0" stroke="url(#ch36-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
      <circle cx="64" cy="66" r="2.0" fill="url(#ch36-gold-core)"/>
    </g>
  </g>
  <g transform="translate(148, 0)">
    <text x="2" y="27" fill="var(--logo-sub, #1d4ed8)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.2">PART III · FOUNDATIONS OF JUDGMENT</text>
    <text x="498" y="27" text-anchor="end" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.0">CH36 · THRESHOLDS</text>
    <text x="0" y="59" fill="var(--logo-ink, #0f172a)" font-family="system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif" font-size="18.5" font-weight="800" letter-spacing="0.95">DECISION, ASYMMETRIC LOSS, AND REGRET</text>
    <line x1="2" y1="78" x2="384" y2="78" stroke="url(#ch36-ring)" stroke-width="1.6" stroke-opacity="0.82"/>
    <circle cx="400" cy="78" r="6.2" stroke="var(--logo-gold, #f59e0b)" stroke-width="1.5" stroke-dasharray="2.2 1.6"/>
    <circle cx="400" cy="78" r="4.0" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.2"/>
    <circle cx="400" cy="78" r="1.8" fill="var(--logo-gold, #f59e0b)"/>
    <circle cx="415" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="430" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="445" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-green, #10b981)" stroke-width="2.0"/>
    <circle cx="445" cy="78" r="1.6" fill="var(--logo-green, #10b981)"/>
    <circle cx="460" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="475" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="490" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-crimson, #e11d48)" stroke-width="2.0"/>
    <circle cx="490" cy="78" r="1.6" fill="var(--logo-crimson, #e11d48)"/>
    <text x="2" y="104" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.8" font-weight="600" letter-spacing="0.65">MINIMAX REGRET · PAUKER–KASSIRER CRITICAL ODDS · VALUE OF INFORMATION</text>
  </g>
  </svg>
</figure>

## By the end of this chapter you can

1. Separate a loss matrix from state probabilities, distinguish cost–loss from expected-regret thresholds, and specify two-tier interface disclosure separating compact live triage cues from on-demand consequence, bound, and frequency cards.
2. Partition interval and diagnostic decisions into three regimes, stating when minimax-regret probing or testing holds and when irreversibility or lethal omission bars them.
3. Bound expected information value against live alert saturation (§5), performative feedback, and incommensurability limits where thresholds fail.

## 1. Actions, states, and the loss matrix

Every decision problem combines three distinct pieces: available actions $\mathcal{A}$, possible real-world states $\mathcal{S}$, and a loss function $\mathcal{L}(a, s)$ scoring what happens when action $a$ meets state $s$ (Raiffa 1968). Statistical archives supply probabilities $\mathbb{P}(s)$ (or partial-identification intervals from [Chapter 35](35-inference-and-evidence.md)). Deciding which actions exist and how heavily each harm weighs remains an ethical and institutional choice.

A loss matrix acts as a constitutional ledger. Valuing a false alarm at ten units and a missed disaster at a thousand is not empirical measurement. It is a moral commitment written by whoever holds authority, borne by whoever suffers errors. Policy fights dressed up as statistical arguments are usually undeclared clashes over cells in that matrix.

## 2. The cost–loss threshold and the Pauker-Kassirer form

In [Chapter 01](01-deciding-without-knowing.md), we met the umbrella threshold intuitively: act whenever the cost of protection sits below expected avoided harm. Two formal thresholds share this intuition yet ask slightly different questions of the ledger.

The **cost–loss threshold** governs a preventive outlay $C > 0$ that prevents a fraction $\eta \in (0, 1]$ of avoidable disaster loss $L > 0$. At hazard probability $p$, acting costs $C + p(1 - \eta)L$ (outlay plus residual unmitigated harm), while waiting costs $p L$. Setting both expected bills equal under $\eta L > C$ gives the breakeven cutoff:

$$p^\ast\_{\mathrm{CL}} = \frac{C}{\eta L} \in (0, 1),$$

clamping at $\min(1, C/(\eta L)) = 1$ if $\eta L \le C$, where action costs more than the harm it could avert.

The **Pauker–Kassirer threshold** (Pauker & Kassirer 1975) instead compares expected regret across two classification errors: net false-positive cost $C\_{\mathrm{FP}} > 0$ when acting unnecessarily, and net false-negative loss $L\_{\mathrm{FN}} > 0$ when missing an event. Acting once expected missed-event regret exceeds expected false-alarm regret ($p\mkern3muL\_{\mathrm{FN}} > (1 - p)\mkern3muC\_{\mathrm{FP}}$) gives:

$$p^\ast\_{\mathrm{PK}} = \frac{C\_{\mathrm{FP}}}{C\_{\mathrm{FP}} + L\_{\mathrm{FN}}} = \frac{1}{1 + L\_{\mathrm{FN}}/C\_{\mathrm{FP}}} \in (0, 1).$$

Read aloud, equal error costs ($C\_{\mathrm{FP}} = L\_{\mathrm{FN}}$) put the cutoff at $1/2$, while severe missed-event harm ($L\_{\mathrm{FN}} \gg C\_{\mathrm{FP}}$) drives it toward zero. Both formulas assume **risk-neutral linear dollars** on **one shared ledger**. Under convex harm or heavy tails, pricing $p L$ at mean damage understates expected harm by Jensen's gap $\Delta\_J \ge 0$ ([Chapter 12](12-derivatives-and-convexity.md)), and no cost ratio overrides non-reciprocal rights ([Chapter 19](19-ethics-and-the-imposition-of-risk.md)) or physical survival margins ([Chapter 41](41-the-triad-and-the-minimum.md)).

### Two thresholds, two denominators

Notice why the denominators differ. Cost–loss divides by *gross avoided damage* $\eta L$ because you pay $C$ in both worlds; expected regret divides by *net error swing* $C\_{\mathrm{FP}} + L\_{\mathrm{FN}}$. Whenever $C\_{\mathrm{FP}} = C$ and net missed regret is $L\_{\mathrm{FN}} = \eta L - C > 0$, both formulas match identically: $C/(C + \eta L - C) = C/(\eta L) = p^\ast\_{\mathrm{CL}}$ ([Chapter 43](43-inquiry-openness-multiverse.md)).

To complete [Chapter 33](33-measuring-and-erring.md)'s ignorance scale $n\_0(p^\ast)$ (extending Keynes's 1921 weight of argument without attributing the binomial sample-size formula to Keynes; [Appendix A.1](appendix-a-mathematics.md)), size a local-alternative binomial draw around $p^\ast = C/(\eta L) \in (0, 1)$ to detect relative shift $\Delta p = \varepsilon\_{\mathrm{rel}} p^\ast$ ($\varepsilon\_{\mathrm{rel}} > 0$ with $p^\ast(1 + \varepsilon\_{\mathrm{rel}}) \le 1$) at error limits $\alpha, \beta \in (0, 1)$ ($\alpha + \beta < 1$): under the local constant-variance approximation $\mathrm{Var} \approx p^\ast(1 - p^\ast)$ (exact finite-shift tests split null $p^\ast$ and alternative $p\_1$), bounding $\sqrt{p^\ast(1 - p^\ast)/n\_0} \le \varepsilon\_{\mathrm{rel}} p^\ast / (z\_{1-\alpha} + z\_{1-\beta})$ yields $n\_0(p^\ast) = \bigl((z\_{1-\alpha} + z\_{1-\beta})/\varepsilon\_{\mathrm{rel}}\bigr)^2 (1 - p^\ast)/p^\ast$. Mistaking $\eta L$ for $L\_{\mathrm{FN}}$ under $\eta L > C$ shrinks the threshold to $C/(C + \eta L) < p^\ast\_{\mathrm{CL}}$ (failing to clamp at $1$ when $\eta L \le C$), inflating $n\_0$ and strictly depressing evidential weight $V = n\_{\mathrm{eff}}/(n\_{\mathrm{eff}} + n\_0)$ whenever $n\_{\mathrm{eff}} > 0$. Distinctly, **standpoint divergence** ([Chapter 32](32-the-question-before-the-data.md) §5) erupts when narrow internal fines ($L\_{\mathrm{inst}} \ll L\_{\mathrm{affected}}$) export harm onto external reception perimeters between $p^\ast\_{\mathrm{affected}}$ and $p^\ast\_{\mathrm{inst}}$.

To keep hidden premises out of time-critical rooms without violating the $\sim 10\text{ bits/s}$ conscious throughput bottleneck (§5; Zheng & Meister 2025), every risk score or alert interface presenting an estimate $\hat{p}$ must follow a two-tier **interface disclosure rule**. Synchronous live triage shows only a compact status cue (commitment regime, stressed axis, uncalibrated/out-of-support quarantine state, and algedonic or manual override) while linking in one unblocked step to an on-demand inspection card (and off-shift audit record) displaying all four structural disclosures:

1. **Consequence ratio:** which cutoff governs the trigger (cost–loss $p^\ast\_{\mathrm{CL}} = \min(1, C/(\eta L))$ or regret $p^\ast\_{\mathrm{PK}} = C\_{\mathrm{FP}}/(C\_{\mathrm{FP}} + L\_{\mathrm{FN}})$).
2. **Commitment regime:** whether the rule commands full commitment/safe abstention, a divisible reversible probe $a^\ast\_{\mathrm{MMR}} \in (0, 1)$ over $[p\_{L,t}, p\_{U,t}]$, an indivisible switch $a \in \lbrace 0, 1\rbrace$, or refusal and hazard escalation.
3. **Calibration and domain limits:** the partial-identification band $[p\_{L,t}, p\_{U,t}]$ when available, quarantining uncalibrated or out-of-support inputs instead of silently extrapolating ([Chapter 05](05-what-a-number-doesnt-say.md); [Chapter 41](41-the-triad-and-the-minimum.md)).
4. **Operational natural frequencies:** concrete local false-alarm rate ($\mathrm{FPR} = 1 - \mathrm{Spec}$) and positive predictive value ($\mathrm{PPV}$, §5).

That two-tier split keeps archive lineage (`T1`/`T3`) and rank-blind halts one step away while routing heavy calibration and numerical drilldowns off-shift (§5–§6), without turning human judgment into a checklist.

When an intermediate diagnostic test exists, costing $C\_{\mathrm{test}}$ with sensitivity $\mathrm{Sens}$ and specificity $\mathrm{Spec}$ beating chance ($\mathrm{Sens} + \mathrm{Spec} > 1$) at viable cost (§6), **Pauker and Kassirer (1980)** divide clinical choice into three zones:

$$\text{Withhold Action } (p < p^\ast\_{\mathrm{test}}) \mkern5mu\mid\mkern5mu \text{Order Diagnostic Test } (p^\ast\_{\mathrm{test}} \le p \le p^\ast\_{\mathrm{treat}}) \mkern5mu\mid\mkern5mu \text{Act Immediately } (p > p^\ast\_{\mathrm{treat}}).$$

Below $p^\ast\_{\mathrm{test}}$, even a positive test leaves risk too small to act; above $p^\ast\_{\mathrm{treat}}$, delaying treatment for lab turnaround causes worse harm.

Two practical lessons carry most decisions:

1. *Consequences set thresholds, not data.* Two experts sharing $\hat{p}$ rationally disagree if one faces cheap prevention against huge loss while the other faces steep outlays.
2. *Cutoffs rarely sit at one-half.* Cheap prophylaxis against lethal threats drops cutoffs to a few percent; waiting for "beyond reasonable doubt" before evacuating a flood plain is negligence.

Forecast quality and decision value therefore diverge: mapped through cost–loss ratios, a forecast holds zero value at extreme ratios and peaks near an interior cutoff, so higher accuracy scores need not improve choices (Richardson 2000).

## 3. Asymmetric loss, standards of proof, and Minimax Regret

Legal systems translate consequence ratios into explicit proof standards:

1. **Preponderance of the evidence:** Equal error gravity ($C\_{\mathrm{FP}} = L\_{\mathrm{FN}} > 0$) sets $p^\ast\_{\mathrm{PK}} = 1/2$, though group statistics above one-half still require individualized case weight $V(x, q) > 0$ ([Chapter 35](35-inference-and-evidence.md) §5; Cohen 1977).
2. **Clear and convincing evidence:** Weighing false sanctions at $C\_{\mathrm{FP}} = k\mkern2mu L\_{\mathrm{FN}}$ ($1 < k < 10$) lifts $p^\ast\_{\mathrm{PK}} = k/(k + 1) \in (1/2, 10/11)$.
3. **Beyond a reasonable doubt:** Blackstone's ratio (Blackstone 1765–1769), ten guilty freed rather than one innocent convicted ($C\_{\mathrm{FP}} = 10 L\_{\mathrm{FN}}$), fixes §2's bar at:
   $$p^\ast\_{\mathrm{PK}} = \frac{10}{10 + 1} = \frac{10}{11}.$$
4. **Learned Hand's negligence rule:** Judge Learned Hand (1947) defined negligence as skipping precautions whenever burden $B < PL$ (matching cost–loss with $C = B, \eta = 1$, or $P > B/L = p^\ast\_{\mathrm{CL}}$). Limited corporate liability truncates private loss to $L\_{\mathrm{private}} < L$, inflating private thresholds toward $1$ and hiding convex tail harm $\Delta\_J \ge 0$ ([Chapter 12](12-derivatives-and-convexity.md)).

What happens when probability $p\_t$ is only partially identified inside an interval $[p\_{L,t}, p\_{U,t}]$ ([Chapter 35](35-inference-and-evidence.md))? Avoidable loss spans $[L\_t, U\_t] := [p\_{L,t}\mkern2mu\eta L,\mkern3mu p\_{U,t}\mkern2mu\eta L]$. If $C < L\_t$ (or boundary $L\_t = C < U\_t$, where acting yields zero regret and weakly dominates abstention), act fully; if $U\_t \le C$, hold off safely. When the threshold falls strictly *inside* the uncertainty band ($L\_t < C < U\_t$, or $p\_{L,t} < p^\ast\_{\mathrm{CL}} < p\_{U,t}$), neither extreme dominates. **Minimax Regret (MMR)** chooses the action that keeps your worst possible post-outcome regret as small as possible across all probabilities inside $[p\_{L,t}, p\_{U,t}]$ (Wald 1945; Savage 1954; Manski 2026; Montiel Olea, Qiu, & Stoye 2025; distinct from statewise regret over $\lbrace H, \neg H\rbrace$, which ignores $[p\_{L,t}, p\_{U,t}]$).

For divisible, linear, **reversible** interventions, MMR prescribes a fractional probe $a^\ast\_{\mathrm{MMR}} \in (0, 1)$ ([Chapter 41](41-the-triad-and-the-minimum.md) §5). Notice two limits: MMR can violate Independence of Irrelevant Alternatives across three or more options (discussed in Savage 1954; Manski 2026) and needs pre-commitment for dynamic consistency. For strictly **indivisible** ($a \in \lbrace 0, 1\rbrace$), reversible steps away from ruin, deterministic MMR switches at the midpoint $(p\_{L,t} + p\_{U,t})/2 = p^\ast\_{\mathrm{CL}}$ (Manski 2026). Irreversible straddles ban fractional probing and escalate fatal-omission traps to governance ([Chapter 41](41-the-triad-and-the-minimum.md) §5). Under Ellsberg ambiguity (Ellsberg 1961), incomplete preferences wait for clear interval dominance unless reversible probing stays survivable.

### Beyond exogenous robustness: archive provenance and institutional governance

Two frameworks widen point rules under ambiguity without opening archive provenance or governance. Entropy-constrained robust control surrounds one baseline law with likelihood-ratio penalties for a single planner (Hansen & Sargent 2016), assuming absolute continuity that leaves off-support shifts unmodeled (Hansen & Sargent 2019). Distributionally Robust Optimization (DRO), surveyed via Sion duality (Rahimian & Mehrotra 2019), minimizes worst-case risk around the empirical sample. Under type-1 Wasserstein ($W\_1$) balls around empirical distribution $\hat{P}\_N$ on closed convex support $\Xi \subseteq \mathbb{R}^m$ with light exponential tails ($\mathbb{E}[\exp(\|\xi\|^a)] < \infty$, $a > 1$) and confidence-$1-\beta$ radius $\varepsilon\_N(\beta) = \bigl(\log(c\_1/\beta)/(c\_2 N)\bigr)^{1/\max\lbrace m, 2\rbrace} \le 1$ ($m \neq 2$, $N \ge \log(c\_1/\beta)/c\_2$), losses convex in the decision variable and equal to a pointwise maximum of finitely many concave (or affine) functions of uncertain vector $\xi \in \Xi \subseteq \mathbb{R}^m$ reduce to tractable convex programs (Mohajerin Esfahani & Kuhn 2018). Under $f$-divergence balls ($D\_f(Q \parallel \hat{P}\_n) \le \rho\_{\mathrm{div}}$, $Q \ll \hat{P}\_n$) with $f(t) \asymp t^k$ ($k > 1$), excess-risk rates adjust to $n^{-1/(k\_\ast \vee 2)}$ up to logarithmic factors for $k\_\ast = k/(k - 1)$ ($n \ge 1$; Duchi & Namkoong 2021). Because neither model catches selective missingness or excluded groups, **archive-provenance checks** (`T1`–`T3`) anchor $[p\_{L,t}, p\_{U,t}]$ inside the epistemic safeguard.

Robust Decisionmaking (RDM) tests scenario ensembles under minimax regret, yet leaves execution incentives unmodeled. Reviewing Savage's group-choice pitfalls, Lempert, Popper, and Bankes (2003) warn that "Participants can easily manipulate outcomes by lying about the weights they assign to alternative futures." Across 43 documents covering 37 deep-uncertainty cases, Bonjean Stanton and Roelich (2021) find 95% examine environmental conditions, only 3% policy uncertainty, 3% decision-maker subjectivity, and zero organizational framing. As King (2004) shows, explaining why individual annuities cost roughly 20% more than group plans, "A key part of managing risk is the design of institutions that determine the ultimate incidence of risk." Stochology therefore subordinates per-step probes $a^\ast\_{\mathrm{MMR}}$ to **physical survival headroom** ([Chapter 38](38-time-and-dependence.md)) and **institutional governance gates** (`T1`–`T7` across [Chapter 29](29-aim-and-the-three-planes.md) and [Chapter 41](41-the-triad-and-the-minimum.md)), closing limited-liability loopholes and spillover harm.

## 4. The Expected Value of Perfect Information (EVPI)

Before spending money or losing precious time collecting more data, ask how much perfect clairvoyance would actually lower expected loss. The **Expected Value of Perfect Information (EVPI)** equals the best expected loss acting *without* knowing state $s$, minus the average minimum loss if $s \in \mathcal{S}$ were revealed before acting:

$$\mathrm{EVPI} = \min\_{a} \mathbb{E}\_s\left[\mathcal{L}(a, s)\right] - \mathbb{E}\_s\left[\min\_{a} \mathcal{L}(a, s)\right] \ge 0,$$

always non-negative by Jensen's inequality on the concave minimum ([Chapter 12](12-derivatives-and-convexity.md)). For the binary cost–loss problem with prior hazard probability $\pi\_0 \in [0, 1]$ ([Chapter 28](28-the-machine-and-the-deception.md); [Appendix A.1](appendix-a-mathematics.md)):

$$\mathrm{EVPI}(\pi\_0) = \max\bigl(0,\mkern3mu \min(C,\mkern3mu \pi\_0\eta L) - \pi\_0\min(C,\mkern3mu \eta L)\bigr).$$

In plain words, information is worth most right where you are maximally torn: $\mathrm{EVPI}(\pi\_0)$ peaks at $C(1 - p^\ast\_{\mathrm{CL}})$ when $\pi\_0 = p^\ast\_{\mathrm{CL}}$ ($\eta L > C$), and drops to $0$ whenever $\eta L \le C$. Noisy tests always yield less ($\mathrm{EVSI} \le \mathrm{EVPI}$), and interval probabilities make $\mathrm{EVPI}$ set-valued.

Three boundaries reduce information value to zero:

1. **Dominance:** one option wins across all states (or $\pi\_0 \in \lbrace 0, 1\rbrace$), making static $\mathrm{EVPI} = 0$.
2. **Signal irrelevance:** data $X$ uncorrelated with payoff states gives $\mathrm{EVSI}(X) = 0$ even when $\mathrm{EVPI} > 0$.
3. **Deadline expiration:** if waiting for data ($\tau\_{\mathrm{collect}}$) exceeds available survival time before ruin, realizable value collapses to zero even while the static formula reads positive.

## 5. Cognitive alert filtering and the information bottleneck

Operators on live consoles never experience sensitivity ($\mathrm{TPR} := \mathrm{Sens}$) or false-alarm rate ($\mathrm{FPR} := 1 - \mathrm{Spec}$) separately. They experience **Positive Predictive Value ($\mathrm{PPV}$)**, the chance an alarm is real, at operational prevalence $\pi \in (0, 1)$ (assuming positive alert rate $\pi\mkern2mu\mathrm{Sens} + (1 - \pi)\mathrm{FPR} > 0$):

$$\mathrm{PPV} = \frac{\pi\mkern2mu\mathrm{Sens}}{\pi\mkern2mu\mathrm{Sens} + (1 - \pi)\mathrm{FPR}}, \qquad \frac{\mathrm{PPV}}{1 - \mathrm{PPV}} = \frac{\pi}{1 - \pi}\cdot\frac{\mathrm{Sens}}{\mathrm{FPR}}.$$

Read aloud, when crises are rare ($\pi \ll \mathrm{FPR}/\mathrm{Sens}$), false alarms swamp real alerts. Hospital validation of the proprietary Epic Sepsis Model across 27,697 patients and 38,455 hospitalizations measured 33% sensitivity paired with just 12% $\mathrm{PPV}$, meaning $1 - \mathrm{PPV} = 88\\%$ of alerts were false alarms, causing severe **alert fatigue** (Wong et al. 2021; [Chapter 05](05-what-a-number-doesnt-say.md)).

Human attention imposes a hard physical bottleneck on live screens. Conscious human behavioral throughput runs through a single serial channel capped near **ten bits per second** ($\sim 10\text{ bits/s}$) (Zheng & Meister 2025), a strict limit for **live time-pressured triage**, not calm off-shift study.

Why does that ten-bit ceiling turn heavy compliance screens into crisis paralysis? Reading an alert of $B\_{\mathrm{audit}}$ bits takes human service time $\tau\_{\mathrm{cog}} = B\_{\mathrm{audit}} / C\_{\mathrm{human}}$ ($C\_{\mathrm{human}} > 0$). Under alert arrival rate $\lambda\_{\mathrm{cog}}$ and arrival or service burstiness ($0 \le c\_a^2, c\_s^2 < \infty$, $c\_a^2 + c\_s^2 > 0$), pushing cognitive utilization $\rho\_{\mathrm{cog}} = \lambda\_{\mathrm{cog}}\tau\_{\mathrm{cog}} \to 1^-$ (distinct from physical queue utilization $\rho = \lambda/\mu$ in [Chapter 38](38-time-and-dependence.md)) drives deliberation queueing wait $\tau\_{\mathrm{delib}}$ upward explosively along Kingman's hyperbola ([Chapter 12](12-derivatives-and-convexity.md); formalized in [Chapter 38](38-time-and-dependence.md)), causing recovery latency to outlast survival time even before saturation ([Chapter 41](41-the-triad-and-the-minimum.md)). Translating conditional percentages into concrete **natural frequencies** speeds Bayesian understanding without statistical training (Gigerenzer & Hoffrage 1995; [Chapter 05](05-what-a-number-doesnt-say.md)), shrinking $B\_{\mathrm{audit}}$ per cue while keeping $\mathrm{FPR}$ and $\mathrm{PPV}$ clear.

Because live attention is strictly serial, consoles must ban **color-only risk cues** (always pairing shape, layout, and natural-frequency text), **competing popups**, and **unbundled sensor bursts** that overload $\lambda\_{\mathrm{cog}}$ and spill mistakes onto external reception perimeters. Alert fatigue reflects either **queue saturation** or **rational Bayesian dismissal** once post-alert $\mathrm{PPV} \le p^\ast\_{\mathrm{PK}}$ (distinct from pre-test abstention when baseline prior probability sits below $p^\ast\_{\mathrm{test}}$, §6). Sustainable consoles pair natural frequencies (Gigerenzer & Hoffrage 1995) with corroborated sequential alerts at zero rank-based silencing ([Chapter 40](40-institution-incentive-metric.md)).

## 6. The three-regime partition, the wrong recommendation and the fixed point

### The diagnostic band, derived and bounded

Recall the Edwin Smith Surgical Papyrus's three-way triage: treat directly, contend with uncertainty, or abstain ([Chapter 27](27-what-judging-is.md); Breasted 1930). Pauker and Kassirer (1980) derived exact modern cutoffs for that tripartition by comparing expected regret across three choices: $R\_{\mathrm{act}} = (1 - p)C\_{\mathrm{FP}}$ for immediate action, $R\_{\mathrm{wait}} = p\mkern3muL\_{\mathrm{FN}}$ for waiting, and $R\_{\mathrm{test}} = C\_{\mathrm{test}} + (1 - p)(1 - \mathrm{Spec})C\_{\mathrm{FP}} + p(1 - \mathrm{Sens})L\_{\mathrm{FN}}$ for testing first. Equating $R\_{\mathrm{test}} = R\_{\mathrm{wait}}$ and $R\_{\mathrm{test}} = R\_{\mathrm{act}}$ fixes the lower and upper testing edges:

$$p^\ast\_{\mathrm{test}} = \frac{C\_{\mathrm{test}} + (1 - \mathrm{Spec})C\_{\mathrm{FP}}}{\mathrm{Sens}\mkern3muL\_{\mathrm{FN}} + (1 - \mathrm{Spec})C\_{\mathrm{FP}}}, \qquad p^\ast\_{\mathrm{treat}} = \frac{\mathrm{Spec}\mkern3muC\_{\mathrm{FP}} - C\_{\mathrm{test}}}{\mathrm{Spec}\mkern3muC\_{\mathrm{FP}} + (1 - \mathrm{Sens})L\_{\mathrm{FN}}}.$$

Read both endpoints: a useless free test ($\mathrm{Sens} = \mathrm{Spec} = 1/2, C\_{\mathrm{test}} = 0$) collapses both edges to $p^\ast\_{\mathrm{PK}}$; a perfect free test ($\mathrm{Sens} = \mathrm{Spec} = 1, C\_{\mathrm{test}} = 0$) stretches testing across $(0, 1)$. For $C\_{\mathrm{FP}}, L\_{\mathrm{FN}} > 0$ and $\mathrm{Sens}, \mathrm{Spec} \in [0, 1]$, a real middle zone $0 \le p^\ast\_{\mathrm{test}} < p^\ast\_{\mathrm{PK}} < p^\ast\_{\mathrm{treat}} \le 1$ exists **if and only if** $\mathrm{Sens} + \mathrm{Spec} > 1$ and test outlay sits strictly below peak sample value at $p^\ast\_{\mathrm{PK}}$:

$$0 \le C\_{\mathrm{test}} < (\mathrm{Sens} + \mathrm{Spec} - 1)\frac{C\_{\mathrm{FP}} L\_{\mathrm{FN}}}{C\_{\mathrm{FP}} + L\_{\mathrm{FN}}} = \mathrm{EVSI}(p^\ast\_{\mathrm{PK}}).$$

Whenever $C\_{\mathrm{test}} > 0$ or the test is imperfect ($\mathrm{Sens}, \mathrm{Spec} < 1$), both bounds sit strictly inside $(0, 1)$; above that cost cap, testing is never worth running. Notice $R\_{\mathrm{test}}$ ignores waiting harm $\Delta L\_{\mathrm{delay}} > 0$ during lab turnaround. When a patient deteriorates rapidly, delay pulls $p^\ast\_{\mathrm{treat}}$ sharply downward.

### The case where the crossed threshold was the wrong decision

Clean arithmetic fails when rows are missing from the loss matrix. In February 1976, a swine-influenza outbreak at Fort Dix, New Jersey caused one death, thirteen ill soldiers, and serological signs of up to $500$ infected recruits (Neustadt & Fineberg 1978). Person-to-person spread reached over $200$ recruits from a strain resembling the 1918 pandemic virus (Sencer & Millar 2006). Pairing tiny per-dose vaccine cost $C$ against 1918-scale catastrophic mortality $L$ drove $C/(\eta L)$ near zero. The national campaign vaccinated $45$ million Americans in ten weeks (Sencer & Millar 2006), over $45$ million in federal summaries (Centers for Disease Control and Prevention 2024). It suspended on 16 December 1976 after Guillain–Barré surveillance revealed an attributable risk near $1$ in $105{,}000$ vaccinees (Neustadt & Fineberg 1978), or about one extra case per $100{,}000$ in later reviews (Centers for Disease Control and Prevention 2024).

The §2 formula worked cleanly on a broken matrix: it omitted the vaccine's own rare side-effect harm and plugged a worst-case 1918 severity number in place of a probability distribution. Adding an intervention's own adverse risk $\lambda > 0$ directly raises cost ($C \mapsto C + \lambda$, [Chapter 43](43-inquiry-openness-multiverse.md)), lifting $C\_{\mathrm{FP}}$ while shrinking net benefit $L\_{\mathrm{FN}}$; omitting adverse tails manufactures a falsely favorable cutoff (Neustadt & Fineberg 1978).

### When the threshold moves the probability it is compared against

Splitting odds from costs fails whenever announcing a cutoff induces $p = \Psi(p^\ast)$ or retraining shifts $\mathcal{D}(\theta)$ (Perdomo et al. 2020; [Chapter 37](37-model-fit-and-error.md) §3; [Chapter 48](48-reactive-worlds-performativity-goodhart.md)). Solving that **fixed-point loop** contracts only under tight Lipschitz reactivity bounds and runs **off-shift** ([Chapter 29](29-aim-and-the-three-planes.md) §4). Sealing commitments before outcomes prevents post-hoc loss tampering without blocking frontline halts or attested `T1` reopening ([Chapter 41](41-the-triad-and-the-minimum.md)).

Mervyn King (2010) captures why institutional feedback differs from physics: in engineering, "there is no-one listening," whereas in institutions, "Outcomes are endogenous to beliefs about the likelihoods of the outcomes." Announcing rigid rules can destabilize systems: post-*Titanic* mandates requiring extra lifeboats made the *S.S. Eastland* top-heavy, capsizing on Lake Michigan with around 850 lives lost, while central-bank discount-window borrowing had to be reported as lagged quarterly averages to stop runs (King 2010). Because respondents' 90% subjective confidence intervals miss the true value half the time, coarse probability ribbons tied to pre-committed rules outperform brittle point triggers (King 2010).

### When the threshold should not be used

Cost–loss formulas stop applying when loss cannot be priced in money, trading budgets against non-negotiable human rights ([Chapter 19](19-ethics-and-the-imposition-of-risk.md)), or when feedback refuses to stabilize. State the incommensurability openly, apply the protective constraint directly, and log why the threshold was withheld; even in medicine, weighing competing harms stays an ethical judgment, not a sensor reading (Pauker & Kassirer 1975).

## Bridge

Drawing on [Chapter 32](32-the-question-before-the-data.md)'s framing cutoffs and [Chapter 35](35-inference-and-evidence.md)'s partial-identification intervals, this chapter assembled cost–loss and expected-regret thresholds (§2), synchronous live interface disclosure, Minimax Regret probing (§3), `EVPI` (§4), cognitive alert queuing at ten bits per second (§5), and endogenous feedback limits (§6). [Chapter 41](41-the-triad-and-the-minimum.md) completes the three-regime lexicographic operator, while [Chapter 48](48-reactive-worlds-performativity-goodhart.md) and [Chapter 49](49-moral-luck-apt-agency-institutions.md) formalize performativity and cognitive queues. Thresholds still depend on fitted models whose error, leakage, and discrepancy open next in [Chapter 37](37-model-fit-and-error.md), *Model, Fit, and Error*.
