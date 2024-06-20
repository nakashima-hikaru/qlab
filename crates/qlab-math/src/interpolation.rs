use crate::value::Value;
use num_traits::Zero;
use qlab_error::InterpolationError;

pub mod linear;
pub mod spline;

trait X<V> {
    fn x(&self) -> &V;
}

struct Point2D<V> {
    x: V,
    y: V,
}

struct Point2DWithSlope<V> {
    coordinate: Point2D<V>,
    dydx: V,
}

impl<V> Point2DWithSlope<V> {
    fn new(x: V, y: V, dydx: V) -> Self {
        Self {
            coordinate: Point2D { x, y },
            dydx,
        }
    }
}

impl<V> X<V> for Point2DWithSlope<V> {
    fn x(&self) -> &V {
        &self.coordinate.x
    }
}

impl<V> X<V> for Point2D<V> {
    fn x(&self) -> &V {
        &self.x
    }
}

pub trait Interpolator: Default {
    type Value: Value;
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
    fn try_fit(
        self,
        xs_and_ys: &[(Self::Value, Self::Value)],
    ) -> Result<Self, InterpolationError<Self::Value>>;

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
    fn try_value(&self, t: Self::Value) -> Result<Self::Value, InterpolationError<Self::Value>>;
}

fn find_index_at_left_boundary<V: PartialOrd>(
    points: &[impl X<V>],
    x: V,
) -> Result<usize, InterpolationError<V>> {
    if points.is_empty() {
        return Err(InterpolationError::InsufficientPointsError(points.len()));
    }
    let pos = points.partition_point(|point| *point.x() < x);
    if pos.is_zero() && *points.iter().next().unwrap().x() <= x {
        return Ok(0);
    }
    if pos.is_zero() {
        return Err(InterpolationError::OutOfLowerBound(x));
    }
    if pos == points.len() {
        return Err(InterpolationError::OutOfUpperBound(x));
    }
    Ok(pos - 1)
}
