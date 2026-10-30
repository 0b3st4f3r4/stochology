# 3. Random Variables, Expectations, Quantiles, and Convexity

> **Three tempos.** **Detachment:** set aside summarizing a random quantity by its mean. **Abstraction:** when consequences curve, the average of outcomes is not the outcome at the average (dominance of curvature). **Return:** random variables, quantiles, and Jensen's inequality. **Negative proof:** none in this chapter; the tools here feed the proofs of later chapters.

## The River With an Average Depth of One Meter

A civil engineer is asked whether an infantry platoon, whose soldiers have an average height of $1.75\text{ m}$ and carry radio packs strapped at $1.40\text{ m}$, can safely wade across a river whose hydrographic bulletin reports:

> *"Mean depth across the 100-meter crossing: $\bar{x} = 1.00\text{ m}$."*

Relying on the reported mean, the engineer approves the crossing. Halfway across, the riverbed drops into a 20-meter navigational channel that is $2.40\text{ m}$ deep. Every radio is flooded and the platoon is swept downstream.

A true mathematical statement ($\bar{x} = 1.00\text{ m}$) produced an operational failure because two operations happened silently between the river and the decision:

1. **Collapsing a Distribution into a Single Number:** The river's depth is a **distribution** across space rather than a single scalar ($80\text{ m}$ of $0.65\text{ m}$ shallows and $20\text{ m}$ of $2.40\text{ m}$ channel: $0.80 \times 0.65 + 0.20 \times 2.40 = 1.00\text{ m}$).
2. **Evaluating a Nonlinear Payoff at the Mean:** The operational damage function $f(x)$ (radio destruction and drowning) is **not linear** in depth $x$. Going from $0.30\text{ m}$ to $0.60\text{ m}$ causes zero damage ($f(0.65) = 0$), whereas crossing $1.40\text{ m}$ causes $100\%$ equipment loss ($f(2.40) = 100$).

Compare the two evaluations:
- **Damage at the average depth:** $f(\mathbb{E}[X]) = f(1.00\text{ m}) = 0\%$ loss.
- **Average damage across the actual river:** $\mathbb{E}[f(X)] = 0.80 \times f(0.65) + 0.20 \times f(2.40) = 0.80(0) + 0.20(100) = 20\%$ loss (and $100\%$ failure for anyone walking the full crossing).

Resolving this discrepancy requires four mathematical primitives:
1. **Random Variables, CDFs, and Quantiles** (how a full distribution is represented in memory);
2. **Moments, Variance, Covariance, and Correlation** (how dispersion and co-movement are measured);
3. **Convexity and Secant Geometry** (why curves bend averages);
4. **Distances Between Distributions** ($\mathrm{CRPS}$ vs. Wasserstein-1 $W\_1$ and Lipschitz sensitivity).

<figure class="stoch-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="1160" height="440" viewBox="0 0 1160 440" fill="none" role="img" aria-labelledby="title-fig-01-foundations">
  <title id="title-fig-01-foundations">From Random Variables and CDFs to Convexity and Distribution Distances (CRPS vs. Wasserstein-1)</title>
  <defs>
  <style>text { font-family: "Inter", system-ui, -apple-system, "Segoe UI", Roboto, sans-serif; } .mono { font-family: "JetBrains Mono", ui-monospace, "Cascadia Code", monospace; }</style>
  </defs>
  <rect x="1" y="1" width="1158" height="438" rx="14" fill="var(--canvas)" stroke="var(--border)" stroke-width="1.5"/>

  <!-- Panel 1: Empirical CDF & Quantiles -->
  <rect x="24" y="24" width="356" height="392" rx="12" fill="var(--elevated)" stroke="var(--sky)" stroke-width="2"/>
  <text x="44" y="54" fill="var(--sky)" class="mono" font-size="11.5" font-weight="800">1 · EMPIRICAL CDF F̂_n(x) &amp; QUANTILES</text>
  <text x="44" y="76" fill="var(--ink)" font-size="14" font-weight="750">Order Statistics: x_(1) ≤ x_(2) ≤ … ≤ x_(n)</text>
  <line x1="56" y1="300" x2="348" y2="300" stroke="var(--muted)" stroke-width="1.5"/>
  <line x1="56" y1="110" x2="56" y2="300" stroke="var(--muted)" stroke-width="1.5"/>
  <path d="M 56 300 L 96 300 L 96 262 L 146 262 L 146 205 L 210 205 L 210 148 L 296 148 L 296 115 L 344 115" stroke="var(--sky)" stroke-width="2.8" fill="none"/>
  <line x1="56" y1="205" x2="146" y2="205" stroke="var(--amber)" stroke-width="1.4" stroke-dasharray="4 3"/>
  <line x1="146" y1="205" x2="146" y2="300" stroke="var(--amber)" stroke-width="1.4" stroke-dasharray="4 3"/>
  <text x="64" y="198" fill="var(--amber)" class="mono" font-size="10.5" font-weight="700">p = 0.50 → q_50 (Median)</text>
  <text x="220" y="290" fill="var(--rose)" class="mono" font-size="10.5" font-weight="700">q_95 (Tail)</text>
  <text x="44" y="340" fill="var(--ink)" class="mono" font-size="11.5" font-weight="700">F(x) = P(X ≤ x) ∈ [0, 1]</text>
  <text x="44" y="364" fill="var(--muted)" font-size="11.5">• Mean E[X] is pulled by extreme upper tail</text>
  <text x="44" y="384" fill="var(--muted)" font-size="11.5">• Quantiles q_α slice rank positions directly</text>

  <!-- Panel 2: Geometry of Covariance & Correlation -->
  <rect x="402" y="24" width="356" height="392" rx="12" fill="var(--elevated)" stroke="var(--emerald)" stroke-width="2"/>
  <text x="422" y="54" fill="var(--emerald)" class="mono" font-size="11.5" font-weight="800">2 · COVARIANCE AS VECTOR GEOMETRY</text>
  <text x="422" y="76" fill="var(--ink)" font-size="14" font-weight="750">ρ_{X,Y} = cos(θ) Between Centered Vectors</text>
  <line x1="450" y1="270" x2="710" y2="270" stroke="var(--muted)" stroke-width="1.5"/>
  <line x1="450" y1="270" x2="680" y2="140" stroke="var(--emerald)" stroke-width="2.8"/>
  <line x1="450" y1="270" x2="700" y2="230" stroke="var(--sky)" stroke-width="2.8"/>
  <path d="M 520 230 A 75 75 0 0 1 530 256" stroke="var(--amber)" stroke-width="2" fill="none"/>
  <text x="540" y="246" fill="var(--amber)" class="mono" font-size="12" font-weight="800">θ</text>
  <text x="640" y="132" fill="var(--emerald)" class="mono" font-size="11.5" font-weight="700">x̃ = X - E[X]</text>
  <text x="650" y="218" fill="var(--sky)" class="mono" font-size="11.5" font-weight="700">ỹ = Y - E[Y]</text>
  <text x="422" y="322" fill="var(--ink)" class="mono" font-size="11.5" font-weight="700">ρ_{X,Y} = ⟨x̃, ỹ⟩ / (‖x̃‖ · ‖ỹ‖) ∈ [-1, 1]</text>
  <text x="422" y="350" fill="var(--muted)" font-size="11.5">• θ = 0°  → ρ = +1 (perfect linear lockstep)</text>
  <text x="422" y="370" fill="var(--muted)" font-size="11.5">• θ = 90° → ρ =  0 (orthogonal / uncorrelated)</text>
  <text x="422" y="390" fill="var(--muted)" font-size="11.5">• Cauchy-Schwarz guarantees |ρ_{X,Y}| ≤ 1</text>

  <!-- Panel 3: CRPS (Vertical) vs Wasserstein-1 (Horizontal) -->
  <rect x="780" y="24" width="356" height="392" rx="12" fill="var(--elevated)" stroke="var(--violet)" stroke-width="2"/>
  <text x="800" y="54" fill="var(--violet)" class="mono" font-size="11.5" font-weight="800">3 · DISTANCES BETWEEN DISTRIBUTIONS</text>
  <text x="800" y="76" fill="var(--ink)" font-size="14" font-weight="750">Vertical (CRPS) vs. Horizontal (W₁)</text>
  <line x1="812" y1="286" x2="1104" y2="286" stroke="var(--muted)" stroke-width="1.5"/>
  <line x1="812" y1="116" x2="812" y2="286" stroke="var(--muted)" stroke-width="1.5"/>
  <path d="M 812 286 C 860 284, 890 120, 960 118 L 1100 118" stroke="var(--sky)" stroke-width="2.6" fill="none"/>
  <path d="M 812 286 C 910 284, 950 120, 1030 118 L 1100 118" stroke="var(--rose)" stroke-width="2.6" fill="none"/>
  <line x1="896" y1="196" x2="962" y2="196" stroke="var(--amber)" stroke-width="2.4"/>
  <text x="902" y="186" fill="var(--amber)" class="mono" font-size="10.5" font-weight="800">← W₁ (horizontal) →</text>
  <text x="800" y="322" fill="var(--ink)" class="mono" font-size="11.5" font-weight="700">W₁(P, Q) = ∫₀¹ |F⁻¹(u) - G⁻¹(u)| du</text>
  <text x="800" y="346" fill="var(--ink)" class="mono" font-size="11.5" font-weight="700">CRPS(F, y) = ∫ (F(x) - 1{x ≥ y})² dx</text>
  <text x="800" y="372" fill="var(--muted)" font-size="11.5">• W₁ measures physical earth-moving cost</text>
  <text x="800" y="392" fill="var(--muted)" font-size="11.5">• CRPS scores full predictive CDF against y</text>
  </svg>
  <figcaption>Figure 3.1 — The three foundational building blocks connecting elementary probability to ontological auditing: (1) Empirical CDFs and order-statistic quantiles; (2) Covariance and Pearson correlation as the cosine of centered vectors; and (3) Wasserstein-1 horizontal transport vs. CRPS vertical calibration.</figcaption>
</figure>

---

## 3.1 From Outcomes to Random Variables, CDFs, and Quantiles

Discrete events $A \subseteq \Omega$ ("the storm hits," "the test is positive") are only the starting point; in engineering and science, observations are numerical measurements: wind speed in knots, queue wait time in seconds, financial loss in millions of krone.

### Definition 3.1 (Random Variable and Cumulative Distribution Function)
A **random variable** $X: \Omega \to \mathbb{R}$ is a deterministic rule that assigns a real number $X(\omega)$ to every elementary outcome $\omega \in \Omega$. Its complete probabilistic law is captured by its **Cumulative Distribution Function (CDF)** $F\_X: \mathbb{R} \to [0, 1]$:

$$F\_X(x) = P(\{\omega \in \Omega : X(\omega) \le x\}) = P(X \le x)$$

Every CDF satisfies three properties:
1. **Monotonicity:** If $a \le b$, then $F\_X(a) \le F\_X(b)$ (adding more possible values can never shrink probability).
2. **Boundary limits:** $\lim\_{x \to -\infty} F\_X(x) = 0$ and $\lim\_{x \to +\infty} F\_X(x) = 1$.
3. **Interval probability:** For any interval $(a, b]$, $P(a < X \le b) = F\_X(b) - F\_X(a)$.

### Empirical CDF and Order Statistics (`QuantileSummary`)
When we observe a finite sample of $n$ measurements $x\_1, x\_2, \dots, x\_n$ in Rust, we first sort them in ascending order to obtain the **order statistics**:

$$x\_{(1)} \le x\_{(2)} \le \dots \le x\_{(n)}$$

The **Empirical CDF** $\hat{F}\_n(x)$ is the staircase function that jumps by $1/n$ at each sorted data point:

$$\hat{F}\_n(x) = \frac{1}{n} \sum\_{i=1}^n \mathbf{1}\{x\_i \le x\}$$

Inverting $\hat{F}\_n$ gives the **$\alpha$-quantile** $q\_\alpha = F\_X^{-1}(\alpha) = \inf\{x \in \mathbb{R} : F\_X(x) \ge \alpha\}$ for any probability level $\alpha \in (0, 1)$:
- **$q\_{0.05}$ (5th percentile):** Only $5\%$ of observations fall at or below this value (lower tail).
- **$q\_{0.50}$ (Median):** Exactly half the probability mass lies below and half above.
- **$q\_{0.95}$ (95th percentile):** $95\%$ of observations fall below, and $5\%$ exceed this threshold (upper tail).

### Worked Example 3.1: Why the Median Ignores Catastrophic Outliers while the Mean Explodes
Consider five daily server latency readings (in ms):

$$x = [10, \; 12, \; 14, \; 16, \; 18]$$

- The **sample mean** is $\bar{x} = \frac{10 + 12 + 14 + 16 + 18}{5} = 14.0\text{ ms}$.
- The **sample median** ($q\_{0.50}$, middle element $x\_{(3)}$) is $14.0\text{ ms}$.

Now suppose on Friday the server suffers a memory stall, so the fifth reading is $1\text{,}018\text{ ms}$ instead of $18\text{ ms}$:

$$x' = [10, \; 12, \; 14, \; 16, \; 1\text{,}018]$$

- The **median** $x'\_{(3)}$ is still **$14.0\text{ ms}$**; it depends only on rank order, so a single extreme observation cannot move it past its neighbors.
- The **mean** jumps to $\bar{x}' = \frac{1\text{,}070}{5} = \mathbf{214.0\text{ ms}}$, which describes neither the four normal days ($10$–$16\text{ ms}$) nor the stalled day ($1\text{,}018\text{ ms}$).

---

## 3.2 Expectation, Variance, Covariance, and Pearson Correlation

### Expectation as a Weighted Sum (Linear Operator)
For a discrete random variable taking values $x\_1, \dots, x\_k$ with probabilities $p\_i = P(X = x\_i)$ (or a continuous random variable with density $p(x) = F'\_X(x)$), the **expectation** (or expected value) is the probability-weighted center of mass:

$$\mathbb{E}[X] = \sum\_{i=1}^k x\_i \, p\_i \qquad \text{or} \qquad \mathbb{E}[X] = \int\_{-\infty}^{\infty} x \, p(x)\,dx$$

A central algebraic property of $\mathbb{E}[\cdot]$ is **linearity**: for any constants $a, b \in \mathbb{R}$ and any two random variables $X, Y$ (even if they are strongly dependent),

$$\mathbb{E}[aX + bY] = a\,\mathbb{E}[X] + b\,\mathbb{E}[Y]$$

### Variance and Standard Deviation
How spread out is $X$ around its center $\mu\_X = \mathbb{E}[X]$? If we averaged the raw deviations $\mathbb{E}[X - \mu\_X]$, positive and negative deviations would cancel to zero ($\mathbb{E}[X] - \mu\_X = 0$). Squaring the deviations before averaging gives the **variance**:

$$\mathrm{Var}(X) = \sigma\_X^2 = \mathbb{E}\!\left[(X - \mu\_X)^2\right] = \mathbb{E}[X^2] - (\mathbb{E}[X])^2$$

Because $\mathrm{Var}(X)$ has squared units (e.g., $\text{ms}^2$ or $\text{dollars}^2$), taking its square root restores the original physical units, giving the **standard deviation** $\sigma\_X = \sqrt{\mathrm{Var}(X)}$.

### Covariance and Pearson Correlation as Vector Cosine ($\rho\_{X,Y}$)
Suppose we observe two variables simultaneously on each unit $i \in \{1, \dots, n\}$: for example, $X\_i$ = wind speed and $Y\_i$ = storm surge, or $R\_i \in \{0, 1\}$ = whether unit $i$ was recorded in a database and $Y\_i$ = its true outcome.

Define the **centered vectors** in $\mathbb{R}^n$ by subtracting their sample means:

$$\tilde{x}\_i = x\_i - \bar{x}, \qquad \tilde{y}\_i = y\_i - \bar{y}$$

1. The **sample covariance** is the average dot product of these centered vectors:
   $$\mathrm{Cov}(X, Y) = \frac{1}{n} \sum\_{i=1}^n (x\_i - \bar{x})(y\_i - \bar{y}) = \frac{1}{n}\langle \tilde{\mathbf{x}}, \tilde{\mathbf{y}} \rangle$$
   When $x\_i > \bar{x}$ and $y\_i > \bar{y}$ happen together (or both fall below their means together), the product $(x\_i - \bar{x})(y\_i - \bar{y})$ is positive.
2. The **Pearson correlation coefficient** $\rho\_{X,Y}$ normalizes covariance by the product of the two standard deviations, which is identically the **cosine of the angle $\theta$** between the centered vectors $\tilde{\mathbf{x}}$ and $\tilde{\mathbf{y}}$ in $\mathbb{R}^n$:

$$\rho\_{X,Y} = \frac{\mathrm{Cov}(X,Y)}{\sigma\_X \sigma\_Y} = \frac{\langle \tilde{\mathbf{x}}, \tilde{\mathbf{y}} \rangle}{\|\tilde{\mathbf{x}}\|\_2 \, \|\tilde{\mathbf{y}}\|\_2} = \cos(\theta) \in [-1, 1]$$

By the Cauchy-Schwarz Inequality ($|\langle \tilde{\mathbf{x}}, \tilde{\mathbf{y}} \rangle| \le \|\tilde{\mathbf{x}}\|\_2 \|\tilde{\mathbf{y}}\|\_2$), correlation is strictly bounded between $-1$ and $+1$. This inner-product identity directly explains why non-random missing data ($R\_i$ correlated with $Y\_i$, $\rho\_{R,Y} \ne 0$) biases sample averages regardless of sample size.

---

## 3.3 Nonlinear Functions, Convexity, and the Derivation of Jensen's Inequality

When $f$ is curved, $\mathbb{E}[f(X)]$ almost never equals $f(\mathbb{E}[X])$.

### Definition 3.2 (Convex Function)
A function $f: \mathbb{R} \to \mathbb{R}$ is **convex** if, for every pair of points $x\_1, x\_2 \in \mathbb{R}$ and every weight $\lambda \in [0, 1]$, the straight chord (secant line) connecting $(x\_1, f(x\_1))$ and $(x\_2, f(x\_2))$ lies **on or above** the graph of $f$:

$$\lambda f(x\_1) + (1 - \lambda) f(x\_2) \ge f\!\left(\lambda x\_1 + (1 - \lambda) x\_2\right)$$

If $f$ is twice differentiable, $f$ is convex if and only if its curvature (second derivative) is non-negative everywhere:

$$f''(x) \ge 0 \quad \forall x \in \mathbb{R}$$

(Conversely, if $f''(x) \le 0$, $f$ is **concave**, like $\ln(x)$ or $\sqrt{x}$, and the inequality reverses: $\mathbb{E}[f(X)] \le f(\mathbb{E}[X])$.)

### Step-by-Step Proof of Jensen's Inequality via Supporting Tangent Lines
Let $\mu = \mathbb{E}[X]$. Because a convex function $f$ always curves upward away from its tangent line at $x = \mu$, write the second-order Taylor expansion of $f(X)$ around $\mu$:

$$f(X) = f(\mu) + f'(\mu)(X - \mu) + \frac{1}{2} f''(\xi)(X - \mu)^2$$

for some intermediate point $\xi$ between $\mu$ and $X$. Now take the expectation $\mathbb{E}[\cdot]$ of both sides using linearity:

$$\mathbb{E}[f(X)] = f(\mu) + f'(\mu)\underbrace{\mathbb{E}[X - \mu]}\_{= \, 0} + \frac{1}{2}\mathbb{E}\!\left[f''(\xi)(X - \mu)^2\right]$$

The right-hand side has three terms:
1. $f(\mu) = f(\mathbb{E}[X])$ is the naïve evaluation at the mean.
2. The first-order linear slope term $f'(\mu)\mathbb{E}[X - \mu]$ **vanishes identically** because $\mathbb{E}[X - \mu] = \mu - \mu = 0$.
3. Because $f''(\xi) \ge 0$ and $(X - \mu)^2 \ge 0$, the remaining curvature term is non-negative. Calling the difference **Jensen's Gap** ($\Delta\_J$), we obtain:

$$\Delta\_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X]) \approx \frac{1}{2} f''(\mu)\,\mathrm{Var}(X) \ge 0$$

> **Core Lesson of Jensen's Gap:** Jensen's Gap is proportional to **curvature times variance** ($\frac{1}{2} f''(\mu)\sigma\_X^2$). Whenever your operational cost function has positive curvature ($f'' > 0$, such as hospital queue delays, option payoffs, or hurricane damage $f(v) \propto v^3$) OR your system has large variance ($\sigma\_X^2 \gg 0$), evaluating your formula at the average input $f(\mathbb{E}[X])$ systematically understates the true average cost $\mathbb{E}[f(X)]$.

---

## 3.4 Comparing Full Distributions: CRPS vs. Wasserstein-1 ($W\_1$) and Lipschitz Continuity

Once we refuse to collapse a random variable $X$ into a lone mean, we need a way to measure the error of a probabilistic forecast $F(x)$ or the distance between two distributions $P$ and $Q$.

### 1. Vertical Distance: Continuous Ranked Probability Score ($\mathrm{CRPS}$)
Suppose a forecaster issues a full predictive CDF $F(x) = P(X \le x)$, and nature subsequently reveals the actual scalar outcome $y \in \mathbb{R}$. Nature's outcome can itself be written as a step-function CDF $\mathbf{1}\{x \ge y\}$ that jumps from $0$ to $1$ at $x = y$.

The **Continuous Ranked Probability Score** (`empirical_crps` in `crates/stochology-core/src/layers/`) integrates the squared vertical gap between the forecast CDF $F(x)$ and nature's step function across the entire real line:

$$\mathrm{CRPS}(F, y) = \int\_{-\infty}^{\infty} \left(F(x) - \mathbf{1}\{x \ge y\}\right)^2 dx = \mathbb{E}\_F[|X - y|] - \frac{1}{2}\mathbb{E}\_F[|X - X'|]$$

where $X, X'$ are independent draws from $F$. The right-hand representation splits into two parts:
- $\mathbb{E}\_F[|X - y|]$ penalizes the forecast if its samples are far from the true outcome $y$ (**accuracy**).
- $-\frac{1}{2}\mathbb{E}\_F[|X - X'|]$ rewards the forecast for spreading its probability mass honestly to reflect true uncertainty (**sharpness and calibration**). If $F$ collapses to a deterministic point forecast $\hat{y}$, the second term vanishes and $\mathrm{CRPS}$ reduces to the Mean Absolute Error $|\hat{y} - y|$.

### 2. Horizontal Distance: Wasserstein-1 ($W\_1$, Earth Mover's Distance) and Lipschitz Functions
Suppose we need to compare two full probability distributions $P$ (with CDF $F$) and $Q$ (with CDF $G$), such as the distribution of commuter behavior *before* vs. *after* a navigation app changes its routing model.

If we think of $P$ and $Q$ as two piles of earth of total mass $1$, the **Wasserstein-1 distance** $W\_1(P, Q)$ (`empirical_wasserstein_1` in `crates/stochology-core/src/layers/`) is the minimum physical work (mass $\times$ horizontal distance moved) required to reshape pile $P$ into pile $Q$. In one dimension, Vallender's theorem proves that this equals the area between their quantile functions (or equivalently between their CDFs):

$$W\_1(P, Q) = \int\_0^1 \left|F^{-1}(u) - G^{-1}(u)\right| du = \int\_{-\infty}^{\infty} |F(x) - G(x)|\,dx$$

The $W\_1$ metric plays a central role in stochastic control and performativity through **Kantorovich-Rubinstein Duality**. A function $h: \mathbb{R} \to \mathbb{R}$ is called **$L$-Lipschitz** if its slope never exceeds $L$:

$$|h(x) - h(x')| \le L \cdot |x - x'| \quad \forall x, x'$$

Kantorovich-Rubinstein duality states that for any $L$-Lipschitz loss function $h$, the change in expected loss when the world shifts from distribution $P$ to distribution $Q$ is bounded by:

$$\left|\mathbb{E}\_{X \sim P}[h(X)] - \mathbb{E}\_{X \sim Q}[h(X)]\right| \le L \cdot W\_1(P, Q)$$

When a deployed model shifts its parameter by $\|\theta - \theta'\|$, and the world's distribution shifts by $W\_1(\mathcal{D}(\theta), \mathcal{D}(\theta')) \le \varepsilon \|\theta - \theta'\|$, Kantorovich-Rubinstein duality bounds whether the feedback loop between Agent and World converges to stability or diverges.

### 3. Foundational Derivation: Tail-Integral Identity and Exact $\mathcal{O}(n \log n)$ Empirical $\mathrm{CRPS}$

Two foundational identities connect quantiles, expectations, and proper scoring rules throughout the Stochology codebase:

1. **Darboux / Quantile-Inversion Identity ($\mathbb{E}[X] = \int\_0^1 F\_X^{-1}(u)\,du$):**
   For any non-negative random variable $X \ge 0$, writing $X = \int\_0^\infty \mathbf{1}\{X > t\}\,dt$ and taking expectations via Fubini's theorem yields the **survival-area identity**:
   $$\mathbb{E}[X] = \int\_0^\infty (1 - F\_X(t))\,dt = \int\_0^1 F\_X^{-1}(u)\,du$$
   Subtracting two CDFs $F$ and $G$ immediately proves **Vallender's Theorem (1973)**: the vertical area between two CDFs $\int\_{-\infty}^\infty |F(x) - G(x)|\,dx$ and the horizontal area between their quantile curves $\int\_0^1 |F^{-1}(u) - G^{-1}(u)|\,du$ are **geometrically identical**—both equal the 1D Wasserstein-1 distance $W\_1(P, Q)$.

2. **Exact $\mathcal{O}(n \log n)$ Order-Statistic Decomposition of $\mathrm{CRPS}$:**
   Given $n$ sorted ensemble forecast draws $x\_{(1)} \le x\_{(2)} \le \dots \le x\_{(n)}$, a naive evaluation of $\frac{1}{2n^2}\sum\_{i=1}^n\sum\_{j=1}^n |x\_i - x\_j|$ requires $\mathcal{O}(n^2)$ operations. However, because $x\_{(i)} \ge x\_{(j)}$ whenever $i > j$, each sorted element $x\_{(i)}$ appears with a positive sign $(i - 1)$ times and with a negative sign $(n - i)$ times, reducing the double sum to a single linear pass after sorting (`exact_loo_empirical_crps` in `crates/stochology-core/src/layers/`):
   $$\frac{1}{2n^2}\sum\_{i=1}^n\sum\_{j=1}^n |x\_i - x\_j| = \frac{1}{n^2}\sum\_{i=1}^n (2i - n - 1)\,x\_{(i)}$$

<figure class="stoch-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="1120" height="340" viewBox="0 0 1120 340" fill="none" role="img" aria-labelledby="title-fig-01b-jensen">
  <title id="title-fig-01b-jensen">Geometric Proof of Jensen's Inequality: Secant Chord E[f(X)] vs. Supporting Tangent at E[X]</title>
  <defs>
    <style>text { font-family: "Inter", system-ui, -apple-system, "Segoe UI", Roboto, sans-serif; } .mono { font-family: "JetBrains Mono", ui-monospace, "Cascadia Code", monospace; }</style>
  </defs>
  <rect x="1" y="1" width="1118" height="338" rx="14" fill="var(--canvas)" stroke="var(--border)" stroke-width="1.5"/>
  <!-- Left Plot: Convex Secant vs Tangent -->
  <rect x="24" y="22" width="520" height="296" rx="12" fill="var(--elevated)" stroke="var(--sky)" stroke-width="2"/>
  <text x="44" y="48" fill="var(--sky)" class="mono" font-size="11.5" font-weight="800">GEOMETRY OF CONVEXITY · SECANT CHORD ABOVE TANGENT</text>
  <line x1="64" y1="276" x2="510" y2="276" stroke="var(--muted)" stroke-width="1.5"/>
  <line x1="64" y1="64" x2="64" y2="276" stroke="var(--muted)" stroke-width="1.5"/>
  <!-- Convex curve f(x) -->
  <path d="M 84 258 Q 260 250, 480 78" stroke="var(--sky)" stroke-width="3" fill="none"/>
  <!-- Tangent line at mu -->
  <line x1="120" y1="268" x2="460" y2="146" stroke="var(--emerald)" stroke-width="1.8" stroke-dasharray="5 4"/>
  <!-- Secant chord between x1 and x2 -->
  <line x1="110" y1="254" x2="460" y2="94" stroke="var(--amber)" stroke-width="2.4" stroke-dasharray="6 3"/>
  <!-- Vertical Jensen Gap at mu = 285 -->
  <line x1="285" y1="174" x2="285" y2="209" stroke="var(--rose)" stroke-width="3.6"/>
  <circle cx="110" cy="254" r="5" fill="var(--amber)"/>
  <circle cx="460" cy="94" r="5" fill="var(--amber)"/>
  <circle cx="285" cy="174" r="5.5" fill="var(--rose)"/>
  <circle cx="285" cy="209" r="5.5" fill="var(--sky)"/>
  <text x="298" y="188" fill="var(--rose)" class="mono" font-size="12" font-weight="800">E[f(X)] (Chord Midpoint)</text>
  <text x="298" y="222" fill="var(--sky)" class="mono" font-size="12" font-weight="800">f(E[X]) (Curve at μ)</text>
  <text x="150" y="198" fill="var(--rose)" class="mono" font-size="12" font-weight="800">Δ_J = E[f(X)] - f(E[X]) ≥ 0</text>
  <text x="80" y="298" fill="var(--muted)" class="mono" font-size="11">x₁ (Shallows)</text>
  <text x="265" y="298" fill="var(--ink)" class="mono" font-size="11" font-weight="700">μ = E[X]</text>
  <text x="425" y="298" fill="var(--muted)" class="mono" font-size="11">x₂ (Channel)</text>

  <!-- Right Plot: Vallender Identity & LOO CRPS -->
  <rect x="576" y="22" width="520" height="296" rx="12" fill="var(--elevated)" stroke="var(--violet)" stroke-width="2"/>
  <text x="596" y="48" fill="var(--violet)" class="mono" font-size="11.5" font-weight="800">SEVEN-NUMBER NONLINEAR SUMMARY (PROHIBITION 2)</text>
  <text x="596" y="76" fill="var(--ink)" font-size="14" font-weight="750">Why a Single Scalar Mean Is Rejected by Gate G6</text>
  <text x="596" y="106" fill="var(--muted)" class="mono" font-size="11.5">1. f(E[X])  — Naive plug-in cost at the mean input</text>
  <text x="596" y="130" fill="var(--muted)" class="mono" font-size="11.5">2. E[f(X)]  — True expected cost across the distribution</text>
  <text x="596" y="154" fill="var(--rose)" class="mono" font-size="11.5" font-weight="700">3. Δ_J      — Jensen's Gap E[f(X)] - f(E[X]) ≈ ½f''(μ)σ²</text>
  <text x="596" y="178" fill="var(--muted)" class="mono" font-size="11.5">4. q_05     — 5th percentile lower-tail outcome</text>
  <text x="596" y="202" fill="var(--emerald)" class="mono" font-size="11.5" font-weight="700">5. q_50     — Rank-robust median (immune to single spike)</text>
  <text x="596" y="226" fill="var(--amber)" class="mono" font-size="11.5" font-weight="700">6. q_95     — 95th percentile upper-tail congestion/loss</text>
  <text x="596" y="250" fill="var(--sky)" class="mono" font-size="11.5" font-weight="700">7. CRPS     — Strictly proper full-CDF calibration score</text>
  <text x="596" y="286" fill="var(--ink)" class="mono" font-size="10.8" font-weight="700">bank_calls.csv (measured): Δ_J=+9.03×10⁶ calls²/d (≈0.85% f(E[X])) · CRPS=9.06×10⁷</text>
  </svg>
  <figcaption><strong>Figure 3.2 — Geometric proof of Jensen's Inequality via secant chords and supporting tangent lines, paired with the mandatory 7-number `NonlinearSummary` audited in Gate G6.</strong></figcaption>
</figure>

### Interactive Laboratory (`/lab`)

Explore how input dispersion $\delta$ and payoff curvature $c$ interact to amplify Jensen's Gap $\Delta\_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X])$ in real time:

<div class="stoch-lab" data-lab="jensen-crps" data-chapter="03-random-variables-convexity.html" data-title="Chapter 3 Lab: Convexity, Secant Geometry &amp; Jensen's Gap (Δ_J) Explorer" data-cli="cargo run --release --bin labs -- --layers"></div>


---

## Computational Audit in Rust

All four building blocks are implemented in pure Rust in [`crates/stochology-core/src/layers/mod.rs`](../crates/stochology-core/src/layers/mod.rs):
- **`NonlinearSummary::evaluate(&samples, f, observed_outcomes)`:** Sorts $n$ observations, computes $\mathbb{E}[X]$, $f(\mathbb{E}[X])$, $\mathbb{E}[f(X)]$, Jensen's Gap $\Delta\_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X])$, the median, $q\_{0.05}$, $q\_{0.95}$, and the empirical $\mathrm{CRPS}$.
- **`empirical_crps(&forecast_samples, y_observed)` & `exact_loo_empirical_crps(&samples)`:** Evaluate the exact $\mathcal{O}(n \log n)$ empirical and leave-one-out $\mathrm{CRPS}$ integrals from sorted ensemble draws.
- **`empirical_wasserstein_1(&samples_p, &samples_q)`:** Computes the exact 1D Earth Mover's distance $W\_1(P, Q)$ between two empirical distributions.

Run Lab 1 to execute these primitives on synthetic and empirical distributions:

~~~bash
cargo run --release --bin labs -- --layers
~~~

## Exercises

1. **Hand Calculation of Order Statistics and Jensen's Gap:** Given the sample $X = \{2, 4, 6, 8, 20\}$ and the convex cost function $f(x) = x^2$:
   - Compute $\mathbb{E}[X]$, the median $q\_{0.50}$, and the variance $\mathrm{Var}(X) = \mathbb{E}[X^2] - (\mathbb{E}[X])^2$.
   - Show that $f(\mathbb{E}[X]) = 64$, $\mathbb{E}[f(X)] = 104$, and verify the exact identity $\Delta\_J = \mathbb{E}[X^2] - (\mathbb{E}[X])^2 = \mathrm{Var}(X) = 40$.
2. **Cosine Geometry of Correlation:** Construct two centered vectors $\tilde{\mathbf{x}} = (-1, 0, 1)$ and $\tilde{\mathbf{y}} = (1, -2, 1)$ in $\mathbb{R}^3$. Show that their dot product $\langle \tilde{\mathbf{x}}, \tilde{\mathbf{y}} \rangle = 0$ ($\rho\_{X,Y} = 0$, zero linear correlation) even though $\tilde{y}\_i = 3\tilde{x}\_i^2 - 2$ is a deterministic quadratic function of $\tilde{x}\_i$. What does this teach about the difference between *uncorrelated* ($\rho = 0$) and *independent*?
3. **Wasserstein-1 vs. CRPS:** Suppose Forecast A predicts a deterministic value $x = 10$ (all mass at $10$), Forecast B predicts a uniform distribution on $[8, 12]$, and the true outcome is $y = 10$. Which forecast has a smaller $\mathrm{CRPS}(F, 10)$? Now suppose the true outcome is $y = 12$: compute $\mathrm{CRPS}(F\_A, 12)$ vs. $\mathrm{CRPS}(F\_B, 12)$ and explain why ensemble spread protects against tail surprises.

## Summary

- **Order statistics and quantiles** ($q\_{05}, q\_{50}, q\_{95}$) remain stable under heavy-tailed shocks that distort sample means and variances.
- **Jensen's Inequality** guarantees that for any convex cost or payoff $f$, $\mathbb{E}[f(X)] \ge f(\mathbb{E}[X])$, making **Jensen's Gap** $\Delta\_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X])$ a mandatory audit quantity (`PROHIBITION_2`).
- **Continuous Ranked Probability Score ($\mathrm{CRPS}$)** and **1D Wasserstein distance ($W\_1$)** evaluate the entire predictive distribution against realized outcomes rather than collapsing forecasts to a single point.
