# Appendix A — Mathematical Reference

This appendix summarizes the Three Canonical Mathematical Operators and their constituent identities and inequalities implemented in pure Rust (`std` only) across [`crates/stochology/src/`](crates/stochology/src/).

---

## A.1 Filter I (Axis 1 — $S_{\mathrm{epi}}$): Canonical Operator I — Partial-Identification Set & Effective Sample Contraction

$$\mathcal{I}_{\mathrm{epi}}(\mathcal{D}_t; \alpha) = \bigl[\hat{\theta}_L - c_{\mathrm{IM}}(\alpha)\,\sigma / \sqrt{n_{\mathrm{eff}}},\;\; \hat{\theta}_U + c_{\mathrm{IM}}(\alpha)\,\sigma / \sqrt{n_{\mathrm{eff}}}\bigr], \qquad \frac{1}{n_{\mathrm{eff}}} = \frac{\mathrm{DEFF}_{\mathrm{total}}}{n} + \left(\frac{1 - f}{f}\right)\rho_{R,Y}^2$$

1. **Cromwell's Interior Probability Floor ([`epistemic::cromwell_clamp`](crates/stochology/src/epistemic.rs)):**
   $$p \in [\varepsilon_{\mathrm{floor}},\, 1 - \varepsilon_{\mathrm{floor}}], \qquad 0 < \varepsilon_{\mathrm{floor}} < \frac{1}{2}$$

2. **Economic Cost–Loss & Pauker–Kassirer Decision Thresholds & EVPI ([`epistemic::EpistemicMarginState::from_inquiry_framing`](crates/stochology/src/epistemic.rs), [`epistemic::pauker_kassirer_threshold`](crates/stochology/src/epistemic.rs)):**
   $$p^* = \frac{C}{\eta L}, \qquad p^*_{\mathrm{PK}} = \frac{1}{1 + B_{\mathrm{net}} / H_{\mathrm{net}}}, \qquad \mathrm{EVPI}(\pi_0) = \min\bigl(C,\, \pi_0 \eta L\bigr) - \pi_0 \min\bigl(C,\, \eta L\bigr) \ge 0$$

3. **$e\text{-BH}$ False Discovery Rate Control Under Arbitrary Dependence ([`epistemic::e_bh_discoveries`](crates/stochology/src/epistemic.rs), Wang & Ramdas, 2022):**
   $$k^* = \max\!\left\{k \in \{1, \dots, K\} : \frac{k \, E_{(k)}}{K} \ge \frac{1}{\alpha}\right\}$$

4. **Ville's Test Martingale Inequality ([`epistemic::EpistemicMarginState::from_ate_bounds`](crates/stochology/src/epistemic.rs), Ville, 1939):**
   $$E_t = \prod_{i=1}^t L_i, \qquad \mathbb{P}_{H_0}\!\left(\sup_{t \ge 1} E_t \ge \frac{1}{\alpha}\right) \le \alpha$$

5. **Two-Arm Manski Non-Parametric ATE Bounds & Adaptive Imbens–Manski Confidence Interval ([`epistemic::EpistemicMarginState::from_ate_bounds`](crates/stochology/src/epistemic.rs), [`epistemic::EpistemicMarginState::imbens_manski_critical_value`](crates/stochology/src/epistemic.rs)):**
   $$\mathrm{ATE}_L^{\mathrm{M}} = L_1^{\mathrm{M}} - U_0^{\mathrm{M}}, \qquad \mathrm{ATE}_U^{\mathrm{M}} = U_1^{\mathrm{M}} - L_0^{\mathrm{M}}, \qquad \Phi\!\left(c_{\mathrm{IM}} + \frac{\hat{\Delta}}{\hat{\sigma}_{\max}}\right) - \Phi(-c_{\mathrm{IM}}) = 1 - \alpha$$

6. **Lee (2009) Monotone Attrition Trimming Fraction ([`epistemic::EpistemicMarginState::from_lee_attrition_bounds`](crates/stochology/src/epistemic.rs)):**
   $$q = \frac{p_T - p_C}{p_T} \quad (p_T > p_C)$$

7. **Peterson (1976) Competing-Risks Fréchet Bounds & Worst-Case Censoring Bounds ([`epistemic::EpistemicMarginState::from_peterson_competing_risks_bounds`](crates/stochology/src/epistemic.rs), [`epistemic::EpistemicMarginState::from_censoring_bounds`](crates/stochology/src/epistemic.rs)):**
   $$\max\bigl(0,\, S_1(t) + S_2(t) - 1\bigr) \le S(t) \le \min\bigl(S_1(t),\, S_2(t)\bigr), \qquad 1 - \frac{d + c}{n} \le S_{\mathrm{cens}}(t) \le 1 - \frac{d}{n}$$

8. **Keynes's (1921) Evidential Weight ([`epistemic::EpistemicMarginState::keynes_weight_from_n_eff`](crates/stochology/src/epistemic.rs)):**
   $$V = \frac{n_{\mathrm{eff}}}{n_{\mathrm{eff}} + n_0} \in (0, 1)$$

9. **Meng's (2018) Exact Big Data Paradox Identity ([`epistemic::EpistemicMarginState::from_meng_big_data`](crates/stochology/src/epistemic.rs)):**
   $$\bar{Y}_n - \bar{Y}_N = \rho_{R,Y} \times \sqrt{\frac{1 - f}{f}} \times \sigma_Y, \qquad n_{\mathrm{eff}} = \frac{f}{1 - f}\,\frac{1}{\rho_{R,Y}^2}$$

10. **Campbell–Moulton Unequal-Cluster Design Effect ([`epistemic::EpistemicMarginState::from_archive_clusters`](crates/stochology/src/epistemic.rs)):**
    $$\tilde{m} = \bar{m}\bigl(1 + \mathrm{CV}^2(m)\bigr), \qquad \mathrm{DEFF}(p) = 1 + (\tilde{m} - 1)\rho_I(p), \qquad n_{\mathrm{eff}}(p) = \frac{n}{\mathrm{DEFF}(p)}$$

11. **Chao's (1984) Lower Bound on the Uncounted Population ([`epistemic::EpistemicMarginState::from_capture_recapture`](crates/stochology/src/epistemic.rs)):**
    $$\hat{n}_{00,\mathrm{Chao}} = \frac{f_1^2}{2 f_2}$$

---

## A.2 Filter II (Axis 2 — $S_{\mathrm{vital}}$): Canonical Operator II — Non-Ergodic Reserve & Survival-Time Barrier

$$dR_t = \bigl(u_t - D_t(\alpha_{\mathrm{Hill}}) - \Delta_J - \tfrac{1}{2}f_t^2\sigma^2\bigr)\,dt, \qquad \mathbb{E}[W_q] \approx \left(\frac{c_a^2 + c_s^2}{2}\right)\frac{\rho_t}{1 - \rho_t}\tau_s, \qquad \mathrm{TTS}_{\mathrm{survival}}(R_t) = \frac{B_t}{\dot{D}_{\mathrm{deficit}}} \ge \mathrm{TTR}_{\mathrm{recovery}}$$

12. **Jensen's Convexity Gap & Empirical CRPS ([`vital::VitalMarginState::from_heavy_tails_and_asymmetry`](crates/stochology/src/vital.rs)):**
    $$\Delta_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X]), \qquad \mathrm{CRPS}(F, y) = \mathbb{E}_F|X - y| - \frac{1}{2}\mathbb{E}_F|X - X'|$$

13. **Hill's (1975) Tail Index Estimator ([`vital::VitalMarginState::from_heavy_tails_and_asymmetry`](crates/stochology/src/vital.rs)):**
    $$\hat{\alpha}_{k,n} = \left(\frac{1}{k}\sum_{i=1}^k \ln\frac{X_{(n - i + 1)}}{X_{(n - k)}}\right)^{-1}$$

14. **Itô Time-Average Growth & Baker–McHale (2013) Epistemic Kelly Allocation ([`vital::VitalMarginState::from_kelly_parameters`](crates/stochology/src/vital.rs)):**
    $$g_{\mathrm{time}}(f) = r + f(\mu - r) - \frac{1}{2}f^2\sigma^2, \qquad k(n_{\mathrm{eff}}) = \frac{1}{1 + 1 / n_{\mathrm{eff}}}, \qquad f^*_{\mathrm{epistemic}} = k(n_{\mathrm{eff}})\,\frac{\hat{\mu} - r}{\sigma^2}$$

15. **Kingman's (1961) $G/G/1$ Heavy-Traffic Queue Waiting Time & Simchi-Levi Survival Condition ([`vital::VitalMarginState::from_queue_basin_and_survival`](crates/stochology/src/vital.rs)):**
    $$\mathbb{E}[W_q] \approx \left(\frac{c_a^2 + c_s^2}{2}\right)\frac{\rho}{1 - \rho}\tau_s, \qquad \mathrm{TTS}_{\mathrm{survival}} = \frac{B_t}{\dot{D}_{\mathrm{deficit}}} \ge \mathrm{TTR}_{\mathrm{recovery}}$$

---

## A.3 Filter III (Axis 3 — $S_{\mathrm{gov}}$): Canonical Operator III — Bitemporal Accountability & Performative Integrity Gate

$$S_{\mathrm{gov}} = \mathcal{O}_{\mathrm{gov}}(\pi_t, \mathcal{I}_t) = \min\bigl(g_{\text{ex-ante}}(\text{CORP}, \text{ICP}, \text{Perdomo}),\; g_{\text{real-time}}(\text{List–Pettit}, \text{Fricker}, \text{Bateson}),\; g_{\text{ex-post}}(\text{Sosa}, \text{Holmström}, \text{Vault})\bigr)$$

16. **Isotonic CORP-PAV Brier Score Decomposition ([`governance::GovernanceState::from_ex_ante_gate`](crates/stochology/src/governance.rs), Dimitriadis, Gneiting & Jordan, 2021):**
    $$\mathrm{BS} = \mathrm{REL}_{\mathrm{CORP}} - \mathrm{RES}_{\mathrm{CORP}} + \mathrm{UNC}_0$$

17. **Perdomo et al. (2020) Performative Contraction & Equilibrium ([`governance::GovernanceState::from_ex_ante_gate`](crates/stochology/src/governance.rs)):**
    $$\kappa_{\mathrm{RRM}} = \frac{\varepsilon \beta}{\gamma} < 1, \qquad \theta_{\mathrm{PS}} = \frac{\mu_0}{1 - \varepsilon}, \qquad \theta_{\mathrm{PO}} = \frac{\mu_0(1 - \varepsilon)}{(1 - \varepsilon)^2 + \varepsilon^2 \sigma^2}$$

18. **Chouldechova's (2017) Algebraic Fairness Trilemma ([`governance::GovernanceState::from_real_time_and_ex_post_gates`](crates/stochology/src/governance.rs)):**
    $$\frac{\mathrm{FPR}_A}{\mathrm{FPR}_B} = \frac{p_A / (1 - p_A)}{p_B / (1 - p_B)} \quad \text{when } \mathrm{PPV}_A = \mathrm{PPV}_B \text{ and } \mathrm{TPR}_A = \mathrm{TPR}_B$$

19. **Fricker's Testimonial Discount on Beer's Algedonic Channel ([`governance::GovernanceState::from_real_time_and_ex_post_gates`](crates/stochology/src/governance.rs)):**
    $$\tau_{\mathrm{alarm}}(\delta_{\mathrm{cred}}) = \left\lceil\frac{\ln(1/\alpha)}{(1 - \delta_{\mathrm{cred}})\ln e_t}\right\rceil \le \mathrm{TTS}_{\mathrm{survival}}$$

20. **Bateson's Double-Bind Trap & Simon–Girard Scapegoat Blame Transfer ([`governance::GovernanceState::from_real_time_and_ex_post_gates`](crates/stochology/src/governance.rs)):**
    $$\mathrm{DB}_t = \beta_{\mathrm{proxy}, t} \cdot \delta_{\mathrm{cred}, t} \cdot \bigl(1 - \mathbf{1}\{\text{brake}_t\}\bigr) \in [0, 1], \qquad \Delta_{\mathrm{scapegoat}} = \max\bigl(0,\, b_{\mathrm{frontline}} - L_{\mathrm{oper}}\bigr)$$

---

## A.4 Leontief Structural Complementarity of the Nuclear Triad

21. **Leontief Structural Complementarity ([`synthesis::TriadicIntegrity`](crates/stochology/src/synthesis.rs), `[Assumed]`):**
    $$M_{\mathrm{Leontief}} = \min(S_{\mathrm{epi}}, S_{\mathrm{vital}}, S_{\mathrm{gov}}) \in [0, 1]$$
    where on a sequential trajectory of horizon $T$ ([`engine::CommunityReserveEnv`](crates/stochology/src/engine.rs), `[Simulated]`):
    $$S_{\mathrm{epi}} = \frac{1}{T}\sum_{t=1}^T S_{\mathrm{epi}, t}\bigl(\mathrm{Cov}_T, V(n_{\mathrm{eff}, t}), \Delta_{\mathrm{id}, t}\bigr), \quad S_{\mathrm{vital}} = \frac{1}{T}\sum_{t=1}^T \min\bigl(S_{\mathrm{tail}, t},\, S_{\mathrm{time}, t},\, S_{\mathrm{surv}, t}\bigr), \quad S_{\mathrm{gov}} = \frac{1}{T}\sum_{t=1}^T \min\bigl(g_{\mathrm{ex\text{-}ante}, t},\, g_{\mathrm{kairos}, t},\, g_{\mathrm{ex\text{-}post}, t}\bigr)$$

---

## A.5 The Grammar of Aim — Syntax Layer and Acceptance Predicate

$$\mathcal{S} = (\sigma_1, \dots, \sigma_N), \quad \sigma_t = (\mathbf{u}_t, \mathbf{w}_t, \mathbf{m}_t), \quad \mathcal{A}(\mathcal{S}; \boldsymbol{\tau}) \in \{\mathrm{Interior}, \mathrm{Boundary}, \mathrm{Collapse}, \mathrm{Unparsable}\}$$

22. **Acceptance Predicate & Fixed Evaluation Order ([`grammar.rs::judge`](crates/stochology/src/grammar.rs), `[Assumed]`):**
    Given a trajectory sentence $\mathcal{S}$ over the seven guard sites $T_1, \dots, T_7$ partitioned $3/2/2$ across $(S_{\mathrm{epi}}, S_{\mathrm{vital}}, S_{\mathrm{gov}})$, the decision procedure `grammar.rs::judge` evaluates verdicts in strict hierarchical order:
    1. **Quarantine:** If any margin $m_j$ is non-finite ($\mathrm{NaN}, \pm\infty$) or a circuit transit is unguarded, $\mathcal{A}(\mathcal{S}; \boldsymbol{\tau}) = \mathrm{Unparsable}$;
    2. **Collapse:** If $\exists j \in \{1, \dots, 7\}$ such that $m_j < 0$, then $\mathcal{A}(\mathcal{S}; \boldsymbol{\tau}) = \mathrm{Collapse}(\text{Proposition}, \text{at } T_j, \text{step } t)$;
    3. **Boundary:** If $\exists j \in \{1, \dots, 7\}$ such that $0 \le m_j < \tau_j$, then $\mathcal{A}(\mathcal{S}; \boldsymbol{\tau}) = \mathrm{Boundary}$;
    4. **Interior:** If $\forall j \in \{1, \dots, 7\}, m_j \ge \tau_j$, then $\mathcal{A}(\mathcal{S}; \boldsymbol{\tau}) = \mathrm{Interior}$.

23. **Non-Commutation Property of Aim ([`grammar.rs::judge`](crates/stochology/src/grammar.rs), `[Assumed]`):**
    $$\mathcal{A}(\mathbf{u}, \mathbf{w}, \mathbf{m}; \boldsymbol{\tau}) = \mathcal{A}(\mathbf{u}, \mathbf{w}', \mathbf{m}; \boldsymbol{\tau}) \quad \forall \, \mathbf{w}, \mathbf{w}'.$$
    The acceptance predicate is invariant to the contingency tape $\mathbf{w}_t$ (*skopós*). No accidental external success can redeem depleted guard margins $\mathbf{m}_t$ or an uncalibrated control tape $\mathbf{u}_t$ (*télos*). Antipater's separation of aim from external outcome is an exact mathematical property of the grammar.

---

## A.6 The Multilayer Knowledge Graph ($\mathcal{G}_1 \leftrightarrow \mathcal{G}_2$)

$$\binom{7}{3} = 12 + 9 + 9 + 4 + 1 = 35, \qquad \mathcal{H}(L_i) = \min_{L_j \in \mathrm{Cone}(L_i)} M_{\mathrm{Leontief}}(L_j)$$

24. **Vandermonde Partition of Triadic Exposure Regimes ([`multilayer.rs::TriadicExposureRegime`](crates/stochology/src/multilayer.rs), `[Assumed]`):**
    Partitioning $\binom{7}{3} = 35$ unique guard triples across the Nuclear Triad with partition sizes $(3, 2, 2)$ for $(S_{\mathrm{epi}}, S_{\mathrm{vital}}, S_{\mathrm{gov}})$ decomposes into five mutually exclusive regimes:
    $$\binom{7}{3} = \sum_{k_1+k_2+k_3=3} \binom{3}{k_1}\binom{2}{k_2}\binom{2}{k_3} = 12 + 9 + 9 + 4 + 1 = 35.$$
    - **Trans-Triadic Keystone ($k_1=1, k_2=1, k_3=1$):** $\binom{3}{1}\binom{2}{1}\binom{2}{1} = 3 \times 2 \times 2 = 12$ orders;
    - **Governance Blind Spot ($k_3=0, k_1 \ge 1, k_2 \ge 1$):** $\binom{3}{2}\binom{2}{1}\binom{2}{0} + \binom{3}{1}\binom{2}{2}\binom{2}{0} = 6 + 3 = 9$ orders;
    - **Vital Blind Spot ($k_2=0, k_1 \ge 1, k_3 \ge 1$):** $\binom{3}{2}\binom{2}{0}\binom{2}{1} + \binom{3}{1}\binom{2}{0}\binom{2}{2} = 6 + 3 = 9$ orders;
    - **Epistemic Blind Spot ($k_1=0$):** $\binom{3}{0}\binom{2}{2}\binom{2}{1} + \binom{3}{0}\binom{2}{1}\binom{2}{2} = 2 + 2 = 4$ orders;
    - **Pure Epistemic Concentration ($k_1=3$):** $\binom{3}{3}\binom{2}{0}\binom{2}{0} = 1$ order.

25. **DAG Ancestor Cone Leontief Homomorphism ([`multilayer.rs::MultilayerHomomorphism::evaluate`](crates/stochology/src/multilayer.rs), `[Assumed]`):**
    $$\mathcal{H}(L_i) = \min_{L_j \in \mathrm{Cone}(L_i)} M_{\mathrm{Leontief}}(L_j) = \min_{L_j \in \mathrm{Cone}(L_i)} \min\bigl(S_{\mathrm{epi}}(L_j),\, S_{\mathrm{vital}}(L_j),\, S_{\mathrm{gov}}(L_j)\bigr).$$
    **Homomorphism Monotonicity:** If any basal order in the dependency cone collapses ($M_{\mathrm{Leontief}}(L_{\mathrm{basal}}) = 0$), then $\mathcal{H}(L_i) = 0$ for all downstream dependent disciplines. Higher-order social and legal institutions cannot survive when foundational physical or epidemiological margins fail.

26. **Motter–Lai (2002) Cascade Breakdown Condition ([`multilayer.rs::MultilayerGraph1`](crates/stochology/src/multilayer.rs), `[Assumed]`):**
    $$C_i = (1 + \alpha) L_{i,0}, \qquad L_j(t) = L_{j,0} + \sum_{k \in \mathcal{F}(t)} L_{k,0} \frac{C_j}{\sum_{m \notin \mathcal{F}(t)} C_m}.$$
    When topological tolerance $\alpha < \alpha_c$, removal of a high-betweenness keystone order triggers a global load-reallocation cascade, causing catastrophic failure across the inter-layer knowledge graph.
