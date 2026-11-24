# Sets, Relations, Functions

<figure class="stoch-figure stoch-logo-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="800" height="132" viewBox="0 0 800 132" fill="none" role="img" aria-label="Stochology CH09 · RELATIONS — Sets, Relations, And Morphisms">
  <defs>
    <!-- Canonical Ring Gradient -->
    <linearGradient id="ch09-ring" x1="2" y1="78" x2="386" y2="78" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="38%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="68%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <!-- 1. Atmospheric Cosmic Field -->
    <radialGradient id="ch09-glow" cx="52%" cy="52%" r="50%">
      <stop offset="0%" stop-color="#fbbf24" stop-opacity="0.24"/>
      <stop offset="48%" stop-color="#38bdf8" stop-opacity="0.14"/>
      <stop offset="80%" stop-color="#e11d48" stop-opacity="0.08"/>
      <stop offset="100%" stop-color="#090e1a" stop-opacity="0"/>
    </radialGradient>
    <!-- 3D Spherical Node Gradients -->
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
    <!-- Stókhos Reticle Gradients -->
    <radialGradient id="ch09-pupil" cx="45%" cy="42%" r="58%">
      <stop offset="0%" stop-color="#020617"/>
      <stop offset="68%" stop-color="#0f172a"/>
      <stop offset="100%" stop-color="#451a03"/>
    </radialGradient>
    <linearGradient id="ch09-gold-mira" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#fef08a"/>
      <stop offset="45%" stop-color="#fbbf24"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <radialGradient id="ch09-gold-core" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#ffffff"/>
      <stop offset="35%" stop-color="#fde047"/>
      <stop offset="100%" stop-color="#d97706"/>
    </radialGradient>
    <!-- Ink curve gradient -->
    <linearGradient id="ch09-ink-curve" x1="0%" y1="0%" x2="100%" y2="0%">
      <stop offset="0%" stop-color="var(--logo-ink, #0f172a)"/>
      <stop offset="50%" stop-color="var(--logo-sub, #1d4ed8)"/>
      <stop offset="100%" stop-color="var(--logo-ink, #0f172a)"/>
    </linearGradient>
    <radialGradient id="ch09-set-epi" cx="45%" cy="45%" r="55%">
      <stop offset="0%" stop-color="#38bdf8" stop-opacity="0.30"/>
      <stop offset="70%" stop-color="#2563eb" stop-opacity="0.18"/>
      <stop offset="100%" stop-color="#1e3a8a" stop-opacity="0.08"/>
    </radialGradient>
    <radialGradient id="ch09-set-vital" cx="55%" cy="45%" r="55%">
      <stop offset="0%" stop-color="#34d399" stop-opacity="0.30"/>
      <stop offset="70%" stop-color="#10b981" stop-opacity="0.18"/>
      <stop offset="100%" stop-color="#065f46" stop-opacity="0.08"/>
    </radialGradient>
    <linearGradient id="ch09-lens-fill" x1="64" y1="44" x2="64" y2="88" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#fde047" stop-opacity="0.32"/>
      <stop offset="50%" stop-color="#38bdf8" stop-opacity="0.24"/>
      <stop offset="100%" stop-color="#10b981" stop-opacity="0.28"/>
    </linearGradient>
    <linearGradient id="ch09-morphism-arrow" x1="38" y1="46" x2="90" y2="46" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#fef08a"/>
      <stop offset="45%" stop-color="#fbbf24"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
  </defs>
  <g>
    <circle cx="64" cy="66" r="48" fill="url(#ch09-glow)"/>
    <circle cx="46" cy="66" r="28" fill="url(#ch09-set-epi)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.6"/>
    <circle cx="82" cy="66" r="28" fill="url(#ch09-set-vital)" stroke="var(--logo-green, #10b981)" stroke-width="2.6"/>
    <path d="M 64 44.55 A 28 28 0 0 1 64 87.45 A 28 28 0 0 1 64 44.55 Z" fill="url(#ch09-lens-fill)"/>
    <path d="M 38 46 Q 63.29 20.71 88.59 44.62" stroke="url(#ch09-morphism-arrow)" stroke-width="3.2" stroke-linecap="round"/>
    <path d="M 82.83 43.81 L 90 46 L 87.81 38.83" stroke="url(#ch09-morphism-arrow)" stroke-width="2.8" stroke-linecap="round" stroke-linejoin="round"/>
    <circle cx="36" cy="66" r="8.5" fill="url(#ch09-node-well)" stroke="url(#ch09-ring-epi)" stroke-width="3.0"/>
    <circle cx="36" cy="66" r="3.8" fill="url(#ch09-core-epi)"/>
    <circle cx="92" cy="66" r="8.5" fill="url(#ch09-node-well)" stroke="url(#ch09-ring-vital)" stroke-width="3.0"/>
    <circle cx="92" cy="66" r="3.8" fill="url(#ch09-core-vital)"/>
    <g>
      <circle cx="64" cy="66" r="7.4" fill="var(--logo-node-bg, #e2e8f0)"/>
      <circle cx="64" cy="66" r="6.4" fill="url(#ch09-pupil)" stroke="url(#ch09-gold-mira)" stroke-width="2.4"/>
      <line x1="54.40" y1="66.00" x2="59.80" y2="66.00" stroke="url(#ch09-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="68.20" y1="66.00" x2="73.60" y2="66.00" stroke="url(#ch09-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="64.00" y1="56.40" x2="64.00" y2="61.80" stroke="url(#ch09-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="64.00" y1="70.20" x2="64.00" y2="75.60" stroke="url(#ch09-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <circle cx="64" cy="66" r="2.2" fill="url(#ch09-gold-core)"/>
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

> Grammar of spaces, equivalence, order, and recording maps, plus the counting root of the multilayer architecture.

## By the end of this chapter you can

1. Write a set, a relation and a function in correct notation, and say what each is *for*.
2. Tell an equivalence relation from an order, with one example of each from data work.
3. Say what injective, surjective and bijective mean, and why the distinction decides whether a
   quantity can be recovered from a record.

## 1. Sets and Measurable Spaces

A municipal dispatch center runs intake software that forces every call into one of six preset hazard boxes before a unit leaves the bay. When a chemical freight train derails, none of the boxes fit. Operators either leave the field blank or drop it into generic traffic. Months later, an audit reads zero chemical emergencies and cuts hazmat funding.

Nothing in that mistake required an average or a probability. The institution simply mistook the categories on its screen for the hazards in the world. Set theory blocks that slide by naming the full space of possibilities $\Omega$, the recorded subset, and the unrecorded remainder before anyone counts.

A **set** is a collection defined solely by what belongs to it. Write $x \in A$ when object $x$ is inside set $A$, and $x \notin A$ when it is not. Two sets are equal precisely when they hold the exact same members, regardless of how they are labeled.

To see the basic set operations at work, picture a tiny four-item universe of dispatch call types:

$$\Omega = \lbrace \text{traffic}, \text{fire}, \text{medical}, \text{chemical} \rbrace.$$

Suppose $A = \lbrace \text{traffic}, \text{fire} \rbrace$ lists the calls logged by day shift and $B = \lbrace \text{fire}, \text{medical} \rbrace$ lists calls needing ladder trucks. Five core operations combine or carve up sets from these ingredients:

- **Union** $A \cup B = \lbrace x : x \in A \text{ or } x \in B \rbrace$ gathers everything in either set, giving $\lbrace \text{traffic}, \text{fire}, \text{medical} \rbrace$.
- **Intersection** $A \cap B = \lbrace x : x \in A \text{ and } x \in B \rbrace$ keeps only shared members, giving $\lbrace \text{fire} \rbrace$.
- **Set difference** $A \setminus B = \lbrace x : x \in A \text{ and } x \notin B \rbrace$ removes $B$ from $A$, leaving $\lbrace \text{traffic} \rbrace$.
- **Complement** $A^c = \Omega \setminus A$ captures everything in $\Omega$ left out by $A$, giving $\lbrace \text{medical}, \text{chemical} \rbrace$.
- **Cartesian product** $A \times B = \lbrace (a, b) : a \in A, b \in B \rbrace$ forms ordered pairs, such as pairing each day-shift type with a truck call: $A \times B$ has $2 \times 2 = 4$ pairs.

Real archives work through a recording map $\phi : \Omega \to \mathcal{X}$ sending world states or entities $\omega \in \Omega$ to feature tuples inside $\mathcal{X} = X\_1 \times X\_2 \times \cdots \times X\_k$, where $k$ fields are drawn from sets $X\_1, X\_2, \dots, X\_k$. Observed sample units $S\_n = \lbrace \omega\_1, \dots, \omega\_n \rbrace \subseteq \Omega$ produce a realized $n$-row record $\bigl(\phi(\omega\_i)\bigr)\_{i=1}^n$ whose populated feature-cell image is $\mathcal{O}\_n = \phi(S\_n) \subseteq \mathcal{X}$ (distinct from archive codomain $\mathcal{A}$ in §3, deduplicating units that share identical feature tuples whenever $\phi$ is non-injective), separating unrecorded units $\omega \in \Omega \setminus S\_n$ from unpopulated feature cells in $\mathcal{X} \setminus \mathcal{O}\_n$.

Empty cells in $\mathcal{X} \setminus \mathcal{O}\_n$ show unobserved combinations in $\mathcal{X}$ (or unseen units in $\Omega$), never proven impossibilities. Some cells stay empty because a finite sample missed a rare group, which capture–recapture methods bound using overlapping lists (Chao 1987, 1989; Chao & Colwell 2017; formalized in [Chapter 34](34-sample-population-archive.md)). Others stay empty because the intake form excluded that category altogether, just like the derailment outside the six boxes.

Before defining probability, an analyst must declare which subsets of $\Omega$ count as answerable questions. Take a tiny universe of three outcomes $\Omega = \lbrace 1, 2, 3 \rbrace$. Suppose a coarse sensor only flags whether outcome $1$ occurred. Then the entire inventory of yes-or-no questions it can answer is:

$$\mathcal{F} = \lbrace \emptyset,\; \lbrace 1 \rbrace,\; \lbrace 2, 3 \rbrace,\; \Omega \rbrace.$$

Read that inventory aloud: you can ask if nothing happened ($\emptyset$), if $1$ happened ($\lbrace 1 \rbrace$), if $1$ did not happen ($\lbrace 2, 3 \rbrace$), or if something happened ($\Omega$). Because $\mathcal{F}$ does not contain $\lbrace 2 \rbrace$ by itself, no calculation inside $\mathcal{F}$ can separate $2$ from $3$.

On finite sets like $\lbrace 1, 2, 3 \rbrace$, a clean record could list every subset in the power set $2^\Omega$ (all $2^3 = 8$ combinations). On an uncountably infinite line like the real numbers $\mathbb{R}$, trying to assign a consistent length to *every* wild subset breaks down. Using the axiom of choice, mathematicians construct **Vitali sets**, which are jagged, scattered point sets on $[0, 1)$ whose rational translates inside a bounded interval (such as shifts by $q \in \mathbb{Q} \cap [-1, 1]$ inside $[-1, 2]$, or modulo-$1$ circular shifts on $[0, 1)$) cannot carry translation-invariant countably additive lengths without summing to an impossible value. In three-dimensional space $\mathbb{R}^3$, the **Banach–Tarski paradox** goes further: a solid ball can be cut into finitely many non-measurable pieces and reassembled by rigid rotations and shifts into two balls of the original size.

Modern probability sidesteps those paradoxes by never asking for the size of every subset in $2^\Omega$. Instead, it restricts valid events to a **$\sigma$-algebra** $\mathcal{F} \subseteq 2^\Omega$. That family must contain the whole universe $\Omega$, stay closed under complements ($A \in \mathcal{F} \implies A^c \in \mathcal{F}$), and stay closed under countable unions ($A\_i \in \mathcal{F} \implies \bigcup\_{i=1}^\infty A\_i \in \mathcal{F}$). Think of $\mathcal{F}$ as a pre-registered question inventory. Sealing an archive freezes $\mathcal{F}$, leaving any omitted event $E \notin \mathcal{F}$ invisible until the institution expands the inventory to a finer algebra $\mathcal{F}'$ with $\mathcal{F} \subsetneq \mathcal{F}'$ ([Chapter 15](15-probability-as-a-measure.md)).

## 2. Relations: Equivalence and Order

A health ministry reviews blood-pressure readings from several rural clinics alongside patient service ratings on a five-point ordinal scale ($1 = \text{poor}$ to $5 = \text{excellent}$). Its report makes two classic mistakes. First, it treats every patient row as completely independent, ignoring that patients inside the same clinic share staff, protocols, and cuff calibration. Second, it treats a small bump in mean satisfaction score as a proportional gain in clinical care. The first error ignores an **equivalence relation** grouping rows into shared buckets; the second mistakes an **order relation** for an equal-spaced ruler.

A **binary relation** $R$ on a set $S$ is simply a subset of pairs $S \times S$. We write $a \sim b$ or $a \le b$ when the pair $(a, b)$ belongs to $R$. Two families of relations organize almost all empirical work: grouping into buckets and ranking by order.

An **equivalence relation** sorts items into non-overlapping buckets using three rules:

- **Reflexive:** every element matches itself ($a \sim a$).
- **Symmetric:** matching works both ways ($a \sim b \implies b \sim a$).
- **Transitive:** matches chain cleanly ($a \sim b \text{ and } b \sim c \implies a \sim c$).

Let us check a tiny four-patient set $S = \lbrace p\_1, p\_2, p\_3, p\_4 \rbrace$. Suppose $p\_1$ and $p\_2$ visit North Clinic while $p\_3$ and $p\_4$ visit South Clinic. Defining $a \sim b$ to mean "visited the same clinic" automatically splits $S$ into two disjoint equivalence classes: $[p\_1] = \lbrace p\_1, p\_2 \rbrace$ and $[p\_3] = \lbrace p\_3, p\_4 \rbrace$. By the fundamental theorem of equivalence relations, every equivalence relation partitions $S$ into non-overlapping classes $[a] = \lbrace x \in S : x \sim a \rbrace$.

Shared clinic, district, or sensor buckets shrink independent information below the raw row count whenever items in the same bucket correlate ([Chapter 14](14-linear-algebra.md); formalized in [Sample, Population, Archive](34-sample-population-archive.md)). Relational databases rely on the exact same bucket geometry through **functional dependencies** (Codd 1979). Grouping rows by candidate-key equivalence classes $[r]\_K$ keeps repeating facts consistent inside normalized tables ([Chapter 34](34-sample-population-archive.md)).

Rankings follow a different grammar. A **preorder** is reflexive ($a \le a$) and transitive ($a \le b \text{ and } b \le c \implies a \le c$), while allowing ties. If two patients $a \ne b$ both rate a clinic `3`, then $a \le b$ and $b \le a$ hold together. Merging tied items into equivalence buckets yields a **partial order**, where mutual ranking forces equality ($a \le b \text{ and } b \le a \implies a = b$, called **antisymmetry**).

A concrete three-element check separates ordinal rankings from cardinal numbers. Suppose three clinic ratings satisfy $1 < 2 < 3$. Relabeling them to $1 < 10 < 100$ preserves every order comparison (*worse*, *equal*, *better*), yet the arithmetic average of $1$ and $100$ is $50.5$ instead of the midpoint $2$ (which corresponds to $10$, not $50.5$, under the relabeling). Ordinal total (or weak) orders survive strictly increasing real relabelings, whereas general partial orders preserve incomparability only under order isomorphisms ($a \le b \iff g(a) \le g(b)$), while arithmetic means fail invariance under both (Stevens 1946; Krantz, Luce, Suppes & Tversky 1971).

Nor must a partial order put every pair on one line. On the real numbers, every pair obeys $x \le y$ or $y \le x$ (a **total order**). By contrast, think of three skills in a training curriculum: *arithmetic*, *algebra*, and *writing*. Arithmetic must precede algebra ($\text{arithmetic} \le \text{algebra}$), while writing has no ordering either before or after algebra; they are **incomparable**. Collapsing an incomparable partial order into one linear leaderboard invents a ranking the world never gave.

Formal classification adds one more distinction (**mereology**, the study of parts and wholes) alongside membership and kinds:

- **Instance membership ($x \in C$):** a concrete individual belongs to a class (for example, North Clinic is a rural clinic). Membership is neither reflexive nor transitive.
- **Taxonomic subsumption (`is_a`, $C\_1 \subseteq C\_2$):** every member of a narrower kind belongs to a broader kind (every rural clinic `is_a` health facility, so North Clinic is a health facility). This relation is a partial order supporting upward inheritance.
- **Mereological parthood (`part_of`, $\le\_{\mathrm{part}}$):** a component sits inside a larger whole (a blood-pressure cuff `part_of` North Clinic, or a brake fault `part_of` a train derailment).

Parthood $\le\_{\mathrm{part}}$ is a partial order (reflexive, antisymmetric, and transitive) that binds compatible sorts (physical objects inside physical objects, or events inside events) without reducing a living clinic to a bare bag of parts. Crucially, `part_of` does **not** turn a part into a subtype (`is_a`). Lifting individual parthood $\le\_{\mathrm{part}}$ to classes via existential parthood (every instance of class $A$ is $\le\_{\mathrm{part}}$ some instance of class $B$), `A part_of B` followed by `B is_a C` yields `A part_of C`, never `A is_a C`, just as `A is_a B` followed by `B part_of C` preserves existential parthood ([Chapter 64](64-layer-taxonomy-systematics-ontology-order.md) §3). Because every cuff belongs to a clinic and every clinic `is_a` health facility, a cuff is `part_of` a health facility, *not* a health facility, and a brake fault `part_of` a derailment is not itself a derailment. Mixing parthood with kind-inclusion corrupts how categories are framed and how denominators are counted before analysis even starts, a problem formalized in [Chapter 29](29-aim-and-the-three-planes.md) and audited in detail in [Chapter 64](64-layer-taxonomy-systematics-ontology-order.md)'s study of formal ontology and order.

## 3. Functions as Measurement and Recording Pipelines

An administrative intake form offers five checkboxes for cause of failure. Every unusual field report that misses the first four boxes gets dumped into box five ("other"). Once the form saves, no downstream analyst can un-pack "other" back into the distinct physical faults that produced it. That irreversible compression is what happens when a recording pipeline fails to preserve distinct inputs.

A **function** $f : A \to B$ assigns each input element $a \in A$ to exactly one output $f(a) \in B$. We call $A$ the **domain**, $B$ the **codomain**, and the subset of outputs actually reached, $f(A) = \lbrace f(a) : a \in A \rbrace \subseteq B$, the **image** (or range).

Three structural properties decide whether an analyst can recover inputs from outputs. Test each on a tiny finite map with three or four elements:

- **Injective (one-to-one):** distinct inputs never collide ($f(a\_1) = f(a\_2) \implies a\_1 = a\_2$). Let $A = \lbrace 1, 2, 3 \rbrace$ and $B = \lbrace x, y, z, w \rbrace$ with $1 \mapsto x$, $2 \mapsto y$, and $3 \mapsto z$. Seeing $y$ tells you uniquely that the input was $2$, even though code $w$ stayed unused.
- **Surjective (onto):** every output code is realized ($f(A) = B$). Let $A = \lbrace 1, 2, 3, 4 \rbrace$ and $B = \lbrace x, y, z \rbrace$ with $1 \mapsto x$, $2 \mapsto y$, $3 \mapsto z$, and $4 \mapsto z$. Every bucket is used, yet seeing $z$ cannot separate $3$ from $4$.
- **Bijective (reversible):** both injective and surjective. Let $A = \lbrace 1, 2, 3 \rbrace$ and $B = \lbrace x, y, z \rbrace$ with $1 \mapsto x$, $2 \mapsto y$, and $3 \mapsto z$. Only a bijection admits a clean two-sided inverse $f^{-1} : B \to A$ mapping $x \mapsto 1$, $y \mapsto 2$, and $z \mapsto 3$.

Measurement is a map $m : \mathcal{W} \to \mathcal{A}$ taking latent world states $\mathcal{W}$ into archive records $\mathcal{A}$. Almost always, $m$ fails both checks: empty codes sit unused while distinct world states land in one shared archive bucket, leaving a multi-element **fiber** $m^{-1}(y) = \lbrace w \in \mathcal{W} : m(w) = y \rbrace$. For our surjective toy example above, the fiber above $z$ is $m^{-1}(z) = \lbrace 3, 4 \rbrace$.

Picking one favorite value out of $m^{-1}(y)$ manufactures fake precision. Instead, Charles Manski's (1990) **partial identification** keeps the entire compatible fiber in view, a set-partitioning strategy that [Integrals and Accumulation](13-integrals-and-accumulation.md) §5 turns into expectation brackets by partitioning $\Omega$ inside the Lebesgue integral and [Inference and Evidence](35-inference-and-evidence.md) carries into data inference. Nor do Codd's (1979) single-valued database keys from §2 restore injectivity: a tidy keyed row can still lump distinct physical failures under one `"other"` code ([Chapter 34](34-sample-population-archive.md)).

Real reporting chains compose several stages in sequence: $(f\_k \circ f\_{k-1} \circ \cdots \circ f\_1)(x)$. Composition stays injective **if** every stage is injective, and **only if** $f\_1$ is injective and each later stage $f\_j$ stays injective on the upstream image $(f\_{j-1} \circ \cdots \circ f\_1)(A)$. Once an early intake step lumps values into coarse bins or caps values at an upper ceiling $K < y\_{\max}$, no downstream math can un-bin the data or recover the true support $[y\_{\min}, y\_{\max}]$. By contrast, architectural safeguards work best when built as bijections: when [Chapter 29](29-aim-and-the-three-planes.md) maps seven guard checks onto the seven transits of the `(World, Archive, Institution)` loop, that correspondence is one-to-one and onto so no crossing stays unguarded.

## 4. Why this chapter is first

Every chapter that follows builds on sets of world states, relations among observations, and maps from evidence to action. Probability is a measure on a set algebra, expectation is a weighted integral over level sets, and a predictive model is an approximation to a recording map. How those mathematical spaces assemble into non-substitutable margins belongs to [The Triad and the Minimum](41-the-triad-and-the-minimum.md) and [Chapter 50](50-methodological-families-reflexivity.md).

### Thirty-five triples across seven strata

Later in this book, [Aim and the Three Planes](29-aim-and-the-three-planes.md) and [Appendix B](appendix-b-architecture-verification.md) organize seven check sites, derived from the ordered crossings and closure of the three-plane loop, into combinations of three. Before encountering that full architecture, we can verify the counting arithmetic straight from basic set theory.

How many three-element subsets can you pick from $n$ distinct items, written $\binom{n}{3}$? Test a tiny four-item set $\lbrace a, b, c, d \rbrace$ by hand: the subsets of size three are $\lbrace a, b, c \rbrace$, $\lbrace a, b, d \rbrace$, $\lbrace a, c, d \rbrace$, and $\lbrace b, c, d \rbrace$, giving $\binom{4}{3} = 4$. With $n = 7$ items, the formula gives:

$$\binom{7}{3} = \frac{7 \times 6 \times 5}{3 \times 2 \times 1} = 35.$$

Read that aloud: thirty-five is also $7 \times 5$, allowing five distinct combinations to be grouped under each of seven primary anchors. Purely arithmetically, for any integer $n \ge 3$, setting $\binom{n}{3} = 5n$ yields $n^2 - 3n - 28 = 0$, whose unique positive integer root is $n = 7$ (since $n = -4 \notin \mathbb{N}$), and the ratio $b(n) = \binom{n}{3}/n = (n-1)(n-2)/6$ increases strictly by $b(n+1) - b(n) = (n-1)/3$ at every step, hitting $5$ only at $n = 7$; [Appendix A](appendix-a-mathematics.md) §A.6 and [Appendix B](appendix-b-architecture-verification.md) §B.2 build the thirty-five-triple partition across the seven transit sites. The seven sites themselves come solely from the three-plane circuit's transits, not from this combinatorial arithmetic.

### When formalisation does not pay

Heavy set notation is unnecessary when $\Omega$ is finite, small, and already resolved down to single outcomes ($\mathcal{F} = 2^\Omega$ with no coarse buckets or omissions). Often, however, an intake form builds a coarse algebra $\mathcal{F} \subsetneq 2^\Omega$ or omits hazards outside preset boxes, as in §1's six-code screen and [Chapter 15](15-probability-as-a-measure.md). Explicit set structure then keeps unlogged complements visible and leaves room for future expansion ($\mathcal{F} \subsetneq \mathcal{F}'$). Where $\mathcal{F} = 2^\Omega$ truly holds, measurability is automatic and attention belongs elsewhere.

## Bridge

[Aim and the Three Planes](29-aim-and-the-three-planes.md), [Sample, Population, Archive](34-sample-population-archive.md), and [Appendix B](appendix-b-architecture-verification.md) place latent states, non-injective recording maps, and the thirty-five combinatorial combinations into the full judgment circuit. Meanwhile, [Chapter 64](64-layer-taxonomy-systematics-ontology-order.md) applies formal ontology and parthood order to scientific taxonomy. [Numbers, Order, and Bounds](10-numbers-order-bounds.md) takes the next step in Part II, placing numbers on an ordered line, clarifying supremum and infimum bounds, and showing how intervals survive computer rounding and extreme tails.
