use crate::business_day_convention::DateRolling;
use crate::calendar::Calendar;
use crate::period::Period;
use chrono::{Datelike, NaiveDate};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::Sub;

/// Represents a date.
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct Date(pub(crate) NaiveDate);

impl Date {
    /// Converts the given year, month, and day into a `NaiveDate` and
    /// returns it as an `Option<Self>`.
    ///
    /// # Arguments
    ///
    /// * `year` - The year as an `i32`.
    /// * `month` - The month as a `u32`.
    /// * `day` - The day as a `u32`.
    ///
    /// # Returns
    ///
    /// An `Option<Self>` which is `Some(Self)` if the given year, month, and day
    /// correspond to a valid date, and `None` otherwise.
    pub fn from_ymd(year: i32, month: u32, day: u32) -> Option<Self> {
        NaiveDate::from_ymd_opt(year, month, day).map(std::convert::Into::into)
    }
    const fn days_in_month(month: u32, leap_year: bool) -> Option<u32> {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => Some(31),
            4 | 6 | 9 | 11 => Some(30),
            2 => {
                if leap_year {
                    Some(29)
                } else {
                    Some(28)
                }
            }
            _ => None,
        }
    }

    /// Checks if the given `self` value represents a leap year.
    ///
    /// # Returns
    ///
    /// Returns `true` if the year is a leap year, `false` otherwise.
    #[must_use]
    #[inline]
    pub const fn leap_year(self) -> bool {
        self.0.leap_year()
    }
    /// Adds the specified number of years to the current date.
    ///
    /// If the resulting date is valid, it returns `Some` containing the new date.
    /// If the resulting date is not valid (e.g. February 29 in a non-leap year),
    /// it returns `None`.
    ///
    /// # Arguments
    ///
    /// * `self` - The current date.
    /// * `rhs_years` - The number of years to add.
    ///
    /// # Returns
    ///
    /// The new date, wrapped in `Some`, if it is valid.
    /// `None` if the resulting date is not valid.
    #[must_use]
    pub fn checked_add_years(self, rhs_years: i32) -> Option<Self> {
        self.0.with_year(self.0.year() + rhs_years).map_or_else(
            || {
                self.0.with_day(28).and_then(|date| {
                    date.with_year(self.0.year() + rhs_years)
                        .map(std::convert::Into::into)
                })
            },
            |date| Some(date.into()),
        )
    }
    /// Adds the specified number of months to the datetime.
    ///
    /// If the operation is successful, returns the resulting datetime wrapped in `Some`.
    ///
    /// # Arguments
    ///
    /// * `rhs_months` - The number of months to add to the datetime.
    ///
    /// # Returns
    ///
    /// * `Some(Self)` - The resulting datetime after adding the months.
    /// * `None` - If the operation could not be performed.
    #[must_use]
    pub fn checked_add_months(self, rhs_months: crate::period::months::Months) -> Option<Self> {
        if let Some(date) = self.0.checked_add_months(chrono::Months::new(rhs_months.0)) {
            Some(date.into())
        } else {
            self.0
                .with_day(Self::days_in_month(self.month(), self.leap_year())?)
                .and_then(|date| {
                    date.checked_add_months(chrono::Months::new(rhs_months.0))
                        .map(std::convert::Into::into)
                })
        }
    }
    /// Subtracts the specified number of months from a `Date`.
    ///
    /// If the resulting date is valid, it returns `Some(date)`, where `date` is the resulting date after subtracting the months.
    /// Otherwise, it returns `None`.
    ///
    /// # Arguments
    ///
    /// * `rhs_months` - The number of months to subtract.
    #[must_use]
    pub fn checked_sub_months(self, rhs_months: crate::period::months::Months) -> Option<Self> {
        if let Some(date) = self.0.checked_sub_months(chrono::Months::new(rhs_months.0)) {
            Some(date.into())
        } else {
            self.0
                .with_day(Self::days_in_month(self.month(), self.leap_year())?)
                .and_then(|date| {
                    date.checked_sub_months(chrono::Months::new(rhs_months.0))
                        .map(std::convert::Into::into)
                })
        }
    }
    /// Adds the given number of days to the current date.
    ///
    /// # Arguments
    ///
    /// * `rhs_days` - The number of days to add.
    ///
    /// # Returns
    ///
    /// Returns `Some(Self)` if the addition is successful, where `Self` is the type of the current date.
    /// Returns `None` if the addition results in an overflow or underflow.
    #[must_use]
    pub fn checked_add_days(self, rhs_days: crate::period::days::Days) -> Option<Self> {
        self.0
            .checked_add_days(chrono::Days::new(rhs_days.0))
            .map(std::convert::Into::into)
    }
    /// Subtract the given number of days from the current value.
    ///
    /// # Arguments
    ///
    /// - `rhs_days`: The number of days to subtract.
    ///
    /// # Returns
    ///
    /// Returns an option containing the new value if subtraction operation is successful,
    /// otherwise returns `None`.
    #[must_use]
    pub fn checked_sub_days(self, rhs_days: crate::period::days::Days) -> Option<Self> {
        self.0
            .checked_sub_days(chrono::Days::new(rhs_days.0))
            .map(std::convert::Into::into)
    }
    /// Makes a new `NaiveDate` for the next calendar date.
    ///
    /// Returns `None` when `self` is the last representable date.
    #[inline]
    #[must_use]
    pub fn succ_opt(self) -> Option<Self> {
        self.0.succ_opt().map(Date)
    }
    /// Makes a new `NaiveDate` for the previous calendar date.
    ///
    /// Returns `None` when `self` is the first representable date.
    #[inline]
    #[must_use]
    pub fn pred_opt(self) -> Option<Self> {
        self.0.pred_opt().map(Date)
    }

    /// Returns the year stored in the corresponding `Date` object.
    #[must_use]
    #[inline]
    pub fn year(&self) -> i32 {
        self.0.year()
    }
    /// Returns the month component of the given value.
    #[must_use]
    #[inline]
    pub fn month(&self) -> u32 {
        self.0.month()
    }
    /// Returns the day of the month represented by the given object.
    ///
    /// # Returns
    ///
    /// The day of the month as a `u32` value.
    #[must_use]
    #[inline]
    pub fn day(self) -> u32 {
        self.0.day()
    }
    /// Retrieves the serial date of the given object.
    ///
    /// The serial date is a representation of the object as the number of days since the Common Era (CE).
    ///
    /// # Returns
    ///
    /// The serial date of the object as an `i32` value.
    #[must_use]
    #[inline]
    pub fn serial_date(self) -> i32 {
        self.0.num_days_from_ce()
    }
    /// Returns an optional `Self`value.
    ///
    /// This method checks if the current date is a weekend (Saturday or Sunday).
    /// If it is, it rolls the date to the next Monday by adding the number of days
    /// from the current day to the next Monday. If the current date is not a weekend,
    /// it checks if the month has changed since the date was initially set.
    /// If the month has changed or rolled over from December to January,
    /// it rolls back the date by subtracting 3 days.
    /// If none of the above conditions are met, it returns `Some(self)`.
    #[must_use]
    pub fn weekend_roll(self) -> Option<Self> {
        let original_month = self.0.month();
        let weekday = self.0.weekday();
        if weekday as u32 > 4 {
            return self.checked_add_days(crate::period::days::Days::new(7 - weekday as u64));
        }
        if original_month < self.0.month() || (original_month == 12 && self.0.month() == 1) {
            return self.checked_sub_days(crate::period::days::Days::new(3));
        }
        Some(self)
    }
}

impl From<NaiveDate> for Date {
    fn from(value: NaiveDate) -> Self {
        Self(value)
    }
}

impl Sub for Date {
    type Output = i32;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0.num_days_from_ce() - rhs.0.num_days_from_ce()
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl Date {
    fn checked_add(self, period: impl Period) -> Option<Self> {
        period.checked_add(self)
    }

    pub fn checked_roll(
        self,
        period: impl Period,
        calendar: &impl Calendar,
        rolling: DateRolling,
    ) -> Option<Self> {
        match rolling {
            DateRolling::Unadjusted => self.checked_add(period),
            DateRolling::Following => {
                let mut ret = self.checked_roll(period, calendar, DateRolling::Unadjusted)?;
                while !calendar.is_business_day(ret) {
                    ret = ret.succ_opt()?;
                }
                Some(ret)
            }
            DateRolling::ModifiedFollowing => {
                let ret = self.checked_roll(period, calendar, DateRolling::Following)?;
                if ret.month() == self.month() {
                    Some(ret)
                } else {
                    self.checked_roll(period, calendar, DateRolling::Preceding)
                }
            }
            DateRolling::Preceding => {
                let mut ret = self.checked_roll(period, calendar, DateRolling::Unadjusted)?;
                while !calendar.is_business_day(ret) {
                    ret = ret.pred_opt()?;
                }
                Some(ret)
            }
            DateRolling::ModifiedPreceding => {
                let ret = self.checked_roll(period, calendar, DateRolling::Preceding)?;
                if ret.month() == self.month() {
                    Some(ret)
                } else {
                    self.checked_roll(period, calendar, DateRolling::Following)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Date;
    use chrono::{Datelike, Weekday};

    #[test]
    fn test_from_ymd_opt_valid_date() {
        let result = Date::from_ymd(2023, 8, 15);
        assert!(result.is_some());
        let result_date = result.unwrap();
        assert_eq!(2023, result_date.year());
        assert_eq!(8, result_date.month());
        assert_eq!(15, result_date.day());
    }

    #[test]
    fn test_from_ymd_opt_invalid_year() {
        let result = Date::from_ymd(2023, 2, 29);
        assert!(result.is_none());
    }

    #[test]
    fn test_days_in_month_february() {
        assert_eq!(Date::days_in_month(2, true).unwrap(), 29);
        assert_eq!(Date::days_in_month(2, false).unwrap(), 28);
    }

    #[test]
    fn test_days_in_month_april() {
        assert_eq!(Date::days_in_month(4, true).unwrap(), 30);
        assert_eq!(Date::days_in_month(4, false).unwrap(), 30);
    }

    #[test]
    fn test_days_in_month_january() {
        assert_eq!(Date::days_in_month(1, true).unwrap(), 31);
        assert_eq!(Date::days_in_month(1, false).unwrap(), 31);
        assert_eq!(Date::days_in_month(2, true).unwrap(), 29);
        assert_eq!(Date::days_in_month(2, false).unwrap(), 28);
    }

    #[test]
    fn test_days_in_month_out_of_range() {
        assert!(Date::days_in_month(13, false).is_none());
    }

    #[test]
    fn test_leap_year() {
        let date = Date::from_ymd(2023, 12, 27).unwrap();
        assert!(!date.leap_year());

        let date = Date::from_ymd(2024, 12, 27).unwrap();
        assert!(date.leap_year());
    }

    #[test]
    fn test_add_years() {
        let date = Date::from_ymd(2023, 12, 31).unwrap();
        let result = date.checked_add_years(1);
        assert!(result.is_some());
        let new_date = result.unwrap();
        assert_eq!(2024, new_date.year());
        assert_eq!(12, new_date.month());
        assert_eq!(31, new_date.day());
        let date = Date::from_ymd(2024, 2, 29).unwrap();
        let result = date.checked_add_years(1);
        let new_date = result.unwrap();
        assert_eq!(2025, new_date.year());
        assert_eq!(2, new_date.month());
        assert_eq!(28, new_date.day());
    }

    #[test]
    fn test_add_months() {
        let date = Date::from_ymd(2023, 12, 31).unwrap();
        let result = date.checked_add_months(crate::period::months::Months::new(2));
        assert!(result.is_some());
        let new_date = result.unwrap();
        assert_eq!(2024, new_date.year());
        assert_eq!(2, new_date.month());
        assert_eq!(29, new_date.day());
    }

    #[test]
    fn test_sub_months() {
        let date = Date::from_ymd(2023, 2, 1).unwrap();
        let result = date.checked_sub_months(crate::period::months::Months::new(2));
        assert!(result.is_some());
        let new_date = result.unwrap();
        assert_eq!(2022, new_date.year());
        assert_eq!(12, new_date.month());
        assert_eq!(1, new_date.day());
    }

    #[test]
    fn test_add_days() {
        let date = Date::from_ymd(2023, 2, 28).unwrap();
        let result = date.checked_add_days(crate::period::days::Days::new(2));
        assert!(result.is_some());
        let new_date = result.unwrap();
        assert_eq!(2023, new_date.year());
        assert_eq!(3, new_date.month());
        assert_eq!(2, new_date.day());
    }

    #[test]
    fn test_sub_days() {
        let date = Date::from_ymd(2023, 3, 1).unwrap();
        let result = date.checked_sub_days(crate::period::days::Days::new(2));
        assert!(result.is_some());
        let new_date = result.unwrap();
        assert_eq!(2023, new_date.year());
        assert_eq!(2, new_date.month());
        assert_eq!(27, new_date.day());
    }

    #[test]
    fn test_serial_date() {
        let date = Date::from_ymd(2023, 3, 1).unwrap();
        assert_eq!(date.serial_date(), 738_580);
    }

    #[test]
    fn test_weekend_roll() {
        let date = Date::from_ymd(2023, 3, 5).unwrap();
        assert_eq!(date.0.weekday(), Weekday::Sun);
        let result = date.weekend_roll();
        assert!(result.is_some());
        let new_date = result.unwrap();
        assert_eq!(2023, new_date.year());
        assert_eq!(3, new_date.month());
        assert_eq!(6, new_date.day());

        let date = Date::from_ymd(2023, 3, 4).unwrap();
        assert_eq!(date.0.weekday(), Weekday::Sat);
        let result = date.weekend_roll();
        assert!(result.is_some());
        let new_date = result.unwrap();
        assert_eq!(2023, new_date.year());
        assert_eq!(3, new_date.month());
        assert_eq!(6, new_date.day());
    }
}
