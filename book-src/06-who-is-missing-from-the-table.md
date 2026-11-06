# Who Is Missing From the Table

> School-level chapter of Part I. It needs no mathematics. The formal version is
> [Sample, Population, Archive](34-sample-population-archive.md).

## By the end of this chapter you can

1. Explain why any recorded dataset is an institutional selection rather than an unmediated copy of reality.
2. Demonstrate how non-random missingness ($\rho_{R,Y} \ne 0$) causes the Big Data Paradox, where more observations increase confidence in an erroneous answer.
3. Classify the three kinds of empty cells in an archive and trace how the mediation circuit's guard sites prevent institutional distortion.

## 1. The survey that only asks those present

A school runs a survey on whether students feel safe walking home after dark. The survey is conducted
in class, on a Tuesday morning, and almost everyone present responds.

Who is missing? The students who were not in class. Some were ill. Some were working. Some had
already dropped out. And it is not unreasonable to suspect that the students who stopped attending
are not a random sample of how the walk home feels.

The survey has a large number of responses and an unrecorded filter. The filter is invisible in the
summary table, because the table contains only the people who passed through the schoolroom door.

This is not an argument against collecting data. It is an argument for the habit of asking, every
single time: *who would not be in this table, and why?*

## 2. Recorded is not the same as happened: Wald's missing armor

Every institution sees a particular slice of the world, and that slice is defined by its own door.

A hospital records the patients who arrive at the hospital. It does not record the people who were ill
and stayed home, the people who died before the ambulance arrived, or the people whose suffering has
no diagnostic billing code in the administrative software. The database can contain millions of rows
and still be a record of admissions rather than of illness.

During World War II, the statistician Abraham Wald at the Statistical Research Group was asked where
to add armor to combat aircraft. Military engineers had carefully mapped the bullet holes on returning
bombers, noting heavy concentrations of damage on the wings and fuselage, and virtually none on the
engines or cockpit. The initial military impulse was obvious: reinforce the wings where the bullet
holes were concentrated.

Wald saw the fatal flaw immediately: the survey included only the aircraft that *returned to the
airfield*. Planes shot in the wings survived to be counted; planes shot in the engines crashed into the
ocean and never entered the military's spreadsheet. The holes on the returning bombers showed where a
plane could be shot and survive. The armor needed to go exactly where there were **no** bullet holes in
the record: on the engines and cockpit.

The same selection filter governs police arrest records, customer complaints, job application portals,
and credit scores. In each case, an institutional door filters what enters the archive. What the door
excludes is not missing at random: institutions exclude specific categories of events for structural
reasons, and those reasons are usually correlated with the very vulnerability being assessed.

## 3. More rows do not fix a filter: The Big Data Paradox

This is the part that surprises many planners. Surely, if we collect enough data, the sampling error
washes out?

It does not, and the reason is straightforward arithmetic. In 2018, the statistician Xiao-Li Meng
formally proved the **Big Data Paradox**: whenever the recording mechanism $R$ is correlated with the
phenomenon $Y$ (a correlation $\rho_{R,Y} \ne 0$, known as the data defect correlation), the effective
sample size $n_{\mathrm{eff}}$ collapses according to:

$$n_{\mathrm{eff}} \le \frac{n}{1 + (N - n)\rho_{R,Y}^2}$$

where $n$ is the sample size and $N$ is the total population.

If you survey $n = 2\,300\,000$ people out of a population of $N = 250\,000\,000$, but your collection
mechanism has even a minuscule selection bias of $\rho_{R,Y} = 0.005$, the effective sample size is
not two million. It is roughly **$n_{\mathrm{eff}} \approx 400$**! With a slightly larger defect of
$\rho_{R,Y} = 0.05$, a multi-million-row dataset carries the statistical information of fewer than
**ten** truly random draws.

Gathering more rows through a biased door does not bring you closer to the truth: it collapses the
standard error around the wrong number, producing catastrophic overconfidence. What converts raw data
into genuine evidence is not its byte count, but its **coverage** and the independence of its door.

## 4. Three ways to be empty

An empty cell in an archive is not one thing. It can be at least three:

1. **Unmeasured (Missingness):** The phenomenon occurred, but the instrument was not looking or the
   question was skipped. This is an ordinary missing value, which can often be bounded or imputed.
2. **Measured and Null (Negative Evidence):** The instrument observed the state and recorded zero. A
   patient with no fever is not a patient with a missing temperature; the zero is an informative signal.
3. **Unrepresentable (Ontological Exclusion):** The category does not exist in the institutional schema.
   There is no column, no dropdown option, and no database field. This is the most destructive form of
   absence, because a phenomenon with no column looks indistinguishable from a phenomenon that never
   occurred.

## 5. The mediation circuit and its guard sites

To prevent this distortion, Stochology models data not as a static table, but as a path traversing a
**mediation circuit across three planes**: the **World** (where events happen), the **Archive**
(where events are recorded), and the **Institution** (where decisions are deliberated and executed).

Along this loop, seven distinct **guard sites** ($T_1$ through $T_7$) protect against corruption:
framing the inquiry ($T_1$), auditing data collection ($T_2$), verifying archive representation ($T_3$),
guarding formalization ($T_4$), preserving temporal order ($T_5$), monitoring performative feedback
($T_6$), and enforcing structural liability ($T_7$). Without these guard sites, institutional records
become funhouse mirrors that reflect only their own administrative convenience.

## Bridge

[Sample, Population, Archive](34-sample-population-archive.md) formalizes coverage bias and the
mathematics of selection, while [The Grammar of Aim](notes/The%20Grammar%20of%20Aim.md) sets out the
syntax of the mediation circuit. In pure Rust, `grammar.rs::CircuitTransit` defines the directed
transits across World, Archive, and Institution that hold the loop together, while
`epistemic.rs::from_meng_big_data` computes the effective sample size collapse under non-random selection
defects. [When the Measure Becomes the Goal](07-when-the-measure-becomes-the-goal.md) examines what
happens when people begin adapting their behavior to the records that measure them.
