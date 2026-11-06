# Aim and the Three Planes

> Foundation chapter of Part III. Its formal content is worked out in the
> [Preface](00-preface.md) and in [Chapter 42](42-sequential-loop-naive-agent.md).

## By the end of this chapter you can

1. Draw the full circuit of a single act of judgment: material world, archive and model,
   institutional deliberation.
2. Explain why cutting the circuit at any one plane produces a specific, predictable failure.
3. Distinguish a good shot that missed from a bad shot that hit.

## 1. Three planes, one circuit

Every act of consequential judgment passes through three distinct ontic planes, in an inescapable sequence:

The **world** plane ($P_1$) is physical and social reality: raw load, material degradation, patient illness,
atmospheric turbulence, liquidity shocks, and the metabolic survival of communities. It operates under its
own irreversible time and physical constraints. The world does not consult the model, nor does it alter its laws
to accommodate human desires.

The **archive** plane ($P_2$) is the structured repository of what was measured, recorded, and inferred. It
contains the telemetry, the sensor streams, the database tables, the fitted statistical models, and the
mathematical intervals they license. This plane is inevitably thinner than the world: it captures only what
an institution possessed the sensors to detect, the budget to log, and the vocabulary to categorize.

The **institution** plane ($P_3$) is deliberation, authority, and accountability: who decides, on what legal
warrant, with what vital reserves at stake, at whose physical risk, and who will answer when the consequences
unfold. Every decision in human society is executed here, mediated by individuals embedded in formal and
informal power structures.

The core axiom of Stochology is that these three planes form **one unified mediation circuit**, not three
autonomous disciplines. An engineer who optimizes the archive ($P_2$) in isolation from institutional governance
produces mathematical ornaments; a policymaker who exercises power ($P_3$) without grounding in physical limits
is hallucinating; an operator who interacts with the world ($P_1$) without historical memory is perpetually
infantile. The irreducible unit of analysis is the closed loop.

```
                  +-----------------------------------+
                  |         INSTITUTION (P3)          |
                  |  Deliberation, Authority, Reserves|
                  +-----------------+-----------------+
                                   / \
                                  /   \
                  Decision & Act /     \ Evidentiary Intake
                     Transit T3 /       \ Transit T2
                               /         \
                              v           \
     +-------------------------+         +-------------------------+
     |        WORLD (P1)       |         |       ARCHIVE (P2)      |
     | Material Process & Loss |-------->| Registers, Fitted Models|
     +-------------------------+ Transit | & Partial Bounds        |
                                   T1    +-------------------------+
```

Across these three planes unfold the seven canonical circuit transits ($T_1$ through $T_7$) and their
corresponding guard sites ($G_1$ through $G_7$ in `grammar.rs::GuardSite`):
- **Direct Loop Transits**:
  - $T_1$ ($P_1 \to P_2$): *Observation and Transduction*. World events become archive rows under selection
    mechanisms, filtering thresholds, and measurement error. Guarded by Archive Integrity ($G_3$) and Inquiry
    Framing ($G_1$).
  - $T_2$ ($P_2 \to P_3$): *Inference and Evidence*. Models are estimated, partial-identification intervals are
    constructed, and hypothesis tests are evaluated. Guarded by Severity ($G_2$) and Formalization Bounds ($G_4$).
  - $T_3$ ($P_3 \to P_1$): *Intervention and Risk Imposition*. Decisions are executed, resources are committed,
    and vital reserves are deployed. Guarded by Temporal Survival ($G_5$, $\mathrm{TTS} \ge \mathrm{TTR}$) and
    Performative Dynamics ($G_6$).
- **Reflexive Locks**:
  - $T_4$ ($P_2 \to P_2$): Epistemic self-audit, verifying that analytical choices were not p-hacked and that
    Cromwell openness is maintained.
  - $T_5$ ($P_1 \to P_1$): Physical vital lock, verifying that the resilience basin is intact and Kingman queue
    congestion does not diverge.
  - $T_6$ ($P_3 \to P_3$): Governance self-audit, auditing algedonic channels, testimonial discounting
    ($\delta_{\mathrm{cred}}$), and conflict-of-interest firewalls.
- **Integral Closure**:
  - $T_7$: Triadic closure across all three planes, computing the non-substitutable Leontief minimum
    $M_{\mathrm{Leontief}} = \min(S_{\mathrm{epi}}, S_{\mathrm{vital}}, S_{\mathrm{gov}})$ ($G_7$).

## 2. Cutting the circuit and missing data mechanisms

Severing the circuit at any single plane produces an unmistakable pathology:

**World-only amnesia ($P_1$ isolated).** Action occurs without memory or institutional reflection. Every crisis
is confronted as if it had no history. The institution fails not from lack of effort, but from structural amnesia:
the same catastrophic error is paid for repeatedly, and no archive ever accumulates to prevent its recurrence.

**Archive-only irrelevance ($P_2$ isolated).** Statistical models are fitted with exquisite rigor, partial
bounds are computed, confidence intervals are tight, and papers are published. Yet the institution never connects
its decisions to the findings. The failure is *irrelevance with high production values*: the most seductive
pathology, because it mimics scientific excellence while allowing real-world catastrophe to proceed unchecked.

**Institution-only autocracy ($P_3$ isolated).** Power decides by administrative fiat. When the archive conflicts
with political convenience, the archive is silenced, re-baselined, or doctored. Alternatively, the archive
documents an impending rupture, but no official possesses the standing or authority to halt the operation.
The failure is deliberate blindness ending in preventable ruin.

At the crucial transit $T_1$ ($P_1 \to P_2$), real-world data collection encounters the three classic missingness
mechanisms formalized by Donald Rubin:
- *Missing Completely at Random (MCAR)*: The probability of an event being recorded is independent of both observed
  and unobserved variables. This is the only benign regime where listwise deletion leaves point estimates unbiased.
- *Missing at Random (MAR)*: Missingness depends on observed features within the database schema. While recoverable
  through conditional modeling, the assumption of MAR is strictly untestable from the data itself.
- *Missing Not at Random (MNAR)*: Missingness depends directly on the unobserved value itself — the most severely
  depleted reserves go unreported, the most distressed patients leave the triage queue, the most toxic loans are
  concealed. Under MNAR, point estimation is impossible; the honest analyst must construct partial-identification
  bounds under worst-case assumptions.

To prevent historical revisionism and hindsight bias across the archive transit, robust systems enforce
**bitemporal data architecture**: every record stores both *valid time* (the physical timestamp when the event
occurred in $P_1$) and *transaction time* (the cryptographic timestamp when the archive $P_2$ recorded or amended
it). Furthermore, where entire subpopulations are missing from the register, Anne Chao's dark-archive species
estimator calculates the number of completely unobserved classes $\hat{n}_{00} = f_1^2 / (2 f_2)$ from the count
of singletons ($f_1$) and doubletons ($f_2$), bounding the silence of the vault.

## 3. Good aim, bad outcome

Suppose an archer executes a technically flawless shot under optimal stance and aim, but an unforeseen squall
deflects the arrow into the sand. Was the action virtuous?

In *Apt Belief*, Ernest Sosa introduced the **AAA framework** of virtue epistemology, which distinguishes three
evaluative levels:
1. **Accuracy** (*skopós*): Was the outcome successful? Did the arrow hit the bullseye?
2. **Adroitness**: Was the performance executed with genuine competence, following valid rules of craft?
3. **Aptness** (*télos*): Was the performance accurate *because* it was adroit?

A shot can be accurate without being adroit — the wild fluked arrow carried onto the target by a freak gust of
luck. It can be adroit without being accurate — the master shot ruined by external weather. Only when accuracy is
causally produced by adroitness does the act attain *aptness*, which is the sole performance worthy of institutional
credit.

Above these three sits a fourth, sovereign grade of judgment: the agent who is adroit, knows the limits of his
competence, **and knows when not to shoot**. Declining to release the arrow when the target is shrouded in fog or
the target has been corrupted is not cowardice; it is meta-cognitive virtue. An agent that cannot lower the bow
will inevitably be ordered to fire into darkness, and will obediently cause catastrophe.

## 4. The circuit as the unit of judgment

Because judgment is a property of the full circuit, an empirical claim that addresses only one plane is
structurally incomplete:
- *"Our machine learning model predicts a 12% probability of system failure."* This is a bare archive-plane
  ($P_2$) statement. It reveals nothing about whether 12% is survivable by the physical grid ($P_1$), nor who
  is legally authorized to disconnect load ($P_3$).
- *"We maintain six weeks of emergency inventory in the warehouse."* This is a bare world-plane ($P_1$) statement.
  It says nothing about whether supply chain replenishment latency ($P_3$) can restore that stock before exhaustion.
- *"The safety board unanimously approved the operational waiver."* This is a bare institution-plane ($P_3$)
  statement. It says nothing about whether the physical premises presented to the board were factually grounded
  in verified archive telemetry ($P_2$).

A complete stochological judgment synthesizes all three: an honest, bounded estimate from the archive, a verified
physical survival condition in the world, and an accountable, pre-registered decision in the institution.

## Bridge

The circuit is executable in [Chapter 42](42-sequential-loop-naive-agent.md) as
`engine.rs::TrajectoryReport`, which carries every step of an agent's run with its declared decision and its
realised outcome. The guard sites that preserve the integrity of the loop are the seven sites of `grammar.rs::GuardSite`
— framing, severity, archive, formalisation, temporal, performative, and liability — partitioned across the
Nuclear Triad as [Appendix B](appendix-b-architecture-verification.md) details; the four grammatical verdicts
issued over a trajectory are `grammar.rs::GrammarVerdict`; and the three-plane ablations demonstrating structural
collapse are the four policies of `engine.rs::StochologicalAgent` — `StochologicalAgent::full`,
`StochologicalAgent::epistemic_only`, `StochologicalAgent::without_epistemic_margin`, and
`StochologicalAgent::without_governance`.
