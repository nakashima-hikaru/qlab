use crate::date::Date;

pub trait Calendar {
    fn is_business_day(&self, date: Date) -> bool;

    fn is_holiday(&self, date: Date) -> bool {
        !self.is_business_day(date)
    }
}

impl<C: calendar::Calendar> Calendar for C {
    fn is_business_day(&self, date: Date) -> bool {
        self.is_business_day(date.0)
    }

    fn is_holiday(&self, date: Date) -> bool {
        self.is_holiday(date.0)
    }
}
