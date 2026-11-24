# Aiming Against Counterfactual Winds

<figure class="stoch-figure stoch-logo-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="800" height="132" viewBox="0 0 800 132" fill="none" role="img" aria-label="Stochology CH87 · T2 — Aiming Against Counterfactual Winds">
  <defs>
    <linearGradient id="ch87-ring" x1="2" y1="78" x2="386" y2="78" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="38%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="68%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <radialGradient id="ch87-glow" cx="50%" cy="50%" r="50%">
      <stop offset="0%" stop-color="#38bdf8" stop-opacity="0.24"/>
      <stop offset="65%" stop-color="#2563eb" stop-opacity="0.12"/>
      <stop offset="100%" stop-color="#090e1a" stop-opacity="0"/>
    </radialGradient>
    <radialGradient id="ch87-pupil" cx="45%" cy="42%" r="58%">
      <stop offset="0%" stop-color="#020617"/>
      <stop offset="100%" stop-color="#451a03"/>
    </radialGradient>
    <linearGradient id="ch87-gold-mira" x1="54" y1="48" x2="84" y2="82" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#fef08a"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <radialGradient id="ch87-node-well" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="var(--logo-well-hi, var(--elevated-bg, #ffffff))"/>
      <stop offset="100%" stop-color="var(--logo-node-bg, #e2e8f0)"/>
    </radialGradient>
  </defs>
  <g>
    <circle cx="64" cy="66" r="46" fill="url(#ch87-glow)"/>
    <rect x="26" y="48" width="76" height="36" rx="4" fill="var(--logo-blue, #2563eb)" fill-opacity="0.14" stroke="var(--logo-blue, #2563eb)" stroke-width="1.8" stroke-dasharray="3.5 2.5"/>
    <path d="M 24 72 C 46 66, 66 42, 102 30" stroke="var(--logo-green, #10b981)" stroke-width="2.6" stroke-linecap="round"/>
    <path d="M 24 72 C 48 76, 72 86, 102 98" stroke="var(--logo-crimson, #e11d48)" stroke-width="2.2" stroke-dasharray="3.5 2.5" stroke-linecap="round"/>
    <circle cx="24" cy="72" r="5.6" fill="url(#ch87-node-well)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.2"/>
    <circle cx="102" cy="30" r="5.6" fill="url(#ch87-node-well)" stroke="var(--logo-green, #10b981)" stroke-width="2.2"/>
    <circle cx="64" cy="66" r="6.2" fill="url(#ch87-pupil)" stroke="url(#ch87-gold-mira)" stroke-width="2.2"/>
    <line x1="56" y1="66" x2="72" y2="66" stroke="url(#ch87-gold-mira)" stroke-width="1.8"/>
    <line x1="64" y1="58" x2="64" y2="74" stroke="url(#ch87-gold-mira)" stroke-width="1.8"/>
  </g>
  <g transform="translate(148, 0)">
    <text x="2" y="27" fill="var(--logo-sub, #1d4ed8)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.2">PART VI · THE TRAJECTORY OF AIM</text>
    <text x="498" y="27" text-anchor="end" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.0">CH87 · T2 · P2→P1</text>
    <text x="0" y="59" fill="var(--logo-ink, #0f172a)" font-family="system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif" font-size="18.2" font-weight="800" letter-spacing="0.85">AIMING AGAINST COUNTERFACTUAL WINDS</text>
    <line x1="2" y1="78" x2="384" y2="78" stroke="url(#ch87-ring)" stroke-width="1.6" stroke-opacity="0.82"/>
    <circle cx="400" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="415" cy="78" r="6.2" stroke="var(--logo-gold, #f59e0b)" stroke-width="1.5" stroke-dasharray="2.2 1.6"/>
    <circle cx="415" cy="78" r="4.0" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.2"/>
    <circle cx="415" cy="78" r="1.8" fill="var(--logo-gold, #f59e0b)"/>
    <circle cx="430" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="445" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="460" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="475" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="490" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <text x="2" y="104" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.0" font-weight="600" letter-spacing="0.48">E-PROCESSES · MANSKI BOUNDS · WHY DRO BALLS FAIL UNDER T3 AND T6</text>
  </g>
  </svg>
</figure>

## By the end of this chapter you can

1. Separate competence from environmental luck using Gettier contamination rates, composite-null *e*-processes, and pre-enumerated *ad hoc* deflation versus attested reopening.
2. Construct sharp non-parametric bounds (Manski, Lee, Peterson, Imbens–Manski) with explicit assignment and positivity premises alongside statistical adequacy.
3. Prove why empirical Wasserstein and $\phi$-divergence balls around $\hat{P}\_n$ or $P\_{\mathrm{obs}}$ fail under `T3` sample bias and state the threshold at which they break under `T6` performative shift (resolved in [Chapter 91](91-when-the-target-reads-the-archer.md)).

Once `T1` locks the framing brief ([Chapter 86](86-posture-before-bowstring-draws.md), open framing and standpoint cutoffs) onto $\sigma\_t = (\mathbf{u}\_t, \mathbf{w}\_t, \mathbf{m}\_t)$ ([Part VI — Introduction](part-vi-introduction.md); [Appendix B](appendix-b-architecture-verification.md) §B.1, the grammar and predicate), the shot meets guard site `T2`. Crossing from Archive ($P\_2$) to World ($P\_1$) ([Chapter 29](29-aim-and-the-three-planes.md), the three-plane circuit), `T2` protects the **Epistemic Margin ($S\_{\mathrm{epi}}$)** against **Collapse Proposition 1 (*Blind Undersizing*)** ([Chapter 41](41-the-triad-and-the-minimum.md), where false point certainty shrinks buffers). Before striking stone, `T2` separates genuine aim from lucky gusts, *ad hoc* excuses, and smoothed averages. Meanwhile, **Reflexive Brake 1** ([Chapter 50, *Five Methodological Families, Process Tracing, and Reflexive Self-Application*](50-methodological-families-reflexivity.md), preventing vacuous skepticism from masquerading as caution) keeps intervals informative, and ex-post $\mathcal{D}\_{\mathrm{diag}}$ never steers live steps.

## 6.2.1 Coincidental Hits, Duhem–Quine Holism, and Agnotological Power

An arrow blown onto the mark hits without proving skill. In Edmund Gettier's (1963) sense ([Chapter 16, *What Knowledge Is*](16-what-knowledge-is.md), separating skill from environmental luck), competence separates from lucky hits via the Gettier contamination ratio $G(p, c) = c/(p + c)$ ($p, c \ge 0$, $p + c > 0$). Here $p$ counts skillful hits and $c$ counts lucky ones (kept distinct from $p$-values, $p\_{\mathrm{obs}}$, and Wasserstein constant $c\_W$); unseparated totals ($c \gg p$) conceal fragility until the wind reverses.

No hypothesis faces reality alone: every empirical test examines a core claim alongside auxiliary assumptions (Duhem 1906; Quine 1951). Agnotology weaponizes that holism by running underpowered studies that pass absent detection power off as proof of safety (Proctor & Schiebinger 2008).

Noting over 350,000 registered chemicals worldwide, Schäffer et al. (2023) document a stark split in Brazilian pesticide approvals. Private contract labs cleared every tested formulation: *"none of the studies regarding mutagenicity, carcinogenicity, and reproductive toxicity conducted by private contract laboratories (used for the registration of 247 pesticide formulations in Brazil) revealed any toxic effects."* Yet among 574 literature studies on those same pesticides, 84 percent of studies by public-institution researchers found toxic effects. This echoes the European REACH compliance deficits and opioid-archive claim propagation audited in [Chapter 68, *Layer 18: Pharmacovigilance, Regulatory Toxicology & Post-Market Safety Order*](68-layer-pharmacovigilance-regulatory-toxicology-order.md) (where regulatory gaps hide harm behind missing tests).

## 6.2.2 Anytime-Valid Test Martingales and Lakatosian Shift Penalty

Guard `T2` locks sequential testing discipline in place before counterfactual trajectories unfold ([Chapter 35, *Inference and Evidence*](35-inference-and-evidence.md), testing under dependence; [Chapter 44, *Underdetermination, Partial Identification, and Severe Testing*](44-underdetermination-partial-identification.md), severing weak evidence from confirmed claims). Classical fixed-horizon $p$-values break if an analyst peeks repeatedly at incoming data and stops when convenient. Composite-null testing replaces fixed horizons with anytime-valid **$e$-processes** $(E\_t)\_{t \ge 0}$ (using lowercase $e$-processes, distinct from Euler's exponential constant $e$ in §6.2.4), which behave like fair betting scores against the null. They obey Ville's (1939) time-uniform bound $\sup\_{P \in \mathcal{P}\_0} \mathbb{P}\_P(\sup\_{t \ge 1} E\_t \ge 1/\alpha) \le \alpha$ ([Chapter 30, *Probability as Language*](30-probability-as-language.md), non-negative supermartingales; Shafer 2021; Howard et al. 2021; Ramdas et al. 2023; Grünwald, de Heide, & Koolen 2024).

Guard `T2` also stops quiet *post-hoc* patching after a miss. Pairing Imre Lakatos's (1968–1969; 1970) progressive-versus-degenerating demarcation with finite pre-registered budget divisor $E\_T / (1 + K\_{\max})$ ($K\_{\max} \in \mathbb{N}\_0$) dilutes evidence across pre-enumerated *ad hoc* narrowings ([Chapter 44](44-underdetermination-partial-identification.md), penalizing post-hoc excuses). By contrast, **attested counter-inductive reopening** ($\mathcal{F}\_t \subsetneq \mathcal{F}\_{t+1}^\ast$; Feyerabend 1970; Canguilhem 1943/1966; [Chapter 93](93-fracturing-the-frame-before-the-seal.md), widening failed frames under custody) retires falsified premises for out-of-sample tests. Alongside post-data severity $\mathrm{SEV}$ (Mayo 2018) and Spanos's (2021) adequacy checks, which miss unmeasured confounding and `T3` omissions, cutoffs lock offline onto $\mathbf{u}\_t$ before release ([Chapter 95](95-collision-at-ten-bits-per-second.md), protecting bounded human review).

## 6.2.3 Partial Identification Under Missing Counterfactuals

When observations are missing or selectively recorded, `T2` refuses point guesses and writes sharp bounds $[L\_t, U\_t]$ onto $\mathbf{u}\_t$ ([Chapter 35](35-inference-and-evidence.md), partial identification intervals; [Chapter 44](44-underdetermination-partial-identification.md), bounding unknowns without invented assumptions). Three classical tools set those limits: Manski's (1989; 1990; 2003; 2026) identification intervals under missing outcomes ($R \in \lbrace 0, 1\rbrace$) or observational selection ($R \equiv 1$), Lee's (2009) monotone-retention trimming over always-observed units, and Peterson's (1976) competing-risk bounds on net survival $S\_j(t)$ without untestable independence.

For finite-sample intervals ($\alpha \in (0, 1/2)$), `T2` imports variance-effective sample size $n\_{\mathrm{eff}}^{\mathrm{var}}$ from `T3` ([Chapter 88](88-what-the-wind-leaves-on-stone.md), cluster and weight penalties). Imbens and Manski (2004) calibrate $c\_{\mathrm{IM}} \in [z\_{1-\alpha}, z\_{1-\alpha/2}]$ for **pointwise coverage of $\theta\_0 \in [\theta\_L, \theta\_U]$** under width superefficiency, whereas set coverage requires $z\_{1-\alpha/2}$; superefficiency holds automatically when $(\hat{\theta}\_L, \hat{\theta}\_U)$ are uniformly jointly asymptotically normal and ordered ($\hat{\theta}\_U \ge \hat{\theta}\_L$) by construction (Stoye 2009).

Molinari (2020) separates **non-refutable** assumptions from **refutable** restrictions returning empty sets when falsified, while Stoye (2021) keeps $\mathrm{CI}^{\mathrm{MA}}$ non-empty even when sample bounds invert ($\hat{\theta}\_U < \hat{\theta}\_L$). Because projecting joint 95 percent ellipsoids inflates marginal critical values from $1.96$ to $4.28$ at $d = 10$ (Kaido, Molinari, & Stoye 2019), `T2` requires scalar-calibrated bounds.

The National Supported Work benchmark shows how observational selection counterfeits precision (LaLonde 1986; Dehejia & Wahba 1999; [Chapter 59, *Layer 09: Experimental Design, Complex Survey Sampling & Causal Methodology Order*](59-layer-experimental-sampling-causal-order.md), experimental versus survey controls). Unlike positive-wage attrition trimming in Lee (2009), comparing experimental controls against non-experimental `CPS-1` controls flips the raw mean difference even when every unit is observed ($R \equiv 1$). Recovering treatment effects therefore requires non-refutable conditional ignorability and common support ([Chapter 44](44-underdetermination-partial-identification.md), separating identification assumptions from sample data).

## 6.2.4 Why Empirical DRO Ambiguity Balls Fail Under `T3` and `T6`

**Distributionally robust optimization** ($\mathrm{DRO}$; Rahimian & Mehrotra 2019) draws a protective safety bubble around observed empirical data and optimizes against the worst law inside it. The construction below proves that this bubble collapses whenever an archive guard (`T3`) dropped extreme outcomes beforehand, or whenever publishing a rule (`T6`) pushes the world outside historical support.

Under i.i.d. sampling, Mohajerin Esfahani and Kuhn (2018) prove that Kullback–Leibler ($\mathrm{KL}$) balls around discrete empirical $\hat{P}\_N$ contain an absolutely continuous truth with probability zero. Empirical Wasserstein-1 radii $\varepsilon\_N \le 1$ scale as $N^{-1/\max\lbrace m, 2 \rbrace}$ ($N \ge 1$, $m \ne 2$) only under light exponential tail moments, suffering the dimension-$m$ curse. Similarly, Duchi and Namkoong (2021) analyze $\phi$-divergence balls $\lbrace Q \ll \hat{P}\_n : D\_\phi(Q \parallel \hat{P}\_n) \le \rho\_{\mathrm{div}} \rbrace$ ($\rho\_{\mathrm{div}} > 0$). Because $Q \ll \hat{P}\_n$ restricts mass strictly to observed support, polynomial divergence growth $\phi(t) \asymp t^k$ ($t \to \infty$, $k \in (1, \infty)$) slows minimax excess risk toward $O\_P(n^{-1/(k\_\ast \vee 2)} \ln n)$ ($k\_\ast = k/(k-1)$).

An explicit counterexample on bounded physical support $Y \in [0, 100]$ exposes why both $\mathrm{DRO}$ families fail under `T3` missingness. That bounded domain matches the stone support in [Chapter 88](88-what-the-wind-leaves-on-stone.md) (archival selection and saturation) and blocks the *anti-Gettier* censoring trap of [Chapter 94](94-yesterdays-crosswind-in-the-string.md) (how right-truncation manufactures fake calm). Suppose the true law is $P^\ast = f\mkern3mu\mathrm{Uniform}(0, 20) + (1 - f)\mkern3mu\delta\_{100}$ with observed share $p\_{\mathrm{obs}} = f = 0.80$ and missing point mass $\delta\_{100}$ of size $1 - f = 0.20$ at $100$. Gate `T3` records $R = 1$ only when $Y \le 20$ and drops the extreme atom at $100$ ($R = 0$). Then observed distribution $P\_{\mathrm{obs}} = \mathrm{Uniform}(0, 20)$ has mean $10$, whereas true expectation is $\mathbb{E}\_{P^\ast}[Y] = 0.80 \cdot 10 + 0.20 \cdot 100 = 28$.

To compute both failure gaps, let $\lambda > 0$ be the $\mathrm{KL}$ dual multiplier, $F\_{\mathrm{obs}}$ and $F\_{P^\ast}$ the cumulative distribution functions, and $h(y) = y$ a $1$-Lipschitz loss. Also let $\varepsilon\_n$ be a decaying Wasserstein radius with constant $c\_W > 0$, dimension $m$, and sample size $n \ge 1$. Both $\mathrm{DRO}$ balls miss $28$:

1. **Absolute-continuity trap ($\phi$-divergence and $\mathrm{KL}$ balls).** Whenever a $\phi$-divergence ball around $P\_{\mathrm{obs}}$ (or $\hat{P}\_n$) enforces $Q \ll P\_{\mathrm{obs}}$, or is generated by a superlinear divergence with $\lim\_{t \to \infty} \phi(t)/t = +\infty$ (such as $\mathrm{KL}$ or $\phi(t) \asymp t^k$, $k > 1$, whereas finite-recession divergences like Total Variation or squared Hellinger admit singular mass unless $\rho\_{\mathrm{div}}$ stays below their singularity threshold), every finite-radius member satisfies $\mathrm{supp}(Q) \subseteq [0, 20]$ and caps $\sup\_{Q \ll P\_{\mathrm{obs}} : D\_\phi(Q \parallel P\_{\mathrm{obs}}) \le \rho\_{\mathrm{div}}} \mathbb{E}\_Q[Y] \le 20 < 28$ for all $\rho\_{\mathrm{div}} \in [0, \infty)$. Minimizing over $\lambda > 0$ (where $e$ is Euler's exponential constant) gives the exact $\mathrm{KL}$ dual on $P\_{\mathrm{obs}} = \mathrm{Uniform}(0, 20)$:

   $$\sup\_{D\_{\mathrm{KL}}(Q \parallel P\_{\mathrm{obs}}) \le \rho\_{\mathrm{div}}} \mathbb{E}\_Q[Y] = \inf\_{\lambda > 0}\left\lbrace \lambda \rho\_{\mathrm{div}} + \lambda \ln\\!\left(\frac{\lambda}{20}\bigl(e^{20/\lambda} - 1\bigr)\right)\right\rbrace < 20 < 28.$$

   At $\rho\_{\mathrm{div}} = 5$, that dual reaches roughly $19.95 < 20$, over eight units below $28$. (On unbounded $P\_{\mathrm{obs}} = \mathcal{N}(\mu, \sigma^2)$ with $\sigma, \rho\_{\mathrm{div}} > 0$, the dual $\mu + \sigma\sqrt{2\rho\_{\mathrm{div}}}$ grows unboundedly only because tails already inhabit $\mathrm{supp}(P\_{\mathrm{obs}})$.)

2. **Decaying-radius trap (Wasserstein-1 balls).** By stochastic dominance ($F\_{\mathrm{obs}} \ge F\_{P^\ast}$), integrating cumulative gaps gives $W\_1(P^\ast, P\_{\mathrm{obs}}) = \int\_0^{100} \bigl(F\_{\mathrm{obs}}(y) - F\_{P^\ast}(y)\bigr)\mkern3mu dy = 0.20 \cdot 10 + 0.20 \cdot (100 - 20) = 18$. For $1$-Lipschitz loss $h(y) = y$, a Wasserstein-1 ball of radius $\varepsilon\_n = c\_W\mkern3mu n^{-1/\max\lbrace m, 2 \rbrace}$ ($c\_W > 0, n \ge 1$) caps expectation at $\mathbb{E}\_{\hat{P}\_n}[Y] + \varepsilon\_n \to 10 < 28$ almost surely. For instance, at $P\_{\mathrm{obs}}$ with $c\_W = 2, m = 1, n = 10{,}000$, $\varepsilon\_n = 0.02$ reaches only $10.02 \ll 28$. By contrast, Manski bounds on $[0, 100]$ give $[0.80 \cdot 10 + 0.20 \cdot 0,\mkern5mu 0.80 \cdot 10 + 0.20 \cdot 100] = [8, 28]$, which encloses $28$.

3. **Performative-shift trap (`T6`).** Even with zero missing data ($f = 1$), deploying decision rule $\theta$ shifts the field distribution to $\mathcal{D}(\theta)$ (Perdomo et al. 2020). Any offline ball around $\hat{P}\_n \sim \mathcal{D}(\theta\_0)$ breaks once $W\_1(\mathcal{D}(\theta), \hat{P}\_n) > \varepsilon\_n$, as resolved in [Chapter 91](91-when-the-target-reads-the-archer.md) (performative shift and offline $\mathrm{DRO}$ failure).

In plain words, if missing records erased everything above $20$, a $\mathrm{KL}$ safety ball never sees past $19.95$ and a shrinking Wasserstein ball stalls at $10.02$, whereas Manski's honest $[8, 28]$ interval catches the true $28$ immediately.

## 6.2.5 Laminar Boundary of `T2`

Critics warn that worst-case rules invite paralysis (Harsanyi 1975) or suffer non-uniqueness under partial identification, where all rules remain admissible and *minimax regret* requires randomization (Klibanoff, Marinacci, & Mukerji 2005; Montiel Olea, Qiu, & Stoye 2025). Stochology separates claim severity from step routing: neither unconditional maximin nor ex-post $\mathcal{D}\_{\mathrm{diag}}$ steers live steps, while straddling bounds enter **Regimes II–III** ([Chapter 41](41-the-triad-and-the-minimum.md), minimax-regret probing versus unilateral refusal; [Chapter 44](44-underdetermination-partial-identification.md), severing paralysis from reversible probing).

Waiting for the Imbens–Manski lower bound $L\_{\mathrm{IM}}$ to exceed minimum harm $\delta\_{\min} > 0$ fails when an **irreversible straddle** on step parameters threatens severe unpriced harm while sampling lags behind cheap precaution. That irreversible straddle (or breached physical/governance floor) triggers **Regime III** live (and $\mathrm{TTS}\_{\mathrm{survival}} < \mathrm{TTR}\_{\mathrm{eff}}$ triggers **Regime III-B**), whereas unprevented spillover exported onto external reception perimeters is scored strictly **ex post** via $\Theta\_{\mathrm{basin}}$ inside $\mathcal{D}\_{\mathrm{diag}}$, as documented in British BSE before March 1996 (Phillips et al. 2000). Still, every `T2` check trusts `T3` for observed fraction $p\_{\mathrm{obs}} = f$, variance-effective sample size $n\_{\mathrm{eff}}^{\mathrm{var}}$, and physical support $[y\_{\min}, y\_{\max}]$.

## Bridge

Severe counterfactual testing is only as faithful as the stone inscription handed to the test. Every anytime-valid *e*-process and Manski bound presupposes uncorrupted $n\_{\mathrm{eff}}^{\mathrm{var}}$ for $c\_{\mathrm{IM}}$, Keynesian weight $V$ under **Reflexive Brake 1**, and separation between *Not observed* strata and *Not askable* exclusions ($\pi\_i \equiv 0$) exporting harm onto external reception perimeters ($\Theta\_{\mathrm{basin}}$).

[What the Wind Leaves on Stone](88-what-the-wind-leaves-on-stone.md) moves to Chapter 88 and guard site `T3`, crossing from World ($P\_1$) to Archive ($P\_2$), auditing selection correlation, additive design effects, unseen counts, vintages, and missing-not-at-random quantile and $\mathrm{CVaR}\_p$ saturation.
