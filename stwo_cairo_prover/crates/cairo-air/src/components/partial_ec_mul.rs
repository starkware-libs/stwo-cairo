// AIR version 2f6e7d38
use crate::components::prelude::*;
use crate::components::subroutines::ec_add::EcAdd;

pub const N_TRACE_COLUMNS: usize = 471;
pub const RELATION_USES_PER_ROW: [RelationUse; 18] = [
    RelationUse {
        relation_id: "PartialEcMul",
        uses: 1,
    },
    RelationUse {
        relation_id: "PedersenPointsTable",
        uses: 1,
    },
    RelationUse {
        relation_id: "RangeCheck_19",
        uses: 12,
    },
    RelationUse {
        relation_id: "RangeCheck_19_B",
        uses: 12,
    },
    RelationUse {
        relation_id: "RangeCheck_19_C",
        uses: 12,
    },
    RelationUse {
        relation_id: "RangeCheck_19_D",
        uses: 9,
    },
    RelationUse {
        relation_id: "RangeCheck_19_E",
        uses: 9,
    },
    RelationUse {
        relation_id: "RangeCheck_19_F",
        uses: 9,
    },
    RelationUse {
        relation_id: "RangeCheck_19_G",
        uses: 9,
    },
    RelationUse {
        relation_id: "RangeCheck_19_H",
        uses: 12,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9",
        uses: 18,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_B",
        uses: 18,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_C",
        uses: 18,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_D",
        uses: 18,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_E",
        uses: 18,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_F",
        uses: 18,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_G",
        uses: 9,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_H",
        uses: 9,
    },
];

pub struct Eval {
    pub claim: Claim,
    pub pedersen_points_table_lookup_elements: relations::PedersenPointsTable,
    pub range_check_9_9_lookup_elements: relations::RangeCheck_9_9,
    pub range_check_9_9_b_lookup_elements: relations::RangeCheck_9_9_B,
    pub range_check_9_9_c_lookup_elements: relations::RangeCheck_9_9_C,
    pub range_check_9_9_d_lookup_elements: relations::RangeCheck_9_9_D,
    pub range_check_9_9_e_lookup_elements: relations::RangeCheck_9_9_E,
    pub range_check_9_9_f_lookup_elements: relations::RangeCheck_9_9_F,
    pub range_check_9_9_g_lookup_elements: relations::RangeCheck_9_9_G,
    pub range_check_9_9_h_lookup_elements: relations::RangeCheck_9_9_H,
    pub range_check_19_h_lookup_elements: relations::RangeCheck_19_H,
    pub range_check_19_lookup_elements: relations::RangeCheck_19,
    pub range_check_19_b_lookup_elements: relations::RangeCheck_19_B,
    pub range_check_19_c_lookup_elements: relations::RangeCheck_19_C,
    pub range_check_19_d_lookup_elements: relations::RangeCheck_19_D,
    pub range_check_19_e_lookup_elements: relations::RangeCheck_19_E,
    pub range_check_19_f_lookup_elements: relations::RangeCheck_19_F,
    pub range_check_19_g_lookup_elements: relations::RangeCheck_19_G,
    pub partial_ec_mul_lookup_elements: relations::PartialEcMul,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct Claim {
    pub log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 107];
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
        let M31_262144 = E::F::from(M31::from(262144));
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
        let pedersen_points_table_output_limb_0_col72 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_1_col73 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_2_col74 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_3_col75 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_4_col76 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_5_col77 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_6_col78 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_7_col79 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_8_col80 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_9_col81 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_10_col82 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_11_col83 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_12_col84 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_13_col85 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_14_col86 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_15_col87 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_16_col88 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_17_col89 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_18_col90 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_19_col91 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_20_col92 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_21_col93 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_22_col94 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_23_col95 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_24_col96 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_25_col97 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_26_col98 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_27_col99 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_28_col100 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_29_col101 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_30_col102 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_31_col103 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_32_col104 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_33_col105 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_34_col106 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_35_col107 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_36_col108 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_37_col109 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_38_col110 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_39_col111 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_40_col112 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_41_col113 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_42_col114 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_43_col115 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_44_col116 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_45_col117 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_46_col118 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_47_col119 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_48_col120 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_49_col121 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_50_col122 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_51_col123 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_52_col124 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_53_col125 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_54_col126 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_55_col127 = eval.next_trace_mask();
        let sub_res_limb_0_col128 = eval.next_trace_mask();
        let sub_res_limb_1_col129 = eval.next_trace_mask();
        let sub_res_limb_2_col130 = eval.next_trace_mask();
        let sub_res_limb_3_col131 = eval.next_trace_mask();
        let sub_res_limb_4_col132 = eval.next_trace_mask();
        let sub_res_limb_5_col133 = eval.next_trace_mask();
        let sub_res_limb_6_col134 = eval.next_trace_mask();
        let sub_res_limb_7_col135 = eval.next_trace_mask();
        let sub_res_limb_8_col136 = eval.next_trace_mask();
        let sub_res_limb_9_col137 = eval.next_trace_mask();
        let sub_res_limb_10_col138 = eval.next_trace_mask();
        let sub_res_limb_11_col139 = eval.next_trace_mask();
        let sub_res_limb_12_col140 = eval.next_trace_mask();
        let sub_res_limb_13_col141 = eval.next_trace_mask();
        let sub_res_limb_14_col142 = eval.next_trace_mask();
        let sub_res_limb_15_col143 = eval.next_trace_mask();
        let sub_res_limb_16_col144 = eval.next_trace_mask();
        let sub_res_limb_17_col145 = eval.next_trace_mask();
        let sub_res_limb_18_col146 = eval.next_trace_mask();
        let sub_res_limb_19_col147 = eval.next_trace_mask();
        let sub_res_limb_20_col148 = eval.next_trace_mask();
        let sub_res_limb_21_col149 = eval.next_trace_mask();
        let sub_res_limb_22_col150 = eval.next_trace_mask();
        let sub_res_limb_23_col151 = eval.next_trace_mask();
        let sub_res_limb_24_col152 = eval.next_trace_mask();
        let sub_res_limb_25_col153 = eval.next_trace_mask();
        let sub_res_limb_26_col154 = eval.next_trace_mask();
        let sub_res_limb_27_col155 = eval.next_trace_mask();
        let sub_p_bit_col156 = eval.next_trace_mask();
        let add_res_limb_0_col157 = eval.next_trace_mask();
        let add_res_limb_1_col158 = eval.next_trace_mask();
        let add_res_limb_2_col159 = eval.next_trace_mask();
        let add_res_limb_3_col160 = eval.next_trace_mask();
        let add_res_limb_4_col161 = eval.next_trace_mask();
        let add_res_limb_5_col162 = eval.next_trace_mask();
        let add_res_limb_6_col163 = eval.next_trace_mask();
        let add_res_limb_7_col164 = eval.next_trace_mask();
        let add_res_limb_8_col165 = eval.next_trace_mask();
        let add_res_limb_9_col166 = eval.next_trace_mask();
        let add_res_limb_10_col167 = eval.next_trace_mask();
        let add_res_limb_11_col168 = eval.next_trace_mask();
        let add_res_limb_12_col169 = eval.next_trace_mask();
        let add_res_limb_13_col170 = eval.next_trace_mask();
        let add_res_limb_14_col171 = eval.next_trace_mask();
        let add_res_limb_15_col172 = eval.next_trace_mask();
        let add_res_limb_16_col173 = eval.next_trace_mask();
        let add_res_limb_17_col174 = eval.next_trace_mask();
        let add_res_limb_18_col175 = eval.next_trace_mask();
        let add_res_limb_19_col176 = eval.next_trace_mask();
        let add_res_limb_20_col177 = eval.next_trace_mask();
        let add_res_limb_21_col178 = eval.next_trace_mask();
        let add_res_limb_22_col179 = eval.next_trace_mask();
        let add_res_limb_23_col180 = eval.next_trace_mask();
        let add_res_limb_24_col181 = eval.next_trace_mask();
        let add_res_limb_25_col182 = eval.next_trace_mask();
        let add_res_limb_26_col183 = eval.next_trace_mask();
        let add_res_limb_27_col184 = eval.next_trace_mask();
        let sub_p_bit_col185 = eval.next_trace_mask();
        let sub_res_limb_0_col186 = eval.next_trace_mask();
        let sub_res_limb_1_col187 = eval.next_trace_mask();
        let sub_res_limb_2_col188 = eval.next_trace_mask();
        let sub_res_limb_3_col189 = eval.next_trace_mask();
        let sub_res_limb_4_col190 = eval.next_trace_mask();
        let sub_res_limb_5_col191 = eval.next_trace_mask();
        let sub_res_limb_6_col192 = eval.next_trace_mask();
        let sub_res_limb_7_col193 = eval.next_trace_mask();
        let sub_res_limb_8_col194 = eval.next_trace_mask();
        let sub_res_limb_9_col195 = eval.next_trace_mask();
        let sub_res_limb_10_col196 = eval.next_trace_mask();
        let sub_res_limb_11_col197 = eval.next_trace_mask();
        let sub_res_limb_12_col198 = eval.next_trace_mask();
        let sub_res_limb_13_col199 = eval.next_trace_mask();
        let sub_res_limb_14_col200 = eval.next_trace_mask();
        let sub_res_limb_15_col201 = eval.next_trace_mask();
        let sub_res_limb_16_col202 = eval.next_trace_mask();
        let sub_res_limb_17_col203 = eval.next_trace_mask();
        let sub_res_limb_18_col204 = eval.next_trace_mask();
        let sub_res_limb_19_col205 = eval.next_trace_mask();
        let sub_res_limb_20_col206 = eval.next_trace_mask();
        let sub_res_limb_21_col207 = eval.next_trace_mask();
        let sub_res_limb_22_col208 = eval.next_trace_mask();
        let sub_res_limb_23_col209 = eval.next_trace_mask();
        let sub_res_limb_24_col210 = eval.next_trace_mask();
        let sub_res_limb_25_col211 = eval.next_trace_mask();
        let sub_res_limb_26_col212 = eval.next_trace_mask();
        let sub_res_limb_27_col213 = eval.next_trace_mask();
        let sub_p_bit_col214 = eval.next_trace_mask();
        let div_res_limb_0_col215 = eval.next_trace_mask();
        let div_res_limb_1_col216 = eval.next_trace_mask();
        let div_res_limb_2_col217 = eval.next_trace_mask();
        let div_res_limb_3_col218 = eval.next_trace_mask();
        let div_res_limb_4_col219 = eval.next_trace_mask();
        let div_res_limb_5_col220 = eval.next_trace_mask();
        let div_res_limb_6_col221 = eval.next_trace_mask();
        let div_res_limb_7_col222 = eval.next_trace_mask();
        let div_res_limb_8_col223 = eval.next_trace_mask();
        let div_res_limb_9_col224 = eval.next_trace_mask();
        let div_res_limb_10_col225 = eval.next_trace_mask();
        let div_res_limb_11_col226 = eval.next_trace_mask();
        let div_res_limb_12_col227 = eval.next_trace_mask();
        let div_res_limb_13_col228 = eval.next_trace_mask();
        let div_res_limb_14_col229 = eval.next_trace_mask();
        let div_res_limb_15_col230 = eval.next_trace_mask();
        let div_res_limb_16_col231 = eval.next_trace_mask();
        let div_res_limb_17_col232 = eval.next_trace_mask();
        let div_res_limb_18_col233 = eval.next_trace_mask();
        let div_res_limb_19_col234 = eval.next_trace_mask();
        let div_res_limb_20_col235 = eval.next_trace_mask();
        let div_res_limb_21_col236 = eval.next_trace_mask();
        let div_res_limb_22_col237 = eval.next_trace_mask();
        let div_res_limb_23_col238 = eval.next_trace_mask();
        let div_res_limb_24_col239 = eval.next_trace_mask();
        let div_res_limb_25_col240 = eval.next_trace_mask();
        let div_res_limb_26_col241 = eval.next_trace_mask();
        let div_res_limb_27_col242 = eval.next_trace_mask();
        let k_col243 = eval.next_trace_mask();
        let carry_0_col244 = eval.next_trace_mask();
        let carry_1_col245 = eval.next_trace_mask();
        let carry_2_col246 = eval.next_trace_mask();
        let carry_3_col247 = eval.next_trace_mask();
        let carry_4_col248 = eval.next_trace_mask();
        let carry_5_col249 = eval.next_trace_mask();
        let carry_6_col250 = eval.next_trace_mask();
        let carry_7_col251 = eval.next_trace_mask();
        let carry_8_col252 = eval.next_trace_mask();
        let carry_9_col253 = eval.next_trace_mask();
        let carry_10_col254 = eval.next_trace_mask();
        let carry_11_col255 = eval.next_trace_mask();
        let carry_12_col256 = eval.next_trace_mask();
        let carry_13_col257 = eval.next_trace_mask();
        let carry_14_col258 = eval.next_trace_mask();
        let carry_15_col259 = eval.next_trace_mask();
        let carry_16_col260 = eval.next_trace_mask();
        let carry_17_col261 = eval.next_trace_mask();
        let carry_18_col262 = eval.next_trace_mask();
        let carry_19_col263 = eval.next_trace_mask();
        let carry_20_col264 = eval.next_trace_mask();
        let carry_21_col265 = eval.next_trace_mask();
        let carry_22_col266 = eval.next_trace_mask();
        let carry_23_col267 = eval.next_trace_mask();
        let carry_24_col268 = eval.next_trace_mask();
        let carry_25_col269 = eval.next_trace_mask();
        let carry_26_col270 = eval.next_trace_mask();
        let mul_res_limb_0_col271 = eval.next_trace_mask();
        let mul_res_limb_1_col272 = eval.next_trace_mask();
        let mul_res_limb_2_col273 = eval.next_trace_mask();
        let mul_res_limb_3_col274 = eval.next_trace_mask();
        let mul_res_limb_4_col275 = eval.next_trace_mask();
        let mul_res_limb_5_col276 = eval.next_trace_mask();
        let mul_res_limb_6_col277 = eval.next_trace_mask();
        let mul_res_limb_7_col278 = eval.next_trace_mask();
        let mul_res_limb_8_col279 = eval.next_trace_mask();
        let mul_res_limb_9_col280 = eval.next_trace_mask();
        let mul_res_limb_10_col281 = eval.next_trace_mask();
        let mul_res_limb_11_col282 = eval.next_trace_mask();
        let mul_res_limb_12_col283 = eval.next_trace_mask();
        let mul_res_limb_13_col284 = eval.next_trace_mask();
        let mul_res_limb_14_col285 = eval.next_trace_mask();
        let mul_res_limb_15_col286 = eval.next_trace_mask();
        let mul_res_limb_16_col287 = eval.next_trace_mask();
        let mul_res_limb_17_col288 = eval.next_trace_mask();
        let mul_res_limb_18_col289 = eval.next_trace_mask();
        let mul_res_limb_19_col290 = eval.next_trace_mask();
        let mul_res_limb_20_col291 = eval.next_trace_mask();
        let mul_res_limb_21_col292 = eval.next_trace_mask();
        let mul_res_limb_22_col293 = eval.next_trace_mask();
        let mul_res_limb_23_col294 = eval.next_trace_mask();
        let mul_res_limb_24_col295 = eval.next_trace_mask();
        let mul_res_limb_25_col296 = eval.next_trace_mask();
        let mul_res_limb_26_col297 = eval.next_trace_mask();
        let mul_res_limb_27_col298 = eval.next_trace_mask();
        let k_col299 = eval.next_trace_mask();
        let carry_0_col300 = eval.next_trace_mask();
        let carry_1_col301 = eval.next_trace_mask();
        let carry_2_col302 = eval.next_trace_mask();
        let carry_3_col303 = eval.next_trace_mask();
        let carry_4_col304 = eval.next_trace_mask();
        let carry_5_col305 = eval.next_trace_mask();
        let carry_6_col306 = eval.next_trace_mask();
        let carry_7_col307 = eval.next_trace_mask();
        let carry_8_col308 = eval.next_trace_mask();
        let carry_9_col309 = eval.next_trace_mask();
        let carry_10_col310 = eval.next_trace_mask();
        let carry_11_col311 = eval.next_trace_mask();
        let carry_12_col312 = eval.next_trace_mask();
        let carry_13_col313 = eval.next_trace_mask();
        let carry_14_col314 = eval.next_trace_mask();
        let carry_15_col315 = eval.next_trace_mask();
        let carry_16_col316 = eval.next_trace_mask();
        let carry_17_col317 = eval.next_trace_mask();
        let carry_18_col318 = eval.next_trace_mask();
        let carry_19_col319 = eval.next_trace_mask();
        let carry_20_col320 = eval.next_trace_mask();
        let carry_21_col321 = eval.next_trace_mask();
        let carry_22_col322 = eval.next_trace_mask();
        let carry_23_col323 = eval.next_trace_mask();
        let carry_24_col324 = eval.next_trace_mask();
        let carry_25_col325 = eval.next_trace_mask();
        let carry_26_col326 = eval.next_trace_mask();
        let sub_res_limb_0_col327 = eval.next_trace_mask();
        let sub_res_limb_1_col328 = eval.next_trace_mask();
        let sub_res_limb_2_col329 = eval.next_trace_mask();
        let sub_res_limb_3_col330 = eval.next_trace_mask();
        let sub_res_limb_4_col331 = eval.next_trace_mask();
        let sub_res_limb_5_col332 = eval.next_trace_mask();
        let sub_res_limb_6_col333 = eval.next_trace_mask();
        let sub_res_limb_7_col334 = eval.next_trace_mask();
        let sub_res_limb_8_col335 = eval.next_trace_mask();
        let sub_res_limb_9_col336 = eval.next_trace_mask();
        let sub_res_limb_10_col337 = eval.next_trace_mask();
        let sub_res_limb_11_col338 = eval.next_trace_mask();
        let sub_res_limb_12_col339 = eval.next_trace_mask();
        let sub_res_limb_13_col340 = eval.next_trace_mask();
        let sub_res_limb_14_col341 = eval.next_trace_mask();
        let sub_res_limb_15_col342 = eval.next_trace_mask();
        let sub_res_limb_16_col343 = eval.next_trace_mask();
        let sub_res_limb_17_col344 = eval.next_trace_mask();
        let sub_res_limb_18_col345 = eval.next_trace_mask();
        let sub_res_limb_19_col346 = eval.next_trace_mask();
        let sub_res_limb_20_col347 = eval.next_trace_mask();
        let sub_res_limb_21_col348 = eval.next_trace_mask();
        let sub_res_limb_22_col349 = eval.next_trace_mask();
        let sub_res_limb_23_col350 = eval.next_trace_mask();
        let sub_res_limb_24_col351 = eval.next_trace_mask();
        let sub_res_limb_25_col352 = eval.next_trace_mask();
        let sub_res_limb_26_col353 = eval.next_trace_mask();
        let sub_res_limb_27_col354 = eval.next_trace_mask();
        let sub_p_bit_col355 = eval.next_trace_mask();
        let sub_res_limb_0_col356 = eval.next_trace_mask();
        let sub_res_limb_1_col357 = eval.next_trace_mask();
        let sub_res_limb_2_col358 = eval.next_trace_mask();
        let sub_res_limb_3_col359 = eval.next_trace_mask();
        let sub_res_limb_4_col360 = eval.next_trace_mask();
        let sub_res_limb_5_col361 = eval.next_trace_mask();
        let sub_res_limb_6_col362 = eval.next_trace_mask();
        let sub_res_limb_7_col363 = eval.next_trace_mask();
        let sub_res_limb_8_col364 = eval.next_trace_mask();
        let sub_res_limb_9_col365 = eval.next_trace_mask();
        let sub_res_limb_10_col366 = eval.next_trace_mask();
        let sub_res_limb_11_col367 = eval.next_trace_mask();
        let sub_res_limb_12_col368 = eval.next_trace_mask();
        let sub_res_limb_13_col369 = eval.next_trace_mask();
        let sub_res_limb_14_col370 = eval.next_trace_mask();
        let sub_res_limb_15_col371 = eval.next_trace_mask();
        let sub_res_limb_16_col372 = eval.next_trace_mask();
        let sub_res_limb_17_col373 = eval.next_trace_mask();
        let sub_res_limb_18_col374 = eval.next_trace_mask();
        let sub_res_limb_19_col375 = eval.next_trace_mask();
        let sub_res_limb_20_col376 = eval.next_trace_mask();
        let sub_res_limb_21_col377 = eval.next_trace_mask();
        let sub_res_limb_22_col378 = eval.next_trace_mask();
        let sub_res_limb_23_col379 = eval.next_trace_mask();
        let sub_res_limb_24_col380 = eval.next_trace_mask();
        let sub_res_limb_25_col381 = eval.next_trace_mask();
        let sub_res_limb_26_col382 = eval.next_trace_mask();
        let sub_res_limb_27_col383 = eval.next_trace_mask();
        let sub_p_bit_col384 = eval.next_trace_mask();
        let mul_res_limb_0_col385 = eval.next_trace_mask();
        let mul_res_limb_1_col386 = eval.next_trace_mask();
        let mul_res_limb_2_col387 = eval.next_trace_mask();
        let mul_res_limb_3_col388 = eval.next_trace_mask();
        let mul_res_limb_4_col389 = eval.next_trace_mask();
        let mul_res_limb_5_col390 = eval.next_trace_mask();
        let mul_res_limb_6_col391 = eval.next_trace_mask();
        let mul_res_limb_7_col392 = eval.next_trace_mask();
        let mul_res_limb_8_col393 = eval.next_trace_mask();
        let mul_res_limb_9_col394 = eval.next_trace_mask();
        let mul_res_limb_10_col395 = eval.next_trace_mask();
        let mul_res_limb_11_col396 = eval.next_trace_mask();
        let mul_res_limb_12_col397 = eval.next_trace_mask();
        let mul_res_limb_13_col398 = eval.next_trace_mask();
        let mul_res_limb_14_col399 = eval.next_trace_mask();
        let mul_res_limb_15_col400 = eval.next_trace_mask();
        let mul_res_limb_16_col401 = eval.next_trace_mask();
        let mul_res_limb_17_col402 = eval.next_trace_mask();
        let mul_res_limb_18_col403 = eval.next_trace_mask();
        let mul_res_limb_19_col404 = eval.next_trace_mask();
        let mul_res_limb_20_col405 = eval.next_trace_mask();
        let mul_res_limb_21_col406 = eval.next_trace_mask();
        let mul_res_limb_22_col407 = eval.next_trace_mask();
        let mul_res_limb_23_col408 = eval.next_trace_mask();
        let mul_res_limb_24_col409 = eval.next_trace_mask();
        let mul_res_limb_25_col410 = eval.next_trace_mask();
        let mul_res_limb_26_col411 = eval.next_trace_mask();
        let mul_res_limb_27_col412 = eval.next_trace_mask();
        let k_col413 = eval.next_trace_mask();
        let carry_0_col414 = eval.next_trace_mask();
        let carry_1_col415 = eval.next_trace_mask();
        let carry_2_col416 = eval.next_trace_mask();
        let carry_3_col417 = eval.next_trace_mask();
        let carry_4_col418 = eval.next_trace_mask();
        let carry_5_col419 = eval.next_trace_mask();
        let carry_6_col420 = eval.next_trace_mask();
        let carry_7_col421 = eval.next_trace_mask();
        let carry_8_col422 = eval.next_trace_mask();
        let carry_9_col423 = eval.next_trace_mask();
        let carry_10_col424 = eval.next_trace_mask();
        let carry_11_col425 = eval.next_trace_mask();
        let carry_12_col426 = eval.next_trace_mask();
        let carry_13_col427 = eval.next_trace_mask();
        let carry_14_col428 = eval.next_trace_mask();
        let carry_15_col429 = eval.next_trace_mask();
        let carry_16_col430 = eval.next_trace_mask();
        let carry_17_col431 = eval.next_trace_mask();
        let carry_18_col432 = eval.next_trace_mask();
        let carry_19_col433 = eval.next_trace_mask();
        let carry_20_col434 = eval.next_trace_mask();
        let carry_21_col435 = eval.next_trace_mask();
        let carry_22_col436 = eval.next_trace_mask();
        let carry_23_col437 = eval.next_trace_mask();
        let carry_24_col438 = eval.next_trace_mask();
        let carry_25_col439 = eval.next_trace_mask();
        let carry_26_col440 = eval.next_trace_mask();
        let sub_res_limb_0_col441 = eval.next_trace_mask();
        let sub_res_limb_1_col442 = eval.next_trace_mask();
        let sub_res_limb_2_col443 = eval.next_trace_mask();
        let sub_res_limb_3_col444 = eval.next_trace_mask();
        let sub_res_limb_4_col445 = eval.next_trace_mask();
        let sub_res_limb_5_col446 = eval.next_trace_mask();
        let sub_res_limb_6_col447 = eval.next_trace_mask();
        let sub_res_limb_7_col448 = eval.next_trace_mask();
        let sub_res_limb_8_col449 = eval.next_trace_mask();
        let sub_res_limb_9_col450 = eval.next_trace_mask();
        let sub_res_limb_10_col451 = eval.next_trace_mask();
        let sub_res_limb_11_col452 = eval.next_trace_mask();
        let sub_res_limb_12_col453 = eval.next_trace_mask();
        let sub_res_limb_13_col454 = eval.next_trace_mask();
        let sub_res_limb_14_col455 = eval.next_trace_mask();
        let sub_res_limb_15_col456 = eval.next_trace_mask();
        let sub_res_limb_16_col457 = eval.next_trace_mask();
        let sub_res_limb_17_col458 = eval.next_trace_mask();
        let sub_res_limb_18_col459 = eval.next_trace_mask();
        let sub_res_limb_19_col460 = eval.next_trace_mask();
        let sub_res_limb_20_col461 = eval.next_trace_mask();
        let sub_res_limb_21_col462 = eval.next_trace_mask();
        let sub_res_limb_22_col463 = eval.next_trace_mask();
        let sub_res_limb_23_col464 = eval.next_trace_mask();
        let sub_res_limb_24_col465 = eval.next_trace_mask();
        let sub_res_limb_25_col466 = eval.next_trace_mask();
        let sub_res_limb_26_col467 = eval.next_trace_mask();
        let sub_res_limb_27_col468 = eval.next_trace_mask();
        let sub_p_bit_col469 = eval.next_trace_mask();
        let enabler = eval.next_trace_mask();

        eval.add_constraint(enabler.clone() * enabler.clone() - enabler.clone());

        eval.add_to_relation(RelationEntry::new(
            &self.pedersen_points_table_lookup_elements,
            E::EF::one(),
            &[
                ((M31_262144.clone() * input_limb_1_col1.clone()) + input_limb_2_col2.clone()),
                pedersen_points_table_output_limb_0_col72.clone(),
                pedersen_points_table_output_limb_1_col73.clone(),
                pedersen_points_table_output_limb_2_col74.clone(),
                pedersen_points_table_output_limb_3_col75.clone(),
                pedersen_points_table_output_limb_4_col76.clone(),
                pedersen_points_table_output_limb_5_col77.clone(),
                pedersen_points_table_output_limb_6_col78.clone(),
                pedersen_points_table_output_limb_7_col79.clone(),
                pedersen_points_table_output_limb_8_col80.clone(),
                pedersen_points_table_output_limb_9_col81.clone(),
                pedersen_points_table_output_limb_10_col82.clone(),
                pedersen_points_table_output_limb_11_col83.clone(),
                pedersen_points_table_output_limb_12_col84.clone(),
                pedersen_points_table_output_limb_13_col85.clone(),
                pedersen_points_table_output_limb_14_col86.clone(),
                pedersen_points_table_output_limb_15_col87.clone(),
                pedersen_points_table_output_limb_16_col88.clone(),
                pedersen_points_table_output_limb_17_col89.clone(),
                pedersen_points_table_output_limb_18_col90.clone(),
                pedersen_points_table_output_limb_19_col91.clone(),
                pedersen_points_table_output_limb_20_col92.clone(),
                pedersen_points_table_output_limb_21_col93.clone(),
                pedersen_points_table_output_limb_22_col94.clone(),
                pedersen_points_table_output_limb_23_col95.clone(),
                pedersen_points_table_output_limb_24_col96.clone(),
                pedersen_points_table_output_limb_25_col97.clone(),
                pedersen_points_table_output_limb_26_col98.clone(),
                pedersen_points_table_output_limb_27_col99.clone(),
                pedersen_points_table_output_limb_28_col100.clone(),
                pedersen_points_table_output_limb_29_col101.clone(),
                pedersen_points_table_output_limb_30_col102.clone(),
                pedersen_points_table_output_limb_31_col103.clone(),
                pedersen_points_table_output_limb_32_col104.clone(),
                pedersen_points_table_output_limb_33_col105.clone(),
                pedersen_points_table_output_limb_34_col106.clone(),
                pedersen_points_table_output_limb_35_col107.clone(),
                pedersen_points_table_output_limb_36_col108.clone(),
                pedersen_points_table_output_limb_37_col109.clone(),
                pedersen_points_table_output_limb_38_col110.clone(),
                pedersen_points_table_output_limb_39_col111.clone(),
                pedersen_points_table_output_limb_40_col112.clone(),
                pedersen_points_table_output_limb_41_col113.clone(),
                pedersen_points_table_output_limb_42_col114.clone(),
                pedersen_points_table_output_limb_43_col115.clone(),
                pedersen_points_table_output_limb_44_col116.clone(),
                pedersen_points_table_output_limb_45_col117.clone(),
                pedersen_points_table_output_limb_46_col118.clone(),
                pedersen_points_table_output_limb_47_col119.clone(),
                pedersen_points_table_output_limb_48_col120.clone(),
                pedersen_points_table_output_limb_49_col121.clone(),
                pedersen_points_table_output_limb_50_col122.clone(),
                pedersen_points_table_output_limb_51_col123.clone(),
                pedersen_points_table_output_limb_52_col124.clone(),
                pedersen_points_table_output_limb_53_col125.clone(),
                pedersen_points_table_output_limb_54_col126.clone(),
                pedersen_points_table_output_limb_55_col127.clone(),
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
                pedersen_points_table_output_limb_0_col72.clone(),
                pedersen_points_table_output_limb_1_col73.clone(),
                pedersen_points_table_output_limb_2_col74.clone(),
                pedersen_points_table_output_limb_3_col75.clone(),
                pedersen_points_table_output_limb_4_col76.clone(),
                pedersen_points_table_output_limb_5_col77.clone(),
                pedersen_points_table_output_limb_6_col78.clone(),
                pedersen_points_table_output_limb_7_col79.clone(),
                pedersen_points_table_output_limb_8_col80.clone(),
                pedersen_points_table_output_limb_9_col81.clone(),
                pedersen_points_table_output_limb_10_col82.clone(),
                pedersen_points_table_output_limb_11_col83.clone(),
                pedersen_points_table_output_limb_12_col84.clone(),
                pedersen_points_table_output_limb_13_col85.clone(),
                pedersen_points_table_output_limb_14_col86.clone(),
                pedersen_points_table_output_limb_15_col87.clone(),
                pedersen_points_table_output_limb_16_col88.clone(),
                pedersen_points_table_output_limb_17_col89.clone(),
                pedersen_points_table_output_limb_18_col90.clone(),
                pedersen_points_table_output_limb_19_col91.clone(),
                pedersen_points_table_output_limb_20_col92.clone(),
                pedersen_points_table_output_limb_21_col93.clone(),
                pedersen_points_table_output_limb_22_col94.clone(),
                pedersen_points_table_output_limb_23_col95.clone(),
                pedersen_points_table_output_limb_24_col96.clone(),
                pedersen_points_table_output_limb_25_col97.clone(),
                pedersen_points_table_output_limb_26_col98.clone(),
                pedersen_points_table_output_limb_27_col99.clone(),
                pedersen_points_table_output_limb_28_col100.clone(),
                pedersen_points_table_output_limb_29_col101.clone(),
                pedersen_points_table_output_limb_30_col102.clone(),
                pedersen_points_table_output_limb_31_col103.clone(),
                pedersen_points_table_output_limb_32_col104.clone(),
                pedersen_points_table_output_limb_33_col105.clone(),
                pedersen_points_table_output_limb_34_col106.clone(),
                pedersen_points_table_output_limb_35_col107.clone(),
                pedersen_points_table_output_limb_36_col108.clone(),
                pedersen_points_table_output_limb_37_col109.clone(),
                pedersen_points_table_output_limb_38_col110.clone(),
                pedersen_points_table_output_limb_39_col111.clone(),
                pedersen_points_table_output_limb_40_col112.clone(),
                pedersen_points_table_output_limb_41_col113.clone(),
                pedersen_points_table_output_limb_42_col114.clone(),
                pedersen_points_table_output_limb_43_col115.clone(),
                pedersen_points_table_output_limb_44_col116.clone(),
                pedersen_points_table_output_limb_45_col117.clone(),
                pedersen_points_table_output_limb_46_col118.clone(),
                pedersen_points_table_output_limb_47_col119.clone(),
                pedersen_points_table_output_limb_48_col120.clone(),
                pedersen_points_table_output_limb_49_col121.clone(),
                pedersen_points_table_output_limb_50_col122.clone(),
                pedersen_points_table_output_limb_51_col123.clone(),
                pedersen_points_table_output_limb_52_col124.clone(),
                pedersen_points_table_output_limb_53_col125.clone(),
                pedersen_points_table_output_limb_54_col126.clone(),
                pedersen_points_table_output_limb_55_col127.clone(),
            ],
            sub_res_limb_0_col128.clone(),
            sub_res_limb_1_col129.clone(),
            sub_res_limb_2_col130.clone(),
            sub_res_limb_3_col131.clone(),
            sub_res_limb_4_col132.clone(),
            sub_res_limb_5_col133.clone(),
            sub_res_limb_6_col134.clone(),
            sub_res_limb_7_col135.clone(),
            sub_res_limb_8_col136.clone(),
            sub_res_limb_9_col137.clone(),
            sub_res_limb_10_col138.clone(),
            sub_res_limb_11_col139.clone(),
            sub_res_limb_12_col140.clone(),
            sub_res_limb_13_col141.clone(),
            sub_res_limb_14_col142.clone(),
            sub_res_limb_15_col143.clone(),
            sub_res_limb_16_col144.clone(),
            sub_res_limb_17_col145.clone(),
            sub_res_limb_18_col146.clone(),
            sub_res_limb_19_col147.clone(),
            sub_res_limb_20_col148.clone(),
            sub_res_limb_21_col149.clone(),
            sub_res_limb_22_col150.clone(),
            sub_res_limb_23_col151.clone(),
            sub_res_limb_24_col152.clone(),
            sub_res_limb_25_col153.clone(),
            sub_res_limb_26_col154.clone(),
            sub_res_limb_27_col155.clone(),
            sub_p_bit_col156.clone(),
            add_res_limb_0_col157.clone(),
            add_res_limb_1_col158.clone(),
            add_res_limb_2_col159.clone(),
            add_res_limb_3_col160.clone(),
            add_res_limb_4_col161.clone(),
            add_res_limb_5_col162.clone(),
            add_res_limb_6_col163.clone(),
            add_res_limb_7_col164.clone(),
            add_res_limb_8_col165.clone(),
            add_res_limb_9_col166.clone(),
            add_res_limb_10_col167.clone(),
            add_res_limb_11_col168.clone(),
            add_res_limb_12_col169.clone(),
            add_res_limb_13_col170.clone(),
            add_res_limb_14_col171.clone(),
            add_res_limb_15_col172.clone(),
            add_res_limb_16_col173.clone(),
            add_res_limb_17_col174.clone(),
            add_res_limb_18_col175.clone(),
            add_res_limb_19_col176.clone(),
            add_res_limb_20_col177.clone(),
            add_res_limb_21_col178.clone(),
            add_res_limb_22_col179.clone(),
            add_res_limb_23_col180.clone(),
            add_res_limb_24_col181.clone(),
            add_res_limb_25_col182.clone(),
            add_res_limb_26_col183.clone(),
            add_res_limb_27_col184.clone(),
            sub_p_bit_col185.clone(),
            sub_res_limb_0_col186.clone(),
            sub_res_limb_1_col187.clone(),
            sub_res_limb_2_col188.clone(),
            sub_res_limb_3_col189.clone(),
            sub_res_limb_4_col190.clone(),
            sub_res_limb_5_col191.clone(),
            sub_res_limb_6_col192.clone(),
            sub_res_limb_7_col193.clone(),
            sub_res_limb_8_col194.clone(),
            sub_res_limb_9_col195.clone(),
            sub_res_limb_10_col196.clone(),
            sub_res_limb_11_col197.clone(),
            sub_res_limb_12_col198.clone(),
            sub_res_limb_13_col199.clone(),
            sub_res_limb_14_col200.clone(),
            sub_res_limb_15_col201.clone(),
            sub_res_limb_16_col202.clone(),
            sub_res_limb_17_col203.clone(),
            sub_res_limb_18_col204.clone(),
            sub_res_limb_19_col205.clone(),
            sub_res_limb_20_col206.clone(),
            sub_res_limb_21_col207.clone(),
            sub_res_limb_22_col208.clone(),
            sub_res_limb_23_col209.clone(),
            sub_res_limb_24_col210.clone(),
            sub_res_limb_25_col211.clone(),
            sub_res_limb_26_col212.clone(),
            sub_res_limb_27_col213.clone(),
            sub_p_bit_col214.clone(),
            div_res_limb_0_col215.clone(),
            div_res_limb_1_col216.clone(),
            div_res_limb_2_col217.clone(),
            div_res_limb_3_col218.clone(),
            div_res_limb_4_col219.clone(),
            div_res_limb_5_col220.clone(),
            div_res_limb_6_col221.clone(),
            div_res_limb_7_col222.clone(),
            div_res_limb_8_col223.clone(),
            div_res_limb_9_col224.clone(),
            div_res_limb_10_col225.clone(),
            div_res_limb_11_col226.clone(),
            div_res_limb_12_col227.clone(),
            div_res_limb_13_col228.clone(),
            div_res_limb_14_col229.clone(),
            div_res_limb_15_col230.clone(),
            div_res_limb_16_col231.clone(),
            div_res_limb_17_col232.clone(),
            div_res_limb_18_col233.clone(),
            div_res_limb_19_col234.clone(),
            div_res_limb_20_col235.clone(),
            div_res_limb_21_col236.clone(),
            div_res_limb_22_col237.clone(),
            div_res_limb_23_col238.clone(),
            div_res_limb_24_col239.clone(),
            div_res_limb_25_col240.clone(),
            div_res_limb_26_col241.clone(),
            div_res_limb_27_col242.clone(),
            k_col243.clone(),
            carry_0_col244.clone(),
            carry_1_col245.clone(),
            carry_2_col246.clone(),
            carry_3_col247.clone(),
            carry_4_col248.clone(),
            carry_5_col249.clone(),
            carry_6_col250.clone(),
            carry_7_col251.clone(),
            carry_8_col252.clone(),
            carry_9_col253.clone(),
            carry_10_col254.clone(),
            carry_11_col255.clone(),
            carry_12_col256.clone(),
            carry_13_col257.clone(),
            carry_14_col258.clone(),
            carry_15_col259.clone(),
            carry_16_col260.clone(),
            carry_17_col261.clone(),
            carry_18_col262.clone(),
            carry_19_col263.clone(),
            carry_20_col264.clone(),
            carry_21_col265.clone(),
            carry_22_col266.clone(),
            carry_23_col267.clone(),
            carry_24_col268.clone(),
            carry_25_col269.clone(),
            carry_26_col270.clone(),
            mul_res_limb_0_col271.clone(),
            mul_res_limb_1_col272.clone(),
            mul_res_limb_2_col273.clone(),
            mul_res_limb_3_col274.clone(),
            mul_res_limb_4_col275.clone(),
            mul_res_limb_5_col276.clone(),
            mul_res_limb_6_col277.clone(),
            mul_res_limb_7_col278.clone(),
            mul_res_limb_8_col279.clone(),
            mul_res_limb_9_col280.clone(),
            mul_res_limb_10_col281.clone(),
            mul_res_limb_11_col282.clone(),
            mul_res_limb_12_col283.clone(),
            mul_res_limb_13_col284.clone(),
            mul_res_limb_14_col285.clone(),
            mul_res_limb_15_col286.clone(),
            mul_res_limb_16_col287.clone(),
            mul_res_limb_17_col288.clone(),
            mul_res_limb_18_col289.clone(),
            mul_res_limb_19_col290.clone(),
            mul_res_limb_20_col291.clone(),
            mul_res_limb_21_col292.clone(),
            mul_res_limb_22_col293.clone(),
            mul_res_limb_23_col294.clone(),
            mul_res_limb_24_col295.clone(),
            mul_res_limb_25_col296.clone(),
            mul_res_limb_26_col297.clone(),
            mul_res_limb_27_col298.clone(),
            k_col299.clone(),
            carry_0_col300.clone(),
            carry_1_col301.clone(),
            carry_2_col302.clone(),
            carry_3_col303.clone(),
            carry_4_col304.clone(),
            carry_5_col305.clone(),
            carry_6_col306.clone(),
            carry_7_col307.clone(),
            carry_8_col308.clone(),
            carry_9_col309.clone(),
            carry_10_col310.clone(),
            carry_11_col311.clone(),
            carry_12_col312.clone(),
            carry_13_col313.clone(),
            carry_14_col314.clone(),
            carry_15_col315.clone(),
            carry_16_col316.clone(),
            carry_17_col317.clone(),
            carry_18_col318.clone(),
            carry_19_col319.clone(),
            carry_20_col320.clone(),
            carry_21_col321.clone(),
            carry_22_col322.clone(),
            carry_23_col323.clone(),
            carry_24_col324.clone(),
            carry_25_col325.clone(),
            carry_26_col326.clone(),
            sub_res_limb_0_col327.clone(),
            sub_res_limb_1_col328.clone(),
            sub_res_limb_2_col329.clone(),
            sub_res_limb_3_col330.clone(),
            sub_res_limb_4_col331.clone(),
            sub_res_limb_5_col332.clone(),
            sub_res_limb_6_col333.clone(),
            sub_res_limb_7_col334.clone(),
            sub_res_limb_8_col335.clone(),
            sub_res_limb_9_col336.clone(),
            sub_res_limb_10_col337.clone(),
            sub_res_limb_11_col338.clone(),
            sub_res_limb_12_col339.clone(),
            sub_res_limb_13_col340.clone(),
            sub_res_limb_14_col341.clone(),
            sub_res_limb_15_col342.clone(),
            sub_res_limb_16_col343.clone(),
            sub_res_limb_17_col344.clone(),
            sub_res_limb_18_col345.clone(),
            sub_res_limb_19_col346.clone(),
            sub_res_limb_20_col347.clone(),
            sub_res_limb_21_col348.clone(),
            sub_res_limb_22_col349.clone(),
            sub_res_limb_23_col350.clone(),
            sub_res_limb_24_col351.clone(),
            sub_res_limb_25_col352.clone(),
            sub_res_limb_26_col353.clone(),
            sub_res_limb_27_col354.clone(),
            sub_p_bit_col355.clone(),
            sub_res_limb_0_col356.clone(),
            sub_res_limb_1_col357.clone(),
            sub_res_limb_2_col358.clone(),
            sub_res_limb_3_col359.clone(),
            sub_res_limb_4_col360.clone(),
            sub_res_limb_5_col361.clone(),
            sub_res_limb_6_col362.clone(),
            sub_res_limb_7_col363.clone(),
            sub_res_limb_8_col364.clone(),
            sub_res_limb_9_col365.clone(),
            sub_res_limb_10_col366.clone(),
            sub_res_limb_11_col367.clone(),
            sub_res_limb_12_col368.clone(),
            sub_res_limb_13_col369.clone(),
            sub_res_limb_14_col370.clone(),
            sub_res_limb_15_col371.clone(),
            sub_res_limb_16_col372.clone(),
            sub_res_limb_17_col373.clone(),
            sub_res_limb_18_col374.clone(),
            sub_res_limb_19_col375.clone(),
            sub_res_limb_20_col376.clone(),
            sub_res_limb_21_col377.clone(),
            sub_res_limb_22_col378.clone(),
            sub_res_limb_23_col379.clone(),
            sub_res_limb_24_col380.clone(),
            sub_res_limb_25_col381.clone(),
            sub_res_limb_26_col382.clone(),
            sub_res_limb_27_col383.clone(),
            sub_p_bit_col384.clone(),
            mul_res_limb_0_col385.clone(),
            mul_res_limb_1_col386.clone(),
            mul_res_limb_2_col387.clone(),
            mul_res_limb_3_col388.clone(),
            mul_res_limb_4_col389.clone(),
            mul_res_limb_5_col390.clone(),
            mul_res_limb_6_col391.clone(),
            mul_res_limb_7_col392.clone(),
            mul_res_limb_8_col393.clone(),
            mul_res_limb_9_col394.clone(),
            mul_res_limb_10_col395.clone(),
            mul_res_limb_11_col396.clone(),
            mul_res_limb_12_col397.clone(),
            mul_res_limb_13_col398.clone(),
            mul_res_limb_14_col399.clone(),
            mul_res_limb_15_col400.clone(),
            mul_res_limb_16_col401.clone(),
            mul_res_limb_17_col402.clone(),
            mul_res_limb_18_col403.clone(),
            mul_res_limb_19_col404.clone(),
            mul_res_limb_20_col405.clone(),
            mul_res_limb_21_col406.clone(),
            mul_res_limb_22_col407.clone(),
            mul_res_limb_23_col408.clone(),
            mul_res_limb_24_col409.clone(),
            mul_res_limb_25_col410.clone(),
            mul_res_limb_26_col411.clone(),
            mul_res_limb_27_col412.clone(),
            k_col413.clone(),
            carry_0_col414.clone(),
            carry_1_col415.clone(),
            carry_2_col416.clone(),
            carry_3_col417.clone(),
            carry_4_col418.clone(),
            carry_5_col419.clone(),
            carry_6_col420.clone(),
            carry_7_col421.clone(),
            carry_8_col422.clone(),
            carry_9_col423.clone(),
            carry_10_col424.clone(),
            carry_11_col425.clone(),
            carry_12_col426.clone(),
            carry_13_col427.clone(),
            carry_14_col428.clone(),
            carry_15_col429.clone(),
            carry_16_col430.clone(),
            carry_17_col431.clone(),
            carry_18_col432.clone(),
            carry_19_col433.clone(),
            carry_20_col434.clone(),
            carry_21_col435.clone(),
            carry_22_col436.clone(),
            carry_23_col437.clone(),
            carry_24_col438.clone(),
            carry_25_col439.clone(),
            carry_26_col440.clone(),
            sub_res_limb_0_col441.clone(),
            sub_res_limb_1_col442.clone(),
            sub_res_limb_2_col443.clone(),
            sub_res_limb_3_col444.clone(),
            sub_res_limb_4_col445.clone(),
            sub_res_limb_5_col446.clone(),
            sub_res_limb_6_col447.clone(),
            sub_res_limb_7_col448.clone(),
            sub_res_limb_8_col449.clone(),
            sub_res_limb_9_col450.clone(),
            sub_res_limb_10_col451.clone(),
            sub_res_limb_11_col452.clone(),
            sub_res_limb_12_col453.clone(),
            sub_res_limb_13_col454.clone(),
            sub_res_limb_14_col455.clone(),
            sub_res_limb_15_col456.clone(),
            sub_res_limb_16_col457.clone(),
            sub_res_limb_17_col458.clone(),
            sub_res_limb_18_col459.clone(),
            sub_res_limb_19_col460.clone(),
            sub_res_limb_20_col461.clone(),
            sub_res_limb_21_col462.clone(),
            sub_res_limb_22_col463.clone(),
            sub_res_limb_23_col464.clone(),
            sub_res_limb_24_col465.clone(),
            sub_res_limb_25_col466.clone(),
            sub_res_limb_26_col467.clone(),
            sub_res_limb_27_col468.clone(),
            sub_p_bit_col469.clone(),
            &self.range_check_9_9_lookup_elements,
            &self.range_check_9_9_b_lookup_elements,
            &self.range_check_9_9_c_lookup_elements,
            &self.range_check_9_9_d_lookup_elements,
            &self.range_check_9_9_e_lookup_elements,
            &self.range_check_9_9_f_lookup_elements,
            &self.range_check_9_9_g_lookup_elements,
            &self.range_check_9_9_h_lookup_elements,
            &self.range_check_19_h_lookup_elements,
            &self.range_check_19_lookup_elements,
            &self.range_check_19_b_lookup_elements,
            &self.range_check_19_c_lookup_elements,
            &self.range_check_19_d_lookup_elements,
            &self.range_check_19_e_lookup_elements,
            &self.range_check_19_f_lookup_elements,
            &self.range_check_19_g_lookup_elements,
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
                sub_res_limb_0_col327.clone(),
                sub_res_limb_1_col328.clone(),
                sub_res_limb_2_col329.clone(),
                sub_res_limb_3_col330.clone(),
                sub_res_limb_4_col331.clone(),
                sub_res_limb_5_col332.clone(),
                sub_res_limb_6_col333.clone(),
                sub_res_limb_7_col334.clone(),
                sub_res_limb_8_col335.clone(),
                sub_res_limb_9_col336.clone(),
                sub_res_limb_10_col337.clone(),
                sub_res_limb_11_col338.clone(),
                sub_res_limb_12_col339.clone(),
                sub_res_limb_13_col340.clone(),
                sub_res_limb_14_col341.clone(),
                sub_res_limb_15_col342.clone(),
                sub_res_limb_16_col343.clone(),
                sub_res_limb_17_col344.clone(),
                sub_res_limb_18_col345.clone(),
                sub_res_limb_19_col346.clone(),
                sub_res_limb_20_col347.clone(),
                sub_res_limb_21_col348.clone(),
                sub_res_limb_22_col349.clone(),
                sub_res_limb_23_col350.clone(),
                sub_res_limb_24_col351.clone(),
                sub_res_limb_25_col352.clone(),
                sub_res_limb_26_col353.clone(),
                sub_res_limb_27_col354.clone(),
                sub_res_limb_0_col441.clone(),
                sub_res_limb_1_col442.clone(),
                sub_res_limb_2_col443.clone(),
                sub_res_limb_3_col444.clone(),
                sub_res_limb_4_col445.clone(),
                sub_res_limb_5_col446.clone(),
                sub_res_limb_6_col447.clone(),
                sub_res_limb_7_col448.clone(),
                sub_res_limb_8_col449.clone(),
                sub_res_limb_9_col450.clone(),
                sub_res_limb_10_col451.clone(),
                sub_res_limb_11_col452.clone(),
                sub_res_limb_12_col453.clone(),
                sub_res_limb_13_col454.clone(),
                sub_res_limb_14_col455.clone(),
                sub_res_limb_15_col456.clone(),
                sub_res_limb_16_col457.clone(),
                sub_res_limb_17_col458.clone(),
                sub_res_limb_18_col459.clone(),
                sub_res_limb_19_col460.clone(),
                sub_res_limb_20_col461.clone(),
                sub_res_limb_21_col462.clone(),
                sub_res_limb_22_col463.clone(),
                sub_res_limb_23_col464.clone(),
                sub_res_limb_24_col465.clone(),
                sub_res_limb_25_col466.clone(),
                sub_res_limb_26_col467.clone(),
                sub_res_limb_27_col468.clone(),
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
            pedersen_points_table_lookup_elements: relations::PedersenPointsTable::dummy(),
            range_check_9_9_lookup_elements: relations::RangeCheck_9_9::dummy(),
            range_check_9_9_b_lookup_elements: relations::RangeCheck_9_9_B::dummy(),
            range_check_9_9_c_lookup_elements: relations::RangeCheck_9_9_C::dummy(),
            range_check_9_9_d_lookup_elements: relations::RangeCheck_9_9_D::dummy(),
            range_check_9_9_e_lookup_elements: relations::RangeCheck_9_9_E::dummy(),
            range_check_9_9_f_lookup_elements: relations::RangeCheck_9_9_F::dummy(),
            range_check_9_9_g_lookup_elements: relations::RangeCheck_9_9_G::dummy(),
            range_check_9_9_h_lookup_elements: relations::RangeCheck_9_9_H::dummy(),
            range_check_19_h_lookup_elements: relations::RangeCheck_19_H::dummy(),
            range_check_19_lookup_elements: relations::RangeCheck_19::dummy(),
            range_check_19_b_lookup_elements: relations::RangeCheck_19_B::dummy(),
            range_check_19_c_lookup_elements: relations::RangeCheck_19_C::dummy(),
            range_check_19_d_lookup_elements: relations::RangeCheck_19_D::dummy(),
            range_check_19_e_lookup_elements: relations::RangeCheck_19_E::dummy(),
            range_check_19_f_lookup_elements: relations::RangeCheck_19_F::dummy(),
            range_check_19_g_lookup_elements: relations::RangeCheck_19_G::dummy(),
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
