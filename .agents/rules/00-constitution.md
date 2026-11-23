---
trigger: always_on
---

# Stochology Critical Review Constitution

This rule governs critical review and audit operations across the *Stochology* repository. The root `AGENTS.md` supersedes this file wherever they interact, including §IV (two-warrant claim discipline and prohibited apparatus), §V-9 (concordance), §V-10 (guard-site cardinality), `Book → Core` authority, and human confirmation for `SUMMARY.md` retitlings.

## 1. Strict Mode Separation (Review Mode vs. Patch Mode)

- **Review Mode (default for all audit and specialist skills):**
  - Strictly **read-only** on `book-src/`, `crates/`, `adapters/`, `consumers/`, root documents, and `.github/checks/`.
  - All reports, counterexample logs, verification tables, and triage panels are written exclusively as append-only files under `reviews/`.
  - After every automated review run, the orchestrator MUST verify `git diff --name-only book-src/ crates/` is empty. Any unauthorized edit to manuscript or core files triggers an immediate `BLOCKED` halt.
- **Patch Mode (human-authorized only):**
  - Edits to `book-src/` or `crates/` are permitted **only** when the user explicitly authorizes a separate patch turn after reviewing triage findings.
  - Proposed manuscript edits MUST obey `AGENTS.md` §IV: never cite `crates/`, Rust symbols, or `cargo test` inside `book-src/` prose, and never invent or rescale empirical numbers.

## 2. Non-Compensatory Leontief Verdict Rule

Honoring **Reflexive Brake 4** (`book-src/50-methodological-families-reflexivity.md` §9.3.4):

- **Zero scalar scores, percentages, or weighted averages.** Never trade prose clarity against a mathematical, logical, citation, or governance defect (`The minimum is not a weighted sum`).
- Categorical Leontief verdicts only (see `.agents/rules/02-severity-rubric.md` for full criteria):
  - `BLOCKED` — at least one `Level 1 (Blocking / Unparsable)` finding.
  - `STRUCTURAL-REVISION` — zero `Level 1`, at least one `Level 2 (Structural)` finding.
  - `EDITORIAL-ONLY` — zero `Level 1` and `Level 2`, at least one `Level 3 (Editorial)` finding.
  - `CLEAN` — zero verified findings across all three levels.

## 3. Evidence, Counterexample, Core-Parity & External-Source Mandates

1. **Exact Citation Location:** Every `Level 1` or `Level 2` finding MUST quote the exact file path, line range (`file:line`), section heading, and verbatim problematic text.
2. **Executable Proof & Normative Core Audit (`formal-stat`, `logic-systems`):**
   - Every formal `Level 1` or `Level 2` critique MUST include an executable Python (`sympy` / `numpy` / stdlib) verification or step-by-step algebraic derivation exposing the implicit hypothesis or counterexample.
   - Every `formal-stat` and `logic-systems` audit MUST inspect the corresponding implementation in `crates/` and run `cargo test --workspace` plus `cargo test --manifest-path crates/host-workspace/Cargo.toml`. Report any text-vs-code divergence explicitly as `[Core-Drift]` (`Book → Core` authority: the chapter governs, the core is repaired unless the chapter itself contains an error).
3. **Mandatory External Source Verification (`philosophy-sources`):**
   - Every philosophical attribution and every audited empirical value in scope MUST be cross-checked against `book-src/appendix-c-sources.md`, `book-src/appendix-d-claim-ledger.md`, AND verified externally via `search_web` or academic literature tools. Never rely on unverified memory.
4. **Adversarial Discipline:** Do not praise the prose or soften defects. Review against failure conditions, strongest rival interpretations, and boundary cases.

## 4. Anti-Compliance-DoS Cognitive Budget (10 bits/s Ceiling)

Respect the human deliberate-attention ceiling (`C_human ≈ 10 bits/s`, Ch. 95):

- Follow the compact template in `.agents/rules/02-severity-rubric.md` (metadata header, 1–2 sentence audited thesis, categorical verdict, 1 findings table, bounded evidence/verification blocks).
- Separate formal validity review from style/pedagogical review across distinct specialists.
- Never dump raw tool logs or prose walls into `reviews/`.

## 5. Append-Only Custody & 4-Phase Workflow

- **Append-only `reviews/`:** Never overwrite historical audit reports. Write timestamped artifacts (`YYYYMMDD-HHMM`), keeping at most a clearly labeled convenience pointer (such as `00-deterministic-report.md` or `triage-latest.md`) alongside timestamped records.
- **Four-Phase Editorial Protocol:**
  1. *Phase 1 — Pre-registration:* Anchor refutation targets on Prologue (`00`), Ch. `41`, Ch. `50` (the 5 Claimed Coupled Results & Hoop Tests), and Layer 0 deterministic gates (`/audit-deterministic`).
  2. *Phase 2 — Blind Parallel Cells:* Dispatch 2–3 relevant specialists per target (`/review-chapter`) without cross-contamination.
  3. *Phase 3 — Cross-Boundary Audit:* Run adjacent specialist pairs and stratum cross-checks (`/cross-check-stratum`) to catch interdisciplinary translation errors.
  4. *Phase 4 — Leontief Triage:* Fold findings non-compensatorily (`/synthesize-triage`) before requesting Patch Mode authorization.
