mod grid_point;
pub mod linear_interpolation;

use num_traits::{Float, FromPrimitive};
use qlab_error::ComputationError;
use qlab_time::date::Date;
use qlab_time::day_count::DayCount;

mod private {
    use num_traits::{Float, FromPrimitive};
    use qlab_error::ComputationError;

    pub trait YieldCurveInner<V: Float + FromPrimitive> {
        fn yield_curve(&self, t: V) -> Result<V, ComputationError>;
    }
}

/// A trait representing a yield curve with discount factor calculations.
///
/// The trait is generic over the type of floating point values (`V`) and the day count convention (`D`).
pub trait YieldCurve<V: Float + FromPrimitive, D: DayCount>: private::YieldCurveInner<V> {
    /// Calculates the settlement date.
    ///
    /// The settlement date is the date on which a financial transaction is executed and when
    /// the transfer of ownership takes place.
    ///
    /// # Returns
    ///
    /// - The settlement date as a `Date` object.
    fn settlement_date(&self) -> Date;
    /// Returns a reference to the count fraction of the day.
    ///
    /// The count fraction of the day is a value that represents the fraction of a day
    /// that has passed from the beginning of the day until the current time.
    /// This method returns a reference to the count fraction.
    ///
    /// # Return
    ///
    /// A reference to the count fraction of the day.
    fn day_count_fraction(&self) -> &D;

    /// Calculates the discount factor between two dates.
    ///
    /// This function calculates the discount factor between two dates, `d1` and `d2`.
    /// The discount factor represents the present value of a future cash flow.
    ///
    /// # Arguments
    ///
    /// * `d1` - The first date. Must be smaller than `d2`.
    /// * `d2` - The second date.
    ///
    /// # Returns
    ///
    /// Returns a `Result` where:
    ///
    /// * `Ok(V)` represents the discount factor between `d1` and `d2`.
    /// * `Err(ComputationError)` represents an error that occurs during the computation. Possible errors include:
    ///
    ///   * `InvalidInput` - If `d1` is greater than or equal to `d2`, or if either `d1` or `d2` exceeds the settlement date.
    ///   * Other computation errors.
    ///
    /// # Errors
    /// An Error returns if invalid inputs are passed
    fn discount_factor(&self, d1: Date, d2: Date) -> Result<V, ComputationError> {
        if d2 < d1 {
            return Err(ComputationError::InvalidInput(format!(
                "d1: {d1:?} must be smaller than d2: {d2:?}"
            )));
        }
        if d1 < self.settlement_date() || d2 < self.settlement_date() {
            return Err(ComputationError::InvalidInput(format!(
                "Either {d1:?} or {d2:?} exceeds settlement date: {:?}",
                self.settlement_date()
            )));
        }
        let t2 = self
            .day_count_fraction()
            .calculate_day_count_fraction(self.settlement_date(), d2)?;
        let y2 = self.yield_curve(t2)?;
        if d1 == self.settlement_date() {
            return Ok((-t2 * y2).exp());
        }
        let t1 = self
            .day_count_fraction()
            .calculate_day_count_fraction(self.settlement_date(), d1)?;
        let y1 = self.yield_curve(t1)?;
        Ok((t1 * y1 - t2 * y2).exp())
    }
}
