# Institution, Incentive, Metric

> Foundation chapter of Part III. Its formal content is worked out in
> [Chapter 49](49-moral-luck-apt-agency-institutions.md).

## By the end of this chapter you can

1. Explain how a measure becomes a target, and name the four ways it then stops measuring.
2. Trace a metric from the person who is scored to the person who chose the score.
3. Say what an audit channel must have to work — access, authority, and a consequence — and what
   happens when one of the three is missing.

## 1. The measure and the target: four Goodhart variants

A metric is originally introduced to render a complex operational state visible to decision-makers. Once visible,
the number is used to compare units; once compared, it is tied to institutional incentives, promotions, and budgets.
At that precise moment, the measure becomes a target, and human behavior reorganizes itself around maximizing
the proxy rather than advancing the underlying mission.

This degeneration does not require dishonesty. It is the predictable outcome of intelligent, competent agents
adapting to an evaluation rule. In modern organizations, metric collapse takes four distinct structural forms:
- **Regressional**: The metric is subject to random noise. Selecting or rewarding extreme performances selects
  heavily for positive luck; performance in subsequent cycles inevitably regresses to the population mean,
  generating a spurious cycle of perceived decline.
- **Extremal**: The target is achieved by driving the operational system to an extreme margin where the historical
  relationship between proxy and purpose ruptures. For instance, reducing patient wait times by discharging patients
  before stabilization.
- **Causal**: The measured index was historically correlated with the goal, but does not cause it. Direct effort
  manipulates the correlate while the true goal remains untouched.
- **Adversarial**: Strategic actors deliberately game, falsify, or gatekeep the intake of cases to flatter the
  metrics (e.g., crime classification downgrading, teaching to the test).

The first three collapses are structural and occur among dedicated professionals doing their jobs with integrity.
They account for the vast majority of institutional failures in the historical record.

## 2. The asymmetry of the metric and blame transfer

A metric invariably possesses an author and a subject, and in hierarchical systems they occupy vastly unequal
positions of power:
- Those who author the metric establish the formulas, choose the denominators, dictate the exclusions, and set
  the performance thresholds.
- Those who are scored control only the frontline execution of the work under the constraints handed down to them.

Into this structural asymmetry, the legal architecture inserts **limited corporate liability**. When an executive
or algorithm designer captures the private upside of risk-taking while corporate reserves absorb the downside,
the private payoff of recklessness is positive even when the net social value is deeply negative. The first asset
cannibalized by this moral hazard is the organization's unmeasured resilience buffer.

When systemic collapse eventually occurs, the institutional assignment of blame is systematically inverted.
Total deficit variance decomposes into two components: *structural variance* (governed by capacity sizing, staffing
ratios, and parameter budgets set at the top) and *operational variance* (stochastic noise handled by frontline
shifts). In complex systems, structural variance accounts for over ninety percent of failure causality. Yet
institutions almost invariably assign one hundred percent of the blame to the frontline operators on duty when
the barrier ruptured. This blame transfer destroys organizational epistemology: a frontline that expects to be
scapegoated for structural deficits will cease reporting early warning anomalies, blinding the institution to its
own impending ruin.

## 3. The three audit requirements and the algedonic channel

An effective institutional review channel requires three non-negotiable operational properties:
1. **Observability**: Unfiltered, direct access to the operational phenomenon without prior sanitization.
2. **Authority**: Binding jurisdiction to command an operational halt, pause, or rollback.
3. **Consequence**: The investigation must produce enforceable structural modifications and genuine liability.

If any one of the three is absent, the audit is a hollow ritual.

In cybernetics, Stafford Beer established that hierarchical reporting structures inevitably filter out bad news.
Because bad news constitutes an implicit indictment of the managerial levels it traverses, each tier has a powerful
incentive to dilute, delay, or suppress the signal. Beer designed the **algedonic channel** — a dedicated,
out-of-band communication loop carrying raw distress signals directly past the administrative hierarchy to sovereign
veto holders.

An algedonic channel is authentic rather than cosmetic if and only if it embodies three strict operational rules:
- **Independent Standing**: The route exists prior to emergencies and requires no permission to use. It cannot be
  conditioned on "exhausting standard reporting lines", and the criticized hierarchy cannot monitor the identities
  of those who activate it.
- **Zero Discount at Reception ($\delta_{\mathrm{cred}} = 0$)**: The distress warning is evaluated at its substantive
  face value. Institutional prejudices regarding status, rank, or credentials cannot be used to discount the message.
  Setting $\delta_{\mathrm{cred}} > 0$ stretches the latency of detection beyond the system's survival buffer.
- **The Halt Holds**: The channel terminates at an authority with binding legal power to halt operations immediately,
  and the halt stands until the physical safety condition is verified. A reporting channel that can only file a
  memo is merely an organizational vent.

## 4. Process tracing and Bayesian diagnostic tests

When auditing institutional failures or verifying causal mechanisms, investigators use **process tracing**
(Bennett and Checkel 2015). A causal hypothesis $H$ is evaluated against historical clues using four canonical
diagnostic tests:

```
                          IS EVIDENCE SUFFICIENT TO CONFIRM?
                                NO                    YES
                     +---------------------+---------------------+
                  NO | STRAW-IN-THE-WIND   | SMOKING GUN         |
IS EVIDENCE          | Low diagnostic power| Passing confirms H; |
NECESSARY            | Updates prior weakly| Failing leaves open |
TO SURVIVE?          +---------------------+---------------------+
                 YES | HOOP TEST           | DOUBLY DECISIVE     |
                     | Passing does not    | Single-handedly     |
                     | confirm; FAIL KILLS | confirms H & kills  |
                     +---------------------+---------------------+
```

- **Straw-in-the-Wind**: Neither necessary nor sufficient. Provides minor probability updating, but cannot affirm
  or eliminate $H$.
- **Hoop Test**: Necessary but not sufficient. The hypothesis must "jump through the hoop" to survive. Passing the
  test does not confirm $H$, but failing it decisively refutes and eliminates $H$.
- **Smoking Gun**: Sufficient but not necessary. Finding the smoking gun in the suspect's hand confirms guilt ($H$),
  but the absence of a smoking gun does not exonerate.
- **Doubly Decisive**: Both necessary and sufficient. Concurrently confirms $H$ and definitively refutes all rival
  explanations.

Following Alan Turing and I. J. Good, the diagnostic weight of an evidentiary clue is measured in **decibans (db)**:

$$W = 10 \log_{10}\left(\frac{\mathbb{P}(E \mid H)}{\mathbb{P}(E \mid \neg H)}\right) \text{ db} \qquad \text{\texttt{[Assumed]}}.$$

A failed hoop test delivers infinite negative decibans, instantly terminating the hypothesis.

## 5. Deliberation paradoxes, commons governance, and the Temporal Vault

In collective institutional governance, Christian List and Philip Pettit discovered the **discursive dilemma**
(or doctrinal paradox). Consider a multi-member tribunal evaluating three propositions: premise $P$, premise $Q$,
and conclusion $R = P \land Q$. If a majority supports $P$ ($2/3$) and a separate majority supports $Q$ ($2/3$),
a premise-based voting procedure rationally affirms $R$ (`governance.rs::evaluate_list_pettit_votes`). However,
if members vote directly on the conclusion $R$, a majority may reject $R$ ($2/3$), producing self-contradictory
institutional acts. Premise-based governance is the only structure that preserves collective rationality.

Furthermore, following Elinor Ostrom's Nobel-winning principles for common-pool resources
(`Governing the Commons` `[Literature]`), long-enduring institutions require auditable field monitoring, graduated
sanctions, and rapid conflict resolution mechanisms, modeled in `engine.rs::CommunityReserveEnv`. This stands in
stark contrast to the high-modernist monoculture diagnosed by James C. Scott in *Seeing Like a State*: when
Prussian foresters replaced diverse ecologies with a single metric of commercial timber (*Normalbaum*), the
first generation produced record volume while the second collapsed in catastrophic forest death (*Waldsterben*).

To inoculate an institution against hindsight bias and post-hoc narrative laundering, all hypotheses, parameter
bounds, and reserve allocations must be cryptographically committed in a **Temporal Vault**
(`governance.rs::TemporalVault::seal`) before the wind reports. The question *what did we know, and what did we
commit to, prior to the outcome?* must have an immutable answer.

## Bridge

Metric capture, blame inversion, and the interrupting algedonic channel are formalised in
[Chapter 49](49-moral-luck-apt-agency-institutions.md) as
`governance.rs::from_real_time_and_ex_post_gates`; the algorithmic disparity audit across protected groups is
executed by `governance.rs::EmpiricalChouldechovaAudit::evaluate`; collective voting paradoxes are evaluated by
`governance.rs::evaluate_list_pettit_votes`; and cryptographic pre-outcome commitment is enforced by
`governance.rs::TemporalVault::seal`. The reflexivity audit that protects this framework from its own metric gaming
is `synthesis.rs::ReflexiveBrakeAudit::evaluate`.
