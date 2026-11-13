# Contributing

Contributions must follow the governance and verification contract in [`AGENTS.md`](AGENTS.md) and the manuscript in [`book-src/`](book-src/) (86 chapters and five appendices). The vocabulary the chapters use is defined in [Appendix E](book-src/appendix-e-lexicon.md); the syntax layer and the 35-order mapping are stated in [Appendix B](book-src/appendix-b-architecture-verification.md).

Nothing in `book-src/` is compiled, and what a change to the text has to survive is a contract about the text: every claim carries its warrant in its own sentence, every chapter keeps the same spine, and the tables that other tables depend on stay in agreement. Beside the text the repository also carries a normative core, transcribed from the same productions and checked by the same integration; it has a narrower standing than the text, and the paragraph that states that standing is the one on the core's place in [`AGENTS.md`](AGENTS.md): it is not evidence for any value a chapter quotes, it is not authority over a chapter's definition, and it is not a citation — where the two disagree, the core is what gets repaired.

## 1. The editorial contract

- **Two warrants, stated in the sentence.** A quantitative claim is either a value a named source reports, quoted at that source's own precision, or a result the reader can reproduce from algebra displayed beside it. There is no third class, and no bracketed provenance label. A number whose warrant the sentence leaves unstated is an invented number, whichever chapter carries it.
- **Sources travel with the paragraph.** When a paragraph is rewritten, its citations move with it. No citation is dropped because the sentence around it lost its number, and no source is credited with a claim it does not support. Where the argument needs more than the sources will carry, the text states the limit of what it knows instead of closing the gap.
- **The manuscript cites sources, not machinery.** No chapter, appendix or root document names a file or module path, a package directory, a build or test invocation, a binary or flag, a data file or archive, a helper script, an internal claim identifier, a random seed, or a block reproducing program output. The editor's rule is to delete the token, keep the argument, and never invent a replacement number to keep a sentence alive.
- **The spine.** Every chapter from `01` to `85` opens on its `# ` title, carries exactly three numbered outcomes under its outcome heading, three to five numbered exercises, and closes with exactly one bridge that hands the argument to the next chapter by name. Numbering is dense, and the outcomes are the first numbered list under their heading, so a counter stops at the first blank line after the block rather than scanning the section.
- **The Part V blockquote is authoritative.** Every chapter from `51` to `85` opens with the layer blockquote naming its stratum, its guard combination and its signature. Table B.1 of Appendix B must agree with it line by line for all thirty-five rows, and the mapping is monotone, so a table re-derived from chapter numbering alone reproduces it — which is why the table is checked against the blockquotes and not against the numbering.
- **Figures.** Every image lives in `book-src/figures/` and is referenced by at least one chapter, and every referenced figure exists. A figure duplicates no text: it carries the relation, and the prose carries the argument for it.
- **Appendices carry no spine.** Appendix B's table and Appendix D's ledger table are the only tables whose cells are contract-bearing. Appendix E is a reading aid and carries no verdict.
- **Authority runs from chapter to appendix.** Where an appendix and a chapter disagree about a definition, the chapter governs and the appendix is corrected, because the chapter is where the claim, its source and its derivation sit side by side.

## 2. Structural verification

The ten checks of [AGENTS.md](AGENTS.md) section V are what a change has to pass. Each is a reading of the text, and a contributor who cannot run one by hand should not claim it passes.

| Check | What fails it |
| :--- | :--- |
| Apparatus scan | Any occurrence of the forbidden tokens listed in section 1, in `book-src/` or a root document |
| Spine | A chapter with zero or two bridges, with a numbered outcome list that is not exactly three long, or with fewer than three or more than five exercises |
| Links | Any relative target that does not resolve after percent-decoding, including targets whose path carries an encoded space |
| Figures | An image in `book-src/figures/` that no chapter references, or a reference to an image that is not there |
| Table B.1 parity | A row of Appendix B disagreeing with a layer blockquote or with the order list of the chapter it points at, in stratum, guard combination or signature |
| Ledger parity | A row of Appendix D naming a source absent from the Appendix C bibliography, or carrying a numeric cell the source does not report |
| Numeral census | Not a gate: it lists every line after the title that carries a numeral with no citation year, no math span and no editorial purpose, and a human justifies each one |
| Published surface | A Part V chapter page missing its masthead or disagreeing internally across its masthead order label, chapter token, guard triple, and `Layer NN` blockquote |
| Concordance | Missing canonical string or any occurrence of a forbidden contradicting variant across `book-src/` |
| Guard-site cardinality | Missing `T1`–`T7` enumeration in Table E.1, missing $\binom{7}{3}=35$ triples or unequal primary-site balance in Table B.1, triple mismatch in a blockquote, or forbidden footing phrases |

Two consequences follow for how a change is prepared. First, the census and the parity checks flag candidates rather than verdicts, so a passing run is not a proof that the prose is right, only that nothing structural broke. Second, a change that adds a source has to add it in three places at once — the sentence that carries the claim, the bibliography entry with the year and volume the source itself reports, and the ledger row if the claim carries a number.

## 3. Reading and rendering the book

The manuscript is read as markdown. The table of contents in [`SUMMARY.md`](book-src/SUMMARY.md) names every chapter and appendix, and it must agree with each file's own title line, so a retitled appendix is retitled in the table of contents in the same change.

A rendered copy can be produced by the renderer configured in `book.toml`, which reads `book-src/`, the theme in `theme/` and the math configuration at the repository root. The rendered tree is not part of the contribution: it is ignored, it is regenerated from the sources, and no claim in the manuscript may depend on it.

## 4. Human-gated paths

Any retitling of an appendix or of a chapter in `SUMMARY.md` requires explicit human confirmation with the inline diff in the request. Everything else in `book-src/` may be edited under the contract above.

## 5. Licensing

Documentation contributions are licensed under CC BY 4.0 ([`LICENSE-TEXT`](LICENSE-TEXT)) and the theme assets under MIT ([`LICENSE`](LICENSE)). The vendored typesetting payload under `theme/mathjax/` keeps its upstream Apache-2.0 license ([`THIRD-PARTY-NOTICES.md`](THIRD-PARTY-NOTICES.md)); a contribution must not relicense it.
