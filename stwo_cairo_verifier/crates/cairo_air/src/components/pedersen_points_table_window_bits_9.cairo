// This file was created by the AIR team.

use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 1;
const LOG_SIZE: u32 = 15;

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
    pub common_lookup_elements: CommonLookupElements,
}

pub impl NewComponentImpl of NewComponent<Component> {
    type Claim = Claim;
    type InteractionClaim = InteractionClaim;

    fn new(
        claim: @Claim,
        interaction_claim: @InteractionClaim,
        common_lookup_elements: @CommonLookupElements,
    ) -> Component {
        Component {
            claim: *claim,
            interaction_claim: *interaction_claim,
            common_lookup_elements: common_lookup_elements.clone(),
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
        let mut pedersen_points_table_window_bits_9_sum_0: QM31 = Zero::zero();
        let seq_15 = preprocessed_mask_values.get_and_mark_used(SEQ_15_IDX);
        let pedersen_points_small_0 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_0_IDX);
        let pedersen_points_small_1 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_1_IDX);
        let pedersen_points_small_2 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_2_IDX);
        let pedersen_points_small_3 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_3_IDX);
        let pedersen_points_small_4 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_4_IDX);
        let pedersen_points_small_5 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_5_IDX);
        let pedersen_points_small_6 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_6_IDX);
        let pedersen_points_small_7 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_7_IDX);
        let pedersen_points_small_8 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_8_IDX);
        let pedersen_points_small_9 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_9_IDX);
        let pedersen_points_small_10 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_10_IDX);
        let pedersen_points_small_11 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_11_IDX);
        let pedersen_points_small_12 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_12_IDX);
        let pedersen_points_small_13 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_13_IDX);
        let pedersen_points_small_14 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_14_IDX);
        let pedersen_points_small_15 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_15_IDX);
        let pedersen_points_small_16 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_16_IDX);
        let pedersen_points_small_17 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_17_IDX);
        let pedersen_points_small_18 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_18_IDX);
        let pedersen_points_small_19 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_19_IDX);
        let pedersen_points_small_20 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_20_IDX);
        let pedersen_points_small_21 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_21_IDX);
        let pedersen_points_small_22 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_22_IDX);
        let pedersen_points_small_23 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_23_IDX);
        let pedersen_points_small_24 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_24_IDX);
        let pedersen_points_small_25 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_25_IDX);
        let pedersen_points_small_26 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_26_IDX);
        let pedersen_points_small_27 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_27_IDX);
        let pedersen_points_small_28 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_28_IDX);
        let pedersen_points_small_29 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_29_IDX);
        let pedersen_points_small_30 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_30_IDX);
        let pedersen_points_small_31 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_31_IDX);
        let pedersen_points_small_32 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_32_IDX);
        let pedersen_points_small_33 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_33_IDX);
        let pedersen_points_small_34 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_34_IDX);
        let pedersen_points_small_35 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_35_IDX);
        let pedersen_points_small_36 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_36_IDX);
        let pedersen_points_small_37 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_37_IDX);
        let pedersen_points_small_38 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_38_IDX);
        let pedersen_points_small_39 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_39_IDX);
        let pedersen_points_small_40 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_40_IDX);
        let pedersen_points_small_41 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_41_IDX);
        let pedersen_points_small_42 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_42_IDX);
        let pedersen_points_small_43 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_43_IDX);
        let pedersen_points_small_44 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_44_IDX);
        let pedersen_points_small_45 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_45_IDX);
        let pedersen_points_small_46 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_46_IDX);
        let pedersen_points_small_47 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_47_IDX);
        let pedersen_points_small_48 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_48_IDX);
        let pedersen_points_small_49 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_49_IDX);
        let pedersen_points_small_50 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_50_IDX);
        let pedersen_points_small_51 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_51_IDX);
        let pedersen_points_small_52 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_52_IDX);
        let pedersen_points_small_53 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_53_IDX);
        let pedersen_points_small_54 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_54_IDX);
        let pedersen_points_small_55 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_55_IDX);

        let [pedersen_points_table_window_bits_9_multiplicity]: [Span<QM31>; 1] =
            (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [pedersen_points_table_window_bits_9_multiplicity]: [QM31; 1] =
            (*pedersen_points_table_window_bits_9_multiplicity
            .try_into()
            .unwrap())
            .unbox();

        core::internal::revoke_ap_tracking();

        pedersen_points_table_window_bits_9_sum_0 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<1791500038, 0, 0, 0>(), seq_15, pedersen_points_small_0,
                    pedersen_points_small_1, pedersen_points_small_2, pedersen_points_small_3,
                    pedersen_points_small_4, pedersen_points_small_5, pedersen_points_small_6,
                    pedersen_points_small_7, pedersen_points_small_8, pedersen_points_small_9,
                    pedersen_points_small_10, pedersen_points_small_11, pedersen_points_small_12,
                    pedersen_points_small_13, pedersen_points_small_14, pedersen_points_small_15,
                    pedersen_points_small_16, pedersen_points_small_17, pedersen_points_small_18,
                    pedersen_points_small_19, pedersen_points_small_20, pedersen_points_small_21,
                    pedersen_points_small_22, pedersen_points_small_23, pedersen_points_small_24,
                    pedersen_points_small_25, pedersen_points_small_26, pedersen_points_small_27,
                    pedersen_points_small_28, pedersen_points_small_29, pedersen_points_small_30,
                    pedersen_points_small_31, pedersen_points_small_32, pedersen_points_small_33,
                    pedersen_points_small_34, pedersen_points_small_35, pedersen_points_small_36,
                    pedersen_points_small_37, pedersen_points_small_38, pedersen_points_small_39,
                    pedersen_points_small_40, pedersen_points_small_41, pedersen_points_small_42,
                    pedersen_points_small_43, pedersen_points_small_44, pedersen_points_small_45,
                    pedersen_points_small_46, pedersen_points_small_47, pedersen_points_small_48,
                    pedersen_points_small_49, pedersen_points_small_50, pedersen_points_small_51,
                    pedersen_points_small_52, pedersen_points_small_53, pedersen_points_small_54,
                    pedersen_points_small_55,
                ]
                    .span(),
            );

        lookup_constraints(
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
            claimed_sum,
            pedersen_points_table_window_bits_9_multiplicity,
            column_size,
            ref interaction_trace_mask_values,
            pedersen_points_table_window_bits_9_sum_0,
        );
    }
}


fn lookup_constraints(
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
    claimed_sum: QM31,
    pedersen_points_table_window_bits_9_multiplicity: QM31,
    column_size: M31,
    ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
    pedersen_points_table_window_bits_9_sum_0: QM31,
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
        * pedersen_points_table_window_bits_9_sum_0)
        + pedersen_points_table_window_bits_9_multiplicity)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
}
