use super::*;
use crate::{Complex, Quaternion, Octonion};

use rand::{prelude::*, Rng};
use rand_distr::StandardNormal;
use rand_xorshift::XorShiftRng;

use approx::*;

const TRANSFORM_ATTEMPTS: usize = 64;
const POINT_ATTEMPTS: usize = 16;


struct TestRng {
    base: XorShiftRng,
}
impl TestRng {
    fn new() -> Self {
        Self { base: XorShiftRng::seed_from_u64(0xdeadbeef) }
    }
    fn sample(&mut self) -> f64 {
        self.base.sample(StandardNormal)
    }
}

trait TestRand {
    fn random(rng: &mut TestRng) -> Self;
}
impl TestRand for f64 {
    fn random(rng: &mut TestRng) -> Self {
        rng.sample()
    }
}
impl TestRand for Complex<f64> {
    fn random(rng: &mut TestRng) -> Self {
        Self::new(f64::random(rng), f64::random(rng))
    }
}
impl TestRand for Quaternion<f64> {
    fn random(rng: &mut TestRng) -> Self {
        Self::new(Complex::random(rng), Complex::random(rng))
    }
}
impl TestRand for Octonion<f64> {
    fn random(rng: &mut TestRng) -> Self {
        Self::new(Quaternion::random(rng), Quaternion::random(rng))
    }
}
impl TestRand for Moebius<Complex<f64>> {
    fn random(rng: &mut TestRng) -> Self {
        Self::new(
            Complex::random(rng),
            Complex::random(rng),
            Complex::random(rng),
            Complex::random(rng),
        )
    }
}
impl TestRand for Moebius<Quaternion<f64>> {
    fn random(rng: &mut TestRng) -> Self {
        Self::new(
            Quaternion::random(rng),
            Quaternion::random(rng),
            Quaternion::random(rng),
            Quaternion::random(rng),
        )
    }
}
impl TestRand for Moebius<Octonion<f64>> {
    fn random(rng: &mut TestRng) -> Self {
        Self::new(
            Octonion::random(rng),
            Octonion::random(rng),
            Octonion::random(rng),
            Octonion::random(rng),
        )
    }
}

#[test]
fn moebius2() {
    let mut rng = TestRng::new();
    for _ in 0..TRANSFORM_ATTEMPTS {
        let a = Moebius::random(&mut rng);
        let b = Moebius::random(&mut rng);
        let c = a.chain(b);
        for _ in 0..POINT_ATTEMPTS {
            let x = Complex::random(&mut rng);
            let y = a.apply(b.apply(x));
            let z = c.apply(x);
            assert_abs_diff_eq!(y, z, epsilon=1e-12);
        }
    }
}

#[test]
fn moebius4() {
    let mut rng = TestRng::new();
    for _ in 0..TRANSFORM_ATTEMPTS {
        let a = Moebius::<Quaternion<_>>::random(&mut rng);
        let b = Moebius::<Quaternion<_>>::random(&mut rng);
        let c = a.chain(b);
        for _ in 0..POINT_ATTEMPTS {
            let x = Quaternion::random(&mut rng);
            let y = a.apply(b.apply(x));
            let z = c.apply(x);
            assert_abs_diff_eq!(y, z, epsilon=1e-12);
        }
    }
}

/// Moebuis transform over octonions isn't chainable and therefore should fail
#[test]
#[should_panic]
fn moebius8() {
    let mut rng = TestRng::new();
    for _ in 0..TRANSFORM_ATTEMPTS {
        let a = Moebius::<Octonion<_>>::random(&mut rng);
        let b = Moebius::<Octonion<_>>::random(&mut rng);
        let c = a.chain(b);
        for _ in 0..POINT_ATTEMPTS {
            let x = Octonion::random(&mut rng);
            let y = a.apply(b.apply(x));
            let z = c.apply(x);
            assert_abs_diff_eq!(y, z, epsilon=1e-12);
        }
    }
}
