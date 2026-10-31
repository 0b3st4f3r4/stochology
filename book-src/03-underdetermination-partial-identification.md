# Chapter 3 — Underdetermination, Partial Identification, and Severe Testing

Even after an inquiry is honestly framed (Chapter 2), the analyst confronts the second constitutive tension of **Mediating Filter I** (**Tension 2**, implemented in [`crates/stochology/src/identification.rs`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/identification.rs)): **underdetermination of theory by finite, incomplete data** (Duhem, 1906; Quine, 1951; Stanford, 2006) versus **severe sequential testing and partial identification** (Ville, 1939; Popper, 1959; Lakatos, 1970; Peterson, 1976; Manski, 1990, 2003; Imbens & Manski, 2004; Lee, 2009; Mayo, 2018; Ramdas et al., 2023).

Why did `NaiveOptimizer` fail so badly on `CommunityReserveEnv` in Chapter 1? Because when $40\%$ to $60\%$ of the vulnerable neighborhood observations were missing from the archive, `NaiveOptimizer` assumed *missing completely at random* (`MCAR`) and reported a single point estimate $\bar{O}_t$. Charles Manski called this **the Law of Decreasing Credibility**: the credibility of inference decreases with the strength of the assumptions maintained.

---

## 3.1 Anytime-Valid Betting Martingales ($e$-Processes) and Lakatos's Shift Ratio

Fixed-horizon $p$-values become invalid whenever an analyst peeks at accumulating data and stops early ("optional stopping"). Following Jean Ville (1939), Glenn Shafer (2021), and Aaditya Ramdas et al. (2023), a sequential **evidence process** (*e-process*, [`EvidenceProcess`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/identification.rs#L17-L87)) is a non-negative test martingale $(E_t)_{t \ge 0}$ with $E_0 = 1$ and $\mathbb{E}_{H_0}[E_t \mid \mathcal{F}_{t-1}] \le E_{t-1}$:

$$E_t = \prod_{i=1}^t L_i$$

By **Ville's Inequality** (1939), the probability that $E_t$ *ever* crosses $1/\alpha$ at any stopping time $\tau$ under $H_0$ is bounded by $\alpha$:

$$\mathbb{P}_{H_0}\!\left(\sup_{t \ge 1} E_t \ge \frac{1}{\alpha}\right) \le \alpha$$

In [`identification.rs`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/identification.rs), [`anytime_confidence_half_width`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/identification.rs#L91-L100) pairs the e-process with a time-uniform confidence sequence radius:

$$r_t(\alpha) = \hat{\sigma}\sqrt{\frac{2\bigl(\ln(1/\alpha) + \ln(1 + \ln\max(n, 1))\bigr)}{n}}$$

And to prevent Duhem–Quine rescue maneuvers—saving a refuted model by tacking on $m_{\mathrm{ad\text{-}hoc}}$ post-hoc auxiliary patches—[`lakatos_shift_ratio`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/identification.rs#L105-L108) penalizes the corroborated evidence by $1 + m_{\mathrm{ad\text{-}hoc}}$, distinguishing **progressive** research programs from **degenerating** ones (Lakatos, 1970).

---

## 3.2 Non-Parametric Partial Identification: Manski, Imbens–Manski, Lee, and Peterson

Instead of inventing unverifiable missingness assumptions to force a point number, Stochology computes **sharp non-parametric bounds**:

1. **Manski (1990, 2003) Worst-Case Bounds & Imbens–Manski (2004) Confidence Interval ([`ManskiBounds`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/identification.rs#L112-L185)):** Let $n_{\mathrm{obs}}$ be the count of observed units with sample mean $\bar{Y}_{\mathrm{obs}}$, $n_{\mathrm{miss}}$ be the count of missing units, $p_{\mathrm{obs}} = \frac{n_{\mathrm{obs}}}{n_{\mathrm{obs}} + n_{\mathrm{miss}}}$ be the response rate, and $[y_{\min}, y_{\max}]$ be the physical support of $Y$. Without *any* assumption on the missingness mechanism (`MNAR` allowed), the true population mean $\mathbb{E}[Y]$ must lie in the sharp interval $[L_{\mathrm{M}}, U_{\mathrm{M}}]$:

   $$L_{\mathrm{M}} = p_{\mathrm{obs}}\,\bar{Y}_{\mathrm{obs}} + (1 - p_{\mathrm{obs}})\,y_{\min}, \qquad U_{\mathrm{M}} = p_{\mathrm{obs}}\,\bar{Y}_{\mathrm{obs}} + (1 - p_{\mathrm{obs}})\,y_{\max}$$

   Notice that the identification width $W_{\mathrm{M}} = U_{\mathrm{M}} - L_{\mathrm{M}} = (1 - p_{\mathrm{obs}})(y_{\max} - y_{\min})$ does **not** shrink to zero as $n \to \infty$; sampling fluctuations around $[L_{\mathrm{M}}, U_{\mathrm{M}}]$ are captured by the Imbens–Manski interval $[L_{\mathrm{IM}}, U_{\mathrm{IM}}]$.

2. **David S. Lee's (2009) Sharp Trimming Bounds Under Differential Attrition ([`LeeAttritionBounds`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/identification.rs#L190-L249)):** In a randomized trial where the treatment arm retains a fraction $p_T$ of subjects and the control arm retains $p_C$ ($p_T > p_C$), differential attrition breaks randomization. Under monotonicity of selection, the excess retained fraction $q = \frac{p_T - p_C}{p_T}$ in the larger-retention arm is trimmed from the top (for the lower bound) and from the bottom (for the upper bound), and [`LeeAttritionBounds::evaluate`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/identification.rs#L204-L248) also reports the **tipping-point shift** $\delta^*$ required to overturn the sign of the effect.

3. **Arthur V. Peterson Jr.'s (1976) Dependent Competing-Risks Survival Bounds ([`PetersonSurvivalBounds`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/identification.rs#L253-L293)):** The Kaplan–Meier estimator assumes that censoring is independent of failure. When competing risks or loss-to-follow-up ($c$ censored patients alongside $d$ observed deaths out of $n$) may be dependent (`MNAR`), the true survival probability $S(t)$ is bounded non-parametrically by:

   $$S_{\mathrm{lower}} = 1 - \frac{d + c}{n}, \qquad S_{\mathrm{upper}} = 1 - \frac{d}{n}$$

4. **The Mayo–Manski Severe Boundary Bridge ([`severe_lower_bound_passed`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/identification.rs#L298-L300)):** A substantive claim $\mu > \mu_0$ passes a severe test under partial identification if and only if the **lower Imbens–Manski bound** $L_{\mathrm{IM}}$ strictly exceeds $\mu_0$.

---

## 3.3 Equipping the `StochologicalAgent` with the Epistemic Margin

Look back at [`StochologicalAgent::act`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/engine.rs#L238-L263) in `engine.rs`: when `use_epistemic_margin = true`, the agent replaces `NaiveOptimizer`'s illusory point interval $(\bar{O}_t - 0.5, \bar{O}_t + 0.5)$ with [`ManskiBounds::evaluate`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/identification.rs#L131-L184) over the observed clusters and `obs.missing_count`. Because $[L_{\mathrm{IM}}, U_{\mathrm{IM}}]$ honestly spans the missing vulnerable households, the agent's epistemic coverage jumps from **$0.0\%$** to **$100.0\%$**.

---

## 3.4 Chapter 3 Laboratory: Real Data on the LaLonde Benchmark

Running Chapter 3 (`cargo run --release --bin stochology -- --chapter 3`) tests these partial-identification tools on Robert LaLonde's (1986) classic benchmark (`data/nsw_experimental.csv`, $n = 445$, and `data/cps_observational.csv`, $n = 15{,}992$, Measured):

```text
=== Chapter 3: Underdetermination & Partial Identification (Filter I · Tension 2) ===
  LaLonde (nsw_experimental.csv vs cps_observational.csv): experimental_ATE=$1794 vs naive_CPS_diff=$-8498
  Manski bounds=[$5339, $10907] (width=$5568) | Lee attrition bounds=[$321, $2235]
  Peterson survival bounds=[0.55, 0.80] vs naive=0.73 | e-process E_t=8.53 (Lakatos prog=8.53 vs deg=0.66)
```

- **Experimental vs. Observational Underdetermination (Measured on `nsw_experimental.csv` & `cps_observational.csv`):** In the randomized National Supported Work (NSW) experiment, the true Average Treatment Effect on 1978 earnings (`re78`) is **$+\$1{,}794$** (`$6,349` treated vs. `$4,555` control). Yet if an analyst naively compares the NSW treated group against the non-experimental Current Population Survey (`cps_observational.csv`, mean `$14,847`), unmeasured selection bias flips the sign to **$-\$8{,}498$**—an error exceeding $\$10{,}290$!
- **Manski & Lee Bounds (Measured on `nsw_experimental.csv` with simulated attrition):** Under $35$ missing outcomes with physical support $[\$0, \$35{,}000]$, `ManskiBounds` yields the honest identification region $[\$5{,}339, \$10{,}907]$ (width $\$5{,}568$), while under differential attrition ($18$ treated vs. $45$ control dropouts), `LeeAttritionBounds` brackets the treatment effect within **$[\$321, \$2{,}235]$**—strictly positive even at the worst-case trimming boundary!

---

## Exercises

1. **Identification vs. Estimation Uncertainty:** In [`ManskiBounds::evaluate`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/identification.rs#L131-L184), double both `observed.len()` and `missing_count` while keeping the sample moments constant. What happens to `identification_width` $U_{\mathrm{M}} - L_{\mathrm{M}}$, and what happens to the Imbens–Manski margin $(U_{\mathrm{IM}} - L_{\mathrm{IM}}) - (U_{\mathrm{M}} - L_{\mathrm{M}})$?
2. **Peterson Competing Risks:** Suppose a clinical trial of $n = 100$ patients records $d = 20$ deaths and $c = 25$ patients lost to follow-up (`PetersonSurvivalBounds::evaluate(20, 25, 55)`). Why is the naive independent censoring survival estimate $0.73$ dangerously over-optimistic if patients leave the study precisely when their condition deteriorates?

