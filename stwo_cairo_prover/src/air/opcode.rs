use stwo_prover::core::channel::Blake2sChannel;

use crate::components::range_check_unit::RangeElements;

#[derive(Clone)]
pub struct OpcodeElements {
    pub range_elements: RangeElements,
}
impl OpcodeElements {
    pub fn draw(channel: &mut Blake2sChannel) -> Self {
        OpcodeElements {
            range_elements: RangeElements::draw(channel),
        }
    }
}
