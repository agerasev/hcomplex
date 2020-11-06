use num_traits::{One, Zero};
use super::*;

#[cfg(feature = "vecmat")]
use vecmat::vector::*;


impl<T: One + Zero> Complex<T> {
    pub fn i() -> Self {
        Self::new(T::zero(), T::one())
    }
}

impl<T: One + Zero> Quaternion<T> {
    pub fn i() -> Self {
        Self::new2(T::zero(), T::one(), T::zero(), T::zero())
    }
    pub fn j() -> Self {
        Self::new2(T::zero(), T::zero(), T::one(), T::zero())
    }
    pub fn k() -> Self {
        Self::new2(T::zero(), T::zero(), T::zero(), T::one())
    }
}

#[cfg(feature = "vecmat")]
impl<T> Complex<T> {
    pub fn into_vector(self) -> Vector2<T> {
        let (re, im) = self.split();
        Vector2::from([re, im])
    }
}

#[cfg(feature = "vecmat")]
impl<T> Quaternion<T> {
    pub fn into_vector(self) -> Vector4<T> {
        let (re, im) = self.split();
        let ((w, x), (y, z)) = (re.split(), im.split());
        Vector4::from([w, x, y, z])
    }
}
