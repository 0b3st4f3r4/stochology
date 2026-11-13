# Review t8 — round 2 sobre o reparo de t7

Revisor: epistemic-auditor. Objeto: `t7` (reparo dos oito achados de `t6`). Nada em `book-src/` foi editado
por esta revisão; nenhum commit.

**Veredito: pass.** Os oito achados fecharam; nenhuma proposição de colapso perdeu força; nenhuma passagem
nova viola §I; citações preservadas; os portões foram lidos no código e a rodada 2 não os tocou — a única
mudança em `structure.py` é o comentário de F8, confirmada por diff contra o snapshot de `t6`.

## Aceitação, item por item

1. **Proposições de colapso com a mesma força — passa.** `book-src/41-the-triad-and-the-minimum.md:199-225`:
   P1 abre condicional ("Where an organization holds … but lacks the epistemic margin"), nomeia
   $\rho_{R,Y}\neq0$, $\mathrm{DEFF}$ e agora a largura dos limites de identificação parcial
   $\Delta_{\mathrm{id}}$, e fecha com "It fails if a trajectory carrying all three omissions is observed to
   size its buffer above the tail it later meets". P2 mantém o parágrafo-modelo e ganha cláusula
   correspondente; P3 abre condicional e ganha a sua. As três são mais fortes que a forma pré-reparo, que
   afirmava incondicionalmente e não dizia o que as refutaria.
2. **Nenhuma passagem nova viola §I — passa.** `41:348` deixou de mandar financiar: "to name the critical axis
   (the smallest one) as the one the next step will face, leaving the funding decision to the operator's own
   per-step state". O veredito descreve; a decisão fica com o operador por passo, como o último parágrafo de §I
   exige. `grep -rn "must fund\|single lever" book-src/` = zero. `22:259-263` repõe um único "and", unifica
   `$T\_0$`, e troca o universal "erasure is never the term an institution runs into" por "not the binding
   term at any institutional power draw" — o que a fórmula exibida sustenta, já que a razão entre teto de
   Landauer e potência de entrada, $T_{\mathrm{year}}/(k_B T_0 \ln 2)$, não depende de $P$. `48:376,388`
   atribuem a dependência na distância a Perdomo et al. (2020) na própria frase; `14:190-196` devolve o
   cenário ao par LaLonde (1986) / Dehejia & Wahba (1999) com Kish (1965).
3. **Citações preservadas — passa.** Delta de citações HEAD→worktree por arquivo: `05` ganha Wong (2021),
   `14` ganha LaLonde (1986) e Dehejia & Wahba (1999), `26` ganha Bernard (1878), `28` nomeia o ato;
   nenhuma perda. `67:94` mantém Chao (1987) e acrescenta Chao & Jost (2012) — a mudança de "Chao's (1987)"
   para a forma parentética é a única alteração de forma, e as duas entradas existem em
   `book-src/appendix-c-sources.md:212-213`. Kish (1965) existe em `appendix-c-sources.md:375`.
   `book-src/36-decision-loss-threshold.md` voltou a ser idêntico ao HEAD (`git status` vazio para o arquivo),
   como F6 pedia.
4. **Portões auditados no código — passa.** `diff` do snapshot de `t6` para o `structure.py` de hoje mostra
   **uma só** mudança: o comentário de `check_collapse_refutation` passou de prometer uma varredura de seção
   para declarar o limite ("a refutation carried in a later sentence of the same section … lies outside this
   gate's reach"). Nenhuma linha de código mudou nesta rodada; portanto nenhum portão foi afrouxado, e a
   declaração de `DECLARED_OPEN` continua `{}`.

## Verificação executada

    python3 .github/checks/structure.py   → exit 0, "all structural gates pass"
      census 523 (advisory) · orphans 10 · display-first 25 · near-dups 0 · cross-refs 15
      notation drift 3 (era 4: `22:262` corrigido) · declared open 0
    python3 .github/checks/wasm_gates.py  → exit 0, 18 gates: 10 decididos, 0 falhos, 8 declarados abertos
    git diff --stat HEAD                  → 37 arquivos, +914, −117

F3 (ledger): os 35 arquivos `book-src/` da árvore estão cobertos. Quatro aparecem por forma abreviada
(`00:294`, `43:252`, `45:200/202`, `book-src/85-…-urban-biospheric-order.md`), o que fecha a cobertura mas
não é verificável por máquina. A aritmética do relatório (16 de t5 + 20 da onda anterior = 36) fecha com 35
porque `36-decision-loss-threshold.md` foi reparado e depois devolvido ao HEAD em F6 — o próprio ledger diz
isso na linha de F6.

## Ressalvas não bloqueantes (low, para a próxima onda)

- `book-src/41-the-triad-and-the-minimum.md:227` — parágrafo órfão ("The configuration has governance as the
  missing axis: epistemic uncertainty weaponised, common vital reserves captured under limited liability.")
  sem antecedente, imediatamente após §3 dizer que a frequência é questão empírica separada. Redação: dizer
  que se trata de uma configuração ilustrativa, ou remover.
- `audit/repair-report.md:157` — atribui ao portão `citations` a verificação da "precisão da fonte"; o portão
  resolve sobrenome+ano contra o Appendix C e não compara valores. A frase deve dizer o que o portão faz.
- `book-src/44-underdetermination-partial-identification.md:192` — "an estimate of the opposite sign and
  larger magnitude" segue sendo magnitude em palavra, ainda que defendida pela decisão explícita de
  `44:196` ("the numbers stay in the cited papers rather than in this chapter"). Registrar como escolha, não
  como garantia.
- `audit/repair-report.md` usa caminho com elipse (`85-…`) e `capítulo:linha` no lugar do caminho completo;
  a cobertura existe, a checagem mecânica dela não.
