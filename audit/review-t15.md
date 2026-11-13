# Revisão t15 — Rodada 5 (t14): Mansfield, 45 milhões, MacKenzie, Chao

**Veredito: PASS.** Os quatro achados low de t13 estão fechados no texto, com fonte verificável e sem
hedge nosso passando por formulação da fonte. Portões verdes, nenhum afrouxamento, ledger coerente.
Dois achados low novos, nenhum bloqueante.

Revisado: t14. Tentativa: `a176dd53-5ce5-4e9a-922d-bf27bb80aba9`. Nada em `book-src/` ou
`.github/checks/` foi editado por mim; nada commitado.

---

## 1. F1 — Mansfield (book-src/28-the-machine-and-the-deception.md:229-231)

Texto agora:

> and the Mansfield Amendment, §203 of the military authorization act for fiscal year 1970
> (Public Law 91-121), enacted in 1969, had already required American defence research funding to show a direct
> and apparent relationship to a specific military function or operation.

- A citação está **na frase**, não no parágrafo: o estatuto, o número da lei, o ano fiscal e o ano de
  aprovação aparecem no mesmo período que carrega a afirmação. F1 fechado.
- Coincide com o que t11 confirmou (PL 91-121, §203, FY1970, aprovada em novembro de 1969, e a redação
  "direct and apparent relationship to a specific military function or operation").
- Entrada nova em `book-src/appendix-c-sources.md:515`: `Public Law 91-121 (1969). Department of Defense
  Appropriation Authorization Act, 1970, §203, 83 Stat. 204, 206 (approved 19 November 1969).` — completa
  (título, seção, Statutes at Large, data), na posição alfabética correta, e citada no texto (não é órfã).
- "attached to" saiu e a oração passou a dizer o que o §203 diz; a substituição de 1973 por 1969 agora
  tem garantia no lugar onde o ano aparece.
- **Limite meu**: não reconsegui abrir o PDF do Statutes at Large (o leitor devolve "unsupported content
  type: application/pdf"), então as páginas 204/206 não foram reconferidas por mim; os identificadores
  (lei, seção, ano fiscal, data) batem com t11 e a entrada é internamente consistente com um ato de três
  páginas. Registro como limite de verificação, não como achado.

## 2. F2 — 45 milhões (book-src/36-decision-loss-threshold.md:315-319)

Texto agora:

> The national immunisation programme that followed vaccinated over 45 million people (Centers for Disease
> Control and Prevention n.d.), was suspended on 16 December 1976, and left some five hundred cases of
> Guillain–Barré syndrome (Neustadt & Fineberg 1983).

Abri a URL citada — <https://www.cdc.gov/vaccine-safety/about/guillain-barre.html> — e ela carrega o
número, no assunto certo:

> In response to a potential pandemic strain of a flu virus in 1976, there was a national campaign for
> people to get the swine flu vaccination. **Over 45 million people were vaccinated** and there were
> increased reports of GBS following vaccination.

- O valor corresponde ao que a fonte citada reporta, na formulação e na precisão da fonte ("over 45
  million"), com a desigualdade preservada. O hedge "roughly", que era nosso e amaciava o número, saiu.
- Neustadt & Fineberg ficam atrás apenas dos casos de Guillain–Barré, como o contrato da rodada pedia.
- Entrada em `book-src/appendix-c-sources.md:211`, completa (autor institucional, título, seção do site,
  URL), na posição alfabética correta, citada no texto.
- Corroboração interna: a mesma página dá ~1 caso adicional por 100.000 vacinados ⇒ 45 milhões × 10⁻⁵ ≈ 450,
  compatível com "some five hundred" — os dois números da frase não se contradizem.

## 3. F3 — MacKenzie (book-src/37-model-fit-and-error.md:192-196)

Texto agora:

> In sociology, Donald MacKenzie (2006) demonstrated that economic models exhibit two forms of performativity:
> *Barnesian performativity* … and *counter-performativity*, where adopting a model creates incentives that
> destroy the model's own preconditions (e.g., portfolio insurance rules that sell as the market falls, the
> case MacKenzie (2006) reads as counter-performative).

- Ano presente na atribuição; `MacKenzie (2006)` está no Appendix C (:423) e já era citado no capítulo
  (37:205). F3 fechado.
- "amplifying the decline" saiu. Nenhuma causalidade contestada permanece afirmada: o que ficou é a
  descrição do mecanismo ("rules that sell as the market falls") e a leitura que MacKenzie faz do caso,
  atribuída a ele.

## 4. F4 — Chao (book-src/67-layer-epidemiology-outbreak-surveillance-order.md:94-95)

Texto agora:

> the estimator Chao (1987) proposed is used here as a lower bound on the uncounted part rather than as a
> point estimate of it.

- O estatuto de limite inferior passou a ser declarado como uso deste livro ("used here"), não como tese
  do artigo de 1987 — que propõe um estimador pontual, como t11 verificou. A citação continua cobrindo só
  o estimador. F4 fechado.
- A leitura continua apoiada pela álgebra do próprio parágrafo (m inflado por dependência positiva enviesa
  N̂ para baixo), que é o que o texto exibe; nada é afirmado sobre a fonte que ela não diga.

## 5. Portões e ledger

| Verificação | Resultado |
| :--- | :--- |
| `python3 .github/checks/structure.py` | exit 0, "all structural gates pass"; census 521 (advisory); órfãos do Appendix C 9; drift 3; **declared open 0** |
| `python3 .github/checks/wasm_gates.py` | exit 0, 18 gates, 10 decididos, 0 falhos, 8 abertos (nenhum marcado pass) |
| Diff de `structure.py` contra HEAD | 16 linhas removidas, as mesmas das rodadas anteriores (docstring, `check_links` por âncora, `check_ledger` com `has_entry(surname, year)`) — nenhuma remoção desta rodada, nenhum portão afrouxado |
| Ledger | `audit/repair-report.md:276-291` traz "Rodada 5 (t14)" com tabela arquivo → o que mudou → achado de origem → portão, as duas entradas bibliográficas e a nota de censo explicando as +3 linhas |
| `ledger-coherence` | verde: os dois caminhos novos citados existem e os tokens de portão (`year-citation`, `citations`, `leitura`) são nomes que o arquivo imprime ou uma leitura declarada |

O censo subir de 518 para 521 é consequência de redação, não de valor novo: "45" passou a ficar numa linha
sem ano de citação (a entrada é `n.d.`), e o rewrap separou "16 December 1976" e "fiscal year 1970" dos
pares nome+ano. Advisory nas duas parcelas, como o ledger declara.

---

## Achados desta revisão

### L1 — low — `book-src/36-decision-loss-threshold.md:315-316` (e `appendix-c-sources.md:211`)

`(Centers for Disease Control and Prevention n.d.)` cita como sem data uma página que o servidor entrega
datada: a versão que abri traz "December 20, 2024" no topo. `n.d.` é justamente a única forma de citação
que não carrega ano, e portanto a única que não permite ligar o valor a uma versão datada da fonte — o
oposto do que §IV pede ("with the value exactly as the source states it, at the source's own precision").
Não é blocker: o número e a formulação estão certos e a fonte sustenta o que a frase diz.

**Correção proposta**: citar a data que a página imprime — `(Centers for Disease Control and Prevention
2024)` e o ano na entrada —, ou, se a data de revisão não for citável como data de publicação, escrever na
própria entrada por que a página é tratada como não datada ("accessed 2026; page undated" ou equivalente).
Qualquer das duas fecha o ponto sem inventar nada.

### L2 — low — `book-src/28-the-machine-and-the-deception.md:229-231`

A oração ficou longa e a coordenação ficou pesada: "the Mansfield Amendment, §203 of the military
authorization act for fiscal year 1970 (Public Law 91-121), enacted in 1969, had already required …". O
conteúdo está correto e a garantia está no lugar; o que sobra é legibilidade, não doutrina.

**Correção proposta**: quebrar em duas orações — "the Mansfield Amendment had already done so: §203 of the
military authorization act for fiscal year 1970 (Public Law 91-121), enacted in 1969, required American
defence research funding to show a direct and apparent relationship to a specific military function or
operation." Nada factual muda.

---

## O que esta revisão não fez

- Não reconferi as páginas 204/206 do Statutes at Large (PDF não suportado pelo leitor desta sessão);
  conferi os identificadores contra t11 e a coerência interna da entrada.
- Não auditei os demais capítulos tocados por rodadas anteriores; o escopo é a rodada 5.
- Não julguei mérito doutrinário das frases reparadas além do que §IV e §I exigem: as quatro correções são
  de atribuição e de formulação, e nenhuma delas altera proposição, álgebra ou veredito triádico.
