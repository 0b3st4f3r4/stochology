# Stratum II — The Instrument and Its Commitment

*An opening page for Stratum II, stated once at the level its five chapters share. It names where each
instance is argued and does not restate them.*

---

Stratum II covers five formal and methodological orders: [Layer 06: Formal Logic, Computability & Linear Resource-Type Order](56-layer-formal-computable-type-order.md), [Layer 07: Probability, Credence and Measurement Order](57-layer-probability-credence-measurement-order.md), [Layer 08: Metrological & Information-Geometric Order](58-layer-metrological-information-geometric-order.md), [Layer 09: Experimental Design, Complex Survey Sampling & Causal Methodology Order](59-layer-experimental-sampling-causal-order.md), and [Layer 10: Cryptographic Protocols, Zero-Knowledge Proofs & Adversarial Verification Order](60-layer-cryptographic-zero-knowledge-adversarial-order.md).

Together, these five chapters state one shared rule at the formal scale: **every empirical claim passes through an instrument that has already decided what can be asked, recorded, or proved before any data arrives.**

## What the five orders share

These chapters look like the most technical section of the book, and they carry the highest leverage. Before a single measurement lands on a spreadsheet, an instrument has already locked in its rules. A logic decides which checks finish in time; a scale decides which averages are legal; a calibration geometry decides how drift and uncertainty combine; a sampling frame decides who gets erased; a protocol decides what a clever adversary can fake.

Across Stratum II, those pre-evidential commitments step up from simple rules of syntax to full adversarial proof:

1. **Syntactic rules and resource budgets (`L06`):** Fast static type checks verify before execution that reserves are spent at most once, while lightweight runtime expiry gates block stale premises, without waiting on undecidable code behavior.
2. **Credences and measurement scales (`L07`):** Beliefs turn into bounded probability intervals rather than fake point guesses, while strict scale rules block illegal averages of ordinal ranks.
3. **Metrology and calibration geometry (`L08`):** Uncertainty evaluated statistically from repeated readings (Type A) stays distinct from uncertainty evaluated via calibration certificates, resolution, drift, and definitional limits (Type B), keeping calibration and proper scores honest against certificate shopping.
4. **Experimental design and complex surveys (`L09`):** Massive databases collapse down to tiny effective sample sizes once selection bias and clustering enter, exposing the danger of pure method lacking both vital buffers and governance (`(3, 0, 0)`).
5. **Adversarial verification and custody (`L10`):** Zero-knowledge proofs conceal secret witnesses from verifiers and reject forged statements from a cheating prover, while post-seal commitments prevent retroactive tampering. Yet those guarantees hold *only conditionally* on external physical vital margins ($k\_2 = 0$) and pre-seal archival custody (`T3Archive`), because timing side channels against secret prover or signer operations and RowHammer memory bit flips defeat abstract algebra whenever constant-time execution, multi-bit ECC/refresh margins, and non-volatile custody logs fail.

Anchored to `T1Framing` as its designated stratum site, Stratum II turns the usual order of inquiry upside down. Instruments do not passively wait to interpret results; they silently define what counts as evidence ahead of time. Across the dependency graph $\mathcal{G}\_1$, Stratum II enters solely from `L01` into `L06` (bypassing `L05`), couples `L05` and `L06` into `L07`, links `L04` and `L07` into `L08`, passes `L07` and `L08` into `L09`, and branches `L06` and `L07` directly into `L10` (bypassing `L08`–`L09`, so `T3Archive` stays absent across $\mathrm{Cone}(L\_{07})$ and $\mathrm{Cone}(L\_{10})$). Whenever an analyst treats an instrument's equations as self-sufficient, zero local vital ($k\_2 = 0$) or governance ($k\_3 = 0$) guard counts inside the signature $(E, V, G) = (k\_1, k\_2, k\_3)$ expose the whole pipeline to quiet failure.

## Where each instance is argued

| Order / Chapter | Primary & Triple | Problem Resolved |
| :--- | :--- | :--- |
| [Layer 06 (Ch. 56)](56-layer-formal-computable-type-order.md) | `T1*`, `{T1, T4, T5}` `(1, 2, 0)` | Decidable syntactic verification, linear/affine resource accounting, escape boundaries, and premise expiry. |
| [Layer 07 (Ch. 57)](57-layer-probability-credence-measurement-order.md) | `T4*`, `{T1, T2, T4}` `(2, 1, 0)` | Frame-indexed credal widths, four-way empty-field semantics, and measurement-scale invariance groups. |
| [Layer 08 (Ch. 58)](58-layer-metrological-information-geometric-order.md) | `T3*`, `{T3, T4, T6}` `(1, 1, 1)` | Type-A/Type-B evaluation, Fisher–Rao and Bregman geometry, and certificate-shopping reactivity. |
| [Layer 09 (Ch. 59)](59-layer-experimental-sampling-causal-order.md) | `T2*`, `{T1, T2, T3}` `(3, 0, 0)` | Multiverse screening, partial identification, complex survey design effects, and hierarchical shrinkage. |
| [Layer 10 (Ch. 60)](60-layer-cryptographic-zero-knowledge-adversarial-order.md) | `T7*`, `{T2, T6, T7}` `(1, 0, 2)` | Adversarial secrecy, sequential soundness *e*-processes, timing/silicon side channels, and post-seal custody. |

## What this stratum does not settle

Stratum II does not declare any single formal tool as the foundation of all the others. These five orders state one shared constraint in five different tongues, not a ladder of prestige. Logic does not replace calibration, calibration does not replace random sampling, and random sampling does not stop an adversary.

Nor can you trade one formal guarantee for another. A clean type check, a tight probability interval, a geometric distance, an unbiased experiment, and a cryptographic seal measure completely different things. What binds all five chapters is one simple promise: an instrument fixes the rules of evidence *before* the evidence arrives, and it cannot silently rewrite those rules *post hoc* once the outcome is known without an explicit, attested reopening.
