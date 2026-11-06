# Chapter 76 — Layer 26: Sequential Decision Theory, Optimal Stopping, Bandits & Real Options Order

> Layer 26 (`multilayer.rs::KnowledgeOrder`, `KnowledgeOrder::L26SequentialDecisionBanditRealOptions`) of Part V · Stratum VI (`multilayer.rs::KnowledgeStratum`, `KnowledgeStratum::S6StrategicEconomic`) · Unique Guard Combination `{T1Framing, T2Severity, T5Temporal}` · Signature `(2, 1, 0)` (`multilayer.rs::TriadicExposureRegime`, `TriadicExposureRegime::GovernanceBlindSpot`). It builds on [Layer 01](51-layer-micro-kinetic-sensorimotor-order.md), [Layer 07](57-layer-probability-credence-measurement-order.md), [Layer 09](59-layer-experimental-sampling-causal-order.md) in $\mathcal{G}_1$ and maps into $\mathcal{G}_2$ via [`notes/The Multilayer Graph of Knowledge and Aim.md`](../notes/The%20Multilayer%20Graph%20of%20Knowledge%20and%20Aim.md).

## By the end of this chapter you can

1. Solve optimal stopping problems via the Snell envelope and derive the classical $1/e \approx 36.8\%$
   exploration threshold in finite horizon search.
2. Prove how Gittins indices decouple multi-armed bandits and explain the Lai–Robbins $\Omega(\ln T)$
   asymptotic regret bound under exploration–exploitation tradeoffs.
3. Analyze why a `(2, 1, 0)` sequential decision rule lacking institutional governance ($S_{\mathrm{gov}}$)
   suffers Cynical Capture (`synthesis.rs::CollapseProposition`) when option values are privatized and
   downside absorption is externalized.

## 1. Optimal stopping: Snell envelope and the $1/e$ Secretary threshold

Sequential decision-making under uncertainty requires deciding not only *what* action to take, but *when*
to terminate inquiry and commit irreversible resources. On a filtered probability space
$(\Omega, \mathcal{F}, (\mathcal{F}_t)_{t=0}^T, \mathbb{P})$, let $Z_t$ be an adapted reward process. The
value of the optimal stopping problem is governed by the **Snell envelope** $U_t$ `[Literature]`:

$$U_T = Z_T, \qquad U_t = \max\bigl(Z_t,\, \mathbb{E}[U_{t+1} \mid \mathcal{F}_t]\bigr) \quad \text{for } t = T-1, \dots, 0 \qquad \text{\texttt{[Assumed]}}.$$

The stopping time $\tau^* = \inf\{t \ge 0 : Z_t = U_t\}$ is the smallest optimal stopping time.

In the classical Secretary Problem (rank-based selection of $n$ candidates without recall), the agent
observes independent uniform permutations. To maximize the probability of selecting the absolute best
candidate, the decision-maker must reject an initial calibration cohort of size $r-1$ and choose the first
subsequent candidate who dominates all previous arrivals. The win probability satisfies:

$$P(r) = \frac{r - 1}{n} \sum_{k=r}^n \frac{1}{k - 1} \approx -\frac{r}{n} \ln\!\left(\frac{r}{n}\right) \qquad \text{\texttt{[Assumed]}}.$$

Maximizing this concave envelope yields $r^* / n \to 1/e \approx 0.3679$. The rational decider must
dedicate the initial $36.8\%$ of the temporal horizon strictly to calibrating the empirical distribution
(`T1Framing`, `T2Severity`), refusing all premature offers regardless of apparent quality.

## 2. Multi-armed bandits: Gittins index and Lai–Robbins regret

Herbert Robbins (1952) formalized the exploration–exploitation dilemma in multi-armed bandits. In an
environment with $K$ independent arms yielding stochastic rewards with unknown means $\mu_i$, early
choices must probe uncertain levers to gather evidence, while later choices exploit the empirical optimum.

John Gittins (1979) established the **Gittins Index Theorem** `[Literature]`: under geometric discounting
$\beta \in (0, 1)$, the complex multi-dimensional Markov decision process decouples completely into $K$
independent one-dimensional optimal stopping problems. Each arm $i$ at state $s_i$ is assigned a dynamic
allocation index:

$$\nu_i(s_i) = \sup_{\tau > 0} \frac{\mathbb{E}\left[\sum_{t=0}^{\tau-1} \beta^t R_t(s_i) \mid s_i\right]}{\mathbb{E}\left[\sum_{t=0}^{\tau-1} \beta^t \mid s_i\right]} \qquad \text{\texttt{[Assumed]}}.$$

The optimal policy simply activates whichever arm currently exhibits the highest index $\nu_i(s_i)$.

In the non-discounted cumulative regret setting over horizon $T$, Tze Leung Lai and Herbert Robbins (1985)
`[Literature]` proved the fundamental asymptotic lower bound on cumulative regret
$R_T = T \mu^* - \sum_{t=1}^T \mathbb{E}[\mu_{I_t}]$:

$$\liminf_{T \to \infty} \frac{R_T}{\ln T} \ge \sum_{i: \mu_i < \mu^*} \frac{\mu^* - \mu_i}{D_{\mathrm{KL}}(p_i \parallel p^*)} \qquad \text{\texttt{[Assumed]}}.$$

Regret must grow at least logarithmically in time $T$; any algorithm attempting sub-logarithmic regret必然
suffers unbounded exponential regret on bad parameter configurations.

## 3. Real options, Wald SPRT, and irreversible commitment

In sequential testing between hypotheses $H_0$ and $H_1$, Abraham Wald's Sequential Probability Ratio Test
(SPRT) defines an open continuation band $(b, a)$. In Stochology, Ville's anytime-valid test martingales
(`epistemic.rs::EpistemicMarginState::from_ate_bounds`) establish severe stopping boundaries without
pre-specifying sample size.

In corporate capital allocation, Avinash Dixit and Robert Pindyck (*Investment Under Uncertainty*, 1994)
`[Literature]` showed that when capital investments involve sunk costs (`T5Temporal`), classical net
present value ($\mathrm{NPV} > 0$) severely over-invests. The real option to defer commitment has positive
economic value $V_{\mathrm{option}} > 0$. The firm should pull the trigger only when expected return
exceeds a widened hurdle:

$$\mathbb{E}[R] \ge I + V_{\mathrm{option}} \qquad \text{\texttt{[Assumed]}}.$$

In Stochology's three regimes of action (`notes/What Must Be Known When We Do Not Know.md`), this
corresponds to **Reversible Fractional Probing**: deploying small, survivable Kelly fractions
$f_{\mathrm{BM}} \in (0, f^*)$ (`vital.rs::VitalMarginState::from_non_ergodic_returns`) to extract
information while hedging against irreversible ruin.

## 4. The three collapses in sequential decision theory

Layer 26's `(2, 1, 0)` signature (`TriadicExposureRegime::GovernanceBlindSpot`) omits $S_{\mathrm{gov}}$
(`T6`, `T7`):
1. **Collapse Proposition 1 ($\neg S_{\mathrm{epi}}$ — Premature Locking):** Terminating bandit exploration
   prematurely on early winning streaks without Cromwell probability floors
   (`epistemic.rs::EpistemicMarginState::from_inquiry_framing`), falling into permanent suboptimal arm
   lock-in (`synthesis.rs::CollapseProposition`, `CollapseProposition::Proposition1BlindUndersizing`).
2. **Collapse Proposition 2 ($\neg S_{\mathrm{vital}}$ — Gambler's Ruin Under Log-Regret):** Achieving
   theoretical $\mathcal{O}(\ln T)$ asymptotic regret while leverage exceeds the Kelly barrier, suffering
   absorbing bankruptcy during finite-horizon variance shocks (`synthesis.rs::CollapseProposition`,
   `CollapseProposition::Proposition2ImpotentLucidity`).
3. **Collapse Proposition 3 ($\neg S_{\mathrm{gov}}$ — Cynical Call-Option Hoarding):** Executive agents
   treating irreversible capital projects as personal call options, extracting high bonuses on upside
   volatility while abandoning downside liabilities and toxic cleanup costs to public taxpayers
   (`governance.rs::GovernanceState::from_real_time_and_ex_post_gates`, `synthesis.rs::CollapseProposition`,
   `CollapseProposition::Proposition3CynicalCapture`).

## Bridge

Layer 26 (`multilayer.rs::KnowledgeOrder`, `KnowledgeOrder::L26SequentialDecisionBanditRealOptions`,
`{T1Framing, T2Severity, T5Temporal}`) propagates its local and ancestor-cone margins through
`multilayer.rs::MultilayerHomomorphism::evaluate` into [Chapter 77 — Layer 27: Game Theory, Strategic
Classification & Adversarial Learning Order](77-layer-game-theory-strategic-adversarial-order.md)
(`KnowledgeOrder::L27GameTheoryStrategicAdversarial`, Stratum VI,
`{T1Framing, T2Severity, T6Performative}`).
