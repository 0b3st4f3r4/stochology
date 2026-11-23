---
trigger: model_decision
description: Objective non-compensatory severity rubric (Levels 1-3), standardized failure tags, categorical Leontief verdicts, and concise specialist report template for Stochology reviews. Load during any chapter, stratum, or triage review.
---

# Severity Rubric & Canonical Specialist Report Schema

Applies strictly under `.agents/rules/00-constitution.md`. Numeric scores, weighted rubrics, and scalar chapter ratings are **prohibited** (Reflexive Brake 4).

## 1. Standardized Failure Tags

Every row in a specialist's findings table MUST carry exactly one primary tag:

| Tag | Meaning |
| :--- | :--- |
| `[Formal/Proof]` | Mathematical error, invalid statistical hypothesis, unstated domain condition, logical contradiction, broken LL(1) / 5-stage predicate or graph invariant |
| `[Citation/Source]` | Misattribution, strawman of cited author, altered/rounded/unwarranted literature numeral (§IV violation), missing Appendix C/D alignment, or failed external verification |
| `[Forced-Analogy]` | Superficial Part V domain mapping where `{T_i,T_j,T_k}`, `(E,V,G)`, or `Cone(L_i)` fails to model genuine domain collapse dynamics |
| `[Notation]` | Symbol collision without local disambiguation, MathJax/CommonMark defect, or §V-9 concordance violation |
| `[Practical-Viability]` | Unmitigated governance capture, incentive incompatibility, Compliance DoS breach, or operational implausibility |
| `[Style/Clarity]` | Redundancy across parts, unclear pedagogical progression, weak `Bridge` handoff, or mismatch with `By the end of this chapter you can` |
| `[Core-Drift]` | Discrepancy between manuscript specification (`book-src/`) and normative Rust implementation/tests (`crates/`), or failing `cargo test` |

## 2. Objective Severity Levels

- **`Level 1 (Blocking / Unparsable)`** — Halts chapter/stratum acceptance (`BLOCKED`):
  - Proven mathematical, statistical, or logical error; executable Python/SymPy counterexample falsifying a claimed bound, theorem, or invariance.
  - Internal contradiction or violation of `AGENTS.md` §I–§IV or §V-9 Concordance.
  - Verified `[Citation/Source]` distortion or invented/altered empirical value.
  - Any `crates/` test failure or normative discrepancy between `book-src/` and `crates/` (`[Core-Drift]`).
  - Broken $\mathcal{G}_1$ acyclicity, wrong $J(7,3)$ degree, missing `Cone(L_i)` axial coverage without domain justification, or Layer 0 structural check failure.
- **`Level 2 (Structural)`** — Requires substantive argument revision (`STRUCTURAL-REVISION`):
  - Unstated necessary hypothesis or domain boundary (e.g., forgetting $f \in (0,1)$, $\alpha > 1$, or finite-variance assumptions).
  - Missing critical counterexample, omitted strongest philosophical/disciplinary objection, or interdisciplinary translation mismatch between math and institutional norm.
  - Strained Part V isomorphism (`[Forced-Analogy]`) or unmitigated adversarial capture pathway (`[Practical-Viability]`).
  - Ambiguous symbol collision inside the same chapter that risks misreading a derivation.
- **`Level 3 (Editorial)`** — Non-blocking polish (`EDITORIAL-ONLY`):
  - Redundant prose, unclear explanation in Part I, minor symbol glossing, or `Bridge` / outcome phrasing improvements.

## 3. Categorical Leontief Folding Rule

Compute verdict strictly by the worst severity present across all findings for the audited scope:

```text
if any(Level 1)       -> BLOCKED
elif any(Level 2)     -> STRUCTURAL-REVISION
elif any(Level 3)     -> EDITORIAL-ONLY
else                  -> CLEAN
```

## 4. Canonical Specialist Report Template (Anti-Compliance-DoS)

Every specialist writes **only** this bounded structure to `reviews/<scope>/<agent>-YYYYMMDD-HHMM.md`. No preamble praise; keep prose tight and falsifiable.

```markdown
# Review Report — <Scope: e.g. Chapter 50 / Stratum S2 / Appendix B>

- **Specialist:** `<skill-name>`
- **Date (UTC/Local):** `YYYY-MM-DD HH:MM`
- **Target File(s):** `<book-src/...>`
- **Leontief Verdict:** `BLOCKED` | `STRUCTURAL-REVISION` | `EDITORIAL-ONLY` | `CLEAN`

## Audited Central Thesis
1–2 crisp sentences stating the exact claim or invariant audited in this pass.

## Findings

| ID | Severity | Tag | Location (`file:line` / `§`) | Defect & Concrete Repair Direction |
| :--- | :--- | :--- | :--- | :--- |
| `F1` | `Level 1` | `[Formal/Proof]` | `book-src/...:123 (§X.Y)` | Concise statement of defect + exact mathematical/textual repair (never cite `crates/` in proposed manuscript text). |

*(If no findings: single line `No verified defects within this specialist's scope.`)*

## Verification & Counterexamples
- **Commands / Checks Run:** exact commands executed (`python3 ...`, `cargo test ...`, Layer 0 scripts) and observed exit status/summary.
- **Executable Derivation / Counterexample:** concise Python snippet or step-by-step symbolic proof for every `Level 1`/`Level 2` `[Formal/Proof]` finding.

## Normative Core (`crates/`) Parity *(Mandatory for `formal-stat` and `logic-systems`)*
- **Workspace Suites Checked:** `cargo test --workspace` and `cargo test --manifest-path crates/host-workspace/Cargo.toml` results.
- **Parity Findings:** `MATCH` or `[Core-Drift]` details mapping `crates/<crate>/src/...` against `book-src/...` (observing `Book → Core` authority).

## Primary-Source Verification Log *(Mandatory for `philosophy-sources`; optional elsewhere)*

| Source / Numeral Cited | Location (`file:line`) | Appendix C/D Status | External Verification (`VERIFIED-EXTERNAL` / `MISMATCH` / `UNVERIFIABLE-EXTERNAL`) | Locator / Query & Notes |
| :--- | :--- | :--- | :--- | :--- |
```
