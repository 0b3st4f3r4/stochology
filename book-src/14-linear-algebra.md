# Linear Algebra

<figure class="stoch-figure stoch-logo-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="660" height="132" viewBox="0 0 660 132" fill="none" role="img" aria-label="Stochology CH14 · SPECTRA — Linear Operators And Eigenmodes">
  <defs>
    <linearGradient id="ch14-ring" x1="2" y1="78" x2="386" y2="78" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="38%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="68%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <radialGradient id="ch14-glow" cx="50%" cy="50%" r="50%">
      <stop offset="0%" stop-color="#38bdf8" stop-opacity="0.25"/>
      <stop offset="50%" stop-color="#2563eb" stop-opacity="0.14"/>
      <stop offset="82%" stop-color="#fbbf24" stop-opacity="0.08"/>
      <stop offset="100%" stop-color="#090e1a" stop-opacity="0"/>
    </radialGradient>
    <linearGradient id="ch14-triad-stroke" x1="18" y1="24" x2="110" y2="104" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.85"/>
      <stop offset="50%" stop-color="var(--logo-gold, #f59e0b)" stop-opacity="0.90"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.85"/>
    </linearGradient>
    <linearGradient id="ch14-fill" x1="22" y1="30" x2="106" y2="98" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)" stop-opacity="0.26"/>
      <stop offset="48%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.28"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.26"/>
    </linearGradient>
    <linearGradient id="ch14-ink-curve" x1="20" y1="66" x2="108" y2="66" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-ink, #0f172a)"/>
      <stop offset="50%" stop-color="var(--logo-sub, #1d4ed8)"/>
      <stop offset="100%" stop-color="var(--logo-ink, #0f172a)"/>
    </linearGradient>
    <radialGradient id="ch14-pupil" cx="45%" cy="42%" r="58%">
      <stop offset="0%" stop-color="#020617"/>
      <stop offset="68%" stop-color="#0f172a"/>
      <stop offset="100%" stop-color="#451a03"/>
    </radialGradient>
    <linearGradient id="ch14-gold-mira" x1="54" y1="48" x2="84" y2="82" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#fef08a"/>
      <stop offset="45%" stop-color="#fbbf24"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <radialGradient id="ch14-gold-core" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#ffffff"/>
      <stop offset="35%" stop-color="#fde047"/>
      <stop offset="100%" stop-color="#d97706"/>
    </radialGradient>
    <radialGradient id="ch14-node-well" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="var(--logo-well-hi, var(--elevated-bg, #ffffff))"/>
      <stop offset="100%" stop-color="var(--logo-node-bg, #e2e8f0)"/>
    </radialGradient>
    <linearGradient id="ch14-ring-epi" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#7dd3fc"/>
      <stop offset="50%" stop-color="#2563eb"/>
      <stop offset="100%" stop-color="#1e3a8a"/>
    </linearGradient>
    <radialGradient id="ch14-core-epi" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#bae6fd"/>
      <stop offset="45%" stop-color="#3b82f6"/>
      <stop offset="100%" stop-color="#1d4ed8"/>
    </radialGradient>
    <linearGradient id="ch14-ring-vital" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#6ee7b7"/>
      <stop offset="50%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#065f46"/>
    </linearGradient>
    <radialGradient id="ch14-core-vital" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#a7f3d0"/>
      <stop offset="45%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#047857"/>
    </radialGradient>
    <linearGradient id="ch14-ring-gov" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#fda4af"/>
      <stop offset="50%" stop-color="#e11d48"/>
      <stop offset="100%" stop-color="#881337"/>
    </linearGradient>
    <radialGradient id="ch14-core-gov" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#fecdd3"/>
      <stop offset="45%" stop-color="#f43f5e"/>
      <stop offset="100%" stop-color="#be123c"/>
    </radialGradient>
  </defs>
  <g>
    <circle cx="64" cy="66" r="48" fill="url(#ch14-glow)"/>
    <circle cx="64" cy="66" r="42" stroke="var(--logo-ticks, #64748b)" stroke-width="1.2" stroke-dasharray="3 3" stroke-opacity="0.45"/>
    <ellipse cx="64" cy="66" rx="40" ry="18" transform="rotate(-28 64 66)" fill="url(#ch14-fill)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.4"/>
    <line x1="28" y1="85" x2="100" y2="47" stroke="var(--logo-green, #10b981)" stroke-width="2.6" stroke-linecap="round"/>
    <line x1="55" y1="49" x2="73" y2="83" stroke="var(--logo-crimson, #e11d48)" stroke-width="2.4" stroke-linecap="round"/>
    <circle cx="100.00" cy="47.00" r="6.55" fill="url(#ch14-node-well)" stroke="url(#ch14-ring-vital)" stroke-width="2.42"/>
    <circle cx="100.00" cy="47.00" r="2.81" fill="url(#ch14-core-vital)"/>
    <circle cx="55.00" cy="49.00" r="6.22" fill="url(#ch14-node-well)" stroke="url(#ch14-ring-gov)" stroke-width="2.29"/>
    <circle cx="55.00" cy="49.00" r="2.66" fill="url(#ch14-core-gov)"/>
    <g>
      <circle cx="64.00" cy="66.00" r="6.66" fill="var(--logo-node-bg, #e2e8f0)"/>
      <circle cx="64.00" cy="66.00" r="5.76" fill="url(#ch14-pupil)" stroke="url(#ch14-gold-mira)" stroke-width="2.16"/>
      <line x1="55.36" y1="66.00" x2="60.22" y2="66.00" stroke="url(#ch14-gold-mira)" stroke-width="2.16" stroke-linecap="round"/>
      <line x1="67.78" y1="66.00" x2="72.64" y2="66.00" stroke="url(#ch14-gold-mira)" stroke-width="2.16" stroke-linecap="round"/>
      <line x1="64.00" y1="57.36" x2="64.00" y2="62.22" stroke="url(#ch14-gold-mira)" stroke-width="2.16" stroke-linecap="round"/>
      <line x1="64.00" y1="69.78" x2="64.00" y2="74.64" stroke="url(#ch14-gold-mira)" stroke-width="2.16" stroke-linecap="round"/>
      <circle cx="64.00" cy="66.00" r="1.98" fill="url(#ch14-gold-core)"/>
    </g>
  </g>
  <g transform="translate(148, 0)">
    <text x="2" y="27" fill="var(--logo-sub, #1d4ed8)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.2">PART II · THE BASES · MATHEMATICS</text>
    <text x="498" y="27" text-anchor="end" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.0">CH14 · SPECTRA</text>
    <text x="0" y="59" fill="var(--logo-ink, #0f172a)" font-family="system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif" font-size="20.2" font-weight="800" letter-spacing="1.15">LINEAR OPERATORS AND EIGENMODES</text>
    <line x1="2" y1="78" x2="384" y2="78" stroke="url(#ch14-ring)" stroke-width="1.6" stroke-opacity="0.82"/>
    <circle cx="400" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.0"/>
    <circle cx="400" cy="78" r="1.6" fill="var(--logo-blue, #2563eb)"/>
    <circle cx="415" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="430" cy="78" r="6.2" stroke="var(--logo-gold, #f59e0b)" stroke-width="1.5" stroke-dasharray="2.2 1.6"/>
    <circle cx="430" cy="78" r="4.0" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.2"/>
    <circle cx="430" cy="78" r="1.8" fill="var(--logo-gold, #f59e0b)"/>
    <circle cx="445" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-green, #10b981)" stroke-width="2.0"/>
    <circle cx="445" cy="78" r="1.6" fill="var(--logo-green, #10b981)"/>
    <circle cx="460" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="475" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="490" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <text x="2" y="104" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.8" font-weight="600" letter-spacing="0.65">PROJECTION MATRICES · COVARIANCE GEOMETRY · SPECTRAL STABILITY RADIUS</text>
  </g>
  </svg>
</figure>

> Sixth chapter of the mathematical block in Part II. The subject is what a matrix does, and why almost
> every model in use is a linear map with a nonlinearity bolted on.

## By the end of this chapter you can

1. Read a matrix as a map between spaces rather than as a table of numbers.
2. Say what rank, kernel and image mean, and why a rank-deficient design cannot answer the question it
   was built for.
3. Explain what an eigenvector is and why directions that survive a map unchanged matter for stability.

## 1. Vectors, Subspaces, and Dimension

A credit-scoring desk inherits a table with a large nominal column count $p$ and advertises a
"$p$-dimensional risk model". An auditor finds most of those columns to be currency conversions, rolling sums,
linear combinations of the first few. Storage bills and matrix dimensions count columns. Inference counts
independent directions of variation. Linear algebra starts by separating the nominal width of a table from the
dimension of the subspace it spans.

A real vector space $\mathbb{R}^n$ is a set of $n$-tuples closed under vector addition ($\mathbf{u} + \mathbf{v}$)
and scalar multiplication ($c\mathbf{u}$). A **linear subspace** $V \subseteq \mathbb{R}^n$ is a subset closed
under both operations.

A collection of vectors $\lbrace\mathbf{v}\_1, \dots, \mathbf{v}\_k\rbrace$ is linearly independent if $\sum\_{i=1}^k c\_i \mathbf{v}\_i = \mathbf{0}$
implies $c\_1 = \cdots = c\_k = 0$. A **basis** is a minimal linearly independent spanning set for $V$. The number
of elements in any basis defines the **dimension** $\dim(V)$.

In data architecture the column count is the nominal feature dimension $p$. The informational content is governed
by the dimension of the spanned subspace. Adding correlated features raises the count without expanding that
dimension.

## 2. Matrices as Linear Operators and the Rank-Nullity Theorem

An $m \times n$ matrix $A$ is a linear operator from an input domain $\mathbb{R}^n$ into a codomain
$\mathbb{R}^m$: $\mathbf{x} \mapsto A\mathbf{x}$. Two subspaces fix its geometry:

- The **kernel** (or null space): $\ker(A) = \lbrace\mathbf{x} \in \mathbb{R}^n : A\mathbf{x} = \mathbf{0}\rbrace$, the
  set of inputs that the operator annihilates.
- The **image** (or column space): $\mathrm{im}(A) = \lbrace{}A\mathbf{x} : \mathbf{x} \in \mathbb{R}^n\rbrace \subseteq \mathbb{R}^m$,
  the subspace of reachable outputs.

By the **rank-nullity theorem**:

$$\dim\bigl(\ker(A)\bigr) + \dim\bigl(\mathrm{im}(A)\bigr) = n.$$

The dimension of the image is the **rank** of the matrix, $\mathrm{rank}(A)$.

A design matrix $X \in \mathbb{R}^{n \times p}$ in a regression or prediction problem may have linearly dependent
columns, $\mathrm{rank}(X) < p$, which means $\dim(\ker(X)) > 0$. The kernel then holds non-trivial parameter vectors
$\mathbf{\beta}\_0 \ne \mathbf{0}$ with $X\mathbf{\beta}\_0 = \mathbf{0}$, and the data cannot distinguish
$\mathbf{\beta}$ from $\mathbf{\beta} + \mathbf{\beta}\_0$. An algorithm that reports a unique point estimate in a
rank-deficient design is imposing an arbitrary regularizer, not learning from the data.

A $p = 3$ compensation audit makes the kernel tangible. An archive records base salary $x\_{i,1}$, annual bonus
$x\_{i,2}$, and total pay $x\_{i,3} = x\_{i,1} + x\_{i,2}$ across $n$ employees. The third column is the sum of the
first two, so the column space $\mathrm{im}(X) \subseteq \mathbb{R}^n$ has dimension $\mathrm{rank}(X) = 2 < 3$, and
rank-nullity guarantees a one-dimensional kernel $\dim(\ker(X)) = 3 - 2 = 1$ spanned by the null vector
$\mathbf{\beta}\_0 = (1, 1, -1)^\top$. For every scalar $c \in \mathbb{R}$, shifting the regression weights from
$\mathbf{\beta}$ to $\mathbf{\beta} + c(1, 1, -1)^\top$ leaves every predicted value in the archive unchanged:
$X(\mathbf{\beta} + c\mathbf{\beta}\_0) = X\mathbf{\beta} + c\mathbf{0} = X\mathbf{\beta}$.

Ask a Cholesky factorization to factor the resulting singular Gram matrix $G = X^\top X$ and the third diagonal
pivot collapses to zero. The factorization fails, and the failure carries information: it refuses to invent a
unique attribution along a direction the design matrix annihilates. A ridge penalty shifts all eigenvalues by
$\lambda > 0$ and selects the minimum-norm solution orthogonal to $\ker(X)$.

## 3. The Gram Matrix and Effective Dimension

A labor-ministry briefing cites the earnings records of the National Supported Work evaluation as though the ministry
held that many independent measurements of earnings — the comparison LaLonde (1986) posed and Dehejia & Wahba (1999)
re-estimated against its experimental benchmark. Interviews drawn in clusters of average size $\bar{m}$ with
intra-cluster correlation $\rho$ carry the information of $n/(1 + (\bar{m} - 1)\rho)$ independent ones (Kish 1965), so the
archive behaves like a sample smaller by that factor. The missing share of the sample was not lost to file
corruption. Within-cluster correlation absorbed it, reducing the effective rank of the observation operator.

The geometry of a feature matrix $X$ sits in its **Gram matrix** $G = X^T X \in \mathbb{R}^{p \times p}$. If $G$ is
invertible, ordinary least squares has a unique solution $\hat{\mathbf{\beta}} = G^{-1} X^T \mathbf{y}$. Under high
collinearity $G$ is ill-conditioned: its smallest eigenvalues approach zero, $(X^T X)^{-1}$ explodes, and parameter
variance inflates.

The true dimensionality of correlated data is captured without column counts by the **effective dimension**
(Roy & Vetterli, 2007), defined from the eigenvalues $\lambda\_1 \ge \cdots \ge \lambda\_p \ge 0$ of $G$:

$$d\_{\mathrm{eff}} = \frac{\bigl(\mathrm{tr}(G)\bigr)^2}{\Vert{}G\Vert\_F^2} = \frac{\left(\sum\_{i=1}^p \lambda\_i\right)^2}{\sum\_{i=1}^p \lambda\_i^2}.$$

With all $p$ features orthogonal and equal in variance ($\lambda\_i = \lambda$) this gives
$d\_{\mathrm{eff}} = p^2 \lambda^2 / (p \lambda^2) = p$. With features perfectly collinear ($\lambda\_1 = \mathrm{tr}(G), \lambda\_{i > 1} = 0$)
it gives $d\_{\mathrm{eff}} = 1$. The quantity says how far multicollinearity collapses degrees of freedom.

The participation-ratio geometry of $d\_{\mathrm{eff}}$ works along both axes of an archive matrix $X \in \mathbb{R}^{n \times p}$,
across rows and across columns:

- **Row-space effective sample size and the Kish floor:** $n$ survey units carry inverse-propensity Horvitz–Thompson weights $w\_i = 1/\pi\_i$ (Horvitz & Thompson 1952; Kish 1965), and the same participation ratio applies to the diagonal weight operator $W = \mathrm{diag}(w\_1, \dots, w\_n)$:

  $$n\_{\mathrm{eff}}^{\mathrm{Kish}} = \frac{\bigl(\mathrm{tr}(W)\bigr)^2}{\Vert{}W\Vert\_F^2} = \frac{\left(\sum\_{i=1}^n w\_i\right)^2}{\sum\_{i=1}^n w\_i^2} = \frac{n}{\mathrm{DEFF}\_{\mathrm{Kish}}}.$$

  In the **homogeneous-stratum limit**, where every unit has equal inclusion probability ($\pi\_i \equiv \pi\_0$), all weights are equal, $\mathrm{DEFF}\_{\mathrm{Kish}} = 1$, and the nominal sample size equals the effective dimension ($n\_{\mathrm{eff}} = n$). Dispersion in the weights pushes the design effect above one and compresses the nominal row count. Keynes's evidential weight $V = n\_{\mathrm{eff}} / (n\_{\mathrm{eff}} + n\_0)$ (Keynes 1921) falls with it, and rises toward one only as the effective sample grows large against the reference scale $n\_0$.
- **Cluster design effect:** Group observations into demographic clusters (Moulton 1986) and the intracluster correlation divides the nominal row count by $\mathrm{DEFF}\_{\mathrm{cluster}}$. Double the design effect and $n\_{\mathrm{eff}}$ halves without a single row leaving the file. An archive can grow in rows while losing evidential weight.
- **Counterexample: when column collinearity defeats a perfect sampling design:** An archive with homogeneous independent rows ($\mathrm{DEFF} = 1$, so $n\_{\mathrm{eff}} = n$) can still carry two standardized regressors with sample correlation $r$. The $2 \times 2$ correlation Gram matrix $G = \bigl[\begin{smallmatrix} 1 & r \\\\ r & 1 \end{smallmatrix}\bigr]$ has eigenvalues $\lambda\_1 = 1 + r$ and $\lambda\_2 = 1 - r$, giving eigenvalue ratio $\lambda\_2 / \lambda\_1 = (1 - r)/(1 + r)$ and effective feature dimension $d\_{\mathrm{eff}} = 4/\bigl((1 + r)^2 + (1 - r)^2\bigr) = 2/(1 + r^2)$, which tends to $1$ as $r \to 1$ despite $p = 2$. Here the bottleneck sits in the columns of $G$, not in the rows. More interviews leave $\lambda\_2 / \lambda\_1$ ill-conditioned, and the remedy is dropping or orthogonalizing the redundant feature, not redesigning the survey.

## 4. Orthogonal Projections and Least Squares

Given an observation vector $\mathbf{y} \in \mathbb{R}^n$ and the subspace $S = \mathrm{im}(X)$ spanned by the regressors,
the point of $S$ that minimizes the Euclidean distance $\Vert\mathbf{y} - \hat{\mathbf{y}}\Vert\_2$ is the
**orthogonal projection** of $\mathbf{y}$ onto $S$:

$$\hat{\mathbf{y}} = P\_X \mathbf{y}, \qquad P\_X = X(X^T X)^{-1} X^T.$$

The projection operator $P\_X$ is symmetric ($P\_X^T = P\_X$) and idempotent ($P\_X^2 = P\_X$). The residual
$\mathbf{e} = \mathbf{y} - \hat{\mathbf{y}} = (I - P\_X)\mathbf{y}$ lies in the orthogonal complement $S^\perp$:
$X^T \mathbf{e} = \mathbf{0}$.

Because least squares is an orthogonal projection, the residual sum of squares can only fall or stay level when new
columns are appended to $X$. Uninformative features mechanically enlarge the subspace, driving in-sample residuals
to zero while out-of-sample prediction error inflates. That is the linear geometry of overfitting.

Pythagoras makes the monotonicity a statement about lengths. Because $P\_X \mathbf{y} \in S$ and $(I - P\_X)\mathbf{y} \in S^\perp$
are orthogonal, $\Vert\mathbf{y}\Vert\_2^2 = \Vert{}P\_X \mathbf{y}\Vert\_2^2 + \Vert\mathbf{y} - P\_X \mathbf{y}\Vert\_2^2$. Appending a
column $\mathbf{x}\_{p+1}$ enlarges the subspace to $S' = \mathrm{im}([X\mkern5mu\mathbf{x}\_{p+1}]) \supseteq S$, so
the minimum distance from $\mathbf{y}$ to $S'$ cannot exceed its distance to $S$. At saturation, where $p = n$
linearly independent noise columns span all of $\mathbb{R}^n$, the projection becomes the identity matrix
$P\_X = I\_n$: the in-sample residual $\Vert\mathbf{y} - \hat{\mathbf{y}}\Vert\_2$ goes to $0$ and nothing about the
signal has been learned.

## 5. High-Dimensional Geometry and Concentration of Measure

When dimensionality $d$ is large, Euclidean geometry behaves in ways that confound low-dimensional intuition:

1. **Surface concentration**: Inside a $d$-dimensional hypersphere of radius $R$, the fraction of volume within a concentric sphere of radius $(1 - \epsilon)R$ is $(1 - \epsilon)^d \le e^{-\epsilon d}$. For large $d$, virtually $100\unicode{x25}$ of the volume of the sphere sits in a paper-thin shell of thickness $\mathcal{O}(R/d)$ against the boundary.
2. **Equatorial concentration**: On a high-dimensional sphere almost all the surface area lies within a narrow equatorial band of distance $\mathcal{O}(1/\sqrt{d})$ from any arbitrary equator. Two random vectors drawn independently and uniformly from the sphere are almost orthogonal with high probability: $\langle \mathbf{u}, \mathbf{v} \rangle \approx 0$.
3. **The Johnson–Lindenstrauss Lemma** (Johnson & Lindenstrauss 1984; Dasgupta & Gupta 2003): For any set of $N$ points in arbitrary dimension $\mathbb{R}^D$, any distortion parameter $\epsilon \in (0, 1)$, and any integer dimension

   $$k \ge \frac{4}{\epsilon^2/2 - \epsilon^3/3}\mkern3mu \ln N$$

   (asymptotically $8 \ln N / \epsilon^2$ for small $\epsilon$), there exists a linear mapping into $\mathbb{R}^k$ that preserves all pairwise squared — and hence unsquared — Euclidean distances within a factor of $1 \pm \epsilon$. Dimensionality reduction succeeds because the geometry of a finite point set is constrained by its size $N$, not by the ambient feature dimension $D$.

A direct moment calculation shows how quickly norm concentration sets in. Let
$\mathbf{x} = (x\_1, \dots, x\_d)^\top \in \mathbb{R}^d$ have independent coordinates with mean $0$, unit variance,
and fourth moment $\mathbb{E}[x\_i^4] = 3$. The squared Euclidean norm $\Vert\mathbf{x}\Vert\_2^2 = \sum\_{i=1}^d x\_i^2$ then
has expectation $\mathbb{E}[\Vert\mathbf{x}\Vert\_2^2] = d$ and variance $\mathrm{Var}(\Vert\mathbf{x}\Vert\_2^2) = 2d$. A first-order
Taylor expansion of the square root around $d$ puts the relative standard deviation of $\Vert\mathbf{x}\Vert\_2 / \sqrt{d}$
at:

$$\frac{\mathrm{SD}(\Vert\mathbf{x}\Vert\_2)}{\sqrt{d}} \approx \frac{\sqrt{2d}}{2d} = \frac{1}{\sqrt{2d}}.$$

In high ambient dimension $d$ random feature vectors lie almost on a deterministic spherical shell of radius
$\sqrt{d}$, and the relative fluctuation shrinks as $1/\sqrt{2d}$. An epistemic margin penalizes unpruned
specification dimensionality for that reason. It does not reward raw feature count.

## 6. Separability over the two-element field, and why parity is not affine

One example fixes what "linear model" means more sharply than any amount of rank algebra, and it is the example
that interrupted the first wave of connectionist research: the exclusive disjunction.

Work over the two-element field $\mathbb{F}\_2 = \lbrace0, 1\rbrace$ with addition modulo two. A dichotomy on $n$ binary
features is **affinely separable** when there exist a weight vector $w \in \mathbb{R}^n$ and a threshold
$\theta$ such that $w \cdot x \ge \theta$ exactly on the positive class. Counting over $\mathbb{F}\_2$ gives
the structural statement: the map $x \mapsto w \cdot x$ is $\mathbb{F}\_2$-linear only when every weight is
itself in $\lbrace0, 1\rbrace$, and then the image is a linear functional whose value is fixed by the parity of the
coordinates that carry unit weight. The parity predicate

$$\psi\_{\mathrm{PARITY}}(x) = \bigoplus\_{i=1}^{n} x\_i$$

is therefore not representable by any single threshold element for $n \ge 2$, while every conjunction and every
disjunction is. Separability is not a matter of having enough data or enough training time. It is a property of the
image of the operator. What a hidden layer adds is a change of basis in which the target becomes separable: the
geometric content of the 1986 recovery (Rumelhart, Hinton & Williams 1986) narrated in
[Chapter 28](28-the-machine-and-the-deception.md) §5, and the reason a rank-deficient design (rank-nullity above)
cannot answer a question that needs the missing direction.

The rank constraint also explains the cheapest modern form of adaptation. In Low-Rank Adaptation (LoRA; Hu et al. 2021),
a low-rank update $A \mapsto A + UV^{\top}$ with $U \in \mathbb{R}^{m \times r}$ and $V \in \mathbb{R}^{n \times r}$ of width $r \ll \min(m, n)$ has rank at most $r$ (parameterized by $(m + n)r$ scalars rather than $m n$) and restricts the change of output $x \mapsto UV^{\top}x$ to an image subspace of dimension at most $r$. That is why such adapters can steer a large operator without
retraining it, and why they cannot repair a deficiency that lives outside the image of that subspace. Rank is not
bookkeeping. It is the size of the set of questions the update can change the answer to.

The same reading covers the modern reduction step. Principal Component Analysis (PCA; Hotelling 1933) and Uniform
Manifold Approximation and Projection (UMAP; McInnes, Healy & Melville 2018) are projections onto a subspace or
manifold chosen to preserve variance or local neighbourhood distances, and the Johnson–Lindenstrauss bound quoted
above states the price in dimension. A projection preserves the geometry of the sampled point cloud. It cannot
create a direction the data never spanned. When a downstream model fails after reduction, the informative question
is which functional of the original space was annihilated by the kernel of the projection, not which algorithm was
used.

Counting the algebraic normal form (ANF) over $\mathbb{F}\_2^d$ pinpoints where the complexity of parity lives
(Minsky & Papert 1969; Rumelhart et al. 1986). Over $\mathbb{F}\_2$, the polynomial
$\psi\_{\mathrm{PARITY}}(x) = x\_1 \oplus x\_2 \oplus \cdots \oplus x\_d = \sum\_{i=1}^d x\_i \bmod 2$ contains exactly $d$
degree-1 monomials and zero higher-order interaction terms. Embed it in the real hypercube $\lbrace0, 1\rbrace^d \subset \mathbb{R}^d$
and the modulo-two wraparound forces every vertex to carry the opposite class label of all $d$ of its immediate
Hamming neighbours: at $d = 3$ there are $3$ ANF monomials across $2^3 = 8$ domain points ($2^{3-1} = 4$ with value
$1$ and $4$ with value $0$), and at $d = 4$ there are $4$ ANF monomials across $2^4 = 16$ domain points
($2^{4-1} = 8$ with value $1$).

No single real hyperplane $w \cdot x = \theta$ slices $\lbrace0, 1\rbrace^d$ so that adjacent vertices alternate sign in every
coordinate direction at once. Rosenblatt's update rule converges on conjunction and on disjunction, stalls well
short of exactness on exclusive disjunction, and exhibits identically zero correlation across every non-constant
order-three local feature on the four-bit hypercube until a hidden-layer basis change restores unit accuracy.

### Dimension is the rank of the design

A survey reporting a very large nominal interview count at a design effect far above one carries the information
of a sample far smaller. Dimension is the rank of the design, not the number of rows. The lesson survives in high
dimension: the norm of a random vector concentrates within a shrinking band around its expectation while the number
of directions grows far faster, so distances stop discriminating where the model becomes most confident. More rows
do not raise the rank when the clusters are the same, and more columns reduce the resolution of every distance
computed in them.

### When the relation is not linear

The thesis fails when the relation carries interaction of order greater than one, because no orthogonal projection
represents it. The rank of the design is then larger than the nominal dimension, and the effective dimension is what
must be recounted. Expand the basis, count again, and (if the count explodes) reduce the basis instead of buying more
rows. The algebraic work of solving the resulting normal equations is also where the conditioning of the recount
becomes visible. The counting rule for effective sample belongs to the survey tradition that named the design effect
(Kish 1965), and the conditioning is the other half of the story: a rank recounted on nearly collinear columns
produces a system whose solve is only as accurate as its condition number.

## Bridge

[Model, Fit, and Error](37-model-fit-and-error.md) translates orthogonal projections into the classical bias-variance
decomposition, while [World-to-Archive, Sampling, and the Big Data Paradox](45-world-to-archive-big-data-paradox.md)
evaluates effective degrees of freedom across stratified clusters. The same epistemic margin tracks effective
degrees of freedom and design effects, and it computes Kish effective sample sizes when variance inflation
collapses high-dimensional archival representations.

[Probability as a Measure](15-probability-as-a-measure.md) assembles these pieces into a single object, the triple
$(\Omega, \mathcal{F}, \mathbb{P})$, and derives conditioning, martingales, and Ville's inequality from it.
