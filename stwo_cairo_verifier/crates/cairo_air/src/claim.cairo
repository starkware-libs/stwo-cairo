use core::array::Span;
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::utils::{OptionImpl, pack_into_qm31s};
use crate::{PublicData, PublicDataImpl};

#[derive(Serde, Drop)]
pub struct FlatClaim {
    pub component_enable_bits: Span<bool>,
    pub component_log_sizes: Span<u32>,
    pub public_data: PublicData,
}

#[generate_trait]
pub impl FlatClaimImpl of FlatClaimTrait {
    fn mix_into(self: @FlatClaim, ref channel: Channel) {
        channel.mix_felts(pack_into_qm31s(array![self.component_enable_bits.len()].span()));
        channel.mix_felts(pack_into_qm31s(enable_bits_to_u32s(*self.component_enable_bits)));
        channel.mix_felts(pack_into_qm31s(*self.component_log_sizes));
        channel
            .mix_felts(
                pack_into_qm31s(array![self.public_data.public_memory.program.len()].span()),
            );
        self.public_data.mix_into(ref channel);
    }
}

/// Converts enable bits to [u32], where each u32 is at most 2^31 - 1.
fn enable_bits_to_u32s(enable_bits: Span<bool>) -> Span<u32> {
    let mut res = array![];
    for bit in enable_bits {
        if *bit {
            res.append(1_u32);
        } else {
            res.append(0_u32);
        }
    }
    res.span()
}
