use num_traits::{Float, FromPrimitive};

#[derive(Eq, PartialEq, Copy, Clone)]
pub(super) struct GridPoint<V: Float + FromPrimitive> {
    pub maturity: V,
    pub spot_yield: V,
}
