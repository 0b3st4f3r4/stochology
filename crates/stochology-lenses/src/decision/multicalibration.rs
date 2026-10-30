//! **Multicalibration** audit (Hébert-Johnson, Kim, Reingold & Rothblum 2018): calibration inside every
//! declared subgroup and forecast bin, not only on average — the calibration side of Bridge $L_1 \to L_3$
//! fairness, next to Chouldechova's error-rate audit.

use stochology_core::Rng;

use super::corp_murphy::CorpPavDecomposition;

/// Named subgroup memberships, one boolean per unit.
pub type SubgroupMemberships = Vec<(String, Vec<bool>)>;

/// One audited (subgroup, forecast bin) cell.
#[derive(Debug, Clone, PartialEq)]
pub struct MulticalibrationCell {
    pub subgroup: String,
    pub bin: usize,
    pub n: usize,
    /// $\mathbb{E}[Y - p \mid S, v]$ in the cell.
    pub mean_residual: f64,
    /// Hoeffding lower confidence bound of $|\mathbb{E}[Y - p \mid S, v]|$ at the Bonferroni level.
    pub gap_lower_bound: f64,
    /// Hoeffding upper confidence bound of the same gap.
    pub gap_upper_bound: f64,
    pub violates: bool,
}

/// $\alpha$-multicalibration audit (Hébert-Johnson, Kim, Reingold & Rothblum 2018, *ICML*, PMLR
/// 80:1939–1948, informative): for every declared subgroup $S$ and global forecast quintile $v$ with at
/// least `MIN_CELL` units, the cell violates when the Hoeffding lower bound of its calibration gap
/// $|\mathbb{E}[Y - p \mid S, v]|$, at level `LEVEL / #cells`, exceeds `TOLERANCE`. The verdict is
/// three-way: `Violated` (some lower bound above α), `Certified` (every upper bound at most α), or
/// `Inconclusive` — "no violation found" is never read as "multicalibrated". The global CORP
/// miscalibration is reported alongside, because a small global REL is exactly what hides a violation.
#[derive(Debug, Clone, PartialEq)]
pub struct MulticalibrationAudit {
    pub units: usize,
    pub subgroups: usize,
    pub cells_audited: usize,
    pub global_rel_corp: f64,
    pub violations: usize,
    pub worst_cell: Option<MulticalibrationCell>,
    /// Largest Hoeffding radius over audited cells (the smallest gap the audit can resolve everywhere).
    pub max_radius: f64,
    pub verdict: MulticalibrationVerdict,
    pub cells: Vec<MulticalibrationCell>,
}

/// Three-way verdict of a multicalibration audit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MulticalibrationVerdict {
    /// Some cell's gap is above the tolerance with confidence.
    Violated,
    /// Every cell's gap is at most the tolerance with confidence.
    Certified,
    /// Neither: the cells are too small to resolve gaps of the tolerance's size.
    Inconclusive,
}

impl MulticalibrationAudit {
    /// Assumed: largest admissible calibration gap α (§8 of `docs/research/pillar-expansion.md`).
    pub const TOLERANCE: f64 = 0.05;
    /// Assumed: smallest audited cell.
    pub const MIN_CELL: usize = 30;
    /// Assumed: number of global forecast bins (quintiles).
    pub const BINS: usize = 5;
    /// Assumed: family-wise level shared across cells (Bonferroni).
    pub const LEVEL: f64 = 0.05;

    /// Audits forecasts `p` against outcomes `y` over named subgroup memberships.
    pub fn audit(p: &[f64], y: &[f64], subgroups: &[(String, Vec<bool>)]) -> Self {
        assert_eq!(p.len(), y.len());
        assert!(!p.is_empty());
        assert!(subgroups.iter().all(|(_, m)| m.len() == p.len()));
        let n = p.len();
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_by(|&a, &b| p[a].total_cmp(&p[b]));
        let mut bin = vec![0usize; n];
        for (rank, &i) in order.iter().enumerate() {
            bin[i] = rank * Self::BINS / n;
        }

        let mut raw = Vec::new();
        for (label, members) in subgroups {
            for b in 0..Self::BINS {
                let (count, sum) = (0..n)
                    .filter(|&i| members[i] && bin[i] == b)
                    .fold((0usize, 0.0), |(c, s), i| (c + 1, s + y[i] - p[i]));
                if count >= Self::MIN_CELL {
                    raw.push((label.clone(), b, count, sum / count as f64));
                }
            }
        }
        let delta = Self::LEVEL / raw.len().max(1) as f64;
        let cells: Vec<MulticalibrationCell> = raw
            .into_iter()
            .map(|(subgroup, bin, count, mean_residual)| {
                // Each residual y - p lies in [-p, 1 - p] (width 1): |mean - E| <= sqrt(ln(2/delta) / 2n).
                let radius = ((2.0 / delta).ln() / (2.0 * count as f64)).sqrt();
                let gap_lower_bound = (mean_residual.abs() - radius).max(0.0);
                MulticalibrationCell {
                    subgroup,
                    bin,
                    n: count,
                    mean_residual,
                    gap_lower_bound,
                    gap_upper_bound: mean_residual.abs() + radius,
                    violates: gap_lower_bound > Self::TOLERANCE,
                }
            })
            .collect();
        let worst_cell = cells
            .iter()
            .max_by(|a, b| a.gap_lower_bound.total_cmp(&b.gap_lower_bound))
            .cloned();
        let violations = cells.iter().filter(|c| c.violates).count();
        let verdict = if violations > 0 {
            MulticalibrationVerdict::Violated
        } else if !cells.is_empty() && cells.iter().all(|c| c.gap_upper_bound <= Self::TOLERANCE) {
            MulticalibrationVerdict::Certified
        } else {
            MulticalibrationVerdict::Inconclusive
        };
        Self {
            units: n,
            subgroups: subgroups.len(),
            cells_audited: cells.len(),
            global_rel_corp: CorpPavDecomposition::compute(p, y).rel_miscalibration,
            violations,
            worst_cell,
            max_radius: cells
                .iter()
                .map(|c| c.gap_upper_bound - c.mean_residual.abs())
                .fold(0.0, f64::max),
            verdict,
            cells,
        }
    }

    /// Simulated predictor for the `subgroup_miscalibration` negative proof: forecast $p \sim U(0.2, 0.8)$,
    /// a half-population subgroup $G$ with true risk $p - 0.15$, and $p + 0.15$ outside it, so
    /// $\mathbb{E}[Y \mid p] = p$ exactly while each subgroup is off by $0.15$. Returns `(p, y, subgroups)`.
    pub fn simulate_hidden_subgroup_bias(
        seed: u64,
        n: usize,
    ) -> (Vec<f64>, Vec<f64>, SubgroupMemberships) {
        let mut rng = Rng::new(seed);
        let (mut p, mut y, mut in_g) = (Vec::new(), Vec::new(), Vec::new());
        for _ in 0..n {
            let forecast = rng.uniform(0.2, 0.8);
            let g = rng.bernoulli(0.5);
            let pi = if g { forecast - 0.15 } else { forecast + 0.15 };
            p.push(forecast);
            y.push(f64::from(u8::from(rng.bernoulli(pi))));
            in_g.push(g);
        }
        let not_g = in_g.iter().map(|&g| !g).collect();
        (p, y, vec![("G".into(), in_g), ("not G".into(), not_g)])
    }
}
