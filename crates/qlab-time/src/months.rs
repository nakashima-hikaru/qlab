#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Months(pub(crate) u32);

impl Months {
    #[must_use]
    pub const fn new(num: u32) -> Self {
        Self(num)
    }
}
