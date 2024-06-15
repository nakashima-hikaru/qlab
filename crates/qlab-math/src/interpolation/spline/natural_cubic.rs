use crate::interpolation;
use crate::interpolation::spline::Value;
use crate::interpolation::{Interpolator, Point2DWithSlope};
use crate::linear_algebra::tridiagonal_matrix::TridiagonalMatrix;
use qlab_error::InterpolationError;

#[derive(Default)]
pub struct NaturalCubic<V: Value> {
    points: Vec<Point2DWithSlope<V>>,
}

impl<V: Value> Interpolator<NaturalCubic<V>, V> for NaturalCubic<V> {
    /// Tries to create a new `NaturalCubic` from the given raw points.
    ///
    /// # Arguments
    ///
    /// * `raw_points` - A slice of tuples representing the raw points. Each tuple
    ///                  should contain a value of type V for the x-coordinate and
    ///                  a value of type V for the y-coordinate.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Self)` if the `NaturalCubic` is successfully created, otherwise returns a
    /// `InterpolationError`.
    ///
    /// # Errors
    ///
    /// Returns `InsufficientPointsError` if the number of `raw_points` is less than 3.
    /// Returns `PointOrderError` if the x-coordinates of the `raw_points` are not in ascending order.
    ///
    /// # Panics
    /// Will panic if `V` fail to cast constants.
    fn try_fit(mut self, raw_points: &[(V, V)]) -> Result<Self, InterpolationError<V>> {
        if raw_points.len() < 3 {
            return Err(InterpolationError::InsufficientPointsError(
                raw_points.len(),
            ));
        }
        let mut du = Vec::with_capacity(raw_points.len() - 1);
        let mut d = Vec::with_capacity(raw_points.len());
        let mut dl = Vec::with_capacity(raw_points.len() - 1);
        for i in 0..raw_points.len() {
            if i == 0 {
                du.push(V::zero());
                d.push(V::one());
            } else if i + 1 == raw_points.len() {
                d.push(V::one());
                dl.push(V::zero());
            } else {
                let h = raw_points[i].0 - raw_points[i - 1].0;
                let h_next = raw_points[i + 1].0 - raw_points[i].0;
                du.push(h_next / V::from_i8(6).unwrap());
                d.push((h + h_next) / V::from_i8(3).unwrap());
                dl.push(h / V::from_i8(6).unwrap());
            }
        }

        let mut b = Vec::with_capacity(raw_points.len());
        for i in 0..raw_points.len() {
            if i == 0 || i + 1 == raw_points.len() {
                b.push(V::zero());
            } else {
                b.push(
                    (raw_points[i + 1].1 - raw_points[i].1)
                        / (raw_points[i + 1].0 - raw_points[i].0)
                        - (raw_points[i].1 - raw_points[i - 1].1)
                            / (raw_points[i].0 - raw_points[i - 1].0),
                );
            }
        }

        let matrix = TridiagonalMatrix::try_new(du, d, dl).unwrap();
        let derivatives = matrix.solve(&b);

        let mut temp = raw_points[0].0;
        let mut points = Vec::new();
        for (&(x, y), dydx) in raw_points.iter().zip(derivatives) {
            let point = Point2DWithSlope::new(x, y, dydx);
            if point.coordinate.x < temp {
                return Err(InterpolationError::PointOrderError);
            }
            temp = point.coordinate.x;
            points.push(point);
        }

        self.points = points;
        Ok(self)
    }

    /// Evaluates the Hermite spline at the given value `x`.
    ///
    /// # Arguments
    ///
    /// * `x` - The value at which to evaluate the spline.
    ///
    /// # Returns
    ///
    /// If `x` is within the range of the spline's points, returns `Ok(y)` where `y` is the value of the spline at `x`.
    ///
    /// # Errors
    /// If `x` is below the lower bound of the spline's points, returns `Err(OutOfLowerBound(x))`.
    /// If `x` is above the upper bound of the spline's points, returns `Err(OutOfUpperBound(x))`.
    ///
    /// # Panics
    /// * Will panic if points contains a point which cannot be compared partially.
    /// * Will panic if `V` cannot cast constants.
    ///
    fn try_value(&self, x: V) -> Result<V, InterpolationError<V>> {
        let pos = interpolation::find_index_at_left_boundary(&self.points, x)?;
        let point = &self.points[pos];
        let next_point = &self.points[pos + 1];
        let h = next_point.coordinate.x - point.coordinate.x;
        let six = V::from_i8(6).unwrap();
        Ok((next_point.coordinate.x - x)
            * (next_point.coordinate.x - x)
            * (next_point.coordinate.x - x)
            / six
            / h
            * point.dydx
            + (x - point.coordinate.x) * (x - point.coordinate.x) * (x - point.coordinate.x)
                / six
                / h
                * next_point.dydx
            + (next_point.coordinate.x - x) * (point.coordinate.y / h - h / six * point.dydx)
            + (x - point.coordinate.x) * (next_point.coordinate.y / h - h / six * next_point.dydx))
    }
}

#[cfg(test)]
mod tests {
    use crate::interpolation::spline::natural_cubic::NaturalCubic;
    use crate::interpolation::Interpolator;

    #[test]
    fn test_f64() {
        let points = [(0.0, 1.0), (0.5, 0.5), (1.0, 0.0)];
        let interpolator = NaturalCubic::default().try_fit(&points).unwrap();
        let val = interpolator.try_value(0.75).unwrap();
        assert!((0.25_f64 - val) / 0.25_f64 < f64::EPSILON);
    }
}
