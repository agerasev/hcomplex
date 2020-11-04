mod moebius;
pub use moebius::*;

#[cfg(test)]
mod tests;


use crate::{Algebra};


/// Complex and hypercomplex transformation basic trait.
pub trait Transform<T: Algebra, U: Algebra<T>> {
    /// Apply the transformation.
    fn apply(&self, x: U) -> U;
}

/// Transformation that has an identity element.
pub trait Identity {
    /// Get an identity element.
    fn identity() -> Self;
}

/// Differentiable transformation.
pub trait Deriv<T: Algebra, U: Algebra<T>>: Transform<T, U> {
    /// Find the derivative of `self` at the specified point `p`.
    fn deriv(&self, p: U) -> U;
}
/// Directionally differentiable transformation.
pub trait DerivDir<T: Algebra, U: Algebra<T>>: Transform<T, U> {
    /// Find the directinal derivative of `self` at the specified point `p` via the specified direction `d`.
    fn deriv_dir(&self, p: U, d: U) -> U;
}

/// Transformation which instances could be chained into another one (i.e. forms a magma).
pub trait Chain<T: Algebra, U: Algebra<T>>: Transform<T, U> {
    fn chain(&self, other: &Self) -> Self;
}


pub mod prelude {
    pub use super::{Transform, Identity, Deriv, DerivDir, Chain};
}
