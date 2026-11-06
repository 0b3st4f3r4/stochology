# Chapter 56 — Layer 06: Formal Logic, Computability & Linear Resource-Type Order

> Layer 06 (`multilayer.rs::KnowledgeOrder`, `KnowledgeOrder::L06FormalComputableLinearType`) of Part V · Stratum II (`multilayer.rs::KnowledgeStratum`, `KnowledgeStratum::S2FormalMetrological`) · Unique Guard Combination `{T1Framing, T4Formalization, T5Temporal}` · Signature `(1, 2, 0)` (`multilayer.rs::TriadicExposureRegime`, `TriadicExposureRegime::GovernanceBlindSpot`). It builds on [Layer 01](51-layer-micro-kinetic-sensorimotor-order.md) in $\mathcal{G}_1$ and maps into $\mathcal{G}_2$ via [`notes/The Multilayer Graph of Knowledge and Aim.md`](../notes/The%20Multilayer%20Graph%20of%20Knowledge%20and%20Aim.md).

## By the end of this chapter you can

1. Explain why Rice's theorem forbids a universal runtime verifier of arbitrary agent semantics, and why
   a syntactic certificate on the control tape is the decidable alternative.
2. State what linear and affine type systems forbid that classical logic allows, and why physical
   reserves and sealed records obey linear rather than classical rules.
3. Map the three collapse propositions onto defects of a formal specification: silent domain narrowing,
   unbounded search depth, and unchecked authority scope.

## 1. Why the audit must be syntactic

A recurring temptation in safety engineering is to treat an autonomous or institutional policy as a
black box and ask a monitor to verify, before the policy acts, that its future execution will never
violate a safety invariant. Computability theory settles that hope with a theorem rather than an
opinion.

By **Rice's theorem** (1953) `[Literature]`, every non-trivial semantic property of the partial
functions computed by an arbitrary program is undecidable. No general procedure can inspect an
unconstrained policy and decide, in finite time, whether it will eventually exhaust a reserve or game a
metric on an unseen input. Waiting for a semantic proof inside a loop with a finite survival horizon
($\mathrm{TTS}_{\mathrm{survival}} < \infty$) is a contradiction in terms: the verifier itself violates
the survival clock before returning an answer.

The only decidable escape is the one formalised in `grammar.rs::judge` and `grammar.rs::parse_ebnf_grammar`:
replace semantic omniscience over the future world (*skopós*) with a **linear-time $LL(1)$ syntactic
certificate** over the agent's declared premises (*télos*). A sentence is well-formed only if its framing
mode, interval bounds $[L_t, U_t]$ with $L_t \le U_t$, effective sample size adjustments, lexicographic
decision rule, and survival brake $\mathrm{TTS}_{\mathrm{survival}} \ge \mathrm{TTR}_{\mathrm{recovery}}$
appear explicitly on the control tape before any write to the world is licensed.

## 2. Linear logic and affine resources

Classical propositional logic permits two structural rules without cost: **contraction** ($A \vdash A \wedge A$,
a premise may be duplicated and used twice) and **weakening** ($A \wedge B \vdash A$, a premise may be
silently discarded). Both rules are true of mathematical propositions and false of physical reserves.

In Jean-Yves Girard's **linear logic** (1987) `[Literature]` — and in the **affine type system** and
ownership semantics of Rust — a resource $A$ cannot be duplicated ($A \nvdash A \otimes A$), and in a
strict linear discipline it cannot be dropped without an explicit consumer. Read through the three axes
of this book, the correspondence is exact:

- A **vital reserve** $R_t$ is a linear resource: spending a unit of stock or staff capacity in one
  commitment makes that exact unit unavailable to every other commitment (`engine.rs::CommunityReserveEnv`).
  Treating a shared buffer as if it satisfied contraction is the formal error behind double-pledged
  collateral and overbooked surge capacity.
- A **pre-outcome seal** (`governance.rs::TemporalVault`) is a single-opening typestate machine: moving
  the sealed commitment into the post-outcome verification step consumes the seal so that the hypothesis
  cannot be reopened and re-fitted to the realised outcome.
- An **algedonic warning** (`governance.rs::AlgedonicChannel`) forbids unrestricted weakening: a distress
  signal from the periphery is an obligation that must be explicitly handled, never silently dropped by
  an intermediate signature.

## 3. Bounded enumeration and the coverage ledger

Even when each individual trajectory is checked in linear time, searching the space of possible guard
configurations across $7$ sites with $g$ grid levels requires $g^7$ evaluations `[Assumed]`. At $g = 3$
($3^7 = 2{,}187$) or $g = 5$ ($5^7 = 78{,}125$) the search is exhaustive; as $g$ grows, an unbounded
search would stall the deliberation loop and induce Collapse Proposition 2 through verification latency
alone.

Hence `grammar.rs::enumerate_with_coverage` and `grammar.rs::shortest_violation_with_coverage` attach a
mandatory **coverage ledger** (`grammar.rs::Coverage`) to every generative search: when the
requested grid exceeds the pre-registered evaluation cap, the search halts inside the survival budget
and flags `truncated = true` explicitly rather than pretending that an incomplete search is a proof of
safety.

## 4. The counterexample: Apollo 13 and the re-derived envelope

Ariane 5 Flight 501 is the canonical instance of the first collapse proposition of §6, and it is worth pairing
with its exact opposite, because the pair separates two things that look identical in an incident report and are
opposites in a control room: *reusing a verified artifact outside its domain* versus *re-deriving the
contract when the domain moves*.

On 13 April 1970 the Apollo 13 service module's oxygen tank exploded roughly 330,000 km from Earth. The
command-and-service module lost its primary power and life-support consumables; the lunar module became the
lifeboat; and the vehicle that remained was outside every design envelope its procedures had been written
for — wrong mass distribution, wrong thermal budget, wrong available power, a carbon-dioxide removal problem
with no adapter, and a return trajectory that had to be rebuilt from free-return to a powered manoeuvre
`[Literature]`. No procedure on the shelf applied, and the software on board was not going to be rewritten.

What the ground did is the interesting part, and it is a formal move rather than a heroic one. The mission
control team ran the vehicle's *simulators* — fifteen of them, reconfigured in parallel under telemetry from
the actual spacecraft — to test candidate procedures against the measured state before transmitting any of
them to the crew `[Literature]`. In the vocabulary of this chapter: the flight envelope is a type, the
procedures are terms typed against it, and after the explosion the type had silently narrowed to a range no
existing term inhabited. The institution's response was not to assume the old type still held (Ariane 5) but
to *re-type the vehicle from telemetry* and re-elaborate the terms against the new type, in a simulator, off
the critical path, before any write reached the physical system.

Three properties of that response are the ones this layer's audit should require, and each has a machine
analogue:

1. **The domain break was detected immediately and treated as the primary fact.** The telemetry did not fit
   the nominal model; instead of adjusting the residual, the team declared the model invalid. That is the
   difference between a bound and a point estimate, and it is what `grammar.rs::judge` enforces when it
   refuses a command whose declared premises are outside their support rather than extrapolating them.
2. **The re-derivation happened in a copy, not in the world.** The simulators are the sandbox of §3: bounded
   enumeration over candidate procedures, at a cost that fits inside the survival horizon. This is also why
   the choice of *what to enumerate* is a design decision — the search had to be bounded by the consumables,
   not by the available compute.
3. **The authority to spend was explicit and bounded.** Every procedure was a write with a named owner, a
   stated budget and a rollback (the next burn opportunity). The governance axis is not decoration here: the
   reason the sequence of burns worked is that each one was reversible in the only sense available — it could
   be corrected later — and that the correction budget was tracked in the same unit as the consumables.

The contrast with Ariane 5 is therefore not "careful versus careless". It is that one institution treated its
envelope as an assumption to be inherited and the other treated it as a measurement to be re-taken. Collapse
Proposition 1 is precisely the failure to notice the difference, and it is a *type* error before it is an
engineering error.

## 5. The ceiling is computable, and that is the point

There is a temptation in every quantitative field to treat a theoretical ceiling as a forecast. The theory of
artificial intelligence supplies the sharpest available example, and this chapter is its natural home.

**AIXI** is the optimal agent for a universal prior: the policy that maximises expected reward with respect to
Solomonoff's universal mixture over computable environments `[Literature]`. It is, in a precise sense, the best
possible performance an agent can achieve when the environment is unknown and the only prior is computability itself.
Two facts about it matter here, and they pull in opposite directions.

The first is that AIXI is **not computable**. Its prior is a mixture over all programs, and the expected value of an
action is defined by a sum over an incomputable set. Every approximating agent is therefore a different object, and
the gap between an approximation and the ideal is not a small perturbation: it is the difference between a limit and
anything that can be run. It is the same obstruction that §1 records for semantic verification — in both cases the
object with the clean theory is the one that cannot be executed — and it is the reason a *ceiling* is a mathematical
statement about an ideal agent, not an upper bound on practice.

The second is that the ceiling is nonetheless informative, because it says *what* is being traded. An agent that
optimises the universal prior spends all its computation on description; an agent that optimises a fixed computable
approximation spends it on search. The distinction between these two budgets is the computable form of the
combinatorial/exploratory split: one axis counts the states an agent can enumerate, the other the hypotheses it can
represent, and the practical difficulty of a domain is a statement about where the search sits on the two axes
together. That split is also the honest way to read every capability taxonomy: a taxonomy that ranks systems on a
single scale has chosen one axis and hidden the other.

The formal layer's contribution is therefore a discipline of statement, and it is the same discipline that makes the
Ariane 5 case legible. A claim of the form "this agent achieves $x$" is admissible when $x$ is measured inside a
declared task distribution, with the approximation named. A claim of the form "this agent approaches the optimum" is
admissible only when the optimum is computable and the metric is stated. A claim that the *field* approaches a
ceiling because compute grows is neither: it compares a measured trajectory against an incomputable reference, which
is the formal restatement of the confident extrapolation that Collapse Proposition 1 describes.

## 6. The three collapses in the formal order

1. **Collapse Proposition 1 ($\neg S_{\mathrm{epi}}$ — Silent Domain Narrowing):** Assuming an input lies
   inside a narrow type range without bounding out-of-range states — the exact mechanism of the 1996
   Ariane 5 Flight 501 loss, where a 64-bit horizontal-velocity float was cast into a 16-bit signed
   integer under the unverified premise that Ariane 4's flight envelope still held `[Literature]`.
2. **Collapse Proposition 2 ($\neg S_{\mathrm{vital}}$ — Unbounded Proof Latency):** Running an
   exponential or undecidable verification routine inside a real-time control loop until
   $\tau_{\mathrm{bureaucratic}} > \mathrm{TTS}_{\mathrm{survival}}$.
3. **Collapse Proposition 3 ($\neg S_{\mathrm{gov}}$ — Ambient Authority and Unchecked Writes):** Allowing
   an agent to execute `grammar.rs::AgentActionKind` writes outside its authorised contract scope
   (`OutOfScope`), spending common reserves without an attested control tape.

## Bridge

Layer 06 (`multilayer.rs::KnowledgeOrder`, `KnowledgeOrder::L06FormalComputableLinearType`, `{T1Framing, T4Formalization, T5Temporal}`) propagates its local and ancestor-cone margins through `multilayer.rs::MultilayerHomomorphism::evaluate` into [Chapter 57 — Layer 07: Probability Foundations, Imprecise Credences & Measurement Axiomatics Order](57-layer-probability-credence-measurement-order.md) (`KnowledgeOrder::L07ProbabilityCredenceMeasurement`, Stratum II, `{T1Framing, T2Severity, T4Formalization}`).
