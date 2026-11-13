# Dois portões residuais — ano sem citação e coerência do ledger (t10)

Data: 2026-11-13. Arquivo tocado: `.github/checks/structure.py` (seções 16 e 17) e
`.github/workflows/ci.yml` (nome do passo). Nada em `book-src/` foi editado: os dois
portões reportam, o reparo é de outro papel. Nada commitado, nenhuma rede.

Rodada verde: `python3 .github/checks/structure.py` → `all structural gates pass`,
exit 0, 2.17 s (limite declarado: ~5 s).

---

## 1. Portão 1 — ano sem citação (`year-citation`)

### 1.1 O defeito que ele existe para ver

O censo de numerais (§V-7) aceita `cite` como `\d{4}[a-z]?\)` — qualquer ano seguido de
parêntese conta como citação. Por isso `26:135` ("1878") e `28:229` ("enacted in 1969")
passam pelo censo sem que nada no repositório leia a frase: a data é uma afirmação sobre
o mundo e o parágrafo não nomeia fonte. Os dois sites citados na tarefa foram o alvo; o
portão mede o corpus inteiro.

### 1.2 Predicado instalado

Unidade: **parágrafo** (um leitor toma as fontes do parágrafo pelas do parágrafo).
Excluídos antes da leitura, com os offsets preservados: blocos `$$`, SVG/HTML, cercas,
math inline, links markdown, code spans, títulos (`#`), blockquotes (`>`) e linhas de
tabela. Um ano de quatro dígitos entre 1500 e 2099 é **atribuído** quando qualquer uma
das formas vale no parágrafo:

1. parêntese com um nome e um ano dentro — `(Wong et al. 2021)`, `(Chao 1987; Chao & Jost 2012)`;
2. nome a menos de 90 caracteres antes do ano, sem mês imediatamente antes —
   `Bernoulli's *Ars Conjectandi*, published posthumously in 1713`;
3. nome a menos de 90 caracteres depois do ano — `In 1961, John Kingman proved`;
4. nome *e* ano adjacentes no texto com parênteses removidos, para o caso de parêntese
   aninhado — `(Office of Qualifications and Examinations Regulation (Ofqual) 2020)`.

Não são nomes as palavras funcionais (`The`, `In`, `Chapter`, `Appendix`, `Figure`…),
nem meses (`In August 2020` é uma data, não uma atribuição). Décadas (`the 1990s`) não
são datas de quatro dígitos e ficam fora.

### 1.3 Medição — a escada, e por que o predicado estreitou

O predicado literal da tarefa ("par de citação no mesmo parágrafo") foi medido em
quatro versões; o número caiu à medida que as formas legítimas de atribuição entraram,
e cada queda foi verificada contra a frase, não contra a contagem:

| versão | o que ela conta | sites |
| :--- | :--- | ---: |
| V1 | qualquer parêntese com um ano no parágrafo | 33 |
| V2 | nome obrigatório antes do ano, dentro do parêntese | 122 |
| V3 | V2 + intervalos (`1943/1966`, `1984, 1987`), possessivo, `et al.`, epônimo | 66 |
| V4 | V3 + nome antes/depois de um ano nu, mês não conta como autor, parêntese aninhado | **4** |

V1 subconta (aceita `(1 January 1878)` como citação); V2 superconta por construção —
`Girard's linear logic (1987)` e `Chao's (1984, 1987)` são atribuições e o padrão não as
lia. A versão instalada é V4: 4 sites, todos reais, nenhum deles um par de citação que
o predicado não soube ler. Com 4 sites o portão **não** vira relatório (o corte da
tarefa era ~40); ele entra como gate, e os 4 ficam declarados como dívida.

### 1.4 Os quatro sites

| file:line | ano | frase | o que falta |
| :--- | :--- | :--- | :--- |
| `book-src/14-linear-algebra.md:289` | 1986 | "the geometric content of the 1986 recovery narrated in [Chapter 28] §5" | a data nomeia um evento que o capítulo 28 narra; ou o ano sai ("the recovery narrated in §5"), ou o evento ganha fonte no 28 |
| `book-src/28-the-machine-and-the-deception.md:227` | 1969 | "The consequence reaches far beyond 1969: **locality of the evidence is not neutral…**" | o parágrafo anterior estabelece o resultado de 1969; este parágrafo o estende sem citar nada. Falta a fonte do resultado estendido, ou a remoção do ano |
| `book-src/36-decision-loss-threshold.md:325` | 1976 | "The check that would have caught 1976 is a term-by-term one…" | o ano remete a um caso do próprio capítulo; ou o antecedente é nomeado (e citado), ou o ano sai |
| `book-src/37-model-fit-and-error.md:195` | 1987 | "(e.g., portfolio insurance models amplifying the 1987 market crash)" | data de evento público sem fonte; ou ganha citação, ou o exemplo perde o ano |

Os quatro estão em `DECLARED_OPEN["year-citation"]`, com a chave `(caminho, ano)`. A
declaração cobre **um** site: uma segunda ocorrência da mesma chave falha. Quando o ano
receber citação, `report_stale_declarations()` imprime a entrada para remoção — a lista
não sobrevive ao defeito.

### 1.5 Limite declarado

O portão lê **parágrafo**, não afirmação. Um ano dentro de um parágrafo que cita
qualquer outra coisa passa, mesmo que a data não seja do trabalho citado — é o preço de
não fazer análise de frase, e é o mesmo preço que o censo já paga. O portão também não
distingue um ano que nomeia um evento de um ano que reporta um valor: os quatro sites
declarados são todos do primeiro tipo, e a decisão sobre eles é editorial (§IV pede
fonte para a data; o leitor sobrevive sem ela em dois dos quatro).

---

## 2. Portão 2 — coerência do ledger (`ledger-coherence`)

`audit/repair-report.md` é a única prova de que cada edição tem achado de origem. O
portão lê o ledger e verifica:

- **(a) caminho** — todo token `book-src/NN-nome.md` ou `book-src/appendix-nome.md`
  citado existe na árvore. Um caminho que não existe torna o registro inverificável.
  O token admite sufixo de linha (`…md:165`), que é como o ledger localiza o trecho.
- **(c) portão** — todo token entre crases na coluna de portão é: `leitura`, um nome de
  portão que este arquivo imprime (lido das próprias chamadas a `fail("<gate>", …)`), ou
  uma função `check_*` definida em `structure.py`. Um `check_inexistente` na coluna é o
  defeito que t8 achou, e é o que (c) pega. Células de prosa (comandos, frases, nomes de
  arquivo) não são tokens de portão e ficam fora.
- **(b) completude — limite declarado.** "Todo caminho alterado aparece no ledger" não é
  mecanizável a partir do ledger, e o portão não roda git (regra da tarefa: nada de git,
  nada de rede). A lista de referência seria a árvore, e a árvore não guarda o que
  mudou. O que o portão pode afirmar é o sentido inverso — o que o ledger nomeia existe —
  e é o que ele afirma. A completude é asserida pela onda que escreveu o ledger
  (`audit/repair-report.md` §6 lista os 16 arquivos de `book-src/` tocados pelo reparo
  t5) e **não** é verificada aqui.

### 2.1 Teste de mutação do portão 2 (executado em `/tmp/mut`, não na árvore)

| mutação | esperado | resultado |
| :--- | :--- | :--- |
| `check_inexistente` na coluna de portão | exit 1 + `[ledger-coherence]` | detectado |
| `book-src/99-ghost-chapter.md` no ledger | exit 1 + `[ledger-coherence]` | detectado |
| `citationn` (nome de portão que não existe) na coluna | exit 1 + `[ledger-coherence]` | detectado |
| `falsifiers` (portão que existe) na mesma posição | exit 0 | não detectado, correto |

A última linha é o controle negativo: sem ela, o portão poderia estar falhando por
qualquer token.

---

## 3. Prova de mutação dos dois portões

Harness em `/tmp/mut-run.py` (fora do repositório): cópia de `book-src/` e
`.github/checks/` em `/tmp/mut`, cada mutação edita **um** arquivo da cópia, roda o
gate ali, restaura o arquivo do repositório pristino, e imprime a tabela. Nenhuma
escrita em `/home/silvano-neto/Documents/stochology`.

Tabela gerada por `/tmp/mut-run.py` (30 mutações, 0 não detectadas; exit 0). Cada linha edita um arquivo da cópia, roda o gate e restaura. As quatro últimas linhas são os controles do portão 2.

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
| a second unrefuted statement of Proposition 1 | collapse-refutation | `book-src/41-the-triad-and-the-minimum.md` | fail | 1 | [collapse-refutation] book-src/41-the-triad-and-the-minimum.md:390: Collapse Proposition 1 carries no sentence saying wh |
| Proposition 1 moved to a chapter the contract also names | collapse-refutation | `book-src/29-aim-and-the-three-planes.md` | fail | 1 | [collapse-refutation] book-src/29-aim-and-the-three-planes.md:320: Collapse Proposition 1 carries no sentence saying wha |
| circuit credited with three transits outside Appendix E | guard-footing | `book-src/09-sets-relations-functions.md` | fail | 1 | [guard-footing] book-src/09-sets-relations-functions.md: the circuit is credited with 'three transits' -> 'three transit |
| same sentence planted in two chapters | report | `book-src/11 + 12` | report 0->1 | 0 |  |
| uncited entry appended to Appendix C | orphan Appendix C entries | `book-src/appendix-c-sources.md` | report 10->11 | 0 |  |
| \rho_{R, Y} with a space: renders the same, is reported | notation drift that render | `book-src/06-who-is-missing-from-the-table.md` | report 3->4 | 0 |  |
| display symbol absent from the chapter's prose | symbols first set inside a | `book-src/01-deciding-without-knowing.md` | report 25->26 | 0 |  |
| abbreviation first set inside a display block | symbols first set inside a | `book-src/01-deciding-without-knowing.md` | report 25->26 | 0 |  |
| cross-reference naming no shared content word | cross-references sharing n | `book-src/01-deciding-without-knowing.md` | report 15->16 | 0 |  |
| declared defect repeated in another chapter | guard-footing | `book-src/appendix-a-mathematics.md` | fail | 1 | [guard-footing] book-src/appendix-a-mathematics.md: the circuit is credited with 'three transits' -> 'three transits of  |
| a dated claim with no citation pair in its paragraph | year-citation | `book-src/01-deciding-without-knowing.md` | fail | 1 | [year-citation] book-src/01-deciding-without-knowing.md:325: the year 1878 is asserted in a paragraph that carries no ci |
| a declaration covering one site does not cover a second | year-citation | `book-src/03-the-biggest-number-in-the-room.md` | fail | 1 | [year-citation] book-src/03-the-biggest-number-in-the-room.md:298: the year 1986 is asserted in a paragraph that carries |
| declared year acquires a citation in the copy | year-citation | `book-src/14-linear-algebra.md` | exit 0 + stale entry | 0 | declared defects no longer present (remove the entry): 1 |
| ledger gate column names check_inexistente | ledger-coherence | `audit/repair-report.md` | fail | 1 | [ledger-coherence] audit/repair-report.md:40: the gate column names check_inexistente, which is not a function in this f |
| ledger names a path that is not in the tree | ledger-coherence | `audit/repair-report.md` | fail | 1 | [ledger-coherence] audit/repair-report.md:23: names book-src/99-ghost-chapter.md, which does not exist in the tree |
| ledger gate column names a gate no reading prints | ledger-coherence | `audit/repair-report.md` | fail | 1 | [ledger-coherence] audit/repair-report.md:23: the gate column names 'citationn', which is neither a gate this file print |
| ledger gate column names an existing gate | ledger-coherence | `audit/repair-report.md` | pass | 0 |  |

---

## 4. Rodapé e CI

`main()` chama `check_year_citation()` e `check_ledger_coherence()` depois dos quinze
portões anteriores; os dois falham pelo mesmo `fail(gate, detail)`, então entram no
rodapé `N structural failure(s)` como os outros. O passo de CI
(`.github/workflows/ci.yml`) passa a nomear `year citation` e `ledger coherence` para
que o job diga o que ele lê. Nenhum dos dois importa rede: `structure.py` continua
importando `glob, html, itertools, os, re, sys, unicodedata, urllib.parse, collections`,
e `urllib.parse` só decodifica percent-encoding de links.
