use crate::date::Date;
use crate::day_count::DayCount;
use num_traits::{Float, FromPrimitive};
use qlab_error::ComputationError;

#[derive(Default, Debug)]
pub struct Act360 {}

impl DayCount for Act360 {
    fn calculate_day_count_fraction<V: Float + FromPrimitive>(
        &self,
        date1: Date,
        date2: Date,
    ) -> Result<V, ComputationError> {
        let date_diff = V::from_i32(date2 - date1).ok_or(ComputationError::CastNumberError)?;
        let denomination = V::from(360.0).ok_or(ComputationError::CastNumberError)?;
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
        let act_360 = Act360::default();
        let date1 = Date::from_ymd_opt(2023, 1, 1).unwrap();
        let date2 = Date::from_ymd_opt(2023, 12, 31).unwrap();
        let diff: f64 = act_360.calculate_day_count_fraction(date1, date2).unwrap();
        assert!((diff - 1.01111).abs() < 0.001);
    }
}
