# Thermodynamics and Entropy

<figure class="stoch-figure stoch-logo-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="660" height="132" viewBox="0 0 660 132" fill="none" role="img" aria-label="Stochology CH22 · ENTROPY — Thermodynamics And Irreversibility">
  <defs>
    <linearGradient id="ch22-ring" x1="2" y1="78" x2="386" y2="78" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)"/>
      <stop offset="38%" stop-color="var(--logo-blue, #2563eb)"/>
      <stop offset="68%" stop-color="var(--logo-gold, #f59e0b)"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)"/>
    </linearGradient>
    <radialGradient id="ch22-glow" cx="50%" cy="50%" r="50%">
      <stop offset="0%" stop-color="#34d399" stop-opacity="0.25"/>
      <stop offset="50%" stop-color="#10b981" stop-opacity="0.14"/>
      <stop offset="82%" stop-color="#fbbf24" stop-opacity="0.08"/>
      <stop offset="100%" stop-color="#090e1a" stop-opacity="0"/>
    </radialGradient>
    <linearGradient id="ch22-triad-stroke" x1="18" y1="24" x2="110" y2="104" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.85"/>
      <stop offset="50%" stop-color="var(--logo-gold, #f59e0b)" stop-opacity="0.90"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.85"/>
    </linearGradient>
    <linearGradient id="ch22-fill" x1="22" y1="30" x2="106" y2="98" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-green, #10b981)" stop-opacity="0.26"/>
      <stop offset="48%" stop-color="var(--logo-blue, #2563eb)" stop-opacity="0.28"/>
      <stop offset="100%" stop-color="var(--logo-crimson, #e11d48)" stop-opacity="0.26"/>
    </linearGradient>
    <linearGradient id="ch22-ink-curve" x1="20" y1="66" x2="108" y2="66" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="var(--logo-ink, #0f172a)"/>
      <stop offset="50%" stop-color="var(--logo-sub, #1d4ed8)"/>
      <stop offset="100%" stop-color="var(--logo-ink, #0f172a)"/>
    </linearGradient>
    <radialGradient id="ch22-pupil" cx="45%" cy="42%" r="58%">
      <stop offset="0%" stop-color="#020617"/>
      <stop offset="68%" stop-color="#0f172a"/>
      <stop offset="100%" stop-color="#451a03"/>
    </radialGradient>
    <linearGradient id="ch22-gold-mira" x1="54" y1="48" x2="84" y2="82" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#fef08a"/>
      <stop offset="45%" stop-color="#fbbf24"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
    <radialGradient id="ch22-gold-core" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#ffffff"/>
      <stop offset="35%" stop-color="#fde047"/>
      <stop offset="100%" stop-color="#d97706"/>
    </radialGradient>
    <radialGradient id="ch22-node-well" cx="38%" cy="35%" r="65%">
      <stop offset="0%" stop-color="var(--logo-well-hi, var(--elevated-bg, #ffffff))"/>
      <stop offset="100%" stop-color="var(--logo-node-bg, #e2e8f0)"/>
    </radialGradient>
    <linearGradient id="ch22-ring-epi" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#7dd3fc"/>
      <stop offset="50%" stop-color="#2563eb"/>
      <stop offset="100%" stop-color="#1e3a8a"/>
    </linearGradient>
    <radialGradient id="ch22-core-epi" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#bae6fd"/>
      <stop offset="45%" stop-color="#3b82f6"/>
      <stop offset="100%" stop-color="#1d4ed8"/>
    </radialGradient>
    <linearGradient id="ch22-ring-vital" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#6ee7b7"/>
      <stop offset="50%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#065f46"/>
    </linearGradient>
    <radialGradient id="ch22-core-vital" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#a7f3d0"/>
      <stop offset="45%" stop-color="#10b981"/>
      <stop offset="100%" stop-color="#047857"/>
    </radialGradient>
    <linearGradient id="ch22-ring-gov" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#fda4af"/>
      <stop offset="50%" stop-color="#e11d48"/>
      <stop offset="100%" stop-color="#881337"/>
    </linearGradient>
    <radialGradient id="ch22-core-gov" cx="35%" cy="35%" r="65%">
      <stop offset="0%" stop-color="#fecdd3"/>
      <stop offset="45%" stop-color="#f43f5e"/>
      <stop offset="100%" stop-color="#be123c"/>
    </radialGradient>
  </defs>
  <g>
    <circle cx="64" cy="66" r="48" fill="url(#ch22-glow)"/>
    <circle cx="64" cy="66" r="42" stroke="var(--logo-ticks, #64748b)" stroke-width="1.2" stroke-dasharray="3 3" stroke-opacity="0.45"/>
    <path d="M 32 36 Q 68 38 96 64 Q 62 94 32 82 Z" fill="url(#ch22-fill)" stroke="var(--logo-green, #10b981)" stroke-width="2.6" stroke-linejoin="round"/>
    <line x1="64" y1="62" x2="104" y2="96" stroke="var(--logo-crimson, #e11d48)" stroke-width="2.6" stroke-linecap="round"/>
    <circle cx="32.00" cy="36.00" r="6.55" fill="url(#ch22-node-well)" stroke="url(#ch22-ring-epi)" stroke-width="2.42"/>
    <circle cx="32.00" cy="36.00" r="2.81" fill="url(#ch22-core-epi)"/>
    <circle cx="96.00" cy="64.00" r="6.55" fill="url(#ch22-node-well)" stroke="url(#ch22-ring-vital)" stroke-width="2.42"/>
    <circle cx="96.00" cy="64.00" r="2.81" fill="url(#ch22-core-vital)"/>
    <g>
      <circle cx="60.00" cy="64.00" r="6.66" fill="var(--logo-node-bg, #e2e8f0)"/>
      <circle cx="60.00" cy="64.00" r="5.76" fill="url(#ch22-pupil)" stroke="url(#ch22-gold-mira)" stroke-width="2.16"/>
      <line x1="51.36" y1="64.00" x2="56.22" y2="64.00" stroke="url(#ch22-gold-mira)" stroke-width="2.16" stroke-linecap="round"/>
      <line x1="63.78" y1="64.00" x2="68.64" y2="64.00" stroke="url(#ch22-gold-mira)" stroke-width="2.16" stroke-linecap="round"/>
      <line x1="60.00" y1="55.36" x2="60.00" y2="60.22" stroke="url(#ch22-gold-mira)" stroke-width="2.16" stroke-linecap="round"/>
      <line x1="60.00" y1="67.78" x2="60.00" y2="72.64" stroke="url(#ch22-gold-mira)" stroke-width="2.16" stroke-linecap="round"/>
      <circle cx="60.00" cy="64.00" r="1.98" fill="url(#ch22-gold-core)"/>
    </g>
  </g>
  <g transform="translate(148, 0)">
    <text x="2" y="27" fill="var(--logo-sub, #1d4ed8)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.2">PART II · THE BASES · NATURAL SCIENCE</text>
    <text x="498" y="27" text-anchor="end" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.5" font-weight="700" letter-spacing="1.0">CH22 · ENTROPY</text>
    <text x="0" y="59" fill="var(--logo-ink, #0f172a)" font-family="system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif" font-size="20.2" font-weight="800" letter-spacing="1.15">THERMODYNAMICS AND IRREVERSIBILITY</text>
    <line x1="2" y1="78" x2="384" y2="78" stroke="url(#ch22-ring)" stroke-width="1.6" stroke-opacity="0.82"/>
    <circle cx="400" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="415" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="430" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="445" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-green, #10b981)" stroke-width="2.0"/>
    <circle cx="445" cy="78" r="1.6" fill="var(--logo-green, #10b981)"/>
    <circle cx="460" cy="78" r="6.2" stroke="var(--logo-gold, #f59e0b)" stroke-width="1.5" stroke-dasharray="2.2 1.6"/>
    <circle cx="460" cy="78" r="4.0" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-green, #10b981)" stroke-width="2.2"/>
    <circle cx="460" cy="78" r="1.8" fill="var(--logo-gold, #f59e0b)"/>
    <circle cx="475" cy="78" r="2.0" fill="var(--logo-ticks, #64748b)" fill-opacity="0.42"/>
    <circle cx="490" cy="78" r="3.8" fill="var(--logo-node-bg, #e2e8f0)" stroke="var(--logo-crimson, #e11d48)" stroke-width="2.0"/>
    <circle cx="490" cy="78" r="1.6" fill="var(--logo-crimson, #e11d48)"/>
    <text x="2" y="104" fill="var(--logo-meta, #475569)" font-family="ui-monospace, 'Cascadia Code', monospace" font-size="10.8" font-weight="600" letter-spacing="0.65">SECOND LAW DISSIPATION · FREE-ENERGY WORK · MAINTENANCE COST OF ORDER</text>
  </g>
  </svg>
</figure>

> Second chapter of the natural-science block in Part II. The subject is why some processes run in one
> direction only, and what that has to do with a reserve.

## By the end of this chapter you can

1. State the first and second laws and say what each forbids.
2. Explain entropy as a counting statement rather than as a substance, and say what "disorder" gets
   wrong.
3. Say why irreversibility is a constraint on institutions and not only on engines.

## 1. The First Law: Conservation and Flow Balance

Consider a water utility or a trauma hospital whose budget report celebrates a cut in preventative maintenance
while pumping volume or admissions remain unchanged. On a spreadsheet a deleted allocation creates a surplus. In a
physical plant no work vanishes. The **First Law of Thermodynamics** states the conservation: within an isolated
system energy is converted among thermal heat $Q \in \mathbb{R}$, work $W \in \mathbb{R}$ and internal energy
$U \in \mathbb{R}$, all in joules:

$$\Delta U = Q - W.$$

For an open control volume (a power station, a biological cell, a logistics hub) exchanging mass fluxes
$\dot{m}\_{\mathrm{in}}, \dot{m}\_{\mathrm{out}} \ge 0$ in $\text{kg/s}$ with specific enthalpy
$h = u + pv$ in $\text{J/kg}$, thermal power $\dot{Q}$ and shaft power $\dot{W}$ in watts, the balance is
instantaneous:

$$\frac{dU}{dt} = \dot{Q} - \dot{W} + \sum\_{\mathrm{in}} \dot{m}\_{\mathrm{in}} h\_{\mathrm{in}} - \sum\_{\mathrm{out}} \dot{m}\_{\mathrm{out}} h\_{\mathrm{out}}.$$

Not all conserved internal energy is available to perform ordered work. At constant ambient temperature $T > 0$ and
volume $V$ the maximum extractable work is bounded by the **Helmholtz free energy** $F = U - TS$, and at constant $T$
and pressure $p > 0$ by the **Gibbs free energy** $G = H - TS = U + pV - TS$, where $S \ge 0$ is the thermodynamic
entropy in $\text{J/K}$. The product $TS$ is degraded thermal energy locked in microscopic agitation.

An administration that demands constant operational output $\dot{W}$ while cutting maintenance inflows does not beat
the First Law. It cannibalises internal free energy ($\Delta F < 0$), converting equipment margin and frontline
stamina into unrecorded wear until a component ruptures.

## 2. The Second Law, Clausius, and the Microstate Count

Total energy is conserved, and its capacity to perform directed work degrades irreversibly in every real process.
Rudolf Clausius (1854/1865) formulated the **Second Law of Thermodynamics**: for any transformation
exchanging heat $dQ$ with a reservoir at absolute temperature $T > 0$, the entropy change $dS$ satisfies the Clausius
inequality:

$$dS = d\_e S + d\_i S \ge \frac{dQ}{T}, \qquad \Delta S\_{\mathrm{isolated}} = \int d\_i S \ge 0,$$

where $d\_e S = dQ / T$ is the reversible entropy flux across the boundary and $d\_i S \ge 0$ is the internal entropy
generated by irreversible friction, finite temperature gradients, or unrestrained expansion ($d\_i S > 0$ for every
finite-rate process).

Any cyclic heat engine drawing thermal energy $Q\_H > 0$ from a hot source at temperature $T\_H > 0$ must therefore
reject at least $Q\_C \ge Q\_H (T\_C / T\_H)$ into a cold sink at temperature $T\_C \in (0, T\_H]$. That bound caps the
fraction of heat convertible into work $W = Q\_H - Q\_C$ at the **Carnot efficiency limit**:

$$\eta\_{\max} = \frac{W\_{\max}}{Q\_H} = 1 - \frac{T\_C}{T\_H} \in [0, 1).$$

Two explicit cases illustrate the boundary:

1. **Sink as warm as the source ($T\_C \to T\_H^-$):** As $T\_C / T\_H \to 1$ the thermal gradient disappears and
$\eta\_{\max} \to 0$. Without a colder sink into which waste entropy $Q\_C / T\_C$ can be discharged, zero work can be
extracted regardless of how large $Q\_H$ is. A steam plant whose source is only twice as hot as its sink can convert
at most half of its heat into work, however hard the boiler is fired, and the other half leaves as waste heat.
2. **Sink at absolute zero ($T\_C \to 0^+$):** Complete conversion of heat into work ($\eta\_{\max} \to 1$) requires a
sink at $T\_C = 0\text{ K}$, which no finite sequence of operations can reach.

Ludwig Boltzmann (1877) grounded the Second Law in combinatorics:

$$S = k\_B \ln \Omega,$$

where $k\_B$ is Boltzmann's constant and $\Omega \ge 1$ is the **multiplicity**, the number of distinct microscopic
configurations (microstates) consistent with a given macroscopic state (temperature, pressure, volume).

To see why entropy is a counting statement rather than a mysterious fluid, consider $N$ distinguishable gas molecules
free to move between two equal halves of an insulated box. The box holds $2^N$ arrangements in all. The completely
segregated macrostate, with every molecule in the left chamber, is a single microstate:
$\Omega\_{\mathrm{segregated}} = \binom{N}{N} = 1$, so $S\_{\mathrm{segregated}} = k\_B \ln 1 = 0$. The balanced
macrostate, with half the molecules on each side, is $\Omega\_{\mathrm{eq}} = \binom{N}{N/2}$ microstates, which for
any macroscopic $N$ exceeds one by an exponential factor through Stirling's approximation. Unconstrained trajectories
migrate toward high entropy because balanced microstates outnumber segregated ones so vastly that no observation will
ever find the gas segregated.

For systems driven rapidly out of equilibrium between two states separated by Helmholtz free-energy difference
$\Delta F$ at inverse temperature $\beta = (k\_B T)^{-1}$, Christopher Jarzynski (1997) proved the exact
non-equilibrium work identity $\langle e^{-\beta W} \rangle = e^{-\beta \Delta F}$. The exponential function is
strictly convex, so Jensen's inequality $\langle e^{-\beta W} \rangle \ge e^{-\beta \langle W \rangle}$ (Jensen
1906) yields the irreversible **dissipated work** overhead immediately:

$$\langle W \rangle \ge \Delta F, \qquad \langle W\_{\mathrm{diss}} \rangle = \langle W \rangle - \Delta F = T \Delta\_i S \ge 0.$$

Driving a physical or organisational transition over a compressed deadline ($\Delta t \to 0$) increases internal
friction $\Delta\_i S > 0$, and the input work $\langle W \rangle$ must then exceed the reversible quasi-static
minimum $\Delta F$ strictly.

## 3. Ilya Prigogine and Dissipative Structures

Walk into an intensive-care operating theatre. Sterility and temperature stability are not static properties of the
walls; they exist only while positive-pressure HEPA air handlers, chilled-water coils, and sterilisation crews work
continuously against microbial and thermal diffusion. Living organisms, cities, electrical grids, and institutions are
**open systems operating far from thermodynamic equilibrium**, sustained by continuous throughputs of energy, matter,
and information.

Ilya Prigogine (1977 Nobel Prize in Chemistry) showed that in an open control volume, total entropy rate of change
$\dot{S} = dS/dt$ splits into internal irreversible generation $\dot{S}\_i \ge 0$ and boundary exchange
$\dot{S}\_e$:

$$\frac{dS}{dt} = \dot{S}\_e + \dot{S}\_i, \qquad \dot{S}\_e = -\frac{\dot{Q}\_{\mathrm{out}}}{T\_0},$$

where $\dot{Q}\_{\mathrm{out}} > 0$ is waste heat rejected into the surrounding environment at temperature
$T\_0 > 0$. Whenever an open system exports entropy faster than internal friction produces it
($\dot{S}\_e < -\dot{S}\_i$, so $\dot{S} < 0$), it maintains a low-entropy **dissipative structure** (Rayleigh–Bénard
convection cells, biological homeostasis, or a reliable emergency dispatch centre) by degrading high-grade free
energy into exported low-grade heat.

## 4. Administrative Dissipation and the Cost of Order

Nicholas Georgescu-Roegen (1971) extended the physical accounting to economic and institutional production, showing that industrial throughput irrevocably degrades low-entropy material and energetic
stocks into high-entropy waste:

1. **Order is a continuous maintenance flux, not a permanent capital stock**: A hospital, air-traffic control tower, or software system (whose audit logs are sealed by one-way cryptographic compression) decays toward operational friction the instant its maintenance flow is throttled.
2. **The bottleneck is replenishment time, not the Landauer bit floor**: A data centre of power $P$ rejecting heat into ambient air at temperature $T\_0$ exports entropy at the rate $\dot{S}\_{\mathrm{out}} = P/T\_0$, and its theoretical Landauer erasure ceiling is $P/(k\_B T\_0 \ln 2)$ bits per second. Real digital machinery sits many orders of magnitude below that ceiling, so the bit floor never binds. What breaks an institution under lean efficiency cuts is the throttling of physical repair, spare capacity, and human recovery time, never the joule cost of flipping bits.
3. **Asymmetric rebuilding times**: Depleting a groundwater aquifer or burning out an experienced trauma nursing team takes weeks; reconstructing that low-entropy reserve requires years of sustained investment.

## 5. The Physical Arrow of Time and the Vital Margin

Thermodynamics establishes that time has an asymmetric arrow: once an unbuffered system crosses an absorbing ruin
boundary ($R\_t \le 0$), it cannot borrow future average surpluses to undo physical collapse.

In a metered electricity record the asymmetry appears as a shape rather than as a theorem. Load over a year is
concentrated far below its maximum: the median interval sits at a small fraction of the annual peak, the upper
percentile sits well above the median, and a summer heatwave drives demand past the percentile a planner would call
extreme. That excursion is irreversible in the only sense that matters operationally, because the megawatts were
consumed and cannot be recalled. Sizing generation tightly against a high load quantile drives utilisation $\rho$
toward one, where the queueing delay and the depletion time to the ruin boundary diverge. Provisioning instead
against a multiple of the annual maximum holds $\rho$ at a level where autonomous survival time exceeds recovery
time, $\mathrm{TTS}\_{\mathrm{survival}} \ge \mathrm{TTR}\_{\mathrm{recovery}}$. Once a reserve crosses zero the
state is absorbing: the trajectory stays ruined for every remaining interval.

### Order is a flow, not a state

Order is a flow, not a state. A watt rejected at ambient temperature $T\_0$ carries $1/T\_0$ watts per kelvin out of the building, and
over a year that flux is enormous compared with the Landauer cost of erasing a bit: the thermodynamic erasure
ceiling for any realistic power draw sits many orders of magnitude above what the machinery actually does. The
binding constraint on an institution is how fast negentropy arrives, which is a question about time.

And time has an arrow. Demand spikes above the extreme percentile and does not un-spike. That asymmetry is the whole
institutional cost: the entropy of a schedule can be reversed on paper and not in the plant.

The thermodynamics cited above is the classical statement, and its unit is joules per kelvin rather than anything
organisational (Clausius 1865; Boltzmann 1877). A claim about dissipation must therefore name a heat flow and a
temperature, or call itself an analogy.

### When the analogy stops being measurable

The thesis fails when "administrative entropy" cannot be expressed in physical units, because there is then no
$\dot Q$ and no $T$ to divide. Keep the analogy as a heuristic, model the decision in declared work-flow units, and
never present it as proof. A dissipation metaphor that cannot be evaluated is a sentence with no referent, while
entropy does not decrease in a closed system and that statement is quantitative before it is rhetorical
(Clausius 1865).

## Exercises

1. Derive the entropy export rate $\dot{S}\_{\mathrm{out}} = P/T\_0$ for a plant of power $P$ rejecting heat into ambient air, and show which of the two quantities an efficiency programme can actually move.
2. Derive the Landauer bound $k\_B T \ln 2$ per erased bit and show why a register of any feasible size still cannot make the bit floor the binding constraint on a computation.
3. Show a case in which holding a reserve is worse than paying a continuous flow (answer: an abundant source with a linear cost, where a large reserve becomes idle capital).
4. State when the entropic analogy stops holding and what replaces it (answer: when no heat flow and no temperature are defined; use declared work flow as the analogy).

## Bridge

[Irreversible Time, Non-Ergodicity, and the Vital Margin](47-irreversible-time-vital-margin.md) translates the
thermodynamic arrow of time into the non-ergodicity of financial and operational survival paths. The vital
margin is the corresponding defense: it asks whether the rate at which a reserve depletes stays bounded inside
the recovery capacity of the system that depends on it.

[Statistical Physics and the Arrow of Time](23-statistical-physics-and-time.md) explains how that arrow
emerges from coarse-graining, and why the ergodic hypothesis fails exactly where survival is at stake.
