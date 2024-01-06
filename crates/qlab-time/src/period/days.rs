use crate::date::Date;
use crate::period::Period;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Days(pub(crate) u64);

impl Days {
    #[must_use]
    pub const fn new(num: u64) -> Self {
        Self(num)
    }
}

impl Period for Days {
    fn checked_add(self, date: Date) -> Option<Date> {
        date.checked_add_days(self)
    }
}
