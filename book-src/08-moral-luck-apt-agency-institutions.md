# Chapter 8 — Moral Luck, Apt Agency, and Institutional Design

Suppose an agent has both an **Epistemic Margin** (Chapters 2–6) and a **Vital Margin** (Chapter 7). What happens if the surrounding **institution** rewards reckless gamblers when they get lucky, punishes prudent operators when a tail storm strikes, aggregates committee votes by premise-majority without a safety veto, discounts warnings from frontline nurses or field technicians, and floods human operators with $94\\%$ false-positive alerts?

That is **Collapse Proposition 3** (*cynical capture and pillaging of reserves*). **Tension 7** (the ethical and institutional mediation of Stochology, implemented in [`crates/stochology/src/governance.rs`](../crates/stochology/src/governance.rs)) resolves the conflict between **ex-ante telic care (*télos*), ex-post outcome luck (*skopós*), and institutional governance** (Beer, 1972; Nagel, 1979; Holmström, 1979; Williams, 1981; Baron & Hershey, 1988; Innes, 1990; Holmström & Milgrom, 1991; Gigerenzer & Hoffrage, 1995; Scanlon, 1998; Sosa, 2007, 2015; Fricker, 2007, 2020; Fantl & McGrath, 2009; List & Pettit, 2011; Hansson, 2013; Chouldechova, 2017; Wong et al., 2021; Zheng & Meister, 2025).

![Single-Opening Cryptographic Temporal Vault](figures/fig-10-temporal-vault.svg)

---

## 8.1 Sosa's Virtue Epistemology and Baron–Hershey Bitemporal Telic Evaluation

In *Moral Luck* (1979/1981), Bernard Williams and Thomas Nagel showed how ordinary moral judgment is corrupted by the stochastic outcome of an action after it leaves the agent's hands, while Jonathan Baron and John C. Hershey (1988) demonstrated empirically that human evaluators suffer from **outcome bias**: judging the exact same ex-ante decision as "competent" when the coin lands heads and "negligent" when it lands tails.

Ernest Sosa (2007, 2015) provided the exact conceptual grammar to overcome outcome bias through his **AAA** (*Accuracy, Adroitness, Aptness*) and second-order **Fully Apt** virtue epistemology ([`SosaJudgmentGrade`](../crates/stochology/src/governance.rs#L17-L28) & [`TelicEvaluation`](../crates/stochology/src/governance.rs#L33-L102)):
- **`InaptAndInaccurate`:** Negligent ex-ante method and bad outcome;
- **`LuckyAccurateOnly`:** Negligent ex-ante method (e.g., unbuffered $98\\%$ utilization) that escaped disaster purely by good weather—must **never** be rewarded;
- **`AdroitExAnteUnluckyOutcome`:** Rigorous ex-ante competence (*télos*) and vital buffering that suffered an exogenous tail shock—must **never** be scapegoated;
- **`FullyAptPerformance`:** Success achieved *because* of competence, or reflective meta-competence that recognizes when epistemic weight $V$ is too low and suspends risky action in favor of the vital safeguard.

To enforce this separation institutionally, [`TelicEvaluation`](../crates/stochology/src/governance.rs#L33-L102) and [`TemporalVault`](../crates/stochology/src/governance.rs#L381-L416) seal the Stage-1 evaluation of ex-ante competence $C(\pi\_t \mid \mathcal{I}\_{t\_{\mathrm{disp}}})$ inside a single-opening SHA-256 cryptographic commitment **before** the outcome $Y\_{t+1}$ is revealed.

---

## 8.2 List–Pettit Discursive Dilemma and Fricker's Testimonial Injustice on Beer's Algedonic Channel

Institutions do not think with a single brain; they aggregate distributed judgments:
1. **Christian List & Philip Pettit's (2011) Discursive Dilemma ([`DiscursiveCommittee`](../crates/stochology/src/governance.rs#L107-L154)):** Suppose deployment requires all threeaxes of the Nuclear Triad ($P\_1 \land P\_2 \land P\_3$) to hold, and three specialists vote: Specialist 1 approves $(T, T, F)$, Specialist 2 approves $(T, F, T)$, and Specialist 3 approves $(F, T, T)$. Every single specialist sees a fatal flaw ($0/3$ conclusion approval), yet **premise-by-premise majority voting** approves each axis $(2/3, 2/3, 2/3)$ and launches the unsafe system! Only a **constitutional conjunctive veto** (Stafford Beer's $S3^\*$ sporadic audit channel) prevents the discursive dilemma.
2. **Miranda Fricker's (2007) Testimonial Epistemic Injustice on Beer's Algedonic Channel ([`AlgedonicChannel`](../crates/stochology/src/governance.rs#L159-L202)):** In Beer's Viable System Model (1972), an **algedonic signal** (pain/distress $e\_t > 1$) bypasses bureaucratic hierarchy to trip the emergency circuit breaker before Time-to-Survive ($\mathrm{TTS}$) runs out. When management applies a **credibility discount** $\delta\_{\mathrm{cred}} \in (0, 1)$ to frontline workers or affected residents ($e\_{\mathrm{discounted}} = e\_t^{1 - \delta\_{\mathrm{cred}}}$), the number of shifts required to cross the alarm threshold $\frac{\ln(1/\alpha)}{(1 - \delta\_{\mathrm{cred}})\ln e\_t}$ stretches past $\mathrm{TTS}$, turning testimonial injustice into physical catastrophe.

---

## 8.3 Innes–Holmström–Milgrom Contracts and Chouldechova's Fairness Trilemma

How do contracts and classifiers distribute risk and error across people?
1. **Innes (1990) Limited Liability & Holmström–Milgrom (1991) Multitask Care ([`ContractAndLiabilityAudit`](../crates/stochology/src/governance.rs#L207-L280)):** Under limited liability, an executive who captures upside bonus $B$ while truncating downside ruin loss $-L$ at zero has positive private expected value $p B > 0$ for a reckless gamble whose social expected value $p B - (1-p)L < 0$ is deeply negative. Only **clawback escrow** $\gamma\_{\mathrm{clawback}}$ restores alignment. Conversely, for frontline caregivers performing both a countable volume proxy $a\_1$ and unmeasured qualitative care $a\_2$, high-powered piece rates $\beta\_1 \gg 0$ cannibalize unmeasured care ($\Delta a\_2 < 0$). And by Bengt Holmström's (1979) **Informativeness Principle**, structural queue collapse ($\Delta\_{\mathrm{struct}}$ from management's capacity sizing) must be attributed to structural leadership, never scapegoated onto frontline operators.
2. **Alexandra Chouldechova's (2017) Algebraic Fairness Trilemma ([`ChouldechovaTrilemma`](../crates/stochology/src/governance.rs#L285-L333)):** For any binary classification score with equal Positive Predictive Value ($\mathrm{PPV}$) and equal True Positive Rate ($\mathrm{TPR} = 1 - \mathrm{FNR}$) across two groups $A$ and $B$ whose base rates $p\_A \ne p\_B$ differ (because of historical and structural inequality), elementary algebra forces the False Positive Rate ratio to be:

   $$\frac{\mathrm{FPR}\_A}{\mathrm{FPR}\_B} = \frac{p\_A / (1 - p\_A)}{p\_B / (1 - p\_B)} \ne 1$$

   No threshold tweak can make $\mathrm{FPR}\_A = \mathrm{FPR}\_B$ without sacrificing either predictive parity or equal opportunity; the institution must explicitly govern **who pays for error**.

---

## 8.4 Zheng–Meister's $10\text{ bits/s}$ Bottleneck and Gigerenzer's Natural Frequencies

Jieyu Zheng and Markus Meister (2025) established that human conscious decision-making processes information at an upper bound of roughly **$10\text{ bits/second}$**, while Andrew Wong et al. (2021) documented that the widely deployed Epic Sepsis Model generated an **$88\\%$ false discovery rate** ($\mathrm{PPV} = 12\\%$, Informative), causing severe clinical alert fatigue.

In [`CognitiveAlertFilter`](../crates/stochology/src/governance.rs#L338-L376), every alarm rule is audited by its Turing–Good deciban weight $W = 10\log\_{10}(\mathrm{TPR}/\mathrm{FPR})$, its shift-level $\mathrm{PPV}$, and Gerd Gigerenzer & Ulrich Hoffrage's (1995) **integer natural frequency tree** out of $N = 1{,}000$ shifts (*"Out of 1,000 shifts, 20 have a true crisis; the filter rings on 16 of them and produces 15 false alarms"*).

![Gigerenzer Natural Frequency Tree vs. Conditional Probability Fallacy](figures/fig-00-gigerenzer-tree.svg)

---

## 8.5 Chapter 8 Laboratory: Real Data on Toronto Arrests (`toronto_arrests.csv`)

Running Chapter 8 (`cargo run --release --bin stochology -- --chapter 8`) evaluates [`governance.rs`](../crates/stochology/src/governance.rs) on the Toronto Police Race and Identity-Based Data dataset (`data/toronto_arrests.csv`, $n = 5{,}226$, Measured):

```text
=== Chapter 8: Moral Luck, Apt Agency & Institutional Design (Tension 7) ===
  Toronto Arrests Chouldechova (toronto_arrests.csv): p_A=0.393, p_B=0.112 -> FPR_A/FPR_B disparity=5.13x
  List-Pettit Discursive Dilemma: premise_maj=true vs constitutional_veto=false | Fricker algedonic delay: 5 -> 11 shifts (TTS=8)
  Sosa/Baron-Hershey grade=AdroitExAnteUnluckyOutcome (bias_removed=0.45) | Holmstrom structural_liability=83.3% vs frontline=6.2%
  Zheng-Meister & Gigerenzer Alerts (N=1000): noisy FDR=93.9% vs filtered PPV=52.7% (16 true alerts, 15 false alarms)
```

- **Chouldechova's Trilemma on `toronto_arrests.csv` ($n = 5{,}226$, Measured):** Because recorded base rates differ across demographic strata ($p\_A = 0.393$ vs. $p\_B = 0.112$), any risk instrument calibrated for equal $\mathrm{PPV}$ and equal $\mathrm{TPR}$ inflicts a **$5.13\times$ higher False Positive Rate** on Group $A$ than on Group $B$.
- **Fricker's Testimonial Discount vs. Time-to-Survive (Analytical):** With frontline distress $e = 1.85$ and $\mathrm{TTS} = 8$ shifts, an undiscounted algedonic channel trips the emergency circuit breaker in **$5$ shifts** ($\le \mathrm{TTS}$), whereas a testimonial discount of $\delta\_{\mathrm{cred}} = 0.55$ delays detection to **$11$ shifts** ($> \mathrm{TTS}$), causing fatal ruin.

---

## Exercises

1. **Collapse Proposition 3 in `CommunityReserveEnv`:** Run `StochologicalAgent::without_governance()` (`use_epistemic_margin = true, use_vital_margin = true, use_institutional_governance = false`). Trace how `proxy_incentive_pressure = 0.85` and `frontline_testimonial_discount = 0.60` pillage the vital reserve that the first two safeguards built.
2. **Natural Frequency Translation:** Using [`CognitiveAlertFilter::evaluate(0.02, 0.80, 0.25, 1000)`](../crates/stochology/src/governance.rs#L356-L375), write out the 4-node Gigerenzer natural frequency tree for the noisy detector and explain why $\mathrm{FDR} = 93.9\\%$ even though $\mathrm{TPR} = 80\\%$ and $\mathrm{FPR} = 25\\%$.

