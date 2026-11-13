# Auditoria algébrica dos blocos de display math

Escopo: os 452 blocos `$$…$$` de `book-src/` (82 arquivos), mais as igualdades inline que carregam fórmula nomeada. `book-src/` não foi tocado por esta auditoria; os artefatos estão todos em `audit/`.

## Método

1. **Extração** — pareamento por offset de `$$` (não por linha: um extrator por linha perde o texto que segue `$$` na mesma linha), `line` = linha do `$$` de abertura. 452 blocos confirmados, inventário em `audit/_blocks.json`.
2. **sympy** — sympy 1.14 com `sympy.parsing.latex`; o parser exige `antlr4-python3-runtime==4.11.1` (a 4.13 falha com `antlr4.error.ErrorListener module is not installed`). Pré-processamento: remoção de `\mkern*mu`, de `\left/\right/\bigl/\bigr`, `\_`→`_`, desembrulho de `\mathrm/\text/\mathbb/\mathcal/\operatorname`, `\tfrac`→`\frac`, `\cdot`→`*`.
3. **Bateria automática** — 112 blocos com exatamente um `=` e sem símbolo de relação: 2 OK, 83 DIFF, 27 ERR (parse impossível). Os 83 DIFF são **falsos positivos** de três tipos: equações implícitas com a incógnita dos dois lados (ex. Lundberg `\lambda(M_X(R)-1)=cR`), definições em que o símbolo aparece nos dois lados (`\rho=\lambda T_s`), e subscritos destruídos ao remover `\mathrm` (`\mathrm{TTS}_{\mathrm{survival}}`). Conclusão registrada no próprio método: **o sympy não emite veredito confiável neste corpus**; foi usado como contraprova pontual, nunca como juiz.
4. **Re-derivação** — cada identidade foi re-derivada à mão contra a forma canônica publicada (Kish, Chao, Hill, Kingman, Meng, Chouldechova, Kelly, Itô, Manski, Tian–Pearl, Imbens–Manski, Murphy, Ramdas, Perdomo, Alexy) e, onde havia número, recomposta numericamente.
5. **Inline** — 126 igualdades inline em `$…$` caem na vizinhança de uma fórmula nomeada; foram triadas e as que carregam identidade nomeada (`14:218`, `34:202/218/247`, `31:220`, `38:191`, `35:258`, `03:263`, `27:168`, `16:229`) conferem. As restantes são definições ou repetem, em forma curta, um bloco de display já auditado acima.

## Contagem por veredito

| Veredito | Blocos |
| :--- | ---: |
| CORRETO | 452 |
| CORRETO-MAS-ROTULADO-ERRADO | 0 |
| ERRADO | 0 |
| AMBÍGUO | 0 |
| **Total** | **452** |

Dos 452: 78 não são igualdades (definições, intervalos, conjuntos, gramáticas) e foram lidos simbolicamente; 374 carregam `=`, dos quais 195 são identidades candidatas (não meras definições) e foram todas re-derivadas uma a uma.

## Os dois blocos que quase foram reprovados — e por que passam

`book-src/47-irreversible-time-vital-margin.md:164` e `book-src/appendix-a-mathematics.md:181` imprimem

`k(n_eff^var) = 1/(1 + 1/n_eff^var) ∈ (0,1]`, ao lado de `k_BM = z²/(1 + z² + f̂²(1+½γ̂₂)/n_eff^vol)`.

Num primeiro passe isto parece erro: **o fator não contém `\hat\mu` nem `\hat\sigma`**, e por isso não shrinkaria mais quando o sinal é fraco, ao contrário do que uma "Kelly shrinkage" deveria fazer. A absolvição vem da frase imediatamente acima, que fixa a variância de estimação `\sigma_{\hat\mu}^2 = \sigma^2/n_eff^var`: inflar a variância do denominador de Kelly com ela dá

`f^* = \hat\mu/(\sigma^2 + \sigma^2/n_eff^var) = k(n_eff^var)\,\hat\mu/\sigma^2`,

exatamente a forma impressa. A derivação é de uma linha e fecha; `k<1` em toda amostra finita, como diz `38:191`. Logo **CORRETO**, e não CORRETO-MAS-ROTULADO-ERRADO: a identidade como exibida é verdadeira e é a que o modelo do próprio texto produz.

Duas ressalvas ficam registradas, e nenhuma delas é defeito algébrico:

- **Atribuição a confirmar na trilha de citações.** O texto atribui a forma simples a Baker & McHale (2013) e imprime `k_BM` na mesma linha. Os dois não são o mesmo objeto: `k` depende só de `n_eff`, `k_BM` depende de `z^2 = n_eff^vol \hat\mu^2/\hat\sigma^2` e da curtose; coincidem apenas quando `z^2 = n_eff`. Se a fonte publica a forma em `z^2`, o que está em jogo é a atribuição — questão da trilha de citações, não desta.
- **Força da frase vizinha.** Dizer que o fator "pulls the reckless plug-in leverage back" para `T` na casa das dezenas de milhares é forte demais: com `n_eff=10^4`, `k=0{,}9999`. A álgebra está certa e a frase é que promete mais do que a fórmula entrega; pertence à trilha de garantia (§IV), não a esta.

## Os 27 blocos não parseáveis, checados à mão

`00:220`, `03:196`, `10:205`, `10:207`, `15:236`, `15:266`, `21:248`, `28:170`, `34:175`, `34:183`, `41:179`, `44:167`, `46:224`, `47:234`, `47:244`, `47:280`, `47:282`, `48:173`, `48:195`, `48:241`, `48:243`, `49:215`, `54:43`, `58:29`, `62:121`, `82:47`, `appendix-a:247`.

Todos CORRETO. Amostras do que a mão precisou fazer: `21:248` é a entropia de Kolmogorov–Sinai `h_{KS}=\sum_{\lambda_i>0}\lambda_i` (Pesin); `48:195` é o `F` clássico de restrição linear; `54:43` é Jensen sobre perda convexa; `62:121` é `\delta x^*=-(\lambda/c^*)\delta c^*` por diferenciação implícita de `x^*=\lambda\ln(c_0/c^*)`; `82:47` é a fórmula do peso de Alexy.

## Notas de julgamento (para não reabrir depois)

- **Peso de Keynes** `V(n_eff)=n_eff/(n_eff+n_0)` (`05:197`, `17:244`, `33:239`, `45:132`, `45:144`, `appendix-a:156`, `appendix-a:421`): **definição construída pelo autor**, não teorema. Confere com os limites declarados (`V=1/2` em `n_eff=n_0`, `V\to1`) e `V'=n_0/(n_eff+n_0)^2>0`. Keynes (1921) fornece a distinção peso/probabilidade, não a forma fechada. CORRETO.
- **Composição de design effects** `1/n_eff=\mathrm{DEFF}/n+((1-f)/f)\bar\rho^2` (`34:220`, `45:132`, `45:144`) e `\mathrm{DEFF}_{total}=\mathrm{DEFF}_{cluster}+\mathrm{DEFF}_{Kish}-1` (`34:218`): identidades definicionais do capítulo, e a forma aditiva é a mesma que o §V-9 do contrato fixa como canônica. O produto é apresentado explicitamente como cota superior, com o termo cruzado não-negativo nomeado. CORRETO.
- **Chao2 em `t=2`**: `f_1^2/(4f_2)` com `f_2=n_{11}` e `f_1=n_{10}+n_{01}` está correto **na convenção do próprio texto**; `f_1^2/(4f_2)\ge n_{10}n_{01}/n_{11}` é AM–GM. Não trocar o 4 por 2.
- **Hill**: `\hat\alpha_k=k/\sum\ln(X_{(n-i+1)}/X_{(n-k)})`, `E[\hat\alpha_k]=\alpha k/(k-1)`, `\mathrm{Var}=\alpha^2k^2/((k-1)^2(k-2))` — confirmados exatamente de `S_k\sim\mathrm{Gamma}(k,1/\alpha)`; `E[\hat\alpha']=\alpha k/(k-2)` é o estimador de variância. CORRETO.
- **Kingman**: o limite com `c_a^2/\rho^2` e a aproximação de tráfego pesado sem ele estão rotulados como limite e como aproximação (`52:21-25`, `47:187-188`), e a coincidência com Pollaczek–Khinchine em `c_a^2=1` está declarada. Testado em M/M/1 e M/G/1: o limite domina a forma exata em toda carga.
- **EVPI** `\min(C,\pi\eta L)-\pi\min(C,\eta L)` (`28:195`, `43:162`, `appendix-a:136`): re-derivado dos dois regimes `\pi<p^*` e `\pi>p^*`; o `\max(0,\cdot)` de `28:195` é redundante, não errado.
- **Cobertura** `\Phi(c_\alpha+\sqrt n(\hat\theta_U-\hat\theta_L)/\hat\sigma)-\Phi(-c_\alpha)=1-\alpha` (`35:212`, `appendix-a:146`): forma Imbens–Manski; `\Delta\to0` dá `z_{1-\alpha/2}`, `\Delta\to\infty` dá `z_{1-\alpha}`.
- **Integral aninhada** `50:298-316`: correta **porque `e` é a variável de integração** (`\ln e\ne1`). Não "corrigir" para 1.
- **`13:265`** não é Kingman: é VaR/CVaR comonotônico, e o contraexemplo de subaditividade do VaR sob eventos mutuamente exclusivos está correto (VaR viola, CVaR satisfaz).
- **Tian–Pearl**: `48:220/222` são `PN`/`PS` com `P(X=1,Y=1)`; `48:285/286` são `PN`/`PS` na forma geral; `39:188` e `48:277` são `PNS`. Três objetos distintos, nenhum trocado.
- **Condições de validade**: `25:310` exige `\gamma T>1`; o teto de TTR (`20:253`, `49:197`, `appendix-a:203`) exige `e_t>1`; `01:237` exige `r<C<\eta L`; `43:144` exige `P(E)>0`. Todas essas condições **estão declaradas** na prosa que acompanha o bloco — é por isso que nenhum bloco foi rotulado CORRETO-MAS-ROTULADO-ERRADO.

## Limites declarados desta auditoria

- **Zero AMBÍGUO** não significa que todo símbolo do apêndice seja auto-contido: vários blocos de `appendix-a` usam símbolos definidos nos capítulos (ex. `c_a^2` no item de Kingman). É o limite editorial que o §II autoriza; nenhum capítulo exibe símbolo que ele mesmo não defina.
- A auditoria verifica **identidade**, não atribuição. Um bloco algebricamente correto atribuído à fonte errada é defeito de citação e pertence à trilha de atribuição. O caso registrado acima (`47:164` / `appendix-a:181`) é exatamente desse tipo: a álgebra fecha, a atribuição é que precisa de conferência.
- Um bloco pode estar correto e ainda violar o §IV (número sem garantia); isso é o censo de numerais, não esta auditoria.

## Oportunidade de portão mecânico

O censo não vê álgebra e o portão estrutural também não. O que esta auditoria fez à mão é mecanizável num ponto estreito e de alto retorno: **nenhum símbolo pode aparecer pela primeira vez dentro de um bloco de display** — toda sigla usada num `$$` deve ocorrer antes no mesmo capítulo, em prosa ou em definição. Isso pega a classe de defeito mais provável numa revisão futura (a fórmula importada que perde a definição no caminho) sem exigir juiz simbólico, que este corpus já mostrou não sustentar (83 falsos positivos em 112 tentativas).

O segundo portão barato nasce do caso que quase foi reprovado: **um fator de encolhimento deve ter o seu argumento definido na mesma seção** e, quando dois fatores com nomes aparentados aparecem lado a lado (`k` e `k_BM`), o texto deve dizer em uma frase por que não são o mesmo objeto. Nenhum dos dois portões lê álgebra; ambos leem dependências de símbolo, que é o que um leitor humano perde primeiro numa revisão.

---

Uma linha por bloco, na ordem dos arquivos:
