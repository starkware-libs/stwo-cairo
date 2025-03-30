use crate::components::prelude::constraint_eval::*;

pub const N_TRACE_COLUMNS: usize = 126;

pub struct Eval {
    pub claim: Claim,
    pub memory_address_to_id_lookup_elements: relations::MemoryAddressToId,
    pub memory_id_to_big_lookup_elements: relations::MemoryIdToBig,
    pub opcodes_lookup_elements: relations::Opcodes,
    pub range_check_19_lookup_elements: relations::RangeCheck_19,
    pub verify_instruction_lookup_elements: relations::VerifyInstruction,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Claim {
    pub log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 19];
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
        let M31_131072 = E::F::from(M31::from(131072));
        let M31_136 = E::F::from(M31::from(136));
        let M31_16 = E::F::from(M31::from(16));
        let M31_2 = E::F::from(M31::from(2));
        let M31_256 = E::F::from(M31::from(256));
        let M31_262144 = E::F::from(M31::from(262144));
        let M31_32 = E::F::from(M31::from(32));
        let M31_32768 = E::F::from(M31::from(32768));
        let M31_32769 = E::F::from(M31::from(32769));
        let M31_4 = E::F::from(M31::from(4));
        let M31_512 = E::F::from(M31::from(512));
        let M31_64 = E::F::from(M31::from(64));
        let M31_8 = E::F::from(M31::from(8));
        let input_pc_col0 = eval.next_trace_mask();
        let input_ap_col1 = eval.next_trace_mask();
        let input_fp_col2 = eval.next_trace_mask();
        let offset0_col3 = eval.next_trace_mask();
        let offset1_col4 = eval.next_trace_mask();
        let dst_base_fp_col5 = eval.next_trace_mask();
        let op0_base_fp_col6 = eval.next_trace_mask();
        let ap_update_add_1_col7 = eval.next_trace_mask();
        let mem_dst_base_col8 = eval.next_trace_mask();
        let mem0_base_col9 = eval.next_trace_mask();
        let dst_id_col10 = eval.next_trace_mask();
        let dst_limb_0_col11 = eval.next_trace_mask();
        let dst_limb_1_col12 = eval.next_trace_mask();
        let dst_limb_2_col13 = eval.next_trace_mask();
        let dst_limb_3_col14 = eval.next_trace_mask();
        let dst_limb_4_col15 = eval.next_trace_mask();
        let dst_limb_5_col16 = eval.next_trace_mask();
        let dst_limb_6_col17 = eval.next_trace_mask();
        let dst_limb_7_col18 = eval.next_trace_mask();
        let dst_limb_8_col19 = eval.next_trace_mask();
        let dst_limb_9_col20 = eval.next_trace_mask();
        let dst_limb_10_col21 = eval.next_trace_mask();
        let dst_limb_11_col22 = eval.next_trace_mask();
        let dst_limb_12_col23 = eval.next_trace_mask();
        let dst_limb_13_col24 = eval.next_trace_mask();
        let dst_limb_14_col25 = eval.next_trace_mask();
        let dst_limb_15_col26 = eval.next_trace_mask();
        let dst_limb_16_col27 = eval.next_trace_mask();
        let dst_limb_17_col28 = eval.next_trace_mask();
        let dst_limb_18_col29 = eval.next_trace_mask();
        let dst_limb_19_col30 = eval.next_trace_mask();
        let dst_limb_20_col31 = eval.next_trace_mask();
        let dst_limb_21_col32 = eval.next_trace_mask();
        let dst_limb_22_col33 = eval.next_trace_mask();
        let dst_limb_23_col34 = eval.next_trace_mask();
        let dst_limb_24_col35 = eval.next_trace_mask();
        let dst_limb_25_col36 = eval.next_trace_mask();
        let dst_limb_26_col37 = eval.next_trace_mask();
        let dst_limb_27_col38 = eval.next_trace_mask();
        let op0_id_col39 = eval.next_trace_mask();
        let op0_limb_0_col40 = eval.next_trace_mask();
        let op0_limb_1_col41 = eval.next_trace_mask();
        let op0_limb_2_col42 = eval.next_trace_mask();
        let op0_limb_3_col43 = eval.next_trace_mask();
        let op0_limb_4_col44 = eval.next_trace_mask();
        let op0_limb_5_col45 = eval.next_trace_mask();
        let op0_limb_6_col46 = eval.next_trace_mask();
        let op0_limb_7_col47 = eval.next_trace_mask();
        let op0_limb_8_col48 = eval.next_trace_mask();
        let op0_limb_9_col49 = eval.next_trace_mask();
        let op0_limb_10_col50 = eval.next_trace_mask();
        let op0_limb_11_col51 = eval.next_trace_mask();
        let op0_limb_12_col52 = eval.next_trace_mask();
        let op0_limb_13_col53 = eval.next_trace_mask();
        let op0_limb_14_col54 = eval.next_trace_mask();
        let op0_limb_15_col55 = eval.next_trace_mask();
        let op0_limb_16_col56 = eval.next_trace_mask();
        let op0_limb_17_col57 = eval.next_trace_mask();
        let op0_limb_18_col58 = eval.next_trace_mask();
        let op0_limb_19_col59 = eval.next_trace_mask();
        let op0_limb_20_col60 = eval.next_trace_mask();
        let op0_limb_21_col61 = eval.next_trace_mask();
        let op0_limb_22_col62 = eval.next_trace_mask();
        let op0_limb_23_col63 = eval.next_trace_mask();
        let op0_limb_24_col64 = eval.next_trace_mask();
        let op0_limb_25_col65 = eval.next_trace_mask();
        let op0_limb_26_col66 = eval.next_trace_mask();
        let op0_limb_27_col67 = eval.next_trace_mask();
        let op1_id_col68 = eval.next_trace_mask();
        let op1_limb_0_col69 = eval.next_trace_mask();
        let op1_limb_1_col70 = eval.next_trace_mask();
        let op1_limb_2_col71 = eval.next_trace_mask();
        let op1_limb_3_col72 = eval.next_trace_mask();
        let op1_limb_4_col73 = eval.next_trace_mask();
        let op1_limb_5_col74 = eval.next_trace_mask();
        let op1_limb_6_col75 = eval.next_trace_mask();
        let op1_limb_7_col76 = eval.next_trace_mask();
        let op1_limb_8_col77 = eval.next_trace_mask();
        let op1_limb_9_col78 = eval.next_trace_mask();
        let op1_limb_10_col79 = eval.next_trace_mask();
        let op1_limb_11_col80 = eval.next_trace_mask();
        let op1_limb_12_col81 = eval.next_trace_mask();
        let op1_limb_13_col82 = eval.next_trace_mask();
        let op1_limb_14_col83 = eval.next_trace_mask();
        let op1_limb_15_col84 = eval.next_trace_mask();
        let op1_limb_16_col85 = eval.next_trace_mask();
        let op1_limb_17_col86 = eval.next_trace_mask();
        let op1_limb_18_col87 = eval.next_trace_mask();
        let op1_limb_19_col88 = eval.next_trace_mask();
        let op1_limb_20_col89 = eval.next_trace_mask();
        let op1_limb_21_col90 = eval.next_trace_mask();
        let op1_limb_22_col91 = eval.next_trace_mask();
        let op1_limb_23_col92 = eval.next_trace_mask();
        let op1_limb_24_col93 = eval.next_trace_mask();
        let op1_limb_25_col94 = eval.next_trace_mask();
        let op1_limb_26_col95 = eval.next_trace_mask();
        let op1_limb_27_col96 = eval.next_trace_mask();
        let k_col97 = eval.next_trace_mask();
        let carry_0_col98 = eval.next_trace_mask();
        let carry_1_col99 = eval.next_trace_mask();
        let carry_2_col100 = eval.next_trace_mask();
        let carry_3_col101 = eval.next_trace_mask();
        let carry_4_col102 = eval.next_trace_mask();
        let carry_5_col103 = eval.next_trace_mask();
        let carry_6_col104 = eval.next_trace_mask();
        let carry_7_col105 = eval.next_trace_mask();
        let carry_8_col106 = eval.next_trace_mask();
        let carry_9_col107 = eval.next_trace_mask();
        let carry_10_col108 = eval.next_trace_mask();
        let carry_11_col109 = eval.next_trace_mask();
        let carry_12_col110 = eval.next_trace_mask();
        let carry_13_col111 = eval.next_trace_mask();
        let carry_14_col112 = eval.next_trace_mask();
        let carry_15_col113 = eval.next_trace_mask();
        let carry_16_col114 = eval.next_trace_mask();
        let carry_17_col115 = eval.next_trace_mask();
        let carry_18_col116 = eval.next_trace_mask();
        let carry_19_col117 = eval.next_trace_mask();
        let carry_20_col118 = eval.next_trace_mask();
        let carry_21_col119 = eval.next_trace_mask();
        let carry_22_col120 = eval.next_trace_mask();
        let carry_23_col121 = eval.next_trace_mask();
        let carry_24_col122 = eval.next_trace_mask();
        let carry_25_col123 = eval.next_trace_mask();
        let carry_26_col124 = eval.next_trace_mask();
        let padding = eval.next_trace_mask();

        eval.add_constraint(padding.clone() * padding.clone() - padding.clone());

        // Decode Instruction.

        // Flag dst_base_fp is a bit.
        eval.add_constraint(
            (dst_base_fp_col5.clone() * (M31_1.clone() - dst_base_fp_col5.clone())),
        );
        // Flag op0_base_fp is a bit.
        eval.add_constraint(
            (op0_base_fp_col6.clone() * (M31_1.clone() - op0_base_fp_col6.clone())),
        );
        // Flag ap_update_add_1 is a bit.
        eval.add_constraint(
            (ap_update_add_1_col7.clone() * (M31_1.clone() - ap_update_add_1_col7.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.verify_instruction_lookup_elements,
            E::EF::one(),
            &[
                input_pc_col0.clone(),
                offset0_col3.clone(),
                offset1_col4.clone(),
                M31_32769.clone(),
                (((dst_base_fp_col5.clone() * M31_8.clone())
                    + (op0_base_fp_col6.clone() * M31_16.clone()))
                    + M31_32.clone()),
                ((M31_1.clone() + (ap_update_add_1_col7.clone() * M31_32.clone()))
                    + M31_256.clone()),
            ],
        ));

        // mem_dst_base.
        eval.add_constraint(
            (mem_dst_base_col8.clone()
                - ((dst_base_fp_col5.clone() * input_fp_col2.clone())
                    + ((M31_1.clone() - dst_base_fp_col5.clone()) * input_ap_col1.clone()))),
        );
        // mem0_base.
        eval.add_constraint(
            (mem0_base_col9.clone()
                - ((op0_base_fp_col6.clone() * input_fp_col2.clone())
                    + ((M31_1.clone() - op0_base_fp_col6.clone()) * input_ap_col1.clone()))),
        );

        // Read Positive Num Bits 252.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (mem_dst_base_col8.clone() + (offset0_col3.clone() - M31_32768.clone())),
                dst_id_col10.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                dst_id_col10.clone(),
                dst_limb_0_col11.clone(),
                dst_limb_1_col12.clone(),
                dst_limb_2_col13.clone(),
                dst_limb_3_col14.clone(),
                dst_limb_4_col15.clone(),
                dst_limb_5_col16.clone(),
                dst_limb_6_col17.clone(),
                dst_limb_7_col18.clone(),
                dst_limb_8_col19.clone(),
                dst_limb_9_col20.clone(),
                dst_limb_10_col21.clone(),
                dst_limb_11_col22.clone(),
                dst_limb_12_col23.clone(),
                dst_limb_13_col24.clone(),
                dst_limb_14_col25.clone(),
                dst_limb_15_col26.clone(),
                dst_limb_16_col27.clone(),
                dst_limb_17_col28.clone(),
                dst_limb_18_col29.clone(),
                dst_limb_19_col30.clone(),
                dst_limb_20_col31.clone(),
                dst_limb_21_col32.clone(),
                dst_limb_22_col33.clone(),
                dst_limb_23_col34.clone(),
                dst_limb_24_col35.clone(),
                dst_limb_25_col36.clone(),
                dst_limb_26_col37.clone(),
                dst_limb_27_col38.clone(),
            ],
        ));

        // Read Positive Num Bits 252.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (mem0_base_col9.clone() + (offset1_col4.clone() - M31_32768.clone())),
                op0_id_col39.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                op0_id_col39.clone(),
                op0_limb_0_col40.clone(),
                op0_limb_1_col41.clone(),
                op0_limb_2_col42.clone(),
                op0_limb_3_col43.clone(),
                op0_limb_4_col44.clone(),
                op0_limb_5_col45.clone(),
                op0_limb_6_col46.clone(),
                op0_limb_7_col47.clone(),
                op0_limb_8_col48.clone(),
                op0_limb_9_col49.clone(),
                op0_limb_10_col50.clone(),
                op0_limb_11_col51.clone(),
                op0_limb_12_col52.clone(),
                op0_limb_13_col53.clone(),
                op0_limb_14_col54.clone(),
                op0_limb_15_col55.clone(),
                op0_limb_16_col56.clone(),
                op0_limb_17_col57.clone(),
                op0_limb_18_col58.clone(),
                op0_limb_19_col59.clone(),
                op0_limb_20_col60.clone(),
                op0_limb_21_col61.clone(),
                op0_limb_22_col62.clone(),
                op0_limb_23_col63.clone(),
                op0_limb_24_col64.clone(),
                op0_limb_25_col65.clone(),
                op0_limb_26_col66.clone(),
                op0_limb_27_col67.clone(),
            ],
        ));

        // Read Positive Num Bits 252.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (input_pc_col0.clone() + M31_1.clone()),
                op1_id_col68.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                op1_id_col68.clone(),
                op1_limb_0_col69.clone(),
                op1_limb_1_col70.clone(),
                op1_limb_2_col71.clone(),
                op1_limb_3_col72.clone(),
                op1_limb_4_col73.clone(),
                op1_limb_5_col74.clone(),
                op1_limb_6_col75.clone(),
                op1_limb_7_col76.clone(),
                op1_limb_8_col77.clone(),
                op1_limb_9_col78.clone(),
                op1_limb_10_col79.clone(),
                op1_limb_11_col80.clone(),
                op1_limb_12_col81.clone(),
                op1_limb_13_col82.clone(),
                op1_limb_14_col83.clone(),
                op1_limb_15_col84.clone(),
                op1_limb_16_col85.clone(),
                op1_limb_17_col86.clone(),
                op1_limb_18_col87.clone(),
                op1_limb_19_col88.clone(),
                op1_limb_20_col89.clone(),
                op1_limb_21_col90.clone(),
                op1_limb_22_col91.clone(),
                op1_limb_23_col92.clone(),
                op1_limb_24_col93.clone(),
                op1_limb_25_col94.clone(),
                op1_limb_26_col95.clone(),
                op1_limb_27_col96.clone(),
            ],
        ));

        // Verify Mul 252.

        // Double Karatsuba N 7 Limb Max Bound 511.

        // Single Karatsuba N 7.

        let z0_tmp_48d52_13_limb_0 =
            eval.add_intermediate((op0_limb_0_col40.clone() * op1_limb_0_col69.clone()));
        let z0_tmp_48d52_13_limb_1 = eval.add_intermediate(
            ((op0_limb_0_col40.clone() * op1_limb_1_col70.clone())
                + (op0_limb_1_col41.clone() * op1_limb_0_col69.clone())),
        );
        let z0_tmp_48d52_13_limb_2 = eval.add_intermediate(
            (((op0_limb_0_col40.clone() * op1_limb_2_col71.clone())
                + (op0_limb_1_col41.clone() * op1_limb_1_col70.clone()))
                + (op0_limb_2_col42.clone() * op1_limb_0_col69.clone())),
        );
        let z0_tmp_48d52_13_limb_3 = eval.add_intermediate(
            ((((op0_limb_0_col40.clone() * op1_limb_3_col72.clone())
                + (op0_limb_1_col41.clone() * op1_limb_2_col71.clone()))
                + (op0_limb_2_col42.clone() * op1_limb_1_col70.clone()))
                + (op0_limb_3_col43.clone() * op1_limb_0_col69.clone())),
        );
        let z0_tmp_48d52_13_limb_4 = eval.add_intermediate(
            (((((op0_limb_0_col40.clone() * op1_limb_4_col73.clone())
                + (op0_limb_1_col41.clone() * op1_limb_3_col72.clone()))
                + (op0_limb_2_col42.clone() * op1_limb_2_col71.clone()))
                + (op0_limb_3_col43.clone() * op1_limb_1_col70.clone()))
                + (op0_limb_4_col44.clone() * op1_limb_0_col69.clone())),
        );
        let z0_tmp_48d52_13_limb_5 = eval.add_intermediate(
            ((((((op0_limb_0_col40.clone() * op1_limb_5_col74.clone())
                + (op0_limb_1_col41.clone() * op1_limb_4_col73.clone()))
                + (op0_limb_2_col42.clone() * op1_limb_3_col72.clone()))
                + (op0_limb_3_col43.clone() * op1_limb_2_col71.clone()))
                + (op0_limb_4_col44.clone() * op1_limb_1_col70.clone()))
                + (op0_limb_5_col45.clone() * op1_limb_0_col69.clone())),
        );
        let z0_tmp_48d52_13_limb_6 = eval.add_intermediate(
            (((((((op0_limb_0_col40.clone() * op1_limb_6_col75.clone())
                + (op0_limb_1_col41.clone() * op1_limb_5_col74.clone()))
                + (op0_limb_2_col42.clone() * op1_limb_4_col73.clone()))
                + (op0_limb_3_col43.clone() * op1_limb_3_col72.clone()))
                + (op0_limb_4_col44.clone() * op1_limb_2_col71.clone()))
                + (op0_limb_5_col45.clone() * op1_limb_1_col70.clone()))
                + (op0_limb_6_col46.clone() * op1_limb_0_col69.clone())),
        );
        let z0_tmp_48d52_13_limb_7 = eval.add_intermediate(
            ((((((op0_limb_1_col41.clone() * op1_limb_6_col75.clone())
                + (op0_limb_2_col42.clone() * op1_limb_5_col74.clone()))
                + (op0_limb_3_col43.clone() * op1_limb_4_col73.clone()))
                + (op0_limb_4_col44.clone() * op1_limb_3_col72.clone()))
                + (op0_limb_5_col45.clone() * op1_limb_2_col71.clone()))
                + (op0_limb_6_col46.clone() * op1_limb_1_col70.clone())),
        );
        let z0_tmp_48d52_13_limb_8 = eval.add_intermediate(
            (((((op0_limb_2_col42.clone() * op1_limb_6_col75.clone())
                + (op0_limb_3_col43.clone() * op1_limb_5_col74.clone()))
                + (op0_limb_4_col44.clone() * op1_limb_4_col73.clone()))
                + (op0_limb_5_col45.clone() * op1_limb_3_col72.clone()))
                + (op0_limb_6_col46.clone() * op1_limb_2_col71.clone())),
        );
        let z0_tmp_48d52_13_limb_9 = eval.add_intermediate(
            ((((op0_limb_3_col43.clone() * op1_limb_6_col75.clone())
                + (op0_limb_4_col44.clone() * op1_limb_5_col74.clone()))
                + (op0_limb_5_col45.clone() * op1_limb_4_col73.clone()))
                + (op0_limb_6_col46.clone() * op1_limb_3_col72.clone())),
        );
        let z0_tmp_48d52_13_limb_10 = eval.add_intermediate(
            (((op0_limb_4_col44.clone() * op1_limb_6_col75.clone())
                + (op0_limb_5_col45.clone() * op1_limb_5_col74.clone()))
                + (op0_limb_6_col46.clone() * op1_limb_4_col73.clone())),
        );
        let z0_tmp_48d52_13_limb_11 = eval.add_intermediate(
            ((op0_limb_5_col45.clone() * op1_limb_6_col75.clone())
                + (op0_limb_6_col46.clone() * op1_limb_5_col74.clone())),
        );
        let z0_tmp_48d52_13_limb_12 =
            eval.add_intermediate((op0_limb_6_col46.clone() * op1_limb_6_col75.clone()));
        let z2_tmp_48d52_14_limb_0 =
            eval.add_intermediate((op0_limb_7_col47.clone() * op1_limb_7_col76.clone()));
        let z2_tmp_48d52_14_limb_1 = eval.add_intermediate(
            ((op0_limb_7_col47.clone() * op1_limb_8_col77.clone())
                + (op0_limb_8_col48.clone() * op1_limb_7_col76.clone())),
        );
        let z2_tmp_48d52_14_limb_2 = eval.add_intermediate(
            (((op0_limb_7_col47.clone() * op1_limb_9_col78.clone())
                + (op0_limb_8_col48.clone() * op1_limb_8_col77.clone()))
                + (op0_limb_9_col49.clone() * op1_limb_7_col76.clone())),
        );
        let z2_tmp_48d52_14_limb_3 = eval.add_intermediate(
            ((((op0_limb_7_col47.clone() * op1_limb_10_col79.clone())
                + (op0_limb_8_col48.clone() * op1_limb_9_col78.clone()))
                + (op0_limb_9_col49.clone() * op1_limb_8_col77.clone()))
                + (op0_limb_10_col50.clone() * op1_limb_7_col76.clone())),
        );
        let z2_tmp_48d52_14_limb_4 = eval.add_intermediate(
            (((((op0_limb_7_col47.clone() * op1_limb_11_col80.clone())
                + (op0_limb_8_col48.clone() * op1_limb_10_col79.clone()))
                + (op0_limb_9_col49.clone() * op1_limb_9_col78.clone()))
                + (op0_limb_10_col50.clone() * op1_limb_8_col77.clone()))
                + (op0_limb_11_col51.clone() * op1_limb_7_col76.clone())),
        );
        let z2_tmp_48d52_14_limb_5 = eval.add_intermediate(
            ((((((op0_limb_7_col47.clone() * op1_limb_12_col81.clone())
                + (op0_limb_8_col48.clone() * op1_limb_11_col80.clone()))
                + (op0_limb_9_col49.clone() * op1_limb_10_col79.clone()))
                + (op0_limb_10_col50.clone() * op1_limb_9_col78.clone()))
                + (op0_limb_11_col51.clone() * op1_limb_8_col77.clone()))
                + (op0_limb_12_col52.clone() * op1_limb_7_col76.clone())),
        );
        let z2_tmp_48d52_14_limb_6 = eval.add_intermediate(
            (((((((op0_limb_7_col47.clone() * op1_limb_13_col82.clone())
                + (op0_limb_8_col48.clone() * op1_limb_12_col81.clone()))
                + (op0_limb_9_col49.clone() * op1_limb_11_col80.clone()))
                + (op0_limb_10_col50.clone() * op1_limb_10_col79.clone()))
                + (op0_limb_11_col51.clone() * op1_limb_9_col78.clone()))
                + (op0_limb_12_col52.clone() * op1_limb_8_col77.clone()))
                + (op0_limb_13_col53.clone() * op1_limb_7_col76.clone())),
        );
        let z2_tmp_48d52_14_limb_7 = eval.add_intermediate(
            ((((((op0_limb_8_col48.clone() * op1_limb_13_col82.clone())
                + (op0_limb_9_col49.clone() * op1_limb_12_col81.clone()))
                + (op0_limb_10_col50.clone() * op1_limb_11_col80.clone()))
                + (op0_limb_11_col51.clone() * op1_limb_10_col79.clone()))
                + (op0_limb_12_col52.clone() * op1_limb_9_col78.clone()))
                + (op0_limb_13_col53.clone() * op1_limb_8_col77.clone())),
        );
        let z2_tmp_48d52_14_limb_8 = eval.add_intermediate(
            (((((op0_limb_9_col49.clone() * op1_limb_13_col82.clone())
                + (op0_limb_10_col50.clone() * op1_limb_12_col81.clone()))
                + (op0_limb_11_col51.clone() * op1_limb_11_col80.clone()))
                + (op0_limb_12_col52.clone() * op1_limb_10_col79.clone()))
                + (op0_limb_13_col53.clone() * op1_limb_9_col78.clone())),
        );
        let z2_tmp_48d52_14_limb_9 = eval.add_intermediate(
            ((((op0_limb_10_col50.clone() * op1_limb_13_col82.clone())
                + (op0_limb_11_col51.clone() * op1_limb_12_col81.clone()))
                + (op0_limb_12_col52.clone() * op1_limb_11_col80.clone()))
                + (op0_limb_13_col53.clone() * op1_limb_10_col79.clone())),
        );
        let z2_tmp_48d52_14_limb_10 = eval.add_intermediate(
            (((op0_limb_11_col51.clone() * op1_limb_13_col82.clone())
                + (op0_limb_12_col52.clone() * op1_limb_12_col81.clone()))
                + (op0_limb_13_col53.clone() * op1_limb_11_col80.clone())),
        );
        let z2_tmp_48d52_14_limb_11 = eval.add_intermediate(
            ((op0_limb_12_col52.clone() * op1_limb_13_col82.clone())
                + (op0_limb_13_col53.clone() * op1_limb_12_col81.clone())),
        );
        let z2_tmp_48d52_14_limb_12 =
            eval.add_intermediate((op0_limb_13_col53.clone() * op1_limb_13_col82.clone()));
        let x_sum_tmp_48d52_15_limb_0 =
            eval.add_intermediate((op0_limb_0_col40.clone() + op0_limb_7_col47.clone()));
        let x_sum_tmp_48d52_15_limb_1 =
            eval.add_intermediate((op0_limb_1_col41.clone() + op0_limb_8_col48.clone()));
        let x_sum_tmp_48d52_15_limb_2 =
            eval.add_intermediate((op0_limb_2_col42.clone() + op0_limb_9_col49.clone()));
        let x_sum_tmp_48d52_15_limb_3 =
            eval.add_intermediate((op0_limb_3_col43.clone() + op0_limb_10_col50.clone()));
        let x_sum_tmp_48d52_15_limb_4 =
            eval.add_intermediate((op0_limb_4_col44.clone() + op0_limb_11_col51.clone()));
        let x_sum_tmp_48d52_15_limb_5 =
            eval.add_intermediate((op0_limb_5_col45.clone() + op0_limb_12_col52.clone()));
        let x_sum_tmp_48d52_15_limb_6 =
            eval.add_intermediate((op0_limb_6_col46.clone() + op0_limb_13_col53.clone()));
        let y_sum_tmp_48d52_16_limb_0 =
            eval.add_intermediate((op1_limb_0_col69.clone() + op1_limb_7_col76.clone()));
        let y_sum_tmp_48d52_16_limb_1 =
            eval.add_intermediate((op1_limb_1_col70.clone() + op1_limb_8_col77.clone()));
        let y_sum_tmp_48d52_16_limb_2 =
            eval.add_intermediate((op1_limb_2_col71.clone() + op1_limb_9_col78.clone()));
        let y_sum_tmp_48d52_16_limb_3 =
            eval.add_intermediate((op1_limb_3_col72.clone() + op1_limb_10_col79.clone()));
        let y_sum_tmp_48d52_16_limb_4 =
            eval.add_intermediate((op1_limb_4_col73.clone() + op1_limb_11_col80.clone()));
        let y_sum_tmp_48d52_16_limb_5 =
            eval.add_intermediate((op1_limb_5_col74.clone() + op1_limb_12_col81.clone()));
        let y_sum_tmp_48d52_16_limb_6 =
            eval.add_intermediate((op1_limb_6_col75.clone() + op1_limb_13_col82.clone()));

        // Single Karatsuba N 7.

        let z0_tmp_48d52_17_limb_0 =
            eval.add_intermediate((op0_limb_14_col54.clone() * op1_limb_14_col83.clone()));
        let z0_tmp_48d52_17_limb_1 = eval.add_intermediate(
            ((op0_limb_14_col54.clone() * op1_limb_15_col84.clone())
                + (op0_limb_15_col55.clone() * op1_limb_14_col83.clone())),
        );
        let z0_tmp_48d52_17_limb_2 = eval.add_intermediate(
            (((op0_limb_14_col54.clone() * op1_limb_16_col85.clone())
                + (op0_limb_15_col55.clone() * op1_limb_15_col84.clone()))
                + (op0_limb_16_col56.clone() * op1_limb_14_col83.clone())),
        );
        let z0_tmp_48d52_17_limb_3 = eval.add_intermediate(
            ((((op0_limb_14_col54.clone() * op1_limb_17_col86.clone())
                + (op0_limb_15_col55.clone() * op1_limb_16_col85.clone()))
                + (op0_limb_16_col56.clone() * op1_limb_15_col84.clone()))
                + (op0_limb_17_col57.clone() * op1_limb_14_col83.clone())),
        );
        let z0_tmp_48d52_17_limb_4 = eval.add_intermediate(
            (((((op0_limb_14_col54.clone() * op1_limb_18_col87.clone())
                + (op0_limb_15_col55.clone() * op1_limb_17_col86.clone()))
                + (op0_limb_16_col56.clone() * op1_limb_16_col85.clone()))
                + (op0_limb_17_col57.clone() * op1_limb_15_col84.clone()))
                + (op0_limb_18_col58.clone() * op1_limb_14_col83.clone())),
        );
        let z0_tmp_48d52_17_limb_5 = eval.add_intermediate(
            ((((((op0_limb_14_col54.clone() * op1_limb_19_col88.clone())
                + (op0_limb_15_col55.clone() * op1_limb_18_col87.clone()))
                + (op0_limb_16_col56.clone() * op1_limb_17_col86.clone()))
                + (op0_limb_17_col57.clone() * op1_limb_16_col85.clone()))
                + (op0_limb_18_col58.clone() * op1_limb_15_col84.clone()))
                + (op0_limb_19_col59.clone() * op1_limb_14_col83.clone())),
        );
        let z0_tmp_48d52_17_limb_6 = eval.add_intermediate(
            (((((((op0_limb_14_col54.clone() * op1_limb_20_col89.clone())
                + (op0_limb_15_col55.clone() * op1_limb_19_col88.clone()))
                + (op0_limb_16_col56.clone() * op1_limb_18_col87.clone()))
                + (op0_limb_17_col57.clone() * op1_limb_17_col86.clone()))
                + (op0_limb_18_col58.clone() * op1_limb_16_col85.clone()))
                + (op0_limb_19_col59.clone() * op1_limb_15_col84.clone()))
                + (op0_limb_20_col60.clone() * op1_limb_14_col83.clone())),
        );
        let z0_tmp_48d52_17_limb_7 = eval.add_intermediate(
            ((((((op0_limb_15_col55.clone() * op1_limb_20_col89.clone())
                + (op0_limb_16_col56.clone() * op1_limb_19_col88.clone()))
                + (op0_limb_17_col57.clone() * op1_limb_18_col87.clone()))
                + (op0_limb_18_col58.clone() * op1_limb_17_col86.clone()))
                + (op0_limb_19_col59.clone() * op1_limb_16_col85.clone()))
                + (op0_limb_20_col60.clone() * op1_limb_15_col84.clone())),
        );
        let z0_tmp_48d52_17_limb_8 = eval.add_intermediate(
            (((((op0_limb_16_col56.clone() * op1_limb_20_col89.clone())
                + (op0_limb_17_col57.clone() * op1_limb_19_col88.clone()))
                + (op0_limb_18_col58.clone() * op1_limb_18_col87.clone()))
                + (op0_limb_19_col59.clone() * op1_limb_17_col86.clone()))
                + (op0_limb_20_col60.clone() * op1_limb_16_col85.clone())),
        );
        let z0_tmp_48d52_17_limb_9 = eval.add_intermediate(
            ((((op0_limb_17_col57.clone() * op1_limb_20_col89.clone())
                + (op0_limb_18_col58.clone() * op1_limb_19_col88.clone()))
                + (op0_limb_19_col59.clone() * op1_limb_18_col87.clone()))
                + (op0_limb_20_col60.clone() * op1_limb_17_col86.clone())),
        );
        let z0_tmp_48d52_17_limb_10 = eval.add_intermediate(
            (((op0_limb_18_col58.clone() * op1_limb_20_col89.clone())
                + (op0_limb_19_col59.clone() * op1_limb_19_col88.clone()))
                + (op0_limb_20_col60.clone() * op1_limb_18_col87.clone())),
        );
        let z0_tmp_48d52_17_limb_11 = eval.add_intermediate(
            ((op0_limb_19_col59.clone() * op1_limb_20_col89.clone())
                + (op0_limb_20_col60.clone() * op1_limb_19_col88.clone())),
        );
        let z0_tmp_48d52_17_limb_12 =
            eval.add_intermediate((op0_limb_20_col60.clone() * op1_limb_20_col89.clone()));
        let z2_tmp_48d52_18_limb_0 =
            eval.add_intermediate((op0_limb_21_col61.clone() * op1_limb_21_col90.clone()));
        let z2_tmp_48d52_18_limb_1 = eval.add_intermediate(
            ((op0_limb_21_col61.clone() * op1_limb_22_col91.clone())
                + (op0_limb_22_col62.clone() * op1_limb_21_col90.clone())),
        );
        let z2_tmp_48d52_18_limb_2 = eval.add_intermediate(
            (((op0_limb_21_col61.clone() * op1_limb_23_col92.clone())
                + (op0_limb_22_col62.clone() * op1_limb_22_col91.clone()))
                + (op0_limb_23_col63.clone() * op1_limb_21_col90.clone())),
        );
        let z2_tmp_48d52_18_limb_3 = eval.add_intermediate(
            ((((op0_limb_21_col61.clone() * op1_limb_24_col93.clone())
                + (op0_limb_22_col62.clone() * op1_limb_23_col92.clone()))
                + (op0_limb_23_col63.clone() * op1_limb_22_col91.clone()))
                + (op0_limb_24_col64.clone() * op1_limb_21_col90.clone())),
        );
        let z2_tmp_48d52_18_limb_4 = eval.add_intermediate(
            (((((op0_limb_21_col61.clone() * op1_limb_25_col94.clone())
                + (op0_limb_22_col62.clone() * op1_limb_24_col93.clone()))
                + (op0_limb_23_col63.clone() * op1_limb_23_col92.clone()))
                + (op0_limb_24_col64.clone() * op1_limb_22_col91.clone()))
                + (op0_limb_25_col65.clone() * op1_limb_21_col90.clone())),
        );
        let z2_tmp_48d52_18_limb_5 = eval.add_intermediate(
            ((((((op0_limb_21_col61.clone() * op1_limb_26_col95.clone())
                + (op0_limb_22_col62.clone() * op1_limb_25_col94.clone()))
                + (op0_limb_23_col63.clone() * op1_limb_24_col93.clone()))
                + (op0_limb_24_col64.clone() * op1_limb_23_col92.clone()))
                + (op0_limb_25_col65.clone() * op1_limb_22_col91.clone()))
                + (op0_limb_26_col66.clone() * op1_limb_21_col90.clone())),
        );
        let z2_tmp_48d52_18_limb_6 = eval.add_intermediate(
            (((((((op0_limb_21_col61.clone() * op1_limb_27_col96.clone())
                + (op0_limb_22_col62.clone() * op1_limb_26_col95.clone()))
                + (op0_limb_23_col63.clone() * op1_limb_25_col94.clone()))
                + (op0_limb_24_col64.clone() * op1_limb_24_col93.clone()))
                + (op0_limb_25_col65.clone() * op1_limb_23_col92.clone()))
                + (op0_limb_26_col66.clone() * op1_limb_22_col91.clone()))
                + (op0_limb_27_col67.clone() * op1_limb_21_col90.clone())),
        );
        let z2_tmp_48d52_18_limb_7 = eval.add_intermediate(
            ((((((op0_limb_22_col62.clone() * op1_limb_27_col96.clone())
                + (op0_limb_23_col63.clone() * op1_limb_26_col95.clone()))
                + (op0_limb_24_col64.clone() * op1_limb_25_col94.clone()))
                + (op0_limb_25_col65.clone() * op1_limb_24_col93.clone()))
                + (op0_limb_26_col66.clone() * op1_limb_23_col92.clone()))
                + (op0_limb_27_col67.clone() * op1_limb_22_col91.clone())),
        );
        let z2_tmp_48d52_18_limb_8 = eval.add_intermediate(
            (((((op0_limb_23_col63.clone() * op1_limb_27_col96.clone())
                + (op0_limb_24_col64.clone() * op1_limb_26_col95.clone()))
                + (op0_limb_25_col65.clone() * op1_limb_25_col94.clone()))
                + (op0_limb_26_col66.clone() * op1_limb_24_col93.clone()))
                + (op0_limb_27_col67.clone() * op1_limb_23_col92.clone())),
        );
        let z2_tmp_48d52_18_limb_9 = eval.add_intermediate(
            ((((op0_limb_24_col64.clone() * op1_limb_27_col96.clone())
                + (op0_limb_25_col65.clone() * op1_limb_26_col95.clone()))
                + (op0_limb_26_col66.clone() * op1_limb_25_col94.clone()))
                + (op0_limb_27_col67.clone() * op1_limb_24_col93.clone())),
        );
        let z2_tmp_48d52_18_limb_10 = eval.add_intermediate(
            (((op0_limb_25_col65.clone() * op1_limb_27_col96.clone())
                + (op0_limb_26_col66.clone() * op1_limb_26_col95.clone()))
                + (op0_limb_27_col67.clone() * op1_limb_25_col94.clone())),
        );
        let z2_tmp_48d52_18_limb_11 = eval.add_intermediate(
            ((op0_limb_26_col66.clone() * op1_limb_27_col96.clone())
                + (op0_limb_27_col67.clone() * op1_limb_26_col95.clone())),
        );
        let z2_tmp_48d52_18_limb_12 =
            eval.add_intermediate((op0_limb_27_col67.clone() * op1_limb_27_col96.clone()));
        let x_sum_tmp_48d52_19_limb_0 =
            eval.add_intermediate((op0_limb_14_col54.clone() + op0_limb_21_col61.clone()));
        let x_sum_tmp_48d52_19_limb_1 =
            eval.add_intermediate((op0_limb_15_col55.clone() + op0_limb_22_col62.clone()));
        let x_sum_tmp_48d52_19_limb_2 =
            eval.add_intermediate((op0_limb_16_col56.clone() + op0_limb_23_col63.clone()));
        let x_sum_tmp_48d52_19_limb_3 =
            eval.add_intermediate((op0_limb_17_col57.clone() + op0_limb_24_col64.clone()));
        let x_sum_tmp_48d52_19_limb_4 =
            eval.add_intermediate((op0_limb_18_col58.clone() + op0_limb_25_col65.clone()));
        let x_sum_tmp_48d52_19_limb_5 =
            eval.add_intermediate((op0_limb_19_col59.clone() + op0_limb_26_col66.clone()));
        let x_sum_tmp_48d52_19_limb_6 =
            eval.add_intermediate((op0_limb_20_col60.clone() + op0_limb_27_col67.clone()));
        let y_sum_tmp_48d52_20_limb_0 =
            eval.add_intermediate((op1_limb_14_col83.clone() + op1_limb_21_col90.clone()));
        let y_sum_tmp_48d52_20_limb_1 =
            eval.add_intermediate((op1_limb_15_col84.clone() + op1_limb_22_col91.clone()));
        let y_sum_tmp_48d52_20_limb_2 =
            eval.add_intermediate((op1_limb_16_col85.clone() + op1_limb_23_col92.clone()));
        let y_sum_tmp_48d52_20_limb_3 =
            eval.add_intermediate((op1_limb_17_col86.clone() + op1_limb_24_col93.clone()));
        let y_sum_tmp_48d52_20_limb_4 =
            eval.add_intermediate((op1_limb_18_col87.clone() + op1_limb_25_col94.clone()));
        let y_sum_tmp_48d52_20_limb_5 =
            eval.add_intermediate((op1_limb_19_col88.clone() + op1_limb_26_col95.clone()));
        let y_sum_tmp_48d52_20_limb_6 =
            eval.add_intermediate((op1_limb_20_col89.clone() + op1_limb_27_col96.clone()));

        let z0_tmp_48d52_21_limb_0 = eval.add_intermediate(z0_tmp_48d52_13_limb_0.clone());
        let z0_tmp_48d52_21_limb_1 = eval.add_intermediate(z0_tmp_48d52_13_limb_1.clone());
        let z0_tmp_48d52_21_limb_2 = eval.add_intermediate(z0_tmp_48d52_13_limb_2.clone());
        let z0_tmp_48d52_21_limb_3 = eval.add_intermediate(z0_tmp_48d52_13_limb_3.clone());
        let z0_tmp_48d52_21_limb_4 = eval.add_intermediate(z0_tmp_48d52_13_limb_4.clone());
        let z0_tmp_48d52_21_limb_5 = eval.add_intermediate(z0_tmp_48d52_13_limb_5.clone());
        let z0_tmp_48d52_21_limb_6 = eval.add_intermediate(z0_tmp_48d52_13_limb_6.clone());
        let z0_tmp_48d52_21_limb_7 = eval.add_intermediate(
            (z0_tmp_48d52_13_limb_7.clone()
                + (((x_sum_tmp_48d52_15_limb_0.clone() * y_sum_tmp_48d52_16_limb_0.clone())
                    - z0_tmp_48d52_13_limb_0.clone())
                    - z2_tmp_48d52_14_limb_0.clone())),
        );
        let z0_tmp_48d52_21_limb_8 = eval.add_intermediate(
            (z0_tmp_48d52_13_limb_8.clone()
                + ((((x_sum_tmp_48d52_15_limb_0.clone() * y_sum_tmp_48d52_16_limb_1.clone())
                    + (x_sum_tmp_48d52_15_limb_1.clone() * y_sum_tmp_48d52_16_limb_0.clone()))
                    - z0_tmp_48d52_13_limb_1.clone())
                    - z2_tmp_48d52_14_limb_1.clone())),
        );
        let z0_tmp_48d52_21_limb_9 = eval.add_intermediate(
            (z0_tmp_48d52_13_limb_9.clone()
                + (((((x_sum_tmp_48d52_15_limb_0.clone() * y_sum_tmp_48d52_16_limb_2.clone())
                    + (x_sum_tmp_48d52_15_limb_1.clone() * y_sum_tmp_48d52_16_limb_1.clone()))
                    + (x_sum_tmp_48d52_15_limb_2.clone() * y_sum_tmp_48d52_16_limb_0.clone()))
                    - z0_tmp_48d52_13_limb_2.clone())
                    - z2_tmp_48d52_14_limb_2.clone())),
        );
        let z0_tmp_48d52_21_limb_10 = eval.add_intermediate(
            (z0_tmp_48d52_13_limb_10.clone()
                + ((((((x_sum_tmp_48d52_15_limb_0.clone()
                    * y_sum_tmp_48d52_16_limb_3.clone())
                    + (x_sum_tmp_48d52_15_limb_1.clone() * y_sum_tmp_48d52_16_limb_2.clone()))
                    + (x_sum_tmp_48d52_15_limb_2.clone() * y_sum_tmp_48d52_16_limb_1.clone()))
                    + (x_sum_tmp_48d52_15_limb_3.clone() * y_sum_tmp_48d52_16_limb_0.clone()))
                    - z0_tmp_48d52_13_limb_3.clone())
                    - z2_tmp_48d52_14_limb_3.clone())),
        );
        let z0_tmp_48d52_21_limb_11 = eval.add_intermediate(
            (z0_tmp_48d52_13_limb_11.clone()
                + (((((((x_sum_tmp_48d52_15_limb_0.clone()
                    * y_sum_tmp_48d52_16_limb_4.clone())
                    + (x_sum_tmp_48d52_15_limb_1.clone()
                        * y_sum_tmp_48d52_16_limb_3.clone()))
                    + (x_sum_tmp_48d52_15_limb_2.clone() * y_sum_tmp_48d52_16_limb_2.clone()))
                    + (x_sum_tmp_48d52_15_limb_3.clone() * y_sum_tmp_48d52_16_limb_1.clone()))
                    + (x_sum_tmp_48d52_15_limb_4.clone() * y_sum_tmp_48d52_16_limb_0.clone()))
                    - z0_tmp_48d52_13_limb_4.clone())
                    - z2_tmp_48d52_14_limb_4.clone())),
        );
        let z0_tmp_48d52_21_limb_12 = eval.add_intermediate(
            (z0_tmp_48d52_13_limb_12.clone()
                + ((((((((x_sum_tmp_48d52_15_limb_0.clone()
                    * y_sum_tmp_48d52_16_limb_5.clone())
                    + (x_sum_tmp_48d52_15_limb_1.clone()
                        * y_sum_tmp_48d52_16_limb_4.clone()))
                    + (x_sum_tmp_48d52_15_limb_2.clone()
                        * y_sum_tmp_48d52_16_limb_3.clone()))
                    + (x_sum_tmp_48d52_15_limb_3.clone() * y_sum_tmp_48d52_16_limb_2.clone()))
                    + (x_sum_tmp_48d52_15_limb_4.clone() * y_sum_tmp_48d52_16_limb_1.clone()))
                    + (x_sum_tmp_48d52_15_limb_5.clone() * y_sum_tmp_48d52_16_limb_0.clone()))
                    - z0_tmp_48d52_13_limb_5.clone())
                    - z2_tmp_48d52_14_limb_5.clone())),
        );
        let z0_tmp_48d52_21_limb_13 = eval.add_intermediate(
            (((((((((x_sum_tmp_48d52_15_limb_0.clone() * y_sum_tmp_48d52_16_limb_6.clone())
                + (x_sum_tmp_48d52_15_limb_1.clone() * y_sum_tmp_48d52_16_limb_5.clone()))
                + (x_sum_tmp_48d52_15_limb_2.clone() * y_sum_tmp_48d52_16_limb_4.clone()))
                + (x_sum_tmp_48d52_15_limb_3.clone() * y_sum_tmp_48d52_16_limb_3.clone()))
                + (x_sum_tmp_48d52_15_limb_4.clone() * y_sum_tmp_48d52_16_limb_2.clone()))
                + (x_sum_tmp_48d52_15_limb_5.clone() * y_sum_tmp_48d52_16_limb_1.clone()))
                + (x_sum_tmp_48d52_15_limb_6.clone() * y_sum_tmp_48d52_16_limb_0.clone()))
                - z0_tmp_48d52_13_limb_6.clone())
                - z2_tmp_48d52_14_limb_6.clone()),
        );
        let z0_tmp_48d52_21_limb_14 = eval.add_intermediate(
            (z2_tmp_48d52_14_limb_0.clone()
                + ((((((((x_sum_tmp_48d52_15_limb_1.clone()
                    * y_sum_tmp_48d52_16_limb_6.clone())
                    + (x_sum_tmp_48d52_15_limb_2.clone()
                        * y_sum_tmp_48d52_16_limb_5.clone()))
                    + (x_sum_tmp_48d52_15_limb_3.clone()
                        * y_sum_tmp_48d52_16_limb_4.clone()))
                    + (x_sum_tmp_48d52_15_limb_4.clone() * y_sum_tmp_48d52_16_limb_3.clone()))
                    + (x_sum_tmp_48d52_15_limb_5.clone() * y_sum_tmp_48d52_16_limb_2.clone()))
                    + (x_sum_tmp_48d52_15_limb_6.clone() * y_sum_tmp_48d52_16_limb_1.clone()))
                    - z0_tmp_48d52_13_limb_7.clone())
                    - z2_tmp_48d52_14_limb_7.clone())),
        );
        let z0_tmp_48d52_21_limb_15 = eval.add_intermediate(
            (z2_tmp_48d52_14_limb_1.clone()
                + (((((((x_sum_tmp_48d52_15_limb_2.clone()
                    * y_sum_tmp_48d52_16_limb_6.clone())
                    + (x_sum_tmp_48d52_15_limb_3.clone()
                        * y_sum_tmp_48d52_16_limb_5.clone()))
                    + (x_sum_tmp_48d52_15_limb_4.clone() * y_sum_tmp_48d52_16_limb_4.clone()))
                    + (x_sum_tmp_48d52_15_limb_5.clone() * y_sum_tmp_48d52_16_limb_3.clone()))
                    + (x_sum_tmp_48d52_15_limb_6.clone() * y_sum_tmp_48d52_16_limb_2.clone()))
                    - z0_tmp_48d52_13_limb_8.clone())
                    - z2_tmp_48d52_14_limb_8.clone())),
        );
        let z0_tmp_48d52_21_limb_16 = eval.add_intermediate(
            (z2_tmp_48d52_14_limb_2.clone()
                + ((((((x_sum_tmp_48d52_15_limb_3.clone()
                    * y_sum_tmp_48d52_16_limb_6.clone())
                    + (x_sum_tmp_48d52_15_limb_4.clone() * y_sum_tmp_48d52_16_limb_5.clone()))
                    + (x_sum_tmp_48d52_15_limb_5.clone() * y_sum_tmp_48d52_16_limb_4.clone()))
                    + (x_sum_tmp_48d52_15_limb_6.clone() * y_sum_tmp_48d52_16_limb_3.clone()))
                    - z0_tmp_48d52_13_limb_9.clone())
                    - z2_tmp_48d52_14_limb_9.clone())),
        );
        let z0_tmp_48d52_21_limb_17 = eval.add_intermediate(
            (z2_tmp_48d52_14_limb_3.clone()
                + (((((x_sum_tmp_48d52_15_limb_4.clone() * y_sum_tmp_48d52_16_limb_6.clone())
                    + (x_sum_tmp_48d52_15_limb_5.clone() * y_sum_tmp_48d52_16_limb_5.clone()))
                    + (x_sum_tmp_48d52_15_limb_6.clone() * y_sum_tmp_48d52_16_limb_4.clone()))
                    - z0_tmp_48d52_13_limb_10.clone())
                    - z2_tmp_48d52_14_limb_10.clone())),
        );
        let z0_tmp_48d52_21_limb_18 = eval.add_intermediate(
            (z2_tmp_48d52_14_limb_4.clone()
                + ((((x_sum_tmp_48d52_15_limb_5.clone() * y_sum_tmp_48d52_16_limb_6.clone())
                    + (x_sum_tmp_48d52_15_limb_6.clone() * y_sum_tmp_48d52_16_limb_5.clone()))
                    - z0_tmp_48d52_13_limb_11.clone())
                    - z2_tmp_48d52_14_limb_11.clone())),
        );
        let z0_tmp_48d52_21_limb_19 = eval.add_intermediate(
            (z2_tmp_48d52_14_limb_5.clone()
                + (((x_sum_tmp_48d52_15_limb_6.clone() * y_sum_tmp_48d52_16_limb_6.clone())
                    - z0_tmp_48d52_13_limb_12.clone())
                    - z2_tmp_48d52_14_limb_12.clone())),
        );
        let z0_tmp_48d52_21_limb_20 = eval.add_intermediate(z2_tmp_48d52_14_limb_6.clone());
        let z0_tmp_48d52_21_limb_21 = eval.add_intermediate(z2_tmp_48d52_14_limb_7.clone());
        let z0_tmp_48d52_21_limb_22 = eval.add_intermediate(z2_tmp_48d52_14_limb_8.clone());
        let z0_tmp_48d52_21_limb_23 = eval.add_intermediate(z2_tmp_48d52_14_limb_9.clone());
        let z0_tmp_48d52_21_limb_24 = eval.add_intermediate(z2_tmp_48d52_14_limb_10.clone());
        let z0_tmp_48d52_21_limb_25 = eval.add_intermediate(z2_tmp_48d52_14_limb_11.clone());
        let z0_tmp_48d52_21_limb_26 = eval.add_intermediate(z2_tmp_48d52_14_limb_12.clone());
        let z2_tmp_48d52_22_limb_0 = eval.add_intermediate(z0_tmp_48d52_17_limb_0.clone());
        let z2_tmp_48d52_22_limb_1 = eval.add_intermediate(z0_tmp_48d52_17_limb_1.clone());
        let z2_tmp_48d52_22_limb_2 = eval.add_intermediate(z0_tmp_48d52_17_limb_2.clone());
        let z2_tmp_48d52_22_limb_3 = eval.add_intermediate(z0_tmp_48d52_17_limb_3.clone());
        let z2_tmp_48d52_22_limb_4 = eval.add_intermediate(z0_tmp_48d52_17_limb_4.clone());
        let z2_tmp_48d52_22_limb_5 = eval.add_intermediate(z0_tmp_48d52_17_limb_5.clone());
        let z2_tmp_48d52_22_limb_6 = eval.add_intermediate(z0_tmp_48d52_17_limb_6.clone());
        let z2_tmp_48d52_22_limb_7 = eval.add_intermediate(
            (z0_tmp_48d52_17_limb_7.clone()
                + (((x_sum_tmp_48d52_19_limb_0.clone() * y_sum_tmp_48d52_20_limb_0.clone())
                    - z0_tmp_48d52_17_limb_0.clone())
                    - z2_tmp_48d52_18_limb_0.clone())),
        );
        let z2_tmp_48d52_22_limb_8 = eval.add_intermediate(
            (z0_tmp_48d52_17_limb_8.clone()
                + ((((x_sum_tmp_48d52_19_limb_0.clone() * y_sum_tmp_48d52_20_limb_1.clone())
                    + (x_sum_tmp_48d52_19_limb_1.clone() * y_sum_tmp_48d52_20_limb_0.clone()))
                    - z0_tmp_48d52_17_limb_1.clone())
                    - z2_tmp_48d52_18_limb_1.clone())),
        );
        let z2_tmp_48d52_22_limb_9 = eval.add_intermediate(
            (z0_tmp_48d52_17_limb_9.clone()
                + (((((x_sum_tmp_48d52_19_limb_0.clone() * y_sum_tmp_48d52_20_limb_2.clone())
                    + (x_sum_tmp_48d52_19_limb_1.clone() * y_sum_tmp_48d52_20_limb_1.clone()))
                    + (x_sum_tmp_48d52_19_limb_2.clone() * y_sum_tmp_48d52_20_limb_0.clone()))
                    - z0_tmp_48d52_17_limb_2.clone())
                    - z2_tmp_48d52_18_limb_2.clone())),
        );
        let z2_tmp_48d52_22_limb_10 = eval.add_intermediate(
            (z0_tmp_48d52_17_limb_10.clone()
                + ((((((x_sum_tmp_48d52_19_limb_0.clone()
                    * y_sum_tmp_48d52_20_limb_3.clone())
                    + (x_sum_tmp_48d52_19_limb_1.clone() * y_sum_tmp_48d52_20_limb_2.clone()))
                    + (x_sum_tmp_48d52_19_limb_2.clone() * y_sum_tmp_48d52_20_limb_1.clone()))
                    + (x_sum_tmp_48d52_19_limb_3.clone() * y_sum_tmp_48d52_20_limb_0.clone()))
                    - z0_tmp_48d52_17_limb_3.clone())
                    - z2_tmp_48d52_18_limb_3.clone())),
        );
        let z2_tmp_48d52_22_limb_11 = eval.add_intermediate(
            (z0_tmp_48d52_17_limb_11.clone()
                + (((((((x_sum_tmp_48d52_19_limb_0.clone()
                    * y_sum_tmp_48d52_20_limb_4.clone())
                    + (x_sum_tmp_48d52_19_limb_1.clone()
                        * y_sum_tmp_48d52_20_limb_3.clone()))
                    + (x_sum_tmp_48d52_19_limb_2.clone() * y_sum_tmp_48d52_20_limb_2.clone()))
                    + (x_sum_tmp_48d52_19_limb_3.clone() * y_sum_tmp_48d52_20_limb_1.clone()))
                    + (x_sum_tmp_48d52_19_limb_4.clone() * y_sum_tmp_48d52_20_limb_0.clone()))
                    - z0_tmp_48d52_17_limb_4.clone())
                    - z2_tmp_48d52_18_limb_4.clone())),
        );
        let z2_tmp_48d52_22_limb_12 = eval.add_intermediate(
            (z0_tmp_48d52_17_limb_12.clone()
                + ((((((((x_sum_tmp_48d52_19_limb_0.clone()
                    * y_sum_tmp_48d52_20_limb_5.clone())
                    + (x_sum_tmp_48d52_19_limb_1.clone()
                        * y_sum_tmp_48d52_20_limb_4.clone()))
                    + (x_sum_tmp_48d52_19_limb_2.clone()
                        * y_sum_tmp_48d52_20_limb_3.clone()))
                    + (x_sum_tmp_48d52_19_limb_3.clone() * y_sum_tmp_48d52_20_limb_2.clone()))
                    + (x_sum_tmp_48d52_19_limb_4.clone() * y_sum_tmp_48d52_20_limb_1.clone()))
                    + (x_sum_tmp_48d52_19_limb_5.clone() * y_sum_tmp_48d52_20_limb_0.clone()))
                    - z0_tmp_48d52_17_limb_5.clone())
                    - z2_tmp_48d52_18_limb_5.clone())),
        );
        let z2_tmp_48d52_22_limb_13 = eval.add_intermediate(
            (((((((((x_sum_tmp_48d52_19_limb_0.clone() * y_sum_tmp_48d52_20_limb_6.clone())
                + (x_sum_tmp_48d52_19_limb_1.clone() * y_sum_tmp_48d52_20_limb_5.clone()))
                + (x_sum_tmp_48d52_19_limb_2.clone() * y_sum_tmp_48d52_20_limb_4.clone()))
                + (x_sum_tmp_48d52_19_limb_3.clone() * y_sum_tmp_48d52_20_limb_3.clone()))
                + (x_sum_tmp_48d52_19_limb_4.clone() * y_sum_tmp_48d52_20_limb_2.clone()))
                + (x_sum_tmp_48d52_19_limb_5.clone() * y_sum_tmp_48d52_20_limb_1.clone()))
                + (x_sum_tmp_48d52_19_limb_6.clone() * y_sum_tmp_48d52_20_limb_0.clone()))
                - z0_tmp_48d52_17_limb_6.clone())
                - z2_tmp_48d52_18_limb_6.clone()),
        );
        let z2_tmp_48d52_22_limb_14 = eval.add_intermediate(
            (z2_tmp_48d52_18_limb_0.clone()
                + ((((((((x_sum_tmp_48d52_19_limb_1.clone()
                    * y_sum_tmp_48d52_20_limb_6.clone())
                    + (x_sum_tmp_48d52_19_limb_2.clone()
                        * y_sum_tmp_48d52_20_limb_5.clone()))
                    + (x_sum_tmp_48d52_19_limb_3.clone()
                        * y_sum_tmp_48d52_20_limb_4.clone()))
                    + (x_sum_tmp_48d52_19_limb_4.clone() * y_sum_tmp_48d52_20_limb_3.clone()))
                    + (x_sum_tmp_48d52_19_limb_5.clone() * y_sum_tmp_48d52_20_limb_2.clone()))
                    + (x_sum_tmp_48d52_19_limb_6.clone() * y_sum_tmp_48d52_20_limb_1.clone()))
                    - z0_tmp_48d52_17_limb_7.clone())
                    - z2_tmp_48d52_18_limb_7.clone())),
        );
        let z2_tmp_48d52_22_limb_15 = eval.add_intermediate(
            (z2_tmp_48d52_18_limb_1.clone()
                + (((((((x_sum_tmp_48d52_19_limb_2.clone()
                    * y_sum_tmp_48d52_20_limb_6.clone())
                    + (x_sum_tmp_48d52_19_limb_3.clone()
                        * y_sum_tmp_48d52_20_limb_5.clone()))
                    + (x_sum_tmp_48d52_19_limb_4.clone() * y_sum_tmp_48d52_20_limb_4.clone()))
                    + (x_sum_tmp_48d52_19_limb_5.clone() * y_sum_tmp_48d52_20_limb_3.clone()))
                    + (x_sum_tmp_48d52_19_limb_6.clone() * y_sum_tmp_48d52_20_limb_2.clone()))
                    - z0_tmp_48d52_17_limb_8.clone())
                    - z2_tmp_48d52_18_limb_8.clone())),
        );
        let z2_tmp_48d52_22_limb_16 = eval.add_intermediate(
            (z2_tmp_48d52_18_limb_2.clone()
                + ((((((x_sum_tmp_48d52_19_limb_3.clone()
                    * y_sum_tmp_48d52_20_limb_6.clone())
                    + (x_sum_tmp_48d52_19_limb_4.clone() * y_sum_tmp_48d52_20_limb_5.clone()))
                    + (x_sum_tmp_48d52_19_limb_5.clone() * y_sum_tmp_48d52_20_limb_4.clone()))
                    + (x_sum_tmp_48d52_19_limb_6.clone() * y_sum_tmp_48d52_20_limb_3.clone()))
                    - z0_tmp_48d52_17_limb_9.clone())
                    - z2_tmp_48d52_18_limb_9.clone())),
        );
        let z2_tmp_48d52_22_limb_17 = eval.add_intermediate(
            (z2_tmp_48d52_18_limb_3.clone()
                + (((((x_sum_tmp_48d52_19_limb_4.clone() * y_sum_tmp_48d52_20_limb_6.clone())
                    + (x_sum_tmp_48d52_19_limb_5.clone() * y_sum_tmp_48d52_20_limb_5.clone()))
                    + (x_sum_tmp_48d52_19_limb_6.clone() * y_sum_tmp_48d52_20_limb_4.clone()))
                    - z0_tmp_48d52_17_limb_10.clone())
                    - z2_tmp_48d52_18_limb_10.clone())),
        );
        let z2_tmp_48d52_22_limb_18 = eval.add_intermediate(
            (z2_tmp_48d52_18_limb_4.clone()
                + ((((x_sum_tmp_48d52_19_limb_5.clone() * y_sum_tmp_48d52_20_limb_6.clone())
                    + (x_sum_tmp_48d52_19_limb_6.clone() * y_sum_tmp_48d52_20_limb_5.clone()))
                    - z0_tmp_48d52_17_limb_11.clone())
                    - z2_tmp_48d52_18_limb_11.clone())),
        );
        let z2_tmp_48d52_22_limb_19 = eval.add_intermediate(
            (z2_tmp_48d52_18_limb_5.clone()
                + (((x_sum_tmp_48d52_19_limb_6.clone() * y_sum_tmp_48d52_20_limb_6.clone())
                    - z0_tmp_48d52_17_limb_12.clone())
                    - z2_tmp_48d52_18_limb_12.clone())),
        );
        let z2_tmp_48d52_22_limb_20 = eval.add_intermediate(z2_tmp_48d52_18_limb_6.clone());
        let z2_tmp_48d52_22_limb_21 = eval.add_intermediate(z2_tmp_48d52_18_limb_7.clone());
        let z2_tmp_48d52_22_limb_22 = eval.add_intermediate(z2_tmp_48d52_18_limb_8.clone());
        let z2_tmp_48d52_22_limb_23 = eval.add_intermediate(z2_tmp_48d52_18_limb_9.clone());
        let z2_tmp_48d52_22_limb_24 = eval.add_intermediate(z2_tmp_48d52_18_limb_10.clone());
        let z2_tmp_48d52_22_limb_25 = eval.add_intermediate(z2_tmp_48d52_18_limb_11.clone());
        let z2_tmp_48d52_22_limb_26 = eval.add_intermediate(z2_tmp_48d52_18_limb_12.clone());
        let x_sum_tmp_48d52_23_limb_0 =
            eval.add_intermediate((op0_limb_0_col40.clone() + op0_limb_14_col54.clone()));
        let x_sum_tmp_48d52_23_limb_1 =
            eval.add_intermediate((op0_limb_1_col41.clone() + op0_limb_15_col55.clone()));
        let x_sum_tmp_48d52_23_limb_2 =
            eval.add_intermediate((op0_limb_2_col42.clone() + op0_limb_16_col56.clone()));
        let x_sum_tmp_48d52_23_limb_3 =
            eval.add_intermediate((op0_limb_3_col43.clone() + op0_limb_17_col57.clone()));
        let x_sum_tmp_48d52_23_limb_4 =
            eval.add_intermediate((op0_limb_4_col44.clone() + op0_limb_18_col58.clone()));
        let x_sum_tmp_48d52_23_limb_5 =
            eval.add_intermediate((op0_limb_5_col45.clone() + op0_limb_19_col59.clone()));
        let x_sum_tmp_48d52_23_limb_6 =
            eval.add_intermediate((op0_limb_6_col46.clone() + op0_limb_20_col60.clone()));
        let x_sum_tmp_48d52_23_limb_7 =
            eval.add_intermediate((op0_limb_7_col47.clone() + op0_limb_21_col61.clone()));
        let x_sum_tmp_48d52_23_limb_8 =
            eval.add_intermediate((op0_limb_8_col48.clone() + op0_limb_22_col62.clone()));
        let x_sum_tmp_48d52_23_limb_9 =
            eval.add_intermediate((op0_limb_9_col49.clone() + op0_limb_23_col63.clone()));
        let x_sum_tmp_48d52_23_limb_10 =
            eval.add_intermediate((op0_limb_10_col50.clone() + op0_limb_24_col64.clone()));
        let x_sum_tmp_48d52_23_limb_11 =
            eval.add_intermediate((op0_limb_11_col51.clone() + op0_limb_25_col65.clone()));
        let x_sum_tmp_48d52_23_limb_12 =
            eval.add_intermediate((op0_limb_12_col52.clone() + op0_limb_26_col66.clone()));
        let x_sum_tmp_48d52_23_limb_13 =
            eval.add_intermediate((op0_limb_13_col53.clone() + op0_limb_27_col67.clone()));
        let y_sum_tmp_48d52_24_limb_0 =
            eval.add_intermediate((op1_limb_0_col69.clone() + op1_limb_14_col83.clone()));
        let y_sum_tmp_48d52_24_limb_1 =
            eval.add_intermediate((op1_limb_1_col70.clone() + op1_limb_15_col84.clone()));
        let y_sum_tmp_48d52_24_limb_2 =
            eval.add_intermediate((op1_limb_2_col71.clone() + op1_limb_16_col85.clone()));
        let y_sum_tmp_48d52_24_limb_3 =
            eval.add_intermediate((op1_limb_3_col72.clone() + op1_limb_17_col86.clone()));
        let y_sum_tmp_48d52_24_limb_4 =
            eval.add_intermediate((op1_limb_4_col73.clone() + op1_limb_18_col87.clone()));
        let y_sum_tmp_48d52_24_limb_5 =
            eval.add_intermediate((op1_limb_5_col74.clone() + op1_limb_19_col88.clone()));
        let y_sum_tmp_48d52_24_limb_6 =
            eval.add_intermediate((op1_limb_6_col75.clone() + op1_limb_20_col89.clone()));
        let y_sum_tmp_48d52_24_limb_7 =
            eval.add_intermediate((op1_limb_7_col76.clone() + op1_limb_21_col90.clone()));
        let y_sum_tmp_48d52_24_limb_8 =
            eval.add_intermediate((op1_limb_8_col77.clone() + op1_limb_22_col91.clone()));
        let y_sum_tmp_48d52_24_limb_9 =
            eval.add_intermediate((op1_limb_9_col78.clone() + op1_limb_23_col92.clone()));
        let y_sum_tmp_48d52_24_limb_10 =
            eval.add_intermediate((op1_limb_10_col79.clone() + op1_limb_24_col93.clone()));
        let y_sum_tmp_48d52_24_limb_11 =
            eval.add_intermediate((op1_limb_11_col80.clone() + op1_limb_25_col94.clone()));
        let y_sum_tmp_48d52_24_limb_12 =
            eval.add_intermediate((op1_limb_12_col81.clone() + op1_limb_26_col95.clone()));
        let y_sum_tmp_48d52_24_limb_13 =
            eval.add_intermediate((op1_limb_13_col82.clone() + op1_limb_27_col96.clone()));

        // Single Karatsuba N 7.

        let z0_tmp_48d52_25_limb_0 = eval.add_intermediate(
            (x_sum_tmp_48d52_23_limb_0.clone() * y_sum_tmp_48d52_24_limb_0.clone()),
        );
        let z0_tmp_48d52_25_limb_1 = eval.add_intermediate(
            ((x_sum_tmp_48d52_23_limb_0.clone() * y_sum_tmp_48d52_24_limb_1.clone())
                + (x_sum_tmp_48d52_23_limb_1.clone() * y_sum_tmp_48d52_24_limb_0.clone())),
        );
        let z0_tmp_48d52_25_limb_2 = eval.add_intermediate(
            (((x_sum_tmp_48d52_23_limb_0.clone() * y_sum_tmp_48d52_24_limb_2.clone())
                + (x_sum_tmp_48d52_23_limb_1.clone() * y_sum_tmp_48d52_24_limb_1.clone()))
                + (x_sum_tmp_48d52_23_limb_2.clone() * y_sum_tmp_48d52_24_limb_0.clone())),
        );
        let z0_tmp_48d52_25_limb_3 = eval.add_intermediate(
            ((((x_sum_tmp_48d52_23_limb_0.clone() * y_sum_tmp_48d52_24_limb_3.clone())
                + (x_sum_tmp_48d52_23_limb_1.clone() * y_sum_tmp_48d52_24_limb_2.clone()))
                + (x_sum_tmp_48d52_23_limb_2.clone() * y_sum_tmp_48d52_24_limb_1.clone()))
                + (x_sum_tmp_48d52_23_limb_3.clone() * y_sum_tmp_48d52_24_limb_0.clone())),
        );
        let z0_tmp_48d52_25_limb_4 = eval.add_intermediate(
            (((((x_sum_tmp_48d52_23_limb_0.clone() * y_sum_tmp_48d52_24_limb_4.clone())
                + (x_sum_tmp_48d52_23_limb_1.clone() * y_sum_tmp_48d52_24_limb_3.clone()))
                + (x_sum_tmp_48d52_23_limb_2.clone() * y_sum_tmp_48d52_24_limb_2.clone()))
                + (x_sum_tmp_48d52_23_limb_3.clone() * y_sum_tmp_48d52_24_limb_1.clone()))
                + (x_sum_tmp_48d52_23_limb_4.clone() * y_sum_tmp_48d52_24_limb_0.clone())),
        );
        let z0_tmp_48d52_25_limb_5 = eval.add_intermediate(
            ((((((x_sum_tmp_48d52_23_limb_0.clone() * y_sum_tmp_48d52_24_limb_5.clone())
                + (x_sum_tmp_48d52_23_limb_1.clone() * y_sum_tmp_48d52_24_limb_4.clone()))
                + (x_sum_tmp_48d52_23_limb_2.clone() * y_sum_tmp_48d52_24_limb_3.clone()))
                + (x_sum_tmp_48d52_23_limb_3.clone() * y_sum_tmp_48d52_24_limb_2.clone()))
                + (x_sum_tmp_48d52_23_limb_4.clone() * y_sum_tmp_48d52_24_limb_1.clone()))
                + (x_sum_tmp_48d52_23_limb_5.clone() * y_sum_tmp_48d52_24_limb_0.clone())),
        );
        let z0_tmp_48d52_25_limb_6 = eval.add_intermediate(
            (((((((x_sum_tmp_48d52_23_limb_0.clone() * y_sum_tmp_48d52_24_limb_6.clone())
                + (x_sum_tmp_48d52_23_limb_1.clone() * y_sum_tmp_48d52_24_limb_5.clone()))
                + (x_sum_tmp_48d52_23_limb_2.clone() * y_sum_tmp_48d52_24_limb_4.clone()))
                + (x_sum_tmp_48d52_23_limb_3.clone() * y_sum_tmp_48d52_24_limb_3.clone()))
                + (x_sum_tmp_48d52_23_limb_4.clone() * y_sum_tmp_48d52_24_limb_2.clone()))
                + (x_sum_tmp_48d52_23_limb_5.clone() * y_sum_tmp_48d52_24_limb_1.clone()))
                + (x_sum_tmp_48d52_23_limb_6.clone() * y_sum_tmp_48d52_24_limb_0.clone())),
        );
        let z0_tmp_48d52_25_limb_7 = eval.add_intermediate(
            ((((((x_sum_tmp_48d52_23_limb_1.clone() * y_sum_tmp_48d52_24_limb_6.clone())
                + (x_sum_tmp_48d52_23_limb_2.clone() * y_sum_tmp_48d52_24_limb_5.clone()))
                + (x_sum_tmp_48d52_23_limb_3.clone() * y_sum_tmp_48d52_24_limb_4.clone()))
                + (x_sum_tmp_48d52_23_limb_4.clone() * y_sum_tmp_48d52_24_limb_3.clone()))
                + (x_sum_tmp_48d52_23_limb_5.clone() * y_sum_tmp_48d52_24_limb_2.clone()))
                + (x_sum_tmp_48d52_23_limb_6.clone() * y_sum_tmp_48d52_24_limb_1.clone())),
        );
        let z0_tmp_48d52_25_limb_8 = eval.add_intermediate(
            (((((x_sum_tmp_48d52_23_limb_2.clone() * y_sum_tmp_48d52_24_limb_6.clone())
                + (x_sum_tmp_48d52_23_limb_3.clone() * y_sum_tmp_48d52_24_limb_5.clone()))
                + (x_sum_tmp_48d52_23_limb_4.clone() * y_sum_tmp_48d52_24_limb_4.clone()))
                + (x_sum_tmp_48d52_23_limb_5.clone() * y_sum_tmp_48d52_24_limb_3.clone()))
                + (x_sum_tmp_48d52_23_limb_6.clone() * y_sum_tmp_48d52_24_limb_2.clone())),
        );
        let z0_tmp_48d52_25_limb_9 = eval.add_intermediate(
            ((((x_sum_tmp_48d52_23_limb_3.clone() * y_sum_tmp_48d52_24_limb_6.clone())
                + (x_sum_tmp_48d52_23_limb_4.clone() * y_sum_tmp_48d52_24_limb_5.clone()))
                + (x_sum_tmp_48d52_23_limb_5.clone() * y_sum_tmp_48d52_24_limb_4.clone()))
                + (x_sum_tmp_48d52_23_limb_6.clone() * y_sum_tmp_48d52_24_limb_3.clone())),
        );
        let z0_tmp_48d52_25_limb_10 = eval.add_intermediate(
            (((x_sum_tmp_48d52_23_limb_4.clone() * y_sum_tmp_48d52_24_limb_6.clone())
                + (x_sum_tmp_48d52_23_limb_5.clone() * y_sum_tmp_48d52_24_limb_5.clone()))
                + (x_sum_tmp_48d52_23_limb_6.clone() * y_sum_tmp_48d52_24_limb_4.clone())),
        );
        let z0_tmp_48d52_25_limb_11 = eval.add_intermediate(
            ((x_sum_tmp_48d52_23_limb_5.clone() * y_sum_tmp_48d52_24_limb_6.clone())
                + (x_sum_tmp_48d52_23_limb_6.clone() * y_sum_tmp_48d52_24_limb_5.clone())),
        );
        let z0_tmp_48d52_25_limb_12 = eval.add_intermediate(
            (x_sum_tmp_48d52_23_limb_6.clone() * y_sum_tmp_48d52_24_limb_6.clone()),
        );
        let z2_tmp_48d52_26_limb_0 = eval.add_intermediate(
            (x_sum_tmp_48d52_23_limb_7.clone() * y_sum_tmp_48d52_24_limb_7.clone()),
        );
        let z2_tmp_48d52_26_limb_1 = eval.add_intermediate(
            ((x_sum_tmp_48d52_23_limb_7.clone() * y_sum_tmp_48d52_24_limb_8.clone())
                + (x_sum_tmp_48d52_23_limb_8.clone() * y_sum_tmp_48d52_24_limb_7.clone())),
        );
        let z2_tmp_48d52_26_limb_2 = eval.add_intermediate(
            (((x_sum_tmp_48d52_23_limb_7.clone() * y_sum_tmp_48d52_24_limb_9.clone())
                + (x_sum_tmp_48d52_23_limb_8.clone() * y_sum_tmp_48d52_24_limb_8.clone()))
                + (x_sum_tmp_48d52_23_limb_9.clone() * y_sum_tmp_48d52_24_limb_7.clone())),
        );
        let z2_tmp_48d52_26_limb_3 = eval.add_intermediate(
            ((((x_sum_tmp_48d52_23_limb_7.clone() * y_sum_tmp_48d52_24_limb_10.clone())
                + (x_sum_tmp_48d52_23_limb_8.clone() * y_sum_tmp_48d52_24_limb_9.clone()))
                + (x_sum_tmp_48d52_23_limb_9.clone() * y_sum_tmp_48d52_24_limb_8.clone()))
                + (x_sum_tmp_48d52_23_limb_10.clone() * y_sum_tmp_48d52_24_limb_7.clone())),
        );
        let z2_tmp_48d52_26_limb_4 = eval.add_intermediate(
            (((((x_sum_tmp_48d52_23_limb_7.clone() * y_sum_tmp_48d52_24_limb_11.clone())
                + (x_sum_tmp_48d52_23_limb_8.clone() * y_sum_tmp_48d52_24_limb_10.clone()))
                + (x_sum_tmp_48d52_23_limb_9.clone() * y_sum_tmp_48d52_24_limb_9.clone()))
                + (x_sum_tmp_48d52_23_limb_10.clone() * y_sum_tmp_48d52_24_limb_8.clone()))
                + (x_sum_tmp_48d52_23_limb_11.clone() * y_sum_tmp_48d52_24_limb_7.clone())),
        );
        let z2_tmp_48d52_26_limb_5 = eval.add_intermediate(
            ((((((x_sum_tmp_48d52_23_limb_7.clone() * y_sum_tmp_48d52_24_limb_12.clone())
                + (x_sum_tmp_48d52_23_limb_8.clone() * y_sum_tmp_48d52_24_limb_11.clone()))
                + (x_sum_tmp_48d52_23_limb_9.clone() * y_sum_tmp_48d52_24_limb_10.clone()))
                + (x_sum_tmp_48d52_23_limb_10.clone() * y_sum_tmp_48d52_24_limb_9.clone()))
                + (x_sum_tmp_48d52_23_limb_11.clone() * y_sum_tmp_48d52_24_limb_8.clone()))
                + (x_sum_tmp_48d52_23_limb_12.clone() * y_sum_tmp_48d52_24_limb_7.clone())),
        );
        let z2_tmp_48d52_26_limb_6 = eval.add_intermediate(
            (((((((x_sum_tmp_48d52_23_limb_7.clone() * y_sum_tmp_48d52_24_limb_13.clone())
                + (x_sum_tmp_48d52_23_limb_8.clone() * y_sum_tmp_48d52_24_limb_12.clone()))
                + (x_sum_tmp_48d52_23_limb_9.clone() * y_sum_tmp_48d52_24_limb_11.clone()))
                + (x_sum_tmp_48d52_23_limb_10.clone() * y_sum_tmp_48d52_24_limb_10.clone()))
                + (x_sum_tmp_48d52_23_limb_11.clone() * y_sum_tmp_48d52_24_limb_9.clone()))
                + (x_sum_tmp_48d52_23_limb_12.clone() * y_sum_tmp_48d52_24_limb_8.clone()))
                + (x_sum_tmp_48d52_23_limb_13.clone() * y_sum_tmp_48d52_24_limb_7.clone())),
        );
        let z2_tmp_48d52_26_limb_7 = eval.add_intermediate(
            ((((((x_sum_tmp_48d52_23_limb_8.clone() * y_sum_tmp_48d52_24_limb_13.clone())
                + (x_sum_tmp_48d52_23_limb_9.clone() * y_sum_tmp_48d52_24_limb_12.clone()))
                + (x_sum_tmp_48d52_23_limb_10.clone() * y_sum_tmp_48d52_24_limb_11.clone()))
                + (x_sum_tmp_48d52_23_limb_11.clone() * y_sum_tmp_48d52_24_limb_10.clone()))
                + (x_sum_tmp_48d52_23_limb_12.clone() * y_sum_tmp_48d52_24_limb_9.clone()))
                + (x_sum_tmp_48d52_23_limb_13.clone() * y_sum_tmp_48d52_24_limb_8.clone())),
        );
        let z2_tmp_48d52_26_limb_8 = eval.add_intermediate(
            (((((x_sum_tmp_48d52_23_limb_9.clone() * y_sum_tmp_48d52_24_limb_13.clone())
                + (x_sum_tmp_48d52_23_limb_10.clone() * y_sum_tmp_48d52_24_limb_12.clone()))
                + (x_sum_tmp_48d52_23_limb_11.clone() * y_sum_tmp_48d52_24_limb_11.clone()))
                + (x_sum_tmp_48d52_23_limb_12.clone() * y_sum_tmp_48d52_24_limb_10.clone()))
                + (x_sum_tmp_48d52_23_limb_13.clone() * y_sum_tmp_48d52_24_limb_9.clone())),
        );
        let z2_tmp_48d52_26_limb_9 = eval.add_intermediate(
            ((((x_sum_tmp_48d52_23_limb_10.clone() * y_sum_tmp_48d52_24_limb_13.clone())
                + (x_sum_tmp_48d52_23_limb_11.clone() * y_sum_tmp_48d52_24_limb_12.clone()))
                + (x_sum_tmp_48d52_23_limb_12.clone() * y_sum_tmp_48d52_24_limb_11.clone()))
                + (x_sum_tmp_48d52_23_limb_13.clone() * y_sum_tmp_48d52_24_limb_10.clone())),
        );
        let z2_tmp_48d52_26_limb_10 = eval.add_intermediate(
            (((x_sum_tmp_48d52_23_limb_11.clone() * y_sum_tmp_48d52_24_limb_13.clone())
                + (x_sum_tmp_48d52_23_limb_12.clone() * y_sum_tmp_48d52_24_limb_12.clone()))
                + (x_sum_tmp_48d52_23_limb_13.clone() * y_sum_tmp_48d52_24_limb_11.clone())),
        );
        let z2_tmp_48d52_26_limb_11 = eval.add_intermediate(
            ((x_sum_tmp_48d52_23_limb_12.clone() * y_sum_tmp_48d52_24_limb_13.clone())
                + (x_sum_tmp_48d52_23_limb_13.clone() * y_sum_tmp_48d52_24_limb_12.clone())),
        );
        let z2_tmp_48d52_26_limb_12 = eval.add_intermediate(
            (x_sum_tmp_48d52_23_limb_13.clone() * y_sum_tmp_48d52_24_limb_13.clone()),
        );
        let x_sum_tmp_48d52_27_limb_0 = eval.add_intermediate(
            (x_sum_tmp_48d52_23_limb_0.clone() + x_sum_tmp_48d52_23_limb_7.clone()),
        );
        let x_sum_tmp_48d52_27_limb_1 = eval.add_intermediate(
            (x_sum_tmp_48d52_23_limb_1.clone() + x_sum_tmp_48d52_23_limb_8.clone()),
        );
        let x_sum_tmp_48d52_27_limb_2 = eval.add_intermediate(
            (x_sum_tmp_48d52_23_limb_2.clone() + x_sum_tmp_48d52_23_limb_9.clone()),
        );
        let x_sum_tmp_48d52_27_limb_3 = eval.add_intermediate(
            (x_sum_tmp_48d52_23_limb_3.clone() + x_sum_tmp_48d52_23_limb_10.clone()),
        );
        let x_sum_tmp_48d52_27_limb_4 = eval.add_intermediate(
            (x_sum_tmp_48d52_23_limb_4.clone() + x_sum_tmp_48d52_23_limb_11.clone()),
        );
        let x_sum_tmp_48d52_27_limb_5 = eval.add_intermediate(
            (x_sum_tmp_48d52_23_limb_5.clone() + x_sum_tmp_48d52_23_limb_12.clone()),
        );
        let x_sum_tmp_48d52_27_limb_6 = eval.add_intermediate(
            (x_sum_tmp_48d52_23_limb_6.clone() + x_sum_tmp_48d52_23_limb_13.clone()),
        );
        let y_sum_tmp_48d52_28_limb_0 = eval.add_intermediate(
            (y_sum_tmp_48d52_24_limb_0.clone() + y_sum_tmp_48d52_24_limb_7.clone()),
        );
        let y_sum_tmp_48d52_28_limb_1 = eval.add_intermediate(
            (y_sum_tmp_48d52_24_limb_1.clone() + y_sum_tmp_48d52_24_limb_8.clone()),
        );
        let y_sum_tmp_48d52_28_limb_2 = eval.add_intermediate(
            (y_sum_tmp_48d52_24_limb_2.clone() + y_sum_tmp_48d52_24_limb_9.clone()),
        );
        let y_sum_tmp_48d52_28_limb_3 = eval.add_intermediate(
            (y_sum_tmp_48d52_24_limb_3.clone() + y_sum_tmp_48d52_24_limb_10.clone()),
        );
        let y_sum_tmp_48d52_28_limb_4 = eval.add_intermediate(
            (y_sum_tmp_48d52_24_limb_4.clone() + y_sum_tmp_48d52_24_limb_11.clone()),
        );
        let y_sum_tmp_48d52_28_limb_5 = eval.add_intermediate(
            (y_sum_tmp_48d52_24_limb_5.clone() + y_sum_tmp_48d52_24_limb_12.clone()),
        );
        let y_sum_tmp_48d52_28_limb_6 = eval.add_intermediate(
            (y_sum_tmp_48d52_24_limb_6.clone() + y_sum_tmp_48d52_24_limb_13.clone()),
        );

        let conv_tmp_48d52_29_limb_0 =
            eval.add_intermediate((z0_tmp_48d52_21_limb_0.clone() - dst_limb_0_col11.clone()));
        let conv_tmp_48d52_29_limb_1 =
            eval.add_intermediate((z0_tmp_48d52_21_limb_1.clone() - dst_limb_1_col12.clone()));
        let conv_tmp_48d52_29_limb_2 =
            eval.add_intermediate((z0_tmp_48d52_21_limb_2.clone() - dst_limb_2_col13.clone()));
        let conv_tmp_48d52_29_limb_3 =
            eval.add_intermediate((z0_tmp_48d52_21_limb_3.clone() - dst_limb_3_col14.clone()));
        let conv_tmp_48d52_29_limb_4 =
            eval.add_intermediate((z0_tmp_48d52_21_limb_4.clone() - dst_limb_4_col15.clone()));
        let conv_tmp_48d52_29_limb_5 =
            eval.add_intermediate((z0_tmp_48d52_21_limb_5.clone() - dst_limb_5_col16.clone()));
        let conv_tmp_48d52_29_limb_6 =
            eval.add_intermediate((z0_tmp_48d52_21_limb_6.clone() - dst_limb_6_col17.clone()));
        let conv_tmp_48d52_29_limb_7 =
            eval.add_intermediate((z0_tmp_48d52_21_limb_7.clone() - dst_limb_7_col18.clone()));
        let conv_tmp_48d52_29_limb_8 =
            eval.add_intermediate((z0_tmp_48d52_21_limb_8.clone() - dst_limb_8_col19.clone()));
        let conv_tmp_48d52_29_limb_9 =
            eval.add_intermediate((z0_tmp_48d52_21_limb_9.clone() - dst_limb_9_col20.clone()));
        let conv_tmp_48d52_29_limb_10 =
            eval.add_intermediate((z0_tmp_48d52_21_limb_10.clone() - dst_limb_10_col21.clone()));
        let conv_tmp_48d52_29_limb_11 =
            eval.add_intermediate((z0_tmp_48d52_21_limb_11.clone() - dst_limb_11_col22.clone()));
        let conv_tmp_48d52_29_limb_12 =
            eval.add_intermediate((z0_tmp_48d52_21_limb_12.clone() - dst_limb_12_col23.clone()));
        let conv_tmp_48d52_29_limb_13 =
            eval.add_intermediate((z0_tmp_48d52_21_limb_13.clone() - dst_limb_13_col24.clone()));
        let conv_tmp_48d52_29_limb_14 = eval.add_intermediate(
            ((z0_tmp_48d52_21_limb_14.clone()
                + ((z0_tmp_48d52_25_limb_0.clone() - z0_tmp_48d52_21_limb_0.clone())
                    - z2_tmp_48d52_22_limb_0.clone()))
                - dst_limb_14_col25.clone()),
        );
        let conv_tmp_48d52_29_limb_15 = eval.add_intermediate(
            ((z0_tmp_48d52_21_limb_15.clone()
                + ((z0_tmp_48d52_25_limb_1.clone() - z0_tmp_48d52_21_limb_1.clone())
                    - z2_tmp_48d52_22_limb_1.clone()))
                - dst_limb_15_col26.clone()),
        );
        let conv_tmp_48d52_29_limb_16 = eval.add_intermediate(
            ((z0_tmp_48d52_21_limb_16.clone()
                + ((z0_tmp_48d52_25_limb_2.clone() - z0_tmp_48d52_21_limb_2.clone())
                    - z2_tmp_48d52_22_limb_2.clone()))
                - dst_limb_16_col27.clone()),
        );
        let conv_tmp_48d52_29_limb_17 = eval.add_intermediate(
            ((z0_tmp_48d52_21_limb_17.clone()
                + ((z0_tmp_48d52_25_limb_3.clone() - z0_tmp_48d52_21_limb_3.clone())
                    - z2_tmp_48d52_22_limb_3.clone()))
                - dst_limb_17_col28.clone()),
        );
        let conv_tmp_48d52_29_limb_18 = eval.add_intermediate(
            ((z0_tmp_48d52_21_limb_18.clone()
                + ((z0_tmp_48d52_25_limb_4.clone() - z0_tmp_48d52_21_limb_4.clone())
                    - z2_tmp_48d52_22_limb_4.clone()))
                - dst_limb_18_col29.clone()),
        );
        let conv_tmp_48d52_29_limb_19 = eval.add_intermediate(
            ((z0_tmp_48d52_21_limb_19.clone()
                + ((z0_tmp_48d52_25_limb_5.clone() - z0_tmp_48d52_21_limb_5.clone())
                    - z2_tmp_48d52_22_limb_5.clone()))
                - dst_limb_19_col30.clone()),
        );
        let conv_tmp_48d52_29_limb_20 = eval.add_intermediate(
            ((z0_tmp_48d52_21_limb_20.clone()
                + ((z0_tmp_48d52_25_limb_6.clone() - z0_tmp_48d52_21_limb_6.clone())
                    - z2_tmp_48d52_22_limb_6.clone()))
                - dst_limb_20_col31.clone()),
        );
        let conv_tmp_48d52_29_limb_21 = eval.add_intermediate(
            ((z0_tmp_48d52_21_limb_21.clone()
                + (((z0_tmp_48d52_25_limb_7.clone()
                    + (((x_sum_tmp_48d52_27_limb_0.clone()
                        * y_sum_tmp_48d52_28_limb_0.clone())
                        - z0_tmp_48d52_25_limb_0.clone())
                        - z2_tmp_48d52_26_limb_0.clone()))
                    - z0_tmp_48d52_21_limb_7.clone())
                    - z2_tmp_48d52_22_limb_7.clone()))
                - dst_limb_21_col32.clone()),
        );
        let conv_tmp_48d52_29_limb_22 = eval.add_intermediate(
            ((z0_tmp_48d52_21_limb_22.clone()
                + (((z0_tmp_48d52_25_limb_8.clone()
                    + ((((x_sum_tmp_48d52_27_limb_0.clone()
                        * y_sum_tmp_48d52_28_limb_1.clone())
                        + (x_sum_tmp_48d52_27_limb_1.clone()
                            * y_sum_tmp_48d52_28_limb_0.clone()))
                        - z0_tmp_48d52_25_limb_1.clone())
                        - z2_tmp_48d52_26_limb_1.clone()))
                    - z0_tmp_48d52_21_limb_8.clone())
                    - z2_tmp_48d52_22_limb_8.clone()))
                - dst_limb_22_col33.clone()),
        );
        let conv_tmp_48d52_29_limb_23 = eval.add_intermediate(
            ((z0_tmp_48d52_21_limb_23.clone()
                + (((z0_tmp_48d52_25_limb_9.clone()
                    + (((((x_sum_tmp_48d52_27_limb_0.clone()
                        * y_sum_tmp_48d52_28_limb_2.clone())
                        + (x_sum_tmp_48d52_27_limb_1.clone()
                            * y_sum_tmp_48d52_28_limb_1.clone()))
                        + (x_sum_tmp_48d52_27_limb_2.clone()
                            * y_sum_tmp_48d52_28_limb_0.clone()))
                        - z0_tmp_48d52_25_limb_2.clone())
                        - z2_tmp_48d52_26_limb_2.clone()))
                    - z0_tmp_48d52_21_limb_9.clone())
                    - z2_tmp_48d52_22_limb_9.clone()))
                - dst_limb_23_col34.clone()),
        );
        let conv_tmp_48d52_29_limb_24 = eval.add_intermediate(
            ((z0_tmp_48d52_21_limb_24.clone()
                + (((z0_tmp_48d52_25_limb_10.clone()
                    + ((((((x_sum_tmp_48d52_27_limb_0.clone()
                        * y_sum_tmp_48d52_28_limb_3.clone())
                        + (x_sum_tmp_48d52_27_limb_1.clone()
                            * y_sum_tmp_48d52_28_limb_2.clone()))
                        + (x_sum_tmp_48d52_27_limb_2.clone()
                            * y_sum_tmp_48d52_28_limb_1.clone()))
                        + (x_sum_tmp_48d52_27_limb_3.clone()
                            * y_sum_tmp_48d52_28_limb_0.clone()))
                        - z0_tmp_48d52_25_limb_3.clone())
                        - z2_tmp_48d52_26_limb_3.clone()))
                    - z0_tmp_48d52_21_limb_10.clone())
                    - z2_tmp_48d52_22_limb_10.clone()))
                - dst_limb_24_col35.clone()),
        );
        let conv_tmp_48d52_29_limb_25 = eval.add_intermediate(
            ((z0_tmp_48d52_21_limb_25.clone()
                + (((z0_tmp_48d52_25_limb_11.clone()
                    + (((((((x_sum_tmp_48d52_27_limb_0.clone()
                        * y_sum_tmp_48d52_28_limb_4.clone())
                        + (x_sum_tmp_48d52_27_limb_1.clone()
                            * y_sum_tmp_48d52_28_limb_3.clone()))
                        + (x_sum_tmp_48d52_27_limb_2.clone()
                            * y_sum_tmp_48d52_28_limb_2.clone()))
                        + (x_sum_tmp_48d52_27_limb_3.clone()
                            * y_sum_tmp_48d52_28_limb_1.clone()))
                        + (x_sum_tmp_48d52_27_limb_4.clone()
                            * y_sum_tmp_48d52_28_limb_0.clone()))
                        - z0_tmp_48d52_25_limb_4.clone())
                        - z2_tmp_48d52_26_limb_4.clone()))
                    - z0_tmp_48d52_21_limb_11.clone())
                    - z2_tmp_48d52_22_limb_11.clone()))
                - dst_limb_25_col36.clone()),
        );
        let conv_tmp_48d52_29_limb_26 = eval.add_intermediate(
            ((z0_tmp_48d52_21_limb_26.clone()
                + (((z0_tmp_48d52_25_limb_12.clone()
                    + ((((((((x_sum_tmp_48d52_27_limb_0.clone()
                        * y_sum_tmp_48d52_28_limb_5.clone())
                        + (x_sum_tmp_48d52_27_limb_1.clone()
                            * y_sum_tmp_48d52_28_limb_4.clone()))
                        + (x_sum_tmp_48d52_27_limb_2.clone()
                            * y_sum_tmp_48d52_28_limb_3.clone()))
                        + (x_sum_tmp_48d52_27_limb_3.clone()
                            * y_sum_tmp_48d52_28_limb_2.clone()))
                        + (x_sum_tmp_48d52_27_limb_4.clone()
                            * y_sum_tmp_48d52_28_limb_1.clone()))
                        + (x_sum_tmp_48d52_27_limb_5.clone()
                            * y_sum_tmp_48d52_28_limb_0.clone()))
                        - z0_tmp_48d52_25_limb_5.clone())
                        - z2_tmp_48d52_26_limb_5.clone()))
                    - z0_tmp_48d52_21_limb_12.clone())
                    - z2_tmp_48d52_22_limb_12.clone()))
                - dst_limb_26_col37.clone()),
        );
        let conv_tmp_48d52_29_limb_27 = eval.add_intermediate(
            ((((((((((((x_sum_tmp_48d52_27_limb_0.clone()
                * y_sum_tmp_48d52_28_limb_6.clone())
                + (x_sum_tmp_48d52_27_limb_1.clone()
                    * y_sum_tmp_48d52_28_limb_5.clone()))
                + (x_sum_tmp_48d52_27_limb_2.clone() * y_sum_tmp_48d52_28_limb_4.clone()))
                + (x_sum_tmp_48d52_27_limb_3.clone() * y_sum_tmp_48d52_28_limb_3.clone()))
                + (x_sum_tmp_48d52_27_limb_4.clone() * y_sum_tmp_48d52_28_limb_2.clone()))
                + (x_sum_tmp_48d52_27_limb_5.clone() * y_sum_tmp_48d52_28_limb_1.clone()))
                + (x_sum_tmp_48d52_27_limb_6.clone() * y_sum_tmp_48d52_28_limb_0.clone()))
                - z0_tmp_48d52_25_limb_6.clone())
                - z2_tmp_48d52_26_limb_6.clone())
                - z0_tmp_48d52_21_limb_13.clone())
                - z2_tmp_48d52_22_limb_13.clone())
                - dst_limb_27_col38.clone()),
        );
        let conv_tmp_48d52_29_limb_28 = eval.add_intermediate(
            (z2_tmp_48d52_22_limb_0.clone()
                + (((z2_tmp_48d52_26_limb_0.clone()
                    + ((((((((x_sum_tmp_48d52_27_limb_1.clone()
                        * y_sum_tmp_48d52_28_limb_6.clone())
                        + (x_sum_tmp_48d52_27_limb_2.clone()
                            * y_sum_tmp_48d52_28_limb_5.clone()))
                        + (x_sum_tmp_48d52_27_limb_3.clone()
                            * y_sum_tmp_48d52_28_limb_4.clone()))
                        + (x_sum_tmp_48d52_27_limb_4.clone()
                            * y_sum_tmp_48d52_28_limb_3.clone()))
                        + (x_sum_tmp_48d52_27_limb_5.clone()
                            * y_sum_tmp_48d52_28_limb_2.clone()))
                        + (x_sum_tmp_48d52_27_limb_6.clone()
                            * y_sum_tmp_48d52_28_limb_1.clone()))
                        - z0_tmp_48d52_25_limb_7.clone())
                        - z2_tmp_48d52_26_limb_7.clone()))
                    - z0_tmp_48d52_21_limb_14.clone())
                    - z2_tmp_48d52_22_limb_14.clone())),
        );
        let conv_tmp_48d52_29_limb_29 = eval.add_intermediate(
            (z2_tmp_48d52_22_limb_1.clone()
                + (((z2_tmp_48d52_26_limb_1.clone()
                    + (((((((x_sum_tmp_48d52_27_limb_2.clone()
                        * y_sum_tmp_48d52_28_limb_6.clone())
                        + (x_sum_tmp_48d52_27_limb_3.clone()
                            * y_sum_tmp_48d52_28_limb_5.clone()))
                        + (x_sum_tmp_48d52_27_limb_4.clone()
                            * y_sum_tmp_48d52_28_limb_4.clone()))
                        + (x_sum_tmp_48d52_27_limb_5.clone()
                            * y_sum_tmp_48d52_28_limb_3.clone()))
                        + (x_sum_tmp_48d52_27_limb_6.clone()
                            * y_sum_tmp_48d52_28_limb_2.clone()))
                        - z0_tmp_48d52_25_limb_8.clone())
                        - z2_tmp_48d52_26_limb_8.clone()))
                    - z0_tmp_48d52_21_limb_15.clone())
                    - z2_tmp_48d52_22_limb_15.clone())),
        );
        let conv_tmp_48d52_29_limb_30 = eval.add_intermediate(
            (z2_tmp_48d52_22_limb_2.clone()
                + (((z2_tmp_48d52_26_limb_2.clone()
                    + ((((((x_sum_tmp_48d52_27_limb_3.clone()
                        * y_sum_tmp_48d52_28_limb_6.clone())
                        + (x_sum_tmp_48d52_27_limb_4.clone()
                            * y_sum_tmp_48d52_28_limb_5.clone()))
                        + (x_sum_tmp_48d52_27_limb_5.clone()
                            * y_sum_tmp_48d52_28_limb_4.clone()))
                        + (x_sum_tmp_48d52_27_limb_6.clone()
                            * y_sum_tmp_48d52_28_limb_3.clone()))
                        - z0_tmp_48d52_25_limb_9.clone())
                        - z2_tmp_48d52_26_limb_9.clone()))
                    - z0_tmp_48d52_21_limb_16.clone())
                    - z2_tmp_48d52_22_limb_16.clone())),
        );
        let conv_tmp_48d52_29_limb_31 = eval.add_intermediate(
            (z2_tmp_48d52_22_limb_3.clone()
                + (((z2_tmp_48d52_26_limb_3.clone()
                    + (((((x_sum_tmp_48d52_27_limb_4.clone()
                        * y_sum_tmp_48d52_28_limb_6.clone())
                        + (x_sum_tmp_48d52_27_limb_5.clone()
                            * y_sum_tmp_48d52_28_limb_5.clone()))
                        + (x_sum_tmp_48d52_27_limb_6.clone()
                            * y_sum_tmp_48d52_28_limb_4.clone()))
                        - z0_tmp_48d52_25_limb_10.clone())
                        - z2_tmp_48d52_26_limb_10.clone()))
                    - z0_tmp_48d52_21_limb_17.clone())
                    - z2_tmp_48d52_22_limb_17.clone())),
        );
        let conv_tmp_48d52_29_limb_32 = eval.add_intermediate(
            (z2_tmp_48d52_22_limb_4.clone()
                + (((z2_tmp_48d52_26_limb_4.clone()
                    + ((((x_sum_tmp_48d52_27_limb_5.clone()
                        * y_sum_tmp_48d52_28_limb_6.clone())
                        + (x_sum_tmp_48d52_27_limb_6.clone()
                            * y_sum_tmp_48d52_28_limb_5.clone()))
                        - z0_tmp_48d52_25_limb_11.clone())
                        - z2_tmp_48d52_26_limb_11.clone()))
                    - z0_tmp_48d52_21_limb_18.clone())
                    - z2_tmp_48d52_22_limb_18.clone())),
        );
        let conv_tmp_48d52_29_limb_33 = eval.add_intermediate(
            (z2_tmp_48d52_22_limb_5.clone()
                + (((z2_tmp_48d52_26_limb_5.clone()
                    + (((x_sum_tmp_48d52_27_limb_6.clone()
                        * y_sum_tmp_48d52_28_limb_6.clone())
                        - z0_tmp_48d52_25_limb_12.clone())
                        - z2_tmp_48d52_26_limb_12.clone()))
                    - z0_tmp_48d52_21_limb_19.clone())
                    - z2_tmp_48d52_22_limb_19.clone())),
        );
        let conv_tmp_48d52_29_limb_34 = eval.add_intermediate(
            (z2_tmp_48d52_22_limb_6.clone()
                + ((z2_tmp_48d52_26_limb_6.clone() - z0_tmp_48d52_21_limb_20.clone())
                    - z2_tmp_48d52_22_limb_20.clone())),
        );
        let conv_tmp_48d52_29_limb_35 = eval.add_intermediate(
            (z2_tmp_48d52_22_limb_7.clone()
                + ((z2_tmp_48d52_26_limb_7.clone() - z0_tmp_48d52_21_limb_21.clone())
                    - z2_tmp_48d52_22_limb_21.clone())),
        );
        let conv_tmp_48d52_29_limb_36 = eval.add_intermediate(
            (z2_tmp_48d52_22_limb_8.clone()
                + ((z2_tmp_48d52_26_limb_8.clone() - z0_tmp_48d52_21_limb_22.clone())
                    - z2_tmp_48d52_22_limb_22.clone())),
        );
        let conv_tmp_48d52_29_limb_37 = eval.add_intermediate(
            (z2_tmp_48d52_22_limb_9.clone()
                + ((z2_tmp_48d52_26_limb_9.clone() - z0_tmp_48d52_21_limb_23.clone())
                    - z2_tmp_48d52_22_limb_23.clone())),
        );
        let conv_tmp_48d52_29_limb_38 = eval.add_intermediate(
            (z2_tmp_48d52_22_limb_10.clone()
                + ((z2_tmp_48d52_26_limb_10.clone() - z0_tmp_48d52_21_limb_24.clone())
                    - z2_tmp_48d52_22_limb_24.clone())),
        );
        let conv_tmp_48d52_29_limb_39 = eval.add_intermediate(
            (z2_tmp_48d52_22_limb_11.clone()
                + ((z2_tmp_48d52_26_limb_11.clone() - z0_tmp_48d52_21_limb_25.clone())
                    - z2_tmp_48d52_22_limb_25.clone())),
        );
        let conv_tmp_48d52_29_limb_40 = eval.add_intermediate(
            (z2_tmp_48d52_22_limb_12.clone()
                + ((z2_tmp_48d52_26_limb_12.clone() - z0_tmp_48d52_21_limb_26.clone())
                    - z2_tmp_48d52_22_limb_26.clone())),
        );
        let conv_tmp_48d52_29_limb_41 = eval.add_intermediate(z2_tmp_48d52_22_limb_13.clone());
        let conv_tmp_48d52_29_limb_42 = eval.add_intermediate(z2_tmp_48d52_22_limb_14.clone());
        let conv_tmp_48d52_29_limb_43 = eval.add_intermediate(z2_tmp_48d52_22_limb_15.clone());
        let conv_tmp_48d52_29_limb_44 = eval.add_intermediate(z2_tmp_48d52_22_limb_16.clone());
        let conv_tmp_48d52_29_limb_45 = eval.add_intermediate(z2_tmp_48d52_22_limb_17.clone());
        let conv_tmp_48d52_29_limb_46 = eval.add_intermediate(z2_tmp_48d52_22_limb_18.clone());
        let conv_tmp_48d52_29_limb_47 = eval.add_intermediate(z2_tmp_48d52_22_limb_19.clone());
        let conv_tmp_48d52_29_limb_48 = eval.add_intermediate(z2_tmp_48d52_22_limb_20.clone());
        let conv_tmp_48d52_29_limb_49 = eval.add_intermediate(z2_tmp_48d52_22_limb_21.clone());
        let conv_tmp_48d52_29_limb_50 = eval.add_intermediate(z2_tmp_48d52_22_limb_22.clone());
        let conv_tmp_48d52_29_limb_51 = eval.add_intermediate(z2_tmp_48d52_22_limb_23.clone());
        let conv_tmp_48d52_29_limb_52 = eval.add_intermediate(z2_tmp_48d52_22_limb_24.clone());
        let conv_tmp_48d52_29_limb_53 = eval.add_intermediate(z2_tmp_48d52_22_limb_25.clone());
        let conv_tmp_48d52_29_limb_54 = eval.add_intermediate(z2_tmp_48d52_22_limb_26.clone());
        let conv_mod_tmp_48d52_30_limb_0 = eval.add_intermediate(
            (((M31_32.clone() * conv_tmp_48d52_29_limb_0.clone())
                - (M31_4.clone() * conv_tmp_48d52_29_limb_21.clone()))
                + (M31_8.clone() * conv_tmp_48d52_29_limb_49.clone())),
        );
        let conv_mod_tmp_48d52_30_limb_1 = eval.add_intermediate(
            (((conv_tmp_48d52_29_limb_0.clone()
                + (M31_32.clone() * conv_tmp_48d52_29_limb_1.clone()))
                - (M31_4.clone() * conv_tmp_48d52_29_limb_22.clone()))
                + (M31_8.clone() * conv_tmp_48d52_29_limb_50.clone())),
        );
        let conv_mod_tmp_48d52_30_limb_2 = eval.add_intermediate(
            (((conv_tmp_48d52_29_limb_1.clone()
                + (M31_32.clone() * conv_tmp_48d52_29_limb_2.clone()))
                - (M31_4.clone() * conv_tmp_48d52_29_limb_23.clone()))
                + (M31_8.clone() * conv_tmp_48d52_29_limb_51.clone())),
        );
        let conv_mod_tmp_48d52_30_limb_3 = eval.add_intermediate(
            (((conv_tmp_48d52_29_limb_2.clone()
                + (M31_32.clone() * conv_tmp_48d52_29_limb_3.clone()))
                - (M31_4.clone() * conv_tmp_48d52_29_limb_24.clone()))
                + (M31_8.clone() * conv_tmp_48d52_29_limb_52.clone())),
        );
        let conv_mod_tmp_48d52_30_limb_4 = eval.add_intermediate(
            (((conv_tmp_48d52_29_limb_3.clone()
                + (M31_32.clone() * conv_tmp_48d52_29_limb_4.clone()))
                - (M31_4.clone() * conv_tmp_48d52_29_limb_25.clone()))
                + (M31_8.clone() * conv_tmp_48d52_29_limb_53.clone())),
        );
        let conv_mod_tmp_48d52_30_limb_5 = eval.add_intermediate(
            (((conv_tmp_48d52_29_limb_4.clone()
                + (M31_32.clone() * conv_tmp_48d52_29_limb_5.clone()))
                - (M31_4.clone() * conv_tmp_48d52_29_limb_26.clone()))
                + (M31_8.clone() * conv_tmp_48d52_29_limb_54.clone())),
        );
        let conv_mod_tmp_48d52_30_limb_6 = eval.add_intermediate(
            ((conv_tmp_48d52_29_limb_5.clone()
                + (M31_32.clone() * conv_tmp_48d52_29_limb_6.clone()))
                - (M31_4.clone() * conv_tmp_48d52_29_limb_27.clone())),
        );
        let conv_mod_tmp_48d52_30_limb_7 = eval.add_intermediate(
            ((((M31_2.clone() * conv_tmp_48d52_29_limb_0.clone())
                + conv_tmp_48d52_29_limb_6.clone())
                + (M31_32.clone() * conv_tmp_48d52_29_limb_7.clone()))
                - (M31_4.clone() * conv_tmp_48d52_29_limb_28.clone())),
        );
        let conv_mod_tmp_48d52_30_limb_8 = eval.add_intermediate(
            ((((M31_2.clone() * conv_tmp_48d52_29_limb_1.clone())
                + conv_tmp_48d52_29_limb_7.clone())
                + (M31_32.clone() * conv_tmp_48d52_29_limb_8.clone()))
                - (M31_4.clone() * conv_tmp_48d52_29_limb_29.clone())),
        );
        let conv_mod_tmp_48d52_30_limb_9 = eval.add_intermediate(
            ((((M31_2.clone() * conv_tmp_48d52_29_limb_2.clone())
                + conv_tmp_48d52_29_limb_8.clone())
                + (M31_32.clone() * conv_tmp_48d52_29_limb_9.clone()))
                - (M31_4.clone() * conv_tmp_48d52_29_limb_30.clone())),
        );
        let conv_mod_tmp_48d52_30_limb_10 = eval.add_intermediate(
            ((((M31_2.clone() * conv_tmp_48d52_29_limb_3.clone())
                + conv_tmp_48d52_29_limb_9.clone())
                + (M31_32.clone() * conv_tmp_48d52_29_limb_10.clone()))
                - (M31_4.clone() * conv_tmp_48d52_29_limb_31.clone())),
        );
        let conv_mod_tmp_48d52_30_limb_11 = eval.add_intermediate(
            ((((M31_2.clone() * conv_tmp_48d52_29_limb_4.clone())
                + conv_tmp_48d52_29_limb_10.clone())
                + (M31_32.clone() * conv_tmp_48d52_29_limb_11.clone()))
                - (M31_4.clone() * conv_tmp_48d52_29_limb_32.clone())),
        );
        let conv_mod_tmp_48d52_30_limb_12 = eval.add_intermediate(
            ((((M31_2.clone() * conv_tmp_48d52_29_limb_5.clone())
                + conv_tmp_48d52_29_limb_11.clone())
                + (M31_32.clone() * conv_tmp_48d52_29_limb_12.clone()))
                - (M31_4.clone() * conv_tmp_48d52_29_limb_33.clone())),
        );
        let conv_mod_tmp_48d52_30_limb_13 = eval.add_intermediate(
            ((((M31_2.clone() * conv_tmp_48d52_29_limb_6.clone())
                + conv_tmp_48d52_29_limb_12.clone())
                + (M31_32.clone() * conv_tmp_48d52_29_limb_13.clone()))
                - (M31_4.clone() * conv_tmp_48d52_29_limb_34.clone())),
        );
        let conv_mod_tmp_48d52_30_limb_14 = eval.add_intermediate(
            ((((M31_2.clone() * conv_tmp_48d52_29_limb_7.clone())
                + conv_tmp_48d52_29_limb_13.clone())
                + (M31_32.clone() * conv_tmp_48d52_29_limb_14.clone()))
                - (M31_4.clone() * conv_tmp_48d52_29_limb_35.clone())),
        );
        let conv_mod_tmp_48d52_30_limb_15 = eval.add_intermediate(
            ((((M31_2.clone() * conv_tmp_48d52_29_limb_8.clone())
                + conv_tmp_48d52_29_limb_14.clone())
                + (M31_32.clone() * conv_tmp_48d52_29_limb_15.clone()))
                - (M31_4.clone() * conv_tmp_48d52_29_limb_36.clone())),
        );
        let conv_mod_tmp_48d52_30_limb_16 = eval.add_intermediate(
            ((((M31_2.clone() * conv_tmp_48d52_29_limb_9.clone())
                + conv_tmp_48d52_29_limb_15.clone())
                + (M31_32.clone() * conv_tmp_48d52_29_limb_16.clone()))
                - (M31_4.clone() * conv_tmp_48d52_29_limb_37.clone())),
        );
        let conv_mod_tmp_48d52_30_limb_17 = eval.add_intermediate(
            ((((M31_2.clone() * conv_tmp_48d52_29_limb_10.clone())
                + conv_tmp_48d52_29_limb_16.clone())
                + (M31_32.clone() * conv_tmp_48d52_29_limb_17.clone()))
                - (M31_4.clone() * conv_tmp_48d52_29_limb_38.clone())),
        );
        let conv_mod_tmp_48d52_30_limb_18 = eval.add_intermediate(
            ((((M31_2.clone() * conv_tmp_48d52_29_limb_11.clone())
                + conv_tmp_48d52_29_limb_17.clone())
                + (M31_32.clone() * conv_tmp_48d52_29_limb_18.clone()))
                - (M31_4.clone() * conv_tmp_48d52_29_limb_39.clone())),
        );
        let conv_mod_tmp_48d52_30_limb_19 = eval.add_intermediate(
            ((((M31_2.clone() * conv_tmp_48d52_29_limb_12.clone())
                + conv_tmp_48d52_29_limb_18.clone())
                + (M31_32.clone() * conv_tmp_48d52_29_limb_19.clone()))
                - (M31_4.clone() * conv_tmp_48d52_29_limb_40.clone())),
        );
        let conv_mod_tmp_48d52_30_limb_20 = eval.add_intermediate(
            ((((M31_2.clone() * conv_tmp_48d52_29_limb_13.clone())
                + conv_tmp_48d52_29_limb_19.clone())
                + (M31_32.clone() * conv_tmp_48d52_29_limb_20.clone()))
                - (M31_4.clone() * conv_tmp_48d52_29_limb_41.clone())),
        );
        let conv_mod_tmp_48d52_30_limb_21 = eval.add_intermediate(
            ((((M31_2.clone() * conv_tmp_48d52_29_limb_14.clone())
                + conv_tmp_48d52_29_limb_20.clone())
                - (M31_4.clone() * conv_tmp_48d52_29_limb_42.clone()))
                + (M31_64.clone() * conv_tmp_48d52_29_limb_49.clone())),
        );
        let conv_mod_tmp_48d52_30_limb_22 = eval.add_intermediate(
            ((((M31_2.clone() * conv_tmp_48d52_29_limb_15.clone())
                - (M31_4.clone() * conv_tmp_48d52_29_limb_43.clone()))
                + (M31_2.clone() * conv_tmp_48d52_29_limb_49.clone()))
                + (M31_64.clone() * conv_tmp_48d52_29_limb_50.clone())),
        );
        let conv_mod_tmp_48d52_30_limb_23 = eval.add_intermediate(
            ((((M31_2.clone() * conv_tmp_48d52_29_limb_16.clone())
                - (M31_4.clone() * conv_tmp_48d52_29_limb_44.clone()))
                + (M31_2.clone() * conv_tmp_48d52_29_limb_50.clone()))
                + (M31_64.clone() * conv_tmp_48d52_29_limb_51.clone())),
        );
        let conv_mod_tmp_48d52_30_limb_24 = eval.add_intermediate(
            ((((M31_2.clone() * conv_tmp_48d52_29_limb_17.clone())
                - (M31_4.clone() * conv_tmp_48d52_29_limb_45.clone()))
                + (M31_2.clone() * conv_tmp_48d52_29_limb_51.clone()))
                + (M31_64.clone() * conv_tmp_48d52_29_limb_52.clone())),
        );
        let conv_mod_tmp_48d52_30_limb_25 = eval.add_intermediate(
            ((((M31_2.clone() * conv_tmp_48d52_29_limb_18.clone())
                - (M31_4.clone() * conv_tmp_48d52_29_limb_46.clone()))
                + (M31_2.clone() * conv_tmp_48d52_29_limb_52.clone()))
                + (M31_64.clone() * conv_tmp_48d52_29_limb_53.clone())),
        );
        let conv_mod_tmp_48d52_30_limb_26 = eval.add_intermediate(
            ((((M31_2.clone() * conv_tmp_48d52_29_limb_19.clone())
                - (M31_4.clone() * conv_tmp_48d52_29_limb_47.clone()))
                + (M31_2.clone() * conv_tmp_48d52_29_limb_53.clone()))
                + (M31_64.clone() * conv_tmp_48d52_29_limb_54.clone())),
        );
        let conv_mod_tmp_48d52_30_limb_27 = eval.add_intermediate(
            (((M31_2.clone() * conv_tmp_48d52_29_limb_20.clone())
                - (M31_4.clone() * conv_tmp_48d52_29_limb_48.clone()))
                + (M31_2.clone() * conv_tmp_48d52_29_limb_54.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(k_col97.clone() + M31_262144.clone())],
        ));

        eval.add_constraint(
            ((carry_0_col98.clone() * M31_512.clone())
                - (conv_mod_tmp_48d52_30_limb_0.clone() - k_col97.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_0_col98.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_1_col99.clone() * M31_512.clone())
                - (conv_mod_tmp_48d52_30_limb_1.clone() + carry_0_col98.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_1_col99.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_2_col100.clone() * M31_512.clone())
                - (conv_mod_tmp_48d52_30_limb_2.clone() + carry_1_col99.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_2_col100.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_3_col101.clone() * M31_512.clone())
                - (conv_mod_tmp_48d52_30_limb_3.clone() + carry_2_col100.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_3_col101.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_4_col102.clone() * M31_512.clone())
                - (conv_mod_tmp_48d52_30_limb_4.clone() + carry_3_col101.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_4_col102.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_5_col103.clone() * M31_512.clone())
                - (conv_mod_tmp_48d52_30_limb_5.clone() + carry_4_col102.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_5_col103.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_6_col104.clone() * M31_512.clone())
                - (conv_mod_tmp_48d52_30_limb_6.clone() + carry_5_col103.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_6_col104.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_7_col105.clone() * M31_512.clone())
                - (conv_mod_tmp_48d52_30_limb_7.clone() + carry_6_col104.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_7_col105.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_8_col106.clone() * M31_512.clone())
                - (conv_mod_tmp_48d52_30_limb_8.clone() + carry_7_col105.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_8_col106.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_9_col107.clone() * M31_512.clone())
                - (conv_mod_tmp_48d52_30_limb_9.clone() + carry_8_col106.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_9_col107.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_10_col108.clone() * M31_512.clone())
                - (conv_mod_tmp_48d52_30_limb_10.clone() + carry_9_col107.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_10_col108.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_11_col109.clone() * M31_512.clone())
                - (conv_mod_tmp_48d52_30_limb_11.clone() + carry_10_col108.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_11_col109.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_12_col110.clone() * M31_512.clone())
                - (conv_mod_tmp_48d52_30_limb_12.clone() + carry_11_col109.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_12_col110.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_13_col111.clone() * M31_512.clone())
                - (conv_mod_tmp_48d52_30_limb_13.clone() + carry_12_col110.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_13_col111.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_14_col112.clone() * M31_512.clone())
                - (conv_mod_tmp_48d52_30_limb_14.clone() + carry_13_col111.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_14_col112.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_15_col113.clone() * M31_512.clone())
                - (conv_mod_tmp_48d52_30_limb_15.clone() + carry_14_col112.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_15_col113.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_16_col114.clone() * M31_512.clone())
                - (conv_mod_tmp_48d52_30_limb_16.clone() + carry_15_col113.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_16_col114.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_17_col115.clone() * M31_512.clone())
                - (conv_mod_tmp_48d52_30_limb_17.clone() + carry_16_col114.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_17_col115.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_18_col116.clone() * M31_512.clone())
                - (conv_mod_tmp_48d52_30_limb_18.clone() + carry_17_col115.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_18_col116.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_19_col117.clone() * M31_512.clone())
                - (conv_mod_tmp_48d52_30_limb_19.clone() + carry_18_col116.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_19_col117.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_20_col118.clone() * M31_512.clone())
                - (conv_mod_tmp_48d52_30_limb_20.clone() + carry_19_col117.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_20_col118.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_21_col119.clone() * M31_512.clone())
                - ((conv_mod_tmp_48d52_30_limb_21.clone() - (M31_136.clone() * k_col97.clone()))
                    + carry_20_col118.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_21_col119.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_22_col120.clone() * M31_512.clone())
                - (conv_mod_tmp_48d52_30_limb_22.clone() + carry_21_col119.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_22_col120.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_23_col121.clone() * M31_512.clone())
                - (conv_mod_tmp_48d52_30_limb_23.clone() + carry_22_col120.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_23_col121.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_24_col122.clone() * M31_512.clone())
                - (conv_mod_tmp_48d52_30_limb_24.clone() + carry_23_col121.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_24_col122.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_25_col123.clone() * M31_512.clone())
                - (conv_mod_tmp_48d52_30_limb_25.clone() + carry_24_col122.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_25_col123.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_26_col124.clone() * M31_512.clone())
                - (conv_mod_tmp_48d52_30_limb_26.clone() + carry_25_col123.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_26_col124.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((conv_mod_tmp_48d52_30_limb_27.clone() - (M31_256.clone() * k_col97.clone()))
                + carry_26_col124.clone()),
        );

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
                (input_pc_col0.clone() + M31_2.clone()),
                (input_ap_col1.clone() + ap_update_add_1_col7.clone()),
                input_fp_col2.clone(),
            ],
        ));

        eval.finalize_logup_in_pairs();
        eval
    }
}
