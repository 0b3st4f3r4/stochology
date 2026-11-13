# Mechanical gates: what was built, what was refused, and how each gate was proved

Scope: `.github/checks/structure.py`, one invocation, no network. Baseline before this
wave: `python3 .github/checks/structure.py` → `all structural gates pass`, exit 0, 0.9 s,
ten gates (apparatus, spine, links, figures, table B.1, ledger, citations, contents,
concordance, guard-site cardinality).

After this wave: exit 0, `all structural gates pass`, **1.96 s**, seventeen gates and
five advisory reports. Every gate below was planted with its defect in a throwaway copy
(`/tmp/mut`, rsync of the repository, never the repository itself), the gate was run
there, the file was restored from the pristine tree, and the run was required to fail
(or, for a report, to move its count). The harness is `/tmp/mut-run.py`; the table at the
end is its output, verbatim: **24 mutations, 0 not detected**.

The rule applied throughout: a gate ships only when it has a defect it catches today and
a false-positive rate that a reader would accept. A candidate that fails either test
becomes a report, or is refused with the reason.

---

## 1. The nine candidate gates

### 1. Orphan Appendix C entries — **report, not gate**

`report_orphan_entries()`. An entry nothing cites is not a defect: §IV says a citation
may never be dropped because the sentence lost its number, but it says nothing against a
bibliography wider than its text. The mechanical difficulty is worse than the policy one:
institutional
authors (`U.S. Securities and Exchange Commission`, `Massachusetts Supreme Judicial
Court`) have no surname to match, and a fold-based surname test produces false negatives
(`Little` is swallowed by the ordinary word *little*, `Brin` by *bring*). Ten entries are
reported today: Advisory Committee…, Bowman, Brin, Citton, LeMoigne, Little, Mandelbrot,
Massachusetts Supreme Judicial Court, National Commission on the BP Deepwater Horizon Oil
Spill, U.S. Securities. A gate here would either fail on a legitimate citation policy or
be tuned until it caught nothing.

Mutation: an uncited entry appended to Appendix C moves the report 10 → 11.

### 2. Year drift (one entry, another year in the text) — **refused, redundant**

Probed before writing anything: 414 surnames, 340 with a single year, 18 candidate
drifts, **all eighteen** artefacts of co-authorship — `Thompson 1952` is Horvitz &
Thompson 1952, `Price 2013` is Lum, Price & Banks 2013, and the same holds for Brown
2007, Pearl 2000, Gneiting 2021, Lai 2002, Scott 1940, Ramdas 2022, Anderson/Ferguson
2004, Robbins 1985, Hardt 2020. The existing `check_citations()` already fails a year that
no entry carries (`has_entry(surname, year)`), which is the only form of drift that is a
defect. A second gate would duplicate it and add eighteen false positives.

### 3. Attribution ambiguity — **gate** (`attribution`)

`check_attribution()`. A `(surname, year)` pair carrying two Appendix C entries cannot be
disambiguated by surname and year, so every in-text call to that pair must name a
co-author that separates the entries, or carry an `a`/`b` suffix. Exactly one pair is
ambiguous today — `Hardt 2016` (Hardt, Megiddo, Papadimitriou & Wootters; Hardt, Price &
Srebro) — and all five calls already name the separating co-author, so the gate is green
and stays green only while the next call does the same.

Mutation: `Hardt, Megiddo, Papadimitriou and Wootters (2016)` reduced to `Hardt (2016)` in
`book-src/77-layer-game-theory-strategic-adversarial-order.md` fails the gate.

### 4. Notation consistency — **gate** (`notation`) + **report**

`check_notation()` covers the families where a variant changes the identity of the symbol:
`S_{\mathrm{epi}}`, `S_{\mathrm{vital}}`, `S_{\mathrm{gov}}`, `\rho_{R,Y}`,
`M_{\mathrm{Leontief}}` must be written with `\mathrm` and with the subscripts in that
order, and `DEFF`, `TTS`, `TTR` must sit inside `\mathrm` wherever they occur in math. A
variant that renders identically — `\rho_{R, Y}` with a space, an unescaped `_` in a
subscript — is a **report** (`report_notation_drift()`, 4 today: one in
`book-src/22-thermodynamics-and-entropy.md:262`, three in
`book-src/45-world-to-archive-big-data-paradox.md:162,163,165`), because a gate on
whitespace inside math would be a formatting opinion, not a reading of the text.

Mutations: `\text{epi}` for `\mathrm{epi}`, `\rho_{Y,R}` for `\rho_{R,Y}`, `\text{DEFF}`
inside math — each fails; `\rho_{R, Y}` moves the report.

### 5. Math integrity — **gate** (`math`)

`check_math()` reads three things: `$$` parity per file; `$` parity **per paragraph**,
because an inline span may wrap a line (`book-src/03-the-biggest-number-in-the-room.md:255-256`,
`book-src/55-layer-continuum-fracture-structural-order.md:49-50`) and a per-line count
would call them unclosed, and a per-line parity count reports every one of them; and a `_` or
`^` outside math, after removing display blocks, inline spans and code spans. `\$` is
neutralised before counting, since an escaped dollar is prose: fourteen occurrences across
`book-src/appendix-d-claim-ledger.md`, `book-src/59-layer-experimental-sampling-causal-order.md:38`,
`book-src/79-layer-official-statistics-forensic-accounting-order.md:54` and
`book-src/82-layer-evidentiary-tort-constitutional-legal-order.md:74-80`.

Mutations: a `$$` opened and never closed fails; a closing `$` deleted from an inline span
fails; `unescaped_underscore` appended to a chapter fails.

### 6. Display formula whose symbols the chapter never defines — **report**

`report_undefined_symbols()` now also collects `\mathrm{…}` names of three letters or
more, and reports a Greek letter or such a name that appears in a display block and in no
prose line of the chapter (25 today). It is not a gate: an abbreviation may be expanded in
words (`FDR` as *false discovery rate*) or defined in an earlier block, and a predicate
narrow enough to survive that would be a parser of English.

### 7. Near-duplicate sentences across chapters — **report**

`report_near_duplicates()` compares sentences of at least 120 characters, normalised, on a
trigram index, at ratio ≥ 0.85, excluding the Part V template sentences (they are the
architecture, not a repeated claim). The first implementation compared every pair and took
over 60 s; the index brought it to 0.31 s and the report to **0** — the corpus does not
repeat itself at that threshold. Kept because a proposition stated twice is exactly how a
contradiction enters, and the cost is now negligible.

Mutation: one sentence planted in two chapters moves the report 0 → 1.

### 8. Cross-reference naming no shared content word — **report**

`report_cross_references()`, 15 hits today, almost all of them range citations
(`Chapters 43–45`) and part-level references where no single chapter title can share a
term. The task says to report and not to fail; the hits agree.

### 9. Falsifier present for each collapse proposition — **gate, on a form the manuscript has**

The candidate's literal form — a refutation sentence in *the same chapter* as the
proposition — does not exist in this corpus and could not be enforced without rewriting
the book: the propositions are stated in `book-src/41-the-triad-and-the-minimum.md`
(199, 205, 216) and the falsifiers are collected in §9.5.3 of
`book-src/50-methodological-families-reflexivity.md` (355). What does exist is a
structural object, and it is now gated twice.

`check_falsifiers()` reads §9.5.3 as a table in prose: four items numbered 1–4, each
naming `$F\_n$`, each carrying a sentence with `fails if`, each naming **exactly one** of
`S_epi`, `S_vital`, `S_gov`, `M_Leontief`, and the four naming all four exactly once. That
is the parity §I claims — one falsifier per component of the triad plus the aggregate.

Mutations: `F_2` losing its failure condition fails; `F_3` retargeted at `S_epi` fails
(the epistemic margin is then tested twice and governance not at all); falsifier 3
renumbered to 4 fails; the section retitled so it can no longer be located fails.

---

## 2. The five gates proposed by the captain, and the algebra audit

### G-A. Concordance is substring-only, so a negation passes — **gate** (`concordance-polarity`)

§V-9 requires the canonical string and forbids the listed variants, but a sentence that
*negates* the canonical proposition contains it verbatim: "it does not enter the minimum
as one reading among three" passes `check_concordance()` while asserting the opposite.
`check_concordance_polarity()` derives the opposite of each canonical string mechanically
(`X is not Y` → `X is Y`; a leading `not` dropped; `A rather than B` → `B rather than A`;
otherwise `is` → `is not`), and fails any occurrence of the opposite that is not negated
within its own clause. The clause window matters: the live sentence in
`book-src/85-…:54` denies the reading twice — first as "not one reading among three", then
as "does not enter the minimum as one reading among three" — and a fixed 24-character
window reads the second as an assertion. Reading back to the last clause boundary
(`. ; : — –` or `, `) reads it correctly. All four rows yield a derived opposite; three
hits in the corpus are negations within their clause and stay green.

Mutations: "The governance coordinate is one reading among three." appended to a chapter
fails; "The minimum is a weighted sum at the close of the trajectory." fails.

### G-B. Nothing checks the three collapse propositions — **gate** (`collapse-refutation`)

`check_collapse_refutation()`. The chapters it reads are not hardcoded: it parses the
contract itself — `contract_chapters("collapse propositions")` finds the §II sentence that
carries the topic *and* backticked chapter numbers, returning `{29, 41, 50}` — and then
requires, for every paragraph that opens with `**Collapse Proposition N`, a sentence in
the same paragraph licensing refutation (`fails if`, `would be false`, `refute*`,
`falsif*`). A chapter the contract names without stating a proposition is not a failure;
stating one without saying what would refute it is.

The three propositions in chapter 41 carry no such sentence — that is a finding, not a
gate failure, so the three sites are **declared open** (§3) and printed. The gate is
therefore green today, and red the moment a chapter the contract names states a
proposition beyond those four sites and leaves out the clause.

Mutations: a duplicated, unrefuted Proposition 1 in chapter 41 fails (the declaration
covers one site, not one key); a Proposition 1 planted in chapter 29 fails.

### G-C. Appendix E derives seven guard sites from three transits — **gate** (`guard-footing`)

§V-10 fixes the count as six ordered crossings plus the closure, `n = P(P−1)+1` at three
planes; `book-src/appendix-e-lexicon.md:66` says the seven tensions are "arranged over
the three transits of the mediation circuit", which is a different arithmetic and the
footing §V-10(e) forbids. `check_guard_footing()` fails the phrase
`three transits of the … circuit` wherever it appears. The declared site is that one
occurrence.

The broader predicate the captain asked for — every count statement about the seven must
carry the footing tokens — was **prototyped and refused**: on the corpus it fires on
`3 + 2 + 2 = 7` triad-axis partitions, on `\binom{8}{3} = 56 against 8 × 5 = 40`,
on exercise answers, and on `22 = 7` in chapter 17. A gate that fires on legitimate
decompositions teaches its reader to ignore it. What survives is the exact phrase, which
has one occurrence and one meaning.

Mutations: the phrase planted in `book-src/09-…md` fails; planted in `appendix-a` fails
(the declaration covers the Appendix E site only); repairing the Appendix E sentence in
the copy leaves the gate green **and** prints the entry as stale.

### G-D. No symbol may be introduced for the first time inside a display block — **report**

Implemented as the extension of `report_undefined_symbols()` described in §1.6: 25 hits.
The captain's literal predicate (identifiers absent from *earlier* prose in the same
section) was measured at 109 blocks and is dominated by subscript abbreviations (`eff`,
`req`, `crit`, `post`) that are conventional rather than defined. Report, not gate.

Mutations: `$$\zeta = \frac{1}{2}$$` appended moves the count 25 → 26; so does
`$$\mathrm{ZZQ} = \frac{1}{2}$$`.

### G-E. Two factors with related names in one display need a sentence separating them — **proposal only, nothing gated**

The live instance is `book-src/47-irreversible-time-vital-margin.md:164`, where
`k(n_{\mathrm{eff}})` and `k_{\mathrm{BM}}` sit in the same display and the text justifies
the first from `\sigma^2_{\hat\mu} = \sigma^2/n_{\mathrm{eff}}` while the relation to
`k_{\mathrm{BM}}` stays implicit. The predicate "two factors whose names share a stem"
does not survive contact with the corpus: `k`, `n`, `T`, `S` and `M` are reused across
chapters for different objects, and the shared stem is as often `k`/`k` for two genuinely
different constants as it is `k`/`k_{\mathrm{BM}}` for two related ones. Deciding whether
two symbols name the same object is a reading of the argument, which is what a gate is
supposed to replace and cannot. Proposed instead as an editorial pass over chapter 47,
not a check.

---

## 3. Declared open defects: a gate that ships ahead of its repair

Two of the new gates catch defects the manuscript still carries, at four sites. Shipping
them red
would leave the tree unusable for everyone working beside it; omitting them would leave
the defect unread. `DECLARED_OPEN` names each unrepaired site by the key its gate would
have used; the gate prints it and fails only on what is not declared.

The list ratchets down in both directions. A declaration covers **one** site: a second
occurrence of the same key is a new defect and fails. And an entry whose defect is gone
is printed as stale — `declared defects no longer present (remove the entry): N` — so a
declaration cannot outlive the reason it was written.

Four entries today:

| gate | site | defect |
| :--- | :--- | :--- |
| `collapse-refutation` | `book-src/41-the-triad-and-the-minimum.md` Collapse Proposition 1 | stated, with no sentence saying what would refute it |
| `collapse-refutation` | same, Proposition 2 | same |
| `collapse-refutation` | same, Proposition 3 | same |
| `guard-footing` | `book-src/appendix-e-lexicon.md` "three transits of the mediation circuit" | derives seven sites from three transits; §V-10 fixes six crossings plus the closure |

Mutations: repairing the Appendix E sentence in the copy leaves exit 0 and reports the
entry as stale; the same footing defect planted in a second file fails.

---

## 4. No gate reaches the network

`structure.py` imports `glob`, `html`, `itertools`, `os`, `re`, `sys`, `unicodedata`,
`urllib.parse` and `collections`. `urllib.parse` is used only for
`unquote()` on link targets (§II requires percent-decoding); nothing opens a socket, reads
a URL or resolves a host. The new gates read `book-src/`, `README.md` and `AGENTS.md`, all
local. Runtime 1.96 s on this machine, against the ~5 s budget.

---

## 5. Mutation run

`python3 /tmp/mut-run.py` — copy at `/tmp/mut`, one defect at a time, restored from the
pristine tree after each.

### 5.1 The repository after the change

```
$ time python3 .github/checks/structure.py
  declared open [collapse-refutation] book-src/41-the-triad-and-the-minimum.md: Collapse Proposition 1
  declared open [collapse-refutation] book-src/41-the-triad-and-the-minimum.md: Collapse Proposition 2
  declared open [collapse-refutation] book-src/41-the-triad-and-the-minimum.md: Collapse Proposition 3
  declared open [guard-footing] book-src/appendix-e-lexicon.md: three transits of the mediation circuit
numeral census (advisory, not a gate):
  ...
orphan Appendix C entries (advisory: a bibliography may be wider than its text): 10
symbols first set inside a display block (advisory: an abbreviation may be expanded in words): 25
near-duplicate sentences across chapters (advisory: template sentences excluded): 0
cross-references sharing no content word with their sentence (advisory): 15
notation drift that renders identically (advisory): 4

all structural gates pass

real    0m1.957s
```

### 5.2 Mutation table

| mutation | gate | file | expected | exit | evidence |
| :--- | :--- | :--- | :--- | :--- | :--- |
| unbalanced '$$' (a display block left open) | math | `book-src/01-deciding-without-knowing.md` | fail | 1 | [math] book-src/01-deciding-without-knowing.md: 7 '$$' delimiters -> a display block never closes |
| closing '$' deleted from an inline span | math | `book-src/01-deciding-without-knowing.md` | fail | 1 | [math] book-src/01-deciding-without-knowing.md:132: paragraph carries an odd number of '$' -> inline math unclosed |
| unescaped '*' in prose | math | `book-src/01-deciding-without-knowing.md` | fail | 1 | [math] book-src/01-deciding-without-knowing.md:325: unescaped '*' outside math -> An unescaped_underscore reaches markdo |
| S_epi set in \text instead of \mathrm | notation | `book-src/06-who-is-missing-from-the-table.md` | fail | 1 | [notation] book-src/06-who-is-missing-from-the-table.md:151: the epistemic margin coordinate written as 'S\_{\text{epi' |
| rho arguments swapped to \rho_{Y,R} | notation | `book-src/06-who-is-missing-from-the-table.md` | fail | 1 | [notation] book-src/06-who-is-missing-from-the-table.md:129: the archive correlation written as '\rho\_{Y,R}' |
| DEFF set with \text inside math | notation | `book-src/06-who-is-missing-from-the-table.md` | fail | 1 | [notation] book-src/06-who-is-missing-from-the-table.md:184: DEFF set without \mathrm inside math |
| ambiguous call: co-author dropped from 'Hardt, ... (2016)' | attribution | `book-src/77-layer-game-theory-strategic-adversarial-order.md` | fail | 1 | [attribution] book-src/77-layer-game-theory-strategic-adversarial-order.md:26: Hardt (2016) is carried by 2 Appendix C e |
| F_2 loses its failure condition | falsifiers | `book-src/50-methodological-families-reflexivity.md` | fail | 1 | [falsifiers] book-src/50-methodological-families-reflexivity.md:360: F_2 states no condition under which it fails |
| F_3 retargeted at S_epi, leaving S_gov untested | falsifiers | `book-src/50-methodological-families-reflexivity.md` | fail | 1 | [falsifiers] book-src/50-methodological-families-reflexivity.md:361: the epistemic margin is tested by more than one fal |
| falsifier 3 renumbered to 4 | falsifiers | `book-src/50-methodological-families-reflexivity.md` | fail | 1 | [falsifiers] book-src/50-methodological-families-reflexivity.md:361: falsifier 3 numbered 4 |
| section retitled, falsifiers no longer located | falsifiers | `book-src/50-methodological-families-reflexivity.md` | fail | 1 | [falsifiers] no chapter carries the 'The Four Canonical Falsifiers' section |
| governance asserted as one reading among three | concordance-polarity | `book-src/06-who-is-missing-from-the-table.md` | fail | 1 | [concordance-polarity] book-src/06-who-is-missing-from-the-table.md: asserts the opposite of the governance coordinate f |
| the minimum asserted as a weighted sum | concordance-polarity | `book-src/01-deciding-without-knowing.md` | fail | 1 | [concordance-polarity] book-src/01-deciding-without-knowing.md: asserts the opposite of the triadic verdict is a minimum |
| a second unrefuted statement of Proposition 1 | collapse-refutation | `book-src/41-the-triad-and-the-minimum.md` | fail | 1 | [collapse-refutation] book-src/41-the-triad-and-the-minimum.md:205: Collapse Proposition 1 carries no sentence saying wh |
| Proposition 1 moved to a chapter the contract also names | collapse-refutation | `book-src/29-aim-and-the-three-planes.md` | fail | 1 | [collapse-refutation] book-src/29-aim-and-the-three-planes.md:320: Collapse Proposition 1 carries no sentence saying wha |
| circuit credited with three transits outside Appendix E | guard-footing | `book-src/09-sets-relations-functions.md` | fail | 1 | [guard-footing] book-src/09-sets-relations-functions.md: the circuit is credited with 'three transits' -> 'three transit |
| same sentence planted in two chapters | report | `book-src/11 + 12` | report 0->1 | 0 |  |
| uncited entry appended to Appendix C | orphan Appendix C entries | `book-src/appendix-c-sources.md` | report 10->11 | 0 |  |
| \rho_{R, Y} with a space: renders the same, is reported | notation drift that render | `book-src/06-who-is-missing-from-the-table.md` | report 4->5 | 0 |  |
| display symbol absent from the chapter's prose | symbols first set inside a | `book-src/01-deciding-without-knowing.md` | report 25->26 | 0 |  |
| abbreviation first set inside a display block | symbols first set inside a | `book-src/01-deciding-without-knowing.md` | report 25->26 | 0 |  |
| cross-reference naming no shared content word | cross-references sharing n | `book-src/01-deciding-without-knowing.md` | report 15->16 | 0 |  |
| declared defect repaired in the copy | declared-open | `book-src/appendix-e-lexicon.md` | exit 0 + stale entry reported | 0 | declared defects no longer present (remove the entry): 1 |
| declared defect repeated in a second chapter | guard-footing | `book-src/appendix-a-mathematics.md` | fail | 1 | [guard-footing] book-src/appendix-a-mathematics.md: the circuit is credited with 'three transits' -> 'three transits of  |

24 mutations, 0 not detected

---

## 6. t19 — colisão de símbolo (rejeitado) e limite de razão (instalado)

Dois candidatos vieram da auditoria algébrica da Parte V (`audit/audit-partV-algebra.md`
§6, achados A1–A4). Regra da tarefa: medir antes de instalar; predicado que não fecha
sem ruído vira proposta com o número medido.

### 6.1 GATE A — colisão de símbolo: **rejeitado, vira proposta**

O defeito é real e está nos três lugares que a auditoria aponta:
`book-src/76-layer-sequential-decision-bandit-real-options-order.md:28` (α, β como taxas
de erro de Wald) contra `:43` (α, β como deriva e raiz característica);
`book-src/78-layer-coherent-risk-endogenous-leverage-order.md:36,40` (k como fator de
Baker–McHale e como desvios-padrão); `book-src/84-layer-infrastructure-nuclear-hydraulic-safety-order.md:27,33`
(ρ como utilização de armazenamento e como fração de causa comum).

Três predicados foram medidos, todos por glosa em prosa:

| predicado | arquivos | candidatos | recall nos 3 achados | precisão (amostra lida) |
| :--- | ---: | ---: | ---: | ---: |
| P1 — dois usos com glosa (`$x$ is/are/denotes…`), primeira glosa fixa o sentido, segunda sem palavra de conteúdo em comum | 41 | 142 | 1/3 (só 84) | ~0/15 |
| P2 — P1 exigindo verbo copulativo **colado** ao span e ≥2 palavras de conteúdo na glosa | 15 | 20 | 0/3 | 0/20 |
| P3 — definição pré-nominal (`the <substantivo> $x$`), dois substantivos distintos | 30 | 71 | 0/3 | ~1/15 |

O padrão é o mesmo nas três: o candidato é a mesma quantidade descrita duas vezes com
outras palavras ("o índice de cauda α" / "α é o parâmetro de fronteira"; "a perda L
sofrida" / "a exposição privada L"), e o defeito verdadeiro não é visto porque **o
manuscrito não glosa o símbolo onde ele colide** — em 76:28 α aparece dentro de
`A=(1-β)/α` sem uma frase que diga "α é a taxa de erro"; a colisão está na aritmética,
não na prosa. Um predicado que só lê glosa é cego exatamente para o caso que importa.

**Proposta (não instalada).** A colisão de símbolo é um problema de contrato de notação,
não de leitura de prosa: cada capítulo declara seus símbolos uma vez, e a verificação
mecânica passa a ser sobre essa declaração — uma tabela de símbolos por capítulo, ou uma
frase única "Symbols in this chapter" com `α …`, `β …` — e então duas declarações do
mesmo símbolo no mesmo arquivo é falha. Isso exige escrever a declaração em 35 capítulos
da Parte V antes que o portão tenha o que ler; enquanto ela não existir, o portão não
tem sujeito. O que a leitura humana pode fazer hoje é a lista dos três achados acima,
que já está em `audit/audit-partV-algebra.md` §7.

Rejeitado como gate: precision ~0 nas três formas, recall 1/3 no máximo, e o custo de
falso positivo é alto (bloquearia 41 arquivos por nada).

### 6.2 GATE B — limite de razão sem hipótese: **instalado** (`ratio-limit`)

Defeito real: `book-src/76-…:43`, "the hurdle $\beta_1/(\beta_1 - 1)$ … tends to
$r/(r - \alpha)$ as the diffusion vanishes", sem `$r > \alpha$`; em $r \le \alpha$ o
denominador não é positivo e a raiz de que a razão veio não está acima de 1.

Predicado instalado, em `structure.py` §18:

1. o limite é o que o `\to` aponta **dentro** do span, ou o span que segue "tends to"
   em prosa;
2. esse limite é uma razão `\frac{A}{B}`, `A/(B)` ou `A/B`, com numerador de **um**
   símbolo;
3. a seção (bloco entre cabeçalhos) precisa de uma desigualdade entre o numerador e
   cada símbolo do denominador, **ou** de positividade de todos os símbolos do
   denominador no arquivo — a hipótese que impede o denominador de anular-se.

A positividade é lida no arquivo, não na seção: `fix $\lambda > 0$` em `48-…:398`
governa o capítulo, e a desigualdade entre dois símbolos é local. O escape de
positividade é do **denominador** apenas: `$r>0$` e `$\alpha>0$` não excluem
$r=\alpha$, então não compram o limite de 76:43.

População medida no corpus: **2** limites de razão em todo `book-src/` — 76:43 (sem
hipótese, verdadeiro positivo) e 48:398 (com `\lambda > 0`, verdadeiro negativo que o
escape aceita). Um hit, zero falso positivo. O site verdadeiro está declarado em
`DECLARED_OPEN["ratio-limit"]` porque o reparo pertence à onda de álgebra; quando a
frase ganhar `$r > \alpha$`, `report_stale_declarations()` imprime a entrada.

Mutação provada em `/tmp/mut` (nunca na árvore), com restauração do arquivo pristino:

| mutação | exit | evidência |
| :--- | ---: | :--- |
| limite de razão sem hipótese plantado em `book-src/09-sets-relations-functions.md` | 1 | `[ratio-limit] book-src/09-sets-relations-functions.md:355: the limit \alpha/(\beta) names no inequality between \alpha and \beta in its section` |
| o mesmo limite com `$\alpha > \beta>0$` na mesma seção (controle negativo) | 0 | só a linha `declared open [ratio-limit] … r/(\alpha)` |
| chave declarada reusada num segundo site do mesmo arquivo | 1 | `[ratio-limit] book-src/76-…:81: the limit r/(\alpha) names no inequality between r and \alpha in its section` |

Rodada verde: `all structural gates pass`, exit 0, **2.19 s**, com
`declared open [ratio-limit] book-src/76-…: r/(\alpha)`. O passo de CI passa a nomear
`ratio limit`.

### 6.3 Limite declarado do portão novo

O portão lê **seção** para a desigualdade, então uma hipótese declarada três parágrafos
antes, sob outro cabeçalho, conta como ausente — foi o que fez 48:398 aparecer na
primeira medição. Também não lê `\lim` com notação de seta fora do span nem uma razão
que o texto nomeie em palavras ("the ratio of $r$ to $r-\alpha$"). O universo medido é
pequeno (2 sites), então o portão é barato e estreito de propósito: ele não promete
cobrir limite de razão em geral, promete cobrir a forma que a auditoria achou.
