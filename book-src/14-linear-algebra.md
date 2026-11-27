# Linear Algebra

<figure class="stoch-figure stoch-logo-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="800" height="132" viewBox="0 0 800 132" fill="none" role="img" aria-label="Stochology CH14 · SPECTRA — Linear Operators And Eigenmodes">
  <defs>
    <!-- Canonical Ring Gradient -->
    <linearGradient id="ch14-ring" x1="2" y1="78" x2="386" y2="78" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="38%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="68%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <!-- 1. Atmospheric Cosmic Field -->
    <radialGradient id="ch14-glow" cx="52%" cy="52%" r="50%">
      <stop offset="0%" stop-color="#fbbf24" stop-opacity="0.24"/>
      <stop offset="48%" stop-color="#38bdf8" stop-opacity="0.14"/>
      <stop offset="80%" stop-color="#e11d48" stop-opacity="0.08"/>
      <stop offset="100%" stop-color="#090e1a" stop-opacity="0"/>
    </radialGradient>
    <!-- 3D Spherical Node Gradients -->
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
    <!-- Stókhos Reticle Gradients -->
    <radialGradient id="ch14-pupil" cx="45%" cy="42%" r="58%">
      <stop offset="0%" stop-color="#020617"/>
      <stop offset="68%" stop-color="#0f172a"/>
      <stop offset="100%" stop-color="#451a03"/>
    </radialGradient>
    <linearGradient id="ch14-gold-mira" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#fef08a"/>
      <stop offset="45%" stop-color="#fbbf24"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <radialGradient id="ch14-gold-core" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#ffffff"/>
      <stop offset="35%" stop-color="#fde047"/>
      <stop offset="100%" stop-color="#d97706"/>
    </radialGradient>
    <!-- Ink curve gradient -->
    <linearGradient id="ch14-ink-curve" x1="0%" y1="0%" x2="100%" y2="0%">
      <stop offset="0%" stop-color="var(--logo-ink, #0f172a)"/>
      <stop offset="50%" stop-color="var(--logo-sub, #1d4ed8)"/>
      <stop offset="100%" stop-color="var(--logo-ink, #0f172a)"/>
    </linearGradient>
    <radialGradient id="ch14-tensor-fill" cx="50%" cy="50%" r="50%">
      <stop offset="0%" stop-color="#2563eb" stop-opacity="0.35"/>
      <stop offset="60%" stop-color="#10b981" stop-opacity="0.20"/>
      <stop offset="100%" stop-color="#090e1a" stop-opacity="0"/>
    </radialGradient>
    <linearGradient id="ch14-eigen-major" x1="28" y1="85" x2="100" y2="47" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#34d399"/>
      <stop offset="60%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#047857"/>
    </linearGradient>
    <linearGradient id="ch14-eigen-minor" x1="55" y1="49" x2="73" y2="83" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#fda4af"/>
      <stop offset="60%" stop-color="#e11d48"/>
      <stop offset="100%" stop-color="#881337"/>
    </linearGradient>
  </defs>
  <g>
    <circle cx="64" cy="66" r="48" fill="url(#ch14-glow)"/>
    <circle cx="64" cy="66" r="42" stroke="var(--logo-ticks, #64748b)" stroke-width="1.4" stroke-dasharray="3 3" stroke-opacity="0.45"/>
    <ellipse cx="64" cy="66" rx="40" ry="18" transform="rotate(-28 64 66)" fill="url(#ch14-tensor-fill)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.8"/>
    <line x1="28" y1="85" x2="89.21" y2="52.70" stroke="url(#ch14-eigen-major)" stroke-width="3.4" stroke-linecap="round"/>
    <path d="M 83.48 51.74 L 90.98 51.76 L 86.77 57.96" stroke="url(#ch14-eigen-major)" stroke-width="2.8" stroke-linecap="round" stroke-linejoin="round"/>
    <line x1="55" y1="49" x2="72.30" y2="81.67" stroke="url(#ch14-eigen-minor)" stroke-width="2.8" stroke-linecap="round"/>
    <path d="M 73.01 76.00 L 73 83 L 67.20 79.08" stroke="url(#ch14-eigen-minor)" stroke-width="2.4" stroke-linecap="round" stroke-linejoin="round"/>
    <circle cx="100" cy="47" r="8.5" fill="url(#ch14-node-well)" stroke="url(#ch14-ring-vital)" stroke-width="3.0"/>
    <circle cx="100" cy="47" r="3.8" fill="url(#ch14-core-vital)"/>
    <circle cx="55" cy="49" r="8.5" fill="url(#ch14-node-well)" stroke="url(#ch14-ring-gov)" stroke-width="3.0"/>
    <circle cx="55" cy="49" r="3.8" fill="url(#ch14-core-gov)"/>
    <g>
      <circle cx="64" cy="66" r="7.4" fill="var(--logo-node-bg, #e2e8f0)"/>
      <circle cx="64" cy="66" r="6.4" fill="url(#ch14-pupil)" stroke="url(#ch14-gold-mira)" stroke-width="2.4"/>
      <line x1="54.40" y1="66.00" x2="59.80" y2="66.00" stroke="url(#ch14-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="68.20" y1="66.00" x2="73.60" y2="66.00" stroke="url(#ch14-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="64.00" y1="56.40" x2="64.00" y2="61.80" stroke="url(#ch14-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="64.00" y1="70.20" x2="64.00" y2="75.60" stroke="url(#ch14-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <circle cx="64" cy="66" r="2.2" fill="url(#ch14-gold-core)"/>
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

> What a matrix does, and why almost every model in use is a linear map with a nonlinearity bolted on.

## By the end of this chapter you can

1. Read a matrix as a map between spaces rather than as a table of numbers.
2. Say what rank, kernel and image mean, and why a rank-deficient design cannot answer the question it
   was built for.
3. Explain what an eigenvector is and why directions that survive a map unchanged matter for stability.

## 1. Vectors, Subspaces, and Dimension

A credit-scoring team inherits a spreadsheet with $p$ columns and advertises a "$p$-dimensional risk model." An audit reveals that most columns are merely currency conversions, rolling sums, and blends of the first few columns. Storage bills count columns; real geometry counts independent directions.

Start in two dimensions. A **2D vector** $\mathbf{x} = (x\_1, x\_2)^\top$ is a column list of two numbers, pictured as an arrow from the origin $(0, 0)^\top$ to coordinates $(x\_1, x\_2)$. Take two concrete vectors: $\mathbf{u} = (2, 1)^\top$ and $\mathbf{v} = (1, 3)^\top$. Two operations combine them:

- **Vector addition** adds matching entries: $\mathbf{u} + \mathbf{v} = (2 + 1,\; 1 + 3)^\top = (3, 4)^\top$.
- **Scalar multiplication** stretches or flips an arrow by a number $c \in \mathbb{R}$: $2\mathbf{u} = (4, 2)^\top$.

Putting both together creates a **linear combination** $c\_1 \mathbf{u} + c\_2 \mathbf{v}$. For instance, $2\mathbf{u} - \mathbf{v} = (4, 2)^\top - (1, 3)^\top = (3, -1)^\top$.

To measure lengths and angles, compute the **dot product** (inner product) $\langle \mathbf{u}, \mathbf{v}\rangle = u\_1 v\_1 + u\_2 v\_2$. On our pair, $\langle \mathbf{u}, \mathbf{v}\rangle = 2(1) + 1(3) = 5$. Comparing $\mathbf{u}$ with itself gives squared Euclidean length $\Vert \mathbf{u}\Vert\_2^2 = \langle \mathbf{u}, \mathbf{u}\rangle = 2^2 + 1^2 = 5$. When the dot product equals zero, two non-zero vectors meet at a right angle (**orthogonal**): check $\langle (1, 0)^\top, (0, 1)^\top\rangle = 1(0) + 0(1) = 0$. Taking the dot product of $\mathbf{u} = (2, 1)^\top$ with the unit axis $(1, 0)^\top$ simply projects $\mathbf{u}$ onto the horizontal axis and reads off coordinate $2$.

Now test when vectors add genuine directions rather than duplicating old ones. The standard vectors $\mathbf{e}\_1 = (1, 0)^\top$ and $\mathbf{e}\_2 = (0, 1)^\top$ are **linearly independent** because $c\_1 \mathbf{e}\_1 + c\_2 \mathbf{e}\_2 = (c\_1, c\_2)^\top = (0, 0)^\top$ forces $c\_1 = c\_2 = 0$; they form a **basis** covering all of $\mathbb{R}^2$, giving dimension $2$. By contrast, $(1, 2)^\top$ and $(2, 4)^\top$ are linearly dependent because $2(1, 2)^\top - 1(2, 4)^\top = (0, 0)^\top$: they span only a $1$-dimensional straight line (**subspace** of dimension $1$).

In general, real vector space $\mathbb{R}^n$ collects all $n$-tuples closed under vector addition ($\mathbf{u} + \mathbf{v}$) and scalar multiplication ($c\mathbf{u}$), and a **linear subspace** $V \subseteq \mathbb{R}^n$ is any subset closed under both. Vectors $\lbrace\mathbf{v}\_1, \dots, \mathbf{v}\_k\rbrace$ are linearly independent if $\sum\_{i=1}^k c\_i \mathbf{v}\_i = \mathbf{0}$ implies $c\_1 = \cdots = c\_k = 0$. A **basis** is a linearly independent set that spans all of $V$, and its vector count defines the **dimension** $\dim(V)$. Duplicating or blending columns widens $p$ without raising dimension.

## 2. Matrices as Linear Operators and the Rank-Nullity Theorem

A matrix is a machine that transforms input vectors $\mathbf{x}$ into output vectors $A\mathbf{x}$. Test a tiny $2 \times 2$ matrix $A$ acting on vector $\mathbf{x} = (3, 1)^\top$:

$$A = \left[\begin{matrix} 1 & 2 \\\\ 2 & 4 \end{matrix}\right], \qquad A\mathbf{x} = \left[\begin{matrix} 1(3) + 2(1) \\\\ 2(3) + 4(1) \end{matrix}\right] = \left[\begin{matrix} 5 \\\\ 10 \end{matrix}\right].$$

Read that multiplication aloud: each entry of $A\mathbf{x}$ is the dot product of one row of $A$ with $\mathbf{x}$. Equivalently, $A\mathbf{x}$ equals $3$ times the first column $(1, 2)^\top$ plus $1$ times the second column $(2, 4)^\top$.

Look closely at what $A$ can output and what it erases on that same $2 \times 2$ example:

- Every output $A\mathbf{x} = (x\_1 + 2x\_2)(1, 2)^\top$ lands on the single $1$-dimensional line through $(1, 2)^\top$. That reachable subspace is the **image** $\mathrm{im}(A)$, with dimension $\mathrm{rank}(A) = 1$.
- Feed in $\mathbf{x} = (-2, 1)^\top$ and you get $A(-2, 1)^\top = (-2 + 2,\; -4 + 4)^\top = (0, 0)^\top$. The entire $1$-dimensional line of multiples $c(-2, 1)^\top$ collapses to zero, forming the **kernel** (null space) $\ker(A)$, with $\dim(\ker(A)) = 1$.
- Notice also a preview of §7's **stretch directions**: feeding $(1, 2)^\top$ into $A$ gives $A(1, 2)^\top = (5, 10)^\top = 5(1, 2)^\top$ (pure scaling by $5$), while $(-2, 1)^\top$ scales by $0$.

Notice that $\dim(\ker(A)) + \mathrm{rank}(A) = 1 + 1 = 2$, equaling the number of input coordinates. That balance holds for every $m \times n$ linear operator $A : \mathbb{R}^n \to \mathbb{R}^m$, with **kernel** $\ker(A) = \lbrace\mathbf{x} \in \mathbb{R}^n : A\mathbf{x} = \mathbf{0}\rbrace$ and **image** $\mathrm{im}(A) = \lbrace A\mathbf{x} : \mathbf{x} \in \mathbb{R}^n \rbrace \subseteq \mathbb{R}^m$.

Systematic elimination on rectangular tables has deep planetary roots. Chapter Eight (*Fāngchéng*, "rectangular arrays") of *The Nine Chapters on the Mathematical Art*, explained in Liu Hui's third-century commentary, solved linear systems by column reduction with signed counting rods. That method predated European Gaussian elimination by centuries (Shen, Crossley & Lun 1999; Chemla & Guo 2004). Modern theory lifts those table steps into invariant subspace dimensions via the **rank-nullity theorem**, where $\mathrm{rank}(A) = \dim(\mathrm{im}(A))$:

$$\dim\bigl(\ker(A)\bigr) + \mathrm{rank}(A) = n.$$

Read that aloud: the dimension erased into the kernel plus the dimension surviving in the image always equals the input dimension $n$.

Apply this to a data table $X \in \mathbb{R}^{n \times p}$ multiplying parameter vector $\mathbf{\beta} \in \mathbb{R}^p$ to predict outcomes $\mathbf{y} \in \mathbb{R}^n$. If columns are linearly dependent, $\mathrm{rank}(X) < p$ forces $\dim(\ker(X)) > 0$. Any non-zero $\mathbf{\beta}\_0 \in \ker(X)$ satisfies $X(\mathbf{\beta} + \mathbf{\beta}\_0) = X\mathbf{\beta}$, leaving data utterly blind between $\mathbf{\beta}$ and $\mathbf{\beta} + \mathbf{\beta}\_0$. An algorithm claiming a unique estimate on a rank-deficient design is adding an unstated preference, not learning from records.

Check a $p = 3$ payroll audit. Suppose an archive records base salary $x\_{i,1}$, annual bonus $x\_{i,2}$, and total pay $x\_{i,3} = x\_{i,1} + x\_{i,2}$ across $n \ge 2$ employees whose salaries and bonuses vary independently. Here $\mathrm{rank}(X) = 2 < 3$, so rank-nullity guarantees a one-dimensional kernel $\dim(\ker(X)) = 3 - 2 = 1$ spanned by $\mathbf{\beta}\_0 = (1, 1, -1)^\top$: shifting $\mathbf{\beta}$ by $c(1, 1, -1)^\top$ ($c \in \mathbb{R}$) never changes any predicted pay.

Ask Cholesky factorization to decompose the singular Gram matrix $G = X^\top X$ and the third diagonal pivot hits zero, refusing to invent attribution along $\ker(X)$. Adding a small ridge penalty $\lambda\_{\mathrm{reg}} > 0$ (kept distinct from eigenvalues $\lambda\_i$ in §7) shifts $G$ to $G + \lambda\_{\mathrm{reg}} I\_p$ on identity matrix $I\_p$, giving unique regularized weights $\hat{\mathbf{\beta}}\_{\lambda\_{\mathrm{reg}}} = (X^\top X + \lambda\_{\mathrm{reg}} I\_p)^{-1} X^\top \mathbf{y} \in \ker(X)^\perp$ that minimize $\Vert\mathbf{y} - X\mathbf{\beta}\Vert\_2^2 + \lambda\_{\mathrm{reg}}\Vert\mathbf{\beta}\Vert\_2^2$. Because any $\lambda\_{\mathrm{reg}} > 0$ shrinks active singular modes $\sigma\_j > 0$ by $\sigma\_j^2 / (\sigma\_j^2 + \lambda\_{\mathrm{reg}}) < 1$, it recovers the minimum-norm solution $X^+\mathbf{y}$ only as $\lambda\_{\mathrm{reg}} \to 0^+$.

## 3. The Gram Matrix and Effective Dimension

Suppose a data table $X \in \mathbb{R}^{n \times p}$ has $3$ rows and $2$ columns, say $X = \left[\begin{matrix} 1 & 0 \\\\ 1 & 1 \\\\ 0 & 1 \end{matrix}\right]$. Multiply $X^\top$ by $X$ to form the $p \times p$ **Gram matrix** $G = X^\top X$:

$$G = X^\top X = \left[\begin{matrix} 1 & 1 & 0 \\\\ 0 & 1 & 1 \end{matrix}\right] \left[\begin{matrix} 1 & 0 \\\\ 1 & 1 \\\\ 0 & 1 \end{matrix}\right] = \left[\begin{matrix} 2 & 1 \\\\ 1 & 2 \end{matrix}\right].$$

Read $G$ aloud: every entry $(j, k)$ of $G$ is simply the dot product between column $j$ and column $k$ of $X$. Diagonal entries measure each column's squared length ($2$ and $2$), while off-diagonal entries ($1$) measure how strongly columns overlap. When columns are linearly independent, $G$ is invertible and yields unique ordinary least squares weights $\hat{\mathbf{\beta}} = G^{-1} X^\top \mathbf{y}$. When columns nearly duplicate one another, $G$ nearly collapses and variance explodes.

Finite-precision computers amplify near-collinearity rapidly. In Nicholas Higham's (2002) conditioning analysis, solving invertible $A\mathbf{x} = \mathbf{b}$ ($\det A \ne 0$) magnifies relative input perturbations by the **condition number** $\kappa(A) = \Vert A\Vert \mkern2mu \Vert A^{-1}\Vert \ge 1$. On the $2 \times 2$ diagonal stretch $A = \left[\begin{matrix} 10 & 0 \\\\ 0 & 0.1 \end{matrix}\right]$, $\Vert A\Vert\_2 = 10$ and $\Vert A^{-1}\Vert\_2 = 10$ give $\kappa\_2(A) = 100$, whereas forming $G = A^\top A$ squares the stretches to $100$ and $0.01$, squaring the condition number to $\kappa\_2(A^\top A) = 10{,}000$.

For a full-column-rank matrix $X \in \mathbb{R}^{n \times p}$ ($n \ge p$) with $\kappa\_2(X) = \sigma\_1(X)/\sigma\_p(X) \ge 1$, normal equations $G = X^\top X$ square sensitivity to $\kappa\_2(X)^2$. Orthogonal QR or singular-value decomposition avoids forming $G$ and keeps sensitivity proportional to $\kappa\_2(X) + \kappa\_2(X)^2 \Vert\mathbf{r}\Vert\_2 / \Vert X\hat{\mathbf{\beta}}\Vert\_2$ for residual $\mathbf{r} = \mathbf{y} - X\hat{\mathbf{\beta}}$ ($\hat{\mathbf{\beta}} \ne \mathbf{0}$), staying linear in $\kappa\_2(X)$ only when $\kappa\_2(X)\,\Vert\mathbf{r}\Vert\_2 \lesssim \Vert X\hat{\mathbf{\beta}}\Vert\_2$ (relative residual at most $\mathcal{O}(\kappa\_2(X)^{-1})$ even when $\Vert X\hat{\mathbf{\beta}}\Vert\_2 \sim \Vert X\Vert\_2 \Vert\hat{\mathbf{\beta}}\Vert\_2$), while larger residuals reintroduce $\kappa\_2(X)^2$ (Higham 2002).

Even at $\kappa\_2(X) = 1$, non-associative IEEE-754 addition ([Chapter 10](10-numbers-order-bounds.md)) causes dynamic GPU and parallel database reductions to drift across thread schedules (Shanmugavelu et al. 2024; Müller, Arteaga, Hoefler, & Alonso 2018). Order-invariant carry accumulators (`RSum` / `PartitionAndAggregate`; Müller et al. 2018) restore bit-reproducible totals at memory and synchronization cost budgeted off-shift ([Chapter 56](56-layer-formal-computable-type-order.md)), without curing $\kappa\_2(X)^2$ ([Chapter 34](34-sample-population-archive.md); [Chapter 45](45-world-to-archive-big-data-paradox.md)).

How many *effective* independent dimensions does $G \ne 0$ really carry? Olivier Roy and Martin Vetterli (2007) measure continuous dimensionality using the Shannon entropy of singular values ($\mathrm{erank}(X)$). Its quadratic twin uses the **trace** $\mathrm{tr}(G)$ (the diagonal sum, equaling eigenvalue sum $\sum\_{i=1}^p \lambda\_i > 0$) and squared Frobenius norm $\Vert G\Vert\_F^2$ (the sum of all squared matrix entries, equaling $\sum\_{i=1}^p \lambda\_i^2 > 0$). Together they define **effective dimension** $d\_{\mathrm{eff}}$:

$$d\_{\mathrm{eff}} = \frac{\bigl(\mathrm{tr}(G)\bigr)^2}{\Vert G\Vert\_F^2} = \frac{\left(\sum\_{i=1}^p \lambda\_i\right)^2}{\sum\_{i=1}^p \lambda\_i^2}.$$

Read that ratio aloud: square the total trace and divide by the sum of squared entries (or squared eigenvalues). Check both extremes: if $p$ columns are perpendicular with equal length ($\lambda\_i = \lambda > 0$), $d\_{\mathrm{eff}} = p^2\lambda^2 / (p\lambda^2) = p$; if all columns collapse onto one line ($\lambda\_1 = \mathrm{tr}(G), \lambda\_{i>1} = 0$), $d\_{\mathrm{eff}} = 1$.

Check a $2 \times 2$ correlation counterexample with $p = 2$ columns having correlation $r \in [-1, 1]$:

$$G = \left[\begin{matrix} 1 & r \\\\ r & 1 \end{matrix}\right].$$

Directly from matrix entries, $\mathrm{tr}(G) = 1 + 1 = 2$ and $\Vert G\Vert\_F^2 = 1^2 + r^2 + r^2 + 1^2 = 2 + 2r^2$ (matching eigenvalues $\lambda\_1 = 1 + |r|$ and $\lambda\_2 = 1 - |r|$ from §7, with ratio $\lambda\_2/\lambda\_1 = (1 - |r|)/(1 + |r|) > 0$ for $|r| < 1$). Thus $d\_{\mathrm{eff}} = 4/(2 + 2r^2) = 2/(1 + r^2)$, dropping from $2$ at $r = 0$ toward $1$ as $|r| \to 1$. Adding more rows leaves collinearity untouched; only orthogonalizing or pruning redundant columns fixes $G$.

The exact same participation-ratio geometry operates along the *rows* of $X$ whenever survey units carry weights or cluster together:

- **Weighted rows and effective sample count:** Given $n \ge 1$ units with inclusion probabilities $\pi\_i \in (0, 1]$ and Horvitz–Thompson weights $w\_i = 1/\pi\_i > 0$ on $W = \mathrm{diag}(w\_1, \dots, w\_n)$ (Horvitz & Thompson 1952; Kish 1965), $W = \mathrm{diag}(1, 3)$ gives $\mathrm{tr}(W) = 4$ and $\Vert W\Vert\_F^2 = 10$, shrinking effective count to $4^2 / 10 = 1.6 < 2$. Writing $\mathrm{DEFF}\_{\mathrm{Kish}} \ge 1$ for Leslie Kish's (1965) weight variance-inflation factor (**design effect**; [Chapter 34](34-sample-population-archive.md)), the Kish effective sample size $n\_{\mathrm{Kish}}$ is:

  $$n\_{\mathrm{Kish}} = \frac{\bigl(\mathrm{tr}(W)\bigr)^2}{\Vert W\Vert\_F^2} = \frac{\left(\sum\_{i=1}^n w\_i\right)^2}{\sum\_{i=1}^n w\_i^2} = \frac{n}{\mathrm{DEFF}\_{\mathrm{Kish}}}.$$

  Equal inclusion ($\pi\_i \equiv \pi\_0 > 0$) keeps $\mathrm{DEFF}\_{\mathrm{Kish}} = 1$, whereas unequal weights raise $\mathrm{DEFF}\_{\mathrm{Kish}} > 1$ and lower evidential weight (Keynes 1921; [Chapter 45](45-world-to-archive-big-data-paradox.md)).
- **Clustered rows and additive design effects:** Across $C \ge 1$ clusters of equal size $m\_c \equiv \bar{m} \ge 1$ and intra-cluster correlation $\rho\_I \in [0, 1]$, Kish (1965) derived $\mathrm{DEFF}\_{\mathrm{cluster}} = 1 + (\bar{m} - 1)\rho\_I$; under varying cluster sizes ($\mathrm{CV}^2(m) > 0$), Brent Moulton's (1986) effective cluster size $\tilde{m} = \bar{m}\bigl(1 + \mathrm{CV}^2(m)\bigr) > \bar{m}$ gives $\mathrm{DEFF}\_{\mathrm{cluster}} = 1 + (\tilde{m} - 1)\rho\_I \ge 1$. Under unit-level weights independent inside clusters and of cluster membership, sizes $m\_c$, and random effects in the large-$C$ limit, the exact composition is additive, yielding $\mathrm{DEFF}\_{\mathrm{total}} = \mathrm{DEFF}\_{\mathrm{cluster}} + \mathrm{DEFF}\_{\mathrm{Kish}} - 1 \ge 1$ ([Chapter 34](34-sample-population-archive.md) §3). Under shock-independent cluster-constant weights with $m\_c \equiv \bar{m}$ or large-$C$ weights independent of $m\_c$ (or size-weighted second-moment orthogonal in $w\_c^2$), variance equals $\mathrm{DEFF}\_{\mathrm{cluster}} \cdot \mathrm{DEFF}\_{\mathrm{Kish}}$, exceeding the additive sum by a non-negative cross-term ($\rho\_I \ge 0$); under unequal size-dependent weights it can understate variance (requiring $\tilde{m}\_w$ or ceiling $\mathrm{DEFF}\_{\mathrm{Kish}}[1 + (m\_{\max} - 1)\rho\_I]$ with $m\_{\max} := \max\_c m\_c$ for shock-independent weights and $\rho\_I \ge 0$, or conditional shock-weighted variance if weights depend on shocks). Thus $n\_{\mathrm{eff}}^{\mathrm{var}} = n / \mathrm{DEFF}\_{\mathrm{total}}$ halves whenever $\mathrm{DEFF}\_{\mathrm{total}}$ doubles, while support exclusion remains an independent non-sampling error ([Chapter 44](44-underdetermination-partial-identification.md); [Chapter 59](59-layer-experimental-sampling-causal-order.md)).

## 4. Orthogonal Projections and Least Squares

To understand least squares geometrically, start with a $2$D shadow. Let target vector $\mathbf{y} = (3, 4)^\top \in \mathbb{R}^2$ and suppose a single feature spans only the horizontal axis along $\mathbf{x} = (1, 0)^\top$. The closest point on that horizontal line to $\mathbf{y}$ is its straight-down shadow $\hat{\mathbf{y}} = (3, 0)^\top$, leaving a perpendicular residual error $\mathbf{e} = \mathbf{y} - \hat{\mathbf{y}} = (0, 4)^\top$. Two facts follow immediately: $\langle \mathbf{x}, \mathbf{e}\rangle = 1(0) + 0(4) = 0$ (the error stands perpendicular to the model line), and $\Vert \mathbf{y}\Vert\_2^2 = 3^2 + 4^2 = 5^2 = \Vert\hat{\mathbf{y}}\Vert\_2^2 + \Vert\mathbf{e}\Vert\_2^2$ (Pythagoras).

In general, given response vector $\mathbf{y} \in \mathbb{R}^n$ and full-column-rank matrix $X \in \mathbb{R}^{n \times p}$ ($\mathrm{rank}(X) = p$, invertible $G = X^\top X$), the point inside subspace $S = \mathrm{im}(X)$ minimizing distance $\Vert\mathbf{y} - \hat{\mathbf{y}}\Vert\_2$ is the **orthogonal projection** given by projection matrix $P\_X$:

$$\hat{\mathbf{y}} = P\_X \mathbf{y}, \qquad P\_X = X(X^\top X)^{-1} X^\top.$$

Read that aloud: $P\_X$ maps any vector $\mathbf{y}$ onto its closest shadow $\hat{\mathbf{y}}$ inside the column span of $X$. By construction, $P\_X$ is symmetric ($P\_X^\top = P\_X$) and idempotent ($P\_X^2 = P\_X$, since projecting a shadow twice leaves it unchanged), placing residual $\mathbf{e} = (I\_n - P\_X)\mathbf{y}$ in the perpendicular complement $S^\perp$ ($X^\top \mathbf{e} = \mathbf{0}$). By Pythagoras, $\Vert\mathbf{y}\Vert\_2^2 = \Vert P\_X \mathbf{y}\Vert\_2^2 + \Vert\mathbf{y} - P\_X \mathbf{y}\Vert\_2^2$.

Appending an extra column $\mathbf{x}\_{p+1}$ enlarges $S$ to $S' = \mathrm{im}([X\mkern5mu\mathbf{x}\_{p+1}]) \supseteq S$, so in-sample training error can never go up even while out-of-sample prediction error worsens. At $p = n$ linearly independent noise columns, $P\_X$ becomes $I\_n$: training error drops to zero while test error blows up ([Chapter 37](37-model-fit-and-error.md) §1). For overparameterized designs ($p > n$), choosing the minimum-norm solution $X^+\mathbf{y}$ orthogonal to $\ker(X)$ (§2) avoids the exact $p = n$ inversion singularity, though avoiding out-of-sample variance blowup still depends on benign signal-to-spectrum alignment rather than holding automatically.

## 5. High-Dimensional Geometry and Concentration of Measure

Low-dimensional intuition misleads badly once dimension $d$ grows large. Compare a disk ($d = 2$) and a ball ($d = 3$) against a $100$-dimensional hypersphere ($d = 100$) of radius $R$. The volume inside radius $0.9 R$ is $0.9^2 = 0.81$ ($81\%$) in 2D and $0.9^3 = 0.729$ ($73\%$) in 3D, yet plunges to $0.9^{100} \approx 0.000027$ in 100D. Almost the entire volume moves into a razor-thin skin near the outer boundary.

Three geometric laws formalize this **concentration of measure**:

1. **Surface concentration:** Inside a $d$-dimensional ball of radius $R$, the volume inside inner radius $(1 - \epsilon)R$ ($\epsilon \in (0, 1)$) shrinks as $(1 - \epsilon)^d \le e^{-\epsilon d}$. For large $d$, virtually all volume packs into a thin shell of width $\mathcal{O}(R/d)$ near the surface.
2. **Equatorial concentration:** On the unit sphere $\mathbb{S}^{d-1} \subset \mathbb{R}^d$, almost all surface area lies within $\mathcal{O}(1/\sqrt{d})$ of any equator. Two independent uniform draws $\mathbf{u}, \mathbf{v} \sim \mathrm{Unif}(\mathbb{S}^{d-1})$ satisfy $\mathbb{E}[\langle\mathbf{u}, \mathbf{v}\rangle] = 0$ and standard deviation $\mathrm{SD}(\langle\mathbf{u}, \mathbf{v}\rangle) = 1/\sqrt{d}$, making random high-dimensional directions nearly perpendicular ($\langle \mathbf{u}, \mathbf{v} \rangle / (\Vert\mathbf{u}\Vert\Vert\mathbf{v}\Vert) \approx 0$).
3. **The Johnson–Lindenstrauss Lemma** (Johnson & Lindenstrauss 1984; Dasgupta & Gupta 2003): Take any $N \ge 2$ points in high ambient dimension $\mathbb{R}^D$, any distortion tolerance $\epsilon \in (0, 1)$, and any integer target dimension $k$ clearing:

   $$k \ge \frac{4}{\epsilon^2/2 - \epsilon^3/3}\mkern3mu \ln N.$$

   Read that aloud: required dimension $k$ grows only logarithmically with point count $N$ (roughly $8 \ln N / \epsilon^2$ for small $\epsilon$) and does not depend on ambient dimension $D$. There always exists a linear map into $\mathbb{R}^k$ preserving all pairwise squared, and hence unsquared, Euclidean distances within factor $1 \pm \epsilon$ (since $\sqrt{1-\epsilon} > 1-\epsilon$ and $\sqrt{1+\epsilon} < 1+\epsilon$).

A quick moment check confirms shell concentration directly. Let $\mathbf{x} = (x\_1, \dots, x\_d)^\top \in \mathbb{R}^d$ have independent coordinates of mean $0$, variance $1$, and fourth moment $\mathbb{E}[x\_i^4] = 3$. Then $\mathbb{E}[\Vert\mathbf{x}\Vert\_2^2] = d$ and $\mathrm{Var}(\Vert\mathbf{x}\Vert\_2^2) = 2d$. Expanding around $\sqrt{d}$ shows that relative spread $\mathrm{SD}(\Vert\mathbf{x}\Vert\_2)/\sqrt{d}$ vanishes as $d$ grows:

$$\frac{\mathrm{SD}(\Vert\mathbf{x}\Vert\_2)}{\sqrt{d}} \approx \frac{\sqrt{2d}}{2d} = \frac{1}{\sqrt{2d}}.$$

Read that aloud: random vectors cluster ever more tightly onto a sphere of radius $\sqrt{d}$, while extra uninformative coordinates blur nearest-neighbor distances without raising real rank.

## 6. Separability over the two-element field, and why parity is not affine

Exclusive OR (**parity**) exposes the exact geometric boundary of linear thresholds. Plot all four two-bit inputs $(x\_1, x\_2) \in \lbrace 0, 1\rbrace^2$ on the corners of a unit square: one straight cut isolates $(1, 1)$ for AND ($x\_1 + x\_2 \ge 1.5$) or separates $(0, 0)$ from the other three for OR ($x\_1 + x\_2 \ge 0.5$). Yet parity outputs $1$ at $(0, 1), (1, 0)$ and $0$ at $(0, 0), (1, 1)$, opposite corners whose diagonals cross at the common midpoint $(0.5, 0.5)$, defeating every single straight cut.

Over the two-element field $\mathbb{F}\_2 = \lbrace 0, 1\rbrace$ (addition modulo $2$, $\oplus$), $n$-bit parity is linear with weights $(1, \dots, 1) \in \mathbb{F}\_2^n$:

$$\psi\_{\mathrm{PARITY}}(x) = \bigoplus\_{i=1}^{n} x\_i,$$

whereas binary AND ($x\_1 x\_2$) and OR ($x\_1 \oplus x\_2 \oplus x\_1 x\_2$) are nonlinear products for $n \ge 2$. Embed the hypercube in real space $\lbrace 0, 1\rbrace^n \subset \mathbb{R}^n$ and ask for **affine separability** (a real vector $w \in \mathbb{R}^n$ and threshold $\theta \in \mathbb{R}$ with $w \cdot x \ge \theta$ on $1$-inputs and $w \cdot x < \theta$ on $0$-inputs), and the roles invert. Every AND ($\sum\_{i \in S} x\_i \ge |S|$) and every OR ($\sum\_{i \in S} x\_i \ge 1$) separates cleanly by one flat hyperplane, whereas $\psi\_{\mathrm{PARITY}}$ is impossible for any single linear threshold whenever $n \ge 2$: on any $2$-dimensional square face ($n - 2$ bits held at $0$), requiring odd corners $e\_1, e\_2$ above $\theta$ and even corners $\mathbf{0}, e\_1 + e\_2$ below $\theta$ yields the flat contradiction $2\theta \le w \cdot e\_1 + w \cdot e\_2 = w \cdot \mathbf{0} + w \cdot (e\_1 + e\_2) < 2\theta$.

Marvin Minsky and Seymour Papert (1969, Theorem 3.1.1) proved that $\psi\_{\mathrm{PARITY}}$ on $\lbrace 0, 1\rbrace^d$ has threshold order $|R| = d$ (where $R = \lbrace 1, \dots, d \rbrace$ denotes the $d$-bit input coordinate set, distinct from §5's radius $R$): no linear threshold reading fewer than all $d$ bits can compute it. Under uniform inputs on $\lbrace 0, 1\rbrace^4$, any predicate $\varphi$ reading at most three bits has $\mathrm{Cov}(\varphi, \psi\_{\mathrm{PARITY}}) = 0$, because flipping an unread bit reverses parity while leaving $\varphi$ unchanged. Separately, Frank Rosenblatt's (1958) perceptron rule converges finitely on linearly separable AND/OR targets (Minsky & Papert 1969, Theorem 11.1), yet cycles indefinitely on parity (§11.9) until hidden layers trained by backpropagation supply a nonlinear representation lift $\Phi(x)$ (Rumelhart, Hinton & Williams 1986; [Chapter 28](28-the-machine-and-the-deception.md) §5).

Dimensionality reduction separates along the same line ([Chapter 37](37-model-fit-and-error.md) §5): linear projections $P$ such as Principal Component Analysis (Hotelling 1933) and Johnson–Lindenstrauss maps (Dasgupta & Gupta 2003) create an explicit null kernel $\ker(P)$ (generalizing §4's orthogonal projection $P\_X$), while nonlinear neighborhood embeddings such as UMAP (McInnes, Healy & Melville 2018) compress metric structure without factor loadings. Neither recovers minority hazards erased before archive sealing ([Chapter 48](48-reactive-worlds-performativity-goodhart.md)).

## 7. Eigenvectors, Basis Change, and Spectral Stability

Recall our opening $2 \times 2$ operator from §2, $A = \left[\begin{matrix} 1 & 2 \\\\ 2 & 4 \end{matrix}\right]$. Most vectors get rotated when $A$ hits them, yet $(1, 2)^\top$ lands right back on its own line stretched by $5$, while $(-2, 1)^\top$ collapses by factor $0$. Those special invariant directions are eigenvectors.

Formally, for a square matrix $A \in \mathbb{R}^{d \times d}$, a non-zero vector $\mathbf{v} \ne \mathbf{0}$ (in $\mathbb{R}^d$ or $\mathbb{C}^d$) is an **eigenvector** with **eigenvalue** $\lambda$ whenever $A\mathbf{v} = \lambda\mathbf{v}$, meaning $\lambda$ solves $\det(A - \lambda I\_d) = 0$ and $\mathbf{v} \in \ker(A - \lambda I\_d)$. Every symmetric positive semi-definite Gram matrix $G = X^\top X$ (§3) admits real non-negative eigenvalues $\lambda\_1 \ge \cdots \ge \lambda\_p \ge 0$ and perpendicular unit eigenvectors forming an orthonormal matrix $Q$ ($Q^\top = Q^{-1}$), diagonalizing cleanly as $G = Q\Lambda Q^\top$.

Changing coordinate axes via an invertible matrix $S \in \mathbb{R}^{d \times d}$ transforms $A$ by similarity, $A \mapsto S^{-1}AS$. General similarity keeps rank, trace, determinant, eigenvalues, and the **spectral radius** $\rho(A) = \max\_i |\lambda\_i(A)|$ unchanged (while mapping flat hyperplanes to flat hyperplanes, leaving XOR inseparable, §6). Yet a skewed basis $S$ can still blow up the matrix norm $\Vert S^{-1}AS\Vert\_2$ arbitrarily. Only an **orthonormal** rotation/reflection $Q^\top = Q^{-1}$ keeps dot products, lengths, angles, and singular values untouched.

Follow what happens along a repeated linear update $\mathbf{x}\_{t+1} = A\mathbf{x}\_t$. Along any eigenvector, steps scale simply as $A^t \mathbf{v} = \lambda^t \mathbf{v}$. Test a tiny diagonal matrix $A = \mathrm{diag}(0.5, 0.2)$: after $t$ steps, $A^t = \mathrm{diag}(0.5^t, 0.2^t)$ shrinks every vector smoothly to zero because both eigenvalues sit inside $(-1, 1)$. In general, powers decay ($\lim\_{t \to \infty} A^t = 0$) **if and only if** $\rho(A) < 1$, while any $|\lambda\_i| > 1$ explodes exponentially.

Symmetric and other **normal** matrices ($A^\top A = A A^\top$) obey $\Vert A^t\Vert\_2 = \rho(A)^t$ at every single step, shrinking monotonically whenever $\rho(A) < 1$. Non-normal matrices ($A^\top A \ne A A^\top$) hide a dangerous finite-time trap. Check the $2 \times 2$ upper-triangular Jordan block $B$ with eigenvalue $\lambda$ ($0 < |\lambda| < 1$) and off-diagonal coupling $\gamma \ne 0$:

$$B = \left[\begin{matrix} \lambda & \gamma \\\\ 0 & \lambda \end{matrix}\right].$$

Raising $B$ to step $t$ gives $B^t = \bigl[\begin{smallmatrix} \lambda^t & t\gamma\lambda^{t-1} \\\\ 0 & \lambda^t \end{smallmatrix}\bigr]$. Even though $\rho(B) = |\lambda| < 1$ guarantees $B^t \to 0$ eventually as $t \to \infty$, a large coupling $|\gamma|$ relative to $1 - |\lambda|$ drives a temporary surge ($\Vert B^t\Vert\_2 \gg 1$) before asymptotic decay finally wins. Spectral radius governs long-run convergence, never short-term safety ($\Vert A\Vert\_2 \le 1$) when directions skew non-normally.

### Dimension is the rank of the design

Feature dimension is column rank, never raw row count, while row-space evidence tracks effective sample size $n\_{\mathrm{eff}}^{\mathrm{var}}$ (§3); neither compensates for the other. Shrinking a linear representation $P$ widens its null kernel $\ker(P)$, erasing minority anomalies under shifting conditions ([Chapter 37](37-model-fit-and-error.md) §5) and hiding costs spilled onto external reception perimeters, namely outside communities and habitats ([Chapter 48](48-reactive-worlds-performativity-goodhart.md)).

### When the relation is not linear

Because affine maps $\mathbf{x} \mapsto W\mathbf{x} + \mathbf{b}$ add features without cross-product terms ($\partial^2 f / \partial x\_i \partial x\_j = 0$), they miss interactions above order one (§6). An explicit nonlinear lift $\Phi(X)$ breaks that linear ceiling yet risks $\kappa\_2^2$ conditioning (§3), floating-point schedule drift, and oversight overload ([Chapter 28](28-the-machine-and-the-deception.md) §3). When a model fails, inspect $\ker(P)$ (the null space of linear projection $P$) and linear additivity before blaming sample size.

## Bridge

[Chapter 10](10-numbers-order-bounds.md) encloses scalar floating-point rounding while [Chapter 56](56-layer-formal-computable-type-order.md) budgets formal verification cost against type and hardware limits. [Chapter 37](37-model-fit-and-error.md) separates linear-projection $\ker(P)$ blind spots from UMAP embeddings, while [Chapter 34](34-sample-population-archive.md) and [Chapter 45](45-world-to-archive-big-data-paradox.md) embed Kish weights and Moulton clustering inside design effects. Finally, [Chapter 48](48-reactive-worlds-performativity-goodhart.md) traces pre-seal blind spots in $\ker(P)$ and reactive spillovers.

Deterministic vector spaces, projections, and spectral stability conclude here; [Probability as a Measure](15-probability-as-a-measure.md) equips outcome spaces with the measurable triple $(\Omega, \mathcal{F}, \mathbb{P})$ and builds conditioning, Bayes' rule, martingales, and Ville's inequality.
