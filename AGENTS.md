# AGENTS.md — Governance, Editorial, and Pedagogical Contract (Stochology)

This file is the binding contract for any agent or contributor working in this repository.

**Stochology** (from *stókhos*, aim or conjecture, and *lógos*, reasoned account) is an **integrative framework and transversal research program for the theory and ethics of aim** — *stókhos* from Plato's *Philebus* 55e–56c, the Hippocratic *sēmeîa*, *kairós* and *mētis*, the Galenic *stokhastikē tékhnē*, Antipater of Tarsus's archer, and Bernoulli's *Ars Conjectandi* — for judgment and decision-making under structural uncertainty when action must precede the outcome. Its organizing object is the **full mediation circuit across world, archive, and institution**, and the contract treats that indivisibility as inherited rather than discovered: Vickers had already described institutional regulation as the appreciation of a whole situation before any part of it is decided (Vickers 1965), Beer argued it from viable systems, Simon from bounded rationality, and the institutional economists from the irreducibility of rules and organizations.

**The repository is a manuscript, not a program.** It holds eighty-six numbered chapters and five appendices in `book-src/`, the figures those chapters reference, and the book configuration. What the program claims for itself is therefore what the text can show: a mediation circuit argued against the sources that established each of its parts, with every value either quoted from a named source or re-derivable from the algebra displayed beside it.

**The core beside the manuscript.** Beside that text the repository also carries a normative core: the productions, the ordered acceptance predicate and the diagnostic fold, transcribed as executable code and checked by the same continuous integration that gates the text. Its standing is narrow and is stated here so that no later reader mistakes it for something larger. It is **not evidence** — no value in a chapter is quoted from it, and none of the checks in §V reads it to decide anything about the text. It is **not authority** — the rule of §II stands unchanged, and where the core and a chapter disagree the core is repaired, because the chapter is where the claim, its source and its derivation sit together. It is **not citable** — the prohibition of §IV reaches it, which is why this paragraph names no directory, no build step and no file, and why the scan of §V-1 runs over this file too. What the core buys is the one thing prose cannot do for itself: a stated production can be run against every boundary case its authors could construct, so that a sentence claiming a fold is the minimum can fail on a machine and not only on a reader. That is a stronger check of the text's own claim, and it is still a check of the claim, not a source for it.

---

## I. The Nuclear Triad and the Three Collapse Propositions

Stochology unites three historical lineages (Epistemological/Statistical, Physical/Cybernetic, Critical/Ethical) into a **Nuclear Triad** of three non-substitutable safeguards:

$$\mathcal{T}\_{\mathrm{Stochology}} = \bigl(\text{Epistemic Margin},\mkern5mu \text{Vital Margin},\mkern5mu \text{Institutional Governance}\bigr), \qquad M\_{\mathrm{Leontief}} = \min\bigl(S\_{\mathrm{epi}}, S\_{\mathrm{vital}}, S\_{\mathrm{gov}}\bigr)$$

Removing any single axis is argued to trigger a specific structural collapse, and each proposition is stated so that it could fail:

1. **Collapse Proposition 1 ($\neg\text{Epistemic Margin} \implies \text{Blind Undersizing}$):** an institution that maintains physical reserves sizes them against a false point certainty when it ignores non-random missingness ($\rho\_{R,Y} \ne 0$), cluster design effects ($\mathrm{DEFF}$), and partial-identification bounds.
2. **Collapse Proposition 2 ($\neg\text{Vital Margin} \implies \text{Impotent Lucidity Before Ruin}$):** knowing the exact width of uncertainty while operating at full utilization or excess leverage, with $\mathrm{TTS}\_{\mathrm{survival}} < \mathrm{TTR}\_{\mathrm{recovery}}$, ends at the ruin barrier.
3. **Collapse Proposition 3 ($\neg\text{Institutional Governance} \implies \text{Cynical Capture and Pillaging}$):** epistemic doubt is weaponized as an agnotological alibi while common vital reserves are pillaged under limited-liability moral hazard, high-powered proxy gaming, and testimonial discounting of frontline warnings.

The triadic verdict is **diagnostic, not a control law**. It is folded from a finished trajectory and reports which plane failed; acting is done by the per-step canonical operator, never by the aggregate. Any text that turns the diagnosis into a decision rule, or that lets a margin aggregate drive an action, has broken this clause.

---

## II. The Corpus and the Authority Rule

**Six layers, one sequence of chapters.** Chapters `27` and `40` hold the ethical and interpersonal refusals. Chapters `29`, `41` and `50` map the three-plane circuit, the Nuclear Triad, the three collapse propositions, the seven guard sites, the four reflexive brakes and the four falsifiers. Appendix B.1 fixes the syntax layer over trajectories, stating the productions, the ordered acceptance predicate and the non-commutation property. Chapter `41` isolates the epistemic transformation that precedes the shot, why the regress of higher-order uncertainty terminates only on the agent's own circuit (*télos*), the `3 → 7` inventory and the three regimes of action. Part V with Appendix B.2 records the $\binom{7}{3} = 35$ triadic combinations against the $7 \times 5 = 35$ stratification of knowledge and the seven stratum anchors as designations. Chapters `20`, `40`, `49` and `85` carry the political philosophy and the institutional cybernetics. Appendix E defines the classical, technical and political vocabulary.

**The authority rule.** A chapter may cite an appendix; **an appendix may not cite a chapter as authority.** Where Appendix B or Appendix E and a chapter disagree about a definition, the chapter governs the definition and the appendix is corrected, because the chapter is where the claim, its source and its derivation sit side by side. Relative links must resolve after percent-decoding, including targets whose path carries an encoded space — a link checker that does not decode `%20` will report false orphans.

**Book structure.** One numbered sequence, `00`–`85`, in five parts, plus the preface at `00` and five appendices:

| Part | Chapters | Content |
| :--- | :--- | :--- |
| Preface | `00` | Genealogy, Nuclear Triad, three collapse propositions, three canonical operators |
| I — Thinking Under Uncertainty | `01`–`08` | The intuitive entry: deciding without knowing, averages and tails, outcome luck, non-substitutable margins |
| II — The Bases | `09`–`26` | Mathematics (`09`–`15`), philosophy and epistemology (`16`–`20`), natural science (`21`–`26`) |
| III — Foundations of Judgment | `27`–`41` | What judging is, probability, inference, decision, cause, institution, and the triad at its minimum |
| IV — The Framework | `42`–`50` | The running case through the sequential loop, the three filters, and the triadic synthesis |
| V — The Multilayer Architecture | `51`–`85` | Thirty-five orders of knowledge in thirty-five chapters, one per order, seven strata |

Part V carries thirty-five orders in thirty-five chapters, one order to a chapter, so the order-to-chapter mapping is now monotone: order `Lnn` is argued in chapter `50+nn`. The stratum distribution is `S1` 51–55, `S2` 56–60, `S3` 61–65, `S4` 66–70, `S5` 71–75, `S6` 76–80, `S7` 81–85. Every Part V chapter opens with a blockquote of the form

```
> Layer NN … Chapter NN · Unique Guard Combination {…} · Signature (E, V, G)
```

and that blockquote is the authority for the order-to-chapter mapping. **Table B.1 of Appendix B must agree with it line by line for all thirty-five rows** — and the mapping is monotone, so a table re-derived from numbering alone reproduces it — which is why the table is checked against the blockquotes and not against the numbering. A chapter number is editorial and carries no machine meaning; an order is a node of the graph and a chapter is where it is argued.

---

## III. Chapter Contracts

Every chapter is markdown only, opens on its `# ` title, and closes with exactly one `## Bridge` that hands the argument to the next chapter by name. No file may begin with an HTML marker: upload content sniffers classify a file beginning with `<!--` as `text/html` and refuse it as markdown. Figures are referenced from `book-src/figures/` and every referenced figure must exist and be referenced by a chapter.

- **Chapters `01`–`85`** (all but the preface) carry `## By the end of this chapter you can` with exactly **three** numbered outcomes, and carry no `## Exercises` section: the exercises are retired pending an interactive companion, and a chapter that grows one meanwhile has broken this clause. Numbering is dense and the outcomes are the first numbered list under their heading; a second numbered list in the same section pushes a naive counter to the wrong verdict, so counters must stop at the first blank line after the block rather than scan the whole section.
- **Chapters `42`–`50`** are the framework chapters. They carry the same spine as any other chapter and additionally carry the worked case; they carry no CLI banner, no printed block and no reference to a pipeline output.
- **Chapters `51`–`85`** carry the layer blockquote and must agree with Appendix B's Table B.1.
- **`00`** is the preface: it is exempt from the outcomes and bridge floor and must not be given one — a preface that grows a syllabus is a preface pretending to be a chapter.
- **Appendices** carry no spine. Appendix B's table and Appendix D's ledger table are the only tables whose cells are contract-bearing; Appendix E's lexicon and its architecture map are a reading aid and carry no verdict, because a table that could return a verdict would be the checklist the fourth reflexive brake prohibits.

**A section may be retitled only when what it named no longer exists** (for example a section called "Laboratory N" after the laboratory apparatus was removed). Chapter titles in `SUMMARY.md` are not renaming targets: the table of contents and the chapter's own `# ` line must agree, and an appendix retitled in its file must be retitled in `SUMMARY.md` in the same change.

---

## IV. Claim Provenance Discipline

Every quantitative claim in `book-src/`, `README.md`, `AGENTS.md`, or any derived text carries one of exactly **two** warrants, stated in the sentence that carries it:

1. **Literature.** An external empirical or conceptual finding quoted with its source, and — where the source reports a value — with the value exactly as the source states it, at the source's own precision, with the citation in the same sentence. Never extend, recompute, round or re-scale a literature value. Examples of the form: Wong et al. (2021) reporting the Epic Sepsis Model's false-alarm rate; Zheng & Meister (2025) reporting the human behavioral and sensory-motor throughput limit; Dehejia & Wahba (1999) reporting the LaLonde benchmark both ways; CAST Investigators (1989) reporting the encainide/flecainide arms.
2. **Derivation.** A result the reader can reproduce from algebra displayed beside it: a definition, an identity, a limit, a combinatorial count, or a symbolic bound. The formula may be specialized, but a *numeric* instantiation of a parameter is not a derivation and does not belong in prose.

The manuscript carries **no bracketed provenance label** and no third class. A number whose warrant the sentence leaves unstated is an invented number, whichever chapter it sits in. A claim that cannot survive without its number is deleted, not reworded.

**Prohibited apparatus.** The manuscript cites sources, not machinery. No chapter, appendix, or root document may name a source file or module path, a crate or package directory, a build or test invocation, a binary or command-line flag, a data file or archive, a helper script, an internal claim identifier, a random seed, or a fenced block reproducing program output. A measurement nobody can re-run from the page is not evidence; it is an assertion with a decimal point. The corresponding rule for an editor is: **delete the token, keep the argument, and never invent a replacement number to keep the sentence alive.** This clause is written without spelling out the forbidden forms, so that the apparatus scan below can be run over this file too — a contract that has to exempt itself from its own gate is not a gate.

**Prohibited citations.** No source may be attributed a claim it does not support, and no citation may be dropped because the sentence around it lost its number. Citations are the asset; when a paragraph is rewritten, its sources travel with it. Where an argument needed more than the sources will support, the text states the limit of what it knows instead of closing the gap with a number nobody published.

---

## V. Verification

The manuscript has no build step: nothing in `book-src/` is compiled, and every gate below is a reading of the text. The core beside it is built and run, but it returns no verdict on the manuscript and no gate below consults it — a defect the core reveals is repaired in the core, and a defect the text reveals is repaired in the text. What can be checked here, and must be checked before a change is proposed, is structural:

1. **Apparatus scan.** No occurrence, anywhere in `book-src/`, `README.md`, `CONTRIBUTING.md` or this file, of: a Rust source extension or a symbol anchor; a crate directory prefix; a build-tool or package-manager invocation; a chapter-flag or output banner; a data-file extension or archive name; a helper-script directory; a backticked claim identifier of the chapter-dot-ordinal form; a seeded simulator symbol; or a fenced block reproducing program output.
2. **Spine.** For every `book-src/NN-*.md` with `NN ≥ 01`: exactly one `## Bridge`, exactly three numbered outcomes, and no `## Exercises` section, which is retired pending the interactive companion. `00` and the appendices are exempt.
3. **Links.** Every relative markdown link resolves after percent-decoding, in `book-src/` and the root documents.
4. **Figures.** Every `book-src/figures/` file is referenced by at least one chapter, and every referenced figure exists.
5. **Table B.1 parity.** All thirty-five rows of Appendix B's table agree with the `> Layer NN … Chapter NN` blockquote and the order list of the chapter they point at, in stratum, guard triple and signature. The token `Layer NN` of the blockquote and the `Chapter NN` token must both name the row of the order the blockquote declares: the row's Order column carries that `Layer NN`, its chapter column carries that `Chapter NN`, and the stratum, guard triple and signature printed in the blockquote are the ones that row carries. The check asks nothing about coverage: a blockquote prints one order, and an order without a chapter of its own has none to print, so an unnamed order is not a failure here. Naming an order the chapter does not carry is a failure; saying nothing about the others is not. The same parity extends to the table of contents: each Part V chapter is listed under the heading of the stratum its blockquote prints, so a chapter filed under another stratum is a failure of this check, and no other ordering of the contents is read.
6. **Ledger parity.** Every claim row in Appendix D names a source whose entry exists in the Appendix C bibliography, and every numeric cell is a value that source reports.
7. **Numeral census.** For each file, list every line after the `# ` title that carries a numeral without a citation year, without a math span, and outside editorial numbering — then justify each one against §IV. The census is a reading aid, not a gate: it flags candidates, and a human decides.
8. **Published surface.** A page that belongs to exactly one chapter and carries its masthead must print that chapter's identity consistently. Four printed objects carry it: the masthead's order label, its chapter token, its guard triple, and the `Layer NN` line of the blockquote. The chapter number is not printed in the title and is not one of them; a chapter's own heading carries its name, and the number it carries in the table of contents and in the masthead is editorial, as §II says. They must agree with one another and with the chapter whose text the page carries, and the guard triple must be the one that page itself prints as its order's unique guard combination; the masthead prints its label twice, as an accessible name and as a visible node, and those two must agree with each other. Such a page fails in three cases: it carries a `Layer NN` blockquote but no masthead; its masthead names a chapter other than the one whose text it carries; or its masthead names an order other than the one its own blockquote declares. The predicate is internal to the page — it compares the printed objects with one another and with the chapter the page carries, not the page with the source that produced it. That the source is what gets corrected follows from reconstruction and not from this comparison: the page is rebuilt from the source, so a divergence found here is repaired in the source, and the source is what a later edition must carry. Nothing here compares the thesis with the world, and the check returns no verdict on any substantive claim — it does not read prose or argument, it does not audit a literature value (that is check six), it does not audit warrant or citation (that is §IV, with the census), it does not extend to a page that carries several mastheads and no single chapter, and it does not replace the seven checks above.

9. **Concordance.** Four propositions are stated in more than one place, and a manuscript that states one of them two ways has contradicted itself before any reader reaches the second site. For each row below, the canonical string must occur somewhere in `book-src/`, and no file of `book-src/` may contain any of that row's contradicting variants. Comparison is literal after collapsing runs of whitespace, so the check is mechanical and reproducible; it does not read prose, and a proposition absent from the table is not thereby licensed to vary — the table's completeness is a declared limit, not a hidden one. The canonical strings and the variants are part of this contract: changing one is a change to this file, and takes the same human confirmation as a retitling. Every wave that repairs a passage touching a proposition in this table re-runs the check before the change closes, because a repair to one site is how a contradiction has entered this manuscript.

| Proposition | Canonical string | Contradicting variants |
| :--- | :--- | :--- |
| The triadic verdict is a minimum, never an aggregate that trades one axis against another | `The minimum is not a weighted sum` | `weighted average of the three`, `weighted sum of the three scores`, `average of the three axes` |
| Design effects compose additively, the product being only an upper bound | `the exact composition is additive` | `composed multiplicatively`, `product of the design effects` |
| The governance coordinate fixes the sign of the differential and is not a third reading | `not one reading among three` | `governance coordinate is one of the three`, `enters the minimum as a third reading` |
| The stratum anchors are recorded designations, not consequences of the table | `designation recorded rather than derived` | `is derived from the table of orders`, `follows from the triples`, `strict 1-to-1 bijection` |

10. **Guard-site cardinality.** The lexicon carries seven guard sites, and the number is not free: it is the number of transits of the mediation circuit — six ordered crossings of distinct planes plus the integral closure, $3 \cdot 2 + 1 = 7$, so that $n = P(P-1)+1$ at three planes. What is checked is the count in the artifact, not the prose that motivates it:

    (a) Appendix E's Table E.1 enumerates `T1`–`T7` once each and names no eighth site;
    (b) every order triple in Table B.1 is a three-element subset of those seven, the table carries $\binom{7}{3} = 35$ distinct ones, and each is carried by exactly one order;
    (c) each stratum's five orders carry five pairwise distinct primary sites, and each of the seven sites is primary in exactly five strata;
    (d) each chapter's `> Layer NN` blockquote prints the triple its row carries;
    (e) no file of `book-src/` contains `the two independent counts agree` or `cheapest available test`, and none derives a count of guard sites from a source other than the circuit's transits.

    A failure in (a)–(d) is a defect of the artifact and is repaired there. (e) cannot be repaired by counting at all: it guards the footing, and the footing is that $b(n) = \binom{n}{3}/n = (n-1)(n-2)/6$ is strictly increasing, so $\binom{n}{3} = 5n$ has one root because $b$ is injective and not because two counts meet. The declared limits are two. The count is forced only once "ordered crossings of distinct planes" and "the closure is itself a guarded transit" are granted; those premises are stated where the circuit is stated, and an eighth site would have to be a guarded object that is neither, which nothing in the corpus supplies. And the check reads the artifact: it returns no verdict on whether the circuit's inventory of transits is complete in the world.

**Human review protection.** Any retitling of an appendix or of a chapter in `SUMMARY.md` requires explicit human confirmation with the inline diff in the request.
