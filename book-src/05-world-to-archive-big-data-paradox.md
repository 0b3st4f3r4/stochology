# Chapter 5 — From World to Archive: Evidential Weight, Big Data Paradox, and the Uncounted

In the age of massive administrative databases, electronic health records, and billion-row telemetry streams, a seductive fallacy dominates institutional decision-making: *"With $n = 2.3\text{ million}$ records, sampling error is practically zero."*

**Tension 4** of **Mediating Filter II** (implemented in [`crates/stochology/src/archive.rs`](../crates/stochology/src/archive.rs)) confronts **raw archive volume ($n$) and single-list administrative visibility** against **Keynes's evidential weight ($V$), Meng's Big Data Paradox ($\rho\_{R,Y}$), Campbell–Moulton cluster design effects, Horvitz–Thompson / Kish weighting, Chao's capture-recapture of the uncounted ($\hat{n}\_{00}$), and bitemporal data vintages** (Keynes, 1921; Horvitz & Thompson, 1952; Kish, 1965; Campbell, 1977; Chao, 1984; Moulton, 1986; Rubin, 1976; Croushore & Stark, 2001; Lum & Price, 2013; Meng, 2018).

---

## 5.1 Keynes's Evidential Weight $V$ vs. Probability $p$

In Chapter 6 of *A Treatise on Probability* (1921), John Maynard Keynes drew a fundamental distinction that twentieth-century textbook statistics almost forgot:

> *"As the relevant evidence at our disposal increases, the magnitude of the probability of the argument may either decrease or increase, according as the new knowledge strengthens the unfavourable or the favourable evidence; but **something** seems to have increased in either case,—we have a more substantial basis upon which to rest our conclusion."*

A forecast of $p = 0.50$ based on a deeply audited cohort of $1{,}200$ independent households and a forecast of $p = 0.50$ based on two unverified rumors have the **exact same probability** $p = 0.50$, yet radically different **evidential weight** $V = V(x/h) \in (0, 1)$. In [`KeynesEvidentialWeight`](../crates/stochology/src/archive.rs#L17-L56):

$$V = \frac{K\_{\mathrm{eff}}}{K\_{\mathrm{eff}} + I\_{\mathrm{unexplored}}}$$

where $K\_{\mathrm{eff}}$ is the effective independent information (after correcting for selection defects and clustering) and $I\_{\mathrm{unexplored}}$ is the scale of unrecorded strata and structural ignorance.

---

## 5.2 Xiao-Li Meng's (2018) Exact Big Data Paradox Identity

Why can a massive database of $n = 2{,}300{,}000$ records have *less* effective information than a simple random sample of $n = 400$? In his landmark 2018 paper *Statistical Paradoxes in Big Data*, Xiao-Li Meng proved an exact algebraic identity for the error between the sample mean $\bar{Y}\_n$ of an archive (indicated by $R\_i \in \\{0, 1\\}$, sampling fraction $f = n / N$) and the true finite-population mean $\bar{Y}\_N$:

$$\bar{Y}\_n - \bar{Y}\_N = \underbrace{\rho\_{R, Y}}\_{\text{Data Defect}} \times \underbrace{\sqrt{\frac{1 - f}{f}}}\_{\text{Data Scarcity}} \times \underbrace{\sigma\_Y}\_{\text{Problem Difficulty}}$$

Look closely at the middle factor $\sqrt{(1 - f)/f}$: in a simple random sample ($\mathbb{E}[\rho\_{R,Y}^2] = \frac{1}{N - 1}$), the population size $N$ cancels out and the standard error shrinks at the familiar rate $1/\sqrt{n}$. **But when recording is non-random** ($\rho\_{R,Y} \ne 0$, for example when vulnerable patients or informal workers are less likely to appear in the digital archive), the error scales with $\sqrt{N}$, and the **effective simple-random-sample size** $n\_{\mathrm{eff}}$ ([`MengBigDataDefect`](../crates/stochology/src/archive.rs#L61-L120)) collapses to:

$$n\_{\mathrm{eff}} = \frac{f}{1 - f}\\,\frac{1}{\rho\_{R,Y}^2}$$

Notice that $n\_{\mathrm{eff}}$ depends only on the sampling fraction $f = n/N$ and the data defect correlation $\rho\_{R,Y}$! If $f = 23\\%$ of a population of $N = 10{,}000{,}000$ ($n = 2{,}300{,}000$) has a selection correlation of just $\rho\_{R,Y} = -0.05$, its effective sample size is **$n\_{\mathrm{eff}} = 119.5$**—a **$99.995\\%$ collapse** in effective sample size!

---

## 5.3 Campbell–Moulton Cluster Design Effects ($\mathrm{DEFF}$)

Even when $\rho\_{R,Y} = 0$, administrative rows are almost never independent: patients are clustered within hospital wards, calls arrive in correlated daily waves, and workers belong to regional cohorts. Under unequal cluster sizes $m\_1, \dots, m\_C$ with mean $\bar{m}$ and coefficient of variation $\mathrm{CV}(m)$, Brent Moulton (1986) and Donald T. Campbell (1977) showed that the effective cluster size is $\tilde{m} = \bar{m}(1 + \mathrm{CV}^2)$, and the **design effect** ([`ClusterDesignEffect`](../crates/stochology/src/archive.rs#L125-L214)) at threshold exceedance quantile $p$ is:

$$\mathrm{DEFF}(p) = 1 + (\tilde{m} - 1)\\,\rho\_I(p), \qquad n\_{\mathrm{eff}}(p) = \frac{n}{\mathrm{DEFF}(p)}$$

where $\rho\_I(p)$ is the intra-cluster correlation (ICC) of tail exceedances $\mathbb{I}\\{Y\_{cj} > q\_p\\}$.

---

## 5.4 Horvitz–Thompson / Kish Weighting and Chao's (1984) Lower Bound on the Uncounted

How do we recover information about under-recorded strata?
1. **Horvitz–Thompson (1952) & Leslie Kish's (1965) Design Effect ([`KishWeightAudit`](../crates/stochology/src/archive.rs#L219-L263)):** When recorded unit $i$ has inclusion probability $\pi\_i > 0$, inverse-probability weights $w\_i = 1/\pi\_i$ remove selection bias, but extreme weight dispersion reduces effective sample size to Kish's floor:

   $$n\_{\mathrm{eff, Kish}} = \frac{\bigl(\sum\_{i=1}^n w\_i\bigr)^2}{\sum\_{i=1}^n w\_i^2} = \frac{n}{1 + \mathrm{CV}^2(w)}$$

2. **Anne Chao's (1984) Capture-Recapture Lower Bound (`hat_n_00`, [`CaptureRecaptureAudit`](../crates/stochology/src/archive.rs#L268-L303)):** What if a vulnerable subpopulation has $\pi\_i = 0$ in a single administrative registry? Following Patrick Ball, Megan Price, and Kristian Lum (HRDAG, 2013), cross-referencing multiple incomplete lists (or counting units captured once $f\_1 = n\_{10} + n\_{01}$ vs. twice $f\_2 = n\_{11}$) yields Chao's non-parametric lower bound on the **completely uncounted population** $n\_{00}$:

   $$\hat{n}\_{00,\mathrm{Chao}} = \frac{f\_1^2}{4 f\_2}, \qquad \hat{N}\_{\mathrm{Petersen}} = \frac{(n\_{10} + n\_{11})(n\_{01} + n\_{11})}{n\_{11}}$$

---

## 5.5 Bitemporal Vintages: Event Time ($t\_{\mathrm{ev}}$) vs. Availability Time ($t\_{\mathrm{disp}}$)

Finally, archives evolve over time: macroeconomic series, epidemiological counts, and hospital records are revised months or quarters after the event occurred. In [`BitemporalRecord`](../crates/stochology/src/archive.rs#L308-L320), [`bitemporal_view_at`](../crates/stochology/src/archive.rs#L323-L343), and [`VintageRevisionAudit`](../crates/stochology/src/archive.rs#L367-L426), every record carries both its **event time** $t\_{\mathrm{ev}}$ and its **institutional availability time** $t\_{\mathrm{disp}} \ge t\_{\mathrm{ev}}$. Any record with $t\_{\mathrm{disp}} < t\_{\mathrm{ev}}$ is rejected as a causal look-ahead leak, and a decision at time $\tau$ may condition only on the latest vintage satisfying $t\_{\mathrm{disp}} \le \tau$.

---

## 5.6 Chapter 5 Laboratory: Real Data on CPS Cohorts, Bank Calls, and Philadelphia Fed Vintages

Running Chapter 5 (`cargo run --release --bin stochology -- --chapter 5`) evaluates these five instruments on three real datasets (`data/cps_observational.csv`, `data/bank_calls.csv`, and `data/rtdsm_routput_vintages.csv`, Measured):

```text
=== Chapter 5: From World to Archive & Big Data Paradox (Filter II · Tension 4) ===
  Meng Big Data (n=2.3M / N=10M, rho=-0.05): n_eff=119.5 | CPS clusters (n=15992): DEFF=13.00, n_eff=1230.2, Keynes_V=0.330
  Bank Calls (bank_calls.csv): DEFF=22.48, n_eff=1233.0 | Kish floor n_eff=12.3 | Chao uncounted n00=450
  Real-Time Vintages (rtdsm_routput_vintages.csv, Q=241): mean_revision=1.47 pp, max_revision=8.02 pp, sign_flips=6.6%
```

- **Cluster Echoes in CPS (`cps_observational.csv`, $n = 15{,}992$, Measured) and Bank Calls (`bank_calls.csv`, $n = 27{,}716$, Measured):** Grouping CPS respondents by demographic-education cohorts yields $\mathrm{DEFF} = 13.00$, shrinking $n = 15{,}992$ to $n\_{\mathrm{eff}} = 1{,}230.2$ (Keynes weight $V = 0.330$). In `bank_calls.csv`, daily call clustering produces $\mathrm{DEFF} = 22.48$, shrinking $n = 27{,}716$ rows to $n\_{\mathrm{eff}} = 1{,}233.0$.
- **Real-Time Macroeconomic Vintages (`rtdsm_routput_vintages.csv`, $Q = 241$ quarters from 1965:Q3 to 2025:Q3, Measured):** Comparing the initial real-time release of U.S. real output growth against its final revised vintage reveals a mean absolute revision of **$1.47\text{ percentage points}$**, a maximum revision of **$8.02\text{ percentage points}$**, and **$6.6\\%$ of quarters flipping sign** between expansion and contraction! Backtesting a policy on final revised data gives the agent crystal-ball knowledge it never possessed in real time.

---

## Exercises

1. **Why `StochologicalAgent` Takes $\min(n\_{\mathrm{eff, DEFF}}, n\_{\mathrm{eff, Meng}})$:** Inspect lines 239–256 of [`crates/stochology/src/engine.rs`](../crates/stochology/src/engine.rs#L239-L256). How doesrecognizing the reduced $n\_{\mathrm{eff}}$ affect the Baker–McHale epistemic Kelly fraction in the agent's next decision step?
2. **Capture-Recapture Dark Figure:** In [`CaptureRecaptureAudit::evaluate(200, 300, 300)`](../crates/stochology/src/archive.rs#L283-L302), $700$ distinct individuals were observed across two registries ($n\_{11} = 200, n\_{10} = 300, n\_{01} = 300$), yet $\hat{n}\_{00} = 450$ individuals remain completely unrecorded by either list. What happens to $\hat{n}\_{00}$ if the two lists suffer positive list dependence (both registries share the same geographic blind spot)?

