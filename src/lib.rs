#![no_std]

use core::fmt::{Debug, Display, Error, Formatter};
use core::ops::{Add, Div, Mul, Rem, Sub};

mod sqrt;
mod utils;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Complex<T> {
    real: T,
    imag: T,
}

pub trait Sqrt {
    fn sqrt(&self) -> Self;
}

impl Sqrt for f32 {
    fn sqrt(&self) -> Self {
        sqrt::sqrt_f32(*self)
    }
}

impl Sqrt for f64 {
    fn sqrt(&self) -> Self {
        sqrt::sqrt_f64(*self)
    }
}

impl<T> Complex<T> {
    pub fn new(real: T, imag: T) -> Self {
        Self { real, imag }
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Sqrt + Clone> Complex<T> {
    pub fn abs(&self) -> T {
        (self.real.clone() * self.real.clone() + self.imag.clone() * self.imag.clone()).sqrt()
    }
}

impl<T: Add<Output = T>> Add for Complex<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Complex<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Clone> Mul for Complex<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            real: self.real.clone() * other.real.clone() - self.imag.clone() * other.imag.clone(),
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
}

impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Clone> Div
    for Complex<T>
{
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let denom =
            other.real.clone() * other.real.clone() + other.imag.clone() * other.imag.clone();

        Self {
            real: (self.real.clone() * other.real.clone() + self.imag.clone() * other.imag.clone())
                / denom.clone(),
            imag: (self.imag * other.real - self.real * other.imag) / denom,
        }
    }
}

impl<
        T: Rem<Output = T>
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Clone,
    > Rem for Complex<T>
{
    type Output = Self;
    fn rem(self, other: Self) -> Self {
        let denom =
            other.real.clone() * other.real.clone() + other.imag.clone() * other.imag.clone();

        Self {
            real: (self.real.clone() * other.real.clone() + self.imag.clone() * other.imag.clone())
                % denom.clone(),
            imag: (self.imag * other.real - self.real * other.imag) % denom,
        }
    }
}

impl<T: Default> Default for Complex<T> {
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}

impl<T: Default> From<T> for Complex<T> {
    fn from(num: T) -> Self {
        Self::new(num, Default::default())
    }
}

impl<T: Display> Display for Complex<T> {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        write!(formatter, "{} + {}*i", self.real, self.imag)
    }
}

impl<T: Display> Debug for Complex<T> {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        write!(formatter, "{} + {}*i", self.real, self.imag)
    }
}

#[cfg(test)]
mod tests {
    use crate::Complex;
    #[test]
    fn add() {
        let c1 = Complex::new(13, 123);
        let c2 = Complex::new(576, 362);

        assert_eq!(c1 + c2, Complex::new(13 + 576, 123 + 362));
    }

    #[test]
    fn sub() {
        let c1 = Complex::new(13, 123);
        let c2 = Complex::new(576, 362);

        assert_eq!(c1 - c2, Complex::new(13 - 576, 123 - 362));
    }

    #[test]
    fn mul() {
        let c1 = Complex::new(13, 123);
        let c2 = Complex::new(576, 362);

        assert_eq!(
            c1 * c2,
            Complex::new(13 * 576 - 123 * 362, 13 * 362 + 576 * 123)
        );
    }

    #[test]
    fn div() {
        let c1 = Complex::new(13_i64, 123);
        let c2 = Complex::new(576, 362);

        let num = c1 * Complex::new(576, -362);
        let denom = 576 * 576 + 362 * 362;
        let res = num / Complex::from(denom);

        assert_eq!(c1 / c2, res);
    }
}
