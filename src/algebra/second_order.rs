use core::{
    ops::{
        Add, Sub, Mul, Div,
        AddAssign, SubAssign, MulAssign, DivAssign,
    },
};
pub use num_traits::{One, Zero, Inv};
use super::{construct::*};


impl<T, U> Construct<T, Construct<T, U>> {
    /// Create from four parts.
    pub fn new2(w: U, x: U, y: U, z: U) -> Self {
        Self::new(Construct::new(w, x), Construct::new(y, z))
    }
}

impl<T, U> Construct<T, Construct<T, U>> where U: Clone, Construct<T, U>: Clone {
    pub fn w(&self) -> U { self.re().re() }
    pub fn x(&self) -> U { self.re().im() }
    pub fn y(&self) -> U { self.im().re() }
    pub fn z(&self) -> U { self.im().im() }
}

impl<T, U> Construct<T, Construct<T, U>> {
    pub fn w_ref(&self) -> &U { self.re_ref().re_ref() }
    pub fn x_ref(&self) -> &U { self.re_ref().im_ref() }
    pub fn y_ref(&self) -> &U { self.im_ref().re_ref() }
    pub fn z_ref(&self) -> &U { self.im_ref().im_ref() }

    pub fn w_mut(&mut self) -> &mut U { self.re_mut().re_mut() }
    pub fn x_mut(&mut self) -> &mut U { self.re_mut().im_mut() }
    pub fn y_mut(&mut self) -> &mut U { self.im_mut().re_mut() }
    pub fn z_mut(&mut self) -> &mut U { self.im_mut().im_mut() }
}


impl<T, U> Add<Construct<T, U>> for Construct<T, Construct<T, U>> where Construct<T, U>: Add<Output=Construct<T, U>> {
    type Output = Self;
    fn add(self, other: Construct<T, U>) -> Self::Output {
        let (re, im) = self.split();
        Self::new(re + other, im)
    }
}
impl<T, U> Sub<Construct<T, U>> for Construct<T, Construct<T, U>> where Construct<T, U>: Sub<Output=Construct<T, U>> {
    type Output = Self;
    fn sub(self, other: Construct<T, U>) -> Self::Output {
        let (re, im) = self.split();
        Self::new(re - other, im)
    }
}
impl<T, U> Add<Construct<T, Construct<T, U>>> for Construct<T, U> where Construct<T, U>: Add<Output=Construct<T, U>> {
    type Output = Construct<T, Construct<T, U>>;
    fn add(self, other: Construct<T, Construct<T, U>>) -> Self::Output {
        // Hypercomplex numbers is associative over addinition/subtraction.
        other + self
    }
}
impl<T, U> Sub<Construct<T, Construct<T, U>>> for Construct<T, U> where Construct<T, U>: Sub<Output=Construct<T, U>> {
    type Output = Construct<T, Construct<T, U>>;
    fn sub(self, other: Construct<T, Construct<T, U>>) -> Self::Output {
        // Hypercomplex numbers is associative over addinition/subtraction.
        other - self
    }
}

impl<T, U> Mul<Construct<T, U>> for Construct<T, Construct<T, U>> where Construct<T, U>: Mul<Output=Construct<T, U>> + Clone {
    type Output = Self;
    fn mul(self, other: Construct<T, U>) -> Self::Output {
        let (re, im) = self.split();
        Self::new(re * other.clone(), im * other)
    }
}
impl<T, U> Div<Construct<T, U>> for Construct<T, Construct<T, U>> where Construct<T, U>: Div<Output=Construct<T, U>> + Clone {
    type Output = Self;
    fn div(self, other: Construct<T, U>) -> Self::Output {
        let (re, im) = self.split();
        Self::new(re / other.clone(), im / other)
    }
}
impl<T, U> Mul<Construct<T, Construct<T, U>>> for Construct<T, U> where Construct<T, U>: Mul<Output=Construct<T, U>> + Clone {
    type Output = Construct<T, Construct<T, U>>;
    fn mul(self, other: Construct<T, Construct<T, U>>) -> Self::Output {
        let (re, im) = other.split();
        Self::Output::new(self.clone() * re, self * im)
    }
}
impl<T, U> Div<Construct<T, Construct<T, U>>> for Construct<T, U> where Construct<T, Self>: Inv<Output=Construct<T, Self>>, Self: Mul<Construct<T, Self>, Output=Construct<T, Self>> {
    type Output = Construct<T, Construct<T, U>>;
    fn div(self, other: Construct<T, Construct<T, U>>) -> Self::Output {
        self * other.inv()
    }
}

impl<T, U> AddAssign<Construct<T, U>> for Construct<T, Construct<T, U>> where Construct<T, U>: AddAssign {
    fn add_assign(&mut self, other: Construct<T, U>) -> () {
        *self.re_mut() += other;
    }
}
impl<T, U> SubAssign<Construct<T, U>> for Construct<T, Construct<T, U>> where Construct<T, U>: SubAssign {
    fn sub_assign(&mut self, other: Construct<T, U>) -> () {
        *self.re_mut() -= other;
    }
}
impl<T, U> MulAssign<Construct<T, U>> for Construct<T, Construct<T, U>> where Self: Mul<Construct<T, U>, Output=Self> + Clone {
    fn mul_assign(&mut self, other: Construct<T, U>) -> () {
        *self = self.clone() * other;
    }
}
impl<T, U> DivAssign<Construct<T, U>> for Construct<T, Construct<T, U>> where Self: Div<Construct<T, U>, Output=Self> + Clone {
    fn div_assign(&mut self, other: Construct<T, U>) -> () {
        *self = self.clone() / other;
    }
}
