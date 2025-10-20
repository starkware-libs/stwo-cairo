// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::mod_utils::ModUtils;

pub const N_TRACE_COLUMNS: usize = 267;
pub const RELATION_USES_PER_ROW: [RelationUse; 2] = [
    RelationUse {
        relation_id: "MemoryAddressToId",
        uses: 29,
    },
    RelationUse {
        relation_id: "MemoryIdToBig",
        uses: 24,
    },
];

pub struct Eval {
    pub claim: Claim,
    pub memory_address_to_id_lookup_elements: relations::MemoryAddressToId,
    pub memory_id_to_big_lookup_elements: relations::MemoryIdToBig,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct Claim {
    pub log_size: u32,
    pub add_mod_builtin_segment_start: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 27];
        TreeVec::new(vec![vec![], trace_log_sizes, interaction_log_sizes])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(self.log_size as u64);
        channel.mix_u64(self.add_mod_builtin_segment_start as u64);
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
        let M31_128 = E::F::from(M31::from(128));
        let M31_16 = E::F::from(M31::from(16));
        let M31_262144 = E::F::from(M31::from(262144));
        let M31_32768 = E::F::from(M31::from(32768));
        let M31_512 = E::F::from(M31::from(512));
        let M31_64 = E::F::from(M31::from(64));
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
        let sub_p_bit_col252 = eval.next_trace_mask();
        let carry_0_col253 = eval.next_trace_mask();
        let carry_1_col254 = eval.next_trace_mask();
        let carry_2_col255 = eval.next_trace_mask();
        let carry_3_col256 = eval.next_trace_mask();
        let carry_4_col257 = eval.next_trace_mask();
        let carry_5_col258 = eval.next_trace_mask();
        let carry_6_col259 = eval.next_trace_mask();
        let carry_7_col260 = eval.next_trace_mask();
        let carry_8_col261 = eval.next_trace_mask();
        let carry_9_col262 = eval.next_trace_mask();
        let carry_10_col263 = eval.next_trace_mask();
        let carry_11_col264 = eval.next_trace_mask();
        let carry_12_col265 = eval.next_trace_mask();
        let carry_13_col266 = eval.next_trace_mask();

        ModUtils::evaluate(
            [
                E::F::from(M31::from(self.claim.add_mod_builtin_segment_start)),
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
        // make sure sub_p_bit is 0 or 1..
        eval.add_constraint(
            ((sub_p_bit_col252.clone() - M31_1.clone()) * sub_p_bit_col252.clone()),
        );
        // carry_0.
        eval.add_constraint(
            (carry_0_col253.clone()
                - ((((((a0_limb_0_col109.clone() + b0_limb_0_col157.clone())
                    - c0_limb_0_col205.clone())
                    - (p0_limb_0_col2.clone() * sub_p_bit_col252.clone()))
                    + (M31_512.clone()
                        * (((a0_limb_1_col110.clone() + b0_limb_1_col158.clone())
                            - c0_limb_1_col206.clone())
                            - (p0_limb_1_col3.clone() * sub_p_bit_col252.clone()))))
                    + (M31_262144.clone()
                        * (((a0_limb_2_col111.clone() + b0_limb_2_col159.clone())
                            - c0_limb_2_col207.clone())
                            - (p0_limb_2_col4.clone() * sub_p_bit_col252.clone()))))
                    * M31_16.clone())),
        );
        // carry is 0 or 1 or -1..
        eval.add_constraint(
            (carry_0_col253.clone()
                * ((carry_0_col253.clone() * carry_0_col253.clone()) - M31_1.clone())),
        );
        // carry_1.
        eval.add_constraint(
            (carry_1_col254.clone()
                - ((((carry_0_col253.clone()
                    + (((a0_limb_3_col112.clone() + b0_limb_3_col160.clone())
                        - c0_limb_3_col208.clone())
                        - (p0_limb_3_col5.clone() * sub_p_bit_col252.clone())))
                    + (M31_512.clone()
                        * (((a0_limb_4_col113.clone() + b0_limb_4_col161.clone())
                            - c0_limb_4_col209.clone())
                            - (p0_limb_4_col6.clone() * sub_p_bit_col252.clone()))))
                    + (M31_262144.clone()
                        * (((a0_limb_5_col114.clone() + b0_limb_5_col162.clone())
                            - c0_limb_5_col210.clone())
                            - (p0_limb_5_col7.clone() * sub_p_bit_col252.clone()))))
                    * M31_16.clone())),
        );
        // carry is 0 or 1 or -1..
        eval.add_constraint(
            (carry_1_col254.clone()
                * ((carry_1_col254.clone() * carry_1_col254.clone()) - M31_1.clone())),
        );
        // carry_2.
        eval.add_constraint(
            (carry_2_col255.clone()
                - ((((carry_1_col254.clone()
                    + (((a0_limb_6_col115.clone() + b0_limb_6_col163.clone())
                        - c0_limb_6_col211.clone())
                        - (p0_limb_6_col8.clone() * sub_p_bit_col252.clone())))
                    + (M31_512.clone()
                        * (((a0_limb_7_col116.clone() + b0_limb_7_col164.clone())
                            - c0_limb_7_col212.clone())
                            - (p0_limb_7_col9.clone() * sub_p_bit_col252.clone()))))
                    + (M31_262144.clone()
                        * (((a0_limb_8_col117.clone() + b0_limb_8_col165.clone())
                            - c0_limb_8_col213.clone())
                            - (p0_limb_8_col10.clone() * sub_p_bit_col252.clone()))))
                    * M31_16.clone())),
        );
        // carry is 0 or 1 or -1..
        eval.add_constraint(
            (carry_2_col255.clone()
                * ((carry_2_col255.clone() * carry_2_col255.clone()) - M31_1.clone())),
        );
        // carry_3.
        eval.add_constraint(
            (carry_3_col256.clone()
                - ((((carry_2_col255.clone()
                    + (((a0_limb_9_col118.clone() + b0_limb_9_col166.clone())
                        - c0_limb_9_col214.clone())
                        - (p0_limb_9_col11.clone() * sub_p_bit_col252.clone())))
                    + (M31_512.clone()
                        * (((a0_limb_10_col119.clone() + b0_limb_10_col167.clone())
                            - c0_limb_10_col215.clone())
                            - (p0_limb_10_col12.clone() * sub_p_bit_col252.clone()))))
                    + (M31_32768.clone()
                        * (((a1_limb_0_col121.clone() + b1_limb_0_col169.clone())
                            - c1_limb_0_col217.clone())
                            - (p1_limb_0_col14.clone() * sub_p_bit_col252.clone()))))
                    * M31_128.clone())),
        );
        // carry is 0 or 1 or -1..
        eval.add_constraint(
            (carry_3_col256.clone()
                * ((carry_3_col256.clone() * carry_3_col256.clone()) - M31_1.clone())),
        );
        // carry_4.
        eval.add_constraint(
            (carry_4_col257.clone()
                - ((((carry_3_col256.clone()
                    + (((a1_limb_1_col122.clone() + b1_limb_1_col170.clone())
                        - c1_limb_1_col218.clone())
                        - (p1_limb_1_col15.clone() * sub_p_bit_col252.clone())))
                    + (M31_512.clone()
                        * (((a1_limb_2_col123.clone() + b1_limb_2_col171.clone())
                            - c1_limb_2_col219.clone())
                            - (p1_limb_2_col16.clone() * sub_p_bit_col252.clone()))))
                    + (M31_262144.clone()
                        * (((a1_limb_3_col124.clone() + b1_limb_3_col172.clone())
                            - c1_limb_3_col220.clone())
                            - (p1_limb_3_col17.clone() * sub_p_bit_col252.clone()))))
                    * M31_16.clone())),
        );
        // carry is 0 or 1 or -1..
        eval.add_constraint(
            (carry_4_col257.clone()
                * ((carry_4_col257.clone() * carry_4_col257.clone()) - M31_1.clone())),
        );
        // carry_5.
        eval.add_constraint(
            (carry_5_col258.clone()
                - ((((carry_4_col257.clone()
                    + (((a1_limb_4_col125.clone() + b1_limb_4_col173.clone())
                        - c1_limb_4_col221.clone())
                        - (p1_limb_4_col18.clone() * sub_p_bit_col252.clone())))
                    + (M31_512.clone()
                        * (((a1_limb_5_col126.clone() + b1_limb_5_col174.clone())
                            - c1_limb_5_col222.clone())
                            - (p1_limb_5_col19.clone() * sub_p_bit_col252.clone()))))
                    + (M31_262144.clone()
                        * (((a1_limb_6_col127.clone() + b1_limb_6_col175.clone())
                            - c1_limb_6_col223.clone())
                            - (p1_limb_6_col20.clone() * sub_p_bit_col252.clone()))))
                    * M31_16.clone())),
        );
        // carry is 0 or 1 or -1..
        eval.add_constraint(
            (carry_5_col258.clone()
                * ((carry_5_col258.clone() * carry_5_col258.clone()) - M31_1.clone())),
        );
        // carry_6.
        eval.add_constraint(
            (carry_6_col259.clone()
                - ((((carry_5_col258.clone()
                    + (((a1_limb_7_col128.clone() + b1_limb_7_col176.clone())
                        - c1_limb_7_col224.clone())
                        - (p1_limb_7_col21.clone() * sub_p_bit_col252.clone())))
                    + (M31_512.clone()
                        * (((a1_limb_8_col129.clone() + b1_limb_8_col177.clone())
                            - c1_limb_8_col225.clone())
                            - (p1_limb_8_col22.clone() * sub_p_bit_col252.clone()))))
                    + (M31_262144.clone()
                        * (((a1_limb_9_col130.clone() + b1_limb_9_col178.clone())
                            - c1_limb_9_col226.clone())
                            - (p1_limb_9_col23.clone() * sub_p_bit_col252.clone()))))
                    * M31_16.clone())),
        );
        // carry is 0 or 1 or -1..
        eval.add_constraint(
            (carry_6_col259.clone()
                * ((carry_6_col259.clone() * carry_6_col259.clone()) - M31_1.clone())),
        );
        // carry_7.
        eval.add_constraint(
            (carry_7_col260.clone()
                - ((((carry_6_col259.clone()
                    + (((a1_limb_10_col131.clone() + b1_limb_10_col179.clone())
                        - c1_limb_10_col227.clone())
                        - (p1_limb_10_col24.clone() * sub_p_bit_col252.clone())))
                    + (M31_64.clone()
                        * (((a2_limb_0_col133.clone() + b2_limb_0_col181.clone())
                            - c2_limb_0_col229.clone())
                            - (p2_limb_0_col26.clone() * sub_p_bit_col252.clone()))))
                    + (M31_32768.clone()
                        * (((a2_limb_1_col134.clone() + b2_limb_1_col182.clone())
                            - c2_limb_1_col230.clone())
                            - (p2_limb_1_col27.clone() * sub_p_bit_col252.clone()))))
                    * M31_128.clone())),
        );
        // carry is 0 or 1 or -1..
        eval.add_constraint(
            (carry_7_col260.clone()
                * ((carry_7_col260.clone() * carry_7_col260.clone()) - M31_1.clone())),
        );
        // carry_8.
        eval.add_constraint(
            (carry_8_col261.clone()
                - ((((carry_7_col260.clone()
                    + (((a2_limb_2_col135.clone() + b2_limb_2_col183.clone())
                        - c2_limb_2_col231.clone())
                        - (p2_limb_2_col28.clone() * sub_p_bit_col252.clone())))
                    + (M31_512.clone()
                        * (((a2_limb_3_col136.clone() + b2_limb_3_col184.clone())
                            - c2_limb_3_col232.clone())
                            - (p2_limb_3_col29.clone() * sub_p_bit_col252.clone()))))
                    + (M31_262144.clone()
                        * (((a2_limb_4_col137.clone() + b2_limb_4_col185.clone())
                            - c2_limb_4_col233.clone())
                            - (p2_limb_4_col30.clone() * sub_p_bit_col252.clone()))))
                    * M31_16.clone())),
        );
        // carry is 0 or 1 or -1..
        eval.add_constraint(
            (carry_8_col261.clone()
                * ((carry_8_col261.clone() * carry_8_col261.clone()) - M31_1.clone())),
        );
        // carry_9.
        eval.add_constraint(
            (carry_9_col262.clone()
                - ((((carry_8_col261.clone()
                    + (((a2_limb_5_col138.clone() + b2_limb_5_col186.clone())
                        - c2_limb_5_col234.clone())
                        - (p2_limb_5_col31.clone() * sub_p_bit_col252.clone())))
                    + (M31_512.clone()
                        * (((a2_limb_6_col139.clone() + b2_limb_6_col187.clone())
                            - c2_limb_6_col235.clone())
                            - (p2_limb_6_col32.clone() * sub_p_bit_col252.clone()))))
                    + (M31_262144.clone()
                        * (((a2_limb_7_col140.clone() + b2_limb_7_col188.clone())
                            - c2_limb_7_col236.clone())
                            - (p2_limb_7_col33.clone() * sub_p_bit_col252.clone()))))
                    * M31_16.clone())),
        );
        // carry is 0 or 1 or -1..
        eval.add_constraint(
            (carry_9_col262.clone()
                * ((carry_9_col262.clone() * carry_9_col262.clone()) - M31_1.clone())),
        );
        // carry_10.
        eval.add_constraint(
            (carry_10_col263.clone()
                - ((((carry_9_col262.clone()
                    + (((a2_limb_8_col141.clone() + b2_limb_8_col189.clone())
                        - c2_limb_8_col237.clone())
                        - (p2_limb_8_col34.clone() * sub_p_bit_col252.clone())))
                    + (M31_512.clone()
                        * (((a2_limb_9_col142.clone() + b2_limb_9_col190.clone())
                            - c2_limb_9_col238.clone())
                            - (p2_limb_9_col35.clone() * sub_p_bit_col252.clone()))))
                    + (M31_262144.clone()
                        * (((a2_limb_10_col143.clone() + b2_limb_10_col191.clone())
                            - c2_limb_10_col239.clone())
                            - (p2_limb_10_col36.clone() * sub_p_bit_col252.clone()))))
                    * M31_128.clone())),
        );
        // carry is 0 or 1 or -1..
        eval.add_constraint(
            (carry_10_col263.clone()
                * ((carry_10_col263.clone() * carry_10_col263.clone()) - M31_1.clone())),
        );
        // carry_11.
        eval.add_constraint(
            (carry_11_col264.clone()
                - ((((carry_10_col263.clone()
                    + (((a3_limb_0_col145.clone() + b3_limb_0_col193.clone())
                        - c3_limb_0_col241.clone())
                        - (p3_limb_0_col38.clone() * sub_p_bit_col252.clone())))
                    + (M31_512.clone()
                        * (((a3_limb_1_col146.clone() + b3_limb_1_col194.clone())
                            - c3_limb_1_col242.clone())
                            - (p3_limb_1_col39.clone() * sub_p_bit_col252.clone()))))
                    + (M31_262144.clone()
                        * (((a3_limb_2_col147.clone() + b3_limb_2_col195.clone())
                            - c3_limb_2_col243.clone())
                            - (p3_limb_2_col40.clone() * sub_p_bit_col252.clone()))))
                    * M31_16.clone())),
        );
        // carry is 0 or 1 or -1..
        eval.add_constraint(
            (carry_11_col264.clone()
                * ((carry_11_col264.clone() * carry_11_col264.clone()) - M31_1.clone())),
        );
        // carry_12.
        eval.add_constraint(
            (carry_12_col265.clone()
                - ((((carry_11_col264.clone()
                    + (((a3_limb_3_col148.clone() + b3_limb_3_col196.clone())
                        - c3_limb_3_col244.clone())
                        - (p3_limb_3_col41.clone() * sub_p_bit_col252.clone())))
                    + (M31_512.clone()
                        * (((a3_limb_4_col149.clone() + b3_limb_4_col197.clone())
                            - c3_limb_4_col245.clone())
                            - (p3_limb_4_col42.clone() * sub_p_bit_col252.clone()))))
                    + (M31_262144.clone()
                        * (((a3_limb_5_col150.clone() + b3_limb_5_col198.clone())
                            - c3_limb_5_col246.clone())
                            - (p3_limb_5_col43.clone() * sub_p_bit_col252.clone()))))
                    * M31_16.clone())),
        );
        // carry is 0 or 1 or -1..
        eval.add_constraint(
            (carry_12_col265.clone()
                * ((carry_12_col265.clone() * carry_12_col265.clone()) - M31_1.clone())),
        );
        // carry_13.
        eval.add_constraint(
            (carry_13_col266.clone()
                - ((((carry_12_col265.clone()
                    + (((a3_limb_6_col151.clone() + b3_limb_6_col199.clone())
                        - c3_limb_6_col247.clone())
                        - (p3_limb_6_col44.clone() * sub_p_bit_col252.clone())))
                    + (M31_512.clone()
                        * (((a3_limb_7_col152.clone() + b3_limb_7_col200.clone())
                            - c3_limb_7_col248.clone())
                            - (p3_limb_7_col45.clone() * sub_p_bit_col252.clone()))))
                    + (M31_262144.clone()
                        * (((a3_limb_8_col153.clone() + b3_limb_8_col201.clone())
                            - c3_limb_8_col249.clone())
                            - (p3_limb_8_col46.clone() * sub_p_bit_col252.clone()))))
                    * M31_16.clone())),
        );
        // carry is 0 or 1 or -1..
        eval.add_constraint(
            (carry_13_col266.clone()
                * ((carry_13_col266.clone() * carry_13_col266.clone()) - M31_1.clone())),
        );
        // last carry needs to be 0..
        eval.add_constraint(
            ((carry_13_col266.clone()
                + (((a3_limb_9_col154.clone() + b3_limb_9_col202.clone())
                    - c3_limb_9_col250.clone())
                    - (p3_limb_9_col47.clone() * sub_p_bit_col252.clone())))
                + (M31_512.clone()
                    * (((a3_limb_10_col155.clone() + b3_limb_10_col203.clone())
                        - c3_limb_10_col251.clone())
                        - (p3_limb_10_col48.clone() * sub_p_bit_col252.clone())))),
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
    use crate::components::constraints_regression_test_values::ADD_MOD_BUILTIN;

    #[test]
    fn add_mod_builtin_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim {
                log_size: 4,
                add_mod_builtin_segment_start: rng.gen::<u32>(),
            },
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        assert_eq!(sum, ADD_MOD_BUILTIN);
    }
}
