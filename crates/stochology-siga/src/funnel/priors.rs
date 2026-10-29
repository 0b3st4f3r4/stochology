//! Filter 3 Prior Predictive Check & Simulation-Based Calibration (`sbc_rank_chi2`,
//! `ln_gamma_lanczos`, `regularized_incomplete_beta`, `fit_shelf_beta_prior`, §3.1).

/// Evaluates the uniformity of Simulation-Based Calibration (SBC) rank counts via Pearson's $\chi^2$.
pub fn sbc_rank_chi2(rank_bins: &[usize]) -> f64 {
    assert!(!rank_bins.is_empty());
    let total: f64 = rank_bins.iter().map(|&obs| obs as f64).sum();
    if total <= 0.0 {
        return f64::INFINITY;
    }
    let expected = total / (rank_bins.len() as f64);
    rank_bins
        .iter()
        .map(|&obs| {
            let diff = (obs as f64) - expected;
            (diff * diff) / expected
        })
        .sum()
}

/// Lanczos approximation for $\ln \Gamma(z)$ ($z > 0$) with 9 coefficients (precision $\sim 10^{-15}$).
pub fn ln_gamma_lanczos(z: f64) -> f64 {
    assert!(z > 0.0, "ln_gamma_lanczos requires z > 0");
    const COEFFS: [f64; 9] = [
        0.999_999_999_999_809_9,
        676.520_368_121_885_1,
        -1_259.139_216_722_402_8,
        771.323_428_777_653_1,
        -176.615_029_162_140_6,
        12.507_343_278_686_905,
        -0.138_571_095_265_720_12,
        9.984_369_578_019_572e-6,
        1.505_632_735_149_311_6e-7,
    ];
    let z_minus_1 = z - 1.0;
    let mut x = COEFFS[0];
    for (i, &c) in COEFFS.iter().enumerate().skip(1) {
        x += c / (z_minus_1 + (i as f64));
    }
    let t = z_minus_1 + 7.5;
    0.5 * std::f64::consts::TAU.ln() + (z_minus_1 + 0.5) * t.ln() - t + x.ln()
}

/// Regularized incomplete Beta function $I_x(a, b) = \frac{B(x; a, b)}{B(a, b)}$ evaluated via
/// Lentz's continued-fraction algorithm (DLMF §8.17.22).
pub fn regularized_incomplete_beta(x: f64, a: f64, b: f64) -> f64 {
    assert!(a > 0.0 && b > 0.0, "requires a > 0 and b > 0");
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }
    let sym_threshold = (a + 1.0) / (a + b + 2.0);
    if x > sym_threshold {
        return 1.0 - regularized_incomplete_beta(1.0 - x, b, a);
    }
    let ln_beta = ln_gamma_lanczos(a) + ln_gamma_lanczos(b) - ln_gamma_lanczos(a + b);
    let front = (a * x.ln() + b * (1.0 - x).ln() - ln_beta).exp() / a;

    // Modified Lentz continued fraction for I_x(a, b)
    let tiny = 1e-30_f64;
    let mut c = 1.0_f64;
    let mut d = 1.0 - ((a + b) * x / (a + 1.0));
    if d.abs() < tiny {
        d = tiny;
    }
    d = 1.0 / d;
    let mut f = d;

    for m in 1..=200 {
        let m_f = m as f64;
        let two_m = 2.0 * m_f;
        // Even step d_{2m}
        let num_even = (m_f * (b - m_f) * x) / ((a + two_m - 1.0) * (a + two_m));
        d = 1.0 + num_even * d;
        if d.abs() < tiny {
            d = tiny;
        }
        c = 1.0 + num_even / c;
        if c.abs() < tiny {
            c = tiny;
        }
        d = 1.0 / d;
        f *= c * d;

        // Odd step d_{2m+1}
        let num_odd = -((a + m_f) * (a + b + m_f) * x) / ((a + two_m) * (a + two_m + 1.0));
        d = 1.0 + num_odd * d;
        if d.abs() < tiny {
            d = tiny;
        }
        c = 1.0 + num_odd / c;
        if c.abs() < tiny {
            c = tiny;
        }
        d = 1.0 / d;
        let delta = c * d;
        f *= delta;
        if (delta - 1.0).abs() < 1e-14 {
            break;
        }
    }
    (front * f).clamp(0.0, 1.0)
}

/// Fits a conjugate Beta prior $\mathrm{Beta}(\hat{\alpha}, \hat{\beta})$ to SHELF elicited quartiles
/// $(q_{25}, q_{50}, q_{75})$ (O'Hagan / Oakley — Filter 3, §3.1) by minimizing the exact CDF residual
/// $\bigl(I_{q_{25}}(\alpha, \beta) - 0.25\bigr)^2 + \bigl(I_{q_{50}}(\alpha, \beta) - 0.50\bigr)^2 + \bigl(I_{q_{75}}(\alpha, \beta) - 0.75\bigr)^2$.
/// Returns `(alpha_hat, beta_hat, cdf_rmse)`.
pub fn fit_shelf_beta_prior(q25: f64, q50: f64, q75: f64) -> (f64, f64, f64) {
    assert!(
        0.0 < q25 && q25 < q50 && q50 < q75 && q75 < 1.0,
        "requires 0 < q25 < q50 < q75 < 1"
    );
    // Method-of-moments warm start from median and IQR
    let iqr = (q75 - q25).max(1e-4);
    let sd_approx = (iqr / 1.349).max(1e-4);
    let mu0 = q50.clamp(0.01, 0.99);
    let conc0 = ((mu0 * (1.0 - mu0)) / (sd_approx * sd_approx) - 1.0).clamp(1.0, 400.0);

    let obj = |a: f64, b: f64| -> f64 {
        let r25 = regularized_incomplete_beta(q25, a, b) - 0.25;
        let r50 = regularized_incomplete_beta(q50, a, b) - 0.50;
        let r75 = regularized_incomplete_beta(q75, a, b) - 0.75;
        ((r25 * r25 + r50 * r50 + r75 * r75) / 3.0).sqrt()
    };

    let mut best_a = (mu0 * conc0).max(0.2);
    let mut best_b = ((1.0 - mu0) * conc0).max(0.2);
    let mut best_err = obj(best_a, best_b);

    for scale in [0.5, 0.2, 0.08, 0.02, 0.005] {
        for da in -4..=4 {
            for db in -4..=4 {
                let cand_a = (best_a * (1.0 + (da as f64) * scale)).clamp(0.1, 1000.0);
                let cand_b = (best_b * (1.0 + (db as f64) * scale)).clamp(0.1, 1000.0);
                let err = obj(cand_a, cand_b);
                if err < best_err {
                    best_err = err;
                    best_a = cand_a;
                    best_b = cand_b;
                }
            }
        }
    }
    (best_a, best_b, best_err)
}
