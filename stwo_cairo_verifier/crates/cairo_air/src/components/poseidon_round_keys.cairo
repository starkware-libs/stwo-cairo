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

const LOG_SIZE: u32 = 6;

#[derive(Drop, Serde, Copy)]
pub struct Claim {}

#[generate_trait]
pub impl ClaimImpl of ClaimTrait {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = LOG_SIZE;
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = ArrayImpl::new_repeated(1, log_size).span();
        let interaction_log_sizes = ArrayImpl::new_repeated(QM31_EXTENSION_DEGREE, log_size).span();
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
    pub poseidon_round_keys_lookup_elements: crate::PoseidonRoundKeysElements,
}

pub impl CairoComponentImpl of CairoComponent<Component> {
    fn mask_points(
        self: @Component,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let log_size = LOG_SIZE;
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
        let PoseidonRoundKeys_z = *self.poseidon_round_keys_lookup_elements.z;
        let mut poseidon_round_keys_alpha_powers = self
            .poseidon_round_keys_lookup_elements
            .alpha_powers
            .span();
        let PoseidonRoundKeys_alpha0 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha1 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha2 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha3 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha4 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha5 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha6 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha7 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha8 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha9 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha10 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha11 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha12 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha13 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha14 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha15 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha16 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha17 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha18 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha19 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha20 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha21 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha22 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha23 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha24 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha25 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha26 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha27 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha28 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha29 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha30 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();

        let log_size = LOG_SIZE;

        let claimed_sum = *self.interaction_claim.claimed_sum;

        let params = constraints::ConstraintParams {
            column_size: m31(pow2(log_size)),
            PoseidonRoundKeys_alpha0,
            PoseidonRoundKeys_alpha1,
            PoseidonRoundKeys_alpha2,
            PoseidonRoundKeys_alpha3,
            PoseidonRoundKeys_alpha4,
            PoseidonRoundKeys_alpha5,
            PoseidonRoundKeys_alpha6,
            PoseidonRoundKeys_alpha7,
            PoseidonRoundKeys_alpha8,
            PoseidonRoundKeys_alpha9,
            PoseidonRoundKeys_alpha10,
            PoseidonRoundKeys_alpha11,
            PoseidonRoundKeys_alpha12,
            PoseidonRoundKeys_alpha13,
            PoseidonRoundKeys_alpha14,
            PoseidonRoundKeys_alpha15,
            PoseidonRoundKeys_alpha16,
            PoseidonRoundKeys_alpha17,
            PoseidonRoundKeys_alpha18,
            PoseidonRoundKeys_alpha19,
            PoseidonRoundKeys_alpha20,
            PoseidonRoundKeys_alpha21,
            PoseidonRoundKeys_alpha22,
            PoseidonRoundKeys_alpha23,
            PoseidonRoundKeys_alpha24,
            PoseidonRoundKeys_alpha25,
            PoseidonRoundKeys_alpha26,
            PoseidonRoundKeys_alpha27,
            PoseidonRoundKeys_alpha28,
            PoseidonRoundKeys_alpha29,
            PoseidonRoundKeys_alpha30,
            PoseidonRoundKeys_z,
            poseidon_round_keys_0: preprocessed_mask_values
                .get(PreprocessedColumn::PoseidonRoundKeys(0)),
            poseidon_round_keys_1: preprocessed_mask_values
                .get(PreprocessedColumn::PoseidonRoundKeys(1)),
            poseidon_round_keys_2: preprocessed_mask_values
                .get(PreprocessedColumn::PoseidonRoundKeys(2)),
            poseidon_round_keys_3: preprocessed_mask_values
                .get(PreprocessedColumn::PoseidonRoundKeys(3)),
            poseidon_round_keys_4: preprocessed_mask_values
                .get(PreprocessedColumn::PoseidonRoundKeys(4)),
            poseidon_round_keys_5: preprocessed_mask_values
                .get(PreprocessedColumn::PoseidonRoundKeys(5)),
            poseidon_round_keys_6: preprocessed_mask_values
                .get(PreprocessedColumn::PoseidonRoundKeys(6)),
            poseidon_round_keys_7: preprocessed_mask_values
                .get(PreprocessedColumn::PoseidonRoundKeys(7)),
            poseidon_round_keys_8: preprocessed_mask_values
                .get(PreprocessedColumn::PoseidonRoundKeys(8)),
            poseidon_round_keys_9: preprocessed_mask_values
                .get(PreprocessedColumn::PoseidonRoundKeys(9)),
            poseidon_round_keys_10: preprocessed_mask_values
                .get(PreprocessedColumn::PoseidonRoundKeys(10)),
            poseidon_round_keys_11: preprocessed_mask_values
                .get(PreprocessedColumn::PoseidonRoundKeys(11)),
            poseidon_round_keys_12: preprocessed_mask_values
                .get(PreprocessedColumn::PoseidonRoundKeys(12)),
            poseidon_round_keys_13: preprocessed_mask_values
                .get(PreprocessedColumn::PoseidonRoundKeys(13)),
            poseidon_round_keys_14: preprocessed_mask_values
                .get(PreprocessedColumn::PoseidonRoundKeys(14)),
            poseidon_round_keys_15: preprocessed_mask_values
                .get(PreprocessedColumn::PoseidonRoundKeys(15)),
            poseidon_round_keys_16: preprocessed_mask_values
                .get(PreprocessedColumn::PoseidonRoundKeys(16)),
            poseidon_round_keys_17: preprocessed_mask_values
                .get(PreprocessedColumn::PoseidonRoundKeys(17)),
            poseidon_round_keys_18: preprocessed_mask_values
                .get(PreprocessedColumn::PoseidonRoundKeys(18)),
            poseidon_round_keys_19: preprocessed_mask_values
                .get(PreprocessedColumn::PoseidonRoundKeys(19)),
            poseidon_round_keys_20: preprocessed_mask_values
                .get(PreprocessedColumn::PoseidonRoundKeys(20)),
            poseidon_round_keys_21: preprocessed_mask_values
                .get(PreprocessedColumn::PoseidonRoundKeys(21)),
            poseidon_round_keys_22: preprocessed_mask_values
                .get(PreprocessedColumn::PoseidonRoundKeys(22)),
            poseidon_round_keys_23: preprocessed_mask_values
                .get(PreprocessedColumn::PoseidonRoundKeys(23)),
            poseidon_round_keys_24: preprocessed_mask_values
                .get(PreprocessedColumn::PoseidonRoundKeys(24)),
            poseidon_round_keys_25: preprocessed_mask_values
                .get(PreprocessedColumn::PoseidonRoundKeys(25)),
            poseidon_round_keys_26: preprocessed_mask_values
                .get(PreprocessedColumn::PoseidonRoundKeys(26)),
            poseidon_round_keys_27: preprocessed_mask_values
                .get(PreprocessedColumn::PoseidonRoundKeys(27)),
            poseidon_round_keys_28: preprocessed_mask_values
                .get(PreprocessedColumn::PoseidonRoundKeys(28)),
            poseidon_round_keys_29: preprocessed_mask_values
                .get(PreprocessedColumn::PoseidonRoundKeys(29)),
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

