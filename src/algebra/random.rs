use core::ops::{Div};
use num_traits::{Float};
use rand::{Rng};
use rand_distr::{Distribution, StandardNormal};
use super::*;


pub trait Random<T> where StandardNormal: Distribution<T> {
    fn sample_snd<R: Rng + ?Sized>(rng: &mut R, snd: &StandardNormal) -> Self;
}
impl<T> Random<T> for T where StandardNormal: Distribution<T> {
    fn sample_snd<R: Rng + ?Sized>(rng: &mut R, snd: &StandardNormal) -> Self {
        rng.sample(snd)
    }
}
impl<T, U: Random<T>> Random<T> for Construct<T, U> where StandardNormal: Distribution<T> {
    fn sample_snd<R: Rng + ?Sized>(rng: &mut R, snd: &StandardNormal) -> Self {
        Construct::new(U::sample_snd(rng, snd), U::sample_snd(rng, snd))
    }
}

pub struct Normal;
pub struct NonZero;
pub struct Unit;

impl<T, U: Random<T>> Distribution<Construct<T, U>> for Normal where StandardNormal: Distribution<T> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Construct<T, U> {
        Construct::sample_snd(rng, &StandardNormal)
    }
}
impl<T: Float, U: Random<T> + NormSqr<Output=T> + Clone> Distribution<Construct<T, U>> for NonZero where StandardNormal: Distribution<T> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Construct<T, U> {
        loop {
            let x = rng.sample(&Normal);
            if x.clone().norm() > T::epsilon() {
                break x;
            }
        }
    }
}
impl<T: Float, U: Random<T> + NormSqr<Output=T> + Div<T, Output=U> + Clone> Distribution<Construct<T, U>> for Unit where StandardNormal: Distribution<T> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Construct<T, U> {
        rng.sample(&NonZero).normalize()
    }
}
