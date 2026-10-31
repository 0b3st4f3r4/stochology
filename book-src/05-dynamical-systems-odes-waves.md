# 5. Differential Equations, Runge-Kutta (RK4), Non-Equilibrium Thermodynamics, and Wave Superposition

> **Three tempos.** **Detachment:** set aside the picture of computation as discrete symbol manipulation detached from physics. **Abstraction:** many systems are open, dissipative, and continuous; their models must respect dynamics, not only frequencies. **Return:** ODEs integrated by RK4, entropy production, and wave superposition. **Negative proof:** none in this chapter; the tools here feed the proofs of later chapters.

## Why the Planet's Weather and Your Ear's Cochlea Do Not Use Binary Long Division

A digital computer simulates a weight bouncing on a spring or a sound wave traveling through air through discrete approximations:
1. It chops continuous time $t$ into discrete ticks $\Delta t$ and continuous voltage into 32-bit binary strings (`010000010...`).
2. If it uses a naive first-order step ($\mathbf{x}\_{n+1} = \mathbf{x}\_n + \Delta t \cdot \mathbf{f}(\mathbf{x}\_n)$, **Forward Euler**), every step overshoots the curved trajectory tangentially outward, **injecting artificial energy into the spring until the simulation diverges.**
3. Even when the digital computer uses a balanced 4th-order integrator (`RK4`), it must shuttle millions of 64-bit numbers back and forth between DRAM and the ALU ($640\text{ pJ}$ per DRAM access) to approximate what a physical capacitor and inductor ($L\ddot{q} + R\dot{q} + \frac{1}{C}q = V(t)$) or a continuous acoustic wave does **in zero clock cycles by obeying physical law**.

A second limitation appears in standard neural networks with piecewise-linear `ReLU` activations ($\mathrm{ReLU}(z) = \max(0, z)$). When trained to solve a physical wave equation $\frac{\partial^2 u}{\partial t^2} = c^2 \frac{\partial^2 u}{\partial x^2}$, a `ReLU` unit has a second derivative that is **identically zero everywhere** ($\frac{d^2}{dz^2}\mathrm{ReLU}(z) = 0$), leaving it blind to second-order curvature and wave interference.

Continuous physical and biological substrates (such as Prigogine's Brusselator chemical oscillator, Lovelock's Daisyworld biosphere ODEs, RLC-mechanical analog computers, Liquid Time-Constant (`LTC`) neural ODEs, and `SIREN` / Wave-Physics PDEs) compute directly through coupled differential equations, non-equilibrium entropy export, and wave superposition.

<figure class="stoch-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="1160" height="440" viewBox="0 0 1160 440" fill="none" role="img" aria-labelledby="title-fig-09-dynamics-waves">
  <title id="title-fig-09-dynamics-waves">Euler vs. RK4 Integration, Open Dissipative Structures, and Wave Superposition / PDEs</title>
  <defs>
  <style>text { font-family: "Inter", system-ui, -apple-system, "Segoe UI", Roboto, sans-serif; } .mono { font-family: "JetBrains Mono", ui-monospace, "Cascadia Code", monospace; }</style>
  </defs>
  <rect x="1" y="1" width="1158" height="438" rx="14" fill="var(--canvas)" stroke="var(--border)" stroke-width="1.5"/>

  <!-- Panel 1: Forward Euler Spiral Out vs RK4 4-Slope Integration -->
  <rect x="24" y="24" width="356" height="392" rx="12" fill="var(--elevated)" stroke="var(--sky)" stroke-width="2"/>
  <text x="44" y="54" fill="var(--sky)" class="mono" font-size="11.5" font-weight="800">1 · ODE INTEGRATION: EULER vs. RK4</text>
  <text x="44" y="76" fill="var(--ink)" font-size="14" font-weight="750">Why 4 Slopes (k₁, k₂, k₃, k₄) Preserve Orbits</text>
  <circle cx="196" cy="186" r="56" stroke="var(--emerald)" stroke-width="2.6" fill="none"/>
  <path d="M 252 186 L 252 130 L 180 114 L 112 178 L 104 268 L 210 296" stroke="var(--rose)" stroke-width="2" stroke-dasharray="5 3" fill="none"/>
  <text x="155" y="190" fill="var(--emerald)" class="mono" font-size="11" font-weight="800">Exact / RK4</text>
  <text x="220" y="114" fill="var(--rose)" class="mono" font-size="10.5" font-weight="700">Euler spirals out!</text>
  <rect x="44" y="274" width="316" height="122" rx="8" fill="var(--canvas)" stroke="var(--border)"/>
  <text x="58" y="298" fill="var(--sky)" class="mono" font-size="11" font-weight="800">Classical 4th-Order Runge-Kutta (RK4):</text>
  <text x="58" y="320" fill="var(--ink)" class="mono" font-size="10.5">k₁ = f(t, x)               (start slope)</text>
  <text x="58" y="338" fill="var(--ink)" class="mono" font-size="10.5">k₂ = f(t+h/2, x + h·k₁/2)  (midpoint trial 1)</text>
  <text x="58" y="356" fill="var(--ink)" class="mono" font-size="10.5">k₃ = f(t+h/2, x + h·k₂/2)  (midpoint trial 2)</text>
  <text x="58" y="374" fill="var(--emerald)" class="mono" font-size="10.5" font-weight="700">x_{n+1} = x_n + (h/6)(k₁ + 2k₂ + 2k₃ + k₄)</text>

  <!-- Panel 2: Open Systems & Hopf Limit Cycles -->
  <rect x="402" y="24" width="356" height="392" rx="12" fill="var(--elevated)" stroke="var(--emerald)" stroke-width="2"/>
  <text x="422" y="54" fill="var(--emerald)" class="mono" font-size="11.5" font-weight="800">2 · OPEN SYSTEMS &amp; HOPF BIFURCATIONS</text>
  <text x="422" y="76" fill="var(--ink)" font-size="14" font-weight="750">Prigogine's Dissipative Structures</text>
  <rect x="422" y="100" width="316" height="96" rx="8" fill="var(--canvas)" stroke="var(--emerald)" stroke-width="1.5"/>
  <text x="436" y="124" fill="var(--emerald)" class="mono" font-size="11.5" font-weight="800">dS = d_e S (flux) + d_i S (production ≥ 0)</text>
  <text x="436" y="148" fill="var(--ink)" font-size="11.5">By exporting entropy to the environment</text>
  <text x="436" y="168" fill="var(--ink)" font-size="11.5">(d_e S &lt; -d_i S &lt; 0), an open system</text>
  <text x="436" y="186" fill="var(--amber)" font-size="11.5" font-weight="700">decreases its net entropy (dS &lt; 0 → self-order)!</text>
  <rect x="422" y="212" width="316" height="114" rx="8" fill="var(--canvas)" stroke="var(--border)"/>
  <text x="436" y="236" fill="var(--sky)" class="mono" font-size="11.5" font-weight="800">2nd-Order State-Space Isomorphism:</text>
  <text x="436" y="260" fill="var(--ink)" class="mono" font-size="11">Mechanical: m ẍ + c ẋ + k x = F(t)</text>
  <text x="436" y="282" fill="var(--ink)" class="mono" font-size="11">Electrical: L q̈ + R q̇ + (1/C)q = V(t)</text>
  <text x="436" y="306" fill="var(--muted)" font-size="11">Identical ODE solved physically by Kirchhoff!</text>
  <text x="422" y="356" fill="var(--muted)" font-size="11.5">• Tr(J) &lt; 0: stable spiral attractor</text>
  <text x="422" y="378" fill="var(--muted)" font-size="11.5">• Tr(J) &gt; 0 (B &gt; 1+A²): sustained limit cycle</text>

  <!-- Panel 3: Wave Superposition & Second Derivatives -->
  <rect x="780" y="24" width="356" height="392" rx="12" fill="var(--elevated)" stroke="var(--violet)" stroke-width="2"/>
  <text x="800" y="54" fill="var(--violet)" class="mono" font-size="11.5" font-weight="800">3 · WAVE INTERFERENCE &amp; PDE OPERATORS</text>
  <text x="800" y="76" fill="var(--ink)" font-size="14" font-weight="750">Why Sinusoids Solve Wave PDEs Naturally</text>
  <path d="M 808 150 Q 848 100, 888 150 T 968 150 T 1048 150 T 1112 150" stroke="var(--sky)" stroke-width="2.5" fill="none"/>
  <text x="808" y="192" fill="var(--sky)" class="mono" font-size="11" font-weight="700">u(x,t) = A sin(kx - ωt + φ)</text>
  <rect x="800" y="210" width="316" height="106" rx="8" fill="var(--canvas)" stroke="var(--violet)" stroke-width="1.5"/>
  <text x="814" y="234" fill="var(--emerald)" class="mono" font-size="11" font-weight="800">SIREN Sinusoid σ(x) = sin(ω₀ x):</text>
  <text x="814" y="256" fill="var(--ink)" class="mono" font-size="11">σ'(x)  =  ω₀ cos(ω₀ x)</text>
  <text x="814" y="276" fill="var(--ink)" class="mono" font-size="11">σ''(x) = -ω₀² sin(ω₀ x)  ⇒  σ'' + ω₀² σ = 0 !</text>
  <text x="814" y="300" fill="var(--rose)" class="mono" font-size="11" font-weight="700">ReLU(x) = max(0,x)  ⇒  ReLU''(x) = 0 everywhere!</text>
  <text x="800" y="346" fill="var(--ink)" class="mono" font-size="11" font-weight="700">Wave PDE: ∂²u/∂t² = c²(x) ∇²u - γ ∂u/∂t + s</text>
  <text x="800" y="370" fill="var(--muted)" font-size="11.5">• Constructive (Δφ = 0): amplitudes add (2A)</text>
  <text x="800" y="390" fill="var(--muted)" font-size="11.5">• Destructive (Δφ = π): amplitudes cancel (0)</text>
  </svg>
  <figcaption>Figure 5.1 — Continuous dynamical systems, non-equilibrium thermodynamics, and wave superposition: (1) Why Forward Euler spirals outward on oscillators while 4th-order Runge-Kutta (RK4) preserves phase-space orbits; (2) Open thermodynamic systems and mechanical-electrical ODE isomorphism; and (3) Wave superposition and why sin(ω₀ x) is an eigenfunction of the second derivative operator.</figcaption>
</figure>

---

## 5.1 Dynamical Systems and Ordinary Differential Equations (ODEs)

### From Rates of Change to State-Space Vectors
Whenever a physical, biological, or economic state $\mathbf{x}(t) = (x\_1(t), \dots, x\_d(t))^\top \in \mathbb{R}^d$ evolves continuously in time, its instantaneous velocity vector is the time derivative $\dot{\mathbf{x}}(t) = \frac{d\mathbf{x}}{dt}$. An **autonomous system of first-order Ordinary Differential Equations (ODEs)** specifies this velocity as a function of the current state:

$$\frac{d\mathbf{x}}{dt} = \mathbf{f}(\mathbf{x})$$

### How to Convert Any Second-Order Physical Law into a First-Order State-Space System
Many laws of physics, such as Newton's Second Law $m\ddot{x} + c\dot{x} + kx = F(t)$ or Kirchhoff's RLC loop equation $L\ddot{q} + R\dot{q} + \frac{1}{C}q = V(t)$, involve a **second derivative** (acceleration $\ddot{x} = \frac{d^2 x}{dt^2}$).

We can solve a second-order ODE with a standard first-order solver by introducing velocity $v = \dot{x}$ as a **second state variable**. Defining the 2D state vector $\mathbf{z}(t) = \begin{bmatrix} x(t) \\ v(t) \end{bmatrix}$ transforms one second-order ODE into two coupled first-order ODEs:

$$\frac{d}{dt}\begin{bmatrix} x \\ v \end{bmatrix} = \begin{bmatrix} v \\ \frac{1}{m}\left(F(t) - c v - k x\right) \end{bmatrix}$$

A mechanical mass-spring-damper and an electrical RLC circuit (used in Chapter 13) share the same state-space structure:

| Physical Role | Mechanical Mass-Spring-Damper | Electrical Series RLC Circuit | Mathematical Term |
| --- | --- | --- | --- |
| **Generalized Coordinate** | Position $x(t)$ | Capacitor Charge $q(t)$ | State $z\_1(t)$ |
| **Generalized Velocity** | Velocity $v(t) = \dot{x}(t)$ | Current $I(t) = \dot{q}(t)$ | State $z\_2(t) = \dot{z}\_1(t)$ |
| **Kinetic Inertia** | Mass $m$ | Inductance $L$ | Coefficient of $\ddot{z}\_1$ |
| **Frictional Dissipation** | Viscous Damper $c$ | Resistance $R$ | Coefficient of $\dot{z}\_1$ |
| **Restoring Stiffness** | Spring Constant $k$ | Inverse Capacitance $1/C$ | Coefficient of $z\_1$ |

Because both physical systems obey the **exact same differential equation**, measuring the voltage across the capacitor in an RLC circuit solves the mechanical differential equation in real time at the speed of electromagnetic propagation.

---

## 5.2 Fixed Points, Jacobian Stability, and Hopf Bifurcations (Limit Cycles)

### Equilibrium (Fixed) Points and Linearization
A state $\mathbf{x}^\ast \in \mathbb{R}^d$ where all rates of change vanish simultaneously,

$$\mathbf{f}(\mathbf{x}^\ast) = \mathbf{0}$$

is called a **fixed point (equilibrium)** of the dynamical system. If the system is nudged slightly away from $\mathbf{x}^\ast$ by a small perturbation $\boldsymbol{\xi}(t) = \mathbf{x}(t) - \mathbf{x}^\ast$, does it return to $\mathbf{x}^\ast$ or move away?

Expanding $\mathbf{f}(\mathbf{x}^\ast + \boldsymbol{\xi})$ in a first-order Taylor series around $\mathbf{x}^\ast$ gives the linearized system:

$$\frac{d\boldsymbol{\xi}}{dt} \approx J(\mathbf{x}^\ast)\,\boldsymbol{\xi}, \qquad \text{where } J\_{ij}(\mathbf{x}^\ast) = \frac{\partial f\_i}{\partial x\_j}(\mathbf{x}^\ast)$$

is the $d \times d$ **Jacobian Matrix** at $\mathbf{x}^\ast$. For a 2D system ($d = 2$), the eigenvalues $\lambda\_{1,2}$ of $J$ are determined completely by its **trace** $\tau = \mathrm{tr}(J) = J\_{11} + J\_{22}$ and **determinant** $\Delta = \det(J) = J\_{11}J\_{22} - J\_{12}J\_{21}$ via the characteristic quadratic $\lambda^2 - \tau\lambda + \Delta = 0$:

$$\lambda\_{1,2} = \frac{\tau \pm \sqrt{\tau^2 - 4\Delta}}{2}$$

When $\Delta > 0$ and $\tau^2 < 4\Delta$, the eigenvalues are a complex-conjugate pair $\lambda\_{1,2} = \alpha \pm i\omega$ with real part $\alpha = \frac{\tau}{2}$:
1. **When $\tau = \mathrm{tr}(J) < 0$ ($\alpha < 0$):** Perturbations decay as $e^{\alpha t}\cos(\omega t)$, spiraling **inward** to a **stable equilibrium** (like a damped pendulum coming to rest).
2. **When $\tau = \mathrm{tr}(J) > 0$ ($\alpha > 0$):** Perturbations grow as $e^{\alpha t}\cos(\omega t)$, spiraling **outward** away from the unstable fixed point. If nonlinear terms bound the system at large amplitudes, the outward spiral settles onto a self-sustained periodic orbit called a **Limit Cycle**.
3. **Hopf Bifurcation ($\tau = \mathrm{tr}(J) = 0$):** The threshold where $\mathrm{tr}(J)$ crosses from negative to positive (turning a static equilibrium into a rhythmic oscillation such as the heartbeat, circadian rhythm, or Prigogine's Brusselator chemical oscillator at $B > 1 + A^2$ in Chapter 22) is a **Hopf Bifurcation**.

---

## 5.3 Numerical Integration: Why Forward Euler Fails and How `RK4` Works

When an ODE $\dot{\mathbf{x}} = \mathbf{f}(t, \mathbf{x})$ has no closed-form analytical solution, numerical integrators step from $\mathbf{x}\_n \approx \mathbf{x}(t\_n)$ to $\mathbf{x}\_{n+1} \approx \mathbf{x}(t\_n + h)$ over a small step size $h = \Delta t$.

### Why Forward Euler Artificially Gains Energy on Oscillators
The simplest scheme is **Forward Euler**, which follows the tangent slope at the start of the interval:

$$\mathbf{x}\_{n+1}^{\text{Euler}} = \mathbf{x}\_n + h\,\mathbf{f}(t\_n, \mathbf{x}\_n)$$

Consider the pure harmonic oscillator $\dot{x} = v, \; \dot{v} = -x$, whose true trajectories are circles $x(t)^2 + v(t)^2 = R^2$ of constant energy. One step of Forward Euler changes the squared radius as follows:

$$x\_{n+1}^2 + v\_{n+1}^2 = (x\_n + h v\_n)^2 + (v\_n - h x\_n)^2 = (1 + h^2)(x\_n^2 + v\_n^2) > x\_n^2 + v\_n^2$$

Every Forward Euler step multiplies the energy by $(1 + h^2) > 1$, causing the trajectory to spiral outward (Panel 1 of Figure 5.1) with global error $\mathcal{O}(h)$.

### The Classical 4th-Order Runge-Kutta (`RK4`) Algorithm
To cancel out first-, second-, and third-order curvature errors without needing higher derivatives of $\mathbf{f}$, Carl Runge and Wilhelm Kutta designed a 4-stage probe across the interval $[t\_n, t\_n + h]$:

1. **Stage 1 ($k\_1$, slope at the start):**
   $$\mathbf{k}\_1 = \mathbf{f}(t\_n, \; \mathbf{x}\_n)$$
2. **Stage 2 ($k\_2$, slope at the midpoint predicted by $k\_1$):**
   $$\mathbf{k}\_2 = \mathbf{f}\!\left(t\_n + \frac{h}{2}, \; \mathbf{x}\_n + \frac{h}{2}\mathbf{k}\_1\right)$$
3. **Stage 3 ($k\_3$, corrected slope at the midpoint predicted by $k\_2$):**
   $$\mathbf{k}\_3 = \mathbf{f}\!\left(t\_n + \frac{h}{2}, \; \mathbf{x}\_n + \frac{h}{2}\mathbf{k}\_2\right)$$
4. **Stage 4 ($k\_4$, slope at the far endpoint predicted by $k\_3$):**
   $$\mathbf{k}\_4 = \mathbf{f}(t\_n + h, \; \mathbf{x}\_n + h\mathbf{k}\_3)$$

Combining these four slopes with Simpson's $1\text{–}2\text{–}2\text{–}1$ quadrature weights gives the **`RK4` update**:

$$\mathbf{x}\_{n+1} = \mathbf{x}\_n + \frac{h}{6}\left(\mathbf{k}\_1 + 2\mathbf{k}\_2 + 2\mathbf{k}\_3 + \mathbf{k}\_4\right)$$

Because the midpoint slopes $\mathbf{k}\_2, \mathbf{k}\_3$ receive double weight and cancel the tangential overshoot of $\mathbf{k}\_1$, `RK4` achieves local truncation error $\mathcal{O}(h^5)$ and global error $\mathcal{O}(h^4)$. Cutting the step size $h$ by $10\times$ shrinks the integration error by **$10\text{,}000\times$**.

---

## 5.4 Open Non-Equilibrium Thermodynamics ($dS = d\_e S + d\_i S$)

Ilya Prigogine (1977 Nobel Prize) showed how a living cell, a biosphere, or a chemical clock can organize itself into rhythmic patterns without violating the Second Law of Thermodynamics. He divided the total entropy change $dS$ of an **open system** (a system that exchanges energy and matter with its surroundings) into two parts:

$$dS = d\_e S + d\_i S$$

1. **Internal Entropy Production ($d\_i S \ge 0$):** Inside the system, irreversible chemical reactions, diffusion, and friction always produce non-negative entropy ($d\_i S / dt \ge 0$, as required by the Second Law). For a chemical reaction network with forward flux $J\_\rho^+$ and reverse flux $J\_\rho^-$, Schnakenberg's formula gives:
   $$\frac{d\_i S}{dt} = k\_B \sum\_\rho \left(J\_\rho^+ - J\_\rho^-\right) \ln\!\left(\frac{J\_\rho^+}{J\_\rho^-}\right) \ge 0$$
   (Since $(a - b)\ln(a/b) \ge 0$ for all $a, b > 0$, this is zero if and only if every reaction is in detailed balance $J\_\rho^+ = J\_\rho^-$.)
2. **External Entropy Exchange ($d\_e S$, unrestricted in sign):** An open system absorbs high-grade free energy (such as sunlight or chemical feed $A, B$) and exports low-grade heat and waste to its environment, making $d\_e S < 0$.

Whenever the outward export of entropy exceeds internal production ($d\_e S < -d\_i S \le 0$), the net entropy of the system decreases ($dS = d\_e S + d\_i S < 0$) or settles into a non-equilibrium steady state ($dS = 0$ with $d\_i S = -d\_e S > 0$), sustaining **Dissipative Structures** far from thermal equilibrium.

In economics and social theory, **Nicholas Georgescu-Roegen (1971)** demonstrated in *The Entropy Law and the Economic Process* that this thermodynamic arrow separates **reversible mechanical time ($t$)**—in which a frictionless pendulum or neoclassical equilibrium equation can run backward ($t \to -t$) without loss—from **irreversible entropic and historical time ($T$)**, in which low-entropy matter-energy reserves are irrevocably degraded and past path-dependent shocks cannot be "un-lived" (Tension 6 of Filter III, developed in [Chapters 9](09-lenses.md), [22](22-relational-ontology.md), and [23](23-stochary-conjectural-cybernetics.md)).

---

## 5.5 Wave Superposition, Interference, and the Wave / Helmholtz PDEs

### Amplitude, Frequency, Phase, and Superposition
A continuous harmonic wave in space $x$ and time $t$ is written as:

$$u(x, t) = A \sin(k x - \omega t + \phi)$$

where $A \ge 0$ is the **amplitude** (peak height; energy scales as $A^2$), $k = 2\pi / \lambda$ is the **spatial wavenumber**, $\omega = 2\pi f$ is the **angular frequency**, and $\phi \in [0, 2\pi)$ is the **phase angle**.

When two waves $u\_1 = A\sin(\theta)$ and $u\_2 = A\sin(\theta + \Delta\phi)$ of the same frequency meet at a point in space, the trigonometric identity $\sin(\theta) + \sin(\theta + \Delta\phi) = 2\cos(\Delta\phi / 2)\sin(\theta + \Delta\phi / 2)$ produces **wave interference**:
- **Constructive Interference ($\Delta\phi = 0$, in phase):** $\cos(0) = 1$, so the combined amplitude **doubles** to $2A$ (and intensity quadruples to $4A^2$).
- **Destructive Interference ($\Delta\phi = \pi = 180^\circ$, out of phase):** $\cos(\pi/2) = 0$, so the two waves **cancel each other** ($u\_1 + u\_2 = 0$).

A single point in a wave medium simultaneously processes both **magnitude ($A$)** and **relative timing/sign ($\phi$)** without a digital logic gate.

### Why Sinusoids (`SIREN`) Solve Second-Order Wave and Helmholtz PDEs While `ReLU` Fails
In physics, wave propagation through a medium with spatially varying wave speed $c(\mathbf{x})$ and damping $\gamma(\mathbf{x})$ is governed by the **Second-Order Wave Partial Differential Equation (PDE)**:

$$\frac{\partial^2 u}{\partial t^2} = c(\mathbf{x})^2 \nabla^2 u - \gamma(\mathbf{x})\frac{\partial u}{\partial t} + s(\mathbf{x}, t)$$

For a steady-state oscillation $u(x, t) = \psi(x)e^{-i\omega t}$, this reduces to the spatial **Helmholtz Equation**:

$$\frac{d^2 \psi(x)}{dx^2} + \omega\_0^2 \psi(x) = s(x)$$

Comparing the second derivatives of two neural activation functions shows why periodic activations fit wave physics:
1. **Piecewise-Linear `ReLU` ($\sigma(x) = \max(0, x)$):**
   $$\sigma'(x) = \mathbf{1}\{x > 0\}, \qquad \sigma''(x) = \mathbf{0} \quad (\forall x \ne 0)$$
   Because $\sigma''(x) = 0$ everywhere, a `ReLU` network has **zero second derivative** and cannot represent the $\frac{d^2\psi}{dx^2}$ Laplacian term of the Helmholtz/Wave PDE.
2. **Sinusoidal Activation (`SIREN`, Sitzmann et al., $\sigma(x) = \sin(\omega\_0 x + b)$):**
   $$\sigma'(x) = \omega\_0 \cos(\omega\_0 x + b), \qquad \sigma''(x) = -\omega\_0^2 \sin(\omega\_0 x + b) = -\omega\_0^2 \sigma(x)$$
   The derivative of a `SIREN` network is another `SIREN` network (phase-shifted by $\pi/2$), and its second derivative is proportional to itself, matching the Helmholtz operator $\frac{d^2\psi}{dx^2} + \omega\_0^2 \psi$ natively.

### Exact Stability Amplification Factor: Why Forward Euler Spirals Outward on Every Harmonic Oscillator

To prove algebraically why Forward Euler injects artificial energy into an undamped harmonic oscillator ($\dot{x} = v, \; \dot{v} = -\omega^2 x$), combine the state vector $(x\_n, v\_n/\omega)$ into a single complex number $z\_n = x\_n + i(v\_n/\omega)$. The continuous ODE is simply $\dot{z} = -i\omega z$, whose exact solution $z(t) = z\_0 e^{-i\omega t}$ rotates around the unit circle with constant energy $|z(t)|^2 = |z\_0|^2$.

Applying one step of **Forward Euler** with step size $h$ multiplies $z\_n$ by the complex amplification factor $G\_{\text{Euler}}(i\omega h) = 1 - i\omega h$:

$$|z\_{n+1}|^2 = |1 - i\omega h|^2 \, |z\_n|^2 = \left(1 + \omega^2 h^2\right) |z\_n|^2 \implies E\_n = E\_0 \left(1 + \omega^2 h^2\right)^n \longrightarrow +\infty$$

Regardless of how small $h > 0$ is, $1 + \omega^2 h^2 > 1$, so Forward Euler exponentially inflates oscillator energy. By contrast, **4th-Order Runge-Kutta (`RK4`)** matches the Taylor series of $e^{-i\omega h}$ through degree $4$:

$$G\_{\mathrm{RK4}}(i\omega h) = 1 - i\omega h - \frac{(\omega h)^2}{2} + i\frac{(\omega h)^3}{6} + \frac{(\omega h)^4}{24} \implies |G\_{\mathrm{RK4}}(i\omega h)|^2 = 1 - \frac{(\omega h)^6}{72} + \frac{(\omega h)^8}{576}$$

reducing single-step amplitude drift from $\mathcal{O}((\omega h)^2)$ to $\mathcal{O}((\omega h)^6)$.

### Interactive Laboratory (`/lab`)

Use the interactive ODE and Brusselator simulator below to compare Forward Euler's parasitic energy explosion against `RK4` and explore the Hopf bifurcation threshold $B > 1 + A^2$:

<div class="stoch-lab" data-lab="rk4-hopf-brusselator" data-chapter="05-dynamical-systems-odes-waves.html" data-title="Chapter 5 Lab: Forward Euler vs. RK4 Phase Integrator &amp; Hopf Bifurcation" data-cli="cargo run --release --bin labs -- --lenses"></div>


---

## Computational Audit in Rust

All dynamical, thermodynamic, and wave-equation solvers built on these foundations live in [`crates/stochology-lenses/src/relational/mod.rs`](../crates/stochology-lenses/src/relational/mod.rs) and [`crates/stochology-lenses/src/analog/mod.rs`](../crates/stochology-lenses/src/analog/mod.rs):
- **`DissipativeAutopoieticAudit` (`relational/mod.rs`):** Integrates the 2D Brusselator ODE system via `RK4` and evaluates Schnakenberg's internal entropy production $d\_i S / dt > 0$.
- **`DaisyworldSystem` (`relational/mod.rs`):** Integrates the coupled Daisyworld biosphere-albedo ODEs via `RK4`.
- **`NativeOdeIsomorphismAudit` (`analog/mod.rs`):** Integrates the 2nd-order RLC / Mass-Spring-Damper ODE and Liquid Time-Constant (`LTC`) neural ODE via `RK4`.
- **`WaveInterferenceLearningAudit` (`analog/mod.rs`):** Compares `SIREN` $\sin(\omega\_0 x)$ against `ReLU` on the 2nd-order Helmholtz PDE and integrates the 2nd-order damped nonlinear Wave PDE.

Run Gate `G8` and Lab 3 to inspect these continuous systems:

~~~bash
cargo run --release --bin labs -- --lenses
cargo run --release --bin gates
~~~

## Exercises

1. **One Step of `RK4` vs. Forward Euler by Hand:** Consider the exponential decay ODE $\dot{x} = -x$ with initial condition $x(0) = 1.0$ and step size $h = 0.5$ (exact solution at $t = 0.5$ is $e^{-0.5} \approx 0.60653$).
   - Compute the Forward Euler approximation $x\_1^{\text{Euler}} = x\_0 + h(-x\_0)$.
   - Compute the four `RK4` slopes $k\_1, k\_2, k\_3, k\_4$ and verify that $x\_1^{\text{RK4}} = 1 + \frac{0.5}{6}(-1 + 2(-0.75) + 2(-0.8125) - 0.59375) \approx 0.60677$, reducing the error of Forward Euler ($0.1065$) by **more than $400\times$** in a single step.
2. **Hopf Bifurcation of the Brusselator:** For the Brusselator reactions $\dot{X} = A - (B + 1)X + X^2 Y$ and $\dot{Y} = BX - X^2 Y$, verify that $(X^\ast, Y^\ast) = (A, B/A)$ is the unique fixed point ($\dot{X} = \dot{Y} = 0$). Compute the $2 \times 2$ Jacobian matrix $J(X^\ast, Y^\ast) = \begin{bmatrix} B - 1 & A^2 \\ -B & -A^2 \end{bmatrix}$ and show that $\det(J) = A^2 > 0$ and $\mathrm{tr}(J) = B - 1 - A^2$, proving that the fixed point loses stability and enters a limit cycle precisely when $B > 1 + A^2$.
3. **Constructive vs. Destructive Interference:** Compute the superposition $u(t) = 3\sin(\omega t) + 3\sin(\omega t + \Delta\phi)$ at $\omega t = \pi/2$ for (a) $\Delta\phi = 0$, (b) $\Delta\phi = \pi/2$, and (c) $\Delta\phi = \pi$.

## Summary

- **Fourth-Order Runge-Kutta (`RK4`)** eliminates the parasitic $\mathcal{O}((\omega h)^2)$ energy drift of Forward Euler by combining four stage slopes per step with $\mathcal{O}(h^4)$ global accuracy.
- **Jacobian trace and determinant** govern local stability and **Hopf bifurcations** ($B > 1 + A^2$ in the Brusselator), where a fixed point gives way to a self-sustaining limit cycle.
- **Wave interference and periodic `SIREN` activations** preserve second-order spatial derivatives $\nabla^2 u$ required by Helmholtz and wave PDEs where piecewise-linear `ReLU` networks vanish almost everywhere.
