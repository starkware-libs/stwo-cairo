use crate::components::prelude::constraint_eval::*;

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
        let M31_0 = E::F::from(M31::from(0));
        let M31_1 = E::F::from(M31::from(1));
        let M31_134217728 = E::F::from(M31::from(134217728));
        let M31_136 = E::F::from(M31::from(136));
        let M31_2 = E::F::from(M31::from(2));
        let M31_256 = E::F::from(M31::from(256));
        let M31_262144 = E::F::from(M31::from(262144));
        let M31_3 = E::F::from(M31::from(3));
        let M31_4 = E::F::from(M31::from(4));
        let M31_5 = E::F::from(M31::from(5));
        let M31_511 = E::F::from(M31::from(511));
        let M31_512 = E::F::from(M31::from(512));
        let M31_6 = E::F::from(M31::from(6));
        let M31_7 = E::F::from(M31::from(7));
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

        // Mod Utils.

        // is_instance_0 is 0 or 1..
        eval.add_constraint(
            (is_instance_0_col0.clone() * (is_instance_0_col0.clone() - M31_1.clone())),
        );
        // is_instance_0 is 0 when instance_num is not 0..
        eval.add_constraint((is_instance_0_col0.clone() * seq.clone()));
        let prev_instance_addr_tmp_c1b19_1 = eval.add_intermediate(
            (E::F::from(M31::from(self.claim.add_mod_builtin_segment_start))
                + (M31_7.clone() * ((seq.clone() - M31_1.clone()) + is_instance_0_col0.clone()))),
        );
        let instance_addr_tmp_c1b19_2 = eval.add_intermediate(
            (E::F::from(M31::from(self.claim.add_mod_builtin_segment_start))
                + (M31_7.clone() * seq.clone())),
        );

        // Read Positive Num Bits 99.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[instance_addr_tmp_c1b19_2.clone(), p0_id_col1.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
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
            ],
        ));

        // Read Positive Num Bits 99.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (instance_addr_tmp_c1b19_2.clone() + M31_1.clone()),
                p1_id_col13.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
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
            ],
        ));

        // Read Positive Num Bits 99.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (instance_addr_tmp_c1b19_2.clone() + M31_2.clone()),
                p2_id_col25.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
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
            ],
        ));

        // Read Positive Num Bits 99.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (instance_addr_tmp_c1b19_2.clone() + M31_3.clone()),
                p3_id_col37.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
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
            ],
        ));

        // Read Positive Num Bits 27.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (instance_addr_tmp_c1b19_2.clone() + M31_4.clone()),
                values_ptr_id_col49.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                values_ptr_id_col49.clone(),
                values_ptr_limb_0_col50.clone(),
                values_ptr_limb_1_col51.clone(),
                values_ptr_limb_2_col52.clone(),
            ],
        ));

        // Read Positive Num Bits 27.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (instance_addr_tmp_c1b19_2.clone() + M31_5.clone()),
                offsets_ptr_id_col53.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                offsets_ptr_id_col53.clone(),
                offsets_ptr_limb_0_col54.clone(),
                offsets_ptr_limb_1_col55.clone(),
                offsets_ptr_limb_2_col56.clone(),
            ],
        ));

        // Read Positive Num Bits 27.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (prev_instance_addr_tmp_c1b19_1.clone() + M31_5.clone()),
                offsets_ptr_prev_id_col57.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                offsets_ptr_prev_id_col57.clone(),
                offsets_ptr_prev_limb_0_col58.clone(),
                offsets_ptr_prev_limb_1_col59.clone(),
                offsets_ptr_prev_limb_2_col60.clone(),
            ],
        ));

        // Read Positive Num Bits 27.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (instance_addr_tmp_c1b19_2.clone() + M31_6.clone()),
                n_id_col61.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                n_id_col61.clone(),
                n_limb_0_col62.clone(),
                n_limb_1_col63.clone(),
                n_limb_2_col64.clone(),
            ],
        ));

        // Read Positive Num Bits 27.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (prev_instance_addr_tmp_c1b19_1.clone() + M31_6.clone()),
                n_prev_id_col65.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                n_prev_id_col65.clone(),
                n_prev_limb_0_col66.clone(),
                n_prev_limb_1_col67.clone(),
                n_prev_limb_2_col68.clone(),
            ],
        ));

        let block_reset_condition_tmp_c1b19_21 = eval.add_intermediate(
            ((((n_prev_limb_0_col66.clone() + (n_prev_limb_1_col67.clone() * M31_512.clone()))
                + (n_prev_limb_2_col68.clone() * M31_262144.clone()))
                - M31_1.clone())
                * (is_instance_0_col0.clone() - M31_1.clone())),
        );
        // Progression of n between instances..
        eval.add_constraint(
            (block_reset_condition_tmp_c1b19_21.clone()
                * ((((n_prev_limb_0_col66.clone()
                    + (n_prev_limb_1_col67.clone() * M31_512.clone()))
                    + (n_prev_limb_2_col68.clone() * M31_262144.clone()))
                    - M31_1.clone())
                    - ((n_limb_0_col62.clone() + (n_limb_1_col63.clone() * M31_512.clone()))
                        + (n_limb_2_col64.clone() * M31_262144.clone())))),
        );
        // Progression of offsets_ptr between instances..
        eval.add_constraint(
            (block_reset_condition_tmp_c1b19_21.clone()
                * ((((offsets_ptr_limb_0_col54.clone()
                    + (offsets_ptr_limb_1_col55.clone() * M31_512.clone()))
                    + (offsets_ptr_limb_2_col56.clone() * M31_262144.clone()))
                    - M31_3.clone())
                    - ((offsets_ptr_prev_limb_0_col58.clone()
                        + (offsets_ptr_prev_limb_1_col59.clone() * M31_512.clone()))
                        + (offsets_ptr_prev_limb_2_col60.clone() * M31_262144.clone())))),
        );

        // Mem Cond Verify Equal Known Id.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (prev_instance_addr_tmp_c1b19_1.clone() + M31_4.clone()),
                values_ptr_prev_id_col69.clone(),
            ],
        ));

        // The two ids are equal if the condition is met.
        eval.add_constraint(
            ((values_ptr_prev_id_col69.clone() - values_ptr_id_col49.clone())
                * block_reset_condition_tmp_c1b19_21.clone()),
        );

        // Mem Cond Verify Equal Known Id.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                prev_instance_addr_tmp_c1b19_1.clone(),
                p_prev0_id_col70.clone(),
            ],
        ));

        // The two ids are equal if the condition is met.
        eval.add_constraint(
            ((p_prev0_id_col70.clone() - p0_id_col1.clone())
                * block_reset_condition_tmp_c1b19_21.clone()),
        );

        // Mem Cond Verify Equal Known Id.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (prev_instance_addr_tmp_c1b19_1.clone() + M31_1.clone()),
                p_prev1_id_col71.clone(),
            ],
        ));

        // The two ids are equal if the condition is met.
        eval.add_constraint(
            ((p_prev1_id_col71.clone() - p1_id_col13.clone())
                * block_reset_condition_tmp_c1b19_21.clone()),
        );

        // Mem Cond Verify Equal Known Id.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (prev_instance_addr_tmp_c1b19_1.clone() + M31_2.clone()),
                p_prev2_id_col72.clone(),
            ],
        ));

        // The two ids are equal if the condition is met.
        eval.add_constraint(
            ((p_prev2_id_col72.clone() - p2_id_col25.clone())
                * block_reset_condition_tmp_c1b19_21.clone()),
        );

        // Mem Cond Verify Equal Known Id.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (prev_instance_addr_tmp_c1b19_1.clone() + M31_3.clone()),
                p_prev3_id_col73.clone(),
            ],
        ));

        // The two ids are equal if the condition is met.
        eval.add_constraint(
            ((p_prev3_id_col73.clone() - p3_id_col37.clone())
                * block_reset_condition_tmp_c1b19_21.clone()),
        );

        // Read Small.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                ((offsets_ptr_limb_0_col54.clone()
                    + (offsets_ptr_limb_1_col55.clone() * M31_512.clone()))
                    + (offsets_ptr_limb_2_col56.clone() * M31_262144.clone())),
                offsets_a_id_col74.clone(),
            ],
        ));

        // Cond Decode Small Sign.

        // msb is a bit.
        eval.add_constraint((msb_col75.clone() * (msb_col75.clone() - M31_1.clone())));
        // mid_limbs_set is a bit.
        eval.add_constraint(
            (mid_limbs_set_col76.clone() * (mid_limbs_set_col76.clone() - M31_1.clone())),
        );
        // Cannot have msb equals 0 and mid_limbs_set equals 1.
        eval.add_constraint((mid_limbs_set_col76.clone() * (msb_col75.clone() - M31_1.clone())));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                offsets_a_id_col74.clone(),
                offsets_a_limb_0_col77.clone(),
                offsets_a_limb_1_col78.clone(),
                offsets_a_limb_2_col79.clone(),
                (mid_limbs_set_col76.clone() * M31_511.clone()),
                (mid_limbs_set_col76.clone() * M31_511.clone()),
                (mid_limbs_set_col76.clone() * M31_511.clone()),
                (mid_limbs_set_col76.clone() * M31_511.clone()),
                (mid_limbs_set_col76.clone() * M31_511.clone()),
                (mid_limbs_set_col76.clone() * M31_511.clone()),
                (mid_limbs_set_col76.clone() * M31_511.clone()),
                (mid_limbs_set_col76.clone() * M31_511.clone()),
                (mid_limbs_set_col76.clone() * M31_511.clone()),
                (mid_limbs_set_col76.clone() * M31_511.clone()),
                (mid_limbs_set_col76.clone() * M31_511.clone()),
                (mid_limbs_set_col76.clone() * M31_511.clone()),
                (mid_limbs_set_col76.clone() * M31_511.clone()),
                (mid_limbs_set_col76.clone() * M31_511.clone()),
                (mid_limbs_set_col76.clone() * M31_511.clone()),
                (mid_limbs_set_col76.clone() * M31_511.clone()),
                (mid_limbs_set_col76.clone() * M31_511.clone()),
                (mid_limbs_set_col76.clone() * M31_511.clone()),
                ((M31_136.clone() * msb_col75.clone()) - mid_limbs_set_col76.clone()),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                (msb_col75.clone() * M31_256.clone()),
            ],
        ));

        // Read Small.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (((offsets_ptr_limb_0_col54.clone()
                    + (offsets_ptr_limb_1_col55.clone() * M31_512.clone()))
                    + (offsets_ptr_limb_2_col56.clone() * M31_262144.clone()))
                    + M31_1.clone()),
                offsets_b_id_col80.clone(),
            ],
        ));

        // Cond Decode Small Sign.

        // msb is a bit.
        eval.add_constraint((msb_col81.clone() * (msb_col81.clone() - M31_1.clone())));
        // mid_limbs_set is a bit.
        eval.add_constraint(
            (mid_limbs_set_col82.clone() * (mid_limbs_set_col82.clone() - M31_1.clone())),
        );
        // Cannot have msb equals 0 and mid_limbs_set equals 1.
        eval.add_constraint((mid_limbs_set_col82.clone() * (msb_col81.clone() - M31_1.clone())));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                offsets_b_id_col80.clone(),
                offsets_b_limb_0_col83.clone(),
                offsets_b_limb_1_col84.clone(),
                offsets_b_limb_2_col85.clone(),
                (mid_limbs_set_col82.clone() * M31_511.clone()),
                (mid_limbs_set_col82.clone() * M31_511.clone()),
                (mid_limbs_set_col82.clone() * M31_511.clone()),
                (mid_limbs_set_col82.clone() * M31_511.clone()),
                (mid_limbs_set_col82.clone() * M31_511.clone()),
                (mid_limbs_set_col82.clone() * M31_511.clone()),
                (mid_limbs_set_col82.clone() * M31_511.clone()),
                (mid_limbs_set_col82.clone() * M31_511.clone()),
                (mid_limbs_set_col82.clone() * M31_511.clone()),
                (mid_limbs_set_col82.clone() * M31_511.clone()),
                (mid_limbs_set_col82.clone() * M31_511.clone()),
                (mid_limbs_set_col82.clone() * M31_511.clone()),
                (mid_limbs_set_col82.clone() * M31_511.clone()),
                (mid_limbs_set_col82.clone() * M31_511.clone()),
                (mid_limbs_set_col82.clone() * M31_511.clone()),
                (mid_limbs_set_col82.clone() * M31_511.clone()),
                (mid_limbs_set_col82.clone() * M31_511.clone()),
                (mid_limbs_set_col82.clone() * M31_511.clone()),
                ((M31_136.clone() * msb_col81.clone()) - mid_limbs_set_col82.clone()),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                (msb_col81.clone() * M31_256.clone()),
            ],
        ));

        // Read Small.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (((offsets_ptr_limb_0_col54.clone()
                    + (offsets_ptr_limb_1_col55.clone() * M31_512.clone()))
                    + (offsets_ptr_limb_2_col56.clone() * M31_262144.clone()))
                    + M31_2.clone()),
                offsets_c_id_col86.clone(),
            ],
        ));

        // Cond Decode Small Sign.

        // msb is a bit.
        eval.add_constraint((msb_col87.clone() * (msb_col87.clone() - M31_1.clone())));
        // mid_limbs_set is a bit.
        eval.add_constraint(
            (mid_limbs_set_col88.clone() * (mid_limbs_set_col88.clone() - M31_1.clone())),
        );
        // Cannot have msb equals 0 and mid_limbs_set equals 1.
        eval.add_constraint((mid_limbs_set_col88.clone() * (msb_col87.clone() - M31_1.clone())));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                offsets_c_id_col86.clone(),
                offsets_c_limb_0_col89.clone(),
                offsets_c_limb_1_col90.clone(),
                offsets_c_limb_2_col91.clone(),
                (mid_limbs_set_col88.clone() * M31_511.clone()),
                (mid_limbs_set_col88.clone() * M31_511.clone()),
                (mid_limbs_set_col88.clone() * M31_511.clone()),
                (mid_limbs_set_col88.clone() * M31_511.clone()),
                (mid_limbs_set_col88.clone() * M31_511.clone()),
                (mid_limbs_set_col88.clone() * M31_511.clone()),
                (mid_limbs_set_col88.clone() * M31_511.clone()),
                (mid_limbs_set_col88.clone() * M31_511.clone()),
                (mid_limbs_set_col88.clone() * M31_511.clone()),
                (mid_limbs_set_col88.clone() * M31_511.clone()),
                (mid_limbs_set_col88.clone() * M31_511.clone()),
                (mid_limbs_set_col88.clone() * M31_511.clone()),
                (mid_limbs_set_col88.clone() * M31_511.clone()),
                (mid_limbs_set_col88.clone() * M31_511.clone()),
                (mid_limbs_set_col88.clone() * M31_511.clone()),
                (mid_limbs_set_col88.clone() * M31_511.clone()),
                (mid_limbs_set_col88.clone() * M31_511.clone()),
                (mid_limbs_set_col88.clone() * M31_511.clone()),
                ((M31_136.clone() * msb_col87.clone()) - mid_limbs_set_col88.clone()),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                (msb_col87.clone() * M31_256.clone()),
            ],
        ));

        let offsets_val_tmp_c1b19_39_limb_0 = eval.add_intermediate(
            ((((offsets_a_limb_0_col77.clone()
                + (offsets_a_limb_1_col78.clone() * M31_512.clone()))
                + (offsets_a_limb_2_col79.clone() * M31_262144.clone()))
                - msb_col75.clone())
                - (M31_134217728.clone() * mid_limbs_set_col76.clone())),
        );
        let offsets_val_tmp_c1b19_39_limb_1 = eval.add_intermediate(
            ((((offsets_b_limb_0_col83.clone()
                + (offsets_b_limb_1_col84.clone() * M31_512.clone()))
                + (offsets_b_limb_2_col85.clone() * M31_262144.clone()))
                - msb_col81.clone())
                - (M31_134217728.clone() * mid_limbs_set_col82.clone())),
        );
        let offsets_val_tmp_c1b19_39_limb_2 = eval.add_intermediate(
            ((((offsets_c_limb_0_col89.clone()
                + (offsets_c_limb_1_col90.clone() * M31_512.clone()))
                + (offsets_c_limb_2_col91.clone() * M31_262144.clone()))
                - msb_col87.clone())
                - (M31_134217728.clone() * mid_limbs_set_col88.clone())),
        );
        let values_ptr_tmp_c1b19_40 = eval.add_intermediate(
            ((values_ptr_limb_0_col50.clone()
                + (values_ptr_limb_1_col51.clone() * M31_512.clone()))
                + (values_ptr_limb_2_col52.clone() * M31_262144.clone())),
        );

        // Read Positive Num Bits 99.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (values_ptr_tmp_c1b19_40.clone() + offsets_val_tmp_c1b19_39_limb_0.clone()),
                a0_id_col92.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
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
            ],
        ));

        // Read Positive Num Bits 99.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                ((values_ptr_tmp_c1b19_40.clone() + offsets_val_tmp_c1b19_39_limb_0.clone())
                    + M31_1.clone()),
                a1_id_col104.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
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
            ],
        ));

        // Read Positive Num Bits 99.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                ((values_ptr_tmp_c1b19_40.clone() + offsets_val_tmp_c1b19_39_limb_0.clone())
                    + M31_2.clone()),
                a2_id_col116.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
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
            ],
        ));

        // Read Positive Num Bits 99.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                ((values_ptr_tmp_c1b19_40.clone() + offsets_val_tmp_c1b19_39_limb_0.clone())
                    + M31_3.clone()),
                a3_id_col128.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
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
            ],
        ));

        // Read Positive Num Bits 99.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (values_ptr_tmp_c1b19_40.clone() + offsets_val_tmp_c1b19_39_limb_1.clone()),
                b0_id_col140.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
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
            ],
        ));

        // Read Positive Num Bits 99.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                ((values_ptr_tmp_c1b19_40.clone() + offsets_val_tmp_c1b19_39_limb_1.clone())
                    + M31_1.clone()),
                b1_id_col152.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
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
            ],
        ));

        // Read Positive Num Bits 99.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                ((values_ptr_tmp_c1b19_40.clone() + offsets_val_tmp_c1b19_39_limb_1.clone())
                    + M31_2.clone()),
                b2_id_col164.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
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
            ],
        ));

        // Read Positive Num Bits 99.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                ((values_ptr_tmp_c1b19_40.clone() + offsets_val_tmp_c1b19_39_limb_1.clone())
                    + M31_3.clone()),
                b3_id_col176.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
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
            ],
        ));

        // Read Positive Num Bits 99.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (values_ptr_tmp_c1b19_40.clone() + offsets_val_tmp_c1b19_39_limb_2.clone()),
                c0_id_col188.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
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
            ],
        ));

        // Read Positive Num Bits 99.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                ((values_ptr_tmp_c1b19_40.clone() + offsets_val_tmp_c1b19_39_limb_2.clone())
                    + M31_1.clone()),
                c1_id_col200.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
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
            ],
        ));

        // Read Positive Num Bits 99.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                ((values_ptr_tmp_c1b19_40.clone() + offsets_val_tmp_c1b19_39_limb_2.clone())
                    + M31_2.clone()),
                c2_id_col212.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
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
            ],
        ));

        // Read Positive Num Bits 99.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                ((values_ptr_tmp_c1b19_40.clone() + offsets_val_tmp_c1b19_39_limb_2.clone())
                    + M31_3.clone()),
                c3_id_col224.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
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
            ],
        ));

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
