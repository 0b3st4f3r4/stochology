# Chapter 45 — From World to Archive: Evidential Weight, Big Data Paradox, and the Uncounted

> Laboratory 4 of the `stochology` CLI: `--chapter 4`.

In the age of massive administrative databases, electronic health records, and billion-row telemetry streams, a seductive fallacy dominates institutional decision-making: *"With $n = 2.3\text{ million}$ records, sampling error is practically zero."*

Completing **Mediating Filter I · Axis 1 ($S_{\mathrm{epi}}$: Epistemic Margin)** (implemented in [`crates/stochology/src/epistemic.rs`](crates/stochology/src/epistemic.rs)), this chapter confronts **raw archive volume ($n$) and single-list administrative visibility** against **Canonical Operator I**: the unified harmonic effective sample size law ($n_{\mathrm{eff}}$) uniting **Keynes's evidential weight ($V$), Meng's Big Data Paradox ($\rho_{R,Y}$), Kish–Moulton cluster design effects ($\mathrm{DEFF}_{\mathrm{cluster}}$), Horvitz–Thompson / Kish weighting ($\mathrm{DEFF}_{\mathrm{Kish}}$), Chao's capture-recapture of the uncounted ($\hat{n}_{00}$), and bitemporal data vintages** (Keynes, 1921; Horvitz & Thompson, 1952; Kish, 1965; Campbell, 1979; Chao, 1984; Moulton, 1986; Rubin, 1976; Croushore & Stark, 2001; Lum & Price, 2013; Meng, 2018):

$$\frac{1}{n_{\mathrm{eff}}} = \frac{\mathrm{DEFF}_{\mathrm{total}}}{n} + \left(\frac{1 - f}{f}\right)\rho_{R,Y}^2, \qquad V(n_{\mathrm{eff}}) = \frac{n_{\mathrm{eff}}}{n_{\mathrm{eff}} + n_0}$$

---

## 4.1 Keynes's Evidential Weight $V$ vs. Probability $p$

In Chapter 47 of *A Treatise on Probability* (1921), John Maynard Keynes drew a fundamental distinction that twentieth-century textbook statistics almost forgot:

> *"As the relevant evidence at our disposal increases, the magnitude of the probability of the argument may either decrease or increase, according as the new knowledge strengthens the unfavourable or the favourable evidence; but **something** seems to have increased in either case,—we have a more substantial basis upon which to rest our conclusion."*

A forecast of $p = 0.50$ based on a deeply audited cohort of $1{,}200$ independent households and a forecast of $p = 0.50$ based on two unverified rumors have the **exact same probability** $p = 0.50$, yet radically different **evidential weight** $V = V(x/h) \in (0, 1)$. In [`EpistemicMarginState::keynes_weight_from_n_eff`](crates/stochology/src/epistemic.rs#L236-L240):

$$V(n_{\mathrm{eff}}) = \frac{n_{\mathrm{eff}}}{n_{\mathrm{eff}} + n_0}$$

where $n_{\mathrm{eff}}$ is the unified effective independent sample size (after correcting for selection defects, clustering, and unequal weights) and $n_0$ is the reference scale of structural ignorance.

---

## 4.2 Xiao-Li Meng's (2018) Exact Big Data Paradox Identity: Algebraic Proof

Why can a massive database of $n = 2{,}300{,}000$ records have *less* effective information than a simple random sample of $n = 400$? In his landmark 2018 paper *Statistical Paradoxes in Big Data*, Xiao-Li Meng proved an exact algebraic identity for the error between the sample mean $\bar{Y}_n$ of an archive and the true finite-population mean $\bar{Y}_N$.

### 4.2.1 Proof of the Big Data Identity

Consider a finite population of size $N$ with values $Y_1, \dots, Y_N$, population mean $\bar{Y}_N = \frac{1}{N}\sum_{i=1}^N Y_i$, and finite-population standard deviation $\sigma_Y = \sqrt{\frac{1}{N}\sum_{i=1}^N (Y_i - \bar{Y}_N)^2}$.

Let $R_i \in \{0, 1\}$ be the recording/response indicator for individual $i$. The recorded sample size is $n = \sum_{i=1}^N R_i$, the sampling fraction is $f = n / N$, and the observed sample mean is $\bar{Y}_n = \frac{1}{n}\sum_{i=1}^N R_i Y_i$. Note that the population mean of $R$ is $\bar{R}_N = \frac{n}{N} = f$, and its variance is:
$$\sigma_R^2 = \frac{1}{N}\sum_{i=1}^N (R_i - f)^2 = f(1 - f)$$
The finite-population covariance between the recording mechanism $R$ and the variable of interest $Y$ is:
$$\mathrm{Cov}_N(R, Y) = \frac{1}{N}\sum_{i=1}^N (R_i - f)(Y_i - \bar{Y}_N) = \frac{1}{N}\sum_{i=1}^N R_i Y_i - f \bar{Y}_N = \frac{n}{N}\bar{Y}_n - f \bar{Y}_N = f\bigl(\bar{Y}_n - \bar{Y}_N\bigr)$$
By definition of the finite-population Pearson correlation coefficient $\rho_{R, Y} = \frac{\mathrm{Cov}_N(R, Y)}{\sigma_R \sigma_Y}$:
$$\rho_{R, Y} = \frac{f(\bar{Y}_n - \bar{Y}_N)}{\sqrt{f(1 - f)}\,\sigma_Y} = \sqrt{\frac{f}{1 - f}}\,\frac{\bar{Y}_n - \bar{Y}_N}{\sigma_Y}$$
Rearranging directly establishes Meng's exact, non-asymptotic identity:
$$\bar{Y}_n - \bar{Y}_N = \underbrace{\rho_{R, Y}}_{\text{Data Defect}} \times \underbrace{\sqrt{\frac{1 - f}{f}}}_{\text{Data Scarcity}} \times \underbrace{\sigma_Y}_{\text{Problem Difficulty}}$$

### 4.2.2 The Collapse of Effective Sample Size ($n_{\mathrm{eff}}$)

To find the effective sample size $n_{\mathrm{eff}}$, equate the squared estimation error $(\bar{Y}_n - \bar{Y}_N)^2$ to the variance of a hypothetical simple random sample $\mathrm{Var}(\bar{Y}_{\mathrm{srs}}) = \frac{\sigma_Y^2}{n_{\mathrm{eff}}}\frac{1-f}{f}$ (or using the infinite-population benchmark $\sigma_Y^2 / n_{\mathrm{eff}}$):
$$\rho_{R, Y}^2 \left(\frac{1 - f}{f}\right)\sigma_Y^2 = \frac{\sigma_Y^2}{n_{\mathrm{eff}}} \implies n_{\mathrm{eff}} = \frac{f}{1 - f}\,\frac{1}{\rho_{R, Y}^2}$$

Notice that $n_{\mathrm{eff}}$ depends only on the sampling fraction $f = n/N$ and the data defect correlation $\rho_{R,Y}$, completely decoupled from the raw sample size $n$!
If an administrative database covers $f = 23\%$ of a population of $N = 10{,}000{,}000$ ($n = 2{,}300{,}000$, `[Assumed]`), but suffers a slight selection bias $\rho_{R,Y} = -0.05$ (`[Assumed]`):
$$n_{\mathrm{eff}} = \frac{0.23}{1 - 0.23}\,\frac{1}{(-0.05)^2} = \frac{0.23}{0.77 \times 0.0025} = \frac{0.23}{0.001925} \approx 119.5$$
The raw sample of $2.3\text{ million}$ records provides the evidential weight of just **$119.5$ independent observations**—a **$99.995\%$ collapse** in information! Under such data defect, standard confidence intervals are shifted so far from the truth that empirical coverage collapses to $0.0000\%$.

---

## 4.3 Campbell–Moulton Cluster Design Effects ($\mathrm{DEFF}$) and Unequal Cluster Sizes

Even when $\rho_{R,Y} = 0$, administrative rows are almost never independent: patients are clustered within hospital wards, calls arrive in correlated daily waves, and workers belong to regional cohorts.

When cluster sizes $m_1, \dots, m_C$ vary with sample mean $\bar{m} = \frac{1}{C}\sum_{c=1}^C m_c$ and variance $\sigma_m^2 = \frac{1}{C}\sum_{c=1}^C (m_c - \bar{m})^2$, Donald Campbell (1979) and Brent Moulton (1986) showed that cluster size heterogeneity inflates the effective cluster size from $\bar{m}$ to:
$$\tilde{m} = \bar{m}\bigl(1 + \mathrm{CV}^2(m)\bigr), \qquad \mathrm{CV}(m) = \frac{\sigma_m}{\bar{m}}$$
The resulting **cluster design effect** ([`EpistemicMarginState::from_archive_clusters`](crates/stochology/src/epistemic.rs#L844-L942)) is:
$$\mathrm{DEFF}_{\mathrm{cluster}} = 1 + (\tilde{m} - 1)\,\rho_I = 1 + \bigl[\bar{m}\bigl(1 + \mathrm{CV}^2(m)\bigr) - 1\bigr]\rho_I$$
where $\rho_I$ is the intra-cluster correlation coefficient (ICC). When clusters are highly variable in size ($\mathrm{CV}(m) > 1$), neglecting the Moulton variance correction $\mathrm{CV}^2(m)$ severely underestimates $\mathrm{DEFF}_{\mathrm{cluster}}$ and understates standard errors, producing phantom statistical significance. The effective sample size for variance sizing collapses to:
$$n_{\mathrm{eff}}^{\mathrm{cluster}} = \frac{n}{\mathrm{DEFF}_{\mathrm{cluster}}}$$

---

## 4.4 Horvitz–Thompson / Kish Weighting and Chao's (1984) Lower Bound on the Uncounted

How do we recover information about under-recorded strata?
1. **Horvitz–Thompson (1952) & Leslie Kish's (1965) Design Effect ([`EpistemicMarginState::from_kish_strata`](crates/stochology/src/epistemic.rs#L1007-L1045)):** When recorded unit $i$ has inclusion probability $\pi_i > 0$, inverse-probability weights $w_i = 1/\pi_i$ remove selection bias, but extreme weight dispersion reduces effective sample size to Kish's floor:

   $$n_{\mathrm{eff, Kish}} = \frac{\bigl(\sum_{i=1}^n w_i\bigr)^2}{\sum_{i=1}^n w_i^2} = \frac{n}{1 + \mathrm{CV}^2(w)}$$

2. **Anne Chao's (1984) Capture-Recapture Lower Bound ($\hat{n}_{00}$, [`EpistemicMarginState::from_capture_recapture`](crates/stochology/src/epistemic.rs#L1051-L1093)):** What if a vulnerable subpopulation has $\pi_i = 0$ in a single administrative registry? Following Patrick Ball, Megan Price, and Kristian Lum (HRDAG, 2013), cross-referencing multiple incomplete lists (or counting units captured once $f_1 = n_{10} + n_{01}$ vs. twice $f_2 = n_{11}$) yields Chao's non-parametric lower bound on the **completely uncounted population** $n_{00}$:

   $$\hat{n}_{00,\mathrm{Chao}} = \frac{f_1^2}{2 f_2}, \qquad \hat{n}_{00,\mathrm{LP}} = \frac{n_{10}\,n_{01}}{n_{11}}$$

---

## 4.5 Bitemporal Vintages: Event Time ($t_{\mathrm{ev}}$) vs. Availability Time ($t_{\mathrm{disp}}$)

Finally, archives evolve dynamically over time: macroeconomic series, epidemiological counts, and hospital records are revised months or quarters after the event occurred. In [`BitemporalRecord`](crates/stochology/src/epistemic.rs#L37-L44), [`bitemporal_view_at`](crates/stochology/src/epistemic.rs#L56-L79), and [`EpistemicMarginState::from_vintages`](crates/stochology/src/epistemic.rs#L1086-L1141), every record carries both its **event time** $t_{\mathrm{ev}}$ and its **institutional availability time** $t_{\mathrm{disp}} \ge t_{\mathrm{ev}}$.

Any record with $t_{\mathrm{disp}} < t_{\mathrm{ev}}$ is rejected as a causal look-ahead leak, and a decision at time $\tau$ may condition only on the latest vintage satisfying $t_{\mathrm{disp}} \le \tau$. When models are trained or evaluated on final revised data, they suffer from **retrospective hindsight leakage**:
- The real-time release available to policymakers at time $t_{\mathrm{disp}}$ often indicates economic expansion ($\Delta Y > 0$), while subsequent benchmark revisions flip the estimate to contraction ($\Delta Y < 0$).
- As demonstrated empirically below on the Philadelphia Fed Real-Time Data Set, **$6.6\%$ of quarters flip sign** between initial release and benchmark revision.
- Retrospective policy simulations using latest-vintage data impute clairvoyant knowledge to the historical decision-maker, leading to severe causal misattribution.

---

## 4.6 Laboratory 4: Real Data on CPS Cohorts, Bank Calls, and Philadelphia Fed Vintages

Running laboratory 4 (`cargo run --release --bin stochology -- --chapter 4`) evaluates Canonical Operator I on three real datasets (`data/cps_observational.csv`, `data/bank_calls.csv`, and `data/rtdsm_routput_vintages.csv`, `[Measured]`):

```text
=== CHAPTER 4: FILTER I (S_EPI) — FROM WORLD TO ARCHIVE & THE BIG DATA PARADOX ===
  Canonical Operator I (Harmonic n_eff & Keynes Weight V = n_eff / (n_eff + n_0)):
  1. Meng Big Data Paradox [Assumed] (N = 10M, n = 2.3M, rho_RY = -0.05): n_eff = 119.5, meng_error = -0.0457, V = 0.046
  2. Demographic Cluster DEFF on real re78 [Measured] (data/cps_observational.csv, [Assumed n_0=25.565]): DEFF = 10.37, n_eff = 1542.0, V = 0.984
  3. Temporal Cohort DEFF [Measured] (data/bank_calls.csv, [Assumed n_0=100.0]): DEFF = 8.19, n_eff = 3384.5, V = 0.971
  4. Kish Weighting Floor [Assumed]: DEFF_Kish = 2.78, n_eff_floor = 360.0 | Chao Unseen Dark Archive [Assumed]: n00_hat = 751.6, N_Chao_LB = 2237.1, U_Chao_95 = 2719.9
  5. Bitemporal Vintages [Measured] (data/rtdsm_routput_vintages.csv, Q=241 evaluated quarters across 244 vintages): mean_rev = 1.47 pp, max_rev = 8.02 pp, sign_flips = 6.6% (causal_view_len=1)
```

- **Cluster Echoes in CPS Real Earnings (`cps_observational.csv`, $n = 15{,}992$, `[Measured]`) and Bank Calls (`bank_calls.csv`, $n = 27{,}716$, `[Measured]`):** Grouping CPS respondents directly by their observed `(age, educ)` demographic-education cells on continuous 1978 earnings (`re78` in thousands of USD, using the exact one-way random-effects ANOVA intra-cluster correlation $\rho_{\mathrm{intra}}$ without any binary tail indicator or synthetic score weights) yields $\mathrm{DEFF} = 10.37$ (`[Measured]`), shrinking $n = 15{,}992$ to $n_{\mathrm{eff}} = 1{,}542.0$ (Keynes weight $V = 0.984$ at `[Assumed n_0=25.565]`). In `bank_calls.csv`, continuous daily call-volume clustering produces $\mathrm{DEFF} = 8.19$ (`[Measured]`), shrinking $n = 27{,}716$ rows to $n_{\mathrm{eff}} = 3{,}384.5$ ($V = 0.971$ at `[Assumed n_0=100.0]`). In addition, Chao's capture-recapture estimator (`from_capture_recapture`) reports the non-degenerate interval $[N_{\mathrm{Chao,LB}}, U_{\mathrm{Chao},95}] = [2{,}237.1, 2{,}719.9]$ (`[Assumed]`).
- **Real-Time Macroeconomic Vintages (`rtdsm_routput_vintages.csv`, $Q = 241$ evaluated quarters spanning 1965:Q3 to 2026:Q2, drawn from 244 real-time vintages whose own releases run 1965:Q4 to 2026:Q3, `[Measured]`):** Comparing the initial real-time release of U.S. real output growth against its final revised vintage reveals a mean absolute revision of **$1.47\text{ percentage points}$** (`[Measured]`), a maximum revision of **$8.02\text{ percentage points}$** (`[Measured]`), and **$6.6\%$ of quarters flipping sign** (`[Measured]`) between expansion and contraction! Backtesting a policy on final revised data gives the agent crystal-ball knowledge it never possessed in real time.

---

## Exercises

1. **Harmonic Unification of $n_{\mathrm{eff}}$:** Inspect [`EpistemicMarginState::unified_effective_n`](crates/stochology/src/epistemic.rs#L205-L217). How does recognizing the reduced $n_{\mathrm{eff}}^{\mathrm{var}}$ affect the Baker–McHale epistemic Kelly fraction in the agent's next decision step?
2. **Capture-Recapture Dark Figure:** In [`EpistemicMarginState::from_capture_recapture(95, 340, 210)`](crates/stochology/src/epistemic.rs#L1051-L1093), $645$ distinct individuals were observed across two registries, yet $\hat{n}_{00} = 751.6$ individuals remain completely unrecorded by either list. What happens to $\hat{n}_{00}$ if the two lists suffer positive list dependence (both registries share the same geographic blind spot)?

