# Justification and Induction

> Second chapter of the philosophical block in Part II. The subject is the oldest problem in the
> theory of evidence: on what basis does the past constrain the future?

## By the end of this chapter you can

1. State the problem of induction and say precisely which premise it denies.
2. Distinguish deductive from inductive support, and say why the second cannot be made deductive.
3. Explain what a probability model does and does not add to an inductive inference.

## 1. Deduction and the Limits of Formal Truth Transfer

A deductive argument is structurally truth-preserving: if the premises are true, the conclusion cannot be false.
Modus ponens ($P \implies Q$, $P$, therefore $Q$) and mathematical proofs operate within this domain.

However, deduction transfers truth without generating empirical contact. A valid syllogism makes explicit what
was already contained within the premises; it cannot verify whether those premises correspond to the physical
world. Empirical inquiry must establish its premises through observation, measurement, and interaction — and
that step is non-deductive.

## 2. Hume's Problem and Goodman's New Riddle of Induction

David Hume (1739/1748) formulated the classical **problem of induction**: inductive generalizations of the form
*all observed instances of $A$ have been $B$, therefore the next instance of $A$ will be $B$* possess no deductive
guarantee. The denial of the conclusion is never a logical contradiction: "the sun will not rise tomorrow" violates
no rule of logic.

Nor can induction be justified empirically by citing its historical success. To argue that induction is reliable
because it has worked in the past is to use an inductive argument to justify induction — a fatal petitio principii
(vicious circularity).

Nelson Goodman (1955) deepened this skepticism with the **new riddle of induction**. Define the predicate *grue*:
an object is grue if it is examined before future date $T$ and found to be green, or not examined before $T$ and
found to be blue. All emeralds examined up to today (time $t < T$) are green; consequently, all examined emeralds
are also grue. If past instances confirm generalizations, the evidence confirms both *all emeralds are green* and
*all emeralds are grue*. Yet at $T$, they yield diametrically opposed predictions: the former predicts the next
emerald will be green, while the latter predicts it will be blue.

Goodman proved that inductive inference is impossible without **projectible predicates**. We prefer *green* over
*grue* not because logic dictates it, but because *green* is culturally and scientifically entrenched. When
institutions build statistical models, their choice of features implicitly embeds entrenched predicates, determining
which futures the model can even conceive.

## 3. Peirce's Inferential Triad: Abduction, Deduction, Induction

Charles Sanders Peirce dismantled the binary opposition between deduction and induction by introducing a
tripartite architecture of scientific reasoning:

1. **Abduction** (Hypothesis formation): Confronted with an anomalous or surprising observation $C$, the mind
   conjectures: *if hypothesis $A$ were true, $C$ would follow as a matter of course; hence, there is reason to suspect
   that $A$ is true*. Abduction is the only logical operation that introduces genuinely novel explanatory ideas.
2. **Deduction** (Consequence tracing): Traces the necessary, observable consequences of hypothesis $A$, deriving
   specific empirical predictions: *if $A$ holds, then under experimental conditions $E$, outcome $O$ must occur*.
3. **Induction** (Severe testing): Submits those deduced consequences to severe empirical confrontation, evaluating
   whether the hypothesis survives genuine opportunities for refutation.

Without abduction, induction is blind data mining; without deduction, abduction is ungrounded speculation; without
induction, deduction is an unverified tautology.

## 4. Agnotology and the Manufacture of Strategic Doubt

In the social and political sphere, inductive uncertainty is frequently weaponized. Robert Proctor and Londa
Schiebinger (2008) founded **agnotology** — the study of culturally and strategically produced ignorance.

The archetypal expression of manufactured doubt is the **tobacco strategy**, encapsulated in an infamous 1969
tobacco industry internal memorandum: *"Doubt is our product, since it is the best means of competing with the
'body of fact' that exists in the minds of the general public."*

When an industry or institution faces empirical evidence demonstrating that its profitable operations impose
catastrophic health, ecological, or systemic risks, it rarely denies the science outright. Instead, it exploits
Humean fallibilism:

- It demands impossible, infinite standards of proof ("the causal mechanism is not 100% resolved");
- It funds redundant studies to manufacture false controversies and statistical noise;
- It uses inductive skepticism as an agnotological alibi to delay regulation while extracting private upside and
  shifting irreversible downside onto the public.

Oliver Cromwell's rule warns against assigning prior probability zero or one to hypotheses ($\mathbb{P}(H) \in (0, 1)$).
In institutional governance, agnotology exploits this openness, claiming that because uncertainty is never zero,
precautionary action is never justified. The ethical response is to recognize that when vital survival is at
stake, action cannot wait for deductive certainty.

## 5. Consequences for Epistemic Margin Sizing

A rigorous institution handles induction not by pretending to possess proof, but by sizing an explicit
**epistemic margin**:

1. It acknowledges the structural underdetermination of models by finite archives;
2. It audits the framing of questions to prevent agnotological suppression of inconvenient hypotheses;
3. It sizes physical vital reserves against the widest plausible inductive bound, refusing to let manufactured
   doubt paralyze precautionary defense.

## Bridge

[The Question Before the Data](32-the-question-before-the-data.md) formalizes inquiry design, while
[Inquiry Openness, Severity, and the Multiverse](43-inquiry-openness-multiverse.md) constructs mathematical tests
for agnotological framing bias and Cromwell violations. In the Stochology library,
`epistemic.rs::from_inquiry_framing` computes the epistemic margin by evaluating framing penalties, Cromwell
boundary enforcement, and Type III inquiry distortions across competing hypotheses.

[Philosophy of Science](18-philosophy-of-science.md) turns justification into a public test:
falsification, Duhem–Quine underdetermination, Lakatosian research programmes, and Mayo's severe
testing.
