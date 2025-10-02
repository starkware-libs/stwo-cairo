// AIR version 98896da1
use crate::components::prelude::*;
use crate::components::subroutines::bitwise_and_num_bits_16::BitwiseAndNumBits16;
use crate::components::subroutines::bitwise_not_num_bits_16::BitwiseNotNumBits16;
use crate::components::subroutines::bitwise_xor_num_bits_16::BitwiseXorNumBits16;
use crate::components::subroutines::triple_sum_32::TripleSum32;

pub const N_TRACE_COLUMNS: usize = 91;
pub const RELATION_USES_PER_ROW: [RelationUse; 8] = [
    RelationUse {
        relation_id: "Sha256BigSigma0",
        uses: 1,
    },
    RelationUse {
        relation_id: "Sha256BigSigma1",
        uses: 1,
    },
    RelationUse {
        relation_id: "Sha256KTable",
        uses: 1,
    },
    RelationUse {
        relation_id: "Sha256Round",
        uses: 1,
    },
    RelationUse {
        relation_id: "Sha256Schedule",
        uses: 1,
    },
    RelationUse {
        relation_id: "VerifyBitwiseAnd_16",
        uses: 10,
    },
    RelationUse {
        relation_id: "VerifyBitwiseNot_16",
        uses: 2,
    },
    RelationUse {
        relation_id: "VerifyBitwiseXor_16",
        uses: 6,
    },
];

pub struct Eval {
    pub claim: Claim,
    pub sha_256_big_sigma_1_lookup_elements: relations::Sha256BigSigma1,
    pub sha_256_big_sigma_0_lookup_elements: relations::Sha256BigSigma0,
    pub verify_bitwise_and_16_lookup_elements: relations::VerifyBitwiseAnd_16,
    pub verify_bitwise_not_16_lookup_elements: relations::VerifyBitwiseNot_16,
    pub verify_bitwise_xor_16_lookup_elements: relations::VerifyBitwiseXor_16,
    pub sha_256_k_table_lookup_elements: relations::Sha256KTable,
    pub sha_256_schedule_lookup_elements: relations::Sha256Schedule,
    pub sha_256_round_lookup_elements: relations::Sha256Round,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct Claim {
    pub log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 12];
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
        let M31_1 = E::F::from(M31::from(1));
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
        let input_limb_12_col12 = eval.next_trace_mask();
        let input_limb_13_col13 = eval.next_trace_mask();
        let input_limb_14_col14 = eval.next_trace_mask();
        let input_limb_15_col15 = eval.next_trace_mask();
        let input_limb_16_col16 = eval.next_trace_mask();
        let input_limb_17_col17 = eval.next_trace_mask();
        let input_limb_18_col18 = eval.next_trace_mask();
        let input_limb_19_col19 = eval.next_trace_mask();
        let input_limb_20_col20 = eval.next_trace_mask();
        let input_limb_21_col21 = eval.next_trace_mask();
        let input_limb_22_col22 = eval.next_trace_mask();
        let input_limb_23_col23 = eval.next_trace_mask();
        let input_limb_24_col24 = eval.next_trace_mask();
        let input_limb_25_col25 = eval.next_trace_mask();
        let input_limb_26_col26 = eval.next_trace_mask();
        let input_limb_27_col27 = eval.next_trace_mask();
        let input_limb_28_col28 = eval.next_trace_mask();
        let input_limb_29_col29 = eval.next_trace_mask();
        let input_limb_30_col30 = eval.next_trace_mask();
        let input_limb_31_col31 = eval.next_trace_mask();
        let input_limb_32_col32 = eval.next_trace_mask();
        let input_limb_33_col33 = eval.next_trace_mask();
        let input_limb_34_col34 = eval.next_trace_mask();
        let input_limb_35_col35 = eval.next_trace_mask();
        let input_limb_36_col36 = eval.next_trace_mask();
        let input_limb_37_col37 = eval.next_trace_mask();
        let input_limb_38_col38 = eval.next_trace_mask();
        let input_limb_39_col39 = eval.next_trace_mask();
        let input_limb_40_col40 = eval.next_trace_mask();
        let input_limb_41_col41 = eval.next_trace_mask();
        let input_limb_42_col42 = eval.next_trace_mask();
        let input_limb_43_col43 = eval.next_trace_mask();
        let input_limb_44_col44 = eval.next_trace_mask();
        let input_limb_45_col45 = eval.next_trace_mask();
        let input_limb_46_col46 = eval.next_trace_mask();
        let input_limb_47_col47 = eval.next_trace_mask();
        let input_limb_48_col48 = eval.next_trace_mask();
        let input_limb_49_col49 = eval.next_trace_mask();
        let sha_256_big_sigma_1_output_limb_0_col50 = eval.next_trace_mask();
        let sha_256_big_sigma_1_output_limb_1_col51 = eval.next_trace_mask();
        let sha_256_big_sigma_0_output_limb_0_col52 = eval.next_trace_mask();
        let sha_256_big_sigma_0_output_limb_1_col53 = eval.next_trace_mask();
        let and_col54 = eval.next_trace_mask();
        let and_col55 = eval.next_trace_mask();
        let not_a_col56 = eval.next_trace_mask();
        let not_a_col57 = eval.next_trace_mask();
        let and_col58 = eval.next_trace_mask();
        let and_col59 = eval.next_trace_mask();
        let xor_col60 = eval.next_trace_mask();
        let xor_col61 = eval.next_trace_mask();
        let ch_limb_0_col62 = eval.next_trace_mask();
        let ch_limb_1_col63 = eval.next_trace_mask();
        let sha_256_k_table_output_limb_0_col64 = eval.next_trace_mask();
        let sha_256_k_table_output_limb_1_col65 = eval.next_trace_mask();
        let triple_sum32_res_limb_0_col66 = eval.next_trace_mask();
        let triple_sum32_res_limb_1_col67 = eval.next_trace_mask();
        let triple_sum32_res_limb_0_col68 = eval.next_trace_mask();
        let triple_sum32_res_limb_1_col69 = eval.next_trace_mask();
        let and_col70 = eval.next_trace_mask();
        let and_col71 = eval.next_trace_mask();
        let and_col72 = eval.next_trace_mask();
        let and_col73 = eval.next_trace_mask();
        let and_col74 = eval.next_trace_mask();
        let and_col75 = eval.next_trace_mask();
        let xor_col76 = eval.next_trace_mask();
        let xor_col77 = eval.next_trace_mask();
        let xor_col78 = eval.next_trace_mask();
        let xor_col79 = eval.next_trace_mask();
        let maj_limb_0_col80 = eval.next_trace_mask();
        let maj_limb_1_col81 = eval.next_trace_mask();
        let triple_sum32_res_limb_0_col82 = eval.next_trace_mask();
        let triple_sum32_res_limb_1_col83 = eval.next_trace_mask();
        let sha_256_schedule_output_limb_0_col84 = eval.next_trace_mask();
        let sha_256_schedule_output_limb_1_col85 = eval.next_trace_mask();
        let triple_sum32_res_limb_0_col86 = eval.next_trace_mask();
        let triple_sum32_res_limb_1_col87 = eval.next_trace_mask();
        let triple_sum32_res_limb_0_col88 = eval.next_trace_mask();
        let triple_sum32_res_limb_1_col89 = eval.next_trace_mask();
        let enabler = eval.next_trace_mask();

        eval.add_constraint(enabler.clone() * enabler.clone() - enabler.clone());

        eval.add_to_relation(RelationEntry::new(
            &self.sha_256_big_sigma_1_lookup_elements,
            E::EF::one(),
            &[
                input_limb_10_col10.clone(),
                input_limb_11_col11.clone(),
                sha_256_big_sigma_1_output_limb_0_col50.clone(),
                sha_256_big_sigma_1_output_limb_1_col51.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.sha_256_big_sigma_0_lookup_elements,
            E::EF::one(),
            &[
                input_limb_2_col2.clone(),
                input_limb_3_col3.clone(),
                sha_256_big_sigma_0_output_limb_0_col52.clone(),
                sha_256_big_sigma_0_output_limb_1_col53.clone(),
            ],
        ));

        BitwiseAndNumBits16::evaluate(
            [input_limb_10_col10.clone(), input_limb_12_col12.clone()],
            and_col54.clone(),
            &self.verify_bitwise_and_16_lookup_elements,
            &mut eval,
        );
        BitwiseAndNumBits16::evaluate(
            [input_limb_11_col11.clone(), input_limb_13_col13.clone()],
            and_col55.clone(),
            &self.verify_bitwise_and_16_lookup_elements,
            &mut eval,
        );
        BitwiseNotNumBits16::evaluate(
            [input_limb_10_col10.clone()],
            not_a_col56.clone(),
            &self.verify_bitwise_not_16_lookup_elements,
            &mut eval,
        );
        BitwiseNotNumBits16::evaluate(
            [input_limb_11_col11.clone()],
            not_a_col57.clone(),
            &self.verify_bitwise_not_16_lookup_elements,
            &mut eval,
        );
        BitwiseAndNumBits16::evaluate(
            [not_a_col56.clone(), input_limb_14_col14.clone()],
            and_col58.clone(),
            &self.verify_bitwise_and_16_lookup_elements,
            &mut eval,
        );
        BitwiseAndNumBits16::evaluate(
            [not_a_col57.clone(), input_limb_15_col15.clone()],
            and_col59.clone(),
            &self.verify_bitwise_and_16_lookup_elements,
            &mut eval,
        );
        BitwiseXorNumBits16::evaluate(
            [and_col54.clone(), and_col58.clone()],
            xor_col60.clone(),
            &self.verify_bitwise_xor_16_lookup_elements,
            &mut eval,
        );
        BitwiseXorNumBits16::evaluate(
            [and_col55.clone(), and_col59.clone()],
            xor_col61.clone(),
            &self.verify_bitwise_xor_16_lookup_elements,
            &mut eval,
        );
        eval.add_to_relation(RelationEntry::new(
            &self.sha_256_k_table_lookup_elements,
            E::EF::one(),
            &[
                input_limb_1_col1.clone(),
                sha_256_k_table_output_limb_0_col64.clone(),
                sha_256_k_table_output_limb_1_col65.clone(),
            ],
        ));

        TripleSum32::evaluate(
            [
                input_limb_16_col16.clone(),
                input_limb_17_col17.clone(),
                sha_256_big_sigma_1_output_limb_0_col50.clone(),
                sha_256_big_sigma_1_output_limb_1_col51.clone(),
                ch_limb_0_col62.clone(),
                ch_limb_1_col63.clone(),
            ],
            triple_sum32_res_limb_0_col66.clone(),
            triple_sum32_res_limb_1_col67.clone(),
            &mut eval,
        );
        TripleSum32::evaluate(
            [
                triple_sum32_res_limb_0_col66.clone(),
                triple_sum32_res_limb_1_col67.clone(),
                sha_256_k_table_output_limb_0_col64.clone(),
                sha_256_k_table_output_limb_1_col65.clone(),
                input_limb_18_col18.clone(),
                input_limb_19_col19.clone(),
            ],
            triple_sum32_res_limb_0_col68.clone(),
            triple_sum32_res_limb_1_col69.clone(),
            &mut eval,
        );
        BitwiseAndNumBits16::evaluate(
            [input_limb_2_col2.clone(), input_limb_4_col4.clone()],
            and_col70.clone(),
            &self.verify_bitwise_and_16_lookup_elements,
            &mut eval,
        );
        BitwiseAndNumBits16::evaluate(
            [input_limb_3_col3.clone(), input_limb_5_col5.clone()],
            and_col71.clone(),
            &self.verify_bitwise_and_16_lookup_elements,
            &mut eval,
        );
        BitwiseAndNumBits16::evaluate(
            [input_limb_2_col2.clone(), input_limb_6_col6.clone()],
            and_col72.clone(),
            &self.verify_bitwise_and_16_lookup_elements,
            &mut eval,
        );
        BitwiseAndNumBits16::evaluate(
            [input_limb_3_col3.clone(), input_limb_7_col7.clone()],
            and_col73.clone(),
            &self.verify_bitwise_and_16_lookup_elements,
            &mut eval,
        );
        BitwiseAndNumBits16::evaluate(
            [input_limb_4_col4.clone(), input_limb_6_col6.clone()],
            and_col74.clone(),
            &self.verify_bitwise_and_16_lookup_elements,
            &mut eval,
        );
        BitwiseAndNumBits16::evaluate(
            [input_limb_5_col5.clone(), input_limb_7_col7.clone()],
            and_col75.clone(),
            &self.verify_bitwise_and_16_lookup_elements,
            &mut eval,
        );
        BitwiseXorNumBits16::evaluate(
            [and_col70.clone(), and_col72.clone()],
            xor_col76.clone(),
            &self.verify_bitwise_xor_16_lookup_elements,
            &mut eval,
        );
        BitwiseXorNumBits16::evaluate(
            [and_col71.clone(), and_col73.clone()],
            xor_col77.clone(),
            &self.verify_bitwise_xor_16_lookup_elements,
            &mut eval,
        );
        BitwiseXorNumBits16::evaluate(
            [xor_col76.clone(), and_col74.clone()],
            xor_col78.clone(),
            &self.verify_bitwise_xor_16_lookup_elements,
            &mut eval,
        );
        BitwiseXorNumBits16::evaluate(
            [xor_col77.clone(), and_col75.clone()],
            xor_col79.clone(),
            &self.verify_bitwise_xor_16_lookup_elements,
            &mut eval,
        );
        TripleSum32::evaluate(
            [
                sha_256_big_sigma_0_output_limb_0_col52.clone(),
                sha_256_big_sigma_0_output_limb_1_col53.clone(),
                maj_limb_0_col80.clone(),
                maj_limb_1_col81.clone(),
                M31_0.clone(),
                M31_0.clone(),
            ],
            triple_sum32_res_limb_0_col82.clone(),
            triple_sum32_res_limb_1_col83.clone(),
            &mut eval,
        );
        eval.add_to_relation(RelationEntry::new(
            &self.sha_256_schedule_lookup_elements,
            E::EF::one(),
            &[
                input_limb_18_col18.clone(),
                input_limb_19_col19.clone(),
                input_limb_20_col20.clone(),
                input_limb_21_col21.clone(),
                input_limb_22_col22.clone(),
                input_limb_23_col23.clone(),
                input_limb_24_col24.clone(),
                input_limb_25_col25.clone(),
                input_limb_26_col26.clone(),
                input_limb_27_col27.clone(),
                input_limb_28_col28.clone(),
                input_limb_29_col29.clone(),
                input_limb_30_col30.clone(),
                input_limb_31_col31.clone(),
                input_limb_32_col32.clone(),
                input_limb_33_col33.clone(),
                input_limb_34_col34.clone(),
                input_limb_35_col35.clone(),
                input_limb_36_col36.clone(),
                input_limb_37_col37.clone(),
                input_limb_38_col38.clone(),
                input_limb_39_col39.clone(),
                input_limb_40_col40.clone(),
                input_limb_41_col41.clone(),
                input_limb_42_col42.clone(),
                input_limb_43_col43.clone(),
                input_limb_44_col44.clone(),
                input_limb_45_col45.clone(),
                input_limb_46_col46.clone(),
                input_limb_47_col47.clone(),
                input_limb_48_col48.clone(),
                input_limb_49_col49.clone(),
                sha_256_schedule_output_limb_0_col84.clone(),
                sha_256_schedule_output_limb_1_col85.clone(),
            ],
        ));

        TripleSum32::evaluate(
            [
                triple_sum32_res_limb_0_col68.clone(),
                triple_sum32_res_limb_1_col69.clone(),
                input_limb_8_col8.clone(),
                input_limb_9_col9.clone(),
                M31_0.clone(),
                M31_0.clone(),
            ],
            triple_sum32_res_limb_0_col86.clone(),
            triple_sum32_res_limb_1_col87.clone(),
            &mut eval,
        );
        TripleSum32::evaluate(
            [
                triple_sum32_res_limb_0_col68.clone(),
                triple_sum32_res_limb_1_col69.clone(),
                triple_sum32_res_limb_0_col82.clone(),
                triple_sum32_res_limb_1_col83.clone(),
                M31_0.clone(),
                M31_0.clone(),
            ],
            triple_sum32_res_limb_0_col88.clone(),
            triple_sum32_res_limb_1_col89.clone(),
            &mut eval,
        );
        eval.add_to_relation(RelationEntry::new(
            &self.sha_256_round_lookup_elements,
            E::EF::from(enabler.clone()),
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
                input_limb_12_col12.clone(),
                input_limb_13_col13.clone(),
                input_limb_14_col14.clone(),
                input_limb_15_col15.clone(),
                input_limb_16_col16.clone(),
                input_limb_17_col17.clone(),
                input_limb_18_col18.clone(),
                input_limb_19_col19.clone(),
                input_limb_20_col20.clone(),
                input_limb_21_col21.clone(),
                input_limb_22_col22.clone(),
                input_limb_23_col23.clone(),
                input_limb_24_col24.clone(),
                input_limb_25_col25.clone(),
                input_limb_26_col26.clone(),
                input_limb_27_col27.clone(),
                input_limb_28_col28.clone(),
                input_limb_29_col29.clone(),
                input_limb_30_col30.clone(),
                input_limb_31_col31.clone(),
                input_limb_32_col32.clone(),
                input_limb_33_col33.clone(),
                input_limb_34_col34.clone(),
                input_limb_35_col35.clone(),
                input_limb_36_col36.clone(),
                input_limb_37_col37.clone(),
                input_limb_38_col38.clone(),
                input_limb_39_col39.clone(),
                input_limb_40_col40.clone(),
                input_limb_41_col41.clone(),
                input_limb_42_col42.clone(),
                input_limb_43_col43.clone(),
                input_limb_44_col44.clone(),
                input_limb_45_col45.clone(),
                input_limb_46_col46.clone(),
                input_limb_47_col47.clone(),
                input_limb_48_col48.clone(),
                input_limb_49_col49.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.sha_256_round_lookup_elements,
            -E::EF::from(enabler.clone()),
            &[
                input_limb_0_col0.clone(),
                (input_limb_1_col1.clone() + M31_1.clone()),
                triple_sum32_res_limb_0_col88.clone(),
                triple_sum32_res_limb_1_col89.clone(),
                input_limb_2_col2.clone(),
                input_limb_3_col3.clone(),
                input_limb_4_col4.clone(),
                input_limb_5_col5.clone(),
                input_limb_6_col6.clone(),
                input_limb_7_col7.clone(),
                triple_sum32_res_limb_0_col86.clone(),
                triple_sum32_res_limb_1_col87.clone(),
                input_limb_10_col10.clone(),
                input_limb_11_col11.clone(),
                input_limb_12_col12.clone(),
                input_limb_13_col13.clone(),
                input_limb_14_col14.clone(),
                input_limb_15_col15.clone(),
                input_limb_20_col20.clone(),
                input_limb_21_col21.clone(),
                input_limb_22_col22.clone(),
                input_limb_23_col23.clone(),
                input_limb_24_col24.clone(),
                input_limb_25_col25.clone(),
                input_limb_26_col26.clone(),
                input_limb_27_col27.clone(),
                input_limb_28_col28.clone(),
                input_limb_29_col29.clone(),
                input_limb_30_col30.clone(),
                input_limb_31_col31.clone(),
                input_limb_32_col32.clone(),
                input_limb_33_col33.clone(),
                input_limb_34_col34.clone(),
                input_limb_35_col35.clone(),
                input_limb_36_col36.clone(),
                input_limb_37_col37.clone(),
                input_limb_38_col38.clone(),
                input_limb_39_col39.clone(),
                input_limb_40_col40.clone(),
                input_limb_41_col41.clone(),
                input_limb_42_col42.clone(),
                input_limb_43_col43.clone(),
                input_limb_44_col44.clone(),
                input_limb_45_col45.clone(),
                input_limb_46_col46.clone(),
                input_limb_47_col47.clone(),
                input_limb_48_col48.clone(),
                input_limb_49_col49.clone(),
                sha_256_schedule_output_limb_0_col84.clone(),
                sha_256_schedule_output_limb_1_col85.clone(),
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
    use crate::components::constraints_regression_test_values::SHA_256_ROUND;

    #[test]
    fn sha_256_round_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim { log_size: 4 },
            sha_256_big_sigma_1_lookup_elements: relations::Sha256BigSigma1::dummy(),
            sha_256_big_sigma_0_lookup_elements: relations::Sha256BigSigma0::dummy(),
            verify_bitwise_and_16_lookup_elements: relations::VerifyBitwiseAnd_16::dummy(),
            verify_bitwise_not_16_lookup_elements: relations::VerifyBitwiseNot_16::dummy(),
            verify_bitwise_xor_16_lookup_elements: relations::VerifyBitwiseXor_16::dummy(),
            sha_256_k_table_lookup_elements: relations::Sha256KTable::dummy(),
            sha_256_schedule_lookup_elements: relations::Sha256Schedule::dummy(),
            sha_256_round_lookup_elements: relations::Sha256Round::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        assert_eq!(sum, SHA_256_ROUND);
    }
}
