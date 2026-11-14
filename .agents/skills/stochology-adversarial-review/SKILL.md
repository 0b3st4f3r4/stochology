---
name: stochology-adversarial-review
description: >-
  Coordinated whole-book adversarial review and primary-source enrichment workflow
  for the Stochology manuscript (book-src/, appendices, and normative core sync in
  crates/), featuring persistent Book Map rounds, 2-to-3-chapter deep-relation waves,
  two-key primary-source citation verification, human-gated architectural revisions,
  and full AGENTS.md structural/surface gates. Activate whenever the user asks to
  adversarially review, audit, refine, defend, or enrich Stochology, run a review
  round or research front (F1-F14), or launch a /teamwork or multi-agent manuscript review.
---

# Stochology Adversarial Review & Enrichment

Reusable runbook and prompt package for conducting coordinated, publication-grade adversarial review and primary-source enrichment rounds on the Stochology manuscript (`book-src/`, appendices, part texts, and downstream normative core synchronization in `crates/`).

## Contents

- [teamwork-prompt.md](./resources/teamwork-prompt.md) — Full multi-round execution prompt (operating rules, repair doctrine, primary-source research protocol, embedded research guide, 6-step round structure, acceptance criteria, and stop conditions).
- [research-guide.md](./references/research-guide.md) — Standalone reference copy of the Research Guide: 3 gain modes, 7 human-gated architectural commitments, 14 research fronts (`F1`–`F14`), 5-tier prioritization, and the `F<n>.<k>` opportunity record template.

## Launching with `/teamwork`

When invoking `teamwork` (`invoke_subagent` with `TypeName: "teamwork"`):

1. Read [teamwork-prompt.md](./resources/teamwork-prompt.md) in full.
2. Pass `[skip-interview]` at the very start of `Prompt`, followed immediately by the contents of [teamwork-prompt.md](./resources/teamwork-prompt.md).
3. Only narrow scope (for example to specific chapters, parts, or research fronts `F1`–`F14`) when the user explicitly requests a focused run; keep all binding rules, verification gates, and human pre-approval gates intact.

## Direct or `swarm` Execution

When executing the workflow directly or orchestrating a `swarm` without `/teamwork`:

1. Follow the 6-step round structure in [teamwork-prompt.md](./resources/teamwork-prompt.md):
   - **Step 1 — Whole-book pass:** Sequentially map every file in reading order into a persistent Book Map (claims + warrants, concepts/notation, dependencies/Bridge, tensions, and `F1`–`F14` tags), build the cross-part relation graph and severity-ranked findings list (taxonomy `(A)`–`(G)`), and in Round 1 audit all existing Appendix D ledger rows against primary texts (`AGENTS.md` §V-6).
   - **Step 2 — Wave segmentation:** Group edits into waves of 2–3 chapters linked by deep graph relations or shared research fronts, naming full-read relation partners. Run at most two concurrent waves, strictly with disjoint edit sets and partner sets. Use a two-key workflow: whoever inserts a citation never verifies it.
   - **Step 3 — Shared appendices:** Route all Appendix C (bibliography) and Appendix D (claim ledger) additions/updates through a single owner who deduplicates, unifies citation style, and enforces §V-6 parity.
   - **Step 4 — Integration & gates:** Merge each wave and verify every structural, provenance, and surface gate before starting a wave touching overlapping files.
   - **Step 5 — Core sync:** Whenever the manuscript changes productions, the ordered acceptance predicate, the diagnostic fold, guard sites, or the Triad, bring `crates/` into agreement with the book (never the reverse) and pass offline Rust and CI wasm checks.
   - **Step 6 — Round close:** Run a red-team re-read of all round diffs against `AGENTS.md` and the prompt, write the round report, and decide whether to start the next round or stop.
2. Consult [research-guide.md](./references/research-guide.md) for research fronts `F1`–`F14`, working leads (all unverified until primary texts are read), priority tiers, and the `F<n>.<k>` opportunity log format (`incorporada | descartada | adiada | escalada`).

## Binding Rules & Verification Gates

- **Read `AGENTS.md` in full before any edit**; `AGENTS.md` overrides this skill on any conflict.
- **Clean repository artifacts:** Store the Book Map, findings, source log, `F<n>.<k>` records, and round reports strictly in the conversation artifact directory (`<appDataDir>/brain/<conversation-id>/`), **never** in the git repository.
- **Primary-source research discipline:** Use web search strictly to discover candidate sources. Only quote or take numeric values after retrieving the source's full primary text (publisher page, PDF, DOI via doi.org/Crossref, or critical edition with canonical locator such as Stephanus/Kühn) and logging the verbatim passage and locator. Never cite abstract-only, paywalled, secondary AI summaries, or unretrievable sources.
- **Repair discipline:** Never weaken a well-founded claim. Narrow or delete unfounded claims without inventing replacement numbers; record every deletion in the round report.
- **Human pre-approval gates:** Never unilaterally modify chapter/appendix titles (`SUMMARY.md` parity), or alter architectural commitments (§I Triad, Collapse Propositions, Leontief minimum, diagnostic-not-control-law status, mediation circuit, 7 guard sites, 35 orders/7 strata, `AGENTS.md`, checks, or `crates/`). Escalate any grounded architectural proposal with primary-source/algebraic rationale and full blast radius for explicit human pre-approval before executing coherently across manuscript, appendices, `AGENTS.md`, checks, and `crates/`.
- **Local commits only:** Write commit messages in Portuguese (one commit per coherent repair). Never push unless explicitly instructed.
- **Verification commands (baseline before edits, and after every wave):**
  - `python3 .github/checks/structure.py` (must pass with zero new advisories beyond baseline)
  - `mdbook build` and `python3 .github/checks/surface.py book` (§V-8)
  - When `crates/` changes to follow the book: `cargo test --offline`, `cargo test --doc --offline -p stoch-core`, and affected CI wasm gates.
