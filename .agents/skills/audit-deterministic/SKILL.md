---
name: audit-deterministic
description: >-
  Run Layer 0 zero-LLM deterministic validation across the entire Stochology
  manuscript (.github/checks/structure.py plus check_graphs.py, check_notation.py,
  and check_crossrefs.py) and generate an append-only baseline report in
  reviews/00-deterministic-report-*.md. Invoke via /audit-deterministic.
---

# Layer 0 Deterministic Audit (`/audit-deterministic`)

Executes all zero-token mechanical validators before any specialist LLM review is launched, avoiding wasted context on broken cross-references, notation drifts, or graph defects.

## Procedure

1. Read [00-constitution.md](../../rules/00-constitution.md) (Review Mode: read-only on `book-src/` and `crates/`).
2. Run the unified Layer 0 suite in one command:
   ```bash
   python3 .agents/scripts/check_crossrefs.py --full-report
   ```
   This runs:
   - `.github/checks/structure.py` (CI gates: apparatus, spine, links, figures, Table B.1 parity, ledger, citations, contents, §V-9 concordance, §V-10 guard cardinality, math syntax, and 5-symbol orthographic checks)
   - `.agents/scripts/check_graphs.py` ($\mathcal{G}_1$ DAG acyclicity, root/sink topology, `Cone(L_i)` sizes, zero-axis `(E,V,G)` inheritance, $\mathcal{G}_2 = J(7,3)$ 12-regularity, Vandermonde counts)
   - `.agents/scripts/check_notation.py` (8 semantic symbol-collision families, `e`/`E`-value casing drift, LaTeX macro drift)
   - `.agents/scripts/check_crossrefs.py` (out-of-range prose citations, sequential `## Bridge` handoffs, unlinked prose citation density)
3. Inspect the generated `reviews/00-deterministic-report.md` (and timestamped `reviews/00-deterministic-report-YYYYMMDD-HHMM.md`).
4. Verify `git diff --name-only book-src/ crates/` is empty.
5. Present a **concise executive summary** (Anti-Compliance-DoS ceiling) listing:
   - Overall Layer-0 verdict (`PASS` or `BLOCKED`)
   - Exact blocking failures (if any) that should be fixed before running `/review-chapter`
   - Top advisory collision hotspots (`check_notation.py`) and `Bridge` warnings (`check_crossrefs.py`)
