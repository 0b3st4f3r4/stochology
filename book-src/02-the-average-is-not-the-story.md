# The Average Is Not the Story

> School-level chapter of Part I. It needs only division. The formal version is
> [Distribution and Moment](31-distribution-and-moment.md), and the tail argument is
> [The Biggest Number in the Room](03-the-biggest-number-in-the-room.md).

## By the end of this chapter you can

1. Compute an average and identify the dispersion, tail risk, and selection bias it systematically leaves out.
2. Give two sets of numbers with the same average and completely different behaviour under nonlinear and convex penalties.
3. Explain why planning with an average can be right about the number and wrong about the plan, showing how sample size inflates false certainty when the collection door is biased.

## 1. One number for many cases

A football team scores 2, 0, 3, 1, and 4 goals in five matches. The average is 2. It is a correct
summary: if you had to describe those five matches with one number, 2 is the honest choice.

Now suppose you are the coach and you want to know whether you will win next Saturday. The average
tells you what a typical match looked like. It does not tell you how far the team can fall. Two teams
can both average 2 goals and be completely different: one scores 2 every week, the other scores 0, 0,
0, 0, and 10.

The second team is not unusual in sport, in business, or in weather. And the difference between the
two is invisible in the average, because averaging is by design an operation of erasure: it sums the
instances and divides by their count, deliberately forgetting the path, the variance, and the extremes.

## 2. A hundred numbers and the convexity trap

Here is an example small enough to check by hand. You have a hundred measurements. Ninety-nine of
them equal 1. One of them equals 1 001.

The average is 11. Not approximately: exactly. Add them up — that is 99 plus 1 001, which is 1 100 —
and divide by 100:

$$\bar{X} = \frac{99 \times 1 + 1 \times 1001}{100} = \frac{1100}{100} = 11$$

Now ask what those numbers are like. Almost every value is 1. There is one value a thousand times
bigger. If you planned with the average, you would expect something near 11 every time, and you would
be wrong ninety-nine times out of a hundred and catastrophically wrong once. The median — the middle
value with half the observations above and half below — is 1.

The danger deepens when consequences are nonlinear. In real systems, the harm of a shortage or delay
is not proportional: running 10% below capacity causes minor inconvenience, but running 50% above
capacity triggers cascading grid collapse or emergency room gridlock. Mathematically, whenever a penalty
function $\phi(X)$ is **convex** (curving upward like a bowl), the expected loss is strictly greater
than the loss evaluated at the average:

$$\mathbb{E}[\phi(X)] > \phi(\mathbb{E}[X])$$

This inequality (known as **Jensen's inequality**) means that planning for the average guarantees
systematic under-provisioning. The difference $\Delta_J = \mathbb{E}[\phi(X)] - \phi(\mathbb{E}[X])$
is the **convexity gap** — the unbudgeted ruin that falls on whoever planned on the mean.

## 3. Spread and shape

There is a second thing you can measure about a set of numbers, beyond where its middle is: how far
it spreads.

Compare two classes that both average 70 on the same test. In one class the marks are 68, 69, 70, 71,
72. In the other they are 40, 55, 70, 85, 100. Both average 70, and no teacher would describe them
the same way. In the first, everybody learned roughly the same thing. In the second, half the class
is lost and half is bored.

Spread is not a nuisance to be averaged away. It is often the most important fact about a set of
numbers, because the extremes are where the failures happen.

## 4. Why plans break: Big data and the illusion of certainty

When an organisation plans with an average, it is making a bet that the unusual case will not arrive
during the period it cares about. That bet is usually fine, and occasionally it is fatal.

A hospital with an average of ten arrivals per hour is fine at eleven and in trouble at thirty. A
school bus designed for the average number of students is comfortable on Tuesday and leaves children
behind on the day of the trip. The average was never the question; the question was how bad the worst
hours get, and how often.

Worse still is the modern temptation to believe that collecting massive amounts of data solves this
problem. When an organisation collects ten million rows of data through a biased channel — for instance,
voluntary smartphone responses that under-represent the elderly or the destitute — the sample average
does not converge to the truth. As the statistician Xiao-Li Meng demonstrated in the **Big Data Paradox**,
when the correlation between whether an event is recorded and its outcome is non-zero ($\rho_{R,Y} \ne 0$),
accumulating millions of observations merely shrinks the confidence interval around a completely wrong
number. The institution becomes mathematically certain of an illusion.

That is why an honest analysis never stops at a point average. It must report the spread, the convex
penalty gap, and the limits of the archive that produced it.

## Bridge

[Distribution and Moment](31-distribution-and-moment.md) makes this precise: the distribution replaces
the average, and the convexity gap $\Delta_J = \mathbb{E}[\phi(X)] - \phi(\mathbb{E}[X])$ explains why
a loss evaluated at the mean understates the true expected loss. In pure Rust,
`vital.rs::VitalMarginState` computes this Jensen gap directly alongside heavy-tail indices, while
`epistemic.rs::EpistemicMarginState` guards against the Big Data Paradox by calculating effective sample
sizes and Keynesian evidential weights that penalize unrepresentative archives. [The Biggest Number in
the Room](03-the-biggest-number-in-the-room.md) turns to the cases where a single extreme value decides
everything.
