use core::ops::{Add};
use super::traits::{Norm, NormL1};
use vecmat::{Dot, NormL1 as VNormL1, NormL2 as VNormL2};
use super::*;


impl<T, U> Dot for Construct<T, U> where T: Add<Output=T>, U: Dot<Output=T> {
    type Output = T;
    fn dot(self, other: Self) -> T {
        let (l, r) = (self.split(), other.split());
        l.0.dot(r.0) + l.1.dot(r.1)
    }
}

impl<T, U> VNormL1 for Construct<T, U> where Self: NormL1<Output=T> {
    type Output = T;
    fn norm_l1(self) -> T {
        NormL1::norm_l1(self)
    }
}
impl<T, U> VNormL2 for Construct<T, U> where Self: Norm<Output=T> {
    type Output = T;
    fn norm_l2(self) -> T {
        self.norm()
    }
}
