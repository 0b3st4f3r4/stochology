---
name: review-chapter
description: >-
  Orchestrate a blind parallel critical review cell (2-3 targeted specialists
  selected by deterministic routing) for a single Stochology chapter (00-99)
  or appendix (A-G), enforcing Review Mode isolation, crates/ parity checks,
  mandatory external source verification, and non-compensatory Leontief folding.
  Invoke via /review-chapter <target>.
---

# Parallel Chapter Review Cell (`/review-chapter <target>`)

Deploys **only the 2–3 specialists matched to the target's epistemic, formal, institutional, or editorial profile** in blind parallel isolation, preventing cognitive saturation (`Compliance DoS`, Ch. 95) and keeping formal verification cleanly separated from editorial prose review.

## 1. Deterministic Specialist Routing Matrix

| Target Scope | Assigned Blind Parallel Cell (2–3 Specialists) |
| :--- | :--- |
| Prologue `00`, Part Intros/Conclusions, Stratum Openings | `coherence-editor` + `philosophy-sources` |
| Part I (`01`, `04`) | `coherence-editor` + `philosophy-sources` |
| Part I (`02`, `03`, `05`, `06`) | `coherence-editor` + `formal-stat` |
| Part I (`07`, `08`) | `coherence-editor` + `redteam-gov` |
| Part II Math (`09`–`15`) | `logic-systems` + `formal-stat` |
| Part II Philosophy (`16`–`18`) | `philosophy-sources` + `logic-systems` |
| Part II Ethics & Institutions (`19`–`20`) | `philosophy-sources` + `redteam-gov` |
| Part II Natural Science (`21`–`24`) | `logic-systems` + `formal-stat` |
| Part II Evolution & Regulation (`25`–`26`) | `logic-systems` + `formal-stat` + `philosophy-sources` |
| Part III Epistemology & Cause (`27`–`29`, `39`) | `philosophy-sources` + `coherence-editor` (use `formal-stat` instead of `coherence-editor` on `39`) |
| Part III Stats & Inference (`30`–`32`, `34`–`37`) | `formal-stat` + `philosophy-sources` |
| Part III Measurement & Time (`33`, `38`) | `formal-stat` + `logic-systems` + `philosophy-sources` |
| Part III Institutions (`40`) | `redteam-gov` + `philosophy-sources` |
| Part III/IV Keystones (`41`, `50`) | `formal-stat` + `logic-systems` + `redteam-gov` (plus `philosophy-sources` on `50` §9.5–§9.7) |
| Part IV Framing & Evidence (`42`–`45`) | `formal-stat` + `philosophy-sources` (add `coherence-editor` on `42`) |
| Part IV Dynamics & Vital (`46`–`47`) | `logic-systems` + `formal-stat` |
| Part IV Governance (`48`–`49`) | `redteam-gov` + `formal-stat` (on `48`) or `philosophy-sources` (on `49`) |
| Part V `S1` (`51`–`55`) | `orders-homomorphism` + `logic-systems` |
| Part V `S2` (`56`–`60`) | `orders-homomorphism` + `logic-systems` + `formal-stat` |
| Part V `S3` (`61`–`65`) | `orders-homomorphism` + `logic-systems` + `philosophy-sources` |
| Part V `S4` (`66`–`70`) | `orders-homomorphism` + `formal-stat` + `philosophy-sources` |
| Part V `S5` (`71`–`75`) | `orders-homomorphism` + `philosophy-sources` + `coherence-editor` |
| Part V `S6` (`76`–`80`) | `orders-homomorphism` + `formal-stat` + `redteam-gov` |
| Part V `S7` (`81`–`85`) | `orders-homomorphism` + `redteam-gov` + `logic-systems` (add `philosophy-sources` on `81`–`82` if time permits) |
| Part VI Laminar Trajectory (`86`–`92`) | `logic-systems` + `coherence-editor` (swap `coherence-editor` for `redteam-gov` on `91`–`92`) |
| Part VII Superposition (`93`–`96`) | `logic-systems` + `redteam-gov` (add `formal-stat` on `94`–`95`) |
| Part VII Exception & Target (`97`–`98`) | `redteam-gov` + `philosophy-sources` |
| Part VII Closing (`99`) | `philosophy-sources` + `coherence-editor` + `redteam-gov` |
| Appendix A | `formal-stat` + `logic-systems` |
| Appendix B | `logic-systems` + `orders-homomorphism` |
| Appendix C, Appendix D | `philosophy-sources` + `formal-stat` |
| Appendix E, Appendix F, Appendix G | `coherence-editor` + `philosophy-sources` |

## 2. Execution Protocol

1. **Resolve Target & Gather Layer-0 Context (Zero-Token Pre-Filter):**
   - Resolve target file under `book-src/` and compute output directory `reviews/chXX/` (or `reviews/appendix-X/` / `reviews/front-matter/`).
   - Run deterministic pre-checks for the target:
     ```bash
     python3 .agents/scripts/check_notation.py --file <target> --markdown
     python3 .agents/scripts/check_crossrefs.py --file <target> --markdown
     ```
     and if target is in Part V (`51`–`85`), also run:
     ```bash
     python3 .agents/scripts/check_graphs.py --layer Lnn --markdown
     ```
2. **Spawn Blind Parallel Subagents (`invoke_subagent`):**
   - Define/configure each selected specialist cleanly so their tool access matches their mandate while enforcing Review Mode:
     - `orders-homomorphism`, `redteam-gov`, `coherence-editor`: `define_subagent` with `EnableWriteTools: false` (mechanically read-only) — return their completed report in their final message so the orchestrator writes it to `reviews/<scope>/<agent>-YYYYMMDD-HHMM.md`.
     - `formal-stat`, `logic-systems`: `define_subagent` with `EnableWriteTools: true` (required for running `python3`/`sympy`/`numpy` counterexamples and `cargo test`), strictly instructed by `00-constitution.md` NEVER to edit `book-src/` or `crates/` and to write **only** to `reviews/<scope>/<agent>-YYYYMMDD-HHMM.md`.
     - `philosophy-sources`: invoke via `TypeName: "self"` (retains `search_web` and literature tools for **mandatory external source verification**), strictly bound by `00-constitution.md` to read-only on `book-src/`/`crates/` and writing only to `reviews/<scope>/philosophy-sources-YYYYMMDD-HHMM.md`.
   - Pass **blind isolation instructions**: each subagent inspects the target chapter, relevant governing/neighbor chapters, `AGENTS.md`, `.agents/rules/00..02`, its own `.agents/skills/<specialist>/SKILL.md`, and the Layer-0 outputs — **never** another specialist's in-flight report.
3. **Wait & Persist Append-Only Artifacts:**
   - Stop calling tools until all parallel subagents reply; then ensure each report exists under `reviews/<scope>/<agent>-YYYYMMDD-HHMM.md` following `.agents/rules/02-severity-rubric.md`.
4. **Custody Gate (Mandatory):**
   - Run `git diff --name-only book-src/ crates/` and confirm **zero modified tracked source files**.
5. **Leontief Fold Synthesis:**
   - Fold the 2–3 specialist verdicts strictly by `min` (any `Level 1` → `BLOCKED`), write `reviews/<scope>/summary-YYYYMMDD-HHMM.md` highlighting cross-specialist intersections, and report concisely to the user.
