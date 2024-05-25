use crate::interpolation::{Method, Point};
use num_traits::Float;
use qlab_error::ComputeError::InvalidInput;
use qlab_error::QLabResult;
use std::fmt::Debug;

/// A linear data structure that stores a collection of points.
///
/// # Type Parameters
///
/// - `V`: The type of values in the points.
///
/// # Examples
///
/// ```
/// use num_traits::Float;
/// use qlab_math::interpolation::linear::Linear;
/// use qlab_math::interpolation::Method;
///
/// let mut linear: Linear<f32> = Linear::default();
///
/// let xs_and_ys: [(f32, f32); 2] = [(1.0_f32, 2.0_f32), (3.0_f32, 4.0_f32)];
///
/// linear.fit(&xs_and_ys).unwrap();
/// assert_eq!(linear.value(2.0).unwrap(), 3.0);
///
/// ```
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
    fn fit(&mut self, xs_and_ys: &[(V, V)]) -> QLabResult<()> {
        let mut points = Vec::with_capacity(xs_and_ys.len());
        for &(x, y) in xs_and_ys {
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
