use crate::yield_curve::grid_point::GridPoint;
use crate::yield_curve::private::YieldCurveInner;
use crate::yield_curve::YieldCurve;
use num_traits::{Float, FromPrimitive};
use qlab_error::ComputationError;
use qlab_time::date::Date;
use qlab_time::day_count::DayCount;
use std::fmt::Debug;

pub struct LinearInterpolation<V: Float + FromPrimitive, D: DayCount> {
    settlement_date: Date,
    points: Vec<GridPoint<V>>,
    day_count: D,
}

impl<V: Float + FromPrimitive, D: DayCount> LinearInterpolation<V, D> {
    /// Creates a new instance of a `Grid` object.
    ///
    /// # Arguments
    ///
    /// * `settlement_date` - The settlement date of the grid.
    /// * `maturities` - An array of maturity dates.
    /// * `spot_yields` - A vector of spot yields.
    /// * `day_count` - The day count convention to calculate the day count fractions.
    ///
    /// # Returns
    ///
    /// Returns a `Grid` object if the input is valid, otherwise returns a `ComputationError` indicating
    /// the reason for the error.
    ///
    /// # Errors
    /// An error returns if invalid inputs are passed by or any day count fraction calculation fails.
    pub fn new(
        settlement_date: Date,
        maturities: &[Date],
        spot_yields: Vec<V>,
        day_count: D,
    ) -> Result<Self, ComputationError> {
        if maturities.len() != spot_yields.len() {
            return Err(ComputationError::InvalidInput(
                "maturities and spot_yields are different lengths".to_string(),
            ));
        }
        let maturities: Vec<_> = maturities
            .iter()
            .map(|maturity| day_count.calculate_day_count_fraction(settlement_date, *maturity))
            .collect::<Result<Vec<V>, _>>()?;
        let mut points = Vec::with_capacity(maturities.len());
        for (maturity, spot_yield) in maturities.iter().copied().zip(spot_yields) {
            points.push(GridPoint {
                maturity,
                spot_yield,
            });
        }
        Ok(Self {
            settlement_date,
            points,
            day_count,
        })
    }
}

impl<V: Float + FromPrimitive + Debug, D: DayCount> YieldCurveInner<V>
    for LinearInterpolation<V, D>
{
    fn yield_curve(&self, t: V) -> Result<V, ComputationError> {
        let last_point = self.points.last().ok_or(ComputationError::InvalidInput(
            "Grid points doesn't exist".to_string(),
        ))?;

        if t >= last_point.maturity {
            return Ok(last_point.spot_yield);
        }
        let idx = self.points.partition_point(|&point| point.maturity < t);
        if idx == 0 {
            return Err(ComputationError::InvalidInput(format!(
                "t: {t:?} is earlier than the maturity of the first grid point"
            )));
        }

        Ok(self.points[idx - 1].spot_yield
            + (self.points[idx].spot_yield - self.points[idx - 1].spot_yield)
                / (self.points[idx].maturity - self.points[idx - 1].maturity)
                * (t - self.points[idx - 1].maturity))
    }
}

impl<V: Float + FromPrimitive + Debug, D: DayCount> YieldCurve<V, D> for LinearInterpolation<V, D> {
    fn settlement_date(&self) -> Date {
        self.settlement_date
    }

    fn day_count_fraction(&self) -> &D {
        &self.day_count
    }
}
