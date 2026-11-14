# Appendix A — Mathematical Reference

<figure class="stoch-figure stoch-logo-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="660" height="132" viewBox="0 0 660 132" fill="none" role="img" aria-label="Stochology Appendix A — Mathematical Reference">
  <defs>
    <linearGradient id="apa-ring" x1="2" y1="78" x2="386" y2="78" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="38%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="68%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <radialGradient id="apa-glow" cx="50%" cy="50%" r="50%">
      <stop offset="0%" stop-color="#fbbf24" stop-opacity="0.24"/>
      <stop offset="50%" stop-color="#38bdf8" stop-opacity="0.14"/>
      <stop offset="82%" stop-color="#e11d48" stop-opacity="0.08"/>
      <stop offset="100%" stop-color="#090e1a" stop-opacity="0"/>
    </radialGradient>
    <linearGradient id="apa-triad-stroke" x1="18" y1="24" x2="110" y2="104" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.85"/>
      <stop offset="50%" stop-color="var(--logo-gold, #f59e0b)" stop-opacity="0.90"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.85"/>
    </linearGradient>
    <linearGradient id="apa-fill" x1="22" y1="30" x2="106" y2="98" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)" stop-opacity="0.26"/>
      <stop offset="48%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.28"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.26"/>
    </linearGradient>
    <linearGradient id="apa-ink-curve" x1="20" y1="66" x2="108" y2="66" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-ink, #0f172a)"/>
      <stop offset="50%" stop-color="var(--logo-sub, #1d4ed8)"/>
      <stop offset="100%" stop-color="var(--logo-ink, #0f172a)"/>
    </linearGradient>
    <radialGradient id="apa-pupil" cx="45%" cy="42%" r="58%">
      <stop offset="0%" stop-color="#020617"/>
      <stop offset="68%" stop-color="#0f172a"/>
      <stop offset="100%" stop-color="#451a03"/>
    </radialGradient>
    <linearGradient id="apa-gold-mira" x1="54" y1="48" x2="84" y2="82" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#fef08a"/>
      <stop offset="45%" stop-color="#fbbf24"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <radialGradient id="apa-gold-core" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#ffffff"/>
      <stop offset="35%" stop-color="#fde047"/>
      <stop offset="100%" stop-color="#d97706"/>
    </radialGradient>
    <radialGradient id="apa-node-well" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="var(--logo-well-hi, var(--elevated-bg, #ffffff))"/>
      <stop offset="100%" stop-color="var(--logo-node-bg, #e2e8f0)"/>
    </radialGradient>
    <linearGradient id="apa-ring-epi" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#7dd3fc"/>
      <stop offset="50%" stop-color="#2563eb"/>
      <stop offset="100%" stop-color="#1e3a8a"/>
    </linearGradient>
    <radialGradient id="apa-core-epi" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#bae6fd"/>
      <stop offset="45%" stop-color="#3b82f6"/>
      <stop offset="100%" stop-color="#1d4ed8"/>
    </radialGradient>
    <linearGradient id="apa-ring-vital" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#6ee7b7"/>
      <stop offset="50%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#065f46"/>
    </linearGradient>
    <radialGradient id="apa-core-vital" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#a7f3d0"/>
      <stop offset="45%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#047857"/>
    </radialGradient>
    <linearGradient id="apa-ring-gov" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#fda4af"/>
      <stop offset="50%" stop-color="#e11d48"/>
      <stop offset="100%" stop-color="#881337"/>
    </linearGradient>
    <radialGradient id="apa-core-gov" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#fecdd3"/>
      <stop offset="45%" stop-color="#f43f5e"/>
      <stop offset="100%" stop-color="#be123c"/>
    </radialGradient>
  </defs>
  <g>
    <circle cx="64" cy="66" r="48" fill="url(#apa-glow)"/>
    <circle cx="64" cy="66" r="40" stroke="url(#apa-triad-stroke)" stroke-width="2.2" stroke-dasharray="4 3"/>
    <polygon points="64,24 102,86 26,86" fill="url(#apa-fill)" stroke="url(#apa-ink-curve)" stroke-width="2.4" stroke-linejoin="round"/>
    <circle cx="64.00" cy="24.00" r="6.89" fill="url(#apa-node-well)" stroke="url(#apa-ring-epi)" stroke-width="2.54"/>
    <circle cx="64.00" cy="24.00" r="2.95" fill="url(#apa-core-epi)"/>
    <circle cx="26.00" cy="86.00" r="6.89" fill="url(#apa-node-well)" stroke="url(#apa-ring-vital)" stroke-width="2.54"/>
    <circle cx="26.00" cy="86.00" r="2.95" fill="url(#apa-core-vital)"/>
    <circle cx="102.00" cy="86.00" r="6.89" fill="url(#apa-node-well)" stroke="url(#apa-ring-gov)" stroke-width="2.54"/>
    <circle cx="102.00" cy="86.00" r="2.95" fill="url(#apa-core-gov)"/>
    <g>
      <circle cx="64.00" cy="66.00" r="6.96" fill="var(--logo-node-bg, #e2e8f0)"/>
      <circle cx="64.00" cy="66.00" r="6.02" fill="url(#apa-pupil)" stroke="url(#apa-gold-mira)" stroke-width="2.26"/>
      <line x1="54.98" y1="66.00" x2="60.05" y2="66.00" stroke="url(#apa-gold-mira)" stroke-width="2.26" stroke-linecap="round"/>
      <line x1="67.95" y1="66.00" x2="73.02" y2="66.00" stroke="url(#apa-gold-mira)" stroke-width="2.26" stroke-linecap="round"/>
      <line x1="64.00" y1="56.98" x2="64.00" y2="62.05" stroke="url(#apa-gold-mira)" stroke-width="2.26" stroke-linecap="round"/>
      <line x1="64.00" y1="69.95" x2="64.00" y2="75.02" stroke="url(#apa-gold-mira)" stroke-width="2.26" stroke-linecap="round"/>
      <circle cx="64.00" cy="66.00" r="2.07" fill="url(#apa-gold-core)"/>
    </g>
  </g>
  <g transform="translate(148, 0)">
    <text x="2" y="27" fill="var(--logo-sub, #1d4ed8)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.2">STOCHOLOGY · TECHNICAL REFERENCE</text>
    <text x="498" y="27" text-anchor="end" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.0">APPENDIX A · OPERATORS</text>
    <text x="0" y="59" fill="var(--logo-ink, #0f172a)" font-family="system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif" font-size="27.5" font-weight="800" letter-spacing="2.0">MATHEMATICAL REFERENCE</text>
    <line x1="2" y1="78" x2="384" y2="78" stroke="url(#apa-ring)" stroke-width="1.6" stroke-opacity="0.82"/>
    <circle cx="400" cy="78" r="6.2" stroke="var(--logo-gold, #f59e0b)" stroke-width="1.5" stroke-dasharray="2.2 1.6"/>
    <circle cx="400" cy="78" r="4.0" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.2"/>
    <circle cx="400" cy="78" r="1.8" fill="var(--logo-gold, #f59e0b)"/>
    <circle cx="415" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="430" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="445" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-green, #10b981)" stroke-width="2.0"/>
    <circle cx="445" cy="78" r="1.6" fill="var(--logo-green, #10b981)"/>
    <circle cx="460" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="475" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="490" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-crimson, #e11d48)" stroke-width="2.0"/>
    <circle cx="490" cy="78" r="1.6" fill="var(--logo-crimson, #e11d48)"/>
    <text x="2" y="104" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.8" font-weight="600" letter-spacing="0.65">CANONICAL OPERATORS I, II, AND III · CLOSED-FORM IDENTITIES · WORKED DERIVATIONS</text>
  </g>
  </svg>
</figure>

This appendix collects the Three Canonical Mathematical Operators with the identities and inequalities they are built from. Each entry names a result, gives its closed form and points to the literature that states it. No entry instantiates a formula numerically: where a reader needs a value, the value is the one the cited source reports, in the units the source uses. The operators are argued in Chapters 42–50; the pages below are the reference layer they presuppose.

---

## A.1 Filter I (Axis 1 — $S\_{\mathrm{epi}}$): Canonical Operator I — Partial-Identification Set & Effective Sample Contraction

$$\mathcal{I}\_{\mathrm{epi}}(\mathcal{D}\_t; \alpha) = \bigl[\hat{\theta}\_L - c\_{\mathrm{IM}}(\alpha)\hat{\sigma}\_L / \sqrt{n\_{\mathrm{eff}}^{\mathrm{var}}},\ \hat{\theta}\_U + c\_{\mathrm{IM}}(\alpha)\hat{\sigma}\_U / \sqrt{n\_{\mathrm{eff}}^{\mathrm{var}}}\bigr], \qquad n\_{\mathrm{eff}}^{\mathrm{var}} = \frac{n}{\mathrm{DEFF}\_{\mathrm{total}}}, \qquad \frac{1}{n\_{\mathrm{eff}}(\bar{\rho})} = \frac{1}{n\_{\mathrm{eff}}^{\mathrm{var}}} + \left(\frac{1 - f}{f}\right)\bar{\rho}^2$$

1. **Cromwell's Interior Probability Floor:**
   $$p \in [\varepsilon\_{\mathrm{floor}}, 1 - \varepsilon\_{\mathrm{floor}}], \qquad 0 < \varepsilon\_{\mathrm{floor}} < \frac{1}{2}$$

2. **Economic Cost–Loss Threshold, Pauker–Kassirer Expected-Regret Threshold & EVPI:**
   $$p^\ast\_{\mathrm{CL}} = \frac{C}{\eta L}, \qquad p^\ast\_{\mathrm{PK}} = \frac{C\_{\mathrm{FP}}}{C\_{\mathrm{FP}} + L\_{\mathrm{FN}}} = \frac{1}{1 + B\_{\mathrm{net}} / H\_{\mathrm{net}}}, \qquad \mathrm{EVPI}(\pi\_0) = \min\bigl(C, \pi\_0 \eta L\bigr) - \pi\_0 \min\bigl(C, \eta L\bigr) \ge 0$$
   The two thresholds are distinct, and substituting one for the other is an error. The **cost–loss** threshold $p^\ast\_{\mathrm{CL}} = C/(\eta L)$ divides the action cost by the efficacy-discounted loss avoided, and it travels with its EVPI. The **expected-regret** threshold $p^\ast\_{\mathrm{PK}} = C\_{\mathrm{FP}}/(C\_{\mathrm{FP}} + L\_{\mathrm{FN}})$, equivalently $1/(1 + B\_{\mathrm{net}}/H\_{\mathrm{net}})$ under the benefit/harm parametrization, is a statement about two error costs and says nothing about the cost–loss quotient.

3. **$e\text{-BH}$ False Discovery Rate Control Under Arbitrary Dependence (Wang & Ramdas, 2022):**
   $$k^\ast = \max\left\lbrace k \in \lbrace 1, \dots, K\rbrace : \frac{k E\_{(k)}}{K} \ge \frac{1}{\alpha}\right\rbrace$$

4. **Ville's Test Martingale Inequality (Ville, 1939):**
   $$E\_t = \prod\_{i=1}^t L\_i, \qquad \mathbb{P}\_{H\_0}\left(\sup\_{t \ge 1} E\_t \ge \frac{1}{\alpha}\right) \le \alpha$$

5. **Two-Arm Manski Non-Parametric ATE Bounds & Adaptive Imbens–Manski Confidence Interval:**
   $$\mathrm{ATE}\_L^{\mathrm{M}} = L\_1^{\mathrm{M}} - U\_0^{\mathrm{M}}, \qquad \mathrm{ATE}\_U^{\mathrm{M}} = U\_1^{\mathrm{M}} - L\_0^{\mathrm{M}}, \qquad \Phi\left(c\_{\mathrm{IM}} + \frac{\hat{\Delta}}{\hat{\sigma}\_{\max}}\right) - \Phi(-c\_{\mathrm{IM}}) = 1 - \alpha$$
   The interval is adaptive in the sense of Imbens & Manski. Its critical value widens with the width of the identification region, and an estimate that is bounded rather than point-identified pays for that width in coverage.

6. **Lee (2009) Monotone Attrition Trimming Fraction:**
   $$q = \frac{p\_T - p\_C}{p\_T} \quad (p\_T > p\_C)$$

7. **Peterson (1976) Competing-Risks Fréchet Bounds & Worst-Case Censoring Bounds:**
   $$\max\bigl(0, S\_1(t) + S\_2(t) - 1\bigr) \le S(t) \le \min\bigl(S\_1(t), S\_2(t)\bigr), \qquad 1 - \frac{d + c}{n} \le S\_{\mathrm{cens}}(t) \le 1 - \frac{d}{n}$$

8. **Keynes's (1921) Evidential Weight & Cost–Loss Reference Scale:**
   $$V\bigl(n\_{\mathrm{eff}}(\bar{\rho})\bigr) = \frac{n\_{\mathrm{eff}}(\bar{\rho})}{n\_{\mathrm{eff}}(\bar{\rho}) + n\_0} \in [0, 1), \qquad n\_0 := n\_{\mathrm{req}}(\alpha, \beta, p^\star) = \left(\frac{z\_{1-\alpha} + z\_{1-\beta}}{\varepsilon\_{\mathrm{rel}}}\right)^2 \frac{1 - p^\star}{p^\star} > 0, \qquad p^\star = \frac{C}{\eta L} \in (0, 1)$$
   evaluated at declared selection-defect bound $\bar{\rho} \ge |\rho\_{R,Y}|$, with $\alpha + \beta < 1$ and relative resolution $\varepsilon\_{\mathrm{rel}} > 0$.

9. **Meng's (2018) Exact Big Data Paradox Identity:**
   $$\bar{Y}\_n - \bar{Y}\_N = \rho\_{R,Y} \times \sqrt{\frac{1 - f}{f}} \times \sigma\_Y, \qquad n\_{\mathrm{Meng}}(\rho\_{R,Y}) = \frac{f}{1 - f}\frac{1}{\rho\_{R,Y}^2}$$

10. **Kish–Moulton Unequal-Cluster Design Effect:**
    $$\tilde{m} = \bar{m}\bigl(1 + \mathrm{CV}^2(m)\bigr), \qquad \mathrm{DEFF}(p) = 1 + (\tilde{m} - 1)\rho\_I(p), \qquad n\_{\mathrm{eff}}^{\mathrm{var}}(p) = \frac{n}{\mathrm{DEFF}(p)}$$

11. **Chao's (1984, 1987) Lower Bounds on the Uncounted Population (Abundance vs. $t$-List Incidence):**
    $$\hat{n}\_{00,\mathrm{Chao1}} = \frac{f\_1^2}{2 f\_2}, \qquad \hat{n}\_{00,\mathrm{Chao2}}(t) = \frac{t - 1}{t}\mkern3mu\frac{f\_1^2}{2 f\_2}, \qquad \hat{n}\_{00,\mathrm{Chao2}}(2) = \frac{f\_1^2}{4 f\_2} = \frac{(n\_{10} + n\_{01})^2}{4 n\_{11}} \ge \hat{n}\_{00,\mathrm{LP}}$$

---

## A.2 Filter II (Axis 2 — $S\_{\mathrm{vital}}$): Canonical Operator II — Non-Ergodic Reserve & Survival-Time Barrier

$$dR\_t = \bigl(u\_t - D\_t(\alpha\_{\mathrm{Hill}}) - \Delta\_J - \tfrac{1}{2}f\_t^2\sigma^2\bigr)dt, \qquad \mathbb{E}[W\_q] \approx \left(\frac{c\_a^2 + c\_s^2}{2}\right)\frac{\rho\_t}{1 - \rho\_t}\tau\_s, \qquad \mathrm{TTS}\_{\mathrm{survival}}(R\_t) = \frac{B\_t}{\dot{D}\_{\mathrm{deficit}}} \ge \mathrm{TTR}\_{\mathrm{eff}}(\mathcal{I}\_t)$$

12. **Jensen's Convexity Gap & Empirical CRPS:**
    $$\Delta\_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X]), \qquad \mathrm{CRPS}(F, y) = \mathbb{E}\_F|X - y| - \frac{1}{2}\mathbb{E}\_F|X - X^\prime|$$

13. **Hill's (1975) Tail Index Estimator:**
    $$\hat{\alpha}\_{k,n} = \left(\frac{1}{k}\sum\_{i=1}^k \ln\frac{X\_{(n - i + 1)}}{X\_{(n - k)}}\right)^{-1}$$

14. **Itô Time-Average Growth & Baker–McHale (2013) Epistemic Kelly Allocation:**
    $$g\_{\mathrm{time}}(f) = r + f(\mu - r) - \frac{1}{2}f^2\sigma^2, \qquad k\bigl(n\_{\mathrm{eff}}^{\mathrm{var}}\bigr) = \frac{1}{1 + 1 / n\_{\mathrm{eff}}^{\mathrm{var}}}, \qquad f^\ast\_{\mathrm{epistemic}} = k\bigl(n\_{\mathrm{eff}}^{\mathrm{var}}\bigr)\frac{\hat{\mu} - r}{\sigma^2}$$

15. **Kingman's (1961, 1962) $GI/G/1$ Queue Bound & Heavy-Traffic Waiting Time, Necessary Simchi-Levi Survival Condition & Dialogic Basin:**
    $$\mathbb{E}[W\_q] \le \left(\frac{c\_a^2/\rho^2 + c\_s^2}{2}\right)\frac{\rho}{1 - \rho}\tau\_s, \qquad \mathbb{E}[W\_q] \approx \left(\frac{c\_a^2 + c\_s^2}{2}\right)\frac{\rho}{1 - \rho}\tau\_s, \qquad \mathrm{TTR}\_{\mathrm{recovery}} := \mathrm{TTR}\_{\mathrm{eff}}(\mathcal{I}\_t) = \tau\_{\mathrm{detect}} + \tau\_{\mathrm{delib}}(\mathcal{I}\_t) + \tau\_{\mathrm{actuate}}, \qquad \mathrm{TTS}\_{\mathrm{survival}} = \frac{B\_t}{\dot{D}\_{\mathrm{deficit}}} \ge \mathrm{TTR}\_{\mathrm{eff}}(\mathcal{I}\_t), \qquad \Theta\_{\mathrm{basin}} = \max\left(0, \frac{B\_{\mathrm{tol}} - (d\_{\mathrm{cum}} + c\mkern3mu\sigma\_{\mathrm{chan}})}{B\_{\mathrm{tol}}}\right)$$
    where $\mathrm{TTR}\_{\mathrm{eff}}(\mathcal{I}\_t)$ instantiates $\mathrm{TTR}\_{\mathrm{recovery}}$ across three non-overlapping phases: $\tau\_{\mathrm{detect}}$ is physical and evidential detection latency up to threshold alarm firing (under $\delta\_{\mathrm{status}} = 0$), $\tau\_{\mathrm{delib}}(\mathcal{I}\_t)$ is institutional and cognitive deliberation latency conditioned on the sealed state $\mathcal{I}\_t$ (cognitive service/queueing $\tau\_{\mathrm{cog}} = B\_{\mathrm{audit}}/C\_{\mathrm{human}}$, any status-discount drag $\delta\_{\mathrm{status}} > 0$, and review/retraining delay up to authorization), and $\tau\_{\mathrm{actuate}}$ is post-authorization execution and queue-drain latency (scaling with $\mathbb{E}[W\_q]$).

---

## A.3 Filter III (Axis 3 — $S\_{\mathrm{gov}}$): Canonical Operator III — Bitemporal Accountability & Performative Integrity Gate

$$\mathcal{O}\_{\mathrm{gov}}(\pi\_t, \mathcal{I}\_t) = \min\bigl(g\_{\text{ex-ante}, t}(\text{CORP}, \text{ICP}, \text{Perdomo}),\mkern5mu g\_{\mathrm{kairos}, t}(\text{List–Pettit}, \text{Fricker}, \text{Bateson}),\mkern5mu g\_{\text{ex-post}, t}(\text{Sosa}, \text{Holmström}, \text{Vault})\bigr)$$

16. **Isotonic CORP-PAV Brier Score Decomposition (Dimitriadis, Gneiting & Jordan, 2021):**
    $$\mathrm{BS} = \mathrm{REL}\_{\mathrm{CORP}} - \mathrm{RES}\_{\mathrm{CORP}} + \mathrm{UNC}\_0$$

17. **Perdomo et al. (2020) Performative Contraction & Linear-Response Equilibria ($L\_2$ Regularization $\lambda \ge 0$, $\beta = 1, \gamma = 1 + \lambda$):**
    $$\kappa\_{\mathrm{RRM}} = \frac{|\varepsilon| \beta}{\gamma} = \frac{|\varepsilon|}{1 + \lambda} < 1, \qquad \theta\_{\mathrm{PS}} = \frac{\mu\_0}{1 + \lambda - \varepsilon}, \qquad \theta\_{\mathrm{PO}} = \frac{(1 - \varepsilon)\mu\_0}{(1 - \varepsilon)^2 + \lambda}$$

18. **Chouldechova's (2017) Algebraic Fairness Trilemma:**
    $$\frac{\mathrm{FPR}\_A}{\mathrm{FPR}\_B} = \frac{p\_A / (1 - p\_A)}{p\_B / (1 - p\_B)} \quad \text{when } \mathrm{PPV}\_A = \mathrm{PPV}\_B \in (0, 1),\mkern5mu \mathrm{TPR}\_A = \mathrm{TPR}\_B > 0,\mkern5mu p\_A, p\_B \in (0, 1)$$

19. **Fricker's Status-Based Testimonial Discount & Corroborated Sequential Filtering on Beer's Algedonic Channel:**
    $$\tau\_{\mathrm{alarm}}(\delta\_{\mathrm{status}}) = \left\lceil\frac{\ln(1/\alpha)}{(1 - \delta\_{\mathrm{status}})\ln e\_t}\right\rceil \le \mathrm{TTS}\_{\mathrm{survival}}, \qquad \delta\_{\mathrm{status}} = 0 \text{ with } \Lambda\_t = \prod\_{s \le t} e\_s \ge E\_{\mathrm{trip}} := \frac{1}{\alpha}, \quad \tau\_{\mathrm{cog}} = \frac{B\_{\mathrm{audit}}}{C\_{\mathrm{human}}}$$
    where $C\_{\mathrm{human}}$ is bounded near $10\text{ bits/s}$ (Zheng & Meister 2025).

20. **Bateson's Double-Bind Trap & Simon–Girard Scapegoat Blame Transfer:**
    $$\mathrm{DB}\_t = \beta\_{\mathrm{proxy}, t} \cdot \delta\_{\mathrm{status}, t} \cdot \bigl(1 - \mathbf{1}\lbrace\text{brake}\_t\rbrace\bigr) \in [0, 1], \qquad \Delta\_{\mathrm{scapegoat}} = \max\bigl(0, b\_{\mathrm{frontline}} - L\_{\mathrm{oper}}\bigr)$$

---

## A.4 Leontief Structural Complementarity of the Nuclear Triad

21. **Leontief Structural Complementarity:**
    $$M\_{\mathrm{Leontief}} = \min(S\_{\mathrm{epi}}, S\_{\mathrm{vital}}, S\_{\mathrm{gov}}) \in [0, 1]$$
    where on a sequential trajectory of horizon $T$:
    $$S\_{\mathrm{epi}} = \inf\_{1 \le t \le T} S\_{\mathrm{epi}, t}\bigl(\mathrm{Cov}\_T, V(n\_{\mathrm{eff}, t}), \Delta\_{\mathrm{id}, t}\bigr), \quad S\_{\mathrm{vital}} = \inf\_{1 \le t \le T} \min\bigl(S\_{\mathrm{tail}, t}, S\_{\mathrm{time}, t}, S\_{\mathrm{surv}, t}\bigr), \quad S\_{\mathrm{gov}} = \inf\_{1 \le t \le T} \min\bigl(g\_{\text{ex-ante}, t}, g\_{\mathrm{kairos}, t}, g\_{\text{ex-post}, t}\bigr)$$
    The minimum is not a weighted sum and carries no exchange rate: a margin that is intact cannot purchase a margin that is exhausted, so the aggregate reports the weakest axis rather than an average of the three. The fold over steps is an infimum for the same reason. The predicate reads a floor, a step at which a margin is exhausted has already returned the verdict, and no later surplus redeems it; an average would let one absorbed step be paid for by the rest of the horizon. The infimum also commutes with every strictly increasing recoding of an axis, which an average does only for the affine one, so the fold stays inside the group under which the chart of the next paragraph is admissible. That invariance covers strictly increasing recoding within a site and within an axis. The cross-axis aggregate is invariant only under a common recoding of all three axes, because the axis comparison is made across native units. The comparability the minimum uses is ordinal and is not an exchange rate. An exchange rate would price one axis against another and license compensation between them; the minimum refuses compensation, and naming the weakest axis therefore compares the axes without pricing one in the other's units.

The quantities the fold touches do not share a scale, and naming the scale of each is what keeps the volume below from overreaching. A site margin $m\_j$ and its declared threshold $\tau\_j$ live on a single ordinal scale per site: the predicate compares the two and compares nothing else, so the admissible recoding is one strictly increasing map applied to both at once (Stevens 1946; Krantz, Luce, Suppes & Tversky 1971). What singles out the point at which a margin is exhausted is the predicate, not the scale, which is why a collapse is a verdict and not a small number. The axis fold $s\_a = \min\_{j \in a} m\_j$ stays inside one axis and therefore inside that axis's own unit, and no ratio between two axes is a ratio of like quantities. The unit-cube score is a chart on the axis reading in that unit — the fold $s\_a$ of the predicate, written through the named components above — so $S\_a = \varphi\_a(s\_a)$ with $\varphi\_a$ strictly increasing and fixed once per axis. Fixing the chart once is what narrows the group. The native margin admits any strictly increasing recoding; the score admits the positive affine maps of one axis at a time, and it is under that narrower group that $S\_a$ is an interval representation and a mean over $t$ at fixed $a$ is licensed as a description of the trajectory, reported beside the fold and never an argument of it, because an average is not monotone and does not commute with a nonlinear chart. That the smallest of the three scores names the weakest axis, rather than the smallest of three study-specific summaries, is a calibration premise of the display and not a consequence of it. No coordinate of the cube can carry a negative margin, so the volume below is a statement about the interior and the boundary band, while a collapse remains a verdict of the predicate at a named site. Where an axis's per-step margin is itself a set, because the defect it rests on is declared rather than measured, the fold reads the set's worst point: the verdict is monotone in every margin, so a set and its lower endpoint return the same verdict, and reading the endpoint is the only reading the display can legitimately show.

The chart change is diagonal, and its Jacobian is what a change of chart costs the volume. Recode one axis alone as $S\_a \mapsto \varphi\_a(S\_a)$ with $\varphi\_a$ continuously differentiable, strictly increasing and anchored at the top of the box, $\varphi\_a(1) = 1$. Then

$$J\_\varphi = \operatorname{diag}\bigl(\varphi\_{\mathrm{epi}}'(S\_{\mathrm{epi}}),\mkern5mu \varphi\_{\mathrm{vital}}'(S\_{\mathrm{vital}}),\mkern5mu \varphi\_{\mathrm{gov}}'(S\_{\mathrm{gov}})\bigr), \qquad \det J\_\varphi = \prod\_{a} \varphi\_a'(S\_a) > 0,$$

the viable box $\lbrace{}S : S\_a \ge \tau \text{ for every } a\rbrace$ maps to $\lbrace{}S' : S'\_a \ge \varphi\_a(\tau)\rbrace$, and

$$\mathrm{Vol}(\tau; \varphi) = \prod\_a \bigl(1 - \varphi\_a(\tau)\bigr) = \int\_{[\tau, 1]^3} \lvert \det J\_\varphi \rvert \mkern3mu dS.$$

On the affine subgroup that remains inside $[0, 1]^3$, namely $\varphi\_a(x) = \alpha\_a x + (1 - \alpha\_a)$ with $0 < \alpha\_a \le 1$ (so $\varphi\_a(0) = 1 - \alpha\_a \ge 0$, with $\alpha\_a = 1$ the unique affine map fixing both $0$ and $1$), the factor is constant and carries $[\tau, 1]$ to $[1 - \alpha\_a(1 - \tau), 1] \subseteq [0, 1]$; the volume is multiplied by $\prod\_a \alpha\_a$ — the determinant — and equals $(1-\tau)^3$ only when every axis keeps the unit chart. Under a wider strictly increasing recoding the volume is $\prod\_a (1 - \varphi\_a(\tau))$: the box survives as a box while its measure does not, and recoding one axis alone moves the identity of the weakest axis with the chart, with no new evidence and no change in the world. The volume is therefore a Lebesgue measure taken in the declared chart, $V\_{\min}(\tau) = (1-\tau)^3$ being its value in the unit chart, and not an invariant of the institution.

---

## A.5 The Grammar of Aim — Syntax Layer and Acceptance Predicate

$$\mathcal{S} = (\sigma\_1, \dots, \sigma\_N), \quad \sigma\_t = (\mathbf{u}\_t, \mathbf{w}\_t, \mathbf{m}\_t), \quad \mathcal{A}(\mathcal{S}; \boldsymbol{\tau}) \in \lbrace\mathrm{Interior}, \mathrm{Boundary}, \mathrm{Collapse}, \mathrm{Unparsable}\rbrace$$

22. **Acceptance Predicate & Fixed Evaluation Order:**
    Given a trajectory sentence $\mathcal{S}$ over the seven guard sites $T\_1, \dots, T\_7$ partitioned $3/2/2$ across $(S\_{\mathrm{epi}}, S\_{\mathrm{vital}}, S\_{\mathrm{gov}})$, the acceptance predicate evaluates verdicts in strict hierarchical order:
    1. **Quarantine:** If any margin $m\_j$ is non-finite ($\mathrm{NaN}, \pm\infty$) or a circuit transit is unguarded, $\mathcal{A}(\mathcal{S}; \boldsymbol{\tau}) = \mathrm{Unparsable}$;
    2. **Collapse:** If $\exists j \in \lbrace 1, \dots, 7\rbrace$ such that $m\_j < 0$, then $\mathcal{A}(\mathcal{S}; \boldsymbol{\tau}) = \mathrm{Collapse}(\text{Proposition}, \text{at } T\_j, \text{step } t)$;
    3. **Boundary:** If $\exists j \in \lbrace 1, \dots, 7\rbrace$ such that $0 \le m\_j < \tau\_j$, then $\mathcal{A}(\mathcal{S}; \boldsymbol{\tau}) = \mathrm{Boundary}$;
    4. **Interior:** If $\forall j \in \lbrace 1, \dots, 7\rbrace, m\_j \ge \tau\_j$, then $\mathcal{A}(\mathcal{S}; \boldsymbol{\tau}) = \mathrm{Interior}$.

    The predicate is evaluated on one step at a time, and its arguments are the sealed fields of that step. What it folds is that step's axis minimum, not the minimum of a running trajectory. A verdict recomputed as the trajectory accumulates is the diagnosis, and it is not this predicate.

23. **Non-Commutation Property of Aim:**
    $$\mathcal{A}(\mathbf{u}, \mathbf{w}, \mathbf{m}; \boldsymbol{\tau}) = \mathcal{A}(\mathbf{u}, \mathbf{w}^\prime, \mathbf{m}; \boldsymbol{\tau}) \quad \forall \mathbf{w}, \mathbf{w}^\prime.$$
    The acceptance predicate is invariant to the contingency tape $\mathbf{w}\_t$ (*skopós*). No accidental external success can redeem depleted guard margins $\mathbf{m}\_t$ or an uncalibrated control tape $\mathbf{u}\_t$ (*télos*). Antipater's separation of aim from external outcome is an exact mathematical property of the grammar.

---

## A.6 The Multilayer Knowledge Graph ($\mathcal{G}\_1 \leftrightarrow \mathcal{G}\_2$)

$$\binom{7}{3} = 12 + 9 + 9 + 4 + 1 = 35, \qquad \mathcal{H}(L\_i) = \min\_{L\_j \in \mathrm{Cone}(L\_i)} M\_{\mathrm{Leontief}}(L\_j)$$

24. **Vandermonde Partition of Triadic Exposure Regimes:**
    Partitioning $\binom{7}{3} = 35$ unique guard triples across the Nuclear Triad with partition sizes $(3, 2, 2)$ for $(S\_{\mathrm{epi}}, S\_{\mathrm{vital}}, S\_{\mathrm{gov}})$ decomposes into five mutually exclusive regimes:
    $$\binom{7}{3} = \sum\_{k\_1+k\_2+k\_3=3} \binom{3}{k\_1}\binom{2}{k\_2}\binom{2}{k\_3} = 12 + 9 + 9 + 4 + 1 = 35.$$
    - **Trans-Triadic Keystone ($k\_1=1, k\_2=1, k\_3=1$):** $\binom{3}{1}\binom{2}{1}\binom{2}{1} = 3 \times 2 \times 2 = 12$ orders;
    - **Governance Axial Exposure ($k\_3=0, k\_1 \ge 1, k\_2 \ge 1$):** $\binom{3}{2}\binom{2}{1}\binom{2}{0} + \binom{3}{1}\binom{2}{2}\binom{2}{0} = 6 + 3 = 9$ orders;
    - **Vital Axial Exposure ($k\_2=0, k\_1 \ge 1, k\_3 \ge 1$):** $\binom{3}{2}\binom{2}{0}\binom{2}{1} + \binom{3}{1}\binom{2}{0}\binom{2}{2} = 6 + 3 = 9$ orders;
    - **Epistemic Axial Exposure ($k\_1=0$):** $\binom{3}{0}\binom{2}{2}\binom{2}{1} + \binom{3}{0}\binom{2}{1}\binom{2}{2} = 2 + 2 = 4$ orders;
    - **Pure Epistemic Concentration ($k\_1=3$):** $\binom{3}{3}\binom{2}{0}\binom{2}{0} = 1$ order.

25. **DAG Ancestor Cone Leontief Homomorphism:**
    $$\mathcal{H}(L\_i) = \min\_{L\_j \in \mathrm{Cone}(L\_i)} M\_{\mathrm{Leontief}}(L\_j) = \min\_{L\_j \in \mathrm{Cone}(L\_i)} \min\bigl(S\_{\mathrm{epi}}(L\_j), S\_{\mathrm{vital}}(L\_j), S\_{\mathrm{gov}}(L\_j)\bigr).$$
    **Homomorphism Monotonicity:** If any basal order in the dependency cone collapses ($M\_{\mathrm{Leontief}}(L\_{\mathrm{basal}}) = 0$), then $\mathcal{H}(L\_i) = 0$ for all downstream dependent disciplines. The transfer is a model reading, not a measurement: it says what follows from the graph's dependency relation, and it does not claim that a cumulative margin has been observed. Higher-order social and legal institutions cannot survive when foundational physical or epidemiological margins fail.

    The four folds are one fold, and it is the minimum that makes them one. Sites to axis, steps to trajectory, axes to aggregate and cone to order are each a minimum taken in native units, and the minimum is associative, commutative and idempotent. Over the finite index set of triples of site, step and ancestral order — finite because the seven sites, the horizon and the thirty-five orders are —
    $$\min\_{a}\ \min\_{1 \le t \le T}\ \min\_{j \in \mathcal{S}\_a}\ \min\_{L\_j \in \mathrm{Cone}(L\_i)} m\_{j,t}(L\_j) = \min\_{(j,\mkern3mut,\mkern3muL) \in J\_i} m\_{j,t}(L), \qquad J\_i = \mathcal{S} \times \lbrace1,\dots,T\rbrace \times \mathrm{Cone}(L\_i),$$
    the three site sets $\mathcal{S}\_a$ partition the seven sites, so the outer minimum over axes reindexes rather than folds a fourth time. An average in place of the trajectory minimum is not idempotent and does not commute with the axis minimum, and a minimum of averages is at least the average of minima:
    $$\min\_a \frac{1}{T}\sum\_{t=1}^{T} S\_{a,t} \ \ge\ \frac{1}{T}\sum\_{t=1}^{T} \min\_a S\_{a,t} \ \ge\ \min\_{(j,\mkern3mut,\mkern3muL) \in J\_i} m\_{j,t}(L).$$
    Both members of this chain are read in the unit chart of the paragraph after item 21, where an axis score and the margin it folds are the same number, $S\_{a,t} = \min\_{j \in \mathcal{S}\_a} m\_{j,t}$. Hence $\mathcal{H}(L\_i) \le \frac{1}{T}\sum\_{t=1}^{T} \min\_a S\_{a,t} \le \min\_a \frac{1}{T}\sum\_{t=1}^{T} S\_{a,t}$. An averaged fold is equal to or above the minimum at every profile, so it can only report more slack than the trajectory sustained — a larger number is read as a larger margin — and the minimum is the conservative reading because it is the least flattering one, not because the average is worse. With the fold taken as a minimum the identity is exact and unconditional: the equality between the fold of item 21 and the meet over the index set needs no further premise.

    **Finite-sample downward bias of noisy minimum estimators and simultaneous uncertainty bands (G4).** The trajectory inequality above assumes the coordinates are known. When margins or threshold parameters $\theta = (\theta\_1, \dots, \theta\_k) \in \mathbb{R}^k$ are instead estimated from a finite sample of sampling-variance effective size $n\_{\mathrm{eff}}^{\mathrm{var}}$ by integrable unbiased estimators $\hat{\theta} = (\hat{\theta}\_1, \dots, \hat{\theta}\_k)$ with $\mathbb{E}[\hat{\theta}\_m] = \theta\_m$, plugging the estimates into the minimum introduces systematic negative bias. Because $x \mapsto \min\_{1 \le m \le k} x\_m$ is the pointwise minimum of linear projections, it is concave on $\mathbb{R}^k$, and Jensen's inequality gives
    $$\mathbb{E}\mkern-3mu\left[\min\_{1 \le m \le k} \hat{\theta}\_m\right] \le \min\_{1 \le m \le k} \mathbb{E}[\hat{\theta}\_m] = \min\_{1 \le m \le k} \theta\_m.$$
    Let $m^\ast \in \arg\min\_{1 \le m \le k} \theta\_m$ attain the population minimum. Writing $\min\_m \hat{\theta}\_m = \hat{\theta}\_{m^\ast} - \max\_{m \ne m^\ast} (\hat{\theta}\_{m^\ast} - \hat{\theta}\_m)\_+$ and taking expectations yields the exact downward-bias representation
    $$b\_{\min}(\hat{\theta}) := \mathbb{E}\mkern-3mu\left[\min\_{1 \le m \le k} \hat{\theta}\_m\right] - \min\_{1 \le m \le k} \theta\_m = -\mkern3mu\mathbb{E}\mkern-3mu\left[\max\_{m \ne m^\ast} \bigl(\hat{\theta}\_{m^\ast} - \hat{\theta}\_m\bigr)\_+\right] \le 0,$$
    which is **strictly negative** whenever at least one competing coordinate $m \ne m^\ast$ has $\mathbb{P}(\min\_{m \ne m^\ast} \hat{\theta}\_m < \hat{\theta}\_{m^\ast}) > 0$ (non-degenerate sampling overlap around the population minimum). In particular, for two tied independent Gaussian estimators $\hat{\theta}\_m \sim \mathcal{N}(\theta\_0, \sigma\_m^2)$ ($m \in \lbrace1, 2\rbrace$, $\sigma\_1^2 + \sigma\_2^2 > 0$), $\min(\hat{\theta}\_1, \hat{\theta}\_2) = \frac{1}{2}(\hat{\theta}\_1 + \hat{\theta}\_2) - \frac{1}{2}|\hat{\theta}\_1 - \hat{\theta}\_2|$ gives $b\_{\min}(\hat{\theta}) = -\sqrt{(\sigma\_1^2 + \sigma\_2^2)/(2\pi)} < 0$; more generally, for $k \ge 2$ tied i.i.d. estimators $\hat{\theta}\_m \stackrel{\text{i.i.d.}}{\sim} \mathcal{N}(\theta\_0, \sigma^2 / n\_{\mathrm{eff}}^{\mathrm{var}})$ ($\sigma > 0$) written as $\theta\_0 + (\sigma/\sqrt{n\_{\mathrm{eff}}^{\mathrm{var}}})Z\_m$, standard Gaussian maximal bounds place the downward shift in
    $$-\frac{\sigma}{\sqrt{n\_{\mathrm{eff}}^{\mathrm{var}}}}\sqrt{2\ln k} \le \mathbb{E}\mkern-3mu\left[\min\_{1 \le m \le k} \hat{\theta}\_m\right] - \theta\_0 < 0,$$
    showing that plug-in minimum bias grows with both estimation dispersion $\sigma/\sqrt{n\_{\mathrm{eff}}^{\mathrm{var}}}$ and coordinate multiplicity $k$.

    To prevent sampling overlap from being confused with physical depletion, the plug-in point minimum $\min\_m \hat{\theta}\_m$ must be distinguished from a **simultaneous uncertainty band (G4)**. Given a joint confidence rectangle $\mathcal{R}\_{1-\alpha} = \prod\_{m=1}^k [\hat{\theta}\_{m,L}, \hat{\theta}\_{m,U}]$ with simultaneous coverage $\mathbb{P}(\theta \in \mathcal{R}\_{1-\alpha}) \ge 1 - \alpha$ (for example, via Bonferroni allocation at level $\alpha / k$ per coordinate or a joint studentized maximum-modulus band), monotonicity of $x \mapsto \min\_{1 \le m \le k} x\_m$ ($x\_m \le y\_m \ \forall m \implies \min\_m x\_m \le \min\_m y\_m$) propagates the joint rectangle directly to the minimum:
    $$\theta \in \prod\_{m=1}^k [\hat{\theta}\_{m,L}, \hat{\theta}\_{m,U}] \implies \min\_{1 \le m \le k} \hat{\theta}\_{m,L} \le \min\_{1 \le m \le k} \theta\_m \le \min\_{1 \le m \le k} \hat{\theta}\_{m,U},$$
    so the induced interval $\bigl[\min\_{1 \le m \le k} \hat{\theta}\_{m,L},\mkern3mu \min\_{1 \le m \le k} \hat{\theta}\_{m,U}\bigr]$ carries simultaneous coverage at least $1 - \alpha$ for the true population minimum. Its lower endpoint supplies the conservative floor across $\mathcal{R}\_{1-\alpha}$ (and preserves ordinal ranking under any common strictly increasing rescaling $\phi$ of the coordinates), while its width $\min\_m \hat{\theta}\_{m,U} - \min\_m \hat{\theta}\_{m,L}$ separates estimation uncertainty from an unambiguous threshold breach ($\min\_m \hat{\theta}\_{m,U} < \tau$).

    The reading is taken in native units, so it presumes the three axis units comparable on one declared ordinal scale. That is the premise item 21 records when it says the smallest of the three scores names the weakest axis. Under it a single strictly increasing map applied to all three axes carries thresholds to their images and leaves both the verdict and the ordering of the axes fixed, while recoding one axis alone is not a reparametrisation of the aggregate but a different reading of the institution. Within one axis the map acts on every site of that axis and commutes with $\min\_{j \in \mathcal{S}\_a}$, which is why the per-site predicate is invariant under a strictly increasing recoding at each site independently while the fold over axes is invariant only under the common one.

    $\mathcal{H}(L\_i) \le M\_{\mathrm{Leontief}}(L\_i)$ because a node lies in its own cone, so an order may read Collapse while its primary site reads Interior. That is what transmission from a basal ancestor is, and it never runs the other way. The seven-site reading is an upper bound on the order's verdict rather than the binding one, and the cone moves that verdict exactly when the node is not itself the binding member of its cone, $\min\_{L \in \mathrm{Cone}(L\_i)} M(L) < M(L\_i)$, a collapsed ancestor over a node already at zero changing nothing. A triple whose signature carries a zero has no floor on that axis, and the totality requirement of the grammar already refuses such a transit as a derivation rather than as a weak one, which is why the five Vandermonde classes name those cases axial exposures—recording the absence of a local guard on that axis when isolated from upstream orders in $\mathcal{G}\_1$.

    The volume of the paragraph after item 21 is the box $\lbrace{}S\_a \ge \varphi\_a(\tau)\rbrace$ in the three-coordinate chart, its side lengths fixed by the chart and the thresholds and by nothing else. A fold over more sites, steps or orders adds comparisons in native units and adds no coordinate, hence no factor and no side length, which is the dimension of the display rather than any assumption of independence. A threshold declared per site moves where along the axis the reading is taken — the box presumes the thresholds declared jointly, as that paragraph states — and a negative margin leaves the cube altogether for the predicate, neither of them changing the box. No volume accumulates across the thirty-five orders: orders differ in which sites they read, which moves the point and can move the verdict by transmission, never in how many coordinates the chart has. What a collapse does inside the cube is move the point out of it, and a collapse in an ancestor lowers $\mathcal{H}$ without touching the box at all.
26. **Motter–Lai (2002) Cascade Breakdown Condition:**
    $$C\_i = (1 + \alpha\_{\mathrm{tol}}) L\_i(\mathcal{G}\_0), \qquad \alpha\_{\mathrm{tol}} = \frac{1 - \rho\_i}{\rho\_i}, \qquad \mathcal{F}(t + 1) = \mathcal{F}(t) \cup \bigl\lbrace{}j \notin \mathcal{F}(t) : L\_j\bigl(\mathcal{G}\_0 \setminus \mathcal{F}(t)\bigr) > C\_j\bigr\rbrace.$$
    Here $L\_j(\mathcal{G}\_0 \setminus \mathcal{F}(t))$ is the load recomputed on the surviving graph after the failed set $\mathcal{F}(t)$ is removed — shortest-path betweenness in Motter & Lai (2002), and equal redistribution across $z$ surviving neighbours ($L\_0 + L\_0/z > C$ once baseline utilisation $\rho > z/(z + 1)$, equivalently $\alpha\_{\mathrm{tol}} < 1/z$) in the homogeneous local benchmark of equal loads and equal capacities that [Chapter 53](53-layer-macro-coupled-topology-order.md) instantiates. When $\alpha\_{\mathrm{tol}}$ lies below the topological cascade threshold $\alpha\_c$, removing a high-load node triggers iterative overload propagation across the coupled graph.

## A.7 Reception-Side Margin — The Fourth Operator, and Why It Is Not a Fourth Axis

The three axes of the triad are the axes the agent controls. A decision is also received, and the reception carries a margin of its own: the distance between the exposure a message creates and the tolerance of the party who has to absorb it. That margin is not an axis. It cannot substitute for the epistemic, the vital or the governance margin, so the triad's minimum is computed over three scores and not four.

The closure has the same shape as the survival barrier, and stating it exactly is what keeps the analogy from being decorative. Write $T$ for the tolerance band, $\delta$ for the drift rate and $s$ for the shock intensity, with $c$ a fixed positive coefficient on the shock; the dialogue basin is $\Theta=\mathrm{clamp}\bigl((T-(\delta+c\mkern3mus))/T, 0, 1\bigr)$: the slack $T-s$, discounted by the drift $\delta$ and by the shock coefficient. Three cases follow, and only the first is comfortable. $T \gg s$ gives a margin that survives a sequence of shocks, $T \approx s$ gives a margin that survives one, and $T < s$ gives no basin at all, which is a missing margin and not a small one.

The distinction between this operator and the vital barrier is what keeps the appendix honest. The vital barrier compares a time to survive with a time to recover; the reception operator compares a tolerance with an exposure. Both are inequalities between a capacity and a demand, and neither is a probability. A reception margin of zero is not a fifty per cent chance of rupture. It is a boundary at which the next increment is absorbed entirely by the receiver rather than by the reserve.

Two consequences are worth recording, because they are the ones a chapter citing this appendix is entitled to use. The reception margin is not conserved: it can be raised by lowering the exposure as well as by raising the tolerance, and the two moves have different costs and different institutions. And it does not enter the Leontief minimum. A reader who adds it as a fourth axis has changed the aggregation rule, and nothing in this appendix licenses that change. What the appendix licenses, and all it licenses, is the reading of a reception failure as a capacity-demand inequality rather than as a matter of taste.
