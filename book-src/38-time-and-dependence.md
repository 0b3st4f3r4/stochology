# Time and Dependence

> Foundation chapter of Part III. It presupposes [Chapter 12](12-derivatives-and-convexity.md)
> and [Chapter 23](23-statistical-physics-and-time.md). Its formal content is worked out in
> [Chapter 47](47-irreversible-time-vital-margin.md).

## By the end of this chapter you can

1. Distinguish an average over many parallel worlds from an average over one historical path.
2. Explain why a positive expected return can still produce a ruined trajectory.
3. Say what a queue does as utilisation approaches its ceiling, and why the delay is not linear.

## 1. Two averages and the breakdown of ergodicity

The distinction between statistical averages originates in the foundations of thermodynamics:
the **ensemble average** averages a physical property across infinitely many non-interacting parallel systems
at a single instant; the **time average** tracks a single system continuously as it evolves forward through
historical time. The two averages coincide if and only if the underlying stochastic process is **ergodic**.

The central reality of management, engineering, and epidemiology is that living systems, critical infrastructure,
and economic institutions are **strictly non-ergodic**. An institution does not possess a thousand parallel
copies among which it can distribute its losses; it has one runway, one electrical grid, one balance sheet,
and one historical path. The state it occupies at timestamp $t+1$ is physically dependent upon the sequence of
states it traversed to arrive there. For any non-ergodic system, planning on the basis of ensemble averages is
an invitation to sudden ruin.

## 2. Ruin as an absorbing state

When a stochastic dynamic contains an **absorbing barrier** — a boundary state from which no subsequent
transition is physically possible (e.g., biological death, insolvency, physical core meltdown) — classical
expected-value optimization fails catastrophically.

Consider a multiplicative gambling game or financial portfolio with a positive expected return across parallel
universes. The expectation is an ensemble calculation: it sums the payoffs across all hypothetical paths,
including those that struck the zero barrier and terminated. But a real-world operator does not experience the
ensemble; the operator experiences the time average of a single realization. The moment a path strikes zero,
multiplicative growth ends permanently. The remaining favorable outcomes in other branches of the probability
tree are completely inaccessible.

Expected value optimization routinely prescribes strategies that assign a positive expected payoff to trajectories
that possess an absorbing probability of unity. The rational objective for an entity traversing irreversible
time is not the maximization of expected wealth, but the **maximization of long-term survival probability**,
prioritizing the maintenance of an unbroken buffer between current operations and the absorbing ruin barrier.

## 3. Itô calculus and the variance drain

In continuous-time compounding processes, the divergence between ensemble drift and time-average growth is
governed by Itô's lemma. Consider an asset or resource reserve $S_t$ following geometric Brownian motion:

$$dS_t = \mu S_t\, dt + \sigma S_t\, dW_t \qquad \text{\texttt{[Assumed]}}.$$

Applying Itô's lemma to the logarithmic trajectory $X_t = \ln S_t$ yields:

$$d(\ln S_t) = \left(\mu - \frac{\sigma^2}{2}\right) dt + \sigma\, dW_t \qquad \text{\texttt{[Assumed]}}.$$

Integrating over time and invoking the strong law of large numbers for Brownian motion demonstrates that the
almost-sure time-average growth rate is:

$$g = \lim_{t \to \infty} \frac{1}{t} \ln\left(\frac{S_t}{S_0}\right) = \mu - \frac{\sigma^2}{2} \qquad \text{\texttt{[Assumed]}}.$$

The term $\frac{1}{2}\sigma^2$ is the **Itô variance drain**. While the ensemble expectation $\mathbb{E}[S_t] = S_0 e^{\mu t}$
grows at rate $\mu$, any individual sample path grows at the substantially lower rate $\mu - \frac{1}{2}\sigma^2$.
If volatility is sufficiently elevated ($\sigma^2 > 2\mu$), the system suffers negative growth ($g < 0$) and
collapses to zero with probability 1, even while its arithmetic expectation diverges to positive infinity.

In capital allocation, John Kelly (1956) showed that when leveraging a bet with fraction $f$, growth follows
$g(f) = \mu f - \frac{1}{2}\sigma^2 f^2$. The optimal growth fraction is $f^* = \mu/\sigma^2$. Beyond the
**ruin boundary** $f_{\mathrm{ruin}} = 2f^*$, the variance drain overwhelms drift, guaranteeing long-run extinction.
Furthermore, in real applications where $\mu$ and $\sigma$ are estimated with sampling noise, the **Baker-McHale
Kelly shrinkage** mandates scaling $f^*$ downward, because parameter uncertainty inflates the risk of over-betting
into the absorbing zone.

## 4. Congestion and the Kingman hyperbola

Queuing systems display a non-linear temporal failure mode that mirrors the absorbing barrier: as operational
utilization approaches capacity, delay does not grow linearly; it diverges hyperbolically.

In 1961, John Kingman proved that for a general single-server queue ($G/G/1$), the expected waiting time in
queue $\mathbb{E}[W_q]$ under heavy traffic satisfies:

$$\mathbb{E}[W_q] \approx \left(\frac{\rho}{1 - \rho}\right) \left(\frac{c_a^2 + c_s^2}{2}\right) \frac{1}{\mu} \qquad \text{\texttt{[Assumed]}},$$

where $\rho = \lambda/\mu$ is capacity utilization, $c_a$ is the coefficient of variation of inter-arrival times,
and $c_s$ is the coefficient of variation of service times.

```
Expected
Waiting
Time E[Wq]
  ^
  |                                        *
  |                                       *
  |                                      *   <- Hyperbolic Divergence:
  |                                     *       rho / (1 - rho)
  |                                    *
  |                                  *
  |                           * * *
  |                 * * * * *
  +---------------------------------------------> Utilization rho
  0               0.5               0.8   0.95  1.0
```

The term $\frac{\rho}{1 - \rho}$ represents the **Kingman hyperbola**. Sizing an emergency department, a highway,
or a compute cluster at $95\%$ utilization ($\rho = 0.95$) multiplies waiting time by a factor of
$0.95 / 0.05 = 19$. Sizing at $99\%$ multiplies it by $99$. Furthermore, variability enters multiplicatively:
bursty arrivals ($c_a \gg 1$) amplify congestion delay across all utilization levels. Operating a critical system
near $100\%$ efficiency is not lean optimization; it is an engineered guarantee of catastrophic gridlock upon
the arrival of the first cluster of shocks.

## 5. Vital normativity, resilience basins, and the master survival inequality

In *The Normal and the Pathological*, Georges Canguilhem established that biological life is defined by **vital
normativity**: health is not passive conformity to a statistical average, but the organic capacity to tolerate
environmental perturbations, absorb trauma, and establish new functional norms. In ecology, C. S. Holling formalized
this as the width of the **resilience basin**: the maximum displacement a system can endure without flipping into
an alternate, degraded regime.

Stochology synthesizes these temporal insights into a single operational comparison: the **Master Survival
Inequality**. Let $\mathrm{TTS}_{\mathrm{survival}}$ be the **Time to Survive** — the duration an institution can
maintain core operations using its existing physical, vital, and financial reserves under emergency depletion.
Let $\mathrm{TTR}_{\mathrm{recovery}}$ be the **Time to Recover** — the latency required to detect the failure,
mobilize replacement supply, clear queuing congestion, and restore equilibrium. A system survives if and only if:

$$\mathrm{TTS}_{\mathrm{survival}} \ge \mathrm{TTR}_{\mathrm{recovery}} \qquad \text{\texttt{[Assumed]}}.$$

Both sides of the inequality are measured in identical physical units (seconds, hours, or days), making the
condition auditable in advance. Sizing reserves to satisfy this inequality guarantees that the system possesses
adequate slack to withstand the Kingman congestion wave, overcome the Itô variance drain, and avoid absorption
at the ruin barrier.

## Bridge

Non-ergodicity, the Itô variance drain, and Kelly leverage sizing are formalised in
[Chapter 47](47-irreversible-time-vital-margin.md) as
`vital.rs::VitalMarginState::from_non_ergodic_returns` and
`vital.rs::VitalMarginState::from_kelly_parameters`; the joint evaluation of Kingman heavy-traffic
queuing, Holling resilience basin width, and the master survival inequality
$\mathrm{TTS}_{\mathrm{survival}} \ge \mathrm{TTR}_{\mathrm{recovery}}$ is executed by
`vital.rs::VitalMarginState::from_queue_basin_and_survival`.
