// This file was created by the AIR team.

use crate::components::subroutines::bitwise_xor_num_bits_8::bitwise_xor_num_bits_8_evaluate;
use crate::components::subroutines::split_16_low_part_size_8::split_16_low_part_size_8_evaluate;
use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 20;
pub const N_INTERACTION_COLUMNS: usize = 24;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 2] = [('VerifyBitwiseXor_8', 8), ('Gate', 3)];

#[derive(Drop, Serde, Copy)]
pub struct Claim {
    pub log_size: u32,
}

pub impl ClaimImpl of ClaimTrait<Claim> {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = *(self.log_size);
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = [log_size; N_TRACE_COLUMNS].span();
        let interaction_log_sizes = [log_size; N_INTERACTION_COLUMNS].span();
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

pub impl AirComponentImpl of AirComponent<Component> {
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
        let mut verify_bitwise_xor_8_sum_0: QM31 = Zero::zero();
        let mut numerator_0: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_sum_1: QM31 = Zero::zero();
        let mut numerator_1: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_sum_2: QM31 = Zero::zero();
        let mut numerator_2: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_sum_3: QM31 = Zero::zero();
        let mut numerator_3: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_sum_4: QM31 = Zero::zero();
        let mut numerator_4: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_sum_5: QM31 = Zero::zero();
        let mut numerator_5: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_sum_6: QM31 = Zero::zero();
        let mut numerator_6: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_sum_7: QM31 = Zero::zero();
        let mut numerator_7: QM31 = Zero::zero();
        let mut gate_sum_8: QM31 = Zero::zero();
        let mut numerator_8: QM31 = Zero::zero();
        let mut gate_sum_9: QM31 = Zero::zero();
        let mut numerator_9: QM31 = Zero::zero();
        let mut gate_sum_10: QM31 = Zero::zero();
        let mut numerator_10: QM31 = Zero::zero();
        let mut gate_sum_11: QM31 = Zero::zero();
        let mut numerator_11: QM31 = Zero::zero();
        let triple_xor_input_addr_0 = preprocessed_mask_values
            .get_and_mark_used(TRIPLE_XOR_INPUT_ADDR_0_IDX);
        let triple_xor_input_addr_1 = preprocessed_mask_values
            .get_and_mark_used(TRIPLE_XOR_INPUT_ADDR_1_IDX);
        let triple_xor_input_addr_2 = preprocessed_mask_values
            .get_and_mark_used(TRIPLE_XOR_INPUT_ADDR_2_IDX);
        let triple_xor_output_addr = preprocessed_mask_values
            .get_and_mark_used(TRIPLE_XOR_OUTPUT_ADDR_IDX);
        let triple_xor_multiplicity = preprocessed_mask_values
            .get_and_mark_used(TRIPLE_XOR_MULTIPLICITY_IDX);

        let [
            input_a_limb_0_col0,
            input_a_limb_1_col1,
            input_b_limb_0_col2,
            input_b_limb_1_col3,
            input_c_limb_0_col4,
            input_c_limb_1_col5,
            input_a_xor_b_xor_c_limb_0_col6,
            input_a_xor_b_xor_c_limb_1_col7,
            ms_8_bits_col8,
            ms_8_bits_col9,
            ms_8_bits_col10,
            ms_8_bits_col11,
            ms_8_bits_col12,
            ms_8_bits_col13,
            ms_8_bits_col14,
            ms_8_bits_col15,
            xor_col16,
            xor_col17,
            xor_col18,
            xor_col19,
        ]: [Span<QM31>; 20] =
            (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [input_a_limb_0_col0]: [QM31; 1] = (*input_a_limb_0_col0.try_into().unwrap()).unbox();
        let [input_a_limb_1_col1]: [QM31; 1] = (*input_a_limb_1_col1.try_into().unwrap()).unbox();
        let [input_b_limb_0_col2]: [QM31; 1] = (*input_b_limb_0_col2.try_into().unwrap()).unbox();
        let [input_b_limb_1_col3]: [QM31; 1] = (*input_b_limb_1_col3.try_into().unwrap()).unbox();
        let [input_c_limb_0_col4]: [QM31; 1] = (*input_c_limb_0_col4.try_into().unwrap()).unbox();
        let [input_c_limb_1_col5]: [QM31; 1] = (*input_c_limb_1_col5.try_into().unwrap()).unbox();
        let [input_a_xor_b_xor_c_limb_0_col6]: [QM31; 1] = (*input_a_xor_b_xor_c_limb_0_col6
            .try_into()
            .unwrap())
            .unbox();
        let [input_a_xor_b_xor_c_limb_1_col7]: [QM31; 1] = (*input_a_xor_b_xor_c_limb_1_col7
            .try_into()
            .unwrap())
            .unbox();
        let [ms_8_bits_col8]: [QM31; 1] = (*ms_8_bits_col8.try_into().unwrap()).unbox();
        let [ms_8_bits_col9]: [QM31; 1] = (*ms_8_bits_col9.try_into().unwrap()).unbox();
        let [ms_8_bits_col10]: [QM31; 1] = (*ms_8_bits_col10.try_into().unwrap()).unbox();
        let [ms_8_bits_col11]: [QM31; 1] = (*ms_8_bits_col11.try_into().unwrap()).unbox();
        let [ms_8_bits_col12]: [QM31; 1] = (*ms_8_bits_col12.try_into().unwrap()).unbox();
        let [ms_8_bits_col13]: [QM31; 1] = (*ms_8_bits_col13.try_into().unwrap()).unbox();
        let [ms_8_bits_col14]: [QM31; 1] = (*ms_8_bits_col14.try_into().unwrap()).unbox();
        let [ms_8_bits_col15]: [QM31; 1] = (*ms_8_bits_col15.try_into().unwrap()).unbox();
        let [xor_col16]: [QM31; 1] = (*xor_col16.try_into().unwrap()).unbox();
        let [xor_col17]: [QM31; 1] = (*xor_col17.try_into().unwrap()).unbox();
        let [xor_col18]: [QM31; 1] = (*xor_col18.try_into().unwrap()).unbox();
        let [xor_col19]: [QM31; 1] = (*xor_col19.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        let split_16_low_part_size_8_output_tmp_4ec2c_1_limb_0: QM31 =
            split_16_low_part_size_8_evaluate(
            input_a_limb_0_col0, ms_8_bits_col8, self.common_lookup_elements, ref sum, random_coeff,
        );
        let split_16_low_part_size_8_output_tmp_4ec2c_3_limb_0: QM31 =
            split_16_low_part_size_8_evaluate(
            input_a_limb_1_col1, ms_8_bits_col9, self.common_lookup_elements, ref sum, random_coeff,
        );
        let split_16_low_part_size_8_output_tmp_4ec2c_5_limb_0: QM31 =
            split_16_low_part_size_8_evaluate(
            input_b_limb_0_col2,
            ms_8_bits_col10,
            self.common_lookup_elements,
            ref sum,
            random_coeff,
        );
        let split_16_low_part_size_8_output_tmp_4ec2c_7_limb_0: QM31 =
            split_16_low_part_size_8_evaluate(
            input_b_limb_1_col3,
            ms_8_bits_col11,
            self.common_lookup_elements,
            ref sum,
            random_coeff,
        );
        let split_16_low_part_size_8_output_tmp_4ec2c_9_limb_0: QM31 =
            split_16_low_part_size_8_evaluate(
            input_c_limb_0_col4,
            ms_8_bits_col12,
            self.common_lookup_elements,
            ref sum,
            random_coeff,
        );
        let split_16_low_part_size_8_output_tmp_4ec2c_11_limb_0: QM31 =
            split_16_low_part_size_8_evaluate(
            input_c_limb_1_col5,
            ms_8_bits_col13,
            self.common_lookup_elements,
            ref sum,
            random_coeff,
        );
        let split_16_low_part_size_8_output_tmp_4ec2c_13_limb_0: QM31 =
            split_16_low_part_size_8_evaluate(
            input_a_xor_b_xor_c_limb_0_col6,
            ms_8_bits_col14,
            self.common_lookup_elements,
            ref sum,
            random_coeff,
        );
        let split_16_low_part_size_8_output_tmp_4ec2c_15_limb_0: QM31 =
            split_16_low_part_size_8_evaluate(
            input_a_xor_b_xor_c_limb_1_col7,
            ms_8_bits_col15,
            self.common_lookup_elements,
            ref sum,
            random_coeff,
        );
        bitwise_xor_num_bits_8_evaluate(
            [
                split_16_low_part_size_8_output_tmp_4ec2c_1_limb_0,
                split_16_low_part_size_8_output_tmp_4ec2c_5_limb_0,
            ],
            xor_col16,
            self.common_lookup_elements,
            ref verify_bitwise_xor_8_sum_0,
            ref numerator_0,
            ref sum,
            random_coeff,
        );
        bitwise_xor_num_bits_8_evaluate(
            [ms_8_bits_col8, ms_8_bits_col10],
            xor_col17,
            self.common_lookup_elements,
            ref verify_bitwise_xor_8_sum_1,
            ref numerator_1,
            ref sum,
            random_coeff,
        );
        bitwise_xor_num_bits_8_evaluate(
            [
                split_16_low_part_size_8_output_tmp_4ec2c_3_limb_0,
                split_16_low_part_size_8_output_tmp_4ec2c_7_limb_0,
            ],
            xor_col18,
            self.common_lookup_elements,
            ref verify_bitwise_xor_8_sum_2,
            ref numerator_2,
            ref sum,
            random_coeff,
        );
        bitwise_xor_num_bits_8_evaluate(
            [ms_8_bits_col9, ms_8_bits_col11],
            xor_col19,
            self.common_lookup_elements,
            ref verify_bitwise_xor_8_sum_3,
            ref numerator_3,
            ref sum,
            random_coeff,
        );

        verify_bitwise_xor_8_sum_4 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<112558620, 0, 0, 0>(), xor_col16,
                    split_16_low_part_size_8_output_tmp_4ec2c_9_limb_0,
                    split_16_low_part_size_8_output_tmp_4ec2c_13_limb_0,
                ]
                    .span(),
            );
        numerator_4 = qm31_const::<1, 0, 0, 0>();

        verify_bitwise_xor_8_sum_5 = self
            .common_lookup_elements
            .combine_qm31(
                [qm31_const::<112558620, 0, 0, 0>(), xor_col17, ms_8_bits_col12, ms_8_bits_col14]
                    .span(),
            );
        numerator_5 = qm31_const::<1, 0, 0, 0>();

        verify_bitwise_xor_8_sum_6 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<112558620, 0, 0, 0>(), xor_col18,
                    split_16_low_part_size_8_output_tmp_4ec2c_11_limb_0,
                    split_16_low_part_size_8_output_tmp_4ec2c_15_limb_0,
                ]
                    .span(),
            );
        numerator_6 = qm31_const::<1, 0, 0, 0>();

        verify_bitwise_xor_8_sum_7 = self
            .common_lookup_elements
            .combine_qm31(
                [qm31_const::<112558620, 0, 0, 0>(), xor_col19, ms_8_bits_col13, ms_8_bits_col15]
                    .span(),
            );
        numerator_7 = qm31_const::<1, 0, 0, 0>();

        gate_sum_8 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<378353459, 0, 0, 0>(), triple_xor_input_addr_0,
                    input_a_limb_0_col0, input_a_limb_1_col1,
                ]
                    .span(),
            );
        numerator_8 = qm31_const::<1, 0, 0, 0>();

        gate_sum_9 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<378353459, 0, 0, 0>(), triple_xor_input_addr_1,
                    input_b_limb_0_col2, input_b_limb_1_col3,
                ]
                    .span(),
            );
        numerator_9 = qm31_const::<1, 0, 0, 0>();

        gate_sum_10 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<378353459, 0, 0, 0>(), triple_xor_input_addr_2,
                    input_c_limb_0_col4, input_c_limb_1_col5,
                ]
                    .span(),
            );
        numerator_10 = qm31_const::<1, 0, 0, 0>();

        gate_sum_11 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<378353459, 0, 0, 0>(), triple_xor_output_addr,
                    input_a_xor_b_xor_c_limb_0_col6, input_a_xor_b_xor_c_limb_1_col7,
                ]
                    .span(),
            );
        numerator_11 = triple_xor_multiplicity;

        lookup_constraints(
            ref sum,
            random_coeff,
            claimed_sum,
            numerator_0,
            numerator_1,
            numerator_2,
            numerator_3,
            numerator_4,
            numerator_5,
            numerator_6,
            numerator_7,
            numerator_8,
            numerator_9,
            numerator_10,
            numerator_11,
            column_size,
            ref interaction_trace_mask_values,
            verify_bitwise_xor_8_sum_0,
            verify_bitwise_xor_8_sum_1,
            verify_bitwise_xor_8_sum_2,
            verify_bitwise_xor_8_sum_3,
            verify_bitwise_xor_8_sum_4,
            verify_bitwise_xor_8_sum_5,
            verify_bitwise_xor_8_sum_6,
            verify_bitwise_xor_8_sum_7,
            gate_sum_8,
            gate_sum_9,
            gate_sum_10,
            gate_sum_11,
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
    numerator_3: QM31,
    numerator_4: QM31,
    numerator_5: QM31,
    numerator_6: QM31,
    numerator_7: QM31,
    numerator_8: QM31,
    numerator_9: QM31,
    numerator_10: QM31,
    numerator_11: QM31,
    column_size: M31,
    ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
    verify_bitwise_xor_8_sum_0: QM31,
    verify_bitwise_xor_8_sum_1: QM31,
    verify_bitwise_xor_8_sum_2: QM31,
    verify_bitwise_xor_8_sum_3: QM31,
    verify_bitwise_xor_8_sum_4: QM31,
    verify_bitwise_xor_8_sum_5: QM31,
    verify_bitwise_xor_8_sum_6: QM31,
    verify_bitwise_xor_8_sum_7: QM31,
    gate_sum_8: QM31,
    gate_sum_9: QM31,
    gate_sum_10: QM31,
    gate_sum_11: QM31,
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
        trace_2_col8,
        trace_2_col9,
        trace_2_col10,
        trace_2_col11,
        trace_2_col12,
        trace_2_col13,
        trace_2_col14,
        trace_2_col15,
        trace_2_col16,
        trace_2_col17,
        trace_2_col18,
        trace_2_col19,
        trace_2_col20,
        trace_2_col21,
        trace_2_col22,
        trace_2_col23,
    ]: [Span<QM31>; 24] =
        (*interaction_trace_mask_values
        .multi_pop_front()
        .unwrap())
        .unbox();

    let [trace_2_col0]: [QM31; 1] = (*trace_2_col0.try_into().unwrap()).unbox();
    let [trace_2_col1]: [QM31; 1] = (*trace_2_col1.try_into().unwrap()).unbox();
    let [trace_2_col2]: [QM31; 1] = (*trace_2_col2.try_into().unwrap()).unbox();
    let [trace_2_col3]: [QM31; 1] = (*trace_2_col3.try_into().unwrap()).unbox();
    let [trace_2_col4]: [QM31; 1] = (*trace_2_col4.try_into().unwrap()).unbox();
    let [trace_2_col5]: [QM31; 1] = (*trace_2_col5.try_into().unwrap()).unbox();
    let [trace_2_col6]: [QM31; 1] = (*trace_2_col6.try_into().unwrap()).unbox();
    let [trace_2_col7]: [QM31; 1] = (*trace_2_col7.try_into().unwrap()).unbox();
    let [trace_2_col8]: [QM31; 1] = (*trace_2_col8.try_into().unwrap()).unbox();
    let [trace_2_col9]: [QM31; 1] = (*trace_2_col9.try_into().unwrap()).unbox();
    let [trace_2_col10]: [QM31; 1] = (*trace_2_col10.try_into().unwrap()).unbox();
    let [trace_2_col11]: [QM31; 1] = (*trace_2_col11.try_into().unwrap()).unbox();
    let [trace_2_col12]: [QM31; 1] = (*trace_2_col12.try_into().unwrap()).unbox();
    let [trace_2_col13]: [QM31; 1] = (*trace_2_col13.try_into().unwrap()).unbox();
    let [trace_2_col14]: [QM31; 1] = (*trace_2_col14.try_into().unwrap()).unbox();
    let [trace_2_col15]: [QM31; 1] = (*trace_2_col15.try_into().unwrap()).unbox();
    let [trace_2_col16]: [QM31; 1] = (*trace_2_col16.try_into().unwrap()).unbox();
    let [trace_2_col17]: [QM31; 1] = (*trace_2_col17.try_into().unwrap()).unbox();
    let [trace_2_col18]: [QM31; 1] = (*trace_2_col18.try_into().unwrap()).unbox();
    let [trace_2_col19]: [QM31; 1] = (*trace_2_col19.try_into().unwrap()).unbox();
    let [trace_2_col20_neg1, trace_2_col20]: [QM31; 2] = (*trace_2_col20.try_into().unwrap())
        .unbox();
    let [trace_2_col21_neg1, trace_2_col21]: [QM31; 2] = (*trace_2_col21.try_into().unwrap())
        .unbox();
    let [trace_2_col22_neg1, trace_2_col22]: [QM31; 2] = (*trace_2_col22.try_into().unwrap())
        .unbox();
    let [trace_2_col23_neg1, trace_2_col23]: [QM31; 2] = (*trace_2_col23.try_into().unwrap())
        .unbox();

    core::internal::revoke_ap_tracking();

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3],
    ))
        * verify_bitwise_xor_8_sum_0
        * verify_bitwise_xor_8_sum_1)
        - (verify_bitwise_xor_8_sum_0 * numerator_1)
        - (verify_bitwise_xor_8_sum_1 * numerator_0));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7],
    )
        - QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3]))
        * verify_bitwise_xor_8_sum_2
        * verify_bitwise_xor_8_sum_3)
        - (verify_bitwise_xor_8_sum_2 * numerator_3)
        - (verify_bitwise_xor_8_sum_3 * numerator_2));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11],
    )
        - QM31Impl::from_partial_evals([trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7]))
        * verify_bitwise_xor_8_sum_4
        * verify_bitwise_xor_8_sum_5)
        - (verify_bitwise_xor_8_sum_4 * numerator_5)
        - (verify_bitwise_xor_8_sum_5 * numerator_4));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
    )
        - QM31Impl::from_partial_evals([trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11]))
        * verify_bitwise_xor_8_sum_6
        * verify_bitwise_xor_8_sum_7)
        - (verify_bitwise_xor_8_sum_6 * numerator_7)
        - (verify_bitwise_xor_8_sum_7 * numerator_6));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
        ))
        * gate_sum_8
        * gate_sum_9)
        - (gate_sum_8 * numerator_9)
        - (gate_sum_9 * numerator_8));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23],
    )
        - QM31Impl::from_partial_evals([trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19])
        - QM31Impl::from_partial_evals(
            [trace_2_col20_neg1, trace_2_col21_neg1, trace_2_col22_neg1, trace_2_col23_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * gate_sum_10
        * gate_sum_11)
        + (gate_sum_10 * numerator_11)
        - (gate_sum_11 * numerator_10));
    sum = sum * random_coeff + constraint_quotient;
}
#[cfg(and(test, feature: "qm31_opcode"))]
mod tests {
    use core::array::ArrayImpl;
    use core::num::traits::Zero;
    #[allow(unused_imports)]
    use stwo_cairo_air::preprocessed_columns::*;
    use stwo_constraint_framework::AirComponent;
    #[allow(unused_imports)]
    use stwo_constraint_framework::{
        LookupElementsTrait, PreprocessedMaskValues, PreprocessedMaskValuesTrait,
    };
    use stwo_verifier_core::fields::qm31::{QM31, QM31Impl, QM31Trait, qm31_const};
    use crate::components::sample_evaluations::*;
    #[allow(unused_imports)]
    use crate::test_utils::{make_interaction_trace, preprocessed_mask_add};
    use crate::utils::*;
    use super::{Claim, Component, InteractionClaim};

    #[test]
    fn test_evaluation_result() {
        let component = Component {
            claim: Claim { log_size: 15 },
            interaction_claim: InteractionClaim {
                claimed_sum: qm31_const::<1398335417, 314974026, 1722107152, 821933968>(),
            },
            common_lookup_elements: LookupElementsTrait::from_z_alpha(
                qm31_const::<445623802, 202571636, 1360224996, 131355117>(),
                qm31_const::<476823935, 939223384, 62486082, 122423602>(),
            ),
        };
        let mut sum: QM31 = Zero::zero();

        let mut preprocessed_trace = PreprocessedMaskValues { values: Default::default() };
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            TRIPLE_XOR_INPUT_ADDR_0_IDX,
            qm31_const::<609298445, 1319370969, 1526988810, 301130926>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            TRIPLE_XOR_INPUT_ADDR_1_IDX,
            qm31_const::<542189266, 1185153241, 1459879946, 301130926>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            TRIPLE_XOR_INPUT_ADDR_2_IDX,
            qm31_const::<475080087, 1050935513, 1392771082, 301130926>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            TRIPLE_XOR_OUTPUT_ADDR_IDX,
            qm31_const::<2078058264, 1287289382, 1925271066, 560922030>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            TRIPLE_XOR_MULTIPLICITY_IDX,
            qm31_const::<576605629, 937297661, 250894038, 1499736593>(),
        );

        let mut trace_columns = [
            [qm31_const::<1659099300, 905558730, 651199673, 1375009625>()].span(),
            [qm31_const::<1591990121, 771341002, 584090809, 1375009625>()].span(),
            [qm31_const::<1793317658, 1173994186, 785417401, 1375009625>()].span(),
            [qm31_const::<1726208479, 1039776458, 718308537, 1375009625>()].span(),
            [qm31_const::<1390662584, 368687818, 382764217, 1375009625>()].span(),
            [qm31_const::<1323553405, 234470090, 315655353, 1375009625>()].span(),
            [qm31_const::<1524880942, 637123274, 516981945, 1375009625>()].span(),
            [qm31_const::<1457771763, 502905546, 449873081, 1375009625>()].span(),
            [qm31_const::<48489085, 1979300555, 1188070585, 1375009625>()].span(),
            [qm31_const::<2128863553, 1845082826, 1120961721, 1375009625>()].span(),
            [qm31_const::<1852335767, 645078115, 2059236183, 343880121>()].span(),
            [qm31_const::<1919444946, 779295843, 2126345047, 343880121>()].span(),
            [qm31_const::<1986554125, 913513571, 45970264, 343880122>()].span(),
            [qm31_const::<2053663304, 1047731299, 113079128, 343880122>()].span(),
            [qm31_const::<1583899051, 108207203, 1790800727, 343880121>()].span(),
            [qm31_const::<1651008230, 242424931, 1857909591, 343880121>()].span(),
            [qm31_const::<1718117409, 376642659, 1925018455, 343880121>()].span(),
            [qm31_const::<1785226588, 510860387, 1992127319, 343880121>()].span(),
            [qm31_const::<1315462335, 1718819938, 1522365270, 343880121>()].span(),
            [qm31_const::<1382571514, 1853037666, 1589474134, 343880121>()].span(),
        ]
            .span();
        let interaction_values = array![
            qm31_const::<1005168032, 79980996, 1847888101, 1941984119>(),
            qm31_const::<1072277211, 214198724, 1914996965, 1941984119>(),
            qm31_const::<1139386390, 348416452, 1982105829, 1941984119>(),
            qm31_const::<1206495569, 482634180, 2049214693, 1941984119>(),
            qm31_const::<736731316, 1690593731, 1579452644, 1941984119>(),
            qm31_const::<803840495, 1824811459, 1646561508, 1941984119>(),
        ];
        let mut interaction_columns = make_interaction_trace(
            interaction_values, qm31_const::<1115374022, 1127856551, 489657863, 643630026>(),
        );
        component
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_trace,
                ref trace_columns,
                ref interaction_columns,
                qm31_const::<474642921, 876336632, 1911695779, 974600512>(),
            );
        preprocessed_trace.validate_usage();
        assert_eq!(sum, QM31Trait::from_fixed_array(TRIPLE_XOR_SAMPLE_EVAL_RESULT))
    }
}
