use core::{
    ops::{
        Neg, Add, Sub, Mul, Div, Rem,
        AddAssign, SubAssign, MulAssign, DivAssign, RemAssign,
    },
    marker::PhantomData,
};
use num_traits::{Num, Zero, One, Float};
//use core::fmt::{Display, Formatter, Result as FmtResult};

use super::traits::{Conj, AbsSqr, Algebra};


/// Cayley–Dickson construction, a basic building block.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Construct<T: Algebra, U: Algebra<T>> {
    pub(super) re: U,
    pub(super) im: U,
    phantom: PhantomData<T>,
}

impl<T: Algebra + Copy, U: Algebra<T> + Copy> Algebra<T> for Construct<T, U> {}

impl<T: Algebra, U: Algebra<T>> Construct<T, U> {
    /// Create from real and imaginary parts.
    pub fn new(re: U, im: U) -> Self {
        Self { re, im, phantom: PhantomData }
    }
    /// Split by real and imaginary parts.
    pub fn split(self) -> (U, U) {
        (self.re, self.im)
    }
}
impl<T: Algebra, U: Algebra<T> + Copy> Construct<T, U> {
    pub fn re(self) -> U {
        self.re
    }
    pub fn im(self) -> U {
        self.im
    }
}

impl<T: Algebra, U: Algebra<T>> Conj for Construct<T, U> {
    fn conj(self) -> Self {
        Self::new(self.re.conj(), -self.im)
    }
}

impl<T: Algebra, U: Algebra<T>> AbsSqr for Construct<T, U> {
    type Output = T;
    fn abs_sqr(self) -> T {
        self.re.abs_sqr() + self.im.abs_sqr()
    }
}
impl<T: Float, U: Algebra<T>> Construct<T, U> {
    pub fn abs(self) -> T {
        self.abs_sqr().sqrt()
    }
}
impl<T: Float, U: Algebra<T> + Copy> Construct<T, U> {
    pub fn normalize(self) -> Self {
        self / self.abs()
    }
}

impl<T: Algebra, U: Algebra<T>> Neg for Construct<T, U> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.re, -self.im)
    }
}

impl<T: Algebra, U: Algebra<T>> Add for Construct<T, U> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::new(self.re + other.re, self.im + other.im)
    }
}
impl<T: Algebra, U: Algebra<T>> Sub for Construct<T, U> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.re - other.re, self.im - other.im)
    }
}
impl<T: Algebra, U: Algebra<T>> Add<T> for Construct<T, U> {
    type Output = Self;
    fn add(self, other: T) -> Self::Output {
        Self::new(self.re + other, self.im)
    }
}
impl<T: Algebra, U: Algebra<T>> Sub<T> for Construct<T, U> {
    type Output = Self;
    fn sub(self, other: T) -> Self::Output {
        Self::new(self.re - other, self.im)
    }
}

impl<T: Algebra + Copy, U: Algebra<T>> Mul<T> for Construct<T, U> {
    type Output = Self;
    fn mul(self, other: T) -> Self::Output {
        Self::new(self.re * other, self.im * other)
    }
}
impl<T: Algebra + Copy, U: Algebra<T>> Div<T> for Construct<T, U> {
    type Output = Self;
    fn div(self, other: T) -> Self::Output {
        Self::new(self.re / other, self.im / other)
    }
}
impl<T: Algebra, U: Algebra<T> + Copy> Mul for Construct<T, U> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self::new(
            self.re * other.re - other.im.conj() * self.im,
            other.im * self.re + self.im * other.re.conj(),
        )
    }
}
impl<T: Algebra + Copy, U: Algebra<T> + Copy> Construct<T, U> {
    fn inv(self) -> Self {
        self.conj() / self.abs_sqr()
    }
}
impl<T: Algebra + Copy, U: Algebra<T> + Copy> Div for Construct<T, U> {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        self * other.inv()
    }
}

impl<T: Algebra, U: Algebra<T>> Zero for Construct<T, U> {
    fn zero() -> Self {
        Self::new(U::zero(), U::zero())
    }
    fn is_zero(&self) -> bool {
        self.re.is_zero() && self.im.is_zero()
    }
}
impl<T: Algebra, U: Algebra<T> + Copy> One for Construct<T, U> {
    fn one() -> Self {
        Self::new(U::one(), U::zero())
    }
}

/// Not implemented yet.
impl<T: Algebra, U: Algebra<T>> Rem for Construct<T, U> {
    type Output = Self;
    fn rem(self, _other: Self) -> Self::Output {
        unimplemented!()
    }
}
/// Not implemented yet.
impl<T: Algebra + Copy, U: Algebra<T> + Copy> Num for Construct<T, U> {
    type FromStrRadixErr = ();
    fn from_str_radix(_str: &str, _radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        unimplemented!()
    }
}

macro_rules! radd { ($T:ident) => (
    /// Workaround for reverse addition.
    impl<U: Algebra<$T>> Add<Construct<$T, U>> for $T {
        type Output = Construct<$T, U>;
        fn add(self, other: Construct<$T, U>) -> Self::Output {
            other + self
        }
    }
) }
macro_rules! rsub { ($T:ident) => (
    /// Workaround for reverse subtraction.
    impl<U: Algebra<$T>> Sub<Construct<$T, U>> for $T {
        type Output = Construct<$T, U>;
        fn sub(self, other: Construct<$T, U>) -> Self::Output {
            -other + self
        }
    }
) }
macro_rules! rmul { ($T:ident) => (
    /// Workaround for reverse multiplication.
    impl<U: Algebra<$T>> Mul<Construct<$T, U>> for $T {
        type Output = Construct<$T, U>;
        fn mul(self, other: Construct<$T, U>) -> Self::Output {
            other*self
        }
    }
) }
macro_rules! rdiv { ($T:ident) => (
    /// Workaround for reverse division.
    impl<U: Algebra<$T> + Copy> Div<Construct<$T, U>> for $T {
        type Output = Construct<$T, U>;
        fn div(self, other: Construct<$T, U>) -> Self::Output {
            other.inv()*self
        }
    }
) }
macro_rules! reverse { ($T:ident) => (
    radd!($T);
    rsub!($T);
    rmul!($T);
    rdiv!($T);
) }
reverse!(f32);
reverse!(f64);

impl<T: Algebra, U: Algebra<T>> AddAssign for Construct<T, U> where U: AddAssign {
    fn add_assign(&mut self, other: Self) -> () {
        self.re += other.re;
        self.im += other.im;
    }
}
impl<T: Algebra, U: Algebra<T>> SubAssign for Construct<T, U> where U: SubAssign {
    fn sub_assign(&mut self, other: Self) -> () {
        self.re -= other.re;
        self.im -= other.im;
    }
}
impl<T: Algebra + Copy, U: Algebra<T> + Copy> MulAssign for Construct<T, U> {
    fn mul_assign(&mut self, other: Self) -> () {
        *self = *self * other;
    }
}
impl<T: Algebra + Copy, U: Algebra<T> + Copy> DivAssign for Construct<T, U> {
    fn div_assign(&mut self, other: Self) -> () {
        *self = *self / other;
    }
}
impl<T: Algebra + Copy, U: Algebra<T> + Copy> RemAssign for Construct<T, U> {
    fn rem_assign(&mut self, other: Self) -> () {
        *self = *self % other;
    }
}
impl<T: Algebra + Copy, U: Algebra<T> + Copy> MulAssign<T> for Construct<T, U> {
    fn mul_assign(&mut self, other: T) -> () {
        *self = *self * other;
    }
}
impl<T: Algebra + Copy, U: Algebra<T> + Copy> DivAssign<T> for Construct<T, U> {
    fn div_assign(&mut self, other: T) -> () {
        *self = *self / other;
    }
}
