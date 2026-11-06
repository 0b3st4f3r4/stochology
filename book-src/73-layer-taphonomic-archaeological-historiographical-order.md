# Chapter 73 — Layer 23: Taphonomic, Archaeological & Bitemporal Historiographical Order

> Layer 23 (`multilayer.rs::KnowledgeOrder`, `KnowledgeOrder::L23TaphonomicHistoriographical`) of Part V · Stratum V (`multilayer.rs::KnowledgeStratum`, `KnowledgeStratum::S5SemioticArchival`) · Unique Guard Combination `{T3Archive, T5Temporal, T7Liability}` · Signature `(1, 1, 1)` (`multilayer.rs::TriadicExposureRegime`, `TriadicExposureRegime::TransTriadicKeystone`). It builds on [Layer 15](65-layer-stratigraphy-paleobiology-deep-time-order.md), [Layer 21](71-layer-linguistic-evidential-pragmatic-order.md), [Layer 22](72-layer-philological-epigraphic-textual-order.md) in $\mathcal{G}_1$ and maps into $\mathcal{G}_2$ via [`notes/The Multilayer Graph of Knowledge and Aim.md`](../notes/The%20Multilayer%20Graph%20of%20Knowledge%20and%20Aim.md).

## By the end of this chapter you can

1. Explain how physical decay laws (taphonomy) and social recording thresholds generate a non-zero
   selection correlation $\rho_{R,Y} \neq 0$ across the material and documentary past.
2. Map Michel-Rolph Trouillot's four moments of historical silencing onto the four archive operators of
   this framework.
3. Distinguish Marc Bloch's *voluntary testimonies* from *involuntary traces*, and say why the first is
   subject to Goodhart's Law while the second requires taphonomic correction.

## 1. Taphonomy as the physical engine of $\rho_{R,Y} \neq 0$

In 1940 the palaeontologist Ivan Efremov coined the term **taphonomy** (*táphos*, burial + *nómos*, law)
for the study of the transition of organic remains from the biosphere into the lithosphere `[Literature]`,
and Michael Brian Schiffer extended the discipline to archaeological formation processes (`C-transforms`
and `N-transforms` `[Literature]`).

Every material archive is filtered by the second law of thermodynamics ([Thermodynamics and Entropy](22-thermodynamics-and-entropy.md)).
Monumental state temples, royal decrees on stone stelae, bronze weaponry, and fired tax tablets possess
high activation barriers against decay ($R_i = 1$). Conversely, the dwellings, textiles, baskets, wood
tools, and oral contracts of enslaved workers, pastoralists, and forest communities decay rapidly in
acidic soil or humid climates ($R_i \to 0$). Because social hierarchy directly governs the durability of
the medium in which a life is materialised, the correlation $\rho_{R,Y}$ between *being preserved in the
archaeological record* ($R_i$) and *holding elite wealth or state power* ($Y_i$) is structurally positive.

By Meng's identity in `epistemic.rs::from_meng_big_data`,

$$\bar{Y}_{\mathrm{excavated}} - \bar{Y}_{\mathrm{historical}} = \sigma_{Y}\,\rho_{R,Y}\sqrt{\frac{1 - f}{f}} \qquad \text{\texttt{[Assumed]}}.$$

Digitising ten million potsherds from royal capitals without correcting for the unpreserved fraction
$1 - f$ does not converge to social history; it narrows the confidence interval around a taphonomic
illusion.

## 2. Trouillot's four silences and the four archive operators

In *Silencing the Past* (1995) `[Literature]`, the anthropologist Michel-Rolph Trouillot showed that
historical silence is not a passive gap waiting to be filled by more documents; it is actively produced
at four distinct sites of the mediation circuit, and each site corresponds to a specific operator in
`epistemic.rs` and `governance.rs`:

1. **The moment of fact creation (*the making of sources*):** Who is granted a category and an entry when
   events occur, and who dies or labours unrecorded. This is the selection correlation $\rho_{R,Y}$ and
   coverage fraction $f$ bounded by `epistemic.rs::from_meng_big_data` and Manski's worst-case interval
   `epistemic.rs::from_ate_bounds`.
2. **The moment of fact assembly (*the making of archives*):** Which records are gathered into official
   repositories and which are left outside every list. When two partial registries exist (such as port
   manifests and parish registers), the completely unrecorded population $n_{00}$ is bounded from below
   by Anne Chao's capture-recapture estimator $\hat{n}_{00} = f_1^2 / (2f_2)$ in
   `epistemic.rs::from_capture_recapture`.
3. **The moment of fact retrieval (*the making of narratives*):** How historians weight surviving boxes
   and select specifications. Unequal fund weighting inflates the Kish design effect in
   `epistemic.rs::from_kish_strata`, while ad-hoc narrative rescue is audited by Lakatos's progressive
   shift ratio `epistemic.rs::lakatos_shift_ratio`.
4. **The moment of retrospective significance (*the making of history in the final instance*):** Judging
   past actors by knowledge that arrived only after their decision, or retroactively purging a ledger.
   This is prevented by bitemporal vintage accounting (`epistemic.rs::bitemporal_view_at`) and
   pre-outcome cryptographic sealing (`governance.rs::TemporalVault::seal`).

## 3. Voluntary chronicles versus involuntary traces

In *The Historian's Craft* (*Apologie pour l'histoire*, 1949 `[Literature]`), Marc Bloch drew the
foundational methodological distinction between **voluntary testimonies** (chronicles, memoirs, official
bulletins written *in order* to persuade contemporaries or posterity) and **involuntary witnesses**
(*témoins malgré eux*: garbage middens, pollen strata, price ledgers, wear patterns on tools, which
were never intended as historical arguments).

In the terms of [Chapter 48](48-reactive-worlds-performativity-goodhart.md), voluntary chronicles are
**performative and Goodhart-captured** (`governance.rs::GoodhartVariant`): once a ruler or agency knows
that a chronicle is the metric by which they will be judged, the chronicle reorganises itself around
reputational gain. Involuntary traces escape adversarial gaming (`T6Performative`), but remain subject
to physical taphonomic filtering (`T3Archive`). Sound historical inference therefore requires triangulating
both: auditing voluntary texts for incentive distortion while bounding involuntary traces for preservation
bias.

## 4. The three collapses in the historiographical order

1. **Collapse Proposition 1 ($\neg S_{\mathrm{epi}}$ — Positivist Archive Fetishism):** Treating *"what
   is not in the documents did not happen"* as an axiom while ignoring taphonomic decay and Trouillot's
   first two silences ($1 - f \to 1$, $\hat{n}_{00} \gg 0$).
2. **Collapse Proposition 2 ($\neg S_{\mathrm{vital}}$ — Slow-Moving Ecological Debt):** Reading only a
   short *événementielle* window while Braudel's *longue durée* `[Literature]` — soil salinisation in
   Mesopotamia, deforestation, aquifer depletion — erodes the resilience basin $D_t/D_0 \to 0$.
3. **Collapse Proposition 3 ($\neg S_{\mathrm{gov}}$ — *Damnatio Memoriae* and Hindsight Scapegoating):**
   Rewriting archives *ex post* to erase warnings that were discounted at the time, or judging past
   operators on post-revision data (`epistemic.rs::bitemporal_view_at` violated).

## Bridge

Layer 23 (`multilayer.rs::KnowledgeOrder`, `KnowledgeOrder::L23TaphonomicHistoriographical`, `{T3Archive, T5Temporal, T7Liability}`) propagates its local and ancestor-cone margins through `multilayer.rs::MultilayerHomomorphism::evaluate` into [Chapter 74 — Layer 24: Critical Archival Studies, Transitional Justice & Subaltern Testimony Order](74-layer-critical-archival-transitional-justice-order.md) (`KnowledgeOrder::L24CriticalArchivalTransitionalJustice`, Stratum V, `{T1Framing, T3Archive, T7Liability}`).
