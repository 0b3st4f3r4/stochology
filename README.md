<p align="center">
  <img src="book-src/logo.svg" alt="Stochology — Theory &amp; Ethics of Aim under Uncertainty" />
</p>

# Stochology — Theory & Ethics of Aim (*Stókhos*) under Uncertainty

<p align="center">
  <a href="https://stochology.org"><img src="https://img.shields.io/badge/Textbook-stochology.org-0f766e.svg" alt="Textbook" /></a>
  <a href="LICENSE-TEXT"><img src="https://img.shields.io/badge/Manuscript-CC%20BY%204.0-2563eb.svg" alt="Manuscript License: CC BY 4.0" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/Theme-MIT-059669.svg" alt="Theme License: MIT" /></a>
</p>

**Stochology** (from Greek **στόχος, *stókhos***, aim or conjecture, and **λόγος, *lógos***, reasoned account) is an integrative framework and transversal research program for the theory and ethics of aim (*stókhos*, from Plato's *Philebus* 55e–56c, Hippocratic *sēmeîa*, *kairós*, and *mētis*, Galenic *stokhastikē tékhnē*, and Antipater of Tarsus's archer to Jakob Bernoulli's *Ars Conjectandi*) in judgment and decision-making under structural uncertainty when action must precede the outcome. It does not claim to be a standalone positive science of mechanical measure (*metrētikē* / *akríbeia*), and it does not claim a new unit of analysis — Geoffrey Vickers already treated institutional regulation as the work of an appreciative system that judges a situation before it decides any part of it (Vickers 1965), Beer argued the same from viable systems, Simon from bounded rationality, and the institutional economists from the irreducibility of rules and organizations. What it claims is narrower: the **full mediation circuit across world, archive, and institution**, argued against the sources that established each of its parts.

Emerging from the three foundational ethical refusals of [Chapter 27](book-src/27-what-judging-is.md), formalized across [Chapters 29](book-src/29-aim-and-the-three-planes.md) and [41](book-src/41-the-triad-and-the-minimum.md), given its syntax layer in [Appendix B.1](book-src/appendix-b-architecture-verification.md), its pre-shot reflexive core in Chapter 41, and its disciplinary topology in Part V, the program is presented as a 86-chapter five-part textbook (`book-src/`, `00`–`85`) organized around the **Sequential Agent–World–Institution Loop** and the **Three Canonical Mathematical Operators**, with the political philosophy in [Chapter 20](book-src/20-legitimacy-and-institutions.md).

Every number in the manuscript is either a value a cited source reports or a result of a derivation printed beside its own algebra. Nothing is quoted from a run, a dataset processing pipeline, or a simulation whose parameters the reader cannot inspect.

---

## 1. The Nuclear Triad, Three Collapse Propositions, and Three Canonical Operators

Stochology unites three historical lineages (**Epistemological/Statistical**, **Physical/Cybernetic**, and **Critical/Ethical**) into a single **Nuclear Triad** of three non-substitutable safeguards ($\sigma \to 0^+$; whether doubled physical reserve buffers can partially compensate a weak axis in mild interior regimes without an active absorbing barrier is held open as the pre-registered empirical test $F\_4$ of [Chapter 50](book-src/50-methodological-families-reflexivity.md)):

$$
\mathcal{T}\_{\mathrm{Stochology}} = \bigl(\text{Epistemic Margin},\mkern5mu \text{Vital Margin},\mkern5mu \text{Institutional Governance}\bigr), \qquad M\_{\mathrm{Leontief}} = \min\bigl(S\_{\mathrm{epi}}, S\_{\mathrm{vital}}, S\_{\mathrm{gov}}\bigr)
$$

Each axis is governed by a **Canonical Mathematical Operator** and carries a **Collapse Proposition** for its removal:

| Axis | Canonical Operator | Missing-Axis Pathology |
| :--- | :--- | :--- |
| **Axis 1** ($S\_{\mathrm{epi}}$, Filter I: Ch. 43–45) | **Operator I — Partial-Identification Set & Effective Sample Contraction:** $\mathcal{I}\_{\mathrm{epi}}(\mathcal{D}\_t; \alpha)$ at $n\_{\mathrm{eff}}^{\mathrm{var}} = n/\mathrm{DEFF}$, $\frac{1}{n\_{\mathrm{eff}}(\bar{\rho})} = \frac{1}{n\_{\mathrm{eff}}^{\mathrm{var}}} + \left(\frac{1-f}{f}\right)\bar{\rho}^2$, $V = \frac{n\_{\mathrm{eff}}(\bar{\rho})}{n\_{\mathrm{eff}}(\bar{\rho}) + n\_0}$ | **Proposition 1 — Blind Undersizing & Net Reserve Erosion** ($\neg S\_{\mathrm{epi}}$): reserves are sized on a false point certainty that ignores non-random missingness ($\rho\_{R,Y} \ne 0$) and cluster design effects ($\mathrm{DEFF}$), so the buffer is anchored to a downward-biased baseline and erodes below its opening level. |
| **Axis 2** ($S\_{\mathrm{vital}}$, Filter II: Ch. 46–47) | **Operator II — Non-Ergodic Reserve & Survival-Time Barrier:** $dR\_t = \bigl(u\_t - D\_t(\alpha\_{\mathrm{Hill}}) - \Delta\_J - \frac{1}{2}f\_t^2\sigma^2\bigr)dt$ together with $\mathrm{TTS}\_{\mathrm{survival}} \ge \mathrm{TTR}\_{\mathrm{recovery}}$ | **Proposition 2 — Impotent Lucidity Before Ruin** ($\neg S\_{\mathrm{vital}}$): the analyst knows the exact width of uncertainty, yet the system operates at full utilization or excess leverage with $\mathrm{TTS}\_{\mathrm{survival}} < \mathrm{TTR}\_{\mathrm{recovery}}$ and is absorbed at the ruin barrier. |
| **Axis 3** ($S\_{\mathrm{gov}}$, Filter III: Ch. 48–49) | **Operator III — Bitemporal Accountability & Performative Integrity Gate:** $\mathcal{O}\_{\mathrm{gov}}(\pi\_t, \mathcal{I}\_t) = \min\bigl(g\_{\text{ex-ante}, t}, g\_{\mathrm{kairos}, t}, g\_{\text{ex-post}, t}\bigr)$ | **Proposition 3 — Cynical Capture and Pillaging** ($\neg S\_{\mathrm{gov}}$): epistemic doubt is weaponized as an alibi for paralysis while common vital reserves are pillaged under limited-liability moral hazard, proxy gaming, and testimonial discounting of frontline warnings. |

The triadic verdict is **diagnostic, not a control law**: it reads a finished trajectory by taking the temporal infimum over each axis's monotone chart coordinates $S\_a = \varphi\_a(s\_a)$ (ordinally box-invariant per axis under Stevens recoding, with Lebesgue volumes conditional on the declared chart via its Jacobian) and reports which plane failed, with no decision rule derived from the aggregate. The operator that acts is the per-step canonical one, never the trajectory fold.

---

## 2. The Seven Guard Sites ($3 + 3 + 1 = 7$) and the 35-Layer Multilayer Knowledge Graph ($\mathcal{G}\_1 \leftrightarrow \mathcal{G}\_2$)

When first-order facts about the external world (*skopós*) are unknown, the infinite regress of higher-order uncertainty terminates only on **reflexive invariants of the agent's own mediation circuit (*télos*)** ([Chapter 41](book-src/41-the-triad-and-the-minimum.md)). Three trunk questions unfold into the **Seven Guard Sites ($3/2/2$)**, corresponding to the $3 + 3 + 1 = 7$ directed transits and integral closure across the three planes (*World*, *Archive*, *Institution*) and paired by recorded designation—not derived from the table—with the **Seven Knowledge Strata (`S1`–`S7`)**:

| Guard Site | Triad Axis ($3/2/2$) | Directed Circuit Transit | Primary Plane ($2/2/2+1$) | Canonical Knowledge Stratum | Primary Bottleneck Orders ($7 \times 5 = 35$) |
| :--- | :---: | :--- | :---: | :--- | :--- |
| **`T1Framing`** | $S\_{\mathrm{epi}}$ | *Institution → Archive* (Reflexive Lock 1) | *Institution* | `S2` Formal-Metrological | `L06, L15, L20, L24, L30` |
| **`T2Severity`** | $S\_{\mathrm{epi}}$ | *Archive → World* (Reflexive Lock 2) | *Archive* | `S4` Clinical-Formative | `L09, L11, L18, L27, L31` |
| **`T3Archive`** | $S\_{\mathrm{epi}}$ | *World → Archive* (Direct Transit 1) | *World* | `S5` Semiotic-Archival | `L04, L08, L17, L22, L29` |
| **`T4Formalization`** | $S\_{\mathrm{vital}}$ | *Archive → Institution* (Direct Transit 2) | *Archive* | `S3` Living-Evolutionary | `L05, L07, L14, L16, L34` |
| **`T5Temporal`** | $S\_{\mathrm{vital}}$ | *World → Institution* (Reflexive Lock 3) | *World* | `S1` Physical-Dynamical | `L01, L12, L23, L26, L33` |
| **`T6Performative`** | $S\_{\mathrm{gov}}$ | *Institution → World* (Direct Transit 3) | *Institution* | `S6` Strategic-Economic | `L03, L19, L25, L28, L35` |
| **`T7Liability`** | $S\_{\mathrm{gov}}$ | *World ↔ Archive ↔ Institution* (Integral) | *Closure* | `S7` Constitutional-Biospheric | `L02, L10, L13, L21, L32` |

Taking all $\binom{7}{3} = 35$ unique 3-site combinations of `{T1, …, T7}` induces a bijective **35-layer directed graph of human knowledge ($\mathcal{G}\_1$)** and its dual **Stochological Guard Graph ($\mathcal{G}\_2$)**, mapped 1-to-1 onto Part V chapters `51`–`85`:

- **Horizontal stratification ($7 \times 5 = 35$):** seven strata of five orders each; within every stratum the union of guard triples covers all seven guard sites, and the five orders carry five pairwise-distinct primary guard sites.
- **Vertical exposure stratification ($12 + 9 + 9 + 4 + 1 = 35$):** by Vandermonde's identity over the $3/2/2$ partition, the triads decompose into $12$ trans-triadic keystones, $9$ governance-exposed, $9$ vital-exposed, $4$ epistemic-exposed and $1$ pure-epistemic-concentration class (where axial exposure records the absence of a local guard on that axis when isolated from upstream orders). A discipline whose local triad omits an axis when isolated is not merely incomplete: it is the regime in which the corresponding collapse proposition is expected to fire.
- The coupling along the ancestor cone $\mathrm{Anc}^\ast(L\_k)$ is read as a **Cumulative Leontief Homomorphism**, a model reading of the graph and not a measured quantity.

---

## 3. The Corpus: Six Layers of Argument ↔ Five-Part Textbook (`00`–`85`) ↔ Appendices

The argument is carried in six layers, and the manuscript is where each layer is argued and checked. A layer names what a block of chapters is doing; the chapter is where the claim, its source and its derivation actually are, and Appendix C.1 records which chapters carry which layer.

1. [Chapters 27](book-src/27-what-judging-is.md) and [40](book-src/40-institution-incentive-metric.md) — the ethical and phenomenological refusals;
2. [Chapters 29](book-src/29-aim-and-the-three-planes.md), [41](book-src/41-the-triad-and-the-minimum.md) and [50](book-src/50-methodological-families-reflexivity.md) — the three-plane circuit, the Nuclear Triad, the three collapse propositions, the seven guard sites, the four reflexive brakes and the four falsifiers;
3. [Appendix B.1](book-src/appendix-b-architecture-verification.md) — the syntax layer over trajectories: seven guard sites across three direct transits, three reflexive locks and one integral closure, four verdicts, and non-commutation of *télos* and *skópos*;
4. [Chapter 41](book-src/41-the-triad-and-the-minimum.md) — why the regress terminates on the agent's own circuit, the `3 → 7` inventory, and the three regimes of action in the dark;
5. [Part V](book-src/51-layer-micro-kinetic-sensorimotor-order.md) with [Appendix B.2](book-src/appendix-b-architecture-verification.md) and Table B.1 — the $\binom{7}{3} = 35$ bijection and the $7 \times 5 = 35$ stratification;
6. [Chapters 20](book-src/20-legitimacy-and-institutions.md), [40](book-src/40-institution-incentive-metric.md), [49](book-src/49-moral-luck-apt-agency-institutions.md) and [85](book-src/85-layer-polycentric-commons-urban-biospheric-order.md) — political philosophy, institutional cybernetics, and self-governance of immanent polycentric commons.

The textbook ([`book-src/`](book-src/), table of contents in [`SUMMARY.md`](book-src/SUMMARY.md)) spans **86 chapters (`00`–`85`) and five appendices** in five parts:

- **Preface (`00`) & Part I — Thinking Under Uncertainty (`01`–`08`):** deciding without knowing, the average that hides its tail, the number seen least often, outcome luck, the denominator nobody quotes, the missing who leave the table, and the measure that stops measuring once it pays.
- **Part II — The Bases (`09`–`26`):** mathematics (`09`–`15`), philosophy and epistemology (`16`–`20`), and natural science (`21`–`26`) — the apparatus the later parts presuppose.
- **Part III — Foundations of Judgment (`27`–`41`):** what judging is, probability as language, distribution and moment, the question before the data, measuring and erring, sample and archive, inference and evidence, decision and threshold, model and error, time and dependence, cause and counterfactual, institution and metric, and the triad at its minimum.
- **Part IV — The Framework (`42`–`50`):** the running case (*the community and the reserve*) carried through the sequential loop, Filter I ($S\_{\mathrm{epi}}$: framing, underdetermination, world-to-archive), Filter II ($S\_{\mathrm{vital}}$: asymmetry, heavy tails, irreversible time), Filter III ($S\_{\mathrm{gov}}$: performativity, moral luck, institutional design), and the triadic synthesis with its pre-registered falsifiers.
- **Part V — The 35 Orders of Knowledge (`51`–`85`):** thirty-five chapters carrying thirty-five orders across seven strata.
  - **Stratum I (`S1`, `51`–`55`, anchor `T5Temporal`):** `L01` Micro-Kinetic & Sensorimotor (`51`), `L02` Meso-Buffered Queue & Dispatch (`52`), `L03` Macro-Coupled Network Contagion (`53`), `L04` Geophysical Chaos & Data Assimilation (`54`), `L05` Continuum Fracture & Structural Engineering (`55`).
  - **Stratum II (`S2`, `56`–`60`, anchor `T1Framing`):** `L06` Formal Logic, Computability & Linear Types (`56`), `L07` Probability, Imprecise Credences & Measurement (`57`), `L08` Metrological & Information-Geometric (`58`), `L09` Experimental Design, Survey Sampling & Causal Inference (`59`), `L10` Cryptographic, Zero-Knowledge & Adversarial Verification (`60`).
  - **Stratum III (`S3`, `61`–`65`, anchor `T4Formalization`):** `L11` Biochemical Kinetics & Molecular Proofreading (`61`), `L12` Morphogenetic & Developmental Canalization (`62`), `L13` Immunological Self-Governance (`63`), `L14` Taxonomy, Systematics & Formal Ontology (`64`), `L15` Stratigraphy, Paleobiology & Deep Time (`65`).
  - **Stratum IV (`S4`, `66`–`70`, anchor `T2Severity`):** `L16` Pharmacological & Clinical Diagnostic (`66`), `L17` Epidemiology & Outbreak Surveillance (`67`), `L18` Pharmacovigilance & Regulatory Toxicology (`68`), `L19` Metascience, Scientometrics & Replication (`69`), `L20` Acoustic, Aesthetic, Tragic & Pedagogical (`70`).
  - **Stratum V (`S5`, `71`–`75`, anchor `T3Archive`):** `L21` Linguistic Evidentiality & Organizational Pragmatics (`71`), `L22` Philological, Epigraphic & Textual Criticism (`72`), `L23` Taphonomic, Archaeological & Historiographical (`73`), `L24` Critical Archival & Transitional Justice (`74`), `L25` Media Ecology & Information Retrieval (`75`).
  - **Stratum VI (`S6`, `76`–`80`, anchor `T6Performative`):** `L26` Sequential Decision, Bandits & Real Options (`76`), `L27` Game Theory & Strategic Classification (`77`), `L28` Macro-Finance, Econometrics & Actuarial Ruin (`78`), `L29` Official Statistics & Forensic Accounting (`79`), `L30` Corporate Governance & Principal-Agent Design (`80`).
  - **Stratum VII (`S7`, `81`–`85`, anchor `T7Liability`):** `L31` Criminal Proof & Forensic Epistemology (`81`), `L32` Evidentiary, Tort & Constitutional Law (`82`), `L33` Distributed Consensus & SRE (`83`), `L34` High-Hazard Nuclear & Hydraulic Safety (`84`), `L35` Polycentric Commons & Biospheric Stewardship (`85`).
- **Appendices:** [`A — Mathematical Reference`](book-src/appendix-a-mathematics.md), [`B — The Grammar of Aim and the Multilayer Architecture`](book-src/appendix-b-architecture-verification.md), [`C — Bibliography`](book-src/appendix-c-sources.md), [`D — Claim Ledger`](book-src/appendix-d-claim-ledger.md), [`E — Lexicon and the Architecture Map`](book-src/appendix-e-lexicon.md).

---

## 4. How to Read It

1. **If you want the argument before the apparatus:** Part I (`01`–`08`), then Part IV (`42`–`50`).
2. **If you want the mathematics first:** Part II (`09`–`15`), then the appendices.
3. **If you are picking one chapter:** Part V is self-contained by stratum; each chapter opens with the layer it carries, the guard triple it occupies, and the order it argues.
4. **If you want the whole program in one pass:** Chapters 29 and 41, then Appendix B.

Every chapter keeps the same spine: three stated outcomes, the worked sections, and one closing bridge to the next chapter; the exercises are retired pending an interactive companion. No chapter requires a computation the chapter does not show.

---

## 5. Claim Discipline

A claim in this manuscript carries one of two warrants, and it names that warrant in the sentence that carries it:

1. **Literature:** an external empirical or conceptual finding quoted with its source and, where the source reports a value, with the value exactly as the source states it (for example Wong et al. 2021 on the Epic Sepsis Model's false-alarm rate, or Zheng & Meister 2025 on the human behavioral and sensory-motor throughput limit).
2. **Derivation:** a result the reader can reproduce from the algebra displayed beside it — a definition, an identity, a limit, or a symbolic bound.

The manuscript carries no bracketed provenance label and no value produced by a pipeline the reader cannot inspect. A number whose warrant the sentence leaves unstated is an invented number, whichever chapter it sits in. The ledger in [Appendix D](book-src/appendix-d-claim-ledger.md) records the literature-warranted claims with the source that carries each one, and [Appendix C](book-src/appendix-c-sources.md) holds the consolidated bibliography.

---

## 6. Citation, License, Contribution

Citation metadata lives in [`CITATION.cff`](CITATION.cff). The manuscript text is released under [Creative Commons Attribution 4.0 International (CC BY 4.0)](LICENSE-TEXT), and the theme assets under the [MIT License](LICENSE). The one vendored third-party component — the typesetting payload under `theme/mathjax/` — is Apache-2.0 and its notice travels with it in [`THIRD-PARTY-NOTICES.md`](THIRD-PARTY-NOTICES.md). Editorial and structural rules that any contribution must satisfy are in [`AGENTS.md`](AGENTS.md); the short version is that a change may argue with a source, but it may not invent a number, cite a source that does not say what the sentence needs, or point at machinery the reader cannot open.
