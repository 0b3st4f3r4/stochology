# Appendix B — The Grammar of Aim and the Multilayer Architecture

<figure class="stoch-figure stoch-logo-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="800" height="132" viewBox="0 0 800 132" fill="none" role="img" aria-label="Stochology Appendix B — The Grammar of Aim and the Multilayer Architecture">
  <defs>
    <linearGradient id="apb-ring" x1="2" y1="78" x2="386" y2="78" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="38%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="68%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <radialGradient id="apb-glow" cx="50%" cy="50%" r="50%">
      <stop offset="0%" stop-color="#fbbf24" stop-opacity="0.24"/>
      <stop offset="48%" stop-color="#38bdf8" stop-opacity="0.14"/>
      <stop offset="80%" stop-color="#e11d48" stop-opacity="0.08"/>
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
    <circle cx="64" cy="66" r="44" fill="none" stroke="var(--logo-ticks, #64748b)" stroke-width="1.2" stroke-dasharray="3 2.5" stroke-opacity="0.45"/>
    <polygon points="64,22 102,44 102,88 64,110 26,88 26,44" fill="url(#apb-fill)" stroke="url(#apb-triad-stroke)" stroke-width="2.4" stroke-linejoin="round"/>
    <circle cx="64" cy="66" r="24" fill="none" stroke="var(--logo-green, #10b981)" stroke-width="1.6" stroke-dasharray="3.5 2.5" stroke-opacity="0.80"/>
    <polygon points="26,44 102,44 64,110" fill="none" stroke="url(#apb-ink-curve)" stroke-width="1.4" stroke-dasharray="3 2" stroke-opacity="0.6" stroke-linejoin="round"/>
    <circle cx="64.00" cy="22.00" r="3.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.55"/>
    <circle cx="102.00" cy="88.00" r="3.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.55"/>
    <circle cx="26.00" cy="88.00" r="3.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.55"/>
    <circle cx="26.00" cy="44.00" r="7.0" fill="url(#apb-node-well)" stroke="url(#apb-ring-epi)" stroke-width="2.5"/>
    <circle cx="26.00" cy="44.00" r="3.0" fill="url(#apb-core-epi)"/>
    <circle cx="102.00" cy="44.00" r="7.0" fill="url(#apb-node-well)" stroke="url(#apb-ring-vital)" stroke-width="2.5"/>
    <circle cx="102.00" cy="44.00" r="3.0" fill="url(#apb-core-vital)"/>
    <circle cx="64.00" cy="110.00" r="7.0" fill="url(#apb-node-well)" stroke="url(#apb-ring-gov)" stroke-width="2.5"/>
    <circle cx="64.00" cy="110.00" r="3.0" fill="url(#apb-core-gov)"/>
    <g id="apb-reticle">
      <circle cx="64.00" cy="66.00" r="7.2" fill="var(--logo-node-bg, #e2e8f0)"/>
      <circle cx="64.00" cy="66.00" r="6.2" fill="url(#apb-pupil)" stroke="url(#apb-gold-mira)" stroke-width="2.2"/>
      <line x1="58.8" y1="66.00" x2="63.5" y2="66.00" stroke="url(#apb-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
      <line x1="64.5" y1="66.00" x2="69.2" y2="66.00" stroke="url(#apb-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
      <line x1="64.00" y1="60.8" x2="64.00" y2="65.5" stroke="url(#apb-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
      <line x1="64.00" y1="66.5" x2="64.00" y2="71.2" stroke="url(#apb-gold-mira)" stroke-width="2.2" stroke-linecap="round"/>
      <circle cx="64.00" cy="66.00" r="2.1" fill="url(#apb-gold-core)"/>
    </g>
  </g>
  <g transform="translate(148, 0)">
    <text x="2" y="27" fill="var(--logo-sub, #1d4ed8)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.2">STOCHOLOGY · TECHNICAL REFERENCE</text>
    <text x="498" y="27" text-anchor="end" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.0">APPENDIX B · GRAMMAR &amp; VERIFICATION</text>
    <text x="0" y="59" fill="var(--logo-ink, #0f172a)" font-family="system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif" font-size="13.8" font-weight="800" letter-spacing="0.55">THE GRAMMAR OF AIM AND THE MULTILAYER ARCHITECTURE</text>
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

This appendix records the formal syntax layer for trajectory admissibility and the combinatorial layer indexing thirty-five knowledge orders across seven strata; conceptual and empirical arguments sit in the chapters and govern wherever an appendix and a chapter disagree, while operators sit in [Appendix A](appendix-a-mathematics.md) and vocabulary in [Appendix E](appendix-e-lexicon.md).

---

## B.1 The Grammar of Aim — Syntax Layer and Acceptance Predicate

Stochology operates as a *transversal grammar* over trajectories: lexicon, productions, and predicate sit across the whole circuit rather than any single plane or stage.

The grammar owns **syntax and acceptance, never dynamics**. Its lexicon is three planes, the three axes of the Nuclear Triad, and the **seven guard sites**, one per constitutive tension, partitioned $3/2/2$ over $(S\_{\mathrm{epi}}, S\_{\mathrm{vital}}, S\_{\mathrm{gov}})$. A step of the loop is a **shot**: a control tape (*télos*) paired with a contingency tape (*skopós*). A guarded trajectory is a **sentence**, sealed over an append-only record: a derivation cannot be revised after its verdict and still presented as the same derivation.

The predicate evaluates five non-commuting stages (after a static pre-stage axis-compatibility check) and returns one of four verdicts in strict precedence: `Unparsable` (quarantine for structural, budget, threshold-invariant, reopening, coverage, or finiteness faults in Stages one through three, as well as missing or fold-incoherent declarations in Stages four and five, naming the defect), `Collapse` (Stage four breach, carrying proposition, site, and step), `Boundary` (Stage five clearance inside the near-critical band, naming weakest site and margin), and `Interior` (full clearance above the band). `Collapse` inherits the Triad's $2^3 = 8$-cell cube directly, while `Unparsable` prevents unparsed, non-finite, or unguarded traces from masquerading as physical catastrophes.

Two generative regimes read the same productions in opposite directions: exhaustive enumeration over the finite guard grid interrogates coverage of the language, while the shortest single-site counterexample interrogates the declared thresholds before field study begins.

**The productions.** The skeleton grammar generates guard-free trajectories, keeping syntax distinct from total guard annotation. Brackets $[X]$ denote an optional element ($\varepsilon \mid X$), and $[X\_1 \mid \dots \mid X\_k]$ denotes an optional exclusive choice ($\varepsilon \mid X\_1 \mid \dots \mid X\_k$). With $\langle\textit{Scope}\rangle$ and finite real $\langle\textit{Scalar}\rangle$ terminals drawn from the step being judged:

$$\begin{aligned}
\langle\textit{Sentence}\rangle &::= \langle\textit{Framing}\rangle\mkern5mu\langle\textit{Counting}\rangle\mkern5mu\langle\textit{Body}\rangle \\\\
\langle\textit{Framing}\rangle &::= \texttt{frame}\mkern5mu[\texttt{insulate} \mid \texttt{open} \mid \texttt{reopen}]\mkern5mu[\texttt{type3}] \\\\
\langle\textit{Counting}\rangle &::= \texttt{count}\mkern5mu\langle\textit{Interval}\rangle\mkern5mu[\texttt{deff}]\mkern5mu[\texttt{vintages}] \\\\
\langle\textit{Body}\rangle &::= \langle\textit{Decision}\rangle\mkern5mu[\langle\textit{Brake}\rangle]\mkern5mu(\langle\textit{Write}\rangle \mid \langle\textit{Read}\rangle)^{\ast}\mkern5mu[\texttt{halt} \mid \langle\textit{Step}\rangle] \\\\
\langle\textit{Step}\rangle &::= \langle\textit{Framing}\rangle\mkern5mu\langle\textit{Counting}\rangle\mkern5mu\langle\textit{Body}\rangle \\\\
\langle\textit{Decision}\rangle &::= \texttt{act}\mkern5mu\texttt{lex}(\texttt{survival}, \texttt{minimax\_regret}) \\\\
\langle\textit{Brake}\rangle &::= \texttt{brake}\mkern5mu\texttt{tts} \ge \texttt{ttr} \\\\
\langle\textit{Write}\rangle &::= \texttt{write}\mkern5mu\langle\textit{Scope}\rangle, \qquad
\langle\textit{Read}\rangle ::= \texttt{read}\mkern5mu\langle\textit{Scope}\rangle, \qquad
\langle\textit{Interval}\rangle ::= \texttt{[}\langle\textit{Scalar}\rangle\texttt{,}\mkern3mu\langle\textit{Scalar}\rangle\texttt{]}
\end{aligned}$$

The literal typewriter delimiters $\texttt{[}$ and $\texttt{]}$ bracket finite real endpoints $L, U \in \mathbb{R}$; admissibility $L \le U$ is checked in Stage one of the predicate rather than inside the context-free rule, so an inverted interval $L > U$ parses as a syntactic interval token and quarantines as `Unparsable` (`Skeleton`).

**Why the recursion is unambiguous and terminates.** The sole recursive non-terminal is $\langle\textit{Step}\rangle$, appearing only as the exclusive right-recursive tail $[\texttt{halt} \mid \langle\textit{Step}\rangle]$ of $\langle\textit{Body}\rangle$ after $\langle\textit{Framing}\rangle$. Mandatory clause openers ($\texttt{frame}$, $\texttt{count}$, $\texttt{act}$) and every optional or repeatable clause inside $\langle\textit{Framing}\rangle$, $\langle\textit{Counting}\rangle$, and $\langle\textit{Body}\rangle$ open with keywords disjoint from their followers ($\lbrace\texttt{insulate}, \texttt{open}, \texttt{reopen}\rbrace$, $\texttt{type3}$, $\texttt{deff}$, $\texttt{vintages}$, $\texttt{brake}$, $\lbrace\texttt{write}, \texttt{read}\rbrace$), and the step tail separates cleanly on single-token lookahead into $\texttt{halt}$ (zero trailing steps), $\texttt{frame}$ (next step), or end-of-input. The grammar is thus deterministically $\mathrm{LL}(1)$: every finite $N$-step trajectory admits one right-branching parse tree terminating at $\texttt{halt}$ or end-of-input, with no post-$\texttt{halt}$ token or competing grouping. Operationally, committee latency exceeding survival time ($\tau\_{\mathrm{bureaucratic}} > \mathrm{TTS}\_{\mathrm{survival}}$) motivates $\langle\textit{Brake}\rangle$ ($\texttt{brake}\mkern5mu\texttt{tts} \ge \texttt{ttr}$) alongside survival preemption.

**No threshold lives in the grammar.** Thresholds are declared parameters of the assessment, never properties of the language; turning an acceptance threshold into a published target triggers the fourth reflexive brake.

**The predicate, in order.** For a single sealed step $\sigma\_t = (\mathbf{u}\_t, \mathbf{w}\_t, \mathbf{m}\_t)$ and declared thresholds $\boldsymbol{\tau} = (\tau\_{\mathrm{acc}}, \delta\_{\mathrm{band}})$ ([Appendix A.5](appendix-a-mathematics.md#a5-the-grammar-of-aim--syntax-layer-and-acceptance-predicate)), a static pre-stage check first verifies that whenever both a guard site $T\_j$ and a collapse proposition are declared, they belong to the same triad axis; a cross-axis site–proposition pairing is refused before Stage one as a static type mismatch (emitting no verdict, distinct from Stage-one, Stage-four, and Stage-five `Unparsable` (`Skeleton`) verdicts). On compatible inputs, the single-step predicate $\mathcal{A}\_{\mathrm{step}}(\sigma\_t; \boldsymbol{\tau})$ runs in strict five-stage order and names each failure:

1. *Skeleton.* An empty derivation, a step missing its attested control tape, a post-$\texttt{halt}$ step, an out-of-scope write or delegation, an unauthorized read, an exceeded budget, a non-finite control-tape scalar or declared threshold ($\mathrm{NaN}, \pm\infty$ in $L, U, C\_{\mathrm{cap}}, R\_{\mathrm{res}}, \tau\_{\mathrm{acc}}, \delta\_{\mathrm{band}}$, or budget quantities), or a control tape or threshold violating a declared invariant — inverted bounds $L > U$, non-positive provisioned capacity $C\_{\mathrm{cap}} \le 0$, negative reserve $R\_{\mathrm{res}} < 0$, or non-positive frontier band width $\delta\_{\mathrm{band}} \le 0$ — returns `Unparsable` naming the defect (`Skeleton` for structural or threshold-invariant faults, `BudgetExceeded` for derivation overruns). Distinct from opportunistic syntax faults, an attested counter-inductive frame reopening ($\texttt{reopen}$) returns a dedicated `CounterInductiveReopen` quarantine reason under `Unparsable` until forward bounds are re-sealed. Fold-attainment coherence of a declared site, proposition, and step is checked in Stages four and five (after Stages two and four establish axial coverage and $m\_\ast$). An ill-formed input is quarantined as `Unparsable` rather than judged as a valid trajectory.
2. *Totality of the annotation.* Each triad axis $(S\_{\mathrm{epi}}, S\_{\mathrm{vital}}, S\_{\mathrm{gov}})$ — partitioned $3/2/2$ across $\lbrace T\_1, T\_2, T\_3\rbrace$, $\lbrace T\_4, T\_5\rbrace$, and $\lbrace T\_6, T\_7\rbrace$ — must carry at least one populated guard reading; an unguarded axis returns `Unparsable` naming the missing axial coverage.
3. *Finiteness.* Any non-finite margin in $\mathbf{m}\_t$ or remaining numerical step reading ($\mathrm{NaN}, \pm\infty$) returns `Unparsable` naming the non-finite margin defect, because otherwise a missing number could silently compare false against a threshold and pass every guard it touches. A missing number must be louder than a bad one.
4. *The Leontief fold.* Axis scores $s\_a = \min\_{j \in a} m\_j$ ($a \in \lbrace\mathrm{epi}, \mathrm{vital}, \mathrm{gov}\rbrace$) fold to $m\_\ast = \min(s\_{\mathrm{epi}}, s\_{\mathrm{vital}}, s\_{\mathrm{gov}})$. If $m\_\ast < \tau\_{\mathrm{acc}}$, the predicate returns `Collapse` naming the proposition, a site $T\_j$ on a minimizing axis attaining $m\_\ast$, and step $t$ (or `Unparsable` with reason `Skeleton` if the declared site, proposition, or step is missing or the declared site sits on an axis strictly above $m\_\ast$). Only when $m\_\ast \ge \tau\_{\mathrm{acc}}$ does evaluation enter Stage five. This per-step fold over sealed margins is not the strictly *ex post* trajectory co-diagnostic $\mathcal{D}\_{\mathrm{diag}} = (M\_{\mathrm{Leontief}}, \Theta\_{\mathrm{basin}})$: the grammar invokes the fold per step and leaves trajectory diagnosis to the retrospective level.
5. *The frontier band.* For $\delta\_{\mathrm{band}} > 0$ (validated in Stage one, and specializing under shifted margins $\tilde{m}\_j = m\_j - \tau\_j$ to $\tau\_{\mathrm{acc}} = 0$, $\delta\_{\mathrm{band}} = \tau$), if $\tau\_{\mathrm{acc}} \le m\_\ast < \tau\_{\mathrm{acc}} + \delta\_{\mathrm{band}}$, the predicate returns `Boundary` naming a site $T\_j$ on an axis attaining $m\_\ast$ and its margin $m\_\ast$ (or `Unparsable` with reason `Skeleton` if the declared site is missing or sits on an axis strictly above $m\_\ast$); when $m\_\ast \ge \tau\_{\mathrm{acc}} + \delta\_{\mathrm{band}}$, it returns `Interior` (naming no breached site or proposition).

Composed over an $N$-step trajectory sentence $\mathcal{S} = (\sigma\_1, \dots, \sigma\_N)$ ($N \ge 1$) without reading $\mathcal{D}\_{\mathrm{diag}}$, Pass one parses the $\mathrm{LL}(1)$ skeleton and runs Stages one through three across every step $\sigma\_1, \dots, \sigma\_N$ before any step enters Stage four, returning the earliest `Unparsable` so structural, coverage, or finiteness defects anywhere in $\mathcal{S}$ strictly preempt `Collapse`. Pass two then applies Stages four and five step by step from each step's own sealed state, stopping at the earliest Stage-four or Stage-five `Unparsable` (`Skeleton`) or `Collapse`. If all $N$ steps clear Stage four without `Unparsable` or `Collapse`, the sentence returns `Boundary` citing the step-minimal near-critical margin $\min\_{1 \le t \le N} m\_{\ast, t} \in [\tau\_{\mathrm{acc}}, \tau\_{\mathrm{acc}} + \delta\_{\mathrm{band}})$ and the attaining site of the earliest minimizing step if at least one step lies in the band, and returns `Interior` only when every step satisfies $m\_{\ast, t} \ge \tau\_{\mathrm{acc}} + \delta\_{\mathrm{band}}$.

**Contingency, prefix, and stage non-commutation.** At every step $t$, $\mathcal{A}\_{\mathrm{step}}$ reads only the sealed control tape $\mathbf{u}\_t$, margins $\mathbf{m}\_t$, and thresholds $\boldsymbol{\tau}$, never $\mathbf{w}\_t$ (*skopós*) or unsealed history: neither an accurate hit nor prior steps alter a step verdict unless pre-sealed into $\mathbf{u}\_t$, making Antipater's separation of *télos* and *skopós* a formal invariance, and the five stages do not commute under permutation.

*Proof.* (i) *Contingency and prefix invariance:* Stages one through five inspect $\mathbf{u}\_t$ solely through attestation, finiteness, bounds, capacity, reserve, budget, and scope, and inspect $\mathbf{m}\_t$ solely for finiteness and $\min$, sealing $\mathbf{w}\_t$ unread in every verdict branch. Hence $\mathcal{A}\_{\mathrm{step}}(\mathbf{u}\_t, \mathbf{w}\_t, \mathbf{m}\_t; \boldsymbol{\tau}) = \mathcal{A}\_{\mathrm{step}}(\mathbf{u}\_t, \mathbf{w}'\_t, \mathbf{m}\_t; \boldsymbol{\tau})$ for any $\mathbf{w}'\_t$ (and likewise across $\mathcal{A}(\mathcal{S}; \boldsymbol{\tau})$), and two admissible transcripts sharing $(\mathbf{u}\_t, \mathbf{m}\_t)$ at step $t$ return identical verdicts regardless of prior history unless pre-recorded into $\mathbf{u}\_t$. (ii) *Stage non-commutation:* Moving Stages four and five ahead of Stage one clears attested $\texttt{reopen}$ probes as `Interior` whenever provisional margins satisfy $m\_\ast \ge \tau\_{\mathrm{acc}} + \delta\_{\mathrm{band}}$ (while moving Stage five ahead of Stage four returns `Interior` on valid `Collapse` if unguarded); moving Stage four ahead of Stage three lets $\mathrm{NaN}$ fail threshold checks silently; and moving Stage three ahead of Stage two treats an empty axis ($\min \emptyset$) as a numerical singularity rather than missing axial coverage. ∎

## B.2 The Multilayer Graph — 35 Orders, 7 Strata, 5 Exposure Regimes

The combinatorial layer carries Part V. It defines the seven strata $S\_1$–$S\_7$ — each fixed to one anchor guard site as a **designation recorded rather than derived**: $S\_1 \leftrightarrow T\_5$, $S\_2 \leftrightarrow T\_1$, $S\_3 \leftrightarrow T\_4$, $S\_4 \leftrightarrow T\_2$, $S\_5 \leftrightarrow T\_3$, $S\_6 \leftrightarrow T\_6$, and $S\_7 \leftrightarrow T\_7$ — and thirty-five orders carrying their guard triple, primary site, stratum, and exposure regime. Two graphs couple those orders: $\mathcal{G}\_1$ is the acyclic dependency DAG printed by each order's ancestor blockquote, rooted at `L01` ($\text{in-degree} = 0$) and ending in five terminal sinks (`L20`, `L21`, `L22`, `L25`, `L35`), so `L35`'s ancestor cone $\mathrm{Cone}(L\_{35})$ covers $35 - 7 = 28$ orders (omitting seven orders routed solely to the four parallel sinks) under the cone homomorphism $\mathcal{H}(L\_i) = \min\_{L\_j \in \mathrm{Cone}(L\_i)} M\_{\mathrm{Leontief}}(L\_j)$ of [Appendix A.6](appendix-a-mathematics.md); and $\mathcal{G}\_2 = J(7, 3)$ is the Johnson graph on the $\binom{7}{3} = 35$ three-site triples, where two orders share an edge iff their triples overlap in two sites ($\lvert g\_i \cap g\_j \rvert = 2$), making every order regular of degree $3(7 - 3) = 12$ across all three-site pairs — spanning $127$ $\mathcal{G}\_1$-comparable pairs ($53$ direct $\mathcal{G}\_1$ edges shared with $J(7, 3)$ out of $84$ total direct $\mathcal{G}\_1$ edges) and $83$ strictly non-ancestral lateral couplings. Across [Appendix A.6](appendix-a-mathematics.md)'s five mutually exclusive exposure-regime classes, $12 + 9 + 9 + 4 + 1 = 35$.

Two counting identities govern Table B.1: the $\binom{7}{3} = 35$ guard triples exhaust all three-site combinations across the thirty-five orders, and each stratum carries five pairwise distinct primary sites while each of the seven sites is primary in five strata ($5 \times 7 = 35$). Each order triple names what that order is organized around rather than a step annotation: unlike Stage two's per-axis sentence requirement, a `0` entry in Table B.1's $(E, V, G)$ signature records only a zero local guard count ($k\_a = 0$ in that order's three-site triple, inheritable upstream across its parents in $\mathcal{G}\_1$ via $\mathrm{Cone}(L\_i)$)—never setting the normalized trajectory axis score $S\_a = 0$ and never triggering Stage-2 `Unparsable` syntactic rejection—while the final column records the domain isomorphism realizing $\mathcal{G}\_1$ and $\mathcal{G}\_2$.

### Table B.1 — Complete 35-Layer Multilayer Knowledge Graph Mapping (Part V)

| Order | Ch | Stratum | Knowledge Order | Primary Site | Unique Guard Triple | Signature (E,V,G) | Exposure Regime | Canonical Isomorphism ($\mathcal{G}\_1 \leftrightarrow \mathcal{G}\_2$) |
| :---: | :---: | :---: | :--- | :---: | :---: | :---: | :--- | :--- |
| `L01` | 51 | `S1` | Micro-kinetic sensorimotor order | `T5` | `{T1, T5, T7}` | `(1, 1, 1)` | Trans-Triadic Keystone | Two-tape separation (`télos`/`skopós`) and $\mathrm{TTS}\_{\mathrm{survival}} \ge \mathrm{TTR}\_{\mathrm{eff}}$ |
| `L02` | 52 | `S1` | Meso-buffered queue order | `T7` | `{T4, T6, T7}` | `(0, 1, 2)` | Epistemic Axial Exposure | Kingman heavy-traffic hyperbola $\mathbb{E}[W\_q] \approx \frac{\rho}{1-\rho}\frac{c\_a^2+c\_s^2}{2} T\_s$, performative wait feedback $W\_q = W\_q(\lambda(W\_q))$, and buffer-liability balance |
| `L03` | 53 | `S1` | Macro-coupled topology order | `T6` | `{T4, T5, T6}` | `(0, 2, 1)` | Epistemic Axial Exposure | Motter–Lai cascade tolerance $\alpha\_{\mathrm{tol}} = (1-\rho)/\rho$ as reciprocal of Kingman, Wirtinger–RCC delay–jitter ceiling $\bar{\tau}^{\mathrm{NL\text{-}Q}}\_{\max}$, and Anderson–Morley 1-hop degree budget |
| `L04` | 54 | `S1` | Geophysical chaos and data assimilation order | `T3` | `{T3, T4, T5}` | `(1, 2, 0)` | Governance Axial Exposure | Lorenz Lyapunov horizon $\tau\_{\mathrm{Lyap}} = \lambda\_1^{-1}\ln(\delta\_{\mathrm{tol}}/\varepsilon\_0)$, subgrid closure, and 4D-Var/CRPS assimilation |
| `L05` | 55 | `S1` | Continuum fracture and structural order | `T4` | `{T2, T4, T7}` | `(1, 1, 1)` | Trans-Triadic Keystone | Griffith critical crack $a\_c$, quartic basin fold bifurcation analogue $\mu\_{\mathrm{crit}} = 3(h/2)^{2/3}$, and Weibull weakest link $\min\_i R\_i$ |
| `L06` | 56 | `S2` | Formal computable linear-type order | `T1` | `{T1, T4, T5}` | `(1, 2, 0)` | Governance Axial Exposure | Rice undecidability $\to$ a syntactic judging procedure and Girard linear/affine resource types |
| `L07` | 57 | `S2` | Probability, credence and measurement order | `T4` | `{T1, T2, T4}` | `(2, 1, 0)` | Governance Axial Exposure | Walley credal bounds $[\underline{P},\overline{P}]$ (instantiated by Manski missingness) and Stevens scale invariance |
| `L08` | 58 | `S2` | Metrological, information-geometric order | `T3` | `{T3, T4, T6}` | `(1, 1, 1)` | Trans-Triadic Keystone | BIPM GUM Type A vs. Type B uncertainty, Bregman Pythagorean inequality, and CORP–PAV Moreau/cone decomposition |
| `L09` | 59 | `S2` | Experimental, sampling and causal order | `T2` | `{T1, T2, T3}` | `(3, 0, 0)` | Pure Epistemic Concentration | Fisher randomization, Kish $n\_{\mathrm{Kish}}$ + Kish–Moulton unequal-cluster $\tilde{m} = \bar{m}(1 + \mathrm{CV}^2(m))$, $\mathrm{DEFF}\_{\mathrm{cluster}} = 1 + (\tilde{m}-1)\rho\_I$ (additive with $\mathrm{DEFF}\_{\mathrm{Kish}}$), and Steegen multiverse / Simonsohn specification curve |
| `L10` | 60 | `S2` | Cryptographic, zero-knowledge, adversarial order | `T7` | `{T2, T6, T7}` | `(1, 0, 2)` | Vital Axial Exposure | Kerckhoffs–Shannon secrecy, Dolev–Yao adversary, zero-knowledge soundness = test-side *e-value*, and two-round threshold Schnorr signatures (`FROST`) with strict single-use nonces and post-sign fault verification |
| `L11` | 61 | `S3` | Biochemical kinetic and molecular order | `T2` | `{T2, T5, T6}` | `(1, 1, 1)` | Trans-Triadic Keystone | Hopfield–Ninio–McKeithan (1995) proofreading error $\varepsilon\_1^N$ with reciprocal selectivity *e-process* $L\_N = \varepsilon\_1^{-N}$ and Michaelis–Menten saturation $f = [S]/(K\_M + [S])$ ($\ne$ Kingman queue: finite-site ceiling vs. unbounded buffer divergence) |
| `L12` | 62 | `S3` | Morphogenetic and developmental order | `T5` | `{T1, T5, T6}` | `(1, 1, 1)` | Trans-Triadic Keystone | Waddington *chreod* / Canguilhem–Holling resilience basin, reaction–diffusion length $\lambda = \sqrt{D\tau}$ with Kramers escape $\tau\_{\mathrm{esc}} = \tau\_0 e^{\Delta V / D\_{\mathrm{eff}}}$ (active noise $D\_{\mathrm{eff}}$ vs. Boltzmann weight $e^{-\Delta V / k\_B T}$), and chaperone capacitor + threshold readout $\delta x^\ast$ |
| `L13` | 63 | `S3` | Immunological self-governance order | `T7` | `{T2, T5, T7}` | `(1, 1, 1)` | Trans-Triadic Keystone | Two-signal conjunctive gate ($s \ge \theta\_s \wedge c \ge \theta\_c$), thymic escape $(1 - q)^k$, somatic survival margin $\mathrm{TTS}\_{\mathrm{survival}}(m) \ge \mathrm{TTR}\_{\mathrm{recovery}}$, and in-band $\mathrm{Treg}$ restraint vs. out-of-band Beer $S3^\ast$ algedonic brake |
| `L14` | 64 | `S3` | Taxonomy, systematics and ontology order | `T4` | `{T1, T3, T4}` | `(2, 1, 0)` | Governance Axial Exposure | Hennig cladistic apomorphy vs. homoplasy and Bowker–Star infrastructural classification torque |
| `L15` | 65 | `S3` | Stratigraphy, paleobiology and deep-time order | `T1` | `{T1, T3, T5}` | `(2, 1, 0)` | Governance Axial Exposure | Designated stratigraphic frames, Signor–Lipps / Strauss–Sadler range extension $a = (1-C)^{-1/(H-1)} - 1$, and Poisson radiometric error $\sigma\_N/N = 1/\sqrt{N}$ vs. closure |
| `L16` | 66 | `S4` | Pharmacological, clinical and diagnostic order | `T4` | `{T2, T4, T5}` | `(1, 2, 0)` | Governance Axial Exposure | Pauker–Kassirer $p^\ast\_{\mathrm{PK}}$ vs. split-ledger cutoff $p^\ast\_{\mathrm{inst}} = C\_{\mathrm{FP}}/(C\_{\mathrm{FP}} + \alpha\_{\mathrm{int}} L\_{\mathrm{FN}})$, tail ratio $\mathrm{TD}\_r/\mathrm{ED}\_q$ vs. therapeutic index $\mathrm{TI} = \mathrm{TD}\_{50}/\mathrm{ED}\_{50}$, and accumulation–clearance therapeutic window |
| `L17` | 67 | `S4` | Epidemiology, outbreak and surveillance order | `T3` | `{T2, T3, T5}` | `(2, 1, 0)` | Governance Axial Exposure | Kermack–McKendrick $R\_t = \beta S/\gamma$ (threshold $1$) / Diekmann $R\_0$, Manski–`Chao2` dark ascertainment, and generation interval $T$ (distinct from serial interval $S\_{ij}$) vs. tracing delay $d/T$ |
| `L18` | 68 | `S4` | Pharmacovigilance, regulatory and toxicological order | `T2` | `{T2, T3, T7}` | `(2, 0, 1)` | Vital Axial Exposure | Bradford Hill coherence, spontaneous-reporting $\rho\_{R,Y}$ and dark-figure $\hat{n}\_{00}$ bounds, and recall veto gate |
| `L19` | 69 | `S4` | Metascience, scientometrics and replication order | `T6` | `{T2, T3, T6}` | `(2, 0, 1)` | Vital Axial Exposure | Ioannidis $\mathrm{PPV}$ under file-drawer bias, Smaldino–McElreath metric selection, and *e-BH* FDR control |
| `L20` | 70 | `S4` | Tragic and pedagogical formative order | `T1` | `{T1, T4, T6}` | `(1, 1, 1)` | Trans-Triadic Keystone | Tragic *hamartía* and *anagnórisis* (*Poetics* 1452a29–33, 1453a8–10), Chevallard–Messick anti-Goodhart audit, and Vygotsky ZPD fractional probe (Pythagorean comma $(3/2)^{12} \ne 2^7$ as residual analogy) |
| `L21` | 71 | `S5` | Linguistic, evidential and pragmatic order | `T7` | `{T3, T6, T7}` | `(1, 0, 2)` | Vital Axial Exposure | Tuyuca/Tariana obligatory morphological evidentials vs. Cuzco Quechua optional enclitics and March–Simon uncertainty absorption |
| `L22` | 72 | `S5` | Philological, epigraphic and textual order | `T3` | `{T2, T3, T4}` | `(2, 1, 0)` | Governance Axial Exposure | Lachmann *stemma codicum* (*eliminatio* vs. cluster $\mathrm{DEFF}\_{\mathrm{total}}$), fixed-point $(\tau, \sigma)$ error severity, and Leiden epigraphic `[...]` letter-space combinatorics $A^k$ |
| `L23` | 73 | `S5` | Taphonomic and historiographical order | `T5` | `{T3, T5, T7}` | `(1, 1, 1)` | Trans-Triadic Keystone | Efremov taphonomic filter $P(S \mid A)$, window decay $S(T) = e^{-\lambda T}$, survival reweighting $\propto 1/P(S \mid \text{class})$, and an inclusion-rule seal |
| `L24` | 74 | `S5` | Critical archival and transitional-justice order | `T1` | `{T1, T3, T7}` | `(2, 0, 1)` | Vital Axial Exposure | Fricker testimonial justice, Trouillot archival silencing, Stoler archival grain ($\rho\_{R,Y} < 0$), and named list vs. capture-recapture total |
| `L25` | 75 | `S5` | Media ecology and information-retrieval order | `T6` | `{T1, T3, T6}` | `(2, 0, 1)` | Vital Axial Exposure | Innis bias of communication and recommender feedback loop = performative $\rho\_{R,Y}$ amplification |
| `L26` | 76 | `S6` | Sequential decision, bandit and real-options order | `T5` | `{T1, T2, T5}` | `(2, 1, 0)` | Governance Axial Exposure | Wald SPRT boundaries, Gittins bandit index, and Dixit–Pindyck irreversible option value |
| `L27` | 77 | `S6` | Game-theoretic, strategic and adversarial order | `T2` | `{T1, T2, T6}` | `(2, 0, 1)` | Vital Axial Exposure | Perdomo performative stability vs. strategic manipulation and minimax adversarial robustness |
| `L28` | 78 | `S6` | Macro-finance, econometric and actuarial order | `T6` | `{T2, T4, T6}` | `(1, 1, 1)` | Trans-Triadic Keystone | Cramér–Lundberg ruin ($\psi(u) \le e^{-Ru}$), Artzner coherent Expected Shortfall vs. VaR, epistemic shrinkage $k(n\_{\mathrm{eff}}^{\mathrm{var}})$ (motivated by Baker–McHale), and Danielsson–Shin / Geanakoplos endogenous leverage cycle |
| `L29` | 79 | `S6` | Official statistics and forensic accounting order | `T3` | `{T3, T4, T7}` | `(1, 1, 1)` | Trans-Triadic Keystone | Pacioli double-entry conservation invariant, Desrosières census ontology, and Sen poverty axioms |
| `L30` | 80 | `S6` | Corporate governance and principal–agent order | `T1` | `{T1, T6, T7}` | `(1, 0, 2)` | Vital Axial Exposure | Holmström–Milgrom multitask distortion, Hurwicz incentive compatibility, and Innes liability |
| `L31` | 81 | `S7` | Criminal proof and forensic epistemology order | `T2` | `{T1, T2, T7}` | `(2, 0, 1)` | Vital Axial Exposure | Blackstone asymmetric error loss, likelihood-ratio forensic weight, and chain-of-custody seal |
| `L32` | 82 | `S7` | Evidentiary, tort and constitutional order | `T7` | `{T1, T4, T7}` | `(1, 1, 1)` | Trans-Triadic Keystone | Learned Hand $B < P \cdot L$ with tail amplification and constitutional supermajority as hysteresis |
| `L33` | 83 | `S7` | Distributed consensus and site-reliability order | `T5` | `{T5, T6, T7}` | `(0, 1, 2)` | Epistemic Axial Exposure | FLP impossibility, Castro–Liskov $3f+1$ quorum, Kingman $\rho/(1-\rho)$ metastable retry storms, 1-hop $f$-collusion / $r$-robust consensus with hybrid RFDE–CRDT coupling, clock-free revocable rehabilitation, and SRE error-budget freeze gate |
| `L34` | 84 | `S7` | High-hazard infrastructure safety order | `T4` | `{T4, T5, T7}` | `(0, 2, 1)` | Epistemic Axial Exposure | Perrow interactive complexity $\times$ tight coupling, Rasmussen drift, and hardwired SCRAM veto |
| `L35` | 85 | `S7` | Polycentric commons and biospheric order | `T6` | `{T3, T5, T6}` | `(1, 1, 1)` | Trans-Triadic Keystone | Ostrom 8 polycentric principles, Rockström planetary boundaries, and Leontief triadic floor |

*Notation scope note for Table B.1:* Summary entries retain each domain's native symbols locally and remain distinct from Stochology's core triad notation: $R$ denotes Weibull weakest-link survival $R\_i$ (`L05`), epidemiologic reproduction $R\_0/R\_t$ (`L17`), or Cramér–Lundberg adjustment coefficient $R$ (`L28`), distinct from reserve $R\_{\mathrm{res}}$ and selection indicator $R$ in $\rho\_{R,Y}$; $S$ denotes susceptible population $S$ (`L17`), substrate $[S]$ (`L11`), serial interval $S\_{ij}$ (`L17`), taphonomic survival filter $P(S \mid A)$ (`L23`), or Beer algedonic channel $S3^\ast$ (`L13`), distinct from triad coordinates $S\_{\mathrm{epi}}, S\_{\mathrm{vital}}, S\_{\mathrm{gov}}$ and strata `S1`–`S7`; $\rho$ denotes queue utilization (`L02`, `L03`, `L33`), intra-cluster correlation $\rho\_I$ (`L09`), or data-defect correlation $\rho\_{R,Y}$ (`L18`, `L24`, `L25`); $\tau$ denotes predicate thresholds $\boldsymbol{\tau}$ (`§B.1`), delay ceiling $\bar{\tau}^{\mathrm{NL\text{-}Q}}\_{\max}$ (`L03`), Lyapunov horizon $\tau\_{\mathrm{Lyap}}$ (`L04`), or stemma tree $\tau$ (`L22`); and $V$ denotes the Kramers potential barrier $\Delta V$ (`L12`), distinct from the Vital axis guard count in $(E, V, G)$.
