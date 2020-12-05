use core::ops::{
    Neg, Add, Sub, Mul, Div,
};
use num_traits::{Zero, One};


/// Something that can be conjugated.
pub trait Conj {
    /// Get conjugate value.
    fn conj(self) -> Self;
}

/// Dot product.
pub trait Dot {
    /// Dot product output type.
    type Output;
    /// Perform dot product.
	fn dot(self, other: Self) -> Self::Output;
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
pub trait NormL1 {
    type Output;
    /// Get the L1 norm of the `self`.
    fn norm_l1(self) -> Self::Output;
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

macro_rules! derive_primitive { ($T:ident) => (
    impl Conj for $T {
        fn conj(self) -> Self {
            self
        }
    }
    impl Dot for $T {
        type Output = Self;
        fn dot(self, other: Self) -> Self {
            self*other
        }
    }
    impl NormSqr for $T {
        type Output = Self;
        fn norm_sqr(self) -> Self {
            self*self
        }
    }
    impl Norm for $T {
        type Output = Self;
        fn norm(self) -> Self {
            self.abs()
        }
    }
    impl NormL1 for $T {
        type Output = Self;
        fn norm_l1(self) -> Self {
            self.abs()
        }
    }
    impl Algebra for $T {}
) }

derive_primitive!(i8);
derive_primitive!(i16);
derive_primitive!(i32);
derive_primitive!(i64);

derive_primitive!(f32);
derive_primitive!(f64);
