mod traits;
mod construct;

#[cfg(test)]
mod approx;
#[cfg(test)]
pub use self::approx::*;

#[cfg(test)]
mod tests;


//use num_complex::{Complex as NumComplex};

pub use traits::{Conj, AbsSqr, Algebra};
pub use construct::{Construct};


/// 2-dimensional commutative and associative algebra
pub type Complex<T> = Construct<T, T>;

/// 4-dimensional associative but non-commutative algebra
pub type Quaternion<T> = Construct<T, Complex<T>>;

/// 8-dimensional non-commutative and non-associative algebra
pub type Octonion<T> = Construct<T, Quaternion<T>>;

/// 16-dimensional non-commutative and non-associative algebra with nontrivial zero divisors
pub type Sedenion<T> = Construct<T, Octonion<T>>;


/*
impl<T> From<NumComplex<T>> for Complex<T> {
    fn from(other: NumComplex<T>) -> Self {
        Self::new2(other.re, other.im)
    }
}
impl<T> Into<NumComplex<T>> for Complex<T> {
    fn from(self) -> NumComplex<T> {
        NumComplex { re: self.0, im: self.1 }
    }
}
*/
