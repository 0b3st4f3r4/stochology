# Deciding Without Knowing

<figure class="stoch-figure stoch-logo-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="800" height="132" viewBox="0 0 800 132" fill="none" role="img" aria-label="Stochology CH01 · STÓKHOS — The Archer And The Crosswind">
  <defs>
    <!-- Divider Progression: Vital Emerald -> Epistemic Sapphire -> Stókhos Gold -> Institutional Crimson -->
    <linearGradient id="ch01-ring" x1="2" y1="78" x2="386" y2="78" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="38%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="68%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <!-- Atmospheric Glow -->
    <radialGradient id="ch01-glow" cx="52%" cy="52%" r="50%">
      <stop offset="0%" stop-color="#fbbf24" stop-opacity="0.24"/>
      <stop offset="48%" stop-color="#38bdf8" stop-opacity="0.14"/>
      <stop offset="80%" stop-color="#e11d48" stop-opacity="0.08"/>
      <stop offset="100%" stop-color="#090e1a" stop-opacity="0"/>
    </radialGradient>
    <!-- Composite Recurve Bow Gradient: Vital Emerald Physical Tension -->
    <linearGradient id="ch01-bow-grad" x1="32" y1="24" x2="32" y2="108" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#34d399"/>
      <stop offset="50%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="100%" stop-color="#047857"/>
    </linearGradient>
    <!-- Stókhos Arrow Trajectory: Sapphire Nock to Solar Gold Tip -->
    <linearGradient id="ch01-arrow-grad" x1="20" y1="66" x2="88" y2="66" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="60%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="#ffffff"/>
    </linearGradient>
    <!-- Turbulent Crosswinds: Epistemic Upper & Governance Lower Streamlines -->
    <linearGradient id="ch01-wind-epi" x1="38" y1="38" x2="88" y2="38" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#38bdf8" stop-opacity="0.9"/>
      <stop offset="60%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.7"/>
      <stop offset="100%" stop-color="#1e3a8a" stop-opacity="0.2"/>
    </linearGradient>
    <linearGradient id="ch01-wind-gov" x1="48" y1="92" x2="90" y2="92" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#fb7185" stop-opacity="0.9"/>
      <stop offset="60%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.7"/>
      <stop offset="100%" stop-color="#881337" stop-opacity="0.2"/>
    </linearGradient>
    <!-- 3D Spherical Node Gradients -->
    <radialGradient id="ch01-node-well" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="var(--logo-well-hi, var(--elevated-bg, #ffffff))"/>
      <stop offset="100%" stop-color="var(--logo-node-bg, #e2e8f0)"/>
    </radialGradient>
    <linearGradient id="ch01-ring-epi" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#7dd3fc"/>
      <stop offset="50%" stop-color="#2563eb"/>
      <stop offset="100%" stop-color="#1e3a8a"/>
    </linearGradient>
    <radialGradient id="ch01-core-epi" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#bae6fd"/>
      <stop offset="45%" stop-color="#3b82f6"/>
      <stop offset="100%" stop-color="#1d4ed8"/>
    </radialGradient>
    <!-- Golden Stókhos Reticle Gradients -->
    <radialGradient id="ch01-pupil" cx="45%" cy="42%" r="58%">
      <stop offset="0%" stop-color="#020617"/>
      <stop offset="68%" stop-color="#0f172a"/>
      <stop offset="100%" stop-color="#451a03"/>
    </radialGradient>
    <linearGradient id="ch01-gold-mira" x1="54" y1="48" x2="84" y2="82" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#fef08a"/>
      <stop offset="45%" stop-color="#fbbf24"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <radialGradient id="ch01-gold-core" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#ffffff"/>
      <stop offset="35%" stop-color="#fde047"/>
      <stop offset="100%" stop-color="#d97706"/>
    </radialGradient>
  </defs>
  <g>
    <!-- Atmospheric Glow & Reference Coordinate Horizon -->
    <circle cx="64" cy="66" r="48" fill="url(#ch01-glow)"/>
    <circle cx="64" cy="66" r="42" stroke="var(--logo-ticks, #64748b)" stroke-width="1.2" stroke-dasharray="3 3" stroke-opacity="0.45"/>
    <!-- Composite Recurve Bow (Vital Margin S_vital Tension) -->
    <path d="M 32 24 C 44 42, 48 54, 42 66 C 48 78, 44 90, 32 108" stroke="url(#ch01-bow-grad)" stroke-width="4.0" stroke-linecap="round"/>
    <circle cx="32" cy="24" r="2.2" fill="#047857"/>
    <circle cx="32" cy="108" r="2.2" fill="#047857"/>
    <!-- Drawn Bowstring under Tension -->
    <path d="M 32 24 L 20 66 L 32 108" stroke="var(--logo-ticks, #64748b)" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"/>
    <!-- Arrow Vector (Stókhos Aim Shaft & Barbed Point) -->
    <line x1="20" y1="66" x2="83" y2="66" stroke="url(#ch01-arrow-grad)" stroke-width="3.0" stroke-linecap="round"/>
    <path d="M 77 60 L 85 66 L 77 72 Z" fill="url(#ch01-gold-mira)" stroke="url(#ch01-gold-mira)" stroke-width="1.6" stroke-linejoin="round"/>
    <line x1="27" y1="66" x2="23" y2="61" stroke="var(--logo-gold, #f59e0b)" stroke-width="1.6" stroke-linecap="round"/>
    <line x1="27" y1="66" x2="23" y2="71" stroke="var(--logo-gold, #f59e0b)" stroke-width="1.6" stroke-linecap="round"/>
    <line x1="24" y1="66" x2="20" y2="62" stroke="var(--logo-gold, #f59e0b)" stroke-width="1.5" stroke-linecap="round"/>
    <line x1="24" y1="66" x2="20" y2="70" stroke="var(--logo-gold, #f59e0b)" stroke-width="1.5" stroke-linecap="round"/>
    <!-- Turbulent Crosswinds (Epistemic Upper Streamline & Governance Lower Streamline) -->
    <path d="M 46 38 C 58 34, 72 44, 88 38" stroke="url(#ch01-wind-epi)" stroke-width="2.2" stroke-dasharray="4 2.5" stroke-linecap="round"/>
    <path d="M 38 48 C 52 44, 62 50, 74 46" stroke="url(#ch01-wind-epi)" stroke-width="1.6" stroke-dasharray="3 2" stroke-linecap="round" stroke-opacity="0.65"/>
    <path d="M 48 94 C 60 90, 74 98, 90 92" stroke="url(#ch01-wind-gov)" stroke-width="2.2" stroke-dasharray="4 2.5" stroke-linecap="round"/>
    <!-- Archer's Nock Node (3D Epistemic Sphere at Release Point) -->
    <circle cx="20" cy="66" r="8.5" fill="url(#ch01-node-well)" stroke="url(#ch01-ring-epi)" stroke-width="3.2"/>
    <circle cx="20" cy="66" r="3.8" fill="url(#ch01-core-epi)"/>
    <!-- Cost-Loss Decision Threshold Reticle at Target (94, 66) -->
    <g id="ch01-reticle">
      <circle cx="94" cy="66" r="7.4" fill="var(--logo-node-bg, #e2e8f0)"/>
      <circle cx="94" cy="66" r="6.4" fill="url(#ch01-pupil)" stroke="url(#ch01-gold-mira)" stroke-width="2.4"/>
      <line x1="84.4" y1="66" x2="89.8" y2="66" stroke="url(#ch01-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="98.2" y1="66" x2="103.6" y2="66" stroke="url(#ch01-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="94" y1="56.4" x2="94" y2="61.8" stroke="url(#ch01-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="94" y1="70.2" x2="94" y2="75.6" stroke="url(#ch01-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <circle cx="94" cy="66" r="2.2" fill="url(#ch01-gold-core)"/>
    </g>
  </g>
  <g transform="translate(148, 0)">
    <text x="2" y="27" fill="var(--logo-sub, #1d4ed8)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.2">PART I · THINKING UNDER UNCERTAINTY</text>
    <text x="498" y="27" text-anchor="end" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.0">CH01 · STÓKHOS</text>
    <text x="0" y="59" fill="var(--logo-ink, #0f172a)" font-family="system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif" font-size="22.2" font-weight="800" letter-spacing="1.35">THE ARCHER AND THE CROSSWIND</text>
    <line x1="2" y1="78" x2="384" y2="78" stroke="url(#ch01-ring)" stroke-width="1.6" stroke-opacity="0.82"/>
    <circle cx="400" cy="78" r="6.2" stroke="var(--logo-gold, #f59e0b)" stroke-width="1.5" stroke-dasharray="2.2 1.6"/>
    <circle cx="400" cy="78" r="4.0" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.2"/>
    <circle cx="400" cy="78" r="1.8" fill="var(--logo-gold, #f59e0b)"/>
    <circle cx="415" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="430" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="445" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-green, #10b981)" stroke-width="2.0"/>
    <circle cx="445" cy="78" r="1.6" fill="var(--logo-green, #10b981)"/>
    <circle cx="460" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="475" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="490" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-crimson, #e11d48)" stroke-width="2.0"/>
    <circle cx="490" cy="78" r="1.6" fill="var(--logo-crimson, #e11d48)"/>
    <text x="2" y="104" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.8" font-weight="600" letter-spacing="0.65">ACTION BEFORE OUTCOME · COST–LOSS THRESHOLD · IRREVERSIBLE STAKES</text>
  </g>
  </svg>
</figure>

> Intuition-first opener to Part I; formal apparatus continues in [What Judging Is, and What It Is Not](27-what-judging-is.md).

## By the end of this chapter you can

1. Tell a question about *what is the case* from a question about *what to do*, and say why the second one cannot be answered by waiting for the first.
2. Explain, with an ordinary example, why a decision costs something on both sides, acting and not acting, and compute the threshold $p^\ast = \min\bigl(1, C / (\eta L)\bigr)$ that sets the bar.
3. Say what this book is about in one sentence, without using the word *uncertainty*.

## 1. Two kinds of question

Some questions ask what is true. *How tall is that building? How many students are in the room?*
These have answers. The answers can be checked, and if two people disagree at least one of them is wrong.

Other questions ask what to do. *Should I take the umbrella? Should I answer this message now or
tomorrow? Should the school cancel the trip on Thursday?* These have answers too, but of a stranger
kind. Two people can look at the same sky, disagree about the umbrella, and both be right, because they
are not only asking about the sky. One of them has a long walk home. The other has a lift.

A flood director or an emergency physician at 07:50 faces the second kind across three arenas this book returns to throughout. In the **World**, the storm has not finished unfolding. In the **Archive**, the register is late and skips patients turned away at the door. In the **Institution**, deciding how much evidence suffices already depends on how grave each mistake is (Rudner 1953; Douglas 2000)—and the official who pulls the lever is rarely the household downstream that takes the hit.

The second kind of question is the subject of this book. It is not the first kind done badly. It is a
different question, with habits of its own.

## 2. The umbrella problem

You are leaving the house at 07:50. The sky is grey, the bus goes at 08:00, and the weather radar will not settle whether the squall hits your street until 09:00. Do you take the umbrella?

Start with what the two mistakes cost in plain words. Take the umbrella and the morning stays dry: you pay a small nuisance cost for carrying it around. Leave it behind and the sky opens: you suffer a much larger soaking loss on the walk home. Even when you bring it, an umbrella rarely blocks every drop of windblown rain, so it prevents *most* of the soaking rather than all of it.

Put simple numbers on the table to see how the choice balances. Suppose carrying the umbrella costs $1$ unit of hassle, getting caught unbuffered costs $10$ units, and the umbrella blocks $80\unicode{x25}$ of the soaking—so even with it, windblown dampness still leaves $2$ units of discomfort. If the rain chance is $50\unicode{x25}$ (half a chance of rain), taking the umbrella costs $1$ for carrying plus $0.5 \times 2 = 1$ in leftover dampness, for an average cost of $2$; leaving it behind costs $0.5 \times 10 = 5$ on average. You take it. Even at a $20\unicode{x25}$ chance of rain, taking it averages $1 + 0.2 \times 2 = 1.4$ against $0.2 \times 10 = 2.0$ for waiting. Where do the two choices exactly break even? At the rain chance where the carrying cost equals the harm the umbrella actually prevents: $1 = p \times 8$, which gives $p = 1/8 = 12.5\unicode{x25}$.

Now turn that arithmetic into general symbols. Write $C$ for the fixed cost of acting (carrying the umbrella, always paid) and $L$ for the unbuffered loss if you wait and the storm hits. Write $\eta$ (between $0$ and $1$) for the share of that loss the umbrella prevents, $p$ for your assessed chance of rain, and $\mathbb{E}$ for the expected (average) cost across rain and shine. Then:

$$\mathbb{E}[\text{Cost}(\text{Act})] = C + (1 - \eta) p L, \qquad \mathbb{E}[\text{Cost}(\text{Wait})] = p L$$

Read that pair aloud: acting costs $C$ up front plus the leftover unblocked share $(1 - \eta)$ whenever rain arrives with probability $p$; waiting costs nothing if it stays dry and the full loss $L$ whenever rain arrives. For positive action cost $C > 0$, positive inaction loss $L > 0$, and positive protection $\eta \in (0, 1]$ (so $\eta L > 0$), acting beats waiting whenever its expected cost is smaller:

$$C + (1 - \eta) p L < p L \iff C < \eta p L \iff p > \frac{C}{\eta L}$$

Since a probability $p$ cannot exceed $1$, the effective action cutoff is $p^\ast = \min\bigl(1, C / (\eta L)\bigr)$: act whenever your assessed chance of rain $p$ exceeds $p^\ast$ (which coincides with the raw cost–loss ratio $C / (\eta L)$ whenever $\eta L > C$). Read the fraction aloud: $C$ sits upstairs because you pay it every time you act, rain or shine; $\eta$ and $L$ sit downstairs because the umbrella only saves you the fraction $\eta$ of the soaking $L$.

Notice both ends of that ratio. If carrying cost $C$ shrinks toward zero, the cutoff $p^\ast$ drops toward zero too: even a tiny chance of rain justifies taking shelter. At the other extreme, suppose the harm prevented is only twice the carrying cost ($\eta L = 2C$); then $p^\ast = 1/2$, so you act only once rain is more likely than not. And if protection costs at least as much as the harm it prevents ($\eta L \le C$), the ratio $C / (\eta L)$ reaches or passes $1$, clamping $p^\ast = 1$ so acting never pays. You do not spend $10$ dollars on an umbrella to protect a $5$-dollar pair of shoes.

Meteorologists have used this cost-to-loss cutoff $p^\ast = \min(1, C / L)$ for decades to decide when to issue storm warnings (Thompson & Brier 1955; Richardson 2000), and $\eta$ extends it cleanly when cover is only partial. In clinical medicine, Stephen Pauker and Jerome Kassirer built the sister rule for deciding when to treat or withhold treatment under unresolved diagnostic uncertainty (Pauker & Kassirer 1975). They compare the harm of treating a healthy patient against the total stakes of treating unnecessarily plus missing a sick patient (developed alongside three-zone testing rules in [Decision, Loss, Threshold](36-decision-loss-threshold.md)).

When $L$ towers over $C$ under full cover ($\eta = 1$), $p^\ast = \min(1, C / L)$ is tiny; someone with an underground garage faces almost zero soaking ($L \approx 0$) under the same grey sky and rightly leaves the umbrella in the stand.

Two uncomfortable limits stay with us for the rest of the book. First, there is no single universal answer, only a right answer *given* each mistake's cost—and *whose* harm counted on the ledger (Harding 1991). Arguing over the umbrella without naming those stakes is talking past each other. Second, this simple rule assumes one person facing one fixed loss $L$ with harmless false alarms. It breaks when losses curve sharply upward in extremes ([Chapter 02](02-the-average-is-not-the-story.md)), when one person pays $C$ while someone else suffers $L$, or when false alarms drown human attention ([Chapter 36](36-decision-loss-threshold.md)). Worse still, afterward you only learn whether it rained—never whether your $p$ matched the true hidden odds. Confusing the result with how well you aimed is this book's commonest trap.

## 3. Why you cannot wait

You can wait until you know, you say. The bus goes at eight. The moment for deciding arrives before the information does.

That is the shape of nearly every serious choice. The exam is graded before you learn which study habit worked, and a medical team treating suspected sepsis cannot wait for a slow blood culture while organs begin to fail. The Hippocratic physicians had a word for that closing window: **καιρός** (*kairós*), the decisive moment that does not wait for certainty (Hippocrates c. 400 BCE, *Aphorisms* I.1). Millennia earlier along the Nile, clinical triage separated actionable emergencies from guarded or untreatable wounds ([Chapter 27](27-what-judging-is.md); Breasted 1930), just as Nahuatl *tlalticpac* ([the Prologue](00-preface.md)) marked earthly footing where every careless step carries irreversible cost. Anyone who stalls for perfect proof while the clock runs out gets ruined by the very storm they waited to confirm, and later improvements in the forecast arrive too late.

The opposite mistake is mistaking a reversible choice for a permanent lock-in, or solving the wrong question with great precision—what Allyn Kimball (1957) called an error of the third kind—while missing wider downstream harm. Only when delay seals ruin, acting keeps things safe, and protection cannot be broken into small reversible steps does an all-or-nothing threshold settle who moves first.

The stance comes from archery. Classical Stoic sources separate the internal **τέλος** (*télos*)—Antipater's requirement to do everything within one's power continuously toward natural ends (Stobaeus 1884, *Ecl.* II.76.11–15)—from the external **σκοπός** (*skopós*; *Ecl.* II.76.16–77.5), a contrast Cicero develops with the archer who must do everything within his power to aim straight while hitting the mark lies beyond full control (*De Finibus* III.22; Cicero 45 BCE) and Plutarch criticizes (*De comm. not.* 1070f–1071e; Plutarch c. 100 CE). Against critics who asked how aiming well could matter if the target can still be missed, separating *télos* from *skopós* shields a single sound act from unfair hindsight while letting craft calibrate across many shots—extending reasoned medical conjecture (*stokhasmós tekhnikós*, Galen c. 200 CE; cf. the conjectural arts *stokhastikaì tékhnai* in the Pseudo-Galenic *Introductio*) and Bernoulli's *ars conjectandi* (Bernoulli 1713) in [What Judging Is, and What It Is Not](27-what-judging-is.md). What you controlled before release and what the wind did afterward are judged by separate standards. The same split governs honest speech ([the Prologue](00-preface.md); [Chapter 29](29-aim-and-the-three-planes.md)): candor is yours to uphold, while how a listener hears it lies downstream. Replacing honest craft with scoreboard hits is the distortion this book traces across institutions.

## 4. What this book is about

This book is about getting better at the second kind of question: deciding before you can know, and being
honest afterwards about what you decided and why.

It is a real skill. It can be learned, and it has more structure than it looks: some of it arithmetic,
which the later chapters supply. The opening chapters need only that you notice three ordinary
difficulties: records are incomplete and filtered, physical time runs out before certainty arrives,
and organizations split who pays for safety from who takes the hit.

Those three map onto the three arenas of the loop this book runs on—**Archive**, **World**,
and **Institution**—and each carries a safeguard that cannot be bought with the other two:

1. **Archive and honest knowledge:** Records are finite, clustered, and filtered before anyone sees them. An honest decision maker refuses fake certainty and tracks both the action cutoff $p^\ast = \min\bigl(1, C / (\eta L)\bigr)$ and what the record leaves unknown. Remember too that a mountain of data collected through one skewed door can hold less real evidence than a small unbiased sample (Meng 2018), a paradox unpacked with simple counts in [Who Is Missing From the Table](06-who-is-missing-from-the-table.md).
2. **World and survival slack:** Physical time runs out while committees deliberate (*kairós*), and damage compounds faster than averages suggest. A sound system keeps real buffers and fast recovery instead of betting survival on an average day.
3. **Institution and accountability:** Budgets often separate the official who pays the precaution cost $C$ from the citizen who suffers the loss $L$. Hindsight also confuses lucky outcomes (*skopós*) with careful aim (*télos*), while false alarms crowd limited human attention. Governance must therefore tie authority to the real exposure it creates and keep frontline warnings open before the window closes.

### The same forecast, two different losers

A city flood official and a floodplain resident read the exact same rain forecast and disagree on whether to evacuate or reinforce the levee. Both can be completely rational inside their own ledgers. They share the action cost $C > 0$ and protection share $\eta \in (0, 1]$, separated only by whose loss $L$ enters the threshold:

$$p^\ast = \min\left(1,\mkern3mu \frac{C}{\eta\mkern3mu L}\right)$$

Read that minimum aloud: the action cutoff is $C / (\eta L)$, capped at $1$ (meaning $100\unicode{x25}$) whenever protection costs more than the harm it prevents.

Try two simple numbers. Suppose setting up flood barriers costs $C = 1$ unit and prevents $100\unicode{x25}$ of damage ($\eta = 1$). The household faces a ruined home worth $L\_{\mathrm{affected}} = 10$ units, so its threshold is $p^\ast\_{\mathrm{affected}} = 1/10 = 10\unicode{x25}$. The city agency faces a capped political or budget loss of only $L\_{\mathrm{inst}} = 2$ units, so its threshold is $p^\ast\_{\mathrm{inst}} = 1/2 = 50\unicode{x25}$. Whenever the forecast sits anywhere between $10\unicode{x25}$ and $50\unicode{x25}$—between $p^\ast\_{\mathrm{affected}}$ and $p^\ast\_{\mathrm{inst}}$—the resident begs for action while the agency waits. Even worse, if the agency's capped liability $L\_{\mathrm{inst}}$ sits at or below $C / \eta$, the ratio reaches or exceeds $1$, so its cutoff clamps at $p^\ast\_{\mathrm{inst}} = 1$ and it *never* acts at all.

That disagreement is not about the weather. Below $p^\ast\_{\mathrm{affected}}$ both wait; above $p^\ast\_{\mathrm{inst}}$ both act; in the gap between them, the ledger splits who pays the bill. Raising $L\_{\mathrm{inst}}$ to reflect the resident's true harm closes the gap without changing a single weather sensor.

### When the thesis stops holding

"You cannot wait" governs all-or-nothing, irreversible stakes. When a choice works like a refundable ticket—most of the cost comes back if the storm misses—because a false alarm wastes little, acting pays sooner and waiting makes sense only at lower probabilities than for a permanent bet. And when you can test the waters with a small, reversible step while uncertainty spans the threshold, taking a partial precaution beats both locking in everything and waiting passively ([Decision, Loss, Threshold](36-decision-loss-threshold.md); [The Triad and the Minimum](41-the-triad-and-the-minimum.md); [Inquiry, Framing, and Antidogmatic Openness](43-inquiry-openness-multiverse.md)). Conversely, when acting *itself* locks in irreversible harm or freezes a bad rule against incoming facts, holding back or revising the rule beats rushing ahead unless standing still guarantees ruin.

## Bridge

[What Judging Is, and What It Is Not](27-what-judging-is.md) develops the classical vocabulary of *stókhos*, while [Decision, Loss, Threshold](36-decision-loss-threshold.md) formalizes cost–loss and clinical regret cutoffs. [The Average Is Not the Story](02-the-average-is-not-the-story.md) turns next to the loss $L$ we assumed was a single tidy number here, showing what spread, upward-curving harm, extreme shocks, and selective records leave out.
