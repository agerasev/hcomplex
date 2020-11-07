use core::ops::{Neg, Add, Mul, Div};
use num_traits::{Zero, One, Num, NumCast};
use vecmat::matrix::{Matrix2x2, Dot};
use crate::{*, transform::*};


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

impl<U: Neg<Output=U> + Num + Clone> Moebius<U> {
    pub fn det(&self) -> U {
        self.data.det()
    }
    pub fn normalize(self) -> Self {
        let det = self.det();
        (self.data / det).into()
    }
}

impl<T: Algebra + Clone> Deriv<Complex<T>> for Moebius<Complex<T>> {
    fn deriv(&self, p: Complex<T>) -> Complex<T> {
        let u: Complex<T> = self.a() * p.clone() + self.b();
        let d: Complex<T> = self.c() * p + self.d();
        return (self.a() * d.clone() - u * self.c()) / (d.clone() * d);
    }
}

impl<T: NumCast + Algebra + Dot<Output=T> + Clone> DerivDir<Quaternion<T>> for Moebius<Complex<T>> {
    fn deriv_dir(&self, p: Quaternion<T>, v: Quaternion<T>) -> Quaternion<T> {
        let u = self.a() * p.clone() + self.b();
        let d = self.c() * p + self.d();
        let d2 = d.clone().abs_sqr();
        let g1 = (self.a() * v.clone()) / d.clone();
        let g21 = (self.c() * v.clone()).conj();
        let g22 = d.clone().conj() * (d.dot(self.c() * v) * T::from(2).unwrap() / d2.clone());
        let g2 = u * ((g21 - g22) / d2);
        return g1 + g2;
    }
}
