use crate::date::Date;
use crate::period::Period;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Months(pub(crate) u32);

impl Months {
    #[must_use]
    pub const fn new(num: u32) -> Self {
        Self(num)
    }
}

impl Period for Months {
    fn checked_add(self, date: Date) -> Option<Date> {
        date.checked_add_months(self)
    }
}
