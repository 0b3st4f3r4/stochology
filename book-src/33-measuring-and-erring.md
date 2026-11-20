# Measuring and Erring

<figure class="stoch-figure stoch-logo-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="800" height="132" viewBox="0 0 800 132" fill="none" role="img" aria-label="Stochology CH33 · METROLOGY — Measuring, Calibrating, And Erring">
  <defs>
    <linearGradient id="ch33-ring" x1="2" y1="78" x2="386" y2="78" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="38%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="68%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <radialGradient id="ch33-glow" cx="50%" cy="50%" r="50%">
      <stop offset="0%" stop-color="#38bdf8" stop-opacity="0.24"/>
      <stop offset="48%" stop-color="#2563eb" stop-opacity="0.14"/>
      <stop offset="80%" stop-color="#e11d48" stop-opacity="0.08"/>
      <stop offset="100%" stop-color="#090e1a" stop-opacity="0"/>
    </radialGradient>
    <linearGradient id="ch33-triad-stroke" x1="18" y1="24" x2="110" y2="104" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.85"/>
      <stop offset="50%" stop-color="var(--logo-gold, #f59e0b)" stop-opacity="0.90"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.85"/>
    </linearGradient>
    <linearGradient id="ch33-fill" x1="22" y1="30" x2="106" y2="98" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)" stop-opacity="0.26"/>
      <stop offset="48%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.28"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.26"/>
    </linearGradient>
    <linearGradient id="ch33-ink-curve" x1="20" y1="66" x2="108" y2="66" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-ink, #0f172a)"/>
      <stop offset="50%" stop-color="var(--logo-sub, #1d4ed8)"/>
      <stop offset="100%" stop-color="var(--logo-ink, #0f172a)"/>
    </linearGradient>
    <radialGradient id="ch33-pupil" cx="45%" cy="42%" r="58%">
      <stop offset="0%" stop-color="#020617"/>
      <stop offset="68%" stop-color="#0f172a"/>
      <stop offset="100%" stop-color="#451a03"/>
    </radialGradient>
    <linearGradient id="ch33-gold-mira" x1="54" y1="48" x2="84" y2="82" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#fef08a"/>
      <stop offset="45%" stop-color="#fbbf24"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <radialGradient id="ch33-gold-core" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#ffffff"/>
      <stop offset="35%" stop-color="#fde047"/>
      <stop offset="100%" stop-color="#d97706"/>
    </radialGradient>
    <radialGradient id="ch33-node-well" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="var(--logo-well-hi, var(--elevated-bg, #ffffff))"/>
      <stop offset="100%" stop-color="var(--logo-node-bg, #e2e8f0)"/>
    </radialGradient>
    <linearGradient id="ch33-ring-epi" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#7dd3fc"/>
      <stop offset="50%" stop-color="#2563eb"/>
      <stop offset="100%" stop-color="#1e3a8a"/>
    </linearGradient>
    <radialGradient id="ch33-core-epi" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#bae6fd"/>
      <stop offset="45%" stop-color="#3b82f6"/>
      <stop offset="100%" stop-color="#1d4ed8"/>
    </radialGradient>
    <linearGradient id="ch33-ring-vital" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#6ee7b7"/>
      <stop offset="50%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#065f46"/>
    </linearGradient>
    <radialGradient id="ch33-core-vital" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#a7f3d0"/>
      <stop offset="45%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#047857"/>
    </radialGradient>
    <linearGradient id="ch33-ring-gov" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#fda4af"/>
      <stop offset="50%" stop-color="#e11d48"/>
      <stop offset="100%" stop-color="#881337"/>
    </linearGradient>
    <radialGradient id="ch33-core-gov" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#fecdd3"/>
      <stop offset="45%" stop-color="#f43f5e"/>
      <stop offset="100%" stop-color="#be123c"/>
    </radialGradient>
  </defs>
  <g>
    <circle cx="64" cy="66" r="48" fill="url(#ch33-glow)"/>
    <circle cx="64" cy="66" r="42" stroke="var(--logo-ticks, #64748b)" stroke-width="1.6" stroke-dasharray="3 3"/>
    <circle cx="64" cy="66" r="24" fill="url(#ch33-fill)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.0" stroke-dasharray="3.5 2.5"/>
    <line x1="50" y1="78" x2="86" y2="46" stroke="var(--logo-crimson, #e11d48)" stroke-width="2.6" stroke-linecap="round"/>
    <circle cx="82" cy="42" r="6.6" fill="url(#ch33-node-well)" stroke="url(#ch33-ring-gov)" stroke-width="2.4"/>
    <circle cx="82" cy="42" r="2.8" fill="url(#ch33-core-gov)"/>
    <circle cx="90" cy="50" r="6.6" fill="url(#ch33-node-well)" stroke="url(#ch33-ring-gov)" stroke-width="2.4"/>
    <circle cx="90" cy="50" r="2.8" fill="url(#ch33-core-gov)"/>
    <g>
      <circle cx="64" cy="66" r="7.0" fill="var(--logo-node-bg, #e2e8f0)"/>
      <circle cx="64" cy="66" r="6.0" fill="url(#ch33-pupil)" stroke="url(#ch33-gold-mira)" stroke-width="2.2"/>
      <line x1="55.0" y1="66" x2="60.0" y2="66" stroke="url(#ch33-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
      <line x1="68.0" y1="66" x2="73.0" y2="66" stroke="url(#ch33-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
      <line x1="64" y1="57.0" x2="64" y2="62.0" stroke="url(#ch33-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
      <line x1="64" y1="70.0" x2="64" y2="75.0" stroke="url(#ch33-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
      <circle cx="64" cy="66" r="2.0" fill="url(#ch33-gold-core)"/>
    </g>
  </g>
  <g transform="translate(148, 0)">
    <text x="2" y="27" fill="var(--logo-sub, #1d4ed8)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.2">PART III · FOUNDATIONS OF JUDGMENT</text>
    <text x="498" y="27" text-anchor="end" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.0">CH33 · METROLOGY</text>
    <text x="0" y="59" fill="var(--logo-ink, #0f172a)" font-family="system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif" font-size="20.2" font-weight="800" letter-spacing="1.15">MEASURING, CALIBRATING, AND ERRING</text>
    <line x1="2" y1="78" x2="384" y2="78" stroke="url(#ch33-ring)" stroke-width="1.6" stroke-opacity="0.82"/>
    <circle cx="400" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="415" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.0"/>
    <circle cx="415" cy="78" r="1.6" fill="var(--logo-blue, #2563eb)"/>
    <circle cx="430" cy="78" r="6.2" stroke="var(--logo-gold, #f59e0b)" stroke-width="1.5" stroke-dasharray="2.2 1.6"/>
    <circle cx="430" cy="78" r="4.0" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.2"/>
    <circle cx="430" cy="78" r="1.8" fill="var(--logo-gold, #f59e0b)"/>
    <circle cx="445" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-green, #10b981)" stroke-width="2.0"/>
    <circle cx="445" cy="78" r="1.6" fill="var(--logo-green, #10b981)"/>
    <circle cx="460" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="475" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="490" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <text x="2" y="104" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.8" font-weight="600" letter-spacing="0.65">RANDOM SCATTER VS. SYSTEMATIC BIAS · INSTRUMENT DRIFT · GUM BOUNDS</text>
  </g>
  </svg>
</figure>

## By the end of this chapter you can

1. Distinguish resolution, precision, and accuracy under GUM/VIM rules, and name when finer
   precision worsens a decision.
2. Identify the measurement scale of a variable (nominal, ordinal, interval, ratio) and say which
   operations it permits.
3. Refuse uncalibrated scale-breaking summaries, and state when a pre-registered outcome threshold
   licenses a fixed scoring sum.

## 1. Precision, accuracy, units, and metrological traceability

Picture an intensive-care monitor flashing a patient's mean arterial blood pressure to fine decimal places every second. The reading $x\_{\mathrm{obs}}$ sits comfortably above a danger threshold $\tau\_{\mathrm{crit}}$. Yet the catheter transducer's zero-point has drifted upward by $+b > 0$ from uncalibrated heating. Staff trust the sharp digits and withhold medicine while the patient's real pressure $x\_{\mathrm{obs}} - b < \tau\_{\mathrm{crit}}$ drops into shock. Fine display resolution hid a fatal bias inside the sensor.

In [Chapter 09](09-sets-relations-functions.md), we treated measurement as a function mapping real world states onto records. Physical metrology (`BIPM/JCGM VIM`, BIPM/JCGM 2012) separates three distinct properties of that mapping: **resolution** (`VIM 4.14`), the smallest change the display can register; **precision** (`VIM 2.15`), how tightly repeated readings cluster together; and **accuracy** (`VIM 2.13`), closeness to the true value, which requires both low scatter and zero bias. A crisp digital gauge can be tightly precise yet wildly wrong, whereas a coarse gauge stays unbiased up to its step width.

Modern data pipelines routinely commit **false precision**: printing four decimal digits from seventeen records, or reporting rates to tenths of a percent from surveys with huge non-response. Digits imply resolving power; a bare point number claims infinite certainty.

The **GUM** (BIPM/JCGM 2008) and **VIM** (BIPM/JCGM 2012) divide uncertainty **evaluation methods** into **Type A** and **Type B** (`GUM §3.3.3`)—not "random" versus "systematic" error. **Type A evaluation** estimates uncertainty from repeated observations: across $n$ independent draws, variance of the mean shrinks as $1/n$ (standard uncertainty as $1/\sqrt{n}$), though group clustering stalls that shrinkage. **Type B evaluation** quantifies uncertainty from calibration certificates, sensor resolution, thermal drift, and **definitional uncertainty** ($u\_{\mathrm{def}}$, `VIM 2.27`; `GUM D.3.4`), leaving an unshrinkable floor $u\_B = O(1)$ no matter how large $n$ grows. Known bias $b$ must be removed directly (`GUM §3.2.4`); only remaining correction uncertainty enters Type B spread.

To see why uncalibrated errors defeat huge datasets, model an observed reading as true state $X\_i \sim (\mu\_X, \sigma\_X^2)$ plus constant offset $b \in \mathbb{R}$ and zero-mean noise $u\_i \sim (0, \sigma\_u^2)$ ($i \in \lbrace 1, \dots, n\rbrace$, uncorrelated with $X\_i$ and across items): $X\_{\mathrm{obs}, i} = X\_i + b + u\_i$. Two traps follow immediately:

1. **Systematic drift ($b \neq 0$):** Under uncorrelated draws, the sample mean $\bar{X}\_{\mathrm{obs}} = \bar{X} + b + \bar{u}$ shrinks variance $(\sigma\_X^2 + \sigma\_u^2)/n \to 0$ as $n \to \infty$, locking precisely onto the wrong target $\mu\_X + b$ instead of $\mu\_X$.
2. **Classical attenuation bias ($\sigma\_X^2, \sigma\_u^2 > 0$):** Fitting $Y\_i = \alpha + \beta X\_i + \varepsilon\_i$ using noisy $X\_{\mathrm{obs},i} = X\_i + u\_i$ ($b = 0$) under exogeneity $\mathrm{Cov}(X, \varepsilon) = 0$ and classical independent error $\mathrm{Cov}(X, u) = \mathrm{Cov}(\varepsilon, u) = 0$ inflates input variance $\mathrm{Var}(X + u) = \sigma\_X^2 + \sigma\_u^2$ and shrinks the estimated slope $\hat{\beta}\_{\mathrm{OLS}}$ by the **reliability ratio** $\lambda \in (0, 1)$:

$$\hat{\beta}\_{\mathrm{OLS}} \xrightarrow{p} \frac{\mathrm{Cov}(X + u,\mkern3mu \beta X + \varepsilon)}{\mathrm{Var}(X + u)} = \beta\mkern3mu\frac{\sigma\_X^2}{\sigma\_X^2 + \sigma\_u^2} = \beta \lambda.$$

In plain words, noisy inputs dilute the apparent relationship. If measurement noise equals true signal variation ($\sigma\_u^2 = \sigma\_X^2 > 0$), $\lambda = 1/2$, cutting the estimated slope in half regardless of sample size. Note that attenuation assumes classical error: Berkson grouping error ($X\_i = X\_{\mathrm{obs},i} + w\_i$ with $\mathrm{Cov}(X\_{\mathrm{obs}}, w) = 0$) leaves regression slopes unbiased, whereas outcome-linked differential error can inflate or flip slopes entirely (causal identification belongs to [Chapter 39](39-cause-intervention-counterfactual.md)).

When human judges score dossiers instead of instruments, Type A variation splits across raters and across repeated ratings by the same judge. Let evaluator $k \in \lbrace 1, \dots, K\rbrace$ make $m \ge 1$ assessments $i \in \lbrace 1, \dots, m\rbrace$ of target $X$, written $X\_{\mathrm{obs}, k, i} = X + b + u\_{\mathrm{inter}, k} + u\_{\mathrm{intra}, k, i}$. Here $b$ is shared systemic bias, $\sigma\_{\mathrm{inter}}^2$ is zero-mean variance across raters, and $\sigma\_{\mathrm{intra}}^2$ is zero-mean scatter within one rater (mutually uncorrelated). Single-rating variance $\sigma\_{\mathrm{eval}}^2$ and panel-mean variance $\sigma\_{\mathrm{mean}}^2(K, m)$ satisfy ($K \ge 1, m \ge 1$):

$$\sigma\_{\mathrm{eval}}^2 = \sigma\_{\mathrm{inter}}^2 + \sigma\_{\mathrm{intra}}^2, \qquad \sigma\_{\mathrm{mean}}^2(K, m) = \frac{\sigma\_{\mathrm{inter}}^2}{K} + \frac{\sigma\_{\mathrm{intra}}^2}{K m}.$$

Read aloud, testing one judge repeatedly ($K = 1, m \to \infty$) shrinks $\sigma\_{\mathrm{intra}}^2/m \to 0$ while leaving between-rater disagreement $\sigma\_{\mathrm{inter}}^2$ intact. Pooling $K$ independent raters cuts both variance components, yet still leaves shared systemic bias $b$ untouched. Worse, feeding symmetric judge noise into a weakest-link minimum pushes reported scores downward via concavity ([Chapter 12](12-derivatives-and-convexity.md)).

Under **metrological traceability** (`VIM 2.41`–`2.42`, BIPM/JCGM 2012), calibration chains combine standard uncertainties in quadrature (`GUM`, BIPM/JCGM 2008) instead of multiplying expanded margins (`VIM 2.35`). No downstream measurement can claim tighter standard error than its upstream anchor. Yet even an unbroken calibration certificate misses a flawed reference inside its own loop. The Hubble Space Telescope's $2.4\text{ m}$ mirror passed its paper certification yet suffered a $0.4\text{-wave rms}$ error at $632.8\text{ nm}$—ten times tolerance—because a lens in its primary null corrector sat $1.3\text{ mm}$ out of place while two independent optical checks flagging the flaw were dismissed (NASA 1990). §4 bounds when further measurement must stop.

## 2. Stevens's scale taxonomy and permissible transformations

Imagine two surgical wards asking post-operative patients to rate pain on ordered cards (`none` $<$ `mild` $<$ `moderate` $<$ `severe` $<$ `excruciating`), labeled with rank codes $r\_1 < r\_2 < r\_3 < r\_4 < r\_5$. A hospital dashboard averages those numeric codes, reports lower mean pain in Ward B ($\bar{X}\_B < \bar{X}\_A$), and penalizes Ward A. That dashboard silently pretended the step from mild to moderate equals the step from severe to excruciating.

As [Chapter 09](09-sets-relations-functions.md) showed when separating equivalence from order, numbers carry different structural commitments. S. S. Stevens (1946) classified measurement scales—placed on axiomatic foundations by Krantz, Luce, Suppes, and Tversky (1971)—according to which mathematical transformations leave meaning intact:

| Scale | Defining Relation | Permissible Group Transformation | Permissible Meaning (Invariant or Equivariant) |
| :--- | :--- | :--- | :--- |
| Nominal | Equivalence ($=, \neq$) | Any bijection $f$ | Mode, contingency tables, entropy (invariant) |
| Ordinal | Strict order ($>, <$) | Strictly increasing map $f$ ($x < y \iff f(x) < f(y)$) | Median, quantiles, Mann–Whitney dominance (invariant) |
| Interval | Known unit, arbitrary zero | Positive affine $f(x) = a x + b\ (a > 0)$ | Pearson $r$, difference ratios (invariant); mean, variance (equivariant) |
| Ratio | Natural physical zero | Positive similarity $f(x) = a x\ (a > 0)$ | Coefficient of variation, raw ratios (invariant, $x > 0$); geometric mean (equivariant, $x > 0$) |

Two daily errors break these rules:

1. **Averaging ordinal codes:** Suppose $K \ge 2$ ordered grades carry strictly increasing scores $r\_1 < \dots < r\_K$, with cumulative ward shares $F\_A(r\_k)$ and $F\_B(r\_k)$ ($F\_A(r\_K) = F\_B(r\_K) = 1$). Before trusting any averaged ward score, ask how sensitive the gap $\overline{\psi(X)}\_A - \overline{\psi(X)}\_B$ is to re-spacing the ordinal grades by another order-preserving code $\psi$ ($r\_j < r\_k \iff \psi(r\_j) < \psi(r\_k)$). Summation by parts over $k = 1, \dots, K - 1$ isolates the step weights:

   $$\overline{\psi(X)}\_A - \overline{\psi(X)}\_B = \sum\_{k=1}^{K-1} \bigl(F\_B(r\_k) - F\_A(r\_k)\bigr)\bigl(\psi(r\_{k+1}) - \psi(r\_k)\bigr).$$

   Read aloud, whenever neither ward uniformly beats the other across all thresholds—so $F\_B(r\_k) - F\_A(r\_k)$ switches sign—re-weighting the positive steps $\psi(r\_{k+1}) - \psi(r\_k) > 0$ flips which ward looks worse without changing a single patient's pain.
2. **Ratios of interval readings:** Calling one Celsius or Fahrenheit reading "twice as warm" as another is meaningless. Under an affine temperature shift $f(T) = aT + b$ ($a > 0$, $b \neq 0$), for any nonzero pair $T\_1 \neq T\_2$ ($T\_1 \neq 0$ and $f(T\_1) \neq 0$), the ratio $T\_2/T\_1$ shifts by $(aT\_2 + b)/(aT\_1 + b) - T\_2/T\_1 = b(T\_1 - T\_2)/\bigl(T\_1(aT\_1 + b)\bigr) \neq 0$. By contrast, ratios of *differences* $\bigl(f(T\_3) - f(T\_2)\bigr)/\bigl(f(T\_2) - f(T\_1)\bigr) = (T\_3 - T\_2)/(T\_2 - T\_1)$ stay invariant because $a$ and $b$ cancel. Raw ratios require a true zero ($b = 0$), such as temperature in kelvins ($\text{K}$).

## 3. The invariance test and non-parametric dominance

One practical rule enforces scale integrity at formalization guard `T4` ([Chapter 29](29-aim-and-the-three-planes.md)):

> Take any proposed uncalibrated summary statistic or ranking rule. Apply an arbitrary admissible transformation from the variable's scale. If the substantive ranking or conclusion flips, the statistic measured an artifact of arbitrary coding rather than the world.

For uncalibrated ordinal cohorts $X = \lbrace x\_1, \dots, x\_m\rbrace$ and $Y = \lbrace y\_1, \dots, y\_n\rbrace$ ($m, n \ge 1$), arithmetic averages fail instantly. Critics often point to fixed clinical scores like Virginia Apgar's (1953) sum of five newborn signs graded $0, 1, 2$ onto $0$–$10$. That score works only because weights and triage cutoff were jointly pre-calibrated against newborn survival—a license revoked whenever disease shifts tolerance bounds (Canguilhem 1943/1966) or gaming corrupts ratings (`T6`).

To compare two ordinal cohorts without inventing fake spacing, count how often an observation from $X$ exceeds one from $Y$, giving half-credit for ties. That yields the empirical **Mann–Whitney dominance** index ($m, n \ge 1$):

$$\hat{\Delta}\_{\mathrm{MW}}(X, Y) = \frac{1}{m n}\sum\_{i=1}^{m}\sum\_{j=1}^{n}\left(\mathbf{1}\lbrace{}x\_i > y\_j\rbrace + \frac{1}{2}\mathbf{1}\lbrace{}x\_i = y\_j\rbrace\right) = \mathbb{P}\_{\mathrm{emp}}(X > Y) + \frac{1}{2}\mathbb{P}\_{\mathrm{emp}}(X = Y) \in [0, 1].$$

In plain words, $\hat{\Delta}\_{\mathrm{MW}}$ estimates $\Delta\_{\mathrm{MW}} = \mathbb{P}(X > Y) + \frac{1}{2}\mathbb{P}(X = Y)$ and stays completely unchanged under *any* common strictly increasing map $\psi$, because $x\_i > y\_j \iff \psi(x\_i) > \psi(y\_j)$ everywhere.

Scale misuse stays common in published research. Reviewing 2016 Likert studies across *JPSP* ($38$), *Psychological Science* ($20$), and *JEP:G* ($20$; $78$ found, $10$ excluded), Liddell and Kruschke (2018) found all $68$ of $68$ ($100\\%$) ordinal analyses used metric models. On Amazon *Drama* ratings ($36$ films, $502$ to $111{,}232$ reviews), metric averages even inverted contrast signs relative to ordered-probit models (Cases $5$ and $6$, $N = 1{,}201$ and $N = 13{,}897$: metric mode $-0.416$, $95\\%$ HDI $[-0.503, -0.329]$ vs. ordered-probit mode $1.24$, $95\\%$ HDI $[0.818, 1.79]$; Liddell & Kruschke 2018). Ordered-probit models estimate latent cutoffs rather than pretending ordinal steps are equal. On ratio scales, robust location estimators resist small archive outliers where means fail (Huber 1964), though neither ordinal dominance nor robust centers replaces heavy-tail risk modeling ([Chapter 31](31-distribution-and-moment.md)).

Even when scales are respected, Samuel Messick's unified validity framework (Messick 1995) warns of **two** construct threats plus a **consequential aspect** across six facets:

1. **Construct underrepresentation:** the test omits vital aspects of the underlying capacity.
2. **Construct-irrelevant variance:** outside noise—test coaching, language barriers, guessing—contaminates scores.
3. **The consequential aspect:** social impacts and incentive distortion under high-stakes scoring.

Across [Chapter 29](29-aim-and-the-three-planes.md)'s circuit, these align directly with framing (`T1`), observation (`T3`), scale formalization (`T4`), and performative action (`T6`).

## 4. Missing is not zero

Suppose a disaster-relief ledger records flood damage across twelve wards. Washed-out bridges keep inspectors out of three riverfront wards for seventy-two hours, leaving their cells blank. An automated script converts blank cells into zeros and sends repair crews to dry hills. The archive did not record zero damage; it recorded the absence of the inspector.

Relational database theory separated blank unknowns from zeros decades ago. In extended model RM/T, E. F. Codd (1979) introduced a null mark $\omega$ under three-valued logic (`3VL`: true, false, maybe; [Chapter 06](06-who-is-missing-from-the-table.md)). Codd (1986) then split missingness into **A-marks** (applicable to the entity, yet unknown) and **I-marks** (declared in the table, yet inapplicable to that row), before introducing four-valued truth tables (`4VL`: true `t`, false `f`, maybe-applicable `a`, maybe-inapplicable `i`) in Codd (1987) so comparisons on either mark never return `TRUE`. Four distinct states govern metrological and database tables:

1. **Not observed (*A-mark*):** The attribute applies to the unit, yet the record missed its value—leaving true state unknown rather than zero (`T3` transit; bounded in [Chapter 34](34-sample-population-archive.md)).
2. **Observed as absent (measured zero):** The sensor inspected the unit and logged an ordinary zero.
3. **Schema-inapplicable (*I-mark*):** A column does not fit that row (like residential flood claims on a public park), marking inapplicability rather than zero or unmeasured harm.
4. **Not askable ($\pi\_i \equiv 0$, pre-schema exclusion):** The category or group has zero column or row in the table (`T1` framing in [Chapter 32](32-the-question-before-the-data.md)). Even Codd's `4VL` cannot encode unframed exclusions; treating schema silence as zero mistakes blindness for proof.

Sensors routinely break Codd's (1986) rule at intake: when a battery monitor or clinic pulse oximeter disconnects and sends a zero-volt packet or blank frame, pipelines reading silent dropout as zero turn an *A-mark* into fake safety—feeding survivorship bias whenever failing units drop offline first.

Standard SQL collapses *A-marks* and *I-marks* into a single `NULL` under Kleene three-valued logic (`TRUE`, `FALSE`, `UNKNOWN`). Take a table of $n\_{\mathrm{tbl}} = n\_{\mathrm{obs}} + n\_{\mathrm{mis}} \ge 1$ rows holding $n\_{\mathrm{obs}}$ measured values (including true zeros) and $n\_{\mathrm{mis}}$ `NULL` entries. Because comparisons against `NULL` return `UNKNOWN` and `WHERE` filters keep only `TRUE`, complementary predicates $P(x) := (x \le c)$ and $\neg P(x) := (x > c)$ lose excluded middle: $n\_{\le c} + n\_{> c} = n\_{\mathrm{obs}} \le n\_{\mathrm{tbl}}$, silently dropping $n\_{\mathrm{mis}}$ rows. Worse, SQL aggregates diverge silently: `COUNT(*)` counts $n\_{\mathrm{tbl}}$, whereas `COUNT(col)` counts $n\_{\mathrm{obs}}$ and `AVG(col)` averages only observed rows ($\bar{Y}\_{\mathrm{obs}}$ for $n\_{\mathrm{obs}} \ge 1$), hiding selection bias. Conversely, `AVG(COALESCE(col, 0))` overwrites missing values with zero—giving $0$ when $n\_{\mathrm{obs}} = 0$ and $\bar{Y}\_{\mathrm{coal}} = \bar{Y}\_{\mathrm{obs}} - \frac{n\_{\mathrm{mis}}}{n\_{\mathrm{tbl}}}\bar{Y}\_{\mathrm{obs}}$ otherwise—injecting downward bias while faking full table coverage $n\_{\mathrm{tbl}}$.

Evidential weight never equals raw row count. Following John Maynard Keynes (1921), Stochology indexes **evidential weight** $V\bigl(n\_{\mathrm{eff}}(\bar{\rho}), n\_0\bigr)$ by comparing a design- and selection-adjusted effective sample $n\_{\mathrm{eff}}(\bar{\rho}) \ge 0$ (derived in [Chapter 34](34-sample-population-archive.md) §3 from cluster design effects $\mathrm{DEFF}\_{\mathrm{total}} \ge 1$, sample fraction $f = n/N$, and selection bound $\bar{\rho} \ge |\rho\_{R,Y}|$) against a positive ignorance scale $n\_0 := n\_{\mathrm{req}}(\alpha, \beta, p^\ast) > 0$.

How do we size that ignorance scale $n\_0$? Anchor it on the local binomial sample needed around cost–loss cutoff $p^\ast = C/(\eta L) \in (0, 1)$ ([Chapter 32](32-the-question-before-the-data.md) §5; derived fully in [Chapter 36](36-decision-loss-threshold.md)). Sizing variance $p^\ast(1 - p^\ast)/n\_0$ to resolve margin $\Delta p = \varepsilon\_{\mathrm{rel}} p^\ast$ ($\varepsilon\_{\mathrm{rel}} > 0$, $p^\ast(1 + \varepsilon\_{\mathrm{rel}}) \le 1$) at two error limits $\alpha, \beta \in (0, 1)$ ($\alpha + \beta < 1$) cancels one $p^\ast$ factor and guarantees $n\_{\mathrm{eff}}(\bar{\rho}) + n\_0 > 0$:

$$V\bigl(n\_{\mathrm{eff}}(\bar{\rho}),\mkern3mu n\_0\bigr) = \frac{n\_{\mathrm{eff}}(\bar{\rho})}{n\_{\mathrm{eff}}(\bar{\rho}) + n\_0} \in [0, 1), \qquad n\_0 := \left(\frac{z\_{1-\alpha} + z\_{1-\beta}}{\varepsilon\_{\mathrm{rel}}}\right)^2 \frac{1 - p^\ast}{p^\ast}.$$

In plain words, evidential weight $V$ increases strictly with effective sample $n\_{\mathrm{eff}}(\bar{\rho})$ ($\partial V / \partial n\_{\mathrm{eff}} = n\_0 / (n\_{\mathrm{eff}} + n\_0)^2 > 0$) between two intuitive limits:

1. **Vacuous, clustered, or biased archive ($n\_{\mathrm{eff}}(\bar{\rho}) \to 0$):** Heavy clustering, tiny coverage $f$ under $\bar{\rho} > 0$, or zero rows pulls $V \to 0$ regardless of megabytes stored.
2. **Intra-schema saturation ($n\_{\mathrm{eff}}(\bar{\rho}) \to \infty$):** Writing $V = (1 + n\_0/n\_{\mathrm{eff}})^{-1}$ shows weight approaches $1$ asymptotically, so rows past $n\_0$ buy tiny extra weight.

Million-row tables lacking provenance carry massive false precision unless deflated by clustering, selection bounds, and explicit *A-mark*/*I-mark* tags. Even then, $V$ sees only inside its frame: when unregistered groups are **Not askable** ($\pi\_i \equiv 0$, `T1`), $V \to 1$ reports false fullness while harm spills onto external reception perimeters (formalized later in Part III and [Appendix A](appendix-a-mathematics.md) §A.7).

### Weight saturates, sample does not

Weight is not probability. Writing $V\bigl(n\_{\mathrm{eff}}(\bar{\rho}), n\_0\bigr) = \bigl(1 + n\_0 / n\_{\mathrm{eff}}(\bar{\rho})\bigr)^{-1}$ for $n\_{\mathrm{eff}}(\bar{\rho}) > 0$ shows that weight tracks the ratio $n\_{\mathrm{eff}}(\bar{\rho}) / n\_0$: a smaller effective sample posts higher weight whenever $n\_0$ is smaller. Meanwhile, quadratic saturation $\partial V / \partial n\_{\mathrm{eff}} = n\_0 / (n\_{\mathrm{eff}} + n\_0)^2$ makes huge archives look nearly equal in weight despite wide gaps in effective sample.

### When precision should stop

Measuring further must stop once marginal measurement cost $\Delta C\_{\mathrm{meas}}$—combining instrumentation outlay and deliberation delay $\Delta\tau\_{\mathrm{cog}}$—exceeds expected decision improvement ([Chapter 36](36-decision-loss-threshold.md) §4) or hits the Type B floor. Under the live attention ceiling of [Chapter 29](29-aim-and-the-three-planes.md) §4 and [Chapter 36](36-decision-loss-threshold.md), `VIM`/`NASA` calibration trees, `T4` scale proofs, and `3VL`/`4VL` null-lineage audits run off-shift so crisis crews inspect only sealed intervals and explicit `Not observed` flags ([Chapter 38](38-time-and-dependence.md)).

## Bridge

[Chapter 14](14-linear-algebra.md) linked linear projections and dependence; this chapter built the metrological rules for reading observations: Type A vs Type B floors (§1), Stevens scale invariance (§2–§3), Codd's missingness logic, and Keynesian evidential weight (§4). How sampling designs, cluster correlation, selection bias $\rho\_{R,Y}$, and multi-list capture–recapture convert raw database rows into effective count $n\_{\mathrm{eff}}(\bar{\rho})$ opens next in [Chapter 34](34-sample-population-archive.md), *Sample, Population, Archive*.
