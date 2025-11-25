// This file was created by the AIR team.

use crate::components::subroutines::read_u_32::read_u_32_evaluate;
use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 212;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 6] = [
    ('BlakeRoundSigma', 1), ('RangeCheck_7_2_5', 16), ('MemoryAddressToId', 16),
    ('MemoryIdToBig', 16), ('BlakeG', 8), ('BlakeRound', 1),
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
        let interaction_log_sizes = [log_size; 120].span();
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
    pub blake_round_sigma_lookup_elements: crate::BlakeRoundSigmaElements,
    pub range_check_7_2_5_lookup_elements: crate::RangeCheck_7_2_5Elements,
    pub memory_address_to_id_lookup_elements: crate::MemoryAddressToIdElements,
    pub memory_id_to_big_lookup_elements: crate::MemoryIdToBigElements,
    pub blake_g_lookup_elements: crate::BlakeGElements,
    pub blake_round_lookup_elements: crate::BlakeRoundElements,
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
            blake_round_sigma_lookup_elements: interaction_elements.blake_round_sigma.clone(),
            range_check_7_2_5_lookup_elements: interaction_elements.range_checks.rc_7_2_5.clone(),
            memory_address_to_id_lookup_elements: interaction_elements.memory_address_to_id.clone(),
            memory_id_to_big_lookup_elements: interaction_elements.memory_id_to_value.clone(),
            blake_g_lookup_elements: interaction_elements.blake_g.clone(),
            blake_round_lookup_elements: interaction_elements.blake_round.clone(),
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
        let mut blake_round_sigma_sum_0: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_1: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_2: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_3: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_4: QM31 = Zero::zero();
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
        let mut range_check_7_2_5_sum_34: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_35: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_36: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_37: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_38: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_39: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_40: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_41: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_42: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_43: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_44: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_45: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_46: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_47: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_48: QM31 = Zero::zero();
        let mut blake_g_sum_49: QM31 = Zero::zero();
        let mut blake_g_sum_50: QM31 = Zero::zero();
        let mut blake_g_sum_51: QM31 = Zero::zero();
        let mut blake_g_sum_52: QM31 = Zero::zero();
        let mut blake_g_sum_53: QM31 = Zero::zero();
        let mut blake_g_sum_54: QM31 = Zero::zero();
        let mut blake_g_sum_55: QM31 = Zero::zero();
        let mut blake_g_sum_56: QM31 = Zero::zero();
        let mut blake_round_sum_57: QM31 = Zero::zero();
        let mut blake_round_sum_58: QM31 = Zero::zero();

        let [
            input_limb_0_col0,
            input_limb_1_col1,
            input_limb_2_col2,
            input_limb_3_col3,
            input_limb_4_col4,
            input_limb_5_col5,
            input_limb_6_col6,
            input_limb_7_col7,
            input_limb_8_col8,
            input_limb_9_col9,
            input_limb_10_col10,
            input_limb_11_col11,
            input_limb_12_col12,
            input_limb_13_col13,
            input_limb_14_col14,
            input_limb_15_col15,
            input_limb_16_col16,
            input_limb_17_col17,
            input_limb_18_col18,
            input_limb_19_col19,
            input_limb_20_col20,
            input_limb_21_col21,
            input_limb_22_col22,
            input_limb_23_col23,
            input_limb_24_col24,
            input_limb_25_col25,
            input_limb_26_col26,
            input_limb_27_col27,
            input_limb_28_col28,
            input_limb_29_col29,
            input_limb_30_col30,
            input_limb_31_col31,
            input_limb_32_col32,
            input_limb_33_col33,
            input_limb_34_col34,
            blake_round_sigma_output_limb_0_col35,
            blake_round_sigma_output_limb_1_col36,
            blake_round_sigma_output_limb_2_col37,
            blake_round_sigma_output_limb_3_col38,
            blake_round_sigma_output_limb_4_col39,
            blake_round_sigma_output_limb_5_col40,
            blake_round_sigma_output_limb_6_col41,
            blake_round_sigma_output_limb_7_col42,
            blake_round_sigma_output_limb_8_col43,
            blake_round_sigma_output_limb_9_col44,
            blake_round_sigma_output_limb_10_col45,
            blake_round_sigma_output_limb_11_col46,
            blake_round_sigma_output_limb_12_col47,
            blake_round_sigma_output_limb_13_col48,
            blake_round_sigma_output_limb_14_col49,
            blake_round_sigma_output_limb_15_col50,
            low_16_bits_col51,
            high_16_bits_col52,
            low_7_ms_bits_col53,
            high_14_ms_bits_col54,
            high_5_ms_bits_col55,
            message_word_0_id_col56,
            low_16_bits_col57,
            high_16_bits_col58,
            low_7_ms_bits_col59,
            high_14_ms_bits_col60,
            high_5_ms_bits_col61,
            message_word_1_id_col62,
            low_16_bits_col63,
            high_16_bits_col64,
            low_7_ms_bits_col65,
            high_14_ms_bits_col66,
            high_5_ms_bits_col67,
            message_word_2_id_col68,
            low_16_bits_col69,
            high_16_bits_col70,
            low_7_ms_bits_col71,
            high_14_ms_bits_col72,
            high_5_ms_bits_col73,
            message_word_3_id_col74,
            low_16_bits_col75,
            high_16_bits_col76,
            low_7_ms_bits_col77,
            high_14_ms_bits_col78,
            high_5_ms_bits_col79,
            message_word_4_id_col80,
            low_16_bits_col81,
            high_16_bits_col82,
            low_7_ms_bits_col83,
            high_14_ms_bits_col84,
            high_5_ms_bits_col85,
            message_word_5_id_col86,
            low_16_bits_col87,
            high_16_bits_col88,
            low_7_ms_bits_col89,
            high_14_ms_bits_col90,
            high_5_ms_bits_col91,
            message_word_6_id_col92,
            low_16_bits_col93,
            high_16_bits_col94,
            low_7_ms_bits_col95,
            high_14_ms_bits_col96,
            high_5_ms_bits_col97,
            message_word_7_id_col98,
            low_16_bits_col99,
            high_16_bits_col100,
            low_7_ms_bits_col101,
            high_14_ms_bits_col102,
            high_5_ms_bits_col103,
            message_word_8_id_col104,
            low_16_bits_col105,
            high_16_bits_col106,
            low_7_ms_bits_col107,
            high_14_ms_bits_col108,
            high_5_ms_bits_col109,
            message_word_9_id_col110,
            low_16_bits_col111,
            high_16_bits_col112,
            low_7_ms_bits_col113,
            high_14_ms_bits_col114,
            high_5_ms_bits_col115,
            message_word_10_id_col116,
            low_16_bits_col117,
            high_16_bits_col118,
            low_7_ms_bits_col119,
            high_14_ms_bits_col120,
            high_5_ms_bits_col121,
            message_word_11_id_col122,
            low_16_bits_col123,
            high_16_bits_col124,
            low_7_ms_bits_col125,
            high_14_ms_bits_col126,
            high_5_ms_bits_col127,
            message_word_12_id_col128,
            low_16_bits_col129,
            high_16_bits_col130,
            low_7_ms_bits_col131,
            high_14_ms_bits_col132,
            high_5_ms_bits_col133,
            message_word_13_id_col134,
            low_16_bits_col135,
            high_16_bits_col136,
            low_7_ms_bits_col137,
            high_14_ms_bits_col138,
            high_5_ms_bits_col139,
            message_word_14_id_col140,
            low_16_bits_col141,
            high_16_bits_col142,
            low_7_ms_bits_col143,
            high_14_ms_bits_col144,
            high_5_ms_bits_col145,
            message_word_15_id_col146,
            blake_g_output_limb_0_col147,
            blake_g_output_limb_1_col148,
            blake_g_output_limb_2_col149,
            blake_g_output_limb_3_col150,
            blake_g_output_limb_4_col151,
            blake_g_output_limb_5_col152,
            blake_g_output_limb_6_col153,
            blake_g_output_limb_7_col154,
            blake_g_output_limb_0_col155,
            blake_g_output_limb_1_col156,
            blake_g_output_limb_2_col157,
            blake_g_output_limb_3_col158,
            blake_g_output_limb_4_col159,
            blake_g_output_limb_5_col160,
            blake_g_output_limb_6_col161,
            blake_g_output_limb_7_col162,
            blake_g_output_limb_0_col163,
            blake_g_output_limb_1_col164,
            blake_g_output_limb_2_col165,
            blake_g_output_limb_3_col166,
            blake_g_output_limb_4_col167,
            blake_g_output_limb_5_col168,
            blake_g_output_limb_6_col169,
            blake_g_output_limb_7_col170,
            blake_g_output_limb_0_col171,
            blake_g_output_limb_1_col172,
            blake_g_output_limb_2_col173,
            blake_g_output_limb_3_col174,
            blake_g_output_limb_4_col175,
            blake_g_output_limb_5_col176,
            blake_g_output_limb_6_col177,
            blake_g_output_limb_7_col178,
            blake_g_output_limb_0_col179,
            blake_g_output_limb_1_col180,
            blake_g_output_limb_2_col181,
            blake_g_output_limb_3_col182,
            blake_g_output_limb_4_col183,
            blake_g_output_limb_5_col184,
            blake_g_output_limb_6_col185,
            blake_g_output_limb_7_col186,
            blake_g_output_limb_0_col187,
            blake_g_output_limb_1_col188,
            blake_g_output_limb_2_col189,
            blake_g_output_limb_3_col190,
            blake_g_output_limb_4_col191,
            blake_g_output_limb_5_col192,
            blake_g_output_limb_6_col193,
            blake_g_output_limb_7_col194,
            blake_g_output_limb_0_col195,
            blake_g_output_limb_1_col196,
            blake_g_output_limb_2_col197,
            blake_g_output_limb_3_col198,
            blake_g_output_limb_4_col199,
            blake_g_output_limb_5_col200,
            blake_g_output_limb_6_col201,
            blake_g_output_limb_7_col202,
            blake_g_output_limb_0_col203,
            blake_g_output_limb_1_col204,
            blake_g_output_limb_2_col205,
            blake_g_output_limb_3_col206,
            blake_g_output_limb_4_col207,
            blake_g_output_limb_5_col208,
            blake_g_output_limb_6_col209,
            blake_g_output_limb_7_col210,
            enabler,
        ]: [Span<QM31>; 212] =
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
        let [input_limb_6_col6]: [QM31; 1] = (*input_limb_6_col6.try_into().unwrap()).unbox();
        let [input_limb_7_col7]: [QM31; 1] = (*input_limb_7_col7.try_into().unwrap()).unbox();
        let [input_limb_8_col8]: [QM31; 1] = (*input_limb_8_col8.try_into().unwrap()).unbox();
        let [input_limb_9_col9]: [QM31; 1] = (*input_limb_9_col9.try_into().unwrap()).unbox();
        let [input_limb_10_col10]: [QM31; 1] = (*input_limb_10_col10.try_into().unwrap()).unbox();
        let [input_limb_11_col11]: [QM31; 1] = (*input_limb_11_col11.try_into().unwrap()).unbox();
        let [input_limb_12_col12]: [QM31; 1] = (*input_limb_12_col12.try_into().unwrap()).unbox();
        let [input_limb_13_col13]: [QM31; 1] = (*input_limb_13_col13.try_into().unwrap()).unbox();
        let [input_limb_14_col14]: [QM31; 1] = (*input_limb_14_col14.try_into().unwrap()).unbox();
        let [input_limb_15_col15]: [QM31; 1] = (*input_limb_15_col15.try_into().unwrap()).unbox();
        let [input_limb_16_col16]: [QM31; 1] = (*input_limb_16_col16.try_into().unwrap()).unbox();
        let [input_limb_17_col17]: [QM31; 1] = (*input_limb_17_col17.try_into().unwrap()).unbox();
        let [input_limb_18_col18]: [QM31; 1] = (*input_limb_18_col18.try_into().unwrap()).unbox();
        let [input_limb_19_col19]: [QM31; 1] = (*input_limb_19_col19.try_into().unwrap()).unbox();
        let [input_limb_20_col20]: [QM31; 1] = (*input_limb_20_col20.try_into().unwrap()).unbox();
        let [input_limb_21_col21]: [QM31; 1] = (*input_limb_21_col21.try_into().unwrap()).unbox();
        let [input_limb_22_col22]: [QM31; 1] = (*input_limb_22_col22.try_into().unwrap()).unbox();
        let [input_limb_23_col23]: [QM31; 1] = (*input_limb_23_col23.try_into().unwrap()).unbox();
        let [input_limb_24_col24]: [QM31; 1] = (*input_limb_24_col24.try_into().unwrap()).unbox();
        let [input_limb_25_col25]: [QM31; 1] = (*input_limb_25_col25.try_into().unwrap()).unbox();
        let [input_limb_26_col26]: [QM31; 1] = (*input_limb_26_col26.try_into().unwrap()).unbox();
        let [input_limb_27_col27]: [QM31; 1] = (*input_limb_27_col27.try_into().unwrap()).unbox();
        let [input_limb_28_col28]: [QM31; 1] = (*input_limb_28_col28.try_into().unwrap()).unbox();
        let [input_limb_29_col29]: [QM31; 1] = (*input_limb_29_col29.try_into().unwrap()).unbox();
        let [input_limb_30_col30]: [QM31; 1] = (*input_limb_30_col30.try_into().unwrap()).unbox();
        let [input_limb_31_col31]: [QM31; 1] = (*input_limb_31_col31.try_into().unwrap()).unbox();
        let [input_limb_32_col32]: [QM31; 1] = (*input_limb_32_col32.try_into().unwrap()).unbox();
        let [input_limb_33_col33]: [QM31; 1] = (*input_limb_33_col33.try_into().unwrap()).unbox();
        let [input_limb_34_col34]: [QM31; 1] = (*input_limb_34_col34.try_into().unwrap()).unbox();
        let [blake_round_sigma_output_limb_0_col35]: [QM31; 1] =
            (*blake_round_sigma_output_limb_0_col35
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_sigma_output_limb_1_col36]: [QM31; 1] =
            (*blake_round_sigma_output_limb_1_col36
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_sigma_output_limb_2_col37]: [QM31; 1] =
            (*blake_round_sigma_output_limb_2_col37
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_sigma_output_limb_3_col38]: [QM31; 1] =
            (*blake_round_sigma_output_limb_3_col38
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_sigma_output_limb_4_col39]: [QM31; 1] =
            (*blake_round_sigma_output_limb_4_col39
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_sigma_output_limb_5_col40]: [QM31; 1] =
            (*blake_round_sigma_output_limb_5_col40
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_sigma_output_limb_6_col41]: [QM31; 1] =
            (*blake_round_sigma_output_limb_6_col41
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_sigma_output_limb_7_col42]: [QM31; 1] =
            (*blake_round_sigma_output_limb_7_col42
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_sigma_output_limb_8_col43]: [QM31; 1] =
            (*blake_round_sigma_output_limb_8_col43
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_sigma_output_limb_9_col44]: [QM31; 1] =
            (*blake_round_sigma_output_limb_9_col44
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_sigma_output_limb_10_col45]: [QM31; 1] =
            (*blake_round_sigma_output_limb_10_col45
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_sigma_output_limb_11_col46]: [QM31; 1] =
            (*blake_round_sigma_output_limb_11_col46
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_sigma_output_limb_12_col47]: [QM31; 1] =
            (*blake_round_sigma_output_limb_12_col47
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_sigma_output_limb_13_col48]: [QM31; 1] =
            (*blake_round_sigma_output_limb_13_col48
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_sigma_output_limb_14_col49]: [QM31; 1] =
            (*blake_round_sigma_output_limb_14_col49
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_sigma_output_limb_15_col50]: [QM31; 1] =
            (*blake_round_sigma_output_limb_15_col50
            .try_into()
            .unwrap())
            .unbox();
        let [low_16_bits_col51]: [QM31; 1] = (*low_16_bits_col51.try_into().unwrap()).unbox();
        let [high_16_bits_col52]: [QM31; 1] = (*high_16_bits_col52.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col53]: [QM31; 1] = (*low_7_ms_bits_col53.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col54]: [QM31; 1] = (*high_14_ms_bits_col54.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col55]: [QM31; 1] = (*high_5_ms_bits_col55.try_into().unwrap()).unbox();
        let [message_word_0_id_col56]: [QM31; 1] = (*message_word_0_id_col56.try_into().unwrap())
            .unbox();
        let [low_16_bits_col57]: [QM31; 1] = (*low_16_bits_col57.try_into().unwrap()).unbox();
        let [high_16_bits_col58]: [QM31; 1] = (*high_16_bits_col58.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col59]: [QM31; 1] = (*low_7_ms_bits_col59.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col60]: [QM31; 1] = (*high_14_ms_bits_col60.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col61]: [QM31; 1] = (*high_5_ms_bits_col61.try_into().unwrap()).unbox();
        let [message_word_1_id_col62]: [QM31; 1] = (*message_word_1_id_col62.try_into().unwrap())
            .unbox();
        let [low_16_bits_col63]: [QM31; 1] = (*low_16_bits_col63.try_into().unwrap()).unbox();
        let [high_16_bits_col64]: [QM31; 1] = (*high_16_bits_col64.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col65]: [QM31; 1] = (*low_7_ms_bits_col65.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col66]: [QM31; 1] = (*high_14_ms_bits_col66.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col67]: [QM31; 1] = (*high_5_ms_bits_col67.try_into().unwrap()).unbox();
        let [message_word_2_id_col68]: [QM31; 1] = (*message_word_2_id_col68.try_into().unwrap())
            .unbox();
        let [low_16_bits_col69]: [QM31; 1] = (*low_16_bits_col69.try_into().unwrap()).unbox();
        let [high_16_bits_col70]: [QM31; 1] = (*high_16_bits_col70.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col71]: [QM31; 1] = (*low_7_ms_bits_col71.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col72]: [QM31; 1] = (*high_14_ms_bits_col72.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col73]: [QM31; 1] = (*high_5_ms_bits_col73.try_into().unwrap()).unbox();
        let [message_word_3_id_col74]: [QM31; 1] = (*message_word_3_id_col74.try_into().unwrap())
            .unbox();
        let [low_16_bits_col75]: [QM31; 1] = (*low_16_bits_col75.try_into().unwrap()).unbox();
        let [high_16_bits_col76]: [QM31; 1] = (*high_16_bits_col76.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col77]: [QM31; 1] = (*low_7_ms_bits_col77.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col78]: [QM31; 1] = (*high_14_ms_bits_col78.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col79]: [QM31; 1] = (*high_5_ms_bits_col79.try_into().unwrap()).unbox();
        let [message_word_4_id_col80]: [QM31; 1] = (*message_word_4_id_col80.try_into().unwrap())
            .unbox();
        let [low_16_bits_col81]: [QM31; 1] = (*low_16_bits_col81.try_into().unwrap()).unbox();
        let [high_16_bits_col82]: [QM31; 1] = (*high_16_bits_col82.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col83]: [QM31; 1] = (*low_7_ms_bits_col83.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col84]: [QM31; 1] = (*high_14_ms_bits_col84.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col85]: [QM31; 1] = (*high_5_ms_bits_col85.try_into().unwrap()).unbox();
        let [message_word_5_id_col86]: [QM31; 1] = (*message_word_5_id_col86.try_into().unwrap())
            .unbox();
        let [low_16_bits_col87]: [QM31; 1] = (*low_16_bits_col87.try_into().unwrap()).unbox();
        let [high_16_bits_col88]: [QM31; 1] = (*high_16_bits_col88.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col89]: [QM31; 1] = (*low_7_ms_bits_col89.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col90]: [QM31; 1] = (*high_14_ms_bits_col90.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col91]: [QM31; 1] = (*high_5_ms_bits_col91.try_into().unwrap()).unbox();
        let [message_word_6_id_col92]: [QM31; 1] = (*message_word_6_id_col92.try_into().unwrap())
            .unbox();
        let [low_16_bits_col93]: [QM31; 1] = (*low_16_bits_col93.try_into().unwrap()).unbox();
        let [high_16_bits_col94]: [QM31; 1] = (*high_16_bits_col94.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col95]: [QM31; 1] = (*low_7_ms_bits_col95.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col96]: [QM31; 1] = (*high_14_ms_bits_col96.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col97]: [QM31; 1] = (*high_5_ms_bits_col97.try_into().unwrap()).unbox();
        let [message_word_7_id_col98]: [QM31; 1] = (*message_word_7_id_col98.try_into().unwrap())
            .unbox();
        let [low_16_bits_col99]: [QM31; 1] = (*low_16_bits_col99.try_into().unwrap()).unbox();
        let [high_16_bits_col100]: [QM31; 1] = (*high_16_bits_col100.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col101]: [QM31; 1] = (*low_7_ms_bits_col101.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col102]: [QM31; 1] = (*high_14_ms_bits_col102.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col103]: [QM31; 1] = (*high_5_ms_bits_col103.try_into().unwrap())
            .unbox();
        let [message_word_8_id_col104]: [QM31; 1] = (*message_word_8_id_col104.try_into().unwrap())
            .unbox();
        let [low_16_bits_col105]: [QM31; 1] = (*low_16_bits_col105.try_into().unwrap()).unbox();
        let [high_16_bits_col106]: [QM31; 1] = (*high_16_bits_col106.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col107]: [QM31; 1] = (*low_7_ms_bits_col107.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col108]: [QM31; 1] = (*high_14_ms_bits_col108.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col109]: [QM31; 1] = (*high_5_ms_bits_col109.try_into().unwrap())
            .unbox();
        let [message_word_9_id_col110]: [QM31; 1] = (*message_word_9_id_col110.try_into().unwrap())
            .unbox();
        let [low_16_bits_col111]: [QM31; 1] = (*low_16_bits_col111.try_into().unwrap()).unbox();
        let [high_16_bits_col112]: [QM31; 1] = (*high_16_bits_col112.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col113]: [QM31; 1] = (*low_7_ms_bits_col113.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col114]: [QM31; 1] = (*high_14_ms_bits_col114.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col115]: [QM31; 1] = (*high_5_ms_bits_col115.try_into().unwrap())
            .unbox();
        let [message_word_10_id_col116]: [QM31; 1] = (*message_word_10_id_col116
            .try_into()
            .unwrap())
            .unbox();
        let [low_16_bits_col117]: [QM31; 1] = (*low_16_bits_col117.try_into().unwrap()).unbox();
        let [high_16_bits_col118]: [QM31; 1] = (*high_16_bits_col118.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col119]: [QM31; 1] = (*low_7_ms_bits_col119.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col120]: [QM31; 1] = (*high_14_ms_bits_col120.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col121]: [QM31; 1] = (*high_5_ms_bits_col121.try_into().unwrap())
            .unbox();
        let [message_word_11_id_col122]: [QM31; 1] = (*message_word_11_id_col122
            .try_into()
            .unwrap())
            .unbox();
        let [low_16_bits_col123]: [QM31; 1] = (*low_16_bits_col123.try_into().unwrap()).unbox();
        let [high_16_bits_col124]: [QM31; 1] = (*high_16_bits_col124.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col125]: [QM31; 1] = (*low_7_ms_bits_col125.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col126]: [QM31; 1] = (*high_14_ms_bits_col126.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col127]: [QM31; 1] = (*high_5_ms_bits_col127.try_into().unwrap())
            .unbox();
        let [message_word_12_id_col128]: [QM31; 1] = (*message_word_12_id_col128
            .try_into()
            .unwrap())
            .unbox();
        let [low_16_bits_col129]: [QM31; 1] = (*low_16_bits_col129.try_into().unwrap()).unbox();
        let [high_16_bits_col130]: [QM31; 1] = (*high_16_bits_col130.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col131]: [QM31; 1] = (*low_7_ms_bits_col131.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col132]: [QM31; 1] = (*high_14_ms_bits_col132.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col133]: [QM31; 1] = (*high_5_ms_bits_col133.try_into().unwrap())
            .unbox();
        let [message_word_13_id_col134]: [QM31; 1] = (*message_word_13_id_col134
            .try_into()
            .unwrap())
            .unbox();
        let [low_16_bits_col135]: [QM31; 1] = (*low_16_bits_col135.try_into().unwrap()).unbox();
        let [high_16_bits_col136]: [QM31; 1] = (*high_16_bits_col136.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col137]: [QM31; 1] = (*low_7_ms_bits_col137.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col138]: [QM31; 1] = (*high_14_ms_bits_col138.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col139]: [QM31; 1] = (*high_5_ms_bits_col139.try_into().unwrap())
            .unbox();
        let [message_word_14_id_col140]: [QM31; 1] = (*message_word_14_id_col140
            .try_into()
            .unwrap())
            .unbox();
        let [low_16_bits_col141]: [QM31; 1] = (*low_16_bits_col141.try_into().unwrap()).unbox();
        let [high_16_bits_col142]: [QM31; 1] = (*high_16_bits_col142.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col143]: [QM31; 1] = (*low_7_ms_bits_col143.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col144]: [QM31; 1] = (*high_14_ms_bits_col144.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col145]: [QM31; 1] = (*high_5_ms_bits_col145.try_into().unwrap())
            .unbox();
        let [message_word_15_id_col146]: [QM31; 1] = (*message_word_15_id_col146
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_0_col147]: [QM31; 1] = (*blake_g_output_limb_0_col147
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_1_col148]: [QM31; 1] = (*blake_g_output_limb_1_col148
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_2_col149]: [QM31; 1] = (*blake_g_output_limb_2_col149
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_3_col150]: [QM31; 1] = (*blake_g_output_limb_3_col150
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_4_col151]: [QM31; 1] = (*blake_g_output_limb_4_col151
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_5_col152]: [QM31; 1] = (*blake_g_output_limb_5_col152
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_6_col153]: [QM31; 1] = (*blake_g_output_limb_6_col153
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_7_col154]: [QM31; 1] = (*blake_g_output_limb_7_col154
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_0_col155]: [QM31; 1] = (*blake_g_output_limb_0_col155
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_1_col156]: [QM31; 1] = (*blake_g_output_limb_1_col156
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_2_col157]: [QM31; 1] = (*blake_g_output_limb_2_col157
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_3_col158]: [QM31; 1] = (*blake_g_output_limb_3_col158
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_4_col159]: [QM31; 1] = (*blake_g_output_limb_4_col159
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_5_col160]: [QM31; 1] = (*blake_g_output_limb_5_col160
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_6_col161]: [QM31; 1] = (*blake_g_output_limb_6_col161
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_7_col162]: [QM31; 1] = (*blake_g_output_limb_7_col162
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_0_col163]: [QM31; 1] = (*blake_g_output_limb_0_col163
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_1_col164]: [QM31; 1] = (*blake_g_output_limb_1_col164
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_2_col165]: [QM31; 1] = (*blake_g_output_limb_2_col165
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_3_col166]: [QM31; 1] = (*blake_g_output_limb_3_col166
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_4_col167]: [QM31; 1] = (*blake_g_output_limb_4_col167
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_5_col168]: [QM31; 1] = (*blake_g_output_limb_5_col168
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_6_col169]: [QM31; 1] = (*blake_g_output_limb_6_col169
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_7_col170]: [QM31; 1] = (*blake_g_output_limb_7_col170
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_0_col171]: [QM31; 1] = (*blake_g_output_limb_0_col171
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_1_col172]: [QM31; 1] = (*blake_g_output_limb_1_col172
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_2_col173]: [QM31; 1] = (*blake_g_output_limb_2_col173
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_3_col174]: [QM31; 1] = (*blake_g_output_limb_3_col174
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_4_col175]: [QM31; 1] = (*blake_g_output_limb_4_col175
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_5_col176]: [QM31; 1] = (*blake_g_output_limb_5_col176
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_6_col177]: [QM31; 1] = (*blake_g_output_limb_6_col177
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_7_col178]: [QM31; 1] = (*blake_g_output_limb_7_col178
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_0_col179]: [QM31; 1] = (*blake_g_output_limb_0_col179
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_1_col180]: [QM31; 1] = (*blake_g_output_limb_1_col180
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_2_col181]: [QM31; 1] = (*blake_g_output_limb_2_col181
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_3_col182]: [QM31; 1] = (*blake_g_output_limb_3_col182
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_4_col183]: [QM31; 1] = (*blake_g_output_limb_4_col183
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_5_col184]: [QM31; 1] = (*blake_g_output_limb_5_col184
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_6_col185]: [QM31; 1] = (*blake_g_output_limb_6_col185
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_7_col186]: [QM31; 1] = (*blake_g_output_limb_7_col186
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_0_col187]: [QM31; 1] = (*blake_g_output_limb_0_col187
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_1_col188]: [QM31; 1] = (*blake_g_output_limb_1_col188
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_2_col189]: [QM31; 1] = (*blake_g_output_limb_2_col189
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_3_col190]: [QM31; 1] = (*blake_g_output_limb_3_col190
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_4_col191]: [QM31; 1] = (*blake_g_output_limb_4_col191
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_5_col192]: [QM31; 1] = (*blake_g_output_limb_5_col192
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_6_col193]: [QM31; 1] = (*blake_g_output_limb_6_col193
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_7_col194]: [QM31; 1] = (*blake_g_output_limb_7_col194
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_0_col195]: [QM31; 1] = (*blake_g_output_limb_0_col195
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_1_col196]: [QM31; 1] = (*blake_g_output_limb_1_col196
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_2_col197]: [QM31; 1] = (*blake_g_output_limb_2_col197
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_3_col198]: [QM31; 1] = (*blake_g_output_limb_3_col198
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_4_col199]: [QM31; 1] = (*blake_g_output_limb_4_col199
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_5_col200]: [QM31; 1] = (*blake_g_output_limb_5_col200
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_6_col201]: [QM31; 1] = (*blake_g_output_limb_6_col201
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_7_col202]: [QM31; 1] = (*blake_g_output_limb_7_col202
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_0_col203]: [QM31; 1] = (*blake_g_output_limb_0_col203
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_1_col204]: [QM31; 1] = (*blake_g_output_limb_1_col204
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_2_col205]: [QM31; 1] = (*blake_g_output_limb_2_col205
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_3_col206]: [QM31; 1] = (*blake_g_output_limb_3_col206
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_4_col207]: [QM31; 1] = (*blake_g_output_limb_4_col207
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_5_col208]: [QM31; 1] = (*blake_g_output_limb_5_col208
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_6_col209]: [QM31; 1] = (*blake_g_output_limb_6_col209
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_7_col210]: [QM31; 1] = (*blake_g_output_limb_7_col210
            .try_into()
            .unwrap())
            .unbox();
        let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        let constraint_quotient = (enabler * enabler - enabler) * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        blake_round_sigma_sum_0 = self
            .blake_round_sigma_lookup_elements
            .combine_qm31(
                [
                    input_limb_1_col1, blake_round_sigma_output_limb_0_col35,
                    blake_round_sigma_output_limb_1_col36, blake_round_sigma_output_limb_2_col37,
                    blake_round_sigma_output_limb_3_col38, blake_round_sigma_output_limb_4_col39,
                    blake_round_sigma_output_limb_5_col40, blake_round_sigma_output_limb_6_col41,
                    blake_round_sigma_output_limb_7_col42, blake_round_sigma_output_limb_8_col43,
                    blake_round_sigma_output_limb_9_col44, blake_round_sigma_output_limb_10_col45,
                    blake_round_sigma_output_limb_11_col46, blake_round_sigma_output_limb_12_col47,
                    blake_round_sigma_output_limb_13_col48, blake_round_sigma_output_limb_14_col49,
                    blake_round_sigma_output_limb_15_col50,
                ],
            );
        read_u_32_evaluate(
            (input_limb_34_col34 + blake_round_sigma_output_limb_0_col35),
            low_16_bits_col51,
            high_16_bits_col52,
            low_7_ms_bits_col53,
            high_14_ms_bits_col54,
            high_5_ms_bits_col55,
            message_word_0_id_col56,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_1,
            ref memory_address_to_id_sum_2,
            ref memory_id_to_big_sum_3,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        read_u_32_evaluate(
            (input_limb_34_col34 + blake_round_sigma_output_limb_1_col36),
            low_16_bits_col57,
            high_16_bits_col58,
            low_7_ms_bits_col59,
            high_14_ms_bits_col60,
            high_5_ms_bits_col61,
            message_word_1_id_col62,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_4,
            ref memory_address_to_id_sum_5,
            ref memory_id_to_big_sum_6,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        read_u_32_evaluate(
            (input_limb_34_col34 + blake_round_sigma_output_limb_2_col37),
            low_16_bits_col63,
            high_16_bits_col64,
            low_7_ms_bits_col65,
            high_14_ms_bits_col66,
            high_5_ms_bits_col67,
            message_word_2_id_col68,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_7,
            ref memory_address_to_id_sum_8,
            ref memory_id_to_big_sum_9,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        read_u_32_evaluate(
            (input_limb_34_col34 + blake_round_sigma_output_limb_3_col38),
            low_16_bits_col69,
            high_16_bits_col70,
            low_7_ms_bits_col71,
            high_14_ms_bits_col72,
            high_5_ms_bits_col73,
            message_word_3_id_col74,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_10,
            ref memory_address_to_id_sum_11,
            ref memory_id_to_big_sum_12,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        read_u_32_evaluate(
            (input_limb_34_col34 + blake_round_sigma_output_limb_4_col39),
            low_16_bits_col75,
            high_16_bits_col76,
            low_7_ms_bits_col77,
            high_14_ms_bits_col78,
            high_5_ms_bits_col79,
            message_word_4_id_col80,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_13,
            ref memory_address_to_id_sum_14,
            ref memory_id_to_big_sum_15,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        read_u_32_evaluate(
            (input_limb_34_col34 + blake_round_sigma_output_limb_5_col40),
            low_16_bits_col81,
            high_16_bits_col82,
            low_7_ms_bits_col83,
            high_14_ms_bits_col84,
            high_5_ms_bits_col85,
            message_word_5_id_col86,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_16,
            ref memory_address_to_id_sum_17,
            ref memory_id_to_big_sum_18,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        read_u_32_evaluate(
            (input_limb_34_col34 + blake_round_sigma_output_limb_6_col41),
            low_16_bits_col87,
            high_16_bits_col88,
            low_7_ms_bits_col89,
            high_14_ms_bits_col90,
            high_5_ms_bits_col91,
            message_word_6_id_col92,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_19,
            ref memory_address_to_id_sum_20,
            ref memory_id_to_big_sum_21,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        read_u_32_evaluate(
            (input_limb_34_col34 + blake_round_sigma_output_limb_7_col42),
            low_16_bits_col93,
            high_16_bits_col94,
            low_7_ms_bits_col95,
            high_14_ms_bits_col96,
            high_5_ms_bits_col97,
            message_word_7_id_col98,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_22,
            ref memory_address_to_id_sum_23,
            ref memory_id_to_big_sum_24,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        read_u_32_evaluate(
            (input_limb_34_col34 + blake_round_sigma_output_limb_8_col43),
            low_16_bits_col99,
            high_16_bits_col100,
            low_7_ms_bits_col101,
            high_14_ms_bits_col102,
            high_5_ms_bits_col103,
            message_word_8_id_col104,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_25,
            ref memory_address_to_id_sum_26,
            ref memory_id_to_big_sum_27,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        read_u_32_evaluate(
            (input_limb_34_col34 + blake_round_sigma_output_limb_9_col44),
            low_16_bits_col105,
            high_16_bits_col106,
            low_7_ms_bits_col107,
            high_14_ms_bits_col108,
            high_5_ms_bits_col109,
            message_word_9_id_col110,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_28,
            ref memory_address_to_id_sum_29,
            ref memory_id_to_big_sum_30,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        read_u_32_evaluate(
            (input_limb_34_col34 + blake_round_sigma_output_limb_10_col45),
            low_16_bits_col111,
            high_16_bits_col112,
            low_7_ms_bits_col113,
            high_14_ms_bits_col114,
            high_5_ms_bits_col115,
            message_word_10_id_col116,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_31,
            ref memory_address_to_id_sum_32,
            ref memory_id_to_big_sum_33,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        read_u_32_evaluate(
            (input_limb_34_col34 + blake_round_sigma_output_limb_11_col46),
            low_16_bits_col117,
            high_16_bits_col118,
            low_7_ms_bits_col119,
            high_14_ms_bits_col120,
            high_5_ms_bits_col121,
            message_word_11_id_col122,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_34,
            ref memory_address_to_id_sum_35,
            ref memory_id_to_big_sum_36,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        read_u_32_evaluate(
            (input_limb_34_col34 + blake_round_sigma_output_limb_12_col47),
            low_16_bits_col123,
            high_16_bits_col124,
            low_7_ms_bits_col125,
            high_14_ms_bits_col126,
            high_5_ms_bits_col127,
            message_word_12_id_col128,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_37,
            ref memory_address_to_id_sum_38,
            ref memory_id_to_big_sum_39,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        read_u_32_evaluate(
            (input_limb_34_col34 + blake_round_sigma_output_limb_13_col48),
            low_16_bits_col129,
            high_16_bits_col130,
            low_7_ms_bits_col131,
            high_14_ms_bits_col132,
            high_5_ms_bits_col133,
            message_word_13_id_col134,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_40,
            ref memory_address_to_id_sum_41,
            ref memory_id_to_big_sum_42,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        read_u_32_evaluate(
            (input_limb_34_col34 + blake_round_sigma_output_limb_14_col49),
            low_16_bits_col135,
            high_16_bits_col136,
            low_7_ms_bits_col137,
            high_14_ms_bits_col138,
            high_5_ms_bits_col139,
            message_word_14_id_col140,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_43,
            ref memory_address_to_id_sum_44,
            ref memory_id_to_big_sum_45,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        read_u_32_evaluate(
            (input_limb_34_col34 + blake_round_sigma_output_limb_15_col50),
            low_16_bits_col141,
            high_16_bits_col142,
            low_7_ms_bits_col143,
            high_14_ms_bits_col144,
            high_5_ms_bits_col145,
            message_word_15_id_col146,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_46,
            ref memory_address_to_id_sum_47,
            ref memory_id_to_big_sum_48,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        blake_g_sum_49 = self
            .blake_g_lookup_elements
            .combine_qm31(
                [
                    input_limb_2_col2, input_limb_3_col3, input_limb_10_col10, input_limb_11_col11,
                    input_limb_18_col18, input_limb_19_col19, input_limb_26_col26,
                    input_limb_27_col27, low_16_bits_col51, high_16_bits_col52, low_16_bits_col57,
                    high_16_bits_col58, blake_g_output_limb_0_col147, blake_g_output_limb_1_col148,
                    blake_g_output_limb_2_col149, blake_g_output_limb_3_col150,
                    blake_g_output_limb_4_col151, blake_g_output_limb_5_col152,
                    blake_g_output_limb_6_col153, blake_g_output_limb_7_col154,
                ],
            );

        blake_g_sum_50 = self
            .blake_g_lookup_elements
            .combine_qm31(
                [
                    input_limb_4_col4, input_limb_5_col5, input_limb_12_col12, input_limb_13_col13,
                    input_limb_20_col20, input_limb_21_col21, input_limb_28_col28,
                    input_limb_29_col29, low_16_bits_col63, high_16_bits_col64, low_16_bits_col69,
                    high_16_bits_col70, blake_g_output_limb_0_col155, blake_g_output_limb_1_col156,
                    blake_g_output_limb_2_col157, blake_g_output_limb_3_col158,
                    blake_g_output_limb_4_col159, blake_g_output_limb_5_col160,
                    blake_g_output_limb_6_col161, blake_g_output_limb_7_col162,
                ],
            );

        blake_g_sum_51 = self
            .blake_g_lookup_elements
            .combine_qm31(
                [
                    input_limb_6_col6, input_limb_7_col7, input_limb_14_col14, input_limb_15_col15,
                    input_limb_22_col22, input_limb_23_col23, input_limb_30_col30,
                    input_limb_31_col31, low_16_bits_col75, high_16_bits_col76, low_16_bits_col81,
                    high_16_bits_col82, blake_g_output_limb_0_col163, blake_g_output_limb_1_col164,
                    blake_g_output_limb_2_col165, blake_g_output_limb_3_col166,
                    blake_g_output_limb_4_col167, blake_g_output_limb_5_col168,
                    blake_g_output_limb_6_col169, blake_g_output_limb_7_col170,
                ],
            );

        blake_g_sum_52 = self
            .blake_g_lookup_elements
            .combine_qm31(
                [
                    input_limb_8_col8, input_limb_9_col9, input_limb_16_col16, input_limb_17_col17,
                    input_limb_24_col24, input_limb_25_col25, input_limb_32_col32,
                    input_limb_33_col33, low_16_bits_col87, high_16_bits_col88, low_16_bits_col93,
                    high_16_bits_col94, blake_g_output_limb_0_col171, blake_g_output_limb_1_col172,
                    blake_g_output_limb_2_col173, blake_g_output_limb_3_col174,
                    blake_g_output_limb_4_col175, blake_g_output_limb_5_col176,
                    blake_g_output_limb_6_col177, blake_g_output_limb_7_col178,
                ],
            );

        blake_g_sum_53 = self
            .blake_g_lookup_elements
            .combine_qm31(
                [
                    blake_g_output_limb_0_col147, blake_g_output_limb_1_col148,
                    blake_g_output_limb_2_col157, blake_g_output_limb_3_col158,
                    blake_g_output_limb_4_col167, blake_g_output_limb_5_col168,
                    blake_g_output_limb_6_col177, blake_g_output_limb_7_col178, low_16_bits_col99,
                    high_16_bits_col100, low_16_bits_col105, high_16_bits_col106,
                    blake_g_output_limb_0_col179, blake_g_output_limb_1_col180,
                    blake_g_output_limb_2_col181, blake_g_output_limb_3_col182,
                    blake_g_output_limb_4_col183, blake_g_output_limb_5_col184,
                    blake_g_output_limb_6_col185, blake_g_output_limb_7_col186,
                ],
            );

        blake_g_sum_54 = self
            .blake_g_lookup_elements
            .combine_qm31(
                [
                    blake_g_output_limb_0_col155, blake_g_output_limb_1_col156,
                    blake_g_output_limb_2_col165, blake_g_output_limb_3_col166,
                    blake_g_output_limb_4_col175, blake_g_output_limb_5_col176,
                    blake_g_output_limb_6_col153, blake_g_output_limb_7_col154, low_16_bits_col111,
                    high_16_bits_col112, low_16_bits_col117, high_16_bits_col118,
                    blake_g_output_limb_0_col187, blake_g_output_limb_1_col188,
                    blake_g_output_limb_2_col189, blake_g_output_limb_3_col190,
                    blake_g_output_limb_4_col191, blake_g_output_limb_5_col192,
                    blake_g_output_limb_6_col193, blake_g_output_limb_7_col194,
                ],
            );

        blake_g_sum_55 = self
            .blake_g_lookup_elements
            .combine_qm31(
                [
                    blake_g_output_limb_0_col163, blake_g_output_limb_1_col164,
                    blake_g_output_limb_2_col173, blake_g_output_limb_3_col174,
                    blake_g_output_limb_4_col151, blake_g_output_limb_5_col152,
                    blake_g_output_limb_6_col161, blake_g_output_limb_7_col162, low_16_bits_col123,
                    high_16_bits_col124, low_16_bits_col129, high_16_bits_col130,
                    blake_g_output_limb_0_col195, blake_g_output_limb_1_col196,
                    blake_g_output_limb_2_col197, blake_g_output_limb_3_col198,
                    blake_g_output_limb_4_col199, blake_g_output_limb_5_col200,
                    blake_g_output_limb_6_col201, blake_g_output_limb_7_col202,
                ],
            );

        blake_g_sum_56 = self
            .blake_g_lookup_elements
            .combine_qm31(
                [
                    blake_g_output_limb_0_col171, blake_g_output_limb_1_col172,
                    blake_g_output_limb_2_col149, blake_g_output_limb_3_col150,
                    blake_g_output_limb_4_col159, blake_g_output_limb_5_col160,
                    blake_g_output_limb_6_col169, blake_g_output_limb_7_col170, low_16_bits_col135,
                    high_16_bits_col136, low_16_bits_col141, high_16_bits_col142,
                    blake_g_output_limb_0_col203, blake_g_output_limb_1_col204,
                    blake_g_output_limb_2_col205, blake_g_output_limb_3_col206,
                    blake_g_output_limb_4_col207, blake_g_output_limb_5_col208,
                    blake_g_output_limb_6_col209, blake_g_output_limb_7_col210,
                ],
            );

        blake_round_sum_57 = self
            .blake_round_lookup_elements
            .combine_qm31(
                [
                    input_limb_0_col0, input_limb_1_col1, input_limb_2_col2, input_limb_3_col3,
                    input_limb_4_col4, input_limb_5_col5, input_limb_6_col6, input_limb_7_col7,
                    input_limb_8_col8, input_limb_9_col9, input_limb_10_col10, input_limb_11_col11,
                    input_limb_12_col12, input_limb_13_col13, input_limb_14_col14,
                    input_limb_15_col15, input_limb_16_col16, input_limb_17_col17,
                    input_limb_18_col18, input_limb_19_col19, input_limb_20_col20,
                    input_limb_21_col21, input_limb_22_col22, input_limb_23_col23,
                    input_limb_24_col24, input_limb_25_col25, input_limb_26_col26,
                    input_limb_27_col27, input_limb_28_col28, input_limb_29_col29,
                    input_limb_30_col30, input_limb_31_col31, input_limb_32_col32,
                    input_limb_33_col33, input_limb_34_col34,
                ],
            );

        blake_round_sum_58 = self
            .blake_round_lookup_elements
            .combine_qm31(
                [
                    input_limb_0_col0, (input_limb_1_col1 + qm31_const::<1, 0, 0, 0>()),
                    blake_g_output_limb_0_col179, blake_g_output_limb_1_col180,
                    blake_g_output_limb_0_col187, blake_g_output_limb_1_col188,
                    blake_g_output_limb_0_col195, blake_g_output_limb_1_col196,
                    blake_g_output_limb_0_col203, blake_g_output_limb_1_col204,
                    blake_g_output_limb_2_col205, blake_g_output_limb_3_col206,
                    blake_g_output_limb_2_col181, blake_g_output_limb_3_col182,
                    blake_g_output_limb_2_col189, blake_g_output_limb_3_col190,
                    blake_g_output_limb_2_col197, blake_g_output_limb_3_col198,
                    blake_g_output_limb_4_col199, blake_g_output_limb_5_col200,
                    blake_g_output_limb_4_col207, blake_g_output_limb_5_col208,
                    blake_g_output_limb_4_col183, blake_g_output_limb_5_col184,
                    blake_g_output_limb_4_col191, blake_g_output_limb_5_col192,
                    blake_g_output_limb_6_col193, blake_g_output_limb_7_col194,
                    blake_g_output_limb_6_col201, blake_g_output_limb_7_col202,
                    blake_g_output_limb_6_col209, blake_g_output_limb_7_col210,
                    blake_g_output_limb_6_col185, blake_g_output_limb_7_col186, input_limb_34_col34,
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
            blake_round_sigma_sum_0,
            range_check_7_2_5_sum_1,
            memory_address_to_id_sum_2,
            memory_id_to_big_sum_3,
            range_check_7_2_5_sum_4,
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
            range_check_7_2_5_sum_34,
            memory_address_to_id_sum_35,
            memory_id_to_big_sum_36,
            range_check_7_2_5_sum_37,
            memory_address_to_id_sum_38,
            memory_id_to_big_sum_39,
            range_check_7_2_5_sum_40,
            memory_address_to_id_sum_41,
            memory_id_to_big_sum_42,
            range_check_7_2_5_sum_43,
            memory_address_to_id_sum_44,
            memory_id_to_big_sum_45,
            range_check_7_2_5_sum_46,
            memory_address_to_id_sum_47,
            memory_id_to_big_sum_48,
            blake_g_sum_49,
            blake_g_sum_50,
            blake_g_sum_51,
            blake_g_sum_52,
            blake_g_sum_53,
            blake_g_sum_54,
            blake_g_sum_55,
            blake_g_sum_56,
            blake_round_sum_57,
            blake_round_sum_58,
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
    blake_round_sigma_sum_0: QM31,
    range_check_7_2_5_sum_1: QM31,
    memory_address_to_id_sum_2: QM31,
    memory_id_to_big_sum_3: QM31,
    range_check_7_2_5_sum_4: QM31,
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
    range_check_7_2_5_sum_34: QM31,
    memory_address_to_id_sum_35: QM31,
    memory_id_to_big_sum_36: QM31,
    range_check_7_2_5_sum_37: QM31,
    memory_address_to_id_sum_38: QM31,
    memory_id_to_big_sum_39: QM31,
    range_check_7_2_5_sum_40: QM31,
    memory_address_to_id_sum_41: QM31,
    memory_id_to_big_sum_42: QM31,
    range_check_7_2_5_sum_43: QM31,
    memory_address_to_id_sum_44: QM31,
    memory_id_to_big_sum_45: QM31,
    range_check_7_2_5_sum_46: QM31,
    memory_address_to_id_sum_47: QM31,
    memory_id_to_big_sum_48: QM31,
    blake_g_sum_49: QM31,
    blake_g_sum_50: QM31,
    blake_g_sum_51: QM31,
    blake_g_sum_52: QM31,
    blake_g_sum_53: QM31,
    blake_g_sum_54: QM31,
    blake_g_sum_55: QM31,
    blake_g_sum_56: QM31,
    blake_round_sum_57: QM31,
    blake_round_sum_58: QM31,
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
    ]: [Span<QM31>; 120] =
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
    let [trace_2_col116_neg1, trace_2_col116]: [QM31; 2] = (*trace_2_col116.try_into().unwrap())
        .unbox();
    let [trace_2_col117_neg1, trace_2_col117]: [QM31; 2] = (*trace_2_col117.try_into().unwrap())
        .unbox();
    let [trace_2_col118_neg1, trace_2_col118]: [QM31; 2] = (*trace_2_col118.try_into().unwrap())
        .unbox();
    let [trace_2_col119_neg1, trace_2_col119]: [QM31; 2] = (*trace_2_col119.try_into().unwrap())
        .unbox();

    core::internal::revoke_ap_tracking();

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3],
    ))
        * blake_round_sigma_sum_0
        * range_check_7_2_5_sum_1)
        - blake_round_sigma_sum_0
        - range_check_7_2_5_sum_1)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7],
    )
        - QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3]))
        * memory_address_to_id_sum_2
        * memory_id_to_big_sum_3)
        - memory_address_to_id_sum_2
        - memory_id_to_big_sum_3)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11],
    )
        - QM31Impl::from_partial_evals([trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7]))
        * range_check_7_2_5_sum_4
        * memory_address_to_id_sum_5)
        - range_check_7_2_5_sum_4
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
        * range_check_7_2_5_sum_34
        * memory_address_to_id_sum_35)
        - range_check_7_2_5_sum_34
        - memory_address_to_id_sum_35)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col72, trace_2_col73, trace_2_col74, trace_2_col75],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col68, trace_2_col69, trace_2_col70, trace_2_col71],
        ))
        * memory_id_to_big_sum_36
        * range_check_7_2_5_sum_37)
        - memory_id_to_big_sum_36
        - range_check_7_2_5_sum_37)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col76, trace_2_col77, trace_2_col78, trace_2_col79],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col72, trace_2_col73, trace_2_col74, trace_2_col75],
        ))
        * memory_address_to_id_sum_38
        * memory_id_to_big_sum_39)
        - memory_address_to_id_sum_38
        - memory_id_to_big_sum_39)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col80, trace_2_col81, trace_2_col82, trace_2_col83],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col76, trace_2_col77, trace_2_col78, trace_2_col79],
        ))
        * range_check_7_2_5_sum_40
        * memory_address_to_id_sum_41)
        - range_check_7_2_5_sum_40
        - memory_address_to_id_sum_41)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col84, trace_2_col85, trace_2_col86, trace_2_col87],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col80, trace_2_col81, trace_2_col82, trace_2_col83],
        ))
        * memory_id_to_big_sum_42
        * range_check_7_2_5_sum_43)
        - memory_id_to_big_sum_42
        - range_check_7_2_5_sum_43)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col88, trace_2_col89, trace_2_col90, trace_2_col91],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col84, trace_2_col85, trace_2_col86, trace_2_col87],
        ))
        * memory_address_to_id_sum_44
        * memory_id_to_big_sum_45)
        - memory_address_to_id_sum_44
        - memory_id_to_big_sum_45)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col92, trace_2_col93, trace_2_col94, trace_2_col95],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col88, trace_2_col89, trace_2_col90, trace_2_col91],
        ))
        * range_check_7_2_5_sum_46
        * memory_address_to_id_sum_47)
        - range_check_7_2_5_sum_46
        - memory_address_to_id_sum_47)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col96, trace_2_col97, trace_2_col98, trace_2_col99],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col92, trace_2_col93, trace_2_col94, trace_2_col95],
        ))
        * memory_id_to_big_sum_48
        * blake_g_sum_49)
        - memory_id_to_big_sum_48
        - blake_g_sum_49)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col100, trace_2_col101, trace_2_col102, trace_2_col103],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col96, trace_2_col97, trace_2_col98, trace_2_col99],
        ))
        * blake_g_sum_50
        * blake_g_sum_51)
        - blake_g_sum_50
        - blake_g_sum_51)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col104, trace_2_col105, trace_2_col106, trace_2_col107],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col100, trace_2_col101, trace_2_col102, trace_2_col103],
        ))
        * blake_g_sum_52
        * blake_g_sum_53)
        - blake_g_sum_52
        - blake_g_sum_53)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col108, trace_2_col109, trace_2_col110, trace_2_col111],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col104, trace_2_col105, trace_2_col106, trace_2_col107],
        ))
        * blake_g_sum_54
        * blake_g_sum_55)
        - blake_g_sum_54
        - blake_g_sum_55)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col112, trace_2_col113, trace_2_col114, trace_2_col115],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col108, trace_2_col109, trace_2_col110, trace_2_col111],
        ))
        * blake_g_sum_56
        * blake_round_sum_57)
        - (blake_g_sum_56 * enabler)
        - blake_round_sum_57)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col116, trace_2_col117, trace_2_col118, trace_2_col119],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col112, trace_2_col113, trace_2_col114, trace_2_col115],
        )
        - QM31Impl::from_partial_evals(
            [trace_2_col116_neg1, trace_2_col117_neg1, trace_2_col118_neg1, trace_2_col119_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * blake_round_sum_58)
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
            blake_g_lookup_elements: make_lookup_elements(
                qm31_const::<1303027045, 1098741784, 1663692553, 948339060>(),
                qm31_const::<435770977, 566354259, 805606465, 2102625819>(),
            ),
            blake_round_lookup_elements: make_lookup_elements(
                qm31_const::<512137121, 4656726, 184578687, 642917762>(),
                qm31_const::<1958399945, 1971391524, 790352857, 78661490>(),
            ),
            blake_round_sigma_lookup_elements: make_lookup_elements(
                qm31_const::<1575475957, 47612785, 1451790004, 55975591>(),
                qm31_const::<1683908581, 578045618, 2096573802, 1109732223>(),
            ),
            memory_address_to_id_lookup_elements: make_lookup_elements(
                qm31_const::<1842771211, 1960835386, 1582137647, 1333140033>(),
                qm31_const::<1360491305, 950648792, 556642685, 2096522554>(),
            ),
            memory_id_to_big_lookup_elements: make_lookup_elements(
                qm31_const::<844624398, 1166453613, 1247584074, 330174372>(),
                qm31_const::<1844105245, 1400976933, 1126903288, 1155460729>(),
            ),
            range_check_7_2_5_lookup_elements: make_lookup_elements(
                qm31_const::<425514336, 1473331321, 384012983, 274885242>(),
                qm31_const::<660930654, 31738603, 1176905988, 765990201>(),
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
            [qm31_const::<2078697440, 2012789179, 1936831203, 502514153>()].span(),
            [qm31_const::<132541330, 267958717, 2138157796, 502514153>()].span(),
            [qm31_const::<65432151, 133740989, 2071048932, 502514153>()].span(),
            [qm31_const::<266759688, 536394173, 124891877, 502514154>()].span(),
            [qm31_const::<199650509, 402176445, 57783013, 502514154>()].span(),
            [qm31_const::<1474714829, 804829627, 1332851427, 502514153>()].span(),
            [qm31_const::<1407605650, 670611899, 1265742563, 502514153>()].span(),
            [qm31_const::<266042923, 536393753, 124891457, 502514014>()].span(),
            [qm31_const::<333152102, 670611481, 192000321, 502514014>()].span(),
            [qm31_const::<400261281, 804829209, 259109185, 502514014>()].span(),
            [qm31_const::<467370460, 939046937, 326218049, 502514014>()].span(),
            [qm31_const::<2145089854, 2147006487, 2003939647, 502514013>()].span(),
            [qm31_const::<64715386, 133740569, 2071048512, 502514013>()].span(),
            [qm31_const::<131824565, 267958297, 2138157376, 502514013>()].span(),
            [qm31_const::<198933744, 402176025, 57782593, 502514014>()].span(),
            [qm31_const::<1876653138, 1610135575, 1735504191, 502514013>()].span(),
            [qm31_const::<1943762317, 1744353303, 1802613055, 502514013>()].span(),
            [qm31_const::<1742537175, 1341700179, 1601286523, 502514033>()].span(),
            [qm31_const::<1675427996, 1207482451, 1534177659, 502514033>()].span(),
            [qm31_const::<1608318817, 1073264723, 1467068795, 502514033>()].span(),
            [qm31_const::<1541209638, 939046995, 1399959931, 502514033>()].span(),
            [qm31_const::<1474100459, 804829267, 1332851067, 502514033>()].span(),
            [qm31_const::<1406991280, 670611539, 1265742203, 502514033>()].span(),
            [qm31_const::<1339882101, 536393811, 1198633339, 502514033>()].span(),
            [qm31_const::<1272772922, 402176083, 1131524475, 502514033>()].span(),
            [qm31_const::<131926960, 267958357, 2138157436, 502514033>()].span(),
            [qm31_const::<64817781, 133740629, 2071048572, 502514033>()].span(),
            [qm31_const::<1491955610, 670690004, 1265820668, 502540188>()].span(),
            [qm31_const::<1424846431, 536472276, 1198711804, 502540188>()].span(),
            [qm31_const::<1357737252, 402254548, 1131602940, 502540188>()].span(),
            [qm31_const::<1290628073, 268036820, 1064494076, 502540188>()].span(),
            [qm31_const::<1223518894, 133819092, 997385212, 502540188>()].span(),
            [qm31_const::<1156409715, 2147085011, 930276347, 502540188>()].span(),
            [qm31_const::<1089300536, 2012867283, 863167483, 502540188>()].span(),
            [qm31_const::<1022191357, 1878649555, 796058619, 502540188>()].span(),
            [qm31_const::<955082178, 1744431827, 728949755, 502540188>()].span(),
            [qm31_const::<887972999, 1610214099, 661840891, 502540188>()].span(),
            [qm31_const::<15491601, 2012867234, 1936909257, 502540171>()].span(),
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
        assert_eq!(sum, QM31Trait::from_fixed_array(BLAKE_ROUND_SAMPLE_EVAL_RESULT))
    }
}
