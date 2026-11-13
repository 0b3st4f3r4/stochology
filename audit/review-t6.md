# Revisão t6 — reparo de t5 (doutrina, garantia, portões)

Data: 2026-11-13. Revisor: epistemic-auditor. Objeto: árvore de trabalho sobre `9fc4e41`
(36 arquivos de `book-src/` + `.github/checks/structure.py` + `audit/repair-report.md`).
Nada editado em `book-src/`; nada commitado.

## Veredito: needs_revision

Quatro critérios de aceitação foram julgados. Dois passam, dois falham.
A doutrina sobrevive: as três proposições de colapso saíram mais fortes e agora falsificáveis, `85:54`
ficou intocada, e nenhum portão foi afrouxado. O que falha é (a) um resíduo §I no próprio capítulo da
tríade, (b) uma citação perdida numa reescrita, (c) uma frase quebrada por uma substituição, e (d) o
ledger não descreve vinte dos trinta e seis arquivos que a árvore altera.

## Tabela de aceitação

| Critério | Veredito | Evidência |
| :--- | :--- | :--- |
| Três proposições com a mesma força de antes | passa | P1/P2/P3 em `41:199–225` seguem universais na forma condicional e agora carregam cláusula de refutação; P1 ganhou $\Delta_{\mathrm{id}}$, que é o enunciado do próprio §I |
| Nenhuma passagem nova viola §I | **falha** | `41:348` manda o veredito financiar um eixo (F1) |
| Toda citação preservada nos parágrafos reescritos | **falha** | `14:191–193` perdeu `(LaLonde 1986; Dehejia & Wahba 1999)` (F4) |
| Portões auditados no código | passa | 5 mutações plantadas, todas disparam; 3 hunks de remoção em `structure.py`, os três estritamente mais fortes |

## Achados

### F1 — GRAVE — o veredito ainda emite ordem

`book-src/41-the-triad-and-the-minimum.md:348` — "The procedure is to declare saturation and to name the
critical axis (the smallest one) as the axis the next step **must fund**, abandoning the composite score".
O reparo trocou "use … as the single lever" por "name … as the axis the next step must fund": renomeou a
alavanca e manteve a diretiva. O mínimo continua selecionando a ação. Cláusula violada: AGENTS.md §I,
último parágrafo ("acting is done by the per-step canonical operator, never by the aggregate; any text
that turns the diagnosis into a decision rule … has broken this clause"). Correção: "…and to name the
critical axis as the one the next step will face, leaving the funding decision to the operator's own
per-step state" — sem imperativo e sem "must".

### F2 — MÉDIO — frase quebrada introduzida pelo reparo

`book-src/22-thermodynamics-and-entropy.md:261–262` — "…carries $1/T_0$ watts per kelvin out of the
building, **and and** the erasure ceiling the same power draw could support is …". O "and" da linha
original ficou e o novo acrescentou outro. Cláusula: §III (markdown legível) e o dever de reparo
verificado; nenhum portão lê prosa. Correção: apagar um dos dois "and". No mesmo período, `$T\_0$` e
`$T_0$` convivem (deriva de notação que o portão `notation` não cobre). O "never" de ":262–263"
("erasure is never the term an institution runs into") é magnitude em palavra, contra §IV-2 e contra o
próprio critério 3 de t5; trocar por "not the binding term at any institutional power draw".

### F3 — MÉDIO — vinte arquivos sem item de ledger

`audit/repair-report.md:110–115` declara 16 arquivos tocados; a árvore altera 36 (`git status`). Sem item
em nenhum dos quatro relatórios: `03`, `07`, `12`, `14`, `16`, `18`, `19`, `22`, `23`, `26`, `28`, `36`,
`37`, `38`, `42`, `44`, `46`, `48`, `49` e `appendix-c`. Os dois defeitos acima (F1 à parte) estão
justamente nesse conjunto. Cláusula: §V (o que se verifica é o texto) e o critério 5 de t6 — sem ledger,
"nenhum não-defeito foi reparado" não é verificável. Correção: estender o ledger com proveniência e
portão para cada um desses arquivos, ou reverter os que não têm achado de origem.

### F4 — MÉDIO — citação perdida

`book-src/14-linear-algebra.md:191–193` — o parágrafo trocou o benchmark citado por um cenário genérico:
"(LaLonde 1986; Dehejia & Wahba 1999)" desapareceu e a fórmula $n/(1+(\bar m-1)\rho)$ entrou no lugar.
A fórmula se sustenta sozinha (§IV-2), o cenário não: "A labor-ministry briefing cites the household
interviews of a national population survey" não tem fonte alguma. Cláusula: §IV, "Citations are the
asset; when a paragraph is rewritten, its sources travel with it". Correção: manter o par LaLonde/Dehejia–
Wahba no cenário e citar Kish (1965) para o fator de desenho, ou remover o cenário e deixar só a derivação.

### F5 — BAIXO — magnitude em palavra trocada por outra

`book-src/48-reactive-worlds-performativity-goodhart.md:376` e `:388` — "by a factor that grows with the
distance the stable point has travelled" e "grows with the distance between the two regions". Saiu "an
order of magnitude larger" e entrou uma monotonia sem álgebra e sem citação na frase, embora Perdomo et
al. (2020) esteja no capítulo. Cláusula: §IV-1/2. Correção: exibir a desigualdade que dá a taxa
(risco performativo Lipschitz na distância) ou citar Perdomo et al. (2020) na mesma frase.

### F6 — BAIXO — valor de literatura mexido sem a fonte em mãos

`book-src/36-decision-loss-threshold.md:316` — "roughly forty-five million" virou "more than forty
million" para (Neustadt & Fineberg 1983), que t1 declarou INVERIFICÁVEL. A direção é conservadora, mas
§IV proíbe reescalar valor de literatura e o item não está no ledger. Correção: registrar o item com o
que se verificou, ou devolver o valor à forma que a fonte sustenta.

### F7 — BAIXO — datas alteradas sem fonte

`book-src/26-regulation-and-homeostasis.md:135` ("his lectures of 1878") e
`book-src/28-the-machine-and-the-deception.md:229` ("the defence authorisation passed in 1969"). Ambas
são afirmações datadas sem citação; a de 26 escapa do censo porque um ano solto satisfaz o predicado de
`CITATION`. Cláusula: §IV (garantia declarada na frase). Correção: citar a fonte da data (Bernard 1878;
a lei de autorização) ou retirar o ano.

### F8 — BAIXO — o portão promete mais do que lê

`.github/checks/structure.py:991–992` diz que a condição de refutação "may sit in that paragraph or in a
sentence of the same section"; `check_collapse_refutation` (`:1010–1026`) lê apenas o parágrafo que abre
em `**Collapse Proposition N**`. Consequência: reenunciações fora dessa forma (por exemplo `50:389`) não
são lidas, e o comentário sugere cobertura que não existe. Correção: implementar a varredura de seção ou
corrigir o comentário para declarar o limite.

## O que foi verificado e está limpo

- `book-src/85-…-urban-biospheric-order.md:54` intocada, e a sede que a sustenta existe:
  `book-src/40-institution-incentive-metric.md:172` — "the governance coordinate is not one reading among
  three: it fixes the sign of the differential on which the other two are priced". O achado A3 de t3 era
  leitura invertida; o rebaixamento está correto.
- `85:58` obedece à licença de publicação de `41:359–361`; "price of one weak axis" saiu.
- `20:196–199` declara a convenção (produto = conjunção no gráfico presença/ausência), o que reconcilia a
  exibição com `41:339` e `appendix-a:217`; a exibição não foi tocada.
- `appendix-e:66` agora funda sete em "six ordered crossings of distinct planes and the integral closure";
  a Tabela E.1 lista sete trânsitos. Resolve §V-10(e) sem tocar a tabela.
- `appendix-a:294`: o benchmark de cargas e capacidades iguais é declarado no próprio apêndice, a álgebra
  ($L_0+L_0/z>C \iff \rho>z/(z+1)$) está exibida ali, e `book-src/53-…:23` instancia exatamente esse caso
  ("spread equally over $z$ neighbours that also carry baseline load $L$"). O link virou remissão, não
  autoridade — E2 resolvido.
- Nenhuma citação perdida fora de `14` (`05` ganhou `Wong et al. 2021`; `67` mantém Chao 1987 e acrescenta
  Chao & Jost 2012; `38` acrescenta Mangel & Samaniego 1984 a Appendix C).
- `41:199–225`: as três proposições continuam universais na forma condicional; P2 preserva o parágrafo
  que já era o modelo de falsificabilidade; `50:389` nomeia agora $\rho_{R,Y}$, $\mathrm{DEFF}$, a largura
  de identificação parcial e $\mathrm{TTS}<\mathrm{TTR}$, então a reenunciação fraca deixou de contradizer
  a sede.
- `19:262–266` cita "the identity of Chapter 49" e a identidade existe: `49:220` §8.3.2 prova
  Chouldechova (2017), e a razão de FPR = razão de chances das taxas-base vale sob PPV e FNR iguais.
- `12:250–252` e `42:207/229` substituem ordem de magnitude por álgebra conferida: $W=\rho/(1-\rho)$ dá
  1, 9, 19 em 0.5, 0.9, 0.95; $\phi\approx(1-\rho)^{-1}$, $\phi''=2(1-\rho)^{-3}$ dão 10× e 1000×.

## Portões — auditoria no código

`DECLARED_OPEN = {}` (`structure.py:518`) com a regra de um site por declaração preservada em `declare()`
(`:527`); `report_stale_declarations()` segue ativo. Os quatro sites declarados foram reparados no texto,
não dispensados.

Só três hunks de `structure.py` apagam linhas pré-existentes, e as três substituições são mais estritas:
docstring; `check_links` (resolução por âncora de heading no lugar de existência de arquivo); `check_ledger`
(`has_entry(surname, year)` no lugar de `surname in bibliography`). Nenhum portão afrouxado.

Mutações plantadas em cópia de `/tmp`, todas revertidas, todas disparando:

| Mutação | Resultado |
| :--- | :--- |
| `appendix-a:217` "The minimum is not a weighted sum" → "is a weighted sum" | exit 1 `[concordance]` + `[concordance-polarity]` |
| citação falsa `(Fakename 1999)` em `appendix-a` | exit 1 `[citations]` |
| remover a cláusula "It fails if…" de P3 | exit 1 `[collapse-refutation]` `41:219` |
| restaurar "arranged over the three transits" em `appendix-e` | exit 1 `[guard-footing]` |
| linha de ledger com `Nobody (1999)` | exit 1 `[ledger]` row 2 |

Execução na árvore: `structure.py` exit 0 "all structural gates pass" (censo 523 advisory, órfãos 10,
display-first 25, near-dups 0, cross-refs 15, notation drift 4 = baseline, declared open 0);
`wasm_gates.py` exit 0, 18 gates, 0 failed, 8 declared open, nenhum marcado pass.
