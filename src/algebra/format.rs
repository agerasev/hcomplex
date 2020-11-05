use std::{
    vec::Vec,
    string::String,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    format, vec,
};
use super::construct::*;


pub trait Format<T> {
    fn level() -> usize;
    fn content(&self) -> Vec<&T>;
    fn name() -> String {
        match Self::level() {
            0 => unreachable!(),
            1 => "Complex".into(),
            2 => "Quaternion".into(),
            3 => "Octonion".into(),
            4 => "Sedenion".into(),
            n @ _ => format!("Construct{}", n),
        }
    }
}

impl<T, U> Format<T> for Construct<T, Construct<T, U>> where Construct<T, U>: Format<T> {
    fn level() -> usize {
        Construct::<T, U>::level() + 1
    }
    fn content(&self) -> Vec<&T> {
        let mut v = self.re_ref().content();
        v.append(&mut self.im_ref().content());
        v
    }
}
impl<T> Format<T> for Construct<T, T> {
    fn level() -> usize {
        1
    }
    fn content(&self) -> Vec<&T> {
        vec!(self.re_ref(), self.im_ref())
    }
}

impl<T: Debug, U> Debug for Construct<T, U> where Self: Format<T> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}({})", Self::name(),
            self.content().into_iter().map(|x| format!("{:?}", x)).collect::<Vec<_>>().join(", ")
        )
    }
}
impl<T: Display, U> Display for Construct<T, U> where Self: Format<T> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}({})", Self::name(),
            self.content().into_iter().map(|x| format!("{}", x)).collect::<Vec<_>>().join(", ")
        )
    }
}

#[cfg(test)]
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
