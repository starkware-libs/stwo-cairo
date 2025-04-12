use core::num::traits::Zero;
use stwo_constraint_framework::{
    PreprocessedColumn, PreprocessedColumnSet, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::Invertible;
use stwo_verifier_core::fields::m31::m31;
use stwo_verifier_core::fields::qm31::{QM31, QM31_EXTENSION_DEGREE};
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::{ArrayImpl, pow2};
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray};
use crate::components::CairoComponent;
use crate::utils::U32Impl;

mod constraints;

pub const ELEM_BITS: u32 = 12;

pub const EXPAND_BITS: u32 = 2;

pub const LIMB_BITS: u32 = ELEM_BITS - EXPAND_BITS;

pub const LOG_SIZE: u32 = (ELEM_BITS - EXPAND_BITS) * 2;

#[derive(Drop, Serde, Copy)]
pub struct Claim {}

#[generate_trait]
pub impl ClaimImpl of ClaimTrait {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let preprocessed_log_sizes = array![LOG_SIZE].span();
        let trace_log_sizes = array![LOG_SIZE].span();
        let interaction_log_sizes = ArrayImpl::new_repeated(QM31_EXTENSION_DEGREE, LOG_SIZE).span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        channel.mix_u64(LOG_SIZE.into());
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
    pub verify_bitwise_xor_12_lookup_elements: crate::VerifyBitwiseXor12BitElements,
}

pub impl CairoComponentImpl of CairoComponent<Component> {
    fn mask_points(
        self: @Component,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let trace_gen = CanonicCosetImpl::new(LOG_SIZE).coset.step_size;
        constraints::mask_points(
            ref preprocessed_column_set,
            ref trace_mask_points,
            ref interaction_trace_mask_points,
            point,
            trace_gen,
            LOG_SIZE,
        );
    }

    fn max_constraint_log_degree_bound(self: @Component) -> u32 {
        LOG_SIZE + 1
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
        let VerifyBitwiseXor_12_z = *self.verify_bitwise_xor_12_lookup_elements.z;
        let mut verify_bitwise_xor_12_alpha_powers = self
            .verify_bitwise_xor_12_lookup_elements
            .alpha_powers
            .span();
        let VerifyBitwiseXor_12_alpha0 = *verify_bitwise_xor_12_alpha_powers.pop_front().unwrap();
        let VerifyBitwiseXor_12_alpha1 = *verify_bitwise_xor_12_alpha_powers.pop_front().unwrap();
        let VerifyBitwiseXor_12_alpha2 = *verify_bitwise_xor_12_alpha_powers.pop_front().unwrap();

        let claimed_sum = *self.interaction_claim.claimed_sum;

        let params = constraints::ConstraintParams {
            VerifyBitwiseXor_12_alpha0,
            VerifyBitwiseXor_12_alpha1,
            VerifyBitwiseXor_12_alpha2,
            VerifyBitwiseXor_12_z,
            claimed_sum,
            bitwise_xor_10_0: preprocessed_mask_values.get(PreprocessedColumn::Xor((LIMB_BITS, 0))),
            bitwise_xor_10_1: preprocessed_mask_values.get(PreprocessedColumn::Xor((LIMB_BITS, 1))),
            bitwise_xor_10_2: preprocessed_mask_values.get(PreprocessedColumn::Xor((LIMB_BITS, 2))),
            column_size: m31(pow2(LOG_SIZE)),
        };

        let trace_domain = CanonicCosetImpl::new(LOG_SIZE);
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

