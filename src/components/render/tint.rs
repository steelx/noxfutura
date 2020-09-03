use crate::components::prelude::*;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct Tint {
    pub color: usize,
}
