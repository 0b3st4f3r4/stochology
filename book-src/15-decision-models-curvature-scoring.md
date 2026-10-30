# 15. Curvature and Scoring

> **Three tempos.** **Detachment:** set aside summarizing uncertainty by the expected mean. **Abstraction:** the map from world state to harm is curved, and catastrophe lives in the curvature (Decision Models, $L\_2 \to L\_3$). **Return:** Jensen's gap, CRPS, strictly proper scores, and the Cromwell floor. **Negative proof:** `gates -- --defect cromwell` (exit `1`).

## Why the Average Day Understates the Call Center's Cost

Suppose a retail bank staffs its customer-service center for the "average business day" of $\mathbb{E}[X] = 32\text{,}664$ calls (`bank_calls.csv`). Suppose further, as an illustration, that the daily cost of congestion grows like the square of call volume, $f(x) = x^2$ (an assumed loss; the repository fits no queueing or abandonment model). Then the extra cost of a hypothetical $38\text{,}000$-call day is not cancelled by the saving on a $28\text{,}000$-call day. On the measured series the resulting Jensen's Gap is modest, $\Delta\_J \approx 0.85\%$ of $f(\mathbb{E}[X])$, but under curvature it is never zero, and its size depends on how sharply the true cost bends. Across **Bridge $L\_2 \to L\_3$ (*Decision Models*)**, compressing a full predictive distribution into a single point mean hides both **Jensen's curvature tax ($\Delta\_J$)** and distributional miscalibration.

## 15.1 Bridge $L\_2 \to L\_3$: Nonlinearity, Jensen's Gap, and $\mathrm{CRPS}$

### Foundations: Convexity and Why the Average Input Does Not Give the Average Output

Once data have crossed into the Model ($L\_2$), a second trap awaits on Bridge $L\_2 \to L\_3$: **reading the expected value $\mathbb{E}[X]$ as if it were the only number the decision-maker ($L\_3$) needs.**

Draw the curve $f(x) = x^2$ on a sheet of paper. It is a bowl opening upward, a **convex** function. Pick two equally likely outcomes: $x\_1 = 0$ and $x\_2 = 2$.
- The average of the inputs is $\mathbb{E}[X] = \frac{0 + 2}{2} = 1$.
- Plugging that average input into the formula gives $f(\mathbb{E}[X]) = 1^2 = 1$.
- The actual average of the outputs is $\mathbb{E}[f(X)] = \frac{0^2 + 2^2}{2} = \frac{0 + 4}{2} = 2$.

The average output $\mathbb{E}[f(X)] = 2$ is twice as large as the output at the average input $f(\mathbb{E}[X]) = 1$. Geometrically, for any convex function (a curve whose second derivative is positive, $f''(x) > 0$), the straight chord connecting any two points on the curve lies strictly **above** the curve itself.

This theorem is **Jensen's Inequality** (Johan Jensen, 1906): for any random variable $X$ and any convex function $f$,

$$\mathbb{E}[f(X)] \ge f(\mathbb{E}[X])$$

We define the exact difference as **Jensen's Gap ($\Delta\_J$)**:

$$\Delta\_J \triangleq \mathbb{E}[f(X)] - f(\mathbb{E}[X])$$

To see *why* $\Delta\_J$ grows when the world becomes more volatile, expand $f(X)$ around the mean $\mu = \mathbb{E}[X]$ using a second-order Taylor series:

$$f(X) \approx f(\mu) + f'(\mu)(X - \mu) + \frac{1}{2}f''(\mu)(X - \mu)^2$$

Taking the expectation of both sides (and noting that $\mathbb{E}[X - \mu] = 0$ while $\mathbb{E}[(X - \mu)^2] = \mathrm{Var}(X)$) yields the **Second-Order Variance Toll**:

$$\Delta\_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X]) \approx \frac{1}{2}  f''(\mu)  \mathrm{Var}(X)$$

Read that equation in plain English:

> **Whenever a cost, waiting time, or damage function bends upward ($f''(\mu) > 0$), variability does not cancel out over time; it adds a positive cost equal to half the curvature times the variance.**

In `bank_calls.csv` ($164$ business days of a North American retail bank's call center), daily call volume fluctuates across days. Under a quadratic staffing congestion penalty, evaluating the cost at the mean daily volume gives $f(\mathbb{E}[X]) = 1.0669 \times 10^9$, whereas the true average daily cost is $\mathbb{E}[f(X)] = 1.0760 \times 10^9$, leaving a **Jensen's Gap of $\Delta\_J = +9\text{,}033\text{,}837.19$** that a spreadsheet using only the average day completely erases. The penalty $f(x) = x^2$ is an illustrative convex cost measured in squared daily calls, not currency, and the gap is $\approx 0.85\%$ of $f(\mathbb{E}[X])$: small for these fairly stable days, and growing with volatility and convexity, which is why it must be reported rather than assumed.

### Evaluating Full Predictive Distributions: The Continuous Ranked Probability Score ($\mathrm{CRPS}$)

Because a single mean hides both Jensen's Gap and tail risk, Prohibition 2 requires every model on Bridge $L\_2 \to L\_3$ to report not only $f(\mathbb{E}[X])$, $\mathbb{E}[f(X)]$, and $\Delta\_J$, but also the **median**, the **5th and 95th percentiles ($q\_{05}, q\_{95}$)**, and the **Continuous Ranked Probability Score ($\mathrm{CRPS}$)**.

Suppose a model issues a full predictive cumulative distribution function (CDF) $F(x) = P(X \le x)$, and reality subsequently materializes at the actual number $y$. A single point metric like Root Mean Squared Error ($\mathrm{RMSE}$) ignores whether the model's uncertainty bands were honest or overconfident. The **$\mathrm{CRPS}$** scores the *entire predictive distribution* at once by integrating the squared distance between the forecast CDF $F(x)$ and the empirical step function $\mathbb{I}\lbrace x \ge y \rbrace$ (which is $0$ below $y$ and jumps to $1$ at $y$):

$$\mathrm{CRPS}(F, y) \triangleq \int\_{-\infty}^{\infty} \bigl(F(x) - \mathbb{I}\lbrace x \ge y \rbrace\bigr)^2 dx = \mathbb{E}\_F|X - y| - \frac{1}{2}\mathbb{E}\_F|X - X'|$$

where $X$ and $X'$ are independent draws from the predictive distribution $F$. The second expression (`NonlinearSummary` in `crates/stochology-core/src/layers/`) has two terms:
- The first term, $\mathbb{E}\_F|X - y|$, rewards the distribution for placing its mass close to the realized truth $y$ (*accuracy*).
- The second term, $-\frac{1}{2}\mathbb{E}\_F|X - X'|$, rewards sharp, well-calibrated spread.
- $\mathrm{CRPS}$ is a **strictly proper scoring rule**: a forecaster minimizes expected $\mathrm{CRPS}$ if and only if they report their true predictive distribution $F$, with zero incentive to hedge or exaggerate confidence.
- **Exact $O(n \log n)$ Order-Statistic Computation (`exact_loo_empirical_crps`):** Naively evaluating leave-one-out empirical $\mathrm{CRPS}$ across $n$ observations requires $O(n^2)$ pairwise comparisons. By sorting the sample $x\_{(1)} \le x\_{(2)} \le \dots \le x\_{(n)}$ once in $O(n \log n)$ time and using prefix sums together with the L-moment order-statistic identity $\sum\_{1 \le i < j \le n} (x\_{(j)} - x\_{(i)}) = \sum\_{i=1}^n (2i - 1 - n)x\_{(i)}$, `NonlinearSummary::evaluate` computes the exact leave-one-out empirical $\mathrm{CRPS}$ in $O(n \log n)$ time.

---

## 15.2 Prohibition 2 — Ban on the Lone Mean (`PROHIBITION_2`)

### Why the Mean Alone Is a Lie Under Curvature, Compounding, or Heavy Tails

In elementary courses, students are taught to summarize a random variable $X$ by its sample mean $\bar{x}$. That habit is harmless only in a linear, thin-tailed, additive Gaussian world. It becomes a dangerous distortion whenever any of three conditions holds:
1. **The payoff or loss function $f(X)$ is nonlinear** (Bridge $L\_2 \to L\_3$, Chapter 15): by Jensen's Inequality, $f(\mathbb{E}[X]) \neq \mathbb{E}[f(X)]$.
2. **The dynamics are multiplicative over time** (Lens 2, Chapter 9): by Itô's variance drain, the ensemble mean growth rate $\mu$ overstates the compounded time-average growth rate $\mu - \frac{1}{2}\sigma^2$.
3. **The upper tail is heavy** (Lens 3, Chapter 9): when the Pareto tail index satisfies $\alpha \le 2$, the population variance is infinite, and the mean is dominated by rare, violent jumps.

### The Seven Mandatory Quantities of `NonlinearSummary`

Under **Prohibition 2**, reporting only $\mathbb{E}[X]$ or $f(\mathbb{E}[X])$ is strictly forbidden. Every Epistemological Card (`epistemological_header.nonlinearity`) and every laboratory summary (`NonlinearSummary` in `crates/stochology-core/src/layers/`) must report **all seven numbers together**:

$$\bigl(  f(\mathbb{E}[X]),\quad \mathbb{E}[f(X)],\quad \Delta\_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X]),\quad \text{median},\quad q\_{05},\quad q\_{95},\quad \mathrm{CRPS}  \bigr)$$

Each of these seven numbers answers a distinct operational question:

| Mandatory Field | Symbol | Operational Question It Answers |
| --- | --- | --- |
| `f_at_mean` | $f(\mathbb{E}[X])$ | What would the cost/payoff be if the world froze at its average input every single day? |
| `expected_f` | $\mathbb{E}[f(X)]$ | What is the true expected cost/payoff once daily fluctuations pass through the curved function $f$? |
| `jensen_gap` | $\Delta\_J$ | How much extra tax ($\mathbb{E}[f(X)] - f(\mathbb{E}[X])$) is volatility charging us because of curvature? |
| `median_f` | $q\_{50}$ | What does a *typical* realization look like, uncorrupted by extreme tail spikes? |
| `q05_f` & `q95_f` | $q\_{05}, q\_{95}$ | What is the $90\%$ central inter-quantile range spanning calm days to surge days? |
| `empirical_crps` | $\mathrm{CRPS}$ | How well does the entire predictive distribution $F(x)$ score out-of-sample against realized outcomes $y$? |

The field names above are those of `NonlinearSummary`. A card carries the same seven quantities as text in its `nonlinearity` string, where `EpistemologicalCard::audit()` requires the keywords `f(E[X])`, `E[f(X)]`, `Jensen`, `median`, `q05`, `q95`, and `CRPS`, parses the numbers after them, and checks a non-zero gap with closure $|\Delta\_J - (\mathbb{E}[f(X)] - f(\mathbb{E}[X]))| \le \max(0.5,\ 10^{-3}\,|\mathbb{E}[f(X)]|)$, the ordering $q\_{05} < \text{median} < q\_{95}$, and $0 < \mathrm{CRPS} < q\_{95} - q\_{05}$.

---

## 15.3 Prohibition 4 — Ban on Dogmatic Extremes and Omitted Attempts (`PROHIBITION_4`)

Prohibition 4 guards against two of the most common sins in quantitative research: **dogmatic certainty ($0\%$ or $100\%$)** and **unreported cherry-picking ($p$-hacking)**.

### Part A: Cromwell's Rule ($p \in [\varepsilon\_{\text{floor}}, 1 - \varepsilon\_{\text{floor}}]$)

In August 1650, Oliver Cromwell wrote to the General Assembly of the Church of Scotland:

> *"I beseech you, in the bowels of Christ, think it possible that you may be mistaken."*

In the 1980s, Bayesian statistician **Dennis Lindley** named **Cromwell's Rule** after this plea. Look back at Bayes' theorem from the Preface:

$$P(H \mid E) = \frac{P(E \mid H)  P(H)}{P(E \mid H)  P(H) + P(E \mid \neg H)  (1 - P(H))}$$

If an agent sets a prior probability to $P(H) = 0$ (or $P(H) = 1$), no amount of empirical evidence $E$ (not even a billion black swans marching past the window) can move the posterior: multiplying by $P(H) = 0$ keeps $P(H \mid E)$ **frozen at $0$ forever**. Furthermore, under the logarithmic scoring rule $-\ln p$, assigning $p = 0$ to an event that subsequently occurs inflicts an **infinite penalty ($\infty$)**.

Therefore, Prohibition 4 bans probabilities of $0\%$ and $100\%$ across the entire protocol. Every declared probability $p$ must lie strictly inside the **Cromwell Corridor** bounded by a declared base-rate floor $\varepsilon\_{\text{floor}} \in (0, 0.05]$:

$$\varepsilon\_{\text{floor}} \le p \le 1 - \varepsilon\_{\text{floor}}$$

If any card or module attempts to declare $p = 0.0$, $p = 1.0$, or $p$ outside $[\varepsilon\_{\text{floor}}, 1 - \varepsilon\_{\text{floor}}]$, Gate `G6` blocks execution with exit code `1` (`--defect cromwell`).

### Step-by-Step Foundational Derivation: Cromwell's Log-Score Bound and the Multiplicity Tax

Why must Cromwell's floor $\varepsilon\_{\text{floor}} \in (0, 0.05]$ be enforced *both* in Bayesian updating and in out-of-sample forecast evaluation?
1. **Bounded Worst-Case Surprise Under the Logarithmic Scoring Rule:**
   Under the strictly proper logarithmic score $S\_{\text{log}}(p, y) = -\bigl[y\ln p + (1-y)\ln(1-p)\bigr]$, a single unpredicted tail event ($y = 1$) against a dogmatic forecast $p \to 0^+$ drives the cumulative loss to $+\infty$, destroying the empirical average across all other $n-1$ observations. Clamping $p \in [\varepsilon\_{\text{floor}}, 1 - \varepsilon\_{\text{floor}}]$ via `apply_cromwell_floor(p, eps)` bounds the maximum single-observation log-loss by:
   $$\sup\_{y \in \lbrace 0, 1\rbrace} S\_{\text{log}}\bigl(\mathrm{clamp}(p, \varepsilon\_{\text{floor}}), y\bigr) \le \ln\left(\frac{1}{\varepsilon\_{\text{floor}}}\right)$$
   At $\varepsilon\_{\text{floor}} = 0.005$, the worst-case surprise per observation is bounded at $\ln(200) \approx 5.2983\text{ nats}$ ($\approx 23.01\text{ decibans}$), ensuring finite empirical variance for sequential *e-processes* (the companion union-bound multiplicity derivation for $K \ge 1$ attempted specifications is presented in [Chapter 17](17-model-decisions-card-preregistration.md)).

---

## Interactive Laboratory (`/lab`)

Explore how cluster correlation ([Chapter 20](20-decision-worlds-regime.md)), Cromwell bounds, and multi-attempt penalties ([Chapter 17](17-model-decisions-card-preregistration.md)) interact with the protocol's boundary gates:

<div class="stoch-lab"
     data-lab="invariants-moulton-chouldechova"
     data-chapter="15-decision-models-curvature-scoring.html"
     data-title="Prohibitions &amp; Boundary Invariant Stress-Test"
     data-subtitle="Audit cluster inflation, base-rate odds locks, and gate thresholds before any claim reaches production">
</div>

---

## Computational Audit in Rust

Run Laboratory 1 (`crates/stochology-core/src/layers/`) to inspect the Bridge $L\_2 \to L\_3$ `NonlinearSummary` and leave-one-out empirical $\mathrm{CRPS}$ on `bank_calls.csv`:

~~~bash
cargo run --release --bin labs -- --layers
~~~

~~~text
=== LAB 1: THREE ONTOLOGICAL LAYERS AND BRIDGE MECHANICS (L1 -> L2 -> L3 -> L1) ===
  [Bridge L2->L3] Jensen Gap (bank_calls, 164 days): f(E[X])=1066924927.55, E[f(X)]=1075958764.73, Delta_J=+9033837.19, median=1025216361.00, q05=830290416.50, q95=1481646385.00, OOS_CRPS=90551681.37
~~~

1. **On Bridge $L\_2 \to L\_3$**, across the $164$ days of `bank_calls.csv`, the illustrative quadratic loss $f(x) = x^2$ exhibits a positive Jensen's Gap of $\Delta\_J = +9\text{,}033\text{,}837.19$ (squared daily calls, $\approx 0.85\%$) above the naive plug-in mean evaluation $f(\mathbb{E}[X])$, and reports the full quantile profile ($q\_{05}$, median, $q\_{95}$) alongside the exact $O(n \log n)$ out-of-sample $\mathrm{CRPS} = 9.055 \times 10^7$.
2. **On Pair $L\_2 \to L\_3$ (`GenerativeRagLlmJudgeCompressionAudit`, Axis 3 of the strategic synthesis in `crates/stochology-lenses/src/strategic/`; the code assigns it to this pair, and filing it in this chapter is the book's authorial mapping):**
   - **RAG Data Processing Inequality (`DpiMarkovChain`):** A Retrieval-Augmented Generation pipeline is modeled as a four-stage chain of binary symmetric channels, $\text{Corpus } W \to \text{Chunk } X \to \text{Embedding } Y \to \text{Rerank } Z$. The axis checks that mutual information can only fall along the chain, $I(W; X) \ge I(W; Y) \ge I(W; Z)$, for both the semantic-chunking and the naive fixed-boundary chains, and the semantic chain keeps $I(W; Z) = 0.382$ nats end to end (`RAG_MI=0.382 nats`, deterministic; the channel error rates are assumed).
   - **Prediction-Powered Inference for LLM-as-a-Judge (`LlmJudgePpiRectifier`, Angelopoulos et al. 2023):** A simulated judge with assumed sensitivity $0.86$ and specificity $0.72$ scores $N = 800$ unlabeled outputs whose true pass rate is $\theta^{\ast} = 0.22$, and $n\_{\text{gold}} = 160$ human-labeled gold items rectify it via $\hat{\theta}\_{\mathrm{PPI}} = \frac{1}{N}\sum\_{i=1}^N \tilde{f}(X\_i) - \frac{1}{n\_{\text{gold}}}\sum\_{j=1}^{n\_{\text{gold}}} (\tilde{f}(X\_j^{\text{gold}}) - Y\_j^{\text{gold}})$; the Rogan-Gladen inversion identity closes. The accuracy claim did **not** survive acceptance: the claim that the rectified error stays below $0.06$ passed on 19 of 20 development seeds but on only $88.0\%$ of the 200 confirmation seeds, because at $n\_{\text{gold}} = 160$ the rectifier's standard error is about $0.035$, so $0.06$ is only about $1.71$ standard errors. The claim is recorded in `discarded_claims` as refuted, and the seed-2026 errors printed by `labs` (`PPI_err=0.011 vs bias=0.188`) are a single draw, not evidence that the corrected judge meets the bound. Sizing the gold set in advance is what `prospective_ppi_gold_sample_size` in `crates/stochology-siga/src/funnel/` is for; it is exercised in a unit test and is not called by the simulation above.
   - **Conformal FinOps Cascade:** Under an assumed cost-loss rule ($C = 20$, $L = 100$, $p^\ast = 0.20$) and assumed compute fees, a calibrated local model that escalates cases near $p^\ast$ to a more expensive verifier costs $17.34$ per decision versus $24.87$ for a naive local model that under-predicts risk, about $30.3\%$ less (`FinOps_cost=17.34 vs 24.87`, simulated, `seed = 2026`).
   - **Outlier-Protected Mixed-Precision `INT4` Quantization:** On 500 simulated Pareto losses (tail index $1.65$), uniform 4-bit clipping at $4.0$ keeps only $0.17$ of the full-precision $\mathrm{ES}\_{0.99}$ (about $83\%$ erased), while routing values above the clip to FP16 keeps $1.00$ (`INT4_ES99_ret=1.00 vs 0.17`, simulated, `seed = 2026`). The same contrast on measured `danish_fire.csv` gives $1.00$ versus $0.17$ (`Fire_ES99_prot=1.00 vs clip=0.17`); the opposite real-data hypothesis, that clipping preserves $\mathrm{ES}\_{0.99}$ within $10\%$, is recorded by `G8` as a refuted discarded finding.

Test the enforcement of the Four Prohibitions directly through the epistemological gates and negative proofs (including `--defect performativity` examined in [Chapter 19](19-world-decisions-performativity.md)):

~~~bash
cargo run --release --bin gates
cargo run --release --bin gates -- --defect cromwell
cargo run --release --bin gates -- --defect performativity
~~~

When `--defect cromwell` injects a dogmatic probability ($p = 0.0$ or $p = 1.0$) into a card, or when `--defect performativity` injects a divergent RRM loop ($r\_{\mathrm{RRM}} \ge 1$), `gates` catches the violation and exits with code `1`.

## Exercises

1. **(Calculation — Jensen's Gap)** A hospital emergency room has capacity $\mu = 20$ patients/hour and waiting time proportional to $W(\lambda) = \frac{\lambda}{\mu - \lambda}$. Compare $W(\mathbb{E}[\lambda])$ against $\mathbb{E}[W(\lambda)]$ when arrival rate $\lambda$ is $10$ patients/hour on half the shifts and $18$ patients/hour on the other half. What is $\Delta\_J$?
2. **(Calculation — Prohibition 2)** Suppose daily server latency penalty is $f(X) = X^3$, where daily traffic load $X$ takes values $1$, $2$, and $6$ with equal probability $1/3$. Compute $\mathbb{E}[X]$, $f(\mathbb{E}[X])$, $\mathbb{E}[f(X)]$, and Jensen's Gap $\Delta\_J$. By what factor does $f(\mathbb{E}[X])$ understate the true average penalty $\mathbb{E}[f(X)]$?
3. **(Concept — Cromwell's Rule)** Why does assigning $P(H) = 0.9999$ behave completely differently under Bayes' theorem from assigning $P(H) = 1.0000$ when a catastrophic counterexample arrives?

<details>
<summary><strong>Analytical &amp; Computational Solutions (Click to Expand)</strong></summary>

1. **Jensen's Gap Calculation:**
   - Average arrival rate: $\mathbb{E}[\lambda] = \frac{10 + 18}{2} = 14\text{ patients/hour}$, so $W(\mathbb{E}[\lambda]) = \frac{14}{20 - 14} = \frac{14}{6} \approx 2.333$ units.
   - True expected wait: $\mathbb{E}[W(\lambda)] = \frac{1}{2}W(10) + \frac{1}{2}W(18) = \frac{1}{2}\left(\frac{10}{10} + \frac{18}{2}\right) = \frac{1 + 9}{2} = 5.000$ units.
   - Jensen's Gap: $\Delta\_J = 5.000 - 2.333 = +2.667$ units (the true wait is more than double the plug-in estimate).
2. **Prohibition 2 Jensen's Gap Calculation:**
   - $\mathbb{E}[X] = \frac{1 + 2 + 6}{3} = 3.0$, so $f(\mathbb{E}[X]) = 3.0^3 = 27.0$.
   - $\mathbb{E}[f(X)] = \frac{1^3 + 2^3 + 6^3}{3} = \frac{1 + 8 + 216}{3} = \frac{225}{3} = 75.0$.
   - $\Delta\_J = 75.0 - 27.0 = +48.0$. Reporting $f(\mathbb{E}[X]) = 27.0$ understates the true average penalty by a factor of $\frac{75}{27} \approx 2.78\times$.
3. **Cromwell's Rule:**
   When $P(H) = 1.0000$, $1 - P(H) = 0$, so Bayes' denominator cannot update away from $H$ even after observing an event with tiny likelihood under $H$. When $P(H) = 0.9999$, prior odds are $9\text{,}999 : 1$, so two independent counterexamples with likelihood ratio $10^{-3}$ immediately flip posterior odds to $9.999 \times 10^{-3} : 1$ ($\approx 1\%$ belief in $H$).

</details>

## Summary

- **Bridge $L\_2 \to L\_3$** is governed by **Jensen's Inequality** ($\Delta\_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X]) \approx \frac{1}{2}f''(\mu)\mathrm{Var}(X)$) and strictly proper distributional scoring via **$\mathrm{CRPS}$**.
- **Prohibition 2** bans reporting a lone mean under nonlinearity, multiplicative compounding, or heavy tails, requiring the 7-number `NonlinearSummary` ($f(\mathbb{E}[X]), \mathbb{E}[f(X)], \Delta\_J, \text{median}, q\_{05}, q\_{95}, \mathrm{CRPS}$).
- **Prohibition 4** bans dogmatic probabilities ($0\%$ or $100\%$, enforcing Cromwell's Rule $p \in [\varepsilon\_{\text{floor}}, 1 - \varepsilon\_{\text{floor}}]$) and pairs with the multiplicity and findings-conservation rules in [Chapter 17](17-model-decisions-card-preregistration.md).
