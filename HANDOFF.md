# Relatório de passagem — sessão remota → Claude Code local

Gerado em 2026-10-30. Repositório: `0b3st4f3r4/stochology`. Este arquivo está no branch `ccr-053feae1-simg5j` só para continuar a tarefa: apague-o antes de levar o branch para o `main`.

## 0. Regras que valem para qualquer commit daqui em diante

- **Proibido** colocar nome ou email do Claude em commits e PRs: nada de `Co-Authored-By: Claude …`, `noreply@anthropic.com` ou linha `Claude-Session:`.
- Autor e committer: `Silvano Neto <dev@silvanoneto.com>`.
- No Claude Code local, desligue a atribuição: em `.claude/settings.json` ou `~/.claude/settings.json`, use `"includeCoAuthoredBy": false` ou `"attribution": {"commit": "", "pr": ""}`, conforme a versão.
- Hook local opcional, que barra commits com coautoria do Claude (não foi instalado na sessão remota):
  ```bash
  cat > .git/hooks/commit-msg <<'EOF'
  #!/bin/sh
  grep -qiE 'co-authored-by:.*(claude|anthropic)|noreply@anthropic\.com' "$1" && { echo "Commit com coautoria do Claude bloqueado."; exit 1; }
  exit 0
  EOF
  chmod +x .git/hooks/commit-msg
  ```
- Siga o `AGENTS.md`: `scripts/verify.sh` precisa passar antes de consolidar. Mudanças em `data/SHA256SUMS`, `cards/*.yaml` e `vault.rs` exigem revisão humana.

## 1. Estado atual do GitHub (verificado no fim da sessão)

| Ref | Aponta para | Observação |
| --- | --- | --- |
| `main` | `67e7ad2` | Histórico reescrito, sem as 3 linhas `Co-Authored-By: Claude`. Árvore idêntica à do antigo `aec784a`. |
| `ccr-053feae1-simg5j` | `f0c0313` | 3 commits à frente do `main` (ver §2). Sem PR aberto. |
| tag `v1.0.0` | **ainda o antigo `aec784a`** (objeto `4b059a0`) | **Pendente**: a sessão remota não conseguiu enviar tags (o servidor encerrava a conexão). Ver §3.1. |

Workflow `Publish Crates to crates.io & GitHub Release` (`release.yml`): o usuário informou que o desativou, mas a API ainda o mostrava `active` na última consulta. Confira na interface antes de enviar a tag.

## 2. O que foi feito no branch `ccr-053feae1-simg5j`

1. `cdf0929` — `docs/SYLLABUS.md` (ementa: 5 Movimentos × 3 tempos, trilhas de 30 h e 90 h) e `docs/RESTRUCTURING_PLAN.md`.
2. `31a7505` — Reestruturação, fases 0–4 do plano (o registro completo está no §7 do plano):
   - **Protocolo:**
     - `docs/PROTOCOL.md` reorganizado: §1 Núcleo, §2 Lentes, §3 Seis Pares (com o mapa par → símbolos → gates → provas → capítulos), §4 Operação, §5 Registro das sínteses, §6 Arquitetura.
     - As 7 sínteses foram para `docs/syntheses/*.md`.
     - `AGENTS.md` §III: Nível 1 mais o registro de Nível 3 (§III.B).
   - **Livro (27 capítulos, 7 partes, Apêndices A–E):**
     - Prefácio §3 → cap. 2 §2.1.
     - Gerstgrasser → cap. 19 §19.3.
     - Cap. 24 novo (Estratégica) e cap. 25 novo (Sociedade, promovido do §22.7).
     - Antigos 24–25 → 26–27; antigos 26–28 → `appendix-{a,b,c}-*.md`; Apêndice E inclui `docs/SYLLABUS.md`.
     - Bloco "Three tempos" no início de cada capítulo.
     - Atualizados: `book.toml` (redirecionamentos), `lab/index.html`, `README.md`, `.agents/`.
   - **`scripts/verify.sh`:**
     - Lista negra de exageros estendida a `docs/SYLLABUS.md` e `docs/syntheses/`.
     - Checagem dos links de apêndice no `SUMMARY.md`.
     - Exigência do bloco de três tempos.
     - Checagem dos alvos de `{{#include}}`.
3. `f0c0313` — Checagem de publicação no CI: `scripts/release_guard.sh`, chamado pelo `release.yml`.
   - Classifica cada crate como `new`, `identical` ou `conflicting` em relação ao crates.io.
   - O passo de publicação só envia as `new`.
   - Em conflito, faz yank (dependentes primeiro) se autorizado e depois falha pedindo aumento de versão.
   - Autorização: input `yank_on_conflict` no disparo manual, ou variável de repositório `STOCHOLOGY_YANK_ON_REWRITE=true` no push de tag.
   - O usuário informou que o `CARGO_REGISTRY_TOKEN` já tem permissão de yank.
   - Testado localmente contra a v1.0.0 publicada: as 5 crates deram `identical`; uma alteração temporária deu `conflicting`, e o modo de simulação respondeu "would yank".
   - Nunca rodou no GitHub.

`scripts/verify.sh` passou de ponta a ponta depois de `31a7505`: 8 gates com exit 0, 21 provas negativas com exit 1, `book_snapshot_parity` verde. Ele **não** foi rodado de novo depois de `f0c0313` (o commit só mexe em CI e em um script novo).

## 3. Pendências, em ordem

### 3.1 Enviar a tag `v1.0.0` reescrita (você mesmo, localmente)
Com o workflow de release **desativado** (a opção deve aparecer como "Enable workflow"):
```bash
git fetch origin
git rev-parse origin/main   # 67e7ad2daf1ae868323e3a8c4bc530ab84785d9d
git tag -f -a v1.0.0 67e7ad2daf1ae868323e3a8c4bc530ab84785d9d -m "Stochology Protocol v1.0.0"
git push --force origin refs/tags/v1.0.0
git ls-remote origin 'refs/tags/v1.0.0*'   # a linha ^{} deve mostrar 67e7ad2…
```
Observação: um push de tag executa o workflow na versão do commit da tag (`67e7ad2`, workflow **antigo**, sem a checagem nova). Se ele rodar por engano, o esperado é falhar no `cargo publish` com "already exists", sem publicar nada.

### 3.2 Limpezas no GitHub
- Confirmar em *Releases* que a v1.0.0 continua ligada à tag.
- Reativar o workflow de release.
- Bloquear de novo o force push no `main` (*Settings → Rules/Branches*).
- Lista de contribuidores: esperar até cerca de 24 h. Se o Claude ainda aparecer, abrir chamado em support.github.com pedindo a limpeza do cache de contribuidores e dos commits soltos.
- Clones antigos em outras máquinas: `git fetch && git reset --hard origin/main` (apaga mudanças locais não commitadas).

### 3.3 Levar o branch para o `main`
- Atualize o clone local e mude para o branch: `git fetch origin && git switch ccr-053feae1-simg5j`. Ele já está sobre o novo `main`, sem as linhas do Claude.
- Rode `scripts/verify.sh` de novo.
- Teste `mdbook build` (versão 0.5.3, a do workflow de Pages). Não foi testado: a sessão remota não pôde baixar o mdbook. Pontos de atenção: `{{#include ../docs/SYLLABUS.md:3:}}` em `book-src/appendix-e-syllabus.md`, a parte "Appendices" no `SUMMARY.md` e os redirecionamentos do `book.toml`.
- Abrir PR ou fazer merge no `main`, sem atribuição ao Claude no corpo do PR.

### 3.4 Decisão em aberto: fase 5 do plano
"Fase 5" = mover arquivos de código para espelhar a nova estrutura, o que obriga a refazer os selos (`data/preregistration/acceptance_registry.txt` e `data/SHA256SUMS`) com revisão humana.

A resposta "de forma alguma" à pergunta "a fase 5 fica de fora?" ficou ambígua. **Nada da fase 5 foi feito.** Recomendação registrada no plano: não fazer, porque a reestruturação é só documental.

### 3.5 Lacunas conhecidas da reestruturação (§7 do `docs/RESTRUCTURING_PLAN.md`)
- Os capítulos antigos ganharam só o bloco de três tempos; o corpo não foi redividido em seções de Desapego, Abstração e Retorno.
- A tabela de procedência ("Provenance Ledger") existe apenas nos caps. 24 e 25.
- Os módulos de `book-src/lab/index.html` sobre a síntese estratégica (Causal SHAP, Saaty, PPI, Tube-MPC, Hub Contagion) ainda apontam para os caps. 11, 15, 16, 19 e 22, e não para o cap. 24.

## 4. Comandos úteis

```bash
scripts/verify.sh                                        # verificação completa (cerca de 10 min)
cargo run --release --bin gates                          # 8 gates, exit 0
scripts/release_guard.sh                                 # compara a versão do workspace com o crates.io
YANK_ON_CONFLICT=true DRY_RUN=true scripts/release_guard.sh   # simula o yank sem token
git log origin/main --format='%an <%ae>%n%B' | grep -ci 'claude\|anthropic'   # deve dar 0
```
