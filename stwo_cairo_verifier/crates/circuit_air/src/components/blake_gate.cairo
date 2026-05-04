// This file was created by the AIR team.

use crate::components::subroutines::create_blake_output::create_blake_output_evaluate;
use crate::components::subroutines::create_blake_round_input::create_blake_round_input_evaluate;
use crate::components::subroutines::qm_31_into_u_32::qm_31_into_u_32_evaluate;
use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 151;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 7] = [
    ('VerifyBitwiseXor_8', 4), ('RangeCheck_16', 16), ('RangeCheck_15', 16), ('BlakeRound', 1),
    ('TripleXor32', 8), ('BlakeOutput', 1), ('Gate', 4),
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
        let interaction_log_sizes = [log_size; 136].span();
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
        let mut range_check_16_sum_4: QM31 = Zero::zero();
        let mut numerator_4: QM31 = Zero::zero();
        let mut range_check_15_sum_5: QM31 = Zero::zero();
        let mut numerator_5: QM31 = Zero::zero();
        let mut blake_message_sum_6: QM31 = Zero::zero();
        let mut numerator_6: QM31 = Zero::zero();
        let mut range_check_16_sum_7: QM31 = Zero::zero();
        let mut numerator_7: QM31 = Zero::zero();
        let mut range_check_15_sum_8: QM31 = Zero::zero();
        let mut numerator_8: QM31 = Zero::zero();
        let mut blake_message_sum_9: QM31 = Zero::zero();
        let mut numerator_9: QM31 = Zero::zero();
        let mut range_check_16_sum_10: QM31 = Zero::zero();
        let mut numerator_10: QM31 = Zero::zero();
        let mut range_check_15_sum_11: QM31 = Zero::zero();
        let mut numerator_11: QM31 = Zero::zero();
        let mut blake_message_sum_12: QM31 = Zero::zero();
        let mut numerator_12: QM31 = Zero::zero();
        let mut range_check_16_sum_13: QM31 = Zero::zero();
        let mut numerator_13: QM31 = Zero::zero();
        let mut range_check_15_sum_14: QM31 = Zero::zero();
        let mut numerator_14: QM31 = Zero::zero();
        let mut blake_message_sum_15: QM31 = Zero::zero();
        let mut numerator_15: QM31 = Zero::zero();
        let mut range_check_16_sum_16: QM31 = Zero::zero();
        let mut numerator_16: QM31 = Zero::zero();
        let mut range_check_15_sum_17: QM31 = Zero::zero();
        let mut numerator_17: QM31 = Zero::zero();
        let mut blake_message_sum_18: QM31 = Zero::zero();
        let mut numerator_18: QM31 = Zero::zero();
        let mut range_check_16_sum_19: QM31 = Zero::zero();
        let mut numerator_19: QM31 = Zero::zero();
        let mut range_check_15_sum_20: QM31 = Zero::zero();
        let mut numerator_20: QM31 = Zero::zero();
        let mut blake_message_sum_21: QM31 = Zero::zero();
        let mut numerator_21: QM31 = Zero::zero();
        let mut range_check_16_sum_22: QM31 = Zero::zero();
        let mut numerator_22: QM31 = Zero::zero();
        let mut range_check_15_sum_23: QM31 = Zero::zero();
        let mut numerator_23: QM31 = Zero::zero();
        let mut blake_message_sum_24: QM31 = Zero::zero();
        let mut numerator_24: QM31 = Zero::zero();
        let mut range_check_16_sum_25: QM31 = Zero::zero();
        let mut numerator_25: QM31 = Zero::zero();
        let mut range_check_15_sum_26: QM31 = Zero::zero();
        let mut numerator_26: QM31 = Zero::zero();
        let mut blake_message_sum_27: QM31 = Zero::zero();
        let mut numerator_27: QM31 = Zero::zero();
        let mut range_check_16_sum_28: QM31 = Zero::zero();
        let mut numerator_28: QM31 = Zero::zero();
        let mut range_check_15_sum_29: QM31 = Zero::zero();
        let mut numerator_29: QM31 = Zero::zero();
        let mut blake_message_sum_30: QM31 = Zero::zero();
        let mut numerator_30: QM31 = Zero::zero();
        let mut range_check_16_sum_31: QM31 = Zero::zero();
        let mut numerator_31: QM31 = Zero::zero();
        let mut range_check_15_sum_32: QM31 = Zero::zero();
        let mut numerator_32: QM31 = Zero::zero();
        let mut blake_message_sum_33: QM31 = Zero::zero();
        let mut numerator_33: QM31 = Zero::zero();
        let mut range_check_16_sum_34: QM31 = Zero::zero();
        let mut numerator_34: QM31 = Zero::zero();
        let mut range_check_15_sum_35: QM31 = Zero::zero();
        let mut numerator_35: QM31 = Zero::zero();
        let mut blake_message_sum_36: QM31 = Zero::zero();
        let mut numerator_36: QM31 = Zero::zero();
        let mut range_check_16_sum_37: QM31 = Zero::zero();
        let mut numerator_37: QM31 = Zero::zero();
        let mut range_check_15_sum_38: QM31 = Zero::zero();
        let mut numerator_38: QM31 = Zero::zero();
        let mut blake_message_sum_39: QM31 = Zero::zero();
        let mut numerator_39: QM31 = Zero::zero();
        let mut range_check_16_sum_40: QM31 = Zero::zero();
        let mut numerator_40: QM31 = Zero::zero();
        let mut range_check_15_sum_41: QM31 = Zero::zero();
        let mut numerator_41: QM31 = Zero::zero();
        let mut blake_message_sum_42: QM31 = Zero::zero();
        let mut numerator_42: QM31 = Zero::zero();
        let mut range_check_16_sum_43: QM31 = Zero::zero();
        let mut numerator_43: QM31 = Zero::zero();
        let mut range_check_15_sum_44: QM31 = Zero::zero();
        let mut numerator_44: QM31 = Zero::zero();
        let mut blake_message_sum_45: QM31 = Zero::zero();
        let mut numerator_45: QM31 = Zero::zero();
        let mut range_check_16_sum_46: QM31 = Zero::zero();
        let mut numerator_46: QM31 = Zero::zero();
        let mut range_check_15_sum_47: QM31 = Zero::zero();
        let mut numerator_47: QM31 = Zero::zero();
        let mut blake_message_sum_48: QM31 = Zero::zero();
        let mut numerator_48: QM31 = Zero::zero();
        let mut range_check_16_sum_49: QM31 = Zero::zero();
        let mut numerator_49: QM31 = Zero::zero();
        let mut range_check_15_sum_50: QM31 = Zero::zero();
        let mut numerator_50: QM31 = Zero::zero();
        let mut blake_message_sum_51: QM31 = Zero::zero();
        let mut numerator_51: QM31 = Zero::zero();
        let mut blake_round_sum_52: QM31 = Zero::zero();
        let mut numerator_52: QM31 = Zero::zero();
        let mut blake_round_sum_53: QM31 = Zero::zero();
        let mut numerator_53: QM31 = Zero::zero();
        let mut triple_xor_32_sum_54: QM31 = Zero::zero();
        let mut numerator_54: QM31 = Zero::zero();
        let mut triple_xor_32_sum_55: QM31 = Zero::zero();
        let mut numerator_55: QM31 = Zero::zero();
        let mut triple_xor_32_sum_56: QM31 = Zero::zero();
        let mut numerator_56: QM31 = Zero::zero();
        let mut triple_xor_32_sum_57: QM31 = Zero::zero();
        let mut numerator_57: QM31 = Zero::zero();
        let mut triple_xor_32_sum_58: QM31 = Zero::zero();
        let mut numerator_58: QM31 = Zero::zero();
        let mut triple_xor_32_sum_59: QM31 = Zero::zero();
        let mut numerator_59: QM31 = Zero::zero();
        let mut triple_xor_32_sum_60: QM31 = Zero::zero();
        let mut numerator_60: QM31 = Zero::zero();
        let mut triple_xor_32_sum_61: QM31 = Zero::zero();
        let mut numerator_61: QM31 = Zero::zero();
        let mut blake_output_sum_62: QM31 = Zero::zero();
        let mut numerator_62: QM31 = Zero::zero();
        let mut blake_output_sum_63: QM31 = Zero::zero();
        let mut numerator_63: QM31 = Zero::zero();
        let mut gate_sum_64: QM31 = Zero::zero();
        let mut numerator_64: QM31 = Zero::zero();
        let mut gate_sum_65: QM31 = Zero::zero();
        let mut numerator_65: QM31 = Zero::zero();
        let mut gate_sum_66: QM31 = Zero::zero();
        let mut numerator_66: QM31 = Zero::zero();
        let mut gate_sum_67: QM31 = Zero::zero();
        let mut numerator_67: QM31 = Zero::zero();
        let t0 = preprocessed_mask_values.get_and_mark_used(T_0_IDX);
        let t1 = preprocessed_mask_values.get_and_mark_used(T_1_IDX);
        let finalize_flag = preprocessed_mask_values.get_and_mark_used(FINALIZE_FLAG_IDX);
        let seq = preprocessed_mask_values
            .get_and_mark_used(seq_column_idx(*(self.claim.log_size)));
        let state_before_addr = preprocessed_mask_values.get_and_mark_used(STATE_BEFORE_ADDR_IDX);
        let compress_enabler = preprocessed_mask_values.get_and_mark_used(COMPRESS_ENABLER_IDX);
        let state_after_addr = preprocessed_mask_values.get_and_mark_used(STATE_AFTER_ADDR_IDX);
        let message0_addr = preprocessed_mask_values.get_and_mark_used(MESSAGE_0_ADDR_IDX);
        let message1_addr = preprocessed_mask_values.get_and_mark_used(MESSAGE_1_ADDR_IDX);
        let message2_addr = preprocessed_mask_values.get_and_mark_used(MESSAGE_2_ADDR_IDX);
        let message3_addr = preprocessed_mask_values.get_and_mark_used(MESSAGE_3_ADDR_IDX);

        let [
            input_state_before_limb0_limb_0_col0,
            input_state_before_limb0_limb_1_col1,
            input_state_before_limb1_limb_0_col2,
            input_state_before_limb1_limb_1_col3,
            input_state_before_limb2_limb_0_col4,
            input_state_before_limb2_limb_1_col5,
            input_state_before_limb3_limb_0_col6,
            input_state_before_limb3_limb_1_col7,
            input_state_before_limb4_limb_0_col8,
            input_state_before_limb4_limb_1_col9,
            input_state_before_limb5_limb_0_col10,
            input_state_before_limb5_limb_1_col11,
            input_state_before_limb6_limb_0_col12,
            input_state_before_limb6_limb_1_col13,
            input_state_before_limb7_limb_0_col14,
            input_state_before_limb7_limb_1_col15,
            input_state_after_limb0_limb_0_col16,
            input_state_after_limb0_limb_1_col17,
            input_state_after_limb1_limb_0_col18,
            input_state_after_limb1_limb_1_col19,
            input_state_after_limb2_limb_0_col20,
            input_state_after_limb2_limb_1_col21,
            input_state_after_limb3_limb_0_col22,
            input_state_after_limb3_limb_1_col23,
            input_state_after_limb4_limb_0_col24,
            input_state_after_limb4_limb_1_col25,
            input_state_after_limb5_limb_0_col26,
            input_state_after_limb5_limb_1_col27,
            input_state_after_limb6_limb_0_col28,
            input_state_after_limb6_limb_1_col29,
            input_state_after_limb7_limb_0_col30,
            input_state_after_limb7_limb_1_col31,
            input_message_limb0_col32,
            input_message_limb1_col33,
            input_message_limb2_col34,
            input_message_limb3_col35,
            input_message_limb4_col36,
            input_message_limb5_col37,
            input_message_limb6_col38,
            input_message_limb7_col39,
            input_message_limb8_col40,
            input_message_limb9_col41,
            input_message_limb10_col42,
            input_message_limb11_col43,
            input_message_limb12_col44,
            input_message_limb13_col45,
            input_message_limb14_col46,
            input_message_limb15_col47,
            ms_8_bits_col48,
            ms_8_bits_col49,
            xor_col50,
            xor_col51,
            xor_col52,
            xor_col53,
            limbi_low_col54,
            limbi_high_col55,
            limbi_inv_or_one_col56,
            limbi_low_col57,
            limbi_high_col58,
            limbi_inv_or_one_col59,
            limbi_low_col60,
            limbi_high_col61,
            limbi_inv_or_one_col62,
            limbi_low_col63,
            limbi_high_col64,
            limbi_inv_or_one_col65,
            limbi_low_col66,
            limbi_high_col67,
            limbi_inv_or_one_col68,
            limbi_low_col69,
            limbi_high_col70,
            limbi_inv_or_one_col71,
            limbi_low_col72,
            limbi_high_col73,
            limbi_inv_or_one_col74,
            limbi_low_col75,
            limbi_high_col76,
            limbi_inv_or_one_col77,
            limbi_low_col78,
            limbi_high_col79,
            limbi_inv_or_one_col80,
            limbi_low_col81,
            limbi_high_col82,
            limbi_inv_or_one_col83,
            limbi_low_col84,
            limbi_high_col85,
            limbi_inv_or_one_col86,
            limbi_low_col87,
            limbi_high_col88,
            limbi_inv_or_one_col89,
            limbi_low_col90,
            limbi_high_col91,
            limbi_inv_or_one_col92,
            limbi_low_col93,
            limbi_high_col94,
            limbi_inv_or_one_col95,
            limbi_low_col96,
            limbi_high_col97,
            limbi_inv_or_one_col98,
            limbi_low_col99,
            limbi_high_col100,
            limbi_inv_or_one_col101,
            blake_round_output_limb_0_col102,
            blake_round_output_limb_1_col103,
            blake_round_output_limb_2_col104,
            blake_round_output_limb_3_col105,
            blake_round_output_limb_4_col106,
            blake_round_output_limb_5_col107,
            blake_round_output_limb_6_col108,
            blake_round_output_limb_7_col109,
            blake_round_output_limb_8_col110,
            blake_round_output_limb_9_col111,
            blake_round_output_limb_10_col112,
            blake_round_output_limb_11_col113,
            blake_round_output_limb_12_col114,
            blake_round_output_limb_13_col115,
            blake_round_output_limb_14_col116,
            blake_round_output_limb_15_col117,
            blake_round_output_limb_16_col118,
            blake_round_output_limb_17_col119,
            blake_round_output_limb_18_col120,
            blake_round_output_limb_19_col121,
            blake_round_output_limb_20_col122,
            blake_round_output_limb_21_col123,
            blake_round_output_limb_22_col124,
            blake_round_output_limb_23_col125,
            blake_round_output_limb_24_col126,
            blake_round_output_limb_25_col127,
            blake_round_output_limb_26_col128,
            blake_round_output_limb_27_col129,
            blake_round_output_limb_28_col130,
            blake_round_output_limb_29_col131,
            blake_round_output_limb_30_col132,
            blake_round_output_limb_31_col133,
            blake_round_output_limb_32_col134,
            triple_xor_32_output_limb_0_col135,
            triple_xor_32_output_limb_1_col136,
            triple_xor_32_output_limb_0_col137,
            triple_xor_32_output_limb_1_col138,
            triple_xor_32_output_limb_0_col139,
            triple_xor_32_output_limb_1_col140,
            triple_xor_32_output_limb_0_col141,
            triple_xor_32_output_limb_1_col142,
            triple_xor_32_output_limb_0_col143,
            triple_xor_32_output_limb_1_col144,
            triple_xor_32_output_limb_0_col145,
            triple_xor_32_output_limb_1_col146,
            triple_xor_32_output_limb_0_col147,
            triple_xor_32_output_limb_1_col148,
            triple_xor_32_output_limb_0_col149,
            triple_xor_32_output_limb_1_col150,
        ]: [Span<QM31>; 151] =
            (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [input_state_before_limb0_limb_0_col0]: [QM31; 1] =
            (*input_state_before_limb0_limb_0_col0
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_before_limb0_limb_1_col1]: [QM31; 1] =
            (*input_state_before_limb0_limb_1_col1
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_before_limb1_limb_0_col2]: [QM31; 1] =
            (*input_state_before_limb1_limb_0_col2
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_before_limb1_limb_1_col3]: [QM31; 1] =
            (*input_state_before_limb1_limb_1_col3
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_before_limb2_limb_0_col4]: [QM31; 1] =
            (*input_state_before_limb2_limb_0_col4
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_before_limb2_limb_1_col5]: [QM31; 1] =
            (*input_state_before_limb2_limb_1_col5
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_before_limb3_limb_0_col6]: [QM31; 1] =
            (*input_state_before_limb3_limb_0_col6
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_before_limb3_limb_1_col7]: [QM31; 1] =
            (*input_state_before_limb3_limb_1_col7
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_before_limb4_limb_0_col8]: [QM31; 1] =
            (*input_state_before_limb4_limb_0_col8
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_before_limb4_limb_1_col9]: [QM31; 1] =
            (*input_state_before_limb4_limb_1_col9
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_before_limb5_limb_0_col10]: [QM31; 1] =
            (*input_state_before_limb5_limb_0_col10
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_before_limb5_limb_1_col11]: [QM31; 1] =
            (*input_state_before_limb5_limb_1_col11
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_before_limb6_limb_0_col12]: [QM31; 1] =
            (*input_state_before_limb6_limb_0_col12
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_before_limb6_limb_1_col13]: [QM31; 1] =
            (*input_state_before_limb6_limb_1_col13
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_before_limb7_limb_0_col14]: [QM31; 1] =
            (*input_state_before_limb7_limb_0_col14
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_before_limb7_limb_1_col15]: [QM31; 1] =
            (*input_state_before_limb7_limb_1_col15
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_after_limb0_limb_0_col16]: [QM31; 1] =
            (*input_state_after_limb0_limb_0_col16
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_after_limb0_limb_1_col17]: [QM31; 1] =
            (*input_state_after_limb0_limb_1_col17
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_after_limb1_limb_0_col18]: [QM31; 1] =
            (*input_state_after_limb1_limb_0_col18
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_after_limb1_limb_1_col19]: [QM31; 1] =
            (*input_state_after_limb1_limb_1_col19
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_after_limb2_limb_0_col20]: [QM31; 1] =
            (*input_state_after_limb2_limb_0_col20
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_after_limb2_limb_1_col21]: [QM31; 1] =
            (*input_state_after_limb2_limb_1_col21
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_after_limb3_limb_0_col22]: [QM31; 1] =
            (*input_state_after_limb3_limb_0_col22
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_after_limb3_limb_1_col23]: [QM31; 1] =
            (*input_state_after_limb3_limb_1_col23
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_after_limb4_limb_0_col24]: [QM31; 1] =
            (*input_state_after_limb4_limb_0_col24
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_after_limb4_limb_1_col25]: [QM31; 1] =
            (*input_state_after_limb4_limb_1_col25
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_after_limb5_limb_0_col26]: [QM31; 1] =
            (*input_state_after_limb5_limb_0_col26
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_after_limb5_limb_1_col27]: [QM31; 1] =
            (*input_state_after_limb5_limb_1_col27
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_after_limb6_limb_0_col28]: [QM31; 1] =
            (*input_state_after_limb6_limb_0_col28
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_after_limb6_limb_1_col29]: [QM31; 1] =
            (*input_state_after_limb6_limb_1_col29
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_after_limb7_limb_0_col30]: [QM31; 1] =
            (*input_state_after_limb7_limb_0_col30
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_after_limb7_limb_1_col31]: [QM31; 1] =
            (*input_state_after_limb7_limb_1_col31
            .try_into()
            .unwrap())
            .unbox();
        let [input_message_limb0_col32]: [QM31; 1] = (*input_message_limb0_col32
            .try_into()
            .unwrap())
            .unbox();
        let [input_message_limb1_col33]: [QM31; 1] = (*input_message_limb1_col33
            .try_into()
            .unwrap())
            .unbox();
        let [input_message_limb2_col34]: [QM31; 1] = (*input_message_limb2_col34
            .try_into()
            .unwrap())
            .unbox();
        let [input_message_limb3_col35]: [QM31; 1] = (*input_message_limb3_col35
            .try_into()
            .unwrap())
            .unbox();
        let [input_message_limb4_col36]: [QM31; 1] = (*input_message_limb4_col36
            .try_into()
            .unwrap())
            .unbox();
        let [input_message_limb5_col37]: [QM31; 1] = (*input_message_limb5_col37
            .try_into()
            .unwrap())
            .unbox();
        let [input_message_limb6_col38]: [QM31; 1] = (*input_message_limb6_col38
            .try_into()
            .unwrap())
            .unbox();
        let [input_message_limb7_col39]: [QM31; 1] = (*input_message_limb7_col39
            .try_into()
            .unwrap())
            .unbox();
        let [input_message_limb8_col40]: [QM31; 1] = (*input_message_limb8_col40
            .try_into()
            .unwrap())
            .unbox();
        let [input_message_limb9_col41]: [QM31; 1] = (*input_message_limb9_col41
            .try_into()
            .unwrap())
            .unbox();
        let [input_message_limb10_col42]: [QM31; 1] = (*input_message_limb10_col42
            .try_into()
            .unwrap())
            .unbox();
        let [input_message_limb11_col43]: [QM31; 1] = (*input_message_limb11_col43
            .try_into()
            .unwrap())
            .unbox();
        let [input_message_limb12_col44]: [QM31; 1] = (*input_message_limb12_col44
            .try_into()
            .unwrap())
            .unbox();
        let [input_message_limb13_col45]: [QM31; 1] = (*input_message_limb13_col45
            .try_into()
            .unwrap())
            .unbox();
        let [input_message_limb14_col46]: [QM31; 1] = (*input_message_limb14_col46
            .try_into()
            .unwrap())
            .unbox();
        let [input_message_limb15_col47]: [QM31; 1] = (*input_message_limb15_col47
            .try_into()
            .unwrap())
            .unbox();
        let [ms_8_bits_col48]: [QM31; 1] = (*ms_8_bits_col48.try_into().unwrap()).unbox();
        let [ms_8_bits_col49]: [QM31; 1] = (*ms_8_bits_col49.try_into().unwrap()).unbox();
        let [xor_col50]: [QM31; 1] = (*xor_col50.try_into().unwrap()).unbox();
        let [xor_col51]: [QM31; 1] = (*xor_col51.try_into().unwrap()).unbox();
        let [xor_col52]: [QM31; 1] = (*xor_col52.try_into().unwrap()).unbox();
        let [xor_col53]: [QM31; 1] = (*xor_col53.try_into().unwrap()).unbox();
        let [limbi_low_col54]: [QM31; 1] = (*limbi_low_col54.try_into().unwrap()).unbox();
        let [limbi_high_col55]: [QM31; 1] = (*limbi_high_col55.try_into().unwrap()).unbox();
        let [limbi_inv_or_one_col56]: [QM31; 1] = (*limbi_inv_or_one_col56.try_into().unwrap())
            .unbox();
        let [limbi_low_col57]: [QM31; 1] = (*limbi_low_col57.try_into().unwrap()).unbox();
        let [limbi_high_col58]: [QM31; 1] = (*limbi_high_col58.try_into().unwrap()).unbox();
        let [limbi_inv_or_one_col59]: [QM31; 1] = (*limbi_inv_or_one_col59.try_into().unwrap())
            .unbox();
        let [limbi_low_col60]: [QM31; 1] = (*limbi_low_col60.try_into().unwrap()).unbox();
        let [limbi_high_col61]: [QM31; 1] = (*limbi_high_col61.try_into().unwrap()).unbox();
        let [limbi_inv_or_one_col62]: [QM31; 1] = (*limbi_inv_or_one_col62.try_into().unwrap())
            .unbox();
        let [limbi_low_col63]: [QM31; 1] = (*limbi_low_col63.try_into().unwrap()).unbox();
        let [limbi_high_col64]: [QM31; 1] = (*limbi_high_col64.try_into().unwrap()).unbox();
        let [limbi_inv_or_one_col65]: [QM31; 1] = (*limbi_inv_or_one_col65.try_into().unwrap())
            .unbox();
        let [limbi_low_col66]: [QM31; 1] = (*limbi_low_col66.try_into().unwrap()).unbox();
        let [limbi_high_col67]: [QM31; 1] = (*limbi_high_col67.try_into().unwrap()).unbox();
        let [limbi_inv_or_one_col68]: [QM31; 1] = (*limbi_inv_or_one_col68.try_into().unwrap())
            .unbox();
        let [limbi_low_col69]: [QM31; 1] = (*limbi_low_col69.try_into().unwrap()).unbox();
        let [limbi_high_col70]: [QM31; 1] = (*limbi_high_col70.try_into().unwrap()).unbox();
        let [limbi_inv_or_one_col71]: [QM31; 1] = (*limbi_inv_or_one_col71.try_into().unwrap())
            .unbox();
        let [limbi_low_col72]: [QM31; 1] = (*limbi_low_col72.try_into().unwrap()).unbox();
        let [limbi_high_col73]: [QM31; 1] = (*limbi_high_col73.try_into().unwrap()).unbox();
        let [limbi_inv_or_one_col74]: [QM31; 1] = (*limbi_inv_or_one_col74.try_into().unwrap())
            .unbox();
        let [limbi_low_col75]: [QM31; 1] = (*limbi_low_col75.try_into().unwrap()).unbox();
        let [limbi_high_col76]: [QM31; 1] = (*limbi_high_col76.try_into().unwrap()).unbox();
        let [limbi_inv_or_one_col77]: [QM31; 1] = (*limbi_inv_or_one_col77.try_into().unwrap())
            .unbox();
        let [limbi_low_col78]: [QM31; 1] = (*limbi_low_col78.try_into().unwrap()).unbox();
        let [limbi_high_col79]: [QM31; 1] = (*limbi_high_col79.try_into().unwrap()).unbox();
        let [limbi_inv_or_one_col80]: [QM31; 1] = (*limbi_inv_or_one_col80.try_into().unwrap())
            .unbox();
        let [limbi_low_col81]: [QM31; 1] = (*limbi_low_col81.try_into().unwrap()).unbox();
        let [limbi_high_col82]: [QM31; 1] = (*limbi_high_col82.try_into().unwrap()).unbox();
        let [limbi_inv_or_one_col83]: [QM31; 1] = (*limbi_inv_or_one_col83.try_into().unwrap())
            .unbox();
        let [limbi_low_col84]: [QM31; 1] = (*limbi_low_col84.try_into().unwrap()).unbox();
        let [limbi_high_col85]: [QM31; 1] = (*limbi_high_col85.try_into().unwrap()).unbox();
        let [limbi_inv_or_one_col86]: [QM31; 1] = (*limbi_inv_or_one_col86.try_into().unwrap())
            .unbox();
        let [limbi_low_col87]: [QM31; 1] = (*limbi_low_col87.try_into().unwrap()).unbox();
        let [limbi_high_col88]: [QM31; 1] = (*limbi_high_col88.try_into().unwrap()).unbox();
        let [limbi_inv_or_one_col89]: [QM31; 1] = (*limbi_inv_or_one_col89.try_into().unwrap())
            .unbox();
        let [limbi_low_col90]: [QM31; 1] = (*limbi_low_col90.try_into().unwrap()).unbox();
        let [limbi_high_col91]: [QM31; 1] = (*limbi_high_col91.try_into().unwrap()).unbox();
        let [limbi_inv_or_one_col92]: [QM31; 1] = (*limbi_inv_or_one_col92.try_into().unwrap())
            .unbox();
        let [limbi_low_col93]: [QM31; 1] = (*limbi_low_col93.try_into().unwrap()).unbox();
        let [limbi_high_col94]: [QM31; 1] = (*limbi_high_col94.try_into().unwrap()).unbox();
        let [limbi_inv_or_one_col95]: [QM31; 1] = (*limbi_inv_or_one_col95.try_into().unwrap())
            .unbox();
        let [limbi_low_col96]: [QM31; 1] = (*limbi_low_col96.try_into().unwrap()).unbox();
        let [limbi_high_col97]: [QM31; 1] = (*limbi_high_col97.try_into().unwrap()).unbox();
        let [limbi_inv_or_one_col98]: [QM31; 1] = (*limbi_inv_or_one_col98.try_into().unwrap())
            .unbox();
        let [limbi_low_col99]: [QM31; 1] = (*limbi_low_col99.try_into().unwrap()).unbox();
        let [limbi_high_col100]: [QM31; 1] = (*limbi_high_col100.try_into().unwrap()).unbox();
        let [limbi_inv_or_one_col101]: [QM31; 1] = (*limbi_inv_or_one_col101.try_into().unwrap())
            .unbox();
        let [blake_round_output_limb_0_col102]: [QM31; 1] = (*blake_round_output_limb_0_col102
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_1_col103]: [QM31; 1] = (*blake_round_output_limb_1_col103
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_2_col104]: [QM31; 1] = (*blake_round_output_limb_2_col104
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_3_col105]: [QM31; 1] = (*blake_round_output_limb_3_col105
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_4_col106]: [QM31; 1] = (*blake_round_output_limb_4_col106
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_5_col107]: [QM31; 1] = (*blake_round_output_limb_5_col107
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_6_col108]: [QM31; 1] = (*blake_round_output_limb_6_col108
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_7_col109]: [QM31; 1] = (*blake_round_output_limb_7_col109
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_8_col110]: [QM31; 1] = (*blake_round_output_limb_8_col110
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_9_col111]: [QM31; 1] = (*blake_round_output_limb_9_col111
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_10_col112]: [QM31; 1] = (*blake_round_output_limb_10_col112
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_11_col113]: [QM31; 1] = (*blake_round_output_limb_11_col113
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_12_col114]: [QM31; 1] = (*blake_round_output_limb_12_col114
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_13_col115]: [QM31; 1] = (*blake_round_output_limb_13_col115
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_14_col116]: [QM31; 1] = (*blake_round_output_limb_14_col116
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_15_col117]: [QM31; 1] = (*blake_round_output_limb_15_col117
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_16_col118]: [QM31; 1] = (*blake_round_output_limb_16_col118
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_17_col119]: [QM31; 1] = (*blake_round_output_limb_17_col119
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_18_col120]: [QM31; 1] = (*blake_round_output_limb_18_col120
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_19_col121]: [QM31; 1] = (*blake_round_output_limb_19_col121
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_20_col122]: [QM31; 1] = (*blake_round_output_limb_20_col122
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_21_col123]: [QM31; 1] = (*blake_round_output_limb_21_col123
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_22_col124]: [QM31; 1] = (*blake_round_output_limb_22_col124
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_23_col125]: [QM31; 1] = (*blake_round_output_limb_23_col125
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_24_col126]: [QM31; 1] = (*blake_round_output_limb_24_col126
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_25_col127]: [QM31; 1] = (*blake_round_output_limb_25_col127
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_26_col128]: [QM31; 1] = (*blake_round_output_limb_26_col128
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_27_col129]: [QM31; 1] = (*blake_round_output_limb_27_col129
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_28_col130]: [QM31; 1] = (*blake_round_output_limb_28_col130
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_29_col131]: [QM31; 1] = (*blake_round_output_limb_29_col131
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_30_col132]: [QM31; 1] = (*blake_round_output_limb_30_col132
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_31_col133]: [QM31; 1] = (*blake_round_output_limb_31_col133
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_32_col134]: [QM31; 1] = (*blake_round_output_limb_32_col134
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
        let [triple_xor_32_output_limb_0_col141]: [QM31; 1] = (*triple_xor_32_output_limb_0_col141
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_1_col142]: [QM31; 1] = (*triple_xor_32_output_limb_1_col142
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_0_col143]: [QM31; 1] = (*triple_xor_32_output_limb_0_col143
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_1_col144]: [QM31; 1] = (*triple_xor_32_output_limb_1_col144
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_0_col145]: [QM31; 1] = (*triple_xor_32_output_limb_0_col145
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_1_col146]: [QM31; 1] = (*triple_xor_32_output_limb_1_col146
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_0_col147]: [QM31; 1] = (*triple_xor_32_output_limb_0_col147
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_1_col148]: [QM31; 1] = (*triple_xor_32_output_limb_1_col148
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_0_col149]: [QM31; 1] = (*triple_xor_32_output_limb_0_col149
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_1_col150]: [QM31; 1] = (*triple_xor_32_output_limb_1_col150
            .try_into()
            .unwrap())
            .unbox();

        core::internal::revoke_ap_tracking();

        let [
            _create_blake_round_input_output_tmp_8e0ec_12_limb_0,
            _create_blake_round_input_output_tmp_8e0ec_12_limb_1,
            _create_blake_round_input_output_tmp_8e0ec_12_limb_2,
            _create_blake_round_input_output_tmp_8e0ec_12_limb_3,
            _create_blake_round_input_output_tmp_8e0ec_12_limb_4,
            _create_blake_round_input_output_tmp_8e0ec_12_limb_5,
            _create_blake_round_input_output_tmp_8e0ec_12_limb_6,
            _create_blake_round_input_output_tmp_8e0ec_12_limb_7,
            _create_blake_round_input_output_tmp_8e0ec_12_limb_8,
            _create_blake_round_input_output_tmp_8e0ec_12_limb_9,
            _create_blake_round_input_output_tmp_8e0ec_12_limb_10,
            _create_blake_round_input_output_tmp_8e0ec_12_limb_11,
            _create_blake_round_input_output_tmp_8e0ec_12_limb_12,
            _create_blake_round_input_output_tmp_8e0ec_12_limb_13,
            _create_blake_round_input_output_tmp_8e0ec_12_limb_14,
            _create_blake_round_input_output_tmp_8e0ec_12_limb_15,
            create_blake_round_input_output_tmp_8e0ec_12_limb_24,
            create_blake_round_input_output_tmp_8e0ec_12_limb_25,
            create_blake_round_input_output_tmp_8e0ec_12_limb_28,
            create_blake_round_input_output_tmp_8e0ec_12_limb_29,
        ] =
            create_blake_round_input_evaluate(
            [
                input_state_before_limb0_limb_0_col0, input_state_before_limb0_limb_1_col1,
                input_state_before_limb1_limb_0_col2, input_state_before_limb1_limb_1_col3,
                input_state_before_limb2_limb_0_col4, input_state_before_limb2_limb_1_col5,
                input_state_before_limb3_limb_0_col6, input_state_before_limb3_limb_1_col7,
                input_state_before_limb4_limb_0_col8, input_state_before_limb4_limb_1_col9,
                input_state_before_limb5_limb_0_col10, input_state_before_limb5_limb_1_col11,
                input_state_before_limb6_limb_0_col12, input_state_before_limb6_limb_1_col13,
                input_state_before_limb7_limb_0_col14, input_state_before_limb7_limb_1_col15,
                finalize_flag,
            ],
            ms_8_bits_col48,
            ms_8_bits_col49,
            xor_col50,
            xor_col51,
            xor_col52,
            xor_col53,
            self.common_lookup_elements,
            t0,
            t1,
            ref verify_bitwise_xor_8_sum_0,
            ref numerator_0,
            ref verify_bitwise_xor_8_sum_1,
            ref numerator_1,
            ref verify_bitwise_xor_8_sum_2,
            ref numerator_2,
            ref verify_bitwise_xor_8_sum_3,
            ref numerator_3,
            ref sum,
            random_coeff,
        );
        qm_31_into_u_32_evaluate(
            [
                input_message_limb0_col32, input_message_limb1_col33, input_message_limb2_col34,
                input_message_limb3_col35, input_message_limb4_col36, input_message_limb5_col37,
                input_message_limb6_col38, input_message_limb7_col39, input_message_limb8_col40,
                input_message_limb9_col41, input_message_limb10_col42, input_message_limb11_col43,
                input_message_limb12_col44, input_message_limb13_col45, input_message_limb14_col46,
                input_message_limb15_col47, seq,
            ],
            limbi_low_col54,
            limbi_high_col55,
            limbi_inv_or_one_col56,
            limbi_low_col57,
            limbi_high_col58,
            limbi_inv_or_one_col59,
            limbi_low_col60,
            limbi_high_col61,
            limbi_inv_or_one_col62,
            limbi_low_col63,
            limbi_high_col64,
            limbi_inv_or_one_col65,
            limbi_low_col66,
            limbi_high_col67,
            limbi_inv_or_one_col68,
            limbi_low_col69,
            limbi_high_col70,
            limbi_inv_or_one_col71,
            limbi_low_col72,
            limbi_high_col73,
            limbi_inv_or_one_col74,
            limbi_low_col75,
            limbi_high_col76,
            limbi_inv_or_one_col77,
            limbi_low_col78,
            limbi_high_col79,
            limbi_inv_or_one_col80,
            limbi_low_col81,
            limbi_high_col82,
            limbi_inv_or_one_col83,
            limbi_low_col84,
            limbi_high_col85,
            limbi_inv_or_one_col86,
            limbi_low_col87,
            limbi_high_col88,
            limbi_inv_or_one_col89,
            limbi_low_col90,
            limbi_high_col91,
            limbi_inv_or_one_col92,
            limbi_low_col93,
            limbi_high_col94,
            limbi_inv_or_one_col95,
            limbi_low_col96,
            limbi_high_col97,
            limbi_inv_or_one_col98,
            limbi_low_col99,
            limbi_high_col100,
            limbi_inv_or_one_col101,
            compress_enabler,
            self.common_lookup_elements,
            ref range_check_16_sum_4,
            ref numerator_4,
            ref range_check_15_sum_5,
            ref numerator_5,
            ref blake_message_sum_6,
            ref numerator_6,
            ref range_check_16_sum_7,
            ref numerator_7,
            ref range_check_15_sum_8,
            ref numerator_8,
            ref blake_message_sum_9,
            ref numerator_9,
            ref range_check_16_sum_10,
            ref numerator_10,
            ref range_check_15_sum_11,
            ref numerator_11,
            ref blake_message_sum_12,
            ref numerator_12,
            ref range_check_16_sum_13,
            ref numerator_13,
            ref range_check_15_sum_14,
            ref numerator_14,
            ref blake_message_sum_15,
            ref numerator_15,
            ref range_check_16_sum_16,
            ref numerator_16,
            ref range_check_15_sum_17,
            ref numerator_17,
            ref blake_message_sum_18,
            ref numerator_18,
            ref range_check_16_sum_19,
            ref numerator_19,
            ref range_check_15_sum_20,
            ref numerator_20,
            ref blake_message_sum_21,
            ref numerator_21,
            ref range_check_16_sum_22,
            ref numerator_22,
            ref range_check_15_sum_23,
            ref numerator_23,
            ref blake_message_sum_24,
            ref numerator_24,
            ref range_check_16_sum_25,
            ref numerator_25,
            ref range_check_15_sum_26,
            ref numerator_26,
            ref blake_message_sum_27,
            ref numerator_27,
            ref range_check_16_sum_28,
            ref numerator_28,
            ref range_check_15_sum_29,
            ref numerator_29,
            ref blake_message_sum_30,
            ref numerator_30,
            ref range_check_16_sum_31,
            ref numerator_31,
            ref range_check_15_sum_32,
            ref numerator_32,
            ref blake_message_sum_33,
            ref numerator_33,
            ref range_check_16_sum_34,
            ref numerator_34,
            ref range_check_15_sum_35,
            ref numerator_35,
            ref blake_message_sum_36,
            ref numerator_36,
            ref range_check_16_sum_37,
            ref numerator_37,
            ref range_check_15_sum_38,
            ref numerator_38,
            ref blake_message_sum_39,
            ref numerator_39,
            ref range_check_16_sum_40,
            ref numerator_40,
            ref range_check_15_sum_41,
            ref numerator_41,
            ref blake_message_sum_42,
            ref numerator_42,
            ref range_check_16_sum_43,
            ref numerator_43,
            ref range_check_15_sum_44,
            ref numerator_44,
            ref blake_message_sum_45,
            ref numerator_45,
            ref range_check_16_sum_46,
            ref numerator_46,
            ref range_check_15_sum_47,
            ref numerator_47,
            ref blake_message_sum_48,
            ref numerator_48,
            ref range_check_16_sum_49,
            ref numerator_49,
            ref range_check_15_sum_50,
            ref numerator_50,
            ref blake_message_sum_51,
            ref numerator_51,
            ref sum,
            random_coeff,
        );

        blake_round_sum_52 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<40528774, 0, 0, 0>(), seq, qm31_const::<0, 0, 0, 0>(),
                    input_state_before_limb0_limb_0_col0, input_state_before_limb0_limb_1_col1,
                    input_state_before_limb1_limb_0_col2, input_state_before_limb1_limb_1_col3,
                    input_state_before_limb2_limb_0_col4, input_state_before_limb2_limb_1_col5,
                    input_state_before_limb3_limb_0_col6, input_state_before_limb3_limb_1_col7,
                    input_state_before_limb4_limb_0_col8, input_state_before_limb4_limb_1_col9,
                    input_state_before_limb5_limb_0_col10, input_state_before_limb5_limb_1_col11,
                    input_state_before_limb6_limb_0_col12, input_state_before_limb6_limb_1_col13,
                    input_state_before_limb7_limb_0_col14, input_state_before_limb7_limb_1_col15,
                    qm31_const::<58983, 0, 0, 0>(), qm31_const::<27145, 0, 0, 0>(),
                    qm31_const::<44677, 0, 0, 0>(), qm31_const::<47975, 0, 0, 0>(),
                    qm31_const::<62322, 0, 0, 0>(), qm31_const::<15470, 0, 0, 0>(),
                    qm31_const::<62778, 0, 0, 0>(), qm31_const::<42319, 0, 0, 0>(),
                    create_blake_round_input_output_tmp_8e0ec_12_limb_24,
                    create_blake_round_input_output_tmp_8e0ec_12_limb_25,
                    qm31_const::<26764, 0, 0, 0>(), qm31_const::<39685, 0, 0, 0>(),
                    create_blake_round_input_output_tmp_8e0ec_12_limb_28,
                    create_blake_round_input_output_tmp_8e0ec_12_limb_29,
                    qm31_const::<52505, 0, 0, 0>(), qm31_const::<23520, 0, 0, 0>(), seq,
                ]
                    .span(),
            );
        numerator_52 = compress_enabler;

        blake_round_sum_53 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<40528774, 0, 0, 0>(), seq, qm31_const::<10, 0, 0, 0>(),
                    blake_round_output_limb_0_col102, blake_round_output_limb_1_col103,
                    blake_round_output_limb_2_col104, blake_round_output_limb_3_col105,
                    blake_round_output_limb_4_col106, blake_round_output_limb_5_col107,
                    blake_round_output_limb_6_col108, blake_round_output_limb_7_col109,
                    blake_round_output_limb_8_col110, blake_round_output_limb_9_col111,
                    blake_round_output_limb_10_col112, blake_round_output_limb_11_col113,
                    blake_round_output_limb_12_col114, blake_round_output_limb_13_col115,
                    blake_round_output_limb_14_col116, blake_round_output_limb_15_col117,
                    blake_round_output_limb_16_col118, blake_round_output_limb_17_col119,
                    blake_round_output_limb_18_col120, blake_round_output_limb_19_col121,
                    blake_round_output_limb_20_col122, blake_round_output_limb_21_col123,
                    blake_round_output_limb_22_col124, blake_round_output_limb_23_col125,
                    blake_round_output_limb_24_col126, blake_round_output_limb_25_col127,
                    blake_round_output_limb_26_col128, blake_round_output_limb_27_col129,
                    blake_round_output_limb_28_col130, blake_round_output_limb_29_col131,
                    blake_round_output_limb_30_col132, blake_round_output_limb_31_col133,
                    blake_round_output_limb_32_col134,
                ]
                    .span(),
            );
        numerator_53 = compress_enabler;
        create_blake_output_evaluate(
            [
                input_state_before_limb0_limb_0_col0, input_state_before_limb0_limb_1_col1,
                input_state_before_limb1_limb_0_col2, input_state_before_limb1_limb_1_col3,
                input_state_before_limb2_limb_0_col4, input_state_before_limb2_limb_1_col5,
                input_state_before_limb3_limb_0_col6, input_state_before_limb3_limb_1_col7,
                input_state_before_limb4_limb_0_col8, input_state_before_limb4_limb_1_col9,
                input_state_before_limb5_limb_0_col10, input_state_before_limb5_limb_1_col11,
                input_state_before_limb6_limb_0_col12, input_state_before_limb6_limb_1_col13,
                input_state_before_limb7_limb_0_col14, input_state_before_limb7_limb_1_col15,
                blake_round_output_limb_0_col102, blake_round_output_limb_1_col103,
                blake_round_output_limb_2_col104, blake_round_output_limb_3_col105,
                blake_round_output_limb_4_col106, blake_round_output_limb_5_col107,
                blake_round_output_limb_6_col108, blake_round_output_limb_7_col109,
                blake_round_output_limb_8_col110, blake_round_output_limb_9_col111,
                blake_round_output_limb_10_col112, blake_round_output_limb_11_col113,
                blake_round_output_limb_12_col114, blake_round_output_limb_13_col115,
                blake_round_output_limb_14_col116, blake_round_output_limb_15_col117,
                blake_round_output_limb_16_col118, blake_round_output_limb_17_col119,
                blake_round_output_limb_18_col120, blake_round_output_limb_19_col121,
                blake_round_output_limb_20_col122, blake_round_output_limb_21_col123,
                blake_round_output_limb_22_col124, blake_round_output_limb_23_col125,
                blake_round_output_limb_24_col126, blake_round_output_limb_25_col127,
                blake_round_output_limb_26_col128, blake_round_output_limb_27_col129,
                blake_round_output_limb_28_col130, blake_round_output_limb_29_col131,
                blake_round_output_limb_30_col132, blake_round_output_limb_31_col133,
            ],
            triple_xor_32_output_limb_0_col135,
            triple_xor_32_output_limb_1_col136,
            triple_xor_32_output_limb_0_col137,
            triple_xor_32_output_limb_1_col138,
            triple_xor_32_output_limb_0_col139,
            triple_xor_32_output_limb_1_col140,
            triple_xor_32_output_limb_0_col141,
            triple_xor_32_output_limb_1_col142,
            triple_xor_32_output_limb_0_col143,
            triple_xor_32_output_limb_1_col144,
            triple_xor_32_output_limb_0_col145,
            triple_xor_32_output_limb_1_col146,
            triple_xor_32_output_limb_0_col147,
            triple_xor_32_output_limb_1_col148,
            triple_xor_32_output_limb_0_col149,
            triple_xor_32_output_limb_1_col150,
            compress_enabler,
            self.common_lookup_elements,
            ref triple_xor_32_sum_54,
            ref numerator_54,
            ref triple_xor_32_sum_55,
            ref numerator_55,
            ref triple_xor_32_sum_56,
            ref numerator_56,
            ref triple_xor_32_sum_57,
            ref numerator_57,
            ref triple_xor_32_sum_58,
            ref numerator_58,
            ref triple_xor_32_sum_59,
            ref numerator_59,
            ref triple_xor_32_sum_60,
            ref numerator_60,
            ref triple_xor_32_sum_61,
            ref numerator_61,
            ref sum,
            random_coeff,
        );

        // Constraint - Blake output h[0].low() matches expected
        let constraint_quotient = ((triple_xor_32_output_limb_0_col135
            - input_state_after_limb0_limb_0_col16));
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - Blake output h[0].high() matches expected
        let constraint_quotient = ((triple_xor_32_output_limb_1_col136
            - input_state_after_limb0_limb_1_col17));
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - Blake output h[1].low() matches expected
        let constraint_quotient = ((triple_xor_32_output_limb_0_col137
            - input_state_after_limb1_limb_0_col18));
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - Blake output h[1].high() matches expected
        let constraint_quotient = ((triple_xor_32_output_limb_1_col138
            - input_state_after_limb1_limb_1_col19));
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - Blake output h[2].low() matches expected
        let constraint_quotient = ((triple_xor_32_output_limb_0_col139
            - input_state_after_limb2_limb_0_col20));
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - Blake output h[2].high() matches expected
        let constraint_quotient = ((triple_xor_32_output_limb_1_col140
            - input_state_after_limb2_limb_1_col21));
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - Blake output h[3].low() matches expected
        let constraint_quotient = ((triple_xor_32_output_limb_0_col141
            - input_state_after_limb3_limb_0_col22));
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - Blake output h[3].high() matches expected
        let constraint_quotient = ((triple_xor_32_output_limb_1_col142
            - input_state_after_limb3_limb_1_col23));
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - Blake output h[4].low() matches expected
        let constraint_quotient = ((triple_xor_32_output_limb_0_col143
            - input_state_after_limb4_limb_0_col24));
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - Blake output h[4].high() matches expected
        let constraint_quotient = ((triple_xor_32_output_limb_1_col144
            - input_state_after_limb4_limb_1_col25));
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - Blake output h[5].low() matches expected
        let constraint_quotient = ((triple_xor_32_output_limb_0_col145
            - input_state_after_limb5_limb_0_col26));
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - Blake output h[5].high() matches expected
        let constraint_quotient = ((triple_xor_32_output_limb_1_col146
            - input_state_after_limb5_limb_1_col27));
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - Blake output h[6].low() matches expected
        let constraint_quotient = ((triple_xor_32_output_limb_0_col147
            - input_state_after_limb6_limb_0_col28));
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - Blake output h[6].high() matches expected
        let constraint_quotient = ((triple_xor_32_output_limb_1_col148
            - input_state_after_limb6_limb_1_col29));
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - Blake output h[7].low() matches expected
        let constraint_quotient = ((triple_xor_32_output_limb_0_col149
            - input_state_after_limb7_limb_0_col30));
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - Blake output h[7].high() matches expected
        let constraint_quotient = ((triple_xor_32_output_limb_1_col150
            - input_state_after_limb7_limb_1_col31));
        sum = sum * random_coeff + constraint_quotient;

        blake_output_sum_62 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<1061955672, 0, 0, 0>(), state_before_addr,
                    input_state_before_limb0_limb_0_col0, input_state_before_limb0_limb_1_col1,
                    input_state_before_limb1_limb_0_col2, input_state_before_limb1_limb_1_col3,
                    input_state_before_limb2_limb_0_col4, input_state_before_limb2_limb_1_col5,
                    input_state_before_limb3_limb_0_col6, input_state_before_limb3_limb_1_col7,
                    input_state_before_limb4_limb_0_col8, input_state_before_limb4_limb_1_col9,
                    input_state_before_limb5_limb_0_col10, input_state_before_limb5_limb_1_col11,
                    input_state_before_limb6_limb_0_col12, input_state_before_limb6_limb_1_col13,
                    input_state_before_limb7_limb_0_col14, input_state_before_limb7_limb_1_col15,
                ]
                    .span(),
            );
        numerator_62 = compress_enabler;

        blake_output_sum_63 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<1061955672, 0, 0, 0>(), state_after_addr,
                    input_state_after_limb0_limb_0_col16, input_state_after_limb0_limb_1_col17,
                    input_state_after_limb1_limb_0_col18, input_state_after_limb1_limb_1_col19,
                    input_state_after_limb2_limb_0_col20, input_state_after_limb2_limb_1_col21,
                    input_state_after_limb3_limb_0_col22, input_state_after_limb3_limb_1_col23,
                    input_state_after_limb4_limb_0_col24, input_state_after_limb4_limb_1_col25,
                    input_state_after_limb5_limb_0_col26, input_state_after_limb5_limb_1_col27,
                    input_state_after_limb6_limb_0_col28, input_state_after_limb6_limb_1_col29,
                    input_state_after_limb7_limb_0_col30, input_state_after_limb7_limb_1_col31,
                ]
                    .span(),
            );
        numerator_63 = compress_enabler;

        gate_sum_64 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<378353459, 0, 0, 0>(), message0_addr, input_message_limb0_col32,
                    input_message_limb1_col33, input_message_limb2_col34, input_message_limb3_col35,
                ]
                    .span(),
            );
        numerator_64 = compress_enabler;

        gate_sum_65 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<378353459, 0, 0, 0>(), message1_addr, input_message_limb4_col36,
                    input_message_limb5_col37, input_message_limb6_col38, input_message_limb7_col39,
                ]
                    .span(),
            );
        numerator_65 = compress_enabler;

        gate_sum_66 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<378353459, 0, 0, 0>(), message2_addr, input_message_limb8_col40,
                    input_message_limb9_col41, input_message_limb10_col42,
                    input_message_limb11_col43,
                ]
                    .span(),
            );
        numerator_66 = compress_enabler;

        gate_sum_67 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<378353459, 0, 0, 0>(), message3_addr, input_message_limb12_col44,
                    input_message_limb13_col45, input_message_limb14_col46,
                    input_message_limb15_col47,
                ]
                    .span(),
            );
        numerator_67 = compress_enabler;

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
            numerator_12,
            numerator_13,
            numerator_14,
            numerator_15,
            numerator_16,
            numerator_17,
            numerator_18,
            numerator_19,
            numerator_20,
            numerator_21,
            numerator_22,
            numerator_23,
            numerator_24,
            numerator_25,
            numerator_26,
            numerator_27,
            numerator_28,
            numerator_29,
            numerator_30,
            numerator_31,
            numerator_32,
            numerator_33,
            numerator_34,
            numerator_35,
            numerator_36,
            numerator_37,
            numerator_38,
            numerator_39,
            numerator_40,
            numerator_41,
            numerator_42,
            numerator_43,
            numerator_44,
            numerator_45,
            numerator_46,
            numerator_47,
            numerator_48,
            numerator_49,
            numerator_50,
            numerator_51,
            numerator_52,
            numerator_53,
            numerator_54,
            numerator_55,
            numerator_56,
            numerator_57,
            numerator_58,
            numerator_59,
            numerator_60,
            numerator_61,
            numerator_62,
            numerator_63,
            numerator_64,
            numerator_65,
            numerator_66,
            numerator_67,
            column_size,
            ref interaction_trace_mask_values,
            verify_bitwise_xor_8_sum_0,
            verify_bitwise_xor_8_sum_1,
            verify_bitwise_xor_8_sum_2,
            verify_bitwise_xor_8_sum_3,
            range_check_16_sum_4,
            range_check_15_sum_5,
            blake_message_sum_6,
            range_check_16_sum_7,
            range_check_15_sum_8,
            blake_message_sum_9,
            range_check_16_sum_10,
            range_check_15_sum_11,
            blake_message_sum_12,
            range_check_16_sum_13,
            range_check_15_sum_14,
            blake_message_sum_15,
            range_check_16_sum_16,
            range_check_15_sum_17,
            blake_message_sum_18,
            range_check_16_sum_19,
            range_check_15_sum_20,
            blake_message_sum_21,
            range_check_16_sum_22,
            range_check_15_sum_23,
            blake_message_sum_24,
            range_check_16_sum_25,
            range_check_15_sum_26,
            blake_message_sum_27,
            range_check_16_sum_28,
            range_check_15_sum_29,
            blake_message_sum_30,
            range_check_16_sum_31,
            range_check_15_sum_32,
            blake_message_sum_33,
            range_check_16_sum_34,
            range_check_15_sum_35,
            blake_message_sum_36,
            range_check_16_sum_37,
            range_check_15_sum_38,
            blake_message_sum_39,
            range_check_16_sum_40,
            range_check_15_sum_41,
            blake_message_sum_42,
            range_check_16_sum_43,
            range_check_15_sum_44,
            blake_message_sum_45,
            range_check_16_sum_46,
            range_check_15_sum_47,
            blake_message_sum_48,
            range_check_16_sum_49,
            range_check_15_sum_50,
            blake_message_sum_51,
            blake_round_sum_52,
            blake_round_sum_53,
            triple_xor_32_sum_54,
            triple_xor_32_sum_55,
            triple_xor_32_sum_56,
            triple_xor_32_sum_57,
            triple_xor_32_sum_58,
            triple_xor_32_sum_59,
            triple_xor_32_sum_60,
            triple_xor_32_sum_61,
            blake_output_sum_62,
            blake_output_sum_63,
            gate_sum_64,
            gate_sum_65,
            gate_sum_66,
            gate_sum_67,
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
    numerator_12: QM31,
    numerator_13: QM31,
    numerator_14: QM31,
    numerator_15: QM31,
    numerator_16: QM31,
    numerator_17: QM31,
    numerator_18: QM31,
    numerator_19: QM31,
    numerator_20: QM31,
    numerator_21: QM31,
    numerator_22: QM31,
    numerator_23: QM31,
    numerator_24: QM31,
    numerator_25: QM31,
    numerator_26: QM31,
    numerator_27: QM31,
    numerator_28: QM31,
    numerator_29: QM31,
    numerator_30: QM31,
    numerator_31: QM31,
    numerator_32: QM31,
    numerator_33: QM31,
    numerator_34: QM31,
    numerator_35: QM31,
    numerator_36: QM31,
    numerator_37: QM31,
    numerator_38: QM31,
    numerator_39: QM31,
    numerator_40: QM31,
    numerator_41: QM31,
    numerator_42: QM31,
    numerator_43: QM31,
    numerator_44: QM31,
    numerator_45: QM31,
    numerator_46: QM31,
    numerator_47: QM31,
    numerator_48: QM31,
    numerator_49: QM31,
    numerator_50: QM31,
    numerator_51: QM31,
    numerator_52: QM31,
    numerator_53: QM31,
    numerator_54: QM31,
    numerator_55: QM31,
    numerator_56: QM31,
    numerator_57: QM31,
    numerator_58: QM31,
    numerator_59: QM31,
    numerator_60: QM31,
    numerator_61: QM31,
    numerator_62: QM31,
    numerator_63: QM31,
    numerator_64: QM31,
    numerator_65: QM31,
    numerator_66: QM31,
    numerator_67: QM31,
    column_size: M31,
    ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
    verify_bitwise_xor_8_sum_0: QM31,
    verify_bitwise_xor_8_sum_1: QM31,
    verify_bitwise_xor_8_sum_2: QM31,
    verify_bitwise_xor_8_sum_3: QM31,
    range_check_16_sum_4: QM31,
    range_check_15_sum_5: QM31,
    blake_message_sum_6: QM31,
    range_check_16_sum_7: QM31,
    range_check_15_sum_8: QM31,
    blake_message_sum_9: QM31,
    range_check_16_sum_10: QM31,
    range_check_15_sum_11: QM31,
    blake_message_sum_12: QM31,
    range_check_16_sum_13: QM31,
    range_check_15_sum_14: QM31,
    blake_message_sum_15: QM31,
    range_check_16_sum_16: QM31,
    range_check_15_sum_17: QM31,
    blake_message_sum_18: QM31,
    range_check_16_sum_19: QM31,
    range_check_15_sum_20: QM31,
    blake_message_sum_21: QM31,
    range_check_16_sum_22: QM31,
    range_check_15_sum_23: QM31,
    blake_message_sum_24: QM31,
    range_check_16_sum_25: QM31,
    range_check_15_sum_26: QM31,
    blake_message_sum_27: QM31,
    range_check_16_sum_28: QM31,
    range_check_15_sum_29: QM31,
    blake_message_sum_30: QM31,
    range_check_16_sum_31: QM31,
    range_check_15_sum_32: QM31,
    blake_message_sum_33: QM31,
    range_check_16_sum_34: QM31,
    range_check_15_sum_35: QM31,
    blake_message_sum_36: QM31,
    range_check_16_sum_37: QM31,
    range_check_15_sum_38: QM31,
    blake_message_sum_39: QM31,
    range_check_16_sum_40: QM31,
    range_check_15_sum_41: QM31,
    blake_message_sum_42: QM31,
    range_check_16_sum_43: QM31,
    range_check_15_sum_44: QM31,
    blake_message_sum_45: QM31,
    range_check_16_sum_46: QM31,
    range_check_15_sum_47: QM31,
    blake_message_sum_48: QM31,
    range_check_16_sum_49: QM31,
    range_check_15_sum_50: QM31,
    blake_message_sum_51: QM31,
    blake_round_sum_52: QM31,
    blake_round_sum_53: QM31,
    triple_xor_32_sum_54: QM31,
    triple_xor_32_sum_55: QM31,
    triple_xor_32_sum_56: QM31,
    triple_xor_32_sum_57: QM31,
    triple_xor_32_sum_58: QM31,
    triple_xor_32_sum_59: QM31,
    triple_xor_32_sum_60: QM31,
    triple_xor_32_sum_61: QM31,
    blake_output_sum_62: QM31,
    blake_output_sum_63: QM31,
    gate_sum_64: QM31,
    gate_sum_65: QM31,
    gate_sum_66: QM31,
    gate_sum_67: QM31,
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
    ]: [Span<QM31>; 136] =
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
    let [trace_2_col132_neg1, trace_2_col132]: [QM31; 2] = (*trace_2_col132.try_into().unwrap())
        .unbox();
    let [trace_2_col133_neg1, trace_2_col133]: [QM31; 2] = (*trace_2_col133.try_into().unwrap())
        .unbox();
    let [trace_2_col134_neg1, trace_2_col134]: [QM31; 2] = (*trace_2_col134.try_into().unwrap())
        .unbox();
    let [trace_2_col135_neg1, trace_2_col135]: [QM31; 2] = (*trace_2_col135.try_into().unwrap())
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
        * range_check_16_sum_4
        * range_check_15_sum_5)
        - (range_check_16_sum_4 * numerator_5)
        - (range_check_15_sum_5 * numerator_4));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
    )
        - QM31Impl::from_partial_evals([trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11]))
        * blake_message_sum_6
        * range_check_16_sum_7)
        - (blake_message_sum_6 * numerator_7)
        + (range_check_16_sum_7 * numerator_6));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
        ))
        * range_check_15_sum_8
        * blake_message_sum_9)
        + (range_check_15_sum_8 * numerator_9)
        - (blake_message_sum_9 * numerator_8));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
        ))
        * range_check_16_sum_10
        * range_check_15_sum_11)
        - (range_check_16_sum_10 * numerator_11)
        - (range_check_15_sum_11 * numerator_10));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23],
        ))
        * blake_message_sum_12
        * range_check_16_sum_13)
        - (blake_message_sum_12 * numerator_13)
        + (range_check_16_sum_13 * numerator_12));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col28, trace_2_col29, trace_2_col30, trace_2_col31],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27],
        ))
        * range_check_15_sum_14
        * blake_message_sum_15)
        + (range_check_15_sum_14 * numerator_15)
        - (blake_message_sum_15 * numerator_14));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col32, trace_2_col33, trace_2_col34, trace_2_col35],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col28, trace_2_col29, trace_2_col30, trace_2_col31],
        ))
        * range_check_16_sum_16
        * range_check_15_sum_17)
        - (range_check_16_sum_16 * numerator_17)
        - (range_check_15_sum_17 * numerator_16));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col36, trace_2_col37, trace_2_col38, trace_2_col39],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col32, trace_2_col33, trace_2_col34, trace_2_col35],
        ))
        * blake_message_sum_18
        * range_check_16_sum_19)
        - (blake_message_sum_18 * numerator_19)
        + (range_check_16_sum_19 * numerator_18));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col40, trace_2_col41, trace_2_col42, trace_2_col43],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col36, trace_2_col37, trace_2_col38, trace_2_col39],
        ))
        * range_check_15_sum_20
        * blake_message_sum_21)
        + (range_check_15_sum_20 * numerator_21)
        - (blake_message_sum_21 * numerator_20));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col44, trace_2_col45, trace_2_col46, trace_2_col47],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col40, trace_2_col41, trace_2_col42, trace_2_col43],
        ))
        * range_check_16_sum_22
        * range_check_15_sum_23)
        - (range_check_16_sum_22 * numerator_23)
        - (range_check_15_sum_23 * numerator_22));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col48, trace_2_col49, trace_2_col50, trace_2_col51],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col44, trace_2_col45, trace_2_col46, trace_2_col47],
        ))
        * blake_message_sum_24
        * range_check_16_sum_25)
        - (blake_message_sum_24 * numerator_25)
        + (range_check_16_sum_25 * numerator_24));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col52, trace_2_col53, trace_2_col54, trace_2_col55],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col48, trace_2_col49, trace_2_col50, trace_2_col51],
        ))
        * range_check_15_sum_26
        * blake_message_sum_27)
        + (range_check_15_sum_26 * numerator_27)
        - (blake_message_sum_27 * numerator_26));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col56, trace_2_col57, trace_2_col58, trace_2_col59],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col52, trace_2_col53, trace_2_col54, trace_2_col55],
        ))
        * range_check_16_sum_28
        * range_check_15_sum_29)
        - (range_check_16_sum_28 * numerator_29)
        - (range_check_15_sum_29 * numerator_28));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col60, trace_2_col61, trace_2_col62, trace_2_col63],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col56, trace_2_col57, trace_2_col58, trace_2_col59],
        ))
        * blake_message_sum_30
        * range_check_16_sum_31)
        - (blake_message_sum_30 * numerator_31)
        + (range_check_16_sum_31 * numerator_30));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col64, trace_2_col65, trace_2_col66, trace_2_col67],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col60, trace_2_col61, trace_2_col62, trace_2_col63],
        ))
        * range_check_15_sum_32
        * blake_message_sum_33)
        + (range_check_15_sum_32 * numerator_33)
        - (blake_message_sum_33 * numerator_32));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col68, trace_2_col69, trace_2_col70, trace_2_col71],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col64, trace_2_col65, trace_2_col66, trace_2_col67],
        ))
        * range_check_16_sum_34
        * range_check_15_sum_35)
        - (range_check_16_sum_34 * numerator_35)
        - (range_check_15_sum_35 * numerator_34));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col72, trace_2_col73, trace_2_col74, trace_2_col75],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col68, trace_2_col69, trace_2_col70, trace_2_col71],
        ))
        * blake_message_sum_36
        * range_check_16_sum_37)
        - (blake_message_sum_36 * numerator_37)
        + (range_check_16_sum_37 * numerator_36));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col76, trace_2_col77, trace_2_col78, trace_2_col79],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col72, trace_2_col73, trace_2_col74, trace_2_col75],
        ))
        * range_check_15_sum_38
        * blake_message_sum_39)
        + (range_check_15_sum_38 * numerator_39)
        - (blake_message_sum_39 * numerator_38));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col80, trace_2_col81, trace_2_col82, trace_2_col83],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col76, trace_2_col77, trace_2_col78, trace_2_col79],
        ))
        * range_check_16_sum_40
        * range_check_15_sum_41)
        - (range_check_16_sum_40 * numerator_41)
        - (range_check_15_sum_41 * numerator_40));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col84, trace_2_col85, trace_2_col86, trace_2_col87],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col80, trace_2_col81, trace_2_col82, trace_2_col83],
        ))
        * blake_message_sum_42
        * range_check_16_sum_43)
        - (blake_message_sum_42 * numerator_43)
        + (range_check_16_sum_43 * numerator_42));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col88, trace_2_col89, trace_2_col90, trace_2_col91],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col84, trace_2_col85, trace_2_col86, trace_2_col87],
        ))
        * range_check_15_sum_44
        * blake_message_sum_45)
        + (range_check_15_sum_44 * numerator_45)
        - (blake_message_sum_45 * numerator_44));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col92, trace_2_col93, trace_2_col94, trace_2_col95],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col88, trace_2_col89, trace_2_col90, trace_2_col91],
        ))
        * range_check_16_sum_46
        * range_check_15_sum_47)
        - (range_check_16_sum_46 * numerator_47)
        - (range_check_15_sum_47 * numerator_46));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col96, trace_2_col97, trace_2_col98, trace_2_col99],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col92, trace_2_col93, trace_2_col94, trace_2_col95],
        ))
        * blake_message_sum_48
        * range_check_16_sum_49)
        - (blake_message_sum_48 * numerator_49)
        + (range_check_16_sum_49 * numerator_48));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col100, trace_2_col101, trace_2_col102, trace_2_col103],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col96, trace_2_col97, trace_2_col98, trace_2_col99],
        ))
        * range_check_15_sum_50
        * blake_message_sum_51)
        + (range_check_15_sum_50 * numerator_51)
        - (blake_message_sum_51 * numerator_50));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col104, trace_2_col105, trace_2_col106, trace_2_col107],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col100, trace_2_col101, trace_2_col102, trace_2_col103],
        ))
        * blake_round_sum_52
        * blake_round_sum_53)
        - (blake_round_sum_52 * numerator_53)
        + (blake_round_sum_53 * numerator_52));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col108, trace_2_col109, trace_2_col110, trace_2_col111],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col104, trace_2_col105, trace_2_col106, trace_2_col107],
        ))
        * triple_xor_32_sum_54
        * triple_xor_32_sum_55)
        - (triple_xor_32_sum_54 * numerator_55)
        - (triple_xor_32_sum_55 * numerator_54));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col112, trace_2_col113, trace_2_col114, trace_2_col115],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col108, trace_2_col109, trace_2_col110, trace_2_col111],
        ))
        * triple_xor_32_sum_56
        * triple_xor_32_sum_57)
        - (triple_xor_32_sum_56 * numerator_57)
        - (triple_xor_32_sum_57 * numerator_56));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col116, trace_2_col117, trace_2_col118, trace_2_col119],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col112, trace_2_col113, trace_2_col114, trace_2_col115],
        ))
        * triple_xor_32_sum_58
        * triple_xor_32_sum_59)
        - (triple_xor_32_sum_58 * numerator_59)
        - (triple_xor_32_sum_59 * numerator_58));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col120, trace_2_col121, trace_2_col122, trace_2_col123],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col116, trace_2_col117, trace_2_col118, trace_2_col119],
        ))
        * triple_xor_32_sum_60
        * triple_xor_32_sum_61)
        - (triple_xor_32_sum_60 * numerator_61)
        - (triple_xor_32_sum_61 * numerator_60));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col124, trace_2_col125, trace_2_col126, trace_2_col127],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col120, trace_2_col121, trace_2_col122, trace_2_col123],
        ))
        * blake_output_sum_62
        * blake_output_sum_63)
        + (blake_output_sum_62 * numerator_63)
        - (blake_output_sum_63 * numerator_62));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col128, trace_2_col129, trace_2_col130, trace_2_col131],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col124, trace_2_col125, trace_2_col126, trace_2_col127],
        ))
        * gate_sum_64
        * gate_sum_65)
        - (gate_sum_64 * numerator_65)
        - (gate_sum_65 * numerator_64));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col132, trace_2_col133, trace_2_col134, trace_2_col135],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col128, trace_2_col129, trace_2_col130, trace_2_col131],
        )
        - QM31Impl::from_partial_evals(
            [trace_2_col132_neg1, trace_2_col133_neg1, trace_2_col134_neg1, trace_2_col135_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * gate_sum_66
        * gate_sum_67)
        - (gate_sum_66 * numerator_67)
        - (gate_sum_67 * numerator_66));
    sum = sum * random_coeff + constraint_quotient;
}
