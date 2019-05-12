use std::ops::{
    Neg, Add, Sub, Mul, Div, Rem, 
    AddAssign, SubAssign, MulAssign, DivAssign, RemAssign,
};
use num_traits::{Num, Float, Zero, One, Inv};
//use std::fmt::{Display, Formatter, Result as FmtResult};

use std::marker::PhantomData;

use crate::traits::{Conj, SqrAbs, Algebra};


/// Cayley–Dickson construction, a basic building block
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Construct<T: Float, A: Algebra<T>>(pub A, pub A, PhantomData<T>);

impl<T: Float, A: Algebra<T>> Construct<T, A> {
    /// Create from two parts
    pub fn new2(re: A, im: A) -> Self {
        Self(re, im, PhantomData)
    }
}

impl<T: Float, A: Algebra<T>> Conj for Construct<T, A> {
    fn conj(self) -> Self {
        Self::new2(self.0.conj(), -self.1)
    }
}

impl<T: Float, A: Algebra<T>> Neg for Construct<T, A> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new2(-self.0, -self.1)
    }
}

impl<T: Float, A: Algebra<T>> Add for Construct<T, A> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::new2(self.0 + other.0, self.1 + other.1)
    }
}

impl<T: Float, A: Algebra<T>> Sub for Construct<T, A> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::new2(self.0 - other.0, self.1 - other.1)
    }
}

impl<T: Float, A: Algebra<T>> Mul<T> for Construct<T, A> {
    type Output = Self;
    fn mul(self, other: T) -> Self::Output {
        Self::new2(self.0*other, self.1*other)
    }
}

impl<T: Float, A: Algebra<T>> Div<T> for Construct<T, A> {
    type Output = Self;
    fn div(self, other: T) -> Self::Output {
        Self::new2(self.0/other, self.1/other)
    }
}

impl<T: Float, A: Algebra<T>> SqrAbs for Construct<T, A> {
    type Output = T;
    fn sqr_abs(self) -> T {
        self.0.sqr_abs() + self.1.sqr_abs()
    }
}

impl<T: Float, A: Algebra<T>> Construct<T, A> {
    pub fn abs(self) -> T {
        self.sqr_abs().sqrt()
    }
}

impl<T: Float, A: Algebra<T>> Inv for Construct<T, A> {
    type Output = Self;
    fn inv(self) -> Self {
        self.conj()/self.sqr_abs()
    }
}

macro_rules! rmul {
    ($F:ident) => (
        /// Workaround for reverse multiplication
        impl<A: Algebra<$F>> Mul<Construct<$F, A>> for $F {
            type Output = Construct<$F, A>;
            fn mul(self, other: Construct<$F, A>) -> Self::Output {
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
        impl<A: Algebra<$F>> Div<Construct<$F, A>> for $F {
            type Output = Construct<$F, A>;
            fn div(self, other: Construct<$F, A>) -> Self::Output {
                other.inv()*self
            }
        }
    )
}
rdiv!(f32);
rdiv!(f64);

impl<T: Float, A: Algebra<T>> Mul for Construct<T, A> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self::new2(
            self.0*other.0 - other.1.conj()*self.1,
            other.1*self.0 + self.1*other.0.conj(),
        )
    }
}

impl<T: Float, A: Algebra<T>> Div for Construct<T, A> {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        self*other.inv()
    }
}

impl<T: Float, A: Algebra<T>> Zero for Construct<T, A> {
    fn zero() -> Self {
        Self::new2(A::zero(), A::zero())
    }
    fn is_zero(&self) -> bool {
        self.0.is_zero() && self.1.is_zero()
    }
}

impl<T: Float, A: Algebra<T>> One for Construct<T, A> {
    fn one() -> Self {
        Self::new2(A::one(), A::zero())
    }
}

/// Not implemented yet
impl<T: Float, A: Algebra<T>> Rem for Construct<T, A> {
    type Output = Self;
    fn rem(self, _other: Self) -> Self::Output {
        unimplemented!()
    }
}

/// Not implemented yet
impl<T: Float, A: Algebra<T>> Num for Construct<T, A> {
    type FromStrRadixErr = ();
    fn from_str_radix(_str: &str, _radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        unimplemented!()
    }
}

impl<T: Float, A: Algebra<T>> Algebra<T> for Construct<T, A> {}


impl<T: Float, A: Algebra<T>> AddAssign for Construct<T, A> {
    fn add_assign(&mut self, other: Self) -> () {
        *self = *self + other;
    }
}

impl<T: Float, A: Algebra<T>> SubAssign for Construct<T, A> {
    fn sub_assign(&mut self, other: Self) -> () {
        *self = *self - other;
    }
}

impl<T: Float, A: Algebra<T>> MulAssign for Construct<T, A> {
    fn mul_assign(&mut self, other: Self) -> () {
        *self = *self*other;
    }
}

impl<T: Float, A: Algebra<T>> DivAssign for Construct<T, A> {
    fn div_assign(&mut self, other: Self) -> () {
        *self = *self/other;
    }
}

impl<T: Float, A: Algebra<T>> RemAssign for Construct<T, A> {
    fn rem_assign(&mut self, other: Self) -> () {
        *self = *self % other;
    }
}

impl<T: Float, A: Algebra<T>> MulAssign<T> for Construct<T, A> {
    fn mul_assign(&mut self, other: T) -> () {
        *self = *self*other;
    }
}

impl<T: Float, A: Algebra<T>> DivAssign<T> for Construct<T, A> {
    fn div_assign(&mut self, other: T) -> () {
        *self = *self/other;
    }
}


impl<T: Float, A: Algebra<T>> Construct<T, A> {
    #[inline]
    pub fn re(self) -> A {
        self.0
    }
    #[inline]
    pub fn im(self) -> A {
        self.1
    }
}

impl<T: Float, A: Algebra<T>> Construct<T, Construct<T, A>> {
    /// Create from four parts
    pub fn new4(w: A, x: A, y: A, z: A) -> Self {
        Self::new2(Construct::new2(w, x), Construct::new2(y, z))
    }

    #[inline]
    pub fn w(self) -> A {
        self.re().re()
    }
    #[inline]
    pub fn x(self) -> A {
        self.re().im()
    }
    #[inline]
    pub fn y(self) -> A {
        self.im().re()
    }
    #[inline]
    pub fn z(self) -> A {
        self.im().im()
    }
}


/// Dummy test to force codecov look at this file
#[cfg(test)]
#[test]
fn dummy() {
    assert_eq!(1, 1);
}
