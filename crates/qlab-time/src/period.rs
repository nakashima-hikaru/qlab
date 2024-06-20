use crate::date::Date;

pub mod days;
pub mod months;
pub mod years;

pub trait Period: Copy {
    fn checked_add(self, date: Date) -> Option<Date>;
}
