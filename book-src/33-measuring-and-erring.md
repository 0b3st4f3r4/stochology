# Measuring and Erring

> Foundation chapter of Part III. It presupposes [Chapter 10](10-numbers-order-bounds.md) and
> [Chapter 12](12-derivatives-and-convexity.md). Its formal content is worked out in
> [Chapter 43](43-inquiry-openness-multiverse.md) and
> [Chapter 46](46-non-identical-asymmetry-heavy-tails.md).

## By the end of this chapter you can

1. Distinguish precision from accuracy, and name the case where more precision makes a decision
   worse.
2. Identify the measurement scale of a variable (nominal, ordinal, interval, ratio) and say which
   operations that scale permits.
3. Refuse a summary statistic that is not invariant under the transformations the scale allows.

## 1. Precision, accuracy, units, and metrological traceability

Precision is a property of a measurement apparatus: how finely it differentiates between neighboring states.
Accuracy is a property of a factual proposition: how closely it corresponds to the physical reality it claims
to describe. A measurement can be exquisitely precise while being completely inaccurate — a micrometer gauge
calibrated with an offset bias — and it can be accurate while remaining coarse, which is the honest condition of
most human knowledge.

The institutional failure that plagues modern data systems is **false precision**: presenting an estimate at
a decimal resolution that the underlying empirical procedure cannot justify. Publishing a risk probability to
four significant digits derived from seventeen historical incidents, or reporting a socio-economic index to
three decimal places from a survey with an unmeasured non-response rate, is not harmless decoration. Digits
constitute an explicit epistemic claim about the resolving power of the evidence, and decision-makers are
entitled to act upon that claim. A number published without an uncertainty interval is a claim to infinite
precision.

International physical metrology codifies this discipline in the **Guide to the Expression of Uncertainty in
Measurement** (`BIPM/JCGM GUM` `[Literature]`). Total measurement variance decomposes into two distinct regimes:
- **Type A uncertainty**: Evaluated by statistical analysis of series of observations, dominated by random
  stochastic scatter that shrinks as $1/\sqrt{n}$.
- **Type B uncertainty**: Evaluated by scientific judgment using prior calibration certificates, instrument
  resolution limits, temperature drift, and the **definitional uncertainty** ($u_{\mathrm{def}}$) inherent in
  specifying the measurand itself. Type B uncertainty remains strictly $O(1)$ regardless of sample size $n$.

Furthermore, under the **unbroken chain of metrological traceability** (`VIM 2.41` `[Literature]`), every link
in an empirical calibration hierarchy must inherit the expanded uncertainty of the standard upstream from it.
No downstream statistical manipulation can ever legitimately report an uncertainty narrower than the reference
standard against which its instruments were calibrated.

## 2. Stevens's scale taxonomy and permissible transformations

Variables do not all measure the world in the same mathematical way. In 1946, S. S. Stevens established the
definitive taxonomy of measurement scales, defined by their mathematical invariance under specific transformation
groups:

```
+-----------+-----------------------------+------------------------------------+--------------------------------+
| Scale     | Defining Relation           | Permissible Group Transformation   | Invariant Mathematical Meaning |
+-----------+-----------------------------+------------------------------------+--------------------------------+
| Nominal   | Equivalence (=, !=)         | One-to-one substitution (f(x))     | Mode, contingency contingency  |
| Ordinal   | Greater than (>, <)         | Strictly monotonic (f'(x) > 0)     | Median, percentiles, dominance |
| Interval  | Known unit, arbitrary zero  | Positive affine (f(x) = ax + b)    | Mean, variance, Pearson r      |
| Ratio     | Natural physical zero       | Positive similarity (f(x) = ax)    | Geometric mean, variation coef |
+-----------+-----------------------------+------------------------------------+--------------------------------+
```

Two widespread practices commit severe scale violations:
1. *Averaging ordinal codes*: Computing an arithmetic mean over ordinal Likert scales ("strongly agree = 5",
   "neutral = 3") or clinical triage categories treats qualitative rank orders as if they possessed constant
   interval units. A monotone transformation of the numerical labels can reverse the ranking of two group means.
2. *Ratios of interval quantities*: Stating that a temperature of $40^\circ\text{C}$ is "twice as warm" as
   $20^\circ\text{C}$ is meaningless; in the Fahrenheit scale ($104^\circ\text{F}$ vs $68^\circ\text{F}$), the
   ratio is $1.53$. Ratios are licensed only when the scale possesses a true physical zero.

## 3. The invariance test and non-parametric dominance

The operational check that enforces scale discipline is the **invariance test**:
> Take any proposed summary statistic or decision rule. Apply an arbitrary mathematical transformation that the
> variable's measurement scale permits. If the substantive decision, ranking, or qualitative conclusion changes,
> then the statistic was measuring the arbitrary artifact of the coordinate system, not the physical state of the world.

When applied to ordinal observations $X$ and $Y$, the arithmetic mean fails the invariance test instantly.
By contrast, the **Mann-Whitney stochastic dominance** probability:

$$\Delta_{\mathrm{MW}} = \mathbb{P}(X > Y) + \frac{1}{2}\mathbb{P}(X = Y) \qquad \text{\texttt{[Assumed]}},$$

passes the invariance test under any strictly increasing monotonic transformation. Whether an experimental
treatment produces systematically faster recovery times than a standard therapy is a purely ordinal claim; it
does not require hospital stay duration to be measured on an idealized ratio scale, and it remains invariant
even if the hospital logging clocks drifted monotonically during the observation window.

In psychometrics and educational measurement, Samuel Messick's unified validity framework `[Literature]`
formalized three structural threats that persist even when scale transformations are respected:
- **Construct underrepresentation**: The metric omits critical, unmeasured dimensions of the underlying capacity.
- **Construct-irrelevant variance**: The score is contaminated by extraneous noise, language barriers, or socioeconomic
  friction unrelated to the true competence.
- **Consequential invalidity**: The social, institutional, and economic damage triggered when the metric is tied to
  high-stakes incentives and warps operational behaviour.

These three psychometric threats correspond 1-to-1 with the framing ($G_1$), archive ($G_3$), and performative
($G_6$) guard sites of `grammar.rs::GuardSite`.

## 4. Missing is not zero

An empty cell in an empirical database has three mutually incompatible interpretations, and software libraries
routinely conflate them:
- **Not observed**: The physical event occurred, but the instrument failed to record it.
- **Observed as absent**: The instrument inspected the state and confirmed that the event did not occur.
- **Not askable**: The category does not exist in the database schema, so the phenomenon cannot be represented.

Treating unobserved cases as absent systematically biases damage rates downward, creating an illusion of safety
in precisely the most vulnerable regions. Worse, treating unaskable categories as absent converts institutional
blindness into an assertion of non-existence.

The evidential weight of an archive cannot be measured by raw byte volume. Following John Maynard Keynes's *Treatise
on Probability*, evidential weight $V(x, q)$ depends upon the completeness of the archive relative to the domain
of inquiry, governed by the effective sample size $n_{\mathrm{eff}}$:

$$V(x, q) = \frac{n_{\mathrm{eff}}}{n_{\mathrm{eff}} + n_0} \in [0, 1) \qquad \text{\texttt{[Assumed]}},$$

where $n_0$ represents the prior scale constant of genuine unobserved uncertainty. An archive with millions of
rows collected under an unmeasured selection filter carries massive false precision, yet its genuine Keynesian
weight remains near zero.

## Bridge

Scale invariance and non-parametric stochastic dominance are formalised in
[Chapter 46](46-non-identical-asymmetry-heavy-tails.md) as `vital.rs::mann_whitney_dominance`; the
metrological and evidential weight of an archive grounded in effective sample size is formalised in
[Chapter 45](45-world-to-archive-big-data-paradox.md) through Keynes's weight
`epistemic.rs::EpistemicMarginState::keynes_weight_from_n_eff`, while the structural integrity of
the measuring transit is audited across the sites of `grammar.rs::GuardSite`.
