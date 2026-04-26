// This file was created by the AIR team.

use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 24;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 1] = [('BlakeOutput', 1)];

#[derive(Drop, Serde, Copy)]
pub struct Claim {
    pub log_size: u32,
}

pub impl ClaimImpl of ClaimTrait<Claim> {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = *(self.log_size);
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = [log_size; N_TRACE_COLUMNS].span();
        let interaction_log_sizes = [log_size; 8].span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        channel.mix_u64((*(self.log_size)).into());
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

pub impl CircuitComponentImpl of CircuitComponent<Component> {
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
        let mut blake_output_sum_0: QM31 = Zero::zero();
        let mut numerator_0: QM31 = Zero::zero();
        let mut gate_sum_1: QM31 = Zero::zero();
        let mut numerator_1: QM31 = Zero::zero();
        let mut gate_sum_2: QM31 = Zero::zero();
        let mut numerator_2: QM31 = Zero::zero();
        let final_state_addr = preprocessed_mask_values.get_and_mark_used(FINAL_STATE_ADDR_IDX);
        let blake_output0_addr = preprocessed_mask_values
            .get_and_mark_used(BLAKE_OUTPUT_0_ADDR_IDX);
        let blake_output0_mults = preprocessed_mask_values
            .get_and_mark_used(BLAKE_OUTPUT_0_MULTS_IDX);
        let blake_output1_addr = preprocessed_mask_values
            .get_and_mark_used(BLAKE_OUTPUT_1_ADDR_IDX);
        let blake_output1_mults = preprocessed_mask_values
            .get_and_mark_used(BLAKE_OUTPUT_1_MULTS_IDX);

        let [
            input_final_state_limb0_limb_0_col0,
            input_final_state_limb0_limb_1_col1,
            input_final_state_limb1_limb_0_col2,
            input_final_state_limb1_limb_1_col3,
            input_final_state_limb2_limb_0_col4,
            input_final_state_limb2_limb_1_col5,
            input_final_state_limb3_limb_0_col6,
            input_final_state_limb3_limb_1_col7,
            input_final_state_limb4_limb_0_col8,
            input_final_state_limb4_limb_1_col9,
            input_final_state_limb5_limb_0_col10,
            input_final_state_limb5_limb_1_col11,
            input_final_state_limb6_limb_0_col12,
            input_final_state_limb6_limb_1_col13,
            input_final_state_limb7_limb_0_col14,
            input_final_state_limb7_limb_1_col15,
            output_limb0_col16,
            output_limb1_col17,
            output_limb2_col18,
            output_limb3_col19,
            output_limb4_col20,
            output_limb5_col21,
            output_limb6_col22,
            output_limb7_col23,
        ]: [Span<QM31>; 24] =
            (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [input_final_state_limb0_limb_0_col0]: [QM31; 1] = (*input_final_state_limb0_limb_0_col0
            .try_into()
            .unwrap())
            .unbox();
        let [input_final_state_limb0_limb_1_col1]: [QM31; 1] = (*input_final_state_limb0_limb_1_col1
            .try_into()
            .unwrap())
            .unbox();
        let [input_final_state_limb1_limb_0_col2]: [QM31; 1] = (*input_final_state_limb1_limb_0_col2
            .try_into()
            .unwrap())
            .unbox();
        let [input_final_state_limb1_limb_1_col3]: [QM31; 1] = (*input_final_state_limb1_limb_1_col3
            .try_into()
            .unwrap())
            .unbox();
        let [input_final_state_limb2_limb_0_col4]: [QM31; 1] = (*input_final_state_limb2_limb_0_col4
            .try_into()
            .unwrap())
            .unbox();
        let [input_final_state_limb2_limb_1_col5]: [QM31; 1] = (*input_final_state_limb2_limb_1_col5
            .try_into()
            .unwrap())
            .unbox();
        let [input_final_state_limb3_limb_0_col6]: [QM31; 1] = (*input_final_state_limb3_limb_0_col6
            .try_into()
            .unwrap())
            .unbox();
        let [input_final_state_limb3_limb_1_col7]: [QM31; 1] = (*input_final_state_limb3_limb_1_col7
            .try_into()
            .unwrap())
            .unbox();
        let [input_final_state_limb4_limb_0_col8]: [QM31; 1] = (*input_final_state_limb4_limb_0_col8
            .try_into()
            .unwrap())
            .unbox();
        let [input_final_state_limb4_limb_1_col9]: [QM31; 1] = (*input_final_state_limb4_limb_1_col9
            .try_into()
            .unwrap())
            .unbox();
        let [input_final_state_limb5_limb_0_col10]: [QM31; 1] =
            (*input_final_state_limb5_limb_0_col10
            .try_into()
            .unwrap())
            .unbox();
        let [input_final_state_limb5_limb_1_col11]: [QM31; 1] =
            (*input_final_state_limb5_limb_1_col11
            .try_into()
            .unwrap())
            .unbox();
        let [input_final_state_limb6_limb_0_col12]: [QM31; 1] =
            (*input_final_state_limb6_limb_0_col12
            .try_into()
            .unwrap())
            .unbox();
        let [input_final_state_limb6_limb_1_col13]: [QM31; 1] =
            (*input_final_state_limb6_limb_1_col13
            .try_into()
            .unwrap())
            .unbox();
        let [input_final_state_limb7_limb_0_col14]: [QM31; 1] =
            (*input_final_state_limb7_limb_0_col14
            .try_into()
            .unwrap())
            .unbox();
        let [input_final_state_limb7_limb_1_col15]: [QM31; 1] =
            (*input_final_state_limb7_limb_1_col15
            .try_into()
            .unwrap())
            .unbox();
        let [output_limb0_col16]: [QM31; 1] = (*output_limb0_col16.try_into().unwrap()).unbox();
        let [output_limb1_col17]: [QM31; 1] = (*output_limb1_col17.try_into().unwrap()).unbox();
        let [output_limb2_col18]: [QM31; 1] = (*output_limb2_col18.try_into().unwrap()).unbox();
        let [output_limb3_col19]: [QM31; 1] = (*output_limb3_col19.try_into().unwrap()).unbox();
        let [output_limb4_col20]: [QM31; 1] = (*output_limb4_col20.try_into().unwrap()).unbox();
        let [output_limb5_col21]: [QM31; 1] = (*output_limb5_col21.try_into().unwrap()).unbox();
        let [output_limb6_col22]: [QM31; 1] = (*output_limb6_col22.try_into().unwrap()).unbox();
        let [output_limb7_col23]: [QM31; 1] = (*output_limb7_col23.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        // Constraint - output_limb0
        let constraint_quotient = ((output_limb0_col16
            - (input_final_state_limb0_limb_0_col0
                + (input_final_state_limb0_limb_1_col1 * qm31_const::<65536, 0, 0, 0>()))));
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - output_limb1
        let constraint_quotient = ((output_limb1_col17
            - (input_final_state_limb1_limb_0_col2
                + (input_final_state_limb1_limb_1_col3 * qm31_const::<65536, 0, 0, 0>()))));
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - output_limb2
        let constraint_quotient = ((output_limb2_col18
            - (input_final_state_limb2_limb_0_col4
                + (input_final_state_limb2_limb_1_col5 * qm31_const::<65536, 0, 0, 0>()))));
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - output_limb3
        let constraint_quotient = ((output_limb3_col19
            - (input_final_state_limb3_limb_0_col6
                + (input_final_state_limb3_limb_1_col7 * qm31_const::<65536, 0, 0, 0>()))));
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - output_limb4
        let constraint_quotient = ((output_limb4_col20
            - (input_final_state_limb4_limb_0_col8
                + (input_final_state_limb4_limb_1_col9 * qm31_const::<65536, 0, 0, 0>()))));
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - output_limb5
        let constraint_quotient = ((output_limb5_col21
            - (input_final_state_limb5_limb_0_col10
                + (input_final_state_limb5_limb_1_col11 * qm31_const::<65536, 0, 0, 0>()))));
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - output_limb6
        let constraint_quotient = ((output_limb6_col22
            - (input_final_state_limb6_limb_0_col12
                + (input_final_state_limb6_limb_1_col13 * qm31_const::<65536, 0, 0, 0>()))));
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - output_limb7
        let constraint_quotient = ((output_limb7_col23
            - (input_final_state_limb7_limb_0_col14
                + (input_final_state_limb7_limb_1_col15 * qm31_const::<65536, 0, 0, 0>()))));
        sum = sum * random_coeff + constraint_quotient;

        blake_output_sum_0 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<1061955672, 0, 0, 0>(), final_state_addr,
                    input_final_state_limb0_limb_0_col0, input_final_state_limb0_limb_1_col1,
                    input_final_state_limb1_limb_0_col2, input_final_state_limb1_limb_1_col3,
                    input_final_state_limb2_limb_0_col4, input_final_state_limb2_limb_1_col5,
                    input_final_state_limb3_limb_0_col6, input_final_state_limb3_limb_1_col7,
                    input_final_state_limb4_limb_0_col8, input_final_state_limb4_limb_1_col9,
                    input_final_state_limb5_limb_0_col10, input_final_state_limb5_limb_1_col11,
                    input_final_state_limb6_limb_0_col12, input_final_state_limb6_limb_1_col13,
                    input_final_state_limb7_limb_0_col14, input_final_state_limb7_limb_1_col15,
                ]
                    .span(),
            );
        numerator_0 = qm31_const::<1, 0, 0, 0>();

        gate_sum_1 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<378353459, 0, 0, 0>(), blake_output0_addr, output_limb0_col16,
                    output_limb1_col17, output_limb2_col18, output_limb3_col19,
                ]
                    .span(),
            );
        numerator_1 = blake_output0_mults;

        gate_sum_2 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<378353459, 0, 0, 0>(), blake_output1_addr, output_limb4_col20,
                    output_limb5_col21, output_limb6_col22, output_limb7_col23,
                ]
                    .span(),
            );
        numerator_2 = blake_output1_mults;

        lookup_constraints(
            ref sum,
            random_coeff,
            claimed_sum,
            numerator_0,
            numerator_1,
            numerator_2,
            column_size,
            ref interaction_trace_mask_values,
            blake_output_sum_0,
            gate_sum_1,
            gate_sum_2,
        );
    }
}


fn lookup_constraints(
    ref sum: QM31,
    random_coeff: QM31,
    claimed_sum: QM31,
    numerator_0: QM31,
    numerator_1: QM31,
    numerator_2: QM31,
    column_size: M31,
    ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
    blake_output_sum_0: QM31,
    gate_sum_1: QM31,
    gate_sum_2: QM31,
) {
    let [
        trace_2_col0,
        trace_2_col1,
        trace_2_col2,
        trace_2_col3,
        trace_2_col4,
        trace_2_col5,
        trace_2_col6,
        trace_2_col7,
    ]: [Span<QM31>; 8] =
        (*interaction_trace_mask_values
        .multi_pop_front()
        .unwrap())
        .unbox();

    let [trace_2_col0]: [QM31; 1] = (*trace_2_col0.try_into().unwrap()).unbox();
    let [trace_2_col1]: [QM31; 1] = (*trace_2_col1.try_into().unwrap()).unbox();
    let [trace_2_col2]: [QM31; 1] = (*trace_2_col2.try_into().unwrap()).unbox();
    let [trace_2_col3]: [QM31; 1] = (*trace_2_col3.try_into().unwrap()).unbox();
    let [trace_2_col4_neg1, trace_2_col4]: [QM31; 2] = (*trace_2_col4.try_into().unwrap()).unbox();
    let [trace_2_col5_neg1, trace_2_col5]: [QM31; 2] = (*trace_2_col5.try_into().unwrap()).unbox();
    let [trace_2_col6_neg1, trace_2_col6]: [QM31; 2] = (*trace_2_col6.try_into().unwrap()).unbox();
    let [trace_2_col7_neg1, trace_2_col7]: [QM31; 2] = (*trace_2_col7.try_into().unwrap()).unbox();

    core::internal::revoke_ap_tracking();

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3],
    ))
        * blake_output_sum_0
        * gate_sum_1)
        + (blake_output_sum_0 * numerator_1)
        - (gate_sum_1 * numerator_0));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7],
    )
        - QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3])
        - QM31Impl::from_partial_evals(
            [trace_2_col4_neg1, trace_2_col5_neg1, trace_2_col6_neg1, trace_2_col7_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * gate_sum_2)
        + numerator_2);
    sum = sum * random_coeff + constraint_quotient;
}
