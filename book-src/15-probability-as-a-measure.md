# Probability as a Measure

> Seventh and last chapter of the mathematical block in Part II. It closes the mathematical base by
> giving probability its proper home: a measure on a set of outcomes.

## By the end of this chapter you can

1. State the three axioms as conditions on a function, and say what each one is protecting.
2. Explain what a random variable is, in the technical sense, without appealing to randomness.
3. Say what independence buys and what conditioning does to the space you are asking about.

## 1. The Measurable Structure: $(\Omega, \mathcal{F}, \mathbb{P})$

A rigorous probability space is a measure triple $(\Omega, \mathcal{F}, \mathbb{P})$ consisting of:

1. The **sample space** $\Omega$: the exhaustive set of all conceivable elementary outcomes $\omega$.
2. The **event space** $\sigma$-algebra $\mathcal{F} \subseteq 2^\Omega$: a family of subsets closed under
   complementation ($A \in \mathcal{F} \implies A^c \in \mathcal{F}$) and countable unions
   ($A_i \in \mathcal{F} \implies \bigcup_{i=1}^\infty A_i \in \mathcal{F}$), with $\Omega \in \mathcal{F}$.
3. The **probability measure** $\mathbb{P} : \mathcal{F} \to [0, 1]$: a countably additive set function.

Restricting $\mathbb{P}$ to a $\sigma$-algebra is not a technical formality. On uncountably infinite spaces
such as $\mathbb{R}$, non-measurable sets (such as Vitali sets constructed using the axiom of choice) cannot
be assigned a consistent length without destroying translation invariance. A $\sigma$-algebra represents the
pre-registered inventory of admissible questions that an institution is mathematically permitted to ask.

## 2. Kolmogorov Axioms and Cox's Plausibility

Andrey Kolmogorov (1933) established the three core axioms:

1. **Non-negativity**: $\mathbb{P}(A) \ge 0$ for all $A \in \mathcal{F}$.
2. **Unit normalization**: $\mathbb{P}(\Omega) = 1$.
3. **Countable additivity**: For any countable sequence of pairwise disjoint events $A_1, A_2, \dots \in \mathcal{F}$:

   $$\mathbb{P}\left(\bigcup_{i=1}^\infty A_i\right) = \sum_{i=1}^\infty \mathbb{P}(A_i).$$

These axioms are an operational contract against incoherence. By the Dutch book theorem (Ramsey, 1926;
de Finetti, 1937), any agent whose odds violate countable additivity can be offered a finite combination of bets,
each acceptable according to the agent's stated odds, whose joint settlement results in sure monetary loss.

Richard Cox (1946) demonstrated that if plausibility assessments over a Boolean algebra of propositions satisfy
basic desiderata of consistency, transitivity, and universal comparability, there exists an order-preserving
rescaling under which plausibilities obey Kolmogorov's rules. Probability is the unique normative logic of
uncertainty that avoids internal contradiction.

## 3. Random Variables and Measurability

A **random variable** is a measurable mapping $X : \Omega \to \mathbb{R}$ such that the pre-image of every Borel
interval belongs to the event space:

$$X^{-1}\bigl((-\infty, c]\bigr) = \{\omega \in \Omega : X(\omega) \le c\} \in \mathcal{F}, \qquad \forall c \in \mathbb{R}.$$

A random variable is not an uncertain number; it is a deterministic measurement function evaluated on an uncertain
world state. Measurability ensures that statements such as "the vital reserve falls below zero" correspond to
valid events in $\mathcal{F}$ to which $\mathbb{P}$ can assign a probability.

## 4. Conditioning, Martingales, and Ville's Inequality

Conditional probability updates the measure when an event $B$ with $\mathbb{P}(B) > 0$ is observed:

$$\mathbb{P}(A \mid B) = \frac{\mathbb{P}(A \cap B)}{\mathbb{P}(B)}.$$

Bayes' theorem follows as the algebraic inversion of conditioning:

$$\mathbb{P}(H_k \mid E) = \frac{\mathbb{P}(E \mid H_k)\mathbb{P}(H_k)}{\sum_j \mathbb{P}(E \mid H_j)\mathbb{P}(H_j)}.$$

Oliver Cromwell's rule (Lindley, 1985) warns that if a prior is assigned probability 0 or 1 ($\mathbb{P}(H) = 0$),
no amount of empirical evidence can ever update it ($\mathbb{P}(H \mid E) = 0$).

When data arrive sequentially over a filtration $(\mathcal{F}_t)_{t \ge 0}$, a stochastic process $(M_t)_{t \ge 0}$
is a **martingale** if $\mathbb{E}[|M_t|] < \infty$ and:

$$\mathbb{E}[M_{t+1} \mid \mathcal{F}_t] = M_t \quad \text{almost surely}.$$

Jean Ville (1939) proved the foundational maximal inequality for non-negative martingales:

$$\mathbb{P}\left(\sup_{t \ge 0} M_t \ge \frac{1}{\alpha}\right) \le \alpha, \qquad \forall \alpha \in (0, 1).$$

Ville's inequality is the bedrock of sequential analysis and anytime-valid testing. Unlike classical fixed-sample
tests that require pre-committing to sample size $N$ to prevent $p$-hacking, non-negative martingales (and e-processes)
permit continuous monitoring and optional stopping without inflating false-alarm rates.

## 5. Strategic Classification and Game-Theoretic Interaction

In real-world institutional settings, the probability space is not passive. When an institution deploys a decision
rule $f_\theta(x)$ to allocate resources, parole, credit, or inspections, evaluated agents act strategically:
they solve an optimization problem to manipulate their observable features $x \to x'$ at cost $c(x, x')$:

$$x'(x; \theta) = \arg\max_{z} \bigl(f_\theta(z) - c(x, z)\bigr).$$

This transforms static decision theory into a **Stackelberg game**. The institution acts as the leader, announcing
rule $\theta$; agents act as followers, gaming the metric. If the institution assumes the distribution
$\mathbb{P}_X$ is invariant, the deployed model degrades under Goodhart's law. Strategic classification demonstrates
that probability in institutional life is inherently interactive and game-theoretic.

## Bridge

[Probability as Language](30-probability-as-language.md) and [Reactive Worlds, Performativity, and Goodhart's Law](48-reactive-worlds-performativity-goodhart.md)
extend measure-theoretic probability to reactive and strategic environments. In the Stochology library,
`governance.rs::GovernanceState` verifies game-theoretic institutional gates and audit trails, while
`epistemic.rs::from_e_process` constructs anytime-valid e-processes that guarantee false-alarm control under
arbitrary optional stopping.
