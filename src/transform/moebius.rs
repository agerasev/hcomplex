use core::ops::{Add, Mul, Div};
use num_traits::{Zero, One};
use vecmat::matrix::{Matrix2x2, Dot};
use super::*;
use crate::{Construct, Algebra};


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Moebius<U> {
    data: Matrix2x2<U>,
}

impl<U> From<[[U; 2]; 2]> for Moebius<U> {
    fn from(array: [[U; 2]; 2]) -> Self {
        Self { data: Matrix2x2::from(array) }
    }
}
impl<U> Into<[[U; 2]; 2]> for Moebius<U> {
    fn into(self) -> [[U; 2]; 2] {
        self.data.into()
    }
}
impl<U> From<Matrix2x2<U>> for Moebius<U> {
    fn from(data: Matrix2x2<U>) -> Self {
        Self { data }
    }
}
impl<U> Into<Matrix2x2<U>> for Moebius<U> {
    fn into(self) -> Matrix2x2<U> {
        self.data
    }
}

impl<U> Moebius<U> {
    pub fn new(a: U, b: U, c: U, d: U) -> Self {
        Self::from([[a, b], [c, d]])
    }

    pub fn a_ref(&self) -> &U { &self.data[(0, 0)] }
    pub fn b_ref(&self) -> &U { &self.data[(0, 1)] }
    pub fn c_ref(&self) -> &U { &self.data[(1, 0)] }
    pub fn d_ref(&self) -> &U { &self.data[(1, 1)] }

    pub fn a_mut(&mut self) -> &mut U { &mut self.data[(0, 0)] }
    pub fn b_mut(&mut self) -> &mut U { &mut self.data[(0, 1)] }
    pub fn c_mut(&mut self) -> &mut U { &mut self.data[(1, 0)] }
    pub fn d_mut(&mut self) -> &mut U { &mut self.data[(1, 1)] }
}

impl<U: Clone> Moebius<U> {
    pub fn a(&self) -> U { self.data[(0, 0)].clone() }
    pub fn b(&self) -> U { self.data[(0, 1)].clone() }
    pub fn c(&self) -> U { self.data[(1, 0)].clone() }
    pub fn d(&self) -> U { self.data[(1, 1)].clone() }
}

impl<U: Zero + One> Identity for Moebius<U> {
    fn identity() -> Self {
        Self::from(Matrix2x2::one())
    }
}

impl<U> Chain<U> for Moebius<U> where U: Add<Output=U> + Mul<Output=U> + Div<Output=U> + Clone {
    fn chain(self, other: Self) -> Self {
        Self::from(Dot::dot(self.data, other.data))
    }
}

impl<U> Transform<U> for Moebius<U> where U: Add<Output=U> + Mul<Output=U> + Div<Output=U> + Clone {
    fn apply(&self, x: U) -> U {
        (self.a()*x.clone() + self.b())/(self.c()*x + self.d())
    }
}
impl<T: Algebra + Clone, U: Algebra<T> + Clone> Transform<Construct<T, Construct<T, U>>> for Moebius<Construct<T, U>> {
    fn apply(&self, x: Construct<T, Construct<T, U>>) -> Construct<T, Construct<T, U>> {
        (self.a()*x.clone() + self.b())/(self.c()*x + self.d())
    }
}
