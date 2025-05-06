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
    log_size: u32,
}

#[generate_trait]
pub impl ClaimImpl of ClaimTrait {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = *self.log_size;
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = ArrayImpl::new_repeated(53, log_size).span();
        let interaction_log_sizes = ArrayImpl::new_repeated(QM31_EXTENSION_DEGREE * 9, log_size)
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
    pub blake_g_lookup_elements: crate::BlakeGElements,
    pub verify_bitwise_xor_12_lookup_elements: crate::VerifyBitwiseXor_12Elements,
    pub verify_bitwise_xor_4_lookup_elements: crate::VerifyBitwiseXor_4Elements,
    pub verify_bitwise_xor_7_lookup_elements: crate::VerifyBitwiseXor_7Elements,
    pub verify_bitwise_xor_8_lookup_elements: crate::VerifyBitwiseXor_8Elements,
    pub verify_bitwise_xor_9_lookup_elements: crate::VerifyBitwiseXor_9Elements,
}

pub impl ComponentImpl of CairoComponent<Component> {
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
        let log_size = *self.claim.log_size;
        let trace_domain = CanonicCosetImpl::new(log_size);
        let domain_vanishing_eval_inv = trace_domain.eval_vanishing(point).inverse();

        let VerifyBitwiseXor_12_z = *self.verify_bitwise_xor_12_lookup_elements.z;
        let mut verify_bitwise_xor_12_alpha_powers = self
            .verify_bitwise_xor_12_lookup_elements
            .alpha_powers
            .span();
        let VerifyBitwiseXor_12_alpha0 = *verify_bitwise_xor_12_alpha_powers.pop_front().unwrap();
        let VerifyBitwiseXor_12_alpha1 = *verify_bitwise_xor_12_alpha_powers.pop_front().unwrap();
        let VerifyBitwiseXor_12_alpha2 = *verify_bitwise_xor_12_alpha_powers.pop_front().unwrap();

        let VerifyBitwiseXor_4_z = *self.verify_bitwise_xor_4_lookup_elements.z;
        let mut verify_bitwise_xor_4_alpha_powers = self
            .verify_bitwise_xor_4_lookup_elements
            .alpha_powers
            .span();
        let VerifyBitwiseXor_4_alpha0 = *verify_bitwise_xor_4_alpha_powers.pop_front().unwrap();
        let VerifyBitwiseXor_4_alpha1 = *verify_bitwise_xor_4_alpha_powers.pop_front().unwrap();
        let VerifyBitwiseXor_4_alpha2 = *verify_bitwise_xor_4_alpha_powers.pop_front().unwrap();

        let VerifyBitwiseXor_7_z = *self.verify_bitwise_xor_7_lookup_elements.z;
        let mut verify_bitwise_xor_7_alpha_powers = self
            .verify_bitwise_xor_7_lookup_elements
            .alpha_powers
            .span();
        let VerifyBitwiseXor_7_alpha0 = *verify_bitwise_xor_7_alpha_powers.pop_front().unwrap();
        let VerifyBitwiseXor_7_alpha1 = *verify_bitwise_xor_7_alpha_powers.pop_front().unwrap();
        let VerifyBitwiseXor_7_alpha2 = *verify_bitwise_xor_7_alpha_powers.pop_front().unwrap();

        let VerifyBitwiseXor_8_z = *self.verify_bitwise_xor_8_lookup_elements.z;
        let mut verify_bitwise_xor_8_alpha_powers = self
            .verify_bitwise_xor_8_lookup_elements
            .alpha_powers
            .span();
        let VerifyBitwiseXor_8_alpha0 = *verify_bitwise_xor_8_alpha_powers.pop_front().unwrap();
        let VerifyBitwiseXor_8_alpha1 = *verify_bitwise_xor_8_alpha_powers.pop_front().unwrap();
        let VerifyBitwiseXor_8_alpha2 = *verify_bitwise_xor_8_alpha_powers.pop_front().unwrap();

        let VerifyBitwiseXor_9_z = *self.verify_bitwise_xor_9_lookup_elements.z;
        let mut verify_bitwise_xor_9_alpha_powers = self
            .verify_bitwise_xor_9_lookup_elements
            .alpha_powers
            .span();
        let VerifyBitwiseXor_9_alpha0 = *verify_bitwise_xor_9_alpha_powers.pop_front().unwrap();
        let VerifyBitwiseXor_9_alpha1 = *verify_bitwise_xor_9_alpha_powers.pop_front().unwrap();
        let VerifyBitwiseXor_9_alpha2 = *verify_bitwise_xor_9_alpha_powers.pop_front().unwrap();

        let BlakeG_z = *self.blake_g_lookup_elements.z;
        let mut blake_g_alpha_powers = self.blake_g_lookup_elements.alpha_powers.span();
        let BlakeG_alpha0 = *blake_g_alpha_powers.pop_front().unwrap();
        let BlakeG_alpha1 = *blake_g_alpha_powers.pop_front().unwrap();
        let BlakeG_alpha2 = *blake_g_alpha_powers.pop_front().unwrap();
        let BlakeG_alpha3 = *blake_g_alpha_powers.pop_front().unwrap();
        let BlakeG_alpha4 = *blake_g_alpha_powers.pop_front().unwrap();
        let BlakeG_alpha5 = *blake_g_alpha_powers.pop_front().unwrap();
        let BlakeG_alpha6 = *blake_g_alpha_powers.pop_front().unwrap();
        let BlakeG_alpha7 = *blake_g_alpha_powers.pop_front().unwrap();
        let BlakeG_alpha8 = *blake_g_alpha_powers.pop_front().unwrap();
        let BlakeG_alpha9 = *blake_g_alpha_powers.pop_front().unwrap();
        let BlakeG_alpha10 = *blake_g_alpha_powers.pop_front().unwrap();
        let BlakeG_alpha11 = *blake_g_alpha_powers.pop_front().unwrap();
        let BlakeG_alpha12 = *blake_g_alpha_powers.pop_front().unwrap();
        let BlakeG_alpha13 = *blake_g_alpha_powers.pop_front().unwrap();
        let BlakeG_alpha14 = *blake_g_alpha_powers.pop_front().unwrap();
        let BlakeG_alpha15 = *blake_g_alpha_powers.pop_front().unwrap();
        let BlakeG_alpha16 = *blake_g_alpha_powers.pop_front().unwrap();
        let BlakeG_alpha17 = *blake_g_alpha_powers.pop_front().unwrap();
        let BlakeG_alpha18 = *blake_g_alpha_powers.pop_front().unwrap();
        let BlakeG_alpha19 = *blake_g_alpha_powers.pop_front().unwrap();

        let claimed_sum = *self.interaction_claim.claimed_sum;

        let params = constraints::ConstraintParams {
            VerifyBitwiseXor_12_z,
            VerifyBitwiseXor_12_alpha0,
            VerifyBitwiseXor_12_alpha1,
            VerifyBitwiseXor_12_alpha2,
            VerifyBitwiseXor_4_z,
            VerifyBitwiseXor_4_alpha0,
            VerifyBitwiseXor_4_alpha1,
            VerifyBitwiseXor_4_alpha2,
            VerifyBitwiseXor_7_z,
            VerifyBitwiseXor_7_alpha0,
            VerifyBitwiseXor_7_alpha1,
            VerifyBitwiseXor_7_alpha2,
            VerifyBitwiseXor_8_z,
            VerifyBitwiseXor_8_alpha0,
            VerifyBitwiseXor_8_alpha1,
            VerifyBitwiseXor_8_alpha2,
            VerifyBitwiseXor_9_z,
            VerifyBitwiseXor_9_alpha0,
            VerifyBitwiseXor_9_alpha1,
            VerifyBitwiseXor_9_alpha2,
            BlakeG_z,
            BlakeG_alpha0,
            BlakeG_alpha1,
            BlakeG_alpha2,
            BlakeG_alpha3,
            BlakeG_alpha4,
            BlakeG_alpha5,
            BlakeG_alpha6,
            BlakeG_alpha7,
            BlakeG_alpha8,
            BlakeG_alpha9,
            BlakeG_alpha10,
            BlakeG_alpha11,
            BlakeG_alpha12,
            BlakeG_alpha13,
            BlakeG_alpha14,
            BlakeG_alpha15,
            BlakeG_alpha16,
            BlakeG_alpha17,
            BlakeG_alpha18,
            BlakeG_alpha19,
            claimed_sum,
            column_size: pow2(log_size).try_into().unwrap(),
        };

        constraints::evaluate_constraints_at_point(
            ref sum,
            ref trace_mask_values,
            ref interaction_trace_mask_values,
            params,
            random_coeff,
            domain_vanishing_eval_inv,
        )
    }
}
