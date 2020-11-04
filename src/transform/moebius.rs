use std::{
    ops::{Index, IndexMut},
    marker::PhantomData,
};
use num_traits::{One};
use vecmat::matrix::{Matrix2x2, Dot};
use super::*;
use crate::{Construct};


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Moebius<T: Algebra, U: Algebra<T>> {
    data: Matrix2x2<U>,
    pd: PhantomData<T>,
}

impl<T: Algebra, U: Algebra<T>> From<[[U; 2]; 2]> for Moebius<T, U> {
    fn from(array: [[U; 2]; 2]) -> Self {
        Self { data: Matrix2x2::from(array), pd: PhantomData }
    }
}
impl<T: Algebra, U: Algebra<T>> Into<[[U; 2]; 2]> for Moebius<T, U> {
    fn into(self) -> [[U; 2]; 2] {
        self.data.into()
    }
}
impl<T: Algebra, U: Algebra<T>> From<Matrix2x2<U>> for Moebius<T, U> {
    fn from(data: Matrix2x2<U>) -> Self {
        Self { data, pd: PhantomData }
    }
}
impl<T: Algebra, U: Algebra<T>> Into<Matrix2x2<U>> for Moebius<T, U> {
    fn into(self) -> Matrix2x2<U> {
        self.data
    }
}

impl<T: Algebra, U: Algebra<T>> Index<usize> for Moebius<T, U> {
    type Output = U;
    fn index(&self, pos: usize) -> &U {
        &self.data[(pos / 2, pos % 2)]
    }
}
impl<T: Algebra, U: Algebra<T>> IndexMut<usize> for Moebius<T, U> {
    fn index_mut(&mut self, pos: usize) -> &mut U {
        &mut self.data[(pos / 2, pos % 2)]
    }
}

impl<T: Algebra, U: Algebra<T>> Moebius<T, U> {
    pub fn new(a: U, b: U, c: U, d: U) -> Self {
        Self::from([[a, b], [c, d]])
    }
}

impl<T: Algebra, U: Algebra<T> + Copy> Identity for Moebius<T, U> {
    fn identity() -> Self {
        Self::from(Matrix2x2::one())
    }
}

impl<T: Algebra, U: Algebra<T> + Copy> Chain<T, U> for Moebius<T, U> {
    fn chain(&self, other: &Self) -> Self {
        Self::from(self.data.dot(other.data))
    }
}

impl<T: Algebra, U: Algebra<T> + Copy> Transform<T, U> for Moebius<T, U> {
    fn apply(&self, x: U) -> U {
        (self[0]*x + self[1])/(self[2]*x + self[3])
    }
}
impl<T: Algebra + Copy, U: Algebra<T> + Copy> Transform<T, Construct<T, Construct<T, U>>> for Moebius<T, Construct<T, U>> {
    fn apply(&self, x: Construct<T, Construct<T, U>>) -> Construct<T, Construct<T, U>> {
        (self[0]*x + self[1])/(self[2]*x + self[3])
    }
}
