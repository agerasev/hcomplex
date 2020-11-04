use core::{
    ops::{
        Add, Sub, Mul, Div,
        AddAssign, SubAssign, MulAssign, DivAssign,
    },
};
use super::{traits::*, construct::*};

impl<T, U> Construct<T, Construct<T, U>> {
    /// Create from four parts.
    pub fn new2(w: U, x: U, y: U, z: U) -> Self {
        Self::new(Construct::new(w, x), Construct::new(y, z))
    }
}

impl<T, U> Construct<T, Construct<T, U>> where U: Clone, Construct<T, U>: Clone {
    pub fn w(&self) -> U {
        self.re().re()
    }
    pub fn x(&self) -> U {
        self.re().im()
    }
    pub fn y(&self) -> U {
        self.im().re()
    }
    pub fn z(&self) -> U {
        self.im().im()
    }
}

impl<T, U> Add<Construct<T, U>> for Construct<T, Construct<T, U>> where T: Algebra + Clone, U: Algebra<T> + Clone {
    type Output = Self;
    fn add(self, other: Construct<T, U>) -> Self::Output {
        Self::new(self.re() + other, self.im())
    }
}
impl<T, U> Sub<Construct<T, U>> for Construct<T, Construct<T, U>> where T: Algebra + Clone, U: Algebra<T> + Clone {
    type Output = Self;
    fn sub(self, other: Construct<T, U>) -> Self::Output {
        Self::new(self.re() - other, self.im())
    }
}
impl<T, U> Add<Construct<T, Construct<T, U>>> for Construct<T, U> where T: Algebra + Clone, U: Algebra<T> + Clone {
    type Output = Construct<T, Construct<T, U>>;
    fn add(self, other: Construct<T, Construct<T, U>>) -> Self::Output {
        Self::Output::new(self + other.re(), other.im())
    }
}
impl<T, U> Sub<Construct<T, Construct<T, U>>> for Construct<T, U> where T: Algebra + Clone, U: Algebra<T> + Clone {
    type Output = Construct<T, Construct<T, U>>;
    fn sub(self, other: Construct<T, Construct<T, U>>) -> Self::Output {
        Self::Output::new(self - other.re(), other.im())
    }
}

impl<T, U> Mul<Construct<T, U>> for Construct<T, Construct<T, U>> where T: Algebra + Clone, U: Algebra<T> + Clone {
    type Output = Self;
    fn mul(self, other: Construct<T, U>) -> Self::Output {
        Self::new(self.re() * other.clone(), self.im() * other)
    }
}
impl<T, U> Div<Construct<T, U>> for Construct<T, Construct<T, U>> where T: Algebra + Clone, U: Algebra<T> + Clone {
    type Output = Self;
    fn div(self, other: Construct<T, U>) -> Self::Output {
        Self::new(self.re() / other.clone(), self.im() / other)
    }
}
impl<T, U> Mul<Construct<T, Construct<T, U>>> for Construct<T, U> where T: Algebra + Clone, U: Algebra<T> + Clone {
    type Output = Construct<T, Construct<T, U>>;
    fn mul(self, other: Construct<T, Construct<T, U>>) -> Self::Output {
        Self::Output::new(self.clone() * other.re(), self * other.im())
    }
}
impl<T, U> Div<Construct<T, Construct<T, U>>> for Construct<T, U> where T: Algebra + Clone, U: Algebra<T> + Clone {
    type Output = Construct<T, Construct<T, U>>;
    fn div(self, other: Construct<T, Construct<T, U>>) -> Self::Output {
        Self::Output::new(self.clone() / other.re(), self / other.im())
    }
}

impl<T, U> AddAssign<Construct<T, U>> for Construct<T, Construct<T, U>> where T: Algebra + Clone, U: Algebra<T> + Clone {
    fn add_assign(&mut self, other: Construct<T, U>) -> () {
        *self = self.clone() + other;
    }
}
impl<T, U> SubAssign<Construct<T, U>> for Construct<T, Construct<T, U>> where T: Algebra + Clone, U: Algebra<T> + Clone {
    fn sub_assign(&mut self, other: Construct<T, U>) -> () {
        *self = self.clone() - other;
    }
}
impl<T, U> MulAssign<Construct<T, U>> for Construct<T, Construct<T, U>> where T: Algebra + Clone, U: Algebra<T> + Clone {
    fn mul_assign(&mut self, other: Construct<T, U>) -> () {
        *self = self.clone() * other;
    }
}
impl<T, U> DivAssign<Construct<T, U>> for Construct<T, Construct<T, U>> where T: Algebra + Clone, U: Algebra<T> + Clone {
    fn div_assign(&mut self, other: Construct<T, U>) -> () {
        *self = self.clone() / other;
    }
}
