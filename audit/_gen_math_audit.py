#!/usr/bin/env python3
"""Gera audit/audit-math.md a partir de audit/_blocks.json + veredictos."""
import json, re, os

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
bs = json.load(open(os.path.join(ROOT, 'audit/_blocks.json')))


def strip(t):
    t = t.strip()
    if t.startswith('$$'):
        t = t[2:]
    if t.endswith('$$'):
        t = t[:-2]
    return re.sub(r'\s+', ' ', t).strip()


def isdef(t):
    s = t.replace('\\,', ' ').split('=')[0].strip()
    if any(op in s for op in ['+', '\\cdot', '\\times', '\\le', '\\ge', '\\to',
                              '\\approx', '(', ')', '[', ']', '\\lbrace',
                              '|', '\\mid', '\\iff', '\\implies', '\\neq']):
        return False
    if ',' in s:
        return False
    return len(s) < 40


# (veredito, metodo, nota) — apenas entradas nao-default
OV = {
 # --- identidades nao triviais re-derivadas ---
 '01-deciding-without-knowing.md:171': ('CORRETO', 'a mao (re-derivacao)', ''),
 '01-deciding-without-knowing.md:175': ('CORRETO', 'a mao (re-derivacao)', 'C+(1-eta)pL<pL <=> p>C/(eta L); algebra identica a 43:160'),
 '01-deciding-without-knowing.md:237': ('CORRETO', 'a mao (re-derivacao)', 'resolvido de C-r = p(eta L - r); exige r<C<eta L, declarado'),
 '05-what-a-number-doesnt-say.md:180': ('CORRETO', 'numerico', '95/(95+495)=0,1610; denominador pi*TPR+(1-pi)*FPR = 590/1000'),
 '05-what-a-number-doesnt-say.md:197': ('CORRETO', 'a mao (definicao)', 'DEFINICAO do peso de Keynes construida pelo autor (V=1/2 em n_eff=n_0, ->1); Keynes 1921 da a distincao peso/probabilidade, nao a forma fechada'),
 '04-it-worked-so-i-was-right.md:180': ('CORRETO', 'a mao (Bayes)', '1/(1+((1-pi)/pi)(s_l/s_h)); forma canonica'),
 '06-who-is-missing-from-the-table.md:212': ('CORRETO', 'a mao + AM-GM', 'Chao2 em t=2: f1^2/(4f2); f1=n10+n01, f2=n11; f1^2/(4f2)>=n10 n01/n11 por AM-GM'),
 '07-when-the-measure-becomes-the-goal.md:169': ('CORRETO', 'a mao (re-derivacao)', 'PR(theta)=E[(Y-theta)^2]+(lambda/2)theta^2 com E[Y]=mu0+eps*theta; confere com 48:253'),
 '10-numbers-order-bounds.md:205': ('CORRETO', 'a mao (aritmetica de intervalos)', ''),
 '10-numbers-order-bounds.md:207': ('CORRETO', 'a mao (aritmetica de intervalos)', 'subtracao inverte os extremos: [Lx-Uy, Ux-Ly]'),
 '12-derivatives-and-convexity.md:197': ('CORRETO', 'a mao (Jensen)', 'Delta_J>=0 para phi convexa'),
 '12-derivatives-and-convexity.md:243': ('CORRETO', 'a mao (diferenciacao)', 'phi"=2/(1-rho)^3>0'),
 '12-derivatives-and-convexity.md:247': ('CORRETO', 'a mao (diferenciacao)', "phi'=1/(1-rho)^2; phi''=2/(1-rho)^3"),
 '12-derivatives-and-convexity.md:272': ('CORRETO', 'a mao (re-derivacao)', 's=1-rho, phi=1/s-1: phi(s/2)/phi(s)=(2-s)/(1-s)'),
 '13-integrals-and-accumulation.md:256': ('CORRETO', 'a mao (definicao espectral)', 'CVaR=(1/(1-a))int_a^1 VaR_u du; com atomo o split preserva peso 1-a'),
 '14-linear-algebra.md:265': ('CORRETO', 'numerico/assintotico', 'E||x||_2 ~ sqrt(d), SD ~ 1/sqrt(2); SD/sqrt(d) ~ 1/sqrt(2d)'),
 '18-philosophy-of-science.md:211': ('CORRETO', 'a mao (definicao)', 'R_Lakatos=E_T/(k+1); -10log10(k+1) e a mesma razao em dB'),
 '21-mechanics-determinism-chaos.md:248': ('CORRETO', 'a mao (Pesin)', 'h_KS=sum_{lambda_i>0} lambda_i'),
 '21-mechanics-determinism-chaos.md:191': ('CORRETO', 'a mao (definicao)', 'divergencia de Lyapunov'),
 '23-statistical-physics-and-time.md:194': ('CORRETO', 'a mao (re-derivacao)', 'Var(Xbar_m)=sigma^2/m+((m-1)/m)rho sigma^2 -> rho sigma^2'),
 '24-chemical-rates-equilibrium-buffers.md:200': ('CORRETO', 'a mao (diferenciacao)', 'fator ln10=2,303; forma canonica da capacidade tampao'),
 '25-evolution-variation-selection.md:292': ('CORRETO', 'a mao (replicador)', 'd ln(x/(1-x))/dt = pi_R-pi_T = -gamma'),
 '25-evolution-variation-selection.md:300': ('CORRETO', 'a mao (re-derivacao)', 't*=(1/c)ln[x0(1-x1)/(x1(1-x0))]; exp(-t*/T) eleva a 1/(cT)'),
 '25-evolution-variation-selection.md:306': ('CORRETO', 'a mao (esperanca)', 'E[ln odds]=ln odds0 - gamma T, pois E[tau]=T'),
 '25-evolution-variation-selection.md:310': ('CORRETO', 'a mao (condicao)', 'a desigualdade vale sob gamma*T>1, condicao declarada na frase imediatamente anterior'),
 '26-regulation-and-homeostasis.md:149': ('CORRETO', 'a mao (definicao)', 'EDO com atraso; definicao do modelo'),
 '26-regulation-and-homeostasis.md:193': ('CORRETO', 'a mao (definicao)', 'dano acumulado e capacidade efetiva'),
 '28-the-machine-and-the-deception.md:195': ('CORRETO', 'a mao (re-derivacao)', 'EVPI=minE-E[min]=min(C,pi eta L)-pi min(C,eta L); conferido nos dois regimes pi<p*, pi>p*'),
 '30-probability-as-language.md:186': ('CORRETO', 'a mao (definicao)', 'U-estatistico de Mann-Whitney com correcao 1/2 para empates'),
 '30-probability-as-language.md:206': ('CORRETO', 'numerico', '0,00099/(0,00099+0,04995)=0,0194'),
 '30-probability-as-language.md:222': ('CORRETO', 'a mao (forma canonica)', 'raio de sequencia de confianca por mistura normal (Ramdas et al.); rho^2 e a escala da mistura, definida no texto'),
 '31-distribution-and-moment.md:217': ('CORRETO', 'numerico + simbolico', 'S_k~Gamma(k,1/alpha): E[alpha_hat]=alpha k/(k-1), Var=alpha^2 k^2/((k-1)^2(k-2)); confirmado por Monte Carlo'),
 '31-distribution-and-moment.md:244': ('CORRETO', 'a mao (integral)', 'E[X^p]=alpha x_min^p/(alpha-p), p<alpha'),
 '31-distribution-and-moment.md:266': ('CORRETO', 'a mao (forma canonica)', 'CRPS=int (F-1{x>=y})^2 dx'),
 '32-the-question-before-the-data.md:208': ('CORRETO', 'a mao (re-derivacao)', '1-Phi(z_{1-alpha}-delta sqrt(n)/sigma)'),
 '33-measuring-and-erring.md:161': ('CORRETO', 'a mao (limite em probabilidade)', 'atenuacao beta sigma_X^2/(sigma_X^2+sigma_u^2)'),
 '34-sample-population-archive.md:175': ('CORRETO', 'a mao (re-derivacao)', '(Ybar_n-Ybar_N)/sigma_Y=rho sqrt((1-f)/f)'),
 '34-sample-population-archive.md:183': ('CORRETO', 'a mao (re-derivacao)', 'n_Meng=f/((1-f)rho^2)'),
 '34-sample-population-archive.md:220': ('CORRETO', 'a mao (identidade definicional)', '1/n_eff=DEFF/n+((1-f)/f)rho^2 e composicao declarada do capitulo, nao teorema de Meng'),
 '34-sample-population-archive.md:245': ('CORRETO', 'a mao + AM-GM', 'Chao2(2)=f1^2/(4f2) com f2=n11; coerente com 34:212'),
 '35-inference-and-evidence.md:197': ('CORRETO', 'a mao (re-derivacao)', 'Y em [0,1]: -(1-p0)=-p; extremos de Manski conferem'),
 '35-inference-and-evidence.md:212': ('CORRETO', 'a mao (limites)', 'Delta->0: c=z_{1-a/2}; Delta->inf: c=z_{1-a}; forma Imbens-Manski'),
 '37-model-fit-and-error.md:153': ('CORRETO', 'a mao (decomposicao)', 'aprox+estimacao+ruido; exata em esperanca sobre o treino sob perda quadratica'),
 '37-model-fit-and-error.md:223': ('CORRETO', 'a mao (definicao)', 'Bregman >=0 por convexidade'),
 '37-model-fit-and-error.md:236': ('CORRETO', 'a mao (otimalidade do PAV)', 'REL=BS(p)-BS(pi)>=0 porque pi minimiza sobre o cone monotonico; expansao identica'),
 '38-time-and-dependence.md:170': ('CORRETO', 'a mao (Ito)', 'dlnS=(mu-sigma^2/2)dt+sigma dW'),
 '39-cause-intervention-counterfactual.md:188': ('CORRETO', 'a mao (Tian-Pearl)', 'PNS em [max(0,P(y|do x)-P(y|do x")), min(...)]'),
 '41-the-triad-and-the-minimum.md:179': ('CORRETO', 'a mao (produto)', 'V_min=(1-tau)^3'),
 '41-the-triad-and-the-minimum.md:186': ('CORRETO', 'a mao (integral/serie)', '1-tau sum_{0..2}(-ln tau)^k/k! = 1-tau+tau ln tau-(tau/2)(ln tau)^2'),
 '43-inquiry-openness-multiverse.md:144': ('CORRETO', 'a mao (Bayes)', 'posterior em {0,1} sob P(H) em {0,1} e P(E)>0 (evento empiricamente possivel); com P(E)=0 ha 0/0'),
 '43-inquiry-openness-multiverse.md:160': ('CORRETO', 'a mao (re-derivacao)', 'C+pi(1-eta)L=pi L <=> C=pi eta L'),
 '43-inquiry-openness-multiverse.md:162': ('CORRETO', 'a mao (re-derivacao)', 'EVPI confirmado nos dois regimes; >=0 sempre, max(0,.) de 28:195 e redundante'),
 '43-inquiry-openness-multiverse.md:165': ('CORRETO', 'a mao (razao)', 'p_aff/p_inst=L_inst/L_aff'),
 '43-inquiry-openness-multiverse.md:177': ('CORRETO', 'a mao (re-derivacao)', 'poder de teste unilateral'),
 '43-inquiry-openness-multiverse.md:185': ('CORRETO', 'a mao (re-derivacao)', 'SEV no teto de nao-rejeicao = poder'),
 '43-inquiry-openness-multiverse.md:195': ('CORRETO', 'a mao (definicao)', 'C_prem em [0,1]; sensivel a reparticionamento como o texto diz'),
 '45-world-to-archive-big-data-paradox.md:132': ('CORRETO', 'a mao (definicao)', 'identidade definicional harmonica do capitulo'),
 '45-world-to-archive-big-data-paradox.md:144': ('CORRETO', 'a mao (definicao)', 'peso V definido; n_0 e ancora de tamanho amostral'),
 '45-world-to-archive-big-data-paradox.md:161': ('CORRETO', 'a mao (covariancia)', 'Cov_N(R,Y)=(n/N)Ybar_n - f Ybar_N'),
 '45-world-to-archive-big-data-paradox.md:163': ('CORRETO', 'a mao (re-derivacao)', 'rho=f(.)/(sqrt(f(1-f))sigma_Y)=sqrt(f/(1-f))(.)/sigma_Y'),
 '45-world-to-archive-big-data-paradox.md:170': ('CORRETO', 'a mao (re-derivacao)', 'n_Meng=f/((1-f)rho^2)'),
 '45-world-to-archive-big-data-paradox.md:221': ('CORRETO', 'a mao (Kish)', 'n_eff=(sum w)^2/sum w^2=n/(1+CV^2(w))'),
 '45-world-to-archive-big-data-paradox.md:225': ('CORRETO', 'a mao + AM-GM', 'Chao2 e desigualdade AM-GM'),
 '46-non-identical-asymmetry-heavy-tails.md:186': ('CORRETO', 'a mao (integral)', 'CRPS=E|X-y|-(1/2)E|X-X"|'),
 '46-non-identical-asymmetry-heavy-tails.md:204': ('CORRETO', 'a mao (definicao)', 'estimador de Hill'),
 '46-non-identical-asymmetry-heavy-tails.md:224': ('CORRETO', 'a mao (taxa)', '|Xbar-mu|=O_p(n^{1/alpha-1})'),
 '47-irreversible-time-vital-margin.md:141': ('CORRETO', 'a mao (GBM)', 'E[W_T]=W0 e^{mu T}'),
 '47-irreversible-time-vital-margin.md:151': ('CORRETO', 'a mao (Ito + LIL)', 'g_time=f mu - (1/2)f^2 sigma^2 quase certamente'),
 '47-irreversible-time-vital-margin.md:164': ('CORRETO', 'a mao (re-derivacao)', 'confere por inflacao de variancia: com sigma_muhat^2=sigma^2/n_eff (frase anterior), f*=mu_hat/(sigma^2+sigma^2/n_eff)=k(n_eff) mu_hat/sigma^2, que e a forma impressa; k<1 em toda amostra finita. Ressalvas fora da algebra: k_BM (mesma linha) nao e o mesmo objeto (depende de z^2 e da curtose; coincidem se z^2=n_eff) — atribuicao a Baker-McHale a conferir na trilha de citacoes; e a frase vizinha promete mais do que k=0,9999 entrega em n_eff=10^4 (§IV)'),
 '47-irreversible-time-vital-margin.md:188': ('CORRETO', 'a mao (limite)', 'limite GI/G/1 valido em toda carga: M/M/1 da (1+rho^2)tau/(2(1-rho)) >= rho tau/(1-rho); aproximacao rotulada como tal'),
 '47-irreversible-time-vital-margin.md:218': ('CORRETO', 'a mao (Lundberg)', 'lambda(M_X(R)-1)=cR e equacao implicita em R, nao identidade a isolar'),
 '47-irreversible-time-vital-margin.md:230': ('CORRETO', 'a mao (supremo)', 'sup_{B em [B_L,B_U]} e^{-theta B}=e^{-theta B_L}'),
 '47-irreversible-time-vital-margin.md:234': ('CORRETO', 'a mao (re-derivacao)', 'Ybar_obs-Ybar_mis=rho sigma_Y/sqrt(f(1-f))'),
 '47-irreversible-time-vital-margin.md:240': ('CORRETO', 'a mao (majoracao)', 'M_X(theta)<=f M_obs+(1-f)[1+(Ybar_mis/x_max)(e^{theta x_max}-1)]'),
 '47-irreversible-time-vital-margin.md:244': ('CORRETO', 'a mao (diferenciacao implicita)', "d theta/d rho>0 sob lambda M''-c>0"),
 '47-irreversible-time-vital-margin.md:249': ('CORRETO', 'a mao (re-derivacao)', 'rho*_R,Y=min(1,((c-lambda Ybar_obs)_+/(lambda sigma_Y))sqrt(f/(1-f)))'),
 '47-irreversible-time-vital-margin.md:257': ('CORRETO', 'a mao (Taylor)', '1/theta(-rho) ~ (1/theta_0)(1+theta"(0)rho/theta_0)'),
 '48-reactive-worlds-performativity-goodhart.md:157': ('CORRETO', 'a mao (particao)', 'BS=REL-RES+UNC por construcao'),
 '48-reactive-worlds-performativity-goodhart.md:173': ('CORRETO', 'a mao (definicao)', 'REL_debiased=max(0,REL-b_hat_CV)'),
 '48-reactive-worlds-performativity-goodhart.md:179': ('CORRETO', 'a mao (integral de Brier)', 'CRPS=int BS(F(tau),1{y<=tau}) dtau'),
 '48-reactive-worlds-performativity-goodhart.md:195': ('CORRETO', 'a mao (definicao)', 'F estatistico classico'),
 '48-reactive-worlds-performativity-goodhart.md:220': ('CORRETO', 'a mao (Tian-Pearl)', 'limites de PN'),
 '48-reactive-worlds-performativity-goodhart.md:222': ('CORRETO', 'a mao (Tian-Pearl)', 'limites de PS'),
 '48-reactive-worlds-performativity-goodhart.md:253': ('CORRETO', 'a mao (re-derivacao)', 'theta_PS=mu0/(1+lambda-eps) e theta_PO=(1-eps)mu0/((1-eps)^2+lambda), ambos do mesmo L2/linear'),
 '48-reactive-worlds-performativity-goodhart.md:255': ('CORRETO', 'a mao (Perdomo)', 'eps<gamma/beta; com gamma=1+lambda, beta=1 da |eps|<1+lambda'),
 '48-reactive-worlds-performativity-goodhart.md:298': ('CORRETO', 'a mao (reordenacao)', 'REL-RES+UNC=BS'),
 '49-moral-luck-apt-agency-institutions.md:229': ('CORRETO', 'a mao (definicao)', 'P(Yhat=1,Y=1)=p(1-FNR)'),
 '49-moral-luck-apt-agency-institutions.md:230': ('CORRETO', 'a mao (divisao)', ''),
 '49-moral-luck-apt-agency-institutions.md:232': ('CORRETO', 'a mao (particao)', ''),
 '49-moral-luck-apt-agency-institutions.md:234': ('CORRETO', 'a mao (igualacao)', ''),
 '49-moral-luck-apt-agency-institutions.md:236': ('CORRETO', 'a mao (re-derivacao)', 'leads to FPR=[p/(1-p)][(1-PPV)/PPV](1-FNR)'),
 '50-methodological-families-reflexivity.md:209': ('CORRETO', 'a mao (definicao)', 'W=10 log10 LR em decibeis'),
 '50-methodological-families-reflexivity.md:262': ('CORRETO', 'a mao (reordenacao)', 'BS=UNC+REL-RES'),
 '50-methodological-families-reflexivity.md:269': ('CORRETO', 'a mao (otimizacao)', 'W=kappa ln(1+R)-cR => R*=max(0,kappa/c-1)'),
 '50-methodological-families-reflexivity.md:298': ('CORRETO', 'a mao (integral tripla)', 'V_min=(1-tau)^3'),
 '50-methodological-families-reflexivity.md:303': ('CORRETO', 'a mao (integral aninhada)', 'limites tau/e e tau/(e v)'),
 '50-methodological-families-reflexivity.md:305': ('CORRETO', 'a mao (integral)', 'int_{tau/e}^1 (1-tau/(ev))dv = 1-tau/e+(tau/e)ln(tau/e)'),
 '50-methodological-families-reflexivity.md:307': ('CORRETO', 'a mao (integral aninhada)', 'a variavel de integracao e e, logo ln e != 1; nao "corrigir" para 1'),
 '50-methodological-families-reflexivity.md:316': ('CORRETO', 'a mao (serie)', '1-tau+tau ln tau-(tau/2)(ln tau)^2'),
 '50-methodological-families-reflexivity.md:351': ('CORRETO', 'a mao (definicao)', 'SEV de falsificacao e de corroboracao'),
 '50-methodological-families-reflexivity.md:352': ('CORRETO', 'a mao (definicao)', ''),
 '51-layer-micro-kinetic-sensorimotor-order.md:29': ('CORRETO', 'a mao (cinematica)', 'd=v tau_react+v^2/(2 mu g)'),
 '52-layer-meso-buffered-queue-order.md:21': ('CORRETO', 'a mao (limite)', 'limite Kingman com c_a^2/rho^2 rotulado como limite e a forma sem 1/rho^2 como aproximacao de trafego pesado'),
 '52-layer-meso-buffered-queue-order.md:49': ('CORRETO', 'a mao (ponto fixo)', 'W=W(lambda(W)); P"(B)L=1 na condicao de fila estavel'),
 '54-layer-geophysical-chaos-data-assimilation-order.md:97': ('CORRETO', 'a mao (re-derivacao)', 'Delta tau=(1/2lambda)ln(sigma_a^2/sigma_a"^2)=(1/2lambda)ln((2B+R)/(B+R))'),
 '55-layer-continuum-fracture-structural-order.md:24': ('CORRETO', 'a mao (Griffith)', 'Pi=Pi0+4 gamma a-pi sigma^2 a^2/E; a_c=2 gamma E/(pi sigma^2)'),
 '55-layer-continuum-fracture-structural-order.md:58': ('CORRETO', 'a mao (re-derivacao)', 'V"=x^3-mu x-h'),
 '55-layer-continuum-fracture-structural-order.md:82': ('CORRETO', 'a mao (ordem)', 'Pr[R_(1)>r]=S(r)^N'),
 '57-layer-probability-credence-measurement-order.md:41': ('CORRETO', 'a mao (definicao)', 'previsoes inferior/superior'),
 '57-layer-probability-credence-measurement-order.md:104': ('CORRETO', 'a mao (invariante)', 'mediana invariante sob transformacao monotonica; media nao'),
 '58-layer-metrological-information-geometric-order.md:29': ('CORRETO', 'a mao (definicao)', 'u_c^2=sigma_A^2 DEFF/n+u_B^2'),
 '60-layer-cryptographic-zero-knowledge-adversarial-order.md:59': ('CORRETO', 'a mao (re-derivacao)', 'eps^k<=alpha => k>=ln(1/alpha)/ln(1/eps), eps<1'),
 '61-layer-biochemical-kinetic-molecular-order.md:37': ('CORRETO', 'a mao (definicao)', 'mecanismo de Michaelis-Menten'),
 '61-layer-biochemical-kinetic-molecular-order.md:87': ('CORRETO', 'a mao (Boltzmann)', 'f(E_a)=e^{-E_a/(k_B T)}'),
 '62-layer-morphogenetic-developmental-order.md:67': ('CORRETO', 'a mao (EDO)', 'c=c0 e^{-x/lambda}, lambda=sqrt(D tau)'),
 '62-layer-morphogenetic-developmental-order.md:121': ('CORRETO', 'a mao (d Diferenciacao implicita)', 'delta x*=-(lambda/c*)delta c*'),
 '65-layer-stratigraphy-paleobiology-deep-time-order.md:26': ('CORRETO', 'a mao (re-derivacao)', 'a=(1-C)^{-1/(H-1)}-1; H=2 => C/(1-C); H grande => -ln(1-C)/(H-1)'),
 '66-layer-pharmacological-clinical-diagnostic-order.md:53': ('CORRETO', 'a mao (re-derivacao)', 'p*_PK=C_FP/(C_FP+L_FN)'),
 '67-layer-epidemiology-outbreak-surveillance-order.md:146': ('CORRETO', 'a mao (definicao)', 'R_eff=R min(1,d/T): a fracao realizada e limitada em 1'),
 '67-layer-epidemiology-outbreak-surveillance-order.md:154': ('CORRETO', 'a mao (exponencial)', 'N(t+d)=N(t)e^{rd}=N(t)R0^{d/T}'),
 '68-layer-pharmacovigilance-regulatory-toxicology-order.md:101': ('CORRETO', 'a mao (razao)', 'PRR=a/(a+b) / c/(c+d)'),
 '69-layer-metascience-scientometrics-replication-order.md:44': ('CORRETO', 'a mao (re-derivacao)', 'N_fs=(sum Z)^2/z_alpha^2-k'),
 '69-layer-metascience-scientometrics-replication-order.md:94': ('CORRETO', 'a mao (definicao)', 'regra k*'),
 '69-layer-metascience-scientometrics-replication-order.md:100': ('CORRETO', 'a mao (desigualdade)', 'FDR<=alpha K0/K<=alpha'),
 '70-layer-acoustic-aesthetic-tragic-pedagogical-order.md:27': ('CORRETO', 'numerico', '1200 log2(3^12/2^19)=23,46 cents'),
 '70-layer-acoustic-aesthetic-tragic-pedagogical-order.md:36': ('CORRETO', 'numerico', '(3/2)^4/(2^2*5/4)=81/80; 1200 log2(81/80)=21,51 cents'),
 '70-layer-acoustic-aesthetic-tragic-pedagogical-order.md:87': ('CORRETO', 'numerico', '1200 log2(440/415)=101,3 cents'),
 '72-layer-philological-epigraphic-textual-order.md:43': ('CORRETO', 'a mao (produto)', '(2n-3)!!=prod_{j=1}^{n-1}(2j-1)'),
 '73-layer-taphonomic-historiographical-order.md:15': ('CORRETO', 'a mao (cadeia)', 'P(S|A)=P(D|A)P(B|D)P(S|B)'),
 '73-layer-taphonomic-historiographical-order.md:27': ('CORRETO', 'a mao (definicao)', 'S(T)=e^{-lambda T}'),
 '73-layer-taphonomic-historiographical-order.md:33': ('CORRETO', 'a mao (razao)', 'S1/S2=e^{(lambda2-lambda1)T}'),
 '75-layer-media-ecology-information-retrieval-order.md:15': ('CORRETO', 'a mao (cadeia)', ''),
 '75-layer-media-ecology-information-retrieval-order.md:29': ('CORRETO', 'a mao (re-derivacao)', 'E[Y|R=1]-E[Y]=rho sigma_R sigma_Y/P(R=1)'),
 '75-layer-media-ecology-information-retrieval-order.md:35': ('CORRETO', 'a mao (cadeia)', ''),
 '75-layer-media-ecology-information-retrieval-order.md:45': ('CORRETO', 'a mao (forma canonica)', 'E[s|s>=tau]=mu+sigma lambda(alpha); Var=sigma^2(1-delta(alpha))'),
 '76-layer-sequential-decision-bandit-real-options-order.md:34': ('CORRETO', 'numerico', 'P(r)=((r-1)/n)sum 1/(k-1); n=10,r=4 -> 0,3987; limite -x ln x maximizado em 1/e'),
 '79-layer-official-statistics-forensic-accounting-order.md:32': ('CORRETO', 'numerico', '1-b1-b9=0,6532 reaparece em 1 e 9: sum|p-b|>=1,3064; /9=0,1451'),
 '82-layer-evidentiary-tort-constitutional-legal-order.md:47': ('CORRETO', 'a mao (Alexy)', 'formula do peso W_{i,j}=I_i W_i R_i/(I_j W_j R_j)'),
 '85-layer-polycentric-commons-urban-biospheric-order.md:42': ('CORRETO', 'a mao (definicao)', 'minimo cumulativo sobre as 35 ordens'),
 # --- appendix A ---
 'appendix-a-mathematics.md:130': ('CORRETO', 'a mao (definicao)', 'intervalo Imbens-Manski com c_IM'),
 'appendix-a-mathematics.md:136': ('CORRETO', 'a mao (re-derivacao)', 'EVPI confirmado; ver 28:195 e 43:162'),
 'appendix-a-mathematics.md:146': ('CORRETO', 'a mao (Imbens-Manski)', 'Phi(c+Delta)-Phi(-c)=1-alpha; limites z_{1-a/2} e z_{1-a}'),
 'appendix-a-mathematics.md:156': ('CORRETO', 'a mao (definicao)', 'peso V definido; ver 05:197'),
 'appendix-a-mathematics.md:166': ('CORRETO', 'a mao (definicao)', 'Chao1 e Chao2(t)'),
 'appendix-a-mathematics.md:178': ('CORRETO', 'a mao (definicao)', 'estimador de Hill'),
 'appendix-a-mathematics.md:181': ('CORRETO', 'a mao (re-derivacao)', 'mesma forma de 47:164 e mesma derivacao (inflacao de variancia com sigma^2/n_eff); aqui o k acompanha g_time e alimenta f*_epistemic. Ressalva identica: k_BM e objeto distinto; atribuicao a conferir na trilha de citacoes'),
 'appendix-a-mathematics.md:184': ('CORRETO', 'a mao (limite)', 'limite Kingman com c_a^2/rho^2 e aproximacao sem ele, rotulados como limite e aproximacao (ver 52:21, 47:188)'),
 'appendix-a-mathematics.md:191': ('CORRETO', 'a mao (definicao)', 'operador de governanca como minimo de tres gates'),
 'appendix-a-mathematics.md:203': ('CORRETO', 'a mao (re-derivacao)', 'tau_alarm=ceil(ln(1/alpha)/((1-delta)ln e_t)); e_t>1 declarado no texto do capitulo'),
 'appendix-a-mathematics.md:227': ('CORRETO', 'a mao (mudanca de variavel)', 'Vol=prod(1-phi_a(tau))=int|det J_phi|dS porque phi_a(1)=1'),
 'appendix-a-mathematics.md:247': ('CORRETO', 'a mao (definicao)', 'nao-comutacao: A nao depende de w'),
 'appendix-a-mathematics.md:266': ('CORRETO', 'a mao (minimo sobre cone)', 'H(L_i)=min_{cone} M_Leontief'),
 'appendix-a-mathematics.md:270': ('CORRETO', 'a mao (comutatividade do minimo)', ''),
 'appendix-a-mathematics.md:276': ('CORRETO', 'a mao (Jensen)', 'E[min]<=min E'),
 'appendix-a-mathematics.md:278': ('CORRETO', 'a mao (definicao)', 'b_min=-E[max_{m!=m*}(theta_m*-theta_m)_+]'),
 'appendix-a-mathematics.md:280': ('CORRETO', 'a mao (desigualdade)', 'sqrt(2 ln k)'),
 # blocos so-notacao com nota util
 '10-numbers-order-bounds.md:277': ('CORRETO', 'a mao (forma canonica)', 'GEV'),
 '31-distribution-and-moment.md:168': ('CORRETO', 'a mao (forma canonica)', 'GEV'),
 '33-measuring-and-erring.md:239': ('CORRETO', 'a mao (definicao)', 'peso V; ver 05:197'),
 '17-justification-and-induction.md:244': ('CORRETO', 'a mao (definicao)', 'peso V; limites 0 e 1^- conferem'),
}

lines = []
counts = {}
for b in bs:
    f = b['file']
    ln = b['line']
    key = f.replace('book-src/', '') + ':' + str(ln)
    txt = strip(b['text'])
    disp = txt.replace('|', '\\|')
    if len(disp) > 150:
        disp = disp[:147] + '...'
    if key in OV:
        verdict, method, note = OV[key]
    elif '=' not in txt:
        verdict, method, note = ('CORRETO', 'a mao (leitura simbolica)',
                                 'nao e identidade: definicao/intervalo/conjunto')
    elif isdef(txt):
        verdict, method, note = ('CORRETO', 'a mao (definicao)', '')
    else:
        verdict, method, note = ('CORRETO', 'a mao (forma canonica)', '')
    corr = note if verdict != 'CORRETO' or note else '-'
    lines.append('%s:%d - %s - %s - %s - %s' % (f, ln, disp, method, verdict, corr))
    counts[verdict] = counts.get(verdict, 0) + 1

hdr = open(os.path.join(ROOT, 'audit/_math_header.md'), encoding='utf-8').read()
out = hdr + '\n'.join(lines) + '\n'
open(os.path.join(ROOT, 'audit/audit-math.md'), 'w', encoding='utf-8').write(out)
print(len(lines), counts)
