use core::ops::{Neg, Add, Sub, Mul, Div};
use num_traits::{Zero, One, NumCast};
use crate::{*, transform::*};


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Moebius<U> {
    data: [U; 4],
}

impl<U> From<[U; 4]> for Moebius<U> {
    fn from(array: [U; 4]) -> Self {
        Self { data: array }
    }
}
impl<U> Into<[U; 4]> for Moebius<U> {
    fn into(self) -> [U; 4] {
        self.data
    }
}

impl<U> Moebius<U> {
    pub fn new(a: U, b: U, c: U, d: U) -> Self {
        Self::from([a, b, c, d])
    }

    pub fn a_ref(&self) -> &U { &self.data[0] }
    pub fn b_ref(&self) -> &U { &self.data[1] }
    pub fn c_ref(&self) -> &U { &self.data[2] }
    pub fn d_ref(&self) -> &U { &self.data[3] }

    pub fn a_mut(&mut self) -> &mut U { &mut self.data[0] }
    pub fn b_mut(&mut self) -> &mut U { &mut self.data[1] }
    pub fn c_mut(&mut self) -> &mut U { &mut self.data[2] }
    pub fn d_mut(&mut self) -> &mut U { &mut self.data[3] }
}

impl<U: Clone> Moebius<U> {
    pub fn a(&self) -> U { self.data[0].clone() }
    pub fn b(&self) -> U { self.data[1].clone() }
    pub fn c(&self) -> U { self.data[2].clone() }
    pub fn d(&self) -> U { self.data[3].clone() }
}

impl<U: Zero + One> Identity for Moebius<U> {
    fn identity() -> Self {
        Self::new(U::one(), U::zero(), U::zero(), U::one())
    }
}

impl<U> Chain<U> for Moebius<U> where U: Add<Output=U> + Mul<Output=U> + Div<Output=U> + Clone {
    fn chain(self, other: Self) -> Self {
        Self::new(
            self.a()*other.a() + self.b()*other.c(),
            self.a()*other.b() + self.b()*other.d(),
            self.c()*other.a() + self.d()*other.c(),
            self.c()*other.b() + self.d()*other.d(),
        )
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

impl<U: Neg<Output=U> + Mul<Output=U> + Div<Output=U> + Sub<Output=U> + Clone> Moebius<U> {
    pub fn det(&self) -> U {
        self.a()*self.d() - self.b()*self.c()
    }
    pub fn normalize(mut self) -> Self {
        let det = self.det();
        self.data.iter_mut().for_each(|x| *x = x.clone() / det.clone());
        self
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
