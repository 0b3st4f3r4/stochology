# Model, Fit, and Error

<figure class="stoch-figure stoch-logo-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="800" height="132" viewBox="0 0 800 132" fill="none" role="img" aria-label="Stochology CH37 · DISCREPANCY — Model, Fit, And Extrapolation Error">
  <defs>
    <linearGradient id="ch37-ring" x1="2" y1="78" x2="386" y2="78" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="38%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="68%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <radialGradient id="ch37-glow" cx="50%" cy="50%" r="50%">
      <stop offset="0%" stop-color="#38bdf8" stop-opacity="0.24"/>
      <stop offset="48%" stop-color="#2563eb" stop-opacity="0.14"/>
      <stop offset="80%" stop-color="#e11d48" stop-opacity="0.08"/>
      <stop offset="100%" stop-color="#090e1a" stop-opacity="0"/>
    </radialGradient>
    <linearGradient id="ch37-triad-stroke" x1="18" y1="24" x2="110" y2="104" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.85"/>
      <stop offset="50%" stop-color="var(--logo-gold, #f59e0b)" stop-opacity="0.90"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.85"/>
    </linearGradient>
    <linearGradient id="ch37-fill" x1="22" y1="30" x2="106" y2="98" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)" stop-opacity="0.26"/>
      <stop offset="48%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.28"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.26"/>
    </linearGradient>
    <linearGradient id="ch37-ink-curve" x1="20" y1="66" x2="108" y2="66" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-ink, #0f172a)"/>
      <stop offset="50%" stop-color="var(--logo-sub, #1d4ed8)"/>
      <stop offset="100%" stop-color="var(--logo-ink, #0f172a)"/>
    </linearGradient>
    <radialGradient id="ch37-pupil" cx="45%" cy="42%" r="58%">
      <stop offset="0%" stop-color="#020617"/>
      <stop offset="68%" stop-color="#0f172a"/>
      <stop offset="100%" stop-color="#451a03"/>
    </radialGradient>
    <linearGradient id="ch37-gold-mira" x1="54" y1="48" x2="84" y2="82" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#fef08a"/>
      <stop offset="45%" stop-color="#fbbf24"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <radialGradient id="ch37-gold-core" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#ffffff"/>
      <stop offset="35%" stop-color="#fde047"/>
      <stop offset="100%" stop-color="#d97706"/>
    </radialGradient>
    <radialGradient id="ch37-node-well" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="var(--logo-well-hi, var(--elevated-bg, #ffffff))"/>
      <stop offset="100%" stop-color="var(--logo-node-bg, #e2e8f0)"/>
    </radialGradient>
    <linearGradient id="ch37-ring-epi" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#7dd3fc"/>
      <stop offset="50%" stop-color="#2563eb"/>
      <stop offset="100%" stop-color="#1e3a8a"/>
    </linearGradient>
    <radialGradient id="ch37-core-epi" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#bae6fd"/>
      <stop offset="45%" stop-color="#3b82f6"/>
      <stop offset="100%" stop-color="#1d4ed8"/>
    </radialGradient>
    <linearGradient id="ch37-ring-vital" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#6ee7b7"/>
      <stop offset="50%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#065f46"/>
    </linearGradient>
    <radialGradient id="ch37-core-vital" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#a7f3d0"/>
      <stop offset="45%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#047857"/>
    </radialGradient>
    <linearGradient id="ch37-ring-gov" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#fda4af"/>
      <stop offset="50%" stop-color="#e11d48"/>
      <stop offset="100%" stop-color="#881337"/>
    </linearGradient>
    <radialGradient id="ch37-core-gov" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#fecdd3"/>
      <stop offset="45%" stop-color="#f43f5e"/>
      <stop offset="100%" stop-color="#be123c"/>
    </radialGradient>
  </defs>
  <g>
    <circle cx="64" cy="66" r="48" fill="url(#ch37-glow)"/>
    <polygon points="68,58 106,28 106,88" fill="url(#ch37-fill)"/>
    <path d="M 22 34 C 36 82, 54 82, 68 58 C 80 42, 94 34, 106 28" stroke="var(--logo-blue, #2563eb)" stroke-width="2.8" stroke-linecap="round"/>
    <path d="M 68 58 C 82 68, 94 80, 106 88" stroke="var(--logo-crimson, #e11d48)" stroke-width="2.4" stroke-dasharray="3.5 2.5" stroke-linecap="round"/>
    <circle cx="22" cy="34" r="6.6" fill="url(#ch37-node-well)" stroke="url(#ch37-ring-epi)" stroke-width="2.4"/>
    <circle cx="22" cy="34" r="2.8" fill="url(#ch37-core-epi)"/>
    <circle cx="48" cy="74" r="6.6" fill="url(#ch37-node-well)" stroke="url(#ch37-ring-vital)" stroke-width="2.4"/>
    <circle cx="48" cy="74" r="2.8" fill="url(#ch37-core-vital)"/>
    <g>
      <circle cx="68" cy="58" r="7.0" fill="var(--logo-node-bg, #e2e8f0)"/>
      <circle cx="68" cy="58" r="6.0" fill="url(#ch37-pupil)" stroke="url(#ch37-gold-mira)" stroke-width="2.2"/>
      <line x1="59.0" y1="58" x2="64.0" y2="58" stroke="url(#ch37-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
      <line x1="72.0" y1="58" x2="77.0" y2="58" stroke="url(#ch37-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
      <line x1="68" y1="49.0" x2="68" y2="54.0" stroke="url(#ch37-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
      <line x1="68" y1="62.0" x2="68" y2="67.0" stroke="url(#ch37-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
      <circle cx="68" cy="58" r="2.0" fill="url(#ch37-gold-core)"/>
    </g>
  </g>
  <g transform="translate(148, 0)">
    <text x="2" y="27" fill="var(--logo-sub, #1d4ed8)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.2">PART III · FOUNDATIONS OF JUDGMENT</text>
    <text x="498" y="27" text-anchor="end" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.0">CH37 · DISCREPANCY</text>
    <text x="0" y="59" fill="var(--logo-ink, #0f172a)" font-family="system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif" font-size="18.5" font-weight="800" letter-spacing="0.95">MODEL, FIT, AND EXTRAPOLATION ERROR</text>
    <line x1="2" y1="78" x2="384" y2="78" stroke="url(#ch37-ring)" stroke-width="1.6" stroke-opacity="0.82"/>
    <circle cx="400" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="415" cy="78" r="6.2" stroke="var(--logo-gold, #f59e0b)" stroke-width="1.5" stroke-dasharray="2.2 1.6"/>
    <circle cx="415" cy="78" r="4.0" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.2"/>
    <circle cx="415" cy="78" r="1.8" fill="var(--logo-gold, #f59e0b)"/>
    <circle cx="430" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="445" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-green, #10b981)" stroke-width="2.0"/>
    <circle cx="445" cy="78" r="1.6" fill="var(--logo-green, #10b981)"/>
    <circle cx="460" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="475" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-crimson, #e11d48)" stroke-width="2.0"/>
    <circle cx="475" cy="78" r="1.6" fill="var(--logo-crimson, #e11d48)"/>
    <circle cx="490" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <text x="2" y="104" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.8" font-weight="600" letter-spacing="0.65">OVERFITTING PENALTY · KENNEDY–O'HAGAN STRUCTURAL DEFICIT · REGIME SHIFT</text>
  </g>
  </svg>
</figure>

## By the end of this chapter you can

1. Decompose the error of a fitted model into the part that shrinks with data and the part that
   does not.
2. Recognise information leakage through the order in which data were handled.
3. Say what a validation scheme silently assumes about the future, and when that assumption is
   false.

## 1. Two errors: approximation and estimation

Picture a coastal reinsurance underwriter six hours before a tropical cyclone makes landfall. Her desk must post a reserve commitment using a regression model fitted on years of satellite storm tracks. When the storm rapidly intensifies over a warm eddy missing from her linear feature set, collecting fifty more years of calm-water tracks will not save the balance sheet.

This failure exposes a fundamental split between what a model class $\mathcal{F}$ can represent and what finite data can pin down. In [Chapter 14](14-linear-algebra.md) and [Chapter 15](15-probability-as-a-measure.md), projections restricted choices to a declared subspace; here, George Box's (1976, 1979) reminder that every model simplifies reality meets Leo Breiman's (2001) contrast between data models and algorithmic predictors. Prediction error breaks cleanly into two parts, and only one responds to sample size:

- **Approximation error (structural bias):** the gap between reality and the best possible rule inside model class $\mathcal{F}$. More rows never shrink it; only changing $\mathcal{F}$ moves it.
- **Estimation error (fitting bias and variance):** the extra error between our finite-sample fit and the best rule in $\mathcal{F}$. Under consistent estimation in a finite-capacity class, it vanishes as $n \to \infty$, yet climbs quickly with model complexity.

This trade-off drives the bias–variance curve: flexible families cut in-sample error until out-of-sample performance degrades.

To make that split exact, consider an unseen test draw $(X, Y)$ independent of training sample $D\_n$, where $Y = f^\star(X) + \varepsilon$ with $\mathbb{E}[\varepsilon \mid X] = 0$ and $\mathrm{Var}(\varepsilon) = \sigma\_\varepsilon^2$. Pick a model $\hat{f}\_n \in \mathcal{F}$ and let $f\_{\mathcal{F}}^\star \in \arg\min\_{f \in \mathcal{F}} \mathbb{E}[(f^\star(X) - f(X))^2]$ be the best population rule inside $\mathcal{F}$. Averaging over $D\_n$ and $(X, Y)$, excess squared risk splits into two legible terms (Bottou & Bousquet 2008):

$$\mathbb{E}\bigl[(Y - \hat{f}\_n(X))^2\bigr] - \sigma\_\varepsilon^2 = \underbrace{\mathbb{E}\bigl[(f^\star(X) - f\_{\mathcal{F}}^\star(X))^2\bigr]}\_{\text{Approximation Risk } B\_{\mathcal{F}}^2} + \underbrace{\mathbb{E}\bigl[(f^\star(X) - \hat{f}\_n(X))^2 - (f^\star(X) - f\_{\mathcal{F}}^\star(X))^2\bigr]}\_{\text{Excess Estimation Risk } \mathcal{E}\_{\mathrm{est}}(\mathcal{F})}.$$

Read aloud, total error above irreducible noise $\sigma\_\varepsilon^2$ equals structural bias $B\_{\mathcal{F}}^2$ plus finite-sample estimation risk $\mathcal{E}\_{\mathrm{est}}(\mathcal{F})$. Expanding around $f\_{\mathcal{F}}^\star$ yields $\mathcal{E}\_{\mathrm{est}}(\mathcal{F}) = V\_n(\mathcal{F}) + 2\mkern2mu\mathbb{E}[(f^\star - f\_{\mathcal{F}}^\star)(f\_{\mathcal{F}}^\star - \hat{f}\_n)]$ with $V\_n(\mathcal{F}) := \mathbb{E}[(f\_{\mathcal{F}}^\star(X) - \hat{f}\_n(X))^2]$. The cross-term stays $\ge 0$ on convex classes, can shift sign on trees or neural nets, and vanishes identically on closed linear subspaces ($f^\star - f\_{\mathcal{F}}^\star \perp \mathcal{F}$). Splitting instead around expected fit $\bar{f}\_n(X) = \mathbb{E}\_{D\_n}[\hat{f}\_n(X)]$ recovers the standard bias–variance formula $\mathbb{E}[(Y - \hat{f}\_n(X))^2] = \mathbb{E}[(f^\star(X) - \bar{f}\_n(X))^2] + \mathbb{E}[(\bar{f}\_n(X) - \hat{f}\_n(X))^2] + \sigma\_\varepsilon^2$. Whenever finite-budget optimization produces $\tilde{f}\_n \in \mathcal{F}$ short of exact empirical optimum $\hat{f}\_n$, a third **optimization error** $\mathcal{E}\_{\mathrm{opt}}(\mathcal{F}) = \mathbb{E}[(f^\star - \tilde{f}\_n)^2 - (f^\star - \hat{f}\_n)^2]$ completes Bottou and Bousquet's (2008) tripartite decomposition: while empirical risk suboptimality $\hat{R}\_n(\tilde{f}\_n) - \hat{R}\_n(\hat{f}\_n) \ge 0$ and combined excess risk $\mathcal{E}\_{\mathrm{est}}(\mathcal{F}) + \mathcal{E}\_{\mathrm{opt}}(\mathcal{F}) = \mathbb{E}[(f^\star - \tilde{f}\_n)^2 - (f^\star - f\_{\mathcal{F}}^\star)^2] \ge 0$ remain non-negative, population $\mathcal{E}\_{\mathrm{opt}}(\mathcal{F})$ itself can be negative when incomplete optimization acts as implicit early-stopping regularization.

Notice what happens as complexity $d$ approaches sample size $n$. On a low-dimensional linear subspace ($d \ll n$), sending $n \to \infty$ shrinks $\mathcal{E}\_{\mathrm{est}}(\mathcal{F}) = V\_n(\mathcal{F}) \to 0$ at $O(d/n)$—scaling as $\sigma\_\varepsilon^2 d / n$ under fixed covariates or well-specified models ($B\_{\mathcal{F}}^2 = 0$), whereas misspecified random-design OLS ($B\_{\mathcal{F}}^2 > 0$) scales with the sandwich variance of total pseudo-true residuals $Y - f\_{\mathcal{F}}^\star(X)$ (of order $(B\_{\mathcal{F}}^2 + \sigma\_\varepsilon^2)d/n$ under regular moments)—leaving total risk floored at $B\_{\mathcal{F}}^2 + \sigma\_\varepsilon^2$. Yet pushing $d$ near $n$ behaves differently in-sample versus out-of-sample. At fixed training covariates ($B\_{\mathcal{F}}^2 = 0$), projection excess error $\sigma\_\varepsilon^2 d/n$ adds to irreducible noise $\sigma\_\varepsilon^2$, raising total fresh-$Y$ prediction MSE $\sigma\_\varepsilon^2(1 + d/n)$ to $2\sigma\_\varepsilon^2$ when $P\_X = I\_n$ interpolates training noise. Under **random-design** Gaussian prediction on *fresh* test inputs ($d \ge 1$, $n > d + 1$, well-specified zero-mean predictors with invertible covariance $\Sigma \succ 0$, homoscedastic noise), exact excess error equals $\sigma\_\varepsilon^2 d / (n - d - 1)$, blowing up to $+\infty$ as $d \to n - 1$. Across candidate classes $\mathcal{F}\_1, \dots, \mathcal{F}\_K$, multiverse analysis (Steegen, Tuerlinckx, Gelman & Vanpaemel 2016) reveals model-selection spread hidden by any single $\mathcal{E}\_{\mathrm{est}}(\mathcal{F})$.

## 2. Four leakage pathways

Information leaks from an analysis's future back into its training past through four distinct paths, inflating validation scores while guaranteeing deployment collapse:

1. **Target leakage through a feature:** an input contains facts causally downstream of the outcome—such as using "assigned ICU bed type" to predict sepsis death when transfer happened after septic shock began.
2. **Temporal leakage:** observations recorded *after* the moment of decision enter training, giving models knowledge frontline staff do not possess.
3. **Group/Cluster leakage:** repeated records from one person, machine, or town get scattered randomly across train and validation splits, letting models memorize unit identities instead of patterns.
4. **Preprocessing leakage:** scaling, quantile normalization, imputation, or feature selection runs across the whole table before splitting, smuggling test-set statistics into training.

Leakage shares one clear symptom: validation metrics look eerily strong while deployment fails. No loss function spots it; only tracing time stamps and record lineage catches what each step knew and when.

Real-time macroeconomic vintages make temporal leakage concrete (Croushore & Stark 2001): substituting benchmark-revised GDP figures published years later in place of the initial quarter-$t$ release feeds future definitions backward. Similarly, six-hourly cyclone track records cluster inside named storms; randomly shuffling rows across train and test splits leaks the storm's central pressure trajectory into validation unless folds block by storm ID or time.

## 3. Goodhart's law and performative prediction

Once a deployed model steers decisions, it stops reading an independent world passively. Publishing parameter $\theta$ shifts the data distribution $\mathcal{D}(\theta)$ itself—what Juan Perdomo, Tijana Zrnic, Celestine Mendler-Dünner, and Moritz Hardt (2020) formalize as **performative prediction**. That shift spans self-fulfilling *Barnesian* conformity and destabilizing *counter-performativity* (MacKenzie 2006) across the four variants of **Goodhart's Law** (Manheim & Garrabrant 2018; detailed in [Chapter 40](40-institution-incentive-metric.md)).

Here lies the boundary between standard statistics and institutional dynamics: exogenous drift responds to better out-of-sample splits, whereas endogenous feedback loops require checking reactivity contraction bounds ([Chapter 15](15-probability-as-a-measure.md))—otherwise repeated retraining diverges or locks into inferior equilibria that push strain onto unconsenting external reception perimeters ([Chapter 40](40-institution-incentive-metric.md); [Chapter 48](48-reactive-worlds-performativity-goodhart.md)).

## 4. Kennedy-O'Hagan model discrepancy and isotonic calibration

Even with leak-free chronological splits, every physical or social model leaves something out. Following Marc Kennedy and Anthony O'Hagan (2001; introduced in [Chapter 32](32-the-question-before-the-data.md)), write true physical response $y(x)$ as simulator prediction $\eta(x, \theta)$ plus systematic **structural discrepancy** $\delta(x)$ and observation noise $\epsilon$:

$$y(x) = \eta(x, \theta) + \delta(x) + \epsilon.$$

In plain words, $\delta(x)$ is not harmless white noise; it is correlated structural error where the simulator omits physics. Tuning parameters $\theta$ to absorb $\delta(x)$ ruins interpretability and fails outside observed support, so discrepancy must be modeled directly and bounded honestly.

Information geometry connects invariant statistical-manifold structure (Čencov 1972/1982) and convexity ([Chapter 12](12-derivatives-and-convexity.md)) with strictly proper scoring rules for calibrated forecasting (Gneiting & Raftery 2007). Any strictly convex potential $\phi$ on a convex domain $\Omega$ that is differentiable on $\operatorname{int}(\Omega)$ defines a **Bregman divergence** (Bregman 1967) for $x \in \Omega$ and $y \in \operatorname{int}(\Omega)$:

$$D\_{\phi}(x \parallel y) = \phi(x) - \phi(y) - \langle\nabla\phi(y), x - y\rangle \ge 0,$$

measuring how far linear tangent approximations fall below the convex surface (for example, $\phi(p) = p^2$ on $[0, 1]$ generates squared Brier error $(p - y)^2$).

How do we tell whether a probabilistic forecast is badly calibrated or just uninformative? Regressing binary outcomes onto non-decreasing probabilities via **Pool Adjacent Violators (PAV)** isotonic regression turns raw forecasts $p\_i \in [0, 1]$ into calibrated probabilities $\pi\_i$, partitioning Brier loss cleanly into miscalibration, resolution, and uncertainty:

$$\text{Brier Loss} = \text{Miscalibration (REL)} - \text{Resolution (RES)} + \text{Uncertainty (UNC)}.$$

Specifically, for forecast probabilities $p\_1, \dots, p\_N \in [0, 1]$ and binary outcomes $y\_1, \dots, y\_N \in \lbrace 0, 1\rbrace$ ($N \ge 1$) with base rate $\bar{y} = \frac{1}{N}\sum\_{i=1}^N y\_i$ and Brier score $\mathrm{BS}(p) = \frac{1}{N}\sum\_{i=1}^N (p\_i - y\_i)^2$, tie-pooled PAV orders outcomes by $p\_i$, averages ties $p\_i = p\_j$, and iteratively merges adjacent violating blocks ($\bar{y}\_{\mathrm{left}} > \bar{y}\_{\mathrm{right}}$) until calibrated probabilities $\pi\_i = \hat{\pi}(p\_i) \in [0, 1]$ form a unique monotone non-decreasing function of $p\_i$—leaving residuals $y - \pi$ orthogonal to centered calibrated fits $\pi - \bar{y}\mathbf{1}$ (Dimitriadis, Gneiting & Jordan 2021). Extending Murphy's (1973) binned vector partition, that **CORP** decomposition (writing $\mathrm{REL}\_{\mathrm{CORP}}$ and $\mathrm{RES}\_{\mathrm{CORP}}$ for Dimitriadis, Gneiting & Jordan's $\mathrm{MCB}$ and $\mathrm{DSC}$) defines three non-negative terms:

$$\mathrm{REL}\_{\mathrm{CORP}} = \mathrm{BS}(p) - \mathrm{BS}(\pi) = \frac{1}{N}\sum\_{i=1}^N (p\_i - \pi\_i)^2 + \frac{2}{N}\sum\_{i=1}^N (p\_i - \pi\_i)(\pi\_i - y\_i) \ge 0, \quad \mathrm{RES}\_{\mathrm{CORP}} = \frac{1}{N}\sum\_{i=1}^N (\pi\_i - \bar{y})^2 \ge 0, \quad \mathrm{UNC}\_0 = \bar{y}(1 - \bar{y}) \ge 0,$$

giving $\mathrm{BS}(p) = \mathrm{REL}\_{\mathrm{CORP}} - \mathrm{RES}\_{\mathrm{CORP}} + \mathrm{UNC}\_0$ identically—where the cross-term $\frac{2}{N}\sum\_{i=1}^N (p\_i - \pi\_i)(\pi\_i - y\_i) \ge 0$ arises from piecewise-constant PAV orthogonality and causes $\mathrm{REL}\_{\mathrm{CORP}}$ to exceed $\frac{1}{N}\sum\_{i=1}^N (p\_i - \pi\_i)^2$ whenever $\pi$ merges adjacent violating blocks beyond mere tie-pooling ([Appendix A](appendix-a-mathematics.md) §A.3 #16). Independently of Murphy/PAV binning choice, integrating thresholded Brier scores across cutoffs yields the continuous ranked probability score (Hersbach 2000). Two clear limits interpret every forecast:

1. **Zero-resolution climatology ($\mathrm{RES}\_{\mathrm{CORP}} = 0$):** Predicting constant probability $p\_i \equiv \bar{y}$ yields zero miscalibration ($\mathrm{REL}\_{\mathrm{CORP}} = 0$) yet zero resolution ($\mathrm{BS} = \mathrm{UNC}\_0$)—perfectly calibrated and useless.
2. **Perfect calibration ($\mathrm{REL}\_{\mathrm{CORP}} = 0$):** When raw forecasts match isotonic calibrated probabilities ($p\_i = \pi\_i$), $\mathrm{BS} = \mathrm{UNC}\_0 - \mathrm{RES}\_{\mathrm{CORP}}$, so further gain requires sharper predictors rather than recalibration.

Across domains, comparing small in-sample $\delta(x)$ against exploding out-of-sample $\delta(x)$ signals regime shift immediately (Kennedy & O'Hagan 2001). Because PAV fits $\pi$ to $y$ in-sample, $\mathrm{BS}(\pi)$ runs optimistically low and inflates apparent $\mathrm{RES}\_{\mathrm{CORP}}$ and $\mathrm{REL}\_{\mathrm{CORP}}$; split-half cross-validation or an independent calibration fold must verify PAV out-of-fold.

## 5. The tabular zoo, read through approximation and estimation error

The exact orthogonality of §1 holds only for linear spaces, yet approximation risk, estimation variance, leakage, performativity, and calibration govern every modern machine-learning family:

- **Trees, random forests, and boosting:** Single decision trees suffer high estimation variance. Random forests average deep trees—and Extra Trees randomize split points further—to cut estimation variance at modest bias (Geurts, Ernst & Wehenkel 2006). Gradient boosting instead adds shallow trees sequentially onto errors to cut approximation bias step by step, using second-order leaf regularization (XGBoost; Chen & Guestrin 2016), histogram binning, gradient-based one-side sampling, and exclusive feature bundling (LightGBM; Ke et al. 2017), or ordered target encoding against prediction shift (CatBoost; Prokhorenkova et al. 2018). Bagging fights variance; boosting fights bias.
- **Kernels and support-vector machines:** Soft-margin SVMs (Cortes & Vapnik 1995) pair maximum margins with nonlinear feature lifts ([Chapter 14](14-linear-algebra.md) §6). Regularization tunes capacity; kernel width sets assumed smoothness scale.
- **Dimensionality reduction:** Linear projections (PCA; Hotelling 1933) erase anything inside their kernel $\ker(P)$ ([Chapter 14](14-linear-algebra.md) §2), while nonlinear neighbor graphs (UMAP; McInnes, Healy & Melville 2018) distort global scales; both break if the target depends on discarded directions.
- **Hyper-parameter search:** Random search, Bayesian optimization, and successive halving all suffer selection optimism if scored on the same split—a winner's curse distinct from PAV overfitting in §4 that requires nested cross-validation or a locked test split.

Model family fixes approximation reach while sample size fixes estimation noise; hyperparameter tuning never converts one into the other. Picking winning architectures on a reused validation fold commits **regressional Goodhart** optimism (Manheim & Garrabrant 2018; distinct from strategic gaming in [Chapter 40](40-institution-incentive-metric.md)). Algorithmic fits excel on clean i.i.d. tasks (Breiman 2001) yet fail under selection bias, vintage leaks (Croushore & Stark 2001), performativity $\mathcal{D}(\theta)$, or environmental shifts (*infidélité du milieu*, Canguilhem 1943/1966). Heavy model audits therefore run off-shift ([Chapter 29](29-aim-and-the-three-planes.md) §4; [Chapter 36](36-decision-loss-threshold.md) §5), while live screens enforce out-of-support and reactivity alarms rejecting ungrounded extrapolation.

### What a larger archive would buy

More rows shrink estimation variance and PAV overfitting (when evaluated out-of-fold), leaving base-rate shifts and structural discrepancy $\delta(x)$ intact. Read CORP diagnostics directly rather than raw Brier scores: strong calibrated models hold $\mathrm{REL}\_{\mathrm{CORP}}$ near zero while $\mathrm{RES}\_{\mathrm{CORP}}$ captures substantial $\mathrm{UNC}\_0$.

### When more data is enough

Adding rows suffices only when structural discrepancy $\delta(x) = 0$ across both training and deployment domains, deployment covariates lie inside training support, target noise carries finite variance ($\alpha > 2$ for squared-error $V\_n$), all key causal variables exist in the table, chronological leakage is zero, sampling selection is absent ($\rho\_{R,Y} = 0$ with known clustering), and behavior does not react to predictions. Under leakage, bigger models just memorize the leak faster; under omitted variables, extra rows sharpen a biased curve. Enlarging $\mathcal{F}$ requires matching sample growth to control $V\_n(\mathcal{F})$ and still cannot invent missing columns.

## Bridge

[Chapter 46](46-non-identical-asymmetry-heavy-tails.md) and [Chapter 48](48-reactive-worlds-performativity-goodhart.md) embed convexity slack, CORP isotonic calibration, structural discrepancy $\delta(x)$, and performative Goodhart checks into Part IV's operational filters. Yet static cross-sectional error splits miss irreversible time along a single path—non-ergodicity, absorbing ruin barriers, and queue survival clocks—which open next in [Chapter 38](38-time-and-dependence.md), *Time and Dependence*.
