# The Question Before the Data

> Foundation chapter of Part III. Its formal content is worked out in
> [Chapter 43](43-inquiry-openness-multiverse.md).

## By the end of this chapter you can

1. State, for any inquiry, who commissioned it, who pays for the error, and which answer was
   already preferred.
2. Detect the four moves that narrow a question before any data is collected: scope, metric,
   comparison, and silence.
3. Write a pre-registration that would survive contact with a disappointing result.

## 1. Every question arrives framed

An empirical inquiry is never an unmediated aperture opened onto nature. It arrives carrying an institutional
sponsor, an allocated budget, an operational deadline, and a pre-existing preference regarding which findings
would be convenient or disruptive. None of this institutional scaffolding is hidden, yet none of it appears in
the rows and columns of the resulting database.

The observational instrument is therefore vastly wider than the dataset itself. Who funded the study determines
which populations were sampled and which were excluded; the tenure and promotion incentives of the investigators
determine which hypotheses are publishable; the legal liability of the commissioning institution determines which
variables are considered actionable and which are suppressed. A genuine audit of evidence must interrogate these
upstream commitments, not because the analysts are presumed corrupt, but because institutional framing silently
decides the boundaries of the answer before the first observation is logged.

One diagnostic rule must be enforced across all scientific and operational evaluation: **an empirical inquiry
that cannot fail is not an inquiry; it is institutional theatre.** If no conceivable pattern of observations
would compel the commissioning body to abandon its preferred course of action, the data collection is purely
ornamental. In corporate and governmental practice, the overwhelming majority of internal program reviews are
engineered to be unfalsifiable from their inception.

## 2. Four narrowing moves and Lakatos problem shifts

The framing of an inquiry is narrowed long before data collection commences. These moves are structural and
admit an exhaustive classification into four recurring maneuvers:

**Scope.** The target population is surreptitiously replaced by an accessible, compliant subpopulation.
"Does this algorithmic intervention improve student learning?" becomes "does this software improve quiz
scores among students who complete all modules on high-speed home internet?" — an entirely different question
whose answer is predetermined to look favorable.

**Metric.** A complex, multidimensional state is replaced by an easily quantifiable proxy that happens to
correlate with it in benign conditions. Diagnostic acuity is replaced by patient throughput; deep comprehension
is replaced by standardized test scores; social stability is replaced by arrest tallies. Once established, the
proxy metric is reified and never questioned.

**Comparison.** The counterfactual baseline is selected post hoc to flatter the favored program. Comparing
performance against the worst historical quarter, against an obsolete technology, or against total inaction
manufactures the illusion of massive success, while comparison against an inexpensive alternative is silenced.

**Silence.** The inquiry's ontological boundary is drawn so that unrecorded harms cannot be articulated. If a
particular ecological damage, psychological toll, or community displacement lacks an assigned column in the
relational database, the resulting cost-benefit analysis is internally consistent, mathematically optimal, and
grotesquely false.

In the philosophy of science, Imre Lakatos distinguished **progressive** from **degenerating research programmes**.
A progressive programme anticipates novel, unexpected facts, expanding its explanatory core. A degenerating
programme, by contrast, responds to anomalies by constructing an endless belt of protective ad hoc auxiliary
hypotheses, narrowing its empirical scope to explain away failures. The four narrowing moves above represent the
institutional technology of degenerating programmes: they protect the organization's favored premise from the
shock of reality.

## 3. Severe testing and the Kennedy-O'Hagan discrepancy model

To rescue empirical inquiry from degenerating framing, Deborah Mayo formulated the **Severity Principle**:
a scientific claim $H$ passes a severe test with data $x_0$ if and only if:
1. $x_0$ accords with the predictions of $H$, and
2. There was a very high probability that the testing procedure would have yielded a result *less* in accord
   with $H$ if $H$ were false in a specified, substantively important way.

A test with low statistical power or an analysis that permits endless post-hoc parameter adjustments cannot
subject a hypothesis to severe testing. Passing an un-severe test provides zero epistemic warrant for action.

When mathematical simulators or predictive models are calibrated against real-world observations, Marc Kennedy
and Anthony O'Hagan (2001) demonstrated that the observed reality $y(x)$ must be modeled as the sum of the
computer code $\eta(x, \theta)$, an explicit **model discrepancy** term $\delta(x)$, and observation error $\epsilon$:

$$y(x) = \eta(x, \theta) + \delta(x) + \epsilon \qquad \text{\texttt{[Assumed]}}.$$

If the analyst ignores $\delta(x)$ and attempts to force the model into agreement with the data solely by adjusting
the calibration parameters $\theta$, the parameter estimates become hopelessly confounded. The model "over-calibrates",
absorbing structural model deficiencies into warped physical parameters that fail catastrophically when extrapolated
outside the training regime.

## 4. Pre-registration, Cromwell's rule, and the multiverse

Pre-registration exists to constrain the analyst against their own post-hoc rationalizations. A valid pre-registration
binds the sample collection protocol, the cleaning transformations, the model specification, and — decisively —
what empirical finding would constitute unambiguous refutation. Done as a hollow ritual, pre-registration lists
the model family but leaves the threshold of failure unstated, preserving the very flexibility that permits
p-hacking.

Furthermore, an honest inquiry must respect **Cromwell's rule**: never assign an absolute prior probability of
zero or one to any contingent hypothesis ($\mathbb{P}(H) \in (0, 1)$). Setting $\mathbb{P}(H) = 1$ permanently
blinds Bayes' theorem, ensuring that no empirical anomaly, no matter how severe, can ever dislodge the belief.

Where multiple defensible analytical choices exist (differing covariate adjustments, outlier cutoffs, or functional
forms), the proper response is **multiverse analysis** and specification curve analysis. Rather than selecting
the single specification that produces a desired p-value, the analyst runs all $M$ defensible specifications.
To control the False Discovery Rate (FDR) across arbitrary, highly dependent model specifications without making
unverifiable independence assumptions, the **$e$-Benjamini-Hochberg ($e$-BH)** procedure ranks the resulting
e-values, guaranteeing rigorous error control across the entire analytical multiverse.

## 5. Whose threshold?

The final element fixed by an inquiry's framing is the decision threshold for action:

$$p^* = \frac{C}{\eta L} \qquad \text{\texttt{[Assumed]}},$$

where $C$ is the cost of intervention, $\eta$ is its effectiveness, and $L$ is the loss incurred if the catastrophe
occurs.

Critically, the loss $L$ is not a single objective number. For an administrative agency or corporation,
$L_{\mathrm{inst}}$ represents its own private exposure: regulatory fines, litigation reserves, or stock depreciation.
For the vulnerable community or patient, $L_{\mathrm{affected}}$ represents irreparable physical, health, or livelihood
devastation. Because $L_{\mathrm{inst}} \ll L_{\mathrm{affected}}$, the institution's action threshold $p^*_{\mathrm{inst}}$
sits far higher than the affected community's threshold $p^*_{\mathrm{affected}}$:

$$p^*_{\mathrm{inst}} = \frac{C}{\eta L_{\mathrm{inst}}} \gg \frac{C}{\eta L_{\mathrm{affected}}} = p^*_{\mathrm{affected}} \qquad \text{\texttt{[Assumed]}}.$$

The resulting conflict is not a scientific dispute over statistical evidence. Both parties can agree precisely on
the estimated probability of harm $\hat{p}$. The community demands immediate preventive intervention because
$\hat{p} > p^*_{\mathrm{affected}}$, while the institution counsels "further study" because $\hat{p} < p^*_{\mathrm{inst}}$.
This is the political anatomy of Kimball's Type III error: answering the institution's cost-minimization question
while pretending to solve the public's survival question.

## Bridge

Framing, Cromwell openness, and the cost-loss threshold gap are formalised in
[Chapter 43](43-inquiry-openness-multiverse.md) as `epistemic.rs::from_inquiry_framing`, which carries both
institutional and affected cost-loss thresholds alongside the expected value of perfect information; the
entire universe of defensible model specifications evaluated under false discovery rate guarantees is
`epistemic.rs::from_multiverse`.
