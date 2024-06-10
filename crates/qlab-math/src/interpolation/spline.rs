use num_traits::{FromPrimitive, Num};
use std::fmt::Debug;
use std::ops::{AddAssign, DivAssign, MulAssign, Neg, SubAssign};
use thiserror::Error;

pub mod catmull_rom;
pub mod hermite;
pub mod natural_cubic;

pub trait InterpolationValue:
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
{
}

impl InterpolationValue for f32 {}

impl InterpolationValue for f64 {}

#[derive(Error, Debug, PartialEq)]
pub enum HermiteSplineError<V: InterpolationValue> {
    #[error("points must be sorted")]
    PointOrderError,
    #[error("out of lower bound: {0}")]
    OutOfLowerBound(V),
    #[error("out of upper bound: {0}")]
    OutOfUpperBound(V),
    #[error("length of inputs: {0} is not enough points for construction")]
    InsufficientPointsError(usize),
}
