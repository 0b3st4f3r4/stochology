# It Worked, So I Was Right

> School-level chapter of Part I. The formal version of the distinction is in
> [Aim and the Three Planes](29-aim-and-the-three-planes.md), and its ethical consequences in
> [Moral Luck, Apt Agency, and Institutional Design](49-moral-luck-apt-agency-institutions.md).

## By the end of this chapter you can

1. Separate the quality of an ex-ante decision from the quality of its ex-post outcome using the fourfold competence-luck matrix.
2. Explain why a room full of winners can contain almost no skill, and how survivorship bias fabricates false methods.
3. Apply the archer's distinction between aim ($télos$) and hit ($skopós$) to design institutional governance that rewards apt agency rather than pure luck.

## 1. Two things that are not the same

A decision is made at a specific point in time with what is known then. An outcome happens afterwards,
in irreversible time, with the cooperation of everything else in the universe.

Because the decision precedes the outcome, four combinations are possible in any uncertain world:

| Ex-Ante Decision Quality | Ex-Post Desirable Outcome | Ex-Post Adverse Outcome |
| :--- | :--- | :--- |
| **Competent / Well-Calibrated** | **Apt Competence:** Careful inquiry rewarded by favorable conditions. | **Noble Misfortune:** Prudent precaution tested by an unpreventable extreme shock. |
| **Reckless / Incompetent** | **Toxic Luck:** Wild gamble rewarded by random chance (the most corrupting case). | **Deserved Catastrophe:** Neglect followed by predictable collapse. |

Notice which cases look alike to an inattentive outside observer:

The two successes (Apt Competence and Toxic Luck) look identical on a short-term spreadsheet: both hit
the revenue target, passed the audit, or won the match. The two failures (Noble Misfortune and Deserved
Catastrophe) both show red ink or broken pipes.

Evaluating decisions purely by their ex-post outcomes is known as **outcome bias** or the problem of
**moral luck**. When an institution cannot distinguish competence from fortune, it systematically
promotes the reckless gamblers who happened to get lucky, while punishing the prudent stewards who
prepared for real risks.

## 2. A room full of winners

Here is a demonstration that needs no mathematics.

A thousand people each flip a fair coin and are paid for heads. After one flip, about five hundred
remain. They flip again; about two hundred and fifty remain. Continue for ten rounds, and roughly one
person is left with ten heads in a row. Bring that person to a stage and ask for the secret.

The room will produce one of two reactions. Either the audience believes there is a secret — the
person must have a special flick of the wrist, a positive mindset, a proprietary crypto-algorithm —
or the audience notices that a thousand people is enough to produce such a person no matter what anyone
does.

The second reaction is the correct one, and it is remarkably hard to hold on to. The person on stage
*is* exceptional: one in a thousand. The mistake is thinking that being one in a thousand implies
having done something that others could copy. Random selection over a large population produces
exceptional records at a mathematically predictable rate, and it produces zero transferable method.

This is where **survivorship bias** enters institutional life. You see the winners because they are
on stage, in the executive suite, or on the cover of the business magazine. The nine hundred and
ninety-nine who used the exact same method and failed are invisible. If the method was indistinguishable
from coin-flipping, the stage is celebrating pure variance masquerading as expertise.

## 3. The shot, the hit, and apt agency

The vocabulary for untangling this comes from archery, and it has an ancient lineage. The Stoics
contrasted the internal duty of the archer (**τέλος, *télos***)—everything within the agent's control,
such as testing the bowstring, measuring the target, and accounting for the crosswind—with the external
strike (**σκοπός, *skopós***), which remains subject to crosswinds after the arrow has left the bow.

In modern epistemology, Ernest Sosa formalized this as the **AAA criteria** of performance:

1. **Accurate:** Did the arrow hit the target? (Did the outcome succeed?)
2. **Adroit:** Was the shot executed with genuine skill, care, and calibration? (Was the decision competent?)
3. **Apt:** Was the shot accurate *because* it was adroit? (Did the success stem from the agent's competence,
   rather than an errant gust of wind that blew a badly aimed arrow back into the bullseye?)

When an environment is **reactive** and dynamic—where other participants, competitors, and institutions
react to your actions over time—evaluating only accuracy destroys the system. An agent who knows the
wind is unreadable and lowers the bow has exercised an adroit competence. An assessment system that
rewards only hits will train that prudence out of them, because declining to take a reckless shot never
appears as a victory on a quarterly scoreboard.

## 4. What this does to schools, teams, and institutions

If an institution measures only outcomes, here is what it teaches, without anyone intending it:

It teaches that luck is skill. It teaches that a person who took an irresponsible risk and got away
with it should be promoted. It teaches that a person who maintained a prudent vital margin and suffered
an unpreventable heavy-tailed shock should be penalized. And it teaches everyone watching that the
safest strategy is not to be competent, but to be seen with winning numbers — to take concealed tail
risks whose profits are pocketed today while leaving the eventual ruin to someone else.

The remedy is not to ignore outcomes. It is to record the decision premises *before* the outcome is
revealed: what was known, what risks were identified, what buffers were reserved, and what was expected.
By sealing the decision premise in a temporal vault before the world reveals its roll of the dice, the
institution can evaluate whether the result was an apt hit, an adroit miss, or a toxic piece of luck.

## Bridge

[Aim and the Three Planes](29-aim-and-the-three-planes.md) develops the formal architecture connecting
internal aim (*télos*) to external result (*skopós*), while
[Moral Luck, Apt Agency, and Institutional Design](49-moral-luck-apt-agency-institutions.md) examines
how institutions govern this gap. In pure Rust, `governance.rs::GovernanceState` implements this
institutional safeguard across its three gates (ex-ante calibration, real-time algedonic warning, and
ex-post audit), while `governance.rs::TelicGrade` (and its classifier `governance.rs::TelicGrade::classify`)
categorizes agency into apt achievement, adroit misfortune, and toxic luck. [What a Number Doesn't Say](05-what-a-number-doesnt-say.md)
turns to how numbers themselves disguise their own unreliability.
