pub mod act_360;
pub mod act_365;
pub mod thirty_360;

use crate::date::Date;
use qlab_error::QLabResult;
use qlab_math::value::Value;

pub trait DayCount {
    /// Calculates the day count fraction between two dates.
    ///
    /// This function calculates the day count fraction between `date1` and `date2`
    /// using a generic type `V`, which must implement the `Real` and `FromPrimitive`
    /// traits. The day count fraction represents the portion of a year between the two
    /// dates, expressed as a fraction.
    ///
    /// # Arguments
    ///
    /// * `date1` - The first date in the calculation.
    /// * `date2` - The second date in the calculation.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with either the calculated day count fraction of type `V`,
    /// or a `ComputationError` if the calculation fails.
    ///
    /// # Errors
    /// An error occurs if a cast from `V` to a primitive type fails.
    fn calculate_day_count_fraction<V: Value>(date1: Date, date2: Date) -> QLabResult<V>;
}
