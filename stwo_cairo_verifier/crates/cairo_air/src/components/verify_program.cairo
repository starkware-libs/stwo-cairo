// This file was created by the AIR team.

use crate::components::subroutines::mem_verify_cond::mem_verify_cond_evaluate;
use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 1;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 2] = [
    ('MemoryAddressToId', 1), ('MemoryIdToBig', 1),
];

#[derive(Drop, Serde, Copy)]
pub struct Claim {
    pub log_size: u32,
    pub verify_program_segment_start: u32,
}

pub impl ClaimImpl of ClaimTrait<Claim> {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = *(self.log_size);
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = [log_size; N_TRACE_COLUMNS].span();
        let interaction_log_sizes = [log_size; 4].span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        channel.mix_u64((*(self.log_size)).into());
        channel.mix_u64((*self.verify_program_segment_start).into());
    }

    fn accumulate_relation_uses(self: @Claim, ref relation_uses: RelationUsesDict) {
        accumulate_relation_uses(ref relation_uses, RELATION_USES_PER_ROW.span(), *self.log_size);
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
    ) {
        let log_size = *(self.claim.log_size);
        let claimed_sum = *self.interaction_claim.claimed_sum;
        let column_size = m31(pow2(log_size));
        let verify_program_segment_start: QM31 = (TryInto::<
            u32, M31,
        >::try_into((*(self.claim.verify_program_segment_start)))
            .unwrap())
            .into();
        let mut memory_address_to_id_sum_0: QM31 = Zero::zero();
        let mut numerator_0: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_1: QM31 = Zero::zero();
        let mut numerator_1: QM31 = Zero::zero();
        let curr_program_28 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_28_IDX);
        let seq = preprocessed_mask_values
            .get_and_mark_used(seq_column_idx(*(self.claim.log_size)));
        let curr_program_0 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_0_IDX);
        let curr_program_1 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_1_IDX);
        let curr_program_2 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_2_IDX);
        let curr_program_3 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_3_IDX);
        let curr_program_4 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_4_IDX);
        let curr_program_5 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_5_IDX);
        let curr_program_6 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_6_IDX);
        let curr_program_7 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_7_IDX);
        let curr_program_8 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_8_IDX);
        let curr_program_9 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_9_IDX);
        let curr_program_10 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_10_IDX);
        let curr_program_11 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_11_IDX);
        let curr_program_12 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_12_IDX);
        let curr_program_13 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_13_IDX);
        let curr_program_14 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_14_IDX);
        let curr_program_15 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_15_IDX);
        let curr_program_16 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_16_IDX);
        let curr_program_17 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_17_IDX);
        let curr_program_18 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_18_IDX);
        let curr_program_19 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_19_IDX);
        let curr_program_20 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_20_IDX);
        let curr_program_21 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_21_IDX);
        let curr_program_22 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_22_IDX);
        let curr_program_23 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_23_IDX);
        let curr_program_24 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_24_IDX);
        let curr_program_25 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_25_IDX);
        let curr_program_26 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_26_IDX);
        let curr_program_27 = preprocessed_mask_values.get_and_mark_used(CURR_PROGRAM_27_IDX);

        let [address_id_col0]: [Span<QM31>; 1] = (*trace_mask_values.multi_pop_front().unwrap())
            .unbox();
        let [address_id_col0]: [QM31; 1] = (*address_id_col0.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        mem_verify_cond_evaluate(
            [
                (verify_program_segment_start + seq), curr_program_0, curr_program_1,
                curr_program_2, curr_program_3, curr_program_4, curr_program_5, curr_program_6,
                curr_program_7, curr_program_8, curr_program_9, curr_program_10, curr_program_11,
                curr_program_12, curr_program_13, curr_program_14, curr_program_15, curr_program_16,
                curr_program_17, curr_program_18, curr_program_19, curr_program_20, curr_program_21,
                curr_program_22, curr_program_23, curr_program_24, curr_program_25, curr_program_26,
                curr_program_27, curr_program_28,
            ],
            address_id_col0,
            self.common_lookup_elements,
            ref memory_address_to_id_sum_0,
            ref numerator_0,
            ref memory_id_to_big_sum_1,
            ref numerator_1,
            ref sum,
            random_coeff,
        );

        lookup_constraints(
            ref sum,
            random_coeff,
            claimed_sum,
            numerator_0,
            numerator_1,
            column_size,
            ref interaction_trace_mask_values,
            memory_address_to_id_sum_0,
            memory_id_to_big_sum_1,
        );
    }
}


fn lookup_constraints(
    ref sum: QM31,
    random_coeff: QM31,
    claimed_sum: QM31,
    numerator_0: QM31,
    numerator_1: QM31,
    column_size: M31,
    ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
    memory_address_to_id_sum_0: QM31,
    memory_id_to_big_sum_1: QM31,
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
        * memory_address_to_id_sum_0
        * memory_id_to_big_sum_1)
        - (memory_address_to_id_sum_0 * numerator_1)
        - (memory_id_to_big_sum_1 * numerator_0));
    sum = sum * random_coeff + constraint_quotient;
}
