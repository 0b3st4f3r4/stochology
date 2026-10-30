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
