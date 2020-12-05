use num_traits::{Float};
use rand::Rng;
use rand_distr::Distribution;
use crate::*;
use super::*;


pub use rand_distr::StandardNormal;

/// Distribution that produces normalized Moebius transformation, i.e. `det() == 1`.
pub struct Normalized;


impl<U> Distribution<Moebius<U>> for StandardNormal where StandardNormal: Distribution<U> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Moebius<U> {
        Moebius::new(
            rng.sample(Self), rng.sample(Self),
            rng.sample(Self), rng.sample(Self),
        )
    }
}

impl<T: Float + Algebra, U: NormSqr<Output=T> + Clone> Distribution<Moebius<Construct<T, U>>> for Normalized where
    StandardNormal: Distribution<Moebius<Construct<T, U>>>,
    Construct<T, U>: Algebra<T>
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Moebius<Construct<T, U>> {
        loop {
            let m = rng.sample(&StandardNormal);
            if m.det().norm() > T::epsilon() {
                break m.normalize();
            }
        }
    }
}
