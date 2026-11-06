# Sequences, Limits, and Continuity

> Third chapter of the mathematical block in Part II. The subject is what it means for something to
> approach something else, and where that fails.

## By the end of this chapter you can

1. Say what it means for a sequence to converge, without circular appeals to "getting closer".
2. Distinguish a limit from a value, and explain why a function can be continuous everywhere except
   at the one point a decision depends on.
3. Say what continuity is not, using a case where small causes have large effects.

## 1. Sequences and Cauchy Convergence

A sequence is an ordered mapping from the natural numbers into a metric space, written $(a_n)_{n=1}^\infty$.
It **converges** to a limit $L \in \mathbb{R}$ if for every adversarial tolerance $\epsilon > 0$, there exists
an index $N(\epsilon) \in \mathbb{N}$ such that for all $n \ge N(\epsilon)$:

$$|a_n - L| < \epsilon.$$

The quantifier order is the core of the definition: the error threshold $\epsilon$ is chosen first by an
adversary; only then is the threshold index $N(\epsilon)$ produced.

A sequence is a **Cauchy sequence** if its terms become arbitrarily close to one another: for every $\epsilon > 0$,
there exists $N$ such that $|a_n - a_m| < \epsilon$ for all $n, m \ge N$. In $\mathbb{R}$, Cauchy completeness
ensures that every Cauchy sequence converges to a limit in $\mathbb{R}$.

In empirical practice, asymptotic convergence ($n \to \infty$) guarantees nothing about finite prefixes.
A sequence may converge at rate $\mathcal{O}(1/\ln n)$, requiring astronomical sample sizes to achieve
modest precision. Worse, if the underlying process is non-stationary or subject to regime shifts, an empirical
sequence that appears settled can diverge the moment environmental conditions change.

## 2. Limits, Continuity, and Uniform Stability

For a real-valued function $f : D \to \mathbb{R}$, the statement $\lim_{x \to x_0} f(x) = L$ asserts that
$f(x)$ can be forced arbitrarily close to $L$ by choosing $x$ sufficiently close to $x_0$. The function is
**continuous at $x_0$** if $x_0 \in D$ and $\lim_{x \to x_0} f(x) = f(x_0)$.

Continuity formalizes the intuition that *small perturbations produce small responses*. However, point
continuity does not imply **uniform continuity**. On an open domain, $f(x) = 1/x$ is continuous at every
$x > 0$, yet its sensitivity $\sup |f'(x)| = \infty$ as $x \to 0$. In institutional decision systems,
uniform stability is required: an admissible decision rule must not amplify tiny observation errors into
arbitrarily large operational disruptions.

Discontinuities represent the points where judgment is forced:

- **Jump discontinuities**: Abrupt threshold switches, such as activating an emergency buffer when risk
  clears a threshold $p^*$;
- **Asymptotic blow-ups**: Divergence to infinity at a finite boundary, such as Kingman queue waiting times
  blowing up as utilization $\rho \to 1$;
- **Regime oscillations**: Limit non-existence due to undamped delayed feedback loops.

## 3. Continuity Is Not Robustness

A function can be infinitely differentiable everywhere and still exhibit extreme instability. Consider
$f(x) = x^k$ for large $k$: a fractional input error $\delta x / x$ induces an output relative error:

$$\frac{\delta f}{f} \approx k \frac{\delta x}{x}.$$

For non-linear models, local continuity provides zero guarantee against catastrophic sensitivity. In dynamic
systems, deterministic chaos (such as the Lorenz attractor) features equations that are continuous and smooth
at every point in phase space, yet nearby trajectories diverge exponentially at rate $e^{\lambda t}$, where
$\lambda > 0$ is the maximal Lyapunov exponent. Determinism does not imply predictability, and continuity
does not imply robustness.

## 4. Non-Ergodicity and the Itô Variance Drain

The most consequential failure of convergence occurs in the divergence between **ensemble averages** and
**time averages**. In classical statistical mechanics, a system is ergodic if the time average along a
single trajectory equals the ensemble average over all possible states:

$$\lim_{T \to \infty} \frac{1}{T} \int_0^T X_t\,dt = \mathbb{E}[X].$$

In multiplicative and dynamic processes, ergodicity fails systematically. Consider a wealth or resource
trajectory modeled by Geometric Brownian Motion:

$$dX_t = \mu X_t\,dt + \sigma X_t\,dW_t, \qquad X_0 > 0.$$

By Itô's lemma, the solution for an individual trajectory is:

$$X_t = X_0 \exp\left( \left(\mu - \frac{\sigma^2}{2}\right) t + \sigma W_t \right).$$

Evaluating the **ensemble average** over all possible universes yields:

$$\mathbb{E}[X_t] = X_0 e^{\mu t}.$$

If $\mu > 0$, the ensemble average grows exponentially without bound. However, by the law of the iterated
logarithm for Brownian motion, almost every single trajectory evolves at the time-average growth rate:

$$g = \lim_{t \to \infty} \frac{1}{t} \ln X_t = \mu - \frac{\sigma^2}{2} \quad \text{almost surely}.$$

The negative term $-\frac{1}{2}\sigma^2$ is the **Itô variance drain**. When $\mu > 0$ but $\mu < \frac{1}{2}\sigma^2$,
the ensemble average explodes to $+\infty$, yet with probability 1, every individual institution following
the process goes bankrupt as $t \to \infty$. The ensemble average is driven by an infinitesimal fraction of
astronomically lucky trajectories, while typical systems suffer irreversible collapse.

## 5. Consequences for Sequential Decision-Making

Institutions live along a single time trajectory; they cannot average across parallel counterfactual worlds.
Evaluating risk using ensemble expectations assumes that bankruptcy in one run can be compensated by extreme
wealth in another. Under non-ergodic multiplicative dynamics, volatility acts as a direct drag on survival.
Any decision rule that fails to account for the Itô variance drain will mistake a path to certain ruin for
a profitable investment.

## Bridge

[Distribution and Moment](31-distribution-and-moment.md) examines the mechanics of sample convergence, while
[Time and Dependence](38-time-and-dependence.md) analyzes how variance drains multiplicative trajectories over time.
In the Stochology library, `vital.rs::from_non_ergodic_returns` computes the time-average growth rate $g = \mu - \frac{1}{2}\sigma^2$
and compares it against ensemble returns, diagnosing when an institution is being misled by ensemble optimism
toward absorbing ruin barriers.

[Derivatives and Convexity](12-derivatives-and-convexity.md) differentiates the limit into local
sensitivity and curvature, which is what makes the Jensen gap between an average and a risk visible.
