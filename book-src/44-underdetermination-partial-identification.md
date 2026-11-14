# Underdetermination, Partial Identification, and Severe Testing

<figure class="stoch-figure stoch-logo-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="660" height="132" viewBox="0 0 660 132" fill="none" role="img" aria-label="Stochology CH44 · S_EPI — Partial Identification And Bounds">
  <defs>
    <linearGradient id="ch44-ring" x1="2" y1="78" x2="386" y2="78" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="38%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="68%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <radialGradient id="ch44-glow" cx="50%" cy="50%" r="50%">
      <stop offset="0%" stop-color="#38bdf8" stop-opacity="0.25"/>
      <stop offset="50%" stop-color="#2563eb" stop-opacity="0.14"/>
      <stop offset="82%" stop-color="#fbbf24" stop-opacity="0.08"/>
      <stop offset="100%" stop-color="#090e1a" stop-opacity="0"/>
    </radialGradient>
    <linearGradient id="ch44-triad-stroke" x1="18" y1="24" x2="110" y2="104" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.85"/>
      <stop offset="50%" stop-color="var(--logo-gold, #f59e0b)" stop-opacity="0.90"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.85"/>
    </linearGradient>
    <linearGradient id="ch44-fill" x1="22" y1="30" x2="106" y2="98" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)" stop-opacity="0.26"/>
      <stop offset="48%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.28"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.26"/>
    </linearGradient>
    <linearGradient id="ch44-ink-curve" x1="20" y1="66" x2="108" y2="66" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-ink, #0f172a)"/>
      <stop offset="50%" stop-color="var(--logo-sub, #1d4ed8)"/>
      <stop offset="100%" stop-color="var(--logo-ink, #0f172a)"/>
    </linearGradient>
    <radialGradient id="ch44-pupil" cx="45%" cy="42%" r="58%">
      <stop offset="0%" stop-color="#020617"/>
      <stop offset="68%" stop-color="#0f172a"/>
      <stop offset="100%" stop-color="#451a03"/>
    </radialGradient>
    <linearGradient id="ch44-gold-mira" x1="54" y1="48" x2="84" y2="82" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#fef08a"/>
      <stop offset="45%" stop-color="#fbbf24"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <radialGradient id="ch44-gold-core" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#ffffff"/>
      <stop offset="35%" stop-color="#fde047"/>
      <stop offset="100%" stop-color="#d97706"/>
    </radialGradient>
    <radialGradient id="ch44-node-well" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="var(--logo-well-hi, var(--elevated-bg, #ffffff))"/>
      <stop offset="100%" stop-color="var(--logo-node-bg, #e2e8f0)"/>
    </radialGradient>
    <linearGradient id="ch44-ring-epi" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#7dd3fc"/>
      <stop offset="50%" stop-color="#2563eb"/>
      <stop offset="100%" stop-color="#1e3a8a"/>
    </linearGradient>
    <radialGradient id="ch44-core-epi" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#bae6fd"/>
      <stop offset="45%" stop-color="#3b82f6"/>
      <stop offset="100%" stop-color="#1d4ed8"/>
    </radialGradient>
    <linearGradient id="ch44-ring-vital" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#6ee7b7"/>
      <stop offset="50%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#065f46"/>
    </linearGradient>
    <radialGradient id="ch44-core-vital" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#a7f3d0"/>
      <stop offset="45%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#047857"/>
    </radialGradient>
    <linearGradient id="ch44-ring-gov" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#fda4af"/>
      <stop offset="50%" stop-color="#e11d48"/>
      <stop offset="100%" stop-color="#881337"/>
    </linearGradient>
    <radialGradient id="ch44-core-gov" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#fecdd3"/>
      <stop offset="45%" stop-color="#f43f5e"/>
      <stop offset="100%" stop-color="#be123c"/>
    </radialGradient>
  </defs>
  <g>
    <circle cx="64" cy="66" r="48" fill="url(#ch44-glow)"/>
    <circle cx="64" cy="66" r="42" stroke="var(--logo-ticks, #64748b)" stroke-width="1.2" stroke-dasharray="3 3" stroke-opacity="0.45"/>
    <rect x="22" y="54" width="84" height="24" rx="4" fill="var(--logo-blue, #2563eb)" fill-opacity="0.16" stroke="var(--logo-blue, #2563eb)" stroke-width="2.0" stroke-dasharray="4 2.5"/>
    <rect x="42" y="48" width="46" height="36" rx="4" fill="var(--logo-green, #10b981)" fill-opacity="0.22" stroke="var(--logo-green, #10b981)" stroke-width="2.6"/>
    <circle cx="22.00" cy="66.00" r="6.38" fill="url(#ch44-node-well)" stroke="url(#ch44-ring-epi)" stroke-width="2.36"/>
    <circle cx="22.00" cy="66.00" r="2.74" fill="url(#ch44-core-epi)"/>
    <circle cx="106.00" cy="66.00" r="6.38" fill="url(#ch44-node-well)" stroke="url(#ch44-ring-gov)" stroke-width="2.36"/>
    <circle cx="106.00" cy="66.00" r="2.74" fill="url(#ch44-core-gov)"/>
    <g>
      <circle cx="68.00" cy="66.00" r="6.96" fill="var(--logo-node-bg, #e2e8f0)"/>
      <circle cx="68.00" cy="66.00" r="6.02" fill="url(#ch44-pupil)" stroke="url(#ch44-gold-mira)" stroke-width="2.26"/>
      <line x1="58.98" y1="66.00" x2="64.05" y2="66.00" stroke="url(#ch44-gold-mira)" stroke-width="2.26" stroke-linecap="round"/>
      <line x1="71.95" y1="66.00" x2="77.02" y2="66.00" stroke="url(#ch44-gold-mira)" stroke-width="2.26" stroke-linecap="round"/>
      <line x1="68.00" y1="56.98" x2="68.00" y2="62.05" stroke="url(#ch44-gold-mira)" stroke-width="2.26" stroke-linecap="round"/>
      <line x1="68.00" y1="69.95" x2="68.00" y2="75.02" stroke="url(#ch44-gold-mira)" stroke-width="2.26" stroke-linecap="round"/>
      <circle cx="68.00" cy="66.00" r="2.07" fill="url(#ch44-gold-core)"/>
    </g>
  </g>
  <g transform="translate(148, 0)">
    <text x="2" y="27" fill="var(--logo-sub, #1d4ed8)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.2">PART IV · THE FRAMEWORK</text>
    <text x="498" y="27" text-anchor="end" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.0">CH44 · S_EPI</text>
    <text x="0" y="59" fill="var(--logo-ink, #0f172a)" font-family="system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif" font-size="20.2" font-weight="800" letter-spacing="1.15">PARTIAL IDENTIFICATION AND BOUNDS</text>
    <line x1="2" y1="78" x2="384" y2="78" stroke="url(#ch44-ring)" stroke-width="1.6" stroke-opacity="0.82"/>
    <circle cx="400" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.0"/>
    <circle cx="400" cy="78" r="1.6" fill="var(--logo-blue, #2563eb)"/>
    <circle cx="415" cy="78" r="6.2" stroke="var(--logo-gold, #f59e0b)" stroke-width="1.5" stroke-dasharray="2.2 1.6"/>
    <circle cx="415" cy="78" r="4.0" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.2"/>
    <circle cx="415" cy="78" r="1.8" fill="var(--logo-gold, #f59e0b)"/>
    <circle cx="430" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.0"/>
    <circle cx="430" cy="78" r="1.6" fill="var(--logo-blue, #2563eb)"/>
    <circle cx="445" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="460" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="475" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="490" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <text x="2" y="104" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.8" font-weight="600" letter-spacing="0.65">LALONDE BENCHMARK · MANSKI AND LEE ATE BOUNDS · PETERSON AND VILLE E-PROCESS</text>
  </g>
  </svg>
</figure>

## By the end of this chapter you can

1. Derive the two-arm worst-case bound from the law of total expectation, and say why its width does not shrink with sample size.
2. Distinguish identification uncertainty from sampling uncertainty, and say which one a confidence interval on the bounds is for.
3. Decide when a bound is the decision and when it is only a preliminary, and name the premise each narrowing buys.

An inquiry can be honestly framed and still underdetermine its answer. Mediating Filter I · Axis 1 ($S\_{\mathrm{epi}}$) continues here, and Tension 2 picks up where Chapter 43 stopped: underdetermination of theory by finite, incomplete data (Duhem, 1906; Quine, 1951; Stanford, 2006) on one side, and severe sequential testing and partial identification (Ville, 1939; Popper, 1959; Lakatos, 1970; Peterson, 1976; Manski, 1990, 2003; Imbens & Manski, 2004; Lee, 2009; Mayo, 2018; Ramdas et al., 2023) on the other.

In Chapter 42 the naive optimiser fails on a large share of missing vulnerable-neighbourhood observations: it assumes *missing completely at random* and reports a single point estimate $\bar{O}\_t$. Charles Manski called this **the Law of Decreasing Credibility**: the credibility of inference decreases with the strength of the assumptions maintained.

---

## 3.1 Anytime-Valid Betting Martingales ($e$-Processes) and Lakatos's Shift Ratio

Fixed-horizon $p$-values become invalid whenever an analyst peeks at accumulating data and stops early ("optional stopping"). Following Jean Ville (1939), Glenn Shafer (2021), and Aaditya Ramdas et al. (2023), a sequential **evidence process** (*e-process*) is a non-negative test martingale $(E\_t)\_{t \ge 0}$ with $E\_0 = 1$ and $\mathbb{E}\_{H\_0}[E\_t \mid \mathcal{F}\_{t-1}] \le E\_{t-1}$:

$$E\_t = \prod\_{i=1}^t e\_i$$

By **Ville's Inequality** (1939), the probability that $E\_t$ *ever* crosses $1/\alpha$ at any stopping time $\tau$ under $H\_0$ is bounded by $\alpha$:

$$\mathbb{P}\_{H\_0}\mkern-3mu\left(\sup\_{t \ge 1} E\_t \ge \frac{1}{\alpha}\right) \le \alpha$$

The operator pairs the e-process with a time-uniform confidence sequence radius, and it penalizes the corroborated evidence by $1 + K\_{\text{ad-hoc}}$, distinguishing **progressive** research programs from **degenerating** ones in Lakatos's (1970) sense: a program that has to keep adding auxiliary hypotheses to survive has bought its survival with its content.

---

## 3.2 Non-Parametric Partial Identification: Manski, Imbens–Manski, Lee, and Peterson

Instead of inventing unverifiable missingness assumptions to force a point number, **Canonical Operator I** computes **sharp non-parametric bounds**:

1. **Algebraic Derivation of Manski (1990, 2003) Two-Arm ATE Bounds Under Attrition:**
   Let $Y(1), Y(0) \in [y\_{\min}, y\_{\max}]$ denote potential outcomes under treatment and control. In an imperfect trial or observational cohort, reporting is recorded by binary indicator $R\_d \in \{0, 1\}$ for subjects assigned to arm $d \in \{0, 1\}$. By the Law of Total Expectation, the marginal mean for arm $d$ decomposes into observed and unobserved strata:
   $$\mathbb{E}[Y(d)] = \mathbb{E}[Y(d) \mid R\_d = 1]\mkern3mu\mathbb{P}(R\_d = 1) + \mathbb{E}[Y(d) \mid R\_d = 0]\mkern3mu\mathbb{P}(R\_d = 0)$$
   Because the unobserved counterfactual $\mathbb{E}[Y(d) \mid R\_d = 0]$ is unknown, Charles Manski's sharp worst-case single-arm bounds insert the physical domain extrema $[y\_{\min}, y\_{\max}]$:
   $$L\_{\mathrm{M}}(d) = p\_{\mathrm{obs}, d}\mkern3mu\bar{Y}\_{\mathrm{obs}, d} + (1 - p\_{\mathrm{obs}, d})\mkern3muy\_{\min}, \qquad U\_{\mathrm{M}}(d) = p\_{\mathrm{obs}, d}\mkern3mu\bar{Y}\_{\mathrm{obs}, d} + (1 - p\_{\mathrm{obs}, d})\mkern3muy\_{\max}$$
   where $p\_{\mathrm{obs}, d} = \mathbb{P}(R\_d = 1)$ and $\bar{Y}\_{\mathrm{obs}, d} = \mathbb{E}[Y(d) \mid R\_d = 1]$. For the causal **Average Treatment Effect** $\mathrm{ATE} = \mathbb{E}[Y(1)] - \mathbb{E}[Y(0)]$, subtracting the lower bound of arm $0$ from the upper bound of arm $1$ (and vice versa) yields the sharp worst-case two-arm bounds:
   $$\Delta\_L = \mathbb{E}[Y(1) \mid R\_1 = 1] p\_{\mathrm{obs}, 1} + y\_{\min}(1 - p\_{\mathrm{obs}, 1}) - \bigl[\mathbb{E}[Y(0) \mid R\_0 = 1] p\_{\mathrm{obs}, 0} + y\_{\max}(1 - p\_{\mathrm{obs}, 0})\bigr]$$
   $$\Delta\_U = \mathbb{E}[Y(1) \mid R\_1 = 1] p\_{\mathrm{obs}, 1} + y\_{\max}(1 - p\_{\mathrm{obs}, 1}) - \bigl[\mathbb{E}[Y(0) \mid R\_0 = 1] p\_{\mathrm{obs}, 0} + y\_{\min}(1 - p\_{\mathrm{obs}, 0})\bigr]$$
   The resulting **identification interval width** is:
   $$\Delta\_{\mathrm{id}} = \Delta\_U - \Delta\_L = (y\_{\max} - y\_{\min})\bigl[(1 - p\_{\mathrm{obs}, 1}) + (1 - p\_{\mathrm{obs}, 0})\bigr]$$
   This width represents **structural underdetermination**: it does *not* shrink to zero as sample size $n \to \infty$, so collecting millions of records under the same non-random missingness design leaves $\Delta\_{\mathrm{id}}$ intact. Sampling fluctuations around $[L\_{\mathrm{M}}, U\_{\mathrm{M}}]$ are captured by the Imbens–Manski (2004) parameter-coverage confidence interval.

2. **David S. Lee's (2009) Sharp Trimming Bounds Under Differential Attrition:**
   In randomized trials where retention rates differ ($p\_{\mathrm{obs}, 1} \ne p\_{\mathrm{obs}, 0}$), differential attrition reintroduces selection bias. Lee (2009) established that if treatment assignment affects attrition monotonically ($R\_i(1) \ge R\_i(0)$ for all $i$), the excess retained fraction in the higher-retention arm (assuming $p\_{\mathrm{obs}, 1} > p\_{\mathrm{obs}, 0}$):
   $$q = \frac{p\_{\mathrm{obs}, 1} - p\_{\mathrm{obs}, 0}}{p\_{\mathrm{obs}, 1}}$$
   can be trimmed from the upper and lower tails of the observed treatment distribution $Y\_{\mathrm{obs}, 1}$. Trimming the top $q$-quantile yields the lower bound $L\_{\mathrm{Lee}}$, while trimming the bottom $q$-quantile yields $U\_{\mathrm{Lee}}$. This yields sharp bounds for the average treatment effect on the "always-reporting" subpopulation without making parametric missing-at-random assumptions — collapsing to a point estimate when attrition balances ($q = 0$), whereas $[L\_{\mathrm{Lee}}, U\_{\mathrm{Lee}}]$ and the whole-population worst-case interval $[\Delta\_L, \Delta\_U]$ bound distinct estimands unless treatment effects are homogeneous across selection strata.

3. **Arthur V. Peterson Jr.'s (1976) Competing-Risks Fréchet Bounds and Censoring Bounds:**
   When two competing risks have marginal cause-specific survival curves $S\_1(t) = \mathbb{P}(T\_1 > t)$ and $S\_2(t) = \mathbb{P}(T\_2 > t)$, analysts routinely assume independence ($S\_{\mathrm{indep}}(t) = S\_1(t) S\_2(t)$). Peterson (1976) proved that under arbitrary dependence, the joint survival function $S(t) = \mathbb{P}(T\_1 > t, T\_2 > t)$ is bounded by the classical Fréchet–Hoeffding copula inequalities:
   $$\max\bigl(0,\mkern5mu S\_1(t) + S\_2(t) - 1\bigr) \le S(t) \le \min\bigl(S\_1(t),\mkern3mu S\_2(t)\bigr)$$
   Analogously, for a finite cohort of $n$ subjects with $d$ observed failures and $c$ censored subjects, worst-case non-parametric survival bounds without an independent-censoring assumption are:
   $$S\_{\mathrm{lower}} = 1 - \frac{d + c}{n}, \qquad S\_{\mathrm{upper}} = 1 - \frac{d}{n}$$
   Any survival estimate claiming higher precision (such as naive Kaplan–Meier) rests on an unverifiable independence assumption.

4. **The Mayo–Manski Severe Boundary Bridge:**
   A substantive policy claim $\mu > \mu\_0$ passes a severe error-statistical test under partial identification if and only if the **lower Imbens–Manski bound** $L\_{\mathrm{IM}}$ strictly exceeds $\mu\_0$, ensuring that neither sampling error nor structural underdetermination can account for the observed effect.

---

## 3.3 Equipping the Agent with the Epistemic Margin

Look back at the loop of Chapter 42. When the epistemic margin is switched on, the policy builds its demand interval from the observed clusters *and* from the count of the missing ones. Because $[L\_{\mathrm{IM}}, U\_{\mathrm{IM}}]$ honestly spans the missing vulnerable households, the interval covers the shifts the naive rule missed, and it covers them for a reason the naive rule cannot reproduce: the width is a statement about the archive rather than about the sample.

The single exception is instructive. A shock whose size exceeds the physical support the bound was built on, the observed maximum of the recorded variable, is not covered by any bound that takes that support as given, and the honest reading is not that the margin failed but that the support assumption did. This is why the final subsection of §3.5 (*Where sharpness stops*) treats the ceiling as a declared institutional measurement rather than an inherited default.

---

## 3.4 The LaLonde Benchmark and the Bounds It Supports

Robert LaLonde's (1986) evaluation of a supported-work training programme is the canonical demonstration of what identification uncertainty is. The randomized experimental comparison gives a positive earnings effect for the treated group. A naive comparison of the same treated group against a non-experimental comparison sample (in LaLonde's paper, a national survey extract) gives an estimate of the opposite sign, attributable to the selection that put the treated in the experiment and the comparison group in a different labour market altogether. The gap between the two is not a sampling fluctuation and will not average away; Dehejia and Wahba's (1999) reanalysis shows that recovering the experimental answer from the observational sample requires a covariate model that earns its claim, not a larger comparison group.

The published benchmark therefore supports two instruments, and the chapter builds both. The first is employment selection inside the file: when the outcome is recorded only for those with positive earnings, the treated and control arms have different positive-earnings shares, and Lee's (2009) monotone-trimming argument bounds the wage effect for the always-positive subpopulation by trimming the excess share from the tails of the higher-retention arm. The second is attrition in either arm, where the two-arm Manski bound prices what the missing units could have contributed by putting them at the extrema of the outcome's support.

The benchmark's own arithmetic is what makes the lesson durable, and it is the reason the numbers stay in the cited papers rather than in this chapter. Each instrument reports an interval and the premise that bought it; monotone selection narrows the attrition bound by the fraction of the file that is left unobserved, while the design error of comparing two populations costs more width than any premise buys back; no arrangement of the sample recovers the experimental answer from a frame that never contained it.

---

### The width is the result

Underdetermination has a number, and the number is a width. Once the estimate stops being a point, the honest report is the interval and the assumption that narrowed it, in that order. A reader who is given the point without the assumption has been given a decision disguised as a measurement.

The two-arm logic is the same at every level: the width of the bound is what the design bought, and the gain from a stronger premise is the difference between two widths. Monotone selection narrows an attrition bound by the unobserved fraction of the higher-retention arm, in the same archive where the design error costs more width than the premise buys back, which is why the premise is worth stating but not worth mistaking for the result.

The e-process sits on the same page for the same reason: an evidence measure that stays valid under any stopping rule is wider than a fixed-sample test, and the width is the price of the freedom to look. Neither instrument removes underdetermination; both report it.

### When the bound is the decision

The bound stops being a preliminary and becomes the object when the estimand is a decision with asymmetric cost. The procedure is to convert the interval into a decision through the declared threshold, say which end of the interval commands the action, and publish both the decision and the end that produced it.

### One discipline, three instruments

The instruments on this page share one discipline. Every time a premise is added, a width must shrink, and the size of the shrink is the price of the premise; every time a stopping rule is relaxed, a width must grow, and the size of the growth is the price of the freedom. A reader who is shown a number without either of those two movements has been shown a point estimate with the accounting removed. The width is therefore the only honest unit of account in this chapter, and the only one a reader can check without re-running anything.

## 3.5 The width at both ends, and the rule that arrives after the harm

### Two limits of the identification width

The form is $\Delta\_{\mathrm{id}} = (y\_{\max} - y\_{\min})\bigl[(1 - p\_{\mathrm{obs},1}) + (1 - p\_{\mathrm{obs},0})\bigr]$, and its two ends are the two ways of destroying a design. Full reporting in both arms, $p\_{\mathrm{obs},1} = p\_{\mathrm{obs},0} = 1$, gives $\Delta\_{\mathrm{id}} = 0$: identification is exact and the interval is the point. No reporting at all, $p\_{\mathrm{obs},1} = p\_{\mathrm{obs},0} = 0$, gives $\Delta\_{\mathrm{id}} = 2(y\_{\max} - y\_{\min})$, so twice the support is the ceiling of ignorance and no design can be worse than empty. In between, the width is linear in the unobserved share of each arm, and the proportionality is the whole content of the form: an archive that halves its attrition halves the identification width, and an archive that doubles its sample at constant attrition moves it not at all.

The second pair of limits belongs to the Imbens–Manski critical value, which interpolates between the one-sided and the two-sided quantile as the identification width grows against the sampling error: at $\Delta\_{\mathrm{id}}/\hat\sigma\_{\max} \to 0$ it returns $z\_{1-\alpha/2}$, and at $\Delta\_{\mathrm{id}}/\hat\sigma\_{\max} \to \infty$ it returns $z\_{1-\alpha}$. The direction is the one a reader does not expect, since the critical value becomes more permissive as the parameter becomes less identified: the interval has already paid for the uncertainty, and the sampling margin stops charging for it twice.

### A rule that arrives after the harm

The severity bridge licenses a substantive claim only when the lower Imbens–Manski bound clears the threshold, and as a rule about claims it is right. As a rule about decisions it waits for a bound that a cheap and reversible action protecting an irreversible harm does not need. Take an interval $\pi \in [\pi\_{\min}, \pi\_{\max}]$ for the probability of an irreversible harm, an action cost $C$, and a mitigated loss $\eta L$ per unit of probability. The severe rule demands a lower bound above its threshold, and the interval's lower end clears nothing worth naming; the cost-loss rule demands only $\pi > C/(\eta L)$. Whenever the interval's midpoint lies above that ratio, the expected loss of inaction exceeds the expected loss of action by a measurable margin, and the severe rule forgoes that saving for as long as the bound stays inconclusive. The British bovine-spongiform-encephalopathy episode is the dated instance: the lower bound never cleared the bar while the precaution stayed cheaper than the loss it avoided, and the announcement of March 1996 arrived after the exposure (Phillips et al. 2000). The lesson is internal to the chapter and it is narrow: a bound-clearing rule is a rule for claims under symmetric loss, and a decision with an irreversible downside belongs to the cost-loss comparison at the interval's own ends.

### Where sharpness stops

The thesis of this chapter is that the worst-case bounds are sharp, that nothing beyond the response indicator is assumed. The condition that invalidates it is a pair of premises the word sharpness hides: the outcome support must be bounded and known a priori, and the response must be monotone in treatment for the trimming to be legitimate at all. Violated, the first gives bounds at infinity and the second gives no interval. The procedure when the condition fails is to turn the truncation into an explicit measurement rather than an inherited default: declare the ceiling the institution actually records against (the claim limit, the payroll maximum, the treaty cap), tag it as an assumption, and report the width's sensitivity to it, which for the two-arm form is $\partial \Delta\_{\mathrm{id}}/\partial y\_{\max} = (1 - p\_{\mathrm{obs},1}) + (1 - p\_{\mathrm{obs},0})$, so every unit added to the ceiling widens the interval by the unobserved share of the two arms. Where no ceiling can be defended, report the sign of the bias and decline to quote a width at all.

## Bridge

This chapter delivers a set where the previous one delivered a criterion, and it leaves behind a
precondition: the set is computed inside a sampling frame the analyst can describe, a treated arm, a
control arm, an assumption about who is observed.

[From World to Archive — the Big Data Paradox](45-world-to-archive-big-data-paradox.md) breaks the
precondition. The archive is not a smaller world but a different object: it records what the institution
chose to write down, at the vintages it chose to publish. Meng's identity makes the damage exact, $\bar{Y}\_n - \bar{Y}\_N = \rho\_{R,Y}\sqrt{(1-f)/f}\mkern3mu\sigma\_Y$, so a
non-random missingness correlation moves the estimate without moving the sample size, and the operator
that prices it is the corrected effective sample size. What travels
forward is the effective sample size as a contraction of the nominal one. The debt is a denominator: an
$n\_{\mathrm{eff}}$ is only as good as the $N$ it is measured against, and [Chapter 45](45-world-to-archive-big-data-paradox.md) records the bitemporal
vintages on which that $N$ depends.
