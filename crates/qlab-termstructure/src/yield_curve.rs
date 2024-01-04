use num_traits::{Float, FromPrimitive};
use qlab_error::ComputeError::InvalidInput;
use qlab_error::QLabResult;
use qlab_math::interpolation::Interpolator;
use qlab_time::date::Date;
use qlab_time::day_count::DayCount;
use std::marker::PhantomData;

mod private {
    use num_traits::{Float, FromPrimitive};
    use qlab_error::QLabResult;
    use qlab_math::interpolation::Interpolator;

    pub trait YieldCurveInner<V: Float + FromPrimitive, I: Interpolator<V>> {
        fn interpolator(&self) -> &I;
        fn yield_curve(&self, t: V) -> QLabResult<V> {
            self.interpolator().value(t)
        }
    }
}

/// A trait representing a yield curve with discount factor calculations.
///
/// The trait is generic over the type of Floating point values (`V`) and the day count convention (`D`).
pub struct YieldCurve<D: DayCount, V: Float + FromPrimitive, I: Interpolator<V>> {
    settlement_date: Date,
    interpolator: I,
    _phantom: PhantomData<V>,
    _day_count: PhantomData<D>,
}

impl<V: Float + FromPrimitive, D: DayCount, I: Interpolator<V>> YieldCurve<D, V, I> {
    /// Creates a new instance of the `QLab` struct.
    ///
    /// # Arguments
    ///
    /// * `settlement_date` - The settlement date of the instrument.
    /// * `maturities` - A slice of maturity dates.
    /// * `spot_yields` - A vector of spot yields.
    /// * `day_count` - The day count convention to use.
    /// * `interpolator` - An interpolator for fitting the yields.
    ///
    /// # Returns
    ///
    /// A `QLabResult` containing the new instance of `QLab`, or an error if the inputs are invalid.
    ///
    /// # Errors
    /// Returns an `Err` variant if the lengths of `maturities` and `spot_yields` do not match.
    pub fn new(
        settlement_date: Date,
        maturities: &[Date],
        spot_yields: &[V],
        mut interpolator: I,
    ) -> QLabResult<Self> {
        if maturities.len() != spot_yields.len() {
            return Err(
                InvalidInput("maturities and spot_yields are different lengths".into()).into(),
            );
        }
        let maturities: Vec<_> = maturities
            .iter()
            .map(|maturity| D::calculate_day_count_fraction(settlement_date, *maturity))
            .collect::<Result<Vec<V>, _>>()?;
        interpolator.fit(&maturities, spot_yields)?;
        Ok(Self {
            _phantom: PhantomData,
            settlement_date,
            _day_count: PhantomData,
            interpolator,
        })
    }
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
    pub fn discount_factor(&self, d1: Date, d2: Date) -> QLabResult<V> {
        if d2 < d1 {
            return Err(
                InvalidInput(format!("d1: {d1} must be smaller than d2: {d2}").into()).into(),
            );
        }
        if d1 < self.settlement_date || d2 < self.settlement_date {
            return Err(InvalidInput(
                format!(
                    "Either {d1} or {d2} exceeds settlement date: {:?}",
                    self.settlement_date
                )
                .into(),
            )
            .into());
        }
        let t2 = D::calculate_day_count_fraction(self.settlement_date, d2)?;
        let y2 = self.yield_curve(t2)?;
        if d1 == self.settlement_date {
            return Ok((-t2 * y2).exp());
        }
        let t1 = D::calculate_day_count_fraction(self.settlement_date, d1)?;
        let y1 = self.yield_curve(t1)?;
        Ok((t1 * y1 - t2 * y2).exp())
    }
    fn yield_curve(&self, t: V) -> QLabResult<V> {
        self.interpolator.value(t)
    }
}
