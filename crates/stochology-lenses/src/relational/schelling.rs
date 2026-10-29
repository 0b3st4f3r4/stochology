//! Axis 3: Thomas Schelling's (1971) Dynamic Model of Segregation (*More Is Different*).

use stochology_core::Rng;

/// Cell state in Thomas Schelling's (1971) 2D dynamic segregation grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchellingCell {
    Empty,
    GroupA,
    GroupB,
}

/// Axis 3: Thomas Schelling's (1971) Dynamic Model of Segregation (*Journal of Mathematical Sociology*, 1, 143–186).
///
/// Demonstrates heteropathic emergence (*More Is Different*): mild individual tolerance ($\tau \approx 1/3$,
/// where every agent gladly accepts a 2/3 majority of the other group) triggers a nonlinear tipping cascade
/// that converges to severe macroscopic segregation ($\ge 78\%$).
#[derive(Debug, Clone)]
pub struct SchellingSegregationModel {
    pub grid_size: usize,
    pub moore_radius: usize,
    pub similarity_threshold_tau: f64,
    pub grid: Vec<SchellingCell>,
}

/// Comparative audit of Schelling's macroscopic phase transition across tolerance thresholds $\tau$.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SchellingPhaseTransitionAudit {
    /// Grid side length $L$ ($L \times L$ torus/grid).
    pub grid_size: usize,
    /// Initial random same-group neighbor fraction before dynamics ($\approx 0.50$).
    pub initial_similarity_fraction: f64,
    /// Sub-critical tolerance threshold $\tau_{\text{low}}$ (e.g., $0.125 = 1/8$).
    pub low_threshold_tau: f64,
    /// Equilibrium same-group neighbor fraction under $\tau_{\text{low}}$ (remains integrated $\approx 0.50\text{–}0.58$).
    pub integrated_similarity_at_low_tau: f64,
    /// Mild, non-hostile tolerance threshold $\tau_{\text{mild}}$ (e.g., $0.375 = 3/8$, tolerating $62.5\%$ out-group).
    pub mild_threshold_tau: f64,
    /// Emergent macroscopic same-group neighbor fraction under $\tau_{\text{mild}}$ ($\ge 0.78$).
    pub emergent_segregation_at_mild_tau: f64,
    /// Heteropathic emergence gap $\bar{s}_{\text{macro}}(\tau_{\text{mild}}) - \tau_{\text{mild}}$ quantifying unintended macro-segregation.
    pub heteropathic_emergence_gap: f64,
    /// Number of tipping relocation rounds until convergence under $\tau_{\text{mild}}$.
    pub convergence_rounds: usize,
    /// Whether the abrupt phase transition from integration to macro-segregation is verified.
    pub phase_transition_verified: bool,
}

impl SchellingSegregationModel {
    /// Initializes a deterministic $L \times L$ Schelling grid (`seed = 2026`) with `empty_fraction` vacant spaces
    /// and balanced populations of `GroupA` and `GroupB`.
    pub fn new(
        seed: u64,
        grid_size: usize,
        moore_radius: usize,
        similarity_threshold_tau: f64,
        empty_fraction: f64,
    ) -> Self {
        assert!(grid_size >= 6 && (1..=2).contains(&moore_radius));
        assert!((0.0..=1.0).contains(&similarity_threshold_tau));
        assert!(empty_fraction > 0.05 && empty_fraction < 0.50);

        let total_cells = grid_size * grid_size;
        let num_empty = ((total_cells as f64) * empty_fraction).round() as usize;
        let num_occupied = total_cells - num_empty;
        let num_a = num_occupied / 2;

        let mut grid = Vec::with_capacity(total_cells);
        grid.extend(std::iter::repeat_n(SchellingCell::Empty, num_empty));
        grid.extend(std::iter::repeat_n(SchellingCell::GroupA, num_a));
        grid.extend(std::iter::repeat_n(
            SchellingCell::GroupB,
            num_occupied - num_a,
        ));

        // Fisher-Yates shuffle with SplitMix64
        let mut rng = Rng::new(seed);
        for i in (1..total_cells).rev() {
            let j = (rng.next_u64() as usize) % (i + 1);
            grid.swap(i, j);
        }

        Self {
            grid_size,
            moore_radius,
            similarity_threshold_tau,
            grid,
        }
    }

    /// Computes the local fraction of same-group neighbors for a hypothetical `group` placed at `(row, col)`.
    pub fn local_similarity_at(&self, row: usize, col: usize, group: SchellingCell) -> f64 {
        if group == SchellingCell::Empty {
            return 1.0;
        }
        let l = self.grid_size as isize;
        let r = self.moore_radius as isize;
        let mut same = 0usize;
        let mut occupied = 0usize;

        for dr in -r..=r {
            for dc in -r..=r {
                if dr == 0 && dc == 0 {
                    continue;
                }
                let nr = row as isize + dr;
                let nc = col as isize + dc;
                if nr >= 0 && nr < l && nc >= 0 && nc < l {
                    let cell = self.grid[(nr as usize) * self.grid_size + (nc as usize)];
                    if cell != SchellingCell::Empty {
                        occupied += 1;
                        if cell == group {
                            same += 1;
                        }
                    }
                }
            }
        }
        if occupied == 0 {
            1.0
        } else {
            (same as f64) / (occupied as f64)
        }
    }

    /// Computes the mean same-group neighbor fraction across all occupied cells in the grid.
    pub fn mean_macro_similarity(&self) -> f64 {
        let mut sum_sim = 0.0;
        let mut count = 0usize;
        for idx in 0..self.grid.len() {
            let cell = self.grid[idx];
            if cell != SchellingCell::Empty {
                let row = idx / self.grid_size;
                let col = idx % self.grid_size;
                sum_sim += self.local_similarity_at(row, col, cell);
                count += 1;
            }
        }
        if count == 0 {
            0.0
        } else {
            sum_sim / (count as f64)
        }
    }

    /// Runs the tipping relocation cascade for up to `max_rounds` rounds, returning the number of rounds executed.
    pub fn run_until_equilibrium(&mut self, max_rounds: usize) -> usize {
        let total = self.grid.len();
        let mut rounds = 0usize;

        for _ in 0..max_rounds {
            let mut moved_any = false;
            for idx in 0..total {
                let agent = self.grid[idx];
                if agent == SchellingCell::Empty {
                    continue;
                }
                let r = idx / self.grid_size;
                let c = idx % self.grid_size;
                if self.local_similarity_at(r, c, agent) + 1e-12 < self.similarity_threshold_tau {
                    // Find the first vacant spot (scanning from a deterministic offset) that satisfies tau
                    self.grid[idx] = SchellingCell::Empty;
                    let mut best_vacant = None;
                    let mut best_sim = -1.0_f64;

                    for step in 1..total {
                        let cand_idx = (idx + step) % total;
                        if self.grid[cand_idx] == SchellingCell::Empty {
                            let cr = cand_idx / self.grid_size;
                            let cc = cand_idx % self.grid_size;
                            let sim = self.local_similarity_at(cr, cc, agent);
                            if sim + 1e-12 >= self.similarity_threshold_tau {
                                best_vacant = Some(cand_idx);
                                break;
                            } else if sim > best_sim {
                                best_sim = sim;
                                best_vacant = Some(cand_idx);
                            }
                        }
                    }

                    if let Some(target_idx) = best_vacant {
                        self.grid[target_idx] = agent;
                        if target_idx != idx {
                            moved_any = true;
                        }
                    } else {
                        self.grid[idx] = agent;
                    }
                }
            }
            rounds += 1;
            if !moved_any {
                break;
            }
        }
        rounds
    }

    /// Audits the macroscopic phase transition between sub-critical tolerance ($\tau = 0.125$)
    /// and mild tolerance ($\tau = 0.375$, where every agent tolerates $62.5\%$ out-group neighbors).
    pub fn audit_phase_transition(seed: u64) -> SchellingPhaseTransitionAudit {
        let grid_size = 16;
        let low_tau = 0.125;
        let mild_tau = 0.375;

        let mut model_low = Self::new(seed, grid_size, 1, low_tau, 0.15);
        let initial_sim = model_low.mean_macro_similarity();
        model_low.run_until_equilibrium(40);
        let sim_low = model_low.mean_macro_similarity();

        let mut model_mild = Self::new(seed, grid_size, 1, mild_tau, 0.15);
        let rounds = model_mild.run_until_equilibrium(60);
        let sim_mild = model_mild.mean_macro_similarity();
        let gap = sim_mild - mild_tau;

        SchellingPhaseTransitionAudit {
            grid_size,
            initial_similarity_fraction: initial_sim,
            low_threshold_tau: low_tau,
            integrated_similarity_at_low_tau: sim_low,
            mild_threshold_tau: mild_tau,
            emergent_segregation_at_mild_tau: sim_mild,
            heteropathic_emergence_gap: gap,
            convergence_rounds: rounds,
            phase_transition_verified: sim_low < 0.60 && sim_mild >= 0.78 && gap > 0.38,
        }
    }
}
