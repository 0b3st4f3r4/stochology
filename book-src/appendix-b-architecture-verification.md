# Appendix B — The Grammar of Aim and the Multilayer Architecture

<figure class="stoch-figure stoch-logo-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="660" height="132" viewBox="0 0 660 132" fill="none" role="img" aria-label="Stochology Appendix B — The Grammar Of Aim And The Multilayer Architecture">
  <defs>
    <linearGradient id="apb-ring" x1="2" y1="78" x2="386" y2="78" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="38%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="68%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <radialGradient id="apb-glow" cx="50%" cy="50%" r="50%">
      <stop offset="0%" stop-color="#fbbf24" stop-opacity="0.24"/>
      <stop offset="50%" stop-color="#38bdf8" stop-opacity="0.14"/>
      <stop offset="82%" stop-color="#e11d48" stop-opacity="0.08"/>
      <stop offset="100%" stop-color="#090e1a" stop-opacity="0"/>
    </radialGradient>
    <linearGradient id="apb-triad-stroke" x1="18" y1="24" x2="110" y2="104" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.85"/>
      <stop offset="50%" stop-color="var(--logo-gold, #f59e0b)" stop-opacity="0.90"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.85"/>
    </linearGradient>
    <linearGradient id="apb-fill" x1="22" y1="30" x2="106" y2="98" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)" stop-opacity="0.26"/>
      <stop offset="48%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.28"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.26"/>
    </linearGradient>
    <linearGradient id="apb-ink-curve" x1="20" y1="66" x2="108" y2="66" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-ink, #0f172a)"/>
      <stop offset="50%" stop-color="var(--logo-sub, #1d4ed8)"/>
      <stop offset="100%" stop-color="var(--logo-ink, #0f172a)"/>
    </linearGradient>
    <radialGradient id="apb-pupil" cx="45%" cy="42%" r="58%">
      <stop offset="0%" stop-color="#020617"/>
      <stop offset="68%" stop-color="#0f172a"/>
      <stop offset="100%" stop-color="#451a03"/>
    </radialGradient>
    <linearGradient id="apb-gold-mira" x1="54" y1="48" x2="84" y2="82" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#fef08a"/>
      <stop offset="45%" stop-color="#fbbf24"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <radialGradient id="apb-gold-core" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#ffffff"/>
      <stop offset="35%" stop-color="#fde047"/>
      <stop offset="100%" stop-color="#d97706"/>
    </radialGradient>
    <radialGradient id="apb-node-well" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="var(--logo-well-hi, var(--elevated-bg, #ffffff))"/>
      <stop offset="100%" stop-color="var(--logo-node-bg, #e2e8f0)"/>
    </radialGradient>
    <linearGradient id="apb-ring-epi" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#7dd3fc"/>
      <stop offset="50%" stop-color="#2563eb"/>
      <stop offset="100%" stop-color="#1e3a8a"/>
    </linearGradient>
    <radialGradient id="apb-core-epi" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#bae6fd"/>
      <stop offset="45%" stop-color="#3b82f6"/>
      <stop offset="100%" stop-color="#1d4ed8"/>
    </radialGradient>
    <linearGradient id="apb-ring-vital" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#6ee7b7"/>
      <stop offset="50%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#065f46"/>
    </linearGradient>
    <radialGradient id="apb-core-vital" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#a7f3d0"/>
      <stop offset="45%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#047857"/>
    </radialGradient>
    <linearGradient id="apb-ring-gov" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#fda4af"/>
      <stop offset="50%" stop-color="#e11d48"/>
      <stop offset="100%" stop-color="#881337"/>
    </linearGradient>
    <radialGradient id="apb-core-gov" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#fecdd3"/>
      <stop offset="45%" stop-color="#f43f5e"/>
      <stop offset="100%" stop-color="#be123c"/>
    </radialGradient>
  </defs>
  <g>
    <circle cx="64" cy="66" r="48" fill="url(#apb-glow)"/>
    <polygon points="64,22 102,44 102,88 64,110 26,88 26,44" fill="url(#apb-fill)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.6" stroke-linejoin="round"/>
    <circle cx="64" cy="66" r="24" stroke="var(--logo-green, #10b981)" stroke-width="2.2" stroke-dasharray="3.5 2.5"/>
    <circle cx="26.00" cy="44.00" r="6.38" fill="url(#apb-node-well)" stroke="url(#apb-ring-epi)" stroke-width="2.36"/>
    <circle cx="26.00" cy="44.00" r="2.74" fill="url(#apb-core-epi)"/>
    <circle cx="102.00" cy="44.00" r="6.38" fill="url(#apb-node-well)" stroke="url(#apb-ring-vital)" stroke-width="2.36"/>
    <circle cx="102.00" cy="44.00" r="2.74" fill="url(#apb-core-vital)"/>
    <circle cx="64.00" cy="110.00" r="6.38" fill="url(#apb-node-well)" stroke="url(#apb-ring-gov)" stroke-width="2.36"/>
    <circle cx="64.00" cy="110.00" r="2.74" fill="url(#apb-core-gov)"/>
    <g>
      <circle cx="64.00" cy="66.00" r="6.96" fill="var(--logo-node-bg, #e2e8f0)"/>
      <circle cx="64.00" cy="66.00" r="6.02" fill="url(#apb-pupil)" stroke="url(#apb-gold-mira)" stroke-width="2.26"/>
      <line x1="54.98" y1="66.00" x2="60.05" y2="66.00" stroke="url(#apb-gold-mira)" stroke-width="2.26" stroke-linecap="round"/>
      <line x1="67.95" y1="66.00" x2="73.02" y2="66.00" stroke="url(#apb-gold-mira)" stroke-width="2.26" stroke-linecap="round"/>
      <line x1="64.00" y1="56.98" x2="64.00" y2="62.05" stroke="url(#apb-gold-mira)" stroke-width="2.26" stroke-linecap="round"/>
      <line x1="64.00" y1="69.95" x2="64.00" y2="75.02" stroke="url(#apb-gold-mira)" stroke-width="2.26" stroke-linecap="round"/>
      <circle cx="64.00" cy="66.00" r="2.07" fill="url(#apb-gold-core)"/>
    </g>
  </g>
  <g transform="translate(148, 0)">
    <text x="2" y="27" fill="var(--logo-sub, #1d4ed8)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.2">STOCHOLOGY · TECHNICAL REFERENCE</text>
    <text x="498" y="27" text-anchor="end" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.0">APPENDIX B · VERIFICATION</text>
    <text x="0" y="59" fill="var(--logo-ink, #0f172a)" font-family="system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif" font-size="22.2" font-weight="800" letter-spacing="1.35">ARCHITECTURE AND VERIFICATION</text>
    <line x1="2" y1="78" x2="384" y2="78" stroke="url(#apb-ring)" stroke-width="1.6" stroke-opacity="0.82"/>
    <circle cx="400" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="415" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.0"/>
    <circle cx="415" cy="78" r="1.6" fill="var(--logo-blue, #2563eb)"/>
    <circle cx="430" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="445" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="460" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-green, #10b981)" stroke-width="2.0"/>
    <circle cx="460" cy="78" r="1.6" fill="var(--logo-green, #10b981)"/>
    <circle cx="475" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="490" cy="78" r="6.2" stroke="var(--logo-gold, #f59e0b)" stroke-width="1.5" stroke-dasharray="2.2 1.6"/>
    <circle cx="490" cy="78" r="4.0" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-crimson, #e11d48)" stroke-width="2.2"/>
    <circle cx="490" cy="78" r="1.8" fill="var(--logo-gold, #f59e0b)"/>
    <text x="2" y="104" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.8" font-weight="600" letter-spacing="0.65">GRAMMAR OF AIM · SEVEN GUARD SITES · THIRTY-FIVE ORDERS</text>
  </g>
  </svg>
</figure>

Two formal layers carry the framework's architecture. A syntax layer decides whether a trajectory is admissible; a combinatorial layer fixes the thirty-five orders of knowledge across seven strata. Both are argued in the chapters — the grammar across Chapters 06, 29, 41, 42, 50, 51 and 56, the multilayer graph in Chapter 09 and across Part V — and this appendix is their reference statement. It gives the alphabet, the productions, the decision procedure and the combinatorial identities, and leaves the argument for each to the chapter that carries it. Appendix E defines the terms in which either layer is discussed. In scope, the single-plane statistical, dynamical and institutional theorems belong to the cited literature sources; what this coupled grammar and combinatorial layer supply are the acceptance predicate, non-commutation, and seven-site closure, without which the framework would be merely a pedagogical anthology.

---

## B.1 The Grammar of Aim — Syntax Layer and Acceptance Predicate

Stochology operates as a *transversal grammar* over trajectories, and the phrase is meant in its technical sense. Alphabet, productions and decision procedure are what the lexicon, the productions and the acceptance predicate below supply, and they sit over the entire mediation circuit rather than belonging to any single plane or stage.

The grammar owns **syntax and acceptance, never dynamics**. Its lexicon is three planes, the three axes of the Nuclear Triad, and the **seven guard sites**, one per constitutive tension, partitioned $3/2/2$ over $(S\_{\mathrm{epi}}, S\_{\mathrm{vital}}, S\_{\mathrm{gov}})$. A step of the loop is a **shot**: a control tape (*télos*) paired with a contingency tape (*skópos*). A guarded trajectory is a **sentence**, sealed over an append-only record: a derivation cannot be revised after its verdict and still presented as the same derivation.

The acceptance predicate returns exactly four verdicts, in a fixed order of evaluation: `Interior`, `Boundary` (inside the near-critical band above the declared thresholds), `Collapse` (carrying the proposition, the site and the step at which it occurs), and `Unparsable`. `Collapse` maps directly onto the Nuclear Triad's three non-substitutable safeguard axes, so the grammar inherits the $2^3 = 8$-cell safeguard cube and its three single-axis failure modes instead of inventing a parallel taxonomy. `Unparsable` is a mandatory **quarantine**. Were collapse a grammar's only failure mode, it would diagnose every phenomenon it cannot parse as a catastrophe — the precise dogmatic error the treatise spends three sections diagnosing. A non-finite margin is quarantined rather than compared, and an unguarded transit is not a weak derivation but a non-derivation.

Two generative regimes read the same productions in the opposite direction: an exhaustive enumeration over the finite guard grid, and the shortest single-site counterexample against the declared thresholds. Coverage interrogates the language. The adversarial regime interrogates the thresholds themselves, and it does so before any field study begins.

**The productions.** The skeleton grammar generates the guard-free trajectories, and guard annotation is a separate total map over them. The separation is what keeps syntax and semantics from contaminating one another. With $\langle\textit{Scope}\rangle$ and $\langle\textit{Scalar}\rangle$ as terminals drawn from the step being judged:

$$\begin{aligned}
\langle\textit{Sentence}\rangle &::= \langle\textit{Framing}\rangle\mkern5mu\langle\textit{Counting}\rangle\mkern5mu\langle\textit{Body}\rangle \\
\langle\textit{Framing}\rangle &::= \texttt{frame}\mkern5mu[\texttt{insulate} \mid \texttt{open}]\mkern5mu[\texttt{type3}] \\
\langle\textit{Counting}\rangle &::= \texttt{count}\mkern5mu\langle\textit{Interval}\rangle\mkern5mu[\texttt{deff}]\mkern5mu[\texttt{vintages}] \\
\langle\textit{Body}\rangle &::= \langle\textit{Decision}\rangle\mkern5mu[\langle\textit{Brake}\rangle]\mkern5mu(\langle\textit{Write}\rangle \mid \langle\textit{Read}\rangle)^{\ast}\mkern5mu[\texttt{halt}]\mkern5mu[\langle\textit{Step}\rangle^{\ast}] \\
\langle\textit{Step}\rangle &::= \langle\textit{Framing}\rangle\mkern5mu\langle\textit{Counting}\rangle\mkern5mu\langle\textit{Body}\rangle \\
\langle\textit{Decision}\rangle &::= \texttt{act}\mkern5mu\texttt{lex}(\texttt{survival}, \texttt{minimax\_regret}) \\
\langle\textit{Brake}\rangle &::= \texttt{brake}\mkern5mu\texttt{tts} \ge \texttt{ttr} \\
\langle\textit{Write}\rangle &::= \texttt{write}\mkern5mu\langle\textit{Scope}\rangle, \qquad
\langle\textit{Read}\rangle ::= \texttt{read}\mkern5mu\langle\textit{Scope}\rangle, \qquad
\langle\textit{Interval}\rangle ::= [\langle\textit{Scalar}\rangle, \langle\textit{Scalar}\rangle] \mkern5mu\mkern5mu (L \le U)
\end{aligned}$$

**Why the recursion terminates.** The only recursive production is $\langle\textit{Step}\rangle$, and its left corner is the institution plane. Under that pivot the parse of a trajectory is deterministic and linear: each step consumes exactly one transit triple, and two partial parses never compete for the same input. Without the pivot the same trajectory admits two parses, and two parses mean two verdicts on one history — the formal shadow of the committee and the auditor disagreeing about what happened while both read the same file. Termination is a time budget, not a depth limit. The recursion stops when the deliberation latency fits inside the survival clock, $\tau\_{\mathrm{bureaucratic}} \le \mathrm{TTS}\_{\mathrm{survival}}$, which is the third reflexive brake appearing as the reason the brake production exists at all.

**No threshold lives in the grammar.** Every threshold is a declared parameter of the assessment and never a property of the language. The grammar names thresholds; it does not own them. An acceptance threshold that becomes a published target is exactly the degeneration the fourth reflexive brake prohibits for the bottleneck minimum.

**The predicate, in order.** The order of evaluation carries the epistemology. Each failure carries its own name.

1. *Skeleton.* An empty derivation, a step missing its attested control tape, an out-of-scope write or delegation, an unauthorized read, an exceeded budget, or a control tape violating a declared invariant — inverted bounds $L > U$, non-positive provisioned capacity, negative reserve — is `Unparsable`, and the reason is named. An ill-formed input has no verdict; it does not have a bad one.
2. *Totality of the annotation.* Every one of the three axes must carry at least one guard reading. An unguarded transit is not a weak derivation, it is not a derivation.
3. *Finiteness.* A non-finite margin is `Unparsable` for a non-finite reading, because the alternative is a number that silently compares false against a threshold and passes every guard it touches. A missing number must be louder than a bad one.
4. *The Leontief fold.* The three axis scores are $s\_a = \min\_{i \in a} m\_i$ over the sites of axis $a$, and the sentence is accepted when the fold of those three scores clears the threshold. This fold is not the triadic diagnosis. It is taken over one step and over sealed margins, and by the non-commutation property below it is invariant to the contingency tape; the diagnosis is taken over a finished trajectory and is not. Identifying the two would contradict that property. So the grammar names its own fold and leaves the diagnosis to the level that folds trajectories rather than steps. The grammar does not re-derive the fold — it appeals to it — so no second rule can run beside it and diverge unnoticed.
5. *The frontier band.* A sentence clearing the fold while its weakest site sits inside a narrow band above the threshold is `Boundary`, and the verdict names that site with its margin. The width of the band is a parameter, not a discovery: accepted and comfortable are different verdicts.

**Non-commutation property.** The acceptance predicate reads the control tape and the guard margins, and never the contingency tape. Hence no admissible verdict is a function of *skopós*: an accurate hit cannot buy an interior verdict from margins that do not support it. Antipater's separation of *télos* and *skópōs* is thereby a theorem about the predicate rather than a remark about archers.

*Proof.* The predicate reads the control tape only through the declared epistemic bounds, the provisioned capacity and the reserve allocation, and it reads the contingency tape only into the seal, never into the verdict. Substituting any contingency tape while holding the control tape and the guard readings fixed therefore leaves every branch of the predicate untouched, so a sentence and its substitution return the same verdict. ∎

## B.2 The Multilayer Graph — 35 Orders, 7 Strata, 5 Exposure Regimes

The combinatorial layer carries Part V. It defines the seven strata $S\_1$–$S\_7$, each designated by one guard site — the designation recorded rather than derived — and the thirty-five orders of knowledge, each carrying its guard sites, its primary guard site, its stratum and its exposure regime; the parent layers of an order are printed where the order is argued, in the chapter blockquote that names the layers it builds on, so the root names none and the ancestry of the graph is read from the chapters. To these it adds the five axis-occupancy classes of Appendix A.6, $12 + 9 + 9 + 4 + 1 = 35$, the knowledge DAG with its ancestor cones, the guard-triple projection of that DAG, and the cumulative transfer map that carries a triad reading from a basal order to every order that depends on it.

Two bijections are what the layer is for. The $\binom{7}{3} = 35$ guard triples correspond one to one with the thirty-five orders — an identity of counting, since the triples exhaust the three-site combinations and each is carried by exactly one order. The seven strata are named by the seven guard sites, and that naming is a designation recorded here and in the preface, not a rule derived from the table. Each stratum is characterised by five of the seven sites, and each of the seven sites is primary in exactly five strata, so no stratum is without a site and no site without a stratum. The chapters of Part V argue each order's guard triple, order by order; of the seven anchors, only $S\_7$'s is argued, in Chapter 85. The triple an order carries is what the order is organised around, not the annotation of a single step. The predicate requires a reading on each of the three axes for a sentence, while an order may be carried by a triple that leaves one axis at zero, because the axis a triple omits is the order's local axial exposure regime—recording the absence of a local guard on that axis when isolated from upstream orders in $\mathcal{G}\_1$—and not a gap in a sentence's annotation.

### Table B.1 — Complete 35-Layer Multilayer Knowledge Graph Mapping (Part V)

| Order | Ch | Stratum | Knowledge Order | Primary Site | Unique Guard Triple | Signature (E,V,G) | Exposure Regime | Canonical Isomorphism ($\mathcal{G}\_1 \leftrightarrow \mathcal{G}\_2$) |
| :---: | :---: | :---: | :--- | :---: | :---: | :---: | :--- | :--- |
| `L01` | 51 | `S1` | Micro-kinetic sensorimotor order | `T5` | `{T1, T5, T7}` | `(1, 1, 1)` | Trans-Triadic Keystone | Two-tape separation (`télos`/`skopós`) and $\mathrm{TTS}\_{\mathrm{survival}} \ge \mathrm{TTR}\_{\mathrm{recovery}}$ |
| `L02` | 52 | `S1` | Meso-buffered queue order | `T7` | `{T4, T6, T7}` | `(0, 1, 2)` | Epistemic Axial Exposure | Kingman heavy-traffic hyperbola $\frac{\rho}{1-\rho}\frac{c\_a^2+c\_s^2}{2}$ and $10\text{ bits/s}$ filter |
| `L03` | 53 | `S1` | Macro-coupled topology order | `T6` | `{T4, T5, T6}` | `(0, 2, 1)` | Epistemic Axial Exposure | Motter–Lai cascade tolerance $\alpha\_{\mathrm{tol}} = (1-\rho)/\rho$ as reciprocal of Kingman |
| `L04` | 54 | `S1` | Geophysical chaos and data assimilation order | `T3` | `{T3, T4, T5}` | `(1, 2, 0)` | Governance Axial Exposure | Lorenz Lyapunov horizon $\tau\_L \sim \lambda\_1^{-1}\ln(\delta\_{\mathrm{tol}}/\varepsilon\_0)$ and 4D-Var/CRPS closure |
| `L05` | 55 | `S1` | Continuum fracture and structural order | `T4` | `{T2, T4, T7}` | `(1, 1, 1)` | Trans-Triadic Keystone | Griffith critical crack $a\_c$ as basin fold bifurcation and Weibull weakest link $\min\_i R\_i$ |
| `L06` | 56 | `S2` | Formal computable linear-type order | `T1` | `{T1, T4, T5}` | `(1, 2, 0)` | Governance Axial Exposure | Rice undecidability $\to$ a syntactic judging procedure and Girard linear/affine resource types |
| `L07` | 57 | `S2` | Probability, credence and measurement order | `T4` | `{T1, T2, T4}` | `(2, 1, 0)` | Governance Axial Exposure | Walley imprecise credences $[\underline{P},\overline{P}]$ = Manski bounds and Stevens scale invariance |
| `L08` | 58 | `S2` | Metrological, information-geometric order | `T3` | `{T3, T4, T6}` | `(1, 1, 1)` | Trans-Triadic Keystone | BIPM GUM Type A vs. Type B uncertainty and Bregman Pythagorean CORP-PAV projection |
| `L09` | 59 | `S2` | Experimental, sampling and causal order | `T2` | `{T1, T2, T3}` | `(3, 0, 0)` | Pure Epistemic Concentration | Fisher randomization + Kish $\mathrm{DEFF} = 1 + (m-1)\rho\_{\mathrm{cl}}$ and Simonsohn multiverse curve |
| `L10` | 60 | `S2` | Cryptographic, zero-knowledge, adversarial order | `T7` | `{T2, T6, T7}` | `(1, 0, 2)` | Vital Axial Exposure | Kerckhoffs–Shannon secrecy, Dolev–Yao adversary, and zero-knowledge soundness = test-side *e-value* |
| `L11` | 61 | `S3` | Biochemical kinetic and molecular order | `T2` | `{T2, T5, T6}` | `(1, 1, 1)` | Trans-Triadic Keystone | Hopfield–Ninio kinetic proofreading $\varepsilon\_1^N$ = *e-process* and Michaelis–Menten saturation $f = [S]/(K\_M + [S])$ ($\ne$ Kingman queue: finite-site ceiling vs. unbounded buffer divergence) |
| `L12` | 62 | `S3` | Morphogenetic and developmental order | `T5` | `{T1, T5, T6}` | `(1, 1, 1)` | Trans-Triadic Keystone | Waddington chreod / Hsp90 capacitance = Canguilhem–Holling basin and Itô evolutionary drain |
| `L13` | 63 | `S3` | Immunological self-governance order | `T7` | `{T2, T5, T7}` | `(1, 1, 1)` | Trans-Triadic Keystone | Two-signal TCR+CD28 = List–Pettit conjunctive veto and Treg/CTLA-4/PD-1 = $S3^\ast$ algedonic brake |
| `L14` | 64 | `S3` | Taxonomy, systematics and ontology order | `T4` | `{T1, T3, T4}` | `(2, 1, 0)` | Governance Axial Exposure | Hennig cladistic apomorphy vs. homoplasy and Bowker–Star infrastructural classification torque |
| `L15` | 65 | `S3` | Stratigraphy, paleobiology and deep-time order | `T1` | `{T1, T3, T5}` | `(2, 1, 0)` | Governance Axial Exposure | Signor–Lipps fossil truncation = archival tail censorship and Raup–Sepkoski extinction $\hat{\alpha}$ |
| `L16` | 66 | `S4` | Pharmacological, clinical and diagnostic order | `T4` | `{T2, T4, T5}` | `(1, 2, 0)` | Governance Axial Exposure | cost–loss $p^\ast\_{\mathrm{CL}} = C/(\eta L)$ and Pauker–Kassirer $p^\ast\_{\mathrm{PK}} = C\_{\mathrm{FP}}/(C\_{\mathrm{FP}} + L\_{\mathrm{FN}})$, therapeutic index $\mathrm{TI} = \mathrm{TD}\_{50}/\mathrm{ED}\_{50}$, and sepsis Golden Hour |
| `L17` | 67 | `S4` | Epidemiology, outbreak and surveillance order | `T3` | `{T2, T3, T5}` | `(2, 1, 0)` | Governance Axial Exposure | Kermack–McKendrick $R\_0 = 1$ transcritical threshold and serial interval vs. tracing queue $\mathrm{TTR}$ |
| `L18` | 68 | `S4` | Pharmacovigilance, regulatory and toxicological order | `T2` | `{T2, T3, T7}` | `(2, 0, 1)` | Vital Axial Exposure | Bradford Hill coherence, FAERS spontaneous reporting $\rho\_{R,Y}$ bias, and recall veto gate |
| `L19` | 69 | `S4` | Metascience, scientometrics and replication order | `T6` | `{T2, T3, T6}` | `(2, 0, 1)` | Vital Axial Exposure | Ioannidis $\mathrm{PPV}$ under file-drawer bias, Smaldino–McElreath metric selection, and *e-BH* FDR control |
| `L20` | 70 | `S4` | Acoustic, aesthetic, tragic and pedagogical order | `T1` | `{T1, T4, T6}` | `(1, 1, 1)` | Trans-Triadic Keystone | Composite order (shared irreducible residual distribution and reversible fractional probe; domains non-reducible and non-isomorphic in scale): Pythagorean comma $(3/2)^{12} \ne 2^7$, tragic *hamartía* vs. $T\_4$, and Vygotsky ZPD fractional probe |
| `L21` | 71 | `S5` | Linguistic, evidential and pragmatic order | `T7` | `{T3, T6, T7}` | `(1, 0, 2)` | Vital Axial Exposure | Tuyuca/Quechua obligatory morphological evidentials vs. March–Simon uncertainty absorption |
| `L22` | 72 | `S5` | Philological, epigraphic and textual order | `T3` | `{T2, T3, T4}` | `(2, 1, 0)` | Governance Axial Exposure | Lachmann *stemma codicum* = cluster $\mathrm{DEFF}$ and Leiden epigraphic `[...]` = Manski bounds |
| `L23` | 73 | `S5` | Taphonomic and historiographical order | `T5` | `{T3, T5, T7}` | `(1, 1, 1)` | Trans-Triadic Keystone | Efremov taphonomic filter = Meng $\rho\_{R,Y}$ + Chao $\hat{n}\_{00}$ and a sealed record against hindsight |
| `L24` | 74 | `S5` | Critical archival and transitional-justice order | `T1` | `{T1, T3, T7}` | `(2, 0, 1)` | Vital Axial Exposure | Fricker testimonial justice, Trouillot archival silencing, and Ball capture-recapture $\hat{n}\_{00}$ |
| `L25` | 75 | `S5` | Media ecology and information-retrieval order | `T6` | `{T1, T3, T6}` | `(2, 0, 1)` | Vital Axial Exposure | Innis bias of communication and recommender feedback loop = performative $\rho\_{R,Y}$ amplification |
| `L26` | 76 | `S6` | Sequential decision, bandit and real-options order | `T5` | `{T1, T2, T5}` | `(2, 1, 0)` | Governance Axial Exposure | Wald SPRT boundaries, Gittins bandit index, and Dixit–Pindyck irreversible option value |
| `L27` | 77 | `S6` | Game-theoretic, strategic and adversarial order | `T2` | `{T1, T2, T6}` | `(2, 0, 1)` | Vital Axial Exposure | Perdomo performative stability vs. strategic manipulation and minimax adversarial robustness |
| `L28` | 78 | `S6` | Macro-finance, econometric and actuarial order | `T6` | `{T2, T4, T6}` | `(1, 1, 1)` | Trans-Triadic Keystone | Cramér–Lundberg ruin, Artzner coherent $\mathrm{CVaR}$ vs. VaR, Lucas critique, and Goodhart 1975 |
| `L29` | 79 | `S6` | Official statistics and forensic accounting order | `T3` | `{T3, T4, T7}` | `(1, 1, 1)` | Trans-Triadic Keystone | Pacioli double-entry conservation invariant, Desrosières census ontology, and Sen poverty axioms |
| `L30` | 80 | `S6` | Corporate governance and principal–agent order | `T1` | `{T1, T6, T7}` | `(1, 0, 2)` | Vital Axial Exposure | Holmström–Milgrom multitask distortion, Hurwicz incentive compatibility, and Innes liability |
| `L31` | 81 | `S7` | Criminal proof and forensic epistemology order | `T2` | `{T1, T2, T7}` | `(2, 0, 1)` | Vital Axial Exposure | Blackstone asymmetric error loss, likelihood-ratio forensic weight, and chain-of-custody seal |
| `L32` | 82 | `S7` | Evidentiary, tort and constitutional order | `T7` | `{T1, T4, T7}` | `(1, 1, 1)` | Trans-Triadic Keystone | Learned Hand $B < P \cdot L$ with tail amplification and constitutional supermajority as hysteresis |
| `L33` | 83 | `S7` | Distributed consensus and site-reliability order | `T5` | `{T5, T6, T7}` | `(0, 1, 2)` | Epistemic Axial Exposure | FLP/CAP theorem, metastable retry storms at $\rho \to 1$, and SRE error-budget freeze gate |
| `L34` | 84 | `S7` | High-hazard infrastructure safety order | `T4` | `{T4, T5, T7}` | `(0, 2, 1)` | Epistemic Axial Exposure | Perrow interactive complexity $\times$ tight coupling, Rasmussen drift, and hardwired SCRAM veto |
| `L35` | 85 | `S7` | Polycentric commons and biospheric order | `T6` | `{T3, T5, T6}` | `(1, 1, 1)` | Trans-Triadic Keystone | Ostrom 8 polycentric principles, Rockström planetary boundaries, and Leontief triadic floor |
