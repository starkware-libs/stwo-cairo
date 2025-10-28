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

#[cfg(not(feature: "poseidon252_verifier"))]
pub impl CairoComponentImpl of CairoComponent<Component> {
    fn mask_points(
        self: @Component,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let log_size = LOG_SIZE;
        let trace_gen = CanonicCosetImpl::new(log_size).coset.step;
        let point_offset_neg_1 = point.add_circle_point_m31(-trace_gen.mul(1).to_point());
        preprocessed_column_set.insert(preprocessed_columns::seq_column_idx(LOG_SIZE));
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__0_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__1_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__2_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__3_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__4_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__5_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__6_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__7_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__8_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__9_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__10_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__11_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__12_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__13_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__14_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__15_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__16_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__17_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__18_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__19_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__20_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__21_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__22_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__23_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__24_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__25_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__26_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__27_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__28_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__29_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__30_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__31_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__32_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__33_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__34_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__35_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__36_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__37_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__38_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__39_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__40_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__41_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__42_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__43_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__44_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__45_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__46_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__47_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__48_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__49_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__50_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__51_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__52_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__53_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__54_IDX);
        preprocessed_column_set.insert(preprocessed_columns::PEDERSEN_POINTS__55_IDX);
        trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
        interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
        interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
        interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
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
        let log_size = LOG_SIZE;
        let trace_domain = CanonicCosetImpl::new(log_size);
        let domain_vanishing_eval_inv = trace_domain.eval_vanishing(point).inverse();
        let claimed_sum = *self.interaction_claim.claimed_sum;
        let column_size = m31(pow2(log_size));
        let mut pedersen_points_table_sum_0: QM31 = Zero::zero();
        let seq = preprocessed_mask_values.get(preprocessed_columns::seq_column_idx(LOG_SIZE));
        let pedersenpoints_0 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__0_IDX);
        let pedersenpoints_1 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__1_IDX);
        let pedersenpoints_2 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__2_IDX);
        let pedersenpoints_3 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__3_IDX);
        let pedersenpoints_4 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__4_IDX);
        let pedersenpoints_5 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__5_IDX);
        let pedersenpoints_6 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__6_IDX);
        let pedersenpoints_7 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__7_IDX);
        let pedersenpoints_8 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__8_IDX);
        let pedersenpoints_9 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__9_IDX);
        let pedersenpoints_10 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__10_IDX);
        let pedersenpoints_11 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__11_IDX);
        let pedersenpoints_12 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__12_IDX);
        let pedersenpoints_13 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__13_IDX);
        let pedersenpoints_14 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__14_IDX);
        let pedersenpoints_15 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__15_IDX);
        let pedersenpoints_16 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__16_IDX);
        let pedersenpoints_17 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__17_IDX);
        let pedersenpoints_18 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__18_IDX);
        let pedersenpoints_19 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__19_IDX);
        let pedersenpoints_20 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__20_IDX);
        let pedersenpoints_21 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__21_IDX);
        let pedersenpoints_22 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__22_IDX);
        let pedersenpoints_23 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__23_IDX);
        let pedersenpoints_24 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__24_IDX);
        let pedersenpoints_25 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__25_IDX);
        let pedersenpoints_26 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__26_IDX);
        let pedersenpoints_27 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__27_IDX);
        let pedersenpoints_28 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__28_IDX);
        let pedersenpoints_29 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__29_IDX);
        let pedersenpoints_30 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__30_IDX);
        let pedersenpoints_31 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__31_IDX);
        let pedersenpoints_32 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__32_IDX);
        let pedersenpoints_33 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__33_IDX);
        let pedersenpoints_34 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__34_IDX);
        let pedersenpoints_35 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__35_IDX);
        let pedersenpoints_36 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__36_IDX);
        let pedersenpoints_37 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__37_IDX);
        let pedersenpoints_38 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__38_IDX);
        let pedersenpoints_39 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__39_IDX);
        let pedersenpoints_40 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__40_IDX);
        let pedersenpoints_41 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__41_IDX);
        let pedersenpoints_42 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__42_IDX);
        let pedersenpoints_43 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__43_IDX);
        let pedersenpoints_44 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__44_IDX);
        let pedersenpoints_45 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__45_IDX);
        let pedersenpoints_46 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__46_IDX);
        let pedersenpoints_47 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__47_IDX);
        let pedersenpoints_48 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__48_IDX);
        let pedersenpoints_49 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__49_IDX);
        let pedersenpoints_50 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__50_IDX);
        let pedersenpoints_51 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__51_IDX);
        let pedersenpoints_52 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__52_IDX);
        let pedersenpoints_53 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__53_IDX);
        let pedersenpoints_54 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__54_IDX);
        let pedersenpoints_55 = preprocessed_mask_values
            .get(preprocessed_columns::PEDERSEN_POINTS__55_IDX);

        let [enabler]: [Span<QM31>; 1] = (*trace_mask_values.multi_pop_front().unwrap()).unbox();
        let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        pedersen_points_table_sum_0 = self
            .pedersen_points_table_lookup_elements
            .combine_qm31(
                [
                    seq, pedersenpoints_0, pedersenpoints_1, pedersenpoints_2, pedersenpoints_3,
                    pedersenpoints_4, pedersenpoints_5, pedersenpoints_6, pedersenpoints_7,
                    pedersenpoints_8, pedersenpoints_9, pedersenpoints_10, pedersenpoints_11,
                    pedersenpoints_12, pedersenpoints_13, pedersenpoints_14, pedersenpoints_15,
                    pedersenpoints_16, pedersenpoints_17, pedersenpoints_18, pedersenpoints_19,
                    pedersenpoints_20, pedersenpoints_21, pedersenpoints_22, pedersenpoints_23,
                    pedersenpoints_24, pedersenpoints_25, pedersenpoints_26, pedersenpoints_27,
                    pedersenpoints_28, pedersenpoints_29, pedersenpoints_30, pedersenpoints_31,
                    pedersenpoints_32, pedersenpoints_33, pedersenpoints_34, pedersenpoints_35,
                    pedersenpoints_36, pedersenpoints_37, pedersenpoints_38, pedersenpoints_39,
                    pedersenpoints_40, pedersenpoints_41, pedersenpoints_42, pedersenpoints_43,
                    pedersenpoints_44, pedersenpoints_45, pedersenpoints_46, pedersenpoints_47,
                    pedersenpoints_48, pedersenpoints_49, pedersenpoints_50, pedersenpoints_51,
                    pedersenpoints_52, pedersenpoints_53, pedersenpoints_54, pedersenpoints_55,
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
