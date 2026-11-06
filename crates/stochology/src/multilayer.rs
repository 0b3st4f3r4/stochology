//! # Multilayer Graphs of Human Knowledge ($\mathcal{G}_1$) and Stochological Aim ($\mathcal{G}_2$)
//!
//! This module formalizes the **bijective 35-layer architecture** connecting:
//! - **$\mathcal{G}_1$ (`MultilayerGraph1`):** The 35-layer Directed Acyclic Graph (DAG) of human knowledge,
//!   in which each layer $L_k$ ($k \in \{1,\dots,35\}$, corresponding 1-to-1 to Part V Chapters `51`–`85`)
//!   is uniquely identified by one of the $\binom{7}{3} = 35$ unordered 3-site combinations of the seven
//!   guard sites of [`crate::grammar::GuardSite`] ($T_1 \dots T_7$), partitioned into **7 Ontological Strata
//!   (`KnowledgeStratum`, $7 \times 5 = 35$)** and **5 Triadic Exposure Regimes (`TriadicExposureRegime`,
//!   $12 + 9 + 9 + 4 + 1 = 35$)**.
//! - **$\mathcal{G}_2$ (`MultilayerGraph2`):** The dual 35-layer Stochological Guard & Triad Graph, which maps
//!   every order of knowledge $L_k$ to its canonical 3-site subset $\{g_a, g_b, g_c\} \in \binom{\mathcal{G}}{3}$,
//!   its isolated exposure vulnerability (`CollapseProposition`), and its cumulative Leontief floor across its
//!   causal/epistemic ancestor cone $\mathrm{Anc}^*(L_k)$:
//!
//! $$\Phi: \mathcal{G}_1 \longrightarrow \mathcal{G}_2, \qquad M_{\mathrm{cum}}(L_k) = \min_{L_j \in \mathrm{Anc}^*(L_k)} \min\bigl(S_{\mathrm{epi}}(L_j),\, S_{\mathrm{vital}}(L_j),\, S_{\mathrm{gov}}(L_j)\bigr).$$
//!
//! Because Leontief non-substitutability commutes with taking minima over ancestor cones, a structural collapse
//! at any basal layer $L_j$ bounds every downstream higher-order discipline $L_k$ that depends on $L_j$.

use crate::grammar::{GuardSite, TriadAxis};
use crate::kernel::SplitMix64;
use crate::synthesis::{CollapseProposition, TriadicIntegrity};

/// The 7 Ontological-Mediational Strata partitioning the $\binom{7}{3} = 35$ orders of knowledge into
/// $7 \times 5 = 35$ layers, such that every stratum of 5 orders covers all 7 [`GuardSite`]s $\{T_1,\dots,T_7\}$.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KnowledgeStratum {
    /// Stratum I (`L01`–`L05`, Chapters `51`–`55`): Basal Physical-Dynamical, Thermodynamic & Spatiotemporal Regimes.
    S1PhysicalDynamical,
    /// Stratum II (`L06`–`L10`, Chapters `56`–`60`): Formal, Computable, Cryptographic, Metrological & Methodological Foundations.
    S2FormalMetrological,
    /// Stratum III (`L11`–`L15`, Chapters `61`–`65`): Molecular, Evolutionary, Immunological & Deep-Time Living Systems.
    S3LivingEvolutionary,
    /// Stratum IV (`L16`–`L20`, Chapters `66`–`70`): Clinical, Epidemiological, Pharmacovigilant, Metascientific & Formative Praxis.
    S4ClinicalFormative,
    /// Stratum V (`L21`–`L25`, Chapters `71`–`75`): Semiotic, Philological, Historiographical, Archival & Media Orders.
    S5SemioticArchival,
    /// Stratum VI (`L26`–`L30`, Chapters `76`–`80`): Strategic, Financial, Econometric, Accounting & Fiduciary Orders.
    S6StrategicEconomic,
    /// Stratum VII (`L31`–`L35`, Chapters `81`–`85`): Legal, Constitutional, Cyber-Physical, High-Hazard & Biospheric Commons Orders.
    S7ConstitutionalBiospheric,
}

impl KnowledgeStratum {
    /// All 7 canonical strata in ascending ontological and institutional order.
    pub const ALL: [Self; 7] = [
        Self::S1PhysicalDynamical,
        Self::S2FormalMetrological,
        Self::S3LivingEvolutionary,
        Self::S4ClinicalFormative,
        Self::S5SemioticArchival,
        Self::S6StrategicEconomic,
        Self::S7ConstitutionalBiospheric,
    ];

    /// 1-based index (`1..=7`) of this stratum.
    pub const fn index(self) -> u8 {
        match self {
            Self::S1PhysicalDynamical => 1,
            Self::S2FormalMetrological => 2,
            Self::S3LivingEvolutionary => 3,
            Self::S4ClinicalFormative => 4,
            Self::S5SemioticArchival => 5,
            Self::S6StrategicEconomic => 6,
            Self::S7ConstitutionalBiospheric => 7,
        }
    }

    /// Short machine-readable code (`"S1"`..`"S7"`).
    pub const fn code(self) -> &'static str {
        match self {
            Self::S1PhysicalDynamical => "S1",
            Self::S2FormalMetrological => "S2",
            Self::S3LivingEvolutionary => "S3",
            Self::S4ClinicalFormative => "S4",
            Self::S5SemioticArchival => "S5",
            Self::S6StrategicEconomic => "S6",
            Self::S7ConstitutionalBiospheric => "S7",
        }
    }

    /// Human-readable title of the stratum.
    pub const fn title(self) -> &'static str {
        match self {
            Self::S1PhysicalDynamical => {
                "Stratum I · Basal Physical-Dynamical, Thermodynamic & Spatiotemporal Regimes"
            }
            Self::S2FormalMetrological => {
                "Stratum II · Formal, Computable, Cryptographic, Metrological & Methodological Foundations"
            }
            Self::S3LivingEvolutionary => {
                "Stratum III · Molecular, Evolutionary, Immunological & Deep-Time Living Systems"
            }
            Self::S4ClinicalFormative => {
                "Stratum IV · Clinical, Epidemiological, Pharmacovigilant, Metascientific & Formative Praxis"
            }
            Self::S5SemioticArchival => {
                "Stratum V · Semiotic, Philological, Historiographical, Archival & Media Orders"
            }
            Self::S6StrategicEconomic => {
                "Stratum VI · Strategic, Financial, Econometric, Accounting & Fiduciary Orders"
            }
            Self::S7ConstitutionalBiospheric => {
                "Stratum VII · Legal, Constitutional, Cyber-Physical, High-Hazard & Biospheric Commons Orders"
            }
        }
    }

    /// The exact 5 [`KnowledgeOrder`]s belonging to this stratum ($7 \times 5 = 35$).
    pub const fn orders(self) -> [KnowledgeOrder; 5] {
        match self {
            Self::S1PhysicalDynamical => [
                KnowledgeOrder::L01MicroKinetic,
                KnowledgeOrder::L02MesoBufferedQueue,
                KnowledgeOrder::L03MacroCoupledTopology,
                KnowledgeOrder::L04GeophysicalChaosDataAssimilation,
                KnowledgeOrder::L05ContinuumFractureStructural,
            ],
            Self::S2FormalMetrological => [
                KnowledgeOrder::L06FormalComputableLinearType,
                KnowledgeOrder::L07ProbabilityCredenceMeasurement,
                KnowledgeOrder::L08MetrologicalInfoGeometric,
                KnowledgeOrder::L09ExperimentalSamplingCausal,
                KnowledgeOrder::L10CryptographicZeroKnowledgeAdversarial,
            ],
            Self::S3LivingEvolutionary => [
                KnowledgeOrder::L11BiochemicalKineticMolecular,
                KnowledgeOrder::L12MorphogeneticDevelopmental,
                KnowledgeOrder::L13ImmunologicalSelfGovernance,
                KnowledgeOrder::L14TaxonomySystematicsOntology,
                KnowledgeOrder::L15StratigraphyPaleobiologyDeepTime,
            ],
            Self::S4ClinicalFormative => [
                KnowledgeOrder::L16PharmacologicalClinicalDiagnostic,
                KnowledgeOrder::L17EpidemiologyOutbreakSurveillance,
                KnowledgeOrder::L18PharmacovigilanceRegulatoryToxicology,
                KnowledgeOrder::L19MetascienceScientometricsReplication,
                KnowledgeOrder::L20AcousticAestheticTragicPedagogical,
            ],
            Self::S5SemioticArchival => [
                KnowledgeOrder::L21LinguisticEvidentialPragmatic,
                KnowledgeOrder::L22PhilologicalEpigraphicTextual,
                KnowledgeOrder::L23TaphonomicHistoriographical,
                KnowledgeOrder::L24CriticalArchivalTransitionalJustice,
                KnowledgeOrder::L25MediaEcologyInformationRetrieval,
            ],
            Self::S6StrategicEconomic => [
                KnowledgeOrder::L26SequentialDecisionBanditRealOptions,
                KnowledgeOrder::L27GameTheoryStrategicAdversarial,
                KnowledgeOrder::L28MacroFinanceEconometricsActuarial,
                KnowledgeOrder::L29OfficialStatisticsForensicAccounting,
                KnowledgeOrder::L30CorporateGovernancePrincipalAgent,
            ],
            Self::S7ConstitutionalBiospheric => [
                KnowledgeOrder::L31CriminalProofForensicEpistemology,
                KnowledgeOrder::L32EvidentiaryTortConstitutional,
                KnowledgeOrder::L33DistributedConsensusSre,
                KnowledgeOrder::L34HighHazardInfrastructureSafety,
                KnowledgeOrder::L35PolycentricCommonsBiospheric,
            ],
        }
    }

    /// Canonical 1-to-1 [`GuardSite`] anchored by this ontological stratum (`7 Strata <-> 7 Guard Sites`):
    /// - `S1PhysicalDynamical` $\longleftrightarrow$ [`GuardSite::T5Temporal`] (`World -> Institution`: irreversible time & survival clock);
    /// - `S2FormalMetrological` $\longleftrightarrow$ [`GuardSite::T1Framing`] (`Institution -> Archive`: formal types, axioms & inquiry framing);
    /// - `S3LivingEvolutionary` $\longleftrightarrow$ [`GuardSite::T4Formalization`] (`Archive -> Institution`: nonlinear canalization, ontology & tail formalization);
    /// - `S4ClinicalFormative` $\longleftrightarrow$ [`GuardSite::T2Severity`] (`Archive -> World`: clinical, epidemiological & metascientific severe testing);
    /// - `S5SemioticArchival` $\longleftrightarrow$ [`GuardSite::T3Archive`] (`World -> Archive`: evidentials, philology, taphonomy & documentary recording);
    /// - `S6StrategicEconomic` $\longleftrightarrow$ [`GuardSite::T6Performative`] (`Institution -> World`: strategic reactivity, endogenous risk & Goodhart);
    /// - `S7ConstitutionalBiospheric` $\longleftrightarrow$ [`GuardSite::T7Liability`] (`World <-> Archive <-> Institution`: legal proof, constitutional veto & commons liability).
    pub const fn canonical_guard_site(self) -> GuardSite {
        match self {
            Self::S1PhysicalDynamical => GuardSite::T5Temporal,
            Self::S2FormalMetrological => GuardSite::T1Framing,
            Self::S3LivingEvolutionary => GuardSite::T4Formalization,
            Self::S4ClinicalFormative => GuardSite::T2Severity,
            Self::S5SemioticArchival => GuardSite::T3Archive,
            Self::S6StrategicEconomic => GuardSite::T6Performative,
            Self::S7ConstitutionalBiospheric => GuardSite::T7Liability,
        }
    }

    /// Inverse of [`Self::canonical_guard_site`]: returns the unique [`KnowledgeStratum`] (`S1`–`S7`)
    /// anchored by `site`.
    pub const fn from_canonical_guard_site(site: GuardSite) -> Self {
        match site {
            GuardSite::T5Temporal => Self::S1PhysicalDynamical,
            GuardSite::T1Framing => Self::S2FormalMetrological,
            GuardSite::T4Formalization => Self::S3LivingEvolutionary,
            GuardSite::T2Severity => Self::S4ClinicalFormative,
            GuardSite::T3Archive => Self::S5SemioticArchival,
            GuardSite::T6Performative => Self::S6StrategicEconomic,
            GuardSite::T7Liability => Self::S7ConstitutionalBiospheric,
        }
    }

    /// Verifies that the union of [`KnowledgeOrder::guard_sites`] across the 5 orders in this stratum
    /// covers all 7 [`GuardSite`]s $\{T_1, T_2, T_3, T_4, T_5, T_6, T_7\}$.
    pub fn covers_all_seven_guard_sites(self) -> bool {
        let mut mask = 0u8;
        for order in self.orders() {
            for site in order.guard_sites() {
                mask |= 1u8 << (site as u8);
            }
        }
        mask == 0b0111_1111
    }

    /// Verifies that the 5 orders in this stratum have 5 pairwise-distinct [`KnowledgeOrder::primary_guard_site`]s
    /// and that the stratum's own [`Self::canonical_guard_site`] is among them.
    pub fn has_distinct_primary_guard_sites(self) -> bool {
        let mut mask = 0u8;
        for order in self.orders() {
            let bit = 1u8 << (order.primary_guard_site() as u8);
            if (mask & bit) != 0 {
                return false;
            }
            mask |= bit;
        }
        let anchor_bit = 1u8 << (self.canonical_guard_site() as u8);
        mask.count_ones() == 5 && (mask & anchor_bit) != 0
    }
}

/// The 5 Triadic Exposure Regimes classifying the $\binom{7}{3} = 35$ combinations by their
/// axis-occupancy signature $(k_{\mathrm{epi}}, k_{\mathrm{vital}}, k_{\mathrm{gov}})$:
/// $$12_{(1,1,1)} + 9_{(2,1,0)\cup(1,2,0)} + 9_{(2,0,1)\cup(1,0,2)} + 4_{(0,2,1)\cup(0,1,2)} + 1_{(3,0,0)} = 35.$$
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TriadicExposureRegime {
    /// Signature $(1,1,1)$ — 12 orders: spans all three axes (`111`, no focal axis omitted).
    TransTriadicKeystone,
    /// Signatures $(2,1,0)$ and $(1,2,0)$ — $6+3=9$ orders: omits $S_{\mathrm{gov}}$ locally (`110` $\to$ Prop 3 if uncoupled).
    GovernanceBlindSpot,
    /// Signatures $(2,0,1)$ and $(1,0,2)$ — $6+3=9$ orders: omits $S_{\mathrm{vital}}$ locally (`101` $\to$ Prop 2 if uncoupled).
    VitalBlindSpot,
    /// Signatures $(0,2,1)$ and $(0,1,2)$ — $2+2=4$ orders: omits $S_{\mathrm{epi}}$ locally (`011` $\to$ Prop 1 if uncoupled).
    EpistemicBlindSpot,
    /// Signature $(3,0,0)$ — 1 order (`L09`): concentrates all 3 sites in $S_{\mathrm{epi}}$ (`100` $\to$ Props 2 & 3 if uncoupled).
    PureEpistemicConcentration,
}

impl TriadicExposureRegime {
    /// All 5 regimes in canonical order.
    pub const ALL: [Self; 5] = [
        Self::TransTriadicKeystone,
        Self::GovernanceBlindSpot,
        Self::VitalBlindSpot,
        Self::EpistemicBlindSpot,
        Self::PureEpistemicConcentration,
    ];

    /// Expected combinatorial count of orders in this regime across $\binom{7}{3} = 35$.
    pub const fn expected_count(self) -> usize {
        match self {
            Self::TransTriadicKeystone => 12,
            Self::GovernanceBlindSpot => 9,
            Self::VitalBlindSpot => 9,
            Self::EpistemicBlindSpot => 4,
            Self::PureEpistemicConcentration => 1,
        }
    }

    /// Structural collapse suffered by a discipline in this regime if deployed in isolation without
    /// inter-layer DAG coupling to its missing axis/axes.
    pub const fn uncoupled_collapse(self) -> CollapseProposition {
        match self {
            Self::TransTriadicKeystone => CollapseProposition::IntactTriad,
            Self::GovernanceBlindSpot => CollapseProposition::Proposition3CynicalCapture,
            Self::VitalBlindSpot => CollapseProposition::Proposition2ImpotentLucidity,
            Self::EpistemicBlindSpot => CollapseProposition::Proposition1BlindUndersizing,
            Self::PureEpistemicConcentration => CollapseProposition::CombinedPropositions2And3,
        }
    }
}

/// The 35 progressively specific orders of knowledge (`L01`–`L35`), mapped 1-to-1 to the
/// $\binom{7}{3} = 35$ unique 3-combinations of [`GuardSite`] and to Part V Chapters `51`–`85`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KnowledgeOrder {
    /// Layer 01 (Ch 51, `{T1, T5, T7}`): Micro-Kinetic & Sensorimotor Order ($\tau_{\mathrm{act}} \ll \tau_{\mathrm{delib}}$).
    L01MicroKinetic,
    /// Layer 02 (Ch 52, `{T4, T6, T7}`): Meso-Buffered Queue, Operational Dispatch & Cognitive Bottleneck Order.
    L02MesoBufferedQueue,
    /// Layer 03 (Ch 53, `{T4, T5, T6}`): Macro-Coupled Network & Topological Contagion Order.
    L03MacroCoupledTopology,
    /// Layer 04 (Ch 54, `{T3, T4, T5}`): Geophysical Fluid Dynamics, Lorenz Chaos & Data Assimilation Order.
    L04GeophysicalChaosDataAssimilation,
    /// Layer 05 (Ch 55, `{T2, T4, T7}`): Continuum Mechanics, Fracture & Structural Engineering Order.
    L05ContinuumFractureStructural,
    /// Layer 06 (Ch 56, `{T1, T4, T5}`): Formal Logic, Computability & Linear Resource-Type Order.
    L06FormalComputableLinearType,
    /// Layer 07 (Ch 57, `{T1, T2, T4}`): Probability Foundations, Imprecise Credences & Measurement Axiomatics Order.
    L07ProbabilityCredenceMeasurement,
    /// Layer 08 (Ch 58, `{T3, T4, T6}`): Metrological & Information-Geometric Order.
    L08MetrologicalInfoGeometric,
    /// Layer 09 (Ch 59, `{T1, T2, T3}`): Experimental Design, Complex Survey Sampling & Causal Methodology Order.
    L09ExperimentalSamplingCausal,
    /// Layer 10 (Ch 60, `{T2, T6, T7}`): Cryptographic Protocols, Zero-Knowledge Proofs & Adversarial Verification Order.
    L10CryptographicZeroKnowledgeAdversarial,
    /// Layer 11 (Ch 61, `{T2, T5, T6}`): Biochemical Kinetics, Enzyme Queues & Molecular Proofreading Order.
    L11BiochemicalKineticMolecular,
    /// Layer 12 (Ch 62, `{T1, T5, T6}`): Morphogenetic, Evolutionary Capacitance & Developmental Canalization Order.
    L12MorphogeneticDevelopmental,
    /// Layer 13 (Ch 63, `{T2, T5, T7}`): Immunological Self-Recognition & Biological Governance Order.
    L13ImmunologicalSelfGovernance,
    /// Layer 14 (Ch 64, `{T1, T3, T4}`): Taxonomy, Phylogenetic Systematics & Formal Ontology Order.
    L14TaxonomySystematicsOntology,
    /// Layer 15 (Ch 65, `{T1, T3, T5}`): Stratigraphy, Paleobiology, Deep Time & Mass-Extinction Order.
    L15StratigraphyPaleobiologyDeepTime,
    /// Layer 16 (Ch 66, `{T2, T4, T5}`): Pharmacological, Clinical Diagnostic & Acute Therapeutic Order.
    L16PharmacologicalClinicalDiagnostic,
    /// Layer 17 (Ch 67, `{T2, T3, T5}`): Infectious Disease Epidemiology, Phylodynamics & Outbreak Surveillance Order.
    L17EpidemiologyOutbreakSurveillance,
    /// Layer 18 (Ch 68, `{T2, T3, T7}`): Pharmacovigilance, Regulatory Toxicology & Post-Market Safety Order.
    L18PharmacovigilanceRegulatoryToxicology,
    /// Layer 19 (Ch 69, `{T2, T3, T6}`): Metascience, Scientometrics, Publication Bias & Replication Order.
    L19MetascienceScientometricsReplication,
    /// Layer 20 (Ch 70, `{T1, T4, T6}`): Acoustic, Aesthetic, Tragic & Pedagogical Formative Order.
    L20AcousticAestheticTragicPedagogical,
    /// Layer 21 (Ch 71, `{T3, T6, T7}`): Linguistic, Grammatical Evidentiality & Organizational Pragmatics Order.
    L21LinguisticEvidentialPragmatic,
    /// Layer 22 (Ch 72, `{T2, T3, T4}`): Philological, Epigraphic & Stemmatic Textual-Criticism Order.
    L22PhilologicalEpigraphicTextual,
    /// Layer 23 (Ch 73, `{T3, T5, T7}`): Taphonomic, Archaeological & Bitemporal Historiographical Order.
    L23TaphonomicHistoriographical,
    /// Layer 24 (Ch 74, `{T1, T3, T7}`): Critical Archival Studies, Transitional Justice & Subaltern Testimony Order.
    L24CriticalArchivalTransitionalJustice,
    /// Layer 25 (Ch 75, `{T1, T3, T6}`): Media Ecology, Information Retrieval & Algorithmic Public-Sphere Order.
    L25MediaEcologyInformationRetrieval,
    /// Layer 26 (Ch 76, `{T1, T2, T5}`): Sequential Decision Theory, Optimal Stopping, Bandits & Real Options Order.
    L26SequentialDecisionBanditRealOptions,
    /// Layer 27 (Ch 77, `{T1, T2, T6}`): Game Theory, Strategic Classification & Adversarial Learning Order.
    L27GameTheoryStrategicAdversarial,
    /// Layer 28 (Ch 78, `{T2, T4, T6}`): Macro-Finance, Econometrics, Actuarial Ruin & Endogenous Risk Order.
    L28MacroFinanceEconometricsActuarial,
    /// Layer 29 (Ch 79, `{T3, T4, T7}`): Official Statistics, Forensic Accounting, Double-Entry Ledger & Welfare Index Order.
    L29OfficialStatisticsForensicAccounting,
    /// Layer 30 (Ch 80, `{T1, T6, T7}`): Corporate Governance, Principal-Agent Theory & Mechanism Design Order.
    L30CorporateGovernancePrincipalAgent,
    /// Layer 31 (Ch 81, `{T1, T2, T7}`): Criminal Procedure, Standards of Proof & Forensic Epistemology Order.
    L31CriminalProofForensicEpistemology,
    /// Layer 32 (Ch 82, `{T1, T4, T7}`): Civil Evidentiary Law, Tort Liability & Constitutional Entrenchment Order.
    L32EvidentiaryTortConstitutional,
    /// Layer 33 (Ch 83, `{T5, T6, T7}`): Distributed Consensus, Fault-Tolerant Computing & SRE Order.
    L33DistributedConsensusSre,
    /// Layer 34 (Ch 84, `{T4, T5, T7}`): High-Hazard Infrastructure, Nuclear & Hydraulic Safety Engineering Order.
    L34HighHazardInfrastructureSafety,
    /// Layer 35 (Ch 85, `{T3, T5, T6}`): Polycentric Commons, Urban Metabolism & Planetary Biospheric Order.
    L35PolycentricCommonsBiospheric,
}

impl KnowledgeOrder {
    /// All 35 orders of knowledge in ascending topological order (`L01`..`L35`).
    pub const ALL: [Self; 35] = [
        Self::L01MicroKinetic,
        Self::L02MesoBufferedQueue,
        Self::L03MacroCoupledTopology,
        Self::L04GeophysicalChaosDataAssimilation,
        Self::L05ContinuumFractureStructural,
        Self::L06FormalComputableLinearType,
        Self::L07ProbabilityCredenceMeasurement,
        Self::L08MetrologicalInfoGeometric,
        Self::L09ExperimentalSamplingCausal,
        Self::L10CryptographicZeroKnowledgeAdversarial,
        Self::L11BiochemicalKineticMolecular,
        Self::L12MorphogeneticDevelopmental,
        Self::L13ImmunologicalSelfGovernance,
        Self::L14TaxonomySystematicsOntology,
        Self::L15StratigraphyPaleobiologyDeepTime,
        Self::L16PharmacologicalClinicalDiagnostic,
        Self::L17EpidemiologyOutbreakSurveillance,
        Self::L18PharmacovigilanceRegulatoryToxicology,
        Self::L19MetascienceScientometricsReplication,
        Self::L20AcousticAestheticTragicPedagogical,
        Self::L21LinguisticEvidentialPragmatic,
        Self::L22PhilologicalEpigraphicTextual,
        Self::L23TaphonomicHistoriographical,
        Self::L24CriticalArchivalTransitionalJustice,
        Self::L25MediaEcologyInformationRetrieval,
        Self::L26SequentialDecisionBanditRealOptions,
        Self::L27GameTheoryStrategicAdversarial,
        Self::L28MacroFinanceEconometricsActuarial,
        Self::L29OfficialStatisticsForensicAccounting,
        Self::L30CorporateGovernancePrincipalAgent,
        Self::L31CriminalProofForensicEpistemology,
        Self::L32EvidentiaryTortConstitutional,
        Self::L33DistributedConsensusSre,
        Self::L34HighHazardInfrastructureSafety,
        Self::L35PolycentricCommonsBiospheric,
    ];

    /// 1-based layer number (`1..=35`).
    pub const fn layer_index(self) -> u8 {
        match self {
            Self::L01MicroKinetic => 1,
            Self::L02MesoBufferedQueue => 2,
            Self::L03MacroCoupledTopology => 3,
            Self::L04GeophysicalChaosDataAssimilation => 4,
            Self::L05ContinuumFractureStructural => 5,
            Self::L06FormalComputableLinearType => 6,
            Self::L07ProbabilityCredenceMeasurement => 7,
            Self::L08MetrologicalInfoGeometric => 8,
            Self::L09ExperimentalSamplingCausal => 9,
            Self::L10CryptographicZeroKnowledgeAdversarial => 10,
            Self::L11BiochemicalKineticMolecular => 11,
            Self::L12MorphogeneticDevelopmental => 12,
            Self::L13ImmunologicalSelfGovernance => 13,
            Self::L14TaxonomySystematicsOntology => 14,
            Self::L15StratigraphyPaleobiologyDeepTime => 15,
            Self::L16PharmacologicalClinicalDiagnostic => 16,
            Self::L17EpidemiologyOutbreakSurveillance => 17,
            Self::L18PharmacovigilanceRegulatoryToxicology => 18,
            Self::L19MetascienceScientometricsReplication => 19,
            Self::L20AcousticAestheticTragicPedagogical => 20,
            Self::L21LinguisticEvidentialPragmatic => 21,
            Self::L22PhilologicalEpigraphicTextual => 22,
            Self::L23TaphonomicHistoriographical => 23,
            Self::L24CriticalArchivalTransitionalJustice => 24,
            Self::L25MediaEcologyInformationRetrieval => 25,
            Self::L26SequentialDecisionBanditRealOptions => 26,
            Self::L27GameTheoryStrategicAdversarial => 27,
            Self::L28MacroFinanceEconometricsActuarial => 28,
            Self::L29OfficialStatisticsForensicAccounting => 29,
            Self::L30CorporateGovernancePrincipalAgent => 30,
            Self::L31CriminalProofForensicEpistemology => 31,
            Self::L32EvidentiaryTortConstitutional => 32,
            Self::L33DistributedConsensusSre => 33,
            Self::L34HighHazardInfrastructureSafety => 34,
            Self::L35PolycentricCommonsBiospheric => 35,
        }
    }

    /// Corresponding Part V chapter number in `book-src/` (`51..=85`).
    pub const fn chapter_number(self) -> u8 {
        50 + self.layer_index()
    }

    /// Short machine-readable code (`"L01"`..`"L35"`).
    pub const fn code(self) -> &'static str {
        match self {
            Self::L01MicroKinetic => "L01",
            Self::L02MesoBufferedQueue => "L02",
            Self::L03MacroCoupledTopology => "L03",
            Self::L04GeophysicalChaosDataAssimilation => "L04",
            Self::L05ContinuumFractureStructural => "L05",
            Self::L06FormalComputableLinearType => "L06",
            Self::L07ProbabilityCredenceMeasurement => "L07",
            Self::L08MetrologicalInfoGeometric => "L08",
            Self::L09ExperimentalSamplingCausal => "L09",
            Self::L10CryptographicZeroKnowledgeAdversarial => "L10",
            Self::L11BiochemicalKineticMolecular => "L11",
            Self::L12MorphogeneticDevelopmental => "L12",
            Self::L13ImmunologicalSelfGovernance => "L13",
            Self::L14TaxonomySystematicsOntology => "L14",
            Self::L15StratigraphyPaleobiologyDeepTime => "L15",
            Self::L16PharmacologicalClinicalDiagnostic => "L16",
            Self::L17EpidemiologyOutbreakSurveillance => "L17",
            Self::L18PharmacovigilanceRegulatoryToxicology => "L18",
            Self::L19MetascienceScientometricsReplication => "L19",
            Self::L20AcousticAestheticTragicPedagogical => "L20",
            Self::L21LinguisticEvidentialPragmatic => "L21",
            Self::L22PhilologicalEpigraphicTextual => "L22",
            Self::L23TaphonomicHistoriographical => "L23",
            Self::L24CriticalArchivalTransitionalJustice => "L24",
            Self::L25MediaEcologyInformationRetrieval => "L25",
            Self::L26SequentialDecisionBanditRealOptions => "L26",
            Self::L27GameTheoryStrategicAdversarial => "L27",
            Self::L28MacroFinanceEconometricsActuarial => "L28",
            Self::L29OfficialStatisticsForensicAccounting => "L29",
            Self::L30CorporateGovernancePrincipalAgent => "L30",
            Self::L31CriminalProofForensicEpistemology => "L31",
            Self::L32EvidentiaryTortConstitutional => "L32",
            Self::L33DistributedConsensusSre => "L33",
            Self::L34HighHazardInfrastructureSafety => "L34",
            Self::L35PolycentricCommonsBiospheric => "L35",
        }
    }

    /// Markdown filename in `book-src/` for this order of knowledge.
    pub const fn chapter_file(self) -> &'static str {
        match self {
            Self::L01MicroKinetic => "51-layer-micro-kinetic-sensorimotor-order.md",
            Self::L02MesoBufferedQueue => "52-layer-meso-buffered-queue-dispatch-order.md",
            Self::L03MacroCoupledTopology => "53-layer-macro-coupled-network-contagion-order.md",
            Self::L04GeophysicalChaosDataAssimilation => {
                "54-layer-geophysical-chaos-data-assimilation-order.md"
            }
            Self::L05ContinuumFractureStructural => {
                "55-layer-fracture-continuum-structural-order.md"
            }
            Self::L06FormalComputableLinearType => "56-layer-formal-computable-type-order.md",
            Self::L07ProbabilityCredenceMeasurement => {
                "57-layer-probability-credence-measurement-order.md"
            }
            Self::L08MetrologicalInfoGeometric => {
                "58-layer-metrological-information-geometric-order.md"
            }
            Self::L09ExperimentalSamplingCausal => "59-layer-experimental-sampling-causal-order.md",
            Self::L10CryptographicZeroKnowledgeAdversarial => {
                "60-layer-cryptographic-zero-knowledge-adversarial-order.md"
            }
            Self::L11BiochemicalKineticMolecular => {
                "61-layer-biochemical-kinetic-molecular-order.md"
            }
            Self::L12MorphogeneticDevelopmental => {
                "62-layer-morphogenetic-evolutionary-developmental-order.md"
            }
            Self::L13ImmunologicalSelfGovernance => {
                "63-layer-immunological-self-governance-order.md"
            }
            Self::L14TaxonomySystematicsOntology => {
                "64-layer-taxonomy-systematics-ontology-order.md"
            }
            Self::L15StratigraphyPaleobiologyDeepTime => {
                "65-layer-stratigraphy-paleobiology-deep-time-order.md"
            }
            Self::L16PharmacologicalClinicalDiagnostic => {
                "66-layer-pharmacological-clinical-diagnostic-order.md"
            }
            Self::L17EpidemiologyOutbreakSurveillance => {
                "67-layer-epidemiology-outbreak-surveillance-order.md"
            }
            Self::L18PharmacovigilanceRegulatoryToxicology => {
                "68-layer-pharmacovigilance-regulatory-toxicology-order.md"
            }
            Self::L19MetascienceScientometricsReplication => {
                "69-layer-metascience-scientometrics-replication-order.md"
            }
            Self::L20AcousticAestheticTragicPedagogical => {
                "70-layer-acoustic-aesthetic-tragic-pedagogical-order.md"
            }
            Self::L21LinguisticEvidentialPragmatic => {
                "71-layer-linguistic-evidential-pragmatic-order.md"
            }
            Self::L22PhilologicalEpigraphicTextual => {
                "72-layer-philological-epigraphic-textual-order.md"
            }
            Self::L23TaphonomicHistoriographical => {
                "73-layer-taphonomic-archaeological-historiographical-order.md"
            }
            Self::L24CriticalArchivalTransitionalJustice => {
                "74-layer-critical-archival-transitional-justice-order.md"
            }
            Self::L25MediaEcologyInformationRetrieval => {
                "75-layer-media-ecology-information-retrieval-order.md"
            }
            Self::L26SequentialDecisionBanditRealOptions => {
                "76-layer-sequential-decision-bandit-real-options-order.md"
            }
            Self::L27GameTheoryStrategicAdversarial => {
                "77-layer-game-theory-strategic-adversarial-order.md"
            }
            Self::L28MacroFinanceEconometricsActuarial => {
                "78-layer-macro-finance-econometrics-actuarial-order.md"
            }
            Self::L29OfficialStatisticsForensicAccounting => {
                "79-layer-official-statistics-forensic-accounting-order.md"
            }
            Self::L30CorporateGovernancePrincipalAgent => {
                "80-layer-corporate-governance-principal-agent-order.md"
            }
            Self::L31CriminalProofForensicEpistemology => {
                "81-layer-criminal-proof-forensic-epistemology-order.md"
            }
            Self::L32EvidentiaryTortConstitutional => {
                "82-layer-evidentiary-tort-constitutional-legal-order.md"
            }
            Self::L33DistributedConsensusSre => "83-layer-distributed-consensus-sre-order.md",
            Self::L34HighHazardInfrastructureSafety => {
                "84-layer-infrastructure-nuclear-hydraulic-safety-order.md"
            }
            Self::L35PolycentricCommonsBiospheric => {
                "85-layer-polycentric-commons-urban-biospheric-order.md"
            }
        }
    }

    /// Ontological-mediational stratum (`S1`–`S7`) to which this order belongs.
    pub const fn stratum(self) -> KnowledgeStratum {
        match self {
            Self::L01MicroKinetic
            | Self::L02MesoBufferedQueue
            | Self::L03MacroCoupledTopology
            | Self::L04GeophysicalChaosDataAssimilation
            | Self::L05ContinuumFractureStructural => KnowledgeStratum::S1PhysicalDynamical,
            Self::L06FormalComputableLinearType
            | Self::L07ProbabilityCredenceMeasurement
            | Self::L08MetrologicalInfoGeometric
            | Self::L09ExperimentalSamplingCausal
            | Self::L10CryptographicZeroKnowledgeAdversarial => {
                KnowledgeStratum::S2FormalMetrological
            }
            Self::L11BiochemicalKineticMolecular
            | Self::L12MorphogeneticDevelopmental
            | Self::L13ImmunologicalSelfGovernance
            | Self::L14TaxonomySystematicsOntology
            | Self::L15StratigraphyPaleobiologyDeepTime => KnowledgeStratum::S3LivingEvolutionary,
            Self::L16PharmacologicalClinicalDiagnostic
            | Self::L17EpidemiologyOutbreakSurveillance
            | Self::L18PharmacovigilanceRegulatoryToxicology
            | Self::L19MetascienceScientometricsReplication
            | Self::L20AcousticAestheticTragicPedagogical => KnowledgeStratum::S4ClinicalFormative,
            Self::L21LinguisticEvidentialPragmatic
            | Self::L22PhilologicalEpigraphicTextual
            | Self::L23TaphonomicHistoriographical
            | Self::L24CriticalArchivalTransitionalJustice
            | Self::L25MediaEcologyInformationRetrieval => KnowledgeStratum::S5SemioticArchival,
            Self::L26SequentialDecisionBanditRealOptions
            | Self::L27GameTheoryStrategicAdversarial
            | Self::L28MacroFinanceEconometricsActuarial
            | Self::L29OfficialStatisticsForensicAccounting
            | Self::L30CorporateGovernancePrincipalAgent => KnowledgeStratum::S6StrategicEconomic,
            Self::L31CriminalProofForensicEpistemology
            | Self::L32EvidentiaryTortConstitutional
            | Self::L33DistributedConsensusSre
            | Self::L34HighHazardInfrastructureSafety
            | Self::L35PolycentricCommonsBiospheric => KnowledgeStratum::S7ConstitutionalBiospheric,
        }
    }

    /// Human-readable title of the knowledge order.
    pub const fn title(self) -> &'static str {
        match self {
            Self::L01MicroKinetic => "Micro-Kinetic & Sensorimotor Order",
            Self::L02MesoBufferedQueue => {
                "Meso-Buffered Queue, Operational Dispatch & Cognitive Bottleneck Order"
            }
            Self::L03MacroCoupledTopology => "Macro-Coupled Network & Topological Contagion Order",
            Self::L04GeophysicalChaosDataAssimilation => {
                "Geophysical Fluid Dynamics, Lorenz Chaos & Data Assimilation Order"
            }
            Self::L05ContinuumFractureStructural => {
                "Continuum Mechanics, Fracture & Structural Engineering Order"
            }
            Self::L06FormalComputableLinearType => {
                "Formal Logic, Computability & Linear Resource-Type Order"
            }
            Self::L07ProbabilityCredenceMeasurement => {
                "Probability Foundations, Imprecise Credences & Measurement Axiomatics Order"
            }
            Self::L08MetrologicalInfoGeometric => "Metrological & Information-Geometric Order",
            Self::L09ExperimentalSamplingCausal => {
                "Experimental Design, Complex Survey Sampling & Causal Methodology Order"
            }
            Self::L10CryptographicZeroKnowledgeAdversarial => {
                "Cryptographic Protocols, Zero-Knowledge Proofs & Adversarial Verification Order"
            }
            Self::L11BiochemicalKineticMolecular => {
                "Biochemical Kinetics, Enzyme Queues & Molecular Proofreading Order"
            }
            Self::L12MorphogeneticDevelopmental => {
                "Morphogenetic, Evolutionary Capacitance & Developmental Canalization Order"
            }
            Self::L13ImmunologicalSelfGovernance => {
                "Immunological Self-Recognition & Biological Governance Order"
            }
            Self::L14TaxonomySystematicsOntology => {
                "Taxonomy, Phylogenetic Systematics & Formal Ontology Order"
            }
            Self::L15StratigraphyPaleobiologyDeepTime => {
                "Stratigraphy, Paleobiology, Deep Time & Mass-Extinction Order"
            }
            Self::L16PharmacologicalClinicalDiagnostic => {
                "Pharmacological, Clinical Diagnostic & Acute Therapeutic Order"
            }
            Self::L17EpidemiologyOutbreakSurveillance => {
                "Infectious Disease Epidemiology, Phylodynamics & Outbreak Surveillance Order"
            }
            Self::L18PharmacovigilanceRegulatoryToxicology => {
                "Pharmacovigilance, Regulatory Toxicology & Post-Market Safety Order"
            }
            Self::L19MetascienceScientometricsReplication => {
                "Metascience, Scientometrics, Publication Bias & Replication Order"
            }
            Self::L20AcousticAestheticTragicPedagogical => {
                "Acoustic, Aesthetic, Tragic & Pedagogical Formative Order"
            }
            Self::L21LinguisticEvidentialPragmatic => {
                "Linguistic, Grammatical Evidentiality & Organizational Pragmatics Order"
            }
            Self::L22PhilologicalEpigraphicTextual => {
                "Philological, Epigraphic & Stemmatic Textual-Criticism Order"
            }
            Self::L23TaphonomicHistoriographical => {
                "Taphonomic, Archaeological & Bitemporal Historiographical Order"
            }
            Self::L24CriticalArchivalTransitionalJustice => {
                "Critical Archival Studies, Transitional Justice & Subaltern Testimony Order"
            }
            Self::L25MediaEcologyInformationRetrieval => {
                "Media Ecology, Information Retrieval & Algorithmic Public-Sphere Order"
            }
            Self::L26SequentialDecisionBanditRealOptions => {
                "Sequential Decision Theory, Optimal Stopping, Bandits & Real Options Order"
            }
            Self::L27GameTheoryStrategicAdversarial => {
                "Game Theory, Strategic Classification & Adversarial Learning Order"
            }
            Self::L28MacroFinanceEconometricsActuarial => {
                "Macro-Finance, Econometrics, Actuarial Ruin & Endogenous Risk Order"
            }
            Self::L29OfficialStatisticsForensicAccounting => {
                "Official Statistics, Forensic Accounting, Double-Entry Ledger & Welfare Index Order"
            }
            Self::L30CorporateGovernancePrincipalAgent => {
                "Corporate Governance, Principal-Agent Theory & Mechanism Design Order"
            }
            Self::L31CriminalProofForensicEpistemology => {
                "Criminal Procedure, Standards of Proof & Forensic Epistemology Order"
            }
            Self::L32EvidentiaryTortConstitutional => {
                "Civil Evidentiary Law, Tort Liability & Constitutional Entrenchment Order"
            }
            Self::L33DistributedConsensusSre => {
                "Distributed Consensus, Fault-Tolerant Computing & SRE Order"
            }
            Self::L34HighHazardInfrastructureSafety => {
                "High-Hazard Infrastructure, Nuclear & Hydraulic Safety Engineering Order"
            }
            Self::L35PolycentricCommonsBiospheric => {
                "Polycentric Commons, Urban Metabolism & Planetary Biospheric Order"
            }
        }
    }

    /// The unique canonical 3-combination $\{g_a, g_b, g_c\} \in \binom{\mathcal{G}}{3}$ ($g_a < g_b < g_c$)
    /// defining this order of knowledge in $\mathcal{G}_2$. Across `KnowledgeOrder::ALL`, this mapping is
    /// a bijection onto all $\binom{7}{3} = 35$ unordered triples of [`GuardSite`].
    pub const fn guard_sites(self) -> [GuardSite; 3] {
        match self {
            Self::L01MicroKinetic => [
                GuardSite::T1Framing,
                GuardSite::T5Temporal,
                GuardSite::T7Liability,
            ],
            Self::L02MesoBufferedQueue => [
                GuardSite::T4Formalization,
                GuardSite::T6Performative,
                GuardSite::T7Liability,
            ],
            Self::L03MacroCoupledTopology => [
                GuardSite::T4Formalization,
                GuardSite::T5Temporal,
                GuardSite::T6Performative,
            ],
            Self::L04GeophysicalChaosDataAssimilation => [
                GuardSite::T3Archive,
                GuardSite::T4Formalization,
                GuardSite::T5Temporal,
            ],
            Self::L05ContinuumFractureStructural => [
                GuardSite::T2Severity,
                GuardSite::T4Formalization,
                GuardSite::T7Liability,
            ],
            Self::L06FormalComputableLinearType => [
                GuardSite::T1Framing,
                GuardSite::T4Formalization,
                GuardSite::T5Temporal,
            ],
            Self::L07ProbabilityCredenceMeasurement => [
                GuardSite::T1Framing,
                GuardSite::T2Severity,
                GuardSite::T4Formalization,
            ],
            Self::L08MetrologicalInfoGeometric => [
                GuardSite::T3Archive,
                GuardSite::T4Formalization,
                GuardSite::T6Performative,
            ],
            Self::L09ExperimentalSamplingCausal => [
                GuardSite::T1Framing,
                GuardSite::T2Severity,
                GuardSite::T3Archive,
            ],
            Self::L10CryptographicZeroKnowledgeAdversarial => [
                GuardSite::T2Severity,
                GuardSite::T6Performative,
                GuardSite::T7Liability,
            ],
            Self::L11BiochemicalKineticMolecular => [
                GuardSite::T2Severity,
                GuardSite::T5Temporal,
                GuardSite::T6Performative,
            ],
            Self::L12MorphogeneticDevelopmental => [
                GuardSite::T1Framing,
                GuardSite::T5Temporal,
                GuardSite::T6Performative,
            ],
            Self::L13ImmunologicalSelfGovernance => [
                GuardSite::T2Severity,
                GuardSite::T5Temporal,
                GuardSite::T7Liability,
            ],
            Self::L14TaxonomySystematicsOntology => [
                GuardSite::T1Framing,
                GuardSite::T3Archive,
                GuardSite::T4Formalization,
            ],
            Self::L15StratigraphyPaleobiologyDeepTime => [
                GuardSite::T1Framing,
                GuardSite::T3Archive,
                GuardSite::T5Temporal,
            ],
            Self::L16PharmacologicalClinicalDiagnostic => [
                GuardSite::T2Severity,
                GuardSite::T4Formalization,
                GuardSite::T5Temporal,
            ],
            Self::L17EpidemiologyOutbreakSurveillance => [
                GuardSite::T2Severity,
                GuardSite::T3Archive,
                GuardSite::T5Temporal,
            ],
            Self::L18PharmacovigilanceRegulatoryToxicology => [
                GuardSite::T2Severity,
                GuardSite::T3Archive,
                GuardSite::T7Liability,
            ],
            Self::L19MetascienceScientometricsReplication => [
                GuardSite::T2Severity,
                GuardSite::T3Archive,
                GuardSite::T6Performative,
            ],
            Self::L20AcousticAestheticTragicPedagogical => [
                GuardSite::T1Framing,
                GuardSite::T4Formalization,
                GuardSite::T6Performative,
            ],
            Self::L21LinguisticEvidentialPragmatic => [
                GuardSite::T3Archive,
                GuardSite::T6Performative,
                GuardSite::T7Liability,
            ],
            Self::L22PhilologicalEpigraphicTextual => [
                GuardSite::T2Severity,
                GuardSite::T3Archive,
                GuardSite::T4Formalization,
            ],
            Self::L23TaphonomicHistoriographical => [
                GuardSite::T3Archive,
                GuardSite::T5Temporal,
                GuardSite::T7Liability,
            ],
            Self::L24CriticalArchivalTransitionalJustice => [
                GuardSite::T1Framing,
                GuardSite::T3Archive,
                GuardSite::T7Liability,
            ],
            Self::L25MediaEcologyInformationRetrieval => [
                GuardSite::T1Framing,
                GuardSite::T3Archive,
                GuardSite::T6Performative,
            ],
            Self::L26SequentialDecisionBanditRealOptions => [
                GuardSite::T1Framing,
                GuardSite::T2Severity,
                GuardSite::T5Temporal,
            ],
            Self::L27GameTheoryStrategicAdversarial => [
                GuardSite::T1Framing,
                GuardSite::T2Severity,
                GuardSite::T6Performative,
            ],
            Self::L28MacroFinanceEconometricsActuarial => [
                GuardSite::T2Severity,
                GuardSite::T4Formalization,
                GuardSite::T6Performative,
            ],
            Self::L29OfficialStatisticsForensicAccounting => [
                GuardSite::T3Archive,
                GuardSite::T4Formalization,
                GuardSite::T7Liability,
            ],
            Self::L30CorporateGovernancePrincipalAgent => [
                GuardSite::T1Framing,
                GuardSite::T6Performative,
                GuardSite::T7Liability,
            ],
            Self::L31CriminalProofForensicEpistemology => [
                GuardSite::T1Framing,
                GuardSite::T2Severity,
                GuardSite::T7Liability,
            ],
            Self::L32EvidentiaryTortConstitutional => [
                GuardSite::T1Framing,
                GuardSite::T4Formalization,
                GuardSite::T7Liability,
            ],
            Self::L33DistributedConsensusSre => [
                GuardSite::T5Temporal,
                GuardSite::T6Performative,
                GuardSite::T7Liability,
            ],
            Self::L34HighHazardInfrastructureSafety => [
                GuardSite::T4Formalization,
                GuardSite::T5Temporal,
                GuardSite::T7Liability,
            ],
            Self::L35PolycentricCommonsBiospheric => [
                GuardSite::T3Archive,
                GuardSite::T5Temporal,
                GuardSite::T6Performative,
            ],
        }
    }

    /// Triadic occupancy signature $(k_{\mathrm{epi}}, k_{\mathrm{vital}}, k_{\mathrm{gov}})$ of [`Self::guard_sites`],
    /// satisfying $k_{\mathrm{epi}} + k_{\mathrm{vital}} + k_{\mathrm{gov}} = 3$.
    pub const fn triadic_occupancy(self) -> (u8, u8, u8) {
        let sites = self.guard_sites();
        let mut epi = 0u8;
        let mut vital = 0u8;
        let mut gov = 0u8;
        let mut i = 0;
        while i < 3 {
            match sites[i].axis() {
                TriadAxis::Epistemic => epi += 1,
                TriadAxis::Vital => vital += 1,
                TriadAxis::Institutional => gov += 1,
            }
            i += 1;
        }
        (epi, vital, gov)
    }

    /// The [`TriadicExposureRegime`] of this order of knowledge based on [`Self::triadic_occupancy`].
    pub const fn exposure_regime(self) -> TriadicExposureRegime {
        match self.triadic_occupancy() {
            (1, 1, 1) => TriadicExposureRegime::TransTriadicKeystone,
            (2, 1, 0) | (1, 2, 0) => TriadicExposureRegime::GovernanceBlindSpot,
            (2, 0, 1) | (1, 0, 2) => TriadicExposureRegime::VitalBlindSpot,
            (0, 2, 1) | (0, 1, 2) => TriadicExposureRegime::EpistemicBlindSpot,
            (3, 0, 0) => TriadicExposureRegime::PureEpistemicConcentration,
            _ => unreachable!(),
        }
    }

    /// Structural collapse suffered by this order if deployed in isolation without inter-layer DAG coupling.
    pub const fn uncoupled_vulnerability(self) -> CollapseProposition {
        self.exposure_regime().uncoupled_collapse()
    }

    /// Primary [`GuardSite`] in $\mathcal{G}_2$ where this order of knowledge exerts its sharpest bottleneck.
    ///
    /// Across `KnowledgeOrder::ALL` ($7 \times 5 = 35$), each of the 7 [`GuardSite`]s is the primary site of
    /// **5** orders, and within every [`KnowledgeStratum`] (`S1`–`S7`) the 5 orders have 5 distinct
    /// primary guard sites including the stratum's [`KnowledgeStratum::canonical_guard_site`].
    pub const fn primary_guard_site(self) -> GuardSite {
        match self {
            // Stratum I (S1, anchor T5): {T5, T7, T6, T3, T4}
            Self::L01MicroKinetic => GuardSite::T5Temporal,
            Self::L02MesoBufferedQueue => GuardSite::T7Liability,
            Self::L03MacroCoupledTopology => GuardSite::T6Performative,
            Self::L04GeophysicalChaosDataAssimilation => GuardSite::T3Archive,
            Self::L05ContinuumFractureStructural => GuardSite::T4Formalization,
            // Stratum II (S2, anchor T1): {T1, T4, T3, T2, T7}
            Self::L06FormalComputableLinearType => GuardSite::T1Framing,
            Self::L07ProbabilityCredenceMeasurement => GuardSite::T4Formalization,
            Self::L08MetrologicalInfoGeometric => GuardSite::T3Archive,
            Self::L09ExperimentalSamplingCausal => GuardSite::T2Severity,
            Self::L10CryptographicZeroKnowledgeAdversarial => GuardSite::T7Liability,
            // Stratum III (S3, anchor T4): {T2, T5, T7, T4, T1}
            Self::L11BiochemicalKineticMolecular => GuardSite::T2Severity,
            Self::L12MorphogeneticDevelopmental => GuardSite::T5Temporal,
            Self::L13ImmunologicalSelfGovernance => GuardSite::T7Liability,
            Self::L14TaxonomySystematicsOntology => GuardSite::T4Formalization,
            Self::L15StratigraphyPaleobiologyDeepTime => GuardSite::T1Framing,
            // Stratum IV (S4, anchor T2): {T4, T3, T2, T6, T1}
            Self::L16PharmacologicalClinicalDiagnostic => GuardSite::T4Formalization,
            Self::L17EpidemiologyOutbreakSurveillance => GuardSite::T3Archive,
            Self::L18PharmacovigilanceRegulatoryToxicology => GuardSite::T2Severity,
            Self::L19MetascienceScientometricsReplication => GuardSite::T6Performative,
            Self::L20AcousticAestheticTragicPedagogical => GuardSite::T1Framing,
            // Stratum V (S5, anchor T3): {T7, T3, T5, T1, T6}
            Self::L21LinguisticEvidentialPragmatic => GuardSite::T7Liability,
            Self::L22PhilologicalEpigraphicTextual => GuardSite::T3Archive,
            Self::L23TaphonomicHistoriographical => GuardSite::T5Temporal,
            Self::L24CriticalArchivalTransitionalJustice => GuardSite::T1Framing,
            Self::L25MediaEcologyInformationRetrieval => GuardSite::T6Performative,
            // Stratum VI (S6, anchor T6): {T5, T2, T6, T3, T1}
            Self::L26SequentialDecisionBanditRealOptions => GuardSite::T5Temporal,
            Self::L27GameTheoryStrategicAdversarial => GuardSite::T2Severity,
            Self::L28MacroFinanceEconometricsActuarial => GuardSite::T6Performative,
            Self::L29OfficialStatisticsForensicAccounting => GuardSite::T3Archive,
            Self::L30CorporateGovernancePrincipalAgent => GuardSite::T1Framing,
            // Stratum VII (S7, anchor T7): {T2, T7, T5, T4, T6}
            Self::L31CriminalProofForensicEpistemology => GuardSite::T2Severity,
            Self::L32EvidentiaryTortConstitutional => GuardSite::T7Liability,
            Self::L33DistributedConsensusSre => GuardSite::T5Temporal,
            Self::L34HighHazardInfrastructureSafety => GuardSite::T4Formalization,
            Self::L35PolycentricCommonsBiospheric => GuardSite::T6Performative,
        }
    }

    /// Primary [`TriadAxis`] of this order's bottleneck.
    pub const fn primary_triad_axis(self) -> TriadAxis {
        self.primary_guard_site().axis()
    }

    /// Canonical mathematical or structural isomorphism tying $\mathcal{G}_1$ to $\mathcal{G}_2$ at this layer.
    pub const fn canonical_isomorphism(self) -> &'static str {
        match self {
            Self::L01MicroKinetic => {
                "Two-tape separation (télos/skopós) and TTS_survival >= TTR_recovery"
            }
            Self::L02MesoBufferedQueue => {
                "Kingman heavy-traffic hyperbola rho/(1-rho) * (c_a^2+c_s^2)/2 and 10 bits/s filter"
            }
            Self::L03MacroCoupledTopology => {
                "Motter-Lai cascade tolerance alpha_tol = (1-rho)/rho as reciprocal of Kingman"
            }
            Self::L04GeophysicalChaosDataAssimilation => {
                "Lorenz Lyapunov horizon tau_L ~ (1/lambda_1) ln(delta_tol/eps_0) and 4D-Var/CRPS closure"
            }
            Self::L05ContinuumFractureStructural => {
                "Griffith critical crack a_c as basin fold bifurcation and Weibull weakest link min_i R_i"
            }
            Self::L06FormalComputableLinearType => {
                "Rice undecidability -> LL(1) syntactic judge() and Girard linear/affine resource types"
            }
            Self::L07ProbabilityCredenceMeasurement => {
                "Walley imprecise credences [P_low, P_high] = Manski bounds and Stevens scale invariance"
            }
            Self::L08MetrologicalInfoGeometric => {
                "BIPM GUM Type A vs Type B uncertainty and Bregman Pythagorean CORP-PAV projection"
            }
            Self::L09ExperimentalSamplingCausal => {
                "Fisher randomization + Kish DEFF = 1 + (m-1)*rho_cl and Simonsohn multiverse curve"
            }
            Self::L10CryptographicZeroKnowledgeAdversarial => {
                "Kerckhoffs-Shannon secrecy, Dolev-Yao adversary, and zero-knowledge soundness = e-value"
            }
            Self::L11BiochemicalKineticMolecular => {
                "Hopfield-Ninio kinetic proofreading eps_1^N = e-process and Michaelis-Menten = Kingman queue"
            }
            Self::L12MorphogeneticDevelopmental => {
                "Waddington chreod / Hsp90 capacitance = Canguilhem-Holling basin and Itô evolutionary drain"
            }
            Self::L13ImmunologicalSelfGovernance => {
                "Two-signal TCR+CD28 = List-Pettit conjunctive veto and Treg/CTLA-4/PD-1 = S3* algedonic brake"
            }
            Self::L14TaxonomySystematicsOntology => {
                "Hennig cladistic apomorphy vs homoplasy and Bowker-Star infrastructural classification torque"
            }
            Self::L15StratigraphyPaleobiologyDeepTime => {
                "Signor-Lipps fossil truncation = archival tail censorship and Raup-Sepkoski extinction alpha"
            }
            Self::L16PharmacologicalClinicalDiagnostic => {
                "Pauker-Kassirer p* = C/(eta*L), therapeutic index TI = TD50/ED50, and sepsis Golden Hour"
            }
            Self::L17EpidemiologyOutbreakSurveillance => {
                "Kermack-McKendrick R_0 = 1 transcritical threshold and serial interval vs tracing queue TTR"
            }
            Self::L18PharmacovigilanceRegulatoryToxicology => {
                "Bradford Hill coherence, FAERS spontaneous reporting rho_{R,Y} bias, and recall veto gate"
            }
            Self::L19MetascienceScientometricsReplication => {
                "Ioannidis PPV under file-drawer bias, Smaldino-McElreath metric selection, and e-BH FDR control"
            }
            Self::L20AcousticAestheticTragicPedagogical => {
                "Pythagorean comma (3/2)^12 != 2^7, tragic hamartía vs T4, and Vygotsky ZPD fractional probe"
            }
            Self::L21LinguisticEvidentialPragmatic => {
                "Tuyuca/Quechua obligatory morphological evidentials vs March-Simon uncertainty absorption"
            }
            Self::L22PhilologicalEpigraphicTextual => {
                "Lachmann stemma codicum = cluster DEFF and Leiden epigraphic [...] = Manski bounds"
            }
            Self::L23TaphonomicHistoriographical => {
                "Efremov taphonomic filter = Meng rho_{R,Y} + Chao n_00 and TemporalVault against hindsight"
            }
            Self::L24CriticalArchivalTransitionalJustice => {
                "Fricker testimonial justice, Trouillot archival silencing, and Ball capture-recapture n_00"
            }
            Self::L25MediaEcologyInformationRetrieval => {
                "Innis bias of communication and recommender feedback loop = performative rho_{R,Y} amplification"
            }
            Self::L26SequentialDecisionBanditRealOptions => {
                "Wald SPRT boundaries, Gittins bandit index, and Dixit-Pindyck irreversible option value"
            }
            Self::L27GameTheoryStrategicAdversarial => {
                "Perdomo performative stability vs strategic manipulation and minimax adversarial robustness"
            }
            Self::L28MacroFinanceEconometricsActuarial => {
                "Cramér-Lundberg ruin, Artzner coherent CVaR vs VaR, Lucas critique, and Goodhart 1975"
            }
            Self::L29OfficialStatisticsForensicAccounting => {
                "Pacioli double-entry conservation invariant, Desrosières census ontology, and Sen poverty axioms"
            }
            Self::L30CorporateGovernancePrincipalAgent => {
                "Holmström-Milgrom multitask distortion, Hurwicz incentive compatibility, and Innes liability"
            }
            Self::L31CriminalProofForensicEpistemology => {
                "Blackstone asymmetric error loss, likelihood-ratio forensic weight, and chain-of-custody seal"
            }
            Self::L32EvidentiaryTortConstitutional => {
                "Learned Hand B < P*L with tail amplification and constitutional supermajority as hysteresis"
            }
            Self::L33DistributedConsensusSre => {
                "FLP/CAP theorem, metastable retry storms at rho -> 1, and SRE error-budget freeze gate"
            }
            Self::L34HighHazardInfrastructureSafety => {
                "Perrow interactive complexity x tight coupling, Rasmussen drift, and hardwired SCRAM veto"
            }
            Self::L35PolycentricCommonsBiospheric => {
                "Ostrom 8 polycentric principles, Rockström planetary boundaries, and Leontief triadic floor"
            }
        }
    }

    /// Direct parent layers in $\mathcal{G}_1$ (`parent_idx < self.layer_index()`), guaranteeing a strict DAG
    /// whose transitive ancestor cone $\mathrm{Anc}^*(L_k)$ always spans all three triad axes.
    pub const fn parent_layers(self) -> &'static [KnowledgeOrder] {
        match self {
            Self::L01MicroKinetic => &[],
            Self::L02MesoBufferedQueue => &[Self::L01MicroKinetic],
            Self::L03MacroCoupledTopology => &[Self::L01MicroKinetic, Self::L02MesoBufferedQueue],
            Self::L04GeophysicalChaosDataAssimilation => {
                &[Self::L01MicroKinetic, Self::L03MacroCoupledTopology]
            }
            Self::L05ContinuumFractureStructural => {
                &[Self::L01MicroKinetic, Self::L02MesoBufferedQueue]
            }
            Self::L06FormalComputableLinearType => &[Self::L01MicroKinetic],
            Self::L07ProbabilityCredenceMeasurement => {
                &[Self::L01MicroKinetic, Self::L06FormalComputableLinearType]
            }
            Self::L08MetrologicalInfoGeometric => &[
                Self::L04GeophysicalChaosDataAssimilation,
                Self::L07ProbabilityCredenceMeasurement,
            ],
            Self::L09ExperimentalSamplingCausal => &[
                Self::L07ProbabilityCredenceMeasurement,
                Self::L08MetrologicalInfoGeometric,
            ],
            Self::L10CryptographicZeroKnowledgeAdversarial => &[
                Self::L06FormalComputableLinearType,
                Self::L07ProbabilityCredenceMeasurement,
            ],
            Self::L11BiochemicalKineticMolecular => &[
                Self::L01MicroKinetic,
                Self::L02MesoBufferedQueue,
                Self::L08MetrologicalInfoGeometric,
            ],
            Self::L12MorphogeneticDevelopmental => &[
                Self::L03MacroCoupledTopology,
                Self::L11BiochemicalKineticMolecular,
            ],
            Self::L13ImmunologicalSelfGovernance => &[
                Self::L11BiochemicalKineticMolecular,
                Self::L12MorphogeneticDevelopmental,
            ],
            Self::L14TaxonomySystematicsOntology => &[
                Self::L06FormalComputableLinearType,
                Self::L12MorphogeneticDevelopmental,
            ],
            Self::L15StratigraphyPaleobiologyDeepTime => &[
                Self::L04GeophysicalChaosDataAssimilation,
                Self::L12MorphogeneticDevelopmental,
                Self::L14TaxonomySystematicsOntology,
            ],
            Self::L16PharmacologicalClinicalDiagnostic => &[
                Self::L09ExperimentalSamplingCausal,
                Self::L11BiochemicalKineticMolecular,
                Self::L13ImmunologicalSelfGovernance,
            ],
            Self::L17EpidemiologyOutbreakSurveillance => &[
                Self::L02MesoBufferedQueue,
                Self::L03MacroCoupledTopology,
                Self::L09ExperimentalSamplingCausal,
                Self::L16PharmacologicalClinicalDiagnostic,
            ],
            Self::L18PharmacovigilanceRegulatoryToxicology => &[
                Self::L09ExperimentalSamplingCausal,
                Self::L16PharmacologicalClinicalDiagnostic,
                Self::L17EpidemiologyOutbreakSurveillance,
            ],
            Self::L19MetascienceScientometricsReplication => &[
                Self::L08MetrologicalInfoGeometric,
                Self::L09ExperimentalSamplingCausal,
                Self::L18PharmacovigilanceRegulatoryToxicology,
            ],
            Self::L20AcousticAestheticTragicPedagogical => &[
                Self::L01MicroKinetic,
                Self::L08MetrologicalInfoGeometric,
                Self::L16PharmacologicalClinicalDiagnostic,
            ],
            Self::L21LinguisticEvidentialPragmatic => &[
                Self::L02MesoBufferedQueue,
                Self::L06FormalComputableLinearType,
                Self::L14TaxonomySystematicsOntology,
            ],
            Self::L22PhilologicalEpigraphicTextual => &[
                Self::L09ExperimentalSamplingCausal,
                Self::L14TaxonomySystematicsOntology,
                Self::L21LinguisticEvidentialPragmatic,
            ],
            Self::L23TaphonomicHistoriographical => &[
                Self::L15StratigraphyPaleobiologyDeepTime,
                Self::L21LinguisticEvidentialPragmatic,
                Self::L22PhilologicalEpigraphicTextual,
            ],
            Self::L24CriticalArchivalTransitionalJustice => &[
                Self::L09ExperimentalSamplingCausal,
                Self::L21LinguisticEvidentialPragmatic,
                Self::L23TaphonomicHistoriographical,
            ],
            Self::L25MediaEcologyInformationRetrieval => &[
                Self::L03MacroCoupledTopology,
                Self::L19MetascienceScientometricsReplication,
                Self::L21LinguisticEvidentialPragmatic,
            ],
            Self::L26SequentialDecisionBanditRealOptions => &[
                Self::L01MicroKinetic,
                Self::L07ProbabilityCredenceMeasurement,
                Self::L09ExperimentalSamplingCausal,
            ],
            Self::L27GameTheoryStrategicAdversarial => &[
                Self::L07ProbabilityCredenceMeasurement,
                Self::L10CryptographicZeroKnowledgeAdversarial,
                Self::L26SequentialDecisionBanditRealOptions,
            ],
            Self::L28MacroFinanceEconometricsActuarial => &[
                Self::L03MacroCoupledTopology,
                Self::L26SequentialDecisionBanditRealOptions,
                Self::L27GameTheoryStrategicAdversarial,
            ],
            Self::L29OfficialStatisticsForensicAccounting => &[
                Self::L08MetrologicalInfoGeometric,
                Self::L09ExperimentalSamplingCausal,
                Self::L23TaphonomicHistoriographical,
                Self::L28MacroFinanceEconometricsActuarial,
            ],
            Self::L30CorporateGovernancePrincipalAgent => &[
                Self::L27GameTheoryStrategicAdversarial,
                Self::L28MacroFinanceEconometricsActuarial,
                Self::L29OfficialStatisticsForensicAccounting,
            ],
            Self::L31CriminalProofForensicEpistemology => &[
                Self::L09ExperimentalSamplingCausal,
                Self::L23TaphonomicHistoriographical,
                Self::L24CriticalArchivalTransitionalJustice,
            ],
            Self::L32EvidentiaryTortConstitutional => &[
                Self::L05ContinuumFractureStructural,
                Self::L30CorporateGovernancePrincipalAgent,
                Self::L31CriminalProofForensicEpistemology,
            ],
            Self::L33DistributedConsensusSre => &[
                Self::L02MesoBufferedQueue,
                Self::L03MacroCoupledTopology,
                Self::L10CryptographicZeroKnowledgeAdversarial,
            ],
            Self::L34HighHazardInfrastructureSafety => &[
                Self::L04GeophysicalChaosDataAssimilation,
                Self::L05ContinuumFractureStructural,
                Self::L32EvidentiaryTortConstitutional,
                Self::L33DistributedConsensusSre,
            ],
            Self::L35PolycentricCommonsBiospheric => &[
                Self::L15StratigraphyPaleobiologyDeepTime,
                Self::L17EpidemiologyOutbreakSurveillance,
                Self::L29OfficialStatisticsForensicAccounting,
                Self::L32EvidentiaryTortConstitutional,
                Self::L34HighHazardInfrastructureSafety,
            ],
        }
    }
}

/// A node in $\mathcal{G}_1$ (`MultilayerGraph1`), representing one order of human knowledge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Graph1Node {
    /// The canonical order of knowledge (`L01`–`L35`).
    pub order: KnowledgeOrder,
    /// The ontological-mediational stratum (`S1`–`S7`).
    pub stratum: KnowledgeStratum,
    /// Corresponding Part V chapter number (`51`–`85`).
    pub chapter: u8,
    /// Direct parent orders in $\mathcal{G}_1$ from which this order inherits constraints.
    pub parents: &'static [KnowledgeOrder],
}

/// Multilayer Graph 1 ($\mathcal{G}_1$): the 35-layer DAG of human knowledge across 7 strata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultilayerGraph1 {
    /// All 35 layer nodes in ascending topological order.
    pub nodes: [Graph1Node; 35],
}

impl MultilayerGraph1 {
    /// Constructs the canonical 35-layer knowledge graph $\mathcal{G}_1$.
    pub fn canonical() -> Self {
        let nodes = KnowledgeOrder::ALL.map(|order| Graph1Node {
            order,
            stratum: order.stratum(),
            chapter: order.chapter_number(),
            parents: order.parent_layers(),
        });
        Self { nodes }
    }

    /// Verifies that $\mathcal{G}_1$ is a strict Directed Acyclic Graph (every parent has a strictly smaller layer index).
    pub fn is_strict_dag(&self) -> bool {
        self.nodes.iter().all(|node| {
            let idx = node.order.layer_index();
            node.parents.iter().all(|p| p.layer_index() < idx)
        })
    }

    /// Total number of directed inter-layer edges $(L_j \to L_k)$ in $\mathcal{G}_1$.
    pub fn edge_count(&self) -> usize {
        self.nodes.iter().map(|n| n.parents.len()).sum()
    }

    /// Computes the reflexive-transitive ancestor cone $\mathrm{Anc}^*(L_k) \subseteq \{L_{01},\dots,L_{35}\}$
    /// in ascending layer order.
    pub fn ancestors_of(&self, target: KnowledgeOrder) -> Vec<KnowledgeOrder> {
        let mut visited = [false; 35];
        let mut stack = vec![target];
        while let Some(curr) = stack.pop() {
            let slot = (curr.layer_index() - 1) as usize;
            if !visited[slot] {
                visited[slot] = true;
                for &p in curr.parent_layers() {
                    stack.push(p);
                }
            }
        }
        let mut cone = Vec::new();
        for order in KnowledgeOrder::ALL {
            if visited[(order.layer_index() - 1) as usize] {
                cone.push(order);
            }
        }
        cone
    }

    /// Verifies that for every order $L_k \in \{L_{01},\dots,L_{35}\}$, the union of [`KnowledgeOrder::guard_sites`]
    /// over its ancestor cone $\mathrm{Anc}^*(L_k)$ covers all three axes of the Nuclear Triad.
    pub fn every_ancestor_cone_spans_all_three_axes(&self) -> bool {
        KnowledgeOrder::ALL.into_iter().all(|order| {
            let mut has_epi = false;
            let mut has_vital = false;
            let mut has_gov = false;
            for anc in self.ancestors_of(order) {
                let (e, v, g) = anc.triadic_occupancy();
                if e > 0 {
                    has_epi = true;
                }
                if v > 0 {
                    has_vital = true;
                }
                if g > 0 {
                    has_gov = true;
                }
            }
            has_epi && has_vital && has_gov
        })
    }
}

/// A node in $\mathcal{G}_2$ (`MultilayerGraph2`), mapping an order of knowledge to the Stochological circuit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Graph2Node {
    /// The canonical order of knowledge (`L01`–`L35`).
    pub order: KnowledgeOrder,
    /// Primary bottleneck [`GuardSite`] in `grammar.rs`.
    pub primary_site: GuardSite,
    /// Primary [`TriadAxis`] of the bottleneck.
    pub primary_axis: TriadAxis,
    /// Unique 3-combination $\{g_a, g_b, g_c\} \in \binom{\mathcal{G}}{3}$ constrained by this layer.
    pub guard_sites: [GuardSite; 3],
    /// Triadic occupancy signature $(k_{\mathrm{epi}}, k_{\mathrm{vital}}, k_{\mathrm{gov}})$.
    pub occupancy: (u8, u8, u8),
    /// Triadic exposure regime of this layer.
    pub regime: TriadicExposureRegime,
    /// Canonical mathematical isomorphism between $\mathcal{G}_1$ and $\mathcal{G}_2$ at this layer.
    pub isomorphism: &'static str,
}

/// Multilayer Graph 2 ($\mathcal{G}_2$): the 35-layer Stochological Guard & Triad Graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultilayerGraph2 {
    /// All 35 dual Stochological layer nodes.
    pub nodes: [Graph2Node; 35],
}

impl MultilayerGraph2 {
    /// Constructs the canonical 35-layer Stochological graph $\mathcal{G}_2$.
    pub fn canonical() -> Self {
        let nodes = KnowledgeOrder::ALL.map(|order| Graph2Node {
            order,
            primary_site: order.primary_guard_site(),
            primary_axis: order.primary_triad_axis(),
            guard_sites: order.guard_sites(),
            occupancy: order.triadic_occupancy(),
            regime: order.exposure_regime(),
            isomorphism: order.canonical_isomorphism(),
        });
        Self { nodes }
    }

    /// Verifies that the 35 layers form an exact bijection onto all $\binom{7}{3} = 35$ unordered
    /// 3-combinations of [`GuardSite`] ($g_a < g_b < g_c$, no repetitions, no omissions).
    pub fn is_bijection_over_35_guard_combinations(&self) -> bool {
        let mut seen = Vec::with_capacity(35);
        for node in &self.nodes {
            let [a, b, c] = node.guard_sites;
            if !((a as u8) < (b as u8) && (b as u8) < (c as u8)) {
                return false;
            }
            if seen.contains(&node.guard_sites) {
                return false;
            }
            seen.push(node.guard_sites);
        }
        seen.len() == 35
    }

    /// Verifies that every [`KnowledgeStratum`] (`S1`–`S7`) covers all 7 [`GuardSite`]s across its 5 orders.
    pub fn every_stratum_covers_all_seven_sites(&self) -> bool {
        KnowledgeStratum::ALL
            .into_iter()
            .all(KnowledgeStratum::covers_all_seven_guard_sites)
    }

    /// Verifies the exact $7 \times 5 = 35$ primary-site balance:
    /// 1. Every [`GuardSite`] (`T1`–`T7`) is the `primary_site` of **5** orders across $\mathcal{G}_2$;
    /// 2. Within every [`KnowledgeStratum`] (`S1`–`S7`), all 5 orders have 5 pairwise-distinct primary sites
    ///    including the stratum's [`KnowledgeStratum::canonical_guard_site`];
    /// 3. [`KnowledgeStratum::canonical_guard_site`] and [`KnowledgeStratum::from_canonical_guard_site`] form a bijection.
    pub fn is_balanced_five_orders_per_primary_site(&self) -> bool {
        let mut counts = [0usize; 7];
        for node in &self.nodes {
            if !node.guard_sites.contains(&node.primary_site) {
                return false;
            }
            counts[node.primary_site as usize] += 1;
        }
        let all_five = counts.iter().all(|&c| c == 5);
        let strata_distinct = KnowledgeStratum::ALL
            .into_iter()
            .all(KnowledgeStratum::has_distinct_primary_guard_sites);
        let bijection_holds = GuardSite::ALL
            .into_iter()
            .all(|s| KnowledgeStratum::from_canonical_guard_site(s).canonical_guard_site() == s)
            && KnowledgeStratum::ALL.into_iter().all(|st| {
                KnowledgeStratum::from_canonical_guard_site(st.canonical_guard_site()) == st
            });
        all_five && strata_distinct && bijection_holds
    }
}

/// Quantitative triadic reading $(S_{\mathrm{epi}}, S_{\mathrm{vital}}, S_{\mathrm{gov}})$ at a single layer $L_k$.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayerTriadReading {
    /// The knowledge order $L_k$.
    pub order: KnowledgeOrder,
    /// Local epistemic margin $S_{\mathrm{epi}}(L_k) \in [0, 1]$.
    pub s_epi: f64,
    /// Local vital margin $S_{\mathrm{vital}}(L_k) \in [0, 1]$.
    pub s_vital: f64,
    /// Local institutional governance margin $S_{\mathrm{gov}}(L_k) \in [0, 1]$.
    pub s_gov: f64,
}

impl LayerTriadReading {
    /// Evaluates the local [`TriadicIntegrity`] of this single layer.
    pub fn local_integrity(&self) -> TriadicIntegrity {
        TriadicIntegrity::evaluate(self.s_epi, self.s_vital, self.s_gov, 0.50)
    }
}

/// Evaluated homomorphism $\Phi: \mathcal{G}_1 \to \mathcal{G}_2$ across all 35 layers, tracking both
/// local layer margins and cumulative ancestor-cone Leontief margins.
#[derive(Debug, Clone, PartialEq)]
pub struct MultilayerHomomorphism {
    /// Local triadic integrity at each of the 35 layers.
    pub local: [TriadicIntegrity; 35],
    /// Cumulative triadic integrity over the ancestor cone $\mathrm{Anc}^*(L_k)$ at each of the 35 layers.
    pub cumulative: [TriadicIntegrity; 35],
    /// Weakest ancestor layer driving the cumulative Leontief floor for each of the 35 layers.
    pub bottleneck_ancestor: [KnowledgeOrder; 35],
}

impl MultilayerHomomorphism {
    /// Evaluates the multilayer homomorphism $\Phi: \mathcal{G}_1 \to \mathcal{G}_2$ across all 35 layers.
    ///
    /// For every layer $L_k$, the cumulative margin on each axis is the minimum across all ancestor layers
    /// $L_j \in \mathrm{Anc}^*(L_k)$, and the cumulative Leontief index is:
    /// $$M_{\mathrm{cum}}(L_k) = \min_{L_j \in \mathrm{Anc}^*(L_k)} M_{\mathrm{Leontief}}(L_j).$$
    pub fn evaluate(g1: &MultilayerGraph1, readings: &[LayerTriadReading; 35]) -> Self {
        let local = readings.map(|r| r.local_integrity());
        let mut cumulative = [local[0]; 35];
        let mut bottleneck_ancestor = KnowledgeOrder::ALL;

        for order in KnowledgeOrder::ALL {
            let slot = (order.layer_index() - 1) as usize;
            let cone = g1.ancestors_of(order);
            let mut min_epi = f64::INFINITY;
            let mut min_vital = f64::INFINITY;
            let mut min_gov = f64::INFINITY;
            let mut worst_leontief = f64::INFINITY;
            let mut worst_order = order;

            for anc in cone {
                let anc_slot = (anc.layer_index() - 1) as usize;
                let r = &readings[anc_slot];
                let integ = &local[anc_slot];
                if r.s_epi < min_epi {
                    min_epi = r.s_epi;
                }
                if r.s_vital < min_vital {
                    min_vital = r.s_vital;
                }
                if r.s_gov < min_gov {
                    min_gov = r.s_gov;
                }
                if integ.leontief_minimum < worst_leontief {
                    worst_leontief = integ.leontief_minimum;
                    worst_order = anc;
                }
            }

            cumulative[slot] = TriadicIntegrity::evaluate(min_epi, min_vital, min_gov, 0.50);
            bottleneck_ancestor[slot] = worst_order;
        }

        Self {
            local,
            cumulative,
            bottleneck_ancestor,
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Social-network topology audit (community detection and centrality, in `std`)
// ---------------------------------------------------------------------------------------------

/// Planted-block specification of the social network analysed by [`TopologyCentralityAudit`].
///
/// Nodes are assigned to `block_count` contiguous blocks; an edge from block $i$ to block $j$ is
/// drawn with probability `intra_block_probability` when $i = j$ and
/// `inter_block_probability` otherwise, and inter-block edges are biased towards the lower block
/// index, which plants a prestige order the centrality measures must recover.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SocialNetworkSpec {
    /// Number of nodes in the network.
    pub node_count: usize,
    /// Number of planted blocks (communities).
    pub block_count: usize,
    /// Edge probability inside a block.
    pub intra_block_probability: f64,
    /// Edge probability between blocks.
    pub inter_block_probability: f64,
    /// Deterministic PRNG seed.
    pub seed: u64,
}

impl SocialNetworkSpec {
    /// Canonical specification: 160 nodes, 4 blocks, $p_{\mathrm{in}} = 0.30$,
    /// $p_{\mathrm{out}} = 0.02$, seed `2026`.
    pub const fn canonical() -> Self {
        Self {
            node_count: 160,
            block_count: 4,
            intra_block_probability: 0.30,
            inter_block_probability: 0.02,
            seed: 2026,
        }
    }
}

impl Default for SocialNetworkSpec {
    fn default() -> Self {
        Self::canonical()
    }
}

/// **Social-network topology audit** — the community-detection and centrality battery the course
/// runs in Gephi, reimplemented in pure `std`: mean weighted degree, density, diameter, local
/// clustering, modularity of the planted partition and of a label-propagation partition,
/// eigenvector centrality, PageRank, HITS hubs and authorities, and Brandes betweenness.
///
/// The point of the battery is topological, not descriptive: behavioural and informational contagion
/// travels through the high-betweenness bridge nodes (Layer 03), and the recomputation of retrieval
/// prestige (Layer 25) is a PageRank computation on a directed citation graph. Local degree is not a
/// proxy for either.
#[derive(Debug, Clone, PartialEq)]
pub struct TopologyCentralityAudit {
    /// Number of nodes.
    pub node_count: usize,
    /// Number of planted blocks.
    pub block_count: usize,
    /// Number of directed edges actually drawn.
    pub edge_count: usize,
    /// Mean weighted out-degree.
    pub mean_weighted_degree: f64,
    /// Graph density of the directed network.
    pub density: f64,
    /// Undirected diameter in hops.
    pub diameter: usize,
    /// Mean local clustering coefficient of the undirected projection.
    pub clustering_coefficient: f64,
    /// Modularity $Q$ of the planted partition.
    pub modularity_planted: f64,
    /// Modularity $Q$ of a single label-propagation sweep.
    pub modularity_label_propagation: f64,
    /// Number of distinct communities the label-propagation sweep recovered.
    pub detected_community_count: usize,
    /// Largest eigenvector centrality (Perron vector of the undirected projection).
    pub eigenvector_max: f64,
    /// Mean PageRank of each planted block, in block order.
    pub pagerank_block_means: Vec<f64>,
    /// Mean eigenvector centrality of each planted block, in block order.
    pub eigenvector_block_means: Vec<f64>,
    /// Largest HITS hub score.
    pub hits_max_hub: f64,
    /// Largest HITS authority score.
    pub hits_max_authority: f64,
    /// Largest normalised betweenness centrality.
    pub betweenness_max: f64,
    /// Mean betweenness of nodes that carry at least one inter-block edge.
    pub bridge_betweenness_mean: f64,
    /// Mean betweenness of nodes that carry only intra-block edges.
    pub interior_betweenness_mean: f64,
    /// Whether PageRank recovers the planted prestige order (strictly decreasing block means).
    pub pagerank_recovers_planted_prestige: bool,
}

impl TopologyCentralityAudit {
    /// Draws the planted-block directed network as an adjacency list of `(target, weight)`.
    fn planted_network(spec: &SocialNetworkSpec) -> (Vec<Vec<(usize, f64)>>, Vec<usize>) {
        let n = spec.node_count.max(2);
        let blocks = spec.block_count.clamp(1, n);
        let block_of: Vec<usize> = (0..n).map(|i| i * blocks / n).collect();
        let mut rng = SplitMix64::new(spec.seed);
        let mut adjacency: Vec<Vec<(usize, f64)>> = vec![Vec::new(); n];
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                let same = block_of[i] == block_of[j];
                let mut p = if same {
                    spec.intra_block_probability
                } else {
                    spec.inter_block_probability
                };
                if !same && block_of[j] > block_of[i] {
                    p *= 0.35;
                }
                if rng.next_f64() < p {
                    adjacency[i].push((j, 1.0));
                }
            }
        }
        (adjacency, block_of)
    }

    /// Undirected projection as a sorted, de-duplicated neighbour list plus the adjacency matrix.
    fn undirected(adjacency: &[Vec<(usize, f64)>]) -> (Vec<Vec<usize>>, Vec<Vec<bool>>) {
        let n = adjacency.len();
        let mut matrix = vec![vec![false; n]; n];
        for (i, row) in adjacency.iter().enumerate() {
            for &(j, _) in row {
                matrix[i][j] = true;
                matrix[j][i] = true;
            }
        }
        let neighbours = (0..n)
            .map(|i| (0..n).filter(|&j| matrix[i][j]).collect::<Vec<_>>())
            .collect();
        (neighbours, matrix)
    }

    /// BFS eccentricity from `source`; returns the farthest distance reached.
    fn eccentricity(neighbours: &[Vec<usize>], source: usize) -> usize {
        let n = neighbours.len();
        let mut dist = vec![usize::MAX; n];
        dist[source] = 0;
        let mut queue = std::collections::VecDeque::from([source]);
        let mut far = 0usize;
        while let Some(u) = queue.pop_front() {
            for &v in &neighbours[u] {
                if dist[v] == usize::MAX {
                    dist[v] = dist[u] + 1;
                    far = far.max(dist[v]);
                    queue.push_back(v);
                }
            }
        }
        far
    }

    /// Brandes betweenness on the undirected unweighted projection, normalised by
    /// $1/((n-1)(n-2))$ for $n > 2$.
    fn betweenness(neighbours: &[Vec<usize>]) -> Vec<f64> {
        let n = neighbours.len();
        let mut centrality = vec![0.0_f64; n];
        for s in 0..n {
            let mut stack: Vec<usize> = Vec::with_capacity(n);
            let mut preds: Vec<Vec<usize>> = vec![Vec::new(); n];
            let mut sigma = vec![0.0_f64; n];
            sigma[s] = 1.0;
            let mut dist = vec![-1_i64; n];
            dist[s] = 0;
            let mut queue = std::collections::VecDeque::from([s]);
            while let Some(v) = queue.pop_front() {
                stack.push(v);
                for &w in &neighbours[v] {
                    if dist[w] < 0 {
                        dist[w] = dist[v] + 1;
                        queue.push_back(w);
                    }
                    if dist[w] == dist[v] + 1 {
                        sigma[w] += sigma[v];
                        preds[w].push(v);
                    }
                }
            }
            let mut delta = vec![0.0_f64; n];
            while let Some(w) = stack.pop() {
                for &v in &preds[w] {
                    if sigma[w] > 0.0 {
                        delta[v] += sigma[v] / sigma[w] * (1.0 + delta[w]);
                    }
                }
                if w != s {
                    centrality[w] += delta[w];
                }
            }
        }
        let scale = if n > 2 {
            1.0 / ((n - 1) * (n - 2)) as f64
        } else {
            1.0
        };
        for c in &mut centrality {
            *c *= scale;
        }
        centrality
    }

    /// Power iteration on a non-negative matrix given as a row-normalisable adjacency.
    fn power_iteration(weights: &[Vec<f64>], iterations: usize) -> Vec<f64> {
        let n = weights.len();
        let mut v = vec![1.0 / (n as f64).sqrt(); n];
        for _ in 0..iterations {
            let mut next = vec![0.0_f64; n];
            for i in 0..n {
                let mut acc = 0.0;
                for j in 0..n {
                    acc += weights[i][j] * v[j];
                }
                next[i] = acc;
            }
            let norm = next.iter().map(|x| x * x).sum::<f64>().sqrt();
            if norm < 1e-15 {
                break;
            }
            for x in &mut next {
                *x /= norm;
            }
            v = next;
        }
        v
    }

    /// Modularity of a partition on the undirected projection.
    fn modularity(matrix: &[Vec<bool>], block_of: &[usize], blocks: usize) -> f64 {
        let n = matrix.len();
        let mut degree = vec![0usize; n];
        let mut total = 0usize;
        for i in 0..n {
            degree[i] = matrix[i].iter().filter(|&&e| e).count();
            total += degree[i];
        }
        if total == 0 {
            return 0.0;
        }
        let mut internal = vec![0usize; blocks];
        let mut block_degree = vec![0usize; blocks];
        for i in 0..n {
            block_degree[block_of[i]] += degree[i];
            for j in (i + 1)..n {
                if matrix[i][j] && block_of[i] == block_of[j] {
                    internal[block_of[i]] += 2;
                }
            }
        }
        let m2 = total as f64;
        let mut q = 0.0;
        for c in 0..blocks {
            let l_c = internal[c] as f64;
            let d_c = block_degree[c] as f64;
            q += l_c / m2 - (d_c / m2).powi(2);
        }
        q
    }

    /// Runs the full battery on the canonical planted network.
    pub fn evaluate(spec: SocialNetworkSpec) -> Self {
        let (adjacency, block_of) = Self::planted_network(&spec);
        let n = adjacency.len();
        let (neighbours, matrix) = Self::undirected(&adjacency);

        let edge_count: usize = adjacency.iter().map(Vec::len).sum();
        let density = edge_count as f64 / (n * (n - 1)).max(1) as f64;
        let mean_weighted_degree = edge_count as f64 / n as f64;
        let diameter = (0..n)
            .map(|i| Self::eccentricity(&neighbours, i))
            .max()
            .unwrap_or(0);

        let mut clustering_sum = 0.0;
        for adjacent in &neighbours {
            let d = adjacent.len();
            if d < 2 {
                continue;
            }
            let mut links = 0usize;
            for (a_idx, &a) in adjacent.iter().enumerate() {
                for &b in &adjacent[(a_idx + 1)..] {
                    if matrix[a][b] {
                        links += 1;
                    }
                }
            }
            clustering_sum += 2.0 * links as f64 / (d * (d - 1)) as f64;
        }
        let clustering_coefficient = clustering_sum / n as f64;

        let modularity_planted = Self::modularity(&matrix, &block_of, spec.block_count);

        // One label-propagation sweep: every node adopts the most frequent label of its neighbours.
        let detected: Vec<usize> = (0..n)
            .map(|i| {
                let mut counts = vec![0usize; spec.block_count];
                for &j in &neighbours[i] {
                    counts[block_of[j]] += 1;
                }
                let mut best = block_of[i];
                for (c, &count) in counts.iter().enumerate() {
                    if count > counts[best] {
                        best = c;
                    }
                }
                best
            })
            .collect();
        let modularity_label_propagation = Self::modularity(&matrix, &detected, spec.block_count);
        let mut detected_communities: Vec<usize> = detected.clone();
        detected_communities.sort_unstable();
        detected_communities.dedup();

        // Eigenvector centrality of the undirected projection, via power iteration.
        let undirected_weights: Vec<Vec<f64>> = (0..n)
            .map(|i| {
                (0..n)
                    .map(|j| if matrix[i][j] { 1.0 } else { 0.0 })
                    .collect()
            })
            .collect();
        let eigen = Self::power_iteration(&undirected_weights, 300);

        // PageRank with damping 0.85 and uniform dangling distribution.
        let mut pagerank = vec![1.0 / n as f64; n];
        for _ in 0..100 {
            let mut next = vec![(1.0 - 0.85) / n as f64; n];
            let mut dangling = 0.0;
            for i in 0..n {
                let out = adjacency[i].len();
                if out == 0 {
                    dangling += pagerank[i];
                    continue;
                }
                let share = 0.85 * pagerank[i] / out as f64;
                for &(j, _) in &adjacency[i] {
                    next[j] += share;
                }
            }
            for value in &mut next {
                *value += 0.85 * dangling / n as f64;
            }
            pagerank = next;
        }

        // HITS: authority = principal vector of A^T A, hub = principal vector of A A^T.
        let mut ata = vec![vec![0.0_f64; n]; n];
        let mut aat = vec![vec![0.0_f64; n]; n];
        for i in 0..n {
            for &(j, _) in &adjacency[i] {
                aat[i][j] = 1.0;
                ata[j][i] = 1.0;
            }
        }
        let authority = Self::power_iteration(&ata, 200);
        let hub = Self::power_iteration(&aat, 200);

        let betweenness = Self::betweenness(&neighbours);
        let betweenness_max = betweenness.iter().copied().fold(0.0_f64, f64::max);

        let mut bridge_sum = 0.0;
        let mut bridge_count = 0usize;
        let mut interior_sum = 0.0;
        let mut interior_count = 0usize;
        for i in 0..n {
            let is_bridge = adjacency[i]
                .iter()
                .any(|&(j, _)| block_of[j] != block_of[i])
                || (0..n).any(|j| block_of[j] != block_of[i] && matrix[i][j]);
            if is_bridge {
                bridge_sum += betweenness[i];
                bridge_count += 1;
            } else {
                interior_sum += betweenness[i];
                interior_count += 1;
            }
        }
        let bridge_betweenness_mean = if bridge_count == 0 {
            0.0
        } else {
            bridge_sum / bridge_count as f64
        };
        let interior_betweenness_mean = if interior_count == 0 {
            0.0
        } else {
            interior_sum / interior_count as f64
        };

        let block_mean = |values: &[f64]| -> Vec<f64> {
            (0..spec.block_count)
                .map(|c| {
                    let members: Vec<f64> = values
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| block_of[*i] == c)
                        .map(|(_, &v)| v)
                        .collect();
                    if members.is_empty() {
                        0.0
                    } else {
                        members.iter().sum::<f64>() / members.len() as f64
                    }
                })
                .collect()
        };

        let pagerank_block_means = block_mean(&pagerank);
        let eigenvector_block_means = block_mean(&eigen);
        let pagerank_recovers_planted_prestige =
            pagerank_block_means.windows(2).all(|w| w[0] > w[1]);

        Self {
            node_count: n,
            block_count: spec.block_count,
            edge_count,
            mean_weighted_degree,
            density,
            diameter,
            clustering_coefficient,
            modularity_planted,
            modularity_label_propagation,
            detected_community_count: detected_communities.len(),
            eigenvector_max: eigen.iter().copied().fold(0.0_f64, f64::max),
            pagerank_block_means,
            eigenvector_block_means,
            hits_max_hub: hub.iter().copied().fold(0.0_f64, f64::max),
            hits_max_authority: authority.iter().copied().fold(0.0_f64, f64::max),
            betweenness_max,
            bridge_betweenness_mean,
            interior_betweenness_mean,
            pagerank_recovers_planted_prestige,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_graphs_form_exact_bijection_over_35_combinations_and_7_strata() {
        let g1 = MultilayerGraph1::canonical();
        let g2 = MultilayerGraph2::canonical();

        assert!(g1.is_strict_dag());
        assert!(g1.edge_count() >= 85);
        assert!(g1.every_ancestor_cone_spans_all_three_axes());
        assert!(g2.is_bijection_over_35_guard_combinations());
        assert!(g2.every_stratum_covers_all_seven_sites());
        assert!(g2.is_balanced_five_orders_per_primary_site());

        for (i, order) in KnowledgeOrder::ALL.iter().enumerate() {
            assert_eq!(order.layer_index() as usize, i + 1);
            assert_eq!(order.chapter_number() as usize, 51 + i);
            assert!(!order.title().is_empty());
            assert!(!order.code().is_empty());
            assert!(!order.chapter_file().is_empty());
            assert!(!order.canonical_isomorphism().is_empty());
            assert_eq!(
                order.primary_triad_axis(),
                order.primary_guard_site().axis()
            );
            assert!(order.guard_sites().contains(&order.primary_guard_site()));
            let (e, v, g) = order.triadic_occupancy();
            assert_eq!(e + v + g, 3);
        }

        for (i, stratum) in KnowledgeStratum::ALL.iter().enumerate() {
            assert_eq!(stratum.index() as usize, i + 1);
            assert!(!stratum.code().is_empty());
            assert!(!stratum.title().is_empty());
            assert_eq!(stratum.orders().len(), 5);
            assert!(stratum.covers_all_seven_guard_sites());
        }

        for regime in TriadicExposureRegime::ALL {
            let count = KnowledgeOrder::ALL
                .iter()
                .filter(|o| o.exposure_regime() == regime)
                .count();
            assert_eq!(count, regime.expected_count());
        }

        let full_cone = g1.ancestors_of(KnowledgeOrder::L35PolycentricCommonsBiospheric);
        assert!(full_cone.contains(&KnowledgeOrder::L01MicroKinetic));
        assert!(full_cone.contains(&KnowledgeOrder::L34HighHazardInfrastructureSafety));
        assert!(full_cone.contains(&KnowledgeOrder::L35PolycentricCommonsBiospheric));
    }

    #[test]
    fn homomorphism_propagates_basal_collapse_to_dependent_higher_orders() {
        let g1 = MultilayerGraph1::canonical();
        let mut readings = KnowledgeOrder::ALL.map(|order| LayerTriadReading {
            order,
            s_epi: 0.85,
            s_vital: 0.82,
            s_gov: 0.88,
        });

        let hom_intact = MultilayerHomomorphism::evaluate(&g1, &readings);
        for c in &hom_intact.cumulative {
            assert_eq!(c.diagnose, CollapseProposition::IntactTriad);
            assert!((c.leontief_minimum - 0.82).abs() < 1e-12);
        }

        // Inject Collapse Proposition 2 (Vital collapse) at Layer 02 (Meso Buffered Queue).
        readings[1].s_vital = 0.05;
        let hom_collapsed = MultilayerHomomorphism::evaluate(&g1, &readings);

        // Layer 01 is upstream of Layer 02 and remains intact.
        assert_eq!(
            hom_collapsed.cumulative[0].diagnose,
            CollapseProposition::IntactTriad
        );

        // Layer 02, Layer 17 (Epidemiology), Layer 33 (Distributed SRE), and Layer 35 (Commons)
        // all depend transitively on Layer 02 and inherit Proposition 2.
        for &idx in &[1usize, 16, 32, 34] {
            assert_eq!(
                hom_collapsed.cumulative[idx].diagnose,
                CollapseProposition::Proposition2ImpotentLucidity
            );
            assert_eq!(
                hom_collapsed.bottleneck_ancestor[idx],
                KnowledgeOrder::L02MesoBufferedQueue
            );
        }
    }
}
