use crate::math::complex::Complex;

pub struct Gates;

impl Gates {
    pub fn x() -> [[Complex; 2]; 2] {
        [[Complex::zero(), Complex::one()],
         [Complex::one(), Complex::zero()]]
    }

    pub fn h() -> [[Complex; 2]; 2] {
        let s = 1.0 / (2.0f64).sqrt();
        [[Complex::new(s, 0.0), Complex::new(s, 0.0)],
         [Complex::new(s, 0.0), Complex::new(-s, 0.0)]]
    }

    pub fn z() -> [[Complex; 2]; 2] {
        [[Complex::one(), Complex::zero()],
         [Complex::zero(), Complex::new(-1.0, 0.0)]]
    }
    pub fn y() -> [[Complex; 2]; 2] {
        [[Complex::zero(), Complex::new(0.0,-1.0)],
         [Complex::new(0.0,1.0), Complex::zero()]]
    }
    pub fn s() -> [[Complex; 2]; 2] {
        [[Complex::one(), Complex::zero()],
         [Complex::zero(), Complex::new(0.0, 1.0)]]
    }
}
