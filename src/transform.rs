use num_traits::{Float};

use std::marker::PhantomData;

use crate::traits::{Algebra};


pub trait Transform<T: Float, A: Algebra<T>> {
    fn apply(&self, x: A) -> A;
}

pub trait Chain<T: Float, A: Algebra<T>> {
    fn chain(&self, other: &Self) -> Self;
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Complex, Quaternion, Octonion, Sedenion};

    use rand::{prelude::*, Rng};
    use rand::distributions::StandardNormal;
    use rand_xorshift::XorShiftRng;

    use assert_approx_eq::assert_approx_eq;


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
            Self::new2(f64::random(rng), f64::random(rng))
        }
    }
    impl TestRand for Quaternion<f64> {
        fn random(rng: &mut TestRng) -> Self {
            Self::new2(Complex::random(rng), Complex::random(rng))
        }
    }
    impl TestRand for Octonion<f64> {
        fn random(rng: &mut TestRng) -> Self {
            Self::new2(Quaternion::random(rng), Quaternion::random(rng))
        }
    }
    impl TestRand for Moebius<f64, Complex<f64>> {
        fn random(rng: &mut TestRng) -> Self {
            Self::new(
                Complex::random(rng),
                Complex::random(rng),
                Complex::random(rng),
                Complex::random(rng),
            )
        }
    }
    impl TestRand for Moebius<f64, Quaternion<f64>> {
        fn random(rng: &mut TestRng) -> Self {
            Self::new(
                Quaternion::random(rng),
                Quaternion::random(rng),
                Quaternion::random(rng),
                Quaternion::random(rng),
            )
        }
    }
    impl TestRand for Moebius<f64, Octonion<f64>> {
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
    fn moebius2_chain() {
        let mut rng = TestRng::new();
        for _ in 0..10 {
            let a = Moebius::random(&mut rng);
            let b = Moebius::random(&mut rng);
            let c = a.chain(&b);
            for _ in 0..10 {
                let x = Complex::random(&mut rng);
                let y = a.apply(b.apply(x));
                let z = c.apply(x);
                assert_approx_eq!(y.re(), z.re());
                assert_approx_eq!(y.im(), z.im());
            }
        }
    }

    #[test]
    fn moebius4_chain() {
        let mut rng = TestRng::new();
        for _ in 0..10 {
            let a = Moebius::random(&mut rng);
            let b = Moebius::random(&mut rng);
            let c = a.chain(&b);
            for _ in 0..10 {
                let x = Quaternion::random(&mut rng);
                let y = a.apply(b.apply(x));
                let z = c.apply(x);
                assert_approx_eq!(y.w(), z.w());
                assert_approx_eq!(y.x(), z.x());
                assert_approx_eq!(y.y(), z.y());
                assert_approx_eq!(y.z(), z.z());
            }
        }
    }

    /// Moebuis transform over octonions isn't chainable and therefore should panic
    #[test]
    #[should_panic]
    fn moebius8_chain() {
        let mut rng = TestRng::new();
        for _ in 0..10 {
            let a = Moebius::random(&mut rng);
            let b = Moebius::random(&mut rng);
            let c = a.chain(&b);
            for _ in 0..10 {
                let x = Octonion::random(&mut rng);
                let y = a.apply(b.apply(x));
                let z = c.apply(x);
                assert_approx_eq!(((y.0).0).0, ((z.0).0).0);
                assert_approx_eq!(((y.0).0).1, ((z.0).0).1);
                assert_approx_eq!(((y.0).1).0, ((z.0).1).0);
                assert_approx_eq!(((y.0).1).1, ((z.0).1).1);
                assert_approx_eq!(((y.1).0).0, ((z.1).0).0);
                assert_approx_eq!(((y.1).0).1, ((z.1).0).1);
                assert_approx_eq!(((y.1).1).0, ((z.1).1).0);
                assert_approx_eq!(((y.1).1).1, ((z.1).1).1);
            }
        }
    }
}