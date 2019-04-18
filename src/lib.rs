pub mod traits;
pub mod construction;

#[cfg(test)]
mod test;

pub mod prelude {
    pub use crate::traits::{Conj, SqrAbs};
}

pub use traits::{Conj, SqrAbs};
pub use construction::{Construction};


/// 2-dimensional commutative and associative algebra
pub type Complex<T> = Construction<T, T>;

/// 4-dimensional associative but non-commutative algebra
pub type Quaternion<T> = Construction<T, Complex<T>>;

/// 8-dimensional non-commutative and non-associative algebra
pub type Octonion<T> = Construction<T, Quaternion<T>>;

/// 16-dimensional non-commutative and non-associative algebra with nontrivial zero divisors
pub type Sedenion<T> = Construction<T, Octonion<T>>;
