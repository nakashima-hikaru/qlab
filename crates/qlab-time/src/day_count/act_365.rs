use crate::date::Date;
use crate::day_count::DayCount;
use qlab_error::{ComputeError, QLabResult};
use qlab_math::value::Value;

#[derive(Debug, Copy, Clone)]
pub struct Act365;

impl DayCount for Act365 {
    fn calculate_day_count_fraction<V: Value>(date1: Date, date2: Date) -> QLabResult<V> {
        let date_diff = V::from_i64(date2 - date1)
            .ok_or_else(|| ComputeError::CastNumberError(format!("{}", date2 - date1).into()))?;
        let denomination = V::from_i32(365).unwrap();

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
        let diff: f64 = Act365::calculate_day_count_fraction(date1, date2).unwrap();
        assert!((diff - 0.99726).abs() < 0.001);
    }
}
