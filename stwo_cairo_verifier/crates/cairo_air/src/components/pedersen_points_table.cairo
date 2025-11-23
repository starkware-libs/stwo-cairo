// This file was created by the AIR team.

use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 1;
const LOG_SIZE: u32 = 23;

#[derive(Drop, Serde, Copy)]
pub struct Claim {}

pub impl ClaimImpl of ClaimTrait<Claim> {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = LOG_SIZE;
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = [log_size; N_TRACE_COLUMNS].span();
        let interaction_log_sizes = [log_size; 4].span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {}

    fn accumulate_relation_uses(self: @Claim, ref relation_uses: RelationUsesDict) {}
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

pub impl NewComponentImpl of NewComponent<Component> {
    type Claim = Claim;
    type InteractionClaim = InteractionClaim;

    fn new(
        claim: @Claim,
        interaction_claim: @InteractionClaim,
        interaction_elements: @CairoInteractionElements,
    ) -> Component {
        Component {
            claim: *claim,
            interaction_claim: *interaction_claim,
            pedersen_points_table_lookup_elements: interaction_elements
                .pedersen_points_table
                .clone(),
        }
    }
}

pub impl CairoComponentImpl of CairoComponent<Component> {
    fn evaluate_constraints_at_point(
        self: @Component,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        let log_size = LOG_SIZE;
        let trace_domain = CanonicCosetImpl::new(log_size);
        let domain_vanishing_eval_inv = trace_domain.eval_vanishing(point).inverse();
        let claimed_sum = *self.interaction_claim.claimed_sum;
        let column_size = m31(pow2(log_size));
        let mut pedersen_points_table_sum_0: QM31 = Zero::zero();
        let seq_23 = preprocessed_mask_values.get_and_mark_used(SEQ_23_IDX);
        let pedersen_points_0 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_0_IDX);
        let pedersen_points_1 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_1_IDX);
        let pedersen_points_2 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_2_IDX);
        let pedersen_points_3 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_3_IDX);
        let pedersen_points_4 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_4_IDX);
        let pedersen_points_5 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_5_IDX);
        let pedersen_points_6 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_6_IDX);
        let pedersen_points_7 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_7_IDX);
        let pedersen_points_8 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_8_IDX);
        let pedersen_points_9 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_9_IDX);
        let pedersen_points_10 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_10_IDX);
        let pedersen_points_11 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_11_IDX);
        let pedersen_points_12 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_12_IDX);
        let pedersen_points_13 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_13_IDX);
        let pedersen_points_14 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_14_IDX);
        let pedersen_points_15 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_15_IDX);
        let pedersen_points_16 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_16_IDX);
        let pedersen_points_17 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_17_IDX);
        let pedersen_points_18 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_18_IDX);
        let pedersen_points_19 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_19_IDX);
        let pedersen_points_20 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_20_IDX);
        let pedersen_points_21 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_21_IDX);
        let pedersen_points_22 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_22_IDX);
        let pedersen_points_23 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_23_IDX);
        let pedersen_points_24 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_24_IDX);
        let pedersen_points_25 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_25_IDX);
        let pedersen_points_26 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_26_IDX);
        let pedersen_points_27 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_27_IDX);
        let pedersen_points_28 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_28_IDX);
        let pedersen_points_29 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_29_IDX);
        let pedersen_points_30 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_30_IDX);
        let pedersen_points_31 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_31_IDX);
        let pedersen_points_32 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_32_IDX);
        let pedersen_points_33 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_33_IDX);
        let pedersen_points_34 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_34_IDX);
        let pedersen_points_35 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_35_IDX);
        let pedersen_points_36 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_36_IDX);
        let pedersen_points_37 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_37_IDX);
        let pedersen_points_38 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_38_IDX);
        let pedersen_points_39 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_39_IDX);
        let pedersen_points_40 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_40_IDX);
        let pedersen_points_41 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_41_IDX);
        let pedersen_points_42 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_42_IDX);
        let pedersen_points_43 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_43_IDX);
        let pedersen_points_44 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_44_IDX);
        let pedersen_points_45 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_45_IDX);
        let pedersen_points_46 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_46_IDX);
        let pedersen_points_47 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_47_IDX);
        let pedersen_points_48 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_48_IDX);
        let pedersen_points_49 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_49_IDX);
        let pedersen_points_50 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_50_IDX);
        let pedersen_points_51 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_51_IDX);
        let pedersen_points_52 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_52_IDX);
        let pedersen_points_53 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_53_IDX);
        let pedersen_points_54 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_54_IDX);
        let pedersen_points_55 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_55_IDX);

        let [enabler]: [Span<QM31>; 1] = (*trace_mask_values.multi_pop_front().unwrap()).unbox();
        let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        pedersen_points_table_sum_0 = self
            .pedersen_points_table_lookup_elements
            .combine_qm31(
                [
                    seq_23, pedersen_points_0, pedersen_points_1, pedersen_points_2,
                    pedersen_points_3, pedersen_points_4, pedersen_points_5, pedersen_points_6,
                    pedersen_points_7, pedersen_points_8, pedersen_points_9, pedersen_points_10,
                    pedersen_points_11, pedersen_points_12, pedersen_points_13, pedersen_points_14,
                    pedersen_points_15, pedersen_points_16, pedersen_points_17, pedersen_points_18,
                    pedersen_points_19, pedersen_points_20, pedersen_points_21, pedersen_points_22,
                    pedersen_points_23, pedersen_points_24, pedersen_points_25, pedersen_points_26,
                    pedersen_points_27, pedersen_points_28, pedersen_points_29, pedersen_points_30,
                    pedersen_points_31, pedersen_points_32, pedersen_points_33, pedersen_points_34,
                    pedersen_points_35, pedersen_points_36, pedersen_points_37, pedersen_points_38,
                    pedersen_points_39, pedersen_points_40, pedersen_points_41, pedersen_points_42,
                    pedersen_points_43, pedersen_points_44, pedersen_points_45, pedersen_points_46,
                    pedersen_points_47, pedersen_points_48, pedersen_points_49, pedersen_points_50,
                    pedersen_points_51, pedersen_points_52, pedersen_points_53, pedersen_points_54,
                    pedersen_points_55,
                ],
            );

        lookup_constraints(
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
            claimed_sum,
            enabler,
            column_size,
            ref interaction_trace_mask_values,
            pedersen_points_table_sum_0,
        );
    }
}


fn lookup_constraints(
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
    claimed_sum: QM31,
    enabler: QM31,
    column_size: M31,
    ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
    pedersen_points_table_sum_0: QM31,
) {
    let [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3]: [Span<QM31>; 4] =
        (*interaction_trace_mask_values
        .multi_pop_front()
        .unwrap())
        .unbox();

    let [trace_2_col0_neg1, trace_2_col0]: [QM31; 2] = (*trace_2_col0.try_into().unwrap()).unbox();
    let [trace_2_col1_neg1, trace_2_col1]: [QM31; 2] = (*trace_2_col1.try_into().unwrap()).unbox();
    let [trace_2_col2_neg1, trace_2_col2]: [QM31; 2] = (*trace_2_col2.try_into().unwrap()).unbox();
    let [trace_2_col3_neg1, trace_2_col3]: [QM31; 2] = (*trace_2_col3.try_into().unwrap()).unbox();

    core::internal::revoke_ap_tracking();

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col0_neg1, trace_2_col1_neg1, trace_2_col2_neg1, trace_2_col3_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * pedersen_points_table_sum_0)
        + enabler)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
}
