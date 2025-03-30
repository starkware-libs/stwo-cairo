use crate::components::prelude::constraint_eval::*;

pub const N_TRACE_COLUMNS: usize = 15;

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
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 3];
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
        let M31_128 = E::F::from(M31::from(128));
        let M31_134217728 = E::F::from(M31::from(134217728));
        let M31_136 = E::F::from(M31::from(136));
        let M31_24 = E::F::from(M31::from(24));
        let M31_256 = E::F::from(M31::from(256));
        let M31_262144 = E::F::from(M31::from(262144));
        let M31_32 = E::F::from(M31::from(32));
        let M31_32767 = E::F::from(M31::from(32767));
        let M31_32768 = E::F::from(M31::from(32768));
        let M31_4 = E::F::from(M31::from(4));
        let M31_511 = E::F::from(M31::from(511));
        let M31_512 = E::F::from(M31::from(512));
        let M31_64 = E::F::from(M31::from(64));
        let input_pc_col0 = eval.next_trace_mask();
        let input_ap_col1 = eval.next_trace_mask();
        let input_fp_col2 = eval.next_trace_mask();
        let offset2_col3 = eval.next_trace_mask();
        let op1_base_fp_col4 = eval.next_trace_mask();
        let op1_base_ap_col5 = eval.next_trace_mask();
        let ap_update_add_1_col6 = eval.next_trace_mask();
        let mem1_base_col7 = eval.next_trace_mask();
        let next_pc_id_col8 = eval.next_trace_mask();
        let msb_col9 = eval.next_trace_mask();
        let mid_limbs_set_col10 = eval.next_trace_mask();
        let next_pc_limb_0_col11 = eval.next_trace_mask();
        let next_pc_limb_1_col12 = eval.next_trace_mask();
        let next_pc_limb_2_col13 = eval.next_trace_mask();
        let padding = eval.next_trace_mask();

        eval.add_constraint(padding.clone() * padding.clone() - padding.clone());

        // Decode Instruction.

        // Flag op1_base_fp is a bit.
        eval.add_constraint(
            (op1_base_fp_col4.clone() * (M31_1.clone() - op1_base_fp_col4.clone())),
        );
        // Flag op1_base_ap is a bit.
        eval.add_constraint(
            (op1_base_ap_col5.clone() * (M31_1.clone() - op1_base_ap_col5.clone())),
        );
        // Flag ap_update_add_1 is a bit.
        eval.add_constraint(
            (ap_update_add_1_col6.clone() * (M31_1.clone() - ap_update_add_1_col6.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.verify_instruction_lookup_elements,
            E::EF::one(),
            &[
                input_pc_col0.clone(),
                M31_32767.clone(),
                M31_32767.clone(),
                offset2_col3.clone(),
                ((M31_24.clone() + (op1_base_fp_col4.clone() * M31_64.clone()))
                    + (op1_base_ap_col5.clone() * M31_128.clone())),
                (M31_4.clone() + (ap_update_add_1_col6.clone() * M31_32.clone())),
            ],
        ));

        // Either flag op1_base_fp is on or flag op1_base_ap is on.
        eval.add_constraint(
            ((op1_base_fp_col4.clone() + op1_base_ap_col5.clone()) - M31_1.clone()),
        );
        // mem1_base.
        eval.add_constraint(
            (mem1_base_col7.clone()
                - ((op1_base_fp_col4.clone() * input_fp_col2.clone())
                    + (op1_base_ap_col5.clone() * input_ap_col1.clone()))),
        );

        // Read Small.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (mem1_base_col7.clone() + (offset2_col3.clone() - M31_32768.clone())),
                next_pc_id_col8.clone(),
            ],
        ));

        // Cond Decode Small Sign.

        // msb is a bit.
        eval.add_constraint((msb_col9.clone() * (msb_col9.clone() - M31_1.clone())));
        // mid_limbs_set is a bit.
        eval.add_constraint(
            (mid_limbs_set_col10.clone() * (mid_limbs_set_col10.clone() - M31_1.clone())),
        );
        // Cannot have msb equals 0 and mid_limbs_set equals 1.
        eval.add_constraint((mid_limbs_set_col10.clone() * (msb_col9.clone() - M31_1.clone())));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                next_pc_id_col8.clone(),
                next_pc_limb_0_col11.clone(),
                next_pc_limb_1_col12.clone(),
                next_pc_limb_2_col13.clone(),
                (mid_limbs_set_col10.clone() * M31_511.clone()),
                (mid_limbs_set_col10.clone() * M31_511.clone()),
                (mid_limbs_set_col10.clone() * M31_511.clone()),
                (mid_limbs_set_col10.clone() * M31_511.clone()),
                (mid_limbs_set_col10.clone() * M31_511.clone()),
                (mid_limbs_set_col10.clone() * M31_511.clone()),
                (mid_limbs_set_col10.clone() * M31_511.clone()),
                (mid_limbs_set_col10.clone() * M31_511.clone()),
                (mid_limbs_set_col10.clone() * M31_511.clone()),
                (mid_limbs_set_col10.clone() * M31_511.clone()),
                (mid_limbs_set_col10.clone() * M31_511.clone()),
                (mid_limbs_set_col10.clone() * M31_511.clone()),
                (mid_limbs_set_col10.clone() * M31_511.clone()),
                (mid_limbs_set_col10.clone() * M31_511.clone()),
                (mid_limbs_set_col10.clone() * M31_511.clone()),
                (mid_limbs_set_col10.clone() * M31_511.clone()),
                (mid_limbs_set_col10.clone() * M31_511.clone()),
                (mid_limbs_set_col10.clone() * M31_511.clone()),
                ((M31_136.clone() * msb_col9.clone()) - mid_limbs_set_col10.clone()),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                (msb_col9.clone() * M31_256.clone()),
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
                    + ((((next_pc_limb_0_col11.clone()
                        + (next_pc_limb_1_col12.clone() * M31_512.clone()))
                        + (next_pc_limb_2_col13.clone() * M31_262144.clone()))
                        - msb_col9.clone())
                        - (M31_134217728.clone() * mid_limbs_set_col10.clone()))),
                (input_ap_col1.clone() + ap_update_add_1_col6.clone()),
                input_fp_col2.clone(),
            ],
        ));

        eval.finalize_logup_in_pairs();
        eval
    }
}
