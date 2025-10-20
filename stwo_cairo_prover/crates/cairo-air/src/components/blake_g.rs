// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::triple_sum_32::TripleSum32;
use crate::components::subroutines::xor_rot_32_r_12::XorRot32R12;
use crate::components::subroutines::xor_rot_32_r_16::XorRot32R16;
use crate::components::subroutines::xor_rot_32_r_7::XorRot32R7;
use crate::components::subroutines::xor_rot_32_r_8::XorRot32R8;

pub const N_TRACE_COLUMNS: usize = 53;
pub const RELATION_USES_PER_ROW: [RelationUse; 6] = [
    RelationUse {
        relation_id: "VerifyBitwiseXor_12",
        uses: 2,
    },
    RelationUse {
        relation_id: "VerifyBitwiseXor_4",
        uses: 2,
    },
    RelationUse {
        relation_id: "VerifyBitwiseXor_7",
        uses: 2,
    },
    RelationUse {
        relation_id: "VerifyBitwiseXor_8",
        uses: 4,
    },
    RelationUse {
        relation_id: "VerifyBitwiseXor_8_B",
        uses: 4,
    },
    RelationUse {
        relation_id: "VerifyBitwiseXor_9",
        uses: 2,
    },
];

pub struct Eval {
    pub claim: Claim,
    pub verify_bitwise_xor_8_lookup_elements: relations::VerifyBitwiseXor_8,
    pub verify_bitwise_xor_8_b_lookup_elements: relations::VerifyBitwiseXor_8_B,
    pub verify_bitwise_xor_12_lookup_elements: relations::VerifyBitwiseXor_12,
    pub verify_bitwise_xor_4_lookup_elements: relations::VerifyBitwiseXor_4,
    pub verify_bitwise_xor_7_lookup_elements: relations::VerifyBitwiseXor_7,
    pub verify_bitwise_xor_9_lookup_elements: relations::VerifyBitwiseXor_9,
    pub blake_g_lookup_elements: relations::BlakeG,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct Claim {
    pub log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 9];
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
        let input_limb_0_col0 = eval.next_trace_mask();
        let input_limb_1_col1 = eval.next_trace_mask();
        let input_limb_2_col2 = eval.next_trace_mask();
        let input_limb_3_col3 = eval.next_trace_mask();
        let input_limb_4_col4 = eval.next_trace_mask();
        let input_limb_5_col5 = eval.next_trace_mask();
        let input_limb_6_col6 = eval.next_trace_mask();
        let input_limb_7_col7 = eval.next_trace_mask();
        let input_limb_8_col8 = eval.next_trace_mask();
        let input_limb_9_col9 = eval.next_trace_mask();
        let input_limb_10_col10 = eval.next_trace_mask();
        let input_limb_11_col11 = eval.next_trace_mask();
        let triple_sum32_res_limb_0_col12 = eval.next_trace_mask();
        let triple_sum32_res_limb_1_col13 = eval.next_trace_mask();
        let ms_8_bits_col14 = eval.next_trace_mask();
        let ms_8_bits_col15 = eval.next_trace_mask();
        let ms_8_bits_col16 = eval.next_trace_mask();
        let ms_8_bits_col17 = eval.next_trace_mask();
        let xor_col18 = eval.next_trace_mask();
        let xor_col19 = eval.next_trace_mask();
        let xor_col20 = eval.next_trace_mask();
        let xor_col21 = eval.next_trace_mask();
        let triple_sum32_res_limb_0_col22 = eval.next_trace_mask();
        let triple_sum32_res_limb_1_col23 = eval.next_trace_mask();
        let ms_4_bits_col24 = eval.next_trace_mask();
        let ms_4_bits_col25 = eval.next_trace_mask();
        let ms_4_bits_col26 = eval.next_trace_mask();
        let ms_4_bits_col27 = eval.next_trace_mask();
        let xor_col28 = eval.next_trace_mask();
        let xor_col29 = eval.next_trace_mask();
        let xor_col30 = eval.next_trace_mask();
        let xor_col31 = eval.next_trace_mask();
        let triple_sum32_res_limb_0_col32 = eval.next_trace_mask();
        let triple_sum32_res_limb_1_col33 = eval.next_trace_mask();
        let ms_8_bits_col34 = eval.next_trace_mask();
        let ms_8_bits_col35 = eval.next_trace_mask();
        let ms_8_bits_col36 = eval.next_trace_mask();
        let ms_8_bits_col37 = eval.next_trace_mask();
        let xor_col38 = eval.next_trace_mask();
        let xor_col39 = eval.next_trace_mask();
        let xor_col40 = eval.next_trace_mask();
        let xor_col41 = eval.next_trace_mask();
        let triple_sum32_res_limb_0_col42 = eval.next_trace_mask();
        let triple_sum32_res_limb_1_col43 = eval.next_trace_mask();
        let ms_9_bits_col44 = eval.next_trace_mask();
        let ms_9_bits_col45 = eval.next_trace_mask();
        let ms_9_bits_col46 = eval.next_trace_mask();
        let ms_9_bits_col47 = eval.next_trace_mask();
        let xor_col48 = eval.next_trace_mask();
        let xor_col49 = eval.next_trace_mask();
        let xor_col50 = eval.next_trace_mask();
        let xor_col51 = eval.next_trace_mask();
        let enabler = eval.next_trace_mask();

        eval.add_constraint(enabler.clone() * enabler.clone() - enabler.clone());

        TripleSum32::evaluate(
            [
                input_limb_0_col0.clone(),
                input_limb_1_col1.clone(),
                input_limb_2_col2.clone(),
                input_limb_3_col3.clone(),
                input_limb_8_col8.clone(),
                input_limb_9_col9.clone(),
            ],
            triple_sum32_res_limb_0_col12.clone(),
            triple_sum32_res_limb_1_col13.clone(),
            &mut eval,
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [xor_rot_32_r_16_output_tmp_f72c8_21_limb_0, xor_rot_32_r_16_output_tmp_f72c8_21_limb_1] =
            XorRot32R16::evaluate(
                [
                    triple_sum32_res_limb_0_col12.clone(),
                    triple_sum32_res_limb_1_col13.clone(),
                    input_limb_6_col6.clone(),
                    input_limb_7_col7.clone(),
                ],
                ms_8_bits_col14.clone(),
                ms_8_bits_col15.clone(),
                ms_8_bits_col16.clone(),
                ms_8_bits_col17.clone(),
                xor_col18.clone(),
                xor_col19.clone(),
                xor_col20.clone(),
                xor_col21.clone(),
                &self.verify_bitwise_xor_8_lookup_elements,
                &self.verify_bitwise_xor_8_b_lookup_elements,
                &mut eval,
            );
        TripleSum32::evaluate(
            [
                input_limb_4_col4.clone(),
                input_limb_5_col5.clone(),
                xor_rot_32_r_16_output_tmp_f72c8_21_limb_0.clone(),
                xor_rot_32_r_16_output_tmp_f72c8_21_limb_1.clone(),
                M31_0.clone(),
                M31_0.clone(),
            ],
            triple_sum32_res_limb_0_col22.clone(),
            triple_sum32_res_limb_1_col23.clone(),
            &mut eval,
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [xor_rot_32_r_12_output_tmp_f72c8_43_limb_0, xor_rot_32_r_12_output_tmp_f72c8_43_limb_1] =
            XorRot32R12::evaluate(
                [
                    input_limb_2_col2.clone(),
                    input_limb_3_col3.clone(),
                    triple_sum32_res_limb_0_col22.clone(),
                    triple_sum32_res_limb_1_col23.clone(),
                ],
                ms_4_bits_col24.clone(),
                ms_4_bits_col25.clone(),
                ms_4_bits_col26.clone(),
                ms_4_bits_col27.clone(),
                xor_col28.clone(),
                xor_col29.clone(),
                xor_col30.clone(),
                xor_col31.clone(),
                &self.verify_bitwise_xor_12_lookup_elements,
                &self.verify_bitwise_xor_4_lookup_elements,
                &mut eval,
            );
        TripleSum32::evaluate(
            [
                triple_sum32_res_limb_0_col12.clone(),
                triple_sum32_res_limb_1_col13.clone(),
                xor_rot_32_r_12_output_tmp_f72c8_43_limb_0.clone(),
                xor_rot_32_r_12_output_tmp_f72c8_43_limb_1.clone(),
                input_limb_10_col10.clone(),
                input_limb_11_col11.clone(),
            ],
            triple_sum32_res_limb_0_col32.clone(),
            triple_sum32_res_limb_1_col33.clone(),
            &mut eval,
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [xor_rot_32_r_8_output_tmp_f72c8_65_limb_0, xor_rot_32_r_8_output_tmp_f72c8_65_limb_1] =
            XorRot32R8::evaluate(
                [
                    triple_sum32_res_limb_0_col32.clone(),
                    triple_sum32_res_limb_1_col33.clone(),
                    xor_rot_32_r_16_output_tmp_f72c8_21_limb_0.clone(),
                    xor_rot_32_r_16_output_tmp_f72c8_21_limb_1.clone(),
                ],
                ms_8_bits_col34.clone(),
                ms_8_bits_col35.clone(),
                ms_8_bits_col36.clone(),
                ms_8_bits_col37.clone(),
                xor_col38.clone(),
                xor_col39.clone(),
                xor_col40.clone(),
                xor_col41.clone(),
                &self.verify_bitwise_xor_8_lookup_elements,
                &self.verify_bitwise_xor_8_b_lookup_elements,
                &mut eval,
            );
        TripleSum32::evaluate(
            [
                triple_sum32_res_limb_0_col22.clone(),
                triple_sum32_res_limb_1_col23.clone(),
                xor_rot_32_r_8_output_tmp_f72c8_65_limb_0.clone(),
                xor_rot_32_r_8_output_tmp_f72c8_65_limb_1.clone(),
                M31_0.clone(),
                M31_0.clone(),
            ],
            triple_sum32_res_limb_0_col42.clone(),
            triple_sum32_res_limb_1_col43.clone(),
            &mut eval,
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [xor_rot_32_r_7_output_tmp_f72c8_87_limb_0, xor_rot_32_r_7_output_tmp_f72c8_87_limb_1] =
            XorRot32R7::evaluate(
                [
                    xor_rot_32_r_12_output_tmp_f72c8_43_limb_0.clone(),
                    xor_rot_32_r_12_output_tmp_f72c8_43_limb_1.clone(),
                    triple_sum32_res_limb_0_col42.clone(),
                    triple_sum32_res_limb_1_col43.clone(),
                ],
                ms_9_bits_col44.clone(),
                ms_9_bits_col45.clone(),
                ms_9_bits_col46.clone(),
                ms_9_bits_col47.clone(),
                xor_col48.clone(),
                xor_col49.clone(),
                xor_col50.clone(),
                xor_col51.clone(),
                &self.verify_bitwise_xor_7_lookup_elements,
                &self.verify_bitwise_xor_9_lookup_elements,
                &mut eval,
            );
        eval.add_to_relation(RelationEntry::new(
            &self.blake_g_lookup_elements,
            -E::EF::from(enabler.clone()),
            &[
                input_limb_0_col0.clone(),
                input_limb_1_col1.clone(),
                input_limb_2_col2.clone(),
                input_limb_3_col3.clone(),
                input_limb_4_col4.clone(),
                input_limb_5_col5.clone(),
                input_limb_6_col6.clone(),
                input_limb_7_col7.clone(),
                input_limb_8_col8.clone(),
                input_limb_9_col9.clone(),
                input_limb_10_col10.clone(),
                input_limb_11_col11.clone(),
                triple_sum32_res_limb_0_col32.clone(),
                triple_sum32_res_limb_1_col33.clone(),
                xor_rot_32_r_7_output_tmp_f72c8_87_limb_0.clone(),
                xor_rot_32_r_7_output_tmp_f72c8_87_limb_1.clone(),
                triple_sum32_res_limb_0_col42.clone(),
                triple_sum32_res_limb_1_col43.clone(),
                xor_rot_32_r_8_output_tmp_f72c8_65_limb_0.clone(),
                xor_rot_32_r_8_output_tmp_f72c8_65_limb_1.clone(),
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
    use crate::components::constraints_regression_test_values::BLAKE_G;

    #[test]
    fn blake_g_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim { log_size: 4 },
            verify_bitwise_xor_8_lookup_elements: relations::VerifyBitwiseXor_8::dummy(),
            verify_bitwise_xor_8_b_lookup_elements: relations::VerifyBitwiseXor_8_B::dummy(),
            verify_bitwise_xor_12_lookup_elements: relations::VerifyBitwiseXor_12::dummy(),
            verify_bitwise_xor_4_lookup_elements: relations::VerifyBitwiseXor_4::dummy(),
            verify_bitwise_xor_7_lookup_elements: relations::VerifyBitwiseXor_7::dummy(),
            verify_bitwise_xor_9_lookup_elements: relations::VerifyBitwiseXor_9::dummy(),
            blake_g_lookup_elements: relations::BlakeG::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        assert_eq!(sum, BLAKE_G);
    }
}
