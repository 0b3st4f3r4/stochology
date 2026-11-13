# Revisão t21 — Rodada 8 (t20) e portão `ratio-limit` (t19)

Objeto: `audit/repair-report.md` §"Rodada 8 (t20)" e o portão instalado em
`.github/checks/structure.py` §`check_ratio_limit`. Revisor: epistemic-auditor.
Estado da árvore revisada: 47 arquivos modificados, +1298/−145, nada commitado.
Nada em `book-src/` nem em `.github/checks/` foi editado por mim.

**VEREDITO: needs_revision.**

Uma promessa do bloco A não fecha: o exercício 4 de 51 manda o leitor seguir "the six
outgoing edges" e atribui o material à Bridge, mas a própria regra que a rodada
instalou (`appendix-b:171`: a ascendência é o que os blockquotes imprimem) implica
oito filhos para o nó raiz — e a Bridge imprime o numeral, não as arestas. Dois
defeitos no mesmo sítio, o primeiro alto: um numeral falso no único capítulo cujo
exercício depende dele. Todo o resto dos blocos A–D e o portão passam.

## A. Promessas

| Item | Veredito | Evidência |
| :--- | :--- | :--- |
| `68:18` sem o "Disciplinary Incompleteness Theorem", ancorado em P2 | fechado | Texto atual: "Show, from the survival-time condition of **Collapse Proposition 2**, why a `(2, 0, 1)` regulatory architecture omitting $S_{\mathrm{vital}}$ takes it when review latency outruns the patient population's survival horizon." A condição final que P2 carrega em `41:205-214` ($\mathrm{TTS}<\mathrm{TTR}$) é preservada. |
| `69:18` idem | fechado | Mesma reancoragem, "when replication horizons outlast deployment"; `69:17` troca o teorema nomeado pela fórmula de `69:24`, com $u(1-\alpha)$ no denominador — a fórmula impressa é a que o capítulo deriva. |
| `67:18` coerente com §4–§5 | fechado | A outcome desce da promessa de P3 para a tese do corpo: "leaves the ratio $d/T$ with no site charged with publishing it, while the window is still open", que é literalmente `67:190-192` ("no local site is charged with publishing the ratio: the laboratory owns $T$, the tracing …"). $d/T$ está definido em `67:144,155,157`. Nenhuma promessa de captura sobrou no sítio. |
| Exercício 4 de 51 respondível com material impresso | **NÃO FECHA** | ver F1 e F2. |
| Promessa de B.2 igual ao que a Table B.1 entrega | fechado | `appendix-b:171` passa a enumerar sítios de guarda, sítio primário, estrato e regime de exposição, e delega a ascendência aos blockquotes. Nenhuma coluna nova: o cabeçalho de `appendix-b:177` está inalterado, e `check_b1` (paridade §V-5) fica verde — as 35 linhas continuam alinhadas com os 35 blockquotes. |
| 34 dos 35 blockquotes carregam "It builds on…" e o raiz não nomeia nenhum | fechado | Contagem por `grep -l` = 34; `51` (o raiz) não tem a frase, exatamente como `appendix-b:171` agora afirma ("the root names none"). |

### F1 — high — out-degree do nó raiz: "six" contra os oito filhos que o livro imprime

`book-src/51-layer-micro-kinetic-sensorimotor-order.md:177` — a Bridge afirma que o
raiz de $\mathcal{G}_1$ exporta "across the **six** outgoing edges $\mathcal{G}_1$
declares for it", e `51:168-169` manda o leitor seguir esse mesmo número. Mas
`book-src/appendix-b-architecture-verification.md:171` define a ascendência como
"the chapter blockquote that names the layers it builds on", e os blockquotes
nomeiam **oito** filhos de Layer 01:

`book-src/52-meso-buffered-queue-order.md`, `book-src/53-layer-macro-coupled-topology-order.md`,
`book-src/54-layer-geophysical-chaos-data-assimilation-order.md`,
`book-src/55-continuum-fracture-structural-order.md`,
`book-src/56-layer-formal-computable-type-order.md`,
`book-src/61-layer-biochemical-kinetic-molecular-order.md`,
`book-src/70-layer-acoustic-aesthetic-tragic-pedagogical-order.md`,
`book-src/76-layer-sequential-decision-bandit-real-options-order.md`.

Oito arquivos, cada um com "It builds on [Layer 01]" no blockquote. Pela regra que a
própria rodada instalou, out-degree(L01) = 8. O capítulo se contradiz com o corpus no
único ponto em que o leitor é mandado contar.

Correção proposta: trocar "six" pelo número que os blockquotes implicam, ou remover o
numeral — "across the outgoing edges the chapters that build on it print".

### F2 — medium — a Bridge não imprime aresta alguma

`book-src/51-layer-micro-kinetic-sensorimotor-order.md:168-169` — o exercício pede
"what the six outgoing edges **this chapter's Bridge prints** do and do not license
about the orders they reach". A Bridge (`51:175-178`) imprime o numeral e a frase
"$\mathcal{G}_1$ declares for it"; não nomeia um único nó de destino. O material
impresso está nos blockquotes dos filhos, não na Bridge. A rodada corrigiu a
inrespondibilidade trocando uma fonte inexistente por outra que também não imprime a
aresta.

Correção proposta: apontar o exercício para onde as arestas estão — "the chapters
whose blockquotes name this layer" — e deixar a Bridge dizer apenas que declara o
número.

## B. Os seis números

| # | Sítio | Destino | Fonte diz isso? |
| :--- | :--- | :--- | :--- |
| 1 | `85:26` | "cutting employment for some $35{,}000$ people" **removido**; "more than $98$ percent" e (Hutchings & Myers 1994; Task Force 1993) ficam | Deleção correta: t16 achou fontes públicas em ~30.000, incompatíveis com 35.000 na precisão da frase; §IV manda deletar o token e manter o argumento, que é o que sobrou: "the moratorium closed a fishery whose spawning biomass had fallen by more than $98$ percent since the early 1960s, taking the economy out from under hundreds of coastal communities (Hutchings & Myers 1994; Task Force 1993)". |
| 2 | `82:79-81` | **mantido**, conferido na história processual | Grimshaw v. Ford, 119 Cal.App.3d 757: júri fixou US$125 mi, o tribunal de primeira instância remitiu para US$3,5 mi, e a apelação confirmou em 1981 — que é exatamente o que a frase afirma ("which the trial court remitted to $\$3.5$ million and the appellate court sustained (California Court of Appeal 1981)"). 125/0,2 = 625 ✓. Sem erro. |
| 3 | `81:34` | **mantido**, janela confirmada | Dookhan: contratação em 2003, afastamento em fevereiro de 2012, demissão em março de 2012. "Between 2003 and 2012" é o que a fonte sustenta; a presunção conclusiva com exigência de materialidade está descrita corretamente (Commonwealth v. Scott). |
| 4 | `84:25` | faixa `0.70`–`0.80` **sai**; fica "$H$ above $0.5$" | É o que Hurst (1951) sustenta e o que o argumento usa; nenhum hedge novo foi introduzido — o limite inferior é o que a citação carrega. |
| 5 | `84:31` | Fukushima ganha a citação no próprio parágrafo | (National Diet of Japan Fukushima Nuclear Accident Independent Investigation Commission 2012), com entrada em `book-src/appendix-c-sources.md:469`. O parágrafo deixou de depender da citação de `84:39`. |
| 6 | `84:37` | Bhopal reescrito em Broughton (2005); "operator warnings were discounted" removido | "the vent-gas scrubber had been turned off three weeks before the release, the refrigeration unit had been drained of its coolant, and the flare system had been out of action for three months (Broughton 2005)" — é o que a revisão de Broughton relata, e a entrada existe em `book-src/appendix-c-sources.md:197` (doi:10.1186/1476-069X-4-6). A frase sem fonte saiu em vez de ganhar número. |

## C. Álgebra

- **`76:42-43` — $r>\alpha$ coerente.** A derivação agora é explícita: com
  $\tfrac12\sigma^2\beta(\beta-1)+\mu\beta-r=0$ o produto das raízes é negativo e
  $f(1)=\mu-r$, logo $\mu<r$ dá uma única raiz $>1$, $\beta_1\to r/\mu$ e o hurdle
  $\beta_1/(\beta_1-1)\to r/(r-\mu)=r/\delta$; em $\mu=r$ a raiz é 1 e o hurdle é
  indefinido; em $\mu>r$ a raiz positiva é $<1$. As três frases do texto conferem.
  $\alpha_W,\beta_W$ separam as taxas de Wald de $\alpha,\beta$ do modelo.
- **`78:36-40` — $k_{\mathrm{BM}}$ e a cota.** $k_{\mathrm{BM}}(n_{\mathrm{eff}})=1/(1+1/n_{\mathrm{eff}})$
  é o fator de encolhimento de Baker–McHale, e a cota $\varphi(k)/k=e^{-k^2/2}/(k\sqrt{2\pi})$
  é declarada sobre a probabilidade de cauda a $k$ desvios (Mills + simetria) e não
  sobre o tamanho da queda. Correto. Ressalva de redação em F3.
- **`84:33` — $\rho_{\mathrm{cc}}$.** $P_{\mathrm{fail}}(k)=(1-\rho_{\mathrm{cc}})p^k+\rho_{\mathrm{cc}}p$,
  $R_{\mathrm{par}}(k)=1-(1-\rho_{\mathrm{cc}})(1-r)^k-\rho_{\mathrm{cc}}(1-r)\le 1-\rho_{\mathrm{cc}}p$,
  $\partial P_{\mathrm{fail}}/\partial k=(1-\rho_{\mathrm{cc}})p^k\ln p<0$ e a série
  $r^k$ com $r^k\ln r<0$: tudo confere, e a notação não colide mais com o $\rho_t$ de
  utilização em `84:27`.

### F3 — low — "the sampling size it counted above"

`book-src/78-layer-macro-finance-econometrics-actuarial-order.md:38-40` — "and $k$
here counts standard deviations, not the sampling size it counted above". Acima, $k$
é o subscrito do fator $k_{\mathrm{BM}}(n_{\mathrm{eff}})$, não um tamanho de amostra;
o tamanho é $n_{\mathrm{eff}}$. A distinção que a frase quer fazer está certa, mas o
nome que ela dá ao uso anterior está errado.

Correção proposta: "not the shrinkage factor whose subscript it named above".

## D. Menores

- **`77` — T1Framing argumentado.** `77:28` abre o §1 com "The framing site is the one
  a published model never prints. A strategic agent answers the question it is asked,
  so the objective the institution declares — which loss, over which population, on
  which horizon — is a strategic variable of the same kind as the weights…". O sítio
  que dava nome à tripla `{T1Framing, T2Severity, T6Performative}` sem aparecer no
  texto agora é argumentado e nomeado. Fechado.
- **"Pure Epistemic Concentration" tem sede.** `book-src/appendix-a-mathematics.md:263`
  (A.6, item 24, partição de Vandermonde) nomeia e conta a classe —
  "**Pure Epistemic Concentration ($k_1=3$):** $\binom{3}{3}\binom{2}{0}\binom{2}{0}=1$
  order" — ao lado das outras quatro (Trans-Triadic 12, Governance 9, Vital 9,
  Epistemic 4), e `59:3,12,101` e `appendix-b:187` usam o mesmo nome. A classe deixou
  de ser rótulo sem definição. Fechado.
- **`part-v-introduction:31`.** Passa a "the numbered sections, which argue the order's
  own subject and then read the signature", que é verdadeiro nos 35 capítulos (só
  51–53 rotulam sítios nos títulos). Fechado.

## Portões

- **`ratio-limit` existe e está vivo.** `check_ratio_limit()` em
  `.github/checks/structure.py:1555`, chamado em `main()`. Chave
  `f"{top}/({','.join(sorted(bottoms))})"`, escape por positividade do denominador
  (`denominator_positive`, lida sobre o arquivo), exigência de desigualdade na seção
  (`constrained`), declaração single-use.
- **Mutação plantada por mim** (cópia integral em `/tmp/rev21`, `tar` sem `.git`):
  - `$g(x)\to\alpha/\beta$` em `09` **não** dispara — escapa por `denominator_positive`
    (09 glosa $\alpha>0,\beta>0$). É o escape documentado por t19, não uma falha nova.
  - `$g(x)\to\zeta/(\zeta-\eta)$` sem hipótese → exit 1,
    `[ratio-limit] book-src/09-sets-relations-functions.md:137: the limit \zeta/(\eta) names no inequality between \zeta and \eta in its section`.
  - controle com "$\zeta>\eta>0$" na mesma seção → exit 0.
  - **reverter `76:43`** para "tends to $r/(r-\mu)$" → exit 1,
    `[ratio-limit] book-src/76-…-order.md:43: the limit r/(\mu) names no inequality between r and \mu in its section`.
    É a prova que importa: o que silencia o portão é o **reparo do texto**, não uma
    declaração.
- **`48:398` continua aceito**, e por hipótese e não por escape: "fix $\lambda > 0$ and
  let reactivity move past unit shift ($\varepsilon = 1$) toward the resonance pole at
  $1+\lambda$", com $\theta_{\mathrm{PS}}=\mu_0/(1+\lambda-\varepsilon)$ divergindo em
  $\varepsilon\to1+\lambda$ e $\theta_{\mathrm{PO}}\to-\mu_0/(1+\lambda)$. Aritmética
  conferida.
- **`DECLARED_OPEN` sem `76:43`.** O dicionário está vazio em
  `structure.py:518`, com o comentário de ratchet. Limite honesto: a árvore mudou às
  20:06:26, **depois** de t20 — o ledger de t20 registou a remoção como fora do
  inScope dela. O critério está satisfeito na árvore revisada; não pela mão do
  reparador, e isso está declarado no ledger, não escondido. `report_stale_declarations()`
  fica em silêncio numa rodada verde.
- **Nenhum portão afrouxado.** Diff de `structure.py` contra HEAD: 16 linhas
  removidas, todas já conhecidas — 4 do docstring, 7 de `check_links` (resolução por
  âncora substituiu a checagem de existência) e 5 de `check_ledger`
  (`has_entry(surname, year)` no lugar de `surname in bibliography`). As duas trocas
  são mais estritas. `wasm_gates.py` inalterado nesta rodada.
- **Rodada verde.** `structure.py` exit 0, "all structural gates pass" (census 521,
  órfãos do Appendix C 9, near-dups 0, cross-refs 15, notation drift 3, declared open
  0). `wasm_gates.py` exit 0, "18 gates: 10 decided here, 0 failed, 8 declared open
  (none of them marked pass)".
- **Ledger coerente.** `audit/repair-report.md:318-360`: 16 linhas de tabela cobrindo
  todos os arquivos tocados, mais "Conferências que não viraram edição" (B2 em 82, B3
  em 81, com a razão de cada um), "Portão provado por mutação" e o "Resíduo declarado,
  com o sítio e a ação". Nada foi editado sem achado de origem.

## Placar

| | |
| :--- | :--- |
| Bloco A | 4 de 5 fechados; exercício 4 de 51 **não fecha** (F1, F2) |
| Bloco B | 6 de 6 com destino verificável; 2 conferidos por mim na fonte processual |
| Bloco C | 3 de 3 corretos; 1 ressalva de redação (F3) |
| Bloco D | 3 de 3 fechados |
| Portões | `ratio-limit` instalado, provado por mutação própria, sem afrouxamento |

Achados: **1 high, 1 medium, 1 low**. O defeito é de contagem, não de doutrina: a
rodada ligou a ascendência aos blockquotes e não contou os filhos do raiz. Consertar
F1 e F2 fecha a rodada 8.
