use super::{traits::*, construct::*};
use approx::*;


impl<T: Algebra + Copy, U: Algebra<T> + Copy> AbsDiffEq for Construct<T, U> where T: AbsDiffEq<Epsilon=T>, U: AbsDiffEq<Epsilon=T> {
    type Epsilon = T;
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        abs_diff_eq!(self.re(), other.re(), epsilon=epsilon) &&
        abs_diff_eq!(self.im(), other.im(), epsilon=epsilon)
    }
}

impl<T: Algebra + Copy, U: Algebra<T> + Copy> RelativeEq for Construct<T, U> where T: RelativeEq<Epsilon=T>, U: RelativeEq<Epsilon=T> {
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }
    fn relative_eq(&self, other: &Self, epsilon: Self::Epsilon, max_relative: Self::Epsilon) -> bool {
        relative_eq!(self.re(), other.re(), epsilon=epsilon, max_relative=max_relative) &&
        relative_eq!(self.im(), other.im(), epsilon=epsilon, max_relative=max_relative)
    }
}

impl<T: Algebra + Copy, U: Algebra<T> + Copy> UlpsEq for Construct<T, U> where T: UlpsEq<Epsilon=T>, U: UlpsEq<Epsilon=T> {
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        ulps_eq!(self.re(), other.re(), epsilon=epsilon, max_ulps=max_ulps) &&
        ulps_eq!(self.im(), other.im(), epsilon=epsilon, max_ulps=max_ulps)
    }
}
