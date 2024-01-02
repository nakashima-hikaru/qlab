#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Days(pub(crate) u64);

impl Days {
    #[must_use]
    pub const fn new(num: u64) -> Self {
        Self(num)
    }
}
