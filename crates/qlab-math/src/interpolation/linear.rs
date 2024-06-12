use crate::interpolation::{Interpolator, Point};
use crate::value::Value;
use num_traits::real::Real;
use qlab_error::InterpolationError;

/// A linear data structure that stores a collection of points.
///
/// # Type Parameters
///
/// - `V`: The type of values in the points.
///
/// # Examples
///
/// ```
/// use qlab_math::interpolation::linear::Linear;
/// use qlab_math::interpolation::Interpolator;
///
/// let mut linear: Linear<f32> = Linear::default();
///
/// let xs_and_ys: [(f32, f32); 2] = [(1.0_f32, 2.0_f32), (3.0_f32, 4.0_f32)];
///
/// linear.try_fit(&xs_and_ys).unwrap();
/// assert_eq!(linear.try_value(2.0).unwrap(), 3.0);
///
/// ```
#[derive(Default)]
pub struct Linear<V> {
    points: Vec<Point<V>>,
}

impl<V: Real> Linear<V> {
    /// Creates a new instance of the `QLab` struct.
    ///
    /// # Arguments
    ///
    /// * `xs` - An array slice containing the x-coordinates of the points.
    /// * `ys` - An array slice containing the y-coordinates of the points.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of `QLabResult` if the lengths of `xs` and `ys` are not equal.
    /// The error message will indicate the lengths of `xs` and `ys`
    #[must_use]
    pub fn new() -> Self {
        Self {
            points: Vec::default(),
        }
    }
}

impl<V: Value> Interpolator<Linear<V>, V> for Linear<V> {
    fn try_fit(mut self, raw_points: &[(V, V)]) -> Result<Self, InterpolationError<V>> {
        let mut points = Vec::with_capacity(raw_points.len());
        for &(x, y) in raw_points {
            points.push(Point { x, y });
        }
        self.points = points;
        Ok(self)
    }

    /// Calculates the value at time `t` using linear interpolation based on a grid of points.
    /// If `t` is greater than or equal to the x-coordinate of the last grid point, the y-coordinate of the last grid point will be returned.
    ///
    /// # Arguments
    ///
    /// * `t` - The time value at which to calculate the interpolated value.
    ///
    /// # Returns
    ///
    /// * `QLabResult<V>` - The interpolated value at time `t`.
    ///     - If `t` is smaller than the x-coordinate of the first grid point, an `InvalidInput` error will be returned.
    ///     - If there are no grid points available, an `InvalidInput` error will be returned.
    ///
    /// # Errors
    ///
    /// * `InvalidInput` - Represents an error when the input is invalid or out-of-bounds.
    fn try_value(&self, t: V) -> Result<V, InterpolationError<V>> {
        let last_point = self
            .points
            .last()
            .ok_or(InterpolationError::InsufficientPointsError(
                self.points.len(),
            ))?;

        if t >= last_point.x {
            return Ok(last_point.y);
        }
        let idx = self.points.partition_point(|&point| point.x < t);
        if idx == 0 {
            return Err(InterpolationError::OutOfLowerBound(t));
        }

        Ok(self.points[idx - 1].y
            + (self.points[idx].y - self.points[idx - 1].y)
                / (self.points[idx].x - self.points[idx - 1].x)
                * (t - self.points[idx - 1].x))
    }
}
