use num_traits::real::Real;
use num_traits::{FromPrimitive, Num};
use std::fmt::Debug;
use std::ops::{AddAssign, DivAssign, MulAssign, Neg, SubAssign};

pub trait Value:
    'static
    + Num
    + Copy
    + PartialOrd
    + Neg<Output = Self>
    + Debug
    + Sized
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + FromPrimitive
    + Real
{
}
