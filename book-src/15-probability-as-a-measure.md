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

> Giving probability its proper home: a measure on a set of outcomes, and why reactive institutions
> break passive measures.

## By the end of this chapter you can

1. State the three axioms, and separate finite betting coherence from countable additivity and imprecise credences.
2. Define random variables as measurable maps, and state what independence buys and where it breaks.
3. Trace how conditioning, filtrations under Ville's inequality, and strategic response reshape measures.

## 1. The Measurable Structure: $(\Omega, \mathcal{F}, \mathbb{P})$

After a regional brownout, an inquiry asks whether an unlogged manual switch threw the grid into cascade. Utility telemetry records megawatt load and temperature, yet includes no field for manual overrides. Building on the sets and $\sigma$-algebras of [Chapter 09](09-sets-relations-functions.md) and the integrals of [Chapter 13](13-integrals-and-accumulation.md), probability theory separates the universe of outcomes ($\Omega$), the closed inventory of answerable questions ($\mathcal{F} \subseteq 2^\Omega$, containing $\Omega$ and closed under complements and countable unions), and the probability measure ($\mathbb{P} : \mathcal{F} \to [0, 1]$) forming a **probability space** $(\Omega, \mathcal{F}, \mathbb{P})$.

On a three-state transformer $\Omega = \lbrace \text{normal}, \text{overload}, \text{fault} \rbrace$, full resolution $2^\Omega$ holds $2^3 = 8$ events. Dividing $\Omega$ into $m$ non-overlapping atomic cells $\lbrace C\_1, \dots, C\_m \rbrace$ generates a finite algebra $\mathcal{F} = \sigma(\lbrace C\_1, \dots, C\_m \rbrace)$ of size $|\mathcal{F}| = 2^m$, such as a coarse fault-only sensor $\mathcal{F} = \lbrace \emptyset, \lbrace \text{fault} \rbrace, \lbrace \text{normal}, \text{overload} \rbrace, \Omega \rbrace$ ($m = 2$, $|\mathcal{F}| = 4$) between $\lbrace \emptyset, \Omega \rbrace$ and $2^\Omega$. On uncountable spaces like $\mathbb{R}$, restricting $\mathbb{P}$ to $\mathcal{F}$ avoids Vitali non-measurability ([Chapter 09](09-sets-relations-functions.md) §1).

Any unlogged sub-event $\emptyset \neq H \subsetneq C\_1$ splitting a coarse bucket sits outside $\mathcal{F}$, leaving only the inner–outer bracket $[0,\mkern3mu \mathbb{P}(C\_1)]$. Cryptographic commit seals freeze $\mathcal{F}\_t$ at logging time ([Chapter 48](48-reactive-worlds-performativity-goodhart.md) §7.7.2). When shifting realities (*infidélité du milieu*; Canguilhem 1943/1966) expose an unlogged hazard $H$, analysts must quarantine the mismatch and declare an audited forward expansion $\mathcal{F}\_t \subsetneq \mathcal{F}'$ without altering sealed history—a governance rule formalized in [Chapter 29](29-aim-and-the-three-planes.md) and [Chapter 41](41-the-triad-and-the-minimum.md).

## 2. Kolmogorov Axioms and Cox's Plausibility

In plain words, Andrey Kolmogorov's (1933) three axioms turn probability into a non-negative unit budget split cleanly across non-overlapping events:

1. **Non-negativity:** No event can carry negative chance, so $\mathbb{P}(A) \ge 0$ for every $A \in \mathcal{F}$.
2. **Unit normalization:** Something inside the sample space must occur, so $\mathbb{P}(\Omega) = 1$.
3. **Countable additivity:** For any countable list of pairwise disjoint events $A\_1, A\_2, \dots \in \mathcal{F}$, the chance that at least one occurs equals the sum of their individual chances:

   $$\mathbb{P}\left(\bigcup\_{i=1}^\infty A\_i\right) = \sum\_{i=1}^\infty \mathbb{P}(A\_i).$$

Check those rules on our three-state transformer: assigning $0.7$ to normal, $0.2$ to overload, and $0.1$ to fault satisfies non-negativity, sums to $0.7 + 0.2 + 0.1 = 1$, and assigns $\mathbb{P}(\lbrace \text{overload}, \text{fault} \rbrace) = 0.2 + 0.1 = 0.3$.

Two classic bridges connect these axioms to real-world counting. First, Jakob Bernoulli's *Ars Conjectandi* (Bernoulli 1713) introduced the term *stochastice* ("the conjectural art", from *stókhos*, aim—*ars conjectandi sive stochastice*) and proved the **Law of Large Numbers**. Across repeated independent trials with constant chance $p$, the observed share of successes converges toward $p$ as trials accumulate. Toss a fair coin $10$ times and $7$ heads is unremarkable; toss it thousands of times and the share settles near $0.5$. Still, Bernoulli's law speaks only to repeatable ensembles and stays silent on unrepeatable single events ([Chapter 30](30-probability-as-language.md) §5). That empirical frequency discipline also rests on older planetary roots. In ninth-century Baghdad, al-Kindī grounded cryptanalysis in stable empirical letter frequencies across long texts (Mrayati, Meer Alam & at-Tayyan 2003). Classical Jaina *syādvāda* likewise indexed modal truth claims to explicit standpoints without confusing qualified judgment with ignorance (Matilal 1981).

Deeper foundations reveal two real tensions beneath Kolmogorov's three axioms. First, subjective coherence via Dutch books (Ramsey 1926; de Finetti 1937) derives non-negativity, normalization, and **finite additivity** from fair betting prices. Test a two-ticket Dutch book by hand. Suppose a desk posts selling prices $q(A) + q(A^c) = 1 - \delta < 1$ ($\delta > 0$) on event $A$ and its complement $A^c$. A trader simply buys Contract 1 (paying $1$ if $\omega \in A$) and Contract 2 (paying $1$ if $\omega \in A^c$) for $1 - \delta$ combined; since $\mathbf{1}\_A + \mathbf{1}\_{A^c} \equiv 1$, the desk collects $1 - \delta$, pays $1$ either way, and suffers a guaranteed loss $\delta$ (symmetrically, at posted bid prices $q(A) + q(A^c) = 1 + \delta > 1$, the trader sells both contracts to the desk for $1 + \delta$ and settles for $1$). Strengthening finite additivity into countable additivity requires Kolmogorov's (1933) Continuity Axiom ($B\_m \downarrow \emptyset \implies \mathbb{P}(B\_m) \to 0$) so infinite limits behave cleanly ([Chapter 09](09-sets-relations-functions.md); [Chapter 30](30-probability-as-language.md)).

Second, Richard Cox's (1946) real-valued plausibility rules diverge from both imprecise and game-theoretic models. Joseph Halpern (1999) showed that Cox's smoothness and comparability axioms break on finite Boolean schemas without added density, and still yield only finite additivity ([Chapter 09](09-sets-relations-functions.md); [Chapter 30](30-probability-as-language.md) §1). Under coarse unmeasurable buckets $[0, \mathbb{P}(C\_1)]$ (§1), **imprecise credal intervals** $[\underline{P}(A), \overline{P}(A)]$ avoid guaranteed loss without forcing sharp priors or total rankings (Keynes 1921; Ellsberg 1961; Walley 1991). Unlike the scalar clamping in §4, credal intervals leave ambiguity open, and simple rules can outperform fine-tuned priors under change (Brighton & Gigerenzer 2012). Meanwhile, **game-theoretic testing by betting** (Shafer 2021; Ramdas, Grünwald, Vovk & Shafer 2023) measures evidence through non-negative capital growth without locking down one static measure across an expanding horizon ($\mathcal{F}\_t \subsetneq \mathcal{F}'$).

## 3. Random Variables and Measurability

A **random variable** $X$ is not a vague, blurry number—it is a clear deterministic measurement function $X : \Omega \to \mathbb{R}$ assigning a real value to each world state $\omega \in \Omega$, such that every threshold question lands inside our event inventory $\mathcal{F}$:

$$X^{-1}\bigl((-\infty, c]\bigr) = \lbrace \omega \in \Omega : X(\omega) \le c \rbrace \in \mathcal{F}, \qquad \forall c \in \mathbb{R}.$$

Check a tiny concrete example on two coin flips, $\Omega = \lbrace HH, HT, TH, TT \rbrace$, where each outcome has probability $1/4$. Let $X(\omega)$ count the heads: $X(HH) = 2$, $X(HT) = X(TH) = 1$, and $X(TT) = 0$. Asking whether $X \le 1$ pulls back to the set $X^{-1}((-\infty, 1]) = \lbrace HT, TH, TT \rbrace \in \mathcal{F}$, confirming that $X$ is measurable.

Read the mapping backward on real systems: world state $\omega \in \Omega$ happens first, measurement $X : \Omega \to \mathbb{R}$ assigns a numerical reading, and $X^{-1}((-\infty, c])$ checks whether that threshold belongs to the archive's algebra. In a storm file, wind speed maps each synoptic fix $\omega$ onto knots; in a reserve trajectory, ruin $\lbrace R\_t \le 0 \rbrace = R\_t^{-1}((-\infty, 0])$ is $\mathcal{F}\_t$-measurable because reserve $R\_t$ depends measurably on shocks up to step $t$.

Two numerical summaries from [Chapter 13](13-integrals-and-accumulation.md) characterize any integrable random variable $X$ with finite mean $\mu = \mathbb{E}[X]$: **expectation** $\mathbb{E}[X]$ locates its probability-weighted center, and **variance** $\mathrm{Var}(X) = \mathbb{E}[(X - \mu)^2]$ measures typical squared spread around that center. Check our two-coin count $X$: $\mathbb{E}[X] = 0(1/4) + 1(2/4) + 2(1/4) = 1$, and $\mathrm{Var}(X) = (0 - 1)^2(1/4) + (1 - 1)^2(2/4) + (2 - 1)^2(1/4) = 0.5$.

### Independence and when factorization fails

Two events $A, B \in \mathcal{F}$ are **independent** when knowing $B$ happened leaves the chance of $A$ unchanged, so joint probability factors: $\mathbb{P}(A \cap B) = \mathbb{P}(A)\mathbb{P}(B)$. Toss two fair coins: $\mathbb{P}(H\_1 \cap H\_2) = \frac{1}{2} \cdot \frac{1}{2} = \frac{1}{4}$. Draw two cards from a deck without replacement, however, and seeing an ace first lowers the chance of drawing an ace second—independence fails.

Events $A\_1, \dots, A\_n$ are mutually independent when every finite subgroup factors similarly, and random variables are independent when all their threshold pre-images factor (Kolmogorov 1933). Given a conditioning event $C \in \mathcal{F}$ ($\mathbb{P}(C) > 0$), conditional independence requires $\mathbb{P}(A \cap B \mid C) = \mathbb{P}(A \mid C)\mathbb{P}(B \mid C)$. Independence buys product measures, additive variances ($\mathrm{Var}(X + Y) = \mathrm{Var}(X) + \mathrm{Var}(Y)$), Bernoulli's law (§2), and predictable error bars; it fails under clustered rows ([Chapter 14](14-linear-algebra.md) §3), continuous peeking (§4), and strategic gaming (§5).

## 4. Conditioning, Martingales, and Ville's Inequality

Conditioning shrinks the sample space to what you just observed and rescales probabilities so the new universe sums to $1$. Roll a fair six-sided die ($\Omega = \lbrace 1, 2, 3, 4, 5, 6 \rbrace$). Before looking, $A = \lbrace 2 \rbrace$ has chance $1/6$. Suppose a helper tells you the roll landed even: $B = \lbrace 2, 4, 6 \rbrace$, which carries $\mathbb{P}(B) = 3/6 = 1/2$. Inside that restricted universe $B$, only $\lbrace 2 \rbrace$ satisfies both $A$ and $B$, so its updated chance doubles to $(1/6) / (1/2) = 1/3$. Formally, for any $B \in \mathcal{F}$ with $\mathbb{P}(B) > 0$:

$$\mathbb{P}(A \mid B) = \frac{\mathbb{P}(A \cap B)}{\mathbb{P}(B)}.$$

Read that formula aloud: divide the probability of both $A$ and $B$ happening together by the total probability of the observed world $B$.

**Bayes' theorem** turns that formula around to update hypothesis probabilities when new evidence arrives. Walk through a simple step-by-step toy test before looking at symbols. Suppose $10\%$ of components in a bin are faulty ($\mathbb{P}(H\_1) = 0.1$) and $90\%$ are sound ($\mathbb{P}(H\_0) = 0.9$). A test flag $E$ catches $90\%$ of faulty parts ($\mathbb{P}(E \mid H\_1) = 0.9$) while falsely flagging $20\%$ of sound parts ($\mathbb{P}(E \mid H\_0) = 0.2$):

1. **Faulty and flagged:** $0.1 \times 0.9 = 0.09$.
2. **Sound and flagged:** $0.9 \times 0.2 = 0.18$.
3. **Total probability of a flag:** $0.09 + 0.18 = 0.27$.
4. **Posterior chance a flagged part is faulty:** $0.09 / 0.27 = 1/3 \approx 0.333$.

Even a fairly sharp screen leaves two-thirds of positive flags as false alarms because faulty parts were rare to begin with. In general, across any disjoint measurable hypotheses $\lbrace H\_j \rbrace$ partitioning $\Omega$ with $\sum\_j \mathbb{P}(E \mid H\_j)\mathbb{P}(H\_j) > 0$:

$$\mathbb{P}(H\_k \mid E) = \frac{\mathbb{P}(E \mid H\_k)\mathbb{P}(H\_k)}{\sum\_j \mathbb{P}(E \mid H\_j)\mathbb{P}(H\_j)}.$$

Read Bayes' rule aloud: multiply prior belief $\mathbb{P}(H\_k)$ by the likelihood $\mathbb{P}(E \mid H\_k)$ of seeing evidence $E$ under $H\_k$, then divide by the total probability of seeing $E$ across all rival hypotheses. Notice Oliver Cromwell's rule (Lindley 1985) directly in our toy numbers. If you set prior $\mathbb{P}(H\_1) = 0$ (or $1$), the numerator multiplies by $0$, locking your posterior at $0$ (or $1$) permanently no matter how loud the test alarm sounds ($\mathbb{P}(H \mid E) = \mathbb{P}(H)$ whenever $\mathbb{P}(E) > 0$).

Sequential monitoring adds a subtler trap: peeking until an alarm rings. Picture a trial board checking incoming data every Friday and planning to stop as soon as a standard fixed-sample test crosses false-alarm level $\alpha \in (0, 1)$. Across $n \ge 1$ independent looks, false-alarm chance already swells to $1 - (1 - \alpha)^n < 1$ (approaching $1$ as $n \to \infty$); on cumulative data, positive correlation between looks still inflates false rejections well above $\alpha$ (Howard et al. 2021). Safe sequential monitoring demands guarantees that hold across *every* possible stopping rule.

To build those guarantees over discrete steps $t \in \mathbb{N}\_0 = \lbrace 0, 1, 2, \dots \rbrace$, model growing information as a **filtration** $(\mathcal{F}\_t)\_{t \in \mathbb{N}\_0}$—an increasing chain of question inventories $\mathcal{F}\_0 \subseteq \mathcal{F}\_1 \subseteq \cdots \subseteq \mathcal{F}$. An integrable process $(M\_t)\_{t \in \mathbb{N}\_0}$ ($M\_t$ measurable with respect to $\mathcal{F}\_t$ and $\mathbb{E}[|M\_t|] < \infty$) is a **martingale** (a fair betting fortune) if $\mathbb{E}[M\_{t+1} \mid \mathcal{F}\_t] = M\_t$ a.s., and a **supermartingale** (a fair or unfavorable game) if $\mathbb{E}[M\_{t+1} \mid \mathcal{F}\_t] \le M\_t$ a.s. For any non-negative supermartingale starting at $\mathbb{E}[M\_0] \le 1$ (usually $M\_0 = 1$), Jean Ville (1939) proved:

$$\mathbb{P}\left(\sup\_{t \in \mathbb{N}\_0} M\_t \ge \frac{1}{\alpha}\right) \le \alpha, \qquad \forall \alpha \in (0, 1).$$

Read Ville's inequality aloud: starting with $1$ unit in a non-negative fair-or-worse game, the chance that your wealth *ever* reaches or exceeds $1/\alpha$ units at any stopping moment is at most $\alpha$. For $\alpha = 0.05$, reaching $20$ units happens at most $5\%$ of the time; general starts scale by $\alpha\mkern2mu\mathbb{E}[M\_0]$, so starting at $2/\alpha$ breaks the bound trivially. For composite null families $\mathcal{P}\_0$, **e-processes**—adapted non-negative processes satisfying $\mathbb{E}\_P[E\_\tau] \le 1$ at every stopping time $\tau$ and every $P \in \mathcal{P}\_0$ (Ramdas, Grünwald, Vovk & Shafer 2023)—preserve that exact $1/\alpha$ threshold regardless of when an analyst peeks.

Three practical safeguards turn these sequential tools into working discipline:

- **Cromwell protection:** Keep prior weights $\pi\_0$ away from $0$ and $1$ inside $[\varepsilon\_{\mathrm{floor}}, 1 - \varepsilon\_{\mathrm{floor}}] \subset (0, 1)$ ($0 < \varepsilon\_{\mathrm{floor}} < 1/2$ for two hypotheses, and $0 < \varepsilon\_{\mathrm{floor}} < 1/m$ with atomic cap $1 - (m - 1)\varepsilon\_{\mathrm{floor}} \le 1 - \varepsilon\_{\mathrm{floor}}$ across $m \ge 2$ disjoint atoms in §1; Lindley 1985) so finite likelihood ratios stay responsive.
- **Turing–Good evidence and `e-BH`:** Starting from $E\_0 = 1$, pick a predictable bet size $\lambda\_{\mathrm{bet}} \in \mathbb{R}$ and conditionally $1$-sub-Gaussian null steps $z\_s$ ($\mathbb{E}\_{H\_0}[\exp(\lambda\_{\mathrm{bet}} z\_s - \frac{1}{2}\lambda\_{\mathrm{bet}}^2) \mid \mathcal{F}\_{s-1}] \le 1$). Multiplying them yields a positive test supermartingale $E\_t = \prod\_{s=1}^t \exp(\lambda\_{\mathrm{bet}} z\_s - \frac{1}{2}\lambda\_{\mathrm{bet}}^2) > 0$. This breaks under heavy-tailed $z\_s$ or data-peeking $\lambda\_{\mathrm{bet}}$ (see [Chapter 18](18-philosophy-of-science.md) and [Chapter 30](30-probability-as-language.md) on open archive expansion $\mathcal{F}\_t \subsetneq \mathcal{F}'$). Positivity keeps Turing–Good decibans $W\_t = 10\log\_{10}(E\_t)$ (Good 1950) finite against $1/\alpha$. Meanwhile, `e-BH` controls the false discovery rate (FDR) across $K\_{\mathrm{test}}$ arbitrarily correlated tests without extra dependence penalties (Wang & Ramdas 2022), using parameters $\lambda\_{\mathrm{bet}}$ and $1/\alpha$ fixed in advance.
- **Width price of continuous monitoring:** Anytime safety widens intervals. For sub-Gaussian spread $\sigma > 0$, large horizon $T \ge 3$, and $\alpha \in (0, 1)$, confidence-sequence radius $r\_T(\alpha)$ decays as $\mathcal{O}(\sigma\sqrt{T^{-1}\ln T})$ (normal mixtures) or $\mathcal{O}(\sigma\sqrt{T^{-1}\ln\ln T})$ (iterated-logarithm bounds). This inflates fixed-sample radius $z\_{1-\alpha/2}\sigma/\sqrt{T}$ by $\Theta(\sqrt{\ln T})$ or $\mathcal{O}(\sqrt{\ln\ln T})$ (Howard et al. 2021; Ramdas et al. 2023), so single-look tests should not pay width for stopping flexibility they never use.

## 5. Strategic Classification and Game-Theoretic Interaction

In institutional settings, probability distributions are rarely passive. Once a rule $f\_\theta$ attaches high stakes to observed scores, evaluated actors adapt behavior or reporting $x \to x'$ against adaptation cost $c(x, x')$. Static prediction then turns into a **Stackelberg game** where every policy $\theta$ induces a different world distribution $\mathbb{P}\_\theta$ under Goodhart's law (Hardt, Megiddo, Papadimitriou & Wootters 2016; Milli et al. 2019; MacKenzie 2006).

In Perdomo et al.'s (2020) **performative prediction**, deploying rule $\theta$ shifts the joint distribution $(X, Y)$ to $\mathcal{D}(\theta)$. That separates **performative stability**—the fixed point where retraining no longer shifts the rule—from the **performative optimum** minimizing total risk net of induced shift, while strong feedback can break convergence altogether. Even the performative optimum optimizes only the institution's own objective: it can dump unfair adjustment burdens onto evaluated groups (Milli et al. 2019), conceal risks inside blind kernels $\ker(P)$ ([Chapter 14](14-linear-algebra.md)), and push unrecorded harm onto external reception perimeters—outside communities and habitats ([Chapter 19](19-ethics-and-the-imposition-of-risk.md)). Full contraction bounds and rule-freeze remedies belong to [Model, Fit, and Error](37-model-fit-and-error.md) and [Reactive Worlds: Causal Invariance, Performativity, and Goodhart's Law](48-reactive-worlds-performativity-goodhart.md).

### When the observation lies outside the measure

Sequential testing breaks whenever data step outside the reference measure's support, dividing into three cases. For continuous densities $f\_1, f\_0$ on $\mathbb{R}$, every point $\lbrace x \rbrace$ has zero probability $\mathbb{P}(\lbrace x \rbrace) = 0$, yet the Radon–Nikodym likelihood ratio $d\mathbb{P}\_1/d\mathbb{P}\_0 = f\_1(x)/f\_0(x)$ stays well-defined whenever $\mathbb{P}\_1$ stays inside $\mathbb{P}\_0$'s support and $f\_0(x) > 0$. If an event $E$ carries $\mathbb{P}\_0(E) = 0 < \mathbb{P}\_1(E)$, the likelihood ratio jumps to $+\infty \ge 1/\alpha$ and rejects $\mathbb{P}\_0$ instantly. When both give zero ($f\_0(x) = f\_1(x) = 0$, or $\mathbb{P}\_0(E) = \mathbb{P}\_1(E) = 0$), or when $E \notin \mathcal{F}\_t$ splits an unlogged bucket, $0/0$ is undefined—requiring quarantine and forward expansion ($\mathcal{F}\_t \subsetneq \mathcal{F}'$; §1).

### When the measure turns into a target

A measure built for a passive world cannot govern a reactive one once high stakes bend behavior ([Prologue](00-preface.md)). Inside contractive feedback, myopic retraining settles short of the performative optimum; beyond contraction, retraining chases its own echo without converging, requiring frozen rules and explicit protections against spillover onto external reception perimeters—outside bystanders and habitats ([Chapter 48](48-reactive-worlds-performativity-goodhart.md)).

## Bridge

[Probability as Language](30-probability-as-language.md) carries this measure foundation into the archive plane with finite vs. countable coherence, imprecise credal intervals, and anytime-valid *e*-processes. Meanwhile, [Model, Fit, and Error](37-model-fit-and-error.md), [Reactive Worlds: Causal Invariance, Performativity, and Goodhart's Law](48-reactive-worlds-performativity-goodhart.md), and [Linear Algebra](14-linear-algebra.md) pair performative shifts, frozen rules, and $\ker(P)$ blind spots with [Chapter 17](17-justification-and-induction.md), [Chapter 19](19-ethics-and-the-imposition-of-risk.md), and [Aim and the Three Planes](29-aim-and-the-three-planes.md).

Part II now turns from mathematics to philosophy in [What Knowledge Is](16-what-knowledge-is.md): consistent probability weights on $(\Omega, \mathcal{F})$ neither separate justified skill from lucky hits nor resolve internal *télos* versus external luck or non-reciprocal risk.
