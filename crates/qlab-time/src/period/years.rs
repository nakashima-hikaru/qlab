use crate::date::Date;
use crate::period::Period;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Years(pub(crate) i32);

impl Years {
    #[must_use]
    pub const fn new(num: i32) -> Self {
        Self(num)
    }
}

impl Period for Years {
    fn checked_add(self, date: Date) -> Option<Date> {
        date.checked_add_years(self)
    }
}
