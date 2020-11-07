use core::ops::Mul;


/// Complex and hypercomplex transformation basic trait.
pub trait Transform<U> {
    /// Apply the transformation.
    fn apply(&self, x: U) -> U;
}

/// Transformation that has an identity element.
pub trait Identity {
    /// Get an identity element.
    fn identity() -> Self;
}

/// Transformation which instances could be chained into another one (i.e. forms a magma).
pub trait Chain<U>: Transform<U> {
    fn chain(self, other: Self) -> Self;
}

/// Differentiable transformation.
pub trait Deriv<U>: Transform<U> {
    /// Find the derivative of `self` at the specified point `p`.
    fn deriv(&self, p: U) -> U;
}
/// Directionally differentiable transformation.
pub trait DerivDir<U>: Transform<U> {
    /// Find the directinal derivative of `self` at the specified point `p` via the specified direction `d`.
    fn deriv_dir(&self, p: U, d: U) -> U;
}
impl<U: Mul<U, Output=U>, M> DerivDir<U> for M where M: Deriv<U> {
    fn deriv_dir(&self, p: U, d: U) -> U {
        self.deriv(p) * d
    }
}
