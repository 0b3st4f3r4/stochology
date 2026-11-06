# The Multilayer Graph of Knowledge and Aim

*How the $\binom{7}{3} = 35$ combinations of the Seven Guard Sites induce a bijective 35-layer DAG of human knowledge ($\mathcal{G}_1$, partitioned into $7 \times 5 = 35$ Ontological Strata and $12 + 9 + 9 + 4 + 1 = 35$ Triadic Exposure Regimes) and its dual Stochological Guard Graph ($\mathcal{G}_2$).*

---

## I. Why Human Knowledge Under Uncertainty Forms a 35-Layer Bijective Simplex

The Grammar of Aim (`crates/stochology/src/grammar.rs`) establishes seven sequential guard sites partitioned across the three axes of the Nuclear Triad as $3 + 2 + 2 = 7$:

$$\mathcal{G}_{\mathrm{epi}} = \{T_1, T_2, T_3\}, \qquad \mathcal{G}_{\mathrm{vital}} = \{T_4, T_5\}, \qquad \mathcal{G}_{\mathrm{gov}} = \{T_6, T_7\}.$$

Ontologically, these seven sites are not an arbitrary list: across the three planes of the mediation circuit ($\mathcal{P} = \{\mathsf{World},\, \mathsf{Archive},\, \mathsf{Institution}\}$), there are $3 \times 2 = 6$ ordered binary transits ($P_{\mathrm{from}} \to P_{\mathrm{to}}, P_{\mathrm{from}} \ne P_{\mathrm{to}}$) — decomposing into the **3 direct transits** of the action loop ($\mathsf{World} \to \mathsf{Archive}$ at $T_3$, $\mathsf{Archive} \to \mathsf{Institution}$ at $T_4$, $\mathsf{Institution} \to \mathsf{World}$ at $T_6$) and the **3 reflexive locks** ($\mathsf{Institution} \to \mathsf{Archive}$ at $T_1$, $\mathsf{Archive} \to \mathsf{World}$ at $T_2$, $\mathsf{World} \to \mathsf{Institution}$ at $T_5$) — plus **1 integral three-plane closure** ($\mathsf{World} \leftrightarrow \mathsf{Archive} \leftrightarrow \mathsf{Institution}$ at $T_7$):

$$3_{\text{direct transits}} + 3_{\text{reflexive locks}} + 1_{\text{integral circuit}} = 7 \text{ Guard Sites} \qquad (\text{\texttt{GuardSite::is\_bijection\_over\_circuit\_transits}}).$$

Every specialized disciplinary Order of Knowledge focuses its primary analytical and institutional machinery on an unordered triad of three distinct guard sites $\{g_a, g_b, g_c\} \subset \{T_1, \dots, T_7\}$ ($g_a < g_b < g_c$). By Vandermonde's identity, the total number of unique 3-combinations without repetition is **35**, partitioned across eight axis-occupancy signatures $(k_{\mathrm{epi}}, k_{\mathrm{vital}}, k_{\mathrm{gov}})$ with $k_{\mathrm{epi}} + k_{\mathrm{vital}} + k_{\mathrm{gov}} = 3$:

$$\binom{7}{3} = \underbrace{\binom{3}{1}\binom{2}{1}\binom{2}{1}}_{(1,1,1)\,=\,12} + \underbrace{\binom{3}{2}\binom{2}{1}\binom{2}{0}}_{(2,1,0)\,=\,6} + \underbrace{\binom{3}{2}\binom{2}{0}\binom{2}{1}}_{(2,0,1)\,=\,6} + \underbrace{\binom{3}{1}\binom{2}{2}\binom{2}{0}}_{(1,2,0)\,=\,3} + \underbrace{\binom{3}{1}\binom{2}{0}\binom{2}{2}}_{(1,0,2)\,=\,3} + \underbrace{\binom{3}{0}\binom{2}{2}\binom{2}{1}}_{(0,2,1)\,=\,2} + \underbrace{\binom{3}{0}\binom{2}{1}\binom{2}{2}}_{(0,1,2)\,=\,2} + \underbrace{\binom{3}{3}\binom{2}{0}\binom{2}{0}}_{(3,0,0)\,=\,1} = 35.$$

This combinatorial structure yields three foundational theorems implemented in `crates/stochology/src/multilayer.rs`:

1. **The Stratum–Guard Bijection and $7 \times 5 = 35$ Primary-Site Balance Theorem:**
   - **Canonical Bijection (`KnowledgeStratum::canonical_guard_site` $\leftrightarrow$ `from_canonical_guard_site`):** The 7 Ontological Strata (`S1`–`S7`) stand in exact 1-to-1 bijection with the 7 Guard Sites (`T1`–`T7`):
     $$S_1 \leftrightarrow T_5,\quad S_2 \leftrightarrow T_1,\quad S_3 \leftrightarrow T_4,\quad S_4 \leftrightarrow T_2,\quad S_5 \leftrightarrow T_3,\quad S_6 \leftrightarrow T_6,\quad S_7 \leftrightarrow T_7.$$
   - **Exact $7 \times 5 = 35$ Primary Bottleneck Balance (`MultilayerGraph2::is_balanced_five_orders_per_primary_site`):** Across the 35 orders, each guard site $T_i \in \{T_1, \dots, T_7\}$ is the primary bottleneck (`primary_guard_site`) of **exactly 5 orders** ($7 \times 5 = 35$, yielding $15 : 10 : 10 = 3 : 2 : 2$ across `Epistemic`, `Vital`, and `Governance`), and within every stratum $s \in \{S_1, \dots, S_7\}$ its 5 orders have **5 pairwise-distinct primary guard sites** including the stratum's own canonical guard site (`KnowledgeStratum::has_distinct_primary_guard_sites`).

2. **The Dual Stratification Theorem ($7 \times 5 = 35$ and $12 + 9 + 9 + 4 + 1 = 35$):**
   - **Horizontal Ontological Stratification (`KnowledgeStratum`, `S1`–`S7`):** The 35 orders partition into **7 Strata of 5 Orders each** (`S1PhysicalDynamical` through `S7ConstitutionalBiospheric`), such that within every single stratum $s \in \{S_1, \dots, S_7\}$, the union of focal guard sites covers all seven sites of the Grammar of Aim:
     $$\forall s \in \{S_1, \dots, S_7\}, \qquad |s| = 5 \quad \text{and} \quad \bigcup_{L_k \in s} \mathrm{guards}(L_k) = \{T_1, T_2, T_3, T_4, T_5, T_6, T_7\}.$$
   - **Vertical Triadic Exposure Stratification (`TriadicExposureRegime`):** Grouping the eight occupancy signatures by which axes of the Nuclear Triad are omitted locally yields five exposure regimes (`TransTriadicKeystone` [12], `GovernanceBlindSpot` [9], `VitalBlindSpot` [9], `EpistemicBlindSpot` [4], and `PureEpistemicConcentration` [1]), proving the **Disciplinary Incompleteness Theorem**: $23/35$ of human disciplines omit at least one axis of the Nuclear Triad in their local 3-site focal triple and therefore suffer a specific `CollapseProposition` (`Proposition1BlindUndersizing`, `Proposition2ImpotentLucidity`, `Proposition3CynicalCapture`, or `CombinedPropositions2And3`) unless coupled in the Multilayer DAG ($\mathcal{G}_1 \leftrightarrow \mathcal{G}_2$) to ancestor layers that supply the missing axis.

3. **The Cumulative Leontief Homomorphism ($\Phi: \mathcal{G}_1 \to \mathcal{G}_2$):**
   Across the reflexive-transitive ancestor cone $\mathrm{Anc}^*(L_k)$ of every order $L_k \in \{L_{01}, \dots, L_{35}\}$, the union of guarded sites spans all three axes of the Nuclear Triad, and the cumulative triadic integrity is bounded by the weakest ancestor layer:
   $$M_{\mathrm{cum}}(L_k) = \min_{L_j \in \mathrm{Anc}^*(L_k)} \min\bigl(S_{\mathrm{epi}}(L_j),\, S_{\mathrm{vital}}(L_j),\, S_{\mathrm{gov}}(L_j)\bigr).$$

---

## II. Table 1 — The 35 Canonical Orders of Knowledge ($\mathcal{G}_1 \leftrightarrow \mathcal{G}_2$)

| Code | Ch | Stratum | Order of Knowledge (`KnowledgeOrder`) | Primary Site | Primary Axis | Unique Guard Triple (`guard_sites`) | Signature $(E,V,G)$ | Canonical Isomorphism ($\mathcal{G}_1 \leftrightarrow \mathcal{G}_2$) |
| :--- | :---: | :---: | :--- | :---: | :---: | :---: | :---: | :--- |
| `L01` | `51` | `S1` | `L01MicroKinetic` | `T5` | `Vital` | `[T1, T5, T7]` | `(1,1,1)` | Two-tape separation (`télos`/`skopós`) and $\mathrm{TTS}_{\mathrm{survival}} \ge \mathrm{TTR}_{\mathrm{recovery}}$ |
| `L02` | `52` | `S1` | `L02MesoBufferedQueue` | `T7` | `Governance` | `[T4, T6, T7]` | `(0,1,2)` | Kingman heavy-traffic hyperbola $\frac{\rho}{1-\rho}\frac{c_a^2+c_s^2}{2}$ and $10\text{ bits/s}$ filter |
| `L03` | `53` | `S1` | `L03MacroCoupledTopology` | `T6` | `Governance` | `[T4, T5, T6]` | `(0,2,1)` | Motter–Lai cascade tolerance $\alpha_{\mathrm{tol}} = (1-\rho)/\rho$ as reciprocal of Kingman |
| `L04` | `54` | `S1` | `L04GeophysicalChaosDataAssimilation` | `T3` | `Epistemic` | `[T3, T4, T5]` | `(1,2,0)` | Lorenz Lyapunov horizon $\tau_L \sim \lambda_1^{-1}\ln(\delta_{\mathrm{tol}}/\varepsilon_0)$ and 4D-Var/CRPS closure |
| `L05` | `55` | `S1` | `L05ContinuumFractureStructural` | `T4` | `Vital` | `[T2, T4, T7]` | `(1,1,1)` | Griffith critical crack $a_c$ as basin fold bifurcation and Weibull weakest link $\min_i R_i$ |
| `L06` | `56` | `S2` | `L06FormalComputableLinearType` | `T1` | `Epistemic` | `[T1, T4, T5]` | `(1,2,0)` | Rice undecidability $\to$ LL(1) syntactic `judge()` and Girard linear/affine resource types |
| `L07` | `57` | `S2` | `L07ProbabilityCredenceMeasurement` | `T4` | `Vital` | `[T1, T2, T4]` | `(2,1,0)` | Walley imprecise credences $[\underline{P},\overline{P}]$ = Manski bounds and Stevens scale invariance |
| `L08` | `58` | `S2` | `L08MetrologicalInfoGeometric` | `T3` | `Epistemic` | `[T3, T4, T6]` | `(1,1,1)` | BIPM GUM Type A vs. Type B uncertainty and Bregman Pythagorean CORP-PAV projection |
| `L09` | `59` | `S2` | `L09ExperimentalSamplingCausal` | `T2` | `Epistemic` | `[T1, T2, T3]` | `(3,0,0)` | Fisher randomization + Kish $\mathrm{DEFF} = 1 + (m-1)\rho_{\mathrm{cl}}$ and Simonsohn multiverse curve |
| `L10` | `60` | `S2` | `L10CryptographicZeroKnowledgeAdversarial` | `T7` | `Governance` | `[T2, T6, T7]` | `(1,0,2)` | Kerckhoffs–Shannon secrecy, Dolev–Yao adversary, and zero-knowledge soundness = *e-value* |
| `L11` | `61` | `S3` | `L11BiochemicalKineticMolecular` | `T2` | `Epistemic` | `[T2, T5, T6]` | `(1,1,1)` | Hopfield–Ninio kinetic proofreading $\varepsilon_1^N$ = *e-process* and Michaelis–Menten = Kingman queue |
| `L12` | `62` | `S3` | `L12MorphogeneticDevelopmental` | `T5` | `Vital` | `[T1, T5, T6]` | `(1,1,1)` | Waddington chreod / Hsp90 capacitance = Canguilhem–Holling basin and Itô evolutionary drain |
| `L13` | `63` | `S3` | `L13ImmunologicalSelfGovernance` | `T7` | `Governance` | `[T2, T5, T7]` | `(1,1,1)` | Two-signal TCR+CD28 = List–Pettit conjunctive veto and Treg/CTLA-4/PD-1 = $S3^*$ algedonic brake |
| `L14` | `64` | `S3` | `L14TaxonomySystematicsOntology` | `T4` | `Vital` | `[T1, T3, T4]` | `(2,1,0)` | Hennig cladistic apomorphy vs. homoplasy and Bowker–Star infrastructural classification torque |
| `L15` | `65` | `S3` | `L15StratigraphyPaleobiologyDeepTime` | `T1` | `Epistemic` | `[T1, T3, T5]` | `(2,1,0)` | Signor–Lipps fossil truncation = archival tail censorship and Raup–Sepkoski extinction $\hat{\alpha}$ |
| `L16` | `66` | `S4` | `L16PharmacologicalClinicalDiagnostic` | `T4` | `Vital` | `[T2, T4, T5]` | `(1,2,0)` | Pauker–Kassirer $p^* = C/(\eta L)$, therapeutic index $\mathrm{TI} = \mathrm{TD}_{50}/\mathrm{ED}_{50}$, and sepsis Golden Hour |
| `L17` | `67` | `S4` | `L17EpidemiologyOutbreakSurveillance` | `T3` | `Epistemic` | `[T2, T3, T5]` | `(2,1,0)` | Kermack–McKendrick $R_0 = 1$ transcritical threshold and serial interval vs. tracing queue $\mathrm{TTR}$ |
| `L18` | `68` | `S4` | `L18PharmacovigilanceRegulatoryToxicology` | `T2` | `Epistemic` | `[T2, T3, T7]` | `(2,0,1)` | Bradford Hill coherence, FAERS spontaneous reporting $\rho_{R,Y}$ bias, and recall veto gate |
| `L19` | `69` | `S4` | `L19MetascienceScientometricsReplication` | `T6` | `Governance` | `[T2, T3, T6]` | `(2,0,1)` | Ioannidis $\mathrm{PPV}$ under file-drawer bias, Smaldino–McElreath metric selection, and *e-BH* FDR control |
| `L20` | `70` | `S4` | `L20AcousticAestheticTragicPedagogical` | `T1` | `Epistemic` | `[T1, T4, T6]` | `(1,1,1)` | Pythagorean comma $(3/2)^{12} \ne 2^7$, tragic *hamartía* vs. $T_4$, and Vygotsky ZPD fractional probe |
| `L21` | `71` | `S5` | `L21LinguisticEvidentialPragmatic` | `T7` | `Governance` | `[T3, T6, T7]` | `(1,0,2)` | Tuyuca/Quechua obligatory morphological evidentials vs. March–Simon uncertainty absorption |
| `L22` | `72` | `S5` | `L22PhilologicalEpigraphicTextual` | `T3` | `Epistemic` | `[T2, T3, T4]` | `(2,1,0)` | Lachmann *stemma codicum* = cluster $\mathrm{DEFF}$ and Leiden epigraphic `[...]` = Manski bounds |
| `L23` | `73` | `S5` | `L23TaphonomicHistoriographical` | `T5` | `Vital` | `[T3, T5, T7]` | `(1,1,1)` | Efremov taphonomic filter = Meng $\rho_{R,Y}$ + Chao $\hat{n}_{00}$ and `TemporalVault` against hindsight |
| `L24` | `74` | `S5` | `L24CriticalArchivalTransitionalJustice` | `T1` | `Epistemic` | `[T1, T3, T7]` | `(2,0,1)` | Fricker testimonial justice, Trouillot archival silencing, and Ball capture-recapture $\hat{n}_{00}$ |
| `L25` | `75` | `S5` | `L25MediaEcologyInformationRetrieval` | `T6` | `Governance` | `[T1, T3, T6]` | `(2,0,1)` | Innis bias of communication and recommender feedback loop = performative $\rho_{R,Y}$ amplification |
| `L26` | `76` | `S6` | `L26SequentialDecisionBanditRealOptions` | `T5` | `Vital` | `[T1, T2, T5]` | `(2,1,0)` | Wald SPRT boundaries, Gittins bandit index, and Dixit–Pindyck irreversible option value |
| `L27` | `77` | `S6` | `L27GameTheoryStrategicAdversarial` | `T2` | `Epistemic` | `[T1, T2, T6]` | `(2,0,1)` | Perdomo performative stability vs. strategic manipulation and minimax adversarial robustness |
| `L28` | `78` | `S6` | `L28MacroFinanceEconometricsActuarial` | `T6` | `Governance` | `[T2, T4, T6]` | `(1,1,1)` | Cramér–Lundberg ruin, Artzner coherent $\mathrm{CVaR}$ vs. VaR, Lucas critique, and Goodhart 1975 |
| `L29` | `79` | `S6` | `L29OfficialStatisticsForensicAccounting` | `T3` | `Epistemic` | `[T3, T4, T7]` | `(1,1,1)` | Pacioli double-entry conservation invariant, Desrosières census ontology, and Sen poverty axioms |
| `L30` | `80` | `S6` | `L30CorporateGovernancePrincipalAgent` | `T1` | `Epistemic` | `[T1, T6, T7]` | `(1,0,2)` | Holmström–Milgrom multitask distortion, Hurwicz incentive compatibility, and Innes liability |
| `L31` | `81` | `S7` | `L31CriminalProofForensicEpistemology` | `T2` | `Epistemic` | `[T1, T2, T7]` | `(2,0,1)` | Blackstone asymmetric error loss, likelihood-ratio forensic weight, and chain-of-custody seal |
| `L32` | `82` | `S7` | `L32EvidentiaryTortConstitutional` | `T7` | `Governance` | `[T1, T4, T7]` | `(1,1,1)` | Learned Hand $B < P \cdot L$ with tail amplification and constitutional supermajority as hysteresis |
| `L33` | `83` | `S7` | `L33DistributedConsensusSre` | `T5` | `Vital` | `[T5, T6, T7]` | `(0,1,2)` | FLP/CAP theorem, metastable retry storms at $\rho \to 1$, and SRE error-budget freeze gate |
| `L34` | `84` | `S7` | `L34HighHazardInfrastructureSafety` | `T4` | `Vital` | `[T4, T5, T7]` | `(0,2,1)` | Perrow interactive complexity $\times$ tight coupling, Rasmussen drift, and hardwired SCRAM veto |
| `L35` | `85` | `S7` | `L35PolycentricCommonsBiospheric` | `T6` | `Governance` | `[T3, T5, T6]` | `(1,1,1)` | Ostrom 8 polycentric principles, Rockström planetary boundaries, and Leontief triadic floor |

---

## III. Machine-Checked Anchors (`multilayer-anchors`)

```text
multilayer-anchors
crates/stochology/src/grammar.rs::CircuitTransit
crates/stochology/src/grammar.rs::GuardSite::transit
crates/stochology/src/grammar.rs::GuardSite::is_bijection_over_circuit_transits
crates/stochology/src/multilayer.rs::KnowledgeStratum
crates/stochology/src/multilayer.rs::KnowledgeStratum::orders
crates/stochology/src/multilayer.rs::KnowledgeStratum::canonical_guard_site
crates/stochology/src/multilayer.rs::KnowledgeStratum::from_canonical_guard_site
crates/stochology/src/multilayer.rs::KnowledgeStratum::covers_all_seven_guard_sites
crates/stochology/src/multilayer.rs::KnowledgeStratum::has_distinct_primary_guard_sites
crates/stochology/src/multilayer.rs::TriadicExposureRegime
crates/stochology/src/multilayer.rs::TriadicExposureRegime::uncoupled_collapse
crates/stochology/src/multilayer.rs::KnowledgeOrder
crates/stochology/src/multilayer.rs::KnowledgeOrder::guard_sites
crates/stochology/src/multilayer.rs::KnowledgeOrder::triadic_occupancy
crates/stochology/src/multilayer.rs::KnowledgeOrder::exposure_regime
crates/stochology/src/multilayer.rs::Graph1Node
crates/stochology/src/multilayer.rs::MultilayerGraph1
crates/stochology/src/multilayer.rs::MultilayerGraph1::canonical
crates/stochology/src/multilayer.rs::MultilayerGraph1::ancestors_of
crates/stochology/src/multilayer.rs::MultilayerGraph1::every_ancestor_cone_spans_all_three_axes
crates/stochology/src/multilayer.rs::Graph2Node
crates/stochology/src/multilayer.rs::MultilayerGraph2
crates/stochology/src/multilayer.rs::MultilayerGraph2::canonical
crates/stochology/src/multilayer.rs::MultilayerGraph2::is_bijection_over_35_guard_combinations
crates/stochology/src/multilayer.rs::MultilayerGraph2::every_stratum_covers_all_seven_sites
crates/stochology/src/multilayer.rs::MultilayerGraph2::is_balanced_five_orders_per_primary_site
crates/stochology/src/multilayer.rs::LayerTriadReading
crates/stochology/src/multilayer.rs::MultilayerHomomorphism
crates/stochology/src/multilayer.rs::MultilayerHomomorphism::evaluate
```
