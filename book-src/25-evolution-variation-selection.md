# Evolution, Variation, and Selection

> Fifth chapter of the natural-science block in Part II. The subject is how a population changes when
> something sorts it, and what happens when the sorting is done by a proxy.

## By the end of this chapter you can

1. State the three conditions that produce evolution, and say which one an institution can easily
   supply by accident.
2. Distinguish selection from drift, and say why direction is not the same as improvement.
3. Explain what artificial selection demonstrates about optimising a measurable substitute.

## 1. The Three Algorithmic Conditions of Selection

Evolution by natural selection (Darwin, 1859) requires neither foresight, consciousness, nor teleological
progress. It is an algorithmic sorting filter operating across populations whenever three conditions are met:

1. **Phenotypic variation**: Individuals within a population exhibit differences in traits, behaviors, or morphology.
2. **Differential reproductive fitness**: Variants differ in their survival and reproduction rates in the prevailing
   environment.
3. **Heritability**: A mechanism exists whereby variations are transmitted across generations (genetic inheritance,
   cultural transmission, or organizational standard operating procedures).

Wherever these three conditions coincide, the statistical composition of the population changes over time.
The filter operates mechanically. In institutional settings, whenever promotions, funding allocations, or
regulatory inspections reward specific behaviors and penalize others, management creates an artificial selection
pressure that evolves the culture of the workforce — often in directions completely contrary to stated institutional goals.

## 2. Fitness Landscapes, Local Optima, and Trade-offs

Sewall Wright (1932) formalized evolutionary dynamics through **fitness landscapes**: a multi-dimensional
topographical map relating genotype or phenotype combinations to reproductive fitness. Natural selection is a
local gradient-ascent algorithm: populations climb the nearest local fitness peak.

Two structural constraints emerge from landscape geometry:

- **Entrapment in local optima**: A population cannot cross a deep adaptive valley of lower fitness to reach a
  higher, distant peak. A system can become exquisitely adapted to a mediocre institutional arrangement,
  trapped because every incremental deviation is locally penalized.
- **Adaptive trade-offs and pleiotropy**: Biological traits are rarely independent; genes exhibit pleiotropy
  (one gene influences multiple traits). Optimizing a single trait (such as lactation yield in cattle or sprint speed
  in cheetahs) imposes severe metabolic or structural costs on bone density, disease resistance, and longevity.
  There is no free optimization: every gain is paid for by sacrificing margins elsewhere.

## 3. Motoo Kimura and Neutral Genetic Drift

Not all directional change represents adaptation. Motoo Kimura (1968) introduced the **neutral theory of
molecular evolution**: at the molecular and genetic level, the vast majority of evolutionary changes and
polymorphisms are selectively neutral, driven not by Darwinian selection, but by stochastic **genetic drift**
in finite populations.

In a finite population of effective size $N_e$, sampling variance in reproductive transmission causes allele
frequencies to wander at random. A neutral or slightly deleterious mutation can become fixed (reach 100% frequency)
purely by chance, with fixation probability equal to its initial frequency $1/(2N_e)$.

In small or insular institutions, bureaucratic drift mirrors genetic drift: arbitrary procedural quirks, jargon,
and habits become permanently fixed not because they improve operational performance, but through random turnover
and the retirement of key personnel. Confusing drift with adaptive wisdom leads institutions to venerate accidental
habits as though they were proven best practices.

## 4. W. Ross Ashby's Law of Requisite Variety

The cybernetic limits of adaptive regulation were formulated by W. Ross Ashby (1956) in *An Introduction to Cybernetics*.
Ashby established the **Law of Requisite Variety**:

> *"Only variety can absorb variety."*

Let $D$ be the set of external disturbances impinging on a system, $R$ be the set of regulatory responses available
to the controller, and $O$ be the resulting outcomes. Measuring variety as the logarithm of the number of distinct
states ($\mathcal{V} = \log_2 N$), Ashby's law states:

$$\mathcal{V}_O \ge \mathcal{V}_D - \mathcal{V}_R.$$

To constrain the variety of outcomes $\mathcal{V}_O$ to a small, safe set of survivable states, the variety of the
regulator $\mathcal{V}_R$ must equal or exceed the variety of disturbances $\mathcal{V}_D$:

$$\mathcal{V}_R \ge \mathcal{V}_D - \mathcal{V}_O^{\mathrm{tolerable}}.$$

If the external environment is complex and volatile, an institution cannot regulate it with a rigid, low-variety
rulebook. When management aggressively standardizes operations — eliminating frontline discretion, stripping
spare capacity, and enforcing uniform checklists — it crushes internal regulator variety $\mathcal{V}_R$.
The predictable consequence is that when novel, high-variety environmental disturbances occur, the system lacks
the requisite behavioral bandwidth to respond, suffering catastrophic regulatory failure.

## 5. Artificial Selection as the Biological Root of Goodhart's Law

For millennia, agricultural breeders have practiced artificial selection, selecting organisms on measurable scalar
traits (milk output, seed size, rapid growth). The universal result is that intense single-trait selection breeds
for the trait while degrading the unselected organism: poultry bred for breast muscle develop skeletal collapse
and cardiac failure; wheat bred for harvest yield loses drought tolerance and fungal resistance.

Artificial selection is the direct biological ancestor of **Goodhart's Law** and **Campbell's Law**:

> *"When a measure becomes a target, it ceases to be a good measure."*

When an institution evaluates and rewards staff based on a single proxy metric (test scores, crime closure rates,
patient turnaround times), it acts as an aggressive artificial breeder. The organization breeds for the metric,
while the complex, multi-dimensional qualities the metric was meant to reflect — genuine education, community safety,
healing — are ruthlessly bred out of the system.

## Bridge

[Institution, Incentive, Metric](40-institution-incentive-metric.md) and [Reactive Worlds, Performativity, and Goodhart's Law](48-reactive-worlds-performativity-goodhart.md)
formalize proxy gaming and metric degradation. In the Stochology library, `governance.rs::GovernanceState`
implements Ashby's requisite variety constraints, auditing whether an institution maintains sufficient multi-channel
governance variety and algedonic escape paths to absorb dynamic environmental shocks without succumbing to metric capture.

[Regulation and Homeostasis](26-regulation-and-homeostasis.md) closes the scientific block by joining
feedback, allostasis, and queueing into a single account of what it costs to run near the ceiling.
