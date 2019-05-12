pub mod traits;
pub mod construct;
pub mod transform;

#[cfg(test)]
mod test;

pub mod prelude {
    pub use crate::traits::{Conj, SqrAbs};
}

pub use traits::{Conj, SqrAbs};
pub use construct::{Construct};


/// 2-dimensional commutative and associative algebra
pub type Complex<T> = Construct<T, T>;

/// 4-dimensional associative but non-commutative algebra
pub type Quaternion<T> = Construct<T, Complex<T>>;

/// 8-dimensional non-commutative and non-associative algebra
pub type Octonion<T> = Construct<T, Quaternion<T>>;

/// 16-dimensional non-commutative and non-associative algebra with nontrivial zero divisors
pub type Sedenion<T> = Construct<T, Octonion<T>>;
