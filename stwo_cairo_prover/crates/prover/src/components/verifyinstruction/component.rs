#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use std::ops::{Mul, Sub};

use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};
use stwo_prover::constraint_framework::logup::{ClaimedPrefixSum, LogupAtRow, LookupElements};
use stwo_prover::constraint_framework::preprocessed_columns::PreprocessedColumn;
use stwo_prover::constraint_framework::{
    EvalAtRow, FrameworkComponent, FrameworkEval, RelationEntry, INTERACTION_TRACE_IDX,
};
use stwo_prover::core::backend::simd::m31::PackedM31;
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::fields::secure_column::SECURE_EXTENSION_DEGREE;
use stwo_prover::core::lookups::utils::Fraction;
use stwo_prover::core::pcs::TreeVec;
use stwo_prover::relation;

use crate::components::range_check_vector::{range_check_4_3, range_check_7_2_5};
use crate::components::{memory, verifyinstruction};
use crate::relations;

relation!(RelationElements, 30);

pub struct Eval {
    pub claim: Claim,
    pub memoryaddresstoid_lookup_elements: relations::AddrToId,
    pub memoryidtobig_lookup_elements: relations::IdToValue,
    pub rangecheck_4_3_lookup_elements: relations::RangeCheck_4_3,
    pub range_check_7_2_5_lookup_elements: relations::RangeCheck_7_2_5,
    pub verifyinstruction_lookup_elements: relations::VerifyInstruction,
}
impl Eval {
    pub fn new(
        claim: Claim,
        memoryaddresstoid_lookup_elements: relations::AddrToId,
        memoryidtobig_lookup_elements: relations::IdToValue,
        rangecheck_4_3_lookup_elements: relations::RangeCheck_4_3,
        range_check_7_2_5_lookup_elements: relations::RangeCheck_7_2_5,
        verifyinstruction_lookup_elements: relations::VerifyInstruction,
    ) -> Self {
        Self {
            claim,
            memoryaddresstoid_lookup_elements,
            memoryidtobig_lookup_elements,
            rangecheck_4_3_lookup_elements,
            range_check_7_2_5_lookup_elements,
            verifyinstruction_lookup_elements,
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Claim {
    pub n_calls: usize,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let log_size = self.n_calls.next_power_of_two().ilog2();
        let preprocesed_trace_log_sizes = vec![log_size];
        let trace_log_sizes = vec![log_size; 28];
        let interaction_log_sizes = vec![log_size; SECURE_EXTENSION_DEGREE * 5];
        TreeVec::new(vec![
            preprocesed_trace_log_sizes,
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
    }
}

pub type Component = FrameworkComponent<Eval>;

impl FrameworkEval for Eval {
    fn log_size(&self) -> u32 {
        self.claim.n_calls.next_power_of_two().ilog2()
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
        let M31_16 = E::F::from(M31::from(16));
        let M31_2 = E::F::from(M31::from(2));
        let M31_2048 = E::F::from(M31::from(2048));
        let M31_256 = E::F::from(M31::from(256));
        let M31_32 = E::F::from(M31::from(32));
        let M31_4 = E::F::from(M31::from(4));
        let M31_512 = E::F::from(M31::from(512));
        let M31_64 = E::F::from(M31::from(64));
        let M31_8 = E::F::from(M31::from(8));
        let M31_8192 = E::F::from(M31::from(8192));
        let input_col0 = eval.next_trace_mask();
        let input_col1 = eval.next_trace_mask();
        let input_col2 = eval.next_trace_mask();
        let input_col3 = eval.next_trace_mask();
        let input_col4 = eval.next_trace_mask();
        let input_col5 = eval.next_trace_mask();
        let input_col6 = eval.next_trace_mask();
        let input_col7 = eval.next_trace_mask();
        let input_col8 = eval.next_trace_mask();
        let input_col9 = eval.next_trace_mask();
        let input_col10 = eval.next_trace_mask();
        let input_col11 = eval.next_trace_mask();
        let input_col12 = eval.next_trace_mask();
        let input_col13 = eval.next_trace_mask();
        let input_col14 = eval.next_trace_mask();
        let input_col15 = eval.next_trace_mask();
        let input_col16 = eval.next_trace_mask();
        let input_col17 = eval.next_trace_mask();
        let input_col18 = eval.next_trace_mask();
        let offset0_low_col19 = eval.next_trace_mask();
        let offset0_mid_col20 = eval.next_trace_mask();
        let offset1_low_col21 = eval.next_trace_mask();
        let offset1_mid_col22 = eval.next_trace_mask();
        let offset1_high_col23 = eval.next_trace_mask();
        let offset2_low_col24 = eval.next_trace_mask();
        let offset2_mid_col25 = eval.next_trace_mask();
        let offset2_high_col26 = eval.next_trace_mask();
        let instruction_id_col27 = eval.next_trace_mask();
        eval.add_constraint(
            ((offset0_low_col19.clone() + (offset0_mid_col20.clone() * M31_512.clone()))
                - input_col1.clone()),
        );
        eval.add_constraint(
            (((offset1_low_col21.clone() + (offset1_mid_col22.clone() * M31_4.clone()))
                + (offset1_high_col23.clone() * M31_2048.clone()))
                - input_col2.clone()),
        );
        eval.add_constraint(
            (((offset2_low_col24.clone() + (offset2_mid_col25.clone() * M31_16.clone()))
                + (offset2_high_col26.clone() * M31_8192.clone()))
                - input_col3.clone()),
        );
        eval.add_to_relation(&[RelationEntry::new(
            &self.range_check_7_2_5_lookup_elements,
            E::EF::one(),
            &[
                offset0_mid_col20.clone(),
                offset1_low_col21.clone(),
                offset1_high_col23.clone(),
            ],
        )]);

        eval.add_to_relation(&[RelationEntry::new(
            &self.rangecheck_4_3_lookup_elements,
            E::EF::one(),
            &[offset2_low_col24.clone(), offset2_high_col26.clone()],
        )]);
        eval.add_constraint((input_col4.clone() * (M31_1.clone() - input_col4.clone())));
        eval.add_constraint((input_col5.clone() * (M31_1.clone() - input_col5.clone())));
        eval.add_constraint((input_col6.clone() * (M31_1.clone() - input_col6.clone())));
        eval.add_constraint((input_col7.clone() * (M31_1.clone() - input_col7.clone())));
        eval.add_constraint((input_col8.clone() * (M31_1.clone() - input_col8.clone())));
        eval.add_constraint((input_col9.clone() * (M31_1.clone() - input_col9.clone())));
        eval.add_constraint((input_col10.clone() * (M31_1.clone() - input_col10.clone())));
        eval.add_constraint((input_col11.clone() * (M31_1.clone() - input_col11.clone())));
        eval.add_constraint((input_col12.clone() * (M31_1.clone() - input_col12.clone())));
        eval.add_constraint((input_col13.clone() * (M31_1.clone() - input_col13.clone())));
        eval.add_constraint((input_col14.clone() * (M31_1.clone() - input_col14.clone())));
        eval.add_constraint((input_col15.clone() * (M31_1.clone() - input_col15.clone())));
        eval.add_constraint((input_col16.clone() * (M31_1.clone() - input_col16.clone())));
        eval.add_constraint((input_col17.clone() * (M31_1.clone() - input_col17.clone())));
        eval.add_constraint((input_col18.clone() * (M31_1.clone() - input_col18.clone())));
        eval.add_to_relation(&[RelationEntry::new(
            &self.memoryaddresstoid_lookup_elements,
            E::EF::one(),
            &[input_col0.clone(), instruction_id_col27.clone()],
        )]);

        eval.add_to_relation(&[RelationEntry::new(
            &self.memoryidtobig_lookup_elements,
            E::EF::one(),
            &[
                instruction_id_col27.clone(),
                offset0_low_col19.clone(),
                (offset0_mid_col20.clone() + (offset1_low_col21.clone() * M31_128.clone())),
                offset1_mid_col22.clone(),
                (offset1_high_col23.clone() + (offset2_low_col24.clone() * M31_32.clone())),
                offset2_mid_col25.clone(),
                (offset2_high_col26.clone()
                    + ((((((M31_0.clone() + (input_col4.clone() * M31_8.clone()))
                        + (input_col5.clone() * M31_16.clone()))
                        + (input_col6.clone() * M31_32.clone()))
                        + (input_col7.clone() * M31_64.clone()))
                        + (input_col8.clone() * M31_128.clone()))
                        + (input_col9.clone() * M31_256.clone()))),
                (((((((((M31_0.clone() + (input_col10.clone() * M31_1.clone()))
                    + (input_col11.clone() * M31_2.clone()))
                    + (input_col12.clone() * M31_4.clone()))
                    + (input_col13.clone() * M31_8.clone()))
                    + (input_col14.clone() * M31_16.clone()))
                    + (input_col15.clone() * M31_32.clone()))
                    + (input_col16.clone() * M31_64.clone()))
                    + (input_col17.clone() * M31_128.clone()))
                    + (input_col18.clone() * M31_256.clone())),
            ],
        )]);

        eval.add_to_relation(&[RelationEntry::new(
            &self.verifyinstruction_lookup_elements,
            -E::EF::one(),
            &[
                input_col0.clone(),
                input_col1.clone(),
                input_col2.clone(),
                input_col3.clone(),
                input_col4.clone(),
                input_col5.clone(),
                input_col6.clone(),
                input_col7.clone(),
                input_col8.clone(),
                input_col9.clone(),
                input_col10.clone(),
                input_col11.clone(),
                input_col12.clone(),
                input_col13.clone(),
                input_col14.clone(),
                input_col15.clone(),
                input_col16.clone(),
                input_col17.clone(),
                input_col18.clone(),
            ],
        )]);

        eval.finalize_logup();
        eval
    }
}
