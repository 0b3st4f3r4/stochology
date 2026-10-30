//! **Anchor regression** (Rothenhäusler, Meinshausen, Bühlmann & Peters 2021): trading in-sample fit
//! for robustness to shifts along exogenous anchors, with the anchor-stability diagnostic.

use stochology_core::{Rng, solve_linear_system};

/// Anchor regression (Rothenhäusler, Meinshausen, Bühlmann & Peters 2021, *J. R. Stat. Soc. B*
/// 83(2):215–246, informative):
/// $$\hat{b}_\gamma = \arg\min_b \|(I - \Pi_A)(Y - Xb)\|^2 + \gamma\,\|\Pi_A(Y - Xb)\|^2,$$
/// solved as OLS on $W_\gamma = I + (\sqrt{\gamma} - 1)\Pi_A$. With categorical anchors, $\Pi_A$ replaces
/// each value by its anchor-cell mean. $\gamma = 1$ is OLS; $\gamma \to \infty$ approaches the IV solution.
/// Anchor stability (OLS ≈ anchor) is read on the slope coefficients (columns after the first).
#[derive(Debug, Clone, PartialEq)]
pub struct AnchorRegressionAudit {
    pub gamma_grid: Vec<f64>,
    /// Coefficients (intercept first) for each γ of the grid.
    pub coefficients: Vec<Vec<f64>>,
    pub train_mse: Vec<f64>,
    pub test_pooled_mse: Vec<f64>,
    /// Largest per-anchor-cell test MSE (the empirical worst case over test environments).
    pub test_worst_cell_mse: Vec<f64>,
    /// $\|\hat{b}_\gamma - \hat{b}_1\| / \|\hat{b}_1\|$ over slopes at `PRIMARY_GAMMA`.
    pub relative_slope_gap: f64,
    pub anchor_stable: bool,
}

/// A design matrix with its response and anchor cell per row.
#[derive(Debug, Clone, PartialEq)]
pub struct AnchoredSample {
    pub x: Vec<Vec<f64>>,
    pub y: Vec<f64>,
    pub cell: Vec<usize>,
}

impl AnchorRegressionAudit {
    /// Assumed: shift strengths audited.
    pub const GAMMA_GRID: [f64; 5] = [1.0, 2.0, 5.0, 10.0, 100.0];
    /// Assumed: primary shift strength.
    pub const PRIMARY_GAMMA: f64 = 5.0;
    /// Assumed: largest relative slope gap at which an invariance claim is admissible.
    pub const STABILITY_TOLERANCE: f64 = 0.10;

    /// Fits $\hat{b}_\gamma$ on `sample`.
    pub fn fit(sample: &AnchoredSample, gamma: f64) -> Vec<f64> {
        assert!(
            gamma > 0.0,
            "gamma = 0 removes the anchor component and can make the system singular"
        );
        let n = sample.y.len();
        assert!(n > 0 && sample.x.len() == n && sample.cell.len() == n);
        let p = sample.x[0].len();
        let cells = sample.cell.iter().copied().max().unwrap_or(0) + 1;
        let mut count = vec![0.0; cells];
        let mut sum_y = vec![0.0; cells];
        let mut sum_x = vec![vec![0.0; p]; cells];
        for i in 0..n {
            let c = sample.cell[i];
            count[c] += 1.0;
            sum_y[c] += sample.y[i];
            for (acc, &v) in sum_x[c].iter_mut().zip(&sample.x[i]) {
                *acc += v;
            }
        }
        let shrink = gamma.sqrt() - 1.0;
        let mut xtx = vec![vec![0.0; p]; p];
        let mut xty = vec![0.0; p];
        let mut row = vec![0.0; p];
        for i in 0..n {
            let c = sample.cell[i];
            for j in 0..p {
                row[j] = sample.x[i][j] + shrink * sum_x[c][j] / count[c];
            }
            let yt = sample.y[i] + shrink * sum_y[c] / count[c];
            for j in 0..p {
                xty[j] += row[j] * yt;
                for k in 0..p {
                    xtx[j][k] += row[j] * row[k];
                }
            }
        }
        solve_linear_system(&xtx, &xty).expect("anchor normal equations must be non-singular")
    }

    fn mse_by_cell(sample: &AnchoredSample, b: &[f64]) -> (f64, f64) {
        let cells = sample.cell.iter().copied().max().unwrap_or(0) + 1;
        let mut sse = vec![0.0; cells];
        let mut count = vec![0usize; cells];
        for ((x, &y), &c) in sample.x.iter().zip(&sample.y).zip(&sample.cell) {
            let r = y - x.iter().zip(b).map(|(a, w)| a * w).sum::<f64>();
            sse[c] += r * r;
            count[c] += 1;
        }
        let pooled = sse.iter().sum::<f64>() / sample.y.len() as f64;
        let worst = sse
            .iter()
            .zip(&count)
            .filter(|&(_, &k)| k > 0)
            .map(|(s, &k)| s / k as f64)
            .fold(0.0, f64::max);
        (pooled, worst)
    }

    /// Fits the γ grid on `train` and scores every fit on the shifted `test` sample.
    pub fn evaluate(train: &AnchoredSample, test: &AnchoredSample) -> Self {
        let coefficients: Vec<Vec<f64>> = Self::GAMMA_GRID
            .iter()
            .map(|&g| Self::fit(train, g))
            .collect();
        let mut train_mse = Vec::new();
        let mut test_pooled_mse = Vec::new();
        let mut test_worst_cell_mse = Vec::new();
        for b in &coefficients {
            train_mse.push(Self::mse_by_cell(train, b).0);
            let (pooled, worst) = Self::mse_by_cell(test, b);
            test_pooled_mse.push(pooled);
            test_worst_cell_mse.push(worst);
        }
        let ols = &coefficients[0];
        let primary = Self::GAMMA_GRID
            .iter()
            .position(|&g| g == Self::PRIMARY_GAMMA)
            .expect("primary gamma is on the grid");
        let anchor = &coefficients[primary];
        let norm = |v: &[f64]| v.iter().map(|a| a * a).sum::<f64>().sqrt();
        let diff: Vec<f64> = anchor[1..]
            .iter()
            .zip(&ols[1..])
            .map(|(a, b)| a - b)
            .collect();
        let relative_slope_gap = norm(&diff) / norm(&ols[1..]).max(f64::MIN_POSITIVE);
        Self {
            gamma_grid: Self::GAMMA_GRID.to_vec(),
            coefficients,
            train_mse,
            test_pooled_mse,
            test_worst_cell_mse,
            relative_slope_gap,
            anchor_stable: relative_slope_gap <= Self::STABILITY_TOLERANCE,
        }
    }

    /// Simulated anchor-confounded structural model for the `anchor_extrapolation` negative proof:
    /// anchor cell effect $a_c$ and hidden confounder $H$ drive $X = a_c + H + \xi_X$, and
    /// $Y = X + 2H + \xi_Y$, so OLS absorbs $H$ and is not anchor-stable. `shift` scales the test anchors.
    pub fn simulate_confounded(seed: u64, n: usize, cells: usize, shift: f64) -> AnchoredSample {
        let mut rng = Rng::new(seed);
        let effects: Vec<f64> = (0..cells).map(|_| shift * rng.normal(0.0, 1.0)).collect();
        let mut sample = AnchoredSample {
            x: Vec::with_capacity(n),
            y: Vec::with_capacity(n),
            cell: Vec::with_capacity(n),
        };
        for i in 0..n {
            let c = i % cells;
            let h = rng.normal(0.0, 1.0);
            let x = effects[c] + h + rng.normal(0.0, 0.5);
            sample.x.push(vec![1.0, x]);
            sample.y.push(x + 2.0 * h + rng.normal(0.0, 0.5));
            sample.cell.push(c);
        }
        sample
    }
}
