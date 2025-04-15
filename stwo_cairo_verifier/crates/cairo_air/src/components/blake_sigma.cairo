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

const BLAKE_SIGMA_LOG_SIZE: u32 = 4;

#[derive(Drop, Serde, Copy)]
pub struct Claim {}

#[generate_trait]
pub impl ClaimImpl of ClaimTrait {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = BLAKE_SIGMA_LOG_SIZE;
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = ArrayImpl::new_repeated(1, log_size).span();
        let interaction_log_sizes = ArrayImpl::new_repeated(QM31_EXTENSION_DEGREE, log_size).span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {}
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
    pub interaction_claim: InteractionClaim,
    pub blake_round_sigma_lookup_elements: crate::BlakeRoundSigmaElements,
}

pub impl ComponentImpl of CairoComponent<Component> {
    fn mask_points(
        self: @Component,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let log_size = BLAKE_SIGMA_LOG_SIZE;
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
        BLAKE_SIGMA_LOG_SIZE + 1
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
        let log_size = BLAKE_SIGMA_LOG_SIZE;
        let trace_domain = CanonicCosetImpl::new(log_size);
        let domain_vanishing_eval_inv = trace_domain.eval_vanishing(point).inverse();

        let BlakeRoundSigma_z = *self.blake_round_sigma_lookup_elements.z;
        let mut blake_round_sigma_alpha_powers = self
            .blake_round_sigma_lookup_elements
            .alpha_powers
            .span();
        let BlakeRoundSigma_alpha0 = *blake_round_sigma_alpha_powers.pop_front().unwrap();
        let BlakeRoundSigma_alpha1 = *blake_round_sigma_alpha_powers.pop_front().unwrap();
        let BlakeRoundSigma_alpha2 = *blake_round_sigma_alpha_powers.pop_front().unwrap();
        let BlakeRoundSigma_alpha3 = *blake_round_sigma_alpha_powers.pop_front().unwrap();
        let BlakeRoundSigma_alpha4 = *blake_round_sigma_alpha_powers.pop_front().unwrap();
        let BlakeRoundSigma_alpha5 = *blake_round_sigma_alpha_powers.pop_front().unwrap();
        let BlakeRoundSigma_alpha6 = *blake_round_sigma_alpha_powers.pop_front().unwrap();
        let BlakeRoundSigma_alpha7 = *blake_round_sigma_alpha_powers.pop_front().unwrap();
        let BlakeRoundSigma_alpha8 = *blake_round_sigma_alpha_powers.pop_front().unwrap();
        let BlakeRoundSigma_alpha9 = *blake_round_sigma_alpha_powers.pop_front().unwrap();
        let BlakeRoundSigma_alpha10 = *blake_round_sigma_alpha_powers.pop_front().unwrap();
        let BlakeRoundSigma_alpha11 = *blake_round_sigma_alpha_powers.pop_front().unwrap();
        let BlakeRoundSigma_alpha12 = *blake_round_sigma_alpha_powers.pop_front().unwrap();
        let BlakeRoundSigma_alpha13 = *blake_round_sigma_alpha_powers.pop_front().unwrap();
        let BlakeRoundSigma_alpha14 = *blake_round_sigma_alpha_powers.pop_front().unwrap();
        let BlakeRoundSigma_alpha15 = *blake_round_sigma_alpha_powers.pop_front().unwrap();
        let BlakeRoundSigma_alpha16 = *blake_round_sigma_alpha_powers.pop_front().unwrap();

        let claimed_sum = *self.interaction_claim.claimed_sum;

        let params = constraints::ConstraintParams {
            BlakeRoundSigma_z,
            BlakeRoundSigma_alpha0,
            BlakeRoundSigma_alpha1,
            BlakeRoundSigma_alpha2,
            BlakeRoundSigma_alpha3,
            BlakeRoundSigma_alpha4,
            BlakeRoundSigma_alpha5,
            BlakeRoundSigma_alpha6,
            BlakeRoundSigma_alpha7,
            BlakeRoundSigma_alpha8,
            BlakeRoundSigma_alpha9,
            BlakeRoundSigma_alpha10,
            BlakeRoundSigma_alpha11,
            BlakeRoundSigma_alpha12,
            BlakeRoundSigma_alpha13,
            BlakeRoundSigma_alpha14,
            BlakeRoundSigma_alpha15,
            BlakeRoundSigma_alpha16,
            blake_sigma_0: preprocessed_mask_values.get(PreprocessedColumn::BlakeSigma(0)),
            blake_sigma_1: preprocessed_mask_values.get(PreprocessedColumn::BlakeSigma(1)),
            blake_sigma_2: preprocessed_mask_values.get(PreprocessedColumn::BlakeSigma(2)),
            blake_sigma_3: preprocessed_mask_values.get(PreprocessedColumn::BlakeSigma(3)),
            blake_sigma_4: preprocessed_mask_values.get(PreprocessedColumn::BlakeSigma(4)),
            blake_sigma_5: preprocessed_mask_values.get(PreprocessedColumn::BlakeSigma(5)),
            blake_sigma_6: preprocessed_mask_values.get(PreprocessedColumn::BlakeSigma(6)),
            blake_sigma_7: preprocessed_mask_values.get(PreprocessedColumn::BlakeSigma(7)),
            blake_sigma_8: preprocessed_mask_values.get(PreprocessedColumn::BlakeSigma(8)),
            blake_sigma_9: preprocessed_mask_values.get(PreprocessedColumn::BlakeSigma(9)),
            blake_sigma_10: preprocessed_mask_values.get(PreprocessedColumn::BlakeSigma(10)),
            blake_sigma_11: preprocessed_mask_values.get(PreprocessedColumn::BlakeSigma(11)),
            blake_sigma_12: preprocessed_mask_values.get(PreprocessedColumn::BlakeSigma(12)),
            blake_sigma_13: preprocessed_mask_values.get(PreprocessedColumn::BlakeSigma(13)),
            blake_sigma_14: preprocessed_mask_values.get(PreprocessedColumn::BlakeSigma(14)),
            blake_sigma_15: preprocessed_mask_values.get(PreprocessedColumn::BlakeSigma(15)),
            seq: preprocessed_mask_values.get(PreprocessedColumn::Seq(log_size)),
            claimed_sum,
            column_size: m31(pow2(log_size)),
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
