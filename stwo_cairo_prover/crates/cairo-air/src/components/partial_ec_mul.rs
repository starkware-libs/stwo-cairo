// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::ec_add::EcAdd;

pub const N_TRACE_COLUMNS: usize = 522;
pub const RELATION_USES_PER_ROW: [RelationUse; 18] = [
    RelationUse {
        relation_id: "PartialEcMul",
        uses: 1,
    },
    RelationUse {
        relation_id: "PedersenPointsTable",
        uses: 2,
    },
    RelationUse {
        relation_id: "RangeCheck_20",
        uses: 24,
    },
    RelationUse {
        relation_id: "RangeCheck_20_B",
        uses: 24,
    },
    RelationUse {
        relation_id: "RangeCheck_20_C",
        uses: 24,
    },
    RelationUse {
        relation_id: "RangeCheck_20_D",
        uses: 24,
    },
    RelationUse {
        relation_id: "RangeCheck_20_E",
        uses: 18,
    },
    RelationUse {
        relation_id: "RangeCheck_20_F",
        uses: 18,
    },
    RelationUse {
        relation_id: "RangeCheck_20_G",
        uses: 18,
    },
    RelationUse {
        relation_id: "RangeCheck_20_H",
        uses: 18,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9",
        uses: 13,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_B",
        uses: 12,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_C",
        uses: 12,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_D",
        uses: 12,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_E",
        uses: 12,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_F",
        uses: 12,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_G",
        uses: 6,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_H",
        uses: 6,
    },
];

pub struct Eval {
    pub claim: Claim,
    pub range_check_9_9_lookup_elements: relations::RangeCheck_9_9,
    pub pedersen_points_table_lookup_elements: relations::PedersenPointsTable,
    pub range_check_9_9_b_lookup_elements: relations::RangeCheck_9_9_B,
    pub range_check_9_9_c_lookup_elements: relations::RangeCheck_9_9_C,
    pub range_check_9_9_d_lookup_elements: relations::RangeCheck_9_9_D,
    pub range_check_9_9_e_lookup_elements: relations::RangeCheck_9_9_E,
    pub range_check_9_9_f_lookup_elements: relations::RangeCheck_9_9_F,
    pub range_check_9_9_g_lookup_elements: relations::RangeCheck_9_9_G,
    pub range_check_9_9_h_lookup_elements: relations::RangeCheck_9_9_H,
    pub range_check_20_lookup_elements: relations::RangeCheck_20,
    pub range_check_20_b_lookup_elements: relations::RangeCheck_20_B,
    pub range_check_20_c_lookup_elements: relations::RangeCheck_20_C,
    pub range_check_20_d_lookup_elements: relations::RangeCheck_20_D,
    pub range_check_20_e_lookup_elements: relations::RangeCheck_20_E,
    pub range_check_20_f_lookup_elements: relations::RangeCheck_20_F,
    pub range_check_20_g_lookup_elements: relations::RangeCheck_20_G,
    pub range_check_20_h_lookup_elements: relations::RangeCheck_20_H,
    pub partial_ec_mul_lookup_elements: relations::PartialEcMul,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct Claim {
    pub log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 129];
        TreeVec::new(vec![vec![], trace_log_sizes, interaction_log_sizes])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(self.log_size as u64);
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct InteractionClaim {
    pub claimed_sum: SecureField,
}
impl InteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_felts(&[self.claimed_sum]);
    }
}

pub type Component = FrameworkComponent<Eval>;

impl FrameworkEval for Eval {
    fn log_size(&self) -> u32 {
        self.claim.log_size
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_size() + 1
    }

    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let M31_0 = E::F::from(M31::from(0));
        let M31_1 = E::F::from(M31::from(1));
        let M31_2 = E::F::from(M31::from(2));
        let M31_4194304 = E::F::from(M31::from(4194304));
        let M31_512 = E::F::from(M31::from(512));
        let input_limb_0_col0 = eval.next_trace_mask();
        let input_limb_1_col1 = eval.next_trace_mask();
        let input_limb_2_col2 = eval.next_trace_mask();
        let input_limb_3_col3 = eval.next_trace_mask();
        let input_limb_4_col4 = eval.next_trace_mask();
        let input_limb_5_col5 = eval.next_trace_mask();
        let input_limb_6_col6 = eval.next_trace_mask();
        let input_limb_7_col7 = eval.next_trace_mask();
        let input_limb_8_col8 = eval.next_trace_mask();
        let input_limb_9_col9 = eval.next_trace_mask();
        let input_limb_10_col10 = eval.next_trace_mask();
        let input_limb_11_col11 = eval.next_trace_mask();
        let input_limb_12_col12 = eval.next_trace_mask();
        let input_limb_13_col13 = eval.next_trace_mask();
        let input_limb_14_col14 = eval.next_trace_mask();
        let input_limb_15_col15 = eval.next_trace_mask();
        let input_limb_16_col16 = eval.next_trace_mask();
        let input_limb_17_col17 = eval.next_trace_mask();
        let input_limb_18_col18 = eval.next_trace_mask();
        let input_limb_19_col19 = eval.next_trace_mask();
        let input_limb_20_col20 = eval.next_trace_mask();
        let input_limb_21_col21 = eval.next_trace_mask();
        let input_limb_22_col22 = eval.next_trace_mask();
        let input_limb_23_col23 = eval.next_trace_mask();
        let input_limb_24_col24 = eval.next_trace_mask();
        let input_limb_25_col25 = eval.next_trace_mask();
        let input_limb_26_col26 = eval.next_trace_mask();
        let input_limb_27_col27 = eval.next_trace_mask();
        let input_limb_28_col28 = eval.next_trace_mask();
        let input_limb_29_col29 = eval.next_trace_mask();
        let input_limb_30_col30 = eval.next_trace_mask();
        let input_limb_31_col31 = eval.next_trace_mask();
        let input_limb_32_col32 = eval.next_trace_mask();
        let input_limb_33_col33 = eval.next_trace_mask();
        let input_limb_34_col34 = eval.next_trace_mask();
        let input_limb_35_col35 = eval.next_trace_mask();
        let input_limb_36_col36 = eval.next_trace_mask();
        let input_limb_37_col37 = eval.next_trace_mask();
        let input_limb_38_col38 = eval.next_trace_mask();
        let input_limb_39_col39 = eval.next_trace_mask();
        let input_limb_40_col40 = eval.next_trace_mask();
        let input_limb_41_col41 = eval.next_trace_mask();
        let input_limb_42_col42 = eval.next_trace_mask();
        let input_limb_43_col43 = eval.next_trace_mask();
        let input_limb_44_col44 = eval.next_trace_mask();
        let input_limb_45_col45 = eval.next_trace_mask();
        let input_limb_46_col46 = eval.next_trace_mask();
        let input_limb_47_col47 = eval.next_trace_mask();
        let input_limb_48_col48 = eval.next_trace_mask();
        let input_limb_49_col49 = eval.next_trace_mask();
        let input_limb_50_col50 = eval.next_trace_mask();
        let input_limb_51_col51 = eval.next_trace_mask();
        let input_limb_52_col52 = eval.next_trace_mask();
        let input_limb_53_col53 = eval.next_trace_mask();
        let input_limb_54_col54 = eval.next_trace_mask();
        let input_limb_55_col55 = eval.next_trace_mask();
        let input_limb_56_col56 = eval.next_trace_mask();
        let input_limb_57_col57 = eval.next_trace_mask();
        let input_limb_58_col58 = eval.next_trace_mask();
        let input_limb_59_col59 = eval.next_trace_mask();
        let input_limb_60_col60 = eval.next_trace_mask();
        let input_limb_61_col61 = eval.next_trace_mask();
        let input_limb_62_col62 = eval.next_trace_mask();
        let input_limb_63_col63 = eval.next_trace_mask();
        let input_limb_64_col64 = eval.next_trace_mask();
        let input_limb_65_col65 = eval.next_trace_mask();
        let input_limb_66_col66 = eval.next_trace_mask();
        let input_limb_67_col67 = eval.next_trace_mask();
        let input_limb_68_col68 = eval.next_trace_mask();
        let input_limb_69_col69 = eval.next_trace_mask();
        let input_limb_70_col70 = eval.next_trace_mask();
        let input_limb_71_col71 = eval.next_trace_mask();
        let window_low_col72 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_0_col73 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_1_col74 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_2_col75 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_3_col76 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_4_col77 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_5_col78 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_6_col79 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_7_col80 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_8_col81 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_9_col82 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_10_col83 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_11_col84 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_12_col85 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_13_col86 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_14_col87 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_15_col88 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_16_col89 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_17_col90 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_18_col91 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_19_col92 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_20_col93 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_21_col94 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_22_col95 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_23_col96 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_24_col97 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_25_col98 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_26_col99 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_27_col100 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_28_col101 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_29_col102 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_30_col103 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_31_col104 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_32_col105 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_33_col106 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_34_col107 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_35_col108 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_36_col109 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_37_col110 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_38_col111 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_39_col112 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_40_col113 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_41_col114 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_42_col115 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_43_col116 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_44_col117 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_45_col118 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_46_col119 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_47_col120 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_48_col121 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_49_col122 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_50_col123 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_51_col124 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_52_col125 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_53_col126 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_54_col127 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_55_col128 = eval.next_trace_mask();
        let slope_limb_0_col129 = eval.next_trace_mask();
        let slope_limb_1_col130 = eval.next_trace_mask();
        let slope_limb_2_col131 = eval.next_trace_mask();
        let slope_limb_3_col132 = eval.next_trace_mask();
        let slope_limb_4_col133 = eval.next_trace_mask();
        let slope_limb_5_col134 = eval.next_trace_mask();
        let slope_limb_6_col135 = eval.next_trace_mask();
        let slope_limb_7_col136 = eval.next_trace_mask();
        let slope_limb_8_col137 = eval.next_trace_mask();
        let slope_limb_9_col138 = eval.next_trace_mask();
        let slope_limb_10_col139 = eval.next_trace_mask();
        let slope_limb_11_col140 = eval.next_trace_mask();
        let slope_limb_12_col141 = eval.next_trace_mask();
        let slope_limb_13_col142 = eval.next_trace_mask();
        let slope_limb_14_col143 = eval.next_trace_mask();
        let slope_limb_15_col144 = eval.next_trace_mask();
        let slope_limb_16_col145 = eval.next_trace_mask();
        let slope_limb_17_col146 = eval.next_trace_mask();
        let slope_limb_18_col147 = eval.next_trace_mask();
        let slope_limb_19_col148 = eval.next_trace_mask();
        let slope_limb_20_col149 = eval.next_trace_mask();
        let slope_limb_21_col150 = eval.next_trace_mask();
        let slope_limb_22_col151 = eval.next_trace_mask();
        let slope_limb_23_col152 = eval.next_trace_mask();
        let slope_limb_24_col153 = eval.next_trace_mask();
        let slope_limb_25_col154 = eval.next_trace_mask();
        let slope_limb_26_col155 = eval.next_trace_mask();
        let slope_limb_27_col156 = eval.next_trace_mask();
        let k_col157 = eval.next_trace_mask();
        let carry_0_col158 = eval.next_trace_mask();
        let carry_1_col159 = eval.next_trace_mask();
        let carry_2_col160 = eval.next_trace_mask();
        let carry_3_col161 = eval.next_trace_mask();
        let carry_4_col162 = eval.next_trace_mask();
        let carry_5_col163 = eval.next_trace_mask();
        let carry_6_col164 = eval.next_trace_mask();
        let carry_7_col165 = eval.next_trace_mask();
        let carry_8_col166 = eval.next_trace_mask();
        let carry_9_col167 = eval.next_trace_mask();
        let carry_10_col168 = eval.next_trace_mask();
        let carry_11_col169 = eval.next_trace_mask();
        let carry_12_col170 = eval.next_trace_mask();
        let carry_13_col171 = eval.next_trace_mask();
        let carry_14_col172 = eval.next_trace_mask();
        let carry_15_col173 = eval.next_trace_mask();
        let carry_16_col174 = eval.next_trace_mask();
        let carry_17_col175 = eval.next_trace_mask();
        let carry_18_col176 = eval.next_trace_mask();
        let carry_19_col177 = eval.next_trace_mask();
        let carry_20_col178 = eval.next_trace_mask();
        let carry_21_col179 = eval.next_trace_mask();
        let carry_22_col180 = eval.next_trace_mask();
        let carry_23_col181 = eval.next_trace_mask();
        let carry_24_col182 = eval.next_trace_mask();
        let carry_25_col183 = eval.next_trace_mask();
        let carry_26_col184 = eval.next_trace_mask();
        let result_x_limb_0_col185 = eval.next_trace_mask();
        let result_x_limb_1_col186 = eval.next_trace_mask();
        let result_x_limb_2_col187 = eval.next_trace_mask();
        let result_x_limb_3_col188 = eval.next_trace_mask();
        let result_x_limb_4_col189 = eval.next_trace_mask();
        let result_x_limb_5_col190 = eval.next_trace_mask();
        let result_x_limb_6_col191 = eval.next_trace_mask();
        let result_x_limb_7_col192 = eval.next_trace_mask();
        let result_x_limb_8_col193 = eval.next_trace_mask();
        let result_x_limb_9_col194 = eval.next_trace_mask();
        let result_x_limb_10_col195 = eval.next_trace_mask();
        let result_x_limb_11_col196 = eval.next_trace_mask();
        let result_x_limb_12_col197 = eval.next_trace_mask();
        let result_x_limb_13_col198 = eval.next_trace_mask();
        let result_x_limb_14_col199 = eval.next_trace_mask();
        let result_x_limb_15_col200 = eval.next_trace_mask();
        let result_x_limb_16_col201 = eval.next_trace_mask();
        let result_x_limb_17_col202 = eval.next_trace_mask();
        let result_x_limb_18_col203 = eval.next_trace_mask();
        let result_x_limb_19_col204 = eval.next_trace_mask();
        let result_x_limb_20_col205 = eval.next_trace_mask();
        let result_x_limb_21_col206 = eval.next_trace_mask();
        let result_x_limb_22_col207 = eval.next_trace_mask();
        let result_x_limb_23_col208 = eval.next_trace_mask();
        let result_x_limb_24_col209 = eval.next_trace_mask();
        let result_x_limb_25_col210 = eval.next_trace_mask();
        let result_x_limb_26_col211 = eval.next_trace_mask();
        let result_x_limb_27_col212 = eval.next_trace_mask();
        let k_col213 = eval.next_trace_mask();
        let carry_0_col214 = eval.next_trace_mask();
        let carry_1_col215 = eval.next_trace_mask();
        let carry_2_col216 = eval.next_trace_mask();
        let carry_3_col217 = eval.next_trace_mask();
        let carry_4_col218 = eval.next_trace_mask();
        let carry_5_col219 = eval.next_trace_mask();
        let carry_6_col220 = eval.next_trace_mask();
        let carry_7_col221 = eval.next_trace_mask();
        let carry_8_col222 = eval.next_trace_mask();
        let carry_9_col223 = eval.next_trace_mask();
        let carry_10_col224 = eval.next_trace_mask();
        let carry_11_col225 = eval.next_trace_mask();
        let carry_12_col226 = eval.next_trace_mask();
        let carry_13_col227 = eval.next_trace_mask();
        let carry_14_col228 = eval.next_trace_mask();
        let carry_15_col229 = eval.next_trace_mask();
        let carry_16_col230 = eval.next_trace_mask();
        let carry_17_col231 = eval.next_trace_mask();
        let carry_18_col232 = eval.next_trace_mask();
        let carry_19_col233 = eval.next_trace_mask();
        let carry_20_col234 = eval.next_trace_mask();
        let carry_21_col235 = eval.next_trace_mask();
        let carry_22_col236 = eval.next_trace_mask();
        let carry_23_col237 = eval.next_trace_mask();
        let carry_24_col238 = eval.next_trace_mask();
        let carry_25_col239 = eval.next_trace_mask();
        let carry_26_col240 = eval.next_trace_mask();
        let result_y_limb_0_col241 = eval.next_trace_mask();
        let result_y_limb_1_col242 = eval.next_trace_mask();
        let result_y_limb_2_col243 = eval.next_trace_mask();
        let result_y_limb_3_col244 = eval.next_trace_mask();
        let result_y_limb_4_col245 = eval.next_trace_mask();
        let result_y_limb_5_col246 = eval.next_trace_mask();
        let result_y_limb_6_col247 = eval.next_trace_mask();
        let result_y_limb_7_col248 = eval.next_trace_mask();
        let result_y_limb_8_col249 = eval.next_trace_mask();
        let result_y_limb_9_col250 = eval.next_trace_mask();
        let result_y_limb_10_col251 = eval.next_trace_mask();
        let result_y_limb_11_col252 = eval.next_trace_mask();
        let result_y_limb_12_col253 = eval.next_trace_mask();
        let result_y_limb_13_col254 = eval.next_trace_mask();
        let result_y_limb_14_col255 = eval.next_trace_mask();
        let result_y_limb_15_col256 = eval.next_trace_mask();
        let result_y_limb_16_col257 = eval.next_trace_mask();
        let result_y_limb_17_col258 = eval.next_trace_mask();
        let result_y_limb_18_col259 = eval.next_trace_mask();
        let result_y_limb_19_col260 = eval.next_trace_mask();
        let result_y_limb_20_col261 = eval.next_trace_mask();
        let result_y_limb_21_col262 = eval.next_trace_mask();
        let result_y_limb_22_col263 = eval.next_trace_mask();
        let result_y_limb_23_col264 = eval.next_trace_mask();
        let result_y_limb_24_col265 = eval.next_trace_mask();
        let result_y_limb_25_col266 = eval.next_trace_mask();
        let result_y_limb_26_col267 = eval.next_trace_mask();
        let result_y_limb_27_col268 = eval.next_trace_mask();
        let k_col269 = eval.next_trace_mask();
        let carry_0_col270 = eval.next_trace_mask();
        let carry_1_col271 = eval.next_trace_mask();
        let carry_2_col272 = eval.next_trace_mask();
        let carry_3_col273 = eval.next_trace_mask();
        let carry_4_col274 = eval.next_trace_mask();
        let carry_5_col275 = eval.next_trace_mask();
        let carry_6_col276 = eval.next_trace_mask();
        let carry_7_col277 = eval.next_trace_mask();
        let carry_8_col278 = eval.next_trace_mask();
        let carry_9_col279 = eval.next_trace_mask();
        let carry_10_col280 = eval.next_trace_mask();
        let carry_11_col281 = eval.next_trace_mask();
        let carry_12_col282 = eval.next_trace_mask();
        let carry_13_col283 = eval.next_trace_mask();
        let carry_14_col284 = eval.next_trace_mask();
        let carry_15_col285 = eval.next_trace_mask();
        let carry_16_col286 = eval.next_trace_mask();
        let carry_17_col287 = eval.next_trace_mask();
        let carry_18_col288 = eval.next_trace_mask();
        let carry_19_col289 = eval.next_trace_mask();
        let carry_20_col290 = eval.next_trace_mask();
        let carry_21_col291 = eval.next_trace_mask();
        let carry_22_col292 = eval.next_trace_mask();
        let carry_23_col293 = eval.next_trace_mask();
        let carry_24_col294 = eval.next_trace_mask();
        let carry_25_col295 = eval.next_trace_mask();
        let carry_26_col296 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_0_col297 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_1_col298 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_2_col299 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_3_col300 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_4_col301 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_5_col302 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_6_col303 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_7_col304 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_8_col305 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_9_col306 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_10_col307 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_11_col308 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_12_col309 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_13_col310 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_14_col311 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_15_col312 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_16_col313 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_17_col314 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_18_col315 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_19_col316 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_20_col317 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_21_col318 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_22_col319 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_23_col320 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_24_col321 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_25_col322 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_26_col323 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_27_col324 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_28_col325 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_29_col326 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_30_col327 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_31_col328 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_32_col329 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_33_col330 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_34_col331 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_35_col332 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_36_col333 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_37_col334 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_38_col335 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_39_col336 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_40_col337 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_41_col338 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_42_col339 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_43_col340 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_44_col341 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_45_col342 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_46_col343 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_47_col344 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_48_col345 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_49_col346 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_50_col347 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_51_col348 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_52_col349 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_53_col350 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_54_col351 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_55_col352 = eval.next_trace_mask();
        let slope_limb_0_col353 = eval.next_trace_mask();
        let slope_limb_1_col354 = eval.next_trace_mask();
        let slope_limb_2_col355 = eval.next_trace_mask();
        let slope_limb_3_col356 = eval.next_trace_mask();
        let slope_limb_4_col357 = eval.next_trace_mask();
        let slope_limb_5_col358 = eval.next_trace_mask();
        let slope_limb_6_col359 = eval.next_trace_mask();
        let slope_limb_7_col360 = eval.next_trace_mask();
        let slope_limb_8_col361 = eval.next_trace_mask();
        let slope_limb_9_col362 = eval.next_trace_mask();
        let slope_limb_10_col363 = eval.next_trace_mask();
        let slope_limb_11_col364 = eval.next_trace_mask();
        let slope_limb_12_col365 = eval.next_trace_mask();
        let slope_limb_13_col366 = eval.next_trace_mask();
        let slope_limb_14_col367 = eval.next_trace_mask();
        let slope_limb_15_col368 = eval.next_trace_mask();
        let slope_limb_16_col369 = eval.next_trace_mask();
        let slope_limb_17_col370 = eval.next_trace_mask();
        let slope_limb_18_col371 = eval.next_trace_mask();
        let slope_limb_19_col372 = eval.next_trace_mask();
        let slope_limb_20_col373 = eval.next_trace_mask();
        let slope_limb_21_col374 = eval.next_trace_mask();
        let slope_limb_22_col375 = eval.next_trace_mask();
        let slope_limb_23_col376 = eval.next_trace_mask();
        let slope_limb_24_col377 = eval.next_trace_mask();
        let slope_limb_25_col378 = eval.next_trace_mask();
        let slope_limb_26_col379 = eval.next_trace_mask();
        let slope_limb_27_col380 = eval.next_trace_mask();
        let k_col381 = eval.next_trace_mask();
        let carry_0_col382 = eval.next_trace_mask();
        let carry_1_col383 = eval.next_trace_mask();
        let carry_2_col384 = eval.next_trace_mask();
        let carry_3_col385 = eval.next_trace_mask();
        let carry_4_col386 = eval.next_trace_mask();
        let carry_5_col387 = eval.next_trace_mask();
        let carry_6_col388 = eval.next_trace_mask();
        let carry_7_col389 = eval.next_trace_mask();
        let carry_8_col390 = eval.next_trace_mask();
        let carry_9_col391 = eval.next_trace_mask();
        let carry_10_col392 = eval.next_trace_mask();
        let carry_11_col393 = eval.next_trace_mask();
        let carry_12_col394 = eval.next_trace_mask();
        let carry_13_col395 = eval.next_trace_mask();
        let carry_14_col396 = eval.next_trace_mask();
        let carry_15_col397 = eval.next_trace_mask();
        let carry_16_col398 = eval.next_trace_mask();
        let carry_17_col399 = eval.next_trace_mask();
        let carry_18_col400 = eval.next_trace_mask();
        let carry_19_col401 = eval.next_trace_mask();
        let carry_20_col402 = eval.next_trace_mask();
        let carry_21_col403 = eval.next_trace_mask();
        let carry_22_col404 = eval.next_trace_mask();
        let carry_23_col405 = eval.next_trace_mask();
        let carry_24_col406 = eval.next_trace_mask();
        let carry_25_col407 = eval.next_trace_mask();
        let carry_26_col408 = eval.next_trace_mask();
        let result_x_limb_0_col409 = eval.next_trace_mask();
        let result_x_limb_1_col410 = eval.next_trace_mask();
        let result_x_limb_2_col411 = eval.next_trace_mask();
        let result_x_limb_3_col412 = eval.next_trace_mask();
        let result_x_limb_4_col413 = eval.next_trace_mask();
        let result_x_limb_5_col414 = eval.next_trace_mask();
        let result_x_limb_6_col415 = eval.next_trace_mask();
        let result_x_limb_7_col416 = eval.next_trace_mask();
        let result_x_limb_8_col417 = eval.next_trace_mask();
        let result_x_limb_9_col418 = eval.next_trace_mask();
        let result_x_limb_10_col419 = eval.next_trace_mask();
        let result_x_limb_11_col420 = eval.next_trace_mask();
        let result_x_limb_12_col421 = eval.next_trace_mask();
        let result_x_limb_13_col422 = eval.next_trace_mask();
        let result_x_limb_14_col423 = eval.next_trace_mask();
        let result_x_limb_15_col424 = eval.next_trace_mask();
        let result_x_limb_16_col425 = eval.next_trace_mask();
        let result_x_limb_17_col426 = eval.next_trace_mask();
        let result_x_limb_18_col427 = eval.next_trace_mask();
        let result_x_limb_19_col428 = eval.next_trace_mask();
        let result_x_limb_20_col429 = eval.next_trace_mask();
        let result_x_limb_21_col430 = eval.next_trace_mask();
        let result_x_limb_22_col431 = eval.next_trace_mask();
        let result_x_limb_23_col432 = eval.next_trace_mask();
        let result_x_limb_24_col433 = eval.next_trace_mask();
        let result_x_limb_25_col434 = eval.next_trace_mask();
        let result_x_limb_26_col435 = eval.next_trace_mask();
        let result_x_limb_27_col436 = eval.next_trace_mask();
        let k_col437 = eval.next_trace_mask();
        let carry_0_col438 = eval.next_trace_mask();
        let carry_1_col439 = eval.next_trace_mask();
        let carry_2_col440 = eval.next_trace_mask();
        let carry_3_col441 = eval.next_trace_mask();
        let carry_4_col442 = eval.next_trace_mask();
        let carry_5_col443 = eval.next_trace_mask();
        let carry_6_col444 = eval.next_trace_mask();
        let carry_7_col445 = eval.next_trace_mask();
        let carry_8_col446 = eval.next_trace_mask();
        let carry_9_col447 = eval.next_trace_mask();
        let carry_10_col448 = eval.next_trace_mask();
        let carry_11_col449 = eval.next_trace_mask();
        let carry_12_col450 = eval.next_trace_mask();
        let carry_13_col451 = eval.next_trace_mask();
        let carry_14_col452 = eval.next_trace_mask();
        let carry_15_col453 = eval.next_trace_mask();
        let carry_16_col454 = eval.next_trace_mask();
        let carry_17_col455 = eval.next_trace_mask();
        let carry_18_col456 = eval.next_trace_mask();
        let carry_19_col457 = eval.next_trace_mask();
        let carry_20_col458 = eval.next_trace_mask();
        let carry_21_col459 = eval.next_trace_mask();
        let carry_22_col460 = eval.next_trace_mask();
        let carry_23_col461 = eval.next_trace_mask();
        let carry_24_col462 = eval.next_trace_mask();
        let carry_25_col463 = eval.next_trace_mask();
        let carry_26_col464 = eval.next_trace_mask();
        let result_y_limb_0_col465 = eval.next_trace_mask();
        let result_y_limb_1_col466 = eval.next_trace_mask();
        let result_y_limb_2_col467 = eval.next_trace_mask();
        let result_y_limb_3_col468 = eval.next_trace_mask();
        let result_y_limb_4_col469 = eval.next_trace_mask();
        let result_y_limb_5_col470 = eval.next_trace_mask();
        let result_y_limb_6_col471 = eval.next_trace_mask();
        let result_y_limb_7_col472 = eval.next_trace_mask();
        let result_y_limb_8_col473 = eval.next_trace_mask();
        let result_y_limb_9_col474 = eval.next_trace_mask();
        let result_y_limb_10_col475 = eval.next_trace_mask();
        let result_y_limb_11_col476 = eval.next_trace_mask();
        let result_y_limb_12_col477 = eval.next_trace_mask();
        let result_y_limb_13_col478 = eval.next_trace_mask();
        let result_y_limb_14_col479 = eval.next_trace_mask();
        let result_y_limb_15_col480 = eval.next_trace_mask();
        let result_y_limb_16_col481 = eval.next_trace_mask();
        let result_y_limb_17_col482 = eval.next_trace_mask();
        let result_y_limb_18_col483 = eval.next_trace_mask();
        let result_y_limb_19_col484 = eval.next_trace_mask();
        let result_y_limb_20_col485 = eval.next_trace_mask();
        let result_y_limb_21_col486 = eval.next_trace_mask();
        let result_y_limb_22_col487 = eval.next_trace_mask();
        let result_y_limb_23_col488 = eval.next_trace_mask();
        let result_y_limb_24_col489 = eval.next_trace_mask();
        let result_y_limb_25_col490 = eval.next_trace_mask();
        let result_y_limb_26_col491 = eval.next_trace_mask();
        let result_y_limb_27_col492 = eval.next_trace_mask();
        let k_col493 = eval.next_trace_mask();
        let carry_0_col494 = eval.next_trace_mask();
        let carry_1_col495 = eval.next_trace_mask();
        let carry_2_col496 = eval.next_trace_mask();
        let carry_3_col497 = eval.next_trace_mask();
        let carry_4_col498 = eval.next_trace_mask();
        let carry_5_col499 = eval.next_trace_mask();
        let carry_6_col500 = eval.next_trace_mask();
        let carry_7_col501 = eval.next_trace_mask();
        let carry_8_col502 = eval.next_trace_mask();
        let carry_9_col503 = eval.next_trace_mask();
        let carry_10_col504 = eval.next_trace_mask();
        let carry_11_col505 = eval.next_trace_mask();
        let carry_12_col506 = eval.next_trace_mask();
        let carry_13_col507 = eval.next_trace_mask();
        let carry_14_col508 = eval.next_trace_mask();
        let carry_15_col509 = eval.next_trace_mask();
        let carry_16_col510 = eval.next_trace_mask();
        let carry_17_col511 = eval.next_trace_mask();
        let carry_18_col512 = eval.next_trace_mask();
        let carry_19_col513 = eval.next_trace_mask();
        let carry_20_col514 = eval.next_trace_mask();
        let carry_21_col515 = eval.next_trace_mask();
        let carry_22_col516 = eval.next_trace_mask();
        let carry_23_col517 = eval.next_trace_mask();
        let carry_24_col518 = eval.next_trace_mask();
        let carry_25_col519 = eval.next_trace_mask();
        let carry_26_col520 = eval.next_trace_mask();
        let enabler = eval.next_trace_mask();

        eval.add_constraint(enabler.clone() * enabler.clone() - enabler.clone());

        let window_high_tmp_71feb_1 = eval.add_intermediate(
            ((input_limb_2_col2.clone() - window_low_col72.clone()) * M31_4194304.clone()),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[window_low_col72.clone(), window_high_tmp_71feb_1.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.pedersen_points_table_lookup_elements,
            E::EF::one(),
            &[
                ((M31_512.clone() * (input_limb_1_col1.clone() * M31_2.clone()))
                    + window_low_col72.clone()),
                pedersen_points_table_output_limb_0_col73.clone(),
                pedersen_points_table_output_limb_1_col74.clone(),
                pedersen_points_table_output_limb_2_col75.clone(),
                pedersen_points_table_output_limb_3_col76.clone(),
                pedersen_points_table_output_limb_4_col77.clone(),
                pedersen_points_table_output_limb_5_col78.clone(),
                pedersen_points_table_output_limb_6_col79.clone(),
                pedersen_points_table_output_limb_7_col80.clone(),
                pedersen_points_table_output_limb_8_col81.clone(),
                pedersen_points_table_output_limb_9_col82.clone(),
                pedersen_points_table_output_limb_10_col83.clone(),
                pedersen_points_table_output_limb_11_col84.clone(),
                pedersen_points_table_output_limb_12_col85.clone(),
                pedersen_points_table_output_limb_13_col86.clone(),
                pedersen_points_table_output_limb_14_col87.clone(),
                pedersen_points_table_output_limb_15_col88.clone(),
                pedersen_points_table_output_limb_16_col89.clone(),
                pedersen_points_table_output_limb_17_col90.clone(),
                pedersen_points_table_output_limb_18_col91.clone(),
                pedersen_points_table_output_limb_19_col92.clone(),
                pedersen_points_table_output_limb_20_col93.clone(),
                pedersen_points_table_output_limb_21_col94.clone(),
                pedersen_points_table_output_limb_22_col95.clone(),
                pedersen_points_table_output_limb_23_col96.clone(),
                pedersen_points_table_output_limb_24_col97.clone(),
                pedersen_points_table_output_limb_25_col98.clone(),
                pedersen_points_table_output_limb_26_col99.clone(),
                pedersen_points_table_output_limb_27_col100.clone(),
                pedersen_points_table_output_limb_28_col101.clone(),
                pedersen_points_table_output_limb_29_col102.clone(),
                pedersen_points_table_output_limb_30_col103.clone(),
                pedersen_points_table_output_limb_31_col104.clone(),
                pedersen_points_table_output_limb_32_col105.clone(),
                pedersen_points_table_output_limb_33_col106.clone(),
                pedersen_points_table_output_limb_34_col107.clone(),
                pedersen_points_table_output_limb_35_col108.clone(),
                pedersen_points_table_output_limb_36_col109.clone(),
                pedersen_points_table_output_limb_37_col110.clone(),
                pedersen_points_table_output_limb_38_col111.clone(),
                pedersen_points_table_output_limb_39_col112.clone(),
                pedersen_points_table_output_limb_40_col113.clone(),
                pedersen_points_table_output_limb_41_col114.clone(),
                pedersen_points_table_output_limb_42_col115.clone(),
                pedersen_points_table_output_limb_43_col116.clone(),
                pedersen_points_table_output_limb_44_col117.clone(),
                pedersen_points_table_output_limb_45_col118.clone(),
                pedersen_points_table_output_limb_46_col119.clone(),
                pedersen_points_table_output_limb_47_col120.clone(),
                pedersen_points_table_output_limb_48_col121.clone(),
                pedersen_points_table_output_limb_49_col122.clone(),
                pedersen_points_table_output_limb_50_col123.clone(),
                pedersen_points_table_output_limb_51_col124.clone(),
                pedersen_points_table_output_limb_52_col125.clone(),
                pedersen_points_table_output_limb_53_col126.clone(),
                pedersen_points_table_output_limb_54_col127.clone(),
                pedersen_points_table_output_limb_55_col128.clone(),
            ],
        ));

        EcAdd::evaluate(
            [
                input_limb_16_col16.clone(),
                input_limb_17_col17.clone(),
                input_limb_18_col18.clone(),
                input_limb_19_col19.clone(),
                input_limb_20_col20.clone(),
                input_limb_21_col21.clone(),
                input_limb_22_col22.clone(),
                input_limb_23_col23.clone(),
                input_limb_24_col24.clone(),
                input_limb_25_col25.clone(),
                input_limb_26_col26.clone(),
                input_limb_27_col27.clone(),
                input_limb_28_col28.clone(),
                input_limb_29_col29.clone(),
                input_limb_30_col30.clone(),
                input_limb_31_col31.clone(),
                input_limb_32_col32.clone(),
                input_limb_33_col33.clone(),
                input_limb_34_col34.clone(),
                input_limb_35_col35.clone(),
                input_limb_36_col36.clone(),
                input_limb_37_col37.clone(),
                input_limb_38_col38.clone(),
                input_limb_39_col39.clone(),
                input_limb_40_col40.clone(),
                input_limb_41_col41.clone(),
                input_limb_42_col42.clone(),
                input_limb_43_col43.clone(),
                input_limb_44_col44.clone(),
                input_limb_45_col45.clone(),
                input_limb_46_col46.clone(),
                input_limb_47_col47.clone(),
                input_limb_48_col48.clone(),
                input_limb_49_col49.clone(),
                input_limb_50_col50.clone(),
                input_limb_51_col51.clone(),
                input_limb_52_col52.clone(),
                input_limb_53_col53.clone(),
                input_limb_54_col54.clone(),
                input_limb_55_col55.clone(),
                input_limb_56_col56.clone(),
                input_limb_57_col57.clone(),
                input_limb_58_col58.clone(),
                input_limb_59_col59.clone(),
                input_limb_60_col60.clone(),
                input_limb_61_col61.clone(),
                input_limb_62_col62.clone(),
                input_limb_63_col63.clone(),
                input_limb_64_col64.clone(),
                input_limb_65_col65.clone(),
                input_limb_66_col66.clone(),
                input_limb_67_col67.clone(),
                input_limb_68_col68.clone(),
                input_limb_69_col69.clone(),
                input_limb_70_col70.clone(),
                input_limb_71_col71.clone(),
                pedersen_points_table_output_limb_0_col73.clone(),
                pedersen_points_table_output_limb_1_col74.clone(),
                pedersen_points_table_output_limb_2_col75.clone(),
                pedersen_points_table_output_limb_3_col76.clone(),
                pedersen_points_table_output_limb_4_col77.clone(),
                pedersen_points_table_output_limb_5_col78.clone(),
                pedersen_points_table_output_limb_6_col79.clone(),
                pedersen_points_table_output_limb_7_col80.clone(),
                pedersen_points_table_output_limb_8_col81.clone(),
                pedersen_points_table_output_limb_9_col82.clone(),
                pedersen_points_table_output_limb_10_col83.clone(),
                pedersen_points_table_output_limb_11_col84.clone(),
                pedersen_points_table_output_limb_12_col85.clone(),
                pedersen_points_table_output_limb_13_col86.clone(),
                pedersen_points_table_output_limb_14_col87.clone(),
                pedersen_points_table_output_limb_15_col88.clone(),
                pedersen_points_table_output_limb_16_col89.clone(),
                pedersen_points_table_output_limb_17_col90.clone(),
                pedersen_points_table_output_limb_18_col91.clone(),
                pedersen_points_table_output_limb_19_col92.clone(),
                pedersen_points_table_output_limb_20_col93.clone(),
                pedersen_points_table_output_limb_21_col94.clone(),
                pedersen_points_table_output_limb_22_col95.clone(),
                pedersen_points_table_output_limb_23_col96.clone(),
                pedersen_points_table_output_limb_24_col97.clone(),
                pedersen_points_table_output_limb_25_col98.clone(),
                pedersen_points_table_output_limb_26_col99.clone(),
                pedersen_points_table_output_limb_27_col100.clone(),
                pedersen_points_table_output_limb_28_col101.clone(),
                pedersen_points_table_output_limb_29_col102.clone(),
                pedersen_points_table_output_limb_30_col103.clone(),
                pedersen_points_table_output_limb_31_col104.clone(),
                pedersen_points_table_output_limb_32_col105.clone(),
                pedersen_points_table_output_limb_33_col106.clone(),
                pedersen_points_table_output_limb_34_col107.clone(),
                pedersen_points_table_output_limb_35_col108.clone(),
                pedersen_points_table_output_limb_36_col109.clone(),
                pedersen_points_table_output_limb_37_col110.clone(),
                pedersen_points_table_output_limb_38_col111.clone(),
                pedersen_points_table_output_limb_39_col112.clone(),
                pedersen_points_table_output_limb_40_col113.clone(),
                pedersen_points_table_output_limb_41_col114.clone(),
                pedersen_points_table_output_limb_42_col115.clone(),
                pedersen_points_table_output_limb_43_col116.clone(),
                pedersen_points_table_output_limb_44_col117.clone(),
                pedersen_points_table_output_limb_45_col118.clone(),
                pedersen_points_table_output_limb_46_col119.clone(),
                pedersen_points_table_output_limb_47_col120.clone(),
                pedersen_points_table_output_limb_48_col121.clone(),
                pedersen_points_table_output_limb_49_col122.clone(),
                pedersen_points_table_output_limb_50_col123.clone(),
                pedersen_points_table_output_limb_51_col124.clone(),
                pedersen_points_table_output_limb_52_col125.clone(),
                pedersen_points_table_output_limb_53_col126.clone(),
                pedersen_points_table_output_limb_54_col127.clone(),
                pedersen_points_table_output_limb_55_col128.clone(),
            ],
            slope_limb_0_col129.clone(),
            slope_limb_1_col130.clone(),
            slope_limb_2_col131.clone(),
            slope_limb_3_col132.clone(),
            slope_limb_4_col133.clone(),
            slope_limb_5_col134.clone(),
            slope_limb_6_col135.clone(),
            slope_limb_7_col136.clone(),
            slope_limb_8_col137.clone(),
            slope_limb_9_col138.clone(),
            slope_limb_10_col139.clone(),
            slope_limb_11_col140.clone(),
            slope_limb_12_col141.clone(),
            slope_limb_13_col142.clone(),
            slope_limb_14_col143.clone(),
            slope_limb_15_col144.clone(),
            slope_limb_16_col145.clone(),
            slope_limb_17_col146.clone(),
            slope_limb_18_col147.clone(),
            slope_limb_19_col148.clone(),
            slope_limb_20_col149.clone(),
            slope_limb_21_col150.clone(),
            slope_limb_22_col151.clone(),
            slope_limb_23_col152.clone(),
            slope_limb_24_col153.clone(),
            slope_limb_25_col154.clone(),
            slope_limb_26_col155.clone(),
            slope_limb_27_col156.clone(),
            k_col157.clone(),
            carry_0_col158.clone(),
            carry_1_col159.clone(),
            carry_2_col160.clone(),
            carry_3_col161.clone(),
            carry_4_col162.clone(),
            carry_5_col163.clone(),
            carry_6_col164.clone(),
            carry_7_col165.clone(),
            carry_8_col166.clone(),
            carry_9_col167.clone(),
            carry_10_col168.clone(),
            carry_11_col169.clone(),
            carry_12_col170.clone(),
            carry_13_col171.clone(),
            carry_14_col172.clone(),
            carry_15_col173.clone(),
            carry_16_col174.clone(),
            carry_17_col175.clone(),
            carry_18_col176.clone(),
            carry_19_col177.clone(),
            carry_20_col178.clone(),
            carry_21_col179.clone(),
            carry_22_col180.clone(),
            carry_23_col181.clone(),
            carry_24_col182.clone(),
            carry_25_col183.clone(),
            carry_26_col184.clone(),
            result_x_limb_0_col185.clone(),
            result_x_limb_1_col186.clone(),
            result_x_limb_2_col187.clone(),
            result_x_limb_3_col188.clone(),
            result_x_limb_4_col189.clone(),
            result_x_limb_5_col190.clone(),
            result_x_limb_6_col191.clone(),
            result_x_limb_7_col192.clone(),
            result_x_limb_8_col193.clone(),
            result_x_limb_9_col194.clone(),
            result_x_limb_10_col195.clone(),
            result_x_limb_11_col196.clone(),
            result_x_limb_12_col197.clone(),
            result_x_limb_13_col198.clone(),
            result_x_limb_14_col199.clone(),
            result_x_limb_15_col200.clone(),
            result_x_limb_16_col201.clone(),
            result_x_limb_17_col202.clone(),
            result_x_limb_18_col203.clone(),
            result_x_limb_19_col204.clone(),
            result_x_limb_20_col205.clone(),
            result_x_limb_21_col206.clone(),
            result_x_limb_22_col207.clone(),
            result_x_limb_23_col208.clone(),
            result_x_limb_24_col209.clone(),
            result_x_limb_25_col210.clone(),
            result_x_limb_26_col211.clone(),
            result_x_limb_27_col212.clone(),
            k_col213.clone(),
            carry_0_col214.clone(),
            carry_1_col215.clone(),
            carry_2_col216.clone(),
            carry_3_col217.clone(),
            carry_4_col218.clone(),
            carry_5_col219.clone(),
            carry_6_col220.clone(),
            carry_7_col221.clone(),
            carry_8_col222.clone(),
            carry_9_col223.clone(),
            carry_10_col224.clone(),
            carry_11_col225.clone(),
            carry_12_col226.clone(),
            carry_13_col227.clone(),
            carry_14_col228.clone(),
            carry_15_col229.clone(),
            carry_16_col230.clone(),
            carry_17_col231.clone(),
            carry_18_col232.clone(),
            carry_19_col233.clone(),
            carry_20_col234.clone(),
            carry_21_col235.clone(),
            carry_22_col236.clone(),
            carry_23_col237.clone(),
            carry_24_col238.clone(),
            carry_25_col239.clone(),
            carry_26_col240.clone(),
            result_y_limb_0_col241.clone(),
            result_y_limb_1_col242.clone(),
            result_y_limb_2_col243.clone(),
            result_y_limb_3_col244.clone(),
            result_y_limb_4_col245.clone(),
            result_y_limb_5_col246.clone(),
            result_y_limb_6_col247.clone(),
            result_y_limb_7_col248.clone(),
            result_y_limb_8_col249.clone(),
            result_y_limb_9_col250.clone(),
            result_y_limb_10_col251.clone(),
            result_y_limb_11_col252.clone(),
            result_y_limb_12_col253.clone(),
            result_y_limb_13_col254.clone(),
            result_y_limb_14_col255.clone(),
            result_y_limb_15_col256.clone(),
            result_y_limb_16_col257.clone(),
            result_y_limb_17_col258.clone(),
            result_y_limb_18_col259.clone(),
            result_y_limb_19_col260.clone(),
            result_y_limb_20_col261.clone(),
            result_y_limb_21_col262.clone(),
            result_y_limb_22_col263.clone(),
            result_y_limb_23_col264.clone(),
            result_y_limb_24_col265.clone(),
            result_y_limb_25_col266.clone(),
            result_y_limb_26_col267.clone(),
            result_y_limb_27_col268.clone(),
            k_col269.clone(),
            carry_0_col270.clone(),
            carry_1_col271.clone(),
            carry_2_col272.clone(),
            carry_3_col273.clone(),
            carry_4_col274.clone(),
            carry_5_col275.clone(),
            carry_6_col276.clone(),
            carry_7_col277.clone(),
            carry_8_col278.clone(),
            carry_9_col279.clone(),
            carry_10_col280.clone(),
            carry_11_col281.clone(),
            carry_12_col282.clone(),
            carry_13_col283.clone(),
            carry_14_col284.clone(),
            carry_15_col285.clone(),
            carry_16_col286.clone(),
            carry_17_col287.clone(),
            carry_18_col288.clone(),
            carry_19_col289.clone(),
            carry_20_col290.clone(),
            carry_21_col291.clone(),
            carry_22_col292.clone(),
            carry_23_col293.clone(),
            carry_24_col294.clone(),
            carry_25_col295.clone(),
            carry_26_col296.clone(),
            &self.range_check_9_9_lookup_elements,
            &self.range_check_9_9_b_lookup_elements,
            &self.range_check_9_9_c_lookup_elements,
            &self.range_check_9_9_d_lookup_elements,
            &self.range_check_9_9_e_lookup_elements,
            &self.range_check_9_9_f_lookup_elements,
            &self.range_check_9_9_g_lookup_elements,
            &self.range_check_9_9_h_lookup_elements,
            &self.range_check_20_lookup_elements,
            &self.range_check_20_b_lookup_elements,
            &self.range_check_20_c_lookup_elements,
            &self.range_check_20_d_lookup_elements,
            &self.range_check_20_e_lookup_elements,
            &self.range_check_20_f_lookup_elements,
            &self.range_check_20_g_lookup_elements,
            &self.range_check_20_h_lookup_elements,
            &mut eval,
        );
        eval.add_to_relation(RelationEntry::new(
            &self.pedersen_points_table_lookup_elements,
            E::EF::one(),
            &[
                ((M31_512.clone() * ((input_limb_1_col1.clone() * M31_2.clone()) + M31_1.clone()))
                    + window_high_tmp_71feb_1.clone()),
                pedersen_points_table_output_limb_0_col297.clone(),
                pedersen_points_table_output_limb_1_col298.clone(),
                pedersen_points_table_output_limb_2_col299.clone(),
                pedersen_points_table_output_limb_3_col300.clone(),
                pedersen_points_table_output_limb_4_col301.clone(),
                pedersen_points_table_output_limb_5_col302.clone(),
                pedersen_points_table_output_limb_6_col303.clone(),
                pedersen_points_table_output_limb_7_col304.clone(),
                pedersen_points_table_output_limb_8_col305.clone(),
                pedersen_points_table_output_limb_9_col306.clone(),
                pedersen_points_table_output_limb_10_col307.clone(),
                pedersen_points_table_output_limb_11_col308.clone(),
                pedersen_points_table_output_limb_12_col309.clone(),
                pedersen_points_table_output_limb_13_col310.clone(),
                pedersen_points_table_output_limb_14_col311.clone(),
                pedersen_points_table_output_limb_15_col312.clone(),
                pedersen_points_table_output_limb_16_col313.clone(),
                pedersen_points_table_output_limb_17_col314.clone(),
                pedersen_points_table_output_limb_18_col315.clone(),
                pedersen_points_table_output_limb_19_col316.clone(),
                pedersen_points_table_output_limb_20_col317.clone(),
                pedersen_points_table_output_limb_21_col318.clone(),
                pedersen_points_table_output_limb_22_col319.clone(),
                pedersen_points_table_output_limb_23_col320.clone(),
                pedersen_points_table_output_limb_24_col321.clone(),
                pedersen_points_table_output_limb_25_col322.clone(),
                pedersen_points_table_output_limb_26_col323.clone(),
                pedersen_points_table_output_limb_27_col324.clone(),
                pedersen_points_table_output_limb_28_col325.clone(),
                pedersen_points_table_output_limb_29_col326.clone(),
                pedersen_points_table_output_limb_30_col327.clone(),
                pedersen_points_table_output_limb_31_col328.clone(),
                pedersen_points_table_output_limb_32_col329.clone(),
                pedersen_points_table_output_limb_33_col330.clone(),
                pedersen_points_table_output_limb_34_col331.clone(),
                pedersen_points_table_output_limb_35_col332.clone(),
                pedersen_points_table_output_limb_36_col333.clone(),
                pedersen_points_table_output_limb_37_col334.clone(),
                pedersen_points_table_output_limb_38_col335.clone(),
                pedersen_points_table_output_limb_39_col336.clone(),
                pedersen_points_table_output_limb_40_col337.clone(),
                pedersen_points_table_output_limb_41_col338.clone(),
                pedersen_points_table_output_limb_42_col339.clone(),
                pedersen_points_table_output_limb_43_col340.clone(),
                pedersen_points_table_output_limb_44_col341.clone(),
                pedersen_points_table_output_limb_45_col342.clone(),
                pedersen_points_table_output_limb_46_col343.clone(),
                pedersen_points_table_output_limb_47_col344.clone(),
                pedersen_points_table_output_limb_48_col345.clone(),
                pedersen_points_table_output_limb_49_col346.clone(),
                pedersen_points_table_output_limb_50_col347.clone(),
                pedersen_points_table_output_limb_51_col348.clone(),
                pedersen_points_table_output_limb_52_col349.clone(),
                pedersen_points_table_output_limb_53_col350.clone(),
                pedersen_points_table_output_limb_54_col351.clone(),
                pedersen_points_table_output_limb_55_col352.clone(),
            ],
        ));

        EcAdd::evaluate(
            [
                result_x_limb_0_col185.clone(),
                result_x_limb_1_col186.clone(),
                result_x_limb_2_col187.clone(),
                result_x_limb_3_col188.clone(),
                result_x_limb_4_col189.clone(),
                result_x_limb_5_col190.clone(),
                result_x_limb_6_col191.clone(),
                result_x_limb_7_col192.clone(),
                result_x_limb_8_col193.clone(),
                result_x_limb_9_col194.clone(),
                result_x_limb_10_col195.clone(),
                result_x_limb_11_col196.clone(),
                result_x_limb_12_col197.clone(),
                result_x_limb_13_col198.clone(),
                result_x_limb_14_col199.clone(),
                result_x_limb_15_col200.clone(),
                result_x_limb_16_col201.clone(),
                result_x_limb_17_col202.clone(),
                result_x_limb_18_col203.clone(),
                result_x_limb_19_col204.clone(),
                result_x_limb_20_col205.clone(),
                result_x_limb_21_col206.clone(),
                result_x_limb_22_col207.clone(),
                result_x_limb_23_col208.clone(),
                result_x_limb_24_col209.clone(),
                result_x_limb_25_col210.clone(),
                result_x_limb_26_col211.clone(),
                result_x_limb_27_col212.clone(),
                result_y_limb_0_col241.clone(),
                result_y_limb_1_col242.clone(),
                result_y_limb_2_col243.clone(),
                result_y_limb_3_col244.clone(),
                result_y_limb_4_col245.clone(),
                result_y_limb_5_col246.clone(),
                result_y_limb_6_col247.clone(),
                result_y_limb_7_col248.clone(),
                result_y_limb_8_col249.clone(),
                result_y_limb_9_col250.clone(),
                result_y_limb_10_col251.clone(),
                result_y_limb_11_col252.clone(),
                result_y_limb_12_col253.clone(),
                result_y_limb_13_col254.clone(),
                result_y_limb_14_col255.clone(),
                result_y_limb_15_col256.clone(),
                result_y_limb_16_col257.clone(),
                result_y_limb_17_col258.clone(),
                result_y_limb_18_col259.clone(),
                result_y_limb_19_col260.clone(),
                result_y_limb_20_col261.clone(),
                result_y_limb_21_col262.clone(),
                result_y_limb_22_col263.clone(),
                result_y_limb_23_col264.clone(),
                result_y_limb_24_col265.clone(),
                result_y_limb_25_col266.clone(),
                result_y_limb_26_col267.clone(),
                result_y_limb_27_col268.clone(),
                pedersen_points_table_output_limb_0_col297.clone(),
                pedersen_points_table_output_limb_1_col298.clone(),
                pedersen_points_table_output_limb_2_col299.clone(),
                pedersen_points_table_output_limb_3_col300.clone(),
                pedersen_points_table_output_limb_4_col301.clone(),
                pedersen_points_table_output_limb_5_col302.clone(),
                pedersen_points_table_output_limb_6_col303.clone(),
                pedersen_points_table_output_limb_7_col304.clone(),
                pedersen_points_table_output_limb_8_col305.clone(),
                pedersen_points_table_output_limb_9_col306.clone(),
                pedersen_points_table_output_limb_10_col307.clone(),
                pedersen_points_table_output_limb_11_col308.clone(),
                pedersen_points_table_output_limb_12_col309.clone(),
                pedersen_points_table_output_limb_13_col310.clone(),
                pedersen_points_table_output_limb_14_col311.clone(),
                pedersen_points_table_output_limb_15_col312.clone(),
                pedersen_points_table_output_limb_16_col313.clone(),
                pedersen_points_table_output_limb_17_col314.clone(),
                pedersen_points_table_output_limb_18_col315.clone(),
                pedersen_points_table_output_limb_19_col316.clone(),
                pedersen_points_table_output_limb_20_col317.clone(),
                pedersen_points_table_output_limb_21_col318.clone(),
                pedersen_points_table_output_limb_22_col319.clone(),
                pedersen_points_table_output_limb_23_col320.clone(),
                pedersen_points_table_output_limb_24_col321.clone(),
                pedersen_points_table_output_limb_25_col322.clone(),
                pedersen_points_table_output_limb_26_col323.clone(),
                pedersen_points_table_output_limb_27_col324.clone(),
                pedersen_points_table_output_limb_28_col325.clone(),
                pedersen_points_table_output_limb_29_col326.clone(),
                pedersen_points_table_output_limb_30_col327.clone(),
                pedersen_points_table_output_limb_31_col328.clone(),
                pedersen_points_table_output_limb_32_col329.clone(),
                pedersen_points_table_output_limb_33_col330.clone(),
                pedersen_points_table_output_limb_34_col331.clone(),
                pedersen_points_table_output_limb_35_col332.clone(),
                pedersen_points_table_output_limb_36_col333.clone(),
                pedersen_points_table_output_limb_37_col334.clone(),
                pedersen_points_table_output_limb_38_col335.clone(),
                pedersen_points_table_output_limb_39_col336.clone(),
                pedersen_points_table_output_limb_40_col337.clone(),
                pedersen_points_table_output_limb_41_col338.clone(),
                pedersen_points_table_output_limb_42_col339.clone(),
                pedersen_points_table_output_limb_43_col340.clone(),
                pedersen_points_table_output_limb_44_col341.clone(),
                pedersen_points_table_output_limb_45_col342.clone(),
                pedersen_points_table_output_limb_46_col343.clone(),
                pedersen_points_table_output_limb_47_col344.clone(),
                pedersen_points_table_output_limb_48_col345.clone(),
                pedersen_points_table_output_limb_49_col346.clone(),
                pedersen_points_table_output_limb_50_col347.clone(),
                pedersen_points_table_output_limb_51_col348.clone(),
                pedersen_points_table_output_limb_52_col349.clone(),
                pedersen_points_table_output_limb_53_col350.clone(),
                pedersen_points_table_output_limb_54_col351.clone(),
                pedersen_points_table_output_limb_55_col352.clone(),
            ],
            slope_limb_0_col353.clone(),
            slope_limb_1_col354.clone(),
            slope_limb_2_col355.clone(),
            slope_limb_3_col356.clone(),
            slope_limb_4_col357.clone(),
            slope_limb_5_col358.clone(),
            slope_limb_6_col359.clone(),
            slope_limb_7_col360.clone(),
            slope_limb_8_col361.clone(),
            slope_limb_9_col362.clone(),
            slope_limb_10_col363.clone(),
            slope_limb_11_col364.clone(),
            slope_limb_12_col365.clone(),
            slope_limb_13_col366.clone(),
            slope_limb_14_col367.clone(),
            slope_limb_15_col368.clone(),
            slope_limb_16_col369.clone(),
            slope_limb_17_col370.clone(),
            slope_limb_18_col371.clone(),
            slope_limb_19_col372.clone(),
            slope_limb_20_col373.clone(),
            slope_limb_21_col374.clone(),
            slope_limb_22_col375.clone(),
            slope_limb_23_col376.clone(),
            slope_limb_24_col377.clone(),
            slope_limb_25_col378.clone(),
            slope_limb_26_col379.clone(),
            slope_limb_27_col380.clone(),
            k_col381.clone(),
            carry_0_col382.clone(),
            carry_1_col383.clone(),
            carry_2_col384.clone(),
            carry_3_col385.clone(),
            carry_4_col386.clone(),
            carry_5_col387.clone(),
            carry_6_col388.clone(),
            carry_7_col389.clone(),
            carry_8_col390.clone(),
            carry_9_col391.clone(),
            carry_10_col392.clone(),
            carry_11_col393.clone(),
            carry_12_col394.clone(),
            carry_13_col395.clone(),
            carry_14_col396.clone(),
            carry_15_col397.clone(),
            carry_16_col398.clone(),
            carry_17_col399.clone(),
            carry_18_col400.clone(),
            carry_19_col401.clone(),
            carry_20_col402.clone(),
            carry_21_col403.clone(),
            carry_22_col404.clone(),
            carry_23_col405.clone(),
            carry_24_col406.clone(),
            carry_25_col407.clone(),
            carry_26_col408.clone(),
            result_x_limb_0_col409.clone(),
            result_x_limb_1_col410.clone(),
            result_x_limb_2_col411.clone(),
            result_x_limb_3_col412.clone(),
            result_x_limb_4_col413.clone(),
            result_x_limb_5_col414.clone(),
            result_x_limb_6_col415.clone(),
            result_x_limb_7_col416.clone(),
            result_x_limb_8_col417.clone(),
            result_x_limb_9_col418.clone(),
            result_x_limb_10_col419.clone(),
            result_x_limb_11_col420.clone(),
            result_x_limb_12_col421.clone(),
            result_x_limb_13_col422.clone(),
            result_x_limb_14_col423.clone(),
            result_x_limb_15_col424.clone(),
            result_x_limb_16_col425.clone(),
            result_x_limb_17_col426.clone(),
            result_x_limb_18_col427.clone(),
            result_x_limb_19_col428.clone(),
            result_x_limb_20_col429.clone(),
            result_x_limb_21_col430.clone(),
            result_x_limb_22_col431.clone(),
            result_x_limb_23_col432.clone(),
            result_x_limb_24_col433.clone(),
            result_x_limb_25_col434.clone(),
            result_x_limb_26_col435.clone(),
            result_x_limb_27_col436.clone(),
            k_col437.clone(),
            carry_0_col438.clone(),
            carry_1_col439.clone(),
            carry_2_col440.clone(),
            carry_3_col441.clone(),
            carry_4_col442.clone(),
            carry_5_col443.clone(),
            carry_6_col444.clone(),
            carry_7_col445.clone(),
            carry_8_col446.clone(),
            carry_9_col447.clone(),
            carry_10_col448.clone(),
            carry_11_col449.clone(),
            carry_12_col450.clone(),
            carry_13_col451.clone(),
            carry_14_col452.clone(),
            carry_15_col453.clone(),
            carry_16_col454.clone(),
            carry_17_col455.clone(),
            carry_18_col456.clone(),
            carry_19_col457.clone(),
            carry_20_col458.clone(),
            carry_21_col459.clone(),
            carry_22_col460.clone(),
            carry_23_col461.clone(),
            carry_24_col462.clone(),
            carry_25_col463.clone(),
            carry_26_col464.clone(),
            result_y_limb_0_col465.clone(),
            result_y_limb_1_col466.clone(),
            result_y_limb_2_col467.clone(),
            result_y_limb_3_col468.clone(),
            result_y_limb_4_col469.clone(),
            result_y_limb_5_col470.clone(),
            result_y_limb_6_col471.clone(),
            result_y_limb_7_col472.clone(),
            result_y_limb_8_col473.clone(),
            result_y_limb_9_col474.clone(),
            result_y_limb_10_col475.clone(),
            result_y_limb_11_col476.clone(),
            result_y_limb_12_col477.clone(),
            result_y_limb_13_col478.clone(),
            result_y_limb_14_col479.clone(),
            result_y_limb_15_col480.clone(),
            result_y_limb_16_col481.clone(),
            result_y_limb_17_col482.clone(),
            result_y_limb_18_col483.clone(),
            result_y_limb_19_col484.clone(),
            result_y_limb_20_col485.clone(),
            result_y_limb_21_col486.clone(),
            result_y_limb_22_col487.clone(),
            result_y_limb_23_col488.clone(),
            result_y_limb_24_col489.clone(),
            result_y_limb_25_col490.clone(),
            result_y_limb_26_col491.clone(),
            result_y_limb_27_col492.clone(),
            k_col493.clone(),
            carry_0_col494.clone(),
            carry_1_col495.clone(),
            carry_2_col496.clone(),
            carry_3_col497.clone(),
            carry_4_col498.clone(),
            carry_5_col499.clone(),
            carry_6_col500.clone(),
            carry_7_col501.clone(),
            carry_8_col502.clone(),
            carry_9_col503.clone(),
            carry_10_col504.clone(),
            carry_11_col505.clone(),
            carry_12_col506.clone(),
            carry_13_col507.clone(),
            carry_14_col508.clone(),
            carry_15_col509.clone(),
            carry_16_col510.clone(),
            carry_17_col511.clone(),
            carry_18_col512.clone(),
            carry_19_col513.clone(),
            carry_20_col514.clone(),
            carry_21_col515.clone(),
            carry_22_col516.clone(),
            carry_23_col517.clone(),
            carry_24_col518.clone(),
            carry_25_col519.clone(),
            carry_26_col520.clone(),
            &self.range_check_9_9_lookup_elements,
            &self.range_check_9_9_b_lookup_elements,
            &self.range_check_9_9_c_lookup_elements,
            &self.range_check_9_9_d_lookup_elements,
            &self.range_check_9_9_e_lookup_elements,
            &self.range_check_9_9_f_lookup_elements,
            &self.range_check_9_9_g_lookup_elements,
            &self.range_check_9_9_h_lookup_elements,
            &self.range_check_20_lookup_elements,
            &self.range_check_20_b_lookup_elements,
            &self.range_check_20_c_lookup_elements,
            &self.range_check_20_d_lookup_elements,
            &self.range_check_20_e_lookup_elements,
            &self.range_check_20_f_lookup_elements,
            &self.range_check_20_g_lookup_elements,
            &self.range_check_20_h_lookup_elements,
            &mut eval,
        );
        eval.add_to_relation(RelationEntry::new(
            &self.partial_ec_mul_lookup_elements,
            E::EF::from(enabler.clone()),
            &[
                input_limb_0_col0.clone(),
                input_limb_1_col1.clone(),
                input_limb_2_col2.clone(),
                input_limb_3_col3.clone(),
                input_limb_4_col4.clone(),
                input_limb_5_col5.clone(),
                input_limb_6_col6.clone(),
                input_limb_7_col7.clone(),
                input_limb_8_col8.clone(),
                input_limb_9_col9.clone(),
                input_limb_10_col10.clone(),
                input_limb_11_col11.clone(),
                input_limb_12_col12.clone(),
                input_limb_13_col13.clone(),
                input_limb_14_col14.clone(),
                input_limb_15_col15.clone(),
                input_limb_16_col16.clone(),
                input_limb_17_col17.clone(),
                input_limb_18_col18.clone(),
                input_limb_19_col19.clone(),
                input_limb_20_col20.clone(),
                input_limb_21_col21.clone(),
                input_limb_22_col22.clone(),
                input_limb_23_col23.clone(),
                input_limb_24_col24.clone(),
                input_limb_25_col25.clone(),
                input_limb_26_col26.clone(),
                input_limb_27_col27.clone(),
                input_limb_28_col28.clone(),
                input_limb_29_col29.clone(),
                input_limb_30_col30.clone(),
                input_limb_31_col31.clone(),
                input_limb_32_col32.clone(),
                input_limb_33_col33.clone(),
                input_limb_34_col34.clone(),
                input_limb_35_col35.clone(),
                input_limb_36_col36.clone(),
                input_limb_37_col37.clone(),
                input_limb_38_col38.clone(),
                input_limb_39_col39.clone(),
                input_limb_40_col40.clone(),
                input_limb_41_col41.clone(),
                input_limb_42_col42.clone(),
                input_limb_43_col43.clone(),
                input_limb_44_col44.clone(),
                input_limb_45_col45.clone(),
                input_limb_46_col46.clone(),
                input_limb_47_col47.clone(),
                input_limb_48_col48.clone(),
                input_limb_49_col49.clone(),
                input_limb_50_col50.clone(),
                input_limb_51_col51.clone(),
                input_limb_52_col52.clone(),
                input_limb_53_col53.clone(),
                input_limb_54_col54.clone(),
                input_limb_55_col55.clone(),
                input_limb_56_col56.clone(),
                input_limb_57_col57.clone(),
                input_limb_58_col58.clone(),
                input_limb_59_col59.clone(),
                input_limb_60_col60.clone(),
                input_limb_61_col61.clone(),
                input_limb_62_col62.clone(),
                input_limb_63_col63.clone(),
                input_limb_64_col64.clone(),
                input_limb_65_col65.clone(),
                input_limb_66_col66.clone(),
                input_limb_67_col67.clone(),
                input_limb_68_col68.clone(),
                input_limb_69_col69.clone(),
                input_limb_70_col70.clone(),
                input_limb_71_col71.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.partial_ec_mul_lookup_elements,
            -E::EF::from(enabler.clone()),
            &[
                input_limb_0_col0.clone(),
                (input_limb_1_col1.clone() + M31_1.clone()),
                input_limb_3_col3.clone(),
                input_limb_4_col4.clone(),
                input_limb_5_col5.clone(),
                input_limb_6_col6.clone(),
                input_limb_7_col7.clone(),
                input_limb_8_col8.clone(),
                input_limb_9_col9.clone(),
                input_limb_10_col10.clone(),
                input_limb_11_col11.clone(),
                input_limb_12_col12.clone(),
                input_limb_13_col13.clone(),
                input_limb_14_col14.clone(),
                input_limb_15_col15.clone(),
                M31_0.clone(),
                result_x_limb_0_col409.clone(),
                result_x_limb_1_col410.clone(),
                result_x_limb_2_col411.clone(),
                result_x_limb_3_col412.clone(),
                result_x_limb_4_col413.clone(),
                result_x_limb_5_col414.clone(),
                result_x_limb_6_col415.clone(),
                result_x_limb_7_col416.clone(),
                result_x_limb_8_col417.clone(),
                result_x_limb_9_col418.clone(),
                result_x_limb_10_col419.clone(),
                result_x_limb_11_col420.clone(),
                result_x_limb_12_col421.clone(),
                result_x_limb_13_col422.clone(),
                result_x_limb_14_col423.clone(),
                result_x_limb_15_col424.clone(),
                result_x_limb_16_col425.clone(),
                result_x_limb_17_col426.clone(),
                result_x_limb_18_col427.clone(),
                result_x_limb_19_col428.clone(),
                result_x_limb_20_col429.clone(),
                result_x_limb_21_col430.clone(),
                result_x_limb_22_col431.clone(),
                result_x_limb_23_col432.clone(),
                result_x_limb_24_col433.clone(),
                result_x_limb_25_col434.clone(),
                result_x_limb_26_col435.clone(),
                result_x_limb_27_col436.clone(),
                result_y_limb_0_col465.clone(),
                result_y_limb_1_col466.clone(),
                result_y_limb_2_col467.clone(),
                result_y_limb_3_col468.clone(),
                result_y_limb_4_col469.clone(),
                result_y_limb_5_col470.clone(),
                result_y_limb_6_col471.clone(),
                result_y_limb_7_col472.clone(),
                result_y_limb_8_col473.clone(),
                result_y_limb_9_col474.clone(),
                result_y_limb_10_col475.clone(),
                result_y_limb_11_col476.clone(),
                result_y_limb_12_col477.clone(),
                result_y_limb_13_col478.clone(),
                result_y_limb_14_col479.clone(),
                result_y_limb_15_col480.clone(),
                result_y_limb_16_col481.clone(),
                result_y_limb_17_col482.clone(),
                result_y_limb_18_col483.clone(),
                result_y_limb_19_col484.clone(),
                result_y_limb_20_col485.clone(),
                result_y_limb_21_col486.clone(),
                result_y_limb_22_col487.clone(),
                result_y_limb_23_col488.clone(),
                result_y_limb_24_col489.clone(),
                result_y_limb_25_col490.clone(),
                result_y_limb_26_col491.clone(),
                result_y_limb_27_col492.clone(),
            ],
        ));

        eval.finalize_logup_in_pairs();
        eval
    }
}

#[cfg(test)]
mod tests {
    use num_traits::Zero;
    use rand::rngs::SmallRng;
    use rand::{Rng, SeedableRng};
    use stwo::core::fields::qm31::QM31;
    use stwo_constraint_framework::expr::ExprEvaluator;

    use super::*;
    use crate::components::constraints_regression_test_values::PARTIAL_EC_MUL;

    #[test]
    fn partial_ec_mul_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim { log_size: 4 },
            range_check_9_9_lookup_elements: relations::RangeCheck_9_9::dummy(),
            pedersen_points_table_lookup_elements: relations::PedersenPointsTable::dummy(),
            range_check_9_9_b_lookup_elements: relations::RangeCheck_9_9_B::dummy(),
            range_check_9_9_c_lookup_elements: relations::RangeCheck_9_9_C::dummy(),
            range_check_9_9_d_lookup_elements: relations::RangeCheck_9_9_D::dummy(),
            range_check_9_9_e_lookup_elements: relations::RangeCheck_9_9_E::dummy(),
            range_check_9_9_f_lookup_elements: relations::RangeCheck_9_9_F::dummy(),
            range_check_9_9_g_lookup_elements: relations::RangeCheck_9_9_G::dummy(),
            range_check_9_9_h_lookup_elements: relations::RangeCheck_9_9_H::dummy(),
            range_check_20_lookup_elements: relations::RangeCheck_20::dummy(),
            range_check_20_b_lookup_elements: relations::RangeCheck_20_B::dummy(),
            range_check_20_c_lookup_elements: relations::RangeCheck_20_C::dummy(),
            range_check_20_d_lookup_elements: relations::RangeCheck_20_D::dummy(),
            range_check_20_e_lookup_elements: relations::RangeCheck_20_E::dummy(),
            range_check_20_f_lookup_elements: relations::RangeCheck_20_F::dummy(),
            range_check_20_g_lookup_elements: relations::RangeCheck_20_G::dummy(),
            range_check_20_h_lookup_elements: relations::RangeCheck_20_H::dummy(),
            partial_ec_mul_lookup_elements: relations::PartialEcMul::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        assert_eq!(sum, PARTIAL_EC_MUL);
    }
}
