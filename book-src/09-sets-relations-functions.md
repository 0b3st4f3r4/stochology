# Sets, Relations, Functions

> First chapter of the mathematical block in Part II. Nothing here is specific to judgment; it is the
> notation the rest of the book uses.

## By the end of this chapter you can

1. Write a set, a relation and a function with correct notation, and say what each one is *for*.
2. Distinguish an equivalence relation from an order, and give one example of each from data work.
3. Say what it means for a function to be injective, surjective or bijective, and why the distinction
   decides whether a quantity can be recovered from a record.

## 1. Sets and Measurable Spaces

A set is a collection of mathematical or physical objects, defined by membership: write $x \in A$ for
membership and $x \notin A$ for its negation. Two sets are equal if and only if they share identical
members (axiom of extensionality).

The foundational operations are:

- **Union** $A \cup B = \{x : x \in A \text{ or } x \in B\}$;
- **Intersection** $A \cap B = \{x : x \in A \text{ and } x \in B\}$;
- **Set difference** $A \setminus B = \{x : x \in A \text{ and } x \notin B\}$;
- **Complement** $A^c = \Omega \setminus A$ relative to an ambient universe $\Omega$;
- **Cartesian product** $A \times B = \{(a, b) : a \in A, b \in B\}$.

In data systems, an archive is a subset of an ambient product space: if an observation comprises $k$
features, the admissible domain is the Cartesian product $\mathcal{X} = X_1 \times X_2 \times \cdots \times X_k$.
When an archive records $n$ rows, it populates an empirical subset $\mathcal{A}_n \subset \mathcal{X}$.
Empty cells and missing combinations in this product space represent unobserved domain regions rather
than proven impossibilities.

When the universe $\Omega$ is uncountably infinite, assigning a consistent size or probability to every
arbitrary subset leads to mathematical antinomies such as the Banach–Tarski paradox. Modern probability
resolves this by restricting valid events to a $\sigma$-algebra $\mathcal{F} \subseteq 2^\Omega$ — a collection
of subsets containing $\Omega$ that is closed under complementation ($A \in \mathcal{F} \implies A^c \in \mathcal{F}$)
and countable unions ($A_i \in \mathcal{F} \implies \bigcup_{i=1}^\infty A_i \in \mathcal{F}$). A $\sigma$-algebra
is a pre-registered inventory of well-formed questions.

## 2. Relations: Equivalence and Order

A binary relation $R$ on a set $S$ is a subset of the Cartesian product $S \times S$, written $a \sim b$
or $a \le b$ when $(a, b) \in R$. Two canonical classes of relations govern all measurement:

An **equivalence relation** is reflexive ($a \sim a$), symmetric ($a \sim b \implies b \sim a$), and
transitive ($a \sim b \text{ and } b \sim c \implies a \sim c$). By the fundamental theorem of equivalence
relations, any equivalence relation partitions $S$ into disjoint equivalence classes $[a] = \{x \in S : x \sim a\}$.
In empirical work, every grouping — by clinical site, school district, sensor batch, or demographic
cohort — is an equivalence partition. Treating clustered data as if each unit were independent ignores
this partition, underestimating variance by the cluster design effect $\mathrm{DEFF} = 1 + (m - 1)\rho$.

A **partial order** is reflexive ($a \le a$), antisymmetric ($a \le b \text{ and } b \le a \implies a = b$),
and transitive. An order allows comparative statements — *worse*, *equal*, *better* — without requiring a
cardinal distance metric. Ordinal rankings are invariant to strictly monotonic transformations, whereas
cardinal arithmetic averages are not. Conflating orders with equivalence partitions leads to the common
pathology of averaging ordinal survey ranks as though equal numerical intervals represented equal physical
differences.

## 3. Functions as Measurement and Recording Pipelines

A function $f : A \to B$ assigns to each element $a \in A$ a unique image $f(a) \in B$. The set $A$ is the
domain, $B$ is the codomain, and the subset $f(A) = \{f(a) : a \in A\} \subseteq B$ is the image or range.

Three structural properties dictate whether information can be recovered without loss:

- **Injective** (one-to-one): $f(a_1) = f(a_2) \implies a_1 = a_2$. No distinct states are merged.
- **Surjective** (onto): $f(A) = B$. Every state in the codomain is realized by at least one input.
- **Bijective**: both injective and surjective. There exists a unique two-sided inverse $f^{-1} : B \to A$.

Measurement in the physical and social worlds is a function $m : \mathcal{W} \to \mathcal{A}$ mapping latent
world states $\mathcal{W}$ into recorded archive states $\mathcal{A}$. In realistic settings, $m$ is almost
never injective: multiple distinct world states generate identical sensor readings or survey responses.
When injectivity fails, the pre-image $m^{-1}(y) = \{w \in \mathcal{W} : m(w) = y\}$ is a set rather than
a point. Point estimation replaces this set with an arbitrary single element; honest partial identification
preserves the entire pre-image set.

Pipelines are functional compositions $(f_k \circ f_{k-1} \circ \cdots \circ f_1)(x)$. Composition preserves
injectivity only if every constituent stage is injective. The moment a single transformation in a pipeline
discards information (such as coarse binning or threshold truncation), injectivity is destroyed irreversibly
for all downstream analyses.

## 4. Axiomatic Foundations: Kolmogorov and Cox

The representation of uncertainty rests on how set operations map to real-valued plausibilities.
Andrey Kolmogorov (1933) grounded probability in measure theory over a measurable space $(\Omega, \mathcal{F})$,
imposing non-negativity, unit normalization, and countable additivity over disjoint sets.

Richard Cox (1946) approached the same structure from operational consistency. Demanding that plausibility
evaluations over a Boolean lattice of propositions satisfy transitivity, consistency, and continuous
differentiability forces plausibility measures to be isomorphic to Kolmogorov probability. Where Kolmogorov
presupposes a measure space, Cox derives probability as the unique consistent extension of Boolean logic.
Both frameworks agree on the operational consequence: any decision procedure whose weights violate set-theoretic
additivity can be exploited by a Dutch book, suffering certain loss across mutually exclusive alternatives.

## 5. Why this chapter is first

Every subsequent chapter formalizes claims over sets of possible worlds, relations among incentives, and
functions that map states to actions. Probability is a normalized measure on a $\sigma$-algebra. Expectation
is an integral of measurable functions. A model is an empirical approximation to a latent functional map.
Having these set-theoretic primitives in hand ensures that we do not mistake an arbitrary summary of a set
for the set itself.

## Bridge

[What Judging Is, and What It Is Not](27-what-judging-is.md) applies these objects to the full decision
circuit: the world is a space of latent states $\mathcal{W}$, the archive is the non-injective image
$m(\mathcal{W})$, and the decision is a policy function selecting actions under partial identification.
In the Stochology library, `epistemic.rs::EpistemicMarginState` tracks this foundational gap between
latent states and observable sets, while `grammar.rs::CircuitTransit` formalizes the transitions of the
mediation loop across world, archive, and institution.
