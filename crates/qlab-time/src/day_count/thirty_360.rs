use crate::date::Date;
use crate::day_count::DayCount;
use qlab_error::ComputeError::InvalidInput;
use qlab_error::{ComputeError, QLabResult};
use qlab_math::value::Value;

#[derive(Debug, Copy, Clone)]
pub struct Thirty360;

impl Thirty360 {
    #[allow(clippy::cast_sign_loss)] // validation deals with it
    fn date_diff(date1: Date, date2: Date) -> QLabResult<u32> {
        if date1 > date2 {
            return Err(
                InvalidInput(format!("date1: {date1} must precede date2: {date2}").into()).into(),
            );
        }
        let d1 = date1.day().min(30);
        let d2 = date2.day().min(30);
        Ok(
            360 * (date2.year() - date1.year()) as u32 + 30 * (date2.month() - date1.month()) + d2
                - d1,
        )
    }
}

impl DayCount for Thirty360 {
    fn calculate_day_count_fraction<V: Value>(date1: Date, date2: Date) -> QLabResult<V> {
        let date_diff = Self::date_diff(date1, date2)?;
        let date_diff = V::from_u32(date_diff)
            .ok_or_else(|| ComputeError::CastNumberError(format!("{date_diff}").into()))?;
        let denomination =
            V::from_i32(360).ok_or_else(|| ComputeError::CastNumberError("360".into()))?;
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
        let diff: f64 = Thirty360::calculate_day_count_fraction(date1, date2).unwrap();
        assert!((diff - 0.997_222).abs() < 0.001);
    }
}
