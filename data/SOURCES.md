# Empirical Datasets of Stochology (`data/*.csv`)

Downloaded on 2026-10-14 and 2026-10-19 from the public Rdatasets mirror (`github.com/vincentarelbundock/Rdatasets`, commit `1dcc2bf5f955cc1224a3e1307256e1fe86b68dae`) and the CRAN mirror on GitHub (`github.com/cran/...`), and on 2026-10-30 from the Federal Reserve Bank of Philadelphia (§3b). SHA-256 digests for all 14 CSV files are stored in `data/SHA256SUMS` and checked at runtime by `stochology-cli` (`verify_sha256sums_manifest` in `crates/stochology-cli/src/loaders.rs`).

## 1. Core Benchmark Datasets (`data/*.csv`)

| File | Source | Description | Protocol Usage |
|---|---|---|---|
| `bank_calls.csv` | Rdatasets `fpp3::bank_calls` | Arrivals every 5 min at the call center of a large North American commercial bank, 3 Mar–24 Oct 2003, business days, 07:00–21:05 (164 days × 169 intervals = 27 716 rows) | Chapter 4 (`run_chapter_4`, `load_bank_call_clusters`, Campbell–Moulton temporal cohort $\mathrm{DEFF}$) |
| `djia_1885_2023.csv` | Rdatasets `stevedata::DJIA` | Daily close of the Dow Jones Industrial Average, 1885-02-16 to 2023-11-21 (37 931 trading days yielding 37 930 daily simple arithmetic returns $(S_t - S_{t-1})/S_{t-1}$; without dividends) | Chapter 6 (`run_chapter_6`, `load_djia_returns`, Itô variance drain & Baker–McHale epistemic Kelly fraction) |
| `nsw_experimental.csv` | Rdatasets `causaldata::nsw_mixtape` | National Supported Work (NSW) randomized job-training experiment (LaLonde 1986; Dehejia–Wahba sample): 185 treated and 260 randomized controls | Chapters 3 & 6 (`run_chapter_3`, `run_chapter_6`, `load_lalonde_nsw_cps`, two-arm Manski & Lee ATE bounds and positive-earner survivor bias) |
| `cps_observational.csv` | Rdatasets `causaldata::cps_mixtape` | Observational controls from the Current Population Survey (15 992 rows) compared against the NSW experimental arm | Chapters 3 & 4 (`run_chapter_3`, `run_chapter_4`, `load_lalonde_nsw_cps`, `load_cps_cohort_clusters`, observational selection bias & demographic cohort $\mathrm{DEFF}$) |
| `danish_fire.csv` | CRAN `fitdistrplus::danishuni` | Fire insurance claims of Copenhagen Re above 1M DKK, 1980-01-03 to 1990-12-31 (2 167 claims, in millions of 1985 DKK) | Chapter 5 (`run_chapter_5`, `load_danish_fire`, Hill tail estimator $\hat{\alpha}_{k,n}$, Jensen gap $\Delta_J$, and split-half CRPS) |

### Verified Primary References
- **Call-Center Arrivals (`bank_calls.csv`):**
  - Weinberg, J., Brown, L. D., & Stroud, J. R. (2007). Bayesian forecasting of an inhomogeneous Poisson process with applications to call center data. *Journal of the American Statistical Association*, 102(480), 1185–1198.
  - Shen, H., & Huang, J. Z. (2008). Interday forecasting and intraday updating of call center arrivals. *Manufacturing & Service Operations Management*, 10(3), 391–410.
- **National Supported Work & CPS Comparison (`nsw_experimental.csv`, `cps_observational.csv`):**
  - LaLonde, R. J. (1986). Evaluating the econometric evaluations of training programs with experimental data. *The American Economic Review*, 76(4), 604–620.
  - Dehejia, R. H., & Wahba, S. (1999). Causal effects in nonexperimental studies: Reevaluating the evaluation of training programs. *Journal of the American Statistical Association*, 94(448), 1053–1062.
- **Danish Fire Reinsurance Losses (`danish_fire.csv`):**
  - McNeil, A. J. (1997). Estimating the tails of loss severity distributions using extreme value theory. *ASTIN Bulletin: The Journal of the IAA*, 27(1), 117–137.
  - Embrechts, P., Klüppelberg, C., & Mikosch, T. (1997). *Modelling Extremal Events for Insurance and Finance*. Springer-Verlag, Berlin.

---

## 2. Operational Streams, Calibration, and Fairness Datasets

Downloaded on 2026-10-19 from Rdatasets (commit `1dcc2bf5f955cc1224a3e1307256e1fe86b68dae`) and the CRAN mirror of `nycflights23` (commit `48bca3116609b15fee8be2eb71b3f16383ca3ec6`, version 0.2.0, licence CC0). Consumed by `crates/stochology-cli/src/loaders.rs` and `crates/stochology-cli/src/chapters.rs`.

| File | Source | Description | Transformation | Protocol Usage |
|---|---|---|---|---|
| `nyc_flights_2013.csv` | Rdatasets `nycflights13::flights` | Departures from New York (EWR, JFK, LGA), Aug–Dec 2013 | Columns: `month, day, sched_dep_time, sched_arr_time, carrier, origin, dest, distance, arr_delay` (months in ASCII sort order `10,11,12,8,9`) | Chapter 5 (`run_chapter_5`, `load_nyc_flight_delays`, arrival delay heavy-tail regime) |
| `nyc_flights_2023.csv` | CRAN `nycflights23::flights` | Departures from New York (EWR, JFK, LGA), Aug–Dec 2023 | Same columns and calendar window | Chapter 5 (`run_chapter_5`, `load_nyc_flight_delays`, decadal shift arrival delay tail regime) |
| `vic_elec.csv` | Rdatasets `tsibbledata::vic_elec` | Half-hourly electricity demand of Victoria, Australia, 2012–2014 (52 608 rows), with temperature and holidays | None | Chapter 6 (`run_chapter_6`, `load_vic_elec_demand`, Kingman queue congestion & Canguilhem–Holling survival margin) |
| `atlantic_storms.csv` | Rdatasets `dplyr::storms` | NOAA HURDAT2 best-track positions, central pressures, and maximum sustained winds of Atlantic storms, 1975–2024 (20 778 rows across 693 storm tracks, yielding 20 085 consecutive same-storm pairs of which 19 025 are strict `+6h` transitions and 1 060 are non-6h synoptic steps) | Filtered to strict `+6h` (`(hour_next - hour_t) mod 24 == 6`) in `load_atlantic_storms_forecasts` | Chapter 7 (`run_chapter_7`, `load_atlantic_storms_forecasts`, `load_atlantic_storms_environments`, CORP-PAV isotonic calibration & multi-regime ICP) |
| `toronto_arrests.csv` | Rdatasets `carData::Arrests` | Arrests for simple possession of marijuana in Toronto, 1997–2002 (5 226 rows), with release decision, perceived colour, and covariates | None | Chapter 8 (`run_chapter_8`, `load_toronto_arrests_full_summary`, Chouldechova fairness trilemma audit) |

---

## 3. Out-of-Time (OOT) and Out-of-Domain (OOD) Flight Reserves

Extracted from the January–July slices of `nycflights13::flights` (commit `1dcc2bf5`) and `nycflights23::flights` (commit `48bca311`), plus `hflights::hflights` (commit `3f5a566f8b294da40e0abac2cfab4f7926eeb89d`, version 0.1, licence CC0). Consumed by `load_oot_ood_flight_delays` in `crates/stochology-cli/src/loaders.rs` and `run_chapter_5` in `crates/stochology-cli/src/chapters.rs`.

| File | Source | Description | Transformation |
|---|---|---|---|
| `nyc_flights_2013_h1.csv` | Rdatasets `nycflights13::flights` | Departures from New York, January to July 2013 (195 583 rows) | Same columns as `nyc_flights_2013.csv` |
| `nyc_flights_2023_h1.csv` | CRAN `nycflights23::flights` | Departures from New York, January to July 2023 (258 613 rows) | Same columns; out-of-time H1 slice |
| `houston_flights_2011.csv` | CRAN `hflights::hflights` | Departures from Houston (IAH, HOU), 2011 (224 591 rows with a departure time) | Same columns; scheduled departure and arrival recovered as actual time minus delay; cancelled flights without departure time excluded |

---

## 3b. Real-Time Macroeconomic Vintages

Downloaded on 2026-10-30 from the Federal Reserve Bank of Philadelphia, Real-Time Data Set for Macroeconomists (`https://www.philadelphiafed.org/-/media/frbp/assets/surveys-and-data/real-time-data/data-files/xlsx/routputqvqd.xlsx`, SHA-256 `25433da00f46764a167590efe5c3e1c20a3e91e98ba0f49e6f92f058bc42c9e3`, 243 161 bytes). Not an R package: the workbook is the primary distribution. Consumed by `crates/stochology-cli/src/loaders.rs` (`load_rtdsm_routput_vintages`) and `crates/stochology-cli/src/chapters.rs` (`run_chapter_4`).

| File | Source | Description | Transformation | Protocol Usage |
|---|---|---|---|---|
| `rtdsm_routput_vintages.csv` | Philadelphia Fed RTDSM `ROUTPUT` (quarterly observations, quarterly vintages) | Real GNP/GDP levels as published in each vintage, 244 vintages 1965Q4–2026Q3, observations from 1947Q1 (47 405 non-missing vintage × quarter values) | Wide matrix (rows `YYYY:Qq`, columns `ROUTPUTyyQq`) reshaped to long form `vintage,obs_quarter,routput` (`#N/A` cells dropped; levels to one decimal; rows sorted). Levels keep each vintage's own base year, so growth is computed inside a vintage | Chapter 4 (`run_chapter_4`, `EpistemicMarginState::from_vintages`) |

Reference: Croushore, D. & Stark, T. (2001). A real-time data set for macroeconomists. *Journal of Econometrics*, 105(1), 111–130.

---

## 3d. Pending national air-transport archive (not sealed)

The Brazilian air-transport statistical series published by the National Civil Aviation Agency (ANAC) is
**not** part of the sealed manifest in this edition. The reason is contractual rather than logistical: a
dataset enters `data/SHA256SUMS` only with a pinned digest, a documented retrieval path, a schema contract
in `crates/stochology-cli/src/loaders.rs`, and explicit human approval of the manifest change.

The retrieval half is implemented in `scripts/anac_fetch.py`, which

1. requests the public distribution with an explicit browser `User-Agent` (the portal answers `403` to a bare `curl`),
2. refuses payloads that are HTML or that lack the required columns (`empresa`, `aeroporto_origem`, `aeroporto_destino`, `partida_prevista`, `partida_real`, `situacao_voo`),
3. prints the SHA-256 of a candidate file for review, and
4. refuses to write an output file until an approved digest is pinned in the script itself.

[Assumed] Until those four conditions are met, every quantitative claim about the Brazilian mesh in
Chapter 46 §5.6 remains `[Literature]`, and the measured tail statistics of Laboratory 5 continue to come
from the four sealed North American windows.

## 4. Upstream Package Licences and Data Provenance

The files in this directory are copies of public research and government datasets redistributed by R packages, except `rtdsm_routput_vintages.csv`, which is converted from the publisher's own workbook (§3b). The repository's own MIT licence covers the Rust source code of this workspace.

| File | Package (licence field) | Original source as documented by the package |
|---|---|---|
| `bank_calls.csv` | `fpp3` (GPL-3) | Weinberg, Brown & Stroud (2007), call-centre data of a North American bank, as used by Shen & Huang (2008) |
| `djia_1885_2023.csv` | `stevedata` (GPL-2) | Daily closes of the Dow Jones Industrial Average, compiled from public market quotes |
| `nsw_experimental.csv`, `cps_observational.csv` | `causaldata` (MIT) | LaLonde (1986) and Dehejia & Wahba (1999), public research data |
| `danish_fire.csv` | `fitdistrplus` (GPL ≥ 2) | Copenhagen Re fire losses, published by McNeil (1997) |
| `nyc_flights_2013.csv`, `nyc_flights_2013_h1.csv` | `nycflights13` (CC0) | US Bureau of Transportation Statistics, public records |
| `nyc_flights_2023.csv`, `nyc_flights_2023_h1.csv` | `nycflights23` (CC0) | US Bureau of Transportation Statistics, public records |
| `houston_flights_2011.csv` | `hflights` (CC0) | US Bureau of Transportation Statistics, public records |
| `vic_elec.csv` | `tsibbledata` (GPL-3) | Australian Energy Market Operator (AEMO), public half-hourly demand |
| `atlantic_storms.csv` | `dplyr` (MIT) | NOAA Atlantic hurricane database (HURDAT2), public records |
| `toronto_arrests.csv` | `carData` (GPL ≥ 2) | Toronto Star freedom-of-information request, published by Fox & Weisberg |
| `rtdsm_routput_vintages.csv` | none (direct download) | Federal Reserve Bank of Philadelphia, Real-Time Data Set for Macroeconomists; public data, cited as Croushore & Stark (2001) |
