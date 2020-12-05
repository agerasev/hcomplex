use super::*;
use num_traits::{Float};
use num_complex::{Complex as NumComplex};


impl<T: Float> Complex<T> {
    /// Convert into `num_complex::Complex` struct.
    pub fn into_num(self) -> NumComplex<T> {
        Into::<NumComplex<_>>::into(self)
    }
    /// Calculate the principal Arg of self.
    pub fn arg(self) -> T {
        self.into_num().arg()
    }
    /// Computes `e^(self)`, where `e` is the base of the natural logarithm.
    pub fn exp(self) -> Self {
        self.into_num().exp().into()
    }
    /// Computes the principal value of natural logarithm of `self`.
    pub fn ln(self) -> Self {
        self.into_num().ln().into()
    }
    /// Computes the principal value of the square root of `self`.
    pub fn sqrt(self) -> Self {
        self.into_num().sqrt().into()
    }
    /// Computes the principal value of the cube root of `self`.
    ///
    /// Note that this does not match the usual result for the cube root of negative real numbers.
    /// For example, the real cube root of `-8` is `-2`, but the principal complex cube root of `-8` is `1 + i√3`.
    pub fn cbrt(self) -> Self {
        self.into_num().cbrt().into()
    }
    /// Raises `self` to an unsigned integer power.
    pub fn powu(self, exp: u32) -> Self {
        self.into_num().powu(exp).into()
    }
    /// Raises `self` to a signed integer power.
    pub fn powi(self, exp: i32) -> Self {
        self.into_num().powi(exp).into()
    }
    /// Raises `self` to a floating point power.
    pub fn powf(self, exp: T) -> Self {
        self.into_num().powf(exp).into()
    }
    /// Raises `self` to a complex power.
    pub fn powc(self, exp: Self) -> Self {
        self.into_num().powc(exp.into_num()).into()
    }
    /// Returns the logarithm of `self` with respect to an arbitrary base.
    pub fn log(self, base: T) -> Self {
        self.into_num().log(base).into()
    }
    /// Raises a floating point number to the complex power `self`.
    pub fn expf(self, base: T) -> Self {
        self.into_num().expf(base).into()
    }
    /// Computes the sine of `self`.
    pub fn sin(self) -> Self {
        self.into_num().sin().into()
    }
    /// Computes the cosine of `self`.
    pub fn cos(self) -> Self {
        self.into_num().cos().into()
    }
    /// Computes the tangent of `self`.
    pub fn tan(self) -> Self {
        self.into_num().tan().into()
    }
    /// Computes the principal value of the inverse sine of `self`.
    pub fn asin(self) -> Self {
        self.into_num().asin().into()
    }
    /// Computes the principal value of the inverse cosine of `self`.
    pub fn acos(self) -> Self {
        self.into_num().acos().into()
    }
    /// Computes the principal value of the inverse tangent of `self`.
    pub fn atan(self) -> Self {
        self.into_num().atan().into()
    }
    /// Computes the hyperbolic sine of `self`.
    pub fn sinh(self) -> Self {
        self.into_num().sinh().into()
    }
    /// Computes the hyperbolic cosine of `self`.
    pub fn cosh(self) -> Self {
        self.into_num().cosh().into()
    }
    /// Computes the hyperbolic tangent of `self`.
    pub fn tanh(self) -> Self {
        self.into_num().tanh().into()
    }
    /// Computes the principal value of the inverse hyperbolic sine of `self`.
    pub fn asinh(self) -> Self {
        self.into_num().asinh().into()
    }
    /// Computes the principal value of the inverse hyperbolic cosine of `self`.
    pub fn acosh(self) -> Self {
        self.into_num().acosh().into()
    }
    /// Computes the principal value of the inverse hyperbolic tangent of `self`.
    pub fn atanh(self) -> Self {
        self.into_num().atanh().into()
    }
    /// Returns `1/self` using floating-point operations.
    pub fn finv(self) -> Self {
        self.into_num().finv().into()
    }
}
impl<T: Float + Clone> Complex<T> where Self: Norm<Output=T> {
    /// Convert to polar form.
    pub fn to_polar(self) -> (T, T) {
        (self.norm(), self.arg())
    }
    /// Convert a polar representation into a complex number.
    pub fn from_polar(r: T, theta: T) -> Self {
        Self::new(T::zero(), theta).exp() * r
    }
}
