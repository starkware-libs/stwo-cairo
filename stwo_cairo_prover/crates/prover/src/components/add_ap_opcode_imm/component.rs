#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};
use stwo_cairo_serialize::CairoSerialize;
use stwo_prover::constraint_framework::logup::{LogupAtRow, LookupElements};
use stwo_prover::constraint_framework::{
    EvalAtRow, FrameworkComponent, FrameworkEval, RelationEntry,
};
use stwo_prover::core::backend::simd::m31::LOG_N_LANES;
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::fields::secure_column::SECURE_EXTENSION_DEGREE;
use stwo_prover::core::pcs::TreeVec;

use crate::cairo_air::preprocessed::PreProcessedColumn;
use crate::relations;

pub struct Eval {
    pub claim: Claim,
    pub memory_address_to_id_lookup_elements: relations::MemoryAddressToId,
    pub memory_id_to_big_lookup_elements: relations::MemoryIdToBig,
    pub opcodes_lookup_elements: relations::Opcodes,
    pub verify_instruction_lookup_elements: relations::VerifyInstruction,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Claim {
    pub n_rows: usize,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let log_size = std::cmp::max(self.n_rows.next_power_of_two().ilog2(), LOG_N_LANES);
        let trace_log_sizes = vec![log_size; 10];
        let interaction_log_sizes = vec![log_size; SECURE_EXTENSION_DEGREE * 3];
        let preprocessed_log_sizes = vec![log_size];
        TreeVec::new(vec![
            preprocessed_log_sizes,
            trace_log_sizes,
            interaction_log_sizes,
        ])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(self.n_rows as u64);
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
        std::cmp::max(self.claim.n_rows.next_power_of_two().ilog2(), LOG_N_LANES)
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
        let M31_32767 = E::F::from(M31::from(32767));
        let M31_32769 = E::F::from(M31::from(32769));
        let M31_511 = E::F::from(M31::from(511));
        let M31_512 = E::F::from(M31::from(512));
        let input_pc_col0 = eval.next_trace_mask();
        let input_ap_col1 = eval.next_trace_mask();
        let input_fp_col2 = eval.next_trace_mask();
        let op1_id_col3 = eval.next_trace_mask();
        let msb_col4 = eval.next_trace_mask();
        let mid_limbs_set_col5 = eval.next_trace_mask();
        let op1_limb_0_col6 = eval.next_trace_mask();
        let op1_limb_1_col7 = eval.next_trace_mask();
        let op1_limb_2_col8 = eval.next_trace_mask();
        let padding = eval.next_trace_mask();

        eval.add_constraint(padding.clone() * padding.clone() - padding.clone());

        // Decode Instruction.

        eval.add_to_relation(RelationEntry::new(
            &self.verify_instruction_lookup_elements,
            E::EF::one(),
            &[
                input_pc_col0.clone(),
                M31_32767.clone(),
                M31_32767.clone(),
                M31_32769.clone(),
                M31_1.clone(),
                M31_1.clone(),
                M31_1.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_1.clone(),
            ],
        ));

        // Read Small.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[(input_pc_col0.clone() + M31_1.clone()), op1_id_col3.clone()],
        ));

        // Cond Decode Small Sign.

        // msb is a bit.
        eval.add_constraint((msb_col4.clone() * (msb_col4.clone() - M31_1.clone())));
        // mid_limbs_set is a bit.
        eval.add_constraint(
            (mid_limbs_set_col5.clone() * (mid_limbs_set_col5.clone() - M31_1.clone())),
        );
        // Cannot have msb equals 0 and mid_limbs_set equals 1.
        eval.add_constraint(
            ((M31_1.clone() * mid_limbs_set_col5.clone()) * (msb_col4.clone() - M31_1.clone())),
        );

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                op1_id_col3.clone(),
                op1_limb_0_col6.clone(),
                op1_limb_1_col7.clone(),
                op1_limb_2_col8.clone(),
                (mid_limbs_set_col5.clone() * M31_511.clone()),
                (mid_limbs_set_col5.clone() * M31_511.clone()),
                (mid_limbs_set_col5.clone() * M31_511.clone()),
                (mid_limbs_set_col5.clone() * M31_511.clone()),
                (mid_limbs_set_col5.clone() * M31_511.clone()),
                (mid_limbs_set_col5.clone() * M31_511.clone()),
                (mid_limbs_set_col5.clone() * M31_511.clone()),
                (mid_limbs_set_col5.clone() * M31_511.clone()),
                (mid_limbs_set_col5.clone() * M31_511.clone()),
                (mid_limbs_set_col5.clone() * M31_511.clone()),
                (mid_limbs_set_col5.clone() * M31_511.clone()),
                (mid_limbs_set_col5.clone() * M31_511.clone()),
                (mid_limbs_set_col5.clone() * M31_511.clone()),
                (mid_limbs_set_col5.clone() * M31_511.clone()),
                (mid_limbs_set_col5.clone() * M31_511.clone()),
                (mid_limbs_set_col5.clone() * M31_511.clone()),
                (mid_limbs_set_col5.clone() * M31_511.clone()),
                (mid_limbs_set_col5.clone() * M31_511.clone()),
                ((M31_136.clone() * msb_col4.clone()) - mid_limbs_set_col5.clone()),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                (msb_col4.clone() * M31_256.clone()),
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
                (input_pc_col0.clone() + M31_2.clone()),
                (input_ap_col1.clone()
                    + ((((op1_limb_0_col6.clone()
                        + (op1_limb_1_col7.clone() * M31_512.clone()))
                        + (op1_limb_2_col8.clone() * M31_262144.clone()))
                        - msb_col4.clone())
                        - (M31_134217728.clone() * mid_limbs_set_col5.clone()))),
                input_fp_col2.clone(),
            ],
        ));

        eval.finalize_logup_in_pairs();
        eval
    }
}
