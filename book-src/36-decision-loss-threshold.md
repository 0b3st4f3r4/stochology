# Decision, Loss, Threshold

> Foundation chapter of Part III. It presupposes [Chapter 12](12-derivatives-and-convexity.md)
> and [Chapter 19](19-ethics-and-the-imposition-of-risk.md). Its formal content is worked out
> in [Chapter 43](43-inquiry-openness-multiverse.md).

## By the end of this chapter you can

1. Turn a decision problem into costs and losses, and say which of the two you are allowed to
   estimate.
2. Derive the threshold at which acting beats waiting, and read it as a property of the
   consequences rather than of the evidence.
3. Say what information would be worth paying for, and when the answer is *none*.

## 1. Actions, states, and the loss matrix

A decision problem consists of three formal structures: the set of available actions $\mathcal{A}$,
the set of possible states of the world $\mathcal{S}$, and the loss function $\mathcal{L}(a, s)$ mapping
each action-state pair to a consequence. Statistical models and historical archives provide evidence solely
about the probability distribution over states $\mathbb{P}(s)$. Everything else — the enumeration of allowable
actions and the valuation of harms — is an ethical and institutional choice.

The loss matrix is therefore a constitutional document. Specifying that a false alarm costs ten units while
a missed catastrophe costs a thousand units is not an empirical measurement; it is an explicit valuation of
competing harms, decided by an author, with grave consequences for those who bear the downside. Most debates
in public policy that present themselves as disagreements over statistical evidence are in reality disguised
conflicts over the entries of the loss matrix.

## 2. The cost–loss threshold and the Pauker-Kassirer form

Two thresholds are routinely filed under one name, and the difference is not cosmetic: the library implements
one of them, and a reader who substitutes the other will compute a different boundary from the same two budgets.

The **cost–loss threshold** comes from a preventive action that incurs a certain protection cost $C > 0$ and
averts a fraction $\eta \in (0, 1]$ of an avoidable loss $L > C$. Let $p$ be the probability that the harmful
event occurs. Acting costs $C$ whatever happens and still leaves $(1 - \eta)L$ to be paid when the event
occurs; not acting costs $L$ when it occurs. Equating the two expected expenses gives:

$$p^*_{\mathrm{CL}} = \frac{C}{\eta L} \qquad \text{\texttt{[Assumed]}}.$$

The **Pauker–Kassirer threshold** (1975) is instead an expected-regret comparison between two classifications.
Let $C_{\mathrm{FP}}$ be the net cost of a false alarm — acting when no event occurs — and $L_{\mathrm{FN}}$
the net loss of a missed event — failing to act when the event occurs. Acting whenever
$p\,L_{\mathrm{FN}} > (1 - p)\,C_{\mathrm{FP}}$ gives

$$p^*_{\mathrm{PK}} = \frac{C_{\mathrm{FP}}}{C_{\mathrm{FP}} + L_{\mathrm{FN}}} = \frac{1}{1 + L_{\mathrm{FN}}/C_{\mathrm{FP}}} \qquad \text{\texttt{[Assumed]}},$$

which is exactly what `epistemic.rs::pauker_kassirer_threshold` computes when handed its two non-negative
terms, $c = C_{\mathrm{FP}}$ and $l = L_{\mathrm{FN}}$, returning $c/(c + l)$ and falling back to parity when
both budgets vanish. The cost–loss form asks whether a protection budget is worth its expected saving; the
expected-regret form asks which of two errors is cheaper. Institutions budget the second, so both are quoted
here and always with the subscript that names them.

The decision rule is the same in either form: execute the intervention when the estimated probability of
danger exceeds the threshold, and withhold it when the estimate falls below.

When an intermediate diagnostic test is available with cost $C_{\mathrm{test}}$, sensitivity $\mathrm{Sens}$,
and specificity $\mathrm{Spec}$, Pauker and Kassirer demonstrated that the decision space partitions into three
distinct operational regimes:

$$\text{Withhold Action} \quad \longleftarrow \quad p^*_{\mathrm{test}} \quad \longleftarrow \quad \text{Order Diagnostic Test} \quad \longrightarrow \quad p^*_{\mathrm{treat}} \quad \longrightarrow \quad \text{Act Immediately} \qquad \text{\texttt{[Assumed]}}.$$

The testing threshold $p^*_{\mathrm{test}}$ defines the point where the expected value of diagnostic clarification
exceeds the physical and financial cost of the test. Below $p^*_{\mathrm{test}}$, the disease is so improbable that
even a positive test result would likely be a false alarm; above $p^*_{\mathrm{treat}}$, the disease is so probable
and dangerous that delaying intervention to wait for test results is malpractice.

Two decisive properties govern the threshold:
1. *The threshold is a property of consequences, not of evidence.* Two agents evaluating the identical probability
   estimate $\hat{p} = 0.08$ will rationally choose opposite actions if one faces a protection cost of $C=1$
   with avoidable loss $L=100$ ($p^*_{\mathrm{CL}} = 0.01$, so act) while the other faces $C=10$ with $L=50$
   ($p^*_{\mathrm{CL}} = 0.20$, so wait). Reporting only the probability $\hat{p}$ without publishing the
   consequence threshold conceals the contested core of the decision.
2. *The threshold routinely sits far from one-half.* Where intervention is cheap and harm is catastrophic, the
   threshold plummets to a few percent. Demanding "proof beyond a reasonable doubt" before evacuating a flood
   plain or administering emergency prophylaxis is reckless negligence.

## 3. Asymmetric loss, standards of proof, and Minimax Regret

Legal traditions formalize this consequence ratio directly into their historical standards of proof `[Literature]`:
- **Preponderance of the evidence**: In private civil disputes where an erroneous verdict in either direction
  carries equal economic gravity, the decision threshold sits just above parity: $p^* = 0.50 + \epsilon$.
- **Clear and convincing evidence**: Where civil sanctions threaten liberty, professional licensure, or parental
  rights, the threshold rises to roughly $p^* \approx 0.75$ `[Assumed]`.
- **Beyond a reasonable doubt**: In criminal prosecution, Blackstone's ratio asserts that it is better that ten
  guilty persons escape than that one innocent suffer, i.e. a false conviction is ten times as costly as a false
  acquittal ($C_{\mathrm{FP}} = 10 L_{\mathrm{FN}}$). Read through the expected-regret form of §2, the threshold
  becomes:
  $$p^*_{\mathrm{PK}} = \frac{10}{10 + 1} = \frac{10}{11} \approx 0.909 \qquad \text{\texttt{[Assumed]}}.$$
- **Learned Hand's negligence formula**: In tort law, Judge Learned Hand defined negligence as failing to take
  precautions where $B < PL$ (Burden $B$, Probability $P$, Loss $L$). Setting $C = B$, $L$ equal to the accident
  loss and $\eta = 1$ reduces the cost–loss threshold of §2 to $P > B/L = p^*_{\mathrm{CL}}$, stated as a legal
  duty of care. When an injurer's downside is truncated by limited corporate liability
  (`governance.rs::ContractAndLiabilityAudit`), their private $L_{\mathrm{private}}$ collapses, driving their
  private threshold $p^*_{\mathrm{private}} = B/L_{\mathrm{private}}$ far above the social optimum and
  externalizing catastrophe onto the public.

When the probability distribution is not known as a single point, but is partially identified within an interval
$[L_t, U_t]$ that straddles the operational threshold $C$ ($L_t < C < U_t$), committing entirely to zero or full
action risks severe error. Under the principle of **Minimax Regret**, the optimal fractional commitment that
minimizes the maximum possible regret is:

$$a^*_{\mathrm{MMR}} = \frac{U_t - C}{U_t - L_t} \in (0, 1) \qquad \text{\texttt{[Assumed]}}.$$

The throttle of the commitment is proportional to the distance between the threshold and the boundaries of what is
unknown.

## 4. The Expected Value of Perfect Information (EVPI)

Before spending organizational resources to collect more data, an institution must calculate what that information
could physically change. The **Expected Value of Perfect Information (EVPI)** is the mathematical difference between
the expected loss of the optimal action chosen under current uncertainty and the expected loss if the true state of
the world were revealed prior to deciding:

$$\mathrm{EVPI} = \min_{a} \mathbb{E}_s\left[\mathcal{L}(a, s)\right] - \mathbb{E}_s\left[\min_{a} \mathcal{L}(a, s)\right] \qquad \text{\texttt{[Assumed]}}.$$

Information possesses an EVPI of exactly zero in three prevalent circumstances:
1. *Irrelevance*: The uncertain variable has no causal bearing on the payoff matrix.
2. *Dominance*: The optimal action is identical across all possible states that the inquiry could reveal.
3. *Latency expiration*: The data collection process takes longer than the operational window to act; the deadline
   forces the decision before the evidence can be delivered.

## 5. Cognitive alert filtering and the information bottleneck

An operational alert system is an automated classifier with a human operator at the terminus. System designers
routinely evaluate detectors by sensitivity and specificity, ignoring that human operators experience only the
**Positive Predictive Value (PPV)**:

$$\mathrm{PPV} = \frac{\mathrm{Sens} \times \text{Prevalence}}{(\mathrm{Sens} \times \text{Prevalence}) + ((1 - \mathrm{Spec}) \times (1 - \text{Prevalence}))} \qquad \text{\texttt{[Assumed]}}.$$

When detecting rare clinical emergencies or cyber intrusions ($\text{Prevalence} = 0.005$), a system with $95\%$
sensitivity and $95\%$ specificity produces an operational PPV of less than $9\%$ `[Assumed]`. More than nine out
of ten alarms are spurious.

Human cognitive psychology and neurobiology impose an unyielding physical constraint on the receiver.
In 2025, Zheng and Meister demonstrated that conscious human behavioral throughput is strictly bounded at
approximately **ten bits per second** ($\sim 10\text{ bits/s}$) `[Literature]`. An institutional alert stream that
inundates human operators with alarms exceeding this throughput capacity triggers an inevitable behavioral collapse:
operators experience **alert fatigue**, dismissing alarms reflexively without conscious cognitive processing.
A high-sensitivity model on paper becomes a zero-sensitivity filter in practice.

Effective alert design must run in reverse from the human bottleneck: calibrate the alarm volume to the operator's
ten-bit bandwidth, allocate the alert budget across priority hazards, and present notifications as natural
frequencies rather than abstract conditional probabilities.

## Bridge

The cost–loss threshold and its expected value of perfect information are formalised in
[Chapter 32](32-the-question-before-the-data.md) §4, while the expected-regret form of §2 is implemented as
`epistemic.rs::pauker_kassirer_threshold`; the audit of limited-liability asymmetry and the Hand formula is
governed by `governance.rs::ContractAndLiabilityAudit`; and the operator's bounded cognitive channel capacity
evaluated against alert streams is executed by `governance.rs::CognitiveAlertFilter::evaluate` in
[Chapter 49](49-moral-luck-apt-agency-institutions.md).
