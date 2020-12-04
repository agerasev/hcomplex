use num_traits::{One, Zero};
use super::*;


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
