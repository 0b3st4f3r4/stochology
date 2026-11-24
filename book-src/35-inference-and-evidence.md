# Inference and Evidence

<figure class="stoch-figure stoch-logo-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="800" height="132" viewBox="0 0 800 132" fill="none" role="img" aria-label="Stochology CH35 · MARTINGALE — Inference, Severity, And E-Processes">
  <defs>
    <linearGradient id="ch35-ring" x1="2" y1="78" x2="386" y2="78" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="38%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="68%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <radialGradient id="ch35-glow" cx="50%" cy="50%" r="50%">
      <stop offset="0%" stop-color="#38bdf8" stop-opacity="0.24"/>
      <stop offset="48%" stop-color="#2563eb" stop-opacity="0.14"/>
      <stop offset="80%" stop-color="#e11d48" stop-opacity="0.08"/>
      <stop offset="100%" stop-color="#090e1a" stop-opacity="0"/>
    </radialGradient>
    <linearGradient id="ch35-triad-stroke" x1="18" y1="24" x2="110" y2="104" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.85"/>
      <stop offset="50%" stop-color="var(--logo-gold, #f59e0b)" stop-opacity="0.90"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.85"/>
    </linearGradient>
    <linearGradient id="ch35-fill" x1="22" y1="30" x2="106" y2="98" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)" stop-opacity="0.26"/>
      <stop offset="48%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.28"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.26"/>
    </linearGradient>
    <linearGradient id="ch35-ink-curve" x1="20" y1="66" x2="108" y2="66" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-ink, #0f172a)"/>
      <stop offset="50%" stop-color="var(--logo-sub, #1d4ed8)"/>
      <stop offset="100%" stop-color="var(--logo-ink, #0f172a)"/>
    </linearGradient>
    <radialGradient id="ch35-pupil" cx="45%" cy="42%" r="58%">
      <stop offset="0%" stop-color="#020617"/>
      <stop offset="68%" stop-color="#0f172a"/>
      <stop offset="100%" stop-color="#451a03"/>
    </radialGradient>
    <linearGradient id="ch35-gold-mira" x1="54" y1="48" x2="84" y2="82" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#fef08a"/>
      <stop offset="45%" stop-color="#fbbf24"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <radialGradient id="ch35-gold-core" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#ffffff"/>
      <stop offset="35%" stop-color="#fde047"/>
      <stop offset="100%" stop-color="#d97706"/>
    </radialGradient>
    <radialGradient id="ch35-node-well" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="var(--logo-well-hi, var(--elevated-bg, #ffffff))"/>
      <stop offset="100%" stop-color="var(--logo-node-bg, #e2e8f0)"/>
    </radialGradient>
    <linearGradient id="ch35-ring-epi" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#7dd3fc"/>
      <stop offset="50%" stop-color="#2563eb"/>
      <stop offset="100%" stop-color="#1e3a8a"/>
    </linearGradient>
    <radialGradient id="ch35-core-epi" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#bae6fd"/>
      <stop offset="45%" stop-color="#3b82f6"/>
      <stop offset="100%" stop-color="#1d4ed8"/>
    </radialGradient>
    <linearGradient id="ch35-ring-vital" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#6ee7b7"/>
      <stop offset="50%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#065f46"/>
    </linearGradient>
    <radialGradient id="ch35-core-vital" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#a7f3d0"/>
      <stop offset="45%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#047857"/>
    </radialGradient>
    <linearGradient id="ch35-ring-gov" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#fda4af"/>
      <stop offset="50%" stop-color="#e11d48"/>
      <stop offset="100%" stop-color="#881337"/>
    </linearGradient>
    <radialGradient id="ch35-core-gov" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#fecdd3"/>
      <stop offset="45%" stop-color="#f43f5e"/>
      <stop offset="100%" stop-color="#be123c"/>
    </radialGradient>
  </defs>
  <g>
    <circle cx="64" cy="66" r="48" fill="url(#ch35-glow)"/>
    <line x1="20" y1="30" x2="108" y2="30" stroke="var(--logo-crimson, #e11d48)" stroke-width="2.4" stroke-dasharray="4 2.5" stroke-linecap="round"/>
    <path d="M 22 94 L 38 82 L 50 88 L 66 64 L 78 70 L 94 30 L 106 24" stroke="var(--logo-blue, #2563eb)" stroke-width="3.0" stroke-linecap="round" stroke-linejoin="round"/>
    <circle cx="22" cy="94" r="6.6" fill="url(#ch35-node-well)" stroke="url(#ch35-ring-epi)" stroke-width="2.4"/>
    <circle cx="22" cy="94" r="2.8" fill="url(#ch35-core-epi)"/>
    <circle cx="66" cy="64" r="6.6" fill="url(#ch35-node-well)" stroke="url(#ch35-ring-vital)" stroke-width="2.4"/>
    <circle cx="66" cy="64" r="2.8" fill="url(#ch35-core-vital)"/>
    <g>
      <circle cx="94" cy="30" r="7.0" fill="var(--logo-node-bg, #e2e8f0)"/>
      <circle cx="94" cy="30" r="6.0" fill="url(#ch35-pupil)" stroke="url(#ch35-gold-mira)" stroke-width="2.2"/>
      <line x1="85.0" y1="30" x2="90.0" y2="30" stroke="url(#ch35-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
      <line x1="98.0" y1="30" x2="103.0" y2="30" stroke="url(#ch35-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
      <line x1="94" y1="21.0" x2="94" y2="26.0" stroke="url(#ch35-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
      <line x1="94" y1="34.0" x2="94" y2="39.0" stroke="url(#ch35-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
      <circle cx="94" cy="30" r="2.0" fill="url(#ch35-gold-core)"/>
    </g>
  </g>
  <g transform="translate(148, 0)">
    <text x="2" y="27" fill="var(--logo-sub, #1d4ed8)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.2">PART III · FOUNDATIONS OF JUDGMENT</text>
    <text x="498" y="27" text-anchor="end" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.0">CH35 · MARTINGALE</text>
    <text x="0" y="59" fill="var(--logo-ink, #0f172a)" font-family="system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif" font-size="18.5" font-weight="800" letter-spacing="0.95">INFERENCE, SEVERITY, AND E-PROCESSES</text>
    <line x1="2" y1="78" x2="384" y2="78" stroke="url(#ch35-ring)" stroke-width="1.6" stroke-opacity="0.82"/>
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
    <text x="2" y="104" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.8" font-weight="600" letter-spacing="0.65">VILLE INEQUALITY · ANYTIME-VALID OPTIONAL STOPPING · SEVERE CORROBORATION</text>
  </g>
  </svg>
</figure>

## By the end of this chapter you can

1. Distinguish an estimate, its interval, and the claim the interval supports.
2. Say what a p-value is not, and what post-data severity or anytime-valid testing adds before a test probes a claim.
3. Separate *is the effect nonzero* from *does the evidence clear a substantive boundary*, and distinguish both from what a decision requires.

## 1. Estimators and their procedural properties

Records passing world-to-archive collection at $T\_3$ ([Chapter 34](34-sample-population-archive.md)) still face severity lock $T\_2$ ($P\_2 \to P\_1$ of [Chapter 29](29-aim-and-the-three-planes.md)): testing inferential claims and intervals severely against real-world mechanisms so false point certainty cannot trigger **Collapse Proposition 1 (Blind Undersizing)** ([Chapter 41](41-the-triad-and-the-minimum.md)).

An estimator is a rule mapping sample data to an output. Its reliability belongs to the *procedure* across repeated draws, never to one lucky historical dataset:

- **Bias** ($\mathbb{E}[\hat{\theta}] - \theta^\ast$): systematic drift separating expected output from the true target.
- **Variance** ($\mathbb{E}[(\hat{\theta} - \mathbb{E}[\hat{\theta}])^2]$): instability across parallel samples.
- **Admissibility:** holding when no rival estimator achieves expected loss at least as low across every possible state ($\le$) while strictly beating it ($<$) in at least one state.

A point estimate without its sampling uncertainty says half the story, and many targets cannot narrow to a point at all. Even inside clean parametric families, C. R. Rao (1945) proved the Fisher-information floor on unbiased estimator variance (alongside Harald Cramér) and proved how conditioning on sufficient statistics cuts variance whenever an estimator ignores sufficient information, linking classical survey work at the Indian Statistical Institute directly to modern severity testing. Yet whenever selection, dropout, or unmeasured confounding intervenes, reality permits only an **identification region**, making that interval itself the honest finding ([Chapter 13](13-integrals-and-accumulation.md)).

## 2. Confidence intervals and interval width as finding

A classical $(1 - \alpha)$ confidence interval ($\alpha \in (0, 1)$) collects every parameter value not rejected by a two-sided level-$\alpha$ test. Three misunderstandings dominate everyday practice:

1. **Not a parameter probability:** $\theta^\ast$ is a fixed world quantity, not a coin flip; $(1 - \alpha)$ grades the procedure's long-run coverage across repeated draws, not the probability that one finished interval caught the truth.
2. **Width is the main finding:** A wide interval does not mean "messy noise." It proves the archive cannot tell major benefit apart from serious harm. Publishing only the midpoint hides the real result.
3. **Model-conditional coverage:** Nominal coverage fails immediately under heavy tails, group clustering, or selective dropout.

## 3. P-values, statistical power, and Mayo's severity principle

A *p*-value answers a narrow hypothetical question: *if the null claim were true, how often would we observe a test statistic at least as extreme as ours?* It is neither the probability that the null is true nor $1$ minus the probability of the alternative.

Failing to reject the null in an underpowered study gives zero evidence of safety. It leaves genuine absence indistinguishable from a weak design, what Deborah Mayo calls "bad evidence, no test" (Mayo 2018; [Chapter 18](18-philosophy-of-science.md) §4). **Power is the pre-data ability to fail when wrong.** An inquiry unable to fail produces institutional theater.

To evaluate evidence *after* seeing data $x\_0$, Mayo (1996, 2018) formulated the **Severity Principle**. Assuming statistical model checks on residuals pass (Spanos 2021; [Chapter 18](18-philosophy-of-science.md) §4), **pre-data power** fixes a rejection cutoff $c\_\alpha$ in advance, whereas **post-data severity** ($\mathrm{SEV}$) plugs the **actual observed** statistic $T(x\_0)$ into the falsifying boundary (using known dispersion $\sigma > 0$ and clustering-adjusted variance count $n\_{\mathrm{eff}}^{\mathrm{var}} = n / \mathrm{DEFF}\_{\mathrm{total}} > 0$ from [Chapter 34](34-sample-population-archive.md)):

- For an exceedance claim $\theta > \theta\_{\mathrm{crit}}$ (where larger $T$ supports exceedance), $\mathrm{SEV}(\theta > \theta\_{\mathrm{crit}};\mkern3mu x\_0) = \mathbb{P}\bigl(T(X) \le T(x\_0) \mid \theta = \theta\_{\mathrm{crit}}\bigr)$ measures how often (with what probability) the test would yield a less favorable signal ($T(X) \le T(x\_0)$) if $\theta = \theta\_{\mathrm{crit}}$.
- Symmetrically, for a non-harm claim $\delta \le \delta\_{\min}$ ($\delta\_{\min} > 0$, where smaller $T$ favors non-harm against $\delta > \delta\_{\min}$), the tail reverses: $\mathrm{SEV}(\delta \le \delta\_{\min};\mkern3mu x\_0) = \mathbb{P}\bigl(T(X) > T(x\_0) \mid \delta = \delta\_{\min}\bigr)$ ([Chapter 32](32-the-question-before-the-data.md) §3).

In a one-sided level-$\alpha$ Gaussian test of $H\_0: \delta \le 0$ against $\delta > 0$ ($\alpha \in (0, 1)$, so normal cutoff $z\_{1-\alpha} > 0$ for $\alpha < 1/2$) using standardized statistic $T(X) = \hat{\delta}\sqrt{n\_{\mathrm{eff}}^{\mathrm{var}}}/\sigma$ from an unbiased Gaussian estimator $\hat{\delta}$ with harm boundary $\delta\_{\min} > 0$ and standard normal CDF $\Phi$, pre-data power and post-data severity upon non-rejection ($T(x\_0) \le z\_{1-\alpha}$) read:

$$\mathrm{Power}(\delta\_{\min}) = 1 - \Phi\left(z\_{1-\alpha} - \frac{\delta\_{\min}\sqrt{n\_{\mathrm{eff}}^{\mathrm{var}}}}{\sigma}\right) \in (0, 1), \qquad \mathrm{SEV}\bigl(\delta \le \delta\_{\min};\mkern3mu x\_0\bigr) = 1 - \Phi\left(T(x\_0) - \frac{\delta\_{\min}\sqrt{n\_{\mathrm{eff}}^{\mathrm{var}}}}{\sigma}\right) \in (0, 1).$$

Read aloud, across non-rejections ($T(x\_0) \le z\_{1-\alpha}$) post-data severity satisfies $\mathrm{SEV}(\delta \le \delta\_{\min};\mkern3mu x\_0) \ge \mathrm{Power}(\delta\_{\min})$, matching pre-data power right at $T(x\_0) = z\_{1-\alpha}$ ([Chapter 43](43-inquiry-openness-multiverse.md) §2.3; [Appendix E](appendix-e-lexicon.md)). Whenever pre-data power against $\delta\_{\min}$ is low (so $\delta\_{\min}\sqrt{n\_{\mathrm{eff}}^{\mathrm{var}}}/\sigma$ is small or modest) and $T(x\_0)$ sits near or barely below $z\_{1-\alpha}$ (just as $\mathrm{SEV} \to 1/2$ at fixed $\hat{\delta}$ when $n\_{\mathrm{eff}}^{\mathrm{var}} \to 0^+$), severity $\mathrm{SEV}$ stays low, proving the non-rejection gave zero warrant for safety ($\delta \le \delta\_{\min}$).

High severity means a claim survived a probe likely to catch its specific flaws; low severity means almost any reality would pass. Severity grades methodological probing rather than Bayesian credence (Mayo 2018). Moreover, fixed-horizon *p*-values collapse under optional stopping unless inference shifts to sequential betting.

### Anytime-valid inference, e-processes, and Ville's inequality

Peeking at accumulating data until $p < \alpha$ destroys fixed-sample error limits. Maintaining **anytime-valid** Type I control at barrier $1/\alpha$ across arbitrary stopping times relies on an adapted non-negative **$e$-process** ($E\_t \ge 0$ with $E\_0 \le 1$ and $\mathbb{E}\_P[E\_\tau] \le 1$ at every stopping time $\tau$ under every $P \in \mathcal{P}\_0$, such as a non-negative test supermartingale bounded via Jean Ville's inequality) and pre-registered patch deflation $E\_t/(K\_{\max} + 1)$ ($K\_{\max} \in \mathbb{N}\_0$) across $K\_{\max} + 1$ individually valid null *$e$*-processes against opportunistic narrowing, while sparing attested counter-inductive reopening tested strictly out-of-sample ($\mathcal{F}\_t \subsetneq \mathcal{F}\_{t+1}^\ast$; Lakatos 1970), as derived in [Chapter 15](15-probability-as-a-measure.md) (§4) and [Chapter 30](30-probability-as-language.md) (§4; Ramdas et al. 2023). Under [Chapter 29](29-aim-and-the-three-planes.md) §4 and [Chapter 36](36-decision-loss-threshold.md) §5, guard site $T\_2$ runs continuous severity profiles and patch checks off-shift, yielding live displays immediately to survival-time halts ([Chapter 41](41-the-triad-and-the-minimum.md)). Unlike the retrospective confounding sensitivity *$E$-value* in §6, a forward *$e$*-process monitors sequential betting wealth $E\_t$ against $1/\alpha$.

## 4. Underdetermination and Manski partial-identification bounds

The sharpest divide in statistics separates a noisy finite-sample estimate from a parameter unidentifiable at *any* sample size. Sampling noise drops as $n$ grows; non-identification survives an infinite dataset.

Recall from [Chapter 13](13-integrals-and-accumulation.md) (§5) how partitioning the outcome space produces worst-case intervals without unstated assumptions. Take binary treatment $D \in \lbrace 0, 1\rbrace$, potential outcomes $Y(1), Y(0) \in [0, 1]$, treatment share $p = P(D = 1) \in (0, 1)$, and observed outcome $Y = Y(D)$ without within-arm dropout. Write observed arm means as $a = \mathbb{E}[Y \mid D = 1]$ and $b = \mathbb{E}[Y \mid D = 0]$. Extending missing-data bounds (Manski 1989) to treatment selection, Charles Manski (1990, 2003) noted that without randomization we observe $Y(1)$ only when $D = 1$ and $Y(0)$ only when $D = 0$. Plugging the worst possible $[0, 1]$ endpoints into the unseen arm traps each mean inside $[ap,\mkern3mu ap + (1 - p)]$ and $[b(1 - p),\mkern3mu b(1 - p) + p]$. Subtracting the widest opposite edges yields the sharp **Manski worst-case bounds**:

$$\mathrm{ATE} \in \bigl[ap - p - b(1 - p),\mkern5mu ap + (1 - p) - b(1 - p)\bigr].$$

In plain words, the observational worst-case interval width equals $(1 - p) + p = 1$ (or $y\_{\max} - y\_{\min}$ on bounded outcomes $[y\_{\min}, y\_{\max}]$) regardless of treatment share $p$ or sample size $n$ when every arm reports fully. Under randomized or unconfounded assignment ($D \perp Y(d)$), full response pins down a point while within-arm dropout $1 - p\_{\mathrm{obs}, d}$ widens the bound to $(y\_{\max} - y\_{\min})\bigl[(1 - p\_{\mathrm{obs}, 1}) + (1 - p\_{\mathrm{obs}, 0})\bigr]$; combining non-randomized selection with within-arm dropout widens it further to $(y\_{\max} - y\_{\min})\bigl[1 + p(1 - p\_{\mathrm{obs}, 1}) + (1 - p)(1 - p\_{\mathrm{obs}, 0})\bigr]$ ([Chapter 44](44-underdetermination-partial-identification.md)). Narrowing below worst-case obeys Manski's **Law of Decreasing Credibility** (Manski 2003; Ho & Rosen 2015/2016): stronger assumptions shrink intervals only by spending credibility. Francesca Molinari (2020) separates **non-refutable** assumptions (which shrink intervals without risking contradiction) from **refutable** assumptions (which collapse the identified set to emptiness when false).

When treatment is randomized (or unconfounded) yet subjects drop out asymmetrically, David Lee (2009) derived sharp trimming bounds assuming **monotone retention** ($R(1) \ge R(0)$ a.s.) and a continuous treated outcome distribution $Y(1)$. Trimming the excess share $q = (p\_{\mathrm{obs}, 1} - p\_{\mathrm{obs}, 0})/p\_{\mathrm{obs}, 1}$ ($p\_{\mathrm{obs}, 1} \ge p\_{\mathrm{obs}, 0} > 0$) from either the top or the bottom tail of the higher-retention arm bounds the ATE **strictly for always-observed units** ($\lbrace R(1) = R(0) = 1\rbrace$). Extending those bounds to the whole population requires identical effects across strata ([Chapter 44](44-underdetermination-partial-identification.md)).

Now combine partial identification with finite-sample noise. Why don't we need full two-sided normal margins $z\_{1-\alpha/2}$ on *both* ends of a wide interval $[\theta\_L, \theta\_U]$? Because if the interval has nonzero width $\Delta = \theta\_U - \theta\_L > 0$, a true parameter $\theta\_0 \in [\theta\_L, \theta\_U]$ cannot sit near *both* ends simultaneously: at most one boundary threatens coverage at once. Using variance-effective sample size $n\_{\mathrm{eff}}^{\mathrm{var}} = n / \mathrm{DEFF}\_{\mathrm{total}} > 0$, estimated width $\hat{\Delta} = \hat{\theta}\_U - \hat{\theta}\_L \ge 0$, and boundary standard errors with $\hat{\sigma}\_{\max} = \max(\hat{\sigma}\_L, \hat{\sigma}\_U) > 0$, Guido Imbens and Charles Manski (2004) attach an adaptive critical value $c\_\alpha$ ($\alpha \in (0, 1/2)$) solving:

$$\mathrm{CI}\_{1-\alpha}^{\theta\_0} = \left[\hat{\theta}\_L - c\_\alpha\frac{\hat{\sigma}\_L}{\sqrt{n\_{\mathrm{eff}}^{\mathrm{var}}}},\mkern5mu \hat{\theta}\_U + c\_\alpha\frac{\hat{\sigma}\_U}{\sqrt{n\_{\mathrm{eff}}^{\mathrm{var}}}}\right], \qquad \Phi\\!\left(c\_\alpha + \frac{\sqrt{n\_{\mathrm{eff}}^{\mathrm{var}}}\mkern2mu\hat{\Delta}}{\hat{\sigma}\_{\max}}\right) - \Phi(-c\_\alpha) = 1 - \alpha.$$

Read across normalized width $r = \sqrt{n\_{\mathrm{eff}}^{\mathrm{var}}}\mkern2mu\hat{\Delta}/\hat{\sigma}\_{\max} \ge 0$: when the set collapses to a point ($r = 0$), $c\_\alpha$ matches two-sided $z\_{1-\alpha/2}$; as $r \to \infty$, $\Phi(c\_\alpha + r) \to 1$ lowers $c\_\alpha$ smoothly to one-sided $z\_{1-\alpha}$ within a few standard errors. Remember this distinction: $c\_\alpha$ gives uniform coverage $1 - \alpha$ for the **true parameter $\theta\_0$**, whereas covering the **entire set $[\theta\_L, \theta\_U]$** still requires $z\_{1-\alpha/2}$ (Imbens & Manski 2004). Valid adaptive inference near $\Delta = 0$ also requires ordered estimators $\hat{\theta}\_U \ge \hat{\theta}\_L$ by construction (satisfied by Manski and Lee bounds, or needing pre-testing via Stoye (2009) and Stoye (2021)). Similarly, competing-risk survival tables yield sharp partial-identification bounds rather than point curves (Peterson 1976; [Chapter 31](31-distribution-and-moment.md) §6).

## 5. Epigraphy and legal standards of proof

To transcribe damaged ancient stone inscriptions without counterfeiting lost words, epigraphers formalized partial identification under the **Leiden Convention** (van Groningen 1932). Brackets mark editorial guesses (`[abc]`), while dots (`[...]`) or dashes (`[---]`) mark missing letters of known or unknown length rather than printing invented words as intact stone; manuscript editors similarly mark corrupt passages with a *crux desperationis* ($\dagger\dots\dagger$). Both practices refuse to counterfeit missing evidence.

In legal theory, L. Jonathan Cohen's **Gatecrasher Paradox** (Cohen 1977) shows why aggregate ratios never establish individual blame. Suppose $499$ spectators bought tickets and $501$ climbed the fence at a $1{,}000$-seat rodeo. For any random spectator, the gatecrashing probability is $p = 501/1{,}000 = 0.501 > 1/2$ (odds $501:499 > 1$), clearing civil "preponderance of the evidence" ($p^\ast\_{\mathrm{PK}} = 1/2$, unlike criminal proof at $10/11$ in [Chapter 36](36-decision-loss-threshold.md) §3). Yet no court can fine an individual on that group headcount alone. The missing piece is not uncertainty over $501/1{,}000$; it is zero individualized evidence. Inspired by Keynes's **weight of argument** (Keynes 1921), Stochology tracks case-specific weight for individual $x$ and claim $q$ via $V(x, q) = n\_{\mathrm{eff}}^{\mathrm{case}}(x, q) / \bigl(n\_{\mathrm{eff}}^{\mathrm{case}}(x, q) + n\_0\bigr)$ ($n\_0 > 0$, where $n\_{\mathrm{eff}}^{\mathrm{case}}(x, q) \ge 0$ indexes individualized case-specific evidential count linking $x$ to $q$). Distinct from archive-wide $n\_{\mathrm{eff}}^{\mathrm{var}}$ and Meng's $n\_{\mathrm{eff}}(\bar{\rho})$ ([Chapter 06](06-who-is-missing-from-the-table.md); [Chapter 34](34-sample-population-archive.md)), naked headcounts give $n\_{\mathrm{eff}}^{\mathrm{case}}(x, q) = 0$ and $V(x, q) = 0$, blocking liability without individualized proof.

## 6. Confounding sensitivity, vital precaution, and undecided intervals

### Confounding sensitivity: Doll, Hill, and the E-value

Waiting forever for randomized trials is neither necessary nor safe when we can calculate how strong an unseen bias must be to erase an empirical finding. In their classic lung-carcinoma case-control study, Richard Doll and Austin Bradford Hill (1950) eliminated selection, interview bias, and reverse causation, concluding smoking is *"a factor, and an important factor, in the production of carcinoma of the lung"*, while leaving open whether an unmeasured genetic or lifestyle confounder explained the rest.

The Surgeon General's report answered that objection using Jerome Cornfield's inequality bounding the minimum confounder-exposure and confounder-outcome strength required (Advisory Committee to the Surgeon General 1964). Later work generalized that check into the **$E$-value** of retrospective confounding sensitivity (Ding & VanderWeele 2016; VanderWeele & Ding 2017), distinct from the forward sequential *$e$*-process ($E\_t$) of §3. For any observed risk ratio $\mathrm{RR} \ge 1$, the minimum symmetric association an unmeasured confounder needs with both exposure and outcome to explain away the observed ratio is:

$$E\text{-value} = \mathrm{RR} + \sqrt{\mathrm{RR}\mkern3mu(\mathrm{RR} - 1)} \qquad (\mathrm{RR} \ge 1).$$

In plain words, the required confounder risk ratio grows faster than $\mathrm{RR}$ itself (exceeding $\mathrm{RR}$ by $\sqrt{\mathrm{RR}(\mathrm{RR} - 1)}$ and approaching $2\mkern2mu\mathrm{RR} - 1/2$ for large $\mathrm{RR}$) and covers protective associations by substituting $\mathrm{RR}^\ast = 1/\mathrm{RR}$ when $\mathrm{RR} \in (0, 1)$, while converting rare odds ratios via $\mathrm{OR} \approx \mathrm{RR}$ or common ones via $\sqrt{\mathrm{OR}}$ (VanderWeele & Ding 2017). Because no plausible mid-century confounder cleared the Cornfield-inequality threshold needed to explain away Doll and Hill's (1950) smoking–lung-cancer association, causal inference held (Advisory Committee to the Surgeon General 1964). By Manski's Law of Decreasing Credibility (Manski 2003; Molinari 2020), however, an $E$-value tests only single-channel confounding; it cannot cure differential measurement bias or `Not askable` exclusions ($\pi\_i = 0$) dropped at $T\_3$.

Furthermore, scientific proof differs from vital precaution. Waiting until a lower Imbens–Manski bound $L\_{\mathrm{IM}} := \hat{\theta}\_L - c\_\alpha\hat{\sigma}\_L/\sqrt{n\_{\mathrm{eff}}^{\mathrm{var}}}$ clears a strict bar makes sense for affirmative scientific claims with symmetric error costs. Demanding that same bar before taking cheap, reversible precautions against irreversible ruin suffocates vital tolerance (Canguilhem 1943/1966) and lets damage outrun recovery time ([Chapter 44](44-underdetermination-partial-identification.md)).

### When the region is honest and the decision is empty

Whenever evidence supports an interval, that interval is the scientific finding, even when an avoidable-loss band $[L\_t, U\_t]$ straddles intervention cost $C > 0$ ($L\_t < C < U\_t$). Rather than defaulting to rigid worst-case maximin, critiqued by Harsanyi (1975), action follows the three-regime partition formalized in [Chapter 36](36-decision-loss-threshold.md) and [Chapter 41](41-the-triad-and-the-minimum.md) once survival headroom and sealed channels hold: strict dominance picks full action or safe inaction, while straddling bounds enter minimax-regret probing strictly for reversible acts. Publishing both the identification interval and its decision regime stops institutions from hiding ambiguity behind fake precision that harms unconsenting external reception perimeters ([Chapter 29](29-aim-and-the-three-planes.md); [Chapter 41](41-the-triad-and-the-minimum.md)).

## Bridge

Inside [Chapter 29](29-aim-and-the-three-planes.md)'s circuit, records clearing $T\_3$ ([Chapter 34](34-sample-population-archive.md)) pass severe world testing at $T\_2$: post-data severity (§3), anytime-valid *$e$*-processes, Manski and Imbens–Manski bounds (§4), individualized case weight (§5), and confounding sensitivity (§6). [Chapter 44, *Underdetermination, Partial Identification, and Severe Testing*](44-underdetermination-partial-identification.md) formalizes these tools inside the running case. Yet intervals turn into acts only once weighed against asymmetric loss matrices and decision thresholds, the subject of [Chapter 36, *Decision, Loss, Threshold*](36-decision-loss-threshold.md).
