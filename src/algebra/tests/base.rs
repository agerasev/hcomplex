use crate::*;
use num_traits::{Zero, One};
use ::approx::*;


#[test]
fn new() {
    let c = Complex::<f32>::new(1.0, 2.0);
    assert_abs_diff_eq!(c, Complex::new(1.0, 2.0));
}

#[test]
fn conj2() {
    let c = Complex::<f32>::new(1.0, 2.0).conj();
    assert_abs_diff_eq!(c, Complex::new(1.0, -2.0));
}

#[test]
fn conj4() {
    let c = Quaternion::<f32>::new2(1.0, 2.0, 3.0, 4.0).conj();
    assert_abs_diff_eq!(c, Quaternion::new2(1.0, -2.0, -3.0, -4.0));
}

#[test]
fn add2() {
    let a = Complex::<f32>::new(1.0, 2.0);
    let b = Complex::<f32>::new(3.0, 4.0);
    let c = a + b;
    assert_abs_diff_eq!(c, Complex::new(4.0, 6.0));

    let mut d = a;
    d += b;
    assert_abs_diff_eq!(d, Complex::new(4.0, 6.0));
}

#[test]
fn sub2() {
    let a = Complex::<f32>::new(2.0, 1.0);
    let b = Complex::<f32>::new(3.0, 4.0);
    let c = a - b;
    assert_abs_diff_eq!(c, Complex::new(-1.0, -3.0));

    let mut d = a;
    d -= b;
    assert_abs_diff_eq!(d, Complex::new(-1.0, -3.0));
}

#[test]
fn add4() {
    let a = Quaternion::<f32>::new2(1.0, 2.0, 3.0, 4.0);
    let b = Quaternion::<f32>::new2(5.0, 6.0, 7.0, 8.0);
    let c = a + b;
    assert_abs_diff_eq!(c, Quaternion::new2(6.0, 8.0, 10.0, 12.0));
}

#[test]
fn sub4() {
    let a = Quaternion::<f32>::new2(4.0, 3.0, 2.0, 1.0);
    let b = Quaternion::<f32>::new2(5.0, 6.0, 7.0, 8.0);
    let c = a - b;
    assert_abs_diff_eq!(c, Quaternion::new2(-1.0, -3.0, -5.0, -7.0));
}

#[test]
fn abs2() {
    let c = Complex::<f32>::new(1.0, 2.0);
    assert_abs_diff_eq!(c.abs_sqr(), 5.0);
    //assert_abs_diff_eq!(c.abs(), 5.0.sqrt());
}

#[test]
fn abs4() {
    let q = Quaternion::<f32>::new2(1.0, 2.0, 3.0, 4.0);
    assert_abs_diff_eq!(q.abs_sqr(), 30.0);
    //assert_abs_diff_eq!(q.abs(), 30.0.sqrt());
}

#[test]
fn mul2() {
    let a = Complex::<f32>::new(1.0, 2.0);
    let b = Complex::<f32>::new(3.0, 4.0);
    let c = a*b;
    assert_abs_diff_eq!(c, Complex::new(-5.0, 10.0));
    let d = b*a;
    assert_abs_diff_eq!(d, Complex::new(-5.0, 10.0));

    let mut e = a;
    e *= b;
    assert_abs_diff_eq!(e, Complex::new(-5.0, 10.0));
}

#[test]
fn smul2() {
    let c = Complex::<f32>::new(1.0, 2.0);
    let f = 2.0;
    let tf = |a: Complex<f32>| {
        assert_abs_diff_eq!(a, Complex::new(2.0, 4.0));
    };
    tf(c*f);
    tf(f*c);
    tf(Complex::<f32>::new(f, 0.0)*c);

    let mut d = c;
    d *= f;
    tf(d);
}

#[test]
fn mul4() {
    let a = Quaternion::<f32>::new2(1.0, 2.0, 3.0, 4.0);
    let b = Quaternion::<f32>::new2(5.0, 6.0, 7.0, 8.0);
    let c = a*b;
    assert_abs_diff_eq!(c, Quaternion::new2(-60.0, 12.0, 30.0, 24.0));
    let d = b*a;
    assert_abs_diff_eq!(d, Quaternion::new2(-60.0, 20.0, 14.0, 32.0));
}

#[test]
fn smul4() {
    let q = Quaternion::<f32>::new2(1.0, 2.0, 3.0, 4.0);
    let tf = |a: Quaternion<f32>| {
        assert_abs_diff_eq!(a, Quaternion::new2(2.0, 4.0, 6.0, 8.0));
    };
    tf(q*2.0);
    tf(2.0*q);
    tf(Quaternion::<f32>::new2(2.0, 0.0, 0.0, 0.0)*q);
}

#[test]
fn inv2() {
    let a = 1.0/Complex::<f32>::new(1.0, 2.0);
    assert_abs_diff_eq!(a, Complex::new(1.0/5.0, -2.0/5.0));
}

#[test]
fn div2() {
    let a = Complex::<f32>::new(3.0, 4.0);
    let b = Complex::<f32>::new(1.0, 2.0);
    let c = a/b;
    assert_abs_diff_eq!(c, Complex::new(11.0/5.0, -2.0/5.0));

    let mut d = a;
    d /= b;
    assert_abs_diff_eq!(d, Complex::new(11.0/5.0, -2.0/5.0));
}

#[test]
fn sdiv2() {
    let c = Complex::<f32>::new(1.0, 2.0);
    let f = 2.0;
    let tf = |a: Complex<f32>| {
        assert_abs_diff_eq!(a, Complex::new(0.5, 1.0));
    };
    tf(c/f);
    tf(c/Complex::<f32>::new(f, 0.0));

    let mut d = c;
    d /= f;
    tf(d);
}

#[test]
fn zero2() {
    let a = Complex::<f32>::zero();
    assert_abs_diff_eq!(a, Complex::new(0.0, 0.0));
}

#[test]
fn zero4() {
    let a = Quaternion::<f32>::zero();
    assert_abs_diff_eq!(a, Quaternion::new2(0.0, 0.0, 0.0, 0.0));
}

#[test]
fn one2() {
    let a = Complex::<f32>::one();
    assert_abs_diff_eq!(a, Complex::new(1.0, 0.0));
}

#[test]
fn one4() {
    let a = Quaternion::<f32>::one();
    assert_abs_diff_eq!(a, Quaternion::new2(1.0, 0.0, 0.0, 0.0));
}
