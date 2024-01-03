use crate::interpolation::private::InterpolatorInner;
use crate::interpolation::{Interpolator, Point};
use num_traits::Float;
use qlab_error::ComputeError::InvalidInput;
use qlab_error::QLabResult;
use std::fmt::Debug;

#[derive(Default)]
pub struct Linear<V: Float> {
    points: Vec<Point<V>>,
}

impl<V: Float> Linear<V> {
    #[must_use]
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }
}

impl<V: Float + Debug> InterpolatorInner<V> for Linear<V> {
    fn set_points(&mut self, points: &[Point<V>]) {
        self.points = points.to_vec();
    }
}

impl<V: Float + Debug> Interpolator<V> for Linear<V> {
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
    fn value(&self, t: V) -> QLabResult<V> {
        let last_point = self
            .points
            .last()
            .ok_or(InvalidInput("Grid points doesn't exist".into()))?;

        if t >= last_point.x {
            return Ok(last_point.y);
        }
        let idx = self.points.partition_point(|&point| point.x < t);
        if idx == 0 {
            return Err(InvalidInput(
                format!("t: {t:?} is smaller than the `x` value of the first grid point").into(),
            )
            .into());
        }

        Ok(self.points[idx - 1].y
            + (self.points[idx].y - self.points[idx - 1].y)
                / (self.points[idx].x - self.points[idx - 1].x)
                * (t - self.points[idx - 1].x))
    }
}
