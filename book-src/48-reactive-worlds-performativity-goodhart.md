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

1. Separate miscalibration from resolution on a forecast archive and say which of the two a retraining loop can move without improving the forecast.
2. Compute performative stability and performative optimality from the closed forms, and show that the gap is a ratio of reactivity to regularisation rather than a property of reactivity alone.
3. Recognise when a retraining loop has no fixed point, and say what an institution does instead of iterating.

An agent can construct an honest **Epistemic Margin** ($S\_{\mathrm{epi}}$, Chapters 43–45) and buffer its **Vital Margin** ($S\_{\mathrm{vital}}$, Chapters 46–47). Then it acts. And a world that is acted upon stops being a passive urn of marbles.

**Mediating Filter III** (**Axis 3 — $S\_{\mathrm{gov}}$: Institutional Governance**) governs the return path from the agent's model and decision back into the reactive social and institutional world. Its first constitutive tension (**Tension 6**) confronts **passive predictive curve-fitting and performative stability** against **isotonic calibration, causal invariance across environments, performative optimality, and Goodhart's Law** (Goodhart, 1975; Lucas, 1976; Strathern, 1997; Tian & Pearl, 2000; Kennedy & O'Hagan, 2001; MacKenzie, 2006; Espeland & Sauder, 2007; Hudgens & Halloran, 2008; Peters, Bühlmann & Meinshausen, 2016; Manheim & Garrabrant, 2018; Milli et al., 2019; Perdomo et al., 2020; Kleinberg & Raghavan, 2020; Dimitriadis, Gneiting & Jordan, 2021).

In the unified mathematical architecture of Stochology, Chapter 48 implements **Gate 1 (Ex-Ante Calibration, Causal Invariance & Performative Audit)** of **Canonical Operator III**, the per-step gate whose temporal infimum over a completed trajectory folds into the diagnostic coordinate $S\_{\mathrm{gov}}$:

$$\mathcal{O}\_{\mathrm{gov}}(\pi\_t, \mathcal{I}\_t) = \min\bigl(g\_{\text{ex-ante}, t}(\text{CORP}, \text{ICP}, \text{Perdomo}),\mkern5mu g\_{\mathrm{kairos}, t}(\text{List–Pettit}, \text{Fricker}, \text{Bateson}),\mkern5mu g\_{\text{ex-post}, t}(\text{Sosa}, \text{Holmström}, \text{Vault})\bigr)$$

![Causal Graphs, Interventions, and Confounding](figures/fig-03-causal-dags.svg)

---

## 7.1 Isotonic CORP-Murphy Brier Score Decomposition via PAV

Any probabilistic forecast $p\_1, \dots, p\_n \in (0, 1)$ deployed against binary outcomes $y\_1, \dots, y\_n \in \lbrace0, 1\rbrace$ raises two questions that must be kept apart. **Honest calibration**: do events predicted at a given probability happen that often? **Discriminative resolution**: can the model separate high-risk shifts from calm shifts?

The classical mean squared error of probabilistic forecasts is the **Brier Score** (Brier, 1950):
$$\mathrm{BS}(p) = \frac{1}{n}\sum\_{i=1}^n (p\_i - y\_i)^2$$
Allan H. Murphy (1973) proved that the Brier score partitions into calibration, resolution, and baseline uncertainty. Classical implementations of that partition cut the forecasts into user-selected histogram bins $B\_1, \dots, B\_K$, and the cut is arbitrary. Coarse bins hide local miscalibration. Fine bins carry extreme sampling variance. And the empirical event frequencies across bins often violate monotonicity.

Timo Dimitriadis, Tilmann Gneiting, and Alexander I. Jordan (2021) removed the choice with the **CORP** (*Consistent, Optimally Binned, Reproducible*) decomposition. CORP selects no bins at all. It computes the unique non-decreasing isotonic regression $\hat{\pi}\_1 \le \dots \le \hat{\pi}\_n$ of the sorted outcomes by the **Pool-Adjacent-Violators (PAV)** algorithm:
$$\min\_{\hat{\pi}\_1 \le \dots \le \hat{\pi}\_n} \sum\_{i=1}^n (\hat{\pi}\_i - y\_{(i)})^2$$

### 7.1.1 The Exact Algebraic Identity

Because $\hat{\pi}$ is the orthogonal least-squares projection of the sorted outcome vector onto the convex cone of non-decreasing sequences, the cross-term between the residuals $(y\_{(i)} - \hat{\pi}\_i)$ and the calibrated centered predictions $(\hat{\pi}\_i - \bar{y})$ vanishes identically over every pooled PAV constant block. What remains is an algebraic identity:

$$\mathrm{BS}(p) = \frac{1}{n}\sum\_{i=1}^n (p\_i - y\_i)^2 = \underbrace{\mathrm{REL}\_{\mathrm{CORP}}}\_{\text{Miscalibration Loss}} - \underbrace{\mathrm{RES}\_{\mathrm{CORP}}}\_{\text{Discriminative Power}} + \underbrace{\mathrm{UNC}\_0}\_{\bar{y}(1-\bar{y})}$$

where:

- $\mathrm{UNC}\_0 = \bar{y}(1 - \bar{y}) = \frac{1}{n}\sum\_{i=1}^n (y\_i - \bar{y})^2$ is the baseline uncertainty of the climatological mean $\bar{y} = \frac{1}{n}\sum y\_i$;
- $\mathrm{RES}\_{\mathrm{CORP}} = \frac{1}{n}\sum\_{i=1}^n (\hat{\pi}\_i - \bar{y})^2 \ge 0$ measures the sharpness with which the model separates distinct outcome regimes;
- $\mathrm{BS}(\hat{\pi}) = \frac{1}{n}\sum\_{i=1}^n (\hat{\pi}\_i - y\_{(i)})^2 = \mathrm{UNC}\_0 - \mathrm{RES}\_{\mathrm{CORP}}$ is the residual loss achieved by an optimally calibrated monotone model;
- $\mathrm{REL}\_{\mathrm{CORP}} = \mathrm{BS}(p) - \mathrm{BS}(\hat{\pi}) \ge 0$ is the resolvable calibration deficit (miscalibration loss).

The decomposition closes, because the three components are an orthogonal split of one sum of squares and not an approximation to it.

### 7.1.2 Finite-Sample PAV Optimism and Split-Half Cross-Validation Debiasing

The PAV algorithm fits the monotone envelope $\hat{\pi}$ to the observed outcomes in-sample, so $\hat{\pi}$ absorbs random sample fluctuations and returns an optimistic calibration estimate (in-sample $\mathrm{REL}$ runs slightly too small).

The operator therefore estimates the finite-sample PAV optimism by interleaved split-half cross-validation. Sort observations by $(p\_i, i)$, partition into even and odd indices, and evaluate each fold's isotonic fit against the held-out fold:
$$\mathrm{REL}\_{\mathrm{CORP}}^{\mathrm{debiased}} = \max\bigl(0,\mkern3mu \mathrm{REL}\_{\mathrm{CORP}} - \hat{b}\_{\mathrm{PAV}}^{\mathrm{CV}}\bigr)$$
This debiasing provides a rigorous finite-sample guarantee against overfitting.

### 7.1.3 Continuous Ranked Probability Score (CRPS) Decomposition

Hersbach (2000) demonstrated that the Continuous Ranked Probability Score (CRPS) of a predictive cumulative distribution $F$ against realized outcome $y \in \mathbb{R}$ can be expressed as an integral of Brier scores across all thresholds $\tau \in \mathbb{R}$:
$$\mathrm{CRPS}(F, y) = \int\_{-\infty}^\infty \mathrm{BS}\bigl(F(\tau),\mkern3mu \mathbf{1}\lbrace{}y \le \tau\rbrace\bigr)\mkern3mu d\tau$$
Stochology applies the CORP-PAV operator to each binary slice along a threshold quadrature grid, so the integral of reliability, resolution, and uncertainty reconstructs the overall CRPS to quadrature precision. The gain over quoting a single CRPS is that the decomposition says *where* the loss lives: in the calibration of the forecast, in its inability to separate regimes, or in the base rate of the phenomenon, which no forecast can move.

---

## 7.2 Invariant Causal Prediction (ICP), Kennedy–O'Hagan Structural Discrepancy, and Tian–Pearl Bounds

A well-calibrated correlation still collapses when the agent intervenes (`do(X = x)`). Judea Pearl (2009) put the reason on the **Ladder of Causation**: passive conditioning $\mathbb{P}(Y \mid X = x)$ (Rung 1) confounds true causal mechanisms with transient environmental proxies. Three structural instruments belong to the ex-ante gate:

### 7.2.1 Invariant Causal Prediction (ICP) Across Heterogeneous Environments

Jonas Peters, Peter Bühlmann, and Nicolai Meinshausen (2016) formalized the principle of **Invariant Causal Prediction (ICP)**: if an agent observes the system across multiple distinct environments or regimes $e \in \mathcal{E}$ (historical epochs, geographical regions, seasonal shifts, or policy interventions), the conditional distribution of $Y$ given its true direct causal parents $\mathbf{PA}(Y)$ is invariant across all environments:
$$Y^{(e)} \mid X\_{\mathbf{PA}(Y)}^{(e)} \sim F\_Y \quad \forall e \in \mathcal{E}$$
Conversely, any regression model relying on a non-causal proxy $X\_{\mathrm{proxy}}$ will exhibit structural coefficient drift across environments as external conditions change.

The operator tests non-empty candidate predictor subsets $S \subseteq \lbrace0, \dots, d-1\rbrace$ ($S \ne \emptyset$; the empty set $S = \emptyset$ tests invariance of the marginal distribution of $Y^{(e)}$ directly across environments without regression degrees of freedom). For each non-empty subset $S$, it computes the pooled residual sum of squares $\mathrm{RSS}\_{\mathrm{restricted}}$ and the environment-specific residual sum of squares $\mathrm{RSS}\_{\mathrm{unrestricted}}$ on globally centered data (without per-environment intercepts), constructing the dimensionless Chow $F$-statistic:
$$F\_S = \frac{(\mathrm{RSS}\_{\mathrm{restricted}} - \mathrm{RSS}\_{\mathrm{unrestricted}}) / df\_1}{\mathrm{RSS}\_{\mathrm{unrestricted}} / df\_2}$$
where $df\_1 = (|\mathcal{E}| - 1)|S|$ and $df\_2 = n\_{\mathrm{total}} - |\mathcal{E}||S|$. A subset $S$ is accepted into the invariant set $\mathcal{A}$ if its invariance test passes ($F\_S \le F\_{1-\alpha}(df\_1, df\_2)$ for $|S| \ge 1$). The estimated causal parent set is the intersection:
$$\hat{S}^\ast = \bigcap\_{S \in \mathcal{A}} S$$
The intersection discards any variable that can be omitted without destroying invariance, and guarantees that $\mathbb{P}(\hat{S}^\ast \subseteq \mathbf{PA}(Y)) \ge 1 - \alpha$.

#### The Agnotological Low-Power Guard

When the effective sample per environment is small, the Chow test lacks power to reject candidate subsets, leading to the erroneous acceptance of the empty set $S = \emptyset$. Under intersection, this causes the estimated causal set to collapse silently to the empty set: $\hat{S}^\ast = \emptyset$. A naive optimizer reads the collapse as the finding that "no causal vulnerability exists". That is the agnotological move of Chapter 43 in a causal register. The gate treats a small per-environment sample as underpowered identification and buffers at the upper bound $U\_t$; it does not read the empty set as a finding.

### 7.2.2 Kennedy & O'Hagan's (2001) Anchored Gaussian Process Discrepancy

No mathematical model $f\_\theta(x)$ describes physical reality without a remainder. Marc C. Kennedy and Anthony O'Hagan (2001) modelled the true physical response $y(x)$ as
$$y(x) = f\_\theta(x) + \delta(x) + \varepsilon$$
where $\delta(x) \sim \mathcal{GP}(0, k(x, x'))$ is a structural discrepancy function modeled by a Gaussian Process with squared-exponential kernel:
$$k(x, x') = s^2 \exp\left(-\frac{(x - x')^2}{2\ell^2}\right)$$
The discrepancy GP is conditioned on empirical forecast residuals at a small number of anchor points:

- **In-Support Regime:** Inside the observed data domain, the posterior standard deviation $\sigma\_{\mathrm{post}}(x^\ast)$ shrinks toward observational noise, correcting structural bias.
- **Extrapolation Regime:** As the decision moves into untested operational regimes ($|x^\ast - x\_{\mathrm{train}}| \gg \ell$), the kernel correlation vector $\mathbf{k}\_\* \to \mathbf{0}$, and posterior uncertainty expands back to the unanchored prior ceiling $\sqrt{s^2 + \sigma\_{\mathrm{noise}}^2}$. The expansion prices extrapolation arrogance and forces the agent to hold reserves.

### 7.2.3 Jin Tian & Judea Pearl's (2000) Sharp Counterfactual Bounds

Attributing causal responsibility after an event requires Pearl's Rung 3: counterfactual probabilities. Combining experimental intervention rates ($p\_{y\_1} = \mathbb{P}(Y\_{x\_1}=1), p\_{y\_0} = \mathbb{P}(Y\_{x\_0}=1)$) with observational joint probabilities ($p\_{xy} = \mathbb{P}(X=x, Y=y)$), Jin Tian and Judea Pearl (2000) derived sharp bounds for:

- **Probability of Necessity ($\mathrm{PN}$):** Did taking action $X=1$ cause outcome $Y=1$?
  $$\max\left(0,\mkern3mu \frac{\mathbb{P}(Y=1) - \mathbb{P}(Y\_{x\_0}=1)}{\mathbb{P}(X=1, Y=1)}\right) \le \mathrm{PN} \le \min\left(1,\mkern3mu \frac{\mathbb{P}(Y\_{x\_0}=0) - \mathbb{P}(X=0, Y=0)}{\mathbb{P}(X=1, Y=1)}\right)$$
- **Probability of Sufficiency ($\mathrm{PS}$):** Would taking action $X=1$ have produced $Y=1$ in an untreated case?
  $$\max\left(0,\mkern3mu \frac{\mathbb{P}(Y\_{x\_1}=1) - \mathbb{P}(Y=1)}{\mathbb{P}(X=0, Y=0)}\right) \le \mathrm{PS} \le \min\left(1,\mkern3mu \frac{\mathbb{P}(Y\_{x\_1}=1) - \mathbb{P}(X=1, Y=1)}{\mathbb{P}(X=0, Y=0)}\right)$$
- **Probability of Necessity and Sufficiency ($\mathrm{PNS}$):** The probability that the outcome responds strictly to the intervention ($Y\_{x\_1}=1 \land Y\_{x\_0}=0$), linked to the conditional counterfactuals via $\mathrm{PNS} = p\_{11}\mkern2mu\mathrm{PN} + p\_{00}\mkern2mu\mathrm{PS}$ and bounded sharply with $p\_y = \mathbb{P}(Y=1) = p\_{11} + p\_{01}$ by (Tian & Pearl 2000):
  $$\max\bigl(0,\mkern3mu p\_{y\_1} - p\_{y\_0},\mkern3mu p\_y - p\_{y\_0},\mkern3mu p\_{y\_1} - p\_y\bigr) \le \mathrm{PNS} \le \min\bigl(p\_{y\_1},\mkern3mu 1 - p\_{y\_0},\mkern3mu p\_{11} + p\_{00},\mkern3mu p\_{y\_1} - p\_{y\_0} + p\_{10} + p\_{01}\bigr)$$

The bounds give distribution-free causal accountability, with no parametric form assumed.

---

## 7.3 Performative Prediction ($\theta\_{\mathrm{PS}}$ vs. $\theta\_{\mathrm{PO}}$), MacKenzie's Regimes, and Goodhart's Four Variants

Deploy an algorithm or a decision rule into a social or institutional environment and the agents inside it adapt to the rule. Juan Perdomo, Tijana Zrnic, Celestine Mendler-Dünner, and Moritz Hardt (2020) called this **performative prediction**.

### 7.3.1 Performative Stability vs. Stackelberg Performative Optimality

Let $\theta \in \Theta$ be the deployed model parameter. Deploying $\theta$ induces a distribution shift, so observations are drawn from $\mathcal{D}(\theta)$.

- The **Decoupled Loss** evaluates candidate parameter $\theta$ on the distribution induced by parameter $\theta'$:
  $$\mathcal{L}(\theta; \theta') = \mathbb{E}\_{Z \sim \mathcal{D}(\theta')}[\ell(Z; \theta)]$$
- **Repeated Risk Minimization (RRM):** Practitioners repeatedly retrain their models on recent historical data:
  $$\theta\_{t+1} = \arg\min\_\theta \mathcal{L}(\theta; \theta\_t)$$
- **Performative Stability ($\theta\_{\mathrm{PS}}$):** A fixed point of RRM satisfies:
  $$\theta\_{\mathrm{PS}} = \arg\min\_\theta \mathcal{L}(\theta; \theta\_{\mathrm{PS}})$$
  At $\theta\_{\mathrm{PS}}$ the model looks perfectly calibrated on the distribution its own deployment generated.
- **Performative Optimality ($\theta\_{\mathrm{PO}}$):** The true Stackelberg leader minimizes total performative risk, anticipating the distribution shift:
  $$\theta\_{\mathrm{PO}} = \arg\min\_\theta \mathrm{PR}(\theta) = \arg\min\_\theta \mathcal{L}(\theta; \theta)$$

Differentiate the performative risk and the failure of stability becomes arithmetic:
$$\nabla \mathrm{PR}(\theta) = \underbrace{\left.\nabla\_\theta \mathcal{L}(\theta; \theta')\right|\_{\theta'=\theta}}\_{\text{Standard Training Gradient}} + \underbrace{\left.\nabla\_{\theta'} \mathcal{L}(\theta; \theta')\right|\_{\theta'=\theta}}\_{\text{Performative Distribution Gradient}}$$
At $\theta\_{\mathrm{PS}}$ the standard training gradient vanishes by definition. The performative gradient does not, whenever reactivity is non-zero. So $\theta\_{\mathrm{PS}}$ is almost never performatively optimal.

Under linear response $Y \sim \mathcal{N}(\mu\_0 + \varepsilon \theta, \sigma^2)$ with $L\_2$ regularization $\lambda$:
$$\theta\_{\mathrm{PS}} = \frac{\mu\_0}{1 + \lambda - \varepsilon}, \qquad \theta\_{\mathrm{PO}} = \frac{(1 - \varepsilon)\mu\_0}{(1 - \varepsilon)^2 + \lambda}$$
In general, RRM is a Lipschitz contraction toward $\theta\_{\mathrm{PS}}$ whenever the Wasserstein-1 sensitivity $\varepsilon$ satisfies Perdomo et al.'s (2020) sufficient condition
$$\varepsilon < \frac{\gamma}{\beta},$$
where $\gamma$ is strong convexity of $\ell(z; \theta)$ in $\theta$ and $\beta$ is joint smoothness (the Lipschitz constant of $\nabla\_\theta \ell(z; \theta)$ with respect to the sample $z$). In the regularized linear-quadratic normal model above, $\gamma = 1 + \lambda$ and $\beta = |\partial\_y \partial\_\theta \ell| = 1$, giving the exact linear RRM map $\theta\_{t+1} = (\mu\_0 + \varepsilon\theta\_t)/(1 + \lambda)$, which contracts to $\theta\_{\mathrm{PS}}$ if and only if $|\varepsilon| < 1 + \lambda$.

### 7.3.2 Donald MacKenzie's (2006) Four Performativity Regimes

Donald MacKenzie, in *An Engine, Not a Camera* (2006), showed that economic models format the markets they describe, an organizing claim Michel Callon (1998) had already made for the sociology of market devices. Stochology sorts performativity into four regimes:

1. **Camera (passive):** Deploying the model has no measurable impact on the underlying data distribution ($\varepsilon \approx 0$).
2. **Effective performativity:** Deploying the rule reshapes the world in alignment with its objective, and the stable point sits close to the optimal one; social welfare improves.
3. **Barnesian self-fulfilment:** Retraining drives predictive residuals toward zero by moulding agents to conform to the formula — the bootstrapped induction in which a classification becomes true of what it classifies once the classified orient to it (Barnes 1983) — yet produces substantial deadweight loss relative to the optimal point, so the gap between the two is no longer negligible.
4. **Counterperformative destruction:** Deploying the rule triggers feedback that undermines its own operational premises, either by reversing the response or by raising realized risk far above the optimal point — dynamic hedging during the October 1987 stock market crash creating the post-crash volatility skew is the standard instance (MacKenzie 2006).

The regimes are ordered by the size of the performative gap. That gap is a ratio of reactivity to regularization, not a property of reactivity alone, and §7.7.1 does the arithmetic.

### 7.3.3 Strategic Effort vs. Cosmetic Gaming and Goodhart's Four Variants

When an algorithm allocates rewards based on proxy metric $M$, agents reallocate effort. Jon Kleinberg and Manish Raghavan (2020) and Smitha Milli et al. (2019) separate:

- **Causal Improvement Effort ($x\_{\mathrm{real}}$):** Investment that improves the substantive goal $G$;
- **Cosmetic Gaming ($x\_{\mathrm{game}}$):** Cheap manipulation of proxy $M$ that leaves $G$ unchanged or degraded.

David Manheim and Scott Garrabrant (2018) formalize the four mechanisms of **Goodhart's Law**:

1. **Regressional Goodhart:** Selecting based on a noisy proxy $M = G + \varepsilon$ selects for extreme noise $\varepsilon$. As the selection threshold increases, the expected goal conditional on the proxy falls further below the threshold.
2. **Extremal Goodhart:** Pushing $M$ into the extreme tail takes the system outside the historical manifold where $M$ and $G$ were positively correlated.
3. **Causal Goodhart:** Intervening on a downstream symptom $M$ ($G \to M$) severs the correlation without moving the causal parent $G$.
4. **Adversarial Goodhart:** Strategic agents actively optimize $M$ at low cost, collapsing the substantive metric. At the interpersonal scale of [Chapter 07](07-when-the-measure-becomes-the-goal.md), the same law has a micro-sociological precursor: when a speaker turns the listener's spontaneous reception (*skopós*) into a policed target of impression management, surveillance pressure erodes communicative trust just as proxy targeting erodes institutional metrics.

---

## 7.4 Three Mechanisms Worked Through

Three core mechanisms are traced below in symbols and not in substituted values, so that a reader can carry any archive through the same steps.

### 7.4.1 The CORP-PAV Decomposition

Sort the forecasts and the outcomes together by forecast probability, so that $y\_{(1)}, \dots, y\_{(n)}$ is the outcome sequence in forecast order. The isotonic fit is the non-decreasing sequence closest to that outcome sequence in mean square.

**Step 1: Pool the violators.** Start with every observation its own block with mean $y\_{(i)}$. Whenever two adjacent blocks are out of order — the earlier block's mean exceeds the later block's — merge them into one block whose value is the pooled mean, and continue until no adjacent pair violates monotonicity. The result is a partition into constant blocks with values $\hat{\pi}\_k$ and sizes $n\_k$. It is the unique minimizer of the constrained least-squares problem, which is why the decomposition below is an identity and not an approximation.

**Step 2: Read the three components off the blocks.** The forecast's Brier score is $\mathrm{BS}(p) = \frac{1}{n}\sum (p\_i - y\_i)^2$; the calibrated model's score is $\mathrm{BS}(\hat{\pi}) = \frac{1}{n}\sum\_{i=1}^n (\hat{\pi}\_i - y\_{(i)})^2 = \frac{1}{n}\sum\_k n\_k \hat{\pi}\_k(1 - \hat{\pi}\_k)$ across the pooled constant blocks (where $\hat{\pi}\_k = \bar{y}\_k$); the base-rate uncertainty is $\mathrm{UNC}\_0 = \bar{y}(1 - \bar{y})$; the resolution is the between-block variance $\mathrm{RES}\_{\mathrm{CORP}} = \frac{1}{n}\sum\_k n\_k(\hat{\pi}\_k - \bar{y})^2$; and the miscalibration loss is the difference $\mathrm{REL}\_{\mathrm{CORP}} = \mathrm{BS}(p) - \mathrm{BS}(\hat{\pi})$.

**Step 3: Check the closure.** Orthogonality of the projection makes the cross term vanish over every pooled block, so
$$\mathrm{REL}\_{\mathrm{CORP}} - \mathrm{RES}\_{\mathrm{CORP}} + \mathrm{UNC}\_0 = \mathrm{BS}(p)$$
holds. A reader who substitutes any forecast vector and any outcome vector into those five expressions can verify the identity. What cannot be substituted is the interpretation: reliability is the part of the loss a recalibration could remove, and resolution the part no recalibration can create.

### 7.4.2 Performative Stability against Performative Optimality

Take the linear response model of §7.3.1, with base state $\mu\_0$, reactivity $\varepsilon$, regularization $\lambda$ and residual variance $\sigma^2$.

**Step 1: Check the contractivity.** For the regularized quadratic loss $\ell(y; \theta) = \frac{1}{2}(y - \theta)^2 + \frac{\lambda}{2}\theta^2$, strong convexity in $\theta$ is $\gamma = 1 + \lambda$ whereas joint smoothness of $\partial\_\theta \ell(y; \theta) = (\theta - y) + \lambda\theta$ in the outcome $y$ is $\beta = 1$. The RRM update $\theta\_{t+1} = (\mu\_0 + \varepsilon\theta\_t)/(1 + \lambda)$ therefore has contraction modulus $|\varepsilon|\beta/\gamma = |\varepsilon|/(1 + \lambda)$ and converges to $\theta\_{\mathrm{PS}}$ if and only if $|\varepsilon| < 1 + \lambda$. At the resonance $\varepsilon = 1 + \lambda$ (for $\mu\_0 \ne 0$) no fixed point exists; for $|\varepsilon| > 1 + \lambda$, the algebraic fixed point $\theta\_{\mathrm{PS}} = \mu\_0/(1 + \lambda - \varepsilon)$ exists but is dynamically repelling, so RRM diverges away from it.

**Step 2: Solve the two points.** The stability point is $\theta\_{\mathrm{PS}} = \mu\_0/(1 + \lambda - \varepsilon)$: it solves the first-order condition of the decoupled loss evaluated at its own induced distribution. The optimal point is $\theta\_{\mathrm{PO}} = (1-\varepsilon)\mu\_0/((1-\varepsilon)^2 + \lambda)$: it solves the first-order condition of the performative risk, in which the response's dependence on the deployed parameter is differentiated rather than held fixed.

**Step 3: Compare the risks.** Evaluate $\mathrm{PR}(\theta) = \frac{1}{2}(\mu\_0 - (1-\varepsilon)\theta)^2 + \frac{1}{2}\sigma^2 + \frac{\lambda}{2}\theta^2$ at both points. The stable point carries a bias term that the optimal point trades against regularization, and the difference in risk is what the retraining loop spends to reach a fixed point. Let $\lambda \to 0$ at fixed $\varepsilon$ and the two points coincide: reactivity alone creates no gap. At $\varepsilon = 1$ the optimal point equals zero while the fixed point sits at $\mu\_0/\lambda$, and as $\varepsilon$ approaches the pole at $1 + \lambda$ the stability point diverges while the optimal point stays bounded at $-\mu\_0/(1 + \lambda)$, so the gap runs to infinity. The regime label of §7.3.2 is read off the size of that gap, which is a statement about the ratio and not about either term.

### 7.4.3 Invariant Causal Prediction on a Toy System

Consider predicting a target $Y$ from two candidates, $X\_1$ a true cause and $X\_2$ a spurious proxy, across two regimes.

- **Ground Truth:** $Y^{(e)} = \beta X\_1^{(e)} + \varepsilon$, with $\varepsilon$ independent of both candidates.
- **Environment 1 (calm regime):** the proxy is coupled to the cause, $X\_2^{(1)} = \kappa\_1 X\_1^{(1)} + \eta\_1$, so an observational regression of $Y$ on $X\_2$ estimates $\beta/\kappa\_1$ and appears to succeed.
- **Environment 2 (storm regime):** an external intervention shifts the coupling to $X\_2^{(2)} = \kappa\_2 X\_1^{(2)} + \eta\_2$ with $\kappa\_2 \ne \kappa\_1$, so the same regression estimates $\beta/\kappa\_2$.

Evaluating candidate subsets by the Chow statistic of §7.2.1:

1. **Candidate $S = \lbrace{}X\_1\rbrace$:** the slope is the same in both environments up to sampling error, the restricted and unrestricted residuals agree, and the subset is **accepted**.
2. **Candidate $S = \lbrace{}X\_2\rbrace$:** the slope moves by the ratio $\kappa\_1/\kappa\_2$, the Chow statistic is large, and the subset is **rejected**.
3. **Empty Set $S = \emptyset$:** the residual variance differs between regimes, so the empty subset is **rejected**.
4. **Candidate $S = \lbrace{}X\_1, X\_2\rbrace$:** conditioning on the true cause shields $Y$, so the conditional distribution is invariant and the subset is **accepted**.

Take the intersection: $\hat{S}^\ast = \lbrace{}X\_1\rbrace \cap \lbrace{}X\_1, X\_2\rbrace = \lbrace{}X\_1\rbrace$. The spurious proxy is eliminated and the true causal parent isolated. The exercise is worth running in symbols because it shows what the intersection buys, which is invariance and not fit, and why the empty set is the dangerous failure mode and not the conservative one.

---

## 7.5 The Gate on a Storm Archive

This chapter's gate is specified against a tropical-storm track record, of the kind a national hurricane centre publishes fix by fix. The forecast question is deliberately narrow: given the synoptic state at one fix, what is the probability that the next fix reaches hurricane-force wind? The question is well posed only over consecutive same-storm transitions at a fixed lead time. A longer window mixes the forecast problem with the interpolation problem.

That is where the gate's report begins. Fitted on one half of the transitions and evaluated on the other, the Brier score decomposes into a miscalibration loss, a resolution term and the base-rate uncertainty of the phenomenon (Murphy 1973). The proportions are the object, not the total. A forecast can earn a good score by resolution while being miscalibrated, or by calibration while resolving nothing, and only the first defect is one a recalibration can repair; a report quoting the score alone says nothing about which of the two a retraining cycle could still move, and the in-sample optimism of the reliability term is estimated by the split-half comparison of §7.1.2.

The causal instrument then tests a physical variable against a geographic one. Split the record into seasonal environments: a predictor that carries the mechanism, such as the pressure deficit of the storm, should keep its slope across them, while a proxy such as latitude, which is correlated with the mechanism through the climatology of one season, should not. The intersection of accepted subsets is the variable set that survives every environment (Peters, Bühlmann & Meinshausen 2016). Outside the anchored support the Gaussian-process discrepancy returns to its prior ceiling: the gate refuses to price an extrapolation. Counterfactual responsibility, when the operator is asked for it, comes from the Tian–Pearl bounds and not from a point attribution.

The performative reading is the honest one. The stability point and the optimal point of §7.4.2 differ when reactivity is positive and regularization finite, and the regime label follows from the size of the gap. The gaming reading is weaker and should be labelled as such: with a proxy gain and a substantive drop supplied as parameters, the collapse verdict holds for any series it is handed and would hold with no series at all. A reader should treat that reading as an illustration of the classifier's logic, not as evidence about an archive.

## 7.6 Optimising inside the dream: extremal Goodhart in latent space

§7.2 established that the Kennedy–O'Hagan discrepancy is a function of position: small where anchors exist,
returning to the prior ceiling outside them. §7.3 classified the four variants of Goodhart drift. Put the two
together and they give the sharpest failure mode of a model-based loop, which is the one an optimiser finds
*first*.

Let $\hat{M}$ be the surrogate and $\delta(z)$ the structural discrepancy at latent state $z$. A policy
optimised against $\hat{M}$ alone is not optimising the goal; it is optimising the goal *plus* the negative of
the discrepancy. Free to choose the trajectory, it selects the region where the gap between $\hat{M}$ and the
world is largest, where $\delta(z)$ is maximal and the surrogate pays least for it. The endpoint is a **Goodhart
extremal** point: the proxy sits at its extreme, and the correlation between proxy and goal, which held only inside
the anchored support, is gone. It is **adversarial** at the same time, because the search is not passive
sampling of a noisy proxy but an active hunt for the proxy's blind spot. The two readings describe one optimiser
action; they are not two degrees of one scale.

The composition is computable on a fixture that any reader can reconstruct. A linear surrogate is fitted by least squares on anchors confined to a support radius; the greedy policy is optimised inside it; the same actions are executed against a world that carries one absorbing constraint the surrogate never observed. The dreamer leaves the support by a distance measured in radii, spends most of its steps outside it, reaches a terminal discrepancy in the top decile of the attainable range, and destroys the world early, while its dreamed return stays several times its realised return. The classifier reports the adversarial variant, and nothing in the fixture supports the regressional one: the loss comes from selecting on *absence of data*, not from selecting on noise.

The Boltzmann temperature over the action distribution is the standard regulariser, and the audit separates what
it achieves from what it is claimed to achieve. Raise the temperature and the realised return rises, ruin recedes,
the terminal discrepancy falls. Temperature is a **distributional robustness** device. It removes the commitment
to the single action the surrogate over-rewards, so the trajectory never travels far enough to meet the unmodelled
constraint. It does not certify $\hat{M}$. The surviving arm survives by abstaining, and an abstaining policy cannot
tell a model that is true at the edge of its support from one that is merely unexplored there.

The institutional consequence follows. A temperature is a *policy-level* mitigation, and every policy-level
mitigation is invisible to the audit that matters, because the model still reports the same confidence
everywhere. Certification is an *evidence-level* act: the discrepancy must be estimated where anchors exist,
bounded where they do not, and the operating point refused when the trajectory leaves the anchored region. That is
the ex-ante gate of this chapter, and the reason it runs before actuation and not after the loss
is observed.

---

### Stability is not merit

A retrained model can sit at a fixed point that costs more performative risk than the optimum and moves further in parameter space, and it is the model the process will actually deliver. Out of sample the discrepancy standard deviation is larger than the in-sample one, and by how much depends on how far the stable point has travelled from the region where its validation was measured — a performative-risk term, because the distribution the score is measured against is the one the model's own deployment produces (Perdomo et al. 2020).

The gap is a price, paid in a currency the report does not track. A score that has converged is a score an optimiser can hit. Whether hitting it is good is a question about the world, and the score cannot answer it. The retraining loop answers it silently every quarter.

The decomposition separates the two. Of the Brier score, one part is miscalibration, a larger part is resolution, and the remainder is the base rate of the phenomenon. A model that improved its score by degrading resolution has improved a number and worsened a forecast.

### When reactivity is negligible

Reactivity is negligible when an individual decision does not move the distribution of the target. Measure the mass of decisions with an aggregate effect. Below a declared threshold, treat the model as non-performative and drop the loop. The rule measures feedback; it takes no stance about markets.

### The loop buys stability with validity

The retraining loop has a second cost the risk comparison hides. A model that has moved a long way in parameter space to reach a stable point has left the region where its in-sample validation was measured, and the discrepancy standard deviation then depends on the distance between the two regions rather than on the score (Perdomo et al. 2020). The loop buys stability at the price of validity, and reports the purchase as an improvement, because the only number it tracks is the score it was trained to hit. An honest report carries two numbers: the score the loop reached, and the distance it travelled to reach it.

## 7.7. The Arithmetic of the Loop, and Where It Stops Closing

### 7.7.1 Two limits of the fixed point

The closed forms of §7.3.1 say how far the claim that stability is almost never optimal can be pushed, and two limits mark the ends.

Let the regularisation vanish, $\lambda \to 0$, at fixed reactivity and base state. Then $\theta\_{\mathrm{PS}} = \mu\_0/(1 - \varepsilon)$ and $\theta\_{\mathrm{PO}} = (1-\varepsilon)\mu\_0/(1-\varepsilon)^2 = \mu\_0/(1 - \varepsilon)$: the two points coincide. Reactivity by itself creates no gap, and a reactive world with no shrinkage cost is one in which retraining to the fixed point is optimal. The gap in the worked example comes from $\lambda$ interacting with $\varepsilon$; its size is the interaction.

Now fix $\lambda > 0$ and let reactivity move past unit shift ($\varepsilon = 1$) toward the resonance pole at $1 + \lambda$. Already at $\varepsilon = 1$, where prediction shifts the mean one-for-one with the base state, $\theta\_{\mathrm{PO}} = (1-\varepsilon)\mu\_0/((1-\varepsilon)^2 + \lambda) = 0$ while the fixed point sits at $\theta\_{\mathrm{PS}} = \mu\_0/\lambda$; as $\varepsilon \to 1 + \lambda$, the stability point diverges as $\theta\_{\mathrm{PS}} = \mu\_0/(1 + \lambda - \varepsilon)$, whereas the optimal point stays finite at $\theta\_{\mathrm{PO}} \to -\mu\_0/(1 + \lambda)$ (and vanishes again only as $|\varepsilon| \to \infty$). The gap runs from nothing to unbounded as the contraction modulus $|\varepsilon|/(1 + \lambda) \uparrow 1$ approaches loss of contractivity right at the resonance pole $\varepsilon = 1 + \lambda$: at $\varepsilon = 1$ the optimal rule recommends $0$ while retraining settles on $\mu\_0/\lambda$, and approaching $1 + \lambda$ the fixed point runs away to infinity even though the optimal policy remains bounded.

Two consequences. The gap is a ratio of $\varepsilon$ to $\lambda$ and to the residual $1 + \lambda - \varepsilon$, so no measurement of $\varepsilon$ alone settles whether a loop is dangerous. And a small gap is evidence of a small ratio, which a large $\lambda$ produces as readily as an inert world does. Reporting $\varepsilon$ without $\lambda$ reports a numerator.

### 7.7.2 When the audit becomes the target

The remedy the gate implements, and the one the four Goodhart variants suggest, is to measure the discrepancy, add an independent check and re-anchor the metric on something the agent does not control. The Texas accountability system of the 1990s is the case in which the remedy fails on its own terms.

The state test was a high-stakes proxy with a published proficiency threshold, and its scores rose through the 1990s. The audit was available and it was taken. Performance on an independent, low-stakes, nationally administered instrument was compared with the state series, and the state gains exceeded the national ones, in the pattern that appears when the test itself becomes the object of instruction (Klein et al. 2000). What makes the episode a counterexample and not an illustration is the next step. Re-anchoring on the national instrument converts it into the next target, and the mechanism is the one §7.3.3 formalises: any measure the institution can observe, the agent can move, so a measure used for audit is a measure used for selection, and the audit signal degrades at the same rate as the proxy it was brought in to check.

The variant at stake is the adversarial case, and its arithmetic says why no audit design escapes it. If cosmetic effort $x\_{\mathrm{game}}$ costs less per unit of measured gain than causal effort $x\_{\mathrm{real}}$, the optimal response to any published measure is cosmetic, and an audit is a second published measure. What escapes the mechanism is evidence the agent cannot produce, sampled with an instrument it does not see and sealed before the incentive exists. That requirement is institutional and cryptographic, not statistical, and it is why the sealed commitment belongs to this operator and not to an appendix.

Even here, the cryptographic seal carries a strict **pre-registration limit**. A bitemporal SHA-256 / HMAC-SHA256 commitment proves chronological integrity and prevents ex-post alteration **from the moment the state is recorded** ($t\_{\mathrm{disp}}$), yet it cannot detect or block framing distortion, selective sampling, or premise manipulation carried out **before** the record is sealed. Because a hash or message authentication code authenticates whatever input it receives, the seal operates as a precondition of temporal integrity rather than a substitute for pre-registration scrutiny: its evidential value depends upstream on open framing in the inquiry transit—monitored by the Simon Premise Rigging Index ($\mathrm{PRI}$) of [Chapter 43](43-inquiry-openness-multiverse.md)—and on an unobstructed algedonic channel ([Chapter 49](49-moral-luck-apt-agency-institutions.md)) able to contest the premises before $t\_{\mathrm{disp}}$ is locked.

### 7.7.3 When the loop has no fixed point

The instruction is to measure the sensitivity and iterate to stability. It presupposes a contracting loop. §7.3.1 states the sufficient contraction condition $\varepsilon < \gamma/\beta$ (exact at $|\varepsilon| < 1 + \lambda$ in the linear-quadratic model). Violate that bound and the retraining loop ceases to contract: at the singular resonance $\varepsilon = 1 + \lambda$ (for $\mu\_0 \ne 0$) or in open reclassification games no fixed point exists at all, while for $|\varepsilon| > 1 + \lambda$ the algebraic fixed point becomes repelling so repeated risk minimisation diverges or cycles instead of finding it.

The condition is not exotic. $\gamma/\beta$ is a property of the loss and $\varepsilon$ a property of the world's response, so an institution whose measured quantity is a share of activity it can itself reclassify has a large $\varepsilon$ against a small $\gamma$, the reversed case. British monetary targeting is the dated instance: a broad aggregate was targeted, the banking system responded by creating liabilities outside it, the aggregate was redefined, and the response moved to the new definition, through the supplementary-deposit regime of the 1970s and its removal in 1980 (Goodhart 1984). Each round was a competent re-estimation of the same fixed point, and each moved the target rather than the world, until the target was abandoned.

When the condition fails, the procedure is not another iteration. Measure $\varepsilon$ across successive rounds, as the change in response per unit change in the deployed rule, and if the sequence is not contracting, stop treating the problem as estimation. Two moves remain. Freeze the rule for a pre-registered interval, so the response is observed without being chased. Or change the object optimised to one the institution cannot reclassify. Reflexivity is then not a modelling defect to be corrected. It is a statement that this loop has no equilibrium, and an institution that keeps iterating will report convergence to whatever it last deployed.

## Bridge

This chapter delivers a gate that is honest about reactivity and still incomplete. It evaluates the
decision before the outcome exists, and the harms it prices are the ones it could anticipate.

[Moral Luck, Apt Agency and Institutions](49-moral-luck-apt-agency-institutions.md) closes the temporal
circuit. Sosa's aptness separates the agent who aimed well from the agent who was lucky. The real-time and
ex-post gates add the channels that only exist during the event and the audit that only exists after it. The
algedonic channel is the brake that survives a discounted testimony, and a single-opening sealed commitment
binds the ex-post gate to a record it cannot renegotiate. What travels forward is a governance margin
computed over three times. The debt is aggregation: three filters now exist and nothing yet says how they
combine.
