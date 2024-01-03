use num_traits::Float;
use qlab_error::ComputeError::InvalidInput;
use qlab_error::QLabResult;

pub mod linear;

#[derive(Copy, Clone)]
pub(crate) struct Point<V: Float> {
    x: V,
    y: V,
}

mod private {
    use crate::interpolation::Point;
    use num_traits::Float;

    pub(crate) trait InterpolatorInner<V: Float> {
        fn set_points(&mut self, points: &[Point<V>]);
    }
}

#[allow(private_bounds)]
pub trait Interpolator<V: Float>: private::InterpolatorInner<V> {
    /// Fits the given data points to the `QLab` object.
    ///
    /// # Arguments
    ///
    /// * `xs` - An array slice containing the x-values of the data points.
    /// * `ys` - An array slice containing the y-values of the data points.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant if the lengths of `xs` and `ys` do not match.
    fn fit(&mut self, xs: &[V], ys: &[V]) -> QLabResult<()> {
        if xs.len() != ys.len() {
            return Err(InvalidInput(
                format!(
                    "The length `xs`: {} must coincide with that of `ys`: {}",
                    xs.len(),
                    ys.len()
                )
                .into(),
            )
            .into());
        }
        let mut points = Vec::with_capacity(xs.len());
        for (&x, &y) in xs.iter().zip(ys) {
            points.push(Point { x, y });
        }
        self.set_points(points.as_ref());
        Ok(())
    }

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
