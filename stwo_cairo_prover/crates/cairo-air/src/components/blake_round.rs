use crate::components::prelude::*;

pub const N_TRACE_COLUMNS: usize = 212;

pub struct Eval {
    pub claim: Claim,
    pub blake_g_lookup_elements: relations::BlakeG,
    pub blake_round_lookup_elements: relations::BlakeRound,
    pub blake_round_sigma_lookup_elements: relations::BlakeRoundSigma,
    pub memory_address_to_id_lookup_elements: relations::MemoryAddressToId,
    pub memory_id_to_big_lookup_elements: relations::MemoryIdToBig,
    pub range_check_7_2_5_lookup_elements: relations::RangeCheck_7_2_5,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Claim {
    pub log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 30];
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
        let M31_1 = E::F::from(M31::from(1));
        let M31_128 = E::F::from(M31::from(128));
        let M31_4 = E::F::from(M31::from(4));
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
        let blake_round_sigma_output_limb_0_col35 = eval.next_trace_mask();
        let blake_round_sigma_output_limb_1_col36 = eval.next_trace_mask();
        let blake_round_sigma_output_limb_2_col37 = eval.next_trace_mask();
        let blake_round_sigma_output_limb_3_col38 = eval.next_trace_mask();
        let blake_round_sigma_output_limb_4_col39 = eval.next_trace_mask();
        let blake_round_sigma_output_limb_5_col40 = eval.next_trace_mask();
        let blake_round_sigma_output_limb_6_col41 = eval.next_trace_mask();
        let blake_round_sigma_output_limb_7_col42 = eval.next_trace_mask();
        let blake_round_sigma_output_limb_8_col43 = eval.next_trace_mask();
        let blake_round_sigma_output_limb_9_col44 = eval.next_trace_mask();
        let blake_round_sigma_output_limb_10_col45 = eval.next_trace_mask();
        let blake_round_sigma_output_limb_11_col46 = eval.next_trace_mask();
        let blake_round_sigma_output_limb_12_col47 = eval.next_trace_mask();
        let blake_round_sigma_output_limb_13_col48 = eval.next_trace_mask();
        let blake_round_sigma_output_limb_14_col49 = eval.next_trace_mask();
        let blake_round_sigma_output_limb_15_col50 = eval.next_trace_mask();
        let low_16_bits_col51 = eval.next_trace_mask();
        let high_16_bits_col52 = eval.next_trace_mask();
        let low_7_ms_bits_col53 = eval.next_trace_mask();
        let high_14_ms_bits_col54 = eval.next_trace_mask();
        let high_5_ms_bits_col55 = eval.next_trace_mask();
        let message_word_0_id_col56 = eval.next_trace_mask();
        let low_16_bits_col57 = eval.next_trace_mask();
        let high_16_bits_col58 = eval.next_trace_mask();
        let low_7_ms_bits_col59 = eval.next_trace_mask();
        let high_14_ms_bits_col60 = eval.next_trace_mask();
        let high_5_ms_bits_col61 = eval.next_trace_mask();
        let message_word_1_id_col62 = eval.next_trace_mask();
        let low_16_bits_col63 = eval.next_trace_mask();
        let high_16_bits_col64 = eval.next_trace_mask();
        let low_7_ms_bits_col65 = eval.next_trace_mask();
        let high_14_ms_bits_col66 = eval.next_trace_mask();
        let high_5_ms_bits_col67 = eval.next_trace_mask();
        let message_word_2_id_col68 = eval.next_trace_mask();
        let low_16_bits_col69 = eval.next_trace_mask();
        let high_16_bits_col70 = eval.next_trace_mask();
        let low_7_ms_bits_col71 = eval.next_trace_mask();
        let high_14_ms_bits_col72 = eval.next_trace_mask();
        let high_5_ms_bits_col73 = eval.next_trace_mask();
        let message_word_3_id_col74 = eval.next_trace_mask();
        let low_16_bits_col75 = eval.next_trace_mask();
        let high_16_bits_col76 = eval.next_trace_mask();
        let low_7_ms_bits_col77 = eval.next_trace_mask();
        let high_14_ms_bits_col78 = eval.next_trace_mask();
        let high_5_ms_bits_col79 = eval.next_trace_mask();
        let message_word_4_id_col80 = eval.next_trace_mask();
        let low_16_bits_col81 = eval.next_trace_mask();
        let high_16_bits_col82 = eval.next_trace_mask();
        let low_7_ms_bits_col83 = eval.next_trace_mask();
        let high_14_ms_bits_col84 = eval.next_trace_mask();
        let high_5_ms_bits_col85 = eval.next_trace_mask();
        let message_word_5_id_col86 = eval.next_trace_mask();
        let low_16_bits_col87 = eval.next_trace_mask();
        let high_16_bits_col88 = eval.next_trace_mask();
        let low_7_ms_bits_col89 = eval.next_trace_mask();
        let high_14_ms_bits_col90 = eval.next_trace_mask();
        let high_5_ms_bits_col91 = eval.next_trace_mask();
        let message_word_6_id_col92 = eval.next_trace_mask();
        let low_16_bits_col93 = eval.next_trace_mask();
        let high_16_bits_col94 = eval.next_trace_mask();
        let low_7_ms_bits_col95 = eval.next_trace_mask();
        let high_14_ms_bits_col96 = eval.next_trace_mask();
        let high_5_ms_bits_col97 = eval.next_trace_mask();
        let message_word_7_id_col98 = eval.next_trace_mask();
        let low_16_bits_col99 = eval.next_trace_mask();
        let high_16_bits_col100 = eval.next_trace_mask();
        let low_7_ms_bits_col101 = eval.next_trace_mask();
        let high_14_ms_bits_col102 = eval.next_trace_mask();
        let high_5_ms_bits_col103 = eval.next_trace_mask();
        let message_word_8_id_col104 = eval.next_trace_mask();
        let low_16_bits_col105 = eval.next_trace_mask();
        let high_16_bits_col106 = eval.next_trace_mask();
        let low_7_ms_bits_col107 = eval.next_trace_mask();
        let high_14_ms_bits_col108 = eval.next_trace_mask();
        let high_5_ms_bits_col109 = eval.next_trace_mask();
        let message_word_9_id_col110 = eval.next_trace_mask();
        let low_16_bits_col111 = eval.next_trace_mask();
        let high_16_bits_col112 = eval.next_trace_mask();
        let low_7_ms_bits_col113 = eval.next_trace_mask();
        let high_14_ms_bits_col114 = eval.next_trace_mask();
        let high_5_ms_bits_col115 = eval.next_trace_mask();
        let message_word_10_id_col116 = eval.next_trace_mask();
        let low_16_bits_col117 = eval.next_trace_mask();
        let high_16_bits_col118 = eval.next_trace_mask();
        let low_7_ms_bits_col119 = eval.next_trace_mask();
        let high_14_ms_bits_col120 = eval.next_trace_mask();
        let high_5_ms_bits_col121 = eval.next_trace_mask();
        let message_word_11_id_col122 = eval.next_trace_mask();
        let low_16_bits_col123 = eval.next_trace_mask();
        let high_16_bits_col124 = eval.next_trace_mask();
        let low_7_ms_bits_col125 = eval.next_trace_mask();
        let high_14_ms_bits_col126 = eval.next_trace_mask();
        let high_5_ms_bits_col127 = eval.next_trace_mask();
        let message_word_12_id_col128 = eval.next_trace_mask();
        let low_16_bits_col129 = eval.next_trace_mask();
        let high_16_bits_col130 = eval.next_trace_mask();
        let low_7_ms_bits_col131 = eval.next_trace_mask();
        let high_14_ms_bits_col132 = eval.next_trace_mask();
        let high_5_ms_bits_col133 = eval.next_trace_mask();
        let message_word_13_id_col134 = eval.next_trace_mask();
        let low_16_bits_col135 = eval.next_trace_mask();
        let high_16_bits_col136 = eval.next_trace_mask();
        let low_7_ms_bits_col137 = eval.next_trace_mask();
        let high_14_ms_bits_col138 = eval.next_trace_mask();
        let high_5_ms_bits_col139 = eval.next_trace_mask();
        let message_word_14_id_col140 = eval.next_trace_mask();
        let low_16_bits_col141 = eval.next_trace_mask();
        let high_16_bits_col142 = eval.next_trace_mask();
        let low_7_ms_bits_col143 = eval.next_trace_mask();
        let high_14_ms_bits_col144 = eval.next_trace_mask();
        let high_5_ms_bits_col145 = eval.next_trace_mask();
        let message_word_15_id_col146 = eval.next_trace_mask();
        let blake_g_output_limb_0_col147 = eval.next_trace_mask();
        let blake_g_output_limb_1_col148 = eval.next_trace_mask();
        let blake_g_output_limb_2_col149 = eval.next_trace_mask();
        let blake_g_output_limb_3_col150 = eval.next_trace_mask();
        let blake_g_output_limb_4_col151 = eval.next_trace_mask();
        let blake_g_output_limb_5_col152 = eval.next_trace_mask();
        let blake_g_output_limb_6_col153 = eval.next_trace_mask();
        let blake_g_output_limb_7_col154 = eval.next_trace_mask();
        let blake_g_output_limb_0_col155 = eval.next_trace_mask();
        let blake_g_output_limb_1_col156 = eval.next_trace_mask();
        let blake_g_output_limb_2_col157 = eval.next_trace_mask();
        let blake_g_output_limb_3_col158 = eval.next_trace_mask();
        let blake_g_output_limb_4_col159 = eval.next_trace_mask();
        let blake_g_output_limb_5_col160 = eval.next_trace_mask();
        let blake_g_output_limb_6_col161 = eval.next_trace_mask();
        let blake_g_output_limb_7_col162 = eval.next_trace_mask();
        let blake_g_output_limb_0_col163 = eval.next_trace_mask();
        let blake_g_output_limb_1_col164 = eval.next_trace_mask();
        let blake_g_output_limb_2_col165 = eval.next_trace_mask();
        let blake_g_output_limb_3_col166 = eval.next_trace_mask();
        let blake_g_output_limb_4_col167 = eval.next_trace_mask();
        let blake_g_output_limb_5_col168 = eval.next_trace_mask();
        let blake_g_output_limb_6_col169 = eval.next_trace_mask();
        let blake_g_output_limb_7_col170 = eval.next_trace_mask();
        let blake_g_output_limb_0_col171 = eval.next_trace_mask();
        let blake_g_output_limb_1_col172 = eval.next_trace_mask();
        let blake_g_output_limb_2_col173 = eval.next_trace_mask();
        let blake_g_output_limb_3_col174 = eval.next_trace_mask();
        let blake_g_output_limb_4_col175 = eval.next_trace_mask();
        let blake_g_output_limb_5_col176 = eval.next_trace_mask();
        let blake_g_output_limb_6_col177 = eval.next_trace_mask();
        let blake_g_output_limb_7_col178 = eval.next_trace_mask();
        let blake_g_output_limb_0_col179 = eval.next_trace_mask();
        let blake_g_output_limb_1_col180 = eval.next_trace_mask();
        let blake_g_output_limb_2_col181 = eval.next_trace_mask();
        let blake_g_output_limb_3_col182 = eval.next_trace_mask();
        let blake_g_output_limb_4_col183 = eval.next_trace_mask();
        let blake_g_output_limb_5_col184 = eval.next_trace_mask();
        let blake_g_output_limb_6_col185 = eval.next_trace_mask();
        let blake_g_output_limb_7_col186 = eval.next_trace_mask();
        let blake_g_output_limb_0_col187 = eval.next_trace_mask();
        let blake_g_output_limb_1_col188 = eval.next_trace_mask();
        let blake_g_output_limb_2_col189 = eval.next_trace_mask();
        let blake_g_output_limb_3_col190 = eval.next_trace_mask();
        let blake_g_output_limb_4_col191 = eval.next_trace_mask();
        let blake_g_output_limb_5_col192 = eval.next_trace_mask();
        let blake_g_output_limb_6_col193 = eval.next_trace_mask();
        let blake_g_output_limb_7_col194 = eval.next_trace_mask();
        let blake_g_output_limb_0_col195 = eval.next_trace_mask();
        let blake_g_output_limb_1_col196 = eval.next_trace_mask();
        let blake_g_output_limb_2_col197 = eval.next_trace_mask();
        let blake_g_output_limb_3_col198 = eval.next_trace_mask();
        let blake_g_output_limb_4_col199 = eval.next_trace_mask();
        let blake_g_output_limb_5_col200 = eval.next_trace_mask();
        let blake_g_output_limb_6_col201 = eval.next_trace_mask();
        let blake_g_output_limb_7_col202 = eval.next_trace_mask();
        let blake_g_output_limb_0_col203 = eval.next_trace_mask();
        let blake_g_output_limb_1_col204 = eval.next_trace_mask();
        let blake_g_output_limb_2_col205 = eval.next_trace_mask();
        let blake_g_output_limb_3_col206 = eval.next_trace_mask();
        let blake_g_output_limb_4_col207 = eval.next_trace_mask();
        let blake_g_output_limb_5_col208 = eval.next_trace_mask();
        let blake_g_output_limb_6_col209 = eval.next_trace_mask();
        let blake_g_output_limb_7_col210 = eval.next_trace_mask();
        let enabler_col_211 = eval.next_trace_mask();

        eval.add_to_relation(RelationEntry::new(
            &self.blake_round_sigma_lookup_elements,
            E::EF::one(),
            &[
                input_limb_1_col1.clone(),
                blake_round_sigma_output_limb_0_col35.clone(),
                blake_round_sigma_output_limb_1_col36.clone(),
                blake_round_sigma_output_limb_2_col37.clone(),
                blake_round_sigma_output_limb_3_col38.clone(),
                blake_round_sigma_output_limb_4_col39.clone(),
                blake_round_sigma_output_limb_5_col40.clone(),
                blake_round_sigma_output_limb_6_col41.clone(),
                blake_round_sigma_output_limb_7_col42.clone(),
                blake_round_sigma_output_limb_8_col43.clone(),
                blake_round_sigma_output_limb_9_col44.clone(),
                blake_round_sigma_output_limb_10_col45.clone(),
                blake_round_sigma_output_limb_11_col46.clone(),
                blake_round_sigma_output_limb_12_col47.clone(),
                blake_round_sigma_output_limb_13_col48.clone(),
                blake_round_sigma_output_limb_14_col49.clone(),
                blake_round_sigma_output_limb_15_col50.clone(),
            ],
        ));

        // Read Blake Word.

        // Verify Blake Word.

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_7_2_5_lookup_elements,
            E::EF::one(),
            &[
                low_7_ms_bits_col53.clone(),
                (high_16_bits_col52.clone() - (high_14_ms_bits_col54.clone() * M31_4.clone())),
                high_5_ms_bits_col55.clone(),
            ],
        ));

        // Mem Verify.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (input_limb_34_col34.clone() + blake_round_sigma_output_limb_0_col35.clone()),
                message_word_0_id_col56.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                message_word_0_id_col56.clone(),
                (low_16_bits_col51.clone() - (low_7_ms_bits_col53.clone() * M31_512.clone())),
                (low_7_ms_bits_col53.clone()
                    + ((high_16_bits_col52.clone()
                        - (high_14_ms_bits_col54.clone() * M31_4.clone()))
                        * M31_128.clone())),
                (high_14_ms_bits_col54.clone() - (high_5_ms_bits_col55.clone() * M31_512.clone())),
                high_5_ms_bits_col55.clone(),
            ],
        ));

        // Read Blake Word.

        // Verify Blake Word.

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_7_2_5_lookup_elements,
            E::EF::one(),
            &[
                low_7_ms_bits_col59.clone(),
                (high_16_bits_col58.clone() - (high_14_ms_bits_col60.clone() * M31_4.clone())),
                high_5_ms_bits_col61.clone(),
            ],
        ));

        // Mem Verify.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (input_limb_34_col34.clone() + blake_round_sigma_output_limb_1_col36.clone()),
                message_word_1_id_col62.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                message_word_1_id_col62.clone(),
                (low_16_bits_col57.clone() - (low_7_ms_bits_col59.clone() * M31_512.clone())),
                (low_7_ms_bits_col59.clone()
                    + ((high_16_bits_col58.clone()
                        - (high_14_ms_bits_col60.clone() * M31_4.clone()))
                        * M31_128.clone())),
                (high_14_ms_bits_col60.clone() - (high_5_ms_bits_col61.clone() * M31_512.clone())),
                high_5_ms_bits_col61.clone(),
            ],
        ));

        // Read Blake Word.

        // Verify Blake Word.

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_7_2_5_lookup_elements,
            E::EF::one(),
            &[
                low_7_ms_bits_col65.clone(),
                (high_16_bits_col64.clone() - (high_14_ms_bits_col66.clone() * M31_4.clone())),
                high_5_ms_bits_col67.clone(),
            ],
        ));

        // Mem Verify.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (input_limb_34_col34.clone() + blake_round_sigma_output_limb_2_col37.clone()),
                message_word_2_id_col68.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                message_word_2_id_col68.clone(),
                (low_16_bits_col63.clone() - (low_7_ms_bits_col65.clone() * M31_512.clone())),
                (low_7_ms_bits_col65.clone()
                    + ((high_16_bits_col64.clone()
                        - (high_14_ms_bits_col66.clone() * M31_4.clone()))
                        * M31_128.clone())),
                (high_14_ms_bits_col66.clone() - (high_5_ms_bits_col67.clone() * M31_512.clone())),
                high_5_ms_bits_col67.clone(),
            ],
        ));

        // Read Blake Word.

        // Verify Blake Word.

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_7_2_5_lookup_elements,
            E::EF::one(),
            &[
                low_7_ms_bits_col71.clone(),
                (high_16_bits_col70.clone() - (high_14_ms_bits_col72.clone() * M31_4.clone())),
                high_5_ms_bits_col73.clone(),
            ],
        ));

        // Mem Verify.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (input_limb_34_col34.clone() + blake_round_sigma_output_limb_3_col38.clone()),
                message_word_3_id_col74.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                message_word_3_id_col74.clone(),
                (low_16_bits_col69.clone() - (low_7_ms_bits_col71.clone() * M31_512.clone())),
                (low_7_ms_bits_col71.clone()
                    + ((high_16_bits_col70.clone()
                        - (high_14_ms_bits_col72.clone() * M31_4.clone()))
                        * M31_128.clone())),
                (high_14_ms_bits_col72.clone() - (high_5_ms_bits_col73.clone() * M31_512.clone())),
                high_5_ms_bits_col73.clone(),
            ],
        ));

        // Read Blake Word.

        // Verify Blake Word.

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_7_2_5_lookup_elements,
            E::EF::one(),
            &[
                low_7_ms_bits_col77.clone(),
                (high_16_bits_col76.clone() - (high_14_ms_bits_col78.clone() * M31_4.clone())),
                high_5_ms_bits_col79.clone(),
            ],
        ));

        // Mem Verify.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (input_limb_34_col34.clone() + blake_round_sigma_output_limb_4_col39.clone()),
                message_word_4_id_col80.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                message_word_4_id_col80.clone(),
                (low_16_bits_col75.clone() - (low_7_ms_bits_col77.clone() * M31_512.clone())),
                (low_7_ms_bits_col77.clone()
                    + ((high_16_bits_col76.clone()
                        - (high_14_ms_bits_col78.clone() * M31_4.clone()))
                        * M31_128.clone())),
                (high_14_ms_bits_col78.clone() - (high_5_ms_bits_col79.clone() * M31_512.clone())),
                high_5_ms_bits_col79.clone(),
            ],
        ));

        // Read Blake Word.

        // Verify Blake Word.

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_7_2_5_lookup_elements,
            E::EF::one(),
            &[
                low_7_ms_bits_col83.clone(),
                (high_16_bits_col82.clone() - (high_14_ms_bits_col84.clone() * M31_4.clone())),
                high_5_ms_bits_col85.clone(),
            ],
        ));

        // Mem Verify.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (input_limb_34_col34.clone() + blake_round_sigma_output_limb_5_col40.clone()),
                message_word_5_id_col86.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                message_word_5_id_col86.clone(),
                (low_16_bits_col81.clone() - (low_7_ms_bits_col83.clone() * M31_512.clone())),
                (low_7_ms_bits_col83.clone()
                    + ((high_16_bits_col82.clone()
                        - (high_14_ms_bits_col84.clone() * M31_4.clone()))
                        * M31_128.clone())),
                (high_14_ms_bits_col84.clone() - (high_5_ms_bits_col85.clone() * M31_512.clone())),
                high_5_ms_bits_col85.clone(),
            ],
        ));

        // Read Blake Word.

        // Verify Blake Word.

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_7_2_5_lookup_elements,
            E::EF::one(),
            &[
                low_7_ms_bits_col89.clone(),
                (high_16_bits_col88.clone() - (high_14_ms_bits_col90.clone() * M31_4.clone())),
                high_5_ms_bits_col91.clone(),
            ],
        ));

        // Mem Verify.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (input_limb_34_col34.clone() + blake_round_sigma_output_limb_6_col41.clone()),
                message_word_6_id_col92.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                message_word_6_id_col92.clone(),
                (low_16_bits_col87.clone() - (low_7_ms_bits_col89.clone() * M31_512.clone())),
                (low_7_ms_bits_col89.clone()
                    + ((high_16_bits_col88.clone()
                        - (high_14_ms_bits_col90.clone() * M31_4.clone()))
                        * M31_128.clone())),
                (high_14_ms_bits_col90.clone() - (high_5_ms_bits_col91.clone() * M31_512.clone())),
                high_5_ms_bits_col91.clone(),
            ],
        ));

        // Read Blake Word.

        // Verify Blake Word.

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_7_2_5_lookup_elements,
            E::EF::one(),
            &[
                low_7_ms_bits_col95.clone(),
                (high_16_bits_col94.clone() - (high_14_ms_bits_col96.clone() * M31_4.clone())),
                high_5_ms_bits_col97.clone(),
            ],
        ));

        // Mem Verify.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (input_limb_34_col34.clone() + blake_round_sigma_output_limb_7_col42.clone()),
                message_word_7_id_col98.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                message_word_7_id_col98.clone(),
                (low_16_bits_col93.clone() - (low_7_ms_bits_col95.clone() * M31_512.clone())),
                (low_7_ms_bits_col95.clone()
                    + ((high_16_bits_col94.clone()
                        - (high_14_ms_bits_col96.clone() * M31_4.clone()))
                        * M31_128.clone())),
                (high_14_ms_bits_col96.clone() - (high_5_ms_bits_col97.clone() * M31_512.clone())),
                high_5_ms_bits_col97.clone(),
            ],
        ));

        // Read Blake Word.

        // Verify Blake Word.

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_7_2_5_lookup_elements,
            E::EF::one(),
            &[
                low_7_ms_bits_col101.clone(),
                (high_16_bits_col100.clone() - (high_14_ms_bits_col102.clone() * M31_4.clone())),
                high_5_ms_bits_col103.clone(),
            ],
        ));

        // Mem Verify.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (input_limb_34_col34.clone() + blake_round_sigma_output_limb_8_col43.clone()),
                message_word_8_id_col104.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                message_word_8_id_col104.clone(),
                (low_16_bits_col99.clone() - (low_7_ms_bits_col101.clone() * M31_512.clone())),
                (low_7_ms_bits_col101.clone()
                    + ((high_16_bits_col100.clone()
                        - (high_14_ms_bits_col102.clone() * M31_4.clone()))
                        * M31_128.clone())),
                (high_14_ms_bits_col102.clone()
                    - (high_5_ms_bits_col103.clone() * M31_512.clone())),
                high_5_ms_bits_col103.clone(),
            ],
        ));

        // Read Blake Word.

        // Verify Blake Word.

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_7_2_5_lookup_elements,
            E::EF::one(),
            &[
                low_7_ms_bits_col107.clone(),
                (high_16_bits_col106.clone() - (high_14_ms_bits_col108.clone() * M31_4.clone())),
                high_5_ms_bits_col109.clone(),
            ],
        ));

        // Mem Verify.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (input_limb_34_col34.clone() + blake_round_sigma_output_limb_9_col44.clone()),
                message_word_9_id_col110.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                message_word_9_id_col110.clone(),
                (low_16_bits_col105.clone() - (low_7_ms_bits_col107.clone() * M31_512.clone())),
                (low_7_ms_bits_col107.clone()
                    + ((high_16_bits_col106.clone()
                        - (high_14_ms_bits_col108.clone() * M31_4.clone()))
                        * M31_128.clone())),
                (high_14_ms_bits_col108.clone()
                    - (high_5_ms_bits_col109.clone() * M31_512.clone())),
                high_5_ms_bits_col109.clone(),
            ],
        ));

        // Read Blake Word.

        // Verify Blake Word.

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_7_2_5_lookup_elements,
            E::EF::one(),
            &[
                low_7_ms_bits_col113.clone(),
                (high_16_bits_col112.clone() - (high_14_ms_bits_col114.clone() * M31_4.clone())),
                high_5_ms_bits_col115.clone(),
            ],
        ));

        // Mem Verify.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (input_limb_34_col34.clone() + blake_round_sigma_output_limb_10_col45.clone()),
                message_word_10_id_col116.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                message_word_10_id_col116.clone(),
                (low_16_bits_col111.clone() - (low_7_ms_bits_col113.clone() * M31_512.clone())),
                (low_7_ms_bits_col113.clone()
                    + ((high_16_bits_col112.clone()
                        - (high_14_ms_bits_col114.clone() * M31_4.clone()))
                        * M31_128.clone())),
                (high_14_ms_bits_col114.clone()
                    - (high_5_ms_bits_col115.clone() * M31_512.clone())),
                high_5_ms_bits_col115.clone(),
            ],
        ));

        // Read Blake Word.

        // Verify Blake Word.

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_7_2_5_lookup_elements,
            E::EF::one(),
            &[
                low_7_ms_bits_col119.clone(),
                (high_16_bits_col118.clone() - (high_14_ms_bits_col120.clone() * M31_4.clone())),
                high_5_ms_bits_col121.clone(),
            ],
        ));

        // Mem Verify.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (input_limb_34_col34.clone() + blake_round_sigma_output_limb_11_col46.clone()),
                message_word_11_id_col122.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                message_word_11_id_col122.clone(),
                (low_16_bits_col117.clone() - (low_7_ms_bits_col119.clone() * M31_512.clone())),
                (low_7_ms_bits_col119.clone()
                    + ((high_16_bits_col118.clone()
                        - (high_14_ms_bits_col120.clone() * M31_4.clone()))
                        * M31_128.clone())),
                (high_14_ms_bits_col120.clone()
                    - (high_5_ms_bits_col121.clone() * M31_512.clone())),
                high_5_ms_bits_col121.clone(),
            ],
        ));

        // Read Blake Word.

        // Verify Blake Word.

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_7_2_5_lookup_elements,
            E::EF::one(),
            &[
                low_7_ms_bits_col125.clone(),
                (high_16_bits_col124.clone() - (high_14_ms_bits_col126.clone() * M31_4.clone())),
                high_5_ms_bits_col127.clone(),
            ],
        ));

        // Mem Verify.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (input_limb_34_col34.clone() + blake_round_sigma_output_limb_12_col47.clone()),
                message_word_12_id_col128.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                message_word_12_id_col128.clone(),
                (low_16_bits_col123.clone() - (low_7_ms_bits_col125.clone() * M31_512.clone())),
                (low_7_ms_bits_col125.clone()
                    + ((high_16_bits_col124.clone()
                        - (high_14_ms_bits_col126.clone() * M31_4.clone()))
                        * M31_128.clone())),
                (high_14_ms_bits_col126.clone()
                    - (high_5_ms_bits_col127.clone() * M31_512.clone())),
                high_5_ms_bits_col127.clone(),
            ],
        ));

        // Read Blake Word.

        // Verify Blake Word.

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_7_2_5_lookup_elements,
            E::EF::one(),
            &[
                low_7_ms_bits_col131.clone(),
                (high_16_bits_col130.clone() - (high_14_ms_bits_col132.clone() * M31_4.clone())),
                high_5_ms_bits_col133.clone(),
            ],
        ));

        // Mem Verify.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (input_limb_34_col34.clone() + blake_round_sigma_output_limb_13_col48.clone()),
                message_word_13_id_col134.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                message_word_13_id_col134.clone(),
                (low_16_bits_col129.clone() - (low_7_ms_bits_col131.clone() * M31_512.clone())),
                (low_7_ms_bits_col131.clone()
                    + ((high_16_bits_col130.clone()
                        - (high_14_ms_bits_col132.clone() * M31_4.clone()))
                        * M31_128.clone())),
                (high_14_ms_bits_col132.clone()
                    - (high_5_ms_bits_col133.clone() * M31_512.clone())),
                high_5_ms_bits_col133.clone(),
            ],
        ));

        // Read Blake Word.

        // Verify Blake Word.

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_7_2_5_lookup_elements,
            E::EF::one(),
            &[
                low_7_ms_bits_col137.clone(),
                (high_16_bits_col136.clone() - (high_14_ms_bits_col138.clone() * M31_4.clone())),
                high_5_ms_bits_col139.clone(),
            ],
        ));

        // Mem Verify.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (input_limb_34_col34.clone() + blake_round_sigma_output_limb_14_col49.clone()),
                message_word_14_id_col140.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                message_word_14_id_col140.clone(),
                (low_16_bits_col135.clone() - (low_7_ms_bits_col137.clone() * M31_512.clone())),
                (low_7_ms_bits_col137.clone()
                    + ((high_16_bits_col136.clone()
                        - (high_14_ms_bits_col138.clone() * M31_4.clone()))
                        * M31_128.clone())),
                (high_14_ms_bits_col138.clone()
                    - (high_5_ms_bits_col139.clone() * M31_512.clone())),
                high_5_ms_bits_col139.clone(),
            ],
        ));

        // Read Blake Word.

        // Verify Blake Word.

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_7_2_5_lookup_elements,
            E::EF::one(),
            &[
                low_7_ms_bits_col143.clone(),
                (high_16_bits_col142.clone() - (high_14_ms_bits_col144.clone() * M31_4.clone())),
                high_5_ms_bits_col145.clone(),
            ],
        ));

        // Mem Verify.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (input_limb_34_col34.clone() + blake_round_sigma_output_limb_15_col50.clone()),
                message_word_15_id_col146.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                message_word_15_id_col146.clone(),
                (low_16_bits_col141.clone() - (low_7_ms_bits_col143.clone() * M31_512.clone())),
                (low_7_ms_bits_col143.clone()
                    + ((high_16_bits_col142.clone()
                        - (high_14_ms_bits_col144.clone() * M31_4.clone()))
                        * M31_128.clone())),
                (high_14_ms_bits_col144.clone()
                    - (high_5_ms_bits_col145.clone() * M31_512.clone())),
                high_5_ms_bits_col145.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.blake_g_lookup_elements,
            E::EF::one(),
            &[
                input_limb_2_col2.clone(),
                input_limb_3_col3.clone(),
                input_limb_10_col10.clone(),
                input_limb_11_col11.clone(),
                input_limb_18_col18.clone(),
                input_limb_19_col19.clone(),
                input_limb_26_col26.clone(),
                input_limb_27_col27.clone(),
                low_16_bits_col51.clone(),
                high_16_bits_col52.clone(),
                low_16_bits_col57.clone(),
                high_16_bits_col58.clone(),
                blake_g_output_limb_0_col147.clone(),
                blake_g_output_limb_1_col148.clone(),
                blake_g_output_limb_2_col149.clone(),
                blake_g_output_limb_3_col150.clone(),
                blake_g_output_limb_4_col151.clone(),
                blake_g_output_limb_5_col152.clone(),
                blake_g_output_limb_6_col153.clone(),
                blake_g_output_limb_7_col154.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.blake_g_lookup_elements,
            E::EF::one(),
            &[
                input_limb_4_col4.clone(),
                input_limb_5_col5.clone(),
                input_limb_12_col12.clone(),
                input_limb_13_col13.clone(),
                input_limb_20_col20.clone(),
                input_limb_21_col21.clone(),
                input_limb_28_col28.clone(),
                input_limb_29_col29.clone(),
                low_16_bits_col63.clone(),
                high_16_bits_col64.clone(),
                low_16_bits_col69.clone(),
                high_16_bits_col70.clone(),
                blake_g_output_limb_0_col155.clone(),
                blake_g_output_limb_1_col156.clone(),
                blake_g_output_limb_2_col157.clone(),
                blake_g_output_limb_3_col158.clone(),
                blake_g_output_limb_4_col159.clone(),
                blake_g_output_limb_5_col160.clone(),
                blake_g_output_limb_6_col161.clone(),
                blake_g_output_limb_7_col162.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.blake_g_lookup_elements,
            E::EF::one(),
            &[
                input_limb_6_col6.clone(),
                input_limb_7_col7.clone(),
                input_limb_14_col14.clone(),
                input_limb_15_col15.clone(),
                input_limb_22_col22.clone(),
                input_limb_23_col23.clone(),
                input_limb_30_col30.clone(),
                input_limb_31_col31.clone(),
                low_16_bits_col75.clone(),
                high_16_bits_col76.clone(),
                low_16_bits_col81.clone(),
                high_16_bits_col82.clone(),
                blake_g_output_limb_0_col163.clone(),
                blake_g_output_limb_1_col164.clone(),
                blake_g_output_limb_2_col165.clone(),
                blake_g_output_limb_3_col166.clone(),
                blake_g_output_limb_4_col167.clone(),
                blake_g_output_limb_5_col168.clone(),
                blake_g_output_limb_6_col169.clone(),
                blake_g_output_limb_7_col170.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.blake_g_lookup_elements,
            E::EF::one(),
            &[
                input_limb_8_col8.clone(),
                input_limb_9_col9.clone(),
                input_limb_16_col16.clone(),
                input_limb_17_col17.clone(),
                input_limb_24_col24.clone(),
                input_limb_25_col25.clone(),
                input_limb_32_col32.clone(),
                input_limb_33_col33.clone(),
                low_16_bits_col87.clone(),
                high_16_bits_col88.clone(),
                low_16_bits_col93.clone(),
                high_16_bits_col94.clone(),
                blake_g_output_limb_0_col171.clone(),
                blake_g_output_limb_1_col172.clone(),
                blake_g_output_limb_2_col173.clone(),
                blake_g_output_limb_3_col174.clone(),
                blake_g_output_limb_4_col175.clone(),
                blake_g_output_limb_5_col176.clone(),
                blake_g_output_limb_6_col177.clone(),
                blake_g_output_limb_7_col178.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.blake_g_lookup_elements,
            E::EF::one(),
            &[
                blake_g_output_limb_0_col147.clone(),
                blake_g_output_limb_1_col148.clone(),
                blake_g_output_limb_2_col157.clone(),
                blake_g_output_limb_3_col158.clone(),
                blake_g_output_limb_4_col167.clone(),
                blake_g_output_limb_5_col168.clone(),
                blake_g_output_limb_6_col177.clone(),
                blake_g_output_limb_7_col178.clone(),
                low_16_bits_col99.clone(),
                high_16_bits_col100.clone(),
                low_16_bits_col105.clone(),
                high_16_bits_col106.clone(),
                blake_g_output_limb_0_col179.clone(),
                blake_g_output_limb_1_col180.clone(),
                blake_g_output_limb_2_col181.clone(),
                blake_g_output_limb_3_col182.clone(),
                blake_g_output_limb_4_col183.clone(),
                blake_g_output_limb_5_col184.clone(),
                blake_g_output_limb_6_col185.clone(),
                blake_g_output_limb_7_col186.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.blake_g_lookup_elements,
            E::EF::one(),
            &[
                blake_g_output_limb_0_col155.clone(),
                blake_g_output_limb_1_col156.clone(),
                blake_g_output_limb_2_col165.clone(),
                blake_g_output_limb_3_col166.clone(),
                blake_g_output_limb_4_col175.clone(),
                blake_g_output_limb_5_col176.clone(),
                blake_g_output_limb_6_col153.clone(),
                blake_g_output_limb_7_col154.clone(),
                low_16_bits_col111.clone(),
                high_16_bits_col112.clone(),
                low_16_bits_col117.clone(),
                high_16_bits_col118.clone(),
                blake_g_output_limb_0_col187.clone(),
                blake_g_output_limb_1_col188.clone(),
                blake_g_output_limb_2_col189.clone(),
                blake_g_output_limb_3_col190.clone(),
                blake_g_output_limb_4_col191.clone(),
                blake_g_output_limb_5_col192.clone(),
                blake_g_output_limb_6_col193.clone(),
                blake_g_output_limb_7_col194.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.blake_g_lookup_elements,
            E::EF::one(),
            &[
                blake_g_output_limb_0_col163.clone(),
                blake_g_output_limb_1_col164.clone(),
                blake_g_output_limb_2_col173.clone(),
                blake_g_output_limb_3_col174.clone(),
                blake_g_output_limb_4_col151.clone(),
                blake_g_output_limb_5_col152.clone(),
                blake_g_output_limb_6_col161.clone(),
                blake_g_output_limb_7_col162.clone(),
                low_16_bits_col123.clone(),
                high_16_bits_col124.clone(),
                low_16_bits_col129.clone(),
                high_16_bits_col130.clone(),
                blake_g_output_limb_0_col195.clone(),
                blake_g_output_limb_1_col196.clone(),
                blake_g_output_limb_2_col197.clone(),
                blake_g_output_limb_3_col198.clone(),
                blake_g_output_limb_4_col199.clone(),
                blake_g_output_limb_5_col200.clone(),
                blake_g_output_limb_6_col201.clone(),
                blake_g_output_limb_7_col202.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.blake_g_lookup_elements,
            E::EF::one(),
            &[
                blake_g_output_limb_0_col171.clone(),
                blake_g_output_limb_1_col172.clone(),
                blake_g_output_limb_2_col149.clone(),
                blake_g_output_limb_3_col150.clone(),
                blake_g_output_limb_4_col159.clone(),
                blake_g_output_limb_5_col160.clone(),
                blake_g_output_limb_6_col169.clone(),
                blake_g_output_limb_7_col170.clone(),
                low_16_bits_col135.clone(),
                high_16_bits_col136.clone(),
                low_16_bits_col141.clone(),
                high_16_bits_col142.clone(),
                blake_g_output_limb_0_col203.clone(),
                blake_g_output_limb_1_col204.clone(),
                blake_g_output_limb_2_col205.clone(),
                blake_g_output_limb_3_col206.clone(),
                blake_g_output_limb_4_col207.clone(),
                blake_g_output_limb_5_col208.clone(),
                blake_g_output_limb_6_col209.clone(),
                blake_g_output_limb_7_col210.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.blake_round_lookup_elements,
            E::EF::from(enabler_col_211.clone()),
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
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.blake_round_lookup_elements,
            E::EF::from(-enabler_col_211.clone()),
            &[
                input_limb_0_col0.clone(),
                (input_limb_1_col1.clone() + M31_1.clone()),
                blake_g_output_limb_0_col179.clone(),
                blake_g_output_limb_1_col180.clone(),
                blake_g_output_limb_0_col187.clone(),
                blake_g_output_limb_1_col188.clone(),
                blake_g_output_limb_0_col195.clone(),
                blake_g_output_limb_1_col196.clone(),
                blake_g_output_limb_0_col203.clone(),
                blake_g_output_limb_1_col204.clone(),
                blake_g_output_limb_2_col205.clone(),
                blake_g_output_limb_3_col206.clone(),
                blake_g_output_limb_2_col181.clone(),
                blake_g_output_limb_3_col182.clone(),
                blake_g_output_limb_2_col189.clone(),
                blake_g_output_limb_3_col190.clone(),
                blake_g_output_limb_2_col197.clone(),
                blake_g_output_limb_3_col198.clone(),
                blake_g_output_limb_4_col199.clone(),
                blake_g_output_limb_5_col200.clone(),
                blake_g_output_limb_4_col207.clone(),
                blake_g_output_limb_5_col208.clone(),
                blake_g_output_limb_4_col183.clone(),
                blake_g_output_limb_5_col184.clone(),
                blake_g_output_limb_4_col191.clone(),
                blake_g_output_limb_5_col192.clone(),
                blake_g_output_limb_6_col193.clone(),
                blake_g_output_limb_7_col194.clone(),
                blake_g_output_limb_6_col201.clone(),
                blake_g_output_limb_7_col202.clone(),
                blake_g_output_limb_6_col209.clone(),
                blake_g_output_limb_7_col210.clone(),
                blake_g_output_limb_6_col185.clone(),
                blake_g_output_limb_7_col186.clone(),
                input_limb_34_col34.clone(),
            ],
        ));

        eval.finalize_logup_in_pairs();
        eval
    }
}
