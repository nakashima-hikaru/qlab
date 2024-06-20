#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DateRolling {
    Unadjusted,
    Following,
    ModifiedFollowing,
    Preceding,
    ModifiedPreceding,
}
