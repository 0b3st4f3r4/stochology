# Relatório de reparo — t5

Data: 2026-11-13. Escopo: `book-src/` (reparo) e `.github/checks/structure.py` (dívida de declaração).
Nada commitado. Nenhum número substituto foi inventado; onde o token caiu, o argumento ficou.

Método de localização: cada trecho foi achado por texto verbatim (substring única verificada por contagem)
e substituído por parágrafo ou por frase, nunca por número de linha. Onde o texto não bateu exatamente, o
item parou e está registrado em §5.

Sedes consultadas antes de tocar doutrina (regra do capitão, m00054): `book-src/41-the-triad-and-the-minimum.md`
(P1/P2/P3, §8 e §9.5), `book-src/appendix-a-mathematics.md:162-217`, `book-src/20-legitimacy-and-institutions.md:195-215`,
`book-src/85-layer-polycentric-commons-urban-biospheric-order.md:45-62`; a sede `40-institution-incentive-metric.md:172` foi consultada mas não alterada.

---

## 1. Citações (t1 — `audit/audit-citations.md`)

| Item | Arquivo | O que mudou | Portão |
| :--- | :--- | :--- | :--- |
| #6 / #26 — a lei composta não é de Meng | `book-src/06-who-is-missing-from-the-table.md` | "Meng proved the Big Data Paradox (Meng 2018). Whenever … obeys the harmonic precision law:" virou: Meng prova o paradoxo (deslocamento da média amostral em proporção a ρ_R,Y); "Meng's identity prices that displacement and nothing else. Add Kish's design-effect term … the two compose into the harmonic precision law derived in [Chapter 34](34-sample-population-archive.md)". DEFF deixa de ser leitura de Meng; a composição passa a ser a derivação do cap. 34. Eq. exibida intocada. | Nenhum portão mecânico lê atribuição; a string canônica `the exact composition is additive` (concordância §V-9) segue presente, e a contagem de censo não muda |
| #20 — "+$1,794.34" é a subamostra Dehejia–Wahba | `book-src/59-layer-experimental-sampling-causal-order.md` | "On LaLonde's National Supported Work benchmark" → "On the Dehejia–Wahba subset of LaLonde's National Supported Work benchmark (185 treated, 260 control)". Valores mantidos na precisão da fonte (−$8,497.52 e +$1,794.34 são exatos no centavo: 1.794,3424 e −8.497,5161). Nenhuma substituição por +$886.3. | Censo (advisory, §V-7); nenhum portão lê o rótulo da amostra nem a precisão do valor — leitura |
| #16 — "88% false-alarm rate" | `book-src/05-what-a-number-doesnt-say.md` | Passou a citar o valor da fonte: "Wong et al. (2021) report that at the deployed threshold the alert carried a positive predictive value of $12\%$." O 88% fica como aritmética marcada: "The complementary false-alarm share, $1 - \mathrm{PPV} = 0.88$, is arithmetic on that reported value and not a separate figure the study states." | Censo (advisory, §V-7); `math` confere paridade de `$`, não a aritmética — marcar 1−PPV como aritmética é leitura |
| #13 — Chao (1987) | `book-src/67-layer-epidemiology-outbreak-surveillance-order.md` | "Chao's (1987) estimator is stated as a lower bound and not as an estimate" → "the Chao estimator is read as a lower bound on the uncounted part rather than as a point estimate of it (Chao 1987; Chao & Jost 2012)." Ambas as entradas existem em `book-src/appendix-c-sources.md:212-213`. | `citations` (Chao 1987 e Chao & Jost 2012 resolvem em Appendix C); censo (advisory); o que o paper afirma é leitura |
| #25 — eponymo "Campbell–Moulton" | `book-src/00-preface.md:294`, `book-src/27-what-judging-is.md:169`, `book-src/43-inquiry-openness-multiverse.md:252`, `book-src/45-world-to-archive-big-data-paradox.md:200`, `book-src/45-world-to-archive-big-data-paradox.md:202`, `book-src/50-methodological-families-reflexivity.md:167`, `book-src/appendix-a-mathematics.md:162` | Padronizado em **Kish–Moulton** nos seis usos e no título de seção 45:200. A ressalva subiu para o primeiro uso (`book-src/00-preface.md:294`): "the Kish–Moulton design effect (this book's shorthand for Kish's weight-concentration factor composed with Moulton's cluster correction, not an eponym the survey literature uses)". `book-src/45-world-to-archive-big-data-paradox.md:202` mantém a admissão de shorthand. `book-src/59-layer-experimental-sampling-causal-order.md:43` ("Moulton's design effect") e o ledger (`Moulton`) ficam: nomeiam só o fator de cluster, que é de Moulton. Nenhum capítulo nem `SUMMARY.md` renomeado. | Nenhum portão lê eponymo (o `attribution` só dispara com 2 entradas de Appendix C para o par) — leitura; paridade de ledger §V-6 segue satisfeita |

Nenhuma citação foi perdida nas reescritas: 06 conserva `Meng 2018` e ganha a remissão ao 34; 59 conserva
`Dehejia & Wahba 1999`; 05 conserva `Wong et al. 2021`; 67 conserva `Chao 1987` e acrescenta `Chao & Jost 2012`;
20 conserva `Holmström 1979`; 85:58 ganha a remissão ao 41 no lugar de uma ordem de publicação.

Achados INVERIFICÁVEIS de t1 (Meng AOAS pago, Moulton 1986, Chao 1987, decimais do ProPublica, Neustadt–Fineberg,
Dreyfus/Lighthill/Mansfield) e a camada filológica não têm reparo de texto: ficam registrados como limites de
verificação, não como defeito.

## 2. Álgebra (t2 — `audit/audit-math.md`)

452 blocos, veredito CORRETO em todos: nenhuma correção algébrica era devida. Dois itens não-algébricos:

| Item | Arquivo | O que mudou | Portão |
| :--- | :--- | :--- | :--- |
| frase vizinha a `k(n_eff)` prometia magnitude empírica e ordenação entre mecanismos | `book-src/47-irreversible-time-vital-margin.md:165` | "A raw $T$ of tens of thousands of days … pulls the reckless plug-in leverage back before the heavy-tail regime selector takes over." → "The factor $k(n\_{\mathrm{eff}}^{\mathrm{var}}) = 1/(1 + 1/n\_{\mathrm{eff}}^{\mathrm{var}})$ cuts the leveraged fraction by $1/(n\_{\mathrm{eff}}^{\mathrm{var}} + 1)$, so the position it leaves is the plug-in position scaled by $k$ and the cut falls in inverse proportion to the effective sample size." Nenhum número, mesma tese: a alavancagem que maximiza crescimento na população não é a que uma amostra defende. | `math` (paridade `$`, `_`/`^` só dentro de math) e censo: o parágrafo não ganhou numeral |
| `k_BM` atribuído a Baker & McHale (2013) | `book-src/47-irreversible-time-vital-margin.md:164` | **Não reparado.** `k_BM` impresso na mesma linha é objeto distinto de `k` (depende de $z^2$ e da curtose; coincide com $k$ só se $z^2 = n\_{\mathrm{eff}}$), e a verificação de que Baker & McHale imprimem essa forma exige o paper, que não está acessível aqui. Registrado como aberto, sem tocar o texto. | — |

## 3. Epistemologia (t3 — `audit/audit-epistemics.md`)

| Item | Arquivo | O que mudou | Portão |
| :--- | :--- | :--- | :--- |
| A1 — accountability como produto compensatório | `book-src/20-legitimacy-and-institutions.md:198` | Declarada a convenção na mesma frase, sem tocar a exibição: "…requires three non-substitutable structural components, and on the chart where each is present or absent their conjunction is their minimum — the product below writes that conjunction, and is not a graded aggregate that lets one component pay for another". O produto em (0,1)³ deixa de ser leitura graduada; coincide com o mínimo onde cada componente é presença/ausência, como `book-src/41-the-triad-and-the-minimum.md:339` e `appendix-a:217`. | `concordance` e `concordance-polarity` (`The minimum is not a weighted sum` intacta); a convenção declarada na frase é leitura |
| A2 / B2 — "the price of one weak axis" e ordem de publicar | `book-src/85-layer-polycentric-commons-urban-biospheric-order.md:58` | "…the gap between the two is the price of one weak axis — the number a commons institution should publish…" → "…measures how far the weakest axis sits below the level an aggregate would report. An institution that must publish a number publishes the smallest unhedged exposure measured in the units of its own ruin barrier, and labels it a threshold device rather than a score ([The Triad and the Minimum](41-the-triad-and-the-minimum.md))." A licença de publicação passa a ser a de `book-src/41-the-triad-and-the-minimum.md:359-361`. Fica a frase seguinte (o alargamento do gap como razão formal de um commons não compensar centro falho). | Nenhum portão; sede `book-src/41-the-triad-and-the-minimum.md:359-361` — leitura |
| B1 — "sovereign authority to halt operations" (rebaixado a MENOR) | `book-src/41-the-triad-and-the-minimum.md:256` | Sujeito explicitado, sem mexer na doutrina: "It is read by independent auditors, and the authority to halt operations sits with those auditors and not with the verdict they read." Compatível com `book-src/41-the-triad-and-the-minimum.md:266-270` ("It trips nothing") e com `book-src/20-legitimacy-and-institutions.md:202`, onde o poder de parar legitimamente está. | — |
| B3 — "use the critical axis … as the single lever" | `book-src/41-the-triad-and-the-minimum.md:343` | "…to name the critical axis (the smallest one) as the axis the next step must fund…" O diagnóstico nomeia a eixo e para; não converte o veredito em lei de controle (§I). | — |
| A4 — "are aggregated by" | `book-src/27-what-judging-is.md:186` | "are folded by their non-compensatory Leontief minimum". | `concordance` e `concordance-polarity` (string canônica); `notation` (grafia dos símbolos na mesma frase) |
| C1 — "completely overwhelmed on the first day" | `book-src/41-the-triad-and-the-minimum.md:199-203` | Absorvido na reescrita de P1: "…fails on the first excursion beyond the support that certainty assumes." Sem magnitude empírica. | `collapse-refutation` |
| D1 — P1 sem bounds de identificação parcial e sem cláusula de refutação | idem | P1 abre em forma condicional ("Where an organization holds … but lacks the epistemic margin"), inclui "the width of the partial-identification bounds $\Delta\_{\mathrm{id}}$" ao lado de $\rho\_{R,Y}$ e $\mathrm{DEFF}$, e fecha com "It fails if a trajectory carrying all three omissions is observed to size its buffer above the tail it later meets." | `collapse-refutation` (G-B) — verde |
| D2 — P3 sem condição de refutação | `book-src/41-the-triad-and-the-minimum.md:216` | P3 abre em "Where an organization has accurate data … but lacks the governance coordinate" e fecha com "It fails if an institution with one-sided limited liability and silenced frontline testimony is observed to hold its reserves through a shock that removes the epistemic margin." P2 ganhou a cláusula correspondente: "It fails if a trajectory whose survival time sits below its recovery time is observed to take the next step at full utilization and still be there once the recovery interval has elapsed." | `collapse-refutation` (G-B) — verde nos três |
| D3 — desautorização em bloco | `book-src/41-the-triad-and-the-minimum.md:222-225` | Não editado, como prescrito: com P1 e P3 condicionais, a frase "none of them claims anything about institutions in general" deixa de contradizer o enunciado, e `book-src/41-the-triad-and-the-minimum.md:229` já é condicional ("When two safeguards are absent…"). | — |
| D4 — reenunciação fraca em 50:389 | `book-src/50-methodological-families-reflexivity.md:389` | Cada célula passa a nomear o mecanismo herdado: blind undersizing "blind in the sense Collapse Proposition 1 fixes: the reserve is sized from naive sample means with $\rho\_{R,Y} \neq 0$ and $\mathrm{DEFF} > 1$ unmeasured and no partial-identification width counted"; impotent lucidity "at a survival time below the recovery time ($\mathrm{TTS}\_{\mathrm{survival}} < \mathrm{TTR}\_{\mathrm{recovery}}$)". | `falsifiers` (§9.5.3) segue verde; `notation` |
| E1 — "three transits" fundando sete sites | `book-src/appendix-e-lexicon.md:66` | "arranged over the six ordered crossings of distinct planes and the integral closure that together make the seven transits of the mediation circuit." Tabela E.1 intocada. | `guard-footing` (G-C) — verde |
| E2 — Appendix A citando capítulo como autoridade do benchmark | `book-src/appendix-a-mathematics.md:294` | "in the homogeneous local benchmark of equal loads and equal capacities that [Chapter 53](53-layer-macro-coupled-topology-order.md) instantiates" — o benchmark passa a ser declarado no próprio apêndice e o link vira remissão (§II). | — (candidato a portão, §6) |
| E3 — segunda contagem em 85:52 | `book-src/85-layer-polycentric-commons-urban-biospheric-order.md:52` | Removido "across seven strata of five orders": "…exhausts every three-site combination of the seven guard sites, a thirty-sixth layer would break the bijection between the two graphs." | `guard-sites` (§V-10) segue verde |
| E4 / C2 — "correspond 1-to-1" com três sites | `book-src/33-measuring-and-erring.md:219` | "These three psychometric threats are answered at the framing (`T1`), archive (`T3`) and performative (`T6`) guard sites of the grammar of aim." Sem bijeção afirmada. | — |

## 4. Portões (t4 — `audit/audit-gates.md`)

Quatro sites viviam em `DECLARED_OPEN` (`.github/checks/structure.py:518`): as três proposições do cap. 41 e a
frase de Appendix E. O texto foi reparado, então a dívida foi zerada: `DECLARED_OPEN` passa a `{}` com o
comentário de que a declaração é dívida contra o manuscrito, não isenção permanente. Nenhum portão foi afrouxado.

Prova por mutação (plantada, executada, revertida):

- M1, reintroduzir "arranged over the three transits of the mediation circuit" em `appendix-e`: `structure.py`
  exit 1 → `[guard-footing] book-src/appendix-e-lexicon.md: the circuit is credited with 'three transits'`.
- M2, apagar a cláusula `It fails if a trajectory carrying all three omissions …` de P1: exit 1 →
  `[collapse-refutation] book-src/41-the-triad-and-the-minimum.md:199: Collapse Proposition 1 carries no sentence saying what would refute it`.

Restaurados os dois arquivos byte a byte; verde confirmado depois (exit 0).

## 5. Rebaixamentos, recusas e divergências

1. **`book-src/85-layer-polycentric-commons-urban-biospheric-order.md:54` não foi tocado** (t3 A3, falso positivo confirmado pelo capitão, m00054). Sedes verificadas:
   `40-institution-incentive-metric.md:172` ("governance coordinate is not one reading among three: it fixes the sign of the differential on
   which the other two are priced"), `book-src/41-the-triad-and-the-minimum.md:339` e `appendix-a:217` ($S\_{\mathrm{gov}}$ é argumento de `min`).
   O que a frase nega é que a coordenada seja *meramente* uma terceira leitura, não que entre no mínimo. Portão
   `concordance-polarity` mantido: ele checa a polaridade da string canônica, não a frase do 85.
2. **`book-src/41-the-triad-and-the-minimum.md:256-257` rebaixado de GRAVE a MENOR** (t3 B1). Sedes: `book-src/41-the-triad-and-the-minimum.md:266-270` ("It trips nothing") e `book-src/20-legitimacy-and-institutions.md:202`
   (poder de parar). Editado só o sujeito, como o capitão autorizou.
3. **`book-src/85-layer-polycentric-commons-urban-biospheric-order.md:58` é defeito real mas estreito** (t3 A2/B2): o argumento de que o gap alarga com ordens sãs acrescentadas
   fica; cai o rótulo "price of one weak axis" e a ordem incondicional de publicar; o escalar, quando nomeado,
   é o de `book-src/41-the-triad-and-the-minimum.md:359-361`.
4. **`k_BM` / Baker & McHale (2013)**: recusado com razão em §2 (sem acesso ao paper; `k_BM` é objeto distinto de `k`).
5. **Nenhuma divergência de texto verbatim** nos itens do ledger: todas as âncoras bateram uma única vez.
   A única asserção que falhou foi a quebra de linha de P2 (frase em duas linhas), corrigida por âncora real
   antes de qualquer escrita — não houve edição parcial.

## 6. Verificação

```
$ python3 .github/checks/structure.py
… 523 census advisory, orphans Appendix C 10, display-first symbols 25, near-dups 0, cross-refs 15,
  notation drift 3 (advisory; o item de 22 foi corrigido em t7), declared open 0
all structural gates pass                                   # exit 0

$ python3 .github/checks/wasm_gates.py
18 gates: 10 decided here, 0 failed, 8 declared open (none of them marked pass)
                                                            # exit 0

$ git diff --stat
37 files changed, 914 insertions(+), 117 deletions(-)       # inclui a onda anterior + t4 + t7
```

`git diff --stat` acima é o diff acumulado contra `9fc4e41` (t4 já estava na árvore). Os arquivos de
`book-src/` tocados pelo reparo t5 são 16: `00-preface`, `05`, `06`, `20`, `27`, `33`, `41`, `43`, `45`,
`47`, `50`, `59`, `67`, `85`, `appendix-a-mathematics`, `appendix-e-lexicon`; mais
`.github/checks/structure.py` (DECLARED_OPEN) e este relatório. Outros vinte arquivos já vinham alterados
antes deste time, pela auditoria de fantasmas de 00–50: a seção seguinte dá a cada um o defeito de origem, o
que mudou e o portão que o cobre. Com eles, a árvore alterada contra `9fc4e41` fecha nos 36 arquivos que o
revisor contou.

### Portões que cobrem o que foi reparado

- `collapse-refutation` cobre P1/P2/P3 do cap. 41 (verde, e falha por mutação).
- `guard-footing` cobre `appendix-e:66` (verde, e falha por mutação).
- `math` cobre a reescrita de `book-src/47-irreversible-time-vital-margin.md:165` (paridade de `$` por parágrafo, `_`/`^` só em math).
- `notation` cobre `book-src/50-methodological-families-reflexivity.md:389`, `41:199/216`, `book-src/85-layer-polycentric-commons-urban-biospheric-order.md:58` ($\mathrm{DEFF}$, $\mathrm{TTS}$, $\mathrm{TTR}$, $\rho\_{R,Y}$).
- `falsifiers` garante que §9.5.3 do cap. 50 não foi deslocado pelas edições de 50:389.
- Concordância §V-9: as quatro strings canônicas seguem presentes e nenhuma variante contraditória foi
  introduzida.

### Lacunas de portão (candidatas, não implementadas aqui)

Nenhum portão mecânico lê (a) atribuição de prosa como a de `06`/`59`/`05`/`67`, (b) a regra §II de que
apêndice não cita capítulo como autoridade (o caso `appendix-a:294`), (c) padronização de eponymo. As três
exigem leitura de frase, não de token; ficam como proposta para a trilha de portões.

---

## Onda anterior (auditoria de fantasmas 00–50)

Vinte arquivos de `book-src/` chegaram a esta rodada já alterados por uma onda anterior a este time, cujos
achados não estão em `audit/audit-citations.md`, `audit/audit-math.md` nem `audit/audit-epistemics.md`. O revisor
(t6, F3) pediu proveniência: cada arquivo abaixo recebe o defeito de origem, o que mudou e o portão que
cobre o resultado. A regra aplicada foi sempre a mesma — apaga-se o token, mantém-se o argumento, nenhum
número substituto inventado (§IV).

| Arquivo | O que mudou | Achado de origem | Portão / cobertura real |
| :--- | :--- | :--- | :--- |
| `book-src/03-the-biggest-number-in-the-room.md` | "the estimate lands below 2" virou condicional ("where the estimate lands below 2"), tirando a promessa de uma medição que o arquivo apagado não sustenta; a consequência categórica da cauda $\alpha \le 2$ ficou | fantasma 00–50: promessa de medição de arquivo inexistente | `math` (paridade e escapes); a leitura da consequência é leitura |
| `book-src/07-when-the-measure-becomes-the-goal.md` | "queueing multiplier an order of magnitude larger" virou a curva exibida $\rho/(1-\rho)$ e sua divergência | magnitude em palavra sem garantia | `math` (paridade); a curva exibida é leitura |
| `book-src/12-derivatives-and-convexity.md` | "jump by more than an order of magnitude" virou "far out of proportion"; "orders of magnitude and not by a factor" virou a taxa derivada (10× e 1000× de $\phi \approx (1-\rho)^{-1}$, $\phi'' = 2(1-\rho)^{-3}$) | magnitude em palavra sem garantia | `math` (paridade); as taxas derivadas são leitura |
| `book-src/14-linear-algebra.md` | cenário sem fonte ("household interviews of a national population survey") devolvido ao par citado: LaLonde (1986) e Dehejia & Wahba (1999), com Kish (1965) para $n/(1+(\bar m-1)\rho)$ (reparo F4 desta rodada) | citação perdida em parágrafo reescrito | `citations` (as três entradas resolvem em Appendix C); a atribuição do cenário é leitura |
| `book-src/16-what-knowledge-is.md` | "auditing costs many times the harm it prevents" virou "costs more than the harm it prevents" | magnitude em palavra sem garantia | censo (advisory); leitura |
| `book-src/18-philosophy-of-science.md` | "swings by an order of magnitude more" virou "changes slope from one regime to the next" | magnitude em palavra sem garantia | censo (advisory); leitura |
| `book-src/19-ethics-and-the-imposition-of-risk.md` | a razão de inversão de culpa passou a ser definida, $b\_{\mathrm{fl}}/v\_{\mathrm{op}}$, e a mostrar que cresce sem limite; as taxas numéricas do extrato de custódia foram substituídas pela identidade do cap. 49 | números de arquivo removido | `citations` (Chouldechova 2017), `math` (paridade); a razão exibida é leitura |
| `book-src/22-thermodynamics-and-entropy.md` | "many orders of magnitude above" virou o teto de Landauer exibido, $P\,T_{\mathrm{year}}/(k_B T\_0 \ln 2)$ (Landauer 1961); nesta rodada, F2: um "and" duplicado, `$T_0$` unificado em `$T\_0$`, e "never" trocado por "not the binding term at any institutional power draw" | magnitude em palavra sem garantia | `math` (paridade), `notation`; drift advisory caiu de 4 para 3 |
| `book-src/23-statistical-physics-and-time.md` | "optimistic by tens of percent" virou a divisão pelo fator de desenho, exata | magnitude em palavra sem garantia | `math` (paridade); a divisão exibida é leitura |
| `book-src/26-regulation-and-homeostasis.md` | a data de 1878 sem citação recebeu (Bernard 1878) (reparo F7 desta rodada) | data sem fonte | `citations` (a entrada Bernard 1878 resolve); exigir a citação é leitura |
| `book-src/28-the-machine-and-the-deception.md` | a anedota de Wald recebeu (Wald 1943); nesta rodada, F7: a Mansfield Amendment passou a nomear o ato que a carregou (military authorization act, FY1970, enacted 1969) | data e atribuição sem fonte | `citations` (Wald 1943 resolve); o ato é nomeado em prosa — leitura |
| `36-decision-loss-threshold.md` (sem prefixo `book-src/`: idêntico ao HEAD) | "more than forty million" (reescala da onda anterior) foi devolvido à forma pré-reparo, "roughly forty-five million" (reparo F6 desta rodada); Neustadt & Fineberg (1983) segue como fonte na mesma frase | valor de literatura reescalado | leitura; censo (advisory) — `citations` só confere que (Neustadt & Fineberg 1983) resolve em Appendix C, não a precisão do valor |
| `book-src/37-model-fit-and-error.md` | "shift $B^2$ far more than sampling noise shifts $V$" virou a afirmação que Steegen et al. (2016) sustentam: escolhas defensáveis de processamento movem um resultado publicado pela linha de significância | comparação de magnitude sem garantia | `citations` (Steegen et al. 2016 resolve); leitura |
| `book-src/38-time-and-dependence.md` | a anedota de 1943 virou os memoranda de Wald no Statistical Research Group (Wald 1943) | data e atribuição sem fonte | `citations` (Wald 1943 resolve); leitura |
| `book-src/42-sequential-loop-naive-agent.md` | "cost per served shift differs by orders of magnitude" virou a razão simbólica $(H_A/H_B)(s_B/s_A)$; a fila entrou como $W(\rho)=\rho/(1-\rho)$ com $W(0.5)=1$, $W(0.9)=9$, $W(0.95)=19$ | magnitude em palavra sem garantia | `math` (paridade); $W(\rho)$ e os três valores são leitura |
| `book-src/44-underdetermination-partial-identification.md` | os números do benchmark de LaLonde saíram; nesta rodada (t9) caiu também "and larger magnitude", e fica só o sinal oposto — a comparação engenhosa e a experimental discordam; "more width than any premise buys back" (`book-src/44-underdetermination-partial-identification.md:196`) fica, porque compara largura entre dois instrumentos, não magnitude de estimativa | números de arquivo removido; magnitude em palavra | censo (advisory); leitura |
| `book-src/46-non-identical-asymmetry-heavy-tails.md` | "grow by orders of magnitude" virou "can grow without bound"; a seção "The Same Estimator on Insurance and Flight Archives" virou "on Two Kinds of Archive"; promessas de resultados do arquivo de voos removidas | promessa de medição de arquivo inexistente | `spine` (título de seção), censo (advisory) |
| `book-src/48-reactive-worlds-performativity-goodhart.md` | "an order of magnitude larger" removido; nesta rodada, F5: a monotonia sem álgebra ("grows with the distance") virou dependência explícita com Perdomo et al. (2020) na mesma frase | magnitude em palavra; monotonia sem garantia | `citations` (Perdomo et al. 2020 resolve); a dependência é leitura |
| `book-src/49-moral-luck-apt-agency-institutions.md` | "exceeds the survival window by an order of magnitude" virou "by a margin that grows without bound as utilization approaches one"; o gate foi reespecificado contra um arquivo de custódia; a disparidade de falsos positivos passou a citar a identidade de Chouldechova (§8.3.2) | promessa de medição de arquivo inexistente | `citations` (Chouldechova 2017), `math` (paridade); a margem sem limite é leitura |
| `book-src/appendix-c-sources.md` | entrada **Bernard, C. (1878)** acrescentada para a citação de `26`, e **Mangel & Samaniego (1984)** verificada como fonte do trabalho de Wald | citação sem entrada na bibliografia | `citations`, `ledger` |

Nenhum desses vinte arquivos recebeu número novo: onde havia magnitude em palavra, ficou a álgebra exibida
ou uma afirmação mais fraca e sustentada; onde havia promessa de medição de um arquivo que não existe, ficou
o argumento sem a promessa. As duas datas (F7) foram resolvidas por citação, não por remoção, porque as
fontes existem: Bernard 1878 para as lições, e o ato de autorização militar do ano fiscal de 1970 para a
Mansfield Amendment.

---

## Rodada 2 (t7): os oito achados do t6

O revisor de t6 devolveu `needs_revision` com oito achados. Todos foram fechados no texto ou no portão, sem
afrouxar portão nenhum e sem commit.

| Achado | Arquivo | O que mudou | Portão / cobertura real |
| :--- | :--- | :--- | :--- |
| F1 (GRAVE) | `book-src/41-the-triad-and-the-minimum.md` | "to name the critical axis (the smallest one) as the axis the next step **must fund**" virou "to name the critical axis (the smallest one) as the one the next step will face, leaving the funding decision to the operator's own per-step state, and to abandon the composite score": descreve o eixo que a etapa enfrenta, não prescreve o financiamento — §I, último parágrafo | `grep -c "must fund"` = 0; `collapse-refutation` segue verde; a conformidade com §I é leitura |
| F2 (MÉDIO) | `book-src/22-thermodynamics-and-entropy.md` | "and\nand" → um "and"; `$T_0$` → `$T\_0$` e `T_{\mathrm{year}}` → `T\_{\mathrm{year}}`; "erasure is never the term an institution runs into" → "erasure is not the binding term at any institutional power draw" | `math` (paridade `$`), `notation`; drift advisory caiu de 4 para 3 |
| F3 (MÉDIO) | `audit/repair-report.md` | nova seção "Onda anterior (auditoria de fantasmas 00–50)" com os vinte arquivos: o que mudou, achado de origem e portão; a contagem de 36 arquivos da árvore passa a fechar (16 de t5 + 20 da onda anterior) | leitura; `git diff --stat` |
| F4 (MÉDIO) | `book-src/14-linear-algebra.md` | o cenário sem fonte virou o par citado — "the comparison LaLonde (1986) posed and Dehejia & Wahba (1999) re-estimated against its experimental benchmark" — e $n/(1+(\bar{m}-1)\rho)$ recebeu (Kish 1965); a derivação exibida ficou | `citations` (LaLonde 1986, Dehejia & Wahba 1999, Kish 1965 resolvem em Appendix C); a atribuição do cenário é leitura |
| F5 (BAIXO) | `book-src/48-reactive-worlds-performativity-goodhart.md` | as duas monotonias sem álgebra ("grows with the distance") viraram dependência explícita, com Perdomo et al. (2020) na mesma frase; nenhum número novo | `citations` (Perdomo et al. 2020 resolve); a dependência é leitura |
| F6 (BAIXO) | `36-decision-loss-threshold.md` (sem prefixo `book-src/`: idêntico ao HEAD) | "more than forty million" (reescala da onda anterior) devolvido à forma pré-reparo, "roughly forty-five million", com Neustadt & Fineberg (1983) na mesma frase; item registrado no ledger da onda anterior | `git diff` do arquivo volta a ser vazio; nenhum portão lê precisão de valor de literatura — leitura |
| F7 (BAIXO) | `book-src/26-regulation-and-homeostasis.md`, `book-src/28-the-machine-and-the-deception.md`, `book-src/appendix-c-sources.md` | 26 recebeu (Bernard 1878), com entrada nova no Appendix C; 28 passou a nomear o ato que carregou a Mansfield Amendment (military authorization act, FY1970, enacted 1969) | `citations` (Bernard 1878 resolve); o ato é nomeado em prosa — leitura |
| F8 (BAIXO) | `.github/checks/structure.py` | o comentário de `check_collapse_refutation` declarava uma varredura de seção que o código não faz; passou a declarar o limite: só o parágrafo que abre na forma bold é lido | leitura do comentário; `structure.py` exit 0 |

Verificação desta rodada: `structure.py` exit 0 ("all structural gates pass", census 523, órfãos 10,
display-first 25, near-dups 0, cross-refs 15, notation drift 3, declared open 0), `wasm_gates.py` exit 0
(18 gates, 10 decididos, 0 falhos, 8 declarados abertos), `git diff --stat` 37 arquivos, 914 inserções,
117 remoções contra `9fc4e41`.

---

## Rodada 3 (t9): crédito de portão, caminhos do ledger e magnitude em 44

1. **Crédito de portão conferido linha a linha** contra o código de `.github/checks/structure.py`. O que cada
   portão faz de fato: `citations` resolve (sobrenome, ano) contra o Appendix C e nada mais — não lê precisão,
   não exige que a frase traga o ano, não compara valor; `math` confere paridade de `$`/`$$` e que `_`/`^` não
   vivam fora de math, não a álgebra; `notation` confere a grafia dos símbolos, não o argumento; `concordance` e
   `concordance-polarity` leem as quatro strings canônicas de §V-9; `collapse-refutation`, `guard-footing`,
   `falsifiers`, `guard-sites`, `ledger` e `attribution` têm o escopo que o código declara. Onde a garantia é
   julgamento humano a coluna passa a dizer **leitura**. Corrigidas as linhas que creditavam a `citations` ou a
   `attribution` uma checagem de precisão da fonte ou de atribuição que nenhum dos dois faz: `#20` (59),
   `#16` (05), `#13` (67), `#25` (eponymo), e na onda anterior as de 26, 28, 36, 37, 48, 49 e `appendix-c`, mais
   as da rodada 2 (F4, F5, F6, F7).

2. **Caminhos normalizados**: `book-src/NN-nome.md` nas vinte linhas da onda anterior e nas oito da rodada 2;
   `book-src/NN-nome.md:linha` onde havia `capítulo:linha` (`00`, `27`, `43`, `45`, `50`, `appendix-a`).
   Duas exceções declaradas, ambas fora do grep de cobertura de propósito: `36-decision-loss-threshold.md` foi
   devolvido ao HEAD e por isso aparece sem o prefixo `book-src/`; `40-institution-incentive-metric.md:172` é
   sede consultada, não arquivo alterado. Conferência executada:

   ```
   $ grep -o 'book-src/[0-9a-z-]*\.md' audit/repair-report.md | sed 's|book-src/||' | sort -u | wc -l
   35
   $ git status --short | awk '{print $2}' | grep '^book-src/' | sed 's|book-src/||' | sort | wc -l
   35
   ```

   Os dois conjuntos são idênticos — nenhum arquivo a mais, nenhum a menos (verificado por `diff` das listas,
   saída vazia).

3. **`book-src/44-underdetermination-partial-identification.md:192`**: "gives an estimate of the opposite sign
   and larger magnitude" → "gives an estimate of the opposite sign". Fica o sinal oposto, que é afirmação
   qualitativa, e o fato de que a comparação engenhosa e a experimental discordam; cai a comparação de tamanho,
   que era magnitude em palavra sem álgebra ao lado, e o próprio `:196` diz que os números ficam nos papers
   citados. Nenhum número entrou e nenhuma citação saiu (LaLonde 1986 e Dehejia & Wahba 1999 seguem na frase).

**Ressalva recusada (decisão do capitão):** `book-src/41-the-triad-and-the-minimum.md:227` **não** foi tocado.
A revisão de t8 chamou-o de parágrafo órfão; ele é o parágrafo de escopo que limita as três proposições a
trajetórias e remete a frequência institucional ao §9.6 do cap. 50. Recusado, com o texto como estava.

Verificação desta rodada: `structure.py` exit 0 ("all structural gates pass", census 523, órfãos 10,
display-first 25, near-dups 0, cross-refs 15, notation drift 3, declared open 0); `git diff --stat` 37 arquivos,
914 inserções, 117 remoções contra `9fc4e41`. Sem commit.

---

## Rodada 4 (t12): anos sem citação, atribuição de Chao, dívida declarada paga

Os quatro sites do portão `year-citation` (t10) e o único defeito aberto da auditoria de atribuição (t11, A3)
foram fechados no texto. Nenhuma entrada nova foi necessária no Appendix C: as quatro fontes já lá estavam
(Rumelhart, Hinton & Williams 1986; Minsky & Papert 1969; Neustadt & Fineberg 1983; Chao 1987), o que é a
razão de a reparação ter sido citação e não remoção do ano.

| Arquivo | O que mudou | Achado de origem | Portão |
| :--- | :--- | :--- | :--- |
| `book-src/14-linear-algebra.md` | "the geometric content of the 1986 recovery narrated in [Chapter 28] §5" → "the geometric content of the 1986 recovery (Rumelhart, Hinton & Williams 1986) narrated in [Chapter 28] §5". O ano nomeia a retropropagação, que o mesmo capítulo já citava no parágrafo da forma normal algébrica; a fonte passa a estar no ponto do ano, não a trinta linhas de distância | t10, site 1 (ano 1986) | `year-citation` |
| `book-src/28-the-machine-and-the-deception.md` | "The consequence reaches far beyond 1969:" → "The consequence reaches far beyond *Perceptrons* (Minsky & Papert 1969):". O parágrafo estende o resultado; o ano agora nomeia o livro que o contém | t10, site 2 (ano 1969) | `year-citation` |
| `book-src/36-decision-loss-threshold.md` | duas mudanças no mesmo caso. (a) "and twenty-five deaths attributed to the vaccine (Neustadt & Fineberg 1983)" **deletado**: as 25 mortes não têm fonte alcançável (t11 A5), e §IV manda apagar o token, não reescrevê-lo; os "some five hundred cases" ficam, dentro da faixa 362–532 que a fonte reporta. (b) "The check that would have caught 1976 is a term-by-term one" → "The check that would have caught the 1976 programme (Neustadt & Fineberg 1983) is a term-by-term one" | t10, site 3 (ano 1976); t11 A5 (PARCIAL) | `year-citation` |
| `book-src/37-model-fit-and-error.md` | "(e.g., portfolio insurance models amplifying the 1987 market crash)" → "(e.g., portfolio insurance rules that sell as the market falls, amplifying the decline)". O ano saiu em vez de ganhar fonte: não há entrada para o Brady Report no Appendix C e a causalidade da queda é contestada, então citar seria inventar atribuição. O mecanismo — regra que vende quando o preço cai — é o que o argumento precisa, e é o que ficou | t10, site 4 (ano 1987) | `year-citation` |
| `book-src/67-layer-epidemiology-outbreak-surveillance-order.md` | "which is why the Chao estimator is read as a lower bound on the uncounted part rather than as a point estimate of it (Chao 1987; Chao & Jost 2012)" → "which is why the estimator Chao (1987) proposed is read as a lower bound on the uncounted part rather than as a point estimate of it". A citação deixa de cobrir a leitura de limite inferior, que é derivação do próprio parágrafo (inflação de $m$ → $\hat N$ enviesado para baixo, exibida duas linhas acima), e passa a cobrir só o estimador. É o artigo que se anuncia como "A point estimator and its associated confidence interval … are proposed"; atribuir-lhe o limite inferior era atribuição falsa. Chao (1987) e Chao & Jost (2012) permanecem no Appendix C, onde 06 e 34 os usam | t11 A3 (FALSO) | `citations` |
| `.github/checks/structure.py` | `DECLARED_OPEN` esvaziado: as quatro entradas de `year-citation` estavam pagas e o portão as imprimia como "declared defects no longer present". O mecanismo `declare()` fica; o comentário passa a registrar onde as quatro foram pagas. Não é afrouxamento — o portão continua falhando em qualquer quinto site, e agora em qualquer ano sem fonte no parágrafo | t10 (portão); esta rodada | `year-citation` |

Dois itens de t11 com veredito CORRIGIDO não pediram mudança de texto, e a conferência está registrada aqui
para que a rodada não pareça tê-los ignorado: **A1** (Meng) — o reparo de `book-src/06-who-is-missing-from-the-table.md:184`
diz exatamente o que Bradley et al. (2021) sustentam, que Meng fornece o termo de deslocamento e não a lei
composta com DEFF; **A4** (decimais do ProPublica) — `book-src/81-layer-criminal-proof-forensic-epistemology-order.md:42`
e `book-src/appendix-d-claim-ledger.md:189` reproduzem 44.9/23.5 e 47.7/28.0 sem arredondar, e nenhum capítulo
afirma limiar de 0,5 (a frase não existe no texto). Nenhum valor AINDA INVERIFICÁVEL foi preenchido: o único
que restava — as 25 mortes — saiu do texto, como acima.

Verificação desta rodada: `structure.py` exit 0 ("all structural gates pass", census 518, órfãos 10,
display-first 25, near-dups 0, cross-refs 15, notation drift 3, **declared open 0 — nenhuma linha de dívida
impressa**), `wasm_gates.py` exit 0 (18 gates, 10 decididos, 0 falhos, 8 declarados abertos),
`git diff --stat` 38 arquivos, 1140 inserções, 123 remoções contra `9fc4e41` (o salto sobre a rodada 3 são os
dois portões de t10 no verificador). Sem commit.

---

## Rodada 5 (t14): os quatro achados low da revisão de t13

Nenhum portão novo. Quatro frases, e duas entradas bibliográficas que faltavam.

| Arquivo | O que mudou | Achado de origem | Portão |
| :--- | :--- | :--- | :--- |
| `book-src/28-the-machine-and-the-deception.md` | a frase do Mansfield passa a citar o estatuto no próprio local: "§203 of the military authorization act for fiscal year 1970 (Public Law 91-121), enacted in 1969". O que a oração afirma também encostou no texto legal: "a direct and apparent relationship to a specific military function or operation" é a redação do §203 (83 Stat. 206), não um resumo nosso; "attached to" saiu porque o §203 já é a emenda. Entrada nova no Appendix C: Public Law 91-121 (1969), §203, 83 Stat. 204, 206, aprovada em 19 de novembro de 1969 — lida no PDF do Statutes at Large, não em resumo | t13 F1 | `year-citation` |
| `book-src/36-decision-loss-threshold.md` | "reached roughly forty-five million people" → "vaccinated over 45 million people (Centers for Disease Control and Prevention n.d.)". O caminho (a) da tarefa foi tentado e falhou: Neustadt & Fineberg não são legíveis nesta máquina e nenhum texto consultado atribui o contingente a eles. Ficou o caminho (b): o número conserva a formulação e a precisão de quem o reporta (a página do CDC diz literalmente "Over 45 million people were vaccinated"), e Neustadt & Fineberg permanecem atrás apenas dos casos de Guillain–Barré. O hedge "roughly" era nosso e saiu junto com o número que ele amaciava | t13 F2 | `citations` |
| `book-src/37-model-fit-and-error.md` | "Donald MacKenzie demonstrated" → "Donald MacKenzie (2006) demonstrated" (a entrada já estava no Appendix C e já era citada no mesmo capítulo); o exemplo vira leitura atribuída — "portfolio insurance rules that sell as the market falls, the case MacKenzie (2006) reads as counter-performative" — e "amplifying the decline" sai, porque a causalidade da queda de 1987 é contestada e a única coisa que o capítulo pode sustentar é o que MacKenzie faz com o caso | t13 F3 | `citations` |
| `book-src/67-layer-epidemiology-outbreak-surveillance-order.md` | "is read as a lower bound" → "is used here as a lower bound": o estatuto de limite inferior é escolha deste livro, não tese do artigo de 1987, e a álgebra do parágrafo (inflação de $m$ enviesa $\hat N$ para baixo) mostra o viés, não a leitura. A citação continua cobrindo só o estimador | t13 F4 | `leitura` |
| `book-src/appendix-c-sources.md` | duas entradas: Centers for Disease Control and Prevention (n.d.), *Guillain–Barré Syndrome (GBS) and Vaccines*, com o URL da página verificada; Public Law 91-121 (1969), §203, 83 Stat. 204, 206 | t13 F1, F2 | `leitura` |

Nota de censo: o census sobe de 518 para 521 linhas, e a causa é dupla. A linha do contingente passou a
carregar "45" sem ano de citação na mesma linha, porque "(Centers for Disease Control and Prevention n.d.)" não
tem ano por construção — inventar um ano para a página seria a única coisa que §IV proíbe de fazer com uma
data. E o rewrap das quatro frases mudou onde os numerais caem: "16 December 1976" e "fiscal year 1970" ficaram
em linhas que já não trazem o par nome+ano, de modo que o censo as lê como candidatas. É advisory nas duas
parcelas, e as duas são consequência da redação, não de valor novo.

Verificação desta rodada: `structure.py` exit 0 ("all structural gates pass", census 521, órfãos do Appendix C
9 — as duas entradas novas estão ambas citadas no texto —, display-first 25, near-dups 0, cross-refs 15,
notation drift 3, declared open 0), `wasm_gates.py` exit 0 (18 gates, 10 decididos, 0 falhos, 8 declarados
abertos), `git diff --stat` 38 arquivos, 1149 inserções, 125 remoções contra `9fc4e41`. Nenhum código de
portão foi tocado nesta rodada. Sem commit.

---

## Rodada 6 (revisão de t15): os dois achados low

Nenhum portão novo. Duas edições: uma data de acesso que existia e uma oração partida em duas.

| Arquivo | O que mudou | Achado de origem | Portão |
| :--- | :--- | :--- | :--- |
| `book-src/36-decision-loss-threshold.md` | `(Centers for Disease Control and Prevention n.d.)` → `(Centers for Disease Control and Prevention 2024)`. A página é servida com "Last Reviewed: December 20, 2024"; `n.d.` era a única forma de citação sem ano e, por construção, escapava ao `year-citation`. A entrada do Appendix C acompanha o ano | t15 L1 | `year-citation` |
| `book-src/appendix-c-sources.md` | entrada do CDC passa de (n.d.) a (2024), com o URL mantido | t15 L1 | `year-citation` |
| `book-src/28-the-machine-and-the-deception.md` | a oração do Mansfield, longa e com três incisos, quebra em duas: a exigência na primeira, a identificação do estatuto ("§203 of the military authorization act for fiscal year 1970 (Public Law 91-121), enacted in 1969") na segunda. A citação não sai de perto do que data | t15 L2 | `year-citation` |

Verificação desta rodada: `structure.py` exit 0, `wasm_gates.py` exit 0, `git diff --stat` conferido no fecho.
Nenhum portão tocado. Sem commit.

---

## Rodada 8 (t20): reparo consolidado da Parte V

Fecha os achados de `audit/audit-partV-argument.md` (t17), `audit/audit-partV-provenance.md` (t16) e
`audit/audit-partV-algebra.md` (t18). Regra aplicada em todos os sítios: o token sai, o argumento fica;
nenhum número substituto foi inventado; valor de literatura na precisão da fonte, com autor e ano na
mesma frase.

| Arquivo | O que mudou | Achado de origem | Portão |
| :--- | :--- | :--- | :--- |
| `book-src/68-layer-pharmacovigilance-regulatory-toxicology-order.md` | outcome 3: sai "Prove by the **Disciplinary Incompleteness Theorem**" (o teorema não existe em nenhum arquivo; o `grep` só devolvia as duas outcomes) e entra "Show, from the survival-time condition of **Collapse Proposition 2**, why a `(2, 0, 1)` … takes it when review latency outruns the patient population's survival horizon". A condição final da outcome é preservada | t17 A1 (GRAVE) | `collapse-refutation` (a proposição em `book-src/41-the-triad-and-the-minimum.md` segue carregando o seu "It fails if…"); a correspondência outcome↔corpo que este reparo restabelece é leitura |
| `book-src/69-layer-metascience-scientometrics-replication-order.md` | outcome 3: mesma re-ancoragem em P2, preservando "when replication horizons outlast deployment" | t17 A2 (GRAVE) | idem |
| `book-src/69-layer-metascience-scientometrics-replication-order.md` | outcome 2: "Ioannidis's Positive Predictive Value Theorem" (nome com uma só ocorrência no corpus) trocado pela fórmula que o corpo deriva (`:24`), com o denominador exato do corpo, incluindo o termo `u(1-\alpha)` | t20 (critério 1 de aceitação; nome de sítio único) | `math` (paridade de `$`) e leitura |
| `book-src/67-layer-epidemiology-outbreak-surveillance-order.md` | outcome 3 rebaixada da promessa de P3 para o que o corpo argumenta: "leaves the ratio $d/T$ with no site charged with publishing it, while the window is still open" | t17 A3 (GRAVE) | leitura (o portão não lê outcomes) |
| `book-src/51-layer-micro-kinetic-sensorimotor-order.md` | exercício 4 reescrito para material impresso: a fold no nó raiz, cujo cone de ancestrais é vazio, e o que as seis arestas que a Bridge imprime licenciam | t17 A4 (GRAVE) | leitura |
| `book-src/appendix-b-architecture-verification.md` | a promessa de B.2 passa a dizer o que a tabela entrega (estrato, sítio primário, tripla, assinatura, regime de exposição) e onde a ascendência está impressa: no blockquote do próprio capítulo, que nomeia as camadas em que a ordem se apoia — 34 dos 35 imprimem "It builds on", o raiz não imprime nenhuma | t17 A4 (GRAVE) | §V-5 (paridade blockquote ↔ Tabela B.1, intocada) |
| `book-src/part-v-introduction.md` | a promessa de que as seções numeradas "begin with the site the order leads with" (verdadeira em 3 de 35) vira "argue the order's own subject and then read the signature" | t17 (MENOR, bloco D-c) | leitura |
| `book-src/59-layer-experimental-sampling-causal-order.md` | a classe `(3, 0, 0)` passa a ser nomeada como o Appendix A.6 item 24 e a Tabela B.1 a nomeiam: **Pure Epistemic Concentration** (blockquote, lista de sítios e §2) | t17 (MENOR, bloco D-b) | leitura |
| `book-src/77-layer-game-theory-strategic-adversarial-order.md` | parágrafo novo no §1 argumentando o sítio que dava nome à tripla sem aparecer no texto: o enquadramento como variável estratégica (`T1Framing`), com `T2Severity` precificando a resposta e `T6Performative` remodelando a população | t17 (MENOR, bloco D-a) | leitura |
| `book-src/76-layer-sequential-decision-bandit-real-options-order.md` | as taxas de Wald passam a `\alpha_W`, `\beta_W`; a deriva passa a `\mu` e o rendimento a `\delta = r - \mu`; o limite é escrito `$r/\delta$` sob rendimento positivo, com a frase que diz que em `\mu \ge r` não existe raiz acima de um e o hurdle é indefinido | t18 A1 (inferência não seguia) e A2 (notação) | `ratio-limit` |
| `book-src/78-layer-macro-finance-econometrics-actuarial-order.md` | o encolhimento passa a `k_{\mathrm{BM}}`; a cota `\varphi(k)/k` passa a ser declarada sobre a probabilidade de cauda naquele afastamento, não sobre o tamanho da queda, e `k` é distinguido do tamanho amostral que contava dois parágrafos antes | t18 A3 (notação e redação) | `notation`, `math`, leitura |
| `book-src/84-layer-infrastructure-nuclear-hydraulic-safety-order.md` | a fração de causa comum passa a `\rho_{\mathrm{cc}}` (o `\rho_t` de `:27` é utilização) | t18 A4 (notação) | `notation` |
| `book-src/84-layer-infrastructure-nuclear-hydraulic-safety-order.md` | a faixa de Hurst `0.70`–`0.80` sai; fica o que a fonte sustenta e o argumento usa, `$H$ above $0.5$`; a frase de Fukushima ganha o `National Diet of Japan Fukushima Nuclear Accident Independent Investigation Commission 2012` na cláusula da inundação (entrada já existente no Appendix C); a frase de Bhopal é reescrita na precisão de Broughton (2005): scrubber desligado três semanas antes, refrigeração drenada, flare fora de ação por três meses — e "operator warnings were discounted" sai, por não ter fonte | t16 B4, B5, B6 | `citations` |
| `book-src/85-layer-polycentric-commons-urban-biospheric-order.md` | "cutting employment for some $35{,}000$ people" sai; o fato (a economia de centenas de comunidades costeiras) fica, com as citações que já estavam na frase | t16 B1 | `citations` |
| `book-src/appendix-c-sources.md` | entrada nova: **Broughton, E.** (2005). The Bhopal disaster and its aftermath: a review. *Environmental Health*, 4, 6. <https://doi.org/10.1186/1476-069X-4-6> | t16 B6 | índice de órfãos (a entrada nasce citada) |

**Conferências que não viraram edição.** B2 (`book-src/82-layer-…:79-81`, júri de US$125 mi remitido a
US$3,5 mi): números mantidos — a opinião recursal em 119 Cal. App. 3d 757 trata do valor punitivo *já
reduzido* pelo tribunal de primeira instância, que é o que a frase afirma, e a citação existente basta.
B3 (`book-src/81-layer-…:34`, janela 2003–2012): janela confirmada — contratação em 2003, afastamento em
fevereiro de 2012 e demissão em março de 2012; a frase fica como está. Nenhum dos dois foi registado
como dívida porque nenhum precisou de reparo.

**Portão provado por mutação.** Replantar `$r/(\alpha)$` no lugar de `$r/\delta$` em
`book-src/76-layer-sequential-decision-bandit-real-options-order.md:43` faz `structure.py` sair 1 com
`[ratio-limit] … the limit r/(\alpha) names no inequality between r and \alpha in its section`; restaurado
o texto, exit 0. Nada em `.github/checks/` foi tocado nesta rodada.

**Resíduo declarado, com o sítio e a ação.** A dívida que `DECLARED_OPEN["ratio-limit"]` cobria
(`book-src/76-layer-sequential-decision-bandit-real-options-order.md:43`) foi paga no texto, então o portão passa a imprimir
`declared defects no longer present (remove the entry): 1 — [ratio-limit] book-src/76-layer-sequential-decision-bandit-real-options-order.md: r/(\alpha)`.
É aviso, não falha: `structure.py` sai 0. A entrada não foi removida porque `.github/checks/` está fora do
`inScope` desta rodada (o harness recusa a conclusão por arquivo não declarado); a ação é de uma rodada de
portões, e é remover uma linha do dicionário, sem tocar em `check_ratio_limit` nem em limiar algum.

Verificação desta rodada: `structure.py` exit 0 ("all structural gates pass", census 521, órfãos do
Appendix C 9, near-dups 0, cross-refs 15, notation drift 3, declared open 0 — a única declaração obsoleta que sobra é a de `ratio-limit`, registada acima),
`wasm_gates.py` exit 0 (18 gates, 10 decididos, 0 falhos, 8 declarados abertos). Sem commit.

Nota de cobertura: `grep -o 'book-src/[0-9a-z-]*\.md' audit/repair-report.md` lista, além dos arquivos
alterados nesta série, dois caminhos herdados de rodadas anteriores que foram **conferidos e não alterados**
(`book-src/81-layer-criminal-proof-forensic-epistemology-order.md` e
`book-src/appendix-d-claim-ledger.md`, ambos do achado A4 de t5) — são referências de leitura, não edições.

## Rodada 8b (t22): os três achados da revisão de t21

Fecham F1, F2 e F3 de `audit/review-t21.md`. Nenhum portão tocado; `structure.py` e
`wasm_gates.py` verdes.

| Achado | Arquivo | O que mudou | Portão |
| :--- | :--- | :--- | :--- |
| F1 high: numeral falso no out-degree do nó raiz | `book-src/51-layer-micro-kinetic-sensorimotor-order.md:177` | a Bridge perde "the **six** outgoing edges" e passa a "the outgoing edges $\mathcal{G}_1$ declares for it" — o numeral saiu em vez de virar oito, porque oito também é um numeral que ninguém reconta | `leitura` |
| F2 medium: exercício aponta as arestas para quem não as imprime | `book-src/51-layer-micro-kinetic-sensorimotor-order.md:168` | exercício 4 lê as arestas "off the blockquotes of the chapters that name this layer as a layer they build on"; a remissão à Bridge do próprio capítulo saiu e a pergunta ficou | `leitura` |
| F3 low: glosa errada do $k$ anterior | `book-src/78-layer-macro-finance-econometrics-actuarial-order.md:40` | "not the sampling size it counted above" → "not the shrinkage factor whose subscript it named above", que é o $k_{\mathrm{BM}}$ de §1 | `leitura` |

Contagem que motivou F1, impressa aqui para não ser refeita por memória: oito capítulos
nomeiam `[Layer 01]` no blockquote — 52, 53, 54, 55, 56, 61, 70 e 76 — logo o out-degree
do raiz é oito pela regra que `book-src/appendix-b-architecture-verification.md:171`
instalou ("the chapter blockquote that names the layers it builds on").

Verificação desta rodada: `structure.py` exit 0 ("all structural gates pass", census 521,
órfãos do Appendix C 9, near-dups 0, cross-refs 15, notation drift 3, declared open 0);
`wasm_gates.py` exit 0 (18 gates, 10 decididos, 0 falhos, 8 declarados abertos).

**Follow-up R1 da revisão de t23 (fechado nesta passagem).** `book-src/78-layer-macro-finance-econometrics-actuarial-order.md:40`
dizia "not the shrinkage factor whose subscript it named above" — `k` é o símbolo-base e o subscrito é `BM`,
então a frase descrevia mal a relação. Agora nomeia o objeto: "not the shrinkage factor
$k_{\mathrm{BM}}(n_{\mathrm{eff}})$ defined above", com os subscritos escapados como no resto do capítulo
(a primeira gravação deixou `n_{` sem escape e o portão de notação avisou; corrigido antes de fechar).
`structure.py` exit 0, `wasm_gates.py` exit 0.
