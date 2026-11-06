# Philosophy of Science

> Third chapter of the philosophical block in Part II. The subject is the criterion by which a claim is
> admitted to a body of knowledge, and the reasons every such criterion is contested.

## By the end of this chapter you can

1. State the falsifiability criterion and the objection that a single hypothesis is never tested alone.
2. Distinguish a progressive from a degenerating research programme, with the observable difference.
3. Say why the demarcation question matters for institutions and not only for philosophers.

## 1. Falsificationism and the Demarcation Criterion

Karl Popper (1934/1959) sought to solve the problem of induction by reformulating scientific rationality:
science advances not by inductive verification, but by deductive falsification. Because no finite set of
favorable observations can logically prove a universal generalization, whereas a single genuine counterexample
can deductively refute it, the hallmark of empirical science is **falsifiability**.

A proposition is scientific if and only if it divides the set of all conceivable observational statements into
two non-empty classes:

1. Those with which it is compatible;
2. Those which it strictly **forbids** (the potential falsifiers).

A theory that forbids nothing — that possesses an explanation for every conceivable outcome after the fact — is
an unfalsifiable dogma, metaphor, or sales pitch. For Popper, the epistemic strength of a hypothesis is measured
by the severity of the attempted refutations it has successfully survived.

## 2. The Duhem–Quine Thesis and Holistic Underdetermination

The structural flaw in naive falsificationism was identified by Pierre Duhem (1906) and Willard Van Orman Quine (1951).
A theoretical hypothesis $H$ is never confronted with empirical observation in total isolation. An empirical prediction
$O$ is deduced from an extensive web of conjunctions:

$$H \land A_1 \land A_2 \land \cdots \land A_k \implies O,$$

where $A_1, \dots, A_k$ are auxiliary hypotheses, including the physical theories governing measurement instruments,
calibration constants, software pipelines, data preprocessing heuristics, and experimental execution integrity.

When the predicted observation fails ($O$ does not occur, $\neg O$), modus tollens entails only that the conjunction
is false:

$$\neg (H \land A_1 \land \cdots \land A_k) \iff \neg H \lor \neg A_1 \lor \cdots \lor \neg A_k.$$

Logically, the failure does not localize the blame. The primary hypothesis $H$ can always be protected by attributing
the failure to an auxiliary assumption: the sensor was miscalibrated, the dataset was contaminated, or the sample
was unrepresentative. In corporate and governmental institutions, the Duhem–Quine leeway becomes an administrative
shield: failing models are perpetually excused by blaming external data anomalies rather than flawed core assumptions.

## 3. Lakatosian Research Programmes: Progressive vs. Degenerating

Imre Lakatos (1970) resolved the Duhem–Quine tension by replacing isolated theories with **research programmes**.
A research programme consists of:

- A **hard core** of foundational commitments, protected against refutation by a methodological negative heuristic;
- A **protective belt** of auxiliary hypotheses that absorb anomalies and undergo continuous adjustment.

Lakatos established an objective, observable demarcation between two evolutionary regimes:

A research programme is **theoretically and empirically progressive** if its auxiliary adjustments lead to the
prediction of novel, hitherto unexpected empirical facts, at least some of which are subsequently corroborated.
The modification occurs *before* the new observation is collected.

A research programme is **degenerating** (or pseudoscientific) if its protective belt expands solely to accommodate
known failures ex post, retrofitting ad-hoc parameters without generating novel testable predictions. The hallmark
of degeneration is temporal: adjustments are manufactured *after* an inconvenient failure to rescue institutional
prestige.

## 4. Deborah Mayo's Error-Statistical Severe Testing

Deborah Mayo (1996/2018) operationalized scientific warrant through the **severity principle**:

> Data $\mathbf{x}$ provide severe evidence for hypothesis $H$ if and only if:
> 1. $\mathbf{x}$ accords with $H$ (good fit); and
> 2. With high probability, the test procedure would have produced a result discordant with $H$, had $H$ been false.

Severity demands that a model pass a test that had a genuine capacity to break it. Testing a model exclusively
against data on which it was trained, or adjusting parameters until an in-sample fit is achieved, has a severity
of zero: such a procedure would find a fit regardless of whether the model was true or false. Honest science
requires out-of-sample adversarial stress tests where failure is a live, observable possibility.

## 5. Epistemic Injustice and Testimonial Silencing

Science and institutional decision-making are human social practices, vulnerable to systemic distortions.
Miranda Fricker (2007) identified two fundamental modalities of **epistemic injustice**:

- **Testimonial injustice**: An observer's testimony receives a credibility deficit owing to prejudice against
  their social identity, status, or organizational position. In engineering disasters (such as the Space Shuttle
  *Challenger* and *Columbia* failures), frontline technicians accurately warned of O-ring embrittlement and foam
  impact damage, but their warnings were dismissed by management as "unscientific" because they lacked formal
  aerospace degree prestige.
- **Hermeneutical injustice**: A community lacks the conceptual, linguistic, or statistical resources to make
  a collective harm intelligible, preventing victims from rendering their experience communicative.

When an institution silences frontline warnings, it does not merely commit an ethical violation; it blinds its
own sensors. The frontline operator is the sensor located closest to physical friction and vital reserve depletion.
Discounting their testimony causes the institution to ignore early warning signs of catastrophic failure.

## Bridge

[Underdetermination, Partial Identification, and Severe Testing](44-underdetermination-partial-identification.md)
computes severity thresholds and bounds on empirical datasets. In the Stochology library,
`epistemic.rs::EpistemicMarginState` evaluates severe boundary clearances via `epistemic.rs::passes_severe_boundary`,
while `governance.rs::GovernanceState` implements institutional audit trails that protect frontline testimonial
evidence against managerial discounting.

[Ethics and the Imposition of Risk](19-ethics-and-the-imposition-of-risk.md) asks who bears the cost
when the test is failed: the aggregation fallacy, non-reciprocal risk imposition, Scanlonian
contractualism, and the residue of moral luck.
