use crate::components::prelude::*;

pub const N_TRACE_COLUMNS: usize = 18;

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
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
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
        let M31_134217728 = E::F::from(M31::from(134217728));
        let M31_136 = E::F::from(M31::from(136));
        let M31_2 = E::F::from(M31::from(2));
        let M31_256 = E::F::from(M31::from(256));
        let M31_262144 = E::F::from(M31::from(262144));
        let M31_32 = E::F::from(M31::from(32));
        let M31_32768 = E::F::from(M31::from(32768));
        let M31_32769 = E::F::from(M31::from(32769));
        let M31_511 = E::F::from(M31::from(511));
        let M31_512 = E::F::from(M31::from(512));
        let M31_68 = E::F::from(M31::from(68));
        let input_pc_col0 = eval.next_trace_mask();
        let input_ap_col1 = eval.next_trace_mask();
        let input_fp_col2 = eval.next_trace_mask();
        let stored_fp_id_col3 = eval.next_trace_mask();
        let stored_fp_limb_0_col4 = eval.next_trace_mask();
        let stored_fp_limb_1_col5 = eval.next_trace_mask();
        let stored_fp_limb_2_col6 = eval.next_trace_mask();
        let stored_ret_pc_id_col7 = eval.next_trace_mask();
        let stored_ret_pc_limb_0_col8 = eval.next_trace_mask();
        let stored_ret_pc_limb_1_col9 = eval.next_trace_mask();
        let stored_ret_pc_limb_2_col10 = eval.next_trace_mask();
        let distance_to_next_pc_id_col11 = eval.next_trace_mask();
        let msb_col12 = eval.next_trace_mask();
        let mid_limbs_set_col13 = eval.next_trace_mask();
        let distance_to_next_pc_limb_0_col14 = eval.next_trace_mask();
        let distance_to_next_pc_limb_1_col15 = eval.next_trace_mask();
        let distance_to_next_pc_limb_2_col16 = eval.next_trace_mask();
        let padding = eval.next_trace_mask();

        eval.add_constraint(padding.clone() * padding.clone() - padding.clone());

        // Decode Instruction.

        eval.add_to_relation(RelationEntry::new(
            &self.verify_instruction_lookup_elements,
            E::EF::one(),
            &[
                input_pc_col0.clone(),
                M31_32768.clone(),
                M31_32769.clone(),
                M31_32769.clone(),
                M31_32.clone(),
                M31_68.clone(),
            ],
        ));

        // Read Positive Num Bits 27.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[input_ap_col1.clone(), stored_fp_id_col3.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                stored_fp_id_col3.clone(),
                stored_fp_limb_0_col4.clone(),
                stored_fp_limb_1_col5.clone(),
                stored_fp_limb_2_col6.clone(),
            ],
        ));

        //[ap] = fp.
        eval.add_constraint(
            (((stored_fp_limb_0_col4.clone() + (stored_fp_limb_1_col5.clone() * M31_512.clone()))
                + (stored_fp_limb_2_col6.clone() * M31_262144.clone()))
                - input_fp_col2.clone()),
        );

        // Read Positive Num Bits 27.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (input_ap_col1.clone() + M31_1.clone()),
                stored_ret_pc_id_col7.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                stored_ret_pc_id_col7.clone(),
                stored_ret_pc_limb_0_col8.clone(),
                stored_ret_pc_limb_1_col9.clone(),
                stored_ret_pc_limb_2_col10.clone(),
            ],
        ));

        //[ap+1] = return_pc.
        eval.add_constraint(
            (((stored_ret_pc_limb_0_col8.clone()
                + (stored_ret_pc_limb_1_col9.clone() * M31_512.clone()))
                + (stored_ret_pc_limb_2_col10.clone() * M31_262144.clone()))
                - (input_pc_col0.clone() + M31_2.clone())),
        );

        // Read Small.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (input_pc_col0.clone() + M31_1.clone()),
                distance_to_next_pc_id_col11.clone(),
            ],
        ));

        // Cond Decode Small Sign.

        // msb is a bit.
        eval.add_constraint((msb_col12.clone() * (msb_col12.clone() - M31_1.clone())));
        // mid_limbs_set is a bit.
        eval.add_constraint(
            (mid_limbs_set_col13.clone() * (mid_limbs_set_col13.clone() - M31_1.clone())),
        );
        // Cannot have msb equals 0 and mid_limbs_set equals 1.
        eval.add_constraint((mid_limbs_set_col13.clone() * (msb_col12.clone() - M31_1.clone())));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                distance_to_next_pc_id_col11.clone(),
                distance_to_next_pc_limb_0_col14.clone(),
                distance_to_next_pc_limb_1_col15.clone(),
                distance_to_next_pc_limb_2_col16.clone(),
                (mid_limbs_set_col13.clone() * M31_511.clone()),
                (mid_limbs_set_col13.clone() * M31_511.clone()),
                (mid_limbs_set_col13.clone() * M31_511.clone()),
                (mid_limbs_set_col13.clone() * M31_511.clone()),
                (mid_limbs_set_col13.clone() * M31_511.clone()),
                (mid_limbs_set_col13.clone() * M31_511.clone()),
                (mid_limbs_set_col13.clone() * M31_511.clone()),
                (mid_limbs_set_col13.clone() * M31_511.clone()),
                (mid_limbs_set_col13.clone() * M31_511.clone()),
                (mid_limbs_set_col13.clone() * M31_511.clone()),
                (mid_limbs_set_col13.clone() * M31_511.clone()),
                (mid_limbs_set_col13.clone() * M31_511.clone()),
                (mid_limbs_set_col13.clone() * M31_511.clone()),
                (mid_limbs_set_col13.clone() * M31_511.clone()),
                (mid_limbs_set_col13.clone() * M31_511.clone()),
                (mid_limbs_set_col13.clone() * M31_511.clone()),
                (mid_limbs_set_col13.clone() * M31_511.clone()),
                (mid_limbs_set_col13.clone() * M31_511.clone()),
                ((M31_136.clone() * msb_col12.clone()) - mid_limbs_set_col13.clone()),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                (msb_col12.clone() * M31_256.clone()),
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
                (input_pc_col0.clone()
                    + ((((distance_to_next_pc_limb_0_col14.clone()
                        + (distance_to_next_pc_limb_1_col15.clone() * M31_512.clone()))
                        + (distance_to_next_pc_limb_2_col16.clone() * M31_262144.clone()))
                        - msb_col12.clone())
                        - (M31_134217728.clone() * mid_limbs_set_col13.clone()))),
                (input_ap_col1.clone() + M31_2.clone()),
                (input_ap_col1.clone() + M31_2.clone()),
            ],
        ));

        eval.finalize_logup_in_pairs();
        eval
    }
}
