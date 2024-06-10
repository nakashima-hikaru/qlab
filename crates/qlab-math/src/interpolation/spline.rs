use crate::value::Value;

pub mod catmull_rom;
pub mod hermite;
pub mod natural_cubic;

impl Value for f32 {}

impl Value for f64 {}
