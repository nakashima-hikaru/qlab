use crate::interpolation::spline::Value;
use crate::interpolation::{find_index_at_left_boundary, Interpolator, Point2D};
use nalgebra::{Matrix4, Vector4};
use qlab_error::InterpolationError;
use std::ops::Mul;

#[derive(Default)]
pub struct CatmullRom<V: Value> {
    points: Vec<Point2D<V>>,
}

impl<V: Value> Interpolator<CatmullRom<V>, V> for CatmullRom<V> {
    /// Constructs a new `CatmullRom` from a slice of raw points.
    ///
    /// # Arguments
    ///
    /// * `raw_points` - A slice of tuples containing raw points `(x, y)` where `x` is the x-coordinate and `y` is the y-coordinate.
    ///
    /// # Returns
    ///
    /// * `Result<Self, InterpolationError<V>>` - A `Result` that either contains the constructed `CatmullRom` or an `InterpolationError`.
    ///
    /// # Errors
    ///
    /// * `InterpolationError::InsufficientPointsError(n)` - If the number of `raw_points` is less than 3, where `n` is the number of `raw_points`.
    /// * `InterpolationError::PointOrderError` - If the x-coordinates of the `raw_points` are not in ascending order.
    ///
    fn try_fit(mut self, raw_points: &[(V, V)]) -> Result<Self, InterpolationError<V>> {
        if raw_points.len() < 3 {
            return Err(InterpolationError::InsufficientPointsError(
                raw_points.len(),
            ));
        }
        let mut temp = raw_points[0].0;
        let mut points = Vec::new();
        for &(x, y) in raw_points {
            let point = Point2D { x, y };
            if point.x < temp {
                return Err(InterpolationError::PointOrderError);
            }
            temp = point.x;
            points.push(point);
        }
        self.points = points;
        Ok(self)
    }
    /// Tries to find the value `x` in the Hermite spline.
    ///
    /// # Arguments
    ///
    /// * `x`: The value to find in the Hermite spline.
    ///
    /// # Returns
    ///
    /// * `Ok(V)`: If the value `x` is found in the Hermite spline, returns the corresponding value `V`.
    /// * `Err(InterpolationError<V>)`: If the value `x` is not found, returns an error indicating whether `x` is out of the lower or upper bound of the spline.
    ///
    /// # Errors
    /// If `x` is below the lower bound of the spline's points, returns `Err(OutOfLowerBound(x))`.
    /// If `x` is above the upper bound of the spline's points, returns `Err(OutOfUpperBound(x))`.
    ///
    /// # Panics
    /// * Will panic if points contains a point which cannot be compared partially.
    /// * Will panic if `V` cannot cast the constant `6`.
    #[allow(clippy::too_many_lines)]
    fn try_value(&self, x: V) -> Result<V, InterpolationError<V>> {
        let pos = find_index_at_left_boundary(&self.points, x)?;

        let point = &self.points[pos];
        let next_point = &self.points[pos + 1];
        let h = next_point.x - point.x;
        let delta = (x - point.x) / h;
        let delta2 = delta * delta;
        let delta3 = delta2 * delta;
        let d = Vector4::new(delta3, delta2, delta, V::one());
        Ok((d.transpose()
            * if pos == 0 {
                let next_next_point = &self.points[pos + 2];
                let next_h = next_next_point.x - next_point.x;
                let beta = h / (h + next_h);
                Matrix4::new(
                    V::zero(),
                    V::one() - beta,
                    -V::one(),
                    beta,
                    V::zero(),
                    -V::one() + beta,
                    V::one(),
                    -beta,
                    V::zero(),
                    -V::one(),
                    V::one(),
                    V::zero(),
                    V::zero(),
                    V::one(),
                    V::zero(),
                    V::zero(),
                )
                .mul(Vector4::new(
                    V::zero(),
                    point.y,
                    next_point.y,
                    next_next_point.y,
                ))
            } else if pos + 2 == self.points.len() {
                let prev_point = &self.points[pos - 1];
                let prev_h = next_point.x - prev_point.x;
                let alpha = h / (h + prev_h);
                Matrix4::new(
                    -alpha,
                    V::one(),
                    -V::one() * alpha,
                    V::zero(),
                    V::from_i8(2).unwrap() * alpha,
                    V::from_i8(-2).unwrap(),
                    V::from_i8(2).unwrap() - V::from_i8(2).unwrap() * alpha,
                    V::zero(),
                    -alpha,
                    V::zero(),
                    alpha,
                    V::zero(),
                    V::zero(),
                    V::one(),
                    V::zero(),
                    V::zero(),
                )
                .mul(Vector4::new(prev_point.y, point.y, next_point.y, V::zero()))
            } else {
                let prev_point = &self.points[pos - 1];
                let prev_h = next_point.x - prev_point.x;
                let alpha = h / (h + prev_h);
                let next_next_point = &self.points[pos + 2];
                let next_h = next_next_point.x - next_point.x;
                let beta = h / (h + next_h);
                Matrix4::new(
                    -alpha,
                    V::from_i8(2).unwrap() - beta,
                    V::from_i8(-2).unwrap() + alpha,
                    beta,
                    V::from_i8(2).unwrap() * alpha,
                    beta - V::from_i8(3).unwrap(),
                    V::from_i8(3).unwrap() - V::from_i8(2).unwrap() * alpha,
                    -beta,
                    -alpha,
                    V::zero(),
                    alpha,
                    V::zero(),
                    V::zero(),
                    V::one(),
                    V::zero(),
                    V::zero(),
                )
                .mul(Vector4::new(
                    prev_point.y,
                    point.y,
                    next_point.y,
                    next_next_point.y,
                ))
            })
        .x)
    }
}

#[cfg(test)]
mod tests {
    use crate::interpolation::spline::catmull_rom::CatmullRom;
    use crate::interpolation::Interpolator;

    #[test]
    fn test_f64() {
        let points = [(0.0, 1.0), (0.5, 0.5), (1.0, 0.0)];
        let interpolator = CatmullRom::default().try_fit(&points).unwrap();
        let val = interpolator.try_value(0.75).unwrap();
        assert!((val - 0.270_833_333_333_333_37_f64).abs() < f64::EPSILON);
    }
}
