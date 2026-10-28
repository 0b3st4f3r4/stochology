# 7. Function Spaces, Orthonormal Bases, Spectral SVD, and Minimax Complexity


## A Million Parameters That Only Know Three Directions

Suppose an engineering team trains a wide neural network or polynomial regression with $p = 1\text{,}000$ features $\phi\_1(x), \phi\_2(x), \dots, \phi\_{1000}(x)$ and boasts that their model has a "1,000-dimensional representation."

When an auditor inspects the $1\text{,}000$ feature columns evaluated on the dataset, however, she finds that features $\phi\_4$ through $\phi\_{1000}$ are nearly collinear copies of the first three features ($\phi\_1, \phi\_2, \phi\_3$). Even though the parameter vector $\boldsymbol{\theta} \in \mathbb{R}^{1000}$ occupies $1\text{,}000$ floating-point slots in RAM, $99.7\%$ of the variance of the feature matrix lives along just **three orthogonal axes**.

Fitting a curve with raw monomial powers $1, x, x^2, x^3, x^4, x^5$ on $[0, 1]$ creates a similar trap. On $[0, 1]$, the correlation between $x^4$ and $x^5$ reaches **$0.996$**. Because those basis functions point in almost the same direction, changing the coefficient of $x^4$ by $+1\text{,}000$ and the coefficient of $x^5$ by $-1\text{,}000$ barely alters the prediction inside $[0, 1]$, yet sends the polynomial to $\pm \infty$ the instant $x$ steps outside $[0, 1]$ (**Runge's Phenomenon**).

This chapter builds basis functions that are **strictly perpendicular (orthonormal)** to one another, measures the **effective number of dimensions $d\_{\mathrm{eff}}$** a model actually uses regardless of its nominal parameter count, and derives the minimax floor that bounds the estimation rate of any algorithm.

<figure class="stoch-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="1160" height="440" viewBox="0 0 1160 440" fill="none" role="img" aria-labelledby="title-fig-07-spectral">
  <title id="title-fig-07-spectral">Orthonormal Function Bases, Compositional Depth (2^D), and Effective Dimension (d_eff)</title>
  <defs>
  <style>text { font-family: "Inter", system-ui, -apple-system, "Segoe UI", Roboto, sans-serif; } .mono { font-family: "JetBrains Mono", ui-monospace, "Cascadia Code", monospace; }</style>
  </defs>
  <rect x="1" y="1" width="1158" height="438" rx="14" fill="var(--canvas)" stroke="var(--border)" stroke-width="1.5"/>

  <!-- Panel 1: Collinear Monomials vs Orthonormal Basis -->
  <rect x="24" y="24" width="356" height="392" rx="12" fill="var(--elevated)" stroke="var(--sky)" stroke-width="2"/>
  <text x="44" y="54" fill="var(--sky)" class="mono" font-size="11.5" font-weight="800">1 · FUNCTION SPACES &amp; ORTHONORMAL BASES</text>
  <text x="44" y="76" fill="var(--ink)" font-size="14" font-weight="750">⟨ψ_j, ψ_k⟩ = E[ψ_j(X)ψ_k(X)] = δ_jk</text>
  <rect x="44" y="98" width="316" height="92" rx="8" fill="var(--canvas)" stroke="var(--rose)" stroke-width="1.5"/>
  <text x="58" y="122" fill="var(--rose)" class="mono" font-size="11" font-weight="800">BAD: Raw Monomials {1, x, x², x³, x⁴}</text>
  <text x="58" y="144" fill="var(--ink)" font-size="11.5">Nearly parallel on [0, 1] (corr(x⁴, x⁵) = 0.996).</text>
  <text x="58" y="164" fill="var(--muted)" font-size="11">Ill-conditioned Gram matrix; coefficients explode!</text>
  <rect x="44" y="204" width="316" height="116" rx="8" fill="var(--canvas)" stroke="var(--emerald)" stroke-width="1.5"/>
  <text x="58" y="228" fill="var(--emerald)" class="mono" font-size="11" font-weight="800">GOOD: Orthonormal Basis (Hermite / Chebyshev)</text>
  <text x="58" y="252" fill="var(--ink)" class="mono" font-size="11">H₀(z) = 1,  H₁(z) = z,  H₂(z) = (z² - 1)/√2</text>
  <text x="58" y="276" fill="var(--ink)" font-size="11.5">Each atom captures a strictly independent</text>
  <text x="58" y="296" fill="var(--ink)" font-size="11.5">mode of variation: Gram matrix G = I!</text>
  <text x="44" y="352" fill="var(--muted)" font-size="11.5">• Parseval's Identity: ‖f‖² = ∑_k c_k²</text>
  <text x="44" y="374" fill="var(--muted)" font-size="11.5">• Dropping small |c_k| gives clean sparse Ŝ*</text>

  <!-- Panel 2: Curse of Dimensionality vs Compositional Sawtooth -->
  <rect x="402" y="24" width="356" height="392" rx="12" fill="var(--elevated)" stroke="var(--emerald)" stroke-width="2"/>
  <text x="422" y="54" fill="var(--emerald)" class="mono" font-size="11.5" font-weight="800">2 · SHALLOW ADDITION vs. DEEP COMPOSITION</text>
  <text x="422" y="76" fill="var(--ink)" font-size="14" font-weight="750">Why Composing D Layers Multiplies Folds (2^D)</text>
  <polyline points="432,190 472,120 512,190 552,120 592,190 632,120 672,190 712,120 732,155" stroke="var(--emerald)" stroke-width="2.6" fill="none"/>
  <text x="432" y="220" fill="var(--emerald)" class="mono" font-size="11.5" font-weight="700">Tent Map Δ(x) = max(0, 1 - |2x - 1|):</text>
  <text x="432" y="244" fill="var(--ink)" class="mono" font-size="11">• Depth 1: Δ(x)       → 1 peak  (2¹ segments)</text>
  <text x="432" y="266" fill="var(--ink)" class="mono" font-size="11">• Depth 2: Δ(Δ(x))    → 2 peaks (2² segments)</text>
  <text x="432" y="288" fill="var(--ink)" class="mono" font-size="11">• Depth D: Δ∘D(x)     → 2^(D-1) peaks (2^D segs)</text>
  <rect x="422" y="308" width="316" height="84" rx="8" fill="var(--canvas)" stroke="var(--border)"/>
  <text x="436" y="332" fill="var(--amber)" class="mono" font-size="11" font-weight="800">Telgarsky Separation Theorem:</text>
  <text x="436" y="354" fill="var(--ink)" font-size="11">A shallow network needs O(2^D) width to match</text>
  <text x="436" y="374" fill="var(--ink)" font-size="11">what a deep compositional network does in O(D)!</text>

  <!-- Panel 3: Gram Spectrum & Effective Dimension -->
  <rect x="780" y="24" width="356" height="392" rx="12" fill="var(--elevated)" stroke="var(--violet)" stroke-width="2"/>
  <text x="800" y="54" fill="var(--violet)" class="mono" font-size="11.5" font-weight="800">3 · GRAM SPECTRUM &amp; EFFECTIVE DIMENSION</text>
  <text x="800" y="76" fill="var(--ink)" font-size="14" font-weight="750">Participation Ratio d_eff = (tr G)² / ‖G‖_F²</text>
  <rect x="800" y="100" width="316" height="106" rx="8" fill="var(--canvas)" stroke="var(--violet)" stroke-width="1.5"/>
  <text x="814" y="126" fill="var(--violet)" class="mono" font-size="11.5" font-weight="800">G = (1/n) Φᵀ Φ,   Eigenvalues λ₁, …, λ_m ≥ 0</text>
  <text x="814" y="152" fill="var(--ink)" class="mono" font-size="11.5">• tr(G)   = ∑ G_ii     = ∑ λ_k</text>
  <text x="814" y="174" fill="var(--ink)" class="mono" font-size="11.5">• ‖G‖_F²  = ∑_ij G_ij² = ∑ λ_k²</text>
  <text x="814" y="196" fill="var(--emerald)" class="mono" font-size="11.5" font-weight="700">⇒ d_eff = (∑ λ_k)² / (∑ λ_k²) ∈ [1, m]</text>
  <rect x="800" y="220" width="316" height="106" rx="8" fill="var(--canvas)" stroke="var(--border)"/>
  <text x="814" y="244" fill="var(--amber)" class="mono" font-size="11" font-weight="800">Two Extreme Regimes of a Representation:</text>
  <text x="814" y="268" fill="var(--rose)" font-size="11">• Rank-1 Collapse (λ₁ &gt; 0, others 0): d_eff = 1.0</text>
  <text x="814" y="290" fill="var(--emerald)" font-size="11">• Isotropic Orthonormal (all λ_k = c): d_eff = m</text>
  <text x="814" y="312" fill="var(--muted)" font-size="10.5">Detects bloated overparameterization instantly!</text>
  <text x="800" y="356" fill="var(--muted)" font-size="11.5">• Minimax Floor: inf_θ̂ sup_θ E[‖θ̂ - θ‖²] ≥ c/n</text>
  <text x="800" y="378" fill="var(--muted)" font-size="11.5">• No estimator can beat Fano / Le Cam bounds</text>
  </svg>
  <figcaption>Figure 7.1 — Foundations of functional approximation and representation complexity: (1) Orthonormal basis expansions vs. collinear monomials; (2) Exponential folding (2^D) via function composition; and (3) Spectral effective dimension d_eff via the Gram participation ratio.</figcaption>
</figure>

---

## 7.1 Functions as Vectors: Inner Products and Orthonormal Bases in $L^2$

In Chapter 1, we saw that two vectors $\mathbf{u}, \mathbf{v} \in \mathbb{R}^n$ are **orthogonal** ($\mathbf{u} \perp \mathbf{v}$) if their dot product $\langle \mathbf{u}, \mathbf{v} \rangle = \sum\_{i=1}^n u\_i v\_i = 0$, and **orthonormal** if they also have unit length $\|\mathbf{u}\|\_2 = 1$.

The same geometry extends to **continuous functions** $f(x)$ and $g(x)$. Define the **$L^2(P\_X)$ inner product** between two functions $f, g$ as the expected value of their product when $X$ is drawn from distribution $P\_X$:

$$\langle f, g \rangle\_{L^2} = \mathbb{E}[f(X)g(X)] = \int f(x)g(x)\,dP\_X(x), \qquad \|f\|\_{L^2} = \sqrt{\langle f, f \rangle\_{L^2}} = \sqrt{\mathbb{E}[f(X)^2]}$$

### Definition 7.1 (Orthonormal Basis of Functions)
A family of basis functions ("atoms") $\{\psi\_0(x), \psi\_1(x), \psi\_2(x), \dots\}$ is **orthonormal** with respect to $P\_X$ if every pair of distinct functions is perpendicular and every function has unit energy:

$$\langle \psi\_j, \psi\_k \rangle\_{L^2} = \mathbb{E}[\psi\_j(X)\psi\_k(X)] = \delta\_{jk} = \begin{cases} 1 & \text{if } j = k, \\ 0 & \text{if } j \ne k. \end{cases}$$

Expanding an unknown target function $f(x) = \sum\_{k=0}^K c\_k \psi\_k(x)$ in an **orthonormal basis** simplifies estimation in three ways:
1. **No Matrix Inversion Needed:** Because the columns are mutually perpendicular, the design Gram matrix is the identity matrix ($G = I$). To find coefficient $c\_j$, we take the inner product of $f$ with $\psi\_j$:
   $$c\_j = \langle f, \psi\_j \rangle\_{L^2} = \mathbb{E}[f(X)\psi\_j(X)]$$
2. **Independent Coefficients (Zero Collinearity):** Adding a new basis function $\psi\_{K+1}(x)$ to the model **does not change** any of the previously estimated coefficients $c\_0, \dots, c\_K$.
3. **Parseval's Energy Identity:** The total squared norm (energy) of the function equals the sum of its squared coefficients, which is the infinite-dimensional Pythagorean theorem:
   $$\|f\|\_{L^2}^2 = \mathbb{E}[f(X)^2] = \sum\_{k=0}^K c\_k^2$$

### Three Canonical Orthonormal Families Used in Stochology
Depending on the domain of $X$, three orthonormal families appear repeatedly across this book:
1. **Probabilists' Orthonormal Hermite Polynomials (for Gaussian-like $Z \sim \mathcal{N}(0, 1)$, used in Chapter 6 Lens 4 for Nonlinear ICP / NILE):**
   Starting from $1, z, z^2, z^3$ and applying Gram-Schmidt orthogonalization against the standard Gaussian density gives:
   $$\psi\_0(z) = 1, \quad \psi\_1(z) = z, \quad \psi\_2(z) = \frac{z^2 - 1}{\sqrt{2}}, \quad \psi\_3(z) = \frac{z^3 - 3z}{\sqrt{6}}$$
   We can verify orthogonality of $\psi\_0$ and $\psi\_2$ directly: since $\mathbb{E}[Z^2] = 1$, $\mathbb{E}[\psi\_0(Z)\psi\_2(Z)] = \frac{\mathbb{E}[Z^2] - 1}{\sqrt{2}} = 0$.
2. **Chebyshev Polynomials $T\_k(x) = \cos(k \arccos x)$ on $[-1, 1]$ (used in Chapter 8 for Authentic Symbolic Forms):**
   By the trigonometric identity $\cos((k+1)\theta) + \cos((k-1)\theta) = 2\cos\theta \cos(k\theta)$, they obey the exact 3-term recurrence:
   $$T\_0(x) = 1, \quad T\_1(x) = x, \quad T\_2(x) = 2x^2 - 1, \quad T\_{k+1}(x) = 2x T\_k(x) - T\_{k-1}(x)$$
   Unlike raw powers $x^k$ that bunch up near $\pm 1$ and blow up outside the interval, every Chebyshev polynomial $T\_k(x)$ oscillates evenly between $-1$ and $+1$ across $[-1, 1]$.
3. **Fourier Sinusoids $\cos(\omega x), \sin(\omega x)$ (used in Chapters 8 and 11 for Barron spaces and `SIREN` wave networks):**
   Different frequencies $\omega \ne \omega'$ are orthogonal over a period, decomposing any signal into amplitude, frequency, and phase.

---

## 7.2 Escaping the Curse of Dimensionality: Spectral Smoothness ($C\_f$) and Depth ($2^D$)

### Why Naive Grid Approximation Fails in High Dimensions ($\varepsilon^{-d}$)
Suppose we want to approximate an arbitrary $1$-Lipschitz function $f: [0, 1]^d \to \mathbb{R}$ to accuracy $\varepsilon = 0.1$ using a lookup table (or local histogram / nearest-neighbor grid):
- In $d = 1$ dimension, we slice $[0, 1]$ into $1/\varepsilon = 10$ bins.
- In $d = 2$ dimensions, a $10 \times 10$ grid requires $10^2 = 100$ bins.
- In $d = 100$ dimensions, a grid of resolution $0.1$ per axis requires $\varepsilon^{-d} = 10^{100}$ bins, exceeding the $10^{80}$ atoms in the observable universe.

This exponential explosion $M \asymp \varepsilon^{-d}$ is Richard Bellman's **Curse of Dimensionality**. A model can learn in $d = 100$ dimensions only when the target function $f$ has **structural regularity** that avoids filling the entire $d$-dimensional cube:

1. **Spectral Smoothness (Barron's Theorem):** If $f(x)$ has a Fourier transform $\hat{f}(\boldsymbol{\omega})$ whose frequency-weighted $L^1$ norm $C\_f = \int \|\boldsymbol{\omega}\|\_1 |\hat{f}(\boldsymbol{\omega})|\,d\boldsymbol{\omega}$ is finite, Andrew Barron (1993) proved that a superposition of $m$ ridge atoms $\sum\_{j=1}^m a\_j \sigma(\mathbf{w}\_j^\top \mathbf{x} + b\_j)$ achieves mean-squared error:
   $$\mathbb{E}\!\left[\left(f(X) - f\_m(X)\right)^2\right] \le \frac{C\_f^2}{m}$$
   The approximation rate $O(1/m)$ is **independent of the ambient dimension $d$**.
2. **Compositional Hierarchy (Telgarsky's $2^D$ Folding):** Composing multiple layers $f\_D \circ f\_{D-1} \circ \dots \circ f\_1(x)$ produces oscillations far more compactly than a single wide layer. Consider the piecewise-linear "tent map" on $[0, 1]$:
   $$\Delta(x) = \max(0, \; 1 - |2x - 1|)$$
   - A single application $\Delta(x)$ folds the interval $[0, 1]$ once, creating $1$ peak and $2^1 = 2$ linear slopes.
   - Composing it with itself $\Delta(\Delta(x))$ folds the folded interval again, creating $2$ peaks and $2^2 = 4$ linear slopes.
   - Composing $D$ layers $\Delta^{\circ D}(x)$ uses only $O(D)$ parameters, yet creates **$2^{D-1}$ peaks and $2^D$ linear segments**.
   Because **addition adds oscillations ($m\_1 + m\_2$) whereas composition multiplies oscillations ($2^D$)**, any shallow network of depth $1$ or $2$ requires exponentially many neurons ($\Omega(2^{D/2})$) to approximate the sawtooth function generated by $D$ composed layers.

---

## 7.3 Gram Matrices, Eigenvalues, and Effective Dimension ($d\_{\mathrm{eff}}$)

Suppose a model maps $n$ input observations $x\_1, \dots, x\_n$ into $m$ feature columns via an $n \times m$ feature matrix $\Phi$, where $\Phi\_{ik} = \phi\_k(x\_i)$. To check whether those $m$ features span independent directions or merely duplicate a few variables, we form the $m \times m$ **empirical Gram (feature covariance) matrix**:

$$G = \frac{1}{n} \Phi^\top \Phi \qquad \left(G\_{jk} = \frac{1}{n}\sum\_{i=1}^n \phi\_j(x\_i)\phi\_k(x\_i)\right)$$

Let $\lambda\_1 \ge \lambda\_2 \ge \dots \ge \lambda\_m \ge 0$ be the eigenvalues of $G$ (each $\lambda\_k$ measures the variance captured along the $k$-th principal orthogonal axis). Two linear-algebra identities can be computed in $\mathcal{O}(m^2)$ time **without running an eigenvalue solver**:

1. **The Trace of $G$ equals the sum of its eigenvalues:**
   $$\mathrm{tr}(G) = \sum\_{k=1}^m G\_{kk} = \sum\_{k=1}^m \lambda\_k$$
2. **The squared Frobenius norm of $G$ equals the sum of its squared eigenvalues:**
   $$\|G\|\_F^2 = \sum\_{j=1}^m \sum\_{k=1}^m G\_{jk}^2 = \mathrm{tr}(G^2) = \sum\_{k=1}^m \lambda\_k^2$$

Taking the ratio of $(\mathrm{tr}\,G)^2$ to $\|G\|\_F^2$ gives the **Participation Ratio / Effective Dimension** (`effective_dimension` in `crates/stochology-lenses/src/machine.rs`):

$$d\_{\mathrm{eff}}(G) = \frac{(\mathrm{tr}\,G)^2}{\|G\|\_F^2} = \frac{\left(\sum\_{k=1}^m \lambda\_k\right)^2}{\sum\_{k=1}^m \lambda\_k^2} \in [1, m]$$

Two extreme cases show how $d\_{\mathrm{eff}}(G)$ measures the active dimensionality of a representation:
- **Case A (Complete Collinear Collapse, where all $m$ features are multiples of a single direction):** Only one eigenvalue is non-zero ($\lambda\_1 > 0, \; \lambda\_2 = \dots = \lambda\_m = 0$). Then:
  $$d\_{\mathrm{eff}}(G) = \frac{(\lambda\_1 + 0)^2}{\lambda\_1^2 + 0} = \mathbf{1.0}$$
  Even if $m = 10\text{,}000$, $d\_{\mathrm{eff}}$ reports that the representation is 1-dimensional.
- **Case B (Orthonormal Basis, where all $m$ features are orthogonal with equal variance $\lambda\_1 = \dots = \lambda\_m = c$):** Then:
  $$d\_{\mathrm{eff}}(G) = \frac{(m c)^2}{m c^2} = \frac{m^2 c^2}{m c^2} = \mathbf{m}$$
  The representation uses $100\%$ of its $m$ dimensions.

---

## 7.4 The Minimax Game: Why No Algorithm Can Beat the Information Floor

To judge whether an estimation error $\mathbb{E}[\|\hat{\theta} - \theta\|^2]$ is as small as the noise allows, Abraham Wald and Lucien Le Cam formulated statistical estimation as a **zero-sum game between the Statistician and Nature**:
- The **Statistician** chooses an estimator $\hat{\theta}(D\_n)$ that maps $n$ noisy observations $D\_n = (X\_1, \dots, X\_n)$ to a parameter guess.
- **Nature** chooses the true parameter $\theta \in \Theta$ to make the Statistician's error as large as possible.

The **Minimax Risk** is the error of the best possible estimator against the hardest parameter $\theta \in \Theta$:

$$\mathcal{R}\_n^\ast(\Theta) = \inf\_{\hat{\theta}} \sup\_{\theta \in \Theta} \mathbb{E}\_\theta\!\left[\|\hat{\theta}(D\_n) - \theta\|\_2^2\right]$$

### Le Cam's Two-Point Method (Intuitive Derivation)
Le Cam's two-point method establishes a lower bound on $\mathcal{R}\_n^\ast(\Theta)$ that holds for every possible algorithm (neural networks, Bayesian posteriors, or random forests):
1. Pick two candidate parameters $\theta\_0, \theta\_1 \in \Theta$ separated by distance $\Delta = \|\theta\_0 - \theta\_1\|\_2$.
2. If $\theta\_0$ and $\theta\_1$ are close enough that the KL divergence between their $n$-sample data distributions $P\_{\theta\_0}^{\otimes n}$ and $P\_{\theta\_1}^{\otimes n}$ satisfies $n \, D\_{\mathrm{KL}}(P\_{\theta\_0} \,\|\, P\_{\theta\_1}) \le \frac{1}{2}$, the two data distributions overlap heavily.
3. Because the data distributions overlap, any estimator $\hat{\theta}$ faces an unavoidable testing error probability of at least $p\_{\mathrm{err}} \ge \frac{1}{4}$, incurring squared error at least $(\Delta/2)^2$.
4. Since $D\_{\mathrm{KL}}(P\_{\theta\_0} \,\|\, P\_{\theta\_1}) \asymp \frac{\Delta^2}{\sigma^2}$, setting $n \frac{\Delta^2}{\sigma^2} \asymp 1$ gives the maximum indistinguishable separation $\Delta^2 \asymp \frac{\sigma^2}{n}$, and across $d$ independent coordinates (by Fano's / Assouad's Lemma):

$$\inf\_{\hat{\theta}} \sup\_{\theta \in \Theta} \mathbb{E}\_\theta\!\left[\|\hat{\theta}(D\_n) - \theta\|\_2^2\right] \ge c \, \frac{d\,\sigma^2}{n}$$

Together, orthonormal bases in $L^2$, compositional folding $2^D$, spectral effective dimension $d\_{\mathrm{eff}}$, and minimax information floors provide the tools needed in Chapter 8 to construct the **$(\varepsilon, \delta)$-Almost-Perfect Machine** and **First-Class Authentic Model Form**.

---

## Computational Audit in Rust

All spectral, compositional, and minimax diagnostics developed in this chapter are implemented in pure Rust in [`crates/stochology-lenses/src/machine.rs`](../crates/stochology-lenses/src/machine.rs) and [`crates/stochology-lenses/src/invariance.rs`](../crates/stochology-lenses/src/invariance.rs):
- **`effective_dimension(&gram, m)`:** Computes $\mathrm{tr}(G)$, $\|G\|\_F^2$, and $d\_{\mathrm{eff}} = (\mathrm{tr}\,G)^2 / \|G\|\_F^2$ directly from any $m \times m$ Gram matrix.
- **`TelgarskyDepthAudit`:** Evaluates iterated tent maps $\Delta^{\circ D}(x)$, counting exact linear segments $2^D$ and comparing deep vs. shallow approximation error.
- **`hermite_basis_3(z)` (`invariance.rs`):** Evaluates the probabilists' orthonormal Hermite basis $\{\psi\_1(z), \psi\_2(z), \psi\_3(z)\}$ for nonlinear invariance testing.

Run Gate `G8` and Lab 3 to verify these foundations:

~~~bash
cargo run --release --bin gates
cargo run --release --bin labs -- --lenses
~~~

## Exercises

1. **Gram-Schmidt Orthogonalization by Hand:** Let $X$ take values $\{-1, 0, +1\}$ with equal probability $1/3$ each.
   - Verify that $\psi\_0(x) = 1$ and $\psi\_1(x) = \sqrt{\frac{3}{2}}\,x$ are orthonormal: $\mathbb{E}[\psi\_0(X)^2] = 1$, $\mathbb{E}[\psi\_1(X)^2] = 1$, and $\mathbb{E}[\psi\_0(X)\psi\_1(X)] = 0$.
   - Starting from $g(x) = x^2$, subtract its projection onto $\psi\_0(x)$ ($\mathbb{E}[X^2] = \frac{2}{3}$) and normalize to construct the quadratic orthonormal atom $\psi\_2(x) = \frac{3}{\sqrt{2}}\!\left(x^2 - \frac{2}{3}\right)$.
2. **Computing Effective Dimension $d\_{\mathrm{eff}}$:** Suppose a 3-feature model has diagonal Gram eigenvalues $\boldsymbol{\lambda} = (9, 1, 0)$.
   - Compute $\mathrm{tr}(G) = \sum \lambda\_k$ and $\|G\|\_F^2 = \sum \lambda\_k^2$.
   - Show that $d\_{\mathrm{eff}}(G) = \frac{10^2}{82} \approx 1.22$, revealing that despite having $3$ nominal features, the representation effectively uses only $1.22$ dimensions.
3. **Telgarsky Sawtooth Counting:** Trace the iterated tent map $\Delta(\Delta(x))$ on $x \in \{0, 0.25, 0.50, 0.75, 1.0\}$ where $\Delta(x) = 1 - |2x - 1|$. How many times does $\Delta(\Delta(x))$ cross the horizontal line $y = 0.5$? How many crossings does $\Delta^{\circ 10}(x)$ have?

