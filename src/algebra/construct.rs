use core::{
    ops::{
        Neg, Add, Sub, Mul, Div, Rem,
        AddAssign, SubAssign, MulAssign, DivAssign,
    },
    marker::PhantomData,
};
use num_traits::{Zero, One, Float, Inv, Num};

use super::traits::{Conj, NormSqr, Norm, L1Norm, Algebra};


/// Cayley–Dickson construction, a basic building block.
///
/// Structure takes two type parameters:
/// + The first one, `T`: a scalar type the algebra is built over.
/// + The second one, `U`: is a type of two components of the construction: `re` and `im`.
#[derive(Clone, Copy, PartialEq)]
pub struct Construct<T, U> {
    re: U,
    im: U,
    ph: PhantomData<T>,
}

impl<T, U> Construct<T, U> {
    /// Create from real and imaginary parts.
    pub fn new(re: U, im: U) -> Self {
        Self { re, im, ph: PhantomData }
    }
    /// Split by real and imaginary parts.
    pub fn split(self) -> (U, U) {
        (self.re, self.im)
    }

    pub fn re_ref(&self) -> &U {
        &self.re
    }
    pub fn im_ref(&self) -> &U {
        &self.im
    }
    pub fn re_mut(&mut self) -> &mut U {
        &mut self.re
    }
    pub fn im_mut(&mut self) -> &mut U {
        &mut self.im
    }
}
impl<T, U> Construct<T, U> where U: Clone {
    pub fn re(&self) -> U {
        self.re.clone()
    }
    pub fn im(&self) -> U {
        self.im.clone()
    }
}

impl<T, U> Conj for Construct<T, U> where U: Conj + Neg<Output=U> {
    fn conj(self) -> Self {
        Self::new(self.re.conj(), -self.im)
    }
}

impl<T, U> NormSqr for Construct<T, U> where T: Add<Output=T>, U: NormSqr<Output=T> {
    type Output = T;
    fn norm_sqr(self) -> T {
        self.re.norm_sqr() + self.im.norm_sqr()
    }
}
impl<T, U> Norm for Construct<T, U> where T: Float, Self: NormSqr<Output=T> {
    type Output = T;
    fn norm(self) -> T {
        self.norm_sqr().sqrt()
    }
}
impl<T, U> L1Norm for Construct<T, U> where T: Add<Output=T>, U: L1Norm<Output=T> {
    type Output = T;
    fn l1_norm(self) -> T {
        self.re.l1_norm() + self.im.l1_norm()
    }
}

impl<T, U> Neg for Construct<T, U> where U: Neg<Output=U> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.re, -self.im)
    }
}

impl<T, U> Add for Construct<T, U> where U: Add<Output=U> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::new(self.re + other.re, self.im + other.im)
    }
}
impl<T, U> Sub for Construct<T, U> where U: Sub<Output=U> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.re - other.re, self.im - other.im)
    }
}
impl<T, U> Add<T> for Construct<T, U> where U: Add<T, Output=U> {
    type Output = Self;
    fn add(self, other: T) -> Self::Output {
        Self::new(self.re + other, self.im)
    }
}
impl<T, U> Sub<T> for Construct<T, U> where U: Sub<T, Output=U> {
    type Output = Self;
    fn sub(self, other: T) -> Self::Output {
        Self::new(self.re - other, self.im)
    }
}

impl<T, U> Mul<T> for Construct<T, U> where T: Clone, U: Mul<T, Output=U> {
    type Output = Self;
    fn mul(self, other: T) -> Self::Output {
        Self::new(self.re * other.clone(), self.im * other)
    }
}
impl<T, U> Div<T> for Construct<T, U> where T: Clone, U: Div<T, Output=U> {
    type Output = Self;
    fn div(self, other: T) -> Self::Output {
        Self::new(self.re / other.clone(), self.im / other)
    }
}
impl<T, U> Mul for Construct<T, U> where U: Clone + Conj + Mul<Output=U> + Add<Output=U> + Sub<Output=U> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self::new(
            self.re() * other.re() - other.im().conj() * self.im(),
            other.im() * self.re() + self.im() * other.re().conj(),
        )
    }
}
impl<T, U> Inv for Construct<T, U> where Self: Clone + Conj + NormSqr<Output=T> + Div<T, Output=Self> {
    type Output = Self;
    fn inv(self) -> Self {
        self.clone().conj() / self.norm_sqr()
    }
}
impl<T, U> Div for Construct<T, U> where Self: Inv<Output=Self> + Mul<Output=Self> {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        self * other.inv()
    }
}

impl<T, U> Zero for Construct<T, U> where U: Zero {
    fn zero() -> Self {
        Self::new(U::zero(), U::zero())
    }
    fn is_zero(&self) -> bool {
        self.re.is_zero() && self.im.is_zero()
    }
}
impl<T, U> One for Construct<T, U> where U: Zero + One, Self: Mul<Output=Self> {
    fn one() -> Self {
        Self::new(U::one(), U::zero())
    }
}

impl<T, U> Construct<T, U> where Self: Clone + Norm<Output=T> + Div<T, Output=Self> {
    pub fn normalize(self) -> Self {
        self.clone() / self.norm()
    }
}

/// Not implemented yet.
impl<T: Num + Algebra + Clone, U: Num + Algebra<T> + Clone> Rem for Construct<T, U> {
    type Output = Self;
    fn rem(self, _other: Self) -> Self::Output {
        unimplemented!()
    }
}
/// Not implemented yet.
impl<T: Num + Algebra + Clone, U: Num + Algebra<T> + Clone> Num for Construct<T, U> {
    type FromStrRadixErr = ();
    fn from_str_radix(_str: &str, _radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        unimplemented!()
    }
}

impl<T, U> Algebra<T> for Construct<T, U> where T: Algebra + Clone, U: Algebra<T> + Clone {}

macro_rules! radd { ($T:ident) => (
    /// Workaround for reverse addition.
    impl<U> Add<Construct<$T, U>> for $T where Construct<$T, U>: Add<$T, Output=Construct<$T, U>> {
        type Output = Construct<$T, U>;
        fn add(self, other: Construct<$T, U>) -> Self::Output {
            other + self
        }
    }
) }
macro_rules! rsub { ($T:ident) => (
    /// Workaround for reverse subtraction.
    impl<U> Sub<Construct<$T, U>> for $T where Construct<$T, U>: Neg<Output=Construct<$T, U>> + Add<$T, Output=Construct<$T, U>> {
        type Output = Construct<$T, U>;
        fn sub(self, other: Construct<$T, U>) -> Self::Output {
            -other + self
        }
    }
) }
macro_rules! rmul { ($T:ident) => (
    /// Workaround for reverse multiplication.
    impl<U> Mul<Construct<$T, U>> for $T where Construct<$T, U>: Mul<$T, Output=Construct<$T, U>> {
        type Output = Construct<$T, U>;
        fn mul(self, other: Construct<$T, U>) -> Self::Output {
            other*self
        }
    }
) }
macro_rules! rdiv { ($T:ident) => (
    /// Workaround for reverse division.
    impl<U> Div<Construct<$T, U>> for $T where Construct<$T, U>: Inv<Output=Construct<$T, U>> + Mul<$T, Output=Construct<$T, U>> + Clone {
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


impl<T, U> AddAssign for Construct<T, U> where U: AddAssign {
    fn add_assign(&mut self, other: Self) -> () {
        self.re += other.re;
        self.im += other.im;
    }
}
impl<T, U> SubAssign for Construct<T, U> where U: SubAssign {
    fn sub_assign(&mut self, other: Self) -> () {
        self.re -= other.re;
        self.im -= other.im;
    }
}
impl<T, U> AddAssign<T> for Construct<T, U> where U: AddAssign<T> {
    fn add_assign(&mut self, other: T) -> () {
        self.re += other;
    }
}
impl<T, U> SubAssign<T> for Construct<T, U> where U: SubAssign<T> {
    fn sub_assign(&mut self, other: T) -> () {
        self.re -= other;
    }
}
impl<T, U> MulAssign<T> for Construct<T, U> where Self: Clone + Mul<T, Output=Self> {
    fn mul_assign(&mut self, other: T) -> () {
        *self = self.clone() * other;
    }
}
impl<T, U> DivAssign<T> for Construct<T, U> where Self: Clone + Div<T, Output=Self> {
    fn div_assign(&mut self, other: T) -> () {
        *self = self.clone() / other;
    }
}
impl<T, U> MulAssign for Construct<T, U> where Self: Clone + Mul<Output=Self> {
    fn mul_assign(&mut self, other: Self) -> () {
        *self = self.clone() * other;
    }
}
impl<T, U> DivAssign for Construct<T, U> where Self: Clone + Div<Output=Self> {
    fn div_assign(&mut self, other: Self) -> () {
        *self = self.clone() / other;
    }
}
