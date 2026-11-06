# Deciding Without Knowing

> Opening chapter of Part I, and the entry point for a reader with no mathematics beyond school
> arithmetic. Nothing here requires a formula. The formal version is
> [What Judging Is, and What It Is Not](27-what-judging-is.md).

## By the end of this chapter you can

1. Tell a question about *what is the case* apart from a question about *what to do*, and say why the second cannot be answered by waiting for the first.
2. Explain, using an ordinary example, why a decision has a cost on both sides — acting and not acting — and calculate the threshold $p^* = C / (\eta L)$ that sets the bar for acting.
3. Say what this book is about in one sentence, without using the word *uncertainty*.

## 1. Two kinds of question

Some questions ask what is true. *How tall is that building? How many students are in the room?*
These have answers, the answers can be checked, and if two people disagree at least one of them is
wrong.

Other questions ask what to do. *Should I take the umbrella? Should I answer this message now or
tomorrow? Should the school cancel the trip on Thursday?* These have answers too, but of a stranger
kind. Two people can look at the same sky, disagree about the umbrella, and both be right — because
they are not only asking about the sky. One of them has a long walk home and the other has a lift.

The second kind of question is the subject of this book. It is not a worse version of the first. It
is a different question, and it needs its own habits of thought.

## 2. The umbrella problem

You are leaving the house. The sky is grey. Do you take the umbrella?

Think about what the two mistakes cost. If you take it and it does not rain, you carried something
useless for a few hours: a cost of action $C$. If you leave it and it rains hard, you spend the afternoon
wet and cold: a loss of unbuffered inaction $L$. If your umbrella has protective efficacy $\eta \in (0, 1]$
(where $\eta = 1$ is complete protection and lower values reflect windblown rain), taking it reduces
your exposure to $(1 - \eta) L$.

Those two costs are rarely the same size. Suppose your subjective assessment of the chance of rain is $p$.
Comparing the expected burden of taking the umbrella against leaving it yields the simplest decision rule:

$$\mathbb{E}[\mathrm{Loss}(\text{Act})] = C + (1 - \eta) p L, \qquad \mathbb{E}[\mathrm{Loss}(\text{Wait})] = p L$$

You act whenever the expected loss of acting is lower than the expected loss of not acting:

$$C + (1 - \eta) p L < p L \iff C < \eta p L \iff p > p^* = \frac{C}{\eta L}$$

This is the whole idea, and it is worth saying exactly: **You act when the chance of the bad thing exceeds
the cost of acting divided by the loss you would suffer by not acting, scaled by efficacy.** In the
umbrella case, the cost of carrying the umbrella $C$ is tiny while the misery of being soaked $L$ is large.
If $C = 1$, $L = 20$, and $\eta = 1$, the threshold is $p^* = 1 / 20 = 0.05$. You rightly take the
umbrella even if there is only a 5% chance of rain. Someone with a car and an underground garage faces
the opposite arithmetic ($C$ is annoyance, $L \approx 0$), and rightly ignores the same grey sky.

Two things follow, and both are uncomfortable:

First, there is no single universally correct answer to the umbrella question. There is only a correct
answer *given* what each mistake costs the person deciding. When two people argue about the umbrella and
never state those costs, they are not having the same argument.

Second, the decision depends on a number nobody has: the true chance of rain $p$. You never find out
whether that number was right. You find out whether it rained. Those are different facts, and confusing
them is the most common mistake in this entire book.

## 3. Why you cannot wait

You might object: just wait until you know whether it will rain. But the bus leaves at eight. The
whole difficulty is that the moment for deciding arrives before the information does.

That is not an accident of the umbrella. It is the shape of almost every decision that matters. The
exam is marked before you know whether your study method worked. The match is played before you know
whether the training was right. The choice of what to study is made years before you find out what
the world wanted. In medicine, waiting for conclusive microbiological culture before administering
antibiotics to a septic patient turns a treatable infection into irreversible organ failure; in the
ancient Hippocratic tradition, this was named **καιρός (*kairós*)**—the fleeting, critical window of
action that does not wait for certainty.

There is an ancient name for this stance, and it comes from archery. The Stoic philosopher Antipater of
Tarsus observed that the archer's internal duty (**τέλος, *télos***)—standing correctly, drawing the string,
measuring the wind, aiming with care—is completed the moment the arrow leaves the bow. The external hit
(**σκοπός, *skopós***) belongs to the crosswinds that happen after release. What you did and what happened
to it are two different things, and both of them can be judged, but never with the same measure.

## 4. What this book is about

This book is about getting better at the second kind of question: deciding before you can know, and
being honest afterwards about what you decided and why.

That is a real skill, it can be learned, and it has a surprising amount of structure. Some of that
structure is arithmetic, and the later chapters will get to it. The first chapters do not need it.
They need only the willingness to notice, in ordinary situations, the three things that make these
decisions hard: you cannot see the future, the records you rely on are incomplete, and the people who
measure you can be affected by being measured.

## Bridge

The rest of Part I takes the ideas in this chapter apart and puts them back together with names.
[What Judging Is, and What It Is Not](27-what-judging-is.md) gives the historical vocabulary and the
three reasons the difficulty is permanent rather than temporary. The decision threshold itself — *act
when the chance exceeds the cost of acting divided by the loss of not acting* — is formalized in
[Decision, Loss, Threshold](36-decision-loss-threshold.md) and implemented in pure Rust as
`epistemic.rs::pauker_kassirer_threshold`, which computes the clinical decision boundary balancing
false-positive action costs against false-negative inaction ruin.
