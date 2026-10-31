# Appendix A — Mathematical Reference

This appendix summarizes the core mathematical identities and inequalities implemented in pure Rust (`std` only) across [`crates/stochology/src/`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/).

---

## A.1 Filter I — Inquiry, Framing, Martingales, and Partial Identification

1. **Cromwell's Interior Probability Floor ([`inquiry::cromwell_clamp`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/inquiry.rs#L35-L38)):**
   $$p \in [\varepsilon_{\mathrm{floor}},\, 1 - \varepsilon_{\mathrm{floor}}], \qquad 0 < \varepsilon_{\mathrm{floor}} < \frac{1}{2}$$

2. **Economic Cost–Loss Decision Threshold & EVPI ([`inquiry::CostLossFraming`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/inquiry.rs#L42-L97)):**
   $$p^* = \frac{C}{\eta L}, \qquad \mathrm{EVPI}(\pi_0) = \min(\pi_0 \eta L,\, C) - \pi_0 C$$

3. **$e\text{-BH}$ False Discovery Rate Control Under Arbitrary Dependence ([`inquiry::e_bh_discoveries`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/inquiry.rs#L207-L234), Wang & Ramdas, 2022):**
   $$k^* = \max\!\left\{k \in \{1, \dots, K\} : \frac{k \, E_{(k)}}{K} \ge \frac{1}{\alpha}\right\}$$

4. **Ville's Test Martingale Inequality ([`identification::EvidenceProcess`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/identification.rs#L17-L87), Ville, 1939):**
   $$E_t = \prod_{i=1}^t L_i, \qquad \mathbb{P}_{H_0}\!\left(\sup_{t \ge 1} E_t \ge \frac{1}{\alpha}\right) \le \alpha$$

5. **Manski Non-Parametric Bounds & Imbens–Manski Confidence Interval ([`identification::ManskiBounds`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/identification.rs#L112-L185)):**
   $$L_{\mathrm{M}} = p_{\mathrm{obs}}\,\bar{Y}_{\mathrm{obs}} + (1 - p_{\mathrm{obs}})\,y_{\min}, \qquad U_{\mathrm{M}} = p_{\mathrm{obs}}\,\bar{Y}_{\mathrm{obs}} + (1 - p_{\mathrm{obs}})\,y_{\max}$$

6. **Lee (2009) Monotone Attrition Trimming Fraction ([`identification::LeeAttritionBounds`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/identification.rs#L190-L249)):**
   $$q = \frac{p_T - p_C}{p_T} \quad (p_T > p_C)$$

7. **Peterson (1976) Dependent Competing-Risks Survival Bounds ([`identification::PetersonSurvivalBounds`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/identification.rs#L253-L293)):**
   $$1 - \frac{d + c}{n} \le S(t) \le 1 - \frac{d}{n}$$

---

## A.2 Filter II — Asymmetry, Heavy Tails, and the Archive

8. **Jensen's Convexity Gap & Empirical CRPS ([`asymmetry::NonlinearSummary`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/asymmetry.rs#L166-L243)):**
   $$\Delta_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X]), \qquad \mathrm{CRPS}(F, y) = \mathbb{E}_F|X - y| - \frac{1}{2}\mathbb{E}_F|X - X'|$$

9. **Hill's (1975) Tail Index Estimator ([`asymmetry::HillTailEstimate`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/asymmetry.rs#L261-L321)):**
   $$\hat{\alpha}_{k,n} = \left(\frac{1}{k}\sum_{i=1}^k \ln\frac{X_{(n - i + 1)}}{X_{(n - k)}}\right)^{-1}$$

10. **Keynes's (1921) Evidential Weight ([`archive::KeynesEvidentialWeight`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/archive.rs#L17-L56)):**
    $$V = \frac{K_{\mathrm{eff}}}{K_{\mathrm{eff}} + I_{\mathrm{unexplored}}} \in (0, 1)$$

11. **Meng's (2018) Exact Big Data Paradox Identity ([`archive::MengBigDataDefect`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/archive.rs#L61-L120)):**
    $$\bar{Y}_n - \bar{Y}_N = \rho_{R,Y} \times \sqrt{\frac{1 - f}{f}} \times \sigma_Y, \qquad n_{\mathrm{eff}} = \frac{f}{1 - f}\,\frac{1}{\rho_{R,Y}^2}$$

12. **Campbell–Moulton Unequal-Cluster Design Effect ([`archive::ClusterDesignEffect`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/archive.rs#L125-L214)):**
    $$\tilde{m} = \bar{m}\bigl(1 + \mathrm{CV}^2(m)\bigr), \qquad \mathrm{DEFF}(p) = 1 + (\tilde{m} - 1)\rho_I(p), \qquad n_{\mathrm{eff}}(p) = \frac{n}{\mathrm{DEFF}(p)}$$

13. **Chao's (1984) Lower Bound on the Uncounted Population ([`archive::CaptureRecaptureAudit`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/archive.rs#L268-L303)):**
    $$\hat{n}_{00,\mathrm{Chao}} = \frac{f_1^2}{4 f_2}$$

---

## A.3 Filter III — Reactive Worlds, Calibration, and Irreversible Time

14. **Isotonic CORP-PAV Brier Score Decomposition ([`performativity::CorpPavDecomposition`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/performativity.rs#L19-L97), Dimitriadis, Gneiting & Jordan, 2021):**
    $$\mathrm{BS} = \mathrm{REL}_{\mathrm{CORP}} - \mathrm{RES}_{\mathrm{CORP}} + \mathrm{UNC}_0$$

15. **Perdomo et al. (2020) Performative Contraction & Equilibrium ([`performativity::PerformativeSystem`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/performativity.rs#L351-L466)):**
    $$\kappa_{\mathrm{RRM}} = \frac{\varepsilon \beta}{\gamma} < 1, \qquad \theta_{\mathrm{PS}} = \frac{\mu_0}{1 - \varepsilon}, \qquad \theta_{\mathrm{PO}} = \frac{\mu_0(1 - \varepsilon)}{(1 - \varepsilon)^2 + \varepsilon^2 \sigma^2}$$

16. **Itô Time-Average Growth & Baker–McHale (2013) Epistemic Kelly Allocation ([`vital::EpistemicKellyAllocation`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/vital.rs#L17-L92)):**
    $$g_{\mathrm{time}}(f) = f\mu - \frac{1}{2}f^2\sigma^2, \qquad f^*_{\mathrm{epistemic}} = \left(\frac{n_{\mathrm{eff}}}{n_{\mathrm{eff}} + (\sigma / \hat{\mu})^2}\right)\frac{\hat{\mu}}{\sigma^2}$$

17. **Kingman's (1961) Heavy-Traffic Queue Factor & Simchi-Levi Survival Condition ([`vital::VitalSlackBuffer`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/vital.rs#L190-L250)):**
    $$W_q \propto \left(\frac{c_a^2 + c_s^2}{2}\right)\frac{\rho}{1 - \rho}, \qquad \mathrm{TTS} = \frac{B_t}{\dot{D}_{\mathrm{deficit}}} \ge \mathrm{TTR}$$

---

## A.4 Ethical & Institutional Mediation and Triadic Synthesis

18. **Chouldechova's (2017) Algebraic Fairness Trilemma ([`governance::ChouldechovaTrilemma`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/governance.rs#L285-L333)):**
    $$\frac{\mathrm{FPR}_A}{\mathrm{FPR}_B} = \frac{p_A / (1 - p_A)}{p_B / (1 - p_B)} \quad \text{when } \mathrm{PPV}_A = \mathrm{PPV}_B \text{ and } \mathrm{TPR}_A = \mathrm{TPR}_B$$

19. **Fricker's Testimonial Discount on Beer's Algedonic Channel ([`governance::AlgedonicChannel`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/governance.rs#L159-L202)):**
    $$\tau_{\mathrm{alarm}}(\delta_{\mathrm{cred}}) = \left\lceil\frac{\ln(1/\alpha)}{(1 - \delta_{\mathrm{cred}})\ln e_t}\right\rceil$$

20. **Leontief Structural Complementarity of the Nuclear Triad ([`synthesis::TriadicIntegrity`](file:///home/silvano-neto/Documents/stochology/crates/stochology/src/synthesis.rs#L34-L79)):**
    $$W_{\mathrm{triad}} = S_{\mathrm{epi}} \cdot S_{\mathrm{vital}} \cdot S_{\mathrm{gov}}, \qquad M_{\mathrm{Leontief}} = \min(S_{\mathrm{epi}}, S_{\mathrm{vital}}, S_{\mathrm{gov}})$$

