use crate::components::prelude::constraint_eval::*;

pub struct Eval {
    pub claim: Claim,
    pub memory_address_to_id_lookup_elements: relations::MemoryAddressToId,
    pub memory_id_to_big_lookup_elements: relations::MemoryIdToBig,
    pub opcodes_lookup_elements: relations::Opcodes,
    pub verify_instruction_lookup_elements: relations::VerifyInstruction,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Claim {
    pub log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; 103];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 5];
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
        let M31_136 = E::F::from(M31::from(136));
        let M31_256 = E::F::from(M31::from(256));
        let M31_32768 = E::F::from(M31::from(32768));
        let M31_4194304 = E::F::from(M31::from(4194304));
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
        let mem_dst_base_col11 = eval.next_trace_mask();
        let mem0_base_col12 = eval.next_trace_mask();
        let mem1_base_col13 = eval.next_trace_mask();
        let dst_id_col14 = eval.next_trace_mask();
        let dst_limb_0_col15 = eval.next_trace_mask();
        let dst_limb_1_col16 = eval.next_trace_mask();
        let dst_limb_2_col17 = eval.next_trace_mask();
        let dst_limb_3_col18 = eval.next_trace_mask();
        let dst_limb_4_col19 = eval.next_trace_mask();
        let dst_limb_5_col20 = eval.next_trace_mask();
        let dst_limb_6_col21 = eval.next_trace_mask();
        let dst_limb_7_col22 = eval.next_trace_mask();
        let dst_limb_8_col23 = eval.next_trace_mask();
        let dst_limb_9_col24 = eval.next_trace_mask();
        let dst_limb_10_col25 = eval.next_trace_mask();
        let dst_limb_11_col26 = eval.next_trace_mask();
        let dst_limb_12_col27 = eval.next_trace_mask();
        let dst_limb_13_col28 = eval.next_trace_mask();
        let dst_limb_14_col29 = eval.next_trace_mask();
        let dst_limb_15_col30 = eval.next_trace_mask();
        let dst_limb_16_col31 = eval.next_trace_mask();
        let dst_limb_17_col32 = eval.next_trace_mask();
        let dst_limb_18_col33 = eval.next_trace_mask();
        let dst_limb_19_col34 = eval.next_trace_mask();
        let dst_limb_20_col35 = eval.next_trace_mask();
        let dst_limb_21_col36 = eval.next_trace_mask();
        let dst_limb_22_col37 = eval.next_trace_mask();
        let dst_limb_23_col38 = eval.next_trace_mask();
        let dst_limb_24_col39 = eval.next_trace_mask();
        let dst_limb_25_col40 = eval.next_trace_mask();
        let dst_limb_26_col41 = eval.next_trace_mask();
        let dst_limb_27_col42 = eval.next_trace_mask();
        let op0_id_col43 = eval.next_trace_mask();
        let op0_limb_0_col44 = eval.next_trace_mask();
        let op0_limb_1_col45 = eval.next_trace_mask();
        let op0_limb_2_col46 = eval.next_trace_mask();
        let op0_limb_3_col47 = eval.next_trace_mask();
        let op0_limb_4_col48 = eval.next_trace_mask();
        let op0_limb_5_col49 = eval.next_trace_mask();
        let op0_limb_6_col50 = eval.next_trace_mask();
        let op0_limb_7_col51 = eval.next_trace_mask();
        let op0_limb_8_col52 = eval.next_trace_mask();
        let op0_limb_9_col53 = eval.next_trace_mask();
        let op0_limb_10_col54 = eval.next_trace_mask();
        let op0_limb_11_col55 = eval.next_trace_mask();
        let op0_limb_12_col56 = eval.next_trace_mask();
        let op0_limb_13_col57 = eval.next_trace_mask();
        let op0_limb_14_col58 = eval.next_trace_mask();
        let op0_limb_15_col59 = eval.next_trace_mask();
        let op0_limb_16_col60 = eval.next_trace_mask();
        let op0_limb_17_col61 = eval.next_trace_mask();
        let op0_limb_18_col62 = eval.next_trace_mask();
        let op0_limb_19_col63 = eval.next_trace_mask();
        let op0_limb_20_col64 = eval.next_trace_mask();
        let op0_limb_21_col65 = eval.next_trace_mask();
        let op0_limb_22_col66 = eval.next_trace_mask();
        let op0_limb_23_col67 = eval.next_trace_mask();
        let op0_limb_24_col68 = eval.next_trace_mask();
        let op0_limb_25_col69 = eval.next_trace_mask();
        let op0_limb_26_col70 = eval.next_trace_mask();
        let op0_limb_27_col71 = eval.next_trace_mask();
        let op1_id_col72 = eval.next_trace_mask();
        let op1_limb_0_col73 = eval.next_trace_mask();
        let op1_limb_1_col74 = eval.next_trace_mask();
        let op1_limb_2_col75 = eval.next_trace_mask();
        let op1_limb_3_col76 = eval.next_trace_mask();
        let op1_limb_4_col77 = eval.next_trace_mask();
        let op1_limb_5_col78 = eval.next_trace_mask();
        let op1_limb_6_col79 = eval.next_trace_mask();
        let op1_limb_7_col80 = eval.next_trace_mask();
        let op1_limb_8_col81 = eval.next_trace_mask();
        let op1_limb_9_col82 = eval.next_trace_mask();
        let op1_limb_10_col83 = eval.next_trace_mask();
        let op1_limb_11_col84 = eval.next_trace_mask();
        let op1_limb_12_col85 = eval.next_trace_mask();
        let op1_limb_13_col86 = eval.next_trace_mask();
        let op1_limb_14_col87 = eval.next_trace_mask();
        let op1_limb_15_col88 = eval.next_trace_mask();
        let op1_limb_16_col89 = eval.next_trace_mask();
        let op1_limb_17_col90 = eval.next_trace_mask();
        let op1_limb_18_col91 = eval.next_trace_mask();
        let op1_limb_19_col92 = eval.next_trace_mask();
        let op1_limb_20_col93 = eval.next_trace_mask();
        let op1_limb_21_col94 = eval.next_trace_mask();
        let op1_limb_22_col95 = eval.next_trace_mask();
        let op1_limb_23_col96 = eval.next_trace_mask();
        let op1_limb_24_col97 = eval.next_trace_mask();
        let op1_limb_25_col98 = eval.next_trace_mask();
        let op1_limb_26_col99 = eval.next_trace_mask();
        let op1_limb_27_col100 = eval.next_trace_mask();
        let sub_p_bit_col101 = eval.next_trace_mask();
        let padding = eval.next_trace_mask();

        eval.add_constraint(padding.clone() * padding.clone() - padding.clone());

        // Decode Instruction.

        eval.add_to_relation(RelationEntry::new(
            &self.verify_instruction_lookup_elements,
            E::EF::one(),
            &[
                input_pc_col0.clone(),
                offset0_col3.clone(),
                offset1_col4.clone(),
                offset2_col5.clone(),
                dst_base_fp_col6.clone(),
                op0_base_fp_col7.clone(),
                M31_0.clone(),
                op1_base_fp_col8.clone(),
                op1_base_ap_col9.clone(),
                M31_1.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                ap_update_add_1_col10.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_1.clone(),
            ],
        ));

        // mem_dst_base.
        eval.add_constraint(
            (mem_dst_base_col11.clone()
                - ((dst_base_fp_col6.clone() * input_fp_col2.clone())
                    + ((M31_1.clone() - dst_base_fp_col6.clone()) * input_ap_col1.clone()))),
        );
        // mem0_base.
        eval.add_constraint(
            (mem0_base_col12.clone()
                - ((op0_base_fp_col7.clone() * input_fp_col2.clone())
                    + ((M31_1.clone() - op0_base_fp_col7.clone()) * input_ap_col1.clone()))),
        );
        // Either flag op1_base_fp is on or flag op1_base_ap is on.
        eval.add_constraint(
            ((op1_base_fp_col8.clone() + op1_base_ap_col9.clone()) - M31_1.clone()),
        );
        // mem1_base.
        eval.add_constraint(
            (mem1_base_col13.clone()
                - ((op1_base_fp_col8.clone() * input_fp_col2.clone())
                    + (op1_base_ap_col9.clone() * input_ap_col1.clone()))),
        );

        // Read Positive Num Bits 252.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (mem_dst_base_col11.clone() + (offset0_col3.clone() - M31_32768.clone())),
                dst_id_col14.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                dst_id_col14.clone(),
                dst_limb_0_col15.clone(),
                dst_limb_1_col16.clone(),
                dst_limb_2_col17.clone(),
                dst_limb_3_col18.clone(),
                dst_limb_4_col19.clone(),
                dst_limb_5_col20.clone(),
                dst_limb_6_col21.clone(),
                dst_limb_7_col22.clone(),
                dst_limb_8_col23.clone(),
                dst_limb_9_col24.clone(),
                dst_limb_10_col25.clone(),
                dst_limb_11_col26.clone(),
                dst_limb_12_col27.clone(),
                dst_limb_13_col28.clone(),
                dst_limb_14_col29.clone(),
                dst_limb_15_col30.clone(),
                dst_limb_16_col31.clone(),
                dst_limb_17_col32.clone(),
                dst_limb_18_col33.clone(),
                dst_limb_19_col34.clone(),
                dst_limb_20_col35.clone(),
                dst_limb_21_col36.clone(),
                dst_limb_22_col37.clone(),
                dst_limb_23_col38.clone(),
                dst_limb_24_col39.clone(),
                dst_limb_25_col40.clone(),
                dst_limb_26_col41.clone(),
                dst_limb_27_col42.clone(),
            ],
        ));

        // Read Positive Num Bits 252.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (mem0_base_col12.clone() + (offset1_col4.clone() - M31_32768.clone())),
                op0_id_col43.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                op0_id_col43.clone(),
                op0_limb_0_col44.clone(),
                op0_limb_1_col45.clone(),
                op0_limb_2_col46.clone(),
                op0_limb_3_col47.clone(),
                op0_limb_4_col48.clone(),
                op0_limb_5_col49.clone(),
                op0_limb_6_col50.clone(),
                op0_limb_7_col51.clone(),
                op0_limb_8_col52.clone(),
                op0_limb_9_col53.clone(),
                op0_limb_10_col54.clone(),
                op0_limb_11_col55.clone(),
                op0_limb_12_col56.clone(),
                op0_limb_13_col57.clone(),
                op0_limb_14_col58.clone(),
                op0_limb_15_col59.clone(),
                op0_limb_16_col60.clone(),
                op0_limb_17_col61.clone(),
                op0_limb_18_col62.clone(),
                op0_limb_19_col63.clone(),
                op0_limb_20_col64.clone(),
                op0_limb_21_col65.clone(),
                op0_limb_22_col66.clone(),
                op0_limb_23_col67.clone(),
                op0_limb_24_col68.clone(),
                op0_limb_25_col69.clone(),
                op0_limb_26_col70.clone(),
                op0_limb_27_col71.clone(),
            ],
        ));

        // Read Positive Num Bits 252.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (mem1_base_col13.clone() + (offset2_col5.clone() - M31_32768.clone())),
                op1_id_col72.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                op1_id_col72.clone(),
                op1_limb_0_col73.clone(),
                op1_limb_1_col74.clone(),
                op1_limb_2_col75.clone(),
                op1_limb_3_col76.clone(),
                op1_limb_4_col77.clone(),
                op1_limb_5_col78.clone(),
                op1_limb_6_col79.clone(),
                op1_limb_7_col80.clone(),
                op1_limb_8_col81.clone(),
                op1_limb_9_col82.clone(),
                op1_limb_10_col83.clone(),
                op1_limb_11_col84.clone(),
                op1_limb_12_col85.clone(),
                op1_limb_13_col86.clone(),
                op1_limb_14_col87.clone(),
                op1_limb_15_col88.clone(),
                op1_limb_16_col89.clone(),
                op1_limb_17_col90.clone(),
                op1_limb_18_col91.clone(),
                op1_limb_19_col92.clone(),
                op1_limb_20_col93.clone(),
                op1_limb_21_col94.clone(),
                op1_limb_22_col95.clone(),
                op1_limb_23_col96.clone(),
                op1_limb_24_col97.clone(),
                op1_limb_25_col98.clone(),
                op1_limb_26_col99.clone(),
                op1_limb_27_col100.clone(),
            ],
        ));

        // Verify Add 252.

        // sub_p_bit is a bit.
        eval.add_constraint(
            (sub_p_bit_col101.clone() * (sub_p_bit_col101.clone() - M31_1.clone())),
        );
        let carry_tmp_3fa46_18 = eval.add_intermediate(
            (((((op0_limb_0_col44.clone() + op1_limb_0_col73.clone()) + M31_0.clone())
                - dst_limb_0_col15.clone())
                - (M31_1.clone() * sub_p_bit_col101.clone()))
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_3fa46_18.clone()
                * ((carry_tmp_3fa46_18.clone() * carry_tmp_3fa46_18.clone()) - M31_1.clone())),
        );
        let carry_tmp_3fa46_19 = eval.add_intermediate(
            (((((op0_limb_1_col45.clone() + op1_limb_1_col74.clone())
                + carry_tmp_3fa46_18.clone())
                - dst_limb_1_col16.clone())
                - (M31_0.clone() * sub_p_bit_col101.clone()))
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_3fa46_19.clone()
                * ((carry_tmp_3fa46_19.clone() * carry_tmp_3fa46_19.clone()) - M31_1.clone())),
        );
        let carry_tmp_3fa46_20 = eval.add_intermediate(
            (((((op0_limb_2_col46.clone() + op1_limb_2_col75.clone())
                + carry_tmp_3fa46_19.clone())
                - dst_limb_2_col17.clone())
                - (M31_0.clone() * sub_p_bit_col101.clone()))
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_3fa46_20.clone()
                * ((carry_tmp_3fa46_20.clone() * carry_tmp_3fa46_20.clone()) - M31_1.clone())),
        );
        let carry_tmp_3fa46_21 = eval.add_intermediate(
            (((((op0_limb_3_col47.clone() + op1_limb_3_col76.clone())
                + carry_tmp_3fa46_20.clone())
                - dst_limb_3_col18.clone())
                - (M31_0.clone() * sub_p_bit_col101.clone()))
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_3fa46_21.clone()
                * ((carry_tmp_3fa46_21.clone() * carry_tmp_3fa46_21.clone()) - M31_1.clone())),
        );
        let carry_tmp_3fa46_22 = eval.add_intermediate(
            (((((op0_limb_4_col48.clone() + op1_limb_4_col77.clone())
                + carry_tmp_3fa46_21.clone())
                - dst_limb_4_col19.clone())
                - (M31_0.clone() * sub_p_bit_col101.clone()))
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_3fa46_22.clone()
                * ((carry_tmp_3fa46_22.clone() * carry_tmp_3fa46_22.clone()) - M31_1.clone())),
        );
        let carry_tmp_3fa46_23 = eval.add_intermediate(
            (((((op0_limb_5_col49.clone() + op1_limb_5_col78.clone())
                + carry_tmp_3fa46_22.clone())
                - dst_limb_5_col20.clone())
                - (M31_0.clone() * sub_p_bit_col101.clone()))
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_3fa46_23.clone()
                * ((carry_tmp_3fa46_23.clone() * carry_tmp_3fa46_23.clone()) - M31_1.clone())),
        );
        let carry_tmp_3fa46_24 = eval.add_intermediate(
            (((((op0_limb_6_col50.clone() + op1_limb_6_col79.clone())
                + carry_tmp_3fa46_23.clone())
                - dst_limb_6_col21.clone())
                - (M31_0.clone() * sub_p_bit_col101.clone()))
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_3fa46_24.clone()
                * ((carry_tmp_3fa46_24.clone() * carry_tmp_3fa46_24.clone()) - M31_1.clone())),
        );
        let carry_tmp_3fa46_25 = eval.add_intermediate(
            (((((op0_limb_7_col51.clone() + op1_limb_7_col80.clone())
                + carry_tmp_3fa46_24.clone())
                - dst_limb_7_col22.clone())
                - (M31_0.clone() * sub_p_bit_col101.clone()))
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_3fa46_25.clone()
                * ((carry_tmp_3fa46_25.clone() * carry_tmp_3fa46_25.clone()) - M31_1.clone())),
        );
        let carry_tmp_3fa46_26 = eval.add_intermediate(
            (((((op0_limb_8_col52.clone() + op1_limb_8_col81.clone())
                + carry_tmp_3fa46_25.clone())
                - dst_limb_8_col23.clone())
                - (M31_0.clone() * sub_p_bit_col101.clone()))
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_3fa46_26.clone()
                * ((carry_tmp_3fa46_26.clone() * carry_tmp_3fa46_26.clone()) - M31_1.clone())),
        );
        let carry_tmp_3fa46_27 = eval.add_intermediate(
            (((((op0_limb_9_col53.clone() + op1_limb_9_col82.clone())
                + carry_tmp_3fa46_26.clone())
                - dst_limb_9_col24.clone())
                - (M31_0.clone() * sub_p_bit_col101.clone()))
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_3fa46_27.clone()
                * ((carry_tmp_3fa46_27.clone() * carry_tmp_3fa46_27.clone()) - M31_1.clone())),
        );
        let carry_tmp_3fa46_28 = eval.add_intermediate(
            (((((op0_limb_10_col54.clone() + op1_limb_10_col83.clone())
                + carry_tmp_3fa46_27.clone())
                - dst_limb_10_col25.clone())
                - (M31_0.clone() * sub_p_bit_col101.clone()))
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_3fa46_28.clone()
                * ((carry_tmp_3fa46_28.clone() * carry_tmp_3fa46_28.clone()) - M31_1.clone())),
        );
        let carry_tmp_3fa46_29 = eval.add_intermediate(
            (((((op0_limb_11_col55.clone() + op1_limb_11_col84.clone())
                + carry_tmp_3fa46_28.clone())
                - dst_limb_11_col26.clone())
                - (M31_0.clone() * sub_p_bit_col101.clone()))
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_3fa46_29.clone()
                * ((carry_tmp_3fa46_29.clone() * carry_tmp_3fa46_29.clone()) - M31_1.clone())),
        );
        let carry_tmp_3fa46_30 = eval.add_intermediate(
            (((((op0_limb_12_col56.clone() + op1_limb_12_col85.clone())
                + carry_tmp_3fa46_29.clone())
                - dst_limb_12_col27.clone())
                - (M31_0.clone() * sub_p_bit_col101.clone()))
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_3fa46_30.clone()
                * ((carry_tmp_3fa46_30.clone() * carry_tmp_3fa46_30.clone()) - M31_1.clone())),
        );
        let carry_tmp_3fa46_31 = eval.add_intermediate(
            (((((op0_limb_13_col57.clone() + op1_limb_13_col86.clone())
                + carry_tmp_3fa46_30.clone())
                - dst_limb_13_col28.clone())
                - (M31_0.clone() * sub_p_bit_col101.clone()))
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_3fa46_31.clone()
                * ((carry_tmp_3fa46_31.clone() * carry_tmp_3fa46_31.clone()) - M31_1.clone())),
        );
        let carry_tmp_3fa46_32 = eval.add_intermediate(
            (((((op0_limb_14_col58.clone() + op1_limb_14_col87.clone())
                + carry_tmp_3fa46_31.clone())
                - dst_limb_14_col29.clone())
                - (M31_0.clone() * sub_p_bit_col101.clone()))
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_3fa46_32.clone()
                * ((carry_tmp_3fa46_32.clone() * carry_tmp_3fa46_32.clone()) - M31_1.clone())),
        );
        let carry_tmp_3fa46_33 = eval.add_intermediate(
            (((((op0_limb_15_col59.clone() + op1_limb_15_col88.clone())
                + carry_tmp_3fa46_32.clone())
                - dst_limb_15_col30.clone())
                - (M31_0.clone() * sub_p_bit_col101.clone()))
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_3fa46_33.clone()
                * ((carry_tmp_3fa46_33.clone() * carry_tmp_3fa46_33.clone()) - M31_1.clone())),
        );
        let carry_tmp_3fa46_34 = eval.add_intermediate(
            (((((op0_limb_16_col60.clone() + op1_limb_16_col89.clone())
                + carry_tmp_3fa46_33.clone())
                - dst_limb_16_col31.clone())
                - (M31_0.clone() * sub_p_bit_col101.clone()))
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_3fa46_34.clone()
                * ((carry_tmp_3fa46_34.clone() * carry_tmp_3fa46_34.clone()) - M31_1.clone())),
        );
        let carry_tmp_3fa46_35 = eval.add_intermediate(
            (((((op0_limb_17_col61.clone() + op1_limb_17_col90.clone())
                + carry_tmp_3fa46_34.clone())
                - dst_limb_17_col32.clone())
                - (M31_0.clone() * sub_p_bit_col101.clone()))
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_3fa46_35.clone()
                * ((carry_tmp_3fa46_35.clone() * carry_tmp_3fa46_35.clone()) - M31_1.clone())),
        );
        let carry_tmp_3fa46_36 = eval.add_intermediate(
            (((((op0_limb_18_col62.clone() + op1_limb_18_col91.clone())
                + carry_tmp_3fa46_35.clone())
                - dst_limb_18_col33.clone())
                - (M31_0.clone() * sub_p_bit_col101.clone()))
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_3fa46_36.clone()
                * ((carry_tmp_3fa46_36.clone() * carry_tmp_3fa46_36.clone()) - M31_1.clone())),
        );
        let carry_tmp_3fa46_37 = eval.add_intermediate(
            (((((op0_limb_19_col63.clone() + op1_limb_19_col92.clone())
                + carry_tmp_3fa46_36.clone())
                - dst_limb_19_col34.clone())
                - (M31_0.clone() * sub_p_bit_col101.clone()))
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_3fa46_37.clone()
                * ((carry_tmp_3fa46_37.clone() * carry_tmp_3fa46_37.clone()) - M31_1.clone())),
        );
        let carry_tmp_3fa46_38 = eval.add_intermediate(
            (((((op0_limb_20_col64.clone() + op1_limb_20_col93.clone())
                + carry_tmp_3fa46_37.clone())
                - dst_limb_20_col35.clone())
                - (M31_0.clone() * sub_p_bit_col101.clone()))
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_3fa46_38.clone()
                * ((carry_tmp_3fa46_38.clone() * carry_tmp_3fa46_38.clone()) - M31_1.clone())),
        );
        let carry_tmp_3fa46_39 = eval.add_intermediate(
            (((((op0_limb_21_col65.clone() + op1_limb_21_col94.clone())
                + carry_tmp_3fa46_38.clone())
                - dst_limb_21_col36.clone())
                - (M31_136.clone() * sub_p_bit_col101.clone()))
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_3fa46_39.clone()
                * ((carry_tmp_3fa46_39.clone() * carry_tmp_3fa46_39.clone()) - M31_1.clone())),
        );
        let carry_tmp_3fa46_40 = eval.add_intermediate(
            (((((op0_limb_22_col66.clone() + op1_limb_22_col95.clone())
                + carry_tmp_3fa46_39.clone())
                - dst_limb_22_col37.clone())
                - (M31_0.clone() * sub_p_bit_col101.clone()))
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_3fa46_40.clone()
                * ((carry_tmp_3fa46_40.clone() * carry_tmp_3fa46_40.clone()) - M31_1.clone())),
        );
        let carry_tmp_3fa46_41 = eval.add_intermediate(
            (((((op0_limb_23_col67.clone() + op1_limb_23_col96.clone())
                + carry_tmp_3fa46_40.clone())
                - dst_limb_23_col38.clone())
                - (M31_0.clone() * sub_p_bit_col101.clone()))
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_3fa46_41.clone()
                * ((carry_tmp_3fa46_41.clone() * carry_tmp_3fa46_41.clone()) - M31_1.clone())),
        );
        let carry_tmp_3fa46_42 = eval.add_intermediate(
            (((((op0_limb_24_col68.clone() + op1_limb_24_col97.clone())
                + carry_tmp_3fa46_41.clone())
                - dst_limb_24_col39.clone())
                - (M31_0.clone() * sub_p_bit_col101.clone()))
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_3fa46_42.clone()
                * ((carry_tmp_3fa46_42.clone() * carry_tmp_3fa46_42.clone()) - M31_1.clone())),
        );
        let carry_tmp_3fa46_43 = eval.add_intermediate(
            (((((op0_limb_25_col69.clone() + op1_limb_25_col98.clone())
                + carry_tmp_3fa46_42.clone())
                - dst_limb_25_col40.clone())
                - (M31_0.clone() * sub_p_bit_col101.clone()))
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_3fa46_43.clone()
                * ((carry_tmp_3fa46_43.clone() * carry_tmp_3fa46_43.clone()) - M31_1.clone())),
        );
        let carry_tmp_3fa46_44 = eval.add_intermediate(
            (((((op0_limb_26_col70.clone() + op1_limb_26_col99.clone())
                + carry_tmp_3fa46_43.clone())
                - dst_limb_26_col41.clone())
                - (M31_0.clone() * sub_p_bit_col101.clone()))
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_3fa46_44.clone()
                * ((carry_tmp_3fa46_44.clone() * carry_tmp_3fa46_44.clone()) - M31_1.clone())),
        );
        eval.add_constraint(
            ((((op0_limb_27_col71.clone() + op1_limb_27_col100.clone())
                + carry_tmp_3fa46_44.clone())
                - dst_limb_27_col42.clone())
                - (M31_256.clone() * sub_p_bit_col101.clone())),
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
                (input_pc_col0.clone() + M31_1.clone()),
                (input_ap_col1.clone() + ap_update_add_1_col10.clone()),
                input_fp_col2.clone(),
            ],
        ));

        eval.finalize_logup_in_pairs();
        eval
    }
}
