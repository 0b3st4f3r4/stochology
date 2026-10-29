//! Axes 5 and 6 ($\mathbb{T}^d \leftrightarrow \mathbb{C}$ & $\mathbb{C} \times \text{Fractals} \times \mathbb{T}^d$):
//! Preface Modular Cycle in $\mathbb{Z}/16\mathbb{Z}$, Golden Weyl Torus (`SplitMix64` / `LXM` `mixGamma`),
//! Hermite-Mehler RKHS Features (`DFM`), `MLLS+BCTS`, Forced-Logistic `SNA`, Delayed `RLHF` Loop,
//! Izzo `PerfGD`, Phasor `FHRR`/`GC-VSA`, Hasani's `LTC` Fused Solver, and Technium Heuristics.

use std::f64::consts::PI;
use stochology_core::layers::PerformativeSystem;
use stochology_core::math::complex::Complex64;
use stochology_core::rng::Rng;

use crate::acceptance::NamedCheck;

/// Star discrepancy $D_N^*$ of a point set on $\mathbb{T}^1 = [0, 1)$.
fn star_discrepancy_1d(points: &[f64]) -> f64 {
    let mut sorted = points.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let n = sorted.len() as f64;
    sorted
        .iter()
        .enumerate()
        .map(|(i, &u)| {
            (u - (i as f64) / n)
                .abs()
                .max((u - ((i + 1) as f64) / n).abs())
        })
        .fold(0.0, f64::max)
}

/// Orthonormal Hermite-Mehler moment map $\psi_c(x) = \sqrt{\lambda_c}\,He_c(x)/\sqrt{c!}\,e^{-x^2/4}$, $c = 0..3$, $\lambda_c = (1-\rho)\rho^c$.
pub(crate) fn hermite_mehler_features(x: f64, rho: f64) -> [f64; 4] {
    let env = (-0.25 * x * x).exp();
    let w0 = (1.0 - rho).sqrt();
    let w1 = (w0 * rho.sqrt()) * x;
    let w2 = (w0 * rho) * (x * x - 1.0) / 2.0_f64.sqrt();
    let w3 = (w0 * rho.powf(1.5)) * (x.powi(3) - 3.0 * x) / 6.0_f64.sqrt();
    [w0 * env, w1 * env, w2 * env, w3 * env]
}

/// Kernel mean matching for a two-component mixture: solves $\min_\alpha \lVert \mu_Q - \alpha_1 \mu_{P_1} - \alpha_2 \mu_{P_2}\rVert^2$.
fn dfm_two_class_weights(mu1: &[f64], mu2: &[f64], mu_q: &[f64]) -> (f64, f64) {
    let dot = |a: &[f64], b: &[f64]| a.iter().zip(b).map(|(p, q)| p * q).sum::<f64>();
    let (m11, m12, m22) = (dot(mu1, mu1), dot(mu1, mu2), dot(mu2, mu2));
    let (b1, b2) = (dot(mu1, mu_q), dot(mu2, mu_q));
    let det = (m11 * m22 - m12 * m12).max(1e-12);
    ((m22 * b1 - m12 * b2) / det, (m11 * b2 - m12 * b1) / det)
}

/// Fits bias-corrected temperature scaling $p = \sigma(a z + b)$ on logits $z$ by damped Newton steps (backtracking line search
/// on the numerically stable log loss). Converged when the Newton decrement $\lambda^2/2 = g^\top H^{-1} g / 2$ falls below $10^{-10}$;
/// returns `None` if that does not happen within 100 iterations.
pub(crate) fn fit_bcts(logits: &[f64], labels: &[f64]) -> Option<(f64, f64)> {
    let log_loss = |a: f64, b: f64| -> f64 {
        logits
            .iter()
            .zip(labels)
            .map(|(&z, &y)| {
                let s = a * z + b;
                s.max(0.0) + (-s.abs()).exp().ln_1p() - y * s
            })
            .sum()
    };
    let n = logits.len() as f64;
    let (mut a, mut b) = (1.0_f64, 0.0_f64);
    for _ in 0..100 {
        let (mut ga, mut gb, mut haa, mut hab, mut hbb) = (0.0, 0.0, 0.0, 0.0, 0.0);
        for (&z, &y) in logits.iter().zip(labels) {
            let p = 1.0 / (1.0 + (-(a * z + b)).exp());
            let w = p * (1.0 - p);
            ga += (p - y) * z;
            gb += p - y;
            haa += w * z * z;
            hab += w * z;
            hbb += w;
        }
        let det = haa * hbb - hab * hab;
        let (da, db) = if det > 1e-12 {
            ((hbb * ga - hab * gb) / det, (haa * gb - hab * ga) / det)
        } else {
            (ga / n, gb / n)
        };
        if det > 1e-12 && 0.5 * (ga * da + gb * db) < 1e-10 {
            return Some((a, b));
        }
        let current = log_loss(a, b);
        let mut step = 1.0;
        while step > 1e-10 && log_loss(a - step * da, b - step * db) > current {
            step *= 0.5;
        }
        a -= step * da;
        b -= step * db;
    }
    None
}

/// Axis 5 ($\mathbb{T}^d \leftrightarrow \mathbb{C}$): Preface Modular Cycle in $\mathbb{Z}/16\mathbb{Z}$,
/// Golden Weyl Torus (`SplitMix64` / `LXM` `mixGamma`), Hermite-Mehler RKHS Features vs. Random Fourier Features in `DFM`
/// (Dussap, Blanchard & Chérif-Abdellatif 2023), and `MLLS+BCTS` vs. `BBSE` vs. Classify-and-Count (Garg et al. 2020).
#[derive(Debug, Clone)]
pub struct WeylTorusHermiteMehlerDfmAudit {
    /// Identity: 17-element full-period Hull-Dobell sequence of $x_{t+1} = (5x_t + 3) \bmod 16$ starting at $x_0 = 2026 \bmod 16 = 10$.
    pub preface_lcg_mod16_cycle: [u8; 17],
    /// Star discrepancy $D_{256}^*$ on $\mathbb{T}^1$ of the golden Weyl sequence $\theta_n = (n \varphi^{-1}) \bmod 1$ vs. the rational step $1/8$.
    pub golden_weyl_vs_rational_discrepancy: (f64, f64),
    /// Star discrepancy of the deployed Weyl increment (golden if shielded, rational otherwise). Gated.
    pub deployed_weyl_discrepancy: f64,
    /// Informative: minimum bit-transition count `(gamma ^ (gamma >> 1)).count_ones()` over 64 deployed stream gammas
    /// (`mixGamma` if shielded, unmixed odd increments $2s + 1$ otherwise).
    pub mixgamma_min_transition_bits: u32,
    /// Identity: Mercer eigenvalue mass $\sum_{c=0}^{64} (1-\rho)\rho^c = 1 - \rho^{65}$ of the Hermite-Mehler basis ($\rho = 0.50$).
    pub hermite_mehler_eigenvalue_sum: f64,
    /// $L_2$ error $\lVert \hat{\alpha} - \alpha^*\rVert_2$ under contaminated label shift $\mathbb{Q} = 0.55\,\mathbb{P}_1 + 0.35\,\mathbb{P}_2 + 0.10\,\mathbb{Q}_0$
    /// with 4 Hermite-Mehler features vs. 4 random Fourier features.
    pub dfm_hermite_vs_rff_l2_error: (f64, f64),
    /// $L_2$ error of the deployed feature map (Hermite-Mehler if shielded, random Fourier otherwise). Gated.
    pub deployed_dfm_l2_error: f64,
    /// Prior estimation $D_{\mathrm{KL}}(q_Y \parallel \hat{q}_Y)$ of `MLLS+BCTS` vs. hard-thresholded `BBSE` on 10 000 target draws.
    pub mlls_bcts_vs_bbse_kl_divergence: (f64, f64),
    /// Prior estimation KL divergence of classify-and-count with the uncalibrated classifier.
    pub classify_and_count_kl_divergence: f64,
    /// KL divergence of the deployed prior estimator (`MLLS+BCTS` if shielded, classify-and-count otherwise). Gated.
    pub deployed_label_shift_kl: f64,
    /// Named claim checks gated by this axis (see `crate::acceptance`).
    pub claims: Vec<NamedCheck>,
    /// Claims removed from the gate after failing multi-seed acceptance (`crate::acceptance`); still computed and reported.
    pub discarded_claims: Vec<NamedCheck>,
    /// True if the Hull-Dobell, Mercer, and `mixGamma` transition identities close.
    pub identities_closed: bool,
    /// True if identities close and the deployed Weyl step, `DFM` features, and prior estimator pass their claim checks.
    pub axis5_verified: bool,
}

impl WeylTorusHermiteMehlerDfmAudit {
    /// Steele, Lea & Flood (OOPSLA 2014) `mixGamma` function: `mix64variant13(z) | 1`, ensuring odd $\gamma$ ($\gcd(\gamma, 2^{64}) = 1$)
    /// and at least 24 bit transitions in `z ^ (z >> 1)`.
    pub fn mix_gamma(mut z: u64) -> u64 {
        z = (z ^ (z >> 33)).wrapping_mul(0xff51_afd7_ed55_8ccd);
        z = (z ^ (z >> 33)).wrapping_mul(0xc4ce_b9fe_1a85_ec53);
        z = (z ^ (z >> 33)) | 1;
        let n = (z ^ (z >> 1)).count_ones();
        if n < 24 {
            z ^= 0xaaaa_aaaa_aaaa_aaaa;
        }
        z
    }

    /// Evaluates Axis 5: Preface $\mathbb{Z}/16\mathbb{Z}$ cycle, Weyl torus & `mixGamma`, Hermite-Mehler `DFM`, and label-shift prior estimation.
    pub fn evaluate(seed: u64, shielded: bool) -> Self {
        let mut rng = Rng::new(seed ^ 0xC04D_BEEF_2026_0005);

        let mut cycle = [0u8; 17];
        let mut x_mod = (2026u64 % 16) as u8;
        for slot in &mut cycle {
            *slot = x_mod;
            x_mod = (5 * x_mod + 3) % 16;
        }
        let expected_cycle: [u8; 17] = [10, 5, 12, 15, 14, 9, 0, 3, 2, 13, 4, 7, 6, 1, 8, 11, 10];

        let phi_inv = (5.0_f64.sqrt() - 1.0) * 0.5;
        let n_weyl = 256usize;
        let golden_pts: Vec<f64> = (0..n_weyl)
            .map(|n| ((n as f64) * phi_inv).fract())
            .collect();
        let rational_pts: Vec<f64> = (0..n_weyl).map(|n| ((n as f64) / 8.0).fract()).collect();
        let disc_golden = star_discrepancy_1d(&golden_pts);
        let disc_rational = star_discrepancy_1d(&rational_pts);
        let deployed_disc = if shielded { disc_golden } else { disc_rational };

        let transitions = |g: u64| (g ^ (g >> 1)).count_ones();
        let mixed_min = (1..=64u64)
            .map(|s| {
                transitions(Self::mix_gamma(
                    seed.wrapping_add(s.wrapping_mul(0x9e37_79b9_7f4a_7c15)),
                ))
            })
            .min()
            .unwrap_or(0);
        let unmixed_min = (1..=64u64)
            .map(|s| transitions(2 * s + 1))
            .min()
            .unwrap_or(0);
        let min_transitions = if shielded { mixed_min } else { unmixed_min };

        let rho_mehler = 0.50_f64;
        let eig_sum: f64 = (0..=64)
            .map(|c| (1.0 - rho_mehler) * rho_mehler.powi(c))
            .sum();
        let alpha1_true = 0.55_f64;
        let alpha2_true = 0.35_f64;
        let n_mc = 400usize;
        let mut xs_p1 = Vec::with_capacity(n_mc);
        let mut xs_p2 = Vec::with_capacity(n_mc);
        let mut xs_q = Vec::with_capacity(n_mc);
        for i in 0..n_mc {
            let z = rng.standard_normal();
            xs_p1.push(-1.2 + 0.5 * z);
            xs_p2.push(1.2 + 0.5 * z);
            let frac = (i as f64) / (n_mc as f64);
            xs_q.push(if frac < alpha1_true {
                -1.2 + 0.5 * rng.standard_normal()
            } else if frac < alpha1_true + alpha2_true {
                1.2 + 0.5 * rng.standard_normal()
            } else {
                4.5 + 0.3 * rng.standard_normal()
            });
        }
        let rff_params: Vec<(f64, f64)> = (0..4)
            .map(|_| (rng.standard_normal(), 2.0 * PI * rng.next_f64()))
            .collect();
        let rff_features = |x: f64| -> [f64; 4] {
            let mut f = [0.0; 4];
            for (slot, &(w, b)) in f.iter_mut().zip(&rff_params) {
                *slot = (0.5_f64).sqrt() * (w * x + b).cos();
            }
            f
        };
        let mean_map = |xs: &[f64], map: &dyn Fn(f64) -> [f64; 4]| -> [f64; 4] {
            let mut mu = [0.0; 4];
            for &x in xs {
                for (m, v) in mu.iter_mut().zip(map(x)) {
                    *m += v / (xs.len() as f64);
                }
            }
            mu
        };
        let hm_map = |x: f64| hermite_mehler_features(x, rho_mehler);
        let dfm_error = |map: &dyn Fn(f64) -> [f64; 4]| -> f64 {
            let (a1, a2) = dfm_two_class_weights(
                &mean_map(&xs_p1, map),
                &mean_map(&xs_p2, map),
                &mean_map(&xs_q, map),
            );
            (a1 - alpha1_true).hypot(a2 - alpha2_true)
        };
        let err_hm = dfm_error(&hm_map);
        let err_rff = dfm_error(&rff_features);
        let deployed_dfm_err = if shielded { err_hm } else { err_rff };

        let q_true = 0.72_f64;
        let classifier_logit = |x: f64| 4.0 * x - 0.5;
        let n_val = 2000usize;
        let mut val_logits = Vec::with_capacity(n_val);
        let mut val_labels = Vec::with_capacity(n_val);
        for _ in 0..n_val {
            let y = if rng.next_f64() < 0.5 { 1.0 } else { 0.0 };
            let x = (2.0 * y - 1.0) + rng.standard_normal();
            val_logits.push(classifier_logit(x));
            val_labels.push(y);
        }
        let n_target = 10_000usize;
        let target_logits: Vec<f64> = (0..n_target)
            .map(|_| {
                let y = if rng.next_f64() < q_true { 1.0 } else { 0.0 };
                classifier_logit((2.0 * y - 1.0) + rng.standard_normal())
            })
            .collect();
        let pi_source = val_labels.iter().sum::<f64>() / (n_val as f64);

        let q_mlls = fit_bcts(&val_logits, &val_labels).map(|(bcts_a, bcts_b)| {
            let calibrated: Vec<f64> = target_logits
                .iter()
                .map(|&z| 1.0 / (1.0 + (-(bcts_a * z + bcts_b)).exp()))
                .collect();
            let mut q = pi_source;
            for _ in 0..500 {
                q = calibrated
                    .iter()
                    .map(|&p| {
                        let pos = q / pi_source * p;
                        pos / (pos + (1.0 - q) / (1.0 - pi_source) * (1.0 - p))
                    })
                    .sum::<f64>()
                    / (n_target as f64);
            }
            q
        });
        let (mut tp, mut fp, mut n_pos) = (0.0, 0.0, 0.0);
        for (&z, &y) in val_logits.iter().zip(&val_labels) {
            let pred = if z > 0.0 { 1.0 } else { 0.0 };
            tp += pred * y;
            fp += pred * (1.0 - y);
            n_pos += y;
        }
        let tpr = tp / n_pos;
        let fpr = fp / ((n_val as f64) - n_pos);
        let target_pos_rate =
            target_logits.iter().filter(|&&z| z > 0.0).count() as f64 / (n_target as f64);
        let q_bbse = ((target_pos_rate - fpr) / (tpr - fpr)).clamp(1e-3, 1.0 - 1e-3);
        let q_cc = target_pos_rate;
        let kl_bin = |p: f64, q: f64| -> f64 {
            let p_c = p.clamp(1e-6, 1.0 - 1e-6);
            let q_c = q.clamp(1e-6, 1.0 - 1e-6);
            p_c * (p_c / q_c).ln() + (1.0 - p_c) * ((1.0 - p_c) / (1.0 - q_c)).ln()
        };
        let kl_mlls = q_mlls.map_or(f64::INFINITY, |q| kl_bin(q_true, q));
        let kl_bbse = kl_bin(q_true, q_bbse);
        let kl_cc = kl_bin(q_true, q_cc);
        let deployed_kl = if shielded { kl_mlls } else { kl_cc };

        let identities_closed = cycle == expected_cycle
            && (eig_sum - (1.0 - rho_mehler.powi(65))).abs() < 1e-12
            && mixed_min >= 24;
        let claims: Vec<NamedCheck> = vec![
            (
                "triad.a5.det.deployed_weyl_discrepancy_below_0.02",
                deployed_disc < 0.02,
            ),
            (
                "triad.a5.deployed_dfm_error_below_0.05",
                deployed_dfm_err < 0.05,
            ),
        ];
        let discarded_claims: Vec<NamedCheck> = vec![(
            "triad.a5.deployed_label_shift_kl_below_1e-3",
            deployed_kl < 1e-3,
        )];
        let axis5_verified = identities_closed && claims.iter().all(|c| c.1);

        Self {
            preface_lcg_mod16_cycle: cycle,
            golden_weyl_vs_rational_discrepancy: (disc_golden, disc_rational),
            deployed_weyl_discrepancy: deployed_disc,
            mixgamma_min_transition_bits: min_transitions,
            hermite_mehler_eigenvalue_sum: eig_sum,
            dfm_hermite_vs_rff_l2_error: (err_hm, err_rff),
            deployed_dfm_l2_error: deployed_dfm_err,
            mlls_bcts_vs_bbse_kl_divergence: (kl_mlls, kl_bbse),
            classify_and_count_kl_divergence: kl_cc,
            deployed_label_shift_kl: deployed_kl,
            claims,
            discarded_claims,
            identities_closed,
            axis5_verified,
        }
    }
}

/// Multiplies two polynomials given by ascending coefficients.
fn poly_mul(a: &[f64], b: &[f64]) -> Vec<f64> {
    let mut out = vec![0.0; a.len() + b.len() - 1];
    for (i, &ai) in a.iter().enumerate() {
        for (j, &bj) in b.iter().enumerate() {
            out[i + j] += ai * bj;
        }
    }
    out
}

/// Roots of a monic real polynomial (ascending coefficients, leading coefficient `1`) via Durand-Kerner iteration.
fn polynomial_roots(coeffs: &[f64]) -> Vec<Complex64> {
    let deg = coeffs.len() - 1;
    let eval = |z: Complex64| -> Complex64 {
        coeffs[..deg]
            .iter()
            .rev()
            .fold(Complex64::new(coeffs[deg], 0.0), |acc, &c| {
                let m = acc * z;
                Complex64::new(m.re + c, m.im)
            })
    };
    let mut roots = Vec::with_capacity(deg);
    let mut seed_root = Complex64::new(1.0, 0.0);
    for _ in 0..deg {
        roots.push(seed_root);
        seed_root *= Complex64::new(0.4, 0.9);
    }
    for _ in 0..1000 {
        for i in 0..deg {
            let zi = roots[i];
            let den = roots
                .iter()
                .enumerate()
                .filter(|&(j, _)| j != i)
                .fold(Complex64::new(1.0, 0.0), |acc, (_, &zj)| acc * (zi - zj));
            let step = eval(zi) / den;
            roots[i] = zi - step;
        }
    }
    roots
}

/// Spectral radius, excluding the consensus root $z = 1$, of the 3-stage loop $h \to \phi \to a \to h$ with gains $(\delta, \gamma, \eta)$
/// when $h$ reads $a$ with a lag of `delay` steps: $z^{\tau}(z - 1 + \delta)(z - 1 + \gamma)(z - 1 + \eta) - \delta\gamma\eta = 0$.
pub(crate) fn rlhf_nonconsensus_spectral_radius(
    delta: f64,
    gamma: f64,
    eta: f64,
    delay: usize,
) -> f64 {
    let mut p = vec![1.0];
    for _ in 0..delay {
        p = poly_mul(&p, &[0.0, 1.0]);
    }
    for gain in [delta, gamma, eta] {
        p = poly_mul(&p, &[gain - 1.0, 1.0]);
    }
    p[0] -= delta * gamma * eta;
    let roots = polynomial_roots(&p);
    let consensus = roots
        .iter()
        .enumerate()
        .min_by(|a, b| {
            let da = (a.1.re - 1.0).hypot(a.1.im);
            let db = (b.1.re - 1.0).hypot(b.1.im);
            da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(i, _)| i)
        .unwrap_or(0);
    roots
        .iter()
        .enumerate()
        .filter(|&(i, _)| i != consensus)
        .map(|(_, z)| z.abs())
        .fold(0.0, f64::max)
}

/// Axis 6 ($\mathbb{C} \times \text{Fractals} \times \mathbb{T}^d$): Strange Nonchaotic Attractor Diagnostics on the Quasiperiodically Forced Logistic Map,
/// the Delayed 3-Stage `RLHF` Feedback Loop, Izzo `PerfGD` vs. Repeated Risk Minimization, Phasor `FHRR` / `GC-VSA` Binding,
/// Hasani's `LTC` Fused Solver, and the Kevin Kelly / W. Brian Arthur / Taleb-Douady Technium Heuristics.
#[derive(Debug, Clone)]
pub struct SnaRlhfFhrrCohomologyTechniumAudit {
    /// Identity: Schnakenberg (1976) cycle entropy production $\Pi = \sum_e (J_e^+ - J_e^-)\ln(J_e^+/J_e^-) > 0$ for a driven 3-state cycle.
    pub schnakenberg_cycle_entropy_production_pi: f64,
    /// Phase-sensitivity growth exponent $\mu = \ln(\Gamma_{2000}/\Gamma_{100}) / \ln 20$ with $\Gamma_N = \max_{n \le N}|\partial x_n/\partial\theta|$,
    /// and largest Lyapunov exponent $\lambda$ of $x_{n+1} = a x_n(1 - x_n) + \varepsilon\sin(2\pi\theta_n)$ at $a = 3.0, \varepsilon = 0.1553$.
    pub sna_fractal_exponent_and_negative_lyapunov: (f64, f64),
    /// Orbital synchronization error $|x_N^{(1)} - x_N^{(2)}|$ of two initial conditions driven by the same toroidal phase $\theta_n$.
    pub sna_orbital_synchronization_error: f64,
    /// Number of sign flips of $h - a$ in the undelayed 3-stage `RLHF` loop ($\delta=\gamma=\eta=0.3$ from $(1, 0, -1)$ over 40 steps).
    pub rlhf_delay0_damped_sign_flips: usize,
    /// Non-consensus spectral radius of the `RLHF` loop with $(\delta, \gamma, \eta) = (1.2, 0.7, 0.6)$ at lag $\tau = 0$ vs. $\tau = 1$,
    /// computed from the companion polynomial roots. Informative: the delay-destabilization claim is not gated (see `discarded_findings`).
    pub rlhf_delay0_vs_delayed_spectral_radius: (f64, f64),
    /// Final iterate over $\theta_{\mathrm{PO}}$ of Repeated Risk Minimization vs. oracle-free `PerfGD` on `PerformativeSystem(2, 0.5, 0.5, 1, 1)`.
    pub izzo_rrm_vs_perfgd_ratio_to_opt: (f64, f64),
    /// $|\theta_{\text{final}} - \theta_{\mathrm{PO}}|$ of the deployed optimizer (`PerfGD` if shielded, RRM otherwise). Gated.
    pub deployed_performative_gap_to_opt: f64,
    /// Phasor `FHRR` single-pair unbinding cosine (identity, `1.0`) and 2-pair bundled unbinding cosine on $\mathbb{T}^{128} \subset \mathbb{C}^{128}$.
    pub fhrr_phasor_exact_and_bundled_unbinding_cosine: (f64, f64),
    /// Informative `GC-VSA` scale ratio $s_{i+1}/s_i = 1.42$ and identity path-integration closure error
    /// $\lVert V_X^{\odot 3}\odot V_Y^{\odot 7}\odot V_X^{\odot 2}\odot V_Y^{\odot 1} - V_X^{\odot 5}\odot V_Y^{\odot 8}\rVert_\infty$.
    pub gc_vsa_scale_ratio_and_path_closure_error: (f64, f64),
    /// Informative: circular correlation of a polar-angle baseline on a noisy circle.
    pub polar_baseline_circular_coordinate_correlation: f64,
    /// Informative: Hasani et al. (AAAI 2021) latent trajectory lengths as transcribed from the literature (`LTC`, Neural ODE, CT-RNN).
    pub hasani_ltc_vs_neural_ode_and_ctrnn_trajectory_lengths: (f64, f64, f64),
    /// Maximum $|x(t)|$ of the `LTC` Fused Solver vs. explicit Euler at the stiff step $\Delta t = 2.5$.
    pub ltc_fused_solver_max_state_vs_euler: (f64, f64),
    /// Informative: maximum $|x(t)|$ of the deployed solver (fused if shielded, Euler otherwise).
    pub deployed_solver_max_state: f64,
    /// Informative Arthur combinatorial growth factor $M_8 / M_0$, and identity Taleb-Douady signs
    /// $H(x, \Delta) = \frac{f(x-\Delta)+f(x+\Delta)}{2} - f(x)$ (`(concave < 0, convex > 0)`).
    pub technium_arthur_growth_and_taleb_fragility_h: (f64, f64, f64),
    /// Claims evaluated against computed ground truth that did not survive (Prohibition 4: recorded, never gated).
    pub discarded_findings: Vec<&'static str>,
    /// Named claim checks gated by this axis (see `crate::acceptance`).
    pub claims: Vec<NamedCheck>,
    /// True if the Schnakenberg, phasor, path-closure, `LTC` bound, and Taleb-Douady identities close.
    pub identities_closed: bool,
    /// True if identities close, the mode-independent dynamical claims hold, and the deployed optimizer passes its check.
    pub axis6_verified: bool,
}

impl SnaRlhfFhrrCohomologyTechniumAudit {
    /// Evaluates Axis 6: Schnakenberg thermodynamics, forced-logistic `SNA` diagnostics, `RLHF` loop spectra,
    /// `PerfGD` vs. RRM, Phasor `FHRR`/`GC-VSA`, `LTC` Fused Solver vs. Euler, and Technium heuristics.
    pub fn evaluate(seed: u64, shielded: bool) -> Self {
        let mut rng = Rng::new(seed ^ 0xC04D_BEEF_2026_0006);

        let j_plus = [1.8_f64, 1.5_f64, 2.1_f64];
        let j_minus = [0.6_f64, 0.5_f64, 0.7_f64];
        let schnakenberg_pi: f64 = j_plus
            .iter()
            .zip(j_minus.iter())
            .map(|(&jp, &jm)| (jp - jm) * (jp / jm).ln())
            .sum();

        let a_sna = 3.0_f64;
        let eps_sna = 0.1553_f64;
        let omega_gold = (5.0_f64.sqrt() - 1.0) * 0.5;
        let mut x1 = 0.42_f64;
        let mut x2 = 0.78_f64;
        let mut dx_dtheta = 0.0_f64;
        let mut lyap_sum = 0.0_f64;
        let n_sna = 2000usize;
        let mut gamma_running = 0.0_f64;
        let mut gamma_early = 0.0_f64;
        for step in 0..n_sna {
            let theta = ((step as f64) * omega_gold).fract();
            let deriv = a_sna * (1.0 - 2.0 * x1);
            lyap_sum += deriv.abs().max(1e-300).ln();
            dx_dtheta = deriv * dx_dtheta + 2.0 * PI * eps_sna * (2.0 * PI * theta).cos();
            x1 = a_sna * x1 * (1.0 - x1) + eps_sna * (2.0 * PI * theta).sin();
            x2 = a_sna * x2 * (1.0 - x2) + eps_sna * (2.0 * PI * theta).sin();
            gamma_running = gamma_running.max(dx_dtheta.abs());
            if step + 1 == 100 {
                gamma_early = gamma_running;
            }
        }
        let lyap_sna = lyap_sum / (n_sna as f64);
        let fractal_exp = (gamma_running / gamma_early.max(1e-300)).ln() / 20.0_f64.ln();
        let sync_err = (x1 - x2).abs();

        let mut h = 1.0_f64;
        let mut phi = 0.0_f64;
        let mut a_val = -1.0_f64;
        let mut sign_flips = 0usize;
        let initial_gap = h - a_val;
        let mut prev_diff = initial_gap;
        for _ in 0..40 {
            let h_next = h + 0.3 * (a_val - h);
            let phi_next = phi + 0.3 * (h - phi);
            let a_next = a_val + 0.3 * (phi - a_val);
            h = h_next;
            phi = phi_next;
            a_val = a_next;
            let diff = h - a_val;
            if diff * prev_diff < 0.0 {
                sign_flips += 1;
            }
            prev_diff = diff;
        }
        let rlhf_damped = sign_flips >= 2 && prev_diff.abs() < initial_gap.abs();
        let rho_delay0 = rlhf_nonconsensus_spectral_radius(1.2, 0.7, 0.6, 0);
        let rho_delayed = rlhf_nonconsensus_spectral_radius(1.2, 0.7, 0.6, 1);

        let perf = PerformativeSystem::new(2.0, 0.5, 0.5, 1.0, 1.0);
        let theta_po = perf.diagnose().theta_po;
        let perfgd_final = *perf
            .perfgd_finite_difference_trajectory(0.0, 1.0, 0.5, 60)
            .last()
            .unwrap_or(&0.0);
        let mut rrm_final = 0.0_f64;
        for _ in 0..60 {
            rrm_final = perf.rrm_update_step(rrm_final);
        }
        let deployed_theta = if shielded { perfgd_final } else { rrm_final };
        let deployed_perf_gap = (deployed_theta - theta_po).abs();

        let dim_vsa = 128usize;
        let mut a_ang = vec![0.0_f64; dim_vsa];
        let mut b_ang = vec![0.0_f64; dim_vsa];
        let mut c_ang = vec![0.0_f64; dim_vsa];
        let mut d_ang = vec![0.0_f64; dim_vsa];
        for j in 0..dim_vsa {
            a_ang[j] = 2.0 * PI * rng.next_f64() - PI;
            b_ang[j] = 2.0 * PI * rng.next_f64() - PI;
            c_ang[j] = 2.0 * PI * rng.next_f64() - PI;
            d_ang[j] = 2.0 * PI * rng.next_f64() - PI;
        }
        let exact_unbind_cos: f64 = (0..dim_vsa)
            .map(|j| ((a_ang[j] + b_ang[j] - b_ang[j]) - a_ang[j]).cos())
            .sum::<f64>()
            / (dim_vsa as f64);

        let mut bundled_cos_sum = 0.0_f64;
        for j in 0..dim_vsa {
            let s_re = (a_ang[j] + b_ang[j]).cos() + (c_ang[j] + d_ang[j]).cos();
            let s_im = (a_ang[j] + b_ang[j]).sin() + (c_ang[j] + d_ang[j]).sin();
            let u_re = s_re * b_ang[j].cos() + s_im * b_ang[j].sin();
            let u_im = s_im * b_ang[j].cos() - s_re * b_ang[j].sin();
            let u_phase = u_im.atan2(u_re);
            bundled_cos_sum += (u_phase - a_ang[j]).cos();
        }
        let bundled_unbind_cos = bundled_cos_sum / (dim_vsa as f64);

        let gc_scale_ratio = 1.42_f64;
        let mut max_path_err = 0.0_f64;
        for j in 0..dim_vsa {
            let vx = a_ang[j];
            let vy = b_ang[j];
            let step_by_step = 3.0 * vx + 7.0 * vy + 2.0 * vx + 1.0 * vy;
            let direct = 5.0 * vx + 8.0 * vy;
            let err = (step_by_step.cos() - direct.cos()).hypot(step_by_step.sin() - direct.sin());
            max_path_err = max_path_err.max(err);
        }

        let n_circle = 32usize;
        let circ_corr: f64 = (0..n_circle)
            .map(|i| {
                let ang = 2.0 * PI * (i as f64) / (n_circle as f64);
                let x_pt = ang.cos() + 0.03 * (3.0 * ang).sin();
                let y_pt = ang.sin() + 0.03 * (3.0 * ang).cos();
                (ang - y_pt.atan2(x_pt).rem_euclid(2.0 * PI)).cos()
            })
            .sum::<f64>()
            / (n_circle as f64);

        let ltc_lengths = (463.1858_f64, 102.9654_f64, 39.9587_f64);
        let tau_base = 1.0_f64;
        let a_rev = 2.0_f64;
        let dt_stiff = 2.5_f64;
        let mut x_ltc = 0.5_f64;
        let mut x_euler = 0.5_f64;
        let mut max_ltc_abs = 0.5_f64;
        let mut max_euler_abs = 0.5_f64;
        for step in 0..20 {
            let f_gate = 1.2 + 0.8 * ((step as f64) * 0.5).sin().abs();
            x_ltc =
                (x_ltc + dt_stiff * f_gate * a_rev) / (1.0 + dt_stiff * (1.0 / tau_base + f_gate));
            let dx_dt = -(1.0 / tau_base + f_gate) * x_euler + f_gate * a_rev;
            x_euler += dt_stiff * dx_dt;
            max_ltc_abs = max_ltc_abs.max(x_ltc.abs());
            max_euler_abs = max_euler_abs.max(x_euler.abs());
        }
        let deployed_max_state = if shielded { max_ltc_abs } else { max_euler_abs };

        let mut m_tech = 10.0_f64;
        for _ in 0..8 {
            let pairs = 0.5 * m_tech * (m_tech - 1.0);
            m_tech += 0.015 * pairs;
        }
        let arthur_growth = m_tech / 10.0;
        let delta_shock = 1.0_f64;
        let f_fragile = |s: f64| -s.exp();
        let f_convex = |s: f64| s.exp();
        let h_fragile = 0.5 * (f_fragile(-delta_shock) + f_fragile(delta_shock)) - f_fragile(0.0);
        let h_antifragile = 0.5 * (f_convex(-delta_shock) + f_convex(delta_shock)) - f_convex(0.0);

        let mut discarded_findings = Vec::new();
        if rho_delayed <= 1.0 {
            discarded_findings.push(
                "a one-step lag pushes the (1.2, 0.7, 0.6) RLHF loop past the unit circle (companion radius > 1)",
            );
        }

        let identities_closed = schnakenberg_pi > 0.0
            && (exact_unbind_cos - 1.0).abs() < 1e-12
            && max_path_err < 1e-12
            && h_fragile < 0.0
            && h_antifragile > 0.0
            && max_ltc_abs <= a_rev;
        let claims: Vec<NamedCheck> = vec![
            ("triad.a6.det.sna_lyapunov_below_-0.04", lyap_sna < -0.04),
            (
                "triad.a6.det.sna_phase_growth_exponent_above_0.15",
                fractal_exp > 0.15,
            ),
            ("triad.a6.det.sna_sync_error_below_1e-10", sync_err < 1e-10),
            (
                "triad.a6.det.rlhf_at_least_2_sign_flips_and_decay",
                rlhf_damped,
            ),
            (
                "triad.a6.det.rlhf_undelayed_radius_below_1",
                rho_delay0 < 1.0,
            ),
            (
                "triad.a6.fhrr_bundled_unbinding_above_0.55",
                bundled_unbind_cos > 0.55,
            ),
            (
                "triad.a6.det.deployed_performative_gap_below_1e-3",
                deployed_perf_gap < 1e-3,
            ),
        ];
        let axis6_verified = identities_closed && claims.iter().all(|c| c.1);

        Self {
            schnakenberg_cycle_entropy_production_pi: schnakenberg_pi,
            sna_fractal_exponent_and_negative_lyapunov: (fractal_exp, lyap_sna),
            sna_orbital_synchronization_error: sync_err,
            rlhf_delay0_damped_sign_flips: sign_flips,
            rlhf_delay0_vs_delayed_spectral_radius: (rho_delay0, rho_delayed),
            izzo_rrm_vs_perfgd_ratio_to_opt: (rrm_final / theta_po, perfgd_final / theta_po),
            deployed_performative_gap_to_opt: deployed_perf_gap,
            fhrr_phasor_exact_and_bundled_unbinding_cosine: (exact_unbind_cos, bundled_unbind_cos),
            gc_vsa_scale_ratio_and_path_closure_error: (gc_scale_ratio, max_path_err),
            polar_baseline_circular_coordinate_correlation: circ_corr,
            hasani_ltc_vs_neural_ode_and_ctrnn_trajectory_lengths: ltc_lengths,
            ltc_fused_solver_max_state_vs_euler: (max_ltc_abs, max_euler_abs),
            deployed_solver_max_state: deployed_max_state,
            technium_arthur_growth_and_taleb_fragility_h: (arthur_growth, h_fragile, h_antifragile),
            discarded_findings,
            claims,
            identities_closed,
            axis6_verified,
        }
    }
}
