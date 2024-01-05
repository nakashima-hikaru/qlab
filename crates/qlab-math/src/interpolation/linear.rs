use crate::interpolation::{Method, Point};
use num_traits::Float;
use qlab_error::ComputeError::InvalidInput;
use qlab_error::QLabResult;
use std::fmt::Debug;

#[derive(Default)]
pub struct Linear<V: Float> {
    points: Vec<Point<V>>,
}

impl<V: Float> Linear<V> {
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

impl<V: Float + Debug> Method<V> for Linear<V> {
    fn fit(&mut self, xs: &[V], ys: &[V]) -> QLabResult<()> {
        if xs.len() != ys.len() {
            return Err(InvalidInput(
                format!(
                    "The length of `xs`: {} must coincide with that of `ys`: {}",
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
        self.points = points;
        Ok(())
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
                format!(
                    "t: {t:?} is smaller than the first grid point: {:?}",
                    self.points
                        .first()
                        .ok_or(InvalidInput("Grid points doesn't exist".into()))?
                        .x
                )
                .into(),
            )
            .into());
        }

        Ok(self.points[idx - 1].y
            + (self.points[idx].y - self.points[idx - 1].y)
                / (self.points[idx].x - self.points[idx - 1].x)
                * (t - self.points[idx - 1].x))
    }
}
