Revise adversarialmente o manuscrito Stochology (book-src/: 86 capítulos, 5 apêndices, textos de parte) em rodadas coordenadas. Cada rodada começa pela leitura do livro inteiro e só depois se divide entre agentes. Corrija vulnerabilidades e enriqueça o texto apenas com fontes primárias efetivamente lidas, em padrão de publicação.

Working directory: /home/silvano-neto/Documents/stochology
Manuscript language: English. Commit messages: Portuguese, one commit per coherent repair.
Binding contract: AGENTS.md — read it in full before any edit; it overrides this prompt on any conflict.

## Operating rules
- Local commits only; never push.
- Retitling a chapter or appendix, editing SUMMARY.md titles, or modifying chapter structure require human pre-approval with the inline diff and rationale (AGENTS.md §III).
- Architectural revisions (to the Nuclear Triad, Collapse Propositions, mediation circuit, guard sites, strata/orders, chapter/appendix structure, AGENTS.md, verification checks, or crates/) must NEVER be made unilaterally by any agent. Any grounded proposal for architectural revision must be escalated to the human as a concrete proposal detailing justification, primary literature/algebraic derivation, and the expected blast radius. Once pre-approved by the human, the team executes the change coherently across the manuscript, appendices, AGENTS.md, checks, and crates/.
- The book is the authority over the normative core (crates/), never the reverse. The manuscript must not mention, depend on, or be adjusted to match the core. After the text changes (including any human pre-approved architectural revision), the core is brought into agreement with the book (see Core sync).
- All working material lives in the conversation artifact directory, NEVER in the repository: the Book Map, the findings, the source log, and the round reports. An earlier audit folder was deliberately removed from the repository.
- Baseline: before any edit, run `python3 .github/checks/structure.py` and save its full output. Every wave must end with this check passing and with no advisory that the baseline did not already have.

## Repair doctrine (AGENTS.md §IV)
- A well-founded claim is never weakened.
- An unfounded claim is narrowed to what its source supports, or deleted. It is never kept alive with a new number, a vaguer paraphrase, or a substitute citation.
- Every deletion is recorded in the round report with the quoted passage and the reason.
- Deleting, inverting, or modifying a thesis-bearing or architectural claim of §I (the Triad, the three Collapse Propositions, the minimum, the diagnostic-not-control-law status, the mediation circuit, guard sites, strata, or orders) requires explicit human pre-approval before commit. Proposals must specify the proposed replacement, justification, and comprehensive blast radius.
- Enrichment replaces weak passages; it does not append. Growth that no finding justifies is itself a finding for the red team.

## Research protocol
- Web search is for DISCOVERING sources only. Never take a value or a quotation from a search summary, a secondary review, or AI output.
- A value or quotation may enter the text only if the agent has retrieved the source's own text and logged, in the source log, the verbatim passage and its locator (page, table, section; a canonical locator for classical texts, e.g. Stephanus or Kühn). Acceptable routes: the publisher page, a PDF, a DOI resolved through doi.org or Crossref, a critical edition.
- Abstract-only, paywalled, or unretrievable sources do not enter.
- Cite in the same sentence as the claim: author(s) and year, with the value exactly as the source states it, at the source's own precision. A numeric instantiation is never a derivation.

## Guia de pesquisa — expansão e refinamento da Estocologia

**Para que serve:** orientar os pesquisadores e revisores das rodadas descritas neste prompt. Não é uma lista de fontes. É um mapa de **onde o programa pode crescer, onde precisa ficar mais preciso e onde está mais exposto**, com pistas para começar a investigação.

> [!IMPORTANT]
> **Nenhum valor numérico deste guia pode ir para o manuscrito**, e o guia não traz nenhum. Todas as pistas bibliográficas são **hipóteses de trabalho**: autor, ano e o que ele *provavelmente* sustenta. Antes de qualquer uso, o pesquisador obtém o texto da fonte e registra o trecho literal e o localizador no log de fontes (§IV do `AGENTS.md` e o protocolo de pesquisa). Quando o guia diz que um autor está "ausente do Apêndice C", isso vem de uma busca por nome no arquivo, que precisa ser confirmada antes de se afirmar uma lacuna.

### 0. Três modos de ganho

Cada oportunidade abaixo é marcada com o modo de ganho que oferece:

| Modo | O que é | Exemplo de resultado |
| :--- | :--- | :--- |
| **Refinamento** | Tornar mais preciso o que o livro já afirma: definição, premissa explícita, limite declarado, condição de falha | Uma proposição de colapso ganha as condições sob as quais *não* vale |
| **Defesa** | Enfrentar o crítico mais forte, com fonte, e mostrar o que sobrevive | O argumento de Harsanyi contra o maximin, confrontado com o mínimo de Leontief |
| **Expansão** | Ligar a Estocologia a um corpo de conhecimento que ela ainda não incorpora, quando a ligação fortalece o circuito | Engenharia de resiliência como literatura da margem vital em operação |

Uma expansão que não refina nem defende nada já existente é **suspeita de inchaço** e cai para o fim da fila.

### 1. Compromissos arquiteturais sujeitos a pré-aprovação humana

Estes pontos são estruturais. Nenhum agente pode alterá-los unilateralmente. Qualquer pesquisa, evidência ou necessidade conceitual que sugira revisar estes compromissos deve ser **escalada ao humano como proposta formal** (com fundamentação em fontes primárias/derivação e mapa de impacto transversal/blast radius). Somente mediante **pré-aprovação explícita do humano**, a alteração arquitetural é executada coordenadamente pelo time em todo o livro, apêndices, `AGENTS.md`, testes e código.

1. **A tríade e o mínimo.** Os três eixos não se substituem, e o veredito é um mínimo, não uma soma ponderada (`The minimum is not a weighted sum`).
2. **Diagnóstico, não controle.** O veredito triádico é dobrado a partir de uma trajetória já concluída; quem age é o operador canônico passo a passo (§I).
3. **Sete sítios de guarda.** São 3·2+1 travessias do circuito (premissas do §V-10). Alterações na contagem ou na topologia do circuito de mediação exigem pré-aprovação humana.
4. **Trinta e cinco ordens.** C(7,3) = 35, uma por capítulo da Parte V. Criar, suprimir ou reorganizar ordens/estratos exige pré-aprovação humana.
5. **As âncoras de estrato são designações**, não consequências da tabela (`designation recorded rather than derived`).
6. **Composição dos efeitos de desenho:** aditiva, com o produto sendo apenas limite superior (`the exact composition is additive`).
7. **O núcleo segue o livro.** Toda expansão ou revisão arquitetural é argumentada e estabelecida no texto; o código de `crates/` e os testes de CI são sincronizados em seguida.

### 2. Frentes de pesquisa

Cada frente traz: **onde fica no livro**, **pergunta**, **modo de ganho**, **pistas** (a verificar) e **risco**.

#### F1. Fundamentos de decisão: o que o mínimo é, axiomaticamente
**Onde:** cap. 41 (§2 e §6), cap. 50 (§9.4), cap. 36.
**Pergunta:** que sistema de axiomas sobre preferências ou sobre avaliação de estados caracteriza a agregação por mínimo entre eixos que não se substituem, e o que a distingue das famílias vizinhas?
- **Refinamento:** declarar quais axiomas o mínimo satisfaz e qual axioma clássico ele abandona de propósito. O candidato óbvio é algum tipo de independência ou continuidade.
- **Defesa:** o mínimo é sensível *apenas* ao pior eixo. Isso desperdiça informação quando os outros dois eixos variam muito (o próprio §6 do cap. 41 já aponta limites).
- **Expansão:** ligação com a teoria da ambiguidade, em que o mínimo aparece como critério sobre um *conjunto* de cenários, e não sobre eixos.

**Pistas:** Gilboa & Schmeidler (1989), maxmin expected utility; Bewley (1986/2002), preferências incompletas; Seidenfeld, Schervish & Kadane (1995), preferências parcialmente ordenadas; Klibanoff, Marinacci & Mukerji (2005), ambiguidade suave, um contraponto ao extremismo do mínimo; Buchak (2013), risco ponderado; Rawls (1971) versus Harsanyi (1975), o debate maximin. Os de Gilboa–Schmeidler, Klibanoff–Marinacci–Mukerji, Rawls e Harsanyi aparecem como ausentes na busca.

**Risco:** confundir o mínimo *entre eixos* (a tese do livro) com o maxmin *sobre cenários*. São objetos diferentes, e a analogia precisa ser marcada como analogia.

#### F2. Agregação sem compensação na escolha social
**Onde:** cap. 41, cap. 50 (§9.3.4, freio 4), cap. 20.
**Pergunta:** o teorema de impossibilidade de Arrow e seus descendentes dizem algo sobre agregar três salvaguardas heterogêneas? O mínimo de Leontief escapa da impossibilidade, e a que custo (por exemplo, abrindo mão da sensibilidade ou da comparabilidade)?
- **Refinamento:** dizer explicitamente que tipo de comparabilidade entre os eixos o mínimo pressupõe. A "invariância ordinal de caixa" do cap. 41 é o ponto de partida.
- **Defesa:** antecipar a objeção de que um mínimo exige eixos comensuráveis na mesma escala.

**Pistas:** Arrow (1951); Sen (já presente, com 3 ocorrências), sobre comparabilidade interpessoal e informação na escolha social; a literatura de análise multicritério não compensatória (verificar autores na escola de Roy/ELECTRE).

#### F3. Robustez e incerteza profunda: os vizinhos metodológicos mais próximos
**Onde:** cap. 36, cap. 44, cap. 47, cap. 50 (§9.1, as famílias metodológicas).
**Pergunta:** em que a Estocologia se distingue da tomada de decisão robusta, do *info-gap* e da robustez à Hansen–Sargent? O que ela acrescenta, e o que deveria importar delas?
- **Defesa:** é a objeção "isto já existe com outro nome". Sem uma resposta clara, o programa parece redundante.
- **Refinamento:** o diferencial provável é o circuito (mundo → arquivo → instituição) e o eixo de governança, que esses métodos tratam como externos. Isso precisa ser *mostrado*, não só afirmado.

**Pistas:** Hansen & Sargent (2008), *Robustness*; Lempert, Popper & Bankes (2003), RDM; Marchau et al. (2019), *Decision Making under Deep Uncertainty*; Ben-Haim (já presente, com 1 ocorrência), info-gap; Kay & King (2020), *Radical Uncertainty*.

#### F4. As três proposições de colapso: condições de falha
**Onde:** prefácio §2, cap. 41 §3, cap. 50 §9.5.
**Pergunta:** para cada proposição, qual é o caso empírico ou histórico que a falsearia, e a literatura registra casos assim?
- **CP1** (sem margem epistêmica → subdimensionamento cego): existem instituições que ignoraram a ausência não aleatória de dados e *não* subdimensionaram? Se existem, por quê? Talvez por excesso de reserva física por acaso, o que seria sorte do resultado (cap. 04).
- **CP2** (sem margem vital → lucidez impotente): casos de alta alavancagem com plena consciência da incerteza que sobreviveram. É o viés de sobrevivência ao contrário.
- **CP3** (sem governança → captura cínica): instituições com governança fraca e reservas preservadas. Os regimes de comuns de Ostrom podem ser contraexemplo ou confirmação, conforme se defina "governança".

**Modo:** refinamento e defesa. É a frente **mais valiosa para a seriedade científica** do programa, porque o §I promete que as proposições são falseáveis.

**Pistas:** Ostrom (presente) e Ostrom (2005), *Understanding Institutional Diversity*; Tainter (1988), *The Collapse of Complex Societies*; Perrow (presente), Vaughan (presente), Reason (presente) para casos de acidente; Oreskes & Conway (2010) e Michaels (2008) para a agnotologia da CP3 (Proctor já presente); Kerr (1975) e Holmström & Milgrom (1991, a verificar se já presente) para o jogo com indicadores.

**Risco:** escolher casos a dedo. O registro deve incluir os casos contrários encontrados, e não só os favoráveis.

#### F5. O circuito e os sítios de guarda: o que a literatura de sistemas sustenta
**Onde:** cap. 29, cap. 26, Apêndice B.1, Apêndice E (Tabela E.1).
**Pergunta:** o teorema do bom regulador, a variedade requisita e a teoria dos sistemas viáveis sustentam que o circuito é indivisível? Ou permitem cortes legítimos? (O cap. 29 já tem "When the full circuit is an overcost".)
- **Refinamento:** tornar explícita a correspondência entre os sete sítios e os conceitos cibernéticos clássicos (sensor, comparador, atuador, metassistema), sem derivar a contagem dessas fontes, o que §V-10(e) proíbe.
- **Expansão:** usar a engenharia de resiliência como literatura das travessias, isto é, como os sistemas falham *entre* planos.

**Pistas:** Conant & Ashby (1970), o teorema do bom regulador (Ashby presente; verificar o artigo); Beer (presente) e a recursão do VSM; Hollnagel, Woods & Leveson (2006), *Resilience Engineering*; Leveson (presente), STAMP/STPA, para controle hierárquico com segurança; Rasmussen (presente), migração para a fronteira.

**Risco:** §V-10(e). A contagem vem das travessias do circuito, e nenhuma fonte externa pode "confirmá-la" unilateralmente. Qualquer proposta de revisão do circuito ou dos sítios de guarda deve ser escalada para pré-aprovação humana com a demonstração do impacto completo.

#### F6. Margem epistêmica: identificação parcial, seleção e o arquivo
**Onde:** cap. 33, 34, 44, 45, 06.
**Pergunta:** a margem epistêmica do livro (ρ_{R,Y}, DEFF, limites de identificação parcial) cobre os principais mecanismos de distância entre mundo e arquivo? O que falta?
- **Expansão:** transportabilidade e fusão de dados, ou seja, quando o arquivo vem de uma população e a decisão vale para outra; seleção à Heckman como caso paramétrico contra o qual a abordagem parcial se define.
- **Refinamento:** alinhar a notação. Hoje há deriva em `\rho\_{R, Y}` no cap. 45, segundo o aviso do `structure.py`.

**Pistas:** Tamer (2010), revisão de identificação parcial (ausente; Manski presente); Heckman (1979); Bareinboim & Pearl (2016), fusão de dados; Meng (presente), o paradoxo do *big data*; Obermeyer et al. (2019), um viés de arquivo com efeito institucional, que costura o circuito inteiro.

#### F7. Margem vital: ruína, ergodicidade e caudas
**Onde:** cap. 46, 47, 03, 78.
**Pergunta:** a condição TTS < TTR se relaciona formalmente com a não ergodicidade e com a teoria da ruína? A literatura sobre caudas pesadas em decisões catastróficas sustenta o tratamento do livro, ou o contesta?
- **Expansão:** o "teorema sombrio" de Weitzman como caso-limite da margem vital; os pontos de virada como fronteiras de ruína em sistemas acoplados.
- **Defesa:** a crítica de que o argumento ergódico é trivial ou mal formulado. Há réplicas publicadas a Peters, e o pesquisador deve procurá-las.

**Pistas:** Peters (2019), *Nature Physics* (verificar se é o "Peters" já presente); Weitzman (2009); Lenton et al. (2008); Taleb (presente); Kelly (presente) e a ligação com o crescimento logarítmico; Holling (presente) e Scheffer (presente) para resiliência ecológica.

#### F8. Governança: do alerta da linha de frente à captura institucional
**Onde:** cap. 40, 49, 20, 80, 85.
**Pergunta:** quais mecanismos institucionais, documentados empiricamente, convertem um alerta da linha de frente em ação, e quais o silenciam? É o desconto testemunhal da CP3.
- **Expansão:** segurança psicológica, *street-level bureaucracy*, organizações de alta confiabilidade, *accountability*, deliberação.
- **Refinamento:** separar a injustiça testemunhal (Fricker, presente) dos mecanismos *organizacionais* que a produzem.

**Pistas:** Edmondson (1999); Lipsky (1980); Weick & Sutcliffe (2001); Bovens (2007); Hirschman (1970), *Exit, Voice, and Loyalty*; Fishkin (2009) e Mansbridge et al. (2012); Coase, Acemoglu & Robinson (2012), Greif (2006), Bowles (2016) para instituições e incentivos. Todos aparecem como ausentes na busca.

**Risco:** o freio 3 (teatro procedimental). Expandir a governança não pode virar receita de processo.

#### F9. Precaução e a ética de impor riscos
**Onde:** cap. 19, 27, 40.
**Pergunta:** onde a Estocologia fica no debate sobre o princípio da precaução? Ela herda a objeção de paralisia de Sunstein, ou escapa dela pelo freio 2 (entesouramento regressivo)?
- **Defesa:** é a crítica mais previsível de um leitor de políticas públicas.
- **Refinamento:** declarar se a tríade é uma versão de precaução, uma alternativa a ela ou um critério que diz quando aplicá-la.

**Pistas:** Sunstein (2005), *Laws of Fear*; Gardiner (2006), o "core precautionary principle" na forma de maximin rawlsiano (que liga a F1 e a F9); Steel (2015); Stirling (2010, *Nature*), "Keep it complex"; Hansson (presente), ética do risco; Funtowicz & Ravetz (1993), ciência pós-normal.

#### F10. Percepção, julgamento e ruído: o agente concreto
**Onde:** Parte I (cap. 01–08), cap. 27, 51.
**Pergunta:** a literatura sobre julgamento humano (heurísticas, ruído, previsão calibrada) sustenta a passagem da Parte I, do agente intuitivo ao circuito? Ou a complica?
- **Expansão:** a calibração de previsores como teste empírico da margem epistêmica individual; o ruído como componente do arquivo que não é viés.
- **Defesa:** a escola de racionalidade ecológica (Gigerenzer, presente) diria que margens explícitas às vezes *pioram* o desempenho ("less is more"). Haldane & Madouros (2012) argumentam algo parecido para a regulação.

**Pistas:** Tetlock (2005) e Tetlock & Gardner (2015); Kahneman, Sibony & Sunstein (2021), *Noise*; Slovic (1987); Kasperson et al. (1988), amplificação social do risco; Haldane & Madouros (2012), "The dog and the frisbee".

#### F11. Demarcação: a Estocologia é um programa progressivo?
**Onde:** prefácio ("Epistemic Demarcation"), cap. 18, cap. 50 (§9.5–9.7).
**Pergunta:** pelos critérios de Lakatos (presente), quais predições *novas* a Estocologia faz, além de reinterpretar casos conhecidos? Os quatro falseadores pré-registrados bastam?
- **Refinamento:** um inventário honesto, separando o que o programa *deriva* do que *herda*. O cap. 50 já tem a seção "What the programme derives versus what it inherits"; a pesquisa deve completá-la.
- **Defesa:** a acusação de programa de "acomodação" por excesso de vocabulário (as 35 ordens podem soar taxonômicas).

**Pistas:** Lakatos (presente); Mayo (presente), severidade; Kitcher (1993) e Longino (presente), sobre o caráter social da objetividade; Daston (1988), *Classical Probability in the Enlightenment*, para a genealogia do "razoável".

#### F12. Genealogia clássica: precisão filológica
**Onde:** prefácio §1, cap. 16, 27, 29, Apêndice E.1.
**Pergunta:** as atribuições clássicas (Platão, *Filebo* 55e–56c; o arqueiro de Antipatro; a *stokhastikē tékhnē* galênica; *kairós* e *mētis*) estão ancoradas em edições críticas e na erudição atual?
- **Refinamento:** o arqueiro chega a nós via Cícero (*De Finibus* III.22), e a atribuição a Antipatro é discutida pelos especialistas. A palavra "Tarsus" não aparece no Apêndice C.
- **Expansão:** Aristóteles sobre a *phronesis* e as artes estocásticas (por exemplo, *Retórica* I.1, *Ética a Nicômaco* II e VI); Nussbaum (1986), *The Fragility of Goodness*, que ligaria a sorte moral do cap. 49 à tradição grega.

**Pistas:** Striker (1986/1996), "Antipater, or the art of living"; Detienne & Vernant (presentes), *mētis*; Daston (1988) e Hacking (presente) para Bernoulli, Huygens e Pascal.

**Risco:** mais alto em atribuição errada (categoria A). Sempre use o localizador canônico.

#### F13. Parte V: aprofundar cada ordem sem criar ordens
**Onde:** cap. 51–85 e a Tabela B.1.
**Pergunta (por ordem):** a combinação de guardas daquela ordem tem uma *assinatura empírica* reconhecível no seu domínio? Existe um caso documentado em que justamente aquelas três guardas falharam ou seguraram?
- **Refinamento:** cada capítulo da Parte V deveria ter pelo menos um caso de domínio com fonte primária, ligado explicitamente à sua tripla.
- **Expansão por relação:** usar o grafo do Mapa do Livro para achar pares de ordens que compartilham dois sítios de guarda, e verificar se os capítulos dialogam. É a categoria G da v2.

Pontos de partida por estrato, só como direção:
- **S1** (físico/cinético): assimilação de dados em geofísica; mecânica da fratura.
- **S2** (formal/medida): metrologia (o vocabulário VIM e o GUM como fontes normativas).
- **S3** (biológico): imunologia do próprio e do não próprio como governança.
- **S4** (clínico/epidemiológico): farmacovigilância (CAST já presente).
- **S5** (linguístico/arquivístico): crítica textual, justiça de transição.
- **S6** (decisão/econômico): opções reais (Dixit & Pindyck, presentes), Gittins (presente).
- **S7** (jurídico/infraestrutural): padrões de prova, SRE, segurança nuclear e hidráulica, comuns policêntricos.

**Risco:** quebrar a paridade com a Tabela B.1 ou a cardinalidade de §V-10 sem pré-aprovação humana. O conteúdo novo nunca altera o blockquote nem a tripla sem proposta formal de revisão arquitetural pré-aprovada pelo humano.

#### F14. Consistência interna como objeto de pesquisa
**Onde:** o livro todo.
**Pergunta:** os conceitos centrais (circuito, margem, veredito, freio, sítio) são usados no mesmo sentido em todas as Partes?

Essa frente não pede leitura externa, e sim o Mapa do Livro. Ela costuma revelar onde uma expansão anterior deformou um conceito, e vem antes das expansões novas.

### 3. Priorização sugerida

| Prioridade | Frentes | Por quê |
| :--- | :--- | :--- |
| 1 | F4, F11, F14 | Seriedade científica (falseabilidade, demarcação) e coerência; tudo o mais depende disso |
| 2 | F1, F2, F3, F9 | As objeções mais previsíveis de leitores especialistas ("já existe", "Harsanyi", "precaução paralisa") |
| 3 | F5, F6, F7, F8 | Aprofundar os três eixos e o circuito com literatura empírica |
| 4 | F10, F12 | Agente concreto e precisão genealógica |
| 5 | F13 | Aprofundar a Parte V ordem a ordem, guiado pelo grafo de relações |

### 4. Como usar este guia nas rodadas

1. Na **leitura do livro inteiro** (Passo 1 da rodada), o cartógrafo marca em cada capítulo quais frentes (F1–F14) o tocam.
2. As **ondas** (2 a 3 capítulos) são formadas juntando capítulos que compartilham uma frente. Os capítulos-parceiros são os outros capítulos marcados com ela.
3. O pesquisador da onda começa pelas pistas da frente, **obtém o texto da fonte**, registra trecho e localizador, e só então propõe a mudança.
4. Toda oportunidade investigada termina num destes estados: **incorporada**, **descartada** (a fonte não sustenta), **adiada** (exige uma onda com outros capítulos) ou **escalada** (toca um compromisso arquitetural da seção 1 para pré-aprovação humana).

#### Modelo de registro (uma entrada por oportunidade)

```markdown
### F<n>.<k> — <título curto>
- Capítulos: <NN, NN> · Parceiros: <NN, NN>
- Modo: refinamento | defesa | expansão
- Pergunta: <uma frase>
- Fontes obtidas: <autor, ano, título> — trecho literal: "<...>" — localizador: <p./tab./§>
- Fontes buscadas e descartadas: <autor, ano> — motivo (inacessível / não sustenta)
- Achado: <o que a fonte sustenta e o que não sustenta>
- Proposta para o texto: <corrigir / limitar / apagar / defender / expandir>
- Invariantes / compromissos arquiteturais tocados: nenhum | <item da seção 1> → proposta escalada para pré-aprovação humana
- Estado: incorporada | descartada | adiada | escalada
```

## Round structure (repeat until a stop condition)

### Step 1 — Whole-book pass (every round opens with it)
A cartographer reads the ENTIRE manuscript in reading order: preface, every chapter, the part texts, and every appendix. Reading is sequential, and after each file the cartographer writes that file's entry into the Book Map (artifact) before moving on. Round 1 builds the map; later rounds re-read the whole book and update it, paying particular attention to whether the previous round's repairs propagated.

For each file, the Book Map records:
- the claims of record and their warrants (literature / derivation / none);
- the concepts and notation it introduces and the ones it consumes from elsewhere (with the source chapter);
- its dependencies, backward and forward, and where its Bridge actually leads;
- tensions with other parts of the book: contradiction, drift, duplication, a claim used before it is established, an argument that another chapter undercuts;
- the research fronts (F1–F14) that touch it.

Across the whole book, the Book Map records:
- a relation graph of the deep links between chapters, including links that cross Parts (e.g. a Part V order relying on a Part III definition);
- a ranked findings list, built with the taxonomy and the severity scale below.

Taxonomy:
(A) misattribution — a source is cited for something it does not say;
(B) unwarranted number — a numeral with neither a literature nor a derivation warrant;
(C) overclaim, or an unfalsifiable statement — especially in the three Collapse Propositions, each of which must state how it could fail;
(D) internal contradiction, or terminology or notation drift across chapters;
(E) a missing strongest counter-argument from a named critic, with its source;
(F) the triadic verdict drifting into a control law or a weighted aggregate (§I, §V-9);
(G) a broken cross-part relation — a chapter that should engage another and does not, or that engages it wrongly.

Severity: critical (false or contract-breaking) / high / medium / low.

In round 1 only, also verify every existing Appendix D ledger row against the source's own text (§V-6) before any wave begins.

### Step 2 — Segmentation into waves
The coordinator forms waves from the Book Map, highest severity first:
- Each wave EDITS 2 to 3 chapters. Group them by depth of relation in the graph, not by numbering; a wave may cross Parts. Waves should cluster chapters sharing a research front (F1–F14).
- Each wave also names its relation partners: the chapters its team must READ in full, without editing them, so that the repairs engage the rest of the book and do not contradict it (including other chapters marked with the same research front).
- At most two waves run concurrently, and only if their edit sets AND their relation partners are disjoint.
- Wave team: one reviewer, one source researcher, one author (in its own working copy), and one independent citation verifier. Two-key rule: whoever inserts a citation never verifies it.
- The wave delivers its repairs. It also delivers the cross-chapter consequences it found, which go back to the Book Map as findings for later waves instead of being fixed outside the wave's edit set.

### Step 3 — Shared appendices
ONE owner holds Appendix C (sources) and Appendix D (ledger). Authors send proposed entries to the owner, who integrates them, de-duplicates them, keeps a single citation style, and enforces ledger parity: every numeric cell must be a value that source reports. No appendix cites a chapter as authority (§II).

### Step 4 — Integration and gates (after every wave)
The integrator merges the wave and runs every gate in the acceptance criteria. A failing wave is repaired before the next wave touching the same files starts.

### Step 5 — Core sync (after every wave that changes what the book says about productions, the ordered acceptance predicate, the diagnostic fold, the guard sites, or the Triad)
A core maintainer reads the changed chapters and Appendix B and brings crates/ into agreement with the book. Then `cargo test --offline` and `cargo test --doc --offline -p stoch-core` must pass, along with the wasm gates of CI if they are affected.
- The book is never edited to match the core.
- If the book is too ambiguous to determine the core's behavior, the fix is a finding against the BOOK (routed to a wave or to the human); the core does not get to decide.

### Step 6 — Round close
A red-team reviewer re-reads every diff of the round against this prompt and AGENTS.md. The round report is written, and the next round opens again at Step 1.

## Acceptance criteria (per round)
### Report (artifact directory)
- [ ] The Book Map is updated for every file, with its relation graph and research front tags (F1–F14).
- [ ] The ranked findings list shows, for each finding, the action taken (corrected / narrowed / deleted / defended with source) or the reason it was deferred to a later round.
- [ ] Every investigated research opportunity has an entry under the `F<n>.<k>` model, recording its outcome (incorporada, descartada, adiada, escalada).
- [ ] The source log gives, for every new or changed citation, the verbatim passage, the locator, and the verifier.
- [ ] Items awaiting human decision or pre-approved are listed: proposed retitles, and proposed architectural revisions (to §I theses, Section 1 commitments, circuit topology, guard sites, strata/orders, or AGENTS.md) with their rationale and blast radius.

### Provenance (§IV)
- [ ] Every new quantitative or empirical claim carries author(s) and year in the same sentence and the source's exact value.
- [ ] Every new source has an Appendix C entry, and every new numeric claim has an Appendix D row whose cells match the source.
- [ ] No prohibited apparatus appears in book-src/, README.md, CONTRIBUTING.md, or AGENTS.md, and the manuscript makes no reference to the core.
- [ ] The numeral census (§V-7) has been run over every modified file, and each new numeral is justified in the report.

### Structure and gates (§III, §V)
- [ ] `python3 .github/checks/structure.py` passes, with no advisories beyond the baseline.
- [ ] `mdbook build` succeeds and `python3 .github/checks/surface.py book` passes (§V-8).
- [ ] Every chapter 01–85 keeps exactly 3 outcomes, 3–5 exercises, and one `## Bridge` naming the next chapter. The preface and the appendices gain no spine. No file starts with an HTML marker.
- [ ] Every referenced figure exists and every figure remains referenced (§V-4).
- [ ] Concordance (§V-9): each canonical string still occurs at least once, and no contradicting variant appears, unless an architectural revision was explicitly pre-approved by the human and updated coherently across all required files.
- [ ] Parity holds between Table B.1, the Layer blockquotes, and the SUMMARY strata (§V-5), and guard-site cardinality holds, including the banned phrases of (e) (§V-10), unless an architectural revision was explicitly pre-approved by the human and updated coherently.
- [ ] If crates/ changed: the core tests pass, and the core agrees with the book as it now reads.

## Stop conditions
Stop and report to the human when any of these holds:
- a whole-book pass yields no new critical or high finding;
- an architectural revision is proposed (to §I theses, Section 1 commitments, circuit topology, guard sites, strata/orders, chapter/appendix structure, or AGENTS.md), awaiting human pre-approval before proceeding with execution;
- a retitle appears necessary (awaiting human pre-approval);
- the book is too ambiguous to fix the core's behavior and its sources do not settle the question;
- the human stops the work.
