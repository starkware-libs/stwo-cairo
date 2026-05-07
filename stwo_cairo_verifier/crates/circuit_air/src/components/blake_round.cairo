// This file was created by the AIR team.

use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 148;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 4] = [
    ('BlakeRoundSigma', 1), ('BlakeMessage', 16), ('BlakeG', 8), ('BlakeRound', 1),
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
        let interaction_log_sizes = [log_size; 56].span();
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
        let mut blake_round_sigma_sum_0: QM31 = Zero::zero();
        let mut numerator_0: QM31 = Zero::zero();
        let mut blake_message_sum_1: QM31 = Zero::zero();
        let mut numerator_1: QM31 = Zero::zero();
        let mut blake_message_sum_2: QM31 = Zero::zero();
        let mut numerator_2: QM31 = Zero::zero();
        let mut blake_message_sum_3: QM31 = Zero::zero();
        let mut numerator_3: QM31 = Zero::zero();
        let mut blake_message_sum_4: QM31 = Zero::zero();
        let mut numerator_4: QM31 = Zero::zero();
        let mut blake_message_sum_5: QM31 = Zero::zero();
        let mut numerator_5: QM31 = Zero::zero();
        let mut blake_message_sum_6: QM31 = Zero::zero();
        let mut numerator_6: QM31 = Zero::zero();
        let mut blake_message_sum_7: QM31 = Zero::zero();
        let mut numerator_7: QM31 = Zero::zero();
        let mut blake_message_sum_8: QM31 = Zero::zero();
        let mut numerator_8: QM31 = Zero::zero();
        let mut blake_message_sum_9: QM31 = Zero::zero();
        let mut numerator_9: QM31 = Zero::zero();
        let mut blake_message_sum_10: QM31 = Zero::zero();
        let mut numerator_10: QM31 = Zero::zero();
        let mut blake_message_sum_11: QM31 = Zero::zero();
        let mut numerator_11: QM31 = Zero::zero();
        let mut blake_message_sum_12: QM31 = Zero::zero();
        let mut numerator_12: QM31 = Zero::zero();
        let mut blake_message_sum_13: QM31 = Zero::zero();
        let mut numerator_13: QM31 = Zero::zero();
        let mut blake_message_sum_14: QM31 = Zero::zero();
        let mut numerator_14: QM31 = Zero::zero();
        let mut blake_message_sum_15: QM31 = Zero::zero();
        let mut numerator_15: QM31 = Zero::zero();
        let mut blake_message_sum_16: QM31 = Zero::zero();
        let mut numerator_16: QM31 = Zero::zero();
        let mut blake_g_sum_17: QM31 = Zero::zero();
        let mut numerator_17: QM31 = Zero::zero();
        let mut blake_g_sum_18: QM31 = Zero::zero();
        let mut numerator_18: QM31 = Zero::zero();
        let mut blake_g_sum_19: QM31 = Zero::zero();
        let mut numerator_19: QM31 = Zero::zero();
        let mut blake_g_sum_20: QM31 = Zero::zero();
        let mut numerator_20: QM31 = Zero::zero();
        let mut blake_g_sum_21: QM31 = Zero::zero();
        let mut numerator_21: QM31 = Zero::zero();
        let mut blake_g_sum_22: QM31 = Zero::zero();
        let mut numerator_22: QM31 = Zero::zero();
        let mut blake_g_sum_23: QM31 = Zero::zero();
        let mut numerator_23: QM31 = Zero::zero();
        let mut blake_g_sum_24: QM31 = Zero::zero();
        let mut numerator_24: QM31 = Zero::zero();
        let mut blake_round_sum_25: QM31 = Zero::zero();
        let mut numerator_25: QM31 = Zero::zero();
        let mut blake_round_sum_26: QM31 = Zero::zero();
        let mut numerator_26: QM31 = Zero::zero();

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
            blake_message_output_message_limb_limb_0_col51,
            blake_message_output_message_limb_limb_1_col52,
            blake_message_output_message_limb_limb_0_col53,
            blake_message_output_message_limb_limb_1_col54,
            blake_message_output_message_limb_limb_0_col55,
            blake_message_output_message_limb_limb_1_col56,
            blake_message_output_message_limb_limb_0_col57,
            blake_message_output_message_limb_limb_1_col58,
            blake_message_output_message_limb_limb_0_col59,
            blake_message_output_message_limb_limb_1_col60,
            blake_message_output_message_limb_limb_0_col61,
            blake_message_output_message_limb_limb_1_col62,
            blake_message_output_message_limb_limb_0_col63,
            blake_message_output_message_limb_limb_1_col64,
            blake_message_output_message_limb_limb_0_col65,
            blake_message_output_message_limb_limb_1_col66,
            blake_message_output_message_limb_limb_0_col67,
            blake_message_output_message_limb_limb_1_col68,
            blake_message_output_message_limb_limb_0_col69,
            blake_message_output_message_limb_limb_1_col70,
            blake_message_output_message_limb_limb_0_col71,
            blake_message_output_message_limb_limb_1_col72,
            blake_message_output_message_limb_limb_0_col73,
            blake_message_output_message_limb_limb_1_col74,
            blake_message_output_message_limb_limb_0_col75,
            blake_message_output_message_limb_limb_1_col76,
            blake_message_output_message_limb_limb_0_col77,
            blake_message_output_message_limb_limb_1_col78,
            blake_message_output_message_limb_limb_0_col79,
            blake_message_output_message_limb_limb_1_col80,
            blake_message_output_message_limb_limb_0_col81,
            blake_message_output_message_limb_limb_1_col82,
            blake_g_output_limb_0_col83,
            blake_g_output_limb_1_col84,
            blake_g_output_limb_2_col85,
            blake_g_output_limb_3_col86,
            blake_g_output_limb_4_col87,
            blake_g_output_limb_5_col88,
            blake_g_output_limb_6_col89,
            blake_g_output_limb_7_col90,
            blake_g_output_limb_0_col91,
            blake_g_output_limb_1_col92,
            blake_g_output_limb_2_col93,
            blake_g_output_limb_3_col94,
            blake_g_output_limb_4_col95,
            blake_g_output_limb_5_col96,
            blake_g_output_limb_6_col97,
            blake_g_output_limb_7_col98,
            blake_g_output_limb_0_col99,
            blake_g_output_limb_1_col100,
            blake_g_output_limb_2_col101,
            blake_g_output_limb_3_col102,
            blake_g_output_limb_4_col103,
            blake_g_output_limb_5_col104,
            blake_g_output_limb_6_col105,
            blake_g_output_limb_7_col106,
            blake_g_output_limb_0_col107,
            blake_g_output_limb_1_col108,
            blake_g_output_limb_2_col109,
            blake_g_output_limb_3_col110,
            blake_g_output_limb_4_col111,
            blake_g_output_limb_5_col112,
            blake_g_output_limb_6_col113,
            blake_g_output_limb_7_col114,
            blake_g_output_limb_0_col115,
            blake_g_output_limb_1_col116,
            blake_g_output_limb_2_col117,
            blake_g_output_limb_3_col118,
            blake_g_output_limb_4_col119,
            blake_g_output_limb_5_col120,
            blake_g_output_limb_6_col121,
            blake_g_output_limb_7_col122,
            blake_g_output_limb_0_col123,
            blake_g_output_limb_1_col124,
            blake_g_output_limb_2_col125,
            blake_g_output_limb_3_col126,
            blake_g_output_limb_4_col127,
            blake_g_output_limb_5_col128,
            blake_g_output_limb_6_col129,
            blake_g_output_limb_7_col130,
            blake_g_output_limb_0_col131,
            blake_g_output_limb_1_col132,
            blake_g_output_limb_2_col133,
            blake_g_output_limb_3_col134,
            blake_g_output_limb_4_col135,
            blake_g_output_limb_5_col136,
            blake_g_output_limb_6_col137,
            blake_g_output_limb_7_col138,
            blake_g_output_limb_0_col139,
            blake_g_output_limb_1_col140,
            blake_g_output_limb_2_col141,
            blake_g_output_limb_3_col142,
            blake_g_output_limb_4_col143,
            blake_g_output_limb_5_col144,
            blake_g_output_limb_6_col145,
            blake_g_output_limb_7_col146,
            enabler_col147,
        ]: [Span<QM31>; 148] =
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
        let [blake_message_output_message_limb_limb_0_col51]: [QM31; 1] =
            (*blake_message_output_message_limb_limb_0_col51
            .try_into()
            .unwrap())
            .unbox();
        let [blake_message_output_message_limb_limb_1_col52]: [QM31; 1] =
            (*blake_message_output_message_limb_limb_1_col52
            .try_into()
            .unwrap())
            .unbox();
        let [blake_message_output_message_limb_limb_0_col53]: [QM31; 1] =
            (*blake_message_output_message_limb_limb_0_col53
            .try_into()
            .unwrap())
            .unbox();
        let [blake_message_output_message_limb_limb_1_col54]: [QM31; 1] =
            (*blake_message_output_message_limb_limb_1_col54
            .try_into()
            .unwrap())
            .unbox();
        let [blake_message_output_message_limb_limb_0_col55]: [QM31; 1] =
            (*blake_message_output_message_limb_limb_0_col55
            .try_into()
            .unwrap())
            .unbox();
        let [blake_message_output_message_limb_limb_1_col56]: [QM31; 1] =
            (*blake_message_output_message_limb_limb_1_col56
            .try_into()
            .unwrap())
            .unbox();
        let [blake_message_output_message_limb_limb_0_col57]: [QM31; 1] =
            (*blake_message_output_message_limb_limb_0_col57
            .try_into()
            .unwrap())
            .unbox();
        let [blake_message_output_message_limb_limb_1_col58]: [QM31; 1] =
            (*blake_message_output_message_limb_limb_1_col58
            .try_into()
            .unwrap())
            .unbox();
        let [blake_message_output_message_limb_limb_0_col59]: [QM31; 1] =
            (*blake_message_output_message_limb_limb_0_col59
            .try_into()
            .unwrap())
            .unbox();
        let [blake_message_output_message_limb_limb_1_col60]: [QM31; 1] =
            (*blake_message_output_message_limb_limb_1_col60
            .try_into()
            .unwrap())
            .unbox();
        let [blake_message_output_message_limb_limb_0_col61]: [QM31; 1] =
            (*blake_message_output_message_limb_limb_0_col61
            .try_into()
            .unwrap())
            .unbox();
        let [blake_message_output_message_limb_limb_1_col62]: [QM31; 1] =
            (*blake_message_output_message_limb_limb_1_col62
            .try_into()
            .unwrap())
            .unbox();
        let [blake_message_output_message_limb_limb_0_col63]: [QM31; 1] =
            (*blake_message_output_message_limb_limb_0_col63
            .try_into()
            .unwrap())
            .unbox();
        let [blake_message_output_message_limb_limb_1_col64]: [QM31; 1] =
            (*blake_message_output_message_limb_limb_1_col64
            .try_into()
            .unwrap())
            .unbox();
        let [blake_message_output_message_limb_limb_0_col65]: [QM31; 1] =
            (*blake_message_output_message_limb_limb_0_col65
            .try_into()
            .unwrap())
            .unbox();
        let [blake_message_output_message_limb_limb_1_col66]: [QM31; 1] =
            (*blake_message_output_message_limb_limb_1_col66
            .try_into()
            .unwrap())
            .unbox();
        let [blake_message_output_message_limb_limb_0_col67]: [QM31; 1] =
            (*blake_message_output_message_limb_limb_0_col67
            .try_into()
            .unwrap())
            .unbox();
        let [blake_message_output_message_limb_limb_1_col68]: [QM31; 1] =
            (*blake_message_output_message_limb_limb_1_col68
            .try_into()
            .unwrap())
            .unbox();
        let [blake_message_output_message_limb_limb_0_col69]: [QM31; 1] =
            (*blake_message_output_message_limb_limb_0_col69
            .try_into()
            .unwrap())
            .unbox();
        let [blake_message_output_message_limb_limb_1_col70]: [QM31; 1] =
            (*blake_message_output_message_limb_limb_1_col70
            .try_into()
            .unwrap())
            .unbox();
        let [blake_message_output_message_limb_limb_0_col71]: [QM31; 1] =
            (*blake_message_output_message_limb_limb_0_col71
            .try_into()
            .unwrap())
            .unbox();
        let [blake_message_output_message_limb_limb_1_col72]: [QM31; 1] =
            (*blake_message_output_message_limb_limb_1_col72
            .try_into()
            .unwrap())
            .unbox();
        let [blake_message_output_message_limb_limb_0_col73]: [QM31; 1] =
            (*blake_message_output_message_limb_limb_0_col73
            .try_into()
            .unwrap())
            .unbox();
        let [blake_message_output_message_limb_limb_1_col74]: [QM31; 1] =
            (*blake_message_output_message_limb_limb_1_col74
            .try_into()
            .unwrap())
            .unbox();
        let [blake_message_output_message_limb_limb_0_col75]: [QM31; 1] =
            (*blake_message_output_message_limb_limb_0_col75
            .try_into()
            .unwrap())
            .unbox();
        let [blake_message_output_message_limb_limb_1_col76]: [QM31; 1] =
            (*blake_message_output_message_limb_limb_1_col76
            .try_into()
            .unwrap())
            .unbox();
        let [blake_message_output_message_limb_limb_0_col77]: [QM31; 1] =
            (*blake_message_output_message_limb_limb_0_col77
            .try_into()
            .unwrap())
            .unbox();
        let [blake_message_output_message_limb_limb_1_col78]: [QM31; 1] =
            (*blake_message_output_message_limb_limb_1_col78
            .try_into()
            .unwrap())
            .unbox();
        let [blake_message_output_message_limb_limb_0_col79]: [QM31; 1] =
            (*blake_message_output_message_limb_limb_0_col79
            .try_into()
            .unwrap())
            .unbox();
        let [blake_message_output_message_limb_limb_1_col80]: [QM31; 1] =
            (*blake_message_output_message_limb_limb_1_col80
            .try_into()
            .unwrap())
            .unbox();
        let [blake_message_output_message_limb_limb_0_col81]: [QM31; 1] =
            (*blake_message_output_message_limb_limb_0_col81
            .try_into()
            .unwrap())
            .unbox();
        let [blake_message_output_message_limb_limb_1_col82]: [QM31; 1] =
            (*blake_message_output_message_limb_limb_1_col82
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_0_col83]: [QM31; 1] = (*blake_g_output_limb_0_col83
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_1_col84]: [QM31; 1] = (*blake_g_output_limb_1_col84
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_2_col85]: [QM31; 1] = (*blake_g_output_limb_2_col85
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_3_col86]: [QM31; 1] = (*blake_g_output_limb_3_col86
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_4_col87]: [QM31; 1] = (*blake_g_output_limb_4_col87
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_5_col88]: [QM31; 1] = (*blake_g_output_limb_5_col88
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_6_col89]: [QM31; 1] = (*blake_g_output_limb_6_col89
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_7_col90]: [QM31; 1] = (*blake_g_output_limb_7_col90
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_0_col91]: [QM31; 1] = (*blake_g_output_limb_0_col91
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_1_col92]: [QM31; 1] = (*blake_g_output_limb_1_col92
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_2_col93]: [QM31; 1] = (*blake_g_output_limb_2_col93
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_3_col94]: [QM31; 1] = (*blake_g_output_limb_3_col94
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_4_col95]: [QM31; 1] = (*blake_g_output_limb_4_col95
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_5_col96]: [QM31; 1] = (*blake_g_output_limb_5_col96
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_6_col97]: [QM31; 1] = (*blake_g_output_limb_6_col97
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_7_col98]: [QM31; 1] = (*blake_g_output_limb_7_col98
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_0_col99]: [QM31; 1] = (*blake_g_output_limb_0_col99
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_1_col100]: [QM31; 1] = (*blake_g_output_limb_1_col100
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_2_col101]: [QM31; 1] = (*blake_g_output_limb_2_col101
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_3_col102]: [QM31; 1] = (*blake_g_output_limb_3_col102
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_4_col103]: [QM31; 1] = (*blake_g_output_limb_4_col103
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_5_col104]: [QM31; 1] = (*blake_g_output_limb_5_col104
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_6_col105]: [QM31; 1] = (*blake_g_output_limb_6_col105
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_7_col106]: [QM31; 1] = (*blake_g_output_limb_7_col106
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_0_col107]: [QM31; 1] = (*blake_g_output_limb_0_col107
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_1_col108]: [QM31; 1] = (*blake_g_output_limb_1_col108
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_2_col109]: [QM31; 1] = (*blake_g_output_limb_2_col109
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_3_col110]: [QM31; 1] = (*blake_g_output_limb_3_col110
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_4_col111]: [QM31; 1] = (*blake_g_output_limb_4_col111
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_5_col112]: [QM31; 1] = (*blake_g_output_limb_5_col112
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_6_col113]: [QM31; 1] = (*blake_g_output_limb_6_col113
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_7_col114]: [QM31; 1] = (*blake_g_output_limb_7_col114
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_0_col115]: [QM31; 1] = (*blake_g_output_limb_0_col115
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_1_col116]: [QM31; 1] = (*blake_g_output_limb_1_col116
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_2_col117]: [QM31; 1] = (*blake_g_output_limb_2_col117
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_3_col118]: [QM31; 1] = (*blake_g_output_limb_3_col118
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_4_col119]: [QM31; 1] = (*blake_g_output_limb_4_col119
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_5_col120]: [QM31; 1] = (*blake_g_output_limb_5_col120
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_6_col121]: [QM31; 1] = (*blake_g_output_limb_6_col121
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_7_col122]: [QM31; 1] = (*blake_g_output_limb_7_col122
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_0_col123]: [QM31; 1] = (*blake_g_output_limb_0_col123
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_1_col124]: [QM31; 1] = (*blake_g_output_limb_1_col124
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_2_col125]: [QM31; 1] = (*blake_g_output_limb_2_col125
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_3_col126]: [QM31; 1] = (*blake_g_output_limb_3_col126
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_4_col127]: [QM31; 1] = (*blake_g_output_limb_4_col127
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_5_col128]: [QM31; 1] = (*blake_g_output_limb_5_col128
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_6_col129]: [QM31; 1] = (*blake_g_output_limb_6_col129
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_7_col130]: [QM31; 1] = (*blake_g_output_limb_7_col130
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_0_col131]: [QM31; 1] = (*blake_g_output_limb_0_col131
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_1_col132]: [QM31; 1] = (*blake_g_output_limb_1_col132
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_2_col133]: [QM31; 1] = (*blake_g_output_limb_2_col133
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_3_col134]: [QM31; 1] = (*blake_g_output_limb_3_col134
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_4_col135]: [QM31; 1] = (*blake_g_output_limb_4_col135
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_5_col136]: [QM31; 1] = (*blake_g_output_limb_5_col136
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_6_col137]: [QM31; 1] = (*blake_g_output_limb_6_col137
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_7_col138]: [QM31; 1] = (*blake_g_output_limb_7_col138
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_0_col139]: [QM31; 1] = (*blake_g_output_limb_0_col139
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_1_col140]: [QM31; 1] = (*blake_g_output_limb_1_col140
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_2_col141]: [QM31; 1] = (*blake_g_output_limb_2_col141
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_3_col142]: [QM31; 1] = (*blake_g_output_limb_3_col142
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_4_col143]: [QM31; 1] = (*blake_g_output_limb_4_col143
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_5_col144]: [QM31; 1] = (*blake_g_output_limb_5_col144
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_6_col145]: [QM31; 1] = (*blake_g_output_limb_6_col145
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_7_col146]: [QM31; 1] = (*blake_g_output_limb_7_col146
            .try_into()
            .unwrap())
            .unbox();
        let [enabler_col147]: [QM31; 1] = (*enabler_col147.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        blake_round_sigma_sum_0 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<1805967942, 0, 0, 0>(), input_limb_1_col1,
                    blake_round_sigma_output_limb_0_col35, blake_round_sigma_output_limb_1_col36,
                    blake_round_sigma_output_limb_2_col37, blake_round_sigma_output_limb_3_col38,
                    blake_round_sigma_output_limb_4_col39, blake_round_sigma_output_limb_5_col40,
                    blake_round_sigma_output_limb_6_col41, blake_round_sigma_output_limb_7_col42,
                    blake_round_sigma_output_limb_8_col43, blake_round_sigma_output_limb_9_col44,
                    blake_round_sigma_output_limb_10_col45, blake_round_sigma_output_limb_11_col46,
                    blake_round_sigma_output_limb_12_col47, blake_round_sigma_output_limb_13_col48,
                    blake_round_sigma_output_limb_14_col49, blake_round_sigma_output_limb_15_col50,
                ]
                    .span(),
            );
        numerator_0 = qm31_const::<1, 0, 0, 0>();

        blake_message_sum_1 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<1492981981, 0, 0, 0>(), input_limb_34_col34,
                    blake_round_sigma_output_limb_0_col35,
                    blake_message_output_message_limb_limb_0_col51,
                    blake_message_output_message_limb_limb_1_col52,
                ]
                    .span(),
            );
        numerator_1 = enabler_col147;

        blake_message_sum_2 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<1492981981, 0, 0, 0>(), input_limb_34_col34,
                    blake_round_sigma_output_limb_1_col36,
                    blake_message_output_message_limb_limb_0_col53,
                    blake_message_output_message_limb_limb_1_col54,
                ]
                    .span(),
            );
        numerator_2 = enabler_col147;

        blake_message_sum_3 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<1492981981, 0, 0, 0>(), input_limb_34_col34,
                    blake_round_sigma_output_limb_2_col37,
                    blake_message_output_message_limb_limb_0_col55,
                    blake_message_output_message_limb_limb_1_col56,
                ]
                    .span(),
            );
        numerator_3 = enabler_col147;

        blake_message_sum_4 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<1492981981, 0, 0, 0>(), input_limb_34_col34,
                    blake_round_sigma_output_limb_3_col38,
                    blake_message_output_message_limb_limb_0_col57,
                    blake_message_output_message_limb_limb_1_col58,
                ]
                    .span(),
            );
        numerator_4 = enabler_col147;

        blake_message_sum_5 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<1492981981, 0, 0, 0>(), input_limb_34_col34,
                    blake_round_sigma_output_limb_4_col39,
                    blake_message_output_message_limb_limb_0_col59,
                    blake_message_output_message_limb_limb_1_col60,
                ]
                    .span(),
            );
        numerator_5 = enabler_col147;

        blake_message_sum_6 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<1492981981, 0, 0, 0>(), input_limb_34_col34,
                    blake_round_sigma_output_limb_5_col40,
                    blake_message_output_message_limb_limb_0_col61,
                    blake_message_output_message_limb_limb_1_col62,
                ]
                    .span(),
            );
        numerator_6 = enabler_col147;

        blake_message_sum_7 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<1492981981, 0, 0, 0>(), input_limb_34_col34,
                    blake_round_sigma_output_limb_6_col41,
                    blake_message_output_message_limb_limb_0_col63,
                    blake_message_output_message_limb_limb_1_col64,
                ]
                    .span(),
            );
        numerator_7 = enabler_col147;

        blake_message_sum_8 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<1492981981, 0, 0, 0>(), input_limb_34_col34,
                    blake_round_sigma_output_limb_7_col42,
                    blake_message_output_message_limb_limb_0_col65,
                    blake_message_output_message_limb_limb_1_col66,
                ]
                    .span(),
            );
        numerator_8 = enabler_col147;

        blake_message_sum_9 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<1492981981, 0, 0, 0>(), input_limb_34_col34,
                    blake_round_sigma_output_limb_8_col43,
                    blake_message_output_message_limb_limb_0_col67,
                    blake_message_output_message_limb_limb_1_col68,
                ]
                    .span(),
            );
        numerator_9 = enabler_col147;

        blake_message_sum_10 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<1492981981, 0, 0, 0>(), input_limb_34_col34,
                    blake_round_sigma_output_limb_9_col44,
                    blake_message_output_message_limb_limb_0_col69,
                    blake_message_output_message_limb_limb_1_col70,
                ]
                    .span(),
            );
        numerator_10 = enabler_col147;

        blake_message_sum_11 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<1492981981, 0, 0, 0>(), input_limb_34_col34,
                    blake_round_sigma_output_limb_10_col45,
                    blake_message_output_message_limb_limb_0_col71,
                    blake_message_output_message_limb_limb_1_col72,
                ]
                    .span(),
            );
        numerator_11 = enabler_col147;

        blake_message_sum_12 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<1492981981, 0, 0, 0>(), input_limb_34_col34,
                    blake_round_sigma_output_limb_11_col46,
                    blake_message_output_message_limb_limb_0_col73,
                    blake_message_output_message_limb_limb_1_col74,
                ]
                    .span(),
            );
        numerator_12 = enabler_col147;

        blake_message_sum_13 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<1492981981, 0, 0, 0>(), input_limb_34_col34,
                    blake_round_sigma_output_limb_12_col47,
                    blake_message_output_message_limb_limb_0_col75,
                    blake_message_output_message_limb_limb_1_col76,
                ]
                    .span(),
            );
        numerator_13 = enabler_col147;

        blake_message_sum_14 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<1492981981, 0, 0, 0>(), input_limb_34_col34,
                    blake_round_sigma_output_limb_13_col48,
                    blake_message_output_message_limb_limb_0_col77,
                    blake_message_output_message_limb_limb_1_col78,
                ]
                    .span(),
            );
        numerator_14 = enabler_col147;

        blake_message_sum_15 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<1492981981, 0, 0, 0>(), input_limb_34_col34,
                    blake_round_sigma_output_limb_14_col49,
                    blake_message_output_message_limb_limb_0_col79,
                    blake_message_output_message_limb_limb_1_col80,
                ]
                    .span(),
            );
        numerator_15 = enabler_col147;

        blake_message_sum_16 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<1492981981, 0, 0, 0>(), input_limb_34_col34,
                    blake_round_sigma_output_limb_15_col50,
                    blake_message_output_message_limb_limb_0_col81,
                    blake_message_output_message_limb_limb_1_col82,
                ]
                    .span(),
            );
        numerator_16 = enabler_col147;

        blake_g_sum_17 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<1139985212, 0, 0, 0>(), input_limb_2_col2, input_limb_3_col3,
                    input_limb_10_col10, input_limb_11_col11, input_limb_18_col18,
                    input_limb_19_col19, input_limb_26_col26, input_limb_27_col27,
                    blake_message_output_message_limb_limb_0_col51,
                    blake_message_output_message_limb_limb_1_col52,
                    blake_message_output_message_limb_limb_0_col53,
                    blake_message_output_message_limb_limb_1_col54, blake_g_output_limb_0_col83,
                    blake_g_output_limb_1_col84, blake_g_output_limb_2_col85,
                    blake_g_output_limb_3_col86, blake_g_output_limb_4_col87,
                    blake_g_output_limb_5_col88, blake_g_output_limb_6_col89,
                    blake_g_output_limb_7_col90,
                ]
                    .span(),
            );
        numerator_17 = qm31_const::<1, 0, 0, 0>();

        blake_g_sum_18 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<1139985212, 0, 0, 0>(), input_limb_4_col4, input_limb_5_col5,
                    input_limb_12_col12, input_limb_13_col13, input_limb_20_col20,
                    input_limb_21_col21, input_limb_28_col28, input_limb_29_col29,
                    blake_message_output_message_limb_limb_0_col55,
                    blake_message_output_message_limb_limb_1_col56,
                    blake_message_output_message_limb_limb_0_col57,
                    blake_message_output_message_limb_limb_1_col58, blake_g_output_limb_0_col91,
                    blake_g_output_limb_1_col92, blake_g_output_limb_2_col93,
                    blake_g_output_limb_3_col94, blake_g_output_limb_4_col95,
                    blake_g_output_limb_5_col96, blake_g_output_limb_6_col97,
                    blake_g_output_limb_7_col98,
                ]
                    .span(),
            );
        numerator_18 = qm31_const::<1, 0, 0, 0>();

        blake_g_sum_19 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<1139985212, 0, 0, 0>(), input_limb_6_col6, input_limb_7_col7,
                    input_limb_14_col14, input_limb_15_col15, input_limb_22_col22,
                    input_limb_23_col23, input_limb_30_col30, input_limb_31_col31,
                    blake_message_output_message_limb_limb_0_col59,
                    blake_message_output_message_limb_limb_1_col60,
                    blake_message_output_message_limb_limb_0_col61,
                    blake_message_output_message_limb_limb_1_col62, blake_g_output_limb_0_col99,
                    blake_g_output_limb_1_col100, blake_g_output_limb_2_col101,
                    blake_g_output_limb_3_col102, blake_g_output_limb_4_col103,
                    blake_g_output_limb_5_col104, blake_g_output_limb_6_col105,
                    blake_g_output_limb_7_col106,
                ]
                    .span(),
            );
        numerator_19 = qm31_const::<1, 0, 0, 0>();

        blake_g_sum_20 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<1139985212, 0, 0, 0>(), input_limb_8_col8, input_limb_9_col9,
                    input_limb_16_col16, input_limb_17_col17, input_limb_24_col24,
                    input_limb_25_col25, input_limb_32_col32, input_limb_33_col33,
                    blake_message_output_message_limb_limb_0_col63,
                    blake_message_output_message_limb_limb_1_col64,
                    blake_message_output_message_limb_limb_0_col65,
                    blake_message_output_message_limb_limb_1_col66, blake_g_output_limb_0_col107,
                    blake_g_output_limb_1_col108, blake_g_output_limb_2_col109,
                    blake_g_output_limb_3_col110, blake_g_output_limb_4_col111,
                    blake_g_output_limb_5_col112, blake_g_output_limb_6_col113,
                    blake_g_output_limb_7_col114,
                ]
                    .span(),
            );
        numerator_20 = qm31_const::<1, 0, 0, 0>();

        blake_g_sum_21 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<1139985212, 0, 0, 0>(), blake_g_output_limb_0_col83,
                    blake_g_output_limb_1_col84, blake_g_output_limb_2_col93,
                    blake_g_output_limb_3_col94, blake_g_output_limb_4_col103,
                    blake_g_output_limb_5_col104, blake_g_output_limb_6_col113,
                    blake_g_output_limb_7_col114, blake_message_output_message_limb_limb_0_col67,
                    blake_message_output_message_limb_limb_1_col68,
                    blake_message_output_message_limb_limb_0_col69,
                    blake_message_output_message_limb_limb_1_col70, blake_g_output_limb_0_col115,
                    blake_g_output_limb_1_col116, blake_g_output_limb_2_col117,
                    blake_g_output_limb_3_col118, blake_g_output_limb_4_col119,
                    blake_g_output_limb_5_col120, blake_g_output_limb_6_col121,
                    blake_g_output_limb_7_col122,
                ]
                    .span(),
            );
        numerator_21 = qm31_const::<1, 0, 0, 0>();

        blake_g_sum_22 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<1139985212, 0, 0, 0>(), blake_g_output_limb_0_col91,
                    blake_g_output_limb_1_col92, blake_g_output_limb_2_col101,
                    blake_g_output_limb_3_col102, blake_g_output_limb_4_col111,
                    blake_g_output_limb_5_col112, blake_g_output_limb_6_col89,
                    blake_g_output_limb_7_col90, blake_message_output_message_limb_limb_0_col71,
                    blake_message_output_message_limb_limb_1_col72,
                    blake_message_output_message_limb_limb_0_col73,
                    blake_message_output_message_limb_limb_1_col74, blake_g_output_limb_0_col123,
                    blake_g_output_limb_1_col124, blake_g_output_limb_2_col125,
                    blake_g_output_limb_3_col126, blake_g_output_limb_4_col127,
                    blake_g_output_limb_5_col128, blake_g_output_limb_6_col129,
                    blake_g_output_limb_7_col130,
                ]
                    .span(),
            );
        numerator_22 = qm31_const::<1, 0, 0, 0>();

        blake_g_sum_23 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<1139985212, 0, 0, 0>(), blake_g_output_limb_0_col99,
                    blake_g_output_limb_1_col100, blake_g_output_limb_2_col109,
                    blake_g_output_limb_3_col110, blake_g_output_limb_4_col87,
                    blake_g_output_limb_5_col88, blake_g_output_limb_6_col97,
                    blake_g_output_limb_7_col98, blake_message_output_message_limb_limb_0_col75,
                    blake_message_output_message_limb_limb_1_col76,
                    blake_message_output_message_limb_limb_0_col77,
                    blake_message_output_message_limb_limb_1_col78, blake_g_output_limb_0_col131,
                    blake_g_output_limb_1_col132, blake_g_output_limb_2_col133,
                    blake_g_output_limb_3_col134, blake_g_output_limb_4_col135,
                    blake_g_output_limb_5_col136, blake_g_output_limb_6_col137,
                    blake_g_output_limb_7_col138,
                ]
                    .span(),
            );
        numerator_23 = qm31_const::<1, 0, 0, 0>();

        blake_g_sum_24 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<1139985212, 0, 0, 0>(), blake_g_output_limb_0_col107,
                    blake_g_output_limb_1_col108, blake_g_output_limb_2_col85,
                    blake_g_output_limb_3_col86, blake_g_output_limb_4_col95,
                    blake_g_output_limb_5_col96, blake_g_output_limb_6_col105,
                    blake_g_output_limb_7_col106, blake_message_output_message_limb_limb_0_col79,
                    blake_message_output_message_limb_limb_1_col80,
                    blake_message_output_message_limb_limb_0_col81,
                    blake_message_output_message_limb_limb_1_col82, blake_g_output_limb_0_col139,
                    blake_g_output_limb_1_col140, blake_g_output_limb_2_col141,
                    blake_g_output_limb_3_col142, blake_g_output_limb_4_col143,
                    blake_g_output_limb_5_col144, blake_g_output_limb_6_col145,
                    blake_g_output_limb_7_col146,
                ]
                    .span(),
            );
        numerator_24 = qm31_const::<1, 0, 0, 0>();

        // Constraint - Enabler is a bit
        let constraint_quotient = (((enabler_col147 * enabler_col147) - enabler_col147));
        sum = sum * random_coeff + constraint_quotient;

        blake_round_sum_25 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<40528774, 0, 0, 0>(), input_limb_0_col0, input_limb_1_col1,
                    input_limb_2_col2, input_limb_3_col3, input_limb_4_col4, input_limb_5_col5,
                    input_limb_6_col6, input_limb_7_col7, input_limb_8_col8, input_limb_9_col9,
                    input_limb_10_col10, input_limb_11_col11, input_limb_12_col12,
                    input_limb_13_col13, input_limb_14_col14, input_limb_15_col15,
                    input_limb_16_col16, input_limb_17_col17, input_limb_18_col18,
                    input_limb_19_col19, input_limb_20_col20, input_limb_21_col21,
                    input_limb_22_col22, input_limb_23_col23, input_limb_24_col24,
                    input_limb_25_col25, input_limb_26_col26, input_limb_27_col27,
                    input_limb_28_col28, input_limb_29_col29, input_limb_30_col30,
                    input_limb_31_col31, input_limb_32_col32, input_limb_33_col33,
                    input_limb_34_col34,
                ]
                    .span(),
            );
        numerator_25 = enabler_col147;

        blake_round_sum_26 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<40528774, 0, 0, 0>(), input_limb_0_col0,
                    (input_limb_1_col1 + qm31_const::<1, 0, 0, 0>()), blake_g_output_limb_0_col115,
                    blake_g_output_limb_1_col116, blake_g_output_limb_0_col123,
                    blake_g_output_limb_1_col124, blake_g_output_limb_0_col131,
                    blake_g_output_limb_1_col132, blake_g_output_limb_0_col139,
                    blake_g_output_limb_1_col140, blake_g_output_limb_2_col141,
                    blake_g_output_limb_3_col142, blake_g_output_limb_2_col117,
                    blake_g_output_limb_3_col118, blake_g_output_limb_2_col125,
                    blake_g_output_limb_3_col126, blake_g_output_limb_2_col133,
                    blake_g_output_limb_3_col134, blake_g_output_limb_4_col135,
                    blake_g_output_limb_5_col136, blake_g_output_limb_4_col143,
                    blake_g_output_limb_5_col144, blake_g_output_limb_4_col119,
                    blake_g_output_limb_5_col120, blake_g_output_limb_4_col127,
                    blake_g_output_limb_5_col128, blake_g_output_limb_6_col129,
                    blake_g_output_limb_7_col130, blake_g_output_limb_6_col137,
                    blake_g_output_limb_7_col138, blake_g_output_limb_6_col145,
                    blake_g_output_limb_7_col146, blake_g_output_limb_6_col121,
                    blake_g_output_limb_7_col122, input_limb_34_col34,
                ]
                    .span(),
            );
        numerator_26 = enabler_col147;

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
            column_size,
            ref interaction_trace_mask_values,
            blake_round_sigma_sum_0,
            blake_message_sum_1,
            blake_message_sum_2,
            blake_message_sum_3,
            blake_message_sum_4,
            blake_message_sum_5,
            blake_message_sum_6,
            blake_message_sum_7,
            blake_message_sum_8,
            blake_message_sum_9,
            blake_message_sum_10,
            blake_message_sum_11,
            blake_message_sum_12,
            blake_message_sum_13,
            blake_message_sum_14,
            blake_message_sum_15,
            blake_message_sum_16,
            blake_g_sum_17,
            blake_g_sum_18,
            blake_g_sum_19,
            blake_g_sum_20,
            blake_g_sum_21,
            blake_g_sum_22,
            blake_g_sum_23,
            blake_g_sum_24,
            blake_round_sum_25,
            blake_round_sum_26,
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
    column_size: M31,
    ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
    blake_round_sigma_sum_0: QM31,
    blake_message_sum_1: QM31,
    blake_message_sum_2: QM31,
    blake_message_sum_3: QM31,
    blake_message_sum_4: QM31,
    blake_message_sum_5: QM31,
    blake_message_sum_6: QM31,
    blake_message_sum_7: QM31,
    blake_message_sum_8: QM31,
    blake_message_sum_9: QM31,
    blake_message_sum_10: QM31,
    blake_message_sum_11: QM31,
    blake_message_sum_12: QM31,
    blake_message_sum_13: QM31,
    blake_message_sum_14: QM31,
    blake_message_sum_15: QM31,
    blake_message_sum_16: QM31,
    blake_g_sum_17: QM31,
    blake_g_sum_18: QM31,
    blake_g_sum_19: QM31,
    blake_g_sum_20: QM31,
    blake_g_sum_21: QM31,
    blake_g_sum_22: QM31,
    blake_g_sum_23: QM31,
    blake_g_sum_24: QM31,
    blake_round_sum_25: QM31,
    blake_round_sum_26: QM31,
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
    ]: [Span<QM31>; 56] =
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
    let [trace_2_col52_neg1, trace_2_col52]: [QM31; 2] = (*trace_2_col52.try_into().unwrap())
        .unbox();
    let [trace_2_col53_neg1, trace_2_col53]: [QM31; 2] = (*trace_2_col53.try_into().unwrap())
        .unbox();
    let [trace_2_col54_neg1, trace_2_col54]: [QM31; 2] = (*trace_2_col54.try_into().unwrap())
        .unbox();
    let [trace_2_col55_neg1, trace_2_col55]: [QM31; 2] = (*trace_2_col55.try_into().unwrap())
        .unbox();

    core::internal::revoke_ap_tracking();

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3],
    ))
        * blake_round_sigma_sum_0
        * blake_message_sum_1)
        - (blake_round_sigma_sum_0 * numerator_1)
        - (blake_message_sum_1 * numerator_0));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7],
    )
        - QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3]))
        * blake_message_sum_2
        * blake_message_sum_3)
        - (blake_message_sum_2 * numerator_3)
        - (blake_message_sum_3 * numerator_2));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11],
    )
        - QM31Impl::from_partial_evals([trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7]))
        * blake_message_sum_4
        * blake_message_sum_5)
        - (blake_message_sum_4 * numerator_5)
        - (blake_message_sum_5 * numerator_4));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
    )
        - QM31Impl::from_partial_evals([trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11]))
        * blake_message_sum_6
        * blake_message_sum_7)
        - (blake_message_sum_6 * numerator_7)
        - (blake_message_sum_7 * numerator_6));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
        ))
        * blake_message_sum_8
        * blake_message_sum_9)
        - (blake_message_sum_8 * numerator_9)
        - (blake_message_sum_9 * numerator_8));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
        ))
        * blake_message_sum_10
        * blake_message_sum_11)
        - (blake_message_sum_10 * numerator_11)
        - (blake_message_sum_11 * numerator_10));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23],
        ))
        * blake_message_sum_12
        * blake_message_sum_13)
        - (blake_message_sum_12 * numerator_13)
        - (blake_message_sum_13 * numerator_12));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col28, trace_2_col29, trace_2_col30, trace_2_col31],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27],
        ))
        * blake_message_sum_14
        * blake_message_sum_15)
        - (blake_message_sum_14 * numerator_15)
        - (blake_message_sum_15 * numerator_14));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col32, trace_2_col33, trace_2_col34, trace_2_col35],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col28, trace_2_col29, trace_2_col30, trace_2_col31],
        ))
        * blake_message_sum_16
        * blake_g_sum_17)
        - (blake_message_sum_16 * numerator_17)
        - (blake_g_sum_17 * numerator_16));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col36, trace_2_col37, trace_2_col38, trace_2_col39],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col32, trace_2_col33, trace_2_col34, trace_2_col35],
        ))
        * blake_g_sum_18
        * blake_g_sum_19)
        - (blake_g_sum_18 * numerator_19)
        - (blake_g_sum_19 * numerator_18));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col40, trace_2_col41, trace_2_col42, trace_2_col43],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col36, trace_2_col37, trace_2_col38, trace_2_col39],
        ))
        * blake_g_sum_20
        * blake_g_sum_21)
        - (blake_g_sum_20 * numerator_21)
        - (blake_g_sum_21 * numerator_20));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col44, trace_2_col45, trace_2_col46, trace_2_col47],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col40, trace_2_col41, trace_2_col42, trace_2_col43],
        ))
        * blake_g_sum_22
        * blake_g_sum_23)
        - (blake_g_sum_22 * numerator_23)
        - (blake_g_sum_23 * numerator_22));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col48, trace_2_col49, trace_2_col50, trace_2_col51],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col44, trace_2_col45, trace_2_col46, trace_2_col47],
        ))
        * blake_g_sum_24
        * blake_round_sum_25)
        - (blake_g_sum_24 * numerator_25)
        - (blake_round_sum_25 * numerator_24));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col52, trace_2_col53, trace_2_col54, trace_2_col55],
    )
        - QM31Impl::from_partial_evals([trace_2_col48, trace_2_col49, trace_2_col50, trace_2_col51])
        - QM31Impl::from_partial_evals(
            [trace_2_col52_neg1, trace_2_col53_neg1, trace_2_col54_neg1, trace_2_col55_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * blake_round_sum_26)
        + numerator_26);
    sum = sum * random_coeff + constraint_quotient;
}
