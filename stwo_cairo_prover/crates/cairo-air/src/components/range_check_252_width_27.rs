// This file was created by the AIR team.

use crate::components::prelude::*;

pub const N_TRACE_COLUMNS: usize = 20;
pub const RELATION_USES_PER_ROW: [RelationUse; 7] = [
    RelationUse {
        relation_id: "RangeCheck_18",
        uses: 7,
    },
    RelationUse {
        relation_id: "RangeCheck_18_B",
        uses: 2,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9",
        uses: 1,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_B",
        uses: 1,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_C",
        uses: 1,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_D",
        uses: 1,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_E",
        uses: 1,
    },
];

pub struct Eval {
    pub claim: Claim,
    pub range_check_9_9_lookup_elements: relations::RangeCheck_9_9,
    pub range_check_18_lookup_elements: relations::RangeCheck_18,
    pub range_check_9_9_b_lookup_elements: relations::RangeCheck_9_9_B,
    pub range_check_18_b_lookup_elements: relations::RangeCheck_18_B,
    pub range_check_9_9_c_lookup_elements: relations::RangeCheck_9_9_C,
    pub range_check_9_9_d_lookup_elements: relations::RangeCheck_9_9_D,
    pub range_check_9_9_e_lookup_elements: relations::RangeCheck_9_9_E,
    pub range_check_252_width_27_lookup_elements: relations::RangeCheck252Width27,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct Claim {
    pub log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 8];
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
        let M31_262144 = E::F::from(M31::from(262144));
        let M31_4194304 = E::F::from(M31::from(4194304));
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
        let limb_0_high_part_col10 = eval.next_trace_mask();
        let limb_1_low_part_col11 = eval.next_trace_mask();
        let limb_2_high_part_col12 = eval.next_trace_mask();
        let limb_3_low_part_col13 = eval.next_trace_mask();
        let limb_4_high_part_col14 = eval.next_trace_mask();
        let limb_5_low_part_col15 = eval.next_trace_mask();
        let limb_6_high_part_col16 = eval.next_trace_mask();
        let limb_7_low_part_col17 = eval.next_trace_mask();
        let limb_8_high_part_col18 = eval.next_trace_mask();
        let enabler = eval.next_trace_mask();

        eval.add_constraint(enabler.clone() * enabler.clone() - enabler.clone());

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[
                limb_0_high_part_col10.clone(),
                limb_1_low_part_col11.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(
                &(input_limb_0_col0.clone()
                    - (limb_0_high_part_col10.clone() * M31_262144.clone())),
            ),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(
                &((input_limb_1_col1.clone() - limb_1_low_part_col11.clone())
                    * M31_4194304.clone()),
            ),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_b_lookup_elements,
            E::EF::one(),
            &[
                limb_2_high_part_col12.clone(),
                limb_3_low_part_col13.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_b_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(
                &(input_limb_2_col2.clone()
                    - (limb_2_high_part_col12.clone() * M31_262144.clone())),
            ),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(
                &((input_limb_3_col3.clone() - limb_3_low_part_col13.clone())
                    * M31_4194304.clone()),
            ),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_c_lookup_elements,
            E::EF::one(),
            &[
                limb_4_high_part_col14.clone(),
                limb_5_low_part_col15.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(
                &(input_limb_4_col4.clone()
                    - (limb_4_high_part_col14.clone() * M31_262144.clone())),
            ),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(
                &((input_limb_5_col5.clone() - limb_5_low_part_col15.clone())
                    * M31_4194304.clone()),
            ),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_d_lookup_elements,
            E::EF::one(),
            &[
                limb_6_high_part_col16.clone(),
                limb_7_low_part_col17.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_b_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(
                &(input_limb_6_col6.clone()
                    - (limb_6_high_part_col16.clone() * M31_262144.clone())),
            ),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(
                &((input_limb_7_col7.clone() - limb_7_low_part_col17.clone())
                    * M31_4194304.clone()),
            ),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_e_lookup_elements,
            E::EF::one(),
            &[limb_8_high_part_col18.clone(), input_limb_9_col9.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_18_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(
                &(input_limb_8_col8.clone()
                    - (limb_8_high_part_col18.clone() * M31_262144.clone())),
            ),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_252_width_27_lookup_elements,
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
    use crate::components::constraints_regression_test_values::RANGE_CHECK_252_WIDTH_27;

    #[test]
    fn range_check_252_width_27_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim { log_size: 4 },
            range_check_9_9_lookup_elements: relations::RangeCheck_9_9::dummy(),
            range_check_18_lookup_elements: relations::RangeCheck_18::dummy(),
            range_check_9_9_b_lookup_elements: relations::RangeCheck_9_9_B::dummy(),
            range_check_18_b_lookup_elements: relations::RangeCheck_18_B::dummy(),
            range_check_9_9_c_lookup_elements: relations::RangeCheck_9_9_C::dummy(),
            range_check_9_9_d_lookup_elements: relations::RangeCheck_9_9_D::dummy(),
            range_check_9_9_e_lookup_elements: relations::RangeCheck_9_9_E::dummy(),
            range_check_252_width_27_lookup_elements: relations::RangeCheck252Width27::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        assert_eq!(sum, RANGE_CHECK_252_WIDTH_27);
    }
}
