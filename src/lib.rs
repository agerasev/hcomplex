use std::ops::{
    Neg, Add, Sub, Mul, Div, 
    AddAssign, SubAssign, MulAssign, DivAssign,
};
use num_traits::{Num, Zero, One, Float, Signed};
//use std::fmt::{Display, Formatter, Result as FmtResult};

/// Something that can be conjugated
pub trait Conj {
    fn conj(self) -> Self;
}

impl<T: Float> Conj for T {
    fn conj(self) -> Self {
        self
    }
}

/// Some algebra over real numbers (e.g. Float, Complex, Quaternion)
pub trait Algebra: Copy + Neg<Output=Self> + Conj {}

impl<T: Float> Algebra for T {}

/// Cayley–Dickson construction
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Construction<T: Algebra>(T, T);

impl<T: Algebra> Construction<T> {
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

impl<T: Algebra> Conj for Construction<T> {
    fn conj(self) -> Self {
        Self::new2(self.re().conj(), -self.im())
    }
}

impl<T: Algebra> Neg for Construction<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new2(-self.re(), -self.im())
    }
}

impl<T: Algebra> Algebra for Construction<T> {}

impl<T: Algebra> Construction<Construction<T>> {
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

/// 2-dimensional commutative and associative algebra
pub type Complex<T> = Construction<T>;
/// 4-dimensional associative but non-commutative algebra
pub type Quaternion<T> = Construction<Construction<T>>;
/// 8-dimensional non-commutative and non-associative algebra
pub type Octonion<T> = Construction<Construction<Construction<T>>>;
/// 16-dimensional non-commutative and non-associative algebra with nontrivial zero divisors
pub type Sedenion<T> = Construction<Construction<Construction<Construction<T>>>>;

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
