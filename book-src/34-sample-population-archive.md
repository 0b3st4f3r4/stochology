# Sample, Population, Archive

> Foundation chapter of Part III. It presupposes [Chapter 14](14-linear-algebra.md) and
> [Chapter 15](15-probability-as-a-measure.md). Its formal content is worked out in
> [Chapter 45](45-world-to-archive-big-data-paradox.md).

## By the end of this chapter you can

1. Say what a sample estimates and what it cannot, for a population that was never enumerated.
2. Separate a large archive from a representative one, and give the arithmetic reason the first
   does not buy the second.
3. Classify missingness as ignorable or not, and say what each classification costs.

## 1. Three different objects

In any empirical investigation, three distinct populations are in play, and in real institutions they
are almost never congruent:

The **target population** ($N$) is the complete universe of cases the decision will govern: every patient
who will arrive in the emergency department next year, every credit borrower facing recession, every structural
joint in the bridge under gale loads. It is never fully enumerated, and by the time it has passed, it has
become history rather than action.

The **archive population** ($n_{\mathrm{arch}}$) is the set of cases an institution actually observed and
recorded. It represents the strict intersection of what physically occurred, what triggered sensor thresholds,
what survived operational data cleaning, and what fell within the categories of the database schema.

The **fitted population** ($n_{\mathrm{fit}}$) is the subset of the archive actually retained by the statistical
pipeline: the rows with complete feature vectors, free of missing fields, that survived listwise deletion and
filtering.

A policy or scientific claim that fails to distinguish which of these three populations it describes is an
unvalidated claim. The vast majority of catastrophic failures in automated machine learning stem from the unmeasured
gulf between the fitted sample and the open target population, a gulf that no in-sample goodness-of-fit statistic
can detect.

## 2. Volume is not representation: Meng's Big Data Paradox

The stubborn intuition that massive sample sizes guarantee asymptotic truth holds under exactly one condition:
that the data recording mechanism is statistically independent of the outcome being measured. When recording is
even slightly selective, data volume sharpens and solidifies the error rather than eliminating it.

In 2018, Xiao-Li Meng proved an exact mathematical identity for the error of the sample mean. Let $f = n/N$ be
the sampling fraction, $\sigma_Y$ the population standard deviation, and $\rho_{R,Y} = \mathrm{Corr}(R, Y)$ the
**data defect correlation** between the recording indicator $R$ ($R=1$ if recorded, $0$ otherwise) and the
substantive outcome $Y$. The standardized estimation error is:

$$\frac{\bar{Y}_n - \bar{Y}_N}{\sigma_Y} = \rho_{R,Y}\sqrt{\frac{1-f}{f}} \qquad \text{\texttt{[Assumed]}}.$$

Read this identity carefully across three structural regimes:
1. *Zero selection bias* ($\rho_{R,Y} = 0$): The recording filter is purely random. The error converges to zero
   at the standard rate $O(1/\sqrt{n})$, the textbook ideal.
2. *Exhaustive census* ($f \to 1$): The archive captures the entire population. The error vanishes because there
   is literally nothing unobserved.
3. *Selective archive under partial coverage* ($\rho_{R,Y} \neq 0, f < 1$): The error is determined entirely by
   the institutional design of the filter. It does **not** shrink as sample size $n$ increases if the sampling
   fraction $f$ remains modest.

Under Meng's identity, the **effective sample size** of a biased big-data archive collapses to:

$$n_{\mathrm{Meng}} = \frac{f}{(1-f)\rho_{R,Y}^2} \qquad \text{\texttt{[Assumed]}}.$$

Consider an administrative database logging ten million records ($n = 10{,}000{,}000$) across a population of
one hundred million ($N = 100{,}000{,}000$, so $f = 0.10$). If self-selection or bureaucratic triage introduces
an imperceptible correlation of merely $\rho_{R,Y} = 0.005$, the effective sample size is:

$$n_{\mathrm{Meng}} = \frac{0.10}{0.90 \times (0.005)^2} = \frac{0.10}{0.0000225} \approx 4{,}444 \qquad \text{\texttt{[Assumed]}}.$$

Ten million rows collected under an uncorrected institutional filter carry the statistical information of fewer
than four thousand five hundred independent random draws, while radiating the spurious authority of a massive census.

## 3. Shared causes, clustering, and weighting efficiency

Administrative and physical records are rarely independent and identically distributed draws. Patients arrive
within clinical wards; credit card transactions cluster by merchant terminals; structural strain readings cluster
by sensor busses. When observations share an unmeasured cluster-level shock, each additional row within the cluster
carries diminishing marginal information.

The **intra-cluster correlation** $\rho_I$ measures the fraction of total variance attributable to differences
between clusters rather than within them. For clusters of average size $\bar{m}$, Donald Moulton (1986) derived
the **cluster design effect**:

$$\mathrm{DEFF}_{\mathrm{cluster}} = 1 + (\bar{m}-1)\rho_I \qquad \text{\texttt{[Assumed]}}.$$

A registry recording $1{,}000$ patients distributed across $40$ hospital wards of $25$ patients each, with a modest
intra-ward correlation of $\rho_I = 0.20$, yields $\mathrm{DEFF} = 1 + (25 - 1) \times 0.20 = 5.8$. The effective
sample size is $1{,}000 / 5.8 \approx 172$ independent patients `[Assumed]`. Collecting ten thousand patients from the
same forty wards increases computational storage without expanding evidentiary diversity.

Similarly, survey post-stratification weights $w_i$ reduce effective sample size. Leslie Kish (1965) proved that
weight dispersion inflates variance by:

$$\mathrm{DEFF}_{\mathrm{Kish}} = 1 + \mathrm{CV}^2(w) = \frac{\sum_i w_i^2}{(\sum_i w_i)^2 / n} \ge 1 \qquad \text{\texttt{[Assumed]}}.$$

Combining Meng's big data defect, Moulton's cluster correlation, and Kish's weight dispersion yields the
**unified harmonic effective sample size**:

$$\frac{1}{n_{\mathrm{eff}}} = \frac{1}{n_{\mathrm{Meng}}} + \frac{\mathrm{DEFF}_{\mathrm{cluster}} - 1}{n} + \frac{\mathrm{DEFF}_{\mathrm{Kish}} - 1}{n} \qquad \text{\texttt{[Assumed]}}.$$

Textual stemmatics (*stemma codicum*) discovered this exact principle in classical philology. In
Karl Lachmann's and Paul Maas's methodology `[Literature]`, counting how many surviving Renaissance
manuscripts agree on a variant reading (*codices plurimi*) commits a nominal-volume fallacy whenever
nine copies derive from a single common damaged archetype (*codices descripti*, $\rho_I \to 1$,
$\mathrm{DEFF} = 9$, $n_{\mathrm{eff}} = 1$ `[Assumed]`). Maas's canon *eliminatio codicum
descriptorum* and the ancient maxim *ponderanda sunt testimonia, non numeranda* (witnesses must be
weighed, not counted) is the philological ancestor of dividing raw volume by design effect before
calculating Keynesian evidential weight (
`epistemic.rs::EpistemicMarginState::keynes_weight_from_n_eff`).

## 4. Missingness and dark archives

Donald Rubin's taxonomy separates missing data by whether missingness is ignorable:
- **MCAR (Missing Completely at Random)**: $\mathbb{P}(M \mid Y_{\mathrm{obs}}, Y_{\mathrm{mis}}) = \mathbb{P}(M)$.
  Observed data is a random subsample. Listwise deletion causes loss of power but zero bias.
- **MAR (Missing at Random)**: $\mathbb{P}(M \mid Y_{\mathrm{obs}}, Y_{\mathrm{mis}}) = \mathbb{P}(M \mid Y_{\mathrm{obs}})$.
  Missingness is conditional on observed covariates. Recoverable through inverse propensity weighting or multiple
  imputation, but fundamentally unverifiable from the dataset itself.
- **MNAR (Missing Not at Random)**: Missingness depends on the unobserved value itself. Imputation creates
  dangerous fictions; the only rigorous recourse is partial identification under worst-case bounds.

To estimate the **dark archive** — entities that appear zero times in the register — Anne Chao (1984) derived the
non-parametric lower bound for unobserved species from the frequencies of singletons ($f_1$, items seen exactly once)
and doubletons ($f_2$, items seen twice):

$$\hat{n}_{00} = \frac{f_1^2}{2 f_2} \qquad \text{\texttt{[Assumed]}}.$$

Archaeological taphonomy (Efremov `[Literature]`) and historical sociology formalize how physical preservation
biases the archive. In *Silencing the Past*, Michel-Rolph Trouillot identified four successive moments where power
manufactures institutional silence:
1. **Fact creation**: Who enters the historical record at all (`epistemic.rs::EpistemicMarginState::from_meng_big_data`).
2. **Fact assembly**: What documents survive into the physical archive (`epistemic.rs::EpistemicMarginState::from_capture_recapture`).
3. **Fact retrieval**: How catalogs and algorithms index and weight what was kept (`epistemic.rs::EpistemicMarginState::from_kish_strata`).
4. **Retrospective significance**: How subsequent generations rewrite the record with hindsight, a distortion
   quarantined by bitemporal logging (`epistemic.rs::bitemporal_view_at`).

## Bridge

Selection bias, clustering, and sampling volume are formalised in
[Chapter 45](45-world-to-archive-big-data-paradox.md) as
`epistemic.rs::EpistemicMarginState::from_archive_clusters`, where the cluster design effect is
separated from the coverage ratio; Meng's Big Data Paradox identity is evaluated in
`epistemic.rs::EpistemicMarginState::from_meng_big_data`; uncounted populations are bounded by
`epistemic.rs::EpistemicMarginState::from_capture_recapture`; and survey weight dispersion is
audited by `epistemic.rs::EpistemicMarginState::from_kish_strata`. Bitemporal archive querying to
prevent hindsight revisionism is executed by
`epistemic.rs::bitemporal_view_at`, and the resulting effective sample size governs Keynes's weight
`epistemic.rs::EpistemicMarginState::keynes_weight_from_n_eff`.
