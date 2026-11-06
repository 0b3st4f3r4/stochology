# Chapter 72 — Layer 22: Philological, Epigraphic & Stemmatic Textual-Criticism Order

> Layer 22 (`multilayer.rs::KnowledgeOrder`, `KnowledgeOrder::L22PhilologicalEpigraphicTextual`) of Part V · Stratum V (`multilayer.rs::KnowledgeStratum`, `KnowledgeStratum::S5SemioticArchival`) · Unique Guard Combination `{T2Severity, T3Archive, T4Formalization}` · Signature `(2, 1, 0)` (`multilayer.rs::TriadicExposureRegime`, `TriadicExposureRegime::GovernanceBlindSpot`). It builds on [Layer 09](59-layer-experimental-sampling-causal-order.md), [Layer 14](64-layer-taxonomy-systematics-ontology-order.md), [Layer 21](71-layer-linguistic-evidential-pragmatic-order.md) in $\mathcal{G}_1$ and maps into $\mathcal{G}_2$ via [`notes/The Multilayer Graph of Knowledge and Aim.md`](../notes/The%20Multilayer%20Graph%20of%20Knowledge%20and%20Aim.md).

## By the end of this chapter you can

1. Compute the effective witness count of a manuscript tradition using Lachmann and Maas's stemmatics,
   and show its identity with the Campbell–Moulton cluster design effect $\mathrm{DEFF}$.
2. Read the Leiden Convention for fractured stone inscriptions (`[...]` and *cruces desperationis*
   $\dagger\dots\dagger$) as material Manski partial-identification bounds.
3. Classify shared manuscript corruptions (*errores coniunctivi* and *errores separativi*) as Smoking Gun
   and Hoop tests in Bayesian process tracing.

## 1. Lachmann's *stemma codicum* is the cluster design effect

Before the nineteenth century, editors establishing an ancient or medieval text routinely followed the
majority rule (*codices plurimi*): if nine surviving Renaissance manuscripts carried reading $A$ and one
tenth-century parchment codex carried reading $B$, reading $A$ won nine to one.

Karl Lachmann (1850) and Paul Maas (*Textkritik*, 1927 `[Literature]`) overturned that practice by proving
that manuscript witnesses are not independent draws. If the nine Renaissance manuscripts ($\mathcal{B} = \{B_1,\dots,B_9\}$)
were all copied from a single lost fifteenth-century exemplar $\beta$, their intra-cluster correlation on
inherited readings is $\rho_I \to 1$. Applying the Campbell–Moulton design-effect formula of
[Chapter 45](45-world-to-archive-big-data-paradox.md) (`epistemic.rs::from_archive_clusters`) to cluster
$\mathcal{B}$ of size $m_{\beta} = 9$ gives

$$\mathrm{DEFF}_{\beta} = 1 + (m_{\beta} - 1)\rho_{I} = 9, \qquad n_{\mathrm{eff},\beta} = \frac{m_{\beta}}{\mathrm{DEFF}_{\beta}} = 1 \qquad \text{\texttt{[Assumed]}}.$$

Nine dependent copies carry the evidential weight (`epistemic.rs::keynes_weight_from_n_eff`) of **one**
witness. Maas's rule ***eliminatio codicum descriptorum*** (setting aside any manuscript proved to be a
copy of an extant exemplar) and the philological maxim *ponderanda sunt testimonia, non numeranda*
(witnesses must be weighed, not counted) are the exact documentary formulation of dividing nominal
archive volume by $\mathrm{DEFF}$.

## 2. Stemmatic process tracing and *lectio difficilior potior*

How does the philologist prove that two manuscripts belong to the same cluster? Maas formalised two
diagnostic tests that map 1-to-1 onto Bennett and Checkel's process-tracing matrix in
[Chapter 50](50-methodological-families-reflexivity.md) (`synthesis.rs::ProcessTracingInference`):

- **Conjunctive error (*error coniunctivus* $\leftrightarrow$ `SmokingGun`):** A peculiar, non-trivial
  corruption shared by manuscripts $B_1$ and $B_2$ that two scribes could not have committed
  independently by chance ($\text{false-positive rate } \mathrm{FPR} \ll 1$, so $\mathrm{LR}^+ \gg 1$).
  Observing it drives the posterior probability of a shared hyparchetype $\beta$ close to one.
- **Separative error (*error separativus* $\leftrightarrow$ `Hoop`):** A major omission in $B_1$ that a
  medieval scribe could not have repaired by conjecture, yet appears intact in $B_2$ ($\text{sensitivity} \to 1$,
  so $\mathrm{LR}^- \ll 1$). Its absence in $B_2$ refutes the hypothesis that $B_2$ was copied from $B_1$.

When two independent branches disagree and neither is eliminated by stemmatics, internal criticism applies
Jean Leclerc's rule ***lectio difficilior potior*** `[Literature]`: the harder, rarer morphological or
lexical reading is more likely to be original than the smooth, commonplace variant (*lectio facilior*),
because copyists systematically normalise unfamiliar outliers toward the Gaussian average of their own
idiom — the scribal ancestor of abusive outlier trimming (`vital.rs::from_heavy_tails_and_asymmetry`).

## 3. Epigraphic *lacunae* as Manski bounds carved in stone

Unlike manuscripts copied across generations, an ancient inscription (*titulus*) on marble or bronze is a
direct physical survival from antiquity — but frequently arrives fractured, with missing letters or lines
(**lacunae**).

Under the international **Leiden Convention** (1931) `[Literature]`, epigraphers are forbidden to print a
conjectural restoration as if it were on the stone. Because ancient inscriptions (especially Greek
*stoichedon* grids) have a physically measurable character width and line length, a fracture of width
$W_{\mathrm{gap}}$ bounds the missing numeral or word strictly within a finite support $[y_{\min}, y_{\max}]$
(`epistemic.rs::from_ate_bounds`):

- **`[...]` (Square brackets):** Marks the exact physical extent of the lacuna and bounds the admissible
  set of restorations compatible with the surviving syntax and stone geometry.
- **$\dagger\dots\dagger$ (*Cruces desperationis*):** Placed around a corrupted passage when no proposed
  emendation passes severe testing (`epistemic.rs::passes_severe_boundary`) — an explicit quarantine
  verdict (`Unparsable`) that prevents subsequent historians from building circular theories on a
  fabricated conjecture.

## 4. The three collapses in the philological and epigraphic order

1. **Collapse Proposition 1 ($\neg S_{\mathrm{epi}}$ — *Codices Plurimi* and Silent Conjecture):**
   Counting dependent copies as independent evidence ($\mathrm{DEFF}$ ignored) or printing an unbracketed
   conjecture over a stone lacuna.
2. **Collapse Proposition 2 ($\neg S_{\mathrm{vital}}$ — The *Codex Unicus* Without Redundancy):**
   Holding an entire textual heritage in a single uncopied manuscript (*codex unicus*) at an unbuffered
   library until a single fire (such as the Cotton Library fire of 1731 `[Literature]`) crosses the
   absorbing ruin barrier.
3. **Collapse Proposition 3 ($\neg S_{\mathrm{gov}}$ — *Textus Receptus* Dogmatism and Forgery):**
   Canonising a corrupted printed edition by institutional decree while suppressing critical apparatus
   warnings ($\delta_{\mathrm{cred}} \to 1$).

## Bridge

Layer 22 (`multilayer.rs::KnowledgeOrder`, `KnowledgeOrder::L22PhilologicalEpigraphicTextual`, `{T2Severity, T3Archive, T4Formalization}`) propagates its local and ancestor-cone margins through `multilayer.rs::MultilayerHomomorphism::evaluate` into [Chapter 73 — Layer 23: Taphonomic, Archaeological & Bitemporal Historiographical Order](73-layer-taphonomic-archaeological-historiographical-order.md) (`KnowledgeOrder::L23TaphonomicHistoriographical`, Stratum V, `{T3Archive, T5Temporal, T7Liability}`).
