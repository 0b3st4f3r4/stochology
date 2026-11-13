# Auditoria epistemológica — o livro contra o próprio contrato

**Objeto.** `book-src/` (86 capítulos, cinco apêndices, `SUMMARY.md`, introduções e conclusões de parte), confrontado com `AGENTS.md` §I–§V.
**Método.** Leitura do contrato inteiro; varredura de prosa (sem `$`, sem `<svg>`, sem cerca de código) por semântica de agregação, por imperativos de ação, por numerais sem garantia e pela fundamentação da contagem de sete; verificação linha a linha com `grep -n` e leitura de contexto. Nenhuma edição em `book-src/`.
**Estado do texto.** `HEAD = 9fc4e41`, árvore de trabalho com 24 arquivos modificados por outros auditores. Os números de linha abaixo foram reconferidos neste estado; se o defeito não aparecer na linha citada, procure a frase citada — a frase é a chave, a linha é a conveniência.
**Autolimite (§IV).** Isto é uma leitura por amostragem dirigida, não um censo exaustivo de 86 capítulos. Onde não achei defeito, digo que não achei, e com que rede procurei. Nenhum achado abaixo é sustentado por uma varredura automática sozinha: cada um carrega a frase citada.

**Placar.** 8 GRAVE, 6 MENOR, 12 linhas liberadas com a garantia que as libera, 5 lacunas de portão.

| Frente | GRAVE | MENOR |
| :--- | :---: | :---: |
| 1. Tríade como mínimo | 3 | 1 |
| 2. Diagnóstico não é lei de controle | 2 | 1 |
| 3. Taxonomia de garantia | 0 | 2 |
| 4. Falsificabilidade das três proposições | 3 | 0 |
| 5. Regra de autoridade e contagem de sete | 1 | 3 |

---

## 1. A tríade como mínimo, nunca como agregado

### GRAVE

**A1 — `book-src/20-legitimacy-and-institutions.md:199` — o capítulo define "accountability" como um produto de três componentes que ele próprio chama de não-substituíveis, e o produto é compensatório em todo o cubo menos nos cantos booleanos — §I (margens não-substituíveis; `M_Leontief = min`) e §V-9 (o mínimo não é uma soma ponderada).**

O texto diz, em 197, "accountability requires three non-substitutable structural components", e imediatamente exibe

$$\mathrm{Accountability} = \mathrm{Observability} \times \mathrm{Authority} \times \mathrm{Consequence}.$$

Em 205–209 a defesa é o colapso a zero: "If any single element is missing, the product collapses to zero." Isso vale em $\{0,1\}^3$; no contínuo $[0,1]^3$ que a própria frase invoca ("Where all three factors equal $1$"), o produto deixa de ser um mínimo: $(0{,}5,\,1,\,1) \mapsto 0{,}5$ e $(0{,}9,\,0{,}7,\,0{,}8) \mapsto 0{,}504$, isto é, subir as duas coordenadas saudáveis eleva a leitura agregada com a deficiente parada — a troca que §I proíbe. O próprio livro já sabe disto e diz o contrário em outro lugar: `book-src/41-the-triad-and-the-minimum.md:339` — "Product aggregation contracts volume against the minimum … it changes the size of the reserve a reader would post" — e `41:349` calcula $V_{\mathrm{prod}}/V_{\min} \to 1/6$, tratando produto e mínimo como funcionais distintos. O capítulo 20 nunca declara a convenção (booleana) que reconciliaria a exibição com a palavra "não-substituível".
**Correção proposta.** Ou declarar a convenção na mesma frase ("on the Boolean chart, where presence is $1$ and absence $0$, the product is the conjunction and coincides with the minimum"), ou trocar a exibição por $\min$ e reservar o produto para o parágrafo em que ele é discutido como funcional distinto. Não deixar as duas leituras disponíveis ao leitor.

**A2 — `book-src/85-layer-polycentric-commons-urban-biospheric-order.md:58` — o texto batiza o intervalo entre mínimo e média de "the price of one weak axis", isto é, cunha uma taxa de câmbio entre a deficiência de um eixo e um escalar — §I (margens não-substituíveis) e `book-src/appendix-a-mathematics.md:217` ("The minimum is not a weighted sum and carries no exchange rate").**

Frase: "For axis scores whose minimum falls well below their arithmetic mean, the gap between the two is the price of one weak axis — the number a commons institution should publish if it publishes anything about its own resilience." A mesma grandeza é descrita, corretamente, em `book-src/27-what-judging-is.md:190`: "the distance between them is the slack the weakest axis has left, a quantity the mean never reports" — folga, não preço. "Preço" converte folga em valor de troca: se a distância é o preço de um eixo fraco, ela é comparável entre eixos e somável ao longo do tempo, que é exactamente o que §I nega. E `appendix-a:217` afirma o oposto em uma linha: nenhum exchange rate.
**Correção proposta.** Manter a descrição de 27:190 ("the gap is the slack the weakest axis has left") e apagar "the price of one weak axis". A cláusula de publicação pertence ao achado B2 e cai com ele.

**A3 — `book-src/85-layer-polycentric-commons-urban-biospheric-order.md:54` — a frase nega que a coordenada de governança entre na função mínimo, quando §I a define como um dos três argumentos — §I (`M_Leontief = min(S_epi, S_vital, S_gov)`) e §V-9, linha da governança.**

Frase: "The governance axis is therefore not one reading among three—it does not enter the minimum as one reading among three, because it sets the terms on which the other two are read."

Duas coisas erradas, e a segunda é pior. (i) A duplicação — a string canônica de §V-9 ("not one reading among three") colada a um glosso que a excede — é a assinatura de um remendo feito para passar o portão, não de uma frase pensada. (ii) O glosso nega o que a definição afirma: se $S_{\mathrm{gov}}$ "does not enter the minimum", o mínimo tem dois argumentos, contra `AGENTS.md:19`, `book-src/09-sets-relations-functions.md:295` e `41:288–292` (a governança possui dois sites, `T6` e `T7`). A leitura caridosa — "não é *apenas* uma leitura entre três" — exige a palavra "merely", que a frase recusa.
**Correção proposta.** "The governance coordinate is not merely one reading among three: it also fixes the terms on which the other two are read, and it enters the minimum as their equal." E ver G2: o portão atual não vê a negação.

### MENOR

**A4 — `book-src/27-what-judging-is.md:186` — vocabulário de agregação ("scores … are aggregated by") para uma dobra não-compensatória — §V-9, espírito ("o mínimo não é uma soma ponderada"), não a letra.**

"the three dimensionless safeguard scores $(S_{\mathrm{epi}}, S_{\mathrm{vital}}, S_{\mathrm{gov}}) \in [0,1]^3$ are aggregated by their non-compensatory Leontief minimum (the $\sigma \to 0^+$ limit of a constant-elasticity-of-substitution aggregate)". O conteúdo está certo e a álgebra acompanha; resta o substantivo. "Score" e "aggregate" são as palavras que o resto do livro gasta para denunciar o funcional compensatório, e §V-9 só proíbe quatro variantes literais.
**Correção proposta.** "are folded by their non-compensatory Leontief minimum", reservando "aggregate" para o caso CES genérico em que $\sigma$ é parâmetro.

### Liberado com garantia (não reabrir sem ler a linha)

`08:149` ("You cannot trade one to compensate for the absence of another") e `08:161` (média aritmética = substitutibilidade infinita, $\partial M_{\mathrm{arith}}/\partial S_i = 1/3$); `09:295–301`; `27:190`; `41:335–339` ($M \le S_i$, e a média não); `49:179` ($C \iff P_1 \land P_2 \land P_3$, a leitura conjuntiva explícita); `50:330–331` (a média aritmética "issues a false-safe pass", o Leontief "records every catastrophic failure"); `appendix-a:217` e `298`; `part-ii-conclusion:25` ("No exchange rate exists between them"); `63:83` (conjunção contra soma ponderada); `75:20` ("A zero in any factor is absorbing") — este último é produto de *fatores de qualidade de arquivo*, não das margens da tríade, e o zero absorvente coincide com o mínimo no ponto que importa; não é violação, mas o capítulo não declara o gráfico e fica a um passo do defeito A1.

---

## 2. O diagnóstico não é lei de controle

### GRAVE

**B1 — `book-src/41-the-triad-and-the-minimum.md:256–257` — o parágrafo da proibição constitucional atribui aos avaliadores do veredito dobrado o poder de parar a operação, e três parágrafos depois o capítulo nega que o veredito acione qualquer coisa — §I, último parágrafo ("acting is done by the per-step canonical operator, never by the aggregate").**

O texto: "Hence the constitutional prohibition: **the Nuclear Triad is a diagnostic, never an optimization objective.** It is evaluated by independent auditors possessing sovereign authority to halt operations." Contra, na mesma página: `41:266–270` — "A number minted after the last decision of a shift cannot steer that shift … Reading $M_{\mathrm{Leontief}}$ as a live trip is the misreading this section exists to prevent" — e `41:279` — "whenever $M_{\mathrm{Leontief}}$ breaches the survival threshold it records the breach and names the binding axis. It trips nothing."

A contradição é de gramática, não de intenção: "It is evaluated by auditors possessing authority to halt" faz da avaliação do agregado a condição do ato de parar. `book-src/20-legitimacy-and-institutions.md:202` mostra qual é a sede correta desse poder — "**Authority**: The auditing body or affected party must possess the legal and operational power to halt, quarantine, or overturn the decision" — que é um componente estrutural da accountability, exercido por passo, não um poder do veredito. O capítulo 41 empresta a autoridade de 20:202 ao número que ele proíbe.
**Correção proposta.** "It is read by independent auditors, who hold the power to halt on the evidence of the per-shift record; the fold is theirs to read and not theirs to act on." — mantendo o poder, tirando-lhe o gatilho.

**B2 — `book-src/85-layer-polycentric-commons-urban-biospheric-order.md:58` — o capítulo recomenda que a instituição *publique* o número dobrado como medida da própria resiliência — §I, último parágrafo, e a proibição de `41:250–254`.**

Frase: "…the gap between the two is the price of one weak axis — the number a commons institution should publish if it publishes anything about its own resilience." O capítulo que fixa a doutrina diz o contrário em duas linhas: `41:250` — "The greatest operational threat to this framework is that the Leontief minimum becomes an institutional target" — e `41:251` — "Any published index tied to executive bonuses will be gamed". E `41:359` já escreveu a política correta de publicação: "if a scalar must be published, publish the smallest unhedged exposure measured in the units of the ruin barrier … and label it a threshold device, not a score". 85:58 publica exactamente o escalar que 41:359 manda não publicar, e no mesmo sopro o chama de preço.
**Correção proposta.** Substituir a cláusula final por: "…the gap is the slack the weakest axis has left; an institution that must publish a number publishes the smallest unhedged exposure in the units of its own barrier, not the gap." (Alinha 85 a 41:359 em vez de contradizê-lo.)

### MENOR

**B3 — `book-src/41-the-triad-and-the-minimum.md:343` — um procedimento no imperativo toma a ordenação do mínimo como alavanca de decisão — §I, último parágrafo; beira, não cruza, porque o veredito pode *nomear* o eixo vinculante (`41:279`).**

"A saturação … The procedure is to declare saturation and to use the critical axis (the smallest one) as the single lever, abandoning the composite score instead of reporting a number that no decision will move." Nomear o eixo mais fraco é licenciado; "the procedure is to … use … as the single lever" transforma o nome em regra. A distinção é fina mas é a distinção inteira que §I existe para guardar.
**Correção proposta.** "The diagnosis names the critical axis and stops there; the operator allocates the next step's slack to that axis on the per-step record, and no composite score is published."

### Liberado com garantia

`00:49` ("strictly as a trajectory-level diagnostic rather than a per-step control law"); `41:272` (testes de prefixo, contingência e selo — a melhor defesa mecânica do livro contra a lei de controle); `41:279`; `41:359` (recusa o argmin entre eixos e rotula o escalar como threshold device); `47:368` ("the smallest crossing buffer is the answer" — é o operador de dimensionamento por passo, não a dobra); `55:159`; `40:272`; `27:196` ("never a metric to be maximized"). Nenhuma ocorrência, em todo o corpus, de imperativo para maximizar a tríade fora de 41 e de 85 — a rede foi `optimize|maximize the minimum|tune the margin|choose the policy that|the operator should select`.

---

## 3. Taxonomia de garantia (§IV)

**Resultado principal: negativo, e é preciso dizê-lo.** Partes I–V estão disciplinadas. Duas redes independentes sobre capítulos 51–85 (numeral sem ano de citação e sem span matemático; quantidade com unidade — *million, per cent, kt, m/s, deaths* — sem ano de citação) devolveram apenas ruído de referência cruzada, assinatura de guarda `(1, 1, 0)` e alvo de link. O balde que o brief chama de 326 linhas aparece como **523** na minha replicação do predicado de `structure.py:388` (numeral na linha, sem ano, sem `$`, fora de numeração editorial); a diferença é de filtro, não de manuscrito — reporto o número e a divergência, não a resolvo.

### MENOR

**C1 — `book-src/41-the-triad-and-the-minimum.md:202–203` — um desfecho datado com a precisão de uma medição, sem garantia — §IV(b), "a numeric instantiation … is not a derivation".**

"…The reserve buffer is sized against a false point certainty and is completely overwhelmed on the first day real-world tail variability strikes." "On the first day" não é um valor de fonte nem uma derivação: é o tipo de número que §IV chama de inventado, e a violação é dupla porque a mesma frase (ver D1) é uma proposição que §I exige falsificável.
**Correção proposta.** "…and is overwhelmed on the first excursion beyond the support the mean assumes." (Sem data, sem dia, falsificável.)

**C2 — `book-src/33-measuring-and-erring.md:219` — uma correspondência bijectiva afirmada, não argumentada — §IV(2) (derivação exige a álgebra ao lado) e §V-10 (designação registrada, não derivada).**

"These three psychometric threats correspond 1-to-1 with the framing (`T1`), archive (`T3`), and performative (`T6`) guard sites of the grammar of aim." Três ameaças, três sites: a contagem fecha, e é só isso que fecha. Nenhuma razão é dada para que a ameaça psicométrica *seja* o site `T1` e não `T2`, e a frase usa o vocabulário da bijeção — o mesmo que §V-10(e) proíbe para os anchors ("strict 1-to-1 bijection" é variante vetada da proposição dos anchors).
**Correção proposta.** "…are answered at the framing (`T1`), archive (`T3`) and performative (`T6`) guard sites" — designação, não correspondência.

### Liberado com garantia (as tentações mais fortes, e o que as garante)

| linha | número | garantia |
| :--- | :--- | :--- |
| `05:169` | "about 16.1%" | árvore de frequências naturais exibida em 173–176 ($95/590$); derivação ao lado |
| `80:34` | "$\approx 3.8$ percent", "26 to 1" | $4.7/125$ exibido na mesma frase, insumos de Lowenstein (2000) |
| `85:62` | "THB 1.425 trillion, about \$46.5 billion" | World Bank (2012) na mesma frase, conversão marcada por "about" |
| `54:53` | "30 kt (15.4 m/s) … 95th percentile" | citação na mesma sentença (a linha quebra no meio dela) |
| `59:36–37` | "$+\$1{,}794.34$ / $-\$8{,}497.52$" | Dehejia & Wahba (1999) na sentença |
| `68:46–51` | "OR 1.43 [1.03, 1.98]" | Nissen & Wolski (2007), com a álgebra em escala log exibida |
| `82:74–80` | "$137$ million vs $49.53$ million" | Grush & Saunby (1973); Dowie (1977); Schwartz (1991), com as parcelas |
| `41:349` | "$V_{\mathrm{prod}}/V_{\min} \to 1/6$", "$5/6$" | as duas formas fechadas exibidas na mesma frase |

**Nota de portão.** `structure.py:662–684` (`check_math`) verifica paridade de delimitadores, não pertinência: um número na prosa ao lado de um display não é confrontado com o display. E o censo é advisory por desenho — `structure.py:1087` imprime o cabeçalho "numeral census (advisory, not a gate):" e `:1088` só chama `census()`, cujo resultado nunca entra na lista de falhas — de modo que nenhum portão hoje falha por um numeral sem garantia. Ver G3.

---

## 4. Falsificabilidade das três proposições de colapso

### GRAVE

**D1 — `book-src/41-the-triad-and-the-minimum.md:199–203` — a Proposição 1 não diz o que a refutaria e perde um dos três mecanismos que o contrato lhe atribui — §I ("each proposition is stated so that it could fail") e a formulação de `AGENTS.md:19`, que nomeia $\rho_{R,Y} \ne 0$, $\mathrm{DEFF}$ **e** limites de identificação parcial.**

O texto nomeia os dois primeiros e não o terceiro; e termina em desfecho ("completely overwhelmed on the first day"), não em condição. Compare com a Proposição 2, que carrega o parágrafo-modelo em `41:209–214`: "This is a theorem of the ruin model under the stated condition … Its empirical content is that the condition is reachable … what the proposition predicts is absorption once the barrier is crossed." A Proposição 1 não tem nada equivalente; a Proposição 3 também não (D2). O livro tem, portanto, uma proposição falsificável e duas que não são, sob uma cláusula que exige as três.
**Correção proposta.** Acrescentar limites de identificação parcial ao mecanismo e um parágrafo de refutação no molde de 209–214: "The proposition fails if a trajectory with an epistemic margin at zero is observed to size its reserve above the tail it later meets."

**D2 — `book-src/41-the-triad-and-the-minimum.md:216–220` — a Proposição 3 é enunciada sem qualquer condição de refutação — §I (mesma cláusula).**

Três orações no presente do indicativo ("operates under", "is weaponized", "are privatized") e nenhuma condição sob a qual o enunciado seria falso. Uma proposição que só descreve o mecanismo por dentro não pode falhar; §I exige que possa.
**Correção proposta.** Fechar com a condição: "It fails if an institution with one-sided limited liability and silenced frontline testimony is observed to hold its reserves through a shock that removes the epistemic margin."

**D3 — `book-src/41-the-triad-and-the-minimum.md:222–225` — o capítulo desautoriza em bloco as três proposições que acabou de enunciar como universais — §I e §IV (a mesma sede não pode dizer as duas coisas).**

"none of them claims anything about institutions in general" — mas 199 diz "An organization maintains…", 216 diz "An organization has…", e 229 diz "When two safeguards are absent, the remaining axis accelerates catastrophe" (universal, sem qualificador). A desautorização é a leitura correcta (as proposições são células de um desenho, como 50:389 desenvolve); o enunciado é que está na forma errada. A contradição existe entre sedes do *mesmo* capítulo, que é a pior espécie: o leitor que para em 203 levou uma lei universal, o que para em 225 levou uma célula.
**Correção proposta.** Reescrever 199 e 216 na forma condicional do desenho: "Where an organization holds physical reserves and lacks the epistemic margin, it sizes them against a false point certainty…", e deixar 222–225 como está.

**D4 — `book-src/50-methodological-families-reflexivity.md:389` — a reenunciação das três proposições é mais fraca que a sede canónica: troca mecanismo por "must appear", e nenhuma das três condições sobrevive — §I, e §V-9 por analogia (duas sedes da mesma proposição).**

"…The cell that keeps the vital margin and drops the epistemic one is where blind undersizing must appear." Sem $\rho_{R,Y}$, sem $\mathrm{DEFF}$, sem limites de identificação parcial, sem estatística de teste. Para a Proposição 2, a reenunciação perde até a desigualdade: "The cell that keeps the epistemic margin and drops the vital one is where impotent lucidity must appear" não menciona $\mathrm{TTS} < \mathrm{TTR}$. As duas frases são verdadeiras e mais fracas — e é exactamente assim que uma proposição entra em contradição consigo mesma por enfraquecimento: a sede fraca é a que um leitor cético pode aceitar, e ela não implica a forte. Note-se que `50:383` (o falsificador $F_4$ e o estreitamento a "regime-conditional boundary law") está **correto**: é declarado prospectivo, e um estreitamento declarado é falsificabilidade, não violação.
**Correção proposta.** Na célula, nomear o mecanismo herdado: "…is where blind undersizing must appear, sized against a false point certainty with $\rho_{R,Y} \ne 0$ and $\mathrm{DEFF} > 1$; the cell is the design's, the mechanism is Chapter 41's." Alternativa mais barata: remeter a condição a 41 em vez de a reescrever, e nunca reescrever.

### Nota de portão (D5)

`structure.py:827–831` diz, no comentário, que "§I states each collapse proposition so that it could fail", e `check_falsifiers` (`structure.py:840–880`) exige, para os **quatro falsificadores do capítulo 50**, que cada item diga "fails if". Para as três proposições, nada. `grep -n "fails if" book-src/41-*.md book-src/00-preface.md` devolve zero. O portão cobre o objeto vizinho e deixa o objeto da cláusula sem cobertura. Ver G1.

---

## 5. Regra de autoridade (§II) e contagem de guard sites (§V-10)

### GRAVE

**E1 — `book-src/appendix-e-lexicon.md:66` — o apêndice funda os sete sites em *três* trânsitos, quando a tabela que ele introduz lista sete e a contagem só se sustenta em seis travessias ordenadas mais o fecho — §V-10(e) ("none derives a count of guard sites from a source other than the circuit's transits") e §II.**

Frase: "Seven constitutive tensions, one per guard site, arranged over the three transits of the mediation circuit." A Tabela E.1 (linhas 70–76) dá a cada site um trânsito próprio — `T1` Institution → inquiry framing, `T2` Hypothesis → empirical test, `T3` Population → documentary record, `T4` Phenomenon → formal summary, `T5` Ensemble → historical trajectory, `T6` Policy rule → social field, `T7` Integral mediation circuit — sete células sob o cabeçalho "Transit and filter". Contra `part-v-introduction:13` — "Three planes, six ordered crossings and the closure give seven guard sites" — e `29:297` — "Three direct transits …, three reflexive locks … and one integral three-plane closure … give 3+3+1=7". Com três trânsitos, sete sites não são os trânsitos: são sete tensões distribuídas por três objetos, e o número sete passa a não ter fundamento nenhum. O defeito é de fundamento, não de redação: é a cláusula (e) a ser violada pela via semântica, sem usar nenhuma das duas frases que o portão procura.
**Correção proposta.** "Seven constitutive tensions, one per guard site, arranged over the six ordered crossings of distinct planes and the integral closure that make the seven transits." E conferir que a introdução da Tabela E.1 não repita "three transits" em nenhuma outra forma.

### MENOR

**E2 — `book-src/appendix-a-mathematics.md:294` — um apêndice cita um capítulo como fonte do benchmark em que a álgebra vale — §II ("an appendix may not cite a chapter as authority").**

"…equal redistribution across $z$ surviving neighbours ($L_0 + L_0/z > C$ once baseline utilisation $\rho > z/(z+1)$, equivalently $\alpha_{\mathrm{tol}} < 1/z$) in the homogeneous local benchmark of [Chapter 53]". A garantia forte está presente — Motter & Lai (2002) citado e $\alpha_{\mathrm{tol}} = (1-\rho_i)/\rho_i$ exibido na entrada anterior —, de modo que o capítulo não está a *provar* nada; está a fixar a premissa do modelo. Mas §II manda que a definição pertença ao capítulo e que o apêndice a receba, não que a peça emprestada. É a única ocorrência em todos os cinco apêndices (varredura por `as Chapter NN establishes|proves|shows` devolve zero: nenhum apêndice usa capítulo como prova).
**Correção proposta.** Declarar o benchmark no próprio apêndice em uma oração ("on a homogeneous graph where the failed node's load is shared equally among $z$ surviving neighbours") e deixar o link como remissão, no fim, sem função probatória.

**E3 — `book-src/85-layer-polycentric-commons-urban-biospheric-order.md:52` — a frase usa a segunda contagem ($7 \times 5$) como medida independente de exaustão, reintroduzindo o argumento que §V-10(e) proíbe pelo nome — §V-10(e), e a razão registada em `09:307–311`.**

"Because $\binom{7}{3} = 35$ exhausts every three-site combination of the seven guard sites across seven strata of five orders, a thirty-sixth layer would break the bijection between the two graphs." O argumento válido é de casa de pombos: $36 > 35$ triplas. "Across seven strata of five orders" não acrescenta nada a esse argumento — e o livro já estabeleceu que não pode acrescentar: `09:307–308` — "Thirty-five is also seven times five, and the five is not a second count of the same triples. It is the size a seven-stratum split forces, $35/7$." Ou seja, o cinco é *derivado* de 35; citá-lo ao lado de $\binom{7}{3}$ como se corroborasse é a forma exata da frase vetada, sem as palavras vetadas.
**Correção proposta.** "Because $\binom{7}{3} = 35$ exhausts every three-site combination, a thirty-sixth layer would have to repeat a triple, and the bijection between the two graphs would break." (Sem o cinco.)

**E4 — `book-src/33-measuring-and-erring.md:219` — ver C2: "correspond 1-to-1" é a linguagem da bijeção derivada aplicada a uma designação — §V-10, rationale ("designation recorded rather than derived").**

### Liberado com garantia

`09:303–315` é a fundamentação modelo do livro e deve ser citada como tal: $b(n)=\binom{n}{3}/n$ estritamente crescente, $b(n+1)-b(n)=(n-1)/3$, raiz única "for the reason any equation $b(n)=c$ does — $b$ is injective — and not because two independent counts happen to meet at seven"; `313–315` testa o fator cinco com o oitavo site ($56$ contra $40$). `00:272–273` (G1 por Vandermonde $12+9+9+4+1$; G2 com "The correspondence is recorded here … and is not derived from the table of orders"); `appendix-b:171` ("the designation recorded rather than derived") e `appendix-b:124` ("Both are argued in the chapters … this appendix is their reference statement"); `part-v-introduction:27`; `29:297`; `part-v-conclusion:25`; `50:426`; `41:288–292` (as três partições de sete — por eixo $3+2+2$, por trânsito $3+3+1$, por plano $2+2+2+1$ — coincidem, o que é uma verificação real da contagem).

---

## 6. Oportunidades de portão mecânico

Cada uma nasce de um achado acima; nenhuma exige ler prosa por semântica, que é onde §V declara o limite.

**G1 — Falsificabilidade das proposições de colapso (de D1–D4).** Estender `check_falsifiers` para uma passagem sobre `book-src/41-the-triad-and-the-minimum.md` §3: sob o título "The three collapse propositions", exigir três itens, cada um nomeando `$\neg S_{\mathrm{epi}}|S_{\mathrm{vital}}|S_{\mathrm{gov}}$`, e cada um contendo uma condição de refutação por marcador declarado (o contrato pode fixar a string: *"It fails if"*). Exigir também que a sede secundária (`50:389`) contenha os tokens de condição herdados ($\rho_{R,Y}$, $\mathrm{DEFF}$, $\mathrm{TTS}<\mathrm{TTR}$) ou uma remissão explícita a 41 — a escolha é entre herdar e remeter, nunca reescrever mais fraco.

**G2 — Negação invisível na concordância (de A3).** `check_concordance` (`structure.py:486–497`) compara substring após colapsar espaços; a string canônica "not one reading among three" passa mesmo quando a frase seguinte a nega. Acrescentar à tabela §V-9 as variantes negadas que preservam a violação: `does not enter the minimum`, `does not enter the minimum as one reading`, `is not an input to the minimum`, e — para a linha dos design effects — `does not compose additively`. Uma variante que só é segura quando negada não é segura.

**G3 — Numeral sem garantia (de C1, e do fato de o censo ser advisory).** Não transformar o censo em portão (produziria 523 falhas, quase todas ruído). Em vez disso, um portão de *diff*: se um patch acrescenta uma linha que o predicado do censo marcaria, e essa linha não traz ano de citação nem span matemático no mesmo parágrafo, o patch falha e o autor declara a garantia em uma linha de justificação. É o único desenho que pega "sized on the first day" sem paralisar o repositório.

**G4 — Capítulo como autoridade em apêndice (de E2).** Advisory primeiro, portão depois: em `book-src/appendix-*.md`, listar toda ocorrência de link para capítulo; falhar quando o predicado da oração que o contém for verbo de garantia (`establishes|shows|proves|derives|reports|fixes`). O caso legítimo (remissão) tem predicado de leitura (`is argued in`, `see`), e a distinção é lexical, não semântica.

**G5 — Fundamento da contagem (de E1).** Duas regras baratas: (a) proibir em `book-src/` o token `three transits` e qualquer `N transits` com $N \ne 7$ em frase que também contenha `guard site`; (b) exigir que toda frase que contenha `seven guard sites` tenha, na mesma frase ou na imediatamente anterior, um dos tokens `crossings`, `closure` ou `transit`. A cláusula (e) atual guarda contra duas frases; estas duas guardam contra a classe.

**G6 (menor, de B1/B3).** Em `book-src/41-*.md` e `85-*.md`, proibir o padrão `(halt|stop|halted)` a menos de 120 caracteres de `M_{\mathrm{Leontief}}` ou `minimum`, salvo quando a frase também contiver `names the binding axis` ou `trips nothing`. É grosseiro e vai gerar dois falsos positivos; os dois falsos positivos são mais baratos que a contradição de B1.

---

## 7. O que esta auditoria não fez

Não li os 86 capítulos inteiros: li o contrato inteiro, os capítulos 08, 09, 20, 27, 29, 33, 41, 47, 49, 50, 75, 85 e os cinco apêndices, e varri o resto por padrão lexical com leitura de contexto nas ocorrências. Não auditei valor de literatura contra a fonte (frente de outro auditor): quando digo "garantia presente", digo que a citação está na mesma sentença e no formato que §IV exige, não que o valor confere. Não medi a álgebra dos 452 blocos de display. Não executei `structure.py` até o fim: a execução completa estourou 60 s neste ambiente e foi interrompida, de modo que o "passa hoje" do brief não foi reconferido por mim — e essa é a única afirmação deste relatório que eu não posso sustentar com saída de ferramenta.
