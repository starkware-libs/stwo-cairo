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
}

#[generate_trait]
pub impl ClaimImpl of ClaimTrait {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = *self.log_size;
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = ArrayImpl::new_repeated(20, log_size).span();
        let interaction_log_sizes = ArrayImpl::new_repeated(QM31_EXTENSION_DEGREE * 8, log_size)
            .span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        channel.mix_u64((*self.log_size).into());
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
    pub range_check_felt_252_width_27_lookup_elements: crate::RangeCheckFelt252Width27Elements,
    pub range_check_9_9_lookup_elements: crate::RangeCheck_9_9Elements,
    pub range_check_18_lookup_elements: crate::RangeCheck_18Elements,
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
        let RangeCheckFelt252Width27_z = *self.range_check_felt_252_width_27_lookup_elements.z;
        let mut range_check_felt_252_width_27_alpha_powers = self
            .range_check_felt_252_width_27_lookup_elements
            .alpha_powers
            .span();
        let RangeCheckFelt252Width27_alpha0 = *range_check_felt_252_width_27_alpha_powers
            .pop_front()
            .unwrap();
        let RangeCheckFelt252Width27_alpha1 = *range_check_felt_252_width_27_alpha_powers
            .pop_front()
            .unwrap();
        let RangeCheckFelt252Width27_alpha2 = *range_check_felt_252_width_27_alpha_powers
            .pop_front()
            .unwrap();
        let RangeCheckFelt252Width27_alpha3 = *range_check_felt_252_width_27_alpha_powers
            .pop_front()
            .unwrap();
        let RangeCheckFelt252Width27_alpha4 = *range_check_felt_252_width_27_alpha_powers
            .pop_front()
            .unwrap();
        let RangeCheckFelt252Width27_alpha5 = *range_check_felt_252_width_27_alpha_powers
            .pop_front()
            .unwrap();
        let RangeCheckFelt252Width27_alpha6 = *range_check_felt_252_width_27_alpha_powers
            .pop_front()
            .unwrap();
        let RangeCheckFelt252Width27_alpha7 = *range_check_felt_252_width_27_alpha_powers
            .pop_front()
            .unwrap();
        let RangeCheckFelt252Width27_alpha8 = *range_check_felt_252_width_27_alpha_powers
            .pop_front()
            .unwrap();
        let RangeCheckFelt252Width27_alpha9 = *range_check_felt_252_width_27_alpha_powers
            .pop_front()
            .unwrap();

        let RangeCheck_9_9_z = *self.range_check_9_9_lookup_elements.z;
        let mut range_check_9_bit_9_bit_alpha_powers = self
            .range_check_9_9_lookup_elements
            .alpha_powers
            .span();
        let RangeCheck_9_9_alpha0 = *range_check_9_bit_9_bit_alpha_powers.pop_front().unwrap();
        let RangeCheck_9_9_alpha1 = *range_check_9_bit_9_bit_alpha_powers.pop_front().unwrap();

        let RangeCheck_18_z = *self.range_check_18_lookup_elements.z;
        let mut range_check_18_bit_alpha_powers = self
            .range_check_18_lookup_elements
            .alpha_powers
            .span();
        let RangeCheck_18_alpha0 = *range_check_18_bit_alpha_powers.pop_front().unwrap();

        let log_size = *self.claim.log_size;

        let claimed_sum = *self.interaction_claim.claimed_sum;

        let params = constraints::ConstraintParams {
            column_size: pow2(log_size).try_into().unwrap(),
            RangeCheck_18_z,
            RangeCheck_18_alpha0,
            RangeCheck_9_9_z,
            RangeCheck_9_9_alpha0,
            RangeCheck_9_9_alpha1,
            RangeCheckFelt252Width27_z,
            RangeCheckFelt252Width27_alpha0,
            RangeCheckFelt252Width27_alpha1,
            RangeCheckFelt252Width27_alpha2,
            RangeCheckFelt252Width27_alpha3,
            RangeCheckFelt252Width27_alpha4,
            RangeCheckFelt252Width27_alpha5,
            RangeCheckFelt252Width27_alpha6,
            RangeCheckFelt252Width27_alpha7,
            RangeCheckFelt252Width27_alpha8,
            RangeCheckFelt252Width27_alpha9,
            claimed_sum,
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

