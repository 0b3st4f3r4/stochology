# What a Number Doesn't Say

> School-level chapter of Part I. It needs only percentages. The formal version is
> [Measuring and Erring](33-measuring-and-erring.md) and
> [Probability as Language](30-probability-as-language.md).

## By the end of this chapter you can

1. Distinguish an instrument's resolution (precision) from its calibration against reality (accuracy).
2. Compute why a diagnostic test with 95% sensitivity produces over 80% false alarms in low-prevalence regimes (base rate neglect).
3. Identify the missing dimensions that must accompany a reported metric — evidential weight, sample wobble, and operational alert fatigue.

## 1. Precise and correct

A bathroom scale shows 63.42 kilograms. It is precise to the hundredth of a kilogram. It is also, if
the scale was never calibrated, wrong by two kilograms every single time `[Assumed]`.

The second fact is the one that matters, and the first fact is the one that gets read. The decimals
look like knowledge. They are only resolution: the scale is telling you it can distinguish 63.41 from
63.42, not that either number is your mass.

This is the difference between **precision** and **accuracy**. Precision is how finely the instrument
distinguishes. Accuracy is whether the answer is close to the truth. A measurement can be precise and
wrong, accurate and vague, or — when things go well — both.

Reports confuse the two all the time, and often on purpose, because decimals persuade. A number with
two decimal places looks more considered than a number with none, even when both came out of the same
uncalibrated guessing.

## 2. Percentages of what: Natural frequencies and alert fatigue

Here is the case that surprises almost everyone the first time they encounter it.

A medical diagnostic test is remarkably good: it catches 95 out of every 100 people who have a condition
(a sensitivity of 95%). It also raises a false alarm for 5 out of every 100 people who are completely
healthy (a false positive rate of 5%). In a general population where 1 person in 100 has the condition
(a base rate or prevalence of 1%), your test comes back positive `[Assumed]`.

What is the chance you actually have the condition?

Most people guess something near 95%. The true answer is about 16.1%.

To see why, translate the percentages into a **natural frequency tree** across a thousand people:

1. **Prevalence:** Out of 1 000 people, exactly 10 have the condition, and 990 are healthy.
2. **True Positives:** Among the 10 who are sick, the test catches 95%: $10 \times 0.95 = 9.5$ people.
3. **False Positives:** Among the 990 who are healthy, the test sounds a false alarm on 5%:
   $990 \times 0.05 = 49.5$ people.
4. **Total Positives:** The test fires for $9.5 + 49.5 = 59$ people in total.

Now ask the question from the perspective of someone holding a positive test slip: Out of the 59 people
who tested positive, how many are actually sick?

$$\mathrm{PPV} = \frac{9.5}{59} \approx 0.161 \implies 16.1\%$$

Out of 59 positive alerts, nearly 50 are false alarms. Over 83% of the alarms are sirens sounding in an
empty room.

This is not a mathematical curiosity: it is a primary driver of real-world catastrophe. In a landmark
2021 study of the widely deployed Epic Sepsis Model across hospital electronic health records (Wong et
al. 2021), researchers found that the algorithm had an 88% false-alarm rate ($1 - \mathrm{PPV} = 0.88$)
`[Literature]`.
Because the base rate of acute sepsis among all admitted patients is low, thousands of daily beeping
monitors created severe **alert fatigue**. Nurses and doctors systematically learned to silence or ignore
the warnings — ensuring that when a genuinely septic patient deteriorated, the warning was dismissed as
just another spurious ping.

Nothing was wrong with the arithmetic of the algorithm's internal sensitivity. The failure lay in
forgetting the base rate: confusing $\mathbb{P}(\text{Alert}\mid\text{Disease})$ with
$\mathbb{P}(\text{Disease}\mid\text{Alert})$.

## 3. A number with no room

The last thing a bare number hides is how much it would have moved.

Suppose a school reports that the average test score rose from 68 to 71. Is that an improvement? It
depends on how many students were involved and how much scores normally fluctuate. With five students,
a three-point move is what happens when two students had a good breakfast. With five thousand students,
it represents a substantial educational shift `[Assumed]`.

A number that arrives without a sense of its own wobble is not a measurement. It is a claim at
infinite confidence, made by someone who did not compute the confidence. The honest form of the claim
includes the evidential weight and the uncertainty interval — and the width of that interval is often
far more informative than the point estimate itself, because it tells you whether anyone should take
costly action or whether the movement is within normal system noise.

## 4. What to ask

Three questions catch most of this distortion, and they cost nothing:

1. *Precise or correct?* Where did this number come from, and when was the measuring instrument last
   calibrated against an independent ground truth?
2. *Out of what?* A percentage is meaningless until you know the denominator, and a conditional probability
   is misleading until you confirm which direction it conditions upon.
3. *How much would it move?* If the measurement were repeated next week, what is its expected variance,
   and does our vital reserve buffer have enough slack to absorb the true worst-case swing?

## Bridge

[Measuring and Erring](33-measuring-and-erring.md) turns these questions into an audit that every summary
statistic must pass; [Probability as Language](30-probability-as-language.md) formalizes Bayesian
conditioning and natural frequency trees. In pure Rust, `epistemic.rs::keynes_weight_from_n_eff` computes
the Keynesian evidential weight $V(n_{\mathrm{eff}}) \in [0, 1]$ to penalize ungrounded precision, while
`vital.rs::VitalMarginState` tests whether the system maintains sufficient physical reserves to survive
operational errors through the survival check `vital.rs::VitalMarginState::survives_disruption`.
[Who Is Missing From the Table](06-who-is-missing-from-the-table.md) examines what happens when the
records themselves omit entire categories of people.
