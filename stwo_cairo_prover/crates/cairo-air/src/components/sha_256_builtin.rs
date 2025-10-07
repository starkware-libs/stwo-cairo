// AIR version 52ac7695-dirty
use crate::components::prelude::*;
use crate::components::subroutines::read_blake_word::ReadBlakeWord;
use crate::components::subroutines::verify_blake_word::VerifyBlakeWord;

pub const N_TRACE_COLUMNS: usize = 176;
pub const RELATION_USES_PER_ROW: [RelationUse; 4] = [
    RelationUse {
        relation_id: "MemoryAddressToId",
        uses: 24,
    },
    RelationUse {
        relation_id: "MemoryIdToBig",
        uses: 24,
    },
    RelationUse {
        relation_id: "RangeCheck_7_2_5",
        uses: 24,
    },
    RelationUse {
        relation_id: "Sha256Round",
        uses: 1,
    },
];

pub struct Eval {
    pub claim: Claim,
    pub range_check_7_2_5_lookup_elements: relations::RangeCheck_7_2_5,
    pub memory_address_to_id_lookup_elements: relations::MemoryAddressToId,
    pub memory_id_to_big_lookup_elements: relations::MemoryIdToBig,
    pub sha_256_round_lookup_elements: relations::Sha256Round,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct Claim {
    pub log_size: u32,
    pub sha256_builtin_segment_start: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 37];
        TreeVec::new(vec![vec![], trace_log_sizes, interaction_log_sizes])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(self.log_size as u64);
        channel.mix_u64(self.sha256_builtin_segment_start as u64);
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
        let M31_10 = E::F::from(M31::from(10));
        let M31_11 = E::F::from(M31::from(11));
        let M31_12 = E::F::from(M31::from(12));
        let M31_13 = E::F::from(M31::from(13));
        let M31_14 = E::F::from(M31::from(14));
        let M31_15 = E::F::from(M31::from(15));
        let M31_15470 = E::F::from(M31::from(15470));
        let M31_16 = E::F::from(M31::from(16));
        let M31_17 = E::F::from(M31::from(17));
        let M31_18 = E::F::from(M31::from(18));
        let M31_19 = E::F::from(M31::from(19));
        let M31_2 = E::F::from(M31::from(2));
        let M31_20 = E::F::from(M31::from(20));
        let M31_20750 = E::F::from(M31::from(20750));
        let M31_21 = E::F::from(M31::from(21));
        let M31_21119 = E::F::from(M31::from(21119));
        let M31_22 = E::F::from(M31::from(22));
        let M31_23 = E::F::from(M31::from(23));
        let M31_23520 = E::F::from(M31::from(23520));
        let M31_24 = E::F::from(M31::from(24));
        let M31_26764 = E::F::from(M31::from(26764));
        let M31_27145 = E::F::from(M31::from(27145));
        let M31_3 = E::F::from(M31::from(3));
        let M31_39685 = E::F::from(M31::from(39685));
        let M31_4 = E::F::from(M31::from(4));
        let M31_42319 = E::F::from(M31::from(42319));
        let M31_44677 = E::F::from(M31::from(44677));
        let M31_47975 = E::F::from(M31::from(47975));
        let M31_5 = E::F::from(M31::from(5));
        let M31_52505 = E::F::from(M31::from(52505));
        let M31_55723 = E::F::from(M31::from(55723));
        let M31_58983 = E::F::from(M31::from(58983));
        let M31_6 = E::F::from(M31::from(6));
        let M31_62322 = E::F::from(M31::from(62322));
        let M31_62778 = E::F::from(M31::from(62778));
        let M31_64 = E::F::from(M31::from(64));
        let M31_7 = E::F::from(M31::from(7));
        let M31_8 = E::F::from(M31::from(8));
        let M31_8067 = E::F::from(M31::from(8067));
        let M31_9 = E::F::from(M31::from(9));
        let seq = eval.get_preprocessed_column(Seq::new(self.log_size()).id());
        let low_16_bits_col0 = eval.next_trace_mask();
        let high_16_bits_col1 = eval.next_trace_mask();
        let low_7_ms_bits_col2 = eval.next_trace_mask();
        let high_14_ms_bits_col3 = eval.next_trace_mask();
        let high_5_ms_bits_col4 = eval.next_trace_mask();
        let input_0_id_col5 = eval.next_trace_mask();
        let low_16_bits_col6 = eval.next_trace_mask();
        let high_16_bits_col7 = eval.next_trace_mask();
        let low_7_ms_bits_col8 = eval.next_trace_mask();
        let high_14_ms_bits_col9 = eval.next_trace_mask();
        let high_5_ms_bits_col10 = eval.next_trace_mask();
        let input_1_id_col11 = eval.next_trace_mask();
        let low_16_bits_col12 = eval.next_trace_mask();
        let high_16_bits_col13 = eval.next_trace_mask();
        let low_7_ms_bits_col14 = eval.next_trace_mask();
        let high_14_ms_bits_col15 = eval.next_trace_mask();
        let high_5_ms_bits_col16 = eval.next_trace_mask();
        let input_2_id_col17 = eval.next_trace_mask();
        let low_16_bits_col18 = eval.next_trace_mask();
        let high_16_bits_col19 = eval.next_trace_mask();
        let low_7_ms_bits_col20 = eval.next_trace_mask();
        let high_14_ms_bits_col21 = eval.next_trace_mask();
        let high_5_ms_bits_col22 = eval.next_trace_mask();
        let input_3_id_col23 = eval.next_trace_mask();
        let low_16_bits_col24 = eval.next_trace_mask();
        let high_16_bits_col25 = eval.next_trace_mask();
        let low_7_ms_bits_col26 = eval.next_trace_mask();
        let high_14_ms_bits_col27 = eval.next_trace_mask();
        let high_5_ms_bits_col28 = eval.next_trace_mask();
        let input_4_id_col29 = eval.next_trace_mask();
        let low_16_bits_col30 = eval.next_trace_mask();
        let high_16_bits_col31 = eval.next_trace_mask();
        let low_7_ms_bits_col32 = eval.next_trace_mask();
        let high_14_ms_bits_col33 = eval.next_trace_mask();
        let high_5_ms_bits_col34 = eval.next_trace_mask();
        let input_5_id_col35 = eval.next_trace_mask();
        let low_16_bits_col36 = eval.next_trace_mask();
        let high_16_bits_col37 = eval.next_trace_mask();
        let low_7_ms_bits_col38 = eval.next_trace_mask();
        let high_14_ms_bits_col39 = eval.next_trace_mask();
        let high_5_ms_bits_col40 = eval.next_trace_mask();
        let input_6_id_col41 = eval.next_trace_mask();
        let low_16_bits_col42 = eval.next_trace_mask();
        let high_16_bits_col43 = eval.next_trace_mask();
        let low_7_ms_bits_col44 = eval.next_trace_mask();
        let high_14_ms_bits_col45 = eval.next_trace_mask();
        let high_5_ms_bits_col46 = eval.next_trace_mask();
        let input_7_id_col47 = eval.next_trace_mask();
        let low_16_bits_col48 = eval.next_trace_mask();
        let high_16_bits_col49 = eval.next_trace_mask();
        let low_7_ms_bits_col50 = eval.next_trace_mask();
        let high_14_ms_bits_col51 = eval.next_trace_mask();
        let high_5_ms_bits_col52 = eval.next_trace_mask();
        let input_8_id_col53 = eval.next_trace_mask();
        let low_16_bits_col54 = eval.next_trace_mask();
        let high_16_bits_col55 = eval.next_trace_mask();
        let low_7_ms_bits_col56 = eval.next_trace_mask();
        let high_14_ms_bits_col57 = eval.next_trace_mask();
        let high_5_ms_bits_col58 = eval.next_trace_mask();
        let input_9_id_col59 = eval.next_trace_mask();
        let low_16_bits_col60 = eval.next_trace_mask();
        let high_16_bits_col61 = eval.next_trace_mask();
        let low_7_ms_bits_col62 = eval.next_trace_mask();
        let high_14_ms_bits_col63 = eval.next_trace_mask();
        let high_5_ms_bits_col64 = eval.next_trace_mask();
        let input_10_id_col65 = eval.next_trace_mask();
        let low_16_bits_col66 = eval.next_trace_mask();
        let high_16_bits_col67 = eval.next_trace_mask();
        let low_7_ms_bits_col68 = eval.next_trace_mask();
        let high_14_ms_bits_col69 = eval.next_trace_mask();
        let high_5_ms_bits_col70 = eval.next_trace_mask();
        let input_11_id_col71 = eval.next_trace_mask();
        let low_16_bits_col72 = eval.next_trace_mask();
        let high_16_bits_col73 = eval.next_trace_mask();
        let low_7_ms_bits_col74 = eval.next_trace_mask();
        let high_14_ms_bits_col75 = eval.next_trace_mask();
        let high_5_ms_bits_col76 = eval.next_trace_mask();
        let input_12_id_col77 = eval.next_trace_mask();
        let low_16_bits_col78 = eval.next_trace_mask();
        let high_16_bits_col79 = eval.next_trace_mask();
        let low_7_ms_bits_col80 = eval.next_trace_mask();
        let high_14_ms_bits_col81 = eval.next_trace_mask();
        let high_5_ms_bits_col82 = eval.next_trace_mask();
        let input_13_id_col83 = eval.next_trace_mask();
        let low_16_bits_col84 = eval.next_trace_mask();
        let high_16_bits_col85 = eval.next_trace_mask();
        let low_7_ms_bits_col86 = eval.next_trace_mask();
        let high_14_ms_bits_col87 = eval.next_trace_mask();
        let high_5_ms_bits_col88 = eval.next_trace_mask();
        let input_14_id_col89 = eval.next_trace_mask();
        let low_16_bits_col90 = eval.next_trace_mask();
        let high_16_bits_col91 = eval.next_trace_mask();
        let low_7_ms_bits_col92 = eval.next_trace_mask();
        let high_14_ms_bits_col93 = eval.next_trace_mask();
        let high_5_ms_bits_col94 = eval.next_trace_mask();
        let input_15_id_col95 = eval.next_trace_mask();
        let sha_256_round_output_limb_0_col96 = eval.next_trace_mask();
        let sha_256_round_output_limb_1_col97 = eval.next_trace_mask();
        let sha_256_round_output_limb_2_col98 = eval.next_trace_mask();
        let sha_256_round_output_limb_3_col99 = eval.next_trace_mask();
        let sha_256_round_output_limb_4_col100 = eval.next_trace_mask();
        let sha_256_round_output_limb_5_col101 = eval.next_trace_mask();
        let sha_256_round_output_limb_6_col102 = eval.next_trace_mask();
        let sha_256_round_output_limb_7_col103 = eval.next_trace_mask();
        let sha_256_round_output_limb_8_col104 = eval.next_trace_mask();
        let sha_256_round_output_limb_9_col105 = eval.next_trace_mask();
        let sha_256_round_output_limb_10_col106 = eval.next_trace_mask();
        let sha_256_round_output_limb_11_col107 = eval.next_trace_mask();
        let sha_256_round_output_limb_12_col108 = eval.next_trace_mask();
        let sha_256_round_output_limb_13_col109 = eval.next_trace_mask();
        let sha_256_round_output_limb_14_col110 = eval.next_trace_mask();
        let sha_256_round_output_limb_15_col111 = eval.next_trace_mask();
        let sha_256_round_output_limb_16_col112 = eval.next_trace_mask();
        let sha_256_round_output_limb_17_col113 = eval.next_trace_mask();
        let sha_256_round_output_limb_18_col114 = eval.next_trace_mask();
        let sha_256_round_output_limb_19_col115 = eval.next_trace_mask();
        let sha_256_round_output_limb_20_col116 = eval.next_trace_mask();
        let sha_256_round_output_limb_21_col117 = eval.next_trace_mask();
        let sha_256_round_output_limb_22_col118 = eval.next_trace_mask();
        let sha_256_round_output_limb_23_col119 = eval.next_trace_mask();
        let sha_256_round_output_limb_24_col120 = eval.next_trace_mask();
        let sha_256_round_output_limb_25_col121 = eval.next_trace_mask();
        let sha_256_round_output_limb_26_col122 = eval.next_trace_mask();
        let sha_256_round_output_limb_27_col123 = eval.next_trace_mask();
        let sha_256_round_output_limb_28_col124 = eval.next_trace_mask();
        let sha_256_round_output_limb_29_col125 = eval.next_trace_mask();
        let sha_256_round_output_limb_30_col126 = eval.next_trace_mask();
        let sha_256_round_output_limb_31_col127 = eval.next_trace_mask();
        let sha_256_round_output_limb_32_col128 = eval.next_trace_mask();
        let sha_256_round_output_limb_33_col129 = eval.next_trace_mask();
        let sha_256_round_output_limb_34_col130 = eval.next_trace_mask();
        let sha_256_round_output_limb_35_col131 = eval.next_trace_mask();
        let sha_256_round_output_limb_36_col132 = eval.next_trace_mask();
        let sha_256_round_output_limb_37_col133 = eval.next_trace_mask();
        let sha_256_round_output_limb_38_col134 = eval.next_trace_mask();
        let sha_256_round_output_limb_39_col135 = eval.next_trace_mask();
        let sha_256_round_output_limb_40_col136 = eval.next_trace_mask();
        let sha_256_round_output_limb_41_col137 = eval.next_trace_mask();
        let sha_256_round_output_limb_42_col138 = eval.next_trace_mask();
        let sha_256_round_output_limb_43_col139 = eval.next_trace_mask();
        let sha_256_round_output_limb_44_col140 = eval.next_trace_mask();
        let sha_256_round_output_limb_45_col141 = eval.next_trace_mask();
        let sha_256_round_output_limb_46_col142 = eval.next_trace_mask();
        let sha_256_round_output_limb_47_col143 = eval.next_trace_mask();
        let low_7_ms_bits_col144 = eval.next_trace_mask();
        let high_14_ms_bits_col145 = eval.next_trace_mask();
        let high_5_ms_bits_col146 = eval.next_trace_mask();
        let output_0_id_col147 = eval.next_trace_mask();
        let low_7_ms_bits_col148 = eval.next_trace_mask();
        let high_14_ms_bits_col149 = eval.next_trace_mask();
        let high_5_ms_bits_col150 = eval.next_trace_mask();
        let output_1_id_col151 = eval.next_trace_mask();
        let low_7_ms_bits_col152 = eval.next_trace_mask();
        let high_14_ms_bits_col153 = eval.next_trace_mask();
        let high_5_ms_bits_col154 = eval.next_trace_mask();
        let output_2_id_col155 = eval.next_trace_mask();
        let low_7_ms_bits_col156 = eval.next_trace_mask();
        let high_14_ms_bits_col157 = eval.next_trace_mask();
        let high_5_ms_bits_col158 = eval.next_trace_mask();
        let output_3_id_col159 = eval.next_trace_mask();
        let low_7_ms_bits_col160 = eval.next_trace_mask();
        let high_14_ms_bits_col161 = eval.next_trace_mask();
        let high_5_ms_bits_col162 = eval.next_trace_mask();
        let output_4_id_col163 = eval.next_trace_mask();
        let low_7_ms_bits_col164 = eval.next_trace_mask();
        let high_14_ms_bits_col165 = eval.next_trace_mask();
        let high_5_ms_bits_col166 = eval.next_trace_mask();
        let output_5_id_col167 = eval.next_trace_mask();
        let low_7_ms_bits_col168 = eval.next_trace_mask();
        let high_14_ms_bits_col169 = eval.next_trace_mask();
        let high_5_ms_bits_col170 = eval.next_trace_mask();
        let output_6_id_col171 = eval.next_trace_mask();
        let low_7_ms_bits_col172 = eval.next_trace_mask();
        let high_14_ms_bits_col173 = eval.next_trace_mask();
        let high_5_ms_bits_col174 = eval.next_trace_mask();
        let output_7_id_col175 = eval.next_trace_mask();

        ReadBlakeWord::evaluate(
            [
                (E::F::from(M31::from(self.claim.sha256_builtin_segment_start))
                    + (seq.clone() * M31_24.clone())),
            ],
            low_16_bits_col0.clone(),
            high_16_bits_col1.clone(),
            low_7_ms_bits_col2.clone(),
            high_14_ms_bits_col3.clone(),
            high_5_ms_bits_col4.clone(),
            input_0_id_col5.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        ReadBlakeWord::evaluate(
            [
                ((E::F::from(M31::from(self.claim.sha256_builtin_segment_start))
                    + (seq.clone() * M31_24.clone()))
                    + M31_1.clone()),
            ],
            low_16_bits_col6.clone(),
            high_16_bits_col7.clone(),
            low_7_ms_bits_col8.clone(),
            high_14_ms_bits_col9.clone(),
            high_5_ms_bits_col10.clone(),
            input_1_id_col11.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        ReadBlakeWord::evaluate(
            [
                ((E::F::from(M31::from(self.claim.sha256_builtin_segment_start))
                    + (seq.clone() * M31_24.clone()))
                    + M31_2.clone()),
            ],
            low_16_bits_col12.clone(),
            high_16_bits_col13.clone(),
            low_7_ms_bits_col14.clone(),
            high_14_ms_bits_col15.clone(),
            high_5_ms_bits_col16.clone(),
            input_2_id_col17.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        ReadBlakeWord::evaluate(
            [
                ((E::F::from(M31::from(self.claim.sha256_builtin_segment_start))
                    + (seq.clone() * M31_24.clone()))
                    + M31_3.clone()),
            ],
            low_16_bits_col18.clone(),
            high_16_bits_col19.clone(),
            low_7_ms_bits_col20.clone(),
            high_14_ms_bits_col21.clone(),
            high_5_ms_bits_col22.clone(),
            input_3_id_col23.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        ReadBlakeWord::evaluate(
            [
                ((E::F::from(M31::from(self.claim.sha256_builtin_segment_start))
                    + (seq.clone() * M31_24.clone()))
                    + M31_4.clone()),
            ],
            low_16_bits_col24.clone(),
            high_16_bits_col25.clone(),
            low_7_ms_bits_col26.clone(),
            high_14_ms_bits_col27.clone(),
            high_5_ms_bits_col28.clone(),
            input_4_id_col29.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        ReadBlakeWord::evaluate(
            [
                ((E::F::from(M31::from(self.claim.sha256_builtin_segment_start))
                    + (seq.clone() * M31_24.clone()))
                    + M31_5.clone()),
            ],
            low_16_bits_col30.clone(),
            high_16_bits_col31.clone(),
            low_7_ms_bits_col32.clone(),
            high_14_ms_bits_col33.clone(),
            high_5_ms_bits_col34.clone(),
            input_5_id_col35.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        ReadBlakeWord::evaluate(
            [
                ((E::F::from(M31::from(self.claim.sha256_builtin_segment_start))
                    + (seq.clone() * M31_24.clone()))
                    + M31_6.clone()),
            ],
            low_16_bits_col36.clone(),
            high_16_bits_col37.clone(),
            low_7_ms_bits_col38.clone(),
            high_14_ms_bits_col39.clone(),
            high_5_ms_bits_col40.clone(),
            input_6_id_col41.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        ReadBlakeWord::evaluate(
            [
                ((E::F::from(M31::from(self.claim.sha256_builtin_segment_start))
                    + (seq.clone() * M31_24.clone()))
                    + M31_7.clone()),
            ],
            low_16_bits_col42.clone(),
            high_16_bits_col43.clone(),
            low_7_ms_bits_col44.clone(),
            high_14_ms_bits_col45.clone(),
            high_5_ms_bits_col46.clone(),
            input_7_id_col47.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        ReadBlakeWord::evaluate(
            [
                ((E::F::from(M31::from(self.claim.sha256_builtin_segment_start))
                    + (seq.clone() * M31_24.clone()))
                    + M31_8.clone()),
            ],
            low_16_bits_col48.clone(),
            high_16_bits_col49.clone(),
            low_7_ms_bits_col50.clone(),
            high_14_ms_bits_col51.clone(),
            high_5_ms_bits_col52.clone(),
            input_8_id_col53.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        ReadBlakeWord::evaluate(
            [
                ((E::F::from(M31::from(self.claim.sha256_builtin_segment_start))
                    + (seq.clone() * M31_24.clone()))
                    + M31_9.clone()),
            ],
            low_16_bits_col54.clone(),
            high_16_bits_col55.clone(),
            low_7_ms_bits_col56.clone(),
            high_14_ms_bits_col57.clone(),
            high_5_ms_bits_col58.clone(),
            input_9_id_col59.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        ReadBlakeWord::evaluate(
            [
                ((E::F::from(M31::from(self.claim.sha256_builtin_segment_start))
                    + (seq.clone() * M31_24.clone()))
                    + M31_10.clone()),
            ],
            low_16_bits_col60.clone(),
            high_16_bits_col61.clone(),
            low_7_ms_bits_col62.clone(),
            high_14_ms_bits_col63.clone(),
            high_5_ms_bits_col64.clone(),
            input_10_id_col65.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        ReadBlakeWord::evaluate(
            [
                ((E::F::from(M31::from(self.claim.sha256_builtin_segment_start))
                    + (seq.clone() * M31_24.clone()))
                    + M31_11.clone()),
            ],
            low_16_bits_col66.clone(),
            high_16_bits_col67.clone(),
            low_7_ms_bits_col68.clone(),
            high_14_ms_bits_col69.clone(),
            high_5_ms_bits_col70.clone(),
            input_11_id_col71.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        ReadBlakeWord::evaluate(
            [
                ((E::F::from(M31::from(self.claim.sha256_builtin_segment_start))
                    + (seq.clone() * M31_24.clone()))
                    + M31_12.clone()),
            ],
            low_16_bits_col72.clone(),
            high_16_bits_col73.clone(),
            low_7_ms_bits_col74.clone(),
            high_14_ms_bits_col75.clone(),
            high_5_ms_bits_col76.clone(),
            input_12_id_col77.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        ReadBlakeWord::evaluate(
            [
                ((E::F::from(M31::from(self.claim.sha256_builtin_segment_start))
                    + (seq.clone() * M31_24.clone()))
                    + M31_13.clone()),
            ],
            low_16_bits_col78.clone(),
            high_16_bits_col79.clone(),
            low_7_ms_bits_col80.clone(),
            high_14_ms_bits_col81.clone(),
            high_5_ms_bits_col82.clone(),
            input_13_id_col83.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        ReadBlakeWord::evaluate(
            [
                ((E::F::from(M31::from(self.claim.sha256_builtin_segment_start))
                    + (seq.clone() * M31_24.clone()))
                    + M31_14.clone()),
            ],
            low_16_bits_col84.clone(),
            high_16_bits_col85.clone(),
            low_7_ms_bits_col86.clone(),
            high_14_ms_bits_col87.clone(),
            high_5_ms_bits_col88.clone(),
            input_14_id_col89.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        ReadBlakeWord::evaluate(
            [
                ((E::F::from(M31::from(self.claim.sha256_builtin_segment_start))
                    + (seq.clone() * M31_24.clone()))
                    + M31_15.clone()),
            ],
            low_16_bits_col90.clone(),
            high_16_bits_col91.clone(),
            low_7_ms_bits_col92.clone(),
            high_14_ms_bits_col93.clone(),
            high_5_ms_bits_col94.clone(),
            input_15_id_col95.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        eval.add_to_relation(RelationEntry::new(
            &self.sha_256_round_lookup_elements,
            -E::EF::one(),
            &[
                seq.clone(),
                M31_0.clone(),
                M31_58983.clone(),
                M31_27145.clone(),
                M31_44677.clone(),
                M31_47975.clone(),
                M31_62322.clone(),
                M31_15470.clone(),
                M31_62778.clone(),
                M31_42319.clone(),
                M31_21119.clone(),
                M31_20750.clone(),
                M31_26764.clone(),
                M31_39685.clone(),
                M31_55723.clone(),
                M31_8067.clone(),
                M31_52505.clone(),
                M31_23520.clone(),
                low_16_bits_col0.clone(),
                high_16_bits_col1.clone(),
                low_16_bits_col6.clone(),
                high_16_bits_col7.clone(),
                low_16_bits_col12.clone(),
                high_16_bits_col13.clone(),
                low_16_bits_col18.clone(),
                high_16_bits_col19.clone(),
                low_16_bits_col24.clone(),
                high_16_bits_col25.clone(),
                low_16_bits_col30.clone(),
                high_16_bits_col31.clone(),
                low_16_bits_col36.clone(),
                high_16_bits_col37.clone(),
                low_16_bits_col42.clone(),
                high_16_bits_col43.clone(),
                low_16_bits_col48.clone(),
                high_16_bits_col49.clone(),
                low_16_bits_col54.clone(),
                high_16_bits_col55.clone(),
                low_16_bits_col60.clone(),
                high_16_bits_col61.clone(),
                low_16_bits_col66.clone(),
                high_16_bits_col67.clone(),
                low_16_bits_col72.clone(),
                high_16_bits_col73.clone(),
                low_16_bits_col78.clone(),
                high_16_bits_col79.clone(),
                low_16_bits_col84.clone(),
                high_16_bits_col85.clone(),
                low_16_bits_col90.clone(),
                high_16_bits_col91.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.sha_256_round_lookup_elements,
            E::EF::one(),
            &[
                seq.clone(),
                M31_64.clone(),
                sha_256_round_output_limb_0_col96.clone(),
                sha_256_round_output_limb_1_col97.clone(),
                sha_256_round_output_limb_2_col98.clone(),
                sha_256_round_output_limb_3_col99.clone(),
                sha_256_round_output_limb_4_col100.clone(),
                sha_256_round_output_limb_5_col101.clone(),
                sha_256_round_output_limb_6_col102.clone(),
                sha_256_round_output_limb_7_col103.clone(),
                sha_256_round_output_limb_8_col104.clone(),
                sha_256_round_output_limb_9_col105.clone(),
                sha_256_round_output_limb_10_col106.clone(),
                sha_256_round_output_limb_11_col107.clone(),
                sha_256_round_output_limb_12_col108.clone(),
                sha_256_round_output_limb_13_col109.clone(),
                sha_256_round_output_limb_14_col110.clone(),
                sha_256_round_output_limb_15_col111.clone(),
                sha_256_round_output_limb_16_col112.clone(),
                sha_256_round_output_limb_17_col113.clone(),
                sha_256_round_output_limb_18_col114.clone(),
                sha_256_round_output_limb_19_col115.clone(),
                sha_256_round_output_limb_20_col116.clone(),
                sha_256_round_output_limb_21_col117.clone(),
                sha_256_round_output_limb_22_col118.clone(),
                sha_256_round_output_limb_23_col119.clone(),
                sha_256_round_output_limb_24_col120.clone(),
                sha_256_round_output_limb_25_col121.clone(),
                sha_256_round_output_limb_26_col122.clone(),
                sha_256_round_output_limb_27_col123.clone(),
                sha_256_round_output_limb_28_col124.clone(),
                sha_256_round_output_limb_29_col125.clone(),
                sha_256_round_output_limb_30_col126.clone(),
                sha_256_round_output_limb_31_col127.clone(),
                sha_256_round_output_limb_32_col128.clone(),
                sha_256_round_output_limb_33_col129.clone(),
                sha_256_round_output_limb_34_col130.clone(),
                sha_256_round_output_limb_35_col131.clone(),
                sha_256_round_output_limb_36_col132.clone(),
                sha_256_round_output_limb_37_col133.clone(),
                sha_256_round_output_limb_38_col134.clone(),
                sha_256_round_output_limb_39_col135.clone(),
                sha_256_round_output_limb_40_col136.clone(),
                sha_256_round_output_limb_41_col137.clone(),
                sha_256_round_output_limb_42_col138.clone(),
                sha_256_round_output_limb_43_col139.clone(),
                sha_256_round_output_limb_44_col140.clone(),
                sha_256_round_output_limb_45_col141.clone(),
                sha_256_round_output_limb_46_col142.clone(),
                sha_256_round_output_limb_47_col143.clone(),
            ],
        ));

        VerifyBlakeWord::evaluate(
            [
                ((E::F::from(M31::from(self.claim.sha256_builtin_segment_start))
                    + (seq.clone() * M31_24.clone()))
                    + M31_16.clone()),
                sha_256_round_output_limb_0_col96.clone(),
                sha_256_round_output_limb_1_col97.clone(),
            ],
            low_7_ms_bits_col144.clone(),
            high_14_ms_bits_col145.clone(),
            high_5_ms_bits_col146.clone(),
            output_0_id_col147.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        VerifyBlakeWord::evaluate(
            [
                ((E::F::from(M31::from(self.claim.sha256_builtin_segment_start))
                    + (seq.clone() * M31_24.clone()))
                    + M31_17.clone()),
                sha_256_round_output_limb_2_col98.clone(),
                sha_256_round_output_limb_3_col99.clone(),
            ],
            low_7_ms_bits_col148.clone(),
            high_14_ms_bits_col149.clone(),
            high_5_ms_bits_col150.clone(),
            output_1_id_col151.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        VerifyBlakeWord::evaluate(
            [
                ((E::F::from(M31::from(self.claim.sha256_builtin_segment_start))
                    + (seq.clone() * M31_24.clone()))
                    + M31_18.clone()),
                sha_256_round_output_limb_4_col100.clone(),
                sha_256_round_output_limb_5_col101.clone(),
            ],
            low_7_ms_bits_col152.clone(),
            high_14_ms_bits_col153.clone(),
            high_5_ms_bits_col154.clone(),
            output_2_id_col155.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        VerifyBlakeWord::evaluate(
            [
                ((E::F::from(M31::from(self.claim.sha256_builtin_segment_start))
                    + (seq.clone() * M31_24.clone()))
                    + M31_19.clone()),
                sha_256_round_output_limb_6_col102.clone(),
                sha_256_round_output_limb_7_col103.clone(),
            ],
            low_7_ms_bits_col156.clone(),
            high_14_ms_bits_col157.clone(),
            high_5_ms_bits_col158.clone(),
            output_3_id_col159.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        VerifyBlakeWord::evaluate(
            [
                ((E::F::from(M31::from(self.claim.sha256_builtin_segment_start))
                    + (seq.clone() * M31_24.clone()))
                    + M31_20.clone()),
                sha_256_round_output_limb_8_col104.clone(),
                sha_256_round_output_limb_9_col105.clone(),
            ],
            low_7_ms_bits_col160.clone(),
            high_14_ms_bits_col161.clone(),
            high_5_ms_bits_col162.clone(),
            output_4_id_col163.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        VerifyBlakeWord::evaluate(
            [
                ((E::F::from(M31::from(self.claim.sha256_builtin_segment_start))
                    + (seq.clone() * M31_24.clone()))
                    + M31_21.clone()),
                sha_256_round_output_limb_10_col106.clone(),
                sha_256_round_output_limb_11_col107.clone(),
            ],
            low_7_ms_bits_col164.clone(),
            high_14_ms_bits_col165.clone(),
            high_5_ms_bits_col166.clone(),
            output_5_id_col167.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        VerifyBlakeWord::evaluate(
            [
                ((E::F::from(M31::from(self.claim.sha256_builtin_segment_start))
                    + (seq.clone() * M31_24.clone()))
                    + M31_22.clone()),
                sha_256_round_output_limb_12_col108.clone(),
                sha_256_round_output_limb_13_col109.clone(),
            ],
            low_7_ms_bits_col168.clone(),
            high_14_ms_bits_col169.clone(),
            high_5_ms_bits_col170.clone(),
            output_6_id_col171.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        VerifyBlakeWord::evaluate(
            [
                ((E::F::from(M31::from(self.claim.sha256_builtin_segment_start))
                    + (seq.clone() * M31_24.clone()))
                    + M31_23.clone()),
                sha_256_round_output_limb_14_col110.clone(),
                sha_256_round_output_limb_15_col111.clone(),
            ],
            low_7_ms_bits_col172.clone(),
            high_14_ms_bits_col173.clone(),
            high_5_ms_bits_col174.clone(),
            output_7_id_col175.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
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
    use crate::components::constraints_regression_test_values::SHA_256_BUILTIN;

    #[test]
    fn sha_256_builtin_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim {
                log_size: 4,
                sha256_builtin_segment_start: rng.gen::<u32>(),
            },
            range_check_7_2_5_lookup_elements: relations::RangeCheck_7_2_5::dummy(),
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            sha_256_round_lookup_elements: relations::Sha256Round::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        assert_eq!(sum, SHA_256_BUILTIN);
    }
}
