# Auditoria de integridade argumentativa da Parte V (t17)

**Objeto.** `book-src/51-*.md` a `book-src/85-*.md` — trinta e cinco capítulos, um por ordem de
conhecimento — contra o que o corpus efetivamente sustenta: referências cruzadas, tripla de guarda,
léxico e outcomes/exercícios.

**Método.** Leitura integral de 51, 52, 53, 59, 60, 63, 67, 68, 69, 75, 77, 79, 82, 84, 85; leitura
dirigida dos demais (cabeçalhos, blockquote, parágrafo de colapso, outcomes, exercícios; vocabulário
por sítio contado por `grep`). Toda referência cruzada numérica de 51–85 foi conferida por script:
para cada `[Layer NN](NN-…)` e `[Chapter NN](NN-…)`, o número do alvo tem de ser `50 + NN` e `NN`.
**Resultado: 0 divergências** — o mapeamento ordem→capítulo está correto em todo o corpus, e as
pontes "The next chapter is [Chapter NN]" também. Onde o alvo foi lido, o conteúdo confere (tabela
em §1.3). Portão de estado: `python3 .github/checks/structure.py` → exit 0.

**Limite declarado.** Para os vinte capítulos não lidos linha a linha, o juízo da §2 é sobre
cabeçalhos, vocabulário de sítio e o parágrafo de colapso; um sítio pode estar argumentado em
prosa sem que nenhuma dessas três marcas o mostre, e o inverso também. Não afirmo mais do que
verifiquei.

**Placar.** 4 GRAVE, 4 MENOR, 6 notas de conformidade.

---

## 1. Referências cruzadas

O portão `cross-refs` (`structure.py:1195`) compara palavras ≥ 4 letras do título do alvo com a
linha que cita. Sobre 51–85 ele produz **um** advisory: `book-src/57-…-order.md:164`. Conferido: o
capítulo 56 não usa a palavra "regime", mas o bullet de identidade de 56 (`56:12`, "governance
axial exposure · cynical capture, Proposition 3") e a §4 de 56 (`56:84–104`) nomeiam P3; a frase é
defensável. **Nota, não achado.**

### 1.1 GRAVE — `book-src/51-layer-micro-kinetic-sensorimotor-order.md:168–169`

Exercício 4: "Count the direct and the transitive descendants of this order in the graph, and say
why the six arrows a reader can follow are not the ancestor cone", sustentado por
`book-src/51-…-order.md:178`: "across the six outgoing edges $\mathcal{G}\_1$ declares for it".
**O que o texto faz:** pede ao leitor que conte descendentes num grafo cujas arestas o corpus não
imprime, e chama de "legíveis" seis setas que não aparecem em página alguma. Table B.1
(`book-src/appendix-b-architecture-verification.md:177`) tem as colunas Order, Ch, Stratum, Knowledge
Order, Primary Site, Unique Guard Triple, Signature, Exposure Regime, Canonical Isomorphism — **não
há coluna de pais nem lista de arestas**; o único capítulo que imprime pais é
`book-src/60-layer-cryptographic-zero-knowledge-adversarial-order.md:100` ("from parents `L01`,
`L02` and `L08`"). O exercício é inrespondível a partir do livro, e a asserção de `:178` é
inchecável.
**Fix (uma linha):** imprimir as parent layers no bullet de identidade de cada um dos 35 capítulos
(ou uma coluna em Table B.1) e então o exercício passa a ter resposta.

### 1.2 MENOR — `book-src/appendix-b-architecture-verification.md:171`

"…the thirty-five orders of knowledge, each carrying its guard sites, its primary guard site, its
stratum, **its parent layers** and its exposure regime."
**O que o texto faz:** promete, para cada ordem, um objeto que nenhuma tabela entrega. Consequência
sistêmica: as arestas afirmadas em `book-src/54-…-order.md:135` (`L04` não é pai de `L05`),
`book-src/59-…-order.md:103`, `book-src/60-…-order.md:99`, `book-src/69-…-order.md:122`,
`book-src/80-…-order.md:63`, `book-src/82-…-order.md:101` não podem ser conferidas pelo leitor.
**Fix:** acrescentar a coluna "Parent layers" a Table B.1 e preenchê-la a partir da mesma fonte que
gera os bullets.

### 1.3 Verificadas corretas (alvo lido, conteúdo confere)

| site | alvo | o que a frase atribui | confere |
| :--- | :--- | :--- | :--- |
| `57:108` | `30:184` | índice de dominância de Mann–Whitney sobre pares, com a identidade $\mathbf{1}\{f(x_i)>f(y_j)\}=\mathbf{1}\{x_i>y_j\}$ | sim |
| `65:15` | `32:134` | "Every question arrives framed" | sim |
| `71:49` | `28:174` | evidencialidade morfológica obrigatória (Aikhenvald 2004) | sim |
| `75:37` | `73:43`, `73:17` | corpus share ≠ population share; "A factor at zero is absorbing" | sim |
| `75:41` | `37:159` | "Four leakage pathways" | sim |
| `85:46,54` | `25:308` | "The exact condition $\gamma_{\mathrm{select}} > 1/T$" | sim |
| `65:60`, `66:59` | `41:216–225` | P3 = alibi agnotológico + responsabilidade limitada | sim |
| `84:62` | 15, 17, 29, 32 | estratigrafia, epidemiologia, estatística oficial, evidência | sim |
| `83:59`, `82:101`, `81:57`, `80:63`, `79:65`, `78:59`, `77:63`, `76:78`, `70:105`, `69:122`, `62:11–13`, `61:11`, `54:134` | — | rótulos "join / converge / opens an edge" | sim |

---

## 2. A tripla de guarda contra o argumento

Verificado antes de tudo: a coluna Exposure Regime de Table B.1 é **consistente nas 35 linhas** com
o zero da assinatura (E = `T1,T2,T3`; V = `T4,T5`; G = `T6,T7`) — p.ex. `L02` `(0,1,2)` → Epistemic,
`L18` `(2,0,1)` → Vital, `L33` `(0,1,2)` → Epistemic. Os doze `(1,1,1)` são exatamente os doze
capítulos rotulados "trans-triadic keystone", e `book-src/part-v-introduction.md:31` diz "the twelve
`(1, 1, 1)` keystones". **Nota, não achado.**

### 2.1 GRAVE — `book-src/68-layer-pharmacovigilance-regulatory-toxicology-order.md:18`

Outcome 3: "Prove by the **Disciplinary Incompleteness Theorem** why a `(2, 0, 1)` regulatory
architecture omitting $S\_{\mathrm{vital}}$ takes **Collapse Proposition 2**…".
**O que o texto faz:** o capítulo promete uma prova a partir de um teorema que **não existe no
corpus**. `grep -rn "Disciplinary Incompleteness" book-src/` devolve apenas este outcome e o de 69
(`book-src/69-…-order.md:18`); o teorema não é enunciado, nomeado, definido nem provado em nenhum
lugar das 86 unidades. O corpo de 68 (§3, "Lucidity without a stop-work trigger") entrega exemplo e
uma frase — `68:135`, "That is Proposition 2 in its exact form" —, não uma prova, e não há como
provar por um teorema inexistente.
**Fix:** trocar o outcome por "Show, from the survival-time condition of Proposition 2, why
review latency outrunning the survival horizon ends at the barrier" (ou enunciar o teorema no
corpo antes de o invocar).

### 2.2 GRAVE — `book-src/69-layer-metascience-scientometrics-replication-order.md:18`

Mesmo defeito, segundo capítulo: "Prove by the **Disciplinary Incompleteness Theorem** why a
`(2, 0, 1)` metascientific audit lacking $S\_{\mathrm{vital}}$ takes **Collapse Proposition 2** when
replication horizons outlast deployment." O corpo (§3) argumenta o regime ("its regime is the second
collapse") e não prova nada a partir de um teorema que não é impresso.
**Fix:** idem — ancorar o outcome na condição de tempo de sobrevivência de P2 (`41:205–214`).

### 2.3 GRAVE — `book-src/67-layer-epidemiology-outbreak-surveillance-order.md:18`

Outcome 3: "Show why a `(2, 1, 0)` surveillance architecture that omits $S\_{\mathrm{gov}}$ takes
**Collapse Proposition 3** while the window is still open."
**O que o texto faz:** o capítulo atribui-se a demonstração de P3 (captura cínica: alibi
agnotológico, pilhagem de reservas comuns, desconto de testemunho) e o corpo não demonstra isso. A
§4–§5 de 67 argumentam outra coisa — que **nenhum sítio local é encarregado de publicar a razão**
("no local site is charged with publishing the ratio") —, o que é ausência estrutural de um sítio, e
não captura. O rótulo vem do bullet `67:12` ("cynical capture, Proposition 3"), que também não é
argumentado.
**Fix:** ou acrescentar ao capítulo o mecanismo de captura (quem se apropria, sob que
responsabilidade limitada, com que testemunho descontado), ou rebaixar o outcome para "Show why
omitting $S\_{\mathrm{gov}}$ leaves the ratio with no site charged to publish it".

### 2.4 MENOR — `book-src/77-layer-game-theory-strategic-adversarial-order.md` (corpo, §§1–3)

Tripla `{T1Framing, T2Severity, T6Performative}`. O corpo argumenta T6 (§1, ponto estável
performático; §2, massa manufaturada) e argumenta a **aritmética** que T2 nomeia (77:28, "a detection
probability $\pi$ and a penalty $p$ … $\pi p < b$"), mas a palavra "severity" não ocorre no capítulo
e **"fram" não ocorre nenhuma vez** (`grep -i fram` = 0): T1 é a linha publicada tratada como objeto
estratégico, não o enquadramento que fixa o que conta como caso. Dois dos três sítios são
argumentados; o terceiro é pressuposto.
**Fix:** nomear o sítio de enquadramento na §2 ("the published line is the frame that fixes what
counts as a case before any outcome is scored").

### 2.5 Distribuição da nomeação (contexto, não achado)

Apenas 51, 52 e 53 nomeiam sítios nos títulos de seção (`52:1. T4Formalization: …`,
`53:1. T4Formalization: …`); os outros 32 capítulos usam títulos temáticos e argumentam os sítios
por substituição. Onde li o corpo inteiro (63, 64, 65, 66, 71, 72, 73, 74, 75, 79), os três sítios
aparecem — p.ex. 63: §1 conjunção/T2, §2 "the margin is time"/T5, §3 custo/T7; 79: §1 T4, §2 T3,
§3 um parágrafo cobre os três colapsos.

---

## 3. Léxico

### 3.1 MENOR — `book-src/59-layer-experimental-sampling-causal-order.md:12,16`

"pure epistemic concentration" para a assinatura `(3, 0, 0)` — o único caso do corpus.
**O que o texto faz:** usa uma classe de exposição que `book-src/appendix-b-architecture-verification.md:173`
não define; ali a regra só cobre a omissão de **um** eixo ("the axis a triple omits is the order's
local axial exposure regime"). Table B.1 repete o rótulo (`L09` → "Pure Epistemic Concentration"),
de modo que o nome é consistente em dois lugares e definido em nenhum.
**Fix:** acrescentar uma oração em `appendix-b:173` definindo a classe de omissão dupla (as duas
exposições são locais porque os dois guardas ausentes não têm sítio próprio na ordem).

### 3.2 MENOR — `book-src/part-v-introduction.md:31`

"…the numbered sections begin with the site the order leads with."
**O que o texto faz:** descreve um padrão que 32 dos 35 capítulos não seguem (§2.5 acima). A §3 do
mesmo parágrafo promete "Each chapter does the same thing three times, once per axis", o que é
verdadeiro em substância e falso na forma dos títulos.
**Fix:** reescrever para "the three sections take the order's three sites in turn, whether or not
each section is titled after its site".

### 3.3 Consistência verificada (par a par)

| termo | uso na Parte V | definição que o governa | veredito |
| :--- | :--- | :--- | :--- |
| Layer `NN` | sempre a ordem `LNN` (blockquote: "Layer 01 of Part V"; corpo: "Layer 19 opens an edge") | `appendix-b:171` | consistente |
| Chapter `NN` | sempre o capítulo (nunca a ordem) | §II do contrato | consistente |
| guard site / transit | `T1`–`T7` são sítios; o trânsito é o cruzamento que o sítio guarda | `appendix-e:66` (reparado: "six ordered crossings … and the integral closure"), Table E.1 | consistente |
| guard triple | sempre um subconjunto de três de `T1`–`T7` | `appendix-b:173` | consistente |
| keystone | os doze `(1,1,1)` | `part-v-introduction.md:31` (única definição, de passagem) | consistente, mas sem entrada no léxico |
| axial exposure | o eixo que a tripla omite | `appendix-b:173` | consistente; ausente de Appendix E |

O léxico (§II: "Appendix E defines the classical, technical and political vocabulary") não traz
verbetes para "keystone", "axial exposure", "order" nem "stratum": quem quiser a definição tem de
achá-la em Appendix B. Não elevo isso a achado porque nenhum uso contradiz a definição; fica como
lacuna de cobertura.

---

## 4. Outcomes e exercícios contra o corpo

Estrutura: **35/35** com exatamente três outcomes e três a cinco exercícios; **nenhum** outcome
remete a capítulo futuro (`grep` de "Chapter \d\d", "next chapter", "will be" nos blocos = 0); o
vocabulário de cada outcome ocorre no corpo do próprio capítulo. Os três defeitos de conteúdo são os
de §2.1–§2.3, mais o de §1.1 (exercício inrespondível).

### 4.1 Padrão de template

Três outcomes de Stratum IV são quase idênticos entre si — o defeito que o contrato pediu para
priorizar:

- `67:18`, `68:18`, `69:18` O1: "Locate **X (`LNN`)** at the unique 3-combination `{…}` in Stratum
  IV." (similaridade 0,66–0,68 entre si);
- `68:18`, `69:18` O3: "Prove by the **Disciplinary Incompleteness Theorem** …" (0,67);
- `59:12`, `60:12` O1/O2: "Identify/Locate … Unify … under …".

O template em si não é defeito — a pergunta é se cada capítulo entrega o que o molde promete. Em 67
e 68/69 não entrega (§2.1–§2.3). Em 60 entrega: o outcome 1 ("the combination that completes the
seven-site union of its stratum") é verdadeiro — a união de `{T1,T4,T5} ∪ {T1,T2,T4} ∪ {T3,T4,T6} ∪
{T1,T2,T3} ∪ {T2,T6,T7}` só fecha os sete sítios com o `T7` de 60.

### 4.2 Exercícios conferidos como respondíveis (amostra lida)

`52:75` e `53:69` ("Reading the signature": por que `(0,1,2)` é exposição **epistêmica**) —
respondíveis, e o gabarito está certo: E = 0 em ambas. `60` (§3 "Why the layer carries a vital axial
exposure", `(1,0,2)`) e `56:158` (§ governança, `(1,2,0)`) idem. `67:125`, `73:83`, `84:37`
respondíveis com o corpo. Nenhum outro exercício de 51–85 exige material externo ao capítulo, com a
exceção de `51:168–169` (§1.1).

---

## 5. O que esta auditoria não fez

Não julguei se as ordens de conhecimento são as ordens certas, nem a verdade das fontes citadas (é a
frente de atribuição). Não reli linha a linha os vinte capítulos listados no limite acima. Não
alterei `book-src/`, não commitei, não deleguei.
