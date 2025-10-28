// This file was created by the AIR team.

use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 1;
const LOG_SIZE: u32 = 6;

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
    pub poseidon_round_keys_lookup_elements: crate::PoseidonRoundKeysElements,
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
            poseidon_round_keys_lookup_elements: interaction_elements.poseidon_round_keys.clone(),
        }
    }
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
        let trace_gen = CanonicCosetImpl::new(log_size).coset.step;
        let point_offset_neg_1 = point.add_circle_point_m31(-trace_gen.mul(1).to_point());
        preprocessed_column_set.insert(preprocessed_columns::seq_column_idx(LOG_SIZE));
        preprocessed_column_set.insert(preprocessed_columns::POSEIDON_ROUND_KEYS__0_IDX);
        preprocessed_column_set.insert(preprocessed_columns::POSEIDON_ROUND_KEYS__1_IDX);
        preprocessed_column_set.insert(preprocessed_columns::POSEIDON_ROUND_KEYS__2_IDX);
        preprocessed_column_set.insert(preprocessed_columns::POSEIDON_ROUND_KEYS__3_IDX);
        preprocessed_column_set.insert(preprocessed_columns::POSEIDON_ROUND_KEYS__4_IDX);
        preprocessed_column_set.insert(preprocessed_columns::POSEIDON_ROUND_KEYS__5_IDX);
        preprocessed_column_set.insert(preprocessed_columns::POSEIDON_ROUND_KEYS__6_IDX);
        preprocessed_column_set.insert(preprocessed_columns::POSEIDON_ROUND_KEYS__7_IDX);
        preprocessed_column_set.insert(preprocessed_columns::POSEIDON_ROUND_KEYS__8_IDX);
        preprocessed_column_set.insert(preprocessed_columns::POSEIDON_ROUND_KEYS__9_IDX);
        preprocessed_column_set.insert(preprocessed_columns::POSEIDON_ROUND_KEYS__10_IDX);
        preprocessed_column_set.insert(preprocessed_columns::POSEIDON_ROUND_KEYS__11_IDX);
        preprocessed_column_set.insert(preprocessed_columns::POSEIDON_ROUND_KEYS__12_IDX);
        preprocessed_column_set.insert(preprocessed_columns::POSEIDON_ROUND_KEYS__13_IDX);
        preprocessed_column_set.insert(preprocessed_columns::POSEIDON_ROUND_KEYS__14_IDX);
        preprocessed_column_set.insert(preprocessed_columns::POSEIDON_ROUND_KEYS__15_IDX);
        preprocessed_column_set.insert(preprocessed_columns::POSEIDON_ROUND_KEYS__16_IDX);
        preprocessed_column_set.insert(preprocessed_columns::POSEIDON_ROUND_KEYS__17_IDX);
        preprocessed_column_set.insert(preprocessed_columns::POSEIDON_ROUND_KEYS__18_IDX);
        preprocessed_column_set.insert(preprocessed_columns::POSEIDON_ROUND_KEYS__19_IDX);
        preprocessed_column_set.insert(preprocessed_columns::POSEIDON_ROUND_KEYS__20_IDX);
        preprocessed_column_set.insert(preprocessed_columns::POSEIDON_ROUND_KEYS__21_IDX);
        preprocessed_column_set.insert(preprocessed_columns::POSEIDON_ROUND_KEYS__22_IDX);
        preprocessed_column_set.insert(preprocessed_columns::POSEIDON_ROUND_KEYS__23_IDX);
        preprocessed_column_set.insert(preprocessed_columns::POSEIDON_ROUND_KEYS__24_IDX);
        preprocessed_column_set.insert(preprocessed_columns::POSEIDON_ROUND_KEYS__25_IDX);
        preprocessed_column_set.insert(preprocessed_columns::POSEIDON_ROUND_KEYS__26_IDX);
        preprocessed_column_set.insert(preprocessed_columns::POSEIDON_ROUND_KEYS__27_IDX);
        preprocessed_column_set.insert(preprocessed_columns::POSEIDON_ROUND_KEYS__28_IDX);
        preprocessed_column_set.insert(preprocessed_columns::POSEIDON_ROUND_KEYS__29_IDX);
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
        let mut poseidon_round_keys_sum_0: QM31 = Zero::zero();
        let seq = preprocessed_mask_values.get(preprocessed_columns::seq_column_idx(LOG_SIZE));
        let poseidonroundkeys_0 = preprocessed_mask_values
            .get(preprocessed_columns::POSEIDON_ROUND_KEYS__0_IDX);
        let poseidonroundkeys_1 = preprocessed_mask_values
            .get(preprocessed_columns::POSEIDON_ROUND_KEYS__1_IDX);
        let poseidonroundkeys_2 = preprocessed_mask_values
            .get(preprocessed_columns::POSEIDON_ROUND_KEYS__2_IDX);
        let poseidonroundkeys_3 = preprocessed_mask_values
            .get(preprocessed_columns::POSEIDON_ROUND_KEYS__3_IDX);
        let poseidonroundkeys_4 = preprocessed_mask_values
            .get(preprocessed_columns::POSEIDON_ROUND_KEYS__4_IDX);
        let poseidonroundkeys_5 = preprocessed_mask_values
            .get(preprocessed_columns::POSEIDON_ROUND_KEYS__5_IDX);
        let poseidonroundkeys_6 = preprocessed_mask_values
            .get(preprocessed_columns::POSEIDON_ROUND_KEYS__6_IDX);
        let poseidonroundkeys_7 = preprocessed_mask_values
            .get(preprocessed_columns::POSEIDON_ROUND_KEYS__7_IDX);
        let poseidonroundkeys_8 = preprocessed_mask_values
            .get(preprocessed_columns::POSEIDON_ROUND_KEYS__8_IDX);
        let poseidonroundkeys_9 = preprocessed_mask_values
            .get(preprocessed_columns::POSEIDON_ROUND_KEYS__9_IDX);
        let poseidonroundkeys_10 = preprocessed_mask_values
            .get(preprocessed_columns::POSEIDON_ROUND_KEYS__10_IDX);
        let poseidonroundkeys_11 = preprocessed_mask_values
            .get(preprocessed_columns::POSEIDON_ROUND_KEYS__11_IDX);
        let poseidonroundkeys_12 = preprocessed_mask_values
            .get(preprocessed_columns::POSEIDON_ROUND_KEYS__12_IDX);
        let poseidonroundkeys_13 = preprocessed_mask_values
            .get(preprocessed_columns::POSEIDON_ROUND_KEYS__13_IDX);
        let poseidonroundkeys_14 = preprocessed_mask_values
            .get(preprocessed_columns::POSEIDON_ROUND_KEYS__14_IDX);
        let poseidonroundkeys_15 = preprocessed_mask_values
            .get(preprocessed_columns::POSEIDON_ROUND_KEYS__15_IDX);
        let poseidonroundkeys_16 = preprocessed_mask_values
            .get(preprocessed_columns::POSEIDON_ROUND_KEYS__16_IDX);
        let poseidonroundkeys_17 = preprocessed_mask_values
            .get(preprocessed_columns::POSEIDON_ROUND_KEYS__17_IDX);
        let poseidonroundkeys_18 = preprocessed_mask_values
            .get(preprocessed_columns::POSEIDON_ROUND_KEYS__18_IDX);
        let poseidonroundkeys_19 = preprocessed_mask_values
            .get(preprocessed_columns::POSEIDON_ROUND_KEYS__19_IDX);
        let poseidonroundkeys_20 = preprocessed_mask_values
            .get(preprocessed_columns::POSEIDON_ROUND_KEYS__20_IDX);
        let poseidonroundkeys_21 = preprocessed_mask_values
            .get(preprocessed_columns::POSEIDON_ROUND_KEYS__21_IDX);
        let poseidonroundkeys_22 = preprocessed_mask_values
            .get(preprocessed_columns::POSEIDON_ROUND_KEYS__22_IDX);
        let poseidonroundkeys_23 = preprocessed_mask_values
            .get(preprocessed_columns::POSEIDON_ROUND_KEYS__23_IDX);
        let poseidonroundkeys_24 = preprocessed_mask_values
            .get(preprocessed_columns::POSEIDON_ROUND_KEYS__24_IDX);
        let poseidonroundkeys_25 = preprocessed_mask_values
            .get(preprocessed_columns::POSEIDON_ROUND_KEYS__25_IDX);
        let poseidonroundkeys_26 = preprocessed_mask_values
            .get(preprocessed_columns::POSEIDON_ROUND_KEYS__26_IDX);
        let poseidonroundkeys_27 = preprocessed_mask_values
            .get(preprocessed_columns::POSEIDON_ROUND_KEYS__27_IDX);
        let poseidonroundkeys_28 = preprocessed_mask_values
            .get(preprocessed_columns::POSEIDON_ROUND_KEYS__28_IDX);
        let poseidonroundkeys_29 = preprocessed_mask_values
            .get(preprocessed_columns::POSEIDON_ROUND_KEYS__29_IDX);

        let [enabler]: [Span<QM31>; 1] = (*trace_mask_values.multi_pop_front().unwrap()).unbox();
        let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        poseidon_round_keys_sum_0 = self
            .poseidon_round_keys_lookup_elements
            .combine_qm31(
                [
                    seq, poseidonroundkeys_0, poseidonroundkeys_1, poseidonroundkeys_2,
                    poseidonroundkeys_3, poseidonroundkeys_4, poseidonroundkeys_5,
                    poseidonroundkeys_6, poseidonroundkeys_7, poseidonroundkeys_8,
                    poseidonroundkeys_9, poseidonroundkeys_10, poseidonroundkeys_11,
                    poseidonroundkeys_12, poseidonroundkeys_13, poseidonroundkeys_14,
                    poseidonroundkeys_15, poseidonroundkeys_16, poseidonroundkeys_17,
                    poseidonroundkeys_18, poseidonroundkeys_19, poseidonroundkeys_20,
                    poseidonroundkeys_21, poseidonroundkeys_22, poseidonroundkeys_23,
                    poseidonroundkeys_24, poseidonroundkeys_25, poseidonroundkeys_26,
                    poseidonroundkeys_27, poseidonroundkeys_28, poseidonroundkeys_29,
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
            poseidon_round_keys_sum_0,
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
    poseidon_round_keys_sum_0: QM31,
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
        * poseidon_round_keys_sum_0)
        + enabler)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
}
