# Derivatives and Convexity

> Fourth chapter of the mathematical block in Part II. The subject is rate of change, and the single
> geometric fact that decides whether planning with an average underfunds the damage.

## By the end of this chapter you can

1. Read a derivative as a rate and a second derivative as a curvature, and say which one a decision
   needs.
2. Identify a function as convex or concave from its shape and say what that implies for an average.
3. Compute the convexity gap for a simple case and say why it is never negative.

## 1. The Derivative as Local Sensitivity

The derivative of a real function $f$ at point $x$ is defined as the limit of the difference quotient:

$$f'(x) = \lim_{h \to 0} \frac{f(x + h) - f(x)}{h}.$$

Geometrically, $f'(x)$ is the slope of the tangent line; operationally, it measures local marginal sensitivity — the
rate of change of the output per unit change in input.

A derivative equal to zero ($f'(x^*) = 0$) identifies a stationary point: a local minimum, a local maximum,
or an inflection point. However, $f'$ alone does not indicate the stability of the stationary point, nor how
the function responds to finite, non-infinitesimal shocks. In decision contexts, planning exclusively on
marginal slopes $f'$ ignores non-linear tail vulnerability.

## 2. The Second Derivative and Curvature

Differentiating again yields the second derivative:

$$f''(x) = \lim_{h \to 0} \frac{f'(x + h) - f'(x)}{h}.$$

The second derivative measures the rate of change of the slope — the **curvature** of the function:

- If $f''(x) > 0$ across an interval, the slope is strictly increasing, the graph bends upwards, and $f$ is **strictly convex**.
- If $f''(x) < 0$, the slope is strictly decreasing, the graph bends downwards, and $f$ is **strictly concave**.
- If $f''(x) = 0$, the function is locally affine (a straight line or inflection).

Curvature is the geometric property that determines whether the expectation of a function equals the function
of an expectation. In physical, biological, and organizational systems, damages are almost universally convex
in stress: a 10% increase in load when capacity is nearly full does not produce a 10% increase in delay, but a
tenfold explosion `[Assumed]`.

## 3. Jensen's Inequality and the Taylor Expansion of the Jensen Gap

A function $\phi : \mathbb{R} \to \mathbb{R}$ is convex on an interval if for all $x, y$ in the interval and all
$\lambda \in [0, 1]$:

$$\phi\bigl(\lambda x + (1 - \lambda) y\bigr) \le \lambda \phi(x) + (1 - \lambda) \phi(y).$$

By Johan Jensen's inequality (1906), for any integrable random variable $X$:

$$\mathbb{E}\bigl[\phi(X)\bigr] \ge \phi\bigl(\mathbb{E}[X]\bigr).$$

The difference is the **Jensen gap** (or convexity gap):

$$\Delta_J(\phi, X) = \mathbb{E}\bigl[\phi(X)\bigr] - \phi\bigl(\mathbb{E}[X]\bigr) \ge 0.$$

To understand the quantitative magnitude of this gap, consider the second-order Taylor series expansion of $\phi(X)$
around the mean $\mu = \mathbb{E}[X]$:

$$\phi(X) \approx \phi(\mu) + \phi'(\mu)(X - \mu) + \frac{1}{2} \phi''(\mu)(X - \mu)^2.$$

Taking mathematical expectations on both sides:

$$\mathbb{E}\bigl[\phi(X)\bigr] \approx \phi(\mu) + \phi'(\mu)\mathbb{E}[X - \mu] + \frac{1}{2}\phi''(\mu)\mathbb{E}\bigl[(X - \mu)^2\bigr].$$

Since $\mathbb{E}[X - \mu] = 0$ and $\mathbb{E}[(X - \mu)^2] = \sigma^2 = \mathrm{Var}(X)$, we obtain the canonical
second-order approximation:

$$\Delta_J(\phi, X) \approx \frac{1}{2} \phi''(\mu) \sigma^2.$$

This identity reveals the anatomy of the Jensen gap: it is the product of physical or structural curvature
$\phi''(\mu)$ and environmental variance $\sigma^2$. The gap is zero if and only if either the function is affine
($\phi'' \equiv 0$) or the world is completely deterministic ($\sigma^2 = 0$).

## 4. The Explosive Curvature of Queues and Ruin

When an institution evaluates risk at the average, substituting $\phi(\mathbb{E}[X])$ for $\mathbb{E}[\phi(X)]$,
it understates true expected harm by exactly $\Delta_J$. In queueing and logistics, John Kingman's heavy-traffic
formula demonstrates that waiting time is proportional to:

$$\phi(\rho) = \frac{\rho}{1 - \rho}, \qquad \rho \in [0, 1),$$

where $\rho$ is utilization. Computing the derivatives:

$$\phi'(\rho) = \frac{1}{(1 - \rho)^2}, \qquad \phi''(\rho) = \frac{2}{(1 - \rho)^3}.$$

As utilization approaches 100% ($\rho \to 1$), the curvature $\phi''(\rho)$ diverges cubically as $\mathcal{O}\bigl((1 - \rho)^{-3}\bigr)$.
At $\rho = 0.50$, $\phi''(0.50) = 16$; at $\rho = 0.90$, $\phi''(0.90) = 2{,}000$; at $\rho = 0.95$, $\phi''(0.95) = 16{,}000$ `[Assumed]`.
A tiny fluctuation in arrival rate at 95% utilization creates a monumental Jensen gap that wipes out unbuffered
operations.

## 5. What Curvature Decides

Three iron rules govern judgment under non-linear curvature:

1. **Planning at the mean guarantees underfunding**: Sizing a reserve or emergency budget against the loss
   at the average $\phi(\mathbb{E}[X])$ leaves the institution short by exactly $\Delta_J \approx \frac{1}{2}\phi''(\mu)\sigma^2$.
2. **Concave benefits create false optimism**: When benefits saturate ($\phi'' < 0$), $\mathbb{E}[\phi(X)] \le \phi(\mathbb{E}[X])$,
   meaning expected returns are strictly less than projected returns at the mean.
3. **Linearity is a fragile fiction**: Assuming a linear cost model ($\phi'' = 0$) does not make the world
   linear; it merely blinds the modeler to the convexity tax.

## Bridge

[Distribution and Moment](31-distribution-and-moment.md) examines moments and tail distributions, while
[The Non-Identical, Scale Invariance, Convexity, and Heavy Tails](46-non-identical-asymmetry-heavy-tails.md)
computes empirical Jensen gaps across heavy-tailed fire and financial loss data. In the Stochology library,
`vital.rs::VitalMarginState` formalizes this vital safeguard, incorporating the second-order Jensen debiasing
into the evaluation of Canguilhem–Holling attraction basins and reserve depletion.

[Integrals and Accumulation](13-integrals-and-accumulation.md) reverses the derivative: accumulation,
expectation as a Lebesgue integral, and coherent risk measures such as expected shortfall.
