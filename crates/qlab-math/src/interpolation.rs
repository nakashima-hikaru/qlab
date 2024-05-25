use num_traits::Float;
use qlab_error::QLabResult;

pub mod linear;

#[derive(Copy, Clone, Debug)]
pub(crate) struct Point<V: Float> {
    x: V,
    y: V,
}

#[allow(private_bounds)]
pub trait Method<V: Float> {
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
    fn fit(&mut self, xs_and_ys: &[(V, V)]) -> QLabResult<()>;

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
    fn value(&self, t: V) -> QLabResult<V>;
}
