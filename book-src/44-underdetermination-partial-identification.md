# Chapter 44 — Underdetermination, Partial Identification, and Severe Testing

> Laboratory 3 of the `stochology` CLI: `--chapter 3`.

Even after an inquiry is honestly framed (Chapter 43), the analyst confronts the second constitutive tension of **Mediating Filter I · Axis 1 ($S_{\mathrm{epi}}$)** (**Tension 2**, implemented in [`crates/stochology/src/epistemic.rs`](crates/stochology/src/epistemic.rs)): **underdetermination of theory by finite, incomplete data** (Duhem, 1906; Quine, 1951; Stanford, 2006) versus **severe sequential testing and partial identification** (Ville, 1939; Popper, 1959; Lakatos, 1970; Peterson, 1976; Manski, 1990, 2003; Imbens & Manski, 2004; Lee, 2009; Mayo, 2018; Ramdas et al., 2023).

Why did `NaiveOptimizer` fail so badly on `CommunityReserveEnv` in Chapter 42? Because when $40\%$ to $60\%$ of the vulnerable neighborhood observations were missing from the archive, `NaiveOptimizer` assumed *missing completely at random* (`MCAR`) and reported a single point estimate $\bar{O}_t$. Charles Manski called this **the Law of Decreasing Credibility**: the credibility of inference decreases with the strength of the assumptions maintained.

---

## 3.1 Anytime-Valid Betting Martingales ($e$-Processes) and Lakatos's Shift Ratio

Fixed-horizon $p$-values become invalid whenever an analyst peeks at accumulating data and stops early ("optional stopping"). Following Jean Ville (1939), Glenn Shafer (2021), and Aaditya Ramdas et al. (2023), a sequential **evidence process** (*e-process*, [`EpistemicMarginState::from_e_process`](crates/stochology/src/epistemic.rs#L798-L838)) is a non-negative test martingale $(E_t)_{t \ge 0}$ with $E_0 = 1$ and $\mathbb{E}_{H_0}[E_t \mid \mathcal{F}_{t-1}] \le E_{t-1}$:

$$E_t = \prod_{i=1}^t e_i$$

By **Ville's Inequality** (1939), the probability that $E_t$ *ever* crosses $1/\alpha$ at any stopping time $\tau$ under $H_0$ is bounded by $\alpha$:

$$\mathbb{P}_{H_0}\!\left(\sup_{t \ge 1} E_t \ge \frac{1}{\alpha}\right) \le \alpha$$

In [`epistemic.rs`](crates/stochology/src/epistemic.rs), [`anytime_valid_radius`](crates/stochology/src/epistemic.rs#L107-L115) pairs the e-process with a time-uniform confidence sequence radius, and [`lakatos_shift_ratio`](crates/stochology/src/epistemic.rs#L120-L122) penalizes the corroborated evidence by $1 + K_{\mathrm{ad\text{-}hoc}}$, distinguishing **progressive** research programs from **degenerating** ones (Lakatos, 1970).

---

## 3.2 Non-Parametric Partial Identification: Manski, Imbens–Manski, Lee, and Peterson

Instead of inventing unverifiable missingness assumptions to force a point number, **Canonical Operator I** ([`EpistemicMarginState`](crates/stochology/src/epistemic.rs#L145-L198)) computes **sharp non-parametric bounds**:

1. **Algebraic Derivation of Manski (1990, 2003) Two-Arm ATE Bounds Under Attrition ([`EpistemicMarginState::from_single_arm_bounds`](crates/stochology/src/epistemic.rs#L402-L460) and [`EpistemicMarginState::from_ate_bounds`](crates/stochology/src/epistemic.rs#L536-L598)):**
   Let $Y(1), Y(0) \in [y_{\min}, y_{\max}]$ denote potential outcomes under treatment and control. In an imperfect trial or observational cohort, reporting is recorded by binary indicator $R_d \in \{0, 1\}$ for subjects assigned to arm $d \in \{0, 1\}$. By the Law of Total Expectation, the marginal mean for arm $d$ decomposes into observed and unobserved strata:
   $$\mathbb{E}[Y(d)] = \mathbb{E}[Y(d) \mid R_d = 1]\,\mathbb{P}(R_d = 1) + \mathbb{E}[Y(d) \mid R_d = 0]\,\mathbb{P}(R_d = 0)$$
   Because the unobserved counterfactual $\mathbb{E}[Y(d) \mid R_d = 0]$ is unknown, Charles Manski's sharp worst-case single-arm bounds insert the physical domain extrema $[y_{\min}, y_{\max}]$:
   $$L_{\mathrm{M}}(d) = p_{\mathrm{obs}, d}\,\bar{Y}_{\mathrm{obs}, d} + (1 - p_{\mathrm{obs}, d})\,y_{\min}, \qquad U_{\mathrm{M}}(d) = p_{\mathrm{obs}, d}\,\bar{Y}_{\mathrm{obs}, d} + (1 - p_{\mathrm{obs}, d})\,y_{\max}$$
   where $p_{\mathrm{obs}, d} = \mathbb{P}(R_d = 1)$ and $\bar{Y}_{\mathrm{obs}, d} = \mathbb{E}[Y(d) \mid R_d = 1]$. For the causal **Average Treatment Effect** $\mathrm{ATE} = \mathbb{E}[Y(1)] - \mathbb{E}[Y(0)]$, subtracting the lower bound of arm $0$ from the upper bound of arm $1$ (and vice versa) yields the sharp worst-case two-arm bounds:
   $$\Delta_L = \mathbb{E}[Y(1) \mid R_1 = 1] p_{\mathrm{obs}, 1} + y_{\min}(1 - p_{\mathrm{obs}, 1}) - \bigl[\mathbb{E}[Y(0) \mid R_0 = 1] p_{\mathrm{obs}, 0} + y_{\max}(1 - p_{\mathrm{obs}, 0})\bigr]$$
   $$\Delta_U = \mathbb{E}[Y(1) \mid R_1 = 1] p_{\mathrm{obs}, 1} + y_{\max}(1 - p_{\mathrm{obs}, 1}) - \bigl[\mathbb{E}[Y(0) \mid R_0 = 1] p_{\mathrm{obs}, 0} + y_{\min}(1 - p_{\mathrm{obs}, 0})\bigr]$$
   The resulting **identification interval width** is:
   $$\Delta_{\mathrm{id}} = \Delta_U - \Delta_L = (y_{\max} - y_{\min})\bigl[(1 - p_{\mathrm{obs}, 1}) + (1 - p_{\mathrm{obs}, 0})\bigr]$$
   This width represents **structural underdetermination**: it does *not* shrink to zero as sample size $n \to \infty$. Collecting millions of records under the same non-random missingness design leaves $\Delta_{\mathrm{id}}$ completely intact. Sampling fluctuations around $[L_{\mathrm{M}}, U_{\mathrm{M}}]$ are captured by the Imbens–Manski (2004) parameter-coverage confidence interval `imbens_manski_bounds`.

2. **David S. Lee's (2009) Sharp Trimming Bounds Under Differential Attrition ([`EpistemicMarginState::from_lee_attrition_bounds`](crates/stochology/src/epistemic.rs#L658-L711)):**
   In randomized trials where retention rates differ ($p_{\mathrm{obs}, 1} \ne p_{\mathrm{obs}, 0}$), differential attrition reintroduces selection bias. Lee (2009) established that if treatment assignment affects attrition monotonically ($R_i(1) \ge R_i(0)$ for all $i$), the excess retained fraction in the higher-retention arm (assuming $p_{\mathrm{obs}, 1} > p_{\mathrm{obs}, 0}$):
   $$q = \frac{p_{\mathrm{obs}, 1} - p_{\mathrm{obs}, 0}}{p_{\mathrm{obs}, 1}}$$
   can be trimmed from the upper and lower tails of the observed treatment distribution $Y_{\mathrm{obs}, 1}$. Trimming the top $q$-quantile yields the lower bound $L_{\mathrm{Lee}}$, while trimming the bottom $q$-quantile yields $U_{\mathrm{Lee}}$. This yields sharp bounds for the average treatment effect on the "always-reporting" subpopulation without making parametric missing-at-random assumptions, strictly narrowing $[L_{\mathrm{Lee}}, U_{\mathrm{Lee}}] \subset [\Delta_L, \Delta_U]$.

3. **Arthur V. Peterson Jr.'s (1976) Competing-Risks Fréchet Bounds and Censoring Bounds ([`EpistemicMarginState::from_peterson_competing_risks_bounds`](crates/stochology/src/epistemic.rs) and [`EpistemicMarginState::from_censoring_bounds`](crates/stochology/src/epistemic.rs)):**
   When two competing risks have marginal cause-specific survival curves $S_1(t) = \mathbb{P}(T_1 > t)$ and $S_2(t) = \mathbb{P}(T_2 > t)$, analysts routinely assume independence ($S_{\mathrm{indep}}(t) = S_1(t) S_2(t)$). Peterson (1976) proved that under arbitrary dependence, the joint survival function $S(t) = \mathbb{P}(T_1 > t, T_2 > t)$ is bounded by the classical Fréchet–Hoeffding copula inequalities:
   $$\max\bigl(0,\; S_1(t) + S_2(t) - 1\bigr) \le S(t) \le \min\bigl(S_1(t),\, S_2(t)\bigr)$$
   Analogously, for a finite cohort of $n$ subjects with $d$ observed failures and $c$ censored subjects (`from_censoring_bounds`), worst-case non-parametric survival bounds without an independent-censoring assumption are:
   $$S_{\mathrm{lower}} = 1 - \frac{d + c}{n}, \qquad S_{\mathrm{upper}} = 1 - \frac{d}{n}$$
   Any survival estimate claiming higher precision (such as naive Kaplan–Meier) rests on an unverifiable independence assumption.

4. **The Mayo–Manski Severe Boundary Bridge ([`EpistemicMarginState::passes_severe_boundary`](crates/stochology/src/epistemic.rs#L614-L616)):**
   A substantive policy claim $\mu > \mu_0$ passes a severe error-statistical test under partial identification if and only if the **lower Imbens–Manski bound** $L_{\mathrm{IM}}$ strictly exceeds $\mu_0$, ensuring that neither sampling error nor structural underdetermination can account for the observed effect.

---

## 3.3 Equipping the `StochologicalAgent` with the Epistemic Margin

Look back at [`StochologicalAgent::act`](crates/stochology/src/engine.rs#L291-L368) in `engine.rs`: when `use_epistemic_margin = true`, the agent calls [`EpistemicMarginState::for_shift_observation`](crates/stochology/src/epistemic.rs#L1153-L1240) over the observed clusters and `obs.missing_count`. Because $[L_{\mathrm{IM}}, U_{\mathrm{IM}}]$ honestly spans the missing vulnerable households, the agent's epistemic coverage jumps from **$25.0\%$** (`0.0%` on tail shocks, under the naive i.i.d. 95% confidence interval) to **$95.8\%$** (`75.0%` on tail shocks, where only the radical out-of-support surge at shift $t = 22$ exceeds $Y_{\max} = 52.0$, `[Simulated]`).

---

## 3.4 Laboratory 3: Real Data on the LaLonde Benchmark

Running laboratory 3 (`cargo run --release --bin stochology -- --chapter 3`) tests these partial-identification tools on Robert LaLonde's (1986) classic benchmark (`data/nsw_experimental.csv`, $n = 445$, and `data/cps_observational.csv`, $n = 15{,}992$, `[Measured]`):

```text
=== CHAPTER 3: FILTER I (S_EPI) — UNDERDETERMINATION & PARTIAL IDENTIFICATION ===
  LaLonde Benchmark [Measured] (data/nsw_experimental.csv vs data/cps_observational.csv):
    Experimental RCT ATE [Measured, zero survey attrition] = $1794 (n_t=185, n_c=260) vs Naive CPS Diff = $-8498 (n_cps=15992)
    Lee (2009) Positive-Wage Selection Bounds [Measured, re78>0: 140/185 vs 168/260]: [$-1142, $2621] (width=$3763)
    Manski Worst-Case Two-Arm ATE Bounds [Assumed: 15/200 & 30/290 attrition]: [$-487, $3439] (width=$3926, Imbens-Manski 95% CI=[$-1639, $4568])
    Lee Monotone Attrition Two-Arm ATE Bounds [Assumed: 15/200 & 30/290 attrition]: [$872, $1996] (width=$1123)
    Censoring worst-case survival bounds [Assumed]: [0.750, 0.860] (KM naive=0.843)
    Peterson (1976) competing-risk Frechet bounds [Assumed]: [0.750, 0.860] (indep=0.765, width=0.110)
    Ville e-Process [Assumed]: terminal E_T = 2164.62 (33.4 decibans), anytime CI = [-0.03, 2.75]
```

- **Experimental vs. Observational Underdetermination (`[Measured]` on `nsw_experimental.csv` & `cps_observational.csv`):** In the randomized National Supported Work (NSW) experiment (Dehejia–Wahba complete-case extract, with zero survey attrition in the file), the true Average Treatment Effect on 1978 earnings (`re78`) is **`+$1,794`** (`$6,349` treated vs. `$4,555` control, $n_t = 185, n_c = 260$, `[Measured]`). Yet if an analyst naively compares the NSW treated group against the non-experimental Current Population Survey (`cps_observational.csv`, mean `$14,847`, $n = 15{,}992$, `[Measured]`), unmeasured selection bias flips the sign to **`-$8,498`**—an error exceeding `$10,290`!
- **Lee (2009) Positive-Wage Employment Selection Bounds (`[Measured]` on `nsw_experimental.csv`):** Because `data/nsw_experimental.csv` has zero survey missingness (`re78` is recorded for all $185$ treated and $260$ control units), the genuine empirical selection problem inside the file is **employment selection into positive wages (`re78 > 0`)**—the exact canonical application of Lee (2009, *Review of Economic Studies*): $140/185$ ($75.7\%$) of treated participants vs. $168/260$ ($64.6\%$) of controls have `re78 > 0` (`[Measured]`). Trimming the excess $(75.7\% - 64.6\%)/75.7\% = 14.6\%$ of positive-earning treated units yields the purely empirical Lee bounds **`[-$1,142, +$2,621]`** (width `$3,763`, `[Measured]`) for the intensive-margin wage effect among always-employed workers.
- **Semi-Synthetic Survey-Attrition Stress Test (`[Assumed]` $15/200$ treated & $30/290$ control missingness on `nsw_experimental.csv`):** To illustrate how two-arm Manski and Lee bounds behave under hypothetical survey non-response (`missing_treated = 15`, `missing_control = 30`, bounded support `[$0, $22,000]`, `[Assumed]`), `EpistemicMarginState::from_ate_bounds` yields the worst-case ATE identification region **`[-$487, +$3,439]`** (width `$3,926`, Imbens–Manski 95% CI **`[-$1,639, +$4,568]`**, `[Assumed]`), while `from_lee_attrition_bounds` yields **`[+$872, +$1,996]`** (width `$1,123`, `[Assumed]`). Both intervals strictly contain the zero-attrition experimental RCT benchmark (`+$1,794`), whereas the naive CPS observational estimate (`-$8,498`) falls outside both.

---

## Exercises

1. **Identification vs. Estimation Uncertainty:** In [`EpistemicMarginState::from_ate_bounds`](crates/stochology/src/epistemic.rs#L536-L598), double both `treated_obs.len()` and `missing_treated` while keeping the sample moments constant. What happens to `identification_width`, and what happens to the Imbens–Manski sampling margin?
2. **Peterson Competing Risks & Worst-Case Censoring:** Suppose a clinical trial of $n = 200$ patients records $d = 28$ deaths and $c = 22$ patients lost to follow-up (`EpistemicMarginState::from_censoring_bounds(28, 22, 150)` and `EpistemicMarginState::from_peterson_competing_risks_bounds(0.86, 0.89)`). Why are the naive independent-censoring ($0.843$) and independent competing-risk ($0.765$) survival estimates dangerously over-optimistic if patients leave the study precisely when their condition deteriorates?

