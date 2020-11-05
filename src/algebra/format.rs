use core::fmt::{Debug, Display, Formatter, Result as FmtResult};
use super::construct::*;

pub enum FmtType {
    Debug,
    Display
}

pub trait Format<T> {
    fn level() -> usize;
    fn write_content_debug(&self, f: &mut Formatter) -> FmtResult where T: Debug;
    fn write_content_display(&self, f: &mut Formatter) -> FmtResult where T: Display;
    fn write_name(f: &mut Formatter) -> FmtResult {
        match Self::level() {
            0 => unreachable!(),
            1 => write!(f, "Complex"),
            2 => write!(f, "Quaternion"),
            3 => write!(f, "Octonion"),
            4 => write!(f, "Sedenion"),
            n @ _ => write!(f, "Construct{}", n),
        }
    }
}

impl<T, U> Format<T> for Construct<T, Construct<T, U>> where Construct<T, U>: Format<T> {
    fn level() -> usize {
        Construct::<T, U>::level() + 1
    }
    fn write_content_debug(&self, f: &mut Formatter) -> FmtResult where T: Debug {
        self.re_ref().write_content_debug(f)?;
        write!(f, ", ")?;
        self.im_ref().write_content_debug(f)
    }
    fn write_content_display(&self, f: &mut Formatter) -> FmtResult where T: Display {
        self.re_ref().write_content_display(f)?;
        write!(f, ", ")?;
        self.im_ref().write_content_display(f)
    }
}
impl<T> Format<T> for Construct<T, T> {
    fn level() -> usize {
        1
    }
    fn write_content_debug(&self, f: &mut Formatter) -> FmtResult where T: Debug {
        write!(f, "{:?}, {:?}", self.re_ref(), self.im_ref())
    }
    fn write_content_display(&self, f: &mut Formatter) -> FmtResult where T: Display {
        write!(f, "{}, {}", self.re_ref(), self.im_ref())
    }
}

impl<T: Debug, U> Debug for Construct<T, U> where Self: Format<T> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        Self::write_name(f)?;
        write!(f, "(")?;
        self.write_content_debug(f)?;
        write!(f, ")")
    }
}
impl<T: Display, U> Display for Construct<T, U> where Self: Format<T> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        Self::write_name(f)?;
        write!(f, "(")?;
        self.write_content_display(f)?;
        write!(f, ")")
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use std::format;
    use crate::algebra::*;

    #[test]
    fn complex() {
        let c = Complex::<i32>::new(1, -2);
        assert_eq!(format!("{:?}", c), "Complex(1, -2)");
        assert_eq!(format!("{}", c), "Complex(1, -2)");
    }

    #[test]
    fn quaternion() {
        let q = Quaternion::<i32>::new2(1, -2, 3, -4);
        assert_eq!(format!("{:?}", q), "Quaternion(1, -2, 3, -4)");
        assert_eq!(format!("{}", q), "Quaternion(1, -2, 3, -4)");
    }
}
