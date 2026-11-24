# Reactive Worlds: Causal Invariance, Performativity, and Goodhart's Law

<figure class="stoch-figure stoch-logo-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="800" height="132" viewBox="0 0 800 132" fill="none" role="img" aria-label="Stochology CH48 · S_GOV — Reactive Worlds And Goodhart'S Law">
  <defs>
    <linearGradient id="ch48-ring" x1="2" y1="78" x2="386" y2="78" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="38%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="68%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <radialGradient id="ch48-glow" cx="50%" cy="50%" r="50%">
      <stop offset="0%" stop-color="#f43f5e" stop-opacity="0.24"/>
      <stop offset="48%" stop-color="#fbbf24" stop-opacity="0.14"/>
      <stop offset="80%" stop-color="#38bdf8" stop-opacity="0.08"/>
      <stop offset="100%" stop-color="#090e1a" stop-opacity="0"/>
    </radialGradient>
    <linearGradient id="ch48-triad-stroke" x1="18" y1="24" x2="110" y2="104" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.85"/>
      <stop offset="50%" stop-color="var(--logo-gold, #f59e0b)" stop-opacity="0.90"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.85"/>
    </linearGradient>
    <linearGradient id="ch48-fill" x1="22" y1="30" x2="106" y2="98" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)" stop-opacity="0.26"/>
      <stop offset="48%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.28"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.26"/>
    </linearGradient>
    <linearGradient id="ch48-ink-curve" x1="20" y1="66" x2="108" y2="66" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-ink, #0f172a)"/>
      <stop offset="50%" stop-color="var(--logo-sub, #1d4ed8)"/>
      <stop offset="100%" stop-color="var(--logo-ink, #0f172a)"/>
    </linearGradient>
    <radialGradient id="ch48-pupil" cx="45%" cy="42%" r="58%">
      <stop offset="0%" stop-color="#020617"/>
      <stop offset="68%" stop-color="#0f172a"/>
      <stop offset="100%" stop-color="#451a03"/>
    </radialGradient>
    <linearGradient id="ch48-gold-mira" x1="54" y1="48" x2="84" y2="82" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#fef08a"/>
      <stop offset="45%" stop-color="#fbbf24"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <radialGradient id="ch48-gold-core" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#ffffff"/>
      <stop offset="35%" stop-color="#fde047"/>
      <stop offset="100%" stop-color="#d97706"/>
    </radialGradient>
    <radialGradient id="ch48-node-well" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="var(--logo-well-hi, var(--elevated-bg, #ffffff))"/>
      <stop offset="100%" stop-color="var(--logo-node-bg, #e2e8f0)"/>
    </radialGradient>
    <linearGradient id="ch48-ring-epi" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#7dd3fc"/>
      <stop offset="50%" stop-color="#2563eb"/>
      <stop offset="100%" stop-color="#1e3a8a"/>
    </linearGradient>
    <radialGradient id="ch48-core-epi" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#bae6fd"/>
      <stop offset="45%" stop-color="#3b82f6"/>
      <stop offset="100%" stop-color="#1d4ed8"/>
    </radialGradient>
    <linearGradient id="ch48-ring-vital" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#6ee7b7"/>
      <stop offset="50%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#065f46"/>
    </linearGradient>
    <radialGradient id="ch48-core-vital" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#a7f3d0"/>
      <stop offset="45%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#047857"/>
    </radialGradient>
    <linearGradient id="ch48-ring-gov" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#fda4af"/>
      <stop offset="50%" stop-color="#e11d48"/>
      <stop offset="100%" stop-color="#881337"/>
    </linearGradient>
    <radialGradient id="ch48-core-gov" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#fecdd3"/>
      <stop offset="45%" stop-color="#f43f5e"/>
      <stop offset="100%" stop-color="#be123c"/>
    </radialGradient>
  </defs>
  <g>
    <circle cx="64" cy="66" r="48" fill="url(#ch48-glow)"/>
    <circle cx="64" cy="66" r="42" stroke="var(--logo-ticks, #64748b)" stroke-width="1.2" stroke-dasharray="3 3" stroke-opacity="0.45"/>
    <line x1="24" y1="96" x2="104" y2="28" stroke="var(--logo-blue, #2563eb)" stroke-width="2.8" stroke-linecap="round"/>
    <line x1="24" y1="76" x2="104" y2="50" stroke="var(--logo-green, #10b981)" stroke-width="2.4" stroke-linecap="round"/>
    <path d="M 24 88 C 52 94, 72 32, 104 36" stroke="var(--logo-crimson, #e11d48)" stroke-width="2.6" stroke-dasharray="4 2.5" stroke-linecap="round"/>
    <circle cx="24" cy="88" r="6.6" fill="url(#ch48-node-well)" stroke="url(#ch48-ring-epi)" stroke-width="2.4"/>
    <circle cx="24" cy="88" r="2.8" fill="url(#ch48-core-epi)"/>
    <circle cx="104" cy="36" r="6.6" fill="url(#ch48-node-well)" stroke="url(#ch48-ring-gov)" stroke-width="2.4"/>
    <circle cx="104" cy="36" r="2.8" fill="url(#ch48-core-gov)"/>
    <g>
      <circle cx="64" cy="62" r="7.0" fill="var(--logo-node-bg, #e2e8f0)"/>
      <circle cx="64" cy="62" r="6.0" fill="url(#ch48-pupil)" stroke="url(#ch48-gold-mira)" stroke-width="2.2"/>
      <line x1="55.0" y1="62" x2="60.0" y2="62" stroke="url(#ch48-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
      <line x1="68.0" y1="62" x2="73.0" y2="62" stroke="url(#ch48-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
      <line x1="64" y1="53.0" x2="64" y2="58.0" stroke="url(#ch48-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
      <line x1="64" y1="66.0" x2="64" y2="71.0" stroke="url(#ch48-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
      <circle cx="64" cy="62" r="2.0" fill="url(#ch48-gold-core)"/>
    </g>
  </g>
  <g transform="translate(148, 0)">
    <text x="2" y="27" fill="var(--logo-sub, #1d4ed8)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.2">PART IV · THE FRAMEWORK</text>
    <text x="498" y="27" text-anchor="end" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.0">CH48 · S_GOV</text>
    <text x="0" y="59" fill="var(--logo-ink, #0f172a)" font-family="system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif" font-size="20.2" font-weight="800" letter-spacing="1.15">REACTIVE WORLDS AND GOODHART'S LAW</text>
    <line x1="2" y1="78" x2="384" y2="78" stroke="url(#ch48-ring)" stroke-width="1.6" stroke-opacity="0.82"/>
    <circle cx="400" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="415" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.0"/>
    <circle cx="415" cy="78" r="1.6" fill="var(--logo-blue, #2563eb)"/>
    <circle cx="430" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="445" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="460" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="475" cy="78" r="6.2" stroke="var(--logo-gold, #f59e0b)" stroke-width="1.5" stroke-dasharray="2.2 1.6"/>
    <circle cx="475" cy="78" r="4.0" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-crimson, #e11d48)" stroke-width="2.2"/>
    <circle cx="475" cy="78" r="1.8" fill="var(--logo-gold, #f59e0b)"/>
    <circle cx="490" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-crimson, #e11d48)" stroke-width="2.0"/>
    <circle cx="490" cy="78" r="1.6" fill="var(--logo-crimson, #e11d48)"/>
    <text x="2" y="104" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.8" font-weight="600" letter-spacing="0.65">STORM CORP-PAV BRIER · INVARIANT CAUSAL PREDICTION · PERDOMO AND GOODHART</text>
  </g>
  </svg>
</figure>

## By the end of this chapter you can

1. Separate resolvable miscalibration from discriminative resolution on a forecast archive via tie-pooled CORP-PAV, and distinguish causal invariance falsification from low-power collapse or extrapolation beyond anchored support.
2. Compute performative stability ($\theta\_{\mathrm{PS}}$), performative optimality ($\theta\_{\mathrm{PO}}$), and the contraction modulus $|\varepsilon|/(1+\lambda)$ from closed forms, and separate causal improvement effort from Goodhart proxy gaming and unanchored latent-space dreaming.
3. Diagnose when repeated retraining loses contractivity or turns its own audit into a target, and state when an institution must replace iteration with an anchored, sunsetted rule freeze or an ungameable objective.

An agent can construct an honest **Epistemic Margin** ($S\_{\mathrm{epi}}$, [Chapters 43](43-inquiry-openness-multiverse.md)–[45](45-world-to-archive-big-data-paradox.md)) and buffer its **Vital Margin** ($S\_{\mathrm{vital}}$, [Chapters 46](46-non-identical-asymmetry-heavy-tails.md)–[47](47-irreversible-time-vital-margin.md)). Then it must act. As soon as the stewardship council of *The Community and the Reserve* ([Chapter 42](42-sequential-loop-naive-agent.md)) publishes allocation quotas and scorecards under proxy gaming pressure ($\beta\_{\mathrm{proxy}}$) and status discounting of frontline warnings ($\delta\_{\mathrm{status}}$), the world stops acting like a passive urn of marbles. People adapt around the rule.

**Mediating Filter III** (**Axis 3 — $S\_{\mathrm{gov}}$: Institutional Governance**) guards that return path from decision back into the reactive social world. Its core **Tension 6** pits passive curve-fitting and self-reinforcing stability against four institutional safeguards. First is isotonic calibration via CORP-PAV decomposition (Dimitriadis, Gneiting & Jordan, 2021; Kennedy & O'Hagan, 2001). Second is causal invariance across environments and counterfactual attribution (Tian & Pearl, 2000; Hudgens & Halloran, 2008; Peters, Bühlmann & Meinshausen, 2016). Third is performative optimality under strategic feedback (Lucas, 1976; MacKenzie, 2006; Espeland & Sauder, 2007; Milli et al., 2019; Perdomo et al., 2020). Fourth is Goodhart's Law under metric gaming (Goodhart, 1975; Strathern, 1997; Manheim & Garrabrant, 2018; Kleinberg & Raghavan, 2020).

In Stochology's architecture, this chapter builds **Gate 1 (Ex-Ante Calibration, Causal Invariance & Performative Audit)**, written $g\_{\text{ex-ante}, t}$, inside **Canonical Operator III**. [Chapter 49](49-moral-luck-apt-agency-institutions.md) then joins Gate 1 with real-time timely intervention (*kairós*, $g\_{\mathrm{kairos}, t}$) and ex-post attribution ($g\_{\text{ex-post}, t}$) to complete the per-step governance minimum $\mathcal{O}\_{\mathrm{gov}}(\pi\_t, \mathcal{I}\_t)$—whose trajectory infimum folds into $S\_{\mathrm{gov}}$ alongside reception-basin integrity $\Theta\_{\mathrm{basin}}$. Deep archive decomposition, ICP screening, and contraction audits in Gate 1 run **offline** between deployment cycles so live triage needs only fast stop-triggers, while **intra-cycle volatility, subgroup shift, and runaway drift circuit-breakers** wire directly into Gate 2 ([Chapter 43](43-inquiry-openness-multiverse.md); [Chapter 49](49-moral-luck-apt-agency-institutions.md)) so rapid counterperformative cascades cannot outrun survival latency within a deployment cycle.

---

## 7.1 Isotonic CORP-Murphy Brier Score Decomposition via PAV

Suppose a model emits probabilistic forecasts $p\_1, \dots, p\_n \in [0, 1]$ for binary outcomes $y\_1, \dots, y\_n \in \lbrace{}0, 1\rbrace$. Before trusting those numbers, an institution must ask two separate questions. **Honest calibration:** when the model predicts $30\%$ risk, does the event occur roughly $30\%$ of the time? **Discriminative resolution:** can the model separate high-risk shifts from calm shifts?

Replacing arbitrary histogram bins (Murphy, 1973) with the **Consistent, Optimal, Reproducible, PAV-based (CORP)** decomposition of the Brier Score (Brier, 1950; Dimitriadis, Gneiting, & Jordan, 2021; [Chapter 37](37-model-fit-and-error.md); [Appendix A](appendix-a-mathematics.md) §A.3) solves both together. Tie-pooled **Pool-Adjacent-Violators (PAV)** monotone recalibration $\hat{\pi}\_i = \hat{\pi}(p\_i)$ leaves residuals $y - \hat{\pi}$ orthogonal to centered fits $\hat{\pi} - \bar{y}\mathbf{1}$, yielding an exact identity without tuning bin edges: $\mathrm{BS}(p) = \mathrm{REL}\_{\mathrm{CORP}} - \mathrm{RES}\_{\mathrm{CORP}} + \mathrm{UNC}\_0$. Here $\mathrm{REL}\_{\mathrm{CORP}} = \mathrm{BS}(p) - \mathrm{BS}(\hat{\pi}) = \frac{1}{n}\sum\_{i=1}^n (p\_i - \hat{\pi}\_i)^2 + \frac{2}{n}\sum\_{i=1}^n (p\_i - \hat{\pi}\_i)(\hat{\pi}\_i - y\_i) \ge 0$ retains its PAV orthogonality cross-term whenever ties pool or monotonicity activates, isolating resolvable miscalibration ($\mathrm{REL}\_{\mathrm{CORP}}$) and discriminative power ($\mathrm{RES}\_{\mathrm{CORP}}$) from climatological base-rate variance ($\mathrm{UNC}\_0 = \bar{y}(1 - \bar{y})$). Crucially, a trivial constant forecast $p\_i \equiv \bar{y}$ achieves zero miscalibration ($\mathrm{REL}\_{\mathrm{CORP}} = 0$) yet zero resolution ($\mathrm{RES}\_{\mathrm{CORP}} = 0$), proving that good calibration alone never certifies predictive skill ([Chapter 37](37-model-fit-and-error.md); [Chapter 50](50-methodological-families-reflexivity.md)).

### 7.1.1 In-Sample PAV Bias and Out-of-Fold Debiasing

Fitting isotonic curve $\hat{\pi}$ on the same outcomes used to score it creates a subtle trap. Because PAV absorbs finite-sample noise in-sample, it understates true population Brier loss $\mathrm{BS}(\pi^\ast)$, artificially boosting $\mathrm{RES}\_{\mathrm{CORP}}$ and biasing in-sample $\mathrm{REL}\_{\mathrm{CORP}}(p) = \mathrm{BS}(p) - \mathrm{BS}(\hat{\pi})$ **upward** ([Chapter 37](37-model-fit-and-error.md)).

To strip out that optimism without leaking future trends, Gate 1 splits each stratum along **pre-registered** disjoint contiguous blocks—such as distinct storm events or time windows, never interleaved adjacent rows or post-hoc splits on correlated series ([Chapter 37](37-model-fit-and-error.md); [Chapter 50](50-methodological-families-reflexivity.md) §9.3.1). We fit isotonic envelopes inside each training fold, evaluate recalibrated loss out-of-fold, compute the empirical optimism $\hat{b}\_{\mathrm{PAV}}^{\mathrm{CV}}$ over in-sample $\mathrm{BS}(\hat{\pi})$ together with **out-of-fold within-stratum debiased resolution** $\mathrm{RES}\_{\mathrm{CORP}}^{\mathrm{debiased}}$, and obtain debiased calibration error $\mathrm{REL}\_{\mathrm{CORP}}^{\mathrm{debiased}}$:
$$\mathrm{REL}\_{\mathrm{CORP}}^{\mathrm{debiased}} = \max\bigl(0,\mkern3mu \mathrm{REL}\_{\mathrm{CORP}} - \hat{b}\_{\mathrm{PAV}}^{\mathrm{CV}}\bigr)$$

In plain words, subtracting the cross-validated optimism term $\hat{b}\_{\mathrm{PAV}}^{\mathrm{CV}}$ keeps minor finite-sample overfitting from masquerading as structural miscalibration—yet subtracting optimism from $\mathrm{REL}\_{\mathrm{CORP}}$ alone fails if uninformative noisy forecasts inflate $\hat{b}\_{\mathrm{PAV}}^{\mathrm{CV}}$ while boasting spurious in-sample $\mathrm{RES}\_{\mathrm{CORP}} > 0$. Under Reflexive Brake 1 ([Chapter 50](50-methodological-families-reflexivity.md) §9.3.1), Gate 1 therefore **rejects certification** whenever out-of-fold raw Brier loss fails to beat within-stratum climatology ($\mathrm{BS}\_{\mathrm{CV}}(p) \ge \mathrm{UNC}\_0$), whenever $\mathrm{RES}\_{\mathrm{CORP}}^{\mathrm{debiased}} \le 0$, or whenever $\hat{b}\_{\mathrm{PAV}}^{\mathrm{CV}}$ exceeds a pre-registered ceiling. This supplies an empirical finite-sample bias adjustment under pre-registered blocks, not a distribution-free finite-sample guarantee.

### 7.1.2 Continuous Ranked Probability Score (CRPS) Decomposition

When outcomes are continuous rather than binary, take any predictive cumulative distribution $F$ with finite first moment ($\mathbb{E}\_F[|Y|] < \infty$) against realized scalar $y \in \mathbb{R}$. Hersbach (2000) wrote the Continuous Ranked Probability Score $\mathrm{CRPS}(F, y)$ as the integral of threshold Brier scores $\mathrm{BS}$ across all cutoffs $\tau$:
$$\mathrm{CRPS}(F, y) = \int\_{-\infty}^\infty \mathrm{BS}\bigl(F(\tau),\mkern3mu \mathbf{1}\lbrace{}y \le \tau\rbrace\bigr)\mkern3mu d\tau$$

In plain words, $\mathrm{CRPS}$ runs a binary calibration check at every possible threshold $\tau$ from $-\infty$ to $+\infty$ and adds up the losses. Applying tie-pooled CORP-PAV on a fine threshold grid splits $\mathrm{CRPS}$ into integrated calibration loss, resolution, and irreducible base-rate spread up to grid error—showing whether error stems from poor calibration, wide fuzziness, or baseline volatility.

---

## 7.2 Invariant Causal Prediction (ICP), Kennedy–O'Hagan Structural Discrepancy, and Tian–Pearl Bounds

Even a perfectly calibrated correlation collapses as soon as an institution intervenes (`do(X = x)`). Judea Pearl (2009) fixed this boundary with his **three-level causal hierarchy** ([Chapter 39](39-cause-intervention-counterfactual.md)): passive observation $\mathbb{P}(Y \mid X = x)$ (Rung 1) tangles true mechanisms together with shifting proxies, common confounders, and selection colliders.

![Confounder versus Berkson Collider in Causal Graphs](figures/fig-03-causal-dags.svg)

In the left diagram, an unmeasured confounder $C$ opens a non-causal back-door path between predictor $X$ and outcome $Y$. In the right diagram, conditioning on a downstream Berkson collider $B$ creates spurious correlation between independent causes. Across diverse environments $e \in \mathcal{E}$ that perturb $X$ or upstream drivers without altering $Y$ directly, models built on confounders or colliders shift slope across regimes. Three tools anchor Gate 1 across all three rungs of causation.

### 7.2.1 Invariant Causal Prediction (ICP) Across Heterogeneous Environments

Jonas Peters, Peter Bühlmann, and Nicolai Meinshausen (2016) formalized **Invariant Causal Prediction (ICP)** ([Chapter 39](39-cause-intervention-counterfactual.md)). Suppose there is no hidden confounding between candidate predictors $X$ and $Y$, and no environment $e \in \mathcal{E}$ intervenes directly on $Y$ (blocking reactive certification up front whenever performativity $\varepsilon \ne 0$ or policy shifts act directly on $Y$). Then the conditional law $Y^{(e)} \mid X\_{\mathbf{PA}(Y)}^{(e)}$ given $Y$'s true causal parents $\mathbf{PA}(Y)$ stays invariant across all regimes $e \in \mathcal{E}$, whereas spurious proxy regressions drift in intercept or slope across environments.

To locate true causal drivers, Gate 1 tests candidate predictor index subsets $S \subseteq \lbrace{}0, \dots, d-1\rbrace$ (distinct from Triad axis scores $S\_{\mathrm{epi}}, S\_{\mathrm{vital}}, S\_{\mathrm{gov}}$). The empty subset $S = \emptyset$ tests whether $Y^{(e)}$ has constant mean and variance across $|\mathcal{E}|$ environments ($|\mathcal{E}| - 1$ and $n\_{\mathrm{total}} - |\mathcal{E}|$ degrees of freedom). For each non-empty subset ($|S| \ge 1$), we compare the pooled residual sum of squares $\mathrm{RSS}\_{\mathrm{restricted}}$ (a single intercept and slope vector $\beta\_S$ shared across all environments) against the joint unrestricted sum of squares $\mathrm{RSS}\_{\mathrm{unrestricted}}$ fitting **separate per-environment intercepts $\alpha\_e$ and environment-specific slopes $\beta\_{S,e}$** across all $e \in \mathcal{E}$, so pure conditional-mean shifts cannot pass undetected. We compare them via the Chow $F$-statistic $F\_S$ with degrees of freedom $df\_1 = (|\mathcal{E}| - 1)(|S| + 1) > 0$ ($|\mathcal{E}| \ge 2$) and $df\_2 = n\_{\mathrm{total}} - |\mathcal{E}|(|S| + 1) > 0$ ($\mathrm{RSS}\_{\mathrm{unrestricted}} > 0$):
$$F\_S = \frac{(\mathrm{RSS}\_{\mathrm{restricted}} - \mathrm{RSS}\_{\mathrm{unrestricted}}) / df\_1}{\mathrm{RSS}\_{\mathrm{unrestricted}} / df\_2}$$

In plain words, $F\_S$ jointly tests whether conditional intercepts or slopes on subset $S$ jump significantly from one environment to another. A subset enters the accepted invariant family $\mathcal{A}$ only if $F\_S \le F\_{1-\alpha}(df\_1, df\_2)$, cross-environment residual-mean equality holds, and residual variance invariance passes across environments (checking full residual distribution invariance whenever non-Gaussian or quantile checks apply). Intersecting all accepted subsets guarantees $\mathbb{P}(\hat{S}^\ast \subseteq \mathbf{PA}(Y)) \ge 1 - \alpha$ ([Chapter 39](39-cause-intervention-counterfactual.md)).

#### The Agnotological Low-Power Guard

Two completely opposite empty outcomes must never be confused ([Chapter 39](39-cause-intervention-counterfactual.md)). Getting an empty accepted family ($\mathcal{A} = \emptyset$) falsifies the invariance premises (pointing to latent confounding, missing causes, or direct intervention on $Y$). By contrast, having the empty set itself pass ($\emptyset \in \mathcal{A}$) because data are too noisy or proxies are collinear silently collapses the intersection to $\hat{S}^\ast = \emptyset$. Gate 1 flags underpowered non-rejection and buffers at its upper bound $U\_t$ rather than mistaking weak tests for proof of zero causal link ([Chapter 43](43-inquiry-openness-multiverse.md)).

### 7.2.2 Kennedy & O'Hagan's (2001) Anchored Gaussian Process Discrepancy

No simulator $f\_\theta(x)$ matches the real world without structural error. Marc C. Kennedy and Anthony O'Hagan (2001) express observations as $y(x) = f\_\theta(x) + \delta(x) + \epsilon\_{\mathrm{obs}}$ with measurement noise $\epsilon\_{\mathrm{obs}} \sim \mathcal{N}(0, \sigma\_{\mathrm{noise}}^2)$ ([Chapter 37](37-model-fit-and-error.md)), modeling unknown structural discrepancy $\delta(x) \sim \mathcal{GP}(0, k(x, x'))$ as a Gaussian process. With output scale $s > 0$ and correlation length $\ell > 0$, the squared-exponential covariance kernel $k(x, x')$ between inputs $x$ and $x'$ is:
$$k(x, x') = s^2 \exp\left(-\frac{(x - x')^2}{2\ell^2}\right)$$

In plain words, discrepancy values at nearby operating points ($|x - x'| \lesssim \ell$) correlate strongly, whereas points far outside observed experience decouple rapidly. Because $(s, \ell)$ are weakly identifiable inside Kennedy–O'Hagan discrepancy fits, Gate 1 pins $(s, \ell)$ via **pre-registered out-of-sample block validation** paired with **hard geometric distance caps in input space independent of $\ell$**, blocking analysts from inflating $\ell$ or shrinking $s \to 0^+$ to pass unanchored states. Conditioning $\delta(x)$ on anchor residuals separates two operating regimes ([Chapter 37](37-model-fit-and-error.md)):

- **In-Support Regime:** Near dense observations within validated length $\ell$ and geometric caps (or when $\sigma\_{\mathrm{noise}} \ll s$), posterior uncertainty $\sigma\_{\mathrm{post}}(x^\ast)$ shrinks toward observation noise $\sigma\_{\mathrm{noise}}$, cleanly isolating structural simulator bias.
- **Extrapolation Regime:** Outside anchored history ($|x^\ast - x\_{\mathrm{train}}| \gg \ell$ or outside geometric caps, entering the unobserved null space $\ker(P)$ of [Chapter 14](14-linear-algebra.md)), kernel correlation vanishes ($\mathbf{k}\_\ast \to \mathbf{0}$) and uncertainty reverts to the prior ceiling $\sqrt{s^2 + \sigma\_{\mathrm{noise}}^2}$—pricing extrapolation ignorance and forcing physical buffers.

### 7.2.3 Jin Tian & Judea Pearl's (2000) Sharp Counterfactual Bounds

Assigning responsibility *after* an accident belongs to Rung 3 counterfactuals. Consider binary intervention $X \in \lbrace{}0, 1\rbrace$ and binary outcome $Y \in \lbrace{}0, 1\rbrace$ satisfying causal consistency ($X = 1 \implies Y\_{x\_1} = Y$ and $X = 0 \implies Y\_{x\_0} = Y$). Combining experimental rates ($p\_{y\_1} = \mathbb{P}(Y\_{x\_1}=1)$, $p\_{y\_0} = \mathbb{P}(Y\_{x\_0}=1)$) with observational joint shares ($p\_{xy} = \mathbb{P}(X=x, Y=y)$) first requires compatibility. Specifically, the margins must satisfy $p\_{11} \le p\_{y\_1} \le 1 - p\_{10}$ and $p\_{01} \le p\_{y\_0} \le 1 - p\_{00}$. Violating those inequalities refutes structural consistency between experimental and observational sources (Tian & Pearl, 2000; [Chapter 39](39-cause-intervention-counterfactual.md)).

When compatible, Jin Tian and Judea Pearl (2000) proved distribution-free sharp bounds on three causal attribution quantities:

- **Probability of Necessity ($\mathrm{PN}$):** Chance that treated harm $Y=1$ would not have happened without $X=1$ (defined when $\mathbb{P}(X=1, Y=1) > 0$; else $[0, 1]$):
  $$\max\left(0,\mkern3mu \frac{\mathbb{P}(Y=1) - \mathbb{P}(Y\_{x\_0}=1)}{\mathbb{P}(X=1, Y=1)}\right) \le \mathrm{PN} \le \min\left(1,\mkern3mu \frac{\mathbb{P}(Y\_{x\_0}=0) - \mathbb{P}(X=0, Y=0)}{\mathbb{P}(X=1, Y=1)}\right)$$
- **Probability of Sufficiency ($\mathrm{PS}$):** Chance that treating an untreated non-responder ($X=0, Y=0$) would have flipped the outcome to $Y=1$ (defined when $\mathbb{P}(X=0, Y=0) > 0$; else $[0, 1]$):
  $$\max\left(0,\mkern3mu \frac{\mathbb{P}(Y\_{x\_1}=1) - \mathbb{P}(Y=1)}{\mathbb{P}(X=0, Y=0)}\right) \le \mathrm{PS} \le \min\left(1,\mkern3mu \frac{\mathbb{P}(Y\_{x\_1}=1) - \mathbb{P}(X=1, Y=1)}{\mathbb{P}(X=0, Y=0)}\right)$$
- **Probability of Necessity and Sufficiency ($\mathrm{PNS}$):** Chance that treatment flips the outcome both ways ($Y\_{x\_1}=1 \land Y\_{x\_0}=0$), equal to $\mathrm{PNS} = p\_{11}\mkern2mu\mathrm{PN} + p\_{00}\mkern2mu\mathrm{PS}$ inside [Chapter 39](39-cause-intervention-counterfactual.md)'s Tian–Pearl envelope.

In plain words, these sharp intervals bound counterfactual causation directly from experimental and observational margins without guessing unobserved individual correlations.

---

## 7.3 Performative Prediction ($\theta\_{\mathrm{PS}}$ vs. $\theta\_{\mathrm{PO}}$), MacKenzie's Regimes, and Goodhart's Four Variants

Publish a predictive rule inside a living community and people react to it. Juan Perdomo, Tijana Zrnic, Celestine Mendler-Dünner, and Moritz Hardt (2020) formalized that feedback loop as **performative prediction**.

### 7.3.1 Performative Stability, Performative Optimality, and Failure of Offline Robustness

Write $\theta \in \Theta$ for the published model parameter and $\mathcal{D}(\theta)$ for the outcome distribution induced when agents respond to $\theta$. In macroeconomics this is the **Lucas critique** (Lucas, 1976): historical correlations break as soon as actors adjust expectations to policy shifts. Across interconnected neighborhoods or programs, **treatment interference** (Hudgens & Halloran, 2008) compounds that shift because intervening on one group changes incentives and outcomes in neighboring cohorts.

Four definitions separate passive retraining from true optimal design:

- **Decoupled Loss:** $\mathcal{L}(\theta; \theta') = \mathbb{E}\_{Z \sim \mathcal{D}(\theta')}[\ell(Z; \theta)]$ scores rule $\theta$ on the population induced by rule $\theta'$.
- **Repeated Risk Minimization (RRM):** Standard iterative retraining sets $\theta\_{t+1} = \arg\min\_\theta \mathcal{L}(\theta; \theta\_t)$.
- **Performative Stability ($\theta\_{\mathrm{PS}}$):** An RRM fixed point $\theta\_{\mathrm{PS}} = \arg\min\_\theta \mathcal{L}(\theta; \theta\_{\mathrm{PS}})$ looks optimal on the very data stream its own deployment induces.
- **Performative Optimality ($\theta\_{\mathrm{PO}}$):** A Stackelberg planner minimizes true performative risk $\mathrm{PR}(\theta) = \mathcal{L}(\theta; \theta)$, choosing $\theta\_{\mathrm{PO}} = \arg\min\_\theta \mathrm{PR}(\theta)$.

Why do stability and optimality split? Differentiating performative risk $\mathrm{PR}(\theta)$ splits total slope into two pieces:
$$\nabla \mathrm{PR}(\theta) = \underbrace{\left.\nabla\_\theta \mathcal{L}(\theta; \theta')\right|\_{\theta'=\theta}}\_{\text{Standard Training Gradient}} + \underbrace{\left.\nabla\_{\theta'} \mathcal{L}(\theta; \theta')\right|\_{\theta'=\theta}}\_{\text{Performative Distribution Gradient}}$$

In plain words, RRM chases only the first gradient—ignoring how changing $\theta$ alters the world $\mathcal{D}(\theta')$. At a stable fixed point $\theta\_{\mathrm{PS}}$, the standard gradient hits zero while the distribution-shift gradient persists, so performative stability generally misses performative optimality.

A simple closed-form model makes the divergence concrete. Let outcomes shift linearly in mean with the deployed rule while holding $\theta$-invariant conditional variance ($\partial\_\theta\mathrm{Var}(Y \mid \theta) = 0$; [Appendix A](appendix-a-mathematics.md) §A.3), specializing to $Y \sim \mathcal{N}(\mu\_0 + \varepsilon\theta, \sigma^2)$ under quadratic loss $\ell(y; \theta) = \frac{1}{2}(y - \theta)^2 + \frac{\lambda}{2}\theta^2$ with externally pre-registered regularization penalty $\lambda > 0$ ($\varepsilon \ne 1 + \lambda$). Solving for the stable fixed point $\theta\_{\mathrm{PS}}$, the true optimum $\theta\_{\mathrm{PO}}$, and their gap yields:
$$\theta\_{\mathrm{PS}} = \frac{\mu\_0}{1 + \lambda - \varepsilon}, \qquad \theta\_{\mathrm{PO}} = \frac{(1 - \varepsilon)\mu\_0}{(1 - \varepsilon)^2 + \lambda}, \qquad \theta\_{\mathrm{PS}} - \theta\_{\mathrm{PO}} = \frac{\varepsilon\lambda\mu\_0}{(1 + \lambda - \varepsilon)\bigl((1 - \varepsilon)^2 + \lambda\bigr)}$$

Whenever sensitivity $\varepsilon \ne 0$ and regularization $\lambda > 0$, retraining settles away from the system optimum. Perdomo et al.'s (2020) general sufficient condition requires $|\varepsilon| < \gamma / \beta\_{\mathrm{joint}}$, where $\gamma$ is strong convexity in $\theta$ and $\beta\_{\mathrm{joint}}$ is the coordinate-wise joint Lipschitz constant of $\nabla\_\theta \ell(z; \theta)$ in $(z, \theta)$ (Assumption A1). For quadratic loss ($\lambda > 0$), $\gamma = 1 + \lambda$ and $\beta\_{\mathrm{joint}} = \max(1 + \lambda, 1) = 1 + \lambda$, giving $|\varepsilon| < 1$. Tracking only mixed sensitivity $L\_y = |\partial\_y \partial\_\theta \ell| = 1$ yields the exact linear RRM update $\theta\_{t+1} = (\mu\_0 + \varepsilon\theta\_t)/(1 + \lambda)$ with modulus $\kappa\_{\mathrm{RRM}} = |\varepsilon|L\_y / \gamma = |\varepsilon|/(1 + \lambda)$, which converges to $\theta\_{\mathrm{PS}}$ if and only if $|\varepsilon| < 1 + \lambda$ (refining [Chapter 37](37-model-fit-and-error.md)'s optimization error $\mathcal{E}\_{\mathrm{opt}}$ under endogenous shift). Whereas $\kappa\_{\mathrm{RRM}}$ is a reactivity-to-regularization ratio, $\theta\_{\mathrm{PS}} - \theta\_{\mathrm{PO}}$ vanishes at both $\lambda \to 0^+$ ($|\varepsilon| < 1$) and $\lambda \to \infty$; Gate 1 therefore pre-registers $\lambda$ externally and caps $|\varepsilon|$ independently of $\lambda$ so inflated $\lambda$ cannot mask destabilizing reactivity.

Passive **distributionally robust optimization (DRO)** fails under this endogenous feedback ([Chapter 44](44-underdetermination-partial-identification.md)): shifting $\theta\_0$ to $\theta$ pushes the world by $W\_1(\mathcal{D}(\theta), \mathcal{D}(\theta\_0)) = |\varepsilon(\theta - \theta\_0)|$ outside any fixed offline ball $\rho\_W$ or historical $f$-divergence ball (or bounded support). Worse, even Stackelberg optimality $\theta\_{\mathrm{PO}}$ ignores cross-boundary harm when institutional minimization dumps risk or exclusions onto outside groups whose external reception perimeters ($\Theta\_{\mathrm{basin}}$) saturate beyond $\mathcal{L}$ ([Chapter 49](49-moral-luck-apt-agency-institutions.md); [Appendix A](appendix-a-mathematics.md) §A.7).

### 7.3.2 Donald MacKenzie's (2006) Performativity Taxonomy

Donald MacKenzie, in *An Engine, Not a Camera* (2006), built on Michel Callon's (1998) sociology of market devices—corroborated in public administration by Wendy Espeland and Michael Sauder's (2007) study of U.S. law school rankings, where published rank scores reshaped admissions and budgets. MacKenzie contrasted a passive **camera baseline** (the model mirrors an unmoved world, $\varepsilon = 0$) with four performative modes:

1. **Generic Performativity:** Actors consult the model or score routinely when making choices, without necessarily steering outcomes toward its forecasts.
2. **Effective Performativity:** Adopting the model causally changes institutional behavior relative to a world without it (a descriptive causal shift, not a welfare gain).
3. **Barnesian Performativity:** Deployment reshapes practices so realized outcomes match the model's assumptions more closely—the self-validating loop analyzed by Barnes (1983). Yet in the regularized linear model above ($\lambda > 0$), RRM's mean bias $\mu(\theta\_{\mathrm{PS}}) - \theta\_{\mathrm{PS}} = \lambda\mu\_0/(1+\lambda-\varepsilon)$ *grows in magnitude* ($|\mu(\theta\_{\mathrm{PS}}) - \theta\_{\mathrm{PS}}|$, when $\mu\_0 \ne 0$) with $\varepsilon \in [0, 1+\lambda)$, so RRM convergence under regularization must not be mistaken for zero-error Barnesian alignment.
4. **Counterperformativity:** Deployment triggers feedback that *undermines* the model's accuracy, as when portfolio-insurance dynamic hedging amplified the October 1987 market crash and created a lasting options volatility skew contradicting constant-volatility Black–Scholes assumptions (MacKenzie, 2006).

These four modes describe the **qualitative direction** of world-to-model feedback—an axis separate from both contraction rate $\kappa\_{\mathrm{RRM}}$ and optimization gap $|\theta\_{\mathrm{PS}} - \theta\_{\mathrm{PO}}|$.

### 7.3.3 Strategic Effort, Disproportionate Burden, and Goodhart's Four Variants

When rewards or penalties attach to a proxy $M$, agents reallocate effort. Jon Kleinberg and Manish Raghavan (2020) separate productive effort toward true goal $G$ (**causal improvement effort**, $x\_{\mathrm{real}}$) from cheap proxy manipulation (**cosmetic gaming**, $x\_{\mathrm{game}}$). Smitha Milli et al. (2019) prove further that optimizing a strategic classifier creates an aggregate **social burden** $B^+(f)$—the adaptation cost people pay to clear a shifted cutoff—that falls heaviest on disadvantaged groups facing higher adaptation costs, draining external reception perimeters ($\Theta\_{\mathrm{basin}}$) even when headline accuracy looks stable.

Connecting [Chapter 07](07-when-the-measure-becomes-the-goal.md) and [Chapter 40](40-institution-incentive-metric.md), David Manheim and Scott Garrabrant's (2018) four-variant Goodhart taxonomy highlights **Extremal Goodhart** (pushing proxy $M$ off its calibrated domain) and **Adversarial Goodhart** (gaming $M$ more cheaply than advancing $G$) as the two reactive failures that defeat offline validation.

---

## 7.4 Invariant Causal Prediction on a Toy System

While CORP-PAV recalibration (§7.1; [Chapter 37](37-model-fit-and-error.md)) and performative contractivity (§7.3) govern calibration and fixed-point stability, walking through [Chapter 39](39-cause-intervention-counterfactual.md)'s ICP screen in symbols shows how Gate 1 isolates a direct cause from a spurious proxy. Consider predicting $Y$ from a direct cause $X\_1$ and a spurious sibling proxy $X\_2$ across two regimes $e \in \lbrace{}1, 2\rbrace$:

- **Ground Truth:** $Y^{(e)} = \beta X\_1^{(e)} + \epsilon\_Y^{(e)}$ ($\beta \ne 0$) with invariant noise $\epsilon\_Y^{(e)} \sim \mathcal{N}(0, \sigma\_Y^2)$ independent of $(X\_1^{(e)}, X\_2^{(e)})$, while the causal driver shifts variance across regimes, $\sigma\_{1,e}^2 = \mathrm{Var}(X\_1^{(e)})$ with $\sigma\_{1,2}^2 \ne \sigma\_{1,1}^2$ (or shifts its mean).
- **Proxy Coupling:** Across calm ($e=1$) and storm ($e=2$) regimes, $X\_2^{(e)} = \kappa\_e X\_1^{(e)} + \eta\_e$ with independent proxy noise $\mathrm{Var}(\eta\_e) = \sigma\_{\eta,e}^2 > 0$ and $\kappa\_2 \ne \kappa\_1$. An observational regression of $Y^{(e)}$ on $X\_2^{(e)}$ alone yields the attenuated slope $\frac{\beta\kappa\_e\sigma\_{1,e}^2}{\kappa\_e^2\sigma\_{1,e}^2 + \sigma\_{\eta,e}^2}$, which shifts across environments.

Screening subsets by the Chow joint intercept-and-slope test (§7.2.1, $df\_1, df\_2$) alongside cross-environment residual variance invariance:

1. **Candidate $S = \lbrace{}X\_1\rbrace$:** Intercept $0$, slope $\beta$, and noise variance $\sigma\_Y^2$ remain invariant across both regimes; the subset is **accepted**.
2. **Candidate $S = \lbrace{}X\_2\rbrace$:** Both the attenuated regression intercept/slope and residual variance shift between regimes $1$ and $2$; the subset is **rejected**.
3. **Empty Set $S = \emptyset$:** Because the cause variance $\sigma\_{1,e}^2$ shifts (or its mean shifts), the marginal distribution of $Y^{(e)}$ differs across environments, so $S = \emptyset$ is **rejected**.
4. **Candidate $S = \lbrace{}X\_1, X\_2\rbrace$:** Conditioning on $X\_1$ blocks the back-door correlation with $X\_2$, keeping conditional intercept $0$, slope $(\beta, 0)$, and residual distribution invariant; the subset is **accepted**.

Intersecting accepted subsets gives $\hat{S}^\ast = \lbrace{}X\_1\rbrace \cap \lbrace{}X\_1, X\_2\rbrace = \lbrace{}X\_1\rbrace$. In plain words, ICP isolates direct cause $X\_1$ cleanly while rejecting the empty set $S = \emptyset$ only because shifting environments perturbed the marginal mean or spread of $Y$.

---

## 7.5 The Gate on a Storm Archive and the Reactive Reserve

Our physical benchmark is a tropical-storm track archive recorded fix by fix at a constant lead time along same-storm paths. Because weather forecasts cannot steer cyclones, that track benchmark sits cleanly on the passive **Camera** baseline ($\varepsilon = 0$). It lets us isolate calibration and causal invariance before strategic human reactions enter.

Partition the archive strictly along **pre-registered storm identifiers or chronological blocks**—never by shuffled row indices across six-hourly dependencies ([Chapter 37](37-model-fit-and-error.md))—and apply tie-pooled CORP-PAV (§7.1). Remember that a constant forecast sets $\mathrm{REL}\_{\mathrm{CORP}} = \mathrm{RES}\_{\mathrm{CORP}} = 0$. Out-of-fold block debiasing stops finite-sample PAV overfitting from skewing calibration scores (Murphy, 1973; Dimitriadis, Gneiting & Jordan, 2021), while requiring $\mathrm{BS}\_{\mathrm{CV}}(p) < \mathrm{UNC}\_0$ and positive out-of-fold resolution $\mathrm{RES}\_{\mathrm{CORP}}^{\mathrm{debiased}} > 0$ prevents noisy forecasts from clearing on $\mathrm{REL}\_{\mathrm{CORP}}^{\mathrm{debiased}} = 0$ alone. Across seasonal regimes $e \in \mathcal{E}$, ICP then tests a physical driver such as storm pressure deficit against a season-tied proxy such as latitude (Peters, Bühlmann & Meinshausen, 2016), while Kennedy–O'Hagan discrepancy and Tian–Pearl bounds verify out-of-support margins and retrospective attribution.

In *The Community and the Reserve* ([Chapter 42](42-sequential-loop-naive-agent.md)), the same gate leaves the passive Camera baseline behind. Once reserve releases or inspection quotas respond to published proxy scores ($\beta\_{\mathrm{proxy}}$), households and managers alter compliance and reporting ($\varepsilon \ne 0$). Decisions also couple neighboring cohorts through treatment interference (Hudgens & Halloran, 2008). Performative divergence ($\theta\_{\mathrm{PS}} \ne \theta\_{\mathrm{PO}}$) and spillover across external reception perimeters ($\Theta\_{\mathrm{basin}}$) now outweigh passive archive errors.

### Stability is not merit

Converging under RRM certifies only a fixed point on the world that rule itself generates—never validity or performative optimality (Perdomo et al., 2020). A rule drifting past its validated baseline $\theta\_0$ leaves anchored support behind, where out-of-sample discrepancy grows with $\Vert\theta\_{\mathrm{PS}} - \theta\_0\Vert$ even while recalibration error looks tiny—just as falling $\mathrm{RES}\_{\mathrm{CORP}}^{\mathrm{debiased}}$ can leave Brier calibration intact. Honest offline audits track three pairs jointly: debiased calibration beside out-of-fold within-stratum resolution ($\mathrm{REL}\_{\mathrm{CORP}}^{\mathrm{debiased}}, \mathrm{RES}\_{\mathrm{CORP}}^{\mathrm{debiased}}$ under $\mathrm{BS}\_{\mathrm{CV}}(p) < \mathrm{UNC}\_0$), performative risk beside $\Vert\theta - \theta\_0\Vert$, and internal risk beside downstream load on external reception perimeters ($\Theta\_{\mathrm{basin}}$).

### When reactivity is negligible

Reactivity is negligible only when deployment shifts neither subgroup, quantile, and Wasserstein distributions ($\varepsilon \approx 0$ across all strata, not merely pooled mean $\varepsilon \approx 0$ hiding minority social burden $B^+(f)$) nor cross-unit behavior under an explicit two-stage cluster-saturation interference design (Hudgens & Halloran, 2008). Only when audited subgroup shift and saturation-tested interference both remain below pre-registered limits may offline tracking simplify to passive Camera checks. Once either crosses threshold—or triggers intra-cycle Gate 2 drift breakers—passive calibration checks stop binding.

---

## 7.6 Optimising inside the dream: extremal Goodhart in latent space

Section 7.2 showed that Kennedy–O'Hagan discrepancy $\delta(z)$ stays pinned only near observed data and snaps back to its prior ceiling $\sqrt{s^2 + \sigma\_{\mathrm{noise}}^2}$ outside anchored support. Writing the true goal as $G(z) = \hat{M}(z) + \delta(z)$ (net of observation noise), the surrogate satisfies $\hat{M}(z) = G(z) - \delta(z)$. Aim a greedy optimizer at $\hat{M}(z)$ alone and it hunts for states where $\hat{M}(z) - G(z) = -\delta(z)$ is huge: unanchored zones where $\delta(z) \ll 0$ so the surrogate wildly *overpredicts* reality.

In Manheim and Garrabrant's (2018) taxonomy, this latent-space illusion is **Extremal Goodhart**. It is neither passive noise regression nor outsider **Adversarial Goodhart**, but an optimizer driving states outside the anchored manifold where surrogate $\hat{M}$ and goal $G$ co-varied—including along unmonitored directions in the empirical null space $\ker(P)$ ([Chapter 14](14-linear-algebra.md)). Imagine $\hat{M}$ trained inside latent support radius $R$ (distinct from physical reserve $R\_t$ and recording indicator in $\rho\_{R,Y}$) while a real absorbing ruin barrier hides just past $R$. A greedy optimizer pushes latent state $z$ far outside the anchored ball ($\Vert z\Vert \gg R$), trading small modeled gains for massive negative discrepancy $\delta(z) \ll 0$ until real trajectories hit ruin while dreamed scores climb endlessly.

Teams often swap greedy argmax selection for a finite-temperature Boltzmann (softmax) policy. Yet entropy regularization is only a **policy-level smoothness** tweak, not distributionally robust optimization (DRO). Higher temperature spreads weight away from the sharpest error peak without correcting misspecified surrogate $\hat{M}$. Whenever surrogate optimism grows faster than policy entropy—or when an unmodeled cliff sits near the support edge—softmax still drifts off the safe manifold.

The operational safeguard splits policy tuning from evidence gates. Certification belongs to **Gate 1**: bound $\delta(z)$ inside anchored support under out-of-sample validated $(s, \ell)$ and hard geometric distance caps independent of $\ell$, revert to the prior ceiling outside it, and block unconstrained optimization at the support edge. Inside **Regime II** ([Chapter 43](43-inquiry-openness-multiverse.md)), an institution with intact vital slack may run reversible fractional minimax-regret probes ($a^\ast\_{\mathrm{MMR}}$) to map the edge **strictly inside interference-shielded, non-hysteretic pilots** so probing does not lock in irreversible Lucas expectation shifts. Under **Regime III** irreversibility, performative hysteresis, or a binary lethal-omission impasse ([Chapter 41](41-the-triad-and-the-minimum.md)), Gate 1 blocks unanchored action upfront instead of discovering the cliff after impact.

## 7.7 The Arithmetic of the Loop, and Where It Stops Closing

### 7.7.1 Two limits of the fixed point

The closed forms of §7.3.1 establish exact limits on when retraining stays close to performatively optimal policy inside our one-dimensional quadratic mean-shift model.

First, hold reactivity fixed inside the contracting interior $|\varepsilon| < 1$ (so $1 - \varepsilon > 0$ and $\varepsilon \ne 1$) and let regularization vanish ($\lambda \to 0^+$). Both $\theta\_{\mathrm{PS}}$ and $\theta\_{\mathrm{PO}}$ converge smoothly to $\mu\_0/(1 - \varepsilon)$: without shrinkage cost, retraining finds the Stackelberg optimum. Outside $|\varepsilon| < 1$, shrinking $\lambda \to 0^+$ crosses the stability wall $|\varepsilon| = 1 + \lambda$, so RRM loses convergence even if algebraic $\theta\_{\mathrm{PS}}$ stays finite. Moreover, under non-quadratic losses with non-zero penalty, performative stability and optimality still diverge.

Second, fix externally pre-registered $\lambda > 0$ and raise sensitivity $\varepsilon$ from unit shift ($\varepsilon = 1$, where outcome means move one-for-one with deployed rule $\theta$) toward the resonance pole $\varepsilon = 1 + \lambda$ (with $\mu\_0 \ne 0$). At $\varepsilon = 1$, the true optimum sits at $\theta\_{\mathrm{PO}} = 0$ while RRM settles at $\theta\_{\mathrm{PS}} = \mu\_0/\lambda$. As $\varepsilon \to 1 + \lambda$, $\theta\_{\mathrm{PO}}$ stays finite at $-\mu\_0/(1 + \lambda)$, whereas $|\theta\_{\mathrm{PS}}| \to \infty$ as contraction modulus $|\varepsilon|/(1 + \lambda) \uparrow 1$.

Two clean audit rules follow. First, contraction modulus $\kappa\_{\mathrm{RRM}} = |\varepsilon|/(1 + \lambda)$ is a reactivity-to-regularization ratio, while signed gap $\theta\_{\mathrm{PS}} - \theta\_{\mathrm{PO}} = \frac{\varepsilon\lambda\mu\_0}{(1+\lambda-\varepsilon)((1-\varepsilon)^2+\lambda)}$ combines $\varepsilon$, $\lambda$, and the resonance denominator $1 + \lambda - \varepsilon$—vanishing at both $\lambda \to 0^+$ ($|\varepsilon| < 1$) and $\lambda \to \infty$. Second, reporting $\varepsilon$ without externally pre-registered regularization $\lambda$ hides the stability denominator, while seeing a tiny gap $|\theta\_{\mathrm{PS}} - \theta\_{\mathrm{PO}}|$ never proves zero reactivity when heavy regularization $\lambda$ squeezes both policies near zero.

### 7.7.2 When the audit becomes the target

Standard anti-Goodhart advice measures proxy divergence, adds an outside audit score, and re-selects on that new benchmark. The 1990s Texas Assessment of Academic Skills (TAAS) cycle reveals why treating an audit check as a new operational target fails. High-stakes state test scores climbed steadily, yet an audit comparing them against the low-stakes National Assessment of Educational Progress (NAEP) found far smaller national gains. That gap matched narrow teaching toward the high-stakes test, prompting Stephen P. Klein et al. (2000) to call for independent low-stakes audits rather than trusting high-stakes series alone. As §7.3.3 predicts, turning an audit into the next binding incentive shifts gaming straight onto the audit.

When cosmetic gaming ($x\_{\mathrm{game}}$) costs less per point than causal improvement ($x\_{\mathrm{real}}$), escaping Goodhart drift needs both low-powered incentives (Holmström & Milgrom, 1991; [Chapter 40](40-institution-incentive-metric.md)) and strict separation of audit from target. Public accountability still demands that scoring rules, cutoffs, and frame-eligibility principles remain transparent and contestable ([Chapter 40](40-institution-incentive-metric.md); [Chapter 49](49-moral-luck-apt-agency-institutions.md)). What stays sealed until ex-post unsealing are only the specific probe draws sampled under that public protocol—paired externally with **non-disclosed multi-list capture–recapture (`T3`, [Chapter 45](45-world-to-archive-big-data-paradox.md))** reaching outside the internal sampling universe so operators cannot execute defeat-gaming rules uniformly on public criteria or dump harm onto *Not askable* strata.

Even sealed audits obey a strict **pre-registration limit**: cryptographic commitments guarantee integrity only **from write time ($t\_{\mathrm{disp}}$) onward**, leaving pre-seal omissions, framing bias, and spillover across community reception perimeters ($\Theta\_{\mathrm{basin}}$) unchecked ([Chapter 43](43-inquiry-openness-multiverse.md); [Chapter 49](49-moral-luck-apt-agency-institutions.md) §8.1.2). Reliable seals therefore depend upstream on the Premise Rigging Index ($\mathrm{PRI}$, [Chapter 43](43-inquiry-openness-multiverse.md)) and an unblocked frontline alarm channel before $t\_{\mathrm{disp}}$ locks ([Appendix A](appendix-a-mathematics.md) §A.7).

### 7.7.3 When the loop has no fixed point

Repeated retraining converges only under contractivity—sufficiently when $|\varepsilon| < \gamma / \beta\_{\mathrm{joint}}$ and exactly when $|\varepsilon| < 1 + \lambda$ in our linear-quadratic model. Once reactivity breaks past that threshold, iterative retraining diverges and chases its own feedback across shifting targets. That instability occurs at pole $\varepsilon = 1 + \lambda$ ($\mu\_0 \ne 0$), in explosive $|\varepsilon| > 1 + \lambda$ loops, or when firms continuously reclassify liabilities outside targeted aggregates as seen in British monetary targeting (Goodhart, 1975; Goodhart, 1984).

Offline Gate 1 therefore monitors empirical sensitivity $\varepsilon$ and contraction modulus under externally locked $\lambda$ between cycles, pairing those between-cycle gates with **real-time intra-cycle volatility and subgroup-drift circuit-breakers** in Gate 2 ([Chapter 49](49-moral-luck-apt-agency-institutions.md)). When contractivity or intra-cycle bounds fail, retraining stops and two disciplined institutional moves replace it (remembering under Reflexive Brake 4 that the co-diagnostic pair $\mathcal{D}\_{\mathrm{diag}} = \bigl(M\_{\mathrm{Leontief}}, \Theta\_{\mathrm{basin}}\bigr)$ folds strictly **ex post** over finished runs and never gates live operations):

1. **Subordinated Rule Freeze:** Freeze the rule across a pre-registered pause to observe unchased behavior—licensed **only** while physical margins hold ($\mathrm{TTS}\_{\mathrm{survival}} \ge \mathrm{TTR}\_{\mathrm{eff}}$, [Chapter 47](47-irreversible-time-vital-margin.md)) and **externally attested per-step non-export standing (`T3`/`T7`)** verifies that harms are not being offloaded onto outsiders (audited retrospectively afterward via $\Theta\_{\mathrm{basin}}$). That pause requires strict liability parity ($q \ge c$, [Chapter 40](40-institution-incentive-metric.md)), an automatic non-self-extending **sunset** blocking perpetual freezes (Reflexive Brake 3, [Chapter 50](50-methodological-families-reflexivity.md)), a prohibition on consecutive freezes without independent external ratification and mandatory cooldown, and attested reopening (`T1`, [Chapter 43](43-inquiry-openness-multiverse.md)) if sudden hazards strike.
2. **Objective Re-anchoring:** Whenever vital margins cannot wait out a freeze or reclassification persists, replace the gamed proxy target with an **externally verifiable physical guardrail** outside agents' unilateral control—paired strictly with **low-powered incentives** (Holmström & Milgrom, 1991) and $q \ge c$ liability parity so the re-anchored physical variable serves as an audit boundary rather than a high-powered KPI that restarts Goodhart gaming.

## Bridge

This chapter builds **Gate 1 ($g\_{\text{ex-ante}, t}$)** of **Canonical Operator III**, the offline institutional audit blocking naive curve-fitting before action runs. It combines tie-pooled CORP calibration and resolution, causal invariance under ICP, out-of-support Kennedy–O'Hagan discrepancy margins, Tian–Pearl bounds, and performative contractivity and Goodhart ceilings. Yet ex-ante auditing leaves real-time warnings and retrospective accountability unprotected.

[Moral Luck, Apt Agency, and Institutional Design](49-moral-luck-apt-agency-institutions.md) finishes Operator III by adding **Gate 2 ($g\_{\mathrm{kairos}, t}$)** and **Gate 3 ($g\_{\text{ex-post}, t}$)**. Gate 2 provides the undiscounted real-time alarm channel protecting frontline testimony during execution. Gate 3 adds the single-opening bitemporal vault separating Sosa's apt agency from moral luck after impact. Together they prepare [Chapter 50](50-methodological-families-reflexivity.md), which folds all three gates into $S\_{\mathrm{gov}}$ beside $\Theta\_{\mathrm{basin}}$ under reflexivity audits.
