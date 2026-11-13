# Decision, Loss, Threshold

<figure class="stoch-figure stoch-logo-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="660" height="132" viewBox="0 0 660 132" fill="none" role="img" aria-label="Stochology CH36 · THRESHOLDS — Decision, Asymmetric Loss, And Regret">
  <defs>
    <linearGradient id="ch36-ring" x1="2" y1="78" x2="386" y2="78" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="38%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="68%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <radialGradient id="ch36-glow" cx="50%" cy="50%" r="50%">
      <stop offset="0%" stop-color="#38bdf8" stop-opacity="0.25"/>
      <stop offset="50%" stop-color="#2563eb" stop-opacity="0.14"/>
      <stop offset="82%" stop-color="#fbbf24" stop-opacity="0.08"/>
      <stop offset="100%" stop-color="#090e1a" stop-opacity="0"/>
    </radialGradient>
    <linearGradient id="ch36-triad-stroke" x1="18" y1="24" x2="110" y2="104" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.85"/>
      <stop offset="50%" stop-color="var(--logo-gold, #f59e0b)" stop-opacity="0.90"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.85"/>
    </linearGradient>
    <linearGradient id="ch36-fill" x1="22" y1="30" x2="106" y2="98" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)" stop-opacity="0.26"/>
      <stop offset="48%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.28"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.26"/>
    </linearGradient>
    <linearGradient id="ch36-ink-curve" x1="20" y1="66" x2="108" y2="66" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-ink, #0f172a)"/>
      <stop offset="50%" stop-color="var(--logo-sub, #1d4ed8)"/>
      <stop offset="100%" stop-color="var(--logo-ink, #0f172a)"/>
    </linearGradient>
    <radialGradient id="ch36-pupil" cx="45%" cy="42%" r="58%">
      <stop offset="0%" stop-color="#020617"/>
      <stop offset="68%" stop-color="#0f172a"/>
      <stop offset="100%" stop-color="#451a03"/>
    </radialGradient>
    <linearGradient id="ch36-gold-mira" x1="54" y1="48" x2="84" y2="82" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#fef08a"/>
      <stop offset="45%" stop-color="#fbbf24"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <radialGradient id="ch36-gold-core" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#ffffff"/>
      <stop offset="35%" stop-color="#fde047"/>
      <stop offset="100%" stop-color="#d97706"/>
    </radialGradient>
    <radialGradient id="ch36-node-well" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="var(--logo-well-hi, var(--elevated-bg, #ffffff))"/>
      <stop offset="100%" stop-color="var(--logo-node-bg, #e2e8f0)"/>
    </radialGradient>
    <linearGradient id="ch36-ring-epi" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#7dd3fc"/>
      <stop offset="50%" stop-color="#2563eb"/>
      <stop offset="100%" stop-color="#1e3a8a"/>
    </linearGradient>
    <radialGradient id="ch36-core-epi" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#bae6fd"/>
      <stop offset="45%" stop-color="#3b82f6"/>
      <stop offset="100%" stop-color="#1d4ed8"/>
    </radialGradient>
    <linearGradient id="ch36-ring-vital" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#6ee7b7"/>
      <stop offset="50%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#065f46"/>
    </linearGradient>
    <radialGradient id="ch36-core-vital" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#a7f3d0"/>
      <stop offset="45%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#047857"/>
    </radialGradient>
    <linearGradient id="ch36-ring-gov" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#fda4af"/>
      <stop offset="50%" stop-color="#e11d48"/>
      <stop offset="100%" stop-color="#881337"/>
    </linearGradient>
    <radialGradient id="ch36-core-gov" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#fecdd3"/>
      <stop offset="45%" stop-color="#f43f5e"/>
      <stop offset="100%" stop-color="#be123c"/>
    </radialGradient>
  </defs>
  <g>
    <circle cx="64" cy="66" r="48" fill="url(#ch36-glow)"/>
    <circle cx="64" cy="66" r="42" stroke="var(--logo-ticks, #64748b)" stroke-width="1.2" stroke-dasharray="3 3" stroke-opacity="0.45"/>
    <line x1="24" y1="92" x2="104" y2="28" stroke="var(--logo-crimson, #e11d48)" stroke-width="2.8" stroke-linecap="round"/>
    <line x1="24" y1="52" x2="104" y2="84" stroke="var(--logo-blue, #2563eb)" stroke-width="2.8" stroke-linecap="round"/>
    <line x1="56" y1="66" x2="56" y2="98" stroke="url(#ch36-gold-mira)" stroke-width="2.4" stroke-dasharray="3 2.5"/>
    <circle cx="24.00" cy="52.00" r="6.38" fill="url(#ch36-node-well)" stroke="url(#ch36-ring-epi)" stroke-width="2.36"/>
    <circle cx="24.00" cy="52.00" r="2.74" fill="url(#ch36-core-epi)"/>
    <circle cx="104.00" cy="28.00" r="6.38" fill="url(#ch36-node-well)" stroke="url(#ch36-ring-gov)" stroke-width="2.36"/>
    <circle cx="104.00" cy="28.00" r="2.74" fill="url(#ch36-core-gov)"/>
    <g>
      <circle cx="56.00" cy="66.00" r="6.96" fill="var(--logo-node-bg, #e2e8f0)"/>
      <circle cx="56.00" cy="66.00" r="6.02" fill="url(#ch36-pupil)" stroke="url(#ch36-gold-mira)" stroke-width="2.26"/>
      <line x1="46.98" y1="66.00" x2="52.05" y2="66.00" stroke="url(#ch36-gold-mira)" stroke-width="2.26" stroke-linecap="round"/>
      <line x1="59.95" y1="66.00" x2="65.02" y2="66.00" stroke="url(#ch36-gold-mira)" stroke-width="2.26" stroke-linecap="round"/>
      <line x1="56.00" y1="56.98" x2="56.00" y2="62.05" stroke="url(#ch36-gold-mira)" stroke-width="2.26" stroke-linecap="round"/>
      <line x1="56.00" y1="69.95" x2="56.00" y2="75.02" stroke="url(#ch36-gold-mira)" stroke-width="2.26" stroke-linecap="round"/>
      <circle cx="56.00" cy="66.00" r="2.07" fill="url(#ch36-gold-core)"/>
    </g>
  </g>
  <g transform="translate(148, 0)">
    <text x="2" y="27" fill="var(--logo-sub, #1d4ed8)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.2">PART III · FOUNDATIONS OF JUDGMENT</text>
    <text x="498" y="27" text-anchor="end" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.0">CH36 · THRESHOLDS</text>
    <text x="0" y="59" fill="var(--logo-ink, #0f172a)" font-family="system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif" font-size="18.5" font-weight="800" letter-spacing="0.95">DECISION, ASYMMETRIC LOSS, AND REGRET</text>
    <line x1="2" y1="78" x2="384" y2="78" stroke="url(#ch36-ring)" stroke-width="1.6" stroke-opacity="0.82"/>
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
    <text x="2" y="104" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.8" font-weight="600" letter-spacing="0.65">MINIMAX REGRET · PAUKER–KASSIRER CRITICAL ODDS · VALUE OF INFORMATION</text>
  </g>
  </svg>
</figure>

> Foundation chapter of Part III. It presupposes [Chapter 12](12-derivatives-and-convexity.md)
> and [Chapter 19](19-ethics-and-the-imposition-of-risk.md). Its formal content is worked out
> in [Chapter 43](43-inquiry-openness-multiverse.md).

## By the end of this chapter you can

1. Turn a decision problem into costs and losses, and say which of the two you are allowed to
   estimate.
2. Derive the threshold at which acting beats waiting, and read it as a property of the
   consequences rather than of the evidence.
3. Say what information would be worth paying for, and when the answer is *none*.

## 1. Actions, states, and the loss matrix

A decision problem has three parts: the set of available actions $\mathcal{A}$, the set of possible states of the world $\mathcal{S}$, and the loss function $\mathcal{L}(a, s)$ that maps each action-state pair to a consequence (Raiffa 1968). Statistical models and historical archives say something about one of them, the probability distribution over states $\mathbb{P}(s)$. Everything else (the enumeration of allowable
actions and the valuation of harms) is an ethical and institutional choice.

The loss matrix is a constitutional document. Setting a false alarm at ten units and a missed catastrophe at a thousand is not an empirical measurement. It is an explicit valuation of competing harms, decided by an author, with grave consequences for whoever bears the downside. Most debates
in public policy that present themselves as disagreements over statistical evidence are in reality disguised
conflicts over the entries of the loss matrix.

## 2. The cost–loss threshold and the Pauker-Kassirer form

Two thresholds are routinely filed under one name, and the difference is not cosmetic: most tooling computes
one of them, and a reader who substitutes the other will compute a different boundary from the same two budgets.

The **cost–loss threshold** comes from a preventive action that incurs a certain protection cost $C > 0$ and
averts a fraction $\eta \in (0, 1]$ of an avoidable loss $L > C$. Let $p$ be the probability that the harmful
event occurs. Acting costs $C$ whatever happens and still leaves $(1 - \eta)L$ to be paid when the event
occurs; not acting costs $L$ when it occurs. Equating the two expected expenses gives:

$$p^\ast\_{\mathrm{CL}} = \frac{C}{\eta L}.$$

The **Pauker–Kassirer threshold** (1975) is instead an expected-regret comparison between two classifications.
Let $C\_{\mathrm{FP}}$ be the net cost of a false alarm (acting when no event occurs) and $L\_{\mathrm{FN}}$
the net loss of a missed event, failing to act when the event occurs. Acting whenever
$p\mkern3muL\_{\mathrm{FN}} > (1 - p)\mkern3muC\_{\mathrm{FP}}$ gives

$$p^\ast\_{\mathrm{PK}} = \frac{C\_{\mathrm{FP}}}{C\_{\mathrm{FP}} + L\_{\mathrm{FN}}} = \frac{1}{1 + L\_{\mathrm{FN}}/C\_{\mathrm{FP}}},$$

which is the expected-regret form written as a ratio: it returns $c/(c + l)$ for two non-negative error costs, falls
to parity when the two budgets are equal, and tends to zero as the missed loss grows without bound relative to the
false-alarm cost. The cost–loss form asks whether a protection budget is worth its expected saving; the
expected-regret form asks which of two errors is cheaper. Institutions budget the second, so both are quoted
here and always with the subscript that names them.

The decision rule is the same in either form: execute the intervention when the estimated probability of
danger exceeds the threshold, and withhold it when the estimate falls below.

When an intermediate diagnostic test is available with cost $C\_{\mathrm{test}}$, sensitivity $\mathrm{Sens}$,
and specificity $\mathrm{Spec}$, Pauker and Kassirer demonstrated that the decision space partitions into three
distinct operational regimes:

$$\text{Withhold Action} \quad \longleftarrow \quad p^\ast\_{\mathrm{test}} \quad \longleftarrow \quad \text{Order Diagnostic Test} \quad \longrightarrow \quad p^\ast\_{\mathrm{treat}} \quad \longrightarrow \quad \text{Act Immediately}.$$

The testing threshold $p^\ast\_{\mathrm{test}}$ defines the point where the expected value of diagnostic clarification
exceeds the physical and financial cost of the test. Below $p^\ast\_{\mathrm{test}}$, the disease is so improbable that
even a positive test result would likely be a false alarm; above $p^\ast\_{\mathrm{treat}}$, the disease is so probable
and dangerous that delaying intervention to wait for test results is malpractice.

Two properties of the threshold do most of the work in practice.

1. *The threshold is a property of consequences, not of evidence.* Two agents evaluating the identical probability
   estimate $\hat{p}$ will rationally choose opposite actions if one faces a small protection cost against a large
   avoidable loss (threshold below $\hat{p}$, so act) while the other faces a protection cost comparable to the
   avoidable loss (threshold above $\hat{p}$, so wait). Reporting only the probability $\hat{p}$ without publishing the
   consequence threshold conceals the contested core of the decision.
2. *The threshold routinely sits far from one-half.* Where intervention is cheap and harm is catastrophic, the
   threshold plummets to a few percent. Demanding "proof beyond a reasonable doubt" before evacuating a flood
   plain or administering emergency prophylaxis is reckless negligence.

A forecast score and a decision rule are two different objects. Mapped through the same cost–loss ratio, the forecast is worth nothing at either extreme and most at an interior ratio, so a better score need not be a better decision (Richardson 2000).

## 3. Asymmetric loss, standards of proof, and Minimax Regret

Legal traditions map this consequence ratio directly onto their hierarchy of standards of proof:

1. **Preponderance of the evidence**: In private civil disputes where an erroneous verdict in either direction
   carries equal positive gravity ($C\_{\mathrm{FP}} = L\_{\mathrm{FN}} > 0$), the expected-regret indifference
   point sits at parity, $p^\ast\_{\mathrm{PK}} = 1/2$, so liability attaches strictly above one-half ($p > 1/2$).
2. **Clear and convincing evidence**: Where civil sanctions threaten liberty, professional licensure, or parental
   rights, pricing a false sanction at an intermediate multiple $k > 1$ of a false dismissal
   ($C\_{\mathrm{FP}} = k\mkern2mu L\_{\mathrm{FN}}$) raises the expected-regret threshold to
   $p^\ast\_{\mathrm{PK}} = k/(k + 1) > 1/2$, strictly between civil parity and the criminal floor whenever
   $1 < k < 10$.
3. **Beyond a reasonable doubt**: In criminal prosecution, Blackstone's ratio (Blackstone 1765–1769) asserts that it is
   better that ten guilty persons escape than that one innocent suffer, i.e. a false conviction is ten times as
   costly as a false acquittal ($C\_{\mathrm{FP}} = 10 L\_{\mathrm{FN}}$). Read through the expected-regret form of
   §2, the threshold becomes:
   $$p^\ast\_{\mathrm{PK}} = \frac{10}{10 + 1} = \frac{10}{11}.$$
4. **Learned Hand's negligence formula**: In tort law, Judge Learned Hand defined negligence as failing to take
   precautions where $B < PL$ (Burden $B$, Probability $P$, Loss $L$). Setting $C = B$, $L$ equal to the accident
   loss and $\eta = 1$ reduces the cost–loss threshold of §2 to $P > B/L = p^\ast\_{\mathrm{CL}}$, stated as a legal
   duty of care. When an injurer's downside is truncated by limited corporate liability,
   their private $L\_{\mathrm{private}}$ collapses, driving their
   private threshold $p^\ast\_{\mathrm{private}} = B/L\_{\mathrm{private}}$ far above the social optimum and
   externalizing catastrophe onto the public.

When the event probability $p\_t$ is not known as a single point, but is partially identified within $[p\_{L,t}, p\_{U,t}]$, the expected avoidable loss $p\_t\mkern2mu\eta L$ lies in the cost-unit interval $[L\_t, U\_t] := [p\_{L,t}\mkern2mu\eta L,\mkern3mu p\_{U,t}\mkern2mu\eta L]$, expressed in the same cost units as $C$. Whenever $[L\_t, U\_t]$ straddles $C$ ($L\_t < C < U\_t$, equivalently $p\_{L,t} < p^\ast\_{\mathrm{CL}} < p\_{U,t}$), committing entirely to zero or full action risks severe error; under **Minimax Regret** over $a \in [0, 1]$, the fractional commitment that minimizes maximum possible regret is:

$$a^\ast\_{\mathrm{MMR}} = \frac{U\_t - C}{U\_t - L\_t} = \frac{p\_{U,t} - p^\ast\_{\mathrm{CL}}}{p\_{U,t} - p\_{L,t}} \in (0, 1).$$

The throttle of the commitment is proportional to the distance between the threshold and the boundaries of what is
unknown.

The interval case is not a technicality: a decision maker facing an unknown composition treats it differently from a known one of the same stated odds, so the rule and the agent's own weighting are two objects (Ellsberg 1961).

## 4. The Expected Value of Perfect Information (EVPI)

Before spending organizational resources to collect more data, an institution must calculate what that information
could physically change. The **Expected Value of Perfect Information (EVPI)** is the mathematical difference between
the expected loss of the optimal action chosen under current uncertainty and the expected loss if the true state of
the world were revealed before deciding:

$$\mathrm{EVPI} = \min\_{a} \mathbb{E}\_s\left[\mathcal{L}(a, s)\right] - \mathbb{E}\_s\left[\min\_{a} \mathcal{L}(a, s)\right].$$

Three circumstances give it a value of zero.

1. *Irrelevance*: The uncertain variable has no causal bearing on the payoff matrix.
2. *Dominance*: The optimal action is identical across all possible states that the inquiry could reveal.
3. *Latency expiration*: The data collection process takes longer than the operational window to act; the deadline
   forces the decision before the evidence can be delivered.

## 5. Cognitive alert filtering and the information bottleneck

An operational alert system is an automated classifier with a human operator at the terminus. System designers
routinely evaluate detectors by sensitivity and specificity, ignoring that human operators experience only the
**Positive Predictive Value (PPV)**:

$$\mathrm{PPV} = \frac{\mathrm{Sens} \times \text{Prevalence}}{(\mathrm{Sens} \times \text{Prevalence}) + ((1 - \mathrm{Spec}) \times (1 - \text{Prevalence}))}.$$

When detecting rare clinical emergencies or cyber intrusions at a low prevalence, a system with high sensitivity and
high specificity still produces an operational PPV far below one: the rarer the event relative to the false-alarm
rate, the smaller the share of alarms that are real, and once prevalence drops well below $(1-\mathrm{Spec})/\mathrm{Sens}$ the positive predictive value falls toward zero, making nearly every alarm spurious.

Human cognitive psychology and neurobiology impose a physical constraint on the receiver.
In 2025, Zheng and Meister demonstrated that conscious human behavioral throughput is strictly bounded at
approximately **ten bits per second** ($\sim 10\text{ bits/s}$) (Zheng & Meister 2025). An institutional alert stream that
inundates human operators with alarms exceeding this throughput capacity triggers an inevitable behavioral collapse:
operators experience **alert fatigue**, dismissing alarms reflexively without conscious cognitive processing.
A high-sensitivity model on paper becomes a zero-sensitivity filter in practice.

Effective alert design must run in reverse from the human bottleneck: calibrate the alarm volume to the operator's
ten-bit bandwidth, allocate the alert budget across priority hazards, and present notifications as natural
frequencies rather than abstract conditional probabilities.

### Two thresholds, two denominators

There are two threshold rules and they are not the same rule. The cost-ratio form gives $p^\ast\_{PK} = C\_{FP}/(C\_{FP} + L\_{FN})$, which reads the missed loss as a plain error cost; the efficiency form gives $p^\ast\_{CL} = C/(\eta L)$, which discounts the avoidable loss by the fraction the protection actually averts. Same decision, two denominators, and a chapter that mixes them will produce a threshold that belongs to neither.

The difference is what the denominator counts. In the cost-ratio form the loss enters unweighted, because it is the loss of a decision already taken; in the efficiency form it is discounted by how often the protective act actually works. Writing one as the other is the most common arithmetic error in published thresholds, and it is invisible because both produce numbers between zero and one. That choice also governs the Keynesian reference sample $n\_0 := n\_{\mathrm{req}}(\alpha, \beta, p^\star) = \bigl(\frac{z\_{1-\alpha} + z\_{1-\beta}}{\varepsilon\_{\mathrm{rel}}}\bigr)^2 \frac{1 - p^\star}{p^\star}$ ($\alpha + \beta < 1$, $\varepsilon\_{\mathrm{rel}} > 0$): under the cost–loss threshold $p^\star = p^\ast\_{\mathrm{CL}}$, the threshold odds factor is $\eta L/C - 1$, whereas under the Pauker–Kassirer form $p^\star = p^\ast\_{\mathrm{PK}}$ it is $L\_{\mathrm{FN}}/C\_{\mathrm{FP}}$.

The threshold is also where the institution and the person diverge: at a forecast that falls between the two thresholds the two rules disagree about acting, and neither is wrong, they answer different questions about who bears the loss.

### When the threshold should not be used

The rule stops being applicable when the loss is not commensurable with the cost, when one side of the comparison is a number and the other is a right. The procedure is then to state the incommensurability, decide on the side that does not require the comparison, and record that the threshold was not applied rather than applying a number that pretends to measure what it does not. The two-rule distinction itself is not new; it is the diagnostic-threshold tradition's own warning against collapsing the cost ratio into a single number (Pauker & Kassirer 1975), and the warning survives because the collapse keeps being convenient.

## 6. The three-regime partition, the wrong recommendation and the fixed point

### The band, derived and pushed to its limits

§2 asserts that a diagnostic test opens a band between two thresholds; the two formulas are short enough to
derive and then to push to their limits. Regret against the alternatives gives, for acting now,
$R\_{\mathrm{act}} = (1-p)C\_{\mathrm{FP}}$; for withholding, $R\_{\mathrm{wait}} = p\mkern3muL\_{\mathrm{FN}}$; and for
testing first, $R\_{\mathrm{test}} = C\_{\mathrm{test}} + (1-p)(1-\mathrm{Spec})C\_{\mathrm{FP}} + p(1-\mathrm{Sens})L\_{\mathrm{FN}}$.
Setting $R\_{\mathrm{test}} = R\_{\mathrm{wait}}$ yields the lower edge and $R\_{\mathrm{test}} = R\_{\mathrm{act}}$
the upper:

$$p^\ast\_{\mathrm{test}} = \frac{C\_{\mathrm{test}} + (1-\mathrm{Spec})C\_{\mathrm{FP}}}{\mathrm{Sens}\mkern3muL\_{\mathrm{FN}} + (1-\mathrm{Spec})C\_{\mathrm{FP}}}, \qquad p^\ast\_{\mathrm{treat}} = \frac{\mathrm{Spec}\mkern3muC\_{\mathrm{FP}} - C\_{\mathrm{test}}}{\mathrm{Spec}\mkern3muC\_{\mathrm{FP}} + (1-\mathrm{Sens})L\_{\mathrm{FN}}}.$$

Take a test that is worthless and free, $\mathrm{Sens} = \mathrm{Spec} = 1/2$ and $C\_{\mathrm{test}} = 0$, and
evaluate both edges: they coincide, so the band has zero width,
which is the correct statement that a coin-flip test cannot occupy a region. Take instead
a perfect and free test, $\mathrm{Sens} = \mathrm{Spec} = 1$ and $C\_{\mathrm{test}} = 0$: the lower edge is
zero and the upper is one, so the band is the whole unit interval and immediate action is never
rational. Between the two limits the arithmetic is the ordinary
case, with an imperfect and costly test yielding two edges strictly inside the unit interval and an ordering
that says test first only where the forecast falls between them.

The operating point that matters comes from the institution rather than from the pair. In a call-arrival series
a rare surge interval sits far below the lower edge of the band, and a detector with conventional sensitivity and
specificity applied at that prevalence delivers a positive predictive value far below one: the partition's own
rule then says withhold and do not order the test, and the reason is visible in the second number, where most
alarms are false.

### The case where the crossed threshold was the wrong decision

The cost–loss form is arithmetic, and arithmetic can be obeyed and still lose. In February 1976 an outbreak of
influenza at Fort Dix, New Jersey, produced one death and a strain resembling the 1918 pandemic virus. The loss
matrix written that spring had a small $C$ (a vaccine dose and a delivery programme) against an $L$ inherited
from 1918, so the threshold sat at a few per cent and any non-trivial probability cleared it. The
national immunisation programme that followed reached roughly forty-five million people, was suspended on
16 December 1976, and left some five hundred cases of Guillain–Barré syndrome and twenty-five deaths attributed to
the vaccine (Neustadt & Fineberg 1983). The form of §2 applied exactly as advertised: the probability
was above the threshold, the recommendation was to act, and the recommendation was wrong, not because the
threshold was miscomputed but because the matrix omitted the vaccine's own downside tail and because $L$ was taken
from the worst historical draw instead of from the distribution of pandemic severities. The chapter's rule is
sound about the boundary and silent about the entries, and the entries are where the loss was booked.

What the form licenses is therefore narrower than §2 implies: the threshold is a correct function of a complete
loss matrix, and no threshold argument can certify its own completeness. The check that would have caught 1976 is
a term-by-term one: every consequential harm that the action itself can cause belongs in $C\_{\mathrm{FP}}$ or in
$L\_{\mathrm{FN}}$, and an action whose adverse tail is unmeasured has an incomplete row, not a favourable ratio.

### When the threshold moves the probability it is compared against

The thesis of the chapter is that the threshold is a property of consequences and not of evidence, which assumes
the two are separable. They are not when the announced threshold changes the distribution of the state: a published
intervention rule invites behaviour that shifts $p$, and a published loss matrix invites the reclassification of
losses out of the expensive cell. In that regime $p$ is a function of $p^\ast$, the product $p\mkern3muL$ that the rule
compares is a fixed point rather than a datum, and the honest procedure is to solve for it, which a performative
model of the deployed system does explicitly, or to remove the discretion that
creates the loop by committing the rule before the exposure, which an append-only sealed record of the commitment
is for. Where neither is available, the correct report is that the threshold
is not applicable because its two arguments are not independent, and the decision must fall back on the
incommensurability procedure of the preceding section rather than on a number that quietly assumes its own
irrelevance.

## Exercises

1. Derive both thresholds symbolically for a protection cost $C$, avertable fraction $\eta$ and avoidable loss $L$, and for a false-alarm cost against a missed-event loss, and show which denominator each one carries (answer: $C/(\eta L)$ and $C\_{\mathrm{FP}}/(C\_{\mathrm{FP}} + L\_{\mathrm{FN}})$).
2. Say which rule a regulator should use when the protective act is unreliable, and why (answer: the efficiency form, because it discounts the loss by the reliability of the act).
3. Take a forecast that lies between the two thresholds and show that the two rules disagree about acting, naming the quantity that makes them disagree (answer: the avertable fraction $\eta$).
4. Construct a case where the threshold should not be applied at all and say what replaces it (answer: incommensurable loss; decide on the side that needs no comparison).

## Bridge

The cost–loss framing threshold is introduced in
[Chapter 32](32-the-question-before-the-data.md) §5 and paired here with the Pauker–Kassirer expected-regret
threshold of §2 and the expected value of perfect information of §4; the audit of limited-liability asymmetry and the Hand formula is
governed by the analysis of contractual liability; and the operator's bounded cognitive channel capacity
evaluated against alert streams is developed in
[Chapter 49](49-moral-luck-apt-agency-institutions.md). A threshold is written over a model, and the fit and error of that model are [Chapter 37](37-model-fit-and-error.md), *Model, Fit, and Error*.
