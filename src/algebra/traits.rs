use core::ops::{Neg, Add, Sub, Mul, Div};
use num_traits::{Num, Float};


/// Something that can be conjugated.
pub trait Conj {
    /// Conjugate.
    fn conj(self) -> Self;
}
/// Square of the absolute value.
pub trait AbsSqr {
    type Output: Algebra;
    /// Get square of the absolute value.
    fn abs_sqr(self) -> Self::Output;
}

/// Algebra over some base.
pub trait Algebra<T: Algebra = Self>:
    Num +
    Neg<Output=Self> +
    Add<T, Output=Self> +
    Sub<T, Output=Self> +
    Mul<T, Output=Self> +
    Div<T, Output=Self> +
    Conj + AbsSqr<Output=T>
{}

impl<T: Float> Conj for T {
    fn conj(self) -> Self {
        self
    }
}
impl<T: Float + Copy> AbsSqr for T {
    type Output = T;
    fn abs_sqr(self) -> Self {
        self*self
    }
}

impl<T: Float> Algebra for T {}
