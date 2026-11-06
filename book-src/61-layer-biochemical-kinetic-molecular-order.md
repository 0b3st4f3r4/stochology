# Chapter 61 — Layer 11: Biochemical Kinetics, Enzyme Queues & Molecular Proofreading Order

> Layer 11 (`multilayer.rs::KnowledgeOrder`, `KnowledgeOrder::L11BiochemicalKineticMolecular`) of Part V · Stratum III (`multilayer.rs::KnowledgeStratum`, `KnowledgeStratum::S3LivingEvolutionary`) · Unique Guard Combination `{T2Severity, T5Temporal, T6Performative}` · Signature `(1, 1, 1)` (`multilayer.rs::TriadicExposureRegime`, `TriadicExposureRegime::TransTriadicKeystone`). It builds on [Layer 01](51-layer-micro-kinetic-sensorimotor-order.md), [Layer 02](52-layer-meso-buffered-queue-dispatch-order.md), [Layer 08](58-layer-metrological-information-geometric-order.md) in $\mathcal{G}_1$ and maps into $\mathcal{G}_2$ via [`notes/The Multilayer Graph of Knowledge and Aim.md`](../notes/The%20Multilayer%20Graph%20of%20Knowledge%20and%20Aim.md).

## By the end of this chapter you can

1. Derive Hopfield–Ninio kinetic proofreading as a biochemical Ville *e*-process that trades metabolic
   energy and deliberation latency for an exponential reduction in false-positive rate.
2. Prove the exact algebraic identity between Michaelis–Menten enzymatic saturation and Kingman's
   heavy-traffic queue multiplier $\rho/(1-\rho)$.
3. Explain why conjugate buffer depletion (such as hepatic glutathione exhaustion) causes an abrupt
   regime switch from linear first-order clearance to divergent zero-order accumulation.

## 1. Kinetic proofreading as a molecular *e*-process

Every adaptive immune response begins with a recognition problem at extreme base-rate asymmetry. A T
cell scanning an antigen-presenting cell encounters $10^5$ to $10^6$ self-peptides for every $1$ to $10$
foreign agonist peptides `[Literature]`, yet the equilibrium dissociation constant $K_D = k_{\mathrm{off}}/k_{\mathrm{on}}$
of a foreign peptide differs from that of a self-peptide by only a factor of roughly $\theta \approx 10$
`[Literature]`. An equilibrium sensor that triggered on single-step occupancy would suffer a
false-positive rate of $\epsilon_1 \approx 1/\theta = 0.10$, unleashing fatal autoimmunity across the
host.

John Hopfield (1974), Jacques Ninio (1975), and Timothy McKeithan (1995) discovered how the cell solves
this `[Literature]`: the T-cell receptor (`TCR/CD3`) chains $N$ irreversible, $\text{ATP}$-consuming
phosphorylation steps (driven by the kinase $\text{Lck}$ and opposed by the phosphatase $\text{SHP-1}$)
before releasing the transcription factor $\text{NFAT}$. If the ligand unbinds before step $N$,
phosphatases reset the complex to zero. At each step $s \in \{1,\dots,N\}$, an agonist with off-rate
$k_{\mathrm{off,ag}}$ advances with probability $k_p/(k_p + k_{\mathrm{off,ag}})$, whereas a self-peptide
with $k_{\mathrm{off,self}} = \theta k_{\mathrm{off,ag}}$ advances with a smaller probability, yielding a
sequential likelihood ratio (*e*-value) $e_s > 1$ per step. After $N$ steps the false-positive rate
falls geometrically:

$$\epsilon_{N} = \epsilon_{1}^{N} \approx \theta^{-N} \qquad \text{\texttt{[Assumed]}},$$

reaching $10^{-6}$ at $N = 6$ `[Assumed]` at the cost of a mandatory deliberation delay
$\tau_{\mathrm{proof}} \approx N/k_p$. It is the exact molecular realisation of Ville's *e*-process and
Mayo's severity gate (`epistemic.rs::passes_severe_boundary`).

## 2. Michaelis–Menten saturation is Kingman's queue hyperbola

Consider a metabolic enzyme or hepatic clearance pathway that eliminates a substrate or toxin at the
Michaelis–Menten rate $v_{\mathrm{elim}}(C) = \frac{V_{\max} C}{K_m + C}$ `[Literature]` while the
substrate enters the bloodstream at constant rate $R_{\mathrm{in}} < V_{\max}$. Define the dimensionless
**metabolic utilisation** $\rho_{\mathrm{met}} \equiv R_{\mathrm{in}} / V_{\max} \in [0, 1)$. Setting
input $R_{\mathrm{in}}$ equal to clearance $v_{\mathrm{elim}}(C_{\mathrm{ss}})$ and solving for the
steady-state concentration $C_{\mathrm{ss}}$ gives

$$R_{\mathrm{in}} = \frac{V_{\max} C_{\mathrm{ss}}}{K_{m} + C_{\mathrm{ss}}} \;\Longleftrightarrow\; C_{\mathrm{ss}}(\rho_{\mathrm{met}}) = K_{m}\frac{\rho_{\mathrm{met}}}{1 - \rho_{\mathrm{met}}} \qquad \text{\texttt{[Assumed]}}.$$

Compare this with Kingman's heavy-traffic queue formula in `vital.rs::from_queue_basin_and_survival`. The
factor $\frac{\rho_{\mathrm{met}}}{1 - \rho_{\mathrm{met}}}$ is **algebraically identical**. At
$\rho_{\mathrm{met}} = 0.20$ the substrate sits at $0.25\,K_m$; at $\rho_{\mathrm{met}} = 0.50$ it sits at
$1.0\,K_m$; at $\rho_{\mathrm{met}} = 0.90$ it jumps to $9\,K_m$; and at $\rho_{\mathrm{met}} = 0.95$ it
explodes to $19\,K_m$ `[Assumed]`. At the same time, the local linearised relaxation rate
$k_{\mathrm{eff}}(\rho_{\mathrm{met}}) = \left.\frac{\mathrm{d}v_{\mathrm{elim}}}{\mathrm{d}C}\right|_{C_{\mathrm{ss}}} = \frac{V_{\max}}{K_m}(1 - \rho_{\mathrm{met}})^2$
vanishes as $\rho_{\mathrm{met}} \to 1$, so the physiological recovery time
$\mathrm{TTR}_{\mathrm{recovery}} \propto (1 - \rho_{\mathrm{met}})^{-2}$ diverges right as the toxic
backlog explodes.

## 3. Conjugate buffer exhaustion and zero-order ruin

In paracetamol (*acetaminophen*) metabolism, roughly $90\%$ of a therapeutic dose is cleared safely by
high-capacity glucuronidation and sulfation, while $5\text{--}10\%$ is oxidised by $\text{CYP2E1}$ into
the reactive electrophile $\text{NAPQI}$ `[Literature]`. Under normal conditions, hepatic **glutathione**
($\text{GSH}$) acts as a fast conjugate buffer ([Chemical Rates, Equilibrium, and Buffers](24-chemical-rates-equilibrium-buffers.md) §4),
neutralising $\text{NAPQI}$ instantly.

Once an overdose depletes hepatic $\text{GSH}$ below roughly $30\%$ of baseline `[Literature]`, the
conjugate buffer hits its absorbing boundary ($R_t \to 0$), $\rho_{\mathrm{met}}$ crosses $1$, and
unbuffered $\text{NAPQI}$ binds covalently to mitochondrial proteins. At that point, knowing the exact
plasma concentration on the Rumack–Matthew nomogram after the regeneration window (`TTR > TTS`) is
Collapse Proposition 2 in clinical biochemistry: perfect diagnostic lucidity after the antidote window
($N$-acetylcysteine) has closed.

## 4. The three collapses in the biochemical order

1. **Collapse Proposition 1 ($\neg S_{\mathrm{epi}}$ — Short-Circuiting Proofreading):** Truncating the
   phosphorylation cascade $N$ (or ignoring pharmacogenomic $\text{CYP2C9}/\text{DPD}$ slow-metaboliser
   subpopulations in the archive), triggering false-positive autoimmunity or unexpected $\rho_{\mathrm{met}} > 1$.
2. **Collapse Proposition 2 ($\neg S_{\mathrm{vital}}$ — Enzymatic Saturation and Buffer Depletion):**
   Dosing near $\rho_{\mathrm{met}} \to 1$ where $C_{\mathrm{ss}} = K_m \frac{\rho_{\mathrm{met}}}{1-\rho_{\mathrm{met}}}$
   diverges and $\mathrm{TTS}_{\mathrm{survival}} < \mathrm{TTR}_{\mathrm{recovery}}$.
3. **Collapse Proposition 3 ($\neg S_{\mathrm{gov}}$ — Receptor Downregulation and Allosteric Sabotage):**
   Silencing inhibitory phosphatases (`SHP-1`) or feedback sensors so the pathway amplifies its own
   signalling without an off-switch.

## Bridge

Layer 11 (`multilayer.rs::KnowledgeOrder`, `KnowledgeOrder::L11BiochemicalKineticMolecular`, `{T2Severity, T5Temporal, T6Performative}`) propagates its local and ancestor-cone margins through `multilayer.rs::MultilayerHomomorphism::evaluate` into [Chapter 62 — Layer 12: Morphogenetic, Evolutionary Capacitance & Developmental Canalization Order](62-layer-morphogenetic-evolutionary-developmental-order.md) (`KnowledgeOrder::L12MorphogeneticDevelopmental`, Stratum III, `{T1Framing, T5Temporal, T6Performative}`).
