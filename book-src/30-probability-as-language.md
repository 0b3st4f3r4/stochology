# Probability as Language

> Foundation chapter of Part III. It presupposes [Chapter 10](10-numbers-order-bounds.md) and
> [Chapter 15](15-probability-as-a-measure.md). Its formal content is worked out in
> [Chapter 44](44-underdetermination-partial-identification.md).

## By the end of this chapter you can

1. Write down the axioms of probability and say what each one forbids you to believe.
2. Update a belief with Bayes' rule and read the result as a statement about evidence, not about
   the world.
3. Explain why an *anytime-valid* statement is different from a statement that is valid at one
   fixed sample size.

## 1. The axioms and what they exclude

Kolmogorov's three foundational conditions of measure-theoretic probability are customarily presented
as constructive definitions. In decision science, they are far better understood as strict prohibitions
on a book of beliefs:

- **Non-negativity** ($\mathbb{P}(A) \ge 0$) forbids holding a degree of belief below impossibility.
- **Normalization** ($\mathbb{P}(\Omega) = 1$) forbids holding a set of mutually exclusive and collectively
  exhaustive outcomes whose degrees of belief do not sum to unity — that is, it strictly prohibits omitting
  unforeseen events while asserting complete coverage.
- **Countable additivity** ($\mathbb{P}(\bigcup_i A_i) = \sum_i \mathbb{P}(A_i)$ for disjoint events) forbids
  assigning more or less credibility to an aggregate claim than the sum of its partitioned possibilities.

The justification for these conditions is operational rather than aesthetic: as demonstrated in the Dutch book
theorem of [Probability as a Measure](15-probability-as-a-measure.md) §2, an agent whose degrees of belief violate
any of these three axioms can be subjected to a system of simultaneous bets that guarantees a sure financial loss
regardless of how the world turns out. Coherence is not a decorative virtue of academic formalism; it is the
minimal mathematical requirement under which an epistemic claim can bind future action without self-destruction.

Crucially, the axioms do not dictate *which* specific numerical probabilities an agent should hold. Two rational
observers can adhere to all three Kolmogorov axioms with absolute fidelity and yet hold diametrically opposed
beliefs based on differing historical priors. Probability theory is a formal syntax for reasoning about evidence,
not an oracle for dictating truth.

## 2. Measurement scales, invariance, and the dialectical penumbra

Before numbers can enter probabilistic calculations, their physical meaning must be grounded in measurement theory.
In 1946, S. S. Stevens introduced a foundational taxonomy of measurement scales, defined strictly by their groups
of **permissible transformations**:
- **Nominal scale**: Numbers serve purely as discrete labels. The permissible group is all one-to-one substitutions
  (permutations). Permissible operations are limited to counts, modes, and equivalence relations.
- **Ordinal scale**: Numbers represent rank order ($A > B > C$), but the numerical distances between ranks have no
  physical meaning. The permissible group is all strictly monotonic increasing transformations ($x \mapsto f(x)$
  with $f'(x) > 0$). Permissible statistics include medians, quantiles, and rank correlations.
- **Interval scale**: Equal numerical intervals reflect equal quantities of the measurand, but the zero point is
  arbitrary (e.g., Celsius temperature). The permissible group is positive affine transformations
  ($x \mapsto ax + b$ with $a > 0$). Permissible statistics include arithmetic means, variances, and standard
  deviations.
- **Ratio scale**: Possesses a natural, non-arbitrary physical zero (e.g., Kelvin temperature, mass, elapsed time).
  The permissible group is similarity transformations ($x \mapsto ax$ with $a > 0$). Permissible operations include
  ratios, geometric means, and coefficients of variation.

Violating scale invariance is the most pervasive source of pseudo-precision in organizational reporting. Averaging
ordinal customer satisfaction scores or clinical triage codes ("severe = 3, moderate = 2, mild = 1") treats an
arbitrary monotonic relabeling as an interval metric, fabricating mathematical units that the underlying phenomenon
does not possess.

Furthermore, real-world judgment constantly operates within what logicians term the **dialectical penumbra**
(or margin of vagueness): the boundary region where categorical distinctions ("safe vs unsafe", "healthy vs
diseased", "employed vs unemployed") are contested and inherently context-dependent. Forcing sharp real-valued
probabilities onto concepts located within the penumbra creates a dangerous illusion of certainty. When the
underlying scale is ordinal, the only invariant statements are those based on stochastic order, such as
Mann-Whitney dominance $\mathbb{P}(X > Y) + \frac{1}{2}\mathbb{P}(X = Y)$.

## 3. Conditioning and belief revision

A conditional probability is not a novel kind of probability; it is the original probability evaluated over a
restricted sub-universe: given that evidence $E$ occurred, we agree to restrict our inquiry to the subset of
worlds where $E$ holds true. That restriction is what makes empirical observation informative: it eliminates
untenable hypotheses, requiring the surviving beliefs to be renormalized over the remaining space.

Bayes' rule provides the exact arithmetic of this renormalization:

$$\mathbb{P}(H \mid E) = \frac{\mathbb{P}(E \mid H)\,\mathbb{P}(H)}{\mathbb{P}(E \mid H)\,\mathbb{P}(H) + \mathbb{P}(E \mid \neg H)\,\mathbb{P}(\neg H)}.$$

The operational lesson of Bayes' rule is concentrated in the denominator. The posterior probability depends
every bit as heavily on the prior base rate $\mathbb{P}(H)$ as on the likelihood ratio of the evidence. Even an
instrument with a remarkable 99% sensitivity and 95% specificity deployed against a rare event with prevalence
$\mathbb{P}(H) = 0.001$ will produce a posterior probability:

$$\mathbb{P}(H \mid +) = \frac{0.99 \times 0.001}{(0.99 \times 0.001) + (0.05 \times 0.999)} \approx \frac{0.00099}{0.00099 + 0.04995} \approx 0.0194 \qquad \text{\texttt{[Assumed]}}.$$

Over 98% of positive alarms will be false. The rarer the event, the more dramatically the posterior is governed
by the ambient base rate rather than the nominal accuracy of the detector.

## 4. Evidence that accumulates: e-processes and anytime validity

Classical Neyman-Pearson hypothesis testing requires sample size $n$ to be pre-registered and fixed in advance.
If an analyst continuously monitors accumulating data and stops the experiment the moment a p-value drops below
$0.05$, the false positive rate inflates dramatically from $5\%$ to $30\%$ or higher. This "optional stopping"
pathology invalidates classical statistical inference.

Modern game-theoretic probability and the theory of martingales resolve this dilemma through **e-processes**.
An *e-value* $E_t$ is a non-negative random variable whose expectation under the null hypothesis is bounded by unity:
$\mathbb{E}_{H_0}[E_t] \le 1$. Under repeated testing, an e-process is a non-negative supermartingale. Jean Ville's
(1939) maximal inequality guarantees that the probability of the process *ever* exceeding a critical threshold
$1/\alpha$ across an infinite time horizon is strictly bounded:

$$\mathbb{P}_{H_0}\left(\exists t \ge 1:\ E_t \ge \frac{1}{\alpha}\right) \le \alpha \qquad \text{\texttt{[Assumed]}}.$$

Because the Ville inequality holds uniformly over all stopping times $\tau$, an investigator or regulatory agency
can monitor an ongoing production stream, clinical trial, or financial market continuously. Stopping early when
evidence is overwhelming does not compromise statistical validity; the safety guarantee is baked into the
supermartingale structure from the outset. This anytime validity licenses continuous institutional surveillance
without the arbitrary penalties of classical alpha-spending schedules.

## 5. Probability is a language, not an intrinsic property

There is an obstinate metaphysical temptation to treat probability as an intrinsic physical property of an
object, analogous to mass, volume, or electrical charge. Two honest observers, possessing different institutional
histories and differing prior evidence, can evaluate the exact same empirical data, apply Bayes' rule with
impeccable arithmetic, and remain permanently apart in their posterior conclusions.

This divergence is not a scandal. It is the reason probability functions as a transparent language for a plural
democracy. What the discipline of Stochology forbids is the authoritarian appeal to "the" objective probability
as an unquestionable mandate that silences political and ethical contestation. When two parties disagree on an
action under uncertainty, the proper response is to decompose the decision into its constitutive elements — the
prior bounds, the loss matrix, the scale invariance, and the empirical evidence — so that the locus of disagreement
is laid bare.

## Bridge

Sequential evidence and anytime validity are formalised in
[Chapter 44](44-underdetermination-partial-identification.md) as e-processes
(`epistemic.rs::EpistemicMarginState::from_e_process`) and their anytime-valid confidence radius
(`epistemic.rs::anytime_valid_radius`), where the Ville maximal supermartingale inequality provides the
mathematical calibration that lets an epistemic bound be quoted at any arbitrary stopping time without
sacrificing error control.
