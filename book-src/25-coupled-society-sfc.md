# 25. Coupled Public-Private Stock-Flow Consistent Society

> **Three tempos.** **Detachment:** set aside the habit of evaluating a public policy and a private credit rule one at a time, as if each acted on a fixed economy. **Abstraction:** every payment is someone else's receipt; when the regulator's balance sheet and the private sector's balance sheets are coupled in closed loop, effects appear that no single-sector model can show (Decision ⇄ World, read across all six pairs). **Return:** the stock-flow consistent transaction matrix, a $2 \times 2$ difference-in-differences that isolates closed-loop coupling, Bester's screening threshold, and Green-Laffont leakage under VCG routing. **Negative proof:** `gates -- --defect coupled_society` (exit `1`).

This chapter was §22.7 of the relational-ontology chapter in earlier editions. It is a Tier 3 synthesis: its claims are pre-registered, gated by `G8`, and every number below carries its provenance class (simulated with seed, measured with dataset, or assumed). The specification is mirrored in [`docs/syntheses/society.md`](../docs/syntheses/society.md).

## Two Ledgers That Must Balance

As a thought experiment, picture a town where the treasury, the bank, the firms, and the households each keep a ledger. A shock hits the supply chain. If the regulator reacts only to the data it can see, with a two-period lag and with the worst-hit firms missing from its records, its policy feeds back into firms' survival, which feeds back into the data the regulator sees next. Asking "does the policy help?" with a one-shot comparison of policy bundles mixes the policy with the feedback loop. This chapter's design separates the two.

In `crates/stochology-lenses/src/society/` (pre-registered in `data/preregistration/coupled_society_preregistration.yaml`, `K = 6`, virgin confirmation block `9_000_200..=9_000_399`), `CoupledPublicPrivateSocietySynthesis` and `EmpiricalCoupledSocietyAudit` test what emerges **only when public and private balance sheets are coupled in closed loop** across $L\_1 \to L\_2 \to L\_3 \to L\_1$:

## 25.1 Closed-Economy Stock-Flow Consistent Accounting (`SfcFlowMatrix`)

Following Wynne Godley & Marc Lavoie (*Monetary Economics*, 2007) and Michał Kalecki (*Theory of Economic Dynamics*, 1954), `SfcFlowMatrix` enforces a 4-sector (Households, Production Firms, Commercial Banks, Public Regulator/Treasury + Capital Account; 7 rows $\times$ 5 columns) transaction-flow matrix under an **assumed** closed-economy setting ($NX \equiv 0$, public debt held as commercial bank reserves $\Delta B\_t \equiv \Delta M\_{B,t}$). Every epoch closes the row-sum, column-sum, Kalecki-Levy profit ($\Pi\_t = I\_t + (G\_t - T\_t) - S\_{H,t}$), and Godley Three-Balances ($(S\_{H,t} - I\_t) + \Pi\_{B,t} \equiv G\_t - T\_t$) identities to $< 10^{-12}$ (`SFC_err=1.8e-14, KG_err=1.8e-14`, simulated, `seed = 2026`), and `SplitMix64::split` per `(domain_tag, epoch, agent_id)` guarantees exact $L\_\infty == 0.0$ permutation invariance across `0..N` vs. `(0..N).rev()`.

## 25.2 `H1a` — Closed- vs. Open-Loop Regulator Coupling (`H1aRegulatorCouplingAblation`)

To separate the effect of closed-loop macroeconomic feedback from a static comparison of policy bundles, `H1aRegulatorCouplingAblation` holds Carroll (1997) buffer-stock households ($N\_H = 120$, **assumed** $\gamma = 2.0$, target liquid wealth $m^\ast = 1.50$), Basel III `CCyB`, and Bester (1985) credit constant across all 4 cells (`policy_shielded` $\in \lbrace\text{false}, \text{true}\rbrace \times$ `closed_loop` $\in \lbrace\text{false}, \text{true}\rbrace$) under a heavy-tailed supply-chain productivity shock at $t\_0 = 10$ (**assumed** Pareto exponent $\alpha = 2.25$, scale $x\_m = 0.18$, shocked fraction $f\_{\text{shock}} = 0.25$). Only the public regulator varies under bitemporal lag $\Delta t\_{\text{disp}} = 2$ and administrative `MNAR` selection ($\rho\_{R,Y} = -0.15, f\_{\text{obs}} = 0.75$): `shielded = true` (bitemporal filter + Manski 2004 cost-loss weighted Minimax-Regret point $\theta\_{\text{MMR}} = (1 - p^\ast) U\_M + p^\ast L\_M$ at $p^\ast = 0.12$ + Info-Gap/Sniedovich floor + oracle-free two-point finite-difference `PerfGD`, Izzo et al. 2021) vs. `shielded = false` (event-time `MNAR` point estimate + damped `RRM`). At `seed = 2026` (simulated), closed-loop survival is $1.000$ vs. $0.533$ (`open_loop`: $0.733$ vs. $0.500$, Coupling DiD $+0.233$) and the continuous CRRA household welfare Coupling DiD is $+0.4549$ (`society.h1a.closed_loop_coupling_did_crra_welfare_above_0.04`, $100.0\%$ on confirmation seeds). **Discarded claim:** `society.h1a.closed_loop_coupling_did_firm_survival_above_0.06` passed on $95.0\%$ ($19/20$) of development seeds `2026..=2045` but was refuted at $94.5\%$ ($189/200 < 0.95$) on the virgin confirmation block `9_000_200..=9_000_399` due to discrete $1/N\_F = 1/30$ firm-count quantization across the 4-cell DiD; it is recorded in `discarded_claims` without altering any simulation parameter.

## 25.3 `H1b` — Bester Collateral Screening vs. Stiglitz-Weiss Pooling (`H1bCreditScreeningAblation`)

With closed-form existence threshold $\lambda^\ast\_{\text{Bester}} = 0.7534 \in (0.55, 0.85)$, Bester collateral screening lifts solvent firm survival by $+0.175$ at $\lambda\_{\text{good}} = 0.50 < \lambda^\ast\_{\text{Bester}}$ (`society.h1b.bester_collateral_screening_beats_stiglitz_weiss_pooling_by_0.08`, $96.5\%$ on confirmation seeds) and automatically switches to low-rate collateralized pooling at $\lambda\_{\text{good}} = 0.88 > \lambda^\ast\_{\text{Bester}}$ ($+0.025$, simulated, `seed = 2026`).

## 25.4 `H2` — VCG Surplus Routing, Green-Laffont Leakage, and the Ostrom Commons (`H2VcgGreenLaffontCommonsAblation`)

Across cohort sizes $M \in \lbrace 4, 8, 16, 32\rbrace$ under an identical `aGRAPA` betting martingale ($E\_{60} = 2753.9$, simulated, `seed = 2026`), routing Clarke's pivotal tax surplus to a **same-cohort** equal-share buffer ($\omega\_i = 1/M$) re-opens Green & Laffont's (1979) indirect tax channel: profitable misreporting is $\bar{\varepsilon}\_{\text{same}}(4) = 0.4286 > 10^{-3}$ and decays strictly as $O(1/M)$ to $\bar{\varepsilon}\_{\text{same}}(32) = 0.0363$ (`society.h2.det.same_cohort_buffer_leakage_decays_as_inv_m`, $100.0\%$). Routing the surplus to a **disjoint** downstream cohort ($\omega\_i = 0$ for reporting cohort $A$) preserves exact DSIC ($\bar{\varepsilon}\_{\text{disjoint}}(M) = 0.0 < 10^{-12}$) and retains Ostrom (1990) renewable commons stock $S\_T / K = 0.730$ vs. $0.050$ under direct proportional rebate (`society.h2.disjoint_cohort_vcg_commons_stock_beats_rebate_by_0.10`, $100.0\%$ on confirmation seeds).

## 25.5 Real-Data Coupler (`EmpiricalCoupledSocietyAudit`)

**Measured** on `cps_observational.csv` ($n = 15{,}992$, Moulton-Campbell $\tilde{m} = 95.65$, $n\_{\text{eff}}(q\_{90}) = 1{,}583.5$, Meng $n\_{\text{eff,Meng}} = 4.5$ under $25\%$ lower-tail `MNAR`, Manski width $\$6{,}391.21$, CRRA $\Delta\_J = -0.0975$), `bank_calls.csv`, and `danish_fire.csv`.

## Computational Audit in Rust

`CoupledPublicPrivateSocietySynthesis::evaluate(2026)` runs in `labs --all` and in Gate `G8`, which evaluates the four gated claims on the virgin confirmation block `9_000_200..=9_000_399`:

~~~bash
cargo run --release --bin labs -- --all
cargo run --release --bin gates
~~~

~~~text
  [Coupled Public-Private SFC Society Synthesis] H1a_SFC_Regulator(Closed_Surv=1.000 vs Naive=0.533, Open_Surv=0.733 vs 0.500, DiD_Surv=+0.233, DiD_CRRA_Welfare=+0.4549, SFC_err=1.8e-14, KG_err=1.8e-14, Perm_Linf=0.0e0) | H1b_Bester_Credit(lambda*=0.753, Gain_0.50=+0.175, Gain_0.88=+0.025) | H2_VCG_Commons(eps_same M4=0.4286->M32=0.0363, eps_disj=0.0e0, Commons_Stock=0.730 vs Rebate=0.050, E_60=2753.9) | EmpCPS(m_tilde=95.65, n_eff=1583.5, Meng_n_eff=4.5, Manski_W=6391.2, CRRA_DJ=-0.0975, Card_OK=true) | identities=true | discarded=["society.h1a.closed_loop_coupling_did_firm_survival_above_0.06: passed 0.950 on development seeds (2026..=2045) but refuted at 0.945 (189/200 < 0.95) on the virgin confirmation block (9_000_200..=9_000_399) due to discrete 1/N_F = 1/30 firm-count quantization across the 4-cell DiD", "Same-cohort equal-share VCG buffer (omega_i = 1/M) is exact DSIC at finite M (refuted: Green-Laffont indirect tax channel yields eps_same(4) > 1e-3, decaying as O(1/M) and requiring disjoint-cohort routing for exact 0.0)", "Bester (1985) separating collateral contract strictly dominates pooling for all safe-borrower shares lambda_good in (0, 1) (refuted at lambda_good = 0.88 > lambda*_Bester where collateral repossession deadweight loss exceeds pooling cross-subsidy)", "cps_observational.csv: unadjusted administrative sample mean under 25% lower-tail MNAR attrition covers the full-population mean within +/- 1 SE (refuted: Meng ddc != 0 shrinks n_eff_Meng, requiring non-parametric Manski bounds)"] | verified=true
~~~

## Negative Proof

With `policy_shielded = false`, the naive arm uses the event-time `MNAR` point estimate with damped `RRM`, Stiglitz-Weiss uncollateralized pooling, and a proportional-rebate VCG. The defect blocks on all 20 development seeds:

~~~bash
cargo run --release --bin gates -- --defect coupled_society
~~~

~~~text
NEGATIVE PROOF [coupled_society] BLOCKED SUCCESSFULLY on all 20 development seeds through seed-dependent claims (identities closed on every seed): at seed 2026 event-time MNAR RRM closed-loop firm survival=0.533 (Coupling DiD gain=+0.000, CRRA welfare DiD=+0.0000), Stiglitz-Weiss uncollateralized pooling survival=0.558 (Bester gain=+0.000), proportional-rebate VCG Ostrom commons stock S_T/K=0.050 (disjoint-cohort gain=+0.000)
~~~

## Provenance Ledger

| Number | Class | Source |
| --- | --- | --- |
| Closed-loop survival $1.000$ vs. $0.533$; CRRA welfare DiD $+0.4549$ | Simulated | `seed = 2026`, `labs --all` |
| $\gamma = 2.0$, $m^\ast = 1.50$, Pareto $\alpha = 2.25$, $x\_m = 0.18$, $f\_{\text{shock}} = 0.25$, $p^\ast = 0.12$ | Assumed | `H1aRegulatorCouplingAblation` inputs |
| $\lambda^\ast\_{\text{Bester}} = 0.7534$ | Simulated (closed form at the assumed parameters) | `H1bCreditScreeningAblation` |
| $\bar{\varepsilon}\_{\text{same}}(4) = 0.4286$, $\bar{\varepsilon}\_{\text{same}}(32) = 0.0363$, $S\_T/K = 0.730$ vs. $0.050$ | Simulated | `seed = 2026` |
| Confirmation pass rates $100.0\%$, $96.5\%$, $100.0\%$, $100.0\%$ (discarded claim: $94.5\%$) | Simulated | 200 confirmation seeds `9_000_200..=9_000_399`, `gates` |
| $\tilde{m} = 95.65$, $n\_{\text{eff}}(q\_{90}) = 1{,}583.5$, $n\_{\text{eff,Meng}} = 4.5$, Manski width $\$6{,}391.21$ | Measured | `cps_observational.csv` (the Meng value is under an imposed $25\%$ lower-tail `MNAR` attrition) |

## Exercises

1. **(Calculation: Kalecki-Levy Profit Identity)** In one epoch of an **assumed** closed economy, investment is $I\_t = 20$, public spending $G\_t = 30$, taxes $T\_t = 25$, and household saving $S\_{H,t} = 10$. Compute aggregate profit $\Pi\_t$ and explain why a government surplus ($G\_t < T\_t$) reduces private profit unless investment or household dissaving offsets it.
2. **(Concept: Why a Difference-in-Differences)** `H1aRegulatorCouplingAblation` reports survival in four cells (shielded/naive $\times$ closed/open loop). Why does the open-loop contrast ($0.733$ vs. $0.500$) have to be subtracted from the closed-loop contrast ($1.000$ vs. $0.533$) before attributing anything to the coupling itself?
3. **(Calculation: Green-Laffont Leakage)** Using the simulated values $\bar{\varepsilon}\_{\text{same}}(4) = 0.4286$ and $\bar{\varepsilon}\_{\text{same}}(32) = 0.0363$, compute $M \cdot \bar{\varepsilon}\_{\text{same}}(M)$ at both cohort sizes. Is the decay consistent with $O(1/M)$? Is it exactly proportional to $1/M$?
4. **(Concept: A Claim That Failed Confirmation)** The claim `society.h1a.closed_loop_coupling_did_firm_survival_above_0.06` passed on $19/20$ development seeds but on $189/200$ confirmation seeds. Why is it reported as discarded rather than approved, and why must the simulation parameters not be changed after seeing the confirmation result?

<details>
<summary><strong>Analytical &amp; Computational Solutions (Click to Expand)</strong></summary>

1. **Kalecki-Levy:** $\Pi\_t = I\_t + (G\_t - T\_t) - S\_{H,t} = 20 + (30 - 25) - 10 = 15$. With $G\_t < T\_t$, the public sector drains net financial assets from the private sector; the drain lowers profit one-for-one unless investment rises or households save less. The identity is accounting, not behaviour: it says where the money must go, not why agents chose it.
2. **Difference-in-Differences:** The shielded regulator might raise survival through its point estimate alone, even with no feedback. The open-loop contrast ($+0.233$) measures that static effect; the closed-loop contrast ($+0.467$) measures static effect plus feedback. Their difference, $+0.233$ (simulated, `seed = 2026`), is the part attributable to closing the loop.
3. **Leakage:** $4 \times 0.4286 = 1.714$ and $32 \times 0.0363 = 1.162$. Both products stay bounded, which is what $O(1/M)$ requires; they are not equal, so the decay is not exactly proportional to $1/M$ at these cohort sizes.
4. **Confirmation:** The acceptance rule requires at least $95\%$ on both seed sets; $189/200 = 0.945 < 0.95$. The development seeds were used to decide what to keep, so only the untouched confirmation block tests the claim. Changing parameters after seeing it would turn the confirmation block into another development set and spend the pre-registration ($K = 6$) without recording a new attempt.

</details>

## Summary

- **SFC accounting** closes the row, column, Kalecki-Levy, and three-balances identities to $< 10^{-12}$ (simulated), with exact agent-order permutation invariance.
- **`H1a`** isolates closed-loop coupling with a $2 \times 2$ DiD: CRRA welfare DiD $+0.4549$ (simulated, `seed = 2026`; gated at $100.0\%$ on confirmation seeds). The firm-survival DiD claim is **discarded** ($94.5\%$).
- **`H1b`** shows Bester screening beating Stiglitz-Weiss pooling below $\lambda^\ast\_{\text{Bester}} = 0.7534$ and switching to pooling above it.
- **`H2`** shows same-cohort VCG rebates leaking as $O(1/M)$ while disjoint-cohort routing keeps exact DSIC and preserves the commons stock.
- **Measured coupler** on `cps_observational.csv` shows how clustering and `MNAR` attrition shrink the effective sample ($n\_{\text{eff,Meng}} = 4.5$ under the imposed attrition).
