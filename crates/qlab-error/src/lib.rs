use std::borrow::Cow;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use thiserror::Error;

#[derive(Debug)]
pub struct ErrString(Cow<'static, str>);

impl<T> From<T> for ErrString
where
    T: Into<Cow<'static, str>>,
{
    fn from(msg: T) -> Self {
        ErrString(msg.into())
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
}

pub type QLabResult<T> = Result<T, QLabError>;
