# Model, Fit, and Error

<figure class="stoch-figure stoch-logo-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="660" height="132" viewBox="0 0 660 132" fill="none" role="img" aria-label="Stochology CH37 · DISCREPANCY — Model, Fit, And Extrapolation Error">
  <defs>
    <linearGradient id="ch37-ring" x1="2" y1="78" x2="386" y2="78" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="38%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="68%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <radialGradient id="ch37-glow" cx="50%" cy="50%" r="50%">
      <stop offset="0%" stop-color="#38bdf8" stop-opacity="0.25"/>
      <stop offset="50%" stop-color="#2563eb" stop-opacity="0.14"/>
      <stop offset="82%" stop-color="#fbbf24" stop-opacity="0.08"/>
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
    <path d="M 68 58 L 106 28 L 106 88 Z" fill="var(--logo-crimson, #e11d48)" fill-opacity="0.22"/>
    <path d="M 22 34 C 36 82, 54 82, 68 58 C 80 42, 94 34, 106 28" stroke="var(--logo-blue, #2563eb)" stroke-width="2.8" stroke-linecap="round"/>
    <path d="M 68 58 C 82 68, 94 80, 106 88" stroke="var(--logo-crimson, #e11d48)" stroke-width="2.4" stroke-dasharray="3.5 2.5" stroke-linecap="round"/>
    <circle cx="48.00" cy="74.00" r="6.55" fill="url(#ch37-node-well)" stroke="url(#ch37-ring-vital)" stroke-width="2.42"/>
    <circle cx="48.00" cy="74.00" r="2.81" fill="url(#ch37-core-vital)"/>
    <g>
      <circle cx="68.00" cy="58.00" r="6.66" fill="var(--logo-node-bg, #e2e8f0)"/>
      <circle cx="68.00" cy="58.00" r="5.76" fill="url(#ch37-pupil)" stroke="url(#ch37-gold-mira)" stroke-width="2.16"/>
      <line x1="59.36" y1="58.00" x2="64.22" y2="58.00" stroke="url(#ch37-gold-mira)" stroke-width="2.16" stroke-linecap="round"/>
      <line x1="71.78" y1="58.00" x2="76.64" y2="58.00" stroke="url(#ch37-gold-mira)" stroke-width="2.16" stroke-linecap="round"/>
      <line x1="68.00" y1="49.36" x2="68.00" y2="54.22" stroke="url(#ch37-gold-mira)" stroke-width="2.16" stroke-linecap="round"/>
      <line x1="68.00" y1="61.78" x2="68.00" y2="66.64" stroke="url(#ch37-gold-mira)" stroke-width="2.16" stroke-linecap="round"/>
      <circle cx="68.00" cy="58.00" r="1.98" fill="url(#ch37-gold-core)"/>
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

> Foundation chapter of Part III. It presupposes [Chapter 09](09-sets-relations-functions.md)
> and [Chapter 14](14-linear-algebra.md). Its formal content is worked out in
> [Chapter 46](46-non-identical-asymmetry-heavy-tails.md) and
> [Chapter 48](48-reactive-worlds-performativity-goodhart.md).

## By the end of this chapter you can

1. Decompose the error of a fitted model into the part that shrinks with data and the part that
   does not.
2. Recognise information leakage through the order in which data were handled.
3. Say what a validation scheme silently assumes about the future, and when that assumption is
   false.

## 1. Two errors: approximation and estimation

The prediction error of a fitted model splits into two parts, and only one of them responds to sample size.

- **Approximation error** (structural bias): The distance between the true physical process and the best possible
  model residing within the functional hypothesis class $\mathcal{F}$ you chose. It does not shrink as you
  collect more rows; it shrinks only when you expand or alter the mathematical family itself.
- **Estimation error** (sample variance): The distance between the specific model fitted on a finite training
  sample and the optimal model in that family. It shrinks toward zero as sample size $n \to \infty$, but expands
  rapidly as the complexity or dimension of the family increases.

This decomposition is the rigorous foundation of the bias-variance trade-off. Increasing model flexibility
improves in-sample fit and reaches a threshold beyond which out-of-sample generalization degrades, so the model
that describes the archive with the highest fidelity is rarely the one that governs operations best.

Picture a coastal reinsurance underwriter six hours before a tropical cyclone makes landfall. Her desk must post a binding reserve commitment from a regression model fitted on years of satellite track transitions. When the storm rapidly intensifies across a warm eddy that her linear feature set never represented, collecting another fifty years of calm-water tracks will not rescue the balance sheet. She has collided with the distinction between what a functional family can express (George Box's warning that all models are structural approximations) and what a finite sample can pin down across Leo Breiman's divide between generative data models and algorithmic predictors.

For a target $Y = f^\star(X) + \varepsilon$ with zero-mean irreducible noise $\mathrm{Var}(\varepsilon) = \sigma\_\varepsilon^2$, best in-class population projection $f\_{\mathcal{F}}^\star = \arg\min\_{f \in \mathcal{F}} \mathbb{E}[(f^\star(X) - f(X))^2]$, and finite-sample estimate $\hat{f}\_n \in \mathcal{F}$, expected squared prediction risk decomposes orthogonally as:

$$\mathbb{E}\bigl[(Y - \hat{f}\_n(X))^2\bigr] = \underbrace{\mathbb{E}\bigl[(f^\star(X) - f\_{\mathcal{F}}^\star(X))^2\bigr]}\_{\text{Approximation Error } B\_{\mathcal{F}}^2} + \underbrace{\mathbb{E}\bigl[(f\_{\mathcal{F}}^\star(X) - \hat{f}\_n(X))^2\bigr]}\_{\text{Estimation Error } V\_n(\mathcal{F})} + \sigma\_\varepsilon^2.$$

Two limits bracket the identity. Send $n \to \infty$ at fixed model dimension $d$ and the estimation variance vanishes ($V\_n(\mathcal{F}) \asymp \sigma\_\varepsilon^2 d / n \to 0$), while risk floors at $B\_{\mathcal{F}}^2 + \sigma\_\varepsilon^2$. However much data the archive holds, total squared error never falls below the structural bias plus the environmental noise. A mismatched family and a flood of rows still leave an error the rows cannot touch.

Reverse the trade at $d = n$ with $B\_{\mathcal{F}}^2 = 0$: structural bias disappears on paper and estimation variance expands to $V\_n(\mathcal{F}) = \sigma\_\varepsilon^2 d / n = \sigma\_\varepsilon^2$, which doubles out-of-sample error. Across competing data-cleaning and functional specifications, Sara Steegen's multiverse analysis (Steegen, Tuerlinckx, Gelman & Vanpaemel 2016) shows that defensible processing choices alone can move a published result across the significance line, so the choice among plausible model classes $\mathcal{F}\_1, \dots, \mathcal{F}\_K$ is a source of variation that the sampling term $V\_n(\mathcal{F})$ of any one of them does not report.

## 2. Four leakage pathways

Information leaks from the chronological future of an analysis into its past. Every channel inflates the validation metrics and guarantees operational failure on deployment.

1. **Target leakage through a feature**: A predictor variable incorporates information that is causally downstream
   of the outcome. For example, using "assigned ICU bed type" to predict patient sepsis mortality when the ICU
   transfer was ordered after septic shock occurred. Instead of forecasting the future, the algorithm reads
   the outcome disguised as an input.
2. **Temporal leakage**: Data collected after the operational moment of decision is incorporated into model
   training. The algorithm is fitted on information that will not exist when frontline operators are forced to act.
3. **Group/Cluster leakage**: Clustered entities (the same patient across multiple visits, the same machine tool,
   or the same geographic municipality) are randomly partitioned across training and validation splits. The model
   memorizes individual entity signatures rather than learning structural generalities.
4. **Preprocessing leakage**: Global scaling, quantile normalization, imputation, or feature selection is executed
   across the entire dataset before splitting into train and test folds. Information from the holdout validation
   distribution silently contaminates the training parameters.

Data leakage has one signature: the validation metrics look perfect. No loss function reveals it. What reveals it is an audit of chronological provenance. *What did each computational step know, and at what physical timestamp did it learn it?*

The revision record of real-time macroeconomic vintages that Croushore & Stark (2001) document makes the first mechanics concrete: replacing the initial real-time GDP release available at decision quarter $t$ with the final benchmark-revised column published years later injects future tax-return and census benchmarks into the past, fabricating out-of-sample precision that no policymaker had on the night of the vote. Temporal clustering does the same with physical records: six-hourly cyclone track observations cluster inside named storms, so randomly shuffling the rows across train and test folds scatters consecutive snapshots of the same hurricane into both sets. The model memorizes the central pressure trajectory of each named storm rather than generalizing to unseen cyclones; only storm-ID block partitioning or strict chronological cutoffs preserve valid out-of-sample evaluation.

## 3. Goodhart's law and performative prediction

When a model is deployed to govern institutional decisions, it ceases to be a passive observer of an exogenous
world. In 2020, Juan Perdomo et al. formalized **performative prediction**: deploying a model parameter $\theta$
actively perturbs the data-generating mechanism, shifting the distribution of inputs and targets:

$$(X, Y) \sim \mathcal{D}(\theta).$$

Optimizing against the historical distribution $\mathcal{D}(\theta\_0)$ leads to performative collapse: the act
of scoring individuals changes their behavior, shifting the distribution away from the manifold on which the
model was trained.

In sociology, Donald MacKenzie (2006) demonstrated that economic models exhibit two forms of performativity:
*Barnesian performativity*, where adopting a model makes reality conform more closely to its assumptions, and
*counter-performativity*, where adopting a model creates incentives that destroy the model's own preconditions
(e.g., portfolio insurance rules that sell as the market falls, the case MacKenzie (2006) reads as
counter-performative).

Performative feedback produces four variants of **Goodhart's Law**.

1. **Regressional**: Extremes of a noisy metric reflect temporary luck; selecting upon them produces inevitable
   regression to the mean.
2. **Extremal**: Pushing a proxy metric to extreme values breaks the historical correlation between proxy and goal.
3. **Causal**: Acting on a non-causal correlate fails to move the intended outcome.
4. **Adversarial**: Strategic agents deliberately game and manipulate the metric to maximize private reward.

Formally, a performative audit evaluates the feedback sensitivity $\varepsilon$ of the world to the deployed parameter $\theta$ and measures the performative gap $|\theta\_{\mathrm{PS}} - \theta\_{\mathrm{PO}}|$ between the **performatively stable** fixed point $\theta\_{\mathrm{PS}}$ (where repeated myopic retraining converges when the institution treats the shifting archive as if it were exogenous) and the **Stackelberg performative optimum** $\theta\_{\mathrm{PO}}$ (which explicitly accounts for how $\theta$ moves $\mathcal{D}(\theta)$ before choosing $\theta$) (Perdomo, Zrnic, Mendler-Dünner & Hardt 2020; MacKenzie 2006). Myopic retraining lands on the stable point at a higher performative risk than the optimal one; accounting for strategic response lowers both the parameter and the risk, and the gap between the two is the measure of what the institution paid for pretending the world was exogenous. When feedback sensitivity is negligible the model acts as a passive camera; when feedback molds behaviour toward the formula and the gap opens, the regime is the Barnesian self-fulfilling one; and when feedback amplifies systemic risk and sensitivity turns strongly negative, the collapse is counterperformative and destructive. When a credit bureau or hospital triage score becomes the rule that allocates capital or beds, the historical correlation between proxy and outcome is sheared apart across the four modes of Goodhart's law (Manheim & Garrabrant 2018).

## 4. Kennedy-O'Hagan model discrepancy and isotonic calibration

Even under rigorous chronological validation, every model remains a mathematical simplification of an open world.
Following Marc Kennedy and Anthony O'Hagan (2001), the true physical response $y(x)$ must be formulated with an
explicit **structural model discrepancy** $\delta(x)$:

$$y(x) = f(x, \theta) + \delta(x) + \epsilon.$$

The discrepancy $\delta(x)$ is not random Gaussian white noise; it is a systematic, spatially or temporally
correlated deficiency. Attempting to tune parameters $\theta$ to absorb $\delta(x)$ produces severe parameter
confounding. An institution must estimate $\delta(x)$ directly through non-parametric methods, and declare where
its bounds remain uncontrolled.

Information geometry unifies model error, convexity, and calibrated probability forecasting (Gneiting & Raftery 2007; Čencov 1972). Any
strictly convex loss potential $\phi$ induces a **Bregman divergence**:

$$D\_{\phi}(x \parallel y) = \phi(x) - \phi(y) - \langle\nabla\phi(y), x - y\rangle \ge 0.$$

When raw predicted probabilities are projected onto the convex cone of non-decreasing calibrated probabilities
via **Pool Adjacent Violators (PAV)** isotonic regression, the
generalised Pythagorean theorem partitions total Brier loss cleanly into three orthogonal components:

$$\text{Brier Loss} = \text{Miscalibration (REL)} - \text{Resolution (RES)} + \text{Uncertainty (UNC)}.$$

This decomposition allows an audit to verify whether a model's error stems from miscalibration (fixable via
isotonic recalibration), low informational resolution, or irreducible environmental stochasticity.

Given binary outcomes $y\_i \in \{0, 1\}$ with base rate $\bar{y} = \frac{1}{N}\sum\_{i=1}^N y\_i$ and raw probability forecasts $p\_i \in [0, 1]$, the isotonic projection computes the non-decreasing projection $\pi\_i = \mathrm{PAV}(p\_i)$ (Dimitriadis, Gneiting & Jordan 2021). The **Consistent, Optimal, Reproducible, PAV-based (CORP)** Murphy partition (Murphy 1973) defines three non-negative quantities:

$$\mathrm{REL}\_{\mathrm{CORP}} = \mathrm{BS}(p) - \mathrm{BS}(\pi) = \frac{1}{N}\sum\_{i=1}^N (p\_i - \pi\_i)^2 + \frac{2}{N}\sum\_{i=1}^N (p\_i - \pi\_i)(\pi\_i - y\_i) \ge 0, \quad \mathrm{RES}\_{\mathrm{CORP}} = \frac{1}{N}\sum\_{i=1}^N (\pi\_i - \bar{y})^2, \quad \mathrm{UNC}\_0 = \bar{y}(1 - \bar{y}),$$

which satisfy $\mathrm{BS}(p) = \mathrm{REL}\_{\mathrm{CORP}} - \mathrm{RES}\_{\mathrm{CORP}} + \mathrm{UNC}\_0$ identically by construction (and integrate across thresholds into the continuous ranked probability score (Hersbach 2000)). Two limit cases bracket every forecast:

1. **Zero-resolution climatology limit ($\mathrm{RES}\_{\mathrm{CORP}} = 0$):** A forecaster who issues a constant probability $p\_i \equiv p\_0$ yields $\pi\_i \equiv \bar{y}$, so $\mathrm{RES}\_{\mathrm{CORP}} = 0$ and $\mathrm{BS} = (p\_0 - \bar{y})^2 + \bar{y}(1-\bar{y})$. When $p\_0 = \bar{y}$, $\mathrm{REL}\_{\mathrm{CORP}} = 0$ as well: the forecast is perfectly calibrated ($\mathrm{BS} = \mathrm{UNC}\_0$) and completely uninformative.
2. **Perfect-calibration limit ($\mathrm{REL}\_{\mathrm{CORP}} = 0$):** When raw forecasts already match their isotonic conditional frequencies ($p\_i = \pi\_i$), miscalibration vanishes and $\mathrm{BS} = \mathrm{UNC}\_0 - \mathrm{RES}\_{\mathrm{CORP}}$, so further improvement requires sharper physical predictors to raise $\mathrm{RES}\_{\mathrm{CORP}}$ rather than post-hoc recalibration.

Applied to a cyclone-intensification archive, the partition separates the three terms cleanly: reliability carries a tiny share of the score, resolution carries the bulk of it, and base-rate uncertainty supplies the floor. The Kennedy–O'Hagan discrepancy term behaves as its theory requires, small in-sample where the family was tuned and large out-of-sample where the unseen intensification regime lives, and the gap between those two magnitudes is itself the diagnosis (Kennedy & O'Hagan 2001). Because PAV fits a flexible step function on finite samples, the reliability term carries a finite-sample optimism that split-half cross-validation measures and removes, so the debiased reliability is what an audit should report.

## 5. The tabular zoo, read through approximation and estimation error

Nothing in §1–§4 depends on the hypothesis class, which is why the standard families can be placed on the same
two axes rather than memorised as a list.

- **Additive trees and their boosted variants.** A single decision tree is a piecewise-constant approximation with
  high variance; a random forest averages many deep trees to cut that variance, and extremely randomized trees (Extra Trees; Geurts, Ernst & Wehenkel 2006) randomize the split thresholds further to push variance down at the cost of a bias the
  averaging cannot remove; gradient boosting instead fits shallow trees sequentially to the residual, trading
  variance for bias one small step at a time, whether through second-order Newton-step tree regularisation (XGBoost; Chen & Guestrin 2016), gradient-based one-side sampling and histogram bundling (LightGBM; Ke et al. 2017), or ordered boosting against target-statistic leakage on categorical features (CatBoost; Prokhorenkova et al. 2018). Bagging attacks estimation error, boosting attacks approximation error, and the hyper-parameters that matter for each are opposite.
- **Kernels and support-vector machines.** A soft-margin support-vector machine (Cortes & Vapnik 1995) pairs a maximum-margin hyperplane with a kernel that implicitly maps the inputs into a
  feature space where a linear separator suffices, the change of basis whose absence made exclusive disjunction
  unlearnable ([Chapter 14](14-linear-algebra.md) §6). The regularisation parameter is a capacity control; the
  kernel bandwidth is a statement about the scale on which the target is assumed to vary. Choosing it from data is
  estimation, declaring it is a modelling assumption, and the two have different failure modes.
- **Dimensionality reduction.** Linear orthogonal projections (PCA; Hotelling 1933) and neighbourhood-graph manifold embeddings (UMAP; McInnes, Healy & Melville 2018) compress the input before the model sees it. They are
  legitimate when the target is a function of the retained subspace, and destructive when it is a function
  of the annihilated one, the kernel of the projection, in the sense of rank-nullity.
- **Hyper-parameter search.** Random search, Bayesian optimisation and successive halving differ in how they spend a
  fixed evaluation budget, not in what they guarantee. Each produces a *selection* estimate, optimistic by construction because the winner was chosen on the same data it was scored on. That is the same
  optimism the split-half PAV debiasing removes for calibration, and it must be removed here or the reported
  score is an in-sample artifact.

The invariant across the zoo is the one this chapter has insisted on: the model class constrains *what* can be
approximated, the sample constrains *how well* it can be estimated, and no amount of tuning converts the first into
the second. A pipeline that reports a single validation number, computed after selecting the class and the
hyper-parameters on the same archive, has reported the maximum of a noisy quantity, which is precisely the
regressional Goodhart mechanism of [Chapter 48](48-reactive-worlds-performativity-goodhart.md). That audit is
enforced ex ante by pairing Kennedy–O'Hagan structural discrepancy against estimation variance and split-half PAV
optimism, and by pricing the Jensen convexity slack that point-mean tabular fits erase under heavy tails, which is
what prevents the point-forecast undersizing that carries a naive optimizer to ruin at the first adverse step.

### What a larger archive would buy

Only a small share of this Brier score is miscalibration, and recalibration by isotonic pooling already returns what share there is. What a larger archive would shrink is the finite-sample optimism, the smallest of the three terms; the base-rate uncertainty it would leave untouched. A model is not fixed by feeding it.

The decomposition is the diagnostic, not the score. Resolution carries the bulk of the total and the uncertainty term the rest, so the number that a reader should look at is the ratio of resolution to miscalibration, not the score itself. A constant forecast illustrates the floor: its reliability and resolution are both zero and its score equals the uncertainty of the base rate, which is a calibrated and useless model.

The fourth quantity is leakage. Normalising before splitting the sample improves validation and worsens deployment, because the split no longer represents the data the model will meet. That failure is invisible in every aggregate: the validation score improves, and the mechanism that produced the improvement is the defect.

### When more data is enough

More data suffices only when the structural discrepancy is zero and the leakage is zero; otherwise the missing quantity is not sample size but the right family of models. The procedure is a discrepancy test against the estimation error, if the discrepancy dominates, change the family, and do not buy rows. The test is cheap and the alternative is expensive: a larger archive buys precision on a misspecified quantity, which is the one purchase no budget can justify.

The alternative is a family change, and a family change is cheap.

## Bridge

Validation assumptions and the convexity slack of unobserved dispersion are formalised in
[Chapter 46](46-non-identical-asymmetry-heavy-tails.md) through the Jensen gap of a convex loss under a heavy
tail; post-hoc probability calibration via
pool-adjacent-violators isotonic regression is developed there as convex projection; and structural model discrepancy alongside performative
Goodhart audits is formalised in
[Chapter 48](48-reactive-worlds-performativity-goodhart.md) through the ex ante gate. A model fitted to a record here is fitted without regard to the order of its observations, and the temporal clustering that order carries is [Chapter 38](38-time-and-dependence.md), *Time and Dependence*.
