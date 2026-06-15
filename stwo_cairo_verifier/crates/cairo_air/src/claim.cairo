use core::array::Span;
use stwo_constraint_framework::CommonLookupElements;
use stwo_verifier_core::TreeArray;
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::fields::qm31::{QM31, QM31Serde};
use stwo_verifier_core::utils::{OptionImpl, pack_into_qm31s};
use crate::claims::CairoClaim;
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


#[derive(Drop)]
pub struct CairoInteractionClaim {
    pub claimed_sums: Array<QM31>,
}

// `Serde` cannot be derived for `Array<QM31>` (the core `Serde<Array<T>>` impl's bounds are not
// satisfied by `QM31`), so the array is serialized element-by-element. This matches the prover,
// which serializes the flattened `Vec<QM31>` of claimed sums.
pub impl CairoInteractionClaimSerde of Serde<CairoInteractionClaim> {
    fn serialize(self: @CairoInteractionClaim, ref output: Array<felt252>) {
        let len: u64 = self.claimed_sums.len().into();
        Serde::serialize(@len, ref output);
        for claimed_sum in self.claimed_sums.span() {
            Serde::serialize(claimed_sum, ref output);
        }
    }

    fn deserialize(ref serialized: Span<felt252>) -> Option<CairoInteractionClaim> {
        let len: u64 = Serde::deserialize(ref serialized)?;
        let mut claimed_sums = array![];
        for _ in 0..len {
            claimed_sums.append(Serde::deserialize(ref serialized)?);
        }
        Some(CairoInteractionClaim { claimed_sums })
    }
}

#[generate_trait]
pub impl CairoInteractionClaimImpl of CairoInteractionClaimTrace {
    fn mix_into(self: @CairoInteractionClaim, ref channel: Channel) {
        channel.mix_felts(self.claimed_sums.span());
    }
}

pub fn lookup_sum(
    claim: @CairoClaim, elements: @CommonLookupElements, interaction_claim: @CairoInteractionClaim,
) -> QM31 {
    let mut sum = claim.public_data.logup_sum(elements);
    for claimed_sum in interaction_claim.claimed_sums.span() {
        sum += *claimed_sum;
    }
    sum
}
