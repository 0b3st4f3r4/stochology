# 12. Simulated Worlds: Null Models, Seeds, Multi-Seed Acceptance

> **Three tempos.** **Detachment:** set aside trusting a single lucky run. **Abstraction:** a model returns to the world first as simulated worlds whose chance must be reproducible and tested across many seeds (Model Worlds, $L\_2 \to L\_1$, reading frame). **Return:** `SplitMix64` substreams, executable null models, and multi-seed acceptance. **Negative proof:** none in this chapter; the tools here feed the proofs of later chapters.

## Why Every Model Must First Build a Counterfactual World

Before an aircraft wing enters a wind tunnel or a clinical trial enrolls its first patient, an engineer asks a generative question on **Pair $L\_2 \to L\_1$ (*Model Worlds*)**, a reverse pair that serves as a reading frame over existing code (seeded null models, `SplitMix64`, and multi-seed acceptance) rather than a new bridge: *If my model's assumptions were true, what kind of synthetic data would it generate, and could my estimation pipeline recover the known parameters I injected?* Without a deterministic, reproducible random number generator (`seed = 2026`, `SplitMix64`), a pre-registered generative null model, and a multi-seed acceptance protocol that separates development seeds from frozen confirmation seeds, simulation easily degenerates into cherry-picking a lucky random seed.

## 12.1 Invariant $\Phi\_1$ — Simulability and the Generative Null Model

Every Epistemological Card declares both `seed_and_environment` (`seed = 2026`, `SplitMix64`) and an executable `null_model` *before* fitting parameters for three reasons:

1. **Defeating Denominator Illusions:** As the Miller-Sanjurjo hot-hand discovery proved, intuitive mental guesses about "what pure chance looks like" under complex stopping rules, sliding windows, or cluster structures are frequently biased. Writing an executable null generator in code and running it with `seed = 2026` reveals the true distribution of the test statistic when the claim is false.
2. **Synthetic Injection Recovery:** Before trusting an estimator on messy real data where the truth is unknown, $\Phi\_1$ requires knowing that the estimator can recover a known parameter $\theta\_{\text{true}}$ injected into simulated data (formalized via Simulation-Based Calibration in Chapters 8 and 12).
3. **Lock-Free Splittable Streams for Parallel Fork-Join Monte Carlo (`SplitMix64::split` & `MonteCarloSplitTreeAudit`):** When a simulation branches across parallel Monte Carlo trees or isolated subagents, sharing a mutable RNG state introduces lock contention and execution-order race conditions, while naive seed offsets (`seed + i`) risk inter-stream correlation. Following **Guy Steele, Doug Lea, and Christine Flood (OOPSLA 2014)** and **Guy Steele & Sebastiano Vigna (OOPSLA 2021, LXM)**, `SplitMix64::split(stream_id)` and `split_fork_join_pair(branch_tag)` derive deterministic, lock-free child streams by hashing the parent state mixed with `stream_id` via a Stafford Mix13-style avalanche finalizer (shifts 30/27/31) and adding a distinct odd increment $\gamma = (\text{stream\_id} \cdot c\_1 \oplus c\_2) \mid 1$ built from two fixed 64-bit constants. Audited in `MonteCarloSplitTreeAudit::evaluate_canonical(2026)`, 16 parallel fork-join streams of $1\text{,}024$ draws each exhibit zero state collisions, maximum pairwise correlation $< 0.12$, and bit-for-bit order-independent reproducibility (`all_checks_passed = true`).

---

## 12.2 Filter 3 — Simulation-Based Calibration (SBC) and SHELF Prior Elicitation

Even with clean data and a sound causal graph, a Bayesian or probabilistic estimation algorithm can still harbor a coding bug or biased approximation.

In 2018, **Sean Talts, Michael Betancourt, Daniel Simpson, Aki Vehtari, and Andrew Gelman** formalized **Simulation-Based Calibration (SBC)** around a self-consistency identity of probability theory:
1. Draw a true parameter $\tilde{\theta} \sim \pi(\theta)$ from the prior distribution.
2. Simulate a synthetic dataset $\tilde{y} \sim p(y \mid \tilde{\theta})$ using the generative model ($\Phi\_1$).
3. Run your inference algorithm on $\tilde{y}$ to draw $M$ posterior samples $\theta\_1, \dots, \theta\_M \sim p(\theta \mid \tilde{y})$.
4. Count the **rank statistic** $r(\lbrace \theta\_m \rbrace, \tilde{\theta}) = \sum\_{m=1}^M \mathbb{I}\lbrace \theta\_m < \tilde{\theta} \rbrace \in \lbrace 0, 1, \dots, M \rbrace$.

**Theorem (Talts et al., 2018):** *If the posterior inference algorithm is mathematically exact, the rank statistic $r$ is distributed **strictly uniformly** across $\lbrace 0, 1, \dots, M \rbrace$.*
- If your algorithm is **overconfident** (posterior intervals too narrow), the true $\tilde{\theta}$ falls in the extremes too often, creating a **$\cup$-shaped** rank histogram.
- If your algorithm is **underconfident** (intervals too wide), the rank histogram heaps up in the middle ($\cap$-shaped).
- If your algorithm is **biased**, the rank histogram tilts to the left or right.

Filter 3 bins the SBC ranks across $B$ bins and computes Pearson's $\chi^2 = \sum\_{b=1}^B \frac{(O\_b - E\_b)^2}{E\_b}$, requiring $\chi^2 \le \chi^2\_{\text{crit}}$. Simultaneously, Filter 3 validates expert prior elicitation under **Anthony O'Hagan's Sheffield Elicitation Framework (SHELF)**, requiring strict physical ordering of the elicited quartiles:

$$L\_{\text{phys}} < q\_{25} < q\_{50} < q\_{75} < U\_{\text{phys}}$$

Once the elicited quartiles $(q\_{25}, q\_{50}, q\_{75})$ are verified, `fit_shelf_beta_prior` (`crates/stochology-siga/src/funnel/`) fits a continuous conjugate **$\mathrm{Beta}(\hat{\alpha}, \hat{\beta})$ prior** by minimizing the cumulative distribution RMSE $\sqrt{\frac{1}{3}\bigl[(I\_{q\_{25}}(\alpha, \beta) - 0.25)^2 + (I\_{q\_{50}}(\alpha, \beta) - 0.50)^2 + (I\_{q\_{75}}(\alpha, \beta) - 0.75)^2\bigr]}$, where the regularized incomplete Beta function $I\_x(\alpha, \beta)$ is evaluated to machine precision in pure Rust via **Lentz's (1976) continued-fraction algorithm** (`regularized_incomplete_beta`) and **Lanczos's $g=7$ log-Gamma approximation** (`ln_gamma_lanczos`).

### Step-by-Step Foundational Derivation: Lentz's Continued Fraction for the Regularized Incomplete Beta Function $I\_x(\alpha, \beta)$

How does Filter 3 fit a conjugate $\mathrm{Beta}(\alpha, \beta)$ distribution to expert SHELF quartiles $(q\_{25}, q\_{50}, q\_{75})$ in pure Rust without external special-function libraries?
1. **Integral Definition and Symmetry Reduction:**
   The cumulative distribution function of $\mathrm{Beta}(\alpha, \beta)$ at $x \in (0, 1)$ is the regularized incomplete Beta function:
   $$I\_x(\alpha, \beta) = \frac{B(x; \alpha, \beta)}{B(\alpha, \beta)} = \frac{1}{B(\alpha, \beta)} \int\_0^x t^{\alpha-1}(1-t)^{\beta-1}\,dt$$
   where $\ln B(\alpha, \beta) = \ln\Gamma(\alpha) + \ln\Gamma(\beta) - \ln\Gamma(\alpha+\beta)$ is evaluated via Lanczos's $g=7$ Chebyshev rational approximation (`ln_gamma_lanczos`). Whenever $x > \frac{\alpha + 1}{\alpha + \beta + 2}$, we apply the exact reflection identity $I\_x(\alpha, \beta) = 1 - I\_{1-x}(\beta, \alpha)$ so the continued fraction always converges rapidly in the tail.
2. **Continued-Fraction Expansion and Lentz's (1976) Modified Algorithm:**
   When $x \le \frac{\alpha + 1}{\alpha + \beta + 2}$, $I\_x(\alpha, \beta)$ admits the classical continued fraction:
   $$I\_x(\alpha, \beta) = \frac{x^\alpha (1-x)^\beta}{\alpha B(\alpha, \beta)} \left(\frac{1}{1 + \frac{d\_1}{1 + \frac{d\_2}{1 + \cdots}}}\right)$$
   with even and odd coefficients:
   $$d\_{2m} = \frac{m(\beta - m)x}{(\alpha + 2m - 1)(\alpha + 2m)}, \qquad d\_{2m+1} = -\frac{(\alpha + m)(\alpha + \beta + m)x}{(\alpha + 2m)(\alpha + 2m + 1)}$$
   In `regularized_incomplete_beta`, Lentz's recurrence maintains two forward ratios $C\_k = 1 + \frac{d\_k}{C\_{k-1}}$ and $D\_k = \frac{1}{1 + d\_k D\_{k-1}}$ (clamping denominators away from zero at $10^{-30}$), multiplying $f\_k = f\_{k-1} C\_k D\_k$ until $|\Delta\_k - 1| = |C\_k D\_k - 1| < 10^{-14}$.

---

## 12.3 Minsky & Papert's Geometric Ceiling, *Society of Mind* Heterogeneous Ensembles, and Multi-Seed Acceptance (`StrategicAiAccommodationSynthesis`)

In `crates/stochology-lenses/src/strategic/`, the **Strategic AI Six-Pair Accommodation Synthesis** (`StrategicAiAccommodationSynthesis`) gathers six simulated audits ("axes") and one real-data coupler. The code's module comment assigns each axis to a pair; the book follows that assignment and files the axes in the chapters where their tools are taught (Axis 2 in [Chapter 11](11-world-models-discrepancy-authentic-form.md), Axis 3 in [Chapter 15](15-decision-models-curvature-scoring.md)), and both assignments are authorial readings, not mechanisms the code enforces. This chapter holds Axis 1 and the multi-seed acceptance that judges all six. Axis 1 (`MinskyPapertGeometrySocietyOfMindAudit`, read here under Pair $L\_2 \to L\_1$) restates three results of Marvin Minsky and Seymour Papert (*Perceptrons*, 1969) and Marvin Minsky (*The Society of Mind*, 1986) as checks run across the multi-seed acceptance protocol (`stochology_lenses::acceptance`, 20 development seeds `2026..=2045` and 200 confirmation seeds `9_000_000..=9_000_199`):
1. **Order of `PARITY` and `XOR` (deterministic identities):** On $\lbrace -1, +1\rbrace^4$, the parity function has zero inner product with every Walsh monomial of degree below 4 (`Walsh_corr=0.0e0`). On `XOR`, the best order-1 threshold $\mathrm{sign}(w\_0 x\_0 + w\_1 x\_1 + b)$ over a small grid of weights classifies $3/4 = 0.75$ of the four points, while the order-2 product feature $x\_0 x\_1$ classifies $4/4$. Both are exact checks folded into the axis identities; no network is trained.
2. **Diameter-Limited vs. Global Connectivity:** On 120 simulated 10-cell strips, half holding one block of four active cells and half holding two separate blocks with the same active mass, a diameter-limited receptor that pools local mass (plus noise) scores $0.51$, while global reachability by graph search scores $1.00$ (`Conn_acc=1.00 vs 0.51`, simulated, `seed = 2026`). The gated claim is that the deployed accuracy exceeds $0.90$.
3. ***Society of Mind* Committee vs. Correlated Voters:** Five voters forecast 300 simulated binary events. In the homogeneous arm their errors share a correlation $\rho = 0.85$ and every voter also carries an assumed, hand-set framing bias of $0.28$ toward $0.5$; in the heterogeneous arm the correlation is $\rho = 0.05$ and there is no bias. The committee Brier scores are $0.110$ versus $0.268$ (`SoM_Brier=0.110 vs 0.268`, simulated, `seed = 2026`). Because the homogeneous arm carries both the correlation and the assumed bias, the gap cannot be credited to correlation alone. The axis also checks Breiman's correlated-ensemble identity $\mathrm{Var}(\bar{\varepsilon}\_M) = \sigma^2\bigl(\rho + \frac{1-\rho}{M}\bigr)$ against an explicit covariance sum. The gated claim is a deployed Brier score below $0.195$.

`G8` accepts the strategic synthesis only through its named claims. The current `gates` run prints:

~~~text
[G8] strategic: 15 gated claims (11 seed-dependent, 4 deterministic) pass on >= 95% of 20 development and 200 sealed confirmation seeds
[G8] strategic: lowest one-sided 95% Clopper-Pearson bound on a confirmation pass rate is 0.977 (strategic.a2.deployed_imputation_variance_retention_above_0.80)
[G8] strategic discarded claim (recorded, not approved): strategic.a2.parametric_mar_mi_covers_true_mean_under_mnar passes on 0.0% of confirmation seeds
[G8] strategic discarded claim (recorded, not approved): strategic.a3.llm_judge_deployed_abs_error_below_0.06 passes on 88.0% of confirmation seeds
~~~

The two discarded claims are refuted, not approved; [Chapter 10](10-world-models-missingness-bounds.md) explains what the first one actually tests and [Chapter 15](15-decision-models-curvature-scoring.md) explains why the second fails. The same run also records a discarded real-data finding: uniform 4-bit clipping on `danish_fire.csv` does not preserve $\mathrm{ES}\_{0.99}$ within $10\%$.

### Negative Proof: The Naive Arm of Every Axis (`--defect strategic_accommodation`)

Each strategic axis is computed twice: in shielded mode with the method the protocol recommends, and with `shielded = false`, where the naive method is deployed instead. The negative proof `strategic_accommodation` runs the naive arm and blocks (exit code `1`) only if three conditions hold together: the axis identities still close on every development seed (the arithmetic is sound in both modes), on each of the 20 development seeds at least one seed-dependent claim fails, and at `seed = 2026` all six axes fail verification. Its current output is a single line:

~~~bash
cargo run --release --bin gates -- --defect strategic_accommodation
~~~

~~~text
NEGATIVE PROOF [strategic_accommodation] BLOCKED SUCCESSFULLY on all 20 development seeds through seed-dependent claims (identities closed on every seed): at seed 2026 diameter-limited connectivity acc=0.51, homogeneous voter Brier=0.268, Rung-1 symptom SHAP do-MSE=0.14, single-mean imputation var retention=0.62, naive i.i.d. clustered power=0.17, uncalibrated LLM-Judge err=0.199, clipped INT4 ES99 retention=0.17, unbuffered hub survival=0.00, rank-reversed AHP portfolio val=11.80, L2 Kalman glitch RMSE=1.47, certainty-equivalent MPC survival=0.59
~~~

All eleven values are simulated at `seed = 2026`, and each is the naive counterpart of a shielded figure in the `labs` line for the synthesis. Read two of them with care. The uncalibrated judge error $0.199$ is a naive baseline; the matching shielded claim (`PPI` error below $0.06$) is itself discarded, so the pair does not show that the corrected judge is accurate enough. The rank-reversed `AHP` portfolio value $11.80$ is constructed: the code hard-codes the reversed choice in the naive arm, so this entry illustrates rank reversal rather than detecting it.

---

## Computational Audit in Rust

Lock-free parallel Monte Carlo stream splitting (`SplitMix64::split(stream_id)` and `MonteCarloSplitTreeAudit`, reported alongside $\Phi\_2$ in [Chapter 20](20-decision-worlds-regime.md)), SHELF conjugate Beta prior fitting (`fit_shelf_beta_prior`, reported in [Chapter 16](16-decision-models-thresholds-evidence.md)), and multi-seed synthesis acceptance (`stochology_lenses::acceptance` and `StrategicAiAccommodationSynthesis` Axis 1) are executed across Laboratory 3, Laboratory 4, Laboratory 5, and Gate `G8`:

~~~bash
cargo run --release --bin labs -- --invariants
cargo run --release --bin labs -- --siga
cargo run --release --bin gates
~~~

Use the interactive Strategic Accommodation laboratory below to explore how prospective sample-size planning, causal attribution, and rank-stability invariants interact across seeds:

<div class="stoch-lab" data-lab="strategic-shap-ppi-ahp" data-chapter="12-model-worlds-simulation-acceptance.html" data-title="Chapter 12 Lab: Multi-Seed Strategic Accommodation, Prospective Gold Sizing &amp; Rank Invariance" data-cli="cargo run --release --bin gates -- --defect strategic_accommodation"></div>

## Exercises

1. **(Concept — Simulation-Based Calibration)** Suppose you run SBC on a new Bayesian inference algorithm and observe that out of $1\text{,}000$ synthetic runs, the true parameter $\tilde{\theta}$ falls below the $10\text{th}$ percentile of the posterior $250$ times and above the $90\text{th}$ percentile $260$ times. Is your algorithm overconfident, underconfident, or unbiased?

<details>
<summary><strong>Analytical &amp; Computational Solutions (Click to Expand)</strong></summary>

1. **Simulation-Based Calibration:**
   In an exact posterior, $\tilde{\theta}$ should fall below the $10\text{th}$ percentile $10\%$ of the time ($100$ times) and above the $90\text{th}$ percentile $10\%$ of the time ($100$ times). Falling in the two $10\%$ tails $250 + 260 = 510$ times ($51\%$ vs. $20\%$) produces a **$\cup$-shaped rank histogram**, proving that the posterior intervals are **severely overconfident (too narrow)**.

</details>

## Summary

- **$\Phi\_1$ (Simulability):** Every claim requires `seed = 2026` (`SplitMix64`) and an executable generative null model.
