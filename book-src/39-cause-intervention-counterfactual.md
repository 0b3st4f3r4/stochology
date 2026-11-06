# Cause, Intervention, Counterfactual

> Foundation chapter of Part III. Its formal content is worked out in
> [Chapter 48](48-reactive-worlds-performativity-goodhart.md).

## By the end of this chapter you can

1. Separate *what goes with what* from *what happens if we act*, and say which one a decision needs.
2. Read a causal claim and name the assumption that would make it false.
3. Ask a counterfactual question precisely enough that it has one answer instead of many.

## 1. Pearl's three-rung causal hierarchy

Causal queries are not all of the same mathematical species; they inhabit three strictly hierarchical levels,
formalized by Judea Pearl as the **Ladder of Causation**:

```
Level 3: COUNTERFACTUALS  P(y_x' | x, y)  "What if we had acted differently? Why did it happen?"
         ^
Level 2: INTERVENTIONS    P(y | do(x))    "What will happen if we take action x?"
         ^
Level 1: ASSOCIATIONS     P(y | x)        "What does observing symptom x tell us about y?"
```

- **Rung 1 (Association)**: Queries about passive observation, conditional probabilities, and correlational
  patterns in the historical record. This is the exclusive domain of standard curve-fitting, regression, and
  supervised deep learning.
- **Rung 2 (Intervention)**: Queries about active manipulation of the environment, denoted by Pearl's do-calculus
  $P(y \mid \mathrm{do}(x))$. An interventional distribution strips away the confounding arrows entering $X$,
  simulating the physical consequence of setting $X=x$ by external policy.
- **Rung 3 (Counterfactuals)**: Queries about retrospective necessity and moral responsibility: *given that
  outcome $y$ actually occurred after action $x$, what would have happened if we had chosen $x'$ instead?*
  Counterfactuals cannot be observed directly in any prospective trial, because the individual case has already
  traversed history.

A policy decision is invariably a Rung 2 or Rung 3 query. Attempting to answer an interventional question
using Rung 1 associative data without structural identification assumptions is the most pervasive category
error in automated governance.

## 2. Invariant Causal Prediction and structural identification

How does an investigator ascend from passive correlation to interventional invariance without conducting a
prohibitive randomized trial? While classical methods rely on untestable conditional independence assumptions
(backdoor adjustment) or exclusion restrictions (instrumental variables), modern causal inference exploits
heterogeneous data regimes.

Jonas Peters, Peter Bühlmann, and Nicolai Meinshausen (2016) formalized **Invariant Causal Prediction (ICP)**.
Suppose an institution collects observational telemetry across multiple distinct environments $e \in \mathcal{E}$
(differing operational sites, distinct policy regimes, or shifting seasonal conditions, modeled by
`governance.rs::CausalEnvironment`). Under arbitrary external interventions that do not perturb the target
variable's direct structural mechanism, the conditional distribution of target $Y$ given its true direct causal
parents $X_{\mathrm{PA}}$ remains strictly **invariant across all environments**:

$$P(Y \mid X_{\mathrm{PA}}, E = e_1) = P(Y \mid X_{\mathrm{PA}}, E = e_2) \qquad \forall e_1, e_2 \in \mathcal{E} \qquad \text{\texttt{[Assumed]}}.$$

Spurious correlates and downstream symptoms break under environmental shifts: they appear predictive in regime $e_1$
and invert in regime $e_2$. The ICP algorithm tests candidate predictor subsets $S \subseteq X$ for invariance
across environments; the intersection of all statistically invariant subsets identifies the genuine causal parents
(`governance.rs::IcpIdentificationStatus`). If the intersection is empty, the procedure halts, demonstrating that
the candidate variables do not contain the complete causal mechanism.

## 3. Counterfactual responsibility: Tian-Pearl bounds

When a catastrophic failure occurs, legal and institutional accountability requires evaluating counterfactual
attribution: *did the deployment of this automated system cause the harm, or would the harm have occurred anyway?*

Pearl formalizes this through the **Probability of Necessity and Sufficiency (PNS)**: the probability that
taking action $x$ would produce outcome $y$, while withholding action ($x'$) would avert it:

$$\mathrm{PNS} = \mathbb{P}(Y_x = 1, Y_{x'} = 0) \qquad \text{\texttt{[Assumed]}}.$$

Because potential outcomes $Y_x$ and $Y_{x'}$ are never simultaneously observable for the same unit, PNS cannot
be estimated as a point. In 2000, Jin Tian and Judea Pearl proved that by synthesizing observational data
$P(y \mid x)$ with experimental trial data $P(y \mid \mathrm{do}(x))$, PNS can be bounded sharply:

$$\max\bigl(0,\, P(y \mid \mathrm{do}(x)) - P(y \mid \mathrm{do}(x'))\bigr) \le \mathrm{PNS} \le \min\bigl(P(y \mid \mathrm{do}(x)),\, P(y' \mid \mathrm{do}(x'))\bigr) \qquad \text{\texttt{[Assumed]}}.$$

These bounds (`governance.rs::evaluate_tian_pearl_bounds`) define the mathematical boundary of retrospective
attribution: they specify precisely how much blame can be forensically attributed to an intervention.

## 4. Institutional design: the three pillars, Sosa AAA, and Fricker's injustice

Causal inference identifies what an intervention would produce; institutional governance determines who possesses
the standing to decide and who must answer for the outcome. A functioning accountability framework rests upon
**Three Pillars of Institutional Design**:
1. **Observability**: Unfettered, un-sanitized sensory access to operational logs and physical telemetry.
2. **Authority**: Formal, legally binding jurisdiction to intervene, halt operations, or veto unsafe deployments.
3. **Consequence**: Symmetric liability; decision-makers must hold meaningful downside exposure if their policies
   inflict disaster.

Ernest Sosa's **AAA virtue epistemology** evaluates the quality of the decider:
- *Accuracy*: Did the decision produce a successful physical outcome?
- *Adroitness*: Was the choice grounded in rigorous causal inference and valid risk bounds?
- *Aptness*: Did the success occur *because* of the agent's adroitness, rather than through moral luck?

When organizations fail, the failure is rarely cognitive alone; it is social and structural. In *Epistemic
Injustice*, Miranda Fricker conceptualized **testimonial injustice**: the structural discounting of a speaker's
credibility caused by institutional prejudice regarding their status or identity. Frontline operators, junior
engineers, and marginalized communities frequently detect physical anomalies and early warning signals long
before they appear on executive dashboards. When institutional hierarchies impose a credibility discount
($\delta_{\mathrm{cred}} > 0$), the warning signal is silenced, truncating the organization's sensory capacity.
Stafford Beer's cybernetic **algedonic channel** exists precisely to bypass this hierarchical filtering,
transmitting raw pain signals directly to the executive veto holder.

## 5. Chouldechova's algorithmic fairness impossibility theorem

When automated decision algorithms are deployed across demographically diverse populations, institutions
frequently demand that the model be simultaneously "unbiased" and "fair". In 2017, Alexandra Chouldechova proved
that this demand is mathematically impossible.

Let $Y \in \{0, 1\}$ be the true outcome, $S \in [0, 1]$ the predicted risk score, and $A \in \{0, 1\}$ a protected
demographic attribute. Suppose the groups exhibit unequal baseline prevalence rates:

$$\mathbb{P}(Y = 1 \mid A = 1) \neq \mathbb{P}(Y = 1 \mid A = 0) \qquad \text{\texttt{[Assumed]}}.$$

Chouldechova proved that no risk score $S$ can simultaneously satisfy:
1. **Within-group calibration**: $\mathbb{P}(Y = 1 \mid S = s, A = 1) = \mathbb{P}(Y = 1 \mid S = s, A = 0) = s$.
2. **False Positive Rate parity**: $\mathbb{P}(S > s^* \mid Y = 0, A = 1) = \mathbb{P}(S > s^* \mid Y = 0, A = 0)$.
3. **False Negative Rate parity**: $\mathbb{P}(S \le s^* \mid Y = 1, A = 1) = \mathbb{P}(S \le s^* \mid Y = 1, A = 0)$,

unless the prediction rule achieves perfect deterministic accuracy ($AUC = 1.0$). If an institution enforces
calibration across groups with differing base rates, it is mathematically forced to generate unequal false positive
rates. Causal fairness cannot be engineered by tuning an algorithm; it requires restructuring the institutional
loss matrix that assigns consequences to errors.

## 6. Why an attribution score is not an intervention

The most widely deployed explanations in applied machine learning — Shapley-value attributions, local surrogate
models, partial-dependence plots — share a structure, and the structure determines exactly what they can and cannot
support. Each of them decomposes the model's output into contributions from the model's own inputs, holding the
*model* fixed `[Literature]`. That is a statement about the fitted function on the archive, and the archive is the
second plane, not the first.

Three consequences follow from the ladder of §1, and they are not philosophical quibbles.

1. **An attribution is a rung-one quantity.** It answers "how does the fitted function respond when this covariate
   moves, with everything else in the model unchanged?" It does not answer "what happens to the outcome if we
   intervene on this covariate in the world?", because an intervention breaks the incoming edges to the variable
   rather than re-weighting a term in a regression. Those two questions have different answers whenever the
   covariates are correlated — the standard case.
2. **The decomposition is conditional on the model being right where the explanation is read.** Attributions are
   computed at points the analyst chooses, which are usually the points the model already scores. A local surrogate
   fitted in a region where the structural discrepancy $\delta(x)$ is large explains the *discrepancy*, faithfully
   and persuasively.
3. **Feature importance inherits the encoding.** Correlated features split their attribution arbitrarily, and
   monotone transforms change the units without changing the ranking, so a top-five list is not a causal ordering
   even when it is stable across seeds.

What restores a causal reading is the machinery of §2 and §3, and it is worth stating what it costs. The
Tian–Pearl bounds of `governance.rs::GovernanceState::evaluate_tian_pearl_bounds` return *intervals* rather than
point attributions, because counterfactual probabilities are not identified by observational data alone; the ICP
machinery of §2 returns a *set* of predictors whose conditional relationship survives across environments, which is
a claim about invariance rather than about magnitude. Both are weaker in presentation and stronger in warrant, and
that asymmetry is the whole point: an interval that is guaranteed to contain the intervention effect is usable for
a decision, while a point attribution that is not identified for one is a rhetorical object.

The institutional form of the rule is therefore simple to state and hard to follow. An explanation may be reported
to the person affected by a decision as an account of *how the model reasoned*, which is a claim on the second
plane. It may be reported as an account of *what would change the outcome*, which is a claim on the first plane,
only when the identification argument of §2–§3 has been made and its interval is attached. Collapse Proposition 3
is what happens when the first is dressed as the second: the appearance of causal transparency becomes the alibi
that forecloses the audit.

## Bridge

Causal environments and invariant prediction are formalised in
[Chapter 48](48-reactive-worlds-performativity-goodhart.md) as `governance.rs::CausalEnvironment` and
`governance.rs::IcpIdentificationStatus`; Tian-Pearl counterfactual bounds on causation are evaluated by
`governance.rs::evaluate_tian_pearl_bounds`. The institutional gates auditing testimonial discounting,
algedonic channels, and algorithmic disparity are formalised in
[Chapter 49](49-moral-luck-apt-agency-institutions.md).
