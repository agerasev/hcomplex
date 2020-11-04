use core::ops::{Neg, Add, Sub, Mul, Div};
use num_traits::{Zero, One, Float};


/// Something that can be conjugated.
pub trait Conj {
    /// Get conjugate value.
    fn conj(self) -> Self;
}

/// Square of L2 norm.
pub trait NormSqr: Sized {
    type Output;
    /// Get square of the norm of the `self`.
    fn norm_sqr(self) -> Self::Output;
    /// Alias to `norm_sqr`.
    fn abs_sqr(self) -> Self::Output {
        self.norm_sqr()
    }
}

/// L2 (Euclidean) Norm.
pub trait Norm: Sized {
    type Output;
    /// Get the norm of the `self`.
    fn norm(self) -> Self::Output;
    /// Alias to `norm`.
    fn abs(self) -> Self::Output {
        self.norm()
    }
}

/// L1 (Manhattan) Norm.
pub trait L1Norm {
    type Output;
    /// Get the L1 norm of the `self`.
    fn l1_norm(self) -> Self::Output;
}

/// Algebra over some base.
pub trait Algebra<T: Algebra = Self>:
    Neg<Output=Self> +
    Add<Output=Self> +
    Sub<Output=Self> +
    Mul<Output=Self> +
    Div<Output=Self> +
    Add<T, Output=Self> +
    Sub<T, Output=Self> +
    Mul<T, Output=Self> +
    Div<T, Output=Self> +
    Zero +
    One +
    Conj +
    NormSqr<Output=T>
{}

impl<T: Float> Conj for T {
    fn conj(self) -> Self {
        self
    }
}
impl<T: Float + Clone> NormSqr for T {
    type Output = T;
    fn norm_sqr(self) -> Self {
        self*self
    }
}
impl<T: Float> Norm for T {
    type Output = T;
    fn norm(self) -> Self {
        self.abs()
    }
}
impl<T: Float> L1Norm for T {
    type Output = T;
    fn l1_norm(self) -> Self {
        self.abs()
    }
}

impl<T: Float> Algebra for T {}
