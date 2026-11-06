# Model, Fit, and Error

> Foundation chapter of Part III. It presupposes [Chapter 09](09-sets-relations-functions.md)
> and [Chapter 14](14-linear-algebra.md). Its formal content is worked out in
> [Chapter 46](46-non-identical-asymmetry-heavy-tails.md) and
> [Chapter 48](48-reactive-worlds-performativity-goodhart.md).

## By the end of this chapter you can

1. Decompose the error of a fitted model into the part that shrinks with data and the part that
   does not.
2. Recognise information leakage through the order in which data were handled.
3. Say what a validation scheme silently assumes about the future, and when that assumption is
   false.

## 1. Two errors: approximation and estimation

The prediction error of any fitted statistical or machine learning model decomposes into two fundamentally
different mathematical components, and only one responds to increasing sample size:
- **Approximation error** (structural bias): The distance between the true physical process and the best possible
  model residing within the functional hypothesis class $\mathcal{F}$ you chose. It does not shrink as you
  collect more rows; it shrinks only when you expand or alter the mathematical family itself.
- **Estimation error** (sample variance): The distance between the specific model fitted on a finite training
  sample and the optimal model in that family. It shrinks toward zero as sample size $n \to \infty$, but expands
  rapidly as the complexity or dimension of the family increases.

This decomposition is the rigorous foundation of the bias-variance trade-off. Increasing model flexibility
invariably improves in-sample training fit, but inevitably reaches a threshold beyond which out-of-sample
generalization degrades. The model that describes historical archives with the highest empirical fidelity is
rarely the model that governs real-world operations best, because an unconstrained model inevitably overfits
and memorizes the archive's idiosyncrasies.

## 2. Four leakage pathways

Information frequently leaks from the chronological future of an analysis into its past. Every leakage channel
artificially inflates validation metrics while guaranteeing operational failure upon deployment:
- **Target leakage through a feature**: A predictor variable incorporates information that is causally downstream
  of the outcome. For example, using "assigned ICU bed type" to predict patient sepsis mortality when the ICU
  transfer was ordered after septic shock occurred. The algorithm is not forecasting the future; it is reading
  the outcome disguised as an input.
- **Temporal leakage**: Data collected after the operational moment of decision is incorporated into model
  training. The algorithm is fitted on information that will not exist when frontline operators are forced to act.
- **Group/Cluster leakage**: Clustered entities (the same patient across multiple visits, the same machine tool,
  or the same geographic municipality) are randomly partitioned across training and validation splits. The model
  memorizes individual entity signatures rather than learning structural generalities.
- **Preprocessing leakage**: Global scaling, quantile normalization, imputation, or feature selection is executed
  across the entire dataset prior to splitting into train and test folds. Information from the holdout validation
  distribution silently contaminates the training parameters.

The universal signature of data leakage is seductive: validation metrics look flawless. Detection cannot be
accomplished by checking loss functions; it requires an uncompromising audit of chronological provenance:
*what did each computational step know, and at what exact physical timestamp did it learn it?*

## 3. Goodhart's law and performative prediction

When a model is deployed to govern institutional decisions, it ceases to be a passive observer of an exogenous
world. In 2020, Juan Perdomo et al. formalized **performative prediction**: deploying a model parameter $\theta$
actively perturbs the data-generating mechanism, shifting the distribution of inputs and targets:

$$(X, Y) \sim \mathcal{D}(\theta) \qquad \text{\texttt{[Assumed]}}.$$

Optimizing against the historical distribution $\mathcal{D}(\theta_0)$ leads to performative collapse: the act
of scoring individuals changes their behavior, shifting the distribution away from the manifold on which the
model was trained.

In sociology, Donald MacKenzie demonstrated that economic models exhibit two forms of performativity:
*Barnesian performativity*, where adopting a model makes reality conform more closely to its assumptions, and
*counter-performativity*, where adopting a model creates incentives that destroy the model's own preconditions
(e.g., portfolio insurance models amplifying the 1987 market crash).

This performative feedback produces the four canonical variants of **Goodhart's Law**
(`governance.rs::GoodhartVariant::classify`):
1. **Regressional**: Extremes of a noisy metric reflect temporary luck; selecting upon them produces inevitable
   regression to the mean.
2. **Extremal**: Pushing a proxy metric to extreme values breaks the historical correlation between proxy and goal.
3. **Causal**: Acting on a non-causal correlate fails to move the intended outcome.
4. **Adversarial**: Strategic agents deliberately game and manipulate the metric to maximize private reward.

## 4. Kennedy-O'Hagan model discrepancy and isotonic calibration

Even under rigorous chronological validation, every model remains a mathematical simplification of an open world.
Following Marc Kennedy and Anthony O'Hagan (2001), the true physical response $y(x)$ must be formulated with an
explicit **structural model discrepancy** $\delta(x)$:

$$y(x) = f(x, \theta) + \delta(x) + \epsilon \qquad \text{\texttt{[Assumed]}}.$$

The discrepancy $\delta(x)$ is not random Gaussian white noise; it is a systematic, spatially or temporally
correlated deficiency. Attempting to tune parameters $\theta$ to absorb $\delta(x)$ produces severe parameter
confounding. An institution must estimate $\delta(x)$ directly through non-parametric methods, and declare where
its bounds remain uncontrolled.

Information geometry unifies model error, convexity, and calibrated probability forecasting `[Literature]`. Any
strictly convex loss potential $\phi$ induces a **Bregman divergence**:

$$D_{\phi}(x \parallel y) = \phi(x) - \phi(y) - \langle\nabla\phi(y), x - y\rangle \ge 0 \qquad \text{\texttt{[Assumed]}}.$$

When raw predicted probabilities are projected onto the convex cone of non-decreasing calibrated probabilities
via **Pool Adjacent Violators (PAV)** isotonic regression (`kernel.rs::pav_isotonic_non_decreasing`), the
generalised Pythagorean theorem partitions total Brier loss cleanly into three orthogonal components:

$$\text{Brier Loss} = \text{Miscalibration (REL)} - \text{Resolution (RES)} + \text{Uncertainty (UNC)} \qquad \text{\texttt{[Assumed]}}.$$

This decomposition allows an audit to verify whether a model's error stems from miscalibration (fixable via
isotonic recalibration), low informational resolution, or irreducible environmental stochasticity.

## 5. The tabular zoo, read through approximation and estimation error

Nothing in §1–§4 depends on the hypothesis class, which is why the standard families can be placed on the same
two axes rather than memorised as a list.

- **Additive trees and their boosted variants.** A single decision tree is a piecewise-constant approximation with
  high variance; a random forest averages many deep trees to cut that variance, at the cost of a bias that the
  averaging cannot remove; gradient boosting instead fits shallow trees sequentially to the residual, trading
  variance for bias one small step at a time. The distinction between bagging and boosting is exactly the
  decomposition of §1: bagging attacks estimation error, boosting attacks approximation error, and the
  hyper-parameters that matter for each are the opposite of one another.
- **Kernels and support-vector machines.** A kernel is a similarity function that implicitly maps the inputs into a
  feature space where a linear separator suffices — the change of basis whose absence made exclusive disjunction
  unlearnable ([Chapter 14](14-linear-algebra.md) §6). The regularisation parameter is a capacity control, and the
  kernel bandwidth is a statement about the scale on which the target is assumed to vary. Choosing it from data is
  estimation; declaring it is a modelling assumption, and the two have different failure modes.
- **Dimensionality reduction.** Projections and embeddings compress the input before the model sees it. They are
  legitimate when the target is a function of the retained subspace, and quietly destructive when it is a function
  of the annihilated one — the kernel of the projection, in the sense of rank-nullity.
- **Hyper-parameter search.** Random search, Bayesian optimisation and successive halving differ in how they spend a
  fixed evaluation budget, not in what they guarantee. Every one of them produces a *selection* estimate, which is
  optimistic by construction because the winner was chosen on the same data it was scored on. That is the same
  optimism the split-half PAV debiasing removes for calibration, and it must be removed here too or the reported
  score is an in-sample artifact.

The invariant across the zoo is the one this chapter has insisted on: the model class constrains *what* can be
approximated, the sample constrains *how well* it can be estimated, and no amount of tuning converts the first into
the second. A pipeline that reports a single validation number, computed after selecting the class and the
hyper-parameters on the same archive, has reported the maximum of a noisy quantity — which is precisely the
regressional Goodhart mechanism of [Chapter 48](48-reactive-worlds-performativity-goodhart.md).

## Bridge

Validation assumptions and the convexity slack of unobserved dispersion are formalised in
[Chapter 46](46-non-identical-asymmetry-heavy-tails.md) as
`vital.rs::VitalMarginState::from_heavy_tails_and_asymmetry`; post-hoc probability calibration via
pool-adjacent-violators isotonic regression is executed by
`kernel.rs::pav_isotonic_non_decreasing`; and structural model discrepancy alongside performative
Goodhart audits is formalised in
[Chapter 48](48-reactive-worlds-performativity-goodhart.md) as
`governance.rs::GovernanceState::from_ex_ante_gate`.
