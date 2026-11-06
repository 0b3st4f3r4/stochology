# Chapter 77 — Layer 27: Game Theory, Strategic Classification & Adversarial Learning Order

> Layer 27 (`multilayer.rs::KnowledgeOrder`, `KnowledgeOrder::L27GameTheoryStrategicAdversarial`) of Part V · Stratum VI (`multilayer.rs::KnowledgeStratum`, `KnowledgeStratum::S6StrategicEconomic`) · Unique Guard Combination `{T1Framing, T2Severity, T6Performative}` · Signature `(2, 0, 1)` (`multilayer.rs::TriadicExposureRegime`, `TriadicExposureRegime::VitalBlindSpot`). It builds on [Layer 07](57-layer-probability-credence-measurement-order.md), [Layer 10](60-layer-cryptographic-zero-knowledge-adversarial-order.md), [Layer 26](76-layer-sequential-decision-bandit-real-options-order.md) in $\mathcal{G}_1$ and maps into $\mathcal{G}_2$ via [`notes/The Multilayer Graph of Knowledge and Aim.md`](../notes/The%20Multilayer%20Graph%20of%20Knowledge%20and%20Aim.md).

## By the end of this chapter you can

1. Contrast Nash and Stackelberg equilibria under strategic classification, formalizing how evaluated agents game
   published decision boundaries $\arg\max_x f(\theta, x) - c(x, x_0)$.
2. Prove the contraction condition $\varepsilon < \gamma / \beta$ in performative prediction and distinguish
   performatively stable points from performatively optimal policies.
3. Explain why a `(2, 0, 1)` strategic equilibrium lacking vital margins ($S_{\mathrm{vital}}$) collapses into Impotent
   Lucidity Before Ruin (`synthesis.rs::CollapseProposition`) when repeated multiplicative games trigger gambler's ruin
   before asymptotic convergence.

## 1. Strategic classification: Hardt et al. and the Stackelberg game

When machine learning algorithms are deployed in high-stakes social and economic domains (credit underwriting, pretrial
release, tax audits, algorithmic hiring), the underlying data distribution is not passive. Evaluated agents actively
adapt their features to clear decision thresholds.

Moritz Hardt, Nimrod Megiddo, Christos Papadimitriou, and Mary Wootters (2016) `[Literature]` formalized **Strategic
Classification** as a leader-follower Stackelberg game. The institutional principal deploys classifier $\theta$. An
agent with true baseline profile $x_0$ and cost function $c(x, x_0)$ selects modified features $x$ by solving:

$$x^*(\theta, x_0) = \arg\max_x \bigl(f(\theta, x) - c(x, x_0)\bigr) \qquad \text{\texttt{[Assumed]}}.$$

In a simultaneous Nash game, each player treats the other's action as fixed. In contrast, under Stackelberg
equilibrium, the institutional learner anticipates the agent's best response function $x^*(\theta)$ and optimizes the
loss over induced features:

$$\min_\theta \mathbb{E}_{(x_0, y)}\!\left[\ell\bigl(f(\theta, x^*(\theta, x_0)),\, y\bigr)\right] \qquad \text{\texttt{[Assumed]}}.$$

When manipulation costs are quadratic ($c(x, x_0) = \frac{1}{2}\|x - x_0\|_2^2$), the best-response displacement
$x^*(\theta, x_0) - x_0$ aligns colinearly with the scoring gradient $\nabla_x f(\theta, x)$, demonstrating that
publishing scoring weights bends population feature geometry prior to outcome evaluation.

Ignoring strategic recourse causes immediate Goodhart metric collapse
(`governance.rs::GovernanceState::from_ex_ante_gate`): proxy metrics that appeared predictive in historical logs decay
as agents invest effort in superficial signal gaming rather than true productivity gains.

## 2. Performative prediction and gradient dynamics

Juan Perdomo, Tijana Zrnic, Celestine Mendler-Dünner, and Moritz Hardt (2020) `[Literature]` generalized strategic
classification into the framework of **Performative Prediction**. Here, the data-generating distribution
$\mathcal{D}(\theta)$ is an endogenous function of the deployed parameter $\theta$.

Let the distribution map $\mathcal{D}(\cdot)$ be $\varepsilon$-sensitive in Wasserstein-1 distance
($W_1(\mathcal{D}(\theta), \mathcal{D}(\theta')) \le \varepsilon \|\theta - \theta'\|$), and assume the loss function
$\ell(z; \theta)$ is $\beta$-jointly smooth and $\gamma$-strongly convex in $\theta$. In repeated risk minimization
(RRM), the institution repeatedly retrains on fresh outcomes:

$$\theta_{t+1} = \arg\min_\theta \mathbb{E}_{Z \sim \mathcal{D}(\theta_t)}[\ell(Z; \theta)] \qquad \text{\texttt{[Assumed]}}.$$

This dynamical sequence converges to a unique **performatively stable point** $\theta_{\mathrm{PS}}$ if and only if the
performative sensitivity satisfies the strict contraction condition
(`governance.rs::GovernanceState::from_ex_ante_gate`):

$$\varepsilon < \frac{\gamma}{\beta} \qquad \text{\texttt{[Assumed]}}.$$

Under Perdomo's framework, repeated risk minimization defines an operator
$G(\theta) = \arg\min_{\theta'} \mathbb{E}_{Z \sim \mathcal{D}(\theta)}[\ell(Z; \theta')]$ whose contraction modulus
satisfies $L_G \le \varepsilon \beta / \gamma < 1$. By Banach's fixed-point theorem, this contraction guarantees linear
convergence to $\theta_{\mathrm{PS}}$. When performative coupling violates contraction
($\varepsilon \beta \ge \gamma$), iterative retraining destabilizes into limit cycles or divergence.

Furthermore, even when $\theta_{\mathrm{PS}}$ exists, it does not minimize true performative risk
$\mathrm{PR}(\theta) = \mathbb{E}_{Z \sim \mathcal{D}(\theta)}[\ell(Z; \theta)]$. Reaching **performative optimality**
$\theta_{\mathrm{PO}}$ requires performative gradient descent, which accounts for the chain-rule term
$\nabla_\theta \mathcal{D}(\theta)$.

## 3. Adversarial minimax robustness and Schelling focal framing

When strategic behavior turns adversarial, institutions must guard against worst-case feature corruptions rather than
cost-bounded manipulations. In robust optimization (Szegedy et al., 2014; Madry et al., 2018 `[Literature]`), the
learner minimizes worst-case empirical risk:

$$\min_\theta \mathbb{E}_{(x, y)}\!\left[\max_{\|\delta\| \le \epsilon} \ell(\theta, x + \delta, y)\right] \qquad \text{\texttt{[Assumed]}}.$$

This minimax formulation embodies severe testing (`T2Severity`): a decision rule is accepted only if no permissible
perturbation within the $\epsilon$-ball can induce classification error.

Simultaneously, Thomas Schelling (*The Strategy of Conflict*, 1960 `[Literature]`) proved that game-theoretic
formalisms cannot resolve equilibrium selection in coordination or deterrence games without institutional **focal
points** (`T1Framing`, `epistemic.rs::EpistemicMarginState::from_inquiry_framing`). Shared cultural conventions, legal
salience, and architectural defaults establish the tacit expectations that guide agents toward a specific equilibrium.

## 4. The three collapses in game-theoretic orders

Layer 27's `(2, 0, 1)` signature (`TriadicExposureRegime::VitalBlindSpot`) lacks $S_{\mathrm{vital}}$ (`T4`, `T5`):
1. **Collapse Proposition 1 ($\neg S_{\mathrm{epi}}$ — Exogeneity Blindness):** Assuming evaluated distributions are
   static ($\varepsilon = 0$), deploying brittle linear scorecards that are gamed into irrelevance within months
   (`synthesis.rs::CollapseProposition`, `CollapseProposition::Proposition1BlindUndersizing`).
2. **Collapse Proposition 2 ($\neg S_{\mathrm{vital}}$ — Impotent Lucidity Before Ruin):** Agents computing exquisite
   infinite-horizon Nash or Stackelberg equilibria while playing multiplicative games with negative time-average growth
   ($g_{\mathrm{time}}(\ell) < 0$ in `vital.rs::VitalMarginState::from_non_ergodic_returns`), suffering absorbing capital
   ruin before equilibrium convergence (`synthesis.rs::CollapseProposition`,
   `CollapseProposition::Proposition2ImpotentLucidity`).
3. **Collapse Proposition 3 ($\neg S_{\mathrm{gov}}$ — Predatory Manipulation Arms Race):** Platforms deploying
   hyper-targeted classifiers to exploit behavioral weaknesses, generating massive deadweight manipulation costs without
   social surplus (`synthesis.rs::CollapseProposition`, `CollapseProposition::Proposition3CynicalCapture`).

## Bridge

Layer 27 (`multilayer.rs::KnowledgeOrder`, `KnowledgeOrder::L27GameTheoryStrategicAdversarial`,
`{T1Framing, T2Severity, T6Performative}`) propagates its local and ancestor-cone margins through
`multilayer.rs::MultilayerHomomorphism::evaluate` into [Chapter 78 — Layer 28: Macro-Finance, Econometrics, Actuarial
Ruin & Endogenous Risk Order](78-layer-macro-finance-econometrics-actuarial-order.md)
(`KnowledgeOrder::L28MacroFinanceEconometricsActuarial`, Stratum VI, `{T2Severity, T4Formalization, T6Performative}`).
