// This file was created by the AIR team.

use crate::components::subroutines::create_blake_output::create_blake_output_evaluate;
use crate::components::subroutines::create_blake_round_input::create_blake_round_input_evaluate;
use crate::components::subroutines::decode_blake_opcode::decode_blake_opcode_evaluate;
use crate::components::subroutines::verify_u_32::verify_u_32_evaluate;
use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 174;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 8] = [
    ('VerifyInstruction', 1), ('MemoryAddressToId', 20), ('MemoryIdToBig', 20),
    ('RangeCheck_7_2_5', 17), ('VerifyBitwiseXor_8', 4), ('BlakeRound', 1), ('TripleXor32', 8),
    ('Opcodes', 1),
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
        let interaction_log_sizes = [log_size; 148].span();
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
    pub range_check_7_2_5_lookup_elements: crate::RangeCheck_7_2_5Elements,
    pub verify_bitwise_xor_8_lookup_elements: crate::VerifyBitwiseXor_8Elements,
    pub blake_round_lookup_elements: crate::BlakeRoundElements,
    pub triple_xor_32_lookup_elements: crate::TripleXor32Elements,
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
            range_check_7_2_5_lookup_elements: interaction_elements.range_checks.rc_7_2_5.clone(),
            verify_bitwise_xor_8_lookup_elements: interaction_elements.verify_bitwise_xor_8.clone(),
            blake_round_lookup_elements: interaction_elements.blake_round.clone(),
            triple_xor_32_lookup_elements: interaction_elements.triple_xor_32.clone(),
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
        let mut range_check_7_2_5_sum_7: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_8: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_9: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_10: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_11: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_12: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_13: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_14: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_15: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_16: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_17: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_18: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_19: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_20: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_21: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_22: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_23: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_24: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_25: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_26: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_27: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_28: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_29: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_30: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_31: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_32: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_33: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_sum_34: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_sum_35: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_sum_36: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_sum_37: QM31 = Zero::zero();
        let mut blake_round_sum_38: QM31 = Zero::zero();
        let mut blake_round_sum_39: QM31 = Zero::zero();
        let mut triple_xor_32_sum_40: QM31 = Zero::zero();
        let mut triple_xor_32_sum_41: QM31 = Zero::zero();
        let mut triple_xor_32_sum_42: QM31 = Zero::zero();
        let mut triple_xor_32_sum_43: QM31 = Zero::zero();
        let mut triple_xor_32_sum_44: QM31 = Zero::zero();
        let mut triple_xor_32_sum_45: QM31 = Zero::zero();
        let mut triple_xor_32_sum_46: QM31 = Zero::zero();
        let mut triple_xor_32_sum_47: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_48: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_49: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_50: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_51: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_52: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_53: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_54: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_55: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_56: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_57: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_58: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_59: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_60: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_61: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_62: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_63: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_64: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_65: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_66: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_67: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_68: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_69: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_70: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_71: QM31 = Zero::zero();
        let mut opcodes_sum_72: QM31 = Zero::zero();
        let mut opcodes_sum_73: QM31 = Zero::zero();
        let seq = preprocessed_mask_values
            .get_and_mark_used(seq_column_idx(*(self.claim.log_size)));

        let [
            input_pc_col0,
            input_ap_col1,
            input_fp_col2,
            offset0_col3,
            offset1_col4,
            offset2_col5,
            dst_base_fp_col6,
            op0_base_fp_col7,
            op1_base_fp_col8,
            ap_update_add_1_col9,
            opcode_extension_col10,
            mem0_base_col11,
            op0_id_col12,
            op0_limb_0_col13,
            op0_limb_1_col14,
            op0_limb_2_col15,
            op0_limb_3_col16,
            partial_limb_msb_col17,
            mem1_base_col18,
            op1_id_col19,
            op1_limb_0_col20,
            op1_limb_1_col21,
            op1_limb_2_col22,
            op1_limb_3_col23,
            partial_limb_msb_col24,
            ap_id_col25,
            ap_limb_0_col26,
            ap_limb_1_col27,
            ap_limb_2_col28,
            ap_limb_3_col29,
            partial_limb_msb_col30,
            mem_dst_base_col31,
            low_16_bits_col32,
            high_16_bits_col33,
            low_7_ms_bits_col34,
            high_14_ms_bits_col35,
            high_5_ms_bits_col36,
            dst_id_col37,
            low_16_bits_col38,
            high_16_bits_col39,
            low_7_ms_bits_col40,
            high_14_ms_bits_col41,
            high_5_ms_bits_col42,
            state_0_id_col43,
            low_16_bits_col44,
            high_16_bits_col45,
            low_7_ms_bits_col46,
            high_14_ms_bits_col47,
            high_5_ms_bits_col48,
            state_1_id_col49,
            low_16_bits_col50,
            high_16_bits_col51,
            low_7_ms_bits_col52,
            high_14_ms_bits_col53,
            high_5_ms_bits_col54,
            state_2_id_col55,
            low_16_bits_col56,
            high_16_bits_col57,
            low_7_ms_bits_col58,
            high_14_ms_bits_col59,
            high_5_ms_bits_col60,
            state_3_id_col61,
            low_16_bits_col62,
            high_16_bits_col63,
            low_7_ms_bits_col64,
            high_14_ms_bits_col65,
            high_5_ms_bits_col66,
            state_4_id_col67,
            low_16_bits_col68,
            high_16_bits_col69,
            low_7_ms_bits_col70,
            high_14_ms_bits_col71,
            high_5_ms_bits_col72,
            state_5_id_col73,
            low_16_bits_col74,
            high_16_bits_col75,
            low_7_ms_bits_col76,
            high_14_ms_bits_col77,
            high_5_ms_bits_col78,
            state_6_id_col79,
            low_16_bits_col80,
            high_16_bits_col81,
            low_7_ms_bits_col82,
            high_14_ms_bits_col83,
            high_5_ms_bits_col84,
            state_7_id_col85,
            ms_8_bits_col86,
            ms_8_bits_col87,
            xor_col88,
            xor_col89,
            xor_col90,
            xor_col91,
            blake_round_output_limb_0_col92,
            blake_round_output_limb_1_col93,
            blake_round_output_limb_2_col94,
            blake_round_output_limb_3_col95,
            blake_round_output_limb_4_col96,
            blake_round_output_limb_5_col97,
            blake_round_output_limb_6_col98,
            blake_round_output_limb_7_col99,
            blake_round_output_limb_8_col100,
            blake_round_output_limb_9_col101,
            blake_round_output_limb_10_col102,
            blake_round_output_limb_11_col103,
            blake_round_output_limb_12_col104,
            blake_round_output_limb_13_col105,
            blake_round_output_limb_14_col106,
            blake_round_output_limb_15_col107,
            blake_round_output_limb_16_col108,
            blake_round_output_limb_17_col109,
            blake_round_output_limb_18_col110,
            blake_round_output_limb_19_col111,
            blake_round_output_limb_20_col112,
            blake_round_output_limb_21_col113,
            blake_round_output_limb_22_col114,
            blake_round_output_limb_23_col115,
            blake_round_output_limb_24_col116,
            blake_round_output_limb_25_col117,
            blake_round_output_limb_26_col118,
            blake_round_output_limb_27_col119,
            blake_round_output_limb_28_col120,
            blake_round_output_limb_29_col121,
            blake_round_output_limb_30_col122,
            blake_round_output_limb_31_col123,
            blake_round_output_limb_32_col124,
            triple_xor_32_output_limb_0_col125,
            triple_xor_32_output_limb_1_col126,
            triple_xor_32_output_limb_0_col127,
            triple_xor_32_output_limb_1_col128,
            triple_xor_32_output_limb_0_col129,
            triple_xor_32_output_limb_1_col130,
            triple_xor_32_output_limb_0_col131,
            triple_xor_32_output_limb_1_col132,
            triple_xor_32_output_limb_0_col133,
            triple_xor_32_output_limb_1_col134,
            triple_xor_32_output_limb_0_col135,
            triple_xor_32_output_limb_1_col136,
            triple_xor_32_output_limb_0_col137,
            triple_xor_32_output_limb_1_col138,
            triple_xor_32_output_limb_0_col139,
            triple_xor_32_output_limb_1_col140,
            low_7_ms_bits_col141,
            high_14_ms_bits_col142,
            high_5_ms_bits_col143,
            new_state_0_id_col144,
            low_7_ms_bits_col145,
            high_14_ms_bits_col146,
            high_5_ms_bits_col147,
            new_state_1_id_col148,
            low_7_ms_bits_col149,
            high_14_ms_bits_col150,
            high_5_ms_bits_col151,
            new_state_2_id_col152,
            low_7_ms_bits_col153,
            high_14_ms_bits_col154,
            high_5_ms_bits_col155,
            new_state_3_id_col156,
            low_7_ms_bits_col157,
            high_14_ms_bits_col158,
            high_5_ms_bits_col159,
            new_state_4_id_col160,
            low_7_ms_bits_col161,
            high_14_ms_bits_col162,
            high_5_ms_bits_col163,
            new_state_5_id_col164,
            low_7_ms_bits_col165,
            high_14_ms_bits_col166,
            high_5_ms_bits_col167,
            new_state_6_id_col168,
            low_7_ms_bits_col169,
            high_14_ms_bits_col170,
            high_5_ms_bits_col171,
            new_state_7_id_col172,
            enabler,
        ]: [Span<QM31>; 174] =
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
        let [op1_base_fp_col8]: [QM31; 1] = (*op1_base_fp_col8.try_into().unwrap()).unbox();
        let [ap_update_add_1_col9]: [QM31; 1] = (*ap_update_add_1_col9.try_into().unwrap()).unbox();
        let [opcode_extension_col10]: [QM31; 1] = (*opcode_extension_col10.try_into().unwrap())
            .unbox();
        let [mem0_base_col11]: [QM31; 1] = (*mem0_base_col11.try_into().unwrap()).unbox();
        let [op0_id_col12]: [QM31; 1] = (*op0_id_col12.try_into().unwrap()).unbox();
        let [op0_limb_0_col13]: [QM31; 1] = (*op0_limb_0_col13.try_into().unwrap()).unbox();
        let [op0_limb_1_col14]: [QM31; 1] = (*op0_limb_1_col14.try_into().unwrap()).unbox();
        let [op0_limb_2_col15]: [QM31; 1] = (*op0_limb_2_col15.try_into().unwrap()).unbox();
        let [op0_limb_3_col16]: [QM31; 1] = (*op0_limb_3_col16.try_into().unwrap()).unbox();
        let [partial_limb_msb_col17]: [QM31; 1] = (*partial_limb_msb_col17.try_into().unwrap())
            .unbox();
        let [mem1_base_col18]: [QM31; 1] = (*mem1_base_col18.try_into().unwrap()).unbox();
        let [op1_id_col19]: [QM31; 1] = (*op1_id_col19.try_into().unwrap()).unbox();
        let [op1_limb_0_col20]: [QM31; 1] = (*op1_limb_0_col20.try_into().unwrap()).unbox();
        let [op1_limb_1_col21]: [QM31; 1] = (*op1_limb_1_col21.try_into().unwrap()).unbox();
        let [op1_limb_2_col22]: [QM31; 1] = (*op1_limb_2_col22.try_into().unwrap()).unbox();
        let [op1_limb_3_col23]: [QM31; 1] = (*op1_limb_3_col23.try_into().unwrap()).unbox();
        let [partial_limb_msb_col24]: [QM31; 1] = (*partial_limb_msb_col24.try_into().unwrap())
            .unbox();
        let [ap_id_col25]: [QM31; 1] = (*ap_id_col25.try_into().unwrap()).unbox();
        let [ap_limb_0_col26]: [QM31; 1] = (*ap_limb_0_col26.try_into().unwrap()).unbox();
        let [ap_limb_1_col27]: [QM31; 1] = (*ap_limb_1_col27.try_into().unwrap()).unbox();
        let [ap_limb_2_col28]: [QM31; 1] = (*ap_limb_2_col28.try_into().unwrap()).unbox();
        let [ap_limb_3_col29]: [QM31; 1] = (*ap_limb_3_col29.try_into().unwrap()).unbox();
        let [partial_limb_msb_col30]: [QM31; 1] = (*partial_limb_msb_col30.try_into().unwrap())
            .unbox();
        let [mem_dst_base_col31]: [QM31; 1] = (*mem_dst_base_col31.try_into().unwrap()).unbox();
        let [low_16_bits_col32]: [QM31; 1] = (*low_16_bits_col32.try_into().unwrap()).unbox();
        let [high_16_bits_col33]: [QM31; 1] = (*high_16_bits_col33.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col34]: [QM31; 1] = (*low_7_ms_bits_col34.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col35]: [QM31; 1] = (*high_14_ms_bits_col35.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col36]: [QM31; 1] = (*high_5_ms_bits_col36.try_into().unwrap()).unbox();
        let [dst_id_col37]: [QM31; 1] = (*dst_id_col37.try_into().unwrap()).unbox();
        let [low_16_bits_col38]: [QM31; 1] = (*low_16_bits_col38.try_into().unwrap()).unbox();
        let [high_16_bits_col39]: [QM31; 1] = (*high_16_bits_col39.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col40]: [QM31; 1] = (*low_7_ms_bits_col40.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col41]: [QM31; 1] = (*high_14_ms_bits_col41.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col42]: [QM31; 1] = (*high_5_ms_bits_col42.try_into().unwrap()).unbox();
        let [state_0_id_col43]: [QM31; 1] = (*state_0_id_col43.try_into().unwrap()).unbox();
        let [low_16_bits_col44]: [QM31; 1] = (*low_16_bits_col44.try_into().unwrap()).unbox();
        let [high_16_bits_col45]: [QM31; 1] = (*high_16_bits_col45.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col46]: [QM31; 1] = (*low_7_ms_bits_col46.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col47]: [QM31; 1] = (*high_14_ms_bits_col47.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col48]: [QM31; 1] = (*high_5_ms_bits_col48.try_into().unwrap()).unbox();
        let [state_1_id_col49]: [QM31; 1] = (*state_1_id_col49.try_into().unwrap()).unbox();
        let [low_16_bits_col50]: [QM31; 1] = (*low_16_bits_col50.try_into().unwrap()).unbox();
        let [high_16_bits_col51]: [QM31; 1] = (*high_16_bits_col51.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col52]: [QM31; 1] = (*low_7_ms_bits_col52.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col53]: [QM31; 1] = (*high_14_ms_bits_col53.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col54]: [QM31; 1] = (*high_5_ms_bits_col54.try_into().unwrap()).unbox();
        let [state_2_id_col55]: [QM31; 1] = (*state_2_id_col55.try_into().unwrap()).unbox();
        let [low_16_bits_col56]: [QM31; 1] = (*low_16_bits_col56.try_into().unwrap()).unbox();
        let [high_16_bits_col57]: [QM31; 1] = (*high_16_bits_col57.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col58]: [QM31; 1] = (*low_7_ms_bits_col58.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col59]: [QM31; 1] = (*high_14_ms_bits_col59.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col60]: [QM31; 1] = (*high_5_ms_bits_col60.try_into().unwrap()).unbox();
        let [state_3_id_col61]: [QM31; 1] = (*state_3_id_col61.try_into().unwrap()).unbox();
        let [low_16_bits_col62]: [QM31; 1] = (*low_16_bits_col62.try_into().unwrap()).unbox();
        let [high_16_bits_col63]: [QM31; 1] = (*high_16_bits_col63.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col64]: [QM31; 1] = (*low_7_ms_bits_col64.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col65]: [QM31; 1] = (*high_14_ms_bits_col65.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col66]: [QM31; 1] = (*high_5_ms_bits_col66.try_into().unwrap()).unbox();
        let [state_4_id_col67]: [QM31; 1] = (*state_4_id_col67.try_into().unwrap()).unbox();
        let [low_16_bits_col68]: [QM31; 1] = (*low_16_bits_col68.try_into().unwrap()).unbox();
        let [high_16_bits_col69]: [QM31; 1] = (*high_16_bits_col69.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col70]: [QM31; 1] = (*low_7_ms_bits_col70.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col71]: [QM31; 1] = (*high_14_ms_bits_col71.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col72]: [QM31; 1] = (*high_5_ms_bits_col72.try_into().unwrap()).unbox();
        let [state_5_id_col73]: [QM31; 1] = (*state_5_id_col73.try_into().unwrap()).unbox();
        let [low_16_bits_col74]: [QM31; 1] = (*low_16_bits_col74.try_into().unwrap()).unbox();
        let [high_16_bits_col75]: [QM31; 1] = (*high_16_bits_col75.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col76]: [QM31; 1] = (*low_7_ms_bits_col76.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col77]: [QM31; 1] = (*high_14_ms_bits_col77.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col78]: [QM31; 1] = (*high_5_ms_bits_col78.try_into().unwrap()).unbox();
        let [state_6_id_col79]: [QM31; 1] = (*state_6_id_col79.try_into().unwrap()).unbox();
        let [low_16_bits_col80]: [QM31; 1] = (*low_16_bits_col80.try_into().unwrap()).unbox();
        let [high_16_bits_col81]: [QM31; 1] = (*high_16_bits_col81.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col82]: [QM31; 1] = (*low_7_ms_bits_col82.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col83]: [QM31; 1] = (*high_14_ms_bits_col83.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col84]: [QM31; 1] = (*high_5_ms_bits_col84.try_into().unwrap()).unbox();
        let [state_7_id_col85]: [QM31; 1] = (*state_7_id_col85.try_into().unwrap()).unbox();
        let [ms_8_bits_col86]: [QM31; 1] = (*ms_8_bits_col86.try_into().unwrap()).unbox();
        let [ms_8_bits_col87]: [QM31; 1] = (*ms_8_bits_col87.try_into().unwrap()).unbox();
        let [xor_col88]: [QM31; 1] = (*xor_col88.try_into().unwrap()).unbox();
        let [xor_col89]: [QM31; 1] = (*xor_col89.try_into().unwrap()).unbox();
        let [xor_col90]: [QM31; 1] = (*xor_col90.try_into().unwrap()).unbox();
        let [xor_col91]: [QM31; 1] = (*xor_col91.try_into().unwrap()).unbox();
        let [blake_round_output_limb_0_col92]: [QM31; 1] = (*blake_round_output_limb_0_col92
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_1_col93]: [QM31; 1] = (*blake_round_output_limb_1_col93
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_2_col94]: [QM31; 1] = (*blake_round_output_limb_2_col94
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_3_col95]: [QM31; 1] = (*blake_round_output_limb_3_col95
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_4_col96]: [QM31; 1] = (*blake_round_output_limb_4_col96
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_5_col97]: [QM31; 1] = (*blake_round_output_limb_5_col97
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_6_col98]: [QM31; 1] = (*blake_round_output_limb_6_col98
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_7_col99]: [QM31; 1] = (*blake_round_output_limb_7_col99
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_8_col100]: [QM31; 1] = (*blake_round_output_limb_8_col100
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_9_col101]: [QM31; 1] = (*blake_round_output_limb_9_col101
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_10_col102]: [QM31; 1] = (*blake_round_output_limb_10_col102
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_11_col103]: [QM31; 1] = (*blake_round_output_limb_11_col103
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_12_col104]: [QM31; 1] = (*blake_round_output_limb_12_col104
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_13_col105]: [QM31; 1] = (*blake_round_output_limb_13_col105
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_14_col106]: [QM31; 1] = (*blake_round_output_limb_14_col106
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_15_col107]: [QM31; 1] = (*blake_round_output_limb_15_col107
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_16_col108]: [QM31; 1] = (*blake_round_output_limb_16_col108
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_17_col109]: [QM31; 1] = (*blake_round_output_limb_17_col109
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_18_col110]: [QM31; 1] = (*blake_round_output_limb_18_col110
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_19_col111]: [QM31; 1] = (*blake_round_output_limb_19_col111
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_20_col112]: [QM31; 1] = (*blake_round_output_limb_20_col112
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_21_col113]: [QM31; 1] = (*blake_round_output_limb_21_col113
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_22_col114]: [QM31; 1] = (*blake_round_output_limb_22_col114
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_23_col115]: [QM31; 1] = (*blake_round_output_limb_23_col115
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_24_col116]: [QM31; 1] = (*blake_round_output_limb_24_col116
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_25_col117]: [QM31; 1] = (*blake_round_output_limb_25_col117
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_26_col118]: [QM31; 1] = (*blake_round_output_limb_26_col118
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_27_col119]: [QM31; 1] = (*blake_round_output_limb_27_col119
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_28_col120]: [QM31; 1] = (*blake_round_output_limb_28_col120
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_29_col121]: [QM31; 1] = (*blake_round_output_limb_29_col121
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_30_col122]: [QM31; 1] = (*blake_round_output_limb_30_col122
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_31_col123]: [QM31; 1] = (*blake_round_output_limb_31_col123
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_32_col124]: [QM31; 1] = (*blake_round_output_limb_32_col124
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_0_col125]: [QM31; 1] = (*triple_xor_32_output_limb_0_col125
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_1_col126]: [QM31; 1] = (*triple_xor_32_output_limb_1_col126
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_0_col127]: [QM31; 1] = (*triple_xor_32_output_limb_0_col127
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_1_col128]: [QM31; 1] = (*triple_xor_32_output_limb_1_col128
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_0_col129]: [QM31; 1] = (*triple_xor_32_output_limb_0_col129
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_1_col130]: [QM31; 1] = (*triple_xor_32_output_limb_1_col130
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_0_col131]: [QM31; 1] = (*triple_xor_32_output_limb_0_col131
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_1_col132]: [QM31; 1] = (*triple_xor_32_output_limb_1_col132
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_0_col133]: [QM31; 1] = (*triple_xor_32_output_limb_0_col133
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_1_col134]: [QM31; 1] = (*triple_xor_32_output_limb_1_col134
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_0_col135]: [QM31; 1] = (*triple_xor_32_output_limb_0_col135
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_1_col136]: [QM31; 1] = (*triple_xor_32_output_limb_1_col136
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_0_col137]: [QM31; 1] = (*triple_xor_32_output_limb_0_col137
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_1_col138]: [QM31; 1] = (*triple_xor_32_output_limb_1_col138
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_0_col139]: [QM31; 1] = (*triple_xor_32_output_limb_0_col139
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_1_col140]: [QM31; 1] = (*triple_xor_32_output_limb_1_col140
            .try_into()
            .unwrap())
            .unbox();
        let [low_7_ms_bits_col141]: [QM31; 1] = (*low_7_ms_bits_col141.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col142]: [QM31; 1] = (*high_14_ms_bits_col142.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col143]: [QM31; 1] = (*high_5_ms_bits_col143.try_into().unwrap())
            .unbox();
        let [new_state_0_id_col144]: [QM31; 1] = (*new_state_0_id_col144.try_into().unwrap())
            .unbox();
        let [low_7_ms_bits_col145]: [QM31; 1] = (*low_7_ms_bits_col145.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col146]: [QM31; 1] = (*high_14_ms_bits_col146.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col147]: [QM31; 1] = (*high_5_ms_bits_col147.try_into().unwrap())
            .unbox();
        let [new_state_1_id_col148]: [QM31; 1] = (*new_state_1_id_col148.try_into().unwrap())
            .unbox();
        let [low_7_ms_bits_col149]: [QM31; 1] = (*low_7_ms_bits_col149.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col150]: [QM31; 1] = (*high_14_ms_bits_col150.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col151]: [QM31; 1] = (*high_5_ms_bits_col151.try_into().unwrap())
            .unbox();
        let [new_state_2_id_col152]: [QM31; 1] = (*new_state_2_id_col152.try_into().unwrap())
            .unbox();
        let [low_7_ms_bits_col153]: [QM31; 1] = (*low_7_ms_bits_col153.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col154]: [QM31; 1] = (*high_14_ms_bits_col154.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col155]: [QM31; 1] = (*high_5_ms_bits_col155.try_into().unwrap())
            .unbox();
        let [new_state_3_id_col156]: [QM31; 1] = (*new_state_3_id_col156.try_into().unwrap())
            .unbox();
        let [low_7_ms_bits_col157]: [QM31; 1] = (*low_7_ms_bits_col157.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col158]: [QM31; 1] = (*high_14_ms_bits_col158.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col159]: [QM31; 1] = (*high_5_ms_bits_col159.try_into().unwrap())
            .unbox();
        let [new_state_4_id_col160]: [QM31; 1] = (*new_state_4_id_col160.try_into().unwrap())
            .unbox();
        let [low_7_ms_bits_col161]: [QM31; 1] = (*low_7_ms_bits_col161.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col162]: [QM31; 1] = (*high_14_ms_bits_col162.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col163]: [QM31; 1] = (*high_5_ms_bits_col163.try_into().unwrap())
            .unbox();
        let [new_state_5_id_col164]: [QM31; 1] = (*new_state_5_id_col164.try_into().unwrap())
            .unbox();
        let [low_7_ms_bits_col165]: [QM31; 1] = (*low_7_ms_bits_col165.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col166]: [QM31; 1] = (*high_14_ms_bits_col166.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col167]: [QM31; 1] = (*high_5_ms_bits_col167.try_into().unwrap())
            .unbox();
        let [new_state_6_id_col168]: [QM31; 1] = (*new_state_6_id_col168.try_into().unwrap())
            .unbox();
        let [low_7_ms_bits_col169]: [QM31; 1] = (*low_7_ms_bits_col169.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col170]: [QM31; 1] = (*high_14_ms_bits_col170.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col171]: [QM31; 1] = (*high_5_ms_bits_col171.try_into().unwrap())
            .unbox();
        let [new_state_7_id_col172]: [QM31; 1] = (*new_state_7_id_col172.try_into().unwrap())
            .unbox();
        let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        let constraint_quotient = (enabler * enabler - enabler) * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;
        let [
            decode_blake_opcode_output_tmp_53f39_42_limb_0,
            decode_blake_opcode_output_tmp_53f39_42_limb_1,
            decode_blake_opcode_output_tmp_53f39_42_limb_2,
            decode_blake_opcode_output_tmp_53f39_42_limb_6,
        ] =
            decode_blake_opcode_evaluate(
            [input_pc_col0, input_ap_col1, input_fp_col2],
            offset0_col3,
            offset1_col4,
            offset2_col5,
            dst_base_fp_col6,
            op0_base_fp_col7,
            op1_base_fp_col8,
            ap_update_add_1_col9,
            opcode_extension_col10,
            mem0_base_col11,
            op0_id_col12,
            op0_limb_0_col13,
            op0_limb_1_col14,
            op0_limb_2_col15,
            op0_limb_3_col16,
            partial_limb_msb_col17,
            mem1_base_col18,
            op1_id_col19,
            op1_limb_0_col20,
            op1_limb_1_col21,
            op1_limb_2_col22,
            op1_limb_3_col23,
            partial_limb_msb_col24,
            ap_id_col25,
            ap_limb_0_col26,
            ap_limb_1_col27,
            ap_limb_2_col28,
            ap_limb_3_col29,
            partial_limb_msb_col30,
            mem_dst_base_col31,
            low_16_bits_col32,
            high_16_bits_col33,
            low_7_ms_bits_col34,
            high_14_ms_bits_col35,
            high_5_ms_bits_col36,
            dst_id_col37,
            self.verify_instruction_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            self.range_check_7_2_5_lookup_elements,
            ref verify_instruction_sum_0,
            ref memory_address_to_id_sum_1,
            ref memory_id_to_big_sum_2,
            ref memory_address_to_id_sum_3,
            ref memory_id_to_big_sum_4,
            ref memory_address_to_id_sum_5,
            ref memory_id_to_big_sum_6,
            ref range_check_7_2_5_sum_7,
            ref memory_address_to_id_sum_8,
            ref memory_id_to_big_sum_9,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let [
            create_blake_round_input_output_tmp_53f39_143_limb_24,
            create_blake_round_input_output_tmp_53f39_143_limb_25,
            create_blake_round_input_output_tmp_53f39_143_limb_28,
            create_blake_round_input_output_tmp_53f39_143_limb_29,
        ] =
            create_blake_round_input_evaluate(
            [
                decode_blake_opcode_output_tmp_53f39_42_limb_0, low_16_bits_col32,
                high_16_bits_col33, decode_blake_opcode_output_tmp_53f39_42_limb_6,
            ],
            low_16_bits_col38,
            high_16_bits_col39,
            low_7_ms_bits_col40,
            high_14_ms_bits_col41,
            high_5_ms_bits_col42,
            state_0_id_col43,
            low_16_bits_col44,
            high_16_bits_col45,
            low_7_ms_bits_col46,
            high_14_ms_bits_col47,
            high_5_ms_bits_col48,
            state_1_id_col49,
            low_16_bits_col50,
            high_16_bits_col51,
            low_7_ms_bits_col52,
            high_14_ms_bits_col53,
            high_5_ms_bits_col54,
            state_2_id_col55,
            low_16_bits_col56,
            high_16_bits_col57,
            low_7_ms_bits_col58,
            high_14_ms_bits_col59,
            high_5_ms_bits_col60,
            state_3_id_col61,
            low_16_bits_col62,
            high_16_bits_col63,
            low_7_ms_bits_col64,
            high_14_ms_bits_col65,
            high_5_ms_bits_col66,
            state_4_id_col67,
            low_16_bits_col68,
            high_16_bits_col69,
            low_7_ms_bits_col70,
            high_14_ms_bits_col71,
            high_5_ms_bits_col72,
            state_5_id_col73,
            low_16_bits_col74,
            high_16_bits_col75,
            low_7_ms_bits_col76,
            high_14_ms_bits_col77,
            high_5_ms_bits_col78,
            state_6_id_col79,
            low_16_bits_col80,
            high_16_bits_col81,
            low_7_ms_bits_col82,
            high_14_ms_bits_col83,
            high_5_ms_bits_col84,
            state_7_id_col85,
            ms_8_bits_col86,
            ms_8_bits_col87,
            xor_col88,
            xor_col89,
            xor_col90,
            xor_col91,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            self.verify_bitwise_xor_8_lookup_elements,
            ref range_check_7_2_5_sum_10,
            ref memory_address_to_id_sum_11,
            ref memory_id_to_big_sum_12,
            ref range_check_7_2_5_sum_13,
            ref memory_address_to_id_sum_14,
            ref memory_id_to_big_sum_15,
            ref range_check_7_2_5_sum_16,
            ref memory_address_to_id_sum_17,
            ref memory_id_to_big_sum_18,
            ref range_check_7_2_5_sum_19,
            ref memory_address_to_id_sum_20,
            ref memory_id_to_big_sum_21,
            ref range_check_7_2_5_sum_22,
            ref memory_address_to_id_sum_23,
            ref memory_id_to_big_sum_24,
            ref range_check_7_2_5_sum_25,
            ref memory_address_to_id_sum_26,
            ref memory_id_to_big_sum_27,
            ref range_check_7_2_5_sum_28,
            ref memory_address_to_id_sum_29,
            ref memory_id_to_big_sum_30,
            ref range_check_7_2_5_sum_31,
            ref memory_address_to_id_sum_32,
            ref memory_id_to_big_sum_33,
            ref verify_bitwise_xor_8_sum_34,
            ref verify_bitwise_xor_8_sum_35,
            ref verify_bitwise_xor_8_sum_36,
            ref verify_bitwise_xor_8_sum_37,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        blake_round_sum_38 = self
            .blake_round_lookup_elements
            .combine_qm31(
                [
                    seq, qm31_const::<0, 0, 0, 0>(), low_16_bits_col38, high_16_bits_col39,
                    low_16_bits_col44, high_16_bits_col45, low_16_bits_col50, high_16_bits_col51,
                    low_16_bits_col56, high_16_bits_col57, low_16_bits_col62, high_16_bits_col63,
                    low_16_bits_col68, high_16_bits_col69, low_16_bits_col74, high_16_bits_col75,
                    low_16_bits_col80, high_16_bits_col81, qm31_const::<58983, 0, 0, 0>(),
                    qm31_const::<27145, 0, 0, 0>(), qm31_const::<44677, 0, 0, 0>(),
                    qm31_const::<47975, 0, 0, 0>(), qm31_const::<62322, 0, 0, 0>(),
                    qm31_const::<15470, 0, 0, 0>(), qm31_const::<62778, 0, 0, 0>(),
                    qm31_const::<42319, 0, 0, 0>(),
                    create_blake_round_input_output_tmp_53f39_143_limb_24,
                    create_blake_round_input_output_tmp_53f39_143_limb_25,
                    qm31_const::<26764, 0, 0, 0>(), qm31_const::<39685, 0, 0, 0>(),
                    create_blake_round_input_output_tmp_53f39_143_limb_28,
                    create_blake_round_input_output_tmp_53f39_143_limb_29,
                    qm31_const::<52505, 0, 0, 0>(), qm31_const::<23520, 0, 0, 0>(),
                    decode_blake_opcode_output_tmp_53f39_42_limb_1,
                ],
            );

        blake_round_sum_39 = self
            .blake_round_lookup_elements
            .combine_qm31(
                [
                    seq, qm31_const::<10, 0, 0, 0>(), blake_round_output_limb_0_col92,
                    blake_round_output_limb_1_col93, blake_round_output_limb_2_col94,
                    blake_round_output_limb_3_col95, blake_round_output_limb_4_col96,
                    blake_round_output_limb_5_col97, blake_round_output_limb_6_col98,
                    blake_round_output_limb_7_col99, blake_round_output_limb_8_col100,
                    blake_round_output_limb_9_col101, blake_round_output_limb_10_col102,
                    blake_round_output_limb_11_col103, blake_round_output_limb_12_col104,
                    blake_round_output_limb_13_col105, blake_round_output_limb_14_col106,
                    blake_round_output_limb_15_col107, blake_round_output_limb_16_col108,
                    blake_round_output_limb_17_col109, blake_round_output_limb_18_col110,
                    blake_round_output_limb_19_col111, blake_round_output_limb_20_col112,
                    blake_round_output_limb_21_col113, blake_round_output_limb_22_col114,
                    blake_round_output_limb_23_col115, blake_round_output_limb_24_col116,
                    blake_round_output_limb_25_col117, blake_round_output_limb_26_col118,
                    blake_round_output_limb_27_col119, blake_round_output_limb_28_col120,
                    blake_round_output_limb_29_col121, blake_round_output_limb_30_col122,
                    blake_round_output_limb_31_col123, blake_round_output_limb_32_col124,
                ],
            );
        create_blake_output_evaluate(
            [
                low_16_bits_col38, high_16_bits_col39, low_16_bits_col44, high_16_bits_col45,
                low_16_bits_col50, high_16_bits_col51, low_16_bits_col56, high_16_bits_col57,
                low_16_bits_col62, high_16_bits_col63, low_16_bits_col68, high_16_bits_col69,
                low_16_bits_col74, high_16_bits_col75, low_16_bits_col80, high_16_bits_col81,
                blake_round_output_limb_0_col92, blake_round_output_limb_1_col93,
                blake_round_output_limb_2_col94, blake_round_output_limb_3_col95,
                blake_round_output_limb_4_col96, blake_round_output_limb_5_col97,
                blake_round_output_limb_6_col98, blake_round_output_limb_7_col99,
                blake_round_output_limb_8_col100, blake_round_output_limb_9_col101,
                blake_round_output_limb_10_col102, blake_round_output_limb_11_col103,
                blake_round_output_limb_12_col104, blake_round_output_limb_13_col105,
                blake_round_output_limb_14_col106, blake_round_output_limb_15_col107,
                blake_round_output_limb_16_col108, blake_round_output_limb_17_col109,
                blake_round_output_limb_18_col110, blake_round_output_limb_19_col111,
                blake_round_output_limb_20_col112, blake_round_output_limb_21_col113,
                blake_round_output_limb_22_col114, blake_round_output_limb_23_col115,
                blake_round_output_limb_24_col116, blake_round_output_limb_25_col117,
                blake_round_output_limb_26_col118, blake_round_output_limb_27_col119,
                blake_round_output_limb_28_col120, blake_round_output_limb_29_col121,
                blake_round_output_limb_30_col122, blake_round_output_limb_31_col123,
            ],
            triple_xor_32_output_limb_0_col125,
            triple_xor_32_output_limb_1_col126,
            triple_xor_32_output_limb_0_col127,
            triple_xor_32_output_limb_1_col128,
            triple_xor_32_output_limb_0_col129,
            triple_xor_32_output_limb_1_col130,
            triple_xor_32_output_limb_0_col131,
            triple_xor_32_output_limb_1_col132,
            triple_xor_32_output_limb_0_col133,
            triple_xor_32_output_limb_1_col134,
            triple_xor_32_output_limb_0_col135,
            triple_xor_32_output_limb_1_col136,
            triple_xor_32_output_limb_0_col137,
            triple_xor_32_output_limb_1_col138,
            triple_xor_32_output_limb_0_col139,
            triple_xor_32_output_limb_1_col140,
            self.triple_xor_32_lookup_elements,
            ref triple_xor_32_sum_40,
            ref triple_xor_32_sum_41,
            ref triple_xor_32_sum_42,
            ref triple_xor_32_sum_43,
            ref triple_xor_32_sum_44,
            ref triple_xor_32_sum_45,
            ref triple_xor_32_sum_46,
            ref triple_xor_32_sum_47,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        verify_u_32_evaluate(
            [
                decode_blake_opcode_output_tmp_53f39_42_limb_2, triple_xor_32_output_limb_0_col125,
                triple_xor_32_output_limb_1_col126,
            ],
            low_7_ms_bits_col141,
            high_14_ms_bits_col142,
            high_5_ms_bits_col143,
            new_state_0_id_col144,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_48,
            ref memory_address_to_id_sum_49,
            ref memory_id_to_big_sum_50,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        verify_u_32_evaluate(
            [
                (decode_blake_opcode_output_tmp_53f39_42_limb_2 + qm31_const::<1, 0, 0, 0>()),
                triple_xor_32_output_limb_0_col127, triple_xor_32_output_limb_1_col128,
            ],
            low_7_ms_bits_col145,
            high_14_ms_bits_col146,
            high_5_ms_bits_col147,
            new_state_1_id_col148,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_51,
            ref memory_address_to_id_sum_52,
            ref memory_id_to_big_sum_53,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        verify_u_32_evaluate(
            [
                (decode_blake_opcode_output_tmp_53f39_42_limb_2 + qm31_const::<2, 0, 0, 0>()),
                triple_xor_32_output_limb_0_col129, triple_xor_32_output_limb_1_col130,
            ],
            low_7_ms_bits_col149,
            high_14_ms_bits_col150,
            high_5_ms_bits_col151,
            new_state_2_id_col152,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_54,
            ref memory_address_to_id_sum_55,
            ref memory_id_to_big_sum_56,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        verify_u_32_evaluate(
            [
                (decode_blake_opcode_output_tmp_53f39_42_limb_2 + qm31_const::<3, 0, 0, 0>()),
                triple_xor_32_output_limb_0_col131, triple_xor_32_output_limb_1_col132,
            ],
            low_7_ms_bits_col153,
            high_14_ms_bits_col154,
            high_5_ms_bits_col155,
            new_state_3_id_col156,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_57,
            ref memory_address_to_id_sum_58,
            ref memory_id_to_big_sum_59,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        verify_u_32_evaluate(
            [
                (decode_blake_opcode_output_tmp_53f39_42_limb_2 + qm31_const::<4, 0, 0, 0>()),
                triple_xor_32_output_limb_0_col133, triple_xor_32_output_limb_1_col134,
            ],
            low_7_ms_bits_col157,
            high_14_ms_bits_col158,
            high_5_ms_bits_col159,
            new_state_4_id_col160,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_60,
            ref memory_address_to_id_sum_61,
            ref memory_id_to_big_sum_62,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        verify_u_32_evaluate(
            [
                (decode_blake_opcode_output_tmp_53f39_42_limb_2 + qm31_const::<5, 0, 0, 0>()),
                triple_xor_32_output_limb_0_col135, triple_xor_32_output_limb_1_col136,
            ],
            low_7_ms_bits_col161,
            high_14_ms_bits_col162,
            high_5_ms_bits_col163,
            new_state_5_id_col164,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_63,
            ref memory_address_to_id_sum_64,
            ref memory_id_to_big_sum_65,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        verify_u_32_evaluate(
            [
                (decode_blake_opcode_output_tmp_53f39_42_limb_2 + qm31_const::<6, 0, 0, 0>()),
                triple_xor_32_output_limb_0_col137, triple_xor_32_output_limb_1_col138,
            ],
            low_7_ms_bits_col165,
            high_14_ms_bits_col166,
            high_5_ms_bits_col167,
            new_state_6_id_col168,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_66,
            ref memory_address_to_id_sum_67,
            ref memory_id_to_big_sum_68,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        verify_u_32_evaluate(
            [
                (decode_blake_opcode_output_tmp_53f39_42_limb_2 + qm31_const::<7, 0, 0, 0>()),
                triple_xor_32_output_limb_0_col139, triple_xor_32_output_limb_1_col140,
            ],
            low_7_ms_bits_col169,
            high_14_ms_bits_col170,
            high_5_ms_bits_col171,
            new_state_7_id_col172,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_69,
            ref memory_address_to_id_sum_70,
            ref memory_id_to_big_sum_71,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        opcodes_sum_72 = self
            .opcodes_lookup_elements
            .combine_qm31([input_pc_col0, input_ap_col1, input_fp_col2]);

        opcodes_sum_73 = self
            .opcodes_lookup_elements
            .combine_qm31(
                [
                    (input_pc_col0 + qm31_const::<1, 0, 0, 0>()),
                    (input_ap_col1 + ap_update_add_1_col9), input_fp_col2,
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
            range_check_7_2_5_sum_7,
            memory_address_to_id_sum_8,
            memory_id_to_big_sum_9,
            range_check_7_2_5_sum_10,
            memory_address_to_id_sum_11,
            memory_id_to_big_sum_12,
            range_check_7_2_5_sum_13,
            memory_address_to_id_sum_14,
            memory_id_to_big_sum_15,
            range_check_7_2_5_sum_16,
            memory_address_to_id_sum_17,
            memory_id_to_big_sum_18,
            range_check_7_2_5_sum_19,
            memory_address_to_id_sum_20,
            memory_id_to_big_sum_21,
            range_check_7_2_5_sum_22,
            memory_address_to_id_sum_23,
            memory_id_to_big_sum_24,
            range_check_7_2_5_sum_25,
            memory_address_to_id_sum_26,
            memory_id_to_big_sum_27,
            range_check_7_2_5_sum_28,
            memory_address_to_id_sum_29,
            memory_id_to_big_sum_30,
            range_check_7_2_5_sum_31,
            memory_address_to_id_sum_32,
            memory_id_to_big_sum_33,
            verify_bitwise_xor_8_sum_34,
            verify_bitwise_xor_8_sum_35,
            verify_bitwise_xor_8_sum_36,
            verify_bitwise_xor_8_sum_37,
            blake_round_sum_38,
            blake_round_sum_39,
            triple_xor_32_sum_40,
            triple_xor_32_sum_41,
            triple_xor_32_sum_42,
            triple_xor_32_sum_43,
            triple_xor_32_sum_44,
            triple_xor_32_sum_45,
            triple_xor_32_sum_46,
            triple_xor_32_sum_47,
            range_check_7_2_5_sum_48,
            memory_address_to_id_sum_49,
            memory_id_to_big_sum_50,
            range_check_7_2_5_sum_51,
            memory_address_to_id_sum_52,
            memory_id_to_big_sum_53,
            range_check_7_2_5_sum_54,
            memory_address_to_id_sum_55,
            memory_id_to_big_sum_56,
            range_check_7_2_5_sum_57,
            memory_address_to_id_sum_58,
            memory_id_to_big_sum_59,
            range_check_7_2_5_sum_60,
            memory_address_to_id_sum_61,
            memory_id_to_big_sum_62,
            range_check_7_2_5_sum_63,
            memory_address_to_id_sum_64,
            memory_id_to_big_sum_65,
            range_check_7_2_5_sum_66,
            memory_address_to_id_sum_67,
            memory_id_to_big_sum_68,
            range_check_7_2_5_sum_69,
            memory_address_to_id_sum_70,
            memory_id_to_big_sum_71,
            opcodes_sum_72,
            opcodes_sum_73,
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
    range_check_7_2_5_sum_7: QM31,
    memory_address_to_id_sum_8: QM31,
    memory_id_to_big_sum_9: QM31,
    range_check_7_2_5_sum_10: QM31,
    memory_address_to_id_sum_11: QM31,
    memory_id_to_big_sum_12: QM31,
    range_check_7_2_5_sum_13: QM31,
    memory_address_to_id_sum_14: QM31,
    memory_id_to_big_sum_15: QM31,
    range_check_7_2_5_sum_16: QM31,
    memory_address_to_id_sum_17: QM31,
    memory_id_to_big_sum_18: QM31,
    range_check_7_2_5_sum_19: QM31,
    memory_address_to_id_sum_20: QM31,
    memory_id_to_big_sum_21: QM31,
    range_check_7_2_5_sum_22: QM31,
    memory_address_to_id_sum_23: QM31,
    memory_id_to_big_sum_24: QM31,
    range_check_7_2_5_sum_25: QM31,
    memory_address_to_id_sum_26: QM31,
    memory_id_to_big_sum_27: QM31,
    range_check_7_2_5_sum_28: QM31,
    memory_address_to_id_sum_29: QM31,
    memory_id_to_big_sum_30: QM31,
    range_check_7_2_5_sum_31: QM31,
    memory_address_to_id_sum_32: QM31,
    memory_id_to_big_sum_33: QM31,
    verify_bitwise_xor_8_sum_34: QM31,
    verify_bitwise_xor_8_sum_35: QM31,
    verify_bitwise_xor_8_sum_36: QM31,
    verify_bitwise_xor_8_sum_37: QM31,
    blake_round_sum_38: QM31,
    blake_round_sum_39: QM31,
    triple_xor_32_sum_40: QM31,
    triple_xor_32_sum_41: QM31,
    triple_xor_32_sum_42: QM31,
    triple_xor_32_sum_43: QM31,
    triple_xor_32_sum_44: QM31,
    triple_xor_32_sum_45: QM31,
    triple_xor_32_sum_46: QM31,
    triple_xor_32_sum_47: QM31,
    range_check_7_2_5_sum_48: QM31,
    memory_address_to_id_sum_49: QM31,
    memory_id_to_big_sum_50: QM31,
    range_check_7_2_5_sum_51: QM31,
    memory_address_to_id_sum_52: QM31,
    memory_id_to_big_sum_53: QM31,
    range_check_7_2_5_sum_54: QM31,
    memory_address_to_id_sum_55: QM31,
    memory_id_to_big_sum_56: QM31,
    range_check_7_2_5_sum_57: QM31,
    memory_address_to_id_sum_58: QM31,
    memory_id_to_big_sum_59: QM31,
    range_check_7_2_5_sum_60: QM31,
    memory_address_to_id_sum_61: QM31,
    memory_id_to_big_sum_62: QM31,
    range_check_7_2_5_sum_63: QM31,
    memory_address_to_id_sum_64: QM31,
    memory_id_to_big_sum_65: QM31,
    range_check_7_2_5_sum_66: QM31,
    memory_address_to_id_sum_67: QM31,
    memory_id_to_big_sum_68: QM31,
    range_check_7_2_5_sum_69: QM31,
    memory_address_to_id_sum_70: QM31,
    memory_id_to_big_sum_71: QM31,
    opcodes_sum_72: QM31,
    opcodes_sum_73: QM31,
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
        trace_2_col24,
        trace_2_col25,
        trace_2_col26,
        trace_2_col27,
        trace_2_col28,
        trace_2_col29,
        trace_2_col30,
        trace_2_col31,
        trace_2_col32,
        trace_2_col33,
        trace_2_col34,
        trace_2_col35,
        trace_2_col36,
        trace_2_col37,
        trace_2_col38,
        trace_2_col39,
        trace_2_col40,
        trace_2_col41,
        trace_2_col42,
        trace_2_col43,
        trace_2_col44,
        trace_2_col45,
        trace_2_col46,
        trace_2_col47,
        trace_2_col48,
        trace_2_col49,
        trace_2_col50,
        trace_2_col51,
        trace_2_col52,
        trace_2_col53,
        trace_2_col54,
        trace_2_col55,
        trace_2_col56,
        trace_2_col57,
        trace_2_col58,
        trace_2_col59,
        trace_2_col60,
        trace_2_col61,
        trace_2_col62,
        trace_2_col63,
        trace_2_col64,
        trace_2_col65,
        trace_2_col66,
        trace_2_col67,
        trace_2_col68,
        trace_2_col69,
        trace_2_col70,
        trace_2_col71,
        trace_2_col72,
        trace_2_col73,
        trace_2_col74,
        trace_2_col75,
        trace_2_col76,
        trace_2_col77,
        trace_2_col78,
        trace_2_col79,
        trace_2_col80,
        trace_2_col81,
        trace_2_col82,
        trace_2_col83,
        trace_2_col84,
        trace_2_col85,
        trace_2_col86,
        trace_2_col87,
        trace_2_col88,
        trace_2_col89,
        trace_2_col90,
        trace_2_col91,
        trace_2_col92,
        trace_2_col93,
        trace_2_col94,
        trace_2_col95,
        trace_2_col96,
        trace_2_col97,
        trace_2_col98,
        trace_2_col99,
        trace_2_col100,
        trace_2_col101,
        trace_2_col102,
        trace_2_col103,
        trace_2_col104,
        trace_2_col105,
        trace_2_col106,
        trace_2_col107,
        trace_2_col108,
        trace_2_col109,
        trace_2_col110,
        trace_2_col111,
        trace_2_col112,
        trace_2_col113,
        trace_2_col114,
        trace_2_col115,
        trace_2_col116,
        trace_2_col117,
        trace_2_col118,
        trace_2_col119,
        trace_2_col120,
        trace_2_col121,
        trace_2_col122,
        trace_2_col123,
        trace_2_col124,
        trace_2_col125,
        trace_2_col126,
        trace_2_col127,
        trace_2_col128,
        trace_2_col129,
        trace_2_col130,
        trace_2_col131,
        trace_2_col132,
        trace_2_col133,
        trace_2_col134,
        trace_2_col135,
        trace_2_col136,
        trace_2_col137,
        trace_2_col138,
        trace_2_col139,
        trace_2_col140,
        trace_2_col141,
        trace_2_col142,
        trace_2_col143,
        trace_2_col144,
        trace_2_col145,
        trace_2_col146,
        trace_2_col147,
    ]: [Span<QM31>; 148] =
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
    let [trace_2_col20]: [QM31; 1] = (*trace_2_col20.try_into().unwrap()).unbox();
    let [trace_2_col21]: [QM31; 1] = (*trace_2_col21.try_into().unwrap()).unbox();
    let [trace_2_col22]: [QM31; 1] = (*trace_2_col22.try_into().unwrap()).unbox();
    let [trace_2_col23]: [QM31; 1] = (*trace_2_col23.try_into().unwrap()).unbox();
    let [trace_2_col24]: [QM31; 1] = (*trace_2_col24.try_into().unwrap()).unbox();
    let [trace_2_col25]: [QM31; 1] = (*trace_2_col25.try_into().unwrap()).unbox();
    let [trace_2_col26]: [QM31; 1] = (*trace_2_col26.try_into().unwrap()).unbox();
    let [trace_2_col27]: [QM31; 1] = (*trace_2_col27.try_into().unwrap()).unbox();
    let [trace_2_col28]: [QM31; 1] = (*trace_2_col28.try_into().unwrap()).unbox();
    let [trace_2_col29]: [QM31; 1] = (*trace_2_col29.try_into().unwrap()).unbox();
    let [trace_2_col30]: [QM31; 1] = (*trace_2_col30.try_into().unwrap()).unbox();
    let [trace_2_col31]: [QM31; 1] = (*trace_2_col31.try_into().unwrap()).unbox();
    let [trace_2_col32]: [QM31; 1] = (*trace_2_col32.try_into().unwrap()).unbox();
    let [trace_2_col33]: [QM31; 1] = (*trace_2_col33.try_into().unwrap()).unbox();
    let [trace_2_col34]: [QM31; 1] = (*trace_2_col34.try_into().unwrap()).unbox();
    let [trace_2_col35]: [QM31; 1] = (*trace_2_col35.try_into().unwrap()).unbox();
    let [trace_2_col36]: [QM31; 1] = (*trace_2_col36.try_into().unwrap()).unbox();
    let [trace_2_col37]: [QM31; 1] = (*trace_2_col37.try_into().unwrap()).unbox();
    let [trace_2_col38]: [QM31; 1] = (*trace_2_col38.try_into().unwrap()).unbox();
    let [trace_2_col39]: [QM31; 1] = (*trace_2_col39.try_into().unwrap()).unbox();
    let [trace_2_col40]: [QM31; 1] = (*trace_2_col40.try_into().unwrap()).unbox();
    let [trace_2_col41]: [QM31; 1] = (*trace_2_col41.try_into().unwrap()).unbox();
    let [trace_2_col42]: [QM31; 1] = (*trace_2_col42.try_into().unwrap()).unbox();
    let [trace_2_col43]: [QM31; 1] = (*trace_2_col43.try_into().unwrap()).unbox();
    let [trace_2_col44]: [QM31; 1] = (*trace_2_col44.try_into().unwrap()).unbox();
    let [trace_2_col45]: [QM31; 1] = (*trace_2_col45.try_into().unwrap()).unbox();
    let [trace_2_col46]: [QM31; 1] = (*trace_2_col46.try_into().unwrap()).unbox();
    let [trace_2_col47]: [QM31; 1] = (*trace_2_col47.try_into().unwrap()).unbox();
    let [trace_2_col48]: [QM31; 1] = (*trace_2_col48.try_into().unwrap()).unbox();
    let [trace_2_col49]: [QM31; 1] = (*trace_2_col49.try_into().unwrap()).unbox();
    let [trace_2_col50]: [QM31; 1] = (*trace_2_col50.try_into().unwrap()).unbox();
    let [trace_2_col51]: [QM31; 1] = (*trace_2_col51.try_into().unwrap()).unbox();
    let [trace_2_col52]: [QM31; 1] = (*trace_2_col52.try_into().unwrap()).unbox();
    let [trace_2_col53]: [QM31; 1] = (*trace_2_col53.try_into().unwrap()).unbox();
    let [trace_2_col54]: [QM31; 1] = (*trace_2_col54.try_into().unwrap()).unbox();
    let [trace_2_col55]: [QM31; 1] = (*trace_2_col55.try_into().unwrap()).unbox();
    let [trace_2_col56]: [QM31; 1] = (*trace_2_col56.try_into().unwrap()).unbox();
    let [trace_2_col57]: [QM31; 1] = (*trace_2_col57.try_into().unwrap()).unbox();
    let [trace_2_col58]: [QM31; 1] = (*trace_2_col58.try_into().unwrap()).unbox();
    let [trace_2_col59]: [QM31; 1] = (*trace_2_col59.try_into().unwrap()).unbox();
    let [trace_2_col60]: [QM31; 1] = (*trace_2_col60.try_into().unwrap()).unbox();
    let [trace_2_col61]: [QM31; 1] = (*trace_2_col61.try_into().unwrap()).unbox();
    let [trace_2_col62]: [QM31; 1] = (*trace_2_col62.try_into().unwrap()).unbox();
    let [trace_2_col63]: [QM31; 1] = (*trace_2_col63.try_into().unwrap()).unbox();
    let [trace_2_col64]: [QM31; 1] = (*trace_2_col64.try_into().unwrap()).unbox();
    let [trace_2_col65]: [QM31; 1] = (*trace_2_col65.try_into().unwrap()).unbox();
    let [trace_2_col66]: [QM31; 1] = (*trace_2_col66.try_into().unwrap()).unbox();
    let [trace_2_col67]: [QM31; 1] = (*trace_2_col67.try_into().unwrap()).unbox();
    let [trace_2_col68]: [QM31; 1] = (*trace_2_col68.try_into().unwrap()).unbox();
    let [trace_2_col69]: [QM31; 1] = (*trace_2_col69.try_into().unwrap()).unbox();
    let [trace_2_col70]: [QM31; 1] = (*trace_2_col70.try_into().unwrap()).unbox();
    let [trace_2_col71]: [QM31; 1] = (*trace_2_col71.try_into().unwrap()).unbox();
    let [trace_2_col72]: [QM31; 1] = (*trace_2_col72.try_into().unwrap()).unbox();
    let [trace_2_col73]: [QM31; 1] = (*trace_2_col73.try_into().unwrap()).unbox();
    let [trace_2_col74]: [QM31; 1] = (*trace_2_col74.try_into().unwrap()).unbox();
    let [trace_2_col75]: [QM31; 1] = (*trace_2_col75.try_into().unwrap()).unbox();
    let [trace_2_col76]: [QM31; 1] = (*trace_2_col76.try_into().unwrap()).unbox();
    let [trace_2_col77]: [QM31; 1] = (*trace_2_col77.try_into().unwrap()).unbox();
    let [trace_2_col78]: [QM31; 1] = (*trace_2_col78.try_into().unwrap()).unbox();
    let [trace_2_col79]: [QM31; 1] = (*trace_2_col79.try_into().unwrap()).unbox();
    let [trace_2_col80]: [QM31; 1] = (*trace_2_col80.try_into().unwrap()).unbox();
    let [trace_2_col81]: [QM31; 1] = (*trace_2_col81.try_into().unwrap()).unbox();
    let [trace_2_col82]: [QM31; 1] = (*trace_2_col82.try_into().unwrap()).unbox();
    let [trace_2_col83]: [QM31; 1] = (*trace_2_col83.try_into().unwrap()).unbox();
    let [trace_2_col84]: [QM31; 1] = (*trace_2_col84.try_into().unwrap()).unbox();
    let [trace_2_col85]: [QM31; 1] = (*trace_2_col85.try_into().unwrap()).unbox();
    let [trace_2_col86]: [QM31; 1] = (*trace_2_col86.try_into().unwrap()).unbox();
    let [trace_2_col87]: [QM31; 1] = (*trace_2_col87.try_into().unwrap()).unbox();
    let [trace_2_col88]: [QM31; 1] = (*trace_2_col88.try_into().unwrap()).unbox();
    let [trace_2_col89]: [QM31; 1] = (*trace_2_col89.try_into().unwrap()).unbox();
    let [trace_2_col90]: [QM31; 1] = (*trace_2_col90.try_into().unwrap()).unbox();
    let [trace_2_col91]: [QM31; 1] = (*trace_2_col91.try_into().unwrap()).unbox();
    let [trace_2_col92]: [QM31; 1] = (*trace_2_col92.try_into().unwrap()).unbox();
    let [trace_2_col93]: [QM31; 1] = (*trace_2_col93.try_into().unwrap()).unbox();
    let [trace_2_col94]: [QM31; 1] = (*trace_2_col94.try_into().unwrap()).unbox();
    let [trace_2_col95]: [QM31; 1] = (*trace_2_col95.try_into().unwrap()).unbox();
    let [trace_2_col96]: [QM31; 1] = (*trace_2_col96.try_into().unwrap()).unbox();
    let [trace_2_col97]: [QM31; 1] = (*trace_2_col97.try_into().unwrap()).unbox();
    let [trace_2_col98]: [QM31; 1] = (*trace_2_col98.try_into().unwrap()).unbox();
    let [trace_2_col99]: [QM31; 1] = (*trace_2_col99.try_into().unwrap()).unbox();
    let [trace_2_col100]: [QM31; 1] = (*trace_2_col100.try_into().unwrap()).unbox();
    let [trace_2_col101]: [QM31; 1] = (*trace_2_col101.try_into().unwrap()).unbox();
    let [trace_2_col102]: [QM31; 1] = (*trace_2_col102.try_into().unwrap()).unbox();
    let [trace_2_col103]: [QM31; 1] = (*trace_2_col103.try_into().unwrap()).unbox();
    let [trace_2_col104]: [QM31; 1] = (*trace_2_col104.try_into().unwrap()).unbox();
    let [trace_2_col105]: [QM31; 1] = (*trace_2_col105.try_into().unwrap()).unbox();
    let [trace_2_col106]: [QM31; 1] = (*trace_2_col106.try_into().unwrap()).unbox();
    let [trace_2_col107]: [QM31; 1] = (*trace_2_col107.try_into().unwrap()).unbox();
    let [trace_2_col108]: [QM31; 1] = (*trace_2_col108.try_into().unwrap()).unbox();
    let [trace_2_col109]: [QM31; 1] = (*trace_2_col109.try_into().unwrap()).unbox();
    let [trace_2_col110]: [QM31; 1] = (*trace_2_col110.try_into().unwrap()).unbox();
    let [trace_2_col111]: [QM31; 1] = (*trace_2_col111.try_into().unwrap()).unbox();
    let [trace_2_col112]: [QM31; 1] = (*trace_2_col112.try_into().unwrap()).unbox();
    let [trace_2_col113]: [QM31; 1] = (*trace_2_col113.try_into().unwrap()).unbox();
    let [trace_2_col114]: [QM31; 1] = (*trace_2_col114.try_into().unwrap()).unbox();
    let [trace_2_col115]: [QM31; 1] = (*trace_2_col115.try_into().unwrap()).unbox();
    let [trace_2_col116]: [QM31; 1] = (*trace_2_col116.try_into().unwrap()).unbox();
    let [trace_2_col117]: [QM31; 1] = (*trace_2_col117.try_into().unwrap()).unbox();
    let [trace_2_col118]: [QM31; 1] = (*trace_2_col118.try_into().unwrap()).unbox();
    let [trace_2_col119]: [QM31; 1] = (*trace_2_col119.try_into().unwrap()).unbox();
    let [trace_2_col120]: [QM31; 1] = (*trace_2_col120.try_into().unwrap()).unbox();
    let [trace_2_col121]: [QM31; 1] = (*trace_2_col121.try_into().unwrap()).unbox();
    let [trace_2_col122]: [QM31; 1] = (*trace_2_col122.try_into().unwrap()).unbox();
    let [trace_2_col123]: [QM31; 1] = (*trace_2_col123.try_into().unwrap()).unbox();
    let [trace_2_col124]: [QM31; 1] = (*trace_2_col124.try_into().unwrap()).unbox();
    let [trace_2_col125]: [QM31; 1] = (*trace_2_col125.try_into().unwrap()).unbox();
    let [trace_2_col126]: [QM31; 1] = (*trace_2_col126.try_into().unwrap()).unbox();
    let [trace_2_col127]: [QM31; 1] = (*trace_2_col127.try_into().unwrap()).unbox();
    let [trace_2_col128]: [QM31; 1] = (*trace_2_col128.try_into().unwrap()).unbox();
    let [trace_2_col129]: [QM31; 1] = (*trace_2_col129.try_into().unwrap()).unbox();
    let [trace_2_col130]: [QM31; 1] = (*trace_2_col130.try_into().unwrap()).unbox();
    let [trace_2_col131]: [QM31; 1] = (*trace_2_col131.try_into().unwrap()).unbox();
    let [trace_2_col132]: [QM31; 1] = (*trace_2_col132.try_into().unwrap()).unbox();
    let [trace_2_col133]: [QM31; 1] = (*trace_2_col133.try_into().unwrap()).unbox();
    let [trace_2_col134]: [QM31; 1] = (*trace_2_col134.try_into().unwrap()).unbox();
    let [trace_2_col135]: [QM31; 1] = (*trace_2_col135.try_into().unwrap()).unbox();
    let [trace_2_col136]: [QM31; 1] = (*trace_2_col136.try_into().unwrap()).unbox();
    let [trace_2_col137]: [QM31; 1] = (*trace_2_col137.try_into().unwrap()).unbox();
    let [trace_2_col138]: [QM31; 1] = (*trace_2_col138.try_into().unwrap()).unbox();
    let [trace_2_col139]: [QM31; 1] = (*trace_2_col139.try_into().unwrap()).unbox();
    let [trace_2_col140]: [QM31; 1] = (*trace_2_col140.try_into().unwrap()).unbox();
    let [trace_2_col141]: [QM31; 1] = (*trace_2_col141.try_into().unwrap()).unbox();
    let [trace_2_col142]: [QM31; 1] = (*trace_2_col142.try_into().unwrap()).unbox();
    let [trace_2_col143]: [QM31; 1] = (*trace_2_col143.try_into().unwrap()).unbox();
    let [trace_2_col144_neg1, trace_2_col144]: [QM31; 2] = (*trace_2_col144.try_into().unwrap())
        .unbox();
    let [trace_2_col145_neg1, trace_2_col145]: [QM31; 2] = (*trace_2_col145.try_into().unwrap())
        .unbox();
    let [trace_2_col146_neg1, trace_2_col146]: [QM31; 2] = (*trace_2_col146.try_into().unwrap())
        .unbox();
    let [trace_2_col147_neg1, trace_2_col147]: [QM31; 2] = (*trace_2_col147.try_into().unwrap())
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
        * range_check_7_2_5_sum_7)
        - memory_id_to_big_sum_6
        - range_check_7_2_5_sum_7)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
        ))
        * memory_address_to_id_sum_8
        * memory_id_to_big_sum_9)
        - memory_address_to_id_sum_8
        - memory_id_to_big_sum_9)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
        ))
        * range_check_7_2_5_sum_10
        * memory_address_to_id_sum_11)
        - range_check_7_2_5_sum_10
        - memory_address_to_id_sum_11)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23],
        ))
        * memory_id_to_big_sum_12
        * range_check_7_2_5_sum_13)
        - memory_id_to_big_sum_12
        - range_check_7_2_5_sum_13)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col28, trace_2_col29, trace_2_col30, trace_2_col31],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27],
        ))
        * memory_address_to_id_sum_14
        * memory_id_to_big_sum_15)
        - memory_address_to_id_sum_14
        - memory_id_to_big_sum_15)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col32, trace_2_col33, trace_2_col34, trace_2_col35],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col28, trace_2_col29, trace_2_col30, trace_2_col31],
        ))
        * range_check_7_2_5_sum_16
        * memory_address_to_id_sum_17)
        - range_check_7_2_5_sum_16
        - memory_address_to_id_sum_17)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col36, trace_2_col37, trace_2_col38, trace_2_col39],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col32, trace_2_col33, trace_2_col34, trace_2_col35],
        ))
        * memory_id_to_big_sum_18
        * range_check_7_2_5_sum_19)
        - memory_id_to_big_sum_18
        - range_check_7_2_5_sum_19)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col40, trace_2_col41, trace_2_col42, trace_2_col43],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col36, trace_2_col37, trace_2_col38, trace_2_col39],
        ))
        * memory_address_to_id_sum_20
        * memory_id_to_big_sum_21)
        - memory_address_to_id_sum_20
        - memory_id_to_big_sum_21)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col44, trace_2_col45, trace_2_col46, trace_2_col47],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col40, trace_2_col41, trace_2_col42, trace_2_col43],
        ))
        * range_check_7_2_5_sum_22
        * memory_address_to_id_sum_23)
        - range_check_7_2_5_sum_22
        - memory_address_to_id_sum_23)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col48, trace_2_col49, trace_2_col50, trace_2_col51],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col44, trace_2_col45, trace_2_col46, trace_2_col47],
        ))
        * memory_id_to_big_sum_24
        * range_check_7_2_5_sum_25)
        - memory_id_to_big_sum_24
        - range_check_7_2_5_sum_25)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col52, trace_2_col53, trace_2_col54, trace_2_col55],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col48, trace_2_col49, trace_2_col50, trace_2_col51],
        ))
        * memory_address_to_id_sum_26
        * memory_id_to_big_sum_27)
        - memory_address_to_id_sum_26
        - memory_id_to_big_sum_27)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col56, trace_2_col57, trace_2_col58, trace_2_col59],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col52, trace_2_col53, trace_2_col54, trace_2_col55],
        ))
        * range_check_7_2_5_sum_28
        * memory_address_to_id_sum_29)
        - range_check_7_2_5_sum_28
        - memory_address_to_id_sum_29)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col60, trace_2_col61, trace_2_col62, trace_2_col63],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col56, trace_2_col57, trace_2_col58, trace_2_col59],
        ))
        * memory_id_to_big_sum_30
        * range_check_7_2_5_sum_31)
        - memory_id_to_big_sum_30
        - range_check_7_2_5_sum_31)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col64, trace_2_col65, trace_2_col66, trace_2_col67],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col60, trace_2_col61, trace_2_col62, trace_2_col63],
        ))
        * memory_address_to_id_sum_32
        * memory_id_to_big_sum_33)
        - memory_address_to_id_sum_32
        - memory_id_to_big_sum_33)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col68, trace_2_col69, trace_2_col70, trace_2_col71],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col64, trace_2_col65, trace_2_col66, trace_2_col67],
        ))
        * verify_bitwise_xor_8_sum_34
        * verify_bitwise_xor_8_sum_35)
        - verify_bitwise_xor_8_sum_34
        - verify_bitwise_xor_8_sum_35)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col72, trace_2_col73, trace_2_col74, trace_2_col75],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col68, trace_2_col69, trace_2_col70, trace_2_col71],
        ))
        * verify_bitwise_xor_8_sum_36
        * verify_bitwise_xor_8_sum_37)
        - verify_bitwise_xor_8_sum_36
        - verify_bitwise_xor_8_sum_37)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col76, trace_2_col77, trace_2_col78, trace_2_col79],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col72, trace_2_col73, trace_2_col74, trace_2_col75],
        ))
        * blake_round_sum_38
        * blake_round_sum_39)
        - blake_round_sum_38
        + blake_round_sum_39)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col80, trace_2_col81, trace_2_col82, trace_2_col83],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col76, trace_2_col77, trace_2_col78, trace_2_col79],
        ))
        * triple_xor_32_sum_40
        * triple_xor_32_sum_41)
        - triple_xor_32_sum_40
        - triple_xor_32_sum_41)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col84, trace_2_col85, trace_2_col86, trace_2_col87],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col80, trace_2_col81, trace_2_col82, trace_2_col83],
        ))
        * triple_xor_32_sum_42
        * triple_xor_32_sum_43)
        - triple_xor_32_sum_42
        - triple_xor_32_sum_43)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col88, trace_2_col89, trace_2_col90, trace_2_col91],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col84, trace_2_col85, trace_2_col86, trace_2_col87],
        ))
        * triple_xor_32_sum_44
        * triple_xor_32_sum_45)
        - triple_xor_32_sum_44
        - triple_xor_32_sum_45)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col92, trace_2_col93, trace_2_col94, trace_2_col95],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col88, trace_2_col89, trace_2_col90, trace_2_col91],
        ))
        * triple_xor_32_sum_46
        * triple_xor_32_sum_47)
        - triple_xor_32_sum_46
        - triple_xor_32_sum_47)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col96, trace_2_col97, trace_2_col98, trace_2_col99],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col92, trace_2_col93, trace_2_col94, trace_2_col95],
        ))
        * range_check_7_2_5_sum_48
        * memory_address_to_id_sum_49)
        - range_check_7_2_5_sum_48
        - memory_address_to_id_sum_49)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col100, trace_2_col101, trace_2_col102, trace_2_col103],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col96, trace_2_col97, trace_2_col98, trace_2_col99],
        ))
        * memory_id_to_big_sum_50
        * range_check_7_2_5_sum_51)
        - memory_id_to_big_sum_50
        - range_check_7_2_5_sum_51)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col104, trace_2_col105, trace_2_col106, trace_2_col107],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col100, trace_2_col101, trace_2_col102, trace_2_col103],
        ))
        * memory_address_to_id_sum_52
        * memory_id_to_big_sum_53)
        - memory_address_to_id_sum_52
        - memory_id_to_big_sum_53)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col108, trace_2_col109, trace_2_col110, trace_2_col111],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col104, trace_2_col105, trace_2_col106, trace_2_col107],
        ))
        * range_check_7_2_5_sum_54
        * memory_address_to_id_sum_55)
        - range_check_7_2_5_sum_54
        - memory_address_to_id_sum_55)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col112, trace_2_col113, trace_2_col114, trace_2_col115],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col108, trace_2_col109, trace_2_col110, trace_2_col111],
        ))
        * memory_id_to_big_sum_56
        * range_check_7_2_5_sum_57)
        - memory_id_to_big_sum_56
        - range_check_7_2_5_sum_57)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col116, trace_2_col117, trace_2_col118, trace_2_col119],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col112, trace_2_col113, trace_2_col114, trace_2_col115],
        ))
        * memory_address_to_id_sum_58
        * memory_id_to_big_sum_59)
        - memory_address_to_id_sum_58
        - memory_id_to_big_sum_59)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col120, trace_2_col121, trace_2_col122, trace_2_col123],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col116, trace_2_col117, trace_2_col118, trace_2_col119],
        ))
        * range_check_7_2_5_sum_60
        * memory_address_to_id_sum_61)
        - range_check_7_2_5_sum_60
        - memory_address_to_id_sum_61)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col124, trace_2_col125, trace_2_col126, trace_2_col127],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col120, trace_2_col121, trace_2_col122, trace_2_col123],
        ))
        * memory_id_to_big_sum_62
        * range_check_7_2_5_sum_63)
        - memory_id_to_big_sum_62
        - range_check_7_2_5_sum_63)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col128, trace_2_col129, trace_2_col130, trace_2_col131],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col124, trace_2_col125, trace_2_col126, trace_2_col127],
        ))
        * memory_address_to_id_sum_64
        * memory_id_to_big_sum_65)
        - memory_address_to_id_sum_64
        - memory_id_to_big_sum_65)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col132, trace_2_col133, trace_2_col134, trace_2_col135],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col128, trace_2_col129, trace_2_col130, trace_2_col131],
        ))
        * range_check_7_2_5_sum_66
        * memory_address_to_id_sum_67)
        - range_check_7_2_5_sum_66
        - memory_address_to_id_sum_67)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col136, trace_2_col137, trace_2_col138, trace_2_col139],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col132, trace_2_col133, trace_2_col134, trace_2_col135],
        ))
        * memory_id_to_big_sum_68
        * range_check_7_2_5_sum_69)
        - memory_id_to_big_sum_68
        - range_check_7_2_5_sum_69)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col140, trace_2_col141, trace_2_col142, trace_2_col143],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col136, trace_2_col137, trace_2_col138, trace_2_col139],
        ))
        * memory_address_to_id_sum_70
        * memory_id_to_big_sum_71)
        - memory_address_to_id_sum_70
        - memory_id_to_big_sum_71)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col144, trace_2_col145, trace_2_col146, trace_2_col147],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col140, trace_2_col141, trace_2_col142, trace_2_col143],
        )
        - QM31Impl::from_partial_evals(
            [trace_2_col144_neg1, trace_2_col145_neg1, trace_2_col146_neg1, trace_2_col147_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * opcodes_sum_72
        * opcodes_sum_73)
        + (opcodes_sum_72 * enabler)
        - (opcodes_sum_73 * enabler))
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
            blake_round_lookup_elements: make_lookup_elements(
                qm31_const::<512137121, 4656726, 184578687, 642917762>(),
                qm31_const::<1958399945, 1971391524, 790352857, 78661490>(),
            ),
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
            range_check_7_2_5_lookup_elements: make_lookup_elements(
                qm31_const::<425514336, 1473331321, 384012983, 274885242>(),
                qm31_const::<660930654, 31738603, 1176905988, 765990201>(),
            ),
            triple_xor_32_lookup_elements: make_lookup_elements(
                qm31_const::<1306465622, 1475920612, 435786988, 143056699>(),
                qm31_const::<1864509813, 1662621571, 448425708, 599804019>(),
            ),
            verify_bitwise_xor_8_lookup_elements: make_lookup_elements(
                qm31_const::<390097169, 1715941348, 958959293, 1227669969>(),
                qm31_const::<105167513, 476596518, 1027059816, 1879697407>(),
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
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            seq_column_idx(component.claim.log_size),
            qm31_const::<735272696, 1215403647, 795393303, 879304430>(),
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
            [qm31_const::<1474817224, 804829687, 1332851487, 502514173>()].span(),
            [qm31_const::<1541926403, 939047415, 1399960351, 502514173>()].span(),
            [qm31_const::<1072162150, 2147006966, 930198302, 502514173>()].span(),
            [qm31_const::<1139271329, 133741047, 997307167, 502514173>()].span(),
            [qm31_const::<1206380508, 267958775, 1064416031, 502514173>()].span(),
            [qm31_const::<1273489687, 402176503, 1131524895, 502514173>()].span(),
            [qm31_const::<1877472298, 1610136055, 1735504671, 502514173>()].span(),
            [qm31_const::<1944581477, 1744353783, 1802613535, 502514173>()].span(),
            [qm31_const::<669619552, 1341700661, 527545181, 502514194>()].span(),
            [qm31_const::<602510373, 1207482933, 460436317, 502514194>()].span(),
            [qm31_const::<535401194, 1073265205, 393327453, 502514194>()].span(),
            [qm31_const::<468292015, 939047477, 326218589, 502514194>()].span(),
            [qm31_const::<401182836, 804829749, 259109725, 502514194>()].span(),
            [qm31_const::<334073657, 670612021, 192000861, 502514194>()].span(),
            [qm31_const::<266964478, 536394293, 124891997, 502514194>()].span(),
            [qm31_const::<199855299, 402176565, 57783133, 502514194>()].span(),
            [qm31_const::<132746120, 267958837, 2138157916, 502514193>()].span(),
            [qm31_const::<65636941, 133741109, 2071049052, 502514193>()].span(),
            [qm31_const::<2146113804, 2147007087, 2003940247, 502514213>()].span(),
            [qm31_const::<65739336, 133741169, 2071049112, 502514213>()].span(),
            [qm31_const::<2011895446, 1878571631, 1869722519, 502514213>()].span(),
            [qm31_const::<2079004625, 2012789359, 1936831383, 502514213>()].span(),
            [qm31_const::<267066873, 536394353, 124892057, 502514214>()].span(),
            [qm31_const::<334176052, 670612081, 192000921, 502514214>()].span(),
            [qm31_const::<132848515, 267958897, 2138157976, 502514213>()].span(),
            [qm31_const::<199957694, 402176625, 57783193, 502514214>()].span(),
            [qm31_const::<1609240372, 1073265263, 1467069335, 502514213>()].span(),
            [qm31_const::<1676349551, 1207482991, 1534178199, 502514213>()].span(),
            [qm31_const::<1475124409, 804829867, 1332851667, 502514233>()].span(),
            [qm31_const::<1408015230, 670612139, 1265742803, 502514233>()].span(),
            [qm31_const::<1609342767, 1073265323, 1467069395, 502514233>()].span(),
            [qm31_const::<1542233588, 939047595, 1399960531, 502514233>()].span(),
            [qm31_const::<1206687693, 267958955, 1064416211, 502514233>()].span(),
            [qm31_const::<1139578514, 133741227, 997307347, 502514233>()].span(),
            [qm31_const::<1340906051, 536394411, 1198633939, 502514233>()].span(),
            [qm31_const::<1273796872, 402176683, 1131525075, 502514233>()].span(),
            [qm31_const::<2011997841, 1878571691, 1869722579, 502514233>()].span(),
            [qm31_const::<1944888662, 1744353963, 1802613715, 502514233>()].span(),
            [qm31_const::<1877062718, 1610135815, 1735504431, 502514093>()].span(),
            [qm31_const::<1944171897, 1744353543, 1802613295, 502514093>()].span(),
            [qm31_const::<2011281076, 1878571271, 1869722159, 502514093>()].span(),
            [qm31_const::<2078390255, 2012788999, 1936831023, 502514093>()].span(),
            [qm31_const::<2145499434, 2147006727, 2003939887, 502514093>()].span(),
            [qm31_const::<65124966, 133740809, 2071048752, 502514093>()].span(),
            [qm31_const::<132234145, 267958537, 2138157616, 502514093>()].span(),
            [qm31_const::<199343324, 402176265, 57782833, 502514094>()].span(),
            [qm31_const::<1340189286, 536393991, 1198633519, 502514093>()].span(),
            [qm31_const::<1407298465, 670611719, 1265742383, 502514093>()].span(),
            [qm31_const::<1206073323, 267958595, 1064415851, 502514113>()].span(),
            [qm31_const::<1138964144, 133740867, 997306987, 502514113>()].span(),
            [qm31_const::<1071854965, 2147006786, 930198122, 502514113>()].span(),
            [qm31_const::<1004745786, 2012789058, 863089258, 502514113>()].span(),
            [qm31_const::<1474510039, 804829507, 1332851307, 502514113>()].span(),
            [qm31_const::<1407400860, 670611779, 1265742443, 502514113>()].span(),
            [qm31_const::<1340291681, 536394051, 1198633579, 502514113>()].span(),
            [qm31_const::<1273182502, 402176323, 1131524715, 502514113>()].span(),
            [qm31_const::<1742946755, 1341700419, 1601286763, 502514113>()].span(),
            [qm31_const::<1675837576, 1207482691, 1534177899, 502514113>()].span(),
            [qm31_const::<535094009, 1073265025, 393327273, 502514134>()].span(),
            [qm31_const::<602203188, 1207482753, 460436137, 502514134>()].span(),
            [qm31_const::<400875651, 804829569, 259109545, 502514134>()].span(),
            [qm31_const::<467984830, 939047297, 326218409, 502514134>()].span(),
            [qm31_const::<266657293, 536394113, 124891817, 502514134>()].span(),
            [qm31_const::<333766472, 670611841, 192000681, 502514134>()].span(),
            [qm31_const::<132438935, 267958657, 2138157736, 502514133>()].span(),
            [qm31_const::<199548114, 402176385, 57782953, 502514134>()].span(),
            [qm31_const::<2145704224, 2147006847, 2003940007, 502514133>()].span(),
            [qm31_const::<65329756, 133740929, 2071048872, 502514133>()].span(),
            [qm31_const::<2011588261, 1878571451, 1869722339, 502514153>()].span(),
            [qm31_const::<1944479082, 1744353723, 1802613475, 502514153>()].span(),
            [qm31_const::<2145806619, 2147006907, 2003940067, 502514153>()].span(),
            [qm31_const::<179325277, 825275894, 97341591, 1357105975>()].span(),
        ]
            .span();
        let interaction_values = array![
            qm31_const::<1005168032, 79980996, 1847888101, 1941984119>(),
            qm31_const::<1072277211, 214198724, 1914996965, 1941984119>(),
            qm31_const::<1139386390, 348416452, 1982105829, 1941984119>(),
            qm31_const::<1206495569, 482634180, 2049214693, 1941984119>(),
            qm31_const::<736731316, 1690593731, 1579452644, 1941984119>(),
            qm31_const::<803840495, 1824811459, 1646561508, 1941984119>(),
            qm31_const::<870949674, 1959029187, 1713670372, 1941984119>(),
            qm31_const::<938058853, 2093246915, 1780779236, 1941984119>(),
            qm31_const::<1542041464, 1153722820, 237275366, 1941984120>(),
            qm31_const::<1609150643, 1287940548, 304384230, 1941984120>(),
            qm31_const::<1577898798, 106101108, 1738096752, 1261630210>(),
            qm31_const::<1510789619, 2119367027, 1670987887, 1261630210>(),
            qm31_const::<1443680440, 1985149299, 1603879023, 1261630210>(),
            qm31_const::<1376571261, 1850931571, 1536770159, 1261630210>(),
            qm31_const::<1309462082, 1716713843, 1469661295, 1261630210>(),
            qm31_const::<1242352903, 1582496115, 1402552431, 1261630210>(),
            qm31_const::<1175243724, 1448278387, 1335443567, 1261630210>(),
            qm31_const::<1108134545, 1314060659, 1268334703, 1261630210>(),
            qm31_const::<2114772230, 1179842932, 127484017, 1261630211>(),
            qm31_const::<2047663051, 1045625204, 60375153, 1261630211>(),
            qm31_const::<906909403, 911407535, 1067008171, 1261630230>(),
            qm31_const::<974018582, 1045625263, 1134117035, 1261630230>(),
            qm31_const::<772691045, 642972079, 932790443, 1261630230>(),
            qm31_const::<839800224, 777189807, 999899307, 1261630230>(),
            qm31_const::<1175346119, 1448278447, 1335443627, 1261630230>(),
            qm31_const::<1242455298, 1582496175, 1402552491, 1261630230>(),
            qm31_const::<1041127761, 1179842991, 1201225899, 1261630230>(),
            qm31_const::<1108236940, 1314060719, 1268334763, 1261630230>(),
            qm31_const::<1443782835, 1985149359, 1603879083, 1261630230>(),
            qm31_const::<1510892014, 2119367087, 1670987947, 1261630230>(),
            qm31_const::<235889765, 1716713953, 395919581, 1261630247>(),
            qm31_const::<168780586, 1582496225, 328810717, 1261630247>(),
            qm31_const::<370108123, 1985149409, 530137309, 1261630247>(),
            qm31_const::<302998944, 1850931681, 463028445, 1261630247>(),
            qm31_const::<2114936696, 1179843040, 127484125, 1261630247>(),
            qm31_const::<2047827517, 1045625312, 60375261, 1261630247>(),
            qm31_const::<101671407, 1448278497, 261701853, 1261630247>(),
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
        assert_eq!(sum, QM31Trait::from_fixed_array(BLAKE_COMPRESS_OPCODE_SAMPLE_EVAL_RESULT))
    }
}
