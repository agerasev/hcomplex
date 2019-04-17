use std::ops::{
    Neg, Add, Sub, Mul, Div, 
    // AddAssign, SubAssign, MulAssign, DivAssign,
};
use num_traits::{Num, Zero, One, Float, Signed};
//use std::fmt::{Display, Formatter, Result as FmtResult};

use std::marker::PhantomData;


/// Something that can be conjugated
pub trait Conj {
    fn conj(self) -> Self;
}

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
    Add<Output=Self> +
    Sub<Output=Self> +
    Mul<T, Output=Self> +
    Div<T, Output=Self> +
    Mul<Output=Self> +
    Div<Output=Self> +
    Neg<Output=Self> +
    Conj +
    SqrAbs<Output=T>
{}

impl<T: Float> Algebra<T> for T {}

/// Cayley–Dickson construction, a basic building block
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Construction<T: Float, A: Algebra<T>>(A, A, PhantomData<T>);

impl<T: Float, A: Algebra<T>> Construction<T, A> {
    /// Create from two parts
    pub fn new2(re: A, im: A) -> Self {
        Self(re, im, PhantomData)
    }
}

impl<T: Float, A: Algebra<T>> Conj for Construction<T, A> {
    fn conj(self) -> Self {
        Self::new2(self.0.conj(), -self.1)
    }
}

impl<T: Float, A: Algebra<T>> Neg for Construction<T, A> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new2(-self.0, -self.1)
    }
}

impl<T: Float, A: Algebra<T>> Add for Construction<T, A> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::new2(self.0 + other.0, self.1 + other.1)
    }
}

impl<T: Float, A: Algebra<T>> Sub for Construction<T, A> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::new2(self.0 - other.0, self.1 - other.1)
    }
}

// Multiplication and division

impl<T: Float, A: Algebra<T>> Mul<T> for Construction<T, A> {
    type Output = Self;
    fn mul(self, other: T) -> Self::Output {
        Self::new2(self.0*other, self.1*other)
    }
}

impl<T: Float, A: Algebra<T>> Div<T> for Construction<T, A> {
    type Output = Self;
    fn div(self, other: T) -> Self::Output {
        Self::new2(self.0/other, self.1/other)
    }
}

impl<T: Float, A: Algebra<T>> SqrAbs for Construction<T, A> {
    type Output = T;
    fn sqr_abs(self) -> T {
        self.0.sqr_abs() + self.1.sqr_abs()
    }
}

impl<T: Float, A: Algebra<T>> Construction<T, A> {
    pub fn abs(self) -> T {
        self.sqr_abs().sqrt()
    }
}

impl<T: Float, A: Algebra<T>> Construction<T, A> {
    /// Inversion 1/x
    pub fn inverse(self) -> Self {
        self.conj()/self.sqr_abs()
    }
}

macro_rules! rmul {
    ($F:ident) => (
        /// Workaround for reverse multiplication
        impl<A: Algebra<$F>> Mul<Construction<$F, A>> for $F {
            type Output = Construction<$F, A>;
            fn mul(self, other: Construction<$F, A>) -> Self::Output {
                other*self
            }
        }
    )
}
rmul!(f32);
rmul!(f64);

macro_rules! rdiv {
    ($F:ident) => (
        /// Workaround for reverse division
        impl<A: Algebra<$F>> Div<Construction<$F, A>> for $F {
            type Output = Construction<$F, A>;
            fn div(self, other: Construction<$F, A>) -> Self::Output {
                other.inverse()*self
            }
        }
    )
}
rdiv!(f32);
rdiv!(f64);

impl<T: Float, A: Algebra<T>> Mul for Construction<T, A> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self::new2(
            self.0*other.0 - other.1.conj()*self.1,
            other.1*self.0 + self.1*other.0.conj(),
        )
    }
}

impl<T: Float, A: Algebra<T>> Div for Construction<T, A> {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        self*other.inverse()
    }
}

impl<T: Float, A: Algebra<T>> Algebra<T> for Construction<T, A> {}

// Fields

impl<T: Float, A: Algebra<T>> Construction<T, A> {
    #[inline] 
    pub fn re(self) -> A {
        self.0
    }
    #[inline]
    pub fn im(self) -> A {
        self.1
    }
}

impl<T: Float, A: Algebra<T>> Construction<T, Construction<T, A>> {
    /// Create from four parts
    fn new4(w: A, x: A, y: A, z: A) -> Self {
        Self::new2(Construction::new2(w, x), Construction::new2(y, z))
    }

    #[inline]
    fn w(self) -> A {
        self.re().re()
    }
    #[inline]
    fn x(self) -> A {
        self.re().im()
    }
    #[inline]
    fn y(self) -> A {
        self.im().re()
    }
    #[inline]
    fn z(self) -> A {
        self.im().im()
    }
}

// Type aliases

/// 2-dimensional commutative and associative algebra
pub type Complex<T> = Construction<T, T>;
/// 4-dimensional associative but non-commutative algebra
pub type Quaternion<T> = Construction<T, Construction<T, T>>;
/// 8-dimensional non-commutative and non-associative algebra
pub type Octonion<T> = Construction<T, Construction<T, Construction<T, T>>>;
/// 16-dimensional non-commutative and non-associative algebra with nontrivial zero divisors
pub type Sedenion<T> = Construction<T, Construction<T, Construction<T, Construction<T, T>>>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new2() {
        let c = Complex::<f32>::new2(1.0, 2.0);
        assert_eq!(c.re(), 1.0);
        assert_eq!(c.im(), 2.0);
    }

    #[test]
    fn conj2() {
        let c = Complex::<f32>::new2(1.0, 2.0).conj();
        assert_eq!(c.re(), 1.0);
        assert_eq!(c.im(), -2.0);
    }

    #[test]
    fn conj4() {
        let c = Quaternion::<f32>::new4(1.0, 2.0, 3.0, 4.0).conj();
        assert_eq!(c.w(), 1.0);
        assert_eq!(c.x(), -2.0);
        assert_eq!(c.y(), -3.0);
        assert_eq!(c.z(), -4.0);
    }

    #[test]
    fn add2() {
        let a = Complex::<f32>::new2(1.0, 2.0);
        let b = Complex::<f32>::new2(3.0, 4.0);
        let c = a + b;
        assert_eq!(c.re(), 4.0);
        assert_eq!(c.im(), 6.0);
    }

    #[test]
    fn sub2() {
        let a = Complex::<f32>::new2(2.0, 1.0);
        let b = Complex::<f32>::new2(3.0, 4.0);
        let c = a - b;
        assert_eq!(c.re(), -1.0);
        assert_eq!(c.im(), -3.0);
    }

    #[test]
    fn add4() {
        let a = Quaternion::<f32>::new4(1.0, 2.0, 3.0, 4.0);
        let b = Quaternion::<f32>::new4(5.0, 6.0, 7.0, 8.0);
        let c = a + b;
        assert_eq!(c.w(), 6.0);
        assert_eq!(c.x(), 8.0);
        assert_eq!(c.y(), 10.0);
        assert_eq!(c.z(), 12.0);
    }

    #[test]
    fn sub4() {
        let a = Quaternion::<f32>::new4(4.0, 3.0, 2.0, 1.0);
        let b = Quaternion::<f32>::new4(5.0, 6.0, 7.0, 8.0);
        let c = a - b;
        assert_eq!(c.w(), -1.0);
        assert_eq!(c.x(), -3.0);
        assert_eq!(c.y(), -5.0);
        assert_eq!(c.z(), -7.0);
    }
}
