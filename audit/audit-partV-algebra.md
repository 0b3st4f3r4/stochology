# Auditoria algébrica — expressões inline da Parte V (capítulos 51–85)

Task `t18` · auditor `math-auditor` · escopo: `book-src/51-*.md` … `book-src/85-*.md`, somente álgebra inline (spans `$…$`) e as frases de inferência que as carregam. Blocos de display math da Parte V já foram cobertos por `audit/audit-math.md` (452/452). Nada em `book-src/` foi editado.

## 1. Método

1. Extração por regex de spans `$…$` fora de `$$`, exigindo `\`, `=`, `<`, `>` ou dígito: **907 spans em 35 arquivos**.
2. Extração das frases de inferência (`therefore`, `so`, `hence`, `thus`, `since`, `gives`, `leaves`, `forces`) que contêm matemática: **202 frases**, lidas integralmente.
3. Para cada span com relação nomeada, recomputação explícita: cadeia simbólica à mão, substituição numérica quando havia valor, e checagem de **direção** (desigualdade, sinal, monotonicidade, expoente assintótico, taxa vs nível).
4. Busca dirigida por classes de risco: (a) quantificadores (`at most/at least/never/always`), (b) razões de halving/doubling/percentagem, (c) `therefore` cuja conclusão não decorre das premissas da mesma seção, (d) instanciação numérica apresentada como álgebra, (e) colisão de símbolo.

Contagem por capítulo: 51:11, 52:35, 53:25, 54:24, 55:42, 56:21, 57:30, 58:34, 59:19, 60:37, 61:17, 62:12, 63:10, 64:4, 65:13, 66:10, 67:41, 68:21, 69:29, 70:19, 71:12, 72:17, 73:10, 74:6, 75:14, 76:52, 77:50, 78:42, 79:44, 80:56, 81:35, 82:45, 83:18, 84:36, 85:16.

**Limites declarados.** Os 907 spans foram enumerados e inspecionados; ~230 foram re-derivados explicitamente (todos os que portam relação entre grandezas nomeadas, mais todas as instâncias numéricas). Os demais são tokens de notação (símbolos isolados, índices, unidades) cuja leitura foi contra a definição local, não re-derivação. Prosa integral lida para 51–58, 60, 64, 67, 69, 74, 76–82, 84, 85; para 59, 61–63, 65, 66, 68, 70–73, 75, 83 a leitura foi das frases de inferência e spans, não do capítulo inteiro. Uma auditoria sem essa declaração afirmaria mais do que fez (§IV).

## 2. Sumário de vereditos

| Veredito | Capítulos | Ocorrências |
| :--- | :--- | :--- |
| CORRETO | 51–75, 77, 79–83, 85 | 32 capítulos, 0 defeito |
| CORRETO com nota de notação | 78, 84 | 2 capítulos, colisão de símbolo, sem erro algébrico |
| INFERÊNCIA NÃO SEGUE (condição omitida) + NOTAÇÃO | 76 | 1 capítulo, 1 achado, 2 sintomas |
| ERRO DE ÁLGEBRA | — | 0 |
| INSTANCIAÇÃO SEM GARANTIA | — | 0 |

Nenhuma igualdade ou desigualdade inline da Parte V é falsa. Os quatro defeitos encontrados são de **condição** e de **notação**: a álgebra exibida está certa, mas uma conclusão só vale sob hipótese não declarada, e três capítulos usam o mesmo símbolo para dois objetos. Ambos são falhas de §IV na medida em que a frase carrega mais do que a álgebra ao lado sustenta.

## 3. Achados

### A1 — `book-src/76-layer-sequential-decision-bandit-real-options-order.md:43` · INFERÊNCIA NÃO SEGUE (condição omitida)

Verbatim: *"The root above one, $\beta\_1$, falls as the diffusion grows, so the hurdle $\beta\_1/(\beta\_1 - 1)$ widens with volatility and tends to $r/(r - \alpha)$ as the diffusion vanishes."*

Recomputação. De $\tfrac12\sigma^2\beta(\beta-1)+\alpha\beta-r=0$:

- $\sigma^2\to0$: $\alpha\beta=r\Rightarrow\beta_1\to r/\alpha$. **Existe raiz acima de 1 somente se $r>\alpha$.**
- $\sigma^2\to\infty$: $\beta^2-\beta\to0\Rightarrow\beta_1\to1$, hurdle $\to\infty$. Correto.
- Monotonicidade: $\partial\beta_1/\partial\sigma^2<0$ pela regra implícita aplicada a $F(\beta,\sigma^2)=\tfrac12\sigma^2\beta(\beta-1)+\alpha\beta-r$, com $F_\beta=\sigma^2(\beta-\tfrac12)+\alpha>0$ no ramo $\beta>1$. Correto.

O que falta: a frase nomeia "the root above one" e imprime o limite $r/(r-\alpha)$ sem declarar $r>\alpha$ (equivalente a um dividend yield $\delta=r-\alpha>0$). Se $r\le\alpha$ não há raiz acima de 1, o hurdle $\beta_1/(\beta_1-1)$ é indefinido (ou negativo) e o limite impresso muda de sinal. A conclusão "widens with volatility" também só vale no ramo $\beta_1>1$.

Fix mínimo: inserir a hipótese na própria frase — *"…and tends to $r/(r-\alpha)$ as the diffusion vanishes (both statements require $r>\alpha$; at $r\le\alpha$ no root above one exists and waiting is unbounded)"*.

### A2 — `book-src/76-…-order.md:28` vs `:43` · NOTAÇÃO (colisão de símbolo no mesmo capítulo)

- `:28` — *"two boundaries, approximately $A = (1-\beta)/\alpha$ and $B = \beta/(1-\alpha)$, built from the tolerated false-positive and false-negative rates"*: aqui $\alpha$ = taxa de falso positivo tolerada, $\beta$ = taxa de falso negativo tolerada (Wald).
- `:43` — *"$\frac12\sigma^2\beta(\beta-1)+\alpha\beta-r=0$"*: aqui $\alpha$ = deriva do processo, $\beta$ = raiz característica (Dixit–Pindyck).

Dois pares ($\alpha,\beta$) com significados incompatíveis a quinze linhas de distância, sem redenominação. O leitor que retém $\beta$ = taxa de falso negativo lê a equação da raiz como equação nas taxas de erro e não chega a nada. Fix mínimo: $\alpha\_W,\beta\_W$ em `:28`, e $\mu$ para a deriva em `:43` (mantendo $\beta_1$ como raiz).

### A3 — `book-src/78-layer-macro-finance-econometrics-actuarial-order.md:36` vs `:40` · NOTAÇÃO (colisão de símbolo no mesmo capítulo)

- `:36` — *"$k(n\_{\mathrm{eff}}) = 1/(1 + 1/n\_{\mathrm{eff}})$"*: $k$ = fator de encolhimento de Baker–McHale, função de $n\_{\mathrm{eff}}$.
- `:40` — *"A Gaussian prices a fall of $k$ standard deviations below $\varphi(k)/k = e^{-k^{2}/2}/(k\sqrt{2\pi})$"*: $k$ = número de desvios-padrão.

O mesmo $k$ nomeia um fator de encolhimento e um quantil na mesma seção. Fix mínimo: $k\_{\mathrm{BM}}$ no primeiro uso (como já aparece em 47 e no próprio capítulo 78 adiante) ou $z$ no segundo.

Além disso, a redação de `:40` atribui a cota ao objeto errado: $\varphi(k)/k$ limita a **probabilidade de cauda** $\bar\Phi(k)$, não a queda. Fix mínimo: *"A Gaussian prices a fall of $k$ standard deviations at an upper-tail probability no larger than $\varphi(k)/k = e^{-k^2/2}/(k\sqrt{2\pi})$"*.

### A4 — `book-src/84-layer-infrastructure-nuclear-hydraulic-safety-order.md:27` vs `:33` · NOTAÇÃO (colisão de símbolo no mesmo capítulo)

- `:27` — *"the empty headroom $1 - \rho\_t$"*: $\rho\_t$ = utilização de armazenamento (freeboard como folga).
- `:33` — *"under a shared-cause failure fraction $\rho \in [0, 1]$"*: $\rho$ = fração de falha por causa comum.

Duas grandezas adimensionais distintas em $[0,1]$, no mesmo capítulo, a seis linhas de distância. Fix mínimo: $\rho\_{\mathrm{cc}}$ para a fração de causa comum (`:33`, `:57` e exercício 2), mantendo $\rho\_t$ para utilização.

Nota de forma, sem erro: o teto de `:33` está correto. $P\_{\mathrm{fail}}(k)=(1-\rho)p^k+\rho p\ge\rho p$; $R\_{\mathrm{par}}(k)=1-P\_{\mathrm{fail}}\le1-\rho p=1-\rho(1-r)$; $\partial P\_{\mathrm{fail}}/\partial k=(1-\rho)p^k\ln p<0$ para $p\in(0,1)$, e a derivada tende a zero, deixando $R\_{\mathrm{par}}$ preso em $1-\rho(1-r)$. Confere.

## 4. Capítulo por capítulo

**51 · micro-kinético/sensório-motor — CORRETO.** `:55` "doubling the speed doubles the distance held": frenagem $d=v^2/(2a)$ dá $d\propto v^2$ e o *tempo* de reação é $v\tau$; a frase fala de "clearance required" na taxa (regime de atraso), onde $d=v\tau\propto v$ — coerente com o regime que ela nomeia. `:88/96` círculo de atrito: $a\_{long}^2+a\_{lat}^2\le(\mu g)^2$ e $a\_{lat}=\mu g\sqrt{1-u^2}$ com $u=a\_{long}/(\mu g)$ — identidade trigonométrica correta.

**52 · filas meso — CORRETO.** `:23/25` $W\approx(\rho/(1-\rho))((c_a^2+c_s^2)/2)(1/\mu)$; o fator adicional $1/\rho^2$ no termo de rajada é a forma finita correta ($c_a^2/\rho^2\to c_a^2$ quando $\rho\to1$), e a prosa rotula qual é limite e qual é aproximação. `:51` "the marginal benefit of the last unit of buffer is smaller" decorre de $\min(L,A)\le L$ monotônico em $L$ — correto.

**53 · macro-acoplado/topologia — CORRETO.** `:17` $\alpha\_{tol}=(1-\rho)/\rho=1/\rho-1$ ("reciprocal of the load factor less one"). `:23` limiar $\rho>z/(z+1)$: de $L/(1-\rho)>L(1+1/z)$; incremento do $z$-ésimo vínculo é $t(z)-t(z-1)=1/(z(z+1))$ — a indexação do texto ("increments $1/(z(z+1))$") é a correta para o incremento atribuído ao vínculo $z$. `:29` contenção antes da extremidade sse $\tau_r<k\tau_p$ — direção correta.

**54 · geofísica/assimilação — CORRETO.** `:28/89` $\Delta\tau=\ln(\delta\_{tol}/\varepsilon_0)/\lambda$, e halving $\varepsilon_0$ soma $\ln2/\lambda$ (aditivo, não multiplicativo) — correto. `:91` dobrar a densidade de observação não dobra o lead time, porque a informação entra em $e^{\lambda\Delta\tau}$ — correto. `:94` $\sigma_a^2=BR/(B+R)$ com $R\to R/2$ dá $\Delta\tau=(1/2\lambda)\ln((2B+R)/(B+R))$ — verificado. `:105` "RMSE falls by $\sqrt{1-3w/4}$": variância $1\to w/4+(1-w)=1-3w/4$ — verificado.

**55 · contínuo/fratura — CORRETO.** `:31` $K\_{Ic}^2=2\gamma E$ (de $G=K^2/E$, $G=2\gamma$); $\Pi(a_c)-\Pi(0)=2\gamma a_c$; $\Pi''(a_c)=-2\pi\sigma^2/E<0$. `:66` dobra de $V'(x)=x^3-\mu x-h$ com $\mu=3x^2$: $x=-(h/2)^{1/3}$, $\mu\_{\max}=3(h/2)^{2/3}$ — re-derivado. `:85` $r_1=r_0N^{-1/m}$ (máximo de Weibull).

**56 · formal/tipos — CORRETO.** `:63` "at most one leaf consumes each resource" é a definição de disciplina afim — correto como definição. `:71` $c(g)=\min(1,C/g^7)$ ⇒ cobertura plena enquanto $g^7\le C$ — correto. `:141` $\ell(t)>0$ força no máximo $m$ re-checagens em $m$ escritas — correto. `:188` separação linear/afim — correta.

**57 · credibilidade/medição — CORRETO.** `:109` sinal de $\mathbf 1\{f(x_i)>f(y_j)\}$ invariante sob transformação monotônica — correto. `:117` $\prod_a(1-\varphi_a(\tau))=(1-\tau)^3$ — correto. `:197` $\mathrm{mean}(e^x)\ne e^{\mathrm{mean}(x)}$ (Jensen estrito) — correto. `:28-46` previsão superior sobe a 1 e $\mathcal P'\subset\mathcal P$ contrai o intervalo — direções corretas.

**58 · metrologia/geométrico — CORRETO.** `:70` $u\_{res}=\delta/\sqrt{12}$; `:71` Fisher $1/[\theta(1-\theta)]$; `:75` distância de Fisher–Rao $2(\arcsin\sqrt{\theta_1}-\arcsin\sqrt{\theta_0})$; `:90/94` $u_A^2=s^2/n\to0$ e $u_c^2=\sigma_A^2\mathrm{DEFF}/n+u_B^2$. `:71` $n^\star=12s^2/\delta^2$ é **cruzamento** $s/\sqrt n=\delta/\sqrt{12}$, não requisito mínimo — o texto o apresenta como o ponto em que as duas parcelas se igualam, correto. Exercício 4: $n=s^2\mathrm{DEFF}/(f u_B^2)$ vale com $f$ = fração da **variância** tipo B (declarado no enunciado) — correto. `:82` "1.3 milímetros" é valor de literatura (NASA 1990) — correto.

**59 · experimento/amostragem — CORRETO.** `:45/96` $\mathrm{SE}\propto\sqrt{1+(m-1)\rho_{cl}}$, $n\_{\mathrm{eff}}=Gm/D$, $D=1+(m-1)\rho_{cl}$. `:68` quadruplicar $n$ halva o termo $1/\sqrt n$ e deixa o termo de fronteira intacto — correto. `:75/76` $(\pi_e-\pi_t)(\tau_1-\tau_0)$ — álgebra correta. `:61` banda $(a_t+a_c)(U-L)$ — coerente com Manski com os $a_t,a_c$ definidos acima.

**60 · cripto/zero-knowledge — CORRETO.** `:30` $H(M\mid C)=H(M)$ ⇒ $H(K\mid C)\ge H(M)$ (Shannon 1949) — a igualdade impressa é a de $H(M\mid C)$, e a desigualdade sobre a equivocação é a conclusão; correto. `:36` erro acumulado $\le\varepsilon^k$ (união sobre rodadas independentes), $\le2^{-k}$ para $\varepsilon\le1/2$. `:37` test supermartingale: $\mathbb E[E_k\mid\mathcal F\_{k-1}]=E\_{k-1}$ sob $H_0$ (igualdade), Ville dá $\Pr[\sup E_k\ge1/\alpha]\le\alpha$ — correto. `:65` $k\ge\ln(1/\alpha)/\ln(1/\varepsilon)$; halving $1-\rho$ mais que dobra $\rho/(1-\rho)$ — correto. `:71` $2^{n/2}$ colisão, $2^n$ pré-imagem, $2^{80}$ para 160 bits — correto.

**61 · cinética — CORRETO.** `:49` $f=[S]/(K_M+[S])$; `:63` $\varepsilon_1^N$; `:164` $K_M=(k\_{-1}+k_2)/k_1$ — correto.

**62 · morfogenético — CORRETO.** `:81` fronteira em $\lambda\ln(c_0/c^\ast)$ — log de razão de concentrações, correto; `:160` $\lambda=\sqrt{D\tau}$ — correto.

**63 · imunológico — CORRETO.** `:15` conjunção $s\ge\theta_s$, $c\ge\theta_c$, $s+\lambda c\ge\theta$ — correto.

**64 · filológico/formal — CORRETO.** `:20-40` mapa nomes→tipos many-to-one: "counting names counts the dictionary" é a consequência correta (nº de nomes ≥ nº de tipos).

**65 · estratigrafia — CORRETO.** `:23/24` $(R/T)^{H-1}$ e $(1+a)^{-(H-1)}$; `:29/30` $C/(1-C)$ e $-\ln(1-C)/(H-1)$ — Strauss–Sadler, ambos os casos limites corretos. `:46` "halve the error, four times the counts": erro $\propto1/\sqrt n$ ⇒ $n\to4n$ — correto.

**66 · farmacologia/clínica — CORRETO.** `:23` $TI\le1$; `:33` $-\ln(0.05)/n\approx3/n$ (2.9957) — correto, e a leitura ("poucos milhares exclui só efeitos acima do nível per-mille") segue de $3/3000=10^{-3}$. `:72` halving $\delta$ reduz o poder via $1-\Phi(z\_{1-\alpha}-\delta\sqrt{n\_{\mathrm{eff}}}/\sigma)$ — direção correta.

**67 · epidemiologia — CORRETO.** `:22/23` $r=\beta S-\gamma$, $\mathrm{TTS}=\ln2/r$; dobrar $r$ halva o relógio; "only the slowest buys the week" (TTS $\ge$ 1 semana ⇒ $r\le\ln2$/semana) — direção correta. `:90` $\hat N=n_1n_2/m$; `:149` "double both clocks, $R\_{\mathrm{eff}}$ unchanged" (razão $d/T$) — correto. `:152/153` $r=\ln R_0/T$ (geração fixa) e $R_0=rT/(1-e^{-rT})$ (infecciosidade uniforme) estão ambos rotulados, com "the exponent below is approximate" — correto. `:174` isolar remove no máximo $1-\theta$ ⇒ $R\_{\mathrm{eff}}\ge\theta R$; :175 exige $\theta R>1$ — declarado. `:200/201` exercícios: $2^4=16$, $2^4-1=15$ — corretos.

**68 · farmacovigilância — CORRETO.** `:24` regra de três $3/p$; `:32` $\lceil\ln(1/\alpha)/\ln(\text{per-look ratio})\rceil$; `:48-50` OR 1.43 [1.03, 1.98] com $\ln(1.98/1.03)=0.6535$, $\ln(1.43/1.03)=0.3284$, e $\ln(1.03)=0.0296$ = 9% do semi-intervalo — "clears the null by only $\ln(1.03)$" correto. `:171` PRR $=a/(a+b)\div c/(c+d)$.

**69 · metaciência — CORRETO.** `:36-38` $N\_{fs}=(\sum Z_i)^2/z_\alpha^2-k$ de Stouffer $\sum Z_i/\sqrt{k+N}=z_\alpha$; quadruplicar $n$ halva o SE ⇒ o limiar publica efeitos metade — correto. `:66/67` 97% vs 36%, $r=0.403\to0.197$ (OSC 2015) — valores de literatura. `:90-100` e-BH: $k^\ast=\max\{k:E\_{(k)}\ge K/(\alpha k)\}$, passo $E_j\ge K/(\alpha D)$, e $\mathrm{FDR}\le\sum\alpha\mathbb E[E_j]/K\le\alpha K_0/K\le\alpha$ — correto.

**70 · acústico/estético — CORRETO.** `:18` $(3/2)^{12}\ne2^7$; `:29` coma pitagórica $3^{12}/2^{19}$, 23.46 cents; `:40` $1200\log_2(5/4)=386.31$ cents, e $400-386.31=13.69$; `:41` coma sintônica $81/80$, 21.51 cents, quarto de coma $5.38$; `:97` $(3/2)^p=2^q$ sem solução inteira positiva — todos reconferidos numericamente.

**71 · linguístico/evidencial/pragmático — CORRETO.** `:13/59` a distinção entre $P(M=\varnothing)=0$ e $P(M=\varnothing)>0$ é a condição que separa um ato de fala de um ato mudo — correta como definição, e a inferência ("an erratum about an evidential morpheme is a new assertion") segue dela. `:35` $\delta_{\mathrm{status}}\in(0,1)$; `:47/63` a assinatura `(1, 0, 2)` é consistente entre os dois usos. O produto $\prod(1-\varepsilon_i)$ e a leitura de custo marginal em `:27` estão em display math e pertencem a `audit-math.md`, onde foram verificados.

**72 · filologia — CORRETO.** `:17` $\rho_I\to1\Rightarrow n\_{\mathrm{eff}}\to1$; `:35` $\sum_{j=1}^{k}A^j=(A^{k+1}-A)/(A-1)$ para $A\ge2$ — correto.

**73 · tafonomia/historiografia — CORRETO.** `:75` cadeia $P(S\mid A)$; `:76` $S(T)=e^{-\lambda T}$ e $S_1/S_2=e^{(\lambda_2-\lambda_1)T}$ — correto.

**74 · justiça transicional — CORRETO.** `:26` $P(e\in\text{record})\le\mathbf 1[e\in\mathcal C]$ ⇒ probabilidade zero fora do conjunto de categorias — a conclusão segue do indicador. `:69` viés de deslocamento ampliado por $\sqrt{(1-f)/f}$ com $\rho\_{R,Y}<0$ — direção correta.

**75 · ecologia da informação — CORRETO.** `:31` $\mathrm{Cov}(R,Y)=P(R=1)(\mathbb E[Y\mid R=1]-\mathbb E[Y])$; `:43` $\alpha=(\tau-\mu)/\sigma$, $\lambda(\alpha)=\varphi(\alpha)/(1-\Phi(\alpha))$, $\delta(\alpha)=\lambda(\lambda-\alpha)$ (Mills); `:68` $\mathbb E[Y\mid R=1]-\mathbb E[Y]=\rho\_{R,Y}\sigma_R\sigma_Y/P(R=1)$ — todos corretos.

**76 · decisão sequencial/bandit — INFERÊNCIA NÃO SEGUE + NOTAÇÃO (A1, A2).** Corretos: `:26` envelope de Snell e $\tau^\ast$; `:28` $A=(1-\beta)/\alpha$, $B=\beta/(1-\alpha)$ (Wald); `:34` $P(r)=\frac{r-1}{n}\sum_{k=r}^n\frac1{k-1}$; `:36/37/38` $x=r/n$, $P\to-x\ln x$, máximo em $1/e$ com valor $1/e$, $P(1)=1/n$, e $P(4)=0.3987>1/e$ porque a soma excede a integral; `:41` constante de Lai–Robbins diverge quando o gap fecha; `:51` harmônico $=\ln n+\gamma+o(1)$. Defeitos: A1 (`:43`) e A2 (`:28` vs `:43`).

**77 · teoria dos jogos — CORRETO.** `:26/30` gap $g=\tau-w\cdot x_0$, custo $g^2/(2\|w\|^2)$, condição $g\le\|w\|\sqrt{2b}$ — correto. `:32` $\varepsilon<\gamma/\beta$, $L_G\le\varepsilon\beta/\gamma<1$, $k\ge\ln(1/\eta)/\ln(1/L_G)$ — correto. `:38/40` massa alcançável $\bar F(\tau-\delta)$, razão $\bar F(\tau-\delta)/\bar F(\tau)$ crescente em $\tau$ para escores log-côncavos — correto (razão de hazard monotônica).

**78 · macro-financeiro/atuarial — CORRETO com NOTAÇÃO (A3).** `:28` $\psi\propto u^{-(\alpha-1)}$, desvio típico $n^{-(1-1/\alpha)}$, fator $2^{\alpha/(\alpha-1)}$ para halvar (recomputado: $n'/n=2^{1/(1-1/\alpha)}$); `:30` $X\sim\mathrm{Exp}(\mu)$: $\lambda\mu R/(1-\mu R)=cR\Rightarrow R=\theta/((1+\theta)\mu)$, $\theta=c/(\lambda\mu)-1$; `:32` Itô com $\sigma^2/2$; `:36` subaditividade, ES 97.5% Basel, $k(n\_{\mathrm{eff}})$; `:38` $t\ge3.0$ vs 1.96 (Harvey–Liu–Zhu); `:40` 22.6% e 20.4% em 19/10/1987 (Carlson 2007). Defeito: A3.

**79 · estatística oficial/contabilidade forense — CORRETO.** `:20/26` Benford $P(d)=\log_{10}(1+1/d)$: 0.3010/0.1761/0.1249/0.0458 — reconferidos. `:28` suporte $[1,\sqrt{10})$: $P(1)=2\log_{10}2=0.6021$, $P(2)=2\log_{10}(3/2)=0.3522$, $P(3)=1-2\log_{10}3=0.0458$, $P(4..9)=0$, soma 1.0001 — correto. `:30` $\mathrm{MAD}=\frac19\sum|p-b|$; `:32` $\mathrm{MAD}\ge2\times0.6532/9\approx0.145$: os sete dígitos ausentes somam $1-b_1-b_9=0.6532$ e a mesma massa reaparece em 1 e 9, logo $\sum|p-b|\ge1.3064$ — correto. `:34` 0.015 (Nigrini 2012). `:38` identidade contábil ativos $=$ passivos $+$ patrimônio; $\hat n_{00}\ge f_1^2/(2f_2)$ (Chao, cota inferior). `:48` índice de Sen $P_S=H[I+(1-I)G\_{poor}]$; transferência ao mais pobre reduz $I$ e $G\_{poor}$ — correto. `:54` "a debit and a credit that net to zero for any value of the transfer" — correto e é exatamente o ponto (a identidade sobrevive à ficção).

**80 · governança corporativa — CORRETO.** `:20/26` $C=\tfrac12c_1a_1^2+\tfrac12c_2(a_2-(\bar a_2-\gamma a_1))^2$, Hessiano definido positivo ($c_1c_2>0$), $a_1^\ast=\beta_1/c_1$, $a_2^\ast=\bar a_2-(\gamma/c_1)\beta_1$ — correto. `:28` $\beta_1^\ast=1/(1+\gamma^2)$, $L^\ast=\gamma^2/(1+\gamma^2)$: com $\gamma=1/2$, $4/5$ e $1/5$; com $\gamma=1$, $1/2$ e $1/2$; razão de perdas $5/2$ — tudo reconferido (a instanciação é álgebra, não número de literatura). `:34` 125 bi de ativos, 4.7 bi de capital ⇒ $4.7/125=3.76\%\approx3.8$ "percent", alavancagem $125/4.7\approx26.6$ (Lowenstein 2000) — aritmética conferida. `:46` "a rule that is steep at a threshold violates the inequality for every type just below it" — segue da IC: um salto de pagamento no limiar é lucrativo para o tipo imediatamente abaixo.

**81 · prova penal/epistemologia forense — CORRETO.** `:26/28` $p^\ast=c/(c+l)$ com odds $c/l=L\_{FP}/L\_{FN}$; $c=10l$ ⇒ $p^\ast=10/11$; $c\to0\Rightarrow p^\ast\to0$, $l\to0\Rightarrow p^\ast\to1$ — limites corretos nas duas pontas. `:32` confusão $\mathbb P(E\mid I)$ com $\mathbb P(I\mid E)$, com o caso Sally Clark (1/8.543 ao quadrado) — coeficiente de erro. `:38/42` trilema: $\mathrm{FPR}\_A/\mathrm{FPR}\_B$ = odds ratio das base rates quando TPR e PPV se igualam — correto.

**82 · responsabilidade civil/constitucional — CORRETO.** `:26` $p^\ast\_{CL}=C/(\eta L)$, $p^\ast\_{PK}=C\_{FP}/(C\_{FP}+L\_{FN})=k/(k+1)=10/11$. `:28` 1.000 espectadores, 499 bilhetes ⇒ 501 sem pagar, $p=0.501$; o peso de Keynes sobe com o denominador individualizado — correto. `:34` $P\min(W,L)$ com $W<L$ e o gap de Jensen $\Delta_J$ — correto. `:57/58` a mediana sobrevive sse $1-F(x)<q$ **e** $F(x)<q$ — correto (deslocável à direita por coalizão de medida $\ge q$ existente sse $1-F(x)\ge q$). `:70` $C\_{FP}=[C/(\eta L-C)]L\_{FN}$, positiva sse $C<\eta L$ — correto. `:75-78` $180\times200.000=36$; $180\times67.000=12.06$; $2.100\times700=1.47$; soma 49.53; $B=137>PL=49.53$; $125/200.000=625\times$ — aritmética conferida.

**83 · consenso distribuído/SRE — CORRETO.** `:26/30` $2f+1$ e $3f+1$ quóruns; `:32` $1-(1-\varepsilon)^m$; `:38` $\rho/(1-\rho)$. A inferência de `:26` ("uma rodada sem limite viola o freio 3") segue do resultado de não-terminação: sem limite de rodadas, $\tau$ não é limitado e a desigualdade $\tau\le\mathrm{TTS}$ não pode ser garantida — correto.

**84 · infraestrutura/nuclear/hidráulica — CORRETO com NOTAÇÃO (A4).** `:25` $n^{0.5}$ vs $n^H$ (Hurst 1951, $H\in[0.70,0.80]$), $\mathrm{DEFF}\_{temp}\sim n^{2H-1}$, e $\sqrt{n^{2H-1}}=n^{H-1/2}$ com $H=0.5\Rightarrow\mathrm{DEFF}=1$ — correto. `:27` "a reservoir sized on the independent-draws law is short by exactly that factor" — direção correta. `:33/57` ver nota em A4. `:43` design wind $m\times$ mediana ⇒ carga $m^2$ — correto.

**85 · commons policêntrico — CORRETO.** `:26` 98% de queda e 35.000 empregos (Hutchings & Myers 1994; Task Force 1993) — valores de literatura. `:32/60` $H\_{MSY}=rK/4$, $D(H)=K\sqrt{1-4H/(rK)}$, $\mathrm dD/\mathrm dH=-2/(r\sqrt{1-4H/(rK)})\to-\infty$ no máximo — derivada reconferida. `:52/79` $\binom{7}{3}=35$ esgota as combinações e um 36º layer quebraria a bijeção — correto. `:54` leitura do sinal: com $q=0$ e $\gamma=c$, a condição $\gamma\_{select}>1/T$ fica mais fácil quando o álibi **encomprida** $T$ (a sobrevivência $\mathbb P(\tau>t^\ast)$ cresce com $T$), coerente com 25:310 — correto.

## 5. Instanciações numéricas verificadas (nenhuma sem garantia)

Álgebra pura, re-derivável da página: 53:23; 55:66/85; 58:71; 60:36/71; 65:46; 66:33; 67:200/201; 70:18-41; 78:28/55; 80:28; 84:43; 82:75-78.
Literatura com fonte na mesma frase: 58:82 (NASA 1990); 67:66/67 (OSC 2015); 68:48-50 (Nissen–Wolski); 78:40 (Carlson 2007); 79:34 (Nigrini 2012); 80:34 (Lowenstein 2000); 81:32 (Sally Clark, 1/8.543); 84:25 (Hurst 1951); 85:26 (Hutchings & Myers 1994; Task Force 1993); 66:29 (poder de dois grupos $\Phi(\delta\sqrt{n/2}/\sigma-z_{1-\alpha/2})$, com o contraste declarado).
Nenhum número apareceu como instanciação de parâmetro apresentada como derivação — o caso mais próximo, 80:28, instancia $\gamma=1/2$ e $\gamma=1$ para exibir uma razão de perdas, e a razão $5/2$ é álgebra, não valor empírico.

## 6. Encaminhamento mecânico

Quatro gates que esta auditoria sugere, todos de leitura:

1. **Colisão de símbolo por capítulo** — para cada capítulo, casar cada símbolo de span inline com a glosa da primeira definição no arquivo; duas glosas distintas para o mesmo símbolo é falha. Pega A2, A3, A4 sem juízo humano.
2. **Raiz/limite sem hipótese** — toda ocorrência de "tends to"/"→" sobre uma razão de parâmetros exige que a hipótese de domínio do denominador apareça na mesma seção. Pega A1.
3. **Cota atribuída ao objeto errado** — frase com "prices/at most" + desigualdade deve nomear a quantidade limitada; "fall of $k$ sd below $\varphi(k)/k$" não nomeia. Pega a redação de A3.
4. **Quantificador direcional** — "at most/at least" com razão ou diferença: checar monotonicidade declarada, como já se fez aqui para 67:174 e 82:57/58.

## 7. Contagem final

907 spans inline enumerados; 202 frases de inferência lidas integralmente; ~230 expressões re-derivadas explicitamente. Veredictos por capítulo: 32 CORRETO, 2 CORRETO-com-notação (78, 84), 1 com INFERÊNCIA NÃO SEGUE (76) — 35 no total. Erros de álgebra: **0**. Instanciações sem garantia: **0**. Quatro achados, todos com fix de uma linha: A1 (condição $r>\alpha$), A2 ($\alpha,\beta$ de Wald vs deriva/raiz), A3 ($k$ de Baker–McHale vs quantil, e a cota de cauda), A4 ($\rho_t$ vs $\rho$ de causa comum).
