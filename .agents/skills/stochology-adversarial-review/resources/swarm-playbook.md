# Playbook de Orquestração Federalizada (`swarm` Nativo + `run_workflow`)

Este guia detalha como operar os especialistas federalizados mantendo a **Estocologia do próprio enxame** ($S\_{\mathrm{epi}}^{\mathrm{swarm}}, S\_{\mathrm{vital}}^{\mathrm{swarm}}, S\_{\mathrm{gov}}^{\mathrm{swarm}} > 0$) com consumo mínimo de cota/contexto e rigor máximo de duas chaves.

> [!NOTE]
> **Para revisão geral atômica do livro inteiro (`Modo S — 112 Ondas × 6 Agentes`):** consulte a constituição mestre [research/guide.md](../../../research/guide.md) e a Seção 5 abaixo para o protocolo em 4 fases ($\alpha$–$\delta$), escalonamento elástico via `review_ops.py next-batch` e pareamento assimétrico `sessão (Carbon) ↔ gemini-4-hydrogen-high`.

---

## 1. Arquitetura Híbrida Recomendada

Combine as primitivas da plataforma segundo o regime cognitivo de cada etapa:

1. **Triagem Mecânica Determinística (`scripts/review_ops.py`):** Custo zero de tokens. Gera linha de base (`baseline`), plano dos 9 fragmentos (`shards`), vizinhança do grafo $J(7,3)$ (`graph`), parceiros de onda e verificação de disjunção (`wave-plan`), e auditoria pós-edição de censo/subscritos/inchaço (`wave-audit`).
2. **Passo 1 — Cartografia Integral em 9 Fragmentos (`run_workflow` com `parallel`):** Para ler os 129 arquivos (`~353k` palavras) sem saturar a janela de um único agente nem inundar o quadro do `swarm` com 129 postagens, execute um fan-out `parallel` de 9 cartógrafos especializados (cada um lendo ~9–25 arquivos / ~15k–66k palavras e gravando seu arquivo em `<artifactDir>/stochology-review/book-map/S*.md`), seguido de 1 agente sintetizador que constrói `cross-part-synthesis.md` e audita o Apêndice D (na Rodada 1).
3. **Passos 2 a 6 — Ondas Dialéticas, Duas Chaves e Red Team (`swarm` Nativo):** Onde o trabalho exige diálogo entre especialistas, busca de fontes primárias, autoria cirúrgica em cópias isoladas (`workspace:"share"`), verificação cega de duas chaves e custódia única dos Apêndices C/D/F, utilize o `swarm` nativo com guildas federalizadas.

---

## 2. Passo 0 e Passo 1: Linha de Base + Cartografia Federalizada (`run_workflow`)

Antes de abrir qualquer onda:

```bash
python3 .agents/skills/stochology-adversarial-review/scripts/review_ops.py baseline \
  --out "<artifactDir>/stochology-review/baseline.json"
python3 .agents/skills/stochology-adversarial-review/scripts/review_ops.py shards \
  --format json --out "<artifactDir>/stochology-review/shards.json"
```

Em seguida, dispare a cartografia paralela via `run_workflow` (ajuste `ARTIFACT_DIR` para o diretório da conversa atual):

```python
import json
from pathlib import Path

ARTIFACT_DIR = args["artifact_dir"]
shards = json.loads(Path(f"{ARTIFACT_DIR}/stochology-review/shards.json").read_text())

phase("Step 1A — 9-Shard Federated Cartography")

def map_shard(s):
    out_file = f"{ARTIFACT_DIR}/stochology-review/book-map/{s['id']}.md"
    files_list = ", ".join(f"book-src/{f}" for f in s["files"])
    return agent(
        prompt=(
            f"Read AGENTS.md and .agents/skills/stochology-adversarial-review/resources/artifact-templates.md.\n"
            f"You are the Federated Cartographer for shard {s['id']} ({s['guild_lead']}).\n"
            f"Read EVERY file in full in reading order: {files_list}.\n"
            f"Write compact Book Map cards (claims+warrants, symbols introduced/consumed, Bridge, "
            f"laminar tensions A-G, superposed tensions H-J, and F1-F23 tags) to {out_file}. "
            f"Never write anything inside the git repository."
        ),
        schema={
            "type": "object",
            "properties": {
                "shard_id": {"type": "string"},
                "mapped_files": {"type": "integer"},
                "high_or_critical_candidates": {"type": "array", "items": {"type": "string"}},
            },
            "required": ["shard_id", "mapped_files", "high_or_critical_candidates"],
        },
        role=f"Cartographer {s['id']}",
    )

shard_results = await parallel(shards, map_shard)
log(f"Mapped {sum(r['mapped_files'] for r in shard_results)} manuscript files across 9 shards.")

phase("Step 1B — Cross-Part Graph & Ranked Findings Synthesis")
synthesis = await agent(
    prompt=(
        f"Read AGENTS.md, all 9 shard files in {ARTIFACT_DIR}/stochology-review/book-map/, "
        f"and run `python3 .agents/skills/stochology-adversarial-review/scripts/review_ops.py graph`.\n"
        f"Synthesize {ARTIFACT_DIR}/stochology-review/book-map/cross-part-synthesis.md containing:\n"
        f"1. Cross-part dependency & concept-drift graph (including Part V J(7,3) sibling gaps);\n"
        f"2. Severity-ranked findings [FIND-R.N] across taxonomy (A)-(J) (laminar + superposed);\n"
        f"3. Candidate disjoint 2-3 chapter waves clustered by deep graph relation and F1-F23 fronts."
    ),
    schema={"type": "string"},
    role="Cross-Part Graph Synthesizer",
)
print(synthesis)
```

*(Na Rodada 1, adicione em paralelo uma tarefa dedicada do `philology-archivist` + `two-key-verifier` para auditar as linhas existentes de `book-src/appendix-d-claim-ledger.md` contra os textos primários conforme `AGENTS.md` §V-6.)*

---

## 3. Iniciando o Enxame Federalizado (`swarm` Nativo)

Ao criar o `swarm` para executar as ondas da rodada (seguindo estritamente a skill `swarm`):

1. **Dê título ao enxame:** `swarm{action:"title", title:"Stochology Federated Review — Round R"}`.
2. **Use prefixos `[Wnnn]` no quadro único (ou canais se `channel` estiver disponível)** mantendo notificações estritamente `@peer` (proibido `@all` para status e `@coordinator` em handoffs intermediários).
3. **Adicione `narrator` (somente se `swarm_narrator` estiver na lista de skills) e `monitor` (somente se `swarm_monitor` estiver na lista de skills)** exatamente no turno de criação, deixando `model` em branco (herda o modelo da conversa).
4. **Adicione apenas os especialistas necessários para as ondas ativas** (enxame enxuto: `waiting` custa zero tokens, mas cada agente criado recebe brief público). Deixe `model` omitido a menos que o usuário peça outro modelo.

### Briefs-Modelo por Especialista Federalizado

#### A. `commons-steward` (`workspace:"inherit"` — Custódio de Apêndices C/D/F e Integrador Permanente)
```text
You are `commons-steward`, sole custodian of `book-src/appendix-c-sources.md`, `book-src/appendix-d-claim-ledger.md`, and `book-src/appendix-f-manuscript-audit.md`, sole writer of `verdict.md`, and wave integrator in `workspace:"inherit"` (main tree).
First read `AGENTS.md`, `.agents/skills/stochology-adversarial-review/references/swarm-stochology-charter.md`, and `<MAIN_ROOT>/research/runs/baseline.json` (or `<artifactDir>/stochology-review/baseline.json`).
Rules:
1. Nobody else edits Appendix C, Appendix D, or Appendix F. Integrate `commons-delta.md` entries into Appendix C alongside `cherry-pick`/`git am` in Mode S Fase δ ONLY AFTER verifying `verdict.md` is absent and all 3 `audit-a{1,2,3}.md` match `candidate.sha256` (`STATUS: APPROVED`), keeping §V-6 ledger parity and §II authority (no appendix cites a chapter as authority). Batch D/F updates at Macro-Stage boundaries.
2. When a wave finishes verification, integrate serially (`git cherry-pick` or `git am candidate.patch`), run `python3 .agents/skills/stochology-adversarial-review/scripts/review_ops.py wave-audit --files <wave_files> --baseline <MAIN_ROOT>/research/runs/baseline.json`, write `verdict.md`, update `state.json` (serialized by `flock`), and report `@coordinator [done]`.
3. Never write working logs inside the git repository. Raise `@coordinator [ALGEDONIC HALT]` immediately on any provenance, architectural, or gate violation.
```

#### B. `philology-archivist` (`workspace:"inherit"` — Genealogia Clássica e Obtenção Primária, 1ª Chave)
```text
You are `philology-archivist` (1st Key: Classical Genealogy & Primary-Source Retrieval).
First read `AGENTS.md` (§IV) and `.agents/skills/stochology-adversarial-review/resources/artifact-templates.md`.
Rules:
1. Web search is ONLY for discovering DOIs/editions. Use `literature-search-openalex`, `literature-search-arxiv`, `literature-search-europepmc`, `pubmed-database`, DOI/Crossref, publisher open full texts, or critical editions (Stephanus, Bekker, Littré, Kühn, SVF) to read the primary text itself.
2. Log every retrieved source in `<artifactDir>/stochology-review/source-log/round-R-sources.md` using `[SRC-R.K]` with verbatim passage, exact locator, and exact source precision. Log inaccessible/paywalled/refuting sources as discarded in `round-R-F-ledger.md` — never guess from memory or abstracts.
3. Hand off by `#seq` doorbell directly to the wave author and `@two-key-verifier` (never copy-paste long prose onto the board).
```

#### C. Autores Especialistas de Guilda (`epi-methodologist`, `vital-cyberneticist`, `gov-ethicist` — `workspace:"share"`)
```text
You are `<guild-name>` (Wave Author in your isolated `share` workspace).
First read `AGENTS.md` in full, `.agents/skills/stochology-adversarial-review/references/swarm-stochology-charter.md`, and `.agents/skills/stochology-adversarial-review/resources/artifact-templates.md`.
For your assigned wave:
1. Run `python3 .agents/skills/stochology-adversarial-review/scripts/review_ops.py wave-plan --edit <chapters>`. Read the target chapters (2–3 max) and all `core_relation_partners_full_read` in full.
2. Phase W1 (Ars Inveniendi): Co-draft `<artifactDir>/stochology-review/waves/wave-R-W-brief.md` with `@counter-inductive-critic` (testing laminar A–G + superposed H–J tensions, J(7,3) bridges, strongest named critics, and subtraction targets).
3. Phase W2 (Ars Judicandi): Edit ONLY your assigned `edit_set` chapters (never Appendix C/D, never `SUMMARY.md` titles or Section 1 architectural commitments without human pre-approval — escalate those via `ESC-NN`). Preserve the exact chapter spine (3 outcomes, NO `## Exercises`, 1 `## Bridge`, escaped `\_` in inline math, inline SVG masthead intact). Every quantitative claim carries same-sentence `(Author Year)` from a logged `[SRC-R.K]` at exact precision OR side-by-side algebra with explicit domain/failure conditions.
4. Run `review_ops.py wave-audit` in your workspace, commit locally in Portuguese (one commit per coherent repair), and ring `@two-key-verifier` with commit hash + artifact paths (`#seq`). Never push.
```

#### D. `counter-inductive-critic` (`workspace:"inherit"` — Abdução Criativa, Sobreposição & Red Team)
```text
You are `counter-inductive-critic` (S4 Creative Abduction, Superposition & Red-Team Challenger).
First read `AGENTS.md` and `.agents/skills/stochology-adversarial-review/references/swarm-stochology-charter.md` (especially F15–F23).
Rules:
1. During Wave Conception (Phase W1): challenge the wave author with simultaneous guard superposition (T1..T7 under Zheng & Meister's ~10 bits/s limit), strongest external critics (Harsanyi, Samuelson, Sunstein, Gigerenzer/Haldane, Molinari), cross-stratum J(7,3) links, reception basin alterity, and Canguilhem's creative normativity — while enforcing Swarm Brake 2 (cut defensive bloat; never turn Stochology into 'Basel IV').
2. If an insight touches protected contracts (`AGENTS.md` §I–§III, §V-9, §V-10) or `SUMMARY.md`/appendix retitlings, channel it into `<artifactDir>/stochology-review/escalations/ESC-NN-<slug>.md` with full blast radius.
3. At Round Close (Step 6): audit every round diff against `AGENTS.md` and the Charter before the round report is sealed.
```

#### E. `two-key-verifier` (`workspace:"inherit"` — Segunda Chave Cega: Fontes + Álgebra Simbólica)
```text
You are `two-key-verifier` (Independent Blinded 2nd Key — Citation & Formal Math Auditor).
First read `AGENTS.md` (§IV–§V) and `.agents/skills/stochology-adversarial-review/resources/artifact-templates.md`.
Rules:
1. Two-key separation is absolute: you NEVER author manuscript prose and NEVER verify a citation you retrieved yourself.
2. For every wave diff:
   (a) Open each cited primary source in `[SRC-R.K]` independently and verify verbatim wording, canonical locator, unit, exact numeric precision, and conceptual fidelity (zero overclaim);
   (b) Write and run `<artifactDir>/stochology-review/scratch/verify_wave_R_W.py` (`python3`) to check every modified/added equation, derivative, integral, limit positivity condition (`ratio-limit`), and inequality;
   (c) Check every numeral census line reported by `review_ops.py wave-audit`.
3. If anything fails, send exact defects back to `@<wave-author>` (or raise `@coordinator [ALGEDONIC HALT]` on fabricated/unread citations or architectural violations). When 100% clean, sign `[SRC-R.K]` and ring `@commons-steward` by `#seq` to integrate.
```

---

## 4. Regras Críticas de Sobrevivência Operacional no `swarm`

- **Aterre antes de encerrar (`kill`):** `kill` apaga permanentemente um workspace `share`/`branch`. Confira `git cherry main <ramo>` sem linhas `+` antes de encerrar qualquer autor de onda (`wNNN-s1..a3`). **Nunca encerre `monitor` nem `commons-steward` entre ondas/lotes.**
- **Commit prévio e caminhos absolutos:** todo `share` nasce de `HEAD` de `main` e não contém `research/runs/`. Comite antes em `main` e passe `<MAIN_ROOT>/research/runs/...` absoluto para leitura, escrita e `--baseline`.
- **Reutilize guildas adormecidas:** um especialista em `waiting` custa zero tokens.
- **Troca de modelo sob cota baixa:** use `swarm{action:"model", name:"<membro>"}` enquanto estiver `waiting` apenas para aterrar ondas em curso ou preservar famílias distintas, mantendo o histórico da conversa.
- **Sincronização do Núcleo (`crates/`):** apenas quando o texto integrado alterar produções, predicado, dobra, sítios ou Tríade (`Livro -> Núcleo`), acione `core-maintainer` (`workspace:"share"`) e rode `review_ops.py gates --with-core`.

---

## 5. Playbook do Modo S (`112 × 6` — Uma Onda por Capítulo com Pareamento Carbon ↔ Hydrogen)

Consulte [research/guide.md](../../../research/guide.md) para a matriz completa das 112 ondas (`W000`–`W111`) e cartões de papel. O ciclo operacional determinístico é:

### 5.1 Inicialização e Despacho de Lotes (`<MAIN_ROOT>/research/runs/`)

```bash
MAIN_ROOT="$(git rev-parse --show-toplevel)"
git -C "$MAIN_ROOT" status --porcelain  # deve estar limpo em book-src/ e research/guide.md
python3 .agents/skills/stochology-adversarial-review/scripts/review_ops.py baseline --out "$MAIN_ROOT/research/runs/baseline.json"
python3 .agents/skills/stochology-adversarial-review/scripts/review_ops.py state --init

# Consultar cota viva via swarm{action:"models"} e marcar Wnnn:active ANTES de adicionar workers
python3 .agents/skills/stochology-adversarial-review/scripts/review_ops.py next-batch \
  --k 4 --quota-gemini <q_gemini>
```

### 5.2 Orquestração no Quadro Único (`[Wnnn]`) e Célula de 6 Especialistas
1. Mantenha `commons-steward` (`workspace:"inherit"`, permanente) e `monitor` (permanente até `K=0` ou fim da sessão, mais `narrator` apenas se `swarm_narrator` estiver nas skills). Use prefixo `[Wnnn]` no quadro único e `@peer` estrito (só use canais dedicados se uma versão futura expuser `channel`).
2. Adicione os 6 membros usando os nomes e especialidades de `review_ops.py next-batch` (omitindo `model` nos assentos `s*` para herdar a sessão — nunca passe `"inherit"` literal — e atribuindo `gemini-4-hydrogen-high` nos assentos `a*`, sem `claude-opus-*`):
   - `wNNN-s1` (`workspace:"share"`, `model` omitido — Relator-Autor Único);
   - `wNNN-a1` (`workspace:"inherit"`, `model:"gemini-4-hydrogen-high"` — Adversarial Substantivo);
   - `wNNN-s2` (`workspace:"inherit"`, `model` omitido — Rigor Formal & 1ª Chave de Fontes);
   - `wNNN-a2` (`workspace:"inherit"`, `model:"gemini-4-hydrogen-high"` — 2ª Chave Cega & `verify_Wnnn.py`);
   - `wNNN-s3` (`workspace:"inherit"`, `model` omitido — Transversalidade, $J(7,3)$ & Voz);
   - `wNNN-a3` (`workspace:"inherit"`, `model:"gemini-4-hydrogen-high"` — Contraindutivo $\bigoplus T\_j$ & Freio 2).
3. Exija barreiras Write-Before-Check em 4 fases (`Fase α` com `DEFEITO VERIFICÁVEL: sim|não` → `Fase β` com saída rápida `6× não` para `CLEAN`, `ESC-NN` ou `brief.md` → `Fase γ` com `commons-delta.md` de `s2`, edição única de `s1`, exportação de `candidate.patch` + `candidate.sha256`, tripla auditoria `audit-a1..a3.md` com teto de 2 revisões/retorno `halted` → `Fase δ` serial pelo `commons-steward` conferindo os 3 hashes, aplicando `C` + *cherry-pick*, gravando `verdict.md` e `state.json`).
4. **Somente após confirmar `git cherry main <ramo>` e `state.json` atualizado**, encerre apenas `wNNN-s1..a3` (`kill` — jamais `monitor` ou `steward`) e consulte `swarm{action:"models"}` antes do lote seguinte.
5. Ao concluir cada Macro-Estágio (com todas as ondas `active` encerradas), consolide `D/F` em lote, rode `review_ops.py gates --full-render --with-pdf` (e `--with-core` no Estágio 6 se o núcleo mudou) e grave `<MAIN_ROOT>/research/runs/stage-reports/stage-<N>.md`.
