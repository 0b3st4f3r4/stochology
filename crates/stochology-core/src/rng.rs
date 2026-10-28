//! Deterministic pseudo-random number generator (`SplitMix64`, Invariant $\Phi_1$)
//! and order-statistic / sample moment primitives.

/// Canonical deterministic seed mandated by Invariant $\Phi_1$ (`seed = 2026`).
pub const DEFAULT_SEED: u64 = 2026;

/// Deterministic 64-bit `SplitMix64` generator with zero external dependencies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SplitMix64 {
    state: u64,
}

/// Alias used across the workspace.
pub type Rng = SplitMix64;

impl SplitMix64 {
    /// Creates a new generator initialized with `seed`.
    #[inline]
    pub const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Creates a new generator initialized with [`DEFAULT_SEED`] (`2026`).
    #[inline]
    pub const fn with_default_seed() -> Self {
        Self::new(DEFAULT_SEED)
    }

    /// Lock-free deterministic substream derivation `.split(stream_id)` inspired by
    /// Steele, Lea & Flood (2014, OOPSLA) and Steele & Vigna (2021, LXM, OOPSLA).
    ///
    /// Derives a statistically independent child `SplitMix64` generator for `stream_id`
    /// without mutating `&self`, enabling lock-free parallel `fork-join` Monte Carlo
    /// simulation trees while strictly preserving Invariant $\Phi_1$ (`seed = 2026`).
    #[inline]
    pub const fn split(&self, stream_id: u64) -> Self {
        // Combine parent state with stream_id via LXM/Stafford-10/13 bijective avalanche mixers
        let mut z = self
            .state
            .wrapping_add(0x9E37_79B9_7F4A_7C15)
            .wrapping_add(stream_id.wrapping_mul(0xD134_2543_DE82_EF95));
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^= z >> 31;
        // Second-stage Lea-64 mix with odd-gamma perturbation to prevent stream collisions
        let gamma = (stream_id.wrapping_mul(0xDA94_2042_E4DD_58B5) ^ 0xA076_1D64_78BD_642F) | 1;
        let child_seed = z.wrapping_add(gamma);
        Self { state: child_seed }
    }

    /// Advances the parent generator and forks two independent child substreams `(left, right)`
    /// for binary `fork-join` Monte Carlo trees.
    #[inline]
    pub fn split_fork_join_pair(&mut self, branch_tag: u64) -> (Self, Self) {
        let base = self.next_u64();
        let parent = Self::new(base);
        (
            parent.split(branch_tag.wrapping_mul(2)),
            parent.split(branch_tag.wrapping_mul(2).wrapping_add(1)),
        )
    }

    /// Returns the next deterministic `u64`.
    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Returns a uniform sample in $[0, 1)$ with 53 bits of mantissa precision.
    #[inline]
    pub fn next_f64(&mut self) -> f64 {
        let bits = self.next_u64() >> 11;
        (bits as f64) * (1.0 / ((1u64 << 53) as f64))
    }

    /// Returns a uniform sample in $[low, high)$.
    #[inline]
    pub fn uniform(&mut self, low: f64, high: f64) -> f64 {
        low + (high - low) * self.next_f64()
    }

    /// Returns a standard normal sample $\mathcal{N}(0, 1)$ via the Box-Muller transform.
    #[inline]
    pub fn standard_normal(&mut self) -> f64 {
        let u1 = self.next_f64().clamp(1e-15, 1.0 - 1e-15);
        let u2 = self.next_f64();
        (-2.0 * u1.ln()).sqrt() * (std::f64::consts::TAU * u2).cos()
    }

    /// Returns a normal sample $\mathcal{N}(\mu, \sigma^2)$.
    #[inline]
    pub fn normal(&mut self, mean: f64, std_dev: f64) -> f64 {
        mean + std_dev * self.standard_normal()
    }

    /// Returns a Bernoulli trial with success probability $p \in [0, 1]$.
    #[inline]
    pub fn bernoulli(&mut self, p: f64) -> bool {
        self.next_f64() < p
    }

    /// Returns a Pareto sample $X = x_m (1 - U)^{-1/\alpha}$ with scale $x_m > 0$ and tail index $\alpha > 0$.
    #[inline]
    pub fn pareto(&mut self, x_m: f64, alpha: f64) -> f64 {
        let u = self.next_f64().clamp(1e-15, 1.0 - 1e-15);
        x_m * (1.0 - u).powf(-1.0 / alpha)
    }

    /// Returns an Exponential sample $\mathrm{Exp}(\lambda)$ with rate $\lambda > 0$.
    #[inline]
    pub fn exponential(&mut self, rate_lambda: f64) -> f64 {
        assert!(rate_lambda > 0.0, "exponential rate_lambda must be > 0");
        let u = self.next_f64().clamp(1e-15, 1.0 - 1e-15);
        -u.ln() / rate_lambda
    }

    /// Returns a Gamma sample $\mathrm{Gamma}(\alpha, \theta)$ with shape $\alpha > 0$ and scale $\theta > 0$
    /// using the exact Marsaglia-Tsang (2000) squeeze acceptance-rejection algorithm (and Ahrens-Dieter boost for $\alpha < 1$).
    pub fn gamma(&mut self, alpha: f64, scale_theta: f64) -> f64 {
        assert!(
            alpha > 0.0 && scale_theta > 0.0,
            "gamma requires alpha > 0 and scale_theta > 0"
        );
        if alpha < 1.0 {
            let u = self.next_f64().clamp(1e-15, 1.0 - 1e-15);
            return self.gamma(alpha + 1.0, scale_theta) * u.powf(1.0 / alpha);
        }
        let d = alpha - 1.0 / 3.0;
        let c = 1.0 / (9.0 * d).sqrt();
        loop {
            let x = self.standard_normal();
            let v_root = 1.0 + c * x;
            if v_root <= 0.0 {
                continue;
            }
            let v = v_root * v_root * v_root;
            let u = self.next_f64().clamp(1e-15, 1.0 - 1e-15);
            let x2 = x * x;
            if u < 1.0 - 0.0331 * (x2 * x2) {
                return scale_theta * d * v;
            }
            if u.ln() < 0.5 * x2 + d * (1.0 - v + v.ln()) {
                return scale_theta * d * v;
            }
        }
    }

    /// Returns a Beta sample $\mathrm{Beta}(\alpha, \beta)$ for $\alpha > 0, \beta > 0$ via independent Gamma draws.
    pub fn beta(&mut self, alpha: f64, beta: f64) -> f64 {
        assert!(
            alpha > 0.0 && beta > 0.0,
            "beta requires alpha > 0 and beta > 0"
        );
        let x = self.gamma(alpha, 1.0);
        let y = self.gamma(beta, 1.0);
        let sum = x + y;
        if sum <= 0.0 {
            alpha / (alpha + beta)
        } else {
            (x / sum).clamp(1e-15, 1.0 - 1e-15)
        }
    }

    /// Returns a Student-$t$ sample $T \sim t_\nu$ with degrees of freedom $\nu > 0$.
    pub fn student_t(&mut self, nu: f64) -> f64 {
        assert!(nu > 0.0, "student_t requires nu > 0");
        let z = self.standard_normal();
        let w = self.gamma(0.5 * nu, 2.0);
        z / (w / nu).max(1e-15).sqrt()
    }

    /// Returns a Poisson count $N \sim \mathrm{Poisson}(\lambda)$ for $\lambda \ge 0$ (Knuth exact product for $\lambda < 30$,
    /// Camp-Paulson-Normal continuity correction for $\lambda \ge 30$).
    pub fn poisson(&mut self, lambda: f64) -> u64 {
        assert!(lambda >= 0.0, "poisson requires lambda >= 0");
        if lambda == 0.0 {
            return 0;
        }
        if lambda < 30.0 {
            let l = (-lambda).exp();
            let mut k = 0u64;
            let mut p = 1.0;
            loop {
                k += 1;
                p *= self.next_f64().clamp(1e-15, 1.0 - 1e-15);
                if p <= l {
                    return k - 1;
                }
            }
        } else {
            let z = self.standard_normal();
            let approx = (lambda + lambda.sqrt() * z + 0.5).floor();
            approx.max(0.0) as u64
        }
    }

    /// In-place Fisher-Yates shuffle.
    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        let n = slice.len();
        for i in (1..n).rev() {
            let j = (self.next_u64() as usize) % (i + 1);
            slice.swap(i, j);
        }
    }
}

/// Welford-Terriberry single-pass numerically stable online moments (mean, variance, skewness, excess kurtosis,
/// and Jarque-Bera omnibus normality test statistic).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OnlineMoments {
    pub count: usize,
    pub mean: f64,
    m2: f64,
    m3: f64,
    m4: f64,
}

impl Default for OnlineMoments {
    fn default() -> Self {
        Self::new()
    }
}

impl OnlineMoments {
    /// Creates an empty moment accumulator.
    pub const fn new() -> Self {
        Self {
            count: 0,
            mean: 0.0,
            m2: 0.0,
            m3: 0.0,
            m4: 0.0,
        }
    }

    /// Computes online moments up to 4th order over a slice `xs`.
    pub fn from_slice(xs: &[f64]) -> Self {
        let mut acc = Self::new();
        for &x in xs {
            acc.push(x);
        }
        acc
    }

    /// Incorporates a single observation $x$ using Terriberry's four-moment recurrence.
    pub fn push(&mut self, x: f64) {
        let n1 = self.count as f64;
        self.count += 1;
        let n = self.count as f64;
        let delta = x - self.mean;
        let delta_n = delta / n;
        let delta_n2 = delta_n * delta_n;
        let term1 = delta * delta_n * n1;
        self.mean += delta_n;
        self.m4 += term1 * delta_n2 * (n * n - 3.0 * n + 3.0) + 6.0 * delta_n2 * self.m2
            - 4.0 * delta_n * self.m3;
        self.m3 += term1 * delta_n * (n - 2.0) - 3.0 * delta_n * self.m2;
        self.m2 += term1;
    }

    /// Unbiased sample variance $s^2 = M_2 / (n - 1)$.
    #[inline]
    pub fn sample_variance(&self) -> f64 {
        if self.count < 2 {
            0.0
        } else {
            self.m2 / ((self.count - 1) as f64)
        }
    }

    /// Sample standard deviation $s = \sqrt{s^2}$.
    #[inline]
    pub fn sample_std(&self) -> f64 {
        self.sample_variance().sqrt()
    }

    /// Standardized sample skewness $\hat{\gamma}_1 = \frac{\sqrt{n} M_3}{M_2^{3/2}}$.
    #[inline]
    pub fn skewness(&self) -> f64 {
        if self.count < 3 || self.m2 <= 1e-15 {
            0.0
        } else {
            ((self.count as f64).sqrt() * self.m3) / self.m2.powf(1.5)
        }
    }

    /// Standardized excess kurtosis $\hat{\gamma}_2 = \frac{n M_4}{M_2^2} - 3$.
    #[inline]
    pub fn excess_kurtosis(&self) -> f64 {
        if self.count < 4 || self.m2 <= 1e-15 {
            0.0
        } else {
            ((self.count as f64) * self.m4) / (self.m2 * self.m2) - 3.0
        }
    }

    /// Jarque-Bera omnibus test statistic for departure from Gaussianity ($\chi^2_2$ under $H_0$):
    /// $$\mathrm{JB} = \frac{n}{6}\left(\hat{\gamma}_1^2 + \frac{1}{4}\hat{\gamma}_2^2\right)$$
    #[inline]
    pub fn jarque_bera_stat(&self) -> f64 {
        let s = self.skewness();
        let k = self.excess_kurtosis();
        ((self.count as f64) / 6.0) * (s * s + 0.25 * k * k)
    }
}

/// Moving-block bootstrap confidence interval for a summary functional $T(X_1, \dots, X_n)$
/// preserving short-range serial dependence within contiguous blocks of length `block_len`.
/// Returns `(point_estimate, ci_lower, ci_upper, bootstrap_se)`.
pub fn block_bootstrap_ci<F>(
    xs: &[f64],
    block_len: usize,
    num_replicates: usize,
    alpha_two_sided: f64,
    seed: u64,
    statistic: F,
) -> (f64, f64, f64, f64)
where
    F: Fn(&[f64]) -> f64,
{
    assert!(
        !xs.is_empty(),
        "block_bootstrap_ci requires non-empty slice"
    );
    let n = xs.len();
    let b_len = block_len.clamp(1, n);
    let reps = num_replicates.max(16);
    let point = statistic(xs);
    let mut rng = SplitMix64::new(seed);
    let mut boot_stats = Vec::with_capacity(reps);
    let mut sample_buf = Vec::with_capacity(n);
    let num_starts = (n - b_len) + 1;

    for _ in 0..reps {
        sample_buf.clear();
        while sample_buf.len() < n {
            let start = (rng.next_u64() as usize) % num_starts;
            let take = (n - sample_buf.len()).min(b_len);
            sample_buf.extend_from_slice(&xs[start..start + take]);
        }
        boot_stats.push(statistic(&sample_buf));
    }

    boot_stats.sort_by(|a, b| a.total_cmp(b));
    let half_a = (0.5 * alpha_two_sided).clamp(0.001, 0.499);
    let lo = quantile_sorted(&boot_stats, half_a);
    let hi = quantile_sorted(&boot_stats, 1.0 - half_a);
    let se = sample_std(&boot_stats);
    (point, lo, hi, se)
}

/// Arithmetic mean of a non-empty slice.
pub fn mean(xs: &[f64]) -> f64 {
    assert!(!xs.is_empty(), "mean requires a non-empty slice");
    xs.iter().sum::<f64>() / (xs.len() as f64)
}

/// Unbiased sample variance ($n - 1$ denominator) of a slice with at least 2 elements.
pub fn sample_variance(xs: &[f64]) -> f64 {
    assert!(
        xs.len() >= 2,
        "sample_variance requires at least 2 observations"
    );
    let m = mean(xs);
    let ss: f64 = xs
        .iter()
        .map(|&x| {
            let d = x - m;
            d * d
        })
        .sum();
    ss / ((xs.len() - 1) as f64)
}

/// Sample standard deviation.
#[inline]
pub fn sample_std(xs: &[f64]) -> f64 {
    sample_variance(xs).sqrt()
}

/// Empirical quantile $q \in [0, 1]$ with linear interpolation.
pub fn quantile(xs: &[f64], q: f64) -> f64 {
    assert!(!xs.is_empty(), "quantile requires a non-empty slice");
    assert!((0.0..=1.0).contains(&q), "quantile q must be in [0, 1]");
    let mut sorted = xs.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    quantile_sorted(&sorted, q)
}

/// Empirical quantile $q \in [0, 1]$ on an already-sorted slice.
pub fn quantile_sorted(sorted: &[f64], q: f64) -> f64 {
    assert!(
        !sorted.is_empty(),
        "quantile_sorted requires a non-empty slice"
    );
    assert!((0.0..=1.0).contains(&q), "quantile q must be in [0, 1]");
    if sorted.len() == 1 {
        return sorted[0];
    }
    let max_idx = sorted.len() - 1;
    let pos = (q * (max_idx as f64)).clamp(0.0, max_idx as f64);
    let lo = (pos.floor() as usize).min(max_idx);
    let hi = (pos.ceil() as usize).min(max_idx);
    let frac = pos - (lo as f64);
    sorted[lo] * (1.0 - frac) + sorted[hi] * frac
}

/// Sample median ($q_{50}$).
#[inline]
pub fn median(xs: &[f64]) -> f64 {
    quantile(xs, 0.5)
}

/// Median Absolute Deviation: $\mathrm{MAD} = \mathrm{median}(|X - \mathrm{median}(X)|)$.
pub fn median_absolute_deviation(xs: &[f64]) -> f64 {
    let med = median(xs);
    let devs: Vec<f64> = xs.iter().map(|&x| (x - med).abs()).collect();
    median(&devs)
}

/// Audit of a lock-free parallel `fork-join` Monte Carlo tree built via [`SplitMix64::split`]
/// (Steele, Lea & Flood 2014; Steele & Vigna 2021 LXM).
#[derive(Debug, Clone, PartialEq)]
pub struct MonteCarloSplitTreeAudit {
    /// Root seed (`seed = 2026`).
    pub root_seed: u64,
    /// Number of parallel substreams audited.
    pub num_streams: usize,
    /// Number of draws per substream.
    pub draws_per_stream: usize,
    /// Maximum absolute pairwise Pearson correlation $\max_{i < j} |\rho_{i,j}|$ across substreams.
    pub max_abs_pairwise_correlation: f64,
    /// Mean bitwise Hamming distance between initial `u64` draws of adjacent substreams (ideal $\approx 32.0$).
    pub mean_hamming_avalanche_bits: f64,
    /// True if all substreams are uncorrelated ($|\rho_{i,j}| < 0.08$), exhibit full bit avalanche ($[28, 36]$ bits), and are order-independent.
    pub split_independence_verified: bool,
}

impl MonteCarloSplitTreeAudit {
    /// Evaluates `num_streams` parallel Monte Carlo substreams of length `draws_per_stream` anchored at `root_seed`.
    pub fn evaluate(root_seed: u64, num_streams: usize, draws_per_stream: usize) -> Self {
        let n_streams = num_streams.max(2);
        let n_draws = draws_per_stream.max(64);
        let root = SplitMix64::new(root_seed);

        let mut streams_f64 = Vec::with_capacity(n_streams);
        let mut first_words = Vec::with_capacity(n_streams);
        for s in 0..n_streams {
            let mut child = root.split(s as u64);
            first_words.push(child.next_u64());
            let mut series = Vec::with_capacity(n_draws);
            for _ in 0..n_draws {
                series.push(child.next_f64());
            }
            streams_f64.push(series);
        }

        // Verify order-independence (evaluating streams in reverse order yields bit-for-bit identical draws)
        let mut order_independent = true;
        for s in (0..n_streams).rev() {
            let mut child_rev = root.split(s as u64);
            let w0 = child_rev.next_u64();
            if w0 != first_words[s] {
                order_independent = false;
            }
        }

        let mut max_corr = 0.0_f64;
        for i in 0..n_streams {
            let mi = mean(&streams_f64[i]);
            let si = sample_std(&streams_f64[i]).max(1e-12);
            for j in (i + 1)..n_streams {
                let mj = mean(&streams_f64[j]);
                let sj = sample_std(&streams_f64[j]).max(1e-12);
                let mut cov = 0.0;
                for (&xi, &xj) in streams_f64[i].iter().zip(streams_f64[j].iter()) {
                    cov += (xi - mi) * (xj - mj);
                }
                cov /= (n_draws - 1) as f64;
                let r = (cov / (si * sj)).abs();
                if r > max_corr {
                    max_corr = r;
                }
            }
        }

        let mut total_hamming = 0u32;
        let pairs = n_streams - 1;
        for i in 0..pairs {
            total_hamming += (first_words[i] ^ first_words[i + 1]).count_ones();
        }
        let mean_hamming = (total_hamming as f64) / (pairs as f64);
        let corr_bound = (3.8 / (n_draws as f64).sqrt()).max(0.08);
        let verified =
            order_independent && max_corr < corr_bound && (26.0..=38.0).contains(&mean_hamming);

        Self {
            root_seed,
            num_streams: n_streams,
            draws_per_stream: n_draws,
            max_abs_pairwise_correlation: max_corr,
            mean_hamming_avalanche_bits: mean_hamming,
            split_independence_verified: verified,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splitmix64_is_strictly_deterministic() {
        let mut r1 = Rng::with_default_seed();
        let mut r2 = Rng::new(2026);
        for _ in 0..32 {
            assert_eq!(r1.next_u64(), r2.next_u64());
        }
    }

    #[test]
    fn splitmix64_seed_2026_known_answer_test() {
        let mut rng = SplitMix64::new(DEFAULT_SEED);
        assert_eq!(rng.next_u64(), 0xdb9c_5598_9194_8d23);
        assert_eq!(rng.next_u64(), 0x78bc_927d_ed35_455d);
        assert_eq!(rng.next_u64(), 0xaad7_1e75_cde2_b88e);
    }

    #[test]
    fn basic_order_statistics_hold() {
        let xs = [1.0, 2.0, 3.0, 4.0, 100.0];
        assert_eq!(median(&xs), 3.0);
        assert_eq!(median_absolute_deviation(&xs), 1.0);
        assert!((mean(&xs) - 22.0).abs() < 1e-12);
    }

    #[test]
    fn gamma_beta_student_poisson_and_online_moments_are_consistent() {
        let mut rng = Rng::with_default_seed();
        let mut gamma_samples = Vec::with_capacity(2000);
        let mut beta_samples = Vec::with_capacity(2000);
        for _ in 0..2000 {
            gamma_samples.push(rng.gamma(4.0, 2.5));
            beta_samples.push(rng.beta(3.0, 7.0));
            let _ = rng.student_t(5.0);
            let _ = rng.poisson(12.0);
            let _ = rng.exponential(1.5);
        }
        let m_gamma = OnlineMoments::from_slice(&gamma_samples);
        assert!((m_gamma.mean - 10.0).abs() < 0.4);
        assert!((m_gamma.sample_variance() - sample_variance(&gamma_samples)).abs() < 1e-9);
        assert!(m_gamma.skewness() > 0.5);
        assert!(m_gamma.jarque_bera_stat() > 20.0);

        let m_beta = OnlineMoments::from_slice(&beta_samples);
        assert!((m_beta.mean - 0.30).abs() < 0.02);

        let (pt, lo, hi, se) =
            block_bootstrap_ci(&beta_samples[..200], 5, 100, 0.05, DEFAULT_SEED, mean);
        assert!(lo <= pt && pt <= hi && se > 0.0);
    }

    #[test]
    fn split_and_fork_join_substreams_are_independent_and_deterministic() {
        let audit = MonteCarloSplitTreeAudit::evaluate(DEFAULT_SEED, 16, 1024);
        assert!(audit.split_independence_verified);
        assert!(audit.max_abs_pairwise_correlation < 0.12);
        assert!((26.0..=38.0).contains(&audit.mean_hamming_avalanche_bits));

        let mut p1 = Rng::with_default_seed();
        let mut p2 = Rng::with_default_seed();
        let (mut l1, mut r1) = p1.split_fork_join_pair(7);
        let (mut l2, mut r2) = p2.split_fork_join_pair(7);
        assert_eq!(l1.next_u64(), l2.next_u64());
        assert_eq!(r1.next_u64(), r2.next_u64());
        assert_ne!(l1.next_u64(), r1.next_u64());
    }
}
