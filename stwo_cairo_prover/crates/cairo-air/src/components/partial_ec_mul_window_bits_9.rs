// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::ec_add::EcAdd;

pub const N_TRACE_COLUMNS: usize = 311;
pub const RELATION_USES_PER_ROW: [RelationUse; 18] = [
    RelationUse {
        relation_id: "PartialEcMulWindowBits9",
        uses: 1,
    },
    RelationUse {
        relation_id: "PedersenPointsTableWindowBits9",
        uses: 1,
    },
    RelationUse {
        relation_id: "RangeCheck_20",
        uses: 12,
    },
    RelationUse {
        relation_id: "RangeCheck_20_B",
        uses: 12,
    },
    RelationUse {
        relation_id: "RangeCheck_20_C",
        uses: 12,
    },
    RelationUse {
        relation_id: "RangeCheck_20_D",
        uses: 12,
    },
    RelationUse {
        relation_id: "RangeCheck_20_E",
        uses: 9,
    },
    RelationUse {
        relation_id: "RangeCheck_20_F",
        uses: 9,
    },
    RelationUse {
        relation_id: "RangeCheck_20_G",
        uses: 9,
    },
    RelationUse {
        relation_id: "RangeCheck_20_H",
        uses: 9,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9",
        uses: 6,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_B",
        uses: 6,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_C",
        uses: 6,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_D",
        uses: 6,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_E",
        uses: 6,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_F",
        uses: 6,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_G",
        uses: 3,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_H",
        uses: 3,
    },
];

pub struct Eval {
    pub claim: Claim,
    pub common_lookup_elements: relations::CommonLookupElements,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct Claim {
    pub log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 65];
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
        let M31_1791500038 = E::F::from(M31::from(1791500038));
        let M31_2038149019 = E::F::from(M31::from(2038149019));
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
        let input_limb_72_col72 = eval.next_trace_mask();
        let input_limb_73_col73 = eval.next_trace_mask();
        let input_limb_74_col74 = eval.next_trace_mask();
        let input_limb_75_col75 = eval.next_trace_mask();
        let input_limb_76_col76 = eval.next_trace_mask();
        let input_limb_77_col77 = eval.next_trace_mask();
        let input_limb_78_col78 = eval.next_trace_mask();
        let input_limb_79_col79 = eval.next_trace_mask();
        let input_limb_80_col80 = eval.next_trace_mask();
        let input_limb_81_col81 = eval.next_trace_mask();
        let input_limb_82_col82 = eval.next_trace_mask();
        let input_limb_83_col83 = eval.next_trace_mask();
        let input_limb_84_col84 = eval.next_trace_mask();
        let input_limb_85_col85 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_0_col86 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_1_col87 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_2_col88 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_3_col89 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_4_col90 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_5_col91 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_6_col92 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_7_col93 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_8_col94 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_9_col95 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_10_col96 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_11_col97 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_12_col98 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_13_col99 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_14_col100 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_15_col101 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_16_col102 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_17_col103 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_18_col104 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_19_col105 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_20_col106 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_21_col107 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_22_col108 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_23_col109 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_24_col110 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_25_col111 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_26_col112 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_27_col113 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_28_col114 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_29_col115 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_30_col116 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_31_col117 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_32_col118 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_33_col119 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_34_col120 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_35_col121 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_36_col122 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_37_col123 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_38_col124 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_39_col125 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_40_col126 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_41_col127 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_42_col128 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_43_col129 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_44_col130 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_45_col131 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_46_col132 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_47_col133 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_48_col134 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_49_col135 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_50_col136 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_51_col137 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_52_col138 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_53_col139 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_54_col140 = eval.next_trace_mask();
        let pedersen_points_table_window_bits_9_output_limb_55_col141 = eval.next_trace_mask();
        let slope_limb_0_col142 = eval.next_trace_mask();
        let slope_limb_1_col143 = eval.next_trace_mask();
        let slope_limb_2_col144 = eval.next_trace_mask();
        let slope_limb_3_col145 = eval.next_trace_mask();
        let slope_limb_4_col146 = eval.next_trace_mask();
        let slope_limb_5_col147 = eval.next_trace_mask();
        let slope_limb_6_col148 = eval.next_trace_mask();
        let slope_limb_7_col149 = eval.next_trace_mask();
        let slope_limb_8_col150 = eval.next_trace_mask();
        let slope_limb_9_col151 = eval.next_trace_mask();
        let slope_limb_10_col152 = eval.next_trace_mask();
        let slope_limb_11_col153 = eval.next_trace_mask();
        let slope_limb_12_col154 = eval.next_trace_mask();
        let slope_limb_13_col155 = eval.next_trace_mask();
        let slope_limb_14_col156 = eval.next_trace_mask();
        let slope_limb_15_col157 = eval.next_trace_mask();
        let slope_limb_16_col158 = eval.next_trace_mask();
        let slope_limb_17_col159 = eval.next_trace_mask();
        let slope_limb_18_col160 = eval.next_trace_mask();
        let slope_limb_19_col161 = eval.next_trace_mask();
        let slope_limb_20_col162 = eval.next_trace_mask();
        let slope_limb_21_col163 = eval.next_trace_mask();
        let slope_limb_22_col164 = eval.next_trace_mask();
        let slope_limb_23_col165 = eval.next_trace_mask();
        let slope_limb_24_col166 = eval.next_trace_mask();
        let slope_limb_25_col167 = eval.next_trace_mask();
        let slope_limb_26_col168 = eval.next_trace_mask();
        let slope_limb_27_col169 = eval.next_trace_mask();
        let k_col170 = eval.next_trace_mask();
        let carry_0_col171 = eval.next_trace_mask();
        let carry_1_col172 = eval.next_trace_mask();
        let carry_2_col173 = eval.next_trace_mask();
        let carry_3_col174 = eval.next_trace_mask();
        let carry_4_col175 = eval.next_trace_mask();
        let carry_5_col176 = eval.next_trace_mask();
        let carry_6_col177 = eval.next_trace_mask();
        let carry_7_col178 = eval.next_trace_mask();
        let carry_8_col179 = eval.next_trace_mask();
        let carry_9_col180 = eval.next_trace_mask();
        let carry_10_col181 = eval.next_trace_mask();
        let carry_11_col182 = eval.next_trace_mask();
        let carry_12_col183 = eval.next_trace_mask();
        let carry_13_col184 = eval.next_trace_mask();
        let carry_14_col185 = eval.next_trace_mask();
        let carry_15_col186 = eval.next_trace_mask();
        let carry_16_col187 = eval.next_trace_mask();
        let carry_17_col188 = eval.next_trace_mask();
        let carry_18_col189 = eval.next_trace_mask();
        let carry_19_col190 = eval.next_trace_mask();
        let carry_20_col191 = eval.next_trace_mask();
        let carry_21_col192 = eval.next_trace_mask();
        let carry_22_col193 = eval.next_trace_mask();
        let carry_23_col194 = eval.next_trace_mask();
        let carry_24_col195 = eval.next_trace_mask();
        let carry_25_col196 = eval.next_trace_mask();
        let carry_26_col197 = eval.next_trace_mask();
        let result_x_limb_0_col198 = eval.next_trace_mask();
        let result_x_limb_1_col199 = eval.next_trace_mask();
        let result_x_limb_2_col200 = eval.next_trace_mask();
        let result_x_limb_3_col201 = eval.next_trace_mask();
        let result_x_limb_4_col202 = eval.next_trace_mask();
        let result_x_limb_5_col203 = eval.next_trace_mask();
        let result_x_limb_6_col204 = eval.next_trace_mask();
        let result_x_limb_7_col205 = eval.next_trace_mask();
        let result_x_limb_8_col206 = eval.next_trace_mask();
        let result_x_limb_9_col207 = eval.next_trace_mask();
        let result_x_limb_10_col208 = eval.next_trace_mask();
        let result_x_limb_11_col209 = eval.next_trace_mask();
        let result_x_limb_12_col210 = eval.next_trace_mask();
        let result_x_limb_13_col211 = eval.next_trace_mask();
        let result_x_limb_14_col212 = eval.next_trace_mask();
        let result_x_limb_15_col213 = eval.next_trace_mask();
        let result_x_limb_16_col214 = eval.next_trace_mask();
        let result_x_limb_17_col215 = eval.next_trace_mask();
        let result_x_limb_18_col216 = eval.next_trace_mask();
        let result_x_limb_19_col217 = eval.next_trace_mask();
        let result_x_limb_20_col218 = eval.next_trace_mask();
        let result_x_limb_21_col219 = eval.next_trace_mask();
        let result_x_limb_22_col220 = eval.next_trace_mask();
        let result_x_limb_23_col221 = eval.next_trace_mask();
        let result_x_limb_24_col222 = eval.next_trace_mask();
        let result_x_limb_25_col223 = eval.next_trace_mask();
        let result_x_limb_26_col224 = eval.next_trace_mask();
        let result_x_limb_27_col225 = eval.next_trace_mask();
        let k_col226 = eval.next_trace_mask();
        let carry_0_col227 = eval.next_trace_mask();
        let carry_1_col228 = eval.next_trace_mask();
        let carry_2_col229 = eval.next_trace_mask();
        let carry_3_col230 = eval.next_trace_mask();
        let carry_4_col231 = eval.next_trace_mask();
        let carry_5_col232 = eval.next_trace_mask();
        let carry_6_col233 = eval.next_trace_mask();
        let carry_7_col234 = eval.next_trace_mask();
        let carry_8_col235 = eval.next_trace_mask();
        let carry_9_col236 = eval.next_trace_mask();
        let carry_10_col237 = eval.next_trace_mask();
        let carry_11_col238 = eval.next_trace_mask();
        let carry_12_col239 = eval.next_trace_mask();
        let carry_13_col240 = eval.next_trace_mask();
        let carry_14_col241 = eval.next_trace_mask();
        let carry_15_col242 = eval.next_trace_mask();
        let carry_16_col243 = eval.next_trace_mask();
        let carry_17_col244 = eval.next_trace_mask();
        let carry_18_col245 = eval.next_trace_mask();
        let carry_19_col246 = eval.next_trace_mask();
        let carry_20_col247 = eval.next_trace_mask();
        let carry_21_col248 = eval.next_trace_mask();
        let carry_22_col249 = eval.next_trace_mask();
        let carry_23_col250 = eval.next_trace_mask();
        let carry_24_col251 = eval.next_trace_mask();
        let carry_25_col252 = eval.next_trace_mask();
        let carry_26_col253 = eval.next_trace_mask();
        let result_y_limb_0_col254 = eval.next_trace_mask();
        let result_y_limb_1_col255 = eval.next_trace_mask();
        let result_y_limb_2_col256 = eval.next_trace_mask();
        let result_y_limb_3_col257 = eval.next_trace_mask();
        let result_y_limb_4_col258 = eval.next_trace_mask();
        let result_y_limb_5_col259 = eval.next_trace_mask();
        let result_y_limb_6_col260 = eval.next_trace_mask();
        let result_y_limb_7_col261 = eval.next_trace_mask();
        let result_y_limb_8_col262 = eval.next_trace_mask();
        let result_y_limb_9_col263 = eval.next_trace_mask();
        let result_y_limb_10_col264 = eval.next_trace_mask();
        let result_y_limb_11_col265 = eval.next_trace_mask();
        let result_y_limb_12_col266 = eval.next_trace_mask();
        let result_y_limb_13_col267 = eval.next_trace_mask();
        let result_y_limb_14_col268 = eval.next_trace_mask();
        let result_y_limb_15_col269 = eval.next_trace_mask();
        let result_y_limb_16_col270 = eval.next_trace_mask();
        let result_y_limb_17_col271 = eval.next_trace_mask();
        let result_y_limb_18_col272 = eval.next_trace_mask();
        let result_y_limb_19_col273 = eval.next_trace_mask();
        let result_y_limb_20_col274 = eval.next_trace_mask();
        let result_y_limb_21_col275 = eval.next_trace_mask();
        let result_y_limb_22_col276 = eval.next_trace_mask();
        let result_y_limb_23_col277 = eval.next_trace_mask();
        let result_y_limb_24_col278 = eval.next_trace_mask();
        let result_y_limb_25_col279 = eval.next_trace_mask();
        let result_y_limb_26_col280 = eval.next_trace_mask();
        let result_y_limb_27_col281 = eval.next_trace_mask();
        let k_col282 = eval.next_trace_mask();
        let carry_0_col283 = eval.next_trace_mask();
        let carry_1_col284 = eval.next_trace_mask();
        let carry_2_col285 = eval.next_trace_mask();
        let carry_3_col286 = eval.next_trace_mask();
        let carry_4_col287 = eval.next_trace_mask();
        let carry_5_col288 = eval.next_trace_mask();
        let carry_6_col289 = eval.next_trace_mask();
        let carry_7_col290 = eval.next_trace_mask();
        let carry_8_col291 = eval.next_trace_mask();
        let carry_9_col292 = eval.next_trace_mask();
        let carry_10_col293 = eval.next_trace_mask();
        let carry_11_col294 = eval.next_trace_mask();
        let carry_12_col295 = eval.next_trace_mask();
        let carry_13_col296 = eval.next_trace_mask();
        let carry_14_col297 = eval.next_trace_mask();
        let carry_15_col298 = eval.next_trace_mask();
        let carry_16_col299 = eval.next_trace_mask();
        let carry_17_col300 = eval.next_trace_mask();
        let carry_18_col301 = eval.next_trace_mask();
        let carry_19_col302 = eval.next_trace_mask();
        let carry_20_col303 = eval.next_trace_mask();
        let carry_21_col304 = eval.next_trace_mask();
        let carry_22_col305 = eval.next_trace_mask();
        let carry_23_col306 = eval.next_trace_mask();
        let carry_24_col307 = eval.next_trace_mask();
        let carry_25_col308 = eval.next_trace_mask();
        let carry_26_col309 = eval.next_trace_mask();
        let enabler_col310 = eval.next_trace_mask();

        eval.add_constraint(
            ((enabler_col310.clone() * enabler_col310.clone()) - enabler_col310.clone()),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.common_lookup_elements,
            E::EF::from(M31_1.clone()),
            &[
                M31_1791500038.clone(),
                ((M31_512.clone() * input_limb_1_col1.clone()) + input_limb_2_col2.clone()),
                pedersen_points_table_window_bits_9_output_limb_0_col86.clone(),
                pedersen_points_table_window_bits_9_output_limb_1_col87.clone(),
                pedersen_points_table_window_bits_9_output_limb_2_col88.clone(),
                pedersen_points_table_window_bits_9_output_limb_3_col89.clone(),
                pedersen_points_table_window_bits_9_output_limb_4_col90.clone(),
                pedersen_points_table_window_bits_9_output_limb_5_col91.clone(),
                pedersen_points_table_window_bits_9_output_limb_6_col92.clone(),
                pedersen_points_table_window_bits_9_output_limb_7_col93.clone(),
                pedersen_points_table_window_bits_9_output_limb_8_col94.clone(),
                pedersen_points_table_window_bits_9_output_limb_9_col95.clone(),
                pedersen_points_table_window_bits_9_output_limb_10_col96.clone(),
                pedersen_points_table_window_bits_9_output_limb_11_col97.clone(),
                pedersen_points_table_window_bits_9_output_limb_12_col98.clone(),
                pedersen_points_table_window_bits_9_output_limb_13_col99.clone(),
                pedersen_points_table_window_bits_9_output_limb_14_col100.clone(),
                pedersen_points_table_window_bits_9_output_limb_15_col101.clone(),
                pedersen_points_table_window_bits_9_output_limb_16_col102.clone(),
                pedersen_points_table_window_bits_9_output_limb_17_col103.clone(),
                pedersen_points_table_window_bits_9_output_limb_18_col104.clone(),
                pedersen_points_table_window_bits_9_output_limb_19_col105.clone(),
                pedersen_points_table_window_bits_9_output_limb_20_col106.clone(),
                pedersen_points_table_window_bits_9_output_limb_21_col107.clone(),
                pedersen_points_table_window_bits_9_output_limb_22_col108.clone(),
                pedersen_points_table_window_bits_9_output_limb_23_col109.clone(),
                pedersen_points_table_window_bits_9_output_limb_24_col110.clone(),
                pedersen_points_table_window_bits_9_output_limb_25_col111.clone(),
                pedersen_points_table_window_bits_9_output_limb_26_col112.clone(),
                pedersen_points_table_window_bits_9_output_limb_27_col113.clone(),
                pedersen_points_table_window_bits_9_output_limb_28_col114.clone(),
                pedersen_points_table_window_bits_9_output_limb_29_col115.clone(),
                pedersen_points_table_window_bits_9_output_limb_30_col116.clone(),
                pedersen_points_table_window_bits_9_output_limb_31_col117.clone(),
                pedersen_points_table_window_bits_9_output_limb_32_col118.clone(),
                pedersen_points_table_window_bits_9_output_limb_33_col119.clone(),
                pedersen_points_table_window_bits_9_output_limb_34_col120.clone(),
                pedersen_points_table_window_bits_9_output_limb_35_col121.clone(),
                pedersen_points_table_window_bits_9_output_limb_36_col122.clone(),
                pedersen_points_table_window_bits_9_output_limb_37_col123.clone(),
                pedersen_points_table_window_bits_9_output_limb_38_col124.clone(),
                pedersen_points_table_window_bits_9_output_limb_39_col125.clone(),
                pedersen_points_table_window_bits_9_output_limb_40_col126.clone(),
                pedersen_points_table_window_bits_9_output_limb_41_col127.clone(),
                pedersen_points_table_window_bits_9_output_limb_42_col128.clone(),
                pedersen_points_table_window_bits_9_output_limb_43_col129.clone(),
                pedersen_points_table_window_bits_9_output_limb_44_col130.clone(),
                pedersen_points_table_window_bits_9_output_limb_45_col131.clone(),
                pedersen_points_table_window_bits_9_output_limb_46_col132.clone(),
                pedersen_points_table_window_bits_9_output_limb_47_col133.clone(),
                pedersen_points_table_window_bits_9_output_limb_48_col134.clone(),
                pedersen_points_table_window_bits_9_output_limb_49_col135.clone(),
                pedersen_points_table_window_bits_9_output_limb_50_col136.clone(),
                pedersen_points_table_window_bits_9_output_limb_51_col137.clone(),
                pedersen_points_table_window_bits_9_output_limb_52_col138.clone(),
                pedersen_points_table_window_bits_9_output_limb_53_col139.clone(),
                pedersen_points_table_window_bits_9_output_limb_54_col140.clone(),
                pedersen_points_table_window_bits_9_output_limb_55_col141.clone(),
            ],
        ));

        EcAdd::evaluate(
            [
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
                input_limb_73_col73.clone(),
                input_limb_74_col74.clone(),
                input_limb_75_col75.clone(),
                input_limb_76_col76.clone(),
                input_limb_77_col77.clone(),
                input_limb_78_col78.clone(),
                input_limb_79_col79.clone(),
                input_limb_80_col80.clone(),
                input_limb_81_col81.clone(),
                input_limb_82_col82.clone(),
                input_limb_83_col83.clone(),
                input_limb_84_col84.clone(),
                input_limb_85_col85.clone(),
                pedersen_points_table_window_bits_9_output_limb_0_col86.clone(),
                pedersen_points_table_window_bits_9_output_limb_1_col87.clone(),
                pedersen_points_table_window_bits_9_output_limb_2_col88.clone(),
                pedersen_points_table_window_bits_9_output_limb_3_col89.clone(),
                pedersen_points_table_window_bits_9_output_limb_4_col90.clone(),
                pedersen_points_table_window_bits_9_output_limb_5_col91.clone(),
                pedersen_points_table_window_bits_9_output_limb_6_col92.clone(),
                pedersen_points_table_window_bits_9_output_limb_7_col93.clone(),
                pedersen_points_table_window_bits_9_output_limb_8_col94.clone(),
                pedersen_points_table_window_bits_9_output_limb_9_col95.clone(),
                pedersen_points_table_window_bits_9_output_limb_10_col96.clone(),
                pedersen_points_table_window_bits_9_output_limb_11_col97.clone(),
                pedersen_points_table_window_bits_9_output_limb_12_col98.clone(),
                pedersen_points_table_window_bits_9_output_limb_13_col99.clone(),
                pedersen_points_table_window_bits_9_output_limb_14_col100.clone(),
                pedersen_points_table_window_bits_9_output_limb_15_col101.clone(),
                pedersen_points_table_window_bits_9_output_limb_16_col102.clone(),
                pedersen_points_table_window_bits_9_output_limb_17_col103.clone(),
                pedersen_points_table_window_bits_9_output_limb_18_col104.clone(),
                pedersen_points_table_window_bits_9_output_limb_19_col105.clone(),
                pedersen_points_table_window_bits_9_output_limb_20_col106.clone(),
                pedersen_points_table_window_bits_9_output_limb_21_col107.clone(),
                pedersen_points_table_window_bits_9_output_limb_22_col108.clone(),
                pedersen_points_table_window_bits_9_output_limb_23_col109.clone(),
                pedersen_points_table_window_bits_9_output_limb_24_col110.clone(),
                pedersen_points_table_window_bits_9_output_limb_25_col111.clone(),
                pedersen_points_table_window_bits_9_output_limb_26_col112.clone(),
                pedersen_points_table_window_bits_9_output_limb_27_col113.clone(),
                pedersen_points_table_window_bits_9_output_limb_28_col114.clone(),
                pedersen_points_table_window_bits_9_output_limb_29_col115.clone(),
                pedersen_points_table_window_bits_9_output_limb_30_col116.clone(),
                pedersen_points_table_window_bits_9_output_limb_31_col117.clone(),
                pedersen_points_table_window_bits_9_output_limb_32_col118.clone(),
                pedersen_points_table_window_bits_9_output_limb_33_col119.clone(),
                pedersen_points_table_window_bits_9_output_limb_34_col120.clone(),
                pedersen_points_table_window_bits_9_output_limb_35_col121.clone(),
                pedersen_points_table_window_bits_9_output_limb_36_col122.clone(),
                pedersen_points_table_window_bits_9_output_limb_37_col123.clone(),
                pedersen_points_table_window_bits_9_output_limb_38_col124.clone(),
                pedersen_points_table_window_bits_9_output_limb_39_col125.clone(),
                pedersen_points_table_window_bits_9_output_limb_40_col126.clone(),
                pedersen_points_table_window_bits_9_output_limb_41_col127.clone(),
                pedersen_points_table_window_bits_9_output_limb_42_col128.clone(),
                pedersen_points_table_window_bits_9_output_limb_43_col129.clone(),
                pedersen_points_table_window_bits_9_output_limb_44_col130.clone(),
                pedersen_points_table_window_bits_9_output_limb_45_col131.clone(),
                pedersen_points_table_window_bits_9_output_limb_46_col132.clone(),
                pedersen_points_table_window_bits_9_output_limb_47_col133.clone(),
                pedersen_points_table_window_bits_9_output_limb_48_col134.clone(),
                pedersen_points_table_window_bits_9_output_limb_49_col135.clone(),
                pedersen_points_table_window_bits_9_output_limb_50_col136.clone(),
                pedersen_points_table_window_bits_9_output_limb_51_col137.clone(),
                pedersen_points_table_window_bits_9_output_limb_52_col138.clone(),
                pedersen_points_table_window_bits_9_output_limb_53_col139.clone(),
                pedersen_points_table_window_bits_9_output_limb_54_col140.clone(),
                pedersen_points_table_window_bits_9_output_limb_55_col141.clone(),
            ],
            slope_limb_0_col142.clone(),
            slope_limb_1_col143.clone(),
            slope_limb_2_col144.clone(),
            slope_limb_3_col145.clone(),
            slope_limb_4_col146.clone(),
            slope_limb_5_col147.clone(),
            slope_limb_6_col148.clone(),
            slope_limb_7_col149.clone(),
            slope_limb_8_col150.clone(),
            slope_limb_9_col151.clone(),
            slope_limb_10_col152.clone(),
            slope_limb_11_col153.clone(),
            slope_limb_12_col154.clone(),
            slope_limb_13_col155.clone(),
            slope_limb_14_col156.clone(),
            slope_limb_15_col157.clone(),
            slope_limb_16_col158.clone(),
            slope_limb_17_col159.clone(),
            slope_limb_18_col160.clone(),
            slope_limb_19_col161.clone(),
            slope_limb_20_col162.clone(),
            slope_limb_21_col163.clone(),
            slope_limb_22_col164.clone(),
            slope_limb_23_col165.clone(),
            slope_limb_24_col166.clone(),
            slope_limb_25_col167.clone(),
            slope_limb_26_col168.clone(),
            slope_limb_27_col169.clone(),
            k_col170.clone(),
            carry_0_col171.clone(),
            carry_1_col172.clone(),
            carry_2_col173.clone(),
            carry_3_col174.clone(),
            carry_4_col175.clone(),
            carry_5_col176.clone(),
            carry_6_col177.clone(),
            carry_7_col178.clone(),
            carry_8_col179.clone(),
            carry_9_col180.clone(),
            carry_10_col181.clone(),
            carry_11_col182.clone(),
            carry_12_col183.clone(),
            carry_13_col184.clone(),
            carry_14_col185.clone(),
            carry_15_col186.clone(),
            carry_16_col187.clone(),
            carry_17_col188.clone(),
            carry_18_col189.clone(),
            carry_19_col190.clone(),
            carry_20_col191.clone(),
            carry_21_col192.clone(),
            carry_22_col193.clone(),
            carry_23_col194.clone(),
            carry_24_col195.clone(),
            carry_25_col196.clone(),
            carry_26_col197.clone(),
            result_x_limb_0_col198.clone(),
            result_x_limb_1_col199.clone(),
            result_x_limb_2_col200.clone(),
            result_x_limb_3_col201.clone(),
            result_x_limb_4_col202.clone(),
            result_x_limb_5_col203.clone(),
            result_x_limb_6_col204.clone(),
            result_x_limb_7_col205.clone(),
            result_x_limb_8_col206.clone(),
            result_x_limb_9_col207.clone(),
            result_x_limb_10_col208.clone(),
            result_x_limb_11_col209.clone(),
            result_x_limb_12_col210.clone(),
            result_x_limb_13_col211.clone(),
            result_x_limb_14_col212.clone(),
            result_x_limb_15_col213.clone(),
            result_x_limb_16_col214.clone(),
            result_x_limb_17_col215.clone(),
            result_x_limb_18_col216.clone(),
            result_x_limb_19_col217.clone(),
            result_x_limb_20_col218.clone(),
            result_x_limb_21_col219.clone(),
            result_x_limb_22_col220.clone(),
            result_x_limb_23_col221.clone(),
            result_x_limb_24_col222.clone(),
            result_x_limb_25_col223.clone(),
            result_x_limb_26_col224.clone(),
            result_x_limb_27_col225.clone(),
            k_col226.clone(),
            carry_0_col227.clone(),
            carry_1_col228.clone(),
            carry_2_col229.clone(),
            carry_3_col230.clone(),
            carry_4_col231.clone(),
            carry_5_col232.clone(),
            carry_6_col233.clone(),
            carry_7_col234.clone(),
            carry_8_col235.clone(),
            carry_9_col236.clone(),
            carry_10_col237.clone(),
            carry_11_col238.clone(),
            carry_12_col239.clone(),
            carry_13_col240.clone(),
            carry_14_col241.clone(),
            carry_15_col242.clone(),
            carry_16_col243.clone(),
            carry_17_col244.clone(),
            carry_18_col245.clone(),
            carry_19_col246.clone(),
            carry_20_col247.clone(),
            carry_21_col248.clone(),
            carry_22_col249.clone(),
            carry_23_col250.clone(),
            carry_24_col251.clone(),
            carry_25_col252.clone(),
            carry_26_col253.clone(),
            result_y_limb_0_col254.clone(),
            result_y_limb_1_col255.clone(),
            result_y_limb_2_col256.clone(),
            result_y_limb_3_col257.clone(),
            result_y_limb_4_col258.clone(),
            result_y_limb_5_col259.clone(),
            result_y_limb_6_col260.clone(),
            result_y_limb_7_col261.clone(),
            result_y_limb_8_col262.clone(),
            result_y_limb_9_col263.clone(),
            result_y_limb_10_col264.clone(),
            result_y_limb_11_col265.clone(),
            result_y_limb_12_col266.clone(),
            result_y_limb_13_col267.clone(),
            result_y_limb_14_col268.clone(),
            result_y_limb_15_col269.clone(),
            result_y_limb_16_col270.clone(),
            result_y_limb_17_col271.clone(),
            result_y_limb_18_col272.clone(),
            result_y_limb_19_col273.clone(),
            result_y_limb_20_col274.clone(),
            result_y_limb_21_col275.clone(),
            result_y_limb_22_col276.clone(),
            result_y_limb_23_col277.clone(),
            result_y_limb_24_col278.clone(),
            result_y_limb_25_col279.clone(),
            result_y_limb_26_col280.clone(),
            result_y_limb_27_col281.clone(),
            k_col282.clone(),
            carry_0_col283.clone(),
            carry_1_col284.clone(),
            carry_2_col285.clone(),
            carry_3_col286.clone(),
            carry_4_col287.clone(),
            carry_5_col288.clone(),
            carry_6_col289.clone(),
            carry_7_col290.clone(),
            carry_8_col291.clone(),
            carry_9_col292.clone(),
            carry_10_col293.clone(),
            carry_11_col294.clone(),
            carry_12_col295.clone(),
            carry_13_col296.clone(),
            carry_14_col297.clone(),
            carry_15_col298.clone(),
            carry_16_col299.clone(),
            carry_17_col300.clone(),
            carry_18_col301.clone(),
            carry_19_col302.clone(),
            carry_20_col303.clone(),
            carry_21_col304.clone(),
            carry_22_col305.clone(),
            carry_23_col306.clone(),
            carry_24_col307.clone(),
            carry_25_col308.clone(),
            carry_26_col309.clone(),
            &self.common_lookup_elements,
            &mut eval,
        );
        eval.add_to_relation(RelationEntry::new(
            &self.common_lookup_elements,
            E::EF::from(enabler_col310.clone()),
            &[
                M31_2038149019.clone(),
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
                input_limb_73_col73.clone(),
                input_limb_74_col74.clone(),
                input_limb_75_col75.clone(),
                input_limb_76_col76.clone(),
                input_limb_77_col77.clone(),
                input_limb_78_col78.clone(),
                input_limb_79_col79.clone(),
                input_limb_80_col80.clone(),
                input_limb_81_col81.clone(),
                input_limb_82_col82.clone(),
                input_limb_83_col83.clone(),
                input_limb_84_col84.clone(),
                input_limb_85_col85.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.common_lookup_elements,
            -E::EF::from(enabler_col310.clone()),
            &[
                M31_2038149019.clone(),
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
                M31_0.clone(),
                result_x_limb_0_col198.clone(),
                result_x_limb_1_col199.clone(),
                result_x_limb_2_col200.clone(),
                result_x_limb_3_col201.clone(),
                result_x_limb_4_col202.clone(),
                result_x_limb_5_col203.clone(),
                result_x_limb_6_col204.clone(),
                result_x_limb_7_col205.clone(),
                result_x_limb_8_col206.clone(),
                result_x_limb_9_col207.clone(),
                result_x_limb_10_col208.clone(),
                result_x_limb_11_col209.clone(),
                result_x_limb_12_col210.clone(),
                result_x_limb_13_col211.clone(),
                result_x_limb_14_col212.clone(),
                result_x_limb_15_col213.clone(),
                result_x_limb_16_col214.clone(),
                result_x_limb_17_col215.clone(),
                result_x_limb_18_col216.clone(),
                result_x_limb_19_col217.clone(),
                result_x_limb_20_col218.clone(),
                result_x_limb_21_col219.clone(),
                result_x_limb_22_col220.clone(),
                result_x_limb_23_col221.clone(),
                result_x_limb_24_col222.clone(),
                result_x_limb_25_col223.clone(),
                result_x_limb_26_col224.clone(),
                result_x_limb_27_col225.clone(),
                result_y_limb_0_col254.clone(),
                result_y_limb_1_col255.clone(),
                result_y_limb_2_col256.clone(),
                result_y_limb_3_col257.clone(),
                result_y_limb_4_col258.clone(),
                result_y_limb_5_col259.clone(),
                result_y_limb_6_col260.clone(),
                result_y_limb_7_col261.clone(),
                result_y_limb_8_col262.clone(),
                result_y_limb_9_col263.clone(),
                result_y_limb_10_col264.clone(),
                result_y_limb_11_col265.clone(),
                result_y_limb_12_col266.clone(),
                result_y_limb_13_col267.clone(),
                result_y_limb_14_col268.clone(),
                result_y_limb_15_col269.clone(),
                result_y_limb_16_col270.clone(),
                result_y_limb_17_col271.clone(),
                result_y_limb_18_col272.clone(),
                result_y_limb_19_col273.clone(),
                result_y_limb_20_col274.clone(),
                result_y_limb_21_col275.clone(),
                result_y_limb_22_col276.clone(),
                result_y_limb_23_col277.clone(),
                result_y_limb_24_col278.clone(),
                result_y_limb_25_col279.clone(),
                result_y_limb_26_col280.clone(),
                result_y_limb_27_col281.clone(),
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
    use crate::components::constraints_regression_test_values::PARTIAL_EC_MUL_WINDOW_BITS_9;

    #[test]
    fn partial_ec_mul_window_bits_9_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim { log_size: 4 },
            common_lookup_elements: relations::CommonLookupElements::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        PARTIAL_EC_MUL_WINDOW_BITS_9.assert_debug_eq(&sum);
    }
}
