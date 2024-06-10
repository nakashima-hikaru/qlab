use crate::interpolation::spline::{HermiteSplineError, InterpolationValue};
use crate::linear_algebra::tridiagonal_matrix::TridiagonalMatrix;
use num_traits::Zero;

struct Point3<V> {
    pub x: V,
    pub y: V,
    pub dydx: V,
}

pub struct NaturalCubic<V: InterpolationValue> {
    points: Vec<Point3<V>>,
}

impl<V: InterpolationValue> NaturalCubic<V> {
    /// Tries to create a new `HermiteSpline` from the given raw points.
    ///
    /// # Arguments
    ///
    /// * `raw_points` - A slice of tuples representing the raw points. Each tuple
    ///                  should contain a value of type V for the x-coordinate and
    ///                  a value of type V for the y-coordinate.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Self)` if the `HermiteSpline` is successfully created, otherwise returns a
    /// `HermiteSplineError`.
    ///
    /// # Errors
    ///
    /// Returns `InsufficientPointsError` if the number of `raw_points` is less than 3.
    /// Returns `PointOrderError` if the x-coordinates of the `raw_points` are not in ascending order.
    ///
    /// # Panics
    /// Will panic if `V` fail to cast constants.
    pub fn try_new(raw_points: &[(V, V)]) -> Result<Self, HermiteSplineError<V>> {
        if raw_points.len() < 3 {
            return Err(HermiteSplineError::InsufficientPointsError(
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
            let point = Point3 { x, y, dydx };
            if point.x < temp {
                return Err(HermiteSplineError::PointOrderError);
            }
            temp = point.x;
            points.push(point);
        }

        Ok(Self { points })
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
    /// * Will panic if `V` cannot cast the constant `6`.
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use qlab_math::interpolation::spline::HermiteSplineError;
    /// use qlab_math::interpolation::spline::natural_cubic::NaturalCubic;
    ///
    /// let points = vec![
    ///     (0.0, 0.0),  // (x, y)
    ///     (1.0, 1.0),
    ///     (2.0, 0.0),
    ///     (3.0, -1.0),
    /// ];
    ///
    /// let spline = NaturalCubic::try_new(&points).unwrap();
    ///
    /// assert!(spline.try_value(0.5).is_ok());
    /// ```
    pub fn try_value(&self, x: V) -> Result<V, HermiteSplineError<V>> {
        match self
            .points
            .binary_search_by(|point| point.x.partial_cmp(&x).unwrap())
        {
            Ok(pos) => Ok(self.points[pos].y),
            Err(pos) => {
                if pos.is_zero() {
                    return Err(HermiteSplineError::OutOfLowerBound(x));
                }
                if pos > self.points.len() {
                    return Err(HermiteSplineError::OutOfUpperBound(x));
                }
                let pos = pos - 1;
                let point = &self.points[pos];
                let next_point = &self.points[pos + 1];
                let h = next_point.x - point.x;
                let six = V::from_i8(6).unwrap();
                Ok(
                    (next_point.x - x) * (next_point.x - x) * (next_point.x - x) / six / h
                        * point.dydx
                        + (x - point.x) * (x - point.x) * (x - point.x) / six / h * next_point.dydx
                        + (next_point.x - x) * (point.y / h - h / six * point.dydx)
                        + (x - point.x) * (next_point.y / h - h / six * next_point.dydx),
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::interpolation::spline::natural_cubic::NaturalCubic;
    #[cfg(feature = "decimal")]
    use rust_decimal::Decimal;

    #[test]
    fn test_f64() {
        let points = [(0.0, 1.0), (0.5, 0.5), (1.0, 0.0)];
        let interpolator = NaturalCubic::try_new(&points).unwrap();
        let val = interpolator.try_value(0.75).unwrap();
        assert!((0.25_f64 - val) / 0.25_f64 < f64::EPSILON);
    }

    #[cfg(feature = "decimal")]
    #[test]
    fn test_decimal() {
        let points = [
            (Decimal::new(0, 0), Decimal::new(1, 0)),
            (Decimal::new(5, 1), Decimal::new(5, 1)),
            (Decimal::new(1, 0), Decimal::new(0, 0)),
        ];
        let interpolator = NaturalCubic::try_new(&points).unwrap();
        let val = interpolator.try_value(Decimal::new(75, 2)).unwrap();
        assert_eq!(val, Decimal::from_str_exact("0.25").unwrap());
    }
}