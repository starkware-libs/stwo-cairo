use core::num::traits::Zero;
use stwo_constraint_framework::{
    PreprocessedColumn, PreprocessedColumnSet, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::Invertible;
use stwo_verifier_core::fields::m31::m31;
use stwo_verifier_core::fields::qm31::{QM31, QM31Serde, QM31_EXTENSION_DEGREE};
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::{ArrayImpl, pow2};
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray};
use crate::components::CairoComponent;
use crate::utils::U32Impl;

mod constraints;

#[derive(Drop, Serde, Copy)]
pub struct Claim {
    pub log_size: u32,
    pub range_check96_builtin_segment_start: u32,
}

#[generate_trait]
pub impl ClaimImpl of ClaimTrait {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = *self.log_size;
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = ArrayImpl::new_repeated(12, log_size).span();
        let interaction_log_sizes = ArrayImpl::new_repeated(QM31_EXTENSION_DEGREE * 2, log_size)
            .span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        channel.mix_u64((*self.log_size).into());
        channel.mix_u64((*self.range_check96_builtin_segment_start).into());
    }
}

#[derive(Drop, Serde, Copy)]
pub struct InteractionClaim {
    pub claimed_sum: QM31,
}

#[generate_trait]
pub impl InteractionClaimImpl of InteractionClaimTrait {
    fn mix_into(self: @InteractionClaim, ref channel: Channel) {
        channel.mix_felts([*self.claimed_sum].span());
    }
}


#[derive(Drop)]
pub struct Component {
    pub claim: Claim,
    pub interaction_claim: InteractionClaim,
    pub memory_address_to_id_lookup_elements: crate::MemoryAddressToIdElements,
    pub memory_id_to_big_lookup_elements: crate::MemoryIdToBigElements,
    pub range_check_6_lookup_elements: crate::RangeCheck_6Elements,
}

pub impl CairoComponentImpl of CairoComponent<Component> {
    fn mask_points(
        self: @Component,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let log_size = *self.claim.log_size;
        let trace_gen = CanonicCosetImpl::new(log_size).coset.step_size;
        constraints::mask_points(
            ref preprocessed_column_set,
            ref trace_mask_points,
            ref interaction_trace_mask_points,
            point,
            trace_gen,
            log_size,
        );
    }

    fn max_constraint_log_degree_bound(self: @Component) -> u32 {
        *self.claim.log_size + 1
    }

    fn evaluate_constraints_at_point(
        self: @Component,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        let MemoryAddressToId_z = *self.memory_address_to_id_lookup_elements.z;
        let mut memory_address_to_id_alpha_powers = self
            .memory_address_to_id_lookup_elements
            .alpha_powers
            .span();
        let MemoryAddressToId_alpha0 = *memory_address_to_id_alpha_powers.pop_front().unwrap();
        let MemoryAddressToId_alpha1 = *memory_address_to_id_alpha_powers.pop_front().unwrap();

        let MemoryIdToBig_z = *self.memory_id_to_big_lookup_elements.z;
        let mut memory_id_to_big_alpha_powers = self
            .memory_id_to_big_lookup_elements
            .alpha_powers
            .span();
        let MemoryIdToBig_alpha0 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha1 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha2 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha3 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha4 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha5 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha6 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha7 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha8 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha9 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha10 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha11 = *memory_id_to_big_alpha_powers.pop_front().unwrap();

        let RangeCheck_6_z = *self.range_check_6_lookup_elements.z;
        let mut range_check_6_alpha_powers = self.range_check_6_lookup_elements.alpha_powers.span();
        let RangeCheck_6_alpha0 = *range_check_6_alpha_powers.pop_front().unwrap();

        let log_size = *self.claim.log_size;

        let claimed_sum = *self.interaction_claim.claimed_sum;

        let range_check96_builtin_segment_start = (*self.claim.range_check96_builtin_segment_start)
            .try_into()
            .unwrap();

        let params = constraints::ConstraintParams {
            column_size: pow2(log_size).try_into().unwrap(),
            builtin_segment_start: range_check96_builtin_segment_start,
            MemoryAddressToId_alpha0,
            MemoryAddressToId_alpha1,
            MemoryAddressToId_z,
            MemoryIdToBig_alpha0,
            MemoryIdToBig_alpha1,
            MemoryIdToBig_alpha10,
            MemoryIdToBig_alpha11,
            MemoryIdToBig_alpha2,
            MemoryIdToBig_alpha3,
            MemoryIdToBig_alpha4,
            MemoryIdToBig_alpha5,
            MemoryIdToBig_alpha6,
            MemoryIdToBig_alpha7,
            MemoryIdToBig_alpha8,
            MemoryIdToBig_alpha9,
            MemoryIdToBig_z,
            RangeCheck_6_alpha0,
            RangeCheck_6_z,
            claimed_sum,
            seq: preprocessed_mask_values.get(PreprocessedColumn::Seq(log_size)),
        };

        let trace_domain = CanonicCosetImpl::new(log_size);
        let vanish_eval = trace_domain.eval_vanishing(point);

        constraints::evaluate_constraints_at_point(
            ref sum,
            ref trace_mask_values,
            ref interaction_trace_mask_values,
            params,
            random_coeff,
            vanish_eval.inverse(),
        );
    }
}

