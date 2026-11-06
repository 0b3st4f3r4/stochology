# Chapter 65 — Layer 15: Stratigraphy, Paleobiology, Deep Time & Mass-Extinction Order

> Layer 15 (`multilayer.rs::KnowledgeOrder`, `KnowledgeOrder::L15StratigraphyPaleobiologyDeepTime`) of Part V · Stratum III (`multilayer.rs::KnowledgeStratum`, `KnowledgeStratum::S3LivingEvolutionary`) · Unique Guard Combination `{T1Framing, T3Archive, T5Temporal}` · Signature `(2, 1, 0)` (`multilayer.rs::TriadicExposureRegime`, `TriadicExposureRegime::GovernanceBlindSpot`). It builds on [Layer 04](54-layer-geophysical-chaos-data-assimilation-order.md), [Layer 12](62-layer-morphogenetic-evolutionary-developmental-order.md), [Layer 14](64-layer-taxonomy-systematics-ontology-order.md) in $\mathcal{G}_1$ and maps into $\mathcal{G}_2$ via [`notes/The Multilayer Graph of Knowledge and Aim.md`](../notes/The%20Multilayer%20Graph%20of%20Knowledge%20and%20Aim.md).

## By the end of this chapter you can
1. Locate **Stratigraphy, Paleobiology, Deep Time & Mass Extinctions (`L15`)** at the unique 3-combination `{T1Framing, T3Archive, T5Temporal}` completing Stratum III in $\mathcal{G}_1 \leftrightarrow \mathcal{G}_2$ (`multilayer.rs::KnowledgeStratum`, `KnowledgeStratum::S3LivingEvolutionary`).
2. Formalize the **Signor–Lipps Stratigraphic Truncation Effect** (`T3Archive`, `T5Temporal`), where Poisson preservation rates $\lambda_i$ make an instantaneous catastrophic mass extinction appear gradual, alongside **Alvarez Catastrophic Framing** (`T1Framing`).
3. Prove by the **Disciplinary Incompleteness Theorem** why a `(2, 1, 0)` deep-time Earth-system reconstruction lacking $S_{\mathrm{gov}}$ (`T6`, `T7`) suffers **Collapse Proposition 3 (`synthesis.rs::CollapseProposition`, `CollapseProposition::Proposition3CynicalCapture`)** when fossil-record sampling uncertainties are exploited to discount anthropogenic extinction rates and biospheric recovery basins.

## 1. The problem: Deep Time and the Uniformitarian Blind Spot

Paleobiology reconstructs the history of macroevolutionary lineages across geological epochs. In Charles Lyell's
foundational *Principles of Geology* (1830), strict uniformitarianism (*natura non facit saltus*) asserted that past
Earth changes resulted solely from continuous, gradual forces operating at observable modern rates. For over a century,
this dogmatic framing (`T1Framing`) ruled out catastrophic cosmic impacts, planetary phase transitions, and rapid
biospheric collapses as unscientific speculation.

The uniformitarian consensus was shattered in 1980 by Luis and Walter Alvarez, Frank Asaro, and Helen Michel. Discovery
of global iridium anomalies at the Cretaceous–Paleogene (K–Pg) stratigraphic boundary proved that a 10-kilometer
asteroid impact terminated the non-avian dinosaurs and over 70% of marine species within an ecological instant. This
revolution demonstrated that the geological record records not merely linear equilibrium, but rare, heavy-tailed
planetary catastrophes operating on survival horizons $\tau_{\mathrm{event}} \ll \tau_{\mathrm{recovery}}$
(`T5Temporal`).

Yet decoding this deep-time record requires reading an incomplete, taphonomically filtered archive: sedimentary basins
where rock deposition is punctuated by vast unconformities and hiatuses.

## 2. The Signor–Lipps Truncation Effect and Extinction Pulses (`T1`, `T3`, `T5`)

In 1982, Philip Signor and Jere Lipps formalized the fundamental observational distortion of paleobiology. Because
fossilization is an exceptionally rare event, the stratigraphic record truncates the observed lifespans of extinct
organisms:

Suppose $S$ biological taxa go extinct simultaneously at true temporal horizon $t_{\mathrm{ext}}$ (`T5Temporal`). In the
sedimentary column, each taxon $i$ is preserved according to an independent Poisson process with stratigraphic
preservation rate $\lambda_i > 0$ (`T3Archive`). The stratigraphically highest observed fossil occurrence
$t_i^{\max} \le t_{\mathrm{ext}}$ of taxon $i$ exhibits a backward gap:

$$G_i = t_{\mathrm{ext}} - t_i^{\max} \sim \mathrm{Exp}(\lambda_i).$$

For abundant taxa (large $\lambda_i$), $t_i^{\max}$ lies close to $t_{\mathrm{ext}}$. For rare taxa (small $\lambda_i$,
connected to `epistemic.rs::EpistemicMarginState::from_archive_clusters`), the last observed fossil appearance occurs
meters or millions of years below the true extinction boundary!

As an inevitable mathematical artifact, an abrupt, catastrophic step-function mass extinction appears in the raw
sedimentary archive as a gradual, protracted decline. David Strauss and Peter Sadler (1989) developed non-parametric
confidence bounds to restore severity (`T2Severity`):

$$\hat{t}_{\mathrm{ext}}(1 - \alpha) = t_i^{\max} + \frac{t_i^{\max} - t_i^{\min}}{m - 1}\bigl(\alpha^{-1/(m - 1)} - 1\bigr),$$

where $m$ is the count of distinct fossil horizons. Correcting for Signor–Lipps truncation reveals that the "Big Five"
mass extinctions identified by Jack Sepkoski and David Raup (1982) represent genuine heavy-tailed shock pulses
(`vital.rs::VitalMarginState`) rather than peaceful background turnover.

## 3. Planetary Basin Recovery Times and Why `(2, 1, 0)` Requires Governance Coupling

The deep-time archive reveals the physical scale of the Canguilhem–Holling recovery basin
(`vital.rs::VitalMarginState::from_queue_basin_and_survival`). Following the End-Permian extinction (the "Great Dying",
252 million years ago), which erased 95% of marine life, the global carbon cycle and marine reef systems remained
unstable for:

$$\mathrm{TTR}_{\mathrm{recovery}} \sim 5\text{--}10\text{ million years}.$$

Prolonged post-extinction ecological stress ("Lilliput effect", where surviving taxa exhibit stunted body sizes) proves
that when environmental destruction breaches resilience thresholds, vital margins cannot be restored on human political
timescales.

Layer 15 possesses the triadic occupancy signature `(2, 1, 0)` (`multilayer.rs::TriadicExposureRegime`,
`TriadicExposureRegime::GovernanceBlindSpot`). It unites catastrophic framing (`T1Framing`), stratigraphic archive
estimation (`T3Archive`), and irreversible deep-time horizons (`T5Temporal`), but contains zero local governance sites
(`k_gov = 0`).

When paleobiological science is deployed without institutional accountability (`T6Performative`, `T7Liability`), it
triggers **Collapse Proposition 3 (`synthesis.rs::CollapseProposition`,
`CollapseProposition::Proposition3CynicalCapture`)**:
- Extractive industrial lobbies exploit Signor–Lipps range uncertainties ("scientists cannot definitively prove past
   extinctions happened quickly") to claim that current industrial extinction rates are within natural historical
   variation.
- Neoclassical economic models apply positive financial discount rates ($r \sim 5\%/\text{year}$) to planetary
   ecologies, mathematically setting the present value of the biosphere past 50 years to zero and discarding the multi-
   million-year recovery basin $\mathrm{TTR} \gg \mathrm{TTS}$.

Deep-time Earth systems achieve ethical aim only when bound to constitutional intergenerational protection (`L32`) and
planetary commons stewardship (`L35`).

## Reproduce

```bash
cargo run --release --bin stochology -- --chapter 5
cargo run --release --bin stochology -- --chapter 6
```

## Bridge

Layer 15 (`multilayer.rs::KnowledgeOrder`, `KnowledgeOrder::L15StratigraphyPaleobiologyDeepTime`, `{T1Framing,
T3Archive, T5Temporal}`) propagates its local and ancestor-cone margins through
`multilayer.rs::MultilayerHomomorphism::evaluate` into [Chapter 66 — Layer 16: Pharmacological, Clinical Diagnostic &
Acute Therapeutic Order](66-layer-pharmacological-clinical-diagnostic-order.md)
(`KnowledgeOrder::L16PharmacologicalClinicalDiagnostic`, Stratum IV, `{T2Severity, T4Formalization, T5Temporal}`).
