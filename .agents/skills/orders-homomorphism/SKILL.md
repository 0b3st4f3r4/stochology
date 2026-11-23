---
name: orders-homomorphism
description: >-
  Part V multilayer architecture and cone-homomorphism auditor for Stochology
  (Chapters 51-85, Orders L01-L35, Stratum I-VII openings, and Appendix B.2).
  Uses Layer 0 check_graphs.py output to test whether each {Ti,Tj,Tk} triple,
  (E,V,G) signature, Cone(L_i) inheritance, and J(7,3) bridge models genuine
  domain collapse dynamics rather than superficial analogy.
---

# Part V Knowledge-Order & Cone-Homomorphism Auditor (`orders-homomorphism`)

## 1. Mandatory Contracts (Read First)

Before auditing any target, read:
1. [00-constitution.md](../../rules/00-constitution.md) (Review Mode: **read-only** on `book-src/` and `crates/`; output only to `reviews/`)
2. [01-notation-ledger.md](../../rules/01-notation-ledger.md) (stratum designations `S1↔T5..S7↔T7` and symbol ledger)
3. [02-severity-rubric.md](../../rules/02-severity-rubric.md) (Leontief verdict, failure tags, and exact output template)

## 2. Primary Audit Scope

- **Manuscript:** All 35 Part V chapters (`51`–`85`, `L01`–`L35`), `book-src/part-v-introduction.md`, `book-src/part-v-conclusion.md`, `book-src/stratum-i-opening.md` through `book-src/stratum-vii-opening.md`, and `book-src/appendix-b-architecture-verification.md` §B.2 (`Table B.1`).

## 3. Mandatory Deterministic Pre-Check

Before inspecting prose for any order `Lnn` or stratum `Sk`, obtain the exact structural graph manifest by reading or running:
- `python3 .agents/scripts/check_graphs.py --layer Lnn --markdown` (for single-chapter reviews), OR
- `python3 .agents/scripts/check_graphs.py --stratum Sk --markdown` (for stratum audits).

Use its verified output for:
1. Immediate parents and full transitive ancestor cone $\mathrm{Cone}(L_i)$ in $\mathcal{G}_1$.
2. Signature `(E, V, G)` and whether any `0` axis count is genuinely inherited from an ancestor in $\mathrm{Cone}(L_i)$.
3. Exact 12 neighbors in the Johnson graph $\mathcal{G}_2 = J(7,3)$ ($\lvert g_i \cap g_j \rvert = 2$).

## 4. Refutation & Homomorphism Stress-Test Checklist

Evaluate every Part V order against four tests designed to catch `[Forced-Analogy]` and structural leaks:

1. **Cone Homomorphism Substance ($\mathcal{H}(L_i) = \min_{L_j \in \mathrm{Cone}(L_i)} M_{\mathrm{Leontief}}(L_j)$):**
   - Does the chapter merely stamp Stochology buzzwords (`Leontief`, `e-value`, `Kingman`, `Algedonic`) onto domain jargon, or does the triadic minimum genuinely explain an empirical collapse mechanism in that discipline?
   - Could you remove the `{T_i, T_j, T_k}` labels without losing any explanatory constraint? If yes, flag `[Forced-Analogy]` (`Level 2` structural).
2. **Guard Triple & Primary Anchor Fit (`{T_i, T_j, T_k}` and `T_p*`):**
   - Does each of the three assigned guard sites correspond to a concrete, irreducible failure mode in the chapter's domain?
   - Does the primary anchor $T_p^\ast$ respect the stratum's recorded designation (`S1↔T5`, `S2↔T1`, `S3↔T4`, `S4↔T2`, `S5↔T3`, `S6↔T6`, `S7↔T7`) without falsely claiming the designation is arithmetically derived (`AGENTS.md` §V-9)?
3. **$\mathcal{G}_1$ Inheritance & Zero-Axis Exposure Verification:**
   - Whenever `(E, V, G)` contains a `0` (e.g., `(0, 1, 2)` on `L02`, `(3, 0, 0)` on `L09`, `(2, 0, 1)` on `L18`/`L19`/`L24`/`L25`/`L27`/`L31`), verify whether the chapter explicitly explains which ancestors in $\mathrm{Cone}(L_i)$ supply the missing axis safeguard—and whether the domain isomorphism genuinely inherits those safeguards rather than assuming them by fiat.
   - Check whether an obvious domain dependency is missing from $\mathcal{G}_1$ or whether a listed parent in the opening `> Layer NN` blockquote is idle prose not used in the chapter.
4. **$\mathcal{G}_2 = J(7,3)$ Neighbors & Terminal Sink Isolation:**
   - Inspect whether the chapter's non-ancestral $J(7,3)$ bridges represent real structural dualities (e.g., Hopfield proofreading vs Michaelis–Menten saturation vs Kingman queues) rather than decorative metaphors.
   - For the 4 parallel sinks outside $\mathrm{Cone}(L_{35})$ (`L20`, `L21`, `L22`, `L25`) and their 3 upstream-only feeders, verify the domain reason they terminate outside `L35`'s cone without breaking three-plane accountability.

## 5. Output Discipline

- Write findings exclusively using the canonical template in `.agents/rules/02-severity-rubric.md` to the path assigned by the orchestrator (or `reviews/chXX/orders-homomorphism-YYYYMMDD-HHMM.md`).
- Cite exact `file:line` locations and reference the verified `Cone(L_i)` / `J(7,3)` facts from `check_graphs.py`.
