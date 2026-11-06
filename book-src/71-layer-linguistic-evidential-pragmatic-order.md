# Chapter 71 — Layer 21: Linguistic, Grammatical Evidentiality & Organizational Pragmatics Order

> Layer 21 (`multilayer.rs::KnowledgeOrder`, `KnowledgeOrder::L21LinguisticEvidentialPragmatic`) of Part V · Stratum V (`multilayer.rs::KnowledgeStratum`, `KnowledgeStratum::S5SemioticArchival`) · Unique Guard Combination `{T3Archive, T6Performative, T7Liability}` · Signature `(1, 0, 2)` (`multilayer.rs::TriadicExposureRegime`, `TriadicExposureRegime::VitalBlindSpot`). It builds on [Layer 02](52-layer-meso-buffered-queue-dispatch-order.md), [Layer 06](56-layer-formal-computable-type-order.md), [Layer 14](64-layer-taxonomy-systematics-ontology-order.md) in $\mathcal{G}_1$ and maps into $\mathcal{G}_2$ via [`notes/The Multilayer Graph of Knowledge and Aim.md`](../notes/The%20Multilayer%20Graph%20of%20Knowledge%20and%20Aim.md).

## By the end of this chapter you can

1. Contrast the obligatory morphological evidentiality of Amazonian and Andean languages with the bare
   assertions of Indo-European bureaucratic prose, and link the difference to March and Simon's
   uncertainty absorption.
2. Separate assertive speech acts (*word-to-world* fit) from declarative and directive speech acts
   (*world-to-word* fit), and explain why confusing the two generates performative distortion.
3. Map Habermas's three validity claims and Austin's felicity conditions onto the Nuclear Triad and the
   quarantine verdicts of `grammar.rs::judge`.

## 1. Morphological evidentiality versus uncertainty absorption

In 1958 James March and Herbert Simon identified a quiet mechanism by which hierarchies manufacture
delusion: **uncertainty absorption** `[Literature]`. An analyst at the edge observes a messy, conditional
set of facts; the summary sent one floor up drops the caveats; the executive brief two floors up quotes
only the point conclusion. Indo-European languages make this effortless because a declarative sentence
such as *"demand is twelve units"* is grammatically complete with zero marking of how that claim was
obtained.

Comparative linguistics shows that human grammar does not have to work this way. In Eastern Tukanoan and
Arawakan languages of the Vaupés basin in Amazonia (such as **Tuyuca** and **Tariana**) and in Andean
languages (such as **Quechua**), analysed by Alexandra Aikhenvald (2004) `[Literature]`, **evidentiality
is an obligatory inflectional category on every declarative verb**. In Tuyuca, a speaker cannot utter a
bare verb for *"the boy played"*; the verb stem must take one of five suffixes declaring the information
source `[Literature]`:

- `-wi` (visual direct observation $\leftrightarrow$ direct **`[Measured]`** reading);
- `-ti` (non-visual sensory perception);
- `-yi` (inference from physical traces left in the world $\leftrightarrow$ Hippocratic *sēmeîon* and
  archived **`[Measured]`** trace);
- `-hĩyi` (logical deduction or assumed premise $\leftrightarrow$ **`[Simulated]`** / **`[Assumed]`**);
- `-yigi` (second-hand report or hearsay $\leftrightarrow$ **`[Literature]`**).

Crucially, omitting the evidential suffix in Tuyuca or Tariana does not produce a vague sentence; it
produces an **ungrammatical string (`Unparsable`)**. The provenance discipline of this repository and the
`Counting` production of `grammar.rs::parse_ebnf_grammar` (`count [L, U] deff vintages`) restore by
explicit formal syntax what high-discipline natural languages enforce in verb morphology.

## 2. Direction of fit and speech-act felicity

In the philosophy of language developed by J. L. Austin (1962) and John Searle (1969, 1979) `[Literature]`,
every utterance performs an **illocutionary act** governed by a **direction of fit**:

- **Assertives** (reports, measurements, readouts) have a *word-to-world* direction of fit ($\downarrow$):
  their task is to match an independently existing state of the world (`grammar.rs::AgentActionKind`
  `Read`).
- **Directives and Declaratives** (orders, resource allocations, published rankings, legal verdicts) have
  a *world-to-word* direction of fit ($\uparrow$): their task is to alter the world so that it conforms
  to the utterance (`grammar.rs::AgentActionKind` `Write`).

Performative deception ([Chapter 48](48-reactive-worlds-performativity-goodhart.md)) occurs when an
institution issues a declarative `Write` — a credit score, a school ranking, a policing heat map — that
actively reorganises human behaviour, and then evaluates the resulting numbers as if they were a passive
assertive `Read` of an untouched world.

Furthermore, Austin showed that an illocutionary act succeeds not by being "true or false" in the
abstract, but by satisfying **felicity conditions** (*preparatory*, *executive*, and *sincerity* rules),
whose violation renders the act **null and void (*misfire*)**. In `grammar.rs::judge`, the quarantine
verdicts `OutOfScope` (acting without standing), `UnauthorizedRead`, `MissingControlTape`, and
`SkeletonMismatch` are formal misfires: an unguarded command is not scored as a bad shot; it is refused
standing as a shot at all.

## 3. Communicative validity and the Nuclear Triad

Jürgen Habermas's theory of communicative action (1981) `[Literature]` grounds rational coordination in
three irreducible validity claims raised by every serious speech act, and their correspondence to the
Nuclear Triad is exact:

1. **Propositional truth** (*Wahrheit* — does the statement respect the evidence and its boundaries?)
   $\Longleftrightarrow$ Epistemic Margin $S_{\mathrm{epi}}$;
2. **Expressive truthfulness and existential grounding** (*Wahrhaftigkeit* — does the speaker declare
   their real premises and bear the physical tension of the act?) $\Longleftrightarrow$ Vital Margin
   $S_{\mathrm{vital}}$ and *télos* attestation;
3. **Normative rightness** (*Richtigkeit* — is the act legitimate and contestable by those bound by it?)
   $\Longleftrightarrow$ Institutional Governance $S_{\mathrm{gov}}$.

## 4. The three collapses in the linguistic and pragmatic order

1. **Collapse Proposition 1 ($\neg S_{\mathrm{epi}}$ — Evidential Stripping):** Dropping provenance
   markers and identification intervals as a claim travels up the reporting chain (uncertainty
   absorption).
2. **Collapse Proposition 2 ($\neg S_{\mathrm{vital}}$ — Channel Congestion and Semantic Overload):**
   Violating Grice's maxim of quantity `[Literature]` during an acute crisis by burying a 30-bit *Kairós*
   alarm inside a 240-bit bureaucratic report (`governance.rs::ZhengMeisterQueueAudit`).
3. **Collapse Proposition 3 ($\neg S_{\mathrm{gov}}$ — Strategic Perlocution and Testimonial Silencing):**
   Disguising a coercive target as a neutral description while discounting frontline speech acts
   ($\delta_{\mathrm{cred}} \to 1$).

## Bridge

Layer 21 (`multilayer.rs::KnowledgeOrder`, `KnowledgeOrder::L21LinguisticEvidentialPragmatic`, `{T3Archive, T6Performative, T7Liability}`) propagates its local and ancestor-cone margins through `multilayer.rs::MultilayerHomomorphism::evaluate` into [Chapter 72 — Layer 22: Philological, Epigraphic & Stemmatic Textual-Criticism Order](72-layer-philological-epigraphic-textual-order.md) (`KnowledgeOrder::L22PhilologicalEpigraphicTextual`, Stratum V, `{T2Severity, T3Archive, T4Formalization}`).
