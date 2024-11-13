#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use std::ops::{Mul, Sub};

use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};
use stwo_prover::constraint_framework::logup::{ClaimedPrefixSum, LogupAtRow, LookupElements};
use stwo_prover::constraint_framework::preprocessed_columns::PreprocessedColumn;
use stwo_prover::constraint_framework::{
    EvalAtRow, FrameworkComponent, FrameworkEval, INTERACTION_TRACE_IDX,
};
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES};
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::fields::secure_column::SECURE_EXTENSION_DEGREE;
use stwo_prover::core::lookups::utils::Fraction;
use stwo_prover::core::pcs::TreeVec;

use crate::components::memory::{addr_to_id, id_to_f252};
use crate::components::{opcodes, verifyinstruction};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RelationElements(LookupElements<4>);
impl RelationElements {
    pub fn draw(channel: &mut impl Channel) -> Self {
        Self(LookupElements::<4>::draw(channel))
    }
    pub fn combine<F: Clone, EF>(&self, values: &[F]) -> EF
    where
        EF: Clone + Zero + From<F> + From<SecureField> + Mul<F, Output = EF> + Sub<EF, Output = EF>,
    {
        self.0.combine(values)
    }
}

pub struct Eval {
    pub claim: Claim,
    pub interaction_claim: InteractionClaim,
    pub addr_to_id_lookup_elements: addr_to_id::RelationElements,
    pub id_to_f252_lookup_elements: id_to_f252::RelationElements,
    pub verifyinstruction_lookup_elements: verifyinstruction::RelationElements,
    pub opcodes_lookup_elements: opcodes::RelationElements,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Claim {
    pub n_calls: usize,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let log_size = std::cmp::max(self.n_calls.next_power_of_two().ilog2(), LOG_N_LANES);
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
        channel.mix_u64(self.n_calls as u64);
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct InteractionClaim {
    pub total_sum: SecureField,
    pub claimed_sum: Option<ClaimedPrefixSum>,
}
impl InteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_felts(&[self.total_sum]);
        if let Some(claimed_sum) = self.claimed_sum {
            channel.mix_felts(&[claimed_sum.0]);
            channel.mix_u64(claimed_sum.1 as u64);
        }
    }
}

pub type Component = FrameworkComponent<Eval>;

impl FrameworkEval for Eval {
    fn log_size(&self) -> u32 {
        std::cmp::max(self.claim.n_calls.next_power_of_two().ilog2(), LOG_N_LANES)
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
        // let M31_134217728 = E::F::from(M31::from(134217728));
        let M31_136 = E::F::from(M31::from(136));
        let M31_256 = E::F::from(M31::from(256));
        // let M31_262144 = E::F::from(M31::from(262144));
        let M31_32767 = E::F::from(M31::from(32767));
        let M31_32769 = E::F::from(M31::from(32769));
        let M31_511 = E::F::from(M31::from(511));
        // let M31_512 = E::F::from(M31::from(512));
        let is_first = eval.get_preprocessed_column(PreprocessedColumn::IsFirst(self.log_size()));
        let mut logup = LogupAtRow::<E>::new(
            INTERACTION_TRACE_IDX,
            self.interaction_claim.total_sum,
            self.interaction_claim.claimed_sum,
            is_first,
        );
        let input_pc_col0 = eval.next_trace_mask();
        let _input_ap_col1 = eval.next_trace_mask();
        let _input_fp_col2 = eval.next_trace_mask();
        let ap_update_add_1_col3 = eval.next_trace_mask();
        let next_pc_id_col4 = eval.next_trace_mask();
        let msb_col5 = eval.next_trace_mask();
        let mid_limbs_set_col6 = eval.next_trace_mask();
        let next_pc_limb_0_col7 = eval.next_trace_mask();
        let next_pc_limb_1_col8 = eval.next_trace_mask();
        let next_pc_limb_2_col9 = eval.next_trace_mask();

        // DecodeInstruction_ccd4c4cd993af638.

        let frac = Fraction::new(
            E::EF::one(),
            self.verifyinstruction_lookup_elements.combine(&[
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
                M31_1.clone(),
                M31_0.clone(),
                M31_0.clone(),
                ap_update_add_1_col3.clone(),
            ]),
        );
        logup.write_frac(&mut eval, frac);

        // ReadSmall.

        let frac = Fraction::new(
            E::EF::one(),
            self.addr_to_id_lookup_elements.combine(&[
                (input_pc_col0.clone() + M31_1.clone()),
                next_pc_id_col4.clone(),
            ]),
        );
        logup.write_frac(&mut eval, frac);

        // CondDecodeSmallSign.

        // msb is a bit.
        eval.add_constraint((msb_col5.clone() * (msb_col5.clone() - M31_1.clone())));
        // mid_limbs_set is a bit.
        eval.add_constraint(
            (mid_limbs_set_col6.clone() * (mid_limbs_set_col6.clone() - M31_1.clone())),
        );
        // Cannot have msb equals 0 and mid_limbs_set equals 1.
        eval.add_constraint(
            ((M31_1.clone() * mid_limbs_set_col6.clone()) * (msb_col5.clone() - M31_1.clone())),
        );

        let frac = Fraction::new(
            E::EF::one(),
            self.id_to_f252_lookup_elements.combine(&[
                next_pc_id_col4.clone(),
                next_pc_limb_0_col7.clone(),
                next_pc_limb_1_col8.clone(),
                next_pc_limb_2_col9.clone(),
                (mid_limbs_set_col6.clone() * M31_511.clone()),
                (mid_limbs_set_col6.clone() * M31_511.clone()),
                (mid_limbs_set_col6.clone() * M31_511.clone()),
                (mid_limbs_set_col6.clone() * M31_511.clone()),
                (mid_limbs_set_col6.clone() * M31_511.clone()),
                (mid_limbs_set_col6.clone() * M31_511.clone()),
                (mid_limbs_set_col6.clone() * M31_511.clone()),
                (mid_limbs_set_col6.clone() * M31_511.clone()),
                (mid_limbs_set_col6.clone() * M31_511.clone()),
                (mid_limbs_set_col6.clone() * M31_511.clone()),
                (mid_limbs_set_col6.clone() * M31_511.clone()),
                (mid_limbs_set_col6.clone() * M31_511.clone()),
                (mid_limbs_set_col6.clone() * M31_511.clone()),
                (mid_limbs_set_col6.clone() * M31_511.clone()),
                (mid_limbs_set_col6.clone() * M31_511.clone()),
                (mid_limbs_set_col6.clone() * M31_511.clone()),
                (mid_limbs_set_col6.clone() * M31_511.clone()),
                (mid_limbs_set_col6.clone() * M31_511.clone()),
                ((M31_136.clone() * msb_col5.clone()) - mid_limbs_set_col6.clone()),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                (msb_col5.clone() * M31_256.clone()),
            ]),
        );
        logup.write_frac(&mut eval, frac);

        // let frac = Fraction::new(
        //     E::EF::one(),
        //     self.opcodes_lookup_elements.combine(&[
        //         input_pc_col0.clone(),
        //         input_ap_col1.clone(),
        //         input_fp_col2.clone(),
        //     ]),
        // );
        // logup.write_frac(&mut eval, frac);
        // let frac = Fraction::new(
        //     -E::EF::one(),
        //     self.opcodes_lookup_elements.combine(&[
        //         (input_pc_col0.clone()
        //             + ((((next_pc_limb_0_col7.clone()
        //                 + (next_pc_limb_1_col8.clone() * M31_512.clone()))
        //                 + (next_pc_limb_2_col9.clone() * M31_262144.clone()))
        //                 - msb_col5.clone())
        //                 - (M31_134217728.clone() * mid_limbs_set_col6.clone()))),
        //         (input_ap_col1.clone() + ap_update_add_1_col3.clone()),
        //         input_fp_col2.clone(),
        //     ]),
        // );
        // logup.write_frac(&mut eval, frac);
        logup.finalize(&mut eval);

        eval
    }
}
