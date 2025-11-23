// This file was created by the AIR team.

use crate::components::subroutines::decode_instruction_de75a::decode_instruction_de75a_evaluate;
use crate::components::subroutines::read_positive_num_bits_252::read_positive_num_bits_252_evaluate;
use crate::components::subroutines::read_small::read_small_evaluate;
use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 47;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 4] = [
    ('VerifyInstruction', 1), ('MemoryAddressToId', 2), ('MemoryIdToBig', 2), ('Opcodes', 1),
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
        let interaction_log_sizes = [log_size; 16].span();
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
    pub verify_instruction_lookup_elements: crate::VerifyInstructionElements,
    pub memory_address_to_id_lookup_elements: crate::MemoryAddressToIdElements,
    pub memory_id_to_big_lookup_elements: crate::MemoryIdToBigElements,
    pub opcodes_lookup_elements: crate::OpcodesElements,
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
            verify_instruction_lookup_elements: interaction_elements.verify_instruction.clone(),
            memory_address_to_id_lookup_elements: interaction_elements.memory_address_to_id.clone(),
            memory_id_to_big_lookup_elements: interaction_elements.memory_id_to_value.clone(),
            opcodes_lookup_elements: interaction_elements.opcodes.clone(),
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
        let log_size = *(self.claim.log_size);
        let trace_domain = CanonicCosetImpl::new(log_size);
        let domain_vanishing_eval_inv = trace_domain.eval_vanishing(point).inverse();
        let claimed_sum = *self.interaction_claim.claimed_sum;
        let column_size = m31(pow2(log_size));
        let mut verify_instruction_sum_0: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_1: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_2: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_3: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_4: QM31 = Zero::zero();
        let mut opcodes_sum_5: QM31 = Zero::zero();
        let mut opcodes_sum_6: QM31 = Zero::zero();

        let [
            input_pc_col0,
            input_ap_col1,
            input_fp_col2,
            offset0_col3,
            dst_base_fp_col4,
            ap_update_add_1_col5,
            mem_dst_base_col6,
            dst_id_col7,
            dst_limb_0_col8,
            dst_limb_1_col9,
            dst_limb_2_col10,
            dst_limb_3_col11,
            dst_limb_4_col12,
            dst_limb_5_col13,
            dst_limb_6_col14,
            dst_limb_7_col15,
            dst_limb_8_col16,
            dst_limb_9_col17,
            dst_limb_10_col18,
            dst_limb_11_col19,
            dst_limb_12_col20,
            dst_limb_13_col21,
            dst_limb_14_col22,
            dst_limb_15_col23,
            dst_limb_16_col24,
            dst_limb_17_col25,
            dst_limb_18_col26,
            dst_limb_19_col27,
            dst_limb_20_col28,
            dst_limb_21_col29,
            dst_limb_22_col30,
            dst_limb_23_col31,
            dst_limb_24_col32,
            dst_limb_25_col33,
            dst_limb_26_col34,
            dst_limb_27_col35,
            dst_sum_inv_col36,
            dst_sum_squares_inv_col37,
            next_pc_id_col38,
            msb_col39,
            mid_limbs_set_col40,
            next_pc_limb_0_col41,
            next_pc_limb_1_col42,
            next_pc_limb_2_col43,
            remainder_bits_col44,
            partial_limb_msb_col45,
            enabler,
        ]: [Span<QM31>; 47] =
            (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [input_pc_col0]: [QM31; 1] = (*input_pc_col0.try_into().unwrap()).unbox();
        let [input_ap_col1]: [QM31; 1] = (*input_ap_col1.try_into().unwrap()).unbox();
        let [input_fp_col2]: [QM31; 1] = (*input_fp_col2.try_into().unwrap()).unbox();
        let [offset0_col3]: [QM31; 1] = (*offset0_col3.try_into().unwrap()).unbox();
        let [dst_base_fp_col4]: [QM31; 1] = (*dst_base_fp_col4.try_into().unwrap()).unbox();
        let [ap_update_add_1_col5]: [QM31; 1] = (*ap_update_add_1_col5.try_into().unwrap()).unbox();
        let [mem_dst_base_col6]: [QM31; 1] = (*mem_dst_base_col6.try_into().unwrap()).unbox();
        let [dst_id_col7]: [QM31; 1] = (*dst_id_col7.try_into().unwrap()).unbox();
        let [dst_limb_0_col8]: [QM31; 1] = (*dst_limb_0_col8.try_into().unwrap()).unbox();
        let [dst_limb_1_col9]: [QM31; 1] = (*dst_limb_1_col9.try_into().unwrap()).unbox();
        let [dst_limb_2_col10]: [QM31; 1] = (*dst_limb_2_col10.try_into().unwrap()).unbox();
        let [dst_limb_3_col11]: [QM31; 1] = (*dst_limb_3_col11.try_into().unwrap()).unbox();
        let [dst_limb_4_col12]: [QM31; 1] = (*dst_limb_4_col12.try_into().unwrap()).unbox();
        let [dst_limb_5_col13]: [QM31; 1] = (*dst_limb_5_col13.try_into().unwrap()).unbox();
        let [dst_limb_6_col14]: [QM31; 1] = (*dst_limb_6_col14.try_into().unwrap()).unbox();
        let [dst_limb_7_col15]: [QM31; 1] = (*dst_limb_7_col15.try_into().unwrap()).unbox();
        let [dst_limb_8_col16]: [QM31; 1] = (*dst_limb_8_col16.try_into().unwrap()).unbox();
        let [dst_limb_9_col17]: [QM31; 1] = (*dst_limb_9_col17.try_into().unwrap()).unbox();
        let [dst_limb_10_col18]: [QM31; 1] = (*dst_limb_10_col18.try_into().unwrap()).unbox();
        let [dst_limb_11_col19]: [QM31; 1] = (*dst_limb_11_col19.try_into().unwrap()).unbox();
        let [dst_limb_12_col20]: [QM31; 1] = (*dst_limb_12_col20.try_into().unwrap()).unbox();
        let [dst_limb_13_col21]: [QM31; 1] = (*dst_limb_13_col21.try_into().unwrap()).unbox();
        let [dst_limb_14_col22]: [QM31; 1] = (*dst_limb_14_col22.try_into().unwrap()).unbox();
        let [dst_limb_15_col23]: [QM31; 1] = (*dst_limb_15_col23.try_into().unwrap()).unbox();
        let [dst_limb_16_col24]: [QM31; 1] = (*dst_limb_16_col24.try_into().unwrap()).unbox();
        let [dst_limb_17_col25]: [QM31; 1] = (*dst_limb_17_col25.try_into().unwrap()).unbox();
        let [dst_limb_18_col26]: [QM31; 1] = (*dst_limb_18_col26.try_into().unwrap()).unbox();
        let [dst_limb_19_col27]: [QM31; 1] = (*dst_limb_19_col27.try_into().unwrap()).unbox();
        let [dst_limb_20_col28]: [QM31; 1] = (*dst_limb_20_col28.try_into().unwrap()).unbox();
        let [dst_limb_21_col29]: [QM31; 1] = (*dst_limb_21_col29.try_into().unwrap()).unbox();
        let [dst_limb_22_col30]: [QM31; 1] = (*dst_limb_22_col30.try_into().unwrap()).unbox();
        let [dst_limb_23_col31]: [QM31; 1] = (*dst_limb_23_col31.try_into().unwrap()).unbox();
        let [dst_limb_24_col32]: [QM31; 1] = (*dst_limb_24_col32.try_into().unwrap()).unbox();
        let [dst_limb_25_col33]: [QM31; 1] = (*dst_limb_25_col33.try_into().unwrap()).unbox();
        let [dst_limb_26_col34]: [QM31; 1] = (*dst_limb_26_col34.try_into().unwrap()).unbox();
        let [dst_limb_27_col35]: [QM31; 1] = (*dst_limb_27_col35.try_into().unwrap()).unbox();
        let [dst_sum_inv_col36]: [QM31; 1] = (*dst_sum_inv_col36.try_into().unwrap()).unbox();
        let [dst_sum_squares_inv_col37]: [QM31; 1] = (*dst_sum_squares_inv_col37
            .try_into()
            .unwrap())
            .unbox();
        let [next_pc_id_col38]: [QM31; 1] = (*next_pc_id_col38.try_into().unwrap()).unbox();
        let [msb_col39]: [QM31; 1] = (*msb_col39.try_into().unwrap()).unbox();
        let [mid_limbs_set_col40]: [QM31; 1] = (*mid_limbs_set_col40.try_into().unwrap()).unbox();
        let [next_pc_limb_0_col41]: [QM31; 1] = (*next_pc_limb_0_col41.try_into().unwrap()).unbox();
        let [next_pc_limb_1_col42]: [QM31; 1] = (*next_pc_limb_1_col42.try_into().unwrap()).unbox();
        let [next_pc_limb_2_col43]: [QM31; 1] = (*next_pc_limb_2_col43.try_into().unwrap()).unbox();
        let [remainder_bits_col44]: [QM31; 1] = (*remainder_bits_col44.try_into().unwrap()).unbox();
        let [partial_limb_msb_col45]: [QM31; 1] = (*partial_limb_msb_col45.try_into().unwrap())
            .unbox();
        let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        let constraint_quotient = (enabler * enabler - enabler) * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;
        let decode_instruction_de75a_output_tmp_f51a9_5_offset0: QM31 =
            decode_instruction_de75a_evaluate(
            input_pc_col0,
            offset0_col3,
            dst_base_fp_col4,
            ap_update_add_1_col5,
            self.verify_instruction_lookup_elements,
            ref verify_instruction_sum_0,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        // Constraint - mem_dst_base
        let constraint_quotient = ((mem_dst_base_col6
            - ((dst_base_fp_col4 * input_fp_col2)
                + ((qm31_const::<1, 0, 0, 0>() - dst_base_fp_col4) * input_ap_col1))))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;
        read_positive_num_bits_252_evaluate(
            (mem_dst_base_col6 + decode_instruction_de75a_output_tmp_f51a9_5_offset0),
            dst_id_col7,
            dst_limb_0_col8,
            dst_limb_1_col9,
            dst_limb_2_col10,
            dst_limb_3_col11,
            dst_limb_4_col12,
            dst_limb_5_col13,
            dst_limb_6_col14,
            dst_limb_7_col15,
            dst_limb_8_col16,
            dst_limb_9_col17,
            dst_limb_10_col18,
            dst_limb_11_col19,
            dst_limb_12_col20,
            dst_limb_13_col21,
            dst_limb_14_col22,
            dst_limb_15_col23,
            dst_limb_16_col24,
            dst_limb_17_col25,
            dst_limb_18_col26,
            dst_limb_19_col27,
            dst_limb_20_col28,
            dst_limb_21_col29,
            dst_limb_22_col30,
            dst_limb_23_col31,
            dst_limb_24_col32,
            dst_limb_25_col33,
            dst_limb_26_col34,
            dst_limb_27_col35,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref memory_address_to_id_sum_1,
            ref memory_id_to_big_sum_2,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let dst_sum_p_zero_tmp_f51a9_11: QM31 = ((((((((((((((((((((((((dst_limb_1_col9
            + dst_limb_2_col10)
            + dst_limb_3_col11)
            + dst_limb_4_col12)
            + dst_limb_5_col13)
            + dst_limb_6_col14)
            + dst_limb_7_col15)
            + dst_limb_8_col16)
            + dst_limb_9_col17)
            + dst_limb_10_col18)
            + dst_limb_11_col19)
            + dst_limb_12_col20)
            + dst_limb_13_col21)
            + dst_limb_14_col22)
            + dst_limb_15_col23)
            + dst_limb_16_col24)
            + dst_limb_17_col25)
            + dst_limb_18_col26)
            + dst_limb_19_col27)
            + dst_limb_20_col28)
            + dst_limb_22_col30)
            + dst_limb_23_col31)
            + dst_limb_24_col32)
            + dst_limb_25_col33)
            + dst_limb_26_col34);

        // Constraint - dst doesn't equal 0
        let constraint_quotient = ((((dst_sum_p_zero_tmp_f51a9_11
            + ((dst_limb_0_col8 + dst_limb_21_col29) + dst_limb_27_col35))
            * dst_sum_inv_col36)
            - qm31_const::<1, 0, 0, 0>()))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;
        let diff_from_p_tmp_f51a9_12: QM31 = (dst_limb_0_col8 - qm31_const::<1, 0, 0, 0>());
        let diff_from_p_tmp_f51a9_13: QM31 = (dst_limb_21_col29 - qm31_const::<136, 0, 0, 0>());
        let diff_from_p_tmp_f51a9_14: QM31 = (dst_limb_27_col35 - qm31_const::<256, 0, 0, 0>());

        // Constraint - dst doesn't equal P
        let constraint_quotient = ((((dst_sum_p_zero_tmp_f51a9_11
            + (((diff_from_p_tmp_f51a9_12 * diff_from_p_tmp_f51a9_12)
                + (diff_from_p_tmp_f51a9_13 * diff_from_p_tmp_f51a9_13))
                + (diff_from_p_tmp_f51a9_14 * diff_from_p_tmp_f51a9_14)))
            * dst_sum_squares_inv_col37)
            - qm31_const::<1, 0, 0, 0>()))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;
        let read_small_output_tmp_f51a9_24_limb_0: QM31 = read_small_evaluate(
            (input_pc_col0 + qm31_const::<1, 0, 0, 0>()),
            next_pc_id_col38,
            msb_col39,
            mid_limbs_set_col40,
            next_pc_limb_0_col41,
            next_pc_limb_1_col42,
            next_pc_limb_2_col43,
            remainder_bits_col44,
            partial_limb_msb_col45,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref memory_address_to_id_sum_3,
            ref memory_id_to_big_sum_4,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        opcodes_sum_5 = self
            .opcodes_lookup_elements
            .combine_qm31([input_pc_col0, input_ap_col1, input_fp_col2]);

        opcodes_sum_6 = self
            .opcodes_lookup_elements
            .combine_qm31(
                [
                    (input_pc_col0 + read_small_output_tmp_f51a9_24_limb_0),
                    (input_ap_col1 + ap_update_add_1_col5), input_fp_col2,
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
            verify_instruction_sum_0,
            memory_address_to_id_sum_1,
            memory_id_to_big_sum_2,
            memory_address_to_id_sum_3,
            memory_id_to_big_sum_4,
            opcodes_sum_5,
            opcodes_sum_6,
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
    verify_instruction_sum_0: QM31,
    memory_address_to_id_sum_1: QM31,
    memory_id_to_big_sum_2: QM31,
    memory_address_to_id_sum_3: QM31,
    memory_id_to_big_sum_4: QM31,
    opcodes_sum_5: QM31,
    opcodes_sum_6: QM31,
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
    ]: [Span<QM31>; 16] =
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
    let [trace_2_col12_neg1, trace_2_col12]: [QM31; 2] = (*trace_2_col12.try_into().unwrap())
        .unbox();
    let [trace_2_col13_neg1, trace_2_col13]: [QM31; 2] = (*trace_2_col13.try_into().unwrap())
        .unbox();
    let [trace_2_col14_neg1, trace_2_col14]: [QM31; 2] = (*trace_2_col14.try_into().unwrap())
        .unbox();
    let [trace_2_col15_neg1, trace_2_col15]: [QM31; 2] = (*trace_2_col15.try_into().unwrap())
        .unbox();

    core::internal::revoke_ap_tracking();

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3],
    ))
        * verify_instruction_sum_0
        * memory_address_to_id_sum_1)
        - verify_instruction_sum_0
        - memory_address_to_id_sum_1)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7],
    )
        - QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3]))
        * memory_id_to_big_sum_2
        * memory_address_to_id_sum_3)
        - memory_id_to_big_sum_2
        - memory_address_to_id_sum_3)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11],
    )
        - QM31Impl::from_partial_evals([trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7]))
        * memory_id_to_big_sum_4
        * opcodes_sum_5)
        - (memory_id_to_big_sum_4 * enabler)
        - opcodes_sum_5)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
    )
        - QM31Impl::from_partial_evals([trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11])
        - QM31Impl::from_partial_evals(
            [trace_2_col12_neg1, trace_2_col13_neg1, trace_2_col14_neg1, trace_2_col15_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * opcodes_sum_6)
        + enabler)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
}
#[cfg(and(test, feature: "qm31_opcode"))]
mod tests {
    use core::array::ArrayImpl;
    use core::num::traits::Zero;
    #[allow(unused_imports)]
    use stwo_cairo_air::preprocessed_columns::{NUM_PREPROCESSED_COLUMNS, seq_column_idx};
    #[allow(unused_imports)]
    use stwo_constraint_framework::{
        LookupElements, PreprocessedMaskValues, PreprocessedMaskValuesTrait,
    };
    use stwo_verifier_core::circle::CirclePoint;
    use stwo_verifier_core::fields::qm31::{QM31, QM31Impl, QM31Trait, qm31_const};
    use crate::cairo_component::*;
    use crate::components::sample_evaluations::*;
    #[allow(unused_imports)]
    use crate::test_utils::{make_interaction_trace, make_lookup_elements, preprocessed_mask_add};
    use crate::utils::*;
    use super::{Claim, Component, InteractionClaim};

    #[test]
    fn test_evaluation_result() {
        let component = Component {
            claim: Claim { log_size: 15 },
            interaction_claim: InteractionClaim {
                claimed_sum: qm31_const::<1398335417, 314974026, 1722107152, 821933968>(),
            },
            memory_address_to_id_lookup_elements: make_lookup_elements(
                qm31_const::<1842771211, 1960835386, 1582137647, 1333140033>(),
                qm31_const::<1360491305, 950648792, 556642685, 2096522554>(),
            ),
            memory_id_to_big_lookup_elements: make_lookup_elements(
                qm31_const::<844624398, 1166453613, 1247584074, 330174372>(),
                qm31_const::<1844105245, 1400976933, 1126903288, 1155460729>(),
            ),
            opcodes_lookup_elements: make_lookup_elements(
                qm31_const::<363325160, 1257307741, 344122312, 91897123>(),
                qm31_const::<1778746199, 966657378, 28413448, 700868625>(),
            ),
            verify_instruction_lookup_elements: make_lookup_elements(
                qm31_const::<1069488928, 1058545294, 340383544, 1219862120>(),
                qm31_const::<1812811714, 1448895316, 1764436954, 1191872819>(),
            ),
        };
        let mut sum: QM31 = Zero::zero();
        let point = CirclePoint {
            x: qm31_const::<461666434, 38651694, 1083586041, 510305943>(),
            y: qm31_const::<817798294, 862569777, 2091320744, 1178484122>(),
        };

        let mut preprocessed_trace = PreprocessedMaskValues { values: Default::default() };

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
            [qm31_const::<1986820986, 913513739, 45970432, 343880178>()].span(),
            [qm31_const::<1919711807, 779296011, 2126345215, 343880177>()].span(),
            [qm31_const::<2121039344, 1181949195, 180188160, 343880178>()].span(),
            [qm31_const::<2053930165, 1047731467, 113079296, 343880178>()].span(),
            [qm31_const::<1718384270, 376642827, 1925018623, 343880177>()].span(),
            [qm31_const::<1651275091, 242425099, 1857909759, 343880177>()].span(),
            [qm31_const::<1852602628, 645078283, 2059236351, 343880177>()].span(),
            [qm31_const::<1785493449, 510860555, 1992127487, 343880177>()].span(),
            [qm31_const::<1449947554, 1987255562, 1656583166, 343880177>()].span(),
            [qm31_const::<1382838375, 1853037834, 1589474302, 343880177>()].span(),
            [qm31_const::<510356977, 108207322, 717059022, 343880161>()].span(),
            [qm31_const::<577466156, 242425050, 784167886, 343880161>()].span(),
            [qm31_const::<376138619, 1987255513, 582841293, 343880161>()].span(),
            [qm31_const::<443247798, 2121473241, 649950157, 343880161>()].span(),
            [qm31_const::<778793693, 645078234, 985494478, 343880161>()].span(),
            [qm31_const::<845902872, 779295962, 1052603342, 343880161>()].span(),
            [qm31_const::<644575335, 376642778, 851276750, 343880161>()].span(),
            [qm31_const::<711684514, 510860506, 918385614, 343880161>()].span(),
            [qm31_const::<1047230409, 1181949146, 1253929934, 343880161>()].span(),
            [qm31_const::<1114339588, 1316166874, 1321038798, 343880161>()].span(),
            [qm31_const::<1717810224, 376642479, 1925018275, 343880061>()].span(),
            [qm31_const::<1650701045, 242424751, 1857909411, 343880061>()].span(),
            [qm31_const::<1583591866, 108207023, 1790800547, 343880061>()].span(),
            [qm31_const::<1516482687, 2121472942, 1723691682, 343880061>()].span(),
            [qm31_const::<1986246940, 913513391, 45970084, 343880062>()].span(),
            [qm31_const::<1919137761, 779295663, 2126344867, 343880061>()].span(),
            [qm31_const::<179325277, 825275894, 97341591, 1357105975>()].span(),
        ]
            .span();
        let interaction_values = array![
            qm31_const::<1005168032, 79980996, 1847888101, 1941984119>(),
            qm31_const::<1072277211, 214198724, 1914996965, 1941984119>(),
            qm31_const::<1139386390, 348416452, 1982105829, 1941984119>(),
            qm31_const::<1206495569, 482634180, 2049214693, 1941984119>(),
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
                point,
            );
        preprocessed_trace.validate_usage();
        assert_eq!(sum, QM31Trait::from_fixed_array(JNZ_OPCODE_TAKEN_SAMPLE_EVAL_RESULT))
    }
}
