# 8. Severe Testing, Multiplicity, Martingales (e-Processes), and Conformal Prediction

> **Three tempos.** **Detachment:** set aside one-shot significance tests and fixed-sample intervals read after peeking. **Abstraction:** a claim deserves trust only if it survived a test that could have refuted it, and evidence must stay valid when monitored continuously. **Return:** severe testing, multiplicity, betting martingales (e-processes), confidence sequences, and conformal prediction. **Negative proof:** `gates -- --defect peeking`, `gates -- --defect var_underreport`; Full track adds `gates -- --defect multiplicity` (exit `1`).

## The Scientist Who Peeked Every Morning Until the Coin Confessed

Suppose a quantitative researcher wants to prove that a new trading strategy (or clinical drug) beats a fair 50/50 coin toss ($H\_0: p = 0.50$) at the classical significance threshold $\alpha = 0.05$.
- She starts an experiment, planning to collect up to $n = 1\text{,}000$ trades.
- However, instead of waiting until trade $1\text{,}000$, she checks the classical $p$-value on her screen **after every single trade** ($t = 10, 11, 12, \dots$), and the very first morning the $p$-value dips to $p = 0.048 < 0.05$, she yells *"Statistically significant!"*, stops the experiment, and publishes her paper.

If the strategy is actually a **100% useless fair coin ($H\_0$ is true)**, a researcher who peeks after every trade up to $n = 1\text{,}000$ faces a false-positive rate far above $5\%$. Because the standardized random walk $\sum X\_i / \sqrt{t}$ fluctuates back and forth by the Law of the Iterated Logarithm ($\limsup\_{t \to \infty} \frac{S\_t}{\sqrt{2t \ln\ln t}} = 1$), **peeking continuously over $1\text{,}000$ steps inflates the false-positive rate from $5\%$ to nearly $40\%$**. With unlimited peeking ($t \to \infty$), a classical $p$-value crosses $p < 0.05$ with **$100\%$ probability**.

Now combine optional stopping with two other everyday practices:
1. Trying $K = 20$ different feature combinations in the notebook and reporting only the winner (**Multiplicity / $p$-Hacking**).
2. Treating $10\text{,}000$ hourly rows from the *same* $10$ hospitals as if they were $10\text{,}000$ independent hospitals (**Pseudoreplication / Cluster Dependence**).

Algorithmic refutation of optional stopping, multiplicity fishing, pseudoreplication, and production drift rests on five inferential pillars: **Severe Testing**, **Anytime-Valid Betting Martingales (*e-processes*)**, **Cluster Intra-Class Correlation (ICC)**, **Simulation-Based Calibration (SBC)**, and **Conformal Prediction under Distribution Shift**.

<figure class="stoch-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="1160" height="440" viewBox="0 0 1160 440" fill="none" role="img" aria-labelledby="title-fig-13-severe-martingales">
  <title id="title-fig-13-severe-martingales">Multiplicity Inflation, Anytime-Valid Betting Martingales (e-Processes), and Conformal Prediction</title>
  <defs>
  <style>text { font-family: "Inter", system-ui, -apple-system, "Segoe UI", Roboto, sans-serif; } .mono { font-family: "JetBrains Mono", ui-monospace, "Cascadia Code", monospace; }</style>
  </defs>
  <rect x="1" y="1" width="1158" height="438" rx="14" fill="var(--canvas)" stroke="var(--border)" stroke-width="1.5"/>

  <!-- Panel 1: The 5 Statistical Errors & Multiplicity K -->
  <rect x="24" y="24" width="356" height="392" rx="12" fill="var(--elevated)" stroke="var(--sky)" stroke-width="2"/>
  <text x="44" y="54" fill="var(--sky)" class="mono" font-size="11.5" font-weight="800">1 · MULTIPLICITY (K) &amp; THE 5 ERRORS</text>
  <text x="44" y="76" fill="var(--ink)" font-size="14" font-weight="750">Why Hiding K Tried Models Wrecks Science</text>
  <rect x="44" y="98" width="316" height="92" rx="8" fill="var(--canvas)" stroke="var(--rose)" stroke-width="1.5"/>
  <text x="58" y="122" fill="var(--rose)" class="mono" font-size="11" font-weight="800">P(at least 1 false alarm in K tries):</text>
  <text x="58" y="146" fill="var(--ink)" class="mono" font-size="12" font-weight="700">α_family = 1 - (1 - α)^K</text>
  <text x="58" y="170" fill="var(--amber)" font-size="11" font-weight="700">For α = 0.05, K = 20  ⇒  α_family = 64.2% !</text>
  <rect x="44" y="204" width="316" height="142" rx="8" fill="var(--canvas)" stroke="var(--border)"/>
  <text x="58" y="228" fill="var(--sky)" class="mono" font-size="11" font-weight="800">Beyond Type I (α) &amp; Type II (β):</text>
  <text x="58" y="252" fill="var(--ink)" font-size="11">• Type III (Kimball): Right answer to WRONG</text>
  <text x="58" y="270" fill="var(--ink)" font-size="11">  operational question (e.g. annual vs shift R₀).</text>
  <text x="58" y="294" fill="var(--ink)" font-size="11">• Type S (Sign): Claiming effect &gt; 0 when &lt; 0.</text>
  <text x="58" y="318" fill="var(--ink)" font-size="11">• Type M (Magnitude): Winner's curse exaggeration</text>
  <text x="58" y="336" fill="var(--ink)" font-size="11">  in small-sample noisy studies (n_eff too small).</text>
  <text x="44" y="378" fill="var(--muted)" font-size="11.5">• Severe Test (Mayo): High probability of</text>
  <text x="44" y="396" fill="var(--muted)" font-size="11.5">  rejecting claim C Whenever C is false.</text>

  <!-- Panel 2: Betting Martingales (e-Processes) & Ville's Inequality -->
  <rect x="402" y="24" width="356" height="392" rx="12" fill="var(--elevated)" stroke="var(--emerald)" stroke-width="2"/>
  <text x="422" y="54" fill="var(--emerald)" class="mono" font-size="11.5" font-weight="800">2 · BETTING MARTINGALES (e-PROCESSES)</text>
  <text x="422" y="76" fill="var(--ink)" font-size="14" font-weight="750">Anytime-Valid Evidence Under Optional Stopping</text>
  <line x1="432" y1="240" x2="728" y2="240" stroke="var(--muted)" stroke-width="1.5"/>
  <line x1="432" y1="126" x2="728" y2="126" stroke="var(--rose)" stroke-width="1.8" stroke-dasharray="5 3"/>
  <text x="560" y="118" fill="var(--rose)" class="mono" font-size="10.5" font-weight="800">Threshold 1/α = 20.0</text>
  <polyline points="432,220 470,210 510,225 550,190 590,165 630,145 670,115 710,102" stroke="var(--emerald)" stroke-width="2.6" fill="none"/>
  <text x="438" y="202" fill="var(--emerald)" class="mono" font-size="11" font-weight="700">E_t = ∏_{i=1}^t L_i  (E_0 = 1)</text>
  <rect x="422" y="256" width="316" height="136" rx="8" fill="var(--canvas)" stroke="var(--emerald)" stroke-width="1.5"/>
  <text x="436" y="280" fill="var(--emerald)" class="mono" font-size="11" font-weight="800">Ville's Martingale Inequality (1939):</text>
  <text x="436" y="304" fill="var(--ink)" class="mono" font-size="11.5">If E_{H₀}[L_i | Past] ≤ 1, then for ALL times t:</text>
  <text x="436" y="330" fill="var(--amber)" class="mono" font-size="11.5" font-weight="800">P_{H₀}( sup_{t ≥ 1} E_t ≥ 1/α ) ≤ α</text>
  <text x="436" y="356" fill="var(--ink)" font-size="11">You cannot turn $1 into $20 in a fair casino</text>
  <text x="436" y="376" fill="var(--ink)" font-size="11">with probability &gt; 5%, no matter when you quit!</text>

  <!-- Panel 3: Split & Weighted Conformal Prediction -->
  <rect x="780" y="24" width="356" height="392" rx="12" fill="var(--elevated)" stroke="var(--violet)" stroke-width="2"/>
  <text x="800" y="54" fill="var(--violet)" class="mono" font-size="11.5" font-weight="800">3 · CONFORMAL PREDICTION &amp; SHIFT ALGEBRA</text>
  <text x="800" y="76" fill="var(--ink)" font-size="14" font-weight="750">Distribution-Free Finite-Sample Coverage</text>
  <rect x="800" y="98" width="316" height="106" rx="8" fill="var(--canvas)" stroke="var(--violet)" stroke-width="1.5"/>
  <text x="814" y="122" fill="var(--violet)" class="mono" font-size="11" font-weight="800">Split Conformal Interval (Scores s_i = |y_i - f̂(x_i)|):</text>
  <text x="814" y="146" fill="var(--ink)" class="mono" font-size="11">Rank quantile q̂ = s_(⌈(n+1)(1-α)⌉)</text>
  <text x="814" y="170" fill="var(--emerald)" class="mono" font-size="11.5" font-weight="700">C(x_{n+1}) = [f̂(x_{n+1}) - q̂,  f̂(x_{n+1}) + q̂]</text>
  <text x="814" y="192" fill="var(--muted)" font-size="10.5">Guarantees P(Y_{n+1} ∈ C(X_{n+1})) ≥ 1 - α</text>
  <rect x="800" y="218" width="316" height="114" rx="8" fill="var(--canvas)" stroke="var(--border)"/>
  <text x="814" y="242" fill="var(--sky)" class="mono" font-size="11" font-weight="800">When the Production World Drifts:</text>
  <text x="814" y="266" fill="var(--ink)" font-size="11">• Covariate Drift P(X)→Q(X): reweight scores</text>
  <text x="814" y="284" fill="var(--ink)" font-size="11">  by likelihood ratio w(x) = dQ/dP (ExTRA-WCP-T).</text>
  <text x="814" y="308" fill="var(--ink)" font-size="11">• Label Shift P(Y)→Q(Y): solve q_pred = C · π_Q</text>
  <text x="814" y="324" fill="var(--ink)" font-size="11">  via BBSE / MLLS / DFM / OTTER.</text>
  <text x="800" y="366" fill="var(--muted)" font-size="11.5">• SBC Rank Histogram: checks if Bayesian/MC</text>
  <text x="800" y="386" fill="var(--muted)" font-size="11.5">  sampler recovers synthetic truth uniformly.</text>
  </svg>
  <figcaption>Figure 8.1 — The four inferential safeguards of severe testing and sequential governance: (1) Multiplicity inflation across K specifications and the 5 statistical errors; (2) Anytime-valid betting martingales (e-processes) and Ville's Inequality; and (3) Distribution-free Conformal Prediction and production shift algebra.</figcaption>
</figure>

---

## 8.1 Severe Testing, Multiplicity ($K$), and the Five Statistical Errors

### Karl Popper and Deborah Mayo's Severity Principle
A high correlation $R^2$ or a small training error cannot validate a scientific claim $C$ on its own, since an overparameterized curve can fit almost any dataset. Karl Popper and philosopher of statistics Deborah Mayo formulated the **Severity Requirement**:

> **Severe Test Criterion:** Data $D$ provide genuine evidence for a claim $C$ **if and only if** $C$ passes a test procedure $T$ that had a **very high probability of falsifying (rejecting) $C$ if $C$ were false**.

### The Five Canonical Statistical Errors
In introductory textbooks, students learn only **Type I Error** ($\alpha$, false positive: rejecting a true null $H\_0$) and **Type II Error** ($\beta$, false negative: failing to reject a false null $H\_0$). In operational reality, three additional errors cause far more damage:
1. **Type III Error (A. W. Kimball, 1957):** Computing the *mathematically exact answer to the wrong operational question*, such as predicting **annual** average hospital bed demand with $99\%$ precision when the nursing supervisor has to staff **8-hour shifts** (enforced by Boundary Invariant $\Phi\_5$ in Chapter 20).
2. **Type S Error (Sign Error, Gelman & Tuerlinckx, 2000):** Claiming with "statistical significance" that an intervention's effect is **positive** when the true causal effect is **negative**.
3. **Type M Error (Magnitude / Exaggeration Ratio, Gelman & Carlin, 2014):** When the effective sample size $n\_{\mathrm{eff}}$ is small and noise $\sigma$ is large, the only estimates $\hat{\theta}$ that cross the significance filter $|\hat{\theta}| > 1.96\,\mathrm{SE}$ are those lucky draws where random noise happened to be huge. Consequently, conditioning on "statistically significant" findings exaggerates the true effect size by $3\times$ to $10\times$ (**the Winner's Curse**).

### Multiplicity Inflation Across $K$ Attempted Specifications
Suppose a team tests $K$ independent candidate hypotheses or feature specifications at nominal level $\alpha = 0.05$, when in reality all $K$ candidates are pure noise ($H\_0$ is true for all $K$).
- The probability that a single test *does not* produce a false alarm is $1 - \alpha = 0.95$.
- The probability that **none** of the $K$ independent tests produces a false alarm is $(1 - \alpha)^K$.
- Therefore, the **Family-Wise Error Rate (FWER)**, which measures the probability of getting **at least one** false discovery across the $K$ attempts, is:

$$\alpha\_{\text{family}}(K) = 1 - (1 - \alpha)^K$$

For $\alpha = 0.05$ and $K = 20$ silent attempts, $\alpha\_{\text{family}}(20) = 1 - 0.95^{20} = \mathbf{64.15\%}$. To keep the family-wise error rate below $\alpha$, **Šidák / Bonferroni correction** requires dividing the per-test threshold by $K$ ($\alpha\_k = \alpha / K$) or multiplying the required *e-value* threshold by $K$ (which is why Chapter 17's `PROHIBITION_4` requires declaring `specifications_tried_k` and `discarded_findings` in every YAML Card).

---

## 8.2 Testing by Betting: Martingales, *e-Processes* ($E\_t$), and Ville's Inequality

To monitor a live model or sequential experiment continuously over time $t = 1, 2, 3, \dots$ **without invalidating the false-positive guarantee when peeking or stopping early**, game-theoretic statistics (Jean Ville, 1939; Glenn Shafer; Aaditya Ramdas) replaces fixed-sample $p$-values with **Testing by Betting**:

### Definition 8.1 (*e-Value* and Cumulative *e-Process*)
An **e-variable** (evidence variable) at step $i$ is a non-negative random variable $L\_i \ge 0$ whose expected value under the null hypothesis $H\_0$ is at most $1$:

$$\mathbb{E}\_{H\_0}\!\left[L\_i \;\middle|\; \mathcal{F}\_{i-1}\right] \le 1$$

Think of $L\_i$ as the one-step wealth multiplier of a fair ticket in a casino where the house ($H\_0$) gives you zero edge: if you enter round $i$ with $\$1$, your expected wealth after round $i$ under $H\_0$ is at most $\$1$.

If we reinvest our wealth round after round, starting from initial capital $E\_0 = 1$, our cumulative wealth after $t$ observations is the product (**cumulative *e-process***):

$$E\_t = \prod\_{i=1}^t L\_i = E\_{t-1} \cdot L\_t$$

For example, if each step $i$ yields a bounded score $S\_i \in [-1, 1]$ with $\mathbb{E}\_{H\_0}[S\_i \mid \mathcal{F}\_{i-1}] \le 0$ (and positive expectation under the alternative $H\_1$), betting a fraction $\lambda\_i \in [0, 1)$ of our current wealth produces the betting factor $L\_i = 1 + \lambda\_i S\_i \ge 0$, or in likelihood-ratio form $L\_i = \frac{p\_{H\_1}(Y\_i)}{p\_{H\_0}(Y\_i)}$.

### Ville's Inequality (Why You Can Peek at $E\_t$ Whenever You Want)
Because $\mathbb{E}\_{H\_0}[E\_t \mid \mathcal{F}\_{t-1}] = E\_{t-1}\,\mathbb{E}\_{H\_0}[L\_t \mid \mathcal{F}\_{t-1}] \le E\_{t-1}$, the sequence $\{E\_t\}\_{t \ge 0}$ is a **non-negative supermartingale** starting at $E\_0 = 1$.

In 1939, Jean Ville proved a generalization of Markov's Inequality that holds simultaneously across the **entire infinite trajectory** $t = 1, 2, \dots, \infty$:

$$P\_{H\_0}\!\left(\sup\_{t \ge 1} E\_t \ge \frac{1}{\alpha}\right) \le \alpha$$

> **Intuition of Ville's Inequality:** In a fair casino ($H\_0$), if you walk in with $E\_0 = \$1$, the probability that your bankroll **ever** touches $\$20$ ($1/\alpha$ for $\alpha = 0.05$) at *any* moment in your life (even if you peek after every single spin of the wheel and quit the instant you hit $\$20$) is **at most $1/20 = 5\%$**.
> Conversely, if the null hypothesis $H\_0$ is **false** and the alternative $H\_1$ is true ($\mathbb{E}\_{H\_1}[\ln L\_i] > 0$), then $E\_t = \exp(\sum\_{i=1}^t \ln L\_i)$ grows **exponentially** to $+\infty$, crossing $1/\alpha = 20$ rapidly.

This is why the Stochology Protocol (Chapters 16 and 17) uses cumulative *e-processes* $E\_t = \prod L\_i$ as its official sequential evidence engine.

---

## 8.3 Cluster Dependence (Intra-Class Correlation $\rho$) and Simulation-Based Calibration (SBC)

### Why Clustered Rows Are Not Independent: Variance Components and ICC ($\rho$)
Suppose we collect $n = 1\text{,}000$ observations grouped into $G = 10$ clusters of size $m = 100$ each (e.g., $100$ flights at each of $10$ airports, or $100$ patients in each of $10$ hospitals).

A two-level random-intercept model writes the outcome of unit $j$ inside cluster $g$ as:

$$Y\_{gj} = \mu + u\_g + e\_{gj}$$

where $u\_g \sim (0, \sigma\_b^2)$ is the **between-cluster shock** shared by every row in cluster $g$ (e.g., a snowstorm at Airport $g$), and $e\_{gj} \sim (0, \sigma\_w^2)$ is the **within-cluster individual noise**. The covariance between two different rows $j \ne k$ inside the *same* cluster $g$ equals the between-cluster variance:

$$\mathrm{Cov}(Y\_{gj}, Y\_{gk}) = \mathrm{Cov}(u\_g + e\_{gj}, \; u\_g + e\_{gk}) = \mathrm{Var}(u\_g) = \sigma\_b^2 > 0$$

Dividing this shared covariance by the total variance $\mathrm{Var}(Y\_{gj}) = \sigma\_b^2 + \sigma\_w^2$ gives the **Intra-Class Correlation (ICC)** $\rho \in [0, 1]$:

$$\rho = \frac{\sigma\_b^2}{\sigma\_b^2 + \sigma\_w^2}$$

Now compute the variance of the grand average $\bar{Y} = \frac{1}{G}\sum\_{g=1}^G \bar{Y}\_g$ across all $n = Gm$ observations:
- Each cluster average $\bar{Y}\_g = \mu + u\_g + \frac{1}{m}\sum\_{j=1}^m e\_{gj}$ has variance $\sigma\_b^2 + \frac{\sigma\_w^2}{m}$.
- Averaging across $G$ independent clusters divides by $G$:

$$\mathrm{Var}(\bar{Y}) = \frac{1}{G}\left(\sigma\_b^2 + \frac{\sigma\_w^2}{m}\right) = \frac{\sigma\_b^2 + \sigma\_w^2}{G m}\left[1 + (m - 1)\rho\right] = \frac{\sigma\_{\text{total}}^2}{n} \cdot \underbrace{\left[1 + (m - 1)\rho\right]}\_{\text{Design Effect } \mathrm{DEFF}}$$

The factor $\mathrm{DEFF} = 1 + (m - 1)\rho$ shows how quickly cluster dependence erodes precision:
- Even a small intra-cluster correlation $\rho = 0.05$ with cluster size $m = 100$ produces $\mathrm{DEFF} = 1 + 99(0.05) = 5.95$, shrinking your **effective sample size** from $n = 1\text{,}000$ down to $n\_{\mathrm{eff}} = \frac{n}{1 + (m-1)\rho} = \frac{1000}{5.95} \approx \mathbf{168}$.
- In Chapter 20 ($\Phi\_2$), we extend this derivation to unequal cluster sizes $\tilde{m} = \bar{m}(1 + \mathrm{CV}^2)$ (**Moulton-Campbell Correction**) and tail threshold exceedances $\rho\_I(p)$.

### Simulation-Based Calibration (SBC, Cook, Gelman & Rubin / Talts et al.)
Before running a Bayesian or Monte Carlo inference engine on real data, we can verify that the inference code itself is unbiased through a three-step synthetic loop:
1. Draw a synthetic ground-truth parameter from the prior: $\theta^{\text{true}} \sim \pi(\theta)$.
2. Simulate a synthetic dataset from the forward model: $y^{\text{sim}} \sim p(y \mid \theta^{\text{true}})$.
3. Run your inference algorithm on $y^{\text{sim}}$ to obtain $M$ posterior draws $\{\theta^{(1)}, \dots, \theta^{(M)}\}$, and count the **rank** $r \in \{0, 1, \dots, M\}$ of the true parameter among the posterior draws:
   $$r = \sum\_{m=1}^M \mathbf{1}\{\theta^{(m)} < \theta^{\text{true}}\}$$

**Theorem (Talts et al., 2018):** If the inference algorithm is mathematically exact, the rank $r$ across repeated synthetic trials is **strictly uniformly distributed** on $\{0, 1, \dots, M\}$.
- If the rank histogram is **$\cup$-shaped** (piling up at the extremes $0$ and $M$), the posterior intervals are **too narrow (overconfident)**.
- If the rank histogram is **$\cap$-shaped** (piling up in the middle), the posterior is **underconfident**.
- If it is **tilted left or right**, the estimator is **systematically biased**.
Filter 3 of the Eliminatory Funnel (Chapter 12) enforces this uniformity via a $\chi^2$ goodness-of-fit test on the SBC rank histogram.

---

## 8.4 Distribution-Free Conformal Prediction and Shift Algebra

### Split Conformal Prediction (Guaranteed Finite-Sample Coverage)
We can wrap any point predictor $\hat{f}(x)$ in a prediction interval $\hat{C}(X\_{n+1})$ that contains the true future outcome $Y\_{n+1}$ at least $1 - \alpha$ (say $90\%$) of the time **without assuming Gaussian residuals or a correctly specified model**.

Vladimir Vovk and colleagues showed that **exchangeability** (where every permutation of the calibration points $(X\_1, Y\_1), \dots, (X\_n, Y\_n)$ and the test point $(X\_{n+1}, Y\_{n+1})$ is equally likely) suffices:
1. On a held-out calibration set of $n$ points, compute the **non-conformity residuals** $s\_i = |Y\_i - \hat{f}(X\_i)|$ for $i = 1, \dots, n$.
2. Because the future test residual $s\_{n+1} = |Y\_{n+1} - \hat{f}(X\_{n+1})|$ is exchangeable with $s\_1, \dots, s\_n$, its rank among all $n + 1$ residuals is uniformly distributed on $\{1, 2, \dots, n + 1\}$.
3. Therefore, setting $\hat{q}$ to the $\lceil (n + 1)(1 - \alpha) \rceil$-th smallest calibration residual $s\_{(k)}$ guarantees that $s\_{n+1} \le \hat{q}$ with probability at least $1 - \alpha$. Constructing the interval:

$$\hat{C}(x\_{n+1}) = \left[\hat{f}(x\_{n+1}) - \hat{q}, \quad \hat{f}(x\_{n+1}) + \hat{q}\right] \implies P\!\left(Y\_{n+1} \in \hat{C}(X\_{n+1})\right) \ge 1 - \alpha$$

### What Happens When the Production Distribution Shifts? (Covariate vs. Label Shift)
In live deployment (Chapter 21), the target environment $Q(X, Y)$ often diverges from the training environment $P(X, Y)$ in one of two structured ways (corresponding to the two ways of factoring $P(X, Y)$):

1. **Covariate Drift ($P(X, Y) = P(X)P(Y \mid X)$ where $Q(X) \ne P(X)$ but $Q(Y \mid X) = P(Y \mid X)$):**
   The input demographics or weather distribution $Q(X)$ changes, while the physical conditional law $P(Y \mid X)$ stays invariant. Ryan Tibshirani et al. (2019) proved that weighting each calibration score $s\_i$ by the density ratio $w(X\_i) = \frac{dQ}{dP}(X\_i)$ (**Weighted Conformal Prediction**) restores coverage, and Lihua Lei & Emmanuel Candès proved that under an estimated tilt weight $\hat{w}$, coverage degrades by at most the Total Variation distance $d\_{\mathrm{TV}}$ (`ExTRA-WCP-T` in Chapter 21):
   $$P\_Q\!\left(Y\_{n+1} \in \hat{C}(X\_{n+1})\right) \ge 1 - \alpha - d\_{\mathrm{TV}}$$
2. **Label Shift ($P(X, Y) = P(Y)P(X \mid Y)$ where $Q(Y) \ne P(Y)$ but $Q(X \mid Y) = P(X \mid Y)$):**
   In causal diagnostic problems ($Y \to X$, where disease $Y$ causes symptoms $X$), an epidemic can jump the base rate $Q(Y = 1)$ from $5\%$ to $40\%$ while the symptom profile given the disease $P(X \mid Y)$ stays identical.
   If we let $C\_{jk} = P(\hat{Y} = j \mid Y = k)$ be the source **confusion matrix** and $\mathbf{q}\_{\text{pred}} = Q(\hat{Y} = j)$ be the vector of predicted class frequencies observed in the new target environment, the Law of Total Probability gives the linear system:
   $$\mathbf{q}\_{\text{pred}} = C \, \boldsymbol{\pi}\_Q \implies \hat{\boldsymbol{\pi}}\_Q = C^{-1} \mathbf{q}\_{\text{pred}}$$
   Solving this equation (via `BBSE`, `MLLS/SLD`, `DFM`, or `OTTER` Optimal Transport in Chapter 21) recovers the new target prevalence $\boldsymbol{\pi}\_Q$ **without needing a single new ground-truth label in production**.

### Step-by-Step Proof of Jean Ville's (1939) Maximal Inequality and Model-Free `GREE` VaR/ES E-Backtesting

Why does a nonnegative test martingale $E\_t = \prod\_{i=1}^t L\_i$ ($E\_0 = 1$, $\mathbb{E}\_0[E\_t \mid \mathcal{F}\_{t-1}] \le E\_{t-1}$) allow you to peek after every single observation without inflating your false-positive rate?

1. **Proof via Doob's Stopped Supermartingale:**
   Fix any threshold $c = 1/\alpha > 1$ (say $c = 20$ for $\alpha = 0.05$), and define the stopping time $\tau = \inf\{t \ge 1 : E\_t \ge c\}$ (the very first moment the evidence crosses $c$). For any finite horizon $T$, the stopped process $E\_{t \wedge \tau}$ is still a nonnegative supermartingale with initial value $E\_0 = 1$, so by Optional Stopping and Markov's Inequality:
   $$1 = E\_0 \ge \mathbb{E}\_0\!\left[E\_{T \wedge \tau}\right] \ge \mathbb{E}\_0\!\left[E\_\tau \, \mathbf{1}\{\tau \le T\}\right] \ge c \cdot P\_0(\tau \le T)$$
   Dividing by $c = 1/\alpha$ and letting $T \to \infty$ by monotone convergence yields **Ville's Inequality**:
   $$P\_0\!\left(\exists t \ge 1 : E\_t \ge \frac{1}{\alpha}\right) \le \alpha$$

2. **Model-Free `GREE` E-Backtesting for Value-at-Risk and Expected Shortfall (`VarEsEBacktest`):**
   Following Wang, Wang & Ziegel (2022/2025), we can audit any reported $(1-\alpha)$-$\mathrm{VaR}$ forecast $v\_t$ and Expected Shortfall forecast $s\_t > v\_t$ against realized losses $L\_t$ without distributional assumptions using the elementaryExpected Shortfall e-statistic:
   $$e\_t^{\mathrm{ES}}(L\_t; v\_t, s\_t) = \frac{(L\_t - v\_t)\_+}{\alpha(s\_t - v\_t)}$$
   Under a truthful risk report ($\mathbb{E}[(L\_t - v\_t)\_+] \le \alpha(s\_t - v\_t)$), $\mathbb{E}[e\_t^{\mathrm{ES}}] \le 1$, so the betting martingale $M\_t = \prod\_{i=1}^t \bigl(1 - \lambda\_i + \lambda\_i e\_i^{\mathrm{ES}}\bigr)$ is a valid test supermartingale (`--defect var_underreport` in Gate `G6`).

### Interactive Laboratory (`/lab`)

Use the interactive Ville martingale vs. optional-stopping laboratory below to compare `aGRAPA`/`GRO` anytime-valid evidence $E\_t$ against the false-positive inflation of repeated $z$-test peeking:

<div class="stoch-lab" data-lab="ville-savi-conformal" data-chapter="08-severe-testing-martingales-conformal.html" data-title="Chapter 8 Lab: Ville Martingale e-Process (E_t) vs. Optional-Stopping Peeking Inflation" data-cli="cargo run --release --bin gates -- --defect peeking"></div>


---

## 8.5 Many Hypotheses at Once: e-BH (`EBhMultiplicityAudit`)

Section 8.1 counted attempts with $K$; the existing `EvidenceProcess` divides the running maximum by $K$, a Bonferroni rule that controls the chance of *any* false rejection. When an analyst tests many hypotheses and wants to keep the share of false discoveries low instead, Wang and Ramdas (2022, *JRSS-B*, informative) give a rule that needs no assumption about how the e-values depend on each other. Sort the e-values $e\_{(1)} \ge \dots \ge e\_{(K)}$ and reject the $\hat{k}$ largest, where

$$\hat{k} = \max\Bigl\{k : e\_{(k)} \ge \frac{K}{\alpha k}\Bigr\}.$$

The false discovery rate is then at most $\alpha$. BH on p-values needs an extra harmonic factor $H\_K = \sum\_{i \le K} 1/i$ for the same guarantee (Benjamini and Yekutieli 2001); e-BH does not. Every hypothesis e-BH does not reject is named as a discarded finding, so Prohibition 4 holds by construction.

**Measured** on `nyc_flights_2013.csv` and `nyc_flights_2023.csv`: one hypothesis per carrier that flew from New York in both August–December windows ($K = 11$). The independent unit is the **day**, not the flight ($\Phi\_2$): flights of one day share weather and congestion. For every calendar day $d$ the carrier flew in both years, $X\_d = r\_{2023}(d) - r\_{2013}(d)$ is the change in its share of arrivals more than 15 minutes late; pairing the same calendar day removes the shared seasonality. The null is "no change" ($\mathbb{E}[X\_d \mid \text{earlier days}] = 0$), tested with GRAPA betting e-values. A first design that treated each flight as a unit was discarded after an independent audit showed its e-values were driven by within-season dependence (`docs/research/pillar-expansion.md`, D7).

~~~text
  [G6] A: e-BH over K=11 carriers (nyc_flights 2013 vs 2023, unit = paired calendar day): 2 rejected ["9E", "HA"], 9 discarded (Benjamini-Yekutieli on 1/e rejects 1; naive peeking would reject 5)
~~~

The naive analyst runs each stream as if it were the only one ($K = 1$) and stops at the first crossing of $1/\alpha$. On a **simulated** global null (seed 2026) that rule discovers something false in half of the replications:

~~~text
NEGATIVE PROOF [multiplicity] BLOCKED SUCCESSFULLY: peeking 20 null e-processes at 1/alpha with K=1 each gives FDR=0.515 > 0.05 (e-BH FDR=0.000, simulated, seed 2026)
~~~

## Computational Audit in Rust

Every method introduced in this chapter is implemented in pure Rust across the workspace:
- **Multiplicity & *e-Processes* (`crates/stochology-core/src/card/`, `crates/stochology-siga/src/roadmap/`):** Audits `attempt_counter_k`, `findings` (`registered`, `used`, `unused`), `e_process_multiplicity`, and Ville's threshold $E\_t / K \ge 1 / \alpha$.
- **Cluster ICC & Moulton-Campbell (`crates/stochology-core/src/invariants.rs`):** Computes $\tilde{m}$, exceedance ICC $\rho\_I(p)$, and $n\_{\mathrm{eff}}(p)$.
- **SBC Rank Uniformity (`crates/stochology-siga/src/funnel/`):** Evaluates the $\chi^2$ rank statistic in Filter 3.
- **Weighted Conformal (`ExTRA-WCP-T`) & Label Shift (`BBSE`, `MLLS`, `DFM`, `OTTER`) (`crates/stochology-governance/src/drift/`):** Executes live drift adaptation and circuit breaking.

Run Labs 4, 5, and 6 to verify these mechanisms:

~~~bash
cargo run --release --bin labs -- --invariants
cargo run --release --bin labs -- --siga
cargo run --release --bin labs -- --governance
~~~

## Exercises

1. **Multiplicity Inflation Calculation:** If a student runs $K = 14$ independent regressions at $\alpha = 0.05$ on pure white noise and reports only the best $p$-value, what is the probability $\alpha\_{\text{family}} = 1 - (1 - 0.05)^{14}$ of getting at least one false positive? What Bonferroni threshold $\alpha / K$ must each test pass to keep the family error $\le 0.05$?
2. **Ville's Inequality and *e-Processes*:** Suppose a sequential test produces likelihood ratios $L\_1 = 2.0, \; L\_2 = 1.8, \; L\_3 = 1.5, \; L\_4 = 4.0$. Compute the cumulative *e-process* trajectory $E\_1, E\_2, E\_3, E\_4$ ($E\_t = \prod\_{i=1}^t L\_i$). At which step $t$ does $E\_t$ cross the $\alpha = 0.05$ Ville threshold $1/\alpha = 20.0$?
3. **Split Conformal Quantile by Hand:** Suppose a held-out calibration set of $n = 9$ observations produces sorted absolute residuals $s\_{(1..9)} = [0.2, 0.5, 0.8, 1.1, 1.4, 1.7, 2.1, 2.8, 4.5]$. For target coverage $1 - \alpha = 0.80$ ($\alpha = 0.20$), compute the index $k = \lceil (9 + 1)(0.80) \rceil = 8$, find $\hat{q} = s\_{(8)}$, and write the $80\%$ conformal prediction interval when $\hat{f}(x\_{10}) = 10.0$.

## Summary

- **Mayo's Severity Principle** and `PROHIBITION_4` require recording the specification counter $K \ge 1$ and penalizing evidence across all attempted models.
- **Ville's Inequality and `aGRAPA`/`GRO` betting martingales** ($E\_t = \prod\_{i=1}^t (1 + \lambda\_i X\_i)$) provide anytime-valid inference and `SAVI` confidence sequences that remain valid under continuous operational monitoring (`--defect peeking`).
- **Distribution-free conformal prediction** (`ExTRA-WCP-T` and online `ACI`) and **model-free `GREE` VaR/ES e-backtesting** (`VarEsEBacktest`) audit tail coverage and risk reports without parametric normality assumptions.
