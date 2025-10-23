// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::create_blake_output::CreateBlakeOutput;
use crate::components::subroutines::create_blake_round_input::CreateBlakeRoundInput;
use crate::components::subroutines::decode_blake_opcode::DecodeBlakeOpcode;
use crate::components::subroutines::verify_u_32::VerifyU32;

pub const N_TRACE_COLUMNS: usize = 174;
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

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
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
        let ap_update_add_1_col9 = eval.next_trace_mask();
        let opcode_extension_col10 = eval.next_trace_mask();
        let mem0_base_col11 = eval.next_trace_mask();
        let op0_id_col12 = eval.next_trace_mask();
        let op0_limb_0_col13 = eval.next_trace_mask();
        let op0_limb_1_col14 = eval.next_trace_mask();
        let op0_limb_2_col15 = eval.next_trace_mask();
        let op0_limb_3_col16 = eval.next_trace_mask();
        let partial_limb_msb_col17 = eval.next_trace_mask();
        let mem1_base_col18 = eval.next_trace_mask();
        let op1_id_col19 = eval.next_trace_mask();
        let op1_limb_0_col20 = eval.next_trace_mask();
        let op1_limb_1_col21 = eval.next_trace_mask();
        let op1_limb_2_col22 = eval.next_trace_mask();
        let op1_limb_3_col23 = eval.next_trace_mask();
        let partial_limb_msb_col24 = eval.next_trace_mask();
        let ap_id_col25 = eval.next_trace_mask();
        let ap_limb_0_col26 = eval.next_trace_mask();
        let ap_limb_1_col27 = eval.next_trace_mask();
        let ap_limb_2_col28 = eval.next_trace_mask();
        let ap_limb_3_col29 = eval.next_trace_mask();
        let partial_limb_msb_col30 = eval.next_trace_mask();
        let mem_dst_base_col31 = eval.next_trace_mask();
        let low_16_bits_col32 = eval.next_trace_mask();
        let high_16_bits_col33 = eval.next_trace_mask();
        let low_7_ms_bits_col34 = eval.next_trace_mask();
        let high_14_ms_bits_col35 = eval.next_trace_mask();
        let high_5_ms_bits_col36 = eval.next_trace_mask();
        let dst_id_col37 = eval.next_trace_mask();
        let low_16_bits_col38 = eval.next_trace_mask();
        let high_16_bits_col39 = eval.next_trace_mask();
        let low_7_ms_bits_col40 = eval.next_trace_mask();
        let high_14_ms_bits_col41 = eval.next_trace_mask();
        let high_5_ms_bits_col42 = eval.next_trace_mask();
        let state_0_id_col43 = eval.next_trace_mask();
        let low_16_bits_col44 = eval.next_trace_mask();
        let high_16_bits_col45 = eval.next_trace_mask();
        let low_7_ms_bits_col46 = eval.next_trace_mask();
        let high_14_ms_bits_col47 = eval.next_trace_mask();
        let high_5_ms_bits_col48 = eval.next_trace_mask();
        let state_1_id_col49 = eval.next_trace_mask();
        let low_16_bits_col50 = eval.next_trace_mask();
        let high_16_bits_col51 = eval.next_trace_mask();
        let low_7_ms_bits_col52 = eval.next_trace_mask();
        let high_14_ms_bits_col53 = eval.next_trace_mask();
        let high_5_ms_bits_col54 = eval.next_trace_mask();
        let state_2_id_col55 = eval.next_trace_mask();
        let low_16_bits_col56 = eval.next_trace_mask();
        let high_16_bits_col57 = eval.next_trace_mask();
        let low_7_ms_bits_col58 = eval.next_trace_mask();
        let high_14_ms_bits_col59 = eval.next_trace_mask();
        let high_5_ms_bits_col60 = eval.next_trace_mask();
        let state_3_id_col61 = eval.next_trace_mask();
        let low_16_bits_col62 = eval.next_trace_mask();
        let high_16_bits_col63 = eval.next_trace_mask();
        let low_7_ms_bits_col64 = eval.next_trace_mask();
        let high_14_ms_bits_col65 = eval.next_trace_mask();
        let high_5_ms_bits_col66 = eval.next_trace_mask();
        let state_4_id_col67 = eval.next_trace_mask();
        let low_16_bits_col68 = eval.next_trace_mask();
        let high_16_bits_col69 = eval.next_trace_mask();
        let low_7_ms_bits_col70 = eval.next_trace_mask();
        let high_14_ms_bits_col71 = eval.next_trace_mask();
        let high_5_ms_bits_col72 = eval.next_trace_mask();
        let state_5_id_col73 = eval.next_trace_mask();
        let low_16_bits_col74 = eval.next_trace_mask();
        let high_16_bits_col75 = eval.next_trace_mask();
        let low_7_ms_bits_col76 = eval.next_trace_mask();
        let high_14_ms_bits_col77 = eval.next_trace_mask();
        let high_5_ms_bits_col78 = eval.next_trace_mask();
        let state_6_id_col79 = eval.next_trace_mask();
        let low_16_bits_col80 = eval.next_trace_mask();
        let high_16_bits_col81 = eval.next_trace_mask();
        let low_7_ms_bits_col82 = eval.next_trace_mask();
        let high_14_ms_bits_col83 = eval.next_trace_mask();
        let high_5_ms_bits_col84 = eval.next_trace_mask();
        let state_7_id_col85 = eval.next_trace_mask();
        let ms_8_bits_col86 = eval.next_trace_mask();
        let ms_8_bits_col87 = eval.next_trace_mask();
        let xor_col88 = eval.next_trace_mask();
        let xor_col89 = eval.next_trace_mask();
        let xor_col90 = eval.next_trace_mask();
        let xor_col91 = eval.next_trace_mask();
        let blake_round_output_limb_0_col92 = eval.next_trace_mask();
        let blake_round_output_limb_1_col93 = eval.next_trace_mask();
        let blake_round_output_limb_2_col94 = eval.next_trace_mask();
        let blake_round_output_limb_3_col95 = eval.next_trace_mask();
        let blake_round_output_limb_4_col96 = eval.next_trace_mask();
        let blake_round_output_limb_5_col97 = eval.next_trace_mask();
        let blake_round_output_limb_6_col98 = eval.next_trace_mask();
        let blake_round_output_limb_7_col99 = eval.next_trace_mask();
        let blake_round_output_limb_8_col100 = eval.next_trace_mask();
        let blake_round_output_limb_9_col101 = eval.next_trace_mask();
        let blake_round_output_limb_10_col102 = eval.next_trace_mask();
        let blake_round_output_limb_11_col103 = eval.next_trace_mask();
        let blake_round_output_limb_12_col104 = eval.next_trace_mask();
        let blake_round_output_limb_13_col105 = eval.next_trace_mask();
        let blake_round_output_limb_14_col106 = eval.next_trace_mask();
        let blake_round_output_limb_15_col107 = eval.next_trace_mask();
        let blake_round_output_limb_16_col108 = eval.next_trace_mask();
        let blake_round_output_limb_17_col109 = eval.next_trace_mask();
        let blake_round_output_limb_18_col110 = eval.next_trace_mask();
        let blake_round_output_limb_19_col111 = eval.next_trace_mask();
        let blake_round_output_limb_20_col112 = eval.next_trace_mask();
        let blake_round_output_limb_21_col113 = eval.next_trace_mask();
        let blake_round_output_limb_22_col114 = eval.next_trace_mask();
        let blake_round_output_limb_23_col115 = eval.next_trace_mask();
        let blake_round_output_limb_24_col116 = eval.next_trace_mask();
        let blake_round_output_limb_25_col117 = eval.next_trace_mask();
        let blake_round_output_limb_26_col118 = eval.next_trace_mask();
        let blake_round_output_limb_27_col119 = eval.next_trace_mask();
        let blake_round_output_limb_28_col120 = eval.next_trace_mask();
        let blake_round_output_limb_29_col121 = eval.next_trace_mask();
        let blake_round_output_limb_30_col122 = eval.next_trace_mask();
        let blake_round_output_limb_31_col123 = eval.next_trace_mask();
        let blake_round_output_limb_32_col124 = eval.next_trace_mask();
        let triple_xor_32_output_limb_0_col125 = eval.next_trace_mask();
        let triple_xor_32_output_limb_1_col126 = eval.next_trace_mask();
        let triple_xor_32_output_limb_0_col127 = eval.next_trace_mask();
        let triple_xor_32_output_limb_1_col128 = eval.next_trace_mask();
        let triple_xor_32_output_limb_0_col129 = eval.next_trace_mask();
        let triple_xor_32_output_limb_1_col130 = eval.next_trace_mask();
        let triple_xor_32_output_limb_0_col131 = eval.next_trace_mask();
        let triple_xor_32_output_limb_1_col132 = eval.next_trace_mask();
        let triple_xor_32_output_limb_0_col133 = eval.next_trace_mask();
        let triple_xor_32_output_limb_1_col134 = eval.next_trace_mask();
        let triple_xor_32_output_limb_0_col135 = eval.next_trace_mask();
        let triple_xor_32_output_limb_1_col136 = eval.next_trace_mask();
        let triple_xor_32_output_limb_0_col137 = eval.next_trace_mask();
        let triple_xor_32_output_limb_1_col138 = eval.next_trace_mask();
        let triple_xor_32_output_limb_0_col139 = eval.next_trace_mask();
        let triple_xor_32_output_limb_1_col140 = eval.next_trace_mask();
        let low_7_ms_bits_col141 = eval.next_trace_mask();
        let high_14_ms_bits_col142 = eval.next_trace_mask();
        let high_5_ms_bits_col143 = eval.next_trace_mask();
        let new_state_0_id_col144 = eval.next_trace_mask();
        let low_7_ms_bits_col145 = eval.next_trace_mask();
        let high_14_ms_bits_col146 = eval.next_trace_mask();
        let high_5_ms_bits_col147 = eval.next_trace_mask();
        let new_state_1_id_col148 = eval.next_trace_mask();
        let low_7_ms_bits_col149 = eval.next_trace_mask();
        let high_14_ms_bits_col150 = eval.next_trace_mask();
        let high_5_ms_bits_col151 = eval.next_trace_mask();
        let new_state_2_id_col152 = eval.next_trace_mask();
        let low_7_ms_bits_col153 = eval.next_trace_mask();
        let high_14_ms_bits_col154 = eval.next_trace_mask();
        let high_5_ms_bits_col155 = eval.next_trace_mask();
        let new_state_3_id_col156 = eval.next_trace_mask();
        let low_7_ms_bits_col157 = eval.next_trace_mask();
        let high_14_ms_bits_col158 = eval.next_trace_mask();
        let high_5_ms_bits_col159 = eval.next_trace_mask();
        let new_state_4_id_col160 = eval.next_trace_mask();
        let low_7_ms_bits_col161 = eval.next_trace_mask();
        let high_14_ms_bits_col162 = eval.next_trace_mask();
        let high_5_ms_bits_col163 = eval.next_trace_mask();
        let new_state_5_id_col164 = eval.next_trace_mask();
        let low_7_ms_bits_col165 = eval.next_trace_mask();
        let high_14_ms_bits_col166 = eval.next_trace_mask();
        let high_5_ms_bits_col167 = eval.next_trace_mask();
        let new_state_6_id_col168 = eval.next_trace_mask();
        let low_7_ms_bits_col169 = eval.next_trace_mask();
        let high_14_ms_bits_col170 = eval.next_trace_mask();
        let high_5_ms_bits_col171 = eval.next_trace_mask();
        let new_state_7_id_col172 = eval.next_trace_mask();
        let enabler = eval.next_trace_mask();

        eval.add_constraint(enabler.clone() * enabler.clone() - enabler.clone());

        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [decode_blake_opcode_output_tmp_53f39_42_limb_0, decode_blake_opcode_output_tmp_53f39_42_limb_1, decode_blake_opcode_output_tmp_53f39_42_limb_2, decode_blake_opcode_output_tmp_53f39_42_limb_6] =
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
                ap_update_add_1_col9.clone(),
                opcode_extension_col10.clone(),
                mem0_base_col11.clone(),
                op0_id_col12.clone(),
                op0_limb_0_col13.clone(),
                op0_limb_1_col14.clone(),
                op0_limb_2_col15.clone(),
                op0_limb_3_col16.clone(),
                partial_limb_msb_col17.clone(),
                mem1_base_col18.clone(),
                op1_id_col19.clone(),
                op1_limb_0_col20.clone(),
                op1_limb_1_col21.clone(),
                op1_limb_2_col22.clone(),
                op1_limb_3_col23.clone(),
                partial_limb_msb_col24.clone(),
                ap_id_col25.clone(),
                ap_limb_0_col26.clone(),
                ap_limb_1_col27.clone(),
                ap_limb_2_col28.clone(),
                ap_limb_3_col29.clone(),
                partial_limb_msb_col30.clone(),
                mem_dst_base_col31.clone(),
                low_16_bits_col32.clone(),
                high_16_bits_col33.clone(),
                low_7_ms_bits_col34.clone(),
                high_14_ms_bits_col35.clone(),
                high_5_ms_bits_col36.clone(),
                dst_id_col37.clone(),
                &self.verify_instruction_lookup_elements,
                &self.memory_address_to_id_lookup_elements,
                &self.memory_id_to_big_lookup_elements,
                &self.range_check_7_2_5_lookup_elements,
                &mut eval,
            );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [create_blake_round_input_output_tmp_53f39_143_limb_24, create_blake_round_input_output_tmp_53f39_143_limb_25, create_blake_round_input_output_tmp_53f39_143_limb_28, create_blake_round_input_output_tmp_53f39_143_limb_29] =
            CreateBlakeRoundInput::evaluate(
                [
                    decode_blake_opcode_output_tmp_53f39_42_limb_0.clone(),
                    low_16_bits_col32.clone(),
                    high_16_bits_col33.clone(),
                    decode_blake_opcode_output_tmp_53f39_42_limb_6.clone(),
                ],
                low_16_bits_col38.clone(),
                high_16_bits_col39.clone(),
                low_7_ms_bits_col40.clone(),
                high_14_ms_bits_col41.clone(),
                high_5_ms_bits_col42.clone(),
                state_0_id_col43.clone(),
                low_16_bits_col44.clone(),
                high_16_bits_col45.clone(),
                low_7_ms_bits_col46.clone(),
                high_14_ms_bits_col47.clone(),
                high_5_ms_bits_col48.clone(),
                state_1_id_col49.clone(),
                low_16_bits_col50.clone(),
                high_16_bits_col51.clone(),
                low_7_ms_bits_col52.clone(),
                high_14_ms_bits_col53.clone(),
                high_5_ms_bits_col54.clone(),
                state_2_id_col55.clone(),
                low_16_bits_col56.clone(),
                high_16_bits_col57.clone(),
                low_7_ms_bits_col58.clone(),
                high_14_ms_bits_col59.clone(),
                high_5_ms_bits_col60.clone(),
                state_3_id_col61.clone(),
                low_16_bits_col62.clone(),
                high_16_bits_col63.clone(),
                low_7_ms_bits_col64.clone(),
                high_14_ms_bits_col65.clone(),
                high_5_ms_bits_col66.clone(),
                state_4_id_col67.clone(),
                low_16_bits_col68.clone(),
                high_16_bits_col69.clone(),
                low_7_ms_bits_col70.clone(),
                high_14_ms_bits_col71.clone(),
                high_5_ms_bits_col72.clone(),
                state_5_id_col73.clone(),
                low_16_bits_col74.clone(),
                high_16_bits_col75.clone(),
                low_7_ms_bits_col76.clone(),
                high_14_ms_bits_col77.clone(),
                high_5_ms_bits_col78.clone(),
                state_6_id_col79.clone(),
                low_16_bits_col80.clone(),
                high_16_bits_col81.clone(),
                low_7_ms_bits_col82.clone(),
                high_14_ms_bits_col83.clone(),
                high_5_ms_bits_col84.clone(),
                state_7_id_col85.clone(),
                ms_8_bits_col86.clone(),
                ms_8_bits_col87.clone(),
                xor_col88.clone(),
                xor_col89.clone(),
                xor_col90.clone(),
                xor_col91.clone(),
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
                low_16_bits_col38.clone(),
                high_16_bits_col39.clone(),
                low_16_bits_col44.clone(),
                high_16_bits_col45.clone(),
                low_16_bits_col50.clone(),
                high_16_bits_col51.clone(),
                low_16_bits_col56.clone(),
                high_16_bits_col57.clone(),
                low_16_bits_col62.clone(),
                high_16_bits_col63.clone(),
                low_16_bits_col68.clone(),
                high_16_bits_col69.clone(),
                low_16_bits_col74.clone(),
                high_16_bits_col75.clone(),
                low_16_bits_col80.clone(),
                high_16_bits_col81.clone(),
                M31_58983.clone(),
                M31_27145.clone(),
                M31_44677.clone(),
                M31_47975.clone(),
                M31_62322.clone(),
                M31_15470.clone(),
                M31_62778.clone(),
                M31_42319.clone(),
                create_blake_round_input_output_tmp_53f39_143_limb_24.clone(),
                create_blake_round_input_output_tmp_53f39_143_limb_25.clone(),
                M31_26764.clone(),
                M31_39685.clone(),
                create_blake_round_input_output_tmp_53f39_143_limb_28.clone(),
                create_blake_round_input_output_tmp_53f39_143_limb_29.clone(),
                M31_52505.clone(),
                M31_23520.clone(),
                decode_blake_opcode_output_tmp_53f39_42_limb_1.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.blake_round_lookup_elements,
            E::EF::one(),
            &[
                seq.clone(),
                M31_10.clone(),
                blake_round_output_limb_0_col92.clone(),
                blake_round_output_limb_1_col93.clone(),
                blake_round_output_limb_2_col94.clone(),
                blake_round_output_limb_3_col95.clone(),
                blake_round_output_limb_4_col96.clone(),
                blake_round_output_limb_5_col97.clone(),
                blake_round_output_limb_6_col98.clone(),
                blake_round_output_limb_7_col99.clone(),
                blake_round_output_limb_8_col100.clone(),
                blake_round_output_limb_9_col101.clone(),
                blake_round_output_limb_10_col102.clone(),
                blake_round_output_limb_11_col103.clone(),
                blake_round_output_limb_12_col104.clone(),
                blake_round_output_limb_13_col105.clone(),
                blake_round_output_limb_14_col106.clone(),
                blake_round_output_limb_15_col107.clone(),
                blake_round_output_limb_16_col108.clone(),
                blake_round_output_limb_17_col109.clone(),
                blake_round_output_limb_18_col110.clone(),
                blake_round_output_limb_19_col111.clone(),
                blake_round_output_limb_20_col112.clone(),
                blake_round_output_limb_21_col113.clone(),
                blake_round_output_limb_22_col114.clone(),
                blake_round_output_limb_23_col115.clone(),
                blake_round_output_limb_24_col116.clone(),
                blake_round_output_limb_25_col117.clone(),
                blake_round_output_limb_26_col118.clone(),
                blake_round_output_limb_27_col119.clone(),
                blake_round_output_limb_28_col120.clone(),
                blake_round_output_limb_29_col121.clone(),
                blake_round_output_limb_30_col122.clone(),
                blake_round_output_limb_31_col123.clone(),
                blake_round_output_limb_32_col124.clone(),
            ],
        ));

        CreateBlakeOutput::evaluate(
            [
                low_16_bits_col38.clone(),
                high_16_bits_col39.clone(),
                low_16_bits_col44.clone(),
                high_16_bits_col45.clone(),
                low_16_bits_col50.clone(),
                high_16_bits_col51.clone(),
                low_16_bits_col56.clone(),
                high_16_bits_col57.clone(),
                low_16_bits_col62.clone(),
                high_16_bits_col63.clone(),
                low_16_bits_col68.clone(),
                high_16_bits_col69.clone(),
                low_16_bits_col74.clone(),
                high_16_bits_col75.clone(),
                low_16_bits_col80.clone(),
                high_16_bits_col81.clone(),
                blake_round_output_limb_0_col92.clone(),
                blake_round_output_limb_1_col93.clone(),
                blake_round_output_limb_2_col94.clone(),
                blake_round_output_limb_3_col95.clone(),
                blake_round_output_limb_4_col96.clone(),
                blake_round_output_limb_5_col97.clone(),
                blake_round_output_limb_6_col98.clone(),
                blake_round_output_limb_7_col99.clone(),
                blake_round_output_limb_8_col100.clone(),
                blake_round_output_limb_9_col101.clone(),
                blake_round_output_limb_10_col102.clone(),
                blake_round_output_limb_11_col103.clone(),
                blake_round_output_limb_12_col104.clone(),
                blake_round_output_limb_13_col105.clone(),
                blake_round_output_limb_14_col106.clone(),
                blake_round_output_limb_15_col107.clone(),
                blake_round_output_limb_16_col108.clone(),
                blake_round_output_limb_17_col109.clone(),
                blake_round_output_limb_18_col110.clone(),
                blake_round_output_limb_19_col111.clone(),
                blake_round_output_limb_20_col112.clone(),
                blake_round_output_limb_21_col113.clone(),
                blake_round_output_limb_22_col114.clone(),
                blake_round_output_limb_23_col115.clone(),
                blake_round_output_limb_24_col116.clone(),
                blake_round_output_limb_25_col117.clone(),
                blake_round_output_limb_26_col118.clone(),
                blake_round_output_limb_27_col119.clone(),
                blake_round_output_limb_28_col120.clone(),
                blake_round_output_limb_29_col121.clone(),
                blake_round_output_limb_30_col122.clone(),
                blake_round_output_limb_31_col123.clone(),
            ],
            triple_xor_32_output_limb_0_col125.clone(),
            triple_xor_32_output_limb_1_col126.clone(),
            triple_xor_32_output_limb_0_col127.clone(),
            triple_xor_32_output_limb_1_col128.clone(),
            triple_xor_32_output_limb_0_col129.clone(),
            triple_xor_32_output_limb_1_col130.clone(),
            triple_xor_32_output_limb_0_col131.clone(),
            triple_xor_32_output_limb_1_col132.clone(),
            triple_xor_32_output_limb_0_col133.clone(),
            triple_xor_32_output_limb_1_col134.clone(),
            triple_xor_32_output_limb_0_col135.clone(),
            triple_xor_32_output_limb_1_col136.clone(),
            triple_xor_32_output_limb_0_col137.clone(),
            triple_xor_32_output_limb_1_col138.clone(),
            triple_xor_32_output_limb_0_col139.clone(),
            triple_xor_32_output_limb_1_col140.clone(),
            &self.triple_xor_32_lookup_elements,
            &mut eval,
        );
        VerifyU32::evaluate(
            [
                decode_blake_opcode_output_tmp_53f39_42_limb_2.clone(),
                triple_xor_32_output_limb_0_col125.clone(),
                triple_xor_32_output_limb_1_col126.clone(),
            ],
            low_7_ms_bits_col141.clone(),
            high_14_ms_bits_col142.clone(),
            high_5_ms_bits_col143.clone(),
            new_state_0_id_col144.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        VerifyU32::evaluate(
            [
                (decode_blake_opcode_output_tmp_53f39_42_limb_2.clone() + M31_1.clone()),
                triple_xor_32_output_limb_0_col127.clone(),
                triple_xor_32_output_limb_1_col128.clone(),
            ],
            low_7_ms_bits_col145.clone(),
            high_14_ms_bits_col146.clone(),
            high_5_ms_bits_col147.clone(),
            new_state_1_id_col148.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        VerifyU32::evaluate(
            [
                (decode_blake_opcode_output_tmp_53f39_42_limb_2.clone() + M31_2.clone()),
                triple_xor_32_output_limb_0_col129.clone(),
                triple_xor_32_output_limb_1_col130.clone(),
            ],
            low_7_ms_bits_col149.clone(),
            high_14_ms_bits_col150.clone(),
            high_5_ms_bits_col151.clone(),
            new_state_2_id_col152.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        VerifyU32::evaluate(
            [
                (decode_blake_opcode_output_tmp_53f39_42_limb_2.clone() + M31_3.clone()),
                triple_xor_32_output_limb_0_col131.clone(),
                triple_xor_32_output_limb_1_col132.clone(),
            ],
            low_7_ms_bits_col153.clone(),
            high_14_ms_bits_col154.clone(),
            high_5_ms_bits_col155.clone(),
            new_state_3_id_col156.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        VerifyU32::evaluate(
            [
                (decode_blake_opcode_output_tmp_53f39_42_limb_2.clone() + M31_4.clone()),
                triple_xor_32_output_limb_0_col133.clone(),
                triple_xor_32_output_limb_1_col134.clone(),
            ],
            low_7_ms_bits_col157.clone(),
            high_14_ms_bits_col158.clone(),
            high_5_ms_bits_col159.clone(),
            new_state_4_id_col160.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        VerifyU32::evaluate(
            [
                (decode_blake_opcode_output_tmp_53f39_42_limb_2.clone() + M31_5.clone()),
                triple_xor_32_output_limb_0_col135.clone(),
                triple_xor_32_output_limb_1_col136.clone(),
            ],
            low_7_ms_bits_col161.clone(),
            high_14_ms_bits_col162.clone(),
            high_5_ms_bits_col163.clone(),
            new_state_5_id_col164.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        VerifyU32::evaluate(
            [
                (decode_blake_opcode_output_tmp_53f39_42_limb_2.clone() + M31_6.clone()),
                triple_xor_32_output_limb_0_col137.clone(),
                triple_xor_32_output_limb_1_col138.clone(),
            ],
            low_7_ms_bits_col165.clone(),
            high_14_ms_bits_col166.clone(),
            high_5_ms_bits_col167.clone(),
            new_state_6_id_col168.clone(),
            &self.range_check_7_2_5_lookup_elements,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        VerifyU32::evaluate(
            [
                (decode_blake_opcode_output_tmp_53f39_42_limb_2.clone() + M31_7.clone()),
                triple_xor_32_output_limb_0_col139.clone(),
                triple_xor_32_output_limb_1_col140.clone(),
            ],
            low_7_ms_bits_col169.clone(),
            high_14_ms_bits_col170.clone(),
            high_5_ms_bits_col171.clone(),
            new_state_7_id_col172.clone(),
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
                (input_ap_col1.clone() + ap_update_add_1_col9.clone()),
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
    use stwo::core::fields::qm31::QM31;
    use stwo_constraint_framework::expr::ExprEvaluator;

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
