//! Pure-Rust double-precision complex arithmetic (`Complex64`, $\mathbb{C}$) with full `std::ops` traits.
//!
//! Used across wave-interference physics, Gabor `WIRE` dictionaries, discrete 2D Helmholtz PDEs,
//! $\mathbb{C}^{n\times n} \hookrightarrow \mathbb{R}^{2n\times 2n}$ matrix isomorphisms, and
//! Holographic Reduced Representations (`FHRR` / `GC-VSA`).

use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

/// Double-precision complex number $z = \text{re} + i\,\text{im} \in \mathbb{C}$.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Complex64 {
    /// Real part $\Re(z)$.
    pub re: f64,
    /// Imaginary part $\Im(z)$.
    pub im: f64,
}

impl Complex64 {
    /// Additive identity $0 + 0i$.
    pub const ZERO: Self = Self { re: 0.0, im: 0.0 };

    /// Multiplicative identity $1 + 0i$.
    pub const ONE: Self = Self { re: 1.0, im: 0.0 };

    /// Imaginary unit $0 + 1i$.
    pub const I: Self = Self { re: 0.0, im: 1.0 };

    /// Constructs $z = \text{re} + i\,\text{im}$.
    #[inline]
    pub const fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    /// Euler unit phasor $e^{i\theta} = \cos\theta + i\sin\theta$.
    #[inline]
    pub fn exp_i(theta: f64) -> Self {
        Self {
            re: theta.cos(),
            im: theta.sin(),
        }
    }

    /// Complex conjugate $\bar{z} = \text{re} - i\,\text{im}$.
    #[inline]
    pub const fn conj(self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    /// Hermitian inner-product term $\bar{self} \cdot \text{other}$.
    #[inline]
    pub fn conj_mul(self, other: Self) -> Self {
        Self {
            re: self.re * other.re + self.im * other.im,
            im: self.re * other.im - self.im * other.re,
        }
    }

    /// Squared modulus $|z|^2 = \text{re}^2 + \text{im}^2$.
    #[inline]
    pub fn norm_sq(self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    /// Modulus $|z| = \sqrt{\text{re}^2 + \text{im}^2}$.
    #[inline]
    pub fn abs(self) -> f64 {
        self.re.hypot(self.im)
    }

    /// Principal argument $\arg(z) \in (-\pi, \pi]$.
    #[inline]
    pub fn arg(self) -> f64 {
        self.im.atan2(self.re)
    }

    /// Converts to a `(re, im)` tuple.
    #[inline]
    pub const fn to_tuple(self) -> (f64, f64) {
        (self.re, self.im)
    }
}

impl From<(f64, f64)> for Complex64 {
    #[inline]
    fn from((re, im): (f64, f64)) -> Self {
        Self { re, im }
    }
}

impl From<Complex64> for (f64, f64) {
    #[inline]
    fn from(z: Complex64) -> Self {
        (z.re, z.im)
    }
}

impl From<f64> for Complex64 {
    #[inline]
    fn from(re: f64) -> Self {
        Self { re, im: 0.0 }
    }
}

impl Add for Complex64 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl AddAssign for Complex64 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl Sub for Complex64 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl SubAssign for Complex64 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.re -= rhs.re;
        self.im -= rhs.im;
    }
}

impl Mul for Complex64 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl MulAssign for Complex64 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Mul<f64> for Complex64 {
    type Output = Self;

    #[inline]
    fn mul(self, scalar: f64) -> Self::Output {
        Self {
            re: self.re * scalar,
            im: self.im * scalar,
        }
    }
}

impl Mul<Complex64> for f64 {
    type Output = Complex64;

    #[inline]
    fn mul(self, z: Complex64) -> Self::Output {
        Complex64 {
            re: self * z.re,
            im: self * z.im,
        }
    }
}

impl Div for Complex64 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        let denom = rhs.norm_sq().max(1e-30);
        Self {
            re: (self.re * rhs.re + self.im * rhs.im) / denom,
            im: (self.im * rhs.re - self.re * rhs.im) / denom,
        }
    }
}

impl Div<f64> for Complex64 {
    type Output = Self;

    #[inline]
    fn div(self, scalar: f64) -> Self::Output {
        Self {
            re: self.re / scalar,
            im: self.im / scalar,
        }
    }
}

impl Neg for Complex64 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl Sum for Complex64 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |acc, z| acc + z)
    }
}

impl<'a> Sum<&'a Complex64> for Complex64 {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |acc, &z| acc + z)
    }
}
