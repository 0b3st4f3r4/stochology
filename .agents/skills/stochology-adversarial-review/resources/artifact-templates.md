# Modelos Canônicos de Artefatos Bitemporais (`<artifactDir>/stochology-review/`)

> [!CAUTION]
> **Regra de Higiene do Repositório:** Nenhum arquivo de trabalho ou log operacional entra em `book-src/`, `crates/` ou no histórico Git do livro. No **Modo A/B**, todos os artefatos vivem em `<appDataDir>/brain/<conversation-id>/stochology-review/`. Na revisão geral **Modo S (`112 × 6`)**, a constituição vive versionada em `research/guide.md`, enquanto todo estado persistente entre sessões e pareceres vive estritamente em `research/runs/` (listado no `.gitignore` e `.rumdl.toml`).

## 1. Layout Padrão do Diretório de Artefatos

```text
<appDataDir>/brain/<conversation-id>/stochology-review/
├── baseline.json                       # Gerado por `review_ops.py baseline --out ...`
├── shards-manifest.md                  # Gerado por `review_ops.py shards --out ...`
├── book-map/
│   ├── S1-preface-part1.md             # Cartografia integral do Fragmento 1
│   ├── S2-part2-math-phil.md           # Cartografia integral do Fragmento 2
│   ├── S3-part2-sci-part3-circuit.md   # Cartografia integral do Fragmento 3
│   ├── S4-part3-judgment-triad.md      # Cartografia integral do Fragmento 4
│   ├── S5-part4-framework-core.md      # Cartografia integral do Fragmento 5
│   ├── S6-part5-strata-1-to-4.md       # Cartografia integral do Fragmento 6
│   ├── S7-part5-strata-5-to-7.md       # Cartografia integral do Fragmento 7
│   └── cross-part-synthesis.md         # Grafo transversal, tensões D/G/H/I/J e auditoria Apêndice D
├── waves/
│   └── wave-R<r>-W<w>-brief.md         # Brief dialético (*Ars Inveniendi*) selado ex ante
├── source-log/
│   └── round-<r>-sources.md            # Registro literal de duas chaves (*Ars Judicandi*)
├── opportunities/
│   └── round-<r>-F-ledger.md           # Entradas F<n>.<k> (incorporada | descartada | adiada | escalada)
├── escalations/
│   └── ESC-<nn>-<slug>.md              # Propostas formais sujeitas à pré-aprovação humana
├── scratch/
│   └── verify_wave_R<r>_W<w>.py        # Verificação algébrica/simbólica independente (2ª chave)
└── reports/
    └── round-<r>-report.md             # Relatório consolidado da rodada + deleções + censo de numerais
```

---

## 2. Cartão de Arquivo no Mapa do Livro (`book-map/S*.md`)

Cada arquivo do fragmento recebe um cartão compacto após leitura integral:

```markdown
### `book-src/<arquivo>.md` — <Título `# `>
- **Papel / Tripla (se Parte V):** <Eixo/Filtro ou `Lnn · Sn · {Ta,Tb,Tc} · (E,V,G)`>
- **Reivindicações e Lastros (§IV):**
  - `<linha>`: <resumo da tese quantitativa/empírica/formal> → `literatura (<Autor Ano>)` | `derivação (<eq.>)` | `SEM LASTRO [ACHADO B]`
- **Conceitos & Notação:**
  - *Introduz:* `<símbolos/conceitos>` · *Consome:* `<símbolos>` (de `<capítulo-fonte>`)
- **Dependências & Ponte:**
  - *Montante/Jusante:* `<capítulos>` · *Bridge entrega a:* `<capítulo seguinte>`
- **Tensões Laminares & em Sobreposição:**
  - *Laminar (`A`–`G`):* <contradição, deriva de notação, ausência de condição de falha, irmão J(7,3) desconectado ou `nenhuma`>
  - *Superposta (`H`–`J`):* <sobrecarga >10 bits/s, cegueira pré-selo/vazamento de *skopós*, risco no Freio 3/algedônico, externalização inter-triádica ou `nenhuma`>
- **Frentes Tocadas:** `F<x>, F<y>`
```

---

## 3. Taxonomia Expandida `(A)`–`(J)` e Registro de Achado

- **`(A)` Misattribution / Deriva Filológica:** Fonte citada para o que não diz, atribuição secundária tratada como primária ou ausência de localizador canônico.
- **`(B)` Unwarranted Numeral / Deriva de Precisão:** Numeral sem lastro na mesma frase (literatura ou álgebra ao lado), arredondamento/reescala indevida ou instanciação numérica fingindo derivação.
- **`(C)` Unfalsifiable Claim / Limite sem Condição de Domínio:** Proposição sem condição explícita de falha ou razão/limite matemático sem restrição de positividade/ordem no denominador (`ratio-limit`).
- **`(D)` Internal Contradiction / Deriva Conceitual ou Notacional:** Oscilação de sentido ou grafia entre capítulos/apêndices.
- **`(E)` Missing Strongest Counter-Argument:** Ausência de confronto com o crítico nomeado mais forte da literatura primária.
- **`(F)` Diagnostic-to-Control Drift / Agregação Compensatória:** Uso de $M\_{\mathrm{Leontief}}$ como controlador passo a passo, meta escalar ou média/produto compensatório (§I, §V-9).
- **`(G)` Broken Cross-Part or Johnson-Graph Relation:** Falta de diálogo ou engajamento incorreto com capítulo-base (Partes II–III) ou ordem irmã em $J(7,3)$ compartilhando dois sítios.
- **`(H)` Destructive Guard Interference / Inchaço "Basileia IV":** Empilhamento de protocolos sem precificar o teto humano de $\sim 10\text{ bits/s}$ em $\tau\_{\mathrm{delib}}(I\_t)$, ou prosa burocrática inchada.
- **`(I)` Pre-Seal Blindness / Vazamento Retroativo do *Skopós* / Captura de Exceção:** Tratamento ingênuo de selos criptográficos, do `PRI`, de trajetórias multi-passo ($t > 1$) ou do Freio Reflexivo 3 / canal algedônico.
- **`(J)` Taxonomic Seam / Externalização Inter-Triádica / Apagamento Afetivo-Reprodutivo / Paralisia Generativa:** Costura compósita não declarada, omissão de impacto sobre a bacia de recepção do Outro ($\Theta\_{\mathrm{basin}}$), apagamento do desgaste alostático psíquico ou da infraestrutura reprodutiva do cuidado, "catedral determinativa" domesticando aporias reais, ou confusão entre abdução/*anagnórisis* legítima e fraude *ad hoc*.

```markdown
### [FIND-<R>.<N>] (<Categoria A–J> · <critical|high|medium|low> · <laminar|superposto>)
- **Locus:** `book-src/<arquivo>.md:<linhas>` (Parceiros afetados: `<arquivos>`)
- **Diagnóstico:** <descrição exata e teste que evidencia o defeito ou oportunidade>
- ** Guilda Responsável:** `<epi-methodologist | vital-cyberneticist | gov-ethicist | philology-archivist>`
- **Resolução:** `corrigido | estreitado | deletado | defendido com fonte | adiado | escalado (ESC-NN)`
- **Commit / Verificação:** `<hash local>` · Verificado por `<two-key-verifier>`
```

---

## 4. Brief Dialético de Onda (`waves/wave-R<r>-W<w>-brief.md`)

Selado **antes** da edição em `book-src/` (Momento I — *Ars Inveniendi*):

```markdown
# Wave R<r>-W<w> — <Título curto>
- **Edit Set (2–3 caps.):** `<arquivos>`
- **Core Relation Partners (leitura integral):** `<arquivos de review_ops.py wave-plan>`
- **Frentes (`F1`–`F23`) & Achados Alvo:** `<lista>`
- **Hipótese Criativa / Dialética:** <isomorfismo transversal, defesa perante crítico ou condição de sobreposição a explicitar>
- **Contra-Hipótese de Severidade (Cromwell):** <o que nos faria descartar esta mudança se a fonte ou a álgebra não sustentar>
- **Plano de Subtração (Freio 2):** <quais repetições ou gorduras serão enxugadas para manter a prosa cortante>
```

---

## 5. Registro de Fontes Primárias em Duas Chaves (`source-log/round-<r>-sources.md`)

Regra férrea: `Obtido por` $\neq$ `Verificado por`.

```markdown
### [SRC-<R>.<K>] <Autor(es)> (<Ano>) — <Título>
- **Rota Primária Aberta:** `<DOI / URL editorial / PMC / arXiv / Edição Crítica>`
- **Obtido por (1ª Chave):** `<philology-archivist ou autor>`
- **Localizador Exato:** `<p. X, Tab. Y, eq. Z, ou Stephanus/Bekker/Kühn/SVF>`
- **Trecho Literal Relevante:**
  > "<citação ipsis litteris no idioma da fonte consultada>"
- **Valor(es) Reportado(s) na Precisão Original:** `<valores exatos + unidades, ou 'conceitual — sem numeral'>`
- **Verificado por (2ª Chave Cega):** `<two-key-verifier>` — `[VERIFICADO: literalidade, precisão e escopo conferem]`
- **Sincronia Apêndices (`commons-steward`):** Apêndice C `[ok]` · Apêndice D `[ok | n/a — sem alegação emprestada nova]`
```

---

## 6. Entrada de Oportunidade de Pesquisa (`opportunities/round-<r>-F-ledger.md`)

```markdown
### F<n>.<k> — <título curto>
- **Capítulos:** `<NN, NN>` · **Parceiros:** `<NN, NN>`
- **Modo:** `refinamento | defesa | expansão`
- **Pergunta:** <uma frase>
- **Fontes obtidas:** `<autor, ano, título>` — trecho literal: `"<...>"` — localizador: `<p./tab./§/Stephanus>` (`[SRC-<R>.<K>]`)
- **Fontes buscadas e descartadas:** `<autor, ano>` — motivo (`inacessível / paywall / não sustenta a hipótese`)
- **Prova algébrica (`scratch`):** `<caminho de verify_wave_*.py ou 'n/a'>`
- **Achado:** <o que a fonte/álgebra sustenta e onde cessa de valer>
- **Proposta executada:** `<corrigir | limitar | apagar | defender | expandir | nenhuma>`
- **Contratos protegidos tocados:** `nenhum` | `<AGENTS.md §I–§III/§V-9/§V-10 ou retitulação SUMMARY.md>` → `ESC-<NN>`
- **Estado:** `incorporada | descartada | adiada | escalada`
```

---

## 7. Dossiê de Escalada Arquitetural ou de Retitulação (`escalations/ESC-<nn>-<slug>.md`)

```markdown
# [ESC-<NN>] <Título da Proposta>
- **Tipo:** `Contrato Protegido (AGENTS.md §I–§III / §V-9 / §V-10)` | `Retitulação SUMMARY.md/Apêndice (diff inline)` | `Ambiguidade Livro → Núcleo`
- **Tensão / Descoberta Originária:** <descrição precisa do achado empírico, algébrico ou de sobreposição>
- **Fundamentação (Fontes Primárias & Álgebra):** <referências `[SRC-...]` e demonstração formal>
- **Opções para Decisão Humana:**
  1. *(Conservadora — dentro da arquitetura atual):* <como acomodar como condição de contorno sem tocar invariantes>
  2. *(Estrutural — requer pré-aprovação humana):* <mudança proposta com diff conceitual exato>
- **Raio de Impacto Completo (*Blast Radius*):**
  - `book-src/` capítulos: `<lista>`
  - Apêndices (`A`–`E`) e `SUMMARY.md`: `<lista>`
  - `AGENTS.md` e `.github/checks/`: `<lista>`
  - `crates/` (`stoch-core`, `stoch-abi`, `stoch-vectors`): `<lista>`
- **Status:** `AGUARDANDO DECISÃO HUMANA (nenhum arquivo protegido foi alterado)`
```

---

## 8. Layout e Modelos do Modo S (`<MAIN_ROOT>/research/runs/` — `112 × 6` Persistente)

```text
<MAIN_ROOT>/research/runs/                # Estritamente fora do Git (.gitignore) — usar caminho absoluto
├── baseline.json                         # `review_ops.py baseline --out "$MAIN_ROOT/research/runs/baseline.json"`
├── state.json                            # `review_ops.py state --init` (112 ondas W000–W111)
├── waves/Wnnn/
│   ├── blind-{s1,a1,s2,a2,s3,a3}.md      # Parecer cego Fase α (escritor único por arquivo)
│   ├── sign-{s1,a1,s2,a2,s3,a3}.md       # Assinatura individual CLEAN (quando 6× não)
│   ├── brief.md                          # Brief Fase β (escritor: wNNN-s1; falha, subtração, adiados)
│   ├── commons-delta.md                  # Entradas bibliográficas C / notas D-F (escritor: wNNN-s2)
│   ├── candidate.patch & candidate.sha256# Exportado por wNNN-s1 do share antes de audit-a*
│   ├── audit-{a1,a2,a3}.md               # STATUS + PATCH-SHA256 (escritor único por arquivo)
│   └── verdict.md                        # Selo final (escritor único: commons-steward)
├── sources/Wnnn-sources.md               # Entradas [SRC-Wnnn.K] com 1ª + 2ª Chaves
├── scratch/verify_Wnnn.py                # Prova simbólica SymPy/Python (wNNN-a2)
├── escalations/ESC-NN-<slug>.md          # Dossiês protegidos aguardando humano
└── stage-reports/stage-<1..6>.md         # Portões de fechamento e adiados por Macro-Estágio
```

### 8.1 Parecer Cego Individual (`<MAIN_ROOT>/research/runs/waves/Wnnn/blind-<seat>.md`)

```markdown
# Blind Review — Onda Wnnn · Assento <wNNN-s1|a1|s2|a2|s3|a3>
- **DEFEITO VERIFICÁVEL:** `sim | não`
- **Especialidade:** `<pairK_role>`
- **Arquivo(s) Alvo & Parceiros Nucleares lidos na íntegra:** `<lista>`

## Achados Candidatos (≤ 3; Taxonomia A–J apenas como rótulo)
- `[FIND-Wnnn.<N>]` (`<A–J>` · `<critical|high|medium|low>`) — `book-src/<arquivo>:<linhas>`: <evidência verificável e hipótese de refutação>

## Pontos Sólidos a Preservar (Anti-Subtração Indevida)
- `<teses, provas ou passagens que não devem ser enfraquecidas>`
```

### 8.2 Brief, Auditoria por Assento e Selo Final (`verdict.md` — escritor único: `commons-steward`)

- **`brief.md` (Modo S):** contém Edit Set, Hipótese dialética, Contra-hipótese de Cromwell, **Condição de falha / recorte de domínio**, **Plano de subtração (Freio 2)** e **Adiados (sem fonte aberta — sobem ao `stage-report`)**.
- **`sign-<papel>.md` / `audit-a{1,2,3}.md`:** Linha 1 `STATUS: CLEAN | APPROVED | REJECTED`; em `audit-a*`, Linha 2 `PATCH-SHA256: <sha256>`.

```markdown
# Veredito Consolidado — Onda Wnnn (<arquivo_alvo>)
- **Desfecho:** `[CLEAN / SEM EDIÇÃO]` | `[EDITED & VERIFIED]` | `[ESCALATED: ESC-NN]` | `[HALTED]`
- **Pareamento de Modelos:** `sessão (Carbon) ↔ gemini-4-hydrogen-high` | `CROSS-FAMILY: degraded`
- **Commit Local Integrado:** `<hash ou n/a>` · **Delta de Palavras (`wave-audit`):** `<+N% / 0%>`
- **Censo de Numerais (§V-7):** `<justificado em todas as linhas tocadas | intacto>`
- **Script Simbólico (`a2`):** `<research/runs/scratch/verify_Wnnn.py — PASS | n/a>`
- **Signatários verificados:** `6/6 sign-*.md` (CLEAN) ou `3/3 audit-a*.md` com `PATCH-SHA256` vigente (`commons-steward`)
```
