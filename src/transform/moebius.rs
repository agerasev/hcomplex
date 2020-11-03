use num_traits::{Float};

use std::marker::PhantomData;

use crate::{Algebra};


pub trait Transform<T: Float, A: Algebra<T>> {
    fn apply(&self, x: A) -> A;
}

pub trait Chain<T: Float, A: Algebra<T>> {
    fn chain(&self, other: &Self) -> Self;
}

pub mod prelude {
    pub use crate::transform::{Transform, Chain};
}

#[derive(Clone, Debug)]
pub struct Moebius<T: Float, A: Algebra<T>> {
    pub a: A,
    pub b: A,
    pub c: A,
    pub d: A,
    pd: PhantomData<T>,
}

impl<T: Float, A: Algebra<T>> Moebius<T, A> {
    pub fn new(a: A, b: A, c: A, d: A) -> Self {
        Self { a, b, c, d, pd: PhantomData }
    }
}

impl<T: Float, A: Algebra<T>> Chain<T, A> for Moebius<T, A> {
    fn chain(&self, other: &Self) -> Self {
        Self::new(
            self.a*other.a + self.b*other.c,
            self.a*other.b + self.b*other.d,
            self.c*other.a + self.d*other.c,
            self.c*other.b + self.d*other.d,
        )
    }
}

impl<T: Float, A: Algebra<T>> Transform<T, A> for Moebius<T, A> {
    fn apply(&self, x: A) -> A {
        (self.a*x + self.b)/(self.c*x + self.d)
    }
}
