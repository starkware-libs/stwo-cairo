// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::double_karatsuba_b1daa::DoubleKaratsubaB1Daa;
use crate::components::subroutines::mod_utils::ModUtils;
use crate::components::subroutines::mod_words_to_12_bit_array::ModWordsTo12BitArray;

pub const N_TRACE_COLUMNS: usize = 426;
pub const RELATION_USES_PER_ROW: [RelationUse; 5] = [
    RelationUse {
        relation_id: "MemoryAddressToId",
        uses: 29,
    },
    RelationUse {
        relation_id: "MemoryIdToBig",
        uses: 24,
    },
    RelationUse {
        relation_id: "RangeCheck_12",
        uses: 32,
    },
    RelationUse {
        relation_id: "RangeCheck_18",
        uses: 62,
    },
    RelationUse {
        relation_id: "RangeCheck_3_6_6_3",
        uses: 40,
    },
];

pub struct Eval {
    pub claim: Claim,
    pub memory_address_to_id_lookup_elements: relations::MemoryAddressToId,
    pub memory_id_to_big_lookup_elements: relations::MemoryIdToBig,
    pub range_check_12_lookup_elements: relations::RangeCheck_12,
    pub range_check_3_6_6_3_lookup_elements: relations::RangeCheck_3_6_6_3,
    pub range_check_18_lookup_elements: relations::RangeCheck_18,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct Claim {
    pub log_size: u32,
    pub mul_mod_builtin_segment_start: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 94];
        TreeVec::new(vec![vec![], trace_log_sizes, interaction_log_sizes])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(self.log_size as u64);
        channel.mix_u64(self.mul_mod_builtin_segment_start as u64);
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
        let M31_131072 = E::F::from(M31::from(131072));
        let M31_524288 = E::F::from(M31::from(524288));
        let seq = eval.get_preprocessed_column(Seq::new(self.log_size()).id());
        let is_instance_0_col0 = eval.next_trace_mask();
        let p0_id_col1 = eval.next_trace_mask();
        let p0_limb_0_col2 = eval.next_trace_mask();
        let p0_limb_1_col3 = eval.next_trace_mask();
        let p0_limb_2_col4 = eval.next_trace_mask();
        let p0_limb_3_col5 = eval.next_trace_mask();
        let p0_limb_4_col6 = eval.next_trace_mask();
        let p0_limb_5_col7 = eval.next_trace_mask();
        let p0_limb_6_col8 = eval.next_trace_mask();
        let p0_limb_7_col9 = eval.next_trace_mask();
        let p0_limb_8_col10 = eval.next_trace_mask();
        let p0_limb_9_col11 = eval.next_trace_mask();
        let p0_limb_10_col12 = eval.next_trace_mask();
        let p1_id_col13 = eval.next_trace_mask();
        let p1_limb_0_col14 = eval.next_trace_mask();
        let p1_limb_1_col15 = eval.next_trace_mask();
        let p1_limb_2_col16 = eval.next_trace_mask();
        let p1_limb_3_col17 = eval.next_trace_mask();
        let p1_limb_4_col18 = eval.next_trace_mask();
        let p1_limb_5_col19 = eval.next_trace_mask();
        let p1_limb_6_col20 = eval.next_trace_mask();
        let p1_limb_7_col21 = eval.next_trace_mask();
        let p1_limb_8_col22 = eval.next_trace_mask();
        let p1_limb_9_col23 = eval.next_trace_mask();
        let p1_limb_10_col24 = eval.next_trace_mask();
        let p2_id_col25 = eval.next_trace_mask();
        let p2_limb_0_col26 = eval.next_trace_mask();
        let p2_limb_1_col27 = eval.next_trace_mask();
        let p2_limb_2_col28 = eval.next_trace_mask();
        let p2_limb_3_col29 = eval.next_trace_mask();
        let p2_limb_4_col30 = eval.next_trace_mask();
        let p2_limb_5_col31 = eval.next_trace_mask();
        let p2_limb_6_col32 = eval.next_trace_mask();
        let p2_limb_7_col33 = eval.next_trace_mask();
        let p2_limb_8_col34 = eval.next_trace_mask();
        let p2_limb_9_col35 = eval.next_trace_mask();
        let p2_limb_10_col36 = eval.next_trace_mask();
        let p3_id_col37 = eval.next_trace_mask();
        let p3_limb_0_col38 = eval.next_trace_mask();
        let p3_limb_1_col39 = eval.next_trace_mask();
        let p3_limb_2_col40 = eval.next_trace_mask();
        let p3_limb_3_col41 = eval.next_trace_mask();
        let p3_limb_4_col42 = eval.next_trace_mask();
        let p3_limb_5_col43 = eval.next_trace_mask();
        let p3_limb_6_col44 = eval.next_trace_mask();
        let p3_limb_7_col45 = eval.next_trace_mask();
        let p3_limb_8_col46 = eval.next_trace_mask();
        let p3_limb_9_col47 = eval.next_trace_mask();
        let p3_limb_10_col48 = eval.next_trace_mask();
        let values_ptr_id_col49 = eval.next_trace_mask();
        let values_ptr_limb_0_col50 = eval.next_trace_mask();
        let values_ptr_limb_1_col51 = eval.next_trace_mask();
        let values_ptr_limb_2_col52 = eval.next_trace_mask();
        let values_ptr_limb_3_col53 = eval.next_trace_mask();
        let partial_limb_msb_col54 = eval.next_trace_mask();
        let offsets_ptr_id_col55 = eval.next_trace_mask();
        let offsets_ptr_limb_0_col56 = eval.next_trace_mask();
        let offsets_ptr_limb_1_col57 = eval.next_trace_mask();
        let offsets_ptr_limb_2_col58 = eval.next_trace_mask();
        let offsets_ptr_limb_3_col59 = eval.next_trace_mask();
        let partial_limb_msb_col60 = eval.next_trace_mask();
        let offsets_ptr_prev_id_col61 = eval.next_trace_mask();
        let offsets_ptr_prev_limb_0_col62 = eval.next_trace_mask();
        let offsets_ptr_prev_limb_1_col63 = eval.next_trace_mask();
        let offsets_ptr_prev_limb_2_col64 = eval.next_trace_mask();
        let offsets_ptr_prev_limb_3_col65 = eval.next_trace_mask();
        let partial_limb_msb_col66 = eval.next_trace_mask();
        let n_id_col67 = eval.next_trace_mask();
        let n_limb_0_col68 = eval.next_trace_mask();
        let n_limb_1_col69 = eval.next_trace_mask();
        let n_limb_2_col70 = eval.next_trace_mask();
        let n_limb_3_col71 = eval.next_trace_mask();
        let partial_limb_msb_col72 = eval.next_trace_mask();
        let n_prev_id_col73 = eval.next_trace_mask();
        let n_prev_limb_0_col74 = eval.next_trace_mask();
        let n_prev_limb_1_col75 = eval.next_trace_mask();
        let n_prev_limb_2_col76 = eval.next_trace_mask();
        let n_prev_limb_3_col77 = eval.next_trace_mask();
        let partial_limb_msb_col78 = eval.next_trace_mask();
        let values_ptr_prev_id_col79 = eval.next_trace_mask();
        let p_prev0_id_col80 = eval.next_trace_mask();
        let p_prev1_id_col81 = eval.next_trace_mask();
        let p_prev2_id_col82 = eval.next_trace_mask();
        let p_prev3_id_col83 = eval.next_trace_mask();
        let offsets_a_id_col84 = eval.next_trace_mask();
        let msb_col85 = eval.next_trace_mask();
        let mid_limbs_set_col86 = eval.next_trace_mask();
        let offsets_a_limb_0_col87 = eval.next_trace_mask();
        let offsets_a_limb_1_col88 = eval.next_trace_mask();
        let offsets_a_limb_2_col89 = eval.next_trace_mask();
        let remainder_bits_col90 = eval.next_trace_mask();
        let partial_limb_msb_col91 = eval.next_trace_mask();
        let offsets_b_id_col92 = eval.next_trace_mask();
        let msb_col93 = eval.next_trace_mask();
        let mid_limbs_set_col94 = eval.next_trace_mask();
        let offsets_b_limb_0_col95 = eval.next_trace_mask();
        let offsets_b_limb_1_col96 = eval.next_trace_mask();
        let offsets_b_limb_2_col97 = eval.next_trace_mask();
        let remainder_bits_col98 = eval.next_trace_mask();
        let partial_limb_msb_col99 = eval.next_trace_mask();
        let offsets_c_id_col100 = eval.next_trace_mask();
        let msb_col101 = eval.next_trace_mask();
        let mid_limbs_set_col102 = eval.next_trace_mask();
        let offsets_c_limb_0_col103 = eval.next_trace_mask();
        let offsets_c_limb_1_col104 = eval.next_trace_mask();
        let offsets_c_limb_2_col105 = eval.next_trace_mask();
        let remainder_bits_col106 = eval.next_trace_mask();
        let partial_limb_msb_col107 = eval.next_trace_mask();
        let a0_id_col108 = eval.next_trace_mask();
        let a0_limb_0_col109 = eval.next_trace_mask();
        let a0_limb_1_col110 = eval.next_trace_mask();
        let a0_limb_2_col111 = eval.next_trace_mask();
        let a0_limb_3_col112 = eval.next_trace_mask();
        let a0_limb_4_col113 = eval.next_trace_mask();
        let a0_limb_5_col114 = eval.next_trace_mask();
        let a0_limb_6_col115 = eval.next_trace_mask();
        let a0_limb_7_col116 = eval.next_trace_mask();
        let a0_limb_8_col117 = eval.next_trace_mask();
        let a0_limb_9_col118 = eval.next_trace_mask();
        let a0_limb_10_col119 = eval.next_trace_mask();
        let a1_id_col120 = eval.next_trace_mask();
        let a1_limb_0_col121 = eval.next_trace_mask();
        let a1_limb_1_col122 = eval.next_trace_mask();
        let a1_limb_2_col123 = eval.next_trace_mask();
        let a1_limb_3_col124 = eval.next_trace_mask();
        let a1_limb_4_col125 = eval.next_trace_mask();
        let a1_limb_5_col126 = eval.next_trace_mask();
        let a1_limb_6_col127 = eval.next_trace_mask();
        let a1_limb_7_col128 = eval.next_trace_mask();
        let a1_limb_8_col129 = eval.next_trace_mask();
        let a1_limb_9_col130 = eval.next_trace_mask();
        let a1_limb_10_col131 = eval.next_trace_mask();
        let a2_id_col132 = eval.next_trace_mask();
        let a2_limb_0_col133 = eval.next_trace_mask();
        let a2_limb_1_col134 = eval.next_trace_mask();
        let a2_limb_2_col135 = eval.next_trace_mask();
        let a2_limb_3_col136 = eval.next_trace_mask();
        let a2_limb_4_col137 = eval.next_trace_mask();
        let a2_limb_5_col138 = eval.next_trace_mask();
        let a2_limb_6_col139 = eval.next_trace_mask();
        let a2_limb_7_col140 = eval.next_trace_mask();
        let a2_limb_8_col141 = eval.next_trace_mask();
        let a2_limb_9_col142 = eval.next_trace_mask();
        let a2_limb_10_col143 = eval.next_trace_mask();
        let a3_id_col144 = eval.next_trace_mask();
        let a3_limb_0_col145 = eval.next_trace_mask();
        let a3_limb_1_col146 = eval.next_trace_mask();
        let a3_limb_2_col147 = eval.next_trace_mask();
        let a3_limb_3_col148 = eval.next_trace_mask();
        let a3_limb_4_col149 = eval.next_trace_mask();
        let a3_limb_5_col150 = eval.next_trace_mask();
        let a3_limb_6_col151 = eval.next_trace_mask();
        let a3_limb_7_col152 = eval.next_trace_mask();
        let a3_limb_8_col153 = eval.next_trace_mask();
        let a3_limb_9_col154 = eval.next_trace_mask();
        let a3_limb_10_col155 = eval.next_trace_mask();
        let b0_id_col156 = eval.next_trace_mask();
        let b0_limb_0_col157 = eval.next_trace_mask();
        let b0_limb_1_col158 = eval.next_trace_mask();
        let b0_limb_2_col159 = eval.next_trace_mask();
        let b0_limb_3_col160 = eval.next_trace_mask();
        let b0_limb_4_col161 = eval.next_trace_mask();
        let b0_limb_5_col162 = eval.next_trace_mask();
        let b0_limb_6_col163 = eval.next_trace_mask();
        let b0_limb_7_col164 = eval.next_trace_mask();
        let b0_limb_8_col165 = eval.next_trace_mask();
        let b0_limb_9_col166 = eval.next_trace_mask();
        let b0_limb_10_col167 = eval.next_trace_mask();
        let b1_id_col168 = eval.next_trace_mask();
        let b1_limb_0_col169 = eval.next_trace_mask();
        let b1_limb_1_col170 = eval.next_trace_mask();
        let b1_limb_2_col171 = eval.next_trace_mask();
        let b1_limb_3_col172 = eval.next_trace_mask();
        let b1_limb_4_col173 = eval.next_trace_mask();
        let b1_limb_5_col174 = eval.next_trace_mask();
        let b1_limb_6_col175 = eval.next_trace_mask();
        let b1_limb_7_col176 = eval.next_trace_mask();
        let b1_limb_8_col177 = eval.next_trace_mask();
        let b1_limb_9_col178 = eval.next_trace_mask();
        let b1_limb_10_col179 = eval.next_trace_mask();
        let b2_id_col180 = eval.next_trace_mask();
        let b2_limb_0_col181 = eval.next_trace_mask();
        let b2_limb_1_col182 = eval.next_trace_mask();
        let b2_limb_2_col183 = eval.next_trace_mask();
        let b2_limb_3_col184 = eval.next_trace_mask();
        let b2_limb_4_col185 = eval.next_trace_mask();
        let b2_limb_5_col186 = eval.next_trace_mask();
        let b2_limb_6_col187 = eval.next_trace_mask();
        let b2_limb_7_col188 = eval.next_trace_mask();
        let b2_limb_8_col189 = eval.next_trace_mask();
        let b2_limb_9_col190 = eval.next_trace_mask();
        let b2_limb_10_col191 = eval.next_trace_mask();
        let b3_id_col192 = eval.next_trace_mask();
        let b3_limb_0_col193 = eval.next_trace_mask();
        let b3_limb_1_col194 = eval.next_trace_mask();
        let b3_limb_2_col195 = eval.next_trace_mask();
        let b3_limb_3_col196 = eval.next_trace_mask();
        let b3_limb_4_col197 = eval.next_trace_mask();
        let b3_limb_5_col198 = eval.next_trace_mask();
        let b3_limb_6_col199 = eval.next_trace_mask();
        let b3_limb_7_col200 = eval.next_trace_mask();
        let b3_limb_8_col201 = eval.next_trace_mask();
        let b3_limb_9_col202 = eval.next_trace_mask();
        let b3_limb_10_col203 = eval.next_trace_mask();
        let c0_id_col204 = eval.next_trace_mask();
        let c0_limb_0_col205 = eval.next_trace_mask();
        let c0_limb_1_col206 = eval.next_trace_mask();
        let c0_limb_2_col207 = eval.next_trace_mask();
        let c0_limb_3_col208 = eval.next_trace_mask();
        let c0_limb_4_col209 = eval.next_trace_mask();
        let c0_limb_5_col210 = eval.next_trace_mask();
        let c0_limb_6_col211 = eval.next_trace_mask();
        let c0_limb_7_col212 = eval.next_trace_mask();
        let c0_limb_8_col213 = eval.next_trace_mask();
        let c0_limb_9_col214 = eval.next_trace_mask();
        let c0_limb_10_col215 = eval.next_trace_mask();
        let c1_id_col216 = eval.next_trace_mask();
        let c1_limb_0_col217 = eval.next_trace_mask();
        let c1_limb_1_col218 = eval.next_trace_mask();
        let c1_limb_2_col219 = eval.next_trace_mask();
        let c1_limb_3_col220 = eval.next_trace_mask();
        let c1_limb_4_col221 = eval.next_trace_mask();
        let c1_limb_5_col222 = eval.next_trace_mask();
        let c1_limb_6_col223 = eval.next_trace_mask();
        let c1_limb_7_col224 = eval.next_trace_mask();
        let c1_limb_8_col225 = eval.next_trace_mask();
        let c1_limb_9_col226 = eval.next_trace_mask();
        let c1_limb_10_col227 = eval.next_trace_mask();
        let c2_id_col228 = eval.next_trace_mask();
        let c2_limb_0_col229 = eval.next_trace_mask();
        let c2_limb_1_col230 = eval.next_trace_mask();
        let c2_limb_2_col231 = eval.next_trace_mask();
        let c2_limb_3_col232 = eval.next_trace_mask();
        let c2_limb_4_col233 = eval.next_trace_mask();
        let c2_limb_5_col234 = eval.next_trace_mask();
        let c2_limb_6_col235 = eval.next_trace_mask();
        let c2_limb_7_col236 = eval.next_trace_mask();
        let c2_limb_8_col237 = eval.next_trace_mask();
        let c2_limb_9_col238 = eval.next_trace_mask();
        let c2_limb_10_col239 = eval.next_trace_mask();
        let c3_id_col240 = eval.next_trace_mask();
        let c3_limb_0_col241 = eval.next_trace_mask();
        let c3_limb_1_col242 = eval.next_trace_mask();
        let c3_limb_2_col243 = eval.next_trace_mask();
        let c3_limb_3_col244 = eval.next_trace_mask();
        let c3_limb_4_col245 = eval.next_trace_mask();
        let c3_limb_5_col246 = eval.next_trace_mask();
        let c3_limb_6_col247 = eval.next_trace_mask();
        let c3_limb_7_col248 = eval.next_trace_mask();
        let c3_limb_8_col249 = eval.next_trace_mask();
        let c3_limb_9_col250 = eval.next_trace_mask();
        let c3_limb_10_col251 = eval.next_trace_mask();
        let ab_minus_c_div_p_limb_0_col252 = eval.next_trace_mask();
        let ab_minus_c_div_p_limb_1_col253 = eval.next_trace_mask();
        let ab_minus_c_div_p_limb_2_col254 = eval.next_trace_mask();
        let ab_minus_c_div_p_limb_3_col255 = eval.next_trace_mask();
        let ab_minus_c_div_p_limb_4_col256 = eval.next_trace_mask();
        let ab_minus_c_div_p_limb_5_col257 = eval.next_trace_mask();
        let ab_minus_c_div_p_limb_6_col258 = eval.next_trace_mask();
        let ab_minus_c_div_p_limb_7_col259 = eval.next_trace_mask();
        let ab_minus_c_div_p_limb_8_col260 = eval.next_trace_mask();
        let ab_minus_c_div_p_limb_9_col261 = eval.next_trace_mask();
        let ab_minus_c_div_p_limb_10_col262 = eval.next_trace_mask();
        let ab_minus_c_div_p_limb_11_col263 = eval.next_trace_mask();
        let ab_minus_c_div_p_limb_12_col264 = eval.next_trace_mask();
        let ab_minus_c_div_p_limb_13_col265 = eval.next_trace_mask();
        let ab_minus_c_div_p_limb_14_col266 = eval.next_trace_mask();
        let ab_minus_c_div_p_limb_15_col267 = eval.next_trace_mask();
        let ab_minus_c_div_p_limb_16_col268 = eval.next_trace_mask();
        let ab_minus_c_div_p_limb_17_col269 = eval.next_trace_mask();
        let ab_minus_c_div_p_limb_18_col270 = eval.next_trace_mask();
        let ab_minus_c_div_p_limb_19_col271 = eval.next_trace_mask();
        let ab_minus_c_div_p_limb_20_col272 = eval.next_trace_mask();
        let ab_minus_c_div_p_limb_21_col273 = eval.next_trace_mask();
        let ab_minus_c_div_p_limb_22_col274 = eval.next_trace_mask();
        let ab_minus_c_div_p_limb_23_col275 = eval.next_trace_mask();
        let ab_minus_c_div_p_limb_24_col276 = eval.next_trace_mask();
        let ab_minus_c_div_p_limb_25_col277 = eval.next_trace_mask();
        let ab_minus_c_div_p_limb_26_col278 = eval.next_trace_mask();
        let ab_minus_c_div_p_limb_27_col279 = eval.next_trace_mask();
        let ab_minus_c_div_p_limb_28_col280 = eval.next_trace_mask();
        let ab_minus_c_div_p_limb_29_col281 = eval.next_trace_mask();
        let ab_minus_c_div_p_limb_30_col282 = eval.next_trace_mask();
        let ab_minus_c_div_p_limb_31_col283 = eval.next_trace_mask();
        let limb1b_0_col284 = eval.next_trace_mask();
        let limb2b_0_col285 = eval.next_trace_mask();
        let limb5b_0_col286 = eval.next_trace_mask();
        let limb6b_0_col287 = eval.next_trace_mask();
        let limb9b_0_col288 = eval.next_trace_mask();
        let limb1b_1_col289 = eval.next_trace_mask();
        let limb2b_1_col290 = eval.next_trace_mask();
        let limb5b_1_col291 = eval.next_trace_mask();
        let limb6b_1_col292 = eval.next_trace_mask();
        let limb9b_1_col293 = eval.next_trace_mask();
        let limb1b_0_col294 = eval.next_trace_mask();
        let limb2b_0_col295 = eval.next_trace_mask();
        let limb5b_0_col296 = eval.next_trace_mask();
        let limb6b_0_col297 = eval.next_trace_mask();
        let limb9b_0_col298 = eval.next_trace_mask();
        let limb1b_1_col299 = eval.next_trace_mask();
        let limb2b_1_col300 = eval.next_trace_mask();
        let limb5b_1_col301 = eval.next_trace_mask();
        let limb6b_1_col302 = eval.next_trace_mask();
        let limb9b_1_col303 = eval.next_trace_mask();
        let limb1b_0_col304 = eval.next_trace_mask();
        let limb2b_0_col305 = eval.next_trace_mask();
        let limb5b_0_col306 = eval.next_trace_mask();
        let limb6b_0_col307 = eval.next_trace_mask();
        let limb9b_0_col308 = eval.next_trace_mask();
        let limb1b_1_col309 = eval.next_trace_mask();
        let limb2b_1_col310 = eval.next_trace_mask();
        let limb5b_1_col311 = eval.next_trace_mask();
        let limb6b_1_col312 = eval.next_trace_mask();
        let limb9b_1_col313 = eval.next_trace_mask();
        let limb1b_0_col314 = eval.next_trace_mask();
        let limb2b_0_col315 = eval.next_trace_mask();
        let limb5b_0_col316 = eval.next_trace_mask();
        let limb6b_0_col317 = eval.next_trace_mask();
        let limb9b_0_col318 = eval.next_trace_mask();
        let limb1b_1_col319 = eval.next_trace_mask();
        let limb2b_1_col320 = eval.next_trace_mask();
        let limb5b_1_col321 = eval.next_trace_mask();
        let limb6b_1_col322 = eval.next_trace_mask();
        let limb9b_1_col323 = eval.next_trace_mask();
        let limb1b_0_col324 = eval.next_trace_mask();
        let limb2b_0_col325 = eval.next_trace_mask();
        let limb5b_0_col326 = eval.next_trace_mask();
        let limb6b_0_col327 = eval.next_trace_mask();
        let limb9b_0_col328 = eval.next_trace_mask();
        let limb1b_1_col329 = eval.next_trace_mask();
        let limb2b_1_col330 = eval.next_trace_mask();
        let limb5b_1_col331 = eval.next_trace_mask();
        let limb6b_1_col332 = eval.next_trace_mask();
        let limb9b_1_col333 = eval.next_trace_mask();
        let limb1b_0_col334 = eval.next_trace_mask();
        let limb2b_0_col335 = eval.next_trace_mask();
        let limb5b_0_col336 = eval.next_trace_mask();
        let limb6b_0_col337 = eval.next_trace_mask();
        let limb9b_0_col338 = eval.next_trace_mask();
        let limb1b_1_col339 = eval.next_trace_mask();
        let limb2b_1_col340 = eval.next_trace_mask();
        let limb5b_1_col341 = eval.next_trace_mask();
        let limb6b_1_col342 = eval.next_trace_mask();
        let limb9b_1_col343 = eval.next_trace_mask();
        let limb1b_0_col344 = eval.next_trace_mask();
        let limb2b_0_col345 = eval.next_trace_mask();
        let limb5b_0_col346 = eval.next_trace_mask();
        let limb6b_0_col347 = eval.next_trace_mask();
        let limb9b_0_col348 = eval.next_trace_mask();
        let limb1b_1_col349 = eval.next_trace_mask();
        let limb2b_1_col350 = eval.next_trace_mask();
        let limb5b_1_col351 = eval.next_trace_mask();
        let limb6b_1_col352 = eval.next_trace_mask();
        let limb9b_1_col353 = eval.next_trace_mask();
        let limb1b_0_col354 = eval.next_trace_mask();
        let limb2b_0_col355 = eval.next_trace_mask();
        let limb5b_0_col356 = eval.next_trace_mask();
        let limb6b_0_col357 = eval.next_trace_mask();
        let limb9b_0_col358 = eval.next_trace_mask();
        let limb1b_1_col359 = eval.next_trace_mask();
        let limb2b_1_col360 = eval.next_trace_mask();
        let limb5b_1_col361 = eval.next_trace_mask();
        let limb6b_1_col362 = eval.next_trace_mask();
        let limb9b_1_col363 = eval.next_trace_mask();
        let carry_0_col364 = eval.next_trace_mask();
        let carry_1_col365 = eval.next_trace_mask();
        let carry_2_col366 = eval.next_trace_mask();
        let carry_3_col367 = eval.next_trace_mask();
        let carry_4_col368 = eval.next_trace_mask();
        let carry_5_col369 = eval.next_trace_mask();
        let carry_6_col370 = eval.next_trace_mask();
        let carry_7_col371 = eval.next_trace_mask();
        let carry_8_col372 = eval.next_trace_mask();
        let carry_9_col373 = eval.next_trace_mask();
        let carry_10_col374 = eval.next_trace_mask();
        let carry_11_col375 = eval.next_trace_mask();
        let carry_12_col376 = eval.next_trace_mask();
        let carry_13_col377 = eval.next_trace_mask();
        let carry_14_col378 = eval.next_trace_mask();
        let carry_15_col379 = eval.next_trace_mask();
        let carry_16_col380 = eval.next_trace_mask();
        let carry_17_col381 = eval.next_trace_mask();
        let carry_18_col382 = eval.next_trace_mask();
        let carry_19_col383 = eval.next_trace_mask();
        let carry_20_col384 = eval.next_trace_mask();
        let carry_21_col385 = eval.next_trace_mask();
        let carry_22_col386 = eval.next_trace_mask();
        let carry_23_col387 = eval.next_trace_mask();
        let carry_24_col388 = eval.next_trace_mask();
        let carry_25_col389 = eval.next_trace_mask();
        let carry_26_col390 = eval.next_trace_mask();
        let carry_27_col391 = eval.next_trace_mask();
        let carry_28_col392 = eval.next_trace_mask();
        let carry_29_col393 = eval.next_trace_mask();
        let carry_30_col394 = eval.next_trace_mask();
        let carry_31_col395 = eval.next_trace_mask();
        let carry_32_col396 = eval.next_trace_mask();
        let carry_33_col397 = eval.next_trace_mask();
        let carry_34_col398 = eval.next_trace_mask();
        let carry_35_col399 = eval.next_trace_mask();
        let carry_36_col400 = eval.next_trace_mask();
        let carry_37_col401 = eval.next_trace_mask();
        let carry_38_col402 = eval.next_trace_mask();
        let carry_39_col403 = eval.next_trace_mask();
        let carry_40_col404 = eval.next_trace_mask();
        let carry_41_col405 = eval.next_trace_mask();
        let carry_42_col406 = eval.next_trace_mask();
        let carry_43_col407 = eval.next_trace_mask();
        let carry_44_col408 = eval.next_trace_mask();
        let carry_45_col409 = eval.next_trace_mask();
        let carry_46_col410 = eval.next_trace_mask();
        let carry_47_col411 = eval.next_trace_mask();
        let carry_48_col412 = eval.next_trace_mask();
        let carry_49_col413 = eval.next_trace_mask();
        let carry_50_col414 = eval.next_trace_mask();
        let carry_51_col415 = eval.next_trace_mask();
        let carry_52_col416 = eval.next_trace_mask();
        let carry_53_col417 = eval.next_trace_mask();
        let carry_54_col418 = eval.next_trace_mask();
        let carry_55_col419 = eval.next_trace_mask();
        let carry_56_col420 = eval.next_trace_mask();
        let carry_57_col421 = eval.next_trace_mask();
        let carry_58_col422 = eval.next_trace_mask();
        let carry_59_col423 = eval.next_trace_mask();
        let carry_60_col424 = eval.next_trace_mask();
        let carry_61_col425 = eval.next_trace_mask();

        ModUtils::evaluate(
            [
                E::F::from(M31::from(self.claim.mul_mod_builtin_segment_start)),
                seq.clone(),
            ],
            is_instance_0_col0.clone(),
            p0_id_col1.clone(),
            p0_limb_0_col2.clone(),
            p0_limb_1_col3.clone(),
            p0_limb_2_col4.clone(),
            p0_limb_3_col5.clone(),
            p0_limb_4_col6.clone(),
            p0_limb_5_col7.clone(),
            p0_limb_6_col8.clone(),
            p0_limb_7_col9.clone(),
            p0_limb_8_col10.clone(),
            p0_limb_9_col11.clone(),
            p0_limb_10_col12.clone(),
            p1_id_col13.clone(),
            p1_limb_0_col14.clone(),
            p1_limb_1_col15.clone(),
            p1_limb_2_col16.clone(),
            p1_limb_3_col17.clone(),
            p1_limb_4_col18.clone(),
            p1_limb_5_col19.clone(),
            p1_limb_6_col20.clone(),
            p1_limb_7_col21.clone(),
            p1_limb_8_col22.clone(),
            p1_limb_9_col23.clone(),
            p1_limb_10_col24.clone(),
            p2_id_col25.clone(),
            p2_limb_0_col26.clone(),
            p2_limb_1_col27.clone(),
            p2_limb_2_col28.clone(),
            p2_limb_3_col29.clone(),
            p2_limb_4_col30.clone(),
            p2_limb_5_col31.clone(),
            p2_limb_6_col32.clone(),
            p2_limb_7_col33.clone(),
            p2_limb_8_col34.clone(),
            p2_limb_9_col35.clone(),
            p2_limb_10_col36.clone(),
            p3_id_col37.clone(),
            p3_limb_0_col38.clone(),
            p3_limb_1_col39.clone(),
            p3_limb_2_col40.clone(),
            p3_limb_3_col41.clone(),
            p3_limb_4_col42.clone(),
            p3_limb_5_col43.clone(),
            p3_limb_6_col44.clone(),
            p3_limb_7_col45.clone(),
            p3_limb_8_col46.clone(),
            p3_limb_9_col47.clone(),
            p3_limb_10_col48.clone(),
            values_ptr_id_col49.clone(),
            values_ptr_limb_0_col50.clone(),
            values_ptr_limb_1_col51.clone(),
            values_ptr_limb_2_col52.clone(),
            values_ptr_limb_3_col53.clone(),
            partial_limb_msb_col54.clone(),
            offsets_ptr_id_col55.clone(),
            offsets_ptr_limb_0_col56.clone(),
            offsets_ptr_limb_1_col57.clone(),
            offsets_ptr_limb_2_col58.clone(),
            offsets_ptr_limb_3_col59.clone(),
            partial_limb_msb_col60.clone(),
            offsets_ptr_prev_id_col61.clone(),
            offsets_ptr_prev_limb_0_col62.clone(),
            offsets_ptr_prev_limb_1_col63.clone(),
            offsets_ptr_prev_limb_2_col64.clone(),
            offsets_ptr_prev_limb_3_col65.clone(),
            partial_limb_msb_col66.clone(),
            n_id_col67.clone(),
            n_limb_0_col68.clone(),
            n_limb_1_col69.clone(),
            n_limb_2_col70.clone(),
            n_limb_3_col71.clone(),
            partial_limb_msb_col72.clone(),
            n_prev_id_col73.clone(),
            n_prev_limb_0_col74.clone(),
            n_prev_limb_1_col75.clone(),
            n_prev_limb_2_col76.clone(),
            n_prev_limb_3_col77.clone(),
            partial_limb_msb_col78.clone(),
            values_ptr_prev_id_col79.clone(),
            p_prev0_id_col80.clone(),
            p_prev1_id_col81.clone(),
            p_prev2_id_col82.clone(),
            p_prev3_id_col83.clone(),
            offsets_a_id_col84.clone(),
            msb_col85.clone(),
            mid_limbs_set_col86.clone(),
            offsets_a_limb_0_col87.clone(),
            offsets_a_limb_1_col88.clone(),
            offsets_a_limb_2_col89.clone(),
            remainder_bits_col90.clone(),
            partial_limb_msb_col91.clone(),
            offsets_b_id_col92.clone(),
            msb_col93.clone(),
            mid_limbs_set_col94.clone(),
            offsets_b_limb_0_col95.clone(),
            offsets_b_limb_1_col96.clone(),
            offsets_b_limb_2_col97.clone(),
            remainder_bits_col98.clone(),
            partial_limb_msb_col99.clone(),
            offsets_c_id_col100.clone(),
            msb_col101.clone(),
            mid_limbs_set_col102.clone(),
            offsets_c_limb_0_col103.clone(),
            offsets_c_limb_1_col104.clone(),
            offsets_c_limb_2_col105.clone(),
            remainder_bits_col106.clone(),
            partial_limb_msb_col107.clone(),
            a0_id_col108.clone(),
            a0_limb_0_col109.clone(),
            a0_limb_1_col110.clone(),
            a0_limb_2_col111.clone(),
            a0_limb_3_col112.clone(),
            a0_limb_4_col113.clone(),
            a0_limb_5_col114.clone(),
            a0_limb_6_col115.clone(),
            a0_limb_7_col116.clone(),
            a0_limb_8_col117.clone(),
            a0_limb_9_col118.clone(),
            a0_limb_10_col119.clone(),
            a1_id_col120.clone(),
            a1_limb_0_col121.clone(),
            a1_limb_1_col122.clone(),
            a1_limb_2_col123.clone(),
            a1_limb_3_col124.clone(),
            a1_limb_4_col125.clone(),
            a1_limb_5_col126.clone(),
            a1_limb_6_col127.clone(),
            a1_limb_7_col128.clone(),
            a1_limb_8_col129.clone(),
            a1_limb_9_col130.clone(),
            a1_limb_10_col131.clone(),
            a2_id_col132.clone(),
            a2_limb_0_col133.clone(),
            a2_limb_1_col134.clone(),
            a2_limb_2_col135.clone(),
            a2_limb_3_col136.clone(),
            a2_limb_4_col137.clone(),
            a2_limb_5_col138.clone(),
            a2_limb_6_col139.clone(),
            a2_limb_7_col140.clone(),
            a2_limb_8_col141.clone(),
            a2_limb_9_col142.clone(),
            a2_limb_10_col143.clone(),
            a3_id_col144.clone(),
            a3_limb_0_col145.clone(),
            a3_limb_1_col146.clone(),
            a3_limb_2_col147.clone(),
            a3_limb_3_col148.clone(),
            a3_limb_4_col149.clone(),
            a3_limb_5_col150.clone(),
            a3_limb_6_col151.clone(),
            a3_limb_7_col152.clone(),
            a3_limb_8_col153.clone(),
            a3_limb_9_col154.clone(),
            a3_limb_10_col155.clone(),
            b0_id_col156.clone(),
            b0_limb_0_col157.clone(),
            b0_limb_1_col158.clone(),
            b0_limb_2_col159.clone(),
            b0_limb_3_col160.clone(),
            b0_limb_4_col161.clone(),
            b0_limb_5_col162.clone(),
            b0_limb_6_col163.clone(),
            b0_limb_7_col164.clone(),
            b0_limb_8_col165.clone(),
            b0_limb_9_col166.clone(),
            b0_limb_10_col167.clone(),
            b1_id_col168.clone(),
            b1_limb_0_col169.clone(),
            b1_limb_1_col170.clone(),
            b1_limb_2_col171.clone(),
            b1_limb_3_col172.clone(),
            b1_limb_4_col173.clone(),
            b1_limb_5_col174.clone(),
            b1_limb_6_col175.clone(),
            b1_limb_7_col176.clone(),
            b1_limb_8_col177.clone(),
            b1_limb_9_col178.clone(),
            b1_limb_10_col179.clone(),
            b2_id_col180.clone(),
            b2_limb_0_col181.clone(),
            b2_limb_1_col182.clone(),
            b2_limb_2_col183.clone(),
            b2_limb_3_col184.clone(),
            b2_limb_4_col185.clone(),
            b2_limb_5_col186.clone(),
            b2_limb_6_col187.clone(),
            b2_limb_7_col188.clone(),
            b2_limb_8_col189.clone(),
            b2_limb_9_col190.clone(),
            b2_limb_10_col191.clone(),
            b3_id_col192.clone(),
            b3_limb_0_col193.clone(),
            b3_limb_1_col194.clone(),
            b3_limb_2_col195.clone(),
            b3_limb_3_col196.clone(),
            b3_limb_4_col197.clone(),
            b3_limb_5_col198.clone(),
            b3_limb_6_col199.clone(),
            b3_limb_7_col200.clone(),
            b3_limb_8_col201.clone(),
            b3_limb_9_col202.clone(),
            b3_limb_10_col203.clone(),
            c0_id_col204.clone(),
            c0_limb_0_col205.clone(),
            c0_limb_1_col206.clone(),
            c0_limb_2_col207.clone(),
            c0_limb_3_col208.clone(),
            c0_limb_4_col209.clone(),
            c0_limb_5_col210.clone(),
            c0_limb_6_col211.clone(),
            c0_limb_7_col212.clone(),
            c0_limb_8_col213.clone(),
            c0_limb_9_col214.clone(),
            c0_limb_10_col215.clone(),
            c1_id_col216.clone(),
            c1_limb_0_col217.clone(),
            c1_limb_1_col218.clone(),
            c1_limb_2_col219.clone(),
            c1_limb_3_col220.clone(),
            c1_limb_4_col221.clone(),
            c1_limb_5_col222.clone(),
            c1_limb_6_col223.clone(),
            c1_limb_7_col224.clone(),
            c1_limb_8_col225.clone(),
            c1_limb_9_col226.clone(),
            c1_limb_10_col227.clone(),
            c2_id_col228.clone(),
            c2_limb_0_col229.clone(),
            c2_limb_1_col230.clone(),
            c2_limb_2_col231.clone(),
            c2_limb_3_col232.clone(),
            c2_limb_4_col233.clone(),
            c2_limb_5_col234.clone(),
            c2_limb_6_col235.clone(),
            c2_limb_7_col236.clone(),
            c2_limb_8_col237.clone(),
            c2_limb_9_col238.clone(),
            c2_limb_10_col239.clone(),
            c3_id_col240.clone(),
            c3_limb_0_col241.clone(),
            c3_limb_1_col242.clone(),
            c3_limb_2_col243.clone(),
            c3_limb_3_col244.clone(),
            c3_limb_4_col245.clone(),
            c3_limb_5_col246.clone(),
            c3_limb_6_col247.clone(),
            c3_limb_7_col248.clone(),
            c3_limb_8_col249.clone(),
            c3_limb_9_col250.clone(),
            c3_limb_10_col251.clone(),
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_12_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&ab_minus_c_div_p_limb_0_col252),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_12_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&ab_minus_c_div_p_limb_1_col253),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_12_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&ab_minus_c_div_p_limb_2_col254),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_12_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&ab_minus_c_div_p_limb_3_col255),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_12_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&ab_minus_c_div_p_limb_4_col256),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_12_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&ab_minus_c_div_p_limb_5_col257),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_12_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&ab_minus_c_div_p_limb_6_col258),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_12_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&ab_minus_c_div_p_limb_7_col259),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_12_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&ab_minus_c_div_p_limb_8_col260),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_12_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&ab_minus_c_div_p_limb_9_col261),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_12_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&ab_minus_c_div_p_limb_10_col262),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_12_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&ab_minus_c_div_p_limb_11_col263),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_12_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&ab_minus_c_div_p_limb_12_col264),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_12_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&ab_minus_c_div_p_limb_13_col265),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_12_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&ab_minus_c_div_p_limb_14_col266),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_12_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&ab_minus_c_div_p_limb_15_col267),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_12_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&ab_minus_c_div_p_limb_16_col268),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_12_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&ab_minus_c_div_p_limb_17_col269),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_12_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&ab_minus_c_div_p_limb_18_col270),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_12_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&ab_minus_c_div_p_limb_19_col271),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_12_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&ab_minus_c_div_p_limb_20_col272),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_12_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&ab_minus_c_div_p_limb_21_col273),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_12_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&ab_minus_c_div_p_limb_22_col274),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_12_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&ab_minus_c_div_p_limb_23_col275),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_12_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&ab_minus_c_div_p_limb_24_col276),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_12_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&ab_minus_c_div_p_limb_25_col277),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_12_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&ab_minus_c_div_p_limb_26_col278),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_12_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&ab_minus_c_div_p_limb_27_col279),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_12_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&ab_minus_c_div_p_limb_28_col280),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_12_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&ab_minus_c_div_p_limb_29_col281),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_12_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&ab_minus_c_div_p_limb_30_col282),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_12_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&ab_minus_c_div_p_limb_31_col283),
        ));

        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [mod_words_to_12_bit_array_output_tmp_cf8b4_185_limb_0, mod_words_to_12_bit_array_output_tmp_cf8b4_185_limb_1, mod_words_to_12_bit_array_output_tmp_cf8b4_185_limb_2, mod_words_to_12_bit_array_output_tmp_cf8b4_185_limb_3, mod_words_to_12_bit_array_output_tmp_cf8b4_185_limb_4, mod_words_to_12_bit_array_output_tmp_cf8b4_185_limb_5, mod_words_to_12_bit_array_output_tmp_cf8b4_185_limb_6, mod_words_to_12_bit_array_output_tmp_cf8b4_185_limb_7, mod_words_to_12_bit_array_output_tmp_cf8b4_185_limb_8, mod_words_to_12_bit_array_output_tmp_cf8b4_185_limb_9, mod_words_to_12_bit_array_output_tmp_cf8b4_185_limb_10, mod_words_to_12_bit_array_output_tmp_cf8b4_185_limb_11, mod_words_to_12_bit_array_output_tmp_cf8b4_185_limb_12, mod_words_to_12_bit_array_output_tmp_cf8b4_185_limb_13, mod_words_to_12_bit_array_output_tmp_cf8b4_185_limb_14, mod_words_to_12_bit_array_output_tmp_cf8b4_185_limb_15] =
            ModWordsTo12BitArray::evaluate(
                [
                    p0_limb_0_col2.clone(),
                    p0_limb_1_col3.clone(),
                    p0_limb_2_col4.clone(),
                    p0_limb_3_col5.clone(),
                    p0_limb_4_col6.clone(),
                    p0_limb_5_col7.clone(),
                    p0_limb_6_col8.clone(),
                    p0_limb_7_col9.clone(),
                    p0_limb_8_col10.clone(),
                    p0_limb_9_col11.clone(),
                    p0_limb_10_col12.clone(),
                    p1_limb_0_col14.clone(),
                    p1_limb_1_col15.clone(),
                    p1_limb_2_col16.clone(),
                    p1_limb_3_col17.clone(),
                    p1_limb_4_col18.clone(),
                    p1_limb_5_col19.clone(),
                    p1_limb_6_col20.clone(),
                    p1_limb_7_col21.clone(),
                    p1_limb_8_col22.clone(),
                    p1_limb_9_col23.clone(),
                    p1_limb_10_col24.clone(),
                ],
                limb1b_0_col284.clone(),
                limb2b_0_col285.clone(),
                limb5b_0_col286.clone(),
                limb6b_0_col287.clone(),
                limb9b_0_col288.clone(),
                limb1b_1_col289.clone(),
                limb2b_1_col290.clone(),
                limb5b_1_col291.clone(),
                limb6b_1_col292.clone(),
                limb9b_1_col293.clone(),
                &self.range_check_3_6_6_3_lookup_elements,
                &mut eval,
            );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [mod_words_to_12_bit_array_output_tmp_cf8b4_206_limb_0, mod_words_to_12_bit_array_output_tmp_cf8b4_206_limb_1, mod_words_to_12_bit_array_output_tmp_cf8b4_206_limb_2, mod_words_to_12_bit_array_output_tmp_cf8b4_206_limb_3, mod_words_to_12_bit_array_output_tmp_cf8b4_206_limb_4, mod_words_to_12_bit_array_output_tmp_cf8b4_206_limb_5, mod_words_to_12_bit_array_output_tmp_cf8b4_206_limb_6, mod_words_to_12_bit_array_output_tmp_cf8b4_206_limb_7, mod_words_to_12_bit_array_output_tmp_cf8b4_206_limb_8, mod_words_to_12_bit_array_output_tmp_cf8b4_206_limb_9, mod_words_to_12_bit_array_output_tmp_cf8b4_206_limb_10, mod_words_to_12_bit_array_output_tmp_cf8b4_206_limb_11, mod_words_to_12_bit_array_output_tmp_cf8b4_206_limb_12, mod_words_to_12_bit_array_output_tmp_cf8b4_206_limb_13, mod_words_to_12_bit_array_output_tmp_cf8b4_206_limb_14, mod_words_to_12_bit_array_output_tmp_cf8b4_206_limb_15] =
            ModWordsTo12BitArray::evaluate(
                [
                    p2_limb_0_col26.clone(),
                    p2_limb_1_col27.clone(),
                    p2_limb_2_col28.clone(),
                    p2_limb_3_col29.clone(),
                    p2_limb_4_col30.clone(),
                    p2_limb_5_col31.clone(),
                    p2_limb_6_col32.clone(),
                    p2_limb_7_col33.clone(),
                    p2_limb_8_col34.clone(),
                    p2_limb_9_col35.clone(),
                    p2_limb_10_col36.clone(),
                    p3_limb_0_col38.clone(),
                    p3_limb_1_col39.clone(),
                    p3_limb_2_col40.clone(),
                    p3_limb_3_col41.clone(),
                    p3_limb_4_col42.clone(),
                    p3_limb_5_col43.clone(),
                    p3_limb_6_col44.clone(),
                    p3_limb_7_col45.clone(),
                    p3_limb_8_col46.clone(),
                    p3_limb_9_col47.clone(),
                    p3_limb_10_col48.clone(),
                ],
                limb1b_0_col294.clone(),
                limb2b_0_col295.clone(),
                limb5b_0_col296.clone(),
                limb6b_0_col297.clone(),
                limb9b_0_col298.clone(),
                limb1b_1_col299.clone(),
                limb2b_1_col300.clone(),
                limb5b_1_col301.clone(),
                limb6b_1_col302.clone(),
                limb9b_1_col303.clone(),
                &self.range_check_3_6_6_3_lookup_elements,
                &mut eval,
            );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [mod_words_to_12_bit_array_output_tmp_cf8b4_227_limb_0, mod_words_to_12_bit_array_output_tmp_cf8b4_227_limb_1, mod_words_to_12_bit_array_output_tmp_cf8b4_227_limb_2, mod_words_to_12_bit_array_output_tmp_cf8b4_227_limb_3, mod_words_to_12_bit_array_output_tmp_cf8b4_227_limb_4, mod_words_to_12_bit_array_output_tmp_cf8b4_227_limb_5, mod_words_to_12_bit_array_output_tmp_cf8b4_227_limb_6, mod_words_to_12_bit_array_output_tmp_cf8b4_227_limb_7, mod_words_to_12_bit_array_output_tmp_cf8b4_227_limb_8, mod_words_to_12_bit_array_output_tmp_cf8b4_227_limb_9, mod_words_to_12_bit_array_output_tmp_cf8b4_227_limb_10, mod_words_to_12_bit_array_output_tmp_cf8b4_227_limb_11, mod_words_to_12_bit_array_output_tmp_cf8b4_227_limb_12, mod_words_to_12_bit_array_output_tmp_cf8b4_227_limb_13, mod_words_to_12_bit_array_output_tmp_cf8b4_227_limb_14, mod_words_to_12_bit_array_output_tmp_cf8b4_227_limb_15] =
            ModWordsTo12BitArray::evaluate(
                [
                    a0_limb_0_col109.clone(),
                    a0_limb_1_col110.clone(),
                    a0_limb_2_col111.clone(),
                    a0_limb_3_col112.clone(),
                    a0_limb_4_col113.clone(),
                    a0_limb_5_col114.clone(),
                    a0_limb_6_col115.clone(),
                    a0_limb_7_col116.clone(),
                    a0_limb_8_col117.clone(),
                    a0_limb_9_col118.clone(),
                    a0_limb_10_col119.clone(),
                    a1_limb_0_col121.clone(),
                    a1_limb_1_col122.clone(),
                    a1_limb_2_col123.clone(),
                    a1_limb_3_col124.clone(),
                    a1_limb_4_col125.clone(),
                    a1_limb_5_col126.clone(),
                    a1_limb_6_col127.clone(),
                    a1_limb_7_col128.clone(),
                    a1_limb_8_col129.clone(),
                    a1_limb_9_col130.clone(),
                    a1_limb_10_col131.clone(),
                ],
                limb1b_0_col304.clone(),
                limb2b_0_col305.clone(),
                limb5b_0_col306.clone(),
                limb6b_0_col307.clone(),
                limb9b_0_col308.clone(),
                limb1b_1_col309.clone(),
                limb2b_1_col310.clone(),
                limb5b_1_col311.clone(),
                limb6b_1_col312.clone(),
                limb9b_1_col313.clone(),
                &self.range_check_3_6_6_3_lookup_elements,
                &mut eval,
            );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [mod_words_to_12_bit_array_output_tmp_cf8b4_248_limb_0, mod_words_to_12_bit_array_output_tmp_cf8b4_248_limb_1, mod_words_to_12_bit_array_output_tmp_cf8b4_248_limb_2, mod_words_to_12_bit_array_output_tmp_cf8b4_248_limb_3, mod_words_to_12_bit_array_output_tmp_cf8b4_248_limb_4, mod_words_to_12_bit_array_output_tmp_cf8b4_248_limb_5, mod_words_to_12_bit_array_output_tmp_cf8b4_248_limb_6, mod_words_to_12_bit_array_output_tmp_cf8b4_248_limb_7, mod_words_to_12_bit_array_output_tmp_cf8b4_248_limb_8, mod_words_to_12_bit_array_output_tmp_cf8b4_248_limb_9, mod_words_to_12_bit_array_output_tmp_cf8b4_248_limb_10, mod_words_to_12_bit_array_output_tmp_cf8b4_248_limb_11, mod_words_to_12_bit_array_output_tmp_cf8b4_248_limb_12, mod_words_to_12_bit_array_output_tmp_cf8b4_248_limb_13, mod_words_to_12_bit_array_output_tmp_cf8b4_248_limb_14, mod_words_to_12_bit_array_output_tmp_cf8b4_248_limb_15] =
            ModWordsTo12BitArray::evaluate(
                [
                    a2_limb_0_col133.clone(),
                    a2_limb_1_col134.clone(),
                    a2_limb_2_col135.clone(),
                    a2_limb_3_col136.clone(),
                    a2_limb_4_col137.clone(),
                    a2_limb_5_col138.clone(),
                    a2_limb_6_col139.clone(),
                    a2_limb_7_col140.clone(),
                    a2_limb_8_col141.clone(),
                    a2_limb_9_col142.clone(),
                    a2_limb_10_col143.clone(),
                    a3_limb_0_col145.clone(),
                    a3_limb_1_col146.clone(),
                    a3_limb_2_col147.clone(),
                    a3_limb_3_col148.clone(),
                    a3_limb_4_col149.clone(),
                    a3_limb_5_col150.clone(),
                    a3_limb_6_col151.clone(),
                    a3_limb_7_col152.clone(),
                    a3_limb_8_col153.clone(),
                    a3_limb_9_col154.clone(),
                    a3_limb_10_col155.clone(),
                ],
                limb1b_0_col314.clone(),
                limb2b_0_col315.clone(),
                limb5b_0_col316.clone(),
                limb6b_0_col317.clone(),
                limb9b_0_col318.clone(),
                limb1b_1_col319.clone(),
                limb2b_1_col320.clone(),
                limb5b_1_col321.clone(),
                limb6b_1_col322.clone(),
                limb9b_1_col323.clone(),
                &self.range_check_3_6_6_3_lookup_elements,
                &mut eval,
            );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [mod_words_to_12_bit_array_output_tmp_cf8b4_269_limb_0, mod_words_to_12_bit_array_output_tmp_cf8b4_269_limb_1, mod_words_to_12_bit_array_output_tmp_cf8b4_269_limb_2, mod_words_to_12_bit_array_output_tmp_cf8b4_269_limb_3, mod_words_to_12_bit_array_output_tmp_cf8b4_269_limb_4, mod_words_to_12_bit_array_output_tmp_cf8b4_269_limb_5, mod_words_to_12_bit_array_output_tmp_cf8b4_269_limb_6, mod_words_to_12_bit_array_output_tmp_cf8b4_269_limb_7, mod_words_to_12_bit_array_output_tmp_cf8b4_269_limb_8, mod_words_to_12_bit_array_output_tmp_cf8b4_269_limb_9, mod_words_to_12_bit_array_output_tmp_cf8b4_269_limb_10, mod_words_to_12_bit_array_output_tmp_cf8b4_269_limb_11, mod_words_to_12_bit_array_output_tmp_cf8b4_269_limb_12, mod_words_to_12_bit_array_output_tmp_cf8b4_269_limb_13, mod_words_to_12_bit_array_output_tmp_cf8b4_269_limb_14, mod_words_to_12_bit_array_output_tmp_cf8b4_269_limb_15] =
            ModWordsTo12BitArray::evaluate(
                [
                    b0_limb_0_col157.clone(),
                    b0_limb_1_col158.clone(),
                    b0_limb_2_col159.clone(),
                    b0_limb_3_col160.clone(),
                    b0_limb_4_col161.clone(),
                    b0_limb_5_col162.clone(),
                    b0_limb_6_col163.clone(),
                    b0_limb_7_col164.clone(),
                    b0_limb_8_col165.clone(),
                    b0_limb_9_col166.clone(),
                    b0_limb_10_col167.clone(),
                    b1_limb_0_col169.clone(),
                    b1_limb_1_col170.clone(),
                    b1_limb_2_col171.clone(),
                    b1_limb_3_col172.clone(),
                    b1_limb_4_col173.clone(),
                    b1_limb_5_col174.clone(),
                    b1_limb_6_col175.clone(),
                    b1_limb_7_col176.clone(),
                    b1_limb_8_col177.clone(),
                    b1_limb_9_col178.clone(),
                    b1_limb_10_col179.clone(),
                ],
                limb1b_0_col324.clone(),
                limb2b_0_col325.clone(),
                limb5b_0_col326.clone(),
                limb6b_0_col327.clone(),
                limb9b_0_col328.clone(),
                limb1b_1_col329.clone(),
                limb2b_1_col330.clone(),
                limb5b_1_col331.clone(),
                limb6b_1_col332.clone(),
                limb9b_1_col333.clone(),
                &self.range_check_3_6_6_3_lookup_elements,
                &mut eval,
            );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [mod_words_to_12_bit_array_output_tmp_cf8b4_290_limb_0, mod_words_to_12_bit_array_output_tmp_cf8b4_290_limb_1, mod_words_to_12_bit_array_output_tmp_cf8b4_290_limb_2, mod_words_to_12_bit_array_output_tmp_cf8b4_290_limb_3, mod_words_to_12_bit_array_output_tmp_cf8b4_290_limb_4, mod_words_to_12_bit_array_output_tmp_cf8b4_290_limb_5, mod_words_to_12_bit_array_output_tmp_cf8b4_290_limb_6, mod_words_to_12_bit_array_output_tmp_cf8b4_290_limb_7, mod_words_to_12_bit_array_output_tmp_cf8b4_290_limb_8, mod_words_to_12_bit_array_output_tmp_cf8b4_290_limb_9, mod_words_to_12_bit_array_output_tmp_cf8b4_290_limb_10, mod_words_to_12_bit_array_output_tmp_cf8b4_290_limb_11, mod_words_to_12_bit_array_output_tmp_cf8b4_290_limb_12, mod_words_to_12_bit_array_output_tmp_cf8b4_290_limb_13, mod_words_to_12_bit_array_output_tmp_cf8b4_290_limb_14, mod_words_to_12_bit_array_output_tmp_cf8b4_290_limb_15] =
            ModWordsTo12BitArray::evaluate(
                [
                    b2_limb_0_col181.clone(),
                    b2_limb_1_col182.clone(),
                    b2_limb_2_col183.clone(),
                    b2_limb_3_col184.clone(),
                    b2_limb_4_col185.clone(),
                    b2_limb_5_col186.clone(),
                    b2_limb_6_col187.clone(),
                    b2_limb_7_col188.clone(),
                    b2_limb_8_col189.clone(),
                    b2_limb_9_col190.clone(),
                    b2_limb_10_col191.clone(),
                    b3_limb_0_col193.clone(),
                    b3_limb_1_col194.clone(),
                    b3_limb_2_col195.clone(),
                    b3_limb_3_col196.clone(),
                    b3_limb_4_col197.clone(),
                    b3_limb_5_col198.clone(),
                    b3_limb_6_col199.clone(),
                    b3_limb_7_col200.clone(),
                    b3_limb_8_col201.clone(),
                    b3_limb_9_col202.clone(),
                    b3_limb_10_col203.clone(),
                ],
                limb1b_0_col334.clone(),
                limb2b_0_col335.clone(),
                limb5b_0_col336.clone(),
                limb6b_0_col337.clone(),
                limb9b_0_col338.clone(),
                limb1b_1_col339.clone(),
                limb2b_1_col340.clone(),
                limb5b_1_col341.clone(),
                limb6b_1_col342.clone(),
                limb9b_1_col343.clone(),
                &self.range_check_3_6_6_3_lookup_elements,
                &mut eval,
            );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [mod_words_to_12_bit_array_output_tmp_cf8b4_311_limb_0, mod_words_to_12_bit_array_output_tmp_cf8b4_311_limb_1, mod_words_to_12_bit_array_output_tmp_cf8b4_311_limb_2, mod_words_to_12_bit_array_output_tmp_cf8b4_311_limb_3, mod_words_to_12_bit_array_output_tmp_cf8b4_311_limb_4, mod_words_to_12_bit_array_output_tmp_cf8b4_311_limb_5, mod_words_to_12_bit_array_output_tmp_cf8b4_311_limb_6, mod_words_to_12_bit_array_output_tmp_cf8b4_311_limb_7, mod_words_to_12_bit_array_output_tmp_cf8b4_311_limb_8, mod_words_to_12_bit_array_output_tmp_cf8b4_311_limb_9, mod_words_to_12_bit_array_output_tmp_cf8b4_311_limb_10, mod_words_to_12_bit_array_output_tmp_cf8b4_311_limb_11, mod_words_to_12_bit_array_output_tmp_cf8b4_311_limb_12, mod_words_to_12_bit_array_output_tmp_cf8b4_311_limb_13, mod_words_to_12_bit_array_output_tmp_cf8b4_311_limb_14, mod_words_to_12_bit_array_output_tmp_cf8b4_311_limb_15] =
            ModWordsTo12BitArray::evaluate(
                [
                    c0_limb_0_col205.clone(),
                    c0_limb_1_col206.clone(),
                    c0_limb_2_col207.clone(),
                    c0_limb_3_col208.clone(),
                    c0_limb_4_col209.clone(),
                    c0_limb_5_col210.clone(),
                    c0_limb_6_col211.clone(),
                    c0_limb_7_col212.clone(),
                    c0_limb_8_col213.clone(),
                    c0_limb_9_col214.clone(),
                    c0_limb_10_col215.clone(),
                    c1_limb_0_col217.clone(),
                    c1_limb_1_col218.clone(),
                    c1_limb_2_col219.clone(),
                    c1_limb_3_col220.clone(),
                    c1_limb_4_col221.clone(),
                    c1_limb_5_col222.clone(),
                    c1_limb_6_col223.clone(),
                    c1_limb_7_col224.clone(),
                    c1_limb_8_col225.clone(),
                    c1_limb_9_col226.clone(),
                    c1_limb_10_col227.clone(),
                ],
                limb1b_0_col344.clone(),
                limb2b_0_col345.clone(),
                limb5b_0_col346.clone(),
                limb6b_0_col347.clone(),
                limb9b_0_col348.clone(),
                limb1b_1_col349.clone(),
                limb2b_1_col350.clone(),
                limb5b_1_col351.clone(),
                limb6b_1_col352.clone(),
                limb9b_1_col353.clone(),
                &self.range_check_3_6_6_3_lookup_elements,
                &mut eval,
            );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [mod_words_to_12_bit_array_output_tmp_cf8b4_332_limb_0, mod_words_to_12_bit_array_output_tmp_cf8b4_332_limb_1, mod_words_to_12_bit_array_output_tmp_cf8b4_332_limb_2, mod_words_to_12_bit_array_output_tmp_cf8b4_332_limb_3, mod_words_to_12_bit_array_output_tmp_cf8b4_332_limb_4, mod_words_to_12_bit_array_output_tmp_cf8b4_332_limb_5, mod_words_to_12_bit_array_output_tmp_cf8b4_332_limb_6, mod_words_to_12_bit_array_output_tmp_cf8b4_332_limb_7, mod_words_to_12_bit_array_output_tmp_cf8b4_332_limb_8, mod_words_to_12_bit_array_output_tmp_cf8b4_332_limb_9, mod_words_to_12_bit_array_output_tmp_cf8b4_332_limb_10, mod_words_to_12_bit_array_output_tmp_cf8b4_332_limb_11, mod_words_to_12_bit_array_output_tmp_cf8b4_332_limb_12, mod_words_to_12_bit_array_output_tmp_cf8b4_332_limb_13, mod_words_to_12_bit_array_output_tmp_cf8b4_332_limb_14, mod_words_to_12_bit_array_output_tmp_cf8b4_332_limb_15] =
            ModWordsTo12BitArray::evaluate(
                [
                    c2_limb_0_col229.clone(),
                    c2_limb_1_col230.clone(),
                    c2_limb_2_col231.clone(),
                    c2_limb_3_col232.clone(),
                    c2_limb_4_col233.clone(),
                    c2_limb_5_col234.clone(),
                    c2_limb_6_col235.clone(),
                    c2_limb_7_col236.clone(),
                    c2_limb_8_col237.clone(),
                    c2_limb_9_col238.clone(),
                    c2_limb_10_col239.clone(),
                    c3_limb_0_col241.clone(),
                    c3_limb_1_col242.clone(),
                    c3_limb_2_col243.clone(),
                    c3_limb_3_col244.clone(),
                    c3_limb_4_col245.clone(),
                    c3_limb_5_col246.clone(),
                    c3_limb_6_col247.clone(),
                    c3_limb_7_col248.clone(),
                    c3_limb_8_col249.clone(),
                    c3_limb_9_col250.clone(),
                    c3_limb_10_col251.clone(),
                ],
                limb1b_0_col354.clone(),
                limb2b_0_col355.clone(),
                limb5b_0_col356.clone(),
                limb6b_0_col357.clone(),
                limb9b_0_col358.clone(),
                limb1b_1_col359.clone(),
                limb2b_1_col360.clone(),
                limb5b_1_col361.clone(),
                limb6b_1_col362.clone(),
                limb9b_1_col363.clone(),
                &self.range_check_3_6_6_3_lookup_elements,
                &mut eval,
            );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_0, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_1, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_2, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_3, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_4, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_5, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_6, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_7, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_8, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_9, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_10, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_11, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_12, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_13, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_14, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_15, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_16, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_17, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_18, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_19, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_20, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_21, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_22, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_23, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_24, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_25, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_26, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_27, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_28, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_29, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_30, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_31, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_32, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_33, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_34, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_35, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_36, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_37, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_38, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_39, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_40, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_41, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_42, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_43, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_44, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_45, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_46, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_47, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_48, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_49, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_50, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_51, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_52, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_53, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_54, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_55, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_56, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_57, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_58, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_59, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_60, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_61, double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_62] =
            DoubleKaratsubaB1Daa::evaluate(
                [
                    mod_words_to_12_bit_array_output_tmp_cf8b4_227_limb_0.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_227_limb_1.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_227_limb_2.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_227_limb_3.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_227_limb_4.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_227_limb_5.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_227_limb_6.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_227_limb_7.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_227_limb_8.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_227_limb_9.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_227_limb_10.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_227_limb_11.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_227_limb_12.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_227_limb_13.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_227_limb_14.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_227_limb_15.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_248_limb_0.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_248_limb_1.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_248_limb_2.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_248_limb_3.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_248_limb_4.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_248_limb_5.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_248_limb_6.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_248_limb_7.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_248_limb_8.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_248_limb_9.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_248_limb_10.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_248_limb_11.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_248_limb_12.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_248_limb_13.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_248_limb_14.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_248_limb_15.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_269_limb_0.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_269_limb_1.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_269_limb_2.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_269_limb_3.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_269_limb_4.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_269_limb_5.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_269_limb_6.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_269_limb_7.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_269_limb_8.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_269_limb_9.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_269_limb_10.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_269_limb_11.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_269_limb_12.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_269_limb_13.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_269_limb_14.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_269_limb_15.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_290_limb_0.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_290_limb_1.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_290_limb_2.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_290_limb_3.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_290_limb_4.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_290_limb_5.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_290_limb_6.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_290_limb_7.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_290_limb_8.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_290_limb_9.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_290_limb_10.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_290_limb_11.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_290_limb_12.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_290_limb_13.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_290_limb_14.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_290_limb_15.clone(),
                ],
                &mut eval,
            );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_0, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_1, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_2, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_3, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_4, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_5, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_6, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_7, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_8, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_9, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_10, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_11, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_12, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_13, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_14, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_15, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_16, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_17, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_18, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_19, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_20, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_21, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_22, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_23, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_24, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_25, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_26, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_27, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_28, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_29, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_30, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_31, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_32, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_33, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_34, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_35, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_36, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_37, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_38, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_39, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_40, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_41, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_42, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_43, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_44, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_45, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_46, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_47, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_48, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_49, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_50, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_51, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_52, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_53, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_54, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_55, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_56, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_57, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_58, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_59, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_60, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_61, double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_62] =
            DoubleKaratsubaB1Daa::evaluate(
                [
                    ab_minus_c_div_p_limb_0_col252.clone(),
                    ab_minus_c_div_p_limb_1_col253.clone(),
                    ab_minus_c_div_p_limb_2_col254.clone(),
                    ab_minus_c_div_p_limb_3_col255.clone(),
                    ab_minus_c_div_p_limb_4_col256.clone(),
                    ab_minus_c_div_p_limb_5_col257.clone(),
                    ab_minus_c_div_p_limb_6_col258.clone(),
                    ab_minus_c_div_p_limb_7_col259.clone(),
                    ab_minus_c_div_p_limb_8_col260.clone(),
                    ab_minus_c_div_p_limb_9_col261.clone(),
                    ab_minus_c_div_p_limb_10_col262.clone(),
                    ab_minus_c_div_p_limb_11_col263.clone(),
                    ab_minus_c_div_p_limb_12_col264.clone(),
                    ab_minus_c_div_p_limb_13_col265.clone(),
                    ab_minus_c_div_p_limb_14_col266.clone(),
                    ab_minus_c_div_p_limb_15_col267.clone(),
                    ab_minus_c_div_p_limb_16_col268.clone(),
                    ab_minus_c_div_p_limb_17_col269.clone(),
                    ab_minus_c_div_p_limb_18_col270.clone(),
                    ab_minus_c_div_p_limb_19_col271.clone(),
                    ab_minus_c_div_p_limb_20_col272.clone(),
                    ab_minus_c_div_p_limb_21_col273.clone(),
                    ab_minus_c_div_p_limb_22_col274.clone(),
                    ab_minus_c_div_p_limb_23_col275.clone(),
                    ab_minus_c_div_p_limb_24_col276.clone(),
                    ab_minus_c_div_p_limb_25_col277.clone(),
                    ab_minus_c_div_p_limb_26_col278.clone(),
                    ab_minus_c_div_p_limb_27_col279.clone(),
                    ab_minus_c_div_p_limb_28_col280.clone(),
                    ab_minus_c_div_p_limb_29_col281.clone(),
                    ab_minus_c_div_p_limb_30_col282.clone(),
                    ab_minus_c_div_p_limb_31_col283.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_185_limb_0.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_185_limb_1.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_185_limb_2.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_185_limb_3.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_185_limb_4.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_185_limb_5.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_185_limb_6.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_185_limb_7.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_185_limb_8.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_185_limb_9.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_185_limb_10.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_185_limb_11.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_185_limb_12.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_185_limb_13.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_185_limb_14.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_185_limb_15.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_206_limb_0.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_206_limb_1.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_206_limb_2.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_206_limb_3.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_206_limb_4.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_206_limb_5.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_206_limb_6.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_206_limb_7.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_206_limb_8.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_206_limb_9.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_206_limb_10.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_206_limb_11.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_206_limb_12.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_206_limb_13.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_206_limb_14.clone(),
                    mod_words_to_12_bit_array_output_tmp_cf8b4_206_limb_15.clone(),
                ],
                &mut eval,
            );
        // carry_0.
        eval.add_constraint(
            (carry_0_col364.clone()
                - (((M31_0.clone()
                    - mod_words_to_12_bit_array_output_tmp_cf8b4_311_limb_0.clone())
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_0.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_0.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_0_col364.clone() + M31_131072.clone())),
        ));

        // carry_1.
        eval.add_constraint(
            (carry_1_col365.clone()
                - (((carry_0_col364.clone()
                    - mod_words_to_12_bit_array_output_tmp_cf8b4_311_limb_1.clone())
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_1.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_1.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_1_col365.clone() + M31_131072.clone())),
        ));

        // carry_2.
        eval.add_constraint(
            (carry_2_col366.clone()
                - (((carry_1_col365.clone()
                    - mod_words_to_12_bit_array_output_tmp_cf8b4_311_limb_2.clone())
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_2.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_2.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_2_col366.clone() + M31_131072.clone())),
        ));

        // carry_3.
        eval.add_constraint(
            (carry_3_col367.clone()
                - (((carry_2_col366.clone()
                    - mod_words_to_12_bit_array_output_tmp_cf8b4_311_limb_3.clone())
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_3.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_3.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_3_col367.clone() + M31_131072.clone())),
        ));

        // carry_4.
        eval.add_constraint(
            (carry_4_col368.clone()
                - (((carry_3_col367.clone()
                    - mod_words_to_12_bit_array_output_tmp_cf8b4_311_limb_4.clone())
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_4.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_4.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_4_col368.clone() + M31_131072.clone())),
        ));

        // carry_5.
        eval.add_constraint(
            (carry_5_col369.clone()
                - (((carry_4_col368.clone()
                    - mod_words_to_12_bit_array_output_tmp_cf8b4_311_limb_5.clone())
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_5.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_5.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_5_col369.clone() + M31_131072.clone())),
        ));

        // carry_6.
        eval.add_constraint(
            (carry_6_col370.clone()
                - (((carry_5_col369.clone()
                    - mod_words_to_12_bit_array_output_tmp_cf8b4_311_limb_6.clone())
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_6.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_6.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_6_col370.clone() + M31_131072.clone())),
        ));

        // carry_7.
        eval.add_constraint(
            (carry_7_col371.clone()
                - (((carry_6_col370.clone()
                    - mod_words_to_12_bit_array_output_tmp_cf8b4_311_limb_7.clone())
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_7.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_7.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_7_col371.clone() + M31_131072.clone())),
        ));

        // carry_8.
        eval.add_constraint(
            (carry_8_col372.clone()
                - (((carry_7_col371.clone()
                    - mod_words_to_12_bit_array_output_tmp_cf8b4_311_limb_8.clone())
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_8.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_8.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_8_col372.clone() + M31_131072.clone())),
        ));

        // carry_9.
        eval.add_constraint(
            (carry_9_col373.clone()
                - (((carry_8_col372.clone()
                    - mod_words_to_12_bit_array_output_tmp_cf8b4_311_limb_9.clone())
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_9.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_9.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_9_col373.clone() + M31_131072.clone())),
        ));

        // carry_10.
        eval.add_constraint(
            (carry_10_col374.clone()
                - (((carry_9_col373.clone()
                    - mod_words_to_12_bit_array_output_tmp_cf8b4_311_limb_10.clone())
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_10.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_10.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_10_col374.clone() + M31_131072.clone())),
        ));

        // carry_11.
        eval.add_constraint(
            (carry_11_col375.clone()
                - (((carry_10_col374.clone()
                    - mod_words_to_12_bit_array_output_tmp_cf8b4_311_limb_11.clone())
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_11.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_11.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_11_col375.clone() + M31_131072.clone())),
        ));

        // carry_12.
        eval.add_constraint(
            (carry_12_col376.clone()
                - (((carry_11_col375.clone()
                    - mod_words_to_12_bit_array_output_tmp_cf8b4_311_limb_12.clone())
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_12.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_12.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_12_col376.clone() + M31_131072.clone())),
        ));

        // carry_13.
        eval.add_constraint(
            (carry_13_col377.clone()
                - (((carry_12_col376.clone()
                    - mod_words_to_12_bit_array_output_tmp_cf8b4_311_limb_13.clone())
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_13.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_13.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_13_col377.clone() + M31_131072.clone())),
        ));

        // carry_14.
        eval.add_constraint(
            (carry_14_col378.clone()
                - (((carry_13_col377.clone()
                    - mod_words_to_12_bit_array_output_tmp_cf8b4_311_limb_14.clone())
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_14.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_14.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_14_col378.clone() + M31_131072.clone())),
        ));

        // carry_15.
        eval.add_constraint(
            (carry_15_col379.clone()
                - (((carry_14_col378.clone()
                    - mod_words_to_12_bit_array_output_tmp_cf8b4_311_limb_15.clone())
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_15.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_15.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_15_col379.clone() + M31_131072.clone())),
        ));

        // carry_16.
        eval.add_constraint(
            (carry_16_col380.clone()
                - (((carry_15_col379.clone()
                    - mod_words_to_12_bit_array_output_tmp_cf8b4_332_limb_0.clone())
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_16.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_16.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_16_col380.clone() + M31_131072.clone())),
        ));

        // carry_17.
        eval.add_constraint(
            (carry_17_col381.clone()
                - (((carry_16_col380.clone()
                    - mod_words_to_12_bit_array_output_tmp_cf8b4_332_limb_1.clone())
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_17.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_17.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_17_col381.clone() + M31_131072.clone())),
        ));

        // carry_18.
        eval.add_constraint(
            (carry_18_col382.clone()
                - (((carry_17_col381.clone()
                    - mod_words_to_12_bit_array_output_tmp_cf8b4_332_limb_2.clone())
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_18.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_18.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_18_col382.clone() + M31_131072.clone())),
        ));

        // carry_19.
        eval.add_constraint(
            (carry_19_col383.clone()
                - (((carry_18_col382.clone()
                    - mod_words_to_12_bit_array_output_tmp_cf8b4_332_limb_3.clone())
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_19.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_19.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_19_col383.clone() + M31_131072.clone())),
        ));

        // carry_20.
        eval.add_constraint(
            (carry_20_col384.clone()
                - (((carry_19_col383.clone()
                    - mod_words_to_12_bit_array_output_tmp_cf8b4_332_limb_4.clone())
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_20.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_20.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_20_col384.clone() + M31_131072.clone())),
        ));

        // carry_21.
        eval.add_constraint(
            (carry_21_col385.clone()
                - (((carry_20_col384.clone()
                    - mod_words_to_12_bit_array_output_tmp_cf8b4_332_limb_5.clone())
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_21.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_21.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_21_col385.clone() + M31_131072.clone())),
        ));

        // carry_22.
        eval.add_constraint(
            (carry_22_col386.clone()
                - (((carry_21_col385.clone()
                    - mod_words_to_12_bit_array_output_tmp_cf8b4_332_limb_6.clone())
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_22.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_22.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_22_col386.clone() + M31_131072.clone())),
        ));

        // carry_23.
        eval.add_constraint(
            (carry_23_col387.clone()
                - (((carry_22_col386.clone()
                    - mod_words_to_12_bit_array_output_tmp_cf8b4_332_limb_7.clone())
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_23.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_23.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_23_col387.clone() + M31_131072.clone())),
        ));

        // carry_24.
        eval.add_constraint(
            (carry_24_col388.clone()
                - (((carry_23_col387.clone()
                    - mod_words_to_12_bit_array_output_tmp_cf8b4_332_limb_8.clone())
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_24.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_24.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_24_col388.clone() + M31_131072.clone())),
        ));

        // carry_25.
        eval.add_constraint(
            (carry_25_col389.clone()
                - (((carry_24_col388.clone()
                    - mod_words_to_12_bit_array_output_tmp_cf8b4_332_limb_9.clone())
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_25.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_25.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_25_col389.clone() + M31_131072.clone())),
        ));

        // carry_26.
        eval.add_constraint(
            (carry_26_col390.clone()
                - (((carry_25_col389.clone()
                    - mod_words_to_12_bit_array_output_tmp_cf8b4_332_limb_10.clone())
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_26.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_26.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_26_col390.clone() + M31_131072.clone())),
        ));

        // carry_27.
        eval.add_constraint(
            (carry_27_col391.clone()
                - (((carry_26_col390.clone()
                    - mod_words_to_12_bit_array_output_tmp_cf8b4_332_limb_11.clone())
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_27.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_27.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_27_col391.clone() + M31_131072.clone())),
        ));

        // carry_28.
        eval.add_constraint(
            (carry_28_col392.clone()
                - (((carry_27_col391.clone()
                    - mod_words_to_12_bit_array_output_tmp_cf8b4_332_limb_12.clone())
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_28.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_28.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_28_col392.clone() + M31_131072.clone())),
        ));

        // carry_29.
        eval.add_constraint(
            (carry_29_col393.clone()
                - (((carry_28_col392.clone()
                    - mod_words_to_12_bit_array_output_tmp_cf8b4_332_limb_13.clone())
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_29.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_29.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_29_col393.clone() + M31_131072.clone())),
        ));

        // carry_30.
        eval.add_constraint(
            (carry_30_col394.clone()
                - (((carry_29_col393.clone()
                    - mod_words_to_12_bit_array_output_tmp_cf8b4_332_limb_14.clone())
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_30.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_30.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_30_col394.clone() + M31_131072.clone())),
        ));

        // carry_31.
        eval.add_constraint(
            (carry_31_col395.clone()
                - (((carry_30_col394.clone()
                    - mod_words_to_12_bit_array_output_tmp_cf8b4_332_limb_15.clone())
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_31.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_31.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_31_col395.clone() + M31_131072.clone())),
        ));

        // carry_32.
        eval.add_constraint(
            (carry_32_col396.clone()
                - ((carry_31_col395.clone()
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_32.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_32.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_32_col396.clone() + M31_131072.clone())),
        ));

        // carry_33.
        eval.add_constraint(
            (carry_33_col397.clone()
                - ((carry_32_col396.clone()
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_33.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_33.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_33_col397.clone() + M31_131072.clone())),
        ));

        // carry_34.
        eval.add_constraint(
            (carry_34_col398.clone()
                - ((carry_33_col397.clone()
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_34.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_34.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_34_col398.clone() + M31_131072.clone())),
        ));

        // carry_35.
        eval.add_constraint(
            (carry_35_col399.clone()
                - ((carry_34_col398.clone()
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_35.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_35.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_35_col399.clone() + M31_131072.clone())),
        ));

        // carry_36.
        eval.add_constraint(
            (carry_36_col400.clone()
                - ((carry_35_col399.clone()
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_36.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_36.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_36_col400.clone() + M31_131072.clone())),
        ));

        // carry_37.
        eval.add_constraint(
            (carry_37_col401.clone()
                - ((carry_36_col400.clone()
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_37.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_37.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_37_col401.clone() + M31_131072.clone())),
        ));

        // carry_38.
        eval.add_constraint(
            (carry_38_col402.clone()
                - ((carry_37_col401.clone()
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_38.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_38.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_38_col402.clone() + M31_131072.clone())),
        ));

        // carry_39.
        eval.add_constraint(
            (carry_39_col403.clone()
                - ((carry_38_col402.clone()
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_39.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_39.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_39_col403.clone() + M31_131072.clone())),
        ));

        // carry_40.
        eval.add_constraint(
            (carry_40_col404.clone()
                - ((carry_39_col403.clone()
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_40.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_40.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_40_col404.clone() + M31_131072.clone())),
        ));

        // carry_41.
        eval.add_constraint(
            (carry_41_col405.clone()
                - ((carry_40_col404.clone()
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_41.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_41.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_41_col405.clone() + M31_131072.clone())),
        ));

        // carry_42.
        eval.add_constraint(
            (carry_42_col406.clone()
                - ((carry_41_col405.clone()
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_42.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_42.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_42_col406.clone() + M31_131072.clone())),
        ));

        // carry_43.
        eval.add_constraint(
            (carry_43_col407.clone()
                - ((carry_42_col406.clone()
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_43.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_43.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_43_col407.clone() + M31_131072.clone())),
        ));

        // carry_44.
        eval.add_constraint(
            (carry_44_col408.clone()
                - ((carry_43_col407.clone()
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_44.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_44.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_44_col408.clone() + M31_131072.clone())),
        ));

        // carry_45.
        eval.add_constraint(
            (carry_45_col409.clone()
                - ((carry_44_col408.clone()
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_45.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_45.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_45_col409.clone() + M31_131072.clone())),
        ));

        // carry_46.
        eval.add_constraint(
            (carry_46_col410.clone()
                - ((carry_45_col409.clone()
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_46.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_46.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_46_col410.clone() + M31_131072.clone())),
        ));

        // carry_47.
        eval.add_constraint(
            (carry_47_col411.clone()
                - ((carry_46_col410.clone()
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_47.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_47.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_47_col411.clone() + M31_131072.clone())),
        ));

        // carry_48.
        eval.add_constraint(
            (carry_48_col412.clone()
                - ((carry_47_col411.clone()
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_48.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_48.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_48_col412.clone() + M31_131072.clone())),
        ));

        // carry_49.
        eval.add_constraint(
            (carry_49_col413.clone()
                - ((carry_48_col412.clone()
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_49.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_49.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_49_col413.clone() + M31_131072.clone())),
        ));

        // carry_50.
        eval.add_constraint(
            (carry_50_col414.clone()
                - ((carry_49_col413.clone()
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_50.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_50.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_50_col414.clone() + M31_131072.clone())),
        ));

        // carry_51.
        eval.add_constraint(
            (carry_51_col415.clone()
                - ((carry_50_col414.clone()
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_51.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_51.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_51_col415.clone() + M31_131072.clone())),
        ));

        // carry_52.
        eval.add_constraint(
            (carry_52_col416.clone()
                - ((carry_51_col415.clone()
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_52.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_52.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_52_col416.clone() + M31_131072.clone())),
        ));

        // carry_53.
        eval.add_constraint(
            (carry_53_col417.clone()
                - ((carry_52_col416.clone()
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_53.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_53.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_53_col417.clone() + M31_131072.clone())),
        ));

        // carry_54.
        eval.add_constraint(
            (carry_54_col418.clone()
                - ((carry_53_col417.clone()
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_54.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_54.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_54_col418.clone() + M31_131072.clone())),
        ));

        // carry_55.
        eval.add_constraint(
            (carry_55_col419.clone()
                - ((carry_54_col418.clone()
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_55.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_55.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_55_col419.clone() + M31_131072.clone())),
        ));

        // carry_56.
        eval.add_constraint(
            (carry_56_col420.clone()
                - ((carry_55_col419.clone()
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_56.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_56.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_56_col420.clone() + M31_131072.clone())),
        ));

        // carry_57.
        eval.add_constraint(
            (carry_57_col421.clone()
                - ((carry_56_col420.clone()
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_57.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_57.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_57_col421.clone() + M31_131072.clone())),
        ));

        // carry_58.
        eval.add_constraint(
            (carry_58_col422.clone()
                - ((carry_57_col421.clone()
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_58.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_58.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_58_col422.clone() + M31_131072.clone())),
        ));

        // carry_59.
        eval.add_constraint(
            (carry_59_col423.clone()
                - ((carry_58_col422.clone()
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_59.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_59.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_59_col423.clone() + M31_131072.clone())),
        ));

        // carry_60.
        eval.add_constraint(
            (carry_60_col424.clone()
                - ((carry_59_col423.clone()
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_60.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_60.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_60_col424.clone() + M31_131072.clone())),
        ));

        // carry_61.
        eval.add_constraint(
            (carry_61_col425.clone()
                - ((carry_60_col424.clone()
                    + (double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_61.clone()
                        - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_61.clone()))
                    * M31_524288.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&(carry_61_col425.clone() + M31_131072.clone())),
        ));

        // final limb constraint.
        eval.add_constraint(
            ((double_karatsuba_b1daa_output_tmp_cf8b4_350_limb_62.clone()
                + carry_61_col425.clone())
                - double_karatsuba_b1daa_output_tmp_cf8b4_368_limb_62.clone()),
        );
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
    use crate::components::constraints_regression_test_values::MUL_MOD_BUILTIN;

    #[test]
    fn mul_mod_builtin_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim {
                log_size: 4,
                mul_mod_builtin_segment_start: rng.gen::<u32>(),
            },
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            range_check_12_lookup_elements: relations::RangeCheck_12::dummy(),
            range_check_3_6_6_3_lookup_elements: relations::RangeCheck_3_6_6_3::dummy(),
            range_check_18_lookup_elements: relations::RangeCheck_18::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        assert_eq!(sum, MUL_MOD_BUILTIN);
    }
}
