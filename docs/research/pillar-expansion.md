# Pillar Expansion — Six Real-Data Instruments (Pre-Registered Specification)

Status: **frozen before implementation** (2026-10-30). This page is the specification each new instrument is
implemented against; it is written before any number below is computed, so every threshold here is **Assumed**
(`AGENTS.md` §II.C) and fixed in advance. Results are reported in the chapters and labs, never edited back into
this page. Deviations found during implementation are appended in §10, never rewritten in place.

## 1. Why now, and what "real-world adherence" means here

The competency graph (`docs/competencies.toml`) turns each negative proof into a verified pathway, so a new
instrument enters the discipline as **one competency + one negative proof + one pathway**, with prerequisites
that never point forward in the book. That gives the expansion a structural place and keeps the kernel intact.

Real-world adherence is measured by one criterion: **an instrument must replace an `Assumed` input with a
`Measured` one, or test a protocol claim against a real dataset in `data/*.csv`.** The audit that motivated
this round:

| Gap | Where it shows | Provenance today |
| --- | --- | --- |
| Wasserstein sensitivity ε is hand-set in every card (`0.02`–`0.20`) | `cards/*.yaml`, `examples/*/card_*.yaml` | Assumed |
| CORP-Murphy covers binary events only; continuous forecasts get a lone CRPS | `layers/jensen.rs` (`empirical_crps`) | Measured but undecomposed |
| The attempt counter K penalizes by Bonferroni on e-values (`sup E / K`), with no FDR control over many hypotheses | `siga/roadmap/martingales.rs` | — |
| Invariance is tested (ICP, NILE, Cinelli-Hazlett) but no estimator trades in-sample fit for shift robustness | `lenses/invariance/` | — |
| The bitemporal clock is proven on synthetic canaries; no real dataset carries revisions | `governance/clock.rs` | Simulated |
| Fairness audits error rates (Chouldechova) but not calibration inside intersecting subgroups | `core/invariants.rs` | Measured (`toronto_arrests.csv`) |

## 2. Selection rule and discarded alternatives

Each pillar receives exactly one instrument, chosen by (a) a peer-reviewed source, (b) a closed form or a
finite algorithm implementable in `std`-only Rust, (c) a real dataset already sealed in `data/`, or one new
public dataset only where none serves, and (d) a negative proof that a naive analyst would actually commit.
Alternatives considered and not used (recorded as unused findings, Prohibition 4):

- **C:** interval-score decomposition (Allen, Burnello & Ziegel) — deferred; CRPS covers the full distribution.
- **A (anytime):** U-eBH randomized refinement — rejected: randomization breaks reproducibility of a gate decision.
- **P:** online-search performative power by browser experiments (Mendler-Dünner, Carovano & Hardt 2024) — no
  public randomized platform log in `data/`; the randomized NSW arms are used instead (§5).
- **I:** Rosenbaum Γ sensitivity — overlaps the existing Cinelli-Hazlett robustness value.
- **B:** ALFRED vintages — needs an API key; the Philadelphia Fed real-time data set is a direct public download.
- **A (auditing):** full multicalibration *learning* (boosting) — the protocol audits, it does not retrain.

## 3. C — Calibrated: `IdrCrpsDecomposition`

- **Source (Literature, informative):** Arnold, Walz, Ziegel & Gneiting (2024), "Decompositions of the mean
  continuous ranked probability score", *Electronic Journal of Statistics* 18(2):4992–5044; Henzi, Ziegel &
  Gneiting (2021), "Isotonic distributional regression", *JRSS-B* 83(5):963–993.
- **Method:** $\overline{\mathrm{CRPS}} = \mathrm{MCB} - \mathrm{DSC} + \mathrm{UNC}$, with the recalibrated
  forecast obtained by isotonic distributional regression (IDR) of the outcome on a real-valued forecast index.
  Using $\mathrm{CRPS}(F, y) = \int (F(z) - \mathbb{1}\{y \le z\})^2\,dz$, IDR under a total order equals
  threshold-wise antitonic regression of $\mathbb{1}\{y \le z\}$, so the existing PAV (`CorpPavDecomposition`)
  is reused threshold by threshold over the union of outcomes and ensemble members; the integral is exact
  because every integrand is piecewise constant on that grid.
- **Restriction (authorial):** the forecast index is the ensemble median (a total order), not the stochastic
  order on whole forecast distributions used by Arnold et al.
- **Data (Measured):** `vic_elec.csv`, daily maximum demand. Forecast for day $d$: the 4-member same-weekday
  ensemble $\{d-7, d-14, d-21, d-28\}$; evaluated on every day with 28 days of history.
- **Admissibility rule (Assumed):** a forecast is admissible when $\mathrm{MCB} \le \tfrac{1}{2}\mathrm{DSC}$
  (miscalibration consumes at most half of the discrimination gain).
- **Negative proof `crps_miscalibration` (gate G6):** the same ensemble shifted by $-8\%$ with spread shrunk
  to a quarter still beats climatology on the lone CRPS ($\mathrm{CRPS} < \mathrm{UNC}$) but violates the rule.

## 4. A — Anytime-valid: `EBhMultiplicityAudit`

- **Source:** Wang & Ramdas (2022), "False discovery rate control with e-values", *JRSS-B* 84(3):822–852.
- **Method:** e-BH at level α over K e-values: reject the $\hat{k}$ largest, where
  $\hat{k} = \max\{k : e_{(k)} \ge K / (\alpha k)\}$. FDR ≤ α under arbitrary dependence. Rejections are the
  used findings; every non-rejected hypothesis is named in the discarded list, and K is the attempt counter.
- **Data (Measured):** `nyc_flights_2013.csv` vs `nyc_flights_2023.csv` (Aug–Dec). One hypothesis per
  carrier present in both years: $H_{0,k}$: the 2023 rate of arrivals more than 15 minutes late equals the
  2013 rate of the same carrier. E-value: the GRAPA betting e-process (`EvidenceProcess::from_grapa_betting`)
  on $X_i = \mathbb{1}\{\text{delay}_i > 15\} - \hat{p}_{2013,k}$ over 2023 flights in schedule order,
  evaluated at the end of the fixed stream. The 2013 rate is conditioned on as a fixed reference.
- **Level (Assumed):** α = 0.05.
- **Negative proof `multiplicity` (gate G6):** under a simulated global null (K = 20 Bernoulli streams of
  length 500, 200 replications, seed 2026), the naive rule "reject each hypothesis whose running e-process
  ever crosses 1/α" (peeking, no multiplicity) has empirical FDR above α, while e-BH on the terminal e-values
  stays at or below α.

## 5. P — Performative-aware: `PerformativePowerAudit`

- **Source:** Hardt, Jagadeesan & Mendler-Dünner (2022), "Performative power", *NeurIPS 35*; Perdomo, Zrnic,
  Mendler-Dünner & Hardt (2020), *ICML*.
- **Method:** a randomized action $\theta \in \{0, 1\}$ gives a measured lower bound on the Wasserstein
  sensitivity: $\varepsilon \ge W_1(\mathcal{D}(1), \mathcal{D}(0)) / |1 - 0|$, with outcomes standardized by
  the control-arm standard deviation. A permutation test (999 permutations, seed 2026) decides whether the
  distance exceeds the noise floor; when it does, the lower bound is the 5% bootstrap quantile (999 resamples,
  seed 2026) of the standardized $W_1$, otherwise it is 0.
- **Reading (authorial extension):** treating a job-training assignment as the "performative" action of a
  decision-maker on the world is an analogy; the distance itself is Measured.
- **Data (Measured):** `nsw_experimental.csv`, outcome `re78`, arms `treat`.
- **Rule:** a declared ε below the measured lower bound is understated; the RRM condition
  $\varepsilon\beta/\gamma < 1$ is re-audited with the measured bound.
- **Negative proof `understated_epsilon` (gate G6):** a declaration of a passive agent (ε = 0) for the
  measured action is blocked.

## 6. I — Invariant: `AnchorRegressionAudit`

- **Source:** Rothenhäusler, Meinshausen, Bühlmann & Peters (2021), "Anchor regression: heterogeneous data
  meet causality", *JRSS-B* 83(2):215–246.
- **Method:** $\hat{b}_\gamma = \arg\min_b \|(I - \Pi_A)(Y - Xb)\|^2 + \gamma\|\Pi_A(Y - Xb)\|^2$, solved as
  OLS on $W_\gamma = I + (\sqrt{\gamma} - 1)\Pi_A$; with categorical anchors $\Pi_A$ is the projection onto
  cell means. Anchor stability: relative gap $\|\hat{b}_\gamma - \hat{b}_1\| / \|\hat{b}_1\|$.
- **Data (Measured):** `nyc_flights_2013.csv` (train) → `nyc_flights_2023.csv` (shifted test). Response:
  arrival delay winsorized to $[-60, 300]$ minutes; predictors: intercept, scheduled departure hour,
  distance in thousands of miles; anchors: origin × month cells. Report pooled and worst-cell test MSE for
  γ ∈ {1, 2, 5, 10, 100}. The sealed reserves (`*_h1.csv`, `houston_flights_2011.csv`) are not opened.
- **Rule (Assumed):** γ = 5 is the primary shift strength; a claim of invariance is admissible only when the
  relative gap at γ = 5 is ≤ 0.10.
- **Negative proof `anchor_extrapolation` (gate G6):** a simulated anchor-confounded structural model
  (seed 2026) where OLS is not anchor-stable; a card that claims invariant coefficients is blocked.

## 7. B — Bitemporal: `RealTimeVintageAudit`

- **Source:** Croushore & Stark (2001), "A real-time data set for macroeconomists", *Journal of Econometrics*
  105(1):111–130; Croushore (2011), "Frontiers of real-time data analysis", *Journal of Economic Literature*
  49(1):72–100.
- **New dataset:** Federal Reserve Bank of Philadelphia, Real-Time Data Set for Macroeconomists, `ROUTPUT`
  (real GNP/GDP), quarterly vintages, converted from `routputqvqd.xlsx` to long form
  `data/rtdsm_routput_vintages.csv` (`vintage,obs_quarter,routput`); transformation in `data/SOURCES.md`.
- **Method:** each value is a bitemporal record with event time = observation quarter and availability time =
  vintage. Growth rates are computed inside a vintage (levels change base year across vintages). Backtest
  1985Q1–2024Q4: at decision quarter $t$ the forecaster sees only vintage $t$; forecast of the next growth =
  mean of the last 4 growth rates in that vintage. Scored against the first release and against the latest
  vintage. Reported: RMSE real-time, RMSE with latest-vintage inputs, mean and SD of the first-to-latest
  revision.
- **Negative proof `vintage_leak` (gate G7):** a backtest that feeds latest-vintage values into past decision
  quarters (availability after the decision) is blocked by the bitemporal clock.

## 8. A — Auditing: `MulticalibrationAudit`

- **Source:** Hébert-Johnson, Kim, Reingold & Rothblum (2018), "Multicalibration: calibration for the
  (computationally-identifiable) masses", *ICML*, PMLR 80:1939–1948.
- **Method:** for each subgroup $S$ in a declared collection and each forecast bin $v$ (global quintiles),
  gap $= |\mathbb{E}[Y - p \mid S, v]|$ on cells with at least 30 units. A cell violates α-multicalibration
  when the Hoeffding lower confidence bound of its gap, at level $0.05 / \#\text{cells}$, exceeds α.
- **Data (Measured):** `toronto_arrests.csv`, outcome `released`. Predictor: logistic regression without
  `colour` (age, sex, employed, citizen, checks, year) fitted on odd `rownames`, globally PAV-recalibrated on
  the same half, audited on even `rownames`. Subgroups: every combination of colour × sex × employed × citizen,
  plus each marginal level.
- **Tolerance (Assumed):** α = 0.05.
- **Negative proof `subgroup_miscalibration` (gate G4):** a simulated predictor (seed 2026) with small global
  REL_CORP that is biased by ±0.15 in complementary subgroups passes the global CORP check and is blocked by
  the subgroup audit.

## 9. Integration contract

Counts move from `3 + 4 + 2 + 3 + 2 + 7 = 21` to `4 + 5 + 3 + 4 + 3 + 8 = 27` negative proofs; six DAG nodes
enter `docs/competencies.toml` (chapters 08, 09, 15, 19, 20, 21). Module roots in `stochology-lenses` and
`stochology-core/src/layers` are sealed, so the acceptance registry is regenerated and `data/SHA256SUMS`
re-sealed under human review; no G8 claim changes.

## 10. Deviations log

(Appended during implementation.)

- **D1 (§3, method).** Ordering by the ensemble median does not make the original forecast $F_i(z)$ antitonic in
  that index (a wider ensemble with a higher median can have a higher CDF at some $z$), so MCB can turn
  negative. Implemented instead: the threshold-wise CORP decomposition (`CorpCrpsDecomposition`), ordering by
  $F_i(z)$ at each threshold. Components stay non-negative and the identity closes exactly; the recalibrated
  $\hat{F}_i$ need not be a CDF, and `max_cdf_violation` reports the largest decrease.
- **D2 (§3, negative proof).** The pre-registered distortion ($-8\%$ bias, spread $\times 0.25$) does **not**
  beat climatology on `vic_elec.csv` (CRPS $548.4$ vs UNC $442.7$ MW), so the lone CRPS would already reject it
  and the proof would show nothing. Attempts (K = 10, all recorded): bias $\in \{-8, -5, -4, -3, -2\}\%$ ×
  spread $\in \{0.25, 0.5\}$. Seven beat climatology; six of those violate the admissibility rule. Chosen:
  $-3\%$, spread $\times 0.25$ (CRPS $422.8 <$ UNC $442.7$; MCB $74.9 > \tfrac12$ DSC $= 47.4$), the
  combination with margin on both sides. The admissibility rule itself ($\mathrm{MCB} \le \tfrac12\mathrm{DSC}$)
  was not changed.
- **D3 (§6, clarification fixed before computing).** The relative anchor-stability gap
  $\|\hat{b}_\gamma - \hat{b}_1\| / \|\hat{b}_1\|$ is taken over the **slope** coefficients only (departure
  hour, distance): an invariance claim is about effects, and the intercept lies in the span of the
  origin × month anchors, so it moves with γ by construction.
- **D4 (§8, correction and reporting).** (a) The Hoeffding radius first used the width 2 of $[-1, 1]$; each
  residual $y - p$ lies in $[-p, 1 - p]$ (width 1), so the radius is $\sqrt{\ln(2/\delta)/2n}$. This is a
  correctness fix, applied after the first run on `toronto_arrests.csv`; it changes no cell's verdict there.
  (b) That first run certified no violation, but its cells (30–523 units; radius 0.36 at the smallest) cannot resolve gaps of the
  tolerance's size, so "no violation" would have been read as "multicalibrated". Added a three-way verdict —
  `Violated`, `Certified` (every upper bound ≤ α), `Inconclusive` — without changing the violation rule,
  the cells, the bins, or α.
- **D5 (§4, reporting addition).** On the real carriers, the naive per-hypothesis peeking rule is also
  reported next to e-BH. It differs from e-BH in two ways at once — it reads the running maximum of each
  e-process (peeking) and applies no multiplicity correction — so the gap between them is not attributable
  to the K correction alone.
- **D6 (§4, reporting addition).** Benjamini-Yekutieli step-up on $p_k = \min(1, 1/e_k)$ is reported next to
  e-BH, to show the harmonic penalty $H_K$ that e-BH avoids under arbitrary dependence.
- **D7 (§4, design change after the adversarial audit; K = 2 for this instrument).** The pre-registered unit,
  the single flight with the pooled 2013 rate as a fixed reference, violates $\Phi_2$: flights of one day share
  weather and congestion, and the August–December season is not stationary. The independent audit found
  running e-values near $10^{30}$ for a carrier whose terminal e-value was $0.32$ (impossible under the stated
  null). Replaced by the paired calendar day: $X_{k,d} = r_{2023}(d) - r_{2013}(d)$ over the days carrier $k$
  flew in both years, in calendar order. First design (discarded, recorded): 7 rejected, 4 discarded, naive 10.
  Current design: 2 rejected (9E, HA), 9 discarded, naive 5, Benjamini-Yekutieli 1.
- **D8 (§5, estimator and attribution change after the adversarial audit).** The plug-in $W_1$ is biased
  upward (permutation null mean $0.1245$ SD on the NSW arms), so its bootstrap quantile is not a valid lower
  bound. The bound is now the 5% bootstrap quantile of $|\bar{y}_1 - \bar{y}_0| / \mathrm{sd}_0$, valid because
  $W_1 \ge |\mu_1 - \mu_0|$: $\varepsilon \ge 0.1133$ (was $0.1423$). The $\varepsilon$-sensitivity inequality is
  credited to Perdomo et al. (2020); Hardt et al. (2022) is the source for measuring power by experiment.
  The bound is in `re78` control-SD units for that action and replaces no card's $\varepsilon$.
- **D9 (§3, §5, §6, reporting completed after the audit).** Now printed by `labs --real-data`: the worst-cell
  test MSE and the pooled MSE for every γ of the grid; the RRM radius at the measured bound, using
  $\beta/\gamma = 1.5$ taken from `examples/credit/` (Assumed, not pre-registered); the largest CDF violation of
  the CORP-CRPS recalibration ($0.133$); the range of multicalibration cell sizes (30–523 units).
- **D10 (§2, process).** A list of deferred candidates was first inserted into §2 after the freeze; it is moved
  here, where post-freeze changes belong:

  Deferred to a later round (recorded from an independent plan reviewed on 2026-10-30, not implemented here):

  - **I:** PPI++ power-tuned prediction-powered inference (Angelopoulos, Duchi & Zrnic) — overlaps the
    existing `LlmJudgePpiRectifier`; a tuning extension, not a new pillar instrument.
  - **B:** reporting-delay audit with right-truncation inverse-probability weights, and conformal PID control
    (Angelopoulos, Candès & Tibshirani) as a successor of `AdaptiveConformalInference`.
  - **C:** omnipredictor dominance across Brier, log, and cost-loss scores on top of multicalibration.
  - **DAG:** knowledge-space fringes (Doignon & Falmagne) — inner and outer fringe of a reader's state on
    `docs/competencies.toml`, to diagnose which competency a failed proof points back to.
  - **Clay:** `DomainVocabulary` for non-English cards — belongs to phase 2 of the protocol profiles
    (`AGENTS.md` §II.D rule 5), which needs its own re-seal.
- **D11 (§3–§8, mechanism of the six new proofs).** None of the six new negative proofs mutates a card or
  injects a fault into the gate pipeline: each recomputes the detector the positive gate uses on the defect
  scenario (the distorted ensemble, the simulated null, the passive declaration, the simulated confounded
  model, the latest-vintage backtest, the simulated hidden bias) and exits 1 when it fires.
