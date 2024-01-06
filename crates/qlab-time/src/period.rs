use crate::date::Date;

pub mod days;
pub mod months;

pub(crate) trait Period: Copy {
    fn checked_add(self, date: Date) -> Option<Date>;
}
