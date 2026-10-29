# 14. The Triadic Synthesis

## When Phase, Scale, and Recurrence Meet in One Physical Machine

Suppose an optical-analog radar processor must reconstruct a scattered wave field, invert an $8 \times 8$ complex MIMO channel on drifted resistive memory cells, and update a closed-loop controller without numerical overflow. Treating each step with standard real-valued, thin-tailed, unbounded Euclidean algebra breaks down: piecewise-linear activations erase the Helmholtz wave operator, resistive drift corrupts one-shot matrix inversion, heavy-tailed waiting times violate Gaussian diffusion, and explicit Euler integration explodes on stiff time steps. Resolving all four failures simultaneously requires uniting the **complex plane ($\mathbb{C}$)**, **fractal anomalous scaling**, and **compact toroidal phase manifolds ($\mathbb{T}^d$)**.

## 14.1 The Triadic Mathematical & Physical Synthesis ($\mathbb{C} \times \text{Fractals} \times \mathbb{T}^d$), the Technium / Technosphere Ontology, and the Spring-and-Horizon Spiral (`stochology_lenses::triad`)

The module `stochology_lenses::triad` (`ComplexFractalToroidalTechniumSynthesis` and 5-CSV real coupler `EmpiricalTriadicTechniumAudit`) unites six mathematical-physical bridges across the complex plane ($\mathbb{C}$), fractal self-similarity and anomalous scaling, and compact toroidal manifolds ($\mathbb{T}^d = (S^1)^d$), coupled to the evolutionary ontology of the **Technium / Technosphere**. Every bridge separates three kinds of quantities. **Claim checks** compare the *deployed* method with simulated ground truth; in `shielded = false` mode the naive method is actually run, so `--defect triadic_technium` blocks on the naive outputs. **Identity checks** (`identities_closed`) are algebraic closures that hold in both modes. **Informative constants** transcribed from the literature are reported and never gated. Claims that fail their pre-registered tolerance are recorded as **discarded findings** (Prohibition 4) instead of being approved.

### Foundational Intuition: Why Complex Numbers ($\mathbb{C}$), Fractals, and Tori ($\mathbb{T}^d$) Form One Coherent Triad

Before inspecting the six bridges, notice why these three mathematical structures naturally complete one another:
1. **The Complex Plane ($\mathbb{C}$ — Amplitude and Phase Together):**
   By **Euler's Formula**, $z = r e^{i\theta} = r(\cos\theta + i\sin\theta)$, a single complex number stores both **energy/magnitude** ($r = |z| \ge 0$) and **angular phase/timing** ($\theta = \arg(z) \in S^1$). Multiplying two complex numbers multiplies their magnitudes and *adds* their phases ($r\_1 e^{i\theta\_1}\cdot r\_2 e^{i\theta\_2} = r\_1 r\_2 e^{i(\theta\_1+\theta\_2)}$). Furthermore, every complex number $a + ib$ is algebraically isomorphic to the $2\times 2$ real rotation-scaling matrix $\begin{pmatrix}a & -b\\ b & a\end{pmatrix}$, allowing complex wave/MIMO systems to execute natively on real analog RRAM crossbars (`Bridge 2`).
2. **Fractals and Fractional Calculus (Scale-Invariance and Power-Law Memory):**
   Whereas smooth textbook calculus assumes thin-tailed Gaussian jumps ($\Delta x \sim (\Delta t)^{1/2}$) and exponential relaxation ($e^{-t/\tau}$), real physical defects in amorphous oxides (`RRAM`/`PCM`), turbulent atmosphere tracks, and financial volatility cascades exhibit **power-law waiting times** $\psi(t) \sim t^{-1-\beta}$ and **heavy-tailed Lévy jumps** ($\sim |x|^{-1-\alpha}$). This produces **fractional subdiffusion** ($\partial\_t^\beta p = D\partial\_x^\alpha p$, spreading as $t^{\beta/\alpha}$) and multifractal scaling (`Bridges 3 & 4`).
3. **The $d$-Dimensional Torus ($\mathbb{T}^d = (S^1)^d = (\mathbb{R}/\mathbb{Z})^d$ — Compact Phase Recurrence Without Overflow):**
   As phase $\theta(t) = (\omega\_1 t, \dots, \omega\_d t) \bmod 1$ winds around the unit circle in each coordinate, a trajectory can evolve forever inside a bounded, compact state space without diverging to infinity. When the rotation increment $\gamma$ is an irrational multiple of the golden ratio $\phi^{-1} = \frac{\sqrt{5}-1}{2}$ (Weyl's equidistribution theorem), the trajectory fills the torus uniformly (`SplitMix64` / `LXM` in `Bridge 5`), and high-dimensional phase vectors $\mathbf{z} \in \mathbb{T}^D \subset \mathbb{C}^D$ bind symbolic concepts via exact unitary phase addition (`FHRR` and `GC-VSA` in `Bridge 6`; rotary position embeddings, RoPE, rest on the same phase addition but are not implemented in this repository).

<figure class="stoch-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="1140" height="330" viewBox="0 0 1140 330" fill="none" role="img" aria-labelledby="title-fig-13b-triad">
  <title id="title-fig-13b-triad">The Six-Bridge Triadic Synthesis Across the Complex Plane (C), Fractals, and the d-Torus (T^d) Coupled to the Technosphere</title>
  <defs>
    <style>text { font-family: "Inter", system-ui, -apple-system, "Segoe UI", Roboto, sans-serif; } .mono { font-family: "JetBrains Mono", ui-monospace, "Cascadia Code", monospace; }</style>
  </defs>
  <rect x="1" y="1" width="1138" height="328" rx="14" fill="var(--canvas)" stroke="var(--border)" stroke-width="1.5"/>
  <!-- Row 1: Bridges 1, 2, 3 -->
  <rect x="20" y="20" width="354" height="134" rx="11" fill="var(--elevated)" stroke="var(--sky)" stroke-width="2"/>
  <text x="36" y="44" fill="var(--sky)" class="mono" font-size="10.5" font-weight="800">BRIDGE 1 · ℂ WAVE PHYSICS &amp; GABOR INRs</text>
  <text x="36" y="68" fill="var(--ink)" font-size="13.5" font-weight="800">2D Helmholtz PDE &amp; Adjoint Photonics</text>
  <text x="36" y="92" fill="var(--muted)" class="mono" font-size="11">WIRE/LSSA Gabor res = 1.0×10⁻¹¹ vs tanh 0.609</text>
  <text x="36" y="112" fill="var(--muted)" class="mono" font-size="11">Improved SIREN {sin,cos} = 1.6×10⁻¹⁵</text>
  <text x="36" y="132" fill="var(--muted)" class="mono" font-size="11">Hughes-Fan Adjoint grad err = 9.21×10⁻¹⁰</text>

  <rect x="393" y="20" width="354" height="134" rx="11" fill="var(--elevated)" stroke="var(--violet)" stroke-width="2"/>
  <text x="409" y="44" fill="var(--violet)" class="mono" font-size="10.5" font-weight="800">BRIDGE 2 · ℂ ↔ FRACTALS &amp; H-MATRICES</text>
  <text x="409" y="68" fill="var(--ink)" font-size="13.5" font-weight="800">ℂⁿˣⁿ ↪ ℝ²ⁿˣ²ⁿ BlockAMC &amp; Hackbusch</text>
  <text x="409" y="92" fill="var(--muted)" class="mono" font-size="11">Isomorphism id_err = 1.12×10⁻¹⁶</text>
  <text x="409" y="112" fill="var(--muted)" class="mono" font-size="11">2-Stage BlockAMC+HP-INV = 1.53×10⁻¹⁰</text>
  <text x="409" y="132" fill="var(--muted)" class="mono" font-size="11">Admissible H-Matrix rank-3 = 9.6×10⁻⁶</text>

  <rect x="766" y="20" width="354" height="134" rx="11" fill="var(--elevated)" stroke="var(--amber)" stroke-width="2"/>
  <text x="782" y="44" fill="var(--amber)" class="mono" font-size="10.5" font-weight="800">BRIDGE 3 · FRACTAL CTRW &amp; ANALOG ECC</text>
  <text x="782" y="68" fill="var(--ink)" font-size="13.5" font-weight="800">G₀ = 2e²/h = 77.48 μS &amp; Roth Codes</text>
  <text x="782" y="92" fill="var(--muted)" class="mono" font-size="11">PCM drift comp MSE: 0.359 → 0.0019</text>
  <text x="782" y="112" fill="var(--muted)" class="mono" font-size="11">CTRW subdiffusion spread = 2.80× (16^β/α)</text>
  <text x="782" y="132" fill="var(--muted)" class="mono" font-size="11">Trimmed Analog ECC recovery = 100% (h₈=28.74)</text>

  <!-- Row 2: Bridges 4, 5, 6 -->
  <rect x="20" y="174" width="354" height="134" rx="11" fill="var(--elevated)" stroke="var(--rose)" stroke-width="2"/>
  <text x="36" y="198" fill="var(--rose)" class="mono" font-size="10.5" font-weight="800">BRIDGE 4 · SCALING &amp; TAILWELSCH-DML</text>
  <text x="36" y="222" fill="var(--ink)" font-size="13.5" font-weight="800">Refusal Gate, Welsch-IRLS &amp; Basel π²/6</text>
  <text x="36" y="246" fill="var(--muted)" class="mono" font-size="11">Bounded-tail refusal gate (&lt; 0.70) active</text>
  <text x="36" y="266" fill="var(--muted)" class="mono" font-size="11">TailWelsch-DML ATE RMSE: 0.200 → 0.048 (-76%)</text>
  <text x="36" y="286" fill="var(--muted)" class="mono" font-size="11">Accumulate Σg⁻² = 1.5274 &lt; π²/6 = 1.6449</text>

  <rect x="393" y="174" width="354" height="134" rx="11" fill="var(--elevated)" stroke="var(--emerald)" stroke-width="2"/>
  <text x="409" y="198" fill="var(--emerald)" class="mono" font-size="10.5" font-weight="800">BRIDGE 5 · 𝕋ᵈ WEYL TORUS &amp; RKHS DFM</text>
  <text x="409" y="222" fill="var(--ink)" font-size="13.5" font-weight="800">Z/16Z Cycle, Golden Weyl &amp; Label Shift</text>
  <text x="409" y="246" fill="var(--muted)" class="mono" font-size="11">x₀=2026 mod 16=10 full 16-state cycle</text>
  <text x="409" y="266" fill="var(--muted)" class="mono" font-size="11">Golden Weyl D*₂₅₆ = 0.0074 vs 1/8 = 0.125</text>
  <text x="409" y="286" fill="var(--muted)" class="mono" font-size="11">Hermite-Mehler DFM L₂ = 0.0136 vs RFF 0.104</text>

  <rect x="766" y="174" width="354" height="134" rx="11" fill="var(--elevated)" stroke="var(--sky)" stroke-width="2"/>
  <text x="782" y="198" fill="var(--sky)" class="mono" font-size="10.5" font-weight="800">BRIDGE 6 · SNA, FHRR, LTC &amp; TECHNIUM</text>
  <text x="782" y="222" fill="var(--ink)" font-size="13.5" font-weight="800">Strange Nonchaotic Torus &amp; Fused LTC</text>
  <text x="782" y="246" fill="var(--muted)" class="mono" font-size="11">SNA λ = -0.045 &lt; 0, roughness μ = 0.53 &gt; 0.15</text>
  <text x="782" y="266" fill="var(--muted)" class="mono" font-size="11">LTC Fused |x| ≤ 1.33 at Δt=2.5 vs Euler 6.6×10¹⁴</text>
  <text x="782" y="286" fill="var(--muted)" class="mono" font-size="11">FHRR T¹²⁸ unbind = 0.66 · GC-VSA = 7.1×10⁻¹⁵</text>
  </svg>
  <figcaption><strong>Figure 14.1 — The Six-Bridge Triadic Mathematical &amp; Physical Synthesis (`stochology_lenses::triad`) audited in Gate G8 and `--defect triadic_technium`.</strong></figcaption>
</figure>

1. **Bridge 1 ($\mathbb{C}$ — Complex Gabor INRs on the 2D Helmholtz PDE & Adjoint *In-Situ* Photonic Backpropagation — `ComplexWaveGaborHelmholtzAudit`):**
   - Solves $(\nabla\_h^2 + k^2)\Phi = q\_h$ at $k = 12$ on an $81 \times 81$ five-point grid. A complex Gabor atom (`WIRE`, $\psi(x) = \exp(i\omega x - |s\_0 r|^2)$) whose frequency is learned by 60 gradient-ascent steps from $\omega\_0 = k/2$ (`LSSA`-style spectral adaptation, frequency error $7.7 \times 10^{-7}$) leaves a relative residual of **$1.0 \times 10^{-11}$**, against **$0.609$** for a spectrally-biased real `tanh` dictionary fitted by least squares.
   - Because the target lies in the Gabor class, this claim checks frequency learning. The `Improved SIREN` $\{\sin, \cos\}$ pair fits every phase of $e^{-s\_0^2x^2}\cos(kx + \varphi)$ (worst residual $1.6 \times 10^{-15}$); that is a trigonometric identity, checked under `identities_closed`, and the single-phase failure at $\varphi = 0$ (residual $1.0$) is informative. `MIRE`'s reported PSNR gain is carried as an informative literature value.
   - Identity: Hughes, Minkov, Shi & Fan's (*Optica* 2018) adjoint gradient $\mathrm{Re}\{k\_0^2 E\_{\mathrm{fwd}} E\_{\mathrm{adj}}\}$ matches central finite differences to $9.21 \times 10^{-10}$.
2. **Bridge 2 ($\mathbb{C} \leftrightarrow \text{Fractals}$ — Complex-to-Real Isomorphism, Two-Stage `BlockAMC`, & Hierarchical Compression — `ComplexRealIsomorphismHMatrixAudit`):**
   - Identity: the complex $4 \times 4$ inverse recovered from $\mathcal{I}(A) = \begin{pmatrix}\mathrm{Re}(A) & -\mathrm{Im}(A)\\ \mathrm{Im}(A) & \mathrm{Re}(A)\end{pmatrix}$ closes to $1.12 \times 10^{-16}$.
   - Two-stage `BlockAMC` with 10 24-bit `HP-INV` refinements solves the $16 \times 16$ real (8x8 complex MIMO) system to **$1.53 \times 10^{-10}$**; one-shot inversion stops at $5.7 \times 10^{-2}$. The 16-tile count and the $+48.83\%$ area / $+40\%$ energy gains are informative.
   - Rank-3 hierarchical approximation of $K\_{ij} = 1/(y\_j - x\_i)$ over a near (inadmissible) and a far ($\eta = 1$ admissible) source cluster: compressing only admissible blocks gives relative Frobenius error **$9.6 \times 10^{-6}$**; compressing every block diverges ($62.0$). The closed-form storage $(2p+1)n$ ($144$ vs. $256$ at $p = 4$) is informative.
3. **Bridge 3 ($\text{Fractals} \leftrightarrow \mathbb{R}^n$ — $G\_0$, PCM Drift, Simulated `CTRW`, & Analog ECC — `FractionalCtrwAnalogEccAudit`):**
   - Identity: $G\_0 = \frac{2e^2}{h} = 77.48\,\mu\text{S}$ from the exact SI constants.
   - State-aware drift compensation with $\hat{\nu}\_i$ estimated from one noisy read at $t/t\_0 = 10$ brings the PCM drift MSE at $t/t\_0 = 10^4$ from $0.359$ to **$0.0019$**.
   - A simulated Montroll-Weiss `CTRW` (2000 walkers, Pareto waiting times $\beta = 0.70$, Chambers-Mallows-Stuck $\alpha = 1.75$ jumps) spreads by **$2.80\times$** between $t$ and $16t$: the anomalous prediction $16^{\beta/\alpha} = 3.03$ is within $8.2\%$, the Fickian prediction $16^{1/2} = 4$ misses by $42.8\%$.
   - Residual-trimmed least squares recovers $100\%$ of 100 analog codewords corrupted by LME noise plus two unlimited-magnitude errors; plain least squares recovers $0\%$. Roth's $[10, 2]$ $m$-height profile ($h\_8 = 28.74$) and the polygonal sector $\pi/2n$ are informative.
4. **Bridge 4 ($\text{Fractals} \leftrightarrow \text{Scaling Laws}$ — Multifractal Cascade, `PDHTE+JK`, `TailWelsch-DML`, & the $\pi^2/6$ Bound — `MultifractalPdhteRagCollapseAudit`):**
   - The deployed pipeline refuses tail extrapolation on bounded thin-tailed data (accepted plateau fraction $< 0.70$), and Welsch-IRLS cuts the ATE RMSE under two-sided Pareto ($\alpha = 1.65$) noise from $0.200$ (OLS) to **$0.048$** ($-75.9\%$).
   - Identity: $\sum\_{g=1}^8 g^{-2} = 1.527422 < \frac{\pi^2}{6}$ (`Replace` value $8.0$ informative). The `RAG Collapse` shares are informative literature values.
   - **Discarded findings:** on 8 simulated depth-14 log-normal cascades the partition-function curvature is $\hat{\tau}'' = -0.117$ against the theoretical $-\sigma^2\ln b = -0.180$, a gap of $0.063$ above the pre-registered $0.06$; and `PDHTE+JK` estimates $\hat{\xi} = 0.438$ against single-$k$ DEdH $0.445$ on a Hall-Welsh sample with $\xi = 0.50$, so it neither meets the $0.05$ tolerance nor beats the naive estimator at `seed = 2026`.
5. **Bridge 5 ($\mathbb{T}^d \leftrightarrow \mathbb{C}$ — Preface Cycle, Weyl Torus, Hermite-Mehler `DFM`, & Label-Shift Priors — `WeylTorusHermiteMehlerDfmAudit`):**
   - Identities: the preface cycle $x\_{t+1} = (5x\_t + 3)\bmod 16$ from $x\_0 = 2026 \bmod 16 = 10$ (`[10, 5, 12, 15, 14, 9, 0, 3, 2, 13, 4, 7, 6, 1, 8, 11, 10]`) and the Mercer mass $\sum\_{c \le 64}(1-\rho)\rho^c = 1 - \rho^{65}$.
   - Golden Weyl rotation reaches star discrepancy $D\_{256}^\ast = 0.0074$ (rational step $1/8$: $0.125$); `mixGamma` keeps $\ge 24$ bit transitions by construction ($25$ here; an identity), against $1$ for unmixed odd increments.
   - Under $10\%$ contamination at $x = 4.5$, `DFM` with 4 Hermite-Mehler features recovers $\alpha^\ast = (0.55, 0.35)$ with $L\_2$ error **$0.0136$**; 4 random Fourier features give $0.104$.
   - With an overconfident, biased classifier $\sigma(4x - 0.5)$, `MLLS+BCTS` estimates the target prior $q = 0.72$ with $D\_{\mathrm{KL}} = 3.1 \times 10^{-4}$, hard `BBSE` with $4.0 \times 10^{-5}$, and classify-and-count with $2.3 \times 10^{-2}$. `BBSE` is the better estimator on this draw. After replacing an undamped Newton step that diverged on most seeds with a damped, convergence-tested `BCTS` fit, `MLLS+BCTS` meets the $10^{-3}$ KL claim on 90% of the 20 acceptance seeds, so that claim is discarded rather than approved.
6. **Bridge 6 ($\mathbb{C} \times \text{Fractals} \times \mathbb{T}^d$ — Forced-Logistic `SNA`, `RLHF` Loop, `PerfGD`, `FHRR`/`GC-VSA`, `LTC` Fused Solver, & Technium Heuristics — `SnaRlhfFhrrCohomologyTechniumAudit`):**
   - The forced logistic map $x\_{n+1} = 3.0\,x\_n(1 - x\_n) + 0.1553\sin(2\pi\theta\_n)$ on the golden torus has Lyapunov exponent $\lambda = -0.045 < 0$ and phase-sensitivity growth $\mu = 0.53 > 0.15$, and two initial conditions synchronize exactly: the strange-nonchaotic signature. The undelayed 3-stage `RLHF` loop damps with $6$ sign flips, and its $(1.2, 0.7, 0.6)$ variant has non-consensus spectral radius $\sqrt{0.48} = 0.6928$.
   - Oracle-free `PerfGD` on `PerformativeSystem` converges to $\theta\_{\mathrm{PO}}$ (ratio $1.0000$); Repeated Risk Minimization stops at $\theta\_{\mathrm{PS}} = \frac{5}{6}\theta\_{\mathrm{PO}}$. The `LTC` Fused Solver stays bounded ($\max|x| = 1.33 \le A = 2$) at the stiff step $\Delta t = 2.5$ where explicit Euler explodes ($6.6 \times 10^{14}$); the bound is a theorem for positive conductance and is checked as an identity. Bundled `FHRR` unbinding on $\mathbb{T}^{128}$ keeps cosine $0.66$.
   - Identities: Schnakenberg $\Pi = 3.96 > 0$, exact phasor unbinding, `GC-VSA` path closure ($7.1 \times 10^{-15}$), and the Taleb-Douady signs $H(x, \Delta) < 0$ (concave) / $> 0$ (convex). Arthur's combinatorial growth factor, Hasani's trajectory lengths, the $1.42$ grid-scale ratio, and a polar-angle circular-coordinate baseline are informative. Perea's persistent-cohomology coordinates and Haff's Six Rules are cited, not computed.
   - **Discarded finding:** the companion polynomial $z(z - 1 + \delta)(z - 1 + \gamma)(z - 1 + \eta) = \delta\gamma\eta$ of the one-step-lagged loop has non-consensus spectral radius $0.818 < 1$, so the claim that a feedback lag destabilizes this `RLHF` loop is refuted.

The real-data coupler approves the `vic_elec.csv` wave $R^2$, the `danish_fire.csv` `PDHTE+JK` tail index ($\hat{\xi} = 0.640$, plateau $\mathrm{CV} < 0.25$), the `djia_1885_2023.csv` scaling curvature ($-0.099 < 0$), and the `LTC` boundedness identity on `bank_calls.csv`. It records one more **discarded finding**: the out-of-time Hermite-Mehler `DFM` estimate of the second-half `atlantic_storms.csv` hurricane prior is $0.269$ against a realized $0.226$, outside the pre-registered $0.03$, so the label-shift assumption does not hold across the two halves of the record.

---

## Computational Audit in Rust

The six-bridge Triadic Synthesis (`ComplexFractalToroidalTechniumSynthesis`) and its 5-CSV real-data coupler (`EmpiricalTriadicTechniumAudit` in `crates/stochology-lenses/src/triad.rs`) are audited in **Laboratory 3** and **Gate `G8`** (across all 20 development seeds `2026..=2045` and 200 confirmation seeds `9_000_000..=9_000_199`), while `--defect triadic_technium` runs the naive unshielded methods (`shielded = false`) and confirms that Gate `G8` halts with exit code `1`:

~~~bash
cargo run --release --bin labs -- --lenses
cargo run --release --bin gates -- --defect triadic_technium
~~~

Use the interactive Triadic and Stochary laboratory below to explore how complex phase, power-law tails, and toroidal recurrence interact:

<div class="stoch-lab" data-lab="stochary-kingman-triad" data-chapter="14-triadic-synthesis.html" data-title="Chapter 14 Lab: Triadic Synthesis (ℂ × Fractals × 𝕋ᵈ), Subdiffusion &amp; Fused LTC Stability" data-cli="cargo run --release --bin gates -- --defect triadic_technium"></div>

## Exercises

1. **Complex-to-Real Matrix Isomorphism (`Bridge 2`):** Show algebraically that for any complex scalars $z\_1 = a + ib$ and $z\_2 = c + id$, the product of their $2 \times 2$ real matrix embeddings $\mathcal{I}(z\_1)\mathcal{I}(z\_2) = \begin{pmatrix}a & -b\\ b & a\end{pmatrix}\begin{pmatrix}c & -d\\ d & c\end{pmatrix}$ equals $\mathcal{I}(z\_1 z\_2)$. Why does this allow an $n \times n$ complex MIMO inversion to execute on a $2n \times 2n$ real analog RRAM crossbar?
2. **Anomalous `CTRW` Subdiffusion vs. Fickian Diffusion (`Bridge 3`):** In a Montroll-Weiss Continuous-Time Random Walk with waiting-time exponent $\beta = 0.70$ and jump-length exponent $\alpha = 1.75$, the characteristic scale grows as $t^{\beta/\alpha}$. Calculate the theoretical spread ratio $(16t)^{\beta/\alpha} / t^{\beta/\alpha} = 16^{0.40}$ when time increases $16$-fold, and compare it against the Fickian $16^{1/2} = 4.0$ prediction.
3. **Toroidal Cycle and Golden Weyl Discrepancy (`Bridge 5`):** Starting from $x\_0 = 2026 \bmod 16 = 10$, compute the first four states of the affine torus recurrence $x\_{t+1} = (5x\_t + 3) \bmod 16$ and explain why $a = 5$ ($a - 1 = 4$ divisible by $4$) and odd $c = 3$ guarantee a full 16-state permutation by the Hull-Dobell theorem.

## Summary

- **The Triadic Synthesis ($\mathbb{C} \times \text{Fractals} \times \mathbb{T}^d$)** unites complex wave phase-amplitude representations (`WIRE` Gabor and `Improved SIREN`), fractal heavy-tailed subdiffusion (`CTRW`, `PDHTE`, and Welsch-IRLS), and compact toroidal recurrence (golden Weyl rotation, `FHRR`/`GC-VSA`, and Hasani's Fused `LTC` solver).
- **Strict separation of claims, identities, and informative constants** ensures that algebraic identities (`all_identities_closed`) close on every seed while refuted hypotheses (such as cascade curvature tolerance, `PDHTE+JK` superiority at `seed = 2026`, lag-induced `RLHF` instability, and out-of-time storm label shift) are transparently recorded in `discarded_findings` (`PROHIBITION_4`).
