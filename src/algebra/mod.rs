mod traits;
mod construct;
mod second_order;
mod math;

#[cfg(feature = "random")]
pub mod random;

#[cfg(feature = "approx")]
pub mod approx;

#[cfg(test)]
mod tests;


use num_complex::{Complex as NumComplex};

pub use traits::{Conj, NormSqr, Norm, L1Norm, Algebra};
pub use construct::{Construct};


/// 2-dimensional commutative and associative algebra.
pub type Complex<T> = Construct<T, T>;

/// 4-dimensional associative but non-commutative algebra.
pub type Quaternion<T> = Construct<T, Complex<T>>;

/// 8-dimensional non-commutative and non-associative algebra.
pub type Octonion<T> = Construct<T, Quaternion<T>>;

/// 16-dimensional non-commutative and non-associative algebra with nontrivial zero divisors.
pub type Sedenion<T> = Construct<T, Octonion<T>>;


impl<T: Algebra> From<NumComplex<T>> for Complex<T> {
    fn from(other: NumComplex<T>) -> Self {
        Self::new(other.re, other.im)
    }
}
impl<T: Algebra> Into<NumComplex<T>> for Complex<T> {
    fn into(self) -> NumComplex<T> {
        let (re, im) = self.split();
        NumComplex { re, im }
    }
}
