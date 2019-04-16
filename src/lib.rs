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
    Neg<Output=Self> +
    Conj +
    SqrAbs<Output=T>
{}

impl<T: Float> Algebra<T> for T {}

/// Cayley–Dickson construction
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Construction<T: Float, A: Algebra<T>>(A, A, PhantomData<T>);


impl<T: Float, A: Algebra<T>> Conj for Construction<T, A> {
    fn conj(self) -> Self {
        Self(self.0.conj(), -self.1, PhantomData)
    }
}

impl<T: Float, A: Algebra<T>> Neg for Construction<T, A> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, PhantomData)
    }
}

impl<T: Float, A: Algebra<T>> Add for Construction<T, A> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1, PhantomData)
    }
}

impl<T: Float, A: Algebra<T>> Sub for Construction<T, A> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1, PhantomData)
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

impl<T: Float, A: Algebra<T>> Algebra<T> for Construction<T, A> {}

// Fields and constructors

/*
impl<T: Float, A: Algebra<T>> Construction<T, A> {
    pub fn new2(re: T, im: T) -> Self {
        Self(re, im)
    }

    #[inline] 
    pub fn re(self) -> T {
        self.0
    }
    #[inline]
    pub fn im(self) -> T {
        self.1
    }
}

impl<T: Float, A: Algebra<T>> Construction<T, Construction<T, A>> {
    fn new4(w: T, x: T, y: T, z: T) -> Self {
        Self::new2(Construction::new2(w, x), Construction::new2(y, z))
    }

    #[inline]
    fn w(self) -> T {
        self.re().re()
    }
    #[inline]
    fn x(self) -> T {
        self.re().im()
    }
    #[inline]
    fn y(self) -> T {
        self.im().re()
    }
    #[inline]
    fn z(self) -> T {
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
}
*/
