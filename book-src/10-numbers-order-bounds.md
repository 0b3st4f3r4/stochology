# Numbers, Order, and Bounds

> Second chapter of the mathematical block in Part II. The subject is the number line, and the reason
> a bound is a stronger object than an estimate.

## By the end of this chapter you can

1. Say why the real numbers need a completeness property that the rationals lack.
2. Compute a supremum and an infimum, and explain why they may not be members of the set.
3. Read an interval $[L, U]$ as a guarantee rather than a summary, and say what changes when a
   quantity is replaced by it.

## 1. The Tower and Dedekind Completeness

Number systems are constructed in successive layers, each resolving an algebraic or topological deficiency
of the layer beneath:

- The **naturals** $\mathbb{N} = \{0, 1, 2, \dots\}$ support counting and succession.
- The **integers** $\mathbb{Z}$ close the naturals under subtraction.
- The **rationals** $\mathbb{Q} = \{p/q : p \in \mathbb{Z}, q \in \mathbb{N} \setminus \{0\}\}$ close the
  integers under division by non-zero elements.

The rational field $\mathbb{Q}$ is dense: between any two distinct rationals $q_1 < q_2$, there exists another
rational $(q_1 + q_2)/2$. Yet density is insufficient for analysis. The set $S = \{q \in \mathbb{Q} : q^2 < 2\}$
is non-empty and bounded above (for instance by $3/2$), yet it possesses no least upper bound within $\mathbb{Q}$,
because $\sqrt{2} \notin \mathbb{Q}$. Rational sequences that ought to converge fail to find an endpoint.

The **reals** $\mathbb{R}$ resolve this gap through the **least upper bound property** (Dedekind completeness):
every non-empty subset of $\mathbb{R}$ that is bounded above has a supremum in $\mathbb{R}$. Completeness
is what guarantees the existence of limits, continuous extrema, Riemann and Lebesgue integrals, and asymptotic
distributional bounds. Without completeness, limiting arguments collapse into unresolvable gaps.

## 2. Order, Intervals, and Bounds

A total order on $\mathbb{R}$ permits the definition of bounded intervals for $a \le b$:

$$[a, b] = \{x \in \mathbb{R} : a \le x \le b\}, \qquad (a, b) = \{x \in \mathbb{R} : a < x < b\}.$$

An element $M$ is an **upper bound** of $S \subset \mathbb{R}$ if $x \le M$ for all $x \in S$. The **supremum**
$\sup S$ is the least upper bound: it satisfies $x \le \sup S$ for all $x \in S$, and for any $\epsilon > 0$,
there exists $s \in S$ such that $s > \sup S - \epsilon$. Symmetrically, the **infimum** $\inf S$ is the
greatest lower bound.

Crucially, neither the supremum nor the infimum needs to be an element of the set. For the open interval
$(0, 1)$, $\sup (0, 1) = 1 \notin (0, 1)$ and $\inf (0, 1) = 0 \notin (0, 1)$. When an institution establishes
a bound — stating that failure probability is strictly bounded above by $\epsilon$ — it guarantees that no
realized state exceeds $\epsilon$, even as empirical conditions arbitrarily approach that ceiling.

## 3. Interval Arithmetic and Partial Identification

When parameters cannot be known with point certainty, intervals serve as propagation guarantees.
Under interval arithmetic, given $X \in [L_X, U_X]$ and $Y \in [L_Y, U_Y]$:

$$[L_X, U_X] + [L_Y, U_Y] = [L_X + L_Y, U_X + U_Y],$$

$$[L_X, U_X] - [L_Y, U_Y] = [L_X - U_Y, U_X - L_Y].$$

Notice the subtraction: uncertainty intervals widen under difference rather than cancel. Point estimates
hide this accumulation of ignorance; interval arithmetic preserves it faithfully.

In causal inference and econometrics, Charles Manski (1990) demonstrated that missing counterfactual data
yields worst-case **partial identification bounds** rather than point parameters. If an outcome $Y \in [0, 1]$
is observed only for treated units ($D=1$) with selection rate $p = \mathbb{P}(D=1)$, the expected treated
outcome $\mathbb{E}[Y(1)]$ without uncheckable assumptions is bounded by:

$$\mathbb{E}[Y(1)] \in \bigl[p \mathbb{E}[Y \mid D=1],\; p \mathbb{E}[Y \mid D=1] + (1 - p)\bigr].$$

The interval width is exactly $1 - p$. No increase in sample size $n$ can shrink this width to zero without
structural assumptions about the missing counterfactuals. Reporting the interval $[L, U]$ communicates what
the evidence actually constrains.

## 4. Heavy Tails, Extremes, and the Tail Index

Bounding real-valued processes becomes acute when distributions are heavy-tailed. A distribution is
**subexponential** if its tail decays slower than any exponential distribution. For heavy-tailed variables
following a Pareto-type power law:

$$\mathbb{P}(X > x) = L(x) x^{-\alpha}, \qquad \text{as } x \to \infty,$$

where $\alpha > 0$ is the Pareto tail index and $L(x)$ is a slowly varying function ($\lim_{t\to\infty} L(tx)/L(t) = 1$).
The tail index $\alpha$ dictates moment existence:

- For $\alpha \le 1$, the theoretical mean $\mathbb{E}[X]$ is infinite.
- For $1 < \alpha \le 2$, the mean is finite but the population variance $\mathrm{Var}(X)$ is infinite,
  invalidating the classical Central Limit Theorem.
- For $\alpha > 2$, both mean and variance are finite, yet extreme outliers continue to dominate sample sums.

The Pickands–Balkema–de Haan theorem (1974/1975) shows that for a broad class of distributions, the conditional
distribution of exceedances over a sufficiently high threshold $u$, $X - u \mid X > u$, converges to the
Generalized Pareto Distribution (GPD):

$$G_\xi(y) = 1 - \left(1 + \frac{\xi y}{\sigma}\right)^{-1/\xi},$$

where $\xi = 1/\alpha$ is the shape parameter. When $\xi > 0$ (heavy tails), the tail has no upper bound,
and sample maxima grow polynomially with sample size.

## 5. Intervals as Honest Answers

Replacing a point estimate with a bound $[L, U]$ shifts the logic of institutional judgment:

1. **Honest uninformative bounds**: If $[L, U]$ spans the entire feasible range, the data do not contain
   the answer. That is a rigorous empirical finding about the data, not a failure of analysis.
2. **Robust decision invariance**: If an action $a^*$ remains optimal for every $\theta \in [L, U]$, the
   decision is robustly justified without requiring further measurement.
3. **Worst-case survival sizing**: Sizing vital reserves against the upper bound $U$ ensures that the system
   does not cross an irreversible ruin barrier under heavy-tailed tail draws.

## Bridge

[Distribution and Moment](31-distribution-and-moment.md) and [The Non-Identical, Scale Invariance, Convexity, and Heavy Tails](46-non-identical-asymmetry-heavy-tails.md)
operationalize these bounds on empirical data. In the Stochology library, `vital.rs::hill_alpha_with_ci95`
estimates the Pareto tail index $\hat{\alpha}$ alongside its asymptotic $95\%$ confidence bounds, while
`epistemic.rs::from_ate_bounds` constructs non-parametric Manski worst-case bounds over unobserved outcomes.
