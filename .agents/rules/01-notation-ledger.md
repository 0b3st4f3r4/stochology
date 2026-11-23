---
trigger: model_decision
description: Canonical Stochology symbol ledger, known semantic symbol-collision families, and MathJax/LaTeX hygiene rules for critical manuscript review. Load during any chapter review, notation audit, or editorial coherence check.
---

# Notation Ledger & Collision Watchlist

Derived from `book-src/appendix-g-glossary.md` (§G.2), `book-src/appendix-e-lexicon.md` (Table E.1), `book-src/appendix-a-mathematics.md`, `book-src/appendix-b-architecture-verification.md`, and `.github/checks/structure.py`. Whenever any appendix and a governing chapter disagree, the governing chapter owns the definition (`AGENTS.md` §II).

## 1. Canonical Core Symbols

| Symbol (LaTeX) | Meaning & Scope | Canonical Location |
| :--- | :--- | :--- |
| $P\_1, P\_2, P\_3$ | Three Planes: World ($P\_1$), Archive ($P\_2$), Institution ($P\_3$) | Ch. `29`, App. `E.4` |
| $T\_1$–$T\_7$ | Seven guard sites partitioned $3/2/2$: $\{T\_1,T\_2,T\_3\}$ ($S\_{\mathrm{epi}}$), $\{T\_4,T\_5\}$ ($S\_{\mathrm{vital}}$), $\{T\_6,T\_7\}$ ($S\_{\mathrm{gov}}$) | App. `E` Table E.1, App. `B.1` |
| $S\_{\mathrm{epi}}, S\_{\mathrm{vital}}, S\_{\mathrm{gov}}$ | Nuclear Triad coordinates in $[0,1]$ (always `\mathrm{...}`, escaped `\_` in inline math) | App. `A.4`, Ch. `41` |
| $M\_{\mathrm{Leontief}}, \Theta\_{\mathrm{basin}}, \mathcal{D}\_{\mathrm{diag}}$ | Leontief bottleneck $\min(S\_a)$, reception perimeter fold, strictly *ex-post* co-diagnostic pair | App. `A.4`, `A.7`, `AGENTS.md` §I |
| $\mathcal{D}\_{\mathrm{diag}}^{\mathrm{eff}}, \Lambda\_{ab}, \lambda\_{a \to \mathrm{basin}}$ | Interference-discounted co-diagnostic pair and cross-axis/perimeter deficit partials | App. `A.7`, Ch. `99` |
| $\sigma\_t = (\mathbf{u}\_t, \mathbf{w}\_t, \mathbf{m}\_t)$ | Step shot: control tape (*télos*), contingency tape (*skopós*), 7-site margins | App. `B.1`, Ch. `41` |
| $\mathcal{A}(\mathbf{u}\_t, \mathbf{w}\_t, \mathbf{m}\_t; \boldsymbol{\tau})$ | Five-stage non-commuting acceptance predicate (`Interior`, `Boundary`, `Collapse`, `Unparsable`) | App. `B.1` |
| $\rho\_{R,Y}$ | Population correlation between selection/recording indicator $R$ and latent $Y$ (never $\rho\_{Y,R}$) | Ch. `45`, App. `A.1` |
| $\mathrm{DEFF}, \rho\_I, n\_{\mathrm{eff}}^{\mathrm{var}}$ | Unit-level design effect ($\mathrm{DEFF}\_{\mathrm{cluster}} + \mathrm{DEFF}\_{\mathrm{Kish}} - 1$, exact additive composition) vs pair-level weighting in Ch. `50` §9.5.3 | Ch. `45`, `50`, App. `A.1` |
| $V(n\_{\mathrm{eff}}), V$ | Operational Keynesian evidential weight $n\_{\mathrm{eff}}/(n\_{\mathrm{eff}}+n\_0)$ or qualitative weight $K/(K+I\_0)$ | Ch. `50`, App. `A.1` |
| $E\_t, e\_i, K\_{\max}$ | Anytime-valid test martingale wealth, *e*-value factor, hypothesis-adjustment deflation budget | Ch. `18`, `44`, App. `A.1` |
| $p^\ast\_{\mathrm{CL}}, p^\ast\_{\mathrm{PK}}$ | Cost–loss threshold $\min(1, C/(\eta L))$ vs Pauker–Kassirer expected-regret cutoff $C\_{\mathrm{FP}}/(C\_{\mathrm{FP}}+L\_{\mathrm{FN}})$ | Ch. `36`, App. `A.1` |
| $\Delta\_J, \mathrm{CRPS}, \mathrm{CVaR}\_p$ | Jensen convexity gap, continuous ranked probability score, conditional Value-at-Risk | Ch. `46`, App. `A.2` |
| $\alpha, \xi$ | Pareto tail exponent ($\alpha \le 2$ infinite variance; $0 < \alpha < 2$ Lévy jumps) and GPD shape | Ch. `46`, App. `A.2` |
| $\mathrm{TTS}\_{\mathrm{survival}}, \mathrm{TTR}\_{\mathrm{eff}}(\mathcal{I}\_t)$ | Survival horizon vs effective detection+deliberation+actuation latency | Ch. `47`, `50`, App. `A.2` |
| $W\_{\mathrm{allostatic}, t}, \bar{\tau}^{\mathrm{NL\text{-}Q}}\_{\max}$ | Cumulative caregiver allostatic wear; Wirtinger–RCC delay-jitter ceiling | Ch. `42`, `99`, App. `A.2` |
| $C\_{\mathrm{human}}, \rho\_{\mathrm{cog}}$ | Human conscious throughput limit ($\approx 10\text{ bits/s}$) and operator audit utilization | Ch. `95` |
| $\theta\_{\mathrm{PS}}, \theta\_{\mathrm{PO}}, \mathrm{PRI}$ | Performative stable vs optimal equilibria; Premise Rigging Index | Ch. `43`, `48`, App. `A.3` |
| $\delta\_{\mathrm{status}}, \mathrm{DB}\_t, B\_{\mathrm{solvency}}$ | Status testimonial discount, double-bind index, solvency buffer surcharge | Ch. `49`, App. `A.3` |
| $R\_1$–$R\_3$, $F\_1$–$F\_5$ | Ethical refusals (`27`, `40`); pre-registered canonical falsifiers (`50`) | Ch. `27`, `40`, `50` |
| `S1`–`S7`, `L01`–`L35`, $\mathcal{G}\_1, \mathcal{G}\_2, \mathcal{H}(L\_i)$ | Seven strata, 35 orders, dependency DAG, Johnson graph $J(7,3)$, cone homomorphism | App. `B.2`, `A.6` |
| $A^\circ, \circ A, ⊬\_{\mathcal{L}}, O(p)$ | LFI consistency operators, paraconsistent non-explosive entailment, deontic obligation | Ch. `41`, `56`, `82` |

### Designated Stratum Anchors (Recorded Designations, Not Derived)

`S1 ↔ T5 (Temporal)` · `S2 ↔ T1 (Framing)` · `S3 ↔ T4 (Formalization)` · `S4 ↔ T2 (Severity)` · `S5 ↔ T3 (Archive)` · `S6 ↔ T6 (Performative)` · `S7 ↔ T7 (Liability)`

## 2. Watched Semantic Symbol-Collision Families

Whenever two senses from the same row co-occur in a chapter without explicit local disambiguation (distinct subscript, typographic face, or prose caveat), raise a `[Notation]` finding (`Level 2` if derivations conflate them; `Level 3` if merely unglossed):

1. **`e` / `E` family:**
   - Euler's constant $e \approx 2.718$ (exponentials $e^{-\dots}$, Cramér–Lundberg $e^{-Ru}$) vs.
   - Ramdas–Ville *e*-value / *e*-process ($e\_i$, $E\_t$, `E_trip`) vs.
   - Confounding sensitivity `$E$-value` (VanderWeele–Ding, Ch. `35`) vs.
   - Clue $E$ in qualitative process tracing (Ch. `50` §9.2) / Exposure axis count `(E, V, G)`.
2. **`R` family:**
   - Vital reserve $R\_t$ / $R\_{\mathrm{res}}$ vs. Ashby requisite variety $R$ vs. selection indicator $R \in \{0,1\}$ in $\rho\_{R,Y}$ vs. epidemiological reproduction $R\_0, R\_t$ (Ch. `67`) vs. Ethical Refusals $R\_1$–$R\_3$ vs. Lakatos deflation $R\_{\mathrm{Lakatos}}$.
3. **`S` family:**
   - Triad axes $S\_{\mathrm{epi}}, S\_{\mathrm{vital}}, S\_{\mathrm{gov}}$ vs. strata `S1`–`S7` vs. thermodynamic/Shannon entropy $S$ (Ch. `22`–`23`) vs. trajectory sentence $\mathcal{S}$ vs. susceptible compartment $S$ (Ch. `67`) vs. satellite stock $S\_k$ (Ch. `85`).
4. **`\rho` family:**
   - Meng's data-defect correlation $\rho\_{R,Y}$ vs. queue traffic utilization $\rho = \lambda/\mu$ (Kingman/Motter–Lai) vs. intra-cluster correlation $\rho\_I$ (written $\rho\_{\mathrm{cl}}$ in Ch. `67` §2) vs. cognitive load $\rho\_{\mathrm{cog}}$ vs. spectral radius/conductance sector $\rho\_{\mathrm{sec}}$ vs. Spearman rank $\rho$.
5. **`\tau` family:**
   - Acceptance threshold vector $\boldsymbol{\tau} = (\tau\_{\mathrm{acc}}, \delta\_{\mathrm{band}})$ vs. latencies ($\tau\_{\mathrm{detect}}, \tau\_{\mathrm{delib}}, \tau\_{\mathrm{actuate}}, \tau\_{\mathrm{bureaucratic}}, \tau\_{\mathrm{fast}}, \tau\_{\mathrm{cog}}$) vs. Wirtinger–RCC delay ceiling $\bar{\tau}^{\mathrm{NL\text{-}Q}}\_{\max}$ vs. Lorenz Lyapunov horizon $\tau\_L$ vs. basin tolerance $\tau\_{\mathrm{tol}}$.
6. **`\delta` family:**
   - Status discount $\delta\_{\mathrm{status}}$ vs. frontier band $\delta\_{\mathrm{band}}$ vs. minimum effect $\delta\_{\min}$ vs. DP slack $\delta$ vs. minimum graph degree $\delta(\mathcal{G})$ vs. Dirac $\delta$.
7. **`K` / `k` family:**
   - Contractual payout cap $K > 0$ (`X \wedge K`) vs. Hill tail cutoff $k$ (Ch. `96` note!) vs. hypothesis adjustment budget $K\_{\max}$ vs. carrying capacity $K$ (Schaefer, App. `E.3`) vs. epistemic shrinkage $k(n\_{\mathrm{eff}}^{\mathrm{var}})$ vs. Keynes acquired knowledge $K$.
8. **`V` family:**
   - Evidential weight $V(n\_{\mathrm{eff}})$ or $K/(K+I\_0)$ vs. Vital axis count in `(E,V,G)` vs. potential barrier $\Delta V$ (Kramers) vs. Lyapunov function $V(x)$ vs. Lebesgue volume $V\_{\min}$.

## 3. MathJax & Concordance Hygiene

- **Inline math subscripts:** Escape underscores inside `$ ... $` (`$S\_{\mathrm{epi}}$`) so CommonMark never parses `_..._` as emphasis.
- **Roman operators inside math:** Always `\mathrm{epi}`, `\mathrm{vital}`, `\mathrm{gov}`, `\mathrm{Leontief}`, `\mathrm{DEFF}`, `\mathrm{TTS}`, `\mathrm{TTR}` (checked by `.github/checks/structure.py`).
- **Four §V-9 Concordance Invariants (never contradict anywhere in `book-src/`):**
  1. `The minimum is not a weighted sum` (never "weighted average/sum of the three")
  2. `the exact composition is additive` (never "composed multiplicatively" / "product of the design effects")
  3. `not one reading among three` (for governance coordinate sign vs reading)
  4. `designation recorded rather than derived` (for stratum anchors `S1..S7` ↔ `T1..T7`)
