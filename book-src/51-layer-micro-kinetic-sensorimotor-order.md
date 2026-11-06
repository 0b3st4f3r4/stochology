# Chapter 51 — Layer 01: Micro-Kinetic & Sensorimotor Order

> Layer 01 (`multilayer.rs::KnowledgeOrder`, `KnowledgeOrder::L01MicroKinetic`) of Part V · Stratum I (`multilayer.rs::KnowledgeStratum`, `KnowledgeStratum::S1PhysicalDynamical`) · Unique Guard Combination `{T1Framing, T5Temporal, T7Liability}` · Signature `(1, 1, 1)` (`multilayer.rs::TriadicExposureRegime`, `TriadicExposureRegime::TransTriadicKeystone`). It forms the basal root ($\mathrm{Anc}^*(L_{01}) = \{L_{01}\}$) of $\mathcal{G}_1$ and maps into $\mathcal{G}_2$ via [`notes/The Multilayer Graph of Knowledge and Aim.md`](../notes/The%20Multilayer%20Graph%20of%20Knowledge%20and%20Aim.md).

## By the end of this chapter you can

1. Separate what the agent issued from what the environment returned, in a loop whose period is
   shorter than the time needed to reconsider the command.
2. Compare the slack before an absorbing boundary with the time required to recover, and decide
   from that comparison alone whether the operating point is admissible.
3. Name the one quantity a fast loop must never optimise directly, and say why maximising it
   guarantees the boundary is reached.

## 1. The problem

There is a class of processes in which the state evolves faster than the controller can deliberate.
Every command is therefore issued on information that is already stale by the time the command acts,
and the delay between measurement and actuation is not a technical nuisance but the dominant term in
the dynamics.

Two structural features define the class. The horizon is **finite**: the process can enter a state
from which no later correction exists, and the crossing is not gradual but absorbing. And the
controller is **accountable**: someone chose the operating point, the thresholds and the response
time, and that choice is what makes the failure attributable rather than merely unfortunate.

Once a horizon is absorbing, the relevant question stops being *what outcome do we expect* and
becomes *what can we guarantee before the horizon*. That reframing is the whole content of this
closure.

## 2. Two tapes

The single most useful decomposition at this scale is the one the framework calls the two tapes.

The **command tape** is what the agent issued: the declared bounds on what it believes, the capacity
it provisioned, the reserve it allocated, the brake it did or did not apply. It is a record of
intention, and it is fully observable.

The **response tape** is what the environment returned: the realised state, the realised load, the
harm that actually occurred, whether the boundary was crossed. It is a record of contingency, and it
is partly outside anyone's control.

The mistake this decomposition prevents is scoring a policy on the response tape alone. A policy
tuned on realised outcomes will discover that the cheapest way to look good is to be lucky: to run
close to the boundary in the cases where the environment happened to be gentle, and to have no
mechanism for the cases where it was not. Because the failure is rare, the score improves and the
exposure grows, silently, at the same time.

The discipline is to score the two tapes separately and then ask whether the good response was
*caused by* a good command. When it was not — when the command was reckless and the outcome was
favourable — the credit is void. That distinction is what makes a fast loop governable at all.

Computability theory and type systems explain why this check must inspect the *syntax* of the command
tape rather than attempt to prove the *semantics* of arbitrary agent code at runtime. By **Rice's
theorem** `[Literature]`, any non-trivial semantic property of an arbitrary program's future execution is
undecidable; a supervisor waiting to verify what an unconstrained black-box policy will do in the world
cannot terminate before the horizon. The only decidable alternative is a conservative syntactic
certificate over declared premises, verified in linear time before actuation (`grammar.rs::judge`). And
in Jean-Yves Girard's **linear logic** `[Literature]` — realised in Rust's affine ownership and move
semantics — finite physical reserves and single-opening pre-outcome records (`governance.rs::TemporalVault`)
are linear resources that forbid contraction (a reserve cannot be duplicated and spent twice) and
unrestricted weakening (an active alarm cannot be silently dropped without handling).

## 3. Slack and recovery

At this scale the survival condition has a direct physical meaning.

Slack is the distance from the boundary divided by the rate at which that distance is being consumed.
Recovery is the time needed to restore the operating margin once it has been eaten. The admissible
operating region is the set of points where slack exceeds recovery, and it is a region one can compute
before the run rather than discover during it.

Two consequences are easy to state and hard to institutionalise. First, the operating point degrades
with every action: a command that consumes margin without restoring it moves the system toward the
region where no correction is possible, and the degradation is cumulative in a way that per-decision
reviews never see. Second, the comparison is only meaningful in the same units. An interval quoted in
probability and a reserve quoted in seconds cannot be compared, and the temptation to compare them
anyway is the origin of most of the confident errors at this scale.

The check is therefore quantitative and cheap: for the current operating point, compute both sides,
and refuse the point when the inequality fails — regardless of how good the expected outcome looks.

## 4. Model-to-world discrepancy

Every fast loop runs on an internal representation, and that representation is wrong in a way that is
not uniform.

Near the middle of the operating envelope the discrepancy between the internal model and the actual
process is small, and a controller tuned there performs well. As the operating point approaches the
limit, the discrepancy grows: the approximations that were harmless at moderate load become the
dominant term exactly where the decision is most consequential. A controller optimised until the
discrepancy was invisible will therefore behave as though the limit does not exist, and will meet the
limit at the moment it is least able to respond.

The remedy is not a better model in general but a declared gap in particular: the discrepancy is
modelled as a function of the operating point, estimated where measurements exist, and bounded where
they do not. What must not happen is the third option — the gap being neither estimated nor declared,
so that the confidence reported downstream is entirely a property of the model's optimism.

## 5. The model inside the fast loop

A controller with a delay budget shorter than its deliberation budget must deliberate elsewhere. The
standard architecture splits the labour into three parts, and the split is worth stating precisely
because each part fails differently `[Literature]`:

1. **Perception compresses.** A variational autoencoder maps the observation $o_t$ to a latent $z_t$,
   so that the controller never sees pixels but a low-dimensional state. What the compressor discards
   is not noise but *unmodelled physics*: everything the training distribution did not vary.
2. **Dynamics predicts.** A recurrent mixture-density network models $P(z_{t+1} \mid z_t, a_t)$,
   which is the object the controller optimises against. The mixture is what allows several futures;
   the recurrence is what allows a horizon.
3. **The controller is small.** A tiny policy $a_t = C(z_t, h_t)$ is trained *inside* the model,
   which is precisely the operation that makes the loop fast and the loop fragile.

Everything about this architecture follows from the fact that the policy is trained on the model's
future, not on the world's. Two later refinements keep the same skeleton and are worth naming because
they each answer one of its failure modes. Recurrent state-space models separate a deterministic
recurrent path from a stochastic latent per step, so that the controller can distinguish what is
*known* from what is *drawn*; latent overshooting trains the model over multi-step horizons rather
than one-step transitions, which is what closes most of the sample-efficiency gap. And reward
compression — the logarithmic lens that maps a wide range of magnitudes into a bounded interval —
stabilises training across domains whose rewards differ by orders of magnitude, which is the same
ordinal-invariance discipline that [Chapter 46](46-non-identical-asymmetry-heavy-tails.md) imposes on
heavy-tailed reserves.

### The dream exploit, stated as a discrepancy

When a policy is optimised against $\hat{M}$ without any constraint that ties the visited latent
states back to the region where $\hat{M}$ was fitted, the optimiser does what optimisers do: it finds
the direction of largest cheap improvement, and that direction is the direction in which $\hat{M}$ is
wrong. Soon the trajectory lives in the region of largest structural discrepancy
$\delta(z) = |f(z) - \hat{f}(z)|$, where the dreamed reward rises and the realised reward collapses
through a constraint the model never represented. This is not a bug in the optimiser. It is the exact
convergence of two facts this treatise has already established: the extremal and adversarial variants
of Goodhart's law ([Chapter 48](48-reactive-worlds-performativity-goodhart.md)) and the growth of
model-to-world discrepancy with distance from the fitted support (§4 above).

The audit `governance.rs::LatentModelExploitationAudit` makes that conjunction computable. A linear
surrogate is fitted by least squares on anchors confined to a support radius, the policy is optimised
inside it, and the same actions are executed against a world carrying one absorbing constraint the
surrogate never saw. On the canonical fixture `[Simulated]` the greedy dreamer leaves the support by
2.87 radii, reaches a terminal discrepancy of 0.883 at the dreamed terminal state, and destroys the
world at step 6 while its dreamed return stays 4.4 times its realised return; the classification is the `Adversarial` variant of `governance.rs::GoodhartVariant`, not `Regressional`, because the policy is not merely
selected on a noisy proxy — it is *searching* the proxy for its blind spot.

The Boltzmann temperature over the action set is the classical mitigation, and the audit reports both
what it buys and what it does not `[Simulated]`: raising $\tau$ across the sweep 0.05, 0.20, 0.80, 3.20,
12.80 raises the realised return monotonically (4.27, 4.84, 5.43, 7.88, 7.88), delays ruin (steps 6,
7, 8, then survival), and collapses the terminal discrepancy from 0.875 to 0.020. Temperature
therefore converts a committed exploit into an exploratory trajectory. It does **not** certify the
model: the trajectory that survives does so because it never commits, not because anyone checked where
$\hat{M}$ stops being true. Certification is a separate act, and it is the one performed by the
discrepancy audit and, at the level of the institution, by the ex-ante gate of
[Chapter 48](48-reactive-worlds-performativity-goodhart.md).

## 6. Emulated embodiment: from a connectome to a control law

The most literal test of §1–§4 is a system in which the "model" is a measured nervous system rather
than a fitted function. The adult *Drosophila* connectome has now been reconstructed at synaptic
resolution — the full-brain electron-microscopy reconstruction published with its wiring diagram and
annotated cell types `[Literature]` — and the natural next step, emulating that circuit and embedding it
in a simulated body, converts neuroanatomy into a controller.

Two stochological observations follow, and neither is about flies.

First, a connectome is an *archive* in the exact sense of [Chapter 34](34-sample-population-archive.md):
a single, non-randomly missing, preparation-specific snapshot of one individual, from which claims about
the species are drawn by an inference step that the diagram itself does not contain. The wiring is
measured; the behaviour is not. Second, the act of embedding the circuit in a body reintroduces the
question of §3: the emulated controller inherits the same $\mathrm{TTS}_{\mathrm{survival}} \ge
\mathrm{TTR}_{\mathrm{recovery}}$ condition as any other fast loop, and its performance on the body is a
statement about the pair, not about the brain alone. A perfect copy of a control circuit attached to a
body with the wrong inertia is a controller with the wrong margin, and no amount of anatomical fidelity
repairs that.

### The step that turns anatomy into a controller

The Drosophila case is worth following one level down, because it is the only published instance in which a
connectome was made *predictive* rather than descriptive, and the method is a template.

The first step is a spiking model: a leaky integrate-and-fire neuron accumulates input in proportion to synaptic
weight until a threshold is crossed, then fires and resets. Four quantities — stimulus, weight, membrane potential,
spike — are enough to convert a static map into an activity trace `[Literature]`. The second step is chemistry: the
sign of each synapse depends on its neurotransmitter, so the same wiring diagram becomes an excitatory or inhibitory
circuit according to a signal that the anatomy alone does not encode. In the published model the whole circuit is
governed by a **single free parameter**, the global synaptic weight, which is the cleanest possible statement that
the structure does the work and the tuning does not `[Literature]`.

The third step is the one that interests this chapter: a *readout*. Feeding behaviour is scored by the firing rate of
one identified motor neuron, over a bounded range, so the model's prediction is a single scalar with units. With that
readout fixed, the model predicts the effect of activating each of 106 cell types, and the prediction is checked
against optogenetic activation in living flies — 91% agreement, including the neurons predicted to have no effect
`[Literature]`. The same machinery then *contradicts* the textbook reading of one receptor, and the living preparation
confirms the model's version `[Literature]`. A second, mechanosensory circuit is used as an out-of-sample test: there,
78 direct synapses between two identified neurons produce no activation at all, because three inhibitory
interneurons short-circuit the path — a fact invisible in the wiring diagram and visible only in the simulated flow
of membrane potential `[Literature]`.

Four stochological lessons, and none of them is about neuroscience.

1. **"Connection is not activation" is the locality theorem of [Chapter 53](53-layer-macro-coupled-network-contagion-order.md)
   §5 in biological form.** Adjacency is a local statistic; whether a signal propagates is a property of the whole
   path, signs included. A graph-based audit that counts edges will always overestimate reachability.
2. **91% agreement is a claim about a support, not a property of the model.** It is 91% on 106 cell types in one
   preparation. The number is silent about neurons outside the annotated set, about modulators absent from the model,
   and about internal states such as hunger — a limitation the authors state and which the discipline of
   [Chapter 45](45-world-to-archive-big-data-paradox.md) would call the archive's coverage, not the model's error.
3. **A one-parameter model that predicts is a stronger artifact than a many-parameter model that fits.** The parameter
   count is not the achievement; the fact that the *structure* carries the prediction is. That is the difference
   between an explanation and a compression, and it is why the readout had to be an observable with units.
4. **The validation loop is the institution.** Prediction, optogenetic test, revision, second circuit. Without the
   second circuit the model would be a post-hoc fit to the first; the out-of-sample circuit is what turns the
   exercise into a test rather than a demonstration. Any predictive system deployed without an equivalent
   out-of-sample trap is running on the first circuit alone.

## 7. Control at the friction limit: model, simulation, and demonstration

The same three-way distinction — model, simulator, demonstration — is what an autonomous-race-car
benchmark isolates, and the audit `engine.rs::FrictionEnvelopeControlAudit` reduces it to its load-bearing
structure `[Simulated]`. A point-mass vehicle is driven around a fixed track under the
friction ellipse $a_{\mathrm{long}}^2 + a_{\mathrm{lat}}^2 \le (\mu g)^2$. Three controllers face the
same surface:

- `ModelBasedMpc` (`engine.rs::ControlRegime`) plans on a physics model whose friction coefficient was
  identified on the deployed surface, with a safety margin;
- `InSimulationTuned` is a two-parameter policy tuned by exhaustive search *inside* a simulator that
  assumes a drier track;
- `DemonstrationFitted` is the same policy class, fitted by least squares to demonstrations recorded from
  the model-based controller.

On the canonical fixture the model-based controller completes the lap with zero envelope violations and
a worst utilisation of 0.966, while both learned controllers lose grip at the sharpest corner — the
in-simulation-tuned one at 1.207 utilisation, the demonstration-fitted one at 1.122. The ordering is the
informative part: imitation is gentler than tuning because the demonstrations carry the teacher's
coupling between speed and curvature, but imitation still fails, because a regressor fitted on the
visited distribution has no reason to be right at the edge of it. Raising the evaluation to a 10%
friction deficit destroys the model-based lap as well, which is the point of the whole exercise: the
margin was never a property of the controller, it was a claim about the surface.

`vital.rs::VitalMarginState` is where that claim belongs. The audit's utilisation is a dimensionless
restatement of slack, and the deficit experiment is the standard test of whether a nominal margin was
ever a survival margin.

## Reproduce

```bash
cargo run --release --bin stochology -- --chapter 1
cargo run --release --bin stochology -- --chapter 6
```

## Bridge

Layer 01 (`multilayer.rs::KnowledgeOrder`, `KnowledgeOrder::L01MicroKinetic`, `{T1Framing, T5Temporal, T7Liability}`) propagates its local and ancestor-cone margins through `multilayer.rs::MultilayerHomomorphism::evaluate` into [Chapter 52 — Layer 02: Meso-Buffered Queue, Operational Dispatch & Cognitive Bottleneck Order](52-layer-meso-buffered-queue-dispatch-order.md) (`KnowledgeOrder::L02MesoBufferedQueue`, Stratum I, `{T4Formalization, T6Performative, T7Liability}`).
