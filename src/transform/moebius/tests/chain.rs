use rand::{prelude::*};
use rand_xorshift::XorShiftRng;
use approx::*;
use crate::{transform::*, Complex, Quaternion, Octonion};


const TRANSFORM_ATTEMPTS: usize = 64;
const POINT_ATTEMPTS: usize = 16;

#[test]
fn moebius2() {
    let mut rng = XorShiftRng::seed_from_u64(0xDEAD0);
    for _ in 0..TRANSFORM_ATTEMPTS {
        let a: Moebius<Complex<f64>> = rng.sample(&Normalized);
        let b: Moebius<Complex<f64>> = rng.sample(&Normalized);
        let c = a.chain(b);
        for _ in 0..POINT_ATTEMPTS {
            let x: Complex<f64> = rng.sample(&StandardNormal);
            let y = a.apply(b.apply(x));
            let z = c.apply(x);
            assert_abs_diff_eq!(y, z, epsilon=1e-12);
        }
    }
}

#[test]
fn moebius4() {
    let mut rng = XorShiftRng::seed_from_u64(0xDEAD1);
    for _ in 0..TRANSFORM_ATTEMPTS {
        let a: Moebius<Quaternion<f64>> = rng.sample(&Normalized);
        let b: Moebius<Quaternion<f64>> = rng.sample(&Normalized);
        let c = a.chain(b);
        for _ in 0..POINT_ATTEMPTS {
            let x: Quaternion<f64> = rng.sample(&StandardNormal);
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
    let mut rng = XorShiftRng::seed_from_u64(0xDEAD2);
    for _ in 0..TRANSFORM_ATTEMPTS {
        let a: Moebius<Octonion<f64>> = rng.sample(&Normalized);
        let b: Moebius<Octonion<f64>> = rng.sample(&Normalized);
        let c = a.chain(b);
        for _ in 0..POINT_ATTEMPTS {
            let x: Octonion<f64> = rng.sample(&StandardNormal);
            let y = a.apply(b.apply(x));
            let z = c.apply(x);
            assert_abs_diff_eq!(y, z, epsilon=1e-12);
        }
    }
}
