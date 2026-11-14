# Probability as a Measure

<figure class="stoch-figure stoch-logo-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="800" height="132" viewBox="0 0 800 132" fill="none" role="img" aria-label="Stochology CH15 · FILTRATION — Probability As Axiomatic Measure">
  <defs>
    <!-- Canonical Ring Gradient -->
    <linearGradient id="ch15-ring" x1="2" y1="78" x2="386" y2="78" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="38%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="68%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <!-- 1. Atmospheric Cosmic Field -->
    <radialGradient id="ch15-glow" cx="52%" cy="52%" r="50%">
      <stop offset="0%" stop-color="#fbbf24" stop-opacity="0.24"/>
      <stop offset="48%" stop-color="#38bdf8" stop-opacity="0.14"/>
      <stop offset="80%" stop-color="#e11d48" stop-opacity="0.08"/>
      <stop offset="100%" stop-color="#090e1a" stop-opacity="0"/>
    </radialGradient>
    <!-- 3D Spherical Node Gradients -->
    <radialGradient id="ch15-node-well" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="var(--logo-well-hi, var(--elevated-bg, #ffffff))"/>
      <stop offset="100%" stop-color="var(--logo-node-bg, #e2e8f0)"/>
    </radialGradient>
    <linearGradient id="ch15-ring-epi" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#7dd3fc"/>
      <stop offset="50%" stop-color="#2563eb"/>
      <stop offset="100%" stop-color="#1e3a8a"/>
    </linearGradient>
    <radialGradient id="ch15-core-epi" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#bae6fd"/>
      <stop offset="45%" stop-color="#3b82f6"/>
      <stop offset="100%" stop-color="#1d4ed8"/>
    </radialGradient>
    <linearGradient id="ch15-ring-vital" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#6ee7b7"/>
      <stop offset="50%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#065f46"/>
    </linearGradient>
    <radialGradient id="ch15-core-vital" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#a7f3d0"/>
      <stop offset="45%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#047857"/>
    </radialGradient>
    <linearGradient id="ch15-ring-gov" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#fda4af"/>
      <stop offset="50%" stop-color="#e11d48"/>
      <stop offset="100%" stop-color="#881337"/>
    </linearGradient>
    <radialGradient id="ch15-core-gov" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#fecdd3"/>
      <stop offset="45%" stop-color="#f43f5e"/>
      <stop offset="100%" stop-color="#be123c"/>
    </radialGradient>
    <!-- Stókhos Reticle Gradients -->
    <radialGradient id="ch15-pupil" cx="45%" cy="42%" r="58%">
      <stop offset="0%" stop-color="#020617"/>
      <stop offset="68%" stop-color="#0f172a"/>
      <stop offset="100%" stop-color="#451a03"/>
    </radialGradient>
    <linearGradient id="ch15-gold-mira" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#fef08a"/>
      <stop offset="45%" stop-color="#fbbf24"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <radialGradient id="ch15-gold-core" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#ffffff"/>
      <stop offset="35%" stop-color="#fde047"/>
      <stop offset="100%" stop-color="#d97706"/>
    </radialGradient>
    <!-- Ink curve gradient -->
    <linearGradient id="ch15-ink-curve" x1="0%" y1="0%" x2="100%" y2="0%">
      <stop offset="0%" stop-color="var(--logo-ink, #0f172a)"/>
      <stop offset="50%" stop-color="var(--logo-sub, #1d4ed8)"/>
      <stop offset="100%" stop-color="var(--logo-ink, #0f172a)"/>
    </linearGradient>
    <radialGradient id="ch15-omega-fill" cx="50%" cy="50%" r="50%">
      <stop offset="0%" stop-color="#2563eb" stop-opacity="0.28"/>
      <stop offset="70%" stop-color="#1e293b" stop-opacity="0.15"/>
      <stop offset="100%" stop-color="#020617" stop-opacity="0.06"/>
    </radialGradient>
    <linearGradient id="ch15-filter-ring" x1="36" y1="38" x2="92" y2="94" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#fde047"/>
      <stop offset="50%" stop-color="#f59e0b"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
  </defs>
  <g>
    <circle cx="64" cy="66" r="48" fill="url(#ch15-glow)"/>
    <circle cx="64" cy="66" r="42" fill="url(#ch15-omega-fill)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.8"/>
    <circle cx="64" cy="66" r="28" stroke="url(#ch15-filter-ring)" stroke-width="2.4" stroke-dasharray="4 2.5"/>
    <line x1="64" y1="24" x2="64" y2="108" stroke="var(--logo-ticks, #64748b)" stroke-width="1.6" stroke-dasharray="3 3"/>
    <line x1="22" y1="66" x2="106" y2="66" stroke="var(--logo-ticks, #64748b)" stroke-width="1.6" stroke-dasharray="3 3"/>
    <circle cx="44" cy="46" r="8.5" fill="url(#ch15-node-well)" stroke="url(#ch15-ring-epi)" stroke-width="3.0"/>
    <circle cx="44" cy="46" r="3.8" fill="url(#ch15-core-epi)"/>
    <circle cx="84" cy="46" r="8.5" fill="url(#ch15-node-well)" stroke="url(#ch15-ring-vital)" stroke-width="3.0"/>
    <circle cx="84" cy="46" r="3.8" fill="url(#ch15-core-vital)"/>
    <circle cx="44" cy="86" r="8.5" fill="url(#ch15-node-well)" stroke="url(#ch15-ring-gov)" stroke-width="3.0"/>
    <circle cx="44" cy="86" r="3.8" fill="url(#ch15-core-gov)"/>
    <g>
      <circle cx="78" cy="80" r="7.4" fill="var(--logo-node-bg, #e2e8f0)"/>
      <circle cx="78" cy="80" r="6.4" fill="url(#ch15-pupil)" stroke="url(#ch15-gold-mira)" stroke-width="2.4"/>
      <line x1="68.40" y1="80.00" x2="73.80" y2="80.00" stroke="url(#ch15-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="82.20" y1="80.00" x2="87.60" y2="80.00" stroke="url(#ch15-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="78.00" y1="70.40" x2="78.00" y2="75.80" stroke="url(#ch15-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="78.00" y1="84.20" x2="78.00" y2="89.60" stroke="url(#ch15-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <circle cx="78" cy="80" r="2.2" fill="url(#ch15-gold-core)"/>
    </g>
  </g>
  <g transform="translate(148, 0)">
    <text x="2" y="27" fill="var(--logo-sub, #1d4ed8)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.2">PART II · THE BASES · MATHEMATICS</text>
    <text x="498" y="27" text-anchor="end" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.0">CH15 · FILTRATION</text>
    <text x="0" y="59" fill="var(--logo-ink, #0f172a)" font-family="system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif" font-size="20.2" font-weight="800" letter-spacing="1.15">PROBABILITY AS AXIOMATIC MEASURE</text>
    <line x1="2" y1="78" x2="384" y2="78" stroke="url(#ch15-ring)" stroke-width="1.6" stroke-opacity="0.82"/>
    <circle cx="400" cy="78" r="6.2" stroke="var(--logo-gold, #f59e0b)" stroke-width="1.5" stroke-dasharray="2.2 1.6"/>
    <circle cx="400" cy="78" r="4.0" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.2"/>
    <circle cx="400" cy="78" r="1.8" fill="var(--logo-gold, #f59e0b)"/>
    <circle cx="415" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.0"/>
    <circle cx="415" cy="78" r="1.6" fill="var(--logo-blue, #2563eb)"/>
    <circle cx="430" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.0"/>
    <circle cx="430" cy="78" r="1.6" fill="var(--logo-blue, #2563eb)"/>
    <circle cx="445" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="460" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="475" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="490" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <text x="2" y="104" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.8" font-weight="600" letter-spacing="0.65">SIGMA-ALGEBRAS · RADON–NIKODYM CONDITIONING · CROMWELL INTERIOR CLAMP</text>
  </g>
</svg>
</figure>

> Seventh and last chapter of the mathematical block in Part II. It closes the mathematical base by
> giving probability its proper home: a measure on a set of outcomes.

## By the end of this chapter you can

1. State the three axioms as conditions on a function, and say what each one is protecting.
2. Explain what a random variable is, in the technical sense, without appealing to randomness.
3. Say what independence buys and what conditioning does to the space you are asking about.

## 1. The Measurable Structure: $(\Omega, \mathcal{F}, \mathbb{P})$

After a regional power-grid brownout, an inquiry board asks for the probability that the cascade began with an
unrecorded manual override in a substation. The utility's telemetry database records half-hourly megawatt load and
ambient temperature, and has no column for manual switch throws. Before any probability number can be computed the
board faces a structural question: does the archive's event algebra even contain the subset of world states the
board is asking about? Measure theory separates three things. First the raw universe of trajectories ($\Omega$). Then the institutional ledger of
distinguishable events ($\mathcal{F}$). Then the probability mass assigned to those events ($\mathbb{P}$).

A rigorous probability space is a measure triple $(\Omega, \mathcal{F}, \mathbb{P})$ consisting of:

1. The **sample space** $\Omega$: the exhaustive set of all conceivable elementary outcomes $\omega$.
2. The **event space** $\sigma$-algebra $\mathcal{F} \subseteq 2^\Omega$: a family of subsets closed under
   complementation ($A \in \mathcal{F} \implies A^c \in \mathcal{F}$) and countable unions
   ($A\_i \in \mathcal{F} \implies \bigcup\_{i=1}^\infty A\_i \in \mathcal{F}$), with $\Omega \in \mathcal{F}$.
3. The **probability measure** $\mathbb{P} : \mathcal{F} \to [0, 1]$: a countably additive set function.

Restricting $\mathbb{P}$ to a $\sigma$-algebra is not a formality. On uncountably infinite spaces such as
$\mathbb{R}$, non-measurable sets, Vitali sets built with the axiom of choice, take no consistent length without
destroying translation invariance. The $\sigma$-algebra is the pre-registered inventory of admissible questions an
institution is mathematically permitted to ask.

In finite institutional archives the $\sigma$-algebra $\mathcal{F}$ has a combinatorial meaning. If a logging
schema partitions the sample space $\Omega$ into $m$ mutually exclusive, non-empty atomic cells
$\lbrace{}C\_1, \dots, C\_m\rbrace$, the generated event algebra $\mathcal{F} = \sigma(\lbrace{}C\_1, \dots, C\_m\rbrace)$ contains every union
of those cells and has cardinality $|\mathcal{F}| = 2^m$; for $m = 3$ observable categories, exactly $2^3 = 8$
events are measurable. Two boundary limits frame the structure: in the trivial algebra
$\mathcal{F}\_0 = \lbrace\emptyset, \Omega\rbrace$ ($m = 1$), no observation distinguishes any two outcomes; in the discrete
power-set algebra $\mathcal{F} = 2^\Omega$, every singleton $\lbrace\omega\rbrace$ is measurable. A hypothesis
$H \subsetneq C\_1$ that splits a single atomic cell without an observable tag lies outside $\mathcal{F}$. No point
probability $\mathbb{P}(H)$ exists, and the institution can report only the inner-outer measure bracket
$[0,\mkern3mu \mathbb{P}(C\_1)]$.

## 2. Kolmogorov Axioms and Cox's Plausibility

Andrey Kolmogorov (1933) established the three core axioms:

1. **Non-negativity**: $\mathbb{P}(A) \ge 0$ for all $A \in \mathcal{F}$.
2. **Unit normalization**: $\mathbb{P}(\Omega) = 1$.
3. **Countable additivity**: For any countable sequence of pairwise disjoint events $A\_1, A\_2, \dots \in \mathcal{F}$:

   $$\mathbb{P}\left(\bigcup\_{i=1}^\infty A\_i\right) = \sum\_{i=1}^\infty \mathbb{P}(A\_i).$$

The axioms are an operational contract against incoherence. By the classical Dutch book theorem (Ramsey 1926;
de Finetti 1937), an agent whose betting quotients violate non-negativity, unit normalization, or **finite
additivity** can be offered a finite combination of bets, each acceptable at the agent's own stated odds, whose
joint settlement yields a sure monetary loss; Kolmogorov's countable additivity strengthens finite additivity with
monotone continuity along decreasing sequences of events ($B\_m \downarrow \emptyset \implies \mathbb{P}(B\_m) \to 0$;
Kolmogorov 1933) so limits and expectations can pass under the integral sign.

Richard Cox (1946) showed that if plausibility assessments over a Boolean algebra of propositions satisfy basic
desiderata of consistency, transitivity, and universal comparability, an order-preserving rescaling exists under
which plausibilities obey Kolmogorov's rules. Probability is then the unique normative logic of uncertainty that
avoids internal contradiction.

A two-ticket settlement shows how the Dutch book theorem enforces Kolmogorov's additivity axiom
(Kolmogorov 1933; Cox 1946). An incoherent risk desk assesses a binary event $A \in \mathcal{F}$ and its complement
$A^c = \Omega \setminus A$ with betting quotients $q(A)$ and $q(A^c)$ satisfying $q(A) + q(A^c) = 1 + \delta > 1$. A
counterparty sells the desk two contracts at the desk's own stated fair values: Contract 1 pays its unit stake if
$\omega \in A$, Contract 2 pays its unit stake if $\omega \in A^c$. The desk pays $1 + \delta$ at time zero. Because
$A \cap A^c = \emptyset$ and $A \cup A^c = \Omega$, every elementary outcome $\omega \in \Omega$ triggers the payout of
exactly one contract ($\mathbf{1}\_A(\omega) + \mathbf{1}\_{A^c}(\omega) \equiv 1$), returning one unit and locking in
a sure net loss of $\delta$ before a single datum is observed.

Jacob Bernoulli's *Ars Conjectandi*, published posthumously in 1713, is the reason this chapter is called what it is. Its fourth part proves that as the number of trials grows the observed frequency converges on the underlying probability of the event, and Bernoulli read that theorem as a licence for the conjectural arts to claim a standard of their own instead of borrowing the geometer's. The theorem licenses measuring what can be repeated. It says nothing about the single case, and that silence is where the rest of this book works.

## 3. Random Variables and Measurability

A **random variable** is a measurable mapping $X : \Omega \to \mathbb{R}$ whose pre-image of every Borel interval
lies in the event space:

$$X^{-1}\bigl((-\infty, c]\bigr) = \lbrace\omega \in \Omega : X(\omega) \le c\rbrace \in \mathcal{F}, \qquad \forall c \in \mathbb{R}.$$

A random variable is not an uncertain number. It is a deterministic measurement function evaluated on an uncertain
world state. Measurability is what makes a statement such as "the vital reserve falls below zero" a valid event in
$\mathcal{F}$ that $\mathbb{P}$ can weight.

Notice the direction of the arrow in $X^{-1}\bigl((-\infty, c]\bigr)$. The outcome $\omega \in \Omega$ lives in the
world, the measurement function $X: \Omega \to \mathbb{R}$ maps $\omega$ onto a real scale, and the pre-image pulls
the threshold interval $(-\infty, c]$ back into $\mathcal{F}$ where $\mathbb{P}$ lives. In a hurricane archive, each
six-hourly storm transition is an elementary record $\omega$, the wind-intensity indicator is a deterministic
function on that record, and the hurricane threshold event is the pre-image whose empirical relative frequency
supplies the base rate. In a sequential reserve trajectory the ruin event
$\lbrace{}R\_t \le 0\rbrace = R\_t^{-1}\bigl((-\infty, 0]\bigr)$ is measurable with respect to the history filtration
$\mathcal{F}\_t$ precisely because $R\_t$ is a deterministic accounting functional of the observed shock path up to
step $t$.

## 4. Conditioning, Martingales, and Ville's Inequality

A safety monitoring committee inspects an incoming trial every Friday, intending to stop the moment a fixed-sample
test crosses its nominal level. Each peek gives noise another chance to ring the bell: under $n$ independent looks
at nominal level $\alpha$, the probability of at least one false rejection rises to $1 - (1 - \alpha)^n$, a multiple
of the nominal rate for a handful of looks and a certainty as the looks accumulate. Sequential probability needs a
process whose false-alarm guarantee holds across every stopping time at once.

Conditional probability updates the measure when an event $B$ with $\mathbb{P}(B) > 0$ is observed:

$$\mathbb{P}(A \mid B) = \frac{\mathbb{P}(A \cap B)}{\mathbb{P}(B)}.$$

Bayes' theorem follows as the algebraic inversion of conditioning:

$$\mathbb{P}(H\_k \mid E) = \frac{\mathbb{P}(E \mid H\_k)\mathbb{P}(H\_k)}{\sum\_j \mathbb{P}(E \mid H\_j)\mathbb{P}(H\_j)}.$$

Oliver Cromwell's rule (Lindley, 1985) warns that if a prior is assigned probability 0 or 1 ($\mathbb{P}(H) = 0$),
no amount of empirical evidence can ever update it ($\mathbb{P}(H \mid E) = 0$).

When data arrive sequentially over a filtration $(\mathcal{F}\_t)\_{t \ge 0}$, a stochastic process $(M\_t)\_{t \ge 0}$
is a **martingale** if $\mathbb{E}[|M\_t|] < \infty$ and:

$$\mathbb{E}[M\_{t+1} \mid \mathcal{F}\_t] = M\_t \quad \text{almost surely}.$$

Jean Ville (1939) proved the foundational maximal inequality for non-negative martingales:

$$\mathbb{P}\left(\sup\_{t \ge 0} M\_t \ge \frac{1}{\alpha}\right) \le \alpha, \qquad \forall \alpha \in (0, 1).$$

Ville's inequality is the bedrock of sequential analysis and anytime-valid testing. Classical fixed-sample tests
require pre-committing to a sample size $N$ to prevent $p$-hacking. Non-negative martingales (and e-processes) permit
continuous monitoring and optional stopping without inflating false-alarm rates.

In the sequential setting these three safeguards are one construction, not three ornaments:

- **Cromwell protection:** Before Bayes' rule updates a prior $\pi\_0$, the update is bounded inside the open interior $[\varepsilon, 1 - \varepsilon]$ (Lindley 1985), so no finite likelihood ratio is annihilated by a dogmatic zero.
- **Turing–Good weight of evidence and Ville's bar:** With sequential observations and standardized innovations $z\_s$, a nonnegative betting process multiplies the factors $\exp(\lambda z\_s - \frac{1}{2}\lambda^2)$ from initial unit wealth $E\_0 = 1$. It applies Imre Lakatos's ad-hoc adjustment penalty $E\_t / (K + 1)$ (Lakatos 1970), and reports the Turing–Good weight of evidence (Good 1950) in decibans as $W\_t = 10\log\_{10}(E\_t)$. The process is valid against the Ville barrier $1/\alpha$. Across a family of $K$ multiverse specifications the e-BH procedure controls the false discovery rate at the nominal level without a Bonferroni-style penalty on each test (Wang & Ramdas 2022).
- **Counterexample: when a fixed-sample test beats the anytime-valid radius:** Anytime validity is insurance against optional stopping, and insurance has a price in interval width. The anytime-valid radius $r\_T(\alpha)$ is a multiple of the fixed-sample normal radius $z\_{1-\alpha/2}\sigma/\sqrt{T}$, and the multiple grows with the number of looks the design permits. A design that commits to looking *exactly once* pays for peeking privileges it never exercises.

## 5. Strategic Classification and Game-Theoretic Interaction

In real-world institutional settings the probability space is not passive. An institution deploys a decision rule
$f\_\theta(x)$ to allocate resources, parole, credit, or inspections, and evaluated agents act strategically: they
solve an optimization problem to manipulate their observable features $x \to x'$ at cost $c(x, x')$:

$$x'(x; \theta) = \arg\max\_{z} \bigl(f\_\theta(z) - c(x, z)\bigr).$$

Static decision theory becomes a **Stackelberg game**. The institution is the leader and announces the rule
$\theta$. Agents are followers and game the metric. Assume the distribution $\mathbb{P}\_X$ invariant and the
deployed model degrades under Goodhart's law. Probability in institutional life is interactive and game-theoretic.

In measure-theoretic terms, strategic classification means there is no single passive probability measure
$\mathbb{P}\_X$ waiting to be estimated (Hardt, Megiddo, Papadimitriou & Wootters 2016; Perdomo et al. 2020; MacKenzie 2006).
Each institutional rule $\theta$ induces its own pushforward distribution
$\mathbb{P}\_{\theta} = \bigl(x'(\cdot; \theta)\bigr)\_{\unicode{x23}}\mathbb{P}\_X$ via the best-response map
$x \mapsto x'(x; \theta)$.

Auditing the performative feedback loop means comparing two fixed points. The myopic performative stability point
$\theta\_{\mathrm{PS}} = \frac{\mu\_0}{1 + \lambda - \varepsilon}$ minimizes loss under the current measure and ignores
how agents will adapt to the update. The Stackelberg performative optimum
$\theta\_{\mathrm{PO}} = \frac{(1 - \varepsilon)\mu\_0}{(1 - \varepsilon)^2 + \lambda}$ does not. The institutional
drift $|\theta\_{\mathrm{PO}} - \theta\_{\mathrm{PS}}|$ records the difference: in the passive limit
$\varepsilon \to 0$, $\theta\_{\mathrm{PS}} = \theta\_{\mathrm{PO}}$ and the gap is zero, and the gap is positive
exactly where the response to the rule is non-trivial.

### Evidence that survives repeated looking

Evidence accumulated without a fixed stopping rule needs its own arithmetic. A fixed-sample test would be tighter
on a single look and invalid the moment someone peeks: repeated looks turn a nominal error rate into a multiple of
itself. Validity and precision are different claims. One survives repeated inspection; the other survives exactly
one. Ville's inequality is the bound that makes the first kind possible.

### When the observation is impossible

The thesis fails when the observed datum has probability zero under the reference. The likelihood ratio is then
undefined, and no accumulation of evidence can be reported. Widen the support, say so, and report the widened model
instead of a ratio computed against a measure that assigns nothing to what was seen. This is the case in which a
model is refuted by a single observation.

The guarantee behind the deciban bar is Ville's inequality, from 1939 (Ville 1939), and it is what makes the
accumulated number valid at every stopping time and not only at a pre-declared one. The practical consequence is
uncomfortable: the interval width is bought once and cannot be narrowed by looking again, so a design that plans to
peek must budget the width up front, and a modest number of looks can spend a large share of the error budget before
any data is seen. The modern consolidation of that idea into a working toolkit for sequential experiments is recent
(Ramdas et al. 2023), and it changed practice mostly by making the budget explicit.

## Bridge

[Probability as Language](30-probability-as-language.md) and [Reactive Worlds, Performativity, and Goodhart's Law](48-reactive-worlds-performativity-goodhart.md)
extend measure-theoretic probability to reactive and strategic environments. The governance side verifies
game-theoretic institutional gates and audit trails. The epistemic side constructs anytime-valid e-processes that
guarantee false-alarm control under arbitrary optional stopping.

Part II turns from mathematics to philosophy in [What Knowledge Is](16-what-knowledge-is.md): the tripartite
analysis, the Gettier problem, and what an institution may legitimately claim to know.
