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

pub const LOG_SIZE: u32 = 23;

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
    pub pedersen_points_table_lookup_elements: crate::PedersenPointsTableElements,
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
        let PedersenPointsTable_z = *self.pedersen_points_table_lookup_elements.z;
        let mut pedersen_points_table_alpha_powers = self
            .pedersen_points_table_lookup_elements
            .alpha_powers
            .span();
        let PedersenPointsTable_alpha0 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha1 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha2 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha3 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha4 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha5 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha6 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha7 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha8 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha9 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha10 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha11 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha12 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha13 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha14 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha15 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha16 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha17 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha18 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha19 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha20 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha21 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha22 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha23 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha24 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha25 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha26 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha27 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha28 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha29 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha30 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha31 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha32 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha33 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha34 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha35 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha36 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha37 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha38 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha39 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha40 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha41 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha42 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha43 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha44 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha45 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha46 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha47 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha48 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha49 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha50 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha51 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha52 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha53 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha54 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha55 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let PedersenPointsTable_alpha56 = *pedersen_points_table_alpha_powers.pop_front().unwrap();
        let log_size = LOG_SIZE;

        let claimed_sum = *self.interaction_claim.claimed_sum;

        let params = constraints::ConstraintParams {
            column_size: m31(pow2(log_size)),
            PedersenPointsTable_alpha0,
            PedersenPointsTable_alpha1,
            PedersenPointsTable_alpha2,
            PedersenPointsTable_alpha3,
            PedersenPointsTable_alpha4,
            PedersenPointsTable_alpha5,
            PedersenPointsTable_alpha6,
            PedersenPointsTable_alpha7,
            PedersenPointsTable_alpha8,
            PedersenPointsTable_alpha9,
            PedersenPointsTable_alpha10,
            PedersenPointsTable_alpha11,
            PedersenPointsTable_alpha12,
            PedersenPointsTable_alpha13,
            PedersenPointsTable_alpha14,
            PedersenPointsTable_alpha15,
            PedersenPointsTable_alpha16,
            PedersenPointsTable_alpha17,
            PedersenPointsTable_alpha18,
            PedersenPointsTable_alpha19,
            PedersenPointsTable_alpha20,
            PedersenPointsTable_alpha21,
            PedersenPointsTable_alpha22,
            PedersenPointsTable_alpha23,
            PedersenPointsTable_alpha24,
            PedersenPointsTable_alpha25,
            PedersenPointsTable_alpha26,
            PedersenPointsTable_alpha27,
            PedersenPointsTable_alpha28,
            PedersenPointsTable_alpha29,
            PedersenPointsTable_alpha30,
            PedersenPointsTable_alpha31,
            PedersenPointsTable_alpha32,
            PedersenPointsTable_alpha33,
            PedersenPointsTable_alpha34,
            PedersenPointsTable_alpha35,
            PedersenPointsTable_alpha36,
            PedersenPointsTable_alpha37,
            PedersenPointsTable_alpha38,
            PedersenPointsTable_alpha39,
            PedersenPointsTable_alpha40,
            PedersenPointsTable_alpha41,
            PedersenPointsTable_alpha42,
            PedersenPointsTable_alpha43,
            PedersenPointsTable_alpha44,
            PedersenPointsTable_alpha45,
            PedersenPointsTable_alpha46,
            PedersenPointsTable_alpha47,
            PedersenPointsTable_alpha48,
            PedersenPointsTable_alpha49,
            PedersenPointsTable_alpha50,
            PedersenPointsTable_alpha51,
            PedersenPointsTable_alpha52,
            PedersenPointsTable_alpha53,
            PedersenPointsTable_alpha54,
            PedersenPointsTable_alpha55,
            PedersenPointsTable_alpha56,
            PedersenPointsTable_z,
            claimed_sum,
            pedersen_points_0: preprocessed_mask_values.get(PreprocessedColumn::PedersenPoints(0)),
            pedersen_points_1: preprocessed_mask_values.get(PreprocessedColumn::PedersenPoints(1)),
            pedersen_points_2: preprocessed_mask_values.get(PreprocessedColumn::PedersenPoints(2)),
            pedersen_points_3: preprocessed_mask_values.get(PreprocessedColumn::PedersenPoints(3)),
            pedersen_points_4: preprocessed_mask_values.get(PreprocessedColumn::PedersenPoints(4)),
            pedersen_points_5: preprocessed_mask_values.get(PreprocessedColumn::PedersenPoints(5)),
            pedersen_points_6: preprocessed_mask_values.get(PreprocessedColumn::PedersenPoints(6)),
            pedersen_points_7: preprocessed_mask_values.get(PreprocessedColumn::PedersenPoints(7)),
            pedersen_points_8: preprocessed_mask_values.get(PreprocessedColumn::PedersenPoints(8)),
            pedersen_points_9: preprocessed_mask_values.get(PreprocessedColumn::PedersenPoints(9)),
            pedersen_points_10: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(10)),
            pedersen_points_11: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(11)),
            pedersen_points_12: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(12)),
            pedersen_points_13: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(13)),
            pedersen_points_14: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(14)),
            pedersen_points_15: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(15)),
            pedersen_points_16: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(16)),
            pedersen_points_17: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(17)),
            pedersen_points_18: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(18)),
            pedersen_points_19: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(19)),
            pedersen_points_20: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(20)),
            pedersen_points_21: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(21)),
            pedersen_points_22: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(22)),
            pedersen_points_23: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(23)),
            pedersen_points_24: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(24)),
            pedersen_points_25: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(25)),
            pedersen_points_26: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(26)),
            pedersen_points_27: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(27)),
            pedersen_points_28: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(28)),
            pedersen_points_29: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(29)),
            pedersen_points_30: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(30)),
            pedersen_points_31: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(31)),
            pedersen_points_32: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(32)),
            pedersen_points_33: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(33)),
            pedersen_points_34: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(34)),
            pedersen_points_35: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(35)),
            pedersen_points_36: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(36)),
            pedersen_points_37: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(37)),
            pedersen_points_38: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(38)),
            pedersen_points_39: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(39)),
            pedersen_points_40: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(40)),
            pedersen_points_41: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(41)),
            pedersen_points_42: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(42)),
            pedersen_points_43: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(43)),
            pedersen_points_44: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(44)),
            pedersen_points_45: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(45)),
            pedersen_points_46: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(46)),
            pedersen_points_47: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(47)),
            pedersen_points_48: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(48)),
            pedersen_points_49: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(49)),
            pedersen_points_50: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(50)),
            pedersen_points_51: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(51)),
            pedersen_points_52: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(52)),
            pedersen_points_53: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(53)),
            pedersen_points_54: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(54)),
            pedersen_points_55: preprocessed_mask_values
                .get(PreprocessedColumn::PedersenPoints(55)),
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

