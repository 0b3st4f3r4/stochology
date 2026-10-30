# 6. Sampling, Matrix Algebra, Estimation, and Regression

> **Three tempos.** **Detachment:** set aside the belief that more rows mean more information. **Abstraction:** the independent unit, not the row, carries information; estimation is compression through a sampling design. **Return:** sampling, matrix algebra, estimation, and regression. **Negative proof:** none in this chapter; the tools here feed the proofs of later chapters.

## When Ten Thousand Data Points Give You Zero New Information About the Bias

Suppose an artillery officer wants to calibrate the elevation angle $\theta$ of a cannon by firing $n$ shells at a target $1\text{,}000\text{ m}$ away and measuring where they land.
- In a laboratory wind tunnel where the atmospheric drag equation is $100\%$ exact, every shell lands at $y\_i = f(x\_i, \theta) + \varepsilon\_i$, where $\varepsilon\_i \sim \mathcal{N}(0, \sigma\_n^2)$ is independent gusts of wind.
- If a single shell has wind noise $\sigma\_n = 20\text{ m}$, averaging $n = 100$ shells shrinks the standard error to $20 / \sqrt{100} = 2\text{ m}$. Averaging $n = 10\text{,}000$ shells shrinks it to $20 / \sqrt{10\text{,}000} = 0.20\text{ m}$.

Encouraged by this $\frac{1}{\sqrt{n}}$ scaling, the officer fires $10\text{,}000$ shells in the field, fits $\hat{\theta}\_{\mathrm{OLS}}$ by least squares, and reports that the cannon's angle is now known to $\pm 0.001^\circ$. Yet the next morning, when he aims at a ridge $2\text{,}500\text{ m}$ away, every shell misses by $180\text{ m}$.

The failure occurred because the officer's ballistic formula $f(x, \theta)$ omitted the quadratic Coriolis and altitude air-density drop $\delta(x)$. When he minimized the sum of squared errors $\sum (y\_i - f(x\_i, \theta))^2$ at $x = 1\text{,}000\text{ m}$, the least-squares formula **forced the parameter $\hat{\theta}$ to absorb the missing physics $\delta(1\text{,}000)$**. Firing $10\text{,}000$ shells only drove the random wind variance $\sigma\_n^2 / n$ to zero; it **locked the parameter into a false value** with near-zero reported uncertainty.

Diagnosing and curing parameter pseudo-convergence requires separating within-regime noise from between-regime ignorance via the Law of Total Variance, replacing uncorrelated least-squares projections with Generalized Least Squares, and factoring positive-definite kernel covariance matrices $\Sigma = L L^\top$ for structural discrepancy $\delta(x) \sim \mathcal{GP}(0, k)$.

<figure class="stoch-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="1160" height="440" viewBox="0 0 1160 440" fill="none" role="img" aria-labelledby="title-fig-03-estimation">
  <title id="title-fig-03-estimation">From Sampling and Total Variance to Orthogonal Projection (OLS/GLS) and Cholesky Kernel Factorization</title>
  <defs>
  <style>text { font-family: "Inter", system-ui, -apple-system, "Segoe UI", Roboto, sans-serif; } .mono { font-family: "JetBrains Mono", ui-monospace, "Cascadia Code", monospace; }</style>
  </defs>
  <rect x="1" y="1" width="1158" height="438" rx="14" fill="var(--canvas)" stroke="var(--border)" stroke-width="1.5"/>

  <!-- Panel 1: Sampling & Law of Total Variance -->
  <rect x="24" y="24" width="356" height="392" rx="12" fill="var(--elevated)" stroke="var(--sky)" stroke-width="2"/>
  <text x="44" y="54" fill="var(--sky)" class="mono" font-size="11.5" font-weight="800">1 · SAMPLING &amp; LAW OF TOTAL VARIANCE</text>
  <text x="44" y="76" fill="var(--ink)" font-size="14" font-weight="750">Eve's Law: Within-Regime + Between-Regime</text>
  <rect x="44" y="102" width="316" height="68" rx="8" fill="var(--canvas)" stroke="var(--border)"/>
  <text x="58" y="128" fill="var(--ink)" class="mono" font-size="11.5" font-weight="700">Var(X̄_n) = σ² / n  ⇒  SE(X̄_n) = σ / √n</text>
  <text x="58" y="152" fill="var(--muted)" font-size="11.5">4× more independent data → ½ the standard error</text>
  <rect x="44" y="186" width="316" height="114" rx="8" fill="var(--canvas)" stroke="var(--sky)" stroke-width="1.5"/>
  <text x="58" y="212" fill="var(--sky)" class="mono" font-size="11.5" font-weight="800">Var(Y) = E[Var(Y | θ)] + Var(E[Y | θ])</text>
  <text x="58" y="242" fill="var(--emerald)" font-size="11.5" font-weight="700">• E[Var(Y | θ)] : Irreducible Aleatoric Noise</text>
  <text x="58" y="266" fill="var(--amber)" font-size="11.5" font-weight="700">• Var(E[Y | θ]) : Reducible Epistemic Ignorance</text>
  <text x="58" y="288" fill="var(--muted)" font-size="11">Separates coin randomness from unknown coin bias</text>
  <text x="44" y="340" fill="var(--muted)" font-size="11.5">• Only independent units shrink σ / √n</text>
  <text x="44" y="362" fill="var(--muted)" font-size="11.5">• Structural bias δ(x) does NOT shrink with n</text>

  <!-- Panel 2: Geometric Projection of OLS / GLS -->
  <rect x="402" y="24" width="356" height="392" rx="12" fill="var(--elevated)" stroke="var(--emerald)" stroke-width="2"/>
  <text x="422" y="54" fill="var(--emerald)" class="mono" font-size="11.5" font-weight="800">2 · LEAST SQUARES AS GEOMETRIC PROJECTION</text>
  <text x="422" y="76" fill="var(--ink)" font-size="14" font-weight="750">Residual r = y - Xθ̂ is Orthogonal to Col(X)</text>
  <polygon points="432,276 540,210 730,210 622,276" fill="var(--canvas)" stroke="var(--muted)" stroke-width="1.5"/>
  <line x1="470" y1="255" x2="660" y2="130" stroke="var(--sky)" stroke-width="2.8"/>
  <line x1="470" y1="255" x2="660" y2="235" stroke="var(--emerald)" stroke-width="2.8"/>
  <line x1="660" y1="235" x2="660" y2="130" stroke="var(--rose)" stroke-width="2.4" stroke-dasharray="5 3"/>
  <rect x="648" y="223" width="12" height="12" stroke="var(--amber)" stroke-width="1.5" fill="none"/>
  <text x="595" y="138" fill="var(--sky)" class="mono" font-size="11.5" font-weight="700">y (Observed)</text>
  <text x="545" y="254" fill="var(--emerald)" class="mono" font-size="11.5" font-weight="700">ŷ = Xθ̂_OLS</text>
  <text x="670" y="185" fill="var(--rose)" class="mono" font-size="11" font-weight="700">r = y - Xθ̂</text>
  <text x="422" y="316" fill="var(--ink)" class="mono" font-size="11.5" font-weight="700">Xᵀ(y - Xθ̂) = 0  ⇒  θ̂_OLS = (XᵀX)⁻¹Xᵀy</text>
  <text x="422" y="344" fill="var(--ink)" class="mono" font-size="11" font-weight="700">GLS (correlated noise Σ): θ̂_GLS = (XᵀΣ⁻¹X)⁻¹XᵀΣ⁻¹y</text>
  <text x="422" y="372" fill="var(--muted)" font-size="11.5">• If true y contains δ(x), Xθ̂ absorbs proj_X(δ)!</text>

  <!-- Panel 3: Covariance Matrices, Kernels & Cholesky -->
  <rect x="780" y="24" width="356" height="392" rx="12" fill="var(--elevated)" stroke="var(--violet)" stroke-width="2"/>
  <text x="800" y="54" fill="var(--violet)" class="mono" font-size="11.5" font-weight="800">3 · KERNEL MATRICES &amp; CHOLESKY Σ = L Lᵀ</text>
  <text x="800" y="76" fill="var(--ink)" font-size="14" font-weight="750">Exact Inversion-Free Linear Algebra</text>
  <rect x="800" y="102" width="316" height="92" rx="8" fill="var(--canvas)" stroke="var(--violet)" stroke-width="1.5"/>
  <text x="814" y="126" fill="var(--violet)" class="mono" font-size="11.5" font-weight="800">k(x, x') = σ_δ² exp(-|x - x'|² / (2ℓ²))</text>
  <text x="814" y="150" fill="var(--ink)" font-size="11.5">Nearby inputs x ≈ x' share correlated errors;</text>
  <text x="814" y="170" fill="var(--ink)" font-size="11.5">distant inputs |x - x'| ≫ ℓ become independent.</text>
  <rect x="800" y="210" width="316" height="104" rx="8" fill="var(--canvas)" stroke="var(--border)"/>
  <text x="814" y="236" fill="var(--emerald)" class="mono" font-size="11.5" font-weight="800">Cholesky: Σ = K + σ_n² I = L · Lᵀ</text>
  <text x="814" y="260" fill="var(--ink)" class="mono" font-size="11">1. Forward solve:  L z = r</text>
  <text x="814" y="280" fill="var(--ink)" class="mono" font-size="11">2. Backward solve: Lᵀ α = z  (α = Σ⁻¹ r)</text>
  <text x="814" y="300" fill="var(--amber)" class="mono" font-size="11">3. Log-det: ln|Σ| = 2 ∑ᵢ ln(L_ii)</text>
  <text x="800" y="346" fill="var(--muted)" font-size="11.5">• Never compute Σ⁻¹ explicitly (numerically unstable)</text>
  <text x="800" y="368" fill="var(--muted)" font-size="11.5">• Diagonal nugget σ_n² I guarantees κ(Σ) is well-posed</text>
  </svg>
  <figcaption>Figure 6.1 — The mathematical bridge from sample statistics to structural uncertainty: (1) The √n law and Eve's Law of Total Variance; (2) Least-squares estimation as orthogonal projection onto the design column space; and (3) Positive-definite kernel covariance matrices solved via Cholesky factorization Σ = L Lᵀ.</figcaption>
</figure>

---

## 6.1 From Finite Samples to the $\sqrt{n}$ Law and Eve's Law of Total Variance

### Why Averaging $n$ Independent Measurements Shrinks Noise by $\sqrt{n}$
Let $X\_1, X\_2, \dots, X\_n$ be **independent and identically distributed (i.i.d.)** random variables with mean $\mu = \mathbb{E}[X\_i]$ and finite variance $\sigma^2 = \mathrm{Var}(X\_i) < \infty$. Consider the sample mean $\bar{X}\_n = \frac{1}{n}\sum\_{i=1}^n X\_i$:

1. **Unbiasedness (by linearity of expectation):**
   $$\mathbb{E}[\bar{X}\_n] = \frac{1}{n}\sum\_{i=1}^n \mathbb{E}[X\_i] = \frac{n\mu}{n} = \mu$$
2. **Variance of the Sample Mean (by independence, $\mathrm{Cov}(X\_i, X\_j) = 0$ for $i \ne j$):**
   Because scaling a random variable by $1/n$ scales its variance by $(1/n)^2 = 1/n^2$,
   $$\mathrm{Var}(\bar{X}\_n) = \mathrm{Var}\!\left(\frac{1}{n}\sum\_{i=1}^n X\_i\right) = \frac{1}{n^2}\sum\_{i=1}^n \mathrm{Var}(X\_i) = \frac{n\sigma^2}{n^2} = \frac{\sigma^2}{n}$$

Taking the square root gives the **Standard Error of the Mean**:

$$\mathrm{SE}(\bar{X}\_n) = \frac{\sigma}{\sqrt{n}}$$

This leads to two classical results:
- **Law of Large Numbers (LLN):** As $n \to \infty$, $\mathrm{Var}(\bar{X}\_n) = \sigma^2 / n \to 0$, so $\bar{X}\_n$ converges in probability to $\mu$ (provided $\sigma^2 < \infty$; as we will see in Chapter 9, heavy-tailed Pareto processes with $\alpha \le 2$ violate this).
- **Central Limit Theorem (CLT):** When finite-variance independent perturbations are summed and scaled by $\sqrt{n}$, the standardized error $Z\_n = \frac{\bar{X}\_n - \mu}{\sigma / \sqrt{n}}$ converges in distribution to a Standard Gaussian $\mathcal{N}(0, 1)$ with density $\phi(z) = \frac{1}{\sqrt{2\pi}}e^{-z^2/2}$.

### Eve's Law (The Law of Total Variance)
When the data-generating process depends on an unknown regime or parameter $\theta$, we need to separate the randomness of individual outcomes *given* $\theta$ from our uncertainty about *which* $\theta$ is true.

First, recall the **Law of Iterated Expectations (Adam's Law)**: the overall expectation of $Y$ is the average of its conditional expectations across all possible values of $\theta$:

$$\mathbb{E}[Y] = \mathbb{E}\_\theta\!\left[\mathbb{E}[Y \mid \theta]\right]$$

Using Adam's Law, we can decompose the total variance $\mathrm{Var}(Y) = \mathbb{E}[Y^2] - (\mathbb{E}[Y])^2$ by adding and subtracting $\mathbb{E}[(\mathbb{E}[Y \mid \theta])^2]$:

$$\begin{aligned}
\mathrm{Var}(Y) &= \mathbb{E}\_\theta\!\left[\mathbb{E}[Y^2 \mid \theta] - (\mathbb{E}[Y \mid \theta])^2\right] + \left(\mathbb{E}\_\theta\!\left[(\mathbb{E}[Y \mid \theta])^2\right] - \left(\mathbb{E}\_\theta[\mathbb{E}[Y \mid \theta]]\right)^2\right) \\
&= \underbrace{\mathbb{E}\_\theta\!\left[\mathrm{Var}(Y \mid \theta)\right]}\_{\text{Average Within-Regime Variance}} + \underbrace{\mathrm{Var}\_\theta\!\left(\mathbb{E}[Y \mid \theta]\right)}\_{\text{Variance Between Regime Means}}
\end{aligned}$$

This identity is known as **Eve's Law** (or the **Law of Total Variance**). In Chapter 2, it provides the mathematical boundary between **Aleatoric Uncertainty** ($\mathbb{E}[\mathrm{Var}(Y \mid \theta)]$, the irreducible physical scatter inside a known regime) and **Epistemic Uncertainty** ($\mathrm{Var}(\mathbb{E}[Y \mid \theta])$, our reducible ignorance about the true regime $\theta$).

---

## 6.2 Applied Matrix Algebra: Positive-Definite Covariance Matrices and Cholesky Factorization

When we observe a vector of $n$ measurements $\mathbf{y} = (y\_1, y\_2, \dots, y\_n)^\top \in \mathbb{R}^n$, their joint second-order structure is captured by an $n \times n$ **Covariance Matrix** $\Sigma$:

$$\Sigma\_{ij} = \mathrm{Cov}(Y\_i, Y\_j) = \mathbb{E}\!\left[(Y\_i - \mu\_i)(Y\_j - \mu\_j)\right]$$

- The **diagonal entries** $\Sigma\_{ii} = \mathrm{Var}(Y\_i) \ge 0$ are the individual variances of each measurement.
- The **off-diagonal entries** $\Sigma\_{ij} = \Sigma\_{ji}$ measure how strongly measurement $i$ and measurement $j$ co-move. Because $\mathrm{Cov}(Y\_i, Y\_j) = \mathrm{Cov}(Y\_j, Y\_i)$, every covariance matrix is **symmetric** ($\Sigma^\top = \Sigma$).

### Why Every Valid Covariance Matrix Is Positive Semi-Definite
Take any weight vector $\mathbf{w} = (w\_1, \dots, w\_n)^\top \in \mathbb{R}^n$ and form the scalar linear combination $Z = \mathbf{w}^\top \mathbf{Y} = \sum\_{i=1}^n w\_i Y\_i$. Because the variance of any real random variable $Z$ cannot be negative:

$$0 \le \mathrm{Var}(\mathbf{w}^\top \mathbf{Y}) = \sum\_{i=1}^n \sum\_{j=1}^n w\_i w\_j \mathrm{Cov}(Y\_i, Y\_j) = \mathbf{w}^\top \Sigma \mathbf{w}$$

If $\mathbf{w}^\top \Sigma \mathbf{w} > 0$ for every non-zero vector $\mathbf{w} \ne \mathbf{0}$, $\Sigma$ is called **Symmetric Positive-Definite (SPD)**. All of its eigenvalues $\lambda\_1 \ge \lambda\_2 \ge \dots \ge \lambda\_n > 0$ are strictly positive, and the ratio $\kappa(\Sigma) = \lambda\_{\max} / \lambda\_{\min}$ is its **condition number** (measuring how close $\Sigma$ is to becoming singular).

### Step-by-Step Cholesky Decomposition ($\Sigma = L L^\top$)
In statistical computing (and inside [`crates/stochology-core/src/uncertainty.rs`](../crates/stochology-core/src/uncertainty.rs)), we frequently need to solve a linear system $\Sigma \boldsymbol{\alpha} = \mathbf{r}$ or evaluate the determinant $|\Sigma|$. **Directly computing the matrix inverse $\Sigma^{-1}$ is numerically unstable:** it wastes flops and amplifies floating-point rounding errors whenever $\kappa(\Sigma)$ is large.

Instead, every Symmetric Positive-Definite matrix $\Sigma$ admits a unique **Cholesky Factorization**, the matrix equivalent of taking a square root:

$$\Sigma = L L^\top$$

where $L$ is a **lower-triangular matrix** ($L\_{ij} = 0$ for all $j > i$) with strictly positive diagonal entries $L\_{ii} > 0$. By writing out the row-by-column product $\Sigma\_{ij} = \sum\_{k=1}^j L\_{ik} L\_{jk}$ entry by entry from top-left to bottom-right, we obtain the exact recursive algorithm used in `cholesky_decompose`:

1. **Diagonal entry ($i = j$):**
   $$L\_{ii} = \sqrt{\Sigma\_{ii} - \sum\_{k=1}^{i-1} L\_{ik}^2}$$
2. **Below-diagonal entry ($i > j$):**
   $$L\_{ij} = \frac{1}{L\_{jj}}\left(\Sigma\_{ij} - \sum\_{k=1}^{j-1} L\_{ik} L\_{jk}\right)$$

### Worked $2 \times 2$ Example of Cholesky Factorization
Suppose two nearby sensors have covariance matrix:

$$\Sigma = \begin{bmatrix} 4 & 2 \\ 2 & 5 \end{bmatrix}$$

Let us find $L = \begin{bmatrix} L\_{11} & 0 \\ L\_{21} & L\_{22} \end{bmatrix}$ such that $L L^\top = \Sigma$:
1. Top-left diagonal: $L\_{11} = \sqrt{\Sigma\_{11}} = \sqrt{4} = 2$.
2. Bottom-left off-diagonal: $L\_{21} = \Sigma\_{21} / L\_{11} = 2 / 2 = 1$.
3. Bottom-right diagonal: $L\_{22} = \sqrt{\Sigma\_{22} - L\_{21}^2} = \sqrt{5 - 1^2} = \sqrt{4} = 2$.

Check: $\begin{bmatrix} 2 & 0 \\ 1 & 2 \end{bmatrix}\begin{bmatrix} 2 & 1 \\ 0 & 2 \end{bmatrix} = \begin{bmatrix} 4 & 2 \\ 2 & 5 \end{bmatrix} = \Sigma$.

Once we have $L$, two operations simplify:
- **Solving $\Sigma \boldsymbol{\alpha} = \mathbf{r}$ without matrix inversion:** First solve the lower-triangular system $L \mathbf{z} = \mathbf{r}$ from top to bottom (**forward substitution**), then solve the upper-triangular system $L^\top \boldsymbol{\alpha} = \mathbf{z}$ from bottom to top (**backward substitution**).
- **Computing the Log-Determinant $\ln|\Sigma|$ without overflow:** Since $|\Sigma| = |L L^\top| = |L|^2 = (\prod\_{i=1}^n L\_{ii})^2$, taking logarithms turns the product into a stable sum:
  $$\ln|\Sigma| = 2 \sum\_{i=1}^n \ln(L\_{ii})$$

---

## 6.3 From Ordinary Least Squares (OLS) to Generalized Least Squares (GLS) and Likelihood

### Ordinary Least Squares (OLS) as Orthogonal Projection
Suppose we model a vector of observations $\mathbf{y} \in \mathbb{R}^n$ as a linear combination of $p$ known feature columns collected in an $n \times p$ design matrix $X$:

$$\mathbf{y} = X\boldsymbol{\theta} + \boldsymbol{\varepsilon}$$

To find the parameter vector $\hat{\boldsymbol{\theta}}\_{\mathrm{OLS}} \in \mathbb{R}^p$, we minimize the squared Euclidean norm of the residual vector $\mathbf{r}(\boldsymbol{\theta}) = \mathbf{y} - X\boldsymbol{\theta}$:

$$S(\boldsymbol{\theta}) = \|\mathbf{y} - X\boldsymbol{\theta}\|\_2^2 = (\mathbf{y} - X\boldsymbol{\theta})^\top (\mathbf{y} - X\boldsymbol{\theta}) = \mathbf{y}^\top \mathbf{y} - 2\boldsymbol{\theta}^\top X^\top \mathbf{y} + \boldsymbol{\theta}^\top X^\top X \boldsymbol{\theta}$$

Setting the gradient with respect to $\boldsymbol{\theta}$ equal to zero gives the **Normal Equations**:

$$\nabla\_{\boldsymbol{\theta}} S(\boldsymbol{\theta}) = -2 X^\top (\mathbf{y} - X\hat{\boldsymbol{\theta}}\_{\mathrm{OLS}}) = \mathbf{0} \implies X^\top X \hat{\boldsymbol{\theta}}\_{\mathrm{OLS}} = X^\top \mathbf{y}$$

Geometrically, $X^\top(\mathbf{y} - X\hat{\boldsymbol{\theta}}\_{\mathrm{OLS}}) = \mathbf{0}$ (shown in Panel 2 of Figure 6.1) means that the shortest distance from $\mathbf{y}$ to the subspace spanned by the columns of $X$ occurs when the residual vector $\mathbf{r} = \mathbf{y} - X\hat{\boldsymbol{\theta}}\_{\mathrm{OLS}}$ is **strictly perpendicular (orthogonal)** to every column of $X$. Solving for $\hat{\boldsymbol{\theta}}\_{\mathrm{OLS}}$ gives:

$$\hat{\boldsymbol{\theta}}\_{\mathrm{OLS}} = (X^\top X)^{-1} X^\top \mathbf{y}$$

### Why OLS Fails Under Missing Physics $\delta(x)$
Suppose the physical world actually generates $\mathbf{y} = X\boldsymbol{\theta}^\ast + \boldsymbol{\delta} + \boldsymbol{\varepsilon}$, where $\boldsymbol{\delta} = (\delta(x\_1), \dots, \delta(x\_n))^\top$ is a systematic structural discrepancy omitted by our model. Plugging $\mathbf{y}$ into the OLS formula gives:

$$\mathbb{E}[\hat{\boldsymbol{\theta}}\_{\mathrm{OLS}}] = (X^\top X)^{-1} X^\top (X\boldsymbol{\theta}^\ast + \boldsymbol{\delta}) = \boldsymbol{\theta}^\ast + \underbrace{(X^\top X)^{-1} X^\top \boldsymbol{\delta}}\_{\text{Ghost Parameter Bias!}}$$

Because OLS forces the residual to be orthogonal to $X$, any part of the missing physics $\boldsymbol{\delta}$ that aligns with $X$ is **projected directly into $\hat{\boldsymbol{\theta}}\_{\mathrm{OLS}}$**, distorting the physical parameter even as $n \to \infty$.

### Generalized Least Squares (GLS) and Gaussian Log-Likelihood
When the errors between observations are correlated with covariance matrix $\Sigma = \mathrm{Cov}(\boldsymbol{\varepsilon})$ (rather than independent white noise $\sigma\_n^2 I$), ordinary Euclidean distance treats redundant, highly correlated points as if they were independent.

Instead, we weight the residuals by the inverse covariance matrix $\Sigma^{-1}$ (which is equivalent to whitening the data by $L^{-1}$, where $\Sigma = L L^\top$), minimizing the **Mahalanobis distance** $(\mathbf{y} - X\boldsymbol{\theta})^\top \Sigma^{-1}(\mathbf{y} - X\boldsymbol{\theta})$. Setting the gradient to zero yields the **Generalized Least Squares (GLS)** estimator:

$$\hat{\boldsymbol{\theta}}\_{\mathrm{GLS}} = (X^\top \Sigma^{-1} X)^{-1} X^\top \Sigma^{-1} \mathbf{y}$$

If the residual vector $\mathbf{r} = \mathbf{y} - X\boldsymbol{\theta}$ follows a multivariate Gaussian distribution $\mathcal{N}(\mathbf{0}, \Sigma)$, its probability density is:

$$p(\mathbf{y} \mid \boldsymbol{\theta}, \Sigma) = \frac{1}{(2\pi)^{n/2} |\Sigma|^{1/2}} \exp\!\left(-\frac{1}{2} \mathbf{r}^\top \Sigma^{-1} \mathbf{r}\right)$$

Taking the natural logarithm gives the **Log-Marginal Likelihood** objective that we maximize in Chapter 11:

$$\ln p(\mathbf{y} \mid \boldsymbol{\theta}, \Sigma) = -\frac{1}{2}\underbrace{\mathbf{r}^\top \Sigma^{-1} \mathbf{r}}\_{\text{Data Fit Penalty}} - \frac{1}{2}\underbrace{\ln|\Sigma|}\_{\text{Complexity Penalty}} - \frac{n}{2}\ln(2\pi)$$

The Gaussian log-likelihood balances data fit ($\mathbf{r}^\top \Sigma^{-1} \mathbf{r}$, computed via Cholesky triangular solves) against model complexity ($\ln|\Sigma| = 2\sum \ln L\_{ii}$).

---

## 6.4 Kernel Functions $k(x, x')$ as Continuous Similarity Tables

How do we construct the covariance matrix $\Sigma$ when we suspect that a physical simulator has a smooth, unknown structural error $\delta(x)$?

Even without the exact formula for $\delta(x)$, a physical continuity principle holds: **if two input settings $x$ and $x'$ are close to each other, the model's structural error $\delta(x)$ and $\delta(x')$ at those two settings will be strongly correlated.** Conversely, if $x$ and $x'$ are far apart, knowing the model's error at $x$ tells us little about its error at $x'$.

We encode this smoothness assumption mathematically using a **Squared-Exponential (RBF) Covariance Kernel**:

$$k(x, x') = \mathrm{Cov}(\delta(x), \delta(x')) = \sigma\_\delta^2 \exp\!\left(-\frac{(x - x')^2}{2\ell^2}\right)$$

This kernel has two hyperparameters:
1. **Amplitude $\sigma\_\delta^2 = k(x, x)$:** When $x = x'$, $\exp(0) = 1$, so $k(x, x) = \sigma\_\delta^2 = \mathrm{Var}(\delta(x))$ is the marginal variance (vertical scale) of the structural discrepancy.
2. **Length-scale $\ell > 0$:** Controls how rapidly similarity decays with horizontal distance $|x - x'|$. When $|x - x'| \ll \ell$, $k(x, x') \approx \sigma\_\delta^2$ (errors move together); when $|x - x'| \gg \ell$, $k(x, x') \to 0$ (errors become independent).

Adding independent sensor measurement noise $\sigma\_n^2$ on the diagonal ($i = j$) produces the exact total covariance matrix used by Kennedy and O'Hagan's Gaussian Process in Chapter 11:

$$\Sigma\_{ij} = K\_{ij} + \sigma\_n^2 \delta\_{ij} = \sigma\_\delta^2 \exp\!\left(-\frac{(x\_i - x\_j)^2}{2\ell^2}\right) + \sigma\_n^2 \mathbf{1}\{i = j\}$$

Adding the positive "nugget" $\sigma\_n^2 I$ shifts every eigenvalue of $K$ upward by $\sigma\_n^2 > 0$, guaranteeing that $\Sigma$ is strictly positive-definite and that its Cholesky factorization $\Sigma = L L^\top$ never divides by zero.

---

## 6.5 Streaming High-Order Moments (`OnlineMoments`) and Künsch's Moving-Block Bootstrap

Two additional foundational statistical primitives in `stochology_core::rng` protect estimation when datasets are massive ($n > 10^5$) or serially correlated across time:

1. **Welford-Terriberry Single-Pass 4th-Order Moments (`OnlineMoments`):**
   Computing variance as $\frac{1}{n}\sum x\_i^2 - \bar{x}^2$ suffers catastrophic floating-point cancellation when $\bar{x}^2 \gg \sigma^2$. Instead, `OnlineMoments` updates the running mean $\bar{x}\_n$ and centered power sums $M\_{2,n}, M\_{3,n}, M\_{4,n}$ in a single $\mathcal{O}(1)$-memory pass using $\delta = x\_n - \bar{x}\_{n-1}$ and $\delta\_n = \delta / n$:
   $$M\_{2,n} = M\_{2,n-1} + \delta(\delta - \delta\_n)(n-1), \qquad \text{Skewness } \hat{\gamma}\_1 = \frac{\sqrt{n}\,M\_3}{M\_2^{3/2}}, \qquad \text{Excess Kurtosis } \hat{\gamma}\_2 = \frac{n M\_4}{M\_2^2} - 3$$
   From these moments, the **Jarque-Bera normality test statistic** $\mathrm{JB} = \frac{n}{6}\left(\hat{\gamma}\_1^2 + \frac{1}{4}\hat{\gamma}\_2^2\right)$ audits whether Gaussian tail assumptions are falsified (measured on `djia_1885_2023.csv`, $n = 37\text{,}930$ trading days: $\hat{\gamma}\_2 = 17.8$ and $\mathrm{JB} = 502\text{,}551$).

2. **Hans R. Künsch's (1989) Moving-Block Bootstrap (`block_bootstrap_ci`):**
   Standard Efron bootstrap resamples individual rows $x\_i$ independently with replacement, destroying temporal autocorrelation $\rho\_k$ and severely understating standard errors. Künsch's moving-block bootstrap slices the time series into overlapping contiguous blocks $B\_j = (x\_j, \dots, x\_{j+\ell-1})$ of length $\ell \sim n^{1/3}$ and resamples whole blocks using `SplitMix64` (`seed = 2026`), preserving within-block dependence structure.

### Interactive Laboratory (`/lab`)

Experiment with sample size $n$, error equicorrelation $\rho$, and structural bias $\delta(x)$ in the interactive OLS vs. Cholesky-GLS laboratory below:

<div class="stoch-lab" data-lab="ols-cholesky-gls" data-chapter="06-sampling-estimation-linear-algebra.html" data-title="Chapter 6 Lab: OLS Orthogonal Projection, Correlated Errors &amp; Cholesky GLS" data-cli="cargo run --release --bin labs -- --uncertainty"></div>


---

## Computational Audit in Rust

In [`crates/stochology-core/src/uncertainty.rs`](../crates/stochology-core/src/uncertainty.rs), you can inspect the exact zero-dependency Rust implementations of the linear-algebra and estimation primitives introduced in this chapter:
- **`cholesky_decompose(matrix, n)`:** Computes the lower-triangular factor $L$ such that $\Sigma = L L^\top$, returning `None` if any diagonal pivot $\le 10^{-12}$.
- **`cholesky_solve(l, b, n)`:** Executes forward substitution ($L\mathbf{z} = \mathbf{b}$) followed by backward substitution ($L^\top \boldsymbol{\alpha} = \mathbf{z}$) in $\mathcal{O}(n^2)$ time.
- **`KennedyOHaganDiscrepancy::kernel(x1, x2)` (`fit_exact_cholesky` & `calibrate_empirical_bayes`):** Evaluates the Squared-Exponential kernel $k(x\_1, x\_2)$, builds the regularized Gram matrix $\Sigma = K + \sigma\_n^2 I$, and maximizes the exact log-marginal likelihood.

Run Lab 2 to see OLS bias vs. Cholesky-based GLS/GP calibration in action:

~~~bash
cargo run --release --bin labs -- --uncertainty
~~~

## Exercises

1. **Verifying Eve's Law by Hand:** A factory operates in two equally likely temperature regimes ($\theta \in \{0, 1\}$, $P(\theta=0)=P(\theta=1)=0.5$). In regime $\theta=0$, output is $Y \mid (\theta=0) \sim \mathcal{N}(10, 4)$ (mean $10$, variance $4$). In regime $\theta=1$, output is $Y \mid (\theta=1) \sim \mathcal{N}(20, 6)$ (mean $20$, variance $6$).
   - Compute the average within-regime (aleatoric) variance $\mathbb{E}[\mathrm{Var}(Y \mid \theta)]$.
   - Compute the between-regime (epistemic) variance $\mathrm{Var}(\mathbb{E}[Y \mid \theta])$.
   - Sum them to find the total marginal variance $\mathrm{Var}(Y)$.
2. **Cholesky Forward and Backward Substitution:** Using the $2 \times 2$ Cholesky factor $L = \begin{bmatrix} 2 & 0 \\ 1 & 2 \end{bmatrix}$ from Worked Example 6.2 (where $\Sigma = \begin{bmatrix} 4 & 2 \\ 2 & 5 \end{bmatrix}$) and right-hand side $\mathbf{r} = \begin{bmatrix} 6 \\ 11 \end{bmatrix}$:
   - Solve $L\mathbf{z} = \mathbf{r}$ by forward substitution for $\mathbf{z} = (z\_1, z\_2)^\top$.
   - Solve $L^\top \boldsymbol{\alpha} = \mathbf{z}$ by backward substitution for $\boldsymbol{\alpha} = (\alpha\_1, \alpha\_2)^\top$, and check that $\Sigma \boldsymbol{\alpha} = \mathbf{r}$ and $\ln|\Sigma| = 2(\ln 2 + \ln 2) = \ln 16$.
3. **Ghost Parameter Bias under OLS:** Suppose a 1D linear model assumes $y = \theta x$ (through the origin) on inputs $x = (1, 2, 3)^\top$, but the true physical world obeys $y = 2x - 0.5 x^2$ (so $\theta^\ast = 2$ and $\delta(x) = -0.5 x^2$). Compute $\hat{\theta}\_{\mathrm{OLS}} = \frac{\sum x\_i y\_i}{\sum x\_i^2}$ by hand and show that OLS estimates $\hat{\theta}\_{\mathrm{OLS}} = 2 - \frac{18}{14} \approx 0.714$, missing the true linear slope $\theta^\ast = 2$ by more than $64\%$.

## Summary

- **Ordinary Least Squares (OLS)** projects outcomes orthogonally onto the design column space under i.i.d. homoscedastic noise, but suffers from understated standard errors under cluster correlation and **ghost parameter bias** when structural discrepancy $\delta(x)$ is omitted.
- **Cholesky decomposition ($\Sigma = LL^\top$)** solves Generalized Least Squares (GLS) and Gaussian Process log-marginal-likelihood equations exactly in $\mathcal{O}(n^3)$ without forming an explicit matrix inverse $\Sigma^{-1}$.
- **Cluster design effects (`DEFF`)** and **Moulton-Campbell effective sample size** $n\_{\mathrm{eff}}(p)$ prevent pseudo-replication when observations arrive in correlated operational clusters ($\Phi\_2$).
