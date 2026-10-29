# 4. Markov Chains, Stochastic Time, and Causal Graphs (DAGs)


## When Controlling for a Variable Invents a Fake Disease

Suppose an emergency-room director wants to know whether having a **broken bone ($X \in \{0, 1\}$)** causes **pneumonia ($Y \in \{0, 1\}$)**.
- In the general city population of $10\text{,}000$ citizens, broken bones ($10\%$ prevalence) and pneumonia ($10\%$ prevalence) are **completely independent**: $P(Y=1 \mid X=1) = P(Y=1 \mid X=0) = 10\%$.
- However, the director does not sample the city street. Instead, he downloads the database of **hospitalized patients ($C = 1$)**, and a person is admitted to the hospital ($C = 1$) if they have *either* a broken bone ($X = 1$) *or* pneumonia ($Y = 1$).

When the hospital data scientist runs a regression inside the hospital database ($C = 1$), he finds a strong negative correlation:
- Among hospitalized patients **without** a broken bone ($X = 0, C = 1$), **$100\%$ have pneumonia ($Y = 1$)** (otherwise they would not be in the hospital).
- Among hospitalized patients **with** a broken bone ($X = 1, C = 1$), only **$10\%$ have pneumonia ($Y = 1$)**.

The data scientist announces: *"Breaking a bone cuts your risk of pneumonia by $90\%$."*

This trap is **Berkson's Paradox (Collider Bias)**. Consider the arrow structure: broken bones cause hospitalization ($X \to C$), and pneumonia also causes hospitalization ($Y \to C$). Thus $C$ is a **common effect (a collider)** $X \to C \leftarrow Y$. Conditioning on a common *cause* (a confounder $X \leftarrow U \to Y$) **removes** spurious correlation; conditioning on a common *effect* (a collider $X \to C \leftarrow Y$) **creates** spurious correlation where none existed.

Three structural mechanisms govern how information, time, and causal interventions propagate across stochastic systems:
1. **Markov Chains ($X \to Y \to Z$) and Stochastic Matrices $W$** (the backbone of the Information Lens and MCMC mixing);
2. **Multiplicative Compounding and the $(\Delta W\_t)^2 = \Delta t$ Scaling of Brownian Motion** (the backbone of the Time and Tail Lenses);
3. **Directed Acyclic Graphs (DAGs), $d$-Separation, and Graph Surgery $\mathrm{do}(X=x)$** (the backbone of the Causal Structure and Decision Lenses).

<figure class="stoch-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="1160" height="440" viewBox="0 0 1160 440" fill="none" role="img" aria-labelledby="title-fig-05-markov-dags">
  <title id="title-fig-05-markov-dags">Markov Chains, Brownian Quadratic Variation, and the Three Canonical DAG Junctions</title>
  <defs>
  <marker id="m-arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
  <path d="M 0 1 L 9 5 L 0 9 z" fill="var(--sky)"/>
  </marker>
  <marker id="m-rose" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
  <path d="M 0 1 L 9 5 L 0 9 z" fill="var(--rose)"/>
  </marker>
  <style>text { font-family: "Inter", system-ui, -apple-system, "Segoe UI", Roboto, sans-serif; } .mono { font-family: "JetBrains Mono", ui-monospace, "Cascadia Code", monospace; }</style>
  </defs>
  <rect x="1" y="1" width="1158" height="438" rx="14" fill="var(--canvas)" stroke="var(--border)" stroke-width="1.5"/>

  <!-- Panel 1: Markov Chains & Stochastic Matrices -->
  <rect x="24" y="24" width="356" height="392" rx="12" fill="var(--elevated)" stroke="var(--sky)" stroke-width="2"/>
  <text x="44" y="54" fill="var(--sky)" class="mono" font-size="11.5" font-weight="800">1 · MARKOV CHAINS (X → Y → Z)</text>
  <text x="44" y="76" fill="var(--ink)" font-size="14" font-weight="750">Memoryless Transition Matrices W</text>
  <circle cx="84" cy="145" r="26" fill="var(--canvas)" stroke="var(--sky)" stroke-width="2"/>
  <text x="78" y="150" fill="var(--ink)" class="mono" font-size="14" font-weight="800">X</text>
  <line x1="112" y1="145" x2="168" y2="145" stroke="var(--sky)" stroke-width="2.2" marker-end="url(#m-arr)"/>
  <text x="126" y="134" fill="var(--sky)" class="mono" font-size="11" font-weight="700">W₁</text>
  <circle cx="200" cy="145" r="26" fill="var(--canvas)" stroke="var(--emerald)" stroke-width="2"/>
  <text x="194" y="150" fill="var(--ink)" class="mono" font-size="14" font-weight="800">Y</text>
  <line x1="228" y1="145" x2="284" y2="145" stroke="var(--sky)" stroke-width="2.2" marker-end="url(#m-arr)"/>
  <text x="242" y="134" fill="var(--sky)" class="mono" font-size="11" font-weight="700">W₂</text>
  <circle cx="316" cy="145" r="26" fill="var(--canvas)" stroke="var(--amber)" stroke-width="2"/>
  <text x="310" y="150" fill="var(--ink)" class="mono" font-size="14" font-weight="800">Z</text>
  <text x="44" y="210" fill="var(--ink)" class="mono" font-size="11.5" font-weight="700">P(Z | X, Y) = P(Z | Y)   (X ⊥⊥ Z | Y)</text>
  <text x="44" y="236" fill="var(--muted)" font-size="11.5">• Future Z depends on past X ONLY through</text>
  <text x="44" y="254" fill="var(--muted)" font-size="11.5">  present state Y.</text>
  <text x="44" y="282" fill="var(--ink)" class="mono" font-size="11.5" font-weight="700">Row-Stochastic Matrix W_ij = P(Y=j | X=i):</text>
  <text x="44" y="306" fill="var(--emerald)" class="mono" font-size="11" font-weight="700">∑_j W_ij = 1,   q = p · W</text>
  <text x="44" y="334" fill="var(--muted)" font-size="11.5">• Multiplying by W averages rows together,</text>
  <text x="44" y="352" fill="var(--muted)" font-size="11.5">  contracting distances between distributions.</text>

  <!-- Panel 2: Stochastic Time & (ΔW_t)² = Δt -->
  <rect x="402" y="24" width="356" height="392" rx="12" fill="var(--elevated)" stroke="var(--emerald)" stroke-width="2"/>
  <text x="422" y="54" fill="var(--emerald)" class="mono" font-size="11.5" font-weight="800">2 · STOCHASTIC TIME &amp; QUADRATIC VARIATION</text>
  <text x="422" y="76" fill="var(--ink)" font-size="14" font-weight="750">Why (dW_t)² = dt Does Not Vanish</text>
  <rect x="422" y="100" width="316" height="92" rx="8" fill="var(--canvas)" stroke="var(--emerald)" stroke-width="1.5"/>
  <text x="436" y="124" fill="var(--emerald)" class="mono" font-size="11.5" font-weight="800">Random Walk Step: ΔW_t = ±√(Δt)</text>
  <text x="436" y="148" fill="var(--ink)" class="mono" font-size="11.5">E[ΔW_t] = 0,   but   (ΔW_t)² = (+√Δt)² = Δt !</text>
  <text x="436" y="172" fill="var(--amber)" font-size="11.5" font-weight="700">Squared noise is of order O(Δt), NOT O(Δt²)!</text>
  <rect x="422" y="208" width="316" height="114" rx="8" fill="var(--canvas)" stroke="var(--border)"/>
  <text x="436" y="232" fill="var(--sky)" class="mono" font-size="11.5" font-weight="800">Multiplicative Asymmetry (+50% vs -50%):</text>
  <text x="436" y="256" fill="var(--ink)" class="mono" font-size="11.5">$100 × 1.50 × 0.50 = $75  (-25% net loss!)</text>
  <text x="436" y="282" fill="var(--ink)" class="mono" font-size="11">Arithmetic Mean: ½(+50% - 50%) =  0.0%</text>
  <text x="436" y="304" fill="var(--rose)" class="mono" font-size="11" font-weight="700">Geometric Growth: √(1.5 × 0.5) - 1 = -13.4%</text>
  <text x="422" y="352" fill="var(--muted)" font-size="11.5">• Volatility σ² silently drains compounding</text>
  <text x="422" y="372" fill="var(--muted)" font-size="11.5">  trajectories by -½σ² per unit time.</text>

  <!-- Panel 3: The Three DAG Junctions -->
  <rect x="780" y="24" width="356" height="392" rx="12" fill="var(--elevated)" stroke="var(--violet)" stroke-width="2"/>
  <text x="800" y="54" fill="var(--violet)" class="mono" font-size="11.5" font-weight="800">3 · THE THREE CAUSAL DAG JUNCTIONS</text>
  <text x="800" y="76" fill="var(--ink)" font-size="14" font-weight="750">When to Condition vs. When Never to Condition</text>

  <!-- Chain -->
  <rect x="800" y="96" width="316" height="74" rx="8" fill="var(--canvas)" stroke="var(--border)"/>
  <text x="814" y="118" fill="var(--sky)" class="mono" font-size="11" font-weight="800">A. CHAIN (Mediator):  X → M → Y</text>
  <text x="814" y="138" fill="var(--ink)" font-size="11">Conditioning on M blocks the causal pipe X → Y.</text>
  <text x="814" y="156" fill="var(--muted)" font-size="10.5">Do NOT control for M if you want total effect of X!</text>

  <!-- Fork -->
  <rect x="800" y="182" width="316" height="82" rx="8" fill="var(--canvas)" stroke="var(--emerald)" stroke-width="1.5"/>
  <text x="814" y="204" fill="var(--emerald)" class="mono" font-size="11" font-weight="800">B. FORK (Confounder): X ← U → Y</text>
  <text x="814" y="224" fill="var(--ink)" font-size="11">Common cause U creates back-door path X ← U → Y.</text>
  <text x="814" y="244" fill="var(--emerald)" font-size="10.5" font-weight="700">MUST condition on U to isolate true causal effect!</text>

  <!-- Collider -->
  <rect x="800" y="276" width="316" height="92" rx="8" fill="var(--canvas)" stroke="var(--rose)" stroke-width="1.8"/>
  <text x="814" y="298" fill="var(--rose)" class="mono" font-size="11" font-weight="800">C. COLLIDER (Berkson): X → C ← Y</text>
  <text x="814" y="318" fill="var(--ink)" class="mono" font-size="11">Path is NATURALLY BLOCKED at C.</text>
  <text x="814" y="338" fill="var(--rose)" font-size="10.5" font-weight="700">Conditioning on C OPENS the path and invents</text>
  <text x="814" y="354" fill="var(--rose)" font-size="10.5" font-weight="700">spurious correlation between independent X &amp; Y!</text>
  </svg>
  <figcaption>Figure 4.1 — The three structural pillars of Markov chains, stochastic time, and causal graphs: (1) Memoryless Markov chains X → Y → Z and row-stochastic matrices W; (2) Quadratic variation (ΔW_t)² = Δt and multiplicative volatility drain; and (3) The three elementary DAG junctions (Chain, Fork/Confounder, and Collider/Berkson).</figcaption>
</figure>

---

## 4.1 Conditional Independence and Markov Chains ($X \to Y \to Z$)

### What Does Conditional Independence Mean?
Recall from the Preface that two random variables $X$ and $Z$ are marginally independent ($X \perp\!\!\!\perp Z$) if $P(X = x, Z = z) = P(X = x)P(Z = z)$.

Now suppose there is an intermediate state $Y$ between $X$ and $Z$. We say that **$X$ and $Z$ are conditionally independent given $Y$**, written:

$$X \perp\!\!\!\perp Z \mid Y$$

if, once you know the exact present state $Y = y$, learning the past state $X = x$ gives you **zero additional information** about the future state $Z = z$:

$$P(Z = z \mid X = x, Y = y) = P(Z = z \mid Y = y)$$

### Discrete-Time Markov Chains and Row-Stochastic Matrices $W$
A sequence of random variables $X \to Y \to Z$ satisfying $X \perp\!\!\!\perp Z \mid Y$ is called a **Markov Chain**: *given the present ($Y$), the future ($Z$) is independent of the past ($X$)*.

When the state space is finite with $m$ states $\{1, 2, \dots, m\}$, the step from $X$ to $Y$ is completely described by an $m \times m$ **Row-Stochastic Transition Matrix** $W$:

$$W\_{ij} = P(Y = j \mid X = i) \ge 0, \qquad \sum\_{j=1}^m W\_{ij} = 1 \quad \forall i \in \{1, \dots, m\}$$

Each row sums to $1$ because starting from state $X = i$, the system must land in *some* state $Y = j \in \{1, \dots, m\}$ with total probability $100\%$. If we represent a probability distribution over states as a row vector $\mathbf{p} = (p\_1, \dots, p\_m)$, then after one step through the channel $W$, the new probability distribution $\mathbf{q}$ is the vector-matrix product:

$$\mathbf{q} = \mathbf{p} W \qquad \left(q\_j = \sum\_{i=1}^m p\_i W\_{ij}\right)$$

And across a two-step Markov chain $X \xrightarrow{W\_1} Y \xrightarrow{W\_2} Z$, the composite transition matrix from $X$ to $Z$ is the matrix product $W\_{12} = W\_1 W\_2$ (**Chapman-Kolmogorov Equation**).

### Worked Example 4.1: How a Noisy Channel Mixes Distributions Together
Consider a binary communication channel (or noisy sensor) with symmetric flip error $\varepsilon = 0.20$:

$$W = \begin{bmatrix} 0.80 & 0.20 \\ 0.20 & 0.80 \end{bmatrix}$$

Suppose hypothesis $A$ puts $100\%$ probability on state $1$, $\mathbf{p} = (1, 0)$, and hypothesis $B$ puts $100\%$ probability on state $2$, $\mathbf{p}' = (0, 1)$. Before passing through $W$, the Total Variation distance between $\mathbf{p}$ and $\mathbf{p}'$ is $\frac{1}{2}\sum |p\_j - p'\_j| = 1.00$ (completely distinguishable).
After passing through $W$:
- $\mathbf{q} = \mathbf{p}W = (0.80, 0.20)$
- $\mathbf{q}' = \mathbf{p}'W = (0.20, 0.80)$

Now the Total Variation distance between $\mathbf{q}$ and $\mathbf{q}'$ has shrunk to $|0.80 - 0.20| = \mathbf{0.60} = 1 - 2\varepsilon$. Every noisy Markov step $W$ blends rows together, irreversibly contracting the distinguishability of distributions. In Chapter 9 (Lens 1), this matrix property yields the **Data Processing Inequality (DPI)** and the exact hierarchy of **Strong Contraction Constants** ($\eta\_{\chi^2}(W) \le \eta\_{\mathrm{KL}}(W) \le \eta\_{\mathrm{TV}}(W)$).

---

## 4.2 Multiplicative Compounding and the $(\Delta W\_t)^2 = \Delta t$ Rule of Stochastic Time

### Why Arithmetic Averages Lie About Multiplicative Trajectories
In an **additive** game, if you win $+\$50$ on Monday and lose $-\$50$ on Tuesday, your net change is $+50 - 50 = \$0$.

In a **multiplicative** system (compound interest, population biology, portfolio returns, evolutionary fitness), changes multiply your current state $W\_t$:
- Suppose you start with $W\_0 = \$100$.
- On Monday your wealth grows by $+50\%$ ($r\_1 = +0.50$), reaching $W\_1 = 100 \times 1.50 = \$150$.
- On Tuesday your wealth drops by $-50\%$ ($r\_2 = -0.50$), reaching $W\_2 = 150 \times 0.50 = \mathbf{\$75}$.

Compare the two averages:
- Your **arithmetic average return** per day is $\frac{+50\% + (-50\%)}{2} = \mathbf{0.0\%}$.
- Yet your actual wealth dropped by **$-25\%$** in two days (a geometric compounding rate of $\sqrt{1.50 \times 0.50} - 1 = \sqrt{0.75} - 1 \approx -13.4\%$ per day).

This divergence occurs because multiplication is linear in **logarithms**: $\ln(W\_T / W\_0) = \sum\_{t=1}^T \ln(1 + r\_t)$. As shown in Chapter 3, the function $f(x) = \ln(1 + x)$ is **strictly concave** ($f''(x) = -\frac{1}{(1+x)^2} < 0$). By our second-order Taylor expansion of Jensen's Inequality around $r = \mu$:

$$\mathbb{E}[\ln(1 + r)] \approx \mu - \frac{1}{2}\sigma^2$$

Every bit of variance $\sigma^2$ in a multiplicative process drags down the long-run growth rate by $-\frac{1}{2}\sigma^2$.

### Why Brownian Motion Has $(\Delta W\_t)^2 = \Delta t$ (The Intuition Behind Itô's Lemma)
In classical deterministic calculus, if a smooth variable $x(t)$ changes by $\Delta x = v\,\Delta t$ over a tiny time step $\Delta t$, then its square is $(\Delta x)^2 = v^2 (\Delta t)^2$, which vanishes much faster than $\Delta t$ as $\Delta t \to 0$. That is why ordinary calculus stops at the first derivative: $df(x) = f'(x)\,dx$.

Now consider a continuous-time **Brownian motion** (random walk) $W\_t$. Over a time interval of length $\Delta t$, the increment $\Delta W\_t = W\_{t+\Delta t} - W\_t$ has mean $0$ and variance equal to the elapsed time $\Delta t$:

$$\mathbb{E}[\Delta W\_t] = 0, \qquad \mathrm{Var}(\Delta W\_t) = \mathbb{E}[(\Delta W\_t)^2] = \Delta t$$

The relation $\mathbb{E}[(\Delta W\_t)^2] = \Delta t$ determines the size of a typical step:

$$\Delta W\_t = \pm \sqrt{\Delta t} \implies (\Delta W\_t)^2 = \left(\pm\sqrt{\Delta t}\right)^2 = \Delta t$$

**The square of a Brownian increment is of first order $\Delta t$, not $(\Delta t)^2$.** Therefore, when we expand any smooth function $f(X\_t)$ of a stochastic process in a Taylor series, we **cannot drop the second-order term** $\frac{1}{2}f''(X\_t)(\Delta X\_t)^2$:

$$df(X\_t) = f'(X\_t)\,dX\_t + \frac{1}{2}f''(X\_t)\,(dX\_t)^2$$

This equation, which follows directly from $(\pm\sqrt{\Delta t})^2 = \Delta t$, is **Itô's Lemma**, which we use in Chapter 9 (Lens 2) to derive exact time-average growth rates, optimal Kelly fractions, and absorbing ruin probabilities.

---

## 4.3 Directed Acyclic Graphs (DAGs): Chains, Forks, Colliders, and $\mathrm{do}(X = x)$

To represent cause-and-effect relationships mathematically without falling into Berkson's Paradox or confounding bias, Judea Pearl formulated causal structure as a **Directed Acyclic Graph (DAG)** $G = (V, E)$, where each node is a variable and each directed arrow $A \to B$ means that $A$ is a direct physical mechanism entering the structural equation $B = f\_B(A, \dots, \varepsilon\_B)$.

Every causal graph, regardless of size, is built out of **three elementary 3-node junctions** (illustrated in Panel 3 of Figure 4.1):

| Junction | Graph Structure | Real-World Example | Marginal State (Before Conditioning) | Effect of Conditioning on Middle Node | Operational Rule |
| --- | --- | --- | --- | --- | --- |
| **1. Chain (Mediation)** | $X \to M \to Y$ | Fire ($X$) $\to$ Smoke ($M$) $\to$ Alarm ($Y$) | $X$ and $Y$ are associated ($X \not\!\perp\!\!\!\perp Y$) | **Blocks** the path ($X \perp\!\!\!\perp Y \mid M$) | **Do NOT condition on $M$** if you want the total causal effect of $X$ on $Y$. |
| **2. Fork (Confounder)** | $X \leftarrow U \to Y$ | Heatwave ($U$) causes both Ice Cream Sales ($X$) and Sunburns ($Y$) | Spurious "back-door" association ($X \not\!\perp\!\!\!\perp Y$) | **Blocks** the back-door path ($X \perp\!\!\!\perp Y \mid U$) | **MUST condition on $U$** to eliminate confounding bias. |
| **3. Collider (Berkson)** | $X \to C \leftarrow Y$ | Broken Bone ($X$) $\to$ Hospitalized ($C$) $\leftarrow$ Pneumonia ($Y$) | Path is **naturally blocked** ($X \perp\!\!\!\perp Y$) | **OPENS** the path ($X \not\!\perp\!\!\!\perp Y \mid C$) | **NEVER condition on a collider $C$** (or any descendant of $C$). |

### Seeing vs. Doing: Observational $P(Y \mid X = x)$ vs. Interventional $P(Y \mid \mathrm{do}(X = x))$
Suppose a Fork confounder $X \leftarrow U \to Y$ is present:
1. **Seeing ($P(Y \mid X = x)$, Rung 1 of Pearl's Ladder):** When we passively observe $X = x$, information flows both **forward** along causal arrows $X \to Y$ AND **backward** along the back-door fork $X \leftarrow U \to Y$ (because observing high ice-cream sales $X$ tells us it is a hot day $U$, which predicts sunburns $Y$).
2. **Doing ($P(Y \mid \mathrm{do}(X = x))$, Rung 2 of Pearl's Ladder):** When we physically **intervene** to force $X := x$ (for example, handing out free ice cream on a random Tuesday), we perform **Graph Surgery**: we **delete all incoming arrows into $X$** (severing $U \to X$) while keeping all outgoing arrows $X \to Y$ intact.

When the confounder $U$ is observed, Pearl's **Back-Door Adjustment Formula** computes the interventional distribution from observational data by averaging over $U$:

$$P(Y = y \mid \mathrm{do}(X = x)) = \sum\_u P(Y = y \mid X = x, U = u) \, P(U = u)$$

When $U$ is partially unobserved or we have data across multiple environments $e \in \mathcal{E}$ (such as different seasons, hospitals, or experimental vs. observational cohorts), Chapter 9 (Lens 4) shows how **Invariant Causal Prediction (ICP)**, **Rosenbaum's $\Lambda^\ast$**, and **Cinelli-Hazlett's Robustness Value $RV\_q$** audit causal claims under environmental shifts and hidden confounding.

### Judea Pearl's Three Rules of `do-Calculus` and the Markov Blanket

To decide whether an interventional query $P(y \mid \mathrm{do}(x))$ can be computed purely from observational probabilities $P(v)$ without running a physical experiment, Judea Pearl (1995) proved that **three symbolic transformation rules** are sound and complete for any Directed Acyclic Graph $G$ (where $G\_{\overline{X}}$ denotes $G$ with all arrows *entering* $X$ deleted, and $G\_{\underline{X}}$ denotes $G$ with all arrows *leaving* $X$ deleted):

1. **Rule 1 (Insertion/Deletion of Observations):**
   $$P(y \mid \mathrm{do}(x), z, w) = P(y \mid \mathrm{do}(x), w) \qquad \text{if } (Y \perp\!\!\!\perp Z \mid X, W)\_{G\_{\overline{X}}}$$
2. **Rule 2 (Action/Observation Exchange — Back-Door Criterion):**
   $$P(y \mid \mathrm{do}(x), \mathrm{do}(z), w) = P(y \mid \mathrm{do}(x), z, w) \qquad \text{if } (Y \perp\!\!\!\perp Z \mid X, W)\_{G\_{\overline{X}\underline{Z}}}$$
3. **Rule 3 (Insertion/Deletion of Actions):**
   $$P(y \mid \mathrm{do}(x), \mathrm{do}(z), w) = P(y \mid \mathrm{do}(x), w) \qquad \text{if } (Y \perp\!\!\!\perp Z \mid X, W)\_{G\_{\overline{X}\overline{Z(W)}}}$$

Furthermore, for any target node $Y$ in a DAG, its **Markov Blanket** $\mathrm{MB}(Y)$ consists of its **direct parents** $\mathrm{Pa}(Y)$, its **direct children** $\mathrm{Ch}(Y)$, and the **co-parents** of its children. While conditioning on the full Markov Blanket renders $Y$ conditionally independent of the rest of the graph, **Eliminatory Filter 2 (`evaluate_filter_2_independence_and_blanket`) strictly forbids conditioning on children or co-parents when estimating a causal effect**, because conditioning on a common child opens a **Berkson collider path**.

### Interactive Laboratory (`/lab`)

Use the interactive Markov contraction and Pearl `do-calculus` simulator below to observe Dobrushin total-variation contraction $\eta\_{\mathrm{TV}}^t$ alongside confounding and Berkson collider bias:

<div class="stoch-lab" data-lab="markov-dag-do" data-chapter="04-markov-chains-causal-graphs.html" data-title="Chapter 4 Lab: Markov Contraction (η_TV), Pearl do(X=x) &amp; Berkson Collider Guard" data-cli="cargo run --release --bin labs -- --lenses"></div>


---

## Computational Audit in Rust

The building blocks of this chapter power the pure-Rust engines in [`crates/stochology-lenses`](../crates/stochology-lenses/src/lib.rs) and [`crates/stochology-siga`](../crates/stochology-siga/src/lib.rs):
- **`DpiMarkovChain` (`crates/stochology-lenses/src/information.rs`):** Constructs row-stochastic matrices $W\_1, W\_2$ for 3-step Markov chains $X \to Y \to Z$ and verifies contraction constants.
- **`TimeKellyDiagnostic` (`crates/stochology-lenses/src/time.rs`):** Simulates multiplicative trajectories and measures the Itô volatility drain $-\frac{1}{2}\sigma^2$.
- **`FunnelCandidate`, `FunnelResult` & `evaluate_four_filter_funnel` (`crates/stochology-siga/src/funnel/mod.rs`):** Enforces Markov Blanket hygiene and rejects any model that conditions on a Berkson collider (`colliders_conditioned_on == 0`).

Run Lab 3 and Lab 5 to inspect these mechanisms in action:

~~~bash
cargo run --release --bin labs -- --lenses
cargo run --release --bin labs -- --siga
~~~

## Exercises

1. **Row-Stochastic Matrix Multiplication:** Given initial state distribution $\mathbf{p} = (0.9, 0.1)$ and transition matrix $W = \begin{bmatrix} 0.7 & 0.3 \\ 0.4 & 0.6 \end{bmatrix}$:
   - Compute $\mathbf{q} = \mathbf{p}W$ after one step and check that $q\_1 + q\_2 = 1$.
   - Find the stationary distribution $\boldsymbol{\pi} = (\pi\_1, \pi\_2)$ satisfying $\boldsymbol{\pi} W = \boldsymbol{\pi}$ and $\pi\_1 + \pi\_2 = 1$.
2. **Numerical Proof of Berkson's Collider Paradox:** Suppose two independent fair coins $X \in \{0, 1\}$ and $Y \in \{0, 1\}$ ($P(X=1)=0.5$, $P(Y=1)=0.5$) determine whether a bell rings: $C = \mathbf{1}\{X + Y \ge 1\}$.
   - Write out the $4$ equally likely outcomes $(X, Y) \in \{(0,0), (0,1), (1,0), (1,1)\}$ and verify that $\mathrm{Cov}(X, Y) = 0$ across all $4$ rows.
   - Now filter to the $3$ rows where the bell rang ($C = 1$): compute $P(Y = 1 \mid X = 0, C = 1)$ vs. $P(Y = 1 \mid X = 1, C = 1)$, and show that conditioning on the collider $C = 1$ creates a negative correlation $\rho\_{X, Y \mid C=1} = -0.50$.
3. **Arithmetic vs. Geometric Compounding:** An asset alternates deterministically between $+60\%$ in odd years and $-40\%$ in even years. Compute its arithmetic average annual return vs. its 2-year multiplier $(1.60 \times 0.60)$, and determine how much wealth remains after $20$ years starting from $\$1\text{,}000$.

## Summary

- **Row-stochastic Markov kernels** $W$ contract state distributions at a rate bounded by Dobrushin's total-variation coefficient $\eta\_{\mathrm{TV}}(W) \le 1$, enforcing the **Data Processing Inequality** across processing pipelines.
- **Multiplicative dynamics** suffer from the **Itô volatility drain** $g\_{\text{time}} = \mu\_{\text{ens}} - \frac{1}{2}\sigma^2$, separating the ensemble average from the time-average trajectory of a single agent.
- **Pearl's Causal Ladder** separates observational conditioning $P(Y \mid X = x)$ (Rung 1) from graph-surgery intervention $P(Y \mid \mathrm{do}(X = x))$ (Rung 2), while **Filter 2** bans conditioning on common-effect colliders (Berkson's paradox).
