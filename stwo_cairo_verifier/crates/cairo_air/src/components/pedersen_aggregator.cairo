// This file was created by the AIR team.

use crate::components::subroutines::read_positive_known_id_num_bits_252::read_positive_known_id_num_bits_252_evaluate;
use crate::components::subroutines::verify_reduced_252::verify_reduced_252_evaluate;
use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 206;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 3] = [
    ('MemoryIdToBig', 3), ('RangeCheck_8', 4), ('PartialEcMul', 2),
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
        let interaction_log_sizes = [log_size; 24].span();
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
    pub memory_id_to_big_lookup_elements: crate::MemoryIdToBigElements,
    pub range_check_8_lookup_elements: crate::RangeCheck_8Elements,
    pub partial_ec_mul_lookup_elements: crate::PartialEcMulElements,
    pub pedersen_aggregator_lookup_elements: crate::PedersenAggregatorElements,
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
            memory_id_to_big_lookup_elements: interaction_elements.memory_id_to_value.clone(),
            range_check_8_lookup_elements: interaction_elements.range_checks.rc_8.clone(),
            partial_ec_mul_lookup_elements: interaction_elements.partial_ec_mul.clone(),
            pedersen_aggregator_lookup_elements: interaction_elements.pedersen_aggregator.clone(),
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
        let mut memory_id_to_big_sum_0: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_1: QM31 = Zero::zero();
        let mut range_check_8_sum_2: QM31 = Zero::zero();
        let mut range_check_8_sum_3: QM31 = Zero::zero();
        let mut range_check_8_sum_4: QM31 = Zero::zero();
        let mut range_check_8_sum_5: QM31 = Zero::zero();
        let mut partial_ec_mul_sum_6: QM31 = Zero::zero();
        let mut partial_ec_mul_sum_7: QM31 = Zero::zero();
        let mut partial_ec_mul_sum_8: QM31 = Zero::zero();
        let mut partial_ec_mul_sum_9: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_10: QM31 = Zero::zero();
        let mut pedersen_aggregator_sum_11: QM31 = Zero::zero();
        let seq = preprocessed_mask_values
            .get_and_mark_used(seq_column_idx(*(self.claim.log_size)));

        let [
            input_limb_0_col0,
            input_limb_1_col1,
            input_limb_2_col2,
            value_limb_0_col3,
            value_limb_1_col4,
            value_limb_2_col5,
            value_limb_3_col6,
            value_limb_4_col7,
            value_limb_5_col8,
            value_limb_6_col9,
            value_limb_7_col10,
            value_limb_8_col11,
            value_limb_9_col12,
            value_limb_10_col13,
            value_limb_11_col14,
            value_limb_12_col15,
            value_limb_13_col16,
            value_limb_14_col17,
            value_limb_15_col18,
            value_limb_16_col19,
            value_limb_17_col20,
            value_limb_18_col21,
            value_limb_19_col22,
            value_limb_20_col23,
            value_limb_21_col24,
            value_limb_22_col25,
            value_limb_23_col26,
            value_limb_24_col27,
            value_limb_25_col28,
            value_limb_26_col29,
            value_limb_27_col30,
            value_limb_0_col31,
            value_limb_1_col32,
            value_limb_2_col33,
            value_limb_3_col34,
            value_limb_4_col35,
            value_limb_5_col36,
            value_limb_6_col37,
            value_limb_7_col38,
            value_limb_8_col39,
            value_limb_9_col40,
            value_limb_10_col41,
            value_limb_11_col42,
            value_limb_12_col43,
            value_limb_13_col44,
            value_limb_14_col45,
            value_limb_15_col46,
            value_limb_16_col47,
            value_limb_17_col48,
            value_limb_18_col49,
            value_limb_19_col50,
            value_limb_20_col51,
            value_limb_21_col52,
            value_limb_22_col53,
            value_limb_23_col54,
            value_limb_24_col55,
            value_limb_25_col56,
            value_limb_26_col57,
            value_limb_27_col58,
            ms_limb_is_max_col59,
            ms_and_mid_limbs_are_max_col60,
            rc_input_col61,
            ms_limb_is_max_col62,
            ms_and_mid_limbs_are_max_col63,
            rc_input_col64,
            partial_ec_mul_output_limb_0_col65,
            partial_ec_mul_output_limb_1_col66,
            partial_ec_mul_output_limb_2_col67,
            partial_ec_mul_output_limb_3_col68,
            partial_ec_mul_output_limb_4_col69,
            partial_ec_mul_output_limb_5_col70,
            partial_ec_mul_output_limb_6_col71,
            partial_ec_mul_output_limb_7_col72,
            partial_ec_mul_output_limb_8_col73,
            partial_ec_mul_output_limb_9_col74,
            partial_ec_mul_output_limb_10_col75,
            partial_ec_mul_output_limb_11_col76,
            partial_ec_mul_output_limb_12_col77,
            partial_ec_mul_output_limb_13_col78,
            partial_ec_mul_output_limb_14_col79,
            partial_ec_mul_output_limb_15_col80,
            partial_ec_mul_output_limb_16_col81,
            partial_ec_mul_output_limb_17_col82,
            partial_ec_mul_output_limb_18_col83,
            partial_ec_mul_output_limb_19_col84,
            partial_ec_mul_output_limb_20_col85,
            partial_ec_mul_output_limb_21_col86,
            partial_ec_mul_output_limb_22_col87,
            partial_ec_mul_output_limb_23_col88,
            partial_ec_mul_output_limb_24_col89,
            partial_ec_mul_output_limb_25_col90,
            partial_ec_mul_output_limb_26_col91,
            partial_ec_mul_output_limb_27_col92,
            partial_ec_mul_output_limb_28_col93,
            partial_ec_mul_output_limb_29_col94,
            partial_ec_mul_output_limb_30_col95,
            partial_ec_mul_output_limb_31_col96,
            partial_ec_mul_output_limb_32_col97,
            partial_ec_mul_output_limb_33_col98,
            partial_ec_mul_output_limb_34_col99,
            partial_ec_mul_output_limb_35_col100,
            partial_ec_mul_output_limb_36_col101,
            partial_ec_mul_output_limb_37_col102,
            partial_ec_mul_output_limb_38_col103,
            partial_ec_mul_output_limb_39_col104,
            partial_ec_mul_output_limb_40_col105,
            partial_ec_mul_output_limb_41_col106,
            partial_ec_mul_output_limb_42_col107,
            partial_ec_mul_output_limb_43_col108,
            partial_ec_mul_output_limb_44_col109,
            partial_ec_mul_output_limb_45_col110,
            partial_ec_mul_output_limb_46_col111,
            partial_ec_mul_output_limb_47_col112,
            partial_ec_mul_output_limb_48_col113,
            partial_ec_mul_output_limb_49_col114,
            partial_ec_mul_output_limb_50_col115,
            partial_ec_mul_output_limb_51_col116,
            partial_ec_mul_output_limb_52_col117,
            partial_ec_mul_output_limb_53_col118,
            partial_ec_mul_output_limb_54_col119,
            partial_ec_mul_output_limb_55_col120,
            partial_ec_mul_output_limb_56_col121,
            partial_ec_mul_output_limb_57_col122,
            partial_ec_mul_output_limb_58_col123,
            partial_ec_mul_output_limb_59_col124,
            partial_ec_mul_output_limb_60_col125,
            partial_ec_mul_output_limb_61_col126,
            partial_ec_mul_output_limb_62_col127,
            partial_ec_mul_output_limb_63_col128,
            partial_ec_mul_output_limb_64_col129,
            partial_ec_mul_output_limb_65_col130,
            partial_ec_mul_output_limb_66_col131,
            partial_ec_mul_output_limb_67_col132,
            partial_ec_mul_output_limb_68_col133,
            partial_ec_mul_output_limb_69_col134,
            partial_ec_mul_output_limb_0_col135,
            partial_ec_mul_output_limb_1_col136,
            partial_ec_mul_output_limb_2_col137,
            partial_ec_mul_output_limb_3_col138,
            partial_ec_mul_output_limb_4_col139,
            partial_ec_mul_output_limb_5_col140,
            partial_ec_mul_output_limb_6_col141,
            partial_ec_mul_output_limb_7_col142,
            partial_ec_mul_output_limb_8_col143,
            partial_ec_mul_output_limb_9_col144,
            partial_ec_mul_output_limb_10_col145,
            partial_ec_mul_output_limb_11_col146,
            partial_ec_mul_output_limb_12_col147,
            partial_ec_mul_output_limb_13_col148,
            partial_ec_mul_output_limb_14_col149,
            partial_ec_mul_output_limb_15_col150,
            partial_ec_mul_output_limb_16_col151,
            partial_ec_mul_output_limb_17_col152,
            partial_ec_mul_output_limb_18_col153,
            partial_ec_mul_output_limb_19_col154,
            partial_ec_mul_output_limb_20_col155,
            partial_ec_mul_output_limb_21_col156,
            partial_ec_mul_output_limb_22_col157,
            partial_ec_mul_output_limb_23_col158,
            partial_ec_mul_output_limb_24_col159,
            partial_ec_mul_output_limb_25_col160,
            partial_ec_mul_output_limb_26_col161,
            partial_ec_mul_output_limb_27_col162,
            partial_ec_mul_output_limb_28_col163,
            partial_ec_mul_output_limb_29_col164,
            partial_ec_mul_output_limb_30_col165,
            partial_ec_mul_output_limb_31_col166,
            partial_ec_mul_output_limb_32_col167,
            partial_ec_mul_output_limb_33_col168,
            partial_ec_mul_output_limb_34_col169,
            partial_ec_mul_output_limb_35_col170,
            partial_ec_mul_output_limb_36_col171,
            partial_ec_mul_output_limb_37_col172,
            partial_ec_mul_output_limb_38_col173,
            partial_ec_mul_output_limb_39_col174,
            partial_ec_mul_output_limb_40_col175,
            partial_ec_mul_output_limb_41_col176,
            partial_ec_mul_output_limb_42_col177,
            partial_ec_mul_output_limb_43_col178,
            partial_ec_mul_output_limb_44_col179,
            partial_ec_mul_output_limb_45_col180,
            partial_ec_mul_output_limb_46_col181,
            partial_ec_mul_output_limb_47_col182,
            partial_ec_mul_output_limb_48_col183,
            partial_ec_mul_output_limb_49_col184,
            partial_ec_mul_output_limb_50_col185,
            partial_ec_mul_output_limb_51_col186,
            partial_ec_mul_output_limb_52_col187,
            partial_ec_mul_output_limb_53_col188,
            partial_ec_mul_output_limb_54_col189,
            partial_ec_mul_output_limb_55_col190,
            partial_ec_mul_output_limb_56_col191,
            partial_ec_mul_output_limb_57_col192,
            partial_ec_mul_output_limb_58_col193,
            partial_ec_mul_output_limb_59_col194,
            partial_ec_mul_output_limb_60_col195,
            partial_ec_mul_output_limb_61_col196,
            partial_ec_mul_output_limb_62_col197,
            partial_ec_mul_output_limb_63_col198,
            partial_ec_mul_output_limb_64_col199,
            partial_ec_mul_output_limb_65_col200,
            partial_ec_mul_output_limb_66_col201,
            partial_ec_mul_output_limb_67_col202,
            partial_ec_mul_output_limb_68_col203,
            partial_ec_mul_output_limb_69_col204,
            pedersen_aggregator_multiplicity,
        ]: [Span<QM31>; 206] =
            (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [input_limb_0_col0]: [QM31; 1] = (*input_limb_0_col0.try_into().unwrap()).unbox();
        let [input_limb_1_col1]: [QM31; 1] = (*input_limb_1_col1.try_into().unwrap()).unbox();
        let [input_limb_2_col2]: [QM31; 1] = (*input_limb_2_col2.try_into().unwrap()).unbox();
        let [value_limb_0_col3]: [QM31; 1] = (*value_limb_0_col3.try_into().unwrap()).unbox();
        let [value_limb_1_col4]: [QM31; 1] = (*value_limb_1_col4.try_into().unwrap()).unbox();
        let [value_limb_2_col5]: [QM31; 1] = (*value_limb_2_col5.try_into().unwrap()).unbox();
        let [value_limb_3_col6]: [QM31; 1] = (*value_limb_3_col6.try_into().unwrap()).unbox();
        let [value_limb_4_col7]: [QM31; 1] = (*value_limb_4_col7.try_into().unwrap()).unbox();
        let [value_limb_5_col8]: [QM31; 1] = (*value_limb_5_col8.try_into().unwrap()).unbox();
        let [value_limb_6_col9]: [QM31; 1] = (*value_limb_6_col9.try_into().unwrap()).unbox();
        let [value_limb_7_col10]: [QM31; 1] = (*value_limb_7_col10.try_into().unwrap()).unbox();
        let [value_limb_8_col11]: [QM31; 1] = (*value_limb_8_col11.try_into().unwrap()).unbox();
        let [value_limb_9_col12]: [QM31; 1] = (*value_limb_9_col12.try_into().unwrap()).unbox();
        let [value_limb_10_col13]: [QM31; 1] = (*value_limb_10_col13.try_into().unwrap()).unbox();
        let [value_limb_11_col14]: [QM31; 1] = (*value_limb_11_col14.try_into().unwrap()).unbox();
        let [value_limb_12_col15]: [QM31; 1] = (*value_limb_12_col15.try_into().unwrap()).unbox();
        let [value_limb_13_col16]: [QM31; 1] = (*value_limb_13_col16.try_into().unwrap()).unbox();
        let [value_limb_14_col17]: [QM31; 1] = (*value_limb_14_col17.try_into().unwrap()).unbox();
        let [value_limb_15_col18]: [QM31; 1] = (*value_limb_15_col18.try_into().unwrap()).unbox();
        let [value_limb_16_col19]: [QM31; 1] = (*value_limb_16_col19.try_into().unwrap()).unbox();
        let [value_limb_17_col20]: [QM31; 1] = (*value_limb_17_col20.try_into().unwrap()).unbox();
        let [value_limb_18_col21]: [QM31; 1] = (*value_limb_18_col21.try_into().unwrap()).unbox();
        let [value_limb_19_col22]: [QM31; 1] = (*value_limb_19_col22.try_into().unwrap()).unbox();
        let [value_limb_20_col23]: [QM31; 1] = (*value_limb_20_col23.try_into().unwrap()).unbox();
        let [value_limb_21_col24]: [QM31; 1] = (*value_limb_21_col24.try_into().unwrap()).unbox();
        let [value_limb_22_col25]: [QM31; 1] = (*value_limb_22_col25.try_into().unwrap()).unbox();
        let [value_limb_23_col26]: [QM31; 1] = (*value_limb_23_col26.try_into().unwrap()).unbox();
        let [value_limb_24_col27]: [QM31; 1] = (*value_limb_24_col27.try_into().unwrap()).unbox();
        let [value_limb_25_col28]: [QM31; 1] = (*value_limb_25_col28.try_into().unwrap()).unbox();
        let [value_limb_26_col29]: [QM31; 1] = (*value_limb_26_col29.try_into().unwrap()).unbox();
        let [value_limb_27_col30]: [QM31; 1] = (*value_limb_27_col30.try_into().unwrap()).unbox();
        let [value_limb_0_col31]: [QM31; 1] = (*value_limb_0_col31.try_into().unwrap()).unbox();
        let [value_limb_1_col32]: [QM31; 1] = (*value_limb_1_col32.try_into().unwrap()).unbox();
        let [value_limb_2_col33]: [QM31; 1] = (*value_limb_2_col33.try_into().unwrap()).unbox();
        let [value_limb_3_col34]: [QM31; 1] = (*value_limb_3_col34.try_into().unwrap()).unbox();
        let [value_limb_4_col35]: [QM31; 1] = (*value_limb_4_col35.try_into().unwrap()).unbox();
        let [value_limb_5_col36]: [QM31; 1] = (*value_limb_5_col36.try_into().unwrap()).unbox();
        let [value_limb_6_col37]: [QM31; 1] = (*value_limb_6_col37.try_into().unwrap()).unbox();
        let [value_limb_7_col38]: [QM31; 1] = (*value_limb_7_col38.try_into().unwrap()).unbox();
        let [value_limb_8_col39]: [QM31; 1] = (*value_limb_8_col39.try_into().unwrap()).unbox();
        let [value_limb_9_col40]: [QM31; 1] = (*value_limb_9_col40.try_into().unwrap()).unbox();
        let [value_limb_10_col41]: [QM31; 1] = (*value_limb_10_col41.try_into().unwrap()).unbox();
        let [value_limb_11_col42]: [QM31; 1] = (*value_limb_11_col42.try_into().unwrap()).unbox();
        let [value_limb_12_col43]: [QM31; 1] = (*value_limb_12_col43.try_into().unwrap()).unbox();
        let [value_limb_13_col44]: [QM31; 1] = (*value_limb_13_col44.try_into().unwrap()).unbox();
        let [value_limb_14_col45]: [QM31; 1] = (*value_limb_14_col45.try_into().unwrap()).unbox();
        let [value_limb_15_col46]: [QM31; 1] = (*value_limb_15_col46.try_into().unwrap()).unbox();
        let [value_limb_16_col47]: [QM31; 1] = (*value_limb_16_col47.try_into().unwrap()).unbox();
        let [value_limb_17_col48]: [QM31; 1] = (*value_limb_17_col48.try_into().unwrap()).unbox();
        let [value_limb_18_col49]: [QM31; 1] = (*value_limb_18_col49.try_into().unwrap()).unbox();
        let [value_limb_19_col50]: [QM31; 1] = (*value_limb_19_col50.try_into().unwrap()).unbox();
        let [value_limb_20_col51]: [QM31; 1] = (*value_limb_20_col51.try_into().unwrap()).unbox();
        let [value_limb_21_col52]: [QM31; 1] = (*value_limb_21_col52.try_into().unwrap()).unbox();
        let [value_limb_22_col53]: [QM31; 1] = (*value_limb_22_col53.try_into().unwrap()).unbox();
        let [value_limb_23_col54]: [QM31; 1] = (*value_limb_23_col54.try_into().unwrap()).unbox();
        let [value_limb_24_col55]: [QM31; 1] = (*value_limb_24_col55.try_into().unwrap()).unbox();
        let [value_limb_25_col56]: [QM31; 1] = (*value_limb_25_col56.try_into().unwrap()).unbox();
        let [value_limb_26_col57]: [QM31; 1] = (*value_limb_26_col57.try_into().unwrap()).unbox();
        let [value_limb_27_col58]: [QM31; 1] = (*value_limb_27_col58.try_into().unwrap()).unbox();
        let [ms_limb_is_max_col59]: [QM31; 1] = (*ms_limb_is_max_col59.try_into().unwrap()).unbox();
        let [ms_and_mid_limbs_are_max_col60]: [QM31; 1] = (*ms_and_mid_limbs_are_max_col60
            .try_into()
            .unwrap())
            .unbox();
        let [rc_input_col61]: [QM31; 1] = (*rc_input_col61.try_into().unwrap()).unbox();
        let [ms_limb_is_max_col62]: [QM31; 1] = (*ms_limb_is_max_col62.try_into().unwrap()).unbox();
        let [ms_and_mid_limbs_are_max_col63]: [QM31; 1] = (*ms_and_mid_limbs_are_max_col63
            .try_into()
            .unwrap())
            .unbox();
        let [rc_input_col64]: [QM31; 1] = (*rc_input_col64.try_into().unwrap()).unbox();
        let [partial_ec_mul_output_limb_0_col65]: [QM31; 1] = (*partial_ec_mul_output_limb_0_col65
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_1_col66]: [QM31; 1] = (*partial_ec_mul_output_limb_1_col66
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_2_col67]: [QM31; 1] = (*partial_ec_mul_output_limb_2_col67
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_3_col68]: [QM31; 1] = (*partial_ec_mul_output_limb_3_col68
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_4_col69]: [QM31; 1] = (*partial_ec_mul_output_limb_4_col69
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_5_col70]: [QM31; 1] = (*partial_ec_mul_output_limb_5_col70
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_6_col71]: [QM31; 1] = (*partial_ec_mul_output_limb_6_col71
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_7_col72]: [QM31; 1] = (*partial_ec_mul_output_limb_7_col72
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_8_col73]: [QM31; 1] = (*partial_ec_mul_output_limb_8_col73
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_9_col74]: [QM31; 1] = (*partial_ec_mul_output_limb_9_col74
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_10_col75]: [QM31; 1] = (*partial_ec_mul_output_limb_10_col75
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_11_col76]: [QM31; 1] = (*partial_ec_mul_output_limb_11_col76
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_12_col77]: [QM31; 1] = (*partial_ec_mul_output_limb_12_col77
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_13_col78]: [QM31; 1] = (*partial_ec_mul_output_limb_13_col78
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_14_col79]: [QM31; 1] = (*partial_ec_mul_output_limb_14_col79
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_15_col80]: [QM31; 1] = (*partial_ec_mul_output_limb_15_col80
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_16_col81]: [QM31; 1] = (*partial_ec_mul_output_limb_16_col81
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_17_col82]: [QM31; 1] = (*partial_ec_mul_output_limb_17_col82
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_18_col83]: [QM31; 1] = (*partial_ec_mul_output_limb_18_col83
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_19_col84]: [QM31; 1] = (*partial_ec_mul_output_limb_19_col84
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_20_col85]: [QM31; 1] = (*partial_ec_mul_output_limb_20_col85
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_21_col86]: [QM31; 1] = (*partial_ec_mul_output_limb_21_col86
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_22_col87]: [QM31; 1] = (*partial_ec_mul_output_limb_22_col87
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_23_col88]: [QM31; 1] = (*partial_ec_mul_output_limb_23_col88
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_24_col89]: [QM31; 1] = (*partial_ec_mul_output_limb_24_col89
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_25_col90]: [QM31; 1] = (*partial_ec_mul_output_limb_25_col90
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_26_col91]: [QM31; 1] = (*partial_ec_mul_output_limb_26_col91
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_27_col92]: [QM31; 1] = (*partial_ec_mul_output_limb_27_col92
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_28_col93]: [QM31; 1] = (*partial_ec_mul_output_limb_28_col93
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_29_col94]: [QM31; 1] = (*partial_ec_mul_output_limb_29_col94
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_30_col95]: [QM31; 1] = (*partial_ec_mul_output_limb_30_col95
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_31_col96]: [QM31; 1] = (*partial_ec_mul_output_limb_31_col96
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_32_col97]: [QM31; 1] = (*partial_ec_mul_output_limb_32_col97
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_33_col98]: [QM31; 1] = (*partial_ec_mul_output_limb_33_col98
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_34_col99]: [QM31; 1] = (*partial_ec_mul_output_limb_34_col99
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_35_col100]: [QM31; 1] =
            (*partial_ec_mul_output_limb_35_col100
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_36_col101]: [QM31; 1] =
            (*partial_ec_mul_output_limb_36_col101
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_37_col102]: [QM31; 1] =
            (*partial_ec_mul_output_limb_37_col102
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_38_col103]: [QM31; 1] =
            (*partial_ec_mul_output_limb_38_col103
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_39_col104]: [QM31; 1] =
            (*partial_ec_mul_output_limb_39_col104
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_40_col105]: [QM31; 1] =
            (*partial_ec_mul_output_limb_40_col105
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_41_col106]: [QM31; 1] =
            (*partial_ec_mul_output_limb_41_col106
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_42_col107]: [QM31; 1] =
            (*partial_ec_mul_output_limb_42_col107
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_43_col108]: [QM31; 1] =
            (*partial_ec_mul_output_limb_43_col108
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_44_col109]: [QM31; 1] =
            (*partial_ec_mul_output_limb_44_col109
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_45_col110]: [QM31; 1] =
            (*partial_ec_mul_output_limb_45_col110
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_46_col111]: [QM31; 1] =
            (*partial_ec_mul_output_limb_46_col111
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_47_col112]: [QM31; 1] =
            (*partial_ec_mul_output_limb_47_col112
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_48_col113]: [QM31; 1] =
            (*partial_ec_mul_output_limb_48_col113
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_49_col114]: [QM31; 1] =
            (*partial_ec_mul_output_limb_49_col114
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_50_col115]: [QM31; 1] =
            (*partial_ec_mul_output_limb_50_col115
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_51_col116]: [QM31; 1] =
            (*partial_ec_mul_output_limb_51_col116
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_52_col117]: [QM31; 1] =
            (*partial_ec_mul_output_limb_52_col117
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_53_col118]: [QM31; 1] =
            (*partial_ec_mul_output_limb_53_col118
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_54_col119]: [QM31; 1] =
            (*partial_ec_mul_output_limb_54_col119
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_55_col120]: [QM31; 1] =
            (*partial_ec_mul_output_limb_55_col120
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_56_col121]: [QM31; 1] =
            (*partial_ec_mul_output_limb_56_col121
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_57_col122]: [QM31; 1] =
            (*partial_ec_mul_output_limb_57_col122
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_58_col123]: [QM31; 1] =
            (*partial_ec_mul_output_limb_58_col123
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_59_col124]: [QM31; 1] =
            (*partial_ec_mul_output_limb_59_col124
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_60_col125]: [QM31; 1] =
            (*partial_ec_mul_output_limb_60_col125
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_61_col126]: [QM31; 1] =
            (*partial_ec_mul_output_limb_61_col126
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_62_col127]: [QM31; 1] =
            (*partial_ec_mul_output_limb_62_col127
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_63_col128]: [QM31; 1] =
            (*partial_ec_mul_output_limb_63_col128
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_64_col129]: [QM31; 1] =
            (*partial_ec_mul_output_limb_64_col129
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_65_col130]: [QM31; 1] =
            (*partial_ec_mul_output_limb_65_col130
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_66_col131]: [QM31; 1] =
            (*partial_ec_mul_output_limb_66_col131
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_67_col132]: [QM31; 1] =
            (*partial_ec_mul_output_limb_67_col132
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_68_col133]: [QM31; 1] =
            (*partial_ec_mul_output_limb_68_col133
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_69_col134]: [QM31; 1] =
            (*partial_ec_mul_output_limb_69_col134
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_0_col135]: [QM31; 1] = (*partial_ec_mul_output_limb_0_col135
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_1_col136]: [QM31; 1] = (*partial_ec_mul_output_limb_1_col136
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_2_col137]: [QM31; 1] = (*partial_ec_mul_output_limb_2_col137
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_3_col138]: [QM31; 1] = (*partial_ec_mul_output_limb_3_col138
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_4_col139]: [QM31; 1] = (*partial_ec_mul_output_limb_4_col139
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_5_col140]: [QM31; 1] = (*partial_ec_mul_output_limb_5_col140
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_6_col141]: [QM31; 1] = (*partial_ec_mul_output_limb_6_col141
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_7_col142]: [QM31; 1] = (*partial_ec_mul_output_limb_7_col142
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_8_col143]: [QM31; 1] = (*partial_ec_mul_output_limb_8_col143
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_9_col144]: [QM31; 1] = (*partial_ec_mul_output_limb_9_col144
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_10_col145]: [QM31; 1] =
            (*partial_ec_mul_output_limb_10_col145
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_11_col146]: [QM31; 1] =
            (*partial_ec_mul_output_limb_11_col146
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_12_col147]: [QM31; 1] =
            (*partial_ec_mul_output_limb_12_col147
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_13_col148]: [QM31; 1] =
            (*partial_ec_mul_output_limb_13_col148
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_14_col149]: [QM31; 1] =
            (*partial_ec_mul_output_limb_14_col149
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_15_col150]: [QM31; 1] =
            (*partial_ec_mul_output_limb_15_col150
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_16_col151]: [QM31; 1] =
            (*partial_ec_mul_output_limb_16_col151
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_17_col152]: [QM31; 1] =
            (*partial_ec_mul_output_limb_17_col152
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_18_col153]: [QM31; 1] =
            (*partial_ec_mul_output_limb_18_col153
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_19_col154]: [QM31; 1] =
            (*partial_ec_mul_output_limb_19_col154
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_20_col155]: [QM31; 1] =
            (*partial_ec_mul_output_limb_20_col155
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_21_col156]: [QM31; 1] =
            (*partial_ec_mul_output_limb_21_col156
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_22_col157]: [QM31; 1] =
            (*partial_ec_mul_output_limb_22_col157
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_23_col158]: [QM31; 1] =
            (*partial_ec_mul_output_limb_23_col158
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_24_col159]: [QM31; 1] =
            (*partial_ec_mul_output_limb_24_col159
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_25_col160]: [QM31; 1] =
            (*partial_ec_mul_output_limb_25_col160
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_26_col161]: [QM31; 1] =
            (*partial_ec_mul_output_limb_26_col161
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_27_col162]: [QM31; 1] =
            (*partial_ec_mul_output_limb_27_col162
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_28_col163]: [QM31; 1] =
            (*partial_ec_mul_output_limb_28_col163
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_29_col164]: [QM31; 1] =
            (*partial_ec_mul_output_limb_29_col164
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_30_col165]: [QM31; 1] =
            (*partial_ec_mul_output_limb_30_col165
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_31_col166]: [QM31; 1] =
            (*partial_ec_mul_output_limb_31_col166
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_32_col167]: [QM31; 1] =
            (*partial_ec_mul_output_limb_32_col167
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_33_col168]: [QM31; 1] =
            (*partial_ec_mul_output_limb_33_col168
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_34_col169]: [QM31; 1] =
            (*partial_ec_mul_output_limb_34_col169
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_35_col170]: [QM31; 1] =
            (*partial_ec_mul_output_limb_35_col170
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_36_col171]: [QM31; 1] =
            (*partial_ec_mul_output_limb_36_col171
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_37_col172]: [QM31; 1] =
            (*partial_ec_mul_output_limb_37_col172
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_38_col173]: [QM31; 1] =
            (*partial_ec_mul_output_limb_38_col173
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_39_col174]: [QM31; 1] =
            (*partial_ec_mul_output_limb_39_col174
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_40_col175]: [QM31; 1] =
            (*partial_ec_mul_output_limb_40_col175
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_41_col176]: [QM31; 1] =
            (*partial_ec_mul_output_limb_41_col176
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_42_col177]: [QM31; 1] =
            (*partial_ec_mul_output_limb_42_col177
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_43_col178]: [QM31; 1] =
            (*partial_ec_mul_output_limb_43_col178
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_44_col179]: [QM31; 1] =
            (*partial_ec_mul_output_limb_44_col179
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_45_col180]: [QM31; 1] =
            (*partial_ec_mul_output_limb_45_col180
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_46_col181]: [QM31; 1] =
            (*partial_ec_mul_output_limb_46_col181
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_47_col182]: [QM31; 1] =
            (*partial_ec_mul_output_limb_47_col182
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_48_col183]: [QM31; 1] =
            (*partial_ec_mul_output_limb_48_col183
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_49_col184]: [QM31; 1] =
            (*partial_ec_mul_output_limb_49_col184
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_50_col185]: [QM31; 1] =
            (*partial_ec_mul_output_limb_50_col185
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_51_col186]: [QM31; 1] =
            (*partial_ec_mul_output_limb_51_col186
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_52_col187]: [QM31; 1] =
            (*partial_ec_mul_output_limb_52_col187
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_53_col188]: [QM31; 1] =
            (*partial_ec_mul_output_limb_53_col188
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_54_col189]: [QM31; 1] =
            (*partial_ec_mul_output_limb_54_col189
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_55_col190]: [QM31; 1] =
            (*partial_ec_mul_output_limb_55_col190
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_56_col191]: [QM31; 1] =
            (*partial_ec_mul_output_limb_56_col191
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_57_col192]: [QM31; 1] =
            (*partial_ec_mul_output_limb_57_col192
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_58_col193]: [QM31; 1] =
            (*partial_ec_mul_output_limb_58_col193
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_59_col194]: [QM31; 1] =
            (*partial_ec_mul_output_limb_59_col194
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_60_col195]: [QM31; 1] =
            (*partial_ec_mul_output_limb_60_col195
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_61_col196]: [QM31; 1] =
            (*partial_ec_mul_output_limb_61_col196
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_62_col197]: [QM31; 1] =
            (*partial_ec_mul_output_limb_62_col197
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_63_col198]: [QM31; 1] =
            (*partial_ec_mul_output_limb_63_col198
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_64_col199]: [QM31; 1] =
            (*partial_ec_mul_output_limb_64_col199
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_65_col200]: [QM31; 1] =
            (*partial_ec_mul_output_limb_65_col200
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_66_col201]: [QM31; 1] =
            (*partial_ec_mul_output_limb_66_col201
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_67_col202]: [QM31; 1] =
            (*partial_ec_mul_output_limb_67_col202
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_68_col203]: [QM31; 1] =
            (*partial_ec_mul_output_limb_68_col203
            .try_into()
            .unwrap())
            .unbox();
        let [partial_ec_mul_output_limb_69_col204]: [QM31; 1] =
            (*partial_ec_mul_output_limb_69_col204
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_aggregator_multiplicity]: [QM31; 1] = (*pedersen_aggregator_multiplicity
            .try_into()
            .unwrap())
            .unbox();

        core::internal::revoke_ap_tracking();

        read_positive_known_id_num_bits_252_evaluate(
            input_limb_0_col0,
            value_limb_0_col3,
            value_limb_1_col4,
            value_limb_2_col5,
            value_limb_3_col6,
            value_limb_4_col7,
            value_limb_5_col8,
            value_limb_6_col9,
            value_limb_7_col10,
            value_limb_8_col11,
            value_limb_9_col12,
            value_limb_10_col13,
            value_limb_11_col14,
            value_limb_12_col15,
            value_limb_13_col16,
            value_limb_14_col17,
            value_limb_15_col18,
            value_limb_16_col19,
            value_limb_17_col20,
            value_limb_18_col21,
            value_limb_19_col22,
            value_limb_20_col23,
            value_limb_21_col24,
            value_limb_22_col25,
            value_limb_23_col26,
            value_limb_24_col27,
            value_limb_25_col28,
            value_limb_26_col29,
            value_limb_27_col30,
            self.memory_id_to_big_lookup_elements,
            ref memory_id_to_big_sum_0,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        read_positive_known_id_num_bits_252_evaluate(
            input_limb_1_col1,
            value_limb_0_col31,
            value_limb_1_col32,
            value_limb_2_col33,
            value_limb_3_col34,
            value_limb_4_col35,
            value_limb_5_col36,
            value_limb_6_col37,
            value_limb_7_col38,
            value_limb_8_col39,
            value_limb_9_col40,
            value_limb_10_col41,
            value_limb_11_col42,
            value_limb_12_col43,
            value_limb_13_col44,
            value_limb_14_col45,
            value_limb_15_col46,
            value_limb_16_col47,
            value_limb_17_col48,
            value_limb_18_col49,
            value_limb_19_col50,
            value_limb_20_col51,
            value_limb_21_col52,
            value_limb_22_col53,
            value_limb_23_col54,
            value_limb_24_col55,
            value_limb_25_col56,
            value_limb_26_col57,
            value_limb_27_col58,
            self.memory_id_to_big_lookup_elements,
            ref memory_id_to_big_sum_1,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        verify_reduced_252_evaluate(
            [
                value_limb_0_col3, value_limb_1_col4, value_limb_2_col5, value_limb_3_col6,
                value_limb_4_col7, value_limb_5_col8, value_limb_6_col9, value_limb_7_col10,
                value_limb_8_col11, value_limb_9_col12, value_limb_10_col13, value_limb_11_col14,
                value_limb_12_col15, value_limb_13_col16, value_limb_14_col17, value_limb_15_col18,
                value_limb_16_col19, value_limb_17_col20, value_limb_18_col21, value_limb_19_col22,
                value_limb_20_col23, value_limb_21_col24, value_limb_22_col25, value_limb_23_col26,
                value_limb_24_col27, value_limb_25_col28, value_limb_26_col29, value_limb_27_col30,
            ],
            ms_limb_is_max_col59,
            ms_and_mid_limbs_are_max_col60,
            rc_input_col61,
            self.range_check_8_lookup_elements,
            ref range_check_8_sum_2,
            ref range_check_8_sum_3,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        verify_reduced_252_evaluate(
            [
                value_limb_0_col31, value_limb_1_col32, value_limb_2_col33, value_limb_3_col34,
                value_limb_4_col35, value_limb_5_col36, value_limb_6_col37, value_limb_7_col38,
                value_limb_8_col39, value_limb_9_col40, value_limb_10_col41, value_limb_11_col42,
                value_limb_12_col43, value_limb_13_col44, value_limb_14_col45, value_limb_15_col46,
                value_limb_16_col47, value_limb_17_col48, value_limb_18_col49, value_limb_19_col50,
                value_limb_20_col51, value_limb_21_col52, value_limb_22_col53, value_limb_23_col54,
                value_limb_24_col55, value_limb_25_col56, value_limb_26_col57, value_limb_27_col58,
            ],
            ms_limb_is_max_col62,
            ms_and_mid_limbs_are_max_col63,
            rc_input_col64,
            self.range_check_8_lookup_elements,
            ref range_check_8_sum_4,
            ref range_check_8_sum_5,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let partial_ec_mul_chain_tmp_tmp_c48a1_8: QM31 = (seq * qm31_const::<2, 0, 0, 0>());

        partial_ec_mul_sum_6 = self
            .partial_ec_mul_lookup_elements
            .combine_qm31(
                [
                    partial_ec_mul_chain_tmp_tmp_c48a1_8, qm31_const::<0, 0, 0, 0>(),
                    (value_limb_0_col3 + (value_limb_1_col4 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_2_col5 + (value_limb_3_col6 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_4_col7 + (value_limb_5_col8 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_6_col9 + (value_limb_7_col10 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_8_col11 + (value_limb_9_col12 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_10_col13 + (value_limb_11_col14 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_12_col15 + (value_limb_13_col16 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_14_col17 + (value_limb_15_col18 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_16_col19 + (value_limb_17_col20 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_18_col21 + (value_limb_19_col22 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_20_col23 + (value_limb_21_col24 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_22_col25 + (value_limb_23_col26 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_24_col27 + (value_limb_25_col28 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_26_col29 + (value_limb_27_col30 * qm31_const::<512, 0, 0, 0>())),
                    qm31_const::<510, 0, 0, 0>(), qm31_const::<315, 0, 0, 0>(),
                    qm31_const::<208, 0, 0, 0>(), qm31_const::<480, 0, 0, 0>(),
                    qm31_const::<418, 0, 0, 0>(), qm31_const::<115, 0, 0, 0>(),
                    qm31_const::<155, 0, 0, 0>(), qm31_const::<54, 0, 0, 0>(),
                    qm31_const::<162, 0, 0, 0>(), qm31_const::<449, 0, 0, 0>(),
                    qm31_const::<428, 0, 0, 0>(), qm31_const::<466, 0, 0, 0>(),
                    qm31_const::<484, 0, 0, 0>(), qm31_const::<169, 0, 0, 0>(),
                    qm31_const::<497, 0, 0, 0>(), qm31_const::<373, 0, 0, 0>(),
                    qm31_const::<98, 0, 0, 0>(), qm31_const::<64, 0, 0, 0>(),
                    qm31_const::<464, 0, 0, 0>(), qm31_const::<498, 0, 0, 0>(),
                    qm31_const::<124, 0, 0, 0>(), qm31_const::<68, 0, 0, 0>(),
                    qm31_const::<379, 0, 0, 0>(), qm31_const::<140, 0, 0, 0>(),
                    qm31_const::<26, 0, 0, 0>(), qm31_const::<22, 0, 0, 0>(),
                    qm31_const::<135, 0, 0, 0>(), qm31_const::<202, 0, 0, 0>(),
                    qm31_const::<156, 0, 0, 0>(), qm31_const::<120, 0, 0, 0>(),
                    qm31_const::<213, 0, 0, 0>(), qm31_const::<389, 0, 0, 0>(),
                    qm31_const::<377, 0, 0, 0>(), qm31_const::<20, 0, 0, 0>(),
                    qm31_const::<325, 0, 0, 0>(), qm31_const::<303, 0, 0, 0>(),
                    qm31_const::<473, 0, 0, 0>(), qm31_const::<334, 0, 0, 0>(),
                    qm31_const::<223, 0, 0, 0>(), qm31_const::<160, 0, 0, 0>(),
                    qm31_const::<225, 0, 0, 0>(), qm31_const::<297, 0, 0, 0>(),
                    qm31_const::<101, 0, 0, 0>(), qm31_const::<420, 0, 0, 0>(),
                    qm31_const::<377, 0, 0, 0>(), qm31_const::<72, 0, 0, 0>(),
                    qm31_const::<191, 0, 0, 0>(), qm31_const::<49, 0, 0, 0>(),
                    qm31_const::<314, 0, 0, 0>(), qm31_const::<27, 0, 0, 0>(),
                    qm31_const::<199, 0, 0, 0>(), qm31_const::<222, 0, 0, 0>(),
                    qm31_const::<79, 0, 0, 0>(), qm31_const::<97, 0, 0, 0>(),
                    qm31_const::<108, 0, 0, 0>(), qm31_const::<141, 0, 0, 0>(),
                ],
            );

        partial_ec_mul_sum_7 = self
            .partial_ec_mul_lookup_elements
            .combine_qm31(
                [
                    partial_ec_mul_chain_tmp_tmp_c48a1_8, qm31_const::<14, 0, 0, 0>(),
                    partial_ec_mul_output_limb_0_col65, partial_ec_mul_output_limb_1_col66,
                    partial_ec_mul_output_limb_2_col67, partial_ec_mul_output_limb_3_col68,
                    partial_ec_mul_output_limb_4_col69, partial_ec_mul_output_limb_5_col70,
                    partial_ec_mul_output_limb_6_col71, partial_ec_mul_output_limb_7_col72,
                    partial_ec_mul_output_limb_8_col73, partial_ec_mul_output_limb_9_col74,
                    partial_ec_mul_output_limb_10_col75, partial_ec_mul_output_limb_11_col76,
                    partial_ec_mul_output_limb_12_col77, partial_ec_mul_output_limb_13_col78,
                    partial_ec_mul_output_limb_14_col79, partial_ec_mul_output_limb_15_col80,
                    partial_ec_mul_output_limb_16_col81, partial_ec_mul_output_limb_17_col82,
                    partial_ec_mul_output_limb_18_col83, partial_ec_mul_output_limb_19_col84,
                    partial_ec_mul_output_limb_20_col85, partial_ec_mul_output_limb_21_col86,
                    partial_ec_mul_output_limb_22_col87, partial_ec_mul_output_limb_23_col88,
                    partial_ec_mul_output_limb_24_col89, partial_ec_mul_output_limb_25_col90,
                    partial_ec_mul_output_limb_26_col91, partial_ec_mul_output_limb_27_col92,
                    partial_ec_mul_output_limb_28_col93, partial_ec_mul_output_limb_29_col94,
                    partial_ec_mul_output_limb_30_col95, partial_ec_mul_output_limb_31_col96,
                    partial_ec_mul_output_limb_32_col97, partial_ec_mul_output_limb_33_col98,
                    partial_ec_mul_output_limb_34_col99, partial_ec_mul_output_limb_35_col100,
                    partial_ec_mul_output_limb_36_col101, partial_ec_mul_output_limb_37_col102,
                    partial_ec_mul_output_limb_38_col103, partial_ec_mul_output_limb_39_col104,
                    partial_ec_mul_output_limb_40_col105, partial_ec_mul_output_limb_41_col106,
                    partial_ec_mul_output_limb_42_col107, partial_ec_mul_output_limb_43_col108,
                    partial_ec_mul_output_limb_44_col109, partial_ec_mul_output_limb_45_col110,
                    partial_ec_mul_output_limb_46_col111, partial_ec_mul_output_limb_47_col112,
                    partial_ec_mul_output_limb_48_col113, partial_ec_mul_output_limb_49_col114,
                    partial_ec_mul_output_limb_50_col115, partial_ec_mul_output_limb_51_col116,
                    partial_ec_mul_output_limb_52_col117, partial_ec_mul_output_limb_53_col118,
                    partial_ec_mul_output_limb_54_col119, partial_ec_mul_output_limb_55_col120,
                    partial_ec_mul_output_limb_56_col121, partial_ec_mul_output_limb_57_col122,
                    partial_ec_mul_output_limb_58_col123, partial_ec_mul_output_limb_59_col124,
                    partial_ec_mul_output_limb_60_col125, partial_ec_mul_output_limb_61_col126,
                    partial_ec_mul_output_limb_62_col127, partial_ec_mul_output_limb_63_col128,
                    partial_ec_mul_output_limb_64_col129, partial_ec_mul_output_limb_65_col130,
                    partial_ec_mul_output_limb_66_col131, partial_ec_mul_output_limb_67_col132,
                    partial_ec_mul_output_limb_68_col133, partial_ec_mul_output_limb_69_col134,
                ],
            );
        let partial_ec_mul_chain_id_tmp_c48a1_23: QM31 = (partial_ec_mul_chain_tmp_tmp_c48a1_8
            + qm31_const::<1, 0, 0, 0>());

        partial_ec_mul_sum_8 = self
            .partial_ec_mul_lookup_elements
            .combine_qm31(
                [
                    partial_ec_mul_chain_id_tmp_c48a1_23, qm31_const::<14, 0, 0, 0>(),
                    (value_limb_0_col31 + (value_limb_1_col32 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_2_col33 + (value_limb_3_col34 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_4_col35 + (value_limb_5_col36 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_6_col37 + (value_limb_7_col38 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_8_col39 + (value_limb_9_col40 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_10_col41 + (value_limb_11_col42 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_12_col43 + (value_limb_13_col44 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_14_col45 + (value_limb_15_col46 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_16_col47 + (value_limb_17_col48 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_18_col49 + (value_limb_19_col50 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_20_col51 + (value_limb_21_col52 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_22_col53 + (value_limb_23_col54 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_24_col55 + (value_limb_25_col56 * qm31_const::<512, 0, 0, 0>())),
                    (value_limb_26_col57 + (value_limb_27_col58 * qm31_const::<512, 0, 0, 0>())),
                    partial_ec_mul_output_limb_14_col79, partial_ec_mul_output_limb_15_col80,
                    partial_ec_mul_output_limb_16_col81, partial_ec_mul_output_limb_17_col82,
                    partial_ec_mul_output_limb_18_col83, partial_ec_mul_output_limb_19_col84,
                    partial_ec_mul_output_limb_20_col85, partial_ec_mul_output_limb_21_col86,
                    partial_ec_mul_output_limb_22_col87, partial_ec_mul_output_limb_23_col88,
                    partial_ec_mul_output_limb_24_col89, partial_ec_mul_output_limb_25_col90,
                    partial_ec_mul_output_limb_26_col91, partial_ec_mul_output_limb_27_col92,
                    partial_ec_mul_output_limb_28_col93, partial_ec_mul_output_limb_29_col94,
                    partial_ec_mul_output_limb_30_col95, partial_ec_mul_output_limb_31_col96,
                    partial_ec_mul_output_limb_32_col97, partial_ec_mul_output_limb_33_col98,
                    partial_ec_mul_output_limb_34_col99, partial_ec_mul_output_limb_35_col100,
                    partial_ec_mul_output_limb_36_col101, partial_ec_mul_output_limb_37_col102,
                    partial_ec_mul_output_limb_38_col103, partial_ec_mul_output_limb_39_col104,
                    partial_ec_mul_output_limb_40_col105, partial_ec_mul_output_limb_41_col106,
                    partial_ec_mul_output_limb_42_col107, partial_ec_mul_output_limb_43_col108,
                    partial_ec_mul_output_limb_44_col109, partial_ec_mul_output_limb_45_col110,
                    partial_ec_mul_output_limb_46_col111, partial_ec_mul_output_limb_47_col112,
                    partial_ec_mul_output_limb_48_col113, partial_ec_mul_output_limb_49_col114,
                    partial_ec_mul_output_limb_50_col115, partial_ec_mul_output_limb_51_col116,
                    partial_ec_mul_output_limb_52_col117, partial_ec_mul_output_limb_53_col118,
                    partial_ec_mul_output_limb_54_col119, partial_ec_mul_output_limb_55_col120,
                    partial_ec_mul_output_limb_56_col121, partial_ec_mul_output_limb_57_col122,
                    partial_ec_mul_output_limb_58_col123, partial_ec_mul_output_limb_59_col124,
                    partial_ec_mul_output_limb_60_col125, partial_ec_mul_output_limb_61_col126,
                    partial_ec_mul_output_limb_62_col127, partial_ec_mul_output_limb_63_col128,
                    partial_ec_mul_output_limb_64_col129, partial_ec_mul_output_limb_65_col130,
                    partial_ec_mul_output_limb_66_col131, partial_ec_mul_output_limb_67_col132,
                    partial_ec_mul_output_limb_68_col133, partial_ec_mul_output_limb_69_col134,
                ],
            );

        partial_ec_mul_sum_9 = self
            .partial_ec_mul_lookup_elements
            .combine_qm31(
                [
                    partial_ec_mul_chain_id_tmp_c48a1_23, qm31_const::<28, 0, 0, 0>(),
                    partial_ec_mul_output_limb_0_col135, partial_ec_mul_output_limb_1_col136,
                    partial_ec_mul_output_limb_2_col137, partial_ec_mul_output_limb_3_col138,
                    partial_ec_mul_output_limb_4_col139, partial_ec_mul_output_limb_5_col140,
                    partial_ec_mul_output_limb_6_col141, partial_ec_mul_output_limb_7_col142,
                    partial_ec_mul_output_limb_8_col143, partial_ec_mul_output_limb_9_col144,
                    partial_ec_mul_output_limb_10_col145, partial_ec_mul_output_limb_11_col146,
                    partial_ec_mul_output_limb_12_col147, partial_ec_mul_output_limb_13_col148,
                    partial_ec_mul_output_limb_14_col149, partial_ec_mul_output_limb_15_col150,
                    partial_ec_mul_output_limb_16_col151, partial_ec_mul_output_limb_17_col152,
                    partial_ec_mul_output_limb_18_col153, partial_ec_mul_output_limb_19_col154,
                    partial_ec_mul_output_limb_20_col155, partial_ec_mul_output_limb_21_col156,
                    partial_ec_mul_output_limb_22_col157, partial_ec_mul_output_limb_23_col158,
                    partial_ec_mul_output_limb_24_col159, partial_ec_mul_output_limb_25_col160,
                    partial_ec_mul_output_limb_26_col161, partial_ec_mul_output_limb_27_col162,
                    partial_ec_mul_output_limb_28_col163, partial_ec_mul_output_limb_29_col164,
                    partial_ec_mul_output_limb_30_col165, partial_ec_mul_output_limb_31_col166,
                    partial_ec_mul_output_limb_32_col167, partial_ec_mul_output_limb_33_col168,
                    partial_ec_mul_output_limb_34_col169, partial_ec_mul_output_limb_35_col170,
                    partial_ec_mul_output_limb_36_col171, partial_ec_mul_output_limb_37_col172,
                    partial_ec_mul_output_limb_38_col173, partial_ec_mul_output_limb_39_col174,
                    partial_ec_mul_output_limb_40_col175, partial_ec_mul_output_limb_41_col176,
                    partial_ec_mul_output_limb_42_col177, partial_ec_mul_output_limb_43_col178,
                    partial_ec_mul_output_limb_44_col179, partial_ec_mul_output_limb_45_col180,
                    partial_ec_mul_output_limb_46_col181, partial_ec_mul_output_limb_47_col182,
                    partial_ec_mul_output_limb_48_col183, partial_ec_mul_output_limb_49_col184,
                    partial_ec_mul_output_limb_50_col185, partial_ec_mul_output_limb_51_col186,
                    partial_ec_mul_output_limb_52_col187, partial_ec_mul_output_limb_53_col188,
                    partial_ec_mul_output_limb_54_col189, partial_ec_mul_output_limb_55_col190,
                    partial_ec_mul_output_limb_56_col191, partial_ec_mul_output_limb_57_col192,
                    partial_ec_mul_output_limb_58_col193, partial_ec_mul_output_limb_59_col194,
                    partial_ec_mul_output_limb_60_col195, partial_ec_mul_output_limb_61_col196,
                    partial_ec_mul_output_limb_62_col197, partial_ec_mul_output_limb_63_col198,
                    partial_ec_mul_output_limb_64_col199, partial_ec_mul_output_limb_65_col200,
                    partial_ec_mul_output_limb_66_col201, partial_ec_mul_output_limb_67_col202,
                    partial_ec_mul_output_limb_68_col203, partial_ec_mul_output_limb_69_col204,
                ],
            );

        memory_id_to_big_sum_10 = self
            .memory_id_to_big_lookup_elements
            .combine_qm31(
                [
                    input_limb_2_col2, partial_ec_mul_output_limb_14_col149,
                    partial_ec_mul_output_limb_15_col150, partial_ec_mul_output_limb_16_col151,
                    partial_ec_mul_output_limb_17_col152, partial_ec_mul_output_limb_18_col153,
                    partial_ec_mul_output_limb_19_col154, partial_ec_mul_output_limb_20_col155,
                    partial_ec_mul_output_limb_21_col156, partial_ec_mul_output_limb_22_col157,
                    partial_ec_mul_output_limb_23_col158, partial_ec_mul_output_limb_24_col159,
                    partial_ec_mul_output_limb_25_col160, partial_ec_mul_output_limb_26_col161,
                    partial_ec_mul_output_limb_27_col162, partial_ec_mul_output_limb_28_col163,
                    partial_ec_mul_output_limb_29_col164, partial_ec_mul_output_limb_30_col165,
                    partial_ec_mul_output_limb_31_col166, partial_ec_mul_output_limb_32_col167,
                    partial_ec_mul_output_limb_33_col168, partial_ec_mul_output_limb_34_col169,
                    partial_ec_mul_output_limb_35_col170, partial_ec_mul_output_limb_36_col171,
                    partial_ec_mul_output_limb_37_col172, partial_ec_mul_output_limb_38_col173,
                    partial_ec_mul_output_limb_39_col174, partial_ec_mul_output_limb_40_col175,
                    partial_ec_mul_output_limb_41_col176,
                ],
            );

        pedersen_aggregator_sum_11 = self
            .pedersen_aggregator_lookup_elements
            .combine_qm31([input_limb_0_col0, input_limb_1_col1, input_limb_2_col2]);

        lookup_constraints(
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
            claimed_sum,
            pedersen_aggregator_multiplicity,
            column_size,
            ref interaction_trace_mask_values,
            memory_id_to_big_sum_0,
            memory_id_to_big_sum_1,
            range_check_8_sum_2,
            range_check_8_sum_3,
            range_check_8_sum_4,
            range_check_8_sum_5,
            partial_ec_mul_sum_6,
            partial_ec_mul_sum_7,
            partial_ec_mul_sum_8,
            partial_ec_mul_sum_9,
            memory_id_to_big_sum_10,
            pedersen_aggregator_sum_11,
        );
    }
}


fn lookup_constraints(
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
    claimed_sum: QM31,
    pedersen_aggregator_multiplicity: QM31,
    column_size: M31,
    ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
    memory_id_to_big_sum_0: QM31,
    memory_id_to_big_sum_1: QM31,
    range_check_8_sum_2: QM31,
    range_check_8_sum_3: QM31,
    range_check_8_sum_4: QM31,
    range_check_8_sum_5: QM31,
    partial_ec_mul_sum_6: QM31,
    partial_ec_mul_sum_7: QM31,
    partial_ec_mul_sum_8: QM31,
    partial_ec_mul_sum_9: QM31,
    memory_id_to_big_sum_10: QM31,
    pedersen_aggregator_sum_11: QM31,
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
        * memory_id_to_big_sum_0
        * memory_id_to_big_sum_1)
        - memory_id_to_big_sum_0
        - memory_id_to_big_sum_1)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7],
    )
        - QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3]))
        * range_check_8_sum_2
        * range_check_8_sum_3)
        - range_check_8_sum_2
        - range_check_8_sum_3)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11],
    )
        - QM31Impl::from_partial_evals([trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7]))
        * range_check_8_sum_4
        * range_check_8_sum_5)
        - range_check_8_sum_4
        - range_check_8_sum_5)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
    )
        - QM31Impl::from_partial_evals([trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11]))
        * partial_ec_mul_sum_6
        * partial_ec_mul_sum_7)
        - partial_ec_mul_sum_6
        + partial_ec_mul_sum_7)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
        ))
        * partial_ec_mul_sum_8
        * partial_ec_mul_sum_9)
        - partial_ec_mul_sum_8
        + partial_ec_mul_sum_9)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23],
    )
        - QM31Impl::from_partial_evals([trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19])
        - QM31Impl::from_partial_evals(
            [trace_2_col20_neg1, trace_2_col21_neg1, trace_2_col22_neg1, trace_2_col23_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * memory_id_to_big_sum_10
        * pedersen_aggregator_sum_11)
        + (memory_id_to_big_sum_10 * pedersen_aggregator_multiplicity)
        - pedersen_aggregator_sum_11)
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
            memory_id_to_big_lookup_elements: make_lookup_elements(
                qm31_const::<844624398, 1166453613, 1247584074, 330174372>(),
                qm31_const::<1844105245, 1400976933, 1126903288, 1155460729>(),
            ),
            partial_ec_mul_lookup_elements: make_lookup_elements(
                qm31_const::<1649646149, 853343631, 2092831524, 2004475967>(),
                qm31_const::<566949925, 426542195, 926007664, 380048330>(),
            ),
            pedersen_aggregator_lookup_elements: make_lookup_elements(
                qm31_const::<920417564, 1680486498, 1628630402, 353948678>(),
                qm31_const::<846637634, 1325318444, 1529670858, 731974051>(),
            ),
            range_check_8_lookup_elements: make_lookup_elements(
                qm31_const::<1180316345, 706098445, 2005498950, 439840985>(),
                qm31_const::<1338115896, 1708611778, 1362220287, 779911332>(),
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
        assert_eq!(sum, QM31Trait::from_fixed_array(PEDERSEN_AGGREGATOR_SAMPLE_EVAL_RESULT))
    }
}
