# Adaptadores de teste

**Estes dois consumidores são dublês de teste criados para ensaiar a ABI; o gatilho
de PLAN §4.2 continua não satisfeito.**

Este diretório guarda o adaptador de um dos dois consumidores. O adaptador não é um
produto nem uma promessa de suporte: existe para exercitar a fronteira wasm do
`crates/stoch-abi` a partir de um runtime de linguagem, e para que o segundo
consumidor (Python, direto no Wasmtime, em `consumers/python/`) tenha contra o que
discordar.

## Conteúdo

| Caminho | Papel |
| :--- | :--- |
| `ts/adapter.ts` | Adaptador TypeScript: handshake de versão em `bigint`, arena na memória do chamador, um único ponto de conversão `bigint → number` declarado, decodificação dos dois registos de saída. |
| `ts/adapter.test.ts` | Os catorze pontos de divergência de `02-arquitetura.md` §8.2 (D1–D14), um teste por ponto. |
| `ts/mutation.mjs` | Estudo de mutação: cada um dos catorze pontos tem uma mutação que só aquele teste apanha. |
| `ts/README.md` | Tabela dos catorze pontos, limites conhecidos e como correr. |

## Como correr

```
node --test adapters/ts/adapter.test.ts
node adapters/ts/mutation.mjs
```

Ambos são necessários: o primeiro mostra que os testes passam contra a porta real, o
segundo que cada teste falha quando o seu ponto é violado — um teste que não falha
quando o ponto muda não prova nada sobre o ponto.

## Limite declarado

O ensaio prova que dois ports concordam, nunca que os vetores cobrem o domínio
(documento de desenho arquivado, `02-arquitetura.md` §8.3). A cobertura é assunto de
`01-funcionalidades.md` §6.3, também arquivado, e do arnês de `crates/stoch-vectors`,
não deste diretório.
