use crate::components::prelude::constraint_eval::*;

pub struct Eval {
    pub claim: Claim,
    pub memory_address_to_id_lookup_elements: relations::MemoryAddressToId,
    pub memory_id_to_big_lookup_elements: relations::MemoryIdToBig,
    pub opcodes_lookup_elements: relations::Opcodes,
    pub range_check_11_lookup_elements: relations::RangeCheck_11,
    pub verify_instruction_lookup_elements: relations::VerifyInstruction,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Claim {
    pub log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; 33];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 6];
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
        let M31_16 = E::F::from(M31::from(16));
        let M31_2 = E::F::from(M31::from(2));
        let M31_256 = E::F::from(M31::from(256));
        let M31_262144 = E::F::from(M31::from(262144));
        let M31_32 = E::F::from(M31::from(32));
        let M31_32768 = E::F::from(M31::from(32768));
        let M31_32769 = E::F::from(M31::from(32769));
        let M31_512 = E::F::from(M31::from(512));
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
        let op0_id_col19 = eval.next_trace_mask();
        let op0_limb_0_col20 = eval.next_trace_mask();
        let op0_limb_1_col21 = eval.next_trace_mask();
        let op0_limb_2_col22 = eval.next_trace_mask();
        let op0_limb_3_col23 = eval.next_trace_mask();
        let op1_id_col24 = eval.next_trace_mask();
        let op1_limb_0_col25 = eval.next_trace_mask();
        let op1_limb_1_col26 = eval.next_trace_mask();
        let op1_limb_2_col27 = eval.next_trace_mask();
        let op1_limb_3_col28 = eval.next_trace_mask();
        let carry_1_col29 = eval.next_trace_mask();
        let carry_3_col30 = eval.next_trace_mask();
        let carry_5_col31 = eval.next_trace_mask();
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
                ((((((M31_0.clone() + (dst_base_fp_col5.clone() * M31_8.clone()))
                    + (op0_base_fp_col6.clone() * M31_16.clone()))
                    + M31_32.clone())
                    + M31_0.clone())
                    + M31_0.clone())
                    + M31_0.clone()),
                ((((M31_1.clone() + (ap_update_add_1_col7.clone() * M31_32.clone()))
                    + M31_0.clone())
                    + M31_0.clone())
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

        // Read Positive Num Bits 72.

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
            ],
        ));

        // Read Positive Num Bits 36.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (mem0_base_col9.clone() + (offset1_col4.clone() - M31_32768.clone())),
                op0_id_col19.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                op0_id_col19.clone(),
                op0_limb_0_col20.clone(),
                op0_limb_1_col21.clone(),
                op0_limb_2_col22.clone(),
                op0_limb_3_col23.clone(),
            ],
        ));

        // Read Positive Num Bits 36.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (input_pc_col0.clone() + M31_1.clone()),
                op1_id_col24.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                op1_id_col24.clone(),
                op1_limb_0_col25.clone(),
                op1_limb_1_col26.clone(),
                op1_limb_2_col27.clone(),
                op1_limb_3_col28.clone(),
            ],
        ));

        // Verify Mul Small.

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_11_lookup_elements,
            E::EF::one(),
            &[carry_1_col29.clone()],
        ));

        // carry 1 definition.
        eval.add_constraint(
            ((carry_1_col29.clone() * M31_262144.clone())
                - (((((M31_0.clone()
                    + ((op0_limb_0_col20.clone() * op1_limb_0_col25.clone()) * M31_1.clone()))
                    - (dst_limb_0_col11.clone() * M31_1.clone()))
                    + ((op0_limb_0_col20.clone() * op1_limb_1_col26.clone()) * M31_512.clone()))
                    + ((op0_limb_1_col21.clone() * op1_limb_0_col25.clone()) * M31_512.clone()))
                    - (dst_limb_1_col12.clone() * M31_512.clone()))),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_11_lookup_elements,
            E::EF::one(),
            &[carry_3_col30.clone()],
        ));

        // carry 3 definition.
        eval.add_constraint(
            ((carry_3_col30.clone() * M31_262144.clone())
                - (((((((((carry_1_col29.clone()
                    + ((op0_limb_0_col20.clone() * op1_limb_2_col27.clone())
                        * M31_1.clone()))
                    + ((op0_limb_1_col21.clone() * op1_limb_1_col26.clone())
                        * M31_1.clone()))
                    + ((op0_limb_2_col22.clone() * op1_limb_0_col25.clone())
                        * M31_1.clone()))
                    - (dst_limb_2_col13.clone() * M31_1.clone()))
                    + ((op0_limb_0_col20.clone() * op1_limb_3_col28.clone())
                        * M31_512.clone()))
                    + ((op0_limb_1_col21.clone() * op1_limb_2_col27.clone())
                        * M31_512.clone()))
                    + ((op0_limb_2_col22.clone() * op1_limb_1_col26.clone()) * M31_512.clone()))
                    + ((op0_limb_3_col23.clone() * op1_limb_0_col25.clone()) * M31_512.clone()))
                    - (dst_limb_3_col14.clone() * M31_512.clone()))),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_11_lookup_elements,
            E::EF::one(),
            &[carry_5_col31.clone()],
        ));

        // carry 5 definition.
        eval.add_constraint(
            ((carry_5_col31.clone() * M31_262144.clone())
                - (((((((carry_3_col30.clone()
                    + ((op0_limb_1_col21.clone() * op1_limb_3_col28.clone())
                        * M31_1.clone()))
                    + ((op0_limb_2_col22.clone() * op1_limb_2_col27.clone())
                        * M31_1.clone()))
                    + ((op0_limb_3_col23.clone() * op1_limb_1_col26.clone()) * M31_1.clone()))
                    - (dst_limb_4_col15.clone() * M31_1.clone()))
                    + ((op0_limb_2_col22.clone() * op1_limb_3_col28.clone()) * M31_512.clone()))
                    + ((op0_limb_3_col23.clone() * op1_limb_2_col27.clone()) * M31_512.clone()))
                    - (dst_limb_5_col16.clone() * M31_512.clone()))),
        );
        // final limb constraint.
        eval.add_constraint(
            (((carry_5_col31.clone() + (op0_limb_3_col23.clone() * op1_limb_3_col28.clone()))
                - (dst_limb_7_col18.clone() * M31_512.clone()))
                - dst_limb_6_col17.clone()),
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
