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
        let trace_log_sizes = vec![self.log_size; 37];
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
        let M31_262144 = E::F::from(M31::from(262144));
        let M31_32768 = E::F::from(M31::from(32768));
        let M31_512 = E::F::from(M31::from(512));
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
        let op0_id_col23 = eval.next_trace_mask();
        let op0_limb_0_col24 = eval.next_trace_mask();
        let op0_limb_1_col25 = eval.next_trace_mask();
        let op0_limb_2_col26 = eval.next_trace_mask();
        let op0_limb_3_col27 = eval.next_trace_mask();
        let op1_id_col28 = eval.next_trace_mask();
        let op1_limb_0_col29 = eval.next_trace_mask();
        let op1_limb_1_col30 = eval.next_trace_mask();
        let op1_limb_2_col31 = eval.next_trace_mask();
        let op1_limb_3_col32 = eval.next_trace_mask();
        let carry_1_col33 = eval.next_trace_mask();
        let carry_3_col34 = eval.next_trace_mask();
        let carry_5_col35 = eval.next_trace_mask();
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
                M31_0.clone(),
                M31_1.clone(),
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

        // Read Positive Num Bits 72.

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
            ],
        ));

        // Read Positive Num Bits 36.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (mem0_base_col12.clone() + (offset1_col4.clone() - M31_32768.clone())),
                op0_id_col23.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                op0_id_col23.clone(),
                op0_limb_0_col24.clone(),
                op0_limb_1_col25.clone(),
                op0_limb_2_col26.clone(),
                op0_limb_3_col27.clone(),
            ],
        ));

        // Read Positive Num Bits 36.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (mem1_base_col13.clone() + (offset2_col5.clone() - M31_32768.clone())),
                op1_id_col28.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                op1_id_col28.clone(),
                op1_limb_0_col29.clone(),
                op1_limb_1_col30.clone(),
                op1_limb_2_col31.clone(),
                op1_limb_3_col32.clone(),
            ],
        ));

        // Verify Mul Small.

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_11_lookup_elements,
            E::EF::one(),
            &[carry_1_col33.clone()],
        ));

        // carry 1 definition.
        eval.add_constraint(
            ((carry_1_col33.clone() * M31_262144.clone())
                - (((((M31_0.clone()
                    + ((op0_limb_0_col24.clone() * op1_limb_0_col29.clone()) * M31_1.clone()))
                    - (dst_limb_0_col15.clone() * M31_1.clone()))
                    + ((op0_limb_0_col24.clone() * op1_limb_1_col30.clone()) * M31_512.clone()))
                    + ((op0_limb_1_col25.clone() * op1_limb_0_col29.clone()) * M31_512.clone()))
                    - (dst_limb_1_col16.clone() * M31_512.clone()))),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_11_lookup_elements,
            E::EF::one(),
            &[carry_3_col34.clone()],
        ));

        // carry 3 definition.
        eval.add_constraint(
            ((carry_3_col34.clone() * M31_262144.clone())
                - (((((((((carry_1_col33.clone()
                    + ((op0_limb_0_col24.clone() * op1_limb_2_col31.clone())
                        * M31_1.clone()))
                    + ((op0_limb_1_col25.clone() * op1_limb_1_col30.clone())
                        * M31_1.clone()))
                    + ((op0_limb_2_col26.clone() * op1_limb_0_col29.clone())
                        * M31_1.clone()))
                    - (dst_limb_2_col17.clone() * M31_1.clone()))
                    + ((op0_limb_0_col24.clone() * op1_limb_3_col32.clone())
                        * M31_512.clone()))
                    + ((op0_limb_1_col25.clone() * op1_limb_2_col31.clone())
                        * M31_512.clone()))
                    + ((op0_limb_2_col26.clone() * op1_limb_1_col30.clone()) * M31_512.clone()))
                    + ((op0_limb_3_col27.clone() * op1_limb_0_col29.clone()) * M31_512.clone()))
                    - (dst_limb_3_col18.clone() * M31_512.clone()))),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_11_lookup_elements,
            E::EF::one(),
            &[carry_5_col35.clone()],
        ));

        // carry 5 definition.
        eval.add_constraint(
            ((carry_5_col35.clone() * M31_262144.clone())
                - (((((((carry_3_col34.clone()
                    + ((op0_limb_1_col25.clone() * op1_limb_3_col32.clone())
                        * M31_1.clone()))
                    + ((op0_limb_2_col26.clone() * op1_limb_2_col31.clone())
                        * M31_1.clone()))
                    + ((op0_limb_3_col27.clone() * op1_limb_1_col30.clone()) * M31_1.clone()))
                    - (dst_limb_4_col19.clone() * M31_1.clone()))
                    + ((op0_limb_2_col26.clone() * op1_limb_3_col32.clone()) * M31_512.clone()))
                    + ((op0_limb_3_col27.clone() * op1_limb_2_col31.clone()) * M31_512.clone()))
                    - (dst_limb_5_col20.clone() * M31_512.clone()))),
        );
        // final limb constraint.
        eval.add_constraint(
            (((carry_5_col35.clone() + (op0_limb_3_col27.clone() * op1_limb_3_col32.clone()))
                - (dst_limb_7_col22.clone() * M31_512.clone()))
                - dst_limb_6_col21.clone()),
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
