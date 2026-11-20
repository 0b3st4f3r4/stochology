---
name: stochology-adversarial-review
description: >-
  Federated multi-specialist adversarial review, counter-inductive stress-testing,
  and primary-source enrichment workflow for the Stochology manuscript (book-src/,
  appendices, and downstream normative core sync in crates/), supporting both
  the flagship 112-wave × 6-agent atomic per-chapter protocol (Mode S: 3 bespoke
  specialties × 1:1 Carbon/Hydrogen adversarial replication with elastic reservoir
  scheduling in research/guide.md) and 9-shard / focused multi-chapter rounds,
  governed by sustainable self-referential Swarm Stochology. Activate whenever
  the user asks (in Portuguese or English) to adversarially review, audit,
  refine, defend, stress-test, or enrich Stochology, run a review round or
  research front (F1-F23), or launch a federated swarm, run_workflow, or
  /teamwork manuscript review.
---

# Stochology Federated Adversarial Review & Enrichment

Publication-grade runbook, deterministic toolkit, and federated multi-agent package for auditing, defending, and enriching the *Stochology* manuscript (`book-src/`: 100 chapters `00`–`99`, part/stratum texts, and 7 appendices `A`–`G`, plus downstream one-way synchronization of `crates/`).

## Core Premise: Sustainable Swarm Stochology + Dual Creative-Scientific Engine

The review swarm applies Stochology reflexively to its own operation ($\mathcal{T}\_{\mathrm{Swarm}} = (S\_{\mathrm{epi}}^{\mathrm{swarm}}, S\_{\mathrm{vital}}^{\mathrm{swarm}}, S\_{\mathrm{gov}}^{\mathrm{swarm}})$):
1. **$S\_{\mathrm{epi}}^{\mathrm{swarm}}$ (Anti-Echo-Chamber & Blinded Two-Key Rigor):** Partition full-book reading into **9 balanced shards** (Mode A) or enforce **independent blind reading (`Fase α`) plus 1:1 asymmetric adversarial pairing (`Orchestrator (Carbon)` inherited on proposers `s*` ↔ `gemini-4-hydrogen-high` on adversarials `a*`; `claude-opus-*` prohibited)** per chapter wave (Mode S), pass exact `#seq` / artifact pointers instead of lossy paraphrases, and enforce strict two-key separation (`researcher/author != two-key-verifier` / `wNNN-s2 != wNNN-a2`) for verbatim primary sources (or critical-edition locators) and symbolic Python math checks.
2. **$S\_{\mathrm{vital}}^{\mathrm{swarm}}$ (Quota, Context & Attention Survival $\mathrm{TTS} \ge \mathrm{TTR}$):** Offload mechanical checks to [review_ops.py](./scripts/review_ops.py) (zero LLM tokens), regulate concurrency elastically (`K=4` >60%, `K=2` [30%, 60%], `K=1` [15%, 30%), checkpoint/pause <15% in the active Gemini reservoir) over strictly disjoint waves via `review_ops.py next-batch` (Mode S) or cap at **at most 2 disjoint waves** (Mode A), isolate authors in `workspace:"share"`, land commits before `kill`, and switch `waiting` helpers via `swarm{action:"model"}` only to finish in-flight waves or rebalance within Gemini tiers.
3. **$S\_{\mathrm{gov}}^{\mathrm{swarm}}$ (Four Swarm Brakes, Algedonic Halt & Human Sovereignty):** Zero reward for finding counts (Brake 1), **subtraction-first anti-bloat budget** ($\le 15\%$ word growth — Brake 2), mechanical-only fast-path fixes (Brake 3), zero scalar chapter scores or rubrics (Brake 4), unfiltered `[ALGEDONIC HALT]` (`S3*`, $\delta\_{\mathrm{status}}=0$), and strict human pre-approval (`ESC-NN`) for protected contracts and `SUMMARY.md` retitlings.
4. **Creativity × Scientificity (*Ars Inveniendi* × *Ars Judicandi*):** Every wave pairs **Phase W1 Creative Abduction** (simultaneous guard superposition $\bigoplus\_{j=1}^7 T\_j$ at 10 bits/s, Part V Johnson-graph $J(7,3)$ bridges, strongest named critics, reception-basin alterity $\Theta\_{\mathrm{basin}}$, and Canguilhem's creative normativity) with **Phase W2/W3 Severe Testing** (full primary texts read & verified verbatim, symbolic equation verification, and explicit failure conditions).

---

## Package Contents (Progressive Disclosure)

Read `AGENTS.md` first, then load only the files needed for your current role/stage:

- [research/guide.md](../../../research/guide.md) — **Master Constitution for Mode S (`112 × 6`):** 112-wave catalog across 6 Macro-Stages, 4-phase protocol (`α`–`δ`), elastic watermarks, and `/swarm` role cards.
- [review_ops.py](./scripts/review_ops.py) — Zero-token CLI: `baseline`, `shards`, `graph`, `waves`, `state`, `next-batch`, `wave-plan`, `wave-audit`, and `gates`.
- [swarm-stochology-charter.md](./references/swarm-stochology-charter.md) — Constitutional charter of the federated guilds, VSM/Ostrom mapping, and the Dual Engine.
- [swarm-playbook.md](./resources/swarm-playbook.md) — Blueprints for **Native `swarm` + `run_workflow`**: 9-shard cartography, specialist briefs, board/channel discipline, and quota survival rules.
- [artifact-templates.md](./resources/artifact-templates.md) — Canonical schemas for shards, `(A)`–`(J)` taxonomy, Mode A/S briefs, `[SRC]` logs, and `ESC-NN` dossiers.
- [teamwork-prompt.md](./resources/teamwork-prompt.md) — Self-contained prompt for `/teamwork` and `F1`–`F23` fronts.

---

## Execution Modes

### Mode S — Atomic Full-Book Review `112 × 6` (Flagship General Review)
1. Read `AGENTS.md` and [research/guide.md](../../../research/guide.md) (wave roles read only `AGENTS.md` + §3; coordinator reads §§4–7, Charter §4, Playbook §5). Ensure `main` has a clean committed `HEAD` before spawning `share` worktrees, and always pass **absolute paths `<MAIN_ROOT>/research/runs/...`**.
2. Initialize state & baseline in `<MAIN_ROOT>/research/runs/`:
   - `python3 .agents/skills/stochology-adversarial-review/scripts/review_ops.py baseline --out "$MAIN_ROOT/research/runs/baseline.json"`
   - `python3 .agents/skills/stochology-adversarial-review/scripts/review_ops.py state --init`
3. Query `swarm{action:"models"}`, fetch next disjoint batch (`review_ops.py next-batch --k 4 --quota-gemini <qG>`), and run `state --set Wnnn:active` (`flock`-locked) BEFORE adding workers (omitting `model` for `s*`; pinning `gemini-4-hydrogen-high` for `a*`; never `claude-opus-*`).
4. Execute up to `K` waves across Stages 1–6 (`W111` isolated last) using `[Wnnn]` board prefixes and targeted `@peer` doorbells (channels only if supported): WBC `Fase α` (`DEFEITO VERIFICÁVEL: sim|não`) → `Fase β` (`6× não` fast-path `CLEAN`, `ESC-NN`, or `brief.md`) → `Fase γ` (`s1` in `share`, `commons-delta.md` queued by `s2`, `candidate.patch` + `PATCH-SHA256`, `a1/a2/a3` `audit-a*.md`, max 2 revisions or `halted`) → `Fase δ` (`commons-steward` in `workspace:"inherit"` verifies 3/3 hashes, applies `C` + integrates, writes `verdict.md`, updates `state.json`, `kill` only `wNNN-s1..a3` — never `monitor` or `steward`; reopen resolved `halted`/`escalated` via `state --set Wnnn:pending`).
5. Run `review_ops.py gates --full-render --with-pdf` (`--with-core` at Stage 6 if core changed) and batch-update Appendices D & F at each Macro-Stage boundary.

### Mode A — Hybrid Federated `swarm` + `run_workflow` (9-Shard Multi-Chapter Rounds)
1. Read `AGENTS.md`, [swarm-stochology-charter.md](./references/swarm-stochology-charter.md), and [swarm-playbook.md](./resources/swarm-playbook.md).
2. Capture baseline & shard plan in `<artifactDir>/stochology-review/` via `python3 .agents/skills/stochology-adversarial-review/scripts/review_ops.py baseline` and `shards`.
3. Run **Step 1 (9-Shard Cartography + Cross-Part Graph Synthesis)** via `run_workflow` `parallel(...)` (plus Round 1 Appendix D primary-text audit).
4. Launch the **Federated Specialist `swarm`** (sizing guilds strictly to active disjoint waves: at most 2 concurrent waves; `commons-steward` as sole owner of Appendices C, D & F; `two-key-verifier` checking primary texts and running scratch Python math proofs; `counter-inductive-critic` co-authoring Phase W1 dialectical briefs and running Step 6 Red-Team audit).

### Mode B — Focused Front or Chapter Cluster Review
When the user scopes the task to specific chapters, strata, or fronts (`F1`–`F23`):
1. Run `review_ops.py baseline` and `review_ops.py wave-plan --edit <targets>` to lock the exact core partner neighborhood.
2. Deploy a **lean federated cell** (3–4 specialists: relevant Domain Author in `workspace:"share"`, `philology-archivist`, `counter-inductive-critic` + `two-key-verifier`, and `commons-steward`/integrator), preserving 100% of the two-key provenance, subtraction-first, and human-escalation gates.

### Mode C — Launching with `/teamwork`
When invoking `teamwork` (`invoke_subagent` with `TypeName: "teamwork"`):
1. Read [teamwork-prompt.md](./resources/teamwork-prompt.md) in full.
2. Pass `[skip-interview]` at the very start of `Prompt`, followed immediately by the contents of [teamwork-prompt.md](./resources/teamwork-prompt.md) (appending user scope constraints only if explicitly requested).

---

## Non-Negotiable Binding Gates (`AGENTS.md` Overrides All)

- **Zero manuscript pollution:** Store persistent Mode S wave state, blind memos, briefs, `[SRC-Wnnn.K]` logs, `verify_Wnnn.py` scripts, escalations, and stage reports strictly in gitignored `research/runs/` (or `<appDataDir>/brain/<conversation-id>/stochology-review/` for single-session Mode A/B rounds). Never create scratch/log files inside `book-src/`, `crates/`, or tracked repository directories (except the versioned `research/guide.md`).
- **Chapter spine (`01`–`99`):** `# Title`, intact SVG masthead (and `> Layer NN` blockquote exclusively in `51`–`85`), `## By the end of this chapter you can` with **3** numbered outcomes, **no `## Exercises` section**, escaped `\_` in inline math subscripts, and **1** `## Bridge` naming the next chapter.
- **Two warrants only (§IV):** Same-sentence primary literature citation at source precision (verbatim-logged & two-key verified) OR side-by-side reproducible algebra (symbolically verified, with explicit domain/failure conditions). Never weaken a sound thesis; narrow or delete unfounded claims and log every deletion.
- **Human pre-approval (`ESC-NN`):** Never unilaterally retitle chapters/appendices in `SUMMARY.md` (requires explicit confirmation with inline diff) or alter `AGENTS.md` §I–§III, §V-9 (concordance), §V-10, `.github/checks/`, or `Book -> Core` authority (non-citable `crates/`).
- **Local commits in Portuguese, never push.** Verify waves and rounds with:
  - `python3 .agents/skills/stochology-adversarial-review/scripts/review_ops.py wave-audit --files <edited> --baseline <artifactDir>/stochology-review/baseline.json`
  - `python3 .agents/skills/stochology-adversarial-review/scripts/review_ops.py gates --full-render --with-pdf` (plus `--with-core` whenever `crates/` syncs to the book).
