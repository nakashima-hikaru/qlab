use crate::date::Date;
use crate::day_count::DayCount;
use num_traits::{Float, FromPrimitive};
use qlab_error::{ComputeError, QLabResult};

#[derive(Debug)]
pub struct Act360;

impl DayCount for Act360 {
    fn calculate_day_count_fraction<V: Float + FromPrimitive>(
        date1: Date,
        date2: Date,
    ) -> QLabResult<V> {
        let date_diff = V::from_i32(date2 - date1).ok_or(ComputeError::CastNumberError(
            format!("{}", date2 - date1).into(),
        ))?;
        let denomination =
            V::from_i32(360).ok_or(ComputeError::CastNumberError(format!("{}", 360).into()))?;
        if denomination.eq(&V::zero()) {
            return Err(ComputeError::ZeroDivisionError.into());
        }
        Ok(date_diff.div(denomination))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::date::Date;
    use crate::day_count::DayCount;

    #[test]
    fn test_calculate_day_count_fraction() {
        let date1 = Date::from_ymd(2023, 1, 1).unwrap();
        let date2 = Date::from_ymd(2023, 12, 31).unwrap();
        let diff: f64 = Act360::calculate_day_count_fraction(date1, date2).unwrap();
        assert!((diff - 1.01111).abs() < 0.001);
    }
}
