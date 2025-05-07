use crate::components::prelude::*;
use crate::components::subroutines::create_blake_output::CreateBlakeOutput;
use crate::components::subroutines::create_blake_round_input::CreateBlakeRoundInput;
use crate::components::subroutines::decode_blake_opcode::DecodeBlakeOpcode;
use crate::components::subroutines::verify_blake_word::VerifyBlakeWord;

pub const N_TRACE_COLUMNS: usize = 169;
pub const RELATION_USES_PER_ROW: [RelationUse; 8] = [
    RelationUse {
        relation_id: "BlakeRound",
        uses: 1,
    },
    RelationUse {
        relation_id: "MemoryAddressToId",
        uses: 20,
    },
    RelationUse {
        relation_id: "MemoryIdToBig",
        uses: 20,
    },
    RelationUse {
        relation_id: "Opcodes",
        uses: 1,
    },
    RelationUse {
        relation_id: "RangeCheck_7_2_5",
        uses: 17,
    },
    RelationUse {
        relation_id: "TripleXor32",
        uses: 8,
    },
    RelationUse {
        relation_id: "VerifyBitwiseXor_8",
        uses: 4,
    },
    RelationUse {
        relation_id: "VerifyInstruction",
        uses: 1,
    },
];

pub struct Eval {
    pub claim: Claim,
    pub verify_instruction_lookup_elements: relations::VerifyInstruction,
    pub memory_address_to_id_lookup_elements: relations::MemoryAddressToId,
    pub memory_id_to_big_lookup_elements: relations::MemoryIdToBig,
    pub range_check_7_2_5_lookup_elements: relations::RangeCheck_7_2_5,
    pub verify_bitwise_xor_8_lookup_elements: relations::VerifyBitwiseXor_8,
    pub blake_round_lookup_elements: relations::BlakeRound,
    pub triple_xor_32_lookup_elements: relations::TripleXor32,
    pub opcodes_lookup_elements: relations::Opcodes,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Claim {
    pub log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 37];
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
        let M31_10 = E::F::from(M31::from(10));
        let M31_15470 = E::F::from(M31::from(15470));
        let M31_2 = E::F::from(M31::from(2));
        let M31_23520 = E::F::from(M31::from(23520));
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
        let M31_58983 = E::F::from(M31::from(58983));
        let M31_6 = E::F::from(M31::from(6));
        let M31_62322 = E::F::from(M31::from(62322));
        let M31_62778 = E::F::from(M31::from(62778));
        let M31_7 = E::F::from(M31::from(7));
        let seq = eval.get_preprocessed_column(Seq::new(self.log_size()).id());
        let input_pc_col0 = eval.next_trace_mask();
        let input_ap_col1 = eval.next_trace_mask();
        let input_fp_col2 = eval.next_trace_mask();
        let offset0_col3 = eval.next_trace_mask();
        let offset1_col4 = eval.next_trace_mask();
        let offset2_col5 = eval.next_trace_mask();
        let dst_base_fp_col6 = eval.next_trace_mask();
        let op0_base_fp_col7 = eval.next_trace_mask();
        let op1_base_fp_col8 = eval.next_trace_mask();
        let op1_base_ap_col9 = eval.next_trace_mask();
        let ap_update_add_1_col10 = eval.next_trace_mask();
        let opcode_extension_col11 = eval.next_trace_mask();
        let mem0_base_col12 = eval.next_trace_mask();
        let op0_id_col13 = eval.next_trace_mask();
        let op0_limb_0_col14 = eval.next_trace_mask();
        let op0_limb_1_col15 = eval.next_trace_mask();
        let op0_limb_2_col16 = eval.next_trace_mask();
        let mem1_base_col17 = eval.next_trace_mask();
        let op1_id_col18 = eval.next_trace_mask();
        let op1_limb_0_col19 = eval.next_trace_mask();
        let op1_limb_1_col20 = eval.next_trace_mask();
        let op1_limb_2_col21 = eval.next_trace_mask();
        let ap_id_col22 = eval.next_trace_mask();
        let ap_limb_0_col23 = eval.next_trace_mask();
        let ap_limb_1_col24 = eval.next_trace_mask();
        let ap_limb_2_col25 = eval.next_trace_mask();
        let mem_dst_base_col26 = eval.next_trace_mask();
        let low_16_bits_col27 = eval.next_trace_mask();
        let high_16_bits_col28 = eval.next_trace_mask();
        let low_7_ms_bits_col29 = eval.next_trace_mask();
        let high_14_ms_bits_col30 = eval.next_trace_mask();
        let high_5_ms_bits_col31 = eval.next_trace_mask();
        let dst_id_col32 = eval.next_trace_mask();
        let low_16_bits_col33 = eval.next_trace_mask();
        let high_16_bits_col34 = eval.next_trace_mask();
        let low_7_ms_bits_col35 = eval.next_trace_mask();
        let high_14_ms_bits_col36 = eval.next_trace_mask();
        let high_5_ms_bits_col37 = eval.next_trace_mask();
        let state_0_id_col38 = eval.next_trace_mask();
        let low_16_bits_col39 = eval.next_trace_mask();
        let high_16_bits_col40 = eval.next_trace_mask();
        let low_7_ms_bits_col41 = eval.next_trace_mask();
        let high_14_ms_bits_col42 = eval.next_trace_mask();
        let high_5_ms_bits_col43 = eval.next_trace_mask();
        let state_1_id_col44 = eval.next_trace_mask();
        let low_16_bits_col45 = eval.next_trace_mask();
        let high_16_bits_col46 = eval.next_trace_mask();
        let low_7_ms_bits_col47 = eval.next_trace_mask();
        let high_14_ms_bits_col48 = eval.next_trace_mask();
        let high_5_ms_bits_col49 = eval.next_trace_mask();
        let state_2_id_col50 = eval.next_trace_mask();
        let low_16_bits_col51 = eval.next_trace_mask();
        let high_16_bits_col52 = eval.next_trace_mask();
        let low_7_ms_bits_col53 = eval.next_trace_mask();
        let high_14_ms_bits_col54 = eval.next_trace_mask();
        let high_5_ms_bits_col55 = eval.next_trace_mask();
        let state_3_id_col56 = eval.next_trace_mask();
        let low_16_bits_col57 = eval.next_trace_mask();
        let high_16_bits_col58 = eval.next_trace_mask();
        let low_7_ms_bits_col59 = eval.next_trace_mask();
        let high_14_ms_bits_col60 = eval.next_trace_mask();
        let high_5_ms_bits_col61 = eval.next_trace_mask();
        let state_4_id_col62 = eval.next_trace_mask();
        let low_16_bits_col63 = eval.next_trace_mask();
        let high_16_bits_col64 = eval.next_trace_mask();
        let low_7_ms_bits_col65 = eval.next_trace_mask();
        let high_14_ms_bits_col66 = eval.next_trace_mask();
        let high_5_ms_bits_col67 = eval.next_trace_mask();
        let state_5_id_col68 = eval.next_trace_mask();
        let low_16_bits_col69 = eval.next_trace_mask();
        let high_16_bits_col70 = eval.next_trace_mask();
        let low_7_ms_bits_col71 = eval.next_trace_mask();
        let high_14_ms_bits_col72 = eval.next_trace_mask();
        let high_5_ms_bits_col73 = eval.next_trace_mask();
        let state_6_id_col74 = eval.next_trace_mask();
        let low_16_bits_col75 = eval.next_trace_mask();
        let high_16_bits_col76 = eval.next_trace_mask();
        let low_7_ms_bits_col77 = eval.next_trace_mask();
        let high_14_ms_bits_col78 = eval.next_trace_mask();
        let high_5_ms_bits_col79 = eval.next_trace_mask();
        let state_7_id_col80 = eval.next_trace_mask();
        let ms_8_bits_col81 = eval.next_trace_mask();
        let ms_8_bits_col82 = eval.next_trace_mask();
        let xor_col83 = eval.next_trace_mask();
        let xor_col84 = eval.next_trace_mask();
        let xor_col85 = eval.next_trace_mask();
        let xor_col86 = eval.next_trace_mask();
        let blake_round_output_limb_0_col87 = eval.next_trace_mask();
        let blake_round_output_limb_1_col88 = eval.next_trace_mask();
        let blake_round_output_limb_2_col89 = eval.next_trace_mask();
        let blake_round_output_limb_3_col90 = eval.next_trace_mask();
        let blake_round_output_limb_4_col91 = eval.next_trace_mask();
        let blake_round_output_limb_5_col92 = eval.next_trace_mask();
        let blake_round_output_limb_6_col93 = eval.next_trace_mask();
        let blake_round_output_limb_7_col94 = eval.next_trace_mask();
        let blake_round_output_limb_8_col95 = eval.next_trace_mask();
        let blake_round_output_limb_9_col96 = eval.next_trace_mask();
        let blake_round_output_limb_10_col97 = eval.next_trace_mask();
        let blake_round_output_limb_11_col98 = eval.next_trace_mask();
        let blake_round_output_limb_12_col99 = eval.next_trace_mask();
        let blake_round_output_limb_13_col100 = eval.next_trace_mask();
        let blake_round_output_limb_14_col101 = eval.next_trace_mask();
        let blake_round_output_limb_15_col102 = eval.next_trace_mask();
        let blake_round_output_limb_16_col103 = eval.next_trace_mask();
        let blake_round_output_limb_17_col104 = eval.next_trace_mask();
        let blake_round_output_limb_18_col105 = eval.next_trace_mask();
        let blake_round_output_limb_19_col106 = eval.next_trace_mask();
        let blake_round_output_limb_20_col107 = eval.next_trace_mask();
        let blake_round_output_limb_21_col108 = eval.next_trace_mask();
        let blake_round_output_limb_22_col109 = eval.next_trace_mask();
        let blake_round_output_limb_23_col110 = eval.next_trace_mask();
        let blake_round_output_limb_24_col111 = eval.next_trace_mask();
        let blake_round_output_limb_25_col112 = eval.next_trace_mask();
        let blake_round_output_limb_26_col113 = eval.next_trace_mask();
        let blake_round_output_limb_27_col114 = eval.next_trace_mask();
        let blake_round_output_limb_28_col115 = eval.next_trace_mask();
        let blake_round_output_limb_29_col116 = eval.next_trace_mask();
        let blake_round_output_limb_30_col117 = eval.next_trace_mask();
        let blake_round_output_limb_31_col118 = eval.next_trace_mask();
        let blake_round_output_limb_32_col119 = eval.next_trace_mask();
        let triple_xor_32_output_limb_0_col120 = eval.next_trace_mask();
        let triple_xor_32_output_limb_1_col121 = eval.next_trace_mask();
        let triple_xor_32_output_limb_0_col122 = eval.next_trace_mask();
        let triple_xor_32_output_limb_1_col123 = eval.next_trace_mask();
        let triple_xor_32_output_limb_0_col124 = eval.next_trace_mask();
        let triple_xor_32_output_limb_1_col125 = eval.next_trace_mask();
        let triple_xor_32_output_limb_0_col126 = eval.next_trace_mask();
        let triple_xor_32_output_limb_1_col127 = eval.next_trace_mask();
        let triple_xor_32_output_limb_0_col128 = eval.next_trace_mask();
        let triple_xor_32_output_limb_1_col129 = eval.next_trace_mask();
        let triple_xor_32_output_limb_0_col130 = eval.next_trace_mask();
        let triple_xor_32_output_limb_1_col131 = eval.next_trace_mask();
        let triple_xor_32_output_limb_0_col132 = eval.next_trace_mask();
        let triple_xor_32_output_limb_1_col133 = eval.next_trace_mask();
        let triple_xor_32_output_limb_0_col134 = eval.next_trace_mask();
        let triple_xor_32_output_limb_1_col135 = eval.next_trace_mask();
        let low_7_ms_bits_col136 = eval.next_trace_mask();
        let high_14_ms_bits_col137 = eval.next_trace_mask();
        let high_5_ms_bits_col138 = eval.next_trace_mask();
        let new_state_0_id_col139 = eval.next_trace_mask();
        let low_7_ms_bits_col140 = eval.next_trace_mask();
        let high_14_ms_bits_col141 = eval.next_trace_mask();
        let high_5_ms_bits_col142 = eval.next_trace_mask();
        let new_state_1_id_col143 = eval.next_trace_mask();
        let low_7_ms_bits_col144 = eval.next_trace_mask();
        let high_14_ms_bits_col145 = eval.next_trace_mask();
        let high_5_ms_bits_col146 = eval.next_trace_mask();
        let new_state_2_id_col147 = eval.next_trace_mask();
        let low_7_ms_bits_col148 = eval.next_trace_mask();
        let high_14_ms_bits_col149 = eval.next_trace_mask();
        let high_5_ms_bits_col150 = eval.next_trace_mask();
        let new_state_3_id_col151 = eval.next_trace_mask();
        let low_7_ms_bits_col152 = eval.next_trace_mask();
        let high_14_ms_bits_col153 = eval.next_trace_mask();
        let high_5_ms_bits_col154 = eval.next_trace_mask();
        let new_state_4_id_col155 = eval.next_trace_mask();
        let low_7_ms_bits_col156 = eval.next_trace_mask();
        let high_14_ms_bits_col157 = eval.next_trace_mask();
        let high_5_ms_bits_col158 = eval.next_trace_mask();
        let new_state_5_id_col159 = eval.next_trace_mask();
        let low_7_ms_bits_col160 = eval.next_trace_mask();
        let high_14_ms_bits_col161 = eval.next_trace_mask();
        let high_5_ms_bits_col162 = eval.next_trace_mask();
        let new_state_6_id_col163 = eval.next_trace_mask();
        let low_7_ms_bits_col164 = eval.next_trace_mask();
        let high_14_ms_bits_col165 = eval.next_trace_mask();
        let high_5_ms_bits_col166 = eval.next_trace_mask();
        let new_state_7_id_col167 = eval.next_trace_mask();
        let enabler = eval.next_trace_mask();

        eval.add_constraint(enabler.clone() * enabler.clone() - enabler.clone());

        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [decode_blake_opcode_output_tmp_53f39_29_limb_0, decode_blake_opcode_output_tmp_53f39_29_limb_1, decode_blake_opcode_output_tmp_53f39_29_limb_2, decode_blake_opcode_output_tmp_53f39_29_limb_6] =
            DecodeBlakeOpcode::evaluate(
                [
                    input_pc_col0.clone(),
                    input_ap_col1.clone(),
                    input_fp_col2.clone(),
                ],
                offset0_col3.clone(),
                offset1_col4.clone(),
                offset2_col5.clone(),
                dst_base_fp_col6.clone(),
                op0_base_fp_col7.clone(),
                op1_base_fp_col8.clone(),
                op1_base_ap_col9.clone(),
                ap_update_add_1_col10.clone(),
                opcode_extension_col11.clone(),
                mem0_base_col12.clone(),
                op0_id_col13.clone(),
                op0_limb_0_col14.clone(),
                op0_limb_1_col15.clone(),
                op0_limb_2_col16.clone(),
                mem1_base_col17.clone(),
                op1_id_col18.clone(),
                op1_limb_0_col19.clone(),
                op1_limb_1_col20.clone(),
                op1_limb_2_col21.clone(),
                ap_id_col22.clone(),
                ap_limb_0_col23.clone(),
                ap_limb_1_col24.clone(),
                ap_limb_2_col25.clone(),
                mem_dst_base_col26.clone(),
                low_16_bits_col27.clone(),
                high_16_bits_col28.clone(),
                low_7_ms_bits_col29.clone(),
                high_14_ms_bits_col30.clone(),
                high_5_ms_bits_col31.clone(),
                dst_id_col32.clone(),
                &self.verify_instruction_lookup_elements,
                &self.memory_address_to_id_lookup_elements,
                &self.memory_id_to_big_lookup_elements,
                &self.range_check_7_2_5_lookup_elements,
                &mut eval,
            );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [create_blake_round_input_output_tmp_53f39_114_limb_24, create_blake_round_input_output_tmp_53f39_114_limb_25, create_blake_round_input_output_tmp_53f39_114_limb_28, create_blake_round_input_output_tmp_53f39_114_limb_29] =
            CreateBlakeRoundInput::evaluate(
                [
                    decode_blake_opcode_output_tmp_53f39_29_limb_0.clone(),
                    low_16_bits_col27.clone(),
                    high_16_bits_col28.clone(),
                    decode_blake_opcode_output_tmp_53f39_29_limb_6.clone(),
                ],
                low_16_bits_col33.clone(),
                high_16_bits_col34.clone(),
                low_7_ms_bits_col35.clone(),
                high_14_ms_bits_col36.clone(),
                high_5_ms_bits_col37.clone(),
                state_0_id_col38.clone(),
                low_16_bits_col39.clone(),
                high_16_bits_col40.clone(),
                low_7_ms_bits_col41.clone(),
                high_14_ms_bits_col42.clone(),
                high_5_ms_bits_col43.clone(),
                state_1_id_col44.clone(),
                low_16_bits_col45.clone(),
                high_16_bits_col46.clone(),
                low_7_ms_bits_col47.clone(),
                high_14_ms_bits_col48.clone(),
                high_5_ms_bits_col49.clone(),
                state_2_id_col50.clone(),
                low_16_bits_col51.clone(),
                high_16_bits_col52.clone(),
                low_7_ms_bits_col53.clone(),
                high_14_ms_bits_col54.clone(),
                high_5_ms_bits_col55.clone(),
                state_3_id_col56.clone(),
                low_16_bits_col57.clone(),
                high_16_bits_col58.clone(),
                low_7_ms_bits_col59.clone(),
                high_14_ms_bits_col60.clone(),
                high_5_ms_bits_col61.clone(),
                state_4_id_col62.clone(),
                low_16_bits_col63.clone(),
                high_16_bits_col64.clone(),
                low_7_ms_bits_col65.clone(),
                high_14_ms_bits_col66.clone(),
                high_5_ms_bits_col67.clone(),
                state_5_id_col68.clone(),
                low_16_bits_col69.clone(),
                high_16_bits_col70.clone(),
                low_7_ms_bits_col71.clone(),
                high_14_ms_bits_col72.clone(),
                high_5_ms_bits_col73.clone(),
                state_6_id_col74.clone(),
                low_16_bits_col75.clone(),
                high_16_bits_col76.clone(),
                low_7_ms_bits_col77.clone(),
                high_14_ms_bits_col78.clone(),
                high_5_ms_bits_col79.clone(),
                state_7_id_col80.clone(),
                ms_8_bits_col81.clone(),
                ms_8_bits_col82.clone(),
                xor_col83.clone(),
                xor_col84.clone(),
                xor_col85.clone(),
                xor_col86.clone(),
                &self.range_check_7_2_5_lookup_elements,
                &self.memory_address_to_id_lookup_elements,
                &self.memory_id_to_big_lookup_elements,
                &self.verify_bitwise_xor_8_lookup_elements,
                &mut eval,
            );
        eval.add_to_relation(RelationEntry::new(
            &self.blake_round_lookup_elements,
            -E::EF::one(),
            &[
                seq.clone(),
                M31_0.clone(),
                low_16_bits_col33.clone(),
                high_16_bits_col34.clone(),
                low_16_bits_col39.clone(),
                high_16_bits_col40.clone(),
                low_16_bits_col45.clone(),
                high_16_bits_col46.clone(),
                low_16_bits_col51.clone(),
                high_16_bits_col52.clone(),
                low_16_bits_col57.clone(),
                high_16_bits_col58.clone(),
                low_16_bits_col63.clone(),
                high_16_bits_col64.clone(),
                low_16_bits_col69.clone(),
                high_16_bits_col70.clone(),
                low_16_bits_col75.clone(),
                high_16_bits_col76.clone(),
                M31_58983.clone(),
                M31_27145.clone(),
                M31_44677.clone(),
                M31_47975.clone(),
                M31_62322.clone(),
                M31_15470.clone(),
                M31_62778.clone(),
                M31_42319.clone(),
                create_blake_round_input_output_tmp_53f39_114_limb_24.clone(),
                create_blake_round_input_output_tmp_53f39_114_limb_25.clone(),
                M31_26764.clone(),
                M31_39685.clone(),
                create_blake_round_input_output_tmp_53f39_114_limb_28.clone(),
                create_blake_round_input_output_tmp_53f39_114_limb_29.clone(),
                M31_52505.clone(),
                M31_23520.clone(),
                decode_blake_opcode_output_tmp_53f39_29_limb_1.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.blake_round_lookup_elements,
            E::EF::one(),
            &[
                seq.clone(),
                M31_10.clone(),
                blake_round_output_limb_0_col87.clone(),
                blake_round_output_limb_1_col88.clone(),
                blake_round_output_limb_2_col89.clone(),
                blake_round_output_limb_3_col90.clone(),
                blake_round_output_limb_4_col91.clone(),
                blake_round_output_limb_5_col92.clone(),
                blake_round_output_limb_6_col93.clone(),
                blake_round_output_limb_7_col94.clone(),
                blake_round_output_limb_8_col95.clone(),
                blake_round_output_limb_9_col96.clone(),
                blake_round_output_limb_10_col97.clone(),
                blake_round_output_limb_11_col98.clone(),
                blake_round_output_limb_12_col99.clone(),
                blake_round_output_limb_13_col100.clone(),
                blake_round_output_limb_14_col101.clone(),
                blake_round_output_limb_15_col102.clone(),
                blake_round_output_limb_16_col103.clone(),
                blake_round_output_limb_17_col104.clone(),
                blake_round_output_limb_18_col105.clone(),
                blake_round_output_limb_19_col106.clone(),
                blake_round_output_limb_20_col107.clone(),
                blake_round_output_limb_21_col108.clone(),
                blake_round_output_limb_22_col109.clone(),
                blake_round_output_limb_23_col110.clone(),
                blake_round_output_limb_24_col111.clone(),
                blake_round_output_limb_25_col112.clone(),
                blake_round_output_limb_26_col113.clone(),
                blake_round_output_limb_27_col114.clone(),
                blake_round_output_limb_28_col115.clone(),
                blake_round_output_limb_29_col116.clone(),
                blake_round_output_limb_30_col117.clone(),
                blake_round_output_limb_31_col118.clone(),
                blake_round_output_limb_32_col119.clone(),
            ],
        ));

        CreateBlakeOutput::evaluate(
            [
                low_16_bits_col33.clone(),
                high_16_bits_col34.clone(),
                low_16_bits_col39.clone(),
                high_16_bits_col40.clone(),
                low_16_bits_col45.clone(),
                high_16_bits_col46.clone(),
                low_16_bits_col51.clone(),
                high_16_bits_col52.clone(),
                low_16_bits_col57.clone(),
                high_16_bits_col58.clone(),
                low_16_bits_col63.clone(),
                high_16_bits_col64.clone(),
                low_16_bits_col69.clone(),
                high_16_bits_col70.clone(),
                low_16_bits_col75.clone(),
                high_16_bits_col76.clone(),
                blake_round_output_limb_0_col87.clone(),
                blake_round_output_limb_1_col88.clone(),
                blake_round_output_limb_2_col89.clone(),
                blake_round_output_limb_3_col90.clone(),
                blake_round_output_limb_4_col91.clone(),
                blake_round_output_limb_5_col92.clone(),
                blake_round_output_limb_6_col93.clone(),
                blake_round_output_limb_7_col94.clone(),
                blake_round_output_limb_8_col95.clone(),
                blake_round_output_limb_9_col96.clone(),
                blake_round_output_limb_10_col97.clone(),
                blake_round_output_limb_11_col98.clone(),
                blake_round_output_limb_12_col99.clone(),
                blake_round_output_limb_13_col100.clone(),
                blake_round_output_limb_14_col101.clone(),
                blake_round_output_limb_15_col102.clone(),
                blake_round_output_limb_16_col103.clone(),
                blake_round_output_limb_17_col104.clone(),
                blake_round_output_limb_18_col105.clone(),
                blake_round_output_limb_19_col106.clone(),
                blake_round_output_limb_20_col107.clone(),
                blake_round_output_limb_21_col108.clone(),
                blake_round_output_limb_22_col109.clone(),
                blake_round_output_limb_23_col110.clone(),
                blake_round_output_limb_24_col111.clone(),
                blake_round_output_limb_25_col112.clone(),
                blake_round_output_limb_26_col113.clone(),
                blake_round_output_limb_27_col114.clone(),
                blake_round_output_limb_28_col115.clone(),
                blake_round_output_limb_29_col116.clone(),
                blake_round_output_limb_30_col117.clone(),
                blake_round_output_limb_31_col118.clone(),
            ],
            triple_xor_32_output_limb_0_col120.clone(),
            triple_xor_32_output_limb_1_col121.clone(),
            triple_xor_32_output_limb_0_col122.clone(),
            triple_xor_32_output_limb_1_col123.clone(),
            triple_xor_32_output_limb_0_col124.clone(),
            triple_xor_32_output_limb_1_col125.clone(),
            triple_xor_32_output_limb_0_col126.clone(),
            triple_xor_32_output_limb_1_col127.clone(),
            triple_xor_32_output_limb_0_col128.clone(),
            triple_xor_32_output_limb_1_col129.clone(),
            triple_xor_32_output_limb_0_col130.clone(),
            triple_xor_32_output_limb_1_col131.clone(),
            triple_xor_32_output_limb_0_col132.clone(),
            triple_xor_32_output_limb_1_col133.clone(),
            triple_xor_32_output_limb_0_col134.clone(),
            triple_xor_32_output_limb_1_col135.clone(),
            &self.triple_xor_32_lookup_elements,
            &mut eval,
        );
        VerifyBlakeWord::evaluate(
            [
                decode_blake_opcode_output_tmp_53f39_29_limb_2.clone(),
                triple_xor_32_output_limb_0_col120.clone(),
                triple_xor_32_output_limb_1_col121.clone(),
            ],
            low_7_ms_bits_col136.clone(),
            high_14_ms_bits_col137.clone(),
            high_5_ms_bits_col138.clone(),
            new_state_0_id_col139.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        VerifyBlakeWord::evaluate(
            [
                (decode_blake_opcode_output_tmp_53f39_29_limb_2.clone() + M31_1.clone()),
                triple_xor_32_output_limb_0_col122.clone(),
                triple_xor_32_output_limb_1_col123.clone(),
            ],
            low_7_ms_bits_col140.clone(),
            high_14_ms_bits_col141.clone(),
            high_5_ms_bits_col142.clone(),
            new_state_1_id_col143.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        VerifyBlakeWord::evaluate(
            [
                (decode_blake_opcode_output_tmp_53f39_29_limb_2.clone() + M31_2.clone()),
                triple_xor_32_output_limb_0_col124.clone(),
                triple_xor_32_output_limb_1_col125.clone(),
            ],
            low_7_ms_bits_col144.clone(),
            high_14_ms_bits_col145.clone(),
            high_5_ms_bits_col146.clone(),
            new_state_2_id_col147.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        VerifyBlakeWord::evaluate(
            [
                (decode_blake_opcode_output_tmp_53f39_29_limb_2.clone() + M31_3.clone()),
                triple_xor_32_output_limb_0_col126.clone(),
                triple_xor_32_output_limb_1_col127.clone(),
            ],
            low_7_ms_bits_col148.clone(),
            high_14_ms_bits_col149.clone(),
            high_5_ms_bits_col150.clone(),
            new_state_3_id_col151.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        VerifyBlakeWord::evaluate(
            [
                (decode_blake_opcode_output_tmp_53f39_29_limb_2.clone() + M31_4.clone()),
                triple_xor_32_output_limb_0_col128.clone(),
                triple_xor_32_output_limb_1_col129.clone(),
            ],
            low_7_ms_bits_col152.clone(),
            high_14_ms_bits_col153.clone(),
            high_5_ms_bits_col154.clone(),
            new_state_4_id_col155.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        VerifyBlakeWord::evaluate(
            [
                (decode_blake_opcode_output_tmp_53f39_29_limb_2.clone() + M31_5.clone()),
                triple_xor_32_output_limb_0_col130.clone(),
                triple_xor_32_output_limb_1_col131.clone(),
            ],
            low_7_ms_bits_col156.clone(),
            high_14_ms_bits_col157.clone(),
            high_5_ms_bits_col158.clone(),
            new_state_5_id_col159.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        VerifyBlakeWord::evaluate(
            [
                (decode_blake_opcode_output_tmp_53f39_29_limb_2.clone() + M31_6.clone()),
                triple_xor_32_output_limb_0_col132.clone(),
                triple_xor_32_output_limb_1_col133.clone(),
            ],
            low_7_ms_bits_col160.clone(),
            high_14_ms_bits_col161.clone(),
            high_5_ms_bits_col162.clone(),
            new_state_6_id_col163.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        VerifyBlakeWord::evaluate(
            [
                (decode_blake_opcode_output_tmp_53f39_29_limb_2.clone() + M31_7.clone()),
                triple_xor_32_output_limb_0_col134.clone(),
                triple_xor_32_output_limb_1_col135.clone(),
            ],
            low_7_ms_bits_col164.clone(),
            high_14_ms_bits_col165.clone(),
            high_5_ms_bits_col166.clone(),
            new_state_7_id_col167.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        eval.add_to_relation(RelationEntry::new(
            &self.opcodes_lookup_elements,
            E::EF::from(enabler.clone()),
            &[
                input_pc_col0.clone(),
                input_ap_col1.clone(),
                input_fp_col2.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.opcodes_lookup_elements,
            -E::EF::from(enabler.clone()),
            &[
                (input_pc_col0.clone() + M31_1.clone()),
                (input_ap_col1.clone() + ap_update_add_1_col10.clone()),
                input_fp_col2.clone(),
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
    use crate::components::constraints_regression_test_values::BLAKE_COMPRESS_OPCODE;

    #[test]
    fn blake_compress_opcode_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim { log_size: 4 },
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            range_check_7_2_5_lookup_elements: relations::RangeCheck_7_2_5::dummy(),
            verify_bitwise_xor_8_lookup_elements: relations::VerifyBitwiseXor_8::dummy(),
            blake_round_lookup_elements: relations::BlakeRound::dummy(),
            triple_xor_32_lookup_elements: relations::TripleXor32::dummy(),
            opcodes_lookup_elements: relations::Opcodes::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        assert_eq!(sum, BLAKE_COMPRESS_OPCODE);
    }
}
