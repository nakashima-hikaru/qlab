use crate::date::Date;
use crate::day_count::DayCount;
use num_traits::{Float, FromPrimitive};
use qlab_error::ComputationError;

#[derive(Default, Debug)]
pub struct Thirty360 {}

impl Thirty360 {
    #[allow(clippy::cast_sign_loss)]  // validation deals with it
    fn date_diff(date1: Date, date2: Date) -> Result<u32, ComputationError> {
        if date1 > date2 {
            return Err(ComputationError::InvalidInput(format!("date1: {date1:?} must precede date2: {date2:?}")));
        }
        let d1 = date1.day().min(30);
        let d2 = date2.day().min(30);
        Ok(360 * (date2.year() - date1.year()) as u32 + 30 * (date2.month() - date1.month()) + d2 - d1)
    }
}

impl DayCount for Thirty360 {
    fn calculate_day_count_fraction<V: Float + FromPrimitive>(
        &self,
        date1: Date,
        date2: Date,
    ) -> Result<V, ComputationError> {
        let date_diff = V::from_u32(Self::date_diff(date1, date2)?)
            .ok_or(ComputationError::CastNumberError)?;
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
        let thirty_360 = Thirty360::default();
        let date1 = Date::from_ymd_opt(2023, 1, 1).unwrap();
        let date2 = Date::from_ymd_opt(2023, 12, 31).unwrap();
        let diff: f64 = thirty_360
            .calculate_day_count_fraction(date1, date2)
            .unwrap();
        assert!((diff - 0.997222).abs() < 0.001);
    }
}
