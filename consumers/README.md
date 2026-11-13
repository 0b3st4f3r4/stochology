# Consumidores-dublê da ABI

**Estes dois consumidores são dublês de teste criados para ensaiar a ABI; o gatilho
de PLAN §4.2 continua não satisfeito.**

Dois consumidores, dois ecossistemas, uma só porta:

| Caminho | Runtime | Caminho até a ABI |
| :--- | :--- | :--- |
| `node/run.mjs` | Node 24 (TypeScript por *type stripping*) | pelo adaptador `adapters/ts/adapter.ts` |
| `python/run.py` | Python 3 + Wasmtime | direto nas cinco exportações, sem adaptador |

O segundo existe para que o primeiro não seja a única testemunha: se os dois
concordam com os mesmos bytes, o desacordo entre eles teria de vir da porta, e não de
uma leitura partilhada. Concordarem **não** prova que os vetores cobrem o domínio
(`02-arquitetura.md` §8.3).

```
node consumers/node/run.mjs        # 50 linhas comparadas com o fixture, 2 inexpressivas
python3 consumers/python/run.py    # invariantes de fronteira, sem adaptador
```

## Nota operacional: `pip`, Wasmtime e o tmpfs `/tmp`

Instalar o Wasmtime nesta máquina com o `TMPDIR` por omissão (`/tmp`, um tmpfs que
costuma andar acima de 80% de ocupação) falha no *unpack*:

```
OSError: [Errno 122] Disk quota exceeded
```

Funciona com `TMPDIR` num diretório do `home`, ou com `--target` para um diretório
sob `$HOME` acrescentado ao `PYTHONPATH`. É um problema de empacotamento da máquina,
não do adaptador nem da porta.

## Limites abertos

Cada consumidor regista os seus no README do respetivo diretório; o mais importante
está em `python/README.md` (L4): a base de arena que a ABI declara é a mesma morada em
que vive o buffer estático de `diagnose`.
