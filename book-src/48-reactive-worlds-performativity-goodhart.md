# Chapter 48 — Reactive Worlds: Causal Invariance, Performativity, and Goodhart's Law

> Laboratory 7 of the `stochology` CLI: `--chapter 7`.

Once an agent has constructed an honest **Epistemic Margin** ($S_{\mathrm{epi}}$, Chapters 43–45) and buffered its **Vital Margin** ($S_{\mathrm{vital}}$, Chapters 46–47), it acts on the world. And once an agent acts on the world, the world does not remain a passive urn of marbles.

**Mediating Filter III** (**Axis 3 — $S_{\mathrm{gov}}$: Institutional Governance**) governs the return path from the agent's model and decision back into the reactive social and institutional world. Its first constitutive tension (**Tension 6**, implemented via [`GovernanceState::from_ex_ante_gate`](crates/stochology/src/governance.rs#L1391-L1583) in [`crates/stochology/src/governance.rs`](crates/stochology/src/governance.rs)) confronts **passive predictive curve-fitting and performative stability** against **isotonic calibration, causal invariance across environments, performative optimality, and Goodhart's Law** (Goodhart, 1975; Lucas, 1976; Strathern, 1997; Tian & Pearl, 2000; Kennedy & O'Hagan, 2001; MacKenzie, 2006; Espeland & Sauder, 2007; Hudgens & Halloran, 2008; Peters, Bühlmann & Meinshausen, 2016; Manheim & Garrabrant, 2018; Milli et al., 2019; Perdomo et al., 2020; Kleinberg & Raghavan, 2020; Dimitriadis, Gneiting & Jordan, 2021, `[Literature]`).

In the unified mathematical architecture of Stochology, Chapter 48 implements **Gate 1 (Ex-Ante Calibration, Causal Invariance & Performative Audit)** of **Canonical Operator III** ($\mathcal{O}_{\mathrm{gov}}$, [`GovernanceState::canonical_score`](crates/stochology/src/governance.rs#L1206-L1223)):

$$S_{\mathrm{gov}} = \mathcal{O}_{\mathrm{gov}}(\pi_t, \mathcal{I}_t) = \min\bigl(g_{\text{ex-ante}}(\text{CORP}, \text{ICP}, \text{Perdomo}),\; g_{\text{kairos}}(\text{List–Pettit}, \text{Fricker}, \text{Bateson}),\; g_{\text{ex-post}}(\text{Sosa}, \text{Holmström}, \text{Vault})\bigr)$$

![Causal Graphs, Interventions, and Confounding](figures/fig-03-causal-dags.svg)

---

## 7.1 Isotonic CORP-Murphy Brier Score Decomposition via PAV

Before deploying any probabilistic forecast $p_1, \dots, p_n \in (0, 1)$ against binary outcomes $y_1, \dots, y_n \in \{0, 1\}$, how do we separate **honest calibration** (do events predicted at $70\%$ happen $70\%$ of the time?) from **discriminative resolution** (can the model separate high-risk shifts from calm shifts)?

The classical mean squared error of probabilistic forecasts is the **Brier Score** (Brier, 1950):
$$\mathrm{BS}(p) = \frac{1}{n}\sum_{i=1}^n (p_i - y_i)^2$$
Allan H. Murphy (1973) proved that the Brier score partitions into calibration, resolution, and baseline uncertainty components. However, classical implementations partition forecasts into arbitrary user-selected histogram bins $B_1, \dots, B_K$. This heuristic binning suffers from severe flaws: coarse bins mask severe local miscalibration, fine bins suffer from extreme sampling variance, and empirical event frequencies across bins often violate monotonicity.

Timo Dimitriadis, Tilmann Gneiting, and Alexander I. Jordan (2021) solved this via the **CORP** (*Consistent, Optimally Binned, Reproducible*) decomposition ([`GovernanceState::evaluate_corp_pav`](crates/stochology/src/governance.rs#L1250-L1302), `[Literature]`). Rather than selecting arbitrary bins, CORP computes the unique non-decreasing isotonic regression $\hat{\pi}_1 \le \dots \le \hat{\pi}_n$ of the sorted outcomes via the **Pool-Adjacent-Violators (PAV)** algorithm ([`pav_isotonic_non_decreasing`](crates/stochology/src/kernel.rs#L675-L702)):
$$\min_{\hat{\pi}_1 \le \dots \le \hat{\pi}_n} \sum_{i=1}^n (\hat{\pi}_i - y_{(i)})^2$$

### 7.1.1 The Exact Algebraic Identity

Because $\hat{\pi}$ is the orthogonal least-squares projection of the sorted outcome vector onto the convex cone of non-decreasing sequences, the cross-term between the residuals $(y_{(i)} - \hat{\pi}_i)$ and the calibrated centered predictions $(\hat{\pi}_i - \bar{y})$ vanishes identically over every pooled PAV constant block. This yields the exact algebraic identity:

$$\mathrm{BS}(p) = \frac{1}{n}\sum_{i=1}^n (p_i - y_i)^2 = \underbrace{\mathrm{REL}_{\mathrm{CORP}}}_{\text{Miscalibration Loss}} - \underbrace{\mathrm{RES}_{\mathrm{CORP}}}_{\text{Discriminative Power}} + \underbrace{\mathrm{UNC}_0}_{\bar{y}(1-\bar{y})}$$

where:
- $\mathrm{UNC}_0 = \bar{y}(1 - \bar{y}) = \frac{1}{n}\sum_{i=1}^n (y_i - \bar{y})^2$ is the baseline uncertainty of the climatological mean $\bar{y} = \frac{1}{n}\sum y_i$;
- $\mathrm{RES}_{\mathrm{CORP}} = \frac{1}{n}\sum_{i=1}^n (\hat{\pi}_i - \bar{y})^2 \ge 0$ measures the sharpness with which the model separates distinct outcome regimes;
- $\mathrm{BS}(\hat{\pi}) = \frac{1}{n}\sum_{i=1}^n (\hat{\pi}_i - y_{(i)})^2 = \mathrm{UNC}_0 - \mathrm{RES}_{\mathrm{CORP}}$ is the residual loss achieved by an optimally calibrated monotone model;
- $\mathrm{REL}_{\mathrm{CORP}} = \mathrm{BS}(p) - \mathrm{BS}(\hat{\pi}) \ge 0$ is the resolvable calibration deficit (miscalibration loss).

This decomposition closes to machine precision (`corp_closure_error < 1e-14`) on all binary datasets.

### 7.1.2 Finite-Sample PAV Optimism and Split-Half Cross-Validation Debiasing

Because the PAV algorithm fits the monotone envelope $\hat{\pi}$ to the observed outcomes in-sample, $\hat{\pi}$ absorbs random sample fluctuations, producing an optimistic estimate of calibration (in-sample $\mathrm{REL}$ is slightly too small).

To prevent overconfidence, [`GovernanceState::evaluate_corp_pav_optimism_cv`](crates/stochology/src/governance.rs#L833-L882) estimates the finite-sample PAV optimism $\hat{b}_{\mathrm{PAV}}^{\mathrm{CV}}$ via interleaved split-half cross-validation. By sorting observations by $(p_i, i)$, partitioning into even and odd indices, and evaluating each fold's isotonic fit against the held-out fold:
$$\mathrm{REL}_{\mathrm{CORP}}^{\mathrm{debiased}} = \max\bigl(0,\, \mathrm{REL}_{\mathrm{CORP}} - \hat{b}_{\mathrm{PAV}}^{\mathrm{CV}}\bigr)$$
This debiasing provides a rigorous finite-sample guarantee against overfitting.

### 7.1.3 Continuous Ranked Probability Score (CRPS) Decomposition

Hersbach (2000) demonstrated that the Continuous Ranked Probability Score (CRPS) of a predictive cumulative distribution $F$ against realized outcome $y \in \mathbb{R}$ can be expressed as an integral of Brier scores across all thresholds $\tau \in \mathbb{R}$:
$$\mathrm{CRPS}(F, y) = \int_{-\infty}^\infty \mathrm{BS}\bigl(F(\tau),\, \mathbf{1}\{y \le \tau\}\bigr)\, d\tau$$
In [`GovernanceState::corp_pav_crps_decomposition`](crates/stochology/src/governance.rs#L1319-L1385), Stochology applies the CORP-PAV operator to each binary slice along a threshold quadrature grid, proving that the integral of reliability, resolution, and uncertainty reconstructs the overall CRPS to quadrature precision.

---

## 7.2 Invariant Causal Prediction (ICP), Kennedy–O'Hagan Structural Discrepancy, and Tian–Pearl Bounds

Even a well-calibrated correlation collapses when the agent intervenes (`do(X = x)`). As Judea Pearl (2009) established across the **Ladder of Causation**, passive conditioning $\mathbb{P}(Y \mid X = x)$ (Rung 1) confounds true causal mechanisms with transient environmental proxies. Stochology pairs three structural instruments inside [`GovernanceState::from_ex_ante_gate`](crates/stochology/src/governance.rs#L1391-L1583):

### 7.2.1 Invariant Causal Prediction (ICP) Across Heterogeneous Environments

Jonas Peters, Peter Bühlmann, and Nicolai Meinshausen (2016) formalized the principle of **Invariant Causal Prediction (ICP)** (`[Literature]`): if an agent observes the system across multiple distinct environments or regimes $e \in \mathcal{E}$ (e.g., historical epochs, geographical regions, seasonal shifts, or policy interventions), the conditional distribution of $Y$ given its true direct causal parents $\mathbf{PA}(Y)$ is invariant across all environments:
$$Y^{(e)} \mid X_{\mathbf{PA}(Y)}^{(e)} \sim F_Y \quad \forall e \in \mathcal{E}$$
Conversely, any regression model relying on a non-causal proxy $X_{\mathrm{proxy}}$ will exhibit structural coefficient drift across environments as external conditions change.

In [`GovernanceState::evaluate_icp_subset_intersection`](crates/stochology/src/governance.rs#L977-L1117), Stochology tests all $2^d$ candidate predictor subsets $S \subseteq \{0, \dots, d-1\}$. For each subset $S$, it computes the pooled regression sum of squares $\mathrm{RSS}_{\mathrm{restricted}}$ and the environment-specific regression sum of squares $\mathrm{RSS}_{\mathrm{unrestricted}}$, constructing the dimensionless Chow $F$-statistic:
$$F_S = \frac{(\mathrm{RSS}_{\mathrm{restricted}} - \mathrm{RSS}_{\mathrm{unrestricted}}) / df_1}{\mathrm{RSS}_{\mathrm{unrestricted}} / df_2}$$
where $df_1 = (|\mathcal{E}| - 1)|S|$ and $df_2 = n_{\mathrm{total}} - |\mathcal{E}||S|$ on centered data. A subset $S$ is accepted into the invariant set $\mathcal{A}$ if $F_S \le F_{1-\alpha}(df_1, df_2)$. The estimated causal parent set is the intersection:
$$\hat{S}^* = \bigcap_{S \in \mathcal{A}} S$$
By taking the intersection, any variable that can be omitted without destroying invariance is discarded, guaranteeing that $\mathbb{P}(\hat{S}^* \subseteq \mathbf{PA}(Y)) \ge 1 - \alpha$.

#### The Agnotological Low-Power Guard
When sample size per environment is small ($n_{\mathrm{eff}}/\text{env} < 150$), the Chow test lacks power to reject candidate subsets, leading to the erroneous acceptance of the empty set $S = \emptyset$. Under intersection, this causes the estimated causal set to collapse silently to the empty set: $\hat{S}^* = \emptyset$. A naive optimizer exploits this to claim that "no causal vulnerability exists." [`IcpIdentificationStatus`](crates/stochology/src/governance.rs#L333-L342) classifies this regime as `EpistemicUnderpoweredSet`, mandating precautionary buffering at the upper bound $U_t$.

### 7.2.2 Kennedy & O'Hagan's (2001) Anchored Gaussian Process Discrepancy

No mathematical model $f_\theta(x)$ is an exact description of physical reality. Marc C. Kennedy and Anthony O'Hagan (2001) formalized this by modeling the true physical response $y(x)$ as (`[Literature]`):
$$y(x) = f_\theta(x) + \delta(x) + \varepsilon$$
where $\delta(x) \sim \mathcal{GP}(0, k(x, x'))$ is a structural discrepancy function modeled by a Gaussian Process with squared-exponential kernel:
$$k(x, x') = s^2 \exp\left(-\frac{(x - x')^2}{2\ell^2}\right)$$
In [`GovernanceState::from_ex_ante_gate`](crates/stochology/src/governance.rs#L1391-L1583), the discrepancy GP is conditioned on empirical forecast tercile residuals $\mathbf{r} = (\bar{y}_k - \bar{f}_k)_{k=1}^3$ at anchor points $x \in \{-1, 0, 1\}$.
- **In-Support Regime:** Inside the observed data domain ($x^* \in [-1, 1]$), the posterior standard deviation $\sigma_{\mathrm{post}}(x^*)$ shrinks toward observational noise ($\sigma_{\mathrm{in}} = 0.044$), correcting structural bias (`[Measured]`).
- **Extrapolation Regime:** As the decision moves into untested operational regimes ($|x^* - x_{\mathrm{train}}| \gg \ell$), the kernel correlation vector $\mathbf{k}_* \to \mathbf{0}$, and posterior uncertainty expands back to the unanchored prior ceiling $\sqrt{s^2 + \sigma_{\mathrm{noise}}^2} = 0.500$ (`[Measured]`). This expansion penalizes extrapolation arrogance and forces the agent to maintain reserves.

### 7.2.3 Jin Tian & Judea Pearl's (2000) Sharp Counterfactual Bounds

Attributing causal responsibility after an event requires Pearl's Rung 3: counterfactual probabilities. Combining experimental intervention rates ($p_{y_1} = \mathbb{P}(Y_{x_1}=1), p_{y_0} = \mathbb{P}(Y_{x_0}=1)$) with observational joint probabilities ($p_{xy} = \mathbb{P}(X=x, Y=y)$), Jin Tian and Judea Pearl (2000) derived sharp bounds for (`[Literature]`):
- **Probability of Necessity ($\mathrm{PN}$):** Did taking action $X=1$ cause outcome $Y=1$?
  $$\max\left(0,\, \frac{\mathbb{P}(Y=1) - \mathbb{P}(Y_{x_0}=1)}{\mathbb{P}(X=1, Y=1)}\right) \le \mathrm{PN} \le \min\left(1,\, \frac{\mathbb{P}(Y_{x_0}=0) - \mathbb{P}(X=0, Y=0)}{\mathbb{P}(X=1, Y=1)}\right)$$
- **Probability of Sufficiency ($\mathrm{PS}$):** Would taking action $X=1$ have produced $Y=1$ in an untreated case?
  $$\max\left(0,\, \frac{\mathbb{P}(Y_{x_1}=1) - \mathbb{P}(Y=1)}{\mathbb{P}(X=0, Y=0)}\right) \le \mathrm{PS} \le \min\left(1,\, \frac{\mathbb{P}(Y_{x_1}=1) - \mathbb{P}(X=1, Y=1)}{\mathbb{P}(X=0, Y=0)}\right)$$
- **Probability of Necessity and Sufficiency ($\mathrm{PNS}$):** The probability that the outcome responds strictly to the intervention ($Y_{x_1}=1 \land Y_{x_0}=0$).

Evaluated in [`GovernanceState::evaluate_tian_pearl_bounds`](crates/stochology/src/governance.rs#L1149-L1174), these sharp bounds provide robust causal accountability without arbitrary parametric functional-form assumptions.

---

## 7.3 Performative Prediction ($\theta_{\mathrm{PS}}$ vs. $\theta_{\mathrm{PO}}$), MacKenzie's Regimes, and Goodhart's Four Variants

When an algorithm or decision rule is deployed into a social or institutional environment, agents adapt to the rule. Juan Perdomo, Tijana Zrnic, Celestine Mendler-Dünner, and Moritz Hardt (2020) formalized this as **performative prediction** (`[Literature]`).

### 7.3.1 Performative Stability vs. Stackelberg Performative Optimality

Let $\theta \in \Theta$ be the deployed model parameter. Deploying $\theta$ induces a distribution shift, so observations are drawn from $\mathcal{D}(\theta)$.
- The **Decoupled Loss** evaluates candidate parameter $\theta$ on the distribution induced by parameter $\theta'$:
  $$\mathcal{L}(\theta; \theta') = \mathbb{E}_{Z \sim \mathcal{D}(\theta')}[\ell(Z; \theta)]$$
- **Repeated Risk Minimization (RRM):** Practitioners repeatedly retrain their models on recent historical data:
  $$\theta_{t+1} = \arg\min_\theta \mathcal{L}(\theta; \theta_t)$$
- **Performative Stability ($\theta_{\mathrm{PS}}$):** A fixed point of RRM satisfies:
  $$\theta_{\mathrm{PS}} = \arg\min_\theta \mathcal{L}(\theta; \theta_{\mathrm{PS}})$$
  At $\theta_{\mathrm{PS}}$, the model appears perfectly calibrated on the distribution generated by its own deployment!
- **Performative Optimality ($\theta_{\mathrm{PO}}$):** The true Stackelberg leader minimizes total performative risk, anticipating the distribution shift:
  $$\theta_{\mathrm{PO}} = \arg\min_\theta \mathrm{PR}(\theta) = \arg\min_\theta \mathcal{L}(\theta; \theta)$$

Differentiating performative risk reveals why stability fails to achieve optimality:
$$\nabla \mathrm{PR}(\theta) = \underbrace{\left.\nabla_\theta \mathcal{L}(\theta; \theta')\right|_{\theta'=\theta}}_{\text{Standard Training Gradient}} + \underbrace{\left.\nabla_{\theta'} \mathcal{L}(\theta; \theta')\right|_{\theta'=\theta}}_{\text{Performative Distribution Gradient}}$$
At $\theta_{\mathrm{PS}}$, the standard training gradient vanishes by definition, but the performative gradient is non-zero whenever $\varepsilon \ne 0$. Consequently, $\theta_{\mathrm{PS}}$ is almost never performatively optimal.

Under linear response $Y \sim \mathcal{N}(\mu_0 + \varepsilon \theta, \sigma^2)$ with $L_2$ regularization $\lambda$ ([`GovernanceState::evaluate_performative_system`](crates/stochology/src/governance.rs#L1123-L1143)):
$$\theta_{\mathrm{PS}} = \frac{\mu_0}{1 + \lambda - \varepsilon}, \qquad \theta_{\mathrm{PO}} = \frac{(1 - \varepsilon)\mu_0}{(1 - \varepsilon)^2 + \lambda}$$
RRM contracts to $\theta_{\mathrm{PS}}$ if and only if the Wasserstein-1 sensitivity $\varepsilon$ satisfies the Lipschitz contractivity condition:
$$\varepsilon < \frac{\gamma}{\beta}$$
where $\gamma$ is the strong convexity parameter and $\beta$ is the smoothness of the loss function.

### 7.3.2 Donald MacKenzie's (2006) Four Performativity Regimes

In *An Engine, Not a Camera*, Donald MacKenzie (2006) showed that economic models format markets rather than merely describing them (`[Literature]`). Stochology classifies performativity into four regimes ([`MacKenzieRegime`](crates/stochology/src/governance.rs#L58-L86)):
1. **`CameraPassive` ($\varepsilon \approx 0$):** Deploying the model has no measurable impact on the underlying data distribution.
2. **`EffectivePerformative` ($\varepsilon > 0, |\theta_{\mathrm{PS}} - \theta_{\mathrm{PO}}| \le 0.05$):** Deploying the rule reshapes the world in alignment with its objective, improving social welfare.
3. **`BarnesianSelfFulfilling` ($\varepsilon > 0, |\theta_{\mathrm{PS}} - \theta_{\mathrm{PO}}| > 0.05$):** Retraining drives predictive residuals toward zero ($\theta \to \theta_{\mathrm{PS}}$) by molding agents to conform to the formula, yet produces substantial deadweight loss relative to $\theta_{\mathrm{PO}}$.
4. **`CounterperformativeDestructive` ($\varepsilon < -0.15$ or $\mathrm{risk}(\theta_{\mathrm{PS}}) > 2\,\mathrm{risk}(\theta_{\mathrm{PO}})$):** Deploying the rule triggers feedback that undermines its own operational premises (e.g., dynamic hedging during the October 1987 stock market crash creating the post-crash volatility skew, `[Literature]`).

### 7.3.3 Strategic Effort vs. Cosmetic Gaming and Goodhart's Four Variants

When an algorithm allocates rewards based on proxy metric $M$, agents reallocate effort. Jon Kleinberg and Manish Raghavan (2020) and Smitha Milli et al. (2019) separate (`[Literature]`):
- **Causal Improvement Effort ($x_{\mathrm{real}}$):** Investment that improves the substantive goal $G$;
- **Cosmetic Gaming ($x_{\mathrm{game}}$):** Cheap manipulation of proxy $M$ that leaves $G$ unchanged or degraded.

David Manheim and Scott Garrabrant (2018) formalize the four mechanisms of **Goodhart's Law** ([`GoodhartVariant`](crates/stochology/src/governance.rs#L91-L121), `[Literature]`):
1. **Regressional Goodhart:** Selecting based on a noisy proxy $M = G + \varepsilon$ selects for extreme noise $\varepsilon$. As the selection threshold increases, $\mathbb{E}[G \mid M \ge \tau]$ falls further below $\tau$.
2. **Extremal Goodhart:** Pushing $M$ into extreme tail percentiles ($z > 2.5$) takes the system outside the historical manifold where $M$ and $G$ were positively correlated.
3. **Causal Goodhart:** Intervening on a downstream symptom $M$ ($G \to M$) severs the correlation without moving the causal parent $G$.
4. **Adversarial Goodhart:** Strategic agents actively optimize $M$ at low cost, collapsing the substantive metric (`goodhart_metric_collapsed = true`).

---

## 7.4 Concrete Step-by-Step Worked Numeric Examples

To build complete operational understanding, we trace the exact arithmetic of three core mechanisms.

### 7.4.1 Worked Example 1: Hand-Calculated CORP-PAV Murphy Decomposition ($n = 5$)

Consider a toy sample of $n = 5$ sorted probabilistic forecasts $p$ and realized binary outcomes $y$:
$$\mathbf{p} = (0.10,\, 0.30,\, 0.40,\, 0.70,\, 0.90), \qquad \mathbf{y} = (0,\, 1,\, 0,\, 1,\, 1)$$

**Step 1: Check Monotonicity and Pool Violators**
The sequence of outcomes $y = (0, 1, 0, 1, 1)$ contains an inversion between index 2 ($y_2 = 1$) and index 3 ($y_3 = 0$), since $1 > 0$.
- Form initial blocks: $B_1 = [0]$ (mean $0.0$), $B_2 = [1]$ (mean $1.0$), $B_3 = [0]$ (mean $0.0$), $B_4 = [1]$ (mean $1.0$), $B_5 = [1]$ (mean $1.0$).
- Compare $B_2$ and $B_3$: violator detected ($1.0 > 0.0$).
- Pool into combined block $B_{2,3} = [1, 0]$ with weight $w = 2$ and pooled mean $\frac{1 + 0}{2} = 0.50$.
- Check adjacent blocks: $0.0 \le 0.50 \le 1.0 \le 1.0$. Monotonicity is satisfied.
- The fitted isotonic recalibration vector is:
  $$\hat{\boldsymbol{\pi}} = (0.00,\, 0.50,\, 0.50,\, 1.00,\, 1.00)$$

**Step 2: Compute Brier Scores**
- **Forecast Brier Score $\mathrm{BS}(p)$:**
  $$\mathrm{BS}(p) = \frac{1}{5}\left[(0.10-0)^2 + (0.30-1)^2 + (0.40-0)^2 + (0.70-1)^2 + (0.90-1)^2\right]$$
  $$\mathrm{BS}(p) = \frac{1}{5}[0.01 + 0.49 + 0.16 + 0.09 + 0.01] = \frac{0.76}{5} = 0.1520$$
- **Calibrated Brier Score $\mathrm{BS}(\hat{\pi})$:**
  $$\mathrm{BS}(\hat{\pi}) = \frac{1}{5}\left[(0.00-0)^2 + (0.50-1)^2 + (0.50-0)^2 + (1.00-1)^2 + (1.00-1)^2\right]$$
  $$\mathrm{BS}(\hat{\pi}) = \frac{1}{5}[0.00 + 0.25 + 0.25 + 0.00 + 0.00] = \frac{0.50}{5} = 0.1000$$

**Step 3: Compute Decomposition Components**
- **Miscalibration Loss ($\mathrm{REL}_{\mathrm{CORP}}$):**
  $$\mathrm{REL}_{\mathrm{CORP}} = \mathrm{BS}(p) - \mathrm{BS}(\hat{\pi}) = 0.1520 - 0.1000 = 0.0520$$
- **Baseline Uncertainty ($\mathrm{UNC}_0$):**
  Base rate $\bar{y} = \frac{0 + 1 + 0 + 1 + 1}{5} = \frac{3}{5} = 0.60$.
  $$\mathrm{UNC}_0 = \bar{y}(1 - \bar{y}) = 0.60 \times (1 - 0.60) = 0.2400$$
- **Discriminative Resolution ($\mathrm{RES}_{\mathrm{CORP}}$):**
  $$\mathrm{RES}_{\mathrm{CORP}} = \frac{1}{5}\sum_{i=1}^5 (\hat{\pi}_i - \bar{y})^2 = \frac{1}{5}\left[(0.00-0.60)^2 + 2\times(0.50-0.60)^2 + 2\times(1.00-0.60)^2\right]$$
  $$\mathrm{RES}_{\mathrm{CORP}} = \frac{1}{5}[0.36 + 2(0.01) + 2(0.16)] = \frac{1}{5}[0.36 + 0.02 + 0.32] = \frac{0.70}{5} = 0.1400$$

**Step 4: Check Algebraic Closure**
$$\mathrm{REL}_{\mathrm{CORP}} - \mathrm{RES}_{\mathrm{CORP}} + \mathrm{UNC}_0 = 0.0520 - 0.1400 + 0.2400 = 0.1520 = \mathrm{BS}(p)$$
The identity holds with exactly zero closure error.

### 7.4.2 Worked Example 2: Hand-Calculated Performative Stability vs. Optimality

Consider a linear performative prediction system with initial base state $\mu_0 = 1.0$, performative sensitivity $\varepsilon = 0.45$, regularization parameter $\lambda = 0.20$, and residual variance $\sigma^2 = 0.10$.

**Step 1: Check Contractivity**
The loss function is quadratic: $\beta = 1 + \lambda = 1.20$, $\gamma = 1 + \lambda = 1.20$.
$$\frac{\varepsilon \beta}{\gamma} = \frac{0.45 \times 1.20}{1.20} = 0.45 < 1.0$$
The system is contractive, guaranteeing that RRM converges to a unique stable point $\theta_{\mathrm{PS}}$.

**Step 2: Solve for Performative Stability $\theta_{\mathrm{PS}}$**
$$\theta_{\mathrm{PS}} = \frac{\mu_0}{1 + \lambda - \varepsilon} = \frac{1.0}{1 + 0.20 - 0.45} = \frac{1.0}{0.75} = \frac{4}{3} \approx 1.3333$$
Under deployment of $\theta_{\mathrm{PS}}$, the expected response is:
$$\mathbb{E}[Y \mid \theta_{\mathrm{PS}}] = \mu_0 + \varepsilon \theta_{\mathrm{PS}} = 1.0 + 0.45 \times \frac{4}{3} = 1.0 + 0.60 = 1.60$$
The prediction bias is $1.60 - 1.3333 = 0.2667$. The total performative risk is:
$$\mathrm{PR}(\theta_{\mathrm{PS}}) = \frac{1}{2}\left[(1.60 - 1.3333)^2 + 0.10\right] + \frac{0.20}{2}(1.3333)^2$$
$$\mathrm{PR}(\theta_{\mathrm{PS}}) = \frac{1}{2}[0.0711 + 0.10] + 0.10 \times 1.7778 = 0.0856 + 0.1778 = 0.2633 \approx 0.263$$

**Step 3: Solve for Performative Optimality $\theta_{\mathrm{PO}}$**
Anticipating that $\mathbb{E}[Y \mid \theta] = \mu_0 + \varepsilon \theta$, the net bias is $\mu_0 - (1 - \varepsilon)\theta = 1.0 - 0.55\theta$.
$$\theta_{\mathrm{PO}} = \frac{(1 - \varepsilon)\mu_0}{(1 - \varepsilon)^2 + \lambda} = \frac{0.55 \times 1.0}{(0.55)^2 + 0.20} = \frac{0.55}{0.3025 + 0.20} = \frac{0.55}{0.5025} \approx 1.0945$$
Under deployment of $\theta_{\mathrm{PO}}$, the expected response is:
$$\mathbb{E}[Y \mid \theta_{\mathrm{PO}}] = 1.0 + 0.45 \times 1.0945 = 1.4925$$
The prediction bias is $1.4925 - 1.0945 = 0.3980$. The total performative risk is:
$$\mathrm{PR}(\theta_{\mathrm{PO}}) = \frac{1}{2}\left[(0.3980)^2 + 0.10\right] + \frac{0.20}{2}(1.0945)^2$$
$$\mathrm{PR}(\theta_{\mathrm{PO}}) = \frac{1}{2}[0.1584 + 0.10] + 0.10 \times 1.1980 = 0.1292 + 0.1198 = 0.2490 \approx 0.249$$

**Step 4: Comparison and Regime Classification**
- Performative Gap: $|\theta_{\mathrm{PS}} - \theta_{\mathrm{PO}}| = |1.3333 - 1.0945| = 0.2388 > 0.05$.
- Substantive Risk Reduction: $\mathrm{PR}(\theta_{\mathrm{PO}}) = 0.249 < \mathrm{PR}(\theta_{\mathrm{PS}}) = 0.263$ (a $5.4\%$ reduction in overall social risk, `[Assumed]`).
- Because $\varepsilon = 0.45 > 0$ and the performative gap exceeds $0.05$, the system is in MacKenzie's **`BarnesianSelfFulfilling`** regime: Repeated Risk Minimization over-intervenes, chasing its own self-induced distribution shift.

### 7.4.3 Worked Example 3: Invariant Causal Prediction Toy System (2 Predictors, 2 Environments)

Consider predicting target $Y$ from two candidates $X_1$ (true cause) and $X_2$ (spurious proxy) across two regimes $e \in \{1, 2\}$ with $n_1 = n_2 = 100$:
- Ground Truth: $Y^{(e)} = 2.0 X_1^{(e)} + \varepsilon$, with $\mathrm{Var}(\varepsilon) = 1.0$.
- In Environment 1 (calm regime): $X_2^{(1)} = 0.5 X_1^{(1)} + \eta_1$, so an observational regression of $Y$ on $X_2$ yields slope $\hat{\beta}_2^{(1)} \approx 4.0$.
- In Environment 2 (storm regime): an external intervention shifts the proxy coupling to $X_2^{(2)} = 1.5 X_1^{(2)} + \eta_2$, so an observational regression of $Y$ on $X_2$ yields slope $\hat{\beta}_2^{(2)} \approx 1.33$.

Evaluating candidate subsets:
1. **Candidate $S = \{X_1\}$:** Estimated slopes are $\hat{\beta}_1^{(1)} = 2.01$ and $\hat{\beta}_1^{(2)} = 1.99$. The Chow test statistic is $F_{\{X_1\}} = 0.02 \le F_{0.95}(1, 196) \approx 3.89$. Subset $\{X_1\}$ is **accepted**.
2. **Candidate $S = \{X_2\}$:** Estimated slopes are $\hat{\beta}_2^{(1)} = 4.02$ and $\hat{\beta}_2^{(2)} = 1.31$. The Chow test statistic is $F_{\{X_2\}} = 48.6 \gg 3.89$. Subset $\{X_2\}$ is **rejected**.
3. **Empty Set $S = \emptyset$:** Residual variance differs significantly between regimes ($F_\emptyset \gg 3.89$). Subset $\emptyset$ is **rejected**.
4. **Candidate $S = \{X_1, X_2\}$:** The conditional distribution is invariant because conditioning on the true cause shields $Y$, yielding $F_{\{X_1, X_2\}} \le F_{\mathrm{crit}}$. Subset $\{X_1, X_2\}$ is **accepted**.

Taking the intersection:
$$\hat{S}^* = \{X_1\} \cap \{X_1, X_2\} = \{X_1\}$$
The spurious proxy $X_2$ is eliminated, isolating the true causal parent.

---

## 7.5 Laboratory 7: Real Data on NOAA Atlantic Storms (`atlantic_storms.csv`)

Running laboratory 7 (`cargo run --release --bin stochology -- --chapter 7`) evaluates Gate 1 of Canonical Operator III on strict +6-hour lead-time synoptic transitions $(t \to t+1)$ along the NOAA HURDAT2 Atlantic Tropical Storm tracks (`data/atlantic_storms.csv`, $n = 19{,}025$ consecutive same-storm strict +6h transitions, `[Measured]`):

```text
=== CHAPTER 7: FILTER III (S_GOV) — REACTIVE WORLDS, PERFORMATIVITY & GOODHART ===
  Canonical Operator III — Gate 1 (Ex-Ante Calibration, Causal Invariance & Performative Gaming Guard):
  1. CORP-PAV +6h Lead-Time Storm Forecast Brier Decomposition [Measured] (data/atlantic_storms.csv, n_transitions = 19025 from 20778 total rows, 1753 track-terminal rows [1752 unpaired + 1 tail]):
     Total Brier = 0.0731 = REL (0.0028) - RES (0.1171) + UNC (0.1873) [closure_err = 4.62e-15]
     Finite-Sample CORP-PAV & ICP Guard [Measured]: b_PAV_CV = 0.0002, REL_debiased = 0.0026, ICP(n_eff/env=184.5) = IdentifiedInvariantSet (benchmark n_eff/env=30 -> EpistemicUnderpoweredSet [Assumed])
  2. Multi-Regime Invariant Causal Prediction (ICP) [Measured]: physical slope range = 0.009 < spurious proxy range = 0.076 (accepted_subsets = 2, S_hat_mask = 0b01)
  3. Kennedy-O'Hagan GP Discrepancy [Measured tercile anchors]: in_sample (mean=-0.0283, std=0.044) vs out_of_sample (mean=-0.0000, std=0.500) | Tian-Pearl [Assumed] PN=[0.75, 1.00], PS=[0.50, 0.75], PNS=[0.50, 0.70]
  4. Perdomo Performative Stability vs Optimality [Assumed]: theta_PS = 1.333 (risk=0.263) vs theta_PO = 1.095 (risk=0.249, MacKenzie = BarnesianSelfFulfilling)
  5. Goodhart Metric Collapse [Assumed]: collapsed = true, variant = Adversarial
```

### 7.5.1 In-Depth Analysis of the Empirical Results

- **Strict +6h Lead-Time CORP-PAV Identity Closure & CV Optimism Debiasing (`data/atlantic_storms.csv`, `[Measured]`):**
  From $20{,}778$ total observations, filtering for same-storm sequential entries separated by exactly 6 hours (`count_atlantic_storms_transitions`) isolates $n = 19{,}025$ valid transitions, excluding $1{,}753$ track-terminal or irregular landfall fixes (`[Measured]`). Fitting a logistic model $P(\text{wind}_{t+1} \ge 64\text{ kt} \mid \text{pressure}_t)$ on the first half of transitions via Newton–Raphson IRLS and evaluating on the full series yields an overall Brier score of $\mathrm{BS} = 0.0731$. The isotonic CORP decomposition resolves this into miscalibration loss $\mathrm{REL} = 0.0028$, discriminative resolution $\mathrm{RES} = 0.1171$, and baseline uncertainty $\mathrm{UNC} = 0.1873$, closing to **$4.62 \times 10^{-15}$** (`[Measured]`). Resolution heavily dominates miscalibration ($\mathrm{RES} \gg \mathrm{REL}$), reflecting high operational predictive skill. Split-half cross-validation detects a minor in-sample optimism of $\hat{b}_{\mathrm{PAV}}^{\mathrm{CV}} = 0.0002$, yielding debiased reliability $\mathrm{REL}_{\mathrm{debiased}} = 0.0026$ (`[Measured]`).
- **Physical Mechanism vs. Geographic Proxy Across Seasonal Regimes (`atlantic_storms.csv`, `[Measured]`):**
  Splitting the storm record into three seasonal environments (`fringe`, `shoulder_aug_oct`, `peak_sep`), subset-intersection ICP tests physical barometric pressure deficit $(1010 - P_t)/20$ against geographic latitude $(L_t - 25)/10$. The physical deficit slope is highly stable across regimes ($\text{range} = 0.009$, `[Measured]`), whereas the latitude proxy exhibits substantial drift ($\text{range} = 0.076$, `[Measured]`). Testing all $2^2 = 4$ subsets accepts 2 subsets, and their bitwise intersection yields `S_hat_mask = 0b01` (`[Measured]`), isolating the physical pressure deficit while rejecting the latitude proxy. Under $n_{\mathrm{eff}}/\text{env} = 184.5 \ge 150$, the system is safely classified as `IdentifiedInvariantSet` (`[Measured]`), whereas a low-power benchmark ($n_{\mathrm{eff}}/\text{env} = 30$) trips `EpistemicUnderpoweredSet` (`[Assumed]`).
- **Kennedy–O'Hagan Structural Discrepancy & Counterfactual Bounds (`[Measured]` & `[Assumed]`):**
  Conditioned on empirical forecast tercile residuals, the in-support posterior discrepancy mean is $-0.0283$ with tight uncertainty ($\sigma = 0.044$, `[Measured]`), while an extrapolated query at $x^* = 8.0$ reverts to the unanchored prior ($\text{mean} = -0.0000, \sigma = 0.500$, `[Measured]`), demonstrating automated epistemic self-containment. Under benchmark counterfactual inputs, Tian–Pearl bounds isolate $\mathrm{PN} \in [0.75, 1.00]$, $\mathrm{PS} \in [0.50, 0.75]$, and $\mathrm{PNS} \in [0.50, 0.70]$ (`[Assumed]`).
- **Performative Stability vs. Stackelberg Optimality & Goodhart Collapse (`[Assumed]`):**
  Deploying the model under Barnesian self-fulfilling feedback ($\varepsilon = 0.45$) leads naive RRM to settle at $\theta_{\mathrm{PS}} = 1.333$ with risk $0.263$, whereas performative optimization steers to $\theta_{\mathrm{PO}} = 1.095$ with lower risk $0.249$ (`[Assumed]`). In the presence of cosmetic proxy gaming ($+0.35$ proxy gain vs $-0.12$ substantive goal drop), the institutional metric collapses under `Adversarial` Goodhart drift (`collapsed = true`, `[Assumed]`).

### 7.6 Optimising inside the dream: extremal Goodhart in latent space

§7.2 established that the Kennedy–O'Hagan discrepancy is a function of position: small where anchors exist,
returning to the prior ceiling outside them. §7.3 classified the four variants of Goodhart drift. The two results
compose into the sharpest failure mode of a model-based decision loop, and the composition deserves its own
paragraph because it is the one an optimiser finds *first*.

Let $\hat{M}$ be the surrogate and $\delta(z)$ the structural discrepancy at latent state $z$. A policy
optimised against $\hat{M}$ alone is not optimising the goal; it is optimising the goal *plus* the negative of
the discrepancy. Since optimisation is free to choose the trajectory, it selects the region where the gap between
$\hat{M}$ and the world is largest and the surrogate pays least for it — the region where $\delta(z)$ is maximal.
The endpoint of that search is a **Goodhart extremal** point: the proxy is at its extreme and the correlation
between proxy and goal, which held only inside the anchored support, has already been abandoned. It is
simultaneously **adversarial**, because the search is not passive sampling of a noisy proxy but an active hunt for
the proxy's blind spot. The classification is therefore not a matter of degree between the four variants; the
extremal and adversarial readings are two descriptions of the same optimiser action.

`governance.rs::LatentModelExploitationAudit` makes the composition computable on a canonical fixture
`[Simulated]`. A linear surrogate is fitted by least squares on anchors confined to a support radius; the greedy
policy is optimised inside it; the same actions are executed against a world that carries one absorbing constraint
the surrogate never observed. The dreamer leaves the support by 2.87 radii, spends 58.3% of its steps outside it,
reaches a terminal discrepancy of 0.883 — a value in the top decile of the attainable range — and destroys the
world at step 6, while its dreamed return of 18.93 stays 4.4 times its realised return of 4.27. The variant
reported by `governance.rs::GoodhartVariant::classify` is `Adversarial`, and no reading of the fixture supports
`Regressional`: the loss here is not produced by selecting on noise but by selecting on *absence of data*.

The Boltzmann temperature over the action distribution is the standard regulariser, and the audit separates what
it achieves from what it is often claimed to achieve `[Simulated]`. Across $\tau \in \{0.05, 0.20, 0.80, 3.20,
12.80\}$ the realised return rises monotonically (4.27, 4.84, 5.43, 7.88, 7.88), ruin recedes (steps 6, 7, 8,
then survival) and the terminal discrepancy falls from 0.875 to 0.020. Temperature is a **distributional
robustness** device: it removes the commitment to the single action the surrogate over-rewards, so the trajectory
never travels far enough to meet the unmodelled constraint. What it does not do is certify $\hat{M}$. The surviving
arm survives by abstaining, and an abstaining policy cannot distinguish a model that is true at the edge of its
support from one that is merely unexplored there.

The institutional consequence is the one this chapter has been building toward. A temperature is a *policy-level*
mitigation, and like every policy-level mitigation it is invisible to the audit that matters, because the model
still reports the same confidence everywhere. Certification is an *evidence-level* act: it requires that the
discrepancy be estimated where anchors exist, bounded where they do not, and that the operating point be refused
when the trajectory leaves the anchored region — which is exactly the ex-ante gate of
`governance.rs::GovernanceState::from_ex_ante_gate`, and the reason the gate is applied before actuation rather
than after the loss is observed.

---

## Exercises

1. **Performative Erosion in `CommunityReserveEnv`:**
   Inspect [`crates/stochology/src/engine.rs`](crates/stochology/src/engine.rs). In the sequential simulation loop, the `NaiveOptimizer` operates with a high proxy incentive pressure of $0.85$.
   - **Question:** Trace how `performative_erosion` accumulates across shifts $t = 1..T$. Why does optimizing the proxy metric temporarily mask risk while degrading the true recovery capacity $D_t$, ultimately accelerating ruin at the absorbing barrier?
   - **Guidance:** In `engine.rs`, notice that `performative_erosion` scales with `proxy_incentive_pressure * (1.0 - true_capacity_fraction)`. As the naive optimizer cuts physical slack to boost short-term cost efficiency, the proxy remains high while true vital resilience collapses. Contrast this with `StochologicalAgent`, which applies the `ReflexiveBrakeAudit` to throttle proxy incentives whenever performative drift is detected.

2. **Kennedy–O'Hagan Extrapolation Guard Analysis:**
   In [`GovernanceState::from_ex_ante_gate`](crates/stochology/src/governance.rs#L1391-L1583), the discrepancy Gaussian Process uses prior variance $s^2 = 0.50^2 = 0.25$ and squared-exponential kernel length-scale $\ell = 0.65$.
   - **Question:** Analytically derive the distance $|x^* - x_{\mathrm{anchor}}|$ at which the posterior variance $\sigma_{\mathrm{post}}^2(x^*)$ recovers $95\%$ of the prior variance ceiling $s^2$. How does this prevent technocratic overconfidence when a policy model is deployed outside historical support?
   - **Guidance:** The posterior variance at query point $x^*$ is $\sigma^2(x^*) = s^2 - \mathbf{k}_*^T (K + \sigma_n^2 I)^{-1} \mathbf{k}_*$. For a distant extrapolation point, $\mathbf{k}_* \approx s^2 \exp\left(-\frac{\Delta x^2}{2\ell^2}\right)$. Setting the reduction term to $\le 0.05 s^2$ gives $\exp\left(-\frac{\Delta x^2}{\ell^2}\right) \approx 0.05$, yielding $\Delta x \approx \ell \sqrt{\ln(20)} \approx 1.73 \ell$. For $\ell = 0.65$, any query beyond $\Delta x \approx 1.12$ units outside the anchor envelope recovers $>95\%$ prior uncertainty.

3. **Derivation of the Performative Stackelberg Gradient:**
   Consider the performative risk function $\mathrm{PR}(\theta) = \frac{1}{2}(\mu_0 - (1 - \varepsilon)\theta)^2 + \frac{1}{2}\sigma^2 + \frac{\lambda}{2}\theta^2$.
   - **Question:** Prove that $\nabla \mathrm{PR}(\theta_{\mathrm{PS}}) = \varepsilon(\mu_0 + \varepsilon \theta_{\mathrm{PS}} - \theta_{\mathrm{PS}})$. Under what conditions on $\varepsilon$ and $\lambda$ does $\theta_{\mathrm{PS}} > \theta_{\mathrm{PO}}$? Why does Repeated Risk Minimization systematically over-intervene in Barnesian self-fulfilling regimes?
   - **Guidance:** Compute the derivative $\frac{d\mathrm{PR}}{d\theta} = -(1 - \varepsilon)(\mu_0 - (1 - \varepsilon)\theta) + \lambda \theta$. Evaluate this derivative at $\theta_{\mathrm{PS}} = \frac{\mu_0}{1 + \lambda - \varepsilon}$. Show that the residual gradient is strictly positive when $\varepsilon > 0$ and $\lambda > 0$, proving that performative risk decreases by reducing $\theta$ below $\theta_{\mathrm{PS}}$ toward $\theta_{\mathrm{PO}}$.

4. **PAV Optimism Debiasing on Finite Datasets:**
   Why does the in-sample Brier resolution $\mathrm{RES}_{\mathrm{CORP}} = \frac{1}{n}\sum (\hat{\pi}_i - \bar{y})^2$ possess a positive finite-sample bias? Explain why split-half cross-validation ([`GovernanceState::evaluate_corp_pav_optimism_cv`](crates/stochology/src/governance.rs#L833-L882)) removes label-leakage bias without destroying isotonic monotonicity.
   - **Guidance:** Because the PAV step pools outcomes based on their observed values $y_i$, the fitted value $\hat{\pi}_i$ is correlated with $y_i$ even if the true underlying forecast has zero discriminative power. Interleaving odd and even observations ensures that the isotonic transformation computed on one fold is evaluated on independent realization labels, isolating the true out-of-sample calibration deficit.
