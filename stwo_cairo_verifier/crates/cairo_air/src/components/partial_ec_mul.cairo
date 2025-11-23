// This file was created by the AIR team.

use crate::components::subroutines::ec_add::ec_add_evaluate;
use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 297;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 18] = [
    ('PedersenPointsTable', 1), ('RangeCheck_9_9', 6), ('RangeCheck_9_9_B', 6),
    ('RangeCheck_9_9_C', 6), ('RangeCheck_9_9_D', 6), ('RangeCheck_9_9_E', 6),
    ('RangeCheck_9_9_F', 6), ('RangeCheck_9_9_G', 3), ('RangeCheck_9_9_H', 3),
    ('RangeCheck_20', 12), ('RangeCheck_20_B', 12), ('RangeCheck_20_C', 12),
    ('RangeCheck_20_D', 12), ('RangeCheck_20_E', 9), ('RangeCheck_20_F', 9), ('RangeCheck_20_G', 9),
    ('RangeCheck_20_H', 9), ('PartialEcMul', 1),
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
        let interaction_log_sizes = [log_size; 260].span();
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
    pub pedersen_points_table_lookup_elements: crate::PedersenPointsTableElements,
    pub range_check_9_9_lookup_elements: crate::RangeCheck_9_9Elements,
    pub range_check_9_9_b_lookup_elements: crate::RangeCheck_9_9_BElements,
    pub range_check_9_9_c_lookup_elements: crate::RangeCheck_9_9_CElements,
    pub range_check_9_9_d_lookup_elements: crate::RangeCheck_9_9_DElements,
    pub range_check_9_9_e_lookup_elements: crate::RangeCheck_9_9_EElements,
    pub range_check_9_9_f_lookup_elements: crate::RangeCheck_9_9_FElements,
    pub range_check_9_9_g_lookup_elements: crate::RangeCheck_9_9_GElements,
    pub range_check_9_9_h_lookup_elements: crate::RangeCheck_9_9_HElements,
    pub range_check_20_lookup_elements: crate::RangeCheck_20Elements,
    pub range_check_20_b_lookup_elements: crate::RangeCheck_20_BElements,
    pub range_check_20_c_lookup_elements: crate::RangeCheck_20_CElements,
    pub range_check_20_d_lookup_elements: crate::RangeCheck_20_DElements,
    pub range_check_20_e_lookup_elements: crate::RangeCheck_20_EElements,
    pub range_check_20_f_lookup_elements: crate::RangeCheck_20_FElements,
    pub range_check_20_g_lookup_elements: crate::RangeCheck_20_GElements,
    pub range_check_20_h_lookup_elements: crate::RangeCheck_20_HElements,
    pub partial_ec_mul_lookup_elements: crate::PartialEcMulElements,
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
            pedersen_points_table_lookup_elements: interaction_elements
                .pedersen_points_table
                .clone(),
            range_check_9_9_lookup_elements: interaction_elements.range_checks.rc_9_9.clone(),
            range_check_9_9_b_lookup_elements: interaction_elements.range_checks.rc_9_9_b.clone(),
            range_check_9_9_c_lookup_elements: interaction_elements.range_checks.rc_9_9_c.clone(),
            range_check_9_9_d_lookup_elements: interaction_elements.range_checks.rc_9_9_d.clone(),
            range_check_9_9_e_lookup_elements: interaction_elements.range_checks.rc_9_9_e.clone(),
            range_check_9_9_f_lookup_elements: interaction_elements.range_checks.rc_9_9_f.clone(),
            range_check_9_9_g_lookup_elements: interaction_elements.range_checks.rc_9_9_g.clone(),
            range_check_9_9_h_lookup_elements: interaction_elements.range_checks.rc_9_9_h.clone(),
            range_check_20_lookup_elements: interaction_elements.range_checks.rc_20.clone(),
            range_check_20_b_lookup_elements: interaction_elements.range_checks.rc_20_b.clone(),
            range_check_20_c_lookup_elements: interaction_elements.range_checks.rc_20_c.clone(),
            range_check_20_d_lookup_elements: interaction_elements.range_checks.rc_20_d.clone(),
            range_check_20_e_lookup_elements: interaction_elements.range_checks.rc_20_e.clone(),
            range_check_20_f_lookup_elements: interaction_elements.range_checks.rc_20_f.clone(),
            range_check_20_g_lookup_elements: interaction_elements.range_checks.rc_20_g.clone(),
            range_check_20_h_lookup_elements: interaction_elements.range_checks.rc_20_h.clone(),
            partial_ec_mul_lookup_elements: interaction_elements.partial_ec_mul.clone(),
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
        let mut pedersen_points_table_sum_0: QM31 = Zero::zero();
        let mut range_check_9_9_sum_1: QM31 = Zero::zero();
        let mut range_check_9_9_b_sum_2: QM31 = Zero::zero();
        let mut range_check_9_9_c_sum_3: QM31 = Zero::zero();
        let mut range_check_9_9_d_sum_4: QM31 = Zero::zero();
        let mut range_check_9_9_e_sum_5: QM31 = Zero::zero();
        let mut range_check_9_9_f_sum_6: QM31 = Zero::zero();
        let mut range_check_9_9_g_sum_7: QM31 = Zero::zero();
        let mut range_check_9_9_h_sum_8: QM31 = Zero::zero();
        let mut range_check_9_9_sum_9: QM31 = Zero::zero();
        let mut range_check_9_9_b_sum_10: QM31 = Zero::zero();
        let mut range_check_9_9_c_sum_11: QM31 = Zero::zero();
        let mut range_check_9_9_d_sum_12: QM31 = Zero::zero();
        let mut range_check_9_9_e_sum_13: QM31 = Zero::zero();
        let mut range_check_9_9_f_sum_14: QM31 = Zero::zero();
        let mut range_check_20_sum_15: QM31 = Zero::zero();
        let mut range_check_20_b_sum_16: QM31 = Zero::zero();
        let mut range_check_20_c_sum_17: QM31 = Zero::zero();
        let mut range_check_20_d_sum_18: QM31 = Zero::zero();
        let mut range_check_20_e_sum_19: QM31 = Zero::zero();
        let mut range_check_20_f_sum_20: QM31 = Zero::zero();
        let mut range_check_20_g_sum_21: QM31 = Zero::zero();
        let mut range_check_20_h_sum_22: QM31 = Zero::zero();
        let mut range_check_20_sum_23: QM31 = Zero::zero();
        let mut range_check_20_b_sum_24: QM31 = Zero::zero();
        let mut range_check_20_c_sum_25: QM31 = Zero::zero();
        let mut range_check_20_d_sum_26: QM31 = Zero::zero();
        let mut range_check_20_e_sum_27: QM31 = Zero::zero();
        let mut range_check_20_f_sum_28: QM31 = Zero::zero();
        let mut range_check_20_g_sum_29: QM31 = Zero::zero();
        let mut range_check_20_h_sum_30: QM31 = Zero::zero();
        let mut range_check_20_sum_31: QM31 = Zero::zero();
        let mut range_check_20_b_sum_32: QM31 = Zero::zero();
        let mut range_check_20_c_sum_33: QM31 = Zero::zero();
        let mut range_check_20_d_sum_34: QM31 = Zero::zero();
        let mut range_check_20_e_sum_35: QM31 = Zero::zero();
        let mut range_check_20_f_sum_36: QM31 = Zero::zero();
        let mut range_check_20_g_sum_37: QM31 = Zero::zero();
        let mut range_check_20_h_sum_38: QM31 = Zero::zero();
        let mut range_check_20_sum_39: QM31 = Zero::zero();
        let mut range_check_20_b_sum_40: QM31 = Zero::zero();
        let mut range_check_20_c_sum_41: QM31 = Zero::zero();
        let mut range_check_20_d_sum_42: QM31 = Zero::zero();
        let mut range_check_9_9_sum_43: QM31 = Zero::zero();
        let mut range_check_9_9_b_sum_44: QM31 = Zero::zero();
        let mut range_check_9_9_c_sum_45: QM31 = Zero::zero();
        let mut range_check_9_9_d_sum_46: QM31 = Zero::zero();
        let mut range_check_9_9_e_sum_47: QM31 = Zero::zero();
        let mut range_check_9_9_f_sum_48: QM31 = Zero::zero();
        let mut range_check_9_9_g_sum_49: QM31 = Zero::zero();
        let mut range_check_9_9_h_sum_50: QM31 = Zero::zero();
        let mut range_check_9_9_sum_51: QM31 = Zero::zero();
        let mut range_check_9_9_b_sum_52: QM31 = Zero::zero();
        let mut range_check_9_9_c_sum_53: QM31 = Zero::zero();
        let mut range_check_9_9_d_sum_54: QM31 = Zero::zero();
        let mut range_check_9_9_e_sum_55: QM31 = Zero::zero();
        let mut range_check_9_9_f_sum_56: QM31 = Zero::zero();
        let mut range_check_20_sum_57: QM31 = Zero::zero();
        let mut range_check_20_b_sum_58: QM31 = Zero::zero();
        let mut range_check_20_c_sum_59: QM31 = Zero::zero();
        let mut range_check_20_d_sum_60: QM31 = Zero::zero();
        let mut range_check_20_e_sum_61: QM31 = Zero::zero();
        let mut range_check_20_f_sum_62: QM31 = Zero::zero();
        let mut range_check_20_g_sum_63: QM31 = Zero::zero();
        let mut range_check_20_h_sum_64: QM31 = Zero::zero();
        let mut range_check_20_sum_65: QM31 = Zero::zero();
        let mut range_check_20_b_sum_66: QM31 = Zero::zero();
        let mut range_check_20_c_sum_67: QM31 = Zero::zero();
        let mut range_check_20_d_sum_68: QM31 = Zero::zero();
        let mut range_check_20_e_sum_69: QM31 = Zero::zero();
        let mut range_check_20_f_sum_70: QM31 = Zero::zero();
        let mut range_check_20_g_sum_71: QM31 = Zero::zero();
        let mut range_check_20_h_sum_72: QM31 = Zero::zero();
        let mut range_check_20_sum_73: QM31 = Zero::zero();
        let mut range_check_20_b_sum_74: QM31 = Zero::zero();
        let mut range_check_20_c_sum_75: QM31 = Zero::zero();
        let mut range_check_20_d_sum_76: QM31 = Zero::zero();
        let mut range_check_20_e_sum_77: QM31 = Zero::zero();
        let mut range_check_20_f_sum_78: QM31 = Zero::zero();
        let mut range_check_20_g_sum_79: QM31 = Zero::zero();
        let mut range_check_20_h_sum_80: QM31 = Zero::zero();
        let mut range_check_20_sum_81: QM31 = Zero::zero();
        let mut range_check_20_b_sum_82: QM31 = Zero::zero();
        let mut range_check_20_c_sum_83: QM31 = Zero::zero();
        let mut range_check_20_d_sum_84: QM31 = Zero::zero();
        let mut range_check_9_9_sum_85: QM31 = Zero::zero();
        let mut range_check_9_9_b_sum_86: QM31 = Zero::zero();
        let mut range_check_9_9_c_sum_87: QM31 = Zero::zero();
        let mut range_check_9_9_d_sum_88: QM31 = Zero::zero();
        let mut range_check_9_9_e_sum_89: QM31 = Zero::zero();
        let mut range_check_9_9_f_sum_90: QM31 = Zero::zero();
        let mut range_check_9_9_g_sum_91: QM31 = Zero::zero();
        let mut range_check_9_9_h_sum_92: QM31 = Zero::zero();
        let mut range_check_9_9_sum_93: QM31 = Zero::zero();
        let mut range_check_9_9_b_sum_94: QM31 = Zero::zero();
        let mut range_check_9_9_c_sum_95: QM31 = Zero::zero();
        let mut range_check_9_9_d_sum_96: QM31 = Zero::zero();
        let mut range_check_9_9_e_sum_97: QM31 = Zero::zero();
        let mut range_check_9_9_f_sum_98: QM31 = Zero::zero();
        let mut range_check_20_sum_99: QM31 = Zero::zero();
        let mut range_check_20_b_sum_100: QM31 = Zero::zero();
        let mut range_check_20_c_sum_101: QM31 = Zero::zero();
        let mut range_check_20_d_sum_102: QM31 = Zero::zero();
        let mut range_check_20_e_sum_103: QM31 = Zero::zero();
        let mut range_check_20_f_sum_104: QM31 = Zero::zero();
        let mut range_check_20_g_sum_105: QM31 = Zero::zero();
        let mut range_check_20_h_sum_106: QM31 = Zero::zero();
        let mut range_check_20_sum_107: QM31 = Zero::zero();
        let mut range_check_20_b_sum_108: QM31 = Zero::zero();
        let mut range_check_20_c_sum_109: QM31 = Zero::zero();
        let mut range_check_20_d_sum_110: QM31 = Zero::zero();
        let mut range_check_20_e_sum_111: QM31 = Zero::zero();
        let mut range_check_20_f_sum_112: QM31 = Zero::zero();
        let mut range_check_20_g_sum_113: QM31 = Zero::zero();
        let mut range_check_20_h_sum_114: QM31 = Zero::zero();
        let mut range_check_20_sum_115: QM31 = Zero::zero();
        let mut range_check_20_b_sum_116: QM31 = Zero::zero();
        let mut range_check_20_c_sum_117: QM31 = Zero::zero();
        let mut range_check_20_d_sum_118: QM31 = Zero::zero();
        let mut range_check_20_e_sum_119: QM31 = Zero::zero();
        let mut range_check_20_f_sum_120: QM31 = Zero::zero();
        let mut range_check_20_g_sum_121: QM31 = Zero::zero();
        let mut range_check_20_h_sum_122: QM31 = Zero::zero();
        let mut range_check_20_sum_123: QM31 = Zero::zero();
        let mut range_check_20_b_sum_124: QM31 = Zero::zero();
        let mut range_check_20_c_sum_125: QM31 = Zero::zero();
        let mut range_check_20_d_sum_126: QM31 = Zero::zero();
        let mut partial_ec_mul_sum_127: QM31 = Zero::zero();
        let mut partial_ec_mul_sum_128: QM31 = Zero::zero();

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
            input_limb_35_col35,
            input_limb_36_col36,
            input_limb_37_col37,
            input_limb_38_col38,
            input_limb_39_col39,
            input_limb_40_col40,
            input_limb_41_col41,
            input_limb_42_col42,
            input_limb_43_col43,
            input_limb_44_col44,
            input_limb_45_col45,
            input_limb_46_col46,
            input_limb_47_col47,
            input_limb_48_col48,
            input_limb_49_col49,
            input_limb_50_col50,
            input_limb_51_col51,
            input_limb_52_col52,
            input_limb_53_col53,
            input_limb_54_col54,
            input_limb_55_col55,
            input_limb_56_col56,
            input_limb_57_col57,
            input_limb_58_col58,
            input_limb_59_col59,
            input_limb_60_col60,
            input_limb_61_col61,
            input_limb_62_col62,
            input_limb_63_col63,
            input_limb_64_col64,
            input_limb_65_col65,
            input_limb_66_col66,
            input_limb_67_col67,
            input_limb_68_col68,
            input_limb_69_col69,
            input_limb_70_col70,
            input_limb_71_col71,
            pedersen_points_table_output_limb_0_col72,
            pedersen_points_table_output_limb_1_col73,
            pedersen_points_table_output_limb_2_col74,
            pedersen_points_table_output_limb_3_col75,
            pedersen_points_table_output_limb_4_col76,
            pedersen_points_table_output_limb_5_col77,
            pedersen_points_table_output_limb_6_col78,
            pedersen_points_table_output_limb_7_col79,
            pedersen_points_table_output_limb_8_col80,
            pedersen_points_table_output_limb_9_col81,
            pedersen_points_table_output_limb_10_col82,
            pedersen_points_table_output_limb_11_col83,
            pedersen_points_table_output_limb_12_col84,
            pedersen_points_table_output_limb_13_col85,
            pedersen_points_table_output_limb_14_col86,
            pedersen_points_table_output_limb_15_col87,
            pedersen_points_table_output_limb_16_col88,
            pedersen_points_table_output_limb_17_col89,
            pedersen_points_table_output_limb_18_col90,
            pedersen_points_table_output_limb_19_col91,
            pedersen_points_table_output_limb_20_col92,
            pedersen_points_table_output_limb_21_col93,
            pedersen_points_table_output_limb_22_col94,
            pedersen_points_table_output_limb_23_col95,
            pedersen_points_table_output_limb_24_col96,
            pedersen_points_table_output_limb_25_col97,
            pedersen_points_table_output_limb_26_col98,
            pedersen_points_table_output_limb_27_col99,
            pedersen_points_table_output_limb_28_col100,
            pedersen_points_table_output_limb_29_col101,
            pedersen_points_table_output_limb_30_col102,
            pedersen_points_table_output_limb_31_col103,
            pedersen_points_table_output_limb_32_col104,
            pedersen_points_table_output_limb_33_col105,
            pedersen_points_table_output_limb_34_col106,
            pedersen_points_table_output_limb_35_col107,
            pedersen_points_table_output_limb_36_col108,
            pedersen_points_table_output_limb_37_col109,
            pedersen_points_table_output_limb_38_col110,
            pedersen_points_table_output_limb_39_col111,
            pedersen_points_table_output_limb_40_col112,
            pedersen_points_table_output_limb_41_col113,
            pedersen_points_table_output_limb_42_col114,
            pedersen_points_table_output_limb_43_col115,
            pedersen_points_table_output_limb_44_col116,
            pedersen_points_table_output_limb_45_col117,
            pedersen_points_table_output_limb_46_col118,
            pedersen_points_table_output_limb_47_col119,
            pedersen_points_table_output_limb_48_col120,
            pedersen_points_table_output_limb_49_col121,
            pedersen_points_table_output_limb_50_col122,
            pedersen_points_table_output_limb_51_col123,
            pedersen_points_table_output_limb_52_col124,
            pedersen_points_table_output_limb_53_col125,
            pedersen_points_table_output_limb_54_col126,
            pedersen_points_table_output_limb_55_col127,
            slope_limb_0_col128,
            slope_limb_1_col129,
            slope_limb_2_col130,
            slope_limb_3_col131,
            slope_limb_4_col132,
            slope_limb_5_col133,
            slope_limb_6_col134,
            slope_limb_7_col135,
            slope_limb_8_col136,
            slope_limb_9_col137,
            slope_limb_10_col138,
            slope_limb_11_col139,
            slope_limb_12_col140,
            slope_limb_13_col141,
            slope_limb_14_col142,
            slope_limb_15_col143,
            slope_limb_16_col144,
            slope_limb_17_col145,
            slope_limb_18_col146,
            slope_limb_19_col147,
            slope_limb_20_col148,
            slope_limb_21_col149,
            slope_limb_22_col150,
            slope_limb_23_col151,
            slope_limb_24_col152,
            slope_limb_25_col153,
            slope_limb_26_col154,
            slope_limb_27_col155,
            k_col156,
            carry_0_col157,
            carry_1_col158,
            carry_2_col159,
            carry_3_col160,
            carry_4_col161,
            carry_5_col162,
            carry_6_col163,
            carry_7_col164,
            carry_8_col165,
            carry_9_col166,
            carry_10_col167,
            carry_11_col168,
            carry_12_col169,
            carry_13_col170,
            carry_14_col171,
            carry_15_col172,
            carry_16_col173,
            carry_17_col174,
            carry_18_col175,
            carry_19_col176,
            carry_20_col177,
            carry_21_col178,
            carry_22_col179,
            carry_23_col180,
            carry_24_col181,
            carry_25_col182,
            carry_26_col183,
            result_x_limb_0_col184,
            result_x_limb_1_col185,
            result_x_limb_2_col186,
            result_x_limb_3_col187,
            result_x_limb_4_col188,
            result_x_limb_5_col189,
            result_x_limb_6_col190,
            result_x_limb_7_col191,
            result_x_limb_8_col192,
            result_x_limb_9_col193,
            result_x_limb_10_col194,
            result_x_limb_11_col195,
            result_x_limb_12_col196,
            result_x_limb_13_col197,
            result_x_limb_14_col198,
            result_x_limb_15_col199,
            result_x_limb_16_col200,
            result_x_limb_17_col201,
            result_x_limb_18_col202,
            result_x_limb_19_col203,
            result_x_limb_20_col204,
            result_x_limb_21_col205,
            result_x_limb_22_col206,
            result_x_limb_23_col207,
            result_x_limb_24_col208,
            result_x_limb_25_col209,
            result_x_limb_26_col210,
            result_x_limb_27_col211,
            k_col212,
            carry_0_col213,
            carry_1_col214,
            carry_2_col215,
            carry_3_col216,
            carry_4_col217,
            carry_5_col218,
            carry_6_col219,
            carry_7_col220,
            carry_8_col221,
            carry_9_col222,
            carry_10_col223,
            carry_11_col224,
            carry_12_col225,
            carry_13_col226,
            carry_14_col227,
            carry_15_col228,
            carry_16_col229,
            carry_17_col230,
            carry_18_col231,
            carry_19_col232,
            carry_20_col233,
            carry_21_col234,
            carry_22_col235,
            carry_23_col236,
            carry_24_col237,
            carry_25_col238,
            carry_26_col239,
            result_y_limb_0_col240,
            result_y_limb_1_col241,
            result_y_limb_2_col242,
            result_y_limb_3_col243,
            result_y_limb_4_col244,
            result_y_limb_5_col245,
            result_y_limb_6_col246,
            result_y_limb_7_col247,
            result_y_limb_8_col248,
            result_y_limb_9_col249,
            result_y_limb_10_col250,
            result_y_limb_11_col251,
            result_y_limb_12_col252,
            result_y_limb_13_col253,
            result_y_limb_14_col254,
            result_y_limb_15_col255,
            result_y_limb_16_col256,
            result_y_limb_17_col257,
            result_y_limb_18_col258,
            result_y_limb_19_col259,
            result_y_limb_20_col260,
            result_y_limb_21_col261,
            result_y_limb_22_col262,
            result_y_limb_23_col263,
            result_y_limb_24_col264,
            result_y_limb_25_col265,
            result_y_limb_26_col266,
            result_y_limb_27_col267,
            k_col268,
            carry_0_col269,
            carry_1_col270,
            carry_2_col271,
            carry_3_col272,
            carry_4_col273,
            carry_5_col274,
            carry_6_col275,
            carry_7_col276,
            carry_8_col277,
            carry_9_col278,
            carry_10_col279,
            carry_11_col280,
            carry_12_col281,
            carry_13_col282,
            carry_14_col283,
            carry_15_col284,
            carry_16_col285,
            carry_17_col286,
            carry_18_col287,
            carry_19_col288,
            carry_20_col289,
            carry_21_col290,
            carry_22_col291,
            carry_23_col292,
            carry_24_col293,
            carry_25_col294,
            carry_26_col295,
            enabler,
        ]: [Span<QM31>; 297] =
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
        let [input_limb_35_col35]: [QM31; 1] = (*input_limb_35_col35.try_into().unwrap()).unbox();
        let [input_limb_36_col36]: [QM31; 1] = (*input_limb_36_col36.try_into().unwrap()).unbox();
        let [input_limb_37_col37]: [QM31; 1] = (*input_limb_37_col37.try_into().unwrap()).unbox();
        let [input_limb_38_col38]: [QM31; 1] = (*input_limb_38_col38.try_into().unwrap()).unbox();
        let [input_limb_39_col39]: [QM31; 1] = (*input_limb_39_col39.try_into().unwrap()).unbox();
        let [input_limb_40_col40]: [QM31; 1] = (*input_limb_40_col40.try_into().unwrap()).unbox();
        let [input_limb_41_col41]: [QM31; 1] = (*input_limb_41_col41.try_into().unwrap()).unbox();
        let [input_limb_42_col42]: [QM31; 1] = (*input_limb_42_col42.try_into().unwrap()).unbox();
        let [input_limb_43_col43]: [QM31; 1] = (*input_limb_43_col43.try_into().unwrap()).unbox();
        let [input_limb_44_col44]: [QM31; 1] = (*input_limb_44_col44.try_into().unwrap()).unbox();
        let [input_limb_45_col45]: [QM31; 1] = (*input_limb_45_col45.try_into().unwrap()).unbox();
        let [input_limb_46_col46]: [QM31; 1] = (*input_limb_46_col46.try_into().unwrap()).unbox();
        let [input_limb_47_col47]: [QM31; 1] = (*input_limb_47_col47.try_into().unwrap()).unbox();
        let [input_limb_48_col48]: [QM31; 1] = (*input_limb_48_col48.try_into().unwrap()).unbox();
        let [input_limb_49_col49]: [QM31; 1] = (*input_limb_49_col49.try_into().unwrap()).unbox();
        let [input_limb_50_col50]: [QM31; 1] = (*input_limb_50_col50.try_into().unwrap()).unbox();
        let [input_limb_51_col51]: [QM31; 1] = (*input_limb_51_col51.try_into().unwrap()).unbox();
        let [input_limb_52_col52]: [QM31; 1] = (*input_limb_52_col52.try_into().unwrap()).unbox();
        let [input_limb_53_col53]: [QM31; 1] = (*input_limb_53_col53.try_into().unwrap()).unbox();
        let [input_limb_54_col54]: [QM31; 1] = (*input_limb_54_col54.try_into().unwrap()).unbox();
        let [input_limb_55_col55]: [QM31; 1] = (*input_limb_55_col55.try_into().unwrap()).unbox();
        let [input_limb_56_col56]: [QM31; 1] = (*input_limb_56_col56.try_into().unwrap()).unbox();
        let [input_limb_57_col57]: [QM31; 1] = (*input_limb_57_col57.try_into().unwrap()).unbox();
        let [input_limb_58_col58]: [QM31; 1] = (*input_limb_58_col58.try_into().unwrap()).unbox();
        let [input_limb_59_col59]: [QM31; 1] = (*input_limb_59_col59.try_into().unwrap()).unbox();
        let [input_limb_60_col60]: [QM31; 1] = (*input_limb_60_col60.try_into().unwrap()).unbox();
        let [input_limb_61_col61]: [QM31; 1] = (*input_limb_61_col61.try_into().unwrap()).unbox();
        let [input_limb_62_col62]: [QM31; 1] = (*input_limb_62_col62.try_into().unwrap()).unbox();
        let [input_limb_63_col63]: [QM31; 1] = (*input_limb_63_col63.try_into().unwrap()).unbox();
        let [input_limb_64_col64]: [QM31; 1] = (*input_limb_64_col64.try_into().unwrap()).unbox();
        let [input_limb_65_col65]: [QM31; 1] = (*input_limb_65_col65.try_into().unwrap()).unbox();
        let [input_limb_66_col66]: [QM31; 1] = (*input_limb_66_col66.try_into().unwrap()).unbox();
        let [input_limb_67_col67]: [QM31; 1] = (*input_limb_67_col67.try_into().unwrap()).unbox();
        let [input_limb_68_col68]: [QM31; 1] = (*input_limb_68_col68.try_into().unwrap()).unbox();
        let [input_limb_69_col69]: [QM31; 1] = (*input_limb_69_col69.try_into().unwrap()).unbox();
        let [input_limb_70_col70]: [QM31; 1] = (*input_limb_70_col70.try_into().unwrap()).unbox();
        let [input_limb_71_col71]: [QM31; 1] = (*input_limb_71_col71.try_into().unwrap()).unbox();
        let [pedersen_points_table_output_limb_0_col72]: [QM31; 1] =
            (*pedersen_points_table_output_limb_0_col72
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_1_col73]: [QM31; 1] =
            (*pedersen_points_table_output_limb_1_col73
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_2_col74]: [QM31; 1] =
            (*pedersen_points_table_output_limb_2_col74
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_3_col75]: [QM31; 1] =
            (*pedersen_points_table_output_limb_3_col75
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_4_col76]: [QM31; 1] =
            (*pedersen_points_table_output_limb_4_col76
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_5_col77]: [QM31; 1] =
            (*pedersen_points_table_output_limb_5_col77
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_6_col78]: [QM31; 1] =
            (*pedersen_points_table_output_limb_6_col78
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_7_col79]: [QM31; 1] =
            (*pedersen_points_table_output_limb_7_col79
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_8_col80]: [QM31; 1] =
            (*pedersen_points_table_output_limb_8_col80
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_9_col81]: [QM31; 1] =
            (*pedersen_points_table_output_limb_9_col81
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_10_col82]: [QM31; 1] =
            (*pedersen_points_table_output_limb_10_col82
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_11_col83]: [QM31; 1] =
            (*pedersen_points_table_output_limb_11_col83
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_12_col84]: [QM31; 1] =
            (*pedersen_points_table_output_limb_12_col84
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_13_col85]: [QM31; 1] =
            (*pedersen_points_table_output_limb_13_col85
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_14_col86]: [QM31; 1] =
            (*pedersen_points_table_output_limb_14_col86
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_15_col87]: [QM31; 1] =
            (*pedersen_points_table_output_limb_15_col87
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_16_col88]: [QM31; 1] =
            (*pedersen_points_table_output_limb_16_col88
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_17_col89]: [QM31; 1] =
            (*pedersen_points_table_output_limb_17_col89
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_18_col90]: [QM31; 1] =
            (*pedersen_points_table_output_limb_18_col90
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_19_col91]: [QM31; 1] =
            (*pedersen_points_table_output_limb_19_col91
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_20_col92]: [QM31; 1] =
            (*pedersen_points_table_output_limb_20_col92
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_21_col93]: [QM31; 1] =
            (*pedersen_points_table_output_limb_21_col93
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_22_col94]: [QM31; 1] =
            (*pedersen_points_table_output_limb_22_col94
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_23_col95]: [QM31; 1] =
            (*pedersen_points_table_output_limb_23_col95
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_24_col96]: [QM31; 1] =
            (*pedersen_points_table_output_limb_24_col96
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_25_col97]: [QM31; 1] =
            (*pedersen_points_table_output_limb_25_col97
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_26_col98]: [QM31; 1] =
            (*pedersen_points_table_output_limb_26_col98
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_27_col99]: [QM31; 1] =
            (*pedersen_points_table_output_limb_27_col99
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_28_col100]: [QM31; 1] =
            (*pedersen_points_table_output_limb_28_col100
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_29_col101]: [QM31; 1] =
            (*pedersen_points_table_output_limb_29_col101
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_30_col102]: [QM31; 1] =
            (*pedersen_points_table_output_limb_30_col102
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_31_col103]: [QM31; 1] =
            (*pedersen_points_table_output_limb_31_col103
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_32_col104]: [QM31; 1] =
            (*pedersen_points_table_output_limb_32_col104
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_33_col105]: [QM31; 1] =
            (*pedersen_points_table_output_limb_33_col105
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_34_col106]: [QM31; 1] =
            (*pedersen_points_table_output_limb_34_col106
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_35_col107]: [QM31; 1] =
            (*pedersen_points_table_output_limb_35_col107
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_36_col108]: [QM31; 1] =
            (*pedersen_points_table_output_limb_36_col108
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_37_col109]: [QM31; 1] =
            (*pedersen_points_table_output_limb_37_col109
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_38_col110]: [QM31; 1] =
            (*pedersen_points_table_output_limb_38_col110
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_39_col111]: [QM31; 1] =
            (*pedersen_points_table_output_limb_39_col111
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_40_col112]: [QM31; 1] =
            (*pedersen_points_table_output_limb_40_col112
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_41_col113]: [QM31; 1] =
            (*pedersen_points_table_output_limb_41_col113
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_42_col114]: [QM31; 1] =
            (*pedersen_points_table_output_limb_42_col114
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_43_col115]: [QM31; 1] =
            (*pedersen_points_table_output_limb_43_col115
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_44_col116]: [QM31; 1] =
            (*pedersen_points_table_output_limb_44_col116
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_45_col117]: [QM31; 1] =
            (*pedersen_points_table_output_limb_45_col117
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_46_col118]: [QM31; 1] =
            (*pedersen_points_table_output_limb_46_col118
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_47_col119]: [QM31; 1] =
            (*pedersen_points_table_output_limb_47_col119
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_48_col120]: [QM31; 1] =
            (*pedersen_points_table_output_limb_48_col120
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_49_col121]: [QM31; 1] =
            (*pedersen_points_table_output_limb_49_col121
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_50_col122]: [QM31; 1] =
            (*pedersen_points_table_output_limb_50_col122
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_51_col123]: [QM31; 1] =
            (*pedersen_points_table_output_limb_51_col123
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_52_col124]: [QM31; 1] =
            (*pedersen_points_table_output_limb_52_col124
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_53_col125]: [QM31; 1] =
            (*pedersen_points_table_output_limb_53_col125
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_54_col126]: [QM31; 1] =
            (*pedersen_points_table_output_limb_54_col126
            .try_into()
            .unwrap())
            .unbox();
        let [pedersen_points_table_output_limb_55_col127]: [QM31; 1] =
            (*pedersen_points_table_output_limb_55_col127
            .try_into()
            .unwrap())
            .unbox();
        let [slope_limb_0_col128]: [QM31; 1] = (*slope_limb_0_col128.try_into().unwrap()).unbox();
        let [slope_limb_1_col129]: [QM31; 1] = (*slope_limb_1_col129.try_into().unwrap()).unbox();
        let [slope_limb_2_col130]: [QM31; 1] = (*slope_limb_2_col130.try_into().unwrap()).unbox();
        let [slope_limb_3_col131]: [QM31; 1] = (*slope_limb_3_col131.try_into().unwrap()).unbox();
        let [slope_limb_4_col132]: [QM31; 1] = (*slope_limb_4_col132.try_into().unwrap()).unbox();
        let [slope_limb_5_col133]: [QM31; 1] = (*slope_limb_5_col133.try_into().unwrap()).unbox();
        let [slope_limb_6_col134]: [QM31; 1] = (*slope_limb_6_col134.try_into().unwrap()).unbox();
        let [slope_limb_7_col135]: [QM31; 1] = (*slope_limb_7_col135.try_into().unwrap()).unbox();
        let [slope_limb_8_col136]: [QM31; 1] = (*slope_limb_8_col136.try_into().unwrap()).unbox();
        let [slope_limb_9_col137]: [QM31; 1] = (*slope_limb_9_col137.try_into().unwrap()).unbox();
        let [slope_limb_10_col138]: [QM31; 1] = (*slope_limb_10_col138.try_into().unwrap()).unbox();
        let [slope_limb_11_col139]: [QM31; 1] = (*slope_limb_11_col139.try_into().unwrap()).unbox();
        let [slope_limb_12_col140]: [QM31; 1] = (*slope_limb_12_col140.try_into().unwrap()).unbox();
        let [slope_limb_13_col141]: [QM31; 1] = (*slope_limb_13_col141.try_into().unwrap()).unbox();
        let [slope_limb_14_col142]: [QM31; 1] = (*slope_limb_14_col142.try_into().unwrap()).unbox();
        let [slope_limb_15_col143]: [QM31; 1] = (*slope_limb_15_col143.try_into().unwrap()).unbox();
        let [slope_limb_16_col144]: [QM31; 1] = (*slope_limb_16_col144.try_into().unwrap()).unbox();
        let [slope_limb_17_col145]: [QM31; 1] = (*slope_limb_17_col145.try_into().unwrap()).unbox();
        let [slope_limb_18_col146]: [QM31; 1] = (*slope_limb_18_col146.try_into().unwrap()).unbox();
        let [slope_limb_19_col147]: [QM31; 1] = (*slope_limb_19_col147.try_into().unwrap()).unbox();
        let [slope_limb_20_col148]: [QM31; 1] = (*slope_limb_20_col148.try_into().unwrap()).unbox();
        let [slope_limb_21_col149]: [QM31; 1] = (*slope_limb_21_col149.try_into().unwrap()).unbox();
        let [slope_limb_22_col150]: [QM31; 1] = (*slope_limb_22_col150.try_into().unwrap()).unbox();
        let [slope_limb_23_col151]: [QM31; 1] = (*slope_limb_23_col151.try_into().unwrap()).unbox();
        let [slope_limb_24_col152]: [QM31; 1] = (*slope_limb_24_col152.try_into().unwrap()).unbox();
        let [slope_limb_25_col153]: [QM31; 1] = (*slope_limb_25_col153.try_into().unwrap()).unbox();
        let [slope_limb_26_col154]: [QM31; 1] = (*slope_limb_26_col154.try_into().unwrap()).unbox();
        let [slope_limb_27_col155]: [QM31; 1] = (*slope_limb_27_col155.try_into().unwrap()).unbox();
        let [k_col156]: [QM31; 1] = (*k_col156.try_into().unwrap()).unbox();
        let [carry_0_col157]: [QM31; 1] = (*carry_0_col157.try_into().unwrap()).unbox();
        let [carry_1_col158]: [QM31; 1] = (*carry_1_col158.try_into().unwrap()).unbox();
        let [carry_2_col159]: [QM31; 1] = (*carry_2_col159.try_into().unwrap()).unbox();
        let [carry_3_col160]: [QM31; 1] = (*carry_3_col160.try_into().unwrap()).unbox();
        let [carry_4_col161]: [QM31; 1] = (*carry_4_col161.try_into().unwrap()).unbox();
        let [carry_5_col162]: [QM31; 1] = (*carry_5_col162.try_into().unwrap()).unbox();
        let [carry_6_col163]: [QM31; 1] = (*carry_6_col163.try_into().unwrap()).unbox();
        let [carry_7_col164]: [QM31; 1] = (*carry_7_col164.try_into().unwrap()).unbox();
        let [carry_8_col165]: [QM31; 1] = (*carry_8_col165.try_into().unwrap()).unbox();
        let [carry_9_col166]: [QM31; 1] = (*carry_9_col166.try_into().unwrap()).unbox();
        let [carry_10_col167]: [QM31; 1] = (*carry_10_col167.try_into().unwrap()).unbox();
        let [carry_11_col168]: [QM31; 1] = (*carry_11_col168.try_into().unwrap()).unbox();
        let [carry_12_col169]: [QM31; 1] = (*carry_12_col169.try_into().unwrap()).unbox();
        let [carry_13_col170]: [QM31; 1] = (*carry_13_col170.try_into().unwrap()).unbox();
        let [carry_14_col171]: [QM31; 1] = (*carry_14_col171.try_into().unwrap()).unbox();
        let [carry_15_col172]: [QM31; 1] = (*carry_15_col172.try_into().unwrap()).unbox();
        let [carry_16_col173]: [QM31; 1] = (*carry_16_col173.try_into().unwrap()).unbox();
        let [carry_17_col174]: [QM31; 1] = (*carry_17_col174.try_into().unwrap()).unbox();
        let [carry_18_col175]: [QM31; 1] = (*carry_18_col175.try_into().unwrap()).unbox();
        let [carry_19_col176]: [QM31; 1] = (*carry_19_col176.try_into().unwrap()).unbox();
        let [carry_20_col177]: [QM31; 1] = (*carry_20_col177.try_into().unwrap()).unbox();
        let [carry_21_col178]: [QM31; 1] = (*carry_21_col178.try_into().unwrap()).unbox();
        let [carry_22_col179]: [QM31; 1] = (*carry_22_col179.try_into().unwrap()).unbox();
        let [carry_23_col180]: [QM31; 1] = (*carry_23_col180.try_into().unwrap()).unbox();
        let [carry_24_col181]: [QM31; 1] = (*carry_24_col181.try_into().unwrap()).unbox();
        let [carry_25_col182]: [QM31; 1] = (*carry_25_col182.try_into().unwrap()).unbox();
        let [carry_26_col183]: [QM31; 1] = (*carry_26_col183.try_into().unwrap()).unbox();
        let [result_x_limb_0_col184]: [QM31; 1] = (*result_x_limb_0_col184.try_into().unwrap())
            .unbox();
        let [result_x_limb_1_col185]: [QM31; 1] = (*result_x_limb_1_col185.try_into().unwrap())
            .unbox();
        let [result_x_limb_2_col186]: [QM31; 1] = (*result_x_limb_2_col186.try_into().unwrap())
            .unbox();
        let [result_x_limb_3_col187]: [QM31; 1] = (*result_x_limb_3_col187.try_into().unwrap())
            .unbox();
        let [result_x_limb_4_col188]: [QM31; 1] = (*result_x_limb_4_col188.try_into().unwrap())
            .unbox();
        let [result_x_limb_5_col189]: [QM31; 1] = (*result_x_limb_5_col189.try_into().unwrap())
            .unbox();
        let [result_x_limb_6_col190]: [QM31; 1] = (*result_x_limb_6_col190.try_into().unwrap())
            .unbox();
        let [result_x_limb_7_col191]: [QM31; 1] = (*result_x_limb_7_col191.try_into().unwrap())
            .unbox();
        let [result_x_limb_8_col192]: [QM31; 1] = (*result_x_limb_8_col192.try_into().unwrap())
            .unbox();
        let [result_x_limb_9_col193]: [QM31; 1] = (*result_x_limb_9_col193.try_into().unwrap())
            .unbox();
        let [result_x_limb_10_col194]: [QM31; 1] = (*result_x_limb_10_col194.try_into().unwrap())
            .unbox();
        let [result_x_limb_11_col195]: [QM31; 1] = (*result_x_limb_11_col195.try_into().unwrap())
            .unbox();
        let [result_x_limb_12_col196]: [QM31; 1] = (*result_x_limb_12_col196.try_into().unwrap())
            .unbox();
        let [result_x_limb_13_col197]: [QM31; 1] = (*result_x_limb_13_col197.try_into().unwrap())
            .unbox();
        let [result_x_limb_14_col198]: [QM31; 1] = (*result_x_limb_14_col198.try_into().unwrap())
            .unbox();
        let [result_x_limb_15_col199]: [QM31; 1] = (*result_x_limb_15_col199.try_into().unwrap())
            .unbox();
        let [result_x_limb_16_col200]: [QM31; 1] = (*result_x_limb_16_col200.try_into().unwrap())
            .unbox();
        let [result_x_limb_17_col201]: [QM31; 1] = (*result_x_limb_17_col201.try_into().unwrap())
            .unbox();
        let [result_x_limb_18_col202]: [QM31; 1] = (*result_x_limb_18_col202.try_into().unwrap())
            .unbox();
        let [result_x_limb_19_col203]: [QM31; 1] = (*result_x_limb_19_col203.try_into().unwrap())
            .unbox();
        let [result_x_limb_20_col204]: [QM31; 1] = (*result_x_limb_20_col204.try_into().unwrap())
            .unbox();
        let [result_x_limb_21_col205]: [QM31; 1] = (*result_x_limb_21_col205.try_into().unwrap())
            .unbox();
        let [result_x_limb_22_col206]: [QM31; 1] = (*result_x_limb_22_col206.try_into().unwrap())
            .unbox();
        let [result_x_limb_23_col207]: [QM31; 1] = (*result_x_limb_23_col207.try_into().unwrap())
            .unbox();
        let [result_x_limb_24_col208]: [QM31; 1] = (*result_x_limb_24_col208.try_into().unwrap())
            .unbox();
        let [result_x_limb_25_col209]: [QM31; 1] = (*result_x_limb_25_col209.try_into().unwrap())
            .unbox();
        let [result_x_limb_26_col210]: [QM31; 1] = (*result_x_limb_26_col210.try_into().unwrap())
            .unbox();
        let [result_x_limb_27_col211]: [QM31; 1] = (*result_x_limb_27_col211.try_into().unwrap())
            .unbox();
        let [k_col212]: [QM31; 1] = (*k_col212.try_into().unwrap()).unbox();
        let [carry_0_col213]: [QM31; 1] = (*carry_0_col213.try_into().unwrap()).unbox();
        let [carry_1_col214]: [QM31; 1] = (*carry_1_col214.try_into().unwrap()).unbox();
        let [carry_2_col215]: [QM31; 1] = (*carry_2_col215.try_into().unwrap()).unbox();
        let [carry_3_col216]: [QM31; 1] = (*carry_3_col216.try_into().unwrap()).unbox();
        let [carry_4_col217]: [QM31; 1] = (*carry_4_col217.try_into().unwrap()).unbox();
        let [carry_5_col218]: [QM31; 1] = (*carry_5_col218.try_into().unwrap()).unbox();
        let [carry_6_col219]: [QM31; 1] = (*carry_6_col219.try_into().unwrap()).unbox();
        let [carry_7_col220]: [QM31; 1] = (*carry_7_col220.try_into().unwrap()).unbox();
        let [carry_8_col221]: [QM31; 1] = (*carry_8_col221.try_into().unwrap()).unbox();
        let [carry_9_col222]: [QM31; 1] = (*carry_9_col222.try_into().unwrap()).unbox();
        let [carry_10_col223]: [QM31; 1] = (*carry_10_col223.try_into().unwrap()).unbox();
        let [carry_11_col224]: [QM31; 1] = (*carry_11_col224.try_into().unwrap()).unbox();
        let [carry_12_col225]: [QM31; 1] = (*carry_12_col225.try_into().unwrap()).unbox();
        let [carry_13_col226]: [QM31; 1] = (*carry_13_col226.try_into().unwrap()).unbox();
        let [carry_14_col227]: [QM31; 1] = (*carry_14_col227.try_into().unwrap()).unbox();
        let [carry_15_col228]: [QM31; 1] = (*carry_15_col228.try_into().unwrap()).unbox();
        let [carry_16_col229]: [QM31; 1] = (*carry_16_col229.try_into().unwrap()).unbox();
        let [carry_17_col230]: [QM31; 1] = (*carry_17_col230.try_into().unwrap()).unbox();
        let [carry_18_col231]: [QM31; 1] = (*carry_18_col231.try_into().unwrap()).unbox();
        let [carry_19_col232]: [QM31; 1] = (*carry_19_col232.try_into().unwrap()).unbox();
        let [carry_20_col233]: [QM31; 1] = (*carry_20_col233.try_into().unwrap()).unbox();
        let [carry_21_col234]: [QM31; 1] = (*carry_21_col234.try_into().unwrap()).unbox();
        let [carry_22_col235]: [QM31; 1] = (*carry_22_col235.try_into().unwrap()).unbox();
        let [carry_23_col236]: [QM31; 1] = (*carry_23_col236.try_into().unwrap()).unbox();
        let [carry_24_col237]: [QM31; 1] = (*carry_24_col237.try_into().unwrap()).unbox();
        let [carry_25_col238]: [QM31; 1] = (*carry_25_col238.try_into().unwrap()).unbox();
        let [carry_26_col239]: [QM31; 1] = (*carry_26_col239.try_into().unwrap()).unbox();
        let [result_y_limb_0_col240]: [QM31; 1] = (*result_y_limb_0_col240.try_into().unwrap())
            .unbox();
        let [result_y_limb_1_col241]: [QM31; 1] = (*result_y_limb_1_col241.try_into().unwrap())
            .unbox();
        let [result_y_limb_2_col242]: [QM31; 1] = (*result_y_limb_2_col242.try_into().unwrap())
            .unbox();
        let [result_y_limb_3_col243]: [QM31; 1] = (*result_y_limb_3_col243.try_into().unwrap())
            .unbox();
        let [result_y_limb_4_col244]: [QM31; 1] = (*result_y_limb_4_col244.try_into().unwrap())
            .unbox();
        let [result_y_limb_5_col245]: [QM31; 1] = (*result_y_limb_5_col245.try_into().unwrap())
            .unbox();
        let [result_y_limb_6_col246]: [QM31; 1] = (*result_y_limb_6_col246.try_into().unwrap())
            .unbox();
        let [result_y_limb_7_col247]: [QM31; 1] = (*result_y_limb_7_col247.try_into().unwrap())
            .unbox();
        let [result_y_limb_8_col248]: [QM31; 1] = (*result_y_limb_8_col248.try_into().unwrap())
            .unbox();
        let [result_y_limb_9_col249]: [QM31; 1] = (*result_y_limb_9_col249.try_into().unwrap())
            .unbox();
        let [result_y_limb_10_col250]: [QM31; 1] = (*result_y_limb_10_col250.try_into().unwrap())
            .unbox();
        let [result_y_limb_11_col251]: [QM31; 1] = (*result_y_limb_11_col251.try_into().unwrap())
            .unbox();
        let [result_y_limb_12_col252]: [QM31; 1] = (*result_y_limb_12_col252.try_into().unwrap())
            .unbox();
        let [result_y_limb_13_col253]: [QM31; 1] = (*result_y_limb_13_col253.try_into().unwrap())
            .unbox();
        let [result_y_limb_14_col254]: [QM31; 1] = (*result_y_limb_14_col254.try_into().unwrap())
            .unbox();
        let [result_y_limb_15_col255]: [QM31; 1] = (*result_y_limb_15_col255.try_into().unwrap())
            .unbox();
        let [result_y_limb_16_col256]: [QM31; 1] = (*result_y_limb_16_col256.try_into().unwrap())
            .unbox();
        let [result_y_limb_17_col257]: [QM31; 1] = (*result_y_limb_17_col257.try_into().unwrap())
            .unbox();
        let [result_y_limb_18_col258]: [QM31; 1] = (*result_y_limb_18_col258.try_into().unwrap())
            .unbox();
        let [result_y_limb_19_col259]: [QM31; 1] = (*result_y_limb_19_col259.try_into().unwrap())
            .unbox();
        let [result_y_limb_20_col260]: [QM31; 1] = (*result_y_limb_20_col260.try_into().unwrap())
            .unbox();
        let [result_y_limb_21_col261]: [QM31; 1] = (*result_y_limb_21_col261.try_into().unwrap())
            .unbox();
        let [result_y_limb_22_col262]: [QM31; 1] = (*result_y_limb_22_col262.try_into().unwrap())
            .unbox();
        let [result_y_limb_23_col263]: [QM31; 1] = (*result_y_limb_23_col263.try_into().unwrap())
            .unbox();
        let [result_y_limb_24_col264]: [QM31; 1] = (*result_y_limb_24_col264.try_into().unwrap())
            .unbox();
        let [result_y_limb_25_col265]: [QM31; 1] = (*result_y_limb_25_col265.try_into().unwrap())
            .unbox();
        let [result_y_limb_26_col266]: [QM31; 1] = (*result_y_limb_26_col266.try_into().unwrap())
            .unbox();
        let [result_y_limb_27_col267]: [QM31; 1] = (*result_y_limb_27_col267.try_into().unwrap())
            .unbox();
        let [k_col268]: [QM31; 1] = (*k_col268.try_into().unwrap()).unbox();
        let [carry_0_col269]: [QM31; 1] = (*carry_0_col269.try_into().unwrap()).unbox();
        let [carry_1_col270]: [QM31; 1] = (*carry_1_col270.try_into().unwrap()).unbox();
        let [carry_2_col271]: [QM31; 1] = (*carry_2_col271.try_into().unwrap()).unbox();
        let [carry_3_col272]: [QM31; 1] = (*carry_3_col272.try_into().unwrap()).unbox();
        let [carry_4_col273]: [QM31; 1] = (*carry_4_col273.try_into().unwrap()).unbox();
        let [carry_5_col274]: [QM31; 1] = (*carry_5_col274.try_into().unwrap()).unbox();
        let [carry_6_col275]: [QM31; 1] = (*carry_6_col275.try_into().unwrap()).unbox();
        let [carry_7_col276]: [QM31; 1] = (*carry_7_col276.try_into().unwrap()).unbox();
        let [carry_8_col277]: [QM31; 1] = (*carry_8_col277.try_into().unwrap()).unbox();
        let [carry_9_col278]: [QM31; 1] = (*carry_9_col278.try_into().unwrap()).unbox();
        let [carry_10_col279]: [QM31; 1] = (*carry_10_col279.try_into().unwrap()).unbox();
        let [carry_11_col280]: [QM31; 1] = (*carry_11_col280.try_into().unwrap()).unbox();
        let [carry_12_col281]: [QM31; 1] = (*carry_12_col281.try_into().unwrap()).unbox();
        let [carry_13_col282]: [QM31; 1] = (*carry_13_col282.try_into().unwrap()).unbox();
        let [carry_14_col283]: [QM31; 1] = (*carry_14_col283.try_into().unwrap()).unbox();
        let [carry_15_col284]: [QM31; 1] = (*carry_15_col284.try_into().unwrap()).unbox();
        let [carry_16_col285]: [QM31; 1] = (*carry_16_col285.try_into().unwrap()).unbox();
        let [carry_17_col286]: [QM31; 1] = (*carry_17_col286.try_into().unwrap()).unbox();
        let [carry_18_col287]: [QM31; 1] = (*carry_18_col287.try_into().unwrap()).unbox();
        let [carry_19_col288]: [QM31; 1] = (*carry_19_col288.try_into().unwrap()).unbox();
        let [carry_20_col289]: [QM31; 1] = (*carry_20_col289.try_into().unwrap()).unbox();
        let [carry_21_col290]: [QM31; 1] = (*carry_21_col290.try_into().unwrap()).unbox();
        let [carry_22_col291]: [QM31; 1] = (*carry_22_col291.try_into().unwrap()).unbox();
        let [carry_23_col292]: [QM31; 1] = (*carry_23_col292.try_into().unwrap()).unbox();
        let [carry_24_col293]: [QM31; 1] = (*carry_24_col293.try_into().unwrap()).unbox();
        let [carry_25_col294]: [QM31; 1] = (*carry_25_col294.try_into().unwrap()).unbox();
        let [carry_26_col295]: [QM31; 1] = (*carry_26_col295.try_into().unwrap()).unbox();
        let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        let constraint_quotient = (enabler * enabler - enabler) * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        pedersen_points_table_sum_0 = self
            .pedersen_points_table_lookup_elements
            .combine_qm31(
                [
                    ((qm31_const::<262144, 0, 0, 0>() * input_limb_1_col1) + input_limb_2_col2),
                    pedersen_points_table_output_limb_0_col72,
                    pedersen_points_table_output_limb_1_col73,
                    pedersen_points_table_output_limb_2_col74,
                    pedersen_points_table_output_limb_3_col75,
                    pedersen_points_table_output_limb_4_col76,
                    pedersen_points_table_output_limb_5_col77,
                    pedersen_points_table_output_limb_6_col78,
                    pedersen_points_table_output_limb_7_col79,
                    pedersen_points_table_output_limb_8_col80,
                    pedersen_points_table_output_limb_9_col81,
                    pedersen_points_table_output_limb_10_col82,
                    pedersen_points_table_output_limb_11_col83,
                    pedersen_points_table_output_limb_12_col84,
                    pedersen_points_table_output_limb_13_col85,
                    pedersen_points_table_output_limb_14_col86,
                    pedersen_points_table_output_limb_15_col87,
                    pedersen_points_table_output_limb_16_col88,
                    pedersen_points_table_output_limb_17_col89,
                    pedersen_points_table_output_limb_18_col90,
                    pedersen_points_table_output_limb_19_col91,
                    pedersen_points_table_output_limb_20_col92,
                    pedersen_points_table_output_limb_21_col93,
                    pedersen_points_table_output_limb_22_col94,
                    pedersen_points_table_output_limb_23_col95,
                    pedersen_points_table_output_limb_24_col96,
                    pedersen_points_table_output_limb_25_col97,
                    pedersen_points_table_output_limb_26_col98,
                    pedersen_points_table_output_limb_27_col99,
                    pedersen_points_table_output_limb_28_col100,
                    pedersen_points_table_output_limb_29_col101,
                    pedersen_points_table_output_limb_30_col102,
                    pedersen_points_table_output_limb_31_col103,
                    pedersen_points_table_output_limb_32_col104,
                    pedersen_points_table_output_limb_33_col105,
                    pedersen_points_table_output_limb_34_col106,
                    pedersen_points_table_output_limb_35_col107,
                    pedersen_points_table_output_limb_36_col108,
                    pedersen_points_table_output_limb_37_col109,
                    pedersen_points_table_output_limb_38_col110,
                    pedersen_points_table_output_limb_39_col111,
                    pedersen_points_table_output_limb_40_col112,
                    pedersen_points_table_output_limb_41_col113,
                    pedersen_points_table_output_limb_42_col114,
                    pedersen_points_table_output_limb_43_col115,
                    pedersen_points_table_output_limb_44_col116,
                    pedersen_points_table_output_limb_45_col117,
                    pedersen_points_table_output_limb_46_col118,
                    pedersen_points_table_output_limb_47_col119,
                    pedersen_points_table_output_limb_48_col120,
                    pedersen_points_table_output_limb_49_col121,
                    pedersen_points_table_output_limb_50_col122,
                    pedersen_points_table_output_limb_51_col123,
                    pedersen_points_table_output_limb_52_col124,
                    pedersen_points_table_output_limb_53_col125,
                    pedersen_points_table_output_limb_54_col126,
                    pedersen_points_table_output_limb_55_col127,
                ],
            );
        ec_add_evaluate(
            [
                input_limb_16_col16, input_limb_17_col17, input_limb_18_col18, input_limb_19_col19,
                input_limb_20_col20, input_limb_21_col21, input_limb_22_col22, input_limb_23_col23,
                input_limb_24_col24, input_limb_25_col25, input_limb_26_col26, input_limb_27_col27,
                input_limb_28_col28, input_limb_29_col29, input_limb_30_col30, input_limb_31_col31,
                input_limb_32_col32, input_limb_33_col33, input_limb_34_col34, input_limb_35_col35,
                input_limb_36_col36, input_limb_37_col37, input_limb_38_col38, input_limb_39_col39,
                input_limb_40_col40, input_limb_41_col41, input_limb_42_col42, input_limb_43_col43,
                input_limb_44_col44, input_limb_45_col45, input_limb_46_col46, input_limb_47_col47,
                input_limb_48_col48, input_limb_49_col49, input_limb_50_col50, input_limb_51_col51,
                input_limb_52_col52, input_limb_53_col53, input_limb_54_col54, input_limb_55_col55,
                input_limb_56_col56, input_limb_57_col57, input_limb_58_col58, input_limb_59_col59,
                input_limb_60_col60, input_limb_61_col61, input_limb_62_col62, input_limb_63_col63,
                input_limb_64_col64, input_limb_65_col65, input_limb_66_col66, input_limb_67_col67,
                input_limb_68_col68, input_limb_69_col69, input_limb_70_col70, input_limb_71_col71,
                pedersen_points_table_output_limb_0_col72,
                pedersen_points_table_output_limb_1_col73,
                pedersen_points_table_output_limb_2_col74,
                pedersen_points_table_output_limb_3_col75,
                pedersen_points_table_output_limb_4_col76,
                pedersen_points_table_output_limb_5_col77,
                pedersen_points_table_output_limb_6_col78,
                pedersen_points_table_output_limb_7_col79,
                pedersen_points_table_output_limb_8_col80,
                pedersen_points_table_output_limb_9_col81,
                pedersen_points_table_output_limb_10_col82,
                pedersen_points_table_output_limb_11_col83,
                pedersen_points_table_output_limb_12_col84,
                pedersen_points_table_output_limb_13_col85,
                pedersen_points_table_output_limb_14_col86,
                pedersen_points_table_output_limb_15_col87,
                pedersen_points_table_output_limb_16_col88,
                pedersen_points_table_output_limb_17_col89,
                pedersen_points_table_output_limb_18_col90,
                pedersen_points_table_output_limb_19_col91,
                pedersen_points_table_output_limb_20_col92,
                pedersen_points_table_output_limb_21_col93,
                pedersen_points_table_output_limb_22_col94,
                pedersen_points_table_output_limb_23_col95,
                pedersen_points_table_output_limb_24_col96,
                pedersen_points_table_output_limb_25_col97,
                pedersen_points_table_output_limb_26_col98,
                pedersen_points_table_output_limb_27_col99,
                pedersen_points_table_output_limb_28_col100,
                pedersen_points_table_output_limb_29_col101,
                pedersen_points_table_output_limb_30_col102,
                pedersen_points_table_output_limb_31_col103,
                pedersen_points_table_output_limb_32_col104,
                pedersen_points_table_output_limb_33_col105,
                pedersen_points_table_output_limb_34_col106,
                pedersen_points_table_output_limb_35_col107,
                pedersen_points_table_output_limb_36_col108,
                pedersen_points_table_output_limb_37_col109,
                pedersen_points_table_output_limb_38_col110,
                pedersen_points_table_output_limb_39_col111,
                pedersen_points_table_output_limb_40_col112,
                pedersen_points_table_output_limb_41_col113,
                pedersen_points_table_output_limb_42_col114,
                pedersen_points_table_output_limb_43_col115,
                pedersen_points_table_output_limb_44_col116,
                pedersen_points_table_output_limb_45_col117,
                pedersen_points_table_output_limb_46_col118,
                pedersen_points_table_output_limb_47_col119,
                pedersen_points_table_output_limb_48_col120,
                pedersen_points_table_output_limb_49_col121,
                pedersen_points_table_output_limb_50_col122,
                pedersen_points_table_output_limb_51_col123,
                pedersen_points_table_output_limb_52_col124,
                pedersen_points_table_output_limb_53_col125,
                pedersen_points_table_output_limb_54_col126,
                pedersen_points_table_output_limb_55_col127,
            ],
            slope_limb_0_col128,
            slope_limb_1_col129,
            slope_limb_2_col130,
            slope_limb_3_col131,
            slope_limb_4_col132,
            slope_limb_5_col133,
            slope_limb_6_col134,
            slope_limb_7_col135,
            slope_limb_8_col136,
            slope_limb_9_col137,
            slope_limb_10_col138,
            slope_limb_11_col139,
            slope_limb_12_col140,
            slope_limb_13_col141,
            slope_limb_14_col142,
            slope_limb_15_col143,
            slope_limb_16_col144,
            slope_limb_17_col145,
            slope_limb_18_col146,
            slope_limb_19_col147,
            slope_limb_20_col148,
            slope_limb_21_col149,
            slope_limb_22_col150,
            slope_limb_23_col151,
            slope_limb_24_col152,
            slope_limb_25_col153,
            slope_limb_26_col154,
            slope_limb_27_col155,
            k_col156,
            carry_0_col157,
            carry_1_col158,
            carry_2_col159,
            carry_3_col160,
            carry_4_col161,
            carry_5_col162,
            carry_6_col163,
            carry_7_col164,
            carry_8_col165,
            carry_9_col166,
            carry_10_col167,
            carry_11_col168,
            carry_12_col169,
            carry_13_col170,
            carry_14_col171,
            carry_15_col172,
            carry_16_col173,
            carry_17_col174,
            carry_18_col175,
            carry_19_col176,
            carry_20_col177,
            carry_21_col178,
            carry_22_col179,
            carry_23_col180,
            carry_24_col181,
            carry_25_col182,
            carry_26_col183,
            result_x_limb_0_col184,
            result_x_limb_1_col185,
            result_x_limb_2_col186,
            result_x_limb_3_col187,
            result_x_limb_4_col188,
            result_x_limb_5_col189,
            result_x_limb_6_col190,
            result_x_limb_7_col191,
            result_x_limb_8_col192,
            result_x_limb_9_col193,
            result_x_limb_10_col194,
            result_x_limb_11_col195,
            result_x_limb_12_col196,
            result_x_limb_13_col197,
            result_x_limb_14_col198,
            result_x_limb_15_col199,
            result_x_limb_16_col200,
            result_x_limb_17_col201,
            result_x_limb_18_col202,
            result_x_limb_19_col203,
            result_x_limb_20_col204,
            result_x_limb_21_col205,
            result_x_limb_22_col206,
            result_x_limb_23_col207,
            result_x_limb_24_col208,
            result_x_limb_25_col209,
            result_x_limb_26_col210,
            result_x_limb_27_col211,
            k_col212,
            carry_0_col213,
            carry_1_col214,
            carry_2_col215,
            carry_3_col216,
            carry_4_col217,
            carry_5_col218,
            carry_6_col219,
            carry_7_col220,
            carry_8_col221,
            carry_9_col222,
            carry_10_col223,
            carry_11_col224,
            carry_12_col225,
            carry_13_col226,
            carry_14_col227,
            carry_15_col228,
            carry_16_col229,
            carry_17_col230,
            carry_18_col231,
            carry_19_col232,
            carry_20_col233,
            carry_21_col234,
            carry_22_col235,
            carry_23_col236,
            carry_24_col237,
            carry_25_col238,
            carry_26_col239,
            result_y_limb_0_col240,
            result_y_limb_1_col241,
            result_y_limb_2_col242,
            result_y_limb_3_col243,
            result_y_limb_4_col244,
            result_y_limb_5_col245,
            result_y_limb_6_col246,
            result_y_limb_7_col247,
            result_y_limb_8_col248,
            result_y_limb_9_col249,
            result_y_limb_10_col250,
            result_y_limb_11_col251,
            result_y_limb_12_col252,
            result_y_limb_13_col253,
            result_y_limb_14_col254,
            result_y_limb_15_col255,
            result_y_limb_16_col256,
            result_y_limb_17_col257,
            result_y_limb_18_col258,
            result_y_limb_19_col259,
            result_y_limb_20_col260,
            result_y_limb_21_col261,
            result_y_limb_22_col262,
            result_y_limb_23_col263,
            result_y_limb_24_col264,
            result_y_limb_25_col265,
            result_y_limb_26_col266,
            result_y_limb_27_col267,
            k_col268,
            carry_0_col269,
            carry_1_col270,
            carry_2_col271,
            carry_3_col272,
            carry_4_col273,
            carry_5_col274,
            carry_6_col275,
            carry_7_col276,
            carry_8_col277,
            carry_9_col278,
            carry_10_col279,
            carry_11_col280,
            carry_12_col281,
            carry_13_col282,
            carry_14_col283,
            carry_15_col284,
            carry_16_col285,
            carry_17_col286,
            carry_18_col287,
            carry_19_col288,
            carry_20_col289,
            carry_21_col290,
            carry_22_col291,
            carry_23_col292,
            carry_24_col293,
            carry_25_col294,
            carry_26_col295,
            self.range_check_9_9_lookup_elements,
            self.range_check_9_9_b_lookup_elements,
            self.range_check_9_9_c_lookup_elements,
            self.range_check_9_9_d_lookup_elements,
            self.range_check_9_9_e_lookup_elements,
            self.range_check_9_9_f_lookup_elements,
            self.range_check_9_9_g_lookup_elements,
            self.range_check_9_9_h_lookup_elements,
            self.range_check_20_lookup_elements,
            self.range_check_20_b_lookup_elements,
            self.range_check_20_c_lookup_elements,
            self.range_check_20_d_lookup_elements,
            self.range_check_20_e_lookup_elements,
            self.range_check_20_f_lookup_elements,
            self.range_check_20_g_lookup_elements,
            self.range_check_20_h_lookup_elements,
            ref range_check_9_9_sum_1,
            ref range_check_9_9_b_sum_2,
            ref range_check_9_9_c_sum_3,
            ref range_check_9_9_d_sum_4,
            ref range_check_9_9_e_sum_5,
            ref range_check_9_9_f_sum_6,
            ref range_check_9_9_g_sum_7,
            ref range_check_9_9_h_sum_8,
            ref range_check_9_9_sum_9,
            ref range_check_9_9_b_sum_10,
            ref range_check_9_9_c_sum_11,
            ref range_check_9_9_d_sum_12,
            ref range_check_9_9_e_sum_13,
            ref range_check_9_9_f_sum_14,
            ref range_check_20_sum_15,
            ref range_check_20_b_sum_16,
            ref range_check_20_c_sum_17,
            ref range_check_20_d_sum_18,
            ref range_check_20_e_sum_19,
            ref range_check_20_f_sum_20,
            ref range_check_20_g_sum_21,
            ref range_check_20_h_sum_22,
            ref range_check_20_sum_23,
            ref range_check_20_b_sum_24,
            ref range_check_20_c_sum_25,
            ref range_check_20_d_sum_26,
            ref range_check_20_e_sum_27,
            ref range_check_20_f_sum_28,
            ref range_check_20_g_sum_29,
            ref range_check_20_h_sum_30,
            ref range_check_20_sum_31,
            ref range_check_20_b_sum_32,
            ref range_check_20_c_sum_33,
            ref range_check_20_d_sum_34,
            ref range_check_20_e_sum_35,
            ref range_check_20_f_sum_36,
            ref range_check_20_g_sum_37,
            ref range_check_20_h_sum_38,
            ref range_check_20_sum_39,
            ref range_check_20_b_sum_40,
            ref range_check_20_c_sum_41,
            ref range_check_20_d_sum_42,
            ref range_check_9_9_sum_43,
            ref range_check_9_9_b_sum_44,
            ref range_check_9_9_c_sum_45,
            ref range_check_9_9_d_sum_46,
            ref range_check_9_9_e_sum_47,
            ref range_check_9_9_f_sum_48,
            ref range_check_9_9_g_sum_49,
            ref range_check_9_9_h_sum_50,
            ref range_check_9_9_sum_51,
            ref range_check_9_9_b_sum_52,
            ref range_check_9_9_c_sum_53,
            ref range_check_9_9_d_sum_54,
            ref range_check_9_9_e_sum_55,
            ref range_check_9_9_f_sum_56,
            ref range_check_20_sum_57,
            ref range_check_20_b_sum_58,
            ref range_check_20_c_sum_59,
            ref range_check_20_d_sum_60,
            ref range_check_20_e_sum_61,
            ref range_check_20_f_sum_62,
            ref range_check_20_g_sum_63,
            ref range_check_20_h_sum_64,
            ref range_check_20_sum_65,
            ref range_check_20_b_sum_66,
            ref range_check_20_c_sum_67,
            ref range_check_20_d_sum_68,
            ref range_check_20_e_sum_69,
            ref range_check_20_f_sum_70,
            ref range_check_20_g_sum_71,
            ref range_check_20_h_sum_72,
            ref range_check_20_sum_73,
            ref range_check_20_b_sum_74,
            ref range_check_20_c_sum_75,
            ref range_check_20_d_sum_76,
            ref range_check_20_e_sum_77,
            ref range_check_20_f_sum_78,
            ref range_check_20_g_sum_79,
            ref range_check_20_h_sum_80,
            ref range_check_20_sum_81,
            ref range_check_20_b_sum_82,
            ref range_check_20_c_sum_83,
            ref range_check_20_d_sum_84,
            ref range_check_9_9_sum_85,
            ref range_check_9_9_b_sum_86,
            ref range_check_9_9_c_sum_87,
            ref range_check_9_9_d_sum_88,
            ref range_check_9_9_e_sum_89,
            ref range_check_9_9_f_sum_90,
            ref range_check_9_9_g_sum_91,
            ref range_check_9_9_h_sum_92,
            ref range_check_9_9_sum_93,
            ref range_check_9_9_b_sum_94,
            ref range_check_9_9_c_sum_95,
            ref range_check_9_9_d_sum_96,
            ref range_check_9_9_e_sum_97,
            ref range_check_9_9_f_sum_98,
            ref range_check_20_sum_99,
            ref range_check_20_b_sum_100,
            ref range_check_20_c_sum_101,
            ref range_check_20_d_sum_102,
            ref range_check_20_e_sum_103,
            ref range_check_20_f_sum_104,
            ref range_check_20_g_sum_105,
            ref range_check_20_h_sum_106,
            ref range_check_20_sum_107,
            ref range_check_20_b_sum_108,
            ref range_check_20_c_sum_109,
            ref range_check_20_d_sum_110,
            ref range_check_20_e_sum_111,
            ref range_check_20_f_sum_112,
            ref range_check_20_g_sum_113,
            ref range_check_20_h_sum_114,
            ref range_check_20_sum_115,
            ref range_check_20_b_sum_116,
            ref range_check_20_c_sum_117,
            ref range_check_20_d_sum_118,
            ref range_check_20_e_sum_119,
            ref range_check_20_f_sum_120,
            ref range_check_20_g_sum_121,
            ref range_check_20_h_sum_122,
            ref range_check_20_sum_123,
            ref range_check_20_b_sum_124,
            ref range_check_20_c_sum_125,
            ref range_check_20_d_sum_126,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        partial_ec_mul_sum_127 = self
            .partial_ec_mul_lookup_elements
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
                    input_limb_33_col33, input_limb_34_col34, input_limb_35_col35,
                    input_limb_36_col36, input_limb_37_col37, input_limb_38_col38,
                    input_limb_39_col39, input_limb_40_col40, input_limb_41_col41,
                    input_limb_42_col42, input_limb_43_col43, input_limb_44_col44,
                    input_limb_45_col45, input_limb_46_col46, input_limb_47_col47,
                    input_limb_48_col48, input_limb_49_col49, input_limb_50_col50,
                    input_limb_51_col51, input_limb_52_col52, input_limb_53_col53,
                    input_limb_54_col54, input_limb_55_col55, input_limb_56_col56,
                    input_limb_57_col57, input_limb_58_col58, input_limb_59_col59,
                    input_limb_60_col60, input_limb_61_col61, input_limb_62_col62,
                    input_limb_63_col63, input_limb_64_col64, input_limb_65_col65,
                    input_limb_66_col66, input_limb_67_col67, input_limb_68_col68,
                    input_limb_69_col69, input_limb_70_col70, input_limb_71_col71,
                ],
            );

        partial_ec_mul_sum_128 = self
            .partial_ec_mul_lookup_elements
            .combine_qm31(
                [
                    input_limb_0_col0, (input_limb_1_col1 + qm31_const::<1, 0, 0, 0>()),
                    input_limb_3_col3, input_limb_4_col4, input_limb_5_col5, input_limb_6_col6,
                    input_limb_7_col7, input_limb_8_col8, input_limb_9_col9, input_limb_10_col10,
                    input_limb_11_col11, input_limb_12_col12, input_limb_13_col13,
                    input_limb_14_col14, input_limb_15_col15, qm31_const::<0, 0, 0, 0>(),
                    result_x_limb_0_col184, result_x_limb_1_col185, result_x_limb_2_col186,
                    result_x_limb_3_col187, result_x_limb_4_col188, result_x_limb_5_col189,
                    result_x_limb_6_col190, result_x_limb_7_col191, result_x_limb_8_col192,
                    result_x_limb_9_col193, result_x_limb_10_col194, result_x_limb_11_col195,
                    result_x_limb_12_col196, result_x_limb_13_col197, result_x_limb_14_col198,
                    result_x_limb_15_col199, result_x_limb_16_col200, result_x_limb_17_col201,
                    result_x_limb_18_col202, result_x_limb_19_col203, result_x_limb_20_col204,
                    result_x_limb_21_col205, result_x_limb_22_col206, result_x_limb_23_col207,
                    result_x_limb_24_col208, result_x_limb_25_col209, result_x_limb_26_col210,
                    result_x_limb_27_col211, result_y_limb_0_col240, result_y_limb_1_col241,
                    result_y_limb_2_col242, result_y_limb_3_col243, result_y_limb_4_col244,
                    result_y_limb_5_col245, result_y_limb_6_col246, result_y_limb_7_col247,
                    result_y_limb_8_col248, result_y_limb_9_col249, result_y_limb_10_col250,
                    result_y_limb_11_col251, result_y_limb_12_col252, result_y_limb_13_col253,
                    result_y_limb_14_col254, result_y_limb_15_col255, result_y_limb_16_col256,
                    result_y_limb_17_col257, result_y_limb_18_col258, result_y_limb_19_col259,
                    result_y_limb_20_col260, result_y_limb_21_col261, result_y_limb_22_col262,
                    result_y_limb_23_col263, result_y_limb_24_col264, result_y_limb_25_col265,
                    result_y_limb_26_col266, result_y_limb_27_col267,
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
            pedersen_points_table_sum_0,
            range_check_9_9_sum_1,
            range_check_9_9_b_sum_2,
            range_check_9_9_c_sum_3,
            range_check_9_9_d_sum_4,
            range_check_9_9_e_sum_5,
            range_check_9_9_f_sum_6,
            range_check_9_9_g_sum_7,
            range_check_9_9_h_sum_8,
            range_check_9_9_sum_9,
            range_check_9_9_b_sum_10,
            range_check_9_9_c_sum_11,
            range_check_9_9_d_sum_12,
            range_check_9_9_e_sum_13,
            range_check_9_9_f_sum_14,
            range_check_20_sum_15,
            range_check_20_b_sum_16,
            range_check_20_c_sum_17,
            range_check_20_d_sum_18,
            range_check_20_e_sum_19,
            range_check_20_f_sum_20,
            range_check_20_g_sum_21,
            range_check_20_h_sum_22,
            range_check_20_sum_23,
            range_check_20_b_sum_24,
            range_check_20_c_sum_25,
            range_check_20_d_sum_26,
            range_check_20_e_sum_27,
            range_check_20_f_sum_28,
            range_check_20_g_sum_29,
            range_check_20_h_sum_30,
            range_check_20_sum_31,
            range_check_20_b_sum_32,
            range_check_20_c_sum_33,
            range_check_20_d_sum_34,
            range_check_20_e_sum_35,
            range_check_20_f_sum_36,
            range_check_20_g_sum_37,
            range_check_20_h_sum_38,
            range_check_20_sum_39,
            range_check_20_b_sum_40,
            range_check_20_c_sum_41,
            range_check_20_d_sum_42,
            range_check_9_9_sum_43,
            range_check_9_9_b_sum_44,
            range_check_9_9_c_sum_45,
            range_check_9_9_d_sum_46,
            range_check_9_9_e_sum_47,
            range_check_9_9_f_sum_48,
            range_check_9_9_g_sum_49,
            range_check_9_9_h_sum_50,
            range_check_9_9_sum_51,
            range_check_9_9_b_sum_52,
            range_check_9_9_c_sum_53,
            range_check_9_9_d_sum_54,
            range_check_9_9_e_sum_55,
            range_check_9_9_f_sum_56,
            range_check_20_sum_57,
            range_check_20_b_sum_58,
            range_check_20_c_sum_59,
            range_check_20_d_sum_60,
            range_check_20_e_sum_61,
            range_check_20_f_sum_62,
            range_check_20_g_sum_63,
            range_check_20_h_sum_64,
            range_check_20_sum_65,
            range_check_20_b_sum_66,
            range_check_20_c_sum_67,
            range_check_20_d_sum_68,
            range_check_20_e_sum_69,
            range_check_20_f_sum_70,
            range_check_20_g_sum_71,
            range_check_20_h_sum_72,
            range_check_20_sum_73,
            range_check_20_b_sum_74,
            range_check_20_c_sum_75,
            range_check_20_d_sum_76,
            range_check_20_e_sum_77,
            range_check_20_f_sum_78,
            range_check_20_g_sum_79,
            range_check_20_h_sum_80,
            range_check_20_sum_81,
            range_check_20_b_sum_82,
            range_check_20_c_sum_83,
            range_check_20_d_sum_84,
            range_check_9_9_sum_85,
            range_check_9_9_b_sum_86,
            range_check_9_9_c_sum_87,
            range_check_9_9_d_sum_88,
            range_check_9_9_e_sum_89,
            range_check_9_9_f_sum_90,
            range_check_9_9_g_sum_91,
            range_check_9_9_h_sum_92,
            range_check_9_9_sum_93,
            range_check_9_9_b_sum_94,
            range_check_9_9_c_sum_95,
            range_check_9_9_d_sum_96,
            range_check_9_9_e_sum_97,
            range_check_9_9_f_sum_98,
            range_check_20_sum_99,
            range_check_20_b_sum_100,
            range_check_20_c_sum_101,
            range_check_20_d_sum_102,
            range_check_20_e_sum_103,
            range_check_20_f_sum_104,
            range_check_20_g_sum_105,
            range_check_20_h_sum_106,
            range_check_20_sum_107,
            range_check_20_b_sum_108,
            range_check_20_c_sum_109,
            range_check_20_d_sum_110,
            range_check_20_e_sum_111,
            range_check_20_f_sum_112,
            range_check_20_g_sum_113,
            range_check_20_h_sum_114,
            range_check_20_sum_115,
            range_check_20_b_sum_116,
            range_check_20_c_sum_117,
            range_check_20_d_sum_118,
            range_check_20_e_sum_119,
            range_check_20_f_sum_120,
            range_check_20_g_sum_121,
            range_check_20_h_sum_122,
            range_check_20_sum_123,
            range_check_20_b_sum_124,
            range_check_20_c_sum_125,
            range_check_20_d_sum_126,
            partial_ec_mul_sum_127,
            partial_ec_mul_sum_128,
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
    pedersen_points_table_sum_0: QM31,
    range_check_9_9_sum_1: QM31,
    range_check_9_9_b_sum_2: QM31,
    range_check_9_9_c_sum_3: QM31,
    range_check_9_9_d_sum_4: QM31,
    range_check_9_9_e_sum_5: QM31,
    range_check_9_9_f_sum_6: QM31,
    range_check_9_9_g_sum_7: QM31,
    range_check_9_9_h_sum_8: QM31,
    range_check_9_9_sum_9: QM31,
    range_check_9_9_b_sum_10: QM31,
    range_check_9_9_c_sum_11: QM31,
    range_check_9_9_d_sum_12: QM31,
    range_check_9_9_e_sum_13: QM31,
    range_check_9_9_f_sum_14: QM31,
    range_check_20_sum_15: QM31,
    range_check_20_b_sum_16: QM31,
    range_check_20_c_sum_17: QM31,
    range_check_20_d_sum_18: QM31,
    range_check_20_e_sum_19: QM31,
    range_check_20_f_sum_20: QM31,
    range_check_20_g_sum_21: QM31,
    range_check_20_h_sum_22: QM31,
    range_check_20_sum_23: QM31,
    range_check_20_b_sum_24: QM31,
    range_check_20_c_sum_25: QM31,
    range_check_20_d_sum_26: QM31,
    range_check_20_e_sum_27: QM31,
    range_check_20_f_sum_28: QM31,
    range_check_20_g_sum_29: QM31,
    range_check_20_h_sum_30: QM31,
    range_check_20_sum_31: QM31,
    range_check_20_b_sum_32: QM31,
    range_check_20_c_sum_33: QM31,
    range_check_20_d_sum_34: QM31,
    range_check_20_e_sum_35: QM31,
    range_check_20_f_sum_36: QM31,
    range_check_20_g_sum_37: QM31,
    range_check_20_h_sum_38: QM31,
    range_check_20_sum_39: QM31,
    range_check_20_b_sum_40: QM31,
    range_check_20_c_sum_41: QM31,
    range_check_20_d_sum_42: QM31,
    range_check_9_9_sum_43: QM31,
    range_check_9_9_b_sum_44: QM31,
    range_check_9_9_c_sum_45: QM31,
    range_check_9_9_d_sum_46: QM31,
    range_check_9_9_e_sum_47: QM31,
    range_check_9_9_f_sum_48: QM31,
    range_check_9_9_g_sum_49: QM31,
    range_check_9_9_h_sum_50: QM31,
    range_check_9_9_sum_51: QM31,
    range_check_9_9_b_sum_52: QM31,
    range_check_9_9_c_sum_53: QM31,
    range_check_9_9_d_sum_54: QM31,
    range_check_9_9_e_sum_55: QM31,
    range_check_9_9_f_sum_56: QM31,
    range_check_20_sum_57: QM31,
    range_check_20_b_sum_58: QM31,
    range_check_20_c_sum_59: QM31,
    range_check_20_d_sum_60: QM31,
    range_check_20_e_sum_61: QM31,
    range_check_20_f_sum_62: QM31,
    range_check_20_g_sum_63: QM31,
    range_check_20_h_sum_64: QM31,
    range_check_20_sum_65: QM31,
    range_check_20_b_sum_66: QM31,
    range_check_20_c_sum_67: QM31,
    range_check_20_d_sum_68: QM31,
    range_check_20_e_sum_69: QM31,
    range_check_20_f_sum_70: QM31,
    range_check_20_g_sum_71: QM31,
    range_check_20_h_sum_72: QM31,
    range_check_20_sum_73: QM31,
    range_check_20_b_sum_74: QM31,
    range_check_20_c_sum_75: QM31,
    range_check_20_d_sum_76: QM31,
    range_check_20_e_sum_77: QM31,
    range_check_20_f_sum_78: QM31,
    range_check_20_g_sum_79: QM31,
    range_check_20_h_sum_80: QM31,
    range_check_20_sum_81: QM31,
    range_check_20_b_sum_82: QM31,
    range_check_20_c_sum_83: QM31,
    range_check_20_d_sum_84: QM31,
    range_check_9_9_sum_85: QM31,
    range_check_9_9_b_sum_86: QM31,
    range_check_9_9_c_sum_87: QM31,
    range_check_9_9_d_sum_88: QM31,
    range_check_9_9_e_sum_89: QM31,
    range_check_9_9_f_sum_90: QM31,
    range_check_9_9_g_sum_91: QM31,
    range_check_9_9_h_sum_92: QM31,
    range_check_9_9_sum_93: QM31,
    range_check_9_9_b_sum_94: QM31,
    range_check_9_9_c_sum_95: QM31,
    range_check_9_9_d_sum_96: QM31,
    range_check_9_9_e_sum_97: QM31,
    range_check_9_9_f_sum_98: QM31,
    range_check_20_sum_99: QM31,
    range_check_20_b_sum_100: QM31,
    range_check_20_c_sum_101: QM31,
    range_check_20_d_sum_102: QM31,
    range_check_20_e_sum_103: QM31,
    range_check_20_f_sum_104: QM31,
    range_check_20_g_sum_105: QM31,
    range_check_20_h_sum_106: QM31,
    range_check_20_sum_107: QM31,
    range_check_20_b_sum_108: QM31,
    range_check_20_c_sum_109: QM31,
    range_check_20_d_sum_110: QM31,
    range_check_20_e_sum_111: QM31,
    range_check_20_f_sum_112: QM31,
    range_check_20_g_sum_113: QM31,
    range_check_20_h_sum_114: QM31,
    range_check_20_sum_115: QM31,
    range_check_20_b_sum_116: QM31,
    range_check_20_c_sum_117: QM31,
    range_check_20_d_sum_118: QM31,
    range_check_20_e_sum_119: QM31,
    range_check_20_f_sum_120: QM31,
    range_check_20_g_sum_121: QM31,
    range_check_20_h_sum_122: QM31,
    range_check_20_sum_123: QM31,
    range_check_20_b_sum_124: QM31,
    range_check_20_c_sum_125: QM31,
    range_check_20_d_sum_126: QM31,
    partial_ec_mul_sum_127: QM31,
    partial_ec_mul_sum_128: QM31,
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
        trace_2_col148,
        trace_2_col149,
        trace_2_col150,
        trace_2_col151,
        trace_2_col152,
        trace_2_col153,
        trace_2_col154,
        trace_2_col155,
        trace_2_col156,
        trace_2_col157,
        trace_2_col158,
        trace_2_col159,
        trace_2_col160,
        trace_2_col161,
        trace_2_col162,
        trace_2_col163,
        trace_2_col164,
        trace_2_col165,
        trace_2_col166,
        trace_2_col167,
        trace_2_col168,
        trace_2_col169,
        trace_2_col170,
        trace_2_col171,
        trace_2_col172,
        trace_2_col173,
        trace_2_col174,
        trace_2_col175,
        trace_2_col176,
        trace_2_col177,
        trace_2_col178,
        trace_2_col179,
        trace_2_col180,
        trace_2_col181,
        trace_2_col182,
        trace_2_col183,
        trace_2_col184,
        trace_2_col185,
        trace_2_col186,
        trace_2_col187,
        trace_2_col188,
        trace_2_col189,
        trace_2_col190,
        trace_2_col191,
        trace_2_col192,
        trace_2_col193,
        trace_2_col194,
        trace_2_col195,
        trace_2_col196,
        trace_2_col197,
        trace_2_col198,
        trace_2_col199,
        trace_2_col200,
        trace_2_col201,
        trace_2_col202,
        trace_2_col203,
        trace_2_col204,
        trace_2_col205,
        trace_2_col206,
        trace_2_col207,
        trace_2_col208,
        trace_2_col209,
        trace_2_col210,
        trace_2_col211,
        trace_2_col212,
        trace_2_col213,
        trace_2_col214,
        trace_2_col215,
        trace_2_col216,
        trace_2_col217,
        trace_2_col218,
        trace_2_col219,
        trace_2_col220,
        trace_2_col221,
        trace_2_col222,
        trace_2_col223,
        trace_2_col224,
        trace_2_col225,
        trace_2_col226,
        trace_2_col227,
        trace_2_col228,
        trace_2_col229,
        trace_2_col230,
        trace_2_col231,
        trace_2_col232,
        trace_2_col233,
        trace_2_col234,
        trace_2_col235,
        trace_2_col236,
        trace_2_col237,
        trace_2_col238,
        trace_2_col239,
        trace_2_col240,
        trace_2_col241,
        trace_2_col242,
        trace_2_col243,
        trace_2_col244,
        trace_2_col245,
        trace_2_col246,
        trace_2_col247,
        trace_2_col248,
        trace_2_col249,
        trace_2_col250,
        trace_2_col251,
        trace_2_col252,
        trace_2_col253,
        trace_2_col254,
        trace_2_col255,
        trace_2_col256,
        trace_2_col257,
        trace_2_col258,
        trace_2_col259,
    ]: [Span<QM31>; 260] =
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
    let [trace_2_col144]: [QM31; 1] = (*trace_2_col144.try_into().unwrap()).unbox();
    let [trace_2_col145]: [QM31; 1] = (*trace_2_col145.try_into().unwrap()).unbox();
    let [trace_2_col146]: [QM31; 1] = (*trace_2_col146.try_into().unwrap()).unbox();
    let [trace_2_col147]: [QM31; 1] = (*trace_2_col147.try_into().unwrap()).unbox();
    let [trace_2_col148]: [QM31; 1] = (*trace_2_col148.try_into().unwrap()).unbox();
    let [trace_2_col149]: [QM31; 1] = (*trace_2_col149.try_into().unwrap()).unbox();
    let [trace_2_col150]: [QM31; 1] = (*trace_2_col150.try_into().unwrap()).unbox();
    let [trace_2_col151]: [QM31; 1] = (*trace_2_col151.try_into().unwrap()).unbox();
    let [trace_2_col152]: [QM31; 1] = (*trace_2_col152.try_into().unwrap()).unbox();
    let [trace_2_col153]: [QM31; 1] = (*trace_2_col153.try_into().unwrap()).unbox();
    let [trace_2_col154]: [QM31; 1] = (*trace_2_col154.try_into().unwrap()).unbox();
    let [trace_2_col155]: [QM31; 1] = (*trace_2_col155.try_into().unwrap()).unbox();
    let [trace_2_col156]: [QM31; 1] = (*trace_2_col156.try_into().unwrap()).unbox();
    let [trace_2_col157]: [QM31; 1] = (*trace_2_col157.try_into().unwrap()).unbox();
    let [trace_2_col158]: [QM31; 1] = (*trace_2_col158.try_into().unwrap()).unbox();
    let [trace_2_col159]: [QM31; 1] = (*trace_2_col159.try_into().unwrap()).unbox();
    let [trace_2_col160]: [QM31; 1] = (*trace_2_col160.try_into().unwrap()).unbox();
    let [trace_2_col161]: [QM31; 1] = (*trace_2_col161.try_into().unwrap()).unbox();
    let [trace_2_col162]: [QM31; 1] = (*trace_2_col162.try_into().unwrap()).unbox();
    let [trace_2_col163]: [QM31; 1] = (*trace_2_col163.try_into().unwrap()).unbox();
    let [trace_2_col164]: [QM31; 1] = (*trace_2_col164.try_into().unwrap()).unbox();
    let [trace_2_col165]: [QM31; 1] = (*trace_2_col165.try_into().unwrap()).unbox();
    let [trace_2_col166]: [QM31; 1] = (*trace_2_col166.try_into().unwrap()).unbox();
    let [trace_2_col167]: [QM31; 1] = (*trace_2_col167.try_into().unwrap()).unbox();
    let [trace_2_col168]: [QM31; 1] = (*trace_2_col168.try_into().unwrap()).unbox();
    let [trace_2_col169]: [QM31; 1] = (*trace_2_col169.try_into().unwrap()).unbox();
    let [trace_2_col170]: [QM31; 1] = (*trace_2_col170.try_into().unwrap()).unbox();
    let [trace_2_col171]: [QM31; 1] = (*trace_2_col171.try_into().unwrap()).unbox();
    let [trace_2_col172]: [QM31; 1] = (*trace_2_col172.try_into().unwrap()).unbox();
    let [trace_2_col173]: [QM31; 1] = (*trace_2_col173.try_into().unwrap()).unbox();
    let [trace_2_col174]: [QM31; 1] = (*trace_2_col174.try_into().unwrap()).unbox();
    let [trace_2_col175]: [QM31; 1] = (*trace_2_col175.try_into().unwrap()).unbox();
    let [trace_2_col176]: [QM31; 1] = (*trace_2_col176.try_into().unwrap()).unbox();
    let [trace_2_col177]: [QM31; 1] = (*trace_2_col177.try_into().unwrap()).unbox();
    let [trace_2_col178]: [QM31; 1] = (*trace_2_col178.try_into().unwrap()).unbox();
    let [trace_2_col179]: [QM31; 1] = (*trace_2_col179.try_into().unwrap()).unbox();
    let [trace_2_col180]: [QM31; 1] = (*trace_2_col180.try_into().unwrap()).unbox();
    let [trace_2_col181]: [QM31; 1] = (*trace_2_col181.try_into().unwrap()).unbox();
    let [trace_2_col182]: [QM31; 1] = (*trace_2_col182.try_into().unwrap()).unbox();
    let [trace_2_col183]: [QM31; 1] = (*trace_2_col183.try_into().unwrap()).unbox();
    let [trace_2_col184]: [QM31; 1] = (*trace_2_col184.try_into().unwrap()).unbox();
    let [trace_2_col185]: [QM31; 1] = (*trace_2_col185.try_into().unwrap()).unbox();
    let [trace_2_col186]: [QM31; 1] = (*trace_2_col186.try_into().unwrap()).unbox();
    let [trace_2_col187]: [QM31; 1] = (*trace_2_col187.try_into().unwrap()).unbox();
    let [trace_2_col188]: [QM31; 1] = (*trace_2_col188.try_into().unwrap()).unbox();
    let [trace_2_col189]: [QM31; 1] = (*trace_2_col189.try_into().unwrap()).unbox();
    let [trace_2_col190]: [QM31; 1] = (*trace_2_col190.try_into().unwrap()).unbox();
    let [trace_2_col191]: [QM31; 1] = (*trace_2_col191.try_into().unwrap()).unbox();
    let [trace_2_col192]: [QM31; 1] = (*trace_2_col192.try_into().unwrap()).unbox();
    let [trace_2_col193]: [QM31; 1] = (*trace_2_col193.try_into().unwrap()).unbox();
    let [trace_2_col194]: [QM31; 1] = (*trace_2_col194.try_into().unwrap()).unbox();
    let [trace_2_col195]: [QM31; 1] = (*trace_2_col195.try_into().unwrap()).unbox();
    let [trace_2_col196]: [QM31; 1] = (*trace_2_col196.try_into().unwrap()).unbox();
    let [trace_2_col197]: [QM31; 1] = (*trace_2_col197.try_into().unwrap()).unbox();
    let [trace_2_col198]: [QM31; 1] = (*trace_2_col198.try_into().unwrap()).unbox();
    let [trace_2_col199]: [QM31; 1] = (*trace_2_col199.try_into().unwrap()).unbox();
    let [trace_2_col200]: [QM31; 1] = (*trace_2_col200.try_into().unwrap()).unbox();
    let [trace_2_col201]: [QM31; 1] = (*trace_2_col201.try_into().unwrap()).unbox();
    let [trace_2_col202]: [QM31; 1] = (*trace_2_col202.try_into().unwrap()).unbox();
    let [trace_2_col203]: [QM31; 1] = (*trace_2_col203.try_into().unwrap()).unbox();
    let [trace_2_col204]: [QM31; 1] = (*trace_2_col204.try_into().unwrap()).unbox();
    let [trace_2_col205]: [QM31; 1] = (*trace_2_col205.try_into().unwrap()).unbox();
    let [trace_2_col206]: [QM31; 1] = (*trace_2_col206.try_into().unwrap()).unbox();
    let [trace_2_col207]: [QM31; 1] = (*trace_2_col207.try_into().unwrap()).unbox();
    let [trace_2_col208]: [QM31; 1] = (*trace_2_col208.try_into().unwrap()).unbox();
    let [trace_2_col209]: [QM31; 1] = (*trace_2_col209.try_into().unwrap()).unbox();
    let [trace_2_col210]: [QM31; 1] = (*trace_2_col210.try_into().unwrap()).unbox();
    let [trace_2_col211]: [QM31; 1] = (*trace_2_col211.try_into().unwrap()).unbox();
    let [trace_2_col212]: [QM31; 1] = (*trace_2_col212.try_into().unwrap()).unbox();
    let [trace_2_col213]: [QM31; 1] = (*trace_2_col213.try_into().unwrap()).unbox();
    let [trace_2_col214]: [QM31; 1] = (*trace_2_col214.try_into().unwrap()).unbox();
    let [trace_2_col215]: [QM31; 1] = (*trace_2_col215.try_into().unwrap()).unbox();
    let [trace_2_col216]: [QM31; 1] = (*trace_2_col216.try_into().unwrap()).unbox();
    let [trace_2_col217]: [QM31; 1] = (*trace_2_col217.try_into().unwrap()).unbox();
    let [trace_2_col218]: [QM31; 1] = (*trace_2_col218.try_into().unwrap()).unbox();
    let [trace_2_col219]: [QM31; 1] = (*trace_2_col219.try_into().unwrap()).unbox();
    let [trace_2_col220]: [QM31; 1] = (*trace_2_col220.try_into().unwrap()).unbox();
    let [trace_2_col221]: [QM31; 1] = (*trace_2_col221.try_into().unwrap()).unbox();
    let [trace_2_col222]: [QM31; 1] = (*trace_2_col222.try_into().unwrap()).unbox();
    let [trace_2_col223]: [QM31; 1] = (*trace_2_col223.try_into().unwrap()).unbox();
    let [trace_2_col224]: [QM31; 1] = (*trace_2_col224.try_into().unwrap()).unbox();
    let [trace_2_col225]: [QM31; 1] = (*trace_2_col225.try_into().unwrap()).unbox();
    let [trace_2_col226]: [QM31; 1] = (*trace_2_col226.try_into().unwrap()).unbox();
    let [trace_2_col227]: [QM31; 1] = (*trace_2_col227.try_into().unwrap()).unbox();
    let [trace_2_col228]: [QM31; 1] = (*trace_2_col228.try_into().unwrap()).unbox();
    let [trace_2_col229]: [QM31; 1] = (*trace_2_col229.try_into().unwrap()).unbox();
    let [trace_2_col230]: [QM31; 1] = (*trace_2_col230.try_into().unwrap()).unbox();
    let [trace_2_col231]: [QM31; 1] = (*trace_2_col231.try_into().unwrap()).unbox();
    let [trace_2_col232]: [QM31; 1] = (*trace_2_col232.try_into().unwrap()).unbox();
    let [trace_2_col233]: [QM31; 1] = (*trace_2_col233.try_into().unwrap()).unbox();
    let [trace_2_col234]: [QM31; 1] = (*trace_2_col234.try_into().unwrap()).unbox();
    let [trace_2_col235]: [QM31; 1] = (*trace_2_col235.try_into().unwrap()).unbox();
    let [trace_2_col236]: [QM31; 1] = (*trace_2_col236.try_into().unwrap()).unbox();
    let [trace_2_col237]: [QM31; 1] = (*trace_2_col237.try_into().unwrap()).unbox();
    let [trace_2_col238]: [QM31; 1] = (*trace_2_col238.try_into().unwrap()).unbox();
    let [trace_2_col239]: [QM31; 1] = (*trace_2_col239.try_into().unwrap()).unbox();
    let [trace_2_col240]: [QM31; 1] = (*trace_2_col240.try_into().unwrap()).unbox();
    let [trace_2_col241]: [QM31; 1] = (*trace_2_col241.try_into().unwrap()).unbox();
    let [trace_2_col242]: [QM31; 1] = (*trace_2_col242.try_into().unwrap()).unbox();
    let [trace_2_col243]: [QM31; 1] = (*trace_2_col243.try_into().unwrap()).unbox();
    let [trace_2_col244]: [QM31; 1] = (*trace_2_col244.try_into().unwrap()).unbox();
    let [trace_2_col245]: [QM31; 1] = (*trace_2_col245.try_into().unwrap()).unbox();
    let [trace_2_col246]: [QM31; 1] = (*trace_2_col246.try_into().unwrap()).unbox();
    let [trace_2_col247]: [QM31; 1] = (*trace_2_col247.try_into().unwrap()).unbox();
    let [trace_2_col248]: [QM31; 1] = (*trace_2_col248.try_into().unwrap()).unbox();
    let [trace_2_col249]: [QM31; 1] = (*trace_2_col249.try_into().unwrap()).unbox();
    let [trace_2_col250]: [QM31; 1] = (*trace_2_col250.try_into().unwrap()).unbox();
    let [trace_2_col251]: [QM31; 1] = (*trace_2_col251.try_into().unwrap()).unbox();
    let [trace_2_col252]: [QM31; 1] = (*trace_2_col252.try_into().unwrap()).unbox();
    let [trace_2_col253]: [QM31; 1] = (*trace_2_col253.try_into().unwrap()).unbox();
    let [trace_2_col254]: [QM31; 1] = (*trace_2_col254.try_into().unwrap()).unbox();
    let [trace_2_col255]: [QM31; 1] = (*trace_2_col255.try_into().unwrap()).unbox();
    let [trace_2_col256_neg1, trace_2_col256]: [QM31; 2] = (*trace_2_col256.try_into().unwrap())
        .unbox();
    let [trace_2_col257_neg1, trace_2_col257]: [QM31; 2] = (*trace_2_col257.try_into().unwrap())
        .unbox();
    let [trace_2_col258_neg1, trace_2_col258]: [QM31; 2] = (*trace_2_col258.try_into().unwrap())
        .unbox();
    let [trace_2_col259_neg1, trace_2_col259]: [QM31; 2] = (*trace_2_col259.try_into().unwrap())
        .unbox();

    core::internal::revoke_ap_tracking();

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3],
    ))
        * pedersen_points_table_sum_0
        * range_check_9_9_sum_1)
        - pedersen_points_table_sum_0
        - range_check_9_9_sum_1)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7],
    )
        - QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3]))
        * range_check_9_9_b_sum_2
        * range_check_9_9_c_sum_3)
        - range_check_9_9_b_sum_2
        - range_check_9_9_c_sum_3)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11],
    )
        - QM31Impl::from_partial_evals([trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7]))
        * range_check_9_9_d_sum_4
        * range_check_9_9_e_sum_5)
        - range_check_9_9_d_sum_4
        - range_check_9_9_e_sum_5)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
    )
        - QM31Impl::from_partial_evals([trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11]))
        * range_check_9_9_f_sum_6
        * range_check_9_9_g_sum_7)
        - range_check_9_9_f_sum_6
        - range_check_9_9_g_sum_7)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
        ))
        * range_check_9_9_h_sum_8
        * range_check_9_9_sum_9)
        - range_check_9_9_h_sum_8
        - range_check_9_9_sum_9)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
        ))
        * range_check_9_9_b_sum_10
        * range_check_9_9_c_sum_11)
        - range_check_9_9_b_sum_10
        - range_check_9_9_c_sum_11)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23],
        ))
        * range_check_9_9_d_sum_12
        * range_check_9_9_e_sum_13)
        - range_check_9_9_d_sum_12
        - range_check_9_9_e_sum_13)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col28, trace_2_col29, trace_2_col30, trace_2_col31],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27],
        ))
        * range_check_9_9_f_sum_14
        * range_check_20_sum_15)
        - range_check_9_9_f_sum_14
        - range_check_20_sum_15)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col32, trace_2_col33, trace_2_col34, trace_2_col35],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col28, trace_2_col29, trace_2_col30, trace_2_col31],
        ))
        * range_check_20_b_sum_16
        * range_check_20_c_sum_17)
        - range_check_20_b_sum_16
        - range_check_20_c_sum_17)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col36, trace_2_col37, trace_2_col38, trace_2_col39],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col32, trace_2_col33, trace_2_col34, trace_2_col35],
        ))
        * range_check_20_d_sum_18
        * range_check_20_e_sum_19)
        - range_check_20_d_sum_18
        - range_check_20_e_sum_19)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col40, trace_2_col41, trace_2_col42, trace_2_col43],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col36, trace_2_col37, trace_2_col38, trace_2_col39],
        ))
        * range_check_20_f_sum_20
        * range_check_20_g_sum_21)
        - range_check_20_f_sum_20
        - range_check_20_g_sum_21)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col44, trace_2_col45, trace_2_col46, trace_2_col47],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col40, trace_2_col41, trace_2_col42, trace_2_col43],
        ))
        * range_check_20_h_sum_22
        * range_check_20_sum_23)
        - range_check_20_h_sum_22
        - range_check_20_sum_23)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col48, trace_2_col49, trace_2_col50, trace_2_col51],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col44, trace_2_col45, trace_2_col46, trace_2_col47],
        ))
        * range_check_20_b_sum_24
        * range_check_20_c_sum_25)
        - range_check_20_b_sum_24
        - range_check_20_c_sum_25)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col52, trace_2_col53, trace_2_col54, trace_2_col55],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col48, trace_2_col49, trace_2_col50, trace_2_col51],
        ))
        * range_check_20_d_sum_26
        * range_check_20_e_sum_27)
        - range_check_20_d_sum_26
        - range_check_20_e_sum_27)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col56, trace_2_col57, trace_2_col58, trace_2_col59],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col52, trace_2_col53, trace_2_col54, trace_2_col55],
        ))
        * range_check_20_f_sum_28
        * range_check_20_g_sum_29)
        - range_check_20_f_sum_28
        - range_check_20_g_sum_29)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col60, trace_2_col61, trace_2_col62, trace_2_col63],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col56, trace_2_col57, trace_2_col58, trace_2_col59],
        ))
        * range_check_20_h_sum_30
        * range_check_20_sum_31)
        - range_check_20_h_sum_30
        - range_check_20_sum_31)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col64, trace_2_col65, trace_2_col66, trace_2_col67],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col60, trace_2_col61, trace_2_col62, trace_2_col63],
        ))
        * range_check_20_b_sum_32
        * range_check_20_c_sum_33)
        - range_check_20_b_sum_32
        - range_check_20_c_sum_33)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col68, trace_2_col69, trace_2_col70, trace_2_col71],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col64, trace_2_col65, trace_2_col66, trace_2_col67],
        ))
        * range_check_20_d_sum_34
        * range_check_20_e_sum_35)
        - range_check_20_d_sum_34
        - range_check_20_e_sum_35)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col72, trace_2_col73, trace_2_col74, trace_2_col75],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col68, trace_2_col69, trace_2_col70, trace_2_col71],
        ))
        * range_check_20_f_sum_36
        * range_check_20_g_sum_37)
        - range_check_20_f_sum_36
        - range_check_20_g_sum_37)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col76, trace_2_col77, trace_2_col78, trace_2_col79],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col72, trace_2_col73, trace_2_col74, trace_2_col75],
        ))
        * range_check_20_h_sum_38
        * range_check_20_sum_39)
        - range_check_20_h_sum_38
        - range_check_20_sum_39)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col80, trace_2_col81, trace_2_col82, trace_2_col83],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col76, trace_2_col77, trace_2_col78, trace_2_col79],
        ))
        * range_check_20_b_sum_40
        * range_check_20_c_sum_41)
        - range_check_20_b_sum_40
        - range_check_20_c_sum_41)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col84, trace_2_col85, trace_2_col86, trace_2_col87],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col80, trace_2_col81, trace_2_col82, trace_2_col83],
        ))
        * range_check_20_d_sum_42
        * range_check_9_9_sum_43)
        - range_check_20_d_sum_42
        - range_check_9_9_sum_43)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col88, trace_2_col89, trace_2_col90, trace_2_col91],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col84, trace_2_col85, trace_2_col86, trace_2_col87],
        ))
        * range_check_9_9_b_sum_44
        * range_check_9_9_c_sum_45)
        - range_check_9_9_b_sum_44
        - range_check_9_9_c_sum_45)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col92, trace_2_col93, trace_2_col94, trace_2_col95],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col88, trace_2_col89, trace_2_col90, trace_2_col91],
        ))
        * range_check_9_9_d_sum_46
        * range_check_9_9_e_sum_47)
        - range_check_9_9_d_sum_46
        - range_check_9_9_e_sum_47)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col96, trace_2_col97, trace_2_col98, trace_2_col99],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col92, trace_2_col93, trace_2_col94, trace_2_col95],
        ))
        * range_check_9_9_f_sum_48
        * range_check_9_9_g_sum_49)
        - range_check_9_9_f_sum_48
        - range_check_9_9_g_sum_49)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col100, trace_2_col101, trace_2_col102, trace_2_col103],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col96, trace_2_col97, trace_2_col98, trace_2_col99],
        ))
        * range_check_9_9_h_sum_50
        * range_check_9_9_sum_51)
        - range_check_9_9_h_sum_50
        - range_check_9_9_sum_51)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col104, trace_2_col105, trace_2_col106, trace_2_col107],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col100, trace_2_col101, trace_2_col102, trace_2_col103],
        ))
        * range_check_9_9_b_sum_52
        * range_check_9_9_c_sum_53)
        - range_check_9_9_b_sum_52
        - range_check_9_9_c_sum_53)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col108, trace_2_col109, trace_2_col110, trace_2_col111],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col104, trace_2_col105, trace_2_col106, trace_2_col107],
        ))
        * range_check_9_9_d_sum_54
        * range_check_9_9_e_sum_55)
        - range_check_9_9_d_sum_54
        - range_check_9_9_e_sum_55)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col112, trace_2_col113, trace_2_col114, trace_2_col115],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col108, trace_2_col109, trace_2_col110, trace_2_col111],
        ))
        * range_check_9_9_f_sum_56
        * range_check_20_sum_57)
        - range_check_9_9_f_sum_56
        - range_check_20_sum_57)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col116, trace_2_col117, trace_2_col118, trace_2_col119],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col112, trace_2_col113, trace_2_col114, trace_2_col115],
        ))
        * range_check_20_b_sum_58
        * range_check_20_c_sum_59)
        - range_check_20_b_sum_58
        - range_check_20_c_sum_59)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col120, trace_2_col121, trace_2_col122, trace_2_col123],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col116, trace_2_col117, trace_2_col118, trace_2_col119],
        ))
        * range_check_20_d_sum_60
        * range_check_20_e_sum_61)
        - range_check_20_d_sum_60
        - range_check_20_e_sum_61)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col124, trace_2_col125, trace_2_col126, trace_2_col127],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col120, trace_2_col121, trace_2_col122, trace_2_col123],
        ))
        * range_check_20_f_sum_62
        * range_check_20_g_sum_63)
        - range_check_20_f_sum_62
        - range_check_20_g_sum_63)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col128, trace_2_col129, trace_2_col130, trace_2_col131],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col124, trace_2_col125, trace_2_col126, trace_2_col127],
        ))
        * range_check_20_h_sum_64
        * range_check_20_sum_65)
        - range_check_20_h_sum_64
        - range_check_20_sum_65)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col132, trace_2_col133, trace_2_col134, trace_2_col135],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col128, trace_2_col129, trace_2_col130, trace_2_col131],
        ))
        * range_check_20_b_sum_66
        * range_check_20_c_sum_67)
        - range_check_20_b_sum_66
        - range_check_20_c_sum_67)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col136, trace_2_col137, trace_2_col138, trace_2_col139],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col132, trace_2_col133, trace_2_col134, trace_2_col135],
        ))
        * range_check_20_d_sum_68
        * range_check_20_e_sum_69)
        - range_check_20_d_sum_68
        - range_check_20_e_sum_69)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col140, trace_2_col141, trace_2_col142, trace_2_col143],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col136, trace_2_col137, trace_2_col138, trace_2_col139],
        ))
        * range_check_20_f_sum_70
        * range_check_20_g_sum_71)
        - range_check_20_f_sum_70
        - range_check_20_g_sum_71)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col144, trace_2_col145, trace_2_col146, trace_2_col147],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col140, trace_2_col141, trace_2_col142, trace_2_col143],
        ))
        * range_check_20_h_sum_72
        * range_check_20_sum_73)
        - range_check_20_h_sum_72
        - range_check_20_sum_73)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col148, trace_2_col149, trace_2_col150, trace_2_col151],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col144, trace_2_col145, trace_2_col146, trace_2_col147],
        ))
        * range_check_20_b_sum_74
        * range_check_20_c_sum_75)
        - range_check_20_b_sum_74
        - range_check_20_c_sum_75)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col152, trace_2_col153, trace_2_col154, trace_2_col155],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col148, trace_2_col149, trace_2_col150, trace_2_col151],
        ))
        * range_check_20_d_sum_76
        * range_check_20_e_sum_77)
        - range_check_20_d_sum_76
        - range_check_20_e_sum_77)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col156, trace_2_col157, trace_2_col158, trace_2_col159],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col152, trace_2_col153, trace_2_col154, trace_2_col155],
        ))
        * range_check_20_f_sum_78
        * range_check_20_g_sum_79)
        - range_check_20_f_sum_78
        - range_check_20_g_sum_79)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col160, trace_2_col161, trace_2_col162, trace_2_col163],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col156, trace_2_col157, trace_2_col158, trace_2_col159],
        ))
        * range_check_20_h_sum_80
        * range_check_20_sum_81)
        - range_check_20_h_sum_80
        - range_check_20_sum_81)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col164, trace_2_col165, trace_2_col166, trace_2_col167],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col160, trace_2_col161, trace_2_col162, trace_2_col163],
        ))
        * range_check_20_b_sum_82
        * range_check_20_c_sum_83)
        - range_check_20_b_sum_82
        - range_check_20_c_sum_83)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col168, trace_2_col169, trace_2_col170, trace_2_col171],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col164, trace_2_col165, trace_2_col166, trace_2_col167],
        ))
        * range_check_20_d_sum_84
        * range_check_9_9_sum_85)
        - range_check_20_d_sum_84
        - range_check_9_9_sum_85)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col172, trace_2_col173, trace_2_col174, trace_2_col175],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col168, trace_2_col169, trace_2_col170, trace_2_col171],
        ))
        * range_check_9_9_b_sum_86
        * range_check_9_9_c_sum_87)
        - range_check_9_9_b_sum_86
        - range_check_9_9_c_sum_87)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col176, trace_2_col177, trace_2_col178, trace_2_col179],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col172, trace_2_col173, trace_2_col174, trace_2_col175],
        ))
        * range_check_9_9_d_sum_88
        * range_check_9_9_e_sum_89)
        - range_check_9_9_d_sum_88
        - range_check_9_9_e_sum_89)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col180, trace_2_col181, trace_2_col182, trace_2_col183],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col176, trace_2_col177, trace_2_col178, trace_2_col179],
        ))
        * range_check_9_9_f_sum_90
        * range_check_9_9_g_sum_91)
        - range_check_9_9_f_sum_90
        - range_check_9_9_g_sum_91)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col184, trace_2_col185, trace_2_col186, trace_2_col187],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col180, trace_2_col181, trace_2_col182, trace_2_col183],
        ))
        * range_check_9_9_h_sum_92
        * range_check_9_9_sum_93)
        - range_check_9_9_h_sum_92
        - range_check_9_9_sum_93)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col188, trace_2_col189, trace_2_col190, trace_2_col191],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col184, trace_2_col185, trace_2_col186, trace_2_col187],
        ))
        * range_check_9_9_b_sum_94
        * range_check_9_9_c_sum_95)
        - range_check_9_9_b_sum_94
        - range_check_9_9_c_sum_95)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col192, trace_2_col193, trace_2_col194, trace_2_col195],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col188, trace_2_col189, trace_2_col190, trace_2_col191],
        ))
        * range_check_9_9_d_sum_96
        * range_check_9_9_e_sum_97)
        - range_check_9_9_d_sum_96
        - range_check_9_9_e_sum_97)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col196, trace_2_col197, trace_2_col198, trace_2_col199],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col192, trace_2_col193, trace_2_col194, trace_2_col195],
        ))
        * range_check_9_9_f_sum_98
        * range_check_20_sum_99)
        - range_check_9_9_f_sum_98
        - range_check_20_sum_99)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col200, trace_2_col201, trace_2_col202, trace_2_col203],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col196, trace_2_col197, trace_2_col198, trace_2_col199],
        ))
        * range_check_20_b_sum_100
        * range_check_20_c_sum_101)
        - range_check_20_b_sum_100
        - range_check_20_c_sum_101)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col204, trace_2_col205, trace_2_col206, trace_2_col207],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col200, trace_2_col201, trace_2_col202, trace_2_col203],
        ))
        * range_check_20_d_sum_102
        * range_check_20_e_sum_103)
        - range_check_20_d_sum_102
        - range_check_20_e_sum_103)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col208, trace_2_col209, trace_2_col210, trace_2_col211],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col204, trace_2_col205, trace_2_col206, trace_2_col207],
        ))
        * range_check_20_f_sum_104
        * range_check_20_g_sum_105)
        - range_check_20_f_sum_104
        - range_check_20_g_sum_105)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col212, trace_2_col213, trace_2_col214, trace_2_col215],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col208, trace_2_col209, trace_2_col210, trace_2_col211],
        ))
        * range_check_20_h_sum_106
        * range_check_20_sum_107)
        - range_check_20_h_sum_106
        - range_check_20_sum_107)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col216, trace_2_col217, trace_2_col218, trace_2_col219],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col212, trace_2_col213, trace_2_col214, trace_2_col215],
        ))
        * range_check_20_b_sum_108
        * range_check_20_c_sum_109)
        - range_check_20_b_sum_108
        - range_check_20_c_sum_109)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col220, trace_2_col221, trace_2_col222, trace_2_col223],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col216, trace_2_col217, trace_2_col218, trace_2_col219],
        ))
        * range_check_20_d_sum_110
        * range_check_20_e_sum_111)
        - range_check_20_d_sum_110
        - range_check_20_e_sum_111)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col224, trace_2_col225, trace_2_col226, trace_2_col227],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col220, trace_2_col221, trace_2_col222, trace_2_col223],
        ))
        * range_check_20_f_sum_112
        * range_check_20_g_sum_113)
        - range_check_20_f_sum_112
        - range_check_20_g_sum_113)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col228, trace_2_col229, trace_2_col230, trace_2_col231],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col224, trace_2_col225, trace_2_col226, trace_2_col227],
        ))
        * range_check_20_h_sum_114
        * range_check_20_sum_115)
        - range_check_20_h_sum_114
        - range_check_20_sum_115)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col232, trace_2_col233, trace_2_col234, trace_2_col235],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col228, trace_2_col229, trace_2_col230, trace_2_col231],
        ))
        * range_check_20_b_sum_116
        * range_check_20_c_sum_117)
        - range_check_20_b_sum_116
        - range_check_20_c_sum_117)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col236, trace_2_col237, trace_2_col238, trace_2_col239],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col232, trace_2_col233, trace_2_col234, trace_2_col235],
        ))
        * range_check_20_d_sum_118
        * range_check_20_e_sum_119)
        - range_check_20_d_sum_118
        - range_check_20_e_sum_119)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col240, trace_2_col241, trace_2_col242, trace_2_col243],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col236, trace_2_col237, trace_2_col238, trace_2_col239],
        ))
        * range_check_20_f_sum_120
        * range_check_20_g_sum_121)
        - range_check_20_f_sum_120
        - range_check_20_g_sum_121)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col244, trace_2_col245, trace_2_col246, trace_2_col247],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col240, trace_2_col241, trace_2_col242, trace_2_col243],
        ))
        * range_check_20_h_sum_122
        * range_check_20_sum_123)
        - range_check_20_h_sum_122
        - range_check_20_sum_123)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col248, trace_2_col249, trace_2_col250, trace_2_col251],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col244, trace_2_col245, trace_2_col246, trace_2_col247],
        ))
        * range_check_20_b_sum_124
        * range_check_20_c_sum_125)
        - range_check_20_b_sum_124
        - range_check_20_c_sum_125)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col252, trace_2_col253, trace_2_col254, trace_2_col255],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col248, trace_2_col249, trace_2_col250, trace_2_col251],
        ))
        * range_check_20_d_sum_126
        * partial_ec_mul_sum_127)
        - (range_check_20_d_sum_126 * enabler)
        - partial_ec_mul_sum_127)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col256, trace_2_col257, trace_2_col258, trace_2_col259],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col252, trace_2_col253, trace_2_col254, trace_2_col255],
        )
        - QM31Impl::from_partial_evals(
            [trace_2_col256_neg1, trace_2_col257_neg1, trace_2_col258_neg1, trace_2_col259_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * partial_ec_mul_sum_128)
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
            partial_ec_mul_lookup_elements: make_lookup_elements(
                qm31_const::<1649646149, 853343631, 2092831524, 2004475967>(),
                qm31_const::<566949925, 426542195, 926007664, 380048330>(),
            ),
            pedersen_points_table_lookup_elements: make_lookup_elements(
                qm31_const::<1429392413, 324063524, 996177436, 120316387>(),
                qm31_const::<607706587, 464765845, 1730008433, 812537185>(),
            ),
            range_check_20_lookup_elements: make_lookup_elements(
                qm31_const::<1932860727, 18341367, 2045797860, 1199128296>(),
                qm31_const::<155807269, 899318514, 1185754398, 1543210647>(),
            ),
            range_check_20_b_lookup_elements: make_lookup_elements(
                qm31_const::<1007364761, 205123076, 1260180806, 296890037>(),
                qm31_const::<145477934, 1058074746, 1168116711, 435369134>(),
            ),
            range_check_20_c_lookup_elements: make_lookup_elements(
                qm31_const::<429346395, 1144671906, 656225764, 296898282>(),
                qm31_const::<1230482199, 1307250087, 1633915804, 2134139353>(),
            ),
            range_check_20_d_lookup_elements: make_lookup_elements(
                qm31_const::<1882422753, 2084114286, 52164273, 296871044>(),
                qm31_const::<1590055113, 1284130096, 2092337358, 877660413>(),
            ),
            range_check_20_e_lookup_elements: make_lookup_elements(
                qm31_const::<1298012403, 876175677, 1595689087, 296878024>(),
                qm31_const::<722537063, 1440411207, 346964784, 1123545529>(),
            ),
            range_check_20_f_lookup_elements: make_lookup_elements(
                qm31_const::<1146789882, 741880003, 1528502277, 296852042>(),
                qm31_const::<946610143, 1094045880, 453212534, 1897158948>(),
            ),
            range_check_20_g_lookup_elements: make_lookup_elements(
                qm31_const::<562379532, 1681425041, 924543443, 296859023>(),
                qm31_const::<424893500, 1310028305, 915116907, 731319435>(),
            ),
            range_check_20_h_lookup_elements: make_lookup_elements(
                qm31_const::<2028159210, 473391334, 320489513, 296834305>(),
                qm31_const::<875540222, 1963593080, 1991994536, 1828476143>(),
            ),
            range_check_9_9_lookup_elements: make_lookup_elements(
                qm31_const::<989827041, 1225728465, 1602128278, 85336129>(),
                qm31_const::<1454375758, 8286589, 1713209810, 1602293816>(),
            ),
            range_check_9_9_b_lookup_elements: make_lookup_elements(
                qm31_const::<676159317, 930503385, 1105489908, 1544380136>(),
                qm31_const::<2129889251, 701815395, 1830411342, 2061777868>(),
            ),
            range_check_9_9_c_lookup_elements: make_lookup_elements(
                qm31_const::<1260569667, 2138441994, 1709448741, 1544373155>(),
                qm31_const::<1022885008, 826842007, 1709607881, 1909661957>(),
            ),
            range_check_9_9_d_lookup_elements: make_lookup_elements(
                qm31_const::<1551136661, 662010924, 2044956999, 1544361134>(),
                qm31_const::<2005146556, 852740197, 532387412, 1763320973>(),
            ),
            range_check_9_9_e_lookup_elements: make_lookup_elements(
                qm31_const::<2135547011, 1869949533, 501432185, 1544354154>(),
                qm31_const::<1771048649, 362596150, 1943805170, 690289666>(),
            ),
            range_check_9_9_f_lookup_elements: make_lookup_elements(
                qm31_const::<821895774, 1467264080, 1373815147, 1544343397>(),
                qm31_const::<1435956769, 1381290646, 1730080787, 865114040>(),
            ),
            range_check_9_9_g_lookup_elements: make_lookup_elements(
                qm31_const::<1406306124, 527719042, 1977773981, 1544336416>(),
                qm31_const::<1018085498, 759742390, 862702750, 464139937>(),
            ),
            range_check_9_9_h_lookup_elements: make_lookup_elements(
                qm31_const::<1696953766, 1198771643, 165798615, 1544324404>(),
                qm31_const::<933744903, 1518924215, 418396039, 1277931404>(),
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
            [qm31_const::<82600780, 2147084962, 2004018121, 502540171>()].span(),
            [qm31_const::<149709959, 133819043, 2071126986, 502540171>()].span(),
            [qm31_const::<216819138, 268036771, 2138235850, 502540171>()].span(),
            [qm31_const::<1894538532, 1475996321, 1668473801, 502540171>()].span(),
            [qm31_const::<1961647711, 1610214049, 1735582665, 502540171>()].span(),
            [qm31_const::<2028756890, 1744431777, 1802691529, 502540171>()].span(),
            [qm31_const::<2095866069, 1878649505, 1869800393, 502540171>()].span(),
            [qm31_const::<552365033, 939125411, 326296523, 502540172>()].span(),
            [qm31_const::<619474212, 1073343139, 393405387, 502540172>()].span(),
            [qm31_const::<149976820, 133819211, 2071127154, 502540227>()].span(),
            [qm31_const::<82867641, 2147085130, 2004018289, 502540227>()].span(),
            [qm31_const::<284195178, 402254667, 57861235, 502540228>()].span(),
            [qm31_const::<217085999, 268036939, 2138236018, 502540227>()].span(),
            [qm31_const::<2029023751, 1744431945, 1802691697, 502540227>()].span(),
            [qm31_const::<1961914572, 1610214217, 1735582833, 502540227>()].span(),
            [qm31_const::<15758462, 2012867402, 1936909425, 502540227>()].span(),
            [qm31_const::<2096132930, 1878649673, 1869800561, 502540227>()].span(),
            [qm31_const::<686850252, 1207561035, 460514419, 502540228>()].span(),
            [qm31_const::<619741073, 1073343307, 393405555, 502540228>()].span(),
            [qm31_const::<820966215, 1475996431, 594732087, 502540208>()].span(),
            [qm31_const::<888075394, 1610214159, 661840951, 502540208>()].span(),
            [qm31_const::<686747857, 1207560975, 460514359, 502540208>()].span(),
            [qm31_const::<753857036, 1341778703, 527623223, 502540208>()].span(),
            [qm31_const::<1089402931, 2012867343, 863167543, 502540208>()].span(),
            [qm31_const::<1156512110, 2147085071, 930276407, 502540208>()].span(),
            [qm31_const::<955184573, 1744431887, 728949815, 502540208>()].span(),
            [qm31_const::<1022293752, 1878649615, 796058679, 502540208>()].span(),
            [qm31_const::<284092783, 402254607, 57861175, 502540208>()].span(),
            [qm31_const::<351201962, 536472335, 124970039, 502540208>()].span(),
            [qm31_const::<2028449705, 1744431597, 1802691349, 502540111>()].span(),
            [qm31_const::<1961340526, 1610213869, 1735582485, 502540111>()].span(),
            [qm31_const::<1894231347, 1475996141, 1668473621, 502540111>()].span(),
            [qm31_const::<1827122168, 1341778413, 1601364757, 502540111>()].span(),
            [qm31_const::<149402774, 133818863, 2071126806, 502540111>()].span(),
            [qm31_const::<82293595, 2147084782, 2004017941, 502540111>()].span(),
            [qm31_const::<15184416, 2012867054, 1936909077, 502540111>()].span(),
            [qm31_const::<2095558884, 1878649325, 1869800213, 502540111>()].span(),
            [qm31_const::<417839490, 670689775, 192078615, 502540112>()].span(),
            [qm31_const::<350730311, 536472047, 124969751, 502540112>()].span(),
            [qm31_const::<551955453, 939125171, 326296283, 502540092>()].span(),
            [qm31_const::<619064632, 1073342899, 393405147, 502540092>()].span(),
            [qm31_const::<686173811, 1207560627, 460514011, 502540092>()].span(),
            [qm31_const::<753282990, 1341778355, 527622875, 502540092>()].span(),
            [qm31_const::<820392169, 1475996083, 594731739, 502540092>()].span(),
            [qm31_const::<887501348, 1610213811, 661840603, 502540092>()].span(),
            [qm31_const::<954610527, 1744431539, 728949467, 502540092>()].span(),
            [qm31_const::<1021719706, 1878649267, 796058331, 502540092>()].span(),
            [qm31_const::<15082021, 2012866994, 1936909017, 502540091>()].span(),
            [qm31_const::<82191200, 2147084722, 2004017881, 502540091>()].span(),
            [qm31_const::<686480996, 1207560807, 460514191, 502540152>()].span(),
            [qm31_const::<619371817, 1073343079, 393405327, 502540152>()].span(),
            [qm31_const::<820699354, 1475996263, 594731919, 502540152>()].span(),
            [qm31_const::<753590175, 1341778535, 527623055, 502540152>()].span(),
            [qm31_const::<954917712, 1744431719, 728949647, 502540152>()].span(),
            [qm31_const::<887808533, 1610213991, 661840783, 502540152>()].span(),
            [qm31_const::<1089136070, 2012867175, 863167375, 502540152>()].span(),
            [qm31_const::<1022026891, 1878649447, 796058511, 502540152>()].span(),
            [qm31_const::<149607564, 133818983, 2071126926, 502540151>()].span(),
            [qm31_const::<82498385, 2147084902, 2004018061, 502540151>()].span(),
            [qm31_const::<1357470391, 402254380, 1131602772, 502540132>()].span(),
            [qm31_const::<1424579570, 536472108, 1198711636, 502540132>()].span(),
            [qm31_const::<1223252033, 133818924, 997385044, 502540132>()].span(),
            [qm31_const::<1290361212, 268036652, 1064493908, 502540132>()].span(),
            [qm31_const::<1089033675, 2012867115, 863167315, 502540132>()].span(),
            [qm31_const::<1156142854, 2147084843, 930276179, 502540132>()].span(),
            [qm31_const::<954815317, 1744431659, 728949587, 502540132>()].span(),
            [qm31_const::<1021924496, 1878649387, 796058451, 502540132>()].span(),
            [qm31_const::<820596959, 1475996203, 594731859, 502540132>()].span(),
            [qm31_const::<887706138, 1610213931, 661840723, 502540132>()].span(),
            [qm31_const::<417429910, 670689535, 192078375, 502540032>()].span(),
            [qm31_const::<350320731, 536471807, 124969511, 502540032>()].span(),
            [qm31_const::<283211552, 402254079, 57860647, 502540032>()].span(),
            [qm31_const::<216102373, 268036351, 2138235430, 502540031>()].span(),
            [qm31_const::<148993194, 133818623, 2071126566, 502540031>()].span(),
            [qm31_const::<81884015, 2147084542, 2004017701, 502540031>()].span(),
            [qm31_const::<14774836, 2012866814, 1936908837, 502540031>()].span(),
            [qm31_const::<2095149304, 1878649085, 1869799973, 502540031>()].span(),
            [qm31_const::<954303342, 1744431359, 728949287, 502540032>()].span(),
            [qm31_const::<887194163, 1610213631, 661840423, 502540032>()].span(),
            [qm31_const::<1088419305, 2012866755, 863166955, 502540012>()].span(),
            [qm31_const::<1155528484, 2147084483, 930275819, 502540012>()].span(),
            [qm31_const::<1222637663, 133818564, 997384684, 502540012>()].span(),
            [qm31_const::<1289746842, 268036292, 1064493548, 502540012>()].span(),
            [qm31_const::<819982589, 1475995843, 594731499, 502540012>()].span(),
            [qm31_const::<887091768, 1610213571, 661840363, 502540012>()].span(),
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
            qm31_const::<34562228, 1314060769, 194592989, 1261630247>(),
            qm31_const::<1846499980, 642972128, 2006532316, 1261630246>(),
            qm31_const::<1779390801, 508754400, 1939423452, 1261630246>(),
            qm31_const::<637858317, 374536263, 798572355, 1261630110>(),
            qm31_const::<704967496, 508753991, 865681219, 1261630110>(),
            qm31_const::<772076675, 642971719, 932790083, 1261630110>(),
            qm31_const::<839185854, 777189447, 999898947, 1261630110>(),
            qm31_const::<906295033, 911407175, 1067007811, 1261630110>(),
            qm31_const::<973404212, 1045624903, 1134116675, 1261630110>(),
            qm31_const::<1040513391, 1179842631, 1201225539, 1261630110>(),
            qm31_const::<1107622570, 1314060359, 1268334403, 1261630110>(),
            qm31_const::<1174731749, 1448278087, 1335443267, 1261630110>(),
            qm31_const::<1241840928, 1582495815, 1402552131, 1261630110>(),
            qm31_const::<2114362650, 1179842692, 127483777, 1261630131>(),
            qm31_const::<2047253471, 1045624964, 60374913, 1261630131>(),
            qm31_const::<1980144292, 911407236, 2140749696, 1261630130>(),
            qm31_const::<1913035113, 777189508, 2073640832, 1261630130>(),
            qm31_const::<235315719, 1716713605, 395919233, 1261630131>(),
            qm31_const::<168206540, 1582495877, 328810369, 1261630131>(),
            qm31_const::<101097361, 1448278149, 261701505, 1261630131>(),
            qm31_const::<33988182, 1314060421, 194592641, 1261630131>(),
            qm31_const::<1577489218, 106100868, 1738096512, 1261630130>(),
            qm31_const::<1510380039, 2119366787, 1670987647, 1261630130>(),
            qm31_const::<1443373255, 1985149119, 1603878843, 1261630150>(),
            qm31_const::<1510482434, 2119366847, 1670987707, 1261630150>(),
            qm31_const::<1309154897, 1716713663, 1469661115, 1261630150>(),
            qm31_const::<1376264076, 1850931391, 1536769979, 1261630150>(),
            qm31_const::<1174936539, 1448278207, 1335443387, 1261630150>(),
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
        assert_eq!(sum, QM31Trait::from_fixed_array(PARTIAL_EC_MUL_SAMPLE_EVAL_RESULT))
    }
}
