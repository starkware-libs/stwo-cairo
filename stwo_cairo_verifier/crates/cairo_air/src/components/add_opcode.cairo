// This file was created by the AIR team.

use crate::components::subroutines::decode_instruction_bc3cd::decode_instruction_bc3cd_evaluate;
use crate::components::subroutines::read_positive_num_bits_252::read_positive_num_bits_252_evaluate;
use crate::components::subroutines::verify_add_252::verify_add_252_evaluate;
use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 103;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 4] = [
    ('VerifyInstruction', 1), ('MemoryAddressToId', 3), ('MemoryIdToBig', 3), ('Opcodes', 1),
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
        let mut memory_address_to_id_sum_5: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_6: QM31 = Zero::zero();
        let mut opcodes_sum_7: QM31 = Zero::zero();
        let mut opcodes_sum_8: QM31 = Zero::zero();

        let [
            input_pc_col0,
            input_ap_col1,
            input_fp_col2,
            offset0_col3,
            offset1_col4,
            offset2_col5,
            dst_base_fp_col6,
            op0_base_fp_col7,
            op1_imm_col8,
            op1_base_fp_col9,
            ap_update_add_1_col10,
            mem_dst_base_col11,
            mem0_base_col12,
            mem1_base_col13,
            dst_id_col14,
            dst_limb_0_col15,
            dst_limb_1_col16,
            dst_limb_2_col17,
            dst_limb_3_col18,
            dst_limb_4_col19,
            dst_limb_5_col20,
            dst_limb_6_col21,
            dst_limb_7_col22,
            dst_limb_8_col23,
            dst_limb_9_col24,
            dst_limb_10_col25,
            dst_limb_11_col26,
            dst_limb_12_col27,
            dst_limb_13_col28,
            dst_limb_14_col29,
            dst_limb_15_col30,
            dst_limb_16_col31,
            dst_limb_17_col32,
            dst_limb_18_col33,
            dst_limb_19_col34,
            dst_limb_20_col35,
            dst_limb_21_col36,
            dst_limb_22_col37,
            dst_limb_23_col38,
            dst_limb_24_col39,
            dst_limb_25_col40,
            dst_limb_26_col41,
            dst_limb_27_col42,
            op0_id_col43,
            op0_limb_0_col44,
            op0_limb_1_col45,
            op0_limb_2_col46,
            op0_limb_3_col47,
            op0_limb_4_col48,
            op0_limb_5_col49,
            op0_limb_6_col50,
            op0_limb_7_col51,
            op0_limb_8_col52,
            op0_limb_9_col53,
            op0_limb_10_col54,
            op0_limb_11_col55,
            op0_limb_12_col56,
            op0_limb_13_col57,
            op0_limb_14_col58,
            op0_limb_15_col59,
            op0_limb_16_col60,
            op0_limb_17_col61,
            op0_limb_18_col62,
            op0_limb_19_col63,
            op0_limb_20_col64,
            op0_limb_21_col65,
            op0_limb_22_col66,
            op0_limb_23_col67,
            op0_limb_24_col68,
            op0_limb_25_col69,
            op0_limb_26_col70,
            op0_limb_27_col71,
            op1_id_col72,
            op1_limb_0_col73,
            op1_limb_1_col74,
            op1_limb_2_col75,
            op1_limb_3_col76,
            op1_limb_4_col77,
            op1_limb_5_col78,
            op1_limb_6_col79,
            op1_limb_7_col80,
            op1_limb_8_col81,
            op1_limb_9_col82,
            op1_limb_10_col83,
            op1_limb_11_col84,
            op1_limb_12_col85,
            op1_limb_13_col86,
            op1_limb_14_col87,
            op1_limb_15_col88,
            op1_limb_16_col89,
            op1_limb_17_col90,
            op1_limb_18_col91,
            op1_limb_19_col92,
            op1_limb_20_col93,
            op1_limb_21_col94,
            op1_limb_22_col95,
            op1_limb_23_col96,
            op1_limb_24_col97,
            op1_limb_25_col98,
            op1_limb_26_col99,
            op1_limb_27_col100,
            sub_p_bit_col101,
            enabler,
        ]: [Span<QM31>; 103] =
            (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [input_pc_col0]: [QM31; 1] = (*input_pc_col0.try_into().unwrap()).unbox();
        let [input_ap_col1]: [QM31; 1] = (*input_ap_col1.try_into().unwrap()).unbox();
        let [input_fp_col2]: [QM31; 1] = (*input_fp_col2.try_into().unwrap()).unbox();
        let [offset0_col3]: [QM31; 1] = (*offset0_col3.try_into().unwrap()).unbox();
        let [offset1_col4]: [QM31; 1] = (*offset1_col4.try_into().unwrap()).unbox();
        let [offset2_col5]: [QM31; 1] = (*offset2_col5.try_into().unwrap()).unbox();
        let [dst_base_fp_col6]: [QM31; 1] = (*dst_base_fp_col6.try_into().unwrap()).unbox();
        let [op0_base_fp_col7]: [QM31; 1] = (*op0_base_fp_col7.try_into().unwrap()).unbox();
        let [op1_imm_col8]: [QM31; 1] = (*op1_imm_col8.try_into().unwrap()).unbox();
        let [op1_base_fp_col9]: [QM31; 1] = (*op1_base_fp_col9.try_into().unwrap()).unbox();
        let [ap_update_add_1_col10]: [QM31; 1] = (*ap_update_add_1_col10.try_into().unwrap())
            .unbox();
        let [mem_dst_base_col11]: [QM31; 1] = (*mem_dst_base_col11.try_into().unwrap()).unbox();
        let [mem0_base_col12]: [QM31; 1] = (*mem0_base_col12.try_into().unwrap()).unbox();
        let [mem1_base_col13]: [QM31; 1] = (*mem1_base_col13.try_into().unwrap()).unbox();
        let [dst_id_col14]: [QM31; 1] = (*dst_id_col14.try_into().unwrap()).unbox();
        let [dst_limb_0_col15]: [QM31; 1] = (*dst_limb_0_col15.try_into().unwrap()).unbox();
        let [dst_limb_1_col16]: [QM31; 1] = (*dst_limb_1_col16.try_into().unwrap()).unbox();
        let [dst_limb_2_col17]: [QM31; 1] = (*dst_limb_2_col17.try_into().unwrap()).unbox();
        let [dst_limb_3_col18]: [QM31; 1] = (*dst_limb_3_col18.try_into().unwrap()).unbox();
        let [dst_limb_4_col19]: [QM31; 1] = (*dst_limb_4_col19.try_into().unwrap()).unbox();
        let [dst_limb_5_col20]: [QM31; 1] = (*dst_limb_5_col20.try_into().unwrap()).unbox();
        let [dst_limb_6_col21]: [QM31; 1] = (*dst_limb_6_col21.try_into().unwrap()).unbox();
        let [dst_limb_7_col22]: [QM31; 1] = (*dst_limb_7_col22.try_into().unwrap()).unbox();
        let [dst_limb_8_col23]: [QM31; 1] = (*dst_limb_8_col23.try_into().unwrap()).unbox();
        let [dst_limb_9_col24]: [QM31; 1] = (*dst_limb_9_col24.try_into().unwrap()).unbox();
        let [dst_limb_10_col25]: [QM31; 1] = (*dst_limb_10_col25.try_into().unwrap()).unbox();
        let [dst_limb_11_col26]: [QM31; 1] = (*dst_limb_11_col26.try_into().unwrap()).unbox();
        let [dst_limb_12_col27]: [QM31; 1] = (*dst_limb_12_col27.try_into().unwrap()).unbox();
        let [dst_limb_13_col28]: [QM31; 1] = (*dst_limb_13_col28.try_into().unwrap()).unbox();
        let [dst_limb_14_col29]: [QM31; 1] = (*dst_limb_14_col29.try_into().unwrap()).unbox();
        let [dst_limb_15_col30]: [QM31; 1] = (*dst_limb_15_col30.try_into().unwrap()).unbox();
        let [dst_limb_16_col31]: [QM31; 1] = (*dst_limb_16_col31.try_into().unwrap()).unbox();
        let [dst_limb_17_col32]: [QM31; 1] = (*dst_limb_17_col32.try_into().unwrap()).unbox();
        let [dst_limb_18_col33]: [QM31; 1] = (*dst_limb_18_col33.try_into().unwrap()).unbox();
        let [dst_limb_19_col34]: [QM31; 1] = (*dst_limb_19_col34.try_into().unwrap()).unbox();
        let [dst_limb_20_col35]: [QM31; 1] = (*dst_limb_20_col35.try_into().unwrap()).unbox();
        let [dst_limb_21_col36]: [QM31; 1] = (*dst_limb_21_col36.try_into().unwrap()).unbox();
        let [dst_limb_22_col37]: [QM31; 1] = (*dst_limb_22_col37.try_into().unwrap()).unbox();
        let [dst_limb_23_col38]: [QM31; 1] = (*dst_limb_23_col38.try_into().unwrap()).unbox();
        let [dst_limb_24_col39]: [QM31; 1] = (*dst_limb_24_col39.try_into().unwrap()).unbox();
        let [dst_limb_25_col40]: [QM31; 1] = (*dst_limb_25_col40.try_into().unwrap()).unbox();
        let [dst_limb_26_col41]: [QM31; 1] = (*dst_limb_26_col41.try_into().unwrap()).unbox();
        let [dst_limb_27_col42]: [QM31; 1] = (*dst_limb_27_col42.try_into().unwrap()).unbox();
        let [op0_id_col43]: [QM31; 1] = (*op0_id_col43.try_into().unwrap()).unbox();
        let [op0_limb_0_col44]: [QM31; 1] = (*op0_limb_0_col44.try_into().unwrap()).unbox();
        let [op0_limb_1_col45]: [QM31; 1] = (*op0_limb_1_col45.try_into().unwrap()).unbox();
        let [op0_limb_2_col46]: [QM31; 1] = (*op0_limb_2_col46.try_into().unwrap()).unbox();
        let [op0_limb_3_col47]: [QM31; 1] = (*op0_limb_3_col47.try_into().unwrap()).unbox();
        let [op0_limb_4_col48]: [QM31; 1] = (*op0_limb_4_col48.try_into().unwrap()).unbox();
        let [op0_limb_5_col49]: [QM31; 1] = (*op0_limb_5_col49.try_into().unwrap()).unbox();
        let [op0_limb_6_col50]: [QM31; 1] = (*op0_limb_6_col50.try_into().unwrap()).unbox();
        let [op0_limb_7_col51]: [QM31; 1] = (*op0_limb_7_col51.try_into().unwrap()).unbox();
        let [op0_limb_8_col52]: [QM31; 1] = (*op0_limb_8_col52.try_into().unwrap()).unbox();
        let [op0_limb_9_col53]: [QM31; 1] = (*op0_limb_9_col53.try_into().unwrap()).unbox();
        let [op0_limb_10_col54]: [QM31; 1] = (*op0_limb_10_col54.try_into().unwrap()).unbox();
        let [op0_limb_11_col55]: [QM31; 1] = (*op0_limb_11_col55.try_into().unwrap()).unbox();
        let [op0_limb_12_col56]: [QM31; 1] = (*op0_limb_12_col56.try_into().unwrap()).unbox();
        let [op0_limb_13_col57]: [QM31; 1] = (*op0_limb_13_col57.try_into().unwrap()).unbox();
        let [op0_limb_14_col58]: [QM31; 1] = (*op0_limb_14_col58.try_into().unwrap()).unbox();
        let [op0_limb_15_col59]: [QM31; 1] = (*op0_limb_15_col59.try_into().unwrap()).unbox();
        let [op0_limb_16_col60]: [QM31; 1] = (*op0_limb_16_col60.try_into().unwrap()).unbox();
        let [op0_limb_17_col61]: [QM31; 1] = (*op0_limb_17_col61.try_into().unwrap()).unbox();
        let [op0_limb_18_col62]: [QM31; 1] = (*op0_limb_18_col62.try_into().unwrap()).unbox();
        let [op0_limb_19_col63]: [QM31; 1] = (*op0_limb_19_col63.try_into().unwrap()).unbox();
        let [op0_limb_20_col64]: [QM31; 1] = (*op0_limb_20_col64.try_into().unwrap()).unbox();
        let [op0_limb_21_col65]: [QM31; 1] = (*op0_limb_21_col65.try_into().unwrap()).unbox();
        let [op0_limb_22_col66]: [QM31; 1] = (*op0_limb_22_col66.try_into().unwrap()).unbox();
        let [op0_limb_23_col67]: [QM31; 1] = (*op0_limb_23_col67.try_into().unwrap()).unbox();
        let [op0_limb_24_col68]: [QM31; 1] = (*op0_limb_24_col68.try_into().unwrap()).unbox();
        let [op0_limb_25_col69]: [QM31; 1] = (*op0_limb_25_col69.try_into().unwrap()).unbox();
        let [op0_limb_26_col70]: [QM31; 1] = (*op0_limb_26_col70.try_into().unwrap()).unbox();
        let [op0_limb_27_col71]: [QM31; 1] = (*op0_limb_27_col71.try_into().unwrap()).unbox();
        let [op1_id_col72]: [QM31; 1] = (*op1_id_col72.try_into().unwrap()).unbox();
        let [op1_limb_0_col73]: [QM31; 1] = (*op1_limb_0_col73.try_into().unwrap()).unbox();
        let [op1_limb_1_col74]: [QM31; 1] = (*op1_limb_1_col74.try_into().unwrap()).unbox();
        let [op1_limb_2_col75]: [QM31; 1] = (*op1_limb_2_col75.try_into().unwrap()).unbox();
        let [op1_limb_3_col76]: [QM31; 1] = (*op1_limb_3_col76.try_into().unwrap()).unbox();
        let [op1_limb_4_col77]: [QM31; 1] = (*op1_limb_4_col77.try_into().unwrap()).unbox();
        let [op1_limb_5_col78]: [QM31; 1] = (*op1_limb_5_col78.try_into().unwrap()).unbox();
        let [op1_limb_6_col79]: [QM31; 1] = (*op1_limb_6_col79.try_into().unwrap()).unbox();
        let [op1_limb_7_col80]: [QM31; 1] = (*op1_limb_7_col80.try_into().unwrap()).unbox();
        let [op1_limb_8_col81]: [QM31; 1] = (*op1_limb_8_col81.try_into().unwrap()).unbox();
        let [op1_limb_9_col82]: [QM31; 1] = (*op1_limb_9_col82.try_into().unwrap()).unbox();
        let [op1_limb_10_col83]: [QM31; 1] = (*op1_limb_10_col83.try_into().unwrap()).unbox();
        let [op1_limb_11_col84]: [QM31; 1] = (*op1_limb_11_col84.try_into().unwrap()).unbox();
        let [op1_limb_12_col85]: [QM31; 1] = (*op1_limb_12_col85.try_into().unwrap()).unbox();
        let [op1_limb_13_col86]: [QM31; 1] = (*op1_limb_13_col86.try_into().unwrap()).unbox();
        let [op1_limb_14_col87]: [QM31; 1] = (*op1_limb_14_col87.try_into().unwrap()).unbox();
        let [op1_limb_15_col88]: [QM31; 1] = (*op1_limb_15_col88.try_into().unwrap()).unbox();
        let [op1_limb_16_col89]: [QM31; 1] = (*op1_limb_16_col89.try_into().unwrap()).unbox();
        let [op1_limb_17_col90]: [QM31; 1] = (*op1_limb_17_col90.try_into().unwrap()).unbox();
        let [op1_limb_18_col91]: [QM31; 1] = (*op1_limb_18_col91.try_into().unwrap()).unbox();
        let [op1_limb_19_col92]: [QM31; 1] = (*op1_limb_19_col92.try_into().unwrap()).unbox();
        let [op1_limb_20_col93]: [QM31; 1] = (*op1_limb_20_col93.try_into().unwrap()).unbox();
        let [op1_limb_21_col94]: [QM31; 1] = (*op1_limb_21_col94.try_into().unwrap()).unbox();
        let [op1_limb_22_col95]: [QM31; 1] = (*op1_limb_22_col95.try_into().unwrap()).unbox();
        let [op1_limb_23_col96]: [QM31; 1] = (*op1_limb_23_col96.try_into().unwrap()).unbox();
        let [op1_limb_24_col97]: [QM31; 1] = (*op1_limb_24_col97.try_into().unwrap()).unbox();
        let [op1_limb_25_col98]: [QM31; 1] = (*op1_limb_25_col98.try_into().unwrap()).unbox();
        let [op1_limb_26_col99]: [QM31; 1] = (*op1_limb_26_col99.try_into().unwrap()).unbox();
        let [op1_limb_27_col100]: [QM31; 1] = (*op1_limb_27_col100.try_into().unwrap()).unbox();
        let [sub_p_bit_col101]: [QM31; 1] = (*sub_p_bit_col101.try_into().unwrap()).unbox();
        let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        let constraint_quotient = (enabler * enabler - enabler) * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;
        let [
            decode_instruction_bc3cd_output_tmp_3fa46_11_offset0,
            decode_instruction_bc3cd_output_tmp_3fa46_11_offset1,
            decode_instruction_bc3cd_output_tmp_3fa46_11_offset2,
            decode_instruction_bc3cd_output_tmp_3fa46_11_op1_base_ap,
        ] =
            decode_instruction_bc3cd_evaluate(
            input_pc_col0,
            offset0_col3,
            offset1_col4,
            offset2_col5,
            dst_base_fp_col6,
            op0_base_fp_col7,
            op1_imm_col8,
            op1_base_fp_col9,
            ap_update_add_1_col10,
            self.verify_instruction_lookup_elements,
            ref verify_instruction_sum_0,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        // Constraint - if imm then offset2 is 1
        let constraint_quotient = ((op1_imm_col8
            * (qm31_const::<1, 0, 0, 0>() - decode_instruction_bc3cd_output_tmp_3fa46_11_offset2)))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - mem_dst_base
        let constraint_quotient = ((mem_dst_base_col11
            - ((dst_base_fp_col6 * input_fp_col2)
                + ((qm31_const::<1, 0, 0, 0>() - dst_base_fp_col6) * input_ap_col1))))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - mem0_base
        let constraint_quotient = ((mem0_base_col12
            - ((op0_base_fp_col7 * input_fp_col2)
                + ((qm31_const::<1, 0, 0, 0>() - op0_base_fp_col7) * input_ap_col1))))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - mem1_base
        let constraint_quotient = ((mem1_base_col13
            - (((op1_imm_col8 * input_pc_col0) + (op1_base_fp_col9 * input_fp_col2))
                + (decode_instruction_bc3cd_output_tmp_3fa46_11_op1_base_ap * input_ap_col1))))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;
        read_positive_num_bits_252_evaluate(
            (mem_dst_base_col11 + decode_instruction_bc3cd_output_tmp_3fa46_11_offset0),
            dst_id_col14,
            dst_limb_0_col15,
            dst_limb_1_col16,
            dst_limb_2_col17,
            dst_limb_3_col18,
            dst_limb_4_col19,
            dst_limb_5_col20,
            dst_limb_6_col21,
            dst_limb_7_col22,
            dst_limb_8_col23,
            dst_limb_9_col24,
            dst_limb_10_col25,
            dst_limb_11_col26,
            dst_limb_12_col27,
            dst_limb_13_col28,
            dst_limb_14_col29,
            dst_limb_15_col30,
            dst_limb_16_col31,
            dst_limb_17_col32,
            dst_limb_18_col33,
            dst_limb_19_col34,
            dst_limb_20_col35,
            dst_limb_21_col36,
            dst_limb_22_col37,
            dst_limb_23_col38,
            dst_limb_24_col39,
            dst_limb_25_col40,
            dst_limb_26_col41,
            dst_limb_27_col42,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref memory_address_to_id_sum_1,
            ref memory_id_to_big_sum_2,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        read_positive_num_bits_252_evaluate(
            (mem0_base_col12 + decode_instruction_bc3cd_output_tmp_3fa46_11_offset1),
            op0_id_col43,
            op0_limb_0_col44,
            op0_limb_1_col45,
            op0_limb_2_col46,
            op0_limb_3_col47,
            op0_limb_4_col48,
            op0_limb_5_col49,
            op0_limb_6_col50,
            op0_limb_7_col51,
            op0_limb_8_col52,
            op0_limb_9_col53,
            op0_limb_10_col54,
            op0_limb_11_col55,
            op0_limb_12_col56,
            op0_limb_13_col57,
            op0_limb_14_col58,
            op0_limb_15_col59,
            op0_limb_16_col60,
            op0_limb_17_col61,
            op0_limb_18_col62,
            op0_limb_19_col63,
            op0_limb_20_col64,
            op0_limb_21_col65,
            op0_limb_22_col66,
            op0_limb_23_col67,
            op0_limb_24_col68,
            op0_limb_25_col69,
            op0_limb_26_col70,
            op0_limb_27_col71,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref memory_address_to_id_sum_3,
            ref memory_id_to_big_sum_4,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        read_positive_num_bits_252_evaluate(
            (mem1_base_col13 + decode_instruction_bc3cd_output_tmp_3fa46_11_offset2),
            op1_id_col72,
            op1_limb_0_col73,
            op1_limb_1_col74,
            op1_limb_2_col75,
            op1_limb_3_col76,
            op1_limb_4_col77,
            op1_limb_5_col78,
            op1_limb_6_col79,
            op1_limb_7_col80,
            op1_limb_8_col81,
            op1_limb_9_col82,
            op1_limb_10_col83,
            op1_limb_11_col84,
            op1_limb_12_col85,
            op1_limb_13_col86,
            op1_limb_14_col87,
            op1_limb_15_col88,
            op1_limb_16_col89,
            op1_limb_17_col90,
            op1_limb_18_col91,
            op1_limb_19_col92,
            op1_limb_20_col93,
            op1_limb_21_col94,
            op1_limb_22_col95,
            op1_limb_23_col96,
            op1_limb_24_col97,
            op1_limb_25_col98,
            op1_limb_26_col99,
            op1_limb_27_col100,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref memory_address_to_id_sum_5,
            ref memory_id_to_big_sum_6,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        verify_add_252_evaluate(
            [
                op0_limb_0_col44, op0_limb_1_col45, op0_limb_2_col46, op0_limb_3_col47,
                op0_limb_4_col48, op0_limb_5_col49, op0_limb_6_col50, op0_limb_7_col51,
                op0_limb_8_col52, op0_limb_9_col53, op0_limb_10_col54, op0_limb_11_col55,
                op0_limb_12_col56, op0_limb_13_col57, op0_limb_14_col58, op0_limb_15_col59,
                op0_limb_16_col60, op0_limb_17_col61, op0_limb_18_col62, op0_limb_19_col63,
                op0_limb_20_col64, op0_limb_21_col65, op0_limb_22_col66, op0_limb_23_col67,
                op0_limb_24_col68, op0_limb_25_col69, op0_limb_26_col70, op0_limb_27_col71,
                op1_limb_0_col73, op1_limb_1_col74, op1_limb_2_col75, op1_limb_3_col76,
                op1_limb_4_col77, op1_limb_5_col78, op1_limb_6_col79, op1_limb_7_col80,
                op1_limb_8_col81, op1_limb_9_col82, op1_limb_10_col83, op1_limb_11_col84,
                op1_limb_12_col85, op1_limb_13_col86, op1_limb_14_col87, op1_limb_15_col88,
                op1_limb_16_col89, op1_limb_17_col90, op1_limb_18_col91, op1_limb_19_col92,
                op1_limb_20_col93, op1_limb_21_col94, op1_limb_22_col95, op1_limb_23_col96,
                op1_limb_24_col97, op1_limb_25_col98, op1_limb_26_col99, op1_limb_27_col100,
                dst_limb_0_col15, dst_limb_1_col16, dst_limb_2_col17, dst_limb_3_col18,
                dst_limb_4_col19, dst_limb_5_col20, dst_limb_6_col21, dst_limb_7_col22,
                dst_limb_8_col23, dst_limb_9_col24, dst_limb_10_col25, dst_limb_11_col26,
                dst_limb_12_col27, dst_limb_13_col28, dst_limb_14_col29, dst_limb_15_col30,
                dst_limb_16_col31, dst_limb_17_col32, dst_limb_18_col33, dst_limb_19_col34,
                dst_limb_20_col35, dst_limb_21_col36, dst_limb_22_col37, dst_limb_23_col38,
                dst_limb_24_col39, dst_limb_25_col40, dst_limb_26_col41, dst_limb_27_col42,
            ],
            sub_p_bit_col101,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        opcodes_sum_7 = self
            .opcodes_lookup_elements
            .combine_qm31([input_pc_col0, input_ap_col1, input_fp_col2]);

        opcodes_sum_8 = self
            .opcodes_lookup_elements
            .combine_qm31(
                [
                    ((input_pc_col0 + qm31_const::<1, 0, 0, 0>()) + op1_imm_col8),
                    (input_ap_col1 + ap_update_add_1_col10), input_fp_col2,
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
            memory_address_to_id_sum_5,
            memory_id_to_big_sum_6,
            opcodes_sum_7,
            opcodes_sum_8,
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
    memory_address_to_id_sum_5: QM31,
    memory_id_to_big_sum_6: QM31,
    opcodes_sum_7: QM31,
    opcodes_sum_8: QM31,
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
        * memory_address_to_id_sum_5)
        - memory_id_to_big_sum_4
        - memory_address_to_id_sum_5)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
    )
        - QM31Impl::from_partial_evals([trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11]))
        * memory_id_to_big_sum_6
        * opcodes_sum_7)
        - (memory_id_to_big_sum_6 * enabler)
        - opcodes_sum_7)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
    )
        - QM31Impl::from_partial_evals([trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15])
        - QM31Impl::from_partial_evals(
            [trace_2_col16_neg1, trace_2_col17_neg1, trace_2_col18_neg1, trace_2_col19_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * opcodes_sum_8)
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
            [qm31_const::<1852028582, 645077935, 2059236003, 343880061>()].span(),
            [qm31_const::<1784919403, 510860207, 1992127139, 343880061>()].span(),
            [qm31_const::<1180936792, 1450384302, 1388147362, 343880061>()].span(),
            [qm31_const::<1113827613, 1316166574, 1321038498, 343880061>()].span(),
            [qm31_const::<241305891, 1718819697, 448623205, 343880041>()].span(),
            [qm31_const::<308415070, 1853037425, 515732069, 343880041>()].span(),
            [qm31_const::<375524249, 1987255153, 582840933, 343880041>()].span(),
            [qm31_const::<442633428, 2121472881, 649949797, 343880041>()].span(),
            [qm31_const::<509742607, 108206962, 717058662, 343880041>()].span(),
            [qm31_const::<576851786, 242424690, 784167526, 343880041>()].span(),
            [qm31_const::<643960965, 376642418, 851276390, 343880041>()].span(),
            [qm31_const::<711070144, 510860146, 918385254, 343880041>()].span(),
            [qm31_const::<778179323, 645077874, 985494118, 343880041>()].span(),
            [qm31_const::<845288502, 779295602, 1052602982, 343880041>()].span(),
            [qm31_const::<375831434, 1987255333, 582841113, 343880101>()].span(),
            [qm31_const::<308722255, 1853037605, 515732249, 343880101>()].span(),
            [qm31_const::<510049792, 108207142, 717058842, 343880101>()].span(),
            [qm31_const::<442940613, 2121473061, 649949977, 343880101>()].span(),
            [qm31_const::<644268150, 376642598, 851276570, 343880101>()].span(),
            [qm31_const::<577158971, 242424870, 784167706, 343880101>()].span(),
            [qm31_const::<778486508, 645078054, 985494298, 343880101>()].span(),
            [qm31_const::<711377329, 510860326, 918385434, 343880101>()].span(),
            [qm31_const::<912704866, 913513510, 1119712026, 343880101>()].span(),
            [qm31_const::<845595687, 779295782, 1052603162, 343880101>()].span(),
            [qm31_const::<1046820829, 1181948906, 1253929694, 343880081>()].span(),
            [qm31_const::<1113930008, 1316166634, 1321038558, 343880081>()].span(),
            [qm31_const::<912602471, 913513450, 1119711966, 343880081>()].span(),
            [qm31_const::<979711650, 1047731178, 1186820830, 343880081>()].span(),
            [qm31_const::<778384113, 645077994, 985494238, 343880081>()].span(),
            [qm31_const::<845493292, 779295722, 1052603102, 343880081>()].span(),
            [qm31_const::<644165755, 376642538, 851276510, 343880081>()].span(),
            [qm31_const::<711274934, 510860266, 918385374, 343880081>()].span(),
            [qm31_const::<1583694261, 108207083, 1790800607, 343880081>()].span(),
            [qm31_const::<1650803440, 242424811, 1857909471, 343880081>()].span(),
            [qm31_const::<108388425, 1450385012, 314406248, 343880298>()].span(),
            [qm31_const::<41279246, 1316167284, 247297384, 343880298>()].span(),
            [qm31_const::<2121653714, 1181949555, 180188520, 343880298>()].span(),
            [qm31_const::<2054544535, 1047731827, 113079656, 343880298>()].span(),
            [qm31_const::<1987435356, 913514099, 45970792, 343880298>()].span(),
            [qm31_const::<1920326177, 779296371, 2126345575, 343880297>()].span(),
            [qm31_const::<1853216998, 645078643, 2059236711, 343880297>()].span(),
            [qm31_const::<1786107819, 510860915, 1992127847, 343880297>()].span(),
            [qm31_const::<1718998640, 376643187, 1925018983, 343880297>()].span(),
            [qm31_const::<1651889461, 242425459, 1857910119, 343880297>()].span(),
            [qm31_const::<779367739, 645078582, 985494826, 343880277>()].span(),
            [qm31_const::<846476918, 779296310, 1052603690, 343880277>()].span(),
            [qm31_const::<913586097, 913514038, 1119712554, 343880277>()].span(),
            [qm31_const::<980695276, 1047731766, 1186821418, 343880277>()].span(),
            [qm31_const::<510931023, 108207670, 717059370, 343880277>()].span(),
            [qm31_const::<578040202, 242425398, 784168234, 343880277>()].span(),
            [qm31_const::<645149381, 376643126, 851277098, 343880277>()].span(),
            [qm31_const::<712258560, 510860854, 918385962, 343880277>()].span(),
            [qm31_const::<1316241171, 1718820406, 1522365738, 343880277>()].span(),
            [qm31_const::<1383350350, 1853038134, 1589474602, 343880277>()].span(),
            [qm31_const::<1340598866, 536394231, 1198633759, 502514173>()].span(),
            [qm31_const::<1407708045, 670611959, 1265742623, 502514173>()].span(),
            [qm31_const::<179325277, 825275894, 97341591, 1357105975>()].span(),
        ]
            .span();
        let interaction_values = array![
            qm31_const::<1005168032, 79980996, 1847888101, 1941984119>(),
            qm31_const::<1072277211, 214198724, 1914996965, 1941984119>(),
            qm31_const::<1139386390, 348416452, 1982105829, 1941984119>(),
            qm31_const::<1206495569, 482634180, 2049214693, 1941984119>(),
            qm31_const::<736731316, 1690593731, 1579452644, 1941984119>(),
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
        assert_eq!(sum, QM31Trait::from_fixed_array(ADD_OPCODE_SAMPLE_EVAL_RESULT))
    }
}
