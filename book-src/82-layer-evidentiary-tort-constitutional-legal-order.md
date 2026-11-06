# Chapter 82 — Layer 32: Civil Evidentiary Law, Tort Liability & Constitutional Entrenchment Order

> Layer 32 (`multilayer.rs::KnowledgeOrder`, `KnowledgeOrder::L32EvidentiaryTortConstitutional`) of Part V · Stratum VII (`multilayer.rs::KnowledgeStratum`, `KnowledgeStratum::S7ConstitutionalBiospheric`) · Unique Guard Combination `{T1Framing, T4Formalization, T7Liability}` · Signature `(1, 1, 1)` (`multilayer.rs::TriadicExposureRegime`, `TriadicExposureRegime::TransTriadicKeystone`). It builds on [Layer 05](55-layer-fracture-continuum-structural-order.md), [Layer 30](80-layer-corporate-governance-principal-agent-order.md), [Layer 31](81-layer-criminal-proof-forensic-epistemology-order.md) in $\mathcal{G}_1$ and maps into $\mathcal{G}_2$ via [`notes/The Multilayer Graph of Knowledge and Aim.md`](../notes/The%20Multilayer%20Graph%20of%20Knowledge%20and%20Aim.md).

## By the end of this chapter you can

1. Derive the three classical legal standards of proof from the expected-regret threshold
   $p^*_{\mathrm{PK}} = C_{\mathrm{FP}}/(C_{\mathrm{FP}} + L_{\mathrm{FN}})$, distinguish it from the
   cost–loss threshold $p^*_{\mathrm{CL}} = C/(\eta L)$, and resolve Cohen's Gatecrasher Paradox using
   Keynes's evidential weight $V$.
2. Explain why Learned Hand's negligence formula fails under convexity and limited liability, and design
   a two-circuit legal remedy that separates no-fault victim repair from escrow clawbacks.
3. Map constitutional pre-commitment, *ex post facto* prohibitions, bicameral vetoes, and whistleblower
   statutes onto the operators of `governance.rs`.

## 1. Standards of proof and the Gatecrasher Paradox

Every trial is a decision under structural uncertainty where acquittal or judgment must precede omniscient
knowledge of the past. Two distinct thresholds are in play, and the three **standards of proof** `[Literature]`
below are stated in the second of them. The **cost–loss threshold** compares a protection budget $C$ with an
efficiency-discounted avoidable loss $\eta L$: acting is worth its expense once the event probability clears

$$p^*_{\mathrm{CL}} = \frac{C}{\eta L} \qquad \text{\texttt{[Assumed]}}.$$

The **expected-regret threshold** instead compares the two classification errors — net cost of a false alarm
$C_{\mathrm{FP}}$ against net loss of a missed event $L_{\mathrm{FN}}$ — and is exactly what
`epistemic.rs::pauker_kassirer_threshold` computes when handed those two non-negative terms
($c = C_{\mathrm{FP}}$, $l = L_{\mathrm{FN}}$), returning $c/(c + l)$ and falling back to parity when both
budgets vanish:

$$p^*_{\mathrm{PK}} = \frac{C_{\mathrm{FP}}}{C_{\mathrm{FP}} + L_{\mathrm{FN}}} \qquad \text{\texttt{[Assumed]}}.$$

The standards are:

- **Preponderance of the evidence ($p^*_{\mathrm{PK}} > 0.50$):** Used in ordinary civil litigation where a false
  positive (ordering a defendant to pay $\$1$) and a false negative (leaving a plaintiff unpaid by $\$1$)
  are treated as symmetric ($C_{\mathrm{FP}} \approx L_{\mathrm{FN}}$).
- **Clear and convincing evidence ($p^*_{\mathrm{PK}} \approx 0.75$ `[Assumed]`):** Required when civil proceedings
  threaten fundamental interests such as parental rights, civil commitment, or deportation.
- **Beyond a reasonable doubt ($p^*_{\mathrm{PK}} \ge 0.91$ `[Assumed]`):** Mandated in criminal prosecutions under
  William Blackstone's ratio (*Commentaries*, 1769 `[Literature]`) that *"it is better that ten guilty
  persons escape, than that one innocent suffer"* ($C_{\mathrm{FP}} = 10\,L_{\mathrm{FN}} \implies p^*_{\mathrm{PK}} = 10/(10+1) \approx 0.909$ `[Assumed]`).

Yet a high probability $p > p^*$ is never sufficient on its own. In L. Jonathan Cohen's **Gatecrasher
Paradox** (*The Probable and the Provable*, 1977 `[Literature]`), an arena promoter proves that $1{,}000$
people sat in the stands while only $499$ tickets were sold, so $501$ spectators crashed the gate. Picked
at random, any spectator has probability $p = 0.501 > 0.50$ of being a gatecrasher `[Assumed]`, yet no
court permits recovery on naked statistical class frequency. The reason is **Keynes's weight of evidence**
$V(n_{\mathrm{eff}}) = n_{\mathrm{eff}}/(n_{\mathrm{eff}} + n_0)$ (`epistemic.rs::EpistemicMarginState::keynes_weight_from_n_eff`):
without case-specific evidence linking the defendant to the act, individualized $n_{\mathrm{eff}} \to 0$,
$V \to 0$, and condemning an individual on group prevalence violates both epistemic weight and the
Chouldechova disparity bound (`governance.rs::EmpiricalChouldechovaAudit`).

## 2. Tort liability, Learned Hand, and the two-circuit remedy

In *United States v. Carroll Towing Co.* (1947) `[Literature]`, Judge Learned Hand defined negligence by
the inequality $B < P L$: an actor is negligent if the burden of adequate precaution $B$ is less than the
probability of harm $P$ multiplied by the gravity of resulting injury $L$. Rearranged as $P > B/L$, Hand's
formula is the cost–loss threshold of §1 at $\eta = 1$ with $C = B$, that is
$p^*_{\mathrm{CL}} = C/(\eta L) = B/L$ `[Assumed]` — a duty-of-care statement about a protection budget, not
an expected-regret comparison of two classification errors.

In corporate and infrastructural practice, however, naive application of $B < PL$ collapses in two ways:
1. **Convexity blindness (`T4Formalization`):** Evaluating $PL$ at the point expectation ignores Jensen's
   convexity gap $\Delta_J = \mathbb{E}[\phi(X)] - \phi(\mathbb{E}[X])$ (`vital.rs::VitalMarginState::from_heavy_tails_and_asymmetry`)
   and heavy-tailed catastrophic loss ($\hat{\alpha} \le 2$).
2. **Limited-liability truncation (`T7Liability`):** As Robert Innes (1990) proved `[Literature]`, when a
   firm's maximum collectible liability is capped at its equity $W < L$, the private expected cost of
   recklessness is $P\min(W, L) \ll PL$, creating a rational incentive to strip buffers and externalise
   tail ruin onto non-consenting citizens (`governance.rs::ContractAndLiabilityAudit`).

Stochology resolves the tension between Bernard Williams's **moral luck** ([Ethics and the Imposition of Risk](19-ethics-and-the-imposition-of-risk.md) §4)
and Ernest Sosa's **telic evaluation** (`governance.rs::TelicGrade`) by splitting legal remedies into
**two orthogonal circuits**:
- **Circuit 1 (Strict no-fault victim repair):** Whenever an activity imposes asymmetric risk on third
  parties, harmed citizens are compensated immediately from an *ex-ante* capitalised reserve fund even
  when the operator's shot was `AdroitExAnteUnluckyOutcome`;
- **Circuit 2 (Telic sanction and escrow clawback):** Punitive damages, bonus clawbacks from long-horizon
  escrow, and license revocation attach strictly to `LuckyAccurateOnly` (reckless buffer stripping that
  got lucky with the weather) and `InaptAndInaccurate`, blocking the $13.6\times$ scapegoating inversion
  (`governance.rs::ContractAndLiabilityAudit`, $85.0\% / 6.25\%$ `[Simulated]`)
  that blames frontline workers for executive undersizing.

## 3. Constitutionalism as an institutional circuit-breaker

As Jon Elster argued in *Ulysses and the Sirens* (1979) `[Literature]`, a constitution is a pre-commitment
device by which a body politic binds its own future impulses during moments of panic or capture:

- **Prohibition of *ex post facto* laws and bills of attainder** $\Longleftrightarrow$ pre-outcome
  cryptographic sealing in `governance.rs::TemporalVault::seal`, forbidding the state from changing the
  acceptance rule after the trajectory is observed;
- **Separation of powers and bicameral/judicial vetoes** $\Longleftrightarrow$ the List–Pettit conjunctive
  gate `governance.rs::GovernanceState::evaluate_list_pettit_votes`;
- **Statutory whistleblower protection and the right to refuse unsafe work** $\Longleftrightarrow$ a legal
  mandate that the testimonial discount on the algedonic channel be fixed at $\delta_{\mathrm{cred}} = 0$
  (`governance.rs::AlgedonicChannel`).

## 4. Statute as a circuit breaker

The two preceding sections treated liability and constitutionalism as circuits. A third layer sits between them, and
it is the one that governs automated decisions in practice: the statutory duty.

Data-protection and artificial-intelligence statutes do not, in the main, prescribe the model. They prescribe
*obligations of process* — a lawful basis for processing, a duty to inform the person affected, a right to human
intervention in decisions with significant effects, a duty to document the logic at a level the person can contest.
Read as engineering, those obligations are the legal form of the ex-ante gate of
[Chapter 48](48-reactive-worlds-performativity-goodhart.md): they require that the premises of the decision be
declared before the decision is executed, and that a route to a human exist and be usable afterwards.

That reading makes three requirements auditable, and each maps onto an artifact this treatise already has.

1. **A declared envelope is a legal artifact, not documentation hygiene.** A model that reports a confidence without a
   statement of the population, the period and the conditions under which it was measured has not documented its
   limits; it has documented its optimism. The duty to inform is satisfied by the envelope, and by nothing else.
2. **Human review is a half-open state and must have a latency.** The right to intervention is real only if the
   review can complete inside the window in which the decision is still reversible — the
   $\mathrm{TTS}_{\mathrm{survival}} \ge \mathrm{TTR}_{\mathrm{recovery}}$ condition of
   [Chapter 47](47-irreversible-time-vital-margin.md) in the register of rights. A review that takes longer than the
   effect it reviews is a review of history.
3. **The standard of proof is the tort standard applied to the artifact.** Under the Learned Hand formulation of §2,
   care is adequate when its burden falls below the expected loss; for an automated decision that means the cost of
   measuring the error rate of the deployed model at the deployed population, weighed against the harm of the errors
   it makes. A model card that reports aggregate accuracy is not evidence at that standard, because aggregate accuracy
   is the one quantity the harmed subgroup does not appear in.

The institutional point is that a statute with a clock is strictly stronger than a professional norm, and the reason is
the one this layer has been building since §1: a norm has no admissible evidence standard attached to it, so it cannot
be failed. An obligation with a deadline, a named artifact and a right of contestation can be failed — which is
precisely what makes it a circuit rather than an aspiration.

A closing asymmetry deserves stating. The operator who follows the statute and still causes harm is in the position of
the adroit agent of [Chapter 49](49-moral-luck-apt-agency-institutions.md): ex-ante sound, ex-post unlucky. The
operator who cannot produce the envelope, the review latency and the subgroup error rate is not unlucky; they are
undocumented, and the difference between those two conditions is what the statutory layer exists to make visible.

## 5. The three collapses in the legal order

1. **Collapse Proposition 1 ($\neg S_{\mathrm{epi}}$ — Actuarial Profiling and Naked Priors):**
   Convicting or detaining individuals on group base rates with near-zero individualized weight $V$ while
   ignoring selection bias in arrest archives (`toronto_arrests.csv`).
2. **Collapse Proposition 2 ($\neg S_{\mathrm{vital}}$ — Irreversible Harm Before Injunctive Relief):**
   Allowing docket congestion ($\rho \to 1$) to delay preliminary injunctions (*periculum in mora*) until
   the ecological or bodily harm is already irreversible ($\mathrm{TTS}_{\mathrm{survival}} < \mathrm{TTR}_{\mathrm{recovery}}$).
3. **Collapse Proposition 3 ($\neg S_{\mathrm{gov}}$ — Judgment-Proof Shell Companies and Scapegoating):**
   Capping corporate downside via limited-liability shells while prosecuting the night-shift operator when
   the unbuffered system fails.

## Bridge

Layer 32 (`multilayer.rs::KnowledgeOrder`, `KnowledgeOrder::L32EvidentiaryTortConstitutional`, `{T1Framing, T4Formalization, T7Liability}`) hands off editorially to [Chapter 83 — Layer 33: Distributed Consensus, Fault-Tolerant Computing & SRE Order](83-layer-distributed-consensus-sre-order.md) (`KnowledgeOrder::L33DistributedConsensusSre`, Stratum VII, `{T5Temporal, T6Performative, T7Liability}`). Layer 33 is **not** a child of Layer 32 in $\mathcal{G}_1$ — its parents are [Layer 02](52-layer-meso-buffered-queue-dispatch-order.md), [Layer 03](53-layer-macro-coupled-network-contagion-order.md) and [Layer 10](60-layer-cryptographic-zero-knowledge-adversarial-order.md) — so the two orders are joined through shared ancestors only, and `multilayer.rs::MultilayerHomomorphism::evaluate` carries no margin of Layer 32 into Layer 33's cone. The shared ancestor that carries the epistemic axis ($S_{\mathrm{epi}}$) that Layer 33's `{T5, T6, T7}` triple omits is Layer 10 (`{T2Severity, T6Performative, T7Liability}`), which Layer 32 reaches through Layer 30 and Layer 27.
