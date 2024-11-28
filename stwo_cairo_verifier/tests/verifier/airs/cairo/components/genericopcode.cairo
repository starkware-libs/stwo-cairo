use stwo_cairo_verifier::channel::{Channel, ChannelImpl};
use stwo_cairo_verifier::circle::CirclePoint;
use stwo_cairo_verifier::fields::qm31::{QM31, QM31_EXTENSION_DEGREE};
use stwo_cairo_verifier::poly::circle::CanonicCosetImpl;
use stwo_cairo_verifier::utils::ArrayImpl;
use stwo_cairo_verifier::{ColumnArray, TreeArray};
use super::super::super::super::constraint_framework::ClaimedPrefixSum;
use super::super::utils::U32Impl;

mod constraints;

pub const LOG_N_LANES: u32 = 4;

#[derive(Drop, Serde, Copy)]
pub struct Claim {
    pub n_calls: usize,
}

#[generate_trait]
pub impl ClaimImpl of ClaimTrait {
    fn log_size(self: @Claim) -> u32 {
        core::cmp::max((*self.n_calls).next_power_of_two().ilog2(), LOG_N_LANES)
    }

    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = self.log_size();
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = ArrayImpl::new_repeated(229, log_size).span();
        let interaction_log_sizes = ArrayImpl::new_repeated(QM31_EXTENSION_DEGREE * 65, log_size)
            .span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        channel.mix_nonce((*self.n_calls).into());
    }
}

#[derive(Drop, Serde, Copy)]
pub struct InteractionClaim {
    pub total_sum: QM31,
    pub claimed_sum: Option<ClaimedPrefixSum>,
}

#[generate_trait]
pub impl InteractionClaimImpl of InteractionClaimTrait {
    fn mix_into(self: @InteractionClaim, ref channel: Channel) {
        channel.mix_felts([*self.total_sum].span());

        if let Option::Some((sum_at_index, index)) = *self.claimed_sum {
            channel.mix_felts([sum_at_index].span());
            channel.mix_nonce(index.into());
        }
    }
}

#[derive(Drop)]
pub struct Component {
    pub claim: Claim,
    pub interaction_claim: InteractionClaim,
    pub memoryaddresstoid_lookup_elements: super::super::AddrToIdElements,
    pub memoryidtobig_lookup_elements: super::super::IdToValueElements,
    pub range_check_19_lookup_elements: super::super::RangeCheck19BitElements,
    pub range_check_9_9_lookup_elements: super::super::RangeCheck9Bit9BitElements,
    pub verifyinstruction_lookup_elements: super::super::VerifyInstructionElements,
    pub opcodes_lookup_elements: super::super::VmElements,
}

#[generate_trait]
pub impl ComponentImpl of ComponentTrait {
    fn mask_points(
        self: @Component,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let claimed_sum_offset = *self.claim.n_calls;
        let trace_gen = CanonicCosetImpl::new(self.claim.log_size()).coset.step_size;
        constraints::mask_points(
            ref trace_mask_points,
            ref interaction_trace_mask_points,
            point,
            trace_gen,
            claimed_sum_offset,
        );
    }

    fn max_constraint_log_degree_bound(self: @Component) -> u32 {
        self.claim.log_size() + 1
    }
}
