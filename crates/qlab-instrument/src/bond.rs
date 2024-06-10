use qlab_error::QLabResult;
use qlab_math::interpolation::Interpolator;
use qlab_math::value::Value;
use qlab_termstructure::yield_curve::YieldCurve;
use qlab_time::date::Date;
use qlab_time::day_count::DayCount;
use qlab_time::frequency::Frequency;
use qlab_time::period::months::Months;
use std::cmp::Ordering;

struct BondCashFlow<V> {
    due_date: Date,
    payment_date: Date,
    payment_amount: V,
}

/// A generic struct representing a bond.
///
/// # Fields
///
/// * `bond_id`: A unique identifier for the bond.
/// * `bond_cash_flows`: A vector of bond cash flows.
///
/// # Generic Parameters
///
/// * `V`: The type of value associated with each bond cash flow.
pub struct Bond<V> {
    bond_id: String,
    bond_cash_flows: Vec<BondCashFlow<V>>,
}

impl<V: Value> Bond<V> {
    /// Creates a new bond with the given parameters.
    ///
    /// This function calculates the cash flows for the bond based on the provided parameters.
    /// It returns `Some(Self)` if the calculations are successful, otherwise it returns `None`.
    ///
    /// # Arguments
    ///
    /// * `bond_id` - The ID of the bond.
    /// * `issue_date` - The date on which the bond was issued.
    /// * `first_coupon_date` - The date of the first coupon payment.
    /// * `penultimate_coupon_date` - The date of the penultimate coupon payment.
    /// * `maturity_date` - The maturity date of the bond.
    /// * `coupon_frequency` - The frequency at which the bond pays coupon payments (e.g. Annually, Semi-Annually, Quarterly).
    /// * `coupon_rate` - The coupon rate of the bond.
    /// * `face_value` - The face value or principal amount of the bond.
    ///
    /// # Return Value
    ///
    /// An `Option<Self>` which contains the newly created bond if the calculations are successful,
    /// otherwise `None` if any errors occur during the calculations.
    ///
    /// # Errors
    /// Returns `None` if construction process fails
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        bond_id: &str,
        issue_date: Date,
        first_coupon_date: Date,
        penultimate_coupon_date: Date,
        maturity_date: Date,
        coupon_frequency: Frequency,
        coupon_rate: V,
        face_value: V,
    ) -> Option<Self> {
        let months_in_regular_coupon_period = Months::new(12 / coupon_frequency as u32);
        let regular_coupon_payment = coupon_rate * face_value / V::from_u8(coupon_frequency as u8)?;

        let mut regular_due_date = first_coupon_date;
        let mut bond_cash_flows = Vec::new();

        while regular_due_date <= penultimate_coupon_date {
            let payment_date = regular_due_date.weekend_roll()?;
            bond_cash_flows.push(BondCashFlow {
                due_date: regular_due_date,
                payment_date,
                payment_amount: regular_coupon_payment,
            });
            regular_due_date =
                regular_due_date.checked_add_months(months_in_regular_coupon_period)?;
        }
        Self::first_cash_flow(
            issue_date,
            first_coupon_date,
            months_in_regular_coupon_period,
            regular_coupon_payment,
            &mut bond_cash_flows,
        )?;
        let final_cash_flow = Self::final_cash_flow(
            penultimate_coupon_date,
            maturity_date,
            face_value,
            months_in_regular_coupon_period,
            regular_coupon_payment,
        )?;
        bond_cash_flows.push(final_cash_flow);
        Some(Self {
            bond_id: bond_id.to_string(),
            bond_cash_flows,
        })
    }

    fn first_cash_flow(
        issue_date: Date,
        first_coupon_date: Date,
        months_in_regular_coupon_period: Months,
        regular_coupon_payment: V,
        bond_cash_flows: &mut [BondCashFlow<V>],
    ) -> Option<()> {
        let first_prior = first_coupon_date.checked_sub_months(months_in_regular_coupon_period)?;
        match first_prior.cmp(&issue_date) {
            Ordering::Less => {
                let coupon_fraction = V::from_i64(first_coupon_date - issue_date)?
                    / V::from_i64(first_coupon_date - first_prior)?;
                bond_cash_flows[0].payment_amount *= coupon_fraction;
            }
            Ordering::Greater => {
                let second_prior =
                    first_prior.checked_sub_months(months_in_regular_coupon_period)?;
                let coupon_fraction = V::from_i64(first_prior - issue_date)?
                    / V::from_i64(first_prior - second_prior)?;
                bond_cash_flows[0].payment_amount += coupon_fraction * regular_coupon_payment;
            }
            Ordering::Equal => {}
        }
        Some(())
    }

    fn final_cash_flow(
        penultimate_coupon_date: Date,
        maturity_date: Date,
        face_value: V,
        months_in_regular_coupon_period: Months,
        regular_coupon_payment: V,
    ) -> Option<BondCashFlow<V>> {
        let mut final_coupon = regular_coupon_payment;
        let maturity_regular_date =
            penultimate_coupon_date.checked_add_months(months_in_regular_coupon_period)?;
        match maturity_date.cmp(&maturity_regular_date) {
            Ordering::Less => {
                let coupon_fraction = V::from_i64(maturity_date - penultimate_coupon_date)?
                    / V::from_i64(maturity_regular_date - penultimate_coupon_date)?;
                final_coupon *= coupon_fraction;
            }
            Ordering::Greater => {
                let next_regular_date =
                    maturity_regular_date.checked_add_months(months_in_regular_coupon_period)?;
                let extra_coupon_fraction = V::from_i64(maturity_date - maturity_regular_date)?
                    / V::from_i64(next_regular_date - maturity_regular_date)?;
                final_coupon += extra_coupon_fraction * regular_coupon_payment;
            }
            Ordering::Equal => {}
        }
        Some(BondCashFlow {
            due_date: maturity_date,
            payment_date: maturity_date,
            payment_amount: face_value + final_coupon,
        })
    }
    /// Calculates the discounted value of the bond's cash flows.
    ///
    /// Parameters:
    /// - `bond_settle_date`: The settlement date of the bond.
    /// - `yield_curve`: The yield curve used to calculate the discount factors.
    ///
    /// Returns:
    /// If successful, returns the discounted value of the bond's cash flows. Otherwise, returns an error.
    ///
    /// # Generic Parameters
    ///
    /// - `D`: The type implementing the `DayCount` trait.
    /// - `V`: The type representing the values of the bond's cash flows.
    ///
    /// # Arguments
    ///
    /// - `bond_settle_date`: The settlement date of the bond.
    /// - `yield_curve`: The yield curve used to calculate the discount factors.
    ///
    /// # Errors
    /// Error occurs if a discount factor calculation fails
    pub fn discounted_value<D: DayCount, I: Interpolator<V>>(
        &self,
        bond_settle_date: Date,
        yield_curve: &YieldCurve<D, V, I>,
    ) -> QLabResult<V> {
        let mut pv = V::zero();
        for i in 0..self.bond_cash_flows.len() {
            if bond_settle_date < self.bond_cash_flows[i].due_date {
                pv += yield_curve
                    .discount_factor(bond_settle_date, self.bond_cash_flows[i].payment_date)?
                    * self.bond_cash_flows[i].payment_amount;
            }
        }
        Ok(pv)
    }
    #[must_use]
    pub fn bond_id(&self) -> &str {
        &self.bond_id
    }
}
