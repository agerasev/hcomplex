use super::*;
use assert_approx_eq::assert_approx_eq;


#[test]
fn new2() {
    let c = Complex::<f32>::new2(1.0, 2.0);
    assert_approx_eq!(c.re(), 1.0);
    assert_approx_eq!(c.im(), 2.0);
}

#[test]
fn conj2() {
    let c = Complex::<f32>::new2(1.0, 2.0).conj();
    assert_approx_eq!(c.re(), 1.0);
    assert_approx_eq!(c.im(), -2.0);
}

#[test]
fn conj4() {
    let c = Quaternion::<f32>::new4(1.0, 2.0, 3.0, 4.0).conj();
    assert_approx_eq!(c.w(), 1.0);
    assert_approx_eq!(c.x(), -2.0);
    assert_approx_eq!(c.y(), -3.0);
    assert_approx_eq!(c.z(), -4.0);
}

#[test]
fn add2() {
    let a = Complex::<f32>::new2(1.0, 2.0);
    let b = Complex::<f32>::new2(3.0, 4.0);
    let c = a + b;
    assert_approx_eq!(c.re(), 4.0);
    assert_approx_eq!(c.im(), 6.0);
}

#[test]
fn sub2() {
    let a = Complex::<f32>::new2(2.0, 1.0);
    let b = Complex::<f32>::new2(3.0, 4.0);
    let c = a - b;
    assert_approx_eq!(c.re(), -1.0);
    assert_approx_eq!(c.im(), -3.0);
}

#[test]
fn add4() {
    let a = Quaternion::<f32>::new4(1.0, 2.0, 3.0, 4.0);
    let b = Quaternion::<f32>::new4(5.0, 6.0, 7.0, 8.0);
    let c = a + b;
    assert_approx_eq!(c.w(), 6.0);
    assert_approx_eq!(c.x(), 8.0);
    assert_approx_eq!(c.y(), 10.0);
    assert_approx_eq!(c.z(), 12.0);
}

#[test]
fn sub4() {
    let a = Quaternion::<f32>::new4(4.0, 3.0, 2.0, 1.0);
    let b = Quaternion::<f32>::new4(5.0, 6.0, 7.0, 8.0);
    let c = a - b;
    assert_approx_eq!(c.w(), -1.0);
    assert_approx_eq!(c.x(), -3.0);
    assert_approx_eq!(c.y(), -5.0);
    assert_approx_eq!(c.z(), -7.0);
}

#[test]
fn abs2() {
    let c = Complex::<f32>::new2(1.0, 2.0);
    assert_approx_eq!(c.sqr_abs(), 5.0);
    assert_approx_eq!(c.abs(), 5.0.sqrt());
}

#[test]
fn abs4() {
    let q = Quaternion::<f32>::new4(1.0, 2.0, 3.0, 4.0);
    assert_approx_eq!(q.sqr_abs(), 30.0);
    assert_approx_eq!(q.abs(), 30.0.sqrt());
}

#[test]
fn mul2() {
    let a = Complex::<f32>::new2(1.0, 2.0);
    let b = Complex::<f32>::new2(3.0, 4.0);
    let c = a*b;
    assert_approx_eq!(c.re(), -5.0);
    assert_approx_eq!(c.im(), 10.0);
    let d = b*a;
    assert_approx_eq!(d.re(), -5.0);
    assert_approx_eq!(d.im(), 10.0);
}

#[test]
fn smul2() {
    let c = Complex::<f32>::new2(1.0, 2.0);
    let tf = |a: Complex<f32>| {
        assert_approx_eq!(a.re(), 2.0);
        assert_approx_eq!(a.im(), 4.0);
    };
    tf(c*2.0);
    tf(2.0*c);
    tf(Complex::<f32>::new2(2.0, 0.0)*c);
}

#[test]
fn mul4() {
    let a = Quaternion::<f32>::new4(1.0, 2.0, 3.0, 4.0);
    let b = Quaternion::<f32>::new4(5.0, 6.0, 7.0, 8.0);
    let c = a*b;
    assert_approx_eq!(c.w(), -60.0);
    assert_approx_eq!(c.x(), 12.0);
    assert_approx_eq!(c.y(), 30.0);
    assert_approx_eq!(c.z(), 24.0);
    let d = b*a;
    assert_approx_eq!(d.w(), -60.0);
    assert_approx_eq!(d.x(), 20.0);
    assert_approx_eq!(d.y(), 14.0);
    assert_approx_eq!(d.z(), 32.0);
}

#[test]
fn smul4() {
    let q = Quaternion::<f32>::new4(1.0, 2.0, 3.0, 4.0);
    let tf = |a: Quaternion<f32>| {
        assert_approx_eq!(a.w(), 2.0);
        assert_approx_eq!(a.x(), 4.0);
        assert_approx_eq!(a.y(), 6.0);
        assert_approx_eq!(a.z(), 8.0);
    };
    tf(q*2.0);
    tf(2.0*q);
    tf(Quaternion::<f32>::new4(2.0, 0.0, 0.0, 0.0)*q);
}

#[test]
fn inv2() {
    let a = 1.0/Complex::<f32>::new2(1.0, 2.0);
    assert_approx_eq!(a.re(), 1.0/5.0);
    assert_approx_eq!(a.im(), -2.0/5.0);
}

#[test]
fn div2() {
    let a = Complex::<f32>::new2(3.0, 4.0);
    let b = Complex::<f32>::new2(1.0, 2.0);
    let c = a/b;
    assert_approx_eq!(c.re(), 11.0/5.0);
    assert_approx_eq!(c.im(), -2.0/5.0);
}
