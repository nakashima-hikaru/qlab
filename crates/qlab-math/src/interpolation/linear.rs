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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpolation::Point;
    use qlab_error::QLabError;

    #[test]
    fn linear_new() {
        let linear: Linear<f64> = Linear::new();
        assert_eq!(linear.points.len(), 0);
    }

    #[test]
    fn set_points() {
        let mut linear: Linear<f64> = Linear::new();
        linear.set_points(&[Point { x: 1.0, y: 2.0 }, Point { x: 3.0, y: 4.0 }]);
        assert_eq!(linear.points.len(), 2);
    }

    #[test]
    fn value_no_points() {
        let linear: Linear<f64> = Linear::new();
        if let Err(QLabError::ComputeError(InvalidInput(_e))) = linear.value(1.0) {
            {}
        } else {
            panic!()
        }
    }

    #[test]
    fn value_small_t() {
        let mut linear: Linear<f64> = Linear::new();
        linear.set_points(&[Point { x: 2.0, y: 3.0 }]);
        if let Err(QLabError::ComputeError(InvalidInput(_e))) = linear.value(1.0) {
            {}
        } else {
            panic!()
        }
    }
}

#[test]
fn value_large_t() {
    let mut linear: Linear<f64> = Linear::new();
    linear.set_points(&[Point { x: 1.0, y: 2.0 }, Point { x: 3.0, y: 4.0 }]);
    let result = linear.value(5.0);
    assert!(f64::abs(result.unwrap() - 4.0f64) < f64::epsilon());
}

#[test]
fn value_interpolate() {
    let mut linear: Linear<f64> = Linear::new();
    linear.set_points(&[Point { x: 1.0, y: 2.0 }, Point { x: 3.0, y: 4.0 }]);
    let result = linear.value(2.0);
    assert!(f64::abs(result.unwrap() - 3.0f64) < f64::epsilon());
}
