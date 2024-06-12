use crate::value::Value;
use qlab_error::InterpolationError;

pub mod linear;
pub mod spline;

#[derive(Copy, Clone, Debug)]
pub(crate) struct Point<V> {
    x: V,
    y: V,
}

pub trait Interpolator<I, V: Value>: Default {
    /// Fits the model to the given data points.
    ///
    /// This function adjusts the parameters of the model to minimize the difference
    /// between the predicted values and the actual values.
    ///
    /// # Arguments
    ///
    /// * `x` - The input data points. It should be an array of type `V`.
    /// * `y` - The output data points. It should be an array of type `V`.
    ///
    /// # Errors
    ///
    /// Returns an error if the fitting process fails.
    fn try_fit(self, xs_and_ys: &[(V, V)]) -> Result<I, InterpolationError<V>>;

    /// Returns the value of type `V` and wraps it in a `QLabResult`.
    ///
    /// # Arguments
    ///
    /// * `t` - The value of type `V`.
    ///
    /// # Returns
    ///
    /// Returns a `QLabResult` that contains the value `t`.
    ///
    /// # Errors
    ///
    /// An Error returns if interpolation fails.
    fn try_value(&self, t: V) -> Result<V, InterpolationError<V>>;
}
