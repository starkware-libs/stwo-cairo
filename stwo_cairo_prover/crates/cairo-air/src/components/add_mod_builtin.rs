use crate::components::prelude::*;
use crate::components::subroutines::mod_utils::ModUtils;

pub const N_TRACE_COLUMNS: usize = 251;

pub struct Eval {
    pub claim: Claim,
    pub memory_address_to_id_lookup_elements: relations::MemoryAddressToId,
    pub memory_id_to_big_lookup_elements: relations::MemoryIdToBig,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
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
        let M31_1 = E::F::from(M31::from(1));
        let M31_512 = E::F::from(M31::from(512));
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
        let offsets_ptr_id_col53 = eval.next_trace_mask();
        let offsets_ptr_limb_0_col54 = eval.next_trace_mask();
        let offsets_ptr_limb_1_col55 = eval.next_trace_mask();
        let offsets_ptr_limb_2_col56 = eval.next_trace_mask();
        let offsets_ptr_prev_id_col57 = eval.next_trace_mask();
        let offsets_ptr_prev_limb_0_col58 = eval.next_trace_mask();
        let offsets_ptr_prev_limb_1_col59 = eval.next_trace_mask();
        let offsets_ptr_prev_limb_2_col60 = eval.next_trace_mask();
        let n_id_col61 = eval.next_trace_mask();
        let n_limb_0_col62 = eval.next_trace_mask();
        let n_limb_1_col63 = eval.next_trace_mask();
        let n_limb_2_col64 = eval.next_trace_mask();
        let n_prev_id_col65 = eval.next_trace_mask();
        let n_prev_limb_0_col66 = eval.next_trace_mask();
        let n_prev_limb_1_col67 = eval.next_trace_mask();
        let n_prev_limb_2_col68 = eval.next_trace_mask();
        let values_ptr_prev_id_col69 = eval.next_trace_mask();
        let p_prev0_id_col70 = eval.next_trace_mask();
        let p_prev1_id_col71 = eval.next_trace_mask();
        let p_prev2_id_col72 = eval.next_trace_mask();
        let p_prev3_id_col73 = eval.next_trace_mask();
        let offsets_a_id_col74 = eval.next_trace_mask();
        let msb_col75 = eval.next_trace_mask();
        let mid_limbs_set_col76 = eval.next_trace_mask();
        let offsets_a_limb_0_col77 = eval.next_trace_mask();
        let offsets_a_limb_1_col78 = eval.next_trace_mask();
        let offsets_a_limb_2_col79 = eval.next_trace_mask();
        let offsets_b_id_col80 = eval.next_trace_mask();
        let msb_col81 = eval.next_trace_mask();
        let mid_limbs_set_col82 = eval.next_trace_mask();
        let offsets_b_limb_0_col83 = eval.next_trace_mask();
        let offsets_b_limb_1_col84 = eval.next_trace_mask();
        let offsets_b_limb_2_col85 = eval.next_trace_mask();
        let offsets_c_id_col86 = eval.next_trace_mask();
        let msb_col87 = eval.next_trace_mask();
        let mid_limbs_set_col88 = eval.next_trace_mask();
        let offsets_c_limb_0_col89 = eval.next_trace_mask();
        let offsets_c_limb_1_col90 = eval.next_trace_mask();
        let offsets_c_limb_2_col91 = eval.next_trace_mask();
        let a0_id_col92 = eval.next_trace_mask();
        let a0_limb_0_col93 = eval.next_trace_mask();
        let a0_limb_1_col94 = eval.next_trace_mask();
        let a0_limb_2_col95 = eval.next_trace_mask();
        let a0_limb_3_col96 = eval.next_trace_mask();
        let a0_limb_4_col97 = eval.next_trace_mask();
        let a0_limb_5_col98 = eval.next_trace_mask();
        let a0_limb_6_col99 = eval.next_trace_mask();
        let a0_limb_7_col100 = eval.next_trace_mask();
        let a0_limb_8_col101 = eval.next_trace_mask();
        let a0_limb_9_col102 = eval.next_trace_mask();
        let a0_limb_10_col103 = eval.next_trace_mask();
        let a1_id_col104 = eval.next_trace_mask();
        let a1_limb_0_col105 = eval.next_trace_mask();
        let a1_limb_1_col106 = eval.next_trace_mask();
        let a1_limb_2_col107 = eval.next_trace_mask();
        let a1_limb_3_col108 = eval.next_trace_mask();
        let a1_limb_4_col109 = eval.next_trace_mask();
        let a1_limb_5_col110 = eval.next_trace_mask();
        let a1_limb_6_col111 = eval.next_trace_mask();
        let a1_limb_7_col112 = eval.next_trace_mask();
        let a1_limb_8_col113 = eval.next_trace_mask();
        let a1_limb_9_col114 = eval.next_trace_mask();
        let a1_limb_10_col115 = eval.next_trace_mask();
        let a2_id_col116 = eval.next_trace_mask();
        let a2_limb_0_col117 = eval.next_trace_mask();
        let a2_limb_1_col118 = eval.next_trace_mask();
        let a2_limb_2_col119 = eval.next_trace_mask();
        let a2_limb_3_col120 = eval.next_trace_mask();
        let a2_limb_4_col121 = eval.next_trace_mask();
        let a2_limb_5_col122 = eval.next_trace_mask();
        let a2_limb_6_col123 = eval.next_trace_mask();
        let a2_limb_7_col124 = eval.next_trace_mask();
        let a2_limb_8_col125 = eval.next_trace_mask();
        let a2_limb_9_col126 = eval.next_trace_mask();
        let a2_limb_10_col127 = eval.next_trace_mask();
        let a3_id_col128 = eval.next_trace_mask();
        let a3_limb_0_col129 = eval.next_trace_mask();
        let a3_limb_1_col130 = eval.next_trace_mask();
        let a3_limb_2_col131 = eval.next_trace_mask();
        let a3_limb_3_col132 = eval.next_trace_mask();
        let a3_limb_4_col133 = eval.next_trace_mask();
        let a3_limb_5_col134 = eval.next_trace_mask();
        let a3_limb_6_col135 = eval.next_trace_mask();
        let a3_limb_7_col136 = eval.next_trace_mask();
        let a3_limb_8_col137 = eval.next_trace_mask();
        let a3_limb_9_col138 = eval.next_trace_mask();
        let a3_limb_10_col139 = eval.next_trace_mask();
        let b0_id_col140 = eval.next_trace_mask();
        let b0_limb_0_col141 = eval.next_trace_mask();
        let b0_limb_1_col142 = eval.next_trace_mask();
        let b0_limb_2_col143 = eval.next_trace_mask();
        let b0_limb_3_col144 = eval.next_trace_mask();
        let b0_limb_4_col145 = eval.next_trace_mask();
        let b0_limb_5_col146 = eval.next_trace_mask();
        let b0_limb_6_col147 = eval.next_trace_mask();
        let b0_limb_7_col148 = eval.next_trace_mask();
        let b0_limb_8_col149 = eval.next_trace_mask();
        let b0_limb_9_col150 = eval.next_trace_mask();
        let b0_limb_10_col151 = eval.next_trace_mask();
        let b1_id_col152 = eval.next_trace_mask();
        let b1_limb_0_col153 = eval.next_trace_mask();
        let b1_limb_1_col154 = eval.next_trace_mask();
        let b1_limb_2_col155 = eval.next_trace_mask();
        let b1_limb_3_col156 = eval.next_trace_mask();
        let b1_limb_4_col157 = eval.next_trace_mask();
        let b1_limb_5_col158 = eval.next_trace_mask();
        let b1_limb_6_col159 = eval.next_trace_mask();
        let b1_limb_7_col160 = eval.next_trace_mask();
        let b1_limb_8_col161 = eval.next_trace_mask();
        let b1_limb_9_col162 = eval.next_trace_mask();
        let b1_limb_10_col163 = eval.next_trace_mask();
        let b2_id_col164 = eval.next_trace_mask();
        let b2_limb_0_col165 = eval.next_trace_mask();
        let b2_limb_1_col166 = eval.next_trace_mask();
        let b2_limb_2_col167 = eval.next_trace_mask();
        let b2_limb_3_col168 = eval.next_trace_mask();
        let b2_limb_4_col169 = eval.next_trace_mask();
        let b2_limb_5_col170 = eval.next_trace_mask();
        let b2_limb_6_col171 = eval.next_trace_mask();
        let b2_limb_7_col172 = eval.next_trace_mask();
        let b2_limb_8_col173 = eval.next_trace_mask();
        let b2_limb_9_col174 = eval.next_trace_mask();
        let b2_limb_10_col175 = eval.next_trace_mask();
        let b3_id_col176 = eval.next_trace_mask();
        let b3_limb_0_col177 = eval.next_trace_mask();
        let b3_limb_1_col178 = eval.next_trace_mask();
        let b3_limb_2_col179 = eval.next_trace_mask();
        let b3_limb_3_col180 = eval.next_trace_mask();
        let b3_limb_4_col181 = eval.next_trace_mask();
        let b3_limb_5_col182 = eval.next_trace_mask();
        let b3_limb_6_col183 = eval.next_trace_mask();
        let b3_limb_7_col184 = eval.next_trace_mask();
        let b3_limb_8_col185 = eval.next_trace_mask();
        let b3_limb_9_col186 = eval.next_trace_mask();
        let b3_limb_10_col187 = eval.next_trace_mask();
        let c0_id_col188 = eval.next_trace_mask();
        let c0_limb_0_col189 = eval.next_trace_mask();
        let c0_limb_1_col190 = eval.next_trace_mask();
        let c0_limb_2_col191 = eval.next_trace_mask();
        let c0_limb_3_col192 = eval.next_trace_mask();
        let c0_limb_4_col193 = eval.next_trace_mask();
        let c0_limb_5_col194 = eval.next_trace_mask();
        let c0_limb_6_col195 = eval.next_trace_mask();
        let c0_limb_7_col196 = eval.next_trace_mask();
        let c0_limb_8_col197 = eval.next_trace_mask();
        let c0_limb_9_col198 = eval.next_trace_mask();
        let c0_limb_10_col199 = eval.next_trace_mask();
        let c1_id_col200 = eval.next_trace_mask();
        let c1_limb_0_col201 = eval.next_trace_mask();
        let c1_limb_1_col202 = eval.next_trace_mask();
        let c1_limb_2_col203 = eval.next_trace_mask();
        let c1_limb_3_col204 = eval.next_trace_mask();
        let c1_limb_4_col205 = eval.next_trace_mask();
        let c1_limb_5_col206 = eval.next_trace_mask();
        let c1_limb_6_col207 = eval.next_trace_mask();
        let c1_limb_7_col208 = eval.next_trace_mask();
        let c1_limb_8_col209 = eval.next_trace_mask();
        let c1_limb_9_col210 = eval.next_trace_mask();
        let c1_limb_10_col211 = eval.next_trace_mask();
        let c2_id_col212 = eval.next_trace_mask();
        let c2_limb_0_col213 = eval.next_trace_mask();
        let c2_limb_1_col214 = eval.next_trace_mask();
        let c2_limb_2_col215 = eval.next_trace_mask();
        let c2_limb_3_col216 = eval.next_trace_mask();
        let c2_limb_4_col217 = eval.next_trace_mask();
        let c2_limb_5_col218 = eval.next_trace_mask();
        let c2_limb_6_col219 = eval.next_trace_mask();
        let c2_limb_7_col220 = eval.next_trace_mask();
        let c2_limb_8_col221 = eval.next_trace_mask();
        let c2_limb_9_col222 = eval.next_trace_mask();
        let c2_limb_10_col223 = eval.next_trace_mask();
        let c3_id_col224 = eval.next_trace_mask();
        let c3_limb_0_col225 = eval.next_trace_mask();
        let c3_limb_1_col226 = eval.next_trace_mask();
        let c3_limb_2_col227 = eval.next_trace_mask();
        let c3_limb_3_col228 = eval.next_trace_mask();
        let c3_limb_4_col229 = eval.next_trace_mask();
        let c3_limb_5_col230 = eval.next_trace_mask();
        let c3_limb_6_col231 = eval.next_trace_mask();
        let c3_limb_7_col232 = eval.next_trace_mask();
        let c3_limb_8_col233 = eval.next_trace_mask();
        let c3_limb_9_col234 = eval.next_trace_mask();
        let c3_limb_10_col235 = eval.next_trace_mask();
        let sub_p_bit_col236 = eval.next_trace_mask();
        let carry_0_col237 = eval.next_trace_mask();
        let carry_1_col238 = eval.next_trace_mask();
        let carry_2_col239 = eval.next_trace_mask();
        let carry_3_col240 = eval.next_trace_mask();
        let carry_4_col241 = eval.next_trace_mask();
        let carry_5_col242 = eval.next_trace_mask();
        let carry_6_col243 = eval.next_trace_mask();
        let carry_7_col244 = eval.next_trace_mask();
        let carry_8_col245 = eval.next_trace_mask();
        let carry_9_col246 = eval.next_trace_mask();
        let carry_10_col247 = eval.next_trace_mask();
        let carry_11_col248 = eval.next_trace_mask();
        let carry_12_col249 = eval.next_trace_mask();
        let carry_13_col250 = eval.next_trace_mask();

        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [mod_utils_output_tmp_c1b19_91_limb_0, mod_utils_output_tmp_c1b19_91_limb_1, mod_utils_output_tmp_c1b19_91_limb_2, mod_utils_output_tmp_c1b19_91_limb_3, mod_utils_output_tmp_c1b19_91_limb_4, mod_utils_output_tmp_c1b19_91_limb_5, mod_utils_output_tmp_c1b19_91_limb_6, mod_utils_output_tmp_c1b19_91_limb_7, mod_utils_output_tmp_c1b19_91_limb_8, mod_utils_output_tmp_c1b19_91_limb_9, mod_utils_output_tmp_c1b19_91_limb_10, mod_utils_output_tmp_c1b19_91_limb_11, mod_utils_output_tmp_c1b19_91_limb_12, mod_utils_output_tmp_c1b19_91_limb_13, mod_utils_output_tmp_c1b19_91_limb_14, mod_utils_output_tmp_c1b19_91_limb_15, mod_utils_output_tmp_c1b19_91_limb_16, mod_utils_output_tmp_c1b19_91_limb_17, mod_utils_output_tmp_c1b19_91_limb_18, mod_utils_output_tmp_c1b19_91_limb_19, mod_utils_output_tmp_c1b19_91_limb_20, mod_utils_output_tmp_c1b19_91_limb_21, mod_utils_output_tmp_c1b19_91_limb_22, mod_utils_output_tmp_c1b19_91_limb_23, mod_utils_output_tmp_c1b19_91_limb_24, mod_utils_output_tmp_c1b19_91_limb_25, mod_utils_output_tmp_c1b19_91_limb_26, mod_utils_output_tmp_c1b19_91_limb_27, mod_utils_output_tmp_c1b19_91_limb_28, mod_utils_output_tmp_c1b19_91_limb_29, mod_utils_output_tmp_c1b19_91_limb_30, mod_utils_output_tmp_c1b19_91_limb_31, mod_utils_output_tmp_c1b19_91_limb_32, mod_utils_output_tmp_c1b19_91_limb_33, mod_utils_output_tmp_c1b19_91_limb_34, mod_utils_output_tmp_c1b19_91_limb_35, mod_utils_output_tmp_c1b19_91_limb_36, mod_utils_output_tmp_c1b19_91_limb_37, mod_utils_output_tmp_c1b19_91_limb_38, mod_utils_output_tmp_c1b19_91_limb_39, mod_utils_output_tmp_c1b19_91_limb_40, mod_utils_output_tmp_c1b19_91_limb_41, mod_utils_output_tmp_c1b19_91_limb_42, mod_utils_output_tmp_c1b19_91_limb_43, mod_utils_output_tmp_c1b19_91_limb_44, mod_utils_output_tmp_c1b19_91_limb_45, mod_utils_output_tmp_c1b19_91_limb_46, mod_utils_output_tmp_c1b19_91_limb_47, mod_utils_output_tmp_c1b19_91_limb_48, mod_utils_output_tmp_c1b19_91_limb_49, mod_utils_output_tmp_c1b19_91_limb_50, mod_utils_output_tmp_c1b19_91_limb_51, mod_utils_output_tmp_c1b19_91_limb_52, mod_utils_output_tmp_c1b19_91_limb_53, mod_utils_output_tmp_c1b19_91_limb_54, mod_utils_output_tmp_c1b19_91_limb_55, mod_utils_output_tmp_c1b19_91_limb_56, mod_utils_output_tmp_c1b19_91_limb_57, mod_utils_output_tmp_c1b19_91_limb_58, mod_utils_output_tmp_c1b19_91_limb_59, mod_utils_output_tmp_c1b19_91_limb_60, mod_utils_output_tmp_c1b19_91_limb_61, mod_utils_output_tmp_c1b19_91_limb_62, mod_utils_output_tmp_c1b19_91_limb_63, mod_utils_output_tmp_c1b19_91_limb_64, mod_utils_output_tmp_c1b19_91_limb_65, mod_utils_output_tmp_c1b19_91_limb_66, mod_utils_output_tmp_c1b19_91_limb_67, mod_utils_output_tmp_c1b19_91_limb_68, mod_utils_output_tmp_c1b19_91_limb_69, mod_utils_output_tmp_c1b19_91_limb_70, mod_utils_output_tmp_c1b19_91_limb_71, mod_utils_output_tmp_c1b19_91_limb_72, mod_utils_output_tmp_c1b19_91_limb_73, mod_utils_output_tmp_c1b19_91_limb_74, mod_utils_output_tmp_c1b19_91_limb_75, mod_utils_output_tmp_c1b19_91_limb_76, mod_utils_output_tmp_c1b19_91_limb_77, mod_utils_output_tmp_c1b19_91_limb_78, mod_utils_output_tmp_c1b19_91_limb_79, mod_utils_output_tmp_c1b19_91_limb_80, mod_utils_output_tmp_c1b19_91_limb_81, mod_utils_output_tmp_c1b19_91_limb_82, mod_utils_output_tmp_c1b19_91_limb_83, mod_utils_output_tmp_c1b19_91_limb_84, mod_utils_output_tmp_c1b19_91_limb_85, mod_utils_output_tmp_c1b19_91_limb_86, mod_utils_output_tmp_c1b19_91_limb_87, mod_utils_output_tmp_c1b19_91_limb_88, mod_utils_output_tmp_c1b19_91_limb_89, mod_utils_output_tmp_c1b19_91_limb_90, mod_utils_output_tmp_c1b19_91_limb_91, mod_utils_output_tmp_c1b19_91_limb_92, mod_utils_output_tmp_c1b19_91_limb_93, mod_utils_output_tmp_c1b19_91_limb_94, mod_utils_output_tmp_c1b19_91_limb_95, mod_utils_output_tmp_c1b19_91_limb_96, mod_utils_output_tmp_c1b19_91_limb_97, mod_utils_output_tmp_c1b19_91_limb_98, mod_utils_output_tmp_c1b19_91_limb_99, mod_utils_output_tmp_c1b19_91_limb_100, mod_utils_output_tmp_c1b19_91_limb_101, mod_utils_output_tmp_c1b19_91_limb_102, mod_utils_output_tmp_c1b19_91_limb_103, mod_utils_output_tmp_c1b19_91_limb_104, mod_utils_output_tmp_c1b19_91_limb_105, mod_utils_output_tmp_c1b19_91_limb_106, mod_utils_output_tmp_c1b19_91_limb_107, mod_utils_output_tmp_c1b19_91_limb_108, mod_utils_output_tmp_c1b19_91_limb_109, mod_utils_output_tmp_c1b19_91_limb_110, mod_utils_output_tmp_c1b19_91_limb_111, mod_utils_output_tmp_c1b19_91_limb_112, mod_utils_output_tmp_c1b19_91_limb_113, mod_utils_output_tmp_c1b19_91_limb_114, mod_utils_output_tmp_c1b19_91_limb_115, mod_utils_output_tmp_c1b19_91_limb_116, mod_utils_output_tmp_c1b19_91_limb_117, mod_utils_output_tmp_c1b19_91_limb_118, mod_utils_output_tmp_c1b19_91_limb_119, mod_utils_output_tmp_c1b19_91_limb_120, mod_utils_output_tmp_c1b19_91_limb_121, mod_utils_output_tmp_c1b19_91_limb_122, mod_utils_output_tmp_c1b19_91_limb_123, mod_utils_output_tmp_c1b19_91_limb_124, mod_utils_output_tmp_c1b19_91_limb_125, mod_utils_output_tmp_c1b19_91_limb_126, mod_utils_output_tmp_c1b19_91_limb_127, mod_utils_output_tmp_c1b19_91_limb_128, mod_utils_output_tmp_c1b19_91_limb_129, mod_utils_output_tmp_c1b19_91_limb_130, mod_utils_output_tmp_c1b19_91_limb_131, mod_utils_output_tmp_c1b19_91_limb_132, mod_utils_output_tmp_c1b19_91_limb_133, mod_utils_output_tmp_c1b19_91_limb_134, mod_utils_output_tmp_c1b19_91_limb_135, mod_utils_output_tmp_c1b19_91_limb_136, mod_utils_output_tmp_c1b19_91_limb_137, mod_utils_output_tmp_c1b19_91_limb_138, mod_utils_output_tmp_c1b19_91_limb_139, mod_utils_output_tmp_c1b19_91_limb_140, mod_utils_output_tmp_c1b19_91_limb_141, mod_utils_output_tmp_c1b19_91_limb_142, mod_utils_output_tmp_c1b19_91_limb_143, mod_utils_output_tmp_c1b19_91_limb_144, mod_utils_output_tmp_c1b19_91_limb_145, mod_utils_output_tmp_c1b19_91_limb_146, mod_utils_output_tmp_c1b19_91_limb_147, mod_utils_output_tmp_c1b19_91_limb_148, mod_utils_output_tmp_c1b19_91_limb_149, mod_utils_output_tmp_c1b19_91_limb_150, mod_utils_output_tmp_c1b19_91_limb_151, mod_utils_output_tmp_c1b19_91_limb_152, mod_utils_output_tmp_c1b19_91_limb_153, mod_utils_output_tmp_c1b19_91_limb_154, mod_utils_output_tmp_c1b19_91_limb_155, mod_utils_output_tmp_c1b19_91_limb_156, mod_utils_output_tmp_c1b19_91_limb_157, mod_utils_output_tmp_c1b19_91_limb_158, mod_utils_output_tmp_c1b19_91_limb_159, mod_utils_output_tmp_c1b19_91_limb_160, mod_utils_output_tmp_c1b19_91_limb_161, mod_utils_output_tmp_c1b19_91_limb_162, mod_utils_output_tmp_c1b19_91_limb_163, mod_utils_output_tmp_c1b19_91_limb_164, mod_utils_output_tmp_c1b19_91_limb_165, mod_utils_output_tmp_c1b19_91_limb_166, mod_utils_output_tmp_c1b19_91_limb_167, mod_utils_output_tmp_c1b19_91_limb_168, mod_utils_output_tmp_c1b19_91_limb_169, mod_utils_output_tmp_c1b19_91_limb_170, mod_utils_output_tmp_c1b19_91_limb_171, mod_utils_output_tmp_c1b19_91_limb_172, mod_utils_output_tmp_c1b19_91_limb_173, mod_utils_output_tmp_c1b19_91_limb_174, mod_utils_output_tmp_c1b19_91_limb_175, mod_utils_output_tmp_c1b19_91_limb_176, mod_utils_output_tmp_c1b19_91_limb_177, mod_utils_output_tmp_c1b19_91_limb_178, mod_utils_output_tmp_c1b19_91_limb_179, mod_utils_output_tmp_c1b19_91_limb_180, mod_utils_output_tmp_c1b19_91_limb_181, mod_utils_output_tmp_c1b19_91_limb_182, mod_utils_output_tmp_c1b19_91_limb_183, mod_utils_output_tmp_c1b19_91_limb_184, mod_utils_output_tmp_c1b19_91_limb_185, mod_utils_output_tmp_c1b19_91_limb_186, mod_utils_output_tmp_c1b19_91_limb_187, mod_utils_output_tmp_c1b19_91_limb_188, mod_utils_output_tmp_c1b19_91_limb_189, mod_utils_output_tmp_c1b19_91_limb_190, mod_utils_output_tmp_c1b19_91_limb_191, mod_utils_output_tmp_c1b19_91_limb_192, mod_utils_output_tmp_c1b19_91_limb_193, mod_utils_output_tmp_c1b19_91_limb_194, mod_utils_output_tmp_c1b19_91_limb_195, mod_utils_output_tmp_c1b19_91_limb_196, mod_utils_output_tmp_c1b19_91_limb_197, mod_utils_output_tmp_c1b19_91_limb_198, mod_utils_output_tmp_c1b19_91_limb_199, mod_utils_output_tmp_c1b19_91_limb_200, mod_utils_output_tmp_c1b19_91_limb_201, mod_utils_output_tmp_c1b19_91_limb_202, mod_utils_output_tmp_c1b19_91_limb_203, mod_utils_output_tmp_c1b19_91_limb_204, mod_utils_output_tmp_c1b19_91_limb_205, mod_utils_output_tmp_c1b19_91_limb_206, mod_utils_output_tmp_c1b19_91_limb_207, mod_utils_output_tmp_c1b19_91_limb_208, mod_utils_output_tmp_c1b19_91_limb_209, mod_utils_output_tmp_c1b19_91_limb_210, mod_utils_output_tmp_c1b19_91_limb_211, mod_utils_output_tmp_c1b19_91_limb_212, mod_utils_output_tmp_c1b19_91_limb_213, mod_utils_output_tmp_c1b19_91_limb_214, mod_utils_output_tmp_c1b19_91_limb_215, mod_utils_output_tmp_c1b19_91_limb_216, mod_utils_output_tmp_c1b19_91_limb_217, mod_utils_output_tmp_c1b19_91_limb_218, mod_utils_output_tmp_c1b19_91_limb_219, mod_utils_output_tmp_c1b19_91_limb_220, mod_utils_output_tmp_c1b19_91_limb_221, mod_utils_output_tmp_c1b19_91_limb_222, mod_utils_output_tmp_c1b19_91_limb_223, mod_utils_output_tmp_c1b19_91_limb_224, mod_utils_output_tmp_c1b19_91_limb_225, mod_utils_output_tmp_c1b19_91_limb_226, mod_utils_output_tmp_c1b19_91_limb_227, mod_utils_output_tmp_c1b19_91_limb_228, mod_utils_output_tmp_c1b19_91_limb_229, mod_utils_output_tmp_c1b19_91_limb_230, mod_utils_output_tmp_c1b19_91_limb_231, mod_utils_output_tmp_c1b19_91_limb_232, mod_utils_output_tmp_c1b19_91_limb_233, mod_utils_output_tmp_c1b19_91_limb_234, mod_utils_output_tmp_c1b19_91_limb_235, mod_utils_output_tmp_c1b19_91_limb_236, mod_utils_output_tmp_c1b19_91_limb_237, mod_utils_output_tmp_c1b19_91_limb_238, mod_utils_output_tmp_c1b19_91_limb_239, mod_utils_output_tmp_c1b19_91_limb_240, mod_utils_output_tmp_c1b19_91_limb_241, mod_utils_output_tmp_c1b19_91_limb_242, mod_utils_output_tmp_c1b19_91_limb_243, mod_utils_output_tmp_c1b19_91_limb_244, mod_utils_output_tmp_c1b19_91_limb_245, mod_utils_output_tmp_c1b19_91_limb_246, mod_utils_output_tmp_c1b19_91_limb_247, mod_utils_output_tmp_c1b19_91_limb_248, mod_utils_output_tmp_c1b19_91_limb_249, mod_utils_output_tmp_c1b19_91_limb_250, mod_utils_output_tmp_c1b19_91_limb_251, mod_utils_output_tmp_c1b19_91_limb_252, mod_utils_output_tmp_c1b19_91_limb_253, mod_utils_output_tmp_c1b19_91_limb_254, mod_utils_output_tmp_c1b19_91_limb_255, mod_utils_output_tmp_c1b19_91_limb_256, mod_utils_output_tmp_c1b19_91_limb_257, mod_utils_output_tmp_c1b19_91_limb_258, mod_utils_output_tmp_c1b19_91_limb_259, mod_utils_output_tmp_c1b19_91_limb_260, mod_utils_output_tmp_c1b19_91_limb_261, mod_utils_output_tmp_c1b19_91_limb_262, mod_utils_output_tmp_c1b19_91_limb_263, mod_utils_output_tmp_c1b19_91_limb_264, mod_utils_output_tmp_c1b19_91_limb_265, mod_utils_output_tmp_c1b19_91_limb_266, mod_utils_output_tmp_c1b19_91_limb_267, mod_utils_output_tmp_c1b19_91_limb_268, mod_utils_output_tmp_c1b19_91_limb_269, mod_utils_output_tmp_c1b19_91_limb_270, mod_utils_output_tmp_c1b19_91_limb_271, mod_utils_output_tmp_c1b19_91_limb_272, mod_utils_output_tmp_c1b19_91_limb_273, mod_utils_output_tmp_c1b19_91_limb_274, mod_utils_output_tmp_c1b19_91_limb_275, mod_utils_output_tmp_c1b19_91_limb_276, mod_utils_output_tmp_c1b19_91_limb_277, mod_utils_output_tmp_c1b19_91_limb_278, mod_utils_output_tmp_c1b19_91_limb_279, mod_utils_output_tmp_c1b19_91_limb_280, mod_utils_output_tmp_c1b19_91_limb_281, mod_utils_output_tmp_c1b19_91_limb_282, mod_utils_output_tmp_c1b19_91_limb_283, mod_utils_output_tmp_c1b19_91_limb_284, mod_utils_output_tmp_c1b19_91_limb_285, mod_utils_output_tmp_c1b19_91_limb_286, mod_utils_output_tmp_c1b19_91_limb_287, mod_utils_output_tmp_c1b19_91_limb_288, mod_utils_output_tmp_c1b19_91_limb_289, mod_utils_output_tmp_c1b19_91_limb_290, mod_utils_output_tmp_c1b19_91_limb_291, mod_utils_output_tmp_c1b19_91_limb_292, mod_utils_output_tmp_c1b19_91_limb_293, mod_utils_output_tmp_c1b19_91_limb_294, mod_utils_output_tmp_c1b19_91_limb_295, mod_utils_output_tmp_c1b19_91_limb_296, mod_utils_output_tmp_c1b19_91_limb_297, mod_utils_output_tmp_c1b19_91_limb_298, mod_utils_output_tmp_c1b19_91_limb_299, mod_utils_output_tmp_c1b19_91_limb_300, mod_utils_output_tmp_c1b19_91_limb_301, mod_utils_output_tmp_c1b19_91_limb_302, mod_utils_output_tmp_c1b19_91_limb_303, mod_utils_output_tmp_c1b19_91_limb_304, mod_utils_output_tmp_c1b19_91_limb_305, mod_utils_output_tmp_c1b19_91_limb_306, mod_utils_output_tmp_c1b19_91_limb_307, mod_utils_output_tmp_c1b19_91_limb_308, mod_utils_output_tmp_c1b19_91_limb_309, mod_utils_output_tmp_c1b19_91_limb_310, mod_utils_output_tmp_c1b19_91_limb_311, mod_utils_output_tmp_c1b19_91_limb_312, mod_utils_output_tmp_c1b19_91_limb_313, mod_utils_output_tmp_c1b19_91_limb_314, mod_utils_output_tmp_c1b19_91_limb_315, mod_utils_output_tmp_c1b19_91_limb_316, mod_utils_output_tmp_c1b19_91_limb_317, mod_utils_output_tmp_c1b19_91_limb_318, mod_utils_output_tmp_c1b19_91_limb_319, mod_utils_output_tmp_c1b19_91_limb_320, mod_utils_output_tmp_c1b19_91_limb_321, mod_utils_output_tmp_c1b19_91_limb_322, mod_utils_output_tmp_c1b19_91_limb_323, mod_utils_output_tmp_c1b19_91_limb_324, mod_utils_output_tmp_c1b19_91_limb_325, mod_utils_output_tmp_c1b19_91_limb_326, mod_utils_output_tmp_c1b19_91_limb_327, mod_utils_output_tmp_c1b19_91_limb_328, mod_utils_output_tmp_c1b19_91_limb_329, mod_utils_output_tmp_c1b19_91_limb_330, mod_utils_output_tmp_c1b19_91_limb_331, mod_utils_output_tmp_c1b19_91_limb_332, mod_utils_output_tmp_c1b19_91_limb_333, mod_utils_output_tmp_c1b19_91_limb_334, mod_utils_output_tmp_c1b19_91_limb_335, mod_utils_output_tmp_c1b19_91_limb_336, mod_utils_output_tmp_c1b19_91_limb_337, mod_utils_output_tmp_c1b19_91_limb_338, mod_utils_output_tmp_c1b19_91_limb_339, mod_utils_output_tmp_c1b19_91_limb_340, mod_utils_output_tmp_c1b19_91_limb_341, mod_utils_output_tmp_c1b19_91_limb_342, mod_utils_output_tmp_c1b19_91_limb_343, mod_utils_output_tmp_c1b19_91_limb_344, mod_utils_output_tmp_c1b19_91_limb_345, mod_utils_output_tmp_c1b19_91_limb_346, mod_utils_output_tmp_c1b19_91_limb_347, mod_utils_output_tmp_c1b19_91_limb_348, mod_utils_output_tmp_c1b19_91_limb_349, mod_utils_output_tmp_c1b19_91_limb_350, mod_utils_output_tmp_c1b19_91_limb_351, mod_utils_output_tmp_c1b19_91_limb_352, mod_utils_output_tmp_c1b19_91_limb_353, mod_utils_output_tmp_c1b19_91_limb_354, mod_utils_output_tmp_c1b19_91_limb_355, mod_utils_output_tmp_c1b19_91_limb_356, mod_utils_output_tmp_c1b19_91_limb_357, mod_utils_output_tmp_c1b19_91_limb_358, mod_utils_output_tmp_c1b19_91_limb_359, mod_utils_output_tmp_c1b19_91_limb_360, mod_utils_output_tmp_c1b19_91_limb_361, mod_utils_output_tmp_c1b19_91_limb_362, mod_utils_output_tmp_c1b19_91_limb_363, mod_utils_output_tmp_c1b19_91_limb_364, mod_utils_output_tmp_c1b19_91_limb_365, mod_utils_output_tmp_c1b19_91_limb_366, mod_utils_output_tmp_c1b19_91_limb_367, mod_utils_output_tmp_c1b19_91_limb_368, mod_utils_output_tmp_c1b19_91_limb_369, mod_utils_output_tmp_c1b19_91_limb_370, mod_utils_output_tmp_c1b19_91_limb_371, mod_utils_output_tmp_c1b19_91_limb_372, mod_utils_output_tmp_c1b19_91_limb_373, mod_utils_output_tmp_c1b19_91_limb_374, mod_utils_output_tmp_c1b19_91_limb_375, mod_utils_output_tmp_c1b19_91_limb_376, mod_utils_output_tmp_c1b19_91_limb_377, mod_utils_output_tmp_c1b19_91_limb_378, mod_utils_output_tmp_c1b19_91_limb_379, mod_utils_output_tmp_c1b19_91_limb_380, mod_utils_output_tmp_c1b19_91_limb_381, mod_utils_output_tmp_c1b19_91_limb_382, mod_utils_output_tmp_c1b19_91_limb_383, mod_utils_output_tmp_c1b19_91_limb_384, mod_utils_output_tmp_c1b19_91_limb_385, mod_utils_output_tmp_c1b19_91_limb_386, mod_utils_output_tmp_c1b19_91_limb_387, mod_utils_output_tmp_c1b19_91_limb_388, mod_utils_output_tmp_c1b19_91_limb_389, mod_utils_output_tmp_c1b19_91_limb_390, mod_utils_output_tmp_c1b19_91_limb_391, mod_utils_output_tmp_c1b19_91_limb_392, mod_utils_output_tmp_c1b19_91_limb_393, mod_utils_output_tmp_c1b19_91_limb_394, mod_utils_output_tmp_c1b19_91_limb_395, mod_utils_output_tmp_c1b19_91_limb_396, mod_utils_output_tmp_c1b19_91_limb_397, mod_utils_output_tmp_c1b19_91_limb_398, mod_utils_output_tmp_c1b19_91_limb_399, mod_utils_output_tmp_c1b19_91_limb_400, mod_utils_output_tmp_c1b19_91_limb_401, mod_utils_output_tmp_c1b19_91_limb_402, mod_utils_output_tmp_c1b19_91_limb_403, mod_utils_output_tmp_c1b19_91_limb_404, mod_utils_output_tmp_c1b19_91_limb_405, mod_utils_output_tmp_c1b19_91_limb_406, mod_utils_output_tmp_c1b19_91_limb_407, mod_utils_output_tmp_c1b19_91_limb_408, mod_utils_output_tmp_c1b19_91_limb_409, mod_utils_output_tmp_c1b19_91_limb_410, mod_utils_output_tmp_c1b19_91_limb_411, mod_utils_output_tmp_c1b19_91_limb_412, mod_utils_output_tmp_c1b19_91_limb_413, mod_utils_output_tmp_c1b19_91_limb_414, mod_utils_output_tmp_c1b19_91_limb_415, mod_utils_output_tmp_c1b19_91_limb_416, mod_utils_output_tmp_c1b19_91_limb_417, mod_utils_output_tmp_c1b19_91_limb_418, mod_utils_output_tmp_c1b19_91_limb_419, mod_utils_output_tmp_c1b19_91_limb_420, mod_utils_output_tmp_c1b19_91_limb_421, mod_utils_output_tmp_c1b19_91_limb_422, mod_utils_output_tmp_c1b19_91_limb_423, mod_utils_output_tmp_c1b19_91_limb_424, mod_utils_output_tmp_c1b19_91_limb_425, mod_utils_output_tmp_c1b19_91_limb_426, mod_utils_output_tmp_c1b19_91_limb_427, mod_utils_output_tmp_c1b19_91_limb_428, mod_utils_output_tmp_c1b19_91_limb_429, mod_utils_output_tmp_c1b19_91_limb_430, mod_utils_output_tmp_c1b19_91_limb_431, mod_utils_output_tmp_c1b19_91_limb_432, mod_utils_output_tmp_c1b19_91_limb_433, mod_utils_output_tmp_c1b19_91_limb_434, mod_utils_output_tmp_c1b19_91_limb_435, mod_utils_output_tmp_c1b19_91_limb_436, mod_utils_output_tmp_c1b19_91_limb_437, mod_utils_output_tmp_c1b19_91_limb_438, mod_utils_output_tmp_c1b19_91_limb_439, mod_utils_output_tmp_c1b19_91_limb_440, mod_utils_output_tmp_c1b19_91_limb_441, mod_utils_output_tmp_c1b19_91_limb_442, mod_utils_output_tmp_c1b19_91_limb_443, mod_utils_output_tmp_c1b19_91_limb_444, mod_utils_output_tmp_c1b19_91_limb_445, mod_utils_output_tmp_c1b19_91_limb_446, mod_utils_output_tmp_c1b19_91_limb_447] =
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
                offsets_ptr_id_col53.clone(),
                offsets_ptr_limb_0_col54.clone(),
                offsets_ptr_limb_1_col55.clone(),
                offsets_ptr_limb_2_col56.clone(),
                offsets_ptr_prev_id_col57.clone(),
                offsets_ptr_prev_limb_0_col58.clone(),
                offsets_ptr_prev_limb_1_col59.clone(),
                offsets_ptr_prev_limb_2_col60.clone(),
                n_id_col61.clone(),
                n_limb_0_col62.clone(),
                n_limb_1_col63.clone(),
                n_limb_2_col64.clone(),
                n_prev_id_col65.clone(),
                n_prev_limb_0_col66.clone(),
                n_prev_limb_1_col67.clone(),
                n_prev_limb_2_col68.clone(),
                values_ptr_prev_id_col69.clone(),
                p_prev0_id_col70.clone(),
                p_prev1_id_col71.clone(),
                p_prev2_id_col72.clone(),
                p_prev3_id_col73.clone(),
                offsets_a_id_col74.clone(),
                msb_col75.clone(),
                mid_limbs_set_col76.clone(),
                offsets_a_limb_0_col77.clone(),
                offsets_a_limb_1_col78.clone(),
                offsets_a_limb_2_col79.clone(),
                offsets_b_id_col80.clone(),
                msb_col81.clone(),
                mid_limbs_set_col82.clone(),
                offsets_b_limb_0_col83.clone(),
                offsets_b_limb_1_col84.clone(),
                offsets_b_limb_2_col85.clone(),
                offsets_c_id_col86.clone(),
                msb_col87.clone(),
                mid_limbs_set_col88.clone(),
                offsets_c_limb_0_col89.clone(),
                offsets_c_limb_1_col90.clone(),
                offsets_c_limb_2_col91.clone(),
                a0_id_col92.clone(),
                a0_limb_0_col93.clone(),
                a0_limb_1_col94.clone(),
                a0_limb_2_col95.clone(),
                a0_limb_3_col96.clone(),
                a0_limb_4_col97.clone(),
                a0_limb_5_col98.clone(),
                a0_limb_6_col99.clone(),
                a0_limb_7_col100.clone(),
                a0_limb_8_col101.clone(),
                a0_limb_9_col102.clone(),
                a0_limb_10_col103.clone(),
                a1_id_col104.clone(),
                a1_limb_0_col105.clone(),
                a1_limb_1_col106.clone(),
                a1_limb_2_col107.clone(),
                a1_limb_3_col108.clone(),
                a1_limb_4_col109.clone(),
                a1_limb_5_col110.clone(),
                a1_limb_6_col111.clone(),
                a1_limb_7_col112.clone(),
                a1_limb_8_col113.clone(),
                a1_limb_9_col114.clone(),
                a1_limb_10_col115.clone(),
                a2_id_col116.clone(),
                a2_limb_0_col117.clone(),
                a2_limb_1_col118.clone(),
                a2_limb_2_col119.clone(),
                a2_limb_3_col120.clone(),
                a2_limb_4_col121.clone(),
                a2_limb_5_col122.clone(),
                a2_limb_6_col123.clone(),
                a2_limb_7_col124.clone(),
                a2_limb_8_col125.clone(),
                a2_limb_9_col126.clone(),
                a2_limb_10_col127.clone(),
                a3_id_col128.clone(),
                a3_limb_0_col129.clone(),
                a3_limb_1_col130.clone(),
                a3_limb_2_col131.clone(),
                a3_limb_3_col132.clone(),
                a3_limb_4_col133.clone(),
                a3_limb_5_col134.clone(),
                a3_limb_6_col135.clone(),
                a3_limb_7_col136.clone(),
                a3_limb_8_col137.clone(),
                a3_limb_9_col138.clone(),
                a3_limb_10_col139.clone(),
                b0_id_col140.clone(),
                b0_limb_0_col141.clone(),
                b0_limb_1_col142.clone(),
                b0_limb_2_col143.clone(),
                b0_limb_3_col144.clone(),
                b0_limb_4_col145.clone(),
                b0_limb_5_col146.clone(),
                b0_limb_6_col147.clone(),
                b0_limb_7_col148.clone(),
                b0_limb_8_col149.clone(),
                b0_limb_9_col150.clone(),
                b0_limb_10_col151.clone(),
                b1_id_col152.clone(),
                b1_limb_0_col153.clone(),
                b1_limb_1_col154.clone(),
                b1_limb_2_col155.clone(),
                b1_limb_3_col156.clone(),
                b1_limb_4_col157.clone(),
                b1_limb_5_col158.clone(),
                b1_limb_6_col159.clone(),
                b1_limb_7_col160.clone(),
                b1_limb_8_col161.clone(),
                b1_limb_9_col162.clone(),
                b1_limb_10_col163.clone(),
                b2_id_col164.clone(),
                b2_limb_0_col165.clone(),
                b2_limb_1_col166.clone(),
                b2_limb_2_col167.clone(),
                b2_limb_3_col168.clone(),
                b2_limb_4_col169.clone(),
                b2_limb_5_col170.clone(),
                b2_limb_6_col171.clone(),
                b2_limb_7_col172.clone(),
                b2_limb_8_col173.clone(),
                b2_limb_9_col174.clone(),
                b2_limb_10_col175.clone(),
                b3_id_col176.clone(),
                b3_limb_0_col177.clone(),
                b3_limb_1_col178.clone(),
                b3_limb_2_col179.clone(),
                b3_limb_3_col180.clone(),
                b3_limb_4_col181.clone(),
                b3_limb_5_col182.clone(),
                b3_limb_6_col183.clone(),
                b3_limb_7_col184.clone(),
                b3_limb_8_col185.clone(),
                b3_limb_9_col186.clone(),
                b3_limb_10_col187.clone(),
                c0_id_col188.clone(),
                c0_limb_0_col189.clone(),
                c0_limb_1_col190.clone(),
                c0_limb_2_col191.clone(),
                c0_limb_3_col192.clone(),
                c0_limb_4_col193.clone(),
                c0_limb_5_col194.clone(),
                c0_limb_6_col195.clone(),
                c0_limb_7_col196.clone(),
                c0_limb_8_col197.clone(),
                c0_limb_9_col198.clone(),
                c0_limb_10_col199.clone(),
                c1_id_col200.clone(),
                c1_limb_0_col201.clone(),
                c1_limb_1_col202.clone(),
                c1_limb_2_col203.clone(),
                c1_limb_3_col204.clone(),
                c1_limb_4_col205.clone(),
                c1_limb_5_col206.clone(),
                c1_limb_6_col207.clone(),
                c1_limb_7_col208.clone(),
                c1_limb_8_col209.clone(),
                c1_limb_9_col210.clone(),
                c1_limb_10_col211.clone(),
                c2_id_col212.clone(),
                c2_limb_0_col213.clone(),
                c2_limb_1_col214.clone(),
                c2_limb_2_col215.clone(),
                c2_limb_3_col216.clone(),
                c2_limb_4_col217.clone(),
                c2_limb_5_col218.clone(),
                c2_limb_6_col219.clone(),
                c2_limb_7_col220.clone(),
                c2_limb_8_col221.clone(),
                c2_limb_9_col222.clone(),
                c2_limb_10_col223.clone(),
                c3_id_col224.clone(),
                c3_limb_0_col225.clone(),
                c3_limb_1_col226.clone(),
                c3_limb_2_col227.clone(),
                c3_limb_3_col228.clone(),
                c3_limb_4_col229.clone(),
                c3_limb_5_col230.clone(),
                c3_limb_6_col231.clone(),
                c3_limb_7_col232.clone(),
                c3_limb_8_col233.clone(),
                c3_limb_9_col234.clone(),
                c3_limb_10_col235.clone(),
                &mut eval,
                &self.memory_address_to_id_lookup_elements,
                &self.memory_id_to_big_lookup_elements,
            );
        // make sure sub_p_bit is 0 or 1..
        eval.add_constraint(
            ((sub_p_bit_col236.clone() - M31_1.clone()) * sub_p_bit_col236.clone()),
        );
        // carry is 0 or 1 or -1..
        eval.add_constraint(
            (carry_0_col237.clone()
                * ((carry_0_col237.clone() * carry_0_col237.clone()) - M31_1.clone())),
        );
        // carry is 0 or 1 or -1..
        eval.add_constraint(
            (carry_1_col238.clone()
                * ((carry_1_col238.clone() * carry_1_col238.clone()) - M31_1.clone())),
        );
        // carry is 0 or 1 or -1..
        eval.add_constraint(
            (carry_2_col239.clone()
                * ((carry_2_col239.clone() * carry_2_col239.clone()) - M31_1.clone())),
        );
        // carry is 0 or 1 or -1..
        eval.add_constraint(
            (carry_3_col240.clone()
                * ((carry_3_col240.clone() * carry_3_col240.clone()) - M31_1.clone())),
        );
        // carry is 0 or 1 or -1..
        eval.add_constraint(
            (carry_4_col241.clone()
                * ((carry_4_col241.clone() * carry_4_col241.clone()) - M31_1.clone())),
        );
        // carry is 0 or 1 or -1..
        eval.add_constraint(
            (carry_5_col242.clone()
                * ((carry_5_col242.clone() * carry_5_col242.clone()) - M31_1.clone())),
        );
        // carry is 0 or 1 or -1..
        eval.add_constraint(
            (carry_6_col243.clone()
                * ((carry_6_col243.clone() * carry_6_col243.clone()) - M31_1.clone())),
        );
        // carry is 0 or 1 or -1..
        eval.add_constraint(
            (carry_7_col244.clone()
                * ((carry_7_col244.clone() * carry_7_col244.clone()) - M31_1.clone())),
        );
        // carry is 0 or 1 or -1..
        eval.add_constraint(
            (carry_8_col245.clone()
                * ((carry_8_col245.clone() * carry_8_col245.clone()) - M31_1.clone())),
        );
        // carry is 0 or 1 or -1..
        eval.add_constraint(
            (carry_9_col246.clone()
                * ((carry_9_col246.clone() * carry_9_col246.clone()) - M31_1.clone())),
        );
        // carry is 0 or 1 or -1..
        eval.add_constraint(
            (carry_10_col247.clone()
                * ((carry_10_col247.clone() * carry_10_col247.clone()) - M31_1.clone())),
        );
        // carry is 0 or 1 or -1..
        eval.add_constraint(
            (carry_11_col248.clone()
                * ((carry_11_col248.clone() * carry_11_col248.clone()) - M31_1.clone())),
        );
        // carry is 0 or 1 or -1..
        eval.add_constraint(
            (carry_12_col249.clone()
                * ((carry_12_col249.clone() * carry_12_col249.clone()) - M31_1.clone())),
        );
        // carry is 0 or 1 or -1..
        eval.add_constraint(
            (carry_13_col250.clone()
                * ((carry_13_col250.clone() * carry_13_col250.clone()) - M31_1.clone())),
        );
        // last carry needs to be 0..
        eval.add_constraint(
            ((carry_13_col250.clone()
                + (((a3_limb_9_col138.clone() + b3_limb_9_col186.clone())
                    - c3_limb_9_col234.clone())
                    - (p3_limb_9_col47.clone() * sub_p_bit_col236.clone())))
                + (M31_512.clone()
                    * (((a3_limb_10_col139.clone() + b3_limb_10_col187.clone())
                        - c3_limb_10_col235.clone())
                        - (p3_limb_10_col48.clone() * sub_p_bit_col236.clone())))),
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
    use stwo_prover::constraint_framework::expr::ExprEvaluator;
    use stwo_prover::core::fields::qm31::QM31;

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
