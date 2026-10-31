# Appendix A — Mathematical Reference

This appendix summarizes the core mathematical identities and inequalities implemented in pure Rust (`std` only) across [`crates/stochology/src/`](../crates/stochology/src/).

---

## A.1 Filter I — Inquiry, Framing, Martingales, and Partial Identification

1. **Cromwell's Interior Probability Floor ([`inquiry::cromwell_clamp`](../crates/stochology/src/inquiry.rs#L35-L38)):**
   $$p \in [\varepsilon\_{\mathrm{floor}},\\, 1 - \varepsilon\_{\mathrm{floor}}], \qquad 0 < \varepsilon\_{\mathrm{floor}} < \frac{1}{2}$$

2. **Economic Cost–Loss Decision Threshold & EVPI ([`inquiry::CostLossFraming`](../crates/stochology/src/inquiry.rs#L42-L97)):**
   $$p^\* = \frac{C}{\eta L}, \qquad \mathrm{EVPI}(\pi\_0) = \min(\pi\_0 \eta L,\\, C) - \pi\_0 C$$

3. **$e\text{-BH}$ False Discovery Rate Control Under Arbitrary Dependence ([`inquiry::e_bh_discoveries`](../crates/stochology/src/inquiry.rs#L207-L234), Wang & Ramdas, 2022):**
   $$k^\* = \max\\!\left\\{k \in \\{1, \dots, K\\} : \frac{k \\, E\_{(k)}}{K} \ge \frac{1}{\alpha}\right\\}$$

4. **Ville's Test Martingale Inequality ([`identification::EvidenceProcess`](../crates/stochology/src/identification.rs#L17-L87), Ville, 1939):**
   $$E\_t = \prod\_{i=1}^t L\_i, \qquad \mathbb{P}\_{H\_0}\\!\left(\sup\_{t \ge 1} E\_t \ge \frac{1}{\alpha}\right) \le \alpha$$

5. **Manski Non-Parametric Bounds & Imbens–Manski Confidence Interval ([`identification::ManskiBounds`](../crates/stochology/src/identification.rs#L112-L185)):**
   $$L\_{\mathrm{M}} = p\_{\mathrm{obs}}\\,\bar{Y}\_{\mathrm{obs}} + (1 - p\_{\mathrm{obs}})\\,y\_{\min}, \qquad U\_{\mathrm{M}} = p\_{\mathrm{obs}}\\,\bar{Y}\_{\mathrm{obs}} + (1 - p\_{\mathrm{obs}})\\,y\_{\max}$$

6. **Lee (2009) Monotone Attrition Trimming Fraction ([`identification::LeeAttritionBounds`](../crates/stochology/src/identification.rs#L190-L249)):**
   $$q = \frac{p\_T - p\_C}{p\_T} \quad (p\_T > p\_C)$$

7. **Peterson (1976) Dependent Competing-Risks Survival Bounds ([`identification::PetersonSurvivalBounds`](../crates/stochology/src/identification.rs#L253-L293)):**
   $$1 - \frac{d + c}{n} \le S(t) \le 1 - \frac{d}{n}$$

---

## A.2 Filter II — Asymmetry, Heavy Tails, and the Archive

8. **Jensen's Convexity Gap & Empirical CRPS ([`asymmetry::NonlinearSummary`](../crates/stochology/src/asymmetry.rs#L166-L243)):**
   $$\Delta\_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X]), \qquad \mathrm{CRPS}(F, y) = \mathbb{E}\_F|X - y| - \frac{1}{2}\mathbb{E}\_F|X - X'|$$

9. **Hill's (1975) Tail Index Estimator ([`asymmetry::HillTailEstimate`](../crates/stochology/src/asymmetry.rs#L261-L321)):**
   $$\hat{\alpha}\_{k,n} = \left(\frac{1}{k}\sum\_{i=1}^k \ln\frac{X\_{(n - i + 1)}}{X\_{(n - k)}}\right)^{-1}$$

10. **Keynes's (1921) Evidential Weight ([`archive::KeynesEvidentialWeight`](../crates/stochology/src/archive.rs#L17-L56)):**
    $$V = \frac{K\_{\mathrm{eff}}}{K\_{\mathrm{eff}} + I\_{\mathrm{unexplored}}} \in (0, 1)$$

11. **Meng's (2018) Exact Big Data Paradox Identity ([`archive::MengBigDataDefect`](../crates/stochology/src/archive.rs#L61-L120)):**
    $$\bar{Y}\_n - \bar{Y}\_N = \rho\_{R,Y} \times \sqrt{\frac{1 - f}{f}} \times \sigma\_Y, \qquad n\_{\mathrm{eff}} = \frac{f}{1 - f}\\,\frac{1}{\rho\_{R,Y}^2}$$

12. **Campbell–Moulton Unequal-Cluster Design Effect ([`archive::ClusterDesignEffect`](../crates/stochology/src/archive.rs#L125-L214)):**
    $$\tilde{m} = \bar{m}\bigl(1 + \mathrm{CV}^2(m)\bigr), \qquad \mathrm{DEFF}(p) = 1 + (\tilde{m} - 1)\rho\_I(p), \qquad n\_{\mathrm{eff}}(p) = \frac{n}{\mathrm{DEFF}(p)}$$

13. **Chao's (1984) Lower Bound on the Uncounted Population ([`archive::CaptureRecaptureAudit`](../crates/stochology/src/archive.rs#L268-L303)):**
    $$\hat{n}\_{00,\mathrm{Chao}} = \frac{f\_1^2}{4 f\_2}$$

---

## A.3 Filter III — Reactive Worlds, Calibration, and Irreversible Time

14. **Isotonic CORP-PAV Brier Score Decomposition ([`performativity::CorpPavDecomposition`](../crates/stochology/src/performativity.rs#L19-L97), Dimitriadis, Gneiting & Jordan, 2021):**
    $$\mathrm{BS} = \mathrm{REL}\_{\mathrm{CORP}} - \mathrm{RES}\_{\mathrm{CORP}} + \mathrm{UNC}\_0$$

15. **Perdomo et al. (2020) Performative Contraction & Equilibrium ([`performativity::PerformativeSystem`](../crates/stochology/src/performativity.rs#L351-L466)):**
    $$\kappa\_{\mathrm{RRM}} = \frac{\varepsilon \beta}{\gamma} < 1, \qquad \theta\_{\mathrm{PS}} = \frac{\mu\_0}{1 - \varepsilon}, \qquad \theta\_{\mathrm{PO}} = \frac{\mu\_0(1 - \varepsilon)}{(1 - \varepsilon)^2 + \varepsilon^2 \sigma^2}$$

16. **Itô Time-Average Growth & Baker–McHale (2013) Epistemic Kelly Allocation ([`vital::EpistemicKellyAllocation`](../crates/stochology/src/vital.rs#L17-L92)):**
    $$g\_{\mathrm{time}}(f) = f\mu - \frac{1}{2}f^2\sigma^2, \qquad f^\*\_{\mathrm{epistemic}} = \left(\frac{n\_{\mathrm{eff}}}{n\_{\mathrm{eff}} + (\sigma / \hat{\mu})^2}\right)\frac{\hat{\mu}}{\sigma^2}$$

17. **Kingman's (1961) Heavy-Traffic Queue Factor & Simchi-Levi Survival Condition ([`vital::VitalSlackBuffer`](../crates/stochology/src/vital.rs#L190-L250)):**
    $$W\_q \propto \left(\frac{c\_a^2 + c\_s^2}{2}\right)\frac{\rho}{1 - \rho}, \qquad \mathrm{TTS} = \frac{B\_t}{\dot{D}\_{\mathrm{deficit}}} \ge \mathrm{TTR}$$

---

## A.4 Ethical & Institutional Mediation and Triadic Synthesis

18. **Chouldechova's (2017) Algebraic Fairness Trilemma ([`governance::ChouldechovaTrilemma`](../crates/stochology/src/governance.rs#L285-L333)):**
    $$\frac{\mathrm{FPR}\_A}{\mathrm{FPR}\_B} = \frac{p\_A / (1 - p\_A)}{p\_B / (1 - p\_B)} \quad \text{when } \mathrm{PPV}\_A = \mathrm{PPV}\_B \text{ and } \mathrm{TPR}\_A = \mathrm{TPR}\_B$$

19. **Fricker's Testimonial Discount on Beer's Algedonic Channel ([`governance::AlgedonicChannel`](../crates/stochology/src/governance.rs#L159-L202)):**
    $$\tau\_{\mathrm{alarm}}(\delta\_{\mathrm{cred}}) = \left\lceil\frac{\ln(1/\alpha)}{(1 - \delta\_{\mathrm{cred}})\ln e\_t}\right\rceil$$

20. **Leontief Structural Complementarity of the Nuclear Triad ([`synthesis::TriadicIntegrity`](../crates/stochology/src/synthesis.rs#L34-L79)):**
    $$W\_{\mathrm{triad}} = S\_{\mathrm{epi}} \cdot S\_{\mathrm{vital}} \cdot S\_{\mathrm{gov}}, \qquad M\_{\mathrm{Leontief}} = \min(S\_{\mathrm{epi}}, S\_{\mathrm{vital}}, S\_{\mathrm{gov}})$$

