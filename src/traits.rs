use std::ops::{Neg, Mul, Div};
use num_traits::{Num, Float};


/// Something that can be conjugated
pub trait Conj {
    fn conj(self) -> Self;
}

/// Square absolute value
pub trait SqrAbs {
    type Output: Float;
    fn sqr_abs(self) -> Self::Output;
}

impl<T: Float> Conj for T {
    fn conj(self) -> Self {
        self
    }
}

impl<T: Float> SqrAbs for T {
    type Output = T;
    fn sqr_abs(self) -> Self {
        self*self
    }
}


/// Some algebra over real numbers (e.g. Float, Complex, Quaternion)
pub trait Algebra<T: Float>:
    Copy +
    Num +
    Mul<T, Output=Self> +
    Div<T, Output=Self> +
    Neg<Output=Self> +
    Conj +
    SqrAbs<Output=T>
{}

impl<T: Float> Algebra<T> for T {}


/// Dummy test to force codecov look at this file
#[cfg(test)]
#[test]
fn dummy() {
    assert_eq!(1, 1);
}
