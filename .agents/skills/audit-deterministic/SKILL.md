---
name: audit-deterministic
description: >-
  Run Layer 0 zero-LLM deterministic validation across the entire Stochology
  manuscript (.github/checks/structure.py plus check_graphs.py, check_notation.py,
  check_symbolic_math.py, check_style_and_citations.py, check_custody_provenance.py,
  and check_crossrefs.py) and generate an append-only baseline report in
  reviews/00-deterministic-report-*.md. Invoke via /audit-deterministic.
---

# Layer 0 Deterministic Audit (`/audit-deterministic`)

Executes all zero-token mechanical validators before any specialist LLM review is launched, avoiding wasted context on broken cross-references, notation drifts, broken proofs, watermark/em-dash regressions, or graph defects.

## Procedure

1. Read [00-constitution.md](../../rules/00-constitution.md) and [03-ai-custody-workflow.md](../../rules/03-ai-custody-workflow.md) (Review Mode: read-only on `book-src/` and `crates/`).
2. Run the unified Layer 0 suite in one command:
   ```bash
   python3 .agents/scripts/check_crossrefs.py --full-report
   ```
   This runs all 7 deterministic checks:
   - `.github/checks/structure.py` (CI gates: apparatus, spine, links, figures, Table B.1 parity, ledger, citations, contents, §V-9 concordance, §V-10 guard cardinality, math syntax, and 5-symbol orthographic checks)
   - `.agents/scripts/check_graphs.py` ($\mathcal{G}_1$ DAG acyclicity, root/sink topology, `Cone(L_i)` sizes, zero-axis `(E,V,G)` inheritance, $\mathcal{G}_2 = J(7,3)$ 12-regularity, Vandermonde counts)
   - `.agents/scripts/check_notation.py` (10 automated semantic symbol-collision families matching the ledger in `01-notation-ledger.md`, `e`/`E`-value casing drift, LaTeX macro drift)
   - `.agents/scripts/check_symbolic_math.py` (SymPy/NumPy symbolic and numerical proofs of 13 core Stochology mathematical claims)
   - `.agents/scripts/check_style_and_citations.py` (running-prose em-dashes, invisible/homoglyph watermarks across `.md`/`.svg`/`README.md`/`CONTRIBUTING.md`, Appendix C DOI/arXiv syntax, advisory LLM clichés)
   - `.agents/scripts/check_custody_provenance.py` (Tier-1 vs Tier-2 custody classification including diff-hunk auto-promotion, line/word prose-bloat delta, Git trailers)
   - `.agents/scripts/check_crossrefs.py` (out-of-range prose citations, sequential `## Bridge` handoffs, unlinked prose citation density)
3. Inspect the generated `reviews/00-deterministic-report.md` (and timestamped `reviews/00-deterministic-report-YYYYMMDD-HHMM.md`).
4. Verify `git status --porcelain --untracked-files=all -- ':!reviews/'` is empty.
5. Present a **concise executive summary** (Anti-Compliance-DoS ceiling) listing:
   - Overall Layer-0 verdict (`PASS` or `BLOCKED`)
   - Exact blocking failures (if any) that should be fixed before running `/review-chapter`
   - Top advisory collision hotspots (`check_notation.py`), style/provenance alerts, and `Bridge` warnings (`check_crossrefs.py`)
