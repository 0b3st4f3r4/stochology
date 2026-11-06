# Chapter 47 — Irreversible Time, Non-Ergodicity, and the Vital Margin

> Laboratory 6 of the `stochology` CLI: `--chapter 6`.

Suppose an agent has solved every epistemic problem of **Filter I** (Chapters 43 through 45) and recognized convex tail asymmetry (Chapter 46): its partial-identification interval $[L_t, U_t]$ covers the true demand on $100\%$ of shifts. Can the agent still suffer catastrophic ruin?

**Yes.** That is **Collapse Proposition 2** (*impotent lucidity before ruin*): knowing the exact width of uncertainty while operating at $96\%$ utilization or excess leverage with zero physical slack ($\mathrm{TTS}_{\mathrm{survival}} < \mathrm{TTR}_{\mathrm{recovery}}$).

Completing **Mediating Filter II · Axis 2 ($S_{\mathrm{vital}}$: Vital Margin)** (implemented in [`crates/stochology/src/vital.rs`](crates/stochology/src/vital.rs)), this chapter confronts **static ensemble expectations ($\mathbb{E}[X_T]$) and "lean" zero-buffer efficiency** against **Canonical Operator II**: **irreversible entropic time, non-ergodic trajectory growth, Canguilhem–Holling resilience basins, Kingman heavy-traffic queues, the survival horizon inequality $\mathrm{TTS}_{\mathrm{survival}} \ge \mathrm{TTR}_{\mathrm{recovery}}$, Abraham Wald's survivorship reconstruction, and the Info-Gap / Sniedovich Maximin safeguard** (Canguilhem, 1943/1966; Wald, 1943, 1945; Itô, 1951; Kelly, 1956; Scarf, 1960; Breiman, 1961; Kingman, 1961; Merton, 1969; Georgescu-Roegen, 1971; Samuelson, 1971, 1979; Holling, 1973; McEwen, 1998; Ben-Haim, 2006; Sniedovich, 2007, 2012; Scheffer et al., 2009; Baker & McHale, 2013; Simchi-Levi et al., 2014; Peters, 2019).

![Info-Gap Horizons and Sniedovich's Global Wald Maximin Safeguard](figures/fig-02-infogap-sniedovich.svg)

---

## 6.1 Non-Ergodic Itô Variance Drain and Baker–McHale Epistemic Kelly Shrinkage

When a reserve or capital stock $W_t$ evolves multiplicatively over irreversible time ($dW_t = W_t(\mu\,dt + \sigma\,dB_t)$), decision theorists and neoclassical economists routinely optimize the **ensemble average** across hypothetical parallel universes:
$$\mathbb{E}[W_T] = W_0 \exp(\mu T) \implies g_{\mathrm{ensemble}} = \lim_{T \to \infty} \frac{1}{T}\ln\frac{\mathbb{E}[W_T]}{W_0} = \mu$$
In physical, ecological, and human institutions, however, an agent cannot live in parallel worlds; the agent inhabits a **single, irreversible trajectory** through time.

### 6.1.1 Proof of the Time-Average vs. Ensemble-Average Divergence

By Kiyosi Itô's (1951) Lemma, the logarithm of wealth $\ln W_t$ follows:
$$d\ln W_t = \frac{1}{W_t} dW_t - \frac{1}{2 W_t^2}(dW_t)^2 = \left(\mu - \frac{1}{2}\sigma^2\right)dt + \sigma\,dB_t$$
Integrating over horizon $T$:
$$\ln\frac{W_T}{W_0} = \left(\mu - \frac{1}{2}\sigma^2\right)T + \sigma B_T$$
Dividing by $T$ and taking the long-run limit as $T \to \infty$, the Law of the Iterated Logarithm guarantees that Brownian fluctuations vanish ($\lim_{T\to\infty} \frac{\sigma B_T}{T} = 0$ almost surely). The true trajectory time-average growth rate is therefore ([`VitalMarginState::from_non_ergodic_returns`](crates/stochology/src/vital.rs#L778-L843)):
$$g_{\mathrm{time}}(f) = \lim_{T \to \infty} \frac{1}{T}\ln\frac{W_T}{W_0} = f\mu - \frac{1}{2}f^2\sigma^2 \quad \text{almost surely}$$

The difference $g_{\mathrm{ensemble}} - g_{\mathrm{time}}(1) = \frac{1}{2}\sigma^2$ is the **Itô variance drain**.
This divergence has profound consequences for risk management:
- Differentiating with respect to leverage $f$ yields John Kelly's (1956) optimal growth leverage $f^* = \mu / \sigma^2$, achieving maximum growth $g^*(f^*) = \frac{\mu^2}{2\sigma^2}$.
- If an optimizer seeks to maximize the ensemble average $\mathbb{E}[W_T]$, it chooses unbounded leverage $f \to \infty$.
- Yet for any leverage exceeding the ruin threshold:
  $$f > \frac{2\mu}{\sigma^2}$$
  the time-average growth rate turns negative: $g_{\mathrm{time}}(f) < 0$. Under such leverage, **the single trajectory collapses to zero with probability 1** ($\lim_{T \to \infty} W_T = 0$ a.s.), even while the mathematical expectation across non-existent parallel universes explodes to infinity ($\mathbb{E}[W_T] \to \infty$)! Optimizing the ensemble average guarantees individual ruin.

### 6.1.2 Baker–McHale Epistemic Shrinkage Under Finite Estimation Sample Size

Moreover, as Paul Samuelson (1971, 1979) warned, the true drift $\mu$ is never known exactly: it is estimated from an effective sample size $n_{\mathrm{eff}}$ (Chapter 45) with estimation variance $\sigma_{\hat{\mu}}^2 = \sigma^2 / n_{\mathrm{eff}}$. Plugging $\hat{\mu}$ directly into John Kelly's (1956) formula $f^*_{\mathrm{plug\text{-}in}} = \hat{\mu}/\sigma^2$ systematically overbets on estimation noise. Rose D. Baker and Ian G. McHale (2013) derived the exact **epistemic Kelly shrinkage** factor implemented in [`VitalMarginState::from_kelly_parameters`](crates/stochology/src/vital.rs#L843-L850) and [`VitalMarginState::from_non_ergodic_returns`](crates/stochology/src/vital.rs#L778-L843):
$$k(n_{\mathrm{eff}}) = \frac{1}{1 + 1 / n_{\mathrm{eff}}} \in (0, 1], \qquad k_{\mathrm{BM}} = \frac{z_{\hat{\mu}}^2}{1 + z_{\hat{\mu}}^2 + \hat{f}^2 (1 + \tfrac{1}{2}\hat{\gamma}_2) / n_{\mathrm{eff}}^{\mathrm{vol}}}$$
where $z_{\hat{\mu}}^2 = n_{\mathrm{eff}}^{\mathrm{vol}} \hat{\mu}^2 / \hat{\sigma}^2$ is the drift signal-to-noise ratio under annual-block volatility-regime clustering ($n_{\mathrm{eff}}^{\mathrm{vol}} = T / \mathrm{DEFF}_{\mathrm{vol}}$) and $\hat{\gamma}_2$ is excess kurtosis. Even when raw $T$ is tens of thousands of days, daily drift has a modest $t$-statistic $z_{\hat{\mu}} \approx 1$, so $k_{\mathrm{BM}}$ shrinks reckless plug-in leverage before the heavy-tail regime selector (`TailRobustWdroCvarKelly`) takes over!

---

## 6.2 Canguilhem's Vital Normativity, Holling's Resilience Basin, and McEwen's Allostatic Wear

In *The Normal and the Pathological* (1943/1966), Georges Canguilhem defined health not as conformity to a statistical average, but as a **margin of tolerance** for the inconstancies of the environment—the capacity to absorb shocks and institute new norms. C. S. Holling (1973) formalized this in ecology by separating:
- **Engineering resilience** ($\lambda_{\mathrm{recovery}} = -\frac{\partial f}{\partial x}\big|_{x^*}$): local recovery speed near equilibrium;
- **Ecological / vital resilience** ($D_t = x_{\mathrm{high}}^* - x_{\mathrm{tip}}^*$): the width of the stability basin before a fold bifurcation tips the system into irreversible collapse.

In [`VitalMarginState::solve_canguilhem_holling_basin`](crates/stochology/src/vital.rs#L926-L992), as chronic stress or Bruce McEwen's (1998) **allostatic load** accumulates, the vital basin ratio $D_t / D_0$ shrinks and Marten Scheffer et al.'s (2009) **Critical Slowing Down** lag-1 autocorrelation $\rho_1 = \exp(-\lambda_{\mathrm{recovery}}\Delta t)$ approaches $1.0$, warning of an approaching tipping point long before the mean state collapses.

---

## 6.3 Kingman's Heavy-Traffic Queue Explosion and the Lean Operating Trap

Why does "lean $97\%$ utilization" destroy hospital emergency rooms, electrical grids, and supply chains?
In queueing physics, let $\tau = 1/\mu_s$ be the mean service time, $\rho = \lambda / \mu_s$ the server utilization, and $c_a^2, c_s^2$ the squared coefficients of variation of inter-arrival and service times. By John F. C. Kingman's (1961) heavy-traffic approximation ([`VitalMarginState::from_queue_basin_and_survival`](crates/stochology/src/vital.rs#L998-L1018)):
$$\mathbb{E}[W_q] \approx \left(\frac{c_a^2 + c_s^2}{2}\right)\,\frac{\rho}{1 - \rho}\,\tau$$

### 6.3.1 The Hyperbolic Utilization Explosion

The factor $\frac{\rho}{1 - \rho}$ is a hyperbola with a vertical asymptote at $\rho = 1$:
- At $\rho = 0.80$ utilization: $\frac{\rho}{1-\rho} = \frac{0.80}{0.20} = 4.0\times$.
- At $\rho = 0.95$ utilization: $\frac{\rho}{1-\rho} = \frac{0.95}{0.05} = 19.0\times$.
Moving utilization from $80\%$ to $95\%$ increases average waiting times and backlog inventory by:
$$\frac{19.0}{4.0} = 4.75\times \quad (+375\%)$$
- At $\rho = 0.969$ (as in the stressed Victoria electrical grid below): $\frac{\rho}{1-\rho} = 31.2\times$, multiplying delays by **$7.8\times$** compared to $\rho = 0.80$!

Under lean optimization, management eliminates "idle headroom" to minimize unit costs, pushing $\rho \to 1$. Because service delay scales with $\mathbb{E}[W_q]$, system recovery time $\mathrm{TTR}_{\mathrm{recovery}}$ explodes hyperbolically.

### 6.3.2 The Master Survival Inequality: $\mathrm{TTS}_{\mathrm{survival}} \ge \mathrm{TTR}_{\mathrm{recovery}}$

Following Herbert Scarf's (1960) $(s, S)$ inventory optimality and David Simchi-Levi et al.'s (2014) supply-chain physics, an institution survives a disruption if and only if its **Time-to-Survival Buffer** ($\mathrm{TTS}_{\mathrm{survival}} = B_t / \dot{D}_{\mathrm{crisis}}$, `survival_slack_tts`) is at least as large as its **Time-to-Recovery / Steering Delay** ($\mathrm{TTR}_{\mathrm{recovery}}$, `recovery_delay_ttr`):
$$\mathrm{TTS}_{\mathrm{survival}} \ge \mathrm{TTR}_{\mathrm{recovery}}$$

When a disruption strikes a system operating at $\rho = 0.95$, its bloated recovery time $\mathrm{TTR}_{\mathrm{recovery}}$ outstrips its lean buffer slack ($\mathrm{TTS}_{\mathrm{survival}} < \mathrm{TTR}_{\mathrm{recovery}}$), driving the vital reserve into the absorbing ruin barrier before repairs can be completed.

---

## 6.4 Abraham Wald's (1943) Latent Survivorship Matrix Solver and Sniedovich's Maximin Safeguard

Finally, [`VitalMarginState::from_queue_basin_and_survival`](crates/stochology/src/vital.rs#L998-L1018) and [`WaldSurvivorshipAudit`](crates/stochology/src/vital.rs#L191-L206) implement two classic defenses against Knightian blindness:

### 6.4.1 Abraham Wald's Latent Survivorship Linear Inversion

In 1943, Abraham Wald was tasked by the Statistical Research Group (SRG) with deciding where to add armor to Allied combat aircraft. The military recorded the bullet holes on planes returning from combat missions. An intuitive point optimizer would reinforce the areas with the most bullet holes.

Wald realized that the sample was truncated: downed planes never returned to base.
Let $c \in \{1, 2\}$ represent two distinct mission cohorts. Let $S_{c, A}$ and $S_{c, B}$ be the observed returning survivor hit counts in zone $A$ (wings/fuselage) and zone $B$ (engine/cockpit). Let $M_c$ be the total number of missing (downed) aircraft in cohort $c$.

Let $\boldsymbol{\theta} = (\theta_A, \theta_B)^T$ be the unknown vulnerability odds multipliers on missing aircraft, where $\theta_z = q_z / (1 - q_z)$ represents the ratio of conditional lethality $q_z = \mathbb{P}(\text{Lost}\mid\text{Hit}_z)$ to survival probability $1 - q_z$. Under latent proportionality, the missing aircraft satisfy the linear system:
$$\begin{pmatrix} S_{1, A} & S_{1, B} \\ S_{2, A} & S_{2, B} \end{pmatrix} \begin{pmatrix} \theta_A \\ \theta_B \end{pmatrix} = \begin{pmatrix} M_1 \\ M_2 \end{pmatrix}$$
Provided the determinant $\Delta = S_{1, A}S_{2, B} - S_{1, B}S_{2, A} \ne 0$, matrix inversion recovers $(\theta_A, \theta_B)$ exactly:
$$\begin{pmatrix} \theta_A \\ \theta_B \end{pmatrix} = \frac{1}{S_{1, A}S_{2, B} - S_{1, B}S_{2, A}} \begin{pmatrix} S_{2, B} & -S_{1, B} \\ -S_{2, A} & S_{1, A} \end{pmatrix} \begin{pmatrix} M_1 \\ M_2 \end{pmatrix}$$
In [`WaldSurvivorshipAudit::solve_latent_two_cohort`](crates/stochology/src/vital.rs), solving this system yields odds multipliers $\theta_A = 0.0776$ and $\theta_B = 5.250$ (where $\theta_B > 1$ represents an odds ratio rather than an impossible probability $p_B > 1$), which map via $q_z = \theta_z / (1 + \theta_z)$ directly to conditional lethalities of $q_A = \theta_A / (1 + \theta_A) = 7.2\%$ for wings versus $q_B = \theta_B / (1 + \theta_B) = 84.0\%$ for engines (`[Assumed]`), and reconstructs the latent downed-aircraft hits $\hat{M}_A = S_A \theta_A = 18$ and $\hat{M}_B = S_B \theta_B = 210$. Wald's vital advice was to armor the engines—the exact zone where returning planes had zero bullet holes!

Complementing this, `evaluate_empirical_truncation_bias` measures empirical survivorship-conditioning bias directly on `data/nsw_experimental.csv` (`re78 > 0` only: full RCT ATE `$1,794` vs. survivor-conditioned `$1,341`, bias `-$453`, `[Measured]`).

### 6.4.2 Info-Gap Robustness Guarded by Moshe Sniedovich's Global Wald Maximin

Local Info-Gap robustness $\hat{\alpha}(q, r_c)$ around a nominal guess $\tilde{u}$ suffers from Moshe Sniedovich's (2007, 2012) *No-Man's-Land* invariance property: an option can look locally robust around $\tilde{u}$ while exposing the community to catastrophic global ruin outside $\hat{\alpha}$. Pairing local robustness with Wald's global Maximin floor prevents local robustness mirages (`sniedovich_no_mans_land_mirage`).

---

## 6.5 Laboratory 6: Real Data on 138 Years of DJIA Returns and Victoria Grid Demand

Running laboratory 6 (`cargo run --release --bin stochology -- --chapter 6`) evaluates [`vital.rs`](crates/stochology/src/vital.rs) on `data/djia_1885_2023.csv` ($37{,}931$ daily closing prices, 1885-02-16 to 2023-11-21, yielding $T = 37{,}930$ daily simple-return pairs), `data/vic_elec.csv` ($n = 52{,}608$ half-hourly electricity demand readings), and `data/nsw_experimental.csv` (`[Measured]`):

```text
=== CHAPTER 6: FILTER II (S_VITAL) — IRREVERSIBLE TIME, NON-ERGODICITY & VITAL MARGIN ===
  Canonical Operator II (Convexity, Time-Average Growth & Survival Inequality TTS_survival >= TTR_recovery):
  1. DJIA Non-Ergodic Trajectory [Measured] (data/djia_1885_2023.csv, T = 37930 trading days, emp_alpha_loss = 2.83):
     Arithmetic Ensemble Drift = +0.000243/day vs Time-Average Growth = +0.000186/day (Itô drain = 23.6%)
     Plug-in Kelly = 2.12 vs Baker-McHale Epistemic Kelly = 1.99 (Shrinkage = 0.9416)
     Heavy-Tail Kelly Regime Selector [Assumed] (alpha=1.65 <= 2.2): regime = TailRobustWdroCvarKelly, naive_quadratic_f = 1.0000 vs WDRO_CVaR95_f = 0.0314
  2. Victoria Electricity Grid [Measured] (data/vic_elec.csv, n = 52608 half-hours, q50=4635 MW, q95=6106 MW, q99=6981 MW, max=9345 MW, ramp_p99=897 MW/h, scale=1.18/1.05 [Assumed]):
     Stressed [Measured quantiles + Assumed ODE h=1.80,K=10] (rho=96.9%): Kingman Wait = 31.2x (G/G/1 E[W_q]=16.17h, var_factor=0.52, rho1_worn=0.732), Basin D_t/D_0 = 0.46, TTS_survival = 2.6h < TTR_recovery = 8.0h (survives=false)
     Buffered [Measured quantiles + Assumed ODE h=1.80,K=10] (rho=47.2%): Kingman Wait = 0.9x (G/G/1 E[W_q]=0.46h, var_factor=0.52, rho1_worn=0.593), Basin D_t/D_0 = 0.92, TTS_survival = 16.5h >= TTR_recovery = 4.8h (survives=true)
  3. Survivorship & Robustness: Wald Latent 2x2 Solver [Assumed] = true (reconstructed M_A=18, M_B=210 -> Zone B 84.0% > Zone A 7.2%) | Empirical NSW Survivor Bias [Measured] (re78>0 only): full=$1794 vs survivor=$1341 (bias=$-453) | Sniedovich Mirage [Assumed] = true
```

- **Itô Variance Drain & Separated Drift vs. Volatility-Regime Baker–McHale Shrinkage on 138 Years of DJIA (`djia_1885_2023.csv`, $37{,}931$ closing prices, $T = 37{,}930$ return pairs, `[Measured]`):** The daily arithmetic ensemble drift is $\hat{\mu} = 0.000243$, but volatility drains the single-trajectory time-average growth rate by **$23.6\%$** to $g_{\mathrm{time}}(1) = 0.000186$ (`[Measured]`). Furthermore, separating the return-drift block design effect ($\mathrm{DEFF}_{\mu}$ on $r_t$) from the squared-return volatility-regime design effect ($\mathrm{DEFF}_{\mathrm{vol}}$ on $r_t^2$) in Baker & McHale's (2013) fourth-moment shrinkage formula shrinks the plug-in Kelly leverage $\hat{f}_{\mathrm{plug\text{-}in}} = 2.12$ by **$k_{\mathrm{BM}} = 0.9416$** to $\hat{f}_{\mathrm{BM}} = 1.99$ (`[Measured]`), while the empirical left-tail Hill exponent on negative daily returns is $\hat{\alpha}_{\mathrm{loss}} = 2.83$ at $k_{\mathrm{loss}} = 500$ ($n_- / 20$ clamped to $[50, 500]$, `[Measured]`). Under an infinite-variance stress benchmark ($\alpha = 1.65 \le 2.2$, `[Assumed]`), the heavy-tail WDRO-$\mathrm{CVaR}_{95}$ selector brings leverage down to $0.0314$ (`[Assumed]`).
- **The two operating points, read off the block above:** the stressed regime at $\rho = 96.9\%$ is the
one where the Kingman factor reaches $31.2\times$ and $\mathrm{TTS}_{\mathrm{survival}} = 2.6\text{ h}$
falls below $\mathrm{TTR}_{\mathrm{recovery}} = 8.0\text{ h}$; the buffered regime at $\rho = 47.2\%$ is
the one where it does not. The grid is the same grid in both rows. Only the headroom differs, and with
it the verdict — the point of the whole chapter, stated once.

## 6.6 The recovery term is not one term: a deployment reading

§6.3 compared a survival time against a recovery time as if recovery were a single act. In any system that runs on a
fitted model, it is at least three, and the middle one is the one institutions omit:

$$\mathrm{TTR}_{\mathrm{recovery}} = \mathrm{TTR}_{\mathrm{detect}} + \mathrm{TTR}_{\mathrm{retrain}} +
\mathrm{TTR}_{\mathrm{roll out}}.$$

The decomposition matters because the three terms are governed by different mechanisms and only the first is
usually instrumented. `vital.rs::ModelDriftFallbackAudit` isolates it on a canonical fixture `[Simulated]`: a
monitored performance signal shifts at step 10, a one-sided CUSUM with slack 0.10 and threshold 0.30 fires at step
11, the reserve affords 26 steps of degraded service, and a retraining that takes 12 shifts completes recovery at
step 23 — inside the survival time, with 7.50 of the original 8.00 reserve left. An institution whose retraining
takes twice as long recovers at step 35 and fails the inequality, on the same drift, with the same monitor and the
same reserve.

Three readings follow, and each is a margin rather than an optimisation.

1. **Without a fallback, detection is worthless.** The no-fallback arm of the same fixture exhausts the reserve
   entirely: it never stops serving the degraded model, so its $\mathrm{TTR}_{\mathrm{recovery}}$ is infinite and the
   inequality fails by construction, not by degree. This is Collapse Proposition 2 in its cleanest form — knowing
   the width of the drop exactly, and having no way to stop consuming the reserve because of it.
2. **The monitor's operating point is a trade between two failure modes.** A lower threshold detects earlier and
   raises the false-alarm rate; a higher threshold does the reverse. The stationary arm of the fixture is what makes
   the trade auditable: with no drift in the world, the same monitor stays silent, so the alarm that fires is
   attributable to the shift rather than to the noise.
3. **The lag that matters is organisational, not algorithmic.** Retraining is cheap; releasing a retrained artifact
   through whatever gates exist is not, and the gates are the term that must be compared with the reserve. A reserve
   sized without reference to the deployment pipeline is a reserve sized against the wrong clock.

The same decomposition is the correct reading of the MLOps practice this chapter's neighbours describe: continuous
training is an attempt to shrink the second and third terms, drift monitoring is an attempt to shrink the first, and
a rollback path is the only mechanism that makes the sum finite when the first two are already spent.

---

## Exercises

1. **Collapse Proposition 2 in `CommunityReserveEnv`:** Run `StochologicalAgent::epistemic_only()` (`use_epistemic_margin = true, use_vital_margin = false, use_institutional_governance = true`, `[Simulated]`). Why does the agent achieve $95.8\%$ epistemic coverage ($75.0\%$ on tail shocks) yet still hit the ruin barrier at step $t = 10$?
2. **Baker–McHale Shrinkage Limit:** In [`VitalMarginState::from_kelly_parameters`](crates/stochology/src/vital.rs#L843-L850), verify that as $n_{\mathrm{eff}} \to 1$, $k(n_{\mathrm{eff}}) = 0.50$, and as $n_{\mathrm{eff}} \to \infty$, $k(n_{\mathrm{eff}}) \to 1$.

