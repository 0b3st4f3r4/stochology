# `stoch-abi` — a porta linear do núcleo

Este crate é a fronteira binária descrita no documento de desenho arquivado
(`02-arquitetura.md` §2 e §8.1), construída sobre o `stoch-core` já entregue. Não é uma segunda implementação: os
seis regimes, os quatro veredictos e a dobra diagnóstica são os do núcleo, e o crate
só faz três coisas — lê os registos `#[repr(C)]`, reconstrói os objetos selados do
núcleo com eles, e escreve os registos de saída.

**Fase 4a, não ativação.** O plano (arquivado fora do repositório) exigia consumidores
reais para ativar a fase; os dois consumidores deste crate são dublês assumidos. O
artefacto está pronto para quando existirem. `02-arquitetura.md` e
`03-cadeia-de-suprimentos.md` são documentos internos de projeto e não fazem parte
deste repositório; as referências ficam como registro do desenho, e nada aqui depende
de abri-los.

## Superfície fechada

Cinco funções e uma memória, nada mais (`02` §2.1):

| Exportação | Assinatura | Entrada | Saída |
| :--- | :--- | :--- | :--- |
| `step` | `(i32, i32) -> i32` | `StateRec*` (256 B) | `RegimeOut*` (64 B), devolve o tag do regime |
| `accept` | `(i32, i32) -> i32` | `StateRec*` | `VerdictOut*` (64 B), devolve o tag do veredicto |
| `diagnose` | `(i32, i32, i32) -> i32` | `StepTriple*`, `len` | `DiagnosisOut*` (48 B) |
| `vectors_hash` | `(i32) -> i32` | `u8[32]*` | escreve o digest do fixture, devolve `0` |
| `abi_version` | `() -> i64` | — | `1` |

O passo de arena do chamador é **320 bytes** (`ARENA_STEP_BYTES`): 256 de `StateRec`
mais 64 de `RegimeOut`, sem sobreposição — `size_of::<StateRec>() +
size_of::<RegimeOut>() == 320` é asserção de compilação, não comentário.

Onde essa arena pode ficar é decidido pelo módulo, não pelo chamador: qualquer intervalo
que toque no armazém estático do módulo — o buffer de 65 536 bytes que `diagnose` usa para
emprestar a trajetória — é recusado com `-14` antes de uma única leitura. `store_span()` e
`store_overlaps()` expõem o mesmo cálculo que o portão faz, para que um adaptador possa
verificar a sua colocação em vez de a supor.

## Layouts

Cada campo, tamanho e offset é asserção `const` contra `02` §2.2 e §8.1
(`core::mem::offset_of!` no topo de `src/lib.rs`). Se uma linha do documento mudar, o
crate deixa de compilar em vez de passar a mentir.

| Registo | Tamanho | Alinhamento | Campos |
| :--- | ---: | ---: | :--- |
| `StateRec` | 256 | 8 | `instrument` 0, `missing_slot` 4, `l` 8, `u` 16, `c` 24, `b` 32, `r` 40, `tts` 48, `ttr_eff` 56, `n_eff` 64, `defect` 72, `reversible` 80, `gov_corrupt` 84, `vital_floor` 88, `gov_floor` 92, `tape` 96, `site` 100, `accept_threshold` 104, `band_width` 112, `guard[7]` 120, `budget` 176, `elapsed` 184, `proposition` 192, `step_index` 196, `reserved1[56]` 200 |
| `RegimeOut` | 64 | 8 | `payload_0` 0 (fração), `payload_1` 8 (piso / slot / causa), `payload_2` 12, `reserved[48]` 16 |
| `VerdictOut` | 64 | 8 | `proposition` 0, `site` 4, `margin` 8, `at_step` 16, `reserved[40]` 24 |
| `StepTriple` | 32 | 8 | `s_epi` 0, `s_vital` 8, `s_gov` 16, `absorbed` 24, `reserve` 28 |
| `DiagnosisOut` | 48 | 8 | `binding_axis` 0, `absorbed` 4, `s_epi` 8, `s_vital` 16, `s_gov` 24, `ruin_step` 32, `terminal_reserve` 40 |

## Códigos de retorno

O canal de erro é o `i32` de retorno (`02` §2.3). `0 ..= 5` é um regime, `0 ..= 3` é um
veredicto ou um eixo de ligação, e os negativos são recusas:

* `-1 ..= -12` — os códigos do núcleo, verbatim de `Error::code()`;
* `-14` — `E_CALLER_CONTRACT`: ponteiro nulo, desalinhado ou fora da memória do
  módulo; `len` negativo ou acima de `MAX_STEPS`; um tag ou flag fora do conjunto
  fechado do documento; um passo absorvido que carrega leitura vital; uma leitura no
  slot que o próprio registo declara ausente;
* `-5` — `E_SCALE_OVERFLOW` — usado também quando uma leitura de guarda está fora de
  `-1 ∪ 0 ..= SCALE`, porque uma margem fora da escala não é uma margem grande.

**Nenhuma exportação entra em pânico e nenhuma devolve sucesso com estado parcial.**
Todo o ponteiro, flag e campo de enchimento é validado antes de o núcleo ser chamado;
cada registo de saída é montado por inteiro antes de uma única escrita; em qualquer
retorno negativo a região de saída fica intacta.

## Os três portões que a porta acrescenta

1. **Portão de campo.** Um tag, uma flag ou um slot fora do seu conjunto fechado é
   recusado com `-14` em vez de ser coagido. Em particular `guard[i]` tem de ser `-1`
   (ausente) ou uma margem em `0 ..= SCALE`.
2. **Portão de alcance.** Um ponteiro cujo intervalo não está inteiramente dentro da
   memória do módulo é recusado antes de ser lido — no wasm32 a memória está a uma
   instrução de distância, e um `len` truncado não pode levar o chamador para fora da
   sua própria arena.
3. **Portão de armazém.** Um intervalo que intersecta o armazém estático do próprio
   módulo — o buffer de 65 536 bytes que `diagnose` empresta à trajetória — é recusado
   com `-14`, para leitura e para escrita, nos quatro pontos de entrada que recebem
   ponteiros. Sem este portão, uma arena colocada no piso que o documento publicava
   *passava*: alinhada, dentro da memória, e o `step` escrevia o resultado do chamador
   por cima das leituras do módulo, ou lia as leituras do módulo como registos do
   chamador. O modo de falha era silencioso, e é a razão de o portão existir em vez de
   um aviso. A fronteira vem de `store_span()` — `addr_of!` sobre o armazém —, não de um
   número medido: a arena do chamador **não pode** sobrepor o armazém, e o módulo recusa
   em vez de confiar num piso.

### Os eixos derivam-se dos sete sítios

`accept` recebe o estado, não três margens por eixo: `02` §2.2 não tem coluna de eixo.
A leitura de um eixo é a mais fraca dos seus sítios e a contagem das que existem — que
é a definição da dobra (`01` §3.3), não uma regra nova. Sítio ausente (`-1`) não conta.
Um registo que declara o instrumento incompleto **e** dá leitura ao slot ausente é
recusado com `-14`: estaria a declarar uma medição que também declara não ter feito.

## A separação em `diagnose`

O consumidor não recebe nada que possa voltar como argumento de `step`: `DiagnosisOut`
não tem campo de `StateRec`, nenhum regime, nenhum conjunto de acção, e não existe
conversão em qualquer dos sentidos. A separação é do tipo, não da convenção.

O portão de leitura selada que `Trajectory::seal` não faz (é `const fn` sobre um
empréstimo, e um empréstimo não pode ser recusado) **existe na porta**: cada uma das
três leituras de cada triplo é verificada contra `-1 ∪ 0 ..= SCALE` antes de a dobra
correr. Um passo absorvido tem de trazer `s_vital = -1`, porque depois da absorção a
coordenada vital é barreira e não número — um valor ali seria uma medição que a
barreira não pode ter, e o núcleo limitar-se-ia a ignorá-la.

A dobra precisa das leituras como uma só fatia e este crate não tem alocador: usa um
buffer estático de `MAX_STEPS = 1024` passos. Um comprimento maior é recusado com
`-14`, nunca truncado — uma dobra truncada é outra dobra e nomearia o passo errado.

## Divergências registadas (não se ajusta o documento)

| # | Divergência | Razão |
| :--- | :--- | :--- |
| D1 | `crate-type = ["cdylib", "rlib"]`, não só `cdylib` | `cargo test -p stoch-abi` liga o crate a binários de teste no host; uma lib só-`cdylib` não é ligável como dependência Rust. O artefacto enviado continua a ser o `.wasm` do `cdylib`. |
| D2 | `no_std` apenas no alvo enviado (`cfg(all(not(test), target_arch = "wasm32"))`) | A build de host existe só para os testes; liga `std` e não precisa de `#[panic_handler]`. Em wasm32 o handler vem do `stoch-core`. |
| D3 | `vectors_hash` escreve os 32 bytes no ponteiro e devolve `0` | §2.1 fixa `(i32) -> i32`: 64 bits não cabem num `i32`. Os primeiros 8 bytes que o adaptador lê são o token do handshake; truncar o digest na fronteira seria perder o resto sem razão. |
| D4 | O registo não tem campo de contingência | §2.2 não o define e `accept` nunca o lê (é o que o teste de não-comutação substitui). A porta usa `ContingencyTape::new(0)`. |
| D5 | `MAX_STEPS = 1024` passos por chamada | Sem alocador, a fatia tem de ser estática e portanto limitada. Recusa declarada, não truncagem. |
| D6 | `__data_end` e `__stack_pointer` **não** são exportados nesta build; e a arena do chamador **não pode** sobrepor o armazém estático do módulo | Exportá-los exige flags de linker (`-C link-arg=--export=...`) que teriam de ser globais do workspace; a build aceite é o comando simples. Consequência para §8.1: o adaptador não pode calcular `align8(__data_end)` a partir da superfície exportada — e, se calculasse, cairia sobre a memória do módulo. A defesa não é um piso: **todo o ponteiro recebido é recusado com `-14` se o intervalo `[ptr, ptr + len)` intersectar o armazém estático do módulo**, e a fronteira é derivada em código de `core::ptr::addr_of!` sobre esse armazém (`store_span`), nunca de um literal medido. O número abaixo é **medição desta build** e serve para diagnóstico, não como conselho de colocação. |
| D7 | O fixture `vectors.kv` carrega três margens por eixo; o registo carrega sete leituras por sítio | São duas formas do mesmo objecto: o ficheiro `key=value` é a forma do harness, o registo é a forma da fronteira. `accept` deriva os eixos dos sete sítios (acima), e o teste diferencial compara a porta com o núcleo sobre o mesmo estado. |

## Medição do módulo (medição nova)

Medido nesta build, não herdado de §0: o `stoch_abi.wasm` release tem **8 124 bytes**,
memória inicial de **18 páginas** (1 179 648 bytes), **10 secções** (Type, Function,
Memory, Global, Export, Code, Data, e três `custom`: `name`, `producers`,
`target_features`), **6 exportações** (`memory` e as cinco funções), **0 secções de
import**, **0 `memory.grow`**, **0 tabelas**, **0 funções de arranque**. O único
segmento de dados tem 60 bytes em `1048576` e contém o digest
`efe2bad329e5727899e4c89b7989d4c9e92cc01c961ce896f71e7f1aed6bd3d0` do fixture corrente;
`__stack_pointer` inicializa em `1048576`.

O segmento de dados acaba em `1048636` (`1048576 + 60`), e o linker colocou o armazém
estático (`TRAJECTORY`, os 1 024 × 64 bytes de `.bss`) no primeiro endereço que respeita
o alinhamento 8 do array: **início `1048640`**, **fim `1114176`** (`1048640 + 65 536`),
contra um `memory.buffer.byteLength` de **1 179 648** (18 páginas) — dentro da memória e é
essa a reserva declarada nesta build. Os três valores foram medidos neste módulo, não
herdados: um build de sonda fora da árvore, com duas exportações temporárias devolvendo
`addr_of!(TRAJECTORY.0)` e `addr_of!(TRAJECTORY.0) + 65 536`, lido por
`WebAssembly.instantiate` no Node, mais `memory.buffer.byteLength`. A sonda não está na
árvore — o `-14` da entrega deriva o endereço do próprio array, e é por isso que ele não
pode envelhecer.

O início **não** é `align8(__data_end)` de uma build anterior: `__data_end` é o fim do
`.rodata` desta build (`1048636`), e o que cai no primeiro múltiplo de 8 a partir dele é
`.bss`, não dados. O número que vale é o endereço que o linker deu ao array — por isso o
módulo o deriva em código, com `core::ptr::addr_of!`, e recusa com `-14`
(`E_CALLER_CONTRACT`) qualquer intervalo que o sobreponha. O valor aqui é medição de
diagnóstico, não instrução de colocação: quem coloca a arena deve derivá-la do topo da
memória, e recebe recusa — não corrupção — se errar.

**Porque o número mudou de 7 943 para 8 124.** O delta é de **+181 bytes, todos na
secção Code** (6 959 → 7 140); Type, Function, Memory, Global, Export, Data e as três
secções `custom` têm exactamente o mesmo tamanho de antes. É o portão de sobreposição,
inlined nos quatro pontos de entrada que recebem ponteiros (`step`, `accept`, `diagnose`,
`vectors_hash`): derivação do endereço do armazém, comparação de intervalos e o caso de
comprimento nulo. Não há função nova na secção Function (continuam 14: três porteiros, as
cinco exportações e seis funções do núcleo), não há exportação nova, e o segmento de dados
não mudou — o armazém continua em memória zero-inicializada.

**Porque o número mudou de 7 925 para 7 943.** A medição anterior era do crate de um
só ficheiro. Depois do split (C6: `#![deny(unsafe_code)]` no crate, `#![allow(unsafe_code)]`
num único módulo, `src/abi.rs`), os três porteiros internos passam a chamar-se
`stoch_abi::abi::*` em vez de `stoch_abi::*`, e é só o `name` — uma secção `custom`,
não executável — que carrega nomes de módulo. Os 18 bytes de diferença estão todos
lá: o `name` tem 521 bytes (0x209), e sem as três secções `custom` o módulo de então tinha
**7 189 bytes**, parte que não depende de nome nenhum. O código, o conjunto de
exportações, os códigos de retorno e o layout não mudaram (as 14 funções da secção
Code continuam a ser as mesmas — três porteiros, as cinco exportações e seis funções
do núcleo —, e o teste de módulo compara o conjunto de exportações por igualdade, não
por continência). Não houve crescimento de código.

Com o portão de sobreposição, a mesma build sem as três secções `custom` mede
**7 370 bytes** (`RUSTFLAGS='-C strip=symbols'`), isto é, os 181 bytes do portão somados
aos 7 189: o portão é código, não nome.

O número `2 748` de §0 é de outra redução (194 linhas) e **não** foi tocado; este é o
tamanho do artefacto que a §2 descreve, agora que existe.

## O que fica por fazer

1. **Portão de leitura selada.** Presente na porta para `diagnose` e para as margens;
   o que resta não é verificável por bytes: a porta pode recusar uma leitura fora da
   escala, não pode saber se a leitura foi *tomada*. A proveniência das sete leituras
   (o instrumento selado a montante) é obrigação do chamador, e o README do adaptador
   deve dizê-lo — aqui fica declarado.
2. **Tamanho do módulo.** Medido acima; o limite aceitável para o adaptador ainda não
   está fixado por ninguém, e fixá-lo é decisão do autor, não desta crate.
3. **Limite de Thompson/Wheeler.** A cadeia de suprimentos não é fechada por esta
   fase: o módulo é compilado por um único `rustc`, e o resíduo A4 (`03:107–148`)
   continua aberto — o ataque de Thompson (1984) sobrevive à leitura completa do
   fonte, e o critério de Wheeler (2005; 2009) exige *Diverse Double-Compiling* com
   cadeias de bootstrap independentes (`mrustc`/GNU Guix), que a Fase 4 não executa.
   Este crate pode ser auditado byte a byte; não pode ser *provado* livre do
   compilador que o produziu.

## Como verificar

```sh
export TMPDIR="${TMPDIR:-$HOME/.cache/rs-tmp}"   # só quando /tmp é um tmpfs pequeno
cargo test --offline -p stoch-abi
cargo build --release --target wasm32-unknown-unknown -p stoch-abi
cargo test --offline --workspace
```

`tests/port.rs` exercita a porta no host (vetor feliz por regime, recusa de `accept`
com código do núcleo, trajetória absorvida, estado parcial recusado sem escrita,
digest contra `sha256sum` do ficheiro em disco e sensibilidade a um byte trocado).
`tests/module.rs` lê o binário: percorre a tabela de secções e a secção de exportação e
compara o conjunto de nomes — não procura um nome dentro de um relatório.

Opcional, com um motor wasm real: `node crates/stoch-abi/tests/wasm-smoke.mjs` instancia
o `.wasm` publicado e chama as cinco exportações pela fronteira `i32` (arena em
`200000`, abaixo do segmento de dados), incluindo os casos de recusa `-14` e o digest
`efe2bad329e57278`. Não faz parte do `cargo test` porque o workspace não depende de um
runtime JavaScript.
