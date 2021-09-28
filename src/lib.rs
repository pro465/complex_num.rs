use core::ops::{Add, Div, Mul, Rem, Sub};

pub struct Complex<T> {
    real: T,
    imag: T,
}

pub trait Sqrt {
    fn sqrt(&self) -> Self;
}

impl Sqrt for f32 {
    fn sqrt(&self) -> Self {
        f32::sqrt(*self)
    }
}

impl Sqrt for f64 {
    fn sqrt(&self) -> Self {
        f64::sqrt(*self)
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

impl<T: Eq> PartialEq for Complex<T> {
    fn eq(&self, other: &Self) -> bool {
        self.real == other.real && self.imag == other.imag
    }
}

impl<T: Eq> Eq for Complex<T> {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
