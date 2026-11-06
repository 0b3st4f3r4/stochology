# Chapter 67 — Layer 17: Infectious Disease Epidemiology, Phylodynamics & Outbreak Surveillance Order

> Layer 17 (`multilayer.rs::KnowledgeOrder`, `KnowledgeOrder::L17EpidemiologyOutbreakSurveillance`) of Part V · Stratum IV (`multilayer.rs::KnowledgeStratum`, `KnowledgeStratum::S4ClinicalFormative`) · Unique Guard Combination `{T2Severity, T3Archive, T5Temporal}` · Signature `(2, 1, 0)` (`multilayer.rs::TriadicExposureRegime`, `TriadicExposureRegime::GovernanceBlindSpot`). It builds on [Layer 02](52-layer-meso-buffered-queue-dispatch-order.md), [Layer 03](53-layer-macro-coupled-network-contagion-order.md), [Layer 09](59-layer-experimental-sampling-causal-order.md), [Layer 16](66-layer-pharmacological-clinical-diagnostic-order.md) in $\mathcal{G}_1$ and maps into $\mathcal{G}_2$ via [`notes/The Multilayer Graph of Knowledge and Aim.md`](../notes/The%20Multilayer%20Graph%20of%20Knowledge%20and%20Aim.md).

## By the end of this chapter you can
1. Locate **Infectious Disease Epidemiology, Phylodynamics & Outbreak Surveillance (`L17`)** at the unique 3-combination `{T2Severity, T3Archive, T5Temporal}` in Stratum IV (`multilayer.rs::KnowledgeStratum`, `KnowledgeStratum::S4ClinicalFormative`).
2. Connect the **Kermack–McKendrick Transcritical Threshold $R_0 = \beta / \gamma$, Overdispersed Superspreading ($k \ll 1$), and Serial Interval vs. Kingman Recovery Latency** (`T5Temporal`, `vital.rs::VitalMarginState::from_queue_basin_and_survival`) with **Non-Random Diagnostic Ascertainment $\rho_{R,Y} \ne 0$** (`T3Archive`, `epistemic.rs::EpistemicMarginState::from_meng_big_data`) and **Manski Seroprevalence Bounds** (`T2Severity`, `epistemic.rs::EpistemicMarginState::from_ate_bounds`).
3. Prove by the **Disciplinary Incompleteness Theorem** why a `(2, 1, 0)` epidemiological surveillance architecture omitting $S_{\mathrm{gov}}$ (`T6`, `T7`) triggers **Collapse Proposition 3 (`synthesis.rs::CollapseProposition`, `CollapseProposition::Proposition3CynicalCapture`)** when frontline clinician warnings are suppressed or when behavioral risk compensation invalidates static projections.

## 1. The problem: Exponential Doubling and the Survival Clock

Epidemic management is a race between biological transmission dynamics and institutional reaction speed. The foundations
of mathematical epidemiology trace from John Snow's 1854 spatial cholera mapping and Ronald Ross's 1911 malaria
difference equations to William Kermack and Anderson McKendrick's 1927 compartmental ordinary differential equations. In
an epidemic model with susceptible fraction $S(t)$, infectious fraction $I(t)$, transmission rate $\beta$, and
recovery/isolation rate $\gamma$:

$$\frac{\mathrm{d}S}{\mathrm{d}t} = -\beta S I, \qquad \frac{\mathrm{d}I}{\mathrm{d}t} = (\beta S - \gamma)I.$$

The basic reproduction number $R_0 = \beta / \gamma$ governs a transcritical bifurcation. When $R_0 S > 1$, the infected
population grows exponentially with intrinsic growth rate $r = \beta S - \gamma > 0$. The epidemic doubling time defines
the physical survival window (`T5Temporal`):

$$\mathrm{TTS}_{\mathrm{survival}} = \frac{\ln 2}{r}.$$

Containing an outbreak requires reducing the effective reproduction number $R_t = R_0 S(t)$ below 1, either through
natural depletion of susceptibles past the herd immunity threshold $1 - 1/R_0$ or through rapid isolation and contact
tracing. Because pathogen doubling operates on an unrelenting physical clock, any delay in detection consumes survival
margin exponentially.

## 2. Superspreading, Queues, and Testing Selection Bias (`T2`, `T3`, `T5`)

Real-world outbreaks violate the assumption of homogeneous mixing across three coupled mechanisms:
1. **Overdispersed Superspreading ($k \ll 1$):** Secondary transmissions follow a negative binomial offspring
   distribution $Z \sim \mathrm{NegBin}(R_0, k)$ where the dispersion parameter is often $k < 0.1$. Under high
   overdispersion, $20\%$ of infected individuals generate over $80\%$ of downstream cases, while the majority infect
   zero contacts. This makes small outbreaks prone to stochastic extinction, but causes explosive exponential surges
   when a superspreading event breaches containment.
2. **Kingman Queue Delays and Vital Rupture (`T5Temporal`):** Diagnostic laboratories, contact tracing teams, and
   intensive care units are finite-capacity service queues (`L02`). When epidemic case arrivals push operational
   utilization toward saturation ($\rho_{\mathrm{ICU}} \to 1$), Kingman's heavy-traffic approximation dictates that
   waiting times explode hyperbolically as $\rho / (1 - \rho)$. Testing and quarantine delays
   $\mathrm{TTR}_{\mathrm{recovery}}(\rho)$ rapidly outstrip pathogen doubling:

$$\mathrm{TTR}_{\mathrm{recovery}}(\rho) > \mathrm{TTS}_{\mathrm{survival}}$$

(`vital.rs::VitalMarginState::from_queue_basin_and_survival`). When tracing latency exceeds the generation interval
$\tau_{\mathrm{gen}}$, contact tracing collapses into complete impotence.
3. **Ascertainment Bias and the Big Data Paradox (`T3Archive`, `T2Severity`):** Routine surveillance archives do not
   record random population samples. Symptomatic individuals test at substantially higher rates, producing a non-zero
   data-defect correlation $\rho_{R,Y} > 0$ (Xiao-Li Meng 2018;
   `epistemic.rs::EpistemicMarginState::from_meng_big_data`). Furthermore, household transmission clusters inflate
   sampling variance by Moulton's design effect $\mathrm{DEFF}_{\mathrm{Moulton}} = 1 + (m - 1)\rho_{\mathrm{cl}}$
   (`epistemic.rs::EpistemicMarginState::from_archive_clusters`). Naive testing positivity wildly overestimates true
   active infection; bounding true prevalence requires non-parametric Manski partial-identification bounds
   (`epistemic.rs::EpistemicMarginState::from_ate_bounds`).

## 3. Gompertz–Makeham Dynamics and Why `(2, 1, 0)` Requires Governance Coupling

Human mortality during respiratory pandemics follows the Gompertz–Makeham actuarial law
$\mu(x) = \alpha \exp(\beta x) + \lambda$, concentrating severe clinical outcomes in older demographic cohorts with
steep allostatic wear.

Layer 17 possesses the triadic occupancy signature `(2, 1, 0)` (`multilayer.rs::TriadicExposureRegime`,
`TriadicExposureRegime::GovernanceBlindSpot`). It unites diagnostic severity (`T2Severity`), epidemiological
surveillance archives (`T3Archive`), and exponential survival time (`T5Temporal`), but contains zero local governance
sites (`k_gov = 0`).

An epidemiological surveillance system deployed without institutional governance (`T6Performative`, `T7Liability`)
triggers **Collapse Proposition 3 (`synthesis.rs::CollapseProposition`,
`CollapseProposition::Proposition3CynicalCapture`)**:
- **Testimonial Silencing:** In the critical early days of an outbreak when exponential doubling is nascent,
   administrative authorities discount or suppress frontline clinical warnings (`T7Liability`,
   `governance.rs::GovernanceState::from_real_time_and_ex_post_gates`) to avoid economic panic, squandering the narrow
   $\mathrm{TTS}_{\mathrm{survival}}$ window.
- **Performative Behavioral Feedback:** Conventional SEIR models treat transmission parameters $\beta$ as exogenous
   physical constants. In reality, published projections alter public behavior immediately (`T6Performative`): citizens
   voluntarily reduce mobility, rendering uncalibrated static forecasts performatively misleading.

Epidemiological modeling achieves public health integrity only when coupled to protected whistleblower channels (`L24`),
statutory emergency powers (`L34`), and polycentric commons coordination (`L35`).

## 4. Lattice contagion: why the homogeneous $R_0$ is an archive artefact

Every formula above treats the susceptible population as well mixed: a single scalar $\beta$ multiplying a
single scalar $S/N$. That assumption is an *archive decision*, not a fact about the pathogen, and the
standard way to see its cost is to place the same transmission rule on a lattice and compare.

Take a cellular automaton in which each cell is susceptible, infected or removed, each infected cell has a
fixed per-step probability of transmitting to each of its four neighbours, and each infected cell recovers
after a fixed number of steps `[Literature]`. The local rule is identical to the one the mean-field
equations encode. The outcome is not.

Two divergences matter here, and both are structural rather than numerical.

1. **The threshold moves.** In the mean-field model the epidemic grows when $R_0 = \beta \tau > 1$. On a
   two-dimensional lattice with four neighbours the corresponding per-contact reproduction number must
   exceed a strictly larger value for a macroscopic outbreak, because transmission must percolate — a
   connected cluster spanning the system — and percolation on a lattice is a global property with its own
   critical point. The same $\beta$ therefore produces an outbreak in one description and a self-limiting
   cluster in the other.
2. **The final size is not a function of $R_0$ alone.** The mean-field final-size equation has the
   homogeneous solution as its only input; on a lattice the final size depends on the spatial arrangement
   of the initially infected set, which is precisely the information the homogeneous model averages away.

The stochological reading is the one the previous section anticipated. The homogeneous $R_0$ is an
estimate computed on a *specific compression* of the population — one that assumes exchangeability of
contacts — and reporting it as a property of the pathogen is the archive-to-population inference of
[Chapter 34](34-sample-population-archive.md) performed without its margin. The lattice model does not
provide a better number; it provides the demonstration that the number was never well defined without a
statement about mixing. That statement belongs to the governance axis: it is a claim about how a
population actually encounters itself, made by someone, at a time, and it is exactly the kind of premise
that Collapse Proposition 3 makes convenient to leave implicit.

The two-armed structure also gives the cheapest available audit. Run the same local rule under two mixing
assumptions — homogeneous and lattice — and compare the predicted peak and final size. A large gap is not
evidence that the lattice is right; it is evidence that the conclusion depends on an assumption that was
not reported.

## Reproduce

```bash
cargo run --release --bin stochology -- --chapter 5
cargo run --release --bin stochology -- --chapter 6
```

## Bridge

Layer 17 (`multilayer.rs::KnowledgeOrder`, `KnowledgeOrder::L17EpidemiologyOutbreakSurveillance`, `{T2Severity,
T3Archive, T5Temporal}`) propagates its local and ancestor-cone margins through
`multilayer.rs::MultilayerHomomorphism::evaluate` into [Chapter 68 — Layer 18: Pharmacovigilance, Regulatory Toxicology
& Post-Market Safety Order](68-layer-pharmacovigilance-regulatory-toxicology-order.md)
(`KnowledgeOrder::L18PharmacovigilanceRegulatoryToxicology`, Stratum IV, `{T2Severity, T3Archive, T7Liability}`).
