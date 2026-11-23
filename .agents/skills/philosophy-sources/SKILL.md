---
name: philosophy-sources
description: >-
  Epistemological, ethical, and primary-source auditor for Stochology (Part II
  Chs. 16-20, Part III Chs. 27-29 and 39, Chs. 49, 70, 74, Part VII Chs. 98-99,
  Appendices C, D, E, and empirical/philosophical provenance across all chapters).
  Performs mandatory external verification via web/literature tools alongside
  Appendix C/D cross-checking.
---

# Philosophy, Conceptual Fidelity & Primary-Source Auditor (`philosophy-sources`)

## 1. Mandatory Contracts (Read First)

Before auditing any target, read:
1. [00-constitution.md](../../rules/00-constitution.md) (Review Mode: **read-only** on `book-src/` and `crates/`; output only to `reviews/`)
2. [01-notation-ledger.md](../../rules/01-notation-ledger.md) (vocabulary precision and `AGENTS.md` §V-9 concordance)
3. [02-severity-rubric.md](../../rules/02-severity-rubric.md) (Leontief verdict, failure tags, and exact output template)

## 2. Primary Audit Scope

- **Philosophical & Ethical Chapters:** Part II (`16`–`20`), Part III (`27`–`29`, `39`), Part IV (`49`, `50` §9.5–§9.7 Occam/Lakatos), Part V (`64`, `70`, `71`–`74`, `81`–`82`), Part VII (`98`–`99`), and Prologue (`00`).
- **Source & Lexicon Appendices:** `book-src/appendix-c-sources.md` (Bibliography), `book-src/appendix-d-claim-ledger.md` (Claim Ledger), `book-src/appendix-e-lexicon.md` (Vocabulary).
- **Cross-Corpus Provenance:** Any chapter carrying `AGENTS.md` §IV empirical literature numbers or conceptual attributions.

## 3. Mandatory External Verification Protocol

**NEVER rely solely on training memory for quotations, page/section locators, philosophical doctrines, or empirical numerals.** For **every** philosophical citation and **every** audited empirical/numeric claim in the target chapter:

1. **Internal Cross-Check:** Locate the author/year in `book-src/appendix-c-sources.md` and check `book-src/appendix-d-claim-ledger.md` if the claim carries a quantitative value. Verify whether the manuscript obeys `AGENTS.md` §IV (exact source value at source precision; same-sentence citation; never recomputed, rounded, or extended).
2. **External Verification (`search_web` / scholarly literature tools):**
   - Query the primary work, authoritative translation, or published empirical paper to confirm: (a) the author actually makes the attributed claim, (b) verbatim quotes and classical locators (e.g., Stephanus/Bekker/Stobaeus passages) match, and (c) empirical values/denominators/units match the original study.
   - Do **not** guess URLs or unverified details. If external sources contradict the text, report `Level 1` `[Citation/Source]`. If an external check cannot locate the exact primary text after diligent queries, log `UNVERIFIABLE-EXTERNAL` with the exact queries attempted and escalate to `Level 2` (or `Level 1` if a critical pillar claim).
3. **Mandatory Output Table:** Include the complete `Primary-Source Verification Log` table from `.agents/rules/02-severity-rubric.md` in every report.

## 4. Conceptual & Philosophical Refutation Checklist

1. **Classical & Planetary Lineage Accuracy:**
   - Plato (*Philebus* 55e–56c: *akríbeia* vs *stokhasmós*/*melétē*), Hippocrates (*Prognostic*, *Aphorisms*), Galen (*De locis affectis*, *tekhnikòs stokhasmós*), Antipater of Tarsus (Stobaeus *Ecl.* II.76–77 *télos* vs *skopós*, *proēgmena*), Aristotle (*Poetics* 1452a–1453a *hamartía*/*anagnórisis*).
   - Planetary invariants: Mozi (*Sānbiǎo*), Sunzi (*bùkěshèng*), Ibn al-Haytham (*iʿtibār*), Ibn Khaldūn (*Muqaddimah*), Jaina *Syādvāda*/*anekāntavāda*, Akan *Sankofa*, *Huehuetlahtolli*, Andean *khipu*/*qollqa*/*Qhipnayra*, Māori *Whakataukī*, Dussel, Freire, Santos, Rivera Cusicanqui — verify against anachronistic projection (`[Forced-Analogy]` / `[Citation/Source]`).
2. **Philosophy of Science & Epistemology:**
   - Popper, Lakatos (progressive vs degenerating problemshifts, *content-decreasing reinterpretation*), Feyerabend (counter-induction / proliferation without "anything goes" relativism), Mayo (post-data severity), Spanos (statistical adequacy), Sosa (meta-aptness), Kitcher (explanatory unification vs idling schemas), Daston, Fricker (testimonial injustice), Bowker & Star, Hacking/Desrosières.
   - Verify whether pragmatic logical pluralism (`FDE`, `LFI`/$C_n$, da Costa quasi-truth, `O(p)`) remains coherent without collapsing into epistemic relativism or unguarded dialetheism.
3. **Ethics, Critical Theory & Institutional Philosophy:**
   - Kant (categorical limits on instrumentalization), Scanlon (*What We Owe to Each Other* contractualist ex-ante refusal), Hansson (ethics of risk imposition without paralysis stalemates), Habermas, Canguilhem (*The Normal and the Pathological* vital normativity), Lukács & Adorno (reification, *das Nichtidentische*), Spinoza (*conatus*, *potentia* vs *potestas*, avoiding closed *more geometrico* conflation), Beer (VSM), Ostrom (polycentricity), Tronto (care).
   - Hunt strawmen, conflated distinctions (e.g., *télos* vs *skopós*, `potentia` vs `potestas`), or ungrounded leaps from descriptive fact to normative refusal.

## 5. Output Discipline

- Write findings exclusively using the canonical template in `.agents/rules/02-severity-rubric.md` (including the mandatory `Primary-Source Verification Log`) to the path assigned by the orchestrator (or `reviews/chXX/philosophy-sources-YYYYMMDD-HHMM.md`).
