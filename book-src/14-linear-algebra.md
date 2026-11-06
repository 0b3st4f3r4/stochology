# Linear Algebra

> Sixth chapter of the mathematical block in Part II. The subject is what a matrix does, and why almost
> every model in use is a linear map with a nonlinearity bolted on.

## By the end of this chapter you can

1. Read a matrix as a map between spaces rather than as a table of numbers.
2. Say what rank, kernel and image mean, and why a rank-deficient design cannot answer the question it
   was built for.
3. Explain what an eigenvector is and why directions that survive a map unchanged matter for stability.

## 1. Vectors, Subspaces, and Dimension

A real vector space $\mathbb{R}^n$ is a set of $n$-tuples closed under vector addition ($\mathbf{u} + \mathbf{v}$)
and scalar multiplication ($c\mathbf{u}$). A **linear subspace** $V \subseteq \mathbb{R}^n$ is a subset closed
under both operations.

A collection of vectors $\{\mathbf{v}_1, \dots, \mathbf{v}_k\}$ is linearly independent if $\sum_{i=1}^k c_i \mathbf{v}_i = \mathbf{0}$
implies $c_1 = \cdots = c_k = 0$. A **basis** is a minimal linearly independent spanning set for $V$. The number
of elements in any basis defines the **dimension** $\dim(V)$.

In data architecture, the number of columns in an archive table is the nominal feature dimension $p$. However,
the true informational content is governed by the dimension of the subspace spanned by those columns. Adding
correlated features increases the column count without expanding the underlying subspace dimension.

## 2. Matrices as Linear Operators and the Rank-Nullity Theorem

An $m \times n$ matrix $A$ represents a linear operator mapping vectors from an input domain $\mathbb{R}^n$
into a codomain $\mathbb{R}^m$: $\mathbf{x} \mapsto A\mathbf{x}$. Two fundamental subspaces define its geometry:

- The **kernel** (or null space): $\ker(A) = \{\mathbf{x} \in \mathbb{R}^n : A\mathbf{x} = \mathbf{0}\}$, the
  set of inputs that the operator annihilates.
- The **image** (or column space): $\mathrm{im}(A) = \{A\mathbf{x} : \mathbf{x} \in \mathbb{R}^n\} \subseteq \mathbb{R}^m$,
  the subspace of reachable outputs.

By the **rank-nullity theorem**:

$$\dim\bigl(\ker(A)\bigr) + \dim\bigl(\mathrm{im}(A)\bigr) = n.$$

The dimension of the image is the **rank** of the matrix, $\mathrm{rank}(A)$.

When a design matrix $X \in \mathbb{R}^{n \times p}$ in a regression or prediction problem has linearly dependent
columns, $\mathrm{rank}(X) < p$, meaning $\dim(\ker(X)) > 0$. The kernel contains non-trivial parameter vectors
$\mathbf{\beta}_0 \ne \mathbf{0}$ such that $X\mathbf{\beta}_0 = \mathbf{0}$. Consequently, the data cannot
distinguish between parameter vector $\mathbf{\beta}$ and $\mathbf{\beta} + \mathbf{\beta}_0$. Any algorithm
that reports a unique point estimate in a rank-deficient design is imposing an arbitrary regularizer rather than
learning from the data.

## 3. The Gram Matrix and Effective Dimension

The geometry of a feature matrix $X$ is encapsulated in its **Gram matrix** $G = X^T X \in \mathbb{R}^{p \times p}$.
If $G$ is invertible, ordinary least squares has a unique solution $\hat{\mathbf{\beta}} = G^{-1} X^T \mathbf{y}$.
However, when features are highly collinear, $G$ is ill-conditioned: its smallest eigenvalues approach zero,
causing $(X^T X)^{-1}$ to explode and inflating parameter variance.

To capture the true dimensionality of correlated data without relying on crude column counts, we define the
**effective dimension** (Roy & Vetterli, 2007) via the eigenvalues $\lambda_1 \ge \cdots \ge \lambda_p \ge 0$ of $G$:

$$d_{\mathrm{eff}} = \frac{\bigl(\mathrm{tr}(G)\bigr)^2}{\|G\|_F^2} = \frac{\left(\sum_{i=1}^p \lambda_i\right)^2}{\sum_{i=1}^p \lambda_i^2}.$$

When all $p$ features are orthogonal and equal in variance ($\lambda_i = \lambda$), $d_{\mathrm{eff}} = p^2 \lambda^2 / (p \lambda^2) = p$.
When features are perfectly collinear ($\lambda_1 = \mathrm{tr}(G), \lambda_{i > 1} = 0$), $d_{\mathrm{eff}} = 1$.
Effective dimension quantifies how severe multicollinearity collapses degrees of freedom.

## 4. Orthogonal Projections and Least Squares

Given an observation vector $\mathbf{y} \in \mathbb{R}^n$ and the subspace $S = \mathrm{im}(X)$ spanned by the
regressors, the point in $S$ that minimizes the Euclidean distance $\|\mathbf{y} - \hat{\mathbf{y}}\|_2$ is the
**orthogonal projection** of $\mathbf{y}$ onto $S$:

$$\hat{\mathbf{y}} = P_X \mathbf{y}, \qquad P_X = X(X^T X)^{-1} X^T.$$

The projection operator $P_X$ is symmetric ($P_X^T = P_X$) and idempotent ($P_X^2 = P_X$). The residual vector
$\mathbf{e} = \mathbf{y} - \hat{\mathbf{y}} = (I - P_X)\mathbf{y}$ lies in the orthogonal complement $S^\perp$:
$X^T \mathbf{e} = \mathbf{0}$.

Because least squares is an orthogonal projection, the residual sum of squares can only decrease or stay constant
when new columns are appended to $X$. Adding uninformative features mechanically enlarges the subspace, driving
in-sample residuals to zero while inflating out-of-sample prediction error — the precise linear geometry of
overfitting.

## 5. High-Dimensional Geometry and Concentration of Measure

When dimensionality $d$ is large, Euclidean geometry behaves in ways that confound low-dimensional intuition:

- **Surface concentration**: In a $d$-dimensional hypersphere of radius $R$, the fraction of volume inside a
  concentric sphere of radius $(1 - \epsilon)R$ is $(1 - \epsilon)^d \le e^{-\epsilon d}$. For large $d$, virtually
  $100\%$ of the volume of the sphere is concentrated in a paper-thin shell of thickness $\mathcal{O}(R/d)$ adjacent
  to the boundary.
- **Equatorial concentration**: In a high-dimensional sphere, almost all the surface area lies within a narrow
  equatorial band of distance $\mathcal{O}(1/\sqrt{d})$ from any arbitrary equator. Any two random vectors drawn
  independently and uniformly from the sphere are almost exactly orthogonal with high probability:
  $\langle \mathbf{u}, \mathbf{v} \rangle \approx 0$.
- **The Johnson–Lindenstrauss Lemma**: Given any set of $N$ points in arbitrary dimension $\mathbb{R}^D$, for any
  distortion parameter $\epsilon \in (0, 1)$, there exists a linear mapping into dimension:

  $$k \ge \frac{8 \ln N}{\epsilon^2}$$

  such that all pairwise Euclidean distances are preserved within a factor of $1 \pm \epsilon$. Dimensionality
  reduction succeeds because the geometry of finite point sets is constrained by sample size $N$ rather than ambient
  feature dimension $D$.

## 6. Separability over the two-element field, and why parity is not affine

One example fixes the meaning of "linear model" more sharply than any amount of rank algebra, and it is the
example that interrupted the first wave of connectionist research: the exclusive disjunction.

Work over the two-element field $\mathbb{F}_2 = \{0, 1\}$ with addition modulo two. A dichotomy on $n$ binary
features is **affinely separable** when there exist a weight vector $w \in \mathbb{R}^n$ and a threshold
$\theta$ such that $w \cdot x \ge \theta$ exactly on the positive class. Counting over $\mathbb{F}_2$ gives
the structural statement: the map $x \mapsto w \cdot x$ is $\mathbb{F}_2$-linear only when every weight is
itself in $\{0, 1\}$, and then the image is a linear functional whose value is fixed by the parity of the
coordinates that carry unit weight. The parity predicate

$$\psi_{\mathrm{PARITY}}(x) = \bigoplus_{i=1}^{n} x_i$$

is therefore not representable by any single threshold element for $n \ge 2$, while every conjunction and every
disjunction is. Separability is not a matter of having enough data or enough training: it is a property of the
image of the operator. What a hidden layer adds is exactly a change of basis in which the target becomes
separable — the geometric content of the 1986 recovery narrated in
[Chapter 28](28-the-machine-and-the-deception.md) §5, and the reason a rank-deficient design (rank-nullity above)
cannot answer a question that needs the missing direction.

The rank constraint also explains the cheapest modern form of adaptation. A low-rank update $A \mapsto A + UV^{\top}$
with $U, V$ of width $r \ll \min(m, n)$ restricts the change of map to a subspace of dimension at most $\min(m, n) r$ —
which is why such adapters can steer a large operator without retraining it, and why they cannot repair a deficiency
that lives outside the image of that subspace. Rank is not a bookkeeping statistic; it is the exact size of the set of
questions the update can change the answer to.

The same reading covers the modern reduction step. Principal components and metric embeddings are projections
onto a subspace chosen to preserve distances, and the Johnson–Lindenstrauss bound quoted above states the price
in dimension. A projection preserves the geometry of the sampled point cloud; it cannot create a direction that
the data never spanned. When a downstream model fails after reduction, the informative question is which
functional of the original space was annihilated by the kernel of the projection, not which algorithm was used.

## Bridge

[Model, Fit, and Error](37-model-fit-and-error.md) translates orthogonal projections into the classical
bias-variance decomposition, while [World-to-Archive, Sampling, and the Big Data Paradox](45-world-to-archive-big-data-paradox.md)
evaluates effective degrees of freedom across stratified clusters. In the Stochology library,
`epistemic.rs::EpistemicMarginState` tracks effective degrees of freedom and design effects, while
`epistemic.rs::from_kish_strata` computes Kish effective sample sizes when variance inflation collapses high-dimensional
archival representations.

[Probability as a Measure](15-probability-as-a-measure.md) assembles these pieces into a single object,
the triple $(\Omega, \mathcal{F}, \mathbb{P})$, and derives conditioning, martingales, and Ville's
inequality from it.
