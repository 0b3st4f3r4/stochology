# Sets, Relations, Functions

<figure class="stoch-figure stoch-logo-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="660" height="132" viewBox="0 0 660 132" fill="none" role="img" aria-label="Stochology CH09 · RELATIONS — Sets, Relations, And Morphisms">
  <defs>
    <linearGradient id="ch09-ring" x1="2" y1="78" x2="386" y2="78" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="38%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="68%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <radialGradient id="ch09-glow" cx="50%" cy="50%" r="50%">
      <stop offset="0%" stop-color="#38bdf8" stop-opacity="0.25"/>
      <stop offset="50%" stop-color="#2563eb" stop-opacity="0.14"/>
      <stop offset="82%" stop-color="#fbbf24" stop-opacity="0.08"/>
      <stop offset="100%" stop-color="#090e1a" stop-opacity="0"/>
    </radialGradient>
    <linearGradient id="ch09-triad-stroke" x1="18" y1="24" x2="110" y2="104" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.85"/>
      <stop offset="50%" stop-color="var(--logo-gold, #f59e0b)" stop-opacity="0.90"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.85"/>
    </linearGradient>
    <linearGradient id="ch09-fill" x1="22" y1="30" x2="106" y2="98" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)" stop-opacity="0.26"/>
      <stop offset="48%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.28"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.26"/>
    </linearGradient>
    <linearGradient id="ch09-ink-curve" x1="20" y1="66" x2="108" y2="66" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-ink, #0f172a)"/>
      <stop offset="50%" stop-color="var(--logo-sub, #1d4ed8)"/>
      <stop offset="100%" stop-color="var(--logo-ink, #0f172a)"/>
    </linearGradient>
    <radialGradient id="ch09-pupil" cx="45%" cy="42%" r="58%">
      <stop offset="0%" stop-color="#020617"/>
      <stop offset="68%" stop-color="#0f172a"/>
      <stop offset="100%" stop-color="#451a03"/>
    </radialGradient>
    <linearGradient id="ch09-gold-mira" x1="54" y1="48" x2="84" y2="82" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#fef08a"/>
      <stop offset="45%" stop-color="#fbbf24"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <radialGradient id="ch09-gold-core" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#ffffff"/>
      <stop offset="35%" stop-color="#fde047"/>
      <stop offset="100%" stop-color="#d97706"/>
    </radialGradient>
    <radialGradient id="ch09-node-well" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="var(--logo-well-hi, var(--elevated-bg, #ffffff))"/>
      <stop offset="100%" stop-color="var(--logo-node-bg, #e2e8f0)"/>
    </radialGradient>
    <linearGradient id="ch09-ring-epi" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#7dd3fc"/>
      <stop offset="50%" stop-color="#2563eb"/>
      <stop offset="100%" stop-color="#1e3a8a"/>
    </linearGradient>
    <radialGradient id="ch09-core-epi" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#bae6fd"/>
      <stop offset="45%" stop-color="#3b82f6"/>
      <stop offset="100%" stop-color="#1d4ed8"/>
    </radialGradient>
    <linearGradient id="ch09-ring-vital" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#6ee7b7"/>
      <stop offset="50%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#065f46"/>
    </linearGradient>
    <radialGradient id="ch09-core-vital" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#a7f3d0"/>
      <stop offset="45%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#047857"/>
    </radialGradient>
    <linearGradient id="ch09-ring-gov" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#fda4af"/>
      <stop offset="50%" stop-color="#e11d48"/>
      <stop offset="100%" stop-color="#881337"/>
    </linearGradient>
    <radialGradient id="ch09-core-gov" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#fecdd3"/>
      <stop offset="45%" stop-color="#f43f5e"/>
      <stop offset="100%" stop-color="#be123c"/>
    </radialGradient>
  </defs>
  <g>
    <circle cx="64" cy="66" r="48" fill="url(#ch09-glow)"/>
    <circle cx="46" cy="66" r="26" fill="var(--logo-blue, #2563eb)" fill-opacity="0.16" stroke="var(--logo-blue, #2563eb)" stroke-width="2.4"/>
    <circle cx="82" cy="66" r="26" fill="var(--logo-green, #10b981)" fill-opacity="0.16" stroke="var(--logo-green, #10b981)" stroke-width="2.4"/>
    <path d="M 38 48 Q 64 26 90 48" stroke="url(#ch09-gold-mira)" stroke-width="2.6" stroke-linecap="round"/>
    <path d="M 84 48 L 91 49 L 89 42" stroke="url(#ch09-gold-mira)" stroke-width="2.4" stroke-linecap="round" stroke-linejoin="round"/>
    <circle cx="36.00" cy="66.00" r="6.55" fill="url(#ch09-node-well)" stroke="url(#ch09-ring-epi)" stroke-width="2.42"/>
    <circle cx="36.00" cy="66.00" r="2.81" fill="url(#ch09-core-epi)"/>
    <circle cx="92.00" cy="66.00" r="6.55" fill="url(#ch09-node-well)" stroke="url(#ch09-ring-vital)" stroke-width="2.42"/>
    <circle cx="92.00" cy="66.00" r="2.81" fill="url(#ch09-core-vital)"/>
    <g>
      <circle cx="64.00" cy="66.00" r="6.51" fill="var(--logo-node-bg, #e2e8f0)"/>
      <circle cx="64.00" cy="66.00" r="5.63" fill="url(#ch09-pupil)" stroke="url(#ch09-gold-mira)" stroke-width="2.11"/>
      <line x1="55.55" y1="66.00" x2="60.30" y2="66.00" stroke="url(#ch09-gold-mira)" stroke-width="2.11" stroke-linecap="round"/>
      <line x1="67.70" y1="66.00" x2="72.45" y2="66.00" stroke="url(#ch09-gold-mira)" stroke-width="2.11" stroke-linecap="round"/>
      <line x1="64.00" y1="57.55" x2="64.00" y2="62.30" stroke="url(#ch09-gold-mira)" stroke-width="2.11" stroke-linecap="round"/>
      <line x1="64.00" y1="69.70" x2="64.00" y2="74.45" stroke="url(#ch09-gold-mira)" stroke-width="2.11" stroke-linecap="round"/>
      <circle cx="64.00" cy="66.00" r="1.94" fill="url(#ch09-gold-core)"/>
    </g>
  </g>
  <g transform="translate(148, 0)">
    <text x="2" y="27" fill="var(--logo-sub, #1d4ed8)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.2">PART II · THE BASES · MATHEMATICS</text>
    <text x="498" y="27" text-anchor="end" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.0">CH09 · RELATIONS</text>
    <text x="0" y="59" fill="var(--logo-ink, #0f172a)" font-family="system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif" font-size="22.2" font-weight="800" letter-spacing="1.35">SETS, RELATIONS, AND MORPHISMS</text>
    <line x1="2" y1="78" x2="384" y2="78" stroke="url(#ch09-ring)" stroke-width="1.6" stroke-opacity="0.82"/>
    <circle cx="400" cy="78" r="6.2" stroke="var(--logo-gold, #f59e0b)" stroke-width="1.5" stroke-dasharray="2.2 1.6"/>
    <circle cx="400" cy="78" r="4.0" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.2"/>
    <circle cx="400" cy="78" r="1.8" fill="var(--logo-gold, #f59e0b)"/>
    <circle cx="415" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="430" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.0"/>
    <circle cx="430" cy="78" r="1.6" fill="var(--logo-blue, #2563eb)"/>
    <circle cx="445" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-green, #10b981)" stroke-width="2.0"/>
    <circle cx="445" cy="78" r="1.6" fill="var(--logo-green, #10b981)"/>
    <circle cx="460" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="475" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="490" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <text x="2" y="104" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.8" font-weight="600" letter-spacing="0.65">PARTITIONS AND EQUIVALENCE · PREORDERS · STRUCTURE-PRESERVING MAPS</text>
  </g>
  </svg>
</figure>

> First chapter of the mathematical block in Part II. Nothing here is specific to judgment; it is the
> notation the rest of the book uses.

## By the end of this chapter you can

1. Write a set, a relation and a function in correct notation, and say what each is *for*.
2. Tell an equivalence relation from an order, with one example of each from data work.
3. Say what injective, surjective and bijective mean, and why the distinction decides whether a
   quantity can be recovered from a record.

## 1. Sets and Measurable Spaces

A municipal dispatch center runs intake software that requires every call to carry one of six preset
hazard codes before a unit goes out. A chemical freight derailment fits none of the boxes. Operators leave
the record blank, or force it into a generic traffic category, and months later an audit reads zero
incidence of chemical emergencies and cuts hazmat funding. Nothing in that failure needed a probability or
an average. The institution took the records its form admits for the hazards that exist. Set theory blocks
the move: it requires the ambient universe $\Omega$, the recorded subset and the unrecorded complement to
be declared before any quantitative claim.

A set is a collection of mathematical or physical objects, defined by membership: write $x \in A$ for
membership and $x \notin A$ for its negation. Set equality holds when the members are identical, nothing
else (axiom of extensionality).

The operations that recur:

- **Union** $A \cup B = \lbrace{}x : x \in A \text{ or } x \in B\rbrace$;
- **Intersection** $A \cap B = \lbrace{}x : x \in A \text{ and } x \in B\rbrace$;
- **Set difference** $A \setminus B = \lbrace{}x : x \in A \text{ and } x \notin B\rbrace$;
- **Complement** $A^c = \Omega \setminus A$ relative to an ambient universe $\Omega$;
- **Cartesian product** $A \times B = \lbrace(a, b) : a \in A, b \in B\rbrace$.

In data systems an archive is a subset of an ambient product space: if an observation comprises $k$
features, the admissible domain is the Cartesian product $\mathcal{X} = X\_1 \times X\_2 \times \cdots \times X\_k$.
When an archive records $n$ rows, it populates an empirical subset $\mathcal{A}\_n \subset \mathcal{X}$.
Empty cells and missing combinations in that product space mark unobserved regions of the domain, not
proven impossibilities. In capture-recapture and archival completeness audits, the size of the unobserved
complement $\Omega \setminus \mathcal{A}\_n$ is bounded from below using Anne Chao's non-parametric
estimator $\hat{n}\_{00} = f\_1^2 / (2 f\_2)$, where $f\_1$ counts classes observed once and $f\_2$
counts classes observed twice (Chao 1984). Whenever $f\_1 > 0$, the archive proves from its own interior
that the complement $\Omega \setminus \mathcal{A}\_n$ is non-empty.

When the universe $\Omega$ is uncountably infinite, assigning a consistent size or probability to every
arbitrary subset leads to antinomies such as the Banach–Tarski paradox. Modern probability restricts valid
events to a $\sigma$-algebra $\mathcal{F} \subseteq 2^\Omega$: a collection of subsets containing
$\Omega$, closed under complementation ($A \in \mathcal{F} \implies A^c \in \mathcal{F}$) and countable
unions ($A\_i \in \mathcal{F} \implies \bigcup\_{i=1}^\infty A\_i \in \mathcal{F}$). A
$\sigma$-algebra is a pre-registered inventory of well-formed questions. If an institutional schema omits
an event $E$ from $\mathcal{F}$, no downstream statistical estimator can assign a probability to $E$
without first expanding the measurable space.

## 2. Relations: Equivalence and Order

A health ministry evaluates a large set of blood-pressure readings drawn from a few dozen rural clinics,
each patient also rating clinic service on a five-point ordinal scale (`1 = poor`, `5 = excellent`). The
report commits two textbook relational errors. It treats every reading as an independent draw, though
patients inside one clinic share cuff calibration and protocol. And it reads a small shift in mean
satisfaction as a proportional clinical gain. The first ignores an **equivalence relation** partitioning
observations into correlated clusters; the second mistakes an **order relation** for a cardinal metric
space.

A binary relation $R$ on a set $S$ is a subset of the Cartesian product $S \times S$, written $a \sim b$
or $a \le b$ when $(a, b) \in R$. Measurement is governed by two canonical classes of relation:

An **equivalence relation** is reflexive ($a \sim a$), symmetric ($a \sim b \implies b \sim a$), and
transitive ($a \sim b \text{ and } b \sim c \implies a \sim c$). By the fundamental theorem of equivalence
relations, any equivalence relation partitions $S$ into disjoint equivalence classes $[a] = \lbrace{}x \in S : x \sim a\rbrace$.
Every grouping in empirical work is such a partition: by clinical site, school district, sensor batch,
demographic cohort. Treat clustered data as if each unit were independent and the partition drops out of
the arithmetic, underestimating variance by the cluster design effect $\mathrm{DEFF} = 1 + (m - 1)\rho$
(Kish 1965; Moulton 1986).

In canonical form the penalty is computed over $C$ non-empty equivalence classes, of average size
$m \ge 1$ rows per cluster, with intracluster correlation $\rho \in [0, 1]$ as the dimensionless share of
variance shared inside a class. It maps nominal sample size $n = C m$ to effective sample size
$n\_{\mathrm{eff}} = n / \mathrm{DEFF} \in [C, n]$. Two boundary limits show what the partition decides:

1. **No echo inside a class ($\rho \to 0$):** $\mathrm{DEFF} \to 1$ and $n\_{\mathrm{eff}} \to n$; the classes
   carry no shared distortion and every row contributes a full unit of information.
2. **Full lockstep inside a class ($\rho \to 1$):** $\mathrm{DEFF} \to m$ and $n\_{\mathrm{eff}} \to C$; the
   nominal rows collapse to one independent reading per class, and true standard errors inflate by $\sqrt{m}$.

Any archive that classifies its own rows carries such a partition. A meteorological archive that tags
each six-hourly observation by storm status partitions its rows into disjoint, exhaustive status classes,
and summing a column by class is the operation the algebra names. The knowledge graph of the later
chapters is built the same way: the thirty-five orders of knowledge fall into seven disjoint strata of
five orders each ($7 \times 5 = 35$), and their triadic axis-occupancy signature
$(k\_{\mathrm{epi}}, k\_{\mathrm{vital}}, k\_{\mathrm{gov}})$ sorts those same thirty-five orders into
five disjoint, exhaustive classes of sizes $12 + 9 + 9 + 4 + 1 = 35$.

A **partial order** is reflexive ($a \le a$), antisymmetric ($a \le b \text{ and } b \le a \implies a = b$),
and transitive. It allows comparative statements (*worse*, *equal*, *better*) without a cardinal distance
metric. Ordinal rankings are invariant to strictly monotonic transformations; cardinal arithmetic averages
are not (Stevens 1946; Krantz, Luce, Suppes & Tversky 1971). Average ordinal survey ranks as though equal
numerical intervals were equal physical differences, and you have the common pathology. A partial order
need not be **total** either: on the real line $\mathbb{R}$, every pair $(x, y)$ satisfies $x \le y$ or
$y \le x$, whereas in a directed acyclic dependency graph two layers $L\_j$ and $L\_k$ with neither
$L\_j \in \mathrm{Anc}^\ast(L\_k)$ nor $L\_k \in \mathrm{Anc}^\ast(L\_j)$ are strictly incomparable.
Collapse a partial order into a single linear leaderboard and you fabricate comparability where none
exists.

## 3. Functions as Measurement and Recording Pipelines

An administrative intake form offers five checkboxes for cause of failure. Every complex field report
that does not fit the first four boxes is mapped to the fifth ("other"), and once that form is saved no
downstream analyst can invert "other" back into the distinct physical failure modes that produced it. That
many-to-one compression, irreversible, is what it means for a recording pipeline to fail injectivity.

A function $f : A \to B$ assigns to each element $a \in A$ a unique image $f(a) \in B$. The set $A$ is the
domain, $B$ the codomain, and the subset $f(A) = \lbrace{}f(a) : a \in A\rbrace \subseteq B$ the image, or range.

Three structural properties decide whether information can be recovered without loss. A map is
**injective** (one-to-one) when $f(a\_1) = f(a\_2) \implies a\_1 = a\_2$, so no distinct states are merged;
**surjective** (onto) when $f(A) = B$, so every state in the codomain is realized by at least one input;
and **bijective** when it is both, in which case a unique two-sided inverse $f^{-1} : B \to A$ exists.

Measurement in the physical and social worlds is a function $m : \mathcal{W} \to \mathcal{A}$ mapping
latent world states $\mathcal{W}$ into recorded archive states $\mathcal{A}$. In realistic settings $m$ is
almost never injective: multiple distinct world states generate identical sensor readings or survey
responses. When injectivity fails, the pre-image $m^{-1}(y) = \lbrace{}w \in \mathcal{W} : m(w) = y\rbrace$ is a set
rather than a point. Point estimation replaces that set with an arbitrary single element. Honest partial
identification keeps the whole pre-image set (Manski 1990). Let a fraction $1 - p$ of an outcome bounded in
$[y\_{\min}, y\_{\max}]$ go unobserved: the pre-image of the recorded sample mean
$\bar{Y}\_{\mathrm{obs}}$ is then the entire identification interval
$[p \bar{Y}\_{\mathrm{obs}} + (1 - p)y\_{\min},\mkern5mu p \bar{Y}\_{\mathrm{obs}} + (1 - p)y\_{\max}]$
of width $(1 - p)(y\_{\max} - y\_{\min})$, which collapses to a point if and only if $p = 1$ (bijective
observation without missingness).

Pipelines are functional compositions $(f\_k \circ f\_{k-1} \circ \cdots \circ f\_1)(x)$, and composition
preserves injectivity only if every constituent stage preserves it. Let one transformation discard
information, coarse binning or threshold truncation, and injectivity is destroyed irreversibly for all
downstream analyses. Internal architectural invariants are the other case: there bijection must be strict,
so that no structural tension goes unguarded. Between the three planes
$(\mathsf{World}, \mathsf{Archive}, \mathsf{Institution})$ there are $3 \times 2 = 6$ ordered binary
transits ($3$ direct loop transits and $3$ reflexive locks) plus $1$ integral three-plane closure
($3 + 3 + 1 = 7$), and the seven guard sites stand in exact bijection onto those seven transits, one site
per transit and no transit unguarded. The same way, the combinatorial count $\binom{n}{3}$ of three-guard
combinations equals the stratified count $5n$ ($5$ orders per primary guard site) if and only if
$\binom{n}{3} = 5n \iff n(n - 1)(n - 2)/6 = 5n \iff n^2 - 3n - 28 = 0$, whose unique positive integer
root is $n = 7$ (with $n = -4$ rejected outside $\mathbb{N}$).

## 4. Axiomatic Foundations: Kolmogorov and Cox

The representation of uncertainty rests on how set operations map to real-valued plausibilities. Andrey
Kolmogorov (1933) grounded probability in measure theory over a measurable space $(\Omega, \mathcal{F})$,
imposing non-negativity ($\mathbb{P}(A) \ge 0$), unit normalization ($\mathbb{P}(\Omega) = 1$), and
countable additivity over disjoint sets. For any two measurable events
$A, B \in \mathcal{F}$, inclusion-exclusion gives
$\mathbb{P}(A \cup B) = \mathbb{P}(A) + \mathbb{P}(B) - \mathbb{P}(A \cap B)$, with two canonical
boundary cases: $A \cap B = \emptyset \implies \mathbb{P}(A \cup B) = \mathbb{P}(A) + \mathbb{P}(B)$
(disjoint additivity) and $A \subseteq B \implies \mathbb{P}(A) \le \mathbb{P}(B)$ (monotonicity under
set inclusion).

Richard Cox (1946) reached the same structure from operational consistency. Require that
plausibility evaluations over a Boolean lattice of propositions be transitive, consistent and continuously
differentiable, and plausibility measures are forced to be isomorphic to Kolmogorov probability. Where
Kolmogorov presupposes a measure space, Cox derives probability as the unique consistent extension of
Boolean logic. Both frameworks agree on the operational consequence. Any decision procedure whose weights
violate set-theoretic additivity can be exploited by a Dutch book, at certain loss across mutually
exclusive alternatives.

## 5. Why this chapter is first

Every subsequent chapter formalizes claims over sets of possible worlds, relations among incentives, and
functions that map states to actions. Probability is a normalized measure on a $\sigma$-algebra. Expectation
is an integral of measurable functions. A model is an empirical approximation to a latent functional map.
With these primitives in hand we do not mistake an arbitrary summary of a set for the set itself.

Set-theoretic classification is one thing; quantitative survival margin is another, and the two must not
be confused. An order of knowledge whose guard-site triple touches all three axes of the triad belongs to
the class of twelve fully triadic orders, yet if one of its three axis margins sits near zero the
non-compensatory Leontief minimum $M\_{\mathrm{Leontief}} = \min(S\_{\mathrm{epi}}, S\_{\mathrm{vital}}, S\_{\mathrm{gov}})$
collapses to that margin, while the arithmetic average
$M\_{\mathrm{arith}} = (S\_{\mathrm{epi}} + S\_{\mathrm{vital}} + S\_{\mathrm{gov}})/3$ stays comfortably
high and reports a solvent institution. The $2 \times 2 \times 2$ factorial decomposition shows the same
failure: a cell carrying two full axes and one absent axis collapses at its minimum while the compensatory
average stays high. Membership in a combinatorial class guarantees which guard sites are inspected, never
that their quantitative margins hold.

### Thirty-five, counted twice

Seven guard sites, taken three at a time, give thirty-five orders, and the distribution is not flat:
twelve triples touch all three axes, nine omit governance, nine omit the vital margin, four omit the
epistemic margin, one concentrates all three sites on a single axis. Thirty-five is also seven times five,
and the five is not a second count of the same triples. It is the size a seven-stratum split forces,
$35/7$. Because $b(n)=\binom{n}{3}/n=(n-1)(n-2)/6$ is strictly increasing, $b(n+1)-b(n)=(n-1)/3$, the
equation $\binom{n}{3}=5n$ has one positive root for the reason any equation $b(n)=c$ does — $b$ is
injective — and not because two independent counts happen to meet at seven.

Add an eighth site and five stops dividing the count: $\binom{8}{3} = 56$ against $8 \times 5 = 40$,
with $56/8 = 7$ orders per primary site instead of five. That checks the factor five; it is not a test the
framework could pass or fail. With $b$ strictly increasing there is exactly one $n$ for each admissible
size, so the arithmetic settles the equation and no second count of the same triples is needed.

Any tabular archive shows the same three objects: its rows form a set, the row-to-identity map is a
function, and the partition into identities is a quotient of that set. Summing a column by identity is
therefore the operation the algebra names, not a spreadsheet habit. The axioms behind the measurability
discussion are Kolmogorov's, from 1933 (Kolmogorov 1933), and the reading of plausibility as a set function
is Cox's, from 1946 (Cox 1946).

### When formalisation does not pay

The claim that formalising in sets pays fails when the universe is finite and small. The algebra of events
is then the power set, measurability is trivial, and invoking it gains nothing. Use the power set, say that
measurability was trivialised, and spend the effort where the sets are large enough for the distinction to
bite.

## Bridge

[What Judging Is, and What It Is Not](27-what-judging-is.md) applies these objects to the full decision
circuit: the world is a space of latent states $\mathcal{W}$, the archive is the non-injective image
$m(\mathcal{W})$, and the decision is a policy function selecting actions under partial identification.
The epistemic margin is a margin on that gap between latent states and observable sets, and the bijective
transitions and partitions of the mediation loop across world, archive, and institution are what the seven
guard sites and the seven strata of knowledge formalize, one site per transit and one stratum per site.

[Numbers, Order, and Bounds](10-numbers-order-bounds.md) builds the ordered arithmetic these objects
live in, and turns intervals into the honest way of stating what a partial answer actually knows.
