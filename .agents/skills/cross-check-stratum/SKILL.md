---
name: cross-check-stratum
description: >-
  Run Phase 3 cross-boundary and Part V stratum audits in Stochology. Supports
  Stratum Mode (S1-S7: all 5 orders + Cone(L_i) + linked Part IV theory via
  run_workflow) and Boundary Pair Mode (adjacent specialists hunting
  interdisciplinary translation errors). Invoke via /cross-check-stratum.
---

# Cross-Boundary & Stratum Audit (`/cross-check-stratum`)

Implements **Phase 3 (Cross-Boundary Audit)** across adjacent disciplinary interfaces and Part V strata, targeting **interdisciplinary translation errors** where an equation is formally valid in one plane but misapplied in institutional, legal, or domain praxis.

## Mode A — Part V Stratum Audit (`/cross-check-stratum S1..S7`)

1. **Deterministic Graph & Part IV Anchor Mapping:**
   - Run:
     ```bash
     python3 .agents/scripts/check_graphs.py --stratum <Sk> --markdown
     ```
   - Load the 5 Part V orders of `<Sk>`, `book-src/stratum-<roman>-opening.md`, and the paired Part IV theoretical chapters:
     - `S1` (`51`–`55`): Chs. `46`, `47`
     - `S2` (`56`–`60`): Chs. `43`, `44`, `45`, `App. B.1`
     - `S3` (`61`–`65`): Chs. `46`, `47`, `24`–`26`
     - `S4` (`66`–`70`): Chs. `43`, `44`, `45`, `46`
     - `S5` (`71`–`75`): Chs. `43`, `45`, `49`, `50` §9.2
     - `S6` (`76`–`80`): Chs. `45`, `46`, `48`, `49`
     - `S7` (`81`–`85`): Chs. `47`, `48`, `49`, `50`
2. **Parallel Workflow Fan-Out (`run_workflow`):**
   - Read the built-in `run-workflow` skill first before authoring the Python workflow script.
   - Fan out via `parallel(...)` across:
     - `orders-homomorphism`: audits $\mathcal{G}_1$ zero-axis inheritance in `Cone(L_i)`, `J(7,3)` dualities, and `[Forced-Analogy]` risks across all 5 orders.
     - `redteam-gov`: stress-tests operational/institutional realism and capture vectors across the 5 orders vs Part IV safeguards.
     - **Stratum technical specialist** (`logic-systems` for `S1`/`S2`/`S3`/`S7`; `formal-stat` for `S4`/`S6`; `philosophy-sources` for `S5`): audits technical derivations, `crates/` parity (for `S1`/`S2`/`S3`/`S4`/`S6`/`S7`), or external primary sources (`S5`).
3. **Cross-Boundary Reconciliation:**
   - Synthesize interdisciplinary discrepancies into `reviews/stratum-<Sk>/boundary-synthesis-YYYYMMDD-HHMM.md`, paying special attention to any mismatch between Part IV theory and Part V operationalization.

## Mode B — Adjacent Specialist Pair Audit (`/cross-check-stratum pair <specA> <specB> <targets>`)

Use for high-risk interdisciplinary seams (examples):
- `formal-stat` + `redteam-gov` on Chs. `48`, `77`, `81`–`82` (Chouldechova trilemma, performative equilibria $\theta_{\mathrm{PS}}$ vs $\theta_{\mathrm{PO}}$, evidentiary law).
- `logic-systems` + `redteam-gov` on Chs. `49`, `83`, `93`–`95` (algedonic channel latency, $e$-process false-alarm filtering, quarantine vs `Compliance DoS`).
- `formal-stat` + `philosophy-sources` on Chs. `18`, `44`, `50` (Popper–Mayo severity, Lakatosian deflation $E_T/(K_{\max}+1)$, Spanos misspecification).

Protocol:
1. Run both specialists in parallel via `invoke_subagent` in blind isolation on the shared target chapters.
2. Write both reports to `reviews/cross-boundary/<pair>-YYYYMMDD-HHMM/`.
3. Produce a short reconciliation diff listing **only** cross-domain translation errors (`Level 1` or `Level 2`), obeying `02-severity-rubric.md`.

## Mandatory Post-Run Custody Gate

Run `git status --porcelain --untracked-files=all -- ':!reviews/'` after every invocation and confirm empty output before reporting completion.
