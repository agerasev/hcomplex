use num_traits::{Float};

use std::marker::PhantomData;

use crate::traits::{Algebra};


pub trait Transform<T: Float, A: Algebra<T>> {
	fn apply(&self, x: A) -> A;
}

pub struct Moebius<T: Float, A: Algebra<T>> {
	pub a: A,
	pub b: A,
	pub c: A,
	pub d: A,
	pd: PhantomData<T>,
}

impl<T: Float, A: Algebra<T>> Transform<T, A> for Moebius<T, A> {
	fn apply(&self, x: A) -> A {
		(self.a*x + self.b)/(self.c*x + self.d)
	}
}

#[cfg(test)]
mod test {
	
}