# Chapter 58 — Layer 08: Metrological & Information-Geometric Order

> Layer 08 (`multilayer.rs::KnowledgeOrder`, `KnowledgeOrder::L08MetrologicalInfoGeometric`) of Part V · Stratum II (`multilayer.rs::KnowledgeStratum`, `KnowledgeStratum::S2FormalMetrological`) · Unique Guard Combination `{T3Archive, T4Formalization, T6Performative}` · Signature `(1, 1, 1)` (`multilayer.rs::TriadicExposureRegime`, `TriadicExposureRegime::TransTriadicKeystone`). It builds on [Layer 04](54-layer-geophysical-chaos-data-assimilation-order.md), [Layer 07](57-layer-probability-credence-measurement-order.md) in $\mathcal{G}_1$ and maps into $\mathcal{G}_2$ via [`notes/The Multilayer Graph of Knowledge and Aim.md`](../notes/The%20Multilayer%20Graph%20of%20Knowledge%20and%20Aim.md).

## By the end of this chapter you can

1. Decompose combined measurement uncertainty into its Type A ($1/\sqrt{n}$) and Type B ($O(1)$)
   components, and show why four operators of this framework are instances of that law.
2. State what Čencov's theorem and the Fisher–Rao metric measure, and how they govern epistemic
   shrinkage under estimation error.
3. Use Bregman's generalised Pythagorean theorem to unify Jensen's convexity gap, model-approximation
   error, and isotonic calibration decomposition.

## 1. Type A and Type B uncertainty in physical metrology

The international standard of physical measurement — the **Guide to the Expression of Uncertainty in
Measurement** (`BIPM/JCGM GUM` `[Literature]`) — writes the combined variance of a measurement
$y = f(x_1,\dots,x_m)$ as the sum of two structurally distinct terms:

$$u_{c}^{2}(y; n) = \frac{\sigma_{A}^{2}\,\mathrm{DEFF}}{n} + u_{B}^{2}(y) \qquad \text{\texttt{[Assumed]}}.$$

**Type A uncertainty** captures random scatter across repeated observations; divided by the cluster
design effect $\mathrm{DEFF}$, it shrinks to zero as $n \to \infty$. **Type B uncertainty** $u_{B}(y)$
captures systematic calibration bias, finite instrument resolution, environmental drift, and the
**definitional uncertainty** ($u_{\mathrm{def}}$, `VIM 2.27` `[Literature]`) intrinsic to how the
measurand is specified. Crucially, $u_{B}(y)$ is independent of sample size $n$.

Four core equations of this book are exact realisations of that metrological split:
- In `epistemic.rs::EpistemicMarginState::unified_effective_n`, the sampling term $\mathrm{DEFF}/n$ is
  Type A, whereas Meng's selection floor $\frac{1-f}{f}\rho_{R,Y}^2$ is a pure Type B variance floor that
  never vanishes as $n \to \infty$;
- In `epistemic.rs::from_ate_bounds`, the Imbens–Manski sampling margin is Type A, whereas the
  partial-identification width $U_t - L_t$ is Type B ambiguity;
- In `governance.rs::from_ex_ante_gate`, finite-sample estimation scatter is Type A, whereas the
  Kennedy–O'Hagan structural discrepancy $\delta_{\mathrm{KOH}}(x)$ is Type B model inadequacy;
- And the **unbroken chain of metrological traceability** (`VIM 2.41` `[Literature]`) — requiring every
  downstream instrument to inherit at least the uncertainty $u_{\ell} \ge u_{\ell-1}$ of its upstream
  reference standard — is the physical law behind bitemporal lineage sealing.

## 2. Čencov's theorem and the Fisher–Rao manifold

When a statistical model family $\mathcal{M} = \{p(x;\theta)\}$ is treated as a smooth manifold,
Nikolai Čencov's theorem (1972) `[Literature]` proves that the **Fisher–Rao metric**

$$g_{ij}(\theta) = \mathbb{E}_{\theta}\!\left[\partial_{i}\ln p(X;\theta)\,\partial_{j}\ln p(X;\theta)\right] \qquad \text{\texttt{[Assumed]}}$$

is the *unique* Riemannian metric invariant under smooth reparameterisations and sufficient statistics.

Read geometrically, the squared Mahalanobis signal-to-noise ratio that governs epistemic Kelly
shrinkage in `vital.rs::from_non_ergodic_returns` (the Baker–McHale shrinkage factor
$k_{\mathrm{BM}} \in (0,1)$ `[Literature]`) is the squared Fisher–Rao geodesic distance between the
estimated drift and the boundary of zero net growth. Whenever that geodesic distance is comparable to
the Type A estimation radius $1/\sqrt{n_{\mathrm{eff}}}$, sizing a bet on the plug-in point estimate
mistakes noise on the manifold for a real gradient.

## 3. Bregman divergences and the generalised Pythagorean theorem

Every strictly convex, differentiable potential $\phi: \Omega \to \mathbb{R}$ generates a **Bregman
divergence**

$$D_{\phi}(x \| y) = \phi(x) - \phi(y) - \langle\nabla\phi(y),\, x - y\rangle \ge 0 \qquad \text{\texttt{[Assumed]}}.$$

Three seemingly separate results in Part IV are projections under $D_{\phi}$:

1. **Jensen's convexity gap (`vital.rs::from_heavy_tails_and_asymmetry`):** For any random shock $X$ with
   mean $\mu = \mathbb{E}[X]$, the expected Bregman divergence from the mean is identically the Jensen
   gap, $\mathbb{E}[D_{\phi}(X \| \mu)] = \mathbb{E}[\phi(X)] - \phi(\mu) = \Delta_{J} \ge 0$ `[Assumed]`.
2. **Orthogonal error decomposition ([Chapter 37](37-model-fit-and-error.md)):** For any convex model
   hypothesis class $\mathcal{C}$ and projection $p^* = \arg\min_{q \in \mathcal{C}} D_{\phi}(y \| q)$,
   Bregman's **generalised Pythagorean inequality** guarantees
   $D_{\phi}(y \| q) \ge D_{\phi}(y \| p^*) + D_{\phi}(p^* \| q)$, separating Type B approximation error
   $D_{\phi}(y \| p^*)$ from Type A estimation error $D_{\phi}(p^* \| \hat{p}_n)$.
3. **Isotonic calibration (`kernel.rs::pav_isotonic_non_decreasing`):** For quadratic $\phi(u) = u^2$,
   projecting binary outcomes $y \in \{0,1\}^n$ onto the convex cone of non-decreasing calibrated
   probabilities $\hat{c} = \mathrm{PAV}(p)$ yields the exact Brier identity
   $\mathrm{BS} = \mathrm{REL}_{\mathrm{CORP}} - \mathrm{RES}_{\mathrm{CORP}} + \mathrm{UNC}_{0}$ with
   zero residual cross-term.

## 4. Implicit representations: when the map is the measurement

A metrological claim is a claim about a field: pressure over a surface, density over a volume, a signed
distance over a room. Classical pipelines discretise the field first and then measure the grid, which
quietly makes the grid the object of record. A family of architectures reverses the order.

**SIREN** (sinusoidal representation networks) parameterises the field directly as a function of
coordinates, $f_\theta(x) = W_n \sin(\omega_0 W_{n-1} \cdots \sin(\omega_0 W_1 x + b_1) \cdots) + b_n$,
and fits $\theta$ to samples `[Literature]`. Three properties are metrologically relevant:

1. **The representation is continuous.** Evaluating at a coordinate that was never sampled is a forward
   pass, not an interpolation rule. There is no grid to fall between, and no cell-size artefact in the
   reported value.
2. **Derivatives are available analytically.** The gradient and the Laplacian of the field follow from the
   chain rule through the sine activations, and the sinusoidal basis is what keeps those derivatives
   informative — a ReLU network is piecewise linear, so its second derivative is zero almost everywhere
   and its curvature estimate is an artefact of the breakpoints. For a field whose measurement model
   involves a differential operator, this is the difference between measuring the field and measuring the
   mesh.
3. **The periodicity is a prior, and priors are checkable.** $\omega_0$ fixes the spatial frequency band
   the network can represent cheaply. A field with structure outside that band is under-fitted in a way
   that looks like noise, which is the classical bandwidth problem of any instrument.

The same reading covers attention. A self-attention layer computes, for each position, a convex combination of the
other positions weighted by a learned similarity $\exp(\langle q, k \rangle / \sqrt{d})$; the similarity is a
kernel, the weights are a partition of unity, and the layer is a data-dependent smoothing operator whose bandwidth is
a parameter of the model. Two consequences follow that the applied literature usually states as engineering advice
and that belong here as metrology: the learned kernel is fitted on a corpus, so it inherits the corpus's support and
its missingness pattern; and the "scale" of a model — parameter count, data volume, compute — enters only through
empirically measured power laws, which are extrapolations over a fitted range and therefore carry exactly the
Kennedy–O'Hagan caveat of the next paragraph. A scaling prediction is a claim about the region where the law was
measured, not about the region where the next model will live.

The stochological reading is the one this chapter has been building. An implicit representation is a
*model with a declared support*, and the two questions of [Chapter 37](37-model-fit-and-error.md) apply
unchanged: approximation error (can the chosen band represent the field?) and estimation error (how much
does the fit move with the sample?). What changes is that the support is continuous, so the
Kennedy–O'Hagan discrepancy $\delta(x)$ becomes a function on a continuum rather than on a finite set of
strata, and "outside the support" stops being a discrete event. The Type B uncertainty of Collapse
Proposition 1 does not disappear when the representation is smooth; it becomes harder to notice, because
the network will happily return a plausible number at a coordinate where no measurement exists.

## 5. The supply side of a scaling law

§4 closed with the observation that a scaling law is a fitted relation over a measured range. The natural next
question is metrological: what is the *input* whose growth the law indexes?

For contemporary language models the input is text — and, more precisely, a particular kind of text: human-produced,
digitised, legally reachable, and adequately represented in the languages and domains the model is claimed to serve.
Each of those adjectives is a filter, and every filter is an archive boundary in the sense of
[Chapter 45](45-world-to-archive-big-data-paradox.md). The quantity that a scaling law treats as a smooth variable is
therefore a stock with a supply curve, and the supply curve is finite on any reasonable horizon. The projection that
the two curves cross — human data supply flattening while model consumption keeps rising — is the operational form of
the claim, and it is a projection, not a measurement `[Literature]`.

Three metrological consequences follow, and they are the ones this layer exists to state.

1. **A fitted law extrapolated past its input regime is not a forecast.** The exponent was estimated where the stock
   was abundant. Beyond the crossing, the regime changed, and the correct statement is the one §4 already imposes:
   the relation holds inside the support where it was measured, and outside it the honest output is a bound, not a
   point.
2. **The filter is where the bias lives.** Text that is digitised, reachable and licensed is not a random sample of
   what people know; it is a sample of what was recorded, by whom, in which language. The effective sample size of a
   corpus is therefore far below its token count, which is the Keynes weight of
   [Chapter 45](45-world-to-archive-big-data-paradox.md) applied to a training set rather than to a survey.
3. **Synthetic data moves the boundary and does not remove it.** Training on generated text changes *which* stock is
   being consumed — from recorded human output to modelled output — and a model fitted largely on its predecessors'
   outputs inherits their discrepancy, with the extra property that the discrepancy is now correlated across the
   corpus. That is a design effect, not a variance, and it does not shrink with more tokens.

The invariance this chapter has insisted on since Čencov applies unchanged: the representation is not the thing
measured. A corpus is an instrument reading of human production, with a calibration range, a resolution and a
missingness pattern. The layer's discipline is to carry those three numbers next to any capability claim that
depends on the corpus — and to refuse the reading that treats the supply curve as a law of nature rather than as a
statement about what has been recorded so far.

## 6. The three collapses in the metrological order

1. **Collapse Proposition 1 ($\neg S_{\mathrm{epi}}$ — Erasing Type B Uncertainty):** Reporting only the
   Type A standard error $\sigma/\sqrt{n}$ of a massive sensor or administrative database while setting
   definitional and selection uncertainty $u_{B}$ to zero.
2. **Collapse Proposition 2 ($\neg S_{\mathrm{vital}}$ — Ignoring the Bregman–Jensen Penalty):** Sizing
   reserves at $\phi(\mathbb{E}[X])$ rather than $\phi(\mathbb{E}[X]) + \mathbb{E}[D_{\phi}(X \| \mathbb{E}[X])]$
   until a convex surge exhausts the buffer.
3. **Collapse Proposition 3 ($\neg S_{\mathrm{gov}}$ — Breaking the Traceability Chain):** Publishing
   recalibrated composite scores that strip upstream calibration intervals or extrapolate outside the
   support where $\delta_{\mathrm{KOH}}(x)$ was measured.

## Bridge

Layer 08 (`multilayer.rs::KnowledgeOrder`, `KnowledgeOrder::L08MetrologicalInfoGeometric`, `{T3Archive, T4Formalization, T6Performative}`) propagates its local and ancestor-cone margins through `multilayer.rs::MultilayerHomomorphism::evaluate` into [Chapter 59 — Layer 09: Experimental Design, Complex Survey Sampling & Causal Methodology Order](59-layer-experimental-sampling-causal-order.md) (`KnowledgeOrder::L09ExperimentalSamplingCausal`, Stratum II, `{T1Framing, T2Severity, T3Archive}`).
