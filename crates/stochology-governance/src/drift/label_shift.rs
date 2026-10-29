//! Auditable Estimators for Label / Prior Probability Shift (**BBSE**, **MLLS/SLD**, **DFM**, and **OTTER**, §4.2).

/// Auditable Estimators for Label / Prior Probability Shift (§4.2):
/// **BBSE**, **MLLS/SLD**, **DFM**, and **OTTER**.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LabelShiftDiagnostic {
    /// Base rate in the source distribution $P_Y(y=1)$.
    pub source_prior_p1: f64,
    /// BBSE estimate (Lipton, Wang & Smola): $\hat{\omega} = C^{-1}\hat{\mu}_t$.
    pub target_prior_bbse: f64,
    /// Smallest singular value $\sigma_{\min}(C)$ of the source confusion matrix (spectral condition $\sigma_{\min}(C) > 0$).
    pub confusion_matrix_sigma_min: f64,
    /// MLLS / SLD estimate (Saerens-Latinne-Decaestecker EM over calibrated posteriors).
    pub target_prior_mlls_sld: f64,
    /// DFM (*Distribution Feature Matching*) estimate and novel-class contamination fraction $\alpha_0^*$.
    pub target_prior_dfm: f64,
    pub novel_class_contamination_alpha0: f64,
    /// Second smallest eigenvalue $\Delta_{\min}$ of the centered Gram matrix in DFM.
    pub gram_eigenvalue_delta_min: f64,
    /// OTTER estimate (*Optimal Transport* with cost $C_{ij} = -\ln P_\theta(Y=j \mid X=x_i)$).
    pub target_prior_otter: f64,
    /// Hoffman stability constant $\kappa$ of the OTTER optimal transport linear program.
    pub hoffman_constant_kappa: f64,
}

/// Computes the exact algebraic Bayes posterior update under Label Shift:
/// $$Q(y = 1 \mid x) = \frac{\omega_1 P(y=1 \mid x)}{\omega_1 P(y=1 \mid x) + \omega_0 (1 - P(y=1 \mid x))}$$
/// where $\omega_1 = \frac{Q_Y(1)}{P_Y(1)}$ and $\omega_0 = \frac{1 - Q_Y(1)}{1 - P_Y(1)}$.
#[inline]
pub fn adjust_posterior_for_label_shift(
    p_source_given_x: f64,
    source_prior_p1: f64,
    target_prior_q1: f64,
) -> f64 {
    let w1 = target_prior_q1 / source_prior_p1.max(1e-12);
    let w0 = (1.0 - target_prior_q1) / (1.0 - source_prior_p1).max(1e-12);
    let num = w1 * p_source_given_x;
    let den = num + w0 * (1.0 - p_source_given_x);
    if den <= 0.0 {
        target_prior_q1
    } else {
        (num / den).clamp(1e-6, 1.0 - 1e-6)
    }
}

/// Executes all four canonical Label Shift estimators (**BBSE**, **MLLS/SLD**, **DFM**, and **OTTER**) without target labels.
#[allow(clippy::too_many_arguments)]
pub fn estimate_label_shift_four_methods(
    source_prior_p1: f64,
    source_tpr: f64,
    source_fpr: f64,
    target_calibrated_posteriors: &[f64],
    source_class0_feature_mean: f64,
    source_class1_feature_mean: f64,
    target_feature_mean: f64,
    novel_class_contamination_alpha0: f64,
) -> LabelShiftDiagnostic {
    assert!(!target_calibrated_posteriors.is_empty());

    // 1. BBSE (Black Box Shift Estimation, Lipton, Wang & Smola):
    // Compute the Bayes-consistent posterior threshold separating y_hat=0 and y_hat=1 under source (p1, TPR, FPR):
    let p_s_given_pos = (source_prior_p1 * source_tpr)
        / (source_prior_p1 * source_tpr + (1.0 - source_prior_p1) * source_fpr).max(1e-12);
    let p_s_given_neg = (source_prior_p1 * (1.0 - source_tpr))
        / (source_prior_p1 * (1.0 - source_tpr) + (1.0 - source_prior_p1) * (1.0 - source_fpr))
            .max(1e-12);
    let bayes_threshold = source_prior_p1.clamp(
        p_s_given_neg.min(p_s_given_pos),
        p_s_given_pos.max(p_s_given_neg),
    );

    let pred_pos_rate = target_calibrated_posteriors
        .iter()
        .filter(|&&p| p >= bayes_threshold)
        .count() as f64
        / (target_calibrated_posteriors.len() as f64);
    let diff_rates = source_tpr - source_fpr;
    let q1_bbse = if diff_rates.abs() < 1e-6 {
        source_prior_p1.clamp(0.001, 0.999)
    } else {
        ((pred_pos_rate - source_fpr) / diff_rates).clamp(0.001, 0.999)
    };
    // Exact smallest singular value of 2x2 joint confusion matrix C
    let c00 = (1.0 - source_fpr) * (1.0 - source_prior_p1);
    let c01 = source_fpr * (1.0 - source_prior_p1);
    let c10 = (1.0 - source_tpr) * source_prior_p1;
    let c11 = source_tpr * source_prior_p1;
    let sigma_min = smallest_singular_value_2x2(c00, c01, c10, c11);

    // 2. MLLS / SLD (Saerens-Latinne-Decaestecker EM algorithm):
    let mut q1_mlls = source_prior_p1.clamp(0.01, 0.99);
    for _ in 0..80 {
        let sum_post: f64 = target_calibrated_posteriors
            .iter()
            .map(|&p| adjust_posterior_for_label_shift(p, source_prior_p1, q1_mlls))
            .sum();
        q1_mlls = (sum_post / (target_calibrated_posteriors.len() as f64)).clamp(0.001, 0.999);
    }

    // 3. DFM (Distribution Feature Matching) with Generalized Label Shift (novel class contamination alpha_0*):
    let diff_mu = source_class1_feature_mean - source_class0_feature_mean;
    let delta_min = diff_mu * diff_mu;
    let effective_scale = (1.0 - novel_class_contamination_alpha0).clamp(0.01, 1.0);
    let q1_dfm = if diff_mu.abs() < 1e-12 {
        source_prior_p1.clamp(0.001, 0.999)
    } else {
        ((target_feature_mean - source_class0_feature_mean) / (effective_scale * diff_mu))
            .clamp(0.001, 0.999)
    };

    // 4. Entropic Optimal Transport 1D Sinkhorn-Knopp Label Shift Estimator (OTTER grid search
    // on cost matrix C_{ij} = -ln P_theta(Y=j | X=x_i)):
    let q1_otter = solve_otter_sinkhorn(
        source_prior_p1,
        source_tpr,
        source_fpr,
        pred_pos_rate,
        target_calibrated_posteriors,
    );
    // Exact Hoffman stability constant kappa(M) = 1 / sigma_min(M) of conditional operator M
    let sigma_min_m =
        smallest_singular_value_2x2(1.0 - source_fpr, 1.0 - source_tpr, source_fpr, source_tpr);
    let hoffman_kappa = 1.0 / sigma_min_m.max(1e-6);

    LabelShiftDiagnostic {
        source_prior_p1,
        target_prior_bbse: q1_bbse,
        confusion_matrix_sigma_min: sigma_min,
        target_prior_mlls_sld: q1_mlls,
        target_prior_dfm: q1_dfm,
        novel_class_contamination_alpha0,
        gram_eigenvalue_delta_min: delta_min,
        target_prior_otter: q1_otter,
        hoffman_constant_kappa: hoffman_kappa,
    }
}

/// Fits a calibrated 1D logistic classifier $P(Y=1 \mid X=x) = \sigma(\beta_0 + \beta_1 x)$ on `(x_source, y_source)`
/// via Newton-Raphson (IRLS) and evaluates it on `x_target`.
///
/// Returns `(source_prior_p1, source_tpr, source_fpr, source_class0_mean, source_class1_mean, target_posteriors)`.
pub fn fit_calibrated_logistic_1d(
    x_source: &[f64],
    y_source: &[f64],
    x_target: &[f64],
) -> (f64, f64, f64, f64, f64, Vec<f64>) {
    assert!(!x_source.is_empty() && x_source.len() == y_source.len());
    let n = x_source.len() as f64;
    let p_base = (y_source.iter().sum::<f64>() / n).clamp(0.001, 0.999);
    let mut b0 = (p_base / (1.0 - p_base)).ln();
    let mut b1 = 0.0_f64;

    for _ in 0..25 {
        let mut g0 = 0.0;
        let mut g1 = 0.0;
        let mut h00 = 1e-4;
        let mut h01 = 0.0;
        let mut h11 = 1e-4;
        for (&x, &y) in x_source.iter().zip(y_source.iter()) {
            let z = (b0 + b1 * x).clamp(-20.0, 20.0);
            let p = 1.0 / (1.0 + (-z).exp());
            let w = (p * (1.0 - p)).max(1e-6);
            let diff = y - p;
            g0 += diff;
            g1 += diff * x;
            h00 += w;
            h01 += w * x;
            h11 += w * x * x;
        }
        let det = (h00 * h11 - h01 * h01).max(1e-9);
        b0 += (h11 * g0 - h01 * g1) / det;
        b1 += (-h01 * g0 + h00 * g1) / det;
    }

    let mut sum0 = 0.0;
    let mut cnt0 = 0usize;
    let mut sum1 = 0.0;
    let mut cnt1 = 0usize;
    let mut tp = 0usize;
    let mut fp = 0usize;
    for (&x, &y) in x_source.iter().zip(y_source.iter()) {
        let p = 1.0 / (1.0 + (-(b0 + b1 * x).clamp(-20.0, 20.0)).exp());
        let pred_pos = p >= p_base;
        if y >= 0.5 {
            sum1 += x;
            cnt1 += 1;
            if pred_pos {
                tp += 1;
            }
        } else {
            sum0 += x;
            cnt0 += 1;
            if pred_pos {
                fp += 1;
            }
        }
    }
    let mu0 = sum0 / (cnt0.max(1) as f64);
    let mu1 = sum1 / (cnt1.max(1) as f64);
    let tpr = (tp as f64 / (cnt1.max(1) as f64)).clamp(0.05, 0.95);
    let fpr = (fp as f64 / (cnt0.max(1) as f64)).clamp(0.01, (tpr - 0.02).max(0.02));
    let target_posteriors: Vec<f64> = x_target
        .iter()
        .map(|&x| {
            let p = 1.0 / (1.0 + (-(b0 + b1 * x).clamp(-20.0, 20.0)).exp());
            p.clamp(0.001, 0.999)
        })
        .collect();

    (p_base, tpr, fpr, mu0, mu1, target_posteriors)
}

/// Exact smallest singular value $\sigma_{\min}(A)$ of a $2 \times 2$ matrix $\begin{pmatrix} a & b \\ c & d \end{pmatrix}$.
fn smallest_singular_value_2x2(a: f64, b: f64, c: f64, d: f64) -> f64 {
    let frob2 = a * a + b * b + c * c + d * d;
    let det = (a * d - b * c).abs();
    let disc = (frob2 * frob2 - 4.0 * det * det).max(0.0).sqrt();
    (0.5 * (frob2 - disc)).max(0.0).sqrt()
}

/// Solves the Entropic Optimal Transport Label Shift problem (OTTER) via Sinkhorn-Knopp matrix scaling
/// on cost $C_{i,j} = -\ln P_\theta(Y=j \mid X=x_i)$ coupled with confusion operator $M$.
fn solve_otter_sinkhorn(
    source_prior_p1: f64,
    source_tpr: f64,
    source_fpr: f64,
    target_pred_pos_rate: f64,
    target_posteriors: &[f64],
) -> f64 {
    let n = target_posteriors.len();
    let inv_n = 1.0 / (n as f64);
    let p1_s = source_prior_p1.clamp(1e-6, 1.0 - 1e-6);
    let p0_s = 1.0 - p1_s;
    let mu_t = [
        (1.0 - target_pred_pos_rate).clamp(1e-6, 1.0 - 1e-6),
        target_pred_pos_rate.clamp(1e-6, 1.0 - 1e-6),
    ];

    // Gibbs kernel K_{i,j} = exp(-C_{i,j} / eps_ot) with class-conditional likelihood weighting
    let mut k_mat = Vec::with_capacity(n);
    for &p in target_posteriors {
        let pc = p.clamp(1e-6, 1.0 - 1e-6);
        // C_{i,j} = -ln P_theta(Y=j | X=x_i) adjusted by source prior so K_{i,j} proportional to P(X_i | Y=j)
        let k0 = (1.0 - pc) / p0_s;
        let k1 = pc / p1_s;
        k_mat.push([k0.max(1e-12), k1.max(1e-12)]);
    }

    // Evaluate Entropic Optimal Transport / Sinkhorn divergence objective over candidate target marginals q = (1 - q1, q1)
    let eval_sinkhorn = |q1: f64| -> f64 {
        let q = [1.0 - q1, q1];
        let mut u = vec![1.0; n];
        let mut v = [1.0, 1.0];
        for _ in 0..15 {
            for (i, u_i) in u.iter_mut().enumerate() {
                let denom = k_mat[i][0] * v[0] + k_mat[i][1] * v[1];
                *u_i = inv_n / denom.max(1e-15);
            }
            for j in 0..2 {
                let col_sum: f64 = (0..n).map(|i| u[i] * k_mat[i][j]).sum();
                v[j] = q[j] / col_sum.max(1e-15);
            }
        }

        // Confusion-pushed target marginal M * q = [(1 - FPR)(1 - q1) + (1 - TPR)q1, FPR(1 - q1) + TPR * q1]
        let mq = [
            ((1.0 - source_fpr) * q[0] + (1.0 - source_tpr) * q[1]).clamp(1e-9, 1.0 - 1e-9),
            (source_fpr * q[0] + source_tpr * q[1]).clamp(1e-9, 1.0 - 1e-9),
        ];
        let kl_confusion = mu_t[0] * (mu_t[0] / mq[0]).ln() + mu_t[1] * (mu_t[1] / mq[1]).ln();

        // Sinkhorn dual KL consistency on k_mat between scaled coupling Pi = diag(u) K diag(v)
        // and the unscaled Bayes reference coupling Pi^(0)_{i,j} = (1/n) * K_{i,j} q_j / (K_{i,0} q_0 + K_{i,1} q_1)
        let mut kl_sinkhorn = 0.0;
        for i in 0..n {
            let ref_denom = (k_mat[i][0] * q[0] + k_mat[i][1] * q[1]).max(1e-15);
            for j in 0..2 {
                let pi_ij = (u[i] * k_mat[i][j] * v[j]).max(1e-15);
                let pi0_ij = (inv_n * k_mat[i][j] * q[j] / ref_denom).max(1e-15);
                kl_sinkhorn += pi_ij * (pi_ij / pi0_ij).ln();
            }
        }
        kl_confusion.max(0.0) + kl_sinkhorn.max(0.0)
    };

    let mut best_q1 = source_prior_p1.clamp(0.001, 0.999);
    let mut best_obj = f64::INFINITY;
    for step in 1..200 {
        let q1 = (step as f64) / 200.0;
        let obj = eval_sinkhorn(q1);
        if obj < best_obj {
            best_obj = obj;
            best_q1 = q1;
        }
    }
    // Golden-section sub-grid refinement around best_q1
    let inv_phi = 0.5 * (5.0_f64.sqrt() - 1.0);
    let mut a = (best_q1 - 0.01).clamp(0.001, 0.999);
    let mut b = (best_q1 + 0.01).clamp(0.001, 0.999);
    let mut c = b - inv_phi * (b - a);
    let mut d = a + inv_phi * (b - a);
    let mut fc = eval_sinkhorn(c);
    let mut fd = eval_sinkhorn(d);
    for _ in 0..32 {
        if fc <= fd {
            b = d;
            d = c;
            fd = fc;
            c = b - inv_phi * (b - a);
            fc = eval_sinkhorn(c);
        } else {
            a = c;
            c = d;
            fc = fd;
            d = a + inv_phi * (b - a);
            fd = eval_sinkhorn(d);
        }
    }
    (0.5 * (a + b)).clamp(0.001, 0.999)
}
