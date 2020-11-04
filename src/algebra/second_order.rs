use core::{
    ops::{
        Add, Sub, Mul, Div,
        AddAssign, SubAssign, MulAssign, DivAssign,
    },
};
use super::{traits::*, construct::*};

impl<T: Algebra + Copy, U: Algebra<T> + Copy> Construct<T, Construct<T, U>> {
    /// Create from four parts.
    pub fn new2(w: U, x: U, y: U, z: U) -> Self {
        Self::new(Construct::new(w, x), Construct::new(y, z))
    }
    pub fn w(self) -> U {
        self.re.re
    }
    pub fn x(self) -> U {
        self.re.im
    }
    pub fn y(self) -> U {
        self.im.re
    }
    pub fn z(self) -> U {
        self.im.im
    }
}

impl<T: Algebra + Copy, U: Algebra<T> + Copy> Add<Construct<T, U>> for Construct<T, Construct<T, U>> {
    type Output = Self;
    fn add(self, other: Construct<T, U>) -> Self::Output {
        Self::new(self.re + other, self.im)
    }
}
impl<T: Algebra + Copy, U: Algebra<T> + Copy> Sub<Construct<T, U>> for Construct<T, Construct<T, U>> {
    type Output = Self;
    fn sub(self, other: Construct<T, U>) -> Self::Output {
        Self::new(self.re - other, self.im)
    }
}
impl<T: Algebra + Copy, U: Algebra<T> + Copy> Add<Construct<T, Construct<T, U>>> for Construct<T, U> {
    type Output = Construct<T, Construct<T, U>>;
    fn add(self, other: Construct<T, Construct<T, U>>) -> Self::Output {
        Self::Output::new(self + other.re, other.im)
    }
}
impl<T: Algebra + Copy, U: Algebra<T> + Copy> Sub<Construct<T, Construct<T, U>>> for Construct<T, U> {
    type Output = Construct<T, Construct<T, U>>;
    fn sub(self, other: Construct<T, Construct<T, U>>) -> Self::Output {
        Self::Output::new(self - other.re, other.im)
    }
}

impl<T: Algebra + Copy, U: Algebra<T> + Copy> Mul<Construct<T, U>> for Construct<T, Construct<T, U>> {
    type Output = Self;
    fn mul(self, other: Construct<T, U>) -> Self::Output {
        Self::new(self.re * other, self.im * other)
    }
}
impl<T: Algebra + Copy, U: Algebra<T> + Copy> Div<Construct<T, U>> for Construct<T, Construct<T, U>> {
    type Output = Self;
    fn div(self, other: Construct<T, U>) -> Self::Output {
        Self::new(self.re / other, self.im / other)
    }
}
impl<T: Algebra + Copy, U: Algebra<T> + Copy> Mul<Construct<T, Construct<T, U>>> for Construct<T, U> {
    type Output = Construct<T, Construct<T, U>>;
    fn mul(self, other: Construct<T, Construct<T, U>>) -> Self::Output {
        Self::Output::new(self * other.re, self * other.im)
    }
}
impl<T: Algebra + Copy, U: Algebra<T> + Copy> Div<Construct<T, Construct<T, U>>> for Construct<T, U> {
    type Output = Construct<T, Construct<T, U>>;
    fn div(self, other: Construct<T, Construct<T, U>>) -> Self::Output {
        Self::Output::new(self / other.re, self / other.im)
    }
}

impl<T: Algebra + Copy, U: Algebra<T> + Copy> AddAssign<Construct<T, U>> for Construct<T, Construct<T, U>> {
    fn add_assign(&mut self, other: Construct<T, U>) -> () {
        *self = *self + other;
    }
}
impl<T: Algebra + Copy, U: Algebra<T> + Copy> SubAssign<Construct<T, U>> for Construct<T, Construct<T, U>> {
    fn sub_assign(&mut self, other: Construct<T, U>) -> () {
        *self = *self - other;
    }
}
impl<T: Algebra + Copy, U: Algebra<T> + Copy> MulAssign<Construct<T, U>> for Construct<T, Construct<T, U>> {
    fn mul_assign(&mut self, other: Construct<T, U>) -> () {
        *self = *self * other;
    }
}
impl<T: Algebra + Copy, U: Algebra<T> + Copy> DivAssign<Construct<T, U>> for Construct<T, Construct<T, U>> {
    fn div_assign(&mut self, other: Construct<T, U>) -> () {
        *self = *self / other;
    }
}
