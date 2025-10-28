// Simple complex number data type
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self { Self { re, im } }
    pub fn zero() -> Self { Self { re: 0.0, im: 0.0 } }
    pub fn one() -> Self { Self { re: 1.0, im: 0.0 } }
    pub fn conj(&self) -> Self { Self { re: self.re, im: -self.im } }
    pub fn magnitude2(&self) -> f64 { self.re * self.re + self.im * self.im }
    pub fn add(&self, other: &Self) -> Self { Self::new(self.re + other.re, self.im + other.im) }
    pub fn sub(&self, other: &Self) -> Self { Self::new(self.re-other.re, self.im-other.im)}
    pub fn div(&self, other: &Self) -> Self {
    let denominator = other.re * other.re + other.im * other.im;
    Self::new(
        (self.re * other.re + self.im * other.im) / denominator,
        (self.im * other.re - self.re * other.im) / denominator
    )
}
    pub fn mul(&self, other: &Self) -> Self {
        Self::new(self.re * other.re - self.im * other.im,
                  self.re * other.im + self.im * other.re)
    }
}
