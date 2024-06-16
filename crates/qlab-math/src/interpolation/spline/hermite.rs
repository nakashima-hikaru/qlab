use crate::interpolation::spline::Value;
use crate::interpolation::{find_index_at_left_boundary, Point2DWithSlope};
use nalgebra::Matrix4;
use nalgebra::Vector4;
use qlab_error::InterpolationError;

pub struct Hermite<V: Value> {
    points: Vec<Point2DWithSlope<V>>,
    m: Matrix4<V>,
}

impl<V: Value> Hermite<V> {
    /// Creates a new instance of `Hermite` from a slice of raw points.
    ///
    /// # Arguments
    ///
    /// * `raw_points` - A slice of tuples representing the raw points of the spline.
    ///                 Each tuple should contain three elements: the x-coordinate, the y-coordinate,
    ///                 and the derivative of y with respect to x (dy/dx).
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the constructed `HermiteSpline` on success,
    /// or a `InterpolationError` if the raw points are not in ascending order based on x-coordinate.
    ///
    /// # Errors
    ///
    /// * `InterpolationError::InsufficientPointsError(n)` - If the number of `raw_points` is less than 3, where `n` is the number of `raw_points`.
    /// * `InterpolationError::PointOrderError` - If the x-coordinates of the `raw_points` are not in ascending order.
    ///
    /// # Panics
    /// Will panic if `V` fail to cast constants.
    pub fn try_new(raw_points: &[(V, V, V)]) -> Result<Self, InterpolationError<V>> {
        let mut temp = raw_points[0].0;

        let mut points = Vec::new();
        for &(x, y, dydx) in raw_points {
            let point = Point2DWithSlope::new(x, y, dydx);
            if point.coordinate.x < temp {
                return Err(InterpolationError::PointOrderError);
            }
            temp = point.coordinate.x;
            points.push(point);
        }
        let m = Matrix4::new(
            V::from_i8(2).unwrap(),
            V::from_i8(-2).unwrap(),
            V::one(),
            V::one(),
            V::from_i8(-3).unwrap(),
            V::from_i8(3).unwrap(),
            V::from_i8(-2).unwrap(),
            -V::one(),
            V::zero(),
            V::zero(),
            V::one(),
            V::zero(),
            V::one(),
            V::zero(),
            V::zero(),
            V::zero(),
        );
        Ok(Self { points, m })
    }

    /// Tries to evaluate the interpolated value of Hermite spline at a given point x.
    ///
    /// # Arguments
    ///
    /// * `x` - The point at which to evaluate the Hermite spline.
    ///
    /// # Returns
    ///
    /// Returns `Ok(value)` if the evaluation was successful, where `value` is the evaluated value of the Hermite spline at `x`.
    /// Returns `Err(error)` if the evaluation was unsuccessful, where `error` is an enum variant indicating the reason for failure.
    ///
    /// # Errors
    ///
    /// Returns `OutOfLowerBound(x)` if `x` is less than the minimum x-coordinate value of any point in the Hermite spline.
    /// Returns `OutOfUpperBound(x)` if `x` is greater than the maximum x-coordinate value of any point in the Hermite spline.
    ///
    /// # Panics
    /// Will panic if partial comparison of points fail.
    pub fn try_value(&self, x: V) -> Result<V, InterpolationError<V>> {
        let pos = find_index_at_left_boundary(&self.points, x)?;

        let point = &self.points[pos];
        let next_point = &self.points[pos + 1];
        let h = next_point.coordinate.x - point.coordinate.x;
        let delta = (x - point.coordinate.x) / h;
        let delta2 = delta * delta;
        let delta3 = delta2 * delta;
        let d = Vector4::new(delta3, delta2, delta, V::one());
        let f = Vector4::new(
            point.coordinate.y,
            next_point.coordinate.y,
            point.dydx * h,
            next_point.dydx * h,
        );
        Ok((d.transpose() * self.m * f).x)
    }
}
