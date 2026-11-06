# **The Grammar of Aim: Derivations, Acceptance, and the Two Movements of the Sequential Loop**

*(A formal syntax layer for Stochology — generation and judgment over the same productions)*

---

## **Abstract**

This note formalizes the **grammar of aim** that underlies [`The Ethics of Aim.md`](The%20Ethics%20of%20Aim.md) but is never written as a grammar there. It is not a fourth essay and does not extend the treatise: it fixes the *syntax layer* — the alphabet, the productions, the acceptance predicate, and the derivational status of the three collapse propositions — and it does so in a form that can be executed. The grammar is deliberately **two-sorted**: a control tape (*télos*, what the agent issues) and a contingency tape (*skopós*, what the world returns). It is also deliberately **single-motored**: the productions carry no dynamics of their own, and every margin is *declared* by the Three Canonical Operators of the companion crate. From this design two movements emerge over one object. **Generation** enumerates the language (coverage regime) and searches for the shortest counterexample against the declared thresholds (adversarial regime), each sentence sealed as a certificate. **Judgment** parses an observed trajectory into a derivation and returns one of four verdicts — interior, frontier, collapse (classified by the existing `CollapseProposition`), or *unparsable* — the last being a mandatory quarantine for radical novelty rather than a diagnostic of failure. The central formal result is a non-commutation property: no admissible verdict is a function of the contingency tape alone, which is Antipater's separation of *télos* and *skopós* rendered as a theorem about the acceptance predicate rather than as a paragraph about archers.

**Keywords:** grammar; derivation; acceptance predicate; partial identification; *télos* and *skopós*; collapse propositions; quarantine; Stochology.

*Note on Provenance and Notation:* The four canonical provenance classes of `AGENTS.md` §III are preserved verbatim: **`[Measured]`** for computations on sealed datasets in `data/*.csv`, **`[Simulated]`** for deterministic outputs of `CommunityReserveEnv` under `DEFAULT_SEED = 2026`, **`[Assumed]`** for stipulated parameters and closed-form analytical theorems/identities, and **`[Literature]`** for external findings. All executable statements in this note are implemented in [`crates/stochology/src/grammar.rs`](../crates/stochology/src/grammar.rs) and cross-checked against this document by [`crates/stochology/tests/grammar_spec.rs`](../crates/stochology/tests/grammar_spec.rs); the document and the module are not permitted to drift apart.

*Status of this note:* Section §7.1 of the treatise already calls Stochology a "transversal grammar". That phrase is used there in the loose sense of a shared vocabulary across five methodological families. This note uses **grammar** in the technical sense: an alphabet, a set of productions, and a decision procedure that separates the sentences of the language from everything else. The technical sense is strictly narrower, and it is the only one that can be checked.

---

## **1. Three Candidate Grammars, and the One That Earns a File**

The repository already holds two notes with two different jobs. [`Aiming Between Mirrors.md`](Aiming%20Between%20Mirrors.md) is a rhetorical and ethical essay: its unit is the argument. [`The Ethics of Aim.md`](The%20Ethics%20of%20Aim.md) is a programmatic treatise: its unit is the section, the table, the lineage. Neither is checkable in the strict sense. A third file is therefore justified only if its unit is *derivational* — if it can say, of a candidate trajectory, that this one is a sentence of the language and that one is not.

Three readings of "grammar" are available, and each buys something different:

1. **Grammar of trajectories.** Alphabet of the loop, productions at the three transits, the seven constitutive tensions as guard positions, and collapse as an unguarded derivation.
2. **Grammar of claims.** A typed language for the provenance tags, so that every quantitative sentence carries its warrant and a checker can refuse a `[Measured]` claim with no sealed dataset behind it.
3. **Grammar of pathologies.** A morphology of failure: promise, inversion, brake — the shape of *Table 2* turned into a rewriting relation.

Only the first earns a note. The second is real and valuable, but it belongs in `scripts/verify.sh`, where the seven existing provenance checks already live: a syntax for claims that nobody parses is a reformatting of the same rules, not a new object. The third is a taxonomy wearing the name of a grammar: it has no decision procedure, because "is this a pathology of type 4?" is answered by judgment rather than by derivation, and its generative power is nil — it cannot produce a trajectory that was not already catalogued.

The criterion that settles the choice is this: **a grammar must be able to refuse.** A diagram can describe; only a grammar can say *this sequence of transits is not a well-formed derivation*, and say it before knowing whether the outcome was good. That is exactly the gap in the current state of the program. *Table 2* of the treatise maps planes, filters, tensions, and safeguards onto one another; it cannot adjudicate a novel trajectory, and it does not pretend to.

---

## **2. Lexicon and the Two Tapes**

The grammar introduces no new vocabulary. Its terminals are the fields that the repository already uses, which is what allows the syntax layer to sit over every chapter at once rather than over one.

**Planes.** `World`, `Archive`, `Institution` — the three planes of the mediation circuit (§1.1 of the treatise). Every transit in a derivation is a movement between two of them, and the institution plane is the only recursive one.

**Axes.** `Epistemic`, `Vital`, `Institutional` — the Nuclear Triad.

**Guard sites.** Seven, one per constitutive tension, in the chronological order of the loop. The partition is $3/2/2$ across the three axes of the Nuclear Triad and is asserted by test, not by convention. Ontologically, the seven sites form an exact $3 + 3 + 1 = 7$ bijection onto the directed transits (`CircuitTransit`) across the three planes (`World`, `Archive`, `Institution`)—namely the $3 \times 2 = 6$ ordered pairs of distinct planes (3 direct loop transits and 3 reflexive locks) plus 1 integral three-plane closure—and an exact 1-to-1 bijection onto the seven ontological strata (`S1`–`S7`) of [`The Multilayer Graph of Knowledge and Aim.md`](The%20Multilayer%20Graph%20of%20Knowledge%20and%20Aim.md):

| Guard site | Tension | Axis | Directed Transit (`CircuitTransit`) | Primary Plane | Canonical Stratum | Governing operator |
| :--- | :---: | :---: | :--- | :---: | :---: | :--- |
| `T1Framing` | 1 | $S_{\mathrm{epi}}$ | `Institution -> Archive` *(Reflexive Lock 1)* | `Institution` | `S2` (`S2FormalMetrological`) | `EpistemicMarginState::from_inquiry_framing` |
| `T2Severity` | 2 | $S_{\mathrm{epi}}$ | `Archive -> World` *(Reflexive Lock 2)* | `World` | `S4` (`S4ClinicalFormative`) | `EpistemicMarginState::from_ate_bounds` |
| `T3Archive` | 3 | $S_{\mathrm{epi}}$ | `World -> Archive` *(Direct Transit 1)* | `Archive` | `S5` (`S5SemioticArchival`) | `EpistemicMarginState::from_archive_clusters` |
| `T4Formalization` | 4 | $S_{\mathrm{vital}}$ | `Archive -> Institution` *(Direct Transit 2)* | `Archive` | `S3` (`S3LivingEvolutionary`) | `VitalMarginState::from_heavy_tails_and_asymmetry` |
| `T5Temporal` | 5 | $S_{\mathrm{vital}}$ | `World -> Institution` *(Reflexive Lock 3)* | `World` | `S1` (`S1PhysicalDynamical`) | `VitalMarginState::from_queue_basin_and_survival` |
| `T6Performative` | 6 | $S_{\mathrm{gov}}$ | `Institution -> World` *(Direct Transit 3)* | `Institution` | `S6` (`S6StrategicEconomic`) | `GovernanceState::from_ex_ante_gate` |
| `T7Liability` | 7 | $S_{\mathrm{gov}}$ | `World <-> Archive <-> Institution` *(Integral)* | `Institution` | `S7` (`S7ConstitutionalBiospheric`) | `TelicGrade`, `TemporalVault` |

*Table 1: The seven guard sites, their constitutive tension, axis (`3/2/2`), directed circuit transit (`3 + 3 + 1 = 7`), primary plane (`2/2/3`), canonical knowledge-graph stratum (`S1`–`S7`), and the canonical operator that declares each margin.*

**Two tapes.** A single step of the loop is a **shot**: a pair `⟨telos, skopos⟩`. The control tape is the `AgentDecision` — the declared bounds, the provisioned capacity, the reserve allocation, the proxy pressure, the testimonial discount, the brake. The contingency tape is the `StepOutcome` — the true latent demand, the realized utilization, the realized harm, the next reserve, the ruin flag, the Sosa grade. This split is not decoration. It is the formal counterpart of the distinction the whole program inherits from Antipater of Tarsus: the shot belongs to the agent, the hit belongs to the wind.

**Guards.** A guard reading is `⟨site, margin, step⟩`, where `margin ∈ [0, 1]` is *declared* by the governing operator and `step` is the step at which a violation was actually **witnessed**, when one was. The distinction between a declared margin and a witnessed violation is where intellectual honesty enters the syntax: **absence of evidence is not a satisfied guard**, and the grammar must be able to say so without pretending to have seen what it did not see.

---

## **3. Productions**

The skeleton grammar generates guard-free trajectories. Guard annotation is a separate, total map — this separation is what keeps syntax and semantics from contaminating one another.

```ebnf
Sentence   ::= Framing Counting Body
Framing    ::= "frame"  [ "insulate" | "open" ] [ "type3" ]
Counting   ::= "count"  Interval [ "deff" ] [ "vintages" ]
Body       ::= Decision [ Brake ] ( Write | Read )* [ Halt ] [ Step* ]
Step       ::= Framing Counting Body          (* recursion: the Institution reframes the World *)
Decision   ::= "act"    "lex" "(" "survival" "," "minimax_regret" ")"
Brake      ::= "brake"  "tts" ">=" "ttr"
Write      ::= "write"  Scope
Read       ::= "read"   Scope
Halt       ::= "halt"
Interval   ::= "[" Scalar "," Scalar "]"      (* requires L <= U *)
```

**Why `Brake` terminates the recursion.** The only recursive production is `Step`, and its left corner is the institution plane. This is not an aesthetic choice. Under a grammar with an institution-left-corner the parse of a trajectory is deterministic and linear: each step consumes exactly one transit triple, and there is never a moment where two partial parses compete for the same input. Under a grammar without that pivot the same trajectory admits two parses, and two parses mean two verdicts on one history — the formal shadow of the situation in which a committee and an auditor disagree about what happened while both are reading the same file.

**Termination of the recursion is a time budget, not a depth limit.** In the physical reading, the recursion stops when the institutional deliberation latency fits inside the survival clock, $\tau_{\mathrm{bureaucratic}} \le \mathrm{TTS}_{\mathrm{survival}}$. That is Reflexive Brake 3 of the treatise (§7.2), and here it appears as the reason the `Brake` production exists at all rather than as a paragraph of institutional advice. A grammar of aim in which the recursive production can outlast the system it describes is a grammar of a system that will not finish its own sentence.

**No threshold lives in the grammar.** Every $\tau$ is a parameter of the engine. The grammar refers to thresholds by name and never owns them, because an acceptance threshold that becomes a published target is exactly the Goodhart degeneration that Reflexive Brake 4 prohibits for $M_{\mathrm{Leontief}}$.

---

## **4. The Acceptance Predicate and the Four Verdicts**

The predicate is evaluated in a fixed order, and the order carries the epistemology.

1. **Skeleton.** An empty derivation (`EmptySentence`), a step missing its attested control tape (`MissingControlTape`), an out-of-scope write or delegation (`OutOfScope`), an unauthorized read (`UnauthorizedRead`), an exceeded token or tool-call budget (`BudgetExhausted`), or a control tape that violates a declared invariant (inverted bounds $L > U$, non-positive provisioned capacity, negative reserve allocation), is `Unparsable(SkeletonMismatch)` or its specific quarantine variant. An ill-formed input has no verdict; it does not have a *bad* verdict.
2. **Totality of the annotation.** Every one of the three axes must carry at least one guard reading. A missing axis is `Unparsable(UnguardedTransit(site))`: an unguarded transit is not a weak derivation, it is not a derivation.
3. **Finite readings.** A non-finite margin is `Unparsable(NonFiniteMargin(site))`. This rule exists because the alternative is worse: a `NaN` that silently compares false against a threshold would pass every guard it touches. In a grammar whose subject is the refusal to let unquantified doubt masquerade as quantified assurance, a missing number must be louder than a bad one.
4. **The Leontief fold.** The three axis scores are $s_a = \min_{i \in a} m_i$ over the sites of axis $a$; the sentence is accepted by `TriadicIntegrity::evaluate(s_epi, s_vital, s_gov, τ)`, whose bottleneck is $M_{\mathrm{Leontief}} = \min(s_{\mathrm{epi}}, s_{\mathrm{vital}}, s_{\mathrm{gov}})$. The grammar does not re-derive this fold. It calls it. If the grammar computed margins independently, the repository would hold two engines and one of them would eventually be wrong without anyone noticing.
5. **The frontier band.** If the fold returns `IntactTriad`, a sentence still sitting inside the band $[\tau, \tau + 0.05)$ on any axis is `Boundary`, not `Interior`. The band is `[Assumed]` and is a parameter, not a discovery: the point is that "accepted" and "comfortable" are different verdicts.

The four verdicts are therefore:

- **`Interior`** — every axis clears $\tau + 0.05$ (the frontier band);
- **`Boundary { site, margin }`** — accepted, but the weakest site of an axis is inside the near-critical band;
- **`Collapse { proposition, at, step_t }`** — at least one axis falls below $\tau$; the mask maps onto the existing `CollapseProposition` of `synthesis.rs`, so the grammar inherits the eight-cell cube rather than inventing a parallel taxonomy; `at` names the first violating site in step order and `step_t` the step at which it was witnessed, when it was;
- **`Unparsable(reason)`** — quarantine, never collapse.

The last verdict is the most important line in the file. A grammar whose only failure mode is collapse will classify every phenomenon it cannot parse as a catastrophe, which is precisely the dogmatic error the treatise spends three sections diagnosing. **The inaudible is not the ruined.** A trajectory carrying a form of suffering for which the archive has no column, an event outside the declared state space, a claim that escapes all seven locks — these are reasons for quarantine, and quarantine has its own name.

---

## **5. The *Télos*/*Skopós* Projection and the Non-Commutation Property**

Let a sentence be a pair $(S, G)$ of shots and guard readings. Define the two projections $\pi_{\text{télos}}$ and $\pi_{\text{skopós}}$ that read the two tapes of each shot.

**Proposition (non-commutation of aim and outcome).** *For any two sentences $(S, G)$ and $(S', G')$ with $\pi_{\text{télos}}(S) = \pi_{\text{télos}}(S')$ and $G = G'$, the acceptance predicate satisfies $\mathrm{judge}(S, G) = \mathrm{judge}(S', G')$. Consequently no admissible verdict is a function of the contingency tape alone.* `[Assumed]`

*Proof.* The predicate reads the control tape only through the declared epistemic bounds, the provisioned capacity, and the reserve allocation — the skeleton check and the payload — and it reads the contingency tape only into the seal, never into the verdict. Substituting any contingency tape with the same control tape and the same guard readings therefore leaves every branch of the predicate untouched. ∎

Two consequences follow, and both are the point of writing the theorem instead of the paragraph.

First, **luck is not evidence of aim.** The engine's own bitemporal taxonomy already distinguishes the four grades: `LuckyAccurateOnly` is the case of an agent that hit the target by moral luck after neglecting the method, and `AdroitExAnteUnluckyOutcome` is the case of an agent that aimed well and was hit by an exogenous shock. The non-commutation property makes the grammar honor that taxonomy *structurally*: because the verdict cannot see the contingent tape, an accurate hit cannot buy an `Interior` verdict that the declared margins do not support. The arrow is graded at the bow.

Second, **the auditor cannot grade what only the world knows.** This is not a defect of the formalization; it is the formalization of a limit already stated in the treatise (§7.2, boundary of failure): when the archive is erased, the sharpest available verdict is a wide interval, and no grammar can manufacture resolution out of a missing column.

The property is asserted executably, by permuting the contingency tape while holding the control tape and the margins fixed and requiring the verdict to be unchanged.

---

## **6. The Generative Movement**

A grammar that only judges is a parser. The same productions, read in the other direction, generate — and the two regimes of generation have different epistemic standing, which must be declared rather than blurred.

**Coverage regime.** `enumerate` walks every guard vector over a finite grid of admissible margins up to a declared depth. The enumeration itself is `[Assumed]`: given a grid of $g$ levels and seven sites, the language contains exactly $g^7$ guard configurations, and the count of collapsing configurations is exactly $g^7$ minus the number of vectors whose every level clears $\tau$. Nothing here is measured, and nothing here is stochastic. The shots attached to each generated sentence are canonical `[Simulated]` placeholders; they exist so the sentence is well-formed, not to claim a measurement.

**Adversarial regime.** `shortest_violation` runs a breadth-first search, shortest derivation first and fewest violated sites first, for the cheapest counterexample against the declared thresholds. This is the grammar's contribution to the program's own refutability: Falsifiers $F_1$–$F_4$ interrogate the world, while the adversarial regime interrogates *the thresholds themselves*, and it does so without waiting for data. If a single guard at a single step can be pushed below $\tau$ at a level the grid admits, the grammar returns that derivation before any field study begins.

**Sealing.** Every sentence constructed by the module is committed with a `TemporalVault` seal over a canonical, dependency-free payload encoding. A sentence is therefore not a narrative about a trajectory but a certificate of one: tamper with a margin or a step and the seal fails. The purpose is the same as in the treatise's pre-registration requirement — to make it impossible to revise a derivation after seeing its verdict and still present it as the same derivation.

**Budget caps and explicit coverage ledger.** Duplicate grid levels are deduplicated before checking `MAX_ENUMERATION = 300_000`. The grid is quantile-subsampled to $6$ levels ($6^7 = 279{,}936$ vectors) only when $g_{\text{dedup}}^7 > 3 \times 10^5$ (so $g = 5$ preserves all $5^7 = 78{,}125$ configurations and $g = 6$ preserves all $6^7 = 279{,}936$ configurations without truncation), depth is clamped to three, and `Coverage` (`enumerate_with_coverage`, `shortest_violation_with_coverage`) records `requested`, `enumerated`, `distinct_levels`, and `truncated`. An unbounded generator is Monte Carlo with a better name; the caps and the `Coverage` certificate are what keep "generation" a finite, reproducible claim.

---

## **7. Instantiation in the Codebase**

### **7.1 Anchor block**

Every production of this note is anchored to a symbol that already exists in the crate. The block below is parsed by the specification test, which fails if an anchor no longer resolves — the same anti-drift discipline that `scripts/verify.sh` already applies to `#L` anchors in the textbook.

```grammar-anchors
S                   -> crates/stochology/src/engine.rs :: CommunityReserveEnv::run_trajectory
Trilha              -> crates/stochology/src/engine.rs :: AgentPolicy
Framing             -> crates/stochology/src/epistemic.rs :: EpistemicMarginState::from_inquiry_framing
Contagem            -> crates/stochology/src/epistemic.rs :: EpistemicMarginState::from_archive_clusters
Formalizacao        -> crates/stochology/src/vital.rs :: VitalMarginState::from_heavy_tails_and_asymmetry
Decisao             -> crates/stochology/src/vital.rs :: VitalMarginState::from_queue_basin_and_survival
Intervencao         -> crates/stochology/src/governance.rs :: GovernanceState::from_ex_ante_gate
Guarda.S_epi        -> crates/stochology/src/epistemic.rs :: EpistemicMarginState::epistemic_margin_score
Guarda.S_vital      -> crates/stochology/src/vital.rs :: VitalMarginState::vital_margin_score
Guarda.S_gov        -> crates/stochology/src/governance.rs :: GovernanceState::nested_leontief_score
Aceitacao           -> crates/stochology/src/synthesis.rs :: TriadicIntegrity::evaluate
Veredito            -> crates/stochology/src/synthesis.rs :: CollapseProposition
Fita.telos          -> crates/stochology/src/engine.rs :: AgentDecision
Fita.skopos         -> crates/stochology/src/engine.rs :: StepOutcome
Selo                -> crates/stochology/src/governance.rs :: TemporalVault::seal
Registo             -> crates/stochology/src/governance.rs :: AuditLog
Freio               -> crates/stochology/src/synthesis.rs :: ReflexiveBrakeAudit::evaluate
Acao                -> crates/stochology/src/grammar.rs :: AgentAction
Leitura             -> crates/stochology/src/grammar.rs :: ReadReading
Cobertura           -> crates/stochology/src/grammar.rs :: Coverage
Quarentena          -> crates/stochology/src/grammar.rs :: UnparsableReason
Geracao.cobertura   -> crates/stochology/src/grammar.rs :: enumerate
Geracao.adversarial -> crates/stochology/src/grammar.rs :: shortest_violation
Transito            -> crates/stochology/src/grammar.rs :: CircuitTransit
Estrato.guarda      -> crates/stochology/src/multilayer.rs :: KnowledgeStratum::canonical_guard_site
```

### **7.2 Worked example: the blind-undersizing cell**

Consider the $2 \times 2 \times 2$ ablation cell `011` — epistemic margin removed, vital and institutional safeguards retained — run on `CommunityReserveEnv::new(DEFAULT_SEED, 24, 45.0)` under `DEFAULT_SEED = 2026`. The engine reports epistemic coverage $25.0\%$ and $M_{\mathrm{Leontief}} = 0.235$ `[Simulated]`. Feeding that report through the recognizer yields:

- **Verdict:** `Collapse`
- **Proposition:** `Proposition1BlindUndersizing`
- **Site:** `T1Framing`
- **Step:** `0`
- **Witnesses:** the first step where the declared bounds failed to cover the true latent demand (`T1Framing`), and the first step whose raw archive mean sat below the true latent demand (`T3Archive`). The earlier of the two, ties broken by site order, is `T1Framing` at step `0`.

Read the verdict as a sentence of the grammar: the derivation opens with a framing that insulates itself from the vulnerable stratum, sizes reserves against the resulting point mirage, and is therefore ill-formed from its first production onward. The collapse proposition is not an interpretation appended to the trajectory; it is the name of the production that was skipped.

Note what the example does **not** claim. It does not claim that `T2Severity`, `T4Formalization`, `T6Performative`, or `T7Liability` held — only that no step in this report witnessed a violation there. The recognizer reports the first *witnessed* violation and stays silent about the rest. That silence is a property of the evidence, not a property of the trajectory.

### **7.3 Agreement with the engine, cell by cell**

The grammar never adjudicates independently. For all eight cells of the factorial cube, the specification test requires the grammar verdict to agree with `report.triadic_integrity.diagnose` — the proposition the engine already computes:

- if the grammar returns `Collapse`, its proposition must equal the engine's diagnosis;
- if the grammar returns `Interior` or `Boundary`, the engine's diagnosis must be `IntactTriad`;
- if the grammar returns `Unparsable` for a trajectory the engine successfully produced, the test fails.

This is a stronger commitment than it looks. It means the note cannot quietly disagree with the code, and it means the code cannot quietly disagree with the note: a change to any of the three margins, to the threshold, or to the seven-site partition breaks the test rather than the prose.

### **7.4 What the grammar does not judge**

Three exclusions are carried over verbatim from the treatise, and they are the reason `Unparsable` is a first-class verdict:

1. **Qualitative hermeneutical silence.** Interval arithmetic over existing columns cannot invent the missing category. A form of suffering that the archive has no schema to record is not a wide bound; it is outside the lexicon.
2. **Ontological novelty.** A regime the state space does not contain is not an extreme tail. The generator can extend the grid; it cannot extend the world.
3. **Living political praxis.** A veto rule is a shell. The grammar can read whether the brake was declared and whether it fired; it cannot read whether the governed still contest it.

Where these three enter, the recognizer returns quarantine. A framework that diagnosed novelty as collapse would have become the very instrument of legitimation it was built to audit.

---

## **8. References**

CICERO, Marcus Tullius (1914). **De Finibus Bonorum et Malorum**. Translated by H. Rackham. Cambridge, MA: Harvard University Press. (Loeb Classical Library). [*De Finibus* III.22, on Antipater's archer and the separation of *télos* and *skopós*.]

HOPCROFT, John E.; MOTWANI, Rajeev; ULLMAN, Jeffrey D. (2006). **Introduction to Automata Theory, Languages, and Computation**. 3rd ed. Boston: Pearson/Addison-Wesley. [Grammars, derivations, ambiguity, and decision procedures.]

MANSKI, Charles F. (2003). **Partial Identification of Probability Distributions**. New York: Springer. [Sharp identification regions as the descriptive discipline the epistemic margin operationalizes.]

MAYO, Deborah G. (2018). **Statistical Inference as Severe Testing: How to Get Beyond the Statistics Wars**. Cambridge: Cambridge University Press. [Severity and the refusal of underpowered absolution.]

SOSA, Ernest (2015). **Judgment and Agency**. Oxford: Oxford University Press. [*Fully apt* performance and the grading of agency by competent aim rather than by lucky outcome.]

*Companion documents:* [`The Ethics of Aim.md`](The%20Ethics%20of%20Aim.md) (the treatise this note formalizes), [`Aiming Between Mirrors.md`](Aiming%20Between%20Mirrors.md) (the ethical refusals the treatise begins from), and [`crates/stochology/src/grammar.rs`](../crates/stochology/src/grammar.rs) (the executable shadow of this note).
