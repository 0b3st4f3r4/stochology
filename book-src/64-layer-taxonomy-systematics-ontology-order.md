# Chapter 64 — Layer 14: Taxonomy, Phylogenetic Systematics & Formal Ontology Order

> Layer 14 (`multilayer.rs::KnowledgeOrder`, `KnowledgeOrder::L14TaxonomySystematicsOntology`) of Part V · Stratum III (`multilayer.rs::KnowledgeStratum`, `KnowledgeStratum::S3LivingEvolutionary`) · Unique Guard Combination `{T1Framing, T3Archive, T4Formalization}` · Signature `(2, 1, 0)` (`multilayer.rs::TriadicExposureRegime`, `TriadicExposureRegime::GovernanceBlindSpot`). It builds on [Layer 06](56-layer-formal-computable-type-order.md), [Layer 12](62-layer-morphogenetic-evolutionary-developmental-order.md) in $\mathcal{G}_1$ and maps into $\mathcal{G}_2$ via [`notes/The Multilayer Graph of Knowledge and Aim.md`](../notes/The%20Multilayer%20Graph%20of%20Knowledge%20and%20Aim.md).

## By the end of this chapter you can
1. Locate **Taxonomy, Phylogenetic Systematics & Formal Ontology (`L14`)** at the unique 3-combination `{T1Framing, T3Archive, T4Formalization}` in Stratum III (`multilayer.rs::KnowledgeStratum`, `KnowledgeStratum::S3LivingEvolutionary`).
2. Connect **Hennigian Cladistics (Synapomorphy vs. Homoplasy)** (`T1Framing`), **Chao Unseen-Species Estimation $\hat{n}_{00} = f_1^2/(2f_2)$** (`T3Archive`, `epistemic.rs::EpistemicMarginState::from_archive_clusters`), and **Allometric Scaling Laws / Dialectical Species Penumbra** (`T4Formalization`, `vital.rs::VitalMarginState`).
3. Prove via **Bowker & Star's Infrastructural Torque** why a `(2, 1, 0)` classification ontology lacking $S_{\mathrm{gov}}$ (`T6`, `T7`) triggers **Collapse Proposition 3 (`synthesis.rs::CollapseProposition`, `CollapseProposition::Proposition3CynicalCapture`)** when diagnostic or administrative categories are gamed for commercial or bureaucratic advantage.

## 1. The problem: The Inescapable Ontology Before Measurement

Every empirical science begins not with numbers, but with categories. Long before an observer can tabulate counts,
estimate treatment effects, or compute variances, they must decide which entities belong together in an equivalence
class. In Carl Linnaeus's *Systema Naturae* (1735), Willi Hennig's *Phylogenetic Systematics* (1950), the Gene
Ontology (GO), and Geoffrey Bowker and Susan Leigh Star's *Sorting Things Out* (1999), classification is revealed
not as a passive clerical catalog, but as an active intervention that carves the state space into admissible and
inadmissible entities (`T1Framing`).

The illusion of the neutral database ignores the reality that any taxonomic boundary embeds normative assumptions.
Grouping biological taxa, clinical syndromes, or social identities into discrete buckets generates what Bowker and
Star term **infrastructural torque**: the structural friction experienced by real-world entities that do not fit
neatly into standardized administrative boxes. When taxonomic schemas ignore the dialectical penumbra of nature,
statistical counts become artefacts of classification rules rather than features of the world.

## 2. Cladistic Framing, Chao's Unseen Taxa, and Allometric Invariants (`T1`, `T3`, `T4`)

A rigorous ontological architecture coordinates three sites:
1. **`T1Framing` (Monophyly vs. Convergent Homoplasy):** Hennigian cladistics insists that biological
   classification must mirror true evolutionary descent. Organisms are grouped strictly by shared derived
characteristics (synapomorphies), rejecting paraphyletic groupings based on ancestral traits (symplesiomorphies)
or polyphyletic groupings based on convergent evolution (homoplasies). Conflating the streamlined hydrodynamic
shape of sharks and dolphins as evidence of common family membership commits a Type III framing error
(`epistemic.rs::EpistemicMarginState::from_inquiry_framing`), misattributing independent selective adaptations
to shared phylogenetic ancestry.
2. **`T3Archive` (Anne Chao's Non-Parametric Unseen Species Lower Bound):** When field biologists or software
   auditors sample an ecological or error archive, rare taxa appear infrequently. If an archive records $f_1$
singleton species (observed exactly once) and $f_2$ doubleton species (observed twice), Anne Chao (1984) proved
that the minimal number of unobserved species lurking in the environment is:

$$\hat{n}_{00} = \frac{f_1^2}{2 f_2}$$

(`epistemic.rs::EpistemicMarginState::from_archive_clusters`). Assuming that an empirical survey has catalogued all
living biodiversity simply because the collection protocol terminated is an agnotological error that violates
Cromwell's rule (`epistemic.rs::EpistemicMarginState::from_inquiry_framing`).
3. **`T4Formalization` (Allometry and the Dialectical Penumbra):** Living organisms obey physical scaling
   constraints that cross taxonomic boundaries. In Max Kleiber's 3/4 allometric metabolic scaling law (1932), the
basal metabolic rate $B$ of organisms from unicellular microbes to blue whales scales with body mass $M$ across
twenty orders of magnitude as:

$$B = B_0 M^{3/4}.$$

West, Brown, and Enquist (WBE 1997) proved that this universal 3/4 exponent emerges from the fractal, space-
filling, area-preserving geometry of hydrodynamic vascular distribution networks. Yet real biological species defy
crisp Aristotelian boundary cuts: ring species (such as *Ensatina* salamanders), hybrid swarms, and prokaryotic
horizontal gene transfer inhabit Nicholas Georgescu-Roegen's dialectical penumbra where an organism is
simultaneously $A$ and $\neg A$ ($A \cap \neg A \ne \emptyset$, `vital.rs::VitalMarginState`).

## 3. Infrastructural Torque and Why `(2, 1, 0)` Requires Governance Coupling

Layer 14 has the triadic occupancy signature `(2, 1, 0)` (`multilayer.rs::TriadicExposureRegime`,
`TriadicExposureRegime::GovernanceBlindSpot`). It formalizes cladistic framing (`T1Framing`), unseen archival
estimation (`T3Archive`), and allometric boundary structures (`T4Formalization`), but possesses zero governance
sites (`k_gov = 0`).

When an ontology is deployed without institutional accountability (`T6Performative`, `T7Liability`), it falls
victim to **Collapse Proposition 3 (`synthesis.rs::CollapseProposition`,
`CollapseProposition::Proposition3CynicalCapture`)**:
- In the International Classification of Diseases (ICD), hospital billing departments perform strategic
   "upcoding": shifting patients from lower-paying diagnostic codes into higher-paying complex disease
classifications to extract insurance rents without improving clinical care.
- In colonial and state censuses, rigid racial or caste taxonomies erase mixed-heritage populations into
   residual "Other" categories, disenfranchising communities while reifying artificial administrative boundaries.
- In machine-learning risk scores, proprietary categorical taxonomies classify loan applicants or defendants
   into opaque risk brackets, shielding institutional decision-makers from tort liability while denying    subjects
the right to contest incorrect classifications (`T7Liability`).

Taxonomic schemas achieve ethical validity only when coupled to transparent audit trails, rights of contestation,
and the ancestors that supply the missing governance sites (`L02` for `T6Performative` and `T7Liability`, `L03` for `T6Performative`).

## Reproduce

```bash
cargo run --release --bin stochology -- --chapter 2
cargo run --release --bin stochology -- --chapter 3
```

## Bridge

Layer 14 (`multilayer.rs::KnowledgeOrder`, `KnowledgeOrder::L14TaxonomySystematicsOntology`, `{T1Framing,
T3Archive, T4Formalization}`) propagates its local and ancestor-cone margins through
`multilayer.rs::MultilayerHomomorphism::evaluate` into [Chapter 65 — Layer 15: Stratigraphy, Paleobiology, Deep
Time & Mass-Extinction Order](65-layer-stratigraphy-paleobiology-deep-time-order.md)
(`KnowledgeOrder::L15StratigraphyPaleobiologyDeepTime`, Stratum III, `{T1Framing, T3Archive, T5Temporal}`).
