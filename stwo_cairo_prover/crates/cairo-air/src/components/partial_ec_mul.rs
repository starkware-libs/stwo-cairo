// AIR version d4d3b5d6
use crate::components::prelude::*;
use crate::components::subroutines::ec_add::EcAdd;

pub const N_TRACE_COLUMNS: usize = 472;
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
    pub range_check_19_lookup_elements: relations::RangeCheck_19,
    pub range_check_19_b_lookup_elements: relations::RangeCheck_19_B,
    pub range_check_19_c_lookup_elements: relations::RangeCheck_19_C,
    pub range_check_19_d_lookup_elements: relations::RangeCheck_19_D,
    pub range_check_19_e_lookup_elements: relations::RangeCheck_19_E,
    pub range_check_19_f_lookup_elements: relations::RangeCheck_19_F,
    pub range_check_19_g_lookup_elements: relations::RangeCheck_19_G,
    pub range_check_19_h_lookup_elements: relations::RangeCheck_19_H,
    pub partial_ec_mul_lookup_elements: relations::PartialEcMul,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
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

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
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
        let input_limb_72_col72 = eval.next_trace_mask();
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
        let sub_res_limb_0_col129 = eval.next_trace_mask();
        let sub_res_limb_1_col130 = eval.next_trace_mask();
        let sub_res_limb_2_col131 = eval.next_trace_mask();
        let sub_res_limb_3_col132 = eval.next_trace_mask();
        let sub_res_limb_4_col133 = eval.next_trace_mask();
        let sub_res_limb_5_col134 = eval.next_trace_mask();
        let sub_res_limb_6_col135 = eval.next_trace_mask();
        let sub_res_limb_7_col136 = eval.next_trace_mask();
        let sub_res_limb_8_col137 = eval.next_trace_mask();
        let sub_res_limb_9_col138 = eval.next_trace_mask();
        let sub_res_limb_10_col139 = eval.next_trace_mask();
        let sub_res_limb_11_col140 = eval.next_trace_mask();
        let sub_res_limb_12_col141 = eval.next_trace_mask();
        let sub_res_limb_13_col142 = eval.next_trace_mask();
        let sub_res_limb_14_col143 = eval.next_trace_mask();
        let sub_res_limb_15_col144 = eval.next_trace_mask();
        let sub_res_limb_16_col145 = eval.next_trace_mask();
        let sub_res_limb_17_col146 = eval.next_trace_mask();
        let sub_res_limb_18_col147 = eval.next_trace_mask();
        let sub_res_limb_19_col148 = eval.next_trace_mask();
        let sub_res_limb_20_col149 = eval.next_trace_mask();
        let sub_res_limb_21_col150 = eval.next_trace_mask();
        let sub_res_limb_22_col151 = eval.next_trace_mask();
        let sub_res_limb_23_col152 = eval.next_trace_mask();
        let sub_res_limb_24_col153 = eval.next_trace_mask();
        let sub_res_limb_25_col154 = eval.next_trace_mask();
        let sub_res_limb_26_col155 = eval.next_trace_mask();
        let sub_res_limb_27_col156 = eval.next_trace_mask();
        let sub_p_bit_col157 = eval.next_trace_mask();
        let add_res_limb_0_col158 = eval.next_trace_mask();
        let add_res_limb_1_col159 = eval.next_trace_mask();
        let add_res_limb_2_col160 = eval.next_trace_mask();
        let add_res_limb_3_col161 = eval.next_trace_mask();
        let add_res_limb_4_col162 = eval.next_trace_mask();
        let add_res_limb_5_col163 = eval.next_trace_mask();
        let add_res_limb_6_col164 = eval.next_trace_mask();
        let add_res_limb_7_col165 = eval.next_trace_mask();
        let add_res_limb_8_col166 = eval.next_trace_mask();
        let add_res_limb_9_col167 = eval.next_trace_mask();
        let add_res_limb_10_col168 = eval.next_trace_mask();
        let add_res_limb_11_col169 = eval.next_trace_mask();
        let add_res_limb_12_col170 = eval.next_trace_mask();
        let add_res_limb_13_col171 = eval.next_trace_mask();
        let add_res_limb_14_col172 = eval.next_trace_mask();
        let add_res_limb_15_col173 = eval.next_trace_mask();
        let add_res_limb_16_col174 = eval.next_trace_mask();
        let add_res_limb_17_col175 = eval.next_trace_mask();
        let add_res_limb_18_col176 = eval.next_trace_mask();
        let add_res_limb_19_col177 = eval.next_trace_mask();
        let add_res_limb_20_col178 = eval.next_trace_mask();
        let add_res_limb_21_col179 = eval.next_trace_mask();
        let add_res_limb_22_col180 = eval.next_trace_mask();
        let add_res_limb_23_col181 = eval.next_trace_mask();
        let add_res_limb_24_col182 = eval.next_trace_mask();
        let add_res_limb_25_col183 = eval.next_trace_mask();
        let add_res_limb_26_col184 = eval.next_trace_mask();
        let add_res_limb_27_col185 = eval.next_trace_mask();
        let sub_p_bit_col186 = eval.next_trace_mask();
        let sub_res_limb_0_col187 = eval.next_trace_mask();
        let sub_res_limb_1_col188 = eval.next_trace_mask();
        let sub_res_limb_2_col189 = eval.next_trace_mask();
        let sub_res_limb_3_col190 = eval.next_trace_mask();
        let sub_res_limb_4_col191 = eval.next_trace_mask();
        let sub_res_limb_5_col192 = eval.next_trace_mask();
        let sub_res_limb_6_col193 = eval.next_trace_mask();
        let sub_res_limb_7_col194 = eval.next_trace_mask();
        let sub_res_limb_8_col195 = eval.next_trace_mask();
        let sub_res_limb_9_col196 = eval.next_trace_mask();
        let sub_res_limb_10_col197 = eval.next_trace_mask();
        let sub_res_limb_11_col198 = eval.next_trace_mask();
        let sub_res_limb_12_col199 = eval.next_trace_mask();
        let sub_res_limb_13_col200 = eval.next_trace_mask();
        let sub_res_limb_14_col201 = eval.next_trace_mask();
        let sub_res_limb_15_col202 = eval.next_trace_mask();
        let sub_res_limb_16_col203 = eval.next_trace_mask();
        let sub_res_limb_17_col204 = eval.next_trace_mask();
        let sub_res_limb_18_col205 = eval.next_trace_mask();
        let sub_res_limb_19_col206 = eval.next_trace_mask();
        let sub_res_limb_20_col207 = eval.next_trace_mask();
        let sub_res_limb_21_col208 = eval.next_trace_mask();
        let sub_res_limb_22_col209 = eval.next_trace_mask();
        let sub_res_limb_23_col210 = eval.next_trace_mask();
        let sub_res_limb_24_col211 = eval.next_trace_mask();
        let sub_res_limb_25_col212 = eval.next_trace_mask();
        let sub_res_limb_26_col213 = eval.next_trace_mask();
        let sub_res_limb_27_col214 = eval.next_trace_mask();
        let sub_p_bit_col215 = eval.next_trace_mask();
        let div_res_limb_0_col216 = eval.next_trace_mask();
        let div_res_limb_1_col217 = eval.next_trace_mask();
        let div_res_limb_2_col218 = eval.next_trace_mask();
        let div_res_limb_3_col219 = eval.next_trace_mask();
        let div_res_limb_4_col220 = eval.next_trace_mask();
        let div_res_limb_5_col221 = eval.next_trace_mask();
        let div_res_limb_6_col222 = eval.next_trace_mask();
        let div_res_limb_7_col223 = eval.next_trace_mask();
        let div_res_limb_8_col224 = eval.next_trace_mask();
        let div_res_limb_9_col225 = eval.next_trace_mask();
        let div_res_limb_10_col226 = eval.next_trace_mask();
        let div_res_limb_11_col227 = eval.next_trace_mask();
        let div_res_limb_12_col228 = eval.next_trace_mask();
        let div_res_limb_13_col229 = eval.next_trace_mask();
        let div_res_limb_14_col230 = eval.next_trace_mask();
        let div_res_limb_15_col231 = eval.next_trace_mask();
        let div_res_limb_16_col232 = eval.next_trace_mask();
        let div_res_limb_17_col233 = eval.next_trace_mask();
        let div_res_limb_18_col234 = eval.next_trace_mask();
        let div_res_limb_19_col235 = eval.next_trace_mask();
        let div_res_limb_20_col236 = eval.next_trace_mask();
        let div_res_limb_21_col237 = eval.next_trace_mask();
        let div_res_limb_22_col238 = eval.next_trace_mask();
        let div_res_limb_23_col239 = eval.next_trace_mask();
        let div_res_limb_24_col240 = eval.next_trace_mask();
        let div_res_limb_25_col241 = eval.next_trace_mask();
        let div_res_limb_26_col242 = eval.next_trace_mask();
        let div_res_limb_27_col243 = eval.next_trace_mask();
        let k_col244 = eval.next_trace_mask();
        let carry_0_col245 = eval.next_trace_mask();
        let carry_1_col246 = eval.next_trace_mask();
        let carry_2_col247 = eval.next_trace_mask();
        let carry_3_col248 = eval.next_trace_mask();
        let carry_4_col249 = eval.next_trace_mask();
        let carry_5_col250 = eval.next_trace_mask();
        let carry_6_col251 = eval.next_trace_mask();
        let carry_7_col252 = eval.next_trace_mask();
        let carry_8_col253 = eval.next_trace_mask();
        let carry_9_col254 = eval.next_trace_mask();
        let carry_10_col255 = eval.next_trace_mask();
        let carry_11_col256 = eval.next_trace_mask();
        let carry_12_col257 = eval.next_trace_mask();
        let carry_13_col258 = eval.next_trace_mask();
        let carry_14_col259 = eval.next_trace_mask();
        let carry_15_col260 = eval.next_trace_mask();
        let carry_16_col261 = eval.next_trace_mask();
        let carry_17_col262 = eval.next_trace_mask();
        let carry_18_col263 = eval.next_trace_mask();
        let carry_19_col264 = eval.next_trace_mask();
        let carry_20_col265 = eval.next_trace_mask();
        let carry_21_col266 = eval.next_trace_mask();
        let carry_22_col267 = eval.next_trace_mask();
        let carry_23_col268 = eval.next_trace_mask();
        let carry_24_col269 = eval.next_trace_mask();
        let carry_25_col270 = eval.next_trace_mask();
        let carry_26_col271 = eval.next_trace_mask();
        let mul_res_limb_0_col272 = eval.next_trace_mask();
        let mul_res_limb_1_col273 = eval.next_trace_mask();
        let mul_res_limb_2_col274 = eval.next_trace_mask();
        let mul_res_limb_3_col275 = eval.next_trace_mask();
        let mul_res_limb_4_col276 = eval.next_trace_mask();
        let mul_res_limb_5_col277 = eval.next_trace_mask();
        let mul_res_limb_6_col278 = eval.next_trace_mask();
        let mul_res_limb_7_col279 = eval.next_trace_mask();
        let mul_res_limb_8_col280 = eval.next_trace_mask();
        let mul_res_limb_9_col281 = eval.next_trace_mask();
        let mul_res_limb_10_col282 = eval.next_trace_mask();
        let mul_res_limb_11_col283 = eval.next_trace_mask();
        let mul_res_limb_12_col284 = eval.next_trace_mask();
        let mul_res_limb_13_col285 = eval.next_trace_mask();
        let mul_res_limb_14_col286 = eval.next_trace_mask();
        let mul_res_limb_15_col287 = eval.next_trace_mask();
        let mul_res_limb_16_col288 = eval.next_trace_mask();
        let mul_res_limb_17_col289 = eval.next_trace_mask();
        let mul_res_limb_18_col290 = eval.next_trace_mask();
        let mul_res_limb_19_col291 = eval.next_trace_mask();
        let mul_res_limb_20_col292 = eval.next_trace_mask();
        let mul_res_limb_21_col293 = eval.next_trace_mask();
        let mul_res_limb_22_col294 = eval.next_trace_mask();
        let mul_res_limb_23_col295 = eval.next_trace_mask();
        let mul_res_limb_24_col296 = eval.next_trace_mask();
        let mul_res_limb_25_col297 = eval.next_trace_mask();
        let mul_res_limb_26_col298 = eval.next_trace_mask();
        let mul_res_limb_27_col299 = eval.next_trace_mask();
        let k_col300 = eval.next_trace_mask();
        let carry_0_col301 = eval.next_trace_mask();
        let carry_1_col302 = eval.next_trace_mask();
        let carry_2_col303 = eval.next_trace_mask();
        let carry_3_col304 = eval.next_trace_mask();
        let carry_4_col305 = eval.next_trace_mask();
        let carry_5_col306 = eval.next_trace_mask();
        let carry_6_col307 = eval.next_trace_mask();
        let carry_7_col308 = eval.next_trace_mask();
        let carry_8_col309 = eval.next_trace_mask();
        let carry_9_col310 = eval.next_trace_mask();
        let carry_10_col311 = eval.next_trace_mask();
        let carry_11_col312 = eval.next_trace_mask();
        let carry_12_col313 = eval.next_trace_mask();
        let carry_13_col314 = eval.next_trace_mask();
        let carry_14_col315 = eval.next_trace_mask();
        let carry_15_col316 = eval.next_trace_mask();
        let carry_16_col317 = eval.next_trace_mask();
        let carry_17_col318 = eval.next_trace_mask();
        let carry_18_col319 = eval.next_trace_mask();
        let carry_19_col320 = eval.next_trace_mask();
        let carry_20_col321 = eval.next_trace_mask();
        let carry_21_col322 = eval.next_trace_mask();
        let carry_22_col323 = eval.next_trace_mask();
        let carry_23_col324 = eval.next_trace_mask();
        let carry_24_col325 = eval.next_trace_mask();
        let carry_25_col326 = eval.next_trace_mask();
        let carry_26_col327 = eval.next_trace_mask();
        let sub_res_limb_0_col328 = eval.next_trace_mask();
        let sub_res_limb_1_col329 = eval.next_trace_mask();
        let sub_res_limb_2_col330 = eval.next_trace_mask();
        let sub_res_limb_3_col331 = eval.next_trace_mask();
        let sub_res_limb_4_col332 = eval.next_trace_mask();
        let sub_res_limb_5_col333 = eval.next_trace_mask();
        let sub_res_limb_6_col334 = eval.next_trace_mask();
        let sub_res_limb_7_col335 = eval.next_trace_mask();
        let sub_res_limb_8_col336 = eval.next_trace_mask();
        let sub_res_limb_9_col337 = eval.next_trace_mask();
        let sub_res_limb_10_col338 = eval.next_trace_mask();
        let sub_res_limb_11_col339 = eval.next_trace_mask();
        let sub_res_limb_12_col340 = eval.next_trace_mask();
        let sub_res_limb_13_col341 = eval.next_trace_mask();
        let sub_res_limb_14_col342 = eval.next_trace_mask();
        let sub_res_limb_15_col343 = eval.next_trace_mask();
        let sub_res_limb_16_col344 = eval.next_trace_mask();
        let sub_res_limb_17_col345 = eval.next_trace_mask();
        let sub_res_limb_18_col346 = eval.next_trace_mask();
        let sub_res_limb_19_col347 = eval.next_trace_mask();
        let sub_res_limb_20_col348 = eval.next_trace_mask();
        let sub_res_limb_21_col349 = eval.next_trace_mask();
        let sub_res_limb_22_col350 = eval.next_trace_mask();
        let sub_res_limb_23_col351 = eval.next_trace_mask();
        let sub_res_limb_24_col352 = eval.next_trace_mask();
        let sub_res_limb_25_col353 = eval.next_trace_mask();
        let sub_res_limb_26_col354 = eval.next_trace_mask();
        let sub_res_limb_27_col355 = eval.next_trace_mask();
        let sub_p_bit_col356 = eval.next_trace_mask();
        let sub_res_limb_0_col357 = eval.next_trace_mask();
        let sub_res_limb_1_col358 = eval.next_trace_mask();
        let sub_res_limb_2_col359 = eval.next_trace_mask();
        let sub_res_limb_3_col360 = eval.next_trace_mask();
        let sub_res_limb_4_col361 = eval.next_trace_mask();
        let sub_res_limb_5_col362 = eval.next_trace_mask();
        let sub_res_limb_6_col363 = eval.next_trace_mask();
        let sub_res_limb_7_col364 = eval.next_trace_mask();
        let sub_res_limb_8_col365 = eval.next_trace_mask();
        let sub_res_limb_9_col366 = eval.next_trace_mask();
        let sub_res_limb_10_col367 = eval.next_trace_mask();
        let sub_res_limb_11_col368 = eval.next_trace_mask();
        let sub_res_limb_12_col369 = eval.next_trace_mask();
        let sub_res_limb_13_col370 = eval.next_trace_mask();
        let sub_res_limb_14_col371 = eval.next_trace_mask();
        let sub_res_limb_15_col372 = eval.next_trace_mask();
        let sub_res_limb_16_col373 = eval.next_trace_mask();
        let sub_res_limb_17_col374 = eval.next_trace_mask();
        let sub_res_limb_18_col375 = eval.next_trace_mask();
        let sub_res_limb_19_col376 = eval.next_trace_mask();
        let sub_res_limb_20_col377 = eval.next_trace_mask();
        let sub_res_limb_21_col378 = eval.next_trace_mask();
        let sub_res_limb_22_col379 = eval.next_trace_mask();
        let sub_res_limb_23_col380 = eval.next_trace_mask();
        let sub_res_limb_24_col381 = eval.next_trace_mask();
        let sub_res_limb_25_col382 = eval.next_trace_mask();
        let sub_res_limb_26_col383 = eval.next_trace_mask();
        let sub_res_limb_27_col384 = eval.next_trace_mask();
        let sub_p_bit_col385 = eval.next_trace_mask();
        let mul_res_limb_0_col386 = eval.next_trace_mask();
        let mul_res_limb_1_col387 = eval.next_trace_mask();
        let mul_res_limb_2_col388 = eval.next_trace_mask();
        let mul_res_limb_3_col389 = eval.next_trace_mask();
        let mul_res_limb_4_col390 = eval.next_trace_mask();
        let mul_res_limb_5_col391 = eval.next_trace_mask();
        let mul_res_limb_6_col392 = eval.next_trace_mask();
        let mul_res_limb_7_col393 = eval.next_trace_mask();
        let mul_res_limb_8_col394 = eval.next_trace_mask();
        let mul_res_limb_9_col395 = eval.next_trace_mask();
        let mul_res_limb_10_col396 = eval.next_trace_mask();
        let mul_res_limb_11_col397 = eval.next_trace_mask();
        let mul_res_limb_12_col398 = eval.next_trace_mask();
        let mul_res_limb_13_col399 = eval.next_trace_mask();
        let mul_res_limb_14_col400 = eval.next_trace_mask();
        let mul_res_limb_15_col401 = eval.next_trace_mask();
        let mul_res_limb_16_col402 = eval.next_trace_mask();
        let mul_res_limb_17_col403 = eval.next_trace_mask();
        let mul_res_limb_18_col404 = eval.next_trace_mask();
        let mul_res_limb_19_col405 = eval.next_trace_mask();
        let mul_res_limb_20_col406 = eval.next_trace_mask();
        let mul_res_limb_21_col407 = eval.next_trace_mask();
        let mul_res_limb_22_col408 = eval.next_trace_mask();
        let mul_res_limb_23_col409 = eval.next_trace_mask();
        let mul_res_limb_24_col410 = eval.next_trace_mask();
        let mul_res_limb_25_col411 = eval.next_trace_mask();
        let mul_res_limb_26_col412 = eval.next_trace_mask();
        let mul_res_limb_27_col413 = eval.next_trace_mask();
        let k_col414 = eval.next_trace_mask();
        let carry_0_col415 = eval.next_trace_mask();
        let carry_1_col416 = eval.next_trace_mask();
        let carry_2_col417 = eval.next_trace_mask();
        let carry_3_col418 = eval.next_trace_mask();
        let carry_4_col419 = eval.next_trace_mask();
        let carry_5_col420 = eval.next_trace_mask();
        let carry_6_col421 = eval.next_trace_mask();
        let carry_7_col422 = eval.next_trace_mask();
        let carry_8_col423 = eval.next_trace_mask();
        let carry_9_col424 = eval.next_trace_mask();
        let carry_10_col425 = eval.next_trace_mask();
        let carry_11_col426 = eval.next_trace_mask();
        let carry_12_col427 = eval.next_trace_mask();
        let carry_13_col428 = eval.next_trace_mask();
        let carry_14_col429 = eval.next_trace_mask();
        let carry_15_col430 = eval.next_trace_mask();
        let carry_16_col431 = eval.next_trace_mask();
        let carry_17_col432 = eval.next_trace_mask();
        let carry_18_col433 = eval.next_trace_mask();
        let carry_19_col434 = eval.next_trace_mask();
        let carry_20_col435 = eval.next_trace_mask();
        let carry_21_col436 = eval.next_trace_mask();
        let carry_22_col437 = eval.next_trace_mask();
        let carry_23_col438 = eval.next_trace_mask();
        let carry_24_col439 = eval.next_trace_mask();
        let carry_25_col440 = eval.next_trace_mask();
        let carry_26_col441 = eval.next_trace_mask();
        let sub_res_limb_0_col442 = eval.next_trace_mask();
        let sub_res_limb_1_col443 = eval.next_trace_mask();
        let sub_res_limb_2_col444 = eval.next_trace_mask();
        let sub_res_limb_3_col445 = eval.next_trace_mask();
        let sub_res_limb_4_col446 = eval.next_trace_mask();
        let sub_res_limb_5_col447 = eval.next_trace_mask();
        let sub_res_limb_6_col448 = eval.next_trace_mask();
        let sub_res_limb_7_col449 = eval.next_trace_mask();
        let sub_res_limb_8_col450 = eval.next_trace_mask();
        let sub_res_limb_9_col451 = eval.next_trace_mask();
        let sub_res_limb_10_col452 = eval.next_trace_mask();
        let sub_res_limb_11_col453 = eval.next_trace_mask();
        let sub_res_limb_12_col454 = eval.next_trace_mask();
        let sub_res_limb_13_col455 = eval.next_trace_mask();
        let sub_res_limb_14_col456 = eval.next_trace_mask();
        let sub_res_limb_15_col457 = eval.next_trace_mask();
        let sub_res_limb_16_col458 = eval.next_trace_mask();
        let sub_res_limb_17_col459 = eval.next_trace_mask();
        let sub_res_limb_18_col460 = eval.next_trace_mask();
        let sub_res_limb_19_col461 = eval.next_trace_mask();
        let sub_res_limb_20_col462 = eval.next_trace_mask();
        let sub_res_limb_21_col463 = eval.next_trace_mask();
        let sub_res_limb_22_col464 = eval.next_trace_mask();
        let sub_res_limb_23_col465 = eval.next_trace_mask();
        let sub_res_limb_24_col466 = eval.next_trace_mask();
        let sub_res_limb_25_col467 = eval.next_trace_mask();
        let sub_res_limb_26_col468 = eval.next_trace_mask();
        let sub_res_limb_27_col469 = eval.next_trace_mask();
        let sub_p_bit_col470 = eval.next_trace_mask();
        let enabler = eval.next_trace_mask();

        eval.add_constraint(enabler.clone() * enabler.clone() - enabler.clone());

        eval.add_to_relation(RelationEntry::new(
            &self.pedersen_points_table_lookup_elements,
            E::EF::one(),
            &[
                ((input_limb_2_col2.clone() + (M31_262144.clone() * input_limb_1_col1.clone()))
                    + input_limb_3_col3.clone()),
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
                input_limb_72_col72.clone(),
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
            sub_res_limb_0_col129.clone(),
            sub_res_limb_1_col130.clone(),
            sub_res_limb_2_col131.clone(),
            sub_res_limb_3_col132.clone(),
            sub_res_limb_4_col133.clone(),
            sub_res_limb_5_col134.clone(),
            sub_res_limb_6_col135.clone(),
            sub_res_limb_7_col136.clone(),
            sub_res_limb_8_col137.clone(),
            sub_res_limb_9_col138.clone(),
            sub_res_limb_10_col139.clone(),
            sub_res_limb_11_col140.clone(),
            sub_res_limb_12_col141.clone(),
            sub_res_limb_13_col142.clone(),
            sub_res_limb_14_col143.clone(),
            sub_res_limb_15_col144.clone(),
            sub_res_limb_16_col145.clone(),
            sub_res_limb_17_col146.clone(),
            sub_res_limb_18_col147.clone(),
            sub_res_limb_19_col148.clone(),
            sub_res_limb_20_col149.clone(),
            sub_res_limb_21_col150.clone(),
            sub_res_limb_22_col151.clone(),
            sub_res_limb_23_col152.clone(),
            sub_res_limb_24_col153.clone(),
            sub_res_limb_25_col154.clone(),
            sub_res_limb_26_col155.clone(),
            sub_res_limb_27_col156.clone(),
            sub_p_bit_col157.clone(),
            add_res_limb_0_col158.clone(),
            add_res_limb_1_col159.clone(),
            add_res_limb_2_col160.clone(),
            add_res_limb_3_col161.clone(),
            add_res_limb_4_col162.clone(),
            add_res_limb_5_col163.clone(),
            add_res_limb_6_col164.clone(),
            add_res_limb_7_col165.clone(),
            add_res_limb_8_col166.clone(),
            add_res_limb_9_col167.clone(),
            add_res_limb_10_col168.clone(),
            add_res_limb_11_col169.clone(),
            add_res_limb_12_col170.clone(),
            add_res_limb_13_col171.clone(),
            add_res_limb_14_col172.clone(),
            add_res_limb_15_col173.clone(),
            add_res_limb_16_col174.clone(),
            add_res_limb_17_col175.clone(),
            add_res_limb_18_col176.clone(),
            add_res_limb_19_col177.clone(),
            add_res_limb_20_col178.clone(),
            add_res_limb_21_col179.clone(),
            add_res_limb_22_col180.clone(),
            add_res_limb_23_col181.clone(),
            add_res_limb_24_col182.clone(),
            add_res_limb_25_col183.clone(),
            add_res_limb_26_col184.clone(),
            add_res_limb_27_col185.clone(),
            sub_p_bit_col186.clone(),
            sub_res_limb_0_col187.clone(),
            sub_res_limb_1_col188.clone(),
            sub_res_limb_2_col189.clone(),
            sub_res_limb_3_col190.clone(),
            sub_res_limb_4_col191.clone(),
            sub_res_limb_5_col192.clone(),
            sub_res_limb_6_col193.clone(),
            sub_res_limb_7_col194.clone(),
            sub_res_limb_8_col195.clone(),
            sub_res_limb_9_col196.clone(),
            sub_res_limb_10_col197.clone(),
            sub_res_limb_11_col198.clone(),
            sub_res_limb_12_col199.clone(),
            sub_res_limb_13_col200.clone(),
            sub_res_limb_14_col201.clone(),
            sub_res_limb_15_col202.clone(),
            sub_res_limb_16_col203.clone(),
            sub_res_limb_17_col204.clone(),
            sub_res_limb_18_col205.clone(),
            sub_res_limb_19_col206.clone(),
            sub_res_limb_20_col207.clone(),
            sub_res_limb_21_col208.clone(),
            sub_res_limb_22_col209.clone(),
            sub_res_limb_23_col210.clone(),
            sub_res_limb_24_col211.clone(),
            sub_res_limb_25_col212.clone(),
            sub_res_limb_26_col213.clone(),
            sub_res_limb_27_col214.clone(),
            sub_p_bit_col215.clone(),
            div_res_limb_0_col216.clone(),
            div_res_limb_1_col217.clone(),
            div_res_limb_2_col218.clone(),
            div_res_limb_3_col219.clone(),
            div_res_limb_4_col220.clone(),
            div_res_limb_5_col221.clone(),
            div_res_limb_6_col222.clone(),
            div_res_limb_7_col223.clone(),
            div_res_limb_8_col224.clone(),
            div_res_limb_9_col225.clone(),
            div_res_limb_10_col226.clone(),
            div_res_limb_11_col227.clone(),
            div_res_limb_12_col228.clone(),
            div_res_limb_13_col229.clone(),
            div_res_limb_14_col230.clone(),
            div_res_limb_15_col231.clone(),
            div_res_limb_16_col232.clone(),
            div_res_limb_17_col233.clone(),
            div_res_limb_18_col234.clone(),
            div_res_limb_19_col235.clone(),
            div_res_limb_20_col236.clone(),
            div_res_limb_21_col237.clone(),
            div_res_limb_22_col238.clone(),
            div_res_limb_23_col239.clone(),
            div_res_limb_24_col240.clone(),
            div_res_limb_25_col241.clone(),
            div_res_limb_26_col242.clone(),
            div_res_limb_27_col243.clone(),
            k_col244.clone(),
            carry_0_col245.clone(),
            carry_1_col246.clone(),
            carry_2_col247.clone(),
            carry_3_col248.clone(),
            carry_4_col249.clone(),
            carry_5_col250.clone(),
            carry_6_col251.clone(),
            carry_7_col252.clone(),
            carry_8_col253.clone(),
            carry_9_col254.clone(),
            carry_10_col255.clone(),
            carry_11_col256.clone(),
            carry_12_col257.clone(),
            carry_13_col258.clone(),
            carry_14_col259.clone(),
            carry_15_col260.clone(),
            carry_16_col261.clone(),
            carry_17_col262.clone(),
            carry_18_col263.clone(),
            carry_19_col264.clone(),
            carry_20_col265.clone(),
            carry_21_col266.clone(),
            carry_22_col267.clone(),
            carry_23_col268.clone(),
            carry_24_col269.clone(),
            carry_25_col270.clone(),
            carry_26_col271.clone(),
            mul_res_limb_0_col272.clone(),
            mul_res_limb_1_col273.clone(),
            mul_res_limb_2_col274.clone(),
            mul_res_limb_3_col275.clone(),
            mul_res_limb_4_col276.clone(),
            mul_res_limb_5_col277.clone(),
            mul_res_limb_6_col278.clone(),
            mul_res_limb_7_col279.clone(),
            mul_res_limb_8_col280.clone(),
            mul_res_limb_9_col281.clone(),
            mul_res_limb_10_col282.clone(),
            mul_res_limb_11_col283.clone(),
            mul_res_limb_12_col284.clone(),
            mul_res_limb_13_col285.clone(),
            mul_res_limb_14_col286.clone(),
            mul_res_limb_15_col287.clone(),
            mul_res_limb_16_col288.clone(),
            mul_res_limb_17_col289.clone(),
            mul_res_limb_18_col290.clone(),
            mul_res_limb_19_col291.clone(),
            mul_res_limb_20_col292.clone(),
            mul_res_limb_21_col293.clone(),
            mul_res_limb_22_col294.clone(),
            mul_res_limb_23_col295.clone(),
            mul_res_limb_24_col296.clone(),
            mul_res_limb_25_col297.clone(),
            mul_res_limb_26_col298.clone(),
            mul_res_limb_27_col299.clone(),
            k_col300.clone(),
            carry_0_col301.clone(),
            carry_1_col302.clone(),
            carry_2_col303.clone(),
            carry_3_col304.clone(),
            carry_4_col305.clone(),
            carry_5_col306.clone(),
            carry_6_col307.clone(),
            carry_7_col308.clone(),
            carry_8_col309.clone(),
            carry_9_col310.clone(),
            carry_10_col311.clone(),
            carry_11_col312.clone(),
            carry_12_col313.clone(),
            carry_13_col314.clone(),
            carry_14_col315.clone(),
            carry_15_col316.clone(),
            carry_16_col317.clone(),
            carry_17_col318.clone(),
            carry_18_col319.clone(),
            carry_19_col320.clone(),
            carry_20_col321.clone(),
            carry_21_col322.clone(),
            carry_22_col323.clone(),
            carry_23_col324.clone(),
            carry_24_col325.clone(),
            carry_25_col326.clone(),
            carry_26_col327.clone(),
            sub_res_limb_0_col328.clone(),
            sub_res_limb_1_col329.clone(),
            sub_res_limb_2_col330.clone(),
            sub_res_limb_3_col331.clone(),
            sub_res_limb_4_col332.clone(),
            sub_res_limb_5_col333.clone(),
            sub_res_limb_6_col334.clone(),
            sub_res_limb_7_col335.clone(),
            sub_res_limb_8_col336.clone(),
            sub_res_limb_9_col337.clone(),
            sub_res_limb_10_col338.clone(),
            sub_res_limb_11_col339.clone(),
            sub_res_limb_12_col340.clone(),
            sub_res_limb_13_col341.clone(),
            sub_res_limb_14_col342.clone(),
            sub_res_limb_15_col343.clone(),
            sub_res_limb_16_col344.clone(),
            sub_res_limb_17_col345.clone(),
            sub_res_limb_18_col346.clone(),
            sub_res_limb_19_col347.clone(),
            sub_res_limb_20_col348.clone(),
            sub_res_limb_21_col349.clone(),
            sub_res_limb_22_col350.clone(),
            sub_res_limb_23_col351.clone(),
            sub_res_limb_24_col352.clone(),
            sub_res_limb_25_col353.clone(),
            sub_res_limb_26_col354.clone(),
            sub_res_limb_27_col355.clone(),
            sub_p_bit_col356.clone(),
            sub_res_limb_0_col357.clone(),
            sub_res_limb_1_col358.clone(),
            sub_res_limb_2_col359.clone(),
            sub_res_limb_3_col360.clone(),
            sub_res_limb_4_col361.clone(),
            sub_res_limb_5_col362.clone(),
            sub_res_limb_6_col363.clone(),
            sub_res_limb_7_col364.clone(),
            sub_res_limb_8_col365.clone(),
            sub_res_limb_9_col366.clone(),
            sub_res_limb_10_col367.clone(),
            sub_res_limb_11_col368.clone(),
            sub_res_limb_12_col369.clone(),
            sub_res_limb_13_col370.clone(),
            sub_res_limb_14_col371.clone(),
            sub_res_limb_15_col372.clone(),
            sub_res_limb_16_col373.clone(),
            sub_res_limb_17_col374.clone(),
            sub_res_limb_18_col375.clone(),
            sub_res_limb_19_col376.clone(),
            sub_res_limb_20_col377.clone(),
            sub_res_limb_21_col378.clone(),
            sub_res_limb_22_col379.clone(),
            sub_res_limb_23_col380.clone(),
            sub_res_limb_24_col381.clone(),
            sub_res_limb_25_col382.clone(),
            sub_res_limb_26_col383.clone(),
            sub_res_limb_27_col384.clone(),
            sub_p_bit_col385.clone(),
            mul_res_limb_0_col386.clone(),
            mul_res_limb_1_col387.clone(),
            mul_res_limb_2_col388.clone(),
            mul_res_limb_3_col389.clone(),
            mul_res_limb_4_col390.clone(),
            mul_res_limb_5_col391.clone(),
            mul_res_limb_6_col392.clone(),
            mul_res_limb_7_col393.clone(),
            mul_res_limb_8_col394.clone(),
            mul_res_limb_9_col395.clone(),
            mul_res_limb_10_col396.clone(),
            mul_res_limb_11_col397.clone(),
            mul_res_limb_12_col398.clone(),
            mul_res_limb_13_col399.clone(),
            mul_res_limb_14_col400.clone(),
            mul_res_limb_15_col401.clone(),
            mul_res_limb_16_col402.clone(),
            mul_res_limb_17_col403.clone(),
            mul_res_limb_18_col404.clone(),
            mul_res_limb_19_col405.clone(),
            mul_res_limb_20_col406.clone(),
            mul_res_limb_21_col407.clone(),
            mul_res_limb_22_col408.clone(),
            mul_res_limb_23_col409.clone(),
            mul_res_limb_24_col410.clone(),
            mul_res_limb_25_col411.clone(),
            mul_res_limb_26_col412.clone(),
            mul_res_limb_27_col413.clone(),
            k_col414.clone(),
            carry_0_col415.clone(),
            carry_1_col416.clone(),
            carry_2_col417.clone(),
            carry_3_col418.clone(),
            carry_4_col419.clone(),
            carry_5_col420.clone(),
            carry_6_col421.clone(),
            carry_7_col422.clone(),
            carry_8_col423.clone(),
            carry_9_col424.clone(),
            carry_10_col425.clone(),
            carry_11_col426.clone(),
            carry_12_col427.clone(),
            carry_13_col428.clone(),
            carry_14_col429.clone(),
            carry_15_col430.clone(),
            carry_16_col431.clone(),
            carry_17_col432.clone(),
            carry_18_col433.clone(),
            carry_19_col434.clone(),
            carry_20_col435.clone(),
            carry_21_col436.clone(),
            carry_22_col437.clone(),
            carry_23_col438.clone(),
            carry_24_col439.clone(),
            carry_25_col440.clone(),
            carry_26_col441.clone(),
            sub_res_limb_0_col442.clone(),
            sub_res_limb_1_col443.clone(),
            sub_res_limb_2_col444.clone(),
            sub_res_limb_3_col445.clone(),
            sub_res_limb_4_col446.clone(),
            sub_res_limb_5_col447.clone(),
            sub_res_limb_6_col448.clone(),
            sub_res_limb_7_col449.clone(),
            sub_res_limb_8_col450.clone(),
            sub_res_limb_9_col451.clone(),
            sub_res_limb_10_col452.clone(),
            sub_res_limb_11_col453.clone(),
            sub_res_limb_12_col454.clone(),
            sub_res_limb_13_col455.clone(),
            sub_res_limb_14_col456.clone(),
            sub_res_limb_15_col457.clone(),
            sub_res_limb_16_col458.clone(),
            sub_res_limb_17_col459.clone(),
            sub_res_limb_18_col460.clone(),
            sub_res_limb_19_col461.clone(),
            sub_res_limb_20_col462.clone(),
            sub_res_limb_21_col463.clone(),
            sub_res_limb_22_col464.clone(),
            sub_res_limb_23_col465.clone(),
            sub_res_limb_24_col466.clone(),
            sub_res_limb_25_col467.clone(),
            sub_res_limb_26_col468.clone(),
            sub_res_limb_27_col469.clone(),
            sub_p_bit_col470.clone(),
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
                input_limb_72_col72.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.partial_ec_mul_lookup_elements,
            -E::EF::from(enabler.clone()),
            &[
                input_limb_0_col0.clone(),
                (input_limb_1_col1.clone() + M31_1.clone()),
                input_limb_2_col2.clone(),
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
                M31_0.clone(),
                sub_res_limb_0_col328.clone(),
                sub_res_limb_1_col329.clone(),
                sub_res_limb_2_col330.clone(),
                sub_res_limb_3_col331.clone(),
                sub_res_limb_4_col332.clone(),
                sub_res_limb_5_col333.clone(),
                sub_res_limb_6_col334.clone(),
                sub_res_limb_7_col335.clone(),
                sub_res_limb_8_col336.clone(),
                sub_res_limb_9_col337.clone(),
                sub_res_limb_10_col338.clone(),
                sub_res_limb_11_col339.clone(),
                sub_res_limb_12_col340.clone(),
                sub_res_limb_13_col341.clone(),
                sub_res_limb_14_col342.clone(),
                sub_res_limb_15_col343.clone(),
                sub_res_limb_16_col344.clone(),
                sub_res_limb_17_col345.clone(),
                sub_res_limb_18_col346.clone(),
                sub_res_limb_19_col347.clone(),
                sub_res_limb_20_col348.clone(),
                sub_res_limb_21_col349.clone(),
                sub_res_limb_22_col350.clone(),
                sub_res_limb_23_col351.clone(),
                sub_res_limb_24_col352.clone(),
                sub_res_limb_25_col353.clone(),
                sub_res_limb_26_col354.clone(),
                sub_res_limb_27_col355.clone(),
                sub_res_limb_0_col442.clone(),
                sub_res_limb_1_col443.clone(),
                sub_res_limb_2_col444.clone(),
                sub_res_limb_3_col445.clone(),
                sub_res_limb_4_col446.clone(),
                sub_res_limb_5_col447.clone(),
                sub_res_limb_6_col448.clone(),
                sub_res_limb_7_col449.clone(),
                sub_res_limb_8_col450.clone(),
                sub_res_limb_9_col451.clone(),
                sub_res_limb_10_col452.clone(),
                sub_res_limb_11_col453.clone(),
                sub_res_limb_12_col454.clone(),
                sub_res_limb_13_col455.clone(),
                sub_res_limb_14_col456.clone(),
                sub_res_limb_15_col457.clone(),
                sub_res_limb_16_col458.clone(),
                sub_res_limb_17_col459.clone(),
                sub_res_limb_18_col460.clone(),
                sub_res_limb_19_col461.clone(),
                sub_res_limb_20_col462.clone(),
                sub_res_limb_21_col463.clone(),
                sub_res_limb_22_col464.clone(),
                sub_res_limb_23_col465.clone(),
                sub_res_limb_24_col466.clone(),
                sub_res_limb_25_col467.clone(),
                sub_res_limb_26_col468.clone(),
                sub_res_limb_27_col469.clone(),
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
    use stwo_prover::constraint_framework::expr::ExprEvaluator;
    use stwo_prover::core::fields::qm31::QM31;

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
