#![no_std]
#[cfg(feature = "std")]
extern crate std;


mod algebra;
pub use algebra::*;

pub mod transform;

pub mod prelude {
    pub use num_traits::{One, Zero, Inv};
    pub use crate::{Conj, Dot, NormSqr, Norm, NormL1, Algebra};
}
