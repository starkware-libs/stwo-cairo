// This file was created by the AIR team.

use crate::components::subroutines::bitwise_xor_num_bits_8::bitwise_xor_num_bits_8_evaluate;
use crate::components::subroutines::bitwise_xor_num_bits_8_b::bitwise_xor_num_bits_8_b_evaluate;
use crate::components::subroutines::split_16_low_part_size_8::split_16_low_part_size_8_evaluate;
use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 21;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 2] = [
    ('VerifyBitwiseXor_8', 4), ('VerifyBitwiseXor_8_B', 4),
];

#[derive(Drop, Serde, Copy)]
pub struct Claim {
    pub log_size: u32,
}

pub impl ClaimImpl of ClaimTrait<Claim> {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = *(self.log_size);
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = [log_size; N_TRACE_COLUMNS].span();
        let interaction_log_sizes = [log_size; 20].span();
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
        let mut verify_bitwise_xor_8_b_sum_4: QM31 = Zero::zero();
        let mut numerator_4: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_b_sum_5: QM31 = Zero::zero();
        let mut numerator_5: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_b_sum_6: QM31 = Zero::zero();
        let mut numerator_6: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_b_sum_7: QM31 = Zero::zero();
        let mut numerator_7: QM31 = Zero::zero();
        let mut triple_xor_32_sum_8: QM31 = Zero::zero();
        let mut numerator_8: QM31 = Zero::zero();

        let [
            input_limb_0_col0,
            input_limb_1_col1,
            input_limb_2_col2,
            input_limb_3_col3,
            input_limb_4_col4,
            input_limb_5_col5,
            ms_8_bits_col6,
            ms_8_bits_col7,
            ms_8_bits_col8,
            ms_8_bits_col9,
            ms_8_bits_col10,
            ms_8_bits_col11,
            xor_col12,
            xor_col13,
            xor_col14,
            xor_col15,
            xor_col16,
            xor_col17,
            xor_col18,
            xor_col19,
            enabler_col20,
        ]: [Span<QM31>; 21] =
            (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [input_limb_0_col0]: [QM31; 1] = (*input_limb_0_col0.try_into().unwrap()).unbox();
        let [input_limb_1_col1]: [QM31; 1] = (*input_limb_1_col1.try_into().unwrap()).unbox();
        let [input_limb_2_col2]: [QM31; 1] = (*input_limb_2_col2.try_into().unwrap()).unbox();
        let [input_limb_3_col3]: [QM31; 1] = (*input_limb_3_col3.try_into().unwrap()).unbox();
        let [input_limb_4_col4]: [QM31; 1] = (*input_limb_4_col4.try_into().unwrap()).unbox();
        let [input_limb_5_col5]: [QM31; 1] = (*input_limb_5_col5.try_into().unwrap()).unbox();
        let [ms_8_bits_col6]: [QM31; 1] = (*ms_8_bits_col6.try_into().unwrap()).unbox();
        let [ms_8_bits_col7]: [QM31; 1] = (*ms_8_bits_col7.try_into().unwrap()).unbox();
        let [ms_8_bits_col8]: [QM31; 1] = (*ms_8_bits_col8.try_into().unwrap()).unbox();
        let [ms_8_bits_col9]: [QM31; 1] = (*ms_8_bits_col9.try_into().unwrap()).unbox();
        let [ms_8_bits_col10]: [QM31; 1] = (*ms_8_bits_col10.try_into().unwrap()).unbox();
        let [ms_8_bits_col11]: [QM31; 1] = (*ms_8_bits_col11.try_into().unwrap()).unbox();
        let [xor_col12]: [QM31; 1] = (*xor_col12.try_into().unwrap()).unbox();
        let [xor_col13]: [QM31; 1] = (*xor_col13.try_into().unwrap()).unbox();
        let [xor_col14]: [QM31; 1] = (*xor_col14.try_into().unwrap()).unbox();
        let [xor_col15]: [QM31; 1] = (*xor_col15.try_into().unwrap()).unbox();
        let [xor_col16]: [QM31; 1] = (*xor_col16.try_into().unwrap()).unbox();
        let [xor_col17]: [QM31; 1] = (*xor_col17.try_into().unwrap()).unbox();
        let [xor_col18]: [QM31; 1] = (*xor_col18.try_into().unwrap()).unbox();
        let [xor_col19]: [QM31; 1] = (*xor_col19.try_into().unwrap()).unbox();
        let [enabler_col20]: [QM31; 1] = (*enabler_col20.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        let split_16_low_part_size_8_output_tmp_298db_1_limb_0: QM31 =
            split_16_low_part_size_8_evaluate(
            input_limb_0_col0, ms_8_bits_col6, self.common_lookup_elements, ref sum, random_coeff,
        );
        let split_16_low_part_size_8_output_tmp_298db_3_limb_0: QM31 =
            split_16_low_part_size_8_evaluate(
            input_limb_1_col1, ms_8_bits_col7, self.common_lookup_elements, ref sum, random_coeff,
        );
        let split_16_low_part_size_8_output_tmp_298db_5_limb_0: QM31 =
            split_16_low_part_size_8_evaluate(
            input_limb_2_col2, ms_8_bits_col8, self.common_lookup_elements, ref sum, random_coeff,
        );
        let split_16_low_part_size_8_output_tmp_298db_7_limb_0: QM31 =
            split_16_low_part_size_8_evaluate(
            input_limb_3_col3, ms_8_bits_col9, self.common_lookup_elements, ref sum, random_coeff,
        );
        let split_16_low_part_size_8_output_tmp_298db_9_limb_0: QM31 =
            split_16_low_part_size_8_evaluate(
            input_limb_4_col4, ms_8_bits_col10, self.common_lookup_elements, ref sum, random_coeff,
        );
        let split_16_low_part_size_8_output_tmp_298db_11_limb_0: QM31 =
            split_16_low_part_size_8_evaluate(
            input_limb_5_col5, ms_8_bits_col11, self.common_lookup_elements, ref sum, random_coeff,
        );
        bitwise_xor_num_bits_8_evaluate(
            [
                split_16_low_part_size_8_output_tmp_298db_1_limb_0,
                split_16_low_part_size_8_output_tmp_298db_5_limb_0,
            ],
            xor_col12,
            self.common_lookup_elements,
            ref verify_bitwise_xor_8_sum_0,
            ref numerator_0,
            ref sum,
            random_coeff,
        );
        bitwise_xor_num_bits_8_evaluate(
            [xor_col12, split_16_low_part_size_8_output_tmp_298db_9_limb_0],
            xor_col13,
            self.common_lookup_elements,
            ref verify_bitwise_xor_8_sum_1,
            ref numerator_1,
            ref sum,
            random_coeff,
        );
        bitwise_xor_num_bits_8_evaluate(
            [ms_8_bits_col6, ms_8_bits_col8],
            xor_col14,
            self.common_lookup_elements,
            ref verify_bitwise_xor_8_sum_2,
            ref numerator_2,
            ref sum,
            random_coeff,
        );
        bitwise_xor_num_bits_8_evaluate(
            [xor_col14, ms_8_bits_col10],
            xor_col15,
            self.common_lookup_elements,
            ref verify_bitwise_xor_8_sum_3,
            ref numerator_3,
            ref sum,
            random_coeff,
        );
        bitwise_xor_num_bits_8_b_evaluate(
            [
                split_16_low_part_size_8_output_tmp_298db_3_limb_0,
                split_16_low_part_size_8_output_tmp_298db_7_limb_0,
            ],
            xor_col16,
            self.common_lookup_elements,
            ref verify_bitwise_xor_8_b_sum_4,
            ref numerator_4,
            ref sum,
            random_coeff,
        );
        bitwise_xor_num_bits_8_b_evaluate(
            [xor_col16, split_16_low_part_size_8_output_tmp_298db_11_limb_0],
            xor_col17,
            self.common_lookup_elements,
            ref verify_bitwise_xor_8_b_sum_5,
            ref numerator_5,
            ref sum,
            random_coeff,
        );
        bitwise_xor_num_bits_8_b_evaluate(
            [ms_8_bits_col7, ms_8_bits_col9],
            xor_col18,
            self.common_lookup_elements,
            ref verify_bitwise_xor_8_b_sum_6,
            ref numerator_6,
            ref sum,
            random_coeff,
        );
        bitwise_xor_num_bits_8_b_evaluate(
            [xor_col18, ms_8_bits_col11],
            xor_col19,
            self.common_lookup_elements,
            ref verify_bitwise_xor_8_b_sum_7,
            ref numerator_7,
            ref sum,
            random_coeff,
        );
        let triple_xor32_output_tmp_298db_28_limb_0: QM31 = (xor_col13
            + (xor_col15 * qm31_const::<256, 0, 0, 0>()));
        let triple_xor32_output_tmp_298db_28_limb_1: QM31 = (xor_col17
            + (xor_col19 * qm31_const::<256, 0, 0, 0>()));

        // Constraint - Enabler is a bit
        let constraint_quotient = (((enabler_col20 * enabler_col20) - enabler_col20));
        sum = sum * random_coeff + constraint_quotient;

        triple_xor_32_sum_8 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<990559919, 0, 0, 0>(), input_limb_0_col0, input_limb_1_col1,
                    input_limb_2_col2, input_limb_3_col3, input_limb_4_col4, input_limb_5_col5,
                    triple_xor32_output_tmp_298db_28_limb_0,
                    triple_xor32_output_tmp_298db_28_limb_1,
                ]
                    .span(),
            );
        numerator_8 = enabler_col20;

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
            column_size,
            ref interaction_trace_mask_values,
            verify_bitwise_xor_8_sum_0,
            verify_bitwise_xor_8_sum_1,
            verify_bitwise_xor_8_sum_2,
            verify_bitwise_xor_8_sum_3,
            verify_bitwise_xor_8_b_sum_4,
            verify_bitwise_xor_8_b_sum_5,
            verify_bitwise_xor_8_b_sum_6,
            verify_bitwise_xor_8_b_sum_7,
            triple_xor_32_sum_8,
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
    column_size: M31,
    ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
    verify_bitwise_xor_8_sum_0: QM31,
    verify_bitwise_xor_8_sum_1: QM31,
    verify_bitwise_xor_8_sum_2: QM31,
    verify_bitwise_xor_8_sum_3: QM31,
    verify_bitwise_xor_8_b_sum_4: QM31,
    verify_bitwise_xor_8_b_sum_5: QM31,
    verify_bitwise_xor_8_b_sum_6: QM31,
    verify_bitwise_xor_8_b_sum_7: QM31,
    triple_xor_32_sum_8: QM31,
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
    ]: [Span<QM31>; 20] =
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
    let [trace_2_col16_neg1, trace_2_col16]: [QM31; 2] = (*trace_2_col16.try_into().unwrap())
        .unbox();
    let [trace_2_col17_neg1, trace_2_col17]: [QM31; 2] = (*trace_2_col17.try_into().unwrap())
        .unbox();
    let [trace_2_col18_neg1, trace_2_col18]: [QM31; 2] = (*trace_2_col18.try_into().unwrap())
        .unbox();
    let [trace_2_col19_neg1, trace_2_col19]: [QM31; 2] = (*trace_2_col19.try_into().unwrap())
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
        * verify_bitwise_xor_8_b_sum_4
        * verify_bitwise_xor_8_b_sum_5)
        - (verify_bitwise_xor_8_b_sum_4 * numerator_5)
        - (verify_bitwise_xor_8_b_sum_5 * numerator_4));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
    )
        - QM31Impl::from_partial_evals([trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11]))
        * verify_bitwise_xor_8_b_sum_6
        * verify_bitwise_xor_8_b_sum_7)
        - (verify_bitwise_xor_8_b_sum_6 * numerator_7)
        - (verify_bitwise_xor_8_b_sum_7 * numerator_6));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
    )
        - QM31Impl::from_partial_evals([trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15])
        - QM31Impl::from_partial_evals(
            [trace_2_col16_neg1, trace_2_col17_neg1, trace_2_col18_neg1, trace_2_col19_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * triple_xor_32_sum_8)
        + numerator_8);
    sum = sum * random_coeff + constraint_quotient;
}
