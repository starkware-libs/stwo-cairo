// AIR version 52ac7695-dirty
use crate::components::prelude::*;
use crate::components::subroutines::bitwise_and_num_bits_8::BitwiseAndNumBits8;
use crate::components::subroutines::bitwise_xor_num_bits_8::BitwiseXorNumBits8;
use crate::components::subroutines::split_16_low_part_size_8::Split16LowPartSize8;

pub const N_TRACE_COLUMNS: usize = 43;
pub const RELATION_USES_PER_ROW: [RelationUse; 4] = [
    RelationUse {
        relation_id: "Sha256BigSigma0O0",
        uses: 1,
    },
    RelationUse {
        relation_id: "Sha256BigSigma0O1",
        uses: 1,
    },
    RelationUse {
        relation_id: "VerifyBitwiseAnd_8",
        uses: 12,
    },
    RelationUse {
        relation_id: "VerifyBitwiseXor_8",
        uses: 4,
    },
];

pub struct Eval {
    pub claim: Claim,
    pub verify_bitwise_and_8_lookup_elements: relations::VerifyBitwiseAnd_8,
    pub sha_256_big_sigma_0_o_0_lookup_elements: relations::Sha256BigSigma0O0,
    pub sha_256_big_sigma_0_o_1_lookup_elements: relations::Sha256BigSigma0O1,
    pub verify_bitwise_xor_8_lookup_elements: relations::VerifyBitwiseXor_8,
    pub sha_256_big_sigma_0_lookup_elements: relations::Sha256BigSigma0,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct Claim {
    pub log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 10];
        TreeVec::new(vec![vec![], trace_log_sizes, interaction_log_sizes])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(self.log_size as u64);
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
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
        let M31_124 = E::F::from(M31::from(124));
        let M31_131 = E::F::from(M31::from(131));
        let M31_15 = E::F::from(M31::from(15));
        let M31_240 = E::F::from(M31::from(240));
        let M31_256 = E::F::from(M31::from(256));
        let input_limb_0_col0 = eval.next_trace_mask();
        let input_limb_1_col1 = eval.next_trace_mask();
        let ms_8_bits_col2 = eval.next_trace_mask();
        let ms_8_bits_col3 = eval.next_trace_mask();
        let and_col4 = eval.next_trace_mask();
        let and_col5 = eval.next_trace_mask();
        let l0_col6 = eval.next_trace_mask();
        let and_col7 = eval.next_trace_mask();
        let and_col8 = eval.next_trace_mask();
        let l1_col9 = eval.next_trace_mask();
        let and_col10 = eval.next_trace_mask();
        let and_col11 = eval.next_trace_mask();
        let l2_col12 = eval.next_trace_mask();
        let and_col13 = eval.next_trace_mask();
        let and_col14 = eval.next_trace_mask();
        let h0_col15 = eval.next_trace_mask();
        let and_col16 = eval.next_trace_mask();
        let and_col17 = eval.next_trace_mask();
        let h1_col18 = eval.next_trace_mask();
        let and_col19 = eval.next_trace_mask();
        let and_col20 = eval.next_trace_mask();
        let h2_col21 = eval.next_trace_mask();
        let sigma_O0_L_col22 = eval.next_trace_mask();
        let sigma_O0_H_col23 = eval.next_trace_mask();
        let sigma_O1_L_col24 = eval.next_trace_mask();
        let sigma_O1_H_col25 = eval.next_trace_mask();
        let sigma_O2_L_col26 = eval.next_trace_mask();
        let sigma_O2_H_col27 = eval.next_trace_mask();
        let sigma_O2_prime_L_col28 = eval.next_trace_mask();
        let sigma_O2_prime_H_col29 = eval.next_trace_mask();
        let ms_8_bits_col30 = eval.next_trace_mask();
        let ms_8_bits_col31 = eval.next_trace_mask();
        let xor_col32 = eval.next_trace_mask();
        let xor_col33 = eval.next_trace_mask();
        let output2l_col34 = eval.next_trace_mask();
        let ms_8_bits_col35 = eval.next_trace_mask();
        let ms_8_bits_col36 = eval.next_trace_mask();
        let xor_col37 = eval.next_trace_mask();
        let xor_col38 = eval.next_trace_mask();
        let output2h_col39 = eval.next_trace_mask();
        let output_low_col40 = eval.next_trace_mask();
        let output_high_col41 = eval.next_trace_mask();
        let enabler = eval.next_trace_mask();

        eval.add_constraint(enabler.clone() * enabler.clone() - enabler.clone());

        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [split_16_low_part_size_8_output_tmp_e3d2a_1_limb_0] = Split16LowPartSize8::evaluate(
            [input_limb_0_col0.clone()],
            ms_8_bits_col2.clone(),
            &mut eval,
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [split_16_low_part_size_8_output_tmp_e3d2a_3_limb_0] = Split16LowPartSize8::evaluate(
            [input_limb_1_col1.clone()],
            ms_8_bits_col3.clone(),
            &mut eval,
        );
        BitwiseAndNumBits8::evaluate(
            [
                split_16_low_part_size_8_output_tmp_e3d2a_1_limb_0.clone(),
                M31_131.clone(),
            ],
            and_col4.clone(),
            &self.verify_bitwise_and_8_lookup_elements,
            &mut eval,
        );
        BitwiseAndNumBits8::evaluate(
            [ms_8_bits_col2.clone(), M31_15.clone()],
            and_col5.clone(),
            &self.verify_bitwise_and_8_lookup_elements,
            &mut eval,
        );
        // l0.
        eval.add_constraint(
            (l0_col6.clone() - (and_col4.clone() + (and_col5.clone() * M31_256.clone()))),
        );
        BitwiseAndNumBits8::evaluate(
            [
                split_16_low_part_size_8_output_tmp_e3d2a_1_limb_0.clone(),
                M31_124.clone(),
            ],
            and_col7.clone(),
            &self.verify_bitwise_and_8_lookup_elements,
            &mut eval,
        );
        BitwiseAndNumBits8::evaluate(
            [ms_8_bits_col2.clone(), M31_0.clone()],
            and_col8.clone(),
            &self.verify_bitwise_and_8_lookup_elements,
            &mut eval,
        );
        // l1.
        eval.add_constraint(
            (l1_col9.clone() - (and_col7.clone() + (and_col8.clone() * M31_256.clone()))),
        );
        BitwiseAndNumBits8::evaluate(
            [
                split_16_low_part_size_8_output_tmp_e3d2a_1_limb_0.clone(),
                M31_0.clone(),
            ],
            and_col10.clone(),
            &self.verify_bitwise_and_8_lookup_elements,
            &mut eval,
        );
        BitwiseAndNumBits8::evaluate(
            [ms_8_bits_col2.clone(), M31_240.clone()],
            and_col11.clone(),
            &self.verify_bitwise_and_8_lookup_elements,
            &mut eval,
        );
        // l2.
        eval.add_constraint(
            (l2_col12.clone() - (and_col10.clone() + (and_col11.clone() * M31_256.clone()))),
        );
        BitwiseAndNumBits8::evaluate(
            [
                split_16_low_part_size_8_output_tmp_e3d2a_3_limb_0.clone(),
                M31_124.clone(),
            ],
            and_col13.clone(),
            &self.verify_bitwise_and_8_lookup_elements,
            &mut eval,
        );
        BitwiseAndNumBits8::evaluate(
            [ms_8_bits_col3.clone(), M31_0.clone()],
            and_col14.clone(),
            &self.verify_bitwise_and_8_lookup_elements,
            &mut eval,
        );
        // h0.
        eval.add_constraint(
            (h0_col15.clone() - (and_col13.clone() + (and_col14.clone() * M31_256.clone()))),
        );
        BitwiseAndNumBits8::evaluate(
            [
                split_16_low_part_size_8_output_tmp_e3d2a_3_limb_0.clone(),
                M31_0.clone(),
            ],
            and_col16.clone(),
            &self.verify_bitwise_and_8_lookup_elements,
            &mut eval,
        );
        BitwiseAndNumBits8::evaluate(
            [ms_8_bits_col3.clone(), M31_240.clone()],
            and_col17.clone(),
            &self.verify_bitwise_and_8_lookup_elements,
            &mut eval,
        );
        // h1.
        eval.add_constraint(
            (h1_col18.clone() - (and_col16.clone() + (and_col17.clone() * M31_256.clone()))),
        );
        BitwiseAndNumBits8::evaluate(
            [
                split_16_low_part_size_8_output_tmp_e3d2a_3_limb_0.clone(),
                M31_131.clone(),
            ],
            and_col19.clone(),
            &self.verify_bitwise_and_8_lookup_elements,
            &mut eval,
        );
        BitwiseAndNumBits8::evaluate(
            [ms_8_bits_col3.clone(), M31_15.clone()],
            and_col20.clone(),
            &self.verify_bitwise_and_8_lookup_elements,
            &mut eval,
        );
        // h2.
        eval.add_constraint(
            (h2_col21.clone() - (and_col19.clone() + (and_col20.clone() * M31_256.clone()))),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.sha_256_big_sigma_0_o_0_lookup_elements,
            E::EF::one(),
            &[
                l0_col6.clone(),
                (h0_col15.clone() + h1_col18.clone()),
                sigma_O0_L_col22.clone(),
                sigma_O0_H_col23.clone(),
                sigma_O2_L_col26.clone(),
                sigma_O2_H_col27.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.sha_256_big_sigma_0_o_1_lookup_elements,
            E::EF::one(),
            &[
                (l1_col9.clone() + l2_col12.clone()),
                h2_col21.clone(),
                sigma_O1_L_col24.clone(),
                sigma_O1_H_col25.clone(),
                sigma_O2_prime_L_col28.clone(),
                sigma_O2_prime_H_col29.clone(),
            ],
        ));

        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [split_16_low_part_size_8_output_tmp_e3d2a_39_limb_0] = Split16LowPartSize8::evaluate(
            [sigma_O2_prime_L_col28.clone()],
            ms_8_bits_col30.clone(),
            &mut eval,
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [split_16_low_part_size_8_output_tmp_e3d2a_41_limb_0] = Split16LowPartSize8::evaluate(
            [sigma_O2_L_col26.clone()],
            ms_8_bits_col31.clone(),
            &mut eval,
        );
        BitwiseXorNumBits8::evaluate(
            [
                split_16_low_part_size_8_output_tmp_e3d2a_39_limb_0.clone(),
                split_16_low_part_size_8_output_tmp_e3d2a_41_limb_0.clone(),
            ],
            xor_col32.clone(),
            &self.verify_bitwise_xor_8_lookup_elements,
            &mut eval,
        );
        BitwiseXorNumBits8::evaluate(
            [ms_8_bits_col30.clone(), ms_8_bits_col31.clone()],
            xor_col33.clone(),
            &self.verify_bitwise_xor_8_lookup_elements,
            &mut eval,
        );
        // output2l.
        eval.add_constraint(
            (output2l_col34.clone() - (xor_col32.clone() + (xor_col33.clone() * M31_256.clone()))),
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [split_16_low_part_size_8_output_tmp_e3d2a_47_limb_0] = Split16LowPartSize8::evaluate(
            [sigma_O2_prime_H_col29.clone()],
            ms_8_bits_col35.clone(),
            &mut eval,
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [split_16_low_part_size_8_output_tmp_e3d2a_49_limb_0] = Split16LowPartSize8::evaluate(
            [sigma_O2_H_col27.clone()],
            ms_8_bits_col36.clone(),
            &mut eval,
        );
        BitwiseXorNumBits8::evaluate(
            [
                split_16_low_part_size_8_output_tmp_e3d2a_47_limb_0.clone(),
                split_16_low_part_size_8_output_tmp_e3d2a_49_limb_0.clone(),
            ],
            xor_col37.clone(),
            &self.verify_bitwise_xor_8_lookup_elements,
            &mut eval,
        );
        BitwiseXorNumBits8::evaluate(
            [ms_8_bits_col35.clone(), ms_8_bits_col36.clone()],
            xor_col38.clone(),
            &self.verify_bitwise_xor_8_lookup_elements,
            &mut eval,
        );
        // output2h.
        eval.add_constraint(
            (output2h_col39.clone() - (xor_col37.clone() + (xor_col38.clone() * M31_256.clone()))),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.sha_256_big_sigma_0_lookup_elements,
            -E::EF::from(enabler.clone()),
            &[
                input_limb_0_col0.clone(),
                input_limb_1_col1.clone(),
                output_low_col40.clone(),
                output_high_col41.clone(),
            ],
        ));

        eval.finalize_logup_in_pairs();
        eval
    }
}

#[cfg(test)]
mod tests {
    use num_traits::Zero;
    use rand::rngs::SmallRng;
    use rand::{Rng, SeedableRng};
    use stwo::core::fields::qm31::QM31;
    use stwo_constraint_framework::expr::ExprEvaluator;

    use super::*;
    use crate::components::constraints_regression_test_values::SHA_256_BIG_SIGMA_0;

    #[test]
    fn sha_256_big_sigma_0_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim { log_size: 4 },
            verify_bitwise_and_8_lookup_elements: relations::VerifyBitwiseAnd_8::dummy(),
            sha_256_big_sigma_0_o_0_lookup_elements: relations::Sha256BigSigma0O0::dummy(),
            sha_256_big_sigma_0_o_1_lookup_elements: relations::Sha256BigSigma0O1::dummy(),
            verify_bitwise_xor_8_lookup_elements: relations::VerifyBitwiseXor_8::dummy(),
            sha_256_big_sigma_0_lookup_elements: relations::Sha256BigSigma0::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        assert_eq!(sum, SHA_256_BIG_SIGMA_0);
    }
}
