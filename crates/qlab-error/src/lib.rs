use thiserror::Error;

#[derive(Error, Debug)]
pub enum ComputationError {
    #[error("Cast failed")]
    CastNumberError,
    #[error("Invalid input")]
    InvalidInput(String),
}
