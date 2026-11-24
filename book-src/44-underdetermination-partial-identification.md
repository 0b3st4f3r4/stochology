# Underdetermination, Partial Identification, and Severe Testing

<figure class="stoch-figure stoch-logo-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="800" height="132" viewBox="0 0 800 132" fill="none" role="img" aria-label="Stochology CH44 · S_EPI — Partial Identification And Bounds">
  <defs>
    <linearGradient id="ch44-ring" x1="2" y1="78" x2="386" y2="78" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="38%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="68%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <radialGradient id="ch44-glow" cx="50%" cy="50%" r="50%">
      <stop offset="0%" stop-color="#38bdf8" stop-opacity="0.24"/>
      <stop offset="48%" stop-color="#2563eb" stop-opacity="0.14"/>
      <stop offset="80%" stop-color="#e11d48" stop-opacity="0.08"/>
      <stop offset="100%" stop-color="#090e1a" stop-opacity="0"/>
    </radialGradient>
    <linearGradient id="ch44-triad-stroke" x1="18" y1="24" x2="110" y2="104" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.85"/>
      <stop offset="50%" stop-color="var(--logo-gold, #f59e0b)" stop-opacity="0.90"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.85"/>
    </linearGradient>
    <linearGradient id="ch44-fill" x1="22" y1="30" x2="106" y2="98" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)" stop-opacity="0.26"/>
      <stop offset="48%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.28"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.26"/>
    </linearGradient>
    <linearGradient id="ch44-ink-curve" x1="20" y1="66" x2="108" y2="66" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-ink, #0f172a)"/>
      <stop offset="50%" stop-color="var(--logo-sub, #1d4ed8)"/>
      <stop offset="100%" stop-color="var(--logo-ink, #0f172a)"/>
    </linearGradient>
    <radialGradient id="ch44-pupil" cx="45%" cy="42%" r="58%">
      <stop offset="0%" stop-color="#020617"/>
      <stop offset="68%" stop-color="#0f172a"/>
      <stop offset="100%" stop-color="#451a03"/>
    </radialGradient>
    <linearGradient id="ch44-gold-mira" x1="54" y1="48" x2="84" y2="82" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#fef08a"/>
      <stop offset="45%" stop-color="#fbbf24"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <radialGradient id="ch44-gold-core" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#ffffff"/>
      <stop offset="35%" stop-color="#fde047"/>
      <stop offset="100%" stop-color="#d97706"/>
    </radialGradient>
    <radialGradient id="ch44-node-well" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="var(--logo-well-hi, var(--elevated-bg, #ffffff))"/>
      <stop offset="100%" stop-color="var(--logo-node-bg, #e2e8f0)"/>
    </radialGradient>
    <linearGradient id="ch44-ring-epi" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#7dd3fc"/>
      <stop offset="50%" stop-color="#2563eb"/>
      <stop offset="100%" stop-color="#1e3a8a"/>
    </linearGradient>
    <radialGradient id="ch44-core-epi" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#bae6fd"/>
      <stop offset="45%" stop-color="#3b82f6"/>
      <stop offset="100%" stop-color="#1d4ed8"/>
    </radialGradient>
    <linearGradient id="ch44-ring-vital" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#6ee7b7"/>
      <stop offset="50%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#065f46"/>
    </linearGradient>
    <radialGradient id="ch44-core-vital" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#a7f3d0"/>
      <stop offset="45%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#047857"/>
    </radialGradient>
    <linearGradient id="ch44-ring-gov" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#fda4af"/>
      <stop offset="50%" stop-color="#e11d48"/>
      <stop offset="100%" stop-color="#881337"/>
    </linearGradient>
    <radialGradient id="ch44-core-gov" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#fecdd3"/>
      <stop offset="45%" stop-color="#f43f5e"/>
      <stop offset="100%" stop-color="#be123c"/>
    </radialGradient>
  </defs>
  <g>
    <circle cx="64" cy="66" r="48" fill="url(#ch44-glow)"/>
    <circle cx="64" cy="66" r="42" stroke="var(--logo-ticks, #64748b)" stroke-width="1.2" stroke-dasharray="3 3" stroke-opacity="0.45"/>
    <rect x="20" y="52" width="88" height="28" rx="4" fill="var(--logo-blue, #2563eb)" fill-opacity="0.16" stroke="var(--logo-blue, #2563eb)" stroke-width="2.0" stroke-dasharray="4 2.5"/>
    <rect x="40" y="46" width="48" height="40" rx="4" fill="var(--logo-green, #10b981)" fill-opacity="0.22" stroke="var(--logo-green, #10b981)" stroke-width="2.6"/>
    <circle cx="20" cy="66" r="6.6" fill="url(#ch44-node-well)" stroke="url(#ch44-ring-epi)" stroke-width="2.4"/>
    <circle cx="20" cy="66" r="2.8" fill="url(#ch44-core-epi)"/>
    <circle cx="108" cy="66" r="6.6" fill="url(#ch44-node-well)" stroke="url(#ch44-ring-gov)" stroke-width="2.4"/>
    <circle cx="108" cy="66" r="2.8" fill="url(#ch44-core-gov)"/>
    <g>
      <circle cx="64" cy="66" r="7.0" fill="var(--logo-node-bg, #e2e8f0)"/>
      <circle cx="64" cy="66" r="6.0" fill="url(#ch44-pupil)" stroke="url(#ch44-gold-mira)" stroke-width="2.2"/>
      <line x1="55.0" y1="66" x2="60.0" y2="66" stroke="url(#ch44-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
      <line x1="68.0" y1="66" x2="73.0" y2="66" stroke="url(#ch44-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
      <line x1="64" y1="57.0" x2="64" y2="62.0" stroke="url(#ch44-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
      <line x1="64" y1="70.0" x2="64" y2="75.0" stroke="url(#ch44-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
      <circle cx="64" cy="66" r="2.0" fill="url(#ch44-gold-core)"/>
    </g>
  </g>
  <g transform="translate(148, 0)">
    <text x="2" y="27" fill="var(--logo-sub, #1d4ed8)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.2">PART IV · THE FRAMEWORK</text>
    <text x="498" y="27" text-anchor="end" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.0">CH44 · S_EPI</text>
    <text x="0" y="59" fill="var(--logo-ink, #0f172a)" font-family="system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif" font-size="20.2" font-weight="800" letter-spacing="1.15">PARTIAL IDENTIFICATION AND BOUNDS</text>
    <line x1="2" y1="78" x2="384" y2="78" stroke="url(#ch44-ring)" stroke-width="1.6" stroke-opacity="0.82"/>
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
    <text x="2" y="104" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.8" font-weight="600" letter-spacing="0.65">LALONDE BENCHMARK · MANSKI AND LEE ATE BOUNDS · PETERSON AND VILLE E-PROCESS</text>
  </g>
  </svg>
</figure>

## By the end of this chapter you can

1. Derive two-arm worst-case bounds under marginal randomized or unconfounded assignment from the law of total expectation, explain why their width persists as sample size grows, and contrast them with covariate-conditional unconfoundedness and observational treatment selection.
2. Separate identification uncertainty from sampling uncertainty, distinguish the Imbens–Manski parameter coverage from uniform set coverage, and state the conditions for anytime-valid *e*-processes and pre-allocated multiplicity deflation.
3. Route an identified interval to abstention, divisible minimax regret, or irreversible-harm threshold escalation while auditing support ceilings and model refutability.

An inquiry can be honestly framed and still underdetermine its answer. Within **Mediating Filter I · Axis 1 ($S\_{\mathrm{epi}}$: Epistemic Margin)**, **Guard Site $T\_2$** (*Test for severity*, crossing Archive $P\_2 \to$ World $P\_1$; [Chapter 29](29-aim-and-the-three-planes.md)) picks up where framing at $T\_1$ left off ([Chapter 43](43-inquiry-openness-multiverse.md)).

Here **Tension 2** confronts underdetermination by incomplete data (Duhem, 1906; Quine, 1951; Stanford, 2006) using severe sequential testing and partial identification (Ville, 1939; Popper, 1959; Lakatos, 1970; Peterson, 1976; Manski, 1990, 2003; Imbens & Manski, 2004; Lee, 2009; Mayo, 2018; Ramdas et al., 2023).

In [Chapter 42](42-sequential-loop-naive-agent.md), assuming missing observations are *missing completely at random* and plugging in the raw mean $\bar{O}\_t$ triggers **Collapse Proposition 1 (*Blind Undersizing*)**. **Canonical Operator I** rejects that shortcut: it refuses unverified point guesses and quarantines broken intervals ($L\_t > U\_t$ or non-finite values) as `Unparsable`. Charles Manski called this discipline **the Law of Decreasing Credibility**: inference loses credibility whenever unverified assumptions tighten. To stop analysts from hiding behind absurdly wide bands, Guard Site $T\_2$ pairs with **Reflexive Brake 1** ([Chapter 50](50-methodological-families-reflexivity.md)) using pre-committed rejection rules sized for bounded human attention ([Chapter 49](49-moral-luck-apt-agency-institutions.md)). Meanwhile, the weakest-axis score $M\_{\mathrm{Leontief}}$ and the retrospective co-diagnostic pair $\mathcal{D}\_{\mathrm{diag}}$ remain strictly ex-post trajectory diagnostics ([Chapter 41](41-the-triad-and-the-minimum.md)).

---

## 3.1 Anytime-Valid Betting Martingales ($e$-Processes) and Lakatos's Shift Ratio

Standard fixed-horizon $p$-values break down as soon as teams peek at incoming data or stop an audit early. Anytime-valid *e*-processes fix that flaw by framing statistical testing as a fair sequential bet against a null model (Ville, 1939; Shafer, 2021; Ramdas et al., 2023; Grünwald et al., 2024; [Chapter 30](30-probability-as-language.md); [Chapter 35](35-inference-and-evidence.md)).

At each shift $i$, an analyst stakes current wealth on a non-negative multiplier $e\_i$ whose expected payoff under any null law $P \in \mathcal{P}\_0$, given past history $\mathcal{F}\_{i-1}$, is at most one: $\sup\_{P \in \mathcal{P}\_0}\mathbb{E}\_P[e\_i \mid \mathcal{F}\_{i-1}] \le 1$. Cumulative evidence wealth $E\_t = \prod\_{i=1}^t e\_i$ starts at $E\_0 = 1$ and remains a test supermartingale satisfying $\sup\_{P \in \mathcal{P}\_0}\mathbb{E}\_P[E\_\tau] \le 1$ at any stopping time $\tau$. By **Ville's Inequality**, the **time-uniform confidence sequence** $\lbrace \theta : E\_t(\theta) < 1/\alpha \rbrace$ holds at error level $\alpha$ no matter when you choose to stop or look (Howard et al., 2021).

How should sequential hypothesis testing budget pre-specified refinements without inviting *post-hoc* rescue tricks? Lakatos (1970) separated **progressive** research programs from **degenerating** ones by checking whether auxiliary patches predict novel, **independently corroborated** facts rather than inventing content-decreasing linguistic reinterpretations. Stochology keeps the statistical multiplicity guard distinct from that semantic criterion. It caps auxiliary refinements at a finite budget $K\_{\max} \in \mathbb{N}\_0$ declared at $t = 0$ and splits initial wealth $E\_0 = 1$ uniformly across the baseline hypothesis and its $K\_{\max}$ pre-listed variants:

$$E\_t^{\mathrm{adj}} = \frac{E\_t}{1 + K\_{\max}}$$

In plain words, pre-allocating equal initial stake at $t = 0$ across $1 + K\_{\max}$ pre-listed variants applies a uniform union-bound dilution by $1 + K\_{\max}$ while keeping weights summing to $1$, so Ville's inequality stays valid intact. Notice its strict limits: dividing by $1 + K\_{\max}$ neither establishes Lakatosian empirical progressiveness (which still demands excess empirical content and independent corroboration) nor prevents event-algebra contraction on its own. Only append-only custody ($\mathcal{F}\_t \subseteq \mathcal{F}\_{t+1}$) blocks opportunistic shrinkage.

By contrast, inventing adaptive tweaks along the way after peeking (say, with dynamic weights $(1 + k)^{-1}$ across steps $k = 0, \dots, K$) produces a harmonic sum $H\_{K+1} = \sum\_{k=0}^K (1 + k)^{-1} > 1$, breaking Ville's bound for every finite $K \ge 1$. Unannounced *post-hoc* patches forfeit anytime Type I error control unless budgeted at $t = 0$ inside a summable series. When a core premise truly fails, **attested counter-inductive reopening** ($\mathcal{F}\_t \subsetneq \mathcal{F}\_{t+1}^\ast$; Feyerabend, 1970; Canguilhem, 1943/1966; *anagnórisis* in §3.5) retires the falsified premise openly under [Chapter 50](50-methodological-families-reflexivity.md) while keeping full diagnostic memory, and initiates a **fresh out-of-sample *e*-process** from $E\_0^{\mathrm{new}} = 1$ on newly sealed data rather than dividing by $1 + K\_{\max}$ or tampering with the retired sequence.

---

## 3.2 Non-Parametric Partial Identification: Manski, Imbens–Manski, Lee, and Peterson

Instead of fabricating missingness assumptions to force a crisp point estimate, **Canonical Operator I** computes **sharp non-parametric bounds**. Each estimator below isolates what the archive proves on its own from what hinges on untestable guesses.

### Manski (1990, 2003) Two-Arm ATE Bounds Under Attrition vs. Observational Selection

Rather than inventing missingness assumptions for an Average Treatment Effect ($\mathrm{ATE}$), Manski bounds unknown missing outcomes using physical support limits.

Suppose potential outcomes lie inside known physical limits $Y(1), Y(0) \in [y\_{\min}, y\_{\max}]$ ($y\_{\min} \le y\_{\max}$) under **marginal** randomized or unconfounded assignment $D \in \lbrace 0, 1 \rbrace$ ($D \perp (Y(1), Y(0))$) with overlap $0 < \mathbb{P}(D = 1) < 1$. When unconfoundedness holds only conditionally on covariates ($D \perp (Y(1), Y(0)) \mid X$ with conditional positivity $0 < \mathbb{P}(D = 1 \mid X) < 1$), apply the arm bounds within each covariate stratum $X = x$ first and integrate over $\mathbb{P}(X)$ rather than plugging marginal arm averages across confounded groups ([Appendix A](appendix-a-mathematics.md) §A.1). Let $R \in \lbrace 0, 1 \rbrace$ record whether a unit is observed (distinct from physical reserve stock $R\_t$), with arm retention share $p\_{\mathrm{obs}, d} = \mathbb{P}(R = 1 \mid D = d) \in [0, 1]$. By convention, $p\_{\mathrm{obs}, d}\bar{Y}\_{\mathrm{obs}, d} := \mathbb{E}[R\mkern1mu Y \mid D = d] = 0$ when $p\_{\mathrm{obs}, d} = 0$. Conditioning cleanly on assigned arm $D = d$ via marginal independence and the Law of Total Expectation splits each arm's true mean into an observed piece and an unseen piece:
$$\mathbb{E}[Y(d)] = \mathbb{E}[Y(d) \mid D = d] = \mathbb{E}[R\mkern1mu Y \mid D = d] + \mathbb{E}[(1 - R)\mkern1mu Y(d) \mid D = d],$$
which for interior retention $p\_{\mathrm{obs}, d} \in (0, 1)$ equals $p\_{\mathrm{obs}, d}\mkern3mu\mathbb{E}[Y(d) \mid R = 1, D = d] + (1 - p\_{\mathrm{obs}, d})\mkern3mu\mathbb{E}[Y(d) \mid R = 0, D = d]$ (avoiding probability-zero conditioning at $p\_{\mathrm{obs}, d} \in \lbrace 0, 1 \rbrace$).

In plain words, the true mean equals the observed average weighted by the fraction seen, plus whatever the unseen fraction averaged weighted by the missing share. Next, plug the worst-case floor $y\_{\min}$ and ceiling $y\_{\max}$ in for the unseen outcome inside $\mathbb{E}[(1 - R)Y(d) \mid D = d]$. That substitution yields Manski's sharp single-arm bounds $L\_{\mathrm{M}}(d)$ and $U\_{\mathrm{M}}(d)$:
$$L\_{\mathrm{M}}(d) = p\_{\mathrm{obs}, d}\mkern3mu\bar{Y}\_{\mathrm{obs}, d} + (1 - p\_{\mathrm{obs}, d})\mkern3muy\_{\min}, \qquad U\_{\mathrm{M}}(d) = p\_{\mathrm{obs}, d}\mkern3mu\bar{Y}\_{\mathrm{obs}, d} + (1 - p\_{\mathrm{obs}, d})\mkern3muy\_{\max}$$

In plain words, if $20\%$ of units are missing, we replace their unknown outcomes first with $y\_{\min}$ and then with $y\_{\max}$. To bound $\mathrm{ATE} = \mathbb{E}[Y(1)] - \mathbb{E}[Y(0)]$, cross-subtract opposite extremes across arms to build $[\Delta\_L, \Delta\_U]$:
$$\Delta\_L = L\_{\mathrm{M}}(1) - U\_{\mathrm{M}}(0), \qquad \Delta\_U = U\_{\mathrm{M}}(1) - L\_{\mathrm{M}}(0)$$

In plain words, the worst-case lower bound pairs the lowest plausible treated outcome with the highest plausible control outcome, and the upper bound flips the pair. Subtracting $\Delta\_L$ from $\Delta\_U$ gives the **identification width** $\Delta\_{\mathrm{id}}$:
$$\Delta\_{\mathrm{id}} = \Delta\_U - \Delta\_L = (y\_{\max} - y\_{\min})\bigl[(1 - p\_{\mathrm{obs}, 1}) + (1 - p\_{\mathrm{obs}, 0})\bigr]$$

In plain words, width equals the full outcome range scaled by the combined missing fractions of both arms. Because $\Delta\_{\mathrm{id}}$ measures **structural underdetermination**, it never vanishes as sample size $n \to \infty$. Even with complete reporting ($R \equiv 1$) in an observational study without randomization, missing counterfactuals still leave a worst-case width of $y\_{\max} - y\_{\min}$ (Manski, 1990; [Chapter 35](35-inference-and-evidence.md)). Random sampling noise around $[\Delta\_L, \Delta\_U]$ is bounded separately below.

### David S. Lee's (2009) Sharp Trimming Bounds Under Differential Attrition

When treatment keeps more people in a program than control, can we narrow those wide worst-case bounds without parametric guesses? Lee's (2009) bounds do so by comparing only the *always-observed* core group, trimming off the excess survivors from the larger arm's best or worst outcomes.

Assume unconfounded assignment ($D \perp (Y(1), Y(0), R(1), R(0))$) with overlap $0 < \mathbb{P}(D = 1) < 1$ (or conditionally within strata $X = x$ with $0 < \mathbb{P}(D = 1 \mid X) < 1$ prior to averaging over $\mathbb{P}(X)$) and monotone retention ($R(1) \ge R(0)$ almost surely, so treatment never drives out someone who stays under control). Require **strictly positive retention in both arms** ($p\_{\mathrm{obs}, 1} \ge p\_{\mathrm{obs}, 0} > 0$) and a continuous treated outcome distribution. The excess treated retention fraction $q$ is:
$$q = \frac{p\_{\mathrm{obs}, 1} - p\_{\mathrm{obs}, 0}}{p\_{\mathrm{obs}, 1}} \in [0, 1)$$

In plain words, $q$ is the share of treated participants who stayed only because they received treatment. We trim either the top $q$-quantile or the bottom $q$-quantile tail of $Y \mid D = 1, R = 1$, then subtract the observed control mean $\mathbb{E}[Y \mid D = 0, R = 1]$. That produces sharp bounds $[L\_{\mathrm{Lee}}, U\_{\mathrm{Lee}}]$ for the average effect on the *always-observed* stratum $\lbrace R(1) = R(0) = 1 \rbrace$ (Lee, 2009). When retention is balanced ($q = 0$), Lee's bounds collapse to a point, whereas Manski's $[\Delta\_L, \Delta\_U]$ bounds the whole population; the two targets match only if treatment effects are identical across selection strata.

### Arthur V. Peterson Jr.'s (1976) Competing-Risks Net-Survival and Censoring Bounds

In medical or equipment survival records, units vanish from follow-up either because they failed from another cause or because monitoring ended early. Treating dropouts as identical to survivors assumes independent censoring, a premise the archive can never verify.

Under competing risks with no simultaneous ties, the **Fréchet–Hoeffding** and **Peterson (1976)** bounds ([Chapter 31](31-distribution-and-moment.md); [Chapter 35](35-inference-and-evidence.md)) bracket latent net survival $S\_j(t)$ using observable crude survival $S(t)$ and cumulative failure incidence $P\_j^\ast(t)$ via $S(t) \le S\_j(t) \le 1 - P\_j^\ast(t)$. Consider a finite cohort of $n > 0$ units tracked to horizon $t$ with $d(t) \ge 0$ observed failures and $c(t) \ge 0$ censored units ($d(t) + c(t) \le n$). Without assuming independent censoring, non-parametric survival bounds $S\_{\mathrm{lower}}(t)$ and $S\_{\mathrm{upper}}(t)$ are:
$$S\_{\mathrm{lower}}(t) = 1 - \frac{d(t) + c(t)}{n}, \qquad S\_{\mathrm{upper}}(t) = 1 - \frac{d(t)}{n}$$

In plain words, the lower bound assumes every censored unit failed immediately, while the upper bound assumes every censored unit survived past horizon $t$. Any method claiming **point identification** (such as naive Kaplan–Meier curves) relies on an unverifiable independent-censoring assumption.

### Imbens–Manski Interval Coverage and the Mayo–Manski Severe Boundary Bridge

Real archives suffer from both missingness (identification width) and finite sample size (sampling noise). How do we combine both into one honest confidence interval that still connects to severe hypothesis tests?

Let $[\hat{\theta}\_L, \hat{\theta}\_U]$ estimate the identified interval $[\theta\_L, \theta\_U]$ with estimated width $\hat{\Delta}\_{\mathrm{id}} = \hat{\theta}\_U - \hat{\theta}\_L \ge 0$ and positive **estimated endpoint standard deviations** $\hat{\sigma}\_L, \hat{\sigma}\_U > 0$ (distinct from standard errors $\hat{\sigma}/\sqrt{n\_{\mathrm{eff}}^{\mathrm{var}}}$), setting $\hat{\sigma}\_{\max} = \max(\hat{\sigma}\_L, \hat{\sigma}\_U) > 0$. Recall from [Chapter 45](45-world-to-archive-big-data-paradox.md) the variance-corrected effective sample size $n\_{\mathrm{eff}}^{\mathrm{var}} > 0$, which equals raw $n$ only under unclustered independent draws.

We apply the **Imbens & Manski (2004)** adaptive interval $[L\_{\mathrm{IM}}, U\_{\mathrm{IM}}]$ for **pointwise asymptotic coverage of the true parameter** $\theta\_0 \in [\theta\_L, \theta\_U]$ at confidence level $1 - \alpha$ under uniform asymptotic normality and superefficiency regularity conditions, keeping it distinct from **uniform coverage of the entire identified set** $[\theta\_L, \theta\_U]$, which uses the wider two-sided $z\_{1-\alpha/2}$ endpoints ([Chapter 35](35-inference-and-evidence.md); Stoye, 2009, 2021). Note a subtle regularity limit: in two-arm Manski attrition with non-degenerate bounded support ($y\_{\max} > y\_{\min}$) and positive endpoint variance, $\Delta\_{\mathrm{id}} = 0$ occurs only at complete retention ($p\_{\mathrm{obs},1} = p\_{\mathrm{obs},0} = 1$), whereas general overlapping bounds require pre-tested intervals (Stoye, 2009) or never-empty constructions such as Stoye's (2021) $\mathrm{CI}^{\mathrm{MA}}$.

This interval bridges partial identification directly to Deborah Mayo's (2018) severe testing ([Chapter 43](43-inquiry-openness-multiverse.md)). Once the sampling design clears **statistical adequacy** (Spanos, 2021) at error level $\alpha \in (0, 1/2)$ and $n\_{\mathrm{eff}}^{\mathrm{var}}$ against a pre-registered substantive effect margin $\delta\_{\min} > 0$, a directional claim $\mu > \mu\_0$ clears the safety margin with severity when $L\_{\mathrm{IM}} > \mu\_0 + \delta\_{\min}$ (blocking trivial large-$n$ discrepancies near $\mu\_0$). Crucially, under valid randomization/unconfoundedness and true physical limits $[y\_{\min}, y\_{\max}]$, **internal MNAR sample attrition** ($\rho\_{R,Y} \ne 0$ among recorded vs. unrecorded eligible subjects) stays validly bracketed inside Manski's sharp bounds; what defeats severity no matter how high $L\_{\mathrm{IM}}$ sits is **unaccounted frame exclusion** (*Not askable* strata $\pi\_i \equiv 0$, [Chapter 45](45-world-to-archive-big-data-paradox.md)), unadjusted treatment confounding, understated support ceilings $[y\_{\min}, y\_{\max}]$, or statistical model inadequacy.

### Parametric Selection Assumptions vs. the Law of Decreasing Credibility

Classical sample-selection econometrics trades wide non-parametric bounds for sharp point numbers by assuming unobserved outcomes follow clean parametric shapes. Framing selection as omitted-variable bias, Heckman (1977) proved that without assumptions on the latent outcome error $U\_1$, even the *sign* of selection bias cannot be identified. While series methods handle weaker conditional-mean axioms, Heckman's two-stage estimator achieves point identification by assuming $(U\_{1i}, U\_{2i})$ are jointly bivariate normal and inserting the inverse Mills ratio $\lambda\_{\mathrm{Mills}}(Z\_i^\top \gamma / \Sigma\_{22}^{1/2})$, evaluated at the standardized probit index and scaled by $\Sigma\_{12}/\Sigma\_{22}^{1/2}$ (an unverified distributional guess).

Exploring the middle ground, Ho and Rosen (2015/2016) illustrate Manski's Law of Decreasing Credibility: tighter identifying axioms erode credibility, and even mild stochastic-dominance assumptions between observed and missing wages break in Roy selection models when potential wages and reservation thresholds move together.

Between worst-case bounds and Gaussian normality sit **distributionally robust optimization** ($\mathrm{DRO}$) ambiguity sets (Rahimian & Mehrotra, 2019). These optimize $\inf\_{x \in \mathcal{X}} \sup\_{P \in \mathcal{P}} \mathbb{E}\_P[h(x, \tilde{\xi})]$ over a neighborhood around the empirical law $\hat{P}\_N$:

- **$1$-Wasserstein balls** $\mathcal{B}\_{\varepsilon\_N}(\hat{P}\_N) = \lbrace Q : d\_{W\_1}(Q, \hat{P}\_N) \le \varepsilon\_N \rbrace$ (order $p = 1$ under ground norm $\Vert\cdot\Vert$) yielding tractable convex reductions for losses convex in $x$ and pointwise maxima of finitely many concave or affine functions in $\tilde{\xi}$, with radius $\varepsilon\_N(\beta) = \bigl(\log(c\_1/\beta)/(c\_2 N)\bigr)^{1/\max\lbrace m, 2 \rbrace} \le 1$ for $N \ge \log(c\_1/\beta)/c\_2$ ($m \ne 2$, confidence $1 - \beta$) under light exponential tails (Mohajerin Esfahani & Kuhn, 2018).
- **$f$-divergence balls** $\lbrace Q \ll \hat{P}\_N : D\_f(Q \parallel \hat{P}\_N) \le \rho\_{\mathrm{div}} \rbrace$ where growth $f(t) \asymp t^k$ as $t \to \infty$ ($k \in (1, \infty)$, dual $k\_\ast = k/(k-1)$) gives minimax rates $N^{-1/(k\_\ast \vee 2)}$ up to logarithmic factors (Duchi & Namkoong, 2021).

Yet empirical $\mathrm{DRO}$ leaves three structural gaps that Operators I–III must cover. First, when non-random selection shifts $\hat{P}\_N$ away from the world ($\rho\_{R,Y} \ne 0$, meaning recording correlates with need; [Chapter 45](45-world-to-archive-big-data-paradox.md)), empirical concentration balls miss the true law. Second, $f$-divergence absolute continuity ($Q \ll \hat{P}\_N$) puts zero mass on unrecorded groups or heavy-tail jumps outside observed support. Third, performative feedback ([Chapter 48](48-reactive-worlds-performativity-goodhart.md)) shifts the target distribution once a policy is deployed.

---

## 3.3 Equipping the Agent with the Epistemic Margin

Return to ***The Community and the Reserve*** ([Chapter 42](42-sequential-loop-naive-agent.md)): **Canonical Operator I** turns observed clusters and known unrecorded households into avoidable-loss bounds $[L\_t, U\_t]$ on the scale of $C$ ([Chapter 43](43-inquiry-openness-multiverse.md)), keeping scalar $[L\_{\mathrm{IM}}, U\_{\mathrm{IM}}]$ separate for parameter inference.

Three regimes break list attrition bounds alone (leading to physical support anchoring in §3.5):

1. **Not askable strata ($\pi\_i \equiv 0$):** Groups erased by eligibility rules leave no missing-unit count in any list and spill unrecorded harm across external reception perimeters (scored ex post by $\Theta\_{\mathrm{basin}}$; [Chapter 43](43-inquiry-openness-multiverse.md); [Chapter 45](45-world-to-archive-big-data-paradox.md)).
2. **Heavy-tail excursions ($\alpha \le 2$):** Power-law demand surges outrun calm-period support limits $[y\_{\min}, y\_{\max}]$ ([Chapter 46](46-non-identical-asymmetry-heavy-tails.md)).
3. **Bitemporal right-truncation ($t\_{\mathrm{ev}} \le \tau < t\_{\mathrm{disp}}$):** Delayed records leave recent distress invisible in unsealed administrative vintages ([Chapter 45](45-world-to-archive-big-data-paradox.md)).

---

## 3.4 The LaLonde Benchmark and the Bounds It Supports

Robert LaLonde's (1986) evaluation of the National Supported Work (NSW) job-training program remains the landmark test of identification failure. Comparing treated trainees against randomized controls shows a positive earnings gain. Pairing those exact same treated trainees with broad non-experimental comparison extracts (`CPS-1` or `PSID-1`) flips the estimated effect negative, while narrower survey extracts (`PSID-3`) stay positive for fragile sample-selection reasons. Reanalyzing those datasets, Dehejia and Wahba (1999) showed that recovering the experimental benchmark requires strong, **non-refutable conditional ignorability and common-support assumptions** over rich pre-treatment earnings histories, not bigger comparison files.

The literature carefully separates two distinct selection settings:

- **Observational comparison frames:** NSW records include zero earnings for unemployed workers ($R \equiv 1$). Without an untestable unconfoundedness premise, counterfactual outcomes stay partially identified at worst-case width $y\_{\max} - y\_{\min}$.
- **Randomized trials with missing outcomes:** **Lee's (2009)** trimming applies to a randomized trial with positive-outcome selection (such as Lee's wage analysis among employed *Job Corps* workers), trimming excess retention $q$ under $R(1) \ge R(0)$ and $p\_{\mathrm{obs}, 1} \ge p\_{\mathrm{obs}, 0} > 0$ to bound the *always-employed* stratum. Two-arm **Manski bounds**, by contrast, cover randomized-arm attrition across $[y\_{\min}, y\_{\max}]$ for the entire trial cohort.

Quantitative benchmark numbers for LaLonde/NSW (Dehejia & Wahba 1999) appear in [Chapter 59](59-layer-experimental-sampling-causal-order.md), while *Job Corps* figures remain in Lee (2009). Pooling mismatched observational records never manufactures experimental identification without an explicit, defended causal premise.

### The width is the result

Refusing unverified point assumptions makes the interval plus its generating premise the auditable deliverable, pricing ignorance rather than erasing it.

### When the bound is the decision

Under asymmetric stakes, intervals route directly through [Chapter 36](36-decision-loss-threshold.md) and [Chapter 41](41-the-triad-and-the-minimum.md), recording both the cutoff and the bound or minimax-regret share governing choice.

### One discipline, three instruments

Stronger assumptions shrink width while looser selection or stopping bounds widen it, keeping width an auditable unit of account.

## 3.5 The width at both ends, and the rule that arrives after the harm

### Two limits of the identification width

Under marginal randomized or unconfounded assignment ($0 < \mathbb{P}(D = 1) < 1$), identification width $\Delta\_{\mathrm{id}} = (y\_{\max} - y\_{\min})\bigl[(1 - p\_{\mathrm{obs},1}) + (1 - p\_{\mathrm{obs},0})\bigr]$ spans two extremes. It hits zero (point identification) at complete reporting ($p\_{\mathrm{obs},1} = p\_{\mathrm{obs},0} = 1$) and stretches to $2(y\_{\max} - y\_{\min})$ when zero units report, scaling linearly with missing arm shares rather than sample count $n$. Meanwhile, the Imbens–Manski critical value $c\_\alpha$ for pointwise parameter coverage shifts smoothly between the two-sided quantile $z\_{1-\alpha/2}$ and one-sided quantile $z\_{1-\alpha}$ as $\sqrt{n\_{\mathrm{eff}}^{\mathrm{var}}}\mkern2mu\hat{\Delta}\_{\mathrm{id}} / \hat{\sigma}\_{\max}$ grows (whereas whole-set coverage of $[\theta\_L, \theta\_U]$ remains anchored at $z\_{1-\alpha/2}$; [Chapter 35](35-inference-and-evidence.md)).

### A rule that arrives after the harm

Notice the clean division between claims and actions. The Mayo–Manski bridge validates **empirical claims** when $L\_{\mathrm{IM}} > \mu\_0 + \delta\_{\min}$ under verified statistical adequacy and frame completeness. **Operational actions**, by contrast, route §3.3's avoidable-loss interval $[L\_t, U\_t]$ through the three regimes of [Chapter 36](36-decision-loss-threshold.md) and [Chapter 41](41-the-triad-and-the-minimum.md):

- **Regime I (Buffered Action):** $L\_t > C$ (or boundary $L\_t = C < U\_t$) guarantees action pays across the whole interval ($a = 1$).
- **Regime I (Safe Abstention):** Non-lethal $U\_t \le C$ (including $L\_t < C = U\_t$ or $L\_t = U\_t = C$) ensures waiting is harmless ($a = 0$).
- **Regime II (Ambiguity Straddle):** When $L\_t < C < U\_t$ far from ruin, divisible reversible acts use the minimax-regret probe $a^\ast\_{\mathrm{MMR}} \in (0, 1)$ ([Chapter 43](43-inquiry-openness-multiverse.md) §2.6). Consider instead an **indivisible reversible act** facing harm probability $\pi \in [\pi\_{\min}, \pi\_{\max}]$, cost $C > 0$, effectiveness $\eta \in (0, 1]$, and reversible loss $L > 0$ (whose avoidable-loss endpoints are $L\_t = \pi\_{\min}\eta L$ and $U\_t = \pi\_{\max}\eta L$). It compares the midpoint $(\pi\_{\min} + \pi\_{\max})/2$ against $p^\ast\_{\mathrm{CL}} = C/(\eta L)$ strictly as **minimax regret under ambiguity** (Manski, 2026), never as an expected-loss dominance theorem.

Suppose action or waiting risks irreversible damage to vital reserves or external reception perimeters ($\Theta\_{\mathrm{basin}}$). Then straddling exits Regime II into **Regime III**: either unilateral refusal or, under dual lethal stakes, a binary **lethal-omission impasse** escalated to standpoint cutoffs ($p^\ast\_{\mathrm{affected}}$ versus $p^\ast\_{\mathrm{inst}}$). Distinct from **Regime III-B (Algedonic Halt)** when survival time runs out ($\mathrm{TTS}\_{\mathrm{survival}} < \mathrm{TTR}\_{\mathrm{eff}}$) in [Chapter 47](47-irreversible-time-vital-margin.md), the British bovine-spongiform-encephalopathy crisis prior to March 1996 (Phillips et al., 2000) shows the human cost of waiting: cheap reversible precautions were delayed until lower-bound scientific certainty arrived after irreversible harm had spilled across external reception perimeters ($\Theta\_{\mathrm{basin}}$).

### Where sharpness stops

Worst-case Manski bounds rely on bounded outcome support $[y\_{\min}, y\_{\max}]$, while Lee's trimming additionally assumes monotone retention ($R(1) \ge R(0)$), continuity, $p\_{\mathrm{obs}, 0} > 0$, and overlap $0 < \mathbb{P}(D = 1) < 1$. Unbounded support blows up worst-case widths, and broken monotonicity invalidates trimming. Setting $[y\_{\min}, y\_{\max}]$ solely from calm past peaks creates an **anti-Gettier trap** ahead of heavy-tail ruin ([Chapter 46](46-non-identical-asymmetry-heavy-tails.md); [Chapter 47](47-irreversible-time-vital-margin.md)). Worse, mistaking a **contractual or payer cap** ($X \wedge K$, with cap limit $K > 0$ distinct from §3.1's hypothesis budget $K\_{\max}$) for a physical limit hides the unpaid tail $(X - K)\_+$, which disappears from the ledger while draining across external reception perimeters ($\Theta\_{\mathrm{basin}}$). Sharp bounds therefore require physical support declarations, stress bounds above historical maxima, and explicit width sensitivity $\partial \Delta\_{\mathrm{id}} / \partial y\_{\max} = (1 - p\_{\mathrm{obs}, 1}) + (1 - p\_{\mathrm{obs}, 0})$.

Conversely, tighter bounds do not guarantee truth. Following Molinari's (2020) taxonomy, **non-refutable restrictions**, such as conditional missing-at-random or conditional ignorability without overidentifying exclusions, can shrink an interval to a point without ever contradicting sample data. By contrast, **refutable restrictions** (such as instrumental-variable exclusions) return an empty identified set when false. Molinari (2020) also separates global misspecification from finite-sample coverage breakdown under small local departures. Pre-declared $t = 0$ variant lists pay the $(1 + K\_{\max})^{-1}$ multiplicity dilution, while unannounced *post-hoc* tightenings destroy anytime Type I validity (§3.1). Genuine structural ***anagnórisis*** logs the refuted premise, reopens the schema counter-inductively ($\mathcal{F}\_t \subsetneq \mathcal{F}\_{t+1}^\ast$), starts a fresh out-of-sample *e*-process on newly sealed data, and validates the widened model out of sample ([Chapter 50](50-methodological-families-reflexivity.md)).

## Bridge

This chapter closes **Guard Site $T\_2$** by replacing fabricated point certainty with operational intervals $[L\_t, U\_t]$, Imbens–Manski bands $[L\_{\mathrm{IM}}, U\_{\mathrm{IM}}]$, and sequential *e*-processes $(E\_t)$. Yet those tools still rest on one fragile premise: that arm retention rates $p\_{\mathrm{obs}, d}$, unclustered precision, and the true population denominator $N$ can be read accurately off the archive.

[From World to Archive: Evidential Weight, Big Data Paradox, and the Uncounted](45-world-to-archive-big-data-paradox.md) confronts that premise at **Guard Site $T\_3$** ($P\_1 \to P\_2$). An archive is an institutional selection grid, not a miniature world. Selection bias ($\rho\_{R,Y}$), `Not askable` exclusions ($\pi\_i \equiv 0$), cluster design effects ($\mathrm{DEFF}$), and delayed vintages warp raw counts until [Chapter 45](45-world-to-archive-big-data-paradox.md) constructs $n\_{\mathrm{eff}}^{\mathrm{var}}$ and $n\_{\mathrm{eff}}(\bar{\rho})$ to complete Mediating Filter I.
