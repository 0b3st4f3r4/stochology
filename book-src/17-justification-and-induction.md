# Justification and Induction

<figure class="stoch-figure stoch-logo-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="800" height="132" viewBox="0 0 800 132" fill="none" role="img" aria-label="Stochology CH17 · SEVERITY — Induction, Corroboration, And Testing">
  <defs>
    <!-- Canonical Ring Gradient -->
    <linearGradient id="ch17-ring" x1="2" y1="78" x2="386" y2="78" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="38%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="68%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <!-- 1. Atmospheric Cosmic Field -->
    <radialGradient id="ch17-glow" cx="52%" cy="52%" r="50%">
      <stop offset="0%" stop-color="#fbbf24" stop-opacity="0.24"/>
      <stop offset="48%" stop-color="#38bdf8" stop-opacity="0.14"/>
      <stop offset="80%" stop-color="#e11d48" stop-opacity="0.08"/>
      <stop offset="100%" stop-color="#090e1a" stop-opacity="0"/>
    </radialGradient>
    <!-- 3D Spherical Node Gradients -->
    <radialGradient id="ch17-node-well" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="var(--logo-well-hi, var(--elevated-bg, #ffffff))"/>
      <stop offset="100%" stop-color="var(--logo-node-bg, #e2e8f0)"/>
    </radialGradient>
    <linearGradient id="ch17-ring-epi" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#7dd3fc"/>
      <stop offset="50%" stop-color="#2563eb"/>
      <stop offset="100%" stop-color="#1e3a8a"/>
    </linearGradient>
    <radialGradient id="ch17-core-epi" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#bae6fd"/>
      <stop offset="45%" stop-color="#3b82f6"/>
      <stop offset="100%" stop-color="#1d4ed8"/>
    </radialGradient>
    <linearGradient id="ch17-ring-vital" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#6ee7b7"/>
      <stop offset="50%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#065f46"/>
    </linearGradient>
    <radialGradient id="ch17-core-vital" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#a7f3d0"/>
      <stop offset="45%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#047857"/>
    </radialGradient>
    <linearGradient id="ch17-ring-gov" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#fda4af"/>
      <stop offset="50%" stop-color="#e11d48"/>
      <stop offset="100%" stop-color="#881337"/>
    </linearGradient>
    <radialGradient id="ch17-core-gov" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#fecdd3"/>
      <stop offset="45%" stop-color="#f43f5e"/>
      <stop offset="100%" stop-color="#be123c"/>
    </radialGradient>
    <!-- Stókhos Reticle Gradients -->
    <radialGradient id="ch17-pupil" cx="45%" cy="42%" r="58%">
      <stop offset="0%" stop-color="#020617"/>
      <stop offset="68%" stop-color="#0f172a"/>
      <stop offset="100%" stop-color="#451a03"/>
    </radialGradient>
    <linearGradient id="ch17-gold-mira" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#fef08a"/>
      <stop offset="45%" stop-color="#fbbf24"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <radialGradient id="ch17-gold-core" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#ffffff"/>
      <stop offset="35%" stop-color="#fde047"/>
      <stop offset="100%" stop-color="#d97706"/>
    </radialGradient>
    <!-- Ink curve gradient -->
    <linearGradient id="ch17-ink-curve" x1="0%" y1="0%" x2="100%" y2="0%">
      <stop offset="0%" stop-color="var(--logo-ink, #0f172a)"/>
      <stop offset="50%" stop-color="var(--logo-sub, #1d4ed8)"/>
      <stop offset="100%" stop-color="var(--logo-ink, #0f172a)"/>
    </linearGradient>
    <linearGradient id="ch17-prior-path" x1="22" y1="84" x2="64" y2="56" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#1e3a8a"/>
      <stop offset="60%" stop-color="#2563eb"/>
      <stop offset="100%" stop-color="#38bdf8"/>
    </linearGradient>
    <linearGradient id="ch17-test-barrier" x1="64" y1="22" x2="64" y2="110" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#fde047"/>
      <stop offset="50%" stop-color="#f59e0b"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <linearGradient id="ch17-corrob-branch" x1="64" y1="56" x2="104" y2="28" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#10b981"/>
      <stop offset="50%" stop-color="#34d399"/>
      <stop offset="100%" stop-color="#a7f3d0"/>
    </linearGradient>
    <linearGradient id="ch17-refute-branch" x1="64" y1="56" x2="104" y2="94" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#e11d48"/>
      <stop offset="100%" stop-color="#881337" stop-opacity="0.4"/>
    </linearGradient>
  </defs>
  <g>
    <circle cx="64" cy="66" r="48" fill="url(#ch17-glow)"/>
    <circle cx="64" cy="66" r="42" stroke="var(--logo-ticks, #64748b)" stroke-width="1.2" stroke-dasharray="3 3" stroke-opacity="0.45"/>
    <path d="M 22 84 C 36 68, 48 58, 64 56" stroke="url(#ch17-prior-path)" stroke-width="3.4" stroke-linecap="round"/>
    <line x1="64" y1="22" x2="64" y2="110" stroke="url(#ch17-test-barrier)" stroke-width="3.0" stroke-dasharray="4 2.5"/>
    <path d="M 64 56 C 74 54, 85.72 46.28, 91.38 40.62" stroke="url(#ch17-corrob-branch)" stroke-width="3.2" stroke-linecap="round"/>
    <path d="M 90.60 46.38 L 92.79 39.21 L 85.62 41.40" stroke="url(#ch17-corrob-branch)" stroke-width="2.6" stroke-linecap="round" stroke-linejoin="round"/>
    <path d="M 64 56 C 78 62, 92 80, 104 94" stroke="url(#ch17-refute-branch)" stroke-width="2.6" stroke-dasharray="3 3" stroke-linecap="round"/>
    <circle cx="26" cy="80" r="8.5" fill="url(#ch17-node-well)" stroke="url(#ch17-ring-epi)" stroke-width="3.0"/>
    <circle cx="26" cy="80" r="3.8" fill="url(#ch17-core-epi)"/>
    <circle cx="100" cy="32" r="8.5" fill="url(#ch17-node-well)" stroke="url(#ch17-ring-vital)" stroke-width="3.0"/>
    <circle cx="100" cy="32" r="3.8" fill="url(#ch17-core-vital)"/>
    <g>
      <circle cx="64" cy="56" r="7.4" fill="var(--logo-node-bg, #e2e8f0)"/>
      <circle cx="64" cy="56" r="6.4" fill="url(#ch17-pupil)" stroke="url(#ch17-gold-mira)" stroke-width="2.4"/>
      <line x1="54.40" y1="56.00" x2="59.80" y2="56.00" stroke="url(#ch17-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="68.20" y1="56.00" x2="73.60" y2="56.00" stroke="url(#ch17-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="64.00" y1="46.40" x2="64.00" y2="51.80" stroke="url(#ch17-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <line x1="64.00" y1="60.20" x2="64.00" y2="65.60" stroke="url(#ch17-gold-mira)" stroke-width="2.4" stroke-linecap="round"/>
      <circle cx="64" cy="56" r="2.2" fill="url(#ch17-gold-core)"/>
    </g>
  </g>
  <g transform="translate(148, 0)">
    <text x="2" y="27" fill="var(--logo-sub, #1d4ed8)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.2">PART II · THE BASES · PHILOSOPHY</text>
    <text x="498" y="27" text-anchor="end" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.0">CH17 · SEVERITY</text>
    <text x="0" y="59" fill="var(--logo-ink, #0f172a)" font-family="system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif" font-size="18.5" font-weight="800" letter-spacing="0.95">INDUCTION, CORROBORATION, AND TESTING</text>
    <line x1="2" y1="78" x2="384" y2="78" stroke="url(#ch17-ring)" stroke-width="1.6" stroke-opacity="0.82"/>
    <circle cx="400" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.0"/>
    <circle cx="400" cy="78" r="1.6" fill="var(--logo-blue, #2563eb)"/>
    <circle cx="415" cy="78" r="6.2" stroke="var(--logo-gold, #f59e0b)" stroke-width="1.5" stroke-dasharray="2.2 1.6"/>
    <circle cx="415" cy="78" r="4.0" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.2"/>
    <circle cx="415" cy="78" r="1.8" fill="var(--logo-gold, #f59e0b)"/>
    <circle cx="430" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-blue, #2563eb)" stroke-width="2.0"/>
    <circle cx="430" cy="78" r="1.6" fill="var(--logo-blue, #2563eb)"/>
    <circle cx="445" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="460" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="475" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="490" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <text x="2" y="104" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.8" font-weight="600" letter-spacing="0.65">HUME'S HORIZON · POPPERIAN REFUTABILITY · MAYO SEVERE TESTING CRITERION</text>
  </g>
</svg>
</figure>

> Second chapter of the philosophical block in Part II. The subject is the oldest problem in the
> theory of evidence: on what basis does the past constrain the future?

## By the end of this chapter you can

1. State the problem of induction and say precisely which premise it denies.
2. Distinguish deductive from inductive support, and say why the second cannot be made deductive.
3. Explain what a probability model does and does not add to an inductive inference.

## 1. Deduction and the Limits of Formal Truth Transfer

Consider a verification team that proves a reserve-allocation routine satisfies every algebraic invariant and can
never divide by zero. Two weeks after deployment the reserve still hits zero. Not because the theorem had a bug, but
because the arrival-rate premise in the specification was half the winter demand. The proof transferred truth
flawlessly from premise to conclusion. The premise itself was never deductively guaranteed by the world.

A deductive argument ($\Gamma \vdash \varphi$) is structurally truth-preserving: if all premises in $\Gamma$ are
true, the conclusion $\varphi$ cannot be false. Modus ponens ($P \implies Q$, $P$, therefore $Q$) and mathematical
proofs operate inside that domain. Deduction is non-ampliative: it transfers truth without generating new empirical
contact. A valid syllogism makes explicit what the premises already contained, and it cannot verify whether those
premises correspond to the physical world.

One degenerate limit narrows the gap. Take a **closed, completely enumerated static domain** with zero measurement
error, so the sampling fraction is $f = n/N = 1$. Empirical generalisation collapses into finite deduction there.
Outside that closed limit, empirical inquiry must establish its premises through observation, measurement, and
interaction, and that step is irreducibly non-deductive.

## 2. Hume's Problem and Goodman's New Riddle of Induction

Picture a consumer credit model trained across five years of low-interest expansion. Every borrower with high
short-term rollover velocity defaulted rarely, because cheap refinancing was always available. On policy cutoff date
$T$ the central bank raises base rates sharply, and the same feature becomes a predictor of insolvency. Up to $T$,
"high rollover predicts solvency" and "high rollover predicts solvency before $T$ and default after $T$" fitted every
historical row identically.

David Hume (1739/1748) formulated the classical **problem of induction**: inductive generalisations of the form
*all observed instances of $A$ have been $B$, therefore the next instance of $A$ will be $B$* possess no deductive
guarantee. The denial of the conclusion is never a logical contradiction: "the sun will not rise tomorrow" violates
no rule of logic. Nor can induction be justified empirically by citing its historical success. To argue that
induction is reliable because it has worked in the past is to argue inductively for induction, a *petitio principii*
(vicious circularity).

The norm behind that gap is older than its statistics: it is "wrong always, everywhere, and for anyone, to believe anything upon insufficient evidence," so an induction that cannot size its support is defective as inquiry (Clifford 1879).

Nelson Goodman (1955) deepened the scepticism with the **new riddle of induction**. Define the
predicate *grue*: an object is grue if it is examined before future date $T$ and found to be green, or not examined
before $T$ and found to be blue:

$$\mathrm{Grue}(x) \iff \bigl(\mathrm{ExaminedBefore}(x, T) \land \mathrm{Green}(x)\bigr) \lor \bigl(\neg\mathrm{ExaminedBefore}(x, T) \land \mathrm{Blue}(x)\bigr).$$

All emeralds examined up to today ($t < T$) are green, so all examined emeralds are also grue. If past instances
confirm generalisations by simple enumeration, the archive confirms both *all emeralds are green* and *all emeralds
are grue*. At $T$ the two yield diametrically opposed predictions: the first says the next emerald will be green, the
second says it will be blue. Goodman proved that inductive inference is impossible without **projectible
predicates**. We prefer *green* over *grue* because *green* is anchored in invariant physical mechanisms, not because
formal logic dictates the preference. When institutions build statistical models, their choice of features embeds
entrenched predicates. That choice determines which futures the model can even conceive.

## 3. Peirce's Inferential Triad: Abduction, Deduction, Induction

Imagine an epidemiological response team investigating a cluster of acute respiratory admissions across three
riverfront clinics. Before any regression, a clinician notices that all affected patients work near a cold-storage
freight depot and conjectures an aerosolised coolant leak ($A$). That conjecture is an explanatory leap, and it
introduces new causal structure. The team then traces what *must* follow if it holds: clinics downwind of the
afternoon breeze registering secondary admissions within forty-eight hours, and upwind clinics registering none. It
dispatches mobile samplers to both corridors to give the prediction a chance to fail.

Charles Sanders Peirce dismantled the binary opposition between deduction and induction with a tripartite architecture of scientific reasoning (Peirce 1899/1931):

1. **Abduction** (Hypothesis formation): Confronted with an anomalous or surprising observation $C$, the mind conjectures: *if hypothesis $A$ were true, $C$ would follow as a matter of course; hence, there is reason to suspect that $A$ is true*. Abduction is the only logical operation that introduces genuinely novel explanatory ideas.
2. **Deduction** (Consequence tracing): Traces the necessary, observable consequences of hypothesis $A$, deriving specific empirical predictions: *if $A$ holds, then under experimental conditions $E$, outcome $O$ must occur*.
3. **Induction** (Severe testing): Submits those deduced consequences to severe empirical confrontation, evaluating whether the hypothesis survives genuine opportunities for refutation.

Without abduction, induction is blind data mining; without deduction, abduction is ungrounded speculation; without
induction, deduction is an unverified tautology.

Inductive margin sizing has an explicit **closed-population counterexample**. An auditor inspects every voucher of a
closed batch, so $f = n/N = 1$. The Keynesian evidential weight already stands at $V = n/(n + n\_0)$ and sampling more
rows is impossible by construction; only measurement error remains.

Project across open, unrandomised populations and inductive transport fails differently. In the National Supported
Work trial ([Numbers, Order, and Bounds](10-numbers-order-bounds.md)) the randomised trial measures a positive average
treatment effect. The naive observational comparison against a population control sample fails to reproduce it and
reverses its sign, and no increase in observational sample size cures that selection error.

## 4. Agnotology and the Manufacture of Strategic Doubt

In the social and political sphere, inductive uncertainty is frequently weaponized. Robert Proctor and Londa Schiebinger (2008) founded **agnotology**, the study of culturally and strategically produced ignorance. The archetypal expression of manufactured doubt is the **tobacco strategy**, encapsulated in an infamous 1969 tobacco industry internal memorandum: *"Doubt is our product, since it is the best means of competing with the 'body of fact' that exists in the minds of the general public."*

An industry or institution facing empirical evidence that its profitable operations impose catastrophic health,
ecological, or systemic risks rarely denies the science outright. It exploits Humean fallibilism through a small
repertoire of bad-faith postures: strategic self-deception about what the evidence implies, bureaucratic insouciance
about the consequences, and outright denial of the local finding:

- It demands impossible, infinite standards of proof ("the causal mechanism is not 100% resolved");
- It funds redundant studies to manufacture false controversies and statistical noise;
- It uses inductive skepticism as an agnotological alibi to delay regulation while extracting private upside and shifting irreversible downside onto the public.

Oliver Cromwell's rule warns against assigning prior probability zero or one to hypotheses
($\mathbb{P}(H) \in (0, 1)$). In institutional governance, agnotology exploits this openness, claiming that because
uncertainty is never zero, precautionary action is never justified. Two operators detect the manoeuvre.

The first is a **premise-rigging index**
$\mathrm{PRI} = \max\bigl(c\_{\mathrm{epi}},\mkern3mu c\_{\mathrm{spill}}\bigr) = \max\bigl(1 - w\_{\mathrm{disp}}/w\_{\mathrm{true}},\mkern3mu 1 - D\_{\mathrm{disp}}/D\_{\mathrm{true}}\bigr) \in [0, 1]$,
which compares the epistemic width ($0 \le w\_{\mathrm{disp}} \le w\_{\mathrm{true}}$, $w\_{\mathrm{true}} > 0$) and the spillover damage ($0 \le D\_{\mathrm{disp}} \le D\_{\mathrm{true}}$, $D\_{\mathrm{true}} > 0$, with $c\_{\mathrm{spill}} = 0$ if $D\_{\mathrm{true}} = 0$) an inquiry declares against the width and the damage
actually present. A rigged framing compresses either or both, so the index rises toward one as the declared problem narrows
([Inquiry Openness, Severity, and the Multiverse](43-inquiry-openness-multiverse.md)).

The second is terminal evidential value $E\_T$, penalised by $E\_T / (k + 1)$ for $k$ post-hoc adjustments
(Lakatos 1970, developed in [Philosophy of Science](18-philosophy-of-science.md) and
[Underdetermination, Partial Identification, and Severe Testing](44-underdetermination-partial-identification.md)).
When vital survival is at stake, action cannot wait for deductive certainty.

## 5. Consequences for Epistemic Margin Sizing

Consider a reserve committee that refuses to fund a protective buffer until analysts deliver "definitive
statistical proof" of a regime shift. A rigorous institution handles induction without pretending to possess proof. It
sizes an explicit **epistemic margin** via the Keynesian weight of argument (Keynes 1921):

$$V(n\_{\mathrm{eff}}; n\_0) = \frac{n\_{\mathrm{eff}}}{n\_{\mathrm{eff}} + n\_0} \in [0, 1), \qquad n\_{\mathrm{eff}} \ge 0,\mkern5mu n\_0 := n\_{\mathrm{req}}(\alpha, \beta, p^\star) = \left(\frac{z\_{1-\alpha} + z\_{1-\beta}}{\varepsilon\_{\mathrm{rel}}}\right)^2 \frac{1 - p^\star}{p^\star} > 0,$$

where $n\_0$ is anchored symbolically to the effective sample required at error tolerances $\alpha + \beta < 1$ and relative resolution $\varepsilon\_{\mathrm{rel}} > 0$ around the cost–loss threshold $p^\star = C/(\eta L) \in (0, 1)$, so $(1 - p^\star)/p^\star = \eta L/C - 1$. Two limit cases govern $V(n\_{\mathrm{eff}}; n\_0)$, one at each end.

- **No evidence ($n\_{\mathrm{eff}} \to 0^+$):** $V(0; n\_0) = 0$. No inductive weight has been earned, and the epistemic margin score collapses to zero.
- **Saturation ($n\_{\mathrm{eff}} \to \infty$ or $n\_0 \to 0^+$):** $V(n\_{\mathrm{eff}}; n\_0) \to 1^-$. Hyperbolic saturation prevents any finite archive from reaching absolute deductive certainty ($V = 1$). If a regulator doubles the reference standard $n\_0$ at fixed $n\_{\mathrm{eff}}$ (or raises the stake ratio $\eta L/C$, lowering $p^\star$), $V$ falls, and the fall is largest where the evidence is thinnest: the same archive carries less weight against a more demanding question.

An institution therefore enforces three rules:

1. It acknowledges the structural underdetermination of models by finite archives;
2. It audits the framing of questions to prevent agnotological suppression of inconvenient hypotheses;
3. It sizes physical vital reserves against the widest plausible inductive bound, refusing to let manufactured doubt paralyze precautionary defense.

### What a decade of data buys

The weight saturates hyperbolically, so a decade of data does not buy a decade's worth of certainty. Multiplying the
effective sample by ten moves $V$ from $n/(n + n\_0)$ to $10n/(10n + n\_0)$, which at a canonical reference scale is a
large gain on the first decade and a small one on the second, and another factor of ten adds only the residue. Three
ad hoc adjustments to a model cost a fixed logarithmic amount of the same currency, because the penalty $E\_T/(k + 1)$
is a division and the deciban scale is a logarithm: an evidential value of $20$ or of $200$, divided fourfold, loses
the same number of decibans. Justify by design, spend on adjustments, and notice which side of the ledger saturates
first.

The ledger saturates because the weight is a ratio, not a count. It measures how much uncertainty has been removed,
and that quantity has a ceiling of one that belongs to the question rather than to the sample. Stating the weight as
the value of a body of evidence is older than any of this data (Keynes 1921). Because the scale is logarithmic, the
same three adjustments cost the same number of decibans against an evidential value of $200$ as against $20$.

### When the generator changes

The thesis fails when the series is not stationary, because no accumulated weight carries across a change of
generator. The condition is a detected regime break. The procedure is to restart the accumulation at the break and to
report the weight since the break only. Any long equity series shows why this is not a technicality: the average
across parallel worlds and the average along one path are different numbers, so induction over one path is induction
over a different object from the one the ensemble describes.

## Bridge

[The Question Before the Data](32-the-question-before-the-data.md) formalises inquiry design, while
[Inquiry Openness, Severity, and the Multiverse](43-inquiry-openness-multiverse.md) constructs mathematical tests for
agnotological framing bias and Cromwell violations. The epistemic margin computes framing penalties, Cromwell
boundary enforcement, and Type III inquiry distortions across competing hypotheses. The Keynesian weight of argument
$V = n\_{\mathrm{eff}}/(n\_{\mathrm{eff}} + n\_0)$ is evaluated against the explicitly declared reference scale
$n\_0 := n\_{\mathrm{req}}(\alpha, \beta, p^\star)$ anchored to the decision's cost–loss threshold $p^\star = C/(\eta L)$.

[Philosophy of Science](18-philosophy-of-science.md) turns justification into a public test:
falsification, Duhem–Quine underdetermination, Lakatosian research programmes, and Mayo's severe
testing.
