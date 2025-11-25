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
        let seq_6 = preprocessed_mask_values.get_and_mark_used(SEQ_6_IDX);
        let poseidon_round_keys_0 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_0_IDX);
        let poseidon_round_keys_1 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_1_IDX);
        let poseidon_round_keys_2 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_2_IDX);
        let poseidon_round_keys_3 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_3_IDX);
        let poseidon_round_keys_4 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_4_IDX);
        let poseidon_round_keys_5 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_5_IDX);
        let poseidon_round_keys_6 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_6_IDX);
        let poseidon_round_keys_7 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_7_IDX);
        let poseidon_round_keys_8 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_8_IDX);
        let poseidon_round_keys_9 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_9_IDX);
        let poseidon_round_keys_10 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_10_IDX);
        let poseidon_round_keys_11 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_11_IDX);
        let poseidon_round_keys_12 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_12_IDX);
        let poseidon_round_keys_13 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_13_IDX);
        let poseidon_round_keys_14 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_14_IDX);
        let poseidon_round_keys_15 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_15_IDX);
        let poseidon_round_keys_16 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_16_IDX);
        let poseidon_round_keys_17 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_17_IDX);
        let poseidon_round_keys_18 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_18_IDX);
        let poseidon_round_keys_19 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_19_IDX);
        let poseidon_round_keys_20 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_20_IDX);
        let poseidon_round_keys_21 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_21_IDX);
        let poseidon_round_keys_22 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_22_IDX);
        let poseidon_round_keys_23 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_23_IDX);
        let poseidon_round_keys_24 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_24_IDX);
        let poseidon_round_keys_25 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_25_IDX);
        let poseidon_round_keys_26 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_26_IDX);
        let poseidon_round_keys_27 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_27_IDX);
        let poseidon_round_keys_28 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_28_IDX);
        let poseidon_round_keys_29 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_29_IDX);

        let [enabler]: [Span<QM31>; 1] = (*trace_mask_values.multi_pop_front().unwrap()).unbox();
        let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        poseidon_round_keys_sum_0 = self
            .poseidon_round_keys_lookup_elements
            .combine_qm31(
                [
                    seq_6, poseidon_round_keys_0, poseidon_round_keys_1, poseidon_round_keys_2,
                    poseidon_round_keys_3, poseidon_round_keys_4, poseidon_round_keys_5,
                    poseidon_round_keys_6, poseidon_round_keys_7, poseidon_round_keys_8,
                    poseidon_round_keys_9, poseidon_round_keys_10, poseidon_round_keys_11,
                    poseidon_round_keys_12, poseidon_round_keys_13, poseidon_round_keys_14,
                    poseidon_round_keys_15, poseidon_round_keys_16, poseidon_round_keys_17,
                    poseidon_round_keys_18, poseidon_round_keys_19, poseidon_round_keys_20,
                    poseidon_round_keys_21, poseidon_round_keys_22, poseidon_round_keys_23,
                    poseidon_round_keys_24, poseidon_round_keys_25, poseidon_round_keys_26,
                    poseidon_round_keys_27, poseidon_round_keys_28, poseidon_round_keys_29,
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
