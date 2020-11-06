use rand::{prelude::*};
use rand_distr::{Uniform};
use rand_xorshift::XorShiftRng;
use ::approx::*;
use crate::{*, random::*};

const SAMPLE_ATTEMPTS: usize = 256;


#[test]
fn constructor() {
    let a = Complex::new(0.0, 1.0);
    assert_abs_diff_eq!(a.re(), 0.0);
    assert_abs_diff_eq!(a.im(), 1.0);
}

#[test]
fn inversion() {
    let mut rng = XorShiftRng::seed_from_u64(0xCAFE0);
    for _ in 0..SAMPLE_ATTEMPTS {
        let a: Complex<f64> = rng.sample(NonZero);
        assert_abs_diff_eq!(a/a, Complex::new(1.0, 0.0));
    }
}

#[test]
fn square_root() {
    let mut rng = XorShiftRng::seed_from_u64(0xCAFE1);
    for _ in 0..SAMPLE_ATTEMPTS {
        let a: Complex<f64> = rng.sample(Normal);
        let b = a.sqrt();
        assert_abs_diff_eq!(b*b, a, epsilon=1e-12);
    }
}

#[test]
fn power() {
    let mut rng = XorShiftRng::seed_from_u64(0xCAFE1);
    for _ in 0..SAMPLE_ATTEMPTS {
        let a: Complex<f64> = rng.sample(Normal);
        let n = rng.sample(Uniform::from(2..12));
        let b = a.powf(1.0 / n as f64);
        let mut c = Complex::new(1.0, 0.0);
        for _ in 0..n {
            c *= b;
        }
        assert_abs_diff_eq!(c, a, epsilon=1e-12);
    }
}

#[test]
fn norm() {
    // TODO: Implement L1 norm.
    //assert_abs_diff_eq!(Complex::new(-1, 2).l1_norm(), 3.0);
    assert_abs_diff_eq!(Complex::new(3.0, -4.0).abs(), 5.0);
}
