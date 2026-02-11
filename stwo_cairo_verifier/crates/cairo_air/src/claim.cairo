use core::array::Span;
use stwo_verifier_core::TreeArray;
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::utils::{OptionImpl, pack_into_qm31s};
use crate::{PublicData, PublicDataImpl, RelationUsesDict};

/// Trait that defines the functionality required by a "claim",
/// where a "claim" is an object that holds public information about
/// one or multiple components whose trace needs to be verified.
pub trait ClaimTrait<T> {
    /// Mix this claim’s public data into the verification transcript (`channel`),
    /// ensuring it influences all subsequently derived challenges.
    fn mix_into(self: @T, ref channel: Channel);
    /// Return the log₂ sizes of the columns in all components of this claim.
    ///
    /// Columns are grouped first by tree, then by column within each tree.
    /// For example, if `claim.log_sizes()[i][j] == n`, the `j`-th column in the
    /// `i`-th tree has size `2^n`.
    fn log_sizes(self: @T) -> TreeArray<Span<u32>>;
    /// Record the lookups used by the components associated with the claim.
    fn accumulate_relation_uses(self: @T, ref relation_uses: RelationUsesDict);
}

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
