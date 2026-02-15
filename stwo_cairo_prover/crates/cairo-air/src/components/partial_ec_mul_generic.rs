// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::ec_add::EcAdd;
use crate::components::subroutines::ec_double::EcDouble;
use crate::components::subroutines::verify_reduced_252::VerifyReduced252;

pub const N_TRACE_COLUMNS: usize = 624;
pub const RELATION_USES_PER_ROW: [RelationUse; 18] = [
    RelationUse {
        relation_id: "PartialEcMulGeneric",
        uses: 1,
    },
    RelationUse {
        relation_id: "RangeCheck_20",
        uses: 28,
    },
    RelationUse {
        relation_id: "RangeCheck_20_B",
        uses: 28,
    },
    RelationUse {
        relation_id: "RangeCheck_20_C",
        uses: 28,
    },
    RelationUse {
        relation_id: "RangeCheck_20_D",
        uses: 28,
    },
    RelationUse {
        relation_id: "RangeCheck_20_E",
        uses: 21,
    },
    RelationUse {
        relation_id: "RangeCheck_20_F",
        uses: 21,
    },
    RelationUse {
        relation_id: "RangeCheck_20_G",
        uses: 21,
    },
    RelationUse {
        relation_id: "RangeCheck_20_H",
        uses: 21,
    },
    RelationUse {
        relation_id: "RangeCheck_8",
        uses: 4,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9",
        uses: 16,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_B",
        uses: 16,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_C",
        uses: 16,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_D",
        uses: 16,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_E",
        uses: 16,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_F",
        uses: 16,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_G",
        uses: 8,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_H",
        uses: 8,
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
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 157];
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
        let M31_1 = E::F::from(M31::from(1));
        let M31_1073741824 = E::F::from(M31::from(1073741824));
        let M31_183619546 = E::F::from(M31::from(183619546));
        let M31_26 = E::F::from(M31::from(26));
        let input_chain_id_col0 = eval.next_trace_mask();
        let input_round_num_col1 = eval.next_trace_mask();
        let input_m_limb_0_col2 = eval.next_trace_mask();
        let input_m_limb_1_col3 = eval.next_trace_mask();
        let input_m_limb_2_col4 = eval.next_trace_mask();
        let input_m_limb_3_col5 = eval.next_trace_mask();
        let input_m_limb_4_col6 = eval.next_trace_mask();
        let input_m_limb_5_col7 = eval.next_trace_mask();
        let input_m_limb_6_col8 = eval.next_trace_mask();
        let input_m_limb_7_col9 = eval.next_trace_mask();
        let input_m_limb_8_col10 = eval.next_trace_mask();
        let input_m_limb_9_col11 = eval.next_trace_mask();
        let input_q_x_limb_0_col12 = eval.next_trace_mask();
        let input_q_x_limb_1_col13 = eval.next_trace_mask();
        let input_q_x_limb_2_col14 = eval.next_trace_mask();
        let input_q_x_limb_3_col15 = eval.next_trace_mask();
        let input_q_x_limb_4_col16 = eval.next_trace_mask();
        let input_q_x_limb_5_col17 = eval.next_trace_mask();
        let input_q_x_limb_6_col18 = eval.next_trace_mask();
        let input_q_x_limb_7_col19 = eval.next_trace_mask();
        let input_q_x_limb_8_col20 = eval.next_trace_mask();
        let input_q_x_limb_9_col21 = eval.next_trace_mask();
        let input_q_x_limb_10_col22 = eval.next_trace_mask();
        let input_q_x_limb_11_col23 = eval.next_trace_mask();
        let input_q_x_limb_12_col24 = eval.next_trace_mask();
        let input_q_x_limb_13_col25 = eval.next_trace_mask();
        let input_q_x_limb_14_col26 = eval.next_trace_mask();
        let input_q_x_limb_15_col27 = eval.next_trace_mask();
        let input_q_x_limb_16_col28 = eval.next_trace_mask();
        let input_q_x_limb_17_col29 = eval.next_trace_mask();
        let input_q_x_limb_18_col30 = eval.next_trace_mask();
        let input_q_x_limb_19_col31 = eval.next_trace_mask();
        let input_q_x_limb_20_col32 = eval.next_trace_mask();
        let input_q_x_limb_21_col33 = eval.next_trace_mask();
        let input_q_x_limb_22_col34 = eval.next_trace_mask();
        let input_q_x_limb_23_col35 = eval.next_trace_mask();
        let input_q_x_limb_24_col36 = eval.next_trace_mask();
        let input_q_x_limb_25_col37 = eval.next_trace_mask();
        let input_q_x_limb_26_col38 = eval.next_trace_mask();
        let input_q_x_limb_27_col39 = eval.next_trace_mask();
        let input_q_y_limb_0_col40 = eval.next_trace_mask();
        let input_q_y_limb_1_col41 = eval.next_trace_mask();
        let input_q_y_limb_2_col42 = eval.next_trace_mask();
        let input_q_y_limb_3_col43 = eval.next_trace_mask();
        let input_q_y_limb_4_col44 = eval.next_trace_mask();
        let input_q_y_limb_5_col45 = eval.next_trace_mask();
        let input_q_y_limb_6_col46 = eval.next_trace_mask();
        let input_q_y_limb_7_col47 = eval.next_trace_mask();
        let input_q_y_limb_8_col48 = eval.next_trace_mask();
        let input_q_y_limb_9_col49 = eval.next_trace_mask();
        let input_q_y_limb_10_col50 = eval.next_trace_mask();
        let input_q_y_limb_11_col51 = eval.next_trace_mask();
        let input_q_y_limb_12_col52 = eval.next_trace_mask();
        let input_q_y_limb_13_col53 = eval.next_trace_mask();
        let input_q_y_limb_14_col54 = eval.next_trace_mask();
        let input_q_y_limb_15_col55 = eval.next_trace_mask();
        let input_q_y_limb_16_col56 = eval.next_trace_mask();
        let input_q_y_limb_17_col57 = eval.next_trace_mask();
        let input_q_y_limb_18_col58 = eval.next_trace_mask();
        let input_q_y_limb_19_col59 = eval.next_trace_mask();
        let input_q_y_limb_20_col60 = eval.next_trace_mask();
        let input_q_y_limb_21_col61 = eval.next_trace_mask();
        let input_q_y_limb_22_col62 = eval.next_trace_mask();
        let input_q_y_limb_23_col63 = eval.next_trace_mask();
        let input_q_y_limb_24_col64 = eval.next_trace_mask();
        let input_q_y_limb_25_col65 = eval.next_trace_mask();
        let input_q_y_limb_26_col66 = eval.next_trace_mask();
        let input_q_y_limb_27_col67 = eval.next_trace_mask();
        let input_accumulator_x_limb_0_col68 = eval.next_trace_mask();
        let input_accumulator_x_limb_1_col69 = eval.next_trace_mask();
        let input_accumulator_x_limb_2_col70 = eval.next_trace_mask();
        let input_accumulator_x_limb_3_col71 = eval.next_trace_mask();
        let input_accumulator_x_limb_4_col72 = eval.next_trace_mask();
        let input_accumulator_x_limb_5_col73 = eval.next_trace_mask();
        let input_accumulator_x_limb_6_col74 = eval.next_trace_mask();
        let input_accumulator_x_limb_7_col75 = eval.next_trace_mask();
        let input_accumulator_x_limb_8_col76 = eval.next_trace_mask();
        let input_accumulator_x_limb_9_col77 = eval.next_trace_mask();
        let input_accumulator_x_limb_10_col78 = eval.next_trace_mask();
        let input_accumulator_x_limb_11_col79 = eval.next_trace_mask();
        let input_accumulator_x_limb_12_col80 = eval.next_trace_mask();
        let input_accumulator_x_limb_13_col81 = eval.next_trace_mask();
        let input_accumulator_x_limb_14_col82 = eval.next_trace_mask();
        let input_accumulator_x_limb_15_col83 = eval.next_trace_mask();
        let input_accumulator_x_limb_16_col84 = eval.next_trace_mask();
        let input_accumulator_x_limb_17_col85 = eval.next_trace_mask();
        let input_accumulator_x_limb_18_col86 = eval.next_trace_mask();
        let input_accumulator_x_limb_19_col87 = eval.next_trace_mask();
        let input_accumulator_x_limb_20_col88 = eval.next_trace_mask();
        let input_accumulator_x_limb_21_col89 = eval.next_trace_mask();
        let input_accumulator_x_limb_22_col90 = eval.next_trace_mask();
        let input_accumulator_x_limb_23_col91 = eval.next_trace_mask();
        let input_accumulator_x_limb_24_col92 = eval.next_trace_mask();
        let input_accumulator_x_limb_25_col93 = eval.next_trace_mask();
        let input_accumulator_x_limb_26_col94 = eval.next_trace_mask();
        let input_accumulator_x_limb_27_col95 = eval.next_trace_mask();
        let input_accumulator_y_limb_0_col96 = eval.next_trace_mask();
        let input_accumulator_y_limb_1_col97 = eval.next_trace_mask();
        let input_accumulator_y_limb_2_col98 = eval.next_trace_mask();
        let input_accumulator_y_limb_3_col99 = eval.next_trace_mask();
        let input_accumulator_y_limb_4_col100 = eval.next_trace_mask();
        let input_accumulator_y_limb_5_col101 = eval.next_trace_mask();
        let input_accumulator_y_limb_6_col102 = eval.next_trace_mask();
        let input_accumulator_y_limb_7_col103 = eval.next_trace_mask();
        let input_accumulator_y_limb_8_col104 = eval.next_trace_mask();
        let input_accumulator_y_limb_9_col105 = eval.next_trace_mask();
        let input_accumulator_y_limb_10_col106 = eval.next_trace_mask();
        let input_accumulator_y_limb_11_col107 = eval.next_trace_mask();
        let input_accumulator_y_limb_12_col108 = eval.next_trace_mask();
        let input_accumulator_y_limb_13_col109 = eval.next_trace_mask();
        let input_accumulator_y_limb_14_col110 = eval.next_trace_mask();
        let input_accumulator_y_limb_15_col111 = eval.next_trace_mask();
        let input_accumulator_y_limb_16_col112 = eval.next_trace_mask();
        let input_accumulator_y_limb_17_col113 = eval.next_trace_mask();
        let input_accumulator_y_limb_18_col114 = eval.next_trace_mask();
        let input_accumulator_y_limb_19_col115 = eval.next_trace_mask();
        let input_accumulator_y_limb_20_col116 = eval.next_trace_mask();
        let input_accumulator_y_limb_21_col117 = eval.next_trace_mask();
        let input_accumulator_y_limb_22_col118 = eval.next_trace_mask();
        let input_accumulator_y_limb_23_col119 = eval.next_trace_mask();
        let input_accumulator_y_limb_24_col120 = eval.next_trace_mask();
        let input_accumulator_y_limb_25_col121 = eval.next_trace_mask();
        let input_accumulator_y_limb_26_col122 = eval.next_trace_mask();
        let input_accumulator_y_limb_27_col123 = eval.next_trace_mask();
        let input_counter_col124 = eval.next_trace_mask();
        let to_add_bit_col125 = eval.next_trace_mask();
        let is_special_round_col126 = eval.next_trace_mask();
        let counter_inverse_col127 = eval.next_trace_mask();
        let next_m_0_col128 = eval.next_trace_mask();
        let next_m_1_col129 = eval.next_trace_mask();
        let next_m_2_col130 = eval.next_trace_mask();
        let next_m_3_col131 = eval.next_trace_mask();
        let next_m_4_col132 = eval.next_trace_mask();
        let next_m_5_col133 = eval.next_trace_mask();
        let next_m_6_col134 = eval.next_trace_mask();
        let next_m_7_col135 = eval.next_trace_mask();
        let next_m_8_col136 = eval.next_trace_mask();
        let next_m_9_col137 = eval.next_trace_mask();
        let next_counter_col138 = eval.next_trace_mask();
        let ms_limb_is_max_col139 = eval.next_trace_mask();
        let ms_and_mid_limbs_are_max_col140 = eval.next_trace_mask();
        let rc_input_col141 = eval.next_trace_mask();
        let ms_limb_is_max_col142 = eval.next_trace_mask();
        let ms_and_mid_limbs_are_max_col143 = eval.next_trace_mask();
        let rc_input_col144 = eval.next_trace_mask();
        let diff_sum_squares_inv_col145 = eval.next_trace_mask();
        let slope_limb_0_col146 = eval.next_trace_mask();
        let slope_limb_1_col147 = eval.next_trace_mask();
        let slope_limb_2_col148 = eval.next_trace_mask();
        let slope_limb_3_col149 = eval.next_trace_mask();
        let slope_limb_4_col150 = eval.next_trace_mask();
        let slope_limb_5_col151 = eval.next_trace_mask();
        let slope_limb_6_col152 = eval.next_trace_mask();
        let slope_limb_7_col153 = eval.next_trace_mask();
        let slope_limb_8_col154 = eval.next_trace_mask();
        let slope_limb_9_col155 = eval.next_trace_mask();
        let slope_limb_10_col156 = eval.next_trace_mask();
        let slope_limb_11_col157 = eval.next_trace_mask();
        let slope_limb_12_col158 = eval.next_trace_mask();
        let slope_limb_13_col159 = eval.next_trace_mask();
        let slope_limb_14_col160 = eval.next_trace_mask();
        let slope_limb_15_col161 = eval.next_trace_mask();
        let slope_limb_16_col162 = eval.next_trace_mask();
        let slope_limb_17_col163 = eval.next_trace_mask();
        let slope_limb_18_col164 = eval.next_trace_mask();
        let slope_limb_19_col165 = eval.next_trace_mask();
        let slope_limb_20_col166 = eval.next_trace_mask();
        let slope_limb_21_col167 = eval.next_trace_mask();
        let slope_limb_22_col168 = eval.next_trace_mask();
        let slope_limb_23_col169 = eval.next_trace_mask();
        let slope_limb_24_col170 = eval.next_trace_mask();
        let slope_limb_25_col171 = eval.next_trace_mask();
        let slope_limb_26_col172 = eval.next_trace_mask();
        let slope_limb_27_col173 = eval.next_trace_mask();
        let k_col174 = eval.next_trace_mask();
        let carry_0_col175 = eval.next_trace_mask();
        let carry_1_col176 = eval.next_trace_mask();
        let carry_2_col177 = eval.next_trace_mask();
        let carry_3_col178 = eval.next_trace_mask();
        let carry_4_col179 = eval.next_trace_mask();
        let carry_5_col180 = eval.next_trace_mask();
        let carry_6_col181 = eval.next_trace_mask();
        let carry_7_col182 = eval.next_trace_mask();
        let carry_8_col183 = eval.next_trace_mask();
        let carry_9_col184 = eval.next_trace_mask();
        let carry_10_col185 = eval.next_trace_mask();
        let carry_11_col186 = eval.next_trace_mask();
        let carry_12_col187 = eval.next_trace_mask();
        let carry_13_col188 = eval.next_trace_mask();
        let carry_14_col189 = eval.next_trace_mask();
        let carry_15_col190 = eval.next_trace_mask();
        let carry_16_col191 = eval.next_trace_mask();
        let carry_17_col192 = eval.next_trace_mask();
        let carry_18_col193 = eval.next_trace_mask();
        let carry_19_col194 = eval.next_trace_mask();
        let carry_20_col195 = eval.next_trace_mask();
        let carry_21_col196 = eval.next_trace_mask();
        let carry_22_col197 = eval.next_trace_mask();
        let carry_23_col198 = eval.next_trace_mask();
        let carry_24_col199 = eval.next_trace_mask();
        let carry_25_col200 = eval.next_trace_mask();
        let carry_26_col201 = eval.next_trace_mask();
        let result_x_limb_0_col202 = eval.next_trace_mask();
        let result_x_limb_1_col203 = eval.next_trace_mask();
        let result_x_limb_2_col204 = eval.next_trace_mask();
        let result_x_limb_3_col205 = eval.next_trace_mask();
        let result_x_limb_4_col206 = eval.next_trace_mask();
        let result_x_limb_5_col207 = eval.next_trace_mask();
        let result_x_limb_6_col208 = eval.next_trace_mask();
        let result_x_limb_7_col209 = eval.next_trace_mask();
        let result_x_limb_8_col210 = eval.next_trace_mask();
        let result_x_limb_9_col211 = eval.next_trace_mask();
        let result_x_limb_10_col212 = eval.next_trace_mask();
        let result_x_limb_11_col213 = eval.next_trace_mask();
        let result_x_limb_12_col214 = eval.next_trace_mask();
        let result_x_limb_13_col215 = eval.next_trace_mask();
        let result_x_limb_14_col216 = eval.next_trace_mask();
        let result_x_limb_15_col217 = eval.next_trace_mask();
        let result_x_limb_16_col218 = eval.next_trace_mask();
        let result_x_limb_17_col219 = eval.next_trace_mask();
        let result_x_limb_18_col220 = eval.next_trace_mask();
        let result_x_limb_19_col221 = eval.next_trace_mask();
        let result_x_limb_20_col222 = eval.next_trace_mask();
        let result_x_limb_21_col223 = eval.next_trace_mask();
        let result_x_limb_22_col224 = eval.next_trace_mask();
        let result_x_limb_23_col225 = eval.next_trace_mask();
        let result_x_limb_24_col226 = eval.next_trace_mask();
        let result_x_limb_25_col227 = eval.next_trace_mask();
        let result_x_limb_26_col228 = eval.next_trace_mask();
        let result_x_limb_27_col229 = eval.next_trace_mask();
        let k_col230 = eval.next_trace_mask();
        let carry_0_col231 = eval.next_trace_mask();
        let carry_1_col232 = eval.next_trace_mask();
        let carry_2_col233 = eval.next_trace_mask();
        let carry_3_col234 = eval.next_trace_mask();
        let carry_4_col235 = eval.next_trace_mask();
        let carry_5_col236 = eval.next_trace_mask();
        let carry_6_col237 = eval.next_trace_mask();
        let carry_7_col238 = eval.next_trace_mask();
        let carry_8_col239 = eval.next_trace_mask();
        let carry_9_col240 = eval.next_trace_mask();
        let carry_10_col241 = eval.next_trace_mask();
        let carry_11_col242 = eval.next_trace_mask();
        let carry_12_col243 = eval.next_trace_mask();
        let carry_13_col244 = eval.next_trace_mask();
        let carry_14_col245 = eval.next_trace_mask();
        let carry_15_col246 = eval.next_trace_mask();
        let carry_16_col247 = eval.next_trace_mask();
        let carry_17_col248 = eval.next_trace_mask();
        let carry_18_col249 = eval.next_trace_mask();
        let carry_19_col250 = eval.next_trace_mask();
        let carry_20_col251 = eval.next_trace_mask();
        let carry_21_col252 = eval.next_trace_mask();
        let carry_22_col253 = eval.next_trace_mask();
        let carry_23_col254 = eval.next_trace_mask();
        let carry_24_col255 = eval.next_trace_mask();
        let carry_25_col256 = eval.next_trace_mask();
        let carry_26_col257 = eval.next_trace_mask();
        let result_y_limb_0_col258 = eval.next_trace_mask();
        let result_y_limb_1_col259 = eval.next_trace_mask();
        let result_y_limb_2_col260 = eval.next_trace_mask();
        let result_y_limb_3_col261 = eval.next_trace_mask();
        let result_y_limb_4_col262 = eval.next_trace_mask();
        let result_y_limb_5_col263 = eval.next_trace_mask();
        let result_y_limb_6_col264 = eval.next_trace_mask();
        let result_y_limb_7_col265 = eval.next_trace_mask();
        let result_y_limb_8_col266 = eval.next_trace_mask();
        let result_y_limb_9_col267 = eval.next_trace_mask();
        let result_y_limb_10_col268 = eval.next_trace_mask();
        let result_y_limb_11_col269 = eval.next_trace_mask();
        let result_y_limb_12_col270 = eval.next_trace_mask();
        let result_y_limb_13_col271 = eval.next_trace_mask();
        let result_y_limb_14_col272 = eval.next_trace_mask();
        let result_y_limb_15_col273 = eval.next_trace_mask();
        let result_y_limb_16_col274 = eval.next_trace_mask();
        let result_y_limb_17_col275 = eval.next_trace_mask();
        let result_y_limb_18_col276 = eval.next_trace_mask();
        let result_y_limb_19_col277 = eval.next_trace_mask();
        let result_y_limb_20_col278 = eval.next_trace_mask();
        let result_y_limb_21_col279 = eval.next_trace_mask();
        let result_y_limb_22_col280 = eval.next_trace_mask();
        let result_y_limb_23_col281 = eval.next_trace_mask();
        let result_y_limb_24_col282 = eval.next_trace_mask();
        let result_y_limb_25_col283 = eval.next_trace_mask();
        let result_y_limb_26_col284 = eval.next_trace_mask();
        let result_y_limb_27_col285 = eval.next_trace_mask();
        let k_col286 = eval.next_trace_mask();
        let carry_0_col287 = eval.next_trace_mask();
        let carry_1_col288 = eval.next_trace_mask();
        let carry_2_col289 = eval.next_trace_mask();
        let carry_3_col290 = eval.next_trace_mask();
        let carry_4_col291 = eval.next_trace_mask();
        let carry_5_col292 = eval.next_trace_mask();
        let carry_6_col293 = eval.next_trace_mask();
        let carry_7_col294 = eval.next_trace_mask();
        let carry_8_col295 = eval.next_trace_mask();
        let carry_9_col296 = eval.next_trace_mask();
        let carry_10_col297 = eval.next_trace_mask();
        let carry_11_col298 = eval.next_trace_mask();
        let carry_12_col299 = eval.next_trace_mask();
        let carry_13_col300 = eval.next_trace_mask();
        let carry_14_col301 = eval.next_trace_mask();
        let carry_15_col302 = eval.next_trace_mask();
        let carry_16_col303 = eval.next_trace_mask();
        let carry_17_col304 = eval.next_trace_mask();
        let carry_18_col305 = eval.next_trace_mask();
        let carry_19_col306 = eval.next_trace_mask();
        let carry_20_col307 = eval.next_trace_mask();
        let carry_21_col308 = eval.next_trace_mask();
        let carry_22_col309 = eval.next_trace_mask();
        let carry_23_col310 = eval.next_trace_mask();
        let carry_24_col311 = eval.next_trace_mask();
        let carry_25_col312 = eval.next_trace_mask();
        let carry_26_col313 = eval.next_trace_mask();
        let new_acculumator_0_0_col314 = eval.next_trace_mask();
        let new_acculumator_0_1_col315 = eval.next_trace_mask();
        let new_acculumator_0_2_col316 = eval.next_trace_mask();
        let new_acculumator_0_3_col317 = eval.next_trace_mask();
        let new_acculumator_0_4_col318 = eval.next_trace_mask();
        let new_acculumator_0_5_col319 = eval.next_trace_mask();
        let new_acculumator_0_6_col320 = eval.next_trace_mask();
        let new_acculumator_0_7_col321 = eval.next_trace_mask();
        let new_acculumator_0_8_col322 = eval.next_trace_mask();
        let new_acculumator_0_9_col323 = eval.next_trace_mask();
        let new_acculumator_0_10_col324 = eval.next_trace_mask();
        let new_acculumator_0_11_col325 = eval.next_trace_mask();
        let new_acculumator_0_12_col326 = eval.next_trace_mask();
        let new_acculumator_0_13_col327 = eval.next_trace_mask();
        let new_acculumator_0_14_col328 = eval.next_trace_mask();
        let new_acculumator_0_15_col329 = eval.next_trace_mask();
        let new_acculumator_0_16_col330 = eval.next_trace_mask();
        let new_acculumator_0_17_col331 = eval.next_trace_mask();
        let new_acculumator_0_18_col332 = eval.next_trace_mask();
        let new_acculumator_0_19_col333 = eval.next_trace_mask();
        let new_acculumator_0_20_col334 = eval.next_trace_mask();
        let new_acculumator_0_21_col335 = eval.next_trace_mask();
        let new_acculumator_0_22_col336 = eval.next_trace_mask();
        let new_acculumator_0_23_col337 = eval.next_trace_mask();
        let new_acculumator_0_24_col338 = eval.next_trace_mask();
        let new_acculumator_0_25_col339 = eval.next_trace_mask();
        let new_acculumator_0_26_col340 = eval.next_trace_mask();
        let new_acculumator_0_27_col341 = eval.next_trace_mask();
        let new_acculumator_1_0_col342 = eval.next_trace_mask();
        let new_acculumator_1_1_col343 = eval.next_trace_mask();
        let new_acculumator_1_2_col344 = eval.next_trace_mask();
        let new_acculumator_1_3_col345 = eval.next_trace_mask();
        let new_acculumator_1_4_col346 = eval.next_trace_mask();
        let new_acculumator_1_5_col347 = eval.next_trace_mask();
        let new_acculumator_1_6_col348 = eval.next_trace_mask();
        let new_acculumator_1_7_col349 = eval.next_trace_mask();
        let new_acculumator_1_8_col350 = eval.next_trace_mask();
        let new_acculumator_1_9_col351 = eval.next_trace_mask();
        let new_acculumator_1_10_col352 = eval.next_trace_mask();
        let new_acculumator_1_11_col353 = eval.next_trace_mask();
        let new_acculumator_1_12_col354 = eval.next_trace_mask();
        let new_acculumator_1_13_col355 = eval.next_trace_mask();
        let new_acculumator_1_14_col356 = eval.next_trace_mask();
        let new_acculumator_1_15_col357 = eval.next_trace_mask();
        let new_acculumator_1_16_col358 = eval.next_trace_mask();
        let new_acculumator_1_17_col359 = eval.next_trace_mask();
        let new_acculumator_1_18_col360 = eval.next_trace_mask();
        let new_acculumator_1_19_col361 = eval.next_trace_mask();
        let new_acculumator_1_20_col362 = eval.next_trace_mask();
        let new_acculumator_1_21_col363 = eval.next_trace_mask();
        let new_acculumator_1_22_col364 = eval.next_trace_mask();
        let new_acculumator_1_23_col365 = eval.next_trace_mask();
        let new_acculumator_1_24_col366 = eval.next_trace_mask();
        let new_acculumator_1_25_col367 = eval.next_trace_mask();
        let new_acculumator_1_26_col368 = eval.next_trace_mask();
        let new_acculumator_1_27_col369 = eval.next_trace_mask();
        let mul_res_limb_0_col370 = eval.next_trace_mask();
        let mul_res_limb_1_col371 = eval.next_trace_mask();
        let mul_res_limb_2_col372 = eval.next_trace_mask();
        let mul_res_limb_3_col373 = eval.next_trace_mask();
        let mul_res_limb_4_col374 = eval.next_trace_mask();
        let mul_res_limb_5_col375 = eval.next_trace_mask();
        let mul_res_limb_6_col376 = eval.next_trace_mask();
        let mul_res_limb_7_col377 = eval.next_trace_mask();
        let mul_res_limb_8_col378 = eval.next_trace_mask();
        let mul_res_limb_9_col379 = eval.next_trace_mask();
        let mul_res_limb_10_col380 = eval.next_trace_mask();
        let mul_res_limb_11_col381 = eval.next_trace_mask();
        let mul_res_limb_12_col382 = eval.next_trace_mask();
        let mul_res_limb_13_col383 = eval.next_trace_mask();
        let mul_res_limb_14_col384 = eval.next_trace_mask();
        let mul_res_limb_15_col385 = eval.next_trace_mask();
        let mul_res_limb_16_col386 = eval.next_trace_mask();
        let mul_res_limb_17_col387 = eval.next_trace_mask();
        let mul_res_limb_18_col388 = eval.next_trace_mask();
        let mul_res_limb_19_col389 = eval.next_trace_mask();
        let mul_res_limb_20_col390 = eval.next_trace_mask();
        let mul_res_limb_21_col391 = eval.next_trace_mask();
        let mul_res_limb_22_col392 = eval.next_trace_mask();
        let mul_res_limb_23_col393 = eval.next_trace_mask();
        let mul_res_limb_24_col394 = eval.next_trace_mask();
        let mul_res_limb_25_col395 = eval.next_trace_mask();
        let mul_res_limb_26_col396 = eval.next_trace_mask();
        let mul_res_limb_27_col397 = eval.next_trace_mask();
        let k_col398 = eval.next_trace_mask();
        let carry_0_col399 = eval.next_trace_mask();
        let carry_1_col400 = eval.next_trace_mask();
        let carry_2_col401 = eval.next_trace_mask();
        let carry_3_col402 = eval.next_trace_mask();
        let carry_4_col403 = eval.next_trace_mask();
        let carry_5_col404 = eval.next_trace_mask();
        let carry_6_col405 = eval.next_trace_mask();
        let carry_7_col406 = eval.next_trace_mask();
        let carry_8_col407 = eval.next_trace_mask();
        let carry_9_col408 = eval.next_trace_mask();
        let carry_10_col409 = eval.next_trace_mask();
        let carry_11_col410 = eval.next_trace_mask();
        let carry_12_col411 = eval.next_trace_mask();
        let carry_13_col412 = eval.next_trace_mask();
        let carry_14_col413 = eval.next_trace_mask();
        let carry_15_col414 = eval.next_trace_mask();
        let carry_16_col415 = eval.next_trace_mask();
        let carry_17_col416 = eval.next_trace_mask();
        let carry_18_col417 = eval.next_trace_mask();
        let carry_19_col418 = eval.next_trace_mask();
        let carry_20_col419 = eval.next_trace_mask();
        let carry_21_col420 = eval.next_trace_mask();
        let carry_22_col421 = eval.next_trace_mask();
        let carry_23_col422 = eval.next_trace_mask();
        let carry_24_col423 = eval.next_trace_mask();
        let carry_25_col424 = eval.next_trace_mask();
        let carry_26_col425 = eval.next_trace_mask();
        let add_res_limb_0_col426 = eval.next_trace_mask();
        let add_res_limb_1_col427 = eval.next_trace_mask();
        let add_res_limb_2_col428 = eval.next_trace_mask();
        let add_res_limb_3_col429 = eval.next_trace_mask();
        let add_res_limb_4_col430 = eval.next_trace_mask();
        let add_res_limb_5_col431 = eval.next_trace_mask();
        let add_res_limb_6_col432 = eval.next_trace_mask();
        let add_res_limb_7_col433 = eval.next_trace_mask();
        let add_res_limb_8_col434 = eval.next_trace_mask();
        let add_res_limb_9_col435 = eval.next_trace_mask();
        let add_res_limb_10_col436 = eval.next_trace_mask();
        let add_res_limb_11_col437 = eval.next_trace_mask();
        let add_res_limb_12_col438 = eval.next_trace_mask();
        let add_res_limb_13_col439 = eval.next_trace_mask();
        let add_res_limb_14_col440 = eval.next_trace_mask();
        let add_res_limb_15_col441 = eval.next_trace_mask();
        let add_res_limb_16_col442 = eval.next_trace_mask();
        let add_res_limb_17_col443 = eval.next_trace_mask();
        let add_res_limb_18_col444 = eval.next_trace_mask();
        let add_res_limb_19_col445 = eval.next_trace_mask();
        let add_res_limb_20_col446 = eval.next_trace_mask();
        let add_res_limb_21_col447 = eval.next_trace_mask();
        let add_res_limb_22_col448 = eval.next_trace_mask();
        let add_res_limb_23_col449 = eval.next_trace_mask();
        let add_res_limb_24_col450 = eval.next_trace_mask();
        let add_res_limb_25_col451 = eval.next_trace_mask();
        let add_res_limb_26_col452 = eval.next_trace_mask();
        let add_res_limb_27_col453 = eval.next_trace_mask();
        let sub_p_bit_col454 = eval.next_trace_mask();
        let slope_limb_0_col455 = eval.next_trace_mask();
        let slope_limb_1_col456 = eval.next_trace_mask();
        let slope_limb_2_col457 = eval.next_trace_mask();
        let slope_limb_3_col458 = eval.next_trace_mask();
        let slope_limb_4_col459 = eval.next_trace_mask();
        let slope_limb_5_col460 = eval.next_trace_mask();
        let slope_limb_6_col461 = eval.next_trace_mask();
        let slope_limb_7_col462 = eval.next_trace_mask();
        let slope_limb_8_col463 = eval.next_trace_mask();
        let slope_limb_9_col464 = eval.next_trace_mask();
        let slope_limb_10_col465 = eval.next_trace_mask();
        let slope_limb_11_col466 = eval.next_trace_mask();
        let slope_limb_12_col467 = eval.next_trace_mask();
        let slope_limb_13_col468 = eval.next_trace_mask();
        let slope_limb_14_col469 = eval.next_trace_mask();
        let slope_limb_15_col470 = eval.next_trace_mask();
        let slope_limb_16_col471 = eval.next_trace_mask();
        let slope_limb_17_col472 = eval.next_trace_mask();
        let slope_limb_18_col473 = eval.next_trace_mask();
        let slope_limb_19_col474 = eval.next_trace_mask();
        let slope_limb_20_col475 = eval.next_trace_mask();
        let slope_limb_21_col476 = eval.next_trace_mask();
        let slope_limb_22_col477 = eval.next_trace_mask();
        let slope_limb_23_col478 = eval.next_trace_mask();
        let slope_limb_24_col479 = eval.next_trace_mask();
        let slope_limb_25_col480 = eval.next_trace_mask();
        let slope_limb_26_col481 = eval.next_trace_mask();
        let slope_limb_27_col482 = eval.next_trace_mask();
        let k_col483 = eval.next_trace_mask();
        let carry_0_col484 = eval.next_trace_mask();
        let carry_1_col485 = eval.next_trace_mask();
        let carry_2_col486 = eval.next_trace_mask();
        let carry_3_col487 = eval.next_trace_mask();
        let carry_4_col488 = eval.next_trace_mask();
        let carry_5_col489 = eval.next_trace_mask();
        let carry_6_col490 = eval.next_trace_mask();
        let carry_7_col491 = eval.next_trace_mask();
        let carry_8_col492 = eval.next_trace_mask();
        let carry_9_col493 = eval.next_trace_mask();
        let carry_10_col494 = eval.next_trace_mask();
        let carry_11_col495 = eval.next_trace_mask();
        let carry_12_col496 = eval.next_trace_mask();
        let carry_13_col497 = eval.next_trace_mask();
        let carry_14_col498 = eval.next_trace_mask();
        let carry_15_col499 = eval.next_trace_mask();
        let carry_16_col500 = eval.next_trace_mask();
        let carry_17_col501 = eval.next_trace_mask();
        let carry_18_col502 = eval.next_trace_mask();
        let carry_19_col503 = eval.next_trace_mask();
        let carry_20_col504 = eval.next_trace_mask();
        let carry_21_col505 = eval.next_trace_mask();
        let carry_22_col506 = eval.next_trace_mask();
        let carry_23_col507 = eval.next_trace_mask();
        let carry_24_col508 = eval.next_trace_mask();
        let carry_25_col509 = eval.next_trace_mask();
        let carry_26_col510 = eval.next_trace_mask();
        let result_x_limb_0_col511 = eval.next_trace_mask();
        let result_x_limb_1_col512 = eval.next_trace_mask();
        let result_x_limb_2_col513 = eval.next_trace_mask();
        let result_x_limb_3_col514 = eval.next_trace_mask();
        let result_x_limb_4_col515 = eval.next_trace_mask();
        let result_x_limb_5_col516 = eval.next_trace_mask();
        let result_x_limb_6_col517 = eval.next_trace_mask();
        let result_x_limb_7_col518 = eval.next_trace_mask();
        let result_x_limb_8_col519 = eval.next_trace_mask();
        let result_x_limb_9_col520 = eval.next_trace_mask();
        let result_x_limb_10_col521 = eval.next_trace_mask();
        let result_x_limb_11_col522 = eval.next_trace_mask();
        let result_x_limb_12_col523 = eval.next_trace_mask();
        let result_x_limb_13_col524 = eval.next_trace_mask();
        let result_x_limb_14_col525 = eval.next_trace_mask();
        let result_x_limb_15_col526 = eval.next_trace_mask();
        let result_x_limb_16_col527 = eval.next_trace_mask();
        let result_x_limb_17_col528 = eval.next_trace_mask();
        let result_x_limb_18_col529 = eval.next_trace_mask();
        let result_x_limb_19_col530 = eval.next_trace_mask();
        let result_x_limb_20_col531 = eval.next_trace_mask();
        let result_x_limb_21_col532 = eval.next_trace_mask();
        let result_x_limb_22_col533 = eval.next_trace_mask();
        let result_x_limb_23_col534 = eval.next_trace_mask();
        let result_x_limb_24_col535 = eval.next_trace_mask();
        let result_x_limb_25_col536 = eval.next_trace_mask();
        let result_x_limb_26_col537 = eval.next_trace_mask();
        let result_x_limb_27_col538 = eval.next_trace_mask();
        let k_col539 = eval.next_trace_mask();
        let carry_0_col540 = eval.next_trace_mask();
        let carry_1_col541 = eval.next_trace_mask();
        let carry_2_col542 = eval.next_trace_mask();
        let carry_3_col543 = eval.next_trace_mask();
        let carry_4_col544 = eval.next_trace_mask();
        let carry_5_col545 = eval.next_trace_mask();
        let carry_6_col546 = eval.next_trace_mask();
        let carry_7_col547 = eval.next_trace_mask();
        let carry_8_col548 = eval.next_trace_mask();
        let carry_9_col549 = eval.next_trace_mask();
        let carry_10_col550 = eval.next_trace_mask();
        let carry_11_col551 = eval.next_trace_mask();
        let carry_12_col552 = eval.next_trace_mask();
        let carry_13_col553 = eval.next_trace_mask();
        let carry_14_col554 = eval.next_trace_mask();
        let carry_15_col555 = eval.next_trace_mask();
        let carry_16_col556 = eval.next_trace_mask();
        let carry_17_col557 = eval.next_trace_mask();
        let carry_18_col558 = eval.next_trace_mask();
        let carry_19_col559 = eval.next_trace_mask();
        let carry_20_col560 = eval.next_trace_mask();
        let carry_21_col561 = eval.next_trace_mask();
        let carry_22_col562 = eval.next_trace_mask();
        let carry_23_col563 = eval.next_trace_mask();
        let carry_24_col564 = eval.next_trace_mask();
        let carry_25_col565 = eval.next_trace_mask();
        let carry_26_col566 = eval.next_trace_mask();
        let result_y_limb_0_col567 = eval.next_trace_mask();
        let result_y_limb_1_col568 = eval.next_trace_mask();
        let result_y_limb_2_col569 = eval.next_trace_mask();
        let result_y_limb_3_col570 = eval.next_trace_mask();
        let result_y_limb_4_col571 = eval.next_trace_mask();
        let result_y_limb_5_col572 = eval.next_trace_mask();
        let result_y_limb_6_col573 = eval.next_trace_mask();
        let result_y_limb_7_col574 = eval.next_trace_mask();
        let result_y_limb_8_col575 = eval.next_trace_mask();
        let result_y_limb_9_col576 = eval.next_trace_mask();
        let result_y_limb_10_col577 = eval.next_trace_mask();
        let result_y_limb_11_col578 = eval.next_trace_mask();
        let result_y_limb_12_col579 = eval.next_trace_mask();
        let result_y_limb_13_col580 = eval.next_trace_mask();
        let result_y_limb_14_col581 = eval.next_trace_mask();
        let result_y_limb_15_col582 = eval.next_trace_mask();
        let result_y_limb_16_col583 = eval.next_trace_mask();
        let result_y_limb_17_col584 = eval.next_trace_mask();
        let result_y_limb_18_col585 = eval.next_trace_mask();
        let result_y_limb_19_col586 = eval.next_trace_mask();
        let result_y_limb_20_col587 = eval.next_trace_mask();
        let result_y_limb_21_col588 = eval.next_trace_mask();
        let result_y_limb_22_col589 = eval.next_trace_mask();
        let result_y_limb_23_col590 = eval.next_trace_mask();
        let result_y_limb_24_col591 = eval.next_trace_mask();
        let result_y_limb_25_col592 = eval.next_trace_mask();
        let result_y_limb_26_col593 = eval.next_trace_mask();
        let result_y_limb_27_col594 = eval.next_trace_mask();
        let k_col595 = eval.next_trace_mask();
        let carry_0_col596 = eval.next_trace_mask();
        let carry_1_col597 = eval.next_trace_mask();
        let carry_2_col598 = eval.next_trace_mask();
        let carry_3_col599 = eval.next_trace_mask();
        let carry_4_col600 = eval.next_trace_mask();
        let carry_5_col601 = eval.next_trace_mask();
        let carry_6_col602 = eval.next_trace_mask();
        let carry_7_col603 = eval.next_trace_mask();
        let carry_8_col604 = eval.next_trace_mask();
        let carry_9_col605 = eval.next_trace_mask();
        let carry_10_col606 = eval.next_trace_mask();
        let carry_11_col607 = eval.next_trace_mask();
        let carry_12_col608 = eval.next_trace_mask();
        let carry_13_col609 = eval.next_trace_mask();
        let carry_14_col610 = eval.next_trace_mask();
        let carry_15_col611 = eval.next_trace_mask();
        let carry_16_col612 = eval.next_trace_mask();
        let carry_17_col613 = eval.next_trace_mask();
        let carry_18_col614 = eval.next_trace_mask();
        let carry_19_col615 = eval.next_trace_mask();
        let carry_20_col616 = eval.next_trace_mask();
        let carry_21_col617 = eval.next_trace_mask();
        let carry_22_col618 = eval.next_trace_mask();
        let carry_23_col619 = eval.next_trace_mask();
        let carry_24_col620 = eval.next_trace_mask();
        let carry_25_col621 = eval.next_trace_mask();
        let carry_26_col622 = eval.next_trace_mask();
        let enabler_col623 = eval.next_trace_mask();

        eval.add_constraint(
            ((enabler_col623.clone() * enabler_col623.clone()) - enabler_col623.clone()),
        );
        // to_add_bit is bool.
        eval.add_constraint(
            (to_add_bit_col125.clone() * (M31_1.clone() - to_add_bit_col125.clone())),
        );
        let not_is_special_round_tmp_7776f_5 =
            eval.add_intermediate((M31_1.clone() - is_special_round_col126.clone()));
        let counter_inverse_inverse_tmp_7776f_6 =
            eval.add_intermediate((input_counter_col124.clone() + is_special_round_col126.clone()));
        // is_special_round is bool.
        eval.add_constraint(
            (is_special_round_col126.clone() * not_is_special_round_tmp_7776f_5.clone()),
        );
        // is_special_round = (counter == 0).
        eval.add_constraint(
            ((input_counter_col124.clone() * counter_inverse_col127.clone())
                - not_is_special_round_tmp_7776f_5.clone()),
        );
        // counter_inverse != 0.
        eval.add_constraint(
            ((counter_inverse_col127.clone() * counter_inverse_inverse_tmp_7776f_6.clone())
                - M31_1.clone()),
        );
        let m0_minus_to_add_bit_tmp_7776f_8 =
            eval.add_intermediate((input_m_limb_0_col2.clone() - to_add_bit_col125.clone()));
        // m0 is exhausted at the end of special rounds.
        eval.add_constraint(
            (m0_minus_to_add_bit_tmp_7776f_8.clone() * is_special_round_col126.clone()),
        );
        // next_m_0.
        eval.add_constraint(
            (next_m_0_col128.clone()
                - ((((m0_minus_to_add_bit_tmp_7776f_8.clone() * M31_1073741824.clone())
                    - input_m_limb_1_col3.clone())
                    * not_is_special_round_tmp_7776f_5.clone())
                    + input_m_limb_1_col3.clone())),
        );
        // next_m_1.
        eval.add_constraint(
            (next_m_1_col129.clone()
                - (((input_m_limb_1_col3.clone() - input_m_limb_2_col4.clone())
                    * not_is_special_round_tmp_7776f_5.clone())
                    + input_m_limb_2_col4.clone())),
        );
        // next_m_2.
        eval.add_constraint(
            (next_m_2_col130.clone()
                - (((input_m_limb_2_col4.clone() - input_m_limb_3_col5.clone())
                    * not_is_special_round_tmp_7776f_5.clone())
                    + input_m_limb_3_col5.clone())),
        );
        // next_m_3.
        eval.add_constraint(
            (next_m_3_col131.clone()
                - (((input_m_limb_3_col5.clone() - input_m_limb_4_col6.clone())
                    * not_is_special_round_tmp_7776f_5.clone())
                    + input_m_limb_4_col6.clone())),
        );
        // next_m_4.
        eval.add_constraint(
            (next_m_4_col132.clone()
                - (((input_m_limb_4_col6.clone() - input_m_limb_5_col7.clone())
                    * not_is_special_round_tmp_7776f_5.clone())
                    + input_m_limb_5_col7.clone())),
        );
        // next_m_5.
        eval.add_constraint(
            (next_m_5_col133.clone()
                - (((input_m_limb_5_col7.clone() - input_m_limb_6_col8.clone())
                    * not_is_special_round_tmp_7776f_5.clone())
                    + input_m_limb_6_col8.clone())),
        );
        // next_m_6.
        eval.add_constraint(
            (next_m_6_col134.clone()
                - (((input_m_limb_6_col8.clone() - input_m_limb_7_col9.clone())
                    * not_is_special_round_tmp_7776f_5.clone())
                    + input_m_limb_7_col9.clone())),
        );
        // next_m_7.
        eval.add_constraint(
            (next_m_7_col135.clone()
                - (((input_m_limb_7_col9.clone() - input_m_limb_8_col10.clone())
                    * not_is_special_round_tmp_7776f_5.clone())
                    + input_m_limb_8_col10.clone())),
        );
        // next_m_8.
        eval.add_constraint(
            (next_m_8_col136.clone()
                - (((input_m_limb_8_col10.clone() - input_m_limb_9_col11.clone())
                    * not_is_special_round_tmp_7776f_5.clone())
                    + input_m_limb_9_col11.clone())),
        );
        // next_m_9.
        eval.add_constraint(
            (next_m_9_col137.clone()
                - (input_m_limb_9_col11.clone() * not_is_special_round_tmp_7776f_5.clone())),
        );
        // next_counter.
        eval.add_constraint(
            (next_counter_col138.clone()
                - ((((input_counter_col124.clone() - M31_1.clone()) - M31_26.clone())
                    * not_is_special_round_tmp_7776f_5.clone())
                    + M31_26.clone())),
        );
        VerifyReduced252::evaluate(
            [
                input_accumulator_x_limb_0_col68.clone(),
                input_accumulator_x_limb_1_col69.clone(),
                input_accumulator_x_limb_2_col70.clone(),
                input_accumulator_x_limb_3_col71.clone(),
                input_accumulator_x_limb_4_col72.clone(),
                input_accumulator_x_limb_5_col73.clone(),
                input_accumulator_x_limb_6_col74.clone(),
                input_accumulator_x_limb_7_col75.clone(),
                input_accumulator_x_limb_8_col76.clone(),
                input_accumulator_x_limb_9_col77.clone(),
                input_accumulator_x_limb_10_col78.clone(),
                input_accumulator_x_limb_11_col79.clone(),
                input_accumulator_x_limb_12_col80.clone(),
                input_accumulator_x_limb_13_col81.clone(),
                input_accumulator_x_limb_14_col82.clone(),
                input_accumulator_x_limb_15_col83.clone(),
                input_accumulator_x_limb_16_col84.clone(),
                input_accumulator_x_limb_17_col85.clone(),
                input_accumulator_x_limb_18_col86.clone(),
                input_accumulator_x_limb_19_col87.clone(),
                input_accumulator_x_limb_20_col88.clone(),
                input_accumulator_x_limb_21_col89.clone(),
                input_accumulator_x_limb_22_col90.clone(),
                input_accumulator_x_limb_23_col91.clone(),
                input_accumulator_x_limb_24_col92.clone(),
                input_accumulator_x_limb_25_col93.clone(),
                input_accumulator_x_limb_26_col94.clone(),
                input_accumulator_x_limb_27_col95.clone(),
            ],
            ms_limb_is_max_col139.clone(),
            ms_and_mid_limbs_are_max_col140.clone(),
            rc_input_col141.clone(),
            &self.common_lookup_elements,
            &mut eval,
        );
        VerifyReduced252::evaluate(
            [
                input_q_x_limb_0_col12.clone(),
                input_q_x_limb_1_col13.clone(),
                input_q_x_limb_2_col14.clone(),
                input_q_x_limb_3_col15.clone(),
                input_q_x_limb_4_col16.clone(),
                input_q_x_limb_5_col17.clone(),
                input_q_x_limb_6_col18.clone(),
                input_q_x_limb_7_col19.clone(),
                input_q_x_limb_8_col20.clone(),
                input_q_x_limb_9_col21.clone(),
                input_q_x_limb_10_col22.clone(),
                input_q_x_limb_11_col23.clone(),
                input_q_x_limb_12_col24.clone(),
                input_q_x_limb_13_col25.clone(),
                input_q_x_limb_14_col26.clone(),
                input_q_x_limb_15_col27.clone(),
                input_q_x_limb_16_col28.clone(),
                input_q_x_limb_17_col29.clone(),
                input_q_x_limb_18_col30.clone(),
                input_q_x_limb_19_col31.clone(),
                input_q_x_limb_20_col32.clone(),
                input_q_x_limb_21_col33.clone(),
                input_q_x_limb_22_col34.clone(),
                input_q_x_limb_23_col35.clone(),
                input_q_x_limb_24_col36.clone(),
                input_q_x_limb_25_col37.clone(),
                input_q_x_limb_26_col38.clone(),
                input_q_x_limb_27_col39.clone(),
            ],
            ms_limb_is_max_col142.clone(),
            ms_and_mid_limbs_are_max_col143.clone(),
            rc_input_col144.clone(),
            &self.common_lookup_elements,
            &mut eval,
        );
        let q_acc_diff_tmp_7776f_13 = eval.add_intermediate(
            (input_q_x_limb_0_col12.clone() - input_accumulator_x_limb_0_col68.clone()),
        );
        let q_acc_diff_tmp_7776f_14 = eval.add_intermediate(
            (input_q_x_limb_1_col13.clone() - input_accumulator_x_limb_1_col69.clone()),
        );
        let q_acc_diff_tmp_7776f_15 = eval.add_intermediate(
            (input_q_x_limb_2_col14.clone() - input_accumulator_x_limb_2_col70.clone()),
        );
        let q_acc_diff_tmp_7776f_16 = eval.add_intermediate(
            (input_q_x_limb_3_col15.clone() - input_accumulator_x_limb_3_col71.clone()),
        );
        let q_acc_diff_tmp_7776f_17 = eval.add_intermediate(
            (input_q_x_limb_4_col16.clone() - input_accumulator_x_limb_4_col72.clone()),
        );
        let q_acc_diff_tmp_7776f_18 = eval.add_intermediate(
            (input_q_x_limb_5_col17.clone() - input_accumulator_x_limb_5_col73.clone()),
        );
        let q_acc_diff_tmp_7776f_19 = eval.add_intermediate(
            (input_q_x_limb_6_col18.clone() - input_accumulator_x_limb_6_col74.clone()),
        );
        let q_acc_diff_tmp_7776f_20 = eval.add_intermediate(
            (input_q_x_limb_7_col19.clone() - input_accumulator_x_limb_7_col75.clone()),
        );
        let q_acc_diff_tmp_7776f_21 = eval.add_intermediate(
            (input_q_x_limb_8_col20.clone() - input_accumulator_x_limb_8_col76.clone()),
        );
        let q_acc_diff_tmp_7776f_22 = eval.add_intermediate(
            (input_q_x_limb_9_col21.clone() - input_accumulator_x_limb_9_col77.clone()),
        );
        let q_acc_diff_tmp_7776f_23 = eval.add_intermediate(
            (input_q_x_limb_10_col22.clone() - input_accumulator_x_limb_10_col78.clone()),
        );
        let q_acc_diff_tmp_7776f_24 = eval.add_intermediate(
            (input_q_x_limb_11_col23.clone() - input_accumulator_x_limb_11_col79.clone()),
        );
        let q_acc_diff_tmp_7776f_25 = eval.add_intermediate(
            (input_q_x_limb_12_col24.clone() - input_accumulator_x_limb_12_col80.clone()),
        );
        let q_acc_diff_tmp_7776f_26 = eval.add_intermediate(
            (input_q_x_limb_13_col25.clone() - input_accumulator_x_limb_13_col81.clone()),
        );
        let q_acc_diff_tmp_7776f_27 = eval.add_intermediate(
            (input_q_x_limb_14_col26.clone() - input_accumulator_x_limb_14_col82.clone()),
        );
        let q_acc_diff_tmp_7776f_28 = eval.add_intermediate(
            (input_q_x_limb_15_col27.clone() - input_accumulator_x_limb_15_col83.clone()),
        );
        let q_acc_diff_tmp_7776f_29 = eval.add_intermediate(
            (input_q_x_limb_16_col28.clone() - input_accumulator_x_limb_16_col84.clone()),
        );
        let q_acc_diff_tmp_7776f_30 = eval.add_intermediate(
            (input_q_x_limb_17_col29.clone() - input_accumulator_x_limb_17_col85.clone()),
        );
        let q_acc_diff_tmp_7776f_31 = eval.add_intermediate(
            (input_q_x_limb_18_col30.clone() - input_accumulator_x_limb_18_col86.clone()),
        );
        let q_acc_diff_tmp_7776f_32 = eval.add_intermediate(
            (input_q_x_limb_19_col31.clone() - input_accumulator_x_limb_19_col87.clone()),
        );
        let q_acc_diff_tmp_7776f_33 = eval.add_intermediate(
            (input_q_x_limb_20_col32.clone() - input_accumulator_x_limb_20_col88.clone()),
        );
        let q_acc_diff_tmp_7776f_34 = eval.add_intermediate(
            (input_q_x_limb_21_col33.clone() - input_accumulator_x_limb_21_col89.clone()),
        );
        let q_acc_diff_tmp_7776f_35 = eval.add_intermediate(
            (input_q_x_limb_22_col34.clone() - input_accumulator_x_limb_22_col90.clone()),
        );
        let q_acc_diff_tmp_7776f_36 = eval.add_intermediate(
            (input_q_x_limb_23_col35.clone() - input_accumulator_x_limb_23_col91.clone()),
        );
        let q_acc_diff_tmp_7776f_37 = eval.add_intermediate(
            (input_q_x_limb_24_col36.clone() - input_accumulator_x_limb_24_col92.clone()),
        );
        let q_acc_diff_tmp_7776f_38 = eval.add_intermediate(
            (input_q_x_limb_25_col37.clone() - input_accumulator_x_limb_25_col93.clone()),
        );
        let q_acc_diff_tmp_7776f_39 = eval.add_intermediate(
            (input_q_x_limb_26_col38.clone() - input_accumulator_x_limb_26_col94.clone()),
        );
        let q_acc_diff_tmp_7776f_40 = eval.add_intermediate(
            (input_q_x_limb_27_col39.clone() - input_accumulator_x_limb_27_col95.clone()),
        );
        // accumulator.x doesn't equal q.x.
        eval.add_constraint(
            ((((((((((((((((((((((((((((((q_acc_diff_tmp_7776f_13
                .clone()
                * q_acc_diff_tmp_7776f_13.clone())
                + (q_acc_diff_tmp_7776f_14.clone()
                    * q_acc_diff_tmp_7776f_14.clone()))
                + (q_acc_diff_tmp_7776f_15.clone()
                    * q_acc_diff_tmp_7776f_15.clone()))
                + (q_acc_diff_tmp_7776f_16.clone()
                    * q_acc_diff_tmp_7776f_16.clone()))
                + (q_acc_diff_tmp_7776f_17.clone()
                    * q_acc_diff_tmp_7776f_17.clone()))
                + (q_acc_diff_tmp_7776f_18.clone()
                    * q_acc_diff_tmp_7776f_18.clone()))
                + (q_acc_diff_tmp_7776f_19.clone()
                    * q_acc_diff_tmp_7776f_19.clone()))
                + (q_acc_diff_tmp_7776f_20.clone()
                    * q_acc_diff_tmp_7776f_20.clone()))
                + (q_acc_diff_tmp_7776f_21.clone()
                    * q_acc_diff_tmp_7776f_21.clone()))
                + (q_acc_diff_tmp_7776f_22.clone()
                    * q_acc_diff_tmp_7776f_22.clone()))
                + (q_acc_diff_tmp_7776f_23.clone()
                    * q_acc_diff_tmp_7776f_23.clone()))
                + (q_acc_diff_tmp_7776f_24.clone()
                    * q_acc_diff_tmp_7776f_24.clone()))
                + (q_acc_diff_tmp_7776f_25.clone()
                    * q_acc_diff_tmp_7776f_25.clone()))
                + (q_acc_diff_tmp_7776f_26.clone()
                    * q_acc_diff_tmp_7776f_26.clone()))
                + (q_acc_diff_tmp_7776f_27.clone()
                    * q_acc_diff_tmp_7776f_27.clone()))
                + (q_acc_diff_tmp_7776f_28.clone()
                    * q_acc_diff_tmp_7776f_28.clone()))
                + (q_acc_diff_tmp_7776f_29.clone() * q_acc_diff_tmp_7776f_29.clone()))
                + (q_acc_diff_tmp_7776f_30.clone() * q_acc_diff_tmp_7776f_30.clone()))
                + (q_acc_diff_tmp_7776f_31.clone() * q_acc_diff_tmp_7776f_31.clone()))
                + (q_acc_diff_tmp_7776f_32.clone() * q_acc_diff_tmp_7776f_32.clone()))
                + (q_acc_diff_tmp_7776f_33.clone() * q_acc_diff_tmp_7776f_33.clone()))
                + (q_acc_diff_tmp_7776f_34.clone() * q_acc_diff_tmp_7776f_34.clone()))
                + (q_acc_diff_tmp_7776f_35.clone() * q_acc_diff_tmp_7776f_35.clone()))
                + (q_acc_diff_tmp_7776f_36.clone() * q_acc_diff_tmp_7776f_36.clone()))
                + (q_acc_diff_tmp_7776f_37.clone() * q_acc_diff_tmp_7776f_37.clone()))
                + (q_acc_diff_tmp_7776f_38.clone() * q_acc_diff_tmp_7776f_38.clone()))
                + (q_acc_diff_tmp_7776f_39.clone() * q_acc_diff_tmp_7776f_39.clone()))
                + (q_acc_diff_tmp_7776f_40.clone() * q_acc_diff_tmp_7776f_40.clone()))
                * diff_sum_squares_inv_col145.clone())
                - M31_1.clone()),
        );
        EcAdd::evaluate(
            [
                input_accumulator_x_limb_0_col68.clone(),
                input_accumulator_x_limb_1_col69.clone(),
                input_accumulator_x_limb_2_col70.clone(),
                input_accumulator_x_limb_3_col71.clone(),
                input_accumulator_x_limb_4_col72.clone(),
                input_accumulator_x_limb_5_col73.clone(),
                input_accumulator_x_limb_6_col74.clone(),
                input_accumulator_x_limb_7_col75.clone(),
                input_accumulator_x_limb_8_col76.clone(),
                input_accumulator_x_limb_9_col77.clone(),
                input_accumulator_x_limb_10_col78.clone(),
                input_accumulator_x_limb_11_col79.clone(),
                input_accumulator_x_limb_12_col80.clone(),
                input_accumulator_x_limb_13_col81.clone(),
                input_accumulator_x_limb_14_col82.clone(),
                input_accumulator_x_limb_15_col83.clone(),
                input_accumulator_x_limb_16_col84.clone(),
                input_accumulator_x_limb_17_col85.clone(),
                input_accumulator_x_limb_18_col86.clone(),
                input_accumulator_x_limb_19_col87.clone(),
                input_accumulator_x_limb_20_col88.clone(),
                input_accumulator_x_limb_21_col89.clone(),
                input_accumulator_x_limb_22_col90.clone(),
                input_accumulator_x_limb_23_col91.clone(),
                input_accumulator_x_limb_24_col92.clone(),
                input_accumulator_x_limb_25_col93.clone(),
                input_accumulator_x_limb_26_col94.clone(),
                input_accumulator_x_limb_27_col95.clone(),
                input_accumulator_y_limb_0_col96.clone(),
                input_accumulator_y_limb_1_col97.clone(),
                input_accumulator_y_limb_2_col98.clone(),
                input_accumulator_y_limb_3_col99.clone(),
                input_accumulator_y_limb_4_col100.clone(),
                input_accumulator_y_limb_5_col101.clone(),
                input_accumulator_y_limb_6_col102.clone(),
                input_accumulator_y_limb_7_col103.clone(),
                input_accumulator_y_limb_8_col104.clone(),
                input_accumulator_y_limb_9_col105.clone(),
                input_accumulator_y_limb_10_col106.clone(),
                input_accumulator_y_limb_11_col107.clone(),
                input_accumulator_y_limb_12_col108.clone(),
                input_accumulator_y_limb_13_col109.clone(),
                input_accumulator_y_limb_14_col110.clone(),
                input_accumulator_y_limb_15_col111.clone(),
                input_accumulator_y_limb_16_col112.clone(),
                input_accumulator_y_limb_17_col113.clone(),
                input_accumulator_y_limb_18_col114.clone(),
                input_accumulator_y_limb_19_col115.clone(),
                input_accumulator_y_limb_20_col116.clone(),
                input_accumulator_y_limb_21_col117.clone(),
                input_accumulator_y_limb_22_col118.clone(),
                input_accumulator_y_limb_23_col119.clone(),
                input_accumulator_y_limb_24_col120.clone(),
                input_accumulator_y_limb_25_col121.clone(),
                input_accumulator_y_limb_26_col122.clone(),
                input_accumulator_y_limb_27_col123.clone(),
                input_q_x_limb_0_col12.clone(),
                input_q_x_limb_1_col13.clone(),
                input_q_x_limb_2_col14.clone(),
                input_q_x_limb_3_col15.clone(),
                input_q_x_limb_4_col16.clone(),
                input_q_x_limb_5_col17.clone(),
                input_q_x_limb_6_col18.clone(),
                input_q_x_limb_7_col19.clone(),
                input_q_x_limb_8_col20.clone(),
                input_q_x_limb_9_col21.clone(),
                input_q_x_limb_10_col22.clone(),
                input_q_x_limb_11_col23.clone(),
                input_q_x_limb_12_col24.clone(),
                input_q_x_limb_13_col25.clone(),
                input_q_x_limb_14_col26.clone(),
                input_q_x_limb_15_col27.clone(),
                input_q_x_limb_16_col28.clone(),
                input_q_x_limb_17_col29.clone(),
                input_q_x_limb_18_col30.clone(),
                input_q_x_limb_19_col31.clone(),
                input_q_x_limb_20_col32.clone(),
                input_q_x_limb_21_col33.clone(),
                input_q_x_limb_22_col34.clone(),
                input_q_x_limb_23_col35.clone(),
                input_q_x_limb_24_col36.clone(),
                input_q_x_limb_25_col37.clone(),
                input_q_x_limb_26_col38.clone(),
                input_q_x_limb_27_col39.clone(),
                input_q_y_limb_0_col40.clone(),
                input_q_y_limb_1_col41.clone(),
                input_q_y_limb_2_col42.clone(),
                input_q_y_limb_3_col43.clone(),
                input_q_y_limb_4_col44.clone(),
                input_q_y_limb_5_col45.clone(),
                input_q_y_limb_6_col46.clone(),
                input_q_y_limb_7_col47.clone(),
                input_q_y_limb_8_col48.clone(),
                input_q_y_limb_9_col49.clone(),
                input_q_y_limb_10_col50.clone(),
                input_q_y_limb_11_col51.clone(),
                input_q_y_limb_12_col52.clone(),
                input_q_y_limb_13_col53.clone(),
                input_q_y_limb_14_col54.clone(),
                input_q_y_limb_15_col55.clone(),
                input_q_y_limb_16_col56.clone(),
                input_q_y_limb_17_col57.clone(),
                input_q_y_limb_18_col58.clone(),
                input_q_y_limb_19_col59.clone(),
                input_q_y_limb_20_col60.clone(),
                input_q_y_limb_21_col61.clone(),
                input_q_y_limb_22_col62.clone(),
                input_q_y_limb_23_col63.clone(),
                input_q_y_limb_24_col64.clone(),
                input_q_y_limb_25_col65.clone(),
                input_q_y_limb_26_col66.clone(),
                input_q_y_limb_27_col67.clone(),
            ],
            slope_limb_0_col146.clone(),
            slope_limb_1_col147.clone(),
            slope_limb_2_col148.clone(),
            slope_limb_3_col149.clone(),
            slope_limb_4_col150.clone(),
            slope_limb_5_col151.clone(),
            slope_limb_6_col152.clone(),
            slope_limb_7_col153.clone(),
            slope_limb_8_col154.clone(),
            slope_limb_9_col155.clone(),
            slope_limb_10_col156.clone(),
            slope_limb_11_col157.clone(),
            slope_limb_12_col158.clone(),
            slope_limb_13_col159.clone(),
            slope_limb_14_col160.clone(),
            slope_limb_15_col161.clone(),
            slope_limb_16_col162.clone(),
            slope_limb_17_col163.clone(),
            slope_limb_18_col164.clone(),
            slope_limb_19_col165.clone(),
            slope_limb_20_col166.clone(),
            slope_limb_21_col167.clone(),
            slope_limb_22_col168.clone(),
            slope_limb_23_col169.clone(),
            slope_limb_24_col170.clone(),
            slope_limb_25_col171.clone(),
            slope_limb_26_col172.clone(),
            slope_limb_27_col173.clone(),
            k_col174.clone(),
            carry_0_col175.clone(),
            carry_1_col176.clone(),
            carry_2_col177.clone(),
            carry_3_col178.clone(),
            carry_4_col179.clone(),
            carry_5_col180.clone(),
            carry_6_col181.clone(),
            carry_7_col182.clone(),
            carry_8_col183.clone(),
            carry_9_col184.clone(),
            carry_10_col185.clone(),
            carry_11_col186.clone(),
            carry_12_col187.clone(),
            carry_13_col188.clone(),
            carry_14_col189.clone(),
            carry_15_col190.clone(),
            carry_16_col191.clone(),
            carry_17_col192.clone(),
            carry_18_col193.clone(),
            carry_19_col194.clone(),
            carry_20_col195.clone(),
            carry_21_col196.clone(),
            carry_22_col197.clone(),
            carry_23_col198.clone(),
            carry_24_col199.clone(),
            carry_25_col200.clone(),
            carry_26_col201.clone(),
            result_x_limb_0_col202.clone(),
            result_x_limb_1_col203.clone(),
            result_x_limb_2_col204.clone(),
            result_x_limb_3_col205.clone(),
            result_x_limb_4_col206.clone(),
            result_x_limb_5_col207.clone(),
            result_x_limb_6_col208.clone(),
            result_x_limb_7_col209.clone(),
            result_x_limb_8_col210.clone(),
            result_x_limb_9_col211.clone(),
            result_x_limb_10_col212.clone(),
            result_x_limb_11_col213.clone(),
            result_x_limb_12_col214.clone(),
            result_x_limb_13_col215.clone(),
            result_x_limb_14_col216.clone(),
            result_x_limb_15_col217.clone(),
            result_x_limb_16_col218.clone(),
            result_x_limb_17_col219.clone(),
            result_x_limb_18_col220.clone(),
            result_x_limb_19_col221.clone(),
            result_x_limb_20_col222.clone(),
            result_x_limb_21_col223.clone(),
            result_x_limb_22_col224.clone(),
            result_x_limb_23_col225.clone(),
            result_x_limb_24_col226.clone(),
            result_x_limb_25_col227.clone(),
            result_x_limb_26_col228.clone(),
            result_x_limb_27_col229.clone(),
            k_col230.clone(),
            carry_0_col231.clone(),
            carry_1_col232.clone(),
            carry_2_col233.clone(),
            carry_3_col234.clone(),
            carry_4_col235.clone(),
            carry_5_col236.clone(),
            carry_6_col237.clone(),
            carry_7_col238.clone(),
            carry_8_col239.clone(),
            carry_9_col240.clone(),
            carry_10_col241.clone(),
            carry_11_col242.clone(),
            carry_12_col243.clone(),
            carry_13_col244.clone(),
            carry_14_col245.clone(),
            carry_15_col246.clone(),
            carry_16_col247.clone(),
            carry_17_col248.clone(),
            carry_18_col249.clone(),
            carry_19_col250.clone(),
            carry_20_col251.clone(),
            carry_21_col252.clone(),
            carry_22_col253.clone(),
            carry_23_col254.clone(),
            carry_24_col255.clone(),
            carry_25_col256.clone(),
            carry_26_col257.clone(),
            result_y_limb_0_col258.clone(),
            result_y_limb_1_col259.clone(),
            result_y_limb_2_col260.clone(),
            result_y_limb_3_col261.clone(),
            result_y_limb_4_col262.clone(),
            result_y_limb_5_col263.clone(),
            result_y_limb_6_col264.clone(),
            result_y_limb_7_col265.clone(),
            result_y_limb_8_col266.clone(),
            result_y_limb_9_col267.clone(),
            result_y_limb_10_col268.clone(),
            result_y_limb_11_col269.clone(),
            result_y_limb_12_col270.clone(),
            result_y_limb_13_col271.clone(),
            result_y_limb_14_col272.clone(),
            result_y_limb_15_col273.clone(),
            result_y_limb_16_col274.clone(),
            result_y_limb_17_col275.clone(),
            result_y_limb_18_col276.clone(),
            result_y_limb_19_col277.clone(),
            result_y_limb_20_col278.clone(),
            result_y_limb_21_col279.clone(),
            result_y_limb_22_col280.clone(),
            result_y_limb_23_col281.clone(),
            result_y_limb_24_col282.clone(),
            result_y_limb_25_col283.clone(),
            result_y_limb_26_col284.clone(),
            result_y_limb_27_col285.clone(),
            k_col286.clone(),
            carry_0_col287.clone(),
            carry_1_col288.clone(),
            carry_2_col289.clone(),
            carry_3_col290.clone(),
            carry_4_col291.clone(),
            carry_5_col292.clone(),
            carry_6_col293.clone(),
            carry_7_col294.clone(),
            carry_8_col295.clone(),
            carry_9_col296.clone(),
            carry_10_col297.clone(),
            carry_11_col298.clone(),
            carry_12_col299.clone(),
            carry_13_col300.clone(),
            carry_14_col301.clone(),
            carry_15_col302.clone(),
            carry_16_col303.clone(),
            carry_17_col304.clone(),
            carry_18_col305.clone(),
            carry_19_col306.clone(),
            carry_20_col307.clone(),
            carry_21_col308.clone(),
            carry_22_col309.clone(),
            carry_23_col310.clone(),
            carry_24_col311.clone(),
            carry_25_col312.clone(),
            carry_26_col313.clone(),
            &self.common_lookup_elements,
            &mut eval,
        );
        // new_acculumator_0_0.
        eval.add_constraint(
            (new_acculumator_0_0_col314.clone()
                - (((result_x_limb_0_col202.clone() - input_accumulator_x_limb_0_col68.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_x_limb_0_col68.clone())),
        );
        // new_acculumator_0_1.
        eval.add_constraint(
            (new_acculumator_0_1_col315.clone()
                - (((result_x_limb_1_col203.clone() - input_accumulator_x_limb_1_col69.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_x_limb_1_col69.clone())),
        );
        // new_acculumator_0_2.
        eval.add_constraint(
            (new_acculumator_0_2_col316.clone()
                - (((result_x_limb_2_col204.clone() - input_accumulator_x_limb_2_col70.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_x_limb_2_col70.clone())),
        );
        // new_acculumator_0_3.
        eval.add_constraint(
            (new_acculumator_0_3_col317.clone()
                - (((result_x_limb_3_col205.clone() - input_accumulator_x_limb_3_col71.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_x_limb_3_col71.clone())),
        );
        // new_acculumator_0_4.
        eval.add_constraint(
            (new_acculumator_0_4_col318.clone()
                - (((result_x_limb_4_col206.clone() - input_accumulator_x_limb_4_col72.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_x_limb_4_col72.clone())),
        );
        // new_acculumator_0_5.
        eval.add_constraint(
            (new_acculumator_0_5_col319.clone()
                - (((result_x_limb_5_col207.clone() - input_accumulator_x_limb_5_col73.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_x_limb_5_col73.clone())),
        );
        // new_acculumator_0_6.
        eval.add_constraint(
            (new_acculumator_0_6_col320.clone()
                - (((result_x_limb_6_col208.clone() - input_accumulator_x_limb_6_col74.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_x_limb_6_col74.clone())),
        );
        // new_acculumator_0_7.
        eval.add_constraint(
            (new_acculumator_0_7_col321.clone()
                - (((result_x_limb_7_col209.clone() - input_accumulator_x_limb_7_col75.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_x_limb_7_col75.clone())),
        );
        // new_acculumator_0_8.
        eval.add_constraint(
            (new_acculumator_0_8_col322.clone()
                - (((result_x_limb_8_col210.clone() - input_accumulator_x_limb_8_col76.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_x_limb_8_col76.clone())),
        );
        // new_acculumator_0_9.
        eval.add_constraint(
            (new_acculumator_0_9_col323.clone()
                - (((result_x_limb_9_col211.clone() - input_accumulator_x_limb_9_col77.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_x_limb_9_col77.clone())),
        );
        // new_acculumator_0_10.
        eval.add_constraint(
            (new_acculumator_0_10_col324.clone()
                - (((result_x_limb_10_col212.clone()
                    - input_accumulator_x_limb_10_col78.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_x_limb_10_col78.clone())),
        );
        // new_acculumator_0_11.
        eval.add_constraint(
            (new_acculumator_0_11_col325.clone()
                - (((result_x_limb_11_col213.clone()
                    - input_accumulator_x_limb_11_col79.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_x_limb_11_col79.clone())),
        );
        // new_acculumator_0_12.
        eval.add_constraint(
            (new_acculumator_0_12_col326.clone()
                - (((result_x_limb_12_col214.clone()
                    - input_accumulator_x_limb_12_col80.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_x_limb_12_col80.clone())),
        );
        // new_acculumator_0_13.
        eval.add_constraint(
            (new_acculumator_0_13_col327.clone()
                - (((result_x_limb_13_col215.clone()
                    - input_accumulator_x_limb_13_col81.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_x_limb_13_col81.clone())),
        );
        // new_acculumator_0_14.
        eval.add_constraint(
            (new_acculumator_0_14_col328.clone()
                - (((result_x_limb_14_col216.clone()
                    - input_accumulator_x_limb_14_col82.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_x_limb_14_col82.clone())),
        );
        // new_acculumator_0_15.
        eval.add_constraint(
            (new_acculumator_0_15_col329.clone()
                - (((result_x_limb_15_col217.clone()
                    - input_accumulator_x_limb_15_col83.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_x_limb_15_col83.clone())),
        );
        // new_acculumator_0_16.
        eval.add_constraint(
            (new_acculumator_0_16_col330.clone()
                - (((result_x_limb_16_col218.clone()
                    - input_accumulator_x_limb_16_col84.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_x_limb_16_col84.clone())),
        );
        // new_acculumator_0_17.
        eval.add_constraint(
            (new_acculumator_0_17_col331.clone()
                - (((result_x_limb_17_col219.clone()
                    - input_accumulator_x_limb_17_col85.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_x_limb_17_col85.clone())),
        );
        // new_acculumator_0_18.
        eval.add_constraint(
            (new_acculumator_0_18_col332.clone()
                - (((result_x_limb_18_col220.clone()
                    - input_accumulator_x_limb_18_col86.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_x_limb_18_col86.clone())),
        );
        // new_acculumator_0_19.
        eval.add_constraint(
            (new_acculumator_0_19_col333.clone()
                - (((result_x_limb_19_col221.clone()
                    - input_accumulator_x_limb_19_col87.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_x_limb_19_col87.clone())),
        );
        // new_acculumator_0_20.
        eval.add_constraint(
            (new_acculumator_0_20_col334.clone()
                - (((result_x_limb_20_col222.clone()
                    - input_accumulator_x_limb_20_col88.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_x_limb_20_col88.clone())),
        );
        // new_acculumator_0_21.
        eval.add_constraint(
            (new_acculumator_0_21_col335.clone()
                - (((result_x_limb_21_col223.clone()
                    - input_accumulator_x_limb_21_col89.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_x_limb_21_col89.clone())),
        );
        // new_acculumator_0_22.
        eval.add_constraint(
            (new_acculumator_0_22_col336.clone()
                - (((result_x_limb_22_col224.clone()
                    - input_accumulator_x_limb_22_col90.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_x_limb_22_col90.clone())),
        );
        // new_acculumator_0_23.
        eval.add_constraint(
            (new_acculumator_0_23_col337.clone()
                - (((result_x_limb_23_col225.clone()
                    - input_accumulator_x_limb_23_col91.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_x_limb_23_col91.clone())),
        );
        // new_acculumator_0_24.
        eval.add_constraint(
            (new_acculumator_0_24_col338.clone()
                - (((result_x_limb_24_col226.clone()
                    - input_accumulator_x_limb_24_col92.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_x_limb_24_col92.clone())),
        );
        // new_acculumator_0_25.
        eval.add_constraint(
            (new_acculumator_0_25_col339.clone()
                - (((result_x_limb_25_col227.clone()
                    - input_accumulator_x_limb_25_col93.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_x_limb_25_col93.clone())),
        );
        // new_acculumator_0_26.
        eval.add_constraint(
            (new_acculumator_0_26_col340.clone()
                - (((result_x_limb_26_col228.clone()
                    - input_accumulator_x_limb_26_col94.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_x_limb_26_col94.clone())),
        );
        // new_acculumator_0_27.
        eval.add_constraint(
            (new_acculumator_0_27_col341.clone()
                - (((result_x_limb_27_col229.clone()
                    - input_accumulator_x_limb_27_col95.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_x_limb_27_col95.clone())),
        );
        // new_acculumator_1_0.
        eval.add_constraint(
            (new_acculumator_1_0_col342.clone()
                - (((result_y_limb_0_col258.clone() - input_accumulator_y_limb_0_col96.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_y_limb_0_col96.clone())),
        );
        // new_acculumator_1_1.
        eval.add_constraint(
            (new_acculumator_1_1_col343.clone()
                - (((result_y_limb_1_col259.clone() - input_accumulator_y_limb_1_col97.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_y_limb_1_col97.clone())),
        );
        // new_acculumator_1_2.
        eval.add_constraint(
            (new_acculumator_1_2_col344.clone()
                - (((result_y_limb_2_col260.clone() - input_accumulator_y_limb_2_col98.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_y_limb_2_col98.clone())),
        );
        // new_acculumator_1_3.
        eval.add_constraint(
            (new_acculumator_1_3_col345.clone()
                - (((result_y_limb_3_col261.clone() - input_accumulator_y_limb_3_col99.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_y_limb_3_col99.clone())),
        );
        // new_acculumator_1_4.
        eval.add_constraint(
            (new_acculumator_1_4_col346.clone()
                - (((result_y_limb_4_col262.clone() - input_accumulator_y_limb_4_col100.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_y_limb_4_col100.clone())),
        );
        // new_acculumator_1_5.
        eval.add_constraint(
            (new_acculumator_1_5_col347.clone()
                - (((result_y_limb_5_col263.clone() - input_accumulator_y_limb_5_col101.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_y_limb_5_col101.clone())),
        );
        // new_acculumator_1_6.
        eval.add_constraint(
            (new_acculumator_1_6_col348.clone()
                - (((result_y_limb_6_col264.clone() - input_accumulator_y_limb_6_col102.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_y_limb_6_col102.clone())),
        );
        // new_acculumator_1_7.
        eval.add_constraint(
            (new_acculumator_1_7_col349.clone()
                - (((result_y_limb_7_col265.clone() - input_accumulator_y_limb_7_col103.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_y_limb_7_col103.clone())),
        );
        // new_acculumator_1_8.
        eval.add_constraint(
            (new_acculumator_1_8_col350.clone()
                - (((result_y_limb_8_col266.clone() - input_accumulator_y_limb_8_col104.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_y_limb_8_col104.clone())),
        );
        // new_acculumator_1_9.
        eval.add_constraint(
            (new_acculumator_1_9_col351.clone()
                - (((result_y_limb_9_col267.clone() - input_accumulator_y_limb_9_col105.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_y_limb_9_col105.clone())),
        );
        // new_acculumator_1_10.
        eval.add_constraint(
            (new_acculumator_1_10_col352.clone()
                - (((result_y_limb_10_col268.clone()
                    - input_accumulator_y_limb_10_col106.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_y_limb_10_col106.clone())),
        );
        // new_acculumator_1_11.
        eval.add_constraint(
            (new_acculumator_1_11_col353.clone()
                - (((result_y_limb_11_col269.clone()
                    - input_accumulator_y_limb_11_col107.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_y_limb_11_col107.clone())),
        );
        // new_acculumator_1_12.
        eval.add_constraint(
            (new_acculumator_1_12_col354.clone()
                - (((result_y_limb_12_col270.clone()
                    - input_accumulator_y_limb_12_col108.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_y_limb_12_col108.clone())),
        );
        // new_acculumator_1_13.
        eval.add_constraint(
            (new_acculumator_1_13_col355.clone()
                - (((result_y_limb_13_col271.clone()
                    - input_accumulator_y_limb_13_col109.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_y_limb_13_col109.clone())),
        );
        // new_acculumator_1_14.
        eval.add_constraint(
            (new_acculumator_1_14_col356.clone()
                - (((result_y_limb_14_col272.clone()
                    - input_accumulator_y_limb_14_col110.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_y_limb_14_col110.clone())),
        );
        // new_acculumator_1_15.
        eval.add_constraint(
            (new_acculumator_1_15_col357.clone()
                - (((result_y_limb_15_col273.clone()
                    - input_accumulator_y_limb_15_col111.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_y_limb_15_col111.clone())),
        );
        // new_acculumator_1_16.
        eval.add_constraint(
            (new_acculumator_1_16_col358.clone()
                - (((result_y_limb_16_col274.clone()
                    - input_accumulator_y_limb_16_col112.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_y_limb_16_col112.clone())),
        );
        // new_acculumator_1_17.
        eval.add_constraint(
            (new_acculumator_1_17_col359.clone()
                - (((result_y_limb_17_col275.clone()
                    - input_accumulator_y_limb_17_col113.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_y_limb_17_col113.clone())),
        );
        // new_acculumator_1_18.
        eval.add_constraint(
            (new_acculumator_1_18_col360.clone()
                - (((result_y_limb_18_col276.clone()
                    - input_accumulator_y_limb_18_col114.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_y_limb_18_col114.clone())),
        );
        // new_acculumator_1_19.
        eval.add_constraint(
            (new_acculumator_1_19_col361.clone()
                - (((result_y_limb_19_col277.clone()
                    - input_accumulator_y_limb_19_col115.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_y_limb_19_col115.clone())),
        );
        // new_acculumator_1_20.
        eval.add_constraint(
            (new_acculumator_1_20_col362.clone()
                - (((result_y_limb_20_col278.clone()
                    - input_accumulator_y_limb_20_col116.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_y_limb_20_col116.clone())),
        );
        // new_acculumator_1_21.
        eval.add_constraint(
            (new_acculumator_1_21_col363.clone()
                - (((result_y_limb_21_col279.clone()
                    - input_accumulator_y_limb_21_col117.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_y_limb_21_col117.clone())),
        );
        // new_acculumator_1_22.
        eval.add_constraint(
            (new_acculumator_1_22_col364.clone()
                - (((result_y_limb_22_col280.clone()
                    - input_accumulator_y_limb_22_col118.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_y_limb_22_col118.clone())),
        );
        // new_acculumator_1_23.
        eval.add_constraint(
            (new_acculumator_1_23_col365.clone()
                - (((result_y_limb_23_col281.clone()
                    - input_accumulator_y_limb_23_col119.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_y_limb_23_col119.clone())),
        );
        // new_acculumator_1_24.
        eval.add_constraint(
            (new_acculumator_1_24_col366.clone()
                - (((result_y_limb_24_col282.clone()
                    - input_accumulator_y_limb_24_col120.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_y_limb_24_col120.clone())),
        );
        // new_acculumator_1_25.
        eval.add_constraint(
            (new_acculumator_1_25_col367.clone()
                - (((result_y_limb_25_col283.clone()
                    - input_accumulator_y_limb_25_col121.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_y_limb_25_col121.clone())),
        );
        // new_acculumator_1_26.
        eval.add_constraint(
            (new_acculumator_1_26_col368.clone()
                - (((result_y_limb_26_col284.clone()
                    - input_accumulator_y_limb_26_col122.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_y_limb_26_col122.clone())),
        );
        // new_acculumator_1_27.
        eval.add_constraint(
            (new_acculumator_1_27_col369.clone()
                - (((result_y_limb_27_col285.clone()
                    - input_accumulator_y_limb_27_col123.clone())
                    * to_add_bit_col125.clone())
                    + input_accumulator_y_limb_27_col123.clone())),
        );
        EcDouble::evaluate(
            [
                input_q_x_limb_0_col12.clone(),
                input_q_x_limb_1_col13.clone(),
                input_q_x_limb_2_col14.clone(),
                input_q_x_limb_3_col15.clone(),
                input_q_x_limb_4_col16.clone(),
                input_q_x_limb_5_col17.clone(),
                input_q_x_limb_6_col18.clone(),
                input_q_x_limb_7_col19.clone(),
                input_q_x_limb_8_col20.clone(),
                input_q_x_limb_9_col21.clone(),
                input_q_x_limb_10_col22.clone(),
                input_q_x_limb_11_col23.clone(),
                input_q_x_limb_12_col24.clone(),
                input_q_x_limb_13_col25.clone(),
                input_q_x_limb_14_col26.clone(),
                input_q_x_limb_15_col27.clone(),
                input_q_x_limb_16_col28.clone(),
                input_q_x_limb_17_col29.clone(),
                input_q_x_limb_18_col30.clone(),
                input_q_x_limb_19_col31.clone(),
                input_q_x_limb_20_col32.clone(),
                input_q_x_limb_21_col33.clone(),
                input_q_x_limb_22_col34.clone(),
                input_q_x_limb_23_col35.clone(),
                input_q_x_limb_24_col36.clone(),
                input_q_x_limb_25_col37.clone(),
                input_q_x_limb_26_col38.clone(),
                input_q_x_limb_27_col39.clone(),
                input_q_y_limb_0_col40.clone(),
                input_q_y_limb_1_col41.clone(),
                input_q_y_limb_2_col42.clone(),
                input_q_y_limb_3_col43.clone(),
                input_q_y_limb_4_col44.clone(),
                input_q_y_limb_5_col45.clone(),
                input_q_y_limb_6_col46.clone(),
                input_q_y_limb_7_col47.clone(),
                input_q_y_limb_8_col48.clone(),
                input_q_y_limb_9_col49.clone(),
                input_q_y_limb_10_col50.clone(),
                input_q_y_limb_11_col51.clone(),
                input_q_y_limb_12_col52.clone(),
                input_q_y_limb_13_col53.clone(),
                input_q_y_limb_14_col54.clone(),
                input_q_y_limb_15_col55.clone(),
                input_q_y_limb_16_col56.clone(),
                input_q_y_limb_17_col57.clone(),
                input_q_y_limb_18_col58.clone(),
                input_q_y_limb_19_col59.clone(),
                input_q_y_limb_20_col60.clone(),
                input_q_y_limb_21_col61.clone(),
                input_q_y_limb_22_col62.clone(),
                input_q_y_limb_23_col63.clone(),
                input_q_y_limb_24_col64.clone(),
                input_q_y_limb_25_col65.clone(),
                input_q_y_limb_26_col66.clone(),
                input_q_y_limb_27_col67.clone(),
            ],
            mul_res_limb_0_col370.clone(),
            mul_res_limb_1_col371.clone(),
            mul_res_limb_2_col372.clone(),
            mul_res_limb_3_col373.clone(),
            mul_res_limb_4_col374.clone(),
            mul_res_limb_5_col375.clone(),
            mul_res_limb_6_col376.clone(),
            mul_res_limb_7_col377.clone(),
            mul_res_limb_8_col378.clone(),
            mul_res_limb_9_col379.clone(),
            mul_res_limb_10_col380.clone(),
            mul_res_limb_11_col381.clone(),
            mul_res_limb_12_col382.clone(),
            mul_res_limb_13_col383.clone(),
            mul_res_limb_14_col384.clone(),
            mul_res_limb_15_col385.clone(),
            mul_res_limb_16_col386.clone(),
            mul_res_limb_17_col387.clone(),
            mul_res_limb_18_col388.clone(),
            mul_res_limb_19_col389.clone(),
            mul_res_limb_20_col390.clone(),
            mul_res_limb_21_col391.clone(),
            mul_res_limb_22_col392.clone(),
            mul_res_limb_23_col393.clone(),
            mul_res_limb_24_col394.clone(),
            mul_res_limb_25_col395.clone(),
            mul_res_limb_26_col396.clone(),
            mul_res_limb_27_col397.clone(),
            k_col398.clone(),
            carry_0_col399.clone(),
            carry_1_col400.clone(),
            carry_2_col401.clone(),
            carry_3_col402.clone(),
            carry_4_col403.clone(),
            carry_5_col404.clone(),
            carry_6_col405.clone(),
            carry_7_col406.clone(),
            carry_8_col407.clone(),
            carry_9_col408.clone(),
            carry_10_col409.clone(),
            carry_11_col410.clone(),
            carry_12_col411.clone(),
            carry_13_col412.clone(),
            carry_14_col413.clone(),
            carry_15_col414.clone(),
            carry_16_col415.clone(),
            carry_17_col416.clone(),
            carry_18_col417.clone(),
            carry_19_col418.clone(),
            carry_20_col419.clone(),
            carry_21_col420.clone(),
            carry_22_col421.clone(),
            carry_23_col422.clone(),
            carry_24_col423.clone(),
            carry_25_col424.clone(),
            carry_26_col425.clone(),
            add_res_limb_0_col426.clone(),
            add_res_limb_1_col427.clone(),
            add_res_limb_2_col428.clone(),
            add_res_limb_3_col429.clone(),
            add_res_limb_4_col430.clone(),
            add_res_limb_5_col431.clone(),
            add_res_limb_6_col432.clone(),
            add_res_limb_7_col433.clone(),
            add_res_limb_8_col434.clone(),
            add_res_limb_9_col435.clone(),
            add_res_limb_10_col436.clone(),
            add_res_limb_11_col437.clone(),
            add_res_limb_12_col438.clone(),
            add_res_limb_13_col439.clone(),
            add_res_limb_14_col440.clone(),
            add_res_limb_15_col441.clone(),
            add_res_limb_16_col442.clone(),
            add_res_limb_17_col443.clone(),
            add_res_limb_18_col444.clone(),
            add_res_limb_19_col445.clone(),
            add_res_limb_20_col446.clone(),
            add_res_limb_21_col447.clone(),
            add_res_limb_22_col448.clone(),
            add_res_limb_23_col449.clone(),
            add_res_limb_24_col450.clone(),
            add_res_limb_25_col451.clone(),
            add_res_limb_26_col452.clone(),
            add_res_limb_27_col453.clone(),
            sub_p_bit_col454.clone(),
            slope_limb_0_col455.clone(),
            slope_limb_1_col456.clone(),
            slope_limb_2_col457.clone(),
            slope_limb_3_col458.clone(),
            slope_limb_4_col459.clone(),
            slope_limb_5_col460.clone(),
            slope_limb_6_col461.clone(),
            slope_limb_7_col462.clone(),
            slope_limb_8_col463.clone(),
            slope_limb_9_col464.clone(),
            slope_limb_10_col465.clone(),
            slope_limb_11_col466.clone(),
            slope_limb_12_col467.clone(),
            slope_limb_13_col468.clone(),
            slope_limb_14_col469.clone(),
            slope_limb_15_col470.clone(),
            slope_limb_16_col471.clone(),
            slope_limb_17_col472.clone(),
            slope_limb_18_col473.clone(),
            slope_limb_19_col474.clone(),
            slope_limb_20_col475.clone(),
            slope_limb_21_col476.clone(),
            slope_limb_22_col477.clone(),
            slope_limb_23_col478.clone(),
            slope_limb_24_col479.clone(),
            slope_limb_25_col480.clone(),
            slope_limb_26_col481.clone(),
            slope_limb_27_col482.clone(),
            k_col483.clone(),
            carry_0_col484.clone(),
            carry_1_col485.clone(),
            carry_2_col486.clone(),
            carry_3_col487.clone(),
            carry_4_col488.clone(),
            carry_5_col489.clone(),
            carry_6_col490.clone(),
            carry_7_col491.clone(),
            carry_8_col492.clone(),
            carry_9_col493.clone(),
            carry_10_col494.clone(),
            carry_11_col495.clone(),
            carry_12_col496.clone(),
            carry_13_col497.clone(),
            carry_14_col498.clone(),
            carry_15_col499.clone(),
            carry_16_col500.clone(),
            carry_17_col501.clone(),
            carry_18_col502.clone(),
            carry_19_col503.clone(),
            carry_20_col504.clone(),
            carry_21_col505.clone(),
            carry_22_col506.clone(),
            carry_23_col507.clone(),
            carry_24_col508.clone(),
            carry_25_col509.clone(),
            carry_26_col510.clone(),
            result_x_limb_0_col511.clone(),
            result_x_limb_1_col512.clone(),
            result_x_limb_2_col513.clone(),
            result_x_limb_3_col514.clone(),
            result_x_limb_4_col515.clone(),
            result_x_limb_5_col516.clone(),
            result_x_limb_6_col517.clone(),
            result_x_limb_7_col518.clone(),
            result_x_limb_8_col519.clone(),
            result_x_limb_9_col520.clone(),
            result_x_limb_10_col521.clone(),
            result_x_limb_11_col522.clone(),
            result_x_limb_12_col523.clone(),
            result_x_limb_13_col524.clone(),
            result_x_limb_14_col525.clone(),
            result_x_limb_15_col526.clone(),
            result_x_limb_16_col527.clone(),
            result_x_limb_17_col528.clone(),
            result_x_limb_18_col529.clone(),
            result_x_limb_19_col530.clone(),
            result_x_limb_20_col531.clone(),
            result_x_limb_21_col532.clone(),
            result_x_limb_22_col533.clone(),
            result_x_limb_23_col534.clone(),
            result_x_limb_24_col535.clone(),
            result_x_limb_25_col536.clone(),
            result_x_limb_26_col537.clone(),
            result_x_limb_27_col538.clone(),
            k_col539.clone(),
            carry_0_col540.clone(),
            carry_1_col541.clone(),
            carry_2_col542.clone(),
            carry_3_col543.clone(),
            carry_4_col544.clone(),
            carry_5_col545.clone(),
            carry_6_col546.clone(),
            carry_7_col547.clone(),
            carry_8_col548.clone(),
            carry_9_col549.clone(),
            carry_10_col550.clone(),
            carry_11_col551.clone(),
            carry_12_col552.clone(),
            carry_13_col553.clone(),
            carry_14_col554.clone(),
            carry_15_col555.clone(),
            carry_16_col556.clone(),
            carry_17_col557.clone(),
            carry_18_col558.clone(),
            carry_19_col559.clone(),
            carry_20_col560.clone(),
            carry_21_col561.clone(),
            carry_22_col562.clone(),
            carry_23_col563.clone(),
            carry_24_col564.clone(),
            carry_25_col565.clone(),
            carry_26_col566.clone(),
            result_y_limb_0_col567.clone(),
            result_y_limb_1_col568.clone(),
            result_y_limb_2_col569.clone(),
            result_y_limb_3_col570.clone(),
            result_y_limb_4_col571.clone(),
            result_y_limb_5_col572.clone(),
            result_y_limb_6_col573.clone(),
            result_y_limb_7_col574.clone(),
            result_y_limb_8_col575.clone(),
            result_y_limb_9_col576.clone(),
            result_y_limb_10_col577.clone(),
            result_y_limb_11_col578.clone(),
            result_y_limb_12_col579.clone(),
            result_y_limb_13_col580.clone(),
            result_y_limb_14_col581.clone(),
            result_y_limb_15_col582.clone(),
            result_y_limb_16_col583.clone(),
            result_y_limb_17_col584.clone(),
            result_y_limb_18_col585.clone(),
            result_y_limb_19_col586.clone(),
            result_y_limb_20_col587.clone(),
            result_y_limb_21_col588.clone(),
            result_y_limb_22_col589.clone(),
            result_y_limb_23_col590.clone(),
            result_y_limb_24_col591.clone(),
            result_y_limb_25_col592.clone(),
            result_y_limb_26_col593.clone(),
            result_y_limb_27_col594.clone(),
            k_col595.clone(),
            carry_0_col596.clone(),
            carry_1_col597.clone(),
            carry_2_col598.clone(),
            carry_3_col599.clone(),
            carry_4_col600.clone(),
            carry_5_col601.clone(),
            carry_6_col602.clone(),
            carry_7_col603.clone(),
            carry_8_col604.clone(),
            carry_9_col605.clone(),
            carry_10_col606.clone(),
            carry_11_col607.clone(),
            carry_12_col608.clone(),
            carry_13_col609.clone(),
            carry_14_col610.clone(),
            carry_15_col611.clone(),
            carry_16_col612.clone(),
            carry_17_col613.clone(),
            carry_18_col614.clone(),
            carry_19_col615.clone(),
            carry_20_col616.clone(),
            carry_21_col617.clone(),
            carry_22_col618.clone(),
            carry_23_col619.clone(),
            carry_24_col620.clone(),
            carry_25_col621.clone(),
            carry_26_col622.clone(),
            &self.common_lookup_elements,
            &mut eval,
        );
        eval.add_to_relation(RelationEntry::new(
            &self.common_lookup_elements,
            E::EF::from(enabler_col623.clone()),
            &[
                M31_183619546.clone(),
                input_chain_id_col0.clone(),
                input_round_num_col1.clone(),
                input_m_limb_0_col2.clone(),
                input_m_limb_1_col3.clone(),
                input_m_limb_2_col4.clone(),
                input_m_limb_3_col5.clone(),
                input_m_limb_4_col6.clone(),
                input_m_limb_5_col7.clone(),
                input_m_limb_6_col8.clone(),
                input_m_limb_7_col9.clone(),
                input_m_limb_8_col10.clone(),
                input_m_limb_9_col11.clone(),
                input_q_x_limb_0_col12.clone(),
                input_q_x_limb_1_col13.clone(),
                input_q_x_limb_2_col14.clone(),
                input_q_x_limb_3_col15.clone(),
                input_q_x_limb_4_col16.clone(),
                input_q_x_limb_5_col17.clone(),
                input_q_x_limb_6_col18.clone(),
                input_q_x_limb_7_col19.clone(),
                input_q_x_limb_8_col20.clone(),
                input_q_x_limb_9_col21.clone(),
                input_q_x_limb_10_col22.clone(),
                input_q_x_limb_11_col23.clone(),
                input_q_x_limb_12_col24.clone(),
                input_q_x_limb_13_col25.clone(),
                input_q_x_limb_14_col26.clone(),
                input_q_x_limb_15_col27.clone(),
                input_q_x_limb_16_col28.clone(),
                input_q_x_limb_17_col29.clone(),
                input_q_x_limb_18_col30.clone(),
                input_q_x_limb_19_col31.clone(),
                input_q_x_limb_20_col32.clone(),
                input_q_x_limb_21_col33.clone(),
                input_q_x_limb_22_col34.clone(),
                input_q_x_limb_23_col35.clone(),
                input_q_x_limb_24_col36.clone(),
                input_q_x_limb_25_col37.clone(),
                input_q_x_limb_26_col38.clone(),
                input_q_x_limb_27_col39.clone(),
                input_q_y_limb_0_col40.clone(),
                input_q_y_limb_1_col41.clone(),
                input_q_y_limb_2_col42.clone(),
                input_q_y_limb_3_col43.clone(),
                input_q_y_limb_4_col44.clone(),
                input_q_y_limb_5_col45.clone(),
                input_q_y_limb_6_col46.clone(),
                input_q_y_limb_7_col47.clone(),
                input_q_y_limb_8_col48.clone(),
                input_q_y_limb_9_col49.clone(),
                input_q_y_limb_10_col50.clone(),
                input_q_y_limb_11_col51.clone(),
                input_q_y_limb_12_col52.clone(),
                input_q_y_limb_13_col53.clone(),
                input_q_y_limb_14_col54.clone(),
                input_q_y_limb_15_col55.clone(),
                input_q_y_limb_16_col56.clone(),
                input_q_y_limb_17_col57.clone(),
                input_q_y_limb_18_col58.clone(),
                input_q_y_limb_19_col59.clone(),
                input_q_y_limb_20_col60.clone(),
                input_q_y_limb_21_col61.clone(),
                input_q_y_limb_22_col62.clone(),
                input_q_y_limb_23_col63.clone(),
                input_q_y_limb_24_col64.clone(),
                input_q_y_limb_25_col65.clone(),
                input_q_y_limb_26_col66.clone(),
                input_q_y_limb_27_col67.clone(),
                input_accumulator_x_limb_0_col68.clone(),
                input_accumulator_x_limb_1_col69.clone(),
                input_accumulator_x_limb_2_col70.clone(),
                input_accumulator_x_limb_3_col71.clone(),
                input_accumulator_x_limb_4_col72.clone(),
                input_accumulator_x_limb_5_col73.clone(),
                input_accumulator_x_limb_6_col74.clone(),
                input_accumulator_x_limb_7_col75.clone(),
                input_accumulator_x_limb_8_col76.clone(),
                input_accumulator_x_limb_9_col77.clone(),
                input_accumulator_x_limb_10_col78.clone(),
                input_accumulator_x_limb_11_col79.clone(),
                input_accumulator_x_limb_12_col80.clone(),
                input_accumulator_x_limb_13_col81.clone(),
                input_accumulator_x_limb_14_col82.clone(),
                input_accumulator_x_limb_15_col83.clone(),
                input_accumulator_x_limb_16_col84.clone(),
                input_accumulator_x_limb_17_col85.clone(),
                input_accumulator_x_limb_18_col86.clone(),
                input_accumulator_x_limb_19_col87.clone(),
                input_accumulator_x_limb_20_col88.clone(),
                input_accumulator_x_limb_21_col89.clone(),
                input_accumulator_x_limb_22_col90.clone(),
                input_accumulator_x_limb_23_col91.clone(),
                input_accumulator_x_limb_24_col92.clone(),
                input_accumulator_x_limb_25_col93.clone(),
                input_accumulator_x_limb_26_col94.clone(),
                input_accumulator_x_limb_27_col95.clone(),
                input_accumulator_y_limb_0_col96.clone(),
                input_accumulator_y_limb_1_col97.clone(),
                input_accumulator_y_limb_2_col98.clone(),
                input_accumulator_y_limb_3_col99.clone(),
                input_accumulator_y_limb_4_col100.clone(),
                input_accumulator_y_limb_5_col101.clone(),
                input_accumulator_y_limb_6_col102.clone(),
                input_accumulator_y_limb_7_col103.clone(),
                input_accumulator_y_limb_8_col104.clone(),
                input_accumulator_y_limb_9_col105.clone(),
                input_accumulator_y_limb_10_col106.clone(),
                input_accumulator_y_limb_11_col107.clone(),
                input_accumulator_y_limb_12_col108.clone(),
                input_accumulator_y_limb_13_col109.clone(),
                input_accumulator_y_limb_14_col110.clone(),
                input_accumulator_y_limb_15_col111.clone(),
                input_accumulator_y_limb_16_col112.clone(),
                input_accumulator_y_limb_17_col113.clone(),
                input_accumulator_y_limb_18_col114.clone(),
                input_accumulator_y_limb_19_col115.clone(),
                input_accumulator_y_limb_20_col116.clone(),
                input_accumulator_y_limb_21_col117.clone(),
                input_accumulator_y_limb_22_col118.clone(),
                input_accumulator_y_limb_23_col119.clone(),
                input_accumulator_y_limb_24_col120.clone(),
                input_accumulator_y_limb_25_col121.clone(),
                input_accumulator_y_limb_26_col122.clone(),
                input_accumulator_y_limb_27_col123.clone(),
                input_counter_col124.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.common_lookup_elements,
            -E::EF::from(enabler_col623.clone()),
            &[
                M31_183619546.clone(),
                input_chain_id_col0.clone(),
                (input_round_num_col1.clone() + M31_1.clone()),
                next_m_0_col128.clone(),
                next_m_1_col129.clone(),
                next_m_2_col130.clone(),
                next_m_3_col131.clone(),
                next_m_4_col132.clone(),
                next_m_5_col133.clone(),
                next_m_6_col134.clone(),
                next_m_7_col135.clone(),
                next_m_8_col136.clone(),
                next_m_9_col137.clone(),
                result_x_limb_0_col511.clone(),
                result_x_limb_1_col512.clone(),
                result_x_limb_2_col513.clone(),
                result_x_limb_3_col514.clone(),
                result_x_limb_4_col515.clone(),
                result_x_limb_5_col516.clone(),
                result_x_limb_6_col517.clone(),
                result_x_limb_7_col518.clone(),
                result_x_limb_8_col519.clone(),
                result_x_limb_9_col520.clone(),
                result_x_limb_10_col521.clone(),
                result_x_limb_11_col522.clone(),
                result_x_limb_12_col523.clone(),
                result_x_limb_13_col524.clone(),
                result_x_limb_14_col525.clone(),
                result_x_limb_15_col526.clone(),
                result_x_limb_16_col527.clone(),
                result_x_limb_17_col528.clone(),
                result_x_limb_18_col529.clone(),
                result_x_limb_19_col530.clone(),
                result_x_limb_20_col531.clone(),
                result_x_limb_21_col532.clone(),
                result_x_limb_22_col533.clone(),
                result_x_limb_23_col534.clone(),
                result_x_limb_24_col535.clone(),
                result_x_limb_25_col536.clone(),
                result_x_limb_26_col537.clone(),
                result_x_limb_27_col538.clone(),
                result_y_limb_0_col567.clone(),
                result_y_limb_1_col568.clone(),
                result_y_limb_2_col569.clone(),
                result_y_limb_3_col570.clone(),
                result_y_limb_4_col571.clone(),
                result_y_limb_5_col572.clone(),
                result_y_limb_6_col573.clone(),
                result_y_limb_7_col574.clone(),
                result_y_limb_8_col575.clone(),
                result_y_limb_9_col576.clone(),
                result_y_limb_10_col577.clone(),
                result_y_limb_11_col578.clone(),
                result_y_limb_12_col579.clone(),
                result_y_limb_13_col580.clone(),
                result_y_limb_14_col581.clone(),
                result_y_limb_15_col582.clone(),
                result_y_limb_16_col583.clone(),
                result_y_limb_17_col584.clone(),
                result_y_limb_18_col585.clone(),
                result_y_limb_19_col586.clone(),
                result_y_limb_20_col587.clone(),
                result_y_limb_21_col588.clone(),
                result_y_limb_22_col589.clone(),
                result_y_limb_23_col590.clone(),
                result_y_limb_24_col591.clone(),
                result_y_limb_25_col592.clone(),
                result_y_limb_26_col593.clone(),
                result_y_limb_27_col594.clone(),
                new_acculumator_0_0_col314.clone(),
                new_acculumator_0_1_col315.clone(),
                new_acculumator_0_2_col316.clone(),
                new_acculumator_0_3_col317.clone(),
                new_acculumator_0_4_col318.clone(),
                new_acculumator_0_5_col319.clone(),
                new_acculumator_0_6_col320.clone(),
                new_acculumator_0_7_col321.clone(),
                new_acculumator_0_8_col322.clone(),
                new_acculumator_0_9_col323.clone(),
                new_acculumator_0_10_col324.clone(),
                new_acculumator_0_11_col325.clone(),
                new_acculumator_0_12_col326.clone(),
                new_acculumator_0_13_col327.clone(),
                new_acculumator_0_14_col328.clone(),
                new_acculumator_0_15_col329.clone(),
                new_acculumator_0_16_col330.clone(),
                new_acculumator_0_17_col331.clone(),
                new_acculumator_0_18_col332.clone(),
                new_acculumator_0_19_col333.clone(),
                new_acculumator_0_20_col334.clone(),
                new_acculumator_0_21_col335.clone(),
                new_acculumator_0_22_col336.clone(),
                new_acculumator_0_23_col337.clone(),
                new_acculumator_0_24_col338.clone(),
                new_acculumator_0_25_col339.clone(),
                new_acculumator_0_26_col340.clone(),
                new_acculumator_0_27_col341.clone(),
                new_acculumator_1_0_col342.clone(),
                new_acculumator_1_1_col343.clone(),
                new_acculumator_1_2_col344.clone(),
                new_acculumator_1_3_col345.clone(),
                new_acculumator_1_4_col346.clone(),
                new_acculumator_1_5_col347.clone(),
                new_acculumator_1_6_col348.clone(),
                new_acculumator_1_7_col349.clone(),
                new_acculumator_1_8_col350.clone(),
                new_acculumator_1_9_col351.clone(),
                new_acculumator_1_10_col352.clone(),
                new_acculumator_1_11_col353.clone(),
                new_acculumator_1_12_col354.clone(),
                new_acculumator_1_13_col355.clone(),
                new_acculumator_1_14_col356.clone(),
                new_acculumator_1_15_col357.clone(),
                new_acculumator_1_16_col358.clone(),
                new_acculumator_1_17_col359.clone(),
                new_acculumator_1_18_col360.clone(),
                new_acculumator_1_19_col361.clone(),
                new_acculumator_1_20_col362.clone(),
                new_acculumator_1_21_col363.clone(),
                new_acculumator_1_22_col364.clone(),
                new_acculumator_1_23_col365.clone(),
                new_acculumator_1_24_col366.clone(),
                new_acculumator_1_25_col367.clone(),
                new_acculumator_1_26_col368.clone(),
                new_acculumator_1_27_col369.clone(),
                next_counter_col138.clone(),
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
    use crate::components::constraints_regression_test_values::PARTIAL_EC_MUL_GENERIC;

    #[test]
    fn partial_ec_mul_generic_constraints_regression() {
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

        PARTIAL_EC_MUL_GENERIC.assert_debug_eq(&sum);
    }
}
