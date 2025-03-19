use crate::prelude::constraint_eval::*;

pub struct Eval {
    pub claim: Claim,
    pub blake_round_lookup_elements: relations::BlakeRound,
    pub memory_address_to_id_lookup_elements: relations::MemoryAddressToId,
    pub memory_id_to_big_lookup_elements: relations::MemoryIdToBig,
    pub opcodes_lookup_elements: relations::Opcodes,
    pub range_check_7_2_5_lookup_elements: relations::RangeCheck_7_2_5,
    pub triple_xor_32_lookup_elements: relations::TripleXor32,
    pub verify_bitwise_xor_8_lookup_elements: relations::VerifyBitwiseXor_8,
    pub verify_instruction_lookup_elements: relations::VerifyInstruction,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Claim {
    pub log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; 171];
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
        let M31_127 = E::F::from(M31::from(127));
        let M31_128 = E::F::from(M31::from(128));
        let M31_14 = E::F::from(M31::from(14));
        let M31_15470 = E::F::from(M31::from(15470));
        let M31_16 = E::F::from(M31::from(16));
        let M31_2 = E::F::from(M31::from(2));
        let M31_23520 = E::F::from(M31::from(23520));
        let M31_256 = E::F::from(M31::from(256));
        let M31_262144 = E::F::from(M31::from(262144));
        let M31_26764 = E::F::from(M31::from(26764));
        let M31_27145 = E::F::from(M31::from(27145));
        let M31_3 = E::F::from(M31::from(3));
        let M31_32 = E::F::from(M31::from(32));
        let M31_32768 = E::F::from(M31::from(32768));
        let M31_39685 = E::F::from(M31::from(39685));
        let M31_4 = E::F::from(M31::from(4));
        let M31_42319 = E::F::from(M31::from(42319));
        let M31_44677 = E::F::from(M31::from(44677));
        let M31_47975 = E::F::from(M31::from(47975));
        let M31_5 = E::F::from(M31::from(5));
        let M31_512 = E::F::from(M31::from(512));
        let M31_52505 = E::F::from(M31::from(52505));
        let M31_55723 = E::F::from(M31::from(55723));
        let M31_57468 = E::F::from(M31::from(57468));
        let M31_58983 = E::F::from(M31::from(58983));
        let M31_6 = E::F::from(M31::from(6));
        let M31_62322 = E::F::from(M31::from(62322));
        let M31_62778 = E::F::from(M31::from(62778));
        let M31_64 = E::F::from(M31::from(64));
        let M31_7 = E::F::from(M31::from(7));
        let M31_8 = E::F::from(M31::from(8));
        let M31_8067 = E::F::from(M31::from(8067));
        let M31_81 = E::F::from(M31::from(81));
        let M31_82 = E::F::from(M31::from(82));
        let M31_9812 = E::F::from(M31::from(9812));
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
        let blake_round_output_limb_33_col120 = eval.next_trace_mask();
        let blake_round_output_limb_34_col121 = eval.next_trace_mask();
        let triple_xor_32_output_low_col122 = eval.next_trace_mask();
        let triple_xor_32_output_high_col123 = eval.next_trace_mask();
        let triple_xor_32_output_low_col124 = eval.next_trace_mask();
        let triple_xor_32_output_high_col125 = eval.next_trace_mask();
        let triple_xor_32_output_low_col126 = eval.next_trace_mask();
        let triple_xor_32_output_high_col127 = eval.next_trace_mask();
        let triple_xor_32_output_low_col128 = eval.next_trace_mask();
        let triple_xor_32_output_high_col129 = eval.next_trace_mask();
        let triple_xor_32_output_low_col130 = eval.next_trace_mask();
        let triple_xor_32_output_high_col131 = eval.next_trace_mask();
        let triple_xor_32_output_low_col132 = eval.next_trace_mask();
        let triple_xor_32_output_high_col133 = eval.next_trace_mask();
        let triple_xor_32_output_low_col134 = eval.next_trace_mask();
        let triple_xor_32_output_high_col135 = eval.next_trace_mask();
        let triple_xor_32_output_low_col136 = eval.next_trace_mask();
        let triple_xor_32_output_high_col137 = eval.next_trace_mask();
        let low_7_ms_bits_col138 = eval.next_trace_mask();
        let high_14_ms_bits_col139 = eval.next_trace_mask();
        let high_5_ms_bits_col140 = eval.next_trace_mask();
        let new_state_0_id_col141 = eval.next_trace_mask();
        let low_7_ms_bits_col142 = eval.next_trace_mask();
        let high_14_ms_bits_col143 = eval.next_trace_mask();
        let high_5_ms_bits_col144 = eval.next_trace_mask();
        let new_state_1_id_col145 = eval.next_trace_mask();
        let low_7_ms_bits_col146 = eval.next_trace_mask();
        let high_14_ms_bits_col147 = eval.next_trace_mask();
        let high_5_ms_bits_col148 = eval.next_trace_mask();
        let new_state_2_id_col149 = eval.next_trace_mask();
        let low_7_ms_bits_col150 = eval.next_trace_mask();
        let high_14_ms_bits_col151 = eval.next_trace_mask();
        let high_5_ms_bits_col152 = eval.next_trace_mask();
        let new_state_3_id_col153 = eval.next_trace_mask();
        let low_7_ms_bits_col154 = eval.next_trace_mask();
        let high_14_ms_bits_col155 = eval.next_trace_mask();
        let high_5_ms_bits_col156 = eval.next_trace_mask();
        let new_state_4_id_col157 = eval.next_trace_mask();
        let low_7_ms_bits_col158 = eval.next_trace_mask();
        let high_14_ms_bits_col159 = eval.next_trace_mask();
        let high_5_ms_bits_col160 = eval.next_trace_mask();
        let new_state_5_id_col161 = eval.next_trace_mask();
        let low_7_ms_bits_col162 = eval.next_trace_mask();
        let high_14_ms_bits_col163 = eval.next_trace_mask();
        let high_5_ms_bits_col164 = eval.next_trace_mask();
        let new_state_6_id_col165 = eval.next_trace_mask();
        let low_7_ms_bits_col166 = eval.next_trace_mask();
        let high_14_ms_bits_col167 = eval.next_trace_mask();
        let high_5_ms_bits_col168 = eval.next_trace_mask();
        let new_state_7_id_col169 = eval.next_trace_mask();
        let padding = eval.next_trace_mask();

        eval.add_constraint(padding.clone() * padding.clone() - padding.clone());

        // Decode Blake Opcode.

        // Decode Instruction.

        // Flag dst_base_fp is a bit.
        eval.add_constraint(
            (dst_base_fp_col6.clone() * (M31_1.clone() - dst_base_fp_col6.clone())),
        );
        // Flag op0_base_fp is a bit.
        eval.add_constraint(
            (op0_base_fp_col7.clone() * (M31_1.clone() - op0_base_fp_col7.clone())),
        );
        // Flag op1_base_fp is a bit.
        eval.add_constraint(
            (op1_base_fp_col8.clone() * (M31_1.clone() - op1_base_fp_col8.clone())),
        );
        // Flag op1_base_ap is a bit.
        eval.add_constraint(
            (op1_base_ap_col9.clone() * (M31_1.clone() - op1_base_ap_col9.clone())),
        );
        // Flag ap_update_add_1 is a bit.
        eval.add_constraint(
            (ap_update_add_1_col10.clone() * (M31_1.clone() - ap_update_add_1_col10.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.verify_instruction_lookup_elements,
            E::EF::one(),
            &[
                input_pc_col0.clone(),
                offset0_col3.clone(),
                offset1_col4.clone(),
                offset2_col5.clone(),
                ((((((M31_0.clone() + (dst_base_fp_col6.clone() * M31_8.clone()))
                    + (op0_base_fp_col7.clone() * M31_16.clone()))
                    + M31_0.clone())
                    + (op1_base_fp_col8.clone() * M31_64.clone()))
                    + (op1_base_ap_col9.clone() * M31_128.clone()))
                    + M31_0.clone()),
                ((((M31_0.clone() + (ap_update_add_1_col10.clone() * M31_32.clone()))
                    + M31_0.clone())
                    + M31_0.clone())
                    + M31_0.clone()),
                opcode_extension_col11.clone(),
            ],
        ));

        // Exactly one of op1_base_fp and op1_base_ap is 1.
        eval.add_constraint(
            ((op1_base_fp_col8.clone() + op1_base_ap_col9.clone()) - M31_1.clone()),
        );
        // OpcodeExtension is either Blake or BlakeFinalize.
        eval.add_constraint(
            ((opcode_extension_col11.clone() - M31_1.clone())
                * (opcode_extension_col11.clone() - M31_2.clone())),
        );
        // mem0_base.
        eval.add_constraint(
            (mem0_base_col12.clone()
                - ((op0_base_fp_col7.clone() * input_fp_col2.clone())
                    + ((M31_1.clone() - op0_base_fp_col7.clone()) * input_ap_col1.clone()))),
        );

        // Read Positive Num Bits 27.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (mem0_base_col12.clone() + (offset1_col4.clone() - M31_32768.clone())),
                op0_id_col13.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                op0_id_col13.clone(),
                op0_limb_0_col14.clone(),
                op0_limb_1_col15.clone(),
                op0_limb_2_col16.clone(),
            ],
        ));

        // mem1_base.
        eval.add_constraint(
            (mem1_base_col17.clone()
                - ((op1_base_fp_col8.clone() * input_fp_col2.clone())
                    + (op1_base_ap_col9.clone() * input_ap_col1.clone()))),
        );

        // Read Positive Num Bits 27.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (mem1_base_col17.clone() + (offset2_col5.clone() - M31_32768.clone())),
                op1_id_col18.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                op1_id_col18.clone(),
                op1_limb_0_col19.clone(),
                op1_limb_1_col20.clone(),
                op1_limb_2_col21.clone(),
            ],
        ));

        // Read Positive Num Bits 27.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[input_ap_col1.clone(), ap_id_col22.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                ap_id_col22.clone(),
                ap_limb_0_col23.clone(),
                ap_limb_1_col24.clone(),
                ap_limb_2_col25.clone(),
            ],
        ));

        // mem_dst_base.
        eval.add_constraint(
            (mem_dst_base_col26.clone()
                - ((dst_base_fp_col6.clone() * input_fp_col2.clone())
                    + ((M31_1.clone() - dst_base_fp_col6.clone()) * input_ap_col1.clone()))),
        );

        // Read Blake Word.

        // Verify Blake Word.

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_7_2_5_lookup_elements,
            E::EF::one(),
            &[
                low_7_ms_bits_col29.clone(),
                (high_16_bits_col28.clone() - (high_14_ms_bits_col30.clone() * M31_4.clone())),
                high_5_ms_bits_col31.clone(),
            ],
        ));

        // Mem Verify.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (mem_dst_base_col26.clone() + (offset0_col3.clone() - M31_32768.clone())),
                dst_id_col32.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                dst_id_col32.clone(),
                (low_16_bits_col27.clone() - (low_7_ms_bits_col29.clone() * M31_512.clone())),
                (low_7_ms_bits_col29.clone()
                    + ((high_16_bits_col28.clone()
                        - (high_14_ms_bits_col30.clone() * M31_4.clone()))
                        * M31_128.clone())),
                (high_14_ms_bits_col30.clone() - (high_5_ms_bits_col31.clone() * M31_512.clone())),
                high_5_ms_bits_col31.clone(),
            ],
        ));

        // Create Blake Round Input.

        // Read Blake Word.

        // Verify Blake Word.

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_7_2_5_lookup_elements,
            E::EF::one(),
            &[
                low_7_ms_bits_col35.clone(),
                (high_16_bits_col34.clone() - (high_14_ms_bits_col36.clone() * M31_4.clone())),
                high_5_ms_bits_col37.clone(),
            ],
        ));

        // Mem Verify.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (((op0_limb_0_col14.clone() + (op0_limb_1_col15.clone() * M31_512.clone()))
                    + (op0_limb_2_col16.clone() * M31_262144.clone()))
                    + M31_0.clone()),
                state_0_id_col38.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                state_0_id_col38.clone(),
                (low_16_bits_col33.clone() - (low_7_ms_bits_col35.clone() * M31_512.clone())),
                (low_7_ms_bits_col35.clone()
                    + ((high_16_bits_col34.clone()
                        - (high_14_ms_bits_col36.clone() * M31_4.clone()))
                        * M31_128.clone())),
                (high_14_ms_bits_col36.clone() - (high_5_ms_bits_col37.clone() * M31_512.clone())),
                high_5_ms_bits_col37.clone(),
            ],
        ));

        // Read Blake Word.

        // Verify Blake Word.

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_7_2_5_lookup_elements,
            E::EF::one(),
            &[
                low_7_ms_bits_col41.clone(),
                (high_16_bits_col40.clone() - (high_14_ms_bits_col42.clone() * M31_4.clone())),
                high_5_ms_bits_col43.clone(),
            ],
        ));

        // Mem Verify.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (((op0_limb_0_col14.clone() + (op0_limb_1_col15.clone() * M31_512.clone()))
                    + (op0_limb_2_col16.clone() * M31_262144.clone()))
                    + M31_1.clone()),
                state_1_id_col44.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                state_1_id_col44.clone(),
                (low_16_bits_col39.clone() - (low_7_ms_bits_col41.clone() * M31_512.clone())),
                (low_7_ms_bits_col41.clone()
                    + ((high_16_bits_col40.clone()
                        - (high_14_ms_bits_col42.clone() * M31_4.clone()))
                        * M31_128.clone())),
                (high_14_ms_bits_col42.clone() - (high_5_ms_bits_col43.clone() * M31_512.clone())),
                high_5_ms_bits_col43.clone(),
            ],
        ));

        // Read Blake Word.

        // Verify Blake Word.

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_7_2_5_lookup_elements,
            E::EF::one(),
            &[
                low_7_ms_bits_col47.clone(),
                (high_16_bits_col46.clone() - (high_14_ms_bits_col48.clone() * M31_4.clone())),
                high_5_ms_bits_col49.clone(),
            ],
        ));

        // Mem Verify.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (((op0_limb_0_col14.clone() + (op0_limb_1_col15.clone() * M31_512.clone()))
                    + (op0_limb_2_col16.clone() * M31_262144.clone()))
                    + M31_2.clone()),
                state_2_id_col50.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                state_2_id_col50.clone(),
                (low_16_bits_col45.clone() - (low_7_ms_bits_col47.clone() * M31_512.clone())),
                (low_7_ms_bits_col47.clone()
                    + ((high_16_bits_col46.clone()
                        - (high_14_ms_bits_col48.clone() * M31_4.clone()))
                        * M31_128.clone())),
                (high_14_ms_bits_col48.clone() - (high_5_ms_bits_col49.clone() * M31_512.clone())),
                high_5_ms_bits_col49.clone(),
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
                (((op0_limb_0_col14.clone() + (op0_limb_1_col15.clone() * M31_512.clone()))
                    + (op0_limb_2_col16.clone() * M31_262144.clone()))
                    + M31_3.clone()),
                state_3_id_col56.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                state_3_id_col56.clone(),
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
                (((op0_limb_0_col14.clone() + (op0_limb_1_col15.clone() * M31_512.clone()))
                    + (op0_limb_2_col16.clone() * M31_262144.clone()))
                    + M31_4.clone()),
                state_4_id_col62.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                state_4_id_col62.clone(),
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
                (((op0_limb_0_col14.clone() + (op0_limb_1_col15.clone() * M31_512.clone()))
                    + (op0_limb_2_col16.clone() * M31_262144.clone()))
                    + M31_5.clone()),
                state_5_id_col68.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                state_5_id_col68.clone(),
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
                (((op0_limb_0_col14.clone() + (op0_limb_1_col15.clone() * M31_512.clone()))
                    + (op0_limb_2_col16.clone() * M31_262144.clone()))
                    + M31_6.clone()),
                state_6_id_col74.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                state_6_id_col74.clone(),
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
                (((op0_limb_0_col14.clone() + (op0_limb_1_col15.clone() * M31_512.clone()))
                    + (op0_limb_2_col16.clone() * M31_262144.clone()))
                    + M31_7.clone()),
                state_7_id_col80.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                state_7_id_col80.clone(),
                (low_16_bits_col75.clone() - (low_7_ms_bits_col77.clone() * M31_512.clone())),
                (low_7_ms_bits_col77.clone()
                    + ((high_16_bits_col76.clone()
                        - (high_14_ms_bits_col78.clone() * M31_4.clone()))
                        * M31_128.clone())),
                (high_14_ms_bits_col78.clone() - (high_5_ms_bits_col79.clone() * M31_512.clone())),
                high_5_ms_bits_col79.clone(),
            ],
        ));

        // Bitwise Xor Num Bits 8.

        eval.add_to_relation(RelationEntry::new(
            &self.verify_bitwise_xor_8_lookup_elements,
            E::EF::one(),
            &[
                (low_16_bits_col27.clone() - (ms_8_bits_col81.clone() * M31_256.clone())),
                M31_127.clone(),
                xor_col83.clone(),
            ],
        ));

        // Bitwise Xor Num Bits 8.

        eval.add_to_relation(RelationEntry::new(
            &self.verify_bitwise_xor_8_lookup_elements,
            E::EF::one(),
            &[ms_8_bits_col81.clone(), M31_82.clone(), xor_col84.clone()],
        ));

        // Bitwise Xor Num Bits 8.

        eval.add_to_relation(RelationEntry::new(
            &self.verify_bitwise_xor_8_lookup_elements,
            E::EF::one(),
            &[
                (high_16_bits_col28.clone() - (ms_8_bits_col82.clone() * M31_256.clone())),
                M31_14.clone(),
                xor_col85.clone(),
            ],
        ));

        // Bitwise Xor Num Bits 8.

        eval.add_to_relation(RelationEntry::new(
            &self.verify_bitwise_xor_8_lookup_elements,
            E::EF::one(),
            &[ms_8_bits_col82.clone(), M31_81.clone(), xor_col86.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.blake_round_lookup_elements,
            -E::EF::one(),
            &[
                (seq.clone() * M31_1.clone()),
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
                (xor_col83.clone() + (xor_col84.clone() * M31_256.clone())),
                (xor_col85.clone() + (xor_col86.clone() * M31_256.clone())),
                M31_26764.clone(),
                M31_39685.clone(),
                (((opcode_extension_col11.clone() - M31_1.clone()) * M31_9812.clone())
                    + ((M31_1.clone() - (opcode_extension_col11.clone() - M31_1.clone()))
                        * M31_55723.clone())),
                (((opcode_extension_col11.clone() - M31_1.clone()) * M31_57468.clone())
                    + ((M31_1.clone() - (opcode_extension_col11.clone() - M31_1.clone()))
                        * M31_8067.clone())),
                M31_52505.clone(),
                M31_23520.clone(),
                ((op1_limb_0_col19.clone() + (op1_limb_1_col20.clone() * M31_512.clone()))
                    + (op1_limb_2_col21.clone() * M31_262144.clone())),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.blake_round_lookup_elements,
            E::EF::one(),
            &[
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
                blake_round_output_limb_33_col120.clone(),
                blake_round_output_limb_34_col121.clone(),
            ],
        ));

        // Create Blake Output.

        eval.add_to_relation(RelationEntry::new(
            &self.triple_xor_32_lookup_elements,
            E::EF::one(),
            &[
                blake_round_output_limb_2_col89.clone(),
                blake_round_output_limb_3_col90.clone(),
                blake_round_output_limb_18_col105.clone(),
                blake_round_output_limb_19_col106.clone(),
                low_16_bits_col33.clone(),
                high_16_bits_col34.clone(),
                triple_xor_32_output_low_col122.clone(),
                triple_xor_32_output_high_col123.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.triple_xor_32_lookup_elements,
            E::EF::one(),
            &[
                blake_round_output_limb_4_col91.clone(),
                blake_round_output_limb_5_col92.clone(),
                blake_round_output_limb_20_col107.clone(),
                blake_round_output_limb_21_col108.clone(),
                low_16_bits_col39.clone(),
                high_16_bits_col40.clone(),
                triple_xor_32_output_low_col124.clone(),
                triple_xor_32_output_high_col125.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.triple_xor_32_lookup_elements,
            E::EF::one(),
            &[
                blake_round_output_limb_6_col93.clone(),
                blake_round_output_limb_7_col94.clone(),
                blake_round_output_limb_22_col109.clone(),
                blake_round_output_limb_23_col110.clone(),
                low_16_bits_col45.clone(),
                high_16_bits_col46.clone(),
                triple_xor_32_output_low_col126.clone(),
                triple_xor_32_output_high_col127.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.triple_xor_32_lookup_elements,
            E::EF::one(),
            &[
                blake_round_output_limb_8_col95.clone(),
                blake_round_output_limb_9_col96.clone(),
                blake_round_output_limb_24_col111.clone(),
                blake_round_output_limb_25_col112.clone(),
                low_16_bits_col51.clone(),
                high_16_bits_col52.clone(),
                triple_xor_32_output_low_col128.clone(),
                triple_xor_32_output_high_col129.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.triple_xor_32_lookup_elements,
            E::EF::one(),
            &[
                blake_round_output_limb_10_col97.clone(),
                blake_round_output_limb_11_col98.clone(),
                blake_round_output_limb_26_col113.clone(),
                blake_round_output_limb_27_col114.clone(),
                low_16_bits_col57.clone(),
                high_16_bits_col58.clone(),
                triple_xor_32_output_low_col130.clone(),
                triple_xor_32_output_high_col131.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.triple_xor_32_lookup_elements,
            E::EF::one(),
            &[
                blake_round_output_limb_12_col99.clone(),
                blake_round_output_limb_13_col100.clone(),
                blake_round_output_limb_28_col115.clone(),
                blake_round_output_limb_29_col116.clone(),
                low_16_bits_col63.clone(),
                high_16_bits_col64.clone(),
                triple_xor_32_output_low_col132.clone(),
                triple_xor_32_output_high_col133.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.triple_xor_32_lookup_elements,
            E::EF::one(),
            &[
                blake_round_output_limb_14_col101.clone(),
                blake_round_output_limb_15_col102.clone(),
                blake_round_output_limb_30_col117.clone(),
                blake_round_output_limb_31_col118.clone(),
                low_16_bits_col69.clone(),
                high_16_bits_col70.clone(),
                triple_xor_32_output_low_col134.clone(),
                triple_xor_32_output_high_col135.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.triple_xor_32_lookup_elements,
            E::EF::one(),
            &[
                blake_round_output_limb_16_col103.clone(),
                blake_round_output_limb_17_col104.clone(),
                blake_round_output_limb_32_col119.clone(),
                blake_round_output_limb_33_col120.clone(),
                low_16_bits_col75.clone(),
                high_16_bits_col76.clone(),
                triple_xor_32_output_low_col136.clone(),
                triple_xor_32_output_high_col137.clone(),
            ],
        ));

        // Verify Blake Word.

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_7_2_5_lookup_elements,
            E::EF::one(),
            &[
                low_7_ms_bits_col138.clone(),
                (triple_xor_32_output_high_col123.clone()
                    - (high_14_ms_bits_col139.clone() * M31_4.clone())),
                high_5_ms_bits_col140.clone(),
            ],
        ));

        // Mem Verify.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (((ap_limb_0_col23.clone() + (ap_limb_1_col24.clone() * M31_512.clone()))
                    + (ap_limb_2_col25.clone() * M31_262144.clone()))
                    + M31_0.clone()),
                new_state_0_id_col141.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                new_state_0_id_col141.clone(),
                (triple_xor_32_output_low_col122.clone()
                    - (low_7_ms_bits_col138.clone() * M31_512.clone())),
                (low_7_ms_bits_col138.clone()
                    + ((triple_xor_32_output_high_col123.clone()
                        - (high_14_ms_bits_col139.clone() * M31_4.clone()))
                        * M31_128.clone())),
                (high_14_ms_bits_col139.clone()
                    - (high_5_ms_bits_col140.clone() * M31_512.clone())),
                high_5_ms_bits_col140.clone(),
            ],
        ));

        // Verify Blake Word.

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_7_2_5_lookup_elements,
            E::EF::one(),
            &[
                low_7_ms_bits_col142.clone(),
                (triple_xor_32_output_high_col125.clone()
                    - (high_14_ms_bits_col143.clone() * M31_4.clone())),
                high_5_ms_bits_col144.clone(),
            ],
        ));

        // Mem Verify.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (((ap_limb_0_col23.clone() + (ap_limb_1_col24.clone() * M31_512.clone()))
                    + (ap_limb_2_col25.clone() * M31_262144.clone()))
                    + M31_1.clone()),
                new_state_1_id_col145.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                new_state_1_id_col145.clone(),
                (triple_xor_32_output_low_col124.clone()
                    - (low_7_ms_bits_col142.clone() * M31_512.clone())),
                (low_7_ms_bits_col142.clone()
                    + ((triple_xor_32_output_high_col125.clone()
                        - (high_14_ms_bits_col143.clone() * M31_4.clone()))
                        * M31_128.clone())),
                (high_14_ms_bits_col143.clone()
                    - (high_5_ms_bits_col144.clone() * M31_512.clone())),
                high_5_ms_bits_col144.clone(),
            ],
        ));

        // Verify Blake Word.

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_7_2_5_lookup_elements,
            E::EF::one(),
            &[
                low_7_ms_bits_col146.clone(),
                (triple_xor_32_output_high_col127.clone()
                    - (high_14_ms_bits_col147.clone() * M31_4.clone())),
                high_5_ms_bits_col148.clone(),
            ],
        ));

        // Mem Verify.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (((ap_limb_0_col23.clone() + (ap_limb_1_col24.clone() * M31_512.clone()))
                    + (ap_limb_2_col25.clone() * M31_262144.clone()))
                    + M31_2.clone()),
                new_state_2_id_col149.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                new_state_2_id_col149.clone(),
                (triple_xor_32_output_low_col126.clone()
                    - (low_7_ms_bits_col146.clone() * M31_512.clone())),
                (low_7_ms_bits_col146.clone()
                    + ((triple_xor_32_output_high_col127.clone()
                        - (high_14_ms_bits_col147.clone() * M31_4.clone()))
                        * M31_128.clone())),
                (high_14_ms_bits_col147.clone()
                    - (high_5_ms_bits_col148.clone() * M31_512.clone())),
                high_5_ms_bits_col148.clone(),
            ],
        ));

        // Verify Blake Word.

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_7_2_5_lookup_elements,
            E::EF::one(),
            &[
                low_7_ms_bits_col150.clone(),
                (triple_xor_32_output_high_col129.clone()
                    - (high_14_ms_bits_col151.clone() * M31_4.clone())),
                high_5_ms_bits_col152.clone(),
            ],
        ));

        // Mem Verify.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (((ap_limb_0_col23.clone() + (ap_limb_1_col24.clone() * M31_512.clone()))
                    + (ap_limb_2_col25.clone() * M31_262144.clone()))
                    + M31_3.clone()),
                new_state_3_id_col153.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                new_state_3_id_col153.clone(),
                (triple_xor_32_output_low_col128.clone()
                    - (low_7_ms_bits_col150.clone() * M31_512.clone())),
                (low_7_ms_bits_col150.clone()
                    + ((triple_xor_32_output_high_col129.clone()
                        - (high_14_ms_bits_col151.clone() * M31_4.clone()))
                        * M31_128.clone())),
                (high_14_ms_bits_col151.clone()
                    - (high_5_ms_bits_col152.clone() * M31_512.clone())),
                high_5_ms_bits_col152.clone(),
            ],
        ));

        // Verify Blake Word.

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_7_2_5_lookup_elements,
            E::EF::one(),
            &[
                low_7_ms_bits_col154.clone(),
                (triple_xor_32_output_high_col131.clone()
                    - (high_14_ms_bits_col155.clone() * M31_4.clone())),
                high_5_ms_bits_col156.clone(),
            ],
        ));

        // Mem Verify.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (((ap_limb_0_col23.clone() + (ap_limb_1_col24.clone() * M31_512.clone()))
                    + (ap_limb_2_col25.clone() * M31_262144.clone()))
                    + M31_4.clone()),
                new_state_4_id_col157.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                new_state_4_id_col157.clone(),
                (triple_xor_32_output_low_col130.clone()
                    - (low_7_ms_bits_col154.clone() * M31_512.clone())),
                (low_7_ms_bits_col154.clone()
                    + ((triple_xor_32_output_high_col131.clone()
                        - (high_14_ms_bits_col155.clone() * M31_4.clone()))
                        * M31_128.clone())),
                (high_14_ms_bits_col155.clone()
                    - (high_5_ms_bits_col156.clone() * M31_512.clone())),
                high_5_ms_bits_col156.clone(),
            ],
        ));

        // Verify Blake Word.

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_7_2_5_lookup_elements,
            E::EF::one(),
            &[
                low_7_ms_bits_col158.clone(),
                (triple_xor_32_output_high_col133.clone()
                    - (high_14_ms_bits_col159.clone() * M31_4.clone())),
                high_5_ms_bits_col160.clone(),
            ],
        ));

        // Mem Verify.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (((ap_limb_0_col23.clone() + (ap_limb_1_col24.clone() * M31_512.clone()))
                    + (ap_limb_2_col25.clone() * M31_262144.clone()))
                    + M31_5.clone()),
                new_state_5_id_col161.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                new_state_5_id_col161.clone(),
                (triple_xor_32_output_low_col132.clone()
                    - (low_7_ms_bits_col158.clone() * M31_512.clone())),
                (low_7_ms_bits_col158.clone()
                    + ((triple_xor_32_output_high_col133.clone()
                        - (high_14_ms_bits_col159.clone() * M31_4.clone()))
                        * M31_128.clone())),
                (high_14_ms_bits_col159.clone()
                    - (high_5_ms_bits_col160.clone() * M31_512.clone())),
                high_5_ms_bits_col160.clone(),
            ],
        ));

        // Verify Blake Word.

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_7_2_5_lookup_elements,
            E::EF::one(),
            &[
                low_7_ms_bits_col162.clone(),
                (triple_xor_32_output_high_col135.clone()
                    - (high_14_ms_bits_col163.clone() * M31_4.clone())),
                high_5_ms_bits_col164.clone(),
            ],
        ));

        // Mem Verify.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (((ap_limb_0_col23.clone() + (ap_limb_1_col24.clone() * M31_512.clone()))
                    + (ap_limb_2_col25.clone() * M31_262144.clone()))
                    + M31_6.clone()),
                new_state_6_id_col165.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                new_state_6_id_col165.clone(),
                (triple_xor_32_output_low_col134.clone()
                    - (low_7_ms_bits_col162.clone() * M31_512.clone())),
                (low_7_ms_bits_col162.clone()
                    + ((triple_xor_32_output_high_col135.clone()
                        - (high_14_ms_bits_col163.clone() * M31_4.clone()))
                        * M31_128.clone())),
                (high_14_ms_bits_col163.clone()
                    - (high_5_ms_bits_col164.clone() * M31_512.clone())),
                high_5_ms_bits_col164.clone(),
            ],
        ));

        // Verify Blake Word.

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_7_2_5_lookup_elements,
            E::EF::one(),
            &[
                low_7_ms_bits_col166.clone(),
                (triple_xor_32_output_high_col137.clone()
                    - (high_14_ms_bits_col167.clone() * M31_4.clone())),
                high_5_ms_bits_col168.clone(),
            ],
        ));

        // Mem Verify.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (((ap_limb_0_col23.clone() + (ap_limb_1_col24.clone() * M31_512.clone()))
                    + (ap_limb_2_col25.clone() * M31_262144.clone()))
                    + M31_7.clone()),
                new_state_7_id_col169.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                new_state_7_id_col169.clone(),
                (triple_xor_32_output_low_col136.clone()
                    - (low_7_ms_bits_col166.clone() * M31_512.clone())),
                (low_7_ms_bits_col166.clone()
                    + ((triple_xor_32_output_high_col137.clone()
                        - (high_14_ms_bits_col167.clone() * M31_4.clone()))
                        * M31_128.clone())),
                (high_14_ms_bits_col167.clone()
                    - (high_5_ms_bits_col168.clone() * M31_512.clone())),
                high_5_ms_bits_col168.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.opcodes_lookup_elements,
            E::EF::from(padding.clone()),
            &[
                input_pc_col0.clone(),
                input_ap_col1.clone(),
                input_fp_col2.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.opcodes_lookup_elements,
            -E::EF::from(padding.clone()),
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
