# Chapter 49 — Moral Luck, Apt Agency, and Institutional Design

> Laboratory 8 of the `stochology` CLI: `--chapter 8`.

Suppose an agent has both an **Epistemic Margin** ($S_{\mathrm{epi}}$, Chapters 43–45) and a **Vital Margin** ($S_{\mathrm{vital}}$, Chapters 46–47). What happens if the surrounding **institution** rewards reckless gamblers when they get lucky, punishes prudent operators when an exogenous tail storm strikes, aggregates committee votes by premise-majority without a safety veto, discounts warnings from frontline nurses or field technicians, and floods human operators with $93.9\%$ false-positive alerts (`[Assumed]`)?

That is **Collapse Proposition 3** (*cynical capture and pillaging of reserves*). **Tension 7** (completing **Mediating Filter III · Axis 3 — $S_{\mathrm{gov}}$: Institutional Governance**, implemented via [`GovernanceState::from_real_time_and_ex_post_gates`](crates/stochology/src/governance.rs#L1590-L1704) in [`crates/stochology/src/governance.rs`](crates/stochology/src/governance.rs)) resolves the conflict between **ex-ante telic care (*télos*), ex-post outcome luck (*skopós*), and institutional governance** (Beer, 1972; Nagel, 1979; Holmström, 1979; Williams, 1981; Baron & Hershey, 1988; Innes, 1990; Holmström & Milgrom, 1991; Gigerenzer & Hoffrage, 1995; Scanlon, 1998; Sosa, 2007, 2015; Fricker, 2007, 2020; Fantl & McGrath, 2009; List & Pettit, 2011; Hansson, 2013; Chouldechova, 2017; Wong et al., 2021; Zheng & Meister, 2025, `[Literature]`).

Together with Chapter 48, Chapter 49 completes **Canonical Operator III** ($\mathcal{O}_{\mathrm{gov}}$, [`GovernanceState::canonical_score`](crates/stochology/src/governance.rs#L1206-L1223)) through **Gate 2 (Real-Time Collegiate & Algedonic Veto)** and **Gate 3 (Ex-Post Bitemporal Accountability & Liability Allocation)**:

$$S_{\mathrm{gov}} = \mathcal{O}_{\mathrm{gov}}(\pi_t, \mathcal{I}_t) = \min\bigl(g_{\text{ex-ante}}(\text{CORP}, \text{ICP}, \text{Perdomo}),\; g_{\text{kairos}}(\text{List–Pettit}, \text{Fricker}, \text{Bateson}),\; g_{\text{ex-post}}(\text{Sosa}, \text{Holmström}, \text{Vault})\bigr)$$

![Single-Opening Cryptographic Temporal Vault](figures/fig-10-temporal-vault.svg)

---

## 8.1 Sosa's Virtue Epistemology, Outcome Bias, and Bitemporal Telic Evaluation

In *Moral Luck* (1979/1981), Bernard Williams and Thomas Nagel demonstrated how human moral judgment is systematically corrupted by the stochastic outcome of an action after it leaves the agent's hands (`[Literature]`). Complementing this, Jonathan Baron and John C. Hershey (1988) demonstrated experimentally that human evaluators suffer from severe **outcome bias** (`[Literature]`): rating the exact same ex-ante medical or operational decision as "competent" when the coin lands heads and "negligent" when an uncontrollable tail risk materializes.

### 8.1.1 Antipater's Archer: Télos vs. Skopós and Sosa's AAA Taxonomy

To overcome outcome bias without collapsing into an ungrounded ethics of mere good intentions, Stochology formalizes the ancient Hellenistic Stoic distinction formulated by **Antipater of Tarsus** (Stobaeus, *Eclogae* II.76.9–15): the archer (*toxótēs*) aiming with consummate care (*télos*, the internal good of doing everything in one's power continuously and undeviatingly) versus striking the external target (*skopós*, a *proēgmenon* or preferred indifferent).

Ernest Sosa (2007, 2015) formalized this distinction into his **AAA** (*Accuracy, Adroitness, Aptness*) virtue epistemology, extended in [`TelicGrade`](crates/stochology/src/governance.rs#L26-L37) (and [`SosaJudgmentGrade`](crates/stochology/src/governance.rs#L53) in [`GovernanceState`](crates/stochology/src/governance.rs#L685-L800)):
- **`InaptAndInaccurate`:** Negligent ex-ante method and bad outcome ($Y = 0$);
- **`LuckyAccurateOnly`:** Negligent ex-ante method (e.g., operating at $98\%$ utilization with zero buffer) that escaped disaster purely by good luck—must **never** be rewarded with bonuses;
- **`AdroitExAnteUnluckyOutcome`:** Rigorous ex-ante competence (*télos*) and vital buffering that suffered an exogenous tail shock ($L_{\mathrm{exog}}$)—must **never** be scapegoated;
- **`FullyAptPerformance`:** Success achieved *because* of competence, or reflective meta-competence that recognizes when epistemic weight $V$ is too low and suspends risky action in favor of vital safeguards.

### 8.1.2 Cryptographic Commitment via TemporalVault

To enforce this separation institutionally, [`GovernanceState::from_real_time_and_ex_post_gates`](crates/stochology/src/governance.rs#L1590-L1704) and [`TemporalVault`](crates/stochology/src/governance.rs#L472-L556) seal the Stage-1 evaluation of ex-ante competence $C(\pi_t \mid \mathcal{I}_{t_{\mathrm{disp}}})$ inside a single-opening SHA-256 and RFC 2104 HMAC-SHA256 cryptographic commitment **before** the outcome $Y_{t+1}$ is revealed (`[Assumed]`). In the political philosophy of **Hannah Arendt (1958 [2016])**, this bitemporal pre-registration is the institutional translation of the **human promise**—erecting "islands of security" in an ocean of future uncertainty—while **Simone Weil (1952 [1949/2001])** grounds the refusal of cosmetic impunity when pre-registered pacts are betrayed.

Crucially, Stochology draws an uncompromising line between **natural crosswinds** (*vento natural*) and **adulterated air before the shot** (*ar adulterado antes do disparo*): resultant moral luck (`AdroitExAnteUnluckyOutcome`) and Bernard Williams's no-fault reparative *agent-regret* apply **only** when the arrow is deflected by genuine exogenous contingency ($L_{\mathrm{exog}}$) after an adroit shot. When the informational and institutional medium was pre-corrupted by premise-rigging at the top (Simon, 1947/1997 [1979]) and retrospective scapegoating (Girard, 1986 [1982/2004]), there is no stochastic misfortune—only deliberate fraud against the social pact.

---

## 8.2 List–Pettit Discursive Dilemma and Fricker's Testimonial Injustice on Beer's Algedonic Channel

Institutions do not think with a single brain; they aggregate distributed judgments across committees and hierarchies. When judgment aggregation is structured naively, organizations systematically approve catastrophic deployments that every individual member would reject.

### 8.2.1 The List–Pettit Discursive Dilemma Matrix

Christian List and Philip Pettit (2011) generalized the legal *doctrinal paradox* into the **Discursive Dilemma** (`[Literature]`). Suppose deployment of an autonomous system requires all three axes of the Nuclear Triad ($C \iff P_1 \land P_2 \land P_3$), where $P_1$ is Epistemic Margin, $P_2$ is Vital Margin, and $P_3$ is Institutional Governance. Consider three specialists voting on deployment:

| Voter / Specialist | Premise $P_1$ (Epistemic) | Premise $P_2$ (Vital) | Premise $P_3$ (Governance) | Conclusion $C = P_1 \land P_2 \land P_3$ |
| :--- | :---: | :---: | :---: | :---: |
| **Specialist 1 (Epistemologist)** | **True** | **True** | False | **False** (Veto: Gov broken) |
| **Specialist 2 (Physicist/Engineer)** | **True** | False | **True** | **False** (Veto: Vital broken) |
| **Specialist 3 (Governance/Ethicist)**| False | **True** | **True** | **False** (Veto: Epi broken) |
| **Premise-Majority Vote** | **True ($2/3$)** | **True ($2/3$)** | **True ($2/3$)** | **True ($2/3 \land 2/3 \land 2/3$)** |
| **Conclusion-Majority Vote** | — | — | — | **False ($0/3$ Approvals)** |

Every single specialist identifies a fatal failure ($0/3$ conclusion approval), yet **premise-by-premise majority voting** approves each axis ($2/3, 2/3, 2/3$) and launches the unsafe system! In [`GovernanceState::evaluate_list_pettit_votes`](crates/stochology/src/governance.rs#L1814-L1831), Stochology enforces a **constitutional conjunctive veto** (Stafford Beer's $S3^*$ sporadic audit channel), ensuring `discursive_dilemma_present = true` blocks deployment whenever any premise is refuted (`[Assumed]`).

### 8.2.2 Miranda Fricker's Testimonial Injustice on Stafford Beer's Algedonic Channel

In Stafford Beer's (1972) Viable System Model, an **algedonic signal** (pain/distress $e_t > 1$) bypasses bureaucratic hierarchy to trip the emergency circuit breaker before the Time-to-Survival Buffer ($\mathrm{TTS}_{\mathrm{survival}}$) runs out (`[Literature]`).

Miranda Fricker (2007, 2020) demonstrated that hierarchical institutions inflict **testimonial epistemic injustice** by discounting the credibility of frontline workers, nurses, or marginalized residents (`[Literature]`). When management applies a credibility discount $\delta_{\mathrm{cred}} \in [0, 1)$ to frontline alarms ($e_{\mathrm{discounted}} = e_t^{1 - \delta_{\mathrm{cred}}}$), the shifts required to trip the emergency circuit breaker stretch from $\mathrm{TTR}_{\mathrm{undiscounted}}$ to $\mathrm{TTR}_{\mathrm{discounted}}$ ([`AlgedonicChannel::evaluate`](crates/stochology/src/governance.rs#L144-L165)):

$$\mathrm{TTR}_{\mathrm{undiscounted}} = \left\lceil \frac{\ln E_{\mathrm{trip}}}{\ln e_t} \right\rceil, \qquad \mathrm{TTR}_{\mathrm{discounted}} = \left\lceil \frac{\ln E_{\mathrm{trip}}}{(1 - \delta_{\mathrm{cred}})\ln e_t} \right\rceil$$

With distress factor $e_t = 2.10$, trip threshold $E_{\mathrm{trip}} = 20.0$, and survival buffer $\mathrm{TTS}_{\mathrm{survival}} = 8.0$ shifts, an undiscounted channel ($\delta_{\mathrm{cred}} = 0$) trips in $\lceil 2.996 / 0.742 \rceil = 5$ shifts ($\le \mathrm{TTS}_{\mathrm{survival}}$), preserving the system. But under a testimonial discount of $\delta_{\mathrm{cred}} = 0.65$, latency stretches to $\lceil 2.996 / (0.35 \times 0.742) \rceil = 12$ shifts ($> \mathrm{TTS}_{\mathrm{survival}}$), causing fatal ruin (`fatal_testimonial_delay = true`, `[Assumed]`).

---

## 8.3 Innes–Holmström–Milgrom Contracts and Chouldechova's Fairness Trilemma

How do contracts, incentives, and classifiers distribute risk and error across humans?

### 8.3.1 Multitask Distortion, Scapegoating, and the Bateson Double-Bind Trap

1. **Innes (1990) Limited Liability and Moral Hazard (`[Literature]`):** Under limited liability, an executive who captures upside bonus $B$ while truncating downside ruin loss $-L$ at zero has positive private expected value $p B > 0$ for a reckless gamble whose social expected value $p B - (1-p)L \ll 0$ is deeply negative. Only **clawback escrow** $\gamma_{\mathrm{clawback}}$ restores alignment.
2. **Holmström–Milgrom (1991) Multitask Care (`[Literature]`):** For frontline workers performing countable volume tasks $a_1$ and unmeasured care $a_2$, high-powered piece rates $\beta_1 \gg 0$ cannibalize care ($\Delta a_2 < 0$). By Bengt Holmström's (1979) **Informativeness Principle**, structural queue collapse ($L_{\mathrm{struct}} = 83.3\%$ from executive capacity sizing vs. $L_{\mathrm{oper}} = 6.3\%$ frontline operational noise) must be attributed to structural leadership (`[Simulated]`).
3. **Simon–Girard Scapegoat Blame Transfer:** When executives rig decision premises at the top (**Herbert Simon, 1947/1997 [1979]**) and trigger **René Girard's (1986 [1982/2004]) scapegoat mechanism** by assigning $b_{\mathrm{frontline}} = 85.0\%$ of blame downward, [`ContractAndLiabilityAudit`](crates/stochology/src/governance.rs#L255-L313) exposes a **scapegoat blame transfer** of:
   $$\Delta_{\mathrm{scapegoat}} = \max(0, b_{\mathrm{frontline}} - L_{\mathrm{oper}}) = 85.0\% - 6.3\% = +78.7\text{ pp} \qquad (13.6\times\text{ blame inversion, } \text{\texttt{[Simulated]}})$$
4. **Gregory Bateson's (1972/2000) Double-Bind Trap (`[Literature]`):** Combining proxy coercion ($\beta_{\mathrm{proxy}, t} \to 1$), testimonial silencing ($\delta_{\mathrm{cred}, t} \to 1$), and a disabled $S3^*$ brake ($\mathbf{1}\{\text{brake}_t\} = 0$) locks operators into a double bind:
   $$\mathrm{DB}_t = \beta_{\mathrm{proxy}, t} \cdot \delta_{\mathrm{cred}, t} \cdot \bigl(1 - \mathbf{1}\{\text{brake}_t\}\bigr) = 0.85 \times 0.60 \times (1 - 0) = 0.510 \quad \text{\texttt{[Simulated]}}$$
   (versus $0.000$ under `FullTriad`, [`BatesonDoubleBindAudit::evaluate`](crates/stochology/src/governance.rs#L186-L206)).

### 8.3.2 Proof of Alexandra Chouldechova's (2017) Algebraic Fairness Impossibility Theorem

When an algorithmic classifier is deployed across groups with unequal base rates $p_A \ne p_B$, machine-learning practitioners frequently promise simultaneous "fairness" across all criteria. Alexandra Chouldechova (2017) proved this is mathematically impossible (`[Literature]`).

**Theorem (Chouldechova, 2017, Eq. 2.5):** Let $Y \in \{0, 1\}$ be the true outcome, $\hat{Y} \in \{0, 1\}$ the binary classification, $p = \mathbb{P}(Y = 1)$ the group base rate, $\mathrm{PPV} = \mathbb{P}(Y = 1 \mid \hat{Y} = 1)$, $\mathrm{FNR} = \mathbb{P}(\hat{Y} = 0 \mid Y = 1)$, and $\mathrm{FPR} = \mathbb{P}(\hat{Y} = 1 \mid Y = 0)$. For any classifier with non-trivial rates:

$$\mathrm{FPR} = \frac{p}{1 - p} \cdot \frac{1 - \mathrm{PPV}}{\mathrm{PPV}} \cdot \bigl(1 - \mathrm{FNR}\bigr)$$

*Proof:* By Bayes' rule and the definition of conditional probability:
$$\mathbb{P}(\hat{Y} = 1, Y = 1) = \mathbb{P}(\hat{Y} = 1) \cdot \mathrm{PPV} = p \cdot (1 - \mathrm{FNR})$$
$$\implies \mathbb{P}(\hat{Y} = 1) = \frac{p(1 - \mathrm{FNR})}{\mathrm{PPV}}$$
By the law of total probability:
$$\mathbb{P}(\hat{Y} = 1) = \mathbb{P}(\hat{Y} = 1, Y = 1) + \mathbb{P}(\hat{Y} = 1, Y = 0) = p(1 - \mathrm{FNR}) + (1 - p)\,\mathrm{FPR}$$
Equating both expressions for $\mathbb{P}(\hat{Y} = 1)$:
$$\frac{p(1 - \mathrm{FNR})}{\mathrm{PPV}} = p(1 - \mathrm{FNR}) + (1 - p)\,\mathrm{FPR}$$
Subtracting $p(1 - \mathrm{FNR})$ from both sides:
$$(1 - p)\,\mathrm{FPR} = p(1 - \mathrm{FNR})\left(\frac{1}{\mathrm{PPV}} - 1\right) = p(1 - \mathrm{FNR})\left(\frac{1 - \mathrm{PPV}}{\mathrm{PPV}}\right)$$
Dividing both sides by $(1 - p)$ completes the proof:
$$\mathrm{FPR} = \frac{p}{1 - p} \cdot \frac{1 - \mathrm{PPV}}{\mathrm{PPV}} \cdot (1 - \mathrm{FNR}) = \mathrm{odds}(p) \cdot \frac{1 - \mathrm{PPV}}{\mathrm{PPV}} \cdot (1 - \mathrm{FNR}) \qquad \square$$

**Corollary (Fairness Impossibility):** If a classifier achieves equal calibration ($\mathrm{PPV}_A = \mathrm{PPV}_B$) and equal false negative rates ($\mathrm{FNR}_A = \mathrm{FNR}_B$) across two groups with different base rates $p_A \ne p_B$, the false positive rate ratio is strictly dictated by the base-rate odds ratio:

$$\frac{\mathrm{FPR}_A}{\mathrm{FPR}_B} = \frac{p_A / (1 - p_A)}{p_B / (1 - p_B)} = \frac{\mathrm{odds}(p_A)}{\mathrm{odds}(p_B)} \ne 1$$

No algorithmic engineering can eliminate this disparity; an institution must publicly decide **who pays for classification errors** under Thomas Scanlon's (1998) contractualist justification (`[Literature]`).

---

## 8.4 Zheng–Meister's $10\text{ bits/s}$ Bottleneck, Cognitive Queues, and Gigerenzer's Natural Frequencies

Institutional accountability collapses when human operators are cognitively overwhelmed by automated dashboards.

### 8.4.1 The Two-Tier Cognitive Queue Under Zheng–Meister Channel Limits

Jieyu Zheng and Markus Meister (2025) established that human behavioral and sensory-motor conscious information throughput is bounded at roughly **$C_{\mathrm{human}} = 10\text{ bits/s}$** (`[Literature]`), while Andrew Wong et al. (2021) documented that the widely deployed Epic Sepsis Model generated an **$88\%$ false-alarm rate** ($1 - \mathrm{PPV} = 88\%$, $\mathrm{PPV} = 12\%$, `[Literature]`), causing widespread alert fatigue.

In [`ZhengMeisterQueueAudit`](crates/stochology/src/governance.rs#L398-L463), an operator inspecting $B_{\mathrm{audit}}$ bits requires cognitive service time $\tau_{\mathrm{cog}} = B_{\mathrm{audit}} / C_{\mathrm{human}}$. Under alert arrival rate $\lambda_a = 0.04\text{ Hz}$ ($1\text{ alert / } 25\text{ s}$), variance factor $0.5(c_a^2 + c_s^2) = 1.25$, actuation latency $\tau_{\mathrm{act}} = 15.0\text{ s}$, and crisis survival window $\mathrm{TTS}_{\mathrm{crisis}} = 45.0\text{ s}$:
1. **Kairos Synchronous Panel (6 scalars, $30\text{ bits}$, [`KairosSynchronousPanel`](crates/stochology/src/governance.rs#L352-L392)):**
   $\tau_{\mathrm{cog}} = 3.0\text{ s}$, $\rho_{\mathrm{cog}} = 0.12$, Kingman wait $W_q = \frac{0.12}{0.88}(1.25)(3.0) = 0.51\text{ s}$, total $\mathrm{TTR}_{\mathrm{total}} = 3.0 + 0.51 + 15.0 = 18.5\text{ s} \le 45.0\text{ s}$ (`survives = true`, `[Assumed]`);
2. **Standard Epistemic Card (18 scalars, $120\text{ bits}$):**
   $\tau_{\mathrm{cog}} = 12.0\text{ s}$, $\rho_{\mathrm{cog}} = 0.48$, Kingman wait $W_q = 13.8\text{ s}$, $\mathrm{TTR}_{\mathrm{total}} = 40.8\text{ s} \le 45.0\text{ s}$ (`[Assumed]`);
3. **Full Uncompressed Audit (36 scalars, $240\text{ bits}$):**
   $\tau_{\mathrm{cog}} = 24.0\text{ s}$, $\rho_{\mathrm{cog}} = 0.96 \gg 0.76$, Kingman wait $W_q = \frac{0.96}{0.04}(1.25)(24.0) = 720.0\text{ s}$, total recovery $\mathrm{TTR}_{\mathrm{total}} = 759.0\text{ s} \gg 45.0\text{ s}$ (`survives = false`, `[Assumed]`), inducing Collapse Proposition 2 via cognitive queue explosion!

### 8.4.2 Gerd Gigerenzer's Natural Frequency Trees Against Conditional Fallacies

In [`CognitiveAlertFilter`](crates/stochology/src/governance.rs#L211-L250), alerts are translated into Gerd Gigerenzer & Ulrich Hoffrage's (1995) integer natural frequency trees (`[Literature]`):
- In a cohort of $N = 1{,}000$ shifts with $2\%$ crisis prevalence ($\pi = 0.02$): $20$ true crises, $980$ non-crises.
- A noisy detector ($\mathrm{TPR} = 80\%, \mathrm{FPR} = 25\%$, Turing–Good weight $W = 10\log_{10}(0.80/0.25) = +5.1\text{ dB}$) rings on $16$ crises and produces $245$ false alarms: $\mathrm{PPV} = 16 / 261 = 6.1\%$, giving an alarm fatigue rate of $\mathrm{FDR} = 93.9\%$ (`[Assumed]`).
- The throughput-filtered detector ($\mathrm{TPR} = 82\%, \mathrm{FPR} = 1.5\%, W = +17.4\text{ dB}$) rings on $16$ crises and only $15$ false alarms, raising $\mathrm{PPV}$ to $52.7\%$ ($16$ true alerts vs. $15$ false alarms per $1{,}000$ shifts, `[Assumed]`).

![Gigerenzer Natural Frequency Tree vs. Conditional Probability Fallacy](figures/fig-00-gigerenzer-tree.svg)

---

## 8.5 Laboratory 8: Real Data on Toronto Arrests (`toronto_arrests.csv`)

Running laboratory 8 (`cargo run --release --bin stochology -- --chapter 8`) evaluates [`governance.rs`](crates/stochology/src/governance.rs) on the Toronto Police Race and Identity-Based Data dataset (`data/toronto_arrests.csv`, $n = 5{,}226$, `[Measured]`):

```text
=== CHAPTER 8: FILTER III (S_GOV) — MORAL LUCK, APT AGENCY & INSTITUTIONAL DESIGN ===
  Canonical Operator III — Gate 2 (Real-Time S3* Algedonic Veto) & Gate 3 (Ex-Post Responsibility):
  1. Sosa / Baron-Hershey Telic Evaluation [Assumed]: grade = AdroitExAnteUnluckyOutcome, eliminated_outcome_bias = +0.45, vault_confirmatory = true (tamper_rejected=true, replay_rejected=true)
  2. List-Pettit & Beer S3* Algedonic Channel [Assumed]: premise_majority = true, conjunctive = false, discursive_dilemma = true
     Undiscounted TTR_recovery = 5 <= TTS_survival = 8 vs Discounted TTR_recovery = 12 > TTS_survival = 8 (fatal_delay=true)
  3. Holmstrom-Innes Liability & Simon-Girard Scapegoating [Assumed]: structural_share = 83.3%, scapegoat_transfer = 78.7% (blame_inversion_ratio = 13.6x), Bateson DB_t = 0.51 (brake_on = 0.00)
  4. Chouldechova Fairness Trilemma [Measured] (data/toronto_arrests.csv, n_all = 5226, n_matched = 891, n_poles = 3577): full marginal prev_Black = 0.259, prev_White = 0.142, odds_FPR_ratio = 2.11x | matched (Unemp+Citizen) prev_Black = 0.354, prev_White = 0.263, odds_FPR_ratio = 1.54x | intersectional poles prev_A = 0.393 (FPR_A=0.278), prev_B = 0.112 (FPR_B=0.054), FPR_A/FPR_B = 5.13x
     Empirical Custody-Risk Classifier [Assumed rule checks>=3 on Measured data/toronto_arrests.csv]: PPV_B=0.365/PPV_W=0.267, FNR_B=0.324/FNR_W=0.438, FPR_Black=0.409 vs FPR_White=0.256 (empirical FPR_ratio=1.60x, Chouldechova identity closure_err=1.11e-16)
  5. Zheng-Meister 10 bits/s & Gigerenzer Natural Frequencies [Assumed]: noisy FDR = 93.9% -> filtered PPV = 52.7% (16 true alerts vs 15 false alarms / 1,000 shifts)
     Two-Tier Cognitive Queue (TTS=45s): Kairos(30b,6s) rho=0.12, TTR=18.5s (ok=true) | Card(120b,18s) rho=0.48, TTR=40.8s | Full(240b,36s) rho=0.96, TTR=759.0s (ok=false)
```

- **Chouldechova's Trilemma Across Four Strata & Empirical Custody-Risk Classifier on `toronto_arrests.csv` ($n_{\mathrm{all}} = 5{,}226$, `[Measured]`):** To separate pure base-rate odds ratios under fixed $(\mathrm{PPV}, \mathrm{FNR})$ from covariate-matched and empirical classifier behavior, Chapter 49 reports four distinct audits on `data/toronto_arrests.csv` (`[Measured]`):
  1. **Full Race Marginal ($n_{\mathrm{all}} = 5{,}226$):** $p_{\mathrm{Black}} = 0.259$ vs. $p_{\mathrm{White}} = 0.142$, forcing an equal-$(\mathrm{PPV}, \mathrm{FNR})$ odds-ratio $\mathrm{FPR}$ disparity of **$2.11\times$** ($\mathrm{odds}(0.259) / \mathrm{odds}(0.142) = 0.3495 / 0.1655$, `[Measured]`);
  2. **Matched Covariate Stratum (`employed == "No" && citizen == "Yes"` for both races, $n_{\mathrm{matched}} = 891$):** Holding employment and citizenship identical yields $p_{\mathrm{Black}} = 0.354$ vs. $p_{\mathrm{White}} = 0.263$, forcing a **$1.54\times$** $\mathrm{FPR}$ disparity ($0.5480 / 0.3569$, `[Measured]`);
  3. **Intersectional Socioeconomic Poles ($n_{\mathrm{poles}} = 3{,}577$):** Comparing unemployed Black arrestees ($p_A = 0.393$) against employed White arrestees ($p_B = 0.112$) yields the extreme **$5.13\times$** odds-ratio upper bound ($\mathrm{FPR}_A = 0.278$ vs. $\mathrm{FPR}_B = 0.054$ at $\mathrm{PPV} = 0.65, \mathrm{FNR} = 0.20$, `[Measured]`);
  4. **Empirical Custody-Risk Classifier (`checks >= 3` predicting `released == "No"`, [`EmpiricalChouldechovaAudit`](crates/stochology/src/governance.rs#L561-L648)):** Evaluating the real decision rule $\hat{Y} = \mathbf{1}\lbrace \text{checks} \ge 3 \lor (\neg\text{employed} \land \neg\text{citizen} \land \text{checks} \ge 1)\rbrace$ across all $5{,}226$ arrests without fixing $\mathrm{PPV}$ or $\mathrm{FNR}$ yields empirical $\mathrm{PPV}_B = 0.365$ vs. $\mathrm{PPV}_W = 0.267$, $\mathrm{FNR}_B = 0.324$ vs. $\mathrm{FNR}_W = 0.438$, and empirical False Positive Rates $\mathrm{FPR}_{\mathrm{Black}} = 0.409$ vs. $\mathrm{FPR}_{\mathrm{White}} = 0.256$ (**$1.60\times$** empirical disparity, with Chouldechova's exact identity closing to **$1.11 \times 10^{-16}$**, `[Measured]`).
- **Fricker's Testimonial Discount vs. Time-to-Survival Buffer ([`AlgedonicChannel`](crates/stochology/src/governance.rs#L125-L165), `[Assumed]`):** With frontline distress $e = 2.10$ and $\mathrm{TTS}_{\mathrm{survival}} = 8$ shifts, an undiscounted algedonic channel trips the emergency circuit breaker in **$5$ shifts** ($\le \mathrm{TTS}_{\mathrm{survival}}$), whereas a testimonial discount of $\delta_{\mathrm{cred}} = 0.65$ delays detection to **$12$ shifts** ($> \mathrm{TTS}_{\mathrm{survival}}$), causing fatal ruin (`[Assumed]`).
- **Two-Tier Cognitive Queue Under Zheng & Meister's $10\text{ bits/s}$ Limit ([`KairosSynchronousPanel`](crates/stochology/src/governance.rs#L352-L392) & [`ZhengMeisterQueueAudit`](crates/stochology/src/governance.rs#L398-L463), `[Assumed]`):** Confronting an operator with the uncompressed 36-scalar audit ($240\text{ bits}$) drives cognitive queue utilization to $\rho_{\mathrm{cog}} = 0.96 > 0.76$ and $\mathrm{TTR}_{\mathrm{cog}} = 759.0\text{ s} \gg \mathrm{TTS}_{\mathrm{survival}} = 45.0\text{ s}$, inducing Collapse Proposition 2 via cognitive overload. Compressing the real-time synchronous *Kairós* display to 6 sufficient statistics ($\le 30\text{ bits}$, `KairosSynchronousPanel`) reduces utilization to $\rho_{\mathrm{cog}} = 0.12$ and $\mathrm{TTR}_{\mathrm{cog}} = 18.5\text{ s} \le 45.0\text{ s}$ (`[Assumed]`).
- **Simon–Girard Scapegoat Blame Transfer & Bateson Double-Bind Trap ([`ContractAndLiabilityAudit`](crates/stochology/src/governance.rs#L255-L313) & [`BatesonDoubleBindAudit`](crates/stochology/src/governance.rs#L171-L207), `[Simulated]`):** Whereas executive structural sizing causes $83.3\%$ of tail-deficit variance versus $6.3\%$ frontline operational noise, scapegoating the frontline at $b_{\mathrm{frontline}} = 85.0\%$ shifts **$+78.7\text{ percentage points}$** of excess blame downward ($13.6\times$ blame inversion), while the combination of $\beta_{\mathrm{proxy}} = 0.85$, $\delta_{\mathrm{cred}} = 0.60$, and disabled $S3^*$ veto locks operators into a Bateson double-bind intensity of $\mathrm{DB}_t = 0.510$ (versus $0.000$ under `FullTriad`, `[Simulated]`).

---

## Exercises

1. **Chouldechova's Fairness Disparity Under Shifting Base Rates:**
   Using Chouldechova's identity $\mathrm{FPR}_g = \mathrm{odds}(p_g) \frac{1 - \mathrm{PPV}_g}{\mathrm{PPV}_g} (1 - \mathrm{FNR}_g)$, suppose an institutional credit or triage algorithm achieves calibration parity $\mathrm{PPV} = 0.70$ and equal opportunity $\mathrm{TPR} = 0.75$ ($\mathrm{FNR} = 0.25$) across two populations: Group 1 with base rate $p_1 = 0.40$ and Group 2 with base rate $p_2 = 0.15$.
   - Calculate the exact $\mathrm{FPR}$ for Group 1 and Group 2.
   - Show that the disparity ratio $\mathrm{FPR}_1 / \mathrm{FPR}_2$ matches the base-rate odds ratio $\mathrm{odds}(p_1) / \mathrm{odds}(p_2)$ exactly.
   - *Guidance:* First compute $\mathrm{odds}(0.40) = 0.40 / 0.60 = 2/3 \approx 0.667$ and $\mathrm{odds}(0.15) = 0.15 / 0.85 \approx 0.1765$. Multiply by $\frac{0.30}{0.70} \times 0.75 \approx 0.3214$. Then verify that $\mathrm{FPR}_1 / \mathrm{FPR}_2 = 0.667 / 0.1765 = 3.78\times$.

2. **List–Pettit Discursive Dilemma and Veto Design:**
   Construct a $4\times 3$ voting matrix for four committee members auditing a high-stakes civil defense deployment requiring Epistemic ($P_1$), Vital ($P_2$), and Governance ($P_3$) clearance.
   - Design votes such that every member rejects deployment ($C_i = \text{False}$ for all $i \in \{1, 2, 3, 4\}$), yet premise-majority voting approves each individual premise by $3/4$ ($75\%$).
   - Explain why an institutional constitution that decides deployment by premise-majority creates catastrophic group irrationality, and show how [`GovernanceState::evaluate_list_pettit_votes`](crates/stochology/src/governance.rs#L1814-L1831) solves this via the conjunctive brake.
   - *Guidance:* Set voter patterns to: Member 1: $(T, T, F)$, Member 2: $(T, F, T)$, Member 3: $(F, T, T)$, Member 4: $(T, T, F)$ ensuring premise sums are $3/4$ on each premise while all $C_i = F$.

3. **Single-Opening Cryptographic Commitment with TemporalVault:**
   Inspect [`TemporalVault`](crates/stochology/src/governance.rs#L472-L556) in `crates/stochology/src/governance.rs`.
   - Explain how RFC 2104 HMAC-SHA256 authenticated pre-registration prevents Baron & Hershey's outcome bias from rewriting ex-ante evaluation standards $C(\pi_t \mid \mathcal{I}_{t_{\mathrm{disp}}})$.
   - What happens when an institution attempts to unseal the vault a second time or with modified parameters? Trace the execution through `unseal` and explain why `is_confirmatory` transitions permanently to `false`.
   - *Guidance:* Review lines 514–535 of `governance.rs`. A second opening hits `if self.open_count >= 1 { self.is_confirmatory = false; return Err(...); }`, preventing post-hoc parameter tweaking and forensic fraud.

4. **Collapse Proposition 3 in `CommunityReserveEnv`:**
   Run `StochologicalAgent::without_governance()` (`use_epistemic_margin = true, use_vital_margin = true, use_institutional_governance = false`). Trace how `proxy_incentive_pressure = 0.85` and `frontline_testimonial_discount = 0.60` pillage the vital reserve that the first two safeguards built.
   - *Guidance:* In `engine.rs`, notice how `frontline_testimonial_discount` suppresses algedonic alarm escalation while `proxy_incentive_pressure` transfers vital reserve surplus into short-term proxy bonuses, accelerating ruin.
