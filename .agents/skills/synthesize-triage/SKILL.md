---
name: synthesize-triage
description: >-
  Consolidate recent Layer 0 reports and specialist reviews under reviews/ into
  an executive Leontief triage panel (Level 1 blocking queue at top, cross-chapter
  systemic clusters, Level 2 structural queue, Level 3 editorial backlog, and
  explicit Patch Mode authorization checklist). Invoke via /synthesize-triage.
---

# Non-Compensatory Leontief Triage Panel (`/synthesize-triage`)

Consolidates multi-agent review outputs into one decision panel under **Phase 4 (Non-Compensatory Synthesis)** while respecting the `10 bits/s` Anti-Compliance-DoS ceiling (`Ch. 95`) and **Reflexive Brake 4** (`00-constitution.md`: zero numeric scores or averages).

## Procedure

1. Read [00-constitution.md](../../rules/00-constitution.md) and [02-severity-rubric.md](../../rules/02-severity-rubric.md).
2. Discover and inspect all existing reports under `reviews/`:
   - `reviews/00-deterministic-report*.md` (latest Layer 0 baseline)
   - `reviews/ch*/*.md`
   - `reviews/stratum-*/*.md`
   - `reviews/cross-boundary/**/*.md`
   - `reviews/appendix-*/*.md`
3. Fold verdicts non-compensatorily per audited unit (`any Level 1 → BLOCKED`; else `any Level 2 → STRUCTURAL-REVISION`; else `any Level 3 → EDITORIAL-ONLY`; else `CLEAN`).
4. Write an append-only artifact `reviews/triage-YYYYMMDD-HHMM.md` AND refresh `reviews/triage-latest.md` with strictly these four sections:
   - **Section 1 — Blocking Queue (`Level 1 / BLOCKED`):** Ordered by structural blast radius (Layer 1 Formal Core & Ch. `50` Hoop Tests → Layer 2 Philosophy/Governance & `[Core-Drift]` → Layer 3 Part V Orders & Layer 0 mechanical blocks). Include exact `file:line`, tag, failing verification/counterexample pointer, and owning specialist report link.
   - **Section 2 — Cross-Chapter Systemic Clusters (Batch-Fix Candidates):** Group recurring findings spanning $\ge 2$ chapters (e.g., recurring symbol collisions, shared `[Citation/Source]` errors, `Cone(L_i)` gaps, or `[Core-Drift]`) so the human author can authorize one clean cross-cutting patch.
   - **Section 3 — Structural (`Level 2`) & Editorial (`Level 3`) Queues:** Compact summary tables capped to avoid wall-of-text cognitive overload (linking back to raw specialist files for full detail).
   - **Section 4 — Patch Mode Authorization Checklist:** Concrete, discrete repair batches waiting for explicit user approval. **Reminder:** Review Mode remains active — make zero edits to `book-src/` or `crates/` during `/synthesize-triage`.
5. Verify `git status --porcelain --untracked-files=all -- ':!reviews/'` is empty and present Section 1 + Section 4 cleanly to the user.
