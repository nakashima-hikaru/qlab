use std::borrow::Cow;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use thiserror::Error;

#[derive(Debug)]
pub struct ErrString(Cow<'static, str>);

impl<T> From<T> for ErrString
where
    T: Into<Cow<'static, str>>,
{
    fn from(msg: T) -> Self {
        Self(msg.into())
    }
}

impl AsRef<str> for ErrString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for ErrString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for ErrString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Error, Debug)]
pub enum QLabError {
    #[error(transparent)]
    ComputeError(#[from] ComputeError),
}

#[derive(Debug, Error)]
pub enum ComputeError {
    #[error("Zero division occurred")]
    ZeroDivisionError,
    #[error("{0} cannot cast to a primitive type")]
    CastNumberError(ErrString),
    #[error("Invalid inputs are passed by: {0}")]
    InvalidInput(ErrString),
    #[error("interpolation failed")]
    InterpolationError,
}

#[derive(Error, Debug, PartialEq)]
pub enum InterpolationError<V> {
    #[error("points must be sorted")]
    PointOrderError,
    #[error("out of lower bound: {0}")]
    OutOfLowerBound(V),
    #[error("out of upper bound: {0}")]
    OutOfUpperBound(V),
    #[error("length of inputs: {0} is not enough points for construction")]
    InsufficientPointsError(usize),
}

impl<T> From<InterpolationError<T>> for QLabError {
    fn from(_err: InterpolationError<T>) -> Self {
        Self::ComputeError(ComputeError::InterpolationError)
    }
}

pub type QLabResult<T> = Result<T, QLabError>;
