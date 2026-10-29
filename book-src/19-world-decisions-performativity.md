# 19. When the Decision Moves the World

## When the Forecast Changes the Weather It Predicts

A meteorologist predicting tomorrow's rain does not change the clouds by publishing a forecast. A central bank predicting inflation, a navigation app predicting street congestion, or a municipal authority tying hospital funding to waiting-time targets operates in a fundamentally different regime: **publishing or acting on the model in Layer $L\_3$ causes agents in Layer $L\_1$ to change their behavior**, shifting the distribution of the world from $\mathcal{D}$ to $\mathcal{D}(\theta)$ across **Bridge $L\_3 \to L\_1$ (*World-Decisions*)**.

## 19.1 Bridge $L\_3 \to L\_1$: Performative Prediction and Repeated Risk Minimization

### Foundations: When Deploying the Model Deforms the World

Standard supervised learning assumes that the data-generating distribution $\mathcal{D}$ is a passive photograph of nature: whether you deploy parameter vector $\theta\_1$ or $\theta\_2$, the world keeps drawing pairs $Z = (X, Y) \sim \mathcal{D}$.

In human and institutional systems, this assumption is false. As economist **Robert Lucas** pointed out in 1976 (*the Lucas Critique*), and as **Charles Goodhart** formulated in monetary policy, whenever an agent ($L\_3$) deploys a predictive model parameterized by $\theta \in \Theta$ to allocate credit, route traffic, set insurance premiums, or trigger police patrols, the population in the World ($L\_1$) reacts to the rule. **Deploying $\theta$ changes the distribution of the data from $\mathcal{D}$ to $\mathcal{D}(\theta)$.**

In 2020, Juan Perdomo, Tijana Zrnic, Celestine Mendler-Dünner, and Moritz Hardt unified this feedback loop under the mathematical theory of **Performative Prediction**.

To reason cleanly about Bridge $L\_3 \to L\_1$, we must distinguish two risk functions:

$$\mathrm{PR}(\theta) \triangleq \mathbb{E}\_{Z \sim \mathcal{D}(\theta)}[\ell(Z; \theta)], \qquad \mathrm{DPR}(\theta, \phi) \triangleq \mathbb{E}\_{Z \sim \mathcal{D}(\theta)}[\ell(Z; \phi)]$$

The two formulas differ in how they treat the world's response:
- In **Decoupled Performative Risk $\mathrm{DPR}(\theta, \phi)$**, the first argument $\theta$ fixes the **World's distribution** $\mathcal{D}(\theta)$ (how the population is currently behaving under the previously deployed policy $\theta$), while the second argument $\phi$ is the **new candidate model** being evaluated on yesterday's data.
- In **Performative Risk $\mathrm{PR}(\theta) = \mathrm{DPR}(\theta, \theta)$**, both arguments move together: it measures the true loss you will experience *after* you deploy $\theta$ and the world shifts to $\mathcal{D}(\theta)$.

This distinction separates two equilibria that are easy to conflate (Prohibition 3):

1. **Performative Stability ($\theta\_{\mathrm{PS}}$, The Myopic Retraining Fixed Point):**
   In a typical MLOps pipeline, data scientists collect data under the current model $\theta\_t$, retrain a new model $\theta\_{t+1} = \arg\min\_\phi \mathrm{DPR}(\theta\_t, \phi)$ as if the distribution were static, deploy $\theta\_{t+1}$, and repeat. This loop is called **Repeated Risk Minimization (RRM)**. If RRM converges to a fixed point where retraining no longer changes the parameters, that point is **Performatively Stable**:
   $$\theta\_{\mathrm{PS}} = \arg\min\_{\phi \in \Theta} \mathrm{DPR}(\theta\_{\mathrm{PS}}, \phi)$$
   At $\theta\_{\mathrm{PS}}$, the model looks optimal on its own fresh validation data, yet it remains blind to how its own deployment shifted $\mathcal{D}(\theta\_{\mathrm{PS}})$.

2. **Performative Optimality ($\theta\_{\mathrm{PO}}$, The True Stackelberg Equilibrium):**
   An agent who understands Bridge $L\_3 \to L\_1$ acts as a **Stackelberg leader**: instead of taking $\mathcal{D}(\theta)$ as frozen, the agent differentiates through the environment's reaction map $\theta \mapsto \mathcal{D}(\theta)$ and minimizes total Performative Risk directly:
   $$\theta\_{\mathrm{PO}} = \arg\min\_{\theta \in \Theta} \mathrm{PR}(\theta)$$

### Step-by-Step Derivation of the Canonical Linear Benchmark

Let us derive the exact gap between passive learning ($\theta\_{\mathrm{SL}}$), Performative Stability ($\theta\_{\mathrm{PS}}$), and Performative Optimality ($\theta\_{\mathrm{PO}})$ in the canonical linear-quadratic system implemented in `PerformativeSystem` (`crates/stochology-core/src/layers.rs`).

Suppose that when the institution deploys decision parameter $\theta$, the outcome in the world reacts linearly with performative sensitivity $\varepsilon$:

$$Y(\theta) = \mu + \varepsilon\theta + \xi, \qquad \mathbb{E}[\xi] = 0,\quad \mathrm{Var}(\xi) = \sigma^2$$

and the institution evaluates candidate parameter $\phi$ under the strongly convex regularized loss:

$$\ell(Y; \phi) = \frac{\gamma}{2}(Y - \phi)^2 - \alpha\phi \qquad (\gamma > 0)$$

1. **Passive Supervised Learning ($\theta\_{\mathrm{SL}}$, ignoring feedback $\varepsilon = 0$):**
   Before the model is deployed ($\theta = 0$), $\mathbb{E}[Y(0)] = \mu$. Minimizing $\mathbb{E}[\ell(Y(0); \phi)] = \frac{\gamma}{2}[(\mu - \phi)^2 + \sigma^2] - \alpha\phi$ with respect to $\phi$ sets the derivative to zero:
   $$-\gamma(\mu - \phi) - \alpha = 0 \implies \theta\_{\mathrm{SL}} = \mu + \frac{\alpha}{\gamma}$$

2. **Repeated Risk Minimization and Performative Stability ($\theta\_{\mathrm{PS}}$):**
   When $\theta\_t$ is deployed in round $t$, the environment shifts its mean to $\mathbb{E}[Y(\theta\_t)] = \mu + \varepsilon\theta\_t$. A myopic retrainer holds $\theta\_t$ fixed and minimizes $\mathrm{DPR}(\theta\_t, \phi)$ over $\phi$:
   $$-\gamma(\mu + \varepsilon\theta\_t - \phi) - \alpha = 0 \implies \theta\_{t+1} = \mu + \frac{\alpha}{\gamma} + \varepsilon\theta\_t$$
   Setting $\theta\_{t+1} = \theta\_t = \theta\_{\mathrm{PS}}$ gives the **Performative Stability fixed point**:
   $$\theta\_{\mathrm{PS}} = \frac{\mu + \alpha / \gamma}{1 - \varepsilon}$$

3. **True Performative Optimality ($\theta\_{\mathrm{PO}}$, Stackelberg Equilibrium):**
   Now substitute $\phi = \theta$ *before* differentiating. The true Performative Risk is:
   $$\mathrm{PR}(\theta) = \mathbb{E}\left[\frac{\gamma}{2}(\mu + \varepsilon\theta + \xi - \theta)^2 - \alpha\theta\right] = \frac{\gamma}{2}\bigl[(1 - \varepsilon)\theta - \mu\bigr]^2 + \frac{\gamma}{2}\sigma^2 - \alpha\theta$$
   Differentiating $\mathrm{PR}(\theta)$ with respect to $\theta$ (including the $(1 - \varepsilon)$ chain-rule factor from the environment's reaction) and setting the derivative to zero yields:
   $$\gamma(1 - \varepsilon)\bigl[(1 - \varepsilon)\theta\_{\mathrm{PO}} - \mu\bigr] - \alpha = 0 \implies \theta\_{\mathrm{PO}} = \frac{\mu}{1 - \varepsilon} + \frac{\alpha}{\gamma(1 - \varepsilon)^2}$$

Compare $\theta\_{\mathrm{PS}}$ and $\theta\_{\mathrm{PO}}$: whenever $\alpha \neq 0$ and $\varepsilon \neq 0$, **retraining to stability ($\theta\_{\mathrm{PS}}$) converges to a strictly suboptimal policy ($\theta\_{\mathrm{PS}} \neq \theta\_{\mathrm{PO}}$)** because myopic retraining ignores the derivative $\frac{\partial \mathcal{D}(\theta)}{\partial \theta}$.

What if the environment is highly reactive? Under Wasserstein-1 distribution sensitivity $W\_1(\mathcal{D}(\theta), \mathcal{D}(\theta')) \le \varepsilon\lVert\theta - \theta'\rVert\_2$, $\gamma$-strong convexity, and $\beta$-smoothness of the loss, Perdomo et al. proved that Repeated Risk Minimization contracts via:

$$\lVert\theta\_{t+1} - \theta\_{\mathrm{PS}}\rVert\_2 \le \left(\frac{\varepsilon\beta}{\gamma}\right) \lVert\theta\_t - \theta\_{\mathrm{PS}}\rVert\_2$$

Thus, RRM converges **if and only if the contraction ratio satisfies $r\_{\mathrm{RRM}} \triangleq \frac{\varepsilon\beta}{\gamma} < 1$**. Whenever $\frac{\varepsilon\beta}{\gamma} \ge 1$, retraining the model on its own induced data causes self-reinforcing oscillations that destabilize the real world (`--defect performativity`).

### Escaping the Myopic Fixed Point: Performative Gradient Descent (`PerfGD`)

How can an institution reach the true Stackelberg Performative Optimum $\theta\_{\mathrm{PO}}$ instead of getting trapped at the myopic retraining equilibrium $\theta\_{\mathrm{PS}}$? In 2021, **Zachary Izzo, Lexing Ying, and James Zou** introduced **Performative Gradient Descent (`PerfGD`)** (`PerformativeSystem::perfgd_trajectory`). Whereas standard retraining ignores how deploying $\theta$ shifts the distribution $\mathcal{D}(\theta)$, `PerfGD` updates $\theta\_t$ along the **total performative gradient**:

$$\nabla\_\theta \mathrm{PR}(\theta\_t) = \underbrace{\nabla\_2 \mathrm{DPR}(\theta\_t, \theta\_t)}\_{\text{Standard Gradient (Myopic)}} + \underbrace{\nabla\_1 \mathrm{DPR}(\theta\_t, \theta\_t)}\_{\text{Performative Distribution Shift Gradient}}$$

In our linear-quadratic system, $\nabla\_\theta \mathrm{PR}(\theta) = \gamma(1 - \varepsilon)\bigl[(1 - \varepsilon)\theta - \mu\bigr] - \alpha$, so stepping via $\theta\_{t+1} = \theta\_t - \eta\nabla\_\theta \mathrm{PR}(\theta\_t)$ converges directly to $\theta\_{\mathrm{PO}} = 2.03125$, achieving strictly lower Performative Risk than Repeated Risk Minimization ($\mathrm{PR}(\theta\_{\mathrm{PO}}) = -0.8203 < \mathrm{PR}(\theta\_{\mathrm{PS}}) = -0.8125$).

### Formal Taxonomy of the Four Variants of Goodhart's Law (`GoodhartTaxonomyAudit`)

When a statistical proxy metric $M$ is promoted into an optimization target on Bridge $L\_3 \to L\_1$, *"when a measure becomes a target, it ceases to be a good measure"* (Charles Goodhart 1975; Marilyn Strathern 1997). Following **David Manheim and Scott Garrabrant (2018)**, `GoodhartTaxonomyAudit` (`crates/stochology-core/src/layers.rs`) decomposes Goodhart collapse into four distinct mathematical mechanisms (`GoodhartVariant`):

1. **Regressional Goodhart (`GoodhartVariant::Regressional`):** Even without strategic gaming or causal change, when $M = Y + \xi$ ($\sigma\_\xi > 0$), conditioning on an extreme proxy realization $M = m$ incurs Galtonian shrinkage $\mathbb{E}[Y \mid M = m] = \mu\_Y + \rho\_{M,Y}\frac{\sigma\_Y}{\sigma\_M}(m - \mu\_M) < m$, producing systematic post-selection disappointment ($+0.600$ at $3\sigma$ when $\rho = 0.80$).
2. **Extremal Goodhart (`GoodhartVariant::Extremal`):** A proxy that correlates tightly with the true objective in the bulk of the distribution ($\alpha\_M = 3.2$) decouples in the tail when the true risk is heavy-tailed ($\alpha\_Y = 1.5$), creating a $q\_{99.9}$ tail gap ($+19.29$).
3. **Causal Goodhart (`GoodhartVariant::Causal`):** When $M$ is a downstream symptom or sibling effect of the true causal mechanism ($U \to Y$ and $U \to M$, so $M \notin \hat{S}^{\ast}$ under `run_nonlinear_icp_nile`), intervening via $\mathrm{do}(M = m)$ shifts the proxy while leaving $\mathbb{E}[Y \mid \mathrm{do}(M = m)] = \mu\_Y$ unchanged (causal intervention gap $+2.56$).
4. **Adversarial Goodhart (`GoodhartVariant::Adversarial`):** Rational agents game the proxy rule directly along the performative distribution-shift gradient $\nabla\_1 \mathrm{DPR}(\theta, \theta)$ (`PerformativeSystem::dpr_gradient_components`), inflating $M$ while degrading true Performative Risk $\mathrm{PR}(\theta\_{\mathrm{PS}}) - \mathrm{PR}(\theta\_{\mathrm{PO}}) = +0.0078 > 0$.

---

## 19.2 Prohibition 3 — Ban on Confusing Learning with Performativity (`PROHIBITION_3`)

### Why Low Validation Error Does Not Prove Causal or Performative Validity

Suppose a credit-scoring model or an airline ground-delay controller achieves a tiny Brier score on last year's holdout data. Two mechanisms across Rung 2 of Pearl's Ladder and Bridge $L\_3 \to L\_1$ can still break the model when deployed tomorrow:
1. **Confounded Passive Fit vs. Intervention:** A model that predicts well passively ($P(Y \mid X)$) may be riding a spurious proxy or confounder that breaks the moment the agent intervenes ($P(Y \mid \mathrm{do}(X))$).
2. **Performative Feedback ($\mathcal{D}(\theta)$):** When deploying parameter $\theta$ causes customers, borrowers, airlines, or drivers to change their behavior, the data distribution shifts from $\mathcal{D}$ to $\mathcal{D}(\theta)$.

### Mandatory Performative Audit on Bridge $L\_3 \to L\_1$

Under **Prohibition 3**, every model operating on Bridge $L\_3 \to L\_1$ must explicitly separate:
- **Performative Stability ($\theta\_{\mathrm{PS}}$):** the fixed point of myopic Repeated Risk Minimization (retraining passively on the distribution induced by the previous round);
- **Performative Optimality ($\theta\_{\mathrm{PO}}$):** the true Stackelberg equilibrium that anticipates the population's strategic response;
- **Wasserstein-1 Distribution Sensitivity ($\varepsilon$)** and the **Strict RRM Contraction Condition**:
  $$r\_{\mathrm{RRM}} = \frac{\varepsilon \beta}{\gamma} < 1$$

If $r\_{\mathrm{RRM}} \ge 1$, `PerformativeSystem` and `EpistemologicalCard::audit()` reject the card immediately (`--defect performativity`), preventing an automated retraining loop from destabilizing the real-world environment.

---

## Computational Audit in Rust

Run Laboratory 1 (`crates/stochology-core/src/layers.rs`) to inspect the Bridge $L\_3 \to L\_1$ performative prediction trajectory (`PerformativeSystem`) and the four-variant Goodhart audit (`GoodhartTaxonomyAudit`):

~~~bash
cargo run --release --bin labs -- --layers
~~~

~~~text
=== LAB 1: THREE ONTOLOGICAL LAYERS AND BRIDGE MECHANICS (L1 -> L2 -> L3 -> L1) ===
  [Bridge L3->L1] Performative Prediction: theta_SL=1.5000, theta_PS=1.8750, theta_PO=2.0312 (PerfGD=2.0312), PR(PS)=-0.8125 vs PR(PO)=-0.8203, r_RRM=0.300 (converges=true), oscillation_violated=[1.0, 0.0, 1.0, 0.0, 1.0] | Goodhart4(Reg=+1.07, Ext=+2.75, Causal_NILE=45.0x, Adv_grad1=-0.150, verified=true)
~~~

1. **On Bridge $L\_3 \to L\_1$**, with $\mu = 1.0, \varepsilon = 0.20, \alpha = 0.50, \gamma = 1.0, \beta = 1.50$, passive learning chooses $\theta\_{\mathrm{SL}} = 1.5000$, Repeated Risk Minimization converges ($r\_{\mathrm{RRM}} = \frac{0.20 \times 1.50}{1.0} = 0.300 < 1$) to $\theta\_{\mathrm{PS}} = 1.8750$ ($\mathrm{PR} = -0.8125$), whereas Performative Gradient Descent (`PerfGD = 2.0312`) reaches the Stackelberg leader optimum $\theta\_{\mathrm{PO}} = 2.0312$ with strictly lower Performative Risk ($\mathrm{PR} = -0.8203$). When the contraction condition is violated, RRM oscillates endlessly (`[1.0, 0.0, 1.0, 0.0, 1.0]`).

Use the interactive closed-loop control and network laboratory below to explore the strategic synthesis's Axes 4 and 6 (`ComplexNetworkHubContagionAudit`, `RobustKalmanClosedLoopControlAudit` in `strategic.rs`; their placement beside this chapter is an authorial reading frame): a 12-node hub-and-spoke delay cascade with and without buffer slack at the hub, Centola complex contagion on a clustered ring, a Huber-clipped Kalman filter versus a Gaussian $L\_2$ Kalman filter under simulated sensor glitches, and Tube-MPC versus certainty-equivalent MPC:

<div class="stoch-lab" data-lab="kalman-control-hub-contagion" data-chapter="19-world-decisions-performativity.html" data-title="Chapter 19 Lab: Huber-Kalman, Tube-MPC &amp; Hub-Buffer Cascades" data-cli="cargo run --release --bin labs -- --layers"></div>

## Exercises

1. **(Laboratory — Performativity Gate)** Run `cargo run --release --bin gates -- --defect performativity` and verify that the gate exits with code `1` when $\frac{\varepsilon\beta}{\gamma} \ge 1$.

<details>
<summary><strong>Analytical &amp; Computational Solutions (Click to Expand)</strong></summary>

1. **Performativity Gate:** Running `cargo run --release --bin gates -- --defect performativity` prints `NEGATIVE PROOF [performativity] BLOCKED SUCCESSFULLY: RRM radius 1.20 >= 1 diverges/oscillates` and exits with code `1`.

</details>

## Summary

- **Bridge $L\_3 \to L\_1$** is governed by **Performative Prediction**, separating passive fit ($\theta\_{\mathrm{SL}}$), myopic retraining stability ($\theta\_{\mathrm{PS}}$), and Stackelberg optimality ($\theta\_{\mathrm{PO}}$), subject to the strict RRM contraction barrier $\frac{\varepsilon\beta}{\gamma} < 1$.
- **Prohibition 3** bans equating passive predictive accuracy with causal or performative validity, requiring explicit separation of $\theta\_{\mathrm{PS}}$ vs. $\theta\_{\mathrm{PO}}$ and verification of $r\_{\mathrm{RRM}} = \frac{\varepsilon\beta}{\gamma} < 1$.
