//! **Block 0 (Kernel) — Deterministic PRNG (`SplitMix64`), FIPS 180-4 SHA-256, and Mathematical Primitives.**
//!
//! Zero-dependency (`std` only) computational kernel for Stochology:
//! 1. Reproducible 64-bit `SplitMix64` pseudo-random number generator (`DEFAULT_SEED = 2026`),
//!    Steele–Vigna lock-free `.split(stream_id)` substream trees, and Welford–Terriberry online moments;
//! 2. FIPS 180-4 SHA-256 and RFC 2104 HMAC-SHA256 for dataset manifest verification (`data/SHA256SUMS`)
//!    and bitemporal `TemporalVault` sealing;
//! 3. Pool-Adjacent-Violators (PAV) isotonic regression, Cholesky SPD linear solver, ridge OLS,
//!    Gaussian CDF / quantile $\Phi, \Phi^{-1}$, Cromwell's antidogmatic clamp, and bitemporal quarter parsing.
//!
//! *Portability & Reproducibility Note:* Integer primitives (`SplitMix64::next_u64`, `.split()`, FIPS 180-4
//! SHA-256, and RFC 2104 HMAC-SHA256) are bit-exact across all target architectures. Transcendental
//! IEEE-754 operations (`.ln()`, `.cos()`, `.powf()`, `.exp()`) delegate to the target platform's `libm`
//! (`std::f64`) and are deterministic per target toolchain within 1–2 ULPs.

/// Canonical default seed for reproducible simulation (`seed = 2026`).
pub const DEFAULT_SEED: u64 = 2026;

/// Deterministic 64-bit `SplitMix64` generator with zero external dependencies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SplitMix64 {
    state: u64,
}

/// Alias used across the library.
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

    /// Lock-free deterministic substream derivation `.split(stream_id)` via Stafford Mix13
    /// bijective avalanche mixing and odd-gamma stream offsetting.
    #[inline]
    pub const fn split(&self, stream_id: u64) -> Self {
        let mut z = self
            .state
            .wrapping_add(0x9E37_79B9_7F4A_7C15)
            .wrapping_add(stream_id.wrapping_mul(0xD134_2543_DE82_EF95));
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^= z >> 31;
        let gamma = (stream_id.wrapping_mul(0xDA94_2042_E4DD_58B5) ^ 0xA076_1D64_78BD_642F) | 1;
        let child_seed = z.wrapping_add(gamma);
        Self { state: child_seed }
    }

    /// Advances the parent generator and forks two independent child substreams `(left, right)`.
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

    /// Returns a uniform sample in `[low, high)`.
    #[inline]
    pub fn uniform(&mut self, low: f64, high: f64) -> f64 {
        low + (high - low) * self.next_f64()
    }

    /// Returns a standard normal sample $\mathcal{N}(0, 1)$ via the Box–Muller transform
    /// over the full IEEE-754 double-precision range ($u_1 \ge \text{\texttt{f64::MIN\_POSITIVE}}$, up to $\approx 37.6\sigma$).
    #[inline]
    pub fn standard_normal(&mut self) -> f64 {
        let u1 = self.next_f64().max(f64::MIN_POSITIVE);
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
        let xm = x_m.max(f64::MIN_POSITIVE);
        let a = alpha.max(1e-6);
        let one_minus_u = (1.0 - self.next_f64()).max(f64::MIN_POSITIVE);
        xm * one_minus_u.powf(-1.0 / a)
    }

    /// Returns an Exponential sample $\mathrm{Exp}(\lambda)$ with rate $\lambda > 0$.
    #[inline]
    pub fn exponential(&mut self, rate_lambda: f64) -> f64 {
        let rate = rate_lambda.max(1e-12);
        let u = self.next_f64().max(f64::MIN_POSITIVE);
        -u.ln() / rate
    }

    /// Returns the natural logarithm $\ln X$ of a standard Gamma draw $X \sim \mathrm{Gamma}(\alpha, 1)$,
    /// preventing IEEE-754 underflow when $\alpha \ll 1$ (e.g., $\alpha \le 0.04$).
    pub fn log_standard_gamma(&mut self, alpha: f64) -> f64 {
        let a = alpha.max(1e-6);
        if a < 1.0 {
            let u = self.next_f64().max(f64::MIN_POSITIVE);
            return self.log_standard_gamma(a + 1.0) + u.ln() / a;
        }
        let d = a - 1.0 / 3.0;
        let c = 1.0 / (9.0 * d).sqrt();
        loop {
            let x = self.standard_normal();
            let v_root = 1.0 + c * x;
            if v_root <= 0.0 {
                continue;
            }
            let v = v_root * v_root * v_root;
            let u = self.next_f64().max(f64::MIN_POSITIVE);
            let x2 = x * x;
            if u < 1.0 - 0.0331 * (x2 * x2) || u.ln() < 0.5 * x2 + d * (1.0 - v + v.ln()) {
                return (d * v).max(f64::MIN_POSITIVE).ln();
            }
        }
    }

    /// Returns a Gamma sample $\mathrm{Gamma}(\alpha, \theta)$ with shape $\alpha > 0$ and scale $\theta > 0$
    /// using the Marsaglia–Tsang (2000) squeeze acceptance-rejection algorithm with log-space boost for $\alpha < 1$.
    pub fn gamma(&mut self, alpha: f64, scale_theta: f64) -> f64 {
        let a = alpha.max(1e-6);
        let theta = scale_theta.max(f64::MIN_POSITIVE);
        (theta.ln() + self.log_standard_gamma(a))
            .exp()
            .max(f64::MIN_POSITIVE)
    }

    /// Returns a Beta sample $\mathrm{Beta}(\alpha, \beta)$ for $\alpha > 0, \beta > 0$ via log-space Gamma draws,
    /// avoiding underflow collapse even for extreme shapes $\alpha, \beta \ll 0.04$.
    pub fn beta(&mut self, alpha: f64, beta: f64) -> f64 {
        let ln_x = self.log_standard_gamma(alpha.max(1e-6));
        let ln_y = self.log_standard_gamma(beta.max(1e-6));
        let ratio = 1.0 / (1.0 + (ln_y - ln_x).exp());
        ratio.clamp(1e-15, 1.0 - 1e-15)
    }

    /// Returns a Student-$t$ sample $T \sim t_\nu$ with degrees of freedom $\nu > 0$.
    pub fn student_t(&mut self, nu: f64) -> f64 {
        let dof = nu.max(1e-6);
        let z = self.standard_normal();
        let w = self.gamma(0.5 * dof, 2.0);
        z / (w / dof).max(1e-15).sqrt()
    }

    /// Exact natural logarithm of factorial $\ln(k!) = \ln\Gamma(k + 1)$.
    #[inline]
    fn ln_factorial(k: u64) -> f64 {
        const TABLE: [f64; 10] = [
            0.0,
            0.0,
            std::f64::consts::LN_2,
            1.791_759_469_228_055,
            3.178_053_830_347_945_8,
            4.787_491_742_782_046,
            6.579_251_212_010_101,
            8.525_161_361_065_415,
            10.604_602_902_745_25,
            12.801_827_480_081_469,
        ];
        if (k as usize) < TABLE.len() {
            return TABLE[k as usize];
        }
        let x = (k + 1) as f64;
        let inv_x = 1.0 / x;
        let inv_x2 = inv_x * inv_x;
        (x - 0.5) * x.ln() - x
            + 0.918_938_533_204_672_7
            + inv_x
                * (1.0 / 12.0 - inv_x2 * (1.0 / 360.0 - inv_x2 * (1.0 / 1260.0 - inv_x2 / 1680.0)))
    }

    /// Returns an exact Poisson count $N \sim \mathrm{Poisson}(\lambda)$ for $\lambda \ge 0$:
    /// uses Knuth's multiplicative inversion for $\lambda < 30$ and Wolfgang Hörmann's (1993)
    /// PTRS transformed rejection algorithm for $\lambda \ge 30$.
    pub fn poisson(&mut self, lambda: f64) -> u64 {
        if !lambda.is_finite() || lambda <= 0.0 {
            return 0;
        }
        if lambda < 30.0 {
            let l = (-lambda).exp();
            let mut k = 0u64;
            let mut p = 1.0;
            loop {
                k += 1;
                p *= self.next_f64().max(f64::MIN_POSITIVE);
                if p <= l {
                    return k - 1;
                }
            }
        } else {
            let s = lambda.sqrt();
            let b = 0.931 + 2.53 * s;
            let a = -0.059 + 0.02483 * b;
            let inv_alpha = 1.1239 + 1.1328 / (b - 3.4);
            let v_r = 0.9277 - 3.6224 / (b - 2.0);
            let ln_lam = lambda.ln();
            loop {
                let u = self.next_f64() - 0.5;
                let v = self.next_f64().max(f64::MIN_POSITIVE);
                let u_s = 0.5 - u.abs();
                let k_f = ((2.0 * a / u_s + b) * u + lambda + 0.43).floor();
                if k_f < 0.0 {
                    continue;
                }
                let k = k_f as u64;
                if u_s >= 0.07 && v <= v_r {
                    return k;
                }
                if u_s < 0.013 && v > u_s {
                    continue;
                }
                let lhs = (v * inv_alpha / (a / (u_s * u_s) + b)).ln();
                let rhs = -lambda + (k as f64) * ln_lam - Self::ln_factorial(k);
                if lhs <= rhs {
                    return k;
                }
            }
        }
    }

    /// In-place Fisher–Yates shuffle using unbiased rejection sampling (zero modulo bias).
    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        let n = slice.len();
        for i in (1..n).rev() {
            let bound = (i + 1) as u64;
            let thresh = u64::MAX - (u64::MAX % bound);
            let r = loop {
                let x = self.next_u64();
                if x < thresh {
                    break x;
                }
            };
            let j = (r % bound) as usize;
            slice.swap(i, j);
        }
    }
}

/// Welford–Terriberry single-pass numerically stable online moments (mean, variance, skewness, excess kurtosis).
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
}

/// Arithmetic mean of a slice (returns `0.0` if `xs` is empty).
pub fn mean(xs: &[f64]) -> f64 {
    if xs.is_empty() {
        return 0.0;
    }
    xs.iter().sum::<f64>() / (xs.len() as f64)
}

/// Unbiased sample variance ($n - 1$ denominator) of a slice (returns `0.0` if `xs.len() < 2`).
pub fn sample_variance(xs: &[f64]) -> f64 {
    if xs.len() < 2 {
        return 0.0;
    }
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

/// Empirical quantile $q \in [0, 1]$ with linear interpolation (returns `0.0` if `xs` is empty).
pub fn quantile(xs: &[f64], q: f64) -> f64 {
    if xs.is_empty() {
        return 0.0;
    }
    let mut sorted = xs.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    quantile_sorted(&sorted, q)
}

/// Empirical quantile $q \in [0, 1]$ on an already-sorted slice (returns `0.0` if `sorted` is empty).
pub fn quantile_sorted(sorted: &[f64], q: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let q_clamped = q.clamp(0.0, 1.0);
    if sorted.len() == 1 {
        return sorted[0];
    }
    let max_idx = sorted.len() - 1;
    let pos = (q_clamped * (max_idx as f64)).clamp(0.0, max_idx as f64);
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

// ============================================================================
// FIPS 180-4 SHA-256 & RFC 2104 HMAC-SHA256
// ============================================================================

const SHA256_K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

const SHA256_H0: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

/// Computes the 32-byte SHA-256 digest of `data`.
pub fn sha256_bytes(data: &[u8]) -> [u8; 32] {
    let mut h = SHA256_H0;
    let bit_len = (data.len() as u64) * 8;

    let mut padded = Vec::with_capacity((data.len() + 9).div_ceil(64) * 64);
    padded.extend_from_slice(data);
    padded.push(0x80);
    while (padded.len() + 8) % 64 != 0 {
        padded.push(0x00);
    }
    padded.extend_from_slice(&bit_len.to_be_bytes());

    for chunk in padded.as_chunks::<64>().0 {
        let mut w = [0u32; 64];
        for (i, &word) in chunk.as_chunks::<4>().0.iter().enumerate() {
            w[i] = u32::from_be_bytes(word);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }

        let mut a = h[0];
        let mut b = h[1];
        let mut c = h[2];
        let mut d = h[3];
        let mut e = h[4];
        let mut f = h[5];
        let mut g = h[6];
        let mut hh = h[7];

        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(SHA256_K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }

    let mut out = [0u8; 32];
    for (i, word) in h.iter().enumerate() {
        out[i * 4..(i + 1) * 4].copy_from_slice(&word.to_be_bytes());
    }
    out
}

/// Computes the 64-character lowercase hexadecimal SHA-256 digest of `data`.
pub fn sha256_hex(data: &[u8]) -> String {
    let bytes = sha256_bytes(data);
    let mut s = String::with_capacity(64);
    for b in bytes {
        use std::fmt::Write;
        let _ = write!(&mut s, "{b:02x}");
    }
    s
}

/// Validates whether `s` is a valid 64-character lowercase hexadecimal SHA-256 string.
pub fn is_valid_sha256_hex(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

/// Computes the 64-character lowercase hexadecimal RFC 2104 HMAC-SHA256 tag of `message` under `key`.
pub fn hmac_sha256_hex(key: &[u8], message: &[u8]) -> String {
    let mut k_block = [0u8; 64];
    if key.len() > 64 {
        let hashed_key = sha256_bytes(key);
        k_block[..32].copy_from_slice(&hashed_key);
    } else {
        k_block[..key.len()].copy_from_slice(key);
    }

    let mut ipad = [0x36u8; 64];
    let mut opad = [0x5cu8; 64];
    for i in 0..64 {
        ipad[i] ^= k_block[i];
        opad[i] ^= k_block[i];
    }

    let mut inner_input = Vec::with_capacity(64 + message.len());
    inner_input.extend_from_slice(&ipad);
    inner_input.extend_from_slice(message);
    let inner_hash = sha256_bytes(&inner_input);

    let mut outer_input = [0u8; 96];
    outer_input[..64].copy_from_slice(&opad);
    outer_input[64..].copy_from_slice(&inner_hash);
    let bytes = sha256_bytes(&outer_input);
    let mut s = String::with_capacity(64);
    for b in bytes {
        use std::fmt::Write;
        let _ = write!(&mut s, "{b:02x}");
    }
    s
}

// ============================================================================
// Mathematical & Statistical Kernel Primitives
// ============================================================================

/// Clamps a probability $p$ into the open-interior Cromwell interval $[\varepsilon, 1 - \varepsilon]$
/// (Lindley, 1985), preventing dogmatic $0\%$ or $100\%$ probabilities that paralyze Bayesian updating.
#[inline]
pub fn cromwell_clamp(p: f64, epsilon_floor: f64) -> f64 {
    let eps = epsilon_floor.clamp(1e-12, 0.49);
    if p.is_nan() {
        0.5
    } else {
        p.clamp(eps, 1.0 - eps)
    }
}

/// Checks whether a probability slice strictly obeys Cromwell's Rule for `epsilon_floor`.
pub fn obeys_cromwell_rule(probs: &[f64], epsilon_floor: f64) -> bool {
    let eps = epsilon_floor.clamp(1e-12, 0.49);
    !probs.is_empty()
        && probs
            .iter()
            .all(|&p| p.is_finite() && p >= eps && p <= 1.0 - eps)
}

/// Standard normal cumulative distribution function $\Phi(z)$ via Abramowitz & Stegun 7.1.26
/// rational approximation (maximum absolute error $< 1.5 \times 10^{-7}$).
pub fn standard_normal_cdf(z: f64) -> f64 {
    let sign = if z < 0.0 { -1.0 } else { 1.0 };
    let x = z.abs() / std::f64::consts::SQRT_2;
    let t = 1.0 / (1.0 + 0.3275911 * x);
    let poly = t
        * (0.254829592
            + t * (-0.284496736 + t * (1.421413741 + t * (-1.453152027 + t * 1.061405429))));
    let erf = 1.0 - poly * (-x * x).exp();
    0.5 * (1.0 + sign * erf)
}

/// Rational approximation to the inverse standard normal CDF $\Phi^{-1}(p)$ for $p \in (0, 1)$
/// (Peter J. Acklam's algorithm; maximum absolute error $\approx 1.15 \times 10^{-9}$,
/// strictly monotone on $(0, 1)$ and continuous at both region boundaries).
#[allow(clippy::excessive_precision)]
pub fn inverse_standard_normal_cdf(p: f64) -> f64 {
    // Central-region coefficients (Acklam 2004 / Wichura AS 241).
    const A: [f64; 6] = [
        -3.969683028665376e+01,
        2.209460984245205e+02,
        -2.759285104469687e+02,
        1.383577518672690e+02,
        -3.066479806614716e+01,
        2.506628277459239e+00,
    ];
    const B: [f64; 5] = [
        -5.447609879822406e+01,
        1.615858368580409e+02,
        -1.556989798598866e+02,
        6.680131188771972e+01,
        -1.328068155288572e+01,
    ];
    // Tail-region coefficients (Acklam 2004).
    const C: [f64; 6] = [
        -7.784894002430293e-03,
        -3.223964580411365e-01,
        -2.400758277161838e+00,
        -2.549732539343734e+00,
        4.374664141464968e+00,
        2.938163982698783e+00,
    ];
    const D: [f64; 4] = [
        7.784695709041462e-03,
        3.224671290700398e-01,
        2.445134137142996e+00,
        3.754408661907416e+00,
    ];
    const P_LOW: f64 = 0.02425;

    let p = p.clamp(1e-12, 1.0 - 1e-12);
    if p < P_LOW {
        let q = (-2.0 * p.ln()).sqrt();
        (((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
    } else if p <= 1.0 - P_LOW {
        let q = p - 0.5;
        let r = q * q;
        (((((A[0] * r + A[1]) * r + A[2]) * r + A[3]) * r + A[4]) * r + A[5]) * q
            / (((((B[0] * r + B[1]) * r + B[2]) * r + B[3]) * r + B[4]) * r + 1.0)
    } else {
        let q = (-2.0 * (1.0 - p).ln()).sqrt();
        -(((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
    }
}

/// Pool-Adjacent-Violators (PAV) algorithm for monotonically non-decreasing isotonic regression.
pub fn pav_isotonic_non_decreasing(y: &[f64]) -> Vec<f64> {
    if y.is_empty() {
        return Vec::new();
    }
    let mut blocks: Vec<(f64, usize)> = Vec::with_capacity(y.len());
    for &val in y {
        blocks.push((val, 1));
        while blocks.len() >= 2 {
            let n = blocks.len();
            let (s_prev, c_prev) = blocks[n - 2];
            let (s_curr, c_curr) = blocks[n - 1];
            if s_prev / (c_prev as f64) > s_curr / (c_curr as f64) {
                blocks.pop();
                blocks[n - 2] = (s_prev + s_curr, c_prev + c_curr);
            } else {
                break;
            }
        }
    }
    let mut out = Vec::with_capacity(y.len());
    for (sum_y, count) in blocks {
        let avg = sum_y / (count as f64);
        for _ in 0..count {
            out.push(avg);
        }
    }
    out
}

/// Solves a symmetric positive-definite linear system $A x = b$ ($d \times d$) via Cholesky decomposition.
#[allow(clippy::needless_range_loop)]
pub fn solve_cholesky_spd(a: &[Vec<f64>], b: &[f64]) -> Option<Vec<f64>> {
    let d = b.len();
    if a.len() != d || a.iter().any(|row| row.len() != d) {
        return None;
    }
    let max_abs = a
        .iter()
        .flatten()
        .fold(0.0_f64, |m, &v| m.max(v.abs()))
        .max(1.0);
    let sym_tol = 1e-10 * max_abs;
    for i in 0..d {
        for j in (i + 1)..d {
            if (a[i][j] - a[j][i]).abs() > sym_tol {
                return None;
            }
        }
    }
    let pivot_tol = 1e-14 * max_abs;
    let mut l = vec![vec![0.0; d]; d];
    for i in 0..d {
        for j in 0..=i {
            let mut sum = a[i][j];
            for k in 0..j {
                sum -= l[i][k] * l[j][k];
            }
            if i == j {
                if sum <= pivot_tol {
                    return None;
                }
                l[i][j] = sum.sqrt();
            } else {
                l[i][j] = sum / l[j][j];
            }
        }
    }
    let mut z = vec![0.0; d];
    for i in 0..d {
        let mut sum = b[i];
        for k in 0..i {
            sum -= l[i][k] * z[k];
        }
        z[i] = sum / l[i][i];
    }
    let mut x = vec![0.0; d];
    for i in (0..d).rev() {
        let mut sum = z[i];
        for k in (i + 1)..d {
            sum -= l[k][i] * x[k];
        }
        x[i] = sum / l[i][i];
    }
    Some(x)
}

/// Ordinary / ridge least-squares estimator $\hat{\beta} = (X^\top X + \lambda I)^{-1} X^\top y$.
/// Returns `None` if `ridge_lambda < 0.0` or non-finite, or if dimensions are invalid.
#[allow(clippy::needless_range_loop)]
pub fn ols_ridge_fit(x: &[Vec<f64>], y: &[f64], ridge_lambda: f64) -> Option<Vec<f64>> {
    if !ridge_lambda.is_finite() || ridge_lambda < 0.0 {
        return None;
    }
    let n = x.len();
    if n == 0 || n != y.len() {
        return None;
    }
    let d = x[0].len();
    if d == 0 {
        return None;
    }
    let mut xtx = vec![vec![0.0; d]; d];
    let mut xty = vec![0.0; d];
    for (row, &yi) in x.iter().zip(y.iter()) {
        if row.len() != d {
            return None;
        }
        for j in 0..d {
            xty[j] += row[j] * yi;
            for k in 0..=j {
                xtx[j][k] += row[j] * row[k];
            }
        }
    }
    let reg = ridge_lambda.max(1e-10);
    for j in 0..d {
        xtx[j][j] += reg;
        for k in (j + 1)..d {
            xtx[j][k] = xtx[k][j];
        }
    }
    solve_cholesky_spd(&xtx, &xty)
}

/// Computes average-tie ranks of `xs` (1-based) and returns Spearman's rank correlation $\rho_s \in [-1, 1]$.
pub fn spearman_rank_correlation(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len().min(y.len());
    if n < 2 {
        return 0.0;
    }
    let ranks = |vals: &[f64]| -> Vec<f64> {
        let mut idx: Vec<usize> = (0..n).collect();
        idx.sort_by(|&i, &j| vals[i].total_cmp(&vals[j]));
        let mut r = vec![0.0; n];
        let mut i = 0;
        while i < n {
            let mut j = i + 1;
            while j < n && (vals[idx[j]] - vals[idx[i]]).abs() <= 1e-12 {
                j += 1;
            }
            let avg_rank = 0.5 * ((i + 1 + j) as f64);
            for k in i..j {
                r[idx[k]] = avg_rank;
            }
            i = j;
        }
        r
    };
    let rx = ranks(x);
    let ry = ranks(y);
    let mean_r = 0.5 * ((n + 1) as f64);
    let mut num = 0.0;
    let mut dx2 = 0.0;
    let mut dy2 = 0.0;
    for (&a, &b) in rx.iter().zip(ry.iter()) {
        let da = a - mean_r;
        let db = b - mean_r;
        num += da * db;
        dx2 += da * da;
        dy2 += db * db;
    }
    let denom = (dx2 * dy2).sqrt();
    if denom <= 1e-15 { 0.0 } else { num / denom }
}

/// Two-sample Kolmogorov–Smirnov supremum distance $D_{\mathrm{KS}} = \sup_t |\hat{F}_1(t) - \hat{F}_2(t)|$.
pub fn kolmogorov_smirnov_distance(a: &[f64], b: &[f64]) -> f64 {
    if a.is_empty() || b.is_empty() {
        return 0.0;
    }
    let mut sa = a.to_vec();
    let mut sb = b.to_vec();
    sa.sort_by(|x, y| x.total_cmp(y));
    sb.sort_by(|x, y| x.total_cmp(y));
    let na = sa.len() as f64;
    let nb = sb.len() as f64;
    let mut ia = 0usize;
    let mut ib = 0usize;
    let mut max_d = 0.0_f64;
    while ia < sa.len() && ib < sb.len() {
        let v = sa[ia].min(sb[ib]);
        while ia < sa.len() && sa[ia] <= v {
            ia += 1;
        }
        while ib < sb.len() && sb[ib] <= v {
            ib += 1;
        }
        let d = ((ia as f64) / na - (ib as f64) / nb).abs();
        if d > max_d {
            max_d = d;
        }
    }
    max_d
}

/// Parses a quarterly vintage string such as `"1965:Q4"` or `"1965Q4"` into a linear quarter index `4 * year + (q - 1)`.
pub fn parse_quarter(s: &str) -> Option<u32> {
    let clean = s.trim();
    let (yr_str, q_str) = if let Some(pos) = clean.find(":Q") {
        (&clean[..pos], &clean[pos + 2..])
    } else {
        let pos = clean.find('Q')?;
        (&clean[..pos], &clean[pos + 1..])
    };
    let year: u32 = yr_str.parse().ok()?;
    let q: u32 = q_str.parse().ok()?;
    if (1..=4).contains(&q) {
        Some(4 * year + (q - 1))
    } else {
        None
    }
}

// ---------------------------------------------------------------------------------------------
// Elementary cellular automata: the phenomenology of a fixed local rule
// ---------------------------------------------------------------------------------------------

/// One of the four elementary cellular-automaton rules exercised by
/// [`CellularAutomatonAudit`].
///
/// Each rule is an 8-bit lookup over the three-cell neighbourhood
/// $(x_{i-1}, x_i, x_{i+1})$, indexed by $4x_{i-1} + 2x_i + x_{i+1}$.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementaryRule {
    /// Rule 0 — every neighbourhood maps to the quiescent state (class 1).
    Zero,
    /// Rule 90 — the new cell is the exclusive disjunction of its two neighbours (class 2/3 boundary).
    Ninety,
    /// Rule 30 — the canonical chaotic rule of Wolfram's study (class 3).
    Thirty,
    /// Rule 110 — the rule proved Turing-complete by Cook (class 4).
    OneTen,
}

impl ElementaryRule {
    /// All four rules, in the order the audit reports them.
    pub const ALL: [Self; 4] = [Self::Zero, Self::Ninety, Self::Thirty, Self::OneTen];

    /// The 8-bit lookup table of the rule.
    pub const fn table(self) -> u8 {
        match self {
            Self::Zero => 0,
            Self::Ninety => 90,
            Self::Thirty => 30,
            Self::OneTen => 110,
        }
    }

    /// Short label used in reports.
    pub const fn code(self) -> &'static str {
        match self {
            Self::Zero => "R0",
            Self::Ninety => "R90",
            Self::Thirty => "R30",
            Self::OneTen => "R110",
        }
    }
}

/// Regime of an elementary rule, assigned from measured behaviour only.
///
/// The four names describe *what the audit observed*, not Wolfram's class numbers: the audit reports
/// the observables (fixed point, temporal period, block entropy, density) and lets the reader match
/// them to the published classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutomatonRegime {
    /// Reaches a fixed point inside the horizon.
    Settling,
    /// Enters a cycle of bounded temporal period.
    Periodic,
    /// Neither fixed nor periodic, with high block entropy.
    Dispersed,
    /// Neither fixed nor periodic, with low block entropy: a sparse, persistent structure.
    Structured,
}

/// Outcome of running one elementary rule on a finite ring.
#[derive(Debug, Clone, PartialEq)]
pub struct CellularAutomatonAudit {
    /// Rule that was run.
    pub rule: ElementaryRule,
    /// Number of cells in the ring.
    pub cells: usize,
    /// Number of time steps simulated.
    pub steps: usize,
    /// First step whose configuration repeats the previous one (`None` if never).
    pub fixed_point_step: Option<usize>,
    /// Smallest temporal period found in the final configuration (`None` if none within `max_period`).
    pub tail_period: Option<usize>,
    /// Block entropy of the final configuration over 8-cell words, in bits (maximum 8).
    pub block_entropy_final: f64,
    /// Largest block entropy reached over the horizon, in bits.
    pub block_entropy_max: f64,
    /// Fraction of live cells in the final configuration.
    pub live_fraction_final: f64,
    /// Regime assigned from the measured behaviour.
    pub regime: AutomatonRegime,
}

impl CellularAutomatonAudit {
    /// Cells in the ring used by the canonical audit.
    pub const CELLS: usize = 201;
    /// Steps simulated by the canonical audit.
    pub const STEPS: usize = 200;
    /// Largest temporal period the audit will look for.
    pub const MAX_PERIOD: usize = 64;

    fn step(state: &[bool], table: u8) -> Vec<bool> {
        let n = state.len();
        (0..n)
            .map(|i| {
                let left = state[(i + n - 1) % n];
                let centre = state[i];
                let right = state[(i + 1) % n];
                let idx = ((left as u8) << 2) | ((centre as u8) << 1) | (right as u8);
                (table >> idx) & 1 == 1
            })
            .collect()
    }

    /// Shannon entropy, in bits, of the distribution of overlapping 8-cell words on the ring.
    fn block_entropy(state: &[bool]) -> f64 {
        const WORD: usize = 8;
        let n = state.len();
        let mut counts = [0usize; 1 << WORD];
        for i in 0..n {
            let mut word = 0usize;
            for k in 0..WORD {
                if state[(i + k) % n] {
                    word |= 1 << k;
                }
            }
            counts[word] += 1;
        }
        let total = n as f64;
        let entropy: f64 = counts
            .iter()
            .filter(|&&c| c > 0)
            .map(|&c| {
                let p = c as f64 / total;
                -p * p.log2()
            })
            .sum();
        if entropy == 0.0 { 0.0 } else { entropy }
    }

    /// Runs one rule and measures its behaviour. Deterministic: one live cell at the centre, no noise.
    pub fn run(rule: ElementaryRule) -> Self {
        let table = rule.table();
        let n = Self::CELLS;
        let mut state = vec![false; n];
        state[n / 2] = true;
        let mut history = Vec::with_capacity(Self::STEPS + 1);
        history.push(state.clone());
        let mut fixed_point_step = None;
        let mut block_entropy_max: f64 = Self::block_entropy(&state);

        for t in 1..=Self::STEPS {
            let next = Self::step(&state, table);
            if next == state && fixed_point_step.is_none() {
                fixed_point_step = Some(t);
            }
            block_entropy_max = block_entropy_max.max(Self::block_entropy(&next));
            state = next;
            history.push(state.clone());
        }

        let mut tail_period = None;
        for p in 1..=Self::MAX_PERIOD {
            if history.len() > p && history[history.len() - 1] == history[history.len() - 1 - p] {
                tail_period = Some(p);
                break;
            }
        }

        let final_state = history.last().expect("history is never empty").clone();
        let block_entropy_final = Self::block_entropy(&final_state);
        let live_fraction_final =
            final_state.iter().filter(|&&b| b).count() as f64 / final_state.len() as f64;
        let regime = if fixed_point_step.is_some() {
            AutomatonRegime::Settling
        } else if tail_period.is_some() {
            AutomatonRegime::Periodic
        } else if block_entropy_final >= 3.0 {
            AutomatonRegime::Dispersed
        } else {
            AutomatonRegime::Structured
        };

        Self {
            rule,
            cells: n,
            steps: Self::STEPS,
            fixed_point_step,
            tail_period,
            block_entropy_final,
            block_entropy_max,
            live_fraction_final,
            regime,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splitmix64_sha256_and_math_primitives_hold() {
        let mut r1 = Rng::with_default_seed();
        let mut r2 = Rng::new(DEFAULT_SEED);
        assert_eq!(r1.next_u64(), 0xdb9c_5598_9194_8d23);
        assert_eq!(r2.next_u64(), 0xdb9c_5598_9194_8d23);

        let (mut l, mut r) = r1.split_fork_join_pair(7);
        assert_ne!(l.next_u64(), r.next_u64());
        assert!((2.0..5.0).contains(&r1.uniform(2.0, 5.0)));
        assert!(r1.exponential(2.0) > 0.0);
        assert!(r1.gamma(2.5, 1.2) > 0.0);
        assert!(r1.gamma(0.5, 1.0) > 0.0);
        assert!(r1.gamma(0.01, 1.0) > 0.0);
        assert!((0.0..1.0).contains(&r1.beta(2.0, 3.0)));
        assert!((0.0..1.0).contains(&r1.beta(0.01, 0.02)));
        assert!(r1.student_t(5.0).is_finite());
        assert!(r1.poisson(4.0) < 100);
        assert!(r1.poisson(50.0) > 0);
        let pois30: Vec<f64> = (0..4000).map(|_| r1.poisson(30.0) as f64).collect();
        assert!((mean(&pois30) - 30.0).abs() < 0.8);
        assert!((sample_variance(&pois30) - 30.0).abs() < 2.5);
        let mut arr = [1, 2, 3, 4, 5];
        r1.shuffle(&mut arr);

        let xs = [1.0, 2.0, 3.0, 4.0, 100.0];
        assert_eq!(median(&xs), 3.0);
        assert_eq!(median_absolute_deviation(&xs), 1.0);
        let m = OnlineMoments::from_slice(&xs);
        assert!((m.sample_variance() - sample_variance(&xs)).abs() < 1e-12);
        assert!((m.sample_std() - sample_std(&xs)).abs() < 1e-12);
        assert!(m.skewness() > 0.0);
        assert!(m.excess_kurtosis() > -2.0);

        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        assert!(is_valid_sha256_hex(&sha256_hex(b"abc")));
        assert_eq!(
            hmac_sha256_hex(b"Jefe", b"what do ya want for nothing?"),
            "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843"
        );

        let y = [0.8, 0.2, 0.5, 0.4, 0.9];
        let iso = pav_isotonic_non_decreasing(&y);
        for w in iso.windows(2) {
            assert!(w[0] <= w[1] + 1e-12);
        }
        let x = vec![
            vec![1.0, 1.0],
            vec![1.0, 2.0],
            vec![1.0, 3.0],
            vec![1.0, 4.0],
        ];
        let y_lin = vec![5.0, 7.0, 9.0, 11.0];
        let beta = ols_ridge_fit(&x, &y_lin, 1e-12).expect("OLS fit");
        assert!((beta[0] - 3.0).abs() < 1e-7 && (beta[1] - 2.0).abs() < 1e-7);
        assert!((standard_normal_cdf(inverse_standard_normal_cdf(0.90)) - 0.90).abs() < 1e-6);
        // Exact tail values of Phi^{-1} (regression pins for the previously broken tail branch).
        assert!((inverse_standard_normal_cdf(0.95) - 1.644_853_627).abs() < 1e-6);
        assert!((inverse_standard_normal_cdf(0.975) - 1.959_963_985).abs() < 1e-6);
        assert!((inverse_standard_normal_cdf(0.99) - 2.326_347_874).abs() < 1e-6);
        assert!((inverse_standard_normal_cdf(0.999) - 3.090_232_306).abs() < 1e-6);
        assert!(inverse_standard_normal_cdf(0.999) > inverse_standard_normal_cdf(0.99));
        assert!(inverse_standard_normal_cdf(0.999) > inverse_standard_normal_cdf(0.975));
        assert!((inverse_standard_normal_cdf(0.05) + 1.644_853_627).abs() < 1e-6);
        assert!(
            (spearman_rank_correlation(&[1.0, 2.0, 3.0], &[10.0, 20.0, 30.0]) - 1.0).abs() < 1e-12
        );
        assert!(kolmogorov_smirnov_distance(&[1.0, 2.0], &[10.0, 20.0]) > 0.9);
        assert!(!obeys_cromwell_rule(&[0.0, 0.5], 1e-4));
        assert!(obeys_cromwell_rule(&[cromwell_clamp(0.0, 1e-4)], 1e-4));
        assert_eq!(parse_quarter("1965:Q4"), Some(1965 * 4 + 3));
    }
}
